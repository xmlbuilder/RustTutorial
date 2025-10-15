# Box, Rc, Arc, RefCell, Rc::strong_count

## 📦 Box<T> — 힙에 저장, 단일 소유
### ✅ 언제 사용하나요?
- 크기를 알 수 없는 타입 (dyn Trait, 재귀 구조 등)
- 큰 데이터를 스택 대신 힙에 저장하고 싶을 때
- 단일 소유권이 충분할 때
### 💡 예제
```rust
trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let pet: Box<dyn Animal> = Box::new(Dog);
    pet.speak(); // 출력: Woof!
}
```


## 🔁 Rc<T> — 참조 카운팅, 단일 스레드 공유
### ✅ 언제 사용하나요?
- 여러 소유자가 필요하지만 단일 스레드 환경일 때
- 불변 공유가 주 목적일 때
### 💡 예제
```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("Hello"));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("a: {}, b: {}", a, b); // 출력: Hello Hello
}
```


## 🧵 Arc<T> — 원자적 참조 카운팅, 멀티 스레드 공유
### ✅ 언제 사용하나요?
- 여러 스레드에서 데이터를 공유해야 할 때
- Rc는 스레드 안전하지 않기 때문에 Arc를 사용
### 💡 예제

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("{:?}", data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```


## 🔄 RefCell<T> — 내부 가변성, 런타임 체크
### ✅ 언제 사용하나요?
- 컴파일 타임에 불가능한 가변성을 런타임에 허용하고 싶을 때
- 단일 스레드 환경에서만 사용 가능
### 💡 예제
```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    *data.borrow_mut() += 1;
    println!("data = {}", data.borrow()); // 출력: data = 6
}
```


## 🔁🔄 Rc<RefCell<T>> — 여러 소유자 + 내부 가변성
### ✅ 언제 사용하나요?
- 여러 소유자가 있고, 그 데이터에 가변 접근이 필요한 경우
- 단일 스레드 환경에서만 사용 가능
### 💡 예제
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = Rc::new(RefCell::new(10));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    *a.borrow_mut() += 5;
    println!("b = {}", b.borrow()); // 출력: b = 15
}
```


## 🧵🔒 Arc<Mutex<T>> — 멀티 스레드 + 내부 가변성 + 동기화
### ✅ 언제 사용하나요?
- 여러 스레드에서 데이터를 공유하고, 동기화된 가변성이 필요할 때
## 💡 예제
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // 출력: Result: 5
}
```            


## 🧭 요약: 언제 어떤 포인터를 써야 할까?

| 상황 또는 목적                           | 추천 스마트 포인터     |
|----------------------------------------|------------------------|
| 단일 소유, 힙에 저장                   | `Box<T>`              |
| 여러 소유자 필요, 단일 스레드 환경     | `Rc<T>`               |
| 여러 소유자 필요, 멀티 스레드 환경     | `Arc<T>`              |
| 내부 가변성 필요, 단일 소유            | `RefCell<T>`          |
| 여러 소유자 + 내부 가변성 (단일 스레드) | `Rc<RefCell<T>`       |
| 멀티 스레드 + 내부 가변성 + 동기화 필요 | `Arc<Mutex<T>`        |


---


## 🧠 왜 이렇게 복잡해졌을까?
### 1. 메모리 안전성 보장 (Safety without Garbage Collector)
Rust는 GC(Garbage Collector) 없이도 메모리 안전성을 보장.  
이를 위해 컴파일 타임에 소유권과 수명(lifetime)을 체크하지만, 이로 인해 복잡한 상황에서는 더 정교한 도구가 필요.
- Box<T>: 힙에 저장하면서 단일 소유권 유지
- Rc<T>: 여러 소유자가 필요할 때
- RefCell<T>: 컴파일 타임에 불가능한 가변성을 런타임에 허용
  
### 2. 동시성 안전성 (Fearless Concurrency)
Rust는 멀티 스레드 프로그래밍을 안전하게 만듬.  
그래서 Arc<T>와 Mutex<T> 같은 도구들이 등장.  
- Arc<T>: 여러 스레드에서 안전하게 공유
- Arc<Mutex<T>>: 공유 + 가변성 + 동기화까지
- 
### 3. 성능 최적화
Rust는 C/C++ 수준의 성능을 목표.  
그래서 불필요한 런타임 오버헤드를 줄이고, 필요한 경우에만 정교한 도구를 쓰도록 설계.  
- Box<T>는 단순하고 빠름
- RefCell<T>는 런타임 체크가 있지만, 필요한 경우에만 사용
- 
### 4. 유연한 추상화
Rust는 재귀적 자료구조, 트레잇 객체, 동적 디스패치 등 다양한 프로그래밍 패턴을 지원하기 위해 스마트 포인터를 도입.

## 🧠 Rust 스마트 포인터들의 목적 정리
| 스마트 포인터       | 주요 목적 및 사용 이유                                      |
|---------------------|-------------------------------------------------------------|
| `Box<T>`            | 힙에 저장 + 단일 소유. 재귀 타입, 트레잇 객체 등에 사용됨     |
| `Rc<T>`             | 참조 카운팅으로 여러 소유자 허용 (단일 스레드 환경)           |
| `Arc<T>`            | 원자적 참조 카운팅으로 멀티 스레드에서 안전한 공유 가능       |
| `RefCell<T>`        | 내부 가변성 허용. 런타임에 borrow 체크 수행 (단일 스레드)     |
| `Rc<RefCell<T>>`    | 여러 소유자 + 내부 가변성. 단일 스레드에서 동적 상태 관리에 유용 |
| `Arc<Mutex<T>>`     | 멀티 스레드 + 내부 가변성 + 동기화. 스레드 간 안전한 공유 및 수정 |

Rust는 "안전하면서도 빠른 언어"라는 목표를 위해 이 복잡함을 감수.  
이제 이 도구들을 잘 활용하면, 버그 없는 고성능 프로그램을 만들 수 있는 강력한 무기.  


## 🛠️ 실전 프로젝트 시나리오별 스마트 포인터 선택
| 시나리오                             | 추천 스마트 포인터     | 이유 및 설명                                                                 |
|--------------------------------------|------------------------|------------------------------------------------------------------------------|
| 트리 구조, 재귀적 자료구조           | `Box<T>`              | 자기 참조 구조를 만들기 위해 힙에 저장 필요. 단일 소유로 충분함.              |
| GUI 상태 관리 (단일 스레드)          | `Rc<RefCell<T>>`      | 여러 컴포넌트가 상태를 공유하며, 내부 가변성이 필요함.                        |
| 웹 서버에서 공유 설정 데이터         | `Arc<T>`              | 여러 스레드에서 읽기만 하는 공유 데이터. 안전한 멀티 스레드 공유 가능.        |
| 멀티 스레드 작업 큐 (가변 접근 포함) | `Arc<Mutex<T>>`       | 여러 스레드가 작업 큐에 접근하고 수정해야 하므로 동기화 필요.                 |
| 게임 엔진의 전역 상태                | `Rc<RefCell<T>>`      | 단일 스레드에서 여러 시스템이 상태를 공유하고 수정함.                         |
| 병렬 처리 환경에서 공유 캐시         | `Arc<RwLock<T>>`      | 다중 읽기 + 단일 쓰기 가능. 캐시처럼 읽기 많은 구조에 적합.                   |
| 단순한 힙 저장 (예: 큰 배열)         | `Box<T>`              | 스택 대신 힙에 저장하여 오버플로 방지. 단일 소유로 충분함.                    |
| 런타임에만 가변성이 필요한 경우      | `RefCell<T>`          | 컴파일 타임에 불가능한 가변성을 런타임에 허용. 단일 스레드에서만 사용 가능.   |


## 🧠 추천 스마트 포인터 조합
| 목적 또는 상황                          | 추천 스마트 포인터 조합         | 설명                                                                 |
|----------------------------------------|----------------------------------|----------------------------------------------------------------------|
| 여러 객체가 하나의 데이터를 공유함       | `Rc<T>` / `Arc<T>`              | 단일 스레드에서는 `Rc`, 멀티 스레드에서는 `Arc` 사용                |
| 공유된 데이터에 가변 접근이 필요함       | `RefCell<T>` / `Mutex<T>`       | `RefCell`은 단일 스레드, `Mutex`는 멀티 스레드에서 내부 가변성 제공 |
| 트레잇 객체를 저장하고 다형성 활용       | `Box<dyn Trait>`                | 트레잇 객체를 힙에 저장하여 런타임 디스패치 가능                     |
| 여러 소유자 + 내부 가변성 (단일 스레드) | `Rc<RefCell<T>>`                | GUI, 이벤트 시스템 등에서 자주 사용됨                                |
| 여러 소유자 + 내부 가변성 (멀티 스레드) | `Arc<Mutex<T>>`                 | 멀티 스레드 환경에서 안전하게 공유 및 수정 가능                      |



## 🧪 샘플 코드: 단일 스레드용 Observer 패턴 (Rc<RefCell<T>> 사용)]
```rust
use std::rc::Rc;
use std::cell::RefCell;

// Observer 트레잇 정의
trait Observer {
    fn on_notify(&self, message: &str);
}

// Observable 구조체
struct Subject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify_all(&self, message: &str) {
        for obs in &self.observers {
            obs.borrow().on_notify(message);
        }
    }
}

// Observer 구현체
struct Logger;
impl Observer for Logger {
    fn on_notify(&self, message: &str) {
        println!("[Logger] Received: {}", message);
    }
}

struct Notifier;
impl Observer for Notifier {
    fn on_notify(&self, message: &str) {
        println!("[Notifier] Alert: {}", message);
    }
}

fn main() {
    let mut subject = Subject::new();

    let logger = Rc::new(RefCell::new(Logger));
    let notifier = Rc::new(RefCell::new(Notifier));

    subject.add_observer(logger);
    subject.add_observer(notifier);

    subject.notify_all("Data updated!");
}
```

## 🔍 실행 결과
[Logger] Received: Data updated!
[Notifier] Alert: Data updated!


## 🧵 멀티 스레드 환경에서는?
멀티 스레드 환경에서는 Arc<Mutex<T>>를 사용해야 함.
예: Arc<Mutex<Vec<Arc<dyn Observer>>>> 같은 구조로 동기화된 공유를 구현할 수 있음.


## 🧠 어떤 스마트 포인터를 써야 할까? (멀티 스레드 Observer 패턴)

| 목적 또는 역할                        | 추천 스마트 포인터                      | 설명                                                                 |
|-------------------------------------|-----------------------------------------|----------------------------------------------------------------------|
| 여러 스레드에서 안전하게 공유       | `Arc<T>`                                | 참조 카운팅을 통해 여러 스레드에서 안전하게 데이터 공유 가능         |
| 내부 가변성 + 동기화                | `Mutex<T>`                              | 데이터에 대한 동시 접근을 막고, 안전한 수정 가능                      |
| 트레잇 객체 저장 + 스레드 안전성    | `Box<dyn Observer + Send + Sync>`       | 다형성을 위해 트레잇 객체를 힙에 저장하며, 스레드 간 안전하게 공유 가능 |



## 🧪 예제 코드: Arc<Mutex<Vec<Box<dyn Observer>>>> 기반 Observer 패턴
```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Observer 트레잇 정의
trait Observer: Send + Sync {
    fn on_notify(&self, message: &str);
}

// Subject 구조체
struct Subject {
    observers: Arc<Mutex<Vec<Box<dyn Observer>>>>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.lock().unwrap().push(observer);
    }

    fn notify_all(&self, message: &str) {
        let observers = self.observers.lock().unwrap();
        for obs in observers.iter() {
            obs.on_notify(message);
        }
    }

    fn get_observers_arc(&self) -> Arc<Mutex<Vec<Box<dyn Observer>>>> {
        Arc::clone(&self.observers)
    }
}

// Observer 구현체
struct Logger;
impl Observer for Logger {
    fn on_notify(&self, message: &str) {
        println!("[Logger] Received: {}", message);
    }
}

struct Notifier;
impl Observer for Notifier {
    fn on_notify(&self, message: &str) {
        println!("[Notifier] Alert: {}", message);
    }
}

fn main() {
    let mut subject = Subject::new();

    subject.add_observer(Box::new(Logger));
    subject.add_observer(Box::new(Notifier));

    let observers_arc = subject.get_observers_arc();

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let obs_clone = Arc::clone(&observers_arc);
            thread::spawn(move || {
                let message = format!("Thread {} update!", i);
                let observers = obs_clone.lock().unwrap();
                for obs in observers.iter() {
                    obs.on_notify(&message);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```


### 🔍 실행 결과 예시
```
[Logger] Received: Thread 0 update!
[Notifier] Alert: Thread 0 update!
[Logger] Received: Thread 1 update!
[Notifier] Alert: Thread 1 update!
[Logger] Received: Thread 2 update!
[Notifier] Alert: Thread 2 update!
```


## ✅ 핵심 포인트
- Arc<Mutex<Vec<Box<dyn Observer>>>>로 스레드 안전한 공유 + 내부 가변성을 동시에 해결
- Box<dyn Observer + Send + Sync>로 다형성 + 스레드 안전성 확보
- 각 스레드가 Subject의 관찰자 목록에 접근하여 알림을 전송

---

- Vec<Rc<RefCell<dyn Observer>>>
- Arc<Mutex<Vec<Box<dyn Observer>>>>
  
이 둘은 Observer 패턴을 구현할 때 자주 등장하는 구조인데, 각각 단일 스레드용과 멀티 스레드용.  
아래에 자세히 설명.


## 🧵 1. Vec<Rc<RefCell<dyn Observer>>> — 단일 스레드용
### 🔍 구조 설명
- Vec<...>: 여러 Observer를 저장하는 리스트
- Rc<T>: 여러 소유자가 같은 Observer를 공유할 수 있게 함
- RefCell<T>: 내부 가변성을 허용 (Observer가 상태를 바꿀 수 있음)
- dyn Observer: 트레잇 객체로 다양한 Observer 타입을 저장
- 
### ✅ 언제 사용하나요?
- 단일 스레드 환경에서
- 여러 Observer가 등록되고, 각 Observer가 상태를 바꿀 수 있어야 할 때
- GUI, 게임 엔진, 이벤트 시스템 등에서 자주 사용
- 
### 💡 예시
```rust
let observers: Vec<Rc<RefCell<dyn Observer>>> = vec![];
```
이 구조는 Subject가 여러 Observer를 등록하고, 각 Observer가 자신의 상태를 바꾸거나 반응할 수 있도록 해줌.

## 🧵 2. Arc<Mutex<Vec<Box<dyn Observer>>>> — 멀티 스레드용
### 🔍 구조 설명
- Arc<T>: 여러 스레드에서 안전하게 공유
- Mutex<T>: 내부 가변성을 스레드 안전하게 보장
- Vec<...>: Observer 리스트
- Box<dyn Observer>: 트레잇 객체를 힙에 저장 (다형성)
  
### ✅ 언제 사용하나요?
- 멀티 스레드 환경에서
- 여러 스레드가 Observer를 등록하거나 알림을 보내야 할 때
- 웹 서버, 백엔드 시스템, 병렬 작업 처리 등
- 
### 💡 예시
```rust
let observers: Arc<Mutex<Vec<Box<dyn Observer + Send + Sync>>>> = Arc::new(Mutex::new(vec![]));
```

여기서 Send + Sync는 Observer가 스레드 간 안전하게 이동되고 공유될 수 있도록 보장.


## 🔁 비교 요약: Observer 패턴에서의 포인터 조합

| 구성 요소                        | `Vec<Rc<RefCell<dyn Observer>>>`            | `Arc<Mutex<Vec<Box<dyn Observer>>>>`               |
|----------------------------------|---------------------------------------------|----------------------------------------------------|
| 스레드 환경                      | 단일 스레드                                 | 멀티 스레드                                        |
| 공유 방식                        | `Rc` (참조 카운팅, 스레드 안전 아님)        | `Arc` (원자적 참조 카운팅, 스레드 안전)            |
| 내부 가변성                      | `RefCell` (런타임 borrow 체크)              | `Mutex` (스레드 안전한 동기화)                     |
| 트레잇 객체 저장 방식            | `dyn Observer` (트레잇 객체, 힙 저장 아님)   | `Box<dyn Observer + Send + Sync>` (힙에 저장 + 스레드 안전) |
| 다형성                           | 가능 (트레잇 객체)                          | 가능 (트레잇 객체 + 스레드 안전성 보장)            |
| 사용 예                          | GUI, 게임 엔진, 이벤트 시스템               | 서버, 백엔드, 병렬 작업 처리                       |


## 🧘‍♂️ 왜 이렇게 복잡할까?
Rust는 컴파일 타임에 안전성을 보장하려고 해요. 그래서 단순한 포인터 대신, 상황에 맞는 스마트 포인터 조합을 써야 함.  
처음엔 어렵지만, 이 구조를 이해하면 버그 없는 고성능 프로그램을 만들 수 있는 강력한 무기가 됨.

---

# Vec의 위치

Vec의 위치가 Vec<Rc<RefCell<dyn Observer>>>와 Arc<Mutex<Vec<Box<dyn Observer>>>>처럼 다르게 쓰이는 이유는 스마트 포인터의 역할과 소유권 구조에 따라 데이터를 어디에 두는지가 달라지기 때문.  
이걸 이해하면 Rust의 메모리 모델이 훨씬 명확해짐.

## 🔍 핵심 차이: Vec이 안에 있느냐, 밖에 있느냐
| 구조                          | Vec의 위치                  | 의미 및 설계 이유                                               |
|-------------------------------|-----------------------------|------------------------------------------------------------------|
| `Vec<Rc<RefCell<...>>>`       | 바깥쪽 (최상위에 위치)      | 리스트는 스택에 있고, 각 요소만 힙에 저장됨. 단일 스레드에서 충분 |
| `Arc<Mutex<Vec<...>>>`        | 안쪽 (`Mutex` 내부에 위치) | 리스트 전체를 힙에 저장하고, 동기화된 상태로 여러 스레드에서 공유 |

## 🧠 왜 이렇게 다르게 설계할까?

### 1. 단일 스레드 vs 멀티 스레드
- `Vec<Rc<RefCell<...>>>` 는 단일 스레드에서만 사용되므로, 리스트 자체를 스택에 두고 각 Observer만 힙에 저장하면 충분.
- `Arc<Mutex<Vec<...>>>` 는 멀티 스레드에서 사용되므로, 리스트 전체를 힙에 저장하고 동기화해야 함.  
그래서 Vec이 Mutex 안에 들어가고, 그걸 Arc로 감싸서 공유하는 구조가 됨.

### 2. 공유 범위의 차이
- `Vec<Rc<...>>` : 리스트는 한 곳에서만 관리하고, 각 Observer를 여러 곳에서 공유
- `Arc<Mutex<Vec<...>>>` : 리스트 자체를 여러 스레드에서 공유하고, 동시에 접근 가능하게 만들어야 함

## 🧪 비유로 이해해보면
- `Vec<Rc<...>>` 는 마치 한 사람이 명단을 들고 있고, 명단 안의 사람들(Observer)은 여러 사람이 공유하는 구조
- `Arc<Mutex<Vec<...>>>` 는 명단 자체를 금고에 넣고, 여러 사람이 금고를 열어서 명단을 보고 수정하는 구조


## ✅ 정리: Observer 패턴에서의 포인터 구조 비교

| 구조                          | 스레드 환경   | 공유 방식 | 리스트 위치 | 내부 가변성 방식 |
|-------------------------------|---------------|------------|---------------|------------------|
| `Vec<Rc<RefCell<...>>>`       | 단일 스레드   | `Rc`       | 바깥쪽 (`Vec`이 최상위) | `RefCell` (런타임 borrow 체크) |
| `Arc<Mutex<Vec<...>>>`        | 멀티 스레드   | `Arc`      | 안쪽 (`Vec`이 `Mutex` 안에 있음) | `Mutex` (스레드 안전한 동기화) |

----

# RefCell 보강

# Rc::make_mut() vs Rc<RefCell<T>>
→ 이건 Rust에서 불변 공유 vs 내부 가변성을 어떻게 설계할지에 대한 선택.

## 🧩 핵심 비교: Rc::make_mut() vs Rc<RefCell<T>>
| 항목                  | Rc::make_mut()                      | Rc<RefCell<T>>                      |
|-----------------------|-------------------------------------|-------------------------------------|
| 가변 접근 방식         | 참조 카운트가 1일 때만 직접 수정 가능 | 언제든지 `borrow_mut()`로 수정 가능 |
| 복제 여부             | 참조자가 여러 명이면 내부 데이터 복제 | 복제 없음, 내부 상태 공유           |
| 안전성                | 컴파일 타임 안전성                  | 런타임 체크 (panic 가능)             |
| 성능                  | 복제 비용 발생 가능                 | 런타임 체크 비용 있음                |
| 내부 구조             | `Rc<T>`                             | `Rc<RefCell<T>>`                    |
| mutable 접근 예시     | `Rc::make_mut(&mut rc)`             | `rc.borrow_mut()`                   |
| 스레드 안전성         | `Rc`: `!Send`, `!Sync`              | `RefCell`: `!Send`, `!Sync`         |


## ✅ 언제 어떤 걸 써야 할까?
### 🔹 Rc::make_mut()를 쓰는 경우
- 대부분 읽기만 하고, 특정 순간에만 수정이 필요할 때
- 복제해도 괜찮을 만큼 데이터가 작거나 clone이 가벼울 때
- 컴파일 타임 안전성을 유지하고 싶을 때
- 예: GUI 트리 구조, undo/redo 시스템, snapshot 기반 캐시
- 
### 🔹 Rc<RefCell<T>>를 쓰는 경우
- 자주 수정이 필요하고, 복제 비용이 부담될 때
- 여러 참조자가 동일한 내부 상태를 공유해야 할 때
- 런타임 체크를 감수하고라도 유연한 상태 관리가 필요할 때
- 예: 게임 엔티티 상태, 이벤트 핸들러, 동적 설정값

## 소유권 이전
RefCell<T>에서 borrow_mut()를 호출한다고 해서 소유권이 이전되는 건 아닙니다.  
    → 대신, 런타임에 mutable 참조를 "빌려오는" 것.
Rust의 소유권 모델은 여전히 유지되고, RefCell은 그걸 런타임 체크로 우회하는 구조입니다.

### 🧩 핵심 개념: borrow_mut()는 소유권 이전이 아니다
```rust
let cell = RefCell::new(10);
let mut_ref = cell.borrow_mut(); // &mut T가 아니라 RefMut<T>
```

- mut_ref는 RefMut<i32> 타입
- 내부적으로 RefCell이 mutable borrow가 발생했음을 기록  
    → 소유권은 여전히 RefCell이 가지고 있음
    → mut_ref는 일시적인 가변 접근 권한만 가진 것

### 🔍 왜 소유권 이전이 아닌가?
- Rust는 소유권 이전은 move로만 발생
- borrow_mut()는 move가 아니라 mutable borrow
- RefMut<T>는 Drop될 때 → 내부 borrow 상태를 해제  
    → 소유권은 절대 바뀌지 않음, 단지 가변 접근 권한을 잠시 빌려오는 것

### ✅ 예시로 확인
```rust
use std::cell::RefCell;
fn main() {
    let cell = RefCell::new(String::from("Hello"));
    {
        let mut s = cell.borrow_mut();
        s.push_str(" JungHwan");
    }
    println!("{}", cell.borrow()); // Hello JungHwan
}
```
- cell은 여전히 소유권을 가지고 있음
- s는 RefMut<String> → Drop되면 mutable borrow 해제  
    → 소유권은 이동하지 않고, 상태만 바뀜


##  borrow_mut() 아래에서 Rc::clone()을 호출하는 건 가능
borrow_mut() 아래에서 Rc::clone()을 호출하는 건 가능하지만, 구조적으로 조심해야 합니다.  
    → 왜냐하면 RefCell의 내부 상태를 mutable하게 빌린 상태에서 그걸 공유하려는 시도는 구조적 충돌을 일으킬 수 있기 때문입니다.

### 🔍 핵심 구조
```rust
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let rc = Rc::new(RefCell::new(42));
    {
        let mut_ref = rc.borrow_mut(); // 내부를 mutable하게 빌림
        // Rc::clone(&rc) 호출은 가능
        let rc2 = Rc::clone(&rc);      // 참조 카운트 증가
        println!("Cloned Rc while mutably borrowed");
    }
    println!("Done");
}
```

- ✅ 컴파일은 잘 됩니다
- ✅ Rc::clone()은 RefCell의 내부 borrow 상태와 무관함
- ❗ 하지만 의미적으로 조심해야 할 부분이 있음

### ⚠️ 왜 조심해야 하는가?
#### 1. mutable borrow 중에 공유가 일어남
- borrow_mut()는 내부 데이터를 독점적으로 빌리는 것
- 그 상태에서 Rc::clone()으로 다른 참조자를 만들면  
    → 그 참조자가 내부 상태를 건드릴 수 있다는 오해를 줄 수 있음
#### 2. 구조적 혼란 가능
- Rc::clone()은 안전하지만  
    → 그 아래에서 rc.borrow_mut()를 계속 유지한 채  
    → 다른 곳에서 borrow()나 borrow_mut()를 시도하면  
    → 런타임 panic 발생 가능  


### ✅ 방법: `std::mem::drop()` 으로 명시적 해제
```rust
use std::cell::RefCell;
use std::mem::drop;

fn main() {
    let cell = RefCell::new(42);

    let mut_ref = cell.borrow_mut();
    println!("Before drop: {}", *mut_ref);
    drop(mut_ref); // 명시적으로 mutable borrow 해제

    // 이후 다시 borrow 가능
    let again = cell.borrow();
    println!("After drop: {}", *again);
}
```

- drop(mut_ref)를 호출하면  
    → RefMut<T>가 즉시 해제됨  
    → 내부 borrow 상태가 즉시 풀림  
- 이후에 borrow()나 borrow_mut()를 다시 호출해도 panic 없이 동작

### 🔍 왜 이게 중요한가?
- 긴 스코프에서 mutable borrow를 짧게 쓰고 싶을 때
- 다시 borrow하려면 이전 borrow를 반드시 해제해야 할 때
- 특히 Rc<RefCell<T>> 구조에서  
    → borrow_mut()로 수정한 뒤  
    → Rc::clone()이나 다른 접근을 하려면  
    → mutable borrow를 먼저 해제해야 안전  

### 🔍 왜 b.borrow()가 필요한가?
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let b = Rc::new(RefCell::new(100));

    // 읽을 때
    let val = b.borrow();
    println!("Read: {}", *val);

    // 쓸 때
    let mut val_mut = b.borrow_mut();
    *val_mut += 1;
    println!("Updated: {}", *val_mut);
}
```

- RefCell<T>는 내부에 T를 가지고 있지만  
    → 직접 접근은 불가능 (b.value 같은 건 안 됨)
- 대신 borrow()나 borrow_mut()를 통해  
    → Ref<T> 또는 RefMut<T>를 얻어야만 접근 가능

## ✅ 핵심 요약
| 작업 방식   | 호출 메서드     | 반환 타입     |
|-------------|------------------|----------------|
| 읽기        | `borrow()`       | `Ref<T>`       |
| 쓰기        | `borrow_mut()`   | `RefMut<T>`    |
    → 모든 접근은 반드시 RefCell을 통해 빌려야 함

---

# Rc::strong_count
Rc::strong_count는 Rc<T>의 현재 강한 참조자 수를 확인하는 함수입니다.  
    → 즉, 이 데이터를 몇 개의 Rc가 공유하고 있는지를 알려주는 도구.

## ✅ 기본 사용법
```rust
use std::rc::Rc;
fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("Strong count: {}", Rc::strong_count(&a)); // 출력: 3
}
```

- Rc::strong_count(&a)는 a를 기준으로 현재 강한 참조자 수를 반환
- 위 예시에서는 a, b, c가 모두 같은 데이터를 참조 → count는 3

## 🔍 언제 쓰면 좋을까?
### 1. Rc::make_mut() 호출 전에 참조자 수 확인
```rust
if Rc::strong_count(&rc) == 1 {
    // 안전하게 수정 가능
}
```

### 2. 디버깅이나 상태 추적
- 구조가 복잡할 때 → 참조자가 몇 개나 살아있는지 확인
- 메모리 누수나 참조 해제 타이밍을 파악할 때 유용

### ⚠️ 주의할 점
- Rc::strong_count()는 정확한 숫자를 반환하지만, 그 순간의 값일 뿐이고 이후에 바뀔 수 있음
- Rc::weak_count()도 함께 쓰면 약한 참조자 수도 확인 가능

---


