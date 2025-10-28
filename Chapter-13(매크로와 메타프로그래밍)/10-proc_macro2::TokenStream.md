# proc_macro2::TokenStream

proc_macro2::TokenStream은 매크로뿐 아니라 일반 Rust 코드 블록을 파싱하거나 분석하는 데도 활용할 수 있습니다.  
특히 syn과 함께 사용하면 수식, 함수, 구조체 등 다양한 코드 요소를 추출하고 조작할 수 있음.

## 🧩 핵심 요약: proc_macro2::TokenStream의 일반 코드 파싱 활용
### ✅ 목적
- 매크로 외부에서도 Rust 코드 블록을 파싱하고 분석하고 싶을 때 사용
- 예: 문자열로 된 Rust 코드 → AST로 파싱 → 구조 분석 또는 코드 생성

### 🔧 기본 흐름
```rust
use proc_macro2::TokenStream;
use syn::{parse2, Expr};

fn main() {
    // Rust 코드 블록을 문자열로 정의
    let code = "1 + 2 * 3";

    // 문자열을 TokenStream으로 변환
    let ts: TokenStream = code.parse().unwrap();

    // TokenStream을 syn을 통해 Expr로 파싱
    let expr: Expr = parse2(ts).unwrap();

    println!("{:#?}", expr); // AST 구조 출력
}
```

### 🔍 설명
- code.parse()는 FromStr 트레잇을 통해 TokenStream으로 변환
- syn::parse2()는 proc_macro2::TokenStream을 받아서 원하는 타입으로 파싱
- 여기서는 Expr을 사용했지만, ItemFn, DeriveInput, Type, Stmt 등도 가능

### 🧠 활용 예시: syn 타입별 코드 파싱

| 코드 요소       | syn 타입       | 설명 및 활용 예시 |
|----------------|----------------|-------------------|
| 수식           | `Expr`         | `1 + 2 * 3` 같은 수식을 파싱하여 연산자/피연산자 분석 |
| 함수 정의      | `ItemFn`       | `fn hello() {}` 함수의 이름, 인자, 반환 타입 등을 추출 |
| 구조체/열거형  | `DeriveInput`  | `struct Person { ... }`를 파싱하여 필드 기반 코드 생성 |
| 타입           | `Type`         | `Vec<String>` 같은 타입을 파싱하여 제네릭, 경로 분석 |
| 문장           | `Stmt`         | `let x = 5;` 같은 문장을 파싱하여 변수 선언, 제어 흐름 분석 |
| 모듈           | `ItemMod`      | `mod my_module { ... }` 모듈 내부 구조 분석 |
| 구현 블록      | `ItemImpl`     | `impl MyTrait for T {}` 구현 대상과 메서드 추출 |


### ⚠️ 주의할 점
- proc_macro2::TokenStream은 proc-macro crate 외부에서도 사용 가능하지만,  
- syn::parse_macro_input!은 proc_macro::TokenStream만 받기 때문에 일반 코드에서는 parse2()를 사용해야 함
- quote!와 함께 사용하면 파싱한 내용을 다시 Rust 코드로 출력 가능

### 🧩 목표
예를 들어 이런 Rust 코드 블록이 있다고 가정해봅시다:
```rust
let a = 10;
let b = a + 5;
```
- 이걸 문자열로 받아서 → TokenStream으로 변환 → syn::Stmt 또는 syn::Block으로 파싱  
  → 변수 이름, 값, 수식 등을 추출하는 방식입니다.

### 🔧 예제 코드: 변수 포함 코드 블록 파싱
```rust
use proc_macro2::TokenStream;
use syn::{parse2, Block};

fn main() {
    let code = r#"
    {
        let a = 10;
        let b = a + 5;
    }
    "#;

    // 문자열 → TokenStream
    let ts: TokenStream = code.parse().unwrap();

    // TokenStream → syn::Block
    let block: Block = parse2(ts).unwrap();

    // 출력: Block 내부의 문장들
    for stmt in block.stmts {
        println!("{:#?}", stmt);
    }
}
```


### 🔍 결과 예시
이 코드를 실행하면 syn::Stmt 타입으로 각 문장이 출력됩니다.  
예를 들어:
```rust
Stmt::Local(
    Local {
        pat: Pat::Ident { ident: "a", ... },
        init: Some(Expr::Lit { lit: 10 }),
        ...
    }
)
```
- 이런 식으로 let a = 10;이라는 문장이 AST로 표현됩니다.
- 이를 통해 변수 이름(a), 초기값(10), 연산식(a + 5) 등을 추출하거나 조작할 수 있음.

## 🧠 확장 아이디어

| 기능 또는 목적     | syn 타입 / 매크로 | 설명 및 활용 예시 |
|--------------------|-------------------|-------------------|
| 변수 이름 추출     | `Pat::Ident`       | `let x = ...`에서 `x`를 추출하여 변수 추적, 이름 분석 |
| 수식 구조 분석     | `Expr::Binary`     | `a + b * c` 같은 수식을 파싱하여 연산자/피연산자 분리 |
| 코드 생성          | `quote!`           | 파싱한 내용을 기반으로 새로운 Rust 코드 생성 |
| 함수 구조 분석     | `ItemFn`           | 함수 이름, 인자, 반환 타입 등을 추출하여 문서화 또는 리팩토링 |
| 구조체 필드 추출   | `DeriveInput`      | 구조체 정의에서 필드 정보를 추출하여 자동 trait 구현 |
| 타입 분석          | `Type`             | `Vec<String>` 같은 타입을 파싱하여 제네릭 구조 분석 |
| 문장 흐름 추적     | `Stmt`             | 코드 블록 내 문장들을 순서대로 분석하여 실행 흐름 시각화 |


## ✅ 결론
- proc_macro2::TokenStream과 syn을 활용하면 변수가 포함된 코드 블록도 자유롭게 파싱하고 분석할 수 있음.
- 이를 기반으로 코드 분석기, 자동 생성기, DSL 인터프리터까지 확장 가능합니다.

--- 

# 변수 외부 주입

Rust 코드 블록 안에 있는 **변수(a, b 등)** 를 외부에서 주입해서,  
그 코드 블록을 실행하거나 수식을 계산하는 방식이 불가능.

## ✅ 결론부터 말하면…
Rust에서는 런타임에 코드 블록을 동적으로 파싱하고 실행하는 기능이 기본적으로 제공되지 않습니다.  
즉, `C#의 Roslyn` 처럼 문자열로 된 코드를 런타임에 컴파일하고 실행하는 기능은 Rust에는 없음.  

### 🧩 해결 방법 1: 수식 파서 라이브러리 사용
예:
```rust 
use evalexpr::*;

fn main() {
    let mut context = HashMapContext::new();
    context.set_value("a".into(), 10.into()).unwrap();
    context.set_value("b".into(), 5.into()).unwrap();

    let result = eval_with_context("a + b * 2", &context).unwrap();
    println!("결과: {}", result); // 출력: 20
}
```
- "a + b * 2" 같은 수식을 문자열로 받아서
- 외부에서 a, b 값을 주입하고
- 계산 결과를 얻을 수 있음

### 🧩 해결 방법 2: proc_macro2 + syn으로 파싱 → 직접 계산 로직 구현
```rust
use proc_macro2::TokenStream;
use syn::{parse2, Expr};

use proc_macro2::TokenStream;
use syn::{parse2, Expr};
use std::collections::HashMap;

fn main() {
    // 수식 문자열
    let code = "a + b * 2";

    // 외부에서 주입할 변수 값들
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), 10);
    vars.insert("b".to_string(), 5);

    // 문자열 → TokenStream → Expr
    let ts: TokenStream = code.parse().unwrap();
    let expr: Expr = parse2(ts).unwrap();

    // 수식 계산
    let result = eval_expr(&expr, &vars);
    println!("결과: {}", result); // 출력: 20
}
```
```rust
// 재귀적으로 Expr을 탐색하며 계산
fn eval_expr(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Lit(lit) => {
            if let syn::Lit::Int(int_lit) = &lit.lit {
                int_lit.base10_parse::<i64>().unwrap()
            } else {
                panic!("숫자 리터럴만 지원합니다")
            }
        }
        Expr::Binary(bin) => {
            let left = eval_expr(&bin.left, vars);
            let right = eval_expr(&bin.right, vars);
            match &bin.op {
                syn::BinOp::Add(_) => left + right,
                syn::BinOp::Sub(_) => left - right,
                syn::BinOp::Mul(_) => left * right,
                syn::BinOp::Div(_) => left / right,
                _ => panic!("지원하지 않는 연산자입니다"),
            }
        }
        Expr::Path(path) => {
            let ident = path.path.segments.first().unwrap().ident.to_string();
            *vars.get(&ident).unwrap_or_else(|| panic!("변수 '{}'이 정의되지 않았습니다", ident))
        }
        _ => panic!("지원하지 않는 표현식입니다"),
    }
}
```

- 이 방식은 AST를 직접 탐색해서 계산 로직을 구현해야 하므로 복잡하지만 유연함
- 예: Expr::Binary를 만나면 좌우 피연산자를 재귀적으로 계산

## 🧠 요약: Rust에서 외부 변수 주입 기반 수식 처리 방식

| 방식               | 핵심 도구            | 특징 및 설명                                                   | 장점                          | 단점                          |
|--------------------|----------------------|----------------------------------------------------------------|-------------------------------|-------------------------------|
| `evalexpr` 방식     | `evalexpr`, `meval`  | 문자열 수식을 파싱하고 외부 변수 값을 주입하여 계산            | 간단하고 빠름                 | Rust 문법 전체는 지원 안 됨   |
| `proc_macro2 + syn`| `proc_macro2`, `syn` | Rust 코드 블록을 AST로 파싱하고 직접 계산 로직을 구현           | 유연하고 Rust 문법 호환 가능  | 구현 복잡도 높고 실행 불가    |


## ✅ 결론
Rust에서는 C#처럼 런타임에 코드 블록을 실행하는 기능은 없지만,  
수식 계산이나 변수 주입은 라이브러리 또는 AST 기반 파싱으로 충분히 구현 가능합니다.

---

# braced!

**braced!** 는 Rust의 매크로에서 중괄호 {}를 파싱할 때 사용하는 매크로입니다.  
주로 syn 크레이트에서 파서 구현 시 사용되며, 중괄호로 감싸진 블록을 안전하게 추출하고  
그 내부 내용을 분석할 수 있게 해줍니다.

## 🧠 braced!의 의미와 사용 목적
- 정의 위치: syn 크레이트의 macro_rules! braced 매크로
- 기능: 중괄호 {}로 감싸진 블록을 파싱하고, 그 내부를 ParseStream으로 노출
- 사용 예시: 구조체 정의, 함수 블록, 매크로 내부 블록 등에서 중괄호를 처리할 때
```rust
let content;
brace_token = braced!(content in input);
```

위 코드는 input 스트림에서 { ... } 블록을 파싱하고,  
그 내부를 content라는 파서 스트림으로 노출합니다.  
이후 content.parse_terminated(...) 같은 방식으로 내부 필드를 파싱할 수 있음.

### 📦 실제 사용 예시
```rust
struct Struct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Struct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}
```

- braced!(content in input)은 { a: A, b: B } 같은 블록을 파싱하고 content로 내부를 넘겨줍니다.
- 이후 content를 통해 내부 필드들을 순차적으로 파싱할 수 있어요.

## ✅ 요약: braced! 매크로

| 항목       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| 용도       | `{}` 중괄호 블록을 파싱하고 내부를 `ParseStream`으로 노출             |
| 반환값     | 내부 블록을 파싱할 수 있는 `ParseStream` (`content`)                 |
| 사용 위치  | `syn` 크레이트 (Rust 파서 구현 시 사용)                              |
| 사용 예시  | `braced!(content in input)` → `{ ... }` 블록을 content로 추출         |
| 주요 활용  | 구조체, enum, 함수, 매크로 등 중괄호 기반 문법 요소 파싱              |

---

# Punctuated<Field, Token![,]>

## 🧠 Punctuated<Field, Token![,]> 구성 요약
이 타입은 다음과 같은 의미를 갖습니다:

| 구성 요소                     | 의미 및 설명                                       |
|------------------------------|----------------------------------------------------|
| `Field`                      | 파싱할 개별 항목 (예: 구조체의 필드 하나)          |
| `Token![,]`                  | 각 항목 사이의 구분자 (쉼표 `,`)                   |
| `Punctuated<Field, Token![,]>` | 쉼표로 구분된 여러 개의 `Field` 리스트를 표현하는 타입 |

이 타입은 syn 크레이트에서 자주 사용되며,  
예를 들어 구조체 정의에서 { a: i32, b: String } 같은 필드 목록을 파싱할 때 유용하게 쓰임.  
즉, 이 표현은 쉼표로 구분된 여러 개의 Field 항목을 파싱해서 저장하는 구조를 의미합니다.

## 📦 예시: 구조체 필드 파싱
```rust
struct MyStruct {
    a: i32,
    b: String,
    c: bool,
}
```

이 구조체를 파싱할 때 fields: Punctuated<Field, Token![,]>는 다음처럼 작동합니다:
- `a`: i32 → Field
- `,` → Token![,]
- `b`: String → Field
- `,` → Token![,]
- `c`: bool → Field
이렇게 `Field`와 `Token![,]` 가 번갈아가며 저장된 리스트가 만들어집니다.

## ✅ 요약
```rust
Punctuated<T, P>
```
- T: 항목 타입 (예: Field)
- P: 구분자 (예: 쉼표 ,)
- 역할: T 항목들이 P로 구분된 리스트를 파싱하고 저장

---

