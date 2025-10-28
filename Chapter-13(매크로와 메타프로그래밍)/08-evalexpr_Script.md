# evalexpr

## 🛠️ `evalexpr` 설치 방법과 주요 함수 정리, 그리고 샘플 코드까지 모두 준비
아래 내용을 따라하면 Rust에서 수식 계산과 변수 주입을 매우 쉽게 구현할 수 있습니다.

## 📦 1. Cargo.toml 설정
```
[dependencies]
evalexpr = "10.0.0"  # 최신 버전은 crates.io에서 확인 가능
```
- 최신 버전 확인: evalexpr on crates.io

## 🧠 2. 주요 함수 요약

| 함수 이름                        | 반환값 또는 역할                         |
|----------------------------------|------------------------------------------|
| `eval("1 + 2")`                  | `Value` (일반 타입: Int, Float 등)       |
| `eval_int("1 + 2")`              | `i64` (정수 결과만 반환)                 |
| `eval_with_context(expr, ctx)`   | `Value` (변수/함수 포함 수식 계산)       |
| `eval_int_with_context(expr, ctx)`| `i64` (변수 포함 수식을 정수로 계산)     |
| `eval_empty_with_context_mut()`  | `()` (변수 할당만 수행, 결과 없음)       |
| `build_operator_tree(expr)`      | `Node` (수식 AST, 반복 계산에 최적화)    |
| `context_map! { ... }`           | `Context` (변수/함수 등록용 매크로)      |


## ✅ 사용 예시 흐름
```rust
let context = context_map! {
    "x" => 3,
    "y" => 4,
    "add" => Function::new(|args| {
        let args = args.as_tuple()?;
        Ok(Value::Int(args[0].as_int()? + args[1].as_int()?))
    })
}.unwrap();

let result = eval_int_with_context("add(x, y)", &context).unwrap();
println!("결과: {}", result); // 출력: 7
```

## 🧪 3. 샘플 코드: 변수 주입 + 수식 계산
### 🔹 기본 수식 계산
```rust
use evalexpr::*;

fn main() {
    let result = eval("1 + 2 * 3").unwrap();
    println!("결과: {}", result); // 출력: 7
}
```


### 🔹 변수 주입 후 계산
```rust
use evalexpr::*;

fn main() {
    let mut context = HashMapContext::new();
    context.set_value("a".into(), 10.into()).unwrap();
    context.set_value("b".into(), 5.into()).unwrap();

    let result = eval_int_with_context("a + b * 2", &context).unwrap();
    println!("결과: {}", result); // 출력: 20
}
```


### 🔹 변수 할당 + 연산
```rust

use evalexpr::*;

fn main() {
    let mut context = HashMapContext::new();
    eval_empty_with_context_mut("x = 3", &mut context).unwrap();
    eval_empty_with_context_mut("x += 2", &mut context).unwrap();

    let result = eval_int_with_context_mut("x", &mut context).unwrap();
    println!("x의 값: {}", result); // 출력: 5
}
```


### 🔹 사용자 정의 함수 포함
```rust
use evalexpr::*;

fn main() {
    let context = context_map! {
        "f" => Function::new(|arg| {
            Ok(Value::Int(arg.as_int()? / 2))
        }),
        "x" => Value::Int(10)
    }.unwrap();

    let result = eval_int_with_context("f(x) + 1", &context).unwrap();
    println!("결과: {}", result); // 출력: 6
}
```


## ✅ 정리
- evalexpr는 Rust에서 수식 계산, 변수 주입, 함수 정의까지 가능한 강력한 도구입니다.
- eval_with_context를 사용하면 외부에서 값을 넣어 계산하거나, DSL처럼 활용할 수 있어요.

---

# 다중 값 얻기

## 🧪 예제: x 값을 변화시키며 수식 결과 배열로 수집
```rust
use evalexpr::*;

fn main() {
    let mut results = Vec::new();
    // 수식: x^2 + 3*x + 1
    let expr = "x * x + 3 * x + 1";

    for x in 0..10 {
        let mut context = HashMapContext::new();
        context.set_value("x".into(), x.into()).unwrap();

        let result = eval_int_with_context(expr, &context).unwrap();
        results.push(result);
    }

    println!("결과 배열: {:?}", results);
}
```


### 🔍 출력 예시
```
결과 배열: [1, 5, 11, 19, 29, 41, 55, 71, 89, 109]
```

### 🧠 설명
- for x in 0..10: x 값을 0부터 9까지 반복
- context.set_value("x", x): x 값을 수식에 주입
- eval_int_with_context(...): 수식을 계산
- results.push(...): 결과를 배열에 저장


## evalexpr 함수로 확장
evalexpr를 활용한 확장 아이디어를 표로 정리하고, 각 항목에 대한 샘플 코드도 함께 제공.

## ✅ 확장 아이디어
| 기능 또는 수식 표현             | 설명                                      |
|-------------------------------|-------------------------------------------|
| `yz`                          | 여러 변수(`y`, `z`)를 함께 주입하여 계산   |
| `"if x > 5 then x else 0"`    | 조건문을 사용하여 분기 처리                |
| `f(x) = x^2`                  | 사용자 정의 함수로 수식 구성               |


## 🧪 샘플 1: 여러 변수 y, z 사용
```rust
use evalexpr::*;

fn main() {
    let mut context = HashMapContext::new();
    context.set_value("y".into(), 3.into()).unwrap();
    context.set_value("z".into(), 4.into()).unwrap();

    let result = eval_int_with_context("y * z + 2", &context).unwrap();
    println!("결과: {}", result); // 출력: 14
}
```


## 🧪 샘플 2: 조건문 "if x > 5 then x else 0"
```rust
use evalexpr::*;

fn main() {
    let mut context = HashMapContext::new();
    context.set_value("x".into(), 7.into()).unwrap();

    let result = eval_int_with_context("if x > 5 then x else 0", &context).unwrap();
    println!("결과: {}", result); // 출력: 7
}
```

## 🧪 샘플 3: 사용자 정의 함수 f(x) = x^2

```rust
use evalexpr::*;

fn main() {
    let context = context_map! {
        "f" => Function::new(|arg| {
            let x = arg.as_int()?;
            Ok(Value::Int(x * x))
        }),
        "x" => Value::Int(6)
    }.unwrap();

    let result = eval_int_with_context("f(x)", &context).unwrap();
    println!("결과: {}", result); // 출력: 36
}
```


## ✅ 요약
- evalexpr는 단순 수식 계산을 넘어서  
    다변수 계산, 조건문 처리, 사용자 정의 함수까지 지원하는 강력한 DSL 엔진이에요.

---

# build_operator_tree를

evalexpr의 build_operator_tree를 활용하면 수식을 미리 파싱해서 반복 계산 시 성능을 최적화할 수 있음.  
아래에 개념 설명과 함께 실전 예제를 정리.

## 🧠 개념: build_operator_tree

| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| 목적             | 문자열 수식을 한 번만 파싱하여 반복 계산 시 성능 최적화               |
| 반환 타입        | `Node` (수식의 AST 구조)                                              |
| 계산 방식        | `Node.eval()` 또는 `Node.eval_with_context()`로 계산 수행             |
| 반복 사용        | `for` 루프 등에서 동일 수식을 여러 번 계산할 때 효율적으로 재사용 가능 |
| 장점             | `eval("...")`보다 빠르고 메모리 효율적                                |


## 🔧 예제: x * x + 3 * x + 1 수식을 반복 계산
```rust
use evalexpr::*;

fn main() {
    // 수식을 미리 파싱해서 연산 트리 생성
    let tree = build_operator_tree("x * x + 3 * x + 1").unwrap();

    let mut results = Vec::new();

    for x in 0..10 {
        let mut context = HashMapContext::new();
        context.set_value("x".into(), x.into()).unwrap();

        // 미리 파싱된 트리로 계산
        let result = tree.eval_int_with_context(&context).unwrap();
        results.push(result);
    }

    println!("결과 배열: {:?}", results);
}
```


### ✅ 출력 결과
```
결과 배열: [1, 5, 11, 19, 29, 41, 55, 71, 89, 109]
```

## 🔍 비교: `eval("...")` vs `build_operator_tree`

| 방식                  | 파싱 방식         | 성능 특성             | 사용 용도                     | 특징 요약                     |
|-----------------------|------------------|------------------------|-------------------------------|-------------------------------|
| `eval("...")`         | 매번 문자열 파싱 | 느림 (반복 시 비효율) | 단발성 계산, 간단한 수식      | 간편하지만 반복에 부적합      |
| `build_operator_tree` | 1회 AST 생성     | 빠름 (재사용 가능)     | 반복 계산, 루프, 캐싱         | 성능 최적화에 유리            |



## ✅ 결론
- build_operator_tree는 수식을 한 번만 파싱하고 재사용할 수 있어  
    반복 계산이 필요한 상황에서 성능을 크게 향상시켜줍니다.

---

# 수식 지원

evalexpr에서 사용할 수 있는 주요 수학 함수는 `sqrt`, `sin`, `cos`, `tan`, `log`, `exp`, `abs`, `round`, `floor`, `ceil`  
는 사용자가 직접 구현해야 한다.
```rust
fn test_math_functions() {
    
    let ctx = context_map! {
        "sqrt" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.sqrt()))
        }),

        "sin" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.sin()))
        }),

        "cos" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.cos()))
        }),

        "tan" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.tan()))
        }),

        "log" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.log10()))
        }),

        "ln" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.ln()))
        }),

        "exp" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.exp()))
        }),

        "abs" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Float(x.abs()))
        }),

        "round" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Int(x.round() as IntType))
        }),

        "floor" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Int(x.floor() as IntType))
        }),

        "ceil" => Function::new(|arg| {
            let x = arg.as_number()?;
            Ok(Value::Int(x.ceil() as IntType))
        }),

        "max" => Function::new(|arg| {
            let args = arg.as_tuple()?;
            let a = args[0].as_number()?;
            let b = args[1].as_number()?;
            Ok(Value::Float(a.max(b)))
        }),

        "min" => Function::new(|arg| {
            let args = arg.as_tuple()?;
            let a = args[0].as_number()?;
            let b = args[1].as_number()?;
            Ok(Value::Float(a.min(b)))
        }),

        "x" => Value::Float(std::f64::consts::PI)

    }.unwrap();

    assert_eq!(eval_float_with_context("sqrt(9) + 1", &ctx).unwrap(), 4.0);
    assert!((eval_float_with_context("sin(x / 2)", &ctx).unwrap() - 1.0).abs() < 0.01);
    assert_eq!(eval_float_with_context("cos(0)", &ctx).unwrap(), 1.0);
    assert!((eval_float_with_context("tan(x / 4)", &ctx).unwrap() - 1.0).abs() < 0.01);
    assert_eq!(eval_float_with_context("log(100)", &ctx).unwrap(), 2.0);
    assert!((eval_float_with_context("ln(2.718)", &ctx).unwrap() - 1.0).abs() < 0.01);
    assert!((eval_float_with_context("exp(1)", &ctx).unwrap() - 2.718).abs() < 0.01);
    assert_eq!(eval_float_with_context("abs(-5)", &ctx).unwrap(), 5.0);
    assert_eq!(eval_int_with_context("round(3.6)", &ctx).unwrap(), 4);
    assert_eq!(eval_int_with_context("floor(3.6)", &ctx).unwrap(), 3);
    assert_eq!(eval_int_with_context("ceil(3.2)", &ctx).unwrap(), 4);
    assert_eq!(eval_float_with_context("max(3, 7)", &ctx).unwrap(), 7.0);
    assert_eq!(eval_float_with_context("min(3, 7)", &ctx).unwrap(), 3.0);
}
```

## 🧠 evalexpr 구현 함수 목록

| 함수 이름       | 설명                                  | 예시 사용법 및 결과             |
|----------------|---------------------------------------|----------------------------------|
| `sqrt(x)`      | x의 제곱근                             | `sqrt(9)` → `3`                 |
| `sin(x)`       | x의 사인 (라디안 기준)                | `sin(3.1415 / 2)` → `1`         |
| `cos(x)`       | x의 코사인                            | `cos(0)` → `1`                  |
| `tan(x)`       | x의 탄젠트                            | `tan(3.1415 / 4)` → `1`         |
| `log(x)`       | x의 로그 (밑 10)                      | `log(100)` → `2`                |
| `ln(x)`        | x의 자연로그 (밑 e)                   | `ln(2.718)` → `1`               |
| `exp(x)`       | e^x                                   | `exp(1)` → `2.718...`           |
| `abs(x)`       | x의 절댓값                            | `abs(-5)` → `5`                 |
| `round(x)`     | x를 반올림                            | `round(3.6)` → `4`              |
| `floor(x)`     | x를 내림                              | `floor(3.6)` → `3`              |
| `ceil(x)`      | x를 올림                              | `ceil(3.2)` → `4`               |
| `max(a, b)`    | a와 b 중 큰 값                        | `max(3, 7)` → `7`               |
| `min(a, b)`    | a와 b 중 작은 값                      | `min(3, 7)` → `3`               |

## 🧪 다항식(polynomial) 수식 예시
다항식은 함수가 아니라 산술 연산으로 직접 표현합니다:
```rust
use evalexpr::*;

fn main() {
    let mut context = HashMapContext::new();
    context.set_value("x".into(), 2.into()).unwrap();

    // 다항식: x^2 + 3x + 1
    let result = eval_int_with_context("x * x + 3 * x + 1", &context).unwrap();
    println!("결과: {}", result); // 출력: 11
}
```

### ✅ 참고
- evalexpr는 라디안 기준으로 삼각함수를 계산합니다.
- pow(x, y) 같은 거듭제곱 함수는 기본적으로 x * x처럼 표현하거나 사용자 정의 함수로 구현해야 합니다.
- build_operator_tree()를 사용하면 수식을 미리 파싱해 반복 계산 시 성능이 향상됩니다.

### 🧪 예제 1: 사용자 정의 pow(x, y) 함수
```rust
use evalexpr::*;

fn main() {
    let context = context_map! {
        "pow" => Function::new(|args| {
            let args = args.as_tuple()?;
            let base = args[0].as_int()?;
            let exp = args[1].as_int()?;
            Ok(Value::Int(base.pow(exp as u32)))
        }),
        "x" => Value::Int(2),
        "y" => Value::Int(3)
    }.unwrap();

    let result = eval_int_with_context("pow(x, y)", &context).unwrap();
    println!("결과: {}", result); // 출력: 8
}
```


### 🧪 예제 2: 사용자 정의 다항식 함수 f(x) = x^2 + 3x + 1
```rust
use evalexpr::*;

fn main() {
    let context = context_map! {
        "f" => Function::new(|arg| {
            let x = arg.as_int()?;
            let result = x * x + 3 * x + 1;
            Ok(Value::Int(result))
        }),
        "x" => Value::Int(4)
    }.unwrap();

    let result = eval_int_with_context("f(x)", &context).unwrap();
    println!("결과: {}", result); // 출력: 33
}
```

### ✅ 요약: 사용자 정의 함수 구현

| 함수 표현     | 내부 구현 방식       | 예시 사용 및 결과     |
|---------------|----------------------|------------------------|
| `pow(x, y)`   | `base.pow(exp)`      | `pow(2, 3)` → `8`      |
| `f(x)`        | `x * x + 3 * x + 1`  | `f(4)` → `33`          |

---
