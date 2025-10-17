# **무엇(데이터)인가** 에서 **무엇을 할 줄 아는가(트레이트)** 로 이동.

## 실전 연습 루트 (2주 분량 스스로 학습 플랜)
### Day 1–2
- 기존 모듈 1개 골라 동사 뽑기 → 트레이트 선언 → ctx 설계만 해보기(코드 미이동).
- 컴파일 안 해도 좋으니 인터페이스 스케치 우선.
### Day 3–4
- 200~400줄짜리 기능을 파이프라인화(로깅·검증·핸들 순으로 3단).
### Day 5–7
- 뷰–모델 한 군데에 시그널/구독 삽입(수동 호출 제거).
- 회귀 테스트로 렌더 호출 누락이 사라졌는지 확인.

### Week 2
- 공간 트리/픽킹/카메라 중 하나를 트레이트 단위로 분할하고 벤치.
- “구성 교체” 시나리오(예: 여러 필터/로거 조합)로 유연성 체감.

## 문서화 템플릿(팀 공유용)
- 목표: “X 모듈을 행위 중심으로 전환하여 ___을 가능하게 한다.”
- 동사 목록: Log, Filter, Handle, Render, Validate…
- 트레이트 사양
```rust
trait Xxx { fn op(&self, input: &A, ctx: &mut Ctx) -> Result<B>; }
```
- 입력/출력/부수효과(어떤 필드를 바꾸는가)를 글머리표로.
- 컨텍스트 정의: 필드, 스레딩, 오류 정책.
- 조립 예: 기본/디버그/테스트 파이프라인 3종.
- 마이그레이션: 바뀐 public API, 삭제된 코드, 대체 지침.

## 테스트: 단위/통합/성능 기준.





## 🧪 Day 1–2: 동사 뽑기 → 트레이트 선언 → ctx 설계
### 🎯 목표
- 기존 기능에서 동사 추출
- 트레이트로 인터페이스 스케치
- Ctx 설계 (부수효과 저장소)
### ✍️ 예시
```rust
// 동사: Log, Validate, Handle

trait Logger {
    fn log(&self, msg: &str, ctx: &mut Ctx);
}

trait Validator {
    fn validate(&self, input: &str, ctx: &mut Ctx) -> bool;
}

trait Handler {
    fn handle(&self, input: &str, ctx: &mut Ctx);
}

struct Ctx {
    logs: Vec<String>,
    errors: Vec<String>,
    output: Vec<String>,
}
```

- ✅ 컴파일 안 돼도 괜찮아요. 인터페이스만 먼저 잡기
- ✅ Ctx는 모든 사이드 이펙트의 중심


## 🧪 Day 3–4: 기능을 파이프라인화
### 🎯 목표
- 3단 파이프라인 구성: Logger → Validator → Handler
- 각 구현은 순수하게, 조립은 외부에서
### ✍️ 예시
```rust
struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, msg: &str, ctx: &mut Ctx) {
        ctx.logs.push(format!("LOG: {}", msg));
    }
}

struct LengthValidator;
impl Validator for LengthValidator {
    fn validate(&self, input: &str, ctx: &mut Ctx) -> bool {
        let valid = input.len() < 20;
        if !valid {
            ctx.errors.push("Too long".to_string());
        }
        valid
    }
}

struct EchoHandler;
impl Handler for EchoHandler {
    fn handle(&self, input: &str, ctx: &mut Ctx) {
        ctx.output.push(format!("Handled: {}", input));
    }
}


fn run_pipeline(
    logger: &dyn Logger,
    validator: &dyn Validator,
    handler: &dyn Handler,
    input: &str,
    ctx: &mut Ctx,
) {
    logger.log(input, ctx);
    if validator.validate(input, ctx) {
        handler.handle(input, ctx);
    }
}
```

- ✅ 각 동사는 트레이트로 분리
- ✅ 조립은 외부에서 → 교체/테스트 쉬움


## 🧪 Day 5–7: 시그널/구독 삽입
### 🎯 목표
- 수동 호출 제거 → 시그널 기반 호출
- 렌더 누락 방지
### ✍️ 예시
```rust
struct Signal<T> {
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Signal<T> {
    fn subscribe<F: Fn(&T) + 'static>(&mut self, f: F) {
        self.subscribers.push(Box::new(f));
    }

    fn emit(&self, value: T) {
        for sub in &self.subscribers {
            sub(&value);
        }
    }
}


fn main() {
    let mut render_signal = Signal::<String> { subscribers: vec![] };

    render_signal.subscribe(|msg| {
        println!("Rendering: {}", msg);
    });

    render_signal.emit("Hello".to_string());
}
```

- ✅ 렌더 호출을 수동 호출에서 해방
- ✅ 테스트로 누락 여부 확인 가능


## 🧪 Week 2: 공간 트리/픽킹/카메라 → 트레이트 분할 + 벤치
### 🎯 목표
- 기능을 트레이트 단위로 분할
- 구성 교체로 유연성 체감
### ✍️ 예시: 카메라
```rust
trait CameraControl {
    fn zoom(&mut self, factor: f32, ctx: &mut Ctx);
    fn pan(&mut self, dx: f32, dy: f32, ctx: &mut Ctx);
}

struct OrbitCamera;
impl CameraControl for OrbitCamera {
    fn zoom(&mut self, factor: f32, ctx: &mut Ctx) {
        ctx.logs.push(format!("Zoom by {}", factor));
    }
    fn pan(&mut self, dx: f32, dy: f32, ctx: &mut Ctx) {
        ctx.logs.push(format!("Pan by {}, {}", dx, dy));
    }
}

```
- ✅ CameraControl 트레이트로 기능 분리
- ✅ OrbitCamera, FreeCamera 등 교체 가능


## 📄 문서화 템플릿 예시
### 목표
“TextProcessor 모듈을 행위 중심으로 전환하여 로깅/검증/처리를 유연하게 조합할 수 있게 한다.”

### 동사 목록
- Log
- Validate
- Handle

### 트레이트 사양
```rust
trait Handler {
    fn handle(&self, input: &str, ctx: &mut Ctx);
}
```

### 입력/출력/부수효과
- 입력: &str
- 출력: 없음
- 부수효과: ctx.output에 결과 저장
### 컨텍스트 정의
```rust
struct Ctx {
    logs: Vec<String>,
    errors: Vec<String>,
    output: Vec<String>,
}
```

### 조립 예
- 기본: ConsoleLogger + LengthValidator + EchoHandler
- 디버그: VerboseLogger + AlwaysPassValidator + NoopHandler
- 테스트: MockLogger + RegexValidator + TestHandler
### 마이그레이션
- 삭제: process_text() 함수
- 대체: run_pipeline(logger, validator, handler, input, ctx)

---

## 🧪 테스트 기준
- ✅ 단위: 각 트레이트 구현에 대해 입력 → ctx 변화 확인
- ✅ 통합: 파이프라인 조합 결과 확인
- ✅ 성능: `cargo bench`로 트레이트 교체 시 영향 측정

---

# 실전 예시

## 🧩 프로젝트 이름: textflow
텍스트 처리 파이프라인 — 로그, 검증, 처리, 렌더링을 조합 가능한 구조로 구현


### 🎯 목표
- 다양한 텍스트 입력을 받아
- 로깅 → 검증 → 처리 → 렌더링 단계를 거쳐
- 결과를 Ctx에 저장하는 파이프라인 기반 처리기를 만든다.

### 📦 프로젝트 구조
```
textflow/
├── src/
│   ├── main.rs
│   ├── ctx.rs
│   ├── traits/
│   │   ├── logger.rs
│   │   ├── validator.rs
│   │   ├── handler.rs
│   │   └── renderer.rs
│   ├── impls/
│   │   ├── console_logger.rs
│   │   ├── length_validator.rs
│   │   ├── echo_handler.rs
│   │   └── html_renderer.rs
│   └── pipeline.rs
├── Cargo.toml

```

### 🧱 핵심 구성 요소
####  1. Ctx: 모든 상태를 담는 컨텍스트

- `src/ctx.rs`

```rust
#[derive(Default)]
pub struct Ctx {
    pub logs: Vec<String>,
    pub errors: Vec<String>,
    pub output: Vec<String>,
}
```


#### 2. 트레이트 정의 (동사 중심)
- `src/traits/logger.rs`
```rust
use crate::ctx::Ctx;

pub trait Logger {
    fn log(&self, msg: &str, ctx: &mut Ctx);
}
```

- `src/traits/validator.rs`
```rust
use crate::ctx::Ctx;

pub trait Validator {
    fn validate(&self, input: &str, ctx: &mut Ctx) -> bool;
}
```

- `src/traits/handler.rs`
```rust
use crate::ctx::Ctx;

pub trait Handler {
    fn handle(&self, input: &str, ctx: &mut Ctx);
}
```

- `src/traits/renderer.rs`
```rust
use crate::ctx::Ctx;

pub trait Renderer {
    fn render(&self, ctx: &mut Ctx);
}
```


#### 3. 구현체 예시
- `src/impls/console_logger.rs`

```rust
use crate::traits::logger::Logger;
use crate::ctx::Ctx;

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str, ctx: &mut Ctx) {
        ctx.logs.push(format!("LOG: {}", msg));
    }
}
```

- `src/impls/length_validator.rs`
```rust
use crate::traits::validator::Validator;
use crate::ctx::Ctx;

pub struct LengthValidator;

impl Validator for LengthValidator {
    fn validate(&self, input: &str, ctx: &mut Ctx) -> bool {
        if input.len() > 20 {
            ctx.errors.push("Too long".to_string());
            false
        } else {
            true
        }
    }
}
```

- `src/impls/echo_handler.rs`
```rust
use crate::traits::handler::Handler;
use crate::ctx::Ctx;

pub struct EchoHandler;

impl Handler for EchoHandler {
    fn handle(&self, input: &str, ctx: &mut Ctx) {
        ctx.output.push(format!("Handled: {}", input));
    }
}
```

- `src/impls/html_renderer.rs`
```rust
use crate::traits::renderer::Renderer;
use crate::ctx::Ctx;

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&self, ctx: &mut Ctx) {
        for line in &ctx.output {
            println!("<p>{}</p>", line);
        }
    }
}
```


### 4. 파이프라인 조립
- `src/pipeline.rs`
```rust
use crate::traits::{logger::Logger, validator::Validator, handler::Handler, renderer::Renderer};
use crate::ctx::Ctx;

pub struct Pipeline<'a> {
    pub logger: &'a dyn Logger,
    pub validator: &'a dyn Validator,
    pub handler: &'a dyn Handler,
    pub renderer: &'a dyn Renderer,
}

impl<'a> Pipeline<'a> {
    pub fn run(&self, input: &str, ctx: &mut Ctx) {
        self.logger.log(input, ctx);
        if self.validator.validate(input, ctx) {
            self.handler.handle(input, ctx);
        }
        self.renderer.render(ctx);
    }
}
```


### 5. 실행 예시
- `src/main.rs`
```rust
mod ctx;
mod traits {
    pub mod logger;
    pub mod validator;
    pub mod handler;
    pub mod renderer;
}
mod impls {
    pub mod console_logger;
    pub mod length_validator;
    pub mod echo_handler;
    pub mod html_renderer;
}
mod pipeline;

use crate::impls::*;
use crate::pipeline::Pipeline;
use crate::ctx::Ctx;

fn main() {
    let logger = console_logger::ConsoleLogger;
    let validator = length_validator::LengthValidator;
    let handler = echo_handler::EchoHandler;
    let renderer = html_renderer::HtmlRenderer;

    let mut ctx = Ctx::default();
    let pipeline = Pipeline {
        logger: &logger,
        validator: &validator,
        handler: &handler,
        renderer: &renderer,
    };

    pipeline.run("hello world", &mut ctx);
}
```


## 🧪 확장 아이디어
- Vec<Box<dyn Handler>>로 핸들러 체인 구성
- Signal<T>로 이벤트 기반 렌더링
- Ctx에 rand, clock, io 핸들 추가
- #[cfg(test)]로 각 트레이트 단위 테스트 작성



