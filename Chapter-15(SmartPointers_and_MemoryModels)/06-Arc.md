# Arc overview
멀티스레드 환경에서 동일 데이터를 여러 스레드가 “읽기” 공유해야 할 때 쓰는 스마트 포인터가 Arc입니다.  
Arc는 참조 카운트를 원자적으로 증가/감소해, 여러 스레드가 안전하게 소유권을 공유할 수 있게 합니다.  
다만 Arc 자체는 “가변 접근”을 제공하지 않습니다.  
값 변경이 필요하면 Mutex, RwLock, Atomic 계열과 결합해야 합니다.

## Rc와의 차이와 스레드 안전성
- 역할: Rc와 Arc 모두 참조 카운트 기반 공유 소유권을 제공합니다.
- Rc: 단일 스레드 전용. Send/Sync가 아니므로 다른 스레드로 이동 불가.
- Arc: 멀티스레드 안전. 참조 카운트 연산이 원자적이어서 여러 스레드에서 안전하게 공유 가능.
- 오해 방지: Arc는 “참조 카운트 변경”만 원자적입니다.
- 데이터 변경 보장 아님: Arc는 내부 데이터 변경을 보호하지 않습니다. 변경하려면 Arc<Mutex<T>>, Arc<RwLock<T>>, 또는 Arc<Atomic*>를 사용하세요.
- 수명 규칙:
- 값의 수명: 공유되는 값은 스레드보다 오래 살아야 합니다. Arc는 이 수명을 참조 카운팅으로 보장합니다.

## 샘플 코드와 해설
### 1) Rc를 스레드로 보내려다 실패 (E0277)
```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let rc = Rc::new("hello");
    let t = thread::spawn(move || {
        println!("{}", rc);
    });

    t.join().unwrap();
}
```

- 문제: Rc<T>는 Send가 아님. 클로저가 rc를 다른 스레드로 “move”하려는 순간 컴파일 에러(E0277).
- 요점: 멀티스레드 공유에는 Arc가 필요합니다.

### 2) Arc로 스레드 간 안전 공유 및 strong_count 확인
```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    let c = a.clone();

    let t1 = thread::spawn(move || {
        println!("Reference count: {}", Arc::strong_count(&b));
    });

    let t2 = thread::spawn(move || {
        println!("Reference count: {}", Arc::strong_count(&c));
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Reference count: {}", Arc::strong_count(&a));
}
```

- 관찰 포인트: 각 시점에서 strong_count는 3을 가리킬 수 있으나, 두 스레드의 출력 순서는 스케줄링에 따라 달라집니다(비결정적).
- 요점: Arc의 복제(clone)는 참조 카운트를 원자적으로 증가시키며, 드롭 시 원자적으로 감소합니다.

### 3) Arc만으로 가변 접근을 시도 → 불가
```rust
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn withdraw(balance: &mut i32, amount: i32) {
    if *balance >= amount {
        thread::sleep(Duration::from_millis(10));
        *balance -= amount;
        println!("Withdraw successful. Balance: {}", balance);
    } else {
        println!("Insufficient balance.");
    }
}

fn main() {
    let balance = Arc::new(100);
    let t1 = thread::spawn(move || {
        withdraw(&mut balance, 50); // 가변 소유권 때문에 불가!
    });

    let t2 = thread::spawn(move || {
        withdraw(&mut balance, 75);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
```

- 문제: Arc<i32>는 내부 값에 대한 동시 가변 접근을 보장하지 않습니다. &mut 대여 자체가 성립하지 않으며, 설령 되더라도 데이터 경합 위험.
- 해법: Arc<Mutex<i32>> 또는 Arc<AtomicI32> 등 동기화 원시를 사용해야 합니다.
### 4) 올바른 수정: Arc<Mutex<T>>로 보호
```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn withdraw(balance: &Arc<Mutex<i32>>, amount: i32) {
    // 스코프를 작게 잡아 락 보유 시간 최소화
    let mut guard = balance.lock().unwrap();
    if *guard >= amount {
        thread::sleep(Duration::from_millis(10)); // 경쟁 조건 관찰용 지연
        *guard -= amount;
        println!("Withdraw successful. Balance: {}", *guard);
    } else {
        println!("Insufficient balance. Balance: {}", *guard);
    }
    // guard 드롭 → 락 해제
}

fn main() {
    let balance = Arc::new(Mutex::new(100));

    let b1 = Arc::clone(&balance);
    let t1 = thread::spawn(move || {
        withdraw(&b1, 50);
    });

    let b2 = Arc::clone(&balance);
    let t2 = thread::spawn(move || {
        withdraw(&b2, 75);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final balance: {}", *balance.lock().unwrap());
}
```

- 핵심:
- Arc: 여러 스레드가 같은 계좌를 소유.
- Mutex: 동일 시점에 한 스레드만 변경하도록 보호.
- 락 스코프 최소화: 데드락·성능 저하 예방.

## 핵심 동작과 주의사항
### 참조 카운트 원자성:
- 장점: 스레드 간 안전한 소유권 공유.
- 제한: 데이터 경쟁을 막지 않습니다. 변경엔 Mutex/RwLock/Atomic이 필요.
  
### 성능 고려:
- 원자 연산 비용: Arc는 Rc보다 느립니다(원자 증가/감소 비용).
- 락 비용: Arc<Mutex<T>>는 추가로 락/언락 비용, 잠금 경합이 있습니다.

### API 관례:
- 읽기 전용 공유: Arc<T>
- 가끔 쓰기, 다수 읽기: Arc<RwLock<T>>
- 카운터/플래그: Arc<AtomicUsize> 등 Atomic 계열
- 고성능/세밀 제어: 필요 시 parking_lot 같은 대안 고려 가능

## 언제 무엇을 쓸까
### 단일 스레드, 공유 소유권:
- 선택: Rc<T>
- 이유: 더 저렴하고 간단.

### 멀티스레드, 읽기 전용 공유:
- 선택: Arc<T>
- 이유: 원자적 참조 카운팅만 필요.

### 멀티스레드, 값 변경 필요:
- 선택: Arc<Mutex<T>> 또는 Arc<RwLock<T>> / Arc<Atomic*>
- 이유: 데이터 경쟁을 방지하면서 안전하게 수정.

---
