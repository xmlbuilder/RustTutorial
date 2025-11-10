# KeyValuePool
아래는 KeyValuePool 구조체와 관련된 Rust 코드의 전체 설명과 함께, 각 메서드를 정리한 함수 표입니다.  
이 구조체는 삽입 순서를 유지하는 키-값 저장소로 설계되어 있으며, HashMap과 Vec을 함께 사용합니다.

## 소스 코드 
```rust
use std::collections::HashMap;

/// (호환용) 원한다면 여전히 상태를 알고 싶을 때 사용
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Upsert {
    Inserted,
    Updated,
}

#[derive(Clone, Debug, Default)]
pub struct KeyValuePool {
    map: HashMap<String, String>,
    keys: Vec<String>, // 삽입 순서 유지용
}
```
```rust
impl KeyValuePool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) -> Option<String> {
        let key = key.into();
        let old = self.map.insert(key.clone(), val.into());
        if old.is_none() {
            self.keys.push(key);
        }
        old
    }

    pub fn set_kind(&mut self, key: impl Into<String>, val: impl Into<String>) -> Upsert {
        if self.set(key, val).is_some() {
            Upsert::Updated
        } else {
            Upsert::Inserted
        }
    }

    pub fn set_value_bool(&mut self, key: impl Into<String>, val: impl Into<String>) -> bool {
        self.set(key, val).is_some()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }

    pub fn get_owned(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    pub fn get_owned_or_empty(&self, key: &str) -> String {
        self.map.get(key).cloned().unwrap_or_default()
    }

    pub fn has(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.keys.clear();
    }

    pub fn keys(&self) -> &[String] {
        &self.keys
    }

    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.map
    }

    pub fn iter_in_insert_order(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.keys
            .iter()
            .filter_map(|k| self.map.get(k).map(|v| (k.as_str(), v.as_str())))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
```

## 🧠 구조체 설명
### 🔹 KeyValuePool
- map: HashMap<String, String> — 키-값 저장소
- keys: Vec<String> — 삽입 순서를 유지하기 위한 키 목록
이 구조체는 일반적인 HashMap의 기능에 더해, 삽입 순서 유지, 업데이트 여부 확인, 다양한 조회 방식 등을 제공합니다.


### 📦 열거형 Upsert
```rust
pub enum Upsert {
    Inserted,
    Updated,
}
```

- Inserted: 새로 삽입된 경우
- Updated: 기존 값을 덮어쓴 경우

## 📋 함수 정리표
| 메서드 이름              | 설명                                                         | 반환 타입             |
|--------------------------|--------------------------------------------------------------|------------------------|
| new()                    | 새 KeyValuePool 인스턴스 생성                                | Self                   |
| set(key, val)            | 키에 값을 설정하고 이전 값을 반환. 삽입 시 순서 기록        | Option<String>         |
| set_kind(key, val)       | 삽입/업데이트 여부를 `Upsert`로 반환                         | Upsert                 |
| set_value_bool(key, val) | 값 설정 후 기존 값이 있었는지 여부를 bool로 반환             | bool                   |
| get(key)                 | 키에 해당하는 값을 참조로 반환 (`&str`)                      | Option<&str>           |
| get_owned(key)           | 키에 해당하는 값을 복사하여 반환 (`String`)                  | Option<String>         |
| get_owned_or_empty(key)  | 키에 해당하는 값이 없으면 빈 문자열 반환                     | String                 |
| has(key)                 | 키 존재 여부 확인                                            | bool                   |
| clear()                  | 모든 키-값과 순서 정보 초기화                                | void                   |
| keys()                   | 삽입된 키 목록을 참조로 반환 (`&[String]`)                   | &[String]              |
| as_map()                 | 내부 HashMap을 참조로 반환                                   | &HashMap<String, String> |
| iter_in_insert_order()   | 삽입 순서대로 (key, value) 쌍을 반환하는 반복자              | Iterator<(&str, &str)> |
| len()                    | 저장된 키-값 쌍의 개수 반환                                  | usize                  |
| is_empty()               | 저장소가 비어있는지 여부 확인                               | bool                   |


## ✅ 특징 요약
- 삽입 순서 유지: keys 벡터를 통해 HashMap의 순서 비결정성을 보완
- 다양한 조회 방식: 참조형, 소유형, 기본값 반환 등
- 업데이트 추적: Upsert를 통해 삽입/수정 여부 확인 가능
- 사용자 친화적 API: set_value_bool, get_owned_or_empty 등 실용적인 메서드 제공


## 🧪 KeyValuePool 테스트 기능 표

| 테스트 함수 이름               | 검증 내용 및 목적                                      |
|-------------------------------|--------------------------------------------------------|
| set_returns_old_value         | 삽입/업데이트 시 반환값 및 키 목록 확인               |
| wrappers_still_available      | set_kind, set_value_bool 래퍼 함수 동작 확인          |
| iterate_in_insert_order       | 삽입 순서대로 반복자 동작 확인                        |
| clear_and_queries             | clear(), has(), get_owned_or_empty() 동작 확인        |
| len_and_is_empty              | 저장소 크기 및 비어있는지 여부 확인                  |
| empty_key_and_value           | 빈 문자열 키/값 처리 가능 여부 확인                   |
| get_vs_get_owned              | 참조형 vs 소유형 반환값 비교                         |
| duplicate_key_does_not_duplicate_order | 중복 키 삽입 시 순서 중복 방지 확인       |


## 테스트 코드
```rust
#[cfg(test)]
mod key_value_pool_tests {
    use nurbslib::core::key_value_pool::{KeyValuePool, Upsert};
```
```rust
    #[test]
    fn set_returns_old_value() {
        let mut kv = KeyValuePool::new();

        // 신규 삽입 → None
        assert_eq!(kv.set("A", "1"), None);
        assert_eq!(kv.get("A"), Some("1"));
        assert_eq!(kv.keys(), &["A"]);

        // 참조 버전
        assert_eq!(kv.get("A"), Some("1"));

        // 소유 버전
        assert_eq!(kv.get_owned("A"), Some("1".to_string()));

        // 필요 시 매핑으로 비교
        assert_eq!(kv.get("A").map(ToOwned::to_owned), Some("1".to_string()));

        // 업데이트 → Some(old)
        assert_eq!(kv.set("A", "2"), Some("1".to_string()));
        assert_eq!(kv.get("A"), Some("2"));
        // 키 목록은 중복 추가되지 않음
        assert_eq!(kv.keys(), &["A"]);

        // 또 다른 신규
        assert_eq!(kv.set("B", "x"), None);
        assert_eq!(kv.keys(), &["A", "B"]);
    }
```
```rust
    #[test]
    fn wrappers_still_available() {
        let mut kv = KeyValuePool::new();
        assert_eq!(kv.set_kind("K", "v1"), Upsert::Inserted);
        assert_eq!(kv.set_kind("K", "v2"), Upsert::Updated);

        assert_eq!(kv.set_value_bool("K", "v3"), true); // Updated
        assert_eq!(kv.set_value_bool("N", "v0"), false); // Inserted
    }
```
```rust
    #[test]
    fn iterate_in_insert_order() {
        let mut kv = KeyValuePool::new();
        kv.set("first", "1"); // None
        kv.set("second", "2"); // None
        kv.set("third", "3"); // None
        kv.set("second", "22"); // Some("2"), 순서는 그대로

        let v: Vec<(&str, &str)> = kv.iter_in_insert_order().collect();
        assert_eq!(v, vec![("first", "1"), ("second", "22"), ("third", "3")]);
    }
```
```rust
    #[test]
    fn clear_and_queries() {
        let mut kv = KeyValuePool::new();
        kv.set("k", "v");
        assert!(kv.has("k"));
        assert_eq!(kv.get_owned_or_empty("nope"), "");
        kv.clear();
        assert!(kv.is_empty());
    }
```
```rust
    #[test]
    fn len_and_is_empty() {
        let mut kv = KeyValuePool::new();
        assert_eq!(kv.len(), 0);
        assert!(kv.is_empty());

        kv.set("A", "1");
        kv.set("B", "2");
        assert_eq!(kv.len(), 2);
        assert!(!kv.is_empty());
    }
```
```rust
    #[test]
    fn empty_key_and_value() {
        let mut kv = KeyValuePool::new();
        kv.set("", "");
        assert_eq!(kv.get(""), Some(""));
        assert_eq!(kv.keys(), &[""]);
    }
```
```rust
    #[test]
    fn get_vs_get_owned() {
        let mut kv = KeyValuePool::new();
        kv.set("X", "value");
        let ref_val = kv.get("X").unwrap();
        let owned_val = kv.get_owned("X").unwrap();
        assert_eq!(ref_val, owned_val);
    }
```
```rust
    #[test]
    fn duplicate_key_does_not_duplicate_order() {
        let mut kv = KeyValuePool::new();
        kv.set("dup", "1");
        kv.set("dup", "2");
        kv.set("dup", "3");
        assert_eq!(kv.keys(), &["dup"]);
        assert_eq!(kv.get("dup"), Some("3"));
    }
}
```
---

# iter_in_insert_order

아래는 iter_in_insert_order 함수의 동작을 단계별로 상세하게 해석한 설명입니다.  
이 함수는 KeyValuePool에 저장된 키-값 쌍을 삽입 순서대로 반복(iterate) 하기 위해 만들어졌습니다.

## 🔍 함수 전체 구조
```rust
pub fn iter_in_insert_order(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
    self.keys
        .iter()
        .filter_map(|k| self.map.get(k).map(|v| (k.as_str(), v.as_str())))
}
```

## 🧩 단계별 설명
### 1️⃣ self.keys.iter()
- self.keys는 Vec<String> 타입으로, 키의 삽입 순서를 저장합니다.
- .iter()를 호출하면 &String 타입의 반복자가 생성됩니다.
- 즉, "A", "B", "C" 순서로 삽입되었다면 그 순서대로 반복됩니다.

### 2️⃣ .filter_map(...)
- filter_map은 Option<T>를 반환하는 클로저를 받아서:
- Some(t)이면 t를 결과에 포함
- None이면 해당 항목을 건너뜁니다
- 이걸 통해 삭제된 키는 자동으로 제외됩니다.

### 3️⃣ self.map.get(k)
- k는 &String이므로 HashMap<String, String>에서 해당 키의 값을 조회합니다.
- 값이 존재하면 Some(&String)을 반환하고, 없으면 None을 반환합니다.

### 4️⃣ .map(|v| (k.as_str(), v.as_str()))
- 키와 값 모두 &String이므로 .as_str()을 사용해 &str로 변환합니다.
- 최종적으로 (&str, &str) 튜플을 반환합니다.

### ✅ 최종 결과
- 이 함수는 (&str, &str) 타입의 반복자를 반환합니다.
- 반환되는 순서는 삽입 순서이며, 삭제된 키는 제외됩니다.
- 예를 들어 다음과 같은 코드가 있다면:
```rust
let mut kv = KeyValuePool::new();
kv.set("A", "1");
kv.set("B", "2");
kv.set("A", "3"); // 업데이트
kv.set("C", "4");
```

- iter_in_insert_order()는 다음을 반환합니다:
```
[("A", "3"), ("B", "2"), ("C", "4")]
```

---

# `+ '_`

Rust에서 + '_는 반환되는 iterator가 self에 종속된 lifetime을 가진다는 의미입니다.  
이걸 정확히 이해하려면 함수 시그니처 전체를 살펴봐야 함:

## 🔍 함수 시그니처
```rust
pub fn iter_in_insert_order(&self) -> impl Iterator<Item = (&str, &str)> + '_
```


## 🧠 + '_의 의미
- impl Iterator<...> + '_는 **익명 타입(impl Trait)** 에 lifetime 바운드를 붙인 것입니다.
- 이 '_는 `&self의 lifetime` 을 의미합니다.
- 즉, 이 iterator는 self가 살아 있는 동안만 유효합니다.

## 📌 왜 필요한가?
반환되는 iterator는 내부적으로 self.keys와 self.map의 참조를 사용합니다:
```rust
self.keys.iter()
    .filter_map(|k| self.map.get(k).map(|v| (k.as_str(), v.as_str())))
```

- self.keys.iter() → &String을 반복
- self.map.get(k) → &String 반환
- k.as_str() / v.as_str() → &str 반환

이 모든 참조는 self의 내부 데이터를 가리키므로, Rust는 반환된 iterator가 self보다 오래 살면 안 된다고 판단합니다.  
그래서 + '_를 붙여서 **이 iterator는 self와 같은 lifetime을 가진다** 고 명시하는 것임.

## ✅ 요약: + '_의 의미

| 표현     | 의미 설명                                 |
|----------|--------------------------------------------|
| + '_     | 반환된 iterator는 `self`의 lifetime에 종속 |
| &str     | `self` 내부의 데이터를 참조함              |
| self     | iterator가 `self`를 기준으로 살아있어야 함 |

---



