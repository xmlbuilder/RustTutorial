# Array
Rust의 배열(array)은 단순한 자료구조 같지만, Rust의 소유권, 타입 시스템, 안전성 철학이 깊게 반영된 구조.

## 🧠 기본 개념: Rust의 배열이란?
- 고정 크기의 동일 타입 요소를 담는 자료구조
- 선언 예시:
```rust
let arr: [i32; 3] = [1, 2, 3];
```
- 타입 [T; N]은 타입 T의 요소 N개를 담는 배열

## 🔍 다른 언어와의 차이점
| 언어      | 크기       | 타입 제약     | 메모리 안전성     | 인덱스 접근 오류 처리 |
|-----------|------------|---------------|--------------------|------------------------|
| Rust      | 고정 크기  | 동일 타입만   | 컴파일 타임 검사   | `panic!`               |
| C/C++     | 고정 크기  | 권장되나 강제 아님 | 런타임 오류 가능 | 정의되지 않은 동작 (UB) |
| Python    | 동적 크기  | 다양한 타입 가능 | 런타임 오류 가능   | `IndexError`           |
| Java      | 고정 크기  | 동일 타입만   | 런타임 검사        | `ArrayIndexOutOfBoundsException` |

## ✅ 장점
- 정적 타입 + 고정 크기로 인해 성능 최적화가 뛰어남
- 컴파일 타임에 크기와 타입 검사 → 안전성 확보
- 범위 검사로 인해 런타임 오류 방지
- const 배열도 가능 → 컴파일 타임 상수로 활용 가능

## ⚠️ 단점
- 크기 변경 불가 → 동적 배열이 필요하면 Vec<T> 사용해야 함
- 제네릭과 함께 쓰기 까다로움 → [T; N]은 N이 타입에 포함되므로 트레잇 구현이 제한됨
- 복잡한 초기화는 문법이 다소 불편할 수 있음

## 🛠️ 사용법 예시
```rust
fn main() {
    let arr = [10, 20, 30];
    println!("첫 번째 요소: {}", arr[0]);

    // 반복문
    for val in arr.iter() {
        println!("값: {}", val);
    }

    // 배열 슬라이스
    let slice = &arr[1..];
    println!("슬라이스: {:?}", slice);
}
```

## 📌 어디에 쓰면 좋을까?
| 용도/상황             | 배열 예시               | 설명                                      |
|------------------------|--------------------------|-------------------------------------------|
| 고정된 데이터 구조     | `[u8; 3]`, `[f64; 2]`     | RGB 색상, 2D 좌표 등 고정 크기 수치 표현     |
| 임베디드 시스템        | `[u8; 16]`                | 스택 기반 메모리 사용, 힙 할당 없이 처리 가능 |
| 컴파일 타임 상수       | `const LUT: [i32; 256]`   | 상수 테이블, 초기값 배열 등                 |
| 수학적 계산 구조       | `[f64; 9]` (3x3 행렬)     | 행렬, 벡터 등 수치 연산에 적합               |
| 반복 처리 최적화       | `[T; N]`                  | 루프 기반 처리 시 캐시 친화적 구조           |




## 🧠 확장: 배열 vs 벡터
| 항목             | 배열 `[T; N]`                     | 벡터 `Vec<T>`                      |
|------------------|-----------------------------------|------------------------------------|
| 크기             | 고정 크기                         | 동적 크기                          |
| 메모리 위치      | 스택                              | 힙                                 |
| 타입 제약        | 동일 타입                         | 동일 타입                          |
| 성능             | 매우 빠름 (캐시 친화적)           | 유연하지만 약간 느림               |
| 초기화 방식      | 컴파일 타임에 크기 필요           | 런타임에 크기 결정 가능            |
| 반복 처리        | 빠르고 안전한 루프 가능           | 반복 가능, 유연한 처리 가능        |
| 사용 예시        | 좌표, 색상, 행렬 등 고정 구조     | 리스트, 동적 컬렉션, 입력 데이터 등 |
| 크기 변경        | 불가능                            | `push`, `pop`, `resize` 가능       |
| 트레잇 제약      | `[T; N]`은 `N`이 타입에 포함됨     | `Vec<T>`는 제네릭 트레잇 구현이 쉬움 |

---

# 다중 배열 선언

다중 배열 선언과 배열의 크기 확인 방법은 다른 언어와는 조금 다르게, 정적 타입 시스템과 안전성을 기반으로 설계되어 있음.

## 🧱 다중 배열 선언 (Nested Arrays)
Rust에서는 배열 안에 배열을 넣어서 2차원, 3차원 배열을 만들 수 있어요. 
예를 들어:
```rust
let matrix: [[i32; 3]; 2] = [
    [1, 2, 3],
    [4, 5, 6],
];
```

- 타입은 [[i32; 3]; 2] → 2개의 [i32; 3] 배열
- matrix[0][1] → 2에 접근

### 예시: 3차원 배열
```rust
let cube: [[[u8; 2]; 3]; 4] = [[[0; 2]; 3]; 4];
```

- 4개의 3×2 배열을 가진 구조
- 모든 요소는 0으로 초기화됨

## 📏 배열의 크기 확인 방법
Rust에서는 배열의 크기를 컴파일 타임에 알고 있으며, 런타임에도 .len() 메서드로 확인할 수 있음.
```rust
let arr = [10, 20, 30, 40];
println!("크기: {}", arr.len()); // 출력: 4
```

### 다중 배열의 크기 확인
```rust
let matrix = [[1, 2, 3], [4, 5, 6]];
println!("행 개수: {}", matrix.len());        // 2
println!("열 개수: {}", matrix[0].len());     // 3
```

.len()은 usize 타입을 반환하며, 슬라이스에서도 동일하게 동작합니다.



## 🔍 배열 vs 슬라이스

| 항목         | 배열 `[T; N]`             | 슬라이스 `&[T]`, `&mut [T]`         |
|--------------|---------------------------|-------------------------------------|
| 크기         | 고정 크기 (컴파일 타임)   | 동적 크기 (런타임에 결정됨)         |
| 메모리 위치  | 스택                      | 힙 또는 스택                        |
| 타입         | `[T; N]`                  | `&[T]`, `&mut [T]`                  |
| .len() 사용  | 가능 (`arr.len()`)        | 가능 (`slice.len()`)               |
| 사용 예시    | 고정된 구조, 성능 우선    | 함수 인자, 유연한 데이터 처리       |
| 변경 가능성  | 불가능                    | `&mut [T]`는 내부 값 변경 가능       |
| 트레잇 구현  | `[T; N]`은 제약 있음      | 슬라이스는 대부분의 트레잇 구현 가능 |

## 🧠 실전 팁
- CAD 시스템에서 행렬, 좌표, LUT 테이블 등은 다중 배열로 표현하기 좋습니다.
- .len()을 통해 크기를 확인하고, for 루프나 .iter()로 안전하게 순회할 수 있어요.
- 배열은 스택에 저장되므로 성능이 뛰어나지만 크기 변경은 불가능합니다. 동적 구조가 필요하면 Vec<Vec<T>>로 전환하세요.

---

# Slice

Rust의 **슬라이스(slice)**는 단순한 참조처럼 보이지만, Rust의 안전성과 성능 철학이 아주 잘 녹아든 구조. 

## 🧠 슬라이스란?
- 슬라이스는 컬렉션의 연속된 일부 요소에 대한 참조입니다.
- 배열, 벡터, 문자열 등에서 일부분만 빌려서 읽거나 수정할 수 있게 해줍니다.
- 슬라이스는 크기를 포함한 포인터로 표현되며, 소유권을 가지지 않습니다.
```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4]; // [2, 3, 4]
```


## 🔍 슬라이스의 종류
| 슬라이스 타입   | 원본 타입 또는 용도     |
|----------------|--------------------------|
| `&[T]`         | 불변 슬라이스 (읽기 전용) |
| `&mut [T]`     | 가변 슬라이스 (읽기 + 쓰기 가능) |
| `&str`         | `String` 또는 문자열 리터럴의 슬라이스 |

### ✅ 장점
- 메모리 안전성: 슬라이스는 범위 검사와 참조 규칙을 따르므로 안전함
- 유연성: 배열, 벡터, 문자열 등 다양한 컬렉션에서 사용 가능
- 성능: 힙 할당 없이 참조만 하므로 빠름
- 함수 인자 전달에 최적화: 소유권 없이 데이터를 넘길 수 있음

### ⚠️ 단점
- 크기 고정 불가: 슬라이스는 런타임에 크기가 결정되므로 [T; N]처럼 타입에 크기를 포함할 수 없음
- 소유권 없음: 슬라이스만으로 데이터를 생성하거나 확장할 수 없음
- 라이프타임 제약: 참조이기 때문에 원본 데이터보다 오래 살 수 없음

### 🛠️ 사용 예시
```rust
fn print_slice(slice: &[i32]) {
    for val in slice {
        println!("{}", val);
    }
}

fn main() {
    let arr = [10, 20, 30, 40, 50];
    let part = &arr[1..4]; // 슬라이스: [20, 30, 40]
    print_slice(part);
}
```

## 📏 슬라이스의 크기 확인
```rust
let slice = &arr[2..];
println!("길이: {}", slice.len()); // 출력: 3
```
슬라이스는 내부적으로 시작 포인터 + 길이를 저장하므로 .len()으로 크기를 확인할 수 있어요.

## 🔄 배열 vs 슬라이스 요약
| 항목         | 배열 `[T; N]`             | 슬라이스 `&[T]`, `&mut [T]`         |
|--------------|---------------------------|-------------------------------------|
| 크기         | 고정 크기 (컴파일 타임)   | 동적 크기 (런타임에 결정됨)         |
| 메모리 위치  | 스택                      | 힙 또는 스택                        |
| 소유권       | 있음                      | 없음                                |
| .len() 사용  | 가능 (`arr.len()`)        | 가능 (`slice.len()`)               |
| 사용 예시    | 고정 구조, 성능 우선      | 함수 인자, 유연한 데이터 처리       |



## 📌 어디에 쓰면 좋을까?
- 함수 인자 전달: 배열이나 벡터의 일부만 넘길 때
- 부분 처리: 전체 데이터를 나누어 처리할 때
- 문자열 조작: &str로 부분 문자열을 다룰 때
- 성능 최적화: 힙 할당 없이 참조만으로 처리할 때

---

# Generic + Array

## 🧠 기본 구조: 제네릭 + 배열
```rust
fn process_array<T>(arr: [T; 4]) {
    // T는 어떤 타입이든 가능
    for item in arr.iter() {
        println!("{:?}", item);
    }
}
```

- 위 예시는 [T; 4]처럼 크기가 고정된 배열을 제네릭 타입으로 받는 함수입니다.
- T는 Copy, Debug, Clone 등 필요한 트레잇을 제약으로 추가할 수 있어요.

## 🔍 배열 크기 N도 제네릭으로 만들기
Rust 1.51부터는 const generics를 통해 배열 크기도 제네릭으로 처리할 수 있음:

```rust
fn process_array<T, const N: usize>(arr: [T; N]) {
    println!("크기: {}", arr.len());
}
```


- N은 컴파일 타임 상수로 처리되며, 다양한 크기의 배열을 하나의 함수로 처리 가능
- 예: [i32; 3], [i32; 10] 등 모두 처리 가능

## ✅ 장점
| 항목             | 설명                                               |
|------------------|----------------------------------------------------|
| 타입 유연성       | 다양한 타입의 배열을 하나의 함수/구조체로 처리 가능 |
| 크기 유연성       | const generics로 다양한 크기의 배열을 처리 가능     |
| 성능 최적화       | 배열은 스택에 저장되므로 빠르고 캐시 친화적         |
| 안전성            | 컴파일 타임에 타입과 크기 모두 검사됨              |
| 재사용성          | 제네릭 함수/구조체로 코드 중복 없이 다양한 상황 대응 |


## ⚠️ 주의할 점

| 항목               | 설명                                      |
|--------------------|-------------------------------------------|
| `[T; N]`의 타입 특성 | 배열 크기 `N`이 타입에 포함됨 → 트레잇 구현 제한 |
| 슬라이스 변환       | `[T; N]`은 `&[T]`로 자동 변환 가능            |
| 반대 변환 불가      | `&[T]` → `[T; N]`은 크기 정보 없어서 불가능     |
| 제네릭 추론 어려움  | `[T; N]`은 타입 추론이 복잡해질 수 있음        |
| 트레잇 구현 제약    | `[T; N]`에 대해 일반적인 트레잇 구현이 어려움   |


## 📦 구조체에 적용 예시
```rust
struct Buffer<T, const N: usize> {
    data: [T; N],
}

impl<T: Copy + Default, const N: usize> Buffer<T, N> {
    fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}
```

- CAD 시스템에서 PointBuffer, ColorTable, MatrixStorage 같은 구조에 유용하게 쓰일 수 있음.

## 🧠 결론
제네릭과 배열을 함께 쓰면 타입 유연성과 성능을 동시에 잡을 수 있지만,
배열 크기 N이 타입에 포함된다는 점을 고려해서 const generics를 적극 활용하는 게 핵심입니다.


--- 

# 실전 예제
제네릭과 배열을 활용해서 Point<T>와 Matrix<T, const N: usize> 같은 구조를 실제로 어떻게 설계.

## 🧭 Point<T> 구조: 좌표 표현
```rust
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub coords: [T; 2], // 2D 좌표
}

impl<T: Copy + std::ops::Add<Output = T>> Point<T> {
    pub fn add(&self, other: &Self) -> Self {
        Point {
            coords: [
                self.coords[0] + other.coords[0],
                self.coords[1] + other.coords[1],
            ],
        }
    }
}
```

## ✅ 특징
- T는 f64, i32, u8 등 다양한 수치 타입으로 사용 가능
- [T; 2] 배열로 고정된 2D 좌표를 표현
- 연산 메서드(add)는 제네릭 타입에 트레잇 제약을 걸어 구현

## 🧮 Matrix<T, const N: usize> 구조: 정사각 행렬
```rust
#[derive(Debug, Clone)]
pub struct Matrix<T, const N: usize> {
    pub data: [[T; N]; N], // N x N 행렬
}

impl<T: Copy + Default + std::ops::Add<Output = T>, const N: usize> Matrix<T, N> {
    pub fn new() -> Self {
        Self {
            data: [[T::default(); N]; N],
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

```

## ✅ 특징
- const N: usize로 크기를 제네릭하게 설정 → Matrix<f64, 3> 같은 타입 가능
- [[T; N]; N]으로 고정된 2차원 배열 표현
- new()는 Default를 활용해 초기화
- add()는 요소별 덧셈을 수행

## 📌 사용 예시
```rust
fn main() {
    let p1 = Point { coords: [1.0, 2.0] };
    let p2 = Point { coords: [3.0, 4.0] };
    let p3 = p1.add(&p2);
    println!("Point: {:?}", p3);

    let m1 = Matrix::<f64, 2>::new();
    let m2 = Matrix::<f64, 2>::new();
    let m3 = m1.add(&m2);
    println!("Matrix: {:?}", m3);
}
```


## 🧠 확장 아이디어
- Point<T, const N: usize>로 다차원 좌표도 표현 가능
- Matrix<T, const ROW: usize, const COL: usize>로 비정사각 행렬도 구현 가능
- Transform<T> 구조에 Matrix<T, 3>를 넣어 CAD 변환 행렬로 활용 가능

---

# 다차원 좌표와 일반 행렬 구조를 확장

Point<T, const N: usize>와 Matrix<T, const ROW: usize, const COL: usize>를 활용해 다차원 좌표와 일반 행렬 구조를 확장


## 🧭 Point<T, const N: usize>: 다차원 좌표 표현
```rust
#[derive(Debug, Clone, Copy)]
pub struct Point<T, const N: usize> {
    pub coords: [T; N],
}

impl<T: Copy + std::ops::Add<Output = T>, const N: usize> Point<T, N> {
    pub fn add(&self, other: &Self) -> Self {
        let mut result = [self.coords[0]; N];
        for i in 0..N {
            result[i] = self.coords[i] + other.coords[i];
        }
        Point { coords: result }
    }
}
```

### ✅ 특징
- N 차원의 좌표를 표현 가능 → 2D, 3D, 4D 등 자유롭게 확장
- T는 f64, i32, u8 등 수치 타입으로 유연하게 사용 가능
- add() 메서드는 요소별 덧셈을 수행

## 🧮 Matrix<T, const ROW: usize, const COL: usize>: 일반 행렬 표현
```rust
#[derive(Debug, Clone)]
pub struct Matrix<T, const ROW: usize, const COL: usize> {
    pub data: [[T; COL]; ROW],
}

impl<T: Copy + Default + std::ops::Add<Output = T>, const ROW: usize, const COL: usize> Matrix<T, ROW, COL> {
    pub fn new() -> Self {
        Self {
            data: [[T::default(); COL]; ROW],
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..ROW {
            for j in 0..COL {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}
```


### ✅ 특징
- ROW, COL을 각각 제네릭으로 받아서 정사각/직사각 행렬 모두 표현 가능
- new()는 Default를 활용해 초기화
- add()는 요소별 덧셈을 수행

## 📌 사용 예시
```rust
fn main() {
    let p1 = Point::<f64, 3> { coords: [1.0, 2.0, 3.0] };
    let p2 = Point::<f64, 3> { coords: [4.0, 5.0, 6.0] };
    let p3 = p1.add(&p2);
    println!("Point3D: {:?}", p3);

    let m1 = Matrix::<f64, 2, 3>::new();
    let m2 = Matrix::<f64, 2, 3>::new();
    let m3 = m1.add(&m2);
    println!("Matrix 2x3: {:?}", m3);
}
```


## 🧠 확장 아이디어
- Transform<T>에 Matrix<T, 3, 3>을 넣어 2D 변환 행렬로 활용
- Point<T, N>에 Dot, Norm, Distance 같은 메서드 추가 가능
- Matrix<T, R, C>에 transpose(), mul() 등 연산 확장 가능

---

# jagged array

Rust에서는 전통적인 의미의 jagged array (불균형 배열)—즉, 각 행의 길이가 다른 2차원 배열—을 정적 배열 [T; N]으로는 직접 표현할 수 없습니다.  
하지만 Rust는 동적 컬렉션과 제네릭을 활용해서 jagged 구조를 충분히 구현할 수 있어요.

## 🧱 기본적으로 불가능한 구조
```rust
// ❌ 불가능: 각 행의 길이가 다른 배열
let jagged = [
    [1, 2],        // 길이 2
    [3, 4, 5],     // 길이 3
];

```
Rust의 [T; N]은 모든 행의 길이가 동일해야 하며, [[T; M]; N]처럼 정적 크기를 요구합니다.

## ✅ 가능하게 만드는 방법: Vec<Vec<T>>
```rust
let jagged: Vec<Vec<i32>> = vec![
    vec![1, 2],
    vec![3, 4, 5],
    vec![6],
];
```

- 각 Vec<T>는 독립적으로 길이를 가질 수 있어 jagged 구조를 표현 가능
- 동적 메모리 할당이 필요하지만, 유연성과 표현력은 충분

## 🔍 고급 대안: JaggedArray 라이브러리
Rust 생태계에는 jagged 구조를 더 효율적으로 다루기 위한 라이브러리도 있어요:
- : 고성능, 캐시 친화적 jagged 배열 표현
- : 불변 구조 기반으로 메모리 최적화된 jagged 배열 구현
이런 라이브러리는 특히 불변 데이터, 읽기 중심 처리, 캐시 최적화가 필요한 경우에 유용합니다.

## 🧠 결론
Rust에서는 [T; N]으로는 jagged array를 직접 표현할 수 없지만,
Vec<Vec<T>> 또는 전용 라이브러리를 통해 동적이고 유연한 jagged 구조를 안전하게 구현할 수 있습니다.
CAD 시스템에서 곡선 리스트나 UI 요소처럼 길이가 가변적인 데이터를 다룰 때 jagged 구조가 필요할 수 있어요.


----

# 실전 예제

## Matrix에 응용
```rust

pub const DET_EPSILON: f64 = 1e-12;

pub type Matrix2x2 = [[f64; 2]; 2];
pub type Matrix3x3 = [[f64; 3]; 3];
pub type Matrix4x4 = [[f64; 4]; 4];


pub fn determinant2(m: &Matrix2x2) -> f64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

pub fn determinant3(m: &Matrix3x3) -> f64 {
    let a = m[0][0]; let b = m[0][1]; let c = m[0][2];
    let d = m[1][0]; let e = m[1][1]; let f = m[1][2];
    let g = m[2][0]; let h = m[2][1]; let i = m[2][2];

    a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
}

pub fn inverse3(m: &Matrix3x3) -> Option<Matrix4x4> {

    let det = determinant3(m);
    if det.abs() < DET_EPSILON {
        return None;
    }

    let mut inv = [[0.0; 4]; 4];
    inv[0][0] = (m[1][1] * m[2][2] - m[1][2] * m[2][1]) / det;
    inv[0][1] = -(m[0][1] * m[2][2] - m[0][2] * m[2][1]) / det;
    inv[0][2] = (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / det;

    inv[1][0] = -(m[1][0] * m[2][2] - m[1][2] * m[2][0]) / det;
    inv[1][1] = (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / det;
    inv[1][2] = -(m[0][0] * m[1][2] - m[1][0] * m[0][2]) / det;

    inv[2][0] = (m[1][0] * m[2][1] - m[1][1] * m[2][0]) / det;
    inv[2][1] = -(m[0][0] * m[2][1] - m[0][1] * m[2][0]) / det;
    inv[2][2] = (m[0][0] * m[1][1] - m[0][1] * m[1][0]) / det;

    inv[3][3] = 1.0;

    Some(inv)
}



pub fn build_minor3x3(source: &Matrix4x4, skip_row: usize, skip_col: usize) -> Matrix3x3 {
    let mut minor = [[0.0; 3]; 3];
    let mut r_dst = 0;

    for r in 0..4 {
        if r == skip_row { continue; }
        let mut c_dst = 0;
        for c in 0..4 {
            if c == skip_col { continue; }
            minor[r_dst][c_dst] = source[r][c];
            c_dst += 1;
        }
        r_dst += 1;
    }

    minor
}

pub fn determinant4(m: &Matrix4x4) -> f64 {
    let mut det = 0.0;
    let mut sign = 1.0;
    for c in 0..4 {
        let minor = build_minor3x3(m, 0, c);
        det += sign * m[0][c] * determinant3(&minor);
        sign = -sign;
    }
    det
}

pub fn inverse4(m: &Matrix4x4) -> Option<Matrix4x4> {
    let det = determinant4(m);
    if det.abs() < DET_EPSILON {
        return None;
    }
    let mut inv = [[0.0; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            let minor = build_minor3x3(m, r, c);
            let mut cof = determinant3(&minor);
            if (r + c) % 2 == 1 {
                cof = -cof;
            }
            inv[c][r] = cof / det;
        }
    }
    Some(inv)
}

pub fn inv3_block_from4(m: &Matrix4x4) -> Option<Matrix4x4> {
    let m3 = [
        [m[0][0], m[0][1], m[0][2]],
        [m[1][0], m[1][1], m[1][2]],
        [m[2][0], m[2][1], m[2][2]],
    ];
    let inv3m = inverse3(&m3)?;
    Some(inv3m)
}

```

```rust
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- 주석으로 rustfmt가 줄바꿈을 추가합니다.
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("행렬: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("전치행렬: {:#?}", transposed);
}
```

