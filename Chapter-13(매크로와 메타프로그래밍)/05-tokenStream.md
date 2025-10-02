# TokenStream이란?

- TokenStream은 Rust 코드 조각을 토큰 단위로 표현한 추상적인 스트림.
- 즉, let x = 5; 같은 코드가 컴파일러 내부에서는 let, x, =, 5, ; 같은 토큰들로 분해돼서 처리됨.
- Procedural Macro는 이 토큰 스트림을 받아서 새로운 토큰 스트림을 반환함 → 이게 최종 코드로 컴파일됨.

## 🔍 어디서 쓰이나? Procedural Macro 사용 위치
| 매크로 어노테이션         | 사용 위치 또는 용도                                |
|---------------------------|-----------------------------------------------------|
| `#[proc_macro]`            | 함수형 매크로. `my_macro!(...)`처럼 호출됨. 일반 코드 생성에 사용 |
| `#[proc_macro_derive]`     | 구조체/열거형에 `#[derive(...)]`를 붙여 트레잇 구현 자동 생성 |
| `#[proc_macro_attribute]`  | 함수, 구조체, 모듈 등에 `#[속성]`처럼 붙여 동작을 수정하거나 확장 |


## 🧩 구조와 특징
- TokenStream은 내부적으로 TokenTree들의 리스트로 구성됨
- TokenTree는 다음 중 하나일 수 있어:
- Group (괄호로 묶인 블록)
- Ident (식별자, 예: my_var)
- Punct (구두점, 예: =, ;)
- Literal (리터럴, 예: 42, "hello")

## ✨ 예시: 문자열을 TokenStream으로 변환
```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_fn(_input: TokenStream) -> TokenStream {
    let code = "fn hello() { println!(\"Hello!\"); }";
    code.parse().unwrap()
}
```

- "fn hello() { ... }"라는 문자열을 TokenStream으로 파싱해서 반환
- 결과적으로 hello() 함수가 생성됨

## 🔧 주요 메서드: TokenStream 조작용

| 메서드             | 설명 또는 역할                                      |
|--------------------|-----------------------------------------------------|
| `.is_empty()`       | TokenStream이 비어 있는지 확인함                     |
| `.to_string()`      | 토큰 스트림을 문자열로 변환함 (디버깅이나 로깅에 유용) |
| `.into_iter()`      | TokenStream을 반복자 형태로 분해함 (`TokenTree` 단위) |
| `.from_iter(...)`   | 여러 `TokenTree`를 하나의 TokenStream으로 결합함     |



## 간단 예시
```rust
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn inspect(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        panic!("입력이 비어 있습니다!");
    }

    for token in input.clone().into_iter() {
        println!("토큰: {}", token.to_string());
    }

    input
}
```
이 매크로는 입력된 토큰을 하나씩 출력하면서, 비어 있으면 에러를 발생.
이런 방식으로 매크로의 입력을 분석하거나 조건에 따라 다른 코드를 생성할 수 있음.



## 🎯 TokenStream 핵심 요약
| 개념             | 설명                                      |
|------------------|-------------------------------------------|
| `TokenStream`     | Rust 코드 조각을 토큰 단위로 표현한 스트림 |
| 구성 요소         | `TokenTree` (Ident, Punct, Literal, Group) |
| 사용 위치         | 모든 Procedural Macro의 입출력 타입         |
| 주요 기능         | 코드 파싱, 생성, 수정, 반복 처리 가능         |


이걸 잘 활용하면 syn으로 파싱하고 quote!로 생성하는 매크로를 자유자재로 만들 수 있음.

---


# 토큰 단위로 출력

Rust의 Procedural Macro에서 TokenStream을 받아서 토큰 단위로 하나씩 출력하는 예제. 
이걸 통해 컴파일 타임에 코드가 어떻게 분해되고 처리되는지를 눈으로 확인 가능.

## 🧪 프로젝트 구조
```
token_inspector/
├── Cargo.toml
├── src/
│   └── lib.rs         ← 매크로 정의
token_demo/
├── Cargo.toml
├── src/
│   └── main.rs        ← 매크로 사용
```


## 🛠️ 1. 매크로 크레이트 설정 (token_inspector/Cargo.toml)
```
[package]
name = "token_inspector"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true
```


## 🔍 2. 매크로 정의 (token_inspector/src/lib.rs)
```rust
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn inspect_tokens(input: TokenStream) -> TokenStream {
    for token in input.clone().into_iter() {
        match &token {
            TokenTree::Ident(ident) => println!("🆔 Ident: {}", ident),
            TokenTree::Literal(lit) => println!("🔢 Literal: {}", lit),
            TokenTree::Punct(punct) => println!("✳️ Punct: {}", punct),
            TokenTree::Group(group) => {
                println!("📦 Group: {:?}", group.delimiter());
                for inner in group.stream() {
                    println!("  └▶ {:?}", inner);
                }
            }
        }
    }

    input // 원래 입력을 그대로 반환
}
```


## 🧩 3. 매크로 사용 (token_demo/src/main.rs)
```rust
use token_inspector::inspect_tokens;

inspect_tokens! {
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}
```


## ✅ 실행 결과 예시
```
🆔 Ident: fn
🆔 Ident: greet
📦 Group: Parenthesis
  └▶ Ident: name
  └▶ Punct: :
  └▶ Punct: &
  └▶ Ident: str
✳️ Punct: ->
🆔 Ident: String
📦 Group: Brace
  └▶ Ident: format
  └▶ Group: Parenthesis
  └▶ Punct: !
  └▶ Group: Parenthesis
```

이 출력은 컴파일 타임에 발생하므로, cargo build 또는 cargo run 시 콘솔에 나타나게 됨


## 🎯 TokenStream 단위 출력 매크로

| 토큰 종류     | 설명                          | 예시 출력                     |
|---------------|-------------------------------|-------------------------------|
| `Ident`       | 식별자                         | `fn`, `greet`, `name`         |
| `Literal`     | 리터럴 값                     | `"Hello, {}!"`, `42`          |
| `Punct`       | 구두점                         | `:`, `->`, `!`                |
| `Group`       | 괄호로 묶인 블록               | `(...)`, `{...}`              |

---

# DSL 응용

Procedural Macro는 수식 파서나 DSL(도메인 특화 언어), 심지어 간단한 컴파일러 전처리기 역할까지 할 수 있음. 
왜냐하면 Rust의 TokenStream은 코드 자체를 컴파일 타임에 분석하고 조작할 수 있게 해주기 때문.

## 🧠 왜 수식 파서에 적합한가?
- TokenStream으로 사용자 입력을 토큰 단위로 분해할 수 있음
- syn을 사용하면 1 + 2 * (3 - 4) 같은 표현식을 AST로 파싱 가능
- quote!를 통해 실제 계산 로직을 Rust 코드로 생성 가능
- 컴파일 타임에 오류 검출, 최적화, 코드 생성까지 가능

## ✨ 예시: DSL로 수식 파서 만들기
```rust
math_expr! {
    let result = calc!(1 + 2 * (3 - 4));
}
```

이런 DSL을 만들면 calc!() 내부에서 수식을 파싱하고, Rust 코드로 변환해서 let result = -1; 같은 코드가 생성됨.

## 🔧 Procedural Macro로 가능한 기능들
### 🎯 수식 파서 매크로 구성 요소 요약
| 구성 요소         | 역할 또는 기능                                      |
|------------------|-----------------------------------------------------|
| `syn::Expr`       | Rust의 표현식(수식)을 파싱하는 AST 타입               |
| `quote!`          | Rust 코드 조각을 `TokenStream`으로 생성하는 매크로     |
| `compile_error!`  | 컴파일 타임에 사용자 정의 오류 메시지를 출력함         |


### 🧩 확장 아이디어
- #[optimize] 매크로로 수식을 컴파일 타임에 미리 계산
- #[validate_expr]로 수식의 문법 검사
- #[generate_ir]로 중간 표현(IR)을 출력해서 컴파일러처럼 동작

## 🎯 결론
Procedural Macro는 단순한 코드 자동화 도구를 넘어서,
컴파일 타임 파서, 트랜스파일러, DSL 인터프리터까지 만들 수 있는 강력한 도구.

----

