# WebAssembly
WebAssembly(WASM)는 웹 개발의 새로운 가능성을 열어주는 기술이고, Rust는 그 세계로 들어가는 아주 강력한 입구.  
초보자가 WebAssembly를 Rust로 시작하려면 환경 설정 → 프로젝트 생성 → 브라우저 연동이라는 흐름을 따라가면 됩니다.

## 🚀 WebAssembly + Rust 시작 가이드
### 1️⃣ 기본 개념 이해
- WebAssembly(WASM): 브라우저에서 실행 가능한 바이너리 포맷. 빠르고 안전함.
- Rust: WASM 타겟으로 컴파일하기에 적합한 언어. 성능과 안전성 모두 우수.
- 목표: Rust로 작성한 코드를  파일로 컴파일하고, HTML/JS에서 실행

### 2️⃣ 개발 환경 준비
| 도구/항목       | 설명                                                  | 설치 방법 예시                          |
|------------------|-------------------------------------------------------|------------------------------------------|
| Rust             | WASM 타겟으로 컴파일 가능한 시스템 프로그래밍 언어     | `curl https://sh.rustup.rs | sh`         |
| wasm-pack        | Rust 코드를 WASM 패키지로 빌드하고 JS와 연동 가능      | `cargo install wasm-pack`                |
| Node.js & npm    | JS 모듈 관리 및 웹 연동에 필요                         | [https://nodejs.org](https://nodejs.org) |
| 웹 서버 도구     | `.wasm` 파일을 브라우저에서 테스트할 때 필요           | `python -m http.server` 또는 `live-server` |
| VS Code (추천)   | Rust + WASM 개발에 적합한 편집기                       | [https://code.visualstudio.com](https://code.visualstudio.com) |


#### Rust 설치
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### wasm-pack 설치
```
cargo install wasm-pack
```


### 3️⃣ 프로젝트 생성
```
wasm-pack new hello-wasm
cd hello-wasm
```

- 기본 Rust 라이브러리 구조 생성됨
- 에 WASM으로 노출할 함수 작성

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 4️⃣ 빌드 및 웹 연동
```
wasm-pack build --target web
```

- 폴더에 과 JS 바인딩 파일 생성됨

#### HTML + JS 예시
```javascript
<script type="module">
  import init, { greet } from "./pkg/hello_wasm.js";

  async function run() {
    await init();
    console.log(greet("JungHwan")); // Hello, JungHwan!
  }

  run();
</script>
```

### 5️⃣ 실행
```
python -m http.server
```

- 브라우저에서  열면 Rust로 만든 함수가 실행됨!

## ⚠️ 초보자가 주의할 점

| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| `wasm-bindgen`      | Rust와 JavaScript 간의 바인딩을 위한 필수 라이브러리. `pub` 함수에 반드시 사용 |
| 브라우저 캐시       | `.wasm` 파일 변경 시 캐시 때문에 이전 버전이 실행될 수 있음 → 강력 새로고침 필요 |
| 디버깅 어려움       | `.wasm`은 바이너리이므로 JS처럼 디버깅이 쉽지 않음 → `console.log` 적극 활용 |
| 타입 제한           | WASM은 기본적으로 숫자와 문자열만 지원 → 복잡한 구조는 JS에서 처리하거나 직렬화 필요 |
| 빌드 타겟 구분      | `--target web`, `--target bundler`, `--target nodejs` 등 목적에 맞게 선택 필요 |
| 파일 경로 문제      | HTML에서 `.wasm` 파일 경로가 정확하지 않으면 로딩 실패 → 상대 경로 확인 필수 |


## 📚 추천 자료
- LogRocket의 Rust + WASM 튜토리얼 (https://blog.logrocket.com/getting-started-with-webassembly-and-rust/)
- MoldStud의 초보자용 WebAssembly 가이드 (https://moldstud.com/articles/p-how-to-get-started-with-webassembly-a-comprehensive-beginners-guide)

---
