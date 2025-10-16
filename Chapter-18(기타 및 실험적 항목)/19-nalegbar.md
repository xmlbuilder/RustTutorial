# nalgebra DMatricx / DVector

## 🧮 DVector 주요 함수 및 예제
```rust

use nalgebra::DVector;

fn main() {
    // 생성
    let v = DVector::from_vec(vec![1.0, 2.0, 3.0]);

    // 길이
    println!("Length: {}", v.len());

    // 인덱스 접근
    println!("v[1] = {}", v[1]);

    // 스칼라 곱
    let scaled = &v * 2.0;
    println!("Scaled: {}", scaled);

    // 벡터 합
    let v2 = DVector::from_element(3, 1.0);
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


## 🧮 DMatrix 주요 함수 및 예제
```rust
use nalgebra::DMatrix;

fn main() {
    // 생성
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
    if let Some(inv) = square.try_inverse() {
        println!("Inverse:\n{}", inv);
    }

    // 행렬식
    println!("Determinant: {}", square.determinant());

    // LU 분해
    let lu = square.lu();
    println!("LU Decomposition:\nL:\n{}\nU:\n{}", lu.l(), lu.u());
}
```


## 📌 핵심 함수 요약
| 타입     | 주요 함수                            | 설명                                               |
|----------|-------------------------------------|----------------------------------------------------|
| DVector  | from_vec, len, dot, norm            | 벡터 생성, 길이 확인, 내적 계산, 벡터 크기(norm) 계산 |
| DMatrix  | from_row_slice, transpose           | 행렬 생성, 전치(transpose)                         |
| DMatrix  | try_inverse, determinant            | 역행렬 계산, 행렬식 계산 (정방행렬에 한함)         |
| DMatrix  | *, +, lu()                          | 행렬 곱셈, 덧셈, LU 분해                            |

---

# LU 

LU 분해를 활용해 선형 방정식 Ax = b의 해를 구하는 방식은 nalgebra에서 매우 직관적으로 처리할 수 있음.  
아래에 다양한 상황별로 lu.solve()를 사용하는 샘플을 정리.

## ✅ 기본 예제: 2x2 시스템
```rust
use nalgebra::{DMatrix, DVector};

fn main() {
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


## ✅ 예제: 3x3 시스템
```rust
fn main() {
    let a = DMatrix::from_row_slice(3, 3, &[
        2.0, -1.0, 0.0,
        -1.0, 2.0, -1.0,
        0.0, -1.0, 2.0,
    ]);
    let b = DVector::from_row_slice(&[1.0, 0.0, 1.0]);

    let lu = a.lu();
    let x = lu.solve(&b).expect("Should solve");
    println!("x = {}", x);
}
```


## ✅ 예제: 해가 없는 경우 (특이 행렬)
```rust
fn main() {
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 2.0, 4.0]); // rank-deficient
    let b = DVector::from_row_slice(&[3.0, 6.0]);

    let lu = a.lu();
    match lu.solve(&b) {
        Some(x) => println!("x = {}", x),
        None => println!("Matrix is singular or system has no unique solution."),
    }
}
```


## ✅ 예제: 여러 b 벡터에 대해 반복적으로 해 구하기
```rust
fn main() {
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


## ✅ 예제: LU 분해로 해 구한 뒤 검증
```rust
fn main() {
    let a = DMatrix::from_row_slice(2, 2, &[4.0, 3.0, 6.0, 3.0]);
    let b = DVector::from_row_slice(&[10.0, 12.0]);

    let lu = a.lu();
    let x = lu.solve(&b).expect("Should solve");

    let b_check = &a * &x;
    println!("Original b: {}", b);
    println!("Computed b: {}", b_check);
}

```

## 🔍 핵심 요약
| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| lu.solve(&b)     | LU 분해를 통해 Ax = b 형태의 선형 시스템을 해결                         |
| 반환 타입        | Option<DVector<f64>> → 해가 없거나 특이 행렬이면 None 반환               |
| 해 검증          | 계산된 해 x에 대해 a * x ≈ b로 결과 검증 가능                            |
| 안정성           | 일반적인 시스템에 대해 빠르고 안정적이나, 특이 행렬에는 실패 가능         |


---

# QR / Cholesky / SVD


## ✅ 기본적으로 제공되는 분해 기능들
| 분해 방식   | 사용 함수               | 설명                                                                 |
|-------------|------------------------|----------------------------------------------------------------------|
| QR 분해      | qr()                   | 직교 행렬 기반 분해. 최소제곱 해법이나 안정적인 수치 계산에 사용         |
| Cholesky 분해 | cholesky().solve(&b)   | 대칭 양의 정부호 행렬에 대해 빠르고 안정적인 해법 제공                  |
| SVD 분해     | svd()                  | 특이값 분해. 특이 행렬이나 과적합 방지, 차원 축소 등에 활용 가능         |


## ✅ QR 분해 예제
```rust
use nalgebra::{DMatrix, DVector};

fn main() {
    // A * x = b 형태의 선형 시스템
    let a = DMatrix::from_row_slice(3, 2, &[
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0,
    ]);
    let b = DVector::from_row_slice(&[7.0, 8.0, 9.0]);

    // QR 분해
    let qr = a.qr();
    let x = qr.solve(&b).expect("QR solve failed");

    println!("Solution x = {}", x);
}
```

- QR은 정방행렬이 아니거나 과잉결정 시스템에도 잘 작동해요.
- 최소제곱 해법에 자주 사용됩니다.

## ✅ Cholesky 분해 예제
```rust
use nalgebra::{DMatrix, DVector};

fn main() {
    // 대칭 양의 정부호 행렬 A
    let a = DMatrix::from_row_slice(3, 3, &[
        4.0, 1.0, 1.0,
        1.0, 3.0, 0.0,
        1.0, 0.0, 2.0,
    ]);
    let b = DVector::from_row_slice(&[1.0, 2.0, 3.0]);

    // Cholesky 분해
    let chol = a.cholesky().expect("Matrix is not positive definite");
    let x = chol.solve(&b);

    println!("Solution x = {}", x);
}
```

- Cholesky는 대칭 + 양의 정부호 조건이 있어야 사용 가능해요.
- 매우 빠르고 안정적인 분해 방식입니다.

## ✅ SVD 분해 예제
```rust
use nalgebra::{DMatrix, DVector};

fn main() {
    // A * x = b 형태의 선형 시스템
    let a = DMatrix::from_row_slice(3, 2, &[
        1.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
    ]);
    let b = DVector::from_row_slice(&[1.0, 2.0, 3.0]);

    // SVD 분해
    let svd = a.svd(true, true);
    let x = svd.solve(&b, 1e-6);

    println!("Solution x = {}", x);
}
```

- SVD는 특이 행렬, 과적합, 차원 축소 등 다양한 상황에 유용해요.
- solve(&b, tol)에서 tol은 작은 특이값 무시 기준입니다.

## 🧠 요약 비교
| 분해 방식   | 사용 예시                   | 조건                          | 특징                                      | 장점                                       |
|-------------|-----------------------------|-------------------------------|-------------------------------------------|--------------------------------------------|
| QR          | qr().solve(&b)              | 아무 행렬                     | 직교 행렬 기반 최소제곱 해법               | 과잉결정 시스템에 적합, 안정적              |
| Cholesky    | cholesky().solve(&b)        | 대칭 + 양의 정부호 행렬       | 빠르고 효율적인 분해                      | 조건 만족 시 가장 빠르고 정확함             |
| SVD         | svd().solve(&b, tol)        | 아무 행렬                     | 특이값 분해 기반 안정적 해법               | 특이 행렬, 과적합 방지, 차원 축소에 유용     |

---



