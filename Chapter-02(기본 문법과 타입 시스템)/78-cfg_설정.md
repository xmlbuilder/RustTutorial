# cfg 설정

Rust에서 `#[cfg(...)]` 와 `cfg!(...)` 는 컴파일 조건부 설정을 위한 매우 강력한 기능입니다.  
이를 통해 `OS`, `빌드 모드(Debug/Release)`, `아키텍처`, `기능(feature)`, `환경 변수` 등에 따라 코드의 일부만 컴파일하거나 실행할 수 있습니다.


## 🧩 `#[cfg(...)]` vs `cfg!(...)` 차이

| 구문         | 평가 시점     | 사용 위치 | 예시                          |
|--------------|----------------|------------|-------------------------------|
| `#[cfg(...)]` | 컴파일 타임    | 속성(attribute) | `#[cfg(target_os = "linux")]` |
| `cfg!(...)`   | 런타임 (true/false) | 조건문 내부     | `if cfg!(debug_assertions)`   |


## 🖥️ OS 관련 조건

| 조건                      | 설명                          |
|---------------------------|-------------------------------|
| target_os = "linux"       | 리눅스 운영체제에서만 컴파일     |
| target_os = "windows"     | 윈도우 운영체제에서만 컴파일     |
| target_os = "macos"       | macOS 운영체제에서만 컴파일     |
| target_family = "unix"    | 유닉스 계열 OS (Linux, macOS 등) |
| target_env = "gnu"        | GNU 환경 (glibc 등)에서 컴파일  |

```rust
#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Windows 전용 코드");
}
```
```rust
pub fn on_get_platform_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        if cfg!(target_arch = "x86_64") {
            "windows_x64"
        } else {
            "windows_x86"
        }
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "macos")]
    {
        "osx"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "unknown"
    }
}
```

## 🧪 빌드 모드 (Debug / Release)

| 조건                    | 설명                         |
|-------------------------|------------------------------|
| debug_assertions        | 디버그 모드일 때 true         |
| not(debug_assertions)   | 릴리즈 모드일 때 true         |

```rust
if cfg!(debug_assertions) {
    println!("디버그 모드입니다");
}
```


## 🧬 아키텍처 조건

| 조건                         | 설명                             |
|------------------------------|----------------------------------|
| target_arch = "x86"          | 32비트 x86 아키텍처에서만 컴파일   |
| target_arch = "x86_64"       | 64비트 x86 아키텍처에서만 컴파일   |
| target_arch = "arm"          | ARM 아키텍처에서만 컴파일         |
| target_pointer_width = "64"  | 64비트 시스템에서만 컴파일        |


## 🧩 기능(feature) 조건
Cargo.toml에 정의한 feature에 따라 컴파일 분기 가능:
```rust
[features]
my_feature = []
```

```rust
#[cfg(feature = "my_feature")]
fn only_if_enabled() {
    println!("my_feature가 활성화됨");
}
```


## 🧪 예시: 조건별 함수 정의
```rust
#[cfg(target_os = "linux")]
fn platform() { println!("Linux"); }
```
```rust
#[cfg(target_os = "windows")]
fn platform() { println!("Windows"); }
```
```rust
fn main() {
    platform();

    if cfg!(debug_assertions) {
        println!("디버그 빌드입니다");
    }
}
```

## ✅ 요약 표

| 조건 종류         | 조건 예시                    | 설명                             |
|------------------|------------------------------|----------------------------------|
| 운영체제          | `target_os = "linux"`         | 리눅스에서만 컴파일               |
| 빌드 모드         | `debug_assertions`            | 디버그 모드일 때 true             |
| 아키텍처          | `target_arch = "x86_64"`      | 64비트 x86 아키텍처에서만 컴파일  |
| 기능(feature)     | `feature = "my_feature"`      | 해당 feature가 활성화될 때 컴파일 |
| 환경              | `target_env = "gnu"`          | GNU 환경(glibc 등)에서 컴파일     |


## 🧪 #[cfg(test)]란?
- 의미: 이 속성이 붙은 코드는 cargo test로 테스트를 실행할 때만 컴파일되고 실행됩니다.
- 용도: 테스트 전용 모듈, 함수, 헬퍼 등을 정의할 때 사용
  
### ✅ 예시
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```
- 위 코드는 cargo build에서는 포함되지 않고, cargo test에서만 컴파일됩니다.

## 🔍 관련 조건 요약
### 🧪 테스트 관련 조건 요약

| 조건            | 설명                                      |
|-----------------|-------------------------------------------|
| `#[cfg(test)]`  | 테스트 실행 시에만 컴파일되는 코드 블록 정의 |
| `#[test]`       | 테스트 함수로 인식되도록 지정               |
| `cfg!(test)`    | 런타임 분기: 테스트 빌드일 때 true 반환     |



### 🧩 cfg!(test) vs #[cfg(test)]

| 구문           | 평가 시점     | 사용 위치         | 예시                          |
|----------------|----------------|--------------------|-------------------------------|
| #[cfg(test)]   | 컴파일 타임    | 모듈, 함수, impl 등 | #[cfg(test)] mod tests        |
| cfg!(test)     | 런타임         | 조건문 내부         | if cfg!(test) { ... }         |



### 📦 테스트 모듈 구조 예시
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
```rust

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }
}
```

## 🧪 target_env 샘플 코드
```rust
#[cfg(target_env = "gnu")]
fn print_env() {
    println!("GNU 환경입니다 (glibc 기반)");
}
```
```rust
#[cfg(target_env = "musl")]
fn print_env() {
    println!("musl 환경입니다 (경량 libc)");
}
```
```rust
#[cfg(target_env = "msvc")]
fn print_env() {
    println!("MSVC 환경입니다 (Windows Visual C++)");
}
```
```rust
fn main() {
    print_env();
}
```


## 📦 Cargo 빌드 예시
- x86_64-unknown-linux-gnu → target_env = "gnu"
- x86_64-unknown-linux-musl → target_env = "musl"
- x86_64-pc-windows-msvc → target_env = "msvc"
```
cargo build --target x86_64-unknown-linux-gnu
```

## 🧩 Markdown 아스키 표 요약
### 🧪 target_env 조건 요약

| 조건                     | 설명                                 |
|--------------------------|--------------------------------------|
| target_env = "gnu"       | GNU 환경 (glibc 기반 리눅스)          |
| target_env = "musl"      | musl 환경 (경량 libc, 정적 링크에 유리) |
| target_env = "msvc"      | MSVC 환경 (Windows Visual C++ 컴파일러) |

---








