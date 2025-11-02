# Vec resize / clone_from / extend_from_slice / copy_from_slice

Rust의 `Vec<T>` 는 매우 강력한 동적 배열 타입이고, `resize`, `clone_from`, `extend_from_slice`, `copy_from_slice`  
는 모두 벡터의 `크기` 나 `내용을 조작` 할 때 자주 쓰이는 메서드입니다.  
각각의 역할과 차이점, 그리고 샘플 코드를 아래에 정리.  

## 🧩 1. `Vec::resize`
### 📌 설명
- 벡터의 길이를 지정한 크기로 늘리거나 줄입니다.
- 늘릴 경우, 새 요소는 지정한 값으로 복제(Clone) 되어 채워집니다.
- 줄일 경우, 뒤에서부터 요소를 제거합니다.

### 📘 시그니처
```rust
fn resize(&mut self, new_len: usize, value: T)
where
    T: Clone
```

### ✅ 예제
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.resize(5, 0); // [1, 2, 3, 0, 0]
    println!("{:?}", v);

    v.resize(2, 0); // [1, 2]
    println!("{:?}", v);
}
```

## 🧩 2. `Vec::clone_from`
### 📌 설명
- 현재 벡터의 내용을 다른 벡터로 교체합니다.
- 기존 `메모리를 재사용` 하려고 시도하므로, `성능상 이점` 이 있을 수 있습니다.
- `Clone 트레잇` 이 필요합니다.

### 📘 시그니처
```rust
fn clone_from(&mut self, source: &Self)
```

### ✅ 예제
```rust
fn main() {
    let mut a = vec![1, 2, 3];
    let b = vec![10, 20];
    a.clone_from(&b); // a becomes [10, 20]
    println!("{:?}", a);
}
```


## 🧩 3.` Vec::extend_from_slice`
### 📌 설명
- 슬라이스(&[T])의 내용을 벡터에 뒤에 추가합니다.
- 벡터의 길이가 늘어납니다.
- `T: Clone` 이 필요합니다.
### 📘 시그니처
```rust
fn extend_from_slice(&mut self, other: &[T])
```

### ✅ 예제
```rust
fn main() {
    let mut v = vec![1, 2];
    let slice = &[3, 4, 5];
    v.extend_from_slice(slice); // [1, 2, 3, 4, 5]
    println!("{:?}", v);
}
```


## 🧩 4. `copy_from_slice`
### 📌 설명
- 슬라이스의 내용을 다른 슬라이스에 복사합니다.
- `길이가 정확히 같아` 야 하며, 벡터의 길이를 미리 맞춰야 합니다.
- T: Copy가 필요합니다.
### 📘 시그니처
```rust
fn copy_from_slice(&mut self, src: &[T])
```

### ✅ 예제
```rust
fn main() {
    let mut v = vec![0; 3]; // [0, 0, 0]
    let src = [10, 20, 30];
    v.copy_from_slice(&src); // [10, 20, 30]
    println!("{:?}", v);
}
```

## 🧠 요약 비교

| 메서드              | 목적                     | 길이 변경 | 요구 조건   | 비고               |
|---------------------|--------------------------|------------|-------------|--------------------|
| resize              | 길이 조정 (늘리기/줄이기) | ✅         | T: Clone    | 새 값으로 채움     |
| clone_from          | 다른 Vec로 교체          | ✅         | T: Clone    | 메모리 재사용 가능 |
| extend_from_slice   | 슬라이스를 뒤에 추가     | ✅         | T: Clone    | append 방식        |
| copy_from_slice     | 슬라이스를 덮어쓰기      | ❌         | T: Copy     | 길이 정확히 같아야 함 |

---


 특정 Range 복사

이 함수는 Vec<f64> 타입의 필드 knots를 가진 구조체에서, **다른 인스턴스의 특정 범위(range)** 를 복사해오는 기능을 구현한 것입니다.  
아래에 단계별로 설명.


## 소스 코드
```rust
pub fn copy_range_from(&mut self, other: &Self, range: std::ops::Range<usize>) {
    let r = range.start.min(other.knots.len())..range.end.min(other.knots.len());
    let n = r.end.saturating_sub(r.start);
    self.knots.resize(n, 0.0);
    if n > 0 {
        self.knots[..n].copy_from_slice(&other.knots[r]);
    }
}
```

## 🧠 함수 목적
```rust
pub fn copy_range_from(&mut self, other: &Self, range: Range<usize>)
```

- self.knots ← other.knots[range]의 값을 복사
- range는 복사하고 싶은 인덱스 범위 (start..end)
- 복사 대상 범위가 other.knots.len()을 초과하지 않도록 안전하게 처리

## 🔍 단계별 설명
### 1️⃣ 범위 안전화
```rust
let r = range.start.min(other.knots.len())..range.end.min(other.knots.len());
```
- range가 other.knots의 길이를 초과할 경우를 대비해 범위를 클램핑(clamp) 처리
- 예: range = 3..10, other.knots.len() = 5 → r = 3..5

### 2️⃣ 복사할 길이 계산
```rust
let n = r.end.saturating_sub(r.start);
```
- 복사할 요소 개수 계산
- saturating_sub은 음수가 되지 않도록 보장 (예: 3 - 5 = 0)

### 3️⃣ self.knots 크기 조정
```rust
self.knots.resize(n, 0.0);
```
- 복사할 만큼의 공간 확보
- 기존 self.knots의 길이를 n으로 맞춤
- 부족하면 0.0으로 채움, 넘치면 잘림

### 4️⃣ 실제 복사 수행
```rust
if n > 0 {
    self.knots[..n].copy_from_slice(&other.knots[r]);
}
```
- copy_from_slice는 동일한 길이의 슬라이스 간 복사를 수행
- self.knots[..n] ← other.knots[r]의 값으로 덮어씀

### ✅ 예시
```rust
let mut a = MyStruct { knots: vec![] };
let b = MyStruct { knots: vec![10.0, 20.0, 30.0, 40.0, 50.0] };

a.copy_range_from(&b, 1..4);
// a.knots == [20.0, 30.0, 40.0]
```

## 🧩 핵심 요약

| 함수              | 역할 설명                                      |
|-------------------|------------------------------------------------|
| `min()`             | 범위 초과 방지: 인덱스를 최대 길이로 클램핑     |
| `saturating_sub`    | 음수 방지: 범위 길이를 안전하게 계산            |
| `resize`            | 복사할 만큼 공간 확보: 부족하면 채우고 넘치면 자름 |
| `copy_from_slice`   | 슬라이스 복사: 동일 길이의 값으로 덮어쓰기      |

- 이 방식은 안전하고 효율적으로 특정 구간만 복사할 수 있어서, 특히 곡선 제어점이나 노드 배열 같은 곳에 유용.

---

