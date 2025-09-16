# 🦀 Rust: Thread-Safe Singleton

Rust에서는 전통적인 객체지향 방식의 Singleton보다는 lazy static 초기화와 **OnceCell / Lazy**를 활용한 안전한 접근 방식이 일반적입니다.

## 클래스 다이아그램
```mermaid
classDiagram
    class Config {
        +host: String
        +port: u16
    }

    class CONFIG {
        <<static>>
        OnceCell<Mutex<Config>>
    }

    class get_config {
        +get_config() &'static Mutex<Config>
    }

    class main {
        +main()
    }

    CONFIG --> Config : holds
    get_config --> CONFIG : accesses
    main --> get_config : calls
    main --> Config : reads
```

## ✅ OnceCell 기반 Singleton
```rust
use once_cell::sync::OnceCell;
use std::sync::Mutex;

#[derive(Debug)]
struct Config {
    pub host: String,
    pub port: u16,
}

static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

fn get_config() -> &'static Mutex<Config> {
    CONFIG.get_or_init(|| {
        Mutex::new(Config {
            host: "localhost".to_string(),
            port: 8080,
        })
    })
}

fn main() {
    let config = get_config().lock().unwrap();
    println!("Host: {}, Port: {}", config.host, config.port);
}
```

### 🔒 특징
- OnceCell은 한 번만 초기화되며 이후에는 불변 참조로 접근
- Mutex를 통해 동시성 안전성 확보
- get_or_init()은 lazy 초기화 방식

## 🧪 Rust 대안: Lazy (once_cell 1.17+)
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    Mutex::new(Config {
        host: "localhost".to_string(),
        port: 8080,
    })
});

fn main() {
    // Thread-safe하게 접근
    let config = CONFIG.lock().unwrap();
    println!("Host: {}, Port: {}", config.host, config.port);
}

```

```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

// Lazy를 이용한 전역 Singleton 선언
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    Mutex::new(Config {
        host: "localhost".to_string(),
        port: 8080,
    })
});

// 접근 함수 정의
fn get_config() -> &'static Mutex<Config> {
    &CONFIG
}

fn main() {
    let config = get_config().lock().unwrap();
    println!("Host: {}, Port: {}", config.host, config.port);
}

```

- Lazy는 OnceCell보다 더 간결하게 표현 가능
- 내부적으로 OnceCell을 사용


## 외부 크레이트로 설치해야 합니다.
### ✅ 설치 방법
```
cargo add once_cell
```

### 혹은 Cargo.toml에 직접 추가해도 됩니다:
```
[dependencies]
once_cell = "1.17"
```

## 📦 주요 기능
- OnceCell<T>: 한 번만 초기화되는 전역 또는 지역 변수
- Lazy<T>: OnceCell을 감싼 더 간편한 lazy 초기화 도구
- sync 버전은 스레드 안전(Thread-Safe)
- unsync 버전은 싱글 스레드 환경에서 더 빠름


## ✅ 접근 방식 비교
| 항목               | `OnceCell<Mutex<T>>`                  | `Lazy<Mutex<T>>`                     |
|--------------------|----------------------------------------|--------------------------------------|
| 초기화 방식         | `get_or_init(|| ...)`                 | `Lazy::new(|| ...)`                  |
| 선언 방식           | `static VAR: OnceCell<Mutex<T>>`      | `static VAR: Lazy<Mutex<T>>`         |
| 접근 방식           | `VAR.get().unwrap().lock().unwrap()`  | `VAR.lock().unwrap()`                |
| 내부 구현 기반      | `OnceCell`                            | `OnceCell`                           |



## 🔍 예시 비교
### 🔸 OnceCell 방식
```rust
use once_cell::sync::OnceCell;
use std::sync::Mutex;

static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

fn get_config() -> &'static Mutex<Config> {
    CONFIG.get_or_init(|| Mutex::new(Config {
        host: "localhost".to_string(),
        port: 8080,
    }))
}
```

### 🔸 Lazy 방식
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config {
    host: "localhost".to_string(),
    port: 8080,
}));

```

- Lazy는 내부적으로 OnceCell을 사용하지만, 초기화와 접근을 더 간단하게 만들어줍니다.
- CONFIG.lock().unwrap()만으로 바로 접근 가능하니, 실전에서는 Lazy가 더 선호되는 경우가 많습니다.

## 🧠 결론
- 둘 다 Thread-Safe Singleton을 구현할 수 있음
- Lazy는 더 간결하고 직관적이지만, OnceCell은 초기화 시점 제어가 더 명확함
- 실무에서는 Lazy<Mutex<T>>를 사용하는 것이 일반적이며, 특히 설정값이나 전역 상태 관리에 적합합니다

---

## 🦀 CONFIG 값을 변경하는 Rust 예제
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

// 전역 Singleton
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    Mutex::new(Config {
        host: "localhost".to_string(),
        port: 8080,
    })
});

// 접근 함수
fn get_config() -> &'static Mutex<Config> {
    &CONFIG
}

// 설정 변경 함수
fn update_config(new_host: &str, new_port: u16) {
    let mut config = CONFIG.lock().unwrap();
    config.host = new_host.to_string();
    config.port = new_port;
}

fn main() {
    {
        let config = get_config().lock().unwrap();
        println!("Before: Host={}, Port={}", config.host, config.port);
    }

    // CONFIG 값 변경
    update_config("127.0.0.1", 3000);

    {
        let config = get_config().lock().unwrap();
        println!("After: Host={}, Port={}", config.host, config.port);
    }
}
```


### ✅ 설명
- CONFIG.lock().unwrap()으로 MutexGuard를 얻고 값을 수정
- update_config() 함수는 외부에서 안전하게 설정을 변경할 수 있도록 캡슐화
- Mutex 덕분에 다중 스레드 환경에서도 안전하게 동작

---


# 🧠 다른 언어에서의 Singleton 구현

## 🧩 C#
```csharp
public sealed class Singleton {
    private static readonly Lazy<Singleton> instance = new(() => new Singleton());
    public static Singleton Instance => instance.Value;
    private Singleton() { }
}

```

- Lazy<T>를 통해 Thread-Safe Lazy Initialization
- sealed로 상속 방지

## 🧩 C++
```cpp
class Singleton {
public:
    static Singleton& getInstance() {
        static Singleton instance;
        return instance;
    }
private:
    Singleton() {}
    Singleton(const Singleton&) = delete;
    void operator=(const Singleton&) = delete;
};
```

- static 지역 변수는 C++11 이후 Thread-Safe
- 복사 생성자와 대입 연산자 삭제로 복사 방지

## 🧩 Python
```python
class Singleton:
    _instance = None

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance
```

- __new__를 오버라이드하여 인스턴스 생성 제어
- GIL 덕분에 기본적으로 Thread-Safe지만, 고성능 환경에서는 threading.Lock 사용 권장

## ✅ 요약 비교
| 언어     | Thread-Safe 방식             | Lazy 초기화 | 특징                          | 비고                         |
|----------|------------------------------|-------------|-------------------------------|------------------------------|
| Rust     | `OnceCell` / `Lazy` + `Mutex` | ✅          | 안전한 초기화, 명시적 동기화     | `once_cell` 크레이트 사용     |
| C#       | `Lazy<T>`                    | ✅          | 간결하고 안전한 접근 방식         | `sealed` 클래스로 상속 방지   |
| C++      | `static` 지역 변수           | ✅          | C++11 이후 Thread-Safe 보장     | 복사 생성자/대입 연산자 삭제 |
| Python   | `__new__` 오버라이드         | ⚠️ 기본 안전 | GIL 덕분에 기본적으로 안전       | 고성능 환경에선 Lock 권장     |


Rust에서는 전역 상태를 최소화하고 명시적으로 관리하는 것이 철학에 부합합니다.

---


