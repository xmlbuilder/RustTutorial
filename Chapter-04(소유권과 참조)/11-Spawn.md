# 🧵 Rust에서 스레드 생성하기: thread::spawn 완전 정리

## 1️⃣ 스레드란 무엇인가?
- 스레드(Thread): 하나의 프로세스 내에서 동시에 실행되는 독립적인 실행 흐름
- Rust의 스레드 모델: OS 스레드를 직접 사용하며, 안전성과 성능을 모두 고려함
- 스레드 생성: std::thread::spawn 함수로 새로운 스레드 생성

## 2️⃣ 기본 스레드 생성 예제
```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

### 🔍 실행 결과 예시
```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
...
```

### ⚠️ 주의
- 메인 스레드가 먼저 종료되면 생성된 스레드가 중간에 멈출 수 있음
- 스레드 실행 순서는 OS 스케줄러에 따라 비결정적임

## 3️⃣ 스레드 종료 대기: join()
```rust
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 생성된 스레드가 끝날 때까지 기다림
}
```

### ✅ 효과
- join()을 호출하면 메인 스레드가 생성된 스레드의 종료를 기다림
- 모든 출력이 보장되며, 스레드 조기 종료 방지

## 4️⃣ move 클로저와 소유권 이동
### ❌ 잘못된 예제 (에러 발생)
```rust
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v); // ❌ v는 빌림됨
    });
    handle.join().unwrap();
}
```

❗ 에러 코드: E0373
closure may outlive the current function, but it borrows v, which is owned by the current function

- Rust는 스레드가 'static 수명을 요구하기 때문에, 지역 변수의 참조는 허용되지 않음
- 해결 방법: move 키워드로 소유권을 클로저로 이동

### ✅ 올바른 예제 (move 사용)
```rust
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v); // ✅ v의 소유권을 이동
    });
    handle.join().unwrap();
}
```

### ⚠️ 이후 v는 사용 불가
// println!("{:?}", v); // ❌ v는 이미 move됨



## 5️⃣ 요약: thread::spawn 핵심 포인트
| 항목             | 설명 또는 키워드                  |
|------------------|----------------------------------|
| 스레드 생성       | `thread::spawn(|| { ... })`       |
| 실행 순서         | 비결정적 (OS 스케줄러에 따라 다름) |
| 스레드 종료 대기   | `.join().unwrap()`               |
| 데이터 캡처 방식   | `move` 키워드로 소유권 이동       |
| 관련 에러 코드     | `E0373`: 클로저가 지역 변수 참조 시 발생 |



## ✅ 실전 팁
- 데이터를 스레드에 넘길 때는 항상 move 사용
- 스레드가 끝나기 전에 메인 스레드가 종료되지 않도록 join() 호출
- 스레드 간 데이터 공유가 필요하면 Arc, Mutex 등 동기화 원시 사용

---

# Arc<Mutex<T>>

Arc<Mutex<T>>를 사용하여 여러 스레드가 하나의 데이터를 안전하게 공유하고 수정하는 Rust 예제.
이 코드는 앞서 설명한 thread::spawn, move, 그리고 Mutex의 자동 언락 기능까지 모두 활용합니다.

## 🧪 예제: Arc<Mutex<i32>>로 공유 카운터 증가시키기
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 공유할 카운터를 Arc<Mutex<i32>>로 감싸기
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 10개의 스레드 생성
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 참조 카운트 증가
        let handle = thread::spawn(move || {
            // 락 획득 → MutexGuard 반환
            let mut num = counter.lock().unwrap();
            *num += 1; // 카운터 증가
            // MutexGuard drop → 자동 언락
        });
        handles.push(handle);
    }

    // 모든 스레드 종료 대기
    for handle in handles {
        handle.join().unwrap();
    }

    // 최종 결과 출력
    println!("Final counter value: {}", *counter.lock().unwrap()); // 10
}
```


## 🔍 코드 설명
| 구성 요소                | 설명 |
|--------------------------|------|
| `Arc<T>`                 | 여러 스레드가 데이터를 안전하게 공유할 수 있도록 참조 카운트를 관리하는 스마트 포인터 |
| `Mutex<T>`               | 하나의 스레드만 데이터에 접근할 수 있도록 보호하는 락 구조 |
| `Arc::clone`             | 참조 카운트를 증가시켜 `Arc`를 다른 스레드로 안전하게 전달 |
| `thread::spawn(move || ...)` | 클로저에 소유권을 이동시켜 스레드에서 안전하게 데이터 사용 가능 |
| `lock().unwrap()`        | `Mutex`의 락을 획득하고 내부 데이터에 접근 (실패 시 panic 발생 가능) |
| `MutexGuard`             | `lock()` 호출 시 반환되는 스마트 포인터로, 스코프 종료 시 자동으로 락 해제됨 |



## ⚠️ 주의사항
- Arc<Mutex<T>>는 읽기/쓰기 공유에 적합하지만, 락 경합이 발생할 수 있으므로 락 범위를 최소화하는 것이 좋습니다.
- unwrap()은 panic 발생 시 프로그램 종료 가능성이 있으므로, 실전에서는 expect() 또는 match로 처리하는 것이 안전합니다.
- 이 구조는 데이터 경쟁 없이 안전하게 공유 상태를 변경할 수 있게 해줍니다.

---

# Arc<RwLock<T>> / Arc<AtomicUsize>

아래는 앞서 보신 Arc<Mutex<T>> 예제를 확장하여, **Arc<RwLock<T>>** 와 **Arc<AtomicUsize>** 를 사용하는 버전입니다.
각 방식은 멀티스레드 환경에서 데이터를 공유하고 수정하는 목적은 같지만, 성능과 동작 방식에서 차이가 있습니다.

## 🧪 예제 1: Arc<RwLock<i32>> — 다중 읽기, 단일 쓰기

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 쓰기 스레드
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.write().unwrap(); // 쓰기 락
            *num += 1;
            println!("Writer updated counter to {}", *num);
        }));
    }

    // 읽기 스레드
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let num = c.read().unwrap(); // 읽기 락
            println!("Reader sees counter as {}", *num);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter (RwLock): {}", *counter.read().unwrap());
}
```

### ✅ 특징
- 여러 스레드가 동시에 읽기 가능
- 쓰기 시에는 단독 접근 필요
- 읽기 많은 워크로드에 적합

## 🧪 예제 2: Arc<AtomicUsize> — 락 없는 원자적 카운터
```rust
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                c.fetch_add(1, Ordering::Relaxed); // 원자적 증가
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter (AtomicUsize): {}", counter.load(Ordering::Relaxed));
}
```

### ✅ 특징
- 락 없이 빠른 연산
- 단일 값의 단순한 변경에 적합
- 복잡한 불변식이나 구조체에는 부적합

## 🔍 비교 요약
| 방식               | 읽기 병렬성 | 쓰기 병렬성 | 락 사용 여부 | 성능       | 적합한 용도             |
|--------------------|--------------|--------------|---------------|------------|--------------------------|
| `Arc<Mutex<T>>`    | ❌            | ❌            | 있음          | 중간       | 단순한 공유 및 변경       |
| `Arc<RwLock<T>>`   | ✅            | ❌            | 있음          | 읽기 많을 때 우수 | 캐시, 설정값, 읽기 중심 데이터 |
| `Arc<AtomicUsize>` | ✅            | ✅ (단일 값)  | 없음          | 매우 빠름  | 카운터, 플래그, 통계 값     |

---


