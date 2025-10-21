# 🔍 Borrow<T>란?
Borrow<T>는 AsRef<T>와 비슷하지만 목적이 다릅니다.  
AsRef<T>는 단순히 참조를 얻는 데 쓰이고, Borrow<T>는 값의 동작을 동일하게 만들기 위해 사용됩니다.  
특히 Eq, Ord, Hash 같은 트레이트가 소유된 값과 참조된 값에서 동일하게 작동해야 할 때 사용됩니다.

```rust
pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}
```


## 📘 기본 예제: String과 &str을 동일하게 비교
```rust
use std::collections::HashMap;
use std::borrow::Borrow;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("apple".to_string(), 3);

    // get 메서드는 &str로도 검색 가능
    let count = map.get("apple"); // &str은 String으로부터 Borrow<str> 구현 덕분에 가능
    println!("{:?}", count); // Some(3)
}
```


## 핵심 포인트:
- HashMap<String, i32>는 키로 String을 사용하지만,  
- get(&str)이 가능한 이유는 String: Borrow<str>이기 때문입니다.  
- Borrow<T>는 Hash, Eq가 동일하게 작동하도록 보장합니다.

## 🛠 실전 예제: 사용자 정의 타입에 Borrow 구현
```rust
use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct UserId(String);

impl PartialEq for UserId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// &str로 검색할 수 있도록 Borrow 구현
impl Borrow<str> for UserId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

fn main() {
    let mut users: HashMap<UserId, &str> = HashMap::new();
    users.insert(UserId("jung".to_string()), "JungHwan");

    // &str로 검색 가능
    if let Some(name) = users.get("jung") {
        println!("Found user: {}", name);
    }
}
```

## 실전 활용 요약:
- UserId는 내부적으로 String을 가지고 있음.
- Borrow<str>를 구현하면 &str로도 HashMap에서 검색 가능.
- 이는 API를 더 유연하고 사용자 친화적으로 만들어 줍니다.

## ✅ 왜 직접 구현하는가?
- HashMap<UserId, V>에서 get(&str)을 가능하게 하려면 UserId: Borrow<str>가 필요합니다.
- Borrow<T>는 Eq, Hash가 일관되게 작동해야 하므로, 내부 값이 T와 같다는 의미를 가져야 합니다.


## 1. 🔄 Borrow<T> vs AsRef<T> 비교표

| 항목               | Borrow<T>                                                | AsRef<T>                                                |
|--------------------|----------------------------------------------------------|----------------------------------------------------------|
| 🔍 목적            | 값의 내부 참조를 통해 Eq, Hash 일관성 유지               | 참조로 변환하여 일반적인 API 유연성 제공                |
| 🧠 주요 사용처     | HashMap.get(&str) 등에서 String과 &str을 동일하게 취급   | PathBuf, String 등 다양한 타입을 참조로 변환할 때        |
| 🔗 트레이트 요구사항 | Eq, Hash가 동일하게 작동해야 함                          | 그런 요구 없음                                           |
| 📦 예시            | HashMap<String, V>.get(&str) 가능                         | fn foo<T: AsRef<Path>>(t: T) 형태로 사용                 |
| 🔄 변환 방식       | 내부 참조를 반환 (`borrow()`)                             | 참조로 변환 (`as_ref()`)                                 |
| 🧩 일반성          | 덜 일반적 (특정 상황에 최적화됨)                         | 더 일반적 (다양한 타입에 적용 가능)                      |

---

## 🔄 Borrow<T> vs BorrowMut<T> 비교표

| 트레이트         | 반환 타입  | 설명                                |
|------------------|------------|-------------------------------------|
| `Borrow<T>`      | `&T`       | 불변 참조를 추상화 — 읽기 전용       |
| `BorrowMut<T>`   | `&mut T`   | 가변 참조를 추상화 — 수정 가능       |

### 예시:
```rust
use std::borrow::{Borrow, BorrowMut};

fn print_len<T: Borrow<str>>(val: T) {
    println!("Length: {}", val.borrow().len());
}

fn append_world<T: BorrowMut<String>>(mut val: T) {
    val.borrow_mut().push_str(" world");
}

fn main() {
    let s = String::from("hello");
    print_len(&s); // Borrow<str>
    let mut s2 = String::from("hello");
    append_world(&mut s2); // BorrowMut<String>
    println!("{}", s2); // hello world
}
```


## 🐄 2. Cow<T>: Clone-on-Write
Cow<T>는 **소유된 값과 참조된 값의 공존** 을 위한 열쇠입니다.

```rust
use std::borrow::Cow;

fn process<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains("!") {
        Cow::Owned(input.replace("!", ""))
    } else {
        Cow::Borrowed(input)
    }
}
```

### 핵심 개념:
- Cow<'a, T>는 Borrow<T>를 구현한 타입을 받아들임
- 내부적으로 Borrowed(&T) 또는 Owned(T)를 가질 수 있음
- 필요할 때만 복사해서 소유권을 획득함 (성능 최적화)

## 🔗 Borrow<T>, BorrowMut<T>, Cow<T> 관계 요약
| 개념            | 관련성 설명                                      |
|-----------------|--------------------------------------------------|
| `Borrow<T>`     | `Cow<T>`는 내부적으로 `Borrow<T>`를 기반으로 동작 |
| `BorrowMut<T>`  | `Cow<T>`는 직접 사용하지 않지만 유사한 개념을 가짐 |
| `Cow<T>`        | `Borrow<T>`를 구현한 타입을 참조 또는 복사하여 처리 |

### 📘 실전 활용 예: Cow<str>로 문자열 처리
```rust
use std::borrow::Cow;

fn normalize(input: &str) -> Cow<str> {
    if input.starts_with("http://") {
        Cow::Owned(input.replacen("http://", "https://", 1))
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    let url = "http://example.com";
    let secure = normalize(url);
    println!("{}", secure); // https://example.com
}
```
---





