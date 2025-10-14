# 🧠 char 타입이란?
Rust의 char 타입은 단일 유니코드 스칼라 값을 나타냅니다.  
즉, 'A', '한', '🦄' 같은 문자 하나를 표현할 수 있어요.
- 크기: 4바이트 (32비트) 고정
- 표현 범위: U+0000부터 U+10FFFF까지의 유니코드 스칼라 값
- ASCII 문자뿐 아니라 한글, 일본어, 이모지 등 다양한 문자를 표현 가능

## 🔍 예제 분석
```rust
use std::mem::size_of;

fn main() {
    let c = 'A';                  // 타입 추론으로 char
    let z: char = 'B';           // 명시적 char 타입
    let unicorn = '🦄';          // 이모지도 char로 표현 가능

    println!("c = {c}");
    println!("z = {z}");
    println!("unicorn = {unicorn}");

    println!("size of char: {} bytes", size_of::<char>()); // 항상 4 bytes

    println!("sizeof a {}", "a".len()); // 문자열 "a"는 UTF-8로 1 byte

    let slice = "한글";
    println!(
        "slice is {} bytes and also {} characters.",
        slice.len(),             // UTF-8로 6 bytes
        slice.chars().count()    // 실제 문자 수는 2개
    );
}
```

🧩 핵심 차이점
| 표현                      | 설명                                      | 결과값 / 크기         |
|---------------------------|-------------------------------------------|------------------------|
| `char`                    | 단일 유니코드 문자 (4바이트 고정)         | 4 bytes                |
| `&str` `"..."`            | UTF-8 문자열 (가변 길이)                  | 길이에 따라 다름       |
| `"한글".len()`            | UTF-8 인코딩 기준 바이트 수               | 6 bytes (`한`=3, `글`=3) |
| `"한글".chars().count()`  | 실제 문자 수 (char 단위)                  | 2 characters           |



## 🛠️ 유용한 메서드들
Rust의 char 타입은 다양한 메서드를 제공합니다:
```rust
let ch = 'A';
println!("{}", ch.is_alphabetic()); // true
println!("{}", ch.is_numeric());    // false
println!("{}", ch.is_uppercase());  // true
println!("{}", ch.to_lowercase());  // 'a'
```


## 💡 참고 개념
- Rust의 char는 C/C++의 char와 다르게 고정 4바이트입니다.
- UTF-8 문자열은 문자마다 1~4바이트를 사용하므로, len()과 chars().count()는 다를 수 있어요.
- 이모지나 한글 같은 문자는 char로 표현 가능하지만, 문자열에서는 바이트 수가 더 커질 수 있어요.


## 🔍 요약
- char는 항상 4바이트로 고정된 크기를 가지며, 하나의 유니코드 문자를 표현합니다.
- &str은 UTF-8로 인코딩된 문자열로, 문자마다 바이트 수가 다를 수 있어요.
- "한글"은 6바이트지만 실제로는 2개의 문자를 담고 있어요 ('한', '글').
---

# Rust의 str과 char: 내부 구조와 변환 흐름 이해하기

Rust에서 str과 char는 모두 유니코드 기반이지만, 역할과 표현 방식이 다릅니다.  
이 둘이 어떻게 데이터를 주고받는지 이해하려면 내부 구조와 변환 흐름을 살펴보는 게 좋음.
| 타입   | 설명                                      |
|--------|-------------------------------------------|
| char   | 유니코드 문자 하나 (UTF-32, 고정 4바이트) |
| str    | UTF-8 인코딩된 문자열 슬라이스            |
| &str   | `str`에 대한 참조, 대부분의 문자열 처리에 사용 |

- char는 단일 문자 하나를 표현하며 항상 4바이트
- str은 여러 문자를 UTF-8로 압축해서 저장 (가변 길이)

## 🔄 변환 흐름
### 1️⃣ char → str
```rust
let c: char = '한';
let s: String = c.to_string(); // "한"
```

- char를 UTF-8로 인코딩 → str로 변환
- "한"은 UTF-8에서 3바이트로 표현됨
### 2️⃣ str → char
```rust
let s = "한글";
let first: char = s.chars().next().unwrap(); // '한'
```

- str를 UTF-8로 디코딩 → char 단위로 분리
- .chars()는 UTF-8을 순차적으로 디코딩해서 char로 반환

## 📦 내부 구조 차이
| 항목           | `char`                            | `str` / `&str`                          |
|----------------|------------------------------------|-----------------------------------------|
| 표현 대상      | 유니코드 문자 하나                | UTF-8 인코딩된 문자열 전체              |
| 크기           | 고정 4바이트 (UTF-32)             | 가변 길이 (1~4바이트/문자)              |
| 인코딩 방식    | UTF-32                            | UTF-8                                   |
| 메모리 구조    | 단일 값                           | 바이트 슬라이스 (`&[u8]`)               |
| 인덱싱         | 직접 가능 (`char`)                | 바이트 단위만 가능 (`str[0]` 불가)      |
| 반복 처리      | 불필요                            | `.chars()`로 UTF-8 디코딩 필요          |

⚠️ str[0]처럼 인덱싱은 불가능 → UTF-8은 가변 길이라 위험함


## ✅ 요약
- char는 유니코드 문자 하나, str은 UTF-8로 인코딩된 문자열
- 서로 변환할 때는 UTF-8 ↔ UTF-32 인코딩/디코딩이 자동으로 처리됨
- Rust는 이 과정을 안전하게 처리하며, .chars(), .to_string() 같은 메서드로 쉽게 변환 가능

---

# char의 용도

char 는 Rust에서 단일 유니코드 문자를 다룰 때 사용되며,  
실전에서는 문자 검사, 필터링, 패턴 매칭, 유효성 검사 등에 자주 활용됩니다.  
아래는 대표적인 실전 예제입니다.

## 🧪 실전 예제: 비밀번호 유효성 검사기
```rust
fn is_valid_password(pw: &str) -> bool {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in pw.chars() {
        if c.is_uppercase() {
            has_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else if "!@#$%^&*()-_=+[]{}".contains(c) {
            has_special = true;
        }
    }
    has_upper && has_lower && has_digit && has_special && pw.len() >= 8
}

fn main() {
    let pw = "RustLang#2025";
    if is_valid_password(pw) {
        println!("✅ 비밀번호가 유효합니다.");
    } else {
        println!("❌ 비밀번호가 유효하지 않습니다.");
    }
}
```

## 🔍 여기서 char가 하는 역할
- pw.chars()로 문자열을 문자 단위로 순회
- 각 char에 대해 is_uppercase, is_lowercase, is_ascii_digit 등 메서드 호출
- 특수 문자 포함 여부도 char 단위로 검사

## ✅ char가 실전에서 쓰이는 대표 용도
| 용도           | 예시 코드 / 설명                                      |
|----------------|--------------------------------------------------------|
| 문자 검사      | `c.is_alphabetic()`, `c.is_numeric()`, `c.is_whitespace()` |
| 패턴 매칭      | `match c { 'a'..='z' => … }` — 알파벳 범위 검사         |
| 문자 변환      | `c.to_ascii_uppercase()`, `c.to_digit(radix)`          |
| 유니코드 문자  | `'😊'`, `'한'`, `'ß'` — 다국어 및 이모지 문자 처리      |
| 입력 유효성    | 비밀번호, 사용자 입력, 토큰 검사 등                    |

## 🔍 보충 설명
- char는 단일 유니코드 문자이기 때문에 정확한 문자 단위 처리가 가능함
- match나 if 조건문에서 범위 검사할 때 매우 유용
- 유니코드 문자도 완벽하게 지원되므로 다국어 입력 처리에 적합

## 📌 요약
- char는 단일 문자 처리에 특화된 타입으로, 문자열을 세밀하게 다룰 때 필수
- str은 전체 문자열을 다루고, char는 그 내부 요소를 분석하는 데 사용됨
- 실전에서는 입력 검증, 텍스트 분석, 유니코드 처리에 자주 활용됨


---


