# Multithreading 개요
멀티스레딩은 하나의 프로세스 안에서 여러 OS 스레드를 생성해 일을 병렬적(물리 코어가 여러 개일 때) 또는 동시적(단일 코어에서 시분할)으로 처리하는 방법입니다.  
Rust는 “데이터 레이스 없는 동시성”을 목표로, 컴파일 타임의 소유권/빌림 검사와 트레잇 경계를 통해 안전한 멀티스레딩을 강력하게 보장합니다.
- 병렬성: 여러 작업이 물리적으로 동시에 실행.
- 동시성: 단일(또는 적은 수의) 코어에서 작업들이 번갈아 수행.
- Rust의 강점: 타입 시스템(소유권, 라이프타임, Send/Sync) + 안전한 표준 동기화 원시(Mutex, RwLock, Condvar, Arc, atomic 등).

## Rust의 스레딩 모델
- OS 스레드: std::thread는 OS 스레드를 직접 사용합니다. 가벼운 그린스레드가 아니라서 스레드 생성/컨텍스트 스위칭 비용이 큼.
- Send/Sync 트레잇:
- Send: 값의 소유권을 스레드 간 이동해도 안전함.
- Sync: &T 참조를 여러 스레드에서 동시에 공유해도 안전함(즉, T: Sync면 &T는 Send).
- 라이프타임 'static: thread::spawn에 전달하는 클로저는 기본적으로 'static을 요구합니다. 
    외부 변수를 캡처할 때는 move로 소유권을 넘기거나, 스코프를 보장하는 다른 방법을 써야 합니다.

## 스레드 스폰과 관리
### 기본 스폰과 조인
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from worker thread!");
        42
    });

    // join은 스레드 종료 대기 + Result<T> 회수
    let result = handle.join().unwrap();
    println!("Worker returned: {result}");
}
```

- thread::spawn: 새 OS 스레드 생성, 핸들 반환.
- join(): 자식 스레드 종료 대기. 패닉 시 Err 반환.

### 이름 지정, 스택 크기 설정
```rust
use std::thread;

fn main() {
    let handle = thread::Builder::new()
        .name("worker-1".to_string())
        // .stack_size(4 * 1024 * 1024)
        .spawn(|| {
            println!("My name: {:?}", thread::current().name());
        })
        .unwrap();

    handle.join().unwrap();
}
```

- thread::Builder: 스레드 이름, 스택 크기 등 설정 가능. 디버깅이 쉬워짐.

### move 클로저와 'static
```rust
use std::thread;

fn main() {
    let data = String::from("hello");

    // move로 소유권을 새 스레드로 이동 → 'static 요구 충족
    let handle = thread::spawn(move || {
        println!("owned: {data}");
    });

    // println!("{data}"); // ❌ 이미 move됨
    handle.join().unwrap();
}
```
- **핵심:** 스폰된 스레드는 부모 스코프보다 오래 살 수 있으므로, 캡처한 값은 스레드 수명 동안 유효해야 합니다. `move`는 이를 보장하는 대표적 방법.

---

# 공유와 변경: Arc, Mutex, RwLock

#### 읽기 전용 공유만 필요할 때: `Arc<T>`
```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1, 2, 3]);
    let a = Arc::clone(&numbers);
    let b = Arc::clone(&numbers);

    let t1 = thread::spawn(move || println!("sum: {}", a.iter().sum::<i32>()));
    let t2 = thread::spawn(move || println!("len: {}", b.len()));

    t1.join().unwrap();
    t2.join().unwrap();
}
```

- 포인트: Arc는 참조 카운트를 원자적으로 관리. “데이터 변경”은 보장하지 않음.
### 변경까지 필요할 때: Arc<Mutex<T>>
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = c.lock().unwrap(); // 락 획득
            *n += 1;                       // 변경
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Result: {}", *counter.lock().unwrap()); // 10
}
```

- Mutex 사용 규칙
- 락 획득 후 사용: lock()은 MutexGuard 반환. 스코프 종료 시 자동 언락.
- 락 범위 최소화: guard를 짧게 유지해 경합/데드락/지연을 줄이기.
- try_lock() 활용: 즉시 실패를 원할 때 비차단 시도.

### 다수 읽기/가끔 쓰기: Arc<RwLock<T>>
- **RwLock**은 동시에 여러 읽기 허용, 쓰기 시 단독 접근. 읽기 많은 워크로드에서 유리.

### 원자적 카운터/플래그: Arc<Atomic*>
```rust
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

fn main() {
    let hits = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let h = Arc::clone(&hits);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                h.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for t in handles { t.join().unwrap(); }
    println!("hits: {}", hits.load(Ordering::Relaxed)); // 4000
}
```

- 장점: 락 오버헤드 없음. 다만 복잡한 불변식 유지엔 적합하지 않을 수 있음.

### 스코프가 있는 스레드와 라이프타임
- 문제: 기본 spawn은 'static을 요구해 지역 참조를 캡처하기 어려움.
- 해법: 스코프가 있는 스레딩으로 부모 스코프가 끝나기 전에 반드시 조인되도록 보장(예: 크레이트의 scoped API).  
표준만 사용한다면 데이터는 move로 넘기거나 Arc로 소유권을 공유하세요.
- 패턴 예시: clone + move
- 읽기 공유: let shared = Arc::clone(&data); thread::spawn(move || { /* read shared */ })
- 쓰기 공유: Arc<Mutex<T>>/Arc<RwLock<T>>를 clone+move로 전달.

### 실전 팁과 흔한 에러
#### 핵심 패턴
- 읽기만: Arc<T>
- 읽기/쓰기: Arc<Mutex<T>> 또는 Arc<RwLock<T>>
- 카운터/플래그: Arc<Atomic*>
- 락 범위 최소화: guard를 빨리 드롭해 교착/경합을 줄이기.
- Send/Sync 확인: 커스텀 타입에 Send/Sync 유도(내부 타입 조합) 여부를 파악.

#### 에러 요약
- E0373 ('static): spawn(|| ...) 클로저가 지역 참조를 빌림 → move로 소유권 이동 또는 수명 보장.
- E0277 (Rc는 Send 아님): Rc<T>를 스레드 간 이동 불가 → Arc<T> 사용.
- E0382 (moved value): move로 값을 넘긴 뒤 다시 사용 → 넘긴 뒤에는 원본 사용 불가.


---

# Arc / Arc<Mutex<_>> / Arc<Atomic_>
👍
아래는 Arc, Arc<Mutex<_>>, Arc<Atomic_> 세 가지를 한 번에 비교할 수 있는 실행 가능한 Rust 예제.
각 케이스별로 멀티스레드 환경에서 어떻게 동작하는지, 그리고 어떤 상황에 적합한지를 한 번에 확인할 수 있습니다.

## 🧩 Arc / Arc<Mutex<_>> / Arc<AtomicUsize> 비교 예제
```rust
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 1. Arc<T> (읽기 전용 공유) ===");
    arc_readonly_example();

    println!("\n=== 2. Arc<Mutex<T>> (읽기/쓰기 공유) ===");
    arc_mutex_example();

    println!("\n=== 3. Arc<AtomicUsize> (원자적 카운터) ===");
    arc_atomic_example();
}
```

### 1) Arc<T> — 읽기 전용 공유
```rust
fn arc_readonly_example() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for _ in 0..3 {
        let shared = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("Sum: {}", shared.iter().sum::<i32>());
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Original data still accessible: {:?}", data);
}
```

### 2) Arc<Mutex<T>> — 읽기/쓰기 공유
```rust
fn arc_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap(); // 락 획득
            *num += 1;
            println!("Thread incremented counter to {}", *num);
            thread::sleep(Duration::from_millis(10));
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Final counter (Mutex): {}", *counter.lock().unwrap());
}
```

### 3) Arc<AtomicUsize> — 원자적 카운터 (락 없이)

```rust
fn arc_atomic_example() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Final counter (Atomic): {}", counter.load(Ordering::Relaxed));
}
```

### 📌 실행 시 예상 출력 (순서는 스레드 스케줄링에 따라 달라짐)

```
=== 1. Arc<T> (읽기 전용 공유) ===
Sum: 15
Sum: 15
Sum: 15
Original data still accessible: [1, 2, 3, 4, 5]

=== 2. Arc<Mutex<T>> (읽기/쓰기 공유) ===
Thread incremented counter to 1
Thread incremented counter to 2
Thread incremented counter to 3
Thread incremented counter to 4
Thread incremented counter to 5
Final counter (Mutex): 5

=== 3. Arc<AtomicUsize> (원자적 카운터) ===
Final counter (Atomic): 50
```


## 🔍 비교 포인트
| 패턴               | 특징                  | 장점                          | 단점                          | 사용 예시 |
|--------------------|-----------------------|-------------------------------|-------------------------------|-----------|
| `Arc<T>`           | 읽기 전용 공유        | 락 오버헤드 없음               | 데이터 변경 불가               | 여러 스레드에서 동일 데이터 읽기 |
| `Arc<Mutex<T>>`    | 읽기/쓰기 공유        | 안전한 변경 가능               | 락 경합 시 성능 저하 가능       | 공유 카운터, 공유 상태 변경 |
| `Arc<AtomicUsize>` | 원자적 단일 값 변경   | 락 없이 빠른 연산 가능         | 복잡한 데이터 구조 변경 불가    | 카운터, 플래그, 통계 값 |
| `Arc<RwLock<T>>`   | 다중 읽기/단일 쓰기   | 읽기 많은 워크로드에 유리      | 쓰기 시 단독 접근 필요          | 캐시, 설정 값 공유 |


💡 이 예제를 실행하면 읽기 전용 공유, 락 기반 변경, 원자적 변경의 차이를 한 번에 확인할 수 있습니다.

---
