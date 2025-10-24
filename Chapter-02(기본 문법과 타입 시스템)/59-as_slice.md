# as_slice
Rust에서 Vec<T> 타입의 모든 데이터를 `슬라이스(slice)` 로 얻으려면 `.as_slice()` 또는 `&vec[..]` 문법을 사용할 수 있습니다.  
아래는 두 가지 방법입니다:
## ✅ 방법 1: as_slice() 메서드 사용
```rust
let vec = vec![1, 2, 3, 4, 5];
let slice: &[i32] = vec.as_slice();
```

## ✅ 방법 2: 슬라이스 문법 사용
```rust
let vec = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &vec[..];
```

두 방법 모두 Vec<T>를 &[T]로 변환하며, 읽기 전용 슬라이스를 제공합니다.  
만약 가변 슬라이스가 필요하다면 &mut vec[..]를 사용할 수 있습니다.  
필요에 따라 슬라이스를 함수에 넘기거나, 부분 슬라이스도 쉽게 만들 수 있음.  
예를 들어 &vec[1..4]는 두 번째부터 네 번째 요소까지를 포함하는 슬라이스입니다.

---
# 상세 설명

`as_slice()` 는 Rust에서 Vec<T>의 모든 요소를 슬라이스(&[T])로 참조할 수 있게 해주는 메서드입니다.  
슬라이스는 벡터의 데이터를 복사하지 않고 읽기 전용으로 접근할 수 있게 해주기 때문에 성능과 안전성 면에서 매우 유용합니다.

## 🧠 as_slice()란?
- Vec<T>는 힙에 저장된 가변 길이 배열입니다.
- as_slice()는 Vec<T>의 내부 데이터를 불변 슬라이스(&[T])로 반환합니다.
- 슬라이스는 복사본이 아니라 참조이므로 메모리 효율적입니다.

```rust
fn main() {
    let vec = vec![10, 20, 30, 40];
    let slice = vec.as_slice(); // &[i32]
    
    println!("{:?}", slice); // 출력: [10, 20, 30, 40]
}
```


## 🧪 실전 예제 1: 슬라이스를 함수에 넘기기
```rust
fn print_slice(data: &[i32]) {
    for val in data {
        println!("값: {}", val);
    }
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let slice = vec.as_slice();
    print_slice(slice);
}
```

- 🔍 이 예제에서는 Vec<i32>를 슬라이스로 변환하여 함수에 전달합니다. 함수는 슬라이스를 읽기 전용으로 처리합니다.

## 🧪 실전 예제 2: 슬라이스로 부분 비교 (`starts_with`, `ends_with`)
```rust
fn main() {
    let vec = vec![100, 200, 300, 400, 500];
    let slice = vec.as_slice();

    if slice.starts_with(&[100, 200]) {
        println!("앞부분이 일치합니다!");
    }

    if slice.ends_with(&[400, 500]) {
        println!("뒷부분이 일치합니다!");
    }
}
```

- ✅ `starts_with()`와 `ends_with()`는 슬라이스에서 특정 패턴을 확인할 때 유용합니다.

## 🧪 실전 예제 3: 슬라이스로 정렬된 데이터 확인
```rust
fn is_sorted(data: &[i32]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let slice = vec.as_slice();

    println!("정렬됨? {}", is_sorted(slice)); // true
}
```
- 🔧 `windows(2)` 는 슬라이스를 2개씩 묶어 순차 비교할 수 있게 해줍니다.

## 💡 요약
| 항목             | 설명                                                  |
|------------------|-------------------------------------------------------|
| as_slice()       | Vec<T>를 불변 슬라이스 &[T]로 변환하는 메서드         |
| &vec[..]         | Vec<T> 전체를 슬라이스로 참조하는 문법                |
| 반환 타입        | &[T] (복사 없이 참조만 수행하는 읽기 전용 슬라이스)   |
| 주요 용도        | 함수 인자 전달, 패턴 비교, 슬라이스 연산 등            |
| 가변 슬라이스    | as_mut_slice() 또는 &mut vec[..] 사용 가능             |



