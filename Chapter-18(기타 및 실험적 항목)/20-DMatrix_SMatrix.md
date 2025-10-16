# DMatrix / SMatrix

## 🧮 DMatrix 핵심 함수 요약 및 샘플

### ✅ 기본 설정
```rust
use nalgebra::{DMatrix, DVector};
```

### ✅ 1. Identity 행렬 생성
```rust
use nalgebra::DMatrix;

fn main() {
    let identity = DMatrix::<f64>::identity(3, 3);
    println!("Identity matrix:\n{}", identity);
}
```

- 함수: DMatrix::identity(rows, cols)
- 설명: 단위 행렬 생성 (정방행렬만 의미 있음)

### ✅ 2. 전치 (Transpose)
```rust
fn main() {
    let m = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let transposed = m.transpose();
    println!("Transposed:\n{}", transposed);
}
```

- 함수: matrix.transpose()
- 설명: 행과 열을 뒤바꿈

### ✅ 3. 역행렬 (Inverse)
```rust
fn main() {
    let m = DMatrix::from_row_slice(2, 2, &[4.0, 7.0, 2.0, 6.0]);
    if let Some(inv) = m.try_inverse() {
        println!("Inverse:\n{}", inv);
    } else {
        println!("Matrix is not invertible.");
    }
}
```

- 함수: matrix.try_inverse()
- 설명: 역행렬 반환 (정방행렬만 가능, 실패 시 None)

### ✅ 4. 특정 행(row) 추출
```rust
fn main() {
    let m = DMatrix::from_row_slice(3, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    let row = m.row(1); // 두 번째 행
    println!("Row 1: {}", row);
}
```

- 함수: matrix.row(index)
- 설명: RowVector로 반환됨

### ✅ 5. 특정 열(column) 추출
```rust
fn main() {
    let m = DMatrix::from_row_slice(3, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    let col = m.column(2); // 세 번째 열
    println!("Column 2: {}", col);
}
```

- 함수: matrix.column(index)
- 설명: Vector로 반환됨

### ✅ 6. 행렬 곱셈
```rust
fn main() {
    let a = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = DMatrix::from_row_slice(3, 2, &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    let product = &a * &b;
    println!("Product:\n{}", product);
}
```

- 연산자: *
- 설명: 행렬 곱 (내적 규칙에 따라 크기 맞아야 함)

### ✅ 7. 행렬 합/차
```rust
fn main() {
    let a = DMatrix::from_element(2, 2, 1.0);
    let b = DMatrix::from_element(2, 2, 2.0);
    let sum = &a + &b;
    let diff = &a - &b;
    println!("Sum:\n{}", sum);
    println!("Difference:\n{}", diff);
}
```

- 연산자: +, -
- 설명: 같은 크기의 행렬끼리만 가능

### ✅ 8. 스칼라 곱/나눗셈
```rust
fn main() {
    let m = DMatrix::from_element(2, 2, 3.0);
    let scaled = &m * 2.0;
    let divided = &m / 3.0;
    println!("Scaled:\n{}", scaled);
    println!("Divided:\n{}", divided);
}
```

- 연산자: *, /
- 설명: 모든 원소에 대해 스칼라 연산

## 📌 참고
- DMatrix는 동적 크기 행렬로, 런타임에 크기를 지정할 수 있어요.
- SMatrix는 정적 크기 행렬로, 컴파일 타임에 크기가 고정됩니다.
- try_inverse()는 실패 가능성이 있으므로 항상 Option 처리 필요합니다.


## 🧮 SMatrix 핵심 함수 요약 및 샘플

### ✅ 기본 설정
```rust
use nalgebra::{SMatrix, SVector};
```


### 🧮 1. Identity 행렬
```rust
fn identity_example() {
    let identity: SMatrix<f64, 3, 3> = SMatrix::identity();
    println!("Identity matrix:\n{}", identity);
}
```


### 🔄 2. 전치 (Transpose)
```rust
fn transpose_example() {
    let m = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let transposed = m.transpose();
    println!("Transposed:\n{}", transposed);
}
```

### 🔁 3. 역행렬 (Inverse)
```rust
fn inverse_example() {
    let m = SMatrix::<f64, 2, 2>::from_row_slice(&[4.0, 7.0, 2.0, 6.0]);
    if let Some(inv) = m.try_inverse() {
        println!("Inverse:\n{}", inv);
    } else {
        println!("Matrix is not invertible.");
    }
}
```

### 📐 4. 특정 행/열 추출
```rust
fn row_col_example() {
    let m = SMatrix::<f64, 3, 3>::from_row_slice(&[
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]);
    let row = m.row(1);
    let col = m.column(2);
    println!("Row 1: {}", row);
    println!("Column 2: {}", col);
}
```


### ✖️ 5. 행렬 곱셈
```rust
fn multiplication_example() {
    let a = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = SMatrix::<f64, 3, 2>::from_row_slice(&[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    let product = a * b;
    println!("Product:\n{}", product);
}
```


### ➕➖ 6. 행렬 합/차
```rust
fn add_sub_example() {
    let a = SMatrix::<f64, 2, 2>::from_element(1.0);
    let b = SMatrix::<f64, 2, 2>::from_element(2.0);
    let sum = a + b;
    let diff = a - b;
    println!("Sum:\n{}", sum);
    println!("Difference:\n{}", diff);
}
```


### 🔢 7. 스칼라 곱/나눗셈
```rust
fn scalar_example() {
    let m = SMatrix::<f64, 2, 2>::from_element(3.0);
    let scaled = m * 2.0;
    let divided = m / 3.0;
    println!("Scaled:\n{}", scaled);
    println!("Divided:\n{}", divided);
}
```

## 🧠 요약 비교
| 기능             | DMatrix 사용 예시      | SMatrix 사용 예시   | 차이점 및 특징     |
|------------------|-------------------------|-----------------|-----------------------|
| 단위 행렬 생성    | DMatrix::<f64>::identity(3, 3)   | SMatrix::<f64, 3, 3>::identity()   | DMatrix는 런타임 크기, SMatrix는 컴파일 타임 고정 크기 |
| 행/열 추출       | .row(i), .column(j)      | .row(i), .column(j)   | 사용 방식 동일, 반환 타입은 고정 크기 여부에 따라 다름 |
| 역행렬 계산       | .try_inverse()        | .try_inverse()             | 정방행렬일 때만 가능, SMatrix는 성능상 더 빠름         |

---

# slice 사용

```rust
let m = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
```
임시 슬라이스 리터럴을 참조해서 값을 복사하는 방식
## ✅ 목적: 외부 heap 데이터를 만들지 않고, 소유권도 넘기지 않기 위해
- &[1.0, ...]는 임시로 스택에 생성된 슬라이스.
- from_row_slice()는 그 값을 복사해서 SMatrix 내부에 저장하고, 원본은 바로 drop됩니다.
- 즉, heap에 따로 벡터를 만들 필요 없이, 짧게 쓰고 버리는 방식.  

&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]처럼 리터럴 배열을 참조하는 경우, Rust는 이 데이터를 **스택(stack)** 에 저장합니다.

## ✅ 정리하면
- [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]  
    → 스택에 저장된 고정 크기 배열 ([f64; 6])
- &[...]  
    → 이 배열에 대한 불변 참조 (&[f64])
- from_row_slice(&[...])  
    → 이 참조된 데이터를 복사해서 SMatrix 내부에 저장
- 함수 호출이 끝나면  
    → 이 임시 배열은 drop되고 사라짐 (하지만 이미 복사됐기 때문에 문제 없음)

## 💡 참고: heap에 안 올라가는 이유
Rust는 [T; N]처럼 크기가 고정된 배열은 스택에 저장합니다.  
heap에 올라가려면 Box<[T]>, Vec<T> 같은 동적 크기 컨테이너를 씀.

## 🔍 예시 비교
| 표현식                        | 저장 위치 | 소유권 있음? | 특징 설명                                                                 |
|------------------------------|------------|---------------|---------------------------------------------------------------------------|
| [1.0, 2.0, 3.0]              | 스택       | ✅ 있음        | 고정 크기 배열. 스택에 직접 저장되며 빠름                                 |
| &[1.0, 2.0, 3.0]            | 스택 참조  | ❌ 없음        | 스택에 있는 배열을 참조. 값은 복사되지 않음                              |
| vec![1.0, 2.0, 3.0]         | 힙         | ✅ 있음        | 동적 크기. heap에 저장되며 크기 변경 가능                                 |
| Box::new([1.0, 2.0, 3.0])   | 힙         | ✅ 있음        | 고정 크기 배열을 heap에 저장. 소유권 있음                                 |

## ⚡ 왜 속도가 빠를까?
| 요소           | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| [f64; N]       | 고정 크기 배열 → 스택에 저장됨 → 할당/해제 비용이 거의 없음              |
| f64            | Copy 트레잇을 구현한 원시 타입 → 복사 비용이 매우 낮음                    |
| &[...]         | 참조만 넘김 → 소유권 이동 없음 → heap 할당 없이 빠르게 처리 가능          |

## 이 조합 덕분에 SMatrix::from_row_slice(&[...]) 같은 코드는
- ✔️ 빠르고
- ✔️ 안전하고
-= ✔️ 메모리 낭비도 없어요

## ✅ 실전에서 왜 이렇게 쓰는가?
- 테스트 코드, 벤치마크, 샘플 생성 등에서 빠르게 데이터를 넘기고 싶을 때
- heap에 벡터를 만들 필요 없이 간단하게 SMatrix나 SVector를 초기화할 때
- 소유권을 넘기지 않고 안전하게 복사만 하고 싶을 때

## 💡 예시 비교
```rust
let m = SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
// 빠르고 안전. 스택에 잠깐 생성된 배열을 복사해서 사용
```

### vec 사용
```rust
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
let m = SMatrix::<f64, 2, 3>::from_row_slice(&data);
// heap에 있는 데이터를 복사. 성능은 괜찮지만 메모리 관리가 더 복잡
```



