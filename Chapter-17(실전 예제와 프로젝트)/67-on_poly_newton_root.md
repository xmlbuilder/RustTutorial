# on_poly_newton_root
## 1. 함수 역할
- 이 함수의 역할은:
  - 다항식
  - 구간 [ul, ur]
  - 초기 추정값 u0
  - Newton 방법으로 근 찾기
## 2. 수식 설명 (Newton 방법 + 다항식)
- 다항식:
```math
p(u)=a_nu^n+a_{n-1}u^{n-1}+\dots +a_1u+a_0
```
- 우리가 찾고 싶은 건:
```math
p(u)=0
```
- Newton 방법은 반복식:
```math
u_{k+1}=u_k-\frac{p(u_k)}{p'(u_k)}
```
- 여기서:
  - $p(u_k)$ = 현재 값
  - $p'(u_k)$ = 도함수 값
  - $\frac{p(u_k)}{p'(u_k)}$ = **기울기 기반 보정량**
- 이걸 반복하면서:
  - $|p(u_k)|<\varepsilon$  이면 “근에 충분히 가까움”
  - 또는 $|(u_{k+1}-u_k)\cdot p'(u_k)|<\varepsilon$  이면 **변화량이 충분히 작음**
- 추가로:
  - u가 [ul, ur] 밖으로 나가면 clamp해서 구간 안으로 되돌림

## 3. 단계별 동작 문서화

- 다항식의 근을 Newton 방법으로 찾는다.

- 다항식:
```math
    p(u) = a[n]*u^n + ... + a[1]*u + a[0]
```
- 입력:
    - `a`    : 계수 배열 (길이 >= n+1)
    - `n`    : 최고 차수 인덱스 (degree)
    - `u0`   : 초기 추정값
    - `ul`   : 근이 존재해야 하는 구간의 하한
    - `ur`   : 근이 존재해야 하는 구간의 상한
    - `eps`  : 허용 오차 (|p(u)|, 또는 Newton step 기반)
    - `itlim`: 최대 반복 횟수

- 출력:
    - `Ok(u)`  : 수렴한 근
    - `Err(..)`: 수렴 실패 또는 수치적 문제

- 알고리즘 개요:
- 1. u_new ← u0 로 시작
- 2. 반복:
    - a) p(u_new), p'(u_new) 계산 (on_poly_ders 사용)
    - b) |p(u_new)| < eps 이면 수렴 → u_new 반환
    - c) p'(u_new) ≈ 0 이면 Newton 불가능 → 에러
    - d) Newton step: u_old = u_new, u_new = u_old - p/ p'
    - e) u_new 를 [ul, ur] 구간으로 clamp
    - f) |(u_new - u_old) * p'(u_new)| < eps 이면 수렴 → u_new 반환
    - g) itlim 만큼 반복 후에도 수렴 못하면 에러

## 4. 최종 Rust 코드
```rust
pub fn poly_newton_root(
    a: &[f64],
    n: usize,
    u0: f64,
    ul: f64,
    ur: f64,
    eps: f64,
    itlim: usize,
) -> Result<f64, &'static str> {
    assert!(a.len() >= n + 1);

    let mut u_new = u0;
    let mut u_old;

    let mut ad = [0.0_f64; 2]; // ad[0] = p(u), ad[1] = p'(u)

    for _k in 0..itlim {
        // p(u), p'(u) 계산
        on_poly_ders(a, n, u_new, 1, &mut ad);
        let f = ad[0];
        let df = ad[1];

        // 함수값이 충분히 작으면 수렴
        if f.abs() < eps {
            return Ok(u_new);
        }

        // 도함수가 너무 작으면 Newton 불가능
        if df.abs() < 1e-14 {
            return Err("Derivative too small");
        }

        // Newton step
        u_old = u_new;
        u_new = u_old - (f / df);

        // 구간 [ul, ur]로 clamp
        if u_new < ul {
            u_new = ul;
        } else if u_new > ur {
            u_new = ur;
        }

        // step 기반 수렴 판정
        if ((u_new - u_old) * df).abs() < eps {
            return Ok(u_new);
        }
    }

    Err("No convergence within iteration limit")
}
```

## 5. on_poly_ders 그대로 재사용
```rust
pub fn on_poly_ders(a: &[f64], n: usize, u: f64, der: usize, ad: &mut [f64]) {
    assert!(a.len() >= n + 1);
    assert!(ad.len() >= der + 1);

    ad[0] = a[n];
    for i in 1..=der {
        ad[i] = 0.0;
    }

    for i in (0..n).rev() {
        let k = der.min(n - i);
        for j in (1..=k).rev() {
            ad[j] = ad[j] * u + ad[j - 1];
        }
        ad[0] = ad[0] * u + a[i];
    }

    let mut fact = 1.0;
    for i in 2..=der {
        fact *= i as f64;
        ad[i] *= fact;
    }
}
```

## 6. 테스트 코드 예시
- 간단한 다항식으로 테스트해보자.
  - p(u)=u^2-4 → 근: u = 2, -2
  - p(u)=u^3-1 → 근: u = 1
```rust
#[cfg(test)]
mod tests {
    use super::{poly_newton_root, on_poly_ders};

    #[test]
    fn test_poly_root_quadratic() {
        // p(u) = u^2 - 4 = 0 → u = 2 or -2
        // 계수: a[0] = -4, a[1] = 0, a[2] = 1
        let a = [-4.0_f64, 0.0, 1.0];
        let n = 2;

        // 근 2 근처에서 시작
        let u0 = 3.0;
        let ul = 0.0;
        let ur = 10.0;
        let eps = 1e-10;
        let itlim = 50;

        let root = poly_newton_root(&a, n, u0, ul, ur, eps, itlim).unwrap();
        assert!((root - 2.0).abs() < 1e-8);
    }
```
```rust
    #[test]
    fn test_poly_root_cubic() {
        // p(u) = u^3 - 1 = 0 → u = 1
        // 계수: a[0] = -1, a[1] = 0, a[2] = 0, a[3] = 1
        let a = [-1.0_f64, 0.0, 0.0, 1.0];
        let n = 3;

        let u0 = 0.5;
        let ul = -10.0;
        let ur = 10.0;
        let eps = 1e-10;
        let itlim = 50;

        let root = poly_newton_root(&a, n, u0, ul, ur, eps, itlim).unwrap();
        assert!((root - 1.0).abs() < 1e-8);
    }
}
```
---
