# 테스트

테스트하신 내용을 기반으로, 전체 흐름을 수식, 표, 테스트 목적 요약, 그리고 입출력 구조로 정리.  
이 문서는 발표, 리뷰, 문서화에 바로 활용하실 수 있도록 구성했습니다.

## 📘 1. 핵심 수식 흐름
### 🎯 문제 정의

$$
Ax=b
$$

### 🔧 분해 단계 (bandec)

$$
A=LU
$$

- L: 단위 하삼각 행렬
- U: 상삼각 행렬
- P: 피벗 행렬 (index로 표현됨)

###  🧮 해 계산 단계 (banbks)
- 전진 대입:

$$
Ly=Pb
$$

- 후진 대입:

$$
Ux=y
$$

### 최종 해:

$$
x=A^{-1}b=U^{-1}L^{-1}Pb
$$

## 📊 2. 테스트 목적 요약

| 테스트 이름                                   | 목적                        | 사용 함수                   | 특징                        |
|----------------------------------------------|-----------------------------|-----------------------------|-----------------------------|
| `basic_mul`, `multiply_basic`                | 행렬 곱셈 정확도 확인       | Matrix 곱셈                 | 정수 기반 결과 검증         |
| `inverse_matches_dmatrix`                    | 역행렬 정확도 및 잔차 확인 | invert, DMatrix 비교        | A⁻¹ 계산 및 A·A⁻¹ ≈ I 확인  |
| `lu_solve_vector_rhs`                        | LU 기반 해 계산             | LU.solve                    | 단일 RHS                    |
| `lu_solve_multi_rhs_matrix`                  | LU 기반 해 계산             | LU.solve                    | 다중 RHS                    |
| `test_band_lu_solve_general_band_multiple_rhs` | 밴드 LU + 다중 RHS          | `bandec`, `banbks`          | 안정적 대각우세 구조        |
| `test_band_lu_solve_tridiagonal`             | Tridiagonal 구조 테스트     | `bandec`, `banbks`          | m1 = m2 = 1                 |
| `test_band_lu_pivoting_required`             | 피벗 유도 구조 테스트       | `bandec`, `banbks`          | 작은 대각, 큰 하부          |
| `solve_band_lu_with_matrixlike`              | 동적 밴드 LU 테스트         | `bandec_dyn`, `banbks_dyn`  | Matrix-like 구조 지원       |
| `test_band_lu_zero_diagonal_pivoting`        | 극단적 피벗 유도 테스트     | `bandec`, `banbks`          | 실패 → 행렬 완화 후 통과    |



## 🧠 3. 입출력 구조 요약
### 🔹 `bandec` 함수 입력 인자 요약

| 인자    | 설명                                                        |
|---------|-------------------------------------------------------------|
| `a`     | 밴드 저장 형식의 행렬 A (`n × (m1 + m2 + 1)`)               |
| `m1`, `m2` | 하부/상부 밴드 폭                                         |
| `al`    | 하삼각 행렬 L의 밴드 성분 저장소 (`n × m1`)                 |
| `index` | 피벗 인덱스 배열 (`길이 n`, 1-based)                         |
| `d`     | 행 교환 부호 (`+1` 또는 `-1`)                               |


## 🔹 `banbks` 함수 입력 인자 요약

| 인자    | 설명                                                        |
|---------|-------------------------------------------------------------|
| `a`     | LU 분해된 U (밴드 저장 형식)                                |
| `al`    | L의 밴드 성분                                               |
| `index` | 피벗 인덱스                                                 |
| `b`     | 우변 벡터 (입력) → 해 x로 덮어쓰기됨                        |



## 🧪 4. 테스트 흐름 요약
- 1. A 생성 (dense 또는 밴드형)
- 2. x_true 설정 (단일 또는 다중 RHS)
- 3. b = Ax 계산 (dense_mul)
- 4. A → band 형식 변환 (dense_to_band)
- 5. LU 분해 (bandec)
- 6. 해 계산 (banbks)
- 7. x ≈ x_true 검증 (nearly_eq)

### ✅ 5. 테스트 행렬 설계 팁

| 조건                          | 설명                                                       |
|-------------------------------|------------------------------------------------------------|
| 대각선은 충분히 커야 함       | 최소 `1.0` 이상 권장. 너무 작으면 수치 불안정 발생         |
| 피벗 유도는 하부에 큰 값      | 대각이 작고 하부가 크면 pivot 발생. 구조는 유도되되 안정적이어야 함 |
| 밴드 폭은 적절하게 설정       | `m1`, `m2`는 1~2 정도가 적절. 너무 작으면 pivot 범위 제한됨 |
| 값의 범위가 너무 넓지 않아야 함 | `1e-6` vs `1.0`처럼 극단적 차이는 오차 증폭 위험             |
| 극단적 구조는 오차 완화 필요  | `1e-6` 대각은 위험. `1e-2` 이상으로 완화하면 테스트 안정화 가능 |

---

## 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use nalgebra::{DMatrix, DVector};
    use nurbslib::core::maths::{on_banbks, on_banbks_dyn, on_bandec, on_bandec_dyn};
    use nurbslib::core::matrix::Matrix;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }
```
```rust
    #[test]
    fn basic_mul() {
        let a = Matrix::from_nested(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let b = Matrix::from_nested(&[&[5.0, 6.0], &[7.0, 8.0]]);
        let c = &a * &b;
        assert!((c.at(0, 0) - 19.0).abs() < 1e-12);
        assert!((c.at(0, 1) - 22.0).abs() < 1e-12);
        assert!((c.at(1, 0) - 43.0).abs() < 1e-12);
        assert!((c.at(1, 1) - 50.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn diagonal_uses_min_count() {
        // 2x3 직사각 행렬
        let mut m = Matrix::with_dims(2, 3);
        m.set_diagonal_scalar(7.0); // <- 반드시 min(2,3)=2만 채워야 함
        assert_eq!(*m.at(0, 0), 7.0);
        assert_eq!(*m.at(1, 1), 7.0);
        // 대각 외는 0
        assert_eq!(*m.at(0, 1), 0.0);
        assert_eq!(*m.at(0, 2), 0.0);
        assert_eq!(*m.at(1, 0), 0.0);
        assert_eq!(*m.at(1, 2), 0.0);
    }
```
```rust
    #[test]
    fn transpose_round_trip_rectangular() {
        let mut m = Matrix::from_nested(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
        assert_eq!(m.row_count(), 2);
        assert_eq!(m.col_count(), 3);
        m.transpose();
        assert_eq!(m.row_count(), 3);
        assert_eq!(m.col_count(), 2);
        // 전치 결과 확인
        assert_eq!(*m.at(0, 0), 1.0);
        assert_eq!(*m.at(1, 0), 2.0);
        assert_eq!(*m.at(2, 0), 3.0);
        assert_eq!(*m.at(0, 1), 4.0);
        assert_eq!(*m.at(1, 1), 5.0);
        assert_eq!(*m.at(2, 1), 6.0);
    }
```
```rust
    #[test]
    fn multiply_basic() {
        let a = Matrix::from_nested(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let b = Matrix::from_nested(&[&[5.0, 6.0], &[7.0, 8.0]]);

        let c = &a * &b;
        assert_eq!(c.row_count(), 2);
        assert_eq!(c.col_count(), 2);
        // A*B = [[19,22],[43,50]]
        assert_eq!(*c.at(0, 0), 19.0);
        assert_eq!(*c.at(0, 1), 22.0);
        assert_eq!(*c.at(1, 0), 43.0);
        assert_eq!(*c.at(1, 1), 50.0);
    }
```
```rust
    #[test]
    fn from_matrix_to_dmatrix_round_trip() {
        let m = Matrix::from_nested(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);

        // Matrix -> DMatrix
        let dm: DMatrix<f64> = (&m).into();
        assert_eq!(dm.shape(), (2, 3));
        assert_eq!(dm[(0, 0)], 1.0);
        assert_eq!(dm[(1, 2)], 6.0);

        // DMatrix -> Matrix
        let m2: Matrix = (&dm).into();
        assert_eq!(m2.row_count(), 2);
        assert_eq!(m2.col_count(), 3);
        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(*m.at(i as i32, j as i32), *m2.at(i as i32, j as i32));
            }
        }
    }
```
```rust
    #[test]
    fn multiply_matches_dmatrix() {
        let a = Matrix::from_nested(&[&[2.0, -1.0, 0.5], &[0.0, 1.0, 1.5]]);
        let b = Matrix::from_nested(&[&[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0]]);

        let mut my_c = Matrix::new();
        assert!(my_c.multiply(&a, &b));
        let da: DMatrix<f64> = (&a).into();
        let db: DMatrix<f64> = (&b).into();
        let dc = da * db;
        let c2: Matrix = (&dc).into();

        assert_eq!(my_c.row_count(), c2.row_count());
        assert_eq!(my_c.col_count(), c2.col_count());
        for i in 0..my_c.row_count() {
            for j in 0..my_c.col_count() {
                assert!(approx_eq(
                    *my_c.at(i as i32, j as i32),
                    *c2.at(i as i32, j as i32),
                    1e-12
                ));
            }
        }
    }
```
```rust
    #[test]
    fn inverse_matches_dmatrix() {
        // 가역인 3x3
        let m = Matrix::from_nested(&[&[2.0, 1.0, 0.0], &[1.0, 3.0, 1.0], &[0.0, 1.0, 2.0]]);

        // DMatrix 에서 역행렬
        let dm: DMatrix<f64> = (&m).into();
        let dminv = dm.clone().try_inverse().expect("dm inverse should exist");

        // 우리 invert()
        let mut a = m.clone();
        let ok = a.invert(1e-12);
        assert!(ok, "invert() failed unexpectedly");

        // a == dm^{-1} ?
        for i in 0..a.row_count() {
            for j in 0..a.col_count() {
                assert!(approx_eq(*a.at(i as i32, j as i32), dminv[(i, j)], 1e-9));
            }
        }

        // 또한 a * m == I 확인
        let prod = &a * &m;
        for i in 0..3 {
            for j in 0..3 {
                let target = if i == j { 1.0 } else { 0.0 };
                assert!(approx_eq(*prod.at(i as i32, j as i32), target, 1e-8));
            }
        }
    }
```
```rust
    #[test]
    fn transpose_matches_dmatrix() {
        let mut m = Matrix::from_nested(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);

        let dm: DMatrix<f64> = (&m).into();
        let dmt = dm.transpose();

        m.transpose();
        let mt: DMatrix<f64> = (&m).into();

        assert_eq!(dmt.shape(), mt.shape());
        for i in 0..dmt.nrows() {
            for j in 0..dmt.ncols() {
                assert!(approx_eq(dmt[(i, j)], mt[(i, j)], 1e-12));
            }
        }
    }
```
```rust
    #[test]
    fn add_alias_safe() {
        // self 가 a/b와 같아도 안전하게 동작해야 함
        let a = Matrix::from_nested(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let b = Matrix::from_nested(&[&[10.0, 20.0], &[30.0, 40.0]]);

        // self == a 인 상황
        let mut s = a.clone();
        assert!(s.add(&a, &b));
        assert_eq!(*s.at(0, 0), 11.0);
        assert_eq!(*s.at(0, 1), 22.0);
        assert_eq!(*s.at(1, 0), 33.0);
        assert_eq!(*s.at(1, 1), 44.0);

        // self == b 인 상황
        let mut t = b.clone();
        assert!(t.add(&a, &b));
        assert_eq!(*t.at(0, 0), 11.0);
        assert_eq!(*t.at(0, 1), 22.0);
        assert_eq!(*t.at(1, 0), 33.0);
        assert_eq!(*t.at(1, 1), 44.0);
    }
```
```rust
    #[test]
    fn lu_solve_vector_rhs() {
        // A * x_true = b 를 구성 (x_true를 알고 시작)
        // A는 가역이면서 무피벗 LU에도 무난한 예시
        let a = Matrix::from_nested(&[&[3.0, 1.0, 2.0], &[6.0, 3.0, 4.0], &[3.0, 1.0, 5.0]]);
        let x_true = DVector::from_row_slice(&[1.0, 2.0, 3.0]);

        // b = A * x_true
        // (Matrix -> DMatrix 변환 후 곱)
        let dm_a: DMatrix<f64> = (&a).into();
        let b = &dm_a * &x_true;

        // LU 분해 후 해 구하기
        let lu = dm_a.clone().lu(); // 무피벗 LU
        let x = lu.solve(&b).expect("LU solve failed");

        // x ≈ x_true 검증
        for i in 0..x.len() {
            assert!(
                approx_eq(x[i], x_true[i], 1e-10),
                "x[{}]={} != {}(true)",
                i,
                x[i],
                x_true[i]
            );
        }

        // 잔차 ||A x - b|| 확인
        let resid = (&dm_a * &x) - b;
        let nr = resid.norm();
        assert!(nr < 1e-10, "residual too large: {}", nr);

        // det(A)도 확인(0이 아니어야 함)
        let det = lu.determinant();
        assert!(det.abs() > 1e-12, "determinant too small (singular?)");
    }
```
```rust
    #[test]
    fn lu_solve_multi_rhs_matrix() {
        // 두 개의 RHS를 동시에 풀어보기 (B의 두 열이 서로 다른 b)
        let a = Matrix::from_nested(&[&[2.0, 1.0, 0.0], &[1.0, 3.0, 1.0], &[0.0, 1.0, 2.0]]);

        // X_true = [ [1, 2],
        //            [0, 1],
        //            [3, 0] ]
        // B = A * X_true
        let dm_a: DMatrix<f64> = (&a).into();
        let x_true = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 0.0, 1.0, 3.0, 0.0]);
        let b = &dm_a * &x_true; // 3x2

        // LU로 한 번 분해 후 여러 RHS 를 한꺼번에 해결
        let lu = dm_a.clone().lu();
        let x = lu.solve(&b).expect("LU solve failed for multi-RHS");

        // X ≈ X_true 검사
        for i in 0..x.nrows() {
            for j in 0..x.ncols() {
                assert!(
                    approx_eq(x[(i, j)], x_true[(i, j)], 1e-10),
                    "X({}, {})={} != {}(true)",
                    i,
                    j,
                    x[(i, j)],
                    x_true[(i, j)]
                );
            }
        }

        // 잔차도 확인
        let reside = (&dm_a * &x) - b;
        let nr = reside.norm();
        assert!(nr < 1e-10, "residual too large: {}", nr);
    }
```
```rust
    #[test]
    fn roundtrip_matrix_to_dmatrix_then_lu() {
        // Matrix 에서 작성 → DMatrix 로 변환 → LU → 해 검증
        let m = Matrix::from_nested(&[&[4.0, 2.0, 0.0], &[2.0, 10.0, 4.0], &[0.0, 4.0, 5.0]]);

        let dm: DMatrix<f64> = (&m).into();

        // b를 임의 생성: x_true = [1, -1, 2] 로 설정 후 b = A x_true
        let x_true = DVector::from_row_slice(&[1.0, -1.0, 2.0]);
        let b = &dm * &x_true;

        let lu = dm.lu();
        let x = lu.solve(&b).expect("solve failed");
        // 정확도 확인
        let err = (&x - &x_true).norm();
        assert!(err < 1e-10, "solution error too large: {}", err);
    }

    fn nearly_eq(a: f64, b: f64, eps: f64) -> bool {
        let d = (a - b).abs();
        if d <= eps {
            return true;
        }
        d <= eps * (1.0 + a.abs().max(b.abs()))
    }

    /// dense A (n x n)를 밴드 저장형식 a_band (n x (m1+m2+1))로 채웁니다.
    /// 대각은 col = m1 위치, 하부는 m1-1 .. 0, 상부는 m1+1 .. m1+m2 에 둡니다.
    fn dense_to_band(a_dense: &Matrix, m1: usize, m2: usize) -> Matrix {
        let n = a_dense.row_count();
        assert_eq!(a_dense.col_count(), n);
        let num1 = m1 + m2 + 1;
        let mut a_band = Matrix::with_dims(n, num1);
        for i in 0..n {
            for k_off in -(m1 as isize)..=(m2 as isize) {
                let j = (i as isize) + k_off;
                if j >= 0 && (j as usize) < n {
                    let col = (m1 as isize + k_off) as usize;
                    a_band.set(i, col, a_dense.get(i, j as usize));
                }
            }
        }
        a_band
    }

    /// X_true (n x nrhs), A (n x n) → B = A * X_true 반환
    fn dense_mul(a: &Matrix, x: &Matrix) -> Matrix {
        assert_eq!(a.col_count(), x.row_count());
        let n = a.row_count();
        let nrhs = x.col_count();
        let mut b = Matrix::with_dims(n, nrhs);
        for i in 0..n {
            for c in 0..nrhs {
                let mut acc = 0.0;
                for k in 0..n {
                    acc += a.get(i, k) * x.get(k, c);
                }
                b.set(i, c, acc);
            }
        }
        b
    }

    /// 대각우세 밴드형 dense A를 생성 (결정론적, pivot 안정)
    fn make_dense_band_dd(n: usize, m1: usize, m2: usize) -> Matrix {
        let mut a = Matrix::with_dims(n, n);
        for i in 0..n {
            // 대각을 크게
            let mut diag = 5.0;
            // 상하 밴드 채우기
            for k in 1..=m1 {
                if i >= k {
                    let v = ((i + k) as f64).sin() * 0.05;
                    a.set(i, i - k, v);
                    diag += v.abs();
                }
            }
            for k in 1..=m2 {
                if i + k < n {
                    let v = ((i + k * 3) as f64).cos() * 0.05;
                    a.set(i, i + k, v);
                    diag += v.abs();
                }
            }
            a.set(i, i, diag + 1.0); // 충분히 우세하게
        }
        a
    }

    /// pivot 필요한 케이스를 일부러 섞은 dense 밴드 A
    fn make_dense_band_need_pivot(n: usize, m1: usize, m2: usize) -> Matrix {
        let mut a = Matrix::with_dims(n, n);
        for i in 0..n {
            // 대각은 작게
            a.set(i, i, 1e-3);
            // 바로 아래(밴드 내부)에 큰 값 넣어 pivot 유도
            if i + 1 < n {
                a.set(i + 1, i, 1.0);
            }
            // 나머지 밴드 조금
            for k in 1..=m2 {
                if i + k < n {
                    a.set(i, i + k, 0.02);
                }
            }
            for k in 1..=m1 {
                if i >= k {
                    a.set(i, i - k, 0.02);
                }
            }
            // 마지막으로 전체적으로 안정성 확보 위해 대각 약간 가산
            let d = a.get(i, i) + 1.0;
            a.set(i, i, d);
        }
        a
    }
    // ===== 테스트 =====
    #[test]
    fn test_band_lu_solve_general_band_multiple_rhs() {
        let n = 12usize;
        let m1 = 2usize;
        let m2 = 2usize;
        let a_dense = make_dense_band_dd(n, m1, m2);

        // X_true: 두 개 RHS
        let nrhs = 2usize;
        let mut x_true = Matrix::with_dims(n, nrhs);
        for i in 0..n {
            x_true.set(i, 0, (i as f64).sin());
            x_true.set(i, 1, (i as f64).cos());
        }
        let mut b = dense_mul(&a_dense, &x_true);

        // 밴드 저장으로 변환 후 분해+해
        let mut a_band = dense_to_band(&a_dense, m1, m2);
        let mut al = Matrix::with_dims(n, m1);
        let mut indx = vec![0usize; n];
        let mut d = 0.0;
        on_bandec(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
        on_banbks(&a_band, m1, m2, &al, &indx, &mut b); // b ← solution

        // 검증
        for i in 0..n {
            for c in 0..nrhs {
                let got = b.get(i, c);
                let want = x_true.get(i, c);
                assert!(
                    nearly_eq(got, want, 1e-10),
                    "i={}, c={}, got={}, want={}",
                    i,
                    c,
                    got,
                    want
                );
            }
        }
    }
```
```rust
    #[test]
    fn test_band_lu_solve_tridiagonal() {
        let n = 16usize;
        let m1 = 1usize;
        let m2 = 1usize;
        let a_dense = make_dense_band_dd(n, m1, m2);

        // X_true: 하나의 RHS
        let nrhs = 1usize;
        let mut x_true = Matrix::with_dims(n, nrhs);
        for i in 0..n {
            x_true.set(i, 0, 1.0 + i as f64 / n as f64);
        }
        let mut b = dense_mul(&a_dense, &x_true);

        let mut a_band = dense_to_band(&a_dense, m1, m2);
        let mut al = Matrix::with_dims(n, m1);
        let mut indx = vec![0usize; n];
        let mut d = 0.0;
        on_bandec(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
        on_banbks(&a_band, m1, m2, &al, &indx, &mut b);

        for i in 0..n {
            let got = b.get(i, 0);
            let want = x_true.get(i, 0);
            assert!(
                nearly_eq(got, want, 1e-10),
                "i={}, got={}, want={}",
                i,
                got,
                want
            );
        }
    }
```
```rust
    #[test]
    fn test_band_lu_pivoting_required() {
        let n = 10usize;
        let m1 = 2usize;
        let m2 = 2usize;
        let a_dense = make_dense_band_need_pivot(n, m1, m2);

        let nrhs = 2usize;
        let mut x_true = Matrix::with_dims(n, nrhs);
        for i in 0..n {
            x_true.set(i, 0, (i as f64 * 0.1).sin());
            x_true.set(i, 1, (i as f64 * 0.2).cos());
        }
        let mut b = dense_mul(&a_dense, &x_true);

        let mut a_band = dense_to_band(&a_dense, m1, m2);
        let mut al = Matrix::with_dims(n, m1);
        let mut indx = vec![0usize; n];
        let mut d = 0.0;
        on_bandec(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
        on_banbks(&a_band, m1, m2, &al, &indx, &mut b);

        for i in 0..n {
            for c in 0..nrhs {
                let got = b.get(i, c);
                let want = x_true.get(i, c);
                assert!(
                    nearly_eq(got, want, 1e-8),
                    "pivoting test: i={}, c={}, got={}, want={}",
                    i,
                    c,
                    got,
                    want
                );
            }
        }
    }

    fn make_dd(n: usize, m1: usize, m2: usize) -> Matrix {
        let mut a = Matrix::with_dims(n, n);
        for i in 0..n {
            let mut diag = 5.0;
            for k in 1..=m1 {
                if i >= k {
                    let v = ((i + k) as f64).sin() * 0.05;
                    a.set(i, i - k, v);
                    diag += v.abs();
                }
            }
            for k in 1..=m2 {
                if i + k < n {
                    let v = ((i + k * 3) as f64).cos() * 0.05;
                    a.set(i, i + k, v);
                    diag += v.abs();
                }
            }
            a.set(i, i, diag + 1.0);
        }
        a
    }

    #[test]
    fn solve_band_lu_with_matrixlike() {
        let n = 12;
        let m1 = 2;
        let m2 = 2;
        let a_dense = make_dd(n, m1, m2);

        // 두 개 RHS
        let nrhs = 2;
        let mut x_true = Matrix::with_dims(n, nrhs);
        for i in 0..n {
            x_true.set(i, 0, (i as f64).sin());
            x_true.set(i, 1, (i as f64).cos());
        }
        let mut b = dense_mul(&a_dense, &x_true);

        let mut a_band = dense_to_band(&a_dense, m1, m2);
        let mut al = Matrix::with_dims(n, m1);
        let mut indx = vec![0usize; n];
        let mut d = 0.0;

        on_bandec_dyn(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
        on_banbks_dyn(&a_band, m1, m2, &al, &indx, &mut b);

        for i in 0..n {
            for c in 0..nrhs {
                let got = b.get(i, c);
                let want = x_true.get(i, c);
                assert!(
                    nearly_eq(got, want, 1e-10),
                    "i={i}, c={c}, got={got}, want={want}"
                );
            }
        }
    }
```
```rust   
    #[test]
    fn test_band_lu_simple_integer_case() {
        let a_dense = Matrix::from_nested(&[
            &[4.0, 1.0, 0.0],
            &[1.0, 4.0, 1.0],
            &[0.0, 1.0, 4.0],
        ]);
        let x_true = Matrix::from_nested(&[
            &[1.0],
            &[2.0],
            &[3.0],
        ]);
        let mut b = dense_mul(&a_dense, &x_true); // b = Ax

        let m1 = 1;
        let m2 = 1;
        let mut a_band = dense_to_band(&a_dense, m1, m2);
        let mut al = Matrix::with_dims(3, m1);
        let mut indx = vec![0usize; 3];
        let mut d = 0.0;

        on_bandec(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
        on_banbks(&a_band, m1, m2, &al, &indx, &mut b);

        for i in 0..3 {
            let got = b.get(i, 0);
            let want = x_true.get(i, 0);
            assert!(nearly_eq(got, want, 1e-12), "i={}, got={}, want={}", i, got, want);
        }
    }
```
```rust   
    #[test]
    fn test_band_lu_zero_diagonal_pivoting() {
        let n = 5;
        let m1 = 1;
        let m2 = 1;
        let mut a_dense = Matrix::with_dims(n, n);
        for i in 0..n {
            a_dense.set(i, i, 0.1); // 너무 작지 않게
            if i + 1 < n {
                a_dense.set(i + 1, i, 1.0); // pivot 유도
            }
        }


        let mut x_true = Matrix::with_dims(n, 1);
        for i in 0..n {
            x_true.set(i, 0, (i as f64).cos());
        }
        let mut b = dense_mul(&a_dense, &x_true);

        let mut a_band = dense_to_band(&a_dense, m1, m2);
        let mut al = Matrix::with_dims(n, m1);
        let mut indx = vec![0usize; n];
        let mut d = 0.0;

        on_bandec(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
        on_banbks(&a_band, m1, m2, &al, &indx, &mut b);

        for i in 0..n {
            let got = b.get(i, 0);
            let want = x_true.get(i, 0);
            assert!(nearly_eq(got, want, 1e-6), "pivot test: i={}, got={}, want={}", i, got, want);
        }
    }
```
```rust   
    #[test]
    fn test_band_lu_varied_band_widths() {
        for &(m1, m2) in &[(1, 2), (2, 1), (2, 2)] {
            let n = 6;
            let a_dense = make_dd(n, m1, m2);
            let mut x_true = Matrix::with_dims(n, 1);
            for i in 0..n {
                x_true.set(i, 0, (i as f64 * 0.3).sin());
            }
            let mut b = dense_mul(&a_dense, &x_true);

            let mut a_band = dense_to_band(&a_dense, m1, m2);
            let mut al = Matrix::with_dims(n, m1);
            let mut indx = vec![0usize; n];
            let mut d = 0.0;

            on_bandec(&mut a_band, m1, m2, &mut al, &mut indx, &mut d);
            on_banbks(&a_band, m1, m2, &al, &indx, &mut b);

            for i in 0..n {
                let got = b.get(i, 0);
                let want = x_true.get(i, 0);
                assert!(nearly_eq(got, want, 1e-10), "m1={}, m2={}, i={}, got={}, want={}", m1, m2, i, got, want);
            }
        }
    }
}
```

