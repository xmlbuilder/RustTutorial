# B_Spline Interpolation

B-스플라인 곡선의 최소제곱 근사에서 고정된 양 끝 제어점을 유지하면서 내부 제어점을 추정하는 방식으로 정확하게 구현되어 있습니다.  
아래에 전체 수식 흐름과 각 단계의 수학적 의미를 자세히 설명.

## 📌 문제 정의
주어진 데이터 점 집합 $\{ P_i\} _{i=0}^{n-1}$ 에 대해, B-스플라인 곡선 $C(u)$ 을 다음과 같이 정의합니다:

$$
C(u)=\sum _{j=0}^{m-1}N_{j,p}(u)\cdot C_j
$$

- $N_{j,p}(u)$ : B-스플라인 기저 함수 (degree p, knot vector U)
- $C_j$: 제어점 (control point)
- $m$: 제어점 개수
- $n$: 데이터 점 개수

## 🎯 목표
- 양 끝 제어점 $C_0=P_0$, $C_{m-1}=P_{n-1}$ 고정
- 나머지 $C_1,\dots ,C_{m-2}$ 는 최소제곱 조건으로 추정

## 🧮 수식 전개
### 1. B-스플라인 근사 모델
- 각 데이터 점 P_i에 대해:

$$
P_i\approx \sum _{j=0}^{m-1}N_{j,p}(u_i)\cdot C_j
$$


- 고정 제어점 분리:

$$
P_i\approx N_{0,p}(u_i)C_0+\sum _{j=1}^{m-2}N_{j,p}(u_i)C_j+N_{m-1,p}(u_i)C_{m-1}
$$

- 이제 $C_1$ 부터 $C_{m-2}$ 까지만 미지수
- 나머지는 이미 알고 있는 값


- 고정된 양 끝 제어점을 우변으로 이항:

$$
P_i-N_{0,p}(u_i)C_0-N_{m-1,p}(u_i)C_{m-1}\approx \sum _{j=1}^{m-2}N_{j,p}(u_i)\cdot C_j
$$

- 좌변은 **수정된 데이터 벡터 $\tilde {P}_i$**가 됨
- 우변은 미지수 $C_j$ 들에 대한 선형 조합


- 이를 다음과 같이 정리:

$$
\tilde {P}_i=\sum _{j=1}^{m-2}N_{j,p}(u_i)\cdot C_j
$$

- 여기서

$$
\tilde {P}_i=P_i-N_{0,p}(u_i)C_0-N_{m-1,p}(u_i)C_{m-1}
$$

#### 🧠 왜 이렇게 해야 할까?

| 항목                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| 고정 제어점 $C_0, C_{m-1}$ | 이미 알고 있는 값이므로 미지수에서 제외함                                      |
| 원래 모델                | $P_i \approx \sum_{j=0}^{m-1} N_{j,p}(u_i) \cdot C_j$             |
| 고정 항 이항             | $P_i - N_{0,p}(u_i)C_0 - N_{m-1,p}(u_i)C_{m-1} \approx \sum_{j=1}^{m-2} N_{j,p}(u_i) \cdot C_j$ |
| 수정된 데이터 벡터       | $P̃ᵢ = P_i - N_{0,p}(u_i) C_0 - N_{m-1,p}(u_i) C_{m-1}$     |
| 최소제곱 정규방정식      | $A^T A \cdot \mathbf{c} = A^T \mathbf{b}$                         |
| 목적                     | 내부 제어점만을 대상으로 안정적이고 효율적인 최소제곱 근사 수행         |


### 2. 최소제곱 문제
위 식을 행렬 형태로 정리하면:

$$
A\cdot \mathbf{c}\approx \mathbf{b}
$$

- $A\in \mathbb{R^{\mathnormal{n\times (m-2)}}}$: 내부 제어점에 대한 기저 함수 값
- $\mathbf{c}\in \mathbb{R^{\mathnormal{(m-2)\times 3}}}$: 내부 제어점 좌표 (x, y, z)
- $\mathbf{b}\in \mathbb{R^{\mathnormal{n\times 3}}}$: 수정된 데이터 점

### 3. 정규방정식 (Normal Equation)
- 최소제곱 해는 다음을 만족:

$$
A^TA\cdot \mathbf{c}=A^T\mathbf{b}
$$

- $G=A^TA$: 그람 행렬 (symmetric, positive semi-definite)
- $rhs=A^T\mathbf{b}$: 우변 벡터

- 4. 선형 시스템 해법
- Cholesky 분해: $G=LL^T$  
    → $Ly=rhs$, $L^Tc=y$
- 실패 시 가우스 소거로 대체

## ✅ 수학적 검증 요약

| 단계                         | 수학적 표현                          | 설명 및 검증 상태                          |
|------------------------------|--------------------------------------|--------------------------------------------|
| 데이터 모델링                | $P_i \approx \sum_{j=0}^{m-1} N_{j,p}(u_i) \cdot C_j$ | B-스플라인 근사 모델 — ✅ 정확함 |
| 양 끝 제어점 고정            | $C_0 = P_0,\quad C_{m-1} = P_{n-1}$ | 경계 조건 적용 — ✅ 정확함       |
| 수정된 데이터 벡터           | $\tilde{P}_i = P_i - N_{0,p}(u_i) C_0 - N_{m-1,p}(u_i) C_{m-1}$ | 고정 제어점 제거 — ✅ 정확함     |
| 최소제곱 정규방정식          | $A^T A \cdot \mathbf{c} = A^T \mathbf{b}$ | 내부 제어점 추정 — ✅ 정확함     |
| 선형 시스템 해법             | Cholesky 또는 Gaussian Elimination | SPD 행렬에 대해 안정적 해법 — ✅ 정확함 |
| 다차원 분리                  | x, y, z 각각 독립적으로 해석        | 좌표축 분리 처리 — ✅ 정확함     |
| 최종 제어점 구성             | $C_0, C_1, \dots, C_{m-2}, C_{m-1}$ | 전체 곡선 구성 — ✅ 정확함       |


## 📌 참고 문헌
- Eberly, D. (Geometric Tools) – Least-Squares Fitting of B-Spline Curves
- The NURBS Book, Piegl & Tiller – Algorithm A9.7 (End-constrained least-squares fitting

## 소스 코드
```rust
fn cholesky_decompose_spd(a: &mut [f64], n: usize) -> bool {
    // a: row-major 상삼각/하삼각 모두 들어있는 dense 대칭
    for i in 0..n {
        for j in 0..=i {
            let mut s = a[i * n + j];
            for k in 0..j {
                s -= a[i * n + k] * a[j * n + k];
            }
            if i == j {
                if s <= 0.0 {
                    return false;
                }
                a[i * n + j] = s.sqrt();
            } else {
                a[i * n + j] = s / a[j * n + j];
            }
        }
        // 상삼각은 0으로 정리(선택)
        for j in (i + 1)..n {
            a[i * n + j] = 0.0;
        }
    }
    true
}
```
```rust
/// Cholesky로 Ax=b 푸는 전진/후진 대치
fn cholesky_solve(a: &[f64], b: &mut [f64], n: usize) {
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
```rust
/// 간단 가우스 소거(부분 피벗) – Cholesky 실패 시 폴백
fn gaussian_solve(mut a: Vec<f64>, mut b: Vec<f64>, n: usize) -> Option<Vec<f64>> {
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
```rust
/// - 첫/끝 제어점은 데이터 양 끝점으로 고정
/// - 내부(m-2) 제어점은 최소제곱으로 추정
/// - 비라셔널(w=1) 가정
pub fn least_squares_end_interpolate(
    points: &[Point],
    degree: usize,  // p
    n_ctrl: usize,  // m
    params: &[f64], // u_i
    knot: &[f64],   // U
) -> Option<Vec<CPoint>> {
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
        cps.push(CPoint::new(points[0].x, points[0].y, points[0].z, 1.0));
        let pe = points[n_data - 1];
        cps.push(CPoint::new(pe.x, pe.y, pe.z, 1.0));
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
        let span = find_span(knot, n_ctrl - 1, p, u);
        let n_vec = basis_funs(knot, span, u, p);

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
    let chol_ok = cholesky_decompose_spd(&mut g_chol, n_unknown);

    let solve_one = |g_dense: &mut [f64], rhs: &mut [f64]| -> Option<Vec<f64>> {
        if chol_ok {
            let a = g_dense.to_vec(); // cholesky_solve는 상삼/하삼 배치로 읽음
            let mut b = rhs.to_vec();
            cholesky_solve(&a, &mut b, n_unknown);
            Some(b)
        } else {
            gaussian_solve(gram_vec.clone(), rhs.to_vec(), n_unknown)
        }
    };

    let xs = solve_one(&mut g_chol, &mut rhs_x)?;
    let ys = solve_one(&mut g_chol, &mut rhs_y)?;
    let zs = solve_one(&mut g_chol, &mut rhs_z)?;

    // 최종 제어점 구성: C0, C1..C_{m-2}, C_{m-1}
    let mut ctrl = Vec::with_capacity(n_ctrl);
    ctrl.push(CPoint::new(p0.x, p0.y, p0.z, 1.0));
    for i in 0..n_unknown {
        ctrl.push(CPoint::new(xs[i], ys[i], zs[i], 1.0));
    }
    ctrl.push(CPoint::new(pend.x, pend.y, pend.z, 1.0));

    Some(ctrl)
}
```

## ✅ End-Constrained Least Squares Interpolation 테스트 요약

| 테스트 함수명                                      | 목적 및 검증 내용                                                                 | 입력 조건                                      | 기대 결과 또는 판정 방식                         |
|---------------------------------------------------|------------------------------------------------------------------------------------|------------------------------------------------|--------------------------------------------------|
| `test_least_squares_line_cubic_clamped`           | x축 직선에 대한 정확한 근사 및 내부 제어점 y≈0 확인                                | p=3, m=4, 직선 샘플 9개                         | 오차 < 1e-9, 내부 CP y ≈ 0                       |
| `test_least_squares_quadratic_like_cubic_fit`     | 포물선 형태 데이터에 대한 근사 정확도 확인                                         | p=3, m=5, y=0.25x², 샘플 21개                   | 오차 < 1e-3                                      |
| `test_least_squares_noisy_data_robustness`        | 노이즈가 있는 직선 데이터에 대한 견고성 확인                                      | p=3, m=4, ±1e-4 노이즈 포함 샘플 21개           | 오차 < 5e-3                                      |
| `test_least_squares_should_panic_on_invalid_fallback` | fallback 실패 시 panic 발생 여부 확인 (의도적 테스트)                        | p=3, m=2, 샘플 2개                              | `#[should_panic(expected = "...")]`로 검증       |
| `test_least_squares_underconstrained_should_fail` | 제어점 수 부족 시 실패하는지 확인                                                 | p=3, m=3, 샘플 2개                              | `None` 반환 확인                                 |
| `test_least_squares_high_degree_dense_data`       | 고차수(p=5) + 고밀도 데이터에 대한 근사 성능 확인                                  | p=5, m=8, y=sin(πx), 샘플 50개                  | 오차 < 1e-2                                      |


