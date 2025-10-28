# 매크로
Rust의 매크로는 단순한 코드 치환을 넘어서 컴파일 타임에 코드 생성과 제어를 가능하게 하는 강력한 도구.  
Rust의 macro_rules!는 C의 `#define` 보다 훨씬 강력하고 안전한 패턴 기반 매크로 시스템.

## 🧠 매크로란?
- 컴파일 타임에 코드 생성을 수행하는 기능
- 반복되는 패턴을 줄이고, 다양한 입력을 처리하며, 런타임 비용 없이 코드 확장 가능
- 함수와 달리 ! 기호를 사용하며, 런타임이 아닌 컴파일 타임에 실행됨  
예: `println!`, `vec!`, `assert_eq!` 등은 모두 매크로

## 🧩 매크로의 종류
| 종류       | 예시 또는 설명                                |
|------------------------|-----------------------------------------------|
| `macro_rules!`         | 선언적 매크로. 패턴 기반 코드 생성             |
| `#[derive]` 매크로     | `#[derive(Debug)]`, `#[derive(Clone)]` 등 자동 trait 구현 |
| 속성(attribute) 매크로 | `#[test]`, `#[route]`, `#[tokio::main]` 등 함수/타입에 부가 동작 부여 |
| 함수(proc macro) 매크로| `my_macro!(...)` 형태. 사용자 정의 구문 처리 가능 |


## 🛠 macro_rules! 사용법
```rust
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}
```
### 사용법
```rust
// 매크로를 정의한 크레이트를 가져옵니다
use my_project::say_hello;
fn main() {
    say_hello!(); // Hello from macro! 출력
}
```

- `#[macro_export]` 를 붙이면 크레이트 루트에 등록되어 외부에서도 사용 가능
- 다양한 패턴을 정의해 입력에 따라 다른 코드를 생성 가능


## 🧠 매크로 구조
```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}
```

### 🧩 macro_rules! 문법 구성 예시

| 구성 요소             | 설명                          |
|------------------------|-------------------------------|
| `macro_rules! square` | `square`라는 이름의 매크로 정의 시작 |
| `($x:expr)`           | 입력 패턴: `$x`는 표현식(`expr`) 타입 |
| `=> { $x * $x }`      | 매크로 본문: `$x`를 두 번 곱한 결과로 치환 |
| `;`                   | 매크로 정의 종료 (여러 패턴 구분 시 사용) |


### 📌 $x:expr이란?
- `$x`는 매크로 인자 이름
- `:expr` 은 이 인자가 표현식(expression) 타입이어야 함을 의미
- 다른 패턴도 있음: `ident`, `ty`, `pat`, `block`, `tt`, `path`, `meta`, `item` 등

### ✅ 사용 예시
```rust
fn main() {
    let a = 3;
    let result = square!(a + 1); // (a + 1) * (a + 1)
    println!("{}", result); // 출력: 16
}

```
- 주의: 괄호 없이 쓰면 a + 1 * a + 1처럼 해석될 수 있으므로
- 매크로 내부에서 괄호로 감싸는 게 안전합니다:
```rust
macro_rules! square {
    ($x:expr) => {
        ($x) * ($x)
    };
}
```

## 🧪 프로시저 매크로 예시

### 🧪 프로시저 매크로란?
- macro_rules!는 패턴 기반 매크로
- proc_macro는 코드 생성 기반 매크로로,
- Rust 컴파일러가 입력된 토큰을 분석하고 새로운 코드를 생성할 수 있게 해줍니다.

```rust
// lib.rs
use proc_macro::TokenStream;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // 입력된 구조체를 분석하고 trait 구현 코드를 생성
}
```

### 🧪 프로시저 매크로 구성 요소 요약

| 코드 요소                     | 설명 또는 대응 사용 방식           |
|------------------------------|------------------------------------|
| `#[proc_macro_derive(MyTrait)]` | 사용자가 `#[derive(MyTrait)]`를 붙이면 이 매크로가 호출됨 |
| `input: TokenStream`         | 컴파일러가 구조체 정의를 토큰 스트림으로 전달 |
| `-> TokenStream`             | 매크로가 생성한 코드(impl 등)를 토큰 스트림으로 반환 |
| `proc_macro`                 | Rust 표준 라이브러리의 procedural macro API |

- proc_macro 크레이트 필요
```
[dependencies]
syn = "2.0"     # 입력된 Rust 코드(AST)를 파싱
quote = "1.0"   # Rust 코드 조각을 생성
proc-macro2 = "1.0" # 확장된 TokenStream (syn/quote와 호환)

[lib]
proc-macro = true
```
- syn, quote 라이브러리로 AST 파싱 및 코드 생성


### 🛠 실전 흐름: 어떻게 동작하나?
#### 1. 사용자 코드
```rust
#[derive(MyTrait)]
struct Person {
    name: String,
    age: u32,
}
```

#### 2. 컴파일러가 MyTrait 매크로 호출
- Person 구조체가 TokenStream으로 매크로 함수에 전달됨

#### 3. 매크로 내부에서 처리
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl MyTrait for #name {
            fn hello() {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    gen.into()
}
```

#### 4. 결과
컴파일러는 다음 코드를 자동 생성:
```rust
impl MyTrait for Person {
    fn hello() {
        println!("Hello from Person");
    }
}
```

## ⚠️ 매크로 사용 시 주의점
| 항목     | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| 디버깅 어려움                | 매크로는 컴파일 타임에 확장되므로 에러 위치 추적이 어려울 수 있음       |
| 이름 충돌 위험               | 전역 네임스페이스에 등록되므로 동일 이름의 매크로가 충돌할 수 있음       |
| 가독성 저하 가능             | 과도한 매크로 사용은 코드 흐름을 불명확하게 만들 수 있음                |
| `#[macro_export]`의 범위     | 루트 스코프에 등록되므로 모듈 경로와 무관하게 전역에서 접근됨            |
| `Copy`와 `Drop`은 양립 불가  | `Drop`을 구현한 타입은 `Copy`를 붙일 수 없음 → 매크로로 생성된 타입도 주의 필요 |


## ✨ 언제 매크로를 쓰면 좋은가?
- 반복되는 코드 패턴을 줄이고 싶을 때
- 다양한 입력을 유연하게 처리하고 싶을 때
- trait 구현, 테스트, 라우팅 등 코드 자동 생성이 필요할 때
- 런타임 비용 없이 컴파일 타임에 코드 확장이 필요할 때

## 🧭 사용자 입장에서 어떻게 시작하면 좋은가?
### ✅ 1. 기본 구조 익히기
- #[proc_macro_derive(...)] → Derive 매크로
- #[proc_macro_attribute] → Attribute 매크로
- #[proc_macro] → 함수형 매크로

### ✅ 2. syn으로 입력 파싱
- DeriveInput, ItemStruct, Field 등으로 구조체/열거형 분석

### ✅ 3. quote!로 코드 생성
- quote! { ... } 안에서 #name, #fields 등으로 동적 코드 생성

### ✅ 4. 테스트와 디버깅
- cargo expand로 매크로가 생성한 코드를 확인
- cargo test로 매크로가 잘 작동하는지 검증

## 📦 매크로 관리 팁
| 전략           | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| `macros.rs`                 | 공통 매크로를 한 파일에 모아두면 유지보수와 재사용에 유리함             |
| `#[macro_export]`           | 매크로를 크레이트 루트에 등록하여 외부에서도 사용할 수 있게 함          |
| 이름에 prefix 붙이기        | `math_approx_f64!`, `util_log!` 등으로 이름 충돌 방지                  |
| 내부 전용은 export 생략     | `macro_rules!`만 사용하고 `pub(crate)` 수준에서 제한하면 안전하게 관리 가능 |

---

# 매크로 선언

 외부 크레이트에서 macro_rules! 매크로를 사용하려면 정확한 위치에 선언과 설정이 필요.
 
## ✅ 1. 매크로 정의 크레이트에서 매크로를 사용하려면
| 위치 | 설명                                                              |
|------------------------------|-------------------------------------------------------------------|
| `macros.rs` 또는 `macros/mod.rs` | `macro_rules!`로 매크로 정의하고 `#[macro_export]`를 붙여야 루트에 등록됨 |
| `lib.rs`                     | `mod macros;` 선언을 통해 매크로 모듈을 루트에 포함시켜야 외부에서 접근 가능 |

```rust
// lib.rs
mod macros; // 루트에 등록해야 #[macro_export]가 작동함
```

```rust
// macros.rs
#[macro_export]
macro_rules! approx_f64 {
    ($a:expr, $b:expr, $eps:expr) => {
        (($a as f64) - ($b as f64)).abs() <= ($eps as f64)
    };
}
```

## ✅ 2. 외부 크레이트에서
|    위치    | 설명                                                              |
|------------------------------|-------------------------------------------------------------------|
| `Cargo.toml`                 | 매크로 정의 크레이트를 `dependencies`에 추가                       |
| 코드 상단                    | `use crate_name::macro_name;` 방식으로 매크로를 명시적으로 가져옴   |


```rust
use your_crate_name::approx_f64;

fn main() {
    assert!(approx_f64!(0.1 + 0.2, 0.3, 1e-9));
}
```

#### 🔎 `#[macro_use] extern crate ...;` 는 Rust 2015 스타일로, 지금은 명시적 `use` 방식이 권장

## ⚠️ 주의 요약
| 항목               | 설명                                                                 |
|----------------------------|----------------------------------------------------------------------|
| `#[macro_export]`          | 매크로를 크레이트 루트에 등록하여 외부에서 사용할 수 있게 함          |
| `mod macros;`              | 반드시 `lib.rs` 또는 `main.rs`에서 선언해야 루트 등록이 작동함         |
| `pub use` 불가             | `macro_rules!` 매크로는 `pub use`로 재수출할 수 없음                  |

---


