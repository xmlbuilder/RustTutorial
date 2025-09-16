# Option
Rust에서 가장 중요한 개념 중 하나인 Option<T> 타입을 보여주는 예제들

## 🧠 Option<T>란?
Rust에서는 null 값이 존재하지 않습니다. 대신 Option<T>이라는 열거형(enum)을 사용해 값이 있을 수도 있고 없을 수도 있는 상황을 안전하게 표현합니다.

enum Option<T> {
    Some(T),
    None,
}


## 📌 의미 요약
| Variant   | 값 존재 여부 | 포함된 값 | 설명                     |
|-----------|--------------|------------|--------------------------|
| Some(T)   | 있음         | T          | 값이 존재하며 T 타입을 포함 |
| None      | 없음         | 없음       | 값이 존재하지 않음         |


## 🔍 기본 사용 예시
```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
```

- Some(5)은 값이 있는 상태
- None은 값이 없는 상태
### match를 통한 패턴 매칭
```rust
match x {
    Some(n) => println!("x is {}", n),
    None => println!("x is not present!"),
}
```


## ✨ if let을 통한 간결한 매칭
```rust
if let Some(n) = x {
    println!("x is {}", n);
}
```
- match보다 간결하게 특정 값만 처리할 수 있음
- 이미지에서는 Some(3)일 때 if let을 사용해 "three"를 출력하는 예시가 있었어요

## ⚠️ unwrap()의 위험
```rust
let x = Some(5);
let y: Option<i32> = None;

println!("{}", x.unwrap()); // OK
println!("{}", y.unwrap()); // ❌ panic 발생
```

- unwrap()은 값이 없을 경우 프로그램을 강제 종료(panic) 시킵니다
- 따라서 실무에서는 unwrap_or, unwrap_or_default, expect를 사용하는 것이 안전합니다

## 🛡️ 안전한 대안들

### unwrap_or
```rust
let x = Some(5);
let y: Option<i32> = None;

println!("{}", x.unwrap_or(-1)); // 5
println!("{}", y.unwrap_or(-1)); // -1
```

### unwrap_or_default
```rust
let x: Option<u32> = None;
let y: Option<u32> = Some(12);

assert_eq!(x.unwrap_or_default(), 0);
assert_eq!(y.unwrap_or_default(), 12);
```


- 이미지에서는 "포함된 Some 값 또는 기본값을 반환"이라는 설명이 있었고,
- None일 경우 해당 타입의 기본값을 반환합니다 (u32의 기본값은 0)
### unwrap_or_else
```rust
let content = get_content().unwrap_or_else(|_| panic!("{}", FileNotDownload));
```

- 클로저를 통해 조건부 처리 가능
- 이미지에서는 "환경 캡처가 가능한 클로저를 통해 새 값을 계산"이라는 설명이 있었어요

### 💥 expect()의 권장 사용
```rust
let item = y.expect("slice should not be empty");
```

- unwrap()과 유사하지만, 사용자 정의 에러 메시지를 출력할 수 있음
- 디버깅 시 매우 유용하며, 실무에서 가장 많이 권장되는 방식입니다

## 실용 예제
```rust
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
fn get_content() -> Result<String, Error> {
    let mut content: String = String::new();
    let filePath = Path::new(std::env::current_dir().unwrap().parent().unwrap()).join("test.txt");
    File::open(filePath)?.read_to_string(&mut content)?;
    Ok(content)
}


#[derive(Debug, Clone)]
struct FileNotDownload;

impl fmt::Display for FileNotDownload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File it not yet downloaded")
    }
}

fn main() {
    let content = get_content().unwrap_or_else(|_| panic!("{}", FileNotDownload));
    println!("{}", content);
}

/*
File it not yet downloaded
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23/library\std\src\panicking.rs:652
   1: core::panicking::panic_fmt
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23/library\core\src\panicking.rs:72
   2: core::panicking::panic_display<sample2::FileNotDownload>
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23\library\core\src\panicking.rs:262
   3: sample2::main::closure$0::panic_cold_display<sample2::FileNotDownload>
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23\library\core\src\panic.rs:99
   4: sample2::main::closure$0
             at .\src\main.rs:26
   5: enum2$<core::result::Result<alloc::string::String,std::io::error::Error> >::unwrap_or_else<alloc::string::String,std::io::error::Error,sample2::main::closure_env$0>
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23\library\core\src\result.rs:1456
   6: sample2::main
             at .\src\main.rs:26
   7: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23\library\core\src\ops\function.rs:250
   8: core::hint::black_box
             at /rustc/3f5fd8dd41153bc5fdca9427e9e05be2c767ba23\library\core\src\hint.rs:338
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\sample2.exe` (exit code: 101)
 */
```


## 🧠 핵심 요약
| 메서드               | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| `unwrap()`           | 값이 없으면 panic 발생. 위험함                                       |
| `unwrap_or(default)` | 값이 없으면 지정한 기본값 반환                                       |
| `unwrap_or_default()`| 값이 없으면 타입의 기본값 반환                                       |
| `unwrap_or_else()`   | 클로저를 통해 동적으로 기본값 계산                                   |
| `expect(msg)`        | panic 발생 시 사용자 메시지 출력                                     |
| `match`              | 모든 경우를 안전하게 처리. 가장 기본적인 방식                        |
| `if let`             | 특정 값만 간결하게 처리할 때 사용                                     |

---


