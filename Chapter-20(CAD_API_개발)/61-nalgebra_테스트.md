## 공용 소스
```rust
mod tests {
    use approx::assert_relative_eq;
    use geometry::math::math_extra::ON_PI;
    use geometry::math::prelude::Vector3D;
    use geometry::math::prelude::matrix::Matrix;
    
    #[allow(unused_imports)]
    use nalgebra::{
        Cholesky, Const, DMatrix, DVector, Dyn, Isometry2, Isometry3, Matrix2x3, Matrix3,
        Matrix3x2, Matrix4, Matrix4x3, OMatrix, Perspective3, Point2, Point3, QR, RealField,
        Rotation3, RowDVector, SMatrix, SVD, SVector, Scalar, Schur, Similarity3, SymmetricEigen,
        Translation3, U3, U4, UnitQuaternion, Vector2, Vector3, Vector4,
    };

    fn v3(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D::new(x, y, z)
    }

    fn m_from_rows(rows: &[&[f64]]) -> Matrix {
        Matrix::from_nested(rows)
    }
}
```

## 🔢 1. dvector_test
- 주제: DVector의 기본 사용법
- 사용된 타입/메서드:
    - DVector::from_vec, from_element, dot, normalize, norm
- 설명:
    - 동적 크기의 벡터(DVector)를 생성하고, 다양한 연산을 수행합니다.
    - 스칼라 곱, 벡터 덧셈, 내적, 정규화, 노름(norm) 계산 등 기본 연산을 익힐 수 있어요.
- 활용 예:
    - 머신러닝, 물리 시뮬레이션, 최적화 문제 등에서 벡터 연산이 필요할 때

### 테스트 코드
```rust
#[test]
fn dvector_test() {
    // 생성
    let v = DVector::from_vec(vec![1.0, 2.0, 3.0]);

    // 길이
    println!("Length: {}", v.len()); //3

    // 인덱스 접근
    println!("v[1] = {}", v[1]); //2.0

    // 스칼라 곱
    let scaled = &v * 2.0;
    println!("Scaled: {}", scaled); //[2.0, 4.0, 6.0]

    // 벡터 합
    let v2 = DVector::from_element(3, 1.0); //[1.0, 1.0, 1.0]
    let sum = &v + &v2;
    println!("Sum: {}", sum);

    // 내적
    let dot = v.dot(&v2);
    println!("Dot product: {}", dot);

    // 정규화
    let normalized = v.normalize();
    println!("Normalized: {}", normalized);

    // 크기(norm)
    println!("Norm: {}", v.norm());
}
```
### 출력 결과
```
Length: 3
v[1] = 2
Scaled: 
  ┌   ┐
  │ 2 │
  │ 4 │
  │ 6 │
  └   ┘
Sum: 
  ┌   ┐
  │ 2 │
  │ 3 │
  │ 4 │
  └   ┘
Dot product: 6
Normalized: 
  ┌                    ┐
  │ 0.2672612419124244 │
  │ 0.5345224838248488 │
  │ 0.8017837257372732 │
  └                    ┘

Norm: 3.7416573867739413
```
## 🔢 2. dmatrix_test
- 주제: DMatrix의 기본 사용법
- 사용된 메서드:
    - from_row_slice, nrows, ncols, transpose, try_inverse, determinant, lu
- 설명:
    - 동적 크기의 행렬을 생성하고, 전치, 곱셈, 역행렬, 행렬식, LU 분해 등을 수행합니다.
    - try_inverse()는 정방행렬일 때만 사용 가능하며, Option을 반환합니다.
- 활용 예:
    - 선형 시스템 해석, 공학 계산, 데이터 변환 등
### 테스트 코드
```rust
fn dmatrix_test() {
    let m = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    // 크기
    println!("Rows: {}, Columns: {}", m.nrows(), m.ncols());

    // 인덱스 접근
    println!("m[(1, 2)] = {}", m[(1, 2)]);

    // 전치
    let transposed = m.transpose();
    println!("Transposed:\n{}", transposed);

    // 행렬 곱
    let m2 = DMatrix::from_element(3, 2, 1.0);
    let product = &m * &m2;
    println!("Product:\n{}", product);

    // 역행렬 (정방행렬일 경우)
    let square = DMatrix::from_row_slice(2, 2, &[4.0, 7.0, 2.0, 6.0]);
    let square_clone = square.clone();
    if let Some(inv) = square.try_inverse() {
        println!("Inverse:\n{}", inv);
    }

    // 행렬식
    println!("Determinant: {}", square_clone.determinant());

    // LU 분해
    let lu = square_clone.lu();
    println!("LU Decomposition:\nL:\n{}\nU:\n{}", lu.l(), lu.u());
}
```
### 출력 결과
```
Rows: 2, Columns: 3
m[(1, 2)] = 6
Transposed:
  ┌     ┐
  │ 1 4 │
  │ 2 5 │
  │ 3 6 │
  └     ┘

Product:
  ┌       ┐
  │  6  6 │
  │ 15 15 │
  └       ┘

Inverse:
  ┌           ┐
  │  0.6 -0.7 │
  │ -0.2  0.4 │
  └           ┘

Determinant: 10
LU Decomposition:
L:
  ┌         ┐
  │   1   0 │
  │ 0.5   1 │
  └         ┘

U:
  ┌         ┐
  │   4   7 │
  │   0 2.5 │
  └         ┘
```

## 🔢 3. lu2x2_test
- 주제: LU 분해를 통한 선형 시스템 해법
- 설명:
    - A * x = b 형태의 선형 방정식을 LU 분해로 푸는 예제입니다.
    - lu.solve(&b)를 통해 해를 구합니다.
- 활용 예:
    - 실시간 시스템에서 빠르게 선형 방정식을 풀어야 할 때

### 테스트 코드
```rust
#[test]
fn lu2x2_test() {
    // A * x = b
    let a = DMatrix::from_row_slice(2, 2, &[3.0, 2.0, 1.0, 4.0]);
    let b = DVector::from_row_slice(&[5.0, 6.0]);

    let lu = a.lu();
    if let Some(x) = lu.solve(&b) {
        println!("Solution x = {}", x);
    } else {
        println!("No solution found.");
    }
}
```
### 출력 결과
```
Solution x = 
  ┌                    ┐
  │ 0.7999999999999999 │
  │                1.3 │
  └                    ┘
```

## 🔢 4. lu3x3_test
- 주제: 3x3 시스템의 LU 해법
- 설명:
    - 위와 동일한 방식으로 3차 시스템을 풉니다.
    - tridiagonal 구조의 행렬을 사용하여 효율적인 계산 예시를 보여줍니다.
### 테스트 코드
```rust
#[test]
fn lu3x3_test() {
    let a = DMatrix::from_row_slice(3, 3, &[2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0]);
    let b = DVector::from_row_slice(&[1.0, 0.0, 1.0]);

    let lu = a.lu();
    let x = lu.solve(&b).expect("Should solve");
    println!("x = {}", x);
}
```
### 출력 결과
```
x = 
  ┌                    ┐
  │                  1 │
  │                  1 │
  │ 0.9999999999999999 │
  └                    ┘

```

## 🔢 5. singularity_test
- 주제: 특이 행렬(singular matrix) 처리
- 설명:
    - 행렬이 rank-deficient일 경우 lu.solve()는 None을 반환합니다.
    - 이 예제는 해가 존재하지 않거나 유일하지 않은 경우를 감지하는 방법을 보여줍니다.
### 테스트 코드
```rust
#[test]
fn singularity_test() {
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 2.0, 4.0]); // rank-deficient
    let b = DVector::from_row_slice(&[3.0, 6.0]);

    let lu = a.lu();
    match lu.solve(&b) {
        Some(x) => println!("x = {}", x),
        None => println!("Matrix is singular or system has no unique solution."),
    }
}
```
### 출력 결과
```
Matrix is singular or system has no unique solution.
```


## 🔢 6. solve_iterator_test
- 주제: LU 분해를 활용한 반복적 선형 시스템 해법
- 사용된 타입/메서드:
    - DMatrix::from_row_slice, lu.solve(&b)
- 설명:
    - 하나의 행렬 A에 대해 여러 개의 우변 벡터 b를 반복적으로 풀어봅니다.
    - LU 분해는 한 번만 수행하고, 여러 b에 대해 재사용합니다.
- 활용 예:
    - 동일한 시스템 구조에서 입력만 바뀌는 경우 (예: 시뮬레이션, 센서 데이터 처리)
### 테스트 코드
```rust
#[test]
fn solve_iterator_test() {
    let a = DMatrix::from_row_slice(2, 2, &[2.0, 1.0, 5.0, 7.0]);
    let lu = a.lu();

    let b_list = vec![
        DVector::from_row_slice(&[11.0, 13.0]),
        DVector::from_row_slice(&[1.0, 1.0]),
        DVector::from_row_slice(&[0.0, 0.0]),
    ];

    for (i, b) in b_list.iter().enumerate() {
        match lu.solve(b) {
            Some(x) => println!("Solution {}: {}", i + 1, x),
            None => println!("Solution {}: No solution", i + 1),
        }
    }
}
```
### 출력 결과
```
Solution 1: 
  ┌                     ┐
  │    7.11111111111111 │
  │ -3.2222222222222214 │
  └                     ┘


Solution 2: 
  ┌                      ┐
  │   0.6666666666666666 │
  │ -0.33333333333333326 │
  └                      ┘


Solution 3: 
  ┌    ┐
  │  0 │
  │ -0 │
  └    ┘
```

## 🔢 7. solve_lu_test
- 주제: LU 분해로 해 구하고 검증하기
- 설명:
    - x = A⁻¹b를 구한 뒤, 다시 A * x를 계산해서 원래의 b와 비교합니다.
    - 수치적 오차를 확인하는 방식으로 해의 정확도를 검증합니다.
- 활용 예:
    - 수치 해법의 안정성 검증, 테스트 케이스 작성
### 테스트 코드
```rust
#[test]
fn solve_lu_test() {
    let a = DMatrix::from_row_slice(2, 2, &[4.0, 3.0, 6.0, 3.0]);
    let b = DVector::from_row_slice(&[10.0, 12.0]);

    let a_clone = a.clone();
    let lu = a.lu();
    let x = lu.solve(&b).expect("Should solve");

    let b_check = &a_clone * &x;
    println!("Original b: {}", b);
    println!("Computed b: {}", b_check);
}
```
### 출력 결과
```
Original b: 
  ┌    ┐
  │ 10 │
  │ 12 │
  └    ┘


Computed b: 
  ┌    ┐
  │ 10 │
  │ 12 │
  └    ┘
```

## 🔢 8. solve_qr_least_squares
- 주제: QR 기반 최소제곱 해법
- 설명:
    - 과결정 시스템 Ax ≈ b에 대해 최소제곱 해를 구합니다.
    - x ≈ (AᵗA)⁻¹Aᵗb 방식으로 계산
- 활용 예:
    - 회귀 분석, 데이터 피팅, 노이즈가 있는 측정값 보정
### 테스트 코드
```rust
#[test]
fn solve_qr_least_squares() {
    let a = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = DVector::from_row_slice(&[7.0, 8.0, 9.0]);

    // 최소제곱 해법: x ≈ (AᵗA)⁻¹ Aᵗ b
    let ata = &a.transpose() * &a;
    let atb = &a.transpose() * &b;

    let x = ata.lu().solve(&atb).expect("Least squares failed");
    println!("Least squares solution x = {}", x);
}
```
### 출력 결과
```
ata = 
  ┌       ┐
  │ 35 44 │
  │ 44 56 │
  └       ┘


atb = 
  ┌     ┐
  │  76 │
  │ 100 │
  └     ┘


Least squares solution x = 
  ┌                    ┐
  │ -5.999999999999983 │
  │  6.499999999999987 │
  └                    ┘
```

## 🔢 9. solve_cholesky_test
- 주제: Cholesky 분해를 통한 SPD 시스템 해법
- 설명:
    - 대칭 양의 정부호 행렬 A에 대해 Ax = b를 푸는 예제
    - Cholesky::new(A)로 분해 후 solve(&b)
- 활용 예:
    - 물리 시뮬레이션, 최적화 문제, covariance 행렬 처리
### 테스트 코드
```rust
#[test]
fn solve_cholesky_test() {
    // 대칭 양의 정부호 행렬 A
    let a = DMatrix::from_row_slice(3, 3, &[4.0, 1.0, 1.0, 1.0, 3.0, 0.0, 1.0, 0.0, 2.0]);
    let b = DVector::from_row_slice(&[1.0, 2.0, 3.0]);

    // Cholesky 분해
    let chol = a.cholesky().expect("Matrix is not positive definite");
    let x = chol.solve(&b);

    println!("Solution x = {}", x);
}
```
### 출력 결과
```
Solution x = 
  ┌                      ┐
  │ -0.36842105263157887 │
  │   0.7894736842105262 │
  │   1.6842105263157894 │
  └                      ┘
```

## 🔢 10. solve_svd_test
- 주제: SVD 기반 선형 시스템 해법
- 설명:
    - A가 정방이 아니거나 특이할 경우에도 안정적으로 해를 구할 수 있음
    - svd.solve(&b, ε)로 수치적 안정성을 확보
- 활용 예:
    - 랭크가 낮거나 특이한 행렬에 대한 해법, 머신러닝에서의 역전파 계산
### 테스트 코드
```rust
#[test]
fn solve_svd_test() {
    // A * x = b 형태의 선형 시스템
    let a = DMatrix::from_row_slice(3, 2, &[1.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
    let b = DVector::from_row_slice(&[1.0, 2.0, 3.0]);

    // SVD 분해
    let svd = a.svd(true, true);
    let x = svd.solve(&b, 1e-6);

    println!("Solution x = {}", x.unwrap());
}
```
### 출력 결과
```
Solution x = 
  ┌                    ┐
  │ 0.9999999999999982 │
  │  2.000000000000001 │
  └                    ┘
```

## 🔢 11. sample_lu_solve
- 주제: LU 분해를 통한 선형 시스템 해법
- 사용된 타입/메서드:
    - Matrix, DMatrix, DVector, lu.solve(&b)
- 설명:
    - Matrix 타입을 DMatrix로 변환한 후, LU 분해를 통해 Ax = b를 풉니다.
    - assert!((A * x - b).norm() < ε)로 해의 정확도를 검증합니다.
- 활용 예:
    - 수치 해법의 정확도 테스트, 작은 시스템의 빠른 해법
### 테스트 코드
```rust
#[test]
fn sample_lu_solve() {
    let a = m_from_rows(&[&[3.0, 1.0, 2.0], &[6.0, 3.0, 4.0], &[3.0, 1.0, 5.0]]);
    let dm: DMatrix<f64> = (&a).into(); // or a.to_dmatrix()
    let b = DVector::from_row_slice(&[1.0, 2.0, 3.0]);

    let lu = dm.clone().lu(); // dm 소비하므로 clone()
    let x = lu.solve(&b).expect("solve failed");
    println!("Solved x = {}", x);    
    assert!((dm * &x - b).norm() < 1e-10);
}
```
### 출력 결과
```rust
Solved x = 
  ┌                      ┐
  │ -0.11111111111111109 │
  │                   -0 │
  │   0.6666666666666666 │
  └                      ┘
```

## 🔢 12. sample_lu_solve_multi_rhs
- 주제: 다중 우변 벡터에 대한 LU 해법
- 설명:
    - Ax = B 형태의 시스템에서 B가 행렬일 경우, 여러 해를 동시에 구합니다.
    - x_true와 비교하여 정확도 검증
- 활용 예:
    - 여러 입력에 대해 병렬적으로 해를 구해야 하는 경우 (예: 배치 처리)
### 테스트 코드
```rust
/* 2) Ax = B (다중 RHS) */
#[test]
fn sample_lu_solve_multi_rhs() {
    let a = m_from_rows(&[&[2.0, 1.0, 0.0], &[1.0, 3.0, 1.0], &[0.0, 1.0, 2.0]]);
    let dm: DMatrix<f64> = (&a).into();
    let x_true = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 0.0, 1.0, 3.0, 0.0]);
    let b = &dm * &x_true;

    let lu = dm.clone().lu();
    let x = lu.solve(&b).unwrap();
    println!("Solved x = {}", x);
    println!("x true = {}", x_true);
    assert!((&x - &x_true).norm() < 1e-10);
}
```
### 출력 결과
```
Solved x = 
  ┌                                             ┐
  │                 0.9999999999999999        2 │
  │ 0.00000000000000017763568394002506        1 │
  │                 2.9999999999999996        0 │
  └                                             ┘


x true = 
  ┌     ┐
  │ 1 2 │
  │ 0 1 │
  │ 3 0 │
  └     ┘
```

## 🔢 13. sample_least_squares_svd
- 주제: SVD 기반 최소제곱 해법
- 설명:
    - 과결정 시스템 Ax ≈ b에 대해 SVD로 해를 구하고, 잔차(norm)를 계산합니다.
    - 정규방정식 조건 Aᵀ(Ax − b) ≈ 0도 검증
- 활용 예:
    - 데이터 피팅, 회귀 분석, 노이즈가 있는 측정값 보정
### 테스트 코드
```rust
/* 3) 선형최소제곱 (QR) — 과결정 시스템 Ax≈b */
#[test]
fn sample_least_squares_svd() {
    use nalgebra::{DMatrix, DVector};

    let dm = DMatrix::from_row_slice(4, 2, &[1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]);
    let b = DVector::from_row_slice(&[1.0, 2.0, 2.0, 4.0]);

    // SVD 최소제곱 해: Option< DMatrix<f64> > (2x1) 반환
    let x = dm
        .clone()
        .svd(true, true)
        .solve(&b, 1e-12)
        .expect("SVD solve failed"); // x: DMatrix<f64> (2x1)

        println!("Solved x = {}", x);
        println!("dm = {}", dm);
        println!("b = {}", b);
    

    // 이 데이터셋은 완전 일치가 아니라서 잔차가 0이 아닙니다. (~0.8366)
    let resid = (&dm * &x - &b).norm();
    assert!(resid < 1.0); // 현실적인 기준 (혹은 ≈0.836 확인)
    // 해 값 자체도 확인 (이론해: [0.9, 0.9])
    assert!(((x[(0, 0)] - 0.9) as f64).abs() < 1e-12);
    assert!(((x[(1, 0)] - 0.9) as f64).abs() < 1e-12);

    // 정규방정식 조건: Aᵀ(Ax − b) ≈ 0
    let normal_resid = dm.transpose() * (&dm * &x - &b);
    assert!(normal_resid.norm() < 1e-10);
}

```
### 출력 결과
```
Solved x = 
  ┌                    ┐
  │ 0.8999999999999988 │
  │ 0.9000000000000008 │
  └                    ┘


dm = 
  ┌     ┐
  │ 1 0 │
  │ 1 1 │
  │ 1 2 │
  │ 1 3 │
  └     ┘


b = 
  ┌   ┐
  │ 1 │
  │ 2 │
  │ 2 │
  │ 4 │
  └   ┘
```

## 🔢 14. sample_cholesky_solve
- 주제: Cholesky 분해를 통한 SPD 시스템 해법
- 설명:
    - 대칭 양의 정부호 행렬 K에 대해 Kx = b를 푸는 예제
    - Cholesky::new(K)로 분해 후 solve(&b)
- 활용 예:
    - 물리 시뮬레이션, 최적화, covariance 행렬 처리
### 테스트 코드
```rust
/* 4) SPD 시스템 (Cholesky) — Kx=b */
#[test]
fn sample_cholesky_solve() {
    let k = m_from_rows(&[&[4.0, 2.0, 0.0], &[2.0, 10.0, 4.0], &[0.0, 4.0, 5.0]]);
    let dk: DMatrix<f64> = (&k).into();
    let b = DVector::from_row_slice(&[2.0, 6.0, 5.0]);
    let chol = Cholesky::new(dk.clone()).expect("not SPD?");
    let x = chol.solve(&b);
    
    println!("Solved x = {}", x);
    println!("b = {}", b);
    println!("dk * x = {}", dk * x);
    assert!((dk * x - b).norm() < 1e-10);
}
```
### 출력 결과
```
Solved x = 
  ┌                     ┐
  │ 0.41379310344827586 │
  │ 0.17241379310344832 │
  │  0.8620689655172413 │
  └                     ┘


b = 
  ┌   ┐
  │ 2 │
  │ 6 │
  │ 5 │
  └   ┘


dk * x = 
  ┌   ┐
  │ 2 │
  │ 6 │
  │ 5 │
  └   ┘

```

## 🔢 15. sample_svd_pseudoinverse
- 주제: SVD 기반 의사역행렬 계산
- 설명:
    - A⁺ = V * S⁺ * Uᵗ 형태로 의사역행렬을 계산하고, A * A⁺ * A ≈ A를 검증
    - 작은 특이값은 무시하고 역수 계산
- 활용 예:
    - 랭크가 낮은 행렬의 역행렬 근사, 저차 근사, 데이터 압축
### 테스트 코드
```rust
#[test]
fn sample_svd_pseudoinverse() {
    let a = m_from_rows(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]); // 2x3
    let dm: DMatrix<f64> = (&a).into();
    let svd = SVD::new(dm.clone(), true, true);
    // 간단한 의사역행렬 A^+ = V * S^+ * U^T
    let (u, s, vt) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
    let s_plus = {
        let mut sp = DMatrix::<f64>::zeros(vt.nrows(), u.ncols());
        for i in 0..s.len() {
            if s[i] > 1e-12 {
                sp[(i, i)] = 1.0 / s[i];
            }
        }
        sp
    };
    let a_pinv = &vt.transpose() * s_plus * u.transpose();
    print!("&dm * &a_pinv * &dm = {}", &dm * &a_pinv * &dm);
    print!("dm = {}", dm);
    print!("a_pinv = {}", a_pinv);
    // 검증: A * A^+ * A ≈ A
    assert!((&dm * &a_pinv * &dm - dm).norm() < 1e-8);
}
```
### 출력 결과
```
&dm * &a_pinv * &dm = 
  ┌                                                          ┐
  │ 0.9999999999999973 1.9999999999999991  3.000000000000001 │
  │  3.999999999999994  4.999999999999997  6.000000000000002 │
  └                                                          ┘

dm = 
  ┌       ┐
  │ 1 2 3 │
  │ 4 5 6 │
  └       ┘
  
  a_pinv = 
  ┌                                           ┐
  │  -0.9444444444444443   0.4444444444444443 │
  │ -0.11111111111111045   0.1111111111111108 │
  │   0.7222222222222228 -0.22222222222222243 │
  └                                           ┘

```

## 🔢 16. sample_symmetric_eigen
- 주제: 대칭 행렬의 고윳값 분해
- 사용된 타입/메서드:
    - SymmetricEigen::new, DMatrix::from_diagonal, eigenvectors, eigenvalues
- 설명:
    - 대칭 행렬 A에 대해 고윳값 분해 A=Q\Lambda Q^T를 수행합니다.
    - SymmetricEigen은 대칭 행렬에 특화된 안정적 고윳값 계산 방식입니다.
    - 재구성된 행렬과 원래 행렬의 차이를 norm()으로 검증합니다.
- 활용 예:
    - 물리 시스템의 진동 모드 분석, PCA(주성분 분석), 스펙트럼 해석
### 테스트 코드
```rust
#[test]
fn sample_symmetric_eigen() {
    let a = m_from_rows(&[&[2.0, -1.0, 0.0], &[-1.0, 2.0, -1.0], &[0.0, -1.0, 2.0]]);
    let dm: DMatrix<f64> = (&a).into();
    // 대칭이므로 SymmetricEigen
    let se = SymmetricEigen::new(dm.clone());
    // 재구성: A ≈ Q * Λ * Q^T
    let recon = &se.eigenvectors
        * DMatrix::from_diagonal(&se.eigenvalues)
        * se.eigenvectors.transpose();
    println!("recon {}", recon);
    assert!((recon - dm).norm() < 1e-8);
}
```
### 출력 결과
```
recon 
  ┌                                                                                             ┐
  │                 1.9999999999999991   -0.9999999999999997 0.00000000000000016653345369377348 │
  │                -0.9999999999999997    1.9999999999999993                -0.9999999999999997 │
  │ 0.00000000000000016653345369377348   -0.9999999999999997                 2.0000000000000004 │
  └                                                                                             ┘
```

## 🔢 17. sample_schur
- 주제: 일반 행렬의 Schur 분해
- 사용된 타입/메서드:
    - Schur::new, unpack(), Q, T
- 설명:
    - 일반 행렬 A에 대해 A=QTQ^T 형태의 Schur 분해를 수행합니다.
    - T는 상삼각 행렬이며, 고윳값 계산의 안정성을 높이는 데 사용됩니다.
- 활용 예:
    - 고윳값 계산 보조, 안정적 수치 해석, 제어 이론
### 테스트 코드
```rust
#[test]
fn sample_schur() {
    let a = m_from_rows(&[&[1.0, 2.0, 3.0], &[0.0, 4.0, 5.0], &[0.0, 0.0, 6.0]]);
    let dm: DMatrix<f64> = (&a).into();
    let schur = Schur::new(dm.clone());
    // A ≈ Q T Q^T
    let (q, t) = schur.unpack();
    let recon = &q * t * q.transpose();
    println!("recon {}", recon);
    assert!((recon - dm).norm() < 1e-8);
}
```
### 출력 결과
```
recon 
  ┌       ┐
  │ 1 2 3 │
  │ 0 4 5 │
  │ 0 0 6 │
  └       ┘
```

## 🔢 18. sample_static_small
- 주제: 정적 크기 행렬/벡터의 선형 시스템 해법
- 사용된 타입/메서드:
    - SMatrix, SVector, lu.solve
- 설명:
    - 크기가 고정된 소형 행렬/벡터를 사용하여 빠르고 안전한 연산을 수행합니다.
    - SMatrix는 컴파일 타임에 크기가 결정되므로 성능과 안전성이 높습니다.
- 활용 예:
    - 임베디드 시스템, 실시간 처리, 고정 구조의 수치 계산
### 테스트 코드
```rust
#[test]
fn sample_static_small() {
    let m: SMatrix<f64, 3, 3> =
        SMatrix::from_row_slice(&[1.0, 2.0, 0.0, 2.0, 5.0, 1.0, 0.0, 1.0, 3.0]);
    let v: SVector<f64, 3> = SVector::from_row_slice(&[1.0, 0.0, -1.0]);
    let x = m.lu().solve(&v).unwrap();
    println!("x = {}", x);
    println!("m * x = {}", m * x);
    assert!((m * x - v).norm() < 1e-12);
}
```
### 출력 결과
```
x = 
  ┌      ┐
  │    6 │
  │ -2.5 │
  │  0.5 │
  └      ┘


m * x = 
  ┌    ┐
  │  1 │
  │  0 │
  │ -1 │
  └    ┘
```

## 🔢 19. sample_slice_block_ops
- 주제: 행렬의 슬라이스 및 블록 연산
- 사용된 메서드:
    - slice, slice_mut, fill
- 설명:
    - 행렬의 특정 부분(서브매트릭스)을 슬라이스로 추출하거나 수정합니다.
    - slice_mut을 통해 블록을 직접 수정할 수 있습니다.
- 활용 예:
    - 부분 행렬 처리, 이미지 처리, 블록 기반 알고리즘
### 테스트 코드
```rust
/* 슬라이스/블록 연산 — 부분 행렬/벡터 접근 */
#[test]
fn sample_slice_block_ops() {
    let a = m_from_rows(&[
        &[1.0, 2.0, 3.0, 4.0],
        &[5.0, 6.0, 7.0, 8.0],
        &[9.0, 10.0, 11.0, 12.0],
    ]);
    let dm: DMatrix<f64> = (&a).into(); // 3x4
    let sub = dm.slice((0, 1), (2, 3)); // rows 0..2, cols 1..4 → 2x3
    assert_eq!(sub.nrows(), 2);
    assert_eq!(sub.ncols(), 3);

    // 블록 만들고 쓰기
    let mut m = dm.clone();
    let mut blk = m.slice_mut((1, 1), (2, 2)); // 2x2
    blk.fill(0.0);
    println!("m = {:?}", m);
    assert_eq!(m[(1, 1)], 0.0);
    assert_eq!(m[(2, 2)], 0.0);
}
```
### 출력 결과
```
m = VecStorage { data: [1.0, 5.0, 9.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0, 4.0, 8.0, 12.0], 
nrows: Dyn(3), ncols: Dyn(4) }
```

## 🔢 20. sample_row_col_vectors
- 주제: 행 벡터와 열 벡터의 내적 계산
- 사용된 타입/메서드:
    - RowDVector, DVector, * 연산자
- 설명:
    - RowDVector와 DVector를 곱하여 스칼라 값을 계산합니다.
    - 내적 계산의 기본 구조를 보여주는 간단한 예제입니다.
- 활용 예:
    - 선형대수 기본 연산, 머신러닝의 dot product, 행렬 곱셈 구성
### 테스트 코드
```rust
#[test]
fn sample_row_col_vectors() {
    let r = RowDVector::from_row_slice(&[1.0, 2.0, 3.0]);
    let c = DVector::from_row_slice(&[4.0, 5.0, 6.0]);
    let dot = r.clone() * c.clone(); // 1x3 * 3x1 = 1x1
    assert_eq!(dot[(0, 0)], 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
}
```


## 🔢 21. sample_geometry_transforms
- 주제: 3D 기하학 변환 (회전, 병진, 유사변환)
- 사용된 타입/메서드:
    - Rotation3, Translation3, Isometry3, Similarity3, UnitQuaternion
- 설명:
    - 3D 벡터와 포인트에 대해 회전, 병진, 스케일을 적용하는 다양한 변환을 실습합니다.
    - Isometry3는 회전 + 병진, Similarity3는 여기에 스케일까지 포함한 변환입니다.
    - UnitQuaternion을 이용한 회전도 포함되어 있어, 회전 표현의 다양성을 보여줍니다.
- 활용 예:
    - 로봇공학, 3D 그래픽스, 물체 추적, 좌표계 변환
### 테스트 코드
```rust
/* 3D 기하 — 회전/병진/유사/쿼터니언 (Vector3D 사용) */
#[test]
fn sample_geometry_transforms() {
    // 우리 Vector3D 데이터를 기하에 사용
    let p = v3(1.0, 0.0, 0.0);
    let axis = v3(0.0, 0.0, 1.0);

    let rot = Rotation3::from_axis_angle(
        &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)),
        std::f64::consts::FRAC_PI_2,
    );
    let trans = Translation3::new(0.0, 2.0, 0.0);
    let q = UnitQuaternion::from_rotation_matrix(&rot);
    let iso = Isometry3::from_parts(trans, q);
    let sim = Similarity3::from_isometry(iso, 2.0); // scale 포함

    // 포인트 변환 (Vector3 → nalgebra::Point3 로 일시 변환)
    let pt = nalgebra::Point3::new(p.x, p.y, p.z);
    let p_iso = iso * pt;
    let p_sim = sim * pt;

    // 쿼터니언 회전
    let q = UnitQuaternion::from_axis_angle(
        &nalgebra::Vector3::z_axis(),
        std::f64::consts::FRAC_PI_2,
    );
    let v = nalgebra::Vector3::new(p.x, p.y, p.z);
    let v_rot = q * v;

    println!("iso = {}", iso);
    println!("sim = {}", sim);
    println!("q = {}", q);
    println!("p_iso = {}", p_iso);
    println!("p_sim = {}", p_sim);
    println!("v_rot = {}", v_rot);

    assert!((p_iso.x - 0.0).abs() < 1e-12 && (p_iso.y - 3.0).abs() < 1e-12);
    assert!((p_sim.y - 4.0).abs() < 1e-12);
    assert!((v_rot.x - 0.0).abs() < 1e-12 && (v_rot.y - 1.0).abs() < 1e-12);
}
```
### 출력 결과
```
iso = Isometry {
Translation {

  ┌       ┐
  │ 0.000 │
  │ 2.000 │
  │ 0.000 │
  └       ┘
}
UnitQuaternion angle: 1.5707963267948966 − axis: (0, 0, 1)}



sim = Similarity {
Isometry {
Translation {
  ┌       ┐
  │ 0.000 │
  │ 2.000 │
  │ 0.000 │
  └       ┘
}
UnitQuaternion angle: 1.5707963267948966 − axis: (0, 0, 1)}
Scaling: 2.000}



q = UnitQuaternion angle: 1.5707963267948966 − axis: (0, 0, 1)


p_iso = {0.0000000000000002220446049250313, 3, 0}
p_sim = {0.0000000000000004440892098500626, 4, 0}
v_rot = 
  ┌                                    ┐
  │ -0.0000000000000002220446049250313 │
  │                 1.0000000000000002 │
  │                                  0 │
  └                                    ┘

```

## 🔢 22. sample_rigid_fit_svd
- 주제: SVD 기반 강체 정합 (rigid registration)
- 설명:
    - 두 점 집합 P와 Q 사이에서 최적의 회전 R과 이동 t를 찾아 Q≈RP+t를 만족시키는 문제
    - 중심화 → 공분산 행렬 → SVD → 회전 행렬 → 이동 벡터 계산
    - Kabsch 알고리즘 구현 예제
- 활용 예:
    - 3D 스캔 정합, 모션 캡처 데이터 정렬, 컴퓨터 비전에서의 포즈 추정
### 테스트 코드
```rust
/* 12) 좌표계 적합(최소자승) — rigid fit (SVD) */
#[test]
fn sample_rigid_fit_svd() {
    // P: N×3 (행에 포인트)
    let p = DMatrix::from_row_slice(3, 3, &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);

    // Q = R*P + t  (여기선 정답 생성)
    let rot =
        Rotation3::from_axis_angle(&nalgebra::Vector3::z_axis(), std::f64::consts::FRAC_PI_2);
    let trans = Vector3::new(0.0, 2.0, 0.0);

    // Q: N×3
    let q = {
        let mut out = DMatrix::<f64>::zeros(p.nrows(), p.ncols());
        for i in 0..p.nrows() {
            let v = Vector3::new(p[(i, 0)], p[(i, 1)], p[(i, 2)]);
            let w = rot * v + trans;
            out[(i, 0)] = w.x;
            out[(i, 1)] = w.y;
            out[(i, 2)] = w.z;
        }
        out
    };

    // === 열벡터(3×N)로 변환 ===
    let x = p.transpose(); // 3×N
    let y = q.transpose(); // 3×N

    // === 중심화 === (각 집합의 centroid)
    let pc_row: RowDVector<f64> = p.row_mean(); // 1×3
    let qc_row: RowDVector<f64> = q.row_mean(); // 1×3
    let pc = Vector3::new(pc_row[0], pc_row[1], pc_row[2]); // 3×1
    let qc = Vector3::new(qc_row[0], qc_row[1], qc_row[2]); // 3×1

    // X0 = X - pc·1^T, Y0 = Y - qc·1^T  (3×N)
    let (_m, n) = (x.nrows(), x.ncols()); // m=3, n=N
    let mut x0 = x.clone();
    let mut y0 = y.clone();
    for j in 0..n {
        x0[(0, j)] -= pc.x;
        x0[(1, j)] -= pc.y;
        x0[(2, j)] -= pc.z;
        y0[(0, j)] -= qc.x;
        y0[(1, j)] -= qc.y;
        y0[(2, j)] -= qc.z;
    }

    // === Kabsch ===
    // H = X0 * Y0^T  (3×3)
    let h = &x0 * y0.transpose();

    // SVD(H) = U Σ V^T
    let svd = SVD::new(h.clone(), true, true);
    let u = svd.u.as_ref().expect("U missing").clone(); // 3×3
    let vt = svd.v_t.as_ref().expect("V^T missing").clone(); // 3×3

    // R = V D U^T,  D = diag(1,1,sign)
    // sign = sign(det(V*U^T))  (반사 방지)
    let v = vt.transpose();
    let mut d = Matrix3::<f64>::identity();
    let sign = (v.clone() * u.transpose()).determinant().signum();
    if sign < 0.0 {
        d[(2, 2)] = -1.0;
    }
    let r = v * d * u.transpose(); // 3×3, 회전

    let r_clone = r.clone();
    // t = qc - R*pc
    let t = qc - r * pc;

    // === 검증: Q ≈ (R*P + t) ===
    // 행 포인트에 적용하려면: P * R^T + 1·t^T   (N×3)
    let recon = {
        let pr = &p * r_clone.transpose();
        let t_row = RowDVector::from_row_slice(&[t.x, t.y, t.z]);
        let rows: Vec<RowDVector<f64>> = vec![t_row; p.nrows()];
        &pr + DMatrix::from_rows(&rows)
    };

    let err = (&recon - &q).norm();
    assert!(err < 1e-10, "rigid fit failed: ||RP+t - Q|| = {}", err);
}

```
### 출력 결과
```
recon = 
  ┌                                                                                                          ┐
  │ 0.00000000000000005551115123125783                                  2                                  0 │
  │  0.0000000000000003885780586188048                 3.0000000000000004                                  0 │
  │                -1.0000000000000004                 2.0000000000000004                                  0 │
  └                                                                                                          ┘
```

## 🔢 23. create_points
- 주제: Point3 생성 방법
- 설명:
    - 다양한 방식으로 3D 포인트를 생성하는 예제
    - 직접 좌표 지정
    - 벡터로부터 변환
    - 원점에 벡터 더하기
    - 동차 좌표(homogeneous coordinates)에서 변환
- 활용 예:
    - 좌표계 간 변환, 3D 모델링, 그래픽스 API 연동
### 테스트 코드
```rust
#[test]
fn create_points() {
    let p0 = Point3::new(2.0, 3.0, 4.0);

    // Build from a coordinates vector.
    let coords = Vector3::new(2.0, 3.0, 4.0);
    let p1 = Point3::from(coords);

    // Build by translating the origin.
    let translation = Vector3::new(2.0, 3.0, 4.0);
    let p2 = Point3::origin() + translation;

    // Build from homogeneous coordinates. The last component of the
    // vector will be removed and all other components divided by 10.0.
    let homogeneous_coords = Vector4::new(20.0, 30.0, 40.0, 10.0);
    let p3 = Point3::from_homogeneous(homogeneous_coords);

    assert_eq!(p0, p1);
    assert_eq!(p0, p2);
    assert_eq!(p0, p3.unwrap());
}
```

## 🔢 24. matrix_reshape
- 주제: 행렬 재구성 (reshape)
- 설명:
    - Matrix2x3 → Matrix3x2로 정적 크기 행렬 재구성
    - DMatrix는 런타임 시 크기 검사로 reshape 가능
    - 정적 행렬은 컴파일 타임에 크기 불일치 시 컴파일 에러 발생
- 활용 예:
    - 데이터 구조 변환, 이미지 처리, 행렬 레이아웃 변경
### 테스트 코드
```rust
#[test]
fn matrix_reshape() {
    // Matrices can be reshaped in-place without moving or copying values.
    let m1 = Matrix2x3::new(1.1, 1.2, 1.3, 2.1, 2.2, 2.3);
    let m2 = Matrix3x2::new(1.1, 2.2, 2.1, 1.3, 1.2, 2.3);

    let m3 = m1.reshape_generic(Const::<3>, Const::<2>);
    assert_eq!(m3, m2);

    // Note that, for statically sized matrices, invalid reshapes will not compile:
    //let m4 = m3.reshape_generic(U3, U3);

    // If dynamically sized matrices are used, the reshaping is checked at run-time.
    let dm1 = DMatrix::from_row_slice(
        4,
        3,
        &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
    );
    let dm2 = DMatrix::from_row_slice(
        6,
        2,
        &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    );

    let dm3 = dm1.reshape_generic(Dyn(6), Dyn(2));

    assert_eq!(dm3, dm2);

    // Invalid reshapings of dynamic matrices will panic at run-time.
    //let dm4 = dm3.reshape_generic(Dyn(6), Dyn(6));
}
```
### 출력 결과
```
m3 = 
  ┌         ┐
  │ 1.1 2.2 │
  │ 2.1 1.3 │
  │ 1.2 2.3 │
  └         ┘


dm3 = 
  ┌     ┐
  │ 1 0 │
  │ 0 1 │
  │ 0 0 │
  │ 0 1 │
  │ 0 0 │
  │ 0 0 │
  └     ┘
```

## 🔢 25. use_dedicated_types
- 주제: 2D 기하학 전용 타입 사용
- 설명:
    - Isometry2, Point2, Vector2를 사용하여 2D 회전 및 병진 변환을 수행
    - ON_PI는 90도 회전을 의미
- 활용 예:
    - 2D 게임, CAD, 로봇 경로 계획
### 테스트 코드
```rust
#[test]
fn use_dedicated_types() {
    let iso = Isometry2::new(Vector2::new(1.0, 1.0), ON_PI);
    let pt = Point2::new(1.0, 0.0);
    let vec = Vector2::x();

    let transformed_pt = iso * pt;
    let transformed_vec = iso * vec;

    println!("transformed_pt = {}", transformed_pt);
    println!("transformed_vec = {}", transformed_vec);

    assert_relative_eq!(transformed_pt, Point2::new(0.0, 1.0));
    assert_relative_eq!(transformed_vec, Vector2::new(-1.0, 0.0));
}
```
### 출력 결과
```
transformed_pt = {0, 1.0000000000000002}
transformed_vec = 
  ┌                                    ┐
  │                                 -1 │
  │ 0.00000000000000012246467991473532 │
  └                                    ┘

```


## 🔢 26. use_homogeneous_coordinates
- 주제: 동차 좌표를 활용한 2D 기하학 변환
- 사용된 타입/메서드:
    - Isometry2, Point2, Vector2, .to_homogeneous(), .from_homogeneous()
- 설명:
    - 2D 회전 및 병진 변환을 동차 좌표로 표현하고, 변환 후 다시 일반 좌표로 복원합니다.
    - 동차 좌표는 행렬 곱으로 변환을 통합할 수 있어 그래픽스에서 널리 사용됩니다.
- 활용 예:
    - 2D 그래픽스, OpenGL/DirectX 변환, CAD 시스템
### 테스트 코드
```rust
#[test]
fn use_homogeneous_coordinates() {
    let iso = Isometry2::new(Vector2::new(1.0, 1.0), ON_PI);
    let pt = Point2::new(1.0, 0.0);
    let vec = Vector2::x();

    // Compute using homogeneous coordinates.
    let hom_iso = iso.to_homogeneous();
    let hom_pt = pt.to_homogeneous();
    let hom_vec = vec.to_homogeneous();

    println!("{:?}", hom_iso);

    let hom_transformed_pt = hom_iso * hom_pt;
    let hom_transformed_vec = hom_iso * hom_vec;

    // Convert back to the cartesian coordinates.
    let transformed_pt = Point2::from_homogeneous(hom_transformed_pt).unwrap();
    let transformed_vec = Vector2::from_homogeneous(hom_transformed_vec).unwrap();

    assert_relative_eq!(transformed_pt, Point2::new(0.0, 1.0));
    assert_relative_eq!(transformed_vec, Vector2::new(-1.0, 0.0));
}
```

## 🔢 27. linear_system_resolution
- 주제: 다양한 방식의 선형 시스템 해법
- 사용된 기능:
    - Matrix4, Vector4, Matrix4x3, lu.solve, solve_mut
- 설명:
    - 단일 우변 벡터, 다중 우변 행렬, 인플레이스 해법 등 다양한 방식으로 Ax = b를 풉니다.
    - solve_mut는 메모리 할당 없이 기존 벡터를 직접 수정합니다.
- 활용 예:
    - 성능 최적화가 중요한 실시간 시스템, 물리 시뮬레이션, 그래픽스 셰이더 계산
### 테스트 코드
```rust
#[test]
fn linear_system_resolution() {
    let a = Matrix4::new(
        1.0, 1.0, 2.0, -5.0, 2.0, 5.0, -1.0, -9.0, 2.0, 1.0, -1.0, 3.0, 1.0, 3.0, 2.0, 7.0,
    );
    let mut b = Vector4::new(3.0, -3.0, -11.0, -5.0);
    let decomp = a.lu();
    let x = decomp.solve(&b).expect("Linear resolution failed.");
    assert_relative_eq!(a * x, b);
    assert_relative_eq!(a * x, b);

    /*
        * It is possible to perform the resolution in-place.
        * This is particularly useful to avoid allocations when
        * `b` is a `DVector` or a `DMatrix`.
        */
    assert!(decomp.solve_mut(&mut b), "Linear resolution failed.");
    assert_relative_eq!(x, b);

    /*
        * It is possible to solve multiple systems
        * simultaneously by using a matrix for `b`.
        */
    let b = Matrix4x3::new(
        3.0, 2.0, 0.0, -3.0, 0.0, 0.0, -11.0, 5.0, -3.0, -5.0, 10.0, 4.0,
    );
    let x = decomp.solve(&b).expect("Linear resolution failed.");
    assert_relative_eq!(a * x, b);
    assert_relative_eq!(a * x, b);
}

```
### 출력 결과
```
x = 
  ┌                     ┐
  │ -3.6666666666666665 │
  │                  -0 │
  │  1.6666666666666667 │
  │ -0.6666666666666666 │
  └                     ┘


x = 
  ┌                                                             ┐
  │ -3.6666666666666665  1.9833333333333334  -1.927777777777778 │
  │                  -0                 0.5  1.1666666666666667 │
  │  1.6666666666666667  1.2166666666666668  0.7277777777777777 │
  │ -0.6666666666666666  0.5833333333333334  0.1388888888888889 │
  └                      
```

### 🔢 28. model_view_projection
- 주제: 3D 그래픽스의 모델-뷰-투영 변환
- 사용된 타입/메서드:
    - Isometry3, Perspective3, .to_homogeneous(), .as_matrix()
- 설명:
    - 객체의 위치(모델), 카메라의 시점(뷰), 원근 투영(프로젝션)을 결합하여 최종 변환 행렬을 생성합니다.
    - model_view_projection = projection * view * model
- 활용 예:
    - 3D 렌더링 파이프라인, 게임 엔진, AR/VR 시스템
### 테스트 코드
```rust
#[test]
fn model_view_projection() {
    // Our object is translated along the x axis.
    let model = Isometry3::new(Vector3::x(), nalgebra::zero());

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye = Point3::new(0.0, 0.0, 1.0);
    let target = Point3::new(1.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = Perspective3::new(16.0 / 9.0, ON_PI / 2.0, 1.0, 1000.0);

    // The combination of the model with the view is still an isometry.
    let model_view = view * model;

    // Convert everything to a `Matrix4` so that they can be combined.
    let mat_model_view = model_view.to_homogeneous();

    // Combine everything.
    let model_view_projection = projection.as_matrix() * mat_model_view;
    println!("{:?}", model_view_projection);
}
```
### 출력 결과
```rust
[[0.39774756441743303, 0.0, 0.7085224103781121, 0.7071067811865475], [0.0, 1.0000000000000002, 0.0, 0.0], [0.39774756441743303, 0.0, -0.7085224103781121, -0.7071067811865475], [6.245004513516507e-17, 0.0, -0.5849571812457779, 1.414213562373095]]
```

## 🔢 29. raw_pointer
- 주제: 벡터/행렬의 원시 포인터 접근
- 사용된 메서드:
    - .as_slice(), .as_ptr(), unsafe 포인터 연산
- 설명:
    - Vector3, Point3, Matrix3의 내부 데이터를 포인터로 접근하여 외부 API에 전달하는 예제
    - 그래픽스 API(OpenGL, Vulkan 등)와의 연동에 필수적인 패턴
- 활용 예:
    - GPU 연산, C/C++ 라이브러리 연동, FFI
### 테스트 코드
```rust
#[test]
fn raw_pointer() {
    let v = Vector3::new(1.0f32, 0.0, 1.0);
    let p = Point3::new(1.0f32, 0.0, 1.0);
    let m = nalgebra::one::<Matrix3<f32>>();

    // Convert to arrays.
    let v_array = v.as_slice();
    let p_array = p.coords.as_slice();
    let m_array = m.as_slice();

    // Get data pointers.
    let v_pointer = v_array.as_ptr();
    let p_pointer = p_array.as_ptr();
    let m_pointer = m_array.as_ptr();

    /* Then pass the raw pointers to some graphics API. */

    #[allow(clippy::float_cmp)]
    unsafe {
        assert_eq!(*v_pointer, 1.0);
        assert_eq!(*v_pointer.offset(1), 0.0);
        assert_eq!(*v_pointer.offset(2), 1.0);

        assert_eq!(*p_pointer, 1.0);
        assert_eq!(*p_pointer.offset(1), 0.0);
        assert_eq!(*p_pointer.offset(2), 1.0);

        assert_eq!(*m_pointer, 1.0);
        assert_eq!(*m_pointer.offset(4), 1.0);
        assert_eq!(*m_pointer.offset(8), 1.0);
    }
}
```

## 🔢 30. scalar_genericity
- 주제: 스칼라 타입 제네릭 처리
- 사용된 트레이트:
-    Scalar, RealField
- 설명:
    - Vector3<T>에서 T가 정수든 실수든 상관없이 처리할 수 있도록 제네릭 트레이트를 활용
    - RealField는 .norm() 같은 수치 연산을 가능하게 함
- 활용 예:
    - 범용 수치 라이브러리 작성, 타입 안정성 확보
### 테스트 코드
```rust
fn print_vector<T: Scalar>(m: &Vector3<T>) {
    println!("{:?}", m)
}

fn print_norm<T: RealField>(v: &Vector3<T>) {
    // NOTE: alternatively, nalgebra already defines `v.norm()`.
    let norm = v.dot(v).sqrt();

    // The RealField bound implies that T is Display so we can
    // use "{}" instead of "{:?}" for the format string.
    println!("{}", norm)
}
 #[test]
    fn scalar_genericity() {
    let v1 = Vector3::new(1, 2, 3);
    let v2 = Vector3::new(1.0, 2.0, 3.0);

    print_vector(&v1);
    print_norm(&v2);
}

```
### 출력 결과
```
[[1, 2, 3]]
3.7416573867739413

```

## 🔢 31. transform_matrix4
- 주제: 3D 변환 행렬을 통한 벡터/포인트 변환
- 사용된 타입/메서드:
    - Matrix4::new_scaling, append_nonuniform_scaling_mut, append_translation, transform_vector, transform_point, from_scaled_axis
- 설명:
    - 균일/비균일 스케일링, 회전, 평행이동을 Matrix4를 통해 조합합니다.
    - transform_vector는 방향 벡터에, transform_point는 위치 벡터에 적용됩니다.
    - 회전 행렬과의 곱셈 순서에 따라 결과가 달라지는 것도 확인합니다.
- 활용 예:
    - 3D 모델의 위치/크기/방향 제어, 게임 엔진의 월드 변환, 물체 애니메이션
### 테스트 코드
```rust
#[test]
fn transform_matrix4() {
    // Create a uniform scaling matrix with scaling factor 2.
    let mut m = Matrix4::new_scaling(2.0);

    assert_eq!(m.transform_vector(&Vector3::x()), Vector3::x() * 2.0);
    assert_eq!(m.transform_vector(&Vector3::y()), Vector3::y() * 2.0);
    assert_eq!(m.transform_vector(&Vector3::z()), Vector3::z() * 2.0);

    // Append a nonuniform scaling in-place.
    m.append_nonuniform_scaling_mut(&Vector3::new(1.0, 2.0, 3.0));

    assert_eq!(m.transform_vector(&Vector3::x()), Vector3::x() * 2.0);
    assert_eq!(m.transform_vector(&Vector3::y()), Vector3::y() * 4.0);
    assert_eq!(m.transform_vector(&Vector3::z()), Vector3::z() * 6.0);

    // Append a translation out-of-place.
    let m2 = m.append_translation(&Vector3::new(42.0, 0.0, 0.0));

    assert_eq!(
        m2.transform_point(&Point3::new(1.0, 1.0, 1.0)),
        Point3::new(42.0 + 2.0, 4.0, 6.0)
    );

    // Create rotation.
    let rot = Matrix4::from_scaled_axis(Vector3::x() * ON_PI);
    let rot_then_m = m * rot; // Right-multiplication is equivalent to prepending `rot` to `m`.
    let m_then_rot = rot * m; // Left-multiplication is equivalent to appending `rot` to `m`.

    let pt = Point3::new(1.0, 2.0, 3.0);

    assert_relative_eq!(
        m.transform_point(&rot.transform_point(&pt)),
        rot_then_m.transform_point(&pt)
    );
    assert_relative_eq!(
        rot.transform_point(&m.transform_point(&pt)),
        m_then_rot.transform_point(&pt)
    );
}
```
### 출력 결과
```
m.transform_point(&rot.transform_point(&pt) = {2, -8.000000000000002, -17.999999999999996}
rot_then_m.transform_point(&pt) = {2, -8.000000000000002, -18}
rot.transform_point(&m.transform_point(&pt)) = {2, -8.000000000000002, -18}
m_then_rot.transform_point(&pt) = {2, -8.000000000000002, -18}
```

## 🔢 32. transform_vector_point3
- 주제: 포인트의 동차 좌표 변환과 검증
- 사용된 메서드:
    - Matrix4::new_rotation_wrt_point, append_scaling_mut, transform_point, Point3::from_homogeneous
- 설명:
    - 회전 중심을 기준으로 회전한 후 스케일링을 적용합니다.
    - Point3를 직접 변환한 결과와, 동차 좌표로 변환 후 다시 복원한 결과가 일치하는지 검증합니다.
- 활용 예:
    - 정밀한 3D 변환 검증, 그래픽스 파이프라인에서의 좌표 변환 일관성 확인
### 테스트 코드
```rust
#[test]
fn transform_vector_point3() {
    let mut m =
        Matrix4::new_rotation_wrt_point(Vector3::x() * 1.57, Point3::new(1.0, 2.0, 1.0));
    m.append_scaling_mut(2.0);

    let point1 = Point3::new(2.0, 3.0, 4.0);
    let homogeneous_point2 = Vector4::new(2.0, 3.0, 4.0, 1.0);

    // First option: use the dedicated `.transform_point(...)` method.
    let transformed_point1 = m.transform_point(&point1);
    // Second option: use the homogeneous coordinates of the point.
    let transformed_homogeneous_point2 = m * homogeneous_point2;

    // Recover the 3D point from its 4D homogeneous coordinates.
    let transformed_point2 = Point3::from_homogeneous(transformed_homogeneous_point2);

    // Check that transforming the 3D point with the `.transform_point` method is
    // indeed equivalent to multiplying its 4D homogeneous coordinates by the 4x4
    // matrix.
    println!("transformed_point1 = {:?}", transformed_point1);
    assert_eq!(transformed_point1, transformed_point2.unwrap());
}
```
### 출력 결과
```
transformed_point1 = [4.0, -1.9984054441695411, 4.004777326128069]
```

## 🔢 33. transformation_pointer
- 주제: 변환 행렬의 원시 포인터 접근
- 사용된 메서드:
    - Isometry3::to_homogeneous, .as_slice(), .as_ptr()
- 설명:
    - Isometry3를 동차 행렬로 변환한 후, 내부 데이터를 포인터로 접근합니다.
    - 그래픽스 API(OpenGL 등)에 행렬 데이터를 넘길 때 필요한 방식입니다.
- 활용 예:
    - GPU 연산, FFI(C/C++ 연동), 실시간 렌더링에서의 행렬 전달
### 테스트 코드
```rust
#[test]
fn transformation_pointer() {
    let iso = Isometry3::new(Vector3::new(1.0f32, 0.0, 1.0), nalgebra::zero());

    // Compute the homogeneous coordinates first.
    let iso_matrix = iso.to_homogeneous();
    let iso_array = iso_matrix.as_slice();
    let iso_pointer = iso_array.as_ptr();

    /* Then pass the raw pointer to some graphics API. */

    #[allow(clippy::float_cmp)]
    unsafe {
        assert_eq!(*iso_pointer, 1.0);
        assert_eq!(*iso_pointer.offset(5), 1.0);
        assert_eq!(*iso_pointer.offset(10), 1.0);
        assert_eq!(*iso_pointer.offset(15), 1.0);

        assert_eq!(*iso_pointer.offset(12), 1.0);
        assert_eq!(*iso_pointer.offset(13), 0.0);
        assert_eq!(*iso_pointer.offset(14), 1.0);
    }
}
```
---




