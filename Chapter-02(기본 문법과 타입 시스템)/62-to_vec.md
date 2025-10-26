# to_vec()
to_vec()는 Rust에서 슬라이스나 이터레이터를 `Vec<T>` 로 `복사` 해 `소유권을 갖는 벡터` 로 만드는 메서드입니다.  
이 메서드는 안전하고 직관적인 방식으로 데이터를 복제하여 가변 벡터로 활용할 수 있게 해줍니다.

## 🧠 to_vec()의 핵심 개념
### ✅ 정의와 목적
- to_vec()은 슬라이스(&[T])나 이터레이터에서 호출할 수 있는 메서드로, `새로운 Vec<T>` 를 생성합니다.
- 생성된 벡터는 힙에 저장되며 소유권을 가짐 → 원본과 독립적으로 수정 가능
```rust
fn main() {
    let arr = [1, 2, 3];
    let slice = &arr[1..];
    let vec = slice.to_vec(); // vec = [2, 3]
}
```

## 🔍 사용 대상 요약

| 원본 타입           | 변환 방식 또는 결과                      |
|---------------------|------------------------------------------|
| `&[T]`              | `.to_vec()` → `Vec<T>`                   |
| `&mut [T]`          | `.to_vec()` → `Vec<T>`                   |
| `Iterator<Item=T>`  | `.collect::<Vec<T>>()` → `Vec<T>`        |
| `str`               | `.as_bytes().to_vec()` → `Vec<u8>`       |
| `String`            | `.into_bytes()` → `Vec<u8>`              |
| `Vec<T>`            | `.clone()` 또는 `to_vec()` → 복제본 생성 |


## ✅ 관련 함수 및 메서드 요약

| 함수/메서드              | 반환 또는 변환 결과                                |
|--------------------------|----------------------------------------------------|
| `to_vec()`               | `Vec<T>` — 슬라이스나 이터레이터를 복사하여 벡터 생성 |
| `to_vec_result()`        | `Result<Vec<T>, E>` — `Result` 이터레이터를 수집     |
| `to_set()`               | `HashSet<T>` — 중복 없는 집합으로 변환               |
| `to_map()`               | `HashMap<K, V>` — 키-값 쌍으로 구성된 맵 생성         |
| `Vec::from(slice)`       | `Vec<T>` — 슬라이스를 벡터로 변환하는 표준 방식       |
| `collect::<Vec<T>>()`    | `Vec<T>` — 이터레이터를 벡터로 수집                  |


## ✨ 예시들
### 🔹 슬라이스 → Vec
```rust
let slice = &[10, 20, 30];
let vec = slice.to_vec(); // vec = [10, 20, 30]
```

### 🔹 이터레이터 → Vec
```rust
let vec = "one two three".split_whitespace().to_vec();
// vec = ["one", "two", "three"]
```

### 🔹 Result 이터레이터 → Vec
```rust
let hex = ["23E", "5F5", "FF00"];
let numbers = hex.iter()
    .map(|s| u32::from_str_radix(s, 16))
    .collect::<Result<Vec<_>, _>>()?;
```


## 🧠 장점
- 소유권 확보: 원본 슬라이스와 독립적으로 사용 가능
- 가변성 확보: Vec<T>는 push, pop, insert 등 다양한 수정 가능
- 안전한 복사: Rust의 메모리 모델에 따라 안전하게 힙에 복사됨

## ⚠️ 주의사항
- to_vec()는 힙에 새 메모리를 할당하므로, 큰 슬라이스를 복사할 때는 성능 고려 필요
- clone()이 필요한 타입에서는 T: Clone 제약이 있음

---



