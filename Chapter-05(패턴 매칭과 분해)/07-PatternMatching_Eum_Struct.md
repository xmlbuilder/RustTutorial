# eum + struct + pattern matching

Rust에서는 enum에 구조체를 넣고 패턴 매칭으로 내부 필드를 직접 분해할 수 있음.  
이건 Rust의 강력한 데이터 표현력 + 안전한 제어 흐름을 동시에 보여주는 기능.

## 🧠 개념 요약: Enum + Struct + Pattern Matching
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },         // 구조체 형태
    Write(String),                   // 튜플 형태
    ChangeColor(u8, u8, u8),         // 튜플 형태
}
```
- Move는 구조체 형태로 필드가 있음 → { x, y }로 분해 가능
- Write, ChangeColor는 튜플 형태 → (value)로 분해

## ✅ 실전 예제: 구조체 형태의 Enum 분해
```rust
enum Command {
    Move { x: i32, y: i32 },
    Resize { width: u32, height: u32 },
    Quit,
}

fn execute(cmd: Command) {
    match cmd {
        Command::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
        }
        Command::Resize { width, height } => {
            println!("Resizing to {}x{}", width, height);
        }
        Command::Quit => {
            println!("Quitting...");
        }
    }
}

fn main() {
    let cmd1 = Command::Move { x: 10, y: 20 };
    let cmd2 = Command::Resize { width: 800, height: 600 };
    let cmd3 = Command::Quit;

    execute(cmd1);
    execute(cmd2);
    execute(cmd3);
}
```


### 🔍 결과
```
Moving to (10, 20)
Resizing to 800x600
Quitting...
```

## ✨ 장점 – Enum + Struct + Pattern Matching

| 기능                  | 설명                                      |
|-----------------------|-------------------------------------------|
| 구조체 형태 표현 가능   | `Move { x, y }`처럼 명확한 필드 이름 사용 가능 |
| 패턴 매칭으로 필드 추출 | `match Command::Move { x, y } => …` 형태로 바로 분해 가능 |
| 안전한 제어 흐름       | 모든 variant를 exhaustively 처리 가능 → 컴파일러가 누락 방지 |

## 📌 핵심:
- enum에 구조체 형태를 넣으면 가독성 + 안전성 + 유지보수성이 모두 향상됨
- match에서 직접 필드를 꺼내 쓸 수 있어서 로직이 간결하고 명확함
- 컴파일러가 모든 경우를 체크해주기 때문에 런타임 오류 없이 안정적


## 💡 요약
```rust
enum MyEnum {
    StructVariant { a: i32, b: i32 },
    TupleVariant(String),
    UnitVariant,
}

match value {
    MyEnum::StructVariant { a, b } => { /* a, b 사용 */ }
    MyEnum::TupleVariant(s) => { /* s 사용 */ }
    MyEnum::UnitVariant => { /* 처리 */ }
}
```

---

# 상태 머신 + 비동기 메시지 처리 샘플

## 🧠 구조 개요
```
┌────────────┐                             ┌────────────────┐ 
│  Sender    │───▶ async mpsc channel ───▶│ Receiver(Task) │
└────────────┘                             └────────────────┘
        │                                        │
        ▼                                        ▼
  enum Message                            enum State
        │                                        │
        ▼                                        ▼
  match msg                                  match state

```


## ✅ 실전 예제: 상태 머신 + 비동기 메시지 처리
```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
```
```rust
#[derive(Debug)]
enum Message {
    Start,
    Stop,
    Move { x: i32, y: i32 },
}
```
```rust
#[derive(Debug)]
enum State {
    Idle,
    Running { x: i32, y: i32 },
    Stopped,
}
```
```rust
async fn state_machine(mut rx: mpsc::Receiver<Message>) {
    let mut state = State::Idle;

    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Start => {
                println!("Received Start");
                state = State::Running { x: 0, y: 0 };
            }
            Message::Stop => {
                println!("Received Stop");
                state = State::Stopped;
            }
            Message::Move { x, y } => {
                println!("Received Move to ({}, {})", x, y);
                if let State::Running { .. } = state {
                    state = State::Running { x, y };
                }
            }
        }

        println!("Current State: {:?}", state);
    }
}
```
```rust
#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(state_machine(rx));

    tx.send(Message::Start).await.unwrap();
    sleep(Duration::from_millis(100)).await;

    tx.send(Message::Move { x: 10, y: 20 }).await.unwrap();
    sleep(Duration::from_millis(100)).await;

    tx.send(Message::Stop).await.unwrap();
}

```

## ✨ 장점 – 상태 머신 + 비동기 메시지 처리

| 구성 요소        | 장점 설명                                      |
|------------------|-----------------------------------------------|
| `enum Message`   | 다양한 명령을 타입 안전하게 표현 가능             |
| `enum State`     | 상태를 명확하게 정의하고 match로 안전하게 분기 가능 |
| `tokio::mpsc`    | 비동기 메시지 전달 → 병렬 처리 및 decoupling 가능 |
| `tokio::spawn`   | 상태 머신을 별도 task로 실행 → 논리적 분리 및 확장성 확보 |



## 💡 확장 아이디어
- Message::Tick → 주기적 상태 업데이트
- State::Error → 에러 상태 표현
- match state 내부에서 match msg로 이중 분기 가능
- select! 매크로로 여러 채널 동시 처리


## 📌 핵심:
- enum을 활용하면 상태와 명령을 명확하게 표현할 수 있고,
- tokio::mpsc와 spawn을 조합하면 비동기 이벤트 기반 시스템을 안전하게 구성할 수 있어요.

---

# Rust / Java Enum 비교

Rust와 Java 모두 enum에 데이터를 넣을 수 있지만, 철학과 구현 방식은 꽤 다릅니다.  
Rust는 패턴 매칭 중심의 타입 안전한 분기,  
Java는 객체 지향 기반의 enum 확장이라는 접근을 취함.

## 🧠 핵심 차이점 요약 – Rust vs Java의 enum
| 항목               | Rust                                      | Java                                      |
|--------------------|-------------------------------------------|-------------------------------------------|
| 데이터 표현 방식     | `enum Variant { field1, field2 }`         | 생성자와 필드로 enum 확장 가능              |
| 분기 처리 방식       | `match`로 패턴 매칭                       | `switch` 또는 메서드 오버라이딩             |
| 분기 강제성          | `exhaustive` – 모든 경우를 컴파일러가 검사   | 선택적 처리 가능 – 누락해도 컴파일 가능      |
| 표현력              | 각 variant마다 다른 구조체/튜플 가능        | 모든 variant가 동일한 필드 구조여야 함       |
| 런타임 비용          | 값 기반, zero-cost abstraction            | enum은 클래스 → 메서드 호출 비용 존재        |
| 사용 목적            | 상태 머신, 메시지, AST 등 표현에 최적화     | 상수 집합 + 일부 동작 정의에 적합             |


## ✅ 예시 비교
### 🔷 Rust
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```
```rust
fn handle(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}
```

### 🔶 Java
```java
enum Message {
    QUIT,
    MOVE(10, 20),
    WRITE("Hello");

    int x, y;
    String text;

    Message() {}
    Message(int x, int y) { this.x = x; this.y = y; }
    Message(String text) { this.text = text; }
}
```

- Java는 enum에 생성자와 필드를 넣을 수 있지만, 모든 variant가 같은 필드를 공유해야 함
- Rust는 variant마다 완전히 다른 타입과 구조를 가질 수 있음

## ✨ Rust의 강점
- match를 통해 모든 경우를 컴파일 타임에 강제 처리
- 각 variant가 독립적인 타입 구조를 가질 수 있음
- enum 자체가 **값(value)** 이기 때문에 런타임 비용이 거의 없음

## 💡 요약 – Rust의 enum과 상태 설계
| 주제               | 핵심 요점                                      |
|--------------------|-----------------------------------------------|
| Rust의 enum         | 각 variant에 서로 다른 구조체/데이터를 담을 수 있음 |
| Pattern Matching    | `match`로 variant를 안전하게 분기하고 필드 추출 가능 |
| 상태 머신           | `enum State`로 상태 표현, `match`로 전이 처리       |
| 비동기 메시지 처리   | `tokio::mpsc` + `tokio::spawn`으로 안전한 이벤트 처리 |
| Java와의 차이점      | Rust는 값 기반 + 타입 안전, Java는 클래스 기반 enum |

---

# enum 상태 확인 

Rust에서는 enum이 현재 어떤 variant인지 확인하고, 그 안의 데이터를 꺼내는 방법이 패턴 매칭 외에도 몇 가지 있음.  
하지만 중요한 건: Rust는 런타임에 enum의 variant 이름을 직접 문자열로 얻는 기능은 기본적으로 제공하지 않습니다.  
대신, 패턴 매칭 + helper 메서드 + matches! 매크로 등을 활용해서 안전하게 확인할 수 있음.  

## ✅ 방법 1: match로 variant 확인 + 데이터 추출
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn check(msg: &Message) {
    match msg {
        Message::Quit => println!("It's Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}
```

## ✅ 방법 2: matches! 매크로로 variant 확인
```rust
if matches!(msg, Message::Quit) {
    println!("It's Quit");
}
```


## ✅ 방법 3: if let으로 특정 variant만 확인
```rust
if let Message::Move { x, y } = msg {
    println!("Moving to ({}, {})", x, y);
}
```

## ✅ 방법 4: helper 메서드로 variant 확인
```rust
impl Message {
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }

    fn as_write(&self) -> Option<&String> {
        if let Message::Write(s) = self {
            Some(s)
        } else {
            None
        }
    }
}
```


## ❌ 불가능한 것: 런타임에 variant 이름을 문자열로 얻기
Rust는 C++이나 Java처럼 enum.toString() 같은 기능은 기본적으로 없음.  
하지만 Debug 트레이트를 구현하면 format!("{:?}", msg)로 문자열로 출력은 가능해요:  
```rust
println!("{:?}", msg); // "Move { x: 10, y: 20 }"
```

## 💡 요약 – enum의 상태 확인 및 데이터 추출 방법
| 방법         | 설명                                                  |
|--------------|-------------------------------------------------------|
| `match`      | 모든 variant를 안전하게 분기하고 내부 데이터를 추출할 수 있음 |
| `matches!`   | 특정 variant인지 간단히 확인할 수 있음 (`matches!(x, Enum::A)`) |
| `if let`     | 특정 variant만 처리할 때 간결하게 사용할 수 있음         |
| helper 메서드| `is_variant()`, `as_variant()` 등으로 API처럼 추상화 가능 |
| `Debug` 출력 | `format!("{:?}", enum)`으로 현재 상태를 문자열로 확인 가능 |

## 📌 핵심:
- Rust는 런타임에 enum의 이름을 직접 문자열로 얻는 기능은 없지만,  
패턴 매칭과 헬퍼 메서드로 안전하고 명확하게 상태를 확인할 수 있음.

---

# 패턴 매칭을 통해 함수와 연결

Rust의 enum은 함수를 직접 넣을 수는 없지만, 패턴 매칭을 통해 상황에 따라 함수가 동작하도록 설계하는 건 아주 자연스럽고 강력한 방식.

## 🧠 핵심 개념
Rust에서는 enum에 함수를 넣는 대신,  
각 variant에 따라 다른 동작을 수행하는 match 블록을 사용합니다.  
또는 trait를 구현해서 enum을 디스패치하는 방식도 가능해요.  

## ✅ 방법 1: match로 함수 호출 분기
```rust
enum Action {
    Print(String),
    Add(i32, i32),
    Quit,
}
```
```rust
fn handle(action: Action) {
    match action {
        Action::Print(msg) => println!("Message: {}", msg),
        Action::Add(a, b) => println!("Sum: {}", a + b),
        Action::Quit => println!("Quitting..."),
    }
}
```


## ✅ 방법 2: enum에 메서드 구현 (impl block)
```rust
enum Command {
    Greet(String),
    Multiply(i32, i32),
}
```
```rust
impl Command {
    fn execute(&self) {
        match self {
            Command::Greet(name) => println!("Hello, {}!", name),
            Command::Multiply(a, b) => println!("Product: {}", a * b),
        }
    }
}

fn main() {
    let cmd = Command::Greet("JungHwan".to_string());
    cmd.execute(); // Hello, JungHwan!
}
```


## ✅ 방법 3: Trait 기반 동작 분리
```rust
trait Runnable {
    fn run(&self);
}
```
```rust
enum Job {
    Email(String),
    Compute(i32),
}
```
```rust
impl Runnable for Job {
    fn run(&self) {
        match self {
            Job::Email(addr) => println!("Sending email to {}", addr),
            Job::Compute(n) => println!("Computing square: {}", n * n),
        }
    }
}
```

## 💡 요약 – enum 기반 동작 분기 전략

| 방식      | 설명                                                  |
|-----------|-------------------------------------------------------|
| `match`   | 각 variant에 따라 분기하고, 해당 함수나 로직을 직접 호출 |
| `impl`    | enum에 메서드를 붙여서 내부에서 match 처리 가능          |
| `trait`   | 공통 인터페이스로 추상화하고, enum에 trait 구현 가능     |

## 📌 핵심:
- match는 가장 직관적이고 강력한 방식
- impl은 enum을 객체처럼 다룰 수 있게 해줌
- trait는 여러 타입에 공통 동작을 부여할 때 유용
- 함수를 직접 넣는 대신, enum의 구조와 match를 활용해 동작을 연결하는 것이 Rust의 철학에 맞고, 타입 안전성과 유지보수성도 뛰어납니다.


---

# 실전 문제

## 🧠 1. Event Handler – 이벤트 기반 처리 구조
Rust에서는 이벤트를 enum으로 정의하고,  
각 이벤트에 대해 핸들러를 match 또는 trait로 처리하는 방식이 일반적입니다.
```rust
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Resize { width: u32, height: u32 },
}
```
```rust
fn handle_event(event: Event) {
    match event {
        Event::Click { x, y } => println!("Clicked at ({}, {})", x, y),
        Event::KeyPress(c) => println!("Key pressed: {}", c),
        Event::Resize { width, height } => println!("Resized to {}x{}", width, height),
    }
}
```

- 이벤트는 enum으로 표현
- 핸들러는 match 또는 trait로 분기

## 🧩 2. Plugin System – 동적으로 기능 확장
Rust는 정적 언어지만, trait 객체를 활용하면 런타임에 동적으로 기능을 확장할 수 있음.  
플러그인은 trait을 구현한 구조체로 만들고, Box<dyn Trait>로 관리합니다.
```rust
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, event: &Event);
}
```
```rust
struct Logger;
impl Plugin for Logger {
    fn name(&self) -> &str { "Logger" }
    fn execute(&self, event: &Event) {
        println!("[LOG] {:?}", event);
    }
}
```
```rust
struct ClickTracker;
impl Plugin for ClickTracker {
    fn name(&self) -> &str { "ClickTracker" }
    fn execute(&self, event: &Event) {
        if let Event::Click { x, y } = event {
            println!("Tracking click at ({}, {})", x, y);
        }
    }
}
```
- 플러그인은 Plugin trait을 구현
- Vec<Box<dyn Plugin>>으로 여러 플러그인을 등록 가능

## 🚀 3. Dynamic Dispatch – 런타임 분기 처리
Rust는 기본적으로 정적 디스패치지만,  
dyn Trait을 사용하면 런타임에 어떤 타입이 실행될지 결정할 수 있음.
```rust
fn dispatch_event(event: Event, plugins: &[Box<dyn Plugin>]) {
    for plugin in plugins {
        plugin.execute(&event);
    }
}
```

- Box<dyn Trait> → 런타임에 어떤 타입인지 결정
- plugin.execute() → 각 플러그인의 구현에 따라 동작

### ✅ 전체 흐름 예시
```rust
fn main() {
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(Logger),
        Box::new(ClickTracker),
    ];

    let event = Event::Click { x: 42, y: 99 };
    dispatch_event(event, &plugins);
}
```
## 💡 요약 – Rust 이벤트 핸들러 & 플러그인 시스템
| 구성 요소          | 설명                                                  |
|--------------------|-------------------------------------------------------|
| `enum Event`        | 다양한 이벤트를 타입 안전하게 표현 (`Click`, `KeyPress` 등) |
| `trait Plugin`      | 플러그인 인터페이스 정의 → 각 기능을 추상화 가능             |
| `Box<dyn Plugin>`   | 런타임에 다양한 플러그인을 등록하고 실행 가능 (동적 디스패치) |
| `dispatch_event`    | 이벤트를 모든 플러그인에 전달하는 핸들러 함수                |


---
