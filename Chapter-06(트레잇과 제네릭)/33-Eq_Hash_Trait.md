# Eq / Hash Trait

## 🧩 Eq 트레잇: "완전한 동등성"
### 정의
Eq는 PartialEq의 상위 트레잇으로, 모든 값이 자기 자신과 같아야 한다는 성질을 보장합니다.
```rust
pub trait Eq: PartialEq<Self> {}
```

### 특징
- Eq는 메서드를 정의하지 않고, PartialEq의 구현이 완전한 동치 관계임을 나타내는 marker trait입니다.
- a == b이면 반드시 b == a이고, a == a는 항상 true여야 합니다.
- 대부분의 기본 타입(i32, String, bool, 등)은 Eq를 자동으로 구현합니다.
### 예시
```rust
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
```


## 🧮 Hash 트레잇: "해시 값을 계산하는 방법"

### 정의
Hash 트레잇은 객체의 해시 값을 계산할 수 있도록 합니다.  
HashMap이나 HashSet 같은 자료구조에서 키로 사용되려면 반드시 `Hash` 를 구현해야 합니다.

```rust
pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);
}
```

### 특징
- Hash는 Hasher라는 해시 상태 객체에 값을 feed하는 방식으로 동작합니다.
- Hash를 구현할 때는 Eq와의 일관성이 매우 중요합니다:
- a == b이면 반드시 hash(a) == hash(b)여야 합니다.
### 예시
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```
```rust
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let mut hasher = DefaultHasher::new();
    p1.hash(&mut hasher);
    println!("Hash: {}", hasher.finish());
}
```

## 🧠 함께 쓰이는 이유

### HashMap이나 HashSet에서 키로 사용되는 타입은 반드시 다음을 만족해야 합니다:
- Eq: 키의 동등성 비교를 위해
- Hash: 키의 해시 값을 계산하기 위해
- 예를 들어, HashMap<Point, String>을 사용하려면 `Point` 는 `Eq` 와 `Hash` 를 모두 구현해야 합니다.

### 🧷 주의할 점
- PartialEq만 구현하고 Eq를 구현하지 않으면, HashMap에서는 사용할 수 없습니다.
- Hash와 Eq의 구현이 일관되지 않으면, 해시 기반 컬렉션에서 버그가 발생할 수 있습니다. 
- 예: a == b인데 hash(a) != hash(b)이면 같은 키인데도 다른 버킷에 저장되어 검색이 안 될 수 있음.

---

#  커스텀 Eq, Hash
Rust에서 Eq와 Hash 트레잇을 커스텀하게 정의하는 것은 구조체나 열거형이 해시맵의 키로 사용되거나 동등성 비교가 필요한 경우에 매우 중요합니다.  
아래에 그 개념과 구현 방법을 자세히 설명.

## 🧠 기본 개념
### Eq 트레잇
- Eq는 PartialEq의 상위 트레잇입니다.
- Eq를 구현하면 "완전한 동등성"을 의미합니다.
- PartialEq은 == 연산자를 오버라이드하며, Eq는 추가적인 메서드를 요구하지 않습니다.
- Eq를 구현하려면 PartialEq도 반드시 구현되어 있어야 합니다.
### Hash 트레잇
- Hash는 해시 값을 생성하는 데 사용됩니다.
- HashMap, HashSet 등에서 키로 사용하려면 반드시 구현되어야 합니다.
- Hasher를 사용해 구조체의 필드를 해시합니다.

### 🛠 구현 예시
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cmp::PartialEq;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// PartialEq 구현
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name // 이름만 같으면 같은 사람으로 간주
    }
}
```
```rust
// Eq 구현 (추가 메서드 없음)
impl Eq for Person {}

// Hash 구현
impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state); // age는 무시하고 name만 해시
    }
}
```

### 🔍 설명
- PartialEq: name 필드만 비교 대상으로 삼음.
- Eq: PartialEq을 기반으로 완전한 동등성 보장.
- Hash: name만 해시 대상으로 삼음. age는 무시됨.
- 이렇게 하면 HashMap<Person, T>에서 Person의 name이 같으면 같은 키로 간주됩니다.

### ⚠️ 주의사항
- Hash와 Eq는 반드시 일관성 있게 구현되어야 합니다.
- 즉, eq(a, b)가 true이면 hash(a) == hash(b)도 반드시 true여야 합니다.
- 그렇지 않으면 HashMap이나 HashSet에서 예기치 않은 동작이 발생할 수 있습니다.
- 필요하다면 `#[derive(PartialEq, Eq, Hash)]` 를 자동으로 붙일 수도 있음. 
- 하지만 커스텀 로직이 필요할 땐 직접 구현해야 합니다.


## HashMap에 적용

- 커스텀 키를 사용하는 HashMap 예제
- 직접 Hasher를 구현하는 예제

### 🧩 1. 커스텀 키를 사용하는 HashMap 예제
```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```
```rust
// PartialEq과 Eq 구현
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

// Hash 구현
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
```
```rust
fn main() {
    let mut map: HashMap<Point, &str> = HashMap::new();
    map.insert(Point { x: 1, y: 2 }, "A");
    map.insert(Point { x: 3, y: 4 }, "B");

    let key = Point { x: 1, y: 2 };
    println!("Value for {:?}: {:?}", key, map.get(&key));
}
```
- ✅ 이 예제에서는 Point 구조체를 키로 사용하며, x와 y 값이 같으면 같은 키로 간주됩니다.

### 🧪 2. 직접 Hasher 구현하기 (고급 예제)
Rust에서는 대부분 DefaultHasher를 사용하지만, 직접 해시 알고리즘을 구현할 수도 있음.    
아래는 XOR 기반의 간단한 해시 구현 예제입니다:
```rust
use std::hash::{Hasher, Hash};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[derive(Debug)]
struct SimpleHasher {
    hash: u64,
}
```
```rust
impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash ^= *byte as u64;
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}
```
```rust
impl Default for SimpleHasher {
    fn default() -> Self {
        SimpleHasher { hash: 0 }
    }
}
```

```rust
// 커스텀 Hasher를 사용하는 BuildHasher
#[derive(Clone)]
struct MyBuildHasher;

impl std::hash::BuildHasher for MyBuildHasher {
    type Hasher = SimpleHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SimpleHasher::default()
    }
}
```

```rust
#[derive(Hash, Eq, PartialEq, Debug)]
struct Key {
    id: u32,
}

fn main() {
    let mut map: HashMap<Key, &str, MyBuildHasher> = HashMap::with_hasher(MyBuildHasher);
    map.insert(Key { id: 42 }, "Meaning of life");
    map.insert(Key { id: 7 }, "Lucky number");

    println!("{:?}", map.get(&Key { id: 42 }));
}
```

- 🔍 이 예제에서는 SimpleHasher를 직접 구현하고, 이를 BuildHasher로 감싸서 HashMap에 적용했어요.

### 💡 언제 직접 Hasher를 구현해야 할까?
- 해시 충돌을 줄이고 싶을 때
- 보안이 중요한 경우 (예: DoS 공격 방지)
- 특정 해시 알고리즘을 테스트하거나 디버깅할 때

---

