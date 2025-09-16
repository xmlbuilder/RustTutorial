# 📦 Rust의 Crate란?
Crate는 Rust에서 컴파일 가능한 코드의 최소 단위입니다. 크게 두 가지 종류가 있어요:
| 구분             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| Crate 정의        | Rust에서 컴파일 가능한 **최소 단위**의 패키지                         |
| 종류              | - **Binary Crate**: 실행 파일 생성 (`main.rs`) <br> - **Library Crate**: 라이브러리 제공 (`lib.rs`) |
| 생성 명령어       | - Binary: `cargo new <프로젝트명>` <br> - Library: `cargo new --lib <라이브러리명>` |
| 엔트리 포인트     | - Binary: `src/main.rs` <br> - Library: `src/lib.rs`                 |
| 구성 파일         | `Cargo.toml` (의존성 및 메타데이터 관리)                             |
| 빌드 도구         | `cargo build`, `cargo run`, `cargo test` 등                          |
| 외부 배포         | [crates.io](https://crates.io)를 통해 공유 및 설치 가능               |
| 내부 구조         | 모듈(`mod`)을 통해 코드 조직화 가능                                   |
| C++ 대응 개념     | - Binary: 실행 파일 (`main.cpp`) <br> - Library: 정적/동적 라이브러리 (`.lib`, `.dll`, `.so`) |

## 🔧 생성 방법
- Binary Crate: cargo new my_project
- Library Crate: cargo new --lib my_library
각 Crate는 Cargo.toml 설정 파일과 src 디렉토리를 포함하며, Cargo가 빌드와 의존성 관리를 자동으로 처리해줍니다.
Crate는 하나의 프로젝트 단위이며, 여러 모듈을 포함할 수 있어요. 모듈은 Crate 내부의 코드 조직 구조입니다.


## 🧩 C++에서 Crate에 대응되는 개념은?
C++에는 Rust의 Crate와 정확히 일치하는 개념은 없지만, 몇 가지 유사한 구조가 있어요:
| Rust 개념         | C++ 대응 개념                     | 설명                                                                 |
|------------------|----------------------------------|----------------------------------------------------------------------|
| Crate            | 프로젝트 / 라이브러리             | C++에서는 프로젝트 단위로 관리되며, 라이브러리는 `.lib`, `.dll`, `.so` 등으로 제공 |
| Binary Crate     | 실행 파일 (Executable)            | `main()` 함수가 포함된 `.cpp` 파일을 컴파일하여 생성된 실행 파일           |
| Library Crate    | 정적/동적 라이브러리              | 다른 프로젝트에서 링크하여 사용하는 `.lib`, `.dll`, `.so` 파일             |
| Cargo.toml       | CMakeLists.txt / Makefile         | 빌드 설정 및 의존성 관리 파일                                          |
| Cargo            | CMake / Make / vcpkg / Conan      | 빌드 및 패키지 관리 도구                                              |
| Crates.io        | GitHub / vcpkg / Conan repository | 외부 라이브러리 배포 및 설치 플랫폼                                    |
| 모듈 (`mod`)     | 네임스페이스 / 헤더 파일 구조     | `namespace`, `.h` / `.cpp` 파일로 코드 조직화                          |
| `use` 키워드     | `#include` / `using namespace`    | 외부 모듈 또는 네임스페이스를 가져오는 방식                            |


## 🔍 차이점 요약
- Rust는 Cargo를 통해 Crate 생성, 빌드, 테스트, 배포까지 통합 관리합니다.
- C++은 Make/CMake + 외부 패키지 매니저를 조합해서 비슷한 기능을 구현해야 합니다.
- Rust Crate는 의존성과 버전 관리가 명확하고, 보안 감사도 쉬운 구조입니다.

## 🧠 비유로 이해하기
- Rust Crate = 📦 하나의 깔끔하게 포장된 상자 (코드 + 설정 + 의존성)
- C++ 프로젝트 = 🧰 여러 도구와 설정이 흩어져 있는 공구함

---

## C++ 비교 하기 (추가)
Rust의 crate는 C++의 **static/dynamic library(.lib/.dll/.so)**처럼 독립적인 모듈 단위로 보면 딱 맞습니다.
둘 다 “기능을 묶어서 재사용 가능한 단위로 만든다”는 철학은 같지만, Rust의 crate는 더 구조적이고 안전하게 설계돼 있어요.

## 🧱 비교 요약: C++ 라이브러리 vs Rust 크레이트
| 비교 항목 | C++ 라이브러리 (.lib/.dll/.so) | Rust 크레이트 (`crate`)                     |
|---------------------------------------------|-------------------------------|---------------------------------------------|
| 모듈 구조                                   | 헤더 + 구현 파일              | `src/lib.rs`, `src/main.rs`                |
| 빌드 설정                                   | `Makefile`, `CMake`           | `Cargo.toml`                                |
| 버전 관리                                   | 수동 관리                     | `semver` 기반 자동 관리                    |
| 배포 방식                                   | 수동 배포 또는 바이너리       | `cargo publish`로 crates.io에 배포         |
| 메모리 안전성                               | 수동 관리                     | `ownership`, `borrow checker`로 보장       |
| 테스트 및 문서화                            | 외부 도구 필요                | `cargo test`, `cargo doc` 내장             |

## 🧠 크레이트를 독립 모듈로 보는 이유
- 기능 단위로 캡슐화: 예를 들어 matrix, vector, geometry 등을 각각 crate로 분리 가능
- 재사용 가능: 다른 프로젝트에서 matrix = "0.1"처럼 바로 가져다 쓸 수 있음
- 버전 관리와 의존성 추적이 자동: C++보다 훨씬 덜 번거롭고 안정적

### ✅ 예시: Matrix 크레이트 만들기
```
cargo new matrix --lib
```

# matrix/Cargo.toml
```
[package]
name = "matrix"
version = "0.1.0"
edition = "2021"

[dependencies]
```

```rust
// matrix/src/lib.rs
pub mod matrix2;
pub mod matrix3;
pub mod matrix4;
```

이렇게 하면 matrix는 완전히 독립적인 모듈이 되고,
다른 프로젝트에서 
```rust
use matrix::matrix3::determinant3;
```
처럼 가져다 쓸 수 있음.

---


