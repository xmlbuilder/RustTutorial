# HashMap / BTreeMap 함수 정리

아래는 HashMap / BTreeMap에서 자주 쓰는 함수들을 정리 + 샘플 코드로 구성한 문서입니다.
각 함수는 main() 함수 안에서 바로 실행 가능한 형태로 제공.


## ✅ HashMap / BTreeMap에서 자주 쓰는 함수들

| 함수명             | 설명                           | 예시 코드 예시                                      |
|--------------------|--------------------------------|-----------------------------------------------------|
| `insert()`         | 키-값 추가                     | `map.insert("apple", 3);`                           |
| `get()`            | 키로 값 조회                   | `map.get("apple")`                                  |
| `entry()`          | 키 존재 여부에 따라 삽입/수정  | `map.entry("apple").or_insert(0);`                  |
| `contains_key()`   | 키 존재 여부 확인              | `map.contains_key("apple")`                         |
| `remove()`         | 키 제거                        | `map.remove("apple");`                              |
| `iter()`           | 키-값 순회                     | `for (k, v) in &map { println!("{} → {}", k, v); }` |
| `keys()` / `values()` | 키 또는 값만 순회          | `map.keys()` / `map.values()`                       |
| `retain()`         | 조건에 따라 요소 유지          | `map.retain(|_, v| *v >= 3);`                       |



### 1. insert() — 키-값 추가
```rust
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);
    println!("{:?}", map); // {"apple": 3, "banana": 5}
}
```


### 2. get() — 키로 값 조회
```rust
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);

    if let Some(val) = map.get("apple") {
        println!("Apple count: {}", val);
    }
}
```


### 3. entry() — 키 존재 여부에 따라 삽입/수정
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.entry("apple").or_insert(0);
    *map.entry("apple").or_insert(0) += 1;

    println!("{:?}", map); // {"apple": 1}
}
```


### 4. contains_key() — 키 존재 여부 확인
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);

    println!("Has apple? {}", map.contains_key("apple")); // true
}
```


### 5. remove() — 키 제거
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.remove("apple");

    println!("{:?}", map); // {}
}
```


### 6. iter() — 키-값 순회
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);

    for (key, value) in &map {
        println!("{} → {}", key, value);
    }
}
```


### 7. keys() / values() — 키 또는 값만 순회
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);

    for key in map.keys() {
        println!("Key: {}", key);
    }

    for value in map.values() {
        println!("Value: {}", value);
    }
}
```


###  8. retain() — 조건에 따라 요소 유지
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);
    map.insert("pear", 1);

    map.retain(|_, &mut v| v >= 3);
    println!("{:?}", map); // {"apple": 3, "banana": 5}
}
```


## 🔁 BTreeMap도 동일하게 사용 가능
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("c", 3);
    map.insert("a", 1);
    map.insert("b", 2);

    for (k, v) in &map {
        println!("{} → {}", k, v); // 키 순서대로 출력됨
    }
}
```
---
# 카운팅, 그룹화, 정렬, 병합
Rust에서 HashMap과 BTreeMap은 단순한 키-값 저장소를 넘어서,  
카운팅, 그룹화, 정렬, 병합 같은 실전 로직을 구조적으로 처리할 수 있는 강력한 도구입니다. 
아래에 각 패턴을 실전 예제 중심 정리.


## 🧩 HashMap vs BTreeMap: 핵심 차이
| 기능         | HashMap                         | BTreeMap                          |
|--------------|----------------------------------|-----------------------------------|
| 정렬 여부    | 없음 (무작위 순서)               | 자동 정렬 (키 기준 오름차순)       |
| 성능         | 빠름 (O(1) 평균)                 | 느림 (O(log n))                   |
| 용도         | 빠른 조회, 카운팅                | 정렬된 출력, 범위 검색             |
| 키 제약      | `Eq + Hash` 필요                 | `Ord` 필요                         |



### 🔢 1. 카운팅 패턴 (빈도수 계산)
```rust
use std::collections::HashMap;

let data = vec!["apple", "banana", "apple", "orange", "banana"];
let mut counter = HashMap::new();

for item in data {
    *counter.entry(item).or_insert(0) += 1;
}
```

- entry().or_insert() 패턴은 존재 여부에 따라 초기화 + 증가를 동시에 처리
- 결과: {"apple": 2, "banana": 2, "orange": 1}

### 🧮 2. 그룹화 패턴 (값을 키로 묶기)
```rust
use std::collections::HashMap;

let data = vec![("A", 1), ("B", 2), ("A", 3), ("B", 4)];
let mut groups: HashMap<&str, Vec<i32>> = HashMap::new();

for (key, value) in data {
    groups.entry(key).or_default().push(value);
}
```

- or_default()는 Vec::new()를 자동으로 호출
- 결과: {"A": [1, 3], "B": [2, 4]}

### 📊 3. 정렬 패턴 (정렬된 출력)
```rust
use std::collections::BTreeMap;

let mut scores = BTreeMap::new();
scores.insert("Alice", 90);
scores.insert("Bob", 85);
scores.insert("Charlie", 95);

for (name, score) in &scores {
    println!("{name}: {score}");
}
```

- BTreeMap은 키 기준으로 자동 정렬
- 결과: Alice → Bob → Charlie 순서로 출력

### 🔗 4. 병합 패턴 (두 맵 합치기)
```rust
use std::collections::HashMap;

let mut map1 = HashMap::from([("A", 1), ("B", 2)]);
let map2 = HashMap::from([("B", 3), ("C", 4)]);

for (k, v) in map2 {
    map1.entry(k).and_modify(|e| *e += v).or_insert(v);
}
```

- and_modify()는 기존 값이 있으면 수정
- or_insert()는 없으면 삽입
- 결과: {"A": 1, "B": 5, "C": 4}

---

