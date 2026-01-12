# 근 찾기 (Newton-Raphson)
- 아래는 제공해주신 다항식 근 찾기 함수들에 대한 수학적 원리, 단계별 설명, 그리고 기능 요약 표입니다.  
- 이 코드는 수치해석에서 매우 중요한 근 찾기 알고리즘을 구현하고 있으며, Newton-Raphson, 이분법, 자동 브래킷팅을 포함합니다.

## 📐 핵심 수식 정리
### 1. Newton-Raphson 방법

- 반복식:

```math
u_{k+1}=u_k-\frac{f(u_k)}{f'(u_k)}
```

- 수렴 조건:

```math
|f(u_k)|<\mathrm{tol}\quad \mathrm{또는} \quad |(u_{k+1}-u_k)\cdot f'(u_k)|<\mathrm{tol}
```


### 2. 이분법 (Bisection)
- 조건:

```math
f(u_l)\cdot f(u_r)<0\quad \mathrm{(부호\  변화)}
```

- 반복식:

```math
u_c=\frac{u_l+u_r}{2}
```

- 수렴 조건:

```math
|f(u_c)|<\mathrm{tol}\quad \mathrm{또는}\quad \frac{u_r-u_l}{2}<\mathrm{tol}
```

### 3. 자동 브래킷팅 + Newton + 이분법
- 1차 스캔으로 부호 변화 구간 탐색
- 초기 추정값: 선형 보간 또는 중간값
- Newton-Raphson 시도
- 실패 시 이분법으로 대체


## 🧭 단계별 설명
- 🔹 on_polynomial_newton_root
    - 초기값 $u0$ 을 구간 $[ul, ur]$ 에 클램핑
    - 반복적으로 Newton-Raphson 적용
    - 도함수가 너무 작으면 실패
    - 수렴 조건 만족 시 반환

- 🔹 on_bisection_root
    - 구간 $[ul, ur]$ 에서 부호 변화 확인
    - 중간값 $c$ 계산
    - $f(c)$ 의 부호에 따라 구간 절반으로 축소
    - 수렴 시 근 반환

- 🔹 on_polynomial_auto_bracket_and_newton
    - samples 개수로 구간 $[umin, umax]$ 을 스캔
    - 부호 변화 구간들(Bracket) 수집
    - 가장 좋은 구간 선택 (중간값의 f 최소)
    - 초기 추정값 계산
    - Newton-Raphson 시도
- 실패 시 이분법으로 대체

## 📊 기능별 정리 표

| 함수 이름                          | 기능 요약                                 | 수학적 핵심 또는 수식 요약                                   |
|-----------------------------------|------------------------------------------|--------------------------------------------------------------|
| `on_polynomial_newton_root`         | Newton-Raphson 근 찾기                   |$u_{k+1} = u_k - \frac{f(u_k)}{f'(u_k)}$                 |
| `on_bisection_root`                 | 이분법 근 찾기                           |$u_c = \frac{u_l + u_r}{2}$                              |
| `on_find_sign_change_brackets`      | 부호 변화 구간 탐색                      |$f(u_k) \cdot f(u_{k+1}) < 0$                            |
| `initial_guess_from_bracket`     | 브래킷 기반 초기 추정값 계산             | 선형 보간 또는 $u_0 = \frac{u_l + u_r}{2}$               |
| `on_polynomial_auto_bracket_and_newton` | 자동 브래킷팅 + Newton + 이분법 통합 | 부호 변화 탐색 → 초기 추정 → Newton → 실패 시 이분법       |

---

```rust
// ------------------------------------------------------------
// Newton options
// ------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct PolynomialNewtonOptions {
    pub max_iter: i32,
    pub deriv_eps: f64, // If set to 0, auto value is used: 1e-30 × (1 + |f'|)
    pub ascending: bool,
}
```
```rust
impl Default for PolynomialNewtonOptions {
    fn default() -> Self {
        Self {
            max_iter: 50,
            deriv_eps: 0.0,
            ascending: false,
        }
    }
}
```
```rust
// ------------------------------------------------------------
// polynomial_newton_root
// ------------------------------------------------------------
pub fn on_polynomial_newton_root(
    a: &[f64],
    u0: f64,
    ul: f64,
    ur: f64,
    tol: f64,
    opt: PolynomialNewtonOptions,
) -> Option<f64> {
    if a.is_empty() {
        return None;
    }
    let mut unew = u0.clamp(ul.min(ur), ul.max(ur));

    let maxit = opt.max_iter.max(1);
    for _ in 0..maxit {
        let (f, df) = on_polynomial_f_df(a, unew, opt.ascending);
        if f.abs() < tol {
            return Some(unew);
        }

        let mut deps = opt.deriv_eps;
        if deps <= 0.0 {
            deps = 1e-30 * (1.0 + df.abs());
        }
        if df.abs() <= deps {
            return None;
        } // 나눗셈 불가

        let uold = unew;
        unew = uold - f / df;
        // 구간 보정
        unew = unew.clamp(ul.min(ur), ul.max(ur));

        if ((unew - uold) * df).abs() < tol {
            return Some(unew);
        }
    }
    None
}
```
```rust
// ------------------------------------------------------------
// Bisection bracket root
// ------------------------------------------------------------
pub fn on_bisection_root(
    a: &[f64],
    ul: f64,
    ur: f64,
    tol: f64,
    ascending: bool,
    max_iter: i32,
) -> Option<f64> {
    let (mut f_l, _) = on_polynomial_f_df(a, ul, ascending);
    let (mut f_r, _) = on_polynomial_f_df(a, ur, ascending);

    if f_l == 0.0 {
        return Some(ul);
    }
    if f_r == 0.0 {
        return Some(ur);
    }
    if f_l * f_r > 0.0 {
        return None;
    }

    let (mut a_l, mut a_r) = (ul, ur);
    for _ in 0..max_iter.max(1) {
        let c = 0.5 * (a_l + a_r);
        let (f_c, _) = on_polynomial_f_df(a, c, ascending);
        if f_c.abs() < tol || 0.5 * (a_r - a_l).abs() < tol {
            return Some(c);
        }
        if f_l * f_c <= 0.0 {
            a_r = c;
            f_r = f_c;
        } else {
            a_l = c;
            f_l = f_c;
        }
    }
    Some(0.5 * (a_l + a_r))
}
```
```rust
// ------------------------------------------------------------
// Automatic bracketing + Newton method + fallback to bisection
// ------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct AutoPolynomialNewtonOptions {
    pub samples: i32, // 그리드 샘플 수
    pub ascending: bool,
    pub newton_max_iter: i32,
    pub newton_deriv_eps: f64,
}
```
```rust
impl Default for AutoPolynomialNewtonOptions {
    fn default() -> Self {
        Self {
            samples: 32,
            ascending: false,
            newton_max_iter: 50,
            newton_deriv_eps: 0.0,
        }
    }
}
```
```rust
#[derive(Clone, Copy)]
struct Bracket {
    ul: f64,
    ur: f64,
    fl: f64,
    fr: f64,
}
```
```rust
fn on_find_sign_change_brackets(
    a: &[f64],
    umin: f64,
    umax: f64,
    samples: i32,
    ascending: bool,
) -> Vec<Bracket> {
    let mut out = Vec::new();
    let s = samples.max(2) as usize;
    let du = (umax - umin) / (s as f64);

    let mut u_prev = umin;
    let (mut f_prev, _) = on_polynomial_f_df(a, u_prev, ascending);

    for k in 1..=s {
        let u_cur = if k == s { umax } else { umin + (k as f64) * du };
        let (f_cur, _) = on_polynomial_f_df(a, u_cur, ascending);

        if f_prev == 0.0 || f_cur == 0.0 || f_prev * f_cur < 0.0 {
            let (mut ul, mut ur, mut fl, mut fr) = (u_prev, u_cur, f_prev, f_cur);
            if ul > ur {
                std::mem::swap(&mut ul, &mut ur);
                std::mem::swap(&mut fl, &mut fr);
            }
            out.push(Bracket { ul, ur, fl, fr });
        }
        u_prev = u_cur;
        f_prev = f_cur;
    }
    out
}
```
```rust
fn initial_guess_from_bracket(b: Bracket) -> f64 {
    if b.fl == 0.0 {
        return b.ul;
    }
    if b.fr == 0.0 {
        return b.ur;
    }
    let denom = b.fr - b.fl;
    if denom.abs() > 0.0 {
        let u_sec = b.ul - b.fl * (b.ur - b.ul) / denom;
        if u_sec >= b.ul.min(b.ur) && u_sec <= b.ul.max(b.ur) {
            return u_sec;
        }
    }
    0.5 * (b.ul + b.ur)
}
```
```rust
pub fn on_polynomial_auto_bracket_and_newton(
    a: &[f64],
    umin: f64,
    umax: f64,
    tol: f64,
    opt: AutoPolynomialNewtonOptions,
) -> Option<f64> {
    // 1) Sign change bracketing
    let br = on_find_sign_change_brackets(a, umin, umax, opt.samples, opt.ascending);
    if br.is_empty() {
        // Attempt Newton method within a narrow bracket around the grid minimum of |f|
        let samples = opt.samples.max(16);
        let mut best_u = umin;
        let mut best_f = f64::INFINITY;
        for k in 0..=samples {
            let u = umin + (umax - umin) * (k as f64) / (samples as f64);
            let (f, _) = on_polynomial_f_df(a, u, opt.ascending);
            let af = f.abs();
            if af < best_f {
                best_f = af;
                best_u = u;
            }
        }
        let eps = 1e-6_f64.max(tol * 10.0);
        let ul = umin.max(best_u - eps);
        let ur = umax.min(best_u + eps);

        let nopt = PolynomialNewtonOptions {
            max_iter: opt.newton_max_iter,
            deriv_eps: opt.newton_deriv_eps,
            ascending: opt.ascending,
        };
        if let Some(u) = on_polynomial_newton_root(a, best_u, ul, ur, tol, nopt) {
            return Some(u);
        }
        // If a valid bracket is formed, apply bisection
        let (fl, _) = on_polynomial_f_df(a, ul, opt.ascending);
        let (fr, _) = on_polynomial_f_df(a, ur, opt.ascending);
        if fl * fr <= 0.0 {
            return on_bisection_root(a, ul, ur, tol, opt.ascending, 64);
        }
        return None;
    }

    // 2) Select bracket with smallest |f| at midpoint
    let mut best_i = 0usize;
    let mut best_score = f64::INFINITY;
    for (i, b) in br.iter().enumerate() {
        let mid = 0.5 * (b.ul + b.ur);
        let (fm, _) = on_polynomial_f_df(a, mid, opt.ascending);
        let score = fm.abs();
        if score < best_score {
            best_score = score;
            best_i = i;
        }
    }
    let b = br[best_i];

    // 3) Initial guess
    let u0 = initial_guess_from_bracket(b);
    let (ul, ur) = (b.ul, b.ur);

    // 4) Newton iteration
    let nopt = PolynomialNewtonOptions {
        max_iter: opt.newton_max_iter,
        deriv_eps: opt.newton_deriv_eps,
        ascending: opt.ascending,
    };
    if let Some(u) = on_polynomial_newton_root(a, u0, ul, ur, tol, nopt) {
        return Some(u);
    }
    // 5) Fallback to bisection
    on_bisection_root(a, ul, ur, tol, opt.ascending, 64)
}
```
---

## 📊 테스트 요약 표

| 테스트 함수 이름                          | 테스트 목적                         | 기대 결과 또는 수식 요약                         |
|-------------------------------------------|--------------------------------------|--------------------------------------------------|
| `test_polynomial_newton_root_basic`       | Newton-Raphson으로 다항식 근 찾기    |$\sqrt{2} \approx 1.4142$                    |
| `test_bisection_root_basic`               | 이분법으로 다항식 근 찾기            |$u = 1.0$                                    |
| `test_polynomial_auto_bracket_and_newton` | 자동 브래킷팅 + Newton + 이분법 통합 |$u = 1.0$ 또는$u = -2.0$                |

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::maths::{on_bisection_root, on_polynomial_auto_bracket_and_newton, 
    on_on_polynomial_f_df, on_polynomial_newton_root, 
    AutoPolynomialNewtonOptions, PolynomialNewtonOptions};

    #[test]
    fn test_polynomial_newton_root_basic() {
        // f(u) = u^2 - 2 → 근: ±√2 ≈ 1.4142
        let coeffs = [-2.0, 0.0, 1.0]; // ascending: true → f(u) = -2 + 0*u + 1*u^2
        let opt = PolynomialNewtonOptions {
            max_iter: 50,
            deriv_eps: 0.0,
            ascending: true,
        };
        let root = on_polynomial_newton_root(&coeffs, 1.0, 0.0, 2.0, 1e-10, opt).unwrap();
        assert!((root - 2.0_f64.sqrt()).abs() < 1e-8);
    }
```
```rust
    #[test]
    fn test_bisection_root_basic() {
        // f(u) = u^3 - u → 근: u = 0, ±1
        let coeffs = [0.0, -1.0, 0.0, 1.0]; // ascending
        let root = on_bisection_root(&coeffs, 0.5, 1.5, 1e-10, true, 100).unwrap();
        assert!((root - 1.0).abs() < 1e-8);
    }
```
```rust
    #[test]
    fn test_polynomial_auto_bracket_and_newton() {
        // f(u) = u^3 - 3u + 2 → 근: u = 1, u = -2
        let coeffs = [2.0, -3.0, 0.0, 1.0]; // ascending
        let opt = AutoPolynomialNewtonOptions {
            samples: 32,
            ascending: true,
            newton_max_iter: 50,
            newton_deriv_eps: 0.0,
        };
        let root = on_polynomial_auto_bracket_and_newton(&coeffs, -3.0, 3.0, 1e-10, opt).unwrap();
        let expected = vec![1.0, -2.0];
        assert!(expected.iter().any(|&r| (r - root).abs() < 1e-8));
    }
```
```rust    
    #[test]
    fn test_on_polynomial_f_df_basic() {
        // f(u) = -6 + 1*u + 1*u^2 = u^2 + u - 6
        let a = [-6.0, 1.0, 1.0]; // ascending 저장(상수, 1차, 2차)
        let (f2, df2) = on_on_polynomial_f_df(&a, 2.0, true);
        // f(2)=0, f'(u)=2u+1 => f'(2)=5
        assert!(f2.abs() < 1e-12);
        assert!((df2 - 5.0).abs() < 1e-12);
    }
```
```rust
    // ---- Newton / Bisection / Auto root -------------------------------------
    #[test]
    fn test_polynomial_newton_root() {
        // (u-1)(u-2)(u-3) = -6 + 11u - 6u^2 + 1u^3
        let a = [-6.0, 11.0, -6.0, 1.0]; // ascending
        let opt = PolynomialNewtonOptions {
            max_iter: 50,
            deriv_eps: 0.0,
            ascending: false,
        };
        // 1~2 사이 루트를 찾아보자 (u≈1)
        let root = on_polynomial_newton_root(&a, 1.1, 1.0, 2.0, 1e-12, opt).unwrap();
        assert!((root - 1.0).abs() < 1e-10);
    }
```
```rust
    #[test]
    fn test_bisection_root() {
        // u^2 + u - 6 = 0 → u ∈ {-3, 2}
        let a = [-6.0, 1.0, 1.0];
        // [-4, -2]에 -3 브래킷
        let r1 = on_bisection_root(&a, -4.0, -2.0, 1e-12, true, 64).unwrap();
        assert!((r1 + 3.0).abs() < 1e-9);
        // [1, 3]에 2 브래킷
        let r2 = on_bisection_root(&a, 1.0, 3.0, 1e-12, true, 64).unwrap();
        assert!((r2 - 2.0).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_polynomial_auto_bracket_and_newton_case2() {
        // (u-2)(u+3) = u^2 + u - 6
        let a = [-6.0, 1.0, 1.0];
        let mut opt = AutoPolynomialNewtonOptions::default();
        opt.ascending = true;
        let r = on_polynomial_auto_bracket_and_newton(&a, -10.0, 10.0, 1e-12, opt).unwrap();
        // 자동으로 어느 루트든 하나는 잡아야 함
        let near_root = (r - 2.0).abs() < 1e-8 || (r + 3.0).abs() < 1e-8;
        assert!(near_root, "found {} which is not near ±roots", r);
    }
}
```
---

