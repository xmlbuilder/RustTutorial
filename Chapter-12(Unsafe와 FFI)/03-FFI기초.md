# FFI Basic

FFI(Foreign Function Interface)는 Rust가 다른 언어(C, C++, Python 등)와 직접 상호작용할 수 있게 해주는 기능.  
Rust는 안전성을 중시하는 언어지만, 시스템 프로그래밍을 위해 외부 라이브러리와 연결할 수 있어야 하니까 FFI는 아주 중요한 역할.

## 🧠 FFI란?
Foreign Function Interface의 줄임말로, Rust 코드에서 외부 언어(C 등)의 함수를 호출하거나,  
반대로 외부에서 Rust 함수를 호출할 수 있게 해주는 인터페이스.


## 🔧 Rust에서 C 함수 호출하기
### 1. C 코드 준비
```rust
// example.c
#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
}

int add(int a, int b) {
    return a + b;
}
```

이걸 컴파일해서 공유 라이브러리로 만들어야 해:
```
gcc -shared -o libexample.so -fPIC example.c
```


### 2. Rust에서 C 함수 선언
```rust
#[link(name = "example")]
extern "C" {
    fn hello();
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        hello(); // C 함수 호출
        let result = add(5, 7);
        println!("5 + 7 = {}", result);
    }
}
```

- #[link(name = "example")]: libexample.so를 링크하라는 의미
- extern "C": C ABI(Application Binary Interface)를 따르겠다는 선언
- unsafe: Rust는 외부 함수의 안전성을 보장할 수 없기 때문에 반드시 unsafe 블록에서 호출해야 함

## 🧱 Rust → C로 함수 내보내기
```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

- #[no_mangle]: Rust가 함수 이름을 변경하지 않도록 함
- extern "C": C에서 호출할 수 있도록 ABI를 맞춤
이렇게 하면 C에서 rust_add를 직접 호출할 수 있음.

## 🧵 문자열과 구조체 다룰 때 주의점
- 문자열은 CString과 CStr를 사용해 변환해야 안전함
- 구조체는 #[repr(C)]를 붙여서 C와 메모리 레이아웃을 맞춰야 함
- 포인터는 반드시 null 체크, 유효성 검사, 메모리 소유권 관리가 필요

## 🛡️ 안전하게 FFI 쓰는 팁
- 외부 함수는 안전한 wrapper 함수로 감싸는 게 좋음
- bindgen을 사용하면 C 헤더 파일을 자동으로 Rust 바인딩으로 변환 가능
- 메모리 관리는 명확하게, 누수 없게!

## 🎯 요약: Rust의 FFI 사용 핵심
| 작업 예시               | 설명                                      |
|------------------------|-------------------------------------------|
| 외부 C 함수 선언        | `extern "C"`로 함수 시그니처 정의          |
| 외부 라이브러리 링크    | `#[link(name = "libname")]`로 연결         |
| Rust 함수 C에 노출      | `#[no_mangle] pub extern "C" fn ...` 사용 |
| 구조체 교환             | `#[repr(C)]`로 메모리 레이아웃 맞춤       |
| 문자열 교환             | `CString`, `CStr`로 안전하게 변환         |
| 호출 시 안전성 보장     | `unsafe` 블록에서만 외부 함수 호출 가능   |

FFI는 Rust의 안전성과 성능을 유지하면서도 외부 세계와 연결할 수 있는 강력한 도구. 

---

# Rust / C#

Rust와 C#은 서로 다른 세계처럼 보일 수 있지만, **FFI(Foreign Function Interface)**를 통해 아주 잘 연동할 수 있음.  
핵심은 Rust에서 C ABI를 따르는 방식으로 함수를 노출하고, C#에서 DllImport를 통해 그 함수를 호출하는 거야.

## 🔗 Rust ↔ C# 연동 개요
### ✅ Rust 쪽: C 스타일로 함수 노출
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- #[no_mangle]: 함수 이름을 그대로 유지
- extern "C": C ABI를 따름 → C#에서 호출 가능
이 코드를 .dll 또는 .so로 빌드해야 해:
```
cargo build --release
```

Windows라면 .dll, Linux는 .so, macOS는 .dylib이 생성돼.

## ✅ C# 쪽: DllImport로 Rust 함수 호출
```csharp
using System;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("your_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int add(int a, int b);

    static void Main()
    {
        int result = add(3, 4);
        Console.WriteLine($"3 + 4 = {result}");
    }
}
```

- "your_rust_lib"는 Rust에서 만든 DLL 이름 (확장자 없이)
- CallingConvention.Cdecl은 Rust의 기본 호출 방식

## 🧠 구조체나 enum도 가능할까?
가능해! 단, Rust 쪽에서 #[repr(C)]를 붙여서 메모리 레이아웃을 C와 호환되게 만들어야 함.  
예를 들어:
```rust
#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}
```

## C#에서는 이렇게 대응:
```rust
[StructLayout(LayoutKind.Sequential)]
public struct Point {
    public double x;
    public double y;
}
```


## 🛠️ 도구 추천
- csharp_binder는 Rust 코드를 자동으로 C# 바인딩으로 변환해주는 도구야
- rust-csharp-ffi 예제 프로젝트도 참고할 만해
- Rust와 .NET 9 통합 가이드는 성능 최적화까지 다뤄

## 🎯 요약: Rust ↔ C# FFI 연동 핵심 비교
| 항목               | Rust 측 구현                          | C# 측 호출 방식                        |
|--------------------|----------------------------------------|----------------------------------------|
| 함수 노출 방식      | `#[no_mangle] extern "C"`              | `[DllImport(...)]`                     |
| 구조체 호환         | `#[repr(C)] struct`                    | `[StructLayout(...)] struct`           |
| 빌드 결과물         | `.dll`, `.so`, `.dylib`                | 해당 플랫폼에 맞는 DLL 로딩            |
| 호출 안전성         | `unsafe` 블록에서 호출 필요            | `CallingConvention.Cdecl`로 ABI 지정   |


---


