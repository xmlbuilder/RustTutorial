# TokenStream으로 DSL 만들기
TokenStream은 보통 프로시저 매크로에서 입력과 출력 타입으로 사용되지만,  
그 자체로 매우 유용한 추상 타입이라서 다른 곳에서도 응용할 수 있습니다.

## 🧠 TokenStream이란?
- `proc_macro::TokenStream` 은 Rust 코드 조각을 토큰 단위로 표현하는 타입.
- 컴파일 타임에 Rust 코드를 분석하거나 생성할 때 사용됩니다.

## 🔍 주로 쓰이는 곳
### 🧩 Rust 프로시저 매크로 종류 요약
| 매크로 종류                  | 설명 |
|------------------------------|------|
| `#[proc_macro_derive(...)]`  | `#[derive(...)]` 형태로 구조체/열거형에 trait 구현을 자동 생성하는 매크로 |
| `#[proc_macro_attribute]`    | `#[속성(...)]` 형태로 함수, 모듈, 구조체 등에 붙여 동작을 수정하거나 확장하는 매크로 |
| `#[proc_macro]`              | `my_macro!(...)`처럼 직접 호출되는 함수형 매크로. 일반적인 텍스트 변환이나 DSL 구현에 사용 |


## ✅ 다른 곳에서도 응용 가능한가?
### 1️⃣ 테스트용 파서/코드 분석기 만들기
- TokenStream을 활용해 Rust 코드 파싱 도구를 만들 수 있어요.
- 예: syn::parse::<DeriveInput>(token_stream)로 구조체 분석

### 2️⃣ 코드 생성 도구
- quote!와 함께 사용하면 코드 생성기, DSL, 빌더 패턴 자동화 도구도 만들 수 있음.
- 예: quote! { fn hello() {} }.into() → TokenStream

### 3️⃣ 컴파일러 플러그인 또는 Lint 도구
- Rust 공식 컴파일러는 플러그인을 제한하지만, TokenStream은 코드 트랜스포머나 정적 분석기의 내부 표현으로도 활용 가능

###⚠️ 단, 제약도 있어요
- proc_macro::TokenStream은 proc-macro crate에서만 사용 가능
- 일반 crate에서는 proc_macro2::TokenStream을 사용해야 함 (syn/quote와 호환)
```rust
use proc_macro2::TokenStream; // 일반 crate에서 사용
```

## 💬 결론
TokenStream은 단순히 매크로용 타입이 아니라,  
Rust 코드 생성과 분석을 위한 핵심 표현 도구.  
매크로 외에도 코드 자동화, 파서, DSL, 테스트 도구 등 다양한 곳에서 응용할 수 있음.

## 🧪 목표: HTML DSL 만들기
- 사용자가 이렇게 작성하면:
```rust
let markup = html! {
    div {
        h1 { "Hello" }
        p { "This is a paragraph." }
    }
};
```

- 자동으로 다음과 같은 문자열이 생성되도록 합니다:
```
<div><h1>Hello</h1><p>This is a paragraph.</p></div>
```

## 📦 프로젝트 구조
```
html_dsl/
├── Cargo.toml
├── html_macro/
│   ├── Cargo.toml
│   └── src/lib.rs
└── html_app/
    ├── Cargo.toml
    └── src/main.rs
```

## 🧰 1. html_macro/src/lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, parse::ParseStream, Token, Ident, LitStr, braced};

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let HtmlNode { tag, children } = parse_macro_input!(input as HtmlNode);

    let rendered_children = children.iter().map(|child| {
        let HtmlNode { tag, children } = child;
        let inner = quote! { #children };
        quote! {
            format!("<{tag}>{}</{tag}>", #inner)
        }
    });

    let output = quote! {
        {
            let mut result = String::new();
            result.push_str(&format!("<{tag}>"));
            #(result.push_str(&#rendered_children);)*
            result.push_str(&format!("</{tag}>"));
            result
        }
    };

    output.into()
}
```
```rust
// DSL 구조 정의
struct HtmlNode {
    tag: Ident,
    children: Vec<HtmlNode>,
}
```
```rust
impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut children = Vec::new();

        while !content.is_empty() {
            children.push(content.parse()?);
        }

        Ok(HtmlNode { tag, children })
    }
}
```

## 🧪 2. html_app/src/main.rs
```rust
use html_macro::html;

fn main() {
    let markup = html! {
        div {
            h1 { "Hello" }
            p { "This is a paragraph." }
        }
    };

    println!("{}", markup);
}
```

## 📦 3. 루트 Cargo.toml
```
[workspace]
members = ["html_macro", "html_app"]
```

## 💡 프로시저 매크로 DSL 구현 포인트 요약

| 코드 요소         | 설명 |
|-------------------|------|
| `#[proc_macro]`   | 직접 호출되는 함수형 매크로. DSL의 진입점 역할을 함 |
| `Parse`           | 사용자 입력을 Rust AST로 파싱하는 트레잇. DSL 문법을 분석하는 데 사용 |
| `quote!`          | Rust 코드 조각을 생성하는 매크로. DSL 결과를 코드로 표현 |
| `format!`         | 문자열을 조립하는 표준 매크로. DSL 결과를 최종 문자열로 출력 |

## 🔍 간단 흐름 요약
- #[proc_macro]로 DSL 매크로 정의
- Parse 트레잇으로 사용자 입력을 구조화
- quote!로 Rust 코드 생성
- format!으로 최종 문자열 조립

----

# 단계별로 정리

## 🔧 1단계: 매크로 진입점 정의
```rust
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
```
- `#[proc_macro]`: 함수형 매크로 정의
- `html! { ... }` 형태로 사용 가능
- `input: TokenStream`: 사용자가 작성한 DSL 코드가 토큰 스트림으로 들어옴

## 🧩 2단계: 입력 파싱
```rust
let HtmlNode { tag, children } = parse_macro_input!(input as HtmlNode);
```
- `parse_macro_input!`: TokenStream을 HtmlNode라는 구조체로 파싱
- `HtmlNode` 는 사용자 DSL의 한 노드를 표현 (예: div { ... })

## 🧱 3단계: 자식 노드 렌더링
```rust
let rendered_children = children.iter().map(|child| {
    let HtmlNode { tag, children } = child;
    let inner = quote! { #children };
    quote! {
        format!("<{tag}>{}</{tag}>", #inner)
    }
});
```
- 각 자식 노드(h1, p 등)에 대해 재귀적으로 HTML 문자열 생성
- `quote!` 는 Rust 코드 조각을 생성
- `format!("<tag>...</tag>")` 형태로 문자열 조립

## 🧱 4단계: 최종 HTML 조립
```rust
let output = quote! {
    {
        let mut result = String::new();
        result.push_str(&format!("<{tag}>"));
        #(result.push_str(&#rendered_children);)*
        result.push_str(&format!("</{tag}>"));
        result
    }
};
```
- `quote!` 로 최종 Rust 코드를 생성
- `#( ... )*` 는 반복 구문 (자식 노드들을 반복해서 삽입)
- result에 HTML 문자열을 순차적으로 조립

## 🔚 5단계: TokenStream으로 반환
```
output.into()
``

- `quote!` 결과를 TokenStream으로 변환
- 컴파일러가 이 코드를 실제로 컴파일하게 됨

## 🧬 6단계: DSL 파서 정의
```rust
struct HtmlNode {
    tag: Ident,
    children: Vec<HtmlNode>,
}
```

- DSL의 한 노드를 표현하는 구조체
- 예: div { ... }, h1 { ... } 등

## 🧪 7단계: Parse 트레잇 구현
```rust
impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag: Ident = input.parse()?;         // 태그 이름
        let content; braced!(content in input);  // 중괄호 내부 파싱

        let mut children = Vec::new();
        while !content.is_empty() {
            children.push(content.parse()?);     // 자식 노드 재귀 파싱
        }

        Ok(HtmlNode { tag, children })
    }
}
```

- syn::Parse 트레잇을 구현해서 사용자 DSL을 AST로 변환
- 중괄호 {} 안의 내용을 재귀적으로 파싱

## ⚠️ 개선 포인트
현재 구현은 "Hello" 같은 문자열 리터럴을 처리하지 못함.  
이를 위해 HtmlNode를 enum으로 바꾸고 Text(String) 타입을 추가하면 더 유연해집니다:
```rust
enum HtmlNode {
    Element { tag: Ident, children: Vec<HtmlNode> },
    Text(LitStr),
}
```
- 그리고 Parse 구현도 분기 처리로 확장하면 완성도 높은 DSL이 됩니다.

## ✅ 결론
이 매크로는 Rust에서 HTML처럼 보이는 DSL을 직접 정의하고 처리하는 예제.  
핵심은 Parse로 DSL을 AST로 바꾸고, quote!로 Rust 코드를 생성하는 구조.  
이 원리를 익히면 자신만의 DSL, 템플릿 엔진, 선언적 API도 만들 수 있음.  

---
