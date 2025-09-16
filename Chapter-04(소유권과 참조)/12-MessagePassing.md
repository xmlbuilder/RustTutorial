# message passing
Rust에서의 메시지 패싱(message passing) 개념과 mpsc 채널을 활용한 실전 예제

## 📬 메시지 패싱이란?
- 정의: 스레드 또는 액터들이 데이터를 담은 메시지를 서로 주고받는 방식
- 목적: 공유 메모리 대신 데이터를 이동시켜 동시성 문제를 회피
- Rust의 방식: std::sync::mpsc 모듈을 통해 채널 기반 메시지 전달을 구현

## 🧵 Rust의 채널 구조
| 구성 요소 | 주요 기능 또는 메서드         | 설명 |
|------------|-------------------------------|------|
| `Sender`   | `send(val)`                   | 메시지를 전송하며, 값의 소유권을 이동시킴 |
| `Receiver` | `recv()`, `try_recv()`        | 메시지를 수신 (`recv`은 블로킹, `try_recv`은 비차단) |
| `mpsc`     | `channel()`                   | Multiple Producer, Single Consumer 채널 생성 함수 |


💡 채널은 스트림처럼 동작하며, 송신자가 드롭되면 채널은 자동으로 닫힙니다.

## 🧪 기본 채널 생성
```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel(); // 채널 생성
}
```

- 아직 어떤 타입도 보내지 않았기 때문에 컴파일되지 않음
- tx.send(val)과 rx.recv() 또는 for val in rx로 사용

## 📤 단일 메시지 전송 & 수신
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // 메시지 전송
    });

    let received = rx.recv().unwrap(); // 메시지 수신
    println!("Got: {}", received);
}
```

- recv()는 블로킹 호출 → 메시지가 올 때까지 기다림
- send()는 Result<T, E> 반환 → 수신자가 없으면 SendError 발생

### ⚠️ 잘못된 예제: 전송 후 값 재사용
```rust
thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    println!("val is {}", val); // ❌ 이미 move됨
});

```

- val은 send()로 move됨 → 이후 사용 불가
- Rust는 데이터 불일치나 경쟁 조건을 방지하기 위해 이를 금지

## 🔁 반복 메시지 전송
```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

- rx는 반복자처럼 사용 가능
- 채널이 닫히면 반복 종료

## 🧬 MPSC: 여러 송신자 → 하나의 수신자
```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 송신자 복제

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx1.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec!["more", "messages", "for", "you"];
        for val in vals {
            tx.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

- tx.clone()을 통해 여러 스레드가 동일한 수신자에게 메시지 전송 가능
- 수신자는 순서 없이 도착한 메시지를 처리

## ⏳ 비차단 수신: try_recv()
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx.send(String::from("hello")).unwrap();
    });

    loop {
        println!("Waiting for the signal...");
        if let Ok(msg) = rx.try_recv() {
            println!("Message: {}", msg);
            break;
        }
        thread::sleep(Duration::from_millis(300));
    }
}
```

- try_recv()는 즉시 반환하며 메시지가 없으면 Err 반환
- 폴링 방식으로 메시지를 기다릴 수 있음

## 🧠 핵심 요약
| 기능              | 설명 |
|-------------------|------|
| `mpsc::channel()` | 송신자(Sender)와 수신자(Receiver)를 생성하는 함수. MPSC(Multiple Producer, Single Consumer) 구조를 제공 |
| `send(val)`       | 송신자가 값을 전송. 값의 소유권이 이동되며, 수신자가 없으면 `SendError` 발생 |
| `recv()`          | 수신자가 값을 블로킹 방식으로 기다림. 값이 오면 `Result<T, E>`로 반환 |
| `try_recv()`      | 수신자가 값을 비차단 방식으로 즉시 확인. 값이 없으면 `Err` 반환 |
| `for val in rx`   | 수신자를 반복자처럼 사용. 채널이 닫힐 때까지 수신된 값을 순차적으로 처리 |
| `tx.clone()`      | 송신자를 복제하여 여러 스레드에서 동일한 수신자에게 메시지를 보낼 수 있게 함 |

---

# *메시지 패싱 구조(mpsc) 확장

아래는 Rust의 **메시지 패싱 구조(mpsc)**를 기반으로 확장한 세 가지 예제입니다.  
각각은 실전에서 자주 쓰이는 패턴인 작업 분산, 이벤트 처리, 그리고 **비동기 채널(futures::channel)**을 다룹니다.

## 🧵 1. 스레드 간 작업 분산 (Work Distribution)
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 작업자 스레드 3개 생성
    for id in 0..3 {
        let rx = rx.clone(); // 수신자 복제
        thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                println!("Worker {id} processing task: {task}");
            }
        });
    }

    // 작업 분산
    for task_id in 1..=10 {
        tx.send(format!("Task #{task_id}")).unwrap();
    }

    // drop tx → 모든 worker 종료
    drop(tx);
}
```

### ✅ 설명:
- 하나의 송신자 → 여러 수신자(worker)로 작업 분산
- 각 스레드는 recv()로 작업을 받아 처리
- drop(tx)로 채널 종료 → worker 루프 종료

## 🔔 2. 이벤트 처리 시스템
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Event {
    Click(String),
    Timeout(u64),
    Shutdown,
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // 이벤트 생성 스레드
    thread::spawn(move || {
        tx.send(Event::Click("Button A".into())).unwrap();
        thread::sleep(Duration::from_secs(1));
        tx.send(Event::Timeout(1000)).unwrap();
        thread::sleep(Duration::from_secs(1));
        tx.send(Event::Shutdown).unwrap();
    });

    // 이벤트 처리 루프
    for event in rx {
        match event {
            Event::Click(label) => println!("Clicked: {label}"),
            Event::Timeout(ms) => println!("Timeout after {ms}ms"),
            Event::Shutdown => {
                println!("Shutting down...");
                break;
            }
        }
    }
}
```

### ✅ 설명:
- enum Event로 다양한 이벤트 정의
- 이벤트를 채널로 전송 → 메인 루프에서 처리
- Shutdown 이벤트로 종료 제어

## ⚡ 3. 비동기 채널 (futures::channel)
```rust
use futures::channel::mpsc;
use futures::stream::StreamExt;
use tokio::task;

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::unbounded();

    // 비동기 작업자 생성
    for id in 0..3 {
        let mut rx = rx.clone();
        task::spawn(async move {
            while let Some(msg) = rx.next().await {
                println!("Async worker {id} received: {msg}");
            }
        });
    }

    // 메시지 전송
    for i in 1..=5 {
        tx.send(format!("Async Task #{i}")).unwrap();
    }

    // 채널 닫기
    drop(tx);
}
```

### ✅ 설명:
- futures::channel::mpsc는 비동기 채널을 제공
- StreamExt::next()로 메시지를 await
- tokio::task::spawn으로 비동기 작업자 실행

## 🧠 확장 요약
| 확장 패턴           | 특징                          | 적합한 용도                 | 예시 방식                     |
|---------------------|-------------------------------|------------------------------|-------------------------------|
| 작업 분산           | 여러 worker가 작업을 나눠 처리 | 병렬 처리, 큐 기반 시스템     | `for task in tasks { tx.send(task) }` |
| 이벤트 처리         | 다양한 이벤트를 enum으로 처리 | UI 이벤트, 상태 전이 시스템   | `match event { ... }`         |
| 비동기 채널         | await 기반 메시지 전달         | 웹 서버, async 서비스 구조    | `futures::channel::mpsc` + `tokio::spawn` |

---



