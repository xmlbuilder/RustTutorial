# unsafe
Rust의 unsafe 기능은 단순히 “위험한 코드”를 허용하는 게 아니라, 안전한 시스템 프로그래밍을 위한 제어권을 개발자에게 넘겨주는 도구. 
C#의 unsafe와 비교하면서, C++과의 연동에서 어떤 장점이 있는지도 함께 정리.

## 🧠 Rust의 unsafe란?
Rust는 기본적으로 메모리 안전성을 보장하는 언어입니다. 하지만 시스템 프로그래밍에서는 때때로 저수준 작업이 필요.  
이때 사용하는 것이 unsafe입니다.
```rust
unsafe {
    let ptr = some_raw_pointer;
    *ptr = 42; // raw pointer dereference
}

```
## 🔧 unsafe에서 허용되는 작업
- raw pointer dereference (*const T, *mut T)
- unsafe 함수 호출 (extern "C" 포함)
- mutable static 변수 접근
- union 필드 접근
- FFI (Foreign Function Interface): C/C++ 함수 호출
unsafe는 Rust의 안전성 모델을 잠시 벗어나지만, 여전히 **컴파일러의 일부 검사(예: 타입 검사)**는 유지됩니다.



## 🆚 C#의 unsafe와 비교
| 항목               | Rust `unsafe`                        | C# `unsafe`                          |
|--------------------|--------------------------------------|--------------------------------------|
| 목적               | 시스템 수준 제어, FFI, 성능 최적화     | 포인터 연산, 네이티브 API 접근         |
| 메모리 안전성      | 기본적으로 안전, `unsafe`로 제어권 위임 | 기본적으로 안전, `unsafe`로 포인터 허용 |
| 사용 방식          | `unsafe { ... }` 블록 또는 함수        | `unsafe` 키워드로 함수/블록 선언       |
| FFI 연동           | `extern "C"`, `bindgen` 등 강력 지원   | `DllImport`, `fixed` 등 제한적 지원    |
| 런타임 검사        | 없음 (컴파일 타임 중심)                | 일부 런타임 검사 있음                   |
| 성능               | C++ 수준의 고성능                      | .NET 런타임 위에서 동작                 |


Rust는 컴파일 타임에 안전성을 강하게 보장하고, unsafe는 그 경계를 명확히 표시합니다.
C#은 런타임 기반 언어이기 때문에 unsafe는 제한적이며, 주로 포인터 연산에 사용됩니다.


## 🤝 Rust와 C++ 연동 시 unsafe의 역할
Rust는 C/C++과 연동할 때 **FFI (Foreign Function Interface)**를 사용하며, 이 과정은 대부분 unsafe입니다.  
이유는:
- Rust는 외부 함수의 메모리 안전성 보장 불가
- C/C++의 포인터, 구조체, 메모리 모델은 Rust와 다름
- Rust 컴파일러는 외부 코드의 동작을 추론할 수 없음

## ✨ 얻을 수 있는 장점
| 항목                   | 설명                                                   |
|------------------------|--------------------------------------------------------|
| 고성능 라이브러리 활용 | `OpenCV`, `BLAS`, `SQLite`, `zlib` 등 C/C++ 라이브러리 직접 사용 가능 |
| 점진적 마이그레이션    | 기존 C/C++ 시스템을 Rust로 부분적으로 이전 가능           |
| 안전성 도입            | 핵심 로직은 Rust로 작성해 메모리 오류 방지, 외부 라이브러리는 FFI로 연결 |
| 성능 유지              | C++ 수준의 성능을 유지하면서 Rust의 안전성 모델 적용 가능   |
| 시스템 통합            | OS API, 디바이스 드라이버, 네이티브 코드와 직접 연동 가능     |

예: Servo 브라우저는 Rust로 작성되었지만, JavaScript 엔진은 C++로 작성된 SpiderMonkey를 FFI로 연동합니다.


### ⚠️ 주의할 점
- unsafe는 “무조건 위험하다”는 뜻이 아니라, “개발자가 책임진다”는 뜻입니다.
- unsafe 블록 안에서도 타입 검사, 라이프타임 검사 일부는 유지됩니다.
- C/C++과 연동할 때는 Null pointer, dangling pointer, buffer overflow 등 전통적인 오류에 주의해야 합니다.

## 🧠 결론
Rust의 unsafe는 제어권을 넘겨주는 도구이며,
C#의 unsafe보다 훨씬 강력하고 정교한 시스템 프로그래밍을 가능하게 합니다.
특히 C++과 연동할 때는 unsafe를 통해 성능과 안정성의 균형을 잡을 수 있습니다.

---

# C와 연동

Rust와 C를 연동하는 가장 기본적인 FFI 샘플 코드입니다. 
Rust에서 C 함수를 호출하는 구조. 
이 예제는 간단한 add 함수를 C에서 정의하고, Rust에서 호출하는 방식입니다.

## 🧩 1. C 코드 (add.c)
```rust
// add.c
int add(int a, int b) {
    return a + b;
}
```

## 🔧 컴파일
```
gcc -c add.c -o add.o
```


## 🧩 2. Rust 코드 (main.rs)
```rust
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        let result = add(3, 4);
        println!("3 + 4 = {}", result);
    }
}
```

- extern "C": C ABI를 따르는 외부 함수 선언
- unsafe: Rust는 외부 함수의 안전성을 보장할 수 없기 때문에 unsafe 블록에서 호출

## 🛠️ 3. Cargo 설정 (Cargo.toml)
```
[package]
name = "rust_ffi_sample"
version = "0.1.0"
edition = "2021"

[build-dependencies]
cc = "1.0"
```


## 🛠️ 4. 빌드 스크립트 (build.rs)
```rust
fn main() {
    cc::Build::new()
        .file("add.c")
        .compile("libadd.a");
}
```

- Rust의 cc 크레이트를 사용해 C 코드를 컴파일하고 링크

### 📦 최종 구조
```
rust_ffi_sample/
├── Cargo.toml
├── build.rs
├── add.c
└── src/
    └── main.rs
```


### ✅ 실행 결과
```
cargo run
# 출력: 3 + 4 = 7
```


## 🧠 확장 아이디어
- #[repr(C)] 구조체를 사용해 C와 Rust 간 데이터 구조 공유
- bindgen을 사용해 C 헤더 파일을 자동으로 Rust 바인딩 생성
- C++과 연동할 때는 extern "C"로 name mangling 방지



## 🔧 Rust가 C 코드를 "직접" 컴파일하진 않지만…
Rust는 자체적으로 C 컴파일러를 포함하고 있지 않아요. 하지만 다음 방식으로 C 컴파일을 자동화할 수 있습니다:
### ✅ 방법 1: cc 크레이트 사용
```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/add.c")
        .compile("libadd.a");
}
```

- Rust가 gcc 또는 clang을 호출해서 C 파일을 컴파일
- 결과는 .a 정적 라이브러리로 생성되어 Rust에 링크됨

### ✅ 방법 2: bindgen으로 C 헤더 자동 바인딩
[build-dependencies]
bindgen = "0.68"


- C 헤더 파일을 분석해서 Rust용 FFI 바인딩을 자동 생성
- build.rs에서 bindgen::Builder를 사용

## 🧠 요약
| 항목               | 설명                                                   |
|--------------------|--------------------------------------------------------|
| Rust 자체 컴파일러 | C 코드 컴파일 기능 없음                                 |
| 자동화 방식        | `build.rs` + `cc` 크레이트로 C 컴파일 자동화 가능         |
| 필요 도구          | 시스템에 `gcc`, `clang` 등 C 컴파일러가 설치되어 있어야 함 |
| 빌드 통합          | Rust 빌드 시 C 코드도 함께 컴파일되어 일관된 환경 유지 가능 |
| FFI 연동           | `extern "C"`로 C 함수 호출, `#[repr(C)]`로 구조체 공유 가능 |



# Lib 연동
Rust에서 기존의 DLL (Windows), LIB (정적 라이브러리),  
또는 **SO (Linux 공유 라이브러리)**를 불러오는 방법은 **FFI (Foreign Function Interface)**를 통해 이루어지며,  
unsafe와 extern "C" 키워드를 활용.

## 🧩 1. Windows: .dll 또는 .lib 사용
## 🔧 DLL 함수 선언
```rust
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}
```

### 🔧 DLL 로드 방식
- 정적 링크: .lib 파일을 함께 제공받은 경우, cargo:rustc-link-lib로 링크
- 동적 로딩: libloading 크레이트를 사용해 런타임에 DLL을 로드

```rust
use libloading::Library;

fn main() {
    unsafe {
        let lib = Library::new("your.dll").unwrap();
        let func: libloading::Symbol<unsafe extern fn(i32, i32) -> i32> =
            lib.get(b"add").unwrap();
        println!("Result: {}", func(2, 3));
    }
}
```

### 🔧 build.rs 설정 예시
```rust
fn main() {
    println!("cargo:rustc-link-lib=dylib=yourlib");
    println!("cargo:rustc-link-search=native=path/to/lib");
}
```


## 🧩 2. Linux/macOS: .so 또는 .a 사용

### 🔧 SO 함수 선언
```rust
extern "C" {
    fn compute(x: f64) -> f64;
}
```

## 🔧 링크 설정
```
# Cargo.toml
[build-dependencies]
cc = "1.0"
```
```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=dylib=yourlib");
    println!("cargo:rustc-link-search=native=./libs");
}
```

- .so 파일은 LD_LIBRARY_PATH에 경로를 추가하거나, 실행 시 환경 변수로 지정

## 🧠 핵심 요약
| 항목               | 설명                                                   |
|--------------------|--------------------------------------------------------|
| `extern "C"` + `unsafe` | C ABI로 외부 함수 선언, 메모리 안전성은 개발자가 책임짐     |
| `.lib`, `.a` + `cargo:rustc-link-lib` | 정적 라이브러리 링크를 위한 빌드 설정 (`build.rs`에서 사용) |
| `libloading` + `.dll`, `.so` | 동적 라이브러리 로딩을 위한 런타임 API (`libloading` 크레이트) |
| `#[repr(C)]`       | C와 동일한 메모리 레이아웃을 보장하는 구조체 선언 방식         |
| `#[no_mangle]`     | 함수 이름을 그대로 유지해 C에서 호출 가능하게 함               |



## ✨ 실전 팁
- C/C++ 라이브러리와 연동할 때는 반드시 함수 시그니처와 ABI를 정확히 맞춰야 합니다.
- bindgen을 사용하면 C 헤더 파일을 자동으로 Rust 바인딩으로 변환할 수 있어 편리합니다.
- #[repr(C)] 구조체를 사용하면 C와 Rust 간 데이터 구조를 안전하게 공유할 수 있습니다.

---
