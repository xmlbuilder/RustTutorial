# binary_search_by
- binary_search_by 는 Rust 슬라이스에서 사용자 정의 비교 함수로 이진 탐색을 할 수 있게 해주는 메서드.
- 기본 binary_search가 Ord에만 의존하는 반면, 이건 **원소 vs 내가 정의한 기준** 으로 비교를 직접 컨트롤할 수 있음.

## 1. 시그니처
```rust
impl<T> [T] {
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> std::cmp::Ordering
}
```
- self: 정렬된 슬라이스 &[T]
- f: 각 원소에 대해 Ordering을 반환하는 함수
  - Less  → **원소 < 찾는 값**
  - Greater → **원소 > 찾는 값**
  - Equal → **원소 == 찾는 값**
- 반환:
  - Ok(idx)   → idx 위치에서 찾음
  - Err(idx)  → 못 찾았고, 그 값을 끼워 넣을 인덱스가 idx

## 2. 가장 기본적인 사용 예
```rust
use std::cmp::Ordering;

let arr = [10, 20, 30, 40, 50];

let target = 30;
let res = arr.binary_search_by(|x| x.cmp(&target));

assert_eq!(res, Ok(2));
```

- 여기서 |x| x.cmp(&target)은:
  - x < target  → Ordering::Less
  - x == target → Ordering::Equal
  - x > target  → Ordering::Greater

## 3. 커스텀 키로 검색할 때 (struct, tuple 등)
- 예를 들어, (key, value) 튜플 배열에서 key 기준으로 찾고 싶을 때:
```rust
use std::cmp::Ordering;

let data = [(1, "a"), (3, "b"), (5, "c"), (7, "d")];

let key = 5;
let res = data.binary_search_by(|(k, _)| k.cmp(&key));

assert_eq!(res, Ok(2));
```

- 혹은 “슬라이스 원소는 struct인데, 그 안의 필드로 찾고 싶다” 할 때도 같은 패턴:
```rust
struct Item {
    id: i32,
    name: &'static str,
}

let items = [
    Item { id: 10, name: "foo" },
    Item { id: 20, name: "bar" },
    Item { id: 30, name: "baz" },
];

let target_id = 20;
let res = items.binary_search_by(|item| item.id.cmp(&target_id));

assert!(matches!(res, Ok(1)));
```


## 4. 찾지 못했을 때의 의미 있는 활용
- Err(idx)는 **정렬을 유지하면서 이 값을 끼워 넣을 위치** 를 의미.
```rust
let arr = [10, 20, 30, 40, 50];

let target = 35;
let res = arr.binary_search_by(|x| x.cmp(&target));

assert_eq!(res, Err(3)); // 35를 넣으면 index 3에 들어가야 정렬 유지
```

- 이걸 이용해서:
  - 정렬된 벡터에 중복 없이 삽입
  - lower_bound / upper_bound 같은 동작 구현
- 같은 걸 쉽게 할 수 있음.

## 5. binary_search_by_key와의 차이
- binary_search_by_key:
  - binary_search_by_key(&key, |x| x.some_field)
  - **키 추출 함수** 만 넘기면 됨
- binary_search_by:
  - 비교 전체를 직접 정의
  - 더 유연하지만, 직접 Ordering을 만들어야 함
- 예:
```rust
let data = [(1, "a"), (3, "b"), (5, "c")];
let res = data.binary_search_by_key(&3, |(k, _)| *k);
// vs
let res2 = data.binary_search_by(|(k, _)| k.cmp(&3));
```

## 6. 한 줄 요약
- binary_search_by는 정렬된 슬라이스에서, **원소를 어떻게 비교할지** 를 직접 정의해서
  이진 탐색을 수행하는 메서드,
- struct/tuple/커스텀 키 기반 검색을 할 때 사실상 기본 도구라고 보면 된다.

---



