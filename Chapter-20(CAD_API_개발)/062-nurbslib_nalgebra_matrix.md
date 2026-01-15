# nurbslib + nalgebra 이용한 수치 해석
nalgebra와 함께 nurbslib이라는 커스텀 수치 라이브러리를 활용한 SVD 기반 선형 시스템 해법과 행렬 분해 테스트들이 포함.  
각 테스트 함수와 보조 함수들을 하나씩 설명.

## 공통 코드
```rust
use nalgebra::{DMatrix, SVD};
use nurbslib::core::matrix::Matrix;
use nurbslib::core::svd::{on_solve_least_squares_svd_na, on_svdcmp};
use nurbslib::core::tarray::TArray;
```
```rust
fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}
```
```rust
fn diff_fro_norm(a: &Matrix, b: &Matrix) -> f64 {
    assert_eq!(a.row_count(), b.row_count());
    assert_eq!(a.col_count(), b.col_count());
    let (r, c) = (a.row_count(), a.col_count());
    let mut s = 0.0;
    for i in 0..r {
        for j in 0..c {
            let v = *a.at(i as i32, j as i32) - *b.at(i as i32, j as i32);
            s += v * v;
        }
    }
    s.sqrt()
}
```
```rust
#[allow(unused)]
// Σ (n×n, n = w.len())
fn make_sigma_square(w: &[f64]) -> Matrix {
    let n = w.len();
    let mut s = Matrix::with_dims(n, n);
    s.zero();
    for i in 0..n {
        *s.at_mut(i as i32, i as i32) = w[i];
    }
    s
}
```
```rust
fn mat_t_mat(m: &Matrix, n: &Matrix) -> Matrix {
    let mut mt = m.clone();
    mt.transpose();
    &mt * n
}
```
```rust
fn has_orthonormal_cols(u: &Matrix, tol: f64) -> bool {
    // U: m×n → UᵀU: n×n
    let utu = mat_t_mat(u, u);
    let n = u.col_count();
    for i in 0..n {
        for j in 0..n {
            let want = if i == j { 1.0 } else { 0.0 };
            if !approx_eq(*utu.at(i as i32, j as i32), want, tol) {
                return false;
            }
        }
    }
    true
}
```
```rust
fn is_orthonormal(v: &Matrix, tol: f64) -> bool {
    let vtv = mat_t_mat(v, v);
    let n = v.row_count();
    for i in 0..n {
        for j in 0..n {
            let want = if i == j { 1.0 } else { 0.0 };
            if !approx_eq(*vtv.at(i as i32, j as i32), want, tol) {
                return false;
            }
        }
    }
    true
}
```
```rust
// Â = U Σ Vᵀ  (U: m×n, Σ: n×n, V: n×n)  — NR 스타일(m≥n)
fn sorted_desc(mut xs: Vec<f64>) -> Vec<f64> {
    xs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    xs
}
```
```rust
fn assert_all_nonneg(ws: &[f64], tol: f64) {
    for &x in ws {
        assert!(x >= -tol, "singular value is negative: {}", x);
    }
}
```
```rust
fn reconstruct(u: &Matrix, w: &[f64], v: &Matrix) -> Matrix {
    let n = w.len();
    let mut s = Matrix::with_dims(n, n);
    s.zero();
    for i in 0..n {
        *s.at_mut(i as i32, i as i32) = w[i];
    }
    let mut vt = v.clone();
    vt.transpose();
    &(*&u * &s) * &vt
}
```
```rust

fn fro(a: &Matrix) -> f64 {
    let (r, c) = (a.row_count(), a.col_count());
    let mut s = 0.0;
    for i in 0..r {
        for j in 0..c {
            let x = *a.at(i as i32, j as i32);
            s += x * x;
        }
    }
    s.sqrt()
}
```
```rust
fn fro_diff(a: &Matrix, b: &Matrix) -> f64 {
    assert_eq!(a.row_count(), b.row_count());
    assert_eq!(a.col_count(), b.col_count());
    let (r, c) = (a.row_count(), a.col_count());
    let mut s = 0.0;
    for i in 0..r {
        for j in 0..c {
            let x = *a.at(i as i32, j as i32) - *b.at(i as i32, j as i32);
            s += x * x;
        }
    }
    s.sqrt()
}
```
## 🔢 1. svd_cmp_test
- 기능: nalgebra::SVD를 사용해 최소제곱 해를 구하고, 참값과 비교
- 용도: SVD 기반 해법의 정확도와 안정성 검증
- 목적:
    - Ax = b에서 x를 SVD로 구함
    - 참값 x_true와 비교하여 잔차(norm)와 노름(norm)을 확인
    - $A ≈ UΣVᵀ$ 재구성 오차도 확인

### 코드
```rust
#[test]
fn svd_cmp_test() {
    // 테스트 행렬 A (3x3)
    let a = DMatrix::from_row_slice(
        3,
        3,
        &[
            1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0,
            3.0, // 풀랭크: 세 번째 행 수정
        ],
    );

    // 참값 x_true
    let x_true = DMatrix::from_column_slice(3, 1, &[1.0, -1.0, 1.0]);

    // b = A x_true
    let b = &a * &x_true;

    // SVD로 최소제곱 해 구하기
    let svd = SVD::new(a.clone(), true, true);
    let x_star = svd.solve(&b, 1e-6).expect("SVD solve failed");

    // 잔차 계산: ||Ax - b||
    let residual = (&a * &x_star - &b).norm();
    println!("Residual ||Ax - b|| = {:.6e}", residual);

    // 최소 노름 확인
    let norm_true = x_true.norm();
    let norm_star = x_star.norm();
    println!("‖x_true‖ = {:.6e}, ‖x_star‖ = {:.6e}", norm_true, norm_star);

    // 재구성 확인: A ≈ UΣVᵀ
    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();
    let sigma = DMatrix::from_diagonal(&svd.singular_values);
    let a_reconstructed = &u * sigma * v_t;
    let fro_error = (&a - &a_reconstructed).norm();
    println!("Reconstruction error (Frobenius) = {:.6e}", fro_error);
}
```
### 출력
```
Residual ||Ax - b|| = 1.275549e-15
‖x_true‖ = 1.732051e0, ‖x_star‖ = 1.732051e0
Reconstruction error (Frobenius) = 2.328823e-15
```

## 🔢 2. dbg_rank1_rect_3x2
- 기능: 랭크 1 행렬의 SVD 수행 및 검증
- 용도: 특이값 분해의 정확성 테스트
- 목적:
    - $A=uv^T$  형태의 랭크 1 행렬 생성
    - SVD 수행 후 특이값이 [9, 0]인지 확인
    - 재구성 오차(Frobenius norm) 검증
### 코드
```rust
#[test]
fn dbg_rank1_rect_3x2() {
    // A = u vᵀ (랭크 1) → σ = [9, 0]
    let u = DMatrix::from_column_slice(3, 1, &[1.0, 2.0, 2.0]); // 3×1
    let v = DMatrix::from_row_slice(1, 2, &[0.0, 3.0]); // 1×2
    let a = &u * &v; // 3×2

    let a0 = a.clone();

    // SVD 수행
    let svd = SVD::new(a.clone(), true, true);
    let sigma: Vec<f64> = svd.singular_values.data.as_vec().clone();
    println!("\n[rank1 3x2] ok=true, w={:?}", sigma);

    // 특이값 정렬
    let mut ws = sigma.clone();
    ws.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("sorted σ = {:?}", ws);

    // 재구성: A ≈ UΣVᵀ
    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();
    let s_mat = DMatrix::from_diagonal(&svd.singular_values);
    let a_rec = &u * s_mat * v_t;

    // Frobenius norm 차이
    let err = (&a0 - &a_rec).norm();
    println!("reconstruct error (fro) = {:.6e}", err);
    println!("‖A‖_F = {:.6},  ‖UΣVᵀ‖_F = {:.6}", a0.norm(), a_rec.norm());

    // 검증
    assert!(
        (ws[0] - 9.0).abs() < 1e-8 && ws[1].abs() < 1e-10,
        "σ = {:?} (expected [9, 0])",
        ws
    );
    assert!(err < 1e-8, "reconstruction error too large");
}
```
### 출력
```
[rank1 3x2] ok=true, w=[9.000000000000002, 0.0]
sorted σ = [9.000000000000002, 0.0]
reconstruct error (fro) = 1.776357e-15
‖A‖_F = 9.000000,  ‖UΣVᵀ‖_F = 9.000000
```

## 🔢 3. test_basic_svd
- 기능: 기본 SVD 수행 및 재구성 확인
- 용도: nalgebra::SVD의 기본 동작 검증
- 목적:
    - $A=U\Sigma V^T$ 로 분해 후 다시 곱해 원래 행렬과 비교
    - 재구성 오차 출력
### 코드
```rust
#[test]
fn test_basic_svd() {
    let a = DMatrix::from_row_slice(3, 3, &[1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0, 4.0]);

    let svd = SVD::new(a.clone(), true, true);
    let u = svd.u.unwrap();
    let s = DMatrix::from_diagonal(&svd.singular_values);
    let v_t = svd.v_t.unwrap();
    let a_rec = &u * s * v_t;

    println!("Original A:\n{}", a);
    println!("Reconstructed A:\n{}", a_rec);
    println!("Reconstruction error: {:.6e}", (&a - &a_rec).norm());
}
```
## 출력
```
Original A:

  ┌       ┐
  │ 1 0 2 │
  │ 0 1 1 │
  │ 1 1 4 │
  └       ┘


Reconstructed A:

  ┌                                                                                                          ┐
  │                                  1 0.00000000000000003469446951953614                  2.000000000000001 │
  │ 0.00000000000000016653345369377348                 0.9999999999999993                 0.9999999999999997 │
  │                 1.0000000000000007                 0.9999999999999989                                  4 │
  └                                                                                                          ┘


Reconstruction error: 1.746080e-15
```
## 🔢 4. test_least_squares_exact
- 기능: 최소제곱 해법의 정확성 검증
- 용도: SVD 기반 해법이 참값과 일치하는지 확인
- 목적:
- x_true와 x_star 비교
- 잔차(norm) 계산
### 코드
```rust
#[test]
fn test_least_squares_exact() {
    let a = DMatrix::from_row_slice(3, 3, &[1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0, 4.0]);
    let x_true = DMatrix::from_column_slice(3, 1, &[1.0, -1.0, 1.0]);
    let b = &a * &x_true;

    let svd = SVD::new(a.clone(), true, true);
    let x_star = svd.solve(&b, 1e-6).unwrap();

    println!("x_true:\n{}", x_true);
    println!("x_star:\n{}", x_star);
    println!("Residual: {:.6e}", (&a * &x_star - &b).norm());
}
```
### 출력
```
x_true:

  ┌    ┐
  │  1 │
  │ -1 │
  │  1 │
  └    ┘

x_star:

  ┌                     ┐
  │  0.9999999999999994 │
  │ -0.9999999999999991 │
  │  0.9999999999999993 │
  └                     ┘

Residual: 2.852215e-15

```

## 🔢 5. test_rank_deficient
- 기능: 랭크가 부족한 행렬에 대한 최소제곱 해법
- 용도: 특이 행렬에 대한 안정적 해법 검증
- 목적:
    - x_star는 최소 노름 해
    - x_true와 비교하여 노름 차이 확인
### 코드
```rust
#[test]
fn test_rank_deficient() {
    let a = DMatrix::from_row_slice(
        3,
        3,
        &[
            1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0,
            3.0, // 세 번째 행 = 첫 두 행의 합
        ],
    );
    let x_true = DMatrix::from_column_slice(3, 1, &[1.0, -1.0, 1.0]);
    let b = &a * &x_true;

    let svd = SVD::new(a.clone(), true, true);
    let x_star = svd.solve(&b, 1e-6).unwrap();

    println!("x_true:\n{}", x_true);
    println!("x_star (minimum norm):\n{}", x_star);
    println!("Residual: {:.6e}", (&a * &x_star - &b).norm());
    println!(
        "‖x_true‖ = {:.6e}, ‖x_star‖ = {:.6e}",
        x_true.norm(),
        x_star.norm()
    );
}
```
### 출력
```
x_true:

  ┌    ┐
  │  1 │
  │ -1 │
  │  1 │
  └    ┘


x_star (minimum norm):

  ┌                     ┐
  │  1.0000000000000002 │
  │ -1.0000000000000007 │
  │  1.0000000000000004 │
  └                     ┘


Residual: 1.275549e-15
‖x_true‖ = 1.732051e0, ‖x_star‖ = 1.732051e0
```

## 🔢 6. test_rectangular_rank1
- 기능: 직사각형 랭크 1 행렬의 SVD
- 용도: 특이값 분해의 정확성 확인
- 목적:
`- 특이값 정렬 및 재구성 오차 확인`
### 코드
```rust
#[test]
fn test_rectangular_rank1() {
    let u = DMatrix::from_column_slice(3, 1, &[1.0, 2.0, 2.0]);
    let v = DMatrix::from_row_slice(1, 2, &[0.0, 3.0]);
    let a = &u * &v;

    let svd = SVD::new(a.clone(), true, true);
    let s: Vec<f64> = svd.singular_values.data.as_vec().clone();
    let mut sorted = s.clone();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();
    let s_mat = DMatrix::from_diagonal(&svd.singular_values);
    let a_rec = &u * s_mat * v_t;

    println!("σ = {:?}", sorted);
    println!("Reconstruction error: {:.6e}", (&a - &a_rec).norm());
}
```
### 출력
```
σ = [9.000000000000002, 0.0]
Reconstruction error: 1.776357e-15

```

## 🔢 7. test_overdetermined
- 기능: 과결정 시스템의 최소제곱 해법
- 용도: SVD로 안정적인 해 구하기
- 목적:
    - Ax ≈ b에서 잔차 계산
    - 과결정 시스템에서도 정확한 해를 구할 수 있는지 확인
### 코드
```rust
 #[test]
fn test_overdetermined() {
    let a = DMatrix::from_row_slice(
        4,
        3,
        &[1.0, 0.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0, 4.0, 2.0, 1.0, 5.0],
    );
    let x_true = DMatrix::from_column_slice(3, 1, &[1.0, -1.0, 1.0]);
    let b = &a * &x_true;

    let svd = SVD::new(a.clone(), true, true);
    let x_star = svd.solve(&b, 1e-6).unwrap();
    
    println!("x_star:\n{}", x_star);
    println!("Residual: {:.6e}", (&a * &x_star - &b).norm());
}
```
### 출력
```
x_star:

  ┌                     ┐
  │  1.0000000000000002 │
  │ -1.0000000000000004 │
  │  0.9999999999999997 │
  └                     ┘
Residual: 2.666847e-15
```

## 🔢 8. svd_identity_3x3
- 기능: 단위 행렬의 SVD 수행
- 용도: 커스텀 SVD 구현(on_svdcmp)의 정확성 검증
- 목적:
    - 특이값이 모두 1인지 확인
    - UᵀU ≈ I, VᵀV ≈ I 검증
    - 재구성 오차 확인

### 코드
```rust
#[test]
fn svd_identity_3x3() {
    let mut a = Matrix::with_dims(3, 3);
    a.set_diagonal_scalar(1.0);
    let a0 = a.clone();

    let mut w = TArray::<f64>::new();
    let mut v = Matrix::new();
    assert!(on_svdcmp(&mut a, &mut w, &mut v));

    assert!(has_orthonormal_cols(&a, 1e-12), "UᵀU ≉ I");
    assert!(is_orthonormal(&v, 1e-12), "VᵀV ≉ I");

    let got = sorted_desc(w.data.clone());
    let expect = vec![1.0, 1.0, 1.0];
    for (g, e) in got.iter().zip(expect.iter()) {
        assert!(approx_eq(*g, *e, 1e-12), "σ mismatch: {g} vs {e}");
    }

    let a_rec = reconstruct(&a, &w.data, &v);
    println!("a:\n{}", a);
    println!("w.data:\n{:?}", w.data);
    println!("v:\n{}", v);
    let err = diff_fro_norm(&a0, &a_rec);
    assert!(err <= 1e-12, "reconstruction error = {}", err);
}
```
### 출력
```
a:
[[1, 0, 0]
[0, 1, 0]
[0, 0, 1]]
w.data:
[1.0, 1.0, 1.0]
v:
[[1, 0, 0]
[0, 1, 0]
[0, 0, 1]]
```

## 🔢 9. svd_diagonal_rect_3x2
- 기능: 직사각형 대각 행렬의 SVD
- 용도: 커스텀 SVD의 정확성 테스트
- 목적:
    - 특이값 [3, 2] 확인
    - 직교성 및 재구성 오차 검증

### 코드
```rust
#[test]
fn svd_diagonal_rect_3x2() {
    // A = diag(3,2) in 3x2 (m≥n)
    let mut a = Matrix::with_dims(3, 2);
    a.zero();
    *a.at_mut(0, 0) = 3.0;
    *a.at_mut(1, 1) = 2.0;
    let a0 = a.clone();

    let mut w = TArray::<f64>::new();
    let mut v = Matrix::new();
    assert!(on_svdcmp(&mut a, &mut w, &mut v));

    assert_all_nonneg(&w.data, 1e-12);
    let got = sorted_desc(w.data.clone());
    let expect = vec![3.0, 2.0];
    for (g, e) in got.iter().zip(expect.iter()) {
        assert!(approx_eq(*g, *e, 1e-10), "σ mismatch: {g} vs {e}");
    }

    assert!(has_orthonormal_cols(&a, 1e-12));
    assert!(is_orthonormal(&v, 1e-12));

    let a_rec = reconstruct(&a, &w.data, &v);
    println!("a:\n{}", a);
    println!("w.data:\n{:?}", w.data);
    println!("v:\n{}", v);    
    let err = diff_fro_norm(&a0, &a_rec);
    assert!(err <= 1e-12, "reconstruction error = {}", err);
}
```
### 출력
```
a:
[[1, 0]
[0, 1]
[0, 0]]
w.data:
[3.0, 2.0]
v:
[[1, 0]
[0, 1]]
```

## 🔢 10. dbg_rank1_rect_3x2_case1
- 기능: 랭크 1 행렬의 수동 생성 및 SVD 검증
- 용도: 커스텀 SVD의 특이값 정렬 및 재구성 정확도 확인
- 목적:
    - 특이값 [9, 0] 확인
    - Frobenius norm 오차 계산

### 코드
```rust
#[test]
fn dbg_rank1_rect_3x2_case1() {
    // A = u vᵀ (랭크 1) → σ = [9, 0]
    let u = [1.0, 2.0, 2.0];
    let v2 = [0.0, 3.0];
    let mut a = Matrix::with_dims(3, 2);
    for i in 0..3 {
        for j in 0..2 {
            *a.at_mut(i as i32, j as i32) = u[i] * v2[j];
        }
    }
    let a0 = a.clone();

    let mut w = TArray::<f64>::new();
    let mut v = Matrix::new();
    let ok = on_svdcmp(&mut a, &mut w, &mut v);
    println!("\n[rank1 3x2] ok={ok}, w={:?}", w.data);

    assert!(ok, "svdcmp failed");

    let mut ws = w.data.clone();
    ws.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("sorted σ = {:?}", ws);

    let a_rec = reconstruct(&a, &w.data, &v);
    let err = fro_diff(&a0, &a_rec);
    println!("reconstruct error (fro) = {:.6e}", err);
    println!("‖A‖_F = {:.6},  ‖UΣVᵀ‖_F = {:.6}", fro(&a0), fro(&a_rec));

    assert!(
        (ws[0] - 9.0).abs() < 1e-8 && ws[1].abs() < 1e-7,
        "σ = {:?} (expected [9,0])",
        ws
    );
    assert!(err < 1e-8, "reconstruction error too large");
}
```
### 출력
```
[rank1 3x2] ok=true, w=[9.000000000000002, 0.0]
sorted σ = [9.000000000000002, 0.0]
reconstruct error (fro) = 1.776357e-15
‖A‖_F = 9.000000,  ‖UΣVᵀ‖_F = 9.000000

```

## 🔢 11. dbg_constructed_answer_4x3
- 기능: 인위적으로 구성한 SVD 행렬에 대한 검증
- 용도: 커스텀 SVD의 특이값 정렬 및 재구성 정확도 확인
- 목적:
    - Σ = diag(7, 3, 1) 구성
    - 재구성 오차와 특이값 일치 여부 확인
### 코드
```rust
#[test]
fn dbg_constructed_answer_4x3() {
    // Σ = diag(7,3,1) 를 인위적으로 구성한 4×3 케이스
    let mut u0 = Matrix::with_dims(4, 3);
    u0.zero();
    *u0.at_mut(0, 0) = 1.0;
    *u0.at_mut(1, 1) = 1.0;
    *u0.at_mut(2, 2) = 1.0;

    let sigma = [7.0, 3.0, 1.0];
    let mut s = Matrix::with_dims(3, 3);
    s.zero();
    for i in 0..3 {
        *s.at_mut(i as i32, i as i32) = sigma[i];
    }

    let (c, s_) = (
        (std::f64::consts::PI / 7.0).cos(),
        (std::f64::consts::PI / 7.0).sin(),
    );
    let mut v0 = Matrix::with_dims(3, 3);
    *v0.at_mut(0, 0) = c;
    *v0.at_mut(0, 1) = -s_;
    *v0.at_mut(0, 2) = 0.0;
    *v0.at_mut(1, 0) = s_;
    *v0.at_mut(1, 1) = c;
    *v0.at_mut(1, 2) = 0.0;
    *v0.at_mut(2, 0) = 0.0;
    *v0.at_mut(2, 1) = 0.0;
    *v0.at_mut(2, 2) = 1.0;

    let mut v0t = v0.clone();
    v0t.transpose();
    let a0 = &(&u0 * &s) * &v0t;

    let mut a = a0.clone();
    let mut w = TArray::<f64>::new();
    let mut v = Matrix::new();
    let ok = on_svdcmp(&mut a, &mut w, &mut v);
    println!("\n[constructed 4x3] ok={ok}, w={:?}", w.data);

    assert!(ok, "svdcmp failed");

    let mut ws = w.data.clone();
    ws.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("sorted σ = {:?}", ws);

    let a_rec = reconstruct(&a, &w.data, &v);
    let err = fro_diff(&a0, &a_rec);
    println!("reconstruct error (fro) = {:.6e}", err);
    println!("‖A‖_F = {:.6},  ‖UΣVᵀ‖_F = {:.6}", fro(&a0), fro(&a_rec));

    let mut ex = sigma.to_vec();
    ex.sort_by(|a, b| b.partial_cmp(a).unwrap());
    for (g, e) in ws.iter().zip(ex.iter()) {
        assert!((g - e).abs() < 1e-8, "σ mismatch: got {}, expect {}", g, e);
    }
    assert!(err < 1e-8, "reconstruction error too large");
}
```
### 출력
```
[constructed 4x3] ok=true, w=[6.999999999999999, 3.0, 0.9999999999999999]
sorted σ = [6.999999999999999, 3.0, 0.9999999999999999]
reconstruct error (fro) = 1.847779e-15
‖A‖_F = 7.681146,  ‖UΣVᵀ‖_F = 7.681146
```

## 🔢 12. svd_constructed_answer_4x3
- 기능: 위와 동일한 구성의 SVD 검증 (중복 테스트)
- 용도: 특이값 정렬 및 오차 확인
- 목적:
    - σ 일치 여부 및 재구성 오차 검증

### 코드
```rust
#[test]
fn svd_constructed_answer_4x3() {
    // Σ = diag(7,3,1) 를 인위적으로 구성한 4×3
    let mut u0 = Matrix::with_dims(4, 3);
    u0.zero();
    *u0.at_mut(0, 0) = 1.0;
    *u0.at_mut(1, 1) = 1.0;
    *u0.at_mut(2, 2) = 1.0; // 직교 열 3개(간단)

    let sigma = [7.0, 3.0, 1.0];
    let mut s = Matrix::with_dims(3, 3);
    s.zero();
    for i in 0..3 {
        *s.at_mut(i as i32, i as i32) = sigma[i];
    }

    let (c, s_) = (
        (std::f64::consts::PI / 7.0).cos(),
        (std::f64::consts::PI / 7.0).sin(),
    );
    let mut v0 = Matrix::with_dims(3, 3);
    *v0.at_mut(0, 0) = c;
    *v0.at_mut(0, 1) = -s_;
    *v0.at_mut(0, 2) = 0.0;
    *v0.at_mut(1, 0) = s_;
    *v0.at_mut(1, 1) = c;
    *v0.at_mut(1, 2) = 0.0;
    *v0.at_mut(2, 0) = 0.0;
    *v0.at_mut(2, 1) = 0.0;
    *v0.at_mut(2, 2) = 1.0;

    let mut v0t = v0.clone();
    v0t.transpose();
    let a0 = &(&u0 * &s) * &v0t;

    let mut a = a0.clone();
    let mut w = TArray::<f64>::new();
    let mut v = Matrix::new();
    let ok = on_svdcmp(&mut a, &mut w, &mut v);
    assert!(ok, "svdcmp failed");

    let mut ws = w.data.clone();
    ws.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("constructed σ = {:?}", ws);
    let mut ex = sigma.to_vec();
    ex.sort_by(|a, b| b.partial_cmp(a).unwrap());
    for (g, e) in ws.iter().zip(ex.iter()) {
        assert!((g - e).abs() < 1e-10, "σ mismatch: got {}, expect {}", g, e);
    }

    let a_rec = reconstruct(&a, &w.data, &v);
    let err = fro_diff(&a0, &a_rec);
    println!("recon err = {:.3e}", err);
    assert!(err < 1e-10, "reconstruction error too large");
}
```
### 출력
```
constructed σ = [6.999999999999999, 3.0, 0.9999999999999999]
recon err = 1.848e-15
```

## 🔢 13. solve_svdcmd
- 기능: 커스텀 SVD 기반 최소제곱 해법
- 용도: on_solve_least_squares_svd_na 함수 테스트
- 목적:
    - 노이즈가 포함된 b에 대해 해 x 구함
    - 잔차(norm) 계산

### 코드
```rust
#[test]
fn solve_svdcmd() {
    let a = Matrix::from_nested(&[&[1.0, 1.0], &[1.0, 2.0], &[1.0, 3.0], &[1.0, 4.0]]);
    let x_true = [2.0, -1.0];
    let mut b = vec![
        1.0 * x_true[0] + 1.0 * x_true[1],
        1.0 * x_true[0] + 2.0 * x_true[1],
        1.0 * x_true[0] + 3.0 * x_true[1],
        1.0 * x_true[0] + 4.0 * x_true[1],
    ];
    b[2] += 0.05; // 노이즈

    let x = on_solve_least_squares_svd_na(a.clone(), &b, 1e-12);
    println!("x* = {:?}", x);

    // 잔차 노름
    let mut s2 = 0.0;
    for i in 0..a.row_count() {
        let ax = a.at(i as i32, 0) * x[0] + a.at(i as i32, 1) * x[1];
        let r = b[i] - ax;
        s2 += r * r;
    }
    println!("||r||2 = {}", s2.sqrt());
}
```
### 출력
```
x* = [2.0000000000000018, -0.9950000000000001]
||r||2 = 0.04183300132670381

```

## 🔢 14. solve_svd_sample_3x3
- 기능: 커스텀 SVD 기반 해법의 정확도 검증
- 용도: 참값과 비교하여 오차 확인
- 목적:
    - x_true와 x 비교
    - 잔차(norm) 검증
### 코드
```rust
#[test]
fn solve_svd_sample_3x3() {
    use nurbslib::core::matrix::Matrix;

    // A 정의
    let a = Matrix::from_nested(&[
        &[1.0, 0.0, 2.0],
        &[0.0, 1.0, 1.0],
        &[1.0, 1.0, 4.0], // ← 세 번째 행 수정: [1,1,4] → 풀랭크
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
    let x = on_solve_least_squares_svd_na(a.clone(), &b, 1e-12);
    println!("x* = {:?}", x);

    // 오차 확인
    for i in 0..3 {
        assert!(
            (x[i] - x_true[i]).abs() < 1e-10,
            "x mismatch at {}: got {}, expect {}",
            i,
            x[i],
            x_true[i]
        );
    }

    // 잔차 노름
    let mut s2 = 0.0;
    for i in 0..3 {
        let ax = a.at(i as i32, 0) * x[0] + a.at(i as i32, 1) * x[1] + a.at(i as i32, 2) * x[2];
        let r = b[i] - ax;
        s2 += r * r;
    }
    println!("||r||₂ = {:.6e}", s2.sqrt());
    assert!(s2.sqrt() < 1e-10, "residual too large");
}
```
### 출력
```
x* = [1.9999999999999993, -0.9999999999999993, 0.9999999999999991]
||r||₂ = 4.446440e-15
```


## 🔢 15. test_is_symmetric
- 기능: 행렬의 대칭성 검사
- 용도: 수치 안정성 확인, 고윳값 분해 전 검사
- 목적:
    - $Aᵀ = A$  여부 확인
    - 허용 오차 기반 검사
### 코드
```rust
fn is_symmetric(m: &Matrix, tol: f64) -> bool {
    let n = m.row_count();
    if n != m.col_count() {
        return false;
    }
    for i in 0..n {
        for j in 0..n {
            let a = *m.at(i as i32, j as i32);
            let b = *m.at(j as i32, i as i32);
            if (a - b).abs() > tol {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_is_symmetric() {
    let mut m = Matrix::with_dims(3, 3);
    m.zero();
    *m.at_mut(0, 1) = 2.0;
    *m.at_mut(1, 0) = 2.0;
    *m.at_mut(2, 2) = 5.0;
    assert!(is_symmetric(&m, 1e-12));

    *m.at_mut(0, 2) = 1.0;
    *m.at_mut(2, 0) = 1.001;
    assert!(!is_symmetric(&m, 1e-4)); // 허용 오차 초과
}
```


## 🔢 16. test_is_diagonal
- 기능: 행렬의 대각성 검사
- 용도: SVD 결과의 Σ 검증
- 목적:
    - 비대각 요소가 허용 오차 이내인지 확인

### 코드
```rust
fn is_diagonal(m: &Matrix, tol: f64) -> bool {
    let (r, c) = (m.row_count(), m.col_count());
    for i in 0..r {
        for j in 0..c {
            if i != j && m.at(i as i32, j as i32).abs() > tol {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_is_diagonal() {
    let mut m = Matrix::with_dims(3, 3);
    m.zero();
    *m.at_mut(0, 0) = 1.0;
    *m.at_mut(1, 1) = 2.0;
    *m.at_mut(2, 2) = 3.0;
    assert!(is_diagonal(&m, 1e-12));

    *m.at_mut(0, 1) = 0.001;
    assert!(!is_diagonal(&m, 1e-4));
}
```


## 🔢 17. test_check_svd_reconstruction
- 기능: SVD 재구성 정확도 확인
- 용도: UΣVᵀ ≈ A 여부 검증
- 목적:
    - Frobenius norm 기반 오차 계산

### 코드
```rust
fn check_svd_reconstruction(
    a0: &Matrix,
    u: &Matrix,
    sigma: &[f64],
    v: &Matrix,
    tol: f64,
) -> bool {
    let mut s = Matrix::with_dims(sigma.len(), sigma.len());
    s.zero();
    for i in 0..sigma.len() {
        *s.at_mut(i as i32, i as i32) = sigma[i];
    }
    let mut vt = v.clone();
    vt.transpose();
    let a_rec = &(u * &s) * &vt;
    fro_diff(a0, &a_rec) < tol
}

#[test]
fn test_check_svd_reconstruction() {
    let mut a = Matrix::with_dims(3, 2);
    *a.at_mut(0, 0) = 3.0;
    *a.at_mut(1, 1) = 4.0;
    let a0 = a.clone();

    let mut w = TArray::<f64>::new();
    let mut v = Matrix::new();
    assert!(on_svdcmp(&mut a, &mut w, &mut v));
    println!("a:\n{}", a);
    println!("w.data:\n{:?}", w.data);
    println!("v:\n{}", v);    
    assert!(check_svd_reconstruction(&a0, &a, &w.data, &v, 1e-10));
}
```
### 출력
```
a:
[[0, 1]
[1, 0]
[0, 0]]
w.data:
[4.0, 3.0]
v:
[[0, 1]
[1, 0]]
```

---

# reconstruct
reconstruct 함수는 SVD 분해 결과를 이용해 원래 행렬을 다시 조립하는 역할을 합니다.
즉, 우리가 SVD로 얻은 $U$, $\Sigma$ , $V^T$ 를 가지고 원래 행렬 A를 근사하거나 복원하는 거예요.

## 🔧 reconstruct 함수의 핵심 역할
```rust
fn reconstruct(u: &Matrix, w: &[f64], v: &Matrix) -> Matrix {
    // w: 특이값 배열 → 대각 행렬 Σ로 변환
    // u: 좌측 특이벡터 행렬
    // v: 우측 특이벡터 행렬
    // 반환값: A ≈ U Σ Vᵀ
}
```

- 입력:
- u: 좌측 특이벡터 행렬 (m×n)
- w: 특이값 배열 → 대각 행렬 Σ로 변환 (n×n)
- v: 우측 특이벡터 행렬 (n×n)
- 출력:
- $A_{\mathrm{reconstructed}}=U\cdot \Sigma \cdot V^T$

### 🎯 `reconstruct` 함수의 목적과 용도

| 목적/용도              | 설명                                                                 | 활용 예시                                      |
|------------------------|----------------------------------------------------------------------|------------------------------------------------|
| SVD 정확도 검증        | SVD로 분해한 $A = UΣVᵀ$ 를 다시 곱해 원래 행렬과 얼마나 유사한지 확인     | 수치 해법 테스트, 알고리즘 검증                |
| 재구성 오차 측정       | 원래 행렬과 재구성 행렬의 차이를 Frobenius norm 등으로 측정             | `diff_fro_norm`, `fro_diff`와 함께 사용        |
| 저차 근사 생성         | 상위 특이값만 사용해 정보 손실을 최소화한 근사 행렬 생성                | 데이터 압축, 노이즈 제거                       |
| 시각적/수치적 비교     | 원본 행렬과 재구성 행렬을 비교하여 직관적으로 SVD의 의미를 이해         | 교육, 디버깅, 시각화                           |
| SVD 기반 알고리즘 단계 | 다른 알고리즘에서 SVD 결과를 활용해 행렬을 다시 조립하는 데 사용         | PCA, 이미지 압축, 추천 시스템 등               |


## 🧪 예시: Frobenius norm으로 오차 확인
```rust
let a_rec = reconstruct(&u, &w, &v);
let error = fro_diff(&a_original, &a_rec);
assert!(error < 1e-10); // 재구성 오차가 충분히 작아야 함
```
---




