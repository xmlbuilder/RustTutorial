# 🧠 Vector란?

Rust에서 Vec<T>는 동적 배열을 의미합니다.
- 고정 길이인 배열([T; N])과 달리, Vec은 크기를 자유롭게 늘릴 수 있는 구조입니다.
- 모든 요소는 동일한 타입이어야 하며, 힙(heap)에 저장됩니다.
- Rust의 안전성 철학에 따라, 벡터는 소유권, 불변/가변 참조, 옵션 타입 등과 잘 연동됩니다.

## 🔧 Vec<T> 주요 기능 요약
| 기능 항목         | 사용 예시 및 설명                                      |
|------------------|--------------------------------------------------------|
| 생성             | `Vec::new()`, `vec![...]`                              |
| 요소 추가        | `.push(value)`                                         |
| 요소 접근        | `v[i]`, `v.get(i)` → `Option<T>` 반환                  |
| 반복 처리        | `for x in &v`, `for x in &mut v`                       |
| 요소 제거        | `.pop()`, `.remove(index)`, `.clear()`                 |
| 길이 확인        | `.len()`                                               |
| 정렬 및 반전     | `.sort()`, `.reverse()`                                |

## 🧠 초기값과 타입 명시
```rust
let v1 = vec![1, 2, 3];                  // 타입 추론
let v2: Vec<f64> = Vec::new();          // 명시적 타입
let v3 = vec![String::from("Hi"), String::from("Rust")]; // 문자열 벡터
```

## 🧪 샘플 코드
### 1. 벡터 생성과 출력
```rust
fn main() {

    let v1: Vec<i32> = Vec::new(); // 빈 벡터
    let v2 = vec![10, 20, 30];     // 초기값 포함 벡터

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}

```

### 2. 요소 추가 및 접근
``` rust
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("전체 벡터: {:?}", v);
    println!("두 번째 요소: {}", v[1]);
    println!("세 번째 요소 (get): {:?}", v.get(2));
}
```

### 3. 반복문으로 요소 출력
```rust
fn main() {
    let v = vec![100, 200, 300];
    for val in &v {
        println!("값: {}", val);
    }
}
```

### 4. 요소 수정
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for val in &mut v {
        *val += 10;
    }
    println!("수정된 벡터: {:?}", v);
}
```

### 5. 중간 삽입/삭제
``` rust
let mut v = vec![1, 2, 4];
v.insert(2, 3); // 2번 인덱스에 3 삽입 → [1, 2, 3, 4]
v.remove(1);    // 1번 인덱스 제거 → [1, 3, 4]
```

### 6.  고차 함수와 체이닝
```rust
let v = vec![1, 2, 3, 4, 5];
let squared_even: Vec<i32> = v
    .into_iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();

println!("{:?}", squared_even); // 출력: [4, 16]
```




### 7. v.extend(&other_vec)
다른 벡터의 요소를 v에 추가합니다.
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let other_vec = vec![4, 5];

    v.extend(&other_vec); // v = [1, 2, 3, 4, 5]
    println!("{:?}", v);
}
```

- &other_vec는 슬라이스로 받아들이므로 Vec, 배열, 슬라이스 모두 확장 가능
- 내부적으로 push를 반복하는 것보다 훨씬 효율적

### 8. for x in v.drain(..)
Vec의 요소를 꺼내면서 비웁니다 (소유권 이동).
```rust
fn main() {
    let mut v = vec![10, 20, 30];
    for x in v.drain(..) {
        println!("Drained: {}", x);
    }
    println!("After drain: {:?}", v); // v = []
}
```
- drain(..)은 Vec의 요소를 반복자 형태로 반환하면서 제거
- for x in v처럼 보이지만, v는 비워지고 x는 move됨

### 9. as_slice()
Vec을 슬라이스(&[T])로 변환합니다.
```rust
fn main() {
    let v = vec![7, 8, 9];
    let slice: &[i32] = v.as_slice();

    println!("{:?}", slice); // [7, 8, 9]
}
```
- &v[..]와 거의 동일한 효과
- 함수 인자로 &[T]를 요구할 때 유용

### 10. 안전한 접근 vs 위험한 접근
```rust
fn main() {
    let v = vec![1, 2, 3];

    // 안전하지 않은 접근 (panic 가능)
    // println!("{}", v[10]); // 주석 해제 시 런타임 에러

    // 안전한 접근
    match v.get(10) {
        Some(val) => println!("값: {}", val),
        None => println!("해당 인덱스에 값이 없습니다."),
    }
}
```

## ✅ 요약 포인트
| 기능             | 메서드/설명                         |
|------------------|-------------------------------------|
| 생성             | `Vec::new()`, `vec![...]`           |
| 추가/삭제        | `push()`, `pop()`, `insert()`, `remove()` |
| 접근             | `v[i]`, `v.get(i)`                  |
| 반복             | `for`, `.iter()`, `.into_iter()`    |
| 변환/필터        | `map()`, `filter()`, `collect()`     |

## ✅ 요약 확장
| 메서드        | 동작 또는 목적           | 대상 또는 반환 타입 | 비고 또는 특징                     |
|---------------|--------------------------|----------------------|------------------------------------|
| `extend()`     | 다른 컬렉션을 뒤에 추가     | `Vec`                | `Vec`, 슬라이스, 배열 모두 확장 가능 |
| `drain(..)`    | 요소를 꺼내면서 제거       | `Vec` → 반복자        | 소유권 이동, 원본 `Vec`은 비워짐     |
| `as_slice()`   | 슬라이스로 참조           | `&[T]` (`&v[..]`)     | `Vec`을 슬라이스로 변환             |


## 🧩 팁
- Vec<T>는 Clone, Debug, IntoIterator 등 다양한 트레잇을 구현하고 있어 매우 유연합니다.
- 벡터는 스코프를 벗어나면 자동으로 메모리가 해제됩니다 (RAII 원칙).
- VecDeque, LinkedList 등 다른 컬렉션과 비교해도 성능과 사용성이 뛰어납니다.

---

## ⚠️ 함수에 벡터를 넘길 때 생기는 주요 소유권 문제

### 1. 소유권 이동 (Move)
```rust
fn consume(v: Vec<i32>) {
    println!("{:?}", v);
}

fn main() {
    let vec = vec![1, 2, 3];
    consume(vec); // vec의 소유권이 함수로 이동
    // println!("{:?}", vec); // ❌ 에러: vec는 더 이상 유효하지 않음
}
```

- Vec<T>는 Copy가 아닌 타입이므로 함수에 값으로 넘기면 소유권이 이동됩니다.
- 이후 원래 변수는 사용 불가.

### 2. 불변 참조로 넘기기 (&Vec<T>)
```rust
fn read(v: &Vec<i32>) {
    println!("읽기: {:?}", v);
}

fn main() {
    let vec = vec![1, 2, 3];
    read(&vec); // ✅ 소유권 유지
    println!("원본 사용 가능: {:?}", vec);
}
```

- 함수에 불변 참조로 넘기면 소유권은 유지되고, 원본도 계속 사용 가능.
- 단, 함수 내에서 수정은 불가능.

### 3. 가변 참조로 넘기기 (&mut Vec<T>)
```rust
fn modify(v: &mut Vec<i32>) {
    v.push(4);
}

fn main() {
    let mut vec = vec![1, 2, 3];
    modify(&mut vec); // ✅ 가변 참조로 수정 가능
    println!("수정된 벡터: {:?}", vec);
}
```

- 가변 참조는 단 하나만 허용되며, 그동안 원본이나 다른 참조는 접근 불가.
- Rust의 Borrow Checker가 이를 강력하게 검사해줍니다.

### 4. 복제해서 넘기기 (vec.clone())
```rust
fn consume(v: Vec<i32>) {
    println!("소비: {:?}", v);
}

fn main() {
    let vec = vec![1, 2, 3];
    consume(vec.clone()); // ✅ 복제본을 넘기므로 원본 사용 가능
    println!("원본 유지: {:?}", vec);
}
```

- .clone()을 사용하면 깊은 복사가 일어나고, 원본은 그대로 유지됩니다.
- 단점은 성능 비용이 발생할 수 있다는 점.


## 🧠 Vec<T> 함수 전달 방식 핵심 요약

| 전달 방식         | 소유권 이동 | 원본 사용 가능 | 수정 가능 | 성능 영향 | 사용 예시                  |
|------------------|-------------|----------------|------------|------------|----------------------------|
| `Vec<T>`         | ✅          | ❌             | ✅         | 없음       | `fn consume(v: Vec<T>)`    |
| `&Vec<T>`        | ❌          | ✅             | ❌         | 없음       | `fn read(v: &Vec<T>)`      |
| `&mut Vec<T>`    | ❌          | ✅             | ✅         | 없음       | `fn modify(v: &mut Vec<T>)`|
| `vec.clone()`    | ❌          | ✅             | ✅         | ✅ (복사 비용) | `fn consume(v: Vec<T>)` with `vec.clone()` |

## 🧠 내부 구조 요약
- Vec<T>는 내부적으로 RawVec<T>를 사용해 메모리 관리
- 메모리 부족 시 2배 확장하며 재할당
- len, capacity, ptr 정보를 갖고 있음

---

# 🧱 배열(array)이란?
Rust의 배열은 고정 크기의 컬렉션입니다.
- 모든 요소는 같은 타입이어야 하며,
- 크기는 컴파일 타임에 결정됩니다.
- 메모리는 **스택(stack)**에 할당됩니다.
## ✅ 선언 방법
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5]; // 타입 추론
    let arr2: [i32; 3] = [10, 20, 30]; // 명시적 타입과 크기
    let arr3 = [0; 5]; // 0으로 초기화된 5개 요소
}
```

## 🔍 주요 특징
- 크기 변경 불가
- 빠른 접근 속도
- 메모리 효율적
- 반복문과 슬라이스에 적합


## 📦 배열과 벡터의 차이점

| 항목             | `[T; N]` (배열)                        | `Vec<T>` (벡터)                          |
|------------------|----------------------------------------|------------------------------------------|
| 크기             | 고정 (컴파일 타임에 결정됨)            | 가변 (런타임에 변경 가능)                |
| 메모리 위치      | 스택                                   | 힙                                       |
| 선언 방식        | `[1, 2, 3]`, `[0; 5]`                   | `vec![1, 2, 3]`, `Vec::new()`            |
| 접근 방식        | `arr[i]`                                | `vec[i]`, `vec.get(i)`                   |
| 수정 방식        | `mut`로 선언 후 직접 인덱스 수정        | `mut`로 선언 후 `.push()`, `.pop()` 등   |
| 반복 처리        | `for val in arr.iter()`                | `for val in vec.iter()` 또는 `.into_iter()` |
| 주요 용도        | 고정된 크기의 빠른 데이터 처리          | 동적 리스트, 유연한 데이터 조작          |



## 🧪 배열 사용 예제
### 1. 배열 선언과 출력
```rust
fn main() {
    let arr = [10, 20, 30];
    println!("배열: {:?}", arr);
}
```

### 2. 배열 요소 접근
```rust
fn main() {
    let arr = [1, 2, 3, 4];
    println!("첫 번째 요소: {}", arr[0]);
    println!("세 번째 요소: {}", arr[2]);
}
```

### 3. 배열 반복문
```rust
fn main() {
    let arr = [100, 200, 300];
    for val in arr.iter() {
        println!("값: {}", val);
    }
}
```

### 4. 슬라이스로 부분 접근
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // 2, 3, 4
    println!("슬라이스: {:?}", slice);
}
```


## 🧠 언제 배열을 쓰면 좋을까?
- 크기가 변하지 않는 데이터를 다룰 때
- 성능이 중요한 경우 (스택 메모리)
- 임베디드 시스템이나 고정된 설정값 처리 시
---

