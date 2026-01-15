# point_inversion (Curve / Surface Point Inversion)

- 이 문서는 `point_inversion.rs`에 들어있는 **Curve / Surface Point Inversion(점 역변환)** 구조를 “나중에 기능을 계속 추가/교체할 수 있도록” 설계 관점에서 정리한 것입니다.  
  - (코드 내부의 이름/타입은 지금 대화에서 공유된 구조를 기준으로 작성했습니다.)

---

## 1. Point Inversion이란?

### Curve inversion
- 주어진 점 `P`에 대해, 곡선 `C(u)` 위에서 거리

$$
g(u)=\|C(u)-P\|^2
$$

- 를 최소로 만드는 파라미터 `u*`를 찾는 문제입니다.

### Surface inversion
- 주어진 점 `P`에 대해, 서페이스 `S(u,v)` 위에서

$$
g(u,v)=\|S(u,v)-P\|^2
$$

- 를 최소로 만드는 `(u*,v*)`를 찾는 문제입니다.

- 둘 다 본질은 **최소제곱(least-squares) 최적화**이며, **Newton / Gauss-Newton / LineSearch / Fallback** 이 자연스럽게 붙습니다.

---

## 2. 현재 구조가 지향하는 목표

- 1. **Seed(초기값) 만들기** 와 **Local refine(국소 수렴)** 을 분리  
- 2. Domain(클램프/랩) 정책을 분리  
- 3. Newton 실패/불안정 시 **Fallback** 을 쉽게 바꿀 수 있게 분리  
- 4. 추후 **Patch 기반(Decompose) + LineSearch2D** 로 확장 가능  
- 5. **Numeric derivative → Analytic derivative** 로 교체 가능  
   - 이미 Surface 쪽에서 analytic 1차/2차 도함수 루틴이 추가된 상태라면, Refiner는 그것을 우선 사용하도록 바꾸는 것이 핵심

---

## 3. Curve Point Inversion 구조 요약

### 3.1 핵심 타입

- `CurvePointInversionOptions`  
  - 반복 횟수, seed 생성 정책(스캔/스프레드), 라인서치 횟수, 브렌트 fallback 등 전체 튜닝 파라미터.

- `CurveInversionFailReason`  
  - 실패 원인을 통일된 enum으로 기록.

- `CurvePointInversionResult`  
  - 결과: `ok`, `u`, `dist2`, `iters`, `used_seed`, `reason`.

### 3.2 Strategy / Policy 분리

- `CurveDomainPolicy`  
  - `CurveClampDomain`: domain 밖을 clamp  
  - `CurveWrapDomain`: periodic 곡선용 wrap

- `CurveSeedStrategy<C: CurveGeom>`  
  - seed 후보 생성.  
  - 기본 구현:  
    - 1) `u0` 주변 후보  
    - 2) (가능하면) knot span 기반 후보  
    - 3) uniform scan에서 best 후보

- `CurveLocalRefiner<C: CurveGeom>`  
  - seed에서 시작해 국소 수렴하는 알고리즘.  
  - 기본 구현: `CurveNewtonLineSearchRefiner`

- `CurveFallbackMinimizer`  
  - Newton 실패/부정확 시 fallback 최적화.  
  - 기본 구현: `CurveBrentFallback`

### 3.3 기본 알고리즘 흐름

- 1) seeds 생성  
- 2) 각 seed에 대해 Newton+LineSearch 실행, 최저 dist2 선택  
- 3) 필요시 Brent fallback 실행해 더 좋은 최소값이 있으면 교체

---

## 4. Surface Point Inversion 구조 요약

- Curve 구조를 거의 그대로 2D로 확장한 형태입니다.

### 4.1 핵심 타입

- `SurfacePointInversionOptions`  
  `scan_u/scan_v`, `seed_spread_u/v`, `newton_step_rel_u/v`, `gn_fallback` 등

- `SurfaceInversionFailReason`  
  `NoDerivatives` 등이 추가됨.

- `SurfacePointInversionResult`  
  결과: `ok`, `(u,v)`, `dist2`, `iters`, `used_seed`, `reason`.

### 4.2 Policy / Strategy

- `SurfaceDomainPolicy`
  - `ClampSurfaceDomain`: clamp
  - `WrapSurfaceDomain`: periodic surface용 wrap

- `SurfaceSeedStrategy<S: SurfaceGeom>`
  - 기본: `(u0,v0)` 주변 격자 + coarse scan best

- `SurfaceLocalRefiner<S: SurfaceGeom>`
  - **Refiner #1**: `Newton2ndLineSearchRefiner`  
    2차 도함수까지 가능한 경우, 2x2 시스템을 풀어 (du,dv) 업데이트  
  - **Refiner #2**: `GaussNewtonDampedRefiner`  
    1차 도함수만으로도 안정적으로 감소하는 방향을 찾는 fallback

### 4.3 Newton2ndLineSearchRefiner의 수식(요지)

목표 함수: `g(u,v)=||S(u,v)-P||^2`  
그라디언트:
- `g_u = 2 (S-P)·S_u`
- `g_v = 2 (S-P)·S_v`

Hessian 근사(정확 Newton을 흉내내는 형태):
- `uu = S_u·S_u + (S-P)·S_uu`
- `uv = S_u·S_v + (S-P)·S_uv`
- `vv = S_v·S_v + (S-P)·S_vv`

- 2x2 선형계:

```math
\begin{bmatrix}
uu  & uv\\
uv & vv
\end{bmatrix}
\begin{bmatrix}
du\\
dv
\end{bmatrix}
=
-
\begin{bmatrix}
(S-P)\cdot S_u\\
(S-P)\cdot S_v
\end{bmatrix}
```

---

## 5. Numeric derivative → Analytic derivative로 바꾸는 이유/방향

- 수치미분은 step 선택에 민감하고, near singular/고곡률에서 노이즈로 det가 깨지거나 descent가 망가질 수 있습니다.  
- 따라서 `SurfaceGeom`에 Analytic derivative가 있다면 다음 정책이 좋습니다.

  - 1) Analytic 우선  
  - 2) Analytic이 없을 때만 Numeric fallback  
  - 3) 둘 다 없으면 `NoDerivatives`

---

## 6. 왜 Decompose(패치 분해)를 PointInversion에 붙이려 하나?

- 전역적으로 inversion이 어려운 이유:
  - 파라미터 영역이 큰 경우 로컬 최소가 여러 개
  - knot span이 크거나 분포가 불균일
  - singular/near singular 영역 존재

- 패치로 나누면:
  - seed 탐색을 패치 단위로 제한 가능
  - 문제 패치만 fallback 강화 가능
  - 경계 처리(클램프/랩)가 단순해짐

---

## 7. 확장 방향 (추가하면 효과 큰 순)

### 7.1 LineSearch2D trait 분리(강력 추천)

- Refiner 내부 backtracking을 전략으로 분리하면 모든 스텝(Newton/GN/LM)에 재사용 가능합니다.

```rust
pub trait LineSearch2D {
    fn search(
        &self,
        dom_u: Interval,
        dom_v: Interval,
        u: f64, v: f64,
        du: f64, dv: f64,
        g0: f64,
        eval_g: &dyn Fn(f64,f64)->f64,
        dom_pol: &dyn SurfaceDomainPolicy,
        max_iters: usize
    ) -> Option<(f64,f64,f64)>; // (u_new, v_new, g_new)
}
```

### 7.2 Fallback 강화 순서(추천)

- 1) Newton2nd + LineSearch2D  
- 2) GN + LineSearch2D  
- 3) 2D coarse grid 재탐색 후 재시도  
- 4) Decompose patch 기반: 패치별 best seed → global best  
- 5) 마지막: 짧은 2D coordinate descent

### 7.3 Singular 줄이기: LM(Levenberg–Marquardt)

- det가 작을 때 바로 실패 대신 `H + λI`로 안정화하고 λ를 키우며 재시도하면 매우 강해집니다.

---

## 8. 테스트 확장 체크리스트

- 기본 성공: bilinear 같은 analytic 서페이스
- 경계: u/v가 0/1 근처, patch 경계 근처
- 어려움: su,sv가 거의 평행, seed가 멀리 있는 케이스
- 회귀: dist뿐 아니라 gradient 작음, perturb에서 dist 증가 확인

---

## 9. 나중에 내가 기능 추가할 때 실전 체크

- 새 Surface 타입: `eval_point`, `domain_u/v`는 필수  
- 가능하면 `evaluate_1st_ders`, 더 좋으면 `evaluate_2nd_ders` 제공  
- Refiner 추가는 `SurfaceLocalRefiner`만 구현하면 주입 가능  
- Patch 기반은 “패치별 seed + best 선택”으로 단순화

---

## 10. 결론

- 지금 구조는 **확장 가능한 뼈대**로 충분히 쓸만합니다.  
- 다음 3개만 들어가면, 이후 강화(LM/trust region/patch 기반)가 매우 쉬워집니다.
  - 1) `LineSearch2D` trait 추가  
  - 2) Refiner들이 line search를 주입받아 사용  
  - 3) derivative 호출을 “Analytic 우선”으로 통일

---

## 소스 코드
```rust
use crate::core::point_ops::PointOps;
use crate::core::prelude::{Interval, Point3D};
use crate::topology::geom_kernel::CurveGeom;
use crate::topology::geom_kernel::SurfaceGeom;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveInversionFailReason {
    None,
    NoSeed,
    SeedTooFar,
    Singular,
    NoDescent,
    OutOfDomain,
    MaxIter,
}
```
```rust
#[derive(Clone, Debug)]
pub struct CurvePointInversionResult {
    pub ok: bool,
    pub u: f64,
    pub dist2: f64,
    pub iters: usize,
    pub used_seed: f64,
    pub reason: CurveInversionFailReason,
}
```
```rust
impl CurvePointInversionResult {
    fn fail(domain: Interval, u0: f64, reason: CurveInversionFailReason) -> Self {
        let u = domain.clamp(u0);
        Self {
            ok: false,
            u,
            dist2: f64::INFINITY,
            iters: 0,
            used_seed: u,
            reason,
        }
    }
}
```
```rust
#[derive(Clone, Debug)]
pub struct CurvePointInversionOptions {
    pub tol: f64, // distance tolerance
    pub max_newton: usize,
    pub scan_samples: usize,   // 0 = off
    pub seed_spread: f64,      // relative to domain length
    pub max_seed_count: usize, // cap seeds
    pub newton_step_rel: f64,  // max |du| <= newton_step_rel * domain_len
    pub line_search_iters: usize,
    pub brent_fallback: bool,
    pub brent_tol: f64,
    pub brent_iters: usize,
    pub seed_far_guard: Option<f64>, // if Some(k), and dist > k => consider seed too far
}
```
```rust
impl Default for CurvePointInversionOptions {
    fn default() -> Self {
        Self {
            tol: 1e-12,
            max_newton: 30,
            scan_samples: 128,
            seed_spread: 0.05,
            max_seed_count: 16,
            newton_step_rel: 0.25,
            line_search_iters: 12,
            brent_fallback: true,
            brent_tol: 1e-14,
            brent_iters: 80,
            seed_far_guard: None,
        }
    }
}
```
```rust
pub trait CurveDomainPolicy {
    fn normalize(&self, dom: Interval, u: f64) -> f64;
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct CurveClampDomain;
impl CurveDomainPolicy for CurveClampDomain {
    fn normalize(&self, dom: Interval, u: f64) -> f64 {
        dom.clamp(u)
    }
}
```
```rust
/// For periodic curves: keep u in [a,b] by wrapping.
/// Use only if curve really periodic in parameter space.
#[derive(Clone, Copy, Debug)]
pub struct CurveWrapDomain;
impl CurveDomainPolicy for CurveWrapDomain {
    fn normalize(&self, dom: Interval, mut u: f64) -> f64 {
        let len = dom.len();
        if len <= 0.0 {
            return dom.t0;
        }
        while u < dom.t0 {
            u += len;
        }
        while u > dom.t1 {
            u -= len;
        }
        // edge: numeric
        dom.clamp(u)
    }
}
```
```rust
pub trait CurveSeedStrategy<C: CurveGeom> {
    fn build_seeds(
        &self,
        curve: &C,
        p: Point3D,
        u0: f64,
        opt: &CurvePointInversionOptions,
    ) -> Vec<f64>;
}
```
```rust
/// Default seeds: u0 neighborhood + (optional) coarse scan + (optional) knot-span sampling
pub struct DefaultCurveSeedStrategy;

impl<C: CurveGeom> CurveSeedStrategy<C> for DefaultCurveSeedStrategy {
    fn build_seeds(
        &self,
        curve: &C,
        p: Point3D,
        u0: f64,
        opt: &CurvePointInversionOptions,
    ) -> Vec<f64> {
        let dom = curve.domain();
        let len = dom.len().abs().max(1e-30);

        let mut seeds: Vec<f64> = Vec::new();
        let u0 = dom.clamp(u0);

        // 1) u0 neighborhood
        let s = opt.seed_spread * len;
        let candidates = [
            u0,
            u0 + s,
            u0 - s,
            u0 + 0.5 * s,
            u0 - 0.5 * s,
            // for periodic-ish robustness: opposite point in param
            u0 + 0.5 * len,
        ];
        for &u in &candidates {
            seeds.push(dom.clamp(u));
        }

        // 2) knot span based candidates (if available)
        if let Some(mut spans) = curve.knot_span_parameters() {
            // keep only those in domain
            spans.retain(|&u| dom.contains(u));
            // sample midpoints of spans and endpoints
            for w in spans.windows(2) {
                let a = w[0];
                let b = w[1];
                seeds.push(dom.clamp(a));
                seeds.push(dom.clamp(b));
                seeds.push(dom.clamp(0.5 * (a + b)));
            }
        }

        // 3) coarse scan: pick best u among uniform samples
        if opt.scan_samples >= 8 {
            let n = opt.scan_samples;
            let mut best_u = dom.t0;
            let mut best_g = f64::INFINITY;
            for i in 0..=n {
                let t = (i as f64) / (n as f64);
                let u = dom.t0 + t * (dom.t1 - dom.t0);
                let g = curve.eval_point(u).distance_square(&p);
                if g < best_g {
                    best_g = g;
                    best_u = u;
                }
            }
            seeds.push(best_u);
        }

        // sort & dedup
        seeds.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        seeds.dedup_by(|a, b| (*a - *b).abs() < 1e-12);

        // cap
        if seeds.len() > opt.max_seed_count {
            // keep first + last + some evenly
            let mut out = Vec::with_capacity(opt.max_seed_count);
            out.push(seeds[0]);
            if opt.max_seed_count > 1 {
                out.push(*seeds.last().unwrap());
            }
            let remain = opt.max_seed_count.saturating_sub(out.len());
            if remain > 0 && seeds.len() > 2 {
                let step = (seeds.len() - 2) as f64 / (remain as f64 + 1.0);
                for k in 0..remain {
                    let idx = 1 + ((k as f64 + 1.0) * step).round() as usize;
                    out.push(seeds[idx.min(seeds.len() - 2)]);
                }
            }
            out.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            out.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
            return out;
        }

        // guard: if requested, filter seeds too far (coarse check)
        if let Some(k) = opt.seed_far_guard {
            let mut filtered = Vec::new();
            for &u in &seeds {
                let g = curve.eval_point(u).distance_square(&p);
                if g <= k * k {
                    filtered.push(u);
                }
            }
            if !filtered.is_empty() {
                return filtered;
            }
        }

        seeds
    }
}
```
```rust
pub trait CurveLocalRefiner<C: CurveGeom> {
    fn refine(
        &self,
        curve: &C,
        p: Point3D,
        seed: f64,
        opt: &CurvePointInversionOptions,
        dom_pol: &dyn CurveDomainPolicy,
    ) -> CurvePointInversionResult;
}
```
```rust
pub struct CurveNewtonLineSearchRefiner;

impl<C: CurveGeom> CurveLocalRefiner<C> for CurveNewtonLineSearchRefiner {
    fn refine(
        &self,
        curve: &C,
        p: Point3D,
        seed: f64,
        opt: &CurvePointInversionOptions,
        dom_pol: &dyn CurveDomainPolicy,
    ) -> CurvePointInversionResult {
        let dom = curve.domain();
        let tol2 = opt.tol * opt.tol;
        let mut u = dom_pol.normalize(dom, seed);

        let mut g = curve.eval_point(u).distance_square(&p);
        let mut best_u = u;
        let mut best_g = g;

        let max_step = opt.newton_step_rel * dom.len().abs().max(1e-30);

        let mut last_g = g;
        let mut iters = 0;

        for k in 0..opt.max_newton {
            iters = k + 1;

            let c = curve.eval_point(u);
            let d1 = curve.eval_tangent(u).expect("Invalid tangent");
            let r = c - p;

            // g'(u) = 2 (C-P)·C'
            let gp = 2.0 * r.dot_vec(&d1);

            // g''(u) = 2( C'·C' + (C-P)·C'' )
            let d1d1 = d1.dot(&d1);
            let denom = if let Some(d2) = curve.eval_second_derivative(u) {
                2.0 * (d1d1 + r.dot_vec(&d2)).abs().max(1e-30)
            } else {
                // fallback: assume convex-ish
                2.0 * d1d1.max(1e-30)
            };

            if !denom.is_finite() || denom <= 0.0 {
                return CurvePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: CurveInversionFailReason::Singular,
                };
            }

            // Newton step
            let mut step = -gp / denom;
            if step > max_step {
                step = max_step;
            }
            if step < -max_step {
                step = -max_step;
            }

            // Line search: require descent in g
            let mut alpha = 1.0;
            let mut accepted = false;
            let mut u_new = u;
            let mut g_new = g;

            for _ls in 0..opt.line_search_iters {
                u_new = dom_pol.normalize(dom, u + alpha * step);
                g_new = curve.eval_point(u_new).distance_square(&p);
                if g_new <= g {
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted {
                // NoDescent: keep best found so far
                return CurvePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: CurveInversionFailReason::NoDescent,
                };
            }

            u = u_new;
            g = g_new;

            if g < best_g {
                best_g = g;
                best_u = u;
            }

            if best_g <= tol2 {
                return CurvePointInversionResult {
                    ok: true,
                    u: best_u,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: CurveInversionFailReason::None,
                };
            }

            // stagnation check
            if (last_g - g).abs() < 1e-18 {
                break;
            }
            last_g = g;
        }

        CurvePointInversionResult {
            ok: best_g <= tol2,
            u: best_u,
            dist2: best_g,
            iters,
            used_seed: seed,
            reason: CurveInversionFailReason::MaxIter,
        }
    }
}
```
```rust
pub trait CurveFallbackMinimizer {
    fn minimize(
        &self,
        f: &dyn Fn(f64) -> f64,
        a: f64,
        x: f64,
        b: f64,
        tol: f64,
        iters: usize,
    ) -> f64;
}
```
```rust
pub struct CurveBrentFallback;
impl CurveFallbackMinimizer for CurveBrentFallback {
    fn minimize(
        &self,
        f: &dyn Fn(f64) -> f64,
        a0: f64,
        x0: f64,
        b0: f64,
        tol: f64,
        iters: usize,
    ) -> f64 {
        let mut a = a0;
        let mut b = b0;
        let c = 0.5 * (3.0 - 5.0_f64.sqrt());

        let mut x = x0.clamp(a, b);
        let mut w = x;
        let mut v = x;

        let mut fx = f(x);
        let mut fw = fx;
        let mut fv = fx;

        let mut d = 0.0;
        let mut e = 0.0_f64;

        for _ in 0..iters {
            let m = 0.5 * (a + b);
            let tol1 = tol * x.abs() + 1e-18_f64;
            let tol2 = 2.0 * tol1;

            if (x - m).abs() <= (tol2 - 0.5 * (b - a)) {
                break;
            }

            let mut p = 0.0;
            let mut q = 0.0;
            let mut r = 0.0;

            if e.abs() > tol1 {
                r = (x - w) * (fx - fv);
                q = (x - v) * (fx - fw);
                p = (x - v) * q - (x - w) * r;
                q = 2.0 * (q - r);
                if q > 0.0 {
                    p = -p;
                }
                q = q.abs();

                let etemp = e;
                e = d;

                if p.abs() < 0.5 * q * etemp && p > q * (a - x) && p < q * (b - x) {
                    d = p / q;
                } else {
                    e = if x >= m { a - x } else { b - x };
                    d = c * e;
                }
            } else {
                e = if x >= m { a - x } else { b - x };
                d = c * e;
            }

            let step = if d.abs() >= tol1 {
                d
            } else {
                if d > 0.0 { tol1 } else { -tol1 }
            };
            let u = (x + step).clamp(a, b);
            let fu = f(u);

            if fu <= fx {
                if u >= x {
                    a = x;
                } else {
                    b = x;
                }
                v = w;
                fv = fw;
                w = x;
                fw = fx;
                x = u;
                fx = fu;
            } else {
                if u < x {
                    a = u;
                } else {
                    b = u;
                }
                if fu <= fw || (w - x).abs() < 1e-18 {
                    v = w;
                    fv = fw;
                    w = u;
                    fw = fu;
                } else if fu <= fv || (v - x).abs() < 1e-18 || (v - w).abs() < 1e-18 {
                    v = u;
                    fv = fu;
                }
            }
        }

        x
    }
}
```
```rust
pub struct CurvePointInverter<C: CurveGeom> {
    pub seed: Box<dyn CurveSeedStrategy<C>>,
    pub refiner: Box<dyn CurveLocalRefiner<C>>,
    pub dom_pol: Box<dyn CurveDomainPolicy>,
    pub fallback: Box<dyn CurveFallbackMinimizer>,
}

impl<C: CurveGeom> Default for CurvePointInverter<C> {
    fn default() -> Self {
        Self {
            seed: Box::new(DefaultCurveSeedStrategy),
            refiner: Box::new(CurveNewtonLineSearchRefiner),
            dom_pol: Box::new(CurveClampDomain),
            fallback: Box::new(CurveBrentFallback),
        }
    }
}
```
```rust
impl<C: CurveGeom> CurvePointInverter<C> {
    pub fn with_periodic_domain(mut self) -> Self {
        self.dom_pol = Box::new(CurveWrapDomain);
        self
    }

    pub fn invert(
        &self,
        curve: &C,
        p: Point3D,
        u0: f64,
        opt: &CurvePointInversionOptions,
    ) -> CurvePointInversionResult {
        let dom = curve.domain();
        let tol2 = opt.tol * opt.tol;

        let seeds = self.seed.build_seeds(curve, p, u0, opt);
        if seeds.is_empty() {
            return CurvePointInversionResult::fail(dom, u0, CurveInversionFailReason::NoSeed);
        }

        let mut best = CurvePointInversionResult::fail(dom, u0, CurveInversionFailReason::NoSeed);

        for &s in &seeds {
            let r = self.refiner.refine(curve, p, s, opt, self.dom_pol.as_ref());
            if r.dist2 < best.dist2 {
                best = r;
            }
            if best.dist2 <= tol2 && best.ok {
                // early exit OK
                break;
            }
        }

        // Brent fallback: only if still not good
        if opt.brent_fallback && best.dist2 > tol2 {
            let a = dom.t0;
            let b = dom.t1;
            let x = best.u;

            let f = |u: f64| {
                curve
                    .eval_point(self.dom_pol.normalize(dom, u))
                    .distance_square(&p)
            };
            let u_brent = self
                .fallback
                .minimize(&f, a, x, b, opt.brent_tol, opt.brent_iters);
            let u_brent = self.dom_pol.normalize(dom, u_brent);
            let g_brent = curve.eval_point(u_brent).distance_square(&p);

            if g_brent < best.dist2 {
                best.u = u_brent;
                best.dist2 = g_brent;
                best.ok = g_brent <= tol2;
                best.reason = if best.ok {
                    CurveInversionFailReason::None
                } else {
                    best.reason
                };
            }
        }

        best
    }
}
```
```rust
// ============================================================================
// Surface Point Inversion (organized like Curve PointInverter)
// ============================================================================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceInversionFailReason {
    None,
    NoSeed,
    SeedTooFar,
    Singular,
    NoDescent,
    OutOfDomain,
    MaxIter,
    NoDerivatives,
    Unknown,
}
```
```rust
#[derive(Clone, Debug)]
pub struct SurfacePointInversionResult {
    pub ok: bool,
    pub u: f64,
    pub v: f64,
    pub dist2: f64,
    pub iters: usize,
    pub used_seed: (f64, f64),
    pub reason: SurfaceInversionFailReason,
}
```
```rust
impl SurfacePointInversionResult {
    fn fail(
        dom_u: Interval,
        dom_v: Interval,
        u0: f64,
        v0: f64,
        reason: SurfaceInversionFailReason,
    ) -> Self {
        let u = dom_u.clamp(u0);
        let v = dom_v.clamp(v0);
        Self {
            ok: false,
            u,
            v,
            dist2: f64::INFINITY,
            iters: 0,
            used_seed: (u, v),
            reason,
        }
    }
}
```
```rust
#[derive(Clone, Debug)]
pub struct SurfacePointInversionOptions {
    pub tol: f64, // distance tolerance
    pub max_newton: usize,
    pub scan_u: usize,          // 0 = off
    pub scan_v: usize,          // 0 = off
    pub seed_spread_u: f64,     // relative to domain length
    pub seed_spread_v: f64,     // relative to domain length
    pub max_seed_count: usize,  // cap seeds
    pub newton_step_rel_u: f64, // max |du| <= rel * len_u
    pub newton_step_rel_v: f64, // max |dv| <= rel * len_v
    pub line_search_iters: usize,
    pub gn_fallback: bool,
    pub gn_max_iter: usize,
    pub seed_far_guard: Option<f64>, // if Some(k), and dist > k => consider seed too far

    pub line_search_mode: SurfaceLineSearchMode,
    /// AfterFailure에서 사용할 "안전 step" 크기(도메인 길이 대비)
    pub fallback_step_rel: f64,
}
```
```rust
impl Default for SurfacePointInversionOptions {
    fn default() -> Self {
        Self {
            tol: 1e-12,
            max_newton: 30,
            scan_u: 9,
            scan_v: 9,
            seed_spread_u: 0.05,
            seed_spread_v: 0.05,
            max_seed_count: 32,
            newton_step_rel_u: 0.25,
            newton_step_rel_v: 0.25,
            line_search_iters: 12,
            gn_fallback: true,
            gn_max_iter: 80,
            seed_far_guard: None,
            line_search_mode: SurfaceLineSearchMode::InsideOnly,
            fallback_step_rel: 0.05,
        }
    }
}
```
```rust
/// Domain policy for (u,v)
pub trait SurfaceDomainPolicy {
    fn normalize_u(&self, dom: Interval, u: f64) -> f64;
    fn normalize_v(&self, dom: Interval, v: f64) -> f64;
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct ClampSurfaceDomain;
impl SurfaceDomainPolicy for ClampSurfaceDomain {
    fn normalize_u(&self, dom: Interval, u: f64) -> f64 {
        dom.clamp(u)
    }
    fn normalize_v(&self, dom: Interval, v: f64) -> f64 {
        dom.clamp(v)
    }
}
```
```rust
/// Wrap domain (for periodic/closed surfaces). Use only when surface actually wraps in param.
#[derive(Clone, Copy, Debug)]
pub struct WrapSurfaceDomain;
impl WrapSurfaceDomain {
    #[inline]
    fn wrap(dom: Interval, mut t: f64) -> f64 {
        let len = dom.len();
        if len <= 0.0 {
            return dom.t0;
        }
        while t < dom.t0 {
            t += len;
        }
        while t > dom.t1 {
            t -= len;
        }
        dom.clamp(t)
    }
}
```
```rust
impl SurfaceDomainPolicy for WrapSurfaceDomain {
    fn normalize_u(&self, dom: Interval, u: f64) -> f64 {
        Self::wrap(dom, u)
    }
    fn normalize_v(&self, dom: Interval, v: f64) -> f64 {
        Self::wrap(dom, v)
    }
}
```
```rust
pub trait SurfaceSeedStrategy<S: SurfaceGeom> {
    fn build_seeds(
        &self,
        srf: &S,
        p: Point3D,
        uv0: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> Vec<(f64, f64)>;
}
```
```rust
/// Default seeds:
/// - neighborhood around (u0,v0)
/// - optional coarse scan (scan_u x scan_v) pick best cell center
/// - optional knot-span sampling if available (if SurfaceGeom exposes it; if not, skip)
pub struct DefaultSurfaceSeedStrategy;

impl<S: SurfaceGeom> SurfaceSeedStrategy<S> for DefaultSurfaceSeedStrategy {
    fn build_seeds(
        &self,
        srf: &S,
        p: Point3D,
        uv0: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> Vec<(f64, f64)> {
        let du = srf.domain_u();
        let dv = srf.domain_v();
        let len_u = du.len().abs().max(1e-30);
        let len_v = dv.len().abs().max(1e-30);

        let mut seeds: Vec<(f64, f64)> = Vec::new();

        let u0 = du.clamp(uv0.0);
        let v0 = dv.clamp(uv0.1);

        // 1) neighborhood
        let su = opt.seed_spread_u * len_u;
        let sv = opt.seed_spread_v * len_v;

        let cand_u = [
            u0,
            u0 + su,
            u0 - su,
            u0 + 0.5 * su,
            u0 - 0.5 * su,
            u0 + 0.5 * len_u,
        ];
        let cand_v = [
            v0,
            v0 + sv,
            v0 - sv,
            v0 + 0.5 * sv,
            v0 - 0.5 * sv,
            v0 + 0.5 * len_v,
        ];

        for &uu in &cand_u {
            for &vv in &cand_v {
                seeds.push((du.clamp(uu), dv.clamp(vv)));
            }
        }

        // 2) coarse scan: find best among grid samples
        if opt.scan_u >= 3 && opt.scan_v >= 3 {
            let nu = opt.scan_u;
            let nv = opt.scan_v;

            let mut best_uv = (u0, v0);
            let mut best_g = f64::INFINITY;

            for j in 0..nv {
                let tv = (j as f64) / ((nv - 1) as f64);
                let v = dv.t0 + tv * (dv.t1 - dv.t0);

                for i in 0..nu {
                    let tu = (i as f64) / ((nu - 1) as f64);
                    let u = du.t0 + tu * (du.t1 - du.t0);

                    let q = srf.eval_point(u, v);
                    let g = q.distance_square(&p);
                    if g < best_g {
                        best_g = g;
                        best_uv = (u, v);
                    }
                }
            }
            seeds.push(best_uv);
        }

        // 3) (optional) knot-span candidates if your SurfaceGeom supports it later
        // if let Some((us, vs)) = srf.knot_span_parameters_uv() { ... }

        // sort & dedup
        seeds.sort_by(|a, b| {
            let ou = a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal);
            if ou != Ordering::Equal {
                ou
            } else {
                a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)
            }
        });
        seeds.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-12 && (a.1 - b.1).abs() < 1e-12);

        // cap
        if seeds.len() > opt.max_seed_count {
            let mut out = Vec::with_capacity(opt.max_seed_count);
            out.push(seeds[0]);
            if opt.max_seed_count > 1 {
                out.push(*seeds.last().unwrap());
            }

            let remain = opt.max_seed_count.saturating_sub(out.len());
            if remain > 0 && seeds.len() > 2 {
                let step = (seeds.len() - 2) as f64 / (remain as f64 + 1.0);
                for k in 0..remain {
                    let idx = 1 + ((k as f64 + 1.0) * step).round() as usize;
                    out.push(seeds[idx.min(seeds.len() - 2)]);
                }
            }

            out.sort_by(|a, b| {
                let ou = a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal);
                if ou != Ordering::Equal {
                    ou
                } else {
                    a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)
                }
            });
            out.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-12 && (a.1 - b.1).abs() < 1e-12);

            // seed far guard (optional)
            if let Some(k) = opt.seed_far_guard {
                let mut filtered = Vec::new();
                for &uv in &out {
                    let g = srf.eval_point(uv.0, uv.1).distance_square(&p);
                    if g <= k * k {
                        filtered.push(uv);
                    }
                }
                if !filtered.is_empty() {
                    return filtered;
                }
            }

            return out;
        }

        // seed far guard (optional)
        if let Some(k) = opt.seed_far_guard {
            let mut filtered = Vec::new();
            for &uv in &seeds {
                let g = srf.eval_point(uv.0, uv.1).distance_square(&p);
                if g <= k * k {
                    filtered.push(uv);
                }
            }
            if !filtered.is_empty() {
                return filtered;
            }
        }

        seeds
    }
}
```
```rust
pub trait SurfaceLocalRefiner<S: SurfaceGeom> {
    fn refine(
        &self,
        srf: &S,
        p: Point3D,
        seed: (f64, f64),
        opt: &SurfacePointInversionOptions,
        dom_pol: &dyn SurfaceDomainPolicy,
    ) -> SurfacePointInversionResult;
}
```
```rust
/// Refiner #1: Newton using 2nd derivatives when available + line search.
/// Uses: ders(0,0)=S, (1,0)=Su, (0,1)=Sv, (2,0)=Suu, (1,1)=Suv, (0,2)=Svv
pub struct Newton2ndLineSearchRefiner;

impl<S: SurfaceGeom> SurfaceLocalRefiner<S> for Newton2ndLineSearchRefiner {
    fn refine(
        &self,
        srf: &S,
        p: Point3D,
        seed: (f64, f64),
        opt: &SurfacePointInversionOptions,
        dom_pol: &dyn SurfaceDomainPolicy,
    ) -> SurfacePointInversionResult {
        let du_dom = srf.domain_u();
        let dv_dom = srf.domain_v();

        let tol2 = opt.tol * opt.tol;

        let mut u = dom_pol.normalize_u(du_dom, seed.0);
        let mut v = dom_pol.normalize_v(dv_dom, seed.1);

        let mut best_u = u;
        let mut best_v = v;
        let mut best_g = srf.eval_point(u, v).distance_square(&p);

        let len_u = du_dom.len().abs().max(1e-30);
        let len_v = dv_dom.len().abs().max(1e-30);
        let max_du = opt.newton_step_rel_u * len_u;
        let max_dv = opt.newton_step_rel_v * len_v;

        let mut iters = 0usize;
        let mut last_g = best_g;

        for k in 0..opt.max_newton {
            iters = k + 1;

            // need 2nd derivatives (n_der=2)
            let ders = srf.evaluate_derivatives_nder(u, v, 2);
            if ders.len() < 3 || ders[0].len() < 3 {
                return SurfacePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::NoDerivatives,
                };
            }

            let s = ders[0][0];
            let su = ders[1][0];
            let sv = ders[0][1];
            let suu = ders[2][0];
            let suv = ders[1][1];
            let svv = ders[0][2];

            let q = Point3D::new(s.x, s.y, s.z);
            let r = q - p;

            let g = q.distance_square(&p);
            if g < best_g {
                best_g = g;
                best_u = u;
                best_v = v;
            }
            if best_g <= tol2 {
                return SurfacePointInversionResult {
                    ok: true,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::None,
                };
            }

            // Build 2x2 system (matching classic ON style):
            // fu = Su·Su + r·Suu
            // fv = Su·Sv + r·Suv
            // gv = Sv·Sv + r·Svv
            let ru = su.dot_pt(&r);
            let rv = sv.dot_pt(&r);

            let uu = su.dot(&su) + r.dot_vec(&suu);
            let uv = su.dot(&sv) + r.dot_vec(&suv);
            let vv = sv.dot(&sv) + r.dot_vec(&svv);

            // Solve [uu uv; uv vv] [du dv] = -[ru rv]
            let det = uu * vv - uv * uv;
            let eps = 1e-14;
            if !det.is_finite() || det.abs() < eps {
                return SurfacePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::Singular,
                };
            }

            let mut du = (-ru * vv + rv * uv) / det;
            let mut dv = (-rv * uu + ru * uv) / det;

            if du > max_du {
                du = max_du;
            }
            if du < -max_du {
                du = -max_du;
            }
            if dv > max_dv {
                dv = max_dv;
            }
            if dv < -max_dv {
                dv = -max_dv;
            }

            // line search: require descent in g
            let mut alpha = 1.0;
            let mut accepted = false;
            let mut u_new = u;
            let mut v_new = v;
            let mut g_new = g;

            for _ls in 0..opt.line_search_iters {
                u_new = dom_pol.normalize_u(du_dom, u + alpha * du);
                v_new = dom_pol.normalize_v(dv_dom, v + alpha * dv);
                g_new = srf.eval_point(u_new, v_new).distance_square(&p);
                if g_new <= g {
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted {
                return SurfacePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::NoDescent,
                };
            }

            u = u_new;
            v = v_new;

            // stagnation
            if (last_g - g_new).abs() < 1e-18 {
                break;
            }
            last_g = g_new;
        }

        SurfacePointInversionResult {
            ok: best_g <= tol2,
            u: best_u,
            v: best_v,
            dist2: best_g,
            iters,
            used_seed: seed,
            reason: SurfaceInversionFailReason::MaxIter,
        }
    }
}
```
```rust
/// Refiner #2 (fallback): Damped Gauss-Newton using only 1st derivatives.
/// Very robust for surfaces (recommended fallback instead of Brent).
pub struct GaussNewtonDampedRefiner;

impl<S: SurfaceGeom> SurfaceLocalRefiner<S> for GaussNewtonDampedRefiner {
    fn refine(
        &self,
        srf: &S,
        p: Point3D,
        seed: (f64, f64),
        opt: &SurfacePointInversionOptions,
        dom_pol: &dyn SurfaceDomainPolicy,
    ) -> SurfacePointInversionResult {
        let du_dom = srf.domain_u();
        let dv_dom = srf.domain_v();

        let tol2 = opt.tol * opt.tol;

        let mut u = dom_pol.normalize_u(du_dom, seed.0);
        let mut v = dom_pol.normalize_v(dv_dom, seed.1);

        let mut best_u = u;
        let mut best_v = v;
        let mut best_g = srf.eval_point(u, v).distance_square(&p);

        let eps = 1e-14;
        let mut iters = 0usize;

        for k in 0..opt.gn_max_iter.max(1) {
            iters = k + 1;

            let ders = srf.evaluate_derivatives_nder(u, v, 1);
            if ders.len() < 2 || ders[0].len() < 2 {
                return SurfacePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::NoDerivatives,
                };
            }

            let s = ders[0][0];
            let su = ders[1][0];
            let sv = ders[0][1];

            let q = Point3D::new(s.x, s.y, s.z);
            let r = q - p;

            let g = q.distance_square(&p);
            if g < best_g {
                best_g = g;
                best_u = u;
                best_v = v;
            }
            if best_g <= tol2 {
                return SurfacePointInversionResult {
                    ok: true,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::None,
                };
            }

            // gradient: [Su·r, Sv·r]
            let gu = su.dot_pt(&r);
            let gv = sv.dot_pt(&r);

            // H = J^T J
            let a = su.dot(&su) + eps;
            let b = su.dot(&sv);
            let c = sv.dot(&sv) + eps;

            let det = a * c - b * b;
            if !det.is_finite() || det.abs() < eps {
                break;
            }

            let du_step = (-gu * c + gv * b) / det;
            let dv_step = (-gv * a + gu * b) / det;

            // backtracking
            let mut alpha = 1.0;
            let mut accepted = false;

            for _ls in 0..opt.line_search_iters.max(8) {
                let uu = dom_pol.normalize_u(du_dom, u + alpha * du_step);
                let vv = dom_pol.normalize_v(dv_dom, v + alpha * dv_step);

                let gg = srf.eval_point(uu, vv).distance_square(&p);
                if gg <= g {
                    u = uu;
                    v = vv;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
                if alpha < 1e-6 {
                    break;
                }
            }

            if !accepted {
                break;
            }
        }

        SurfacePointInversionResult {
            ok: best_g <= tol2,
            u: best_u,
            v: best_v,
            dist2: best_g,
            iters,
            used_seed: seed,
            reason: SurfaceInversionFailReason::MaxIter,
        }
    }
}
```
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceLineSearchMode {
    /// Refiner 내부에서만 사용(기존 방식). Refiner가 NoDescent면 그대로 실패.
    InsideOnly,

    /// Refiner 결과가 NoDescent/Singular/MaxIter 등으로 나쁘면,
    /// (best_uv 기준으로) gradient 방향 같은 "안전 step"을 만들어 LineSearch로 한 번 더 시도.
    AfterFailure,

    /// 매 iteration에서 step을 만든 뒤 반드시 LineSearch로만 진행(강제 안전모드)
    Always,
    OnFail,
}
```
```rust
pub trait LineSearch2D<S: SurfaceGeom> {
    /// Returns accepted (u,v,dist2,alpha) or None.
    fn search(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        step: (f64, f64),
        dom_pol: &dyn SurfaceDomainPolicy,
        opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64, f64, f64)>;
}
```
```rust
pub struct BacktrackingLineSearch2D {
    pub alpha0: f64,
    pub shrink: f64,
    pub min_alpha: f64,
}
```
```rust
impl Default for BacktrackingLineSearch2D {
    fn default() -> Self {
        Self {
            alpha0: 1.0,
            shrink: 0.5,
            min_alpha: 1e-6,
        }
    }
}
```
```rust
impl<S: SurfaceGeom> LineSearch2D<S> for BacktrackingLineSearch2D {
    fn search(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        step: (f64, f64),
        dom_pol: &dyn SurfaceDomainPolicy,
        opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64, f64, f64)> {
        let du = srf.domain_u();
        let dv = srf.domain_v();

        let (u, v) = uv;
        let d2_old = srf.eval_point(u, v).distance_square(&p);

        let mut alpha = self.alpha0;
        for _ in 0..opt.line_search_iters.max(1) {
            let uu = dom_pol.normalize_u(du, u + alpha * step.0);
            let vv = dom_pol.normalize_v(dv, v + alpha * step.1);
            let d2 = srf.eval_point(uu, vv).distance_square(&p);

            if d2 <= d2_old {
                return Some((uu, vv, d2, alpha));
            }

            alpha *= self.shrink;
            if alpha < self.min_alpha {
                break;
            }
        }

        None
    }
}
```
```rust
pub struct BisectLineSearch2D {
    pub alpha0: f64,
    pub shrink: f64,
    pub min_alpha: f64,
    pub bisect_iters: usize,
}
```
```rust
impl Default for BisectLineSearch2D {
    fn default() -> Self {
        Self {
            alpha0: 1.0,
            shrink: 0.5,
            min_alpha: 1e-6,
            bisect_iters: 12,
        }
    }
}
```
```rust
impl<S: SurfaceGeom> LineSearch2D<S> for BisectLineSearch2D {
    fn search(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        step: (f64, f64),
        dom_pol: &dyn SurfaceDomainPolicy,
        opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64, f64, f64)> {
        let du = srf.domain_u();
        let dv = srf.domain_v();

        let (u, v) = uv;
        let d2_old = srf.eval_point(u, v).distance_square(&p);

        // 1) find any decreasing alpha and build bracket
        let mut alpha_hi = self.alpha0;
        let mut alpha_lo = 0.0; // last failing alpha
        let mut found = None;

        for _ in 0..opt.line_search_iters.max(1) {
            let uu = dom_pol.normalize_u(du, u + alpha_hi * step.0);
            let vv = dom_pol.normalize_v(dv, v + alpha_hi * step.1);
            let d2 = srf.eval_point(uu, vv).distance_square(&p);

            if d2 <= d2_old {
                found = Some((uu, vv, d2, alpha_hi));
                break;
            }

            alpha_lo = alpha_hi;
            alpha_hi *= self.shrink;
            if alpha_hi < self.min_alpha {
                break;
            }
        }

        let mut best = found?; // no descent at all

        // 2) if we don't have a real bracket, done
        if alpha_lo <= 0.0 {
            return Some(best);
        }

        // bracket: [alpha_hi (success), alpha_lo (fail)]
        let mut a = best.3; // success
        let mut b = alpha_lo; // fail

        for _ in 0..self.bisect_iters {
            let m = 0.5 * (a + b);

            let uu = dom_pol.normalize_u(du, u + m * step.0);
            let vv = dom_pol.normalize_v(dv, v + m * step.1);
            let d2 = srf.eval_point(uu, vv).distance_square(&p);

            if d2 <= d2_old {
                a = m;
                if d2 < best.2 {
                    best = (uu, vv, d2, m);
                }
            } else {
                b = m;
            }

            if (b - a).abs() < 1e-12 {
                break;
            }
        }

        Some(best)
    }
}
```
```rust
pub trait SurfaceStepPolicy<S: SurfaceGeom> {
    /// Returns proposed step (du,dv) or None if cannot compute.
    fn propose_step(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64)>;
}
```
```rust
pub struct Newton2ndStep;
impl<S: SurfaceGeom> SurfaceStepPolicy<S> for Newton2ndStep {
    fn propose_step(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64)> {
        let (u, v) = uv;

        let ders = srf.evaluate_derivatives_nder(u, v, 2);
        if ders.len() < 3 || ders[0].len() < 3 {
            return None;
        }

        let s = ders[0][0];
        let su = ders[1][0];
        let sv = ders[0][1];
        let suu = ders[2][0];
        let suv = ders[1][1];
        let svv = ders[0][2];

        let q = Point3D::new(s.x, s.y, s.z);
        let r = q - p;

        let ru = su.dot_pt(&r);
        let rv = sv.dot_pt(&r);

        let uu = su.dot(&su) + r.dot_vec(&suu);
        let uvv = su.dot(&sv) + r.dot_vec(&suv);
        let vv = sv.dot(&sv) + r.dot_vec(&svv);

        let det = uu * vv - uvv * uvv;
        let eps = 1e-14;
        if !det.is_finite() || det.abs() < eps {
            return None;
        }

        let mut du = (-ru * vv + rv * uvv) / det;
        let mut dv = (-rv * uu + ru * uvv) / det;

        // clamp step
        let du_dom = srf.domain_u();
        let dv_dom = srf.domain_v();
        let len_u = du_dom.len().abs().max(1e-30);
        let len_v = dv_dom.len().abs().max(1e-30);

        let max_du = opt.newton_step_rel_u * len_u;
        let max_dv = opt.newton_step_rel_v * len_v;

        if du > max_du {
            du = max_du;
        }
        if du < -max_du {
            du = -max_du;
        }
        if dv > max_dv {
            dv = max_dv;
        }
        if dv < -max_dv {
            dv = -max_dv;
        }

        Some((du, dv))
    }
}
```
```rust
pub struct GaussNewtonStep;

impl<S: SurfaceGeom> SurfaceStepPolicy<S> for GaussNewtonStep {
    fn propose_step(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        _opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64)> {
        let (u, v) = uv;

        let ders = srf.evaluate_derivatives_nder(u, v, 1);
        if ders.len() < 2 || ders[0].len() < 2 {
            return None;
        }

        let s = ders[0][0];
        let su = ders[1][0];
        let sv = ders[0][1];

        let q = Point3D::new(s.x, s.y, s.z);
        let r = q - p;

        let gu = su.dot_pt(&r);
        let gv = sv.dot_pt(&r);

        let eps = 1e-14;
        let a = su.dot(&su) + eps;
        let b = su.dot(&sv);
        let c = sv.dot(&sv) + eps;

        let det = a * c - b * b;
        if !det.is_finite() || det.abs() < eps {
            return None;
        }

        let du = (-gu * c + gv * b) / det;
        let dv = (-gv * a + gu * b) / det;

        Some((du, dv))
    }
}
```
```rust
pub struct SurfacePointInverter<S: SurfaceGeom> {
    pub seed: Arc<dyn SurfaceSeedStrategy<S>>,
    pub refiner: Arc<dyn SurfaceLocalRefiner<S>>,
    pub dom_pol: Arc<dyn SurfaceDomainPolicy>,
    pub fallback_refiner: Arc<dyn SurfaceLocalRefiner<S>>, // GN fallback

    /// primary stepper (Newton 2nd)
    pub step_primary: Arc<dyn SurfaceStepPolicy<S>>,
    /// fallback stepper (GN)
    pub step_fallback: Arc<dyn SurfaceStepPolicy<S>>,

    /// global line search
    pub line_search: Arc<dyn LineSearch2D<S>>,
}
```
```rust
impl<S: SurfaceGeom> Default for SurfacePointInverter<S> {
    fn default() -> Self {
        Self {
            seed: Arc::new(DefaultSurfaceSeedStrategy),
            refiner: Arc::new(Newton2ndLineSearchRefiner),
            dom_pol: Arc::new(ClampSurfaceDomain),
            fallback_refiner: Arc::new(GaussNewtonDampedRefiner),
            step_primary: Arc::new(Newton2ndStep),
            step_fallback: Arc::new(GaussNewtonStep),
            line_search: Arc::new(BisectLineSearch2D::default()),
        }
    }
}
```
```rust
impl<S: SurfaceGeom> SurfacePointInverter<S> {
    pub fn with_periodic_domain(mut self) -> Self {
        self.dom_pol = Arc::new(WrapSurfaceDomain);
        self
    }
```
```rust
    pub fn fork_with_seed(&self, seed: Arc<dyn SurfaceSeedStrategy<S>>) -> Self {
        Self {
            seed,
            refiner: self.refiner.clone(),
            dom_pol: self.dom_pol.clone(),
            fallback_refiner: self.fallback_refiner.clone(),
            step_primary: self.step_primary.clone(),
            step_fallback: self.step_fallback.clone(),
            line_search: self.line_search.clone(),
        }
    }
```
```rust
    pub fn invert(
        &self,
        srf: &S,
        p: Point3D,
        uv0: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> SurfacePointInversionResult {
        let du = srf.domain_u();
        let dv = srf.domain_v();
        let tol2 = opt.tol * opt.tol;

        let seeds = self.seed.build_seeds(srf, p, uv0, opt);
        if seeds.is_empty() {
            return SurfacePointInversionResult::fail(
                du,
                dv,
                uv0.0,
                uv0.1,
                SurfaceInversionFailReason::NoSeed,
            );
        }

        let mut best = SurfacePointInversionResult::fail(
            du,
            dv,
            uv0.0,
            uv0.1,
            SurfaceInversionFailReason::NoSeed,
        );

        // 1) primary (Newton2ndStep + line search)
        for &s in &seeds {
            let r = self.solve_with_stepper(srf, p, s, opt, self.step_primary.as_ref());
            if r.dist2 < best.dist2 {
                best = r;
            }
            if best.ok && best.dist2 <= tol2 {
                return best;
            }
        }

        // 2) fallback (GN step + line search)
        if opt.gn_fallback && (!best.ok || best.dist2 > tol2) {
            for &s in &seeds {
                let r = self.solve_with_stepper(srf, p, s, opt, self.step_fallback.as_ref());
                if r.dist2 < best.dist2 {
                    best = r;
                }
                if best.ok && best.dist2 <= tol2 {
                    return best;
                }
            }
        }

        // 3) after-failure line-search only improvement (optional)
        if opt.line_search_mode == SurfaceLineSearchMode::AfterFailure && best.dist2.is_finite() {
            if let Some((uu, vv, g2)) = self.line_search_only_improve(srf, p, (best.u, best.v), opt)
            {
                if g2 < best.dist2 {
                    best.u = uu;
                    best.v = vv;
                    best.dist2 = g2;
                    best.ok = g2 <= tol2;
                    if best.ok {
                        best.reason = SurfaceInversionFailReason::None;
                    }
                }
            }
        }

        best
    }
```
```rust
    fn solve_with_stepper(
        &self,
        srf: &S,
        p: Point3D,
        seed: (f64, f64),
        opt: &SurfacePointInversionOptions,
        stepper: &dyn SurfaceStepPolicy<S>,
    ) -> SurfacePointInversionResult {
        let du_dom = srf.domain_u();
        let dv_dom = srf.domain_v();
        let tol2 = opt.tol * opt.tol;

        let mut u = self.dom_pol.normalize_u(du_dom, seed.0);
        let mut v = self.dom_pol.normalize_v(dv_dom, seed.1);

        let mut best_u = u;
        let mut best_v = v;
        let mut best_g = srf.eval_point(u, v).distance_square(&p);

        let mut last_g = best_g;
        let mut iters = 0usize;

        for k in 0..opt.max_newton.max(1) {
            iters = k + 1;

            let g = srf.eval_point(u, v).distance_square(&p);
            if g < best_g {
                best_g = g;
                best_u = u;
                best_v = v;
            }
            if best_g <= tol2 {
                return SurfacePointInversionResult {
                    ok: true,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::None,
                };
            }

            let step = match stepper.propose_step(srf, p, (u, v), opt) {
                Some(st) => st,
                None => {
                    return SurfacePointInversionResult {
                        ok: best_g <= tol2,
                        u: best_u,
                        v: best_v,
                        dist2: best_g,
                        iters,
                        used_seed: seed,
                        reason: SurfaceInversionFailReason::NoDerivatives,
                    };
                }
            };

            // *** 핵심: line search 적용 정책 ***
            let accepted =
                self.line_search
                    .search(srf, p, (u, v), step, self.dom_pol.as_ref(), opt);

            let Some((uu, vv, g_new, _alpha)) = accepted else {
                return SurfacePointInversionResult {
                    ok: best_g <= tol2,
                    u: best_u,
                    v: best_v,
                    dist2: best_g,
                    iters,
                    used_seed: seed,
                    reason: SurfaceInversionFailReason::NoDescent,
                };
            };

            u = uu;
            v = vv;

            if (last_g - g_new).abs() < 1e-18 {
                break;
            }
            last_g = g_new;
        }

        SurfacePointInversionResult {
            ok: best_g <= tol2,
            u: best_u,
            v: best_v,
            dist2: best_g,
            iters,
            used_seed: seed,
            reason: SurfaceInversionFailReason::MaxIter,
        }
    }
```
```rust
    fn line_search_only_improve(
        &self,
        srf: &S,
        p: Point3D,
        uv: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> Option<(f64, f64, f64)> {
        let (u, v) = uv;

        // need first derivatives for gradient
        let ders = srf.evaluate_derivatives_nder(u, v, 1);
        if ders.len() < 2 || ders[0].len() < 2 {
            return None;
        }

        let s = ders[0][0];
        let su = ders[1][0];
        let sv = ders[0][1];

        let q = Point3D::new(s.x, s.y, s.z);
        let r = q - p;

        let gu = su.dot_pt(&r);
        let gv = sv.dot_pt(&r);

        // step = -grad, scaled
        let du_dom = srf.domain_u();
        let dv_dom = srf.domain_v();
        let len_u = du_dom.len().abs().max(1e-30);
        let len_v = dv_dom.len().abs().max(1e-30);

        let mut du = -gu;
        let mut dv = -gv;

        let nrm = (du * du + dv * dv).sqrt();
        if !nrm.is_finite() || nrm < 1e-30 {
            return None;
        }

        let scale_u = opt.fallback_step_rel * len_u;
        let scale_v = opt.fallback_step_rel * len_v;

        du = du / nrm * scale_u;
        dv = dv / nrm * scale_v;

        let got = self
            .line_search
            .search(srf, p, (u, v), (du, dv), self.dom_pol.as_ref(), opt)?;

        Some((got.0, got.1, got.2))
    }
}
```
```rust
use crate::core::point_grid::SharedPointGrid;

#[derive(Clone, Debug)]
pub struct PointGridSurfaceSeedStrategy {
    pub grid: SharedPointGrid,
    /// best k개를 seed 후보로 쓸지 (보통 1~8 권장)
    pub k_best: usize,
    /// grid seed 주변에 neighborhood seeds를 얼마나 추가할지
    pub add_neighborhood: bool,
}
```
```rust
impl PointGridSurfaceSeedStrategy {
    pub fn new(grid: SharedPointGrid) -> Self {
        Self {
            grid,
            k_best: 4,
            add_neighborhood: true,
        }
    }
}
```
```rust
impl<S: SurfaceGeom> SurfaceSeedStrategy<S> for PointGridSurfaceSeedStrategy {
    fn build_seeds(
        &self,
        srf: &S,
        p: Point3D,
        uv0: (f64, f64),
        opt: &SurfacePointInversionOptions,
    ) -> Vec<(f64, f64)> {
        let du = srf.domain_u();
        let dv = srf.domain_v();
        let len_u = du.len().abs().max(1e-30);
        let len_v = dv.len().abs().max(1e-30);

        let mut seeds: Vec<(f64, f64)> = Vec::new();

        // 0) uv0는 항상 포함(기존 로직 유지)
        let u0 = du.clamp(uv0.0);
        let v0 = dv.clamp(uv0.1);
        seeds.push((u0, v0));

        // 1) grid에서 best k개 seed 추출
        let k = self.k_best.max(1);
        for (uv, _d2) in self.grid.0.nearest_k_uv(p, k) {
            seeds.push((du.clamp(uv.0), dv.clamp(uv.1)));

            if self.add_neighborhood {
                let su = opt.seed_spread_u * len_u;
                let sv = opt.seed_spread_v * len_v;

                let cand_u = [uv.0, uv.0 + su, uv.0 - su, uv.0 + 0.5 * su, uv.0 - 0.5 * su];
                let cand_v = [uv.1, uv.1 + sv, uv.1 - sv, uv.1 + 0.5 * sv, uv.1 - 0.5 * sv];
                for &uu in &cand_u {
                    for &vv in &cand_v {
                        seeds.push((du.clamp(uu), dv.clamp(vv)));
                    }
                }
            }
        }

        // 2) (옵션) 기존 coarse scan도 함께 쓰고 싶으면:
        // - DefaultSurfaceSeedStrategy를 합성하거나,
        // - 여기서 opt.scan_u/scan_v 로직을 그대로 복사하면 됨.
        // 지금은 "grid를 최대한 살린" 버전이라 grid seed 중심으로만 간다.
        // sort & dedup
        seeds.sort_by(|a, b| {
            let ou = a.0.partial_cmp(&b.0).unwrap_or(core::cmp::Ordering::Equal);
            if ou != core::cmp::Ordering::Equal {
                ou
            } else {
                a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal)
            }
        });
        seeds.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-12 && (a.1 - b.1).abs() < 1e-12);

        // cap
        if seeds.len() > opt.max_seed_count {
            seeds.truncate(opt.max_seed_count);
        }

        // seed far guard (optional)
        if let Some(k) = opt.seed_far_guard {
            let mut filtered = Vec::new();
            for &uv in &seeds {
                let g = srf.eval_point(uv.0, uv.1).distance_square(&p);
                if g <= k * k {
                    filtered.push(uv);
                }
            }
            if !filtered.is_empty() {
                return filtered;
            }
        }
        seeds
    }
}
```
---


