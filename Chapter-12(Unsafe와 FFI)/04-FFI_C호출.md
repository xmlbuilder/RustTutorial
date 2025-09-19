# C++ 연동
Rust와 C은 직접적으로 FFI를 통해 연결할 수 있지만, **C 클래스와의 호환성은 까다롭고 주의가 필요**.  
Rust는 기본적으로 C ABI만 지원하기 때문에, C++의 클래스, 가상 함수, 템플릿, 이름 맹글링(name mangling) 같은 고유한 기능은 바로 다룰 수 없음.  
하지만 몇 가지 방법으로 간접적으로 연동할 수 있음.

## 🧠 C++ 클래스와 직접 호환되지 않는 이유

| C++ 기능         | Rust에서 직접 지원 여부 | 호환되지 않는 이유                                |
|------------------|--------------------------|---------------------------------------------------|
| 클래스 구조       | ❌                       | Rust는 C++의 객체 모델과 메모리 레이아웃을 이해하지 못함 |
| 가상 함수         | ❌                       | vtable 구조가 다르고, ABI가 호환되지 않음         |
| 템플릿            | ❌                       | Rust에는 템플릿 개념이 없고, 제네릭 방식이 다름     |
| 이름 맹글링       | ❌                       | C++는 함수 이름을 컴파일 시 변경 (mangling)함     |
| 상속 및 다형성    | ❌                       | Rust는 상속을 지원하지 않으며, C++의 다형성과 구조가 다름 |

## ✅ 해결 방법: C++ 클래스를 C 스타일로 래핑
### 1. C++에서 클래스 기능을 C 함수로 래핑
```cpp
// my_class.hpp
class MyClass {
public:
    int value;
    MyClass(int v) : value(v) {}
    int get_value() { return value; }
};

extern "C" {
    MyClass* my_class_new(int v);
    int my_class_get_value(MyClass* instance);
}


// my_class.cpp
#include "my_class.hpp"

extern "C" {
    MyClass* my_class_new(int v) {
        return new MyClass(v);
    }

    int my_class_get_value(MyClass* instance) {
        return instance->get_value();
    }
}
```

이렇게 하면 C++ 클래스의 기능을 C 함수처럼 사용할 수 있어.

## 2. Rust에서 C++ 래핑 함수 호출
```rust
#[repr(C)]
pub struct MyClass;

extern "C" {
    fn my_class_new(v: i32) -> *mut MyClass;
    fn my_class_get_value(instance: *mut MyClass) -> i32;
}

fn main() {
    unsafe {
        let obj = my_class_new(42);
        let val = my_class_get_value(obj);
        println!("Value from C++: {}", val);
    }
}
```


## 🧱 더 안전하고 현대적인 방법: cxx 크레이트 사용
Rust 생태계에는 라는 라이브러리가 있어. 이건 C++ 클래스와 Rust를 더 안전하고 직관적으로 연결해주는 도구.
```
[dependencies]
cxx = "1.0"
cxx-build = "1.0"
```

C++ 클래스와 Rust 구조체를 서로 연결하고, 메서드도 직접 호출할 수 있음. 
이름 맹글링, 메모리 안전성, 빌드 설정까지 자동으로 처리해줘서 훨씬 편리하고 안전.

## 🎯 요약: Rust ↔ C++ 클래스 연동 방식
| 연동 방식          | Rust 측 구현 방식             | C++ 측 요구사항 또는 특징         | 설명                                      |
|-------------------|-------------------------------|-----------------------------------|-------------------------------------------|
| C 스타일 래핑      | `extern "C"` 함수로 클래스 기능 노출 | 클래스 기능을 C 함수로 감싸야 함     | 직접 클래스는 못 쓰지만 기능은 호출 가능 |
| `cxx` 크레이트 사용 | `#[cxx::bridge]`로 안전한 바인딩 생성 | C++ 클래스 정의를 그대로 사용 가능   | 클래스, 메서드, 구조체까지 안전하게 연동 |

---

# cxx 사용법

C++ 클래스와 직접 호환되진 않지만, 간접적으로 래핑하거나 cxx 같은 도구를 쓰면 충분히 연동 가능.  


## 🧱 1. 프로젝트 구조
```
rust-cxx-sample/
├── Cargo.toml
├── build.rs
├── src/
│   ├── main.rs
│   └── lib.rs
├── include/
│   └── demo.h
└── src/
    └── demo.cc
```


## 🧰 2. Cargo.toml
```
[package]
name = "rust_cxx_sample"
version = "0.1.0"
edition = "2021"

[dependencies]
cxx = "1.0"

[build-dependencies]
cxx-build = "1.0"
```


## 🛠️ 3. build.rs
```rust
fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/demo.cc")
        .include("include")
        .flag_if_supported("-std=c++14")
        .compile("demo");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/demo.cc");
    println!("cargo:rerun-if-changed=include/demo.h");
}
```


## 🧩 4. src/lib.rs — Rust ↔ C++ 연결 정의
```rust
#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("demo.h");

        type Demo;

        fn new_demo(value: i32) -> UniquePtr<Demo>;
        fn get_value(self: &Demo) -> i32;
    }
}
```


## 🧠 5. include/demo.h — C++ 클래스 헤더
```cpp
#pragma once

class Demo {
public:
    Demo(int value);
    int get_value() const;

private:
    int value_;
};

Demo* new_demo(int value);
```


## ⚙️ 6. src/demo.cc — C++ 클래스 구현
```cpp
#include "demo.h"

Demo::Demo(int value) : value_(value) {}

int Demo::get_value() const {
    return value_;
}

Demo* new_demo(int value) {
    return new Demo(value);
}
```


## 🚀 7. src/main.rs — Rust에서 C++ 클래스 사용
```rust
use rust_cxx_sample::ffi;

fn main() {
    let demo = ffi::new_demo(42);
    println!("Value from C++: {}", demo.get_value());
}
```


## ✅ 실행
```
cargo run
```

출력:
```
Value from C++: 42
```

## 🎯 요약: Rust ↔ C++ 연동 (cxx 크레이트 기반)

| 구성 요소         | 역할 또는 설명                                      |
|------------------|-----------------------------------------------------|
| `#[cxx::bridge]`  | Rust ↔ C++ 타입 및 함수 바인딩 정의                 |
| `UniquePtr<T>`    | C++ 객체를 Rust에서 안전하게 소유하고 관리하는 포인터 |
| `build.rs`        | C++ 코드 컴파일 및 바인딩 설정                      |
| `.h` / `.cc` 파일 | C++ 클래스 및 함수 정의, Rust에서 호출될 대상       |

---


# C와 연동 

Rust와 C를 구조체 단위로 연동하는 건 FFI(Foreign Function Interface)의 핵심 중 하나.  
Rust는 기본적으로 C와 ABI 호환이 되기 때문에, 몇 가지 규칙만 지키면 구조체도 안전하게 주고받을 수 있음.

## 🧱 1. 핵심 개념: #[repr(C)]
Rust의 구조체는 기본적으로 메모리 정렬 방식이 Rust 내부 규칙을 따르는데, C와 호환되려면 반드시 #[repr(C)]를 붙여야 함. 
이건 메모리 레이아웃을 C 스타일로 고정해주는 역할을 해.
```rust
#[repr(C)]
pub struct MyStruct {
    pub x: i32,
    pub y: f32,
}
```


## 🧰 2. C 헤더 파일 정의
Rust 구조체와 동일한 레이아웃을 C에서도 정의해야 함:
```cpp
// my_struct.h
typedef struct {
    int x;
    float y;
} MyStruct;

void print_struct(MyStruct s);
```


## 🧩 3. Rust에서 C 함수 선언

```rust
extern "C" {
    fn print_struct(s: MyStruct);
}
```

이렇게 하면 Rust에서 C 함수를 호출할 수 있어.

## 🧪 4. C 함수 구현
```c
// my_struct.c
#include <stdio.h>
#include "my_struct.h"

void print_struct(MyStruct s) {
    printf("x = %d, y = %f\n", s.x, s.y);
}
```


## 🚀 5. Rust에서 구조체 생성 및 전달
```rust
fn main() {
    let s = MyStruct { x: 10, y: 3.14 };
    unsafe {
        print_struct(s);
    }
}
```
- unsafe는 필수야. Rust는 외부 함수의 안전성을 보장하지 않기 때문이지.

## 🧠 구조체 포인터를 넘기고 싶다면?
C에서 구조체를 포인터로 받는 경우도 많음. 
그럴 땐 Rust에서 *mut MyStruct 또는 *const MyStruct를 사용하면 돼.
C 코드
```c
void modify_struct(MyStruct* s) {
    s->x += 1;
    s->y *= 2;
}
```

Rust 선언
```rust
extern "C" {
    fn modify_struct(s: *mut MyStruct);
}
```

Rust 사용
```rust
fn main() {
    let mut s = MyStruct { x: 5, y: 1.5 };
    unsafe {
        modify_struct(&mut s);
    }
    println!("Modified: x = {}, y = {}", s.x, s.y);
}
```


## 🧯 주의사항: Rust ↔ C 구조체 연동 시 필수 체크
| 항목           | 설명                                         |
|----------------|----------------------------------------------|
| `#[repr(C)]`   | Rust 구조체의 메모리 레이아웃을 C와 호환되게 지정 |
| 타입 일치      | `i32`, `f32` 등 C와 정확히 크기/정렬 맞춰야 함     |
| unsafe 블록    | 외부 함수 호출은 항상 `unsafe`로 감싸야 함         |
| 포인터 유효성  | C에 넘기는 포인터는 null 체크 및 메모리 유효성 보장 |
| 구조체 크기    | Rust와 C에서 구조체 크기가 완전히 동일해야 함     |



## 🎯 요약: Rust ↔ C 구조체 FFI 연동 핵심
| 항목            | 설명 또는 역할                                      |
|-----------------|-----------------------------------------------------|
| `#[repr(C)]`     | Rust 구조체의 메모리 레이아웃을 C와 호환되게 지정     |
| `extern "C"`     | C 함수 또는 구조체를 Rust에서 호출 가능하게 선언      |
| `unsafe`         | 외부 함수 호출 시 Rust가 안전성을 보장하지 않으므로 필요 |
| `*mut T`, `*const T` | 구조체를 포인터로 전달할 때 사용하는 타입 (가변/불변) |

---

# C 접근

Rust에서 C 함수나 구조체를 사용하는 방식은 두 가지:

## 🧭 1. Rust 구조 안에 직접 포함되는 경우
이건 C 코드를 Rust 프로젝트 안에서 함께 컴파일하는 방식. 
예를 들어:
- build.rs에서 .c 파일을 컴파일하도록 설정
- cc 또는 cxx-build 크레이트를 사용
- C 함수는 extern "C"로 선언해서 Rust에서 직접 호출
📦 이 방식은 DLL을 따로 만들지 않고, Rust 바이너리에 C 코드가 정적으로 포함.
👉 즉, Rust 구조 안에 들어오는 방식.

## 🧭 2. DLL(또는 .so, .dylib)을 사용하는 경우
이건 C 코드가 외부 라이브러리로 컴파일되어 있고, Rust는 그걸 동적으로 로드해서 사용하는 방식이야.
- C 코드를 먼저 .dll, .so, .dylib로 빌드
- Rust에서 #[link(name = "libname")]으로 연결
- 함수는 extern "C"로 선언하고 unsafe로 호출
📦 이 방식은 C 코드가 Rust 프로젝트 외부에 있고, 실행 시 동적 링크를 통해 연결.
👉 즉, DLL을 사용하는 방식.

## 🎯 요약: Rust ↔ C 구조체 연동 방식
| 연동 방식             | 구현 위치            | 결과물 또는 연결 방식       | 설명                                       |
|----------------------|----------------------|-----------------------------|--------------------------------------------|
| 내부 포함 방식        | Rust 프로젝트 내부    | 정적 링크 (바이너리에 포함) | C 코드를 Rust와 함께 컴파일하여 포함         |
| 외부 라이브러리 방식  | 별도 DLL 또는 SO 파일 | `.dll`, `.so`, `.dylib`     | Rust에서 외부 라이브러리를 동적으로 호출     |


### 1. Rust와 C를 구조체 단위로 연동하는 FFI 프로젝트를 만들 때의 전체 디렉토리 구조, build.rs 설정, 그리고 핵심 코드 샘플을 단계별로 설명. 
C 코드를 Rust 프로젝트에 내부 포함하는 방식.

#### 📁 프로젝트 디렉토리 구조
```
rust_c_ffi_sample/
├── Cargo.toml
├── build.rs
├── src/
│   └── main.rs
├── c_src/
│   ├── my_struct.h
│   └── my_struct.c
```


#### 🧰 Cargo.toml
```
[package]
name = "rust_c_ffi_sample"
version = "0.1.0"
edition = "2021"

[build-dependencies]
cc = "1.0"
```

cc 크레이트는 C 코드를 Rust 빌드 과정에 포함시켜주는 도구.


#### 🛠️ build.rs
```rust
fn main() {
    cc::Build::new()
        .file("c_src/my_struct.c")
        .include("c_src")
        .compile("my_struct");

    println!("cargo:rerun-if-changed=c_src/my_struct.c");
    println!("cargo:rerun-if-changed=c_src/my_struct.h");
}
```

C 코드를 컴파일하고 Rust 바이너리에 정적으로 링크.


#### 🧩 c_src/my_struct.h (C 헤더)
```c
#ifndef MY_STRUCT_H
#define MY_STRUCT_H

typedef struct {
    int x;
    float y;
} MyStruct;

void print_struct(MyStruct s);
void modify_struct(MyStruct* s);

#endif
```


#### ⚙️ c_src/my_struct.c (C 구현)
```c
#include <stdio.h>
#include "my_struct.h"

void print_struct(MyStruct s) {
    printf("C: x = %d, y = %f\n", s.x, s.y);
}

void modify_struct(MyStruct* s) {
    s->x += 10;
    s->y *= 2.0f;
}
```


#### 🦀 src/main.rs (Rust 코드)
```rust
#[repr(C)]
pub struct MyStruct {
    pub x: i32,
    pub y: f32,
}

extern "C" {
    fn print_struct(s: MyStruct);
    fn modify_struct(s: *mut MyStruct);
}

fn main() {
    let mut s = MyStruct { x: 5, y: 1.5 };
    
    unsafe {
        print_struct(s);
        modify_struct(&mut s);
        print_struct(s);
    }
}
```


#### ✅ 실행 결과
```
cargo run
```

#### 출력 예시:
```
C: x = 5, y = 1.500000
C: x = 15, y = 3.000000
```


### 🎯 요약: Rust ↔ C 구조체 FFI 연동 핵심 요소
| 구성 요소       | 설명 또는 역할                                      |
|----------------|-----------------------------------------------------|
| `#[repr(C)]`    | Rust 구조체의 메모리 레이아웃을 C와 호환되게 지정       |
| `extern "C"`    | C 함수 또는 구조체를 Rust에서 호출 가능하게 선언        |
| `build.rs`      | C 코드를 Rust 빌드 과정에 포함시키는 설정 스크립트       |
| `cc` 크레이트   | C 파일을 컴파일하고 Rust 바이너리에 정적으로 링크함     |
| `unsafe`        | 외부 함수 호출 시 Rust가 안전성을 보장하지 않기 때문에 필요 |



### 2. dll & so
C에서 만든 .dll 또는 .so 같은 외부 공유 라이브러리를 Rust에서 FFI로 불러오는 방식.  
Rust 프로젝트는 외부 라이브러리를 동적으로 링크해서 함수나 구조체를 사용할 수 있음.

#### 📁 디렉토리 구조 (DLL/Shared Library 방식)
```
rust_dll_ffi/
├── Cargo.toml
├── src/
│   └── main.rs
├── c_lib/
│   ├── mylib.h
│   └── mylib.c
```

c_lib 디렉토리는 C 코드와 헤더를 포함하고, .dll 또는 .so로 빌드됨


####  ⚙️ C 코드: c_lib/mylib.c
```c
#include <stdio.h>
#include "mylib.h"

void greet(const char* name) {
    printf("Hello, %s from C library!\n", name);
}

int add(int a, int b) {
    return a + b;
}
```


#### 📄 C 헤더: c_lib/mylib.h
```c
#ifndef MYLIB_H
#define MYLIB_H

void greet(const char* name);
int add(int a, int b);

#endif
```


#### 🛠️ C 라이브러리 빌드
Windows (DLL)
```
gcc -shared -o mylib.dll -fPIC mylib.c
```

Linux/macOS (SO)
```
gcc -shared -o libmylib.so -fPIC mylib.c
```

생성된 .dll 또는 .so 파일은 Rust 실행 파일과 같은 디렉토리에 위치


#### 🦀 Rust 코드: src/main.rs
```rust
use std::ffi::CString;

extern "C" {
    fn greet(name: *const i8);
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let name = CString::new("JungHwan").expect("CString failed");
    
    unsafe {
        greet(name.as_ptr());
        let result = add(10, 20);
        println!("Rust: 10 + 20 = {}", result);
    }
}
```


#### 🔗 라이브러리 연결: Cargo.toml 설정
Rust는 .dll 또는 .so를 자동으로 링크하지 않기 때문에, 실행 시 해당 파일이 같은 디렉토리에 있어야 함.  
#[link(name = "mylib")]를 쓰지 않아도 되고, extern "C" 선언만으로 충분.
만약 정적 링크를 원한다면 .a 또는 .lib 파일을 만들고 #[link(name = "mylib")]를 사용해야 함.


### ✅ 실행 방법
- Rust 프로젝트 빌드:
cargo build
- .dll 또는 .so 파일을 target/debug/ 또는 실행 디렉토리에 복사
- 실행:
```
cargo run
```


## 🎯 요약: Rust ↔ 외부 C 라이브러리 (.dll / .so) 연동

| 구성 요소       | 설명 또는 역할                                      |
|----------------|-----------------------------------------------------|
| `.dll` / `.so`  | 외부에서 컴파일된 C 공유 라이브러리 (동적 링크 대상)   |
| `extern "C"`    | Rust에서 외부 C 함수의 시그니처를 선언                |
| `CString`       | Rust 문자열을 C 스타일 포인터(`*const char`)로 변환   |
| `unsafe`        | 외부 함수 호출 시 Rust가 안전성을 보장하지 않기 때문에 필요 |
| `gcc -shared ...` | C 코드를 공유 라이브러리로 컴파일하는 명령어 (`.dll`, `.so`) |



#[link(name = "mylib")]는 Rust가 외부 라이브러리를 연결할 때 사용하는 속성.  
이때 "mylib"은 libmylib.so, mylib.dll, 또는 libmylib.dylib 같은 공유 라이브러리 파일 이름.   
그런데 이걸 Rust가 어디서 찾느냐가 핵심.

## 📍 라이브러리 위치 지정 방법
Rust는 기본적으로 시스템의 라이브러리 검색 경로 또는 명시적으로 지정한 경로에서 해당 파일을 찾으려고 함. 
경로를 지정하는 방법은 크게 두 가지:

### ✅ 1. build.rs에서 cargo:rustc-link-search 사용
```rust
fn main() {
    println!("cargo:rustc-link-search=native=./libs");
}
```

- ./libs는 libmylib.so 또는 mylib.dll이 위치한 디렉토리
- Rust는 이 경로를 컴파일 시 -L 옵션으로 넘겨서 라이브러리를 찾게 돼

### ✅ 2. .cargo/config.toml에서 설정
```
[build]
rustflags = ["-L", "native=./libs"]
```

- 프로젝트 루트에 .cargo/config.toml 파일을 만들고 위처럼 설정하면 돼
- 이 방식은 build.rs 없이도 경로를 지정할 수 있어

## 🧠 파일 이름 규칙
Rust는 플랫폼에 따라 자동으로 접두사와 확장자를 붙여서 찾음:
### 🎯 요약: Rust가 찾는 외부 라이브러리 파일 이름
| 플랫폼       | 기대하는 파일 이름     |
|--------------|-------------------------|
| Linux        | `libmylib.so`           |
| macOS        | `libmylib.dylib`        |
| Windows      | `mylib.dll`             |

즉, #[link(name = "mylib")]이면 Rust는 libmylib.so 또는 mylib.dll을 찾게 돼



### 🎯 #[link(name = "mylib")] 사용 시 라이브러리 위치 지정
| 구성 요소                  | 설명                                      |
|---------------------------|-------------------------------------------|
| `#[link(name = "mylib")]`  | Rust가 `libmylib.so` 또는 `mylib.dll`을 찾도록 지정 |
| `cargo:rustc-link-search` | `build.rs`에서 라이브러리 경로를 명시         |
| `.cargo/config.toml`      | 프로젝트 설정 파일에서 경로 지정 가능         |
| 파일 위치                 | `target/debug/`, `./libs/`, 또는 시스템 경로  |

라이브러리를 어디에 둘지 고민된다면, 가장 쉬운 방법은 Rust 실행 파일과 같은 디렉토리에 .dll 또는 .so 두면 됨. 


---

# 디렉토리 구조
Rust에서 외부 C 라이브러리 (.dll, .so)를 FFI로 연동하는 실제 프로젝트 구조.  
이 구조는 C 코드를 별도로 컴파일해서 공유 라이브러리로 만들고, Rust에서 그걸 동적으로 호출하는 방식.

## 📁 프로젝트 디렉토리 구조
```
rust_dll_ffi/
├── Cargo.toml
├── src/
│   └── main.rs
├── c_lib/
│   ├── mylib.h
│   └── mylib.c
├── libs/
│   └── libmylib.so   ← Linux/macOS용
│   └── mylib.dll     ← Windows용
```

c_lib/는 C 소스코드와 헤더
libs/는 컴파일된 공유 라이브러리 (.dll / .so)


## 🧰 Cargo.toml
```
[package]
name = "rust_dll_ffi"
version = "0.1.0"
edition = "2021"
```

외부 라이브러리를 사용할 때는 특별한 의존성 없이 extern "C"만으로 충분해


## ⚙️ C 코드: c_lib/mylib.c
```c
#include <stdio.h>
#include "mylib.h"

void greet(const char* name) {
    printf("Hello, %s from C!\n", name);
}

int add(int a, int b) {
    return a + b;
}
```


## 📄 C 헤더: c_lib/mylib.h
```c
#ifndef MYLIB_H
#define MYLIB_H

void greet(const char* name);
int add(int a, int b);

#endif
```


## 🛠️ C 라이브러리 빌드 명령어
Linux/macOS (.so)
```
gcc -shared -fPIC -o libs/libmylib.so c_lib/mylib.c
```

Windows (.dll)
```
gcc -shared -o libs/mylib.dll c_lib/mylib.c
```


## 🦀 Rust 코드: src/main.rs
```rust
use std::ffi::CString;

#[link(name = "mylib")]
extern "C" {
    fn greet(name: *const i8);
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let name = CString::new("JungHwan").expect("CString failed");

    unsafe {
        greet(name.as_ptr());
        let result = add(7, 8);
        println!("Rust: 7 + 8 = {}", result);
    }
}
```


## 🛠️ 라이브러리 경로 설정 (선택 사항)
Rust가 .dll 또는 .so를 찾을 수 있도록 경로를 지정할 수 있음:
build.rs (선택)
```rust
fn main() {
    println!("cargo:rustc-link-search=native=libs");
}
```

또는 실행 시 환경 변수로 지정:
```
LD_LIBRARY_PATH=./libs cargo run      # Linux/macOS
set PATH=%CD%\libs && cargo run       # Windows
```


## ✅ 실행 결과
```
Hello, JungHwan from C!
Rust: 7 + 8 = 15
```


## 🎯 요약: Rust ↔ 외부 C 라이브러리 (.dll / .so) 연동 구조
| 구성 요소         | 설명 또는 역할                                      |
|------------------|-----------------------------------------------------|
| `c_lib/`          | C 소스 및 헤더 파일을 저장하는 디렉토리               |
| `libs/`           | 컴파일된 공유 라이브러리 (`libmylib.so`, `mylib.dll`) 위치 |
| `#[link(name)]`   | Rust가 외부 라이브러리를 연결할 때 사용하는 속성         |
| `extern "C"`      | Rust에서 외부 C 함수의 시그니처를 선언하는 키워드        |
| `CString`         | Rust 문자열을 C 스타일 포인터(`*const char`)로 변환     |
| `unsafe`          | 외부 함수 호출 시 Rust가 안전성을 보장하지 않기 때문에 필요 |

---

