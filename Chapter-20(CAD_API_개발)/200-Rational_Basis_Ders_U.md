# on_rational_basis_derivatives_wrt_knot_nonvanishing
- 이 함수는 NURBS 이론에서도 가장 난이도 높은 영역(knot‑sensitivity, basis derivative w.r.t. knot)

## 🎯 이 함수가 하는 일 (한 줄 요약)
- NURBS 곡선의 rational basis function $R_i(u)$ 를 특정 knot $u_k$ 에 대해 미분한 값
```math
\frac{\partial R_i(u)}{\partial u_k}
``` 
- 을 계산한다.
즉,
- u 방향 미분이 아니라
- knot 값에 대한 민감도(sensitivity) 를 계산하는 함수다.
- 이건 NURBS 연구/최적화/역문제에서만 쓰는 매우 특수한 기능이다.

## 📌 왜 이런 미분이 필요한가?
- NURBS 곡선은 다음과 같이 정의됨:
```math
C(u)=\sum _iR_i(u)P_i
```
- 여기서
```math
R_i(u)=\frac{N_i^p(u)w_i}{\sum _jN_j^p(u)w_j}
```
- 만약 knot $u_k$ 를 움직이면
    - basis $N_i^p(u)$ 가 변하고
    - 따라서 rational basis $R_i(u)$ 도 변한다
    - 결국 곡선 전체가 변한다
- 이 함수는 바로 그 민감도를 계산한다:
```math
\frac{\partial R_i(u)}{\partial u_k}
```
- 이 값은 다음에 사용된다:
    - ✔ Knot optimization
    - ✔ Knot smoothing
    - ✔ Knot insertion sensitivity
    - ✔ Reverse engineering / fitting
    - ✔ Shape optimization
    - ✔ Error estimation
- 일반 CAD에서는 거의 안 쓰지만,  
    고급 NURBS 연구나 solver coupling에서는 반드시 필요하다.

## 🧠 수식적으로 이 함수가 계산하는 것
- NURBS basis:
```math
R_i(u)=\frac{N_i(u)w_i}{W(u)},\quad W(u)=\sum _jN_j(u)w_j
```
- knot u_k 에 대한 미분:
```math
\frac{\partial R_i(u)}{\partial u_k}=\frac{\frac{\partial N_i(u)}{\partial u_k}w_i\cdot W(u)-N_i(u)
w_i\cdot \frac{\partial W(u)}{\partial u_k}}{W(u)^2}
```
- 여기서
```math
\frac{\partial W(u)}{\partial u_k}=\sum _j\frac{\partial N_j(u)}{\partial u_k}w_j
```
- 즉, 핵심은
- ✔ B‑spline basis $N_i(u)$ 의 knot‑derivative
```math
\frac{\partial N_i(u)}{\partial u_k}
```
- 이걸 계산하는 게 가장 어렵다.
- 코드에서는 이걸 이 함수가 계산한다.
```rust
on_rational_basis_derivative_wrt_knot(cur, i, k, u, flk, flp, ...)
```

## on_rational_basis_derivatives_wrt_knot_nonvanishing 의 역할

### 1) 현재 knot k 주변의 non‑vanishing basis index 범위 결정

```rust
let mut kk = k;
match flk {
    Side::Left => {
        if u > knots[k] {
            while kk + 1 < knots.len() && on_are_equal(knots[kk], knots[kk + 1], KTOL) {
                kk += 1;
            }
        }
    }
    Side::Right => {
        if u < knots[k] {
            while kk >= 1 && on_are_equal(knots[kk], knots[kk - 1], KTOL) {
                kk -= 1;
            }
        }
    }
}
```

- 즉,
    - knot multiplicity가 있을 때
    - 왼쪽/오른쪽 파생 방향에 따라
    - 실제로 영향을 주는 knot index kk 를 결정한다.
### 2) 영향받는 basis index 범위: i = kk-p-1 .. kk
    - 이 범위는 해당 knot이 영향을 주는 basis 함수들이다.
    - 총 p+2개.
### 3) 각 basis에 대해 knot‑derivative 계산

```rust
rk[t] = on_rational_basis_derivative_wrt_knot(...)
```

- 4) 결과를 Rk[0..p+1] 에 저장

## 📦 입출력 의미 정리
- 입력

| 이름 | 타입 | 의미 |
|------|------|------|
| curve | &NurbsCurve | 대상 NURBS 곡선 |
| k     | usize       | 미분할 knot index (u_k) |
| u     | Real        | 평가할 파라미터 값 |
| flk   | Side        | knot 방향 (LEFT / RIGHT) |
| flp   | Side        | u 방향 (LEFT / RIGHT) |
| rk    | &mut [Real] | 출력 배열 (길이 = p+2) |

- 출력
```math
rk[0] = dR_{kk-p-1}(u)/du_k
```
```math
rk[1] = dR_{kk-p}(u)/du_k
```
```math
...
```
```math
rk[p+1] = dR_{kk}(u)/du_k
```

## 🧩 비슷한 함수들과의 차이
| 함수 이름                         | 계산 대상                         | 무엇을 미분/계산하는가?                           |
|-----------------------------------|-----------------------------------|----------------------------------------------------|
| basis                           | B-spline basis $N_i(u)$             | basis 값                                           |
| basis_ders                            | $dN_i/du$                           | basis의 u-미분                                     |
| basis_rat                             | $R_i(u)$                            | rational basis $(N_i * w_i / W)$                     |
| bais_rat_ders                        | $dR_i/du$                           | rational basis의 u-미분                            |
| bais_ers_knots                          | $dN_i/du_k$                         | basis의 knot-derivative (knot에 대한 민감도)       |
| bais_rat_ders_knots  (지금 함수)             | $dR_i/du_k$                         | rational basis의 knot-derivative (최종 민감도)     |
즉:
- bais_ers_knots = basis의 knot‑derivative
- bais_rat_ders_knots = rational basis의 knot‑derivative (최종 결과)

## 🎯 요약
- 이 함수는:
    - ✔ NURBS basis $R_i(u)$ 를
    - ✔ 특정 knot $u_k$ 에 대해
    - ✔ 왼쪽/오른쪽 방향으로
    - ✔ non‑vanishing basis만 골라
    - ✔ p+2개의 민감도 값을 계산하는 함수
    - 즉, 곡선이 knot 변화에 얼마나 민감한지를 계산하는 고급 함수다.

---
## 소스 코드
```rust
/// Derivative of rational basis R_i(u) with respect to knot u_k.
/// Rust version of N_kntrck.
/// flk = LEFT/RIGHT → derivative wrt knot direction
/// flp = LEFT/RIGHT → derivative wrt parameter u
pub fn on_rational_basis_derivative_wrt_knot(
    curve: &NurbsCurve,
    i: usize,
    k: usize,
    u: Real,
    flk: Side,
    flp: Side,
) -> Result<Real, NurbsError> {
    // ---- basic validity ----
    let cv_count = curve.cv_count();
    if cv_count == 0 {
        return Err(NurbsError::InvalidArgument {
            msg: "curve has no control points".into(),
        });
    }
    let n = cv_count - 1;

    if i > n {
        return Err(NurbsError::InvalidArgument {
            msg: format!("basis index {} out of range 0..{}", i, n),
        });
    }

    // ---- 1) denominator W(u) ----
    let cfn = curve.extract_denominator_cfun()?;

    // 여기서는 der=0 호출로 값만 획득
    let d0 = {
        let d = cfun_derivatives(&cfn, u, flp, 0)?;
        if d.is_empty() {
            return Err(NurbsError::NumericError {
                msg: "cfun_derivatives returned empty".into(),
            });
        }
        d[0]
    };

    if d0.abs() < ON_ZERO_TOL {
        return Err(NurbsError::NumericError {
            msg: "denominator W(u) is zero".into(),
        });
    }

    // ---- 2) ∂N_i/∂U_k ----
    let nk = on_compute_basis_knot_derivative(&curve.kv, i, k, curve.degree, u, flk, flp)?;

    // ---- 3) ∂W/∂U_k ----
    let fd = on_cfun_derivative_wrt_knot(&cfn, k, u, flk, flp)?;

    // ---- 4) R_i(u) ----
    // rational_basis_single는 "값"만 계산하는 저수준 유틸
    let knots = curve.kv.as_slice();
    let weights = curve.weights_vec(); 
    let p = curve.degree as usize;

    let r = on_rational_basis_single(knots, weights.as_slice(), p, i, u);

    // ---- 5) final ----
    let wi = curve.weight(i).unwrap_or(1.0);
    Ok((wi * nk - fd * r) / d0)
}
```
```rust
/// Derivatives of all non-vanishing rational basis functions w.r.t. a knot u_k.
/// - computes values for indices i = kk-p-1 .. kk (p+2 values)
/// - stores them into `rk[0..p+2]` in that order.
///
/// Notes:
/// - Caller must provide `rk.len() == (p+2)`
/// - Requires knot multiplicity at `k` to be < degree
pub fn on_rational_basis_derivatives_wrt_knot_nonvanishing(
    curve: &NurbsCurve,
    k: usize,
    u: Real,
    flk: Side, // LEFT/RIGHT derivative w.r.t knot direction
    flp: Side, // LEFT/RIGHT derivative w.r.t parameter u
    rk: &mut [Real],
) -> Result<()> {
    let p = curve.degree as usize;

    // ---- output size check: Rk[0..p+1] (p+2 values) ----
    if rk.len() != p + 2 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("rk must have length p+2={}, got {}", p + 2, rk.len()),
        });
    }

    // ---- knot vector + index validity ----
    let knots = curve.kv.as_slice();
    if knots.is_empty() {
        return Err(NurbsError::InvalidArgument {
            msg: "curve knot vector is empty".into(),
        });
    }
    if k >= knots.len() {
        return Err(NurbsError::InvalidArgument {
            msg: format!("k={} out of range 0..{}", k, knots.len() - 1),
        });
    }

    // ---- multiplicity check: mult(k) < p (C says "less than degree") ----
    // (Your project may already have a multiplicity routine. This is local & safe.)
    let mk = on_knot_multiplicity(knots, k);
    if mk >= p {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "knot multiplicity must be < degree (mult={}, degree={})",
                mk, p
            ),
        });
    }

    // Use an equality tol for knot comparison to avoid floating noise.
    const KTOL: Real = 0.0; // knots should be exact in your pipeline; keep 0 unless you must relax
    let mut kk = k;

    match flk {
        Side::Left => {
            if u > knots[k] {
                while kk + 1 < knots.len() && on_are_equal(knots[kk], knots[kk + 1], KTOL) {
                    kk += 1;
                }
            }
        }
        Side::Right => {
            if u < knots[k] {
                while kk >= 1 && on_are_equal(knots[kk], knots[kk - 1], KTOL) {
                    kk -= 1;
                }
            }
        }
    }

    // ---- range i = kk-p-1 .. kk  (p+2 entries) ----
    // INDEX and assumes range is valid for basis derivative routine.
    // We'll validate to avoid underflow.
    if kk + 1 < p + 2 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "invalid kk/p combination: kk={}, p={}, requires kk >= p+1",
                kk, p
            ),
        });
    }

    let i0 = kk - (p + 1); // kk - p - 1
    let i1 = kk;           // inclusive

    // ---- fill output ----
    // C: Rk[i-kk+p+1] where i from i0..=i1
    for (t, i) in (i0..=i1).enumerate() {
        rk[t] = on_rational_basis_derivative_wrt_knot(curve, i, k, u, flk, flp)?;
    }

    Ok(())
}
```
---

