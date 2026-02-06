# on_knot_nonrational_bivariate_basis_ders

- 이 함수는 NURBS 커널의 가장 핵심적인 1D B‑spline basis + 미분 계산 루틴.


## 1. 이 함수가 하는 일 (수식 관점)
- 목표
- 주어진:
    - knot vector U
    - 차수 p
    - 파라미터 u
    - span index i
    - 최대 미분 차수 d
- 에 대해 다음을 계산한다:
```math
DU[k][j]=\frac{d^k}{du^k}N_{i-p+j,p}(u)
```
- 여기서:
    - j=0..p: 현재 span에서 non-zero인 basis 함수들
    - k=0..d: 미분 차수
- 즉:
    - 현재 span에서 non-zero인 (p+1)개의 B-spline basis와  
        그들의 0~d차 미분을 모두 계산하는 함수.

- 이 결과는 2D bivariate basis 계산(bd[k][l][i][j])의 핵심 입력이 된다.

## 2. 알고리즘 개요 (Cox–de Boor + 미분 재귀)
- 이 함수는 두 단계로 구성된다.

### 단계 1) ndu 테이블로 basis 함수 계산
- Cox–de Boor 재귀를 다음 형태로 테이블화한 것:
- ndu[j][r]
    - j차 재귀 단계에서 basis 값
    - 최종적으로 ndu[j][p]가 basis $N_{i-p+j,p}(u)$
- 이 테이블은 다음을 만족한다:
```math
ndu[j][r]=\mathrm{basis\  value\  at\  recursion\  depth\  }j
```
- 최종적으로:
```rust
DU[0][j]=ndu[j][p]
```
- 즉, 0차 미분(기본 basis)은 ndu의 마지막 열에서 얻는다.

### 단계 2) a[][] 재귀로 미분 계산
- 미분은 다음 수식을 기반으로 한다:
```math
\frac{d}{du}N_{i,p}(u)=\frac{p}{U_{i+p}-U_i}N_{i,p-1}(u)-\frac{p}{U_{i+p+1}-U_{i+1}}N_{i+1,p-1}(u)
```
- 이걸 k차 미분까지 확장하면:
```math
N_{i,p}^{(k)}(u)=\sum _{j=0}^ka_{k,j}\cdot ndu[rk+j][pk]
```
- 여기서 a[][]는 미분 계수를 저장하는 2×(p+1) 테이블이다.
- 즉:
- ndu 테이블을 기반으로,  
    a[][] 재귀를 이용해 k차 미분을 계산한다.


## 3. 코드 단계별 해설

### (1) degree 0 처리
```rust
if p == 0 {
    out[0][0] = 1.0;
    return Ok(out);
}
```
- 0차 B-spline은 항상 1이므로 미분은 모두 0.

### (2) span 유효성 검사
```rust
if i + p + 1 > m { ... }
```

- basis 계산 시 U[i + j]를 접근하므로  
    span + p + 1이 knot 범위를 넘으면 오류.

### (3) dmax = min(d, p)
```rust
let dmax = d.min(p);
```

- p차 B-spline은 p차 이상 미분하면 0이므로  
    미분은 최대 p까지만 의미가 있다.

### (4) ndu 테이블 초기화
```rust
ndu[0][0] = 1.0;
```
- Cox–de Boor 재귀의 시작점.

### (5) Cox–de Boor 재귀 (basis 계산)
```rust
for j in 1..=p {
    left[j] = u - U[i+1-j]
    right[j] = U[i+j] - u

    for r in 0..j {
        denom = right[r+1] + left[j-r]
        temp = ndu[r][j-1] / denom
        ndu[r][j] = saved + right[r+1] * temp
        saved = left[j-r] * temp
    }
    ndu[j][j] = saved
}
```

- 이 블록은 ndu[][] 계산과 완전히 동일하다.
- 결과적으로:
- $ndu[j][p] = basis N_{i-p+j,p}(u)$

### (6) 0차 미분(기본 basis) 저장
```rust
for j in 0..=p {
    du[0][j] = ndu[j][p];
}
```


### (7) 미분 계산 (a[][] 재귀)
- 이 부분이 가장 복잡하지만, 수식 그대로다.
- 핵심 구조
```rust
for r in 0..=p {        // basis index
    for k in 1..=dmax { // derivative order
        compute du[k][r] using a[][] and ndu[][]
    }
}
```

- 내부 로직
    - rk = r - k
    - pk = p - k
- 이건 미분 시 basis 차수가 줄어드는 구조를 반영한 것.
- a[][] 테이블
```rust
a[s2][0] = a[s1][0] / denom
a[s2][j] = (a[s1][j] - a[s1][j-1]) / denom
a[s2][k] = -a[s1][k-1] / denom
```

- 이게 바로 B-spline 미분의 재귀 공식.
- 최종 미분값
```rust
du[k][r] = dval;
```

### (8) 미분 계수에 factorial 계수 곱하기
```rust
let mut rfac = p as Real;
for k in 1..=dmax {
    for j in 0..=p {
        du[k][j] *= rfac;
    }
    rfac *= (p - k) as Real;
}
```

- 이 부분은 매우 중요하다.
- B-spline 미분 공식에는 다음 계수가 붙는다:
```math
p,\quad p(p-1),\quad p(p-1)(p-2),\dots
``` 
- 즉:
    - 1차 미분: × p
    - 2차 미분: × p(p−1)
    - 3차 미분: × p(p−1)(p−2)
    - …
- 이걸 rfac로 누적해서 곱해주는 단계다.

## 4. 최종 출력 형태
- du[k][j]는 다음을 의미한다:
    - k = 0..d
    - j = 0..p
    - basis index = i - p + j
- 즉:
```math
DU[k][j]=\frac{d^k}{du^k}N_{i-p+j,p}(u)
```
- 이 테이블은 이후:
```rust
bd[k][l][i][j] = du[k][i] * dv[l][j]
```

- 로 확장되어 2D basis 미분 테이블이 된다.


## 5. 전체 흐름 요약
- 이 함수는:
    - Cox–de Boor 재귀로 basis 계산 (ndu)
    - a[][] 재귀로 미분 계산
    - factorial 계수 곱하기
    - du[k][j] 반환
- 그리고 이 결과는:
    - 2D basis 미분 (bd[k][l][i][j])
    - 표면 미분 (sdw[k][l])
    - tangent, normal, curvature 계산
- 의 기반이 된다.

## 📌 CAD 커널 전체 구조에서의 역할 (전체 흐름도)
- 아래는 NURBS 표면 평가 전체 파이프라인을 단계별로 나타낸 흐름도야.
```
┌──────────────────────────────────────────────────────────────┐
│ 1. 입력: NURBS Surface (control net, knot vectors, degrees)  │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 2. 파라미터 (u, v) 입력                                       │
│    - tangent, normal, curvature 계산 시에도 동일              │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 3. span index 찾기 (on_find_span_left_right)                 │
│    - u → usp                                                 │
│    - v → vsp                                                 │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 4. 1D basis + 미분 계산 (★ 지금 설명한 함수 ★)               │
│    on_all_non_vanishing_basis_and_ders_1d                    │
│                                                              │
│    결과:                                                      │
│      du[k][i] = d^k/du^k N_i(u)                              │
│      dv[l][j] = d^l/dv^l M_j(v)                              │
│                                                              │
│    → u 방향 (p+1개), v 방향 (q+1개) basis와 미분을 모두 계     │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 5. 2D bivariate basis 미분 생성                               │
│    on_knot_nonrational_bivariate_basis_ders                  │
│                                                              │
│    bd[k][l][i][j] = du[k][i] * dv[l][j]                      │
│                                                              │
│    → 표면 미분에 필요한 모든 (k,l) 혼합 미분 basis 생성         │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 6. 표면의 homogeneous 미분 계산                               │
│    on_surface_homogeneous_derivatives                        │
│                                                              │
│    SDw[k][l] = Σ_i Σ_j bd[k][l][i][j] * Pw[i,j]              │
│                                                              │
│    → control point와 basis 미분을 곱해 표면 미분 생성          │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 7. 필요 시 Euclidean 변환                                     │
│    S = (X/W, Y/W, Z/W)                                       │
│    Su = (Xu*W - X*Wu) / W^2                                  │
│    Sv = ...                                                  │
│    Suu, Suv, Svv 등도 동일                                    │
└──────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ 8. 최종 응용                                                  │
│    - 표면 점 S(u,v)                                           │
│    - tangent Su, Sv                                          │
│    - normal Su × Sv                                          │
│    - curvature (Suu, Suv, Svv)                               │
│    - offset surface                                          │
│    - trimming, intersection                                  │
│    - CAD/CAM toolpath                                        │
└──────────────────────────────────────────────────────────────┘
```


## 📌 이 함수가 전체 파이프라인에서 맡는 “정확한 역할”
### ✔ 역할 1: 표면 미분의 가장 기초가 되는 1D basis 미분 계산
- 표면 미분은 결국:
```math
S(u,v)=\sum _i\sum _jN_i(u)M_j(v)P_{i,j}
```
이므로,
```math
\frac{\partial S}{\partial u}=\sum _i\sum _jN_i'(u)M_j(v)P_{i,j}
```
```math
\frac{\partial ^2S}{\partial u^2}=\sum _i\sum _jN_i''(u)M_j(v)P_{i,j}
```
- 이런 식으로 1D basis 미분이 반드시 필요하다.
- 이 함수가 바로 그 1D basis 미분 테이블을 만든다.

### ✔ 역할 2: 2D basis 미분의 입력값 제공
- 2D basis는 단순히 곱이다:
```math
BD[k][l][i][j]=du[k][i]\cdot dv[l][j]
```
- 즉:
    - u 방향 미분 du
    - v 방향 미분 dv
- 이 둘이 있어야 2D basis가 만들어진다.

### ✔ 역할 3: 표면 미분의 모든 차수(k,l)를 계산하는 기반
- 표면 미분은:
```math
S^{(k,l)}(u,v)=\sum _i\sum _jBD[k][l][i][j]\cdot P_{i,j}
```
- 즉:
- 이 함수가 없으면 표면 미분 자체를 계산할 수 없다.


## ✔ 역할 4: CAD 커널의 모든 고급 기능의 기반
- 이 함수는 CAD 커널의 거의 모든 기능의 “기초 연산”이다.
    - tangent vector
    - normal vector
    - curvature
    - principal curvature
    - offset surface
    - surface-surface intersection
    - trimming curve 생성
    - toolpath 생성
    - tessellation
    - shading normal
    - adaptive refinement
- 이 모든 기능은 결국:
    - $S(u,v)$
    - $S_u,S_v$
    - $S_{uu},S_{uv},S_{vv}$
- 이런 미분값을 필요로 한다.
- 그리고 이 미분값은 전부 이 함수에서 시작된다.

## 📌 전체 파이프라인을 “수식 흐름”으로 요약
- 1D basis → 1D basis 미분 → 2D basis 미분 → 표면 미분 → 기하학적 속성


- 수식으로는:
```math
du[k][i]\quad \mathrm{and}\quad dv[l][j]
```
↓
```math
BD[k][l][i][j]=du[k][i]\cdot dv[l][j]
```
↓
```math
SD_w[k][l]=\sum _i\sum _jBD[k][l][i][j]P_{i,j}^w
```
↓
```math
S,S_u,S_v,S_{uu},S_{uv},S_{vv}
```
↓
- tangent
- normal
- curvature
- offset
- intersection
- tessellation

---

## 소스 코드
```rust
/// Return:
/// - bd[k][l][i][j] = (d^k/du^k N_{ku-p+i,p}(u)) * (d^l/dv^l N_{kv-q+j,q}(v))
///   where i=0..p, j=0..q
/// - usp, vsp: knot span indices
pub fn on_knot_nonrational_bivariate_basis_ders(
    ku: &KnotVector,
    kv: &KnotVector,
    p: Degree,
    q: Degree,
    u: Param,
    v: Param,
    ufl: Side,
    vfl: Side,
    mfl_upper_half_only_when_equal: bool,
    udr: usize,
    vdr: usize,
) -> Result<(Vec<Vec<Vec<Vec<Real>>>>, Index, Index)> {
    let p = p as usize;
    let q = q as usize;

    if ku.knots.is_empty() || kv.knots.is_empty() {
        return Err(NurbsError::InvalidArgument {
            msg: "knot_abd: empty knot vector".into(),
        });
    }
    let mu = ku.knots.len() - 1;
    let mv = kv.knots.len() - 1;

    // parameter range check
    if u < ku.knots[0] || u > ku.knots[mu] || v < kv.knots[0] || v > kv.knots[mv] {
        return Err(NurbsError::InvalidArgument {
            msg: "knot_abd: parameter out of bounds".into(),
        });
    }

    // span indices
    let usp = on_find_span_left_right(ku, p as Degree, u, ufl)?;
    let vsp = on_find_span_left_right(kv, q as Degree, v, vfl)?;

    // Compute DU (udr x (p+1)), DV (vdr x (q+1))
    let du = on_all_non_vanishing_basis_and_ders_1d(ku, p, u, usp, udr)?;
    let dv = on_all_non_vanishing_basis_and_ders_1d(kv, q, v, vsp, vdr)?;

    // allocate BD: [udr+1][vdr+1][p+1][q+1]
    let mut bd = vec![vec![vec![vec![0.0 as Real; q + 1]; p + 1]; vdr + 1]; udr + 1];

    let jump_upper = mfl_upper_half_only_when_equal && (udr == vdr);

    for i in 0..=p {
        for j in 0..=q {
            for k in 0..=udr {
                if jump_upper {
                    // l <= udr-k
                    let lmax = (udr - k).min(vdr);
                    for l in 0..=lmax {
                        bd[k][l][i][j] = du[k][i] * dv[l][j];
                    }
                } else {
                    for l in 0..=vdr {
                        bd[k][l][i][j] = du[k][i] * dv[l][j];
                    }
                }
            }
        }
    }

    Ok((bd, usp, vsp))
}
```
```rust

/// 1D helper: compute all non-vanishing basis functions and derivatives up to `d`
/// for span `i` (where u in [U[i],U[i+1]] depending on side).
///
/// Output DU[k][j] where:
/// - j=0..p corresponds to basis N_{i-p+j,p}(u)
/// - k=0..d derivatives

pub fn on_all_non_vanishing_basis_and_ders_1d(
    kv: &KnotVector,
    p: usize,
    u: Real,
    span: Index,
    d: usize,
) -> Result<Vec<Vec<Real>>> {
    let uvec = &kv.knots;
    let m = uvec.len() - 1;

    if p == 0 {
        // degree 0: only one basis = 1, all derivatives 0
        let mut out = vec![vec![0.0; 1]; d + 1];
        out[0][0] = 1.0;
        return Ok(out);
    }

    // span must be valid to access U[span + j]
    let i = span as usize;
    if i + p + 1 > m {
        return Err(NurbsError::InvalidArgument {
            msg: format!("basis_ders_1d: span out of range i={} p={} m={}", i, p, m),
        });
    }

    let dmax = d.min(p);

    // ndu is (p+1)x(p+1)
    let mut ndu = vec![vec![0.0 as Real; p + 1]; p + 1];
    let mut left = vec![0.0 as Real; p + 1];
    let mut right = vec![0.0 as Real; p + 1];

    // a is 2 x (p+1)
    let mut a = vec![vec![0.0 as Real; p + 1]; 2];

    // DU is (d+1) x (p+1)
    let mut du = vec![vec![0.0 as Real; p + 1]; d + 1];

    // Basis functions (C: ndu)
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - uvec[i + 1 - j];
        right[j] = uvec[i + j] - u;

        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let denom = ndu[j][r];
            if denom.abs() < 1e-18 {
                return Err(NurbsError::NumericError {
                    msg: format!("basis_ders_1d: ndu denom ~0 at j={} r={}", j, r),
                });
            }
            let temp = ndu[r][j - 1] / denom;
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // Load basis
    for j in 0..=p {
        du[0][j] = ndu[j][p];
    }

    // Derivatives
    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        for k in 1..=dmax {
            let mut dval = 0.0;
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            if r >= k {
                let denom = ndu[(pk + 1) as usize][rk as usize];
                if denom.abs() < 1e-18 {
                    return Err(NurbsError::NumericError {
                        msg: format!("basis_ders_1d: denom ~0 (r>=k) r={} k={}", r, k),
                    });
                }
                a[s2][0] = a[s1][0] / denom;
                dval = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk {
                k - 1
            } else {
                (p - r) as usize
            };

            for j in j1..=j2 {
                let denom = ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                if denom.abs() < 1e-18 {
                    return Err(NurbsError::NumericError {
                        msg: format!("basis_ders_1d: denom ~0 mid r={} k={} j={}", r, k, j),
                    });
                }
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / denom;
                dval += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            if r as isize <= pk {
                let denom = ndu[(pk + 1) as usize][r];
                if denom.abs() < 1e-18 {
                    return Err(NurbsError::NumericError {
                        msg: format!("basis_ders_1d: denom ~0 tail r={} k={}", r, k),
                    });
                }
                a[s2][k] = -a[s1][k - 1] / denom;
                dval += a[s2][k] * ndu[r][pk as usize];
            }

            du[k][r] = dval;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // Multiply through by correct factors (C block)
    let mut rfac = p as Real;
    for k in 1..=dmax {
        for j in 0..=p {
            du[k][j] *= rfac;
        }
        rfac *= (p - k) as Real;
    }

    // For k > p, derivatives are 0 (already initialized)
    Ok(du)
}
```
---
