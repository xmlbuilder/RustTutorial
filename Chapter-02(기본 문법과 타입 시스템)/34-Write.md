# Write


## 🔍 출력 매크로 요약

| 매크로       | 설명 또는 특징                          |
|--------------|------------------------------------------|
| `format!`    | 문자열을 생성해서 반환 (출력은 하지 않음) |
| `eprintln!`  | 표준 에러 스트림에 출력 (디버깅용)        |
| `write!`     | `Write` 트레잇을 구현한 대상에 출력       |
| `writeln!`   | `write!` + 줄바꿈 포함 (`\n`)             |

## ✅ 사용 예시
### 1. format!: 문자열 생성
let msg = format!("User {} logged in at {}", user_id, timestamp);
// msg는 String 타입 → 로그 저장, UI 출력 등에 사용


### 2. eprintln!: 에러 출력
eprintln!("Failed to connect to server: {}", err);
// 표준 에러 스트림으로 출력됨 → 콘솔에서 빨간 글씨로 보일 수 있음


### 3. write! / writeln!: 파일 또는 스트림 출력
use std::fs::File;
use std::io::{Write, BufWriter};

let file = File::create("log.txt")?;
let mut writer = BufWriter::new(file);

write!(writer, "Start time: {}", start_time)?;
writeln!(writer, "End time: {}", end_time)?;
// 파일에 문자열이 저장됨



## 🧠 실무 팁
- format!은 출력하지 않고 반환하므로, 로그 버퍼나 UI 텍스트에 적합
- eprintln!은 표준 에러로 출력되므로, stderr 리디렉션 가능
- write!/writeln!은 파일, 네트워크, 메모리 버퍼 등 다양한 대상에 출력 가능
- 모든 write!류는 std::fmt::Write 또는 std::io::Write 트레잇을 구현한 타입에 사용 가능

---

# Debug Trait

format! 매크로에서 {:?} 또는 {:#?}처럼 디버그 포맷을 사용하려면, 해당 타입이 반드시 std::fmt::Debug 트레잇을 구현하고 있어야 합니다.

## ✅ 기본 타입은 자동 지원
- i32, f64, bool, char, String, Vec<T> 등 대부분의 기본 타입과 표준 컬렉션은 이미 Debug를 구현하고 있어서 {:?}로 바로 출력 가능합니다.
let v = vec![1, 2, 3];
let s = format!("{:?}", v); // OK



## ⚠️ 사용자 정의 타입은 직접 선언 필요
struct Point {
    x: f64,
    y: f64,
}

// 에러! Debug가 구현되지 않음
// let s = format!("{:?}", Point { x: 1.0, y: 2.0 });


## ✅ 해결 방법: #[derive(Debug)] 추가
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

let p = Point { x: 1.0, y: 2.0 };
let s = format!("{:?}", p); // OK



## 🧠 실무 팁
| 상황 또는 기능               | 설명 또는 필요 조건                         |
|-----------------------------|----------------------------------------------|
| `{:?}` 사용                 | `Debug` 트레잇이 구현된 타입만 출력 가능       |
| `#[derive(Debug)]` 선언     | 사용자 정의 타입에서 자동으로 `Debug` 구현     |
| `impl fmt::Debug for Type` | 커스텀 출력이 필요할 경우 직접 구현 가능       |



요약하자면:
format! 자체는 Debug를 요구하지 않지만, {:?} 포맷을 사용하면 그 타입이 Debug를 구현하고 있어야 한다는 거예요.


---

# Display 사용법

Display 트레잇은 Rust에서 사람이 읽기 좋은 형식으로 출력할 때 사용하는 트레잇. 
println!("{}", value)처럼 {} 포맷을 사용할 때, 해당 타입이 Display를 구현하고 있어야 합니다.
 기본 타입은 이미 구현돼 있지만, **사용자 정의 타입(구조체, 열거형 등)**은 직접 구현해줘야 함.

## 🧩 기본 구조
```rust
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

// Display 트레잇 구현
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 출력 형식 지정
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

- fmt() 함수는 write!() 매크로를 사용해 출력 내용을 f에 씁니다.
- fmt::Result는 std::fmt::Result와 동일하며, 보통 Ok(())로 반환합니다.

## ✅ 사용 예시
```rust
fn main() {
    let p = Point { x: 1.23, y: 4.56 };
    println!("{}", p); // → (1.23, 4.56)
}
```


## 🔍 포맷 옵션 활용
```rust
write!(f, "x: {:>6.2}, y: {:>6.2}", self.x, self.y)
```

- {:>6.2}: 오른쪽 정렬, 소수점 2자리, 총 너비 6칸
- 다양한 포맷 옵션은 format!과 동일하게 사용 가능

## 🧠 실무 팁


| 기능 또는 상황             | 설명 또는 필요 조건                       |
|----------------------------|--------------------------------------------|
| `println!("{}", value)`    | `Display` 트레잇이 구현돼 있어야 사용 가능  |
| `#[derive(Debug)]`         | `{:?}` 포맷 사용을 위한 자동 `Debug` 구현   |
| `Display` + `write!`       | `write!(f, "...")`로 사용자 정의 출력 구현  |
| `ToString`                 | `Display` 구현 시 `.to_string()` 자동 지원  |



### ⚠️ 주의할 점
- Display는 사람이 읽기 좋은 형식을 위한 것이고,
- Debug는 개발자 디버깅용으로 내부 상태를 빠르게 확인하는 용도입니다.
- Display는 직접 구현해야 하고, Debug는 #[derive(Debug)]로 자동 처리 가능

---

# Display trait 원리 설명

Rust의 Display 트레잇을 구현할 때 나오는 이 구조는 처음 보면 낯설고, 자꾸 잊게 되는 대표적인 문법 중 하나.
아래에 이 구조를 한 줄씩 해부해서 설명.

## 🧩 구조 전체
```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "출력할 내용")
}
```


## 🔍 한 줄씩 해부
### 1. fn fmt(...) -> fmt::Result
- Display 트레잇을 구현할 때 반드시 이 함수를 정의해야 함
- fmt는 Rust의 포맷팅 시스템에서 호출되는 함수 이름 (고정)
- 반환 타입은 fmt::Result → 보통 Ok(()) 또는 Err(...)로 처리
### 2. &self
- 출력 대상이 되는 자기 자신 (self)를 참조
- 예: Point { x: 1.0, y: 2.0 }라면 self.x, self.y를 출력에 사용
### 3. f: &mut fmt::Formatter<'_>
- 출력 내용을 담을 포맷터 객체
- write!(f, "...")를 통해 여기에 문자열을 써 넣음
- '_'는 **수명(lifetime)**을 자동 추론하라는 의미 → 신경 안 써도 됨
### 4. write!(f, "...")
- 실제 출력 내용을 작성하는 부분
- format!과 문법은 같지만, 결과를 f에 써 넣는다는 점이 다름

## ✅ 예제: 구조체에 Display 구현
```rust
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

이렇게 하면 println!("{}", Point { x: 1.0, y: 2.0 })가 (1.0, 2.0)으로 출력됩니다.

## 🧠 기억 팁
| 구성 요소         | 설명 또는 기억 방식                          |
|------------------|----------------------------------------------|
| `&self`          | 출력 대상 자기 자신                         |
| `f: &mut ...`    | 출력 버퍼 (여기에 문자열을 써 넣는다)        |
| `write!(f, ...)` | 실제 출력 내용을 작성하는 부분               |
| `fmt::Result`    | 반환 타입 → 보통 `Ok(())`으로 처리           |

이 구조는 “Rust에서 출력 형식을 커스터마이징할 때 쓰는 공식”이라고 생각하시면 돼요.

--- 
# print! / write! 차이

Rust에서 print! 계열과 write! 계열을 구별하는 이유는 단순히 출력 대상이 다르기 때문.  
파일에 쓰는 것 외에도, 이 둘은 철학과 용도가 다릅니다.

## 🧩 핵심 차이: 출력 대상
| 매크로               | 출력 대상            | 사용 예시                          | 특징 요약                     |
|----------------------|----------------------|------------------------------------|-------------------------------|
| `print!`, `println!` | 표준 출력 (`stdout`) | 콘솔 메시지, 사용자 안내           | 고정된 대상, 간편 출력         |
| `write!`, `writeln!` | 임의의 버퍼 (`Write`) | 파일, 메모리, 네트워크 스트림 등   | 유연한 대상, 트레잇 기반 출력 |



## 🧠 실무에서 왜 구별하나?
### 1. 표준 출력은 고정된 대상
- println!은 항상 콘솔로 출력됨
- 대상이 바뀌지 않음 → 유연성 부족
### 2. write!는 출력 대상이 유연함
- File, Vec<u8>, TcpStream, String, BufWriter 등 다양한 곳에 출력 가능
- 예: 로그를 파일에 쓰거나, UI 텍스트 버퍼에 넣거나, 네트워크로 전송할 때
``rust
use std::fmt::Write;

let mut s = String::new();
write!(s, "Hello {}", "Rust").unwrap();
// s == "Hello Rust"
```

### 3. 트레잇 기반 설계
- write!는 std::fmt::Write 또는 std::io::Write 트레잇을 구현한 타입에만 사용 가능
- 이 덕분에 출력 로직을 추상화할 수 있음 → 테스트, 리디렉션, 로깅에 유리

✅ 예시 비교
```rust
println!("Hello"); // 콘솔에 출력

use std::fs::File;
use std::io::Write;

let mut file = File::create("log.txt")?;
write!(file, "Hello")?; // 파일에 출력
```

## 🔧 실무 팁
| 상황 또는 용도           | 사용 매크로             |
|--------------------------|--------------------------|
| 표준 출력                | `println!`               |
| 표준 에러 출력 (`stderr`) | `eprintln!`              |
| 파일, 버퍼, 네트워크 등   | `write!`, `writeln!`     |
| 문자열 생성 (출력은 안 함) | `format!`                |


요약하자면:
print!는 고정된 콘솔 출력,
write!는 출력 대상이 유연해서 실무에서 훨씬 강력한 도구입니다.

---

# 메모리 버퍼

Rust에서는 메모리 버퍼를 직접 만들고 write! 매크로로 그 안에 문자열을 쓸 수 있습니다.  
이건 콘솔이나 파일이 아닌, String이나 Vec<u8> 같은 메모리 기반 버퍼에 포맷된 데이터를 누적할 때 아주 유용.

## 🧪 예제 1: String에 쓰기 (std::fmt::Write)
```rust
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    write!(buffer, "Hello, {}!", "Rust").unwrap();
    println!("{}", buffer); // → Hello, Rust!
}
```

- String은 std::fmt::Write 트레잇을 구현하고 있어서 write! 사용 가능
- format!과 달리 버퍼에 누적해서 여러 번 쓸 수 있음

## 🧪 예제 2: Vec<u8>에 쓰기 (std::io::Write)
```rust
use std::io::Write;

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    write!(buffer, "Data: {}", 42).unwrap();
    println!("{}", String::from_utf8(buffer).unwrap()); // → Data: 42
}
```

- Vec<u8>는 std::io::Write 트레잇을 구현
- 바이너리 데이터나 UTF-8 문자열을 다룰 때 유용

## 🧠 출력 대상별 트레잇 정리
| 버퍼 타입   | 구현된 트레잇     | 용도 예시                         |
|-------------|-------------------|-----------------------------------|
| `String`    | `fmt::Write`      | 텍스트 누적, 로그 버퍼, UI 메시지 |
| `Vec<u8>`   | `io::Write`       | 바이너리 출력, UTF-8 문자열 저장  |
| `BufWriter` | `io::Write`       | 파일 출력 최적화, 버퍼링 처리     |

이 방식은 특히 로그 시스템, 템플릿 엔진, UI 렌더링, 테스트용 출력 캡처 등에 아주 강력.


