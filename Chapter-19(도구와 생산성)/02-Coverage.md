## 🧪 Rust Coverage Tool이 하는 일
| 기능               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 코드 실행 추적      | 테스트 중 실제 실행된 코드 라인을 추적함                              |
| 커버리지 분석       | 전체 코드 중 테스트된 비율을 계산함                                   |
| 미커버리지 식별     | 테스트되지 않은 코드 영역을 시각적으로 표시함                         |
| 리포트 생성         | HTML, lcov 등 다양한 형식으로 커버리지 리포트 생성 가능               |
| CI/CD 통합          | 자동화된 테스트 파이프라인에 통합하여 품질 관리에 활용 가능            |


## 🔧 대표적인 Rust Coverage 툴
| 툴 이름               | 특징                                                                 |
|------------------------|----------------------------------------------------------------------|
| `cargo tarpaulin`      | Rust 전용 커버리지 툴. `cargo`와 통합되어 사용 간편. Linux/x86_64 지원. |
| `kcov`                 | 다양한 언어 지원. Rust 바이너리 실행 후 커버리지 수집. HTML 리포트 생성. |
| `llvm-source-based`    | Rust 컴파일러가 직접 커버리지 정보 생성. 높은 정확도. `-Z instrument-coverage` 사용. |

예: 
```
cargo tarpaulin --out Html → HTML 커버리지 리포트 생성
```

## ✅ 왜 중요할까?
- 테스트 누락 방지: 중요한 로직이 테스트되지 않았을 경우 빠르게 파악 가능
- 품질 향상: 커버리지를 높이면 버그 가능성이 줄어듦
- 리팩토링 안전성 확보: 테스트가 잘 덮고 있는지 확인하고 안심하고 수정 가능

---

# tarpaulin 설치

cargo tarpaulin은 Rust에 기본 내장된 도구가 아님.
따라서 직접 설치해줘야 사용 가능합니다.

## 🔧 설치 방법
```
cargo install cargo-tarpaulin
```

- 이 명령어는 cargo를 통해 tarpaulin을 설치합니다
- 설치 후에는 cargo tarpaulin 명령어로 커버리지 분석 가능

## 🧪 사용 예시
```
cargo tarpaulin --out Html
```

- 테스트를 실행하면서 커버리지 측정
- 결과를 HTML 리포트로 출력

## ✅ 참고 사항

| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| 플랫폼 제한       | Linux/x86_64에서 가장 안정적으로 동작함                              |
| Windows 지원      | 제한적이며 일부 기능은 불안정하거나 미지원일 수 있음                  |
| 설치 필요         | Rust에 내장되어 있지 않으며 `cargo install cargo-tarpaulin`으로 설치 필요 |
| 실행 방식         | 테스트 실행과 동시에 커버리지 측정 (`cargo tarpaulin`)                |
| 리포트 출력       | `--out Html` 옵션으로 HTML 형식 리포트 생성 가능                      |

혹시 macOS나 Windows에서 쓰고 계신가요? 그럴 경우엔 llvm-source-based coverage 방식이 더 안정적.

--- 

# LLVM source-based coverage

Rust의 LLVM source-based coverage는 cargo tarpaulin처럼 별도 툴을 설치하는 방식이 아니라,
Rust 컴파일러 자체에 내장된 기능이에요. 다만 몇 가지 설정이 필요.

## 🧰 설치 없이 사용하는 방식

LLVM 기반 커버리지는 nightly Rust에서만 작동합니다. 
먼저 nightly toolchain을 설치:
```
rustup install nightly
rustup default nightly
```


### ⚙️ 커버리지 활성화 방법
- 컴파일 시 커버리지 instrumentation 추가
```
RUSTFLAGS="-Zinstrument-coverage" cargo test
```

- 테스트 실행 후 커버리지 데이터 생성
```
LLVM_PROFILE_FILE="your_binary-%p-%m.profraw" cargo test
```

- 리포트 생성 (llvm-tools 설치 필요)
```
cargo install llvm-tools-preview
```

- 리포트 생성 도구 실행
```
llvm-profdata merge -sparse your_binary-*.profraw -o merged.profdata
llvm-cov show ./target/debug/your_binary \
  --instr-profile=merged.profdata \
  --format=html \
  --output-dir=coverage
```


## ✅ 요약 테이블
| 단계               | 명령어 또는 설명                                      |
|--------------------|-------------------------------------------------------|
| nightly 설치        | `rustup install nightly`                              |
| 커버리지 컴파일     | `RUSTFLAGS="-Zinstrument-coverage" cargo test`        |
| 프로파일 생성       | `LLVM_PROFILE_FILE="..." cargo test`                  |
| 도구 설치           | `cargo install llvm-tools-preview`                    |
| 리포트 생성         | `llvm-profdata`, `llvm-cov show`로 HTML 출력 가능     |


---

## 🔍 역할 분해
```
LLVM_PROFILE_FILE="your_binary-%p-%m.profraw" cargo test
```

- LLVM_PROFILE_FILE=... → 환경 변수 설정
이건 LLVM 커버리지 데이터를 저장할 파일명을 지정하는 설정이에요
- cargo test → 테스트 실행
Rust 테스트를 실행하면서 커버리지 데이터를 수집함
즉, 이 명령어는 커버리지 데이터를 저장할 파일명을 설정하면서 테스트를 실행하는 것이에요.

## 🧠 왜 이렇게 쓰는가?
LLVM 커버리지는 테스트 실행 중에 **프로파일 데이터(.profraw)**를 생성해요.
이 데이터를 나중에 llvm-profdata, llvm-cov로 분석해서 리포트를 만들죠.
파일명에 %p와 %m을 쓰는 이유는:
| 코드 | 의미                          |
|------|-------------------------------|
| %p   | 프로세스 ID (Process ID)      |
| %m   | 모듈 이름 (Module name)       |

→ 여러 테스트가 동시에 실행될 때 파일명이 겹치지 않도록 해주는 안전장치예요.

## ✅ 요약
| 구성 요소                  | 역할                                      |
|----------------------------|-------------------------------------------|
| `LLVM_PROFILE_FILE=...`    | 커버리지 데이터 저장 경로 설정             |
| `cargo test`               | 테스트 실행 + 커버리지 수집               |
| `%p`, `%m`                 | 파일명 중복 방지용 변수                   |

---

## 🔍 예시
```
LLVM_PROFILE_FILE="myproj-%p-%m.profraw" cargo test
```
테스트 실행 중에 생성되는 파일 이름은 예를 들어:
myproj-12345-mycrate.profraw

- 12345는 실행 중인 프로세스의 ID
- mycrate는 실행된 바이너리 또는 crate 이름
이렇게 하면 병렬 테스트나 여러 crate 테스트 시에도 파일 이름이 겹치지 않도록 안전하게 분리할 수 있음.

---




