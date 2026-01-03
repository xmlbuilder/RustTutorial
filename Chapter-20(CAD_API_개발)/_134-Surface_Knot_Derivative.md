# 📘 NURBS Surface Knot Derivative문서
## 1. 목적
- on_compute_surface_basis_derivative_wrt_knot는 NURBS 표면에서 특정 knot $t_k$ 에 대해  
    모든 non-vanishing basis function의 편미분을 계산하는 알고리즘이다.
- 즉,
```math
\frac{\partial }{\partial t_k}\left[ N_{i,p}(u)\cdot N_{j,q}(v)\right] 
```
- 을 계산한다.
- 이 값은 다음과 같은 작업에서 필수적이다:
    - Knot insertion / removal의 민감도 분석
    - Surface fitting에서 knot optimization
    - Surface smoothing / fairing
    - Knot refinement 알고리즘의 Jacobian 계산
    - Reverse engineering / parameterization 개선

## 2. on_compute_surface_basis_derivative_wrt_knot 입력과 출력
### 입력 (Input)

| 이름      | 설명 |
|-----------|------|
| knu, knv  | U, V 방향 KnotVector |
| p, q      | U, V 방향 degree |
| k         | 미분할 knot index |
| u, v      | 평가할 파라미터 값 |
| dir       | UDIR 또는 VDIR (미분 방향) |
| flk       | LEFT / RIGHT (knot derivative 방향) |
| ulp, vlp  | u, v가 span의 왼쪽/오른쪽 경계인지 |
| Bk        | 결과 저장 배열 (basis derivative matrix) |
| spn       | span index (출력) |

### 출력 (Output)

| 이름 | 설명 |
|------|------|
| Bk   | basis derivative 값들 (크기: UDIR → (p+2)×(q+1), VDIR → (p+1)×(q+2)) |
| spn  | v 방향 span index |


### 출력
- Bk[i][j]:
    - UDIR일 때: 크기 = $(p+2)\times (q+1)$
    - VDIR일 때: 크기 = $(p+1)\times (q+2)$

## 3. 수식 정리
### 3.1 기본 B-spline basis
```math
N_{i,p}(u)
```
### 3.2 표면 basis
```math
S(u,v)=\sum _{i,j}N_{i,p}(u)N_{j,q}(v)P_{i,j}
```
### 3.3 knot derivative
- 우리가 원하는 값:
```math
\frac{\partial }{\partial t_k}\left[ N_{i,p}(u)N_{j,q}(v)\right] 
```
- 체인 룰:
```math
=\left( \frac{\partial N_{i,p}(u)}{\partial t_k}\right) N_{j,q}(v)+N_{i,p}(u)\left( \frac{\partial N_{j,q}(v)}{\partial t_k}\right)
``` 
- 하지만 on_compute_surface_basis_derivative_wrt_knot 는 UDIR 또는 VDIR 중 하나만 계산한다.

- UDIR:
```math
Bk[i][j]=\frac{\partial N_{i,p}(u)}{\partial t_k}\cdot N_{j,q}(v)
```
- VDIR:
```math
Bk[i][j]=N_{i,p}(u)\cdot \frac{\partial N_{j,q}(v)}{\partial t_k}
```
## 4. 핵심 알고리즘 흐름
- UDIR일 때:
    - on_compute_basis_derivative_wrt_knot
        - U-knot에 대한 basis derivative 계산
    - 결과: Nu[0..p+1]
        - on_compute_all_basis
        - V 방향 basis 계산
- 결과: Nv[0..q]
- 곱셈:
```math
Bk[i][j]=Nu[i]\cdot Nv[j]
```

## 5. Rust 포팅 시 구조
- Rust에서는 다음과 같이 함수가 나뉜다:
    - on_compute_basis_derivative_wrt_knot
    - on_compute_all_basis
    - on_compute_surface_basis_derivative_wrt_knot
- Rust 함수 이름 예시:
```rust
pub fn on_compute_surface_basis_derivative_wrt_knot(
    knu: &KnotVector,
    knv: &KnotVector,
    p: usize,
    q: usize,
    k: usize,
    u: f64,
    v: f64,
    dir: SurfaceDir,
    flk: Side,
    ulp: Side,
    vlp: Side,
    bk: &mut [Vec<f64>],
    span_v: &mut usize,
) -> Result<()>
```


## 6. 사용 예시
```rust
let mut bk = vec![vec![0.0; q+1]; p+2];
let mut span_v = 0;

on_compute_surface_basis_derivative_wrt_knot(
    &knu,
    &knv,
    p,
    q,
    k,
    u,
    v,
    SurfaceDir::UDir,
    Side::Left,
    Side::Left,
    Side::Left,
    &mut bk,
    &mut span_v,
)?;
```


## 7. 실제 사용 시나리오
- ✔ Knot removal error bound (N_toocrb)
    - knot 제거 시 오차를 예측하기 위해
    - basis derivative wrt knot이 필요함
- ✔ Knot optimization
    - surface fitting에서 knot 위치를 최적화할 때
    - Jacobian 계산에 사용
- ✔ Surface fairing
    - knot smoothing
    - knot spacing optimization
- ✔ Reverse engineering
    - CAD 데이터에서 knot distribution을 재구성할 때

## 8. 주의 사항
- knot derivative는 knot multiplicity < degree일 때만 정의됨
- span 경계에서 LEFT/RIGHT 선택이 중요
- basis derivative는 매우 민감하므로 double precision 필수
- Rust에서는 반드시 방어 코드 필요


---

## 소스 코드
```rust
pub fn on_compute_basis_derivative_wrt_knot(
    kv: &KnotVector,
    k: usize,
    p: usize,
    u: f64,
    flk: Side,
    flp: Side,
    nk: &mut [f64],
) -> Result<()> {
    if nk.len() < p + 2 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("Nk must have length >= {}", p + 2),
        });
    }

    // Always initialize output like C-family expectation
    nk[..(p + 2)].fill(0.0);

    let u_vec = kv.as_slice();
    if u_vec.len() < 2 {
        return Err(NurbsError::InvalidArgument {
            msg: "knot vector too short".into(),
        });
    }

    let m = u_vec.len() - 1;
    if m < p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "knot vector too short (m < p+1)".into(),
        });
    }
    if k > m {
        return Err(NurbsError::InvalidArgument {
            msg: format!("knot index {} out of range (0..={})", k, m),
        });
    }

    // C 주석 조건(flK)
    match flk {
        Side::Left => {
            if k == 0 || u_vec[k] == u_vec[k - 1] {
                return Err(NurbsError::InvalidArgument {
                    msg: "LEFT knot-derivative requires k>0 and U[k] != U[k-1]".into(),
                });
            }
        }
        Side::Right => {
            if k >= m || u_vec[k] == u_vec[k + 1] {
                return Err(NurbsError::InvalidArgument {
                    msg: "RIGHT knot-derivative requires k<m and U[k] != U[k+1]".into(),
                });
            }
        }
    }

    // basis index valid range: i in [0, m-p-1]
    let i_max_valid = (m - p - 1) as isize;

    // i = k-p-1 .. k  (may include values outside [0..i_max_valid])
    let start = k as isize - p as isize - 1;
    let end = k as isize;

    for i_is in start..=end {
        let idx = (i_is - start) as usize; // 0..p+1

        // If i is outside basis index range, derivative is 0 (leave as 0.0)
        if i_is < 0 || i_is > i_max_valid {
            continue;
        }

        // 정상 계산
        nk[idx] = on_compute_basis_knot_derivative(kv, i_is as usize, k, p as Degree, u, flk, flp)?;
    }

    Ok(())
}
```

```rust
pub fn on_compute_all_basis(
    kv: &KnotVector,
    p: usize,
    mut u: f64,
    side: Side,
    n: &mut [f64],        // N[0..p]
    span_out: &mut usize, // span index
) -> Result<()> {
    let knots = kv.as_slice();
    let m = knots.len() - 1;

    if n.len() < p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("N must have length >= {}", p + 1),
        });
    }

    // -----------------------------
    // 1) Clamp u into [U[0], U[m]]
    // -----------------------------
    if u < knots[0] {
        u = knots[0];
    }
    if u > knots[m] {
        u = knots[m];
    }

    // -----------------------------
    // 2) Special cases (C 코드 그대로)
    // -----------------------------
    if (u - knots[p]).abs() < f64::EPSILON {
        n[0] = 1.0;
        for k in 1..=p {
            n[k] = 0.0;
        }
        *span_out = p;
        return Ok(());
    }

    if (u - knots[m - p]).abs() < f64::EPSILON {
        n[p] = 1.0;
        for k in 0..p {
            n[k] = 0.0;
        }
        *span_out = m - p - 1;
        return Ok(());
    }

    // -----------------------------
    // 3) Find span
    // -----------------------------
    let span = on_find_span_left_right(kv, p as Degree, u, side)?;
    *span_out = span;

    // -----------------------------
    // 4) Compute basis functions (C 코드 그대로)
    // -----------------------------
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];

    n[0] = 1.0;

    for k in 1..=p {
        left[k] = u - knots[span + 1 - k];
        right[k] = knots[span + k] - u;

        let mut saved = 0.0;

        for l in 0..k {
            let temp = n[l] / (right[l + 1] + left[k - l]);
            n[l] = saved + right[l + 1] * temp;
            saved = left[k - l] * temp;
        }

        n[k] = saved;
    }

    Ok(())
}
```

```rust
pub fn on_compute_surface_basis_derivative_wrt_knot(
    knu: &KnotVector,
    knv: &KnotVector,
    p: usize,
    q: usize,
    k: usize,
    u: f64,
    v: f64,
    dir: SurfaceDir,
    flk: Side,
    ulp: Side,
    vlp: Side,
    bk: &mut [Vec<f64>],
    spn: &mut usize,
) -> Result<()> {
    match dir {
        SurfaceDir::UDir => {
            // NOTE: spn is the V-direction span index (same as C: N_kntalb(knv,...,&spn))
            if bk.len() < p + 2 {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("Bk must have at least {} rows for UDIR", p + 2),
                });
            }
            for row in 0..=p + 1 {
                if bk[row].len() < q + 1 {
                    return Err(NurbsError::InvalidArgument {
                        msg: format!("Bk[{}] must have at least {} columns for UDIR", row, q + 1),
                    });
                }
            }

            let mut nu = vec![0.0; p + 2];
            let mut nv = vec![0.0; q + 1];

            on_compute_basis_derivative_wrt_knot(knu, k, p, u, flk, ulp, &mut nu)?;
            on_compute_all_basis(knv, q, v, vlp, &mut nv, spn)?;

            for i in 0..=p + 1 {
                for j in 0..=q {
                    bk[i][j] = nu[i] * nv[j];
                }
            }
        }

        SurfaceDir::VDir => {
            // NOTE: spn is the U-direction span index (same as C: N_kntalb(knu,...,&spn))
            if bk.len() < p + 1 {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("Bk must have at least {} rows for VDIR", p + 1),
                });
            }
            for row in 0..=p {
                if bk[row].len() < q + 2 {
                    return Err(NurbsError::InvalidArgument {
                        msg: format!("Bk[{}] must have at least {} columns for VDIR", row, q + 2),
                    });
                }
            }

            let mut nu = vec![0.0; p + 1];
            let mut nv = vec![0.0; q + 2];

            on_compute_all_basis(knu, p, u, ulp, &mut nu, spn)?;
            on_compute_basis_derivative_wrt_knot(knv, k, q, v, flk, vlp, &mut nv)?;

            for i in 0..=p {
                for j in 0..=q + 1 {
                    bk[i][j] = nu[i] * nv[j];
                }
            }
        }
    }
    Ok(())
}
```

---
