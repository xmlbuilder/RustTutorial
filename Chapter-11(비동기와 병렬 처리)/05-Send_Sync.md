# Send Sync

- Send: T가 스레드 간 이동이 안전하다면, T의 타입은 Send입니다.
- Sync: &T가 스레드 간 이동이 안전하다면, &T의 타입은 Sync입니다.

## 🧠 핵심 개념: Send와 Sync Trait

Rust에서 스레드 간 데이터 공유를 안전하게 하기 위해
두 가지 특성(trait)을 사용합니다:
| Trait | 의미 | 언제 필요함 | 기본 구현 타입 | 구현되지 않는 대표 타입 |
|-------|------|--------------|----------------|--------------------------|
| Send  | 값을 다른 스레드로 이동할 수 있음 | thread::spawn, Mutex<T> 등 | i32, f64, Vec<T>, Box<T> | Rc<T>, RefCell<T>, Box<dyn> |
| Sync  | 여러 스레드에서 동시에 참조(&T) 가능 | Arc<T>, &T 공유 시 | i32, f64, Vec<T>, Arc<T> | RefCell<T>, Rc<T>, Box<dyn> |

이 두 trait은 Rust가 **멀티스레드 환경에서 데이터 경쟁(race condition)** 을 막기 위해
컴파일 타임에 검사하는 안전 장치입니다.

## 🔍 Send 자세히 보기
- Send는 타입이 다른 스레드로 안전하게 이동될 수 있는지를 나타냅니다.
- 대부분의 기본 타입(i32, f64, Vec<T>, Box<T>)은 Send를 자동 구현합니다.
- 소유권을 다른 스레드로 이동하면 소멸자가 해당 스레드에서 실행됩니다.
- 여기서 의문은 “언제 한 스레드에서 값을 할당하고 다른 스레드에서 값을 할당 해제할 수 있는가” 입니다.
- 하지만 Rc<T>, RefCell<T>, Box<dyn Trait> 같은 타입은 기본적으로 Send가 아닙니다.
### 예시
```rust
fn spawn_thread(v: Vec<i32>) {
    std::thread::spawn(move || {
        println!("{:?}", v);
    });
}
```
- Vec<i32>는 Send를 구현하므로 스레드로 이동 가능
- 만약 Rc<T>를 넘기면 컴파일 에러 발생 → Rc는 Send가 아님

## 🔍 Sync 자세히 보기
- Sync는 타입이 **여러 스레드에서 동시에 참조(&T)** 될 수 있는지를 나타냅니다.
- &T가 Send가 되려면 T가 Sync여야 합니다.
- &T가 Send인 경우에만 T의 타입이 Sync가 됩니다
- 예를 들어 Arc<T>는 내부적으로 Sync를 구현해서 여러 스레드에서 참조 가능하게 해줍니다.
예시
```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);

let d1 = data.clone();
let d2 = data.clone();

std::thread::spawn(move || println!("{:?}", d1));
std::thread::spawn(move || println!("{:?}", d2));
```
- Arc<T>는 Sync를 구현하므로 여러 스레드에서 안전하게 참조 가능

### ⚠️ 왜 Box<dyn Fn()>는 기본적으로 Send가 아닌가?
- dyn Trait은 런타임에 어떤 타입이 들어올지 모르기 때문에
Rust는 기본적으로 Send와 Sync를 자동 구현하지 않습니다.
- 그래서 Box<dyn Fn(...)>를 Mutex로 감싸거나 스레드로 넘기려면
반드시 + Send + Sync를 명시해야 합니다.
Box<dyn Fn(...) + Send + Sync>

이렇게 하면 Rust가 컴파일 타임에
“이 함수는 스레드 간 안전하게 이동/참조 가능하다”고 판단합니다.

## ✅ 정리
| 타입           | 의미                         | 예시 타입                     | 주의사항 / 요구사항         |
|----------------|------------------------------|-------------------------------|------------------------------|
| `Send`         | 다른 스레드로 이동 가능       | `Vec<T>`, `Box<T>`, `Arc<T>`  | `Mutex<T>` 사용 시 필요      |
| `Sync`         | 여러 스레드에서 참조 가능     | `&T`                          | `Arc<T>`로 공유 시 필요      |
| `Box<dyn Fn>`  | 동적 함수 호출               | -                             | `+ Send + Sync + 'static` 필요 |


## 🧩 실전 팁
- Mutex<T>를 쓰려면 T: Send여야 함
- Arc<Mutex<T>>는 T: Send + Sync여야 함
- 클로저를 Box로 감쌀 때는 항상 + Send + Sync + 'static 붙이는 습관


## 전역 싱글톤 (Mutex 사용시)

전역 싱글톤으로 FunctionRegister를 만들면서 Mutex로 감쌌기 때문입니다.

## 🔍 핵심 상황 요약
### ✅ 우리가 했던 작업
```rust
pub static FUNCTION_REGISTER: Lazy<Mutex<FunctionRegister>> = Lazy::new(|| {
    let mut reg = FunctionRegister::new();
    reg.register_scalar("HIC15", ...);
    reg.register_vector("HeadAcc", ...);
    Mutex::new(reg)
});


pub enum FunctionType {
    Scalar(Box<dyn Fn(&[&TArray<f64>]) -> f64 + Send + Sync>),
    Vector(Box<dyn Fn(&[&TArray<f64>]) -> Vec<f64> + Send + Sync>),
}

pub fn register_scalar<F>(&mut self, name: &str, func: F)
where
    F: Fn(&[&TArray<f64>]) -> f64 + Send + Sync + 'static,

```

### ⚠️ 왜 에러가 났나?
- Mutex<T>는 내부 타입 T가 **Send** 를 만족해야 함
- FunctionRegister 안에는 Box<dyn Fn(...)>가 들어있음
- Box<dyn Fn>는 기본적으로 Send가 아니기 때문에
→ Mutex<FunctionRegister>가 스레드 안전하지 않다고 판단됨

### ✅ 해결 방법
우리가 함수 등록할 때 아래처럼 수정:
```rust
pub enum FunctionType {
    Scalar(Box<dyn Fn(&[&TArray<f64>]) -> f64 + Send + Sync>),
    Vector(Box<dyn Fn(&[&TArray<f64>]) -> Vec<f64> + Send + Sync>),
}
```

그리고 register_scalar() 함수도:
```rust
pub fn register_scalar<F>(&mut self, name: &str, func: F)
where
    F: Fn(&[&TArray<f64>]) -> f64 + Send + Sync + 'static,
```

이렇게 해서 FunctionRegister 전체가 Send를 만족하게 만들었고,
Mutex로 감싸서 전역에서 안전하게 사용할 수 있게 됨.

## 💡 정리
| 구성 요소         | 요구 조건 또는 설명           |
|------------------|-------------------------------|
| `Lazy<Mutex<...>>` | 전역 초기화 시 스레드 안전 필요 |
| `Mutex<T>`         | `T: Send` 여야 함             |
| `Box<dyn Fn>`      | 기본적으로 `Send` 아님         |
| `Send + Sync`      | 명시적으로 붙여야 안전하게 공유 가능 |

즉, 우리가 직접 thread::spawn()을 쓰진 않았지만
전역 공유를 위해 Mutex를 썼기 때문에 Send와 Sync가 필요했던 것.

## Rust의 철학:
“멀티스레드를 쓰든 안 쓰든, 공유하려면 안전하게 해라.”


## 🔧 예시로 이해하기
- Lazy<Mutex<FunctionRegister>>를 만들려면
→ FunctionRegister가 Send를 만족해야 함
→ 내부에 있는 Box<dyn Fn>도 Send + Sync로 명시해야 함
```rust
Box<dyn Fn(...) + Send + Sync + 'static>
```
이렇게 하면 Mutex로 감싸도 컴파일 타임에 스레드 안전성이 보장됩니다.

## 실전 예제
```rust
type AnalysisScenario = Box<dyn Fn(&mut ResultContainer, &DataContainer) + Send + Sync>;

pub struct ScenarioRegister {
    scenarios: HashMap<String, AnalysisScenario>,
}

impl ScenarioRegister {
    pub fn new() -> Self {
        Self { scenarios: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, func: AnalysisScenario) {
        self.scenarios.insert(name.to_string(), func);
    }

    pub fn run(&self, name: &str, result: &mut ResultContainer, data: &DataContainer) -> bool {
        if let Some(func) = self.scenarios.get(name) {
            func(result, data);
            true
        } else {
            false
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.scenarios.contains_key(name)
    }

    pub fn list(&self) -> Vec<&String> {
        self.scenarios.keys().collect()
    }
}


pub static SCENARIO_REGISTER: Lazy<Mutex<ScenarioRegister>> = Lazy::new(|| {
    let mut reg = ScenarioRegister::new();
    reg.register("HeadInjury",  Box::new(calc_head_injury));
    Mutex::new(reg)
});
```
---

## Send + Sync
- i8, f32, bool, char, &str, …  
- (T1, T2), [T; N], &[T], struct { x: T }, …  
- String, Option<T>, Vec<T>, Box<T>, …  
- Arc<T>: 참조 카운트 조작을 아토믹 하기 때문에 스레드 안전함.  
- Mutex<T>: 값을 접근하기 위해 뮤택스를 잠궈야 하기 때문에 스레드 안전함.  
- AtomicBool, AtomicU8, …: 값을 접근할 때 특별한 아토믹 명령어들을 사용합니다.  
- 제네릭 타입은 일반적으로 타입 파라메터가 Send + Sync이면 Send + Sync 입니다.  

## Send + !Sync
- 아래 타입들은 다른 스레드로 이동될 수 있지만 내부적으로 값이 변경될 수 있기 때문에 스레드 안전하지 않습니다:
    - mpsc::Sender<T>
    - mpsc::Receiver<T>
    - Cell<T>
    - RefCell<T>

## !Send + Sync
- 아래 타입들은 스레드 안전하지만 다른 스레드로 이동될 수 없습니다:
    - MutexGuard<T: Sync>: Uses OS level primitives which must be deallocated on the thread which created them.

## !Send + !Sync
- 아래 타입들은 스레드 안전하지도 않고 다른 스레드로 이동될 수도 없습니다:
    - Rc<T>: Rc<T> 는 아토믹하지 않은 방식으로 참조 카운트를 조작하는 RcBox<T>를 참조합니다.
    - *const T, *mut T: 러스트는 포인터가 스레드 안전하지 않다고 가정합니다.

---
## 상태 공유

- 러스트는 주로 아래 두 가지 타입을 이용해서 공유 데이터 동기화를 수행합니다:
- Arc<T>, T에 대한 아토믹 참조 카운트: 이 참조는 다수의 스레드 사이에서 공유될 수 있고, 참조하던 마지막 스레드가 종료할 경우 T를 반환합니다.
- Mutex<T>: T값에 대한 상호 배제 엑세스를 보장합니다.



