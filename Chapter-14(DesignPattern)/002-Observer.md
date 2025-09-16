# 🦀 Rust: Trait + Lazy + Mutex 기반 Observer

```rust
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

trait Observer: Send + Sync {
    fn update(&self, message: &str);
}

struct Subject {
    observers: Vec<Arc<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: Vec::new() }
    }

    fn register(&mut self, obs: Arc<dyn Observer>) {
        self.observers.push(obs);
    }

    fn notify_all(&self, msg: &str) {
        for obs in &self.observers {
            obs.update(msg);
        }
    }
}

static SUBJECT: Lazy<Mutex<Subject>> = Lazy::new(|| Mutex::new(Subject::new()));

fn get_subject() -> &'static Mutex<Subject> {
    &SUBJECT
}

struct Logger;
impl Observer for Logger {
    fn update(&self, msg: &str) {
        println!("[Logger] {}", msg);
    }
}

struct Notifier;
impl Observer for Notifier {
    fn update(&self, msg: &str) {
        println!("[Notifier] {}", msg);
    }
}

fn main() {
    let logger = Arc::new(Logger);
    let notifier = Arc::new(Notifier);

    {
        let mut subject = get_subject().lock().unwrap();
        subject.register(logger);
        subject.register(notifier);
    }

    {
        let subject = get_subject().lock().unwrap();
        subject.notify_all("Rust Observer: Update triggered!");
    }
}

```

## 🧩 C++: Interface + Vector 기반 Observer
```cpp
#include <iostream>
#include <vector>
#include <memory>

class Observer {
public:
    virtual void update(const std::string& msg) = 0;
};

class Subject {
    std::vector<std::shared_ptr<Observer>> observers;
public:
    void registerObserver(std::shared_ptr<Observer> obs) {
        observers.push_back(obs);
    }

    void notifyAll(const std::string& msg) {
        for (auto& obs : observers) {
            obs->update(msg);
        }
    }
};

class Logger : public Observer {
public:
    void update(const std::string& msg) override {
        std::cout << "[Logger] " << msg << std::endl;
    }
};

/ Concrete Observer 2
class Alert : public Observer {
public:
    void update(const std::string& msg) override {
        if (msg.find("error") != std::string::npos) {
            std::cout << "[Alert] ⚠️ Critical issue detected: " << msg << std::endl;
        }
    }
};

// Concrete Observer 3
class AuditTrail : public Observer {
public:
    void update(const std::string& msg) override {
        std::cout << "[AuditTrail] Logged event: " << msg << std::endl;
    }
};

// main 함수
int main() {
    Subject system;

    auto logger = std::make_shared<Logger>();
    auto alert = std::make_shared<Alert>();
    auto audit = std::make_shared<AuditTrail>();

    system.registerObserver(logger);
    system.registerObserver(alert);
    system.registerObserver(audit);

    system.notifyAll("System started");
    system.notifyAll("User login successful");
    system.notifyAll("Disk error detected");

    return 0;
}
```

## 🧩 C#: Interface + List + Event 기반 Observer
```csharp
using System;
using System.Collections.Generic;

interface IObserver {
    void Update(string message);
}

class Subject {
    private List<IObserver> observers = new();

    public void Register(IObserver obs) => observers.Add(obs);
    public void NotifyAll(string msg) {
        foreach (var obs in observers) {
            obs.Update(msg);
        }
    }
}

class Logger : IObserver {
    public void Update(string msg) => Console.WriteLine($"[Logger] {msg}");
}

class Program {
    static void Main() {
        Subject system = new();

        IObserver logger = new Logger();
        system.Register(logger);
        system.NotifyAll("System initialized");
        system.NotifyAll("User login successful");
        system.NotifyAll("Disk error detected");
    }
}
```


## 🐍 Python: Duck Typing + List 기반 Observer

```python
class Subject:
    def __init__(self):
        self.observers = []

    def register(self, obs):
        self.observers.append(obs)

    def notify_all(self, msg):
        for obs in self.observers:
            obs.update(msg)

class Logger:
    def update(self, msg):
        print(f"[Logger] {msg}")

# 메인 함수
def main():
    system = Subject()

    logger = Logger()
    system.register(logger)

    system.notify_all("System initialized")
    system.notify_all("User login successful")
    system.notify_all("Disk error detected")

if __name__ == "__main__":
    main()

```

## ✅ 비교 요약
| 언어   | 구조 방식             | Thread-Safe       | 특징                                      | 비고                          |
|--------|----------------------|-------------------|-------------------------------------------|-------------------------------|
| Rust   | Trait + Arc + Mutex  | ✅ 명시적 동기화   | 안전한 공유, 고성능, 명확한 책임 분리       | `once_cell` + `Lazy` 사용     |
| C++    | Interface + Vector   | ❌ 기본 아님       | 수동 동기화 필요, 성능 유리                 | C++11 이후 `static` 초기화 안전 |
| C#     | Interface + List     | ✅ 자동 동기화     | 이벤트 기반 확장 가능, 직관적 구조          | `delegate` / `event` 활용 가능 |
| Python | Duck Typing + List   | ⚠️ GIL 기반        | 간단하지만 고성능 환경에선 Lock 필요        | `threading.Lock` 권장         |


---

## 🧠 선언 구조 해부: Vec<Arc<dyn Observer>>
이걸 안쪽부터 바깥쪽으로 해석하면 쉬워요:
- Observer: 우리가 정의한 트레잇 (인터페이스)
- dyn Observer: 트레잇 객체 (동적 디스패치)
- Arc<dyn Observer>: 여러 곳에서 안전하게 공유 가능한 스마트 포인터
- Vec<Arc<dyn Observer>>: 옵저버들을 담는 벡터 (리스트)
### 📌 즉, “Observer 트레잇을 구현한 객체들을 Arc로 감싸서 Vec에 담는다”는 뜻이에요.

## 🧩 익숙해지는 방법
### 1. 안쪽부터 바깥쪽으로 해석하는 습관
Rust의 타입은 중첩 구조가 많기 때문에, 안쪽부터 해석하는 연습이 중요해요.
예시:
```rust
Option<Result<String, Error>>
```

→ Error가 발생할 수 있는 String 결과를 Option으로 감싼 것

### 2. 자주 쓰는 패턴을 손으로 직접 써보기
예를 들어 아래 패턴들을 직접 선언해보면 감이 잡혀요:
```rust
Vec<String>
Vec<Box<dyn Trait>>
Vec<Arc<dyn Trait>>
HashMap<String, Arc<dyn Trait>>
```



### 3. 타입 별칭(type alias)을 적극 활용
복잡한 타입은 별칭으로 간단하게 만들 수 있어요:
```rust
type ObserverList = Vec<Arc<dyn Observer>>;

struct Subject {
    observers: ObserverList,
}
```

이렇게 하면 가독성이 훨씬 좋아지고, 선언 순서에 대한 부담도 줄어들어요.

### 4. IDE 도움 받기
Rust 지원이 좋은 IDE (예: VSCode + rust-analyzer)를 쓰면
마우스를 올려서 타입을 확인하거나 자동 완성으로 구조를 파악하기 쉬워요.

### 5. 비유로 기억하기
- Arc: “공유 가능한 스마트 포인터” → 여러 사람이 보는 책
- dyn Trait: “동적 디스패치” → 인터페이스를 통해 행동을 결정
- Vec: “리스트” → 여러 개를 담는 상자
📦 Vec<Arc<dyn Observer>> = 여러 사람이 공유하는 인터페이스 객체 리스트

---


