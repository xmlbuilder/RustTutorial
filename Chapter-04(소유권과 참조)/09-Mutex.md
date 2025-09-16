# Mutex
Mutex / Arc / Rc / 멀티스레드 환경에서의 사용법과 주의점을 정리.
실제 코드 예제와 함께, 각 단계에서 왜 에러가 발생하는지, 그리고 어떻게 해결하는지를 한눈에 볼 수 있도록 구성.

## 1. Mutex 기본 개념
- Mutex = Mutual Exclusion (상호 배제)
- 한 시점에 오직 하나의 스레드만 데이터에 접근 가능
- 데이터 접근 전 lock() 호출 → 락 획득
- 데이터 사용 후 락 해제 필요
→ Rust에서는 MutexGuard가 Drop될 때 자동 해제

```rust
use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // 락 획득
        *num = 6; // 데이터 수정
    } // MutexGuard drop → 락 자동 해제

    println!("m = {:?}", m); // Mutex { data: 6, poisoned: false, .. }
    println!("m = {}", m.lock().unwrap()); // 6
}
```

핵심 포인트
- lock() → MutexGuard 반환 (스마트 포인터)
- Deref 구현 → 내부 데이터 접근 가능
- 스코프 종료 시 Drop → 락 자동 해제

## 2. Mutex를 멀티스레드에서 사용하기
### ❌ 잘못된 예제 1 — 단일 Mutex를 move 없이 사용
```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut num = counter.lock().unwrap(); // ❌ counter를 클로저에서 캡처 불가
            *num += 1;
        });
        handles.push(handle);
    }
}
```

#### 에러 요약 (E0373)
closure may outlive the current function, but it borrows counter →
스레드가 'static 수명을 요구하는데, counter는 지역 변수라 빌림 불가


### ❌ 잘못된 예제 2 — move 사용했지만 Mutex 단독
```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // ❌ 첫 스레드로 counter 소유권 이동
            *num += 1;
        });
        handles.push(handle);
    }
}
```

#### 에러 요약 (E0382)
use of moved value: counter →
첫 번째 스레드로 counter가 이동해버려, 다음 반복에서 사용할 수 없음


### 3. Rc<T>로 참조 카운트 시도
```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
}
```

#### 에러 요약 (E0277)
Rc<Mutex<i32>> cannot be sent between threads safely →
Rc는 Send/Sync가 아니므로 스레드 간 이동 불가


#### 4. Arc<T>로 해결
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // 10
}
```

#### 설명
- Arc = Atomic Reference Counted → 멀티스레드 안전한 참조 카운트
- Arc<Mutex<T>> → 여러 스레드가 안전하게 공유 + 변경 가능

### 5. 이미지 속 에러 메시지 요약과 코드 매핑
| 에러 코드 | 원인/키워드       | 예시 코드 |
|-----------|------------------|-----------|
| E0373     | `'static` 수명 문제 | `thread::spawn(|| { counter.lock()... })` |
| E0382     | move 후 재사용     | `thread::spawn(move || { ... }); println!("{:?}", counter);` |
| E0277     | `Rc<T>`는 `Send`/`Sync` 미구현 | `Rc<Mutex<i32>>` |


### 6. 핵심 정리
- Mutex: 한 번에 하나의 스레드만 데이터 접근 가능
- MutexGuard: 락 자동 해제 보장
- Rc<T>: 단일 스레드 전용 참조 카운트
- Arc<T>: 멀티스레드 안전 참조 카운트
- Arc<Mutex<T>>: 멀티스레드에서 안전하게 공유 + 변경

---

