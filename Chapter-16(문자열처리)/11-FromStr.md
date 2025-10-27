# 🧠 FromStr란?
- std::str::FromStr 트레잇은 문자열을 특정 타입으로 파싱(parse) 하기 위한 표준 인터페이스입니다.
- 핵심 메서드는 from_str(s: &str) -> Result<Self, Self::Err>입니다.
- parse() 메서드는 내부적으로 FromStr을 호출합니다.

```rust
use std::str::FromStr;

let s = "42";
let n = i32::from_str(s).unwrap();      // 명시적 호출
let m: i32 = s.parse().unwrap();        // 암묵적 호출 (더 자주 사용됨)
```

## 🧪 커스텀 타입에 FromStr 구현 예제
```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x = x.trim().parse::<i32>().map_err(|_| ParsePointError)?;
        let y = y.trim().parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x, y })
    }
}

fn main() {
    let p: Point = "(3,7)".parse().unwrap();
    println!("{:?}", p); // 출력: Point { x: 3, y: 7 }
}
```

## ✅ 사용 시 주의점
- `FromStr`는 `소유권 타입` 만 지원합니다. &i32 같은 참조 타입으로 값을 반환 할 수 없습니다.
- Err 타입은 반드시 정의해야 하며, 보통 커스텀 에러 타입을 사용합니다.
- Display와 FromStr은 round-trip이 가능하도록 구현하는 것이 좋습니다.

---

# strip_prefix()

Rust의 strip_prefix()는 문자열에서 **특정 접두어(prefix)** 를 제거할 때 사용하는 메서드입니다.  

## 🧠 strip_prefix()란?
- strip_prefix(prefix: &str)는 문자열이 해당 prefix로 시작하면 그 접두어를 제거한 나머지 문자열을 Some(&str)로 반환합니다.
- 접두어가 없으면 None을 반환합니다.
```rust
fn main() {
    let s = "Hello, JungHwan!";
    if let Some(rest) = s.strip_prefix("Hello, ") {
        println!("남은 문자열: {}", rest); // 출력: JungHwan!
    } else {
        println!("접두어가 없습니다");
    }
}
```

## ✅ 특징 요약

| 항목           | 설명                                      |
|----------------|-------------------------------------------|
| 반환 타입      | `Option<&str>` → `Some(rest)` 또는 `None` |
| 원본 변경 여부 | ❌ 원본 문자열은 변경되지 않음             |
| 비교 방식      | 접두어가 정확히 일치해야 제거됨            |
| 대소문자 구분  | `"Rust"` ≠ `"rust"` (대소문자 구분함)     |



## 🧪 실전 예제
```rust
fn main() {
    let url = "https://example.com";
    let clean = url.strip_prefix("https://").unwrap_or(url);
    println!("URL without prefix: {}", clean); // 출력: example.com
}
```
- unwrap_or()를 쓰면 접두어가 없을 때도 안전하게 원본을 유지할 수 있음.

## 🔁 관련 메서드

| 메서드                | 설명                                                  |
|-----------------------|-------------------------------------------------------|
| `strip_suffix()`      | 문자열이 특정 접미어로 끝나면 그 부분을 제거하고 반환 |
| `starts_with()`       | 문자열이 특정 접두어로 시작하는지 여부를 확인         |
| `trim_start_matches()`| 접두어(또는 패턴)를 반복적으로 제거함                 |


## 🧪 간단 예제
```rust
fn main() {
    let s = "///path/to/file";

    println!("{:?}", s.strip_prefix("/"));         // Some("//path/to/file")
    println!("{:?}", s.strip_suffix("file"));      // Some("///path/to/")
    println!("{}", s.starts_with("///"));          // true
    println!("{:?}", s.trim_start_matches("/"));   // "path/to/file"
}
```



## 🧪 1. strip_prefix를 활용한 간단 파서 만들기
```rust
fn parse_command(input: &str) -> Option<&str> {
    input.strip_prefix("cmd:")
}

fn main() {
    let raw = "cmd:shutdown";
    match parse_command(raw) {
        Some(cmd) => println!("Parsed command: {}", cmd), // 출력: shutdown
        None => println!("Invalid command format"),
    }
}
```
- "cmd:" 접두어가 있는 문자열만 파싱해서 명령어 부분만 추출하는 간단 파서입니다.


## 🎯 2. match와 함께 쓰는 예제
```rust
fn main() {
    let input = "user:JungHwan";

    match input.strip_prefix("user:") {
        Some(name) => println!("Hello, {}!", name),
        None => println!("No user prefix found"),
    }
}
```
- match를 사용하면 Option<&str>을 깔끔하게 분기 처리할 수 있음.

## 🧠 3. Option 처리 방식 요약

| 방식            | 설명 또는 동작 조건                     |
|-----------------|------------------------------------------|
| `match`         | `Some`과 `None` 모두를 명시적으로 처리   |
| `if let`        | `Some`일 때만 처리, `None`은 무시 가능   |
| `unwrap_or()`   | `None`일 경우 기본값으로 대체            |
| `map()`         | `Some`일 때만 변환 함수 적용             |
| `and_then()`    | 중첩된 `Option`을 연결해 처리할 때 사용  |

## 예시: unwrap_or와 map
```rust
fn main() {
    let input = "lang:Rust";

    let lang = input
        .strip_prefix("lang:")
        .unwrap_or("Unknown");

    println!("Language: {}", lang); // 출력: Rust
}
```

## 🧪 간단 예제 모음
```rust
let maybe_name = Some("JungHwan");

// match
match maybe_name {
    Some(name) => println!("Hello, {}", name),
    None => println!("No name found"),
}

// if let
if let Some(name) = maybe_name {
    println!("Welcome, {}", name);
}

// unwrap_or
let name = maybe_name.unwrap_or("Guest");
println!("User: {}", name);

// map
maybe_name.map(|name| println!("Mapped name: {}", name));

// and_then
maybe_name.and_then(|name| Some(name.len())).map(|len| println!("Name length: {}", len));
```

## 💡 실전 팁
- strip_prefix는 파서 만들 때 접두어 제거 + 안전한 분기 처리를 동시에 해결해줘서 매우 유용
- match, if let, unwrap_or, map을 상황에 따라 적절히 조합하면 깔끔한 로직을 만들 수 있음


---

## 🧠 해석: "FromStr는 소유권 타입만 지원합니다"
- FromStr는 문자열을 새로운 값으로 파싱하는 트레잇입니다.
- 이때 반환되는 타입은 반드시 소유권을 가진 타입이어야 합니다.
- 예를 들어 i32, String, Vec<u8> 같은 타입은 OK
- 하지만 &i32, &str, &T 같은 참조 타입은 지원되지 않습니다.

## 🔍 왜 참조 타입은 안 되는가?
- FromStr은 문자열을 파싱해서 새로운 값을 생성하는 트레잇입니다.
- 참조 타입은 기존 데이터를 빌려오는 것이기 때문에  
  → 파싱 결과를 담을 메모리 소유자가 필요합니다.
- Rust의 안전성 모델상, FromStr은 소유권 있는 타입만 반환해야 안전하게 동작할 수 있음.

## ✅ 예시
```rust
use std::str::FromStr;

let num: i32 = i32::from_str("42").unwrap(); // OK
let s: String = String::from_str("hello").unwrap(); // OK

let r: &i32 = i32::from_str("42").unwrap(); // ❌ 컴파일 오류: 참조 타입은 안 됨
```

## 💬 결론
FromStr은 문자열을 파싱해서 새로운 값을 생성하는 트레잇이기 때문에  
반환 타입은 반드시 소유권을 가진 타입이어야 하고,  
&i32 같은 참조 타입으로 결과 값을 줄 수 없습니다.

---





