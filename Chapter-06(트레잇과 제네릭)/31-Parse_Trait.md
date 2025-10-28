# Parse Trait
impl Parse는 Rust의  크레이트에서 제공하는 **공식 트레이트(인터페이스)** 입니다.  
이 트레이트를 구현하면 사용자 정의 타입에 대해 Rust 코드의 구문을 파싱할 수 있는 로직을 직접 정의할 수 있음.

## 🧠 Parse 트레이트란?

| 항목           | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| 정의 위치      | `syn::parse::Parse` 트레이트                                         |
| 사용 목적      | `proc_macro` 입력을 사용자 정의 타입으로 파싱할 때 사용              |
| 핵심 메서드    | `fn parse(input: ParseStream) -> Result<Self>`                       |
| 사용 예시      | 구조체, enum, DSL, 매크로 입력 등을 직접 파싱할 때 구현              |

## 📦 예시: 구조체 파싱
```parsed_result
use syn::{parse::{Parse, ParseStream}, Token, Ident, Result};

struct MyStruct {
    name: Ident,
    comma: Token![,],
    value: Ident,
}

impl Parse for MyStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MyStruct {
            name: input.parse()?,
            comma: input.parse()?,
            value: input.parse()?,
        })
    }
}
```
- input.parse()?는 ParseStream에서 원하는 토큰을 하나씩 파싱
- MyStruct는 Parse를 구현했기 때문에 syn::parse_macro_input! 같은 매크로에서 직접 파싱 가능

## ✅ 요약: Parse 트레이트

| 항목         | 설명                                               |
|--------------|----------------------------------------------------|
| Parse        | `syn` 크레이트에서 제공하는 파싱용 트레이트        |
| 정의 위치    | `syn::parse::Parse`                                |
| 사용 목적    | `proc_macro` 입력을 사용자 정의 타입으로 파싱       |
| 입력 타입    | `ParseStream` — 토큰 스트림을 순차적으로 파싱       |
| 핵심 메서드  | `fn parse(input: ParseStream) -> Result<Self>`     |

## ✅ 전체 예제: MyStruct 파싱 + 메인 매크로
### 📦 1. 구조체 정의 및 Parse 구현
```rust
use syn::{parse::{Parse, ParseStream}, Ident, Token, Result};

pub struct MyStruct {
    pub name: Ident,
    pub comma: Token![,],
    pub value: Ident,
}

impl Parse for MyStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MyStruct {
            name: input.parse()?,
            comma: input.parse()?,
            value: input.parse()?,
        })
    }
}
```

### 📦 2. 메인 매크로 함수 (lib.rs 또는 main.rs)
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use crate::MyStruct;

#[proc_macro]
pub fn parse_my_struct(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as MyStruct);

    let name = &parsed.name;
    let value = &parsed.value;

    let expanded = quote! {
        fn parsed_result() {
            println!("Parsed name: {}", stringify!(#name));
            println!("Parsed value: {}", stringify!(#value));
        }
    };

    TokenStream::from(expanded)
}
```

### 📦 3. 사용 예시 (main.rs)
```rust
parse_my_struct!(my_name, my_value);
```
- 이 코드는 my_name과 my_value라는 식별자를 받아서 MyStruct로 파싱한 뒤, parsed_result() 함수를 생성합니다.

## ✅ 요약: MyStruct 파싱 흐름

| 항목         | 설명                                                           |
|--------------|----------------------------------------------------------------|
| MyStruct     | `Ident`, `Token![,]`, `Ident`로 구성된 사용자 정의 구조체       |
| impl Parse   | `ParseStream`에서 `Ident`, `Token![,]`, `Ident` 순으로 파싱     |
| proc_macro   | 매크로 입력을 `MyStruct`로 파싱하여 코드 생성에 활용            |

- comma: Token![,]는 Rust의  크레이트에서 쉼표(,) 토큰을 명시적으로 파싱하기 위해 사용되는 타입. 
- 이 값은 input.parse()?를 통해 자동으로 생성됩니다.

## 🧠 comma: Token![,] 생성 방식
### 📦 코드 예시
```rust
comma: input.parse()?,
```
- input은 ParseStream 타입으로, 토큰 스트림을 순차적으로 읽어가는 역할을 합니다.
- Token![,]는 syn이 제공하는 매크로로, 쉼표 토큰을 명시적으로 지정합니다.
- input.parse()?는 현재 위치에서 쉼표가 있는지 확인하고, 있으면 Token![,] 타입으로 반환합니다.

## ✅ 내부 동작 흐름

| 단계         | 설명                                                           |
|--------------|----------------------------------------------------------------|
| ParseStream  | 토큰 스트림을 순차적으로 읽어가는 입력 파서                    |
| ,Token![,]   | 쉼표 토큰을 명시적으로 지정하여 해당 위치에서 파싱 시도         |
| comma        | `input.parse()?`를 통해 쉼표가 존재하면 `Token![,]`로 반환됨    |
| 결과         | `comma` 변수에 쉼표 토큰이 저장되어 이후 로직에서 활용 가능     |


## 📌 요약: Token![,] 파싱 흐름

| 항목             | 설명                                               |
|------------------|----------------------------------------------------|
| Token![,]        | `syn`에서 쉼표 토큰을 명시적으로 지정하는 매크로   |
| input.parse()?   | 현재 위치에서 쉼표 토큰을 파싱 시도                 |
| 결과 저장 대상   | 파싱된 쉼표 토큰은 `comma` 변수에 저장됨           |


## 🧠 왜 comma를 넣는가?

| 항목           | 설명                                                               |
|----------------|--------------------------------------------------------------------|
| a, b           | 쉼표로 구분된 항목을 정확하게 파싱하기 위한 입력 형식              |
| Token![,]      | `syn`에서 쉼표 토큰을 명시적으로 파싱하기 위한 타입                |
| quote!comma    | 코드 생성 시 쉼표를 포함하려면 `comma`를 `quote!`에 넣어야 함       |


## ✅ 구조체로 사용할 때는?
- MyStruct를 단순 데이터 구조로 사용한다면 comma는 제거해도 무방합니다.
- 하지만 MyStruct를 파서용 타입으로 사용하거나 proc_macro에서 코드 생성에 활용한다면 comma는 필요합니다.

## 📦 예시 비교
### ❌ 일반 구조체로 사용할 경우 (불필요)
```rust
struct MyStruct {
    name: Ident,
    value: Ident,
}
```

### ✅ 파싱용 구조체로 사용할 경우 (필요)
```rust
struct MyStruct {
    name: Ident,
    comma: Token![,],
    value: Ident,
}
```

## ✨ 요약: comma의 역할 in proc_macro

| 항목         | 설명                                                           |
|--------------|----------------------------------------------------------------|
| comma        | `Token![,]`으로 파싱된 쉼표 토큰. 위치 정보(span)를 포함함     |
| 파싱 목적    | 쉼표로 구분된 구문을 정확하게 인식하고 구조화하기 위해 사용     |
| 코드 생성    | `quote!`에서 쉼표를 포함한 코드 생성 시 `comma`가 필요함       |
| 사용 위치    | `proc_macro` 입력 파서 및 코드 생성 로직 내부                   |

---
