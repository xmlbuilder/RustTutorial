# rust-analyzer
rust-analyzer는 Rust 개발을 위한 고급 IDE 기능을 제공하는 공식 언어 서버입니다.  
코드 자동 완성, 정의로 이동, 실시간 오류 표시 등 개발 생산성을 크게 향상시켜줍니다.

## 🧠 rust-analyzer란?
- Rust용 Language Server Protocol(LSP) 구현체로, 다양한 코드 편집기에서 IDE 수준의 기능을 제공합니다.
- Rust 프로젝트를 분석해 정적 검사, 코드 탐색, 리팩토링 지원을 수행합니다.
- VS Code, Vim, Emacs, Zed 등 주요 에디터에서 사용 가능.

## 🚀 rust-analyzer 주요 기능

| 기능                     | 설명 |
|--------------------------|------|
| 자동 완성                | 변수, 함수, 타입 등을 실시간으로 추천해 빠른 코딩 지원 |
| 정의로 이동              | 심볼 클릭 시 해당 정의 위치로 즉시 이동 (`Go to Definition`) |
| 타입 및 문서 보기        | 마우스 오버 시 타입 정보와 문서 주석을 팝업으로 표시 |
| 실시간 오류 및 경고 표시 | 컴파일 전에도 코드 오류나 경고를 즉시 확인 가능 |
| 리팩토링 지원            | 변수 이름 변경, 코드 정리 등 자동화된 리팩토링 기능 제공 |
| `rustfmt` 연동           | 코드 저장 시 자동 포맷 적용 가능 (`cargo fmt` 기반) |


## 🛠️ 설치 및 설정
### ✅ VS Code에서 설치
- VS Code 열기
- Extensions 탭에서 rust-analyzer 검색
- 설치 후 자동 활성화
- rust-analyzer는 rustup으로 설치된 Rust 프로젝트를 자동 인식합니다.

### ✅ 다른 에디터에서 사용
- Vim/Neovim: coc.nvim, nvim-lspconfig 등과 연동
- Emacs: lsp-mode 또는 eglot 사용
- Zed: 기본 지원

### ⚙️ 설정 팁 (settings.json 예시)
```
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.inlayHints.enable": true,
  "rust-analyzer.cargo.allFeatures": true
}
```
- clippy를 통해 저장 시 코드 품질 검사도 함께 수행 가능

## ✅ rust-analyzer 실전 장점

| 항목                     | 설명 |
|--------------------------|------|
| 개발 생산성 향상         | 자동 완성, 오류 표시로 빠른 개발 가능 |
| 코드 품질 향상           | 실시간 검사 및 리팩토링 지원 |
| 협업 효율 증가           | 일관된 코드 탐색과 문서 확인 가능 |
| 다양한 에디터 호환       | VS Code 외에도 Vim, Emacs 등에서 사용 가능 |
| Rust 생태계 통합         | `cargo`, `clippy`, `rustfmt` 등과 연동 가능 |



# RustRover 셋팅 가이드
JetBrains에서 만든 Rust 전용 IDE로, 대부분의 개발 환경이 자동으로 설정되기 때문에 따로 복잡한 셋팅이 필요 없음.

## ✅ RustRover에서 기본적으로 제공되는 기능

| 기능 항목                  | 설명 |
|----------------------------|------|
| `rust-analyzer`            | 내장된 언어 서버로 자동 완성, 오류 표시, 타입 추론 등 IDE 기능 제공 |
| `cargo`, `rustfmt`, `clippy` | 프로젝트 열면 자동으로 감지 및 연동 → 빌드, 포맷, 린트 모두 지원 |
| 코드 분석 및 탐색          | `rust-analyzer` 기반으로 정의로 이동, 문서 보기, 리팩토링 가능 |
| 테스트 및 실행 지원        | `cargo test`, `cargo run`을 GUI에서 실행 가능 |
| 자동 포맷 적용             | 저장 시 `rustfmt` 실행으로 코드 스타일 자동 정리 |


## ⚙️ 확인만 하면 좋은 설정
RustRover는 대부분 자동이지만, 아래 항목은 한 번 확인해두면 좋아요:
- Rust toolchain 경로: Settings > Languages & Frameworks > Rust
- rustup으로 설치된 경로가 자동 인식됨
- rustfmt 자동 실행 여부: Settings > Rust > Rustfmt
- 저장 시 포맷 적용 여부 설정 가능
- clippy 검사 활성화: Settings > Rust > External Linters
- 코드 품질 검사 도구

## RustRover 셋팅 요약
- `rust-analyzer`, `rustfmt`, `clippy` 모두 자동 연동
- 별도 설치나 설정 없이 바로 사용 가능
- 설정 메뉴에서 포맷, 검사, 툴체인 경로만 확인하면 끝

---




