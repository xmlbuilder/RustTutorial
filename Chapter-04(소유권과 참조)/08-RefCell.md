# RefCell
Rust의 RefCell<T>와 내부 가변성 패턴에 대해 전체적으로 정리.  
이 개념은 Rust의 안전성과 유연성을 동시에 다루는 핵심적인 부분이라, 제대로 이해하면 Rust의 메모리 모델을 훨씬 깊이 있게 다룰 수 있음.

## 🧠 내부 가변성 (Interior Mutability)이란?
Rust는 기본적으로 불변 참조(&T)로는 값을 변경할 수 없도록 설계되어 있습니다.  
하지만 때로는 불변 참조를 통해서도 내부 데이터를 변경하고 싶을 때가 있죠. 이럴 때 사용하는 패턴이 바로 내부 가변성입니다.
- RefCell<T>는 불변 참조를 통해서도 내부 값을 변경할 수 있게 해주는 타입입니다.
- 이는 Rust의 컴파일 타임 빌림 규칙을 런타임 검사로 대체함으로써 가능해집니다.
- 내부적으로는 unsafe 코드를 사용하지만, API는 안전하게 제공됩니다.

## 🔍 RefCell<T>의 특징 요약
| 특징 항목             | RefCell<T> 설명                                      |
|----------------------|-----------------------------------------------------|
| 소유권               | 단일 소유자                                          |
| 빌림 검사 시점       | **런타임**에 검사                                     |
| 불변 빌림            | 가능 (`borrow()`)                                    |
| 가변 빌림            | 가능 (`borrow_mut()`), 단 **동시에 하나만 허용**       |
| 내부 가변성          | ✅ 지원 (불변 참조로도 내부 값 변경 가능)             |
| 스레드 안전성        | ❌ 단일 스레드에서만 사용 가능                        |
| 규칙 위반 시 결과     | **`panic!` 발생** (런타임에 중복 가변 빌림 등 위반 시) |


#### RefCell<T>는 컴파일 타임에 에러가 발생하지 않기 때문에, 실수로 중복 가변 빌림을 하면 런타임에서 프로그램이 종료될 수 있습니다.


## 🧪 RefCell<T> vs Box<T> vs Rc<T>
| 스마트 포인터 | 소유권         | 여러 소유권 시점         | 변경 가능성 검사 시점 | 내부 가변성 | 스레드 안전성 |
|----------------|----------------|---------------------------|------------------------|--------------|----------------|
| `Box<T>`       | 단일           | ❌                        | 컴파일 타임            | ❌ 없음       | ✅ 안전함       |
| `Rc<T>`        | 복수 (참조 카운트) | ✅ 런타임에 확인           | 컴파일 타임            | ❌ 없음       | ❌ 단일 스레드 |
| `RefCell<T>`   | 단일           | ❌                        | **런타임**              | ✅ 있음       | ❌ 단일 스레드 |


## 🛠 코드 예시 1: Rc<Owner>와 RefCell<Vec<Tool>> 조합
### 전체 코드
```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Owner  {
    name: String,
    tools: RefCell<Vec<Tool>>,
}

struct Tool {
    name: String,
    owner: Rc<Owner>,
}

fn main() {

    let indo = Rc::new(Owner{
        name: "indo".to_string(),
        tools: RefCell::new(Vec::new()),
    });

    let plier = Tool {
        name: "plier".to_string(),
        owner: indo.clone(),
    };

    let wrench = Tool{
        name: "wrench".to_string(),
        owner: indo.clone(),
    };

    indo.tools.borrow_mut().push(plier);
    indo.tools.borrow_mut().push(wrench);
    for tool in indo.tools.borrow().iter() {
        println!("{} {}", tool.name, tool.owner.name);
    }

    /*
    plier indo
    wrench indo
     */
}



```

### 소스 설명

```rust
struct Owner {
    name: String,
    tools: RefCell<Vec<Tool>>,
}

struct Tool {
    name: String,
    owner: Rc<Owner>,
}
```
- Owner는 tools 필드를 RefCell로 감싸서 불변 참조로도 툴 목록을 수정할 수 있게 합니다.
- Tool은 Rc<Owner>를 통해 공유 소유권을 가집니다.
```rust
indo.tools.borrow_mut().push(plier);
indo.tools.borrow_mut().push(wrench);
```

- borrow_mut()을 통해 Vec<Tool>에 툴을 추가합니다.
- borrow()를 통해 툴 목록을 읽어올 수 있습니다.

## 🌳 코드 예시 2: 트리 구조에서 Rc<RefCell<T>> 활용
### 전체코드
```rust

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::vec::Vec;

type Wrapper<T> = Rc<RefCell<T>>;

fn wrap<T>(data : T) -> Wrapper<T> {
    Rc::new(RefCell::new(data))
}

#[derive(Debug)]
struct Node<T>{
    data : T,
    children: Vec<Wrapper<Node<T>>>
}

impl <T : Display> Node<T>{
    fn add_child(&mut self, child : Wrapper<Node<T>>){
        self.children.push(child);
    }

    fn new(data : T) -> Node<T>{
        Node {
            data,
            children: Vec::new()
        }
    }

    fn depth_first(&self) {
        println!("node {}", self.data);
        for child in self.children.iter(){
            child.borrow().depth_first();
        }
    }
}

fn main() {

    let a = wrap(Node::new("a"));
    let b = wrap(Node::new("b"));
    let c = wrap(Node::new("c"));
    let d = wrap(Node::new("d"));
    a.borrow_mut().add_child(Rc::clone(&b));
    b.borrow_mut().add_child(Rc::clone(&c));
    c.borrow_mut().add_child(Rc::clone(&d));
    a.borrow_mut().depth_first()
}

```

### 코드 설명
```rust
type Wrapper<T> = Rc<RefCell<T>>;
```

- 트리 구조에서 각 노드는 Rc<RefCell<Node<T>>>로 감싸져 있어 공유 소유권과 내부 가변성을 동시에 제공합니다.
```rust
a.borrow_mut().add_child(Rc::clone(&b));
b.borrow_mut().add_child(Rc::clone(&c));
```

- 각 노드가 자식 노드를 추가할 수 있고,
- depth_first()를 통해 깊이 우선 탐색을 수행합니다.


## 코드 예시 3:
### 전체 소스
```rust
// 벡터로 소유권이 이동한 alice의 소유권을 다시 벡터로 대여 할 수 없음

use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let alice = RefCell::new(Person {
        name: "Alice".to_string(),
        age: 30,
    });

    let people = vec![alice]; 
    for person in &people {
        person.borrow_mut().age += 1;
    }

    println!("Alice is now {} years old", alice.borrow().age);
}
```
###  출력 결과
```
error[E0382]: borrow of moved value: `alice`
  --> src/main.rs:15:43
   |
7  |     let alice = RefCell::new(Person {
   |         ----- move occurs because `alice` has type `RefCell<Person>`,
   |               which does not implement the `Copy` trait
11 |     let people = vec![alice];
   |                          ^^^^ value moved here
...
15 |     println!("Alice is now {} years old", alice.borrow().age);
   |                                           ^^^^^ value borrowed here after move
```

### 수정 코드
```rust

// Rc<RefCell<T>>를 사용하면 여러 소유자가 값을 변경 가능

use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let alice = Rc::new(RefCell::new(Person {
        name: "Alice".to_string(),
        age: 30,
    }));

    let people = vec![alice.clone()]; // `Rc`를 복제해서 소유권을 공유
    for person in &people {
        person.borrow_mut().age += 1; // 가변 소유권 대여
    }

    println!("Alice is now {} years old", alice.borrow().age);
}
```


### ⚠️ RefCell 사용 시 주의점
- 가변 빌림은 단 한 번만 가능합니다. 중복 가변 빌림 시 런타임에서 panic!이 발생합니다.
- 예시:
```rust
let mut borrow1 = indo.tools.borrow_mut();
let mut borrow2 = indo.tools.borrow_mut(); // ❌ 런타임 에러 발생
```
- 멀티스레드 환경에서는 사용 불가합니다. 이 경우 Mutex<T>를 사용해야 합니다.

## ✅ 정리
RefCell<T>는 Rust에서 불변 참조로도 내부 값을 변경할 수 있게 해주는 유일한 안전한 방법입니다.  
하지만 런타임 검사에 의존하기 때문에, 사용 시에는 빌림 규칙을 철저히 지켜야 하며, 멀티스레드 환경에서는 절대 사용하지 말아야 합니다.

---

## Arc/Mutex

| 스마트 포인터 / 동기화 타입 | 소유권            | 여러 소유권 시점         | 변경 가능성 검사 시점 | 내부 가변성 | 스레드 안전성 | 주요 용도 |
|----------------------------|-------------------|--------------------------|----------------------|-------------|---------------|-----------|
| `Box<T>`                   | 단일              | ❌                       | 컴파일 타임           | ❌ 없음      | ✅ 안전        | 힙에 데이터 저장, 단일 소유 |
| `Rc<T>`                    | 복수 (참조 카운트)| ✅ 런타임에 확인          | 컴파일 타임           | ❌ 없음      | ❌ 단일 스레드 | 단일 스레드에서 공유 소유 |
| `RefCell<T>`               | 단일              | ❌                       | **런타임**            | ✅ 있음      | ❌ 단일 스레드 | 단일 스레드에서 내부 가변성 |
| `Arc<T>`                   | 복수 (원자적 참조 카운트) | ✅ 런타임에 확인  | 컴파일 타임           | ❌ 없음      | ✅ 멀티스레드  | 멀티스레드에서 공유 소유 |
| `Mutex<T>`                 | 단일 (락으로 보호)| ❌ (락을 통해 단일 접근)  | 런타임 (락 획득 시)   | ✅ 있음      | ✅ 멀티스레드  | 멀티스레드에서 내부 가변성 |
| `Arc<Mutex<T>>`            | 복수              | ✅ 런타임 + 락으로 제어   | 런타임 (락 획득 시)   | ✅ 있음      | ✅ 멀티스레드  | 멀티스레드에서 공유 + 변경 |

## 📌 핵심 정리
- Box<T> → 단일 소유, 힙에 저장, 가장 단순한 스마트 포인터
- Rc<T> → 단일 스레드에서만 여러 소유 가능 (읽기 전용 공유)
- RefCell<T> → 단일 스레드에서 내부 가변성 제공 (런타임 빌림 검사)
- Arc<T> → 멀티스레드에서 여러 소유 가능 (읽기 전용 공유)
- Mutex<T> → 멀티스레드에서 내부 가변성 제공 (락 기반)
- Arc<Mutex<T>> → 멀티스레드에서 여러 소유 + 변경 가능 (가장 흔한 패턴)

💡 예를 들어,
- 싱글 스레드에서 여러 곳이 데이터를 수정해야 한다면 → Rc<RefCell<T>>
- 멀티 스레드에서 여러 곳이 데이터를 수정해야 한다면 → Arc<Mutex<T>>


## 🧩 Arc<Mutex<T>> 예제 — 멀티스레드에서 안전한 공유 + 변경
```rust
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // Arc<Mutex<T>>로 감싸서 멀티스레드 안전한 공유 소유권 + 내부 가변성 제공
    let alice = Arc::new(Mutex::new(Person {
        name: "Alice".to_string(),
        age: 30,
    }));

    let mut handles = vec![];

    // 5개의 스레드가 동시에 age를 증가
    for i in 0..5 {
        let alice_clone = Arc::clone(&alice);
        let handle = thread::spawn(move || {
            let mut person = alice_clone.lock().unwrap(); // Mutex 잠금
            person.age += 1;
            println!("Thread {i} updated age to {}", person.age);
        });
        handles.push(handle);
    }

    // 모든 스레드가 끝날 때까지 대기
    for handle in handles {
        handle.join().unwrap();
    }

    // 최종 결과 출력
    let person = alice.lock().unwrap();
    println!("Final: {:?} is now {} years old", person.name, person.age);
}
```


## 🔍 코드 설명
- Arc<T> → 멀티스레드에서 참조 카운트를 증가시켜 여러 소유자가 가능하게 함
- Mutex<T> → 한 번에 하나의 스레드만 데이터에 접근하도록 보호
- Arc<Mutex<T>> → 멀티스레드에서 공유 소유권 + 내부 가변성 제공
- lock() → Mutex 잠금, unwrap()은 잠금 실패 시 패닉 처리
- thread::spawn() → 여러 스레드에서 동시에 접근

## 💡 실행 예시 (출력 순서는 스레드 스케줄링에 따라 달라짐)
```
Thread 0 updated age to 31
Thread 1 updated age to 32
Thread 2 updated age to 33
Thread 3 updated age to 34
Thread 4 updated age to 35
Final: "Alice" is now 35 years old

```



