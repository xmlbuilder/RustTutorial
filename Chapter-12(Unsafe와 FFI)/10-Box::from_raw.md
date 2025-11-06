# Box::from_raw / Vec::from_raw_parts / std::slice::from_raw_parts_mut

`Box::from_raw(ptr)` 는 FFI에서 외부(C/C++)로부터 받은 포인터를 Rust의 안전한 `소유권 타입(Box)` 으로 변환하는 핵심 도구입니다.  
하지만 `매우 위험한 unsafe 함수` 이므로 메모리 소유권, 할당 방식, drop 시점 등을 정확히 이해하고 사용해야 합니다.

## 📦 `Box::from_raw(ptr)` 란?
- 정의: Box::from_raw(ptr: *mut T) -> Box<T>
- 역할: 외부에서 할당된 메모리 블록을 Rust의 Box<T>로 감싸서 소유권을 획득
- 전제 조건: 해당 포인터는 반드시 Box::into_raw()로 생성된 것이거나, Rust의 할당 규칙과 호환되는 방식으로 생성되어야 함

## 🧠 FFI 관점에서의 사용 시나리오
### 1. C에서 메모리 할당 → Rust에서 해제
```rust
// C 코드
void* allocate_buffer(size_t size) {
    return malloc(size);
}
```
```rust
extern "C" {
    fn allocate_buffer(size: usize) -> *mut u8;
}
```
```rust
fn main() {
    unsafe {
        let ptr = allocate_buffer(1024);
        let boxed: Box<[u8]> = Box::from_raw(std::slice::from_raw_parts_mut(ptr, 1024));
        // 사용 후 drop으로 자동 해제됨
    }
}
```

- 주의: Rust의 Box는 liballoc 기반의 heap allocator를 사용하므로, C의 malloc과 호환되지 않음  
    → 이 경우 Box::from_raw()로 감싸면 `undefined behavior` 발생 가능

###  2. Rust에서 할당 → C에 전달 → 다시 Rust에서 해제
```rust
#[no_mangle]
pub extern "C" fn create_buffer() -> *mut u8 {
    let boxed = vec![0u8; 1024].into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed); // drop 방지
    ptr
}
```
```rust
#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut u8) {
    unsafe {
        let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, 1024));
        // drop으로 자동 해제됨
    }
}
```
- 이 방식은 Rust가 할당하고 Rust가 해제하므로 안전하게 Box::from_raw() 사용 가능

## ⚠️ 위험 요소 및 주의사항
| 항목                                | 설명                                                                 |
|-------------------------------------|----------------------------------------------------------------------|
| `Box::from_raw()`                   | 포인터를 Box로 감싸면서 소유권을 가져오지만, drop 책임도 함께 생김     |
| from_raw() 후 drop 누락             | drop을 호출하지 않으면 메모리 누수 발생                              |
| null 포인터                         | `null` 포인터를 넘기면 즉시 undefined behavior 발생                   |
| double free                         | 같은 포인터를 여러 번 `from_raw()` 하면 중복 해제로 인해 UB 발생       |
| slice 처리                          | 단일 값은 `Box::from_raw(ptr)` 사용, 배열은 `Box::from_raw(slice::from_raw_parts_mut(ptr, len))` 필요 |
| 포인터 유효성                       | 포인터가 유효하고 Rust


## ✅ 안전하게 사용하려면
- 포인터가 Rust에서 할당된 것인지 확실히 확인
- slice일 경우 반드시 길이 정보와 함께 재구성
- drop 시점은 명확하게 제어하거나 ManuallyDrop과 함께 사용
- FFI에서 malloc/free를 사용하는 경우에는 Box::from_raw() 대신 C 전용 해제 함수 사용

## 🔐 대안: Vec::from_raw_parts
- Vec::from_raw_parts(ptr, len, capacity)는 Box::from_raw()보다 더 유연하게 메모리 블록을 재구성할 수 있음
- 특히 C에서 malloc으로 할당한 메모리를 Rust에서 Vec으로 감싸고 싶을 때 유용

---

## 🧠 Vec::from_raw_parts
### 📌 정의
```rust
pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>
```
- ptr: 데이터가 있는 포인터
- length: 실제 요소 개수
- capacity: 할당된 공간의 총 개수

### ✅ 예시
```rust
fn main() {
    let mut v = Vec::with_capacity(10);
    v.extend_from_slice(&[1, 2, 3]);

    let ptr = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    std::mem::forget(v); // drop 방지

    let rebuilt = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("{:?}", rebuilt); // [1, 2, 3]
}
```
- Vec은 length와 capacity를 모두 관리하므로 from_raw_parts가 필요
- FFI에서 malloc으로 할당한 메모리를 Vec으로 감싸려면 반드시 capacity도 알아야 안전

## 🧠 Box::from_raw
### 📌 정의
```rust
pub unsafe fn from_raw(ptr: *mut T) -> Box<T>
```
- ptr: 단일 값 또는 slice의 포인터

### ✅ 단일 값 예시
```rust
fn main() {
    let b = Box::new(42);
    let ptr = Box::into_raw(b);

    let rebuilt = unsafe { Box::from_raw(ptr) };
    println!("{}", rebuilt); // 42
}
```

### ✅ slice 예시
```rust
fn main() {
    let v = vec![10, 20, 30];
    let boxed = v.into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    let len = boxed.len();

    std::mem::forget(boxed);

    let rebuilt = unsafe {
        Box::from_raw(std::slice::from_raw_parts_mut(ptr, len))
    };
    println!("{:?}", rebuilt); // [10, 20, 30]
}
```
- Box<[T]>는 slice이므로 slice::from_raw_parts_mut로 먼저 감싸야 함

## ⚠️ 주의사항
| 항목                                | 설명                                                                 |
|-------------------------------------|----------------------------------------------------------------------|
| 소유권 이전                         | `Box::from_raw()`은 포인터의 소유권을 Rust로 이전함 → drop 책임 발생   |
| drop 방지                           | 기존 `Box` 또는 `Vec`을 `std::mem::forget()`으로 drop되지 않게 해야 함 |
| slice 처리                          | 배열 포인터는 `Box::from_raw(slice::from_raw_parts_mut(...))` 형태로 감싸야 안전 |
| 포인터 유효성                       | 포인터가 유효하고 Rust의 할당 규칙과 호환되는지 반드시 확인해야 함     |
| 중복 해제                           | 같은 포인터를 여러 번 `from_raw()` 하면 double free로 인해 UB 발생     |


## ✅ 요약 비교
| 항목                     | Vec::from_raw_parts                            | Box::from_raw                                      |
|--------------------------|------------------------------------------------|----------------------------------------------------|
| 대상                     | 가변 길이 배열 (Vec<T>)                        | 단일 값 또는 slice                                 |
| 필요한 정보              | ptr, length, capacity                          | ptr만 필요 (slice일 경우 길이 포함한 slice 필요)   |
| slice 처리               | 직접 처리 가능                                 | `Box::from_raw(slice::from_raw_parts_mut(...))` 필요 |
| drop 책임                | Vec이 drop 시 자동 해제                        | Box이 drop 시 자동 해제                            |
| FFI 적합도               | C에서 malloc한 배열을 Rust Vec으로 감쌀 때     | C에서 malloc한 단일 값 또는 slice를 감쌀 때        |

---

## 🧠 타입을 식별하는 방법 (FFI에서)
### 1. 🔖 외부에서 타입을 명시적으로 전달
가장 일반적인 방법은 C/C++ 또는 외부 시스템에서 타입 정보를 함께 전달하는 것입니다:
```cpp
// C 예시
void* get_buffer(size_t* len, int* type_code); // type_code: 1 = i32, 2 = f32, ...

enum TypeCode {
    I32 = 1,
    F32 = 2,
    // ...
}
```

```rust
unsafe {
    let mut len = 0;
    let mut type_code = 0;
    let ptr = get_buffer(&mut len, &mut type_code);

    match type_code {
        1 => {
            let slice = std::slice::from_raw_parts(ptr as *const i32, len);
            println!("i32 slice: {:?}", slice);
        }
        2 => {
            let slice = std::slice::from_raw_parts(ptr as *const f32, len);
            println!("f32 slice: {:?}", slice);
        }
        _ => panic!("Unknown type"),
    }
}
```

### 2. 🧬 타입을 구조체로 래핑해서 전달
#### C에서 구조체로 타입 정보를 포함시켜 전달하면 더 안전합니다:
```cpp
typedef struct {
    void* data;
    size_t len;
    int type_code;
} Buffer;
```

#### Rust에서 이를 FFI로 받아서 처리:
```rust
#[repr(C)]
struct Buffer {
    data: *mut std::ffi::c_void,
    len: usize,
    type_code: i32,
}
```

### 3. ❌ 타입을 추측하는 방식은 위험
Rust에서는 다음과 같은 방식은 절대 안전하지 않습니다:
```rust
let slice = std::slice::from_raw_parts(ptr as *const T, len); // T가 뭔지 모름
```

- T를 추측할 수 없기 때문에 UB 발생 가능
- 포인터 크기와 alignment만으로 타입을 유추하는 건 불가능

## ✅ 요약
| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| `int`                 | 타입 코드 전달 (예: 1 = i32, 2 = f32 등)                             |
| `data + len + type_code` | 포인터, 길이, 타입 정보를 구조체로 함께 전달 → 안전하고 명확한 처리 가능 |

Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)) 이 표현에서 타입이 명시적으로 보이지 않지만,  
Rust는 컴파일러가 타입을 추론할 수 있도록 문맥에서 이미 타입이 결정되어 있기 때문에 동작하는 것임.

## 🧠 타입 추론이 어떻게 이루어졌는가?
```rust
let v = vec![10, 20, 30]; // 타입: Vec<i32>
let boxed = v.into_boxed_slice(); // 타입: Box<[i32]>
let ptr = boxed.as_mut_ptr();     // 타입: *mut i32
let len = boxed.len();            // 타입: usize
```
- boxed가 Box<[i32]>이므로 ptr은 *mut i32
- std::slice::from_raw_parts_mut(ptr, len) → 타입: &mut [i32]
- 따라서 Box::from_raw(...) → 타입: Box<[i32]>  
    즉, 모든 타입 정보는 앞에서 이미 결정되어 있고, Rust의 강력한 타입 추론 시스템이 이를 자동으로 이어받아 처리.

## ✅ 만약 타입이 불명확하다면?
타입이 명확하지 않으면 컴파일러가 오류를 냅니다:
```rust
let ptr: *mut T = ...; // T가 뭔지 모르면 컴파일 불가
let slice = std::slice::from_raw_parts_mut(ptr, len); // ❌ T가 없으면 추론 불가

- 이럴 땐 명시적으로 타입을 지정해야 함:
```rust
let slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
```

| 흐름 단계                     | 설명                                                                 |
|------------------------------|----------------------------------------------------------------------|
| `Vec<i32>`                   | 가변 길이 벡터, 타입 명시됨                                          |
| `Box<[i32]>`                 | `Vec`를 `into_boxed_slice()`로 변환 → 타입 유지                      |
| `*mut i32`                   | `Box`에서 `as_mut_ptr()` 호출 → 포인터 타입 추론됨                   |
| `&mut [i32]`                 | `slice::from_raw_parts_mut(ptr, len)` → 포인터 기반 slice 생성        |
| `Box<[i32]>`                 | `Box::from_raw(slice)` → slice 타입 기반으로 Box 추론됨              |

- 타입 추론이 되지 않거나 컴파일러가 타입을 알 수 없을 때는 명시적으로 타입을 지정해주는 캐스팅을 통해 컴파일 오류를 피할 수 있습니다.

### ✅ 예: 타입 추론이 안 될 때 as *const f32로 명시
```rust
let ptr: *mut std::ffi::c_void = get_some_buffer(); // 타입 불명
let len: usize = get_length();

let float_ptr = ptr as *const f32;
let slice = unsafe { std::slice::from_raw_parts(float_ptr, len) };
```
- ptr은 원래 *mut c_void이므로 타입 정보가 없음
- as *const f32로 명시적으로 캐스팅하면 컴파일러가 T = f32로 인식
- 이후 from_raw_parts에서 타입 추론이 가능해짐

---

# 소유권 없이 사용하기

FFI에서 소유권 없이 메모리를 “사용만” 하고 싶을 때,  
Rust에서는 다음과 같은 방식으로 읽기/쓰기 접근은 하되 drop 책임은 지지 않는 안전한 구조를 만들 수 있음.

## 🧠 소유권 없이 메모리 사용하기: 핵심 전략
| 기술                          | 입력 포인터 타입 | 설명                                                         |
|-------------------------------|------------------|--------------------------------------------------------------|
| `*const T` / `*mut T`         | `*const T`, `*mut T` | 기본 포인터 타입. 소유권 없음. 직접 접근 가능. drop 없음.     |
| `std::slice::from_raw_parts()`| `*const T`       | 읽기 전용 slice 생성. drop 없음. 안전하게 읽기 가능.         |
| `std::slice::from_raw_parts_mut()` | `*mut T`     | 쓰기 가능한 slice 생성. drop 없음. 안전하게 수정 가능.        |
| `ManuallyDrop<T>`             | `T`              | drop을 막고 수동으로 해제 시점 제어. 소유권 없이 관리 가능.     |
| `PhantomData<T>`              | 타입 정보만      | 타입만 추적하고 drop은 하지 않음. zero-cost marker로 활용.     |


## ✅ 예시: C에서 받은 메모리 블록을 읽기만 하기
```rust
extern "C" {
    fn get_buffer() -> *const f32;
    fn get_length() -> usize;
}

fn main() {
    unsafe {
        let ptr = get_buffer();
        let len = get_length();

        let slice: &[f32] = std::slice::from_raw_parts(ptr, len);
        println!("{:?}", slice); // 읽기만 함, drop 없음
    }
}
```
- slice는 &[f32] 타입 → 읽기 전용
- Rust는 drop하지 않음 → 소유권 없음
- 메모리 해제는 C에서 책임져야 함

## ✅ 예시: 쓰기만 하고 해제는 하지 않기
```rust
extern "C" {
    fn get_buffer_mut() -> *mut u8;
    fn get_length() -> usize;
}

fn main() {
    unsafe {
        let ptr = get_buffer_mut();
        let len = get_length();

        let slice: &mut [u8] = std::slice::from_raw_parts_mut(ptr, len);
        slice[0] = 42; // 쓰기 가능
    }
}
```
- &mut [u8] → 쓰기 가능
- drop 없음 → 소유권 없이 사용만

| 목적               | 방법 및 설명                                              |
|--------------------|-----------------------------------------------------------|
| 읽기 전용 접근      | `*const T` → `&[T]`로 변환하여 안전하게 읽기               |
| 읽기/쓰기 접근      | `*mut T` → `&mut [T]`로 변환하여 안전하게 수정 가능         |
| drop 방지          | `ManuallyDrop<T>`로 감싸서 Rust가 자동으로 drop하지 않게 함 |
| 타입 정보 유지만   | `PhantomData<T>`로 타입만 추적하고 drop은 하지 않음         |

---



