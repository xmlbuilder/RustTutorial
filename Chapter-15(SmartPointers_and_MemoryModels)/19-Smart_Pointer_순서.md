# 타입 래핑 순서
`Arc<Mutex<T>>`, `Rc<RefCell<T>>` 같은 타입 래핑은 동시성, 소유권, 가변성을 제어하기 위한 핵심 패턴입니다.  
아래에 각 조합의 의미, 사용 목적, 묶는 순서, 그리고 실전 패턴을 정리.

## 🧠 핵심 개념 요약

| 타입           | 설명                                      | 주요 메서드 또는 특징     |
|----------------|-------------------------------------------|----------------------------|
| `Rc<T>`        | 단일 스레드에서 참조 카운트 기반 공유 소유권     | `clone()`                  |
| `Arc<T>`       | 멀티 스레드에서 안전한 참조 카운트 공유 소유권   | `clone()` (atomic)        |
| `RefCell<T>`   | 단일 스레드에서 런타임 내부 가변성 제공          | `borrow()`, `borrow_mut()`|
| `Mutex<T>`     | 멀티 스레드에서 안전한 가변성 제공               | `lock()`                  |


## 🧩 조합별 의미와 사용 패턴
### ✅ Rc<RefCell<T>>
- 단일 스레드에서 여러 소유자가 가변 데이터를 공유할 때 사용
- Rc → 공유 소유권
- RefCell → 내부 가변성 (런타임 체크)
```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(5));
*data.borrow_mut() += 1;
```            
- 📌 패턴 용도: GUI, 트리 구조, 단일 스레드 앱에서 노드 간 공유

### ✅ Arc<Mutex<T>>
- 멀티 스레드 환경에서 여러 스레드가 가변 데이터를 안전하게 공유
- Arc → 원자적 참조 카운트로 공유 소유권
- Mutex → 스레드 안전한 가변성
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10).map(|_| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1;
    })
}).collect();

for h in handles {
    h.join().unwrap();
}
```
- 📌 패턴 용도: 병렬 처리, 웹 서버, 채널 공유 상태 등

### ✅ Rc<Mutex<T>> (⚠️ 드물게 사용)
- 단일 스레드에서 락 기반 가변성이 필요할 때
- RefCell 대신 Mutex를 쓰는 경우 (예: API 요구)
- 📌 주의: Mutex는 스레드 안전을 위한 것이므로 Rc와 함께 쓰는 건 일반적이지 않음

### ✅ Arc<RefCell<T>> (⚠️ 위험)
- 멀티 스레드에서 RefCell 사용은 안전하지 않음
- RefCell은 런타임 체크만 하므로 데이터 경쟁(race condition) 발생 가능
- 📌 권장하지 않음: 대신 Arc<Mutex<T>>를 사용

### 🧭 묶는 순서 정리
```
Arc<Mutex<T>>     // 멀티 스레드 + 가변성
Rc<RefCell<T>>    // 단일 스레드 + 가변성
Rc<T>             // 단일 스레드 + 불변 공유
Arc<T>            // 멀티 스레드 + 불변 공유
```
- 가변성이 필요하면: Mutex (멀티 스레드), RefCell (단일 스레드)
- 공유가 필요하면: Arc (멀티 스레드), Rc (단일 스레드)

## 🧪 실전 패턴 예시

| 상황                     | 추천 타입               |
|--------------------------|--------------------------|
| 트리 구조 (단일 스레드) | `Rc<RefCell<Node>>`     |
| 웹 서버 상태 공유       | `Arc<Mutex<State>>`     |
| GUI 위젯 공유           | `Rc<RefCell<Widget>>`   |
| 병렬 작업 결과 수집     | `Arc<Mutex<Vec<T>>>`    |

---

# RefCell<Rc<T>> / Rc<RefCell<T>>

`RefCell<Rc<T>>` 는 Rust에서 흔치 않지만 동작은 가능합니다.  
다만 이 조합은 일반적인 패턴인 `Rc<RefCell<T>>` 와는 의미와 목적이 완전히 다릅니다.  
아래에 두 조합을 비교하면서 `RefCell<Rc<T>>` 의 의미를 설명.

## 🔄 비교: Rc<RefCell<T>> vs RefCell<Rc<T>>

| 조합               | 의미 및 목적                                               |
|--------------------|------------------------------------------------------------|
| `Rc<RefCell<T>>`     | 여러 소유자가 하나의 가변 데이터를 공유함 (가장 일반적인 패턴) |
| `RefCell<Rc<T>>`     | 하나의 소유자가 공유 대상(Rc)을 런타임에 변경하고 싶을 때 사용 |


## 🔍 RefCell<Rc<T>>의 의미
- `Rc<T>` 는 불변 공유 소유권을 나타냅니다
- `RefCell<Rc<T>>` 는 그 Rc<T> 자체를 런타임에 변경 가능하게 만듭니다
즉, 이 조합은 공유 대상 자체를 바꾸고 싶을 때 사용됩니다. 예를 들어:  

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Data {
    value: i32,
}
```
```rust        
fn main() {
    let a = Rc::new(Data { value: 1 });
    let b = Rc::new(Data { value: 2 });

    let holder = RefCell::new(a); // RefCell<Rc<Data>>

    println!("{:?}", holder.borrow()); // Rc<Data> with value 1

    // Rc<Data> 자체를 변경
    *holder.borrow_mut() = b;

    println!("{:?}", holder.borrow()); // Rc<Data> with value 2
}
```
### 📌 요점
- `RefCell<Rc<T>>` 는 데이터를 공유하는 게 아니라, 어떤 데이터를 공유할지 바꾸는 용도
- `Rc<RefCell<T>>` 는 하나의 데이터를 여러 소유자가 공유하면서 수정 가능하게 하는 용도

## 🧠 요약
| 조합               | 목적                                      | 예시 상황                         |
|--------------------|-------------------------------------------|------------------------------------|
| `Rc<RefCell<T>>`     | 여러 소유자가 하나의 가변 데이터를 공유      | 트리 구조, GUI 위젯 공유 등        |
| `RefCell<Rc<T>>`     | 하나의 소유자가 공유 대상 자체를 바꾸고 싶을 때 | 상태 스왑, 동적 참조 변경 등       |

--- 

# Arc<Mutex<Vec<T>>> / Vec<Arc<Mutex<T>>>

`Arc<Mutex<Vec<T>>>` 와 `Vec<Arc<Mutex<T>>>` 는 동작은 비슷해 보일 수 있지만 의미와 동시성 제어 방식이 완전히 다릅니다.  
아래에 두 구조의 차이점과 사용 목적을 명확하게 정리.

## 🔄 비교: Arc<Mutex<Vec<T>>> vs Vec<Arc<Mutex<T>>>
| 구조                     | 의미 및 동작 방식                                                                 |
|--------------------------|------------------------------------------------------------------------------------|
| `Arc<Mutex<Vec<T>>>`       | 하나의 전체 벡터를 여러 스레드가 공유하며, 벡터 전체에 대해 락을 걸고 접근함         |
| `Vec<Arc<Mutex<T>>>`       | 벡터는 불변이며, 각 요소 T에 대해 개별적으로 락을 걸 수 있음 (미세한 동시성 제어 가능) |


## ✅ Arc<Mutex<Vec<T>>>
- 전체 벡터를 하나의 락으로 감싸서 공유
- 모든 스레드가 같은 벡터를 공유하며, 접근 시 lock()으로 전체를 잠금
- 예: 여러 스레드가 하나의 작업 큐를 공유할 때
```rust
let shared_vec = Arc::new(Mutex::new(vec![1, 2, 3]));
let cloned = Arc::clone(&shared_vec);
let mut vec = cloned.lock().unwrap();
vec.push(4);
```
- 📌 장점: 구조가 단순하고 전체 데이터 일괄 처리에 적합
- 📌 단점: 병렬성이 낮음 — 하나의 락이 전체를 잠금

## ✅ Vec<Arc<Mutex<T>>>
- 벡터 자체는 불변이며, 각 요소 T를 Arc<Mutex<T>>로 감싸서 개별적으로 락 가능
- 예: 여러 스레드가 각기 다른 객체를 병렬로 처리할 때
```rust
let items: Vec<Arc<Mutex<i32>>> = vec![
    Arc::new(Mutex::new(1)),
    Arc::new(Mutex::new(2)),
    Arc::new(Mutex::new(3)),
];

let item = Arc::clone(&items[0]);
let mut val = item.lock().unwrap();
*val += 10;
```
- 📌 장점: 병렬성이 높음 — 각 요소에 대해 독립적으로 락 가능
- 📌 단점: 구조가 복잡해지고, 전체 순회 시 락 관리가 까다로움

## 구조도
아래는 Arc<Mutex<Vec<T>>>와 Vec<Arc<Mutex<T>>>의 구조적 차이를 시각적으로 표현한 다이어그램입니다..

```mermaid
graph TD
  subgraph Arc<Mutex<Vec<T>>>
    A1[Arc]
    A2[Mutex]
    A3[Vec]
    A4[T1]
    A5[T2]
    A6[T3]
    A1 --> A2
    A2 --> A3
    A3 --> A4
    A3 --> A5
    A3 --> A6
  end

  subgraph Vec<Arc<Mutex<T>>>
    B1[Vec]
    B2[Arc1]
    B3[Mutex1]
    B4[T1]
    B5[Arc2]
    B6[Mutex2]
    B7[T2]
    B8[Arc3]
    B9[Mutex3]
    B10[T3]
    B1 --> B2
    B2 --> B3
    B3 --> B4
    B1 --> B5
    B5 --> B6
    B6 --> B7
    B1 --> B8
    B8 --> B9
    B9 --> B10
  end
```
## 📌 설명
- `Arc<Mutex<Vec<T>>>`: 하나의 `Vec<T>` 를 Mutex로 감싸고, 그 전체를 Arc로 공유
- `Vec<Arc<Mutex<T>>>`: 각 T를 Mutex로 감싸고, 각각을 Arc로 공유 → 병렬 처리에 유리

## 🧠 요약
| 목적                          | 추천 구조               |
|-------------------------------|--------------------------|
| 전체 데이터 공유 및 일괄 처리 | `Arc<Mutex<Vec<T>>`     |
| 각 요소 병렬 처리 및 독립 제어 | `Vec<Arc<Mutex<T>>`     |

---










