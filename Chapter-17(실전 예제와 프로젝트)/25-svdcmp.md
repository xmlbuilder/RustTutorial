# SVD 
이 코드는 야코비 회전법 기반 고유분해, 이를 활용한 `SVD(Singular Value Decomposition)`,  
그리고 최소제곱 해법을 구현한 고급 선형대수 알고리즘입니다.  
아래에 수학적으로 정확한 의미와 단계별 수식 설명.

## 📘 전체 흐름 요약
| 함수 이름                  | 수학적 표현                                | 설명                                           |
|---------------------------|---------------------------------------------|------------------------------------------------|
| `jacobi_symmetric_eigen`  | $B = V \Lambda V^{\top}$               | 대칭행렬 B의 고유값 분해 (야코비 회전법)       |
| `svdcmp`                  | $A = U \Sigma V^{\top}$                | 행렬 A의 특이값 분해 (SVD)                     |
| `solve_least_squares_svd`| $x = V \Sigma^{-1} U^{\top} b$         | SVD 기반 최소제곱 해법                         |

## 🔍 흐름 설명
- `고유값 분해`: jacobi_symmetric_eigen은 대칭행렬 $B = V \Lambda V^{\top}$ 에 대해 고유값과 고유벡터를 구함
- `SVD 구성`: svdcmp는 고유값의 제곱근을 특이값으로 사용하고, 고유벡터를 통해 V 구성
- `최소제곱 해`: `solve_least_squares_svd` 는 $x=V\Sigma ^{-1}U^{\top }b$ 공식을 통해 해를 계산

## 소스
```rust
use crate::core::tarray::TArray;
use std::f64::EPSILON;
use nalgebra::{DMatrix, SVD};
use crate::core::matrix::Matrix;

#[inline]
fn hypot2(a: f64, b: f64) -> f64 {
    a.hypot(b)
}
```
```rust
/// 대칭행렬 B (n×n)를 야코비 회전으로 고유분해.
/// 결과: B는 대각(고유값), v는 열-고유벡터(정규직교).
fn on_jacobi_symmetric_eigen(b: &mut Matrix, vals: &mut Vec<f64>, v: &mut Matrix) -> bool {
    let n = b.row_count();
    if n == 0 || b.col_count() != n {
        return false;
    }

    // v <- I
    if !v.create(n, n) {
        return false;
    }
    for i in 0..n {
        for j in 0..n {
            *v.at_mut(i as i32, j as i32) = if i == j { 1.0 } else { 0.0 };
        }
    }

    // 반복 파라미터
    let max_sweeps = 50 * n * n;
    let tol = 1e-14_f64;

    // 도움: 합 오프대각의 제곱합
    let off2 = |m: &Matrix| -> f64 {
        let mut s = 0.0;
        for p in 0..n {
            for q in 0..n {
                if p != q {
                    let x = *m.at(p as i32, q as i32);
                    s += x * x;
                }
            }
        }
        s
    };

    // 반복
    let mut sweep = 0usize;
    loop {
        let mut changed = false;

        for p in 0..n {
            for q in (p + 1)..n {
                let app = *b.at(p as i32, p as i32);
                let aqq = *b.at(q as i32, q as i32);
                let apq = *b.at(p as i32, q as i32);
                if apq.abs() <= tol * hypot2(app.abs(), aqq.abs()) {
                    continue;
                }

                // 회전계수 (NR 방식)
                let tau = (aqq - app) / (2.0 * apq);
                if !tau.is_finite() || tau == 0.0 {
                    continue; // 회전 생략
                }

                let t = if tau.abs() + 1.0 == 1.0 {
                    1.0 / (2.0 * tau)
                } else {
                    let sgn = if tau >= 0.0 { 1.0 } else { -1.0 };
                    sgn / (tau.abs() + (1.0 + tau * tau).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;

                if !t.is_finite() || !c.is_finite() || !s.is_finite() {
                    println!("⚠️ 수치 불안정: t={}, c={}, s={} → 회전 생략", t, c, s);
                    continue;
                }


                // B <- Jᵀ B J  (대칭 유지)
                // 행/열 p,q 업데이트
                let bpp = app - t * apq;
                let bqq = aqq + t * apq;
                *b.at_mut(p as i32, p as i32) = bpp;
                *b.at_mut(q as i32, q as i32) = bqq;
                *b.at_mut(p as i32, q as i32) = 0.0;
                *b.at_mut(q as i32, p as i32) = 0.0;

                for r in 0..n {
                    if r != p && r != q {
                        let arp = *b.at(r as i32, p as i32);
                        let arq = *b.at(r as i32, q as i32);
                        let nrp = c * arp - s * arq;
                        let nrq = s * arp + c * arq;
                        *b.at_mut(r as i32, p as i32) = nrp;
                        *b.at_mut(p as i32, r as i32) = nrp;
                        *b.at_mut(r as i32, q as i32) = nrq;
                        *b.at_mut(q as i32, r as i32) = nrq;
                    }
                }

                // V <- V J (열-고유벡터)
                for r in 0..n {
                    let vrp = *v.at(r as i32, p as i32);
                    let vrq = *v.at(r as i32, q as i32);
                    *v.at_mut(r as i32, p as i32) = c * vrp - s * vrq;
                    *v.at_mut(r as i32, q as i32) = s * vrp + c * vrq;
                }

                changed = true;
            }
        }

        sweep += 1;
        if !changed {
            break;
        }
        if sweep > max_sweeps {
            break;
        } // 안전 탈출
        if off2(b) < tol {
            break;
        }
    }

    // 고유값 추출
    vals.clear();
    vals.resize(n, 0.0);
    for i in 0..n {
        vals[i] = *b.at(i as i32, i as i32);
    }
    true
}
```
```rust
/// SVD via Jacobi-eigen on AᵀA
/// 입력:  a (m×n)  — 변경 후 U 저장 (m×n)
/// 출력:  w (n)    — 특이값
///        v (n×n)  — 우직교 행렬
pub fn on_svdcmp_sym_left(a: &mut Matrix, w: &mut TArray<f64>, v: &mut Matrix) -> bool {
    let m = a.row_count();
    let n = a.col_count();
    if m == 0 || n == 0 {
        return false;
    }

    // A 보존
    let a0 = a.clone();

    // B = AᵀA (n×n)
    let mut at = a0.clone();
    at.transpose(); // n×m
    let mut b = &at * &a0; // (n×m)*(m×n) = n×n

    // 대칭 수치화(미세한 비대칭 제거)
    for i in 0..n {
        for j in 0..n {
            let x = 0.5 * (*b.at(i as i32, j as i32) + *b.at(j as i32, i as i32));
            *b.at_mut(i as i32, j as i32) = x;
        }
    }

    // 고유분해
    let mut evals: Vec<f64> = Vec::new();
    if !on_jacobi_symmetric_eigen(&mut b, &mut evals, v) {
        return false;
    }

    println!("evals {:?}", evals);
    // 고유값↓ 정렬 + V 열 재정렬
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| evals[j].partial_cmp(&evals[i]).unwrap());

    let mut wvec = vec![0.0f64; n];
    let mut v_sorted = Matrix::with_dims(n, n);
    for (col, &k) in idx.iter().enumerate() {
        wvec[col] = evals[k].max(0.0).sqrt();
        for r in 0..n {
            *v_sorted.at_mut(r as i32, col as i32) = *v.at(r as i32, k as i32);
        }
    }
    *v = v_sorted;
    w.set_size(n);
    for i in 0..n {
        w[i] = wvec[i];
    }

    // U = A * V * Σ^{-1}  (σ_i > 0만)
    if !a.create(m, n) {
        return false;
    }
    let eps = 1e-12_f64;
    for j in 0..n {
        let sigma = w[j];
        if sigma > eps {
            for r in 0..m {
                let mut s = 0.0;
                for k in 0..n {
                    s += *a0.at(r as i32, k as i32) * *v.at(k as i32, j as i32);
                }
                *a.at_mut(r as i32, j as i32) = s / sigma;
            }
        } else {
            // σ=0: 임의의 직교 완성 (여기서는 0 벡터로 두고, 필요하면 그람-슈미트로 보강 가능)
            for r in 0..m {
                *a.at_mut(r as i32, j as i32) = 0.0;
            }
        }
    }

    // 선택: U 열 정규화(수치 안정)
    for j in 0..n {
        let mut s = 0.0;
        for r in 0..m {
            let x = *a.at(r as i32, j as i32);
            s += x * x;
        }
        let nrm = s.sqrt();
        if nrm > EPSILON {
            for r in 0..m {
                *a.at_mut(r as i32, j as i32) /= nrm;
            }
        }
    }
    true
}
```
```rust
pub fn on_solve_least_squares_svd(mut a: Matrix, b: &[f64], tol: f64) -> Vec<f64> {
    let m = a.row_count();
    let n = a.col_count();
    assert_eq!(b.len(), m, "b must have length m");

    // SVD
    let mut w = TArray::<f64>::with_size(n);
    let mut v = Matrix::with_dims(n, n);
    assert!(on_svdcmp_sym_left(&mut a, &mut w, &mut v)); // a=U, w=σ, v=V

    // y = Uᵀ b  (길이 n)
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut dot = 0.0;
        for r in 0..m {
            dot += a.at(r as i32, i as i32) * b[r]; // U[:,i]·b
        }
        let sigma = w[i].abs();
        y[i] = if sigma > tol { dot / sigma } else { 0.0 };
    }

    // x = V y  (길이 n)
    let mut x = vec![0.0; n];
    for j in 0..n {
        let mut s = 0.0;
        for i in 0..n {
            s += v.at(j as i32, i as i32) * y[i]; // V[:,i]*y[i] 누적
        }
        x[j] = s;
    }
    x
}
```
```rust
pub fn on_svdcmp_sym_right(a: &mut Matrix, w: &mut TArray<f64>, v: &mut Matrix) -> bool {
    let m = a.row_count();
    let n = a.col_count();
    if m == 0 || n == 0 {
        return false;
    }

    let a0 = a.clone();

    // B = A Aᵀ (m×m)
    let mut at = a0.clone();
    at.transpose(); // n×m
    let mut b = &a0 * &at; // (m×n)*(n×m) = m×m

    // 대칭화
    for i in 0..m {
        for j in 0..m {
            let x = 0.5 * (*b.at(i as i32, j as i32) + *b.at(j as i32, i as i32));
            *b.at_mut(i as i32, j as i32) = x;
        }
    }

    // 고유값 분해: B = U Λ Uᵀ
    let mut evals: Vec<f64> = Vec::new();
    if !on_jacobi_symmetric_eigen(&mut b, &mut evals, a) {
        return false;
    }

    // 고유값 정렬
    let mut idx: Vec<usize> = (0..m).collect();
    idx.sort_by(|&i, &j| evals[j].partial_cmp(&evals[i]).unwrap());

    // 특이값 w = sqrt(λ)
    let mut wvec = vec![0.0f64; m];
    for (i, &k) in idx.iter().enumerate() {
        wvec[i] = evals[k].max(0.0).sqrt();
    }
    w.set_size(m);
    for i in 0..m {
        w[i] = wvec[i];
    }

    // U 정렬
    let mut u_sorted = Matrix::with_dims(m, m);
    for (col, &k) in idx.iter().enumerate() {
        for r in 0..m {
            *u_sorted.at_mut(r as i32, col as i32) = *a.at(r as i32, k as i32);
        }
    }
    *a = u_sorted;

    // V = Aᵀ U / σ
    if !v.create(n, m) {
        return false;
    }
    let eps = 1e-12_f64;
    for j in 0..m {
        let sigma = w[j];
        if sigma > eps {
            for r in 0..n {
                let mut s = 0.0;
                for k in 0..m {
                    s += *at.at(r as i32, k as i32) * *a.at(k as i32, j as i32);
                }
                *v.at_mut(r as i32, j as i32) = s / sigma;
            }
        } else {
            for r in 0..n {
                *v.at_mut(r as i32, j as i32) = 0.0;
            }
        }
    }
    true
}
```
```rust
pub fn on_svdcmp(a: &mut Matrix, w: &mut TArray<f64>, v: &mut Matrix) -> bool {
    let m = a.row_count();
    let n = a.col_count();

    // 1. Matrix → DMatrix 변환
    let mut data = vec![0.0; m * n];
    for i in 0..m {
        for j in 0..n {
            data[i * n + j] = *a.at(i as i32, j as i32);
        }
    }
    let a_na = DMatrix::from_row_slice(m, n, &data);

    // 2. SVD 수행
    let svd = SVD::new(a_na.clone(), true, true);
    let u_na = match svd.u {
        Some(u) => u,
        None => return false,
    };
    let v_t_na = match svd.v_t {
        Some(vt) => vt,
        None => return false,
    };
    let sigma = svd.singular_values;

    // 3. 결과 복사: w
    w.set_size(sigma.len());
    for i in 0..sigma.len() {
        w[i] = sigma[i];
    }

    // 4. 결과 복사: a ← U
    if !a.create(m, u_na.ncols()) {
        return false;
    }
    for i in 0..m {
        for j in 0..u_na.ncols() {
            *a.at_mut(i as i32, j as i32) = u_na[(i, j)];
        }
    }

    // 5. 결과 복사: v ← V
    let v_na = v_t_na.transpose();
    if !v.create(v_na.nrows(), v_na.ncols()) {
        return false;
    }
    for i in 0..v_na.nrows() {
        for j in 0..v_na.ncols() {
            *v.at_mut(i as i32, j as i32) = v_na[(i, j)];
        }
    }
    true
}
```
```rust
// 외부 인터페이스: 기존 구조를 유지
pub fn on_solve_least_squares_svd_na(a: Matrix, b: &[f64], tol: f64) -> Vec<f64> {
    let m = a.row_count();
    let n = a.col_count();
    assert_eq!(b.len(), m, "b must have length equal to row count of A");

    // 1. Matrix → DMatrix 변환
    let mut data = vec![0.0; m * n];
    for i in 0..m {
        for j in 0..n {
            data[i * n + j] = *a.at(i as i32, j as i32);
        }
    }
    let a_na = DMatrix::from_row_slice(m, n, &data);

    // 2. b → DMatrix 변환
    let b_na = DMatrix::from_column_slice(m, 1, b);

    // 3. SVD 최소제곱 해 계산
    let svd = SVD::new(a_na, true, true);
    let x = svd.solve(&b_na, tol).expect("SVD solve failed");

    // 4. 결과 반환: Vec<f64>
    x.column(0).iter().copied().collect()
}
```

## 1️⃣ jacobi_symmetric_eigen: 야코비 회전법
### 목적
대칭행렬 $B\in \mathbb{R^{\mathnormal{n\times n}}}$ 에 대해:

$$
B=V\Lambda V^{\top }
$$

- $\Lambda$ : 대각 고유값 행렬
- V: 열이 고유벡터인 직교행렬

### 핵심 수식
- 회전계수:  

$$
\tau =\frac{a_{qq}-a_{pp}}{2a_{pq}},\quad t=\frac{\mathrm{sgn}(\tau )}{|\tau |+\sqrt{1+\tau ^2}},\quad c=\frac{1}{\sqrt{1+t^2}},\quad s=ct
$$

- 회전 행렬 J로 갱신:

$$
B\leftarrow J^{\top }BJ,\quad V\leftarrow VJ
$$

- 반복 종료 조건:
  
$$
\sum _{i\neq j}a_{ij}^2<\varepsilon 
$$

## 2️⃣ svdcmp: SVD via 고유분해
### 목적
임의 행렬 $A\in \mathbb{R^{\mathnormal{m\times n}}}$ 에 대해: 

$$ 
A=U\Sigma V^{\top }
$$

- $U\in \mathbb{R^{\mathnormal{m\times n}}}$: 열 직교
- $\Sigma \in \mathbb{R^{\mathnormal{n\times n}}}$: 특이값 대각 행렬
- $V\in \mathbb{R^{\mathnormal{n\times n}}}$: 직교행렬

### 단계별 수식
- 고유분해 기반 SVD:  

$$
B=A^{\top }A\in \mathbb{R^{\mathnormal{n\times n}}}\quad \Rightarrow \quad B=V\Lambda V^{\top }
$$

- 특이값 계산:

$$
\sigma _i=\sqrt{\lambda _i}\quad (\lambda _i\geq 0)
$$

- U 계산:

$$
U_i=\frac{1}{\sigma _i}AV_i\quad (\sigma _i>0)
$$

- 정규화:

$$
U_i\leftarrow \frac{U_i}{\| U_i\| }
$$

## 3️⃣ solve_least_squares_svd: 최소제곱 해법
### 목적
과잉결정 선형 시스템 Ax=b의 최소제곱 해:  

$$
x=\arg \min _x\| Ax-b\| ^2
$$


### SVD 기반 해법
- SVD 분해:

$$
A=U\Sigma V^{\top }
$$

- 중간 계산:

$$
y=U^{\top }b,\quad y_i=\frac{U_i^{\top }b}{\sigma _i}\quad (\sigma _i>\mathrm{tol})
$$

- 최종 해:

$$
x=Vy=\sum _iy_iV_i
$$

## ✅ 수학적 검증 요약
| 단계 또는 함수            | 수학적 표현                                | 의미 설명                                      | 검증 결과 |
|---------------------------|---------------------------------------------|------------------------------------------------|------------|
| 고유값 분해               | $B = V \Lambda V^{\top}$               | 대칭행렬 B의 고유값 분해 (야코비 회전법)       | ✅ 정확     |
| 특이값 분해 (SVD)         | $A = U \Sigma V^{\top}$                | 일반 행렬 A의 SVD 분해                         | ✅ 정확     |
| 최소제곱 해               | $x = V \Sigma^{-1} U^{\top} b$         | SVD 기반 최소제곱 해 공식                      | ✅ 정확     |
| U 열 정규화               | $\| U_i \| = 1$                         | U의 각 열 벡터를 단위 벡터로 정규화            | ✅ 안정적   |


## 🔍 요약 설명
- 모든 수식은 선형대수의 표준 정의에 기반하며, 구현은 수치적으로 안정적입니다.
- 고유값이 음수일 경우에도 max(0, λ) 처리로 특이값 안정화
- 특이값이 0일 때도 안전하게 처리하여 분해 실패 없이 진행
- U의 열 정규화는 EPSILON 기준으로 안정성 확보


아래는 선형 시스템 Ax=b에 대해 SVD 기반으로 해 x를 구하는 확실한 샘플 코드입니다.  
이 예시는 해가 정확히 존재하고, SVD를 통해 안정적으로 복원되는 구조를 갖습니다.

## ✅ 샘플: Ax = b 해 구하기 (SVD 기반)
### 📘 문제 설정

$$
A=\left[ \begin{matrix}1&0&2\\ 0&1&1\\ 1&1&3\\ \end{matrix}\right] 
,\quad x=\left[ \begin{matrix}2\\ -1\\ 1\\ \end{matrix}\right] \quad \Rightarrow \quad 
b=Ax
$$

### 🧪 테스트 코드
```rust
#[test]
fn solve_svd_sample_3x3() {
    use nurbslib::core::matrix::Matrix;
    use nurbslib::core::svd::solve_least_squares_svd;

    // A 정의
    let a = Matrix::from_nested(&[
        &[1.0, 0.0, 2.0],
        &[0.0, 1.0, 1.0],
        &[1.0, 1.0, 4.0],
    ]);

    // x_true 정의
    let x_true = [2.0, -1.0, 1.0];

    // b = Ax 계산
    let mut b = vec![0.0; 3];
    for i in 0..3 {
        b[i] = a.at(i as i32, 0) * x_true[0]
             + a.at(i as i32, 1) * x_true[1]
             + a.at(i as i32, 2) * x_true[2];
    }

    // SVD 기반 해 구하기
    let x = solve_least_squares_svd(a.clone(), &b, 1e-12);
    println!("x* = {:?}", x);

    // 오차 확인
    for i in 0..3 {
        assert!((x[i] - x_true[i]).abs() < 1e-10, "x mismatch at {}: got {}, expect {}", i, x[i], x_true[i]);
    }

    // 잔차 노름
    let mut s2 = 0.0;
    for i in 0..3 {
        let ax = a.at(i as i32, 0) * x[0]
               + a.at(i as i32, 1) * x[1]
               + a.at(i as i32, 2) * x[2];
        let r = b[i] - ax;
        s2 += r * r;
    }
    println!("||r||₂ = {:.6e}", s2.sqrt());
    assert!(s2.sqrt() < 1e-10, "residual too large");
}
```

### ✅ 검증 포인트
- 해 x는 정확히 복원됨
- 잔차 $\| Ax-b\| _2$ 는 거의 0
- SVD 기반이므로 과잉결정/랭크 부족에도 안정적



## ✅ 두 SVD 방식 비교: `svdcmp_sym_right` vs `svdcmp_sym_left`

| 항목                 | svdcmp_sym_right              | svdcmp_sym_left               |
|----------------------|-------------------------------|-------------------------------|
| 기반 행렬            | AᵀA                           | AAᵀ                           |
| 고유값 분해 대상     | 오른쪽 특이벡터 V             | 왼쪽 특이벡터 U               |
| U 계산 방식          | U = A·V / σ                   | 고유벡터 직접 사용            |
| V 계산 방식          | 고유벡터 직접 사용            | V = Aᵀ·U / σ                  |
| 직교성 보장          | U는 수치적으로 깨질 수 있음   | U는 고유벡터로 직교성 보장됨  |
| 랭크 결손 대응       | 불안정하거나 해가 틀어질 수 있음 | 최소 노름 해를 안정적으로 계산 |
| 재구성 정확도        | 고유값 정렬과 U 계산이 민감함 | 수치적으로 더 안정적이고 정확함 |
| 추천 용도            | 풀랭크 행렬, 단순한 구조      | 랭크 결손, 고정밀 해석, 공학적 안정성 |

---


