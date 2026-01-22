# Scalar Brent Func  
- 이 세 함수는 1차원 스칼라 함수의 최소값을 찾기 위한 Brent 알고리즘 패밀리.


## 🌕 1) on_min_fun_brent — Brent 최소화 알고리즘 (핵심)
- 이 함수는 브래킷이 이미 주어진 상태에서  
    1D 함수 f(x)의 최소값을 찾는다.
- ✔ 입력 조건 (Bracketing triple)
- 세 점이 필요하다:
```math
x_a<x_b<x_c
```
- 그리고
```math
f(x_a)>f(x_b)<f(x_c)
```
- 즉, x_b가 지역 최소 근처에 있는 “골짜기” 형태여야 한다.
- 이걸 “브래킷(bracket)”이라고 부른다.

## 🔍 Brent 알고리즘이 뭔가?
- Brent method는 다음 두 가지를 자동으로 섞어서 사용하는 알고리즘이다:
### 1) 황금분할 탐색 (Golden Section Search)
- 안정적
- 수렴은 느림
- 파라볼라가 잘 안 맞을 때 사용
### 2) 포물선 보간 (Parabolic Interpolation)
- 빠름
- 하지만 잘못된 방향으로 튈 수 있음
- 조건이 맞을 때만 사용
- Brent는 이 둘을 조건부로 섞어서 안정성과 속도를 동시에 확보한다.

## 🧠 알고리즘 흐름 (핵심 요약)
- 변수 의미
    - x : 현재 최소 후보
    - w : 이전 최소 후보
    - v : 그 이전 후보
    - fx, fw, fv : 각각의 함수값
    - a, b : 현재 탐색 구간
    - d, e : 이동 거리 (parabolic or golden)

- ✔ 1) 수렴 검사
```rust
if (x - xm).abs() <= tol2 - 0.5 * (b - a) {
    return (x, fx);
}
```

- 현재 x가 구간 중앙 xm 근처에 충분히 가까우면 종료.

- ✔ 2) 포물선 보간 시도
```rust
r = (x - w) * (fx - fv);
q = (x - v) * (fx - fw);
p = (x - v) * q - (x - w) * r;
q = 2.0 * (q - r);
```

- 이 수식은 세 점 (x, w, v) 를 지나는 포물선을 만들고  
    그 포물선의 최소점을 찾는 과정이다.
- 조건이 맞으면:
```rust
d = p / q;
u = x + d;
```

- 즉, 포물선의 최소점으로 이동.

- ✔ 3) 포물선이 위험하면 황금분할로 fallback
- 포물선 보간이 다음 조건 중 하나라도 만족하면 실패로 간주:
    - 구간 밖으로 나감
    - 이동량이 너무 큼
    - q가 너무 작음
    - 포물선이 뒤집힘
- 그 경우:
```rust
d = GOLD * (a - x)  또는  GOLD * (b - x)
```

- 즉, 황금분할로 안전하게 이동.

- ✔ 4) 새로운 점 u 평가 후 구간 업데이트
```rust
if fu <= fx {
    // u가 더 좋으면 x를 u로 교체
} else {
    // 아니면 구간만 줄임
}
```

- 이 과정이 반복되며 최소값에 수렴한다.

## 🎯 결과
```rust
(xmin, fmin)
```


## 🌕 2) on_find_bracket_by_sampling — 브래킷 자동 찾기
- Brent는 브래킷이 있어야만 작동한다.
- 그래서 이 함수는:
    - ✔ [lo, hi] 구간을 일정 간격으로 샘플링
    - ✔ 모든 인접한 3점 (xa, xb, xc)에 대해
```math
f(x_a)>f(x_b)<f(x_c)
```
- 을 만족하는 트리플을 찾는다.

- ✔ 알고리즘 흐름
    - samples 개수만큼 균일 샘플링
- 모든 i에 대해
    - (i-1, i, i+1) 트리플 검사
- 조건을 만족하는 트리플 중
    - f(xb)가 가장 작은 것을 선택
- 없으면 “약한 조건”
    - >= 비교로 완화
    - 그래도 없으면 None

## 🌕 3) on_min_fun_brent_auto_bracket — 자동 브래킷 + Brent
- 이 함수는:
    - 1) 브래킷을 자동으로 찾고
    - 2) 찾으면 Brent로 최소화하고
    - 3) 결과를 반환한다
- 즉, 완전 자동 1D 최소화 함수다.

- ✔ 알고리즘 흐름
- 샘플 수 = samples
- 최대 5번 반복
    - 샘플링 → 브래킷 찾기
    - 찾으면 Brent 실행
    - 없으면 샘플 수 2배 증가
    - 그래도 못 찾으면 None

### 🧩 세 함수의 관계
```rust
on_min_fun_brent_auto_bracket
    └── on_find_bracket_by_sampling
            └── (xa, xb, xc) 찾기
    └── on_min_fun_brent
            └── 최소값 찾기

```
- 즉:
    - auto_bracket → 자동 브래킷 + 최소화
    - find_bracket → 브래킷만 찾기
    - brent → 브래킷이 주어졌을 때 최소화

### 🏁 어디에 쓰는가? (NURBS / CAD / CAE)
- 이 함수들은 다음 작업에서 매우 자주 쓰인다:
    - 곡선/곡면에서 최근접점 찾기
    - 거리 최소화
    - 곡선-곡선 최소 거리
    - 곡면-곡선 최소 거리
    - 곡면-곡면 최소 거리
    - trimming curve parameter optimization
    - Newton 실패 시 fallback
    - offset curve self-intersection 탐지
    - curvature 최소/최대 찾기
- 즉, 기하 엔진의 최저 레벨 최적화 루틴이다.

---

## 소스 코드
```rust

/// Brent's method for 1D function minimization (Numerical Recipes style).
/// Requires a bracketing triple (xa < xb < xc) such that:
///     f(xa) > f(xb) < f(xc)
///
/// - f: scalar function f(x)
/// - xa, xb, xc: bracketing points
/// - fb: f(xb)
/// - x_tol: relative tolerance for x (fractional accuracy)
/// - f_tol: absolute tolerance for f(x)
///
/// Returns (xmin, fmin)
pub fn on_min_fun_brent<F>(
    xa: Real,
    xb: Real,
    xc: Real,
    fb: Real,
    f: F,
    x_tol: Real,
    f_tol: Real,
) -> (f64, f64)
where
    F: Fn(f64) -> Real,
{
    const GOLD: Real = 0.3819660;
    const MAX_IT: usize = 100;

    let (mut a, mut b) = if xa < xc { (xa, xc) } else { (xc, xa) };

    let mut x = xb;
    let mut w = xb;
    let mut v = xb;

    let mut fx = fb;
    let mut fw = fb;
    let mut fv = fb;

    let mut e = 0.0;
    let mut d = 0.0;

    for _ in 0..MAX_IT {
        let xm = 0.5 * (a + b);
        let tol1 = x_tol * x.abs() + ON_ZERO_TOL;
        let tol2 = 2.0 * tol1;

        // Convergence check
        if (x - xm).abs() <= tol2 - 0.5 * (b - a) {
            return (x, fx);
        }

        let mut p = 0.0;
        let mut q = 0.0;
        let mut r = 0.0;

        let mut u;

        if e.abs() > tol1 {
            // Parabolic fit
            r = (x - w) * (fx - fv);
            q = (x - v) * (fx - fw);
            p = (x - v) * q - (x - w) * r;
            q = 2.0 * (q - r);

            if q > 0.0 {
                p = -p;
            }
            q = q.abs();

            let e_temp = e;
            e = d;

            if p.abs() >= 0.5 * q * e_temp || p <= q * (a - x) || p >= q * (b - x) {
                // Golden section step
                e = if x >= xm { a - x } else { b - x };
                d = GOLD * e;
            } else {
                // Parabolic step
                d = p / q;
                u = x + d;

                if (u - a).abs() < tol2 || (b - u).abs() < tol2 {
                    d = if d >= 0.0 { tol1 } else { -tol1 };
                }
            }
        } else {
            // Golden section step
            e = if x >= xm { a - x } else { b - x };
            d = GOLD * e;
        }

        u = if d.abs() >= tol1 {
            x + d
        } else {
            x + d.signum() * tol1
        };
        let fu = f(u);

        if fu <= f_tol {
            return (u, fu);
        }

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

            if fu <= fw || w == x {
                v = w;
                fv = fw;
                w = u;
                fw = fu;
            } else if fu <= fv || v == x || v == w {
                v = u;
                fv = fu;
            }
        }
    }

    (x, fx)
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct BrentBracket {
    pub xa: Real,
    pub xb: Real,
    pub xc: Real,
    pub fa: Real,
    pub fb: Real,
    pub fc: Real,
}
```
```rust
/// Scan adjacent triples in [lo,hi] to find bracketing triple:
/// xa < xb < xc and f(xa) > f(xb) < f(xc)
///
/// This version cannot infinite-loop.
pub fn on_find_bracket_by_sampling<F>(
    lo: Real,
    hi: Real,
    samples: usize,
    f: &F,
) -> Option<BrentBracket>
where
    F: Fn(f64) -> Real,
{
    if !(lo < hi) || samples < 3 {
        return None;
    }

    let n = samples;
    let step = (hi - lo) / (n as f64 - 1.0);

    let mut xs = Vec::with_capacity(n);
    let mut fs = Vec::with_capacity(n);

    for i in 0..n {
        let x = lo + step * (i as f64);
        xs.push(x);
        fs.push(f(x));
    }

    // Find the best (lowest fb) among all adjacent triples that satisfy the bracket condition.
    let mut best: Option<BrentBracket> = None;

    for i in 1..(n - 1) {
        let xa = xs[i - 1];
        let xb = xs[i];
        let xc = xs[i + 1];
        let fa = fs[i - 1];
        let fb = fs[i];
        let fc = fs[i + 1];

        if fa > fb && fc > fb {
            let cand = BrentBracket {
                xa,
                xb,
                xc,
                fa,
                fb,
                fc,
            };
            best = match best {
                None => Some(cand),
                Some(cur) => {
                    if cand.fb < cur.fb {
                        Some(cand)
                    } else {
                        Some(cur)
                    }
                }
            };
        }
    }

    // Fallback: if no strict bracket found, try "weak" bracket (>=) for flat sampling / noise.
    if best.is_none() {
        for i in 1..(n - 1) {
            let xa = xs[i - 1];
            let xb = xs[i];
            let xc = xs[i + 1];
            let fa = fs[i - 1];
            let fb = fs[i];
            let fc = fs[i + 1];

            if fa >= fb && fc >= fb {
                let cand = BrentBracket {
                    xa,
                    xb,
                    xc,
                    fa,
                    fb,
                    fc,
                };
                best = match best {
                    None => Some(cand),
                    Some(cur) => {
                        if cand.fb < cur.fb {
                            Some(cand)
                        } else {
                            Some(cur)
                        }
                    }
                };
            }
        }
    }

    best
}
```
```rust
pub fn on_min_fun_brent_auto_bracket<F>(
    lo: Real,
    hi: Real,
    mut samples: usize,
    f: F,
    x_tol: Real,
    f_tol: Real,
) -> Option<(f64, f64)>
where
    F: Fn(f64) -> Real,
{
    // Try a few times increasing sampling density.
    // (No infinite loop: bounded attempts)
    for _ in 0..5 {
        if let Some(br) = on_find_bracket_by_sampling(lo, hi, samples, &f) {
            // IMPORTANT: if fb isn't exactly f(xb) (it is), pass it directly
            let (xmin, fmin) = on_min_fun_brent(br.xa, br.xb, br.xc, br.fb, &f, x_tol, f_tol);
            return Some((xmin, fmin));
        }
        samples = samples.saturating_mul(2);
        if samples < 16 {
            samples = 16;
        }
    }
    None
}
```
---
