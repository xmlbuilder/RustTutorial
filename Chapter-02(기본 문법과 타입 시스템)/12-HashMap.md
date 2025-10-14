# HashMap

## 🎯 HashMap을 사용하는 이유

| 필요 상황                     | HashMap의 역할 및 장점                                      |
|------------------------------|-------------------------------------------------------------|
| 빠른 키 기반 조회             | 평균적으로 `O(1)` 시간 복잡도로 값에 접근 가능              |
| 키-값 쌍 데이터 구조          | `key → value` 형태로 명확하게 데이터 구조화 가능             |
| 값 삽입 및 갱신              | `.insert()`로 키에 대응하는 값을 삽입하거나 덮어쓰기 가능     |
| 조건 기반 검색               | `.contains_key()`, `.values().any()` 등으로 조건 필터링 가능  |
| 동적 데이터 저장             | 런타임에 키-값을 자유롭게 추가/삭제 가능                     |
| 복합 관계 표현               | `HashMap<String, Book>` 또는 `HashMap<Book, String>` 등으로 복잡한 관계 표현 가능 |
| 다양한 시스템 구성 요소에 활용 | 캐시, 설정값 저장, 인덱스 관리 등 다양한 용도에 적합          |




## 🧠 HashMap 핵심 개념 요약
| 🧠 기능 항목                | 🔧 사용 예시 및 설명                                               |
|----------------------------|--------------------------------------------------------------------|
| 선언 및 가져오기           | `use std::collections::HashMap;`                                   |
| 생성                       | `HashMap::new()`, `HashMap::from([...])`                           |
| 삽입                       | `.insert(key, value)`                                              |
| 삭제                       | `.remove(key)`                                                     |
| 조회                       | `.get(&key)` → `Option<&V>`                                        |
| 수정                       | `.entry(key).and_modify(\|v\| ...)`                                  |
| 반복                       | `for (k, v) in &map`                                               |
| 키 존재 확인               | `.contains_key(&key)`                                              |
| 값 조건 검색               | `.values().any(\|v\| ...)`                                           |



## 📦 소유권과 트레잇 관련 주의사항
### 1. HashMap의 키로 구조체를 쓰려면?
```rust
#[derive(Debug, Eq, Hash, PartialEq)]
struct Book { ... }
```

- `Eq`, `Hash`, `PartialEq` 트레잇이 필요함
- 이유: HashMap은 내부적으로 키를 해시하고 비교해야 하기 때문
### 2. 키로 구조체를 쓰면 소유권이 이동됨
```rust
library.insert(book1, "1".to_string()); // book1은 여기서 move됨
```

- 이후 book1을 직접 사용할 수 없음
- 대신 library.get(`&Book::new(...)`)처럼 동일한 값의 새 인스턴스로 조회 가능

## 🔍 실전 패턴 정리
### ✅ 문자열 키로 구조체 값 저장
```rust
let mut library: HashMap<String, Book> = HashMap::new();
library.insert("Rust".to_string(), Book::new(...));
```

- 키는 복사 가능한 String
- 값은 구조체 → 소유권은 HashMap이 가짐
### ✅ 구조체 키로 문자열 값 저장
```rust
#[derive(Debug, Eq, Hash, PartialEq)]
struct Book { ... }

let mut library: HashMap<Book, String> = HashMap::new();
library.insert(book1, "Rust".to_string());
```

- 키는 구조체 → 반드시 `Eq`, `Hash`, `PartialEq` 필요
- 값은 복사 가능한 String

## 🧪 조회 시 주의할 점
| 🧪 조회 방식               | 🔍 설명 및 주의사항                                                  |
|---------------------------|---------------------------------------------------------------------|
| `.get(&book1)`            | `book1`이 `.insert()`로 인해 move되었으면 사용할 수 없음             |
| `.get(&Book::new(...))`   | 동일한 값의 새 인스턴스를 생성하여 참조로 조회 가능                 |
| `.unwrap_or(...)`         | 조회 실패 시 기본값을 반환하여 안전하게 처리 가능 (`Option` 활용)    |


## ✅ HashMap 실전에서 주의할 점
| 항목                     | 설명 |
|--------------------------|------|
| 구조체를 키로 사용 시     | `Eq`, `Hash`, `PartialEq` 트레잇 필요 |
| 소유권 이동 주의         | `.insert()` 시 키나 값이 move됨 |
| 참조로 조회할 때         | `.get(&key)`는 참조자 필요 |
| 값 수정 시               | `.entry(key).and_modify(...)` 사용 |
| 값이 없을 경우 처리      | `.unwrap_or(...)`, `.is_none()` 등으로 안전하게 처리 |

----

# Find & Remove

Rust에서는 HashMap에서 값을 찾거나 제거할 때 아래 메서드를 사용합니다:
## 🔍 HashMap에서 값 찾기와 제거
| 동작 유형 | 메서드                     | 반환값 및 설명                                      |
|-----------|----------------------------|-----------------------------------------------------|
| 찾기      | `get(&key)`                | `Option<&V>` → `Some(value)` 또는 `None`            |
| 수정용 찾기 | `get_mut(&key)`           | `Option<&mut V>` → 값에 대한 가변 참조               |
| 제거      | `remove(&key)`             | `Option<V>` → 제거된 값 반환 또는 `None`            |
| 조건 제거 | `retain(\|k, v\| 조건)`      | 조건을 만족하지 않는 항목 제거 (`filter`처럼 작동) |


## ✅ 예시 코드
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);
map.insert("banana", 5);

// find
if let Some(&count) = map.get("apple") {
    println!("🍎 count: {}", count);
}

// remove
if let Some(removed) = map.remove("banana") {
    println!("🍌 removed: {}", removed);
}
```


## 🧬 구조체 내부에 HashMap이 있을 때 생명 주기와 주의사항

Rust에서는 구조체 안에 HashMap을 넣을 때 **값의 생명 주기(lifetime)** 와 **소유권(ownership)** 을 잘 관리해야 합니다.
### 1. 🔗 참조를 저장할 경우 → 생명 주기 명시 필요
```rust
use std::collections::HashMap;

struct Book<'a> {
    metadata: HashMap<&'a str, &'a str>,
}
```

- 'a 생명 주기를 명시하지 않으면 컴파일 에러 발생
- 참조된 값이 구조체보다 먼저 drop되면 dangling reference가 생김
### 2. 📦 값 자체를 저장할 경우 → 생명 주기 필요 없음
```rust
struct Book {
    metadata: HashMap<String, String>,
}
```

- String은 소유권을 가지므로 생명 주기 명시 불필요
- 구조체가 drop될 때 HashMap도 함께 drop됨

### 3. ⚠️ HashMap 사용 시 주의사항 (Rust 기준)

| 항목       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| `&str` 참조 | 생명 주기(`lifetime`) 명시 필수. 외부 데이터가 먼저 drop되면 위험함. |
| `Clone`     | `HashMap`을 복제하려면 `HashMap<K, V>: Clone` 조건을 만족해야 함.     |
| `serde`     | 직렬화/역직렬화를 위해 `K: Serialize`, `V: Serialize` 필요.           |
| 멀티스레드  | `HashMap`은 `Send`/`Sync`가 아님 → `Mutex<HashMap<...>>`로 감싸야 안전. |


## 🧠 실전 팁
- 구조체 안에 `HashMap<&str, &str>` 처럼 참조를 넣는 건 위험할 수 있음.  
    → 가능하면 String을 사용해 소유권을 명확히 하는 게 안전합니다.
- remove()는 반환값을 활용해 로그를 남기거나 조건 분기할 수 있어요.
- get_mut()으로 직접 값을 수정할 수 있으니, insert() 반복보다 효율적일 수 있습니다.

----

# HashMap내에 String

String이 heap에 저장된다는 점에서 생명 주기가 있다는 건 맞지만, 여기서 중요한 건 소유권과 drop 시점의 관계입니다.

## 🔍 HashMap<String, String>에서 생명 주기와 drop 순서
### ✅ 핵심 개념
- String은 소유권을 가진 값입니다. 즉, HashMap이 String을 소유하고 있기 때문에,
해당 String은 HashMap이 drop될 때 함께 drop됩니다.
- 생명 주기(lifetime)는 참조 타입(&str, &String)에서만 명시적으로 관리해야 합니다.
- HashMap<String, String>에서는 생명 주기 명시가 필요하지 않습니다.

## 🧠 구조적으로 보면
```rust
struct Book {
    metadata: HashMap<String, String>,
}
```
- 이 구조체가 drop되면 → 내부 HashMap도 drop됨
- HashMap이 drop되면 → 내부의 String 값들도 함께 drop됨
- 즉, String이 HashMap보다 오래 살아야 할 필요는 없습니다.
오히려 HashMap이 String을 소유하고 관리하기 때문에,
String의 생명 주기는 HashMap의 생명 주기에 종속됩니다.

### ⚠️ 생명 주기가 문제가 되는 경우는?
다음과 같은 경우에는 생명 주기 명시가 필요합니다:
```rust
struct Book<'a> {
    metadata: HashMap<&'a str, &'a str>,
}
```

- 여기서는 Book이 외부의 `&str` 을 참조하고 있으므로 `'a` 생명 주기를 명시해야 합니다.  
- 이 경우 Book이 참조하는 데이터가 먼저 drop되면 문제가 생기므로,  
Rust가 컴파일 타임에 생명 주기를 체크합니다.

## ✅ 결론
HashMap<String, String>에서 String은 heap에 있지만,
생명 주기 명시가 필요하지 않으며,  
HashMap이 drop될 때 함께 안전하게 drop됩니다.


