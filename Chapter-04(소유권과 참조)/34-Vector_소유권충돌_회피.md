# Vector에 소유권 적용 여부

지금까지 살펴본 split_at_mut(), chunks_mut(), windows_mut() 같은 기법들은
array뿐 아니라 Vec<T>에도 그대로 적용 가능합니다.
왜냐하면 Vec<T>는 내부적으로 &mut [T]로 변환이 가능하기 때문.

## ✅ Vector에서도 그대로 적용 가능한 기법들

### 1. split_at_mut() on Vec
```rust
fn vec_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = v.split_at_mut(3);

    left[0] += 10;
    right[2] += 20;

    println!("Result: {:?}", v); // [11, 2, 3, 4, 5, 26]
}
```


### 2. chunks_mut() on Vec
```rust
fn vec_chunks_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    for chunk in v.chunks_mut(2) {
        chunk[0] *= 2;
    }

    println!("Result: {:?}", v); // [2, 2, 6, 4, 10, 6]
}
```
#### ✅ 실전 예시
```rust
for chunk in v.chunks_mut(2) {
    chunk[0] *= 2;
    if let Some(x) = chunk.get_mut(1) {
        *x *= 10;
    }
}
```
- get_mut(1)은 안전하게 접근 가능
- panic 없이 처리됨

### 3. windows_mut() on Vec
```rust
fn vec_windows_mut() {
    let mut v = vec![1, 2, 3, 4, 5];

    for window in v.windows_mut(2) {
        window[0] += window[1];
    }

    println!("Result: {:?}", v); // [3, 5, 7, 9, 5]
}
```


### 4. scoped 접근 on Vec
```rust
fn vec_scoped_access() {
    let mut v = vec![1, 2, 3];

    {
        let a = &mut v[0];
        *a += 10;
    }

    {
        let b = &mut v[2];
        *b += 20;
    }

    println!("Result: {:?}", v); // [11, 2, 23]
}
```


### 5. RefCell<Vec<T>> — 내부 가변성
```rust
use std::cell::RefCell;

fn vec_refcell() {
    let v = RefCell::new(vec![1, 2, 3]);

    {
        let mut borrow = v.borrow_mut();
        borrow[1] += 100;
    }

    println!("Result: {:?}", v.borrow()); // [1, 102, 3]
}
```


### 6. raw pointer on Vec
```rust
fn vec_raw_pointer() {
    let mut v = vec![1, 2, 3];
    let p1 = &mut v[0] as *mut i32;
    let p2 = &mut v[2] as *mut i32;

    unsafe {
        *p1 += 100;
        *p2 += 200;
    }

    println!("Result: {:?}", v); // [101, 2, 203]
}
```


### ❌ 예외: split_array_mut()는 배열 전용
- split_array_mut()는 [T; N] 고정 크기 배열에만 적용 가능
- Vec<T>는 크기가 동적이므로 이 기법은 사용할 수 없음

## 💬 결론
Vec<T>는 내부적으로 &mut [T]로 변환되기 때문에 대부분의 slice 기반 기법을 그대로 사용할 수 있음.  
단, split_array_mut()처럼 고정 크기 배열 전용 기법은 예외입니다.

---
# 병렬 처리 고려

RefCell<T>는 런타임 borrow 체크를 기반으로 하기 때문에 병렬 처리에서는 절대 사용할 수 없습니다.  
병렬 환경에서는 스레드 안전성과 동시 접근 제어가 핵심이기 때문에 다음과 같은 방식으로 접근해야 합니다.

## ✅ 병렬 환경에서 RefCell 대신 써야 하는 것들
### 1. Mutex<T> — 스레드 간 가변 접근 제어
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn mutex_example() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec[i] += 10;
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {:?}", data.lock().unwrap()); // [11, 12, 13]
}
```

- Arc<Mutex<T>>로 공유 + 가변 접근
- 병렬 환경에서 안전하게 Vec<T> 수정 가능

### 2. RwLock<T> — 읽기/쓰기 분리
```rust
use std::sync::{Arc, RwLock};

fn rwlock_example() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    {
        let read = data.read().unwrap();
        println!("Read: {:?}", *read);
    }
    {
        let mut write = data.write().unwrap();
        write[0] += 100;
    }
    println!("Final: {:?}", data.read().unwrap()); // [101, 2, 3]
}
```

- 여러 스레드가 동시에 읽고, 하나만 쓸 수 있음
- 병렬 Matrix 연산에서 읽기 병렬화에 유리

### 3. Atomic 타입 — 원자적 연산
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

fn atomic_example() {
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        std::thread::spawn(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Count: {}", counter.load(Ordering::SeqCst)); // 10
}
```

- AtomicUsize, AtomicBool, AtomicPtr 등
- 병렬 연산에서 경량 동기화에 적합

### 4. Rayon + 안전한 slice 분할
```rust
use rayon::prelude::*;
fn rayon_chunks_example() {
    let mut data = vec![1, 2, 3, 4, 5, 6];

    data.par_chunks_mut(2).for_each(|chunk| {
        chunk[0] *= 2;
    });
    println!("Result: {:?}", data); // [2, 2, 6, 4, 10, 6]
}
```

- par_chunks_mut()는 내부적으로 안전하게 slice를 분할
- RefCell 없이도 병렬 가변 접근 가능

### ❌ RefCell이 병렬에서 안 되는 이유
| 원인                  | 설명                                               |
|-----------------------|----------------------------------------------------|
| `RefCell<T>`는 `!Sync`| 스레드 간 공유 불가능. 컴파일 타임에 막힘         |
| 런타임 borrow 체크     | 병렬 접근 시 중복 borrow 발생 → panic 가능         |
| 내부 가변성은 단일 스레드용 | 병렬 연산에는 `Mutex`, `RwLock`, `Atomic`이 필요 |


## 💬 결론
병렬 환경에서는 RefCell은 절대 금물입니다.  
대신 Arc<Mutex<T>>, Arc<RwLock<T>>, Atomic 타입,  
그리고 Rayon의 안전한 병렬 slice 분할을 활용하면  
Rust의 안전성과 병렬 성능을 동시에 확보할 수 있음.  

--- 

# 스코프 오버 헤드 분석

## 🔍 이 코드에서의 오버헤드 분석
```rust
{
    let mut borrow = data.borrow_mut();
    borrow[0] += 1;
}
{
    let borrow = data.borrow();
    println!("{}", borrow[0]);
}
```

- {} 블록은 컴파일 타임에 정적 분석되므로 런타임 오버헤드는 거의 없음  
- RefCell::borrow_mut() / borrow()는 런타임 체크 비용이 있음  
- 내부적으로 usize 카운터로 불변/가변 대여 상태를 추적  
- panic!이 발생할 수 있는 동적 대여 검사가 들어감  

## 🧠 다른 우회 기법과 비교
| 기법               | 런타임 오버헤드     | 안전성       | 특징 및 설명                                      |
|--------------------|----------------------|--------------|--------------------------------------------------|
| `{}` 스코프 분리   | 매우 낮음 🔹          | 안전함 ✅     | Drop 시점 명확, 컴파일 타임에 소멸자 호출됨       |
| `drop(borrow)`     | 매우 낮음 🔹          | 안전함 ✅     | 스코프 없이도 대여 해제 가능, 명시적 제어         |
| `unsafe` 접근      | 없음 ⚠️ (위험함)      | 위험함 ❌     | RefCell 없이 raw pointer 직접 접근, UB 가능성 있음 |
| `Cell<T>`          | 낮음 🔸               | 안전함 ✅     | Copy 타입만 가능, 내부 mutability 제공            |
| `Rc<RefCell<T>>`   | 중간 🔸               | 안전함 ✅     | 참조 카운팅 + 런타임 대여 검사 포함               |
| `Mutex<T>`         | 높음 🔺               | 스레드 안전 ✅| lock/unlock 비용 있음, 멀티스레드 환경 대응        |


---


# chunks_mut

chunks_mut(2)는 slice를 겹치지 않는 2개씩의 블록으로 나누는 방식입니다.  
그렇기 때문에 전체를 2개씩 분리해서 접근한다는 의미 

## ✅ 예시로 설명
```rust
let mut data = [1, 2, 3, 4, 5, 6];
for chunk in data.chunks_mut(2) {
    println!("{:?}", chunk);
}
```

### 출력 결과:
```
[1, 2]
[3, 4]
[5, 6]
```

- chunks_mut(2)는 [1,2], [3,4], [5,6] 이렇게 겹치지 않는 3개의 블록을 반환
- 각 chunk는 &mut [T] 타입이고, 동시에 접근해도 안전함

### 🔍 핵심 포인트 — `chunks_mut(n)`
| 개념            | 반환 타입     | 설명 및 예시                                      |
|-----------------|---------------|---------------------------------------------------|
| `chunks_mut(n)` | `&mut [T]`    | slice를 겹치지 않는 `n`개씩의 가변 블록으로 나눔 |
| 예시            |               | `[1,2,3,4,5].chunks_mut(2)` → `[1,2]`, `[3,4]`, `[5]` |

## 💬 결론

chunks_mut(2)는 2개씩 끊어서 겹치지 않는 블록을 만들기 때문에  
전체 길이가 6이면 3개의 chunk에 안전하게 접근할 수 있다는 뜻.

---

# Automic

## 🔍 주요 Atomic 타입들

| Atomic 타입                          | 내부 값 타입       |
|-------------------------------------|--------------------|
| AtomicUsize                         | usize              |
| AtomicIsize                         | isize              |
| AtomicBool                          | bool               |
| AtomicI32 / AtomicU32               | i32 / u32          |
| AtomicI64 / AtomicU64               | i64 / u64          |
| AtomicPtr<T>                        | *mut T (raw ptr)   |


### ✅ 사용 예시
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);

// 증가
counter.fetch_add(1, Ordering::SeqCst);

// 읽기
let value = counter.load(Ordering::SeqCst);
```

- Ordering::SeqCst는 가장 강력한 메모리 순서 보장
- fetch_add, fetch_sub, store, load, compare_exchange 등 다양한 메서드 제공

## 💡 실전에서 선택 기준
| 상황                     | 추천 Atomic 타입       |
|--------------------------|------------------------|
| 스레드 안전 카운터        | AtomicUsize            |
| 상태 플래그 (on/off)      | AtomicBool             |
| 포인터 교체, 동적 참조    | AtomicPtr<T>           |
| 고정 크기 정수 연산       | AtomicI32 / AtomicU64  |


## 🔍 그럼 float을 atomic하게 다루고 싶을 땐?
### ✅ 1. AtomicU32 / AtomicU64로 float을 비트로 변환해서 처리
```rust
use std::sync::atomic::{AtomicU32, Ordering};

let f = 1.23f32;
let bits = f.to_bits(); // f32 → u32
let atomic = AtomicU32::new(bits);

// 읽기
let loaded_bits = atomic.load(Ordering::SeqCst);
let loaded_f = f32::from_bits(loaded_bits); // u32 → f32

// 쓰기
let new_f = 4.56f32;
atomic.store(new_f.to_bits(), Ordering::SeqCst);
```

- 핵심 아이디어: float을 비트로 바꿔서 AtomicU32나 AtomicU64로 다룸
- 주의점: 산술 연산 (+, -, *)은 직접 못함 → 값을 꺼내서 연산 후 다시 저장해야 함

### ✅ 2. parking_lot::Mutex<f32> 또는 RwLock<f32> 사용
- 락 기반으로 float을 안전하게 공유
- 성능이 중요하지 않거나 연산이 복잡할 때 적합

### ✅ 3. atomic_float 크레이트 사용 (외부 라이브러리)
```
# Cargo.toml
atomic_float = "1.0"
```

```rust
use atomic_float::AtomicF32;

let atomic = AtomicF32::new(1.23);
let val = atomic.load(Ordering::SeqCst);
atomic.store(4.56, Ordering::SeqCst);
```

- AtomicF32, AtomicF64을 직접 제공
- 내부적으로 to_bits() 방식 사용

## 💡 float을 atomic하게 다루는 방법
| 방법                  | 장점                         | 단점                          |
|-----------------------|------------------------------|-------------------------------|
| AtomicU32 + to_bits() | 표준 라이브러리로 가능       | 산술 연산 직접 구현 필요      |
| Mutex<f32>            | 간단하고 직관적              | 락 비용 발생                  |
| atomic_float 크레이트 | float 전용 atomic 타입 제공  | 외부 라이브러리 의존          |

---

