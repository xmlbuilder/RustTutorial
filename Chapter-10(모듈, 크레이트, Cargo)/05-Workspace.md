# Workspace
Rust의 workspace는 여러 개의 크레이트(Crate)를 하나의 프로젝트처럼 관리할 수 있게 해주는 구조입니다.
복잡한 프로젝트를 모듈화하고, 빌드/테스트/의존성 관리를 효율적으로 하기 위한 핵심 기능이에요.

## 🧱 Rust Workspace란?
Workspace = 여러 개의 패키지를 하나의 단위로 묶어 관리하는 구조

- 하나의 루트 디렉토리에 여러 개의 member crate를 포함
- 공통된 Cargo.lock과 target/ 디렉토리를 공유
- cargo build, cargo test 등을 전체 workspace에 대해 한 번에 실행 가능

## 📁 기본 구조 예시

```
my_workspace/
├── Cargo.toml          ← workspace 루트 (virtual manifest)
├── crates/
│   ├── core_lib/
│   │   └── Cargo.toml  ← member crate
│   └── app/
│       └── Cargo.toml  ← member crate
```


루트 Cargo.toml에는 [workspace] 섹션만 있고, [package]는 없음 → virtual manifest

```
# my_workspace/Cargo.toml
[workspace]
members = [
    "crates/core_lib",
    "crates/app"
]
```


## 🧠 Rust Workspace 핵심 특징

| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| `cargo build`      | 모든 member crate를 한 번에 빌드                                     |
| `cargo test`       | 전체 workspace에 대해 테스트 실행                                     |
| `Cargo.lock`       | workspace 전체에서 의존성 버전을 통일                                 |
| `target/`          | 빌드 결과를 공유하여 캐시 효율화                                     |
| `path`             | crate 간 로컬 경로 참조 가능 (`{ path = "../other_crate" }`)         |



## 🧪 실전 예시
```
# crates/app/Cargo.toml
[dependencies]
core_lib = { path = "../core_lib" }
```

이렇게 하면 core_lib crate를 로컬 경로로 참조할 수 있어요.
Workspace 내부라면 별도 설정 없이도 잘 작동합니다.

## ⚠️ 주의사항
- 루트 Cargo.toml에 [package]가 없으면 virtual manifest로 간주됨
- 루트에 [package]가 있으면 해당 crate가 루트 패키지가 됨
- members는 상대 경로로 지정해야 하며, exclude로 특정 crate 제외 가능

## ✨ 언제 쓰면 좋을까?
- 프로젝트를 여러 crate로 나누고 싶을 때
- 라이브러리와 실행 파일을 함께 관리할 때
- 공통 의존성과 설정을 공유하고 싶을 때
- 오픈소스 프로젝트처럼 모듈이 많은 구조를 만들고 싶을 때

---

# Package를 묶어서 관리
Rust의 workspace는 여러 개의 package를 하나의 프로젝트처럼 묶어서 관리할 수 있습니다.
즉, 하나의 workspace 안에 여러 개의 독립적인 Cargo 패키지를 포함할 수 있어요.


## 🧠 핵심 개념 정리

| 개념         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| `package`    | 하나의 `Cargo.toml`을 가진 독립적인 Rust 프로젝트 (라이브러리 또는 바이너리) |
| `crate`      | 컴파일 단위. 하나의 package는 하나의 라이브러리 crate와 여러 바이너리 crate를 포함 가능 |
| `Cargo.toml` | Rust 프로젝트의 메타데이터 및 의존성을 정의하는 설정 파일             |
| `workspace`  | 여러 개의 package를 하나로 묶어 공통 빌드/테스트/의존성 관리를 수행     |
| `members`    | workspace에 포함된 package들의 경로 목록 (`Cargo.toml`의 `[workspace]` 섹션에 정의) |


## 📁 예시 구조
```
my_workspace/
├── Cargo.toml          ← workspace 루트 (virtual manifest)
├── core_lib/           ← package 1
│   └── Cargo.toml
├── app/                ← package 2
│   └── Cargo.toml
├── tools/gen_docs/     ← package 3
│   └── Cargo.toml
```

루트 Cargo.toml에는 [workspace] 섹션만 있고, 각 하위 디렉토리는 자기만의 package를 갖습니다.
```
# my_workspace/Cargo.toml
[workspace]
members = [
    "core_lib",
    "app",
    "tools/gen_docs"
]
```


## ✅ 요약
- Rust workspace는 여러 개의 package를 포함할 수 있다
- 각 package는 자기만의 Cargo.toml을 갖고 독립적으로 빌드 가능
- workspace 루트에서 cargo build, cargo test 등을 실행하면 전체 package가 함께 처리됨
- 각 package는 서로를 path로 참조할 수 있어 모듈화와 재사용성이 뛰어남

--- 

# 여러개의 exe를 가짐

workspace 구조를 활용하면 **여러 개의 독립적인 실행 파일(.exe)**을 가질 수 있습니다.
각 실행 파일은 자기만의 package로 구성되며, 서로 다른 기능이나 목적을 가진 프로그램으로 동작할 수 있어요.

## 🧱 구조 예시: 여러 실행 파일을 가진 workspace
```
my_workspace/
├── Cargo.toml              ← workspace 루트 (virtual manifest)
├── tools/
│   ├── image_converter/    ← 실행 파일 1
│   │   └── Cargo.toml
│   ├── log_parser/         ← 실행 파일 2
│   │   └── Cargo.toml
│   └── cli_helper/         ← 실행 파일 3
│       └── Cargo.toml
```

루트 Cargo.toml:
```
[workspace]
members = [
    "tools/image_converter",
    "tools/log_parser",
    "tools/cli_helper"
]
```

각 하위 디렉토리는 독립적인 binary crate로서 .exe를 생성합니다.

## ✅ 빌드 결과
```
cargo build --release
```

실행 파일들이 다음 경로에 생성됩니다:
```
target/release/image_converter.exe
target/release/log_parser.exe
target/release/cli_helper.exe
```


## 🧠 Rust Workspace 장점

| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| 독립 실행 파일    | 여러 개의 binary crate를 독립적으로 실행 가능 (`app`, `tool`, `cli` 등) |
| 모듈화 구조       | 기능별로 crate를 나누어 유지보수와 테스트가 쉬움                     |
| 공통 라이브러리   | `core_lib` 같은 crate를 만들어 여러 실행 파일에서 공유 가능           |
| 빌드 효율화       | `target/` 디렉토리 공유로 빌드 캐시가 통합되어 빠름                   |
| 의존성 통일       | 하나의 `Cargo.lock`으로 전체 workspace의 버전 충돌 방지               |
| 테스트 통합       | `cargo test`로 전체 crate의 테스트를 한 번에 실행 가능                |



## ✨ 실무 활용 예
- core_lib: 공통 로직
- server: 백엔드 실행 파일
- client: CLI 도구
- admin_tool: 관리자 전용 유틸리티
이렇게 나누면 유지보수도 쉬워지고, 각 기능을 독립적으로 개발할 수 있음.


