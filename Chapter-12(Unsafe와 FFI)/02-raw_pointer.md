# raw pointer
Rust의 raw pointer는 일반적인 참조(&T, &mut T)와는 달리 메모리 안전성을 보장하지 않는 포인터.  
C나 C++의 포인터와 비슷한 개념.  
Rust는 기본적으로 안전한 언어지만, 시스템 프로그래밍이나 FFI(C와의 상호작용) 같은 저수준 작업을 위해 raw pointer를 제공.

## 🧩 Raw Pointer란?
Rust에서 raw pointer는 두 가지 타입:
- *const T → 불변(raw) 포인터: 데이터를 읽을 수 있지만 수정은 불가능
- *mut T → 가변(raw) 포인터: 데이터를 읽고 수정 가능
이 포인터들은 소유권, 수명, 동시성 검사에서 자유롭지만, 그만큼 위험성도 높음. 
그래서 raw pointer를 사용할 때는 반드시 unsafe 블록 안에서만 접근.

## 🛠️ 생성 방법
```rust
fn main() {
    let x = 42;
    let r1: *const i32 = &x;       // 불변 raw pointer
    let mut y = 99;
    let r2: *mut i32 = &mut y;     // 가변 raw pointer
}
```

또는 Box<T>를 raw pointer로 변환할 수도 있어:
```rust
let boxed = Box::new(123);
let raw: *mut i32 = Box::into_raw(boxed); // 소유권을 넘기고 raw pointer로 변환
```

### ⚠️ 사용 시 주의점
- **역참조(dereference)**는 unsafe 블록에서만 가능:
```rust    
unsafe {
    println!("r1 points to: {}", *r1);
    *r2 = 100;
}
```

- null pointer도 만들 수 있어:
```
let null_ptr: *const i32 = std::ptr::null();
```

- 포인터 연산도 가능하지만, 역시 unsafe:
```
unsafe {
    let offset_ptr = r1.offset(1); // 다음 메모리 위치
}
```

## 📦 언제 사용하나?
| 상황                         | 설명                                                         |
|------------------------------|--------------------------------------------------------------|
| C 라이브러리와 연동할 때     | FFI에서 C API는 raw pointer를 사용하므로 직접 포인터를 다뤄야 함 |
| 성능 최적화가 필요한 경우    | 안전성보다 속도가 중요한 경우, 제약 없는 포인터 접근이 유리함   |
| 메모리 레이아웃을 직접 제어할 때 | `#[repr(C)]`를 사용해 구조체의 메모리 배치를 C와 호환되게 설정함 |
| 고급 시스템 프로그래밍       | OS 커널, 드라이버, 게임 엔진 등에서 low-level 메모리 접근이 필요함 |
| `unsafe` 코드에서 제어가 필요할 때 | 안전한 Rust로는 표현하기 어려운 로직을 직접 구현할 때 사용됨     |

## 🧠 Raw Pointer vs Reference
| 항목               | 참조 (`&T`, `&mut T`)         | Raw Pointer (`*const T`, `*mut T`)         |
|--------------------|-------------------------------|--------------------------------------------|
| 안전성             | 컴파일러가 메모리 안전성 보장 | `unsafe` 블록에서만 사용 가능, 안전성 직접 보장 필요 |
| null 가능 여부      | `None`으로 표현 (옵션 타입)   | `std::ptr::null()` 또는 null 값 직접 사용 가능 |
| 포인터 연산         | 불가능                        | `offset`, `add` 등 포인터 연산 가능 (`unsafe` 필요) |
| 사용 용도           | 일반적인 안전한 코드          | FFI, 시스템 프로그래밍, 고성능 제어 등 저수준 작업 |
| 역참조              | 안전하게 `*ref` 가능          | `unsafe`에서만 `*ptr`로 역참조 가능           |

## 🔐 요약
Rust의 raw pointer는 강력하지만 위험한 도구.  
안전한 Rust에서는 거의 사용하지 않지만, 시스템 프로그래밍이나 FFI에서는 필수적.  
사용할 때는 항상 unsafe 블록을 통해 명시적으로 위험을 감수해야 함.


Rust는 기본적으로 메모리 안전성을 철저히 보장하지만, unsafe 블록 안에서는 C처럼 직접 메모리를 건드릴 수 있고,  그만큼 죽을 수도 있음. 
즉, **"Rust도 죽는다"**는 말이 틀리지 않음 — 단, 그건 개발자가 unsafe를 통해 안전장치를 해제할 경우.

## 🔥 왜 죽을 수 있을까?
Rust의 unsafe는 이렇게 선언:
“컴파일러야, 이건 내가 책임질게. 너는 검사하지 마.”

이 말은 곧:
- null 포인터 역참조
- 유효하지 않은 주소 접근
- 정렬되지 않은 메모리 접근
- 데이터 레이스
- 미정의 동작(UB, Undefined Behavior)
이런 것들이 발생할 수 있다.  
실제로 unsafe에서 잘못된 포인터를 역참조하면 **세그멘테이션 폴트(segfault)**가 나서 프로그램이 바로 크래시돼.
예시:
```rust
fn main() {
    let ptr = 0xDEADBEEF as *const i32;
    unsafe {
        // 이건 실제로 죽을 수 있음
        println!("Value: {}", *ptr);
    }
}
```
이 코드는 임의의 주소를 역참조하는데, 해당 주소에 접근 권한이 없으면 프로그램은 바로 종료.

## 🧠 Rust의 철학
Rust는 기본적으로 **"안전한 코드에서는 절대 죽지 않는다"**는 철학을 가지고 있음.  
하지만 시스템 프로그래밍 언어로서 저수준 제어가 필요할 때는 unsafe를 통해 그 책임을 개발자에게 넘겨줌.

그래서 Rust는 이렇게 구분돼:
- ✅ Safe Rust: 컴파일러가 모든 메모리 오류를 방지
- ⚠️ Unsafe Rust: 개발자가 직접 메모리를 제어, 실수하면 죽음

## 🛡️ 안전하게 쓰는 법
- unsafe 블록은 최소한으로 좁게 사용
- 포인터 유효성, 정렬, null 여부를 직접 확인
- 가능하면 unsafe 코드를 안전한 추상화로 감싸기
예를 들어, 표준 라이브러리도 내부적으로는 unsafe를 쓰지만, 외부에는 안전한 API만 제공.

요약하자면:
Rust도 죽는다. 하지만 죽을 수 있는 문은 개발자가 직접 열어야 한다.

---

# unsafe
unsafe를 쓴다고 해서 Rust가 막아주는 건 아니고, 오히려 막아주던 안전장치를 개발자가 직접 해제하는 것.
## 🔓 unsafe는 보호가 아니라 “책임 전가”
Rust는 기본적으로 메모리 안전성, 데이터 경쟁 방지, null 참조 금지 같은 강력한 보호 장치를 제공. 
즉, unsafe는 보호를 강화하는 게 아니라, 보호를 해제하고 직접 제어할 수 있게 해주는 도구.

## 🔧 unsafe로 할 수 있는 일들
Rust에서 unsafe를 사용하면 다음과 같은 작업이 가능해져:
- Raw pointer (*const T, *mut T) 역참조
- unsafe fn 호출 (예: FFI 함수)
- static mut 변수 접근
- union 필드 접근
- unsafe trait 구현
이런 작업들은 Rust의 안전성 모델로는 보장할 수 없기 때문에, unsafe 블록 안에서만 허용.  

Rust는 기본적으로 안전한 메모리 접근만 허용. 
즉, &T, &mut T, Box<T>, Vec<T> 같은 스마트한 추상화를 통해 메모리를 다룰 수 있어.  
이 방식은 컴파일러가 소유권, 수명, 동시성, 경계 검사를 철저히 체크해주기 때문에 unsafe 없이도 대부분의 작업이 가능.

## ✅ unsafe 없이 가능한 메모리 접근
- 스택 변수 접근: let x = 10; println!("{}", x);
- 참조자 사용: let r = &x; println!("{}", *r);
- Box, Vec, String 등 힙 할당: 자동으로 메모리 관리됨
- 슬라이스 접근: let arr = [1, 2, 3]; println!("{}", arr[1]); → 경계 검사 있음
- 스마트 포인터 사용: Rc, Arc, RefCell, Mutex 등
이 모든 건 컴파일러가 안전성을 보장해줘서 unsafe 없이도 마음 놓고 쓸 수 있어.

### ❌ unsafe 없이 불가능한 메모리 접근
컴파일러가 메모리 안전성을 보장할 수 없기 때문에, 다음과 같은 작업은 반드시 `unsafe`가 필요:
| 작업 예시                  | 설명                                                         |
|----------------------------|--------------------------------------------------------------|
| `*ptr` (Raw pointer 역참조) | 포인터가 유효한지 컴파일러가 확인할 수 없기 때문에 위험함     |
| `alloc` / `dealloc`        | 수동 메모리 할당 및 해제는 Rust의 안전한 메모리 모델을 벗어남 |
| 슬라이스 분할 (중첩 참조)   | 동일한 슬라이스를 동시에 `&mut`로 나누는 경우 빌림 규칙 위반  |
| FFI 함수 호출              | 외부 함수의 안전성을 Rust가 보장할 수 없으므로 `unsafe` 필요 |
| `static mut` 전역 변수     | 데이터 레이스 발생 가능성이 있어 반드시 `unsafe`로 접근해야 함 |

---


## ✅ unsafe 없이 가능한 고급 메모리 조작 예제
Rust는 안전한 추상화를 통해 많은 저수준 메모리 작업을 unsafe 없이도 할 수 있음. 
아래는 그런 예제들:
### 1. Vec을 사용한 동적 메모리 관리
```rust
fn main() {
    let mut buffer = Vec::with_capacity(1024);
    for i in 0..1024 {
        buffer.push(i);
    }
    println!("Last item: {}", buffer[1023]); // 경계 검사 포함
}
```

- 힙에 메모리를 할당하고 자동으로 관리됨
- 경계 검사 덕분에 안전하게 접근 가능


### 2. Box<T>로 힙에 구조체 저장
```rust
struct Data {
    value: i32,
}

fn main() {
    let boxed = Box::new(Data { value: 42 });
    println!("Value: {}", boxed.value);
}
```

- Box는 힙에 데이터를 저장하면서도 안전한 참조를 제공
### 3. slice.split_at_mut()으로 안전한 분할
```rust
fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let (left, right) = data.split_at_mut(2);
    left[0] = 10;
    right[0] = 20;
    println!("{:?}", data); // [10, 2, 20, 4, 5]
}
```

- 내부적으로 unsafe를 사용하지만, API는 안전하게 제공됨

## ❗ unsafe가 반드시 필요한 상황들
Rust가 안전성을 보장할 수 없는 경우에는 unsafe가 필요해. 아래는 대표적인 예시들이야:
### 1. Raw Pointer 역참조
```rust
let x = 42;
let ptr = &x as *const i32;
unsafe {
    println!("Value: {}", *ptr); // unsafe 없이 불가능
}
```

### 2. FFI (C 함수 호출)
```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-5) = {}", abs(-5));
    }
}
```

- 외부 함수는 Rust의 안전성 모델을 따르지 않기 때문에 unsafe 필요


### 3. 메모리 직접 할당 및 해제
```rust
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::from_size_align(4, 4).unwrap();
    unsafe {
        let ptr = alloc(layout);
        *(ptr as *mut i32) = 123;
        println!("Value: {}", *(ptr as *mut i32));
        dealloc(ptr, layout);
    }
}
```


- 수동 메모리 관리는 반드시 unsafe로 감싸야 함
### 4. 전역 가변 변수 (static mut)
```rust
static mut COUNTER: i32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;
    }
}
```

- 데이터 레이스 가능성이 있기 때문에 unsafe 필요

### 5. 복잡한 자료구조 구현 (예: 링 버퍼, 슬라이스 분할)
```rust
fn split_at_unchecked<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```
- Rust의 빌림 검사기를 우회해야 할 때 unsafe가 필요

## 🧠 요약: Safe vs Unsafe 메모리 조작
| 작업 예시                        | unsafe 없이 가능 | unsafe 필요 | 설명 |
|----------------------------------|------------------|-------------|------|
| `Vec`, `Box`, `slice` 사용       | ✅               | ❌          | 안전한 힙 메모리 관리 및 경계 검사 포함 |
| Raw pointer 역참조 (`*ptr`)      | ❌               | ✅          | 포인터 유효성 확인 불가, 직접 역참조 필요 |
| FFI (C 함수 호출)                | ❌               | ✅          | 외부 함수는 Rust의 안전성 모델을 따르지 않음 |
| 직접 메모리 할당/해제 (`alloc`)  | ❌               | ✅          | 수동 메모리 관리는 안전성 보장 불가 |
| 전역 가변 변수 (`static mut`)    | ❌               | ✅          | 데이터 레이스 가능성 있음 |
| 고급 자료구조 구현 (링 버퍼 등)  | ❌               | ✅          | 빌림 검사기를 우회해야 하는 경우 발생 |


Rust는 기본적으로 안전한 추상화를 통해 많은 저수준 작업을 감싸지만, 진짜 시스템 수준의 제어가 필요할 땐 unsafe가 필수.  
하지만 그 unsafe도 작게, 명확하게, 안전하게 감싸는 것이 Rust의 미덕.

---

# C#과 Rust 비교
Rust의 unsafe와 C#의 unsafe는 이름은 같지만 철학, 사용 목적, 안전성 모델이 완전히 다름.

## 🧠 핵심 차이: Rust vs C#의 `unsafe`

| 항목               | Rust `unsafe`                                  | C# `unsafe`                                     |
|--------------------|------------------------------------------------|-------------------------------------------------|
| 철학               | 안전한 언어에서 제한적으로 위험 허용          | 안전한 언어에 C 스타일 포인터 기능 추가         |
| 기본 안전성        | 모든 코드가 기본적으로 안전                    | 기본적으로 안전하지만 `unsafe`로 포인터 허용    |
| 포인터 사용        | `*const T`, `*mut T` → `unsafe`에서만 사용     | `int*`, `char*` 등 C 스타일 포인터 사용 가능     |
| null 포인터         | `std::ptr::null()` 사용                        | `null` 직접 사용 가능                           |
| 메모리 연산        | `offset`, `add` 등 → `unsafe` 필요             | 포인터 산술 가능 (`+`, `-`, `[]`)               |
| FFI 연동           | C/C++ 함수 호출 시 `unsafe` 필수               | C 함수 호출 시 `unsafe` 필요                    |
| 사용 범위 지정     | `unsafe` 블록 단위로 제한                     | `unsafe` 메서드, 블록, 클래스 전체 지정 가능     |
| 런타임 위험        | 세그폴트, UB 발생 가능                         | 널 참조, 버퍼 오버런 등 C 수준의 위험 존재       |


## 🔍 Rust unsafe의 특징
- 컴파일러가 안전성 검사를 하지 않는다는 선언.
- 하지만 여전히 타입 시스템, 빌림 검사기, 수명 검사기는 작동함.
- unsafe는 작게, 명확하게, 안전하게 감싸는 것이 권장됨.
- 예: 포인터 역참조, FFI, static mut, 고급 자료구조 구현 등

## 🔍 C# unsafe의 특징
- C 스타일 포인터(int*, byte*)를 사용할 수 있게 해줌.
- fixed 키워드로 GC가 이동시키지 않도록 메모리를 고정해야 함.
- 예: 이미지 처리, 네이티브 API 호출, 고성능 연산 등에서 사용됨

## 🎯 요약
Rust의 unsafe는 안전한 언어에서 제한적으로 위험을 허용하는 철학적 도구고,
C#의 unsafe는 안전한 언어에 C 스타일 포인터를 도입하는 실용적 기능.

Rust는 unsafe를 최소화하고 안전한 추상화로 감싸는 것을 강조하고,
C#은 unsafe를 성능 최적화나 네이티브 연동을 위한 실용적 수단으로 사용.

--- 

# Clone

fn split_at_unchecked<T>(slice: &mut [T], mid: usize)는 하나의 mutable slice를 두 개의 겹치지 않는 mutable slice로 나누는 함수. 
이 함수는 unsafe로 구현되며, 경계 검사를 하지 않기 때문에 **mid 값이 잘못되면 UB(Undefined Behavior)**가 발생할 수 있음.
그런데 여기서 핵심은:
이렇게 나눈 두 개의 &mut [T]를 각각 .clone() 하면 어떻게 될까?

## 🔍 &mut [T]는 Clone을 구현하지 않는다
Rust에서 &mut [T]는 mutable 참조이기 때문에, 기본적으로 Clone 트레이트를 구현하지 않음. 
즉, 아래와 같은 코드는 컴파일되지 않음:
```rust
let (left, right) = split_at_unchecked(&mut data, mid);
let left_clone = left.clone(); // ❌ 컴파일 에러
```

## ✅ 해결 방법: 데이터를 복사하고 싶다면 to_vec() 또는 to_owned()
만약 slice의 내용을 복사하고 싶다면, 실제 데이터를 복사하는 방식으로 해야 함:
```rust
let left_vec = left.to_vec();     // Vec<T>로 복사
let right_vec = right.to_vec();   // Vec<T>로 복사
```

이렇게 하면 left_vec과 right_vec은 원본과 독립적인 복사본이 되며, 안전하게 사용할 수 있음.

### ⚠️ 주의: &mut [T]는 독점적인 접근 권한을 의미
&mut [T]는 해당 메모리에 대해 단 하나의 mutable 접근만 허용하기 때문에, 복사해서 여러 개의 mutable 참조를 만들 수는 없어. 
Rust의 빌림 검사기가 이걸 강하게 막아주는 이유는 바로 데이터 레이스와 UB를 방지하기 위해서야.


## 🧠 요약: `&mut [T]` 슬라이스 복사 관련
| 메서드            | 가능 여부 | 결과 타입 | 설명                                 |
|-------------------|-----------|------------|--------------------------------------|
| `left.clone()`     | ❌        | 없음       | `&mut [T]`는 `Clone`을 구현하지 않음 |
| `left.to_vec()`    | ✅        | `Vec<T>`   | 슬라이스의 내용을 복사하여 새 벡터 생성 |
| `left.to_owned()`  | ✅        | `Vec<T>`   | `to_vec()`과 동일하게 동작함         |

---


