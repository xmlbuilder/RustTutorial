## on_cholesky_solve
- 이 함수는 **Cholesky 분해된 행렬 L을 이용해 Ax = b를 푸는 전진/후진 대치(forward/backward substitution)**  를 구현한 코드.
- 앞서 만든 on_cholesky_decompose_spd와 함께 쓰면 SPD 행렬 A에 대해  
    매우 빠르고 안정적인 선형 시스템 해법이 완성된다.

### 📘 on_cholesky_solve(a, b, n)
- Cholesky 분해된 행렬 L을 이용해 Ax = b를 푸는 전진/후진 대치

### 1️⃣ 전제 조건
- 이 함수는 다음을 가정한다:
    - a는 Cholesky 분해된 하삼각 행렬 L  
        (즉, A = L·Lᵀ)
- b는 입력 시 우변 벡터 b,
    - 출력 시 해 x로 덮어쓰기(overwrite)됨
- n은 행렬 크기
    - 즉, 이 함수는 다음을 푼다:
```math
Ax=b,\quad A=LL^{\top }
```
- 이를 두 단계로 나눈다:
    - 전진 대치: $Ly=b$
    - 후진 대치: $L^{\top }x=y$

### 2️⃣ 전진 대치 (Forward Substitution)
```rust
for i in 0..n {
    let mut s = b[i];
    for k in 0..i {
        s -= a[i*n + k] * b[k];
    }
    b[i] = s / a[i*n + i];
}
```

- 수식:
```math
y_i=\frac{1}{L_{ii}}\left( b_i-\sum _{k=0}^{i-1}L_{ik}y_k\right)
``` 
- 이 단계가 끝나면 b[i]는 모두 $y_i$ 로 바뀐다.

### 3️⃣ 후진 대치 (Backward Substitution)
```rust
for i in (0..n).rev() {
    let mut s = b[i];
    for k in (i+1)..n {
        s -= a[k*n + i] * b[k];
    }
    b[i] = s / a[i*n + i];
}
```

- 여기서 a[k*n + i]는 Lᵀ의 (i,k) 원소에 해당한다.
- 수식:
```math
x_i=\frac{1}{L_{ii}}\left( y_i-\sum _{k=i+1}^{n-1}L_{ki}x_k\right)
``` 
- 이 단계가 끝나면 b[i]는 최종 해 $x_i$ 가 된다.

### 📌 최종 요약
- 이 함수는 다음을 수행한다:
    - 전진 대치로 Ly=b 해결
    - 후진 대치로 $L^{\top }x=y$ 해결
    - 결과를 b에 덮어쓰기

### 📌 기하학적/수치적 활용
- 이 함수는 다음에서 매우 중요해:
    - SPD 행렬 기반 least-squares
    - NURBS fitting / smoothing
    - 물리 시뮬레이션 (mass matrix SPD)
    - 최적화 문제
    - CAD/Geometry에서 안정적인 선형 시스템 해법
- Cholesky는 SPD 행렬에 대해 가장 빠르고 안정적인 해법이기 때문에  
    이 함수는 실전 엔진에서 매우 중요한 구성 요소.

```rust
/// Cholesky로 Ax=b 푸는 전진/후진 대치
fn on_cholesky_solve(a: &[f64], b: &mut [f64], n: usize) {
    // L y = b
    for i in 0..n {
        let mut s = b[i];
        for k in 0..i {
            s -= a[i * n + k] * b[k];
        }
        b[i] = s / a[i * n + i];
    }
    // L^T x = y
    for i in (0..n).rev() {
        let mut s = b[i];
        for k in (i + 1)..n {
            s -= a[k * n + i] * b[k];
        }
        b[i] = s / a[i * n + i];
    }
}
```
## on_gaussian_solve
- 이 코드는 Cholesky 분해가 실패했을 때 사용하는 안전한 폴백(fallback)  
    선형 시스템 해법으로, 부분 피벗(partial pivoting)을 포함한 고전적  
    Gaussian Elimination + Back‑Substitution 을 정확하게 구현한 형태.
- 수치적으로 안정성을 확보하면서도 구현이 간결해서, 실전 CAD/Geometry 엔진에서 자주 쓰는 패턴.

### 📘 on_gaussian_solve(a, b, n)
- 부분 피벗을 포함한 Gaussian Elimination으로 Ax = b를 푸는 폴백(fallback) 솔버

### 1️⃣ 목적
- 이 함수는 다음을 수행한다:
    - 행렬 A와 벡터 b를 받아
    - 부분 피벗 Gaussian 소거로 Ax = b를 풀고
    - 해 x를 반환한다 (Some(x))
- 만약:
    - A가 특이(singular)하거나
    - 피벗이 너무 작아 수치적으로 불안정하면 None을 반환한다.
- 즉, Cholesky가 실패했을 때 사용하는 일반 행렬용 안전한 솔버다.

### 2️⃣ 입력/출력
- 입력
    - a: 크기 n×n의 행렬 A (row-major), 복사본으로 받음
    - b: 크기 n의 벡터 b (복사본)
    - n: 행렬 크기
- 출력
    - 성공 → Some(x)
    - 실패 → None

### 3️⃣ 알고리즘 구조
- 전체 과정은 다음 두 단계로 구성된다:
    - 전진 소거(Forward Elimination)
    - 부분 피벗 선택
    - 행 교환
    - 아래 행들 제거하여 상삼각 행렬 U 생성
    - 후진 대치(Back Substitution)
    - Ux = y 를 풀어 x 계산

### 4️⃣ 코드 상세 해설
- ✔ (1) 부분 피벗 선택
```rust
let mut piv = i;
let mut maxv = a[i*n + i].abs();
for r in (i+1)..n {
    let v = a[r*n + i].abs();
    if v > maxv {
        maxv = v;
        piv = r;
    }
}
if maxv <= 1e-30 { return None; }
```

- 현재 열 i에서 절댓값이 가장 큰 행을 pivot으로 선택
- pivot이 너무 작으면 → 행렬이 특이(singular) → 실패

- ✔ (2) pivot 행 교환
```rust
if piv != i {
    for c in i..n {
        a.swap(i*n + c, piv*n + c);
    }
    b.swap(i, piv);
}
```
- A의 i행 ↔ piv행 교환
- b도 동일하게 교환
- 부분 피벗(partial pivoting)으로 수치 안정성 확보

- ✔ (3) 소거(Elimination)
```rust
let diag = a[i*n + i];
for r in (i+1)..n {
    let f = a[r*n + i] / diag;
    if f == 0.0 { continue; }
    for c in i..n {
        a[r*n + c] -= f * a[i*n + c];
    }
    b[r] -= f * b[i];
}
```

- 수식:
```math
A_{r,*}\leftarrow A_{r,*}-fA_{i,*}
```
```math
b_r\leftarrow b_r-fb_i
```
- 이 과정을 통해 A는 상삼각 행렬 U로 변환된다.

- ✔ (4) 후진 대치 (Back Substitution)
```rust
for i in (0..n).rev() {
    let mut s = b[i];
    for c in (i+1)..n {
        s -= a[i*n + c] * b[c];
    }
    let d = a[i*n + i];
    if d.abs() <= 1e-30 { return None; }
    b[i] = s / d;
}
```

- 수식:
```math
x_i=\frac{1}{U_{ii}}\left( b_i-\sum _{c=i+1}^{n-1}U_{ic}x_c\right)
``` 
- 이 단계가 끝나면 b는 최종 해 x가 된다.

### 📌 최종 요약
- 이 함수는:
    - 부분 피벗 Gaussian 소거로 Ax = b를 해결
    - 특이 행렬 또는 불안정한 pivot 발견 시 None 반환
    - 성공 시 해 x를 Some(x)로 반환
    - Cholesky가 실패했을 때 사용하는 일반 행렬용 폴백 솔버

### 📌 실전 활용
- 이 솔버는 다음 상황에서 매우 유용:
    - Cholesky가 실패한 비-SPD 행렬
    - NURBS fitting / smoothing에서 일반 least-squares
    - CAD/Geometry에서 작은 수치 오차로 SPD가 깨진 경우
    - 물리 시뮬레이션에서 비대칭/비SPD 시스템 처리
- 특히 부분 피벗을 사용한 점이 실전 엔진에서 매우 중요.

```rust
/// 간단 가우스 소거(부분 피벗) – Cholesky 실패 시 폴백
fn on_gaussian_solve(mut a: Vec<f64>, mut b: Vec<f64>, n: usize) -> Option<Vec<f64>> {
    // 증분행렬 [A|b]
    for i in 0..n {
        // pivot
        let mut piv = i;
        let mut maxv = a[i * n + i].abs();
        for r in (i + 1)..n {
            let v = a[r * n + i].abs();
            if v > maxv {
                maxv = v;
                piv = r;
            }
        }
        if maxv <= 1e-30 {
            return None;
        }
        if piv != i {
            for c in i..n {
                a.swap(i * n + c, piv * n + c);
            }
            b.swap(i, piv);
        }
        // eliminate
        let diag = a[i * n + i];
        for r in (i + 1)..n {
            let f = a[r * n + i] / diag;
            if f == 0.0 {
                continue;
            }
            for c in i..n {
                a[r * n + c] -= f * a[i * n + c];
            }
            b[r] -= f * b[i];
        }
    }
    // back-subst
    for i in (0..n).rev() {
        let mut s = b[i];
        for c in (i + 1)..n {
            s -= a[i * n + c] * b[c];
        }
        let d = a[i * n + i];
        if d.abs() <= 1e-30 {
            return None;
        }
        b[i] = s / d;
    }
    Some(b)
}
```
## on_least_squares_end_interpolate

- **양 끝 제어점을 고정한 상태에서 내부 제어점만 최소제곱으로 추정하는 B‑spline curve fitting** 을 구현한 알고리즘.


### 📘 on_least_squares_end_interpolate
- 양 끝 제어점을 고정하고 내부 제어점만 최소제곱으로 추정하는 B‑spline fitting

### 1️⃣ 목적
- 입력:
    - 데이터 점 $P_i$
    - B‑spline 차수 p
    - 제어점 개수 m
    - 파라미터 $u_i$
    - knot vector U
- 출력:
- 제어점 $C_0,C_1,\dots ,C_{m-1}$
- 단,
    - $C_0=P_0$
    - $C_{m-1}=P_{n-1}$
    - 내부 제어점 $C_1\dots C_{m-2}$ 만 최소제곱으로 계산
- 즉,
```math
C_0=P_0,\quad C_{m-1}=P_{n-1}
```
- 을 강제한 상태에서,
```math
\min _{C_1,\dots ,C_{m-2}}\sum _i\| P_i-\sum _jN_{j,p}(u_i)C_j\| ^2
```
을 푸는 알고리즘이다.

### 2️⃣ 내부 제어점 개수
```rust
let n_unknown = n_ctrl - 2;
```

- 첫/끝 제어점은 고정
- 내부 제어점만 미지수
- 미지수 개수 = m-2

### 3️⃣ 선형 시스템 구성
- 최소제곱 문제는 다음의 normal equation으로 귀결된다:
```math
GC=R
```
- 여기서:
    - $G=A^{\top }A$ (Gram matrix)
    - $R=A^{\top }b$
    - A: basis function matrix
    - $b_i=P_i-N_0(u_i)C_0-N_{m-1}(u_i)C_{m-1}$
- 즉, 양 끝 제어점의 영향은 b에 미리 반영하여 내부 제어점만 남긴다.

### 4️⃣ 데이터 점마다 basis function 계산
```rust
let span = on_find_span_index(...);
on_basis_func(span, u, p, knot, &mut n_vec);
```

- span = basis가 non‑zero인 구간
- $N_{span-p},\dots ,N_{span}$ 계산

### 5️⃣ b_i 계산 (경계 제어점 영향 제거)
```rust
bx -= aij * p0.x;
...
bz -= aij * pend.z;
```

- 즉:
```rust
b_i=P_i-N_0(u_i)C_0-N_{m-1}(u_i)C_{m-1}
```

### 6️⃣ 내부 열(1..m-2)만 AᵀA, Aᵀb에 누적
```rust
rhs_x[lj] += aij * bx;
gram_vec[lj * n_unknown + lk] += aij * n_vec[k];
```

- 내부 제어점에 해당하는 basis만 사용
- Gram matrix는 대칭 SPD가 되는 것이 정상

### 7️⃣ 선형 시스템 풀이
- 먼저 Cholesky 시도:
```rust
let chol_ok = on_cholesky_decompose_spd(&mut g_chol, n_unknown);
```

- SPD이면 매우 빠르고 안정적
- 실패하면 Gaussian elimination fallback:
```rust
on_gaussian_solve(...)
```

- 이 구조는 실전 CAD 엔진에서 매우 중요한 안정성 패턴이다.

### 8️⃣ x, y, z 각각 독립적으로 풀이
```rust
let xs = solve_one(...);
let ys = solve_one(...);
let zs = solve_one(...);
```

- B‑spline fitting은 각 좌표가 독립적이므로 3번 풀면 된다.

### 9️⃣ 최종 제어점 구성
```rust
ctrl.push(Point4D::new(p0.x, p0.y, p0.z, 1.0));
for i in 0..n_unknown {
    ctrl.push(Point4D::new(xs[i], ys[i], zs[i], 1.0));
}
ctrl.push(Point4D::new(pend.x, pend.y, pend.z, 1.0));
```

- 즉:
```math
C_0=P_0,\quad C_j=(x_j,y_j,z_j),\quad C_{m-1}=P_{n-1}
```
### 📌 최종 요약
- 이 함수는 다음을 수행한다:
    - 양 끝 제어점을 데이터 양 끝점으로 고정
    - 내부 제어점만 최소제곱으로 추정
    - basis function을 이용해 normal equation 구성
    - Gram matrix를 Cholesky로 풀고, 실패 시 Gaussian fallback
    - 최종적으로 B‑spline 제어점 벡터 반환

```rust
/// - 첫/끝 제어점은 데이터 양 끝점으로 고정
/// - 내부(m-2) 제어점은 최소제곱으로 추정
/// - 비라셔널(w=1) 가정
pub fn on_least_squares_end_interpolate(
    points: &[Point3D],
    degree: usize,  // p
    n_ctrl: usize,  // m
    params: &[f64], // u_i
    knot: &[f64],   // U
) -> Option<Vec<Point4D>> {
    let n_data = points.len();
    if n_data < 2 || n_ctrl < degree + 1 {
        return None;
    }
    if knot.len() != n_ctrl + degree + 1 {
        return None;
    }
    if params.len() != n_data {
        return None;
    }

    // 내부 제어점 개수 (미지수) = m-2, 첫/끝은 고정
    if n_ctrl < 2 {
        return None;
    }
    let n_unknown = n_ctrl - 2;
    if n_unknown == 0 {
        // 제어점이 2개면 직선 – 첫/끝만 반환
        let mut cps = Vec::with_capacity(2);
        cps.push(Point4D::new(points[0].x, points[0].y, points[0].z, 1.0));
        let pe = points[n_data - 1];
        cps.push(Point4D::new(pe.x, pe.y, pe.z, 1.0));
        return Some(cps);
    }

    // 그람행렬 G = A^T A (n_unknown x n_unknown), RHS_x/y/z = A^T (b)
    // b_i = P_i - N_{i,0}*P0 - N_{i,m-1}*P_{m-1}
    let mut gram_vec = vec![0.0f64; n_unknown * n_unknown];
    let mut rhs_x = vec![0.0f64; n_unknown];
    let mut rhs_y = vec![0.0f64; n_unknown];
    let mut rhs_z = vec![0.0f64; n_unknown];

    let p0 = points[0];
    let pend = points[n_data - 1];

    // 한 데이터 점마다 기저 N(span, u) 누적
    let p = degree;
    for i in 0..n_data {
        let u = params[i];
        // find_span: n = m-1
        let span = on_find_span_index(n_ctrl - 1, p as Degree, u, knot);
        let mut n_vec = vec![0.0; n_ctrl + p + 1];
        on_basis_func(span, u, p as Degree, knot, &mut n_vec);

        // b_i = Pi - N0 * P0 - N_last * Pend
        // (여기서 N0는 실제 0번째 열의 계수인지, N_last는 마지막 열 계수인지
        //  — span-p..span 범위 내에서 해당하는 열(0, m-1)이 있으면 그 계수를 쓰는 개념.
        //  하지만 C# 코드는 Equation을 만들어 pos별로 접근했으므로,
        //  동일하게 처리: 내부에서 0 또는 m-1 열이 포함되어 있으면 그만큼 빼 준다.)

        let pi = points[i];
        let mut bx = pi.x;
        let mut by = pi.y;
        let mut bz = pi.z;

        // span 에 해당하는 전역 열 idx = span-p .. span
        let col0 = if span >= p { span - p } else { 0 };
        for j in 0..=p {
            let col = col0 + j;
            let aij = n_vec[j];
            if col == 0 {
                bx -= aij * p0.x;
                by -= aij * p0.y;
                bz -= aij * p0.z;
            } else if col == n_ctrl - 1 {
                bx -= aij * pend.x;
                by -= aij * pend.y;
                bz -= aij * pend.z;
            }
        }

        // 내부 열(1..m-2)에 대해서만 A와 b를 누적 → G += a^T a, rhs += a^T b
        // 내부 열의 로컬 인덱스 = (col-1) in [0..n_unknown-1]
        for j in 0..=p {
            let colj = col0 + j;
            if colj == 0 || colj == n_ctrl - 1 {
                continue;
            }
            let lj = colj - 1; // 0..n_unknown-1
            let aij = n_vec[j];

            // RHS
            rhs_x[lj] += aij * bx;
            rhs_y[lj] += aij * by;
            rhs_z[lj] += aij * bz;

            // G(=A^T A)
            for k in 0..=p {
                let colk = col0 + k;
                if colk == 0 || colk == n_ctrl - 1 {
                    continue;
                }
                let lk = colk - 1;
                gram_vec[lj * n_unknown + lk] += aij * n_vec[k];
            }
        }
    }

    // 이제 G * X = RHS 를 x,y,z 각각에 대해 풉니다.
    // 우선 Cholesky 시도 → 실패 시 가우스 소거 폴백
    let mut g_chol = gram_vec.clone();
    let chol_ok = on_cholesky_decompose_spd(&mut g_chol, n_unknown);

    let solve_one = |g_dense: &mut [f64], rhs: &mut [f64]| -> Option<Vec<f64>> {
        if chol_ok {
            let a = g_dense.to_vec(); // cholesky_solve는 상삼/하삼 배치로 읽음
            let mut b = rhs.to_vec();
            on_cholesky_solve(&a, &mut b, n_unknown);
            Some(b)
        } else {
            on_gaussian_solve(gram_vec.clone(), rhs.to_vec(), n_unknown)
        }
    };

    let xs = solve_one(&mut g_chol, &mut rhs_x)?;
    let ys = solve_one(&mut g_chol, &mut rhs_y)?;
    let zs = solve_one(&mut g_chol, &mut rhs_z)?;

    // 최종 제어점 구성: C0, C1..C_{m-2}, C_{m-1}
    let mut ctrl = Vec::with_capacity(n_ctrl);
    ctrl.push(Point4D::new(p0.x, p0.y, p0.z, 1.0));
    for i in 0..n_unknown {
        ctrl.push(Point4D::new(xs[i], ys[i], zs[i], 1.0));
    }
    ctrl.push(Point4D::new(pend.x, pend.y, pend.z, 1.0));

    Some(ctrl)
}
```
## on_solve_2x2
- 이 세 가지 2×2 선형 시스템 솔버는 정확도·안정성·성능이라는  
    서로 다른 목적을 위해 설계된 훌륭한 계층 구조.

### 📘 2×2 Linear Solver Suite
- Robust, Pivoting, and Fast Solvers for Small Linear Systems

### 1️⃣ Solve2x2Result 구조체
```rust
#[derive(Copy, Clone, Debug)]
pub struct Solve2x2Result {
    pub rank: i32,
    pub x: f64,
    pub y: f64,
    pub pivot_ratio: f64,
}
```

- rank
- 0 → 모든 계수가 0 (해 없음 또는 무한대)
- 1 → 1차원 해 공간 (특이 행렬)
- 2 → 정상(full-rank) 해 존재
- x, y → 해
- pivot_ratio → 수치 안정성 지표
- 작은 값일수록 ill-conditioned
- 1에 가까울수록 안정적

### 2️⃣ on_solve_2x2
- 가장 견고한(robust) 2×2 선형 시스템 솔버 — 완전한 pivoting + rank detection

- ✔ 목적
- 다음 시스템을 푼다:
```math
\left[ \begin{matrix}m_{00}&m_{01}\\ m_{10}&m_{11}\end{matrix}\right] \left[ \begin{matrix}x\\ y\end{matrix}\right] =\left[ \begin{matrix}d_0\\ d_1\end{matrix}\right]
``` 
- 최대 절댓값 피벗 선택 (full pivoting)
- 행/열 스왑
- 정확한 rank 판정
- pivot ratio 계산
- 수치적으로 가장 안전한 방식

- ✔ 핵심 알고리즘 요약
    - 4개 원소 중 절댓값이 가장 큰 pivot 선택
    - pivot이 속한 행/열을 앞으로 스왑
    - 첫 pivot으로 정규화
    - 두 번째 pivot으로 소거
    - 두 번째 pivot이 0이면 rank=1
    - back-substitution
    - 열 스왑 여부에 따라 (x,y) 복원
    - pivot ratio 계산

- ✔ 특징
    - 가장 안정적
    - rank 0/1/2 정확 판정
    - pivot ratio로 condition number 추정 가능
    - CAD/Geometry에서 매우 중요한 “degenerate case” 처리에 최적

### 3️⃣ on_solve_2x2_tuple
- 위 함수의 경량 버전 — 동일한 알고리즘, 반환 형식만 단순화

- ✔ 목적
- on_solve_2x2와 동일한 pivoting 기반 알고리즘:
    - 구조체 대신 (rank, x, y, pivot_ratio) 튜플 반환
    - 코드가 더 간결
    - 동일한 안정성 보장

- ✔ 특징
    - on_solve_2x2와 동일한 수치적 동작
    - 구조체를 만들 필요 없을 때 사용
    - 성능은 거의 동일

### 4️⃣ on_solve_2x2_fast
- 가장 빠른 2×2 솔버 — pivoting 없음, 단순 determinant 방식

- ✔ 목적
- 다음 시스템을 가장 빠르게 푼다:
```math
\left[ \begin{matrix}a&b\\ c&d\end{matrix}\right] \left[ \begin{matrix}s\\ t\end{matrix}\right] =\left[ \begin{matrix}e\\ f\end{matrix}\right]
``` 
- 해:
```math
s=\frac{ed-bf}{ad-bc},\quad t=\frac{af-ec}{ad-bc}
```

- ✔ 안정성 보강
```rust
let scale = max(|a|,|b|,|c|,|d|,|e|,|f|, 1.0);
let det = a*d - b*c;
if det.abs() < ON_TOL12 * scale { return None; }
```
- 입력 값의 크기를 기준으로 determinant가 충분히 큰지 검사
- pivoting은 없지만, 스케일 기반 singularity 체크로 최소한의 안정성 확보

- ✔ 특징
    - 가장 빠름
    - pivoting 없음 → ill-conditioned 행렬에서는 위험
    - CAD에서 “확실히 invertible”인 2×2만 처리할 때 적합
    - fallback 없이 바로 계산해야 할 때 사용

### 📌 세 함수의 비교 요약
| Function            | Stability     | Speed   | Pivoting       | Rank Detection | Use Case                         |
|---------------------|---------------|---------|----------------|----------------|----------------------------------|
| on_solve_2x2        | Highest       | Medium  | Full pivoting  | Yes            | Robust general-purpose solver    |
| on_solve_2x2_tuple  | Highest       | Medium  | Full pivoting  | Yes            | Lightweight return (tuple)       |
| on_solve_2x2_fast   | Low–Medium    | Fastest | None           | No             | When matrix is surely invertible |


### 📌 실전 CAD/Geometry에서의 활용
- 곡선/곡면 fitting
- Jacobian 2×2 inversion
- Newton iteration
- 2D parameter correction
- degenerate case detection
- intersection solver
- 특히 on_solve_2x2는 degenerate geometry(평행, 일치, 매우 작은 각도 등)에서  
    필수적인 안정성을 제공한다.



```rust
#[derive(Copy, Clone, Debug)]
pub struct Solve2x2Result {
    pub rank: i32,
    pub x: f64,
    pub y: f64,
    pub pivot_ratio: f64,
}
```
```rust

pub fn on_solve_2x2(
    mut m00: f64,
    mut m01: f64,
    mut m10: f64,
    mut m11: f64,
    mut d0: f64,
    mut d1: f64,
) -> Solve2x2Result {
    use core::mem;

    // pivot 선택 (최대 절댓값)
    let mut which = 0usize;
    let mut vmax = m00.abs();
    let v01 = m01.abs();
    if v01 > vmax {
        vmax = v01;
        which = 1;
    }
    let v10 = m10.abs();
    if v10 > vmax {
        vmax = v10;
        which = 2;
    }
    let v11 = m11.abs();
    if v11 > vmax {
        vmax = v11;
        which = 3;
    }

    let mut x = 0.0;
    let mut y = 0.0;
    let mut pivot_ratio = 0.0;

    if vmax == 0.0 {
        return Solve2x2Result {
            rank: 0,
            x,
            y,
            pivot_ratio,
        };
    }

    // val5=max pivot, val6=min pivot (초기값은 vmax)
    let mut val5 = vmax;
    let mut val6 = vmax;

    // 열 스왑?
    let mut swapped_cols = false;
    if which % 2 == 1 {
        swapped_cols = true;
        mem::swap(&mut m00, &mut m01);
        mem::swap(&mut m10, &mut m11);
    }
    // 행 스왑?
    if which > 1 {
        mem::swap(&mut d0, &mut d1);
        mem::swap(&mut m00, &mut m10);
        mem::swap(&mut m01, &mut m11);
    }

    // 첫 피벗으로 정규화
    let inv = 1.0 / m00;
    m01 *= inv;
    d0 *= inv;

    // 소거
    if m10 != 0.0 {
        m11 -= m10 * m01;
        d1 -= m10 * d0;
    }

    // 두 번째 피벗 체크 (정확 비교)
    if m11 == 0.0 {
        return Solve2x2Result {
            rank: 1,
            x,
            y,
            pivot_ratio: 0.0,
        };
    }

    // pivot ratio 갱신
    let v = m11.abs();
    if v > val5 {
        val5 = v;
    } else if v < val6 {
        val6 = v;
    }
    pivot_ratio = val6 / val5;

    // back substitution
    d1 /= m11;
    if m01 != 0.0 {
        d0 -= m01 * d1;
    }

    if !swapped_cols {
        x = d0;
        y = d1;
    } else {
        y = d0;
        x = d1;
    }

    Solve2x2Result {
        rank: 2,
        x,
        y,
        pivot_ratio,
    }
}
```
```rust
pub fn on_solve_2x2_tuple(
    mut m00: f64,
    mut m01: f64,
    mut m10: f64,
    mut m11: f64,
    mut d0: f64,
    mut d1: f64,
) -> (i32, f64, f64, f64) {
    let pivot_ratio;

    // choose max-abs pivot in 2x2
    let mut i = 0;
    let mut x = m00.abs();
    let mut y = m01.abs();
    if y > x {
        x = y;
        i = 1;
    }
    y = m10.abs();
    if y > x {
        x = y;
        i = 2;
    }
    y = m11.abs();
    if y > x {
        x = y;
        i = 3;
    }

    if x == 0.0 {
        return (0, 0.0, 0.0, 0.0); // rank 0
    }
    let mut minpiv = x;
    let mut maxpiv = x;

    // track where x/y land if we swap columns
    let mut xy_swapped = false;

    // if pivot column is 1, swap columns 0<->1
    if i % 2 == 1 {
        xy_swapped = true;
        swap(&mut m00, &mut m01);
        swap(&mut m10, &mut m11);
    }
    // if pivot row is 1, swap rows 0<->1 (affects d as well)
    if i > 1 {
        swap(&mut d0, &mut d1);
        swap(&mut m00, &mut m10);
        swap(&mut m01, &mut m11);
    }

    // eliminate
    let inv = 1.0 / m00;
    m01 *= inv;
    d0 *= inv;
    if m10 != 0.0 {
        m11 -= m10 * m01;
        d1 -= m10 * d0;
    }

    if m11 == 0.0 {
        // rank 1
        // x = d0, y will be 0 if m01==0 else free variable; 원 코드와 동일하게 반환만 함
        let (x_ans, y_ans) = if xy_swapped { (0.0, d0) } else { (d0, 0.0) };
        return (1, x_ans, y_ans, 0.0);
    }

    // pivot stats
    let y_abs = m11.abs();
    if y_abs > maxpiv {
        maxpiv = y_abs;
    } else if y_abs < minpiv {
        minpiv = y_abs;
    }

    // back-substitute
    d1 /= m11;
    if m01 != 0.0 {
        d0 -= m01 * d1;
    }

    let (mut x_ans, mut y_ans) = (d0, d1);
    if xy_swapped {
        swap(&mut x_ans, &mut y_ans);
    }

    pivot_ratio = minpiv / maxpiv;
    (2, x_ans, y_ans, pivot_ratio)
}
```
```rust
pub fn on_solve_2x2_fast(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Option<(f64, f64)> {
    let scale = a
        .abs()
        .max(b.abs())
        .max(c.abs())
        .max(d.abs())
        .max(e.abs())
        .max(f.abs())
        .max(1.0);
    let det = a * d - b * c;
    if det.abs() < ON_TOL12 * scale {
        return None;
    }
    let s = (e * d - b * f) / det;
    let t = (a * f - e * c) / det;
    Some((s, t))
}
```
## 아래의 수식은 설명 생략

```rust
#[inline]
pub fn on_cross_2d(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}
```
```rust
#[inline]
pub fn on_cross_vec_2d(a: Point2D, b: Point2D, c: Point2D) -> f64 {
    let ux = b.x - a.x;
    let uy = b.y - a.y;
    let vx = c.x - a.x;
    let vy = c.y - a.y;
    on_cross_2d(ux, uy, vx, vy)
}
```
```rust
#[inline]
pub fn on_signed_area(poly: &[Point2D]) -> f64 {
    let n = poly.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        a += poly[j].x * poly[i].y - poly[i].x * poly[j].y;
        j = i;
    }
    0.5 * a
}
```
```rust
#[inline]
pub fn on_is_convex_ccw(poly: &[Point2D], eps: f64) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }
    for i in 0..n {
        let a = &poly[i];
        let b = &poly[(i + 1) % n];
        let c = &poly[(i + 2) % n];
        let cross = on_cross_2d(b.x - a.x, b.y - a.y, c.x - b.x, c.y - b.y);
        if cross < -eps {
            return false;
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_mat3x3_close(a: [[f64; 3]; 3], b: [[f64; 3]; 3], e: f64) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if !on_are_equal(a[i][j], b[i][j], e) {
                return false;
            }
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_mat4x4_close(a: [[f64; 4]; 4], b: [[f64; 4]; 4], e: f64) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if !on_are_equal(a[i][j], b[i][j], e) {
                return false;
            }
        }
    }
    true
}
```

```rust
#[inline]
pub fn on_cross_2d(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}
```
```rust
#[inline]
pub fn on_cross_vec_2d(a: Point2D, b: Point2D, c: Point2D) -> f64 {
    let ux = b.x - a.x;
    let uy = b.y - a.y;
    let vx = c.x - a.x;
    let vy = c.y - a.y;
    on_cross_2d(ux, uy, vx, vy)
}
```
```rust
#[inline]
pub fn on_signed_area(poly: &[Point2D]) -> f64 {
    let n = poly.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        a += poly[j].x * poly[i].y - poly[i].x * poly[j].y;
        j = i;
    }
    0.5 * a
}
```
```rust
#[inline]
pub fn on_is_convex_ccw(poly: &[Point2D], eps: f64) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }
    for i in 0..n {
        let a = &poly[i];
        let b = &poly[(i + 1) % n];
        let c = &poly[(i + 2) % n];
        let cross = on_cross_2d(b.x - a.x, b.y - a.y, c.x - b.x, c.y - b.y);
        if cross < -eps {
            return false;
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_mat3x3_close(a: [[f64; 3]; 3], b: [[f64; 3]; 3], e: f64) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if !on_are_equal(a[i][j], b[i][j], e) {
                return false;
            }
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_mat4x4_close(a: [[f64; 4]; 4], b: [[f64; 4]; 4], e: f64) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if !on_are_equal(a[i][j], b[i][j], e) {
                return false;
            }
        }
    }
    true
}
```
---
