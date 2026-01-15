# Bezier Function Product
- on_bezier_function_product_range / on_bezier_function_product_full
- 개요
  - on_bezier_function_product_range와 on_bezier_function_product_full는
  - 두 개의 Bezier 함수 f(t),g(t) 의 곱
  ```math
  h(t)=f(t)\, g(t)
  ```
  - 을 다시 Bezier 형태로 정확하게 표현하는 함수다.
- 즉, control value 레벨에서 곱셈을 수행해, 곱 함수의 Bezier control polygon을 구하는 역할을 한다.

## 1. 수학적 정의
### 1.1 입력 Bezier 함수
- 두 Bezier 함수:
```math
f(t)=\sum _{j=0}^pf_j\, B_{j,p}(t)
```
```math
g(t)=\sum _{k=0}^qg_k\, B_{k,q}(t)
```
- 여기서
    - $B_{j,p}(t)$: 차수 p의 Bernstein basis
    - $f_j$, $g_k$: Bezier control 값
    - p,q: 각 함수의 차수

### 1.2 곱 함수의 Bezier 표현
- 우리가 만들고 싶은 것은:
```math
h(t)=f(t)\, g(t)
```
- 이를 차수 n=p+q인 Bezier로 표현하면:
```math
h(t)=\sum _{i=0}^nh_i\, B_{i,n}(t)
```
- 여기서 $h_i$ 가 우리가 구해야 할 곱의 control 값이다.

### 1.3 Bernstein 곱의 핵심 수식
- Bernstein basis끼리의 곱은 다음과 같이 정리된다:
```math
B_{j,p}(t)\, B_{k,q}(t)=\frac{{p \choose j}{q \choose k}}{{p+q \choose j+k}}\, B_{j+k,p+q}(t)
```
- 이를 이용하면:
```math
h_i=\sum _jA[i][j]\, f_j\, g_{i-j}
```
- 여기서
```math
A[i][j]=\frac{{p \choose j}{q \choose i-j}}{{p+q \choose i}}
```
- 단, i-j가 $0\leq i-j\leq q$ 범위에 있을 때만 유효.
- 이게 바로 bezier_product_matrix에서 만든 행렬을 그대로 사용하는 수식이다.

## 2. 함수별 역할
### 2.1 on_bezier_function_product_range
```rust
pub fn on_bezier_function_product_range(
    f: &[f64],
    p: usize,
    g: &[f64],
    q: usize,
    s: usize,
    e: usize,
) -> Vec<Real>
```

- 역할:
    - 차수 p Bezier 함수 f, 차수 q Bezier 함수 g에 대해  
        곱 함수 $h(t)=f(t)\, g(t)$ 의 control 값 $h_i$ 를 구한다.
- 단, 인덱스 범위 [s,e]에 대해서만 $h_i$ 를 계산하고,
- 나머지 인덱스는 0으로 둔다.
- 수식:
```math
n=p+q
```
```math
h_i=\left\{ \, \begin{array}{ll}\textstyle \displaystyle \sum _{j=\max (0,i-q)}^{\min (i,p)}\frac{{p \choose j}{q \choose i-j}}{{p+q \choose i}}\, f_j\, g_{i-j},&\textstyle s\leq i\leq e\\ \textstyle 0,&\textstyle \mathrm{otherwise}\end{array}\right.
``` 
- 코드에서:
    - on_bezier_product_matrix(p, q) → A[i][j] 생성
    - jl, jh → 유효한 j 범위 $\max (0,i-q),\min (i,p)$
    - `acc += a[i][j] * f[j] * g[i-j];` → 위 수식 그대로 구현

### 2.2 on_bezier_function_product_full
```rust
pub fn on_bezier_function_product_full(
    f: &[Real],
    g: &[Real],
) -> Vec<Real> {
    let p = f.len().checked_sub(1).expect("empty f");
    let q = g.len().checked_sub(1).expect("empty g");
    on_bezier_function_product_range(f, p, g, q, 0, p + q)
}
```

- 역할:
    - 전체 범위 [0,p+q]에 대해 곱의 control 값 $h_0,\dots ,h_{p+q}$ 를 모두 계산.
    - 즉, 완전한 곱 Bezier 함수의 control polygon을 반환.
- 수식은 위와 동일하되, 단순히 s=0,e=p+q인 경우.

### 2.3 on_eval_bezier_cfun / on_bernstein
```rust
pub fn on_eval_bezier_cfun(ctrl: &[Real], u: Real) -> Real {
    let n = ctrl.len() - 1;
    let mut s = 0.0;
    for i in 0..=n {
        s += ctrl[i] * on_bernstein(n, i, u);
    }
    s
}
```
```math
\mathrm{eval\_ bezier}(c_0,\dots ,c_n;u)=\sum _{i=0}^nc_i\, B_{i,n}(u)
```
- on_bernstein는 De Casteljau 스타일로 안정적으로 $B_{i,p}(u)$ 를 계산:
    - 초기값: tmp[p - i] = 1
    - 반복적으로
```math
\mathrm{tmp}[j]=(1-u)\, \mathrm{tmp}[j]+u\, \mathrm{tmp}[j-1]
```
- 최종적으로 tmp[p]가 $B_{i,p}(u)$ 가 됨.
- 수학적으로 올바른 Bernstein 계산 방식이다.

## 3. 용도
- 이 Bezier 곱 함수들은 다음과 같은 곳에서 핵심적으로 쓰인다:
    - B-spline 곱셈의 Bezier 단계
    - B-spline을 Bezier 세그먼트로 분해한 뒤, 각 세그먼트에서 곱을 계산할 때 사용.
    - Rational NURBS의 분자/분모 연산
    - N(u),D(u)의 곱, 제곱 등.
    - 에너지/적분 기반 알고리즘
    - $\int f(t)^2dt, \int f(t)g(t)dt$ 같은 항의 integrand 구성.
    - 곡률, 미분, 최적화
    - rational derivative에서 분자/분모에 곱이 등장할 때.

## 4. 수식 검증
- Bernstein 곱 수식
```math
B_{j,p}(t)\, B_{k,q}(t)=\frac{{p \choose j}{q \choose k}}{{p+q \choose j+k}}\, B_{j+k,p+q}(t)
```
- bezier_product_matrix에서
```math
A[i][j]=\frac{{p \choose j}{q \choose i-j}}{{p+q \choose i}}
```
- 로 정확히 구현됨.
- 곱 control 값 수식
```math
h_i=\sum _jA[i][j]f_jg_{i-j}
```
- on_bezier_function_product_range에서 그대로 구현.

### 테스트
- b_cfnprt_eval_matches_true_product에서
```math
H(t)=F(t)G(t)
```
- 를 샘플링으로 직접 검증:
```rust
let refv = on_eval_bezier_cfun(&f, u) * on_eval_bezier_cfun(&g, u);
let got  = on_eval_bezier_cfun(&fg, u);
assert!((got - refv).abs() < 5e-12);\
```
- 수식과 구현이 일치함을 강하게 보증.

## 결론
- on_bezier_function_product_range / on_bezier_function_product_full는  
    Bezier 곱셈의 정석 수식을 정확하게 구현하고 있다.
- 수식적으로도, 구현적으로도 문제되는 부분은 없다.
- 테스트가 곱 함수의 값 수준에서 검증하고 있어,
- 수학적 정당성이 충분히 확보된 상태다.

---
## 소스 코드
```rust
pub fn on_bezier_function_product_range(
    f: &[f64],
    p: usize,
    g: &[f64],
    q: usize,
    s: usize,
    e: usize,
) -> Vec<Real> {
    let n = p + q;

    assert_eq!(f.len(), p + 1, "f.len() must be p+1");
    assert_eq!(g.len(), q + 1, "g.len() must be q+1");
    assert!(
        s <= e && e <= n,
        "range [s,e] must satisfy 0 <= s <= e <= p+q"
    );

    // Use verified product matrix
    let a = on_bezier_product_matrix(p, q);

    let mut fg = vec![0.0; n + 1];
    for i in s..=e {
        let jl = if i >= q { i - q } else { 0 };
        let jh = if i <= p { i } else { p };

        let mut acc = 0.0;
        for j in jl..=jh {
            acc += a[i][j] * f[j] * g[i - j];
        }
        fg[i] = acc;
    }
    fg
}
```
```rust
/// Convenience: compute full product control polygon (s=0,e=p+q)
pub fn on_bezier_function_product_full(
    f: &[Real],
    g: &[Real],
) -> Vec<Real> {
    let p = f.len().checked_sub(1).expect("empty f");
    let q = g.len().checked_sub(1).expect("empty g");
    on_bezier_function_product_range(f, p, g, q, 0, p + q)
}
```

---
## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::basis::on_eval_bezier_cfun;
    use nurbslib::core::bezier_curve::{on_bezier_function_product_full, on_bezier_function_product_range};
    use nurbslib::core::types::Real;
```
```rust
    #[test]
    fn b_bz_func_prod_partial_range_only_writes_range() {
        // p=4, q=3 => n=7
        let f = vec![1.0, -2.0, 0.5, 3.0, -1.0];
        let g = vec![0.25, 2.0, -1.5, 0.75];

        let fg = on_bezier_function_product_range(&f, 4, &g, 3, 2, 5);
        assert_eq!(fg.len(), 8);

        // outside range should remain 0
        for i in 0..2 { assert!(fg[i].abs() < 1e-15); }
        for i in 6..8 { assert!(fg[i].abs() < 1e-15); }
    }
```
```rust
    #[test]
    fn b_bz_func_prod_partial_ranges_match_full() {
        let f = vec![ 1.2, -0.7, 2.1, 0.3, -1.5 ]; // p=4
        let g = vec![ 0.9,  1.4, -0.2, 0.6 ];      // q=3
        let full = on_bezier_function_product_full(&f, &g);

        // compute two chunks and add them
        let mut acc = vec![0.0; full.len()];
        let c1 = on_bezier_function_product_range(&f, 4, &g, 3, 0, 3);
        let c2 = on_bezier_function_product_range(&f, 4, &g, 3, 4, 7);
        for i in 0..acc.len() {
            acc[i] = c1[i] + c2[i];
        }

        for i in 0..full.len() {
            assert!((acc[i] - full[i]).abs() < 1e-14);
        }
    }
```
```rust

    #[test]
    fn b_bz_func_prod_eval_matches_true_product() {
        let f = vec![ 0.3, 1.1, -0.4, 2.0, 0.7 ]; // p=4
        let g = vec![ -1.2, 0.5, 1.8, -0.1 ];     // q=3

        let fg = on_bezier_function_product_full(&f, &g);

        for k in 0..=80 {
            let u = (k as Real) / 80.0;
            let ref_v = on_eval_bezier_cfun(&f, u) * on_eval_bezier_cfun(&g, u);
            let got = on_eval_bezier_cfun(&fg, u);
            assert!((got - ref_v).abs() < 5e-12, "u={} got={} ref={}", u, got, ref_v);
        }
    }
}
```
---
