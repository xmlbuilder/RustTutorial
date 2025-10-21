# Rust의 `Vec<T>::binary_search_by_key` 가이드
Vec<T>의 `binary_search_by_key` 는 Rust에서 정렬된 벡터를 빠르게 탐색할 수 있는 매우 강력한 함수.  
아래에 자세한 설명과 함께 관련 함수들, 그리고 실전 샘플 코드를 정리.

## 🔍 `binary_search_by_key` 란?
```rust
fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
where
    F: FnMut(&T) -> B,
    B: Ord,
```

- 정렬된 벡터에서 key 값을 기준으로 이진 탐색을 수행
- f는 각 요소에서 키를 추출하는 함수
- b는 찾고자 하는 키 값
- 반환값:
- `Ok(index)` → 해당 키를 가진 요소의 인덱스
- `Err(insertion_point)` → 키가 없을 경우 삽입 가능한 위치

## 🧪 예제: 사용자 정의 구조체 탐색
```rust
#[derive(Debug)]
struct Item {
    id: u32,
    name: &'static str,
}

fn main() {
    let mut items = vec![
        Item { id: 1, name: "Pen" },
        Item { id: 3, name: "Notebook" },
        Item { id: 5, name: "Eraser" },
    ];

    // id 기준으로 정렬되어 있다고 가정
    match items.binary_search_by_key(&3, |item| item.id) {
        Ok(index) => println!("Found at index {}: {:?}", index, items[index]),
        Err(pos) => println!("Not found, can insert at index {}", pos),
    }
}
```
### 🔸 출력
```
Found at index 1: Item { id: 3, name: "Notebook" }
```

## 🧩 관련 함수 비교

| 함수 이름              | 설명                          | 요구 조건            |
|------------------------|-------------------------------|----------------------|
| `binary_search`          | 전체 값으로 탐색              | T: Ord               |
| `binary_search_by`       | 커스텀 비교 함수로 탐색       | Fn(&T) -> Ordering   |
| `binary_search_by_key`   | 키 추출 함수로 탐색           | Fn(&T) -> K, K: Ord  |

## 🔁 예시 비교
```rust
let nums = vec![10, 20, 30, 40];

// binary_search
assert_eq!(nums.binary_search(&30), Ok(2));

// binary_search_by
assert_eq!(nums.binary_search_by(|x| x.cmp(&25)), Err(2));

// binary_search_by_key
assert_eq!(nums.binary_search_by_key(&30, |x| *x), Ok(2));
```

## 🛠️ 실전 활용: 정렬된 구조체에서 빠른 조회
예를 들어 CAD 시스템에서 Coefficient { pos, val } 구조체를 pos 기준으로 정렬해두고,  
특정 열 인덱스를 빠르게 찾을 수 있습니다.

```rust:
let coeffs = vec![
    Coefficient { pos: 0, val: 1.0 },
    Coefficient { pos: 2, val: -3.0 },
    Coefficient { pos: 5, val: 2.5 },
];

if let Ok(i) = coeffs.binary_search_by_key(&2, |c| c.pos) {
    println!("Found: {:?}", coeffs[i]);
}
```


## ✅ 요약
- `binary_search_by_key` 는 정렬된 벡터에서 특정 키를 빠르게 찾는 데 최적
- 사용자 정의 타입에서도 키 추출 함수로 유연하게 탐색 가능
- CAD, 게임, 데이터베이스 등에서 빠른 조회가 필요한 경우 매우 유용

---

✅ binary_search_by가 지원되는 컬렉션
| 컬렉션 타입        | 지원 여부 | 사용 예시 또는 대안                  |
|--------------------|-----------|--------------------------------------|
| Vec<T>             | ✅ 지원   | vec.binary_search_by(...)            |
| &[T] (슬라이스)    | ✅ 지원   | slice.binary_search_by(...)          |
| BTreeMap<K, V>     | ❌ 직접 지원 안 함 | get, range, lower_bound 등으로 대체 가능 |
| HashMap<K, V>      | ❌ 지원 안 함 | 정렬되지 않으므로 이진 탐색 불가     |
| LinkedList<T>      | ❌ 지원 안 함 | 순차 탐색만 가능                     |
| BinaryHeap<T>      | ❌ 지원 안 함 | 힙 구조라 정렬 순서가 아님           |



## 🧪 예시: 슬라이스에서 binary_search_by
```rust
fn main() {
    let data = [10, 20, 30, 40, 50]; // 배열
    let slice = &data[..];          // 슬라이스로 변환

    let result = slice.binary_search_by(|x| x.cmp(&35)); // ✅ OK

    match result {
        Ok(i) => println!("Found at index {}", i),
        Err(i) => println!("Not found, insert at index {}", i),
    }
}
```

Rust 1.55 이후부터는 배열 타입 [T; N]에도 슬라이스 메서드들이 자동으로 구현되도록 개선되었기 때문에,  
binary_search_by를 배열에 직접 호출해도 작동합니다.

## 🧠 왜 되는 걸까?
Rust에서는 배열 [T; N]에 대해 자동으로 Deref 트레이트를 통해 슬라이스 &[T]로 변환되도록 구현되어 있습니다.  
그래서 다음과 같은 코드도 컴파일되고 실행됩니다:
```rust
let data = [10, 20, 30, 40, 50]; // 배열
let result = data.binary_search_by(|x| x.cmp(&35)); // ✅ 작동함
```

Rust 내부적으로는 이 호출이 다음과 같이 처리됩니다:  
```rust
(&data[..]).binary_search_by(...)
```
- 즉, 배열이 슬라이스처럼 동작하도록 자동 변환.

## ✅ 요약
| 항목                | 설명                                      | 예시 또는 조건                     |
|---------------------|-------------------------------------------|------------------------------------|
| binary_search_by    | 정렬된 컬렉션에서 커스텀 비교로 탐색      | Fn(&T) → Ordering                  |
| 사용 대상           | 슬라이스 &[T], Vec<T>에서 사용 가능       | data.binary_search_by(...)         |
| 반환값              | Ok(index) 또는 Err(insertion_point)       | 탐색 성공/실패 위치 반환           |
| 관련 함수           | binary_search, binary_search_by_key       | 전체 값 또는 키 기반 탐색          |

---

## 🧠 다른 컬렉션에서 비슷한 기능을 원한다면?
- `BTreeMap` 에서 범위 기반 탐색을 하고 싶다면:  
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(1, "a");
map.insert(3, "b");
map.insert(5, "c");

// 가장 가까운 키 이상을 찾고 싶을 때
if let Some((k, v)) = map.range(2..).next() {
    println!("First key ≥ 2: {} → {}", k, v);
}
```
- `VecDeque`, `LinkedList`, `HashMap` 등은 정렬된 구조가 아니므로 이진 탐색은 불가능합니다.

## 🔧 커스텀 컬렉션에서 사용하고 싶다면?
직접 Vec<T>를 내부에 두고 정렬을 유지하면서 binary_search_by를 래핑하는 방식으로 구현할 수 있음:
```rust
struct SortedList<T> {
    data: Vec<T>,
}

impl<T: Ord> SortedList<T> {
    fn insert(&mut self, value: T) {
        match self.data.binary_search(&value) {
            Ok(_) => {} // 중복 처리
            Err(i) => self.data.insert(i, value),
        }
    }
}
```

## ✅ 요약
- binary_search_by는 정렬된 슬라이스나 Vec에서만 사용 가능
- BTreeMap은 범위 기반 탐색으로 유사 기능 제공
- 다른 컬렉션에서는 직접 정렬 구조를 유지하거나 커스텀 구현 필요
