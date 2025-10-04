# Union

Rust의 union 타입은 C와의 FFI(Foreign Function Interface) 호환성을 위해 도입된 구조.
하지만 Rust의 안전성과 철학과는 거리가 있기 때문에  
    → 사용 시 반드시 unsafe가 필요하고, 프로그래머가 직접 상태를 추적해야 합니다.

## 🧩 왜 Rust에 union이 있는가?
### ✅ 1. C와의 구조적 호환
- C에서는 union을 통해 여러 타입을 같은 메모리 공간에 저장함
- 예: `union Value { int i; float f; char* s; };`
- Rust가 C 라이브러리와 안전하게 연동하려면  
    → 동일한 메모리 표현을 갖는 union이 필요
### ✅ 2. #[repr(C)]의 의미
- Rust의 메모리 레이아웃을 C와 동일하게 맞추기 위한 속성  
    → 필드들이 C에서 정의된 순서와 크기로 배치됨  
    → FFI에서 구조체나 유니온을 정확히 해석할 수 있음

## 🔍 Rust union의 특징
```rust
#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}
```

- i와 b는 같은 메모리 공간을 공유  
    → 실제로는 하나의 값만 유효하지만, Rust는 그걸 추적하지 않음  
    → 프로그래머가 직접 “지금 어떤 필드가 유효한지”를 관리해야 함  

## ⚠️ 왜 unsafe가 필요한가?
```rust
let u = MyUnion { i: 42 };
println!("int: {}", unsafe { u.i });         // OK
println!("부울: {}", unsafe {  u.b});       // ❌ UB (정의되지 않은 동작)
```
- u.i는 우리가 직접 초기화했으므로 안전
- u.b는 초기화하지 않았는데 접근 → undefined behavior
- Rust는 이런 위험을 막기 위해 union의 필드 접근을 unsafe로 제한

## ✅ enum과의 차이점
| 항목               | `enum`                                      | `union`                                       |
|--------------------|---------------------------------------------|-----------------------------------------------|
| 메모리 구조        | tag + data (활성 variant 추적 가능)         | 모든 필드가 같은 메모리 공간 공유             |
| 안전성             | Rust가 활성 variant를 추적 → 안전            | 프로그래머가 직접 추적해야 함 → `unsafe` 필요 |
| 필드 접근 방식     | `match`로 안전하게 분기                     | `unsafe { u.field }`로 직접 접근              |
| 용도               | 상태 표현, 로직 분기                        | FFI, 저수준 메모리 제어용                     |
| `#[repr(C)]` 필요  | 선택적                                      | C와 호환하려면 필수                           |
| 런타임 검사        | 있음 (tag 기반)                             | 없음 (UB 가능성 존재)                         |

→ 즉, 데이터가 어디에 있는지는 내가 직접 관리해야 하고, Rust는 그걸 알려주지도, 검사하지도 않습니다.

## 🧩 Rust union vs C++ union — 핵심 차이
| 항목               | Rust `union`                        | C++ `union` / `std::variant`             |
|--------------------|-------------------------------------|------------------------------------------|
| 메모리 공유        | ✅ 모든 필드가 같은 공간 공유         | ✅ 동일                                   |
| 안전성             | ❌ `unsafe` 필요                     | ❌ 기본 `union`은 unsafe, `variant`는 안전 |
| 활성 필드 추적     | ❌ 프로그래머가 직접 추적            | ❌ `union`은 수동, ✅ `variant`는 자동 추적 |
| 타입 검사          | ✅ 강한 타입 시스템                  | ❌ `union`은 약함, ✅ `variant`는 강함      |
| FFI 호환성         | ✅ `#[repr(C)]` 필요                 | ✅ C ABI와 호환                           |
| 런타임 검사        | ❌ 없음                              | ❌ `union` 없음, ✅ `variant` 있음          |
| 안전한 대안        | ❌ 없음 (직접 enum + union 조합 필요) | ✅ `std::variant`                        


## 🔍 Rust에서는 왜 “어디에 데이터가 있는지” 알 수 없나?
- union은 모든 필드가 같은 메모리 공간을 공유합니다
- Rust는 enum처럼 tag를 저장하지 않기 때문에  
    → 현재 어떤 필드가 활성화됐는지 알 수 없음  
    → 이건 Rust가 일부러 안전하지 않다고 선언한 영역이에요
```rust
union MyUnion {
    i: u8,
    b: bool,
}
```

- MyUnion { i: 42 }를 만들면  
    → 메모리에는 42가 들어가지만  
    → b로 읽으면 bool로 해석되므로 UB 발생 가능

## ✅ Rust의 철학: 안전은 기본, union은 예외
- Rust는 기본적으로 모든 메모리 접근을 안전하게 보장하려고 설계됨
- 하지만 union은 FFI나 저수준 시스템 프로그래밍을 위해  
    → 일부러 “안전하지 않은 구조”로 남겨둠
- 그래서 union을 쓸 때는 항상 unsafe가 필요하고,  
    → “지금 어떤 필드가 유효한지”는 내가 직접 추적해야 함


---
# C++ Union 활용법
C++에서는 union, reinterpret_cast, 또는 memcpy를 통해 같은 메모리 공간을 다른 타입으로 해석하는 방식을 자주 씁니다.  
Rust에서도 이건 가능합니다.  
하지만 Rust는 명시적으로 unsafe로 선언하고, 그 위험을 프로그래머가 책임지도록 설계되어 있습니다.

## 🧩 Rust에서 “같은 메모리를 다른 타입으로 보는” 방법
### ✅ 1. union을 사용하는 방식
```rust
#[repr(C)]
union IntFloat {
    i: u32,
    f: f32,
}
fn main() {
    let raw = IntFloat { i: 0x40490fdb }; // 3.1415927f32의 비트 표현
    let pi = unsafe { raw.f };
    println!("Float: {}", pi); // 출력: 3.1415927
}
```
- 0x40490fdb는 IEEE 754로 표현된 3.1415927f32
- unsafe 블록에서 f로 읽으면 → 비트 그대로 float로 해석

### ✅ 2. transmute를 사용하는 방식 (더 위험)
```rust
use std::mem::transmute;

fn main() {
    let i: u32 = 0x40490fdb;
    let f: f32 = unsafe { transmute(i) };
    println!("Float: {}", f);
}
```

- transmute는 타입을 강제로 바꾸는 함수
- 매우 위험하므로 → 정확한 크기와 정렬이 맞아야 함
- Rust는 이걸 unsafe로 제한함

### ✅ 3. from_bits()를 사용하는 안전한 방식
```rust
fn main() {
    let i: u32 = 0x40490fdb;
    let f = f32::from_bits(i);
    println!("Float: {}", f);
}
```

- f32::from_bits(u32)는 비트를 그대로 해석해서 float로 변환  
    → Rust가 제공하는 안전한 API

## 🔍 Rust의 철학: 가능하지만 명시적으로 위험을 선언
- Rust는 C++처럼 “비트 해석”을 허용하지만  
    → 반드시 unsafe 또는 명시적 API (from_bits)를 통해서만 가능
- 이유는: 타입 안전성과 메모리 안정성을 지키기 위해

## ✅ 가장 안전한 방식: f64::from_bits(u64)
```rust
fn main() {
    let bytes: [u8; 8] = [0xdb, 0x0f, 0x49, 0x40, 0x00, 0x00, 0x00, 0x00]; // 3.1415927 in f64
    let raw = u64::from_le_bytes(bytes); // little-endian 해석
    let value = f64::from_bits(raw);
    println!("Double: {}", value); // 출력: 3.1415927
}
```

- from_le_bytes()는 바이트 배열을 u64로 해석
- from_bits()는 u64를 f64로 해석  
    → 완전히 안전하고 Rust가 보장하는 방식

## 🔍 만약 4바이트씩 읽어서 8바이트로 묶고 싶다면?
```rust
fn read_double_from_u32_pair(low: u32, high: u32) -> f64 {
    let raw: u64 = ((high as u64) << 32) | (low as u64);
    f64::from_bits(raw)
}

fn main() {
    let low = 0x40490fdb;
    let high = 0x00000000;
    let value = read_double_from_u32_pair(low, high);
    println!("Double: {}", value); // 출력: 3.1415927
}
```

- C++에서 reinterpret_cast<double*>(&buffer) 하던 걸
- Rust에서는 u32 두 개를 u64로 조합해서 f64::from_bits()로 해석

## ⚠️ transmute 방식 (가능하지만 위험)
```rust
use std::mem::transmute;

fn main() {
    let raw: u64 = 0x40490fdb00000000;
    let value: f64 = unsafe { transmute(raw) };
    println!("Double: {}", value);
}
```

- transmute는 타입을 강제로 바꾸는 함수
- 매우 위험하므로 → 크기와 정렬이 정확히 맞아야 함
- Rust는 이걸 unsafe로 제한

## ✅ 배열 → 슬라이스 → 타입 변환 흐름

예를 들어, 8바이트 배열을 f64로 해석하고 싶다면:
```rust
fn main() {
    let buffer: [u8; 8] = [0xdb, 0x0f, 0x49, 0x40, 0x00, 0x00, 0x00, 0x00];
    let slice = &buffer[..]; // 슬라이스로 가져오기

    let raw = u64::from_le_bytes(slice.try_into().unwrap());
    let value = f64::from_bits(raw);

    println!("Double: {}", value); // 출력: 3.1415927
}
```

- try_into()는 [u8] 슬라이스를 [u8; 8]로 변환
- from_le_bytes()는 바이트를 u64로 해석
- from_bits()는 u64를 f64로 해석

## 🔍 여러 개 쪼개서 처리할 수도 있어요
```rust
fn parse_f64_chunks(data: &[u8]) -> Vec<f64> {
    data.chunks(8)
        .map(|chunk| {
            let arr: [u8; 8] = chunk.try_into().unwrap();
            let raw = u64::from_le_bytes(arr);
            f64::from_bits(raw)
        })
        .collect()
}
```

- chunks(8)로 8바이트씩 슬라이스
- 각 슬라이스를 f64로 해석  
    → 혼재된 데이터에서 double만 뽑아내는 구조


## 🧩 Rust에서 Step 단위 처리 전략
### ✅ 1. 결과 버퍼를 전체로 잡지 말고, step 단위로 슬라이스
```rust
const STEP_SIZE: usize = 1024 * 1024; // 예: 1MB 단위

fn read_step(file: &mut File, step_index: usize) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![0u8; STEP_SIZE];
    let offset = step_index * STEP_SIZE;
    file.seek(SeekFrom::Start(offset as u64))?;
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}
```

- File::seek()으로 원하는 step 위치로 이동
- read_exact()으로 해당 step만 읽어오기
- → 메모리 부담 없이 필요한 부분만 처리 가능

### ✅ 2. step 단위로 해석
```rust
fn parse_step(buffer: &[u8]) -> Vec<f32> {
    buffer
        .chunks(4)
        .map(|chunk| {
            let arr: [u8; 4] = chunk.try_into().unwrap();
            f32::from_le_bytes(arr)
        })
        .collect()
}
```

- 4바이트씩 쪼개서 f32로 해석
- → LS-DYNA explicit 결과와 동일한 방식

### ✅ 3. double 결과도 동일하게 처리
```rust
fn parse_step_double(buffer: &[u8]) -> Vec<f64> {
    buffer
        .chunks(8)
        .map(|chunk| {
            let arr: [u8; 8] = chunk.try_into().unwrap();
            f64::from_le_bytes(arr)
        })
        .collect()
}
```

- implicit 해석 결과를 f64로 해석  
    → 구조를 알고 있는 해석자가 정확히 해석

## 🔍 추가 전략: mmap, streaming, chunked I/O
- memmap2 크레이트로 파일을 메모리에 매핑해서  
    → 슬라이스처럼 접근 가능
- BufReader로 streaming 처리
- rayon으로 step 단위 병렬 처리도 가능


## ✅ 핵심 API: seek()
```rust
use std::fs::File;
use std::io::{Seek, SeekFrom};

fn main() -> std::io::Result<()> {
    let mut file = File::open("result.d3plot")?;

    // 예: 1GB 위치로 이동
    file.seek(SeekFrom::Start(1024 * 1024 * 1024))?;

    // 이후 원하는 만큼 읽기
    let mut buffer = [0u8; 4096];
    file.read_exact(&mut buffer)?;

    Ok(())
}
```

- SeekFrom::Start(offset) → 파일 시작 기준으로 이동
- SeekFrom::Current(offset) → 현재 위치 기준
- SeekFrom::End(offset) → 파일 끝 기준

## 🔍 C의 fseek() / setpos()와 대응
| C/C++ 함수        | Rust 대응 함수                     |
|------------------|------------------------------------|
| `fseek(fp, ...)` | `file.seek(SeekFrom::...)`         |
| `setpos()`       | `file.seek(SeekFrom::Start(pos))`  |

## ✅ 추가 설명
- SeekFrom::Start(pos) → 파일의 시작 위치에서 pos 바이트만큼 이동
- SeekFrom::Current(offset) → 현재 위치 기준으로 이동
- SeekFrom::End(offset) → 파일 끝 기준으로 이동
    → C의 fseek(fp, offset, SEEK_SET)와 완전히 대응되는 구조입니다
    → 기능적으로 동일하며, Rust는 타입 안전성과 에러 처리를 더 강화한 구조

---

