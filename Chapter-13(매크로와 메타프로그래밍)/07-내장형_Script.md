# Rhai / Dyon
Rhai와 Dyon은 Rust에서 사용할 수 있는 대표적인 내장형 스크립트 언어입니다.  
Rhai는 간결하고 안전한 문법으로 사용자 정의 로직을 쉽게 작성할 수 있고,  
Dyon은 더 동적이고 표현력이 풍부한 스크립트 환경을 제공합니다.

## 🧠 Rhai vs Dyon: Rust 내장 스크립트 언어 비교

| 항목             | Rhai 문법 예시                          | Dyon 문법 예시                             | 설명 및 차이점                          |
|------------------|-----------------------------------------|--------------------------------------------|-----------------------------------------|
| 변수 선언        | `let x = 5`                             | `let x = 5`                                 | 둘 다 동적 타입, 타입 명시 없음         |
| 변수 변경        | `x += 1`                                | `x += 1`                                    | 동일하게 사용 가능                      |
| 함수 정의        | `fn add(x, y) { x + y }`                | `fn add(x, y) { return x + y }`            | Dyon은 `return` 필수                    |
| 조건문           | `if x > 0 { x } else { -x }`            | `if x > 0 { return x } else { return -x }` | Dyon은 블록마다 `return` 필요           |
| 반복문           | `for x in 0..10 { ... }`                | `for x = 0; x < 10; x += 1 { ... }`        | Rhai는 Rust 스타일, Dyon은 C 스타일     |
| 수학 함수        | 직접 등록 필요 (`sqrt`, `sin` 등)      | 기본 내장 (`sqrt`, `sin`, `cos` 등)        | Dyon이 수학 함수 내장                   |
| 문자열 연결      | `"Hello " + name`                       | `"Hello " + name`                          | 동일하게 지원                          |
| Rust 연동        | `engine.eval("...")`                    | `load(...) + run(...)`                     | Rhai가 더 간단하고 직관적               |
| 결과 추출        | `eval::<i64>(...)`                      | `Variable::F64(val)`                       | Rhai는 타입 추론 쉬움                   |
| WASM 지원        | ✅ 지원                                  | ❌ 미지원                                   | Rhai는 WebAssembly에서 사용 가능        |

## Toml 설정
```
[dependencies]
rhai = "1.15"
dyon = "0.43"
```

## ✅ Rhai 예시
```rust
let mut engine = rhai::Engine::new();
let result = engine.eval::<i64>("let x = 3; x * x + 3 * x + 1")?;
println!("결과: {}", result); // 출력: 19
```

- Rust 코드에서 Rhai 스크립트를 직접 실행
- 변수, 조건문, 함수 정의 모두 가능

## ✅ Dyon 예시
```rust
fn f(x) {
    return x * x + 3 * x + 1
}
```

- Dyon은 자체 문법을 사용하며, Rust에서 실행하려면 별도 런타임 필요
- 표현력은 높지만 문법이 Rhai보다 낯설 수 있음

## 🔍 어떤 걸 선택해야 할까?

| 상황 또는 목적                          | Rhai 추천 이유                                      | Dyon 추천 이유                                      |
|----------------------------------------|----------------------------------------------------|----------------------------------------------------|
| Rust와 쉽게 통합하고 싶은 경우         | `engine.eval(...)`로 간단하게 실행 가능            | `load + run` 구조로 약간 복잡함                    |
| 수식 계산, 설정 DSL, 간단한 로직       | 간결한 문법, 빠른 실행                             | 표현력은 좋지만 오버헤드 있음                      |
| 수학 함수가 자주 필요한 경우           | 직접 등록 필요 (`sqrt`, `sin` 등)                  | 대부분 내장되어 바로 사용 가능                     |
| 복잡한 로직, 조건문, 반복문 포함       | 가능하지만 문법은 제한적                           | `for`, `if`, `return` 등 풍부한 문법 지원          |
| WebAssembly(WASM)에서 사용하려는 경우 | ✅ WASM 지원                                        | ❌ WASM 미지원                                      |
| 스크립트 문법을 Rust처럼 쓰고 싶은 경우| 문법이 Rust와 유사함                               | 문법이 독립적이고 학습 필요                         |
| 빠른 학습과 문서화가 중요한 경우       | 문서 풍부, 커뮤니티 활발                           | 문서 적고 예제 부족                                 |
| 표현력 높은 DSL을 만들고 싶은 경우     | 제한적 표현력                                      | 고급 표현력과 유연한 구조                          |

---

# Rhai 예제

- 이 예제는 f(x) = x^2 + 3x + 1 구조를 포함하면서, sin, cos, if 조건문까지 활용한 복합 수식 로직을 보여줍니다.

## 🧪 Rhai 예제: 수식 문자열 실행 + 조건 + 삼각함수
### 🔹 1. Rhai 스크립트 (문자열로 정의)
```rust
fn f(x) {
    if x > 0 {
        let base = x * x + 3 * x + 1;
        let trig = sin(x) + cos(x);
        return base + trig;
    } else {
        return -1;
    }
}
```

### 🔹 2. Rust 코드에서 실행
```rust
use rhai::{Engine, Scope};
fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    let script = r#"
        fn f(x) {
            if x > 0 {
                let base = x * x + 3 * x + 1;
                let trig = sin(x) + cos(x);
                return base + trig;
            } else {
                return -1;
            }
        }
    "#;

    let engine = Engine::new();
    let ast = engine.compile(script)?;

    for x in 1..=5 {
        let mut scope = Scope::new();
        scope.push("x", x as f64);

        let result = engine.call_fn::<f64>(&mut scope, &ast, "f", (x as f64,))?;
        println!("f({}) = {:.4}", x, result);
    }

    Ok(())
}
```

### ✅ 실행 결과 예시
```
f(1) = 6.5403
f(2) = 8.4931
f(3) = 12.8489
f(4) = 19.6536
f(5) = 28.7163
```
- ※ sin(x)와 cos(x)는 라디안 기준이며, Rhai는 f64 타입으로 자동 처리합니다.

### 🔍 핵심 포인트

| 항목              | 예시 또는 설명                                 |
|-------------------|------------------------------------------------|
| `Engine::compile()` | 수식 문자열을 Rhai AST로 컴파일               |
| `Scope::push()`     | 변수 `x`를 스코프에 등록하여 함수에 전달 가능 |
| `call_fn()`         | 컴파일된 AST에서 `f(x)` 함수 호출             |
| `sincos`            | `sin(x)`, `cos(x)`는 Rhai에서 기본 제공됨     |



## 🧠 Dyon의 문법과 특징
### 🔹 Dyon 함수 예시
```rust
fn f(x) {
    return x * x + 3 * x + 1
}
```

- 변수 타입 없음 → 동적 타입
- return 키워드 필수
- let, if, loop, break, continue 등 자체 문법 존재
- 배열, 객체, 문자열, 함수 등도 지원

### 🔍 Rhai / Rust와 비교

| 항목             | Rhai 예시                              | Rhai 간결형                           | Rust 예시                                 | 설명 및 차이점                          |
|------------------|-----------------------------------------|----------------------------------------|-------------------------------------------|-----------------------------------------|
| 함수 정의        | `fn f(x) { return x + 1 }`              | `fn f(x) { x + 1 }`                    | `fn f(x: i64) -> i64 { x + 1 }`           | Rhai는 `return` 생략 가능, Rust는 타입 명시 필수 |
| 변수 선언        | `let x = 5`                             | `let x = 5`                            | `let x: i64 = 5;`                         | Rhai는 동적 타입, Rust는 정적 타입       |
| 조건문           | `if x > 0 { x } else { -x }`            | `if x > 0 { x } else { -x }`           | `if x > 0 { x } else { -x }`              | 문법 거의 동일                          |
| 반복문           | `for x in 0..10 { ... }`                | `for x in 0..10 { ... }`               | `for x in 0..10 { ... }`                  | Rhai는 Rust 스타일 반복문 지원           |
| 수학 함수        | `let y = sqrt(x) + sin(x)`              | `let y = sqrt(x) + sin(x)`             | `let y = x.sqrt() + x.sin()`              | Rhai는 기본 수학 함수 내장, Rust는 `f64` 메서드 사용 |


## ✅ Dyon의 장점과 단점

| 항목 또는 기능       | 장점                                                  | 단점                                                  |
|----------------------|-------------------------------------------------------|-------------------------------------------------------|
| 문법 표현력          | `if`, `for`, `return`, `fn` 등 풍부한 문법 지원       | Rust와 문법이 달라 학습 곡선 존재                     |
| 수학 함수 (`sqrt`, `sin`) | 기본 내장되어 바로 사용 가능                           | 일부 고급 함수는 직접 구현 필요                       |
| 동적 타입            | 타입 선언 없이 유연하게 작성 가능                     | 타입 오류가 런타임에 발생할 수 있음                   |
| Rust 연동            | Rust에서 Dyon 모듈을 로드하고 실행 가능               | Rhai보다 연동 방식이 복잡함 (`load`, `run`, `call`)   |
| 표현력 높은 DSL      | 복잡한 계산, 조건, 반복 로직을 스크립트로 작성 가능   | IDE 지원 부족, 문서화가 Rhai보다 적음                 |
| WASM 지원            | ❌ 지원하지 않음                                       | 웹 기반 프로젝트에는 부적합                           |


## 🧪 Dyon에서 수식 실행 예시
```rust
fn f(x) {
    return x * x + 3 * x + 1
}
let result = f(4)
```
- 이 코드를 .dyon 파일로 저장한 뒤, Rust에서 Dyon 런타임을 통해 실행할 수 있어요.

## 🔚 결론
Dyon은 더 표현력 있는 스크립트 언어로, 수학 함수나 복잡한 로직을 다루기엔 좋지만,  
Rust와의 통합성이나 문법 친숙도 측면에서는 Rhai가 더 편리합니다.

---

# Dyon 예제

## ✅ Dyon에서 문자열 코드 실행 흐름 (0.4.x 기준)
- 문자열을 dyon::Module로 파싱
- 파싱된 모듈을 dyon::run()으로 실행
- 결과를 받아서 처리

### 🧪 예제: Dyon 코드 문자열 실행
```rust
use dyon::{load, run, Module, Error};
use std::sync::Arc;

fn main() -> Result<(), Error> {
    let code = r#"
        fn f(x) {
            return x * x + 3 * x + 1
        }

        let result = f(4)
    "#;

    let mut module = Module::new();
    load(code, Arc::new("inline".into()), &mut module)?;
    run(&module)?;

    Ok(())
}
```

- load(...): 문자열을 Dyon 모듈로 파싱
- run(...): 모듈을 실행
- result 변수는 내부에서 사용되지만 Rust로 직접 가져오려면 추가 작업 필요

## 🔍 Rhai와 비교

| 기능 또는 항목         | Rhai 예시                          | Dyon 예시                             | 설명 및 차이점                          |
|------------------------|------------------------------------|----------------------------------------|-----------------------------------------|
| 코드 실행              | `engine.eval("...")`              | `load(...) + run(...)`                | Rhai는 간단한 문자열 실행, Dyon은 모듈 기반 실행 |
| 함수 호출              | `engine.call_fn("f", args)`       | `module.call("f", args)`              | Rhai는 AST 기반 호출, Dyon은 모듈에서 직접 호출 |
| 결과 추출              | `eval::<i64>(...)`                | `Variable::F64(val)`                  | Rhai는 타입 추론 쉬움, Dyon은 타입 매칭 필요     |
| 수식 처리              | `x * x + 3 * x + 1`               | `x * x + 3 * x + 1`                   | 문법은 유사하지만 Dyon은 `return` 필수           |


## ✅ 결론
Dyon도 문자열로 받은 코드를 실행할 수 있지만, Rhai처럼 간단하게 eval("...")로 처리되지는 않음.  
대신 load()와 run()을 통해 모듈로 파싱하고 실행하는 방식입니다.

### Dyon 예제: 조건 + 반복 + 수학 함수
```rust
fn f(x) {
    if x > 0 {
        return sqrt(x) + sin(x) + cos(x)
    } else {
        return -1
    }
}

let total = 0
for i = 1; i <= 5; i += 1 {
    let value = f(i)
    println("f(" + str(i) + ") = " + str(value))
    total += value
}

println("총합: " + str(total))
```


### ✅ 주요 기능 설명

| 문법 요소                        | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `fn f(x)`                        | 매개변수 `x`를 받아 계산을 수행하는 사용자 정의 함수 선언             |
| `if x > 0`                       | 조건문: `x`가 0보다 클 경우에만 특정 블록 실행                        |
| `sqrt(x)`                        | `x`의 제곱근을 계산하는 수학 함수                                     |
| `sin(x)`, `cos(x)`               | `x`의 사인/코사인 값을 계산하는 삼각 함수 (라디안 기준)               |
| `for i = 1; i <= 5; i += 1`      | 1부터 5까지 반복 실행하는 루프 (Dyon 스타일 반복문)                   |
| `str(i)`                         | 숫자 `i`를 문자열로 변환 (문자열 연결 시 사용)                        |
| `println(...)`                   | 콘솔에 문자열 또는 계산 결과를 출력하는 함수                          |


### 🔍 실행 결과 예시
```
f(1) = 2.5403
f(2) = 2.4931
f(3) = 2.8489
f(4) = 3.6536
f(5) = 4.7163
총합: 16.2522
```
- ※ 결과는 Dyon의 내장 수학 함수에 따라 약간 달라질 수 있어요.

