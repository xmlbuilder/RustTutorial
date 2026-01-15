# Bezier Product Matrix
- on_bezier_product_matrix와 on_product_ctrl_via_matrix는 Bezier 곱셈의 수학적 핵심을 정확하게 구현.


## 🎯 1. Bezier 곱셈의 수학적 정의
- 두 Bezier 함수가 있다고 하자:
```math
F(u)=\sum _{j=0}^pf_j\, B_{j,p}(u)
```
```math
G(u)=\sum _{k=0}^qg_k\, B_{k,q}(u)
```
- 여기서
  - $B_{j,p}(u)$ = Bernstein basis of degree p
  - $f_j$, $g_k$ = control values

- ✔ 우리가 만들고 싶은 것
```math
H(u)=F(u)\, G(u)
```
- 이걸 Bezier basis of degree p+q 로 표현하고 싶다:
```math
H(u)=\sum _{i=0}^{p+q}h_i\, B_{i,p+q}(u)
```
- 즉, 곱의 control value $h_i$ 를 구하는 것이 목표.

## 🎯 2. 핵심 수식: Bernstein 곱의 분해
- Bernstein basis끼리의 곱은 다음과 같은 놀라운 성질을 가진다:
```math
B_{j,p}(u)\, B_{k,q}(u)=\frac{{p \choose j}{q \choose k}}{{p+q \choose j+k}}\, B_{j+k,p+q}(u)
```
- 이게 모든 걸 해결한다.

## 🎯 3. Product matrix A의 정의
- 위 식을 이용하면:
```math
h_i=\sum _jA[i][j]\, f_j\, g_{i-j}
```
- 여기서
```math
A[i][j]=\frac{{p \choose j}{q \choose i-j}}{{p+q \choose i}}
```
- 단, i-j가 0..q 범위에 있어야 함.

## 🎯 4. 코드가 구현한 수식
- 코드:
```math
a[i][j] = (1 / C(n,i)) * C(p,j) * C(q, i-j)
```

- 수식으로 쓰면:
```math
A[i][j]=\frac{{p \choose j}{q \choose i-j}}{{p+q \choose i}}
```


## 🎯 5. product_ctrl_via_matrix의 수식
```math
h[i] = sum_j A[i][j] * f[j] * g[i-j]
```

- 수식 그대로:
```math
h_i=\sum _{j=\max (0,i-q)}^{\min (i,p)}A[i][j]\, f_j\, g_{i-j}
```

## 🎯 6. 용도 (왜 필요한가?)
- Bezier 곱셈은 NURBS 커널에서 매우 중요한 역할을 한다:
- ✔ Rational curve numerator/denominator 분리
```math
C(u)=\frac{N(u)}{D(u)}
```
- 에서
  - $N(u)$ 와 $D(u)$ 를 곱하거나
  - $D(u)^2$, $N(u)D(u)$ 같은 항이 필요할 때
- ✔ 곡선의 에너지/적분 계산
```math
\int F(u)^2\, du
```
- 이런 항은 Bezier 곱이 필요함.
- ✔ 곡률, 미분, 최적화
- rational derivative는 다음 꼴이 자주 등장:
```math
\frac{N'(u)D(u)-N(u)D'(u)}{D(u)^2}
```
- 여기서 $D(u)^2$ 는 Bezier 곱.
- ✔ B-spline 곱셈의 기본 building block
  - B-spline 곱셈은
  - knot refinement
  - Bezier 분해
  - Bezier 곱
  - knot removal
- 로 구성되는데 그 중 Bezier 곱이 바로 이 matrix.

## 🎯 7. 수식에 문제 있는지 점검
- ✔ binomial coefficient 계산
  - u128로 정확하게 계산 → 매우 좋음
  - floating drift 없음.
- ✔ A[i][j]의 정의
  - 이론과 100% 일치.
- ✔ symmetry fill
```math
A[i][j]=A[n-i][m-j]
```
- 이건 Bernstein 곱의 대칭성에서 나오는 성질이며
- 수학적으로 완전히 맞다.
- ✔ boundary
  - A[0][0]=1
  - A[n][m]=1

- ✔ 테스트
- 세 가지 테스트 모두 수학적으로 타당하고
- 특히 마지막 테스트는 함수 곱과 Bezier 곱이 일치함을 직접 검증하므로 매우 강력함.

## 🎯 8. 결론: 수식은 완벽하다
- bezier_product_matrix는 Bezier 곱셈의 정석 공식을 정확하게 구현하고 있다.
  - 수식 문제 없음
  - 구현 문제 없음
  - 테스트도 수학적으로 타당
  - 수치적으로도 안정적 (binomial을 u128로 계산한 덕분)
- 즉, 이 부분은 완벽하게 맞다.

---

## 테스트 코드
```rust
/// Compute Bezier product matrix A for degrees p and q.
/// Output matrix A has size (n+1) x (m+1), where:
///   n = p+q, m = p
///   A[i][j] = (1 / C(n,i)) * C(p,j) * C(q, i-j)  for i in 1..=floor(n/2), j range valid
///   A[i][j] = A[n-i][m-j] for i > floor(n/2)
///
/// This matrix is used to compute control points of the product Bezier curve/function:
///   Given Bezier control vectors f[0..p], g[0..q],
///   Product controls h[0..n] can be formed by:
///     h[i] = sum_{j} A[i][j] * f[j] * g[i-j]  (j valid)
pub fn on_bezier_product_matrix(p: usize, q: usize) -> Vec<Vec<Real>> {
    let n = p + q;
    let m = p;

    let bin = on_pascal_triangle_u128(n);

    let mut a = vec![vec![0.0 as Real; m + 1]; n + 1];

    // boundary rows
    a[0][0] = 1.0;
    a[n][m] = 1.0;

    let r = n / 2;

    for i in 1..=r {
        let inv = 1.0 / (bin[n][i] as Real);

        let jl = if i >= q { i - q } else { 0 };
        let jh = if i <= p { i } else { p };

        for j in jl..=jh {
            let cij = (bin[p][j] as Real) * (bin[q][i - j] as Real);
            a[i][j] = inv * cij;
        }
    }

    // symmetry fill
    for i in (r + 1)..n {
        let jl = if i >= q { i - q } else { 0 };
        let jh = if i <= p { i } else { p };

        for j in jl..=jh {
            a[i][j] = a[n - i][m - j];
        }
    }

    a
}
```
```rust
pub fn on_product_ctrl_via_matrix(f: &[Real], g: &[Real], p: usize, q: usize) -> Vec<Real> {
    let a = on_bezier_product_matrix(p, q);
    let n = p + q;
    let mut h = vec![0.0; n + 1];

    for i in 0..=n {
        let jl = if i >= q { i - q } else { 0 };
        let jh = if i <= p { i } else { p };

        let mut sum = 0.0;
        for j in jl..=jh {
            sum += a[i][j] * f[j] * g[i - j];
        }
        h[i] = sum;
    }
    h
}
```
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
```rust
pub fn on_pascal_triangle_u128(n: usize) -> Vec<Vec<u128>> {
    let mut bin = vec![vec![0u128; n + 1]; n + 1];

    bin[0][0] = 1;
    if n == 0 {
        return bin;
    }

    bin[1][0] = 1;
    bin[1][1] = 1;
    if n == 1 {
        return bin;
    }

    for k in 2..=n {
        bin[k][0] = 1;
        let r = k / 2;
        let mut tmp2: u128 = 1;
        for j in 1..=r {
            let tmp1 = bin[k - 1][j];
            let v = bin[k - 1][j] + tmp2;
            bin[k][j] = v;
            bin[k][k - j] = v;
            tmp2 = tmp1;
        }
        bin[k][k] = 1;
    }
    bin
}
```
---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::types::{Real};
    use nurbslib::core::basis::{on_bezier_product_matrix, on_product_ctrl_via_matrix, on_eval_bezier_cfun};

    #[test]
    fn bz_prod_mat_boundary_values() {
        let a = on_bezier_product_matrix(3, 2);
        assert!((a[0][0] - 1.0).abs() < 1e-15);

        let n = 5usize;
        let m = 3usize;
        assert!((a[n][m] - 1.0).abs() < 1e-15);
    }
```
```rust
    #[test]
    fn bz_prod_mat_symmetry_property() {
        let p = 5usize;
        let q = 4usize;
        let a = on_bezier_product_matrix(p, q);
        let n = p + q;
        let m = p;

        for i in 0..=n {
            for j in 0..=m {
                // only compare entries that are "valid range" for i,j
                let jl = if i >= q { i - q } else { 0 };
                let jh = if i <= p { i } else { p };
                if j < jl || j > jh { continue; }

                let v1 = a[i][j];
                let ii = n - i;
                let jj = m - j;

                let jl2 = if ii >= q { ii - q } else { 0 };
                let jh2 = if ii <= p { ii } else { p };
                if jj < jl2 || jj > jh2 { continue; }

                let v2 = a[ii][jj];
                assert!((v1 - v2).abs() < 1e-15);
            }
        }
    }
```
```rust
    #[test]
    fn bz_prod_mat_matches_true_product_samples() {
        let p = 4usize;
        let q = 3usize;

        // deterministic "random-ish" controls
        let f = vec![ 1.2, -0.7,  2.1,  0.3, -1.5 ];
        let g = vec![ 0.9,  1.4, -0.2,  0.6 ];

        let h = on_product_ctrl_via_matrix(&f, &g, p, q);

        for k in 0..=50 {
            let u = (k as Real) / 50.0;
            let fv = on_eval_bezier_cfun(&f, u);
            let gv = on_eval_bezier_cfun(&g, u);
            let hv = on_eval_bezier_cfun(&h, u);

            let refv = fv * gv;
            assert!((hv - refv).abs() < 5e-12, "u={} hv={} ref={}", u, hv, refv);
        }
    }
}
```
---
