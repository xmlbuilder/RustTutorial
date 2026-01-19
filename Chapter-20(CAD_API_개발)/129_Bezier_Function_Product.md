# Bezier 함수 곱
## 1. 수학적 의미: 두 개의 2변수 Bezier 함수의 곱
### 1.1. bivariate Bezier function 정의
- 두 변수 Bezier 함수 f(u,v) 를 다음과 같이 정의하자:
    - 첫 번째 함수 f: degree (p,q)
    - 두 번째 함수 g: degree (r,s)
```math
f(u,v)=\sum _{i=0}^p\sum _{j=0}^qf_{i,j}\, B_{i,p}(u)\, B_{j,q}(v)
```
```math
g(u,v)=\sum _{k=0}^r\sum _{l=0}^sg_{k,l}\, B_{k,r}(u)\, B_{l,s}(v)
```
- 여기서
- $B_{i,p}(u)$ 는 1D Bernstein 다항식:
```math
B_{i,p}(u)={p \choose i}u^i(1-u)^{p-i}
```

### 1.2. 곱 함수 $h=f\cdot g$
- 두 함수를 곱하면:

```math
h(u,v)=f(u,v)\, g(u,v)
```

- 이를 전개하면:
```math
h(u,v)=\sum _{i=0}^p\sum _{j=0}^q\sum _{k=0}^r\sum _{l=0}^sf_{i,j}g_{k,l}\, B_{i,p}(u)B_{k,r}(u)\, B_{j,q}(v)B_{l,s}(v)
```

- 여기서 핵심은:
```math
B_{i,p}(u)B_{k,r}(u)
```
- 이것은 다시 degree p+r 인 Bernstein basis들의 선형 결합으로 쓸 수 있음:
```math
B_{i,p}(u)B_{k,r}(u)=\sum _{m=0}^{p+r}U_{m,i,k}\, B_{m,p+r}(u)
```
- 마찬가지로 v 방향에 대해:
```math
B_{j,q}(v)B_{l,s}(v)=\sum _{n=0}^{q+s}V_{n,j,l}\, B_{n,q+s}(v)
```
- 여기서
- $U_{m,i,k}$, $V_{n,j,l}$ 가 바로 product matrix 계수들이고,
- 코드에서는 이것이 pmu, pmv 행렬에 저장된다.
- 결국:
```math
h(u,v)=\sum _{m=0}^{p+r}\sum _{n=0}^{q+s}h_{m,n}\, B_{m,p+r}(u)\, B_{n,q+s}(v)
```
- 이때 새 control value $h_{m,n}$ 는
```math
h_{m,n}=\sum _i\sum _j\sum _k\sum _lU_{m,i,k}\, V_{n,j,l}\, f_{i,j}\, g_{k,l}
```
- 라고 쓸 수 있다.

### 1.3. C 코드 / Rust 구현에 대응되는 형태
- C 코드 B_sfnprt 의 핵심 루프는:
```math
fg[i][j] += U[i][k] * V[j][l] * f[k][l] * g[i-k][j-l];
```

- 여기서:
    - i: u 방향 결과 인덱스 (0..p+r)
    - j: v 방향 결과 인덱스 (0..q+s)
    - k: u 방향에서 f의 인덱스
    - l: v 방향에서 f의 인덱스
    - i-k: g의 u 인덱스
    - j-l: g의 v 인덱스
- 정리하면 새 control value $fg_{i,j}$ 는:
```math
fg_{i,j}=\sum _{k=\max (0,i-r)}^{\min (p,i)}\sum _{l=\max (0,j-s)}^{\min (q,j)}U_{i,k}\, V_{j,l}\, f_{k,l}\, g_{i-k,j-l}
```
- 여기서
```math
U_{i,k} = pmu[i][k], \quad V_{j,l} = pmv[j][l].
```
- 즉:
    - u 방향 product matrix: pmu (size: (p+r+1) x (p+1))
    - v 방향 product matrix: pmv (size: (q+s+1) x (q+1))
    - f, g 의 control value 를 2D 합성곱처럼 섞되,
    - 각 방향에 대해 product matrix 계수를 곱해주는 구조다.

### 1.4. 직관적인 해석
- 1D 에서 Bezier function 곱셈은:
- 두 개의 Bezier function을 곱하면 degree가 더해지고, 새 control value는  
    원래 control value들의 **가중합**으로 결정된다.
- 2D 에서는:
    - u 방향 product: (p,r) → p+r
    - v 방향 product: (q,s) → q+s
    - 둘을 tensor-product 형태로 결합
- 결국:
    - f, g 가 각각 smooth한 Bezier surface function이라면
    - 그 곱 h = f*g 역시 smooth한 Bezier surface function이고
    - control value를 이렇게 product matrix 기반으로 계산하면 원래 함수의 곱을 정확히 표현하는 Bezier 표현을 얻는다.

## 2. Rust 함수 요약 (pmu/pmv 사용하는 버전)
- 우리가 만든 함수:
```rust
pub fn on_bezier_surface_function_product_with_matrices(
    f: &[Vec<f64>],
    p: usize,
    q: usize,
    g: &[Vec<f64>],
    r: usize,
    s: usize,
    pmu: &[Vec<f64>],
    pmv: &[Vec<f64>],
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<f64>>, NurbsError>
```

- 수학적으로는:
    - 입력:
        - f: degree (p,q)
        - g: degree (r,s)
        - pmu: u 방향 product matrix (p+r+1 x p+1)
        - pmv: v 방향 product matrix (q+s+1 x q+1)
        - [su, eu]: u 방향 출력 인덱스 범위
        - [sv, ev]: v 방향 출력 인덱스 범위
    - 출력:
        - fg: degree (p+r, q+s) 의 Bezier surface function control value 을 계산한다.

## 소스 코드
```rust
/// Product of two bivariate Bezier functions using precomputed product matrices.
///
/// This is the Rust equivalent of C's B_sfnprt(), but assumes pmu/pmv
/// (U and V product matrices) are already computed and passed in.
///
/// f: (p+1) x (q+1)
/// g: (r+1) x (s+1)
/// pmu: (p+r+1) x (p+1)
/// pmv: (q+s+1) x (q+1)
///
/// fg: (p+r+1) x (q+s+1)
pub fn on_bezier_surface_function_product_with_matrices(
    f: &[Vec<f64>],
    p: usize,
    q: usize,
    g: &[Vec<f64>],
    r: usize,
    s: usize,
    pmu: &[Vec<f64>],
    pmv: &[Vec<f64>],
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<f64>>, NurbsError> {
    // dimension checks
    if pmu.len() != p + r + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("pmu row count must be p+r+1 (got {}, expected {})", pmu.len(), p + r + 1),
        });
    }
    for (i, row) in pmu.iter().enumerate() {
        if row.len() != p + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!("pmu[{}].len() must be p+1 (got {}, expected {})", i, row.len(), p + 1),
            });
        }
    }

    if pmv.len() != q + s + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("pmv row count must be q+s+1 (got {}, expected {})", pmv.len(), q + s + 1),
        });
    }
    for (j, row) in pmv.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!("pmv[{}].len() must be q+1 (got {}, expected {})", j, row.len(), q + 1),
            });
        }
    }

    let pu = p + r;
    let qv = q + s;

    if su > eu || eu > pu {
        return Err(NurbsError::InvalidArgument {
            msg: format!("u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}", su, eu, pu),
        });
    }
    if sv > ev || ev > qv {
        return Err(NurbsError::InvalidArgument {
            msg: format!("v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}", sv, ev, qv),
        });
    }

    // output fg
    let mut fg = vec![vec![0.0_f64; qv + 1]; pu + 1];

    // main computation
    for i in su..=eu {
        for j in sv..=ev {
            let kl = i.saturating_sub(r);
            let kh = p.min(i);
            let ll = j.saturating_sub(s);
            let lh = q.min(j);

            let mut sum = 0.0;

            for k in kl..=kh {
                let uik = pmu[i][k];
                for l in ll..=lh {
                    let vjl = pmv[j][l];
                    sum += uik * vjl * f[k][l] * g[i - k][j - l];
                }
            }

            fg[i][j] = sum;
        }
    }

    Ok(fg)
}
```
```rust
// C의 pmu/pmv에 해당: product matrix를 (degA+degB+1) x (degA+1)로 만든다.
/// U: (p+r+1) x (p+1), V: (q+s+1) x (q+1)
pub fn on_build_product_matrix(deg_a: usize, deg_b: usize) -> Vec<Vec<f64>> {
    let new_deg = deg_a + deg_b;
    let mut m = vec![vec![0.0; deg_a + 1]; new_deg + 1];
    for i in 0..=new_deg {
        // k range는 실제론 max(0,i-deg_b)..min(deg_a,i)만 유효하지만
        // 행렬은 전체 채워도 됨(0이거나 on_product_matrix가 0 주면 OK)
        for k in 0..=deg_a {
            m[i][k] = on_product_matrix(deg_a, deg_b, i, k);
        }
    }
    m
}
```
```rust
pub fn on_build_product_matrix_u(p: usize, r: usize) -> Vec<Vec<f64>> {
    let pu = p + r;
    let mut pmu = vec![vec![0.0_f64; p + 1]; pu + 1];
    for i in 0..=pu {
        for k in 0..=p {
            pmu[i][k] = on_product_matrix(p, r, i, k);
        }
    }
    pmu
}
```
```rust
pub fn on_build_product_matrix_v(q: usize, s: usize) -> Vec<Vec<f64>> {
    let qv = q + s;
    let mut pmv = vec![vec![0.0_f64; q + 1]; qv + 1];
    for j in 0..=qv {
        for l in 0..=q {
            pmv[j][l] = on_product_matrix(q, s, j, l);
        }
    }
    pmv
}
```

## 3. 테스트 코드 예제
### 3.1. 작은 예: 상수 함수 곱
- 가장 단순한 sanity check:
    - f(u,v) = 2 (상수 함수)
    - g(u,v) = 3 (상수 함수)
    - 기대값: h(u,v) = 6 (상수 함수)
- 즉:
    - f 의 모든 control value = 2.0
    - g 의 모든 control value = 3.0
    - h 의 모든 control value = 6.0 이 되어야 한다.
- 이때, 상수 함수이므로 사실 product matrix 값에 상관 없이 곱셈 결과는 control value 전체가 f*g 가 되어야 한다.
- 테스트 코드를 이렇게 짤 수 있다:
```rust
#[cfg(test)]
mod tests_bivariate_bezier_product_with_matrices {
    use crate::core::types::NurbsError;
    use crate::core::bezier_surface_function::on_bezier_surface_function_product_with_matrices;
    use crate::core::basis::on_product_matrix;

    #[test]
    fn test_bivariate_bezier_product_constant_functions() -> Result<(), NurbsError> {
        // f: degree (p,q), g: degree (r,s)
        let p = 1usize;
        let q = 2usize;
        let r = 2usize;
        let s = 1usize;

        // f: (p+1) x (q+1) = 2 x 3, 모두 2.0
        let mut f = vec![vec![0.0; q + 1]; p + 1];
        for i in 0..=p {
            for j in 0..=q {
                f[i][j] = 2.0;
            }
        }

        // g: (r+1) x (s+1) = 3 x 2, 모두 3.0
        let mut g = vec![vec![0.0; s + 1]; r + 1];
        for i in 0..=r {
            for j in 0..=s {
                g[i][j] = 3.0;
            }
        }

        // product matrices
        let pmu = build_product_matrix(p, r); // size (p+r+1) x (p+1)
        let pmv = build_product_matrix(q, s); // size (q+s+1) x (q+1)

        let pu = p + r;
        let qv = q + s;

        let fg = on_bezier_surface_function_product_with_matrices(
            &f,
            p,
            q,
            &g,
            r,
            s,
            &pmu,
            &pmv,
            0,
            pu,
            0,
            qv,
        )?;

        // 기대값: 상수 2 * 3 = 6
        for i in 0..=pu {
            for j in 0..=qv {
                assert!(
                    (fg[i][j] - 6.0).abs() < 1e-9,
                    "fg[{}][{}] = {}, expected 6.0",
                    i,
                    j,
                    fg[i][j]
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_bivariate_bezier_product_simple_linear() -> Result<(), NurbsError> {
        // f(u,v) = u + v  (degree (1,1))
        // g(u,v) = 1      (degree (0,0))
        // h(u,v) = u + v  (same as f)

        let p = 1usize;
        let q = 1usize;
        let r = 0usize;
        let s = 0usize;

        // f control values: degree (1,1)
        // f00, f01
        // f10, f11
        //
        // u ~ [0,1], v ~ [0,1]
        //
        // 간단히 corner 에서 값 지정:
        // f(0,0)=0, f(1,0)=1, f(0,1)=1, f(1,1)=2
        let f = vec![
            vec![0.0, 1.0], // i=0
            vec![1.0, 2.0], // i=1
        ];

        // g(u,v) = 1 (degree 0,0) => control value 하나
        let g = vec![vec![1.0]];

        // product matrices
        let pmu = build_product_matrix(p, r); // (p+r+1) x (p+1) = (2) x (2)
        let pmv = build_product_matrix(q, s); // (q+s+1) x (q+1) = (2) x (2)

        let pu = p + r;
        let qv = q + s;

        let fg = on_bezier_surface_function_product_with_matrices(
            &f,
            p,
            q,
            &g,
            r,
            s,
            &pmu,
            &pmv,
            0,
            pu,
            0,
            qv,
        )?;

        // g=1 이므로 h = f*g = f, 따라서 control value 동일해야 함
        for i in 0..=pu {
            for j in 0..=qv {
                assert!(
                    (fg[i][j] - f[i][j]).abs() < 1e-9,
                    "fg[{}][{}]={}, expected f[{}][{}]={}",
                    i,
                    j,
                    fg[i][j],
                    i,
                    j,
                    f[i][j]
                );
            }
        }

        Ok(())
    }
}
```

- 설명:
    - build_product_matrix(p, q):
    - 이미 구현되어 있는 on_product_matrix(p,q,i,k) 를 이용해서 (p+q+1) x (p+1) 행렬을 만드는 헬퍼.
- 첫 번째 테스트:
    - f = 2, g = 3 → fg = 6 상수인지 확인.
- 두 번째 테스트:
    - g = 1 (degree 0,0) 이므로 f * g = f 가 되어야 함 → control value 동일 확인.
- 이 두 가지로:
    - matrix dimension 체크
    - 인덱스 로직 (kl, kh, ll, lh)
    - pmu/pmv 적용 방식
- 까지 대부분 검증할 수 있다.

---

## 📘 B_SFNPRT 함수의 용도 (핵심 요약)
- 두 개의 2D Bezier Surface Function(스칼라 필드)의 곱을  
    정확한 Bezier Surface Function 형태로 다시 표현하는 함수.
- 즉,
```math
h(u,v)=f(u,v)\cdot g(u,v)
```
- 을 Bezier basis 위에서 다시 표현하기 위해 새로운 control value $h_{i,j}$ 를 계산하는 함수.

## 📘 왜 이런 함수가 필요한가?
- Bezier / NURBS 기반의 CAD·CAE 알고리즘에서는
    **함수의 곱** 이 매우 자주 등장한다.
- 예를 들어:
    - NURBS 곡면의 weight function 곱
    - rational surface의 분자/분모 곱
    - partial derivative 계산 시 등장하는 항들
    - Jacobian determinant 계산
    - surface fitting / approximation
    - implicit surface 변환
    - FEM shape function product
    - CFD boundary condition product
    - Bezier patch 간의 blending
- 이런 곳에서 두 개의 Bezier function을 곱한 결과를 다시 Bezier function으로 표현해야 한다.
- 그때 필요한 것이 바로 B_SFNPRT.

## 📘 함수의 역할을 직관적으로 설명하면
- ✔ 입력
    - f(u,v): degree (p,q)
    - g(u,v): degree (r,s)
- ✔ 출력
    - h(u,v) = f(u,v) * g(u,v): degree (p+r, q+s)
- ✔ 하는 일
    - f 와 g 의 control value 를 **곱** 하는 것이 아니라
    - Bezier basis 간의 곱을 product matrix로 변환하여 정확한 새로운 control value를 계산하는 것
- 즉, 단순한 element-wise multiply 가 아니다.

## 📘 왜 element-wise multiply 가 아닌가?
- Bezier function은:
```math
f(u,v)=\sum f_{i,j}B_{i,p}(u)B_{j,q}(v)
```
- 이기 때문에 control value 자체는 **함수 값** 이 아니다.
- 따라서:
```math
fg[i][j] = f[i][j] * g[i][j]
```

- 이렇게 하면 수학적으로 완전히 틀린 결과가 된다.
- 정확한 곱을 얻으려면:
```math
B_{i,p}(u)B_{k,r}(u)
```
- 을 다시 degree (p+r) 의 Bernstein basis로 변환해야 한다.
- 이 변환을 수행하는 것이 product matrix (pmu, pmv).

## 📘 on_bezier_surface_function_product_with_matrices 의 실제 용도
- 아래는 실제 CAD/CAE 알고리즘에서 이 함수가 쓰이는 대표적인 경우들이다.

### 1) NURBS 곡면의 rational form 계산
- NURBS surface:
```math
S(u,v)=\frac{\sum P_{i,j}w_{i,j}B_{i,p}(u)B_{j,q}(v)}{\sum w_{i,j}B_{i,p}(u)B_{j,q}(v)}
```
- 여기서 분자와 분모는 모두 Bezier surface function.
- 분자 = control point * weight
- 분모 = weight function
- 이 둘을 곱하거나 나누는 과정에서 Bezier function product가 반드시 필요하다.

### 2) Bezier surface의 partial derivative 계산
- 편미분:
```math
\frac{\partial S}{\partial u}=\frac{S_uW-SW_u}{W^2}
```
- 여기서:
    - $S_u$, $W_u$ 는 Bezier function
    - 분자에 product가 들어감
    - 분모 $W^2$ 도 product
- 즉, Bezier function product 없이는 정확한 rational derivative 계산이 불가능하다.

### 3) Bezier patch blending / multiplication
- 두 개의 surface function을 곱해서 새로운 blending function을 만들 때 사용.
- 예:
    - Coons patch
    - Gordon surface
    - T-spline blending
    - FEM shape function blending

- 4) Implicit surface 변환
- Implicit function:
```math
F(x,y,z)=0
```
- 을 Bezier basis로 변환할 때 항들 간의 곱이 등장한다.

## 5) FEM/CFD shape function product
- Finite Element Method 에서:
    - shape function N_i(u,v)
    - Jacobian determinant
    - stiffness matrix integrand
- 이런 것들이 모두 함수의 곱으로 구성된다.
- Bezier 기반 FEM에서는 이 product를 Bezier basis로 변환해야 한다.

## 📘 정리: on_bezier_surface_function_product_with_matrices 의 용도
| 용도                  | 설명                                                         |
|-----------------------|--------------------------------------------------------------|
| Rational NURBS 계산   | 분자/분모 곱, weight function 곱                             |
| Partial derivative    | $\( S_u W - S W_u \)$ 계산 시 product 필요                     |
| Surface blending      | Coons, Gordon, T-spline blending                             |
| FEM/CFD               | shape function product, Jacobian                             |
| Implicit surface      | 항들의 곱을 Bezier basis로 변환                             |
| Surface fitting       | basis 변환 과정에서 product 등장                            |

- 즉,
- ✔ **두 개의 Bezier surface function을 곱한 결과를 정확한 Bezier surface function으로 표현하는 함수**
- 이게 on_bezier_surface_function_product_with_matrices 의 본질적인 용도다.
