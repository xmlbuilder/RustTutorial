# Observer 실무에 적용

## 📘 구조 설명: Observer 패턴 in Rust
이 코드는 전통적인 Observer 패턴을 Rust의 Rc와 Weak 포인터를 활용해 구현한 구조입니다.

## 소스
```rust
use crate::core::subject;

pub trait Observer {
    fn update_observer(
        &self,
        subject: *const subject::Subject,
        msg: u32,
        w_param: u64,
        l_param: u64,
    ) -> i32;
}
```

```rust
use crate::core::observer::Observer;
use std::rc::{Rc, Weak};

pub struct Subject {
    observers: Vec<Weak<dyn Observer>>,
}
```
```rust
impl Subject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn attach_observer(&mut self, obs: &Rc<dyn Observer>) {
        let ptr = Rc::as_ptr(obs) as *const ();
        let exists = self.observers.iter().any(|w| {
            if let Some(s) = w.upgrade() {
                (Rc::as_ptr(&s) as *const ()) == ptr
            } else {
                false
            }
        });
        if !exists {
            self.observers.push(Rc::downgrade(obs));
        }
    }

    pub fn detach_observer(&mut self, obs: &Rc<dyn Observer>) {
        let ptr = Rc::as_ptr(obs) as *const ();
        self.observers.retain(|w| {
            if let Some(s) = w.upgrade() {
                (Rc::as_ptr(&s) as *const ()) != ptr
            } else {
                false
            }
        });
    }

    pub fn clear_observers(&mut self) {
        self.observers.clear();
    }

    pub fn notify(&mut self, msg: u32, wparam: u64, lparam: u64) -> i32 {
        self.observers.retain(|w| w.upgrade().is_some());
        for w in &self.observers {
            if let Some(rc) = w.upgrade() {
                let r = rc.update_observer(self as *const Subject, msg, wparam, lparam);
                if r != 0 {
                    return r;
                }
            }
        }
        0
    }
}
```
```rust
impl Default for Subject {
    fn default() -> Self {
        Self::new()
    }
}


```


## 🔧 주요 구성 요소

| 구성 요소             | 역할 설명                                                             |
|----------------------|------------------------------------------------------------------------|
| `Subject`            | 관찰 대상. 여러 `Observer`를 등록하고 이벤트 발생 시 알림을 전달함       |
| `Observer`           | 관찰자 인터페이스. `update_observer()`를 통해 알림을 수신함              |
| `update_observer()`  | 옵저버가 알림을 받을 때 호출되는 메서드. 메시지와 파라미터를 전달받음     |
| `Rc<dyn Observer>`   | 옵저버를 참조 카운팅 방식으로 소유. `Subject`에 등록할 때 사용됨          |
| `Weak<dyn Observer>` | `Subject`가 옵저버를 소유하지 않도록 하는 약한 참조. 메모리 누수 방지용    |
| `SubjectObserverWeak`| `Weak<dyn Observer>`의 의미를 명확히 하기 위한 설명적 이름 또는 별칭       |

## 🔁 동작 흐름
- attach_observer()로 옵저버 등록
- notify() 호출 시 모든 살아있는 옵저버에게 메시지 전달
- detach_observer()로 옵저버 제거 가능
- clear_observers()로 전체 제거 가능



Subject 구조체의 `attach_observer`, `detach_observer`, `notify` 메서드는   
Rust의 동적 트레잇 객체, Rc/Weak 포인터, 그리고 포인터 비교를 활용한 고급 패턴에 대한 설명.
아래에 각 메서드를 단계적으로 해부해서 설명.

## 🧩 1. attach_observer
```rust
pub fn attach_observer(&mut self, obs: &Rc<dyn Observer>) {
    let ptr = Rc::as_ptr(obs) as *const ();
    let exists = self.observers.iter().any(|w| {
        if let Some(s) = w.upgrade() {
            (Rc::as_ptr(&s) as *const ()) == ptr
        } else {
            false
        }
    });
    if !exists {
        self.observers.push(Rc::downgrade(obs));
    }
}
```
### 🔍 단계별 설명
- `Rc::as_ptr(obs)` → Rc 내부의 `실제 포인터` 를 가져옴
- Rc<dyn Trait>는 트레잇 객체이므로 `Rc::as_ptr()` 은 `*const dyn Trait` 반환
- `as *const ()` → 트레잇 객체의 포인터를 `raw 포인터` 로 변환
- 트레잇 객체는 `fat pointer` 이므로 비교를 위해 `*const ()` 로 단순화
- self.observers.iter().any(...)
- 기존에 동일한 옵저버가 등록되어 있는지 확인
- `w.upgrade() → Weak` 를 `Rc로 승격`
- 살아있는 옵저버만 비교
- `Rc::as_ptr(&s)` → 기존 옵저버의 포인터와 비교
- if !exists → 중복이 없으면 `Rc::downgrade(obs)` 로 `Weak` 로 변환해 등록

## 🧩 2. detach_observer
```rust
pub fn detach_observer(&mut self, obs: &Rc<dyn Observer>) {
    let ptr = Rc::as_ptr(obs) as *const ();
    self.observers.retain(|w| {
        if let Some(s) = w.upgrade() {
            (Rc::as_ptr(&s) as *const ()) != ptr
        } else {
            false
        }
    });
}
```

### 🔍 단계별 설명
- `Rc::as_ptr(obs)` → 제거 대상 옵저버의 `포인터 추출`
- `retain(...)` → 조건에 맞는 옵저버만 남김
- `upgrade()` → `살아있는 옵저버만 비교`
- `!= ptr` → 제거 대상과 포인터가 다르면 유지
- else { false } → 죽은 옵저버는 제거

## 🧩 3. notify
```rust
pub fn notify(&mut self, msg: u32, wparam: u64, lparam: u64) -> i32 {
    self.observers.retain(|w| w.upgrade().is_some());
    for w in &self.observers {
        if let Some(rc) = w.upgrade() {
            let r = rc.update_observer(self as *const Subject, msg, wparam, lparam);
            if r != 0 {
                return r;
            }
        }
    }
    0
}
```

### 🔍 단계별 설명
- `retain(|w| w.upgrade().is_some())`
    - 죽은 옵저버 제거
- `for w in &self.observers` → 살아있는 옵저버 순회
- `w.upgrade()` → `Weak` → `Rc`
- `rc.update_observer(...)` 호출
- 옵저버에게 메시지 전달
- if r != 0 → 옵저버가 특별한 응답을 하면 즉시 반환
- 0 → 모든 옵저버가 정상 응답하면 기본값 반환

## ✅ 핵심 요약

| 메서드   | 핵심 동작 요약                          |
|----------|------------------------------------------|
| attach   | `Rc`를 `Weak`로 다운그레이드하여 등록     |
| detach   | 포인터 비교로 해당 옵저버 제거            |
| notify   | 살아있는 옵저버에게 메시지 전달 및 응답 처리 |

----

## 테스트 코드

```rust

use crate::core::observer::{Observer, Subject};
use std::rc::Rc;
use std::cell::RefCell;

struct LoggerObserver {
    id: u32,
    log: Rc<RefCell<Vec<String>>>,
}
```
```rust
impl LoggerObserver {
    fn new(id: u32, log: Rc<RefCell<Vec<String>>>) -> Self {
        Self { id, log }
    }
}
```
```rust
impl Observer for LoggerObserver {
    fn update_observer(
        &self,
        _subject: *const Subject,
        msg: u32,
        w_param: u64,
        l_param: u64,
    ) -> i32 {
        let entry = format!("Observer {} received msg={}, w={}, l={}", self.id, msg, w_param, l_param);
        self.log.borrow_mut().push(entry);
        0
    }
}
```
```rust
#[test]
fn test_subject_observer_notify() {
    let mut subject = Subject::new();
    let log = Rc::new(RefCell::new(Vec::new()));

    let obs1 = Rc::new(LoggerObserver::new(1, Rc::clone(&log)));
    let obs2 = Rc::new(LoggerObserver::new(2, Rc::clone(&log)));

    subject.attach_observer(&obs1);
    subject.attach_observer(&obs2);

    let result = subject.notify(42, 100, 200);
    assert_eq!(result, 0);

    let entries = log.borrow();
    assert_eq!(entries.len(), 2);
    assert!(entries[0].contains("Observer 1 received msg=42"));
    assert!(entries[1].contains("Observer 2 received msg=42"));
}
```
```rust
#[test]
fn test_no_duplicate_observers() {
    let mut subject = Subject::new();
    let log = Rc::new(RefCell::new(Vec::new()));
    let obs = Rc::new(LoggerObserver::new(1, Rc::clone(&log)));

    subject.attach_observer(&obs);
    subject.attach_observer(&obs); // 중복 등록 시도

    let result = subject.notify(1, 0, 0);
    assert_eq!(result, 0);
    assert_eq!(log.borrow().len(), 1); // 한 번만 호출되어야 함
}
```

## ✅ 결론
이 구조는 Rust에서 안전하게 Observer 패턴을 구현하는 좋은 예입니다.
- Rc + Weak으로 메모리 순환 방지
- upgrade()로 살아있는 옵저버만 호출
- notify()는 옵저버가 0 이외의 값을 반환하면 즉시 중단

---

# Subect / Observer 위치

Subject가 `Weak<dyn Observer>` 를 보유하는 구조에서는 Observer가 살아있지 않으면 알림을 받을 수 없기 때문에, 
두 객체가 같은 생명주기(Container) 안에 있어야 안정적으로 동작합니다.

## 🧠 왜 Container가 필요한가?
### 🔗 Weak 참조의 특징
- Weak<T>는 Rc<T>의 소유권을 갖지 않음
- Rc::downgrade()로 생성되며, upgrade()로 살아있는 Rc를 얻을 수 있음
- Rc가 drop되면 Weak는 None을 반환
### ⚠️ 문제 상황
- Subject는 Weak<dyn Observer>만 보유
- Observer가 외부에서 drop되면 notify() 시 upgrade() 실패
- → 알림 누락, 옵저버 제거됨

### ✅ 해결 전략: Container로 생명주기 묶기
```rust
struct Container {
    subject: Subject,
    observers: Vec<Rc<dyn Observer>>,
}
```
### ✔️ 장점
- Subject는 Weak로 참조
- Container는 Rc로 Observer를 소유 → 생명 유지
- Subject.notify()는 항상 살아있는 옵저버를 대상으로 동작

### 🧩 예시 구조
```rust
struct Container {
    subject: Subject,
    observers: Vec<Rc<dyn Observer>>,
}
```
```rust
impl Container {
    fn new() -> Self {
        
        let mut subject = Subject::new();
        let log = Rc::new(RefCell::new(Vec::new()));

        let obs1: Rc<dyn Observer> = Rc::new(LoggerObserver::new(1, Rc::clone(&log)));
        let obs2: Rc<dyn Observer> = Rc::new(LoggerObserver::new(2, Rc::clone(&log)));

        subject.attach_observer(&obs1);
        subject.attach_observer(&obs2);

        Self {
            subject,
            observers: vec![obs1, obs2],
        }
    }

    fn notify_all(&mut self, msg: u32, w: u64, l: u64) {
        self.subject.notify(msg, w, l);
    }
}
```

## 🧠 확장 아이디어
- Container를 Rc<RefCell<Container>>로 감싸면 동적 등록/제거도 가능
- Observer가 Container를 참조해야 할 경우 → Weak<Container>로 역참조 가능

# Event Bus

Subject를 이벤트 버스로 바꾸는 핵심은 Observer 패턴 → Event Bus 패턴으로 구조를 전환하는 것입니다.  
아래에 그 차이점과 전환 방법을 단계적으로 설명.

## 🔁 Observer vs Event Bus

| 항목             | SubjectObserver           | EventBus                   |
|------------------|---------------------------|----------------------------|
| 구조             | `Subject` ↔ `Observer`     | `EventBus` ↔ `Listener`     |
| 호출 방식        | `Subject.notify()`         | `EventBus.emit(event)`      |
| 메시지 형식      | `msg`, `wparam`, `lparam`  | `Event` enum + payload      |
| 연결 방식        | 직접 참조 (`Weak`)         | 등록 기반 브로드캐스트      |
| 확장성           | 제한적 (트레잇 기반)        | 유연함 (이벤트 타입 다양화) |
| decoupling 수준 | 낮음 (타이트 커플링)       | 높음 (완전 분리 가능)       |

## 🧠 전환 전략: Subject → EventBus
### 1. 이벤트 타입 정의
```rust
#[derive(Debug, Clone)]
pub enum Event {
    DataUpdated(u64),
    ErrorOccurred(String),
    Custom(u32, u64, u64),
}
```

### 2. 리스너 트레잇 정의
```rust
pub trait EventListener {
    fn on_event(&self, event: &Event);
}
```

### 3. EventBus 구조체
```rust
pub struct EventBus {
    listeners: Vec<Weak<dyn EventListener>>,
}
``

### 4. 등록 / 제거 / 브로드캐스트 메서드
```rust
impl EventBus {
    pub fn register(&mut self, listener: &Rc<dyn EventListener>) {
        let ptr = Rc::as_ptr(listener) as *const ();
        let exists = self.listeners.iter().any(|w| {
            w.upgrade().map_or(false, |s| Rc::as_ptr(&s) as *const () == ptr)
        });
        if !exists {
            self.listeners.push(Rc::downgrade(listener));
        }
    }

    pub fn unregister(&mut self, listener: &Rc<dyn EventListener>) {
        let ptr = Rc::as_ptr(listener) as *const ();
        self.listeners.retain(|w| {
            w.upgrade().map_or(false, |s| Rc::as_ptr(&s) as *const () != ptr)
        });
    }

    pub fn emit(&mut self, event: Event) {
        self.listeners.retain(|w| w.upgrade().is_some());
        for w in &self.listeners {
            if let Some(rc) = w.upgrade() {
                rc.on_event(&event);
            }
        }
    }
}
```

## ✅ 장점
- 이벤트 타입을 명확하게 정의할 수 있음
- 발신자와 수신자가 완전히 분리됨
- 다양한 리스너가 다양한 이벤트에 반응 가능
- 테스트와 로깅, 확장성이 뛰어남


## 샘플 소스
아래는 Rust로 구현한 EventBus 패턴의 전체 샘플 코드입니다.
이 코드는 EventBus를 통해 여러 리스너에게 이벤트를 브로드캐스트하는 구조를 보여줍니다.

### 🧩 전체 구조
```rust
use std::rc::{Rc, Weak};

// 이벤트 타입 정의
#[derive(Debug, Clone)]
pub enum Event {
    DataUpdated(u64),
    ErrorOccurred(String),
    Custom(u32, u64, u64),
}
```
```rust
// 리스너 트레잇
pub trait EventListener {
    fn on_event(&self, event: &Event);
}
```

```rust
// 이벤트 버스
pub struct EventBus {
    listeners: Vec<Weak<dyn EventListener>>,
}
```

```rust
impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn register(&mut self, listener: &Rc<dyn EventListener>) {
        let ptr = Rc::as_ptr(listener) as *const ();
        let exists = self.listeners.iter().any(|w| {
            w.upgrade().map_or(false, |s| Rc::as_ptr(&s) as *const () == ptr)
        });
        if !exists {
            self.listeners.push(Rc::downgrade(listener));
        }
    }

    pub fn unregister(&mut self, listener: &Rc<dyn EventListener>) {
        let ptr = Rc::as_ptr(listener) as *const ();
        self.listeners.retain(|w| {
            w.upgrade().map_or(false, |s| Rc::as_ptr(&s) as *const () != ptr)
        });
    }

    pub fn emit(&mut self, event: Event) {
        self.listeners.retain(|w| w.upgrade().is_some());
        for w in &self.listeners {
            if let Some(rc) = w.upgrade() {
                rc.on_event(&event);
            }
        }
    }
}
```


### 🧪 샘플 리스너 구현
```rust
use std::cell::RefCell;

struct Logger {
    id: u32,
    log: Rc<RefCell<Vec<String>>>,
}
```
```rust
impl Logger {
    fn new(id: u32, log: Rc<RefCell<Vec<String>>>) -> Self {
        Self { id, log }
    }
}
```
```rust
impl EventListener for Logger {
    fn on_event(&self, event: &Event) {
        let msg = format!("Logger {} received: {:?}", self.id, event);
        self.log.borrow_mut().push(msg);
    }
}
```


### 🧪 테스트 예제
```rust
fn main() {
    let mut bus = EventBus::new();
    let log = Rc::new(RefCell::new(Vec::new()));

    let logger1: Rc<dyn EventListener> = Rc::new(Logger::new(1, Rc::clone(&log)));
    let logger2: Rc<dyn EventListener> = Rc::new(Logger::new(2, Rc::clone(&log)));

    bus.register(&logger1);
    bus.register(&logger2);

    bus.emit(Event::DataUpdated(42));
    bus.emit(Event::ErrorOccurred("Something went wrong".into()));

    for entry in log.borrow().iter() {
        println!("{}", entry);
    }
}
```

### ✅ 출력 예시
```
Logger 1 received: DataUpdated(42)
Logger 2 received: DataUpdated(42)
Logger 1 received: ErrorOccurred("Something went wrong")
Logger 2 received: ErrorOccurred("Something went wrong")
```
---




