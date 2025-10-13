# Crate 구조
Rust에서 crate는 컴파일 단위이자 코드의 논리적 묶음입니다.  
쉽게 말해, C++의 static/dynamic 라이브러리나 실행 파일처럼 하나의 독립된 컴파일 결과물을 만드는 단위.  
아래에 crate 생성 방법과 구조를 정리.

## 🛠️ Crate 생성 방법
Rust에서는 cargo를 사용해 crate를 생성합니다.  
두 가지 유형:

### 🧱 Crate 생성 구조 요약
| 유형             | 생성 명령어                  | 주요 파일       | 설명                                  |
|------------------|------------------------------|------------------|---------------------------------------|
| 바이너리 크레이트 | `cargo new myapp --bin`      | `main.rs`        | 실행 가능한 프로그램. `fn main()` 포함 |
| 라이브러리 크레이트 | `cargo new mylib --lib`      | `lib.rs`         | 다른 코드에서 사용할 수 있는 라이브러리 |


## 📦 Crate 기본 구조
```
mycrate/
├── Cargo.toml          # 패키지 메타데이터 및 의존성 정의
└── src/
    ├── main.rs         # 바이너리 크레이트의 루트 (엔트리포인트)
    └── lib.rs          # 라이브러리 크레이트의 루트 (공용 API 정의)
```

- Cargo.toml: crate 이름, 버전, 의존성, 빌드 설정 등을 정의
- src/main.rs: fn main()이 포함된 실행 파일의 진입점
- src/lib.rs: pub fn 등 외부에 노출할 API 정의

## 🧠 Crate 내부 구조 확장
Rust는 모듈 기반 구조를 통해 crate를 확장할 수 있어요:
```
src/
├── lib.rs
├── utils.rs           # 모듈 파일
└── network/
    └── mod.rs         # 서브모듈 디렉터리
```

```rust
// lib.rs
mod utils;
pub mod network;
```

- mod utils; → utils.rs 파일을 모듈로 포함
- pub mod network; → network/mod.rs를 외부에 공개

## 🚀 Crate 배포 흐름
- cargo build → crate 컴파일
- cargo test → 테스트 실행
- cargo doc --open → 문서 자동 생성
- cargo publish → crates.io에 배포

## 🔍 참고 개념
| 개념             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| **패키지 (Package)** | 하나 이상의 crate를 포함하는 Cargo 프로젝트. `Cargo.toml`로 관리됨       |
| **크레이트 (Crate)** | 컴파일 단위. 바이너리 또는 라이브러리 형태로 존재함                     |
| **모듈 (Module)**   | crate 내부의 코드 조직화 단위. `mod`, `pub mod`로 선언                   |
| **워크스페이스 (Workspace)** | 여러 crate를 하나의 프로젝트로 묶어 관리. 대규모 시스템에 적합         |

## 요약하자면:
Crate는 Rust의 컴파일 단위이며, cargo new로 생성하고 Cargo.toml과 src/ 구조로 관리 됩니다.
바이너리와 라이브러리 크레이트를 함께 만들 수도 있고, 모듈로 확장해서 대규모 시스템도 구성할 수 있음.

---



