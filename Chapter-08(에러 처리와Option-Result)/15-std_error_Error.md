# std::error::Error

std::error::Error와 관련내용을 정리

## Rust 에러 처리 정리 (`std::error::Error` 중심)

### 1. 에러 처리 기본 구조
- Rust 표준 라이브러리에는 `std::error::Error` 트레잇이 있음
- 모든 에러 타입(`io::Error`, `fmt::Error`, 사용자 정의 에러 등`)은 이 트레잇을 구현 가능
- 범용적으로 에러를 다루려면 `Result<T, Box<dyn Error>>` 형태를 사용

```rust
use std::error::Error;

pub trait RunCommand {
    fn run(&mut self) -> Result<RunResult, Box<dyn Error>>;
}
```


### 2. map_err와 ? 연산자
- ?는 Result<T, E>에서 Err(e)를 만나면 그대로 전파
- map_err는 Err(e)를 다른 타입으로 변환할 때 사용
- 예시:
```rust
let writer = BdfWriter::new(&self.path)
    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
```

- 여기서 e는 BdfWriter::new가 반환한 std::io::Error
- 변환 후 Box<dyn Error>로 상위 함수에 전파됨

### 3. e의 정체
- e는 호출한 함수가 반환한 Result<T, E>의 Err 값
- 예: File::create(path)가 실패하면 Err(e) 반환, 이때 e는 std::io::Error
- 직접 생성도 가능:
```rust
use std::io::{self, Error, ErrorKind};

fn new(path: &str) -> Result<BdfWriter, io::Error> {
    if path.is_empty() {
        return Err(io::Error::new(ErrorKind::InvalidInput, "path is empty"));
    }
    Ok(BdfWriter { path: path.to_string() })
}
```

### 4. 사용자 정의 에러
- std::error::Error 트레잇을 구현하면 내가 만든 에러도 Err(e)로 던질 수 있음
```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError { details: msg.to_string() }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.details)
    }
}

impl Error for MyError {}
```

- 사용 예시:
```rust
fn do_something(flag: bool) -> Result<(), Box<dyn Error>> {
    if !flag {
        return Err(Box::new(MyError::new("flag was false")));
    }
    Ok(())
}
```


### 5. 요약
- std::error::Error는 범용 에러 인터페이스
- Box<dyn Error>로 감싸면 다양한 에러 타입을 한꺼번에 처리 가능
- map_err로 변환, ?로 전파
- e는 호출한 함수가 반환한 에러 객체
- 사용자 정의 에러도 Error 트레잇을 구현하면 동일하게 던질 수 있음

---

# anyhow
지금까지는 std::error::Error만을 쓰면서 Result<T, Box<dyn Error>> 형태로 에러를 다루고,  
map_err(|e| Box::new(e) as Box<dyn Error>)? 같은 변환을 직접 해줘야 했음.  
anyhow 크레이트는 이런 반복적인 변환 과정을 간소화해 줍니다.

## 🔍 std::error::Error 기반의 불편함
- 반환 타입을 항상 Result<T, Box<dyn Error>>로 써야 함
- ? 연산자 사용 시 에러 타입이 맞지 않으면 map_err로 변환해야 함
- 커스텀 에러 메시지를 붙이려면 io::Error::new 같은 걸 직접 써야 함
- 예시:
```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("missing.txt")
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(())
}
```

## ✅ anyhow가 간소화하는 방식

### 1. 에러 타입 통합
- anyhow::Result<T>는 사실상 Result<T, anyhow::Error>
- anyhow::Error는 모든 std::error::Error를 자동으로 래핑할 수 있음
- 따라서 map_err 없이 바로 ? 사용 가능
```rust
use anyhow::Result;

fn run() -> Result<()> {
    let file = std::fs::File::open("missing.txt")?; // 자동 변환
    Ok(())
}
```

### 2. 에러 메시지 추가 (Context)
- 에러가 발생했을 때 추가 설명을 붙일 수 있음
```rust
use anyhow::{Result, Context};

fn run() -> Result<()> {
    let file = std::fs::File::open("missing.txt")
        .context("파일을 열 수 없습니다")?;
    Ok(())
}
```

- 출력 시:
```
파일을 열 수 없습니다
Caused by: No such file or directory (os error 2)
```


### 3. 커스텀 에러 생성
- anyhow! 매크로로 간단히 에러를 만들 수 있음
```rust
use anyhow::{Result, anyhow};

fn run(flag: bool) -> Result<()> {
    if !flag {
        return Err(anyhow!("flag was false"));
    }
    Ok(())
}
```

## 🎯 요약
- std::error::Error만 쓰면 에러 변환을 직접 해줘야 해서 코드가 장황해짐
- anyhow는 모든 에러를 자동으로 anyhow::Error로 래핑 → ?만 써도 됨
- context로 추가 설명을 붙일 수 있어 디버깅 편리
- anyhow! 매크로로 커스텀 에러를 간단히 생성 가능
- 👉 즉, anyhow는 에러 타입 변환과 메시지 관리의 반복 작업을 줄여주는 도구.

---

