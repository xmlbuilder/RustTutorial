# Arena
## 🧠 Arena란?
- Arena는 메모리 풀(Pool) 또는 버퍼처럼 동작하는 구조입니다.
- 여러 객체를 같은 생명주기로 묶어서 한 번에 할당하고, 한 번에 해제합니다.
- 개별 객체를 drop하거나 free하지 않고, Arena 전체를 해제합니다.

## 🔧 Rust에서 Arena 사용 방식  
Rust 표준 라이브러리에는 Arena가 직접 포함되어 있진 않지만, 다음과 같은 crate들이 Arena를 제공합니다:
| Crate 이름    | 주요 구조 및 사용 방식         |
|---------------|-------------------------------|
| `typed-arena` | `Arena<T>.alloc()` 사용        |
| `bumpalo`     | `Bump.alloc()` 사용            |
| `id-arena`    | ID 기반으로 객체를 관리        |


## ✨ 예시: typed-arena
```rust
use typed_arena::Arena;
struct Node {
    value: i32,
}
fn main() {
    let arena = Arena::new();
    let node = arena.alloc(Node { value: 42 });
    println!("Node value: {}", node.value);
}
```
- `arena.alloc(...)` 은 Box처럼 힙에 객체를 할당하지만, Arena 내부에 저장됩니다.
- Arena가 drop되면 모든 객체가 한 번에 해제됩니다.

## ✅ Arena의 장점
| 항목             | 설명                                                             |
|------------------|------------------------------------------------------------------|
| 빠른 할당        | 포인터만 이동하므로 개별 힙 할당보다 훨씬 빠름                    |
| 해제 비용 없음   | 객체마다 drop하지 않고 Arena 전체를 한 번에 해제 가능             |
| 생명주기 관리    | 동일한 생명주기를 가진 객체들을 묶어 관리하기에 적합               |
| 구조적 설계      | 트리, 그래프, 파서 등 복잡한 구조를 만들 때 유용                  |


## ⚠️ 주의할 점
- Arena에 할당된 객체는 개별적으로 drop되지 않음 → Drop trait이 호출되지 않음
- 메모리 누수 위험은 없음 (Arena 자체가 drop되면 메모리 해제됨)
- 참조가 Arena보다 오래 살아서는 안 됨 → Rust의 borrow checker가 이를 막아줌


## 📦 Arena 관련 Crate 설치 방법
### 1. ✅ typed-arena
타입 안전한 Arena를 제공하며, 간단한 구조의 AST나 트리 구조에 적합합니다.
```
[dependencies]
typed-arena = "2.0"
```

#### 사용 예: 
```rust
let arena = typed_arena::Arena::new();
```

### 2. ⚡ bumpalo
고성능 bump allocator. 매우 빠른 할당이 필요할 때 사용합니다.
```
[dependencies]
bumpalo = "3.14"
```

#### 사용 예: 
```rust
let bump = bumpalo::Bump::new();
```

### 3. 🆔 id-arena
ID 기반으로 노드를 관리하는 Arena. 그래프나 트리 구조에서 노드를 인덱스로 참조할 때 유용합니다.
```
[dependencies]
id-arena = "2.2"
```

#### 사용 예:
```rust
let mut arena = id_arena::Arena::new();
let a = arena.alloc("node A");
let b = arena.alloc("node B");
```

## 📝 요약
| 항목         | 설명                         | Cargo.toml 설정               |
|--------------|------------------------------|-------------------------------|
| typed-arena  | 타입 안전 Arena              | `typed-arena = "2.0"`         |
| bumpalo      | 빠른 bump allocator          | `bumpalo = "3.14"`            |
| id-arena     | ID 기반 노드 관리 Arena      | `id-arena = "2.2"`            |


## 🧠 Arena를 사용하는 예제들
### 🧩 1. 파서에서 Arena 활용
파서나 AST를 만들 때, 노드들이 같은 생명주기를 공유하므로 Arena 할당이 적합합니다.
```rust
use typed_arena::Arena;

#[derive(Debug)]
struct AstNode {
    name: String,
    children: Vec<&'static AstNode>,
}

fn main() {
    let arena = Arena::new();

    let root = arena.alloc(AstNode {
        name: "root".into(),
        children: vec![],
    });

    let child = arena.alloc(AstNode {
        name: "child".into(),
        children: vec![],
    });

    root.children.push(child);
    println!("{:?}", root);
}
```
- Arena를 사용하면 AST 노드들이 동일한 생명주기를 가지므로 borrow checker 문제 없이 참조를 유지할 수 있습니다.


### 🔗 2. 그래프 구조에서 Arena 활용
그래프는 노드 간 참조가 많아 Rust의 소유권 모델과 충돌하기 쉬운데, Arena를 사용하면 이를 완화할 수 있습니다.
```rust
use typed_arena::Arena;

#[derive(Debug)]
struct Node<'a> {
    value: i32,
    edges: Vec<&'a Node<'a>>,
}

fn main() {
    let arena = Arena::new();

    let a = arena.alloc(Node { value: 1, edges: vec![] });
    let b = arena.alloc(Node { value: 2, edges: vec![a] });
    a.edges.push(b);

    println!("Node A: {:?}", a);
    println!("Node B: {:?}", b);
}
```
- 이 방식은 그래프의 순환 참조도 안전하게 표현할 수 있으며, Arena가 drop될 때 전체 메모리를 한 번에 해제합니다.

## ✅ 요약
| 항목             | 설명                                      |
|------------------|-------------------------------------------|
| Arena 장점       | 빠른 할당, 생명주기 공유, 구조적 설계에 유리 |
| 파서 활용        | AST 노드들을 Arena에 저장해 참조 문제 해결   |
| 그래프 활용      | 노드 간 참조를 Arena로 안전하게 구성 가능    |


## 🧩 예제: id-arena로 배열처럼 노드 저장하고 인덱스로 접근하기
### 1. Cargo.toml 설정
```
[dependencies]
id-arena = "2.2"
```


### 2. 코드 예제
```rust
use id_arena::Arena;

#[derive(Debug)]
struct Item {
    name: String,
    value: i32,
}

fn main() {
    // Arena 생성
    let mut arena = Arena::new();

    // Arena에 Item을 저장하고 ID를 받음
    let id1 = arena.alloc(Item { name: "Apple".into(), value: 100 });
    let id2 = arena.alloc(Item { name: "Banana".into(), value: 200 });
    let id3 = arena.alloc(Item { name: "Cherry".into(), value: 300 });

    // ID를 통해 Arena 내부 객체에 접근
    println!("Item 1: {:?}", arena[id1]);
    println!("Item 2: {:?}", arena[id2]);
    println!("Item 3: {:?}", arena[id3]);

    // 배열처럼 ID를 Vec에 저장하고 반복 처리
    let ids = vec![id1, id2, id3];
    for id in ids {
        let item = &arena[id];
        println!("{} = {}", item.name, item.value);
    }
}
```

## ✅ 핵심 포인트
| 항목           | 설명                                                   |
|----------------|--------------------------------------------------------|
| `Arena::alloc()` | 객체를 Arena에 저장하고 고유 ID를 반환함               |
| `arena[id]`     | 반환된 ID를 통해 Arena 내부의 객체에 배열처럼 접근 가능 |
| `Vec<Id>`       | 여러 ID를 저장해 반복 처리하거나 구조적으로 관리 가능   |


## 🔍 기본 구조: Arena는 Map이 아니다
```rust
let id = arena.alloc(data);      // ✅ ID → 데이터 가능
let data = &arena[id];           // ✅ ID로 접근

// ❌ arena.find(|d| d == target) 같은 건 없음
// ❌ arena.get_id(data) 같은 것도 없음
```

## ✅ 해결 방법: 별도의 HashMap을 병행 사용
- 데이터가 고유 ID를 갖고 있고, 그 ID를 통해 빠르게 찾고 싶다면 **HashMap<ID, T> 또는 HashMap<CustomKey, ID>** 를 병행해서 사용해야 합니다.
### 예시: 이름으로 ID를 찾기
```rust
use id_arena::{Arena, Id};
use std::collections::HashMap;

#[derive(Debug)]
struct Item {
    name: String,
    value: i32,
}

fn main() {
    let mut arena = Arena::new();
    let mut name_to_id = HashMap::new();

    let id1 = arena.alloc(Item { name: "apple".into(), value: 10 });
    let id2 = arena.alloc(Item { name: "banana".into(), value: 20 });

    name_to_id.insert("apple".to_string(), id1);
    name_to_id.insert("banana".to_string(), id2);

    // 이름으로 ID 찾고 Arena에서 데이터 조회
    if let Some(id) = name_to_id.get("banana") {
        println!("Found: {:?}", arena[*id]);
    }
}
```
## 🧠 요약

| 항목                   | 설명                                                             |
|------------------------|------------------------------------------------------------------|
| `arena[id]`            | ID를 통해 Arena 내부 객체에 배열처럼 접근 가능                   |
| `HashMap<CustomKey, Id>` | 커스텀 키로 Arena의 ID를 관리하고 빠르게 조회 가능               |
| `find()`               | Arena 자체에는 `find()` 메서드 없음 → 직접 순회하거나 Map 사용 필요 |

---
# id-arena Wrapper

id-arena를 래핑해서 다양한 검색 기능을 제공하는 커스텀 Arena 구조를 만든 예제입니다.  
이 구조는 다음 기능들을 포함합니다:
- ✅ 하나씩 찾기 (find_by_name)
- ✅ 전체 가져오기 (all)
- ✅ 여러 개 찾기 (find_many)
- ✅ 데이터로부터 ID 얻기 (find_id)

## 🧱 CustomArena 구조 정의
```rust
use id_arena::{Arena, Id};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Item {
    name: String,
    value: i32,
}

struct CustomArena {
    arena: Arena<Item>,
    name_to_id: HashMap<String, Id<Item>>,
}

impl CustomArena {
    fn new() -> Self {
        Self {
            arena: Arena::new(),
            name_to_id: HashMap::new(),
        }
    }

    fn insert(&mut self, item: Item) -> Id<Item> {
        let id = self.arena.alloc(item.clone());
        self.name_to_id.insert(item.name.clone(), id);
        id
    }

    fn find_by_name(&self, name: &str) -> Option<&Item> {
        self.name_to_id.get(name).map(|id| &self.arena[*id])
    }

    fn find_id(&self, name: &str) -> Option<Id<Item>> {
        self.name_to_id.get(name).copied()
    }

    fn find_many(&self, names: &[&str]) -> Vec<&Item> {
        names.iter()
            .filter_map(|name| self.find_by_name(name))
            .collect()
    }

    fn all(&self) -> Vec<&Item> {
        self.arena.iter().collect()
    }
}
```

### 🧪 사용 예제
```rust
fn main() {
    let mut store = CustomArena::new();

    store.insert(Item { name: "apple".into(), value: 100 });
    store.insert(Item { name: "banana".into(), value: 200 });
    store.insert(Item { name: "cherry".into(), value: 300 });

    // 하나씩 찾기
    if let Some(item) = store.find_by_name("banana") {
        println!("Found: {:?}", item);
    }

    // ID 얻기
    if let Some(id) = store.find_id("cherry") {
        println!("Cherry ID: {:?}", id);
    }

    // 여러 개 찾기
    let results = store.find_many(&["apple", "cherry"]);
    for item in results {
        println!("Batch found: {:?}", item);
    }

    // 전체 가져오기
    for item in store.all() {
        println!("All: {:?}", item);
    }
}
```

## ✅ 요약 기능표
| 기능 이름         | 메서드 이름        | 설명                                       |
|------------------|--------------------|--------------------------------------------|
| 하나 찾기        | `find_by_name()`   | 이름으로 객체를 조회                        |
| ID 얻기          | `find_id()`        | 이름으로 Arena ID를 조회                    |
| 여러 개 찾기     | `find_many()`      | 이름 배열로 여러 객체를 한 번에 조회         |
| 전체 가져오기    | `all()`            | Arena에 저장된 모든 객체를 반환              |


## ✅ 제네릭 함수 예제
```rust
fn echo<T>(value: T) -> T {
    value
}

fn main() {
    let a = echo(42);
    let b = echo("hello");
    println!("{}, {}", a, b);
}
```
- T는 어떤 타입이든 받을 수 있는 제네릭 타입
- 반환 타입도 T로 동일하게 설정

## ✅ 제네릭 구조체 예제
```rust
struct Wrapper<T> {
    value: T,
}

fn main() {
    let int_wrap = Wrapper { value: 10 };
    let str_wrap = Wrapper { value: "Rust" };

    println!("int: {}, str: {}", int_wrap.value, str_wrap.value);
}
```

- Wrapper<T>는 어떤 타입이든 감쌀 수 있어요
- 라이프타임 없이 단순한 값만 다룰 때는 아주 깔끔하게 작동합니다

## ✅ 제네릭 + 메서드 예제
```rust
impl<T> Wrapper<T> {
    fn get(&self) -> &T {
        &self.value
    }
}
```

- get() 메서드는 내부 값을 참조로 반환
- 라이프타임 생략 가능: Rust가 자동 추론해줍니다

## 🧠 요약
| 항목               | 설명                                               |
|--------------------|----------------------------------------------------|
| `fn echo<T>`        | 제네릭 함수. 어떤 타입이든 받아서 그대로 반환         |
| `struct Wrapper<T>` | 제네릭 구조체. 다양한 타입을 감싸는 컨테이너 역할     |
| `impl<T>`           | 구조체에 제네릭 메서드를 추가할 수 있는 기본 문법     |



## 🧩 Trait + 제네릭 구조체 예제
```rust
// Trait 정의: Printable
trait Printable {
    fn print(&self);
}

// 구조체 정의: Wrapper<T>
struct Wrapper<T> {
    value: T,
}

// Trait 구현: Wrapper<T>가 T가 Printable일 때만 Printable을 구현
impl<T: Printable> Printable for Wrapper<T> {
    fn print(&self) {
        self.value.print();
    }
}

// 예시 타입: Item
struct Item {
    name: String,
    value: i32,
}

// Item에 Printable 구현
impl Printable for Item {
    fn print(&self) {
        println!("Item: {} = {}", self.name, self.value);
    }
}

// 사용 예제
fn main() {
    let item = Item { name: "apple".into(), value: 100 };
    let wrapped = Wrapper { value: item };

    wrapped.print(); // Wrapper가 내부 Item의 print()를 호출
}
```
## ✅ 핵심 포인트
| 항목                 | 설명                                               |
|----------------------|----------------------------------------------------|
| `trait Printable`     | 공통 행위를 정의하는 trait (`print()` 메서드)       |
| `impl<T: Printable>`  | T가 Printable일 때만 Wrapper<T>도 Printable 구현    |
| `wrapped.print()`     | Wrapper가 내부 Printable 타입의 print()를 호출함   |

---

## 🧩 Arena + Trait 예제: Actionable을 구현한 노드들

### 1. Trait 정의
```rust
trait Actionable {
    fn act(&self);
}
```


### 2. Arena에 저장할 타입 정의
```rust
struct Task {
    name: String,
}

impl Actionable for Task {
    fn act(&self) {
        println!("Running task: {}", self.name);
    }
}
```

## 3. Arena에 저장하고 trait 호출
```rust
use id_arena::{Arena, Id};

fn main() {
    let mut arena = Arena::new();

    let id1 = arena.alloc(Task { name: "Compile".into() });
    let id2 = arena.alloc(Task { name: "Test".into() });
    let id3 = arena.alloc(Task { name: "Deploy".into() });

    // Arena에 저장된 객체들에 대해 trait 메서드 호출
    for id in [id1, id2, id3] {
        let task = &arena[id];
        task.act(); // ✅ Actionable trait 호출
    }
}
```

## ✅ 핵심 포인트
| 항목               | 설명                                               |
|--------------------|----------------------------------------------------|
| `trait Actionable` | 공통 행위를 정의하는 trait (`act()` 메서드)         |
| `impl Actionable`  | Arena에 저장할 타입이 trait을 구현                  |
| `arena[id].act()`  | Arena에서 꺼낸 객체에 trait 메서드를 호출함         |



## 🌱 객체 지향 → 행위 중심 전환의 핵심
| 객체 지향 사고       | 행위 중심 사고 (Rust 스타일)             |
|----------------------|-------------------------------------------|
| 객체가 상태와 행위를 모두 가짐 | 데이터는 구조체, 행위는 trait로 분리           |
| 상속 기반 다형성      | trait 기반의 명시적 구현과 조합            |
| 캡슐화된 객체         | Arena나 구조체에 저장된 값 + trait로 행동 정의 |
| 메서드 호출 중심      | trait 메서드 호출 또는 함수형 스타일 처리     |

---


