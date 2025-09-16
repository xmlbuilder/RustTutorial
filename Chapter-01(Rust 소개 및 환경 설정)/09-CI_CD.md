# CI/CD

CI/CD와 GitHub Actions를 활용한 Rust 자동화는 개발자가 코드를 푸시하거나 PR을 만들 때,
자동으로 테스트하고 코드 스타일을 검사해주는 시스템.
즉, 사람이 직접 cargo test, cargo fmt, cargo clippy를 실행하지 않아도
GitHub이 알아서 검사해주는 로봇을 만드는 것이라고 생각하시면 됩니다.

## 🚀 CI/CD란?

| 용어       | 의미                                      | 역할 및 목적                                                  |
|------------|-------------------------------------------|---------------------------------------------------------------|
| CI         | Continuous Integration (지속적 통합)       | 코드 변경 시 자동으로 테스트 및 빌드를 수행하여 오류를 빠르게 발견 |
| CD         | Continuous Delivery 또는 Deployment       | 테스트 통과 후 자동으로 배포하거나 배포 가능한 상태로 유지         |
| GitHub Actions | GitHub에서 제공하는 자동화 도구         | CI/CD를 구현하기 위한 워크플로우 설정. `cargo test`, `fmt`, `clippy` 등을 자동 실행 |


### 💡 Rust에서는 보통 CI만 먼저 설정하고, CD는 나중에 필요할 때 추가합니다.

## 🤖 GitHub Actions란?
GitHub에서 제공하는 자동화 도구입니다.
.github/workflows/ 폴더에 설정 파일을 만들면,
코드가 변경될 때마다 자동으로 명령어를 실행해줍니다.

## 🧪 Rust에서 자동화하는 명령어들
| 명령어         | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| `cargo test`   | 테스트 코드 실행. 기능이 정상적으로 동작하는지 검증함                   |
| `cargo fmt`    | 코드 스타일 자동 정리. 들여쓰기, 줄바꿈 등 일관된 포맷 유지                |
| `cargo clippy` | 코드 품질 검사. 불필요하거나 위험한 코드 패턴을 경고하고 개선 제안함       |


## 🛠️ GitHub Actions 설정 예시
```
# .github/workflows/ci.yml

name: Rust CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Run cargo fmt
      run: cargo fmt --all -- --check

    - name: Run cargo clippy
      run: cargo clippy --all-targets --all-features -- -D warnings

    - name: Run cargo test
      run: cargo test
```

### ✅ 이 설정의 의미
- push나 pull_request가 발생하면 자동 실행됨
- Rust 설치 후 fmt, clippy, test를 순서대로 실행
- --check, -D warnings는 실패 시 CI가 중단되도록 설정

### 📁 위치 설명

.github/workflows 디렉토리는 GitHub Actions 설정 파일을 저장하는 곳인데, 
이건 로컬 컴퓨터에도 존재해야 합니다.
```
프로젝트 루트/
├── Cargo.toml
├── src/
├── .github/
│   └── workflows/
│       └── ci.yml  ← GitHub Actions 설정 파일
```
- 이 디렉토리는 프로젝트 루트 폴더에 직접 만들어야 해요.

### 💡 참고
- 이 디렉토리는 GitHub에 푸시되어야 동작합니다.
- 로컬에서 만든 후 git add .github → git commit → git push 하면 GitHub가 자동으로 인식해요.


## 🎯 왜 중요한가요?
- ✅ 실수 방지: 테스트 실패나 스타일 오류를 미리 잡아줌
- 🧠 협업 효율: 팀원 간 코드 품질을 자동으로 통일
- 🚀 배포 안정성: 코드가 항상 검증된 상태로 유지됨

---


