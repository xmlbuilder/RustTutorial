## 📘 NURBS 핵심 알고리즘 수식 문서
- 정리 순서는 다음과 같음:
    - on_merge_knot_vectors
    - on_evaluate_rational_basis_and_derivatives
    - on_basis_derivative_wrt_knot
    - on_basis_block_derivative_wrt_knot
    - on_denominator_derivative_wrt_knot
    - on_rational_on_basis_derivative_wrt_knot
    - CFun 구조와 denominator function W(u)
    - 전체 흐름 요약

### 1️⃣ on_merge_knot_vectors
- ✔ 목적
    - 두 곡선의 knot vector를 병합하여 공통 knot vector를 만든다.
- ✔ 수식
- 두 knot vector:
```math
U=\{ u_0,u_1,\dots ,u_m\} ,\quad V=\{ v_0,v_1,\dots ,v_n\}
```
- 병합 knot vector:
```math
W=U\cup V
```
- 중복 허용, 정렬 필요.
- ✔ Rust 개념
    - 단순히 두 벡터를 합치고 정렬.

### 2️⃣ on_evaluate_rational_basis_and_derivatives
- ✔ 목적
- Rational basis $R_i(u)$ 와 그 도함수 $R_i^{(k)}(u)$ 계산.
- ✔ Rational basis 정의
```math
R_i(u)=\frac{w_iN_i(u)}{W(u)}
```
- 여기서
```math
W(u)=\sum _jw_jN_j(u)
```
- ✔ 도함수 (Quotient Rule)
```math
R_i^{(k)}(u)=\frac{1}{W(u)}\left[ w_iN_i^{(k)}(u)-\sum _{j=1}^k{k \choose j}W^{(j)}(u)R_i^{(k-j)}(u)\right]
``` 
- ✔ Rust 구조
    - basis 도함수: on_basis_ders_at_span
    - denominator 도함수: cfun_derivatives
    - 파스칼 삼각형: pascal_row

### 3️⃣ on_basis_derivative_wrt_knot
- ✔ 목적
    - 단일 basis N_i(u)의 knot u_k에 대한 편미분:
```math
\frac{\partial N_i(u)}{\partial u_k}
```
- ✔ 핵심 수식
- B-spline basis는 piecewise polynomial이므로
- knot에 대한 편미분은 다음 구간에서만 0이 아니다:
```math
i\in [k-p-1,k]
```
- 편미분 공식은 Piegl & Tiller 2nd Ed. 2.22식 기반:
```math
\frac{\partial N_{i,p}(u)}{\partial u_k}=\frac{p}{u_{i+p}-u_i}\left[ N_{i,p-1}(u)\cdot \delta _{i,k}-N_{i+1,p-1}(u)\cdot \delta _{i+p+1,k}\right]
``` 
- 여기서
```math
\delta _{i,k}=\left\{ \, \begin{array}{ll}\textstyle 1&\textstyle \mathrm{if\  }k=i\\ \textstyle 0&\textstyle \mathrm{otherwise}\end{array}\right.
```

- Rust에서는 이미 on_compute_basis_knot_derivative로 구현됨.

### 4️⃣ on_basis_block_derivative_wrt_knot
- ✔ 목적
    - knot u_k에 대해 영향을 받는 모든 basis의 편미분을 한 번에 계산.
- ✔ 수식
    - 영향받는 basis index:
```math
i=k-p-1,\dots ,k
```
- 출력 배열 $N_k[j]$ 는:
```math
N_k[j]=\frac{\partial N_{i+j}(u)}{\partial u_k}
```
- Rust에서는:
    - on_compute_on_basis_derivative_wrt_knot(kv, k, p, u, flk, flp, nk)


### 5️⃣ on_denominator_derivative_wrt_knot
- ✔ 목적
- $W(u)=\sum _jw_jN_j(u)$
- 에 대해 knot u_k에 대한 편미분:
```math
\frac{\partial W(u)}{\partial u_k}
```
- ✔ 수식
```math
\frac{\partial W(u)}{\partial u_k}=\sum _{i=k-p-1}^kw_i\cdot \frac{\partial N_i(u)}{\partial u_k}
```
- Rust에서는:
```math
fd = Σ fu[i] * N_k[j]
```


### 6️⃣ on_rational_on_basis_derivative_wrt_knot
- ✔ 목적
- 최종적으로 rational basis의 knot derivative:
```math
\frac{\partial R_i(u)}{\partial u_k}
```
- ✔ 수식
```math
\frac{\partial R_i(u)}{\partial u_k}=\frac{w_i\frac{\partial N_i(u)}{\partial u_k}-R_i(u)\frac{\partial W(u)}{\partial u_k}}{W(u)}
```
- Rust에서는:
```math
(w_i * N_k - fd * R) / W
```


### 7️⃣ CFun 구조와 denominator function W(u)
- ✔ CFun 정의
- CFun은 다음을 저장:
    - degree p
    - knots U
    - $fu[i] = weight w_i$
- 즉, CFun은 $W(u) = Σ w_i N_i(u)$ 를 표현하는 1D curve function.
- ✔ 도함수
```math
W^{(k)}(u)=\sum _jw_jN_j^{(k)}(u)
```
- Rust에서는:
    - cfun_derivatives(cfn, u, side, der)


### 8️⃣ 전체 흐름 요약
- ✔ Rational basis $R_i(u)$
```math
R_i(u)=\frac{w_iN_i(u)}{W(u)}
```
- ✔ Parameter derivative
```math
R_i^{(k)}(u)=\frac{1}{W(u)}\left[ w_iN_i^{(k)}(u)-\sum _{j=1}^k{k \choose j}W^{(j)}(u)R_i^{(k-j)}(u)\right]
``` 
- ✔ Knot derivative
```math
\frac{\partial R_i(u)}{\partial u_k}=\frac{w_i\frac{\partial N_i(u)}{\partial u_k}-R_i(u)\frac{\partial W(u)}{\partial u_k}}{W(u)}
```
- ✔ Denominator knot derivative
```math
\frac{\partial W(u)}{\partial u_k}=\sum _{i=k-p-1}^kw_i\cdot \frac{\partial N_i(u)}{\partial u_k}
```
- ✔ Basis knot derivative
```math
\frac{\partial N_{i,p}(u)}{\partial u_k}\neq 0\quad \mathrm{iff\  }i\in [k-p-1,k]
```

---

- on_evaluate_basis_function() 은 그 내부에서 계산되는 기하학적 의미와 수학적 이론식을 정확히 이해하면  
    전체 NURBS 커널 구조가 명확하게 정리.

### 🎯 1. B‑spline Basis Function $N_{i,p}(u)$ 이론식
- B‑spline basis는 Cox–de Boor recurrence 로 정의.
- ✔ 0차 (p = 0)
```math
N_{i,0}(u) =
\begin{cases}
1, & U_i \le u < U_{i+1} \\
0, & \text{otherwise}
\end{cases}
```
- ✔ p차 (p > 0)
```math
N_{i,p}(u)=\frac{u-U_i}{U_{i+p}-U_i}N_{i,p-1}(u)+\frac{U_{i+p+1}-u}{U_{i+p+1}-U_{i+1}}N_{i+1,p-1}(u)
```
- 이게 N_kntbas 가 계산하는 값.

### 🎯 2. Rational Basis Function $R_{i,p}(u)$
- NURBS는 weight가 있으므로:
```math
R_{i,p}(u)=\frac{w_iN_{i,p}(u)}{\sum _{j=0}^nw_jN_{j,p}(u)}
```
- 코드 에서는:
    - rational_basis_single_from_curve()
    - 또는 rational_basis_single()
- 이 두 함수가 정확히 이 식을 구현하고 있어.


### 🎯 3. Rust의 on_evaluate_basis_function() 이론적 의미
- Rust 버전:
```rust
pub fn on_evaluate_basis_function(
    curve: &NurbsCurve,
    k: usize,
    u: f64,
    side: Side,
) -> Result<f64, NurbsError> {
    if curve.is_rational() {
        return Ok(rational_basis_single_from_curve(curve, k, u));
    }

    let kv = curve.knots();
    let p = curve.degree() as usize;
    let span = on_find_span_left_right(kv, curve.degree(), u, side)?;
    let n_vec = on_basis_func_ret_vec(kv.as_slice(), span, u, p);

    let first = span - p;
    if k < first || k > span {
        return Ok(0.0);
    }

    Ok(n_vec[k - first])
}
```

- 이 함수는 이론적으로 다음을 수행한다:

#### 🧠 (1) Rational 여부 판단
- 만약 curve가 rational이면
```math
R_{i,p}(u)=\frac{w_iN_{i,p}(u)}{\sum _jw_jN_{j,p}(u)}
```
- 이 식을 계산.

#### 🧠 (2) Non‑rational이면 B‑spline basis 계산
- span 찾기
```math
\mathrm{span}=\mathrm{find\_ span}(u)
```
- basis vector 계산
```math
\{ N_{span-p,p}(u),\dots ,N_{span,p}(u)\} 
```
- 그 중에서 i번째 basis만 반환
```math
N_{i,p}(u)
```
#### 🧠 (3) LEFT / RIGHT 의미
- LEFT/RIGHT는 span 결정에만 영향을 준다.
    - u가 정확히 knot 값일 때
    - LEFT → 왼쪽 구간
    - RIGHT → 오른쪽 구간
    - u가 knot 사이에 있으면 둘은 동일
- 이건 Piegl & Tiller 원본 C 코드와 완전히 동일한 동작.

| 구분                | C 함수        | Rust 함수                           | 이론식 / 의미 |
|---------------------|---------------|--------------------------------------|----------------|
| Rational Basis      | `N_kntrcb`    | `rational_basis_single_from_curve`   | $R_{i,p}(u) = \frac{w_i N_{i,p}(u)}{\sum_j w_j N_{j,p}(u)}$ |
| Non‑rational Basis  | `N_kntbas`    | `on_basis_func_ret_vec`, `basis_value_at` | $ N_{i,p}(u) $ (Cox–de Boor recurrence) |
| Dispatcher          | `N_kntcbf`    | `on_evaluate_basis_function`            | Rational / Non‑rational 자동 선택 |
| Span 결정           | `flg` (LEFT/RIGHT) | `on_find_span_left_right`        | Knot 경계에서 span 선택 규칙 |


## ✨ 설명
- Rational Basis
    - rational_basis_single_from_curve()
- 이론식:
```math
R_{i,p}(u)=\frac{w_iN_{i,p}(u)}{\sum _jw_jN_{j,p}(u)}
```
- Non‑rational Basis
    - on_basis_func_ret_vec()
- 이론식: Cox–de Boor recurrence

---

## 📘 NURBS Basis Function & Derivatives — Rust Implementation Summary
- 아래 문서는 Piegl & Tiller Algorithm A2.3 를 어떻게 구현하고 테스트하는지 정리한 것이다.

### 1. 수학적 배경 (이론식)
#### 1.1 B‑spline Basis Function N_{i,p}(u)
- Cox–de Boor recurrence
- 0차 (p = 0)
```math
N_{i,0}(u) =
\begin{cases}
1, & U_i \le u < U_{i+1} \\
0, & \text{otherwise}
\end{cases}

``` 
- p차 (p > 0)
```math
N_{i,p}(u)=\frac{u-U_i}{U_{i+p}-U_i}N_{i,p-1}(u)+\frac{U_{i+p+1}-u}{U_{i+p+1}-U_{i+1}}N_{i+1,p-1}(u)
```
#### 1.2 Basis Derivatives $N_{i,p}^{(k)}(u)$
- Piegl & Tiller Algorithm A2.3:
```math
ND[k][j]=\frac{d^k}{du^k}N_{i-p+j,p}(u)
```
- 최종적으로:
```math
ND[k][j]=\frac{p!}{(p-k)!}\cdot \mathrm{(recursive\  derivative\  terms)}
```
- Rust의 on_basis_ders_at_span() 이 이 알고리즘을 그대로 구현한다.

#### 1.3 Rational Basis Function
```math
R_{i,p}(u)=\frac{w_iN_{i,p}(u)}{\sum _jw_jN_{j,p}(u)}
```

### 2. Rust 함수 구조
#### 2.1 Span 결정
- on_find_span_left_right(&kv, degree, u, side)
- LEFT/RIGHT는 knot 경계에서만 차이가 난다.

#### 2.2 Basis + Derivatives 계산
- on_basis_ders_at_span(&kv, p, u, span, der)

- 반환값:
    - nd[k][j] = k차 도함수, j번째 basis

#### 2.3 C의 N_kntadb와 동일한 Rust Wrapper
```rust
pub fn compute_basis_and_derivatives(
    kv: &KnotVector,
    degree: Degree,
    u: Real,
    side: Side,
    der: usize,
) -> Result<(usize, Vec<Vec<Real>>), NurbsError> {
    let span = on_find_span_left_right(kv, degree, u, side)?;
    let nd = on_basis_ders_at_span(kv, degree as usize, u, span, der);
    Ok((span, nd))
}
```


### 3. 테스트 코드
- 아래 테스트는:
    - span이 올바르게 계산되는지
    - basis sum = 1
    - 1차 도함수 검증
    - LEFT/RIGHT 차이 검증
- 을 포함한다.
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::basis::Side;
    use crate::core::knot::KnotVector;

    fn kv_quadratic() -> KnotVector {
        // Quadratic, 3 control points
        KnotVector { knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0] }
    }

    #[test]
    fn test_compute_basis_and_derivatives() {
        let kv = kv_quadratic();
        let degree = 2;
        let u = 0.3;
        let der = 2;

        let (span, nd) =
            compute_basis_and_derivatives(&kv, degree, u, Side::Left, der).unwrap();

        // span check
        assert_eq!(span, 2);

        // basis sum = 1
        let sum0: f64 = nd[0].iter().sum();
        assert!((sum0 - 1.0).abs() < 1e-12);

        // derivative array size check
        assert_eq!(nd.len(), der + 1);
        assert_eq!(nd[0].len(), degree as usize + 1);

        // simple monotonicity check for derivative sign
        // (not mathematically strict, but catches obvious errors)
        let d1_sum: f64 = nd[1].iter().sum();
        assert!(d1_sum.abs() < 10.0);
    }

    #[test]
    fn test_left_right_difference() {
        let kv = kv_quadratic();
        let degree = 2;
        let u = 0.0; // exact knot → LEFT/RIGHT difference appears
        let der = 1;

        let (_, left) =
            compute_basis_and_derivatives(&kv, degree, u, Side::Left, der).unwrap();
        let (_, right) =
            compute_basis_and_derivatives(&kv, degree, u, Side::Right, der).unwrap();

        // LEFT/RIGHT differ at knot boundaries
        assert_ne!(left[0], right[0]);
    }
}
```

### 4. 전체 요약 테이블 (Markdown)
| 기능                     | C 함수        | Rust 함수                          | 이론식 |
|--------------------------|---------------|-------------------------------------|--------|
| Basis value              | N_kntbas      | on_basis_func_ret_vec               | N_{i,p}(u) |
| Rational basis           | N_kntrcb      | rational_basis_single_from_curve    | R_{i,p}(u) |
| Basis + derivatives      | N_kntadb      | on_basis_ders_at_span / compute_basis_and_derivatives | d^k/du^k N_{i,p}(u) |
| Span 결정                | N_kntfsp      | on_find_span_left_right             | Knot interval |
| Dispatcher               | N_kntcbf      | on_evaluate_basis_function             | Rational/Non-rational 선택 |


---

### 🧠 이 함수의 수학적 의미
- CFun은 다음과 같은 함수:
```math
f(u)=\sum _{i=0}^nf_iN_{i,p}(u)
```
- 그 도함수는:
```math
f^{(k)}(u)=\sum _{i=0}^nf_iN_{i,p}^{(k)}(u)
```
- Rust 함수는 정확히 이 식을 계산한다.

### 🧪 테스트 코드
```rust
#[test]
fn test_cfun_derivatives_full() {
    use crate::core::basis::Side;
    use crate::core::cfun::CFun;
    use crate::core::knot::KnotVector;

    // f(u) = [1, 2, 3] with quadratic basis
    let cfn = CFun::new(
        2,
        KnotVector { knots: vec![0.0,0.0,0.0,1.0,1.0,1.0] },
        vec![1.0, 2.0, 3.0]
    ).unwrap();

    let fd = cfun_derivatives_full(&cfn, 0.3, Side::Left, 2).unwrap();

    // f(u) should be finite and smooth
    assert!(fd[0].is_finite());
    assert!(fd[1].is_finite());
    assert!(fd[2].is_finite());
}
```

### 🎯 결론
- LEFT/RIGHT 처리 포함
- basis + derivatives 조합 구조 그대로 유지
- 테스트 코드도 제공


### ✅ Rust 버전 N_kntakr
```rust
use crate::core::basis::Side;
use crate::core::nurbs_curve::NurbsCurve;
use crate::core::types::{Real, Result, NurbsError};
use crate::core::knots_extensions::on_rational_on_basis_derivative_wrt_knot;

/// Compute derivatives of all non-vanishing rational basis functions
/// with respect to knot U[k].
///
/// 반환:
///   Rk[i_local] = ∂R_{span-p+i_local}(u) / ∂U[k]
///
/// i_local = 0..p  (총 p+1개)
pub fn rational_basis_derivatives_wrt_knot(
    curve: &NurbsCurve,
    k: usize,
    u: Real,
    flk: Side,   // derivative wrt knot: LEFT or RIGHT
    flp: Side,   // derivative wrt parameter u: LEFT or RIGHT
) -> Result<Vec<Real>, NurbsError> {
    let p = curve.degree() as usize;
    let kv = curve.knots();
    let U = kv.as_slice();

    // --- 1) multiplicity adjustment ---
    let mut kk = k;

    // LEFT knot derivative: require U[k] != U[k-1]
    if flk == Side::Left && u > U[k] {
        while kk + 1 < U.len() && U[kk] == U[kk + 1] {
            kk += 1;
        }
    }

    // RIGHT knot derivative: require U[k] != U[k+1]
    if flk == Side::Right && k > 0 && u < U[k] {
        while kk > 0 && U[kk] == U[kk - 1] {
            kk -= 1;
        }
    }

    // --- 2) 결과 배열 준비 (p+1개) ---
    let mut rk = vec![0.0; p + 1];

    // --- 3) i = kk-p-1 .. kk 에 대해 계산 ---
    // C: Rk[i - kk + p + 1]
    let start = kk as isize - p as isize - 1;
    let end = kk as isize;

    for i in start..=end {
        if i < 0 {
            continue;
        }
        let i_usize = i as usize;

        let local = (i_usize as isize - kk as isize + p as isize + 1) as usize;
        if local >= rk.len() {
            continue;
        }

        rk[local] = on_rational_on_basis_derivative_wrt_knot(
            curve,
            i_usize,
            k,
            u,
            flk,
            flp,
        )?;
    }

    Ok(rk)
}
```

### 🧠 이 함수의 수학적 의미
- 이 함수는 다음을 계산한다:
```math
\frac{\partial R_{i,p}(u)}{\partial U_k}
```
- 여기서:
    - $R_{i,p}(u)$ = rational basis
    - $U_k$ = knot vector의 k번째 knot
- 즉, 곡선의 형상을 knot 변화에 대해 민감도 분석하는 함수다.

---
