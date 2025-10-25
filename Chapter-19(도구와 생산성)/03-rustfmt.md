# `rustfmt` 소개 및 사용법
rustfmt는 Rust 코드 스타일을 자동으로 정리해주는 공식 포맷터입니다.  
일관된 코드 스타일을 유지하고 협업 시 가독성을 높이는 데 필수적인 도구.

🧰 rustfmt란?
- Rust 공식 포맷터로, 소스 코드를 자동으로 정리해줍니다.
- Rust 팀이 정의한 표준 스타일 가이드를 기반으로 작동합니다.
- cargo fmt 명령으로 프로젝트 전체를 포맷팅할 수 있습니다.

## 🚀 설치 방법
Rust를 설치하면 기본적으로 포함되어 있지만, 누락된 경우:
```
rustup component add rustfmt
```
- Rustup을 통해 설치된 Rust Toolchain에 rustfmt를 추가합니다.


## 🛠️ 사용 방법
### 1. 단일 파일 포맷팅
```
rustfmt main.rs
```

### 2. 프로젝트 전체 포맷팅 (권장)
```
cargo fmt
```

- Cargo.toml이 있는 루트 디렉토리에서 실행
- 모든 .rs 파일을 자동으로 포맷

### ⚙️ 설정 커스터마이징
`rustfmt.toml` 파일을 프로젝트 루트에 추가하면 포맷 스타일을 조정할 수 있음.

#### 예시:
```
max_width = 100
hard_tabs = false
use_small_heuristics = "Max"
```
- 다양한 옵션은 공식 rustfmt 설정 문서에서 확인 가능


## ✅ 실전에서 왜 중요한가?

| 목적                     | 설명 |
|--------------------------|------|
| 코드 스타일 일관성 유지  | 팀원 간 스타일 충돌 없이 동일한 포맷 유지 가능 |
| 리뷰 효율 향상           | 스타일 논쟁 없이 로직 중심으로 리뷰 가능 |
| 자동화 및 CI 통합        | `cargo fmt --check`로 포맷 검사를 자동화 가능 |
| 생산성 향상              | 수동 정리 없이 자동 포맷 → 개발 집중도 증가 |
| 오픈소스 기여 용이       | Rust 커뮤니티 표준 스타일을 따르므로 기여 시 호환성 높음 |

## rustfmt 핵심 요약
- Rust 공식 코드 포맷터
- `cargo fmt`로 전체 프로젝트 포맷 가능
- `rustfmt.toml`로 스타일 커스터마이징
- 협업, 리뷰, 자동화에 필수 도구

---

## 📍 rustfmt.toml 생성 위치
- 프로젝트 루트 디렉토리에 만들어야 합니다.
- 즉, Cargo.toml이 있는 폴더에 rustfmt.toml을 함께 두면 됩니다.
```
your-project/
├── Cargo.toml
├── rustfmt.toml   ← 여기에 생성!
├── src/
│   └── main.rs
```
- Rustfmt는 cargo fmt 실행 시 해당 디렉토리에서 rustfmt.toml을 자동으로 읽어 적용합니다.


## 🛠️ 생성 방법
- 프로젝트 루트로 이동:
```
cd your-project/
```

- 파일 생성:
```
touch rustfmt.toml
```

- 예시 설정 추가:
```
max_width = 100
hard_tabs = false
use_small_heuristics = "Max"
```


## ✅ 확인 방법
- cargo fmt 실행 후 스타일이 적용되는지 확인
- cargo fmt --check로 포맷이 맞는지 검사 가능

---


