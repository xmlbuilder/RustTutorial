# Grid2D 
Grid2D sampling for point inversion
Grid2D는 초기값을 빠르게, 안정적으로 잡아주는 베이스가 됩니다.  
전반적으로 구조는 괜찮고, 몇 가지 수식적·수치적 안정성 포인트와 실용 개선을 제안.

## Correctness and API alignment
- Interval semantics: Interval의 min/max가 실제 표면 유효 도메인과 일치하는지 확인 필요.  
    일부 라이브러리는 클램프된 구간이 $[U[p]], U[n+1]]$ 처럼 내부적으로 정의됩니다.  
    이 차이가 있으면 경계에서 평가가 실패하거나 반복 knot 근처에서 잡음이 커질 수 있습니다.
- Loop bounds: 현재 i, j를 0..=nu, 0..=nv로 순회합니다.
- 의도: nu, nv **세그먼트 수** 로 균일 분할해 **nu+1, nv+1개의 샘플** 을 만들려는 것.
- 대안: nu, nv를 **샘플 수** 로 정의하고 0..nu로 순회하면 혼장 방지에 더 직관적입니다.
- on_clamp01 미사용: 파일 상단에 on_clamp01가 import되어 있는데 사용하지 않습니다.
- 개선: t_u, t_v를 안전하게 clamp하거나, Interval이 역전 (min>max) 인 경우를 방어 필요.
```rust
let (u0, u1) = if du.min <= du.max { (du.min, du.max) } else { (du.max, du.min) };
let (v0, v1) = if dv.min <= dv.max { (dv.min, dv.max) } else { (dv.max, dv.min) };
```
```rust
pub fn on_clamp_to_domain(mut x: f64, dom: Interval) -> f64 {
    if x < dom.min() {
        x = dom.min();
    }
    if x > dom.max() {
        x = dom.max();
    }
    x
}
```
```rust
for i in 0..=self.nu {
    let u = u0 + (u1 - u0) * (i as f64 / self.nu as f64);
    on_clamp_to_domain(u, du);
    for j in 0..=self.nv {
        let v = v0 + (v1 - v0) * (j as f64 / self.nv as f64);
        on_clamp_to_domain(v, dv);

```

- Trait design: ParamSurface는 최소 인터페이스로 적절합니다.
- 확장: 초기값 이후의 정밀화까지 염두에 두면, 다음 메서드를 추가하는 게 좋습니다.
- `eval_du_dv(u, v)`: 1차 도함수 제공
- `closest_on_iso_u(u, p)`, `closest_on_iso_v(v, p)`: iso-curve 근사 최근접

## Numerical considerations
- Sampling jitter for symmetry breaking: 완전 균일 격자는 고차 곡면의 반복 구조에서 동일한 거리 값이  
    다수 생겨 초기 선택이 불안정할 수 있습니다.
- 개선: 작은 지터를 넣어 타이-브레이크를 줄임
```rust
let eps = 1e-9;
let t_u = (i as f64 + eps * (i as f64).sin()) / self.nu as f64;
let t_v = (j as f64 + eps * (j as f64).cos()) / self.nv as f64;
```
- Adaptive refinement (2–3 단계): 한 번의 격자만으로 부족한 경우가 많습니다.
- 전략: coarse → refine around best → fine
- 1단계: 9×9 또는 11×11.
- 2단계: 1단계 최적점 주변을 국소적인 박스로 재샘플(예: u±Δu, v±Δv).
- 3단계: 도함수 기반 1–3 회 뉴턴 스텝.
- Local Newton step: 초기값을 얻은 뒤 수렴 가속.
- 목표 함수: 
- Gradient: $\nabla \phi =\left[ \begin{matrix}(S(u,v)-p)^{\top }S_u\\ ; \quad (S(u,v)-p)^{\top }S_v\end{matrix}\right]$ 
- Hessian (Gauss–Newton 근사):

$$
H\approx \left[ \begin{matrix}S_u^{\top }S_u&S_u^{\top }S_v\\ ; \quad S_v^{\top }S_u&S_v^{\top }S_v\end{matrix}\right] 
$$

- 업데이트: $[\Delta u,\Delta v]^{\top }=-H^{-1}\nabla \phi$ , 이후 $u,v$ 를 도메인으로 클램프.
- 라인서치: Armijo 백트래킹으로 안정화.
- Distance accumulation: d2 계산에서 overflow 위험은 적지만, 큰 좌표 스케일에서는 float 정밀도 문제를 줄이기 위해  
    p를 원점 이동(센터링)하는 것도 도움됩니다.

## Trimming and point-in-polygon
- Polygon winding and degeneracy: 현재 ray-casting 구현은 간단하고 실용적입니다.
- 개선: 세로 엣지(pj.y == pi.y)나 매우 작은 높이 차(pj.y - pi.y ≈ 0)에 대한 분모 안정화는 이미 1e-15로 처리했지만,  
    정확한 경계 포함 규칙(on-edge 포함/제외)을 명확히 처리 필요.  
    경계 포함이 필요하면 교차 판정에 등호 케이스를 별도 처리 필요.
- Multiple trim loops and holes: point_in_any_polygon은 외곽/홀 구분 없이 **하나라도 안에 있으면** 으로 처리합니다.
- 개선: 일반적인 트림은 **외곽 루프 안 AND 어떤 홀 루프 밖** 규칙이 필요합니다.
- 제안: Polygon2D에 is_outer 플래그를 둘 필요 있음.
- inside = in_any_outer && !in_any_hole
- UV normalization: 트림 루프 좌표가 절대 도메인 좌표인지, 정규화(0–1)인지 프로젝트 규약을 일관되게 맞출 필요 있음.

## Enhancements that help point inversion
- Label: 초기 coarse grid에서 후보를 K개(예: 3–5개) 뽑아 병렬로 각각 국소 뉴턴을 수행한 뒤 최종 최적을 선택하면  
    지역 최적 함정에서 벗어나기 쉽습니다.
- Label: 샘플링 시 iso-curve 근사 루틴을 병행하면 빠르게 좋은 후보를 얻습니다.
- 방법: U 고정 후 V를 1D로 줄여 recent point on curve 문제로 접근(1D 라인서치).
- Label: 도메인 스팬 기반 step 설정. numeric_partials에서 구현한  **지역 knot span 폭 기반 스텝** 을  
    Newton 업데이트의 Δu, Δv 제한에도 사용.
- Label: Plateau 감지. Δu, Δv가 반복적으로 아주 작고, 감소가 미미하면 스텝 축소나 다른 초기 후보로 교체.

## Suggested code refinements
- Adaptive two-stage sampler
```rust
pub fn best_initial_uv_adaptive<S>(&self, surface: &S, p: &Point3D) -> (f64, f64)
where
    S: ParamSurface,
{
    let du = surface.domain_u();
    let dv = surface.domain_v();
    let (u0, u1) = if du.min <= du.max { (du.min, du.max) } else { (du.max, du.min) };
    let (v0, v1) = if dv.min <= dv.max { (dv.min, dv.max) } else { (dv.max, dv.min) };

    // 1단계: coarse
    let (mut u_best, mut v_best, mut best_d2) = (u0, v0, f64::INFINITY);
    for i in 0..=self.nu {
        let t_u = i as f64 / self.nu as f64;
        let u = u0 + (u1 - u0) * t_u;
        for j in 0..=self.nv {
            let t_v = j as f64 / self.nv as f64;
            let v = v0 + (v1 - v0) * t_v;
            let q = surface.eval(u, v);
            let d2 = (q.x - p.x).powi(2) + (q.y - p.y).powi(2) + (q.z - p.z).powi(2);
            if d2 < best_d2 {
                best_d2 = d2;
                u_best = u;
                v_best = v;
            }
        }
    }

    // 2단계: refine around best
    let du_box = 0.1 * (u1 - u0);
    let dv_box = 0.1 * (v1 - v0);
    let ur0 = (u_best - du_box).max(u0);
    let ur1 = (u_best + du_box).min(u1);
    let vr0 = (v_best - dv_box).max(v0);
    let vr1 = (v_best + dv_box).min(v1);

    for i in 0..=self.nu {
        let t_u = i as f64 / self.nu as f64;
        let u = ur0 + (ur1 - ur0) * t_u;
        for j in 0..=self.nv {
            let t_v = j as f64 / self.nv as f64;
            let v = vr0 + (vr1 - vr0) * t_v;
            let q = surface.eval(u, v);
            let d2 = (q.x - p.x).powi(2) + (q.y - p.y).powi(2) + (q.z - p.z).powi(2);
            if d2 < best_d2 {
                best_d2 = d2;
                u_best = u;
                v_best = v;
            }
        }
    }

    (u_best, v_best)
}
```

### Armijo backtracking for Newton refinement
- Gradient and step: 위 수식에 따라 ∇φ와 H를 구성하고, 실패 시 그라디언트 하강으로 폴백.
- Backtracking condition:

$$
\phi (u+\alpha \Delta u,v+\alpha \Delta v)\leq \phi (u,v)+c_1\alpha \nabla \phi ^{\top }[\Delta u,\Delta v]
$$

- Practical defaults: c1=1e-4, τ=0.5, max_backtracks=20.

- Bottom line
    - 핵심 안정화: 도메인 확인, 균일 그리드의 적응적 리파인, 뉴턴 + Armijo 결합, 트림 로직의 외곽/홀 구분.
    - 수식적으로: 목적함수 $φ(u,v)=½‖S(u,v)−p‖²$ 기반의 $∇φ$, H(가우스–뉴턴 근사)로 업데이트하면 초기값에서 빠르게 수렴합니다.
    - 실용 팁: 후보 K개 병렬 정밀화, 지역 knot span 폭 기반 스텝 제한, plateau 감지로 튼튼한 point inversion이 됩니다.



### Newton refinement for point inversion on ParamSurface
- 아래 루틴은 ParamSurface만으로 동작하도록, 수치 도함수와 Armijo 백트래킹 라인서치를 포함한 2D 뉴턴(가우스–뉴턴 근사) 정밀화를 제공합니다.
- 초기값에서 시작해 표면 S(u,v)가 목표 점 p에 가장 가까워지도록 (u,v)를 갱신합니다.

#### 공개 API
```rust
/// 뉴턴 정밀화 옵션
#[derive(Debug, Clone)]
pub struct NewtonRefineOptions {
    pub max_iters: usize,   // 최대 반복 횟수
    pub tol_uv: f64,        // 파라미터 스텝 허용 오차 (Δu, Δv)
    pub tol_phi: f64,       // 목적함수 감소 허용 오차
    pub c1: f64,            // Armijo 조건 상수 (보통 1e-4)
    pub tau: f64,           // 백트래킹 축소율 (보통 0.5)
    pub max_backtracks: usize, // 최대 백트래킹 횟수
}

impl Default for NewtonRefineOptions {
    fn default() -> Self {
        Self {
            max_iters: 20,
            tol_uv: 1e-10,
            tol_phi: 1e-12,
            c1: 1e-4,
            tau: 0.5,
            max_backtracks: 20,
        }
    }
}
```
```rust
/// 뉴턴 정밀화 결과
#[derive(Debug, Clone)]
pub struct NewtonRefineResult {
    pub u: f64,
    pub v: f64,
    pub iters: usize,
    pub converged: bool,
    pub final_phi: f64,
}
```


### 핵심 함수
```rust
use crate::math::prelude::{Point3D, Vector3D};
use super::ParamSurface; // Grid2D와 동일 모듈 트레이트
use crate::math::interval::Interval;

/// 목적함수 φ(u,v) = 1/2 ||S(u,v) - p||^2
fn phi<S: ParamSurface>(surf: &S, p: &Point3D, u: f64, v: f64) -> f64 {
    let q = surf.eval(u, v);
    let dx = q.x - p.x;
    let dy = q.y - p.y;
    let dz = q.z - p.z;
    0.5 * (dx*dx + dy*dy + dz*dz)
}

/// 수치 도함수: S_u, S_v (중앙차분, 경계에서는 일방차분)
fn numeric_partials<S: ParamSurface>(
    surf: &S,
    u: f64,
    v: f64,
    du_dom: Interval,
    dv_dom: Interval,
) -> (Vector3D, Vector3D)
{
    // 도메인 길이 기반 스텝 (너무 작지 않게 eps 보장)
    let umin = du_dom.min.min(du_dom.max);
    let umax = du_dom.min.max(du_dom.max);
    let vmin = dv_dom.min.min(dv_dom.max);
    let vmax = dv_dom.min.max(dv_dom.max);

    let du = ((umax - umin).abs() * 1e-6).max(1e-12);
    let dv = ((vmax - vmin).abs() * 1e-6).max(1e-12);

    let u_plus = (u + du).min(umax);
    let u_minus = (u - du).max(umin);
    let v_plus = (v + dv).min(vmax);
    let v_minus = (v - dv).max(vmin);

    let q_u_plus = surf.eval(u_plus, v);
    let q_u_minus = surf.eval(u_minus, v);
    let denom_u = (u_plus - u_minus).max(1e-15);
    let su = Vector3D::new(
        (q_u_plus.x - q_u_minus.x) / denom_u,
        (q_u_plus.y - q_u_minus.y) / denom_u,
        (q_u_plus.z - q_u_minus.z) / denom_u,
    );

    let q_v_plus = surf.eval(u, v_plus);
    let q_v_minus = surf.eval(u, v_minus);
    let denom_v = (v_plus - v_minus).max(1e-15);
    let sv = Vector3D::new(
        (q_v_plus.x - q_v_minus.x) / denom_v,
        (q_v_plus.y - q_v_minus.y) / denom_v,
        (q_v_plus.z - q_v_minus.z) / denom_v,
    );

    (su, sv)
}
```
```rust
/// 2x2 선형 시스템 해법 (Cramer 또는 직접 역행렬)
fn solve_2x2(a11: f64, a12: f64, a21: f64, a22: f64, b1: f64, b2: f64) -> Option<(f64, f64)> {
    let det = a11 * a22 - a12 * a21;
    if det.abs() < 1e-20 {
        return None;
    }
    let inv_det = 1.0 / det;
    let du = ( a22 * b1 - a12 * b2) * inv_det;
    let dv = (-a21 * b1 + a11 * b2) * inv_det;
    Some((du, dv))
}
```
```rust
/// Armijo 백트래킹 라인서치: φ(u+αΔu, v+αΔv) ≤ φ(u,v) + c1 α ∇φ^T [Δu,Δv]
fn backtracking_armijo<S: ParamSurface>(
    surf: &S,
    p: &Point3D,
    u: f64,
    v: f64,
    du: f64,
    dv: f64,
    grad_u: f64,
    grad_v: f64,
    opts: &NewtonRefineOptions,
    dom_u: Interval,
    dom_v: Interval,
) -> Option<(f64, f64, f64)> {
    let mut alpha = 1.0;
    let phi0 = phi(surf, p, u, v);
    let dir_dot_grad = grad_u * du + grad_v * dv; // ∇φ^T s
    for _ in 0..opts.max_backtracks {
        let un = (u + alpha * du).clamp(dom_u.min.min(dom_u.max), dom_u.min.max(dom_u.max));
        let vn = (v + alpha * dv).clamp(dom_v.min.min(dom_v.max), dom_v.min.max(dom_v.max));
        let phin = phi(surf, p, un, vn);
        let rhs = phi0 + opts.c1 * alpha * dir_dot_grad;
        if phin <= rhs {
            return Some((un, vn, phin));
        }
        alpha *= opts.tau;
    }
    None
}
```
```rust
/// 뉴턴(가우스–뉴턴) 정밀화 본체
pub fn refine_point_inversion_newton<S: ParamSurface>(
    surface: &S,
    p: &Point3D,
    u0: f64,
    v0: f64,
    opts: NewtonRefineOptions,
) -> NewtonRefineResult {
    let du_dom = surface.domain_u();
    let dv_dom = surface.domain_v();

    // 초기값 클램프
    let mut u = u0.clamp(du_dom.min.min(du_dom.max), du_dom.min.max(du_dom.max));
    let mut v = v0.clamp(dv_dom.min.min(dv_dom.max), dv_dom.min.max(dv_dom.max));

    let mut phi_val = phi(surface, p, u, v);

    let mut converged = false;

    for iter in 0..opts.max_iters {
        // 1) 수치 도함수
        let (su, sv) = numeric_partials(surface, u, v, du_dom, dv_dom);
        let q = surface.eval(u, v);
        let r = Vector3D::new(q.x - p.x, q.y - p.y, q.z - p.z); // residual

        // 2) ∇φ = [ r·S_u ; r·S_v ]
        let grad_u = r.x * su.x + r.y * su.y + r.z * su.z;
        let grad_v = r.x * sv.x + r.y * sv.y + r.z * sv.z;

        // 3) H ≈ [[ S_u·S_u, S_u·S_v ], [ S_v·S_u, S_v·S_v ]]
        let suu = su.x * su.x + su.y * su.y + su.z * su.z;
        let suv = su.x * sv.x + su.y * sv.y + su.z * sv.z;
        let svv = sv.x * sv.x + sv.y * sv.y + sv.z * sv.z;

        // 4) Δ = - H^{-1} ∇φ
        let rhs1 = -grad_u;
        let rhs2 = -grad_v;

        let step_opt = solve_2x2(suu, suv, suv, svv, rhs1, rhs2);
        let (mut du_step, mut dv_step) = match step_opt {
            Some(s) => s,
            None => {
                // H가 특이하면 그래디언트 하강으로 폴백
                let norm_g = (grad_u * grad_u + grad_v * grad_v).sqrt();
                if norm_g < 1e-20 {
                    converged = true;
                    return NewtonRefineResult { u, v, iters: iter, converged, final_phi: phi_val };
                }
                let scale = -1.0 / norm_g;
                (grad_u * scale, grad_v * scale)
            }
        };

        // 5) 스텝 제한: 도메인 폭의 작은 비율
        let u_width = (du_dom.max - du_dom.min).abs().max(1.0);
        let v_width = (dv_dom.max - dv_dom.min).abs().max(1.0);
        let max_scale = 0.25; // 도메인 폭의 1/4 이상 움직이지 않도록 제한
        du_step = du_step.clamp(-max_scale * u_width, max_scale * u_width);
        dv_step = dv_step.clamp(-max_scale * v_width, max_scale * v_width);

        // 6) Armijo 백트래킹
        if let Some((u_new, v_new, phi_new)) = backtracking_armijo(
            surface, p, u, v, du_step, dv_step, grad_u, grad_v, &opts, du_dom, dv_dom
        ) {
            // 수용
            let delta_u = (u_new - u).abs();
            let delta_v = (v_new - v).abs();
            u = u_new;
            v = v_new;
            phi_val = phi_new;

            // 수렴 체크
            if delta_u <= opts.tol_uv && delta_v <= opts.tol_uv {
                converged = true;
                return NewtonRefineResult { u, v, iters: iter + 1, converged, final_phi: phi_val };
            }
            if phi_val <= opts.tol_phi {
                converged = true;
                return NewtonRefineResult { u, v, iters: iter + 1, converged, final_phi: phi_val };
            }
        } else {
            // 백트래킹 실패 → 작은 스텝으로 한 번 시도 후 종료
            let alpha_min = 1e-6;
            let u_try = (u + alpha_min * du_step).clamp(du_dom.min.min(du_dom.max), du_dom.min.max(du_dom.max));
            let v_try = (v + alpha_min * dv_step).clamp(dv_dom.min.min(dv_dom.max), dv_dom.min.max(dv_dom.max));
            let phi_try = phi(surface, p, u_try, v_try);
            if phi_try < phi_val {
                u = u_try;
                v = v_try;
                phi_val = phi_try;
            }
            break;
        }
    }

    NewtonRefineResult { u, v, iters: opts.max_iters, converged, final_phi: phi_val }
}
```

### 사용 예

```rust
let opts = NewtonRefineOptions::default();
let init_uv = grid2d.best_initial_uv(&surface, &p);
let result = refine_point_inversion_newton(&surface, &p, init_uv.0, init_uv.1, opts);

if result.converged {
    // result.u, result.v 사용
} else {
    // 필요 시 다른 초기값으로 재시도 또는 샘플링 리파인
}
```

### 포인트
- ParamSurface에 도함수가 없으므로 중앙차분으로 S_u, S_v를 계산했습니다. 내부 knot 정보를 모를 때의 안전한 기본값입니다.
- Hessian은 가우스–뉴턴 근사. 거리 제곱 최소화에 적절하며 안정적입니다.
- Armijo 백트래킹으로 스텝을 자동 축소해 발산을 방지합니다.
- 도메인 폭 기반 스텝 제한으로 큰 점프를 억제합니다.
- Hessian 특이 시 그래디언트 하강으로 폴백합니다.

```rust
pub fn point_inversion(
    &self,
    p: Point3D,
    u_hint: Option<f64>,
    v_hint: Option<f64>,
    opts: NewtonRefineOptions,
) -> (bool, f64, f64, Point3D) {

    // 1) 힌트 있으면 그대로 사용
    let mut uv_candidates: Vec<(f64, f64)> = Vec::new();
    if let (Some(u), Some(v)) = (u_hint, v_hint) {
        uv_candidates.push((u, v));
    } else {
        // 2) 초기 Grid2D coarse sampling
        let g5 = Grid2D::new(5, 5).best_initial_uv(self, &p);
        uv_candidates.push(g5);
    }

    // 3) 점점 더 fine한 grid도 추가
    uv_candidates.push(Grid2D::new(9, 9).best_initial_uv(self, &p));
    uv_candidates.push(Grid2D::new(17, 17).best_initial_uv(self, &p));

    // 도메인
    let du = self.domain_u();
    let dv = self.domain_v();

    // 4) 후보 UV들에 대해 Newton refine 시도
    let mut best = None;
    let mut best_phi = f64::INFINITY;

    for (u0, v0) in uv_candidates {
        let r = refine_point_inversion_newton(self, &p, u0, v0, opts.clone());
        if r.final_phi < best_phi {
            best_phi = r.final_phi;
            best = Some(r);
        }
        if r.converged {
            break;
        }
    }

    // 5) 결과 결정
    if let Some(r) = best {
        let q = self.eval(r.u, r.v);
        return (r.converged, r.u, r.v, q);
    }

    // 실패
    (false, du.min, dv.min, self.eval(du.min, dv.min))
}
```
```rust
// src/geom/utils/grid2d.rs

use crate::math::prelude::{Point3D, Vector3D};
use crate::math::interval::Interval;
use crate::geom::surface::Surface;    // 실제 NURBS surface 타입
use crate::math::point2d::Point2D;
use crate::geom::polygon2d::Polygon2D;

/// 최소 인터페이스 정의
pub trait ParamSurface {
    fn domain_u(&self) -> Interval;
    fn domain_v(&self) -> Interval;
    fn eval(&self, u: f64, v: f64) -> Point3D;
}
```
```rust
impl ParamSurface for Surface {
    fn domain_u(&self) -> Interval { self.domain_u() }
    fn domain_v(&self) -> Interval { self.domain_v() }
    fn eval(&self, u: f64, v: f64) -> Point3D { self.eval(u, v) }
}
```
```rust
/// Grid2D: UV 격자 샘플링
#[derive(Debug, Clone)]
pub struct Grid2D {
    pub nu: usize, // U방향 샘플 개수
    pub nv: usize, // V방향 샘플 개수
}
```
```rust
impl Grid2D {
    pub fn new(nu: usize, nv: usize) -> Self {
        Grid2D { nu: nu.max(2), nv: nv.max(2) }
    }
```
```rust
    /// 주어진 점 p에 대해, 그리드 샘플 중 가장 가까운 UV를 찾는다.
    pub fn best_initial_uv<S>(&self, surface: &S, p: &Point3D) -> (f64, f64)
    where S: ParamSurface {
        let du = surface.domain_u();
        let dv = surface.domain_v();
        let (u0, u1) = (du.min.min(du.max), du.min.max(du.max));
        let (v0, v1) = (dv.min.min(dv.max), dv.min.max(dv.max));

        let mut best_u = u0;
        let mut best_v = v0;
        let mut best_d2 = f64::INFINITY;

        for i in 0..=self.nu {
            let u = u0 + (u1 - u0) * (i as f64 / self.nu as f64);
            for j in 0..=self.nv {
                let v = v0 + (v1 - v0) * (j as f64 / self.nv as f64);
                let q = surface.eval(u, v);
                let d2 = (q.x - p.x).powi(2) + (q.y - p.y).powi(2) + (q.z - p.z).powi(2);
                if d2 < best_d2 {
                    best_d2 = d2;
                    best_u = u;
                    best_v = v;
                }
            }
        }
        (best_u, best_v)
    }
```
```rust
    /// 트림 루프 고려 버전
    pub fn best_initial_uv_with_trim<S>(
        &self,
        surface: &S,
        p: &Point3D,
        trim_loops: &[Polygon2D],
    ) -> Option<(f64, f64)>
    where S: ParamSurface {
        if trim_loops.is_empty() {
            return Some(self.best_initial_uv(surface, p));
        }

        let du = surface.domain_u();
        let dv = surface.domain_v();
        let (u0, u1) = (du.min.min(du.max), du.min.max(du.max));
        let (v0, v1) = (dv.min.min(dv.max), dv.min.max(dv.max));

        let mut best_uv: Option<(f64, f64)> = None;
        let mut best_d2 = f64::INFINITY;

        for i in 0..=self.nu {
            let u = u0 + (u1 - u0) * (i as f64 / self.nu as f64);
            for j in 0..=self.nv {
                let v = v0 + (v1 - v0) * (j as f64 / self.nv as f64);
                let uv = Point2D::new(u, v);

                if !point_in_any_polygon(&uv, trim_loops) { continue; }

                let q = surface.eval(u, v);
                let d2 = (q.x - p.x).powi(2) + (q.y - p.y).powi(2) + (q.z - p.z).powi(2);
                if d2 < best_d2 {
                    best_d2 = d2;
                    best_uv = Some((u, v));
                }
            }
        }
        best_uv
    }
}
```
```rust
/// UV가 여러 트림 폴리곤 중 하나라도 안에 있으면 true
fn point_in_any_polygon(p: &Point2D, polys: &[Polygon2D]) -> bool {
    polys.iter().any(|poly| point_in_polygon(p, poly))
}
```
```rust
/// 간단한 point-in-polygon (ray casting)
fn point_in_polygon(p: &Point2D, poly: &Polygon2D) -> bool {
    let mut inside = false;
    let n = poly.points.len();
    if n < 3 { return false; }
    let mut j = n - 1;
    for i in 0..n {
        let pi = &poly.points[i];
        let pj = &poly.points[j];
        let intersect = ((pi.y > p.y) != (pj.y > p.y))
            && (p.x < (pj.x - pi.x) * (p.y - pi.y) / ((pj.y - pi.y) + 1e-15) + pi.x);
        if intersect { inside = !inside; }
        j = i;
    }
    inside
}
```
```rust
use crate::math::prelude::Point3D;
use crate::math::interval::Interval;
use crate::geom::utils::grid2d::{Grid2D, ParamSurface};
use crate::geom::utils::newton_refine::{
    NewtonRefineOptions, NewtonRefineResult, refine_point_inversion_newton,
};
```
```rust
impl Surface {
    /// Point Inversion: 3D 점 p에 대해 표면 상의 최근접 (u,v) 찾기
    ///
    /// 반환: (converged, u, v, q)
    pub fn point_inversion(
        &self,
        p: Point3D,
        u_hint: Option<f64>,
        v_hint: Option<f64>,
        opts: NewtonRefineOptions,
    ) -> (bool, f64, f64, Point3D) {
        // 1) 후보 UV 초기화
        let mut uv_candidates: Vec<(f64, f64)> = Vec::new();

        if let (Some(u), Some(v)) = (u_hint, v_hint) {
            uv_candidates.push((u, v));
        } else {
            // coarse grid
            let g5 = Grid2D::new(5, 5).best_initial_uv(self, &p);
            uv_candidates.push(g5);
        }

        // 2) 더 fine한 grid 후보 추가
        uv_candidates.push(Grid2D::new(9, 9).best_initial_uv(self, &p));
        uv_candidates.push(Grid2D::new(17, 17).best_initial_uv(self, &p));

        // 3) 도메인
        let du = self.domain_u();
        let dv = self.domain_v();

        // 4) 후보 UV들에 대해 Newton refine 시도
        let mut best: Option<NewtonRefineResult> = None;
        let mut best_phi = f64::INFINITY;

        for (u0, v0) in uv_candidates {
            let r = refine_point_inversion_newton(self, &p, u0, v0, opts.clone());
            if r.final_phi < best_phi {
                best_phi = r.final_phi;
                best = Some(r.clone());
            }
            if r.converged {
                best = Some(r);
                break;
            }
        }

        // 5) 결과 결정
        if let Some(r) = best {
            let q = self.eval(r.u, r.v);
            return (r.converged, r.u, r.v, q);
        }

        // 실패 시 fallback
        (false, du.min, dv.min, self.eval(du.min, dv.min))
    }
}
```

### 📌 루틴 설명
- 초기 후보 설정
- 힌트가 있으면 그대로 사용
- 없으면 coarse grid (5×5)에서 최근접 후보 추가
- fine grid 후보 추가
- 9×9, 17×17 grid에서 각각 최근접 후보 추가
- Newton refine
- 각 후보에 대해 refine_point_inversion_newton 실행
- 가장 작은 목적함수 φ 선택, converged면 바로 break
- 결과 반환
- 성공 시 (converged, u, v, q) 반환
- 실패 시 도메인 최소값으로 fallback

## ✅ 요약
- Grid2D로 coarse/fine 후보를 잡고
- Newton refine으로 정밀화
- Point Inversion 전체 루틴이 완성됩니다.

---


## 트림 영역 고려 버전(point_inversion_with_trim)도 확장
### Point inversion with trimming
트림 루프를 고려해 UV 초기 후보를 트림 영역 안에서만 선택하고, 뉴턴 정밀화 중에도 도메인 클램프를 유지하는 확장입니다.  
아래 코드는 앞서 정의한 ParamSurface, Grid2D, Newton refine를 그대로 활용합니다.

```rust
use crate::math::prelude::Point3D;
use crate::geom::polygon2d::Polygon2D;
use crate::geom::utils::grid2d::{Grid2D, ParamSurface};
use crate::geom::utils::newton_refine::{
    NewtonRefineOptions, NewtonRefineResult, refine_point_inversion_newton,
};

impl Surface {
    /// Point inversion with trimming:
    /// - trim_loops: UV 공간상의 트리밍 폴리곤들 (외곽/홀 구분이 없다면,
    ///   우선 "폴리곤 안"을 유효 영역으로 간주합니다)
    ///
    /// 반환: (converged, u, v, q)
    pub fn point_inversion_with_trim(
        &self,
        p: Point3D,
        u_hint: Option<f64>,
        v_hint: Option<f64>,
        trim_loops: &[Polygon2D],
        opts: NewtonRefineOptions,
    ) -> (bool, f64, f64, Point3D) {
        // 1) 후보 UV 초기화
        let mut uv_candidates: Vec<(f64, f64)> = Vec::new();

        // 1-1) 힌트가 트림 안이면 사용
        if let (Some(u), Some(v)) = (u_hint, v_hint) {
            if trim_loops.is_empty() || uv_in_any_trim(u, v, trim_loops) {
                uv_candidates.push((u, v));
            }
        }

        // 2) Grid2D 초기 후보 (트림 고려)
        if trim_loops.is_empty() {
            // 트림이 없으면 일반 버전
            uv_candidates.push(Grid2D::new(5, 5).best_initial_uv(self, &p));
        } else if let Some(uv) = Grid2D::new(5, 5).best_initial_uv_with_trim(self, &p, trim_loops) {
            uv_candidates.push(uv);
        }

        // 3) 더 fine한 grid 후보 추가 (트림 고려)
        if trim_loops.is_empty() {
            uv_candidates.push(Grid2D::new(9, 9).best_initial_uv(self, &p));
            uv_candidates.push(Grid2D::new(17, 17).best_initial_uv(self, &p));
        } else {
            if let Some(uv) = Grid2D::new(9, 9).best_initial_uv_with_trim(self, &p, trim_loops) {
                uv_candidates.push(uv);
            }
            if let Some(uv) = Grid2D::new(17, 17).best_initial_uv_with_trim(self, &p, trim_loops) {
                uv_candidates.push(uv);
            }
        }

        // 4) 후보 UV들에 대해 Newton refine 시도
        let du = self.domain_u();
        let dv = self.domain_v();
        let mut best: Option<NewtonRefineResult> = None;
        let mut best_phi = f64::INFINITY;

        for (u0, v0) in uv_candidates {
            let r = refine_point_inversion_newton(self, &p, u0, v0, opts.clone());
            // 트림이 있으면, 업데이트된 (u,v)가 트림 안에 있는지 확인
            if !trim_loops.is_empty() && !uv_in_any_trim(r.u, r.v, trim_loops) {
                // 트림 밖으로 나갔으면 스킵하거나, 경계 재투영 로직을 추가할 수 있음
                continue;
            }

            if r.final_phi < best_phi {
                best_phi = r.final_phi;
                best = Some(r.clone());
            }
            if r.converged {
                best = Some(r);
                break;
            }
        }

        // 5) 결과 결정
        if let Some(r) = best {
            let q = self.eval(r.u, r.v);
            return (r.converged, r.u, r.v, q);
        }

        // 실패 시 fallback: 트림이 있으면 첫 트림 폴리곤의 내부 가까운 점을 찾는 등
        // 별도 로직을 둘 수 있으나, 여기서는 도메인 최소로 단순 폴백
        (false, du.min, dv.min, self.eval(du.min, dv.min))
    }
}
```
```rust
/// UV가 여러 트림 폴리곤 중 하나라도 안에 있으면 true
fn uv_in_any_trim(u: f64, v: f64, polys: &[Polygon2D]) -> bool {
    use crate::math::point2d::Point2D;
    let uv = Point2D::new(u, v);
    polys.iter().any(|poly| point_in_polygon(&uv, poly))
}
```
```rust
/// 간단한 point-in-polygon (ray casting) — Grid2D와 동일 구현을 재사용
fn point_in_polygon(p: &crate::math::point2d::Point2D, poly: &Polygon2D) -> bool {
    let mut inside = false;
    let n = poly.points.len();
    if n < 3 { return false; }
    let mut j = n - 1;
    for i in 0..n {
        let pi = &poly.points[i];
        let pj = &poly.points[j];
        let intersect = ((pi.y > p.y) != (pj.y > p.y))
            && (p.x < (pj.x - pi.x) * (p.y - pi.y) / ((pj.y - pi.y) + 1e-15) + pi.x);
        if intersect { inside = !inside; }
        j = i;
    }
    inside
}
```


### Notes and recommended enhancements
- 트림 경계 로직:
- 현재는 **폴리곤 안** 만 유효로 간주합니다. 외곽/홀 구분이 필요하면 Polygon2D에 is_outer/is_hole  
    플래그를 두고 `inside = in_any_outer && !in_any_hole` 규칙을 적용.
- 뉴턴 스텝이 트림 밖으로 나갔을 때 **트림 경계에 투영** 하는 폴백을 추가하면 더 견고해집니다.  
    예: 트림 경계를 구성하는 UV 세그먼트에 최근접 투영 후 그 점에서 재시작.
- 초기 후보 다양화:
- coarse/fine grid 외에 iso-curve 기반 후보(고정 U에서 V 최소화, 고정 V에서 U 최소화)를  
    1–2개 추가하면 복잡한 트림에서도 안정적입니다.
- 성능:
- 17×17까지 평가 비용이 커질 수 있으니, adaptive refine(최적 후보 주변 박스 재샘플링)을 고려

## 🔍 테스트 아이디어
- 단순한 표면 정의
    - 예: 단위 정사각형 평면 z=0, 도메인 [0,1]×[0,1].
    - eval(u,v) = Point3D { x: u, y: v, z: 0 }.
- 테스트 점 선택
    - 예: p=(0.3,0.7,0.0).
    - 이 점은 표면 위에 있으므로 최근접 UV는 (0.3, 0.7) 근처가 나와야 함.
- Grid2D 샘플링
    - coarse grid (5×5, 9×9 등)에서 best_initial_uv 호출.
    - 반환된 (u,v)가 실제 점 근처인지 확인.
- 트림 영역 테스트
    - 예: 도메인 [0,1]×[0,1]에서 사각형 트림 (0.2,0.2)–(0.8,0.8).
    - 점 p가 트림 안에 있으면 후보 반환, 밖이면 None.

## 🧑‍💻 예제 코드 (Rust 테스트)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::interval::Interval;
    use crate::math::prelude::Point3D;

    struct TestPlane;
    impl ParamSurface for TestPlane {
        fn domain_u(&self) -> Interval { Interval::new(0.0, 1.0) }
        fn domain_v(&self) -> Interval { Interval::new(0.0, 1.0) }
        fn eval(&self, u: f64, v: f64) -> Point3D {
            Point3D { x: u, y: v, z: 0.0 }
        }
    }
```
```rust
    #[test]
    fn test_grid2d_basic() {
        let surf = TestPlane;
        let p = Point3D { x: 0.3, y: 0.7, z: 0.0 };

        let g5 = Grid2D::new(5, 5);
        let (u, v) = g5.best_initial_uv(&surf, &p);

        // 근사적으로 (0.3, 0.7)에 가까운 값이어야 함
        assert!((u - 0.3).abs() < 0.2);
        assert!((v - 0.7).abs() < 0.2);
    }
```
```rust
    #[test]
    fn test_grid2d_trim() {
        use crate::math::point2d::Point2D;
        use crate::geom::polygon2d::Polygon2D;

        let surf = TestPlane;
        let p = Point3D { x: 0.5, y: 0.5, z: 0.0 };

        // 트림 영역: 사각형 (0.2,0.2) ~ (0.8,0.8)
        let poly = Polygon2D {
            points: vec![
                Point2D::new(0.2, 0.2),
                Point2D::new(0.8, 0.2),
                Point2D::new(0.8, 0.8),
                Point2D::new(0.2, 0.8),
            ],
        };

        let g9 = Grid2D::new(9, 9);
        let uv_opt = g9.best_initial_uv_with_trim(&surf, &p, &[poly]);

        assert!(uv_opt.is_some());
        let (u, v) = uv_opt.unwrap();
        assert!((u - 0.5).abs() < 0.2);
        assert!((v - 0.5).abs() < 0.2);
    }
}
```


## ✅ 요약
- Grid2D만 테스트할 때는 단순한 평면 표면을 만들어서 best_initial_uv가 올바른 근사값을 반환하는지 확인하면 됩니다.
- 트림 영역까지 포함해 best_initial_uv_with_trim이 올바르게 동작하는지도 검증할 수 있습니다.
- 👉 이렇게 하면 Point Inversion 없이도 Grid2D의 샘플링 정확성과 트림 처리를 독립적으로 테스트할 수 있습니다.
