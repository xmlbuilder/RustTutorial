# core::mem
Rust의 core::mem 모듈은 메모리 조작과 관련된 핵심 함수들을 제공합니다.  
아래는 자주 사용되는 주요 함수들의 요약 표와 각 함수의 사용 예시입니다.  

## 🧠 core::mem 주요 함수 요약
| 함수 이름         | 설명                                                             |
|------------------|------------------------------------------------------------------|
| `size_of::<T>()` | 타입 `T`의 크기를 바이트 단위로 반환                             |
| `align_of::<T>()`| 타입 `T`의 메모리 정렬 크기를 바이트 단위로 반환                 |
| `swap()`         | 두 변수의 값을 서로 교환                                          |
| `replace()`      | 기존 값을 새 값으로 대체하고 이전 값을 반환                      |
| `take()`         | 값을 `T::default()`로 대체하고 기존 값을 반환                    |
| `drop()`         | 값의 소유권을 제거하고 명시적으로 drop 호출                      |
| `forget()`       | 값의 소유권을 제거하지만 drop 호출하지 않음 (메모리 누수 가능)   |
| `discriminant()` | enum의 현재 variant를 식별하는 값 반환                           |
| `transmute()` ⚠  | 타입 간 비트 단위 변환 (매우 위험, 타입 크기 동일해야 함)        |



## 📦 함수별 사용 예시
### 1. size_of::<T>()
```rust
use std::mem;

fn main() {
    println!("Size of u32: {}", mem::size_of::<u32>()); // 출력: 4
}
```

### 2. align_of::<T>()
```rust
use std::mem;

#[repr(align(8))]
struct Aligned;

fn main() {
    println!("Alignment of Aligned: {}", mem::align_of::<Aligned>()); // 출력: 8
}
```

### 3. swap()

```rust
use std::mem;

fn main() {
    let mut a = 1;
    let mut b = 2;
    mem::swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b); // 출력: a = 2, b = 1
}
```

### 4. replace()
```rust
use std::mem;

fn main() {
    let mut x = String::from("old");
    let y = mem::replace(&mut x, String::from("new"));
    println!("x = {}, y = {}", x, y); // 출력: x = new, y = old
}
```

### 5. take()
```rust
use std::mem;

fn main() {
    let mut x = Some(42);
    let y = mem::take(&mut x);
    println!("x = {:?}, y = {:?}", x, y); // 출력: x = None, y = Some(42)
}
```


### 6. drop()
```rust
fn main() {
    let x = String::from("hello");
    std::mem::drop(x); // x는 여기서 drop됨
    // println!("{}", x); // 컴파일 에러: x는 더 이상 사용 불가
}
```

### 7. forget() ⚠
```rust
fn main() {
    let x = Box::new(42);
    std::mem::forget(x); // drop 호출되지 않음 → 메모리 누수 가능
}
```

### 8. discriminant()
```rust
use std::mem;

#[derive(Debug)]
enum MyEnum { A, B }
```
```rust
fn main() {
    let a = MyEnum::A;
    let b = MyEnum::B;
    println!("{:?}", mem::discriminant(&a) == mem::discriminant(&b)); // false
}
```

### 9. transmute() ⚠
```rust
use std::mem;

fn main() {
    let num: u32 = 0x12345678;
    let bytes: [u8; 4] = unsafe { mem::transmute(num) };
    println!("{:?}", bytes); // 출력: [120, 86, 52, 18] (리틀 엔디안 기준)
}
```
- 이 함수들은 대부분 저수준 시스템 프로그래밍, 성능 최적화, 또는 unsafe 코드 작성 시 유용하게 사용됩니다.

std::mem::forget 또는 core::mem::forget는 Rust에서 의도적으로 drop을 생략하고 값을 메모리에 남겨두는 함수입니다.  
위험해 보이지만, 몇 가지 특수한 상황에서 꼭 필요한 기능이에요.

## 🧠 forget의 존재 이유
| 목적 또는 상황             | 설명                                                                 |
|----------------------------|----------------------------------------------------------------------|
| `T`의 `Drop` 생략          | `forget`은 값의 소유권을 제거하지만 `Drop`을 호출하지 않음            |
| 메모리 해제 방지           | 자원을 해제하지 않고 메모리에 남겨두어야 할 때 사용                   |
| FFI 연동                   | 외부 C 코드에 포인터를 넘길 때 Rust가 자원을 해제하면 안 되는 경우     |
| self-referential 구조      | `Pin`, `ManuallyDrop` 등에서 drop 순서를 제어할 필요가 있을 때         |


## ⚠️ 왜 위험한가?
- forget은 **메모리 누수(leak)** 를 유발할 수 있음.
- Box, Vec, File, MutexGuard 등 자원을 해제해야 하는 타입에 사용하면 자원이 해제되지 않음.
- Drop이 호출되지 않기 때문에 RAII 원칙을 깨뜨림.

## ✅ 언제 필요한가?
### 1. 자원 해제를 의도적으로 지연하거나 방지할 때
```rust
let x = Box::new(42);
std::mem::forget(x); // drop 호출 안됨 → 메모리 누수
```

### 2. FFI (Foreign Function Interface)와 상호작용할 때
- C 코드에 포인터를 넘기고 Rust가 해당 자원을 해제하면 안 되는 경우

### 3. self-referential 구조에서 drop 순서가 위험할 때
- 예: Pin, ManuallyDrop, MaybeUninit과 함께 사용

## 🚫 사용 시 주의사항
- 일반적인 Rust 코드에서는 거의 사용하지 않음
- drop()과 반대되는 역할
- forget()을 쓸 바에야 ManuallyDrop<T>를 쓰는 것이 더 안전하고 명시적

---
# std::mem::ManuallyDrop
std::mem::ManuallyDrop<T>는 Rust에서 drop을 수동으로 제어할 수 있도록 설계된 안전한 타입 래퍼입니다.  
mem::forget처럼 drop을 막지만, 훨씬 더 명시적이고 안전한 방식으로 사용됩니다.

## 🧠 ManuallyDrop 개념 요약
| 항목                        | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| Drop 제어                   | `Drop` 호출을 막고 자원을 수동으로 해제할 수 있도록 함               |
| 자동 해제 방지              | `T`는 자동으로 drop되지 않음                                        |
| 안전한 대안                 | `forget`처럼 drop을 막지만 더 안전하고 명시적인 방식                 |
| unsafe drop 필요            | `ManuallyDrop::drop(&mut T)`은 `unsafe` 블록에서만 호출 가능         |
| 사용 예시                   | `T`를 감싸고 필요 시 `ManuallyDrop::drop()`으로 수동 해제             |


## ✅ 기본 사용 예시
```rust
use std::mem::ManuallyDrop;

fn main() {
    let x = ManuallyDrop::new(String::from("hello"));
    println!("{}", *x); // 접근은 가능
    // drop(x); // 자동 drop 안됨
}
```
- x는 String을 감싸지만 자동으로 drop되지 않음
- *x로 내부 값에 접근 가능

## 🔧 수동으로 drop 호출
```rust
use std::mem::{ManuallyDrop, drop};

fn main() {
    let mut x = ManuallyDrop::new(String::from("manual"));
    unsafe {
        ManuallyDrop::drop(&mut x); // 수동 drop
    }
}
```
- drop()은 unsafe 블록에서만 호출 가능
- 이 방식은 자원 해제 시점을 명확히 제어할 수 있음

## 🧪 왜 필요할까?
### 1. FFI (C와 상호작용)
```rust
#[no_mangle]
pub extern "C" fn pass_string(ptr: *mut String) {
    let _ = unsafe { ManuallyDrop::new(Box::from_raw(ptr)) };
    // drop 생략 → C 코드에서 해제할 수 있도록
}
```

### 2. self-referential 구조
```rust
struct SelfRef {
    data: ManuallyDrop<String>,
    ptr: *const u8,
}
```
- data가 먼저 drop되면 ptr이 dangling이 될 수 있음
- drop 순서를 제어하기 위해 ManuallyDrop 사용

## ⚠️ 주의사항
| 항목                        | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| `drop()` 호출 금지          | `ManuallyDrop<T>`는 자동으로 drop되지 않으므로 일반 `drop()` 호출은 무의미 |
| `ManuallyDrop::drop()` 사용 | 자원을 수동으로 해제하려면 `unsafe` 블록에서 `ManuallyDrop::drop()` 호출 필요 |
| 중복 drop 금지              | 같은 자원을 여러 번 drop하면 undefined behavior 발생 가능             |
| 메모리 누수 가능성          | `drop()`을 호출하지 않으면 자원이 해제되지 않음 → 누수 발생             |

## ✅ 비교: ManuallyDrop vs forget
| 항목               | `ManuallyDrop<T>`                                      | `mem::forget(T)`                                      |
|--------------------|--------------------------------------------------------|--------------------------------------------------------|
| 목적               | drop을 막고 수동으로 자원 해제를 제어                  | drop을 막고 값만 메모리에 남김                         |
| 안전성             | 상대적으로 안전 (명시적 제어, 타입 보호)               | 위험함 (drop 불가, 메모리 누수 가능)                   |
| drop 제어 방식     | `ManuallyDrop::drop(&mut T)`으로 수동 해제 (unsafe 필요) | drop 호출 불가, 자동 해제 없음                         |
| 사용 추천          | 고급 메모리 제어, FFI, self-referential 구조            | 거의 사용하지 않음, 특수한 unsafe 상황에서만 사용      |

```rust
use std::mem::ManuallyDrop;

fn main() {
    // String을 ManuallyDrop으로 감싼다
    let mut message = ManuallyDrop::new(String::from("안녕하세요, JungHwan님!"));

    // 내부 값에 접근
    println!("메시지 내용: {}", *message);

    // 수동으로 drop 호출 (unsafe 필요)
    unsafe {
        ManuallyDrop::drop(&mut message);
    }

    // 이후 message는 더 이상 사용하면 안 됨
    // println!("{}", *message); // 컴파일은 되지만 undefined behavior 가능성 있음
}
```

# 소유권 보존

ManuallyDrop<T>는 Rust의 소유권 시스템을 우회해서 drop을 막는 도구이기 때문에, **해제하려면 반드시 해당 변수에 대한 접근 권한(소유권 또는 가변 참조)** 이 있어야 합니다.

## 🧠 왜 저장해 둬야 할까?
| 항목                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| 값의 생존                     | drop을 호출하려면 값이 변수에 저장되어 있어야 접근 가능               |
| `ManuallyDrop::drop(&mut T)` | 수동 해제를 위해 `&mut` 참조가 필요 → 변수로 저장되어야 함           |
| `forget`                      | 값의 소유권을 제거하고 drop을 생략 → 변수 없이도 호출 가능하지만 위험 |
| `forget` vs `ManuallyDrop`    | `forget`은 무조건 버림, `ManuallyDrop`은 나중에 직접 해제할 의도      |


## ✅ 예시: 저장 안 하면 해제 불가
```rust
use std::mem::ManuallyDrop;

fn main() {
    // ❌ 이렇게 하면 drop할 방법이 없음
    ManuallyDrop::new(String::from("hello"));

    // 이 시점에서 접근할 수 있는 변수 없음 → drop 불가 → 메모리 누수
}
```


## ✅ 예시: 변수로 저장하고 수동 해제
```rust
use std::mem::ManuallyDrop;

fn main() {
    let mut msg = ManuallyDrop::new(String::from("hello"));

    // 안전하게 접근 가능
    println!("메시지: {}", *msg);

    // 수동 해제
    unsafe {
        ManuallyDrop::drop(&mut msg);
    }
}
```

## 🔐 요약
- ManuallyDrop은 **내가 나중에 책임지고 해제할게** 라는 약속.
- 그러려면 그 값을 어디엔가 저장해두고, 해제 시점에 접근할 수 있어야 함.
- 그렇지 않으면 메모리 누수가 발생하고, Rust의 안전성 보장이 깨질 수 있음.


## 🧠 왜 싱글톤이 적합할까?
| 항목                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| 생명주기 관리            | 프로그램 종료까지 자원을 유지할 수 있음                              |
| 접근성 확보              | 어디서든 접근 가능 → drop 시점 제어가 쉬움                            |
| 수동 해제 가능           | 종료 시점에 `ManuallyDrop::drop()`을 명시적으로 호출 가능             |
| 안전한 공유              | `Mutex`, `OnceCell`, `Lazy` 등으로 thread-safe하게 초기화 및 접근 가능 |


## ✅ 예시: OnceLock + ManuallyDrop
```rust
use std::mem::ManuallyDrop;
use std::sync::OnceLock;
```
```rust
static GLOBAL: OnceLock<ManuallyDrop<String>> = OnceLock::new();
```
```rust
fn main() {
    // 초기화
    GLOBAL.set(ManuallyDrop::new(String::from("전역 메시지"))).unwrap();

    // 어디서든 접근 가능
    println!("메시지: {}", *GLOBAL.get().unwrap());

    // 종료 시점에 수동 drop
    unsafe {
        let mut_ref = GLOBAL.get().unwrap() as *const _ as *mut ManuallyDrop<String>;
        ManuallyDrop::drop(&mut *mut_ref);
    }
}
```

### 🔐 주의사항
- `ManuallyDrop::drop()` 은 unsafe이므로 반드시 drop 대상이 유효한지 확인하고 호출해야 함
- OnceLock, Lazy, Mutex 등과 함께 쓰면 초기화와 동시성 문제를 안전하게 해결할 수 있음
- 종료 시점에 ShutdownHook이나 Drop 구현체를 활용해 자동 해제도 가능

---

## 🧠 왜 싱글톤에 Drop을 구현해야 할까?
| 항목                         | 설명                                                                 |
|------------------------------|----------------------------------------------------------------------|
| `Drop` 자동 호출             | 프로그램 종료 시점에 `Drop` 트레잇이 자동으로 호출되어 자원 정리 가능     |
| `ManuallyDrop::drop()` 제어 | `Drop` 내부에서 unsafe하게 `ManuallyDrop::drop()`을 호출해 수동 해제 수행 |
| 실수 방지                    | `drop()` 호출을 깜빡해도 `Drop` 구현으로 자원 누수 방지 가능             |
| `Drop` vs `ShutdownHook`     | Rust에서는 `Drop`이 JVM의 `ShutdownHook`처럼 종료 후처리를 담당함        |


## ✅ 예시: 전역 싱글톤 + Drop 구현

```rust
use std::mem::ManuallyDrop;
use std::sync::OnceLock;

struct GlobalResource {
    data: ManuallyDrop<String>,
}
```
```rust
impl Drop for GlobalResource {
    fn drop(&mut self) {
        println!("GlobalResource drop 호출됨");
        unsafe {
            ManuallyDrop::drop(&mut self.data);
        }
    }
}
```
```rust
static GLOBAL: OnceLock<GlobalResource> = OnceLock::new();
```
```rust
fn main() {
    GLOBAL.set(GlobalResource {
        data: ManuallyDrop::new(String::from("전역 자원")),
    }).unwrap();

    println!("자원 내용: {}", *GLOBAL.get().unwrap().data);
    // 프로그램 종료 시 Drop 자동 호출됨
}
```
## 🔐 보너스 팁
- Drop은 Rust의 RAII 원칙을 따르므로, 명시적 종료 로직 없이도 자원 정리가 가능합니다
- ManuallyDrop을 감싼 구조체에 Drop을 구현하면 안전성과 유지보수성이 크게 향상됩니다
- OnceCell, Lazy, Mutex 등과 함께 쓰면 초기화와 동시성 문제도 해결됩니다

---

# 메모리 큰 블록 관리

FFI에서 아주 큰 메모리 블록을 받을 때 ManuallyDrop을 활용한 싱글톤 구조는 안전성과 제어력을 동시에 확보할 수 있는 매우 효과적인 전략입니다.

| 구성 요소              | 역할 설명                                                                 |
|------------------------|---------------------------------------------------------------------------|
| `Box::from_raw(ptr)`   | FFI로 받은 포인터를 Rust의 소유권 있는 타입(Box)으로 변환                  |
| `ManuallyDrop<T>`      | drop을 막고 Rust가 자원을 자동 해제하지 않도록 보호                        |
| 전역 구조 (싱글톤 등)  | 프로그램 종료까지 자원을 유지하며 어디서든 접근 가능                       |
| `Drop` 트레잇 구현     | 종료 시점에 `ManuallyDrop::drop()`을 호출하여 자원 정리 책임 수행           |
| `ManuallyDrop::drop()` | unsafe 블록에서 수동으로 자원을 해제하는 함수                              |


## ✅ 예시: FFI에서 받은 메모리 블록을 안전하게 관리
```rust
use std::mem::ManuallyDrop;
use std::sync::OnceLock;

struct HugeBlock {
    data: ManuallyDrop<Box<[u8]>>,
}
```
```rust
impl Drop for HugeBlock {
    fn drop(&mut self) {
        println!("HugeBlock drop 호출됨");
        unsafe {
            ManuallyDrop::drop(&mut self.data);
        }
    }
}
```
```rust
static GLOBAL_BLOCK: OnceLock<HugeBlock> = OnceLock::new();
```
```rust
#[no_mangle]
pub extern "C" fn receive_block(ptr: *mut u8, len: usize) {
    let boxed = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)) };
    GLOBAL_BLOCK.set(HugeBlock {
        data: ManuallyDrop::new(boxed),
    }).unwrap();
}
```

## 🔐 이 방식의 장점
- Rust가 자원을 자동으로 해제하지 않음 → FFI에서 메모리 해제 충돌 방지
- 전역 구조에 저장 → 프로그램 종료까지 생존 보장
- Drop 트레잇으로 정리 책임 명확화 → 실수 방지, 유지보수 용이
- unsafe 최소화 → drop()만 unsafe로 제한

요약하자면, ManuallyDrop<T>는 Rust에서 자원 해제 시점을 개발자가 직접 제어하고 싶을 때 사용하는 도구입니다.

---






