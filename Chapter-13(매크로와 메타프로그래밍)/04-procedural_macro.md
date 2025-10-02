# Procedural Macro

## 🧠 개념: Procedural Macro란?
- 컴파일 타임에 실행되는 함수처럼 동작함
- Rust 코드(토큰 스트림)를 받아서 새로운 Rust 코드(토큰 스트림)를 반환함
- 코드 생성, 수정, 확장이 가능함
- 반드시 proc-macro 타입의 라이브러리 크레이트로 작성해야 함

## 🧩 종류: Rust Procedural Macro 유형
| 매크로 유형        | 사용 예시                        | 설명                                      |
|--------------------|----------------------------------|-------------------------------------------|
| Function-like       | `my_macro!(...)`                | 함수처럼 호출되는 매크로. 입력을 받아 코드 생성 |
| Derive macro        | `#[derive(MyTrait)]`            | 구조체/열거형에 트레잇 구현을 자동 생성       |
| Attribute macro     | `#[my_attr] fn foo() {...}`     | 함수, 구조체 등에 속성처럼 붙여서 동작 수정 또는 확장 |


## 🛠️ 기본 구조
### 1. Procedural Macro Crate 생성
```
cargo new my_macro --lib
```

Cargo.toml에 다음 추가:
```
[lib]
proc-macro = true
```


### 2. Function-like 매크로 예시
```rust
// lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn hello_macro(_input: TokenStream) -> TokenStream {
    "fn hello() { println!(\"Hello from macro!\"); }".parse().unwrap()
}
```

#### 사용 예:
```rust
hello_macro!(); // → hello() 함수가 생성됨
```


### 3. Derive 매크로 예시
```rust
#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // input: 구조체 정의 전체
    // 반환: impl MyTrait for 해당 구조체
    // 실제 구현은 syn + quote 크레이트로 파싱하고 생성함
}
```

#### 사용 예:
```rust
#[derive(MyTrait)]
struct MyStruct;
```


### 4. Attribute 매크로 예시
```rust
#[proc_macro_attribute]
pub fn my_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // item: 함수나 구조체 전체
    // attr: #[my_attr(값)]에서 괄호 안의 값
    item
}
```

#### 사용 예:
```rust
#[my_attr]
fn foo() {}
```



## 🔧 주요 크레이트: Procedural Macro 개발에 필수
| 크레이트       | 주요 역할 또는 기능                                |
|----------------|----------------------------------------------------|
| `proc_macro`   | 컴파일러가 제공하는 기본 API. `TokenStream` 타입을 통해 코드 입출력 처리 |
| `syn`          | `TokenStream`을 Rust의 AST(Abstract Syntax Tree)로 파싱 |
| `quote`        | Rust 코드 조각을 `TokenStream`으로 생성 (코드 생성기 역할) |


## 🎯 Procedural Macro 핵심 요약
| 구성 요소         | 설명                                      |
|------------------|-------------------------------------------|
| `proc-macro` 크레이트 | Procedural macro는 별도 라이브러리로 작성됨 |
| `#[proc_macro]`       | 함수형 매크로 정의                        |
| `#[proc_macro_derive]`| derive 매크로 정의                        |
| `#[proc_macro_attribute]` | 속성 매크로 정의                     |
| `TokenStream`         | 매크로 입출력 타입 (Rust 코드의 토큰 집합) |
| `syn`, `quote`        | 코드 파싱 및 생성 도구                    |




## 🎯 Procedural Macro가 만들어진 이유
### 1. 컴파일 타임 코드 생성 (메타 프로그래밍)
- 반복적이고 보일러플레이트한 코드를 자동으로 생성
- 예: #[derive(Serialize)]를 붙이면 impl Serialize가 자동 생성됨
- 개발자가 직접 구현하지 않아도 되므로 생산성과 유지보수성이 향상됨

### 2. 기존 매크로의 한계 극복
- macro_rules!는 패턴 매칭 기반이라 복잡한 구조를 다루기 어려움
- 예를 들어 구조체 필드 이름을 파싱하거나 트레잇을 자동 구현하는 건 불가능에 가까움
- Procedural Macro는 AST 수준에서 코드 분석 및 생성이 가능함

### 3. 라이브러리 생태계 확장
- serde, tokio, actix, diesel 같은 라이브러리들이 복잡한 트레잇 구현을 자동화
- 사용자 정의 트레잇을 #[derive(MyTrait)]로 간단히 적용 가능
- Rust 생태계의 표준화된 코드 생성 방식으로 자리잡음

### 4. 유지보수성과 안전성 향상
- 반복적인 구현을 줄이고, 실수 가능성을 낮춤
- 컴파일 타임에 오류를 잡을 수 있어 런타임 버그를 예방함
- compile_error!를 통해 사용자에게 명확한 피드백 제공 가능


---

# procedural macro 작동 원리

개발자가 구조체를 정의하고 #[derive(무언가)]를 붙이면, 컴파일 타임에 매크로가 자동으로 트레잇 구현 코드를 생성해줌.   
우리가 직접 #[derive(Hello)]를 만들면, 구조체에 hello() 메서드를 자동으로 추가해주는 매크로를 구현할 수 있음.

### 🧪 1. 프로젝트 구조
```
hello_macro/
├── Cargo.toml
├── src/
│   └── lib.rs         ← 매크로 정의
hello_macro_demo/
├── Cargo.toml
├── src/
│   └── main.rs        ← 매크로 사용
```


### 🛠️ 2. 매크로 크레이트 설정 (hello_macro/Cargo.toml)
```
[package]
name = "hello_macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```


### 🧠 3. 매크로 정의 (hello_macro/src/lib.rs)
```rust
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl Hello for #name {
            fn hello() {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };
    TokenStream::from(expanded)
}
```


### 🧩 4. 트레잇 정의 (hello_macro_demo/src/main.rs)
```rust
use hello_macro::Hello;

#[derive(Hello)]
struct MyStruct;

trait Hello {
    fn hello();
}

fn main() {
    MyStruct::hello();
}
```


#### ✅ 실행 결과
```
Hello from MyStruct!
```


## 🎯 요약: #[derive(Hello)] 매크로 동작 흐름
| 단계                | 설명                                      |
|---------------------|-------------------------------------------|
| `#[derive(Hello)]`   | 구조체에 Hello 트레잇 구현을 자동으로 요청함     |
| `impl Hello for ...` | Procedural Macro가 트레잇 구현 코드를 생성함     |
| `MyStruct::hello()`  | 생성된 메서드를 호출하여 동작 확인               |

---

## Attribute Procedural Macro 

Rust에서 직접 만들 수 있는 Attribute Procedural Macro의 실제적인 샘플. 
이 매크로는 함수에 `#[log_execution]` 을 붙이면, 해당 함수가 실행될 때 자동으로 로그를 출력해주는 기능. 
실전에서 디버깅, 로깅, 성능 측정 등에 아주 유용하게 쓰일 수 있음.

## 🧪 프로젝트 구조
```
log_macro/
├── Cargo.toml
├── src/
│   └── lib.rs         ← 매크로 정의
log_macro_demo/
├── Cargo.toml
├── src/
│   └── main.rs        ← 매크로 사용
```


## 🛠️ 1. 매크로 크레이트 설정 (log_macro/Cargo.toml)
```
[package]
name = "log_macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
```


## 🧠 2. 매크로 정의 (log_macro/src/lib.rs)
```rust
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_execution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;

    let expanded = quote! {
        #vis #sig {
            println!("🔍 실행 시작: {}", stringify!(#fn_name));
            let result = (|| #block )();
            println!("✅ 실행 완료: {}", stringify!(#fn_name));
            result
        }
    };
    TokenStream::from(expanded)
}
```


## 🧩 3. 매크로 사용 (log_macro_demo/src/main.rs)
```rust
use log_macro::log_execution;

#[log_execution]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let message = greet("JungHwan");
    println!("{}", message);
}
```


### ✅ 실행 결과
```
🔍 실행 시작: greet
Hello, JungHwan!
✅ 실행 완료: greet

```


## 🎯 요약: #[log_execution] Attribute 매크로 구성
| 구성 요소             | 설명                                      |
|----------------------|-------------------------------------------|
| `#[log_execution]`    | 함수에 붙여서 실행 전후 로그를 자동 출력함     |
| `proc_macro_attribute` | Attribute 매크로를 정의하는 핵심 어노테이션   |
| `syn::ItemFn`         | 함수 전체를 파싱해서 이름, 시그니처, 본문을 추출 |
| `quote!`              | 새로운 함수 본문을 생성하는 코드 생성기 역할     |


## 🔍 quote! 매크로란?
- Rust 코드처럼 생긴 템플릿을 작성하고
- 그 안에 #변수를 삽입해서 실제 코드 조각을 생성함
예를 들어:
```rust
quote! {
    pub fn hello() {
        println!("Hello!");
    }
}
```
이건 TokenStream으로 변환되어 컴파일러에게 전달.
이걸 Procedural Macro에서 쓰면, 원래 함수의 구조를 그대로 유지하면서 본문만 수정하거나 감싸는 작업이 가능.

## ✨ 예시 코드
```rust
let expanded = quote! {
    #vis #sig {
        println!("Start: {}", stringify!(#fn_name));
        let result = (|| #block )();
        println!("End: {}", stringify!(#fn_name));
        result
    }
};
```

이건 원래 함수의 vis, sig, block을 그대로 유지하면서, 실행 전후에 로그를 추가하는 매크로.

### 🎯 quote! 매크로 내 #vis, #sig 의미
| 표현     | 설명                                      |
|----------|-------------------------------------------|
| `#vis`   | 함수의 가시성 (예: `pub`)                  |
| `#sig`   | 함수의 시그니처 (이름, 인자, 반환 타입 포함) |
| `#block` | 함수 본문                                 |

---

### stringify!
Rust의 stringify! 매크로는 코드 조각을 문자열로 변환하는 매크로입니다.  
하지만 중요한 특징은:
실제 값이 아닌, 코드 자체를 문자열로 바꾼다.

---

# 매크로 log_execution 단계별 설명
매크로 log_execution은 함수 실행 전후에 로그를 출력하는 기능을 자동으로 삽입합니다.  
아래에 단계별로 구조와 동작 원리를 자세히 설명.

### 🧠 전체 목적
```rust
#[log_execution]
fn my_function() {
    // 실제 로직
}
```

이렇게 매크로를 붙이면 다음과 같은 코드로 확장됩니다:
```rust
fn my_function() {
    println!("🔍 실행 시작: my_function");
    let result = (|| {
        // 실제 로직
    })();
    println!("✅ 실행 완료: my_function");
    result
}
```
### 🧩 단계별 설명
####  🔹 1단계: 매크로 선언
```rust
#[proc_macro_attribute]
pub fn log_execution(_attr: TokenStream, item: TokenStream) -> TokenStream
```

- `#[proc_macro_attribute]`: 이 매크로는 attribute macro임을 나타냅니다.
- item: 매크로가 적용된 함수 코드가 TokenStream으로 들어옵니다.
- _attr: 매크로 인자 (사용하지 않으므로 _로 무시)

#### 🔹 2단계: 함수 파싱
```rust
let input = parse_macro_input!(item as ItemFn);
```

- item을 syn::ItemFn으로 파싱 → 함수 전체 구조를 AST로 변환
- ItemFn은 함수의 이름, 시그니처, 블록, 어트리뷰트 등을 포함

#### 🔹 3단계: 주요 정보 추출
```rust
let fn_name = &input.sig.ident;
let block = &input.block;
let sig = &input.sig;
let vis = &input.vis;

```
- `fn_name`: 함수 이름 (my_function)
- `block`: 함수 본문 { ... }
- `sig`: 함수 시그니처 (fn my_function())
- `vis`: 함수의 공개 여부 (pub, private 등)

#### 🔹 4단계: 코드 생성
```rust
let expanded = quote! {
    #vis #sig {
        println!("🔍 실행 시작: {}", stringify!(#fn_name));
        let result = (|| #block )();
        println!("✅ 실행 완료: {}", stringify!(#fn_name));
        result
    }
};
```

- `quote!`: Rust 코드 조각을 생성하는 매크로
- `stringify!(#fn_name)`: 함수 이름을 문자열로 출력
- `(|| #block )()`: 클로저로 감싸서 함수 본문을 실행하고 result에 저장
- `result`: 함수의 반환값을 그대로 유지

#### 🔹 5단계: 반환
```rust
TokenStream::from(expanded)
```

- 생성된 코드를 TokenStream으로 변환하여 컴파일러에 반환

### 📦 핵심 타입 요약
| 타입           | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| `TokenStream`  | 매크로 입출력용 코드 스트림. 컴파일러가 매크로에 전달하거나 반환함. |
| `ItemFn`       | 함수 전체를 표현하는 syn의 AST 타입. 이름, 시그니처, 본문 등을 포함. |
| `quote!`       | Rust 코드 조각을 생성하는 매크로. 변수 삽입과 코드 템플릿에 사용됨.  |
| `stringify!`   | 코드 조각을 문자열로 변환. 실제 값이 아닌 코드 자체를 문자열로 만듦. |



### 🧪 사용 예시
```rust
#[log_execution]
fn greet() -> &'static str {
    println!("Hello, JungHwan!");
    "완료"
}

fn main() {
    let msg = greet();
    println!("결과: {}", msg);
}
```

### 출력 결과:
```
🔍 실행 시작: greet
Hello, JungHwan!
✅ 실행 완료: greet
결과: 완료
```

---

# 🔍 stringify!의 역할
```rust
let s = stringify!(1 + 2);
println!("{}", s); // 출력: "1 + 2"
```

- stringify!(1 + 2)는 **"1 + 2"**라는 문자열을 생성합니다.
- 실제로 계산하지 않고, 코드 그대로를 문자열로 바꿉니다.

## 🧠 언제 쓰냐면…
| 용도                         | 설명                                                                 |
|------------------------------|----------------------------------------------------------------------|
| 디버깅용                     | 코드 조각을 그대로 문자열로 출력하고 싶을 때                         |
| 매크로 내부에서 코드 추적    | 어떤 식이 들어왔는지 문자열로 확인하고 싶을 때                       |
| 코드 생성용 매크로에서 활용  | `quote!`와 함께 사용해 코드 조각을 문자열로 삽입할 때                |
| 문서 자동 생성               | 코드 조각을 설명이나 주석으로 변환할 때                              |


### 🧪 예제: 매크로 내부에서 사용
```rust
macro_rules! log_expr {
    ($e:expr) => {
        println!("Evaluating: {}", stringify!($e));
        println!("Result: {}", $e);
    };
}

fn main() {
    log_expr!(3 * 4 + 1);
}
```

### 출력:
```
Evaluating: 3 * 4 + 1
Result: 13
```

- stringify!($e)는 "3 * 4 + 1"을 출력
- $e는 실제로 계산되어 13을 출력

### ⚠️ 주의할 점
- stringify!는 컴파일 타임에 문자열을 생성합니다.
- 변수 값을 문자열로 바꾸고 싶다면 format!이나 .to_string()을 사용해야 함.




