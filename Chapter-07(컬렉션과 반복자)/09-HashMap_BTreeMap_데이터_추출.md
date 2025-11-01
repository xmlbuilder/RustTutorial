# HashMap / BTreeMap 데이터 추출

Rust에서 HashMap이나 BTreeMap을 순회하면서 key만, value만, 또는 둘 다 따로 추출하는 방법은 매우 유용하고 자주 쓰이는 패턴.

## 🧠 Map 순회하면서 key/value 추출하기
### 1. 전체 (key, value) 순회
```rust
use std::collections::HashMap;
let map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

for (key, value) in &map {
    println!("{}: {}", key, value);
}
```
### 2. 🔑 Key만 추출
#### ✅ 참조로 순회
```rust
for key in map.keys() {
    println!("key: {}", key);
}
```

#### ✅ 복사해서 Vec으로 수집
```rust
let keys: Vec<_> = map.keys().cloned().collect();
```


### 3. 📦 Value만 추출
#### ✅ 참조로 순회
```rust
for value in map.values() {
    println!("value: {}", value);
}
```

#### ✅ 복사해서 Vec으로 수집
```rust
let values: Vec<_> = map.values().cloned().collect();
```


#### 4. 🧪 가변 참조로 value 수정
```rust
for value in map.values_mut() {
    *value += 1;
}
```

values_mut()은 &mut V를 반환하므로
직접 값을 수정할 수 있음.


#### 5. 🔄 key/value를 분리해서 각각 Vec에 담기
```rust
let (keys, values): (Vec<_>, Vec<_>) = map.iter().map(|(k, v)| (k.clone(), v.clone())).unzip();
```


### 🧠 Map 순회 요약 표
| 목적             | 메서드             | 반환 타입                  | 수정 가능 |
|------------------|--------------------|----------------------------|------------|
| key 순회         | `map.keys()`       | `Iterator<&K>`             | ❌         |
| value 순회       | `map.values()`     | `Iterator<&V>`             | ❌         |
| value 수정       | `map.values_mut()` | `Iterator<&mut V>`         | ✅         |
| 전체 순회        | `map.iter()`       | `Iterator<(&K, &V)>`       | ❌         |
| 전체 수정        | `map.iter_mut()`   | `Iterator<(&K, &mut V)>`   | ✅         |


---
# Key 값 수정

## ✅ 그럼 어떻게 해야 할까?

### 1. 기존 key를 제거하고 새 key로 다시 삽입
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("old_key", 42);

    if let Some(value) = map.remove("old_key") {
        map.insert("new_key", value);
    }

    println!("{:?}", map); // {"new_key": 42}
}
```
- 이렇게 하면 값은 유지하면서 key만 바꾸는 효과를 얻을 수 있음.
 
### 2. value는 자유롭게 수정 가능
```rust
if let Some(v) = map.get_mut("new_key") {
    *v += 1;
}
```

- get_mut()을 쓰면 value는 안전하게 수정 가능합니다.

### 🔑 핵심 요약: HashMap 수정 가능 여부

| 항목         | 수정 가능 여부 | 방법 또는 메서드       | 비고 또는 특징                         |
|--------------|----------------|------------------------|----------------------------------------|
| key          | ❌ 불가능       | `remove` → `insert`    | 직접 수정 불가, 해시 안정성 문제로 차단 |
| value        | ✅ 가능         | `get_mut()`            | 안전하게 참조 후 수정 가능              |

Rust는 **안전한 상태를 유지하는 것** 을 최우선으로 하기 때문에 key 수정처럼 위험한 작업은 아예 막아두고, 우회 방식만 허용합니다.



