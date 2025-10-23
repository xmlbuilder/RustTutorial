# Upcasting 

Rust에서 Arc, Box, Rc 같은 스마트 포인터를 사용할 때 trait object로 업캐스팅하는건 실전에서 자주 마주치는 고급 개념.  
특히 Observer, Register, Plugin 구조에서 필수.

## 🧠 기본 개념: 업캐스팅이란?
Rust는 정적 타입 언어라서
Arc<ConcreteType>와 Arc<dyn Trait>는 서로 다른 타입입니다.  
업캐스팅은 ConcreteType이 Trait을 구현하고 있을 때,  
스마트 포인터를 dyn Trait로 변환하는 작업입니다.

## ✅ Box, Arc, Rc 업캐스팅 예시
### 🔹 Box<T> → Box<dyn Trait>
```rust
struct MyType;
impl MyTrait for MyType {}

let boxed: Box<MyType> = Box::new(MyType);
let trait_box: Box<dyn MyTrait> = boxed; // OK
``
- Box는 소유권을 가지므로 업캐스팅이 가장 간단함
- 런타임에 vtable을 붙여서 dyn Trait로 변환

### 🔹 `Arc<T>` → `Arc<dyn Trait>`
```rust
let arc: Arc<MyType> = Arc::new(MyType);
let trait_arc: Arc<dyn MyTrait> = arc; // OK
```

- Arc는 참조 카운트 기반 스마트 포인터
- 업캐스팅 시 Arc::new(...)로 만든 후 Arc<dyn Trait>로 변환 가능
- 주의: Arc::ptr_eq()로 비교하려면 동일한 Arc<dyn Trait> 타입이어야 함

### 🔹 Rc<T> → Rc<dyn Trait>
```rust
let rc: Rc<MyType> = Rc::new(MyType);
let trait_rc: Rc<dyn MyTrait> = rc; // OK
```
- Rc는 싱글 스레드용 참조 카운트
- Arc와 동일한 방식으로 업캐스팅 가능
- Rc::ptr_eq()도 동일하게 작동

## ⚠️ 스마트 포인터 업캐스팅 주의사항
| 항목                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `Arc<Concrete>` → `Arc<dyn Trait>` | 자동 변환되지 않음 → 명시적 업캐스팅 필요 (`Arc::new(...) as Arc<dyn Trait>`) |
| `Arc::ptr_eq()`               | 비교하려면 **같은 타입의 `Arc<dyn Trait>`** 이어야 함               |
| `dyn Trait + Send + Sync`     | 멀티스레드 환경에서는 trait object에 **`Send + Sync` 명시** 필요     |
| `Clone`                       | `Arc`, `Rc`는 참조 카운트 기반 → `clone()`으로 복제 가능             |

---

## 🔧 실전 예시: 옵저버 등록/해제

## 전체 코드
```rust
pub trait ResultObserver: Send + Sync {
    fn notify(&self, key: &str);
}

use std::collections::HashMap;
use std::sync::Arc;
use crate::core::calc_injury::{compute_hic15, compute_resultant};
use crate::core::data_container::DataContainer;
use crate::core::tarray::TArray;
use crate::traits::result_observer::ResultObserver;

pub struct ResultContainer {
    channels: HashMap<String, TArray<f64>>,
    metrics: HashMap<String, f64>,
    observers: Vec<Arc<dyn ResultObserver>>,
}

impl ResultContainer {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            metrics: HashMap::new(),
            observers: vec![],
        }
    }

    // -------------------- 저장 --------------------

    /// 채널 결과 저장
    pub fn insert_channel(&mut self, name: &str, data: TArray<f64>) {
        self.channels.insert(name.to_string(), data);
        self.notify_observers(name);
    }

    /// 단일 값 결과 저장
    pub fn insert_metric(&mut self, key: &str, value: f64) {
        self.metrics.insert(key.to_string(), value);
        self.notify_observers(key);

    }

    // -------------------- 조회 --------------------

    /// 채널 결과 조회
    pub fn get_channel(&self, name: &str) -> Option<&TArray<f64>> {
        self.channels.get(name)
    }

    /// 단일 값 결과 조회
    pub fn get_metric(&self, key: &str) -> Option<f64> {
        self.metrics.get(key).copied()
    }

    /// 채널 이름 목록
    pub fn get_channel_names(&self) -> Vec<&String> {
        self.channels.keys().collect()
    }

    /// 결과 키 목록
    pub fn get_metric_keys(&self) -> Vec<&String> {
        self.metrics.keys().collect()
    }

    /// 채널 존재 여부
    pub fn has_channel(&self, name: &str) -> bool {
        self.channels.contains_key(name)
    }

    /// 결과 키 존재 여부
    pub fn has_metric(&self, key: &str) -> bool {
        self.metrics.contains_key(key)
    }

    // -------------------- 분석 실행 --------------------

    /// 분석 함수 실행 후 단일 값 저장
    pub fn run_scalar_metric<F>(
        &mut self,
        name: &str,
        data_container: &DataContainer,
        channel_names: &[&str],
        func: F,
    )
    where
        F: Fn(&[&TArray<f64>]) -> f64,
    {
        if let Some(arrays) = data_container.compute_injury_metric(channel_names, |arr| vec![func(arr)]) {
            self.insert_metric(name, arrays[0]);
        }
    }

    /// 분석 함수 실행 후 벡터 값 저장
    pub fn run_vector_metric<F>(
        &mut self,
        name: &str,
        data_container: &DataContainer,
        channel_names: &[&str],
        func: F,
    )
    where
        F: Fn(&[&TArray<f64>]) -> Vec<f64>,
    {
        if let Some(result) = data_container.compute_injury_metric(channel_names, func) {
            self.insert_channel(name, TArray::from(result));
        }
    }

    pub fn add_observer(&mut self, observer: Arc<dyn ResultObserver>) {
        self.observers.push(observer);
    }
    pub fn detach_observer(&mut self, target: &Arc<dyn ResultObserver>) {
        self.observers.retain(|obs| !Arc::ptr_eq(obs, target));
    }

    fn notify_observers(&self, key: &str) {
        for obs in &self.observers {
            obs.notify(key);
        }
    }
}

```


### 핵심 코드
```rust
let concrete = Arc::new(GUIState::new());
let observer: Arc<dyn ResultObserver> = concrete.clone(); // 업캐스팅

result.add_observer(observer.clone());
result.detach_observer(&observer); // 타입 일치!
```

- GUIState가 ResultObserver를 구현하고 있어야 함
- detach_observer()에서 Arc::ptr_eq()로 비교하려면  
    **같은 타입의 Arc<dyn Trait>** 를 넘겨야 함

## 🧩 스마트 포인터 업캐스팅 요약 표

| 스마트 포인터 | 업캐스팅 가능 여부         | 스레드 안전성 (`Send`, `Sync`) |
|----------------|-----------------------------|-------------------------------|
| `Box<T>`        | ✅ `Box<dyn Trait>` 가능     | ❌ 기본적으로 `Send` 아님       |
| `Arc<T>`        | ✅ `Arc<dyn Trait>` 가능     | ✅ `Send + Sync` 가능           |
| `Rc<T>`         | ✅ `Rc<dyn Trait>` 가능      | ❌ `Send` 불가 (`!Send`)        |
| `&T`            | ✅ `&dyn Trait` 가능         | 🔄 수명 관리 필요               |


### ✅ 해결 방법: Arc<GUIState> → Arc<dyn ResultObserver>로 업캐스팅
```rust
let gui_state: Arc<GUIState> = Arc::new(GUIState::new());
let observer: Arc<dyn ResultObserver> = gui_state.clone(); // 업캐스팅

result.add_observer(observer.clone());
result.detach_observer(&observer); // 타입 일치!
```


### 🔍 왜 필요한가?
Rust는 Arc<GUIState>와 Arc<dyn ResultObserver>를  
서로 다른 타입으로 간주합니다.
심지어 GUIState가 ResultObserver를 구현하고 있어도  
Arc<GUIState>는 자동으로 Arc<dyn ResultObserver>로 변환되지 않음.
그래서 명시적으로 업캐스팅해야 합니다:

```rust
let observer: Arc<dyn ResultObserver> = gui_state.clone();
```


## ✨ 전체 흐름 예시
```rust
let gui_state: Arc<GUIState> = Arc::new(GUIState::new());
let observer: Arc<dyn ResultObserver> = gui_state.clone();

result.add_observer(observer.clone());
result.insert("HIC15", 42.0);
assert!(gui_state.was_notified_with("HIC15"));

gui_state.notified_keys.lock().unwrap().clear();

result.detach_observer(&observer);
result.insert("HIC15", 99.0);
assert!(!gui_state.was_notified_with("HIC15"));
```

이제 detach_observer()도 완벽하게 작동하고,
테스트에서도 옵저버 등록/해제 흐름을 깔끔하게 확인할 수 있습니다.


### 🔍 왜 clone()이 필요한가?
- Arc<T>는 참조 카운트 기반 스마트 포인터입니다.
- `Arc::clone()` 을 호출하면 같은 데이터를 가리키는 또 다른 Arc 인스턴스를 생성합니다.
- 이때 내부 데이터는 복사되지 않고, 참조 카운트만 증가합니다.

### 🧠 이 코드에서 일어나는 일
```rust
let concrete: Arc<GUIState> = Arc::new(GUIState::new());
let observer: Arc<dyn ResultObserver> = concrete.clone(); // 업캐스팅
```

- concrete는 Arc<GUIState> 타입
- clone()을 호출하면 Arc<GUIState>가 하나 더 생김
- 이걸 Arc<dyn ResultObserver>로 업캐스팅하면서 observer에 할당
즉, clone()은 참조를 하나 더 만들기 위한 안전한 방법이고,  
그걸 dyn Trait로 업캐스팅해서 trait object로 사용하는 것임.

### ✅ 왜 그냥 let observer = concrete; 하면 안 되나?
Rust는 `move semantics` 가 기본이라서  
concrete를 그대로 observer에 넘기면 concrete를 사용할 수 없게 됩니다.
```rust
let observer: Arc<dyn ResultObserver> = concrete; // ❌ concrete는 move됨
```

이렇게 하면 concrete는 더 이상 사용할 수 없고,
테스트나 후속 로직에서 concrete를 참조하려면 `clone()` 이 필요합니다.

## ✨ 업캐스팅과 clone() 요약

| 항목        | 설명                                                                 |
|-------------|----------------------------------------------------------------------|
| `clone()`   | `Arc`나 `Rc`의 참조 카운트를 증가시켜 **원본을 유지하면서 복제** 함         |
| 업캐스팅    | `Arc<GUIState>` → `Arc<dyn ResultObserver>`로 **명시적 타입 변환 필요**     |
| 이유        | `Arc::ptr_eq()` 등에서 **동일한 타입의 `Arc<dyn Trait>`** 이어야 비교 가능함 |


---

## 🔍 기본 개념: Trait Object와 Lifetime
```rust
let rc: Rc<MyType> = Rc::new(MyType);
let trait_rc: Rc<dyn MyTrait> = rc; // OK
```

이 코드는 다음과 같은 전제 하에 컴파일됩니다:
- MyType: MyTrait + 'static
- 즉, MyType의 인스턴스가 'static lifetime을 가지므로  
    Rc<dyn MyTrait>도 'static lifetime을 가질 수 있음

## ⚠️ 만약 lifetime이 짧으면?
```rust
fn make_trait_object<'a>(value: &'a MyType) -> Rc<dyn MyTrait + 'a> {
    let rc = Rc::new(value); // Rc<&'a MyType>
    rc // ❌ 에러: Rc<&MyType>는 Rc<dyn MyTrait + 'a>로 업캐스팅 불가
}
```

- 여기서는 Rc<&MyType>이므로 내부 타입이 참조
- Rc<dyn MyTrait>로 업캐스팅하려면 MyType 자체가 들어 있어야 함
- 참조 타입을 trait object로 만들려면 &dyn Trait 또는 Box<dyn Trait>가 더 적합

## ✅ 안전한 업캐스팅 조건
| 조건                        | 업캐스팅 가능 형태             | 특이사항 또는 제약 조건             |
|-----------------------------|-------------------------------|-------------------------------------|
| `T: Trait + 'static`        | `Rc<dyn Trait>` / `Arc<dyn Trait>` | `'static` lifetime 필요             |
| 참조 타입 (`&T`)            | `&dyn Trait`                  | lifetime 명시 필요 (`'a`)           |
| `Box<T>`                    | `Box<dyn Trait>`              | 가장 유연함, 소유권 기반            |


## ✨ 예시: lifetime 명시된 trait object
```rust
fn make_trait<'a>(value: &'a MyType) -> Box<dyn MyTrait + 'a> {
    Box::new(value) // OK
}
```
- Box<dyn Trait + 'a> 형태로 lifetime을 명시하면
참조 기반 trait object도 안전하게 생성 가능


## 🔍 왜 Rc<&'a MyType> → Rc<dyn MyTrait + 'a>는 안 되는가?
```rust
fn make_trait_object<'a>(value: &'a MyType) -> Rc<dyn MyTrait + 'a> {
    let rc = Rc::new(value); // Rc<&'a MyType>
    rc // ❌ 에러
}
```

- Rc::new(value)는 Rc<&'a MyType>을 생성함
- 즉, 참조 타입을 Rc로 감싼 것
- Rust는 Rc<&T>를 Rc<dyn Trait>로 직접 업캐스팅할 수 없음  
    → 이유: Rc<&T>는 참조를 감싼 스마트 포인터이고,  
    Rc<dyn Trait>는 trait object 자체를 감싼 스마트 포인터  
    즉, 타입이 완전히 다릅니다.

## ✅ 왜 Box<dyn MyTrait + 'a>는 되는가?
```rust
fn make_trait<'a>(value: &'a MyType) -> Box<dyn MyTrait + 'a> {
    Box::new(value) // OK
}
```

- Box::new(value)는 Box<&'a MyType>을 생성
- Rust는 Box<&'a MyType> → Box<dyn MyTrait + 'a>로 자동 업캐스팅 가능
- 이유: Box는 소유권을 가지며, 내부 타입이 trait object로 변환될 수 있음
- Box<dyn Trait>는 런타임에 vtable을 붙여서 trait object로 안전하게 변환됨

## 🧠 핵심 차이 요약
| 원래 타입         | 업캐스팅 대상           | 가능 여부 / 이유                      |
|-------------------|--------------------------|----------------------------------------|
| `Rc<&T>`          | `Rc<dyn Trait>`          | ❌ 불가능 — 참조를 감싼 포인터는 업캐스팅 불가 |
| `Box<&T>`         | `Box<dyn Trait>`         | ✅ 가능 — `Box`는 참조도 trait object로 감쌀 수 있음 |
| `Box<T>`          | `Box<dyn Trait>`         | ✅ 가능 — 가장 일반적인 trait object 업캐스팅 방식 |


## ✨ 비유로 이해하면
- Rc<&T>는 **참조를 공유하는 포인터**
- Box<T>는 **실제 값을 소유하는 포인터**
- trait object는 vtable을 포함한 동적 디스패치 구조이므로  
    실제 값을 소유하거나 참조해야 안전하게 만들 수 있음
- Box는 그걸 만족하지만 Rc<&T>는 이중 포인터가 되어버려서 안 됩니다

---

## 컴파일 안되는 이유 분석 상세
```rust
fn make_trait_object<'a>(value: &'a MyType) -> Rc<dyn MyTrait + 'a> {
    let rc = Rc::new(value); // Rc<&'a MyType>
    rc // ❌ 에러: Rc<&MyType>는 Rc<dyn MyTrait + 'a>로 업캐스팅 불가
}
```

## 🧩 코드 분석: 왜 Rc<&MyType> → Rc<dyn MyTrait> 업캐스팅이 안 되는가?
## 1️⃣ value: &'a MyType → Rc::new(value)의 결과
- value는 &'a MyType 타입의 참조입니다.
- Rc::new(value)는 Rc<&'a MyType>을 생성합니다.
- 즉, Rc가 MyType을 직접 소유하는 것이 아니라 참조를 소유합니다.

### 2️⃣ 원하는 반환 타입: Rc<dyn MyTrait + 'a>
- 이 타입은 trait object를 Rc로 감싸는 형태입니다.
- 즉, Rc가 dyn MyTrait을 직접 소유해야 합니다.
- 하지만 현재 Rc<&MyType>은 MyType의 참조를 소유하고 있어서 타입이 다릅니다.

## 3️⃣ 왜 업캐스팅이 안 되는가?
- Rust는 Rc<&MyType>을 Rc<dyn MyTrait>로 자동 변환하지 않습니다.
- 이유는 다음과 같습니다:
  
| 표현             | 설명                                                              |
|------------------|-------------------------------------------------------------------|
| `Rc<&MyType>`     | `Rc`가 `MyType`의 참조를 소유함 → 타입: `Rc<&MyType>`             |
| `Rc<dyn MyTrait>` | `Rc`가 trait object를 직접 소유함 → 타입: `Rc<dyn MyTrait>`       |
| 문제점            | `&MyType`는 `dyn MyTrait`로 자동 변환되지 않음                   |
| 타입 불일치       | `Rc<&MyType>` ≠ `Rc<dyn MyTrait>`                                 |

## 4️⃣ 해결 방법
### ✅ 방법 1: value를 직접 Rc로 감싸고 dyn MyTrait로 캐스팅
```rust
fn make_trait_object<'a>(value: MyType) -> Rc<dyn MyTrait> {
    Rc::new(value) as Rc<dyn MyTrait>
}
```
- 이 경우 MyType을 직접 소유하므로 Rc<MyType> → Rc<dyn MyTrait>로 업캐스팅 가능

### ✅ 방법 2: Rc::from(value as &dyn MyTrait) 사용
```rust
fn make_trait_object<'a>(value: &'a MyType) -> Rc<dyn MyTrait + 'a> {
    Rc::from(value as &dyn MyTrait)
}
```
- value as &dyn MyTrait로 먼저 trait object로 변환한 뒤 Rc::from()으로 감쌈
- 이 경우 Rc는 dyn MyTrait 참조를 소유하게 됨 → 타입 일치

## ✅ 핵심 요약
| 표현                   | 설명                                                              |
|------------------------|-------------------------------------------------------------------|
| `Rc<&MyType>`          | `Rc`가 `MyType`의 참조를 소유함 (`Rc<&T>` 형태)                   |
| `Rc<dyn MyTrait>`      | `Rc`가 trait object를 직접 소유함 (`Rc<dyn Trait>` 형태)          |
| 타입 불일치            | `Rc<&T>`는 `Rc<dyn Trait>`로 자동 업캐스팅되지 않음              |
| 변환 방법              | `Rc::from(value as &dyn Trait)` 또는 `Rc::new(value)` 후 캐스팅   |

---

# 컴파일 되는 이유 분석 상세
```rust
fn make_trait<'a>(value: &'a MyType) -> Box<dyn MyTrait + 'a> {
    Box::new(value) // OK
}
```

## ✅ 왜 Box::new(value)는 Box<dyn MyTrait>로 변환 가능한가?
### 1️⃣ value: &'a MyType — 참조 타입
- value는 &MyType 타입입니다.
- MyType이 MyTrait을 구현하고 있다면, &MyType은 dyn MyTrait로 자동 변환될 수 있음.
- 이 변환은 trait object로의 자동 참조 변환(coercion) 덕분입니다.

### 2️⃣ Box::new(value) — Box가 참조를 감쌈
- Box::new(value)는 Box<&MyType>을 생성합니다.
- 그런데 Rust는 Box<&MyType>을 자동으로 Box<dyn MyTrait>로 변환해줄 수 있음.
- 이건 Rust가 &MyType을 &dyn MyTrait로 변환할 수 있다는 사실을 알고 있기 때문입니다.

### 3️⃣ 핵심: Box<T>는 Box<dyn Trait>로 업캐스팅 가능
- Rust는 다음과 같은 변환을 허용합니다:
```rust
let boxed: Box<dyn MyTrait> = Box::new(&my_type); // OK
```
- 이건 &MyType → &dyn MyTrait 변환이 가능하고,
- Box<&MyType> → Box<dyn MyTrait>로도 자동 업캐스팅이 가능하기 때문입니다.

### 4️⃣ 왜 Rc::new(value)는 안 됐었나?
- Rc는 Box와 달리 자동 업캐스팅을 지원하지 않습니다.
- Rc<&MyType>은 Rc<dyn MyTrait>로 변환되지 않음.
- 대신 명시적으로 Rc::from(value as &dyn MyTrait)처럼 작성해야 합니다.

## 🔍 비교 요약
| 표현                          | 가능 여부 | 변환 방식 설명                                 |
|-------------------------------|------------|------------------------------------------------|
| `Box::new(&MyType)` → `Box<dyn MyTrait>` | ✅         | `&MyType` → `&dyn MyTrait` 자동 변환 + `Box`는 업캐스팅 지원 |
| `Rc::new(&MyType)` → `Rc<dyn MyTrait>`  | ❌         | `Rc`는 자동 업캐스팅 미지원 → `Rc::from(value as &dyn Trait)` 필요 |


✅ 결론
- Box는 trait object로의 자동 업캐스팅을 지원하므로 Box::new(&value)가 Box<dyn Trait>로 변환 가능
- Rc는 이 자동 변환을 지원하지 않기 때문에 명시적 캐스팅이 필요함
- 이 차이는 스마트 포인터의 trait object 처리 방식에서 비롯됩니다

---

## 🧠 핵심 차이: Box vs Rc의 메모리 구조
| 항목                 | `Box<T>`                                | `Rc<T>`                                 |
|----------------------|------------------------------------------|------------------------------------------|
| 소유 방식            | 단일 소유자                              | 참조 카운트 기반 공유 소유자             |
| 메모리 배치          | 힙에 `T`를 직접 저장                     | 힙에 `RcHeader` + `T` 구조로 저장         |
| trait object 변환     | `Box<T>` → `Box<dyn Trait>` 가능         | `Rc<T>` → `Rc<dyn Trait>` 불가능          |
| 변환 방식            | 자동 업캐스팅 지원 (`Box::new(&T)`)     | 명시적 변환 필요 (`Rc::from(&T as &dyn Trait)`) |
| 사용 용도            | 단독 소유, 간단한 구조                   | 다중 소유, 복잡한 내부 구조               |


## 🔍 구조적 차이 설명
### 1️⃣ Box<T>는 힙에 T를 직접 저장
- Box<T>는 T를 힙에 직접 저장하고, 그 포인터를 가지고 있음.
- 그래서 Box<MyType>은 Box<dyn MyTrait>로 안전하게 업캐스팅 가능:
- Box<MyType> → Box<dyn MyTrait>: 포인터만 바꾸면 됨
### 2️⃣ Rc<T>는 내부에 참조 카운트 메타데이터가 있음
- Rc<T>는 힙에 다음과 같은 구조를 저장:
```
[ RcHeader | T ]
```
- RcHeader에는 참조 카운트, drop 체크 등 메타데이터가 들어 있음.
- 이 구조 때문에 Rc<MyType>을 Rc<dyn MyTrait>로 직접 변환할 수 없음:
- Rc<&MyType> → Rc<dyn MyTrait>는 타입 불일치 + 메모리 레이아웃 불일치

## 📦 왜 Box는 되는가?
- Box<T>는 단순한 포인터 → Box<dyn Trait>로 변환 시, 포인터만 바꾸면 끝
- Rust는 이 변환을 자동으로 지원함
## 🚫 왜 Rc는 안 되는가?
- Rc<T>는 내부 구조가 고정되어 있어 Rc<dyn Trait>로 변환하려면 전체 구조를 새로 만들어야 함
- Rust는 이 변환을 자동으로 지원하지 않음
- 대신 `Rc::from(value as &dyn Trait)` 처럼 명시적 참조 변환을 해야 함

## ✅ 정리
- Box는 힙에 T를 직접 저장하므로 trait object로의 변환이 자연스럽고 안전함
- Rc는 참조 카운트 기반 구조를 가지므로 trait object로의 자동 변환이 불가능함
- 이 차이는 Rust의 메모리 안전성과 타입 안정성을 위한 설계 결정

---







