# unsafe 회피 접근 기술

unsafe를 피하면서도 동적 메모리 접근을 안전하게 처리하는 방법은
바로 Box, Vec, Arc 같은 스마트 포인터와 컨테이너 타입을 활용하는 것입니다.  
이들은 내부적으로 unsafe를 사용하지만, 외부에서는 안전한 API를 제공합니다.  

## 📦 Box<T> — 힙에 단일 객체 저장
- Box는 단일 값을 힙에 저장하고, 소유권을 명확히 관리합니다.
- unsafe 없이 힙 메모리 접근 가능
```rust
let b = Box::new(42);
println!("{}", *b); // 안전하게 힙 접근
```

- Box는 Drop이 자동 호출되므로 메모리 누수 없음
- Box<T>는 T의 소유권을 갖고, 스코프 종료 시 자동 해제

## 📚 Vec<T> — 힙에 동적 배열
- Vec은 가변 길이 배열을 힙에 저장
- 내부적으로 malloc/realloc 기반이지만 안전한 API 제공
```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
println!("{:?}", v); // 안전하게 힙 접근
```

- Vec은 Index, Iter, get() 등 다양한 안전한 접근 방식 제공
- 범위 초과 시 panic! 발생 → unsafe 없이 오류 감지 가능

## 🧠 Arc<T> — 힙에 공유 객체 (멀티스레드)
- Arc는 참조 카운팅 기반 공유 포인터
- Rc와 달리 스레드 안전 (Atomic Reference Counting)
```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let cloned = Arc::clone(&data);
println!("{:?}", cloned); // 안전하게 공유된 힙 접근
```

- Arc는 Mutex, RwLock과 함께 사용하면 멀티스레드에서도 안전한 공유 가능
- 내부적으로 unsafe를 사용하지만, 외부 API는 완전 안전

## 🔐 unsafe를 회피하는 핵심 전략
| 목적            | 안전한 타입         | 주요 메서드/특징           | 설명                                      |
|-----------------|---------------------|-----------------------------|-------------------------------------------|
| 힙에 단일 값     | Box<T>              | new()                       | 단일 객체를 힙에 저장, 소유권 명확        |
| 힙에 배열        | Vec<T>              | push(), get(), iter()       | 가변 길이 배열, 안전한 인덱싱 및 반복      |
| 힙에 공유 값     | Arc<T>              | clone(), Mutex              | 참조 카운팅 기반 공유, 스레드 안전         |
| 내부 mutability | Cell<T>, RefCell<T> | get(), set(), borrow()      | Copy 타입 또는 런타임 대여 검사 기반 변경  |



## 💡 결론
Rust는 unsafe 없이도 힙 메모리 접근을 안전하게 할 수 있는 구조를 제공합니다.  
Box, Vec, Arc는 모두 내부적으로 unsafe를 사용하지만  
외부에서는 컴파일러가 안전을 보장하는 API만 노출하므로  
**“내가 하던 일을 안전하게 자동화해주는 도구”** 처럼 느껴질 수 있음.


---

# mutable에 새로운 값 복사

rust에서 함수 인자로 구조체를 &mut로 넘기면 그 구조체의 내부 값을 변경할 수는 있지만,
새로운 구조체를 할당해서 바꾸는 건 불가능합니다.
왜냐하면 &mut는 가변 참조이지 소유권을 넘긴 게 아니기 때문.

## 🔧 예시
```rust
struct Data {
    value: i32,
}

fn modify(d: &mut Data) {
    d.value += 1; // ✅ 내부 값 변경 가능
    // d = &mut Data { value: 100 }; ❌ 구조체 자체를 새로 할당 불가
}
```

- d.value += 1 → OK
- d = ... → ❌ 불가능, d는 구조체 자체가 아니라 참조이기 때문

## ✅ 구조체 자체를 바꾸고 싶다면?
- 소유권을 넘겨야 함 (d: Data)
- 또는 Option<&mut T>로 감싸서 교체 가능성 열기
- 또는 Box<T>나 Arc<Mutex<T>>로 힙에 올려서 교체 가능하게 만들기
```rust
fn replace(mut d: Data) -> Data {
    Data { value: d.value + 100 } // ✅ 새 구조체 반환 가능
}
```
## 💡 핵심 요약
| 방식              | 구조체 내부 변경 가능 | 구조체 자체 교체 가능 | 특징 및 설명                          |
|------------------|------------------------|------------------------|---------------------------------------|
| &mut T           | ✅ 가능                 | ❌ 불가능              | 참조 기반, 내부 필드만 수정 가능       |
| T (소유권 이동)  | ✅ 가능                 | ✅ 가능                | 함수에 소유권 넘기면 전체 교체 가능    |
| Box<T>           | ✅ 가능                 | ✅ 가능 (`*box = new`) | 힙에 저장된 구조체를 교체 가능         |
| Arc<Mutex<T>>    | ✅ 가능 (lock 후)       | ✅ 가능 (lock 후 교체) | 스레드 안전 공유, 내부 변경 및 교체 가능 |



## 🔍 왜 헷갈리는가?
- &mut T도 “값을 바꾼다” → 변경처럼 보임
- T를 넘겨서 새로 만든 것도 “값이 바뀐다” → 이것도 변경처럼 보임
- 하지만 Rust는 **“소유권을 넘겼느냐”** 를 기준으로 메모리 해제, 이동, 복사, 교체를 결정함

## 🔑 핵심 차이점
| 방식     | 의미                          | 결과                                |
|----------|-------------------------------|-------------------------------------|
| &mut T   | 가변 참조를 빌려서 수정함     | 구조체의 내부 필드만 변경 가능       |
| T        | 소유권을 넘겨서 새로 만듦     | 구조체 전체를 교체하거나 새로 생성 가능 |


## 🧠 예시로 직관적으로 보기
```rust
struct Data { value: i32 }

fn change(d: &mut Data) {
    d.value += 1; // ✅ 변경: 주소는 그대로, 값만 바뀜
}

fn replace(d: Data) -> Data {
    Data { value: d.value + 100 } // ✅ 소유권 이전: 새 구조체 생성
}
```

- change()는 값을 바꾸는 함수
- replace()는 값을 새로 만드는 함수

## 🔧 안전한 교체 패턴
### 1. Option<T>로 감싸서 교체 가능하게 만들기
```rust
struct App {
    config: Option<Config>,
}

fn reload(app: &mut App) {
    app.config = Some(Config::new()); // ✅ 새로 생성해서 교체
}
```

### 2. Box<T>로 힙에 올려서 교체
```rust
struct App {
    config: Box<Config>,
}

fn reload(app: &mut App) {
    app.config = Box::new(Config::new()); // ✅ 힙 객체 교체
}
```

### 3. Arc<Mutex<T>>로 공유 + 교체
```rust
use std::sync::{Arc, Mutex};

struct App {
    config: Arc<Mutex<Config>>,
}

fn reload(app: &App) {
    let mut cfg = app.config.lock().unwrap();
    *cfg = Config::new(); // ✅ 내부 값 교체
}
```



말씀하신 대로 Option<T>, Box<T>, Arc<T>는 소유권을 갖는 타입이기 때문에  
값을 바꾸는 게 아니라, 소유권을 넘겨서 새 값을 “가져가는” 방식으로  
메모리 교체가 안전하게 이루어집니다.  

## 🔧 포인터처럼 보이지만 포인터보다 안전한 구조
| 타입        | 소유권 있음 | Drop 자동 | 값 교체 가능 | Null 표현 가능 | 스레드 안전 |
|-------------|--------------|------------|----------------|----------------|--------------|
| Option<T>   | ✅            | ✅         | ✅ (`Some(new)`) | ✅ (`None`)     | ✅ (T에 따라 다름) |
| Box<T>      | ✅            | ✅         | ✅ (`*box = new`) | ❌              | ❌ (단일 스레드)   |
| Arc<T>      | ✅            | ✅         | ✅ (`lock + replace`) | ❌              | ✅ (Atomic)       |


- 이들은 모두 소유권을 갖고 있으므로 =로 새 값을 넣으면 기존 값은 drop됨
- Rust는 이 과정을 컴파일 타임에 안전하게 보장함

## 🧠 포인터처럼 보이지만 포인터보다 강력한 이유
- C++에서는 T*에 new T를 넣으면 기존 메모리 누수 위험
- Rust에서는 Box<T>에 Box::new(...)를 넣으면
    → 기존 값은 자동으로 Drop 호출됨  
    → 메모리 누수 없음, 안전한 교체  

```rust    
let mut config = Box::new(Config::default());
config = Box::new(Config::new()); // ✅ 기존 값 drop, 새 값 소유
```


## 💡 Default를 만들어 놓으면 더 쉬워짐
```rust
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
}
```

let mut cfg = Box::new(Config::default()); // ✅ 기본값으로 초기화
cfg = Box::new(Config { host: "localhost".into(), port: 8080 }); // ✅ 새 값으로 교체


- Default는 초기화 패턴을 단순화시켜줌
- Option::take()와 함께 쓰면 값을 꺼내고 교체하는 패턴도 가능

---
