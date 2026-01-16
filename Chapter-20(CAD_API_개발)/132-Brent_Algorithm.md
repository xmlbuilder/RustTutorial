# 🎯 Brent’s Method — 1D 최소값 찾기 알고리즘 요약
- Brent 알고리즘은 다음 두 가지를 혼합해서 사용하는 1D 최적화 알고리즘이다:
  - Golden Section Search (황금분할 탐색)
    - 항상 수렴하지만 느림
  - Parabolic Interpolation (이차 보간)
    - 빠르지만 실패할 수 있음
- Brent는 이 둘을 상황에 따라 자동으로 선택해서 빠르고 안정적으로 최소값을 찾는다.

## 📌 전제 조건 (중요)
- Brent는 반드시 다음 조건이 필요하다:
- ✔ 브래킷된 최소값
  - 세 점 x_a<x_b<x_c 에 대해:
```math
f(x_a)>f(x_b)<f(x_c)
```

- 즉, 구간 안에 최소값이 하나 존재해야 한다.
- 이걸 unimodal(단봉형) 함수라고 한다.

## 📌 Brent 알고리즘의 핵심 아이디어
- Brent는 매 반복에서 다음을 한다:

### 1) Parabolic Step (이차 보간)
- 최근 세 점 x,w,v 와 함수값 f(x),f(w),f(v) 를 이용해 이들을 지나는 이차함수(parabola) 를 만든다.
- 그 이차함수의 최소점:
```math
u=x-\frac{(x-w)^2(f(x)-f(v))-(x-v)^2(f(x)-f(w))}{2[(x-w)(f(x)-f(v))-(x-v)(f(x)-f(w))]}
```

- 이게 parabolic interpolation step이다.
- 이게 성공하면 매우 빠르게 수렴한다.

### 2) Parabolic Step 실패 조건
- 다음 중 하나라도 만족하면 parabolic step은 신뢰할 수 없다:
  - u 가 현재 구간 [a,b] 밖으로 벗어남
  - 이동량이 너무 작음
  - 분모가 너무 작아 수치적으로 불안정
  - 이전 단계보다 개선이 없음
- 이 경우 Brent는 parabolic step을 버리고 golden section step을 사용한다.

### 3) Golden Section Step
- 황금비 \phi =0.618... 를 이용해 구간을 줄인다:
```math
d=\mathrm{gold}\cdot (b-x)\quad \mathrm{또는}\quad d=\mathrm{gold}\cdot (a-x)
```
- 여기서 gold = 0.3819660 (1 - 1/φ)
- 이 스텝은 항상 안정적이다.

### 4) 새 점 u에서 f(u)를 계산하고 구간 업데이트
- f(u)<f(x) 이면
  - 최소값 후보를 u로 이동
  - 구간도 u를 기준으로 줄임
- 아니면
  - 구간만 줄임
- 그리고 보조점 w, v도 업데이트한다.

### 5) 수렴 조건
- 다음 조건 중 하나를 만족하면 종료:
- ✔ 위치 기반 수렴
```math
|x-\mathrm{midpoint}|\leq 2\cdot \mathrm{tol1}-\frac{b-a}{2}
```
- ✔ 함수값 기반 수렴
```math
f(x)\leq f_{\mathrm{tol}}
```
## 📌 Brent가 빠른 이유
- parabolic step이 성공하면 초고속 수렴
- 실패해도 golden section step이 항상 보장된 수렴
- 두 방법을 자동으로 섞어서 사용
- 그래서 Newton보다 안정적이고, golden section보다 빠르다.

## 📌 Brent가 실패하는 경우
- 다음 함수에서는 절대 안정적으로 작동하지 않는다:
  - sin(x), cos(x) 같은 periodic 함수
  - 여러 개의 극값이 있는 함수
  - concave/convex가 빠르게 바뀌는 함수
### 이유는:
- parabolic interpolation이 항상 잘못된 방향으로 튀고
- golden section step도 구간을 잘못 줄일 수 있기 때문 즉, Brent는 반드시 unimodal 함수에서만 사용해야 한다.

## 🎨 Brent Method의 시각적 흐름
- 예시 함수:
```math
f(x) = (x - 2)^2 + 1
```

- 최소값은 x = 2
- 초기 브래킷:
```math
xa < xb < xc
```
```math
f(xa) > f(xb) < f(xc)
```


### 📌 1단계 — 초기 브래킷
- x-axis
```
0    1    2    3    4
|----|----|----|----|
```
```
xa         xb         xc
*----------*----------*
 f(xa)     f(xb)      f(xc)
   \        |        /
    \       |       /
     \      |      /
      \     |     /
       \    |    /
        \   |   /
         \  |  /
          \ | /
           \|/
            V  (minimum)
```
- x = xb 가 현재 최소 후보
- w = xb, v = xb (초기에는 모두 동일)

### 📌 2단계 — Parabolic Step 시도
- Brent는 최근 3점 (x, w, v)을 이용해
- 이차곡선을 맞추고 그 최소점을 예측한다.
```
xa         xb         xc
*----------*----------*
           |\
           | \
           |  \
           |   \   ← parabola minimum predicted at u
           |    \
           |     \
           |      *
                 u (new candidate)
```

### 📌 3단계 — u에서 f(u) 계산 후 구간 업데이트
- 만약 f(u) < f(x):
```
xa         u         xb         xc
*----------*----------*----------*
```

- 그리고 점들의 역할이 이렇게 이동한다:
```
v ← w
w ← x
x ← u
```

그림으로 보면:
- old:
```
   v = w = x = xb
```
- new:
```
   v = old w
   w = old x
   x = u (new best)
```

### 📌 4단계 — Golden Section Step (parabolic 실패 시)
- 만약 parabolic step이 불안정하거나 구간 밖으로 나가면
- Brent는 안전한 황금분할 스텝을 사용한다.
```
xa         xb         xc
*----------*----------*
           |<--gold-->|
           u
```

즉, 구간을 0.618 : 0.382 비율로 줄인다.

### 📌 5단계 — 반복하면서 최소값으로 수렴
- Brent는 parabolic step과 golden section step을
- 상황에 따라 섞어서 사용한다.
- 수렴 과정은 이렇게 보인다:
- iteration 1:
```
xa ---- x -------- xc
```
- iteration 2:
```
xa ---- u -- x --- xc
```
- iteration 3:
```
xa -- u -- x ----- xc
```
- iteration 4:
```
xa - u - x ------- xc
```
- iteration 5:
```
xa - x ----------- xc
```
- iteration 6:
```
xa ~ x (converged)
```

## 🎯 최종적으로 x가 최소값에 도달
```
xa ≈ xb ≈ xc ≈ xmin
             ↓
            (minimum)
```

## 📌 Brent의 핵심을 그림으로 요약하면
- Start:     xa ---- xb ---- xc
- Parabolic: xa -- u -- xb ---- xc
- Golden:    xa -- u --------- xc
- Update:    xa -- x -- w -- v -- xc
- Repeat:    shrink until |b - a| < tol



## 🎉 초간단 요약 그림
```
xa ---- xb ---- xc
        |
        v
      (x)
```
- parabolic → u
- golden    → u

- update bracket
- repeat until convergence

---

## 샘플 코드
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
    xa: f64,
    xb: f64,
    xc: f64,
    fb: f64,
    f: F,
    x_tol: f64,
    f_tol: f64,
) -> (f64, f64)
where
    F: Fn(f64) -> f64,
{
    const GOLD: f64 = 0.3819660;
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
---
## 테스트 코드
```rust
#[test]
fn brent_quadratic_min_at_shifted_point() {
    // f(x) = (x-2.5)^2 + 3
    let f = |x: f64| {
        let t = x - 2.5;
        t * t + 3.0
    };

    // bracket around 2.5: xa=0, xb=3, xc=6
    // f(0)=9.25 > f(3)=3.25 < f(6)=15.25
    let xa = 0.0;
    let xb = 3.0;
    let xc = 6.0;
    let fb = f(xb);

    let (xmin, fmin) = on_min_fun_brent(xa, xb, xc, fb, f, 1e-12, 1e-15);

    assert!(approx(xmin, 2.5, 1e-8), "xmin={}", xmin);
    assert!(approx(fmin, 3.0, 1e-10), "fmin={}", fmin);
}
```
---



