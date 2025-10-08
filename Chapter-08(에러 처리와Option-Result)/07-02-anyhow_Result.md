# anyhow::Result
anyhow::Result는 Rust 언어에서 사용하는 에러 처리 도구 중 하나입니다.  
Rust의 표준 라이브러리에는 Result<T, E> 타입이 있지만, anyhow 크레이트를 사용하면 에러 타입을 더 유연하고 간단하게 다룰 수 있음.

## 🧭 anyhow::Result란?
```rust
use anyhow::Result;
```

- anyhow::Result<T>는 사실 Result<T, anyhow::Error>의 타입 별칭입니다.
- 즉, anyhow::Result<T>는 성공 시 T, 실패 시 anyhow::Error를 반환하는 구조.
- anyhow::Error는 다양한 에러 타입을 자동으로 래핑해주기 때문에, 여러 종류의 에러를 하나의 타입으로 처리할 수 있어요.

## ✅ 주요 특징
| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| 타입 단순화           | `Result<T>`는 내부적으로 `Result<T, anyhow::Error>`와 동일함           |
| 에러 통합             | 다양한 에러 타입을 `anyhow::Error`로 래핑하여 하나의 타입으로 처리 가능   |
| 디버깅 정보 포함       | `anyhow::Error`는 **백트레이스와 문맥 정보**를 포함할 수 있어 디버깅에 유리 |
| `?` 연산자와 궁합 좋음 | `?` 연산자를 사용해 에러를 간결하게 전파 가능                            |


## 🧪 예시 코드
```rust
use anyhow::{Result, anyhow};

fn might_fail(flag: bool) -> Result<String> {
    if flag {
        Ok("성공!".to_string())
    } else {
        Err(anyhow!("실패했습니다"))
    }
}

fn main() -> Result<()> {
    let msg = might_fail(true)?;
    println!("{}", msg);
    Ok(())
}
```

- anyhow!("실패했습니다")는 문자열을 에러로 래핑
- Result<()>는 main 함수에서도 에러를 간단하게 처리할 수 있게 해줌

## 🧠 언제 쓰면 좋을까?
- 애플리케이션 레벨에서 다양한 에러를 하나로 처리하고 싶을 때
- 복잡한 에러 타입 정의 없이 빠르게 개발하고 싶을 때
- **라이브러리보다는 실행 프로그램(main)** 에 적합

## ⚠️ 주의할 점
- anyhow::Result는 모든 에러를 하나로 묶기 때문에, 에러의 종류를 구분해서 처리하기는 어려움
- 라이브러리 개발 시에는 thiserror와 Result<T, MyError> 같은 명시적인 에러 타입이 더 적합

## 🧭 비교 요약: thiserror vs eyre vs std::error::Error
| 항목               | thiserror                          | eyre                                 | std::error::Error (기본)               |
|--------------------|------------------------------------|--------------------------------------|----------------------------------------|
| 목적               | 라이브러리용 커스텀 에러 타입 정의     | 애플리케이션용 간편 에러 처리           | Rust 기본 에러 trait                    |
| 에러 타입          | 명시적 enum 기반                    | `anyhow::Error`                      | 직접 구현한 struct/enum                |
| 주요 기능          | `#[derive(Error)]` 자동 구현         | `?`, `bail!`, `wrap_err` 등 간편 처리 | `Display`, `Debug`, `source()` 직접 구현 |
| 디버깅 정보        | `Debug`, `Display` 제공              | `anyhow::Error`에 백트레이스 포함 가능 | 기본 trait만 제공                      |
| 에러 구분          | ✅ 가능 (variant로 구분)             | ❌ 불가 (모든 에러가 래핑됨)            | ✅ 가능 (직접 타입 정의 시)             |
| 적합한 용도        | 라이브러리, API                     | CLI 앱, 빠른 개발                     | 저수준 구현, 의존성 최소화             |


---

# Sources:

## 🧪 예시 비교
### 1. thiserror 사용 예
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("파일 읽기 실패: {0}")]
    ReadError(std::io::Error),

    #[error("파싱 실패: {0}")]
    ParseError(std::num::ParseIntError),
}
```
- ? 연산자와 함께 map_err(MyError::ReadError)로 에러 변환 가능
- 에러 종류를 명확히 구분하고 처리 가능

### 2. eyre 사용 예
```rust
use eyre::{Result, eyre};
fn run() -> Result<()> {
    let file = std::fs::read_to_string("data.txt")
        .wrap_err("파일을 읽는 중 오류 발생")?;
    Ok(())
}
```

- wrap_err, bail! 등으로 간편하게 에러 생성 및 문맥 추가
- 에러 타입 구분은 불가능하지만 빠르고 유연함

### 3. std::error::Error만 사용할 경우
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "에러 발생!")
    }
}
impl Error for MyError {}

```

- Display, Debug, Error trait을 직접 구현해야 함
- 에러 타입 변환, 문맥 추가 등은 수작업 필요
- 실무에서는 유지보수가 어려움

### 🧠 Rust 기본만 사용할 경우의 차이점
- 에러 타입을 직접 정의하고 trait을 수동으로 구현해야 함
- ? 연산자는 사용 가능하지만, 에러 변환은 map_err 등으로 직접 처리해야 함
- 문맥 정보, 백트레이스, 에러 래핑 기능 없음
- 유지보수와 확장성이 떨어짐

### 🏁 결론
- 라이브러리 개발: thiserror로 명확한 에러 타입 정의
- 애플리케이션 개발: eyre 또는 anyhow로 빠르고 간단한 에러 처리
- Rust 기본만 사용: 학습 목적이나 의존성 최소화가 필요할 때만 추천

---


## 🧭 Result<T, Box<dyn Error>>란?
```rust
use std::error::Error;

fn do_something() -> Result<(), Box<dyn Error>> {
    // ...
}
```
- Result<T, E>는 Rust의 표준 에러 처리 타입
- Box<dyn Error>는 어떤 에러 타입이든 담을 수 있는 동적 에러 박스
- 즉, Result<T, Box<dyn Error>>는 성공 시 T, 실패 시 어떤 에러든 처리 가능한 구조

## ✅ 주요 특징
| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 다양한 에러 처리    | `io::Error`, `fmt::Error` 등 여러 에러 타입을 하나로 처리 가능             |
| trait 기반          | `dyn Error`는 `std::error::Error`를 구현한 모든 타입을 담을 수 있음         |
| 동적 디스패치       | 에러 타입을 런타임에 결정 (`Box<dyn Error>`로 힙에 저장)                   |
| 표준 라이브러리 사용 | 외부 크레이트 없이 Rust 기본만으로 유연한 에러 처리 가능                   |

## 🧪 예시 코드
```rust
use std::error::Error;
use std::fs::File;

fn open_file() -> Result<(), Box<dyn Error>> {
    let _f = File::open("missing.txt")?;
    Ok(())
}

fn main() {
    if let Err(e) = open_file() {
        println!("에러 발생: {}", e);
    }
}
```

- File::open은 std::io::Error를 반환하지만 Box<dyn Error>로 자동 변환됨
- ? 연산자와 함께 사용하면 에러 전파가 간결해짐

## 🔄 비교: Box<dyn Error> vs anyhow::Error
| 항목               | Box<dyn Error>                              | anyhow::Error                             |
|--------------------|---------------------------------------------|-------------------------------------------|
| 라이브러리 의존성   | ❌ 없음 (표준 라이브러리만 사용)              | ✅ `anyhow` 크레이트 필요                   |
| 에러 타입 유연성    | ✅ 다양한 에러 타입 처리 가능 (`dyn Error`)     | ✅ 모든 에러를 자동 래핑                    |
| 문맥 추가 기능      | ❌ 없음 (직접 구현 필요)                       | ✅ `context`, `wrap_err` 등으로 문맥 추가 가능 |
| 디버깅 정보         | ✅ `Display`, `Debug` 기본 제공                | ✅ `Display`, `Debug` + 백트레이스 포함 가능 |
| 에러 구분           | ✅ 다운캐스팅으로 타입 구분 가능                | ❌ 모든 에러가 `anyhow::Error`로 통합됨      |
| 적합한 용도         | 표준 기반 애플리케이션, 의존성 최소화 환경     | CLI 앱, 빠른 개발, 복잡한 에러 흐름 처리     |


## 🧠 언제 사용하면 좋을까?
- 외부 크레이트 없이 Rust 표준만으로 에러 처리하고 싶을 때
- 여러 에러 타입을 하나의 반환 타입으로 통합하고 싶을 때
- 라이브러리보다는 간단한 애플리케이션이나 학습용 프로젝트에 적합

---
# thiserror 에러 던지는 방법

## ✅ 에러 던지는 방법
### 1. map_err로 명시적으로 변환
```rust
use std::fs::File;
use std::num::ParseIntError;

fn read_file() -> Result<String, MyError> {
    let content = std::fs::read_to_string("data.txt")
        .map_err(MyError::ReadError)?;
    Ok(content)
}

fn parse_number(s: &str) -> Result<i32, MyError> {
    let num = s.trim().parse::<i32>()
        .map_err(MyError::ParseError)?;
    Ok(num)
}
```

- map_err(MyError::ReadError)로 std::io::Error를 MyError로 변환
- ? 연산자와 함께 사용하면 깔끔하게 에러 전파 가능

### 2. From 트레이트를 구현하면 자동 변환 가능
```rust
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::ReadError(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::ParseError(err)
    }
}


fn read_file() -> Result<String, MyError> {
    let content = std::fs::read_to_string("data.txt")?; // 자동 변환됨
    Ok(content)
}
```

- ? 연산자만으로도 자동으로 MyError로 변환됨
- thiserror는 #[from] 속성을 사용하면 이 구현을 자동으로 생성!

### 3. #[from] 속성으로 자동 변환
```rust
#[derive(Error, Debug)]
pub enum MyError {
    #[error("파일 읽기 실패: {0}")]
    #[from]
    ReadError(std::io::Error),

    #[error("파싱 실패: {0}")]
    #[from]
    ParseError(std::num::ParseIntError),
}
```

- 이렇게 하면 From 트레이트를 자동으로 구현해줌
- ? 연산자만으로도 에러를 MyError로 던질 수 있음

## 🧪 전체 예시
```rust
fn process() -> Result<i32, MyError> {
    let content = std::fs::read_to_string("data.txt")?; // ReadError
    let num = content.trim().parse::<i32>()?;           // ParseError
    Ok(num)
}
```

---


## ✅ 받은 쪽에서의 처리 방식
```rust
match process() {
    Ok(value) => {
        println!("성공: {}", value);
    }
    Err(e) => {
        println!("에러 발생: {}", e);
    }
}
```

### 또는 더 간단하게 if let을 사용할 수도 있음:
```rust
if let Err(e) = process() {
    println!("에러 처리: {}", e);
}
```

## 💡 어떤 에러 타입이든 동일하게 처리 가능
| Result 타입                    | 처리 방식         |
|-------------------------------|-------------------|
| `Result<T, MyError>`          | `match`, `if let` |
| `Result<T, anyhow::Error>`    | `match`, `if let` |
| `Result<T, Box<dyn Error>>`   | `match`, `if let` |

- Display와 Debug가 구현되어 있으면 println!("{}", e)로 출력 가능
- thiserror는 자동으로 Display와 Debug를 구현해주기 때문에 출력도 깔끔

---

