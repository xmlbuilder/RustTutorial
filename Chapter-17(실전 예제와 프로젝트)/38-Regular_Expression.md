# 🧠 Rust에서 정규 표현식 처리 방법
## ✅ 1. 설치 (Cargo.toml)
```
[dependencies]
regex = "1.10.2"
```


### ✅ 2. 기본 사용 예제
```rust
use regex::Regex;

fn main() {
    let text = "test@domain.com";
    let re = Regex::new(r"[\w.-]+@[\w.-]+\.[a-zA-Z]{2,}").unwrap();

    for mat in re.find_iter(text) {
        println!("Matched: {}", mat.as_str());
    }
}
```
### 출력 결과
```
Matched: test@domain.com
```

## 🔧 Java/C# → Rust 정규식 변환 예시

| 기능               | Java/C# 정규식                        | Rust 정규식 (`r"..."`)                  |
|--------------------|----------------------------------------|-----------------------------------------|
| 이메일 추출         | `[\\w.-]+@[\\w.-]+\\.[a-zA-Z]{2,}`     | `r"[\w.-]+@[\w.-]+\.[a-zA-Z]{2,}"`       |
| URL 추출           | `https?://[^\\s]+`                    | `r"https?://[^\s]+"`                    |
| 날짜 추출          | `\\d{4}-\\d{2}-\\d{2}`                 | `r"\d{4}-\d{2}-\d{2}"`                   |
| 한글 단어 추출      | `[가-힣]{2,}`                          | `r"[가-힣]{2,}"`                         |
| 숫자 추출          | `\\d+`                                 | `r"\d+"`                                 |
| 태그 제거          | `<[^>]+>`                              | `r"<[^>]+>"`                             |
| 확장자 추출        | `\\.(\\w+)$`                           | `r"\.(\w+)$"`                            |
| 반복 문자 감지      | `(\\w)\\1`                             | `r"(\w)\1"`                              |
| 카드번호 포맷팅    | `(\\d{4})(\\d{4})(\\d{4})(\\d{4})`     | `r"(\d{4})(\d{4})(\d{4})(\d{4})"`        |



## ✨ 고급 기능
### 🔹 그룹 캡처
```rust
let re = Regex::new(r"<([^<>]+)>").unwrap();
for cap in re.captures_iter("<값형><참조형>") {
    println!("Captured: {}", &cap[1]);
}
```
#### 출력 결과
```
Captured: 값형
Captured: 참조형
```

### 🔹 치환
```rust
let re = Regex::new(r"쪼끔씩|쪼금씩|쬐끔씩").unwrap();
let result = re.replace_all("쪼끔씩 진행해 보자", "조금씩");
println!("{}", result); // "조금씩 진행해 보자"
```

### 🔹 줄 단위 매칭 (멀티라인)
```rust
let re = Regex::new(r"(?m)^[a-zA-Z]{5,7}").unwrap();
for mat in re.find_iter("Word\nExcel\nPowerPoint") {
    println!("{}", mat.as_str());
}
```
### 출력 결과
```
Excel
PowerPo
```

### 🚧 주의사항
- Rust의 regex는 lookbehind를 지원하지 않음
- 유니코드 문자 클래스 (\p{Hangul} 등)도 제한적
- 멀티라인, DOTALL 등은 (?m), (?s)로 설정

---

# Rust 정규식 샘플 모음
## 📦 사전 준비: Cargo.toml
```
[dependencies]
regex = "1.10.2"
fancy-regex = "0.11"
```


## ✅ 1. 이메일 추출
```rust
use regex::Regex;

fn main() {
    let text = "test@domain.com";
    let re = Regex::new(r"[\w.-]+@[\w.-]+\.[a-zA-Z]{2,}").unwrap();

    for mat in re.find_iter(text) {
        println!("Email: {}", mat.as_str());
    }
}
```


## ✅ 2. 꺾쇠 괄호 안 텍스트 추출
```rust
fn main() {
    let text = "C#에는 <값형>과 <참조형>이라는 형이 존재합니다.";
    let re = Regex::new(r"<([^<>]+)>").unwrap();

    for cap in re.captures_iter(text) {
        println!("Tag content: {}", &cap[1]);
    }
}
```


## ✅ 3. 대소문자 구분 없는 단어 매칭
```rust
fn main() {
    let text = "kor, KOR, Kor";
    let re = Regex::new("(?i)\\bkor\\b").unwrap();

    for mat in re.find_iter(text) {
        println!("Matched: {}", mat.as_str());
    }
}
```


## ✅ 4. 줄마다 특정 길이의 단어 추출
```rust
fn main() {
    let text = "Word\nExcel\nPowerPoint\nOutlook\nOneNote";
    let re = Regex::new(r"(?m)^[a-zA-Z]{5,7}").unwrap();

    for mat in re.find_iter(text) {
        println!("Line match: {}", mat.as_str());
    }
}
```


## ✅ 5. 유사 표현 치환
```rust
fn main() {
    let text = "쪼끔씩 진행해 보자";
    let re = Regex::new(r"쪼끔씩|쪼금씩|쬐끔씩").unwrap();

    let result = re.replace_all(text, "조금씩");
    println!("{}", result);
}
```


## ✅ 6. 확장자 변경
```rust
fn main() {
    let text = "foo.htm bar.html baz.htm";
    let re = Regex::new(r"\.htm\b").unwrap();

    let result = re.replace_all(text, ".html");
    println!("{}", result);
}
```


## ✅ 7. 숫자 + 단위 치환
```rust
fn main() {
    let text = "1024바이트, 8바이트 문자, 바이트, 킬로바이트";
    let re = Regex::new(r"(\d+)바이트").unwrap();

    let result = re.replace_all(text, "$1byte");
    println!("{}", result);
}
```


## ✅ 8. 카드번호 포맷팅
```rust
fn main() {
    let text = "1234567890123456";
    let re = Regex::new(r"(\d{4})(\d{4})(\d{4})(\d{4})").unwrap();

    let result = re.replace_all(text, "$1-$2-$3-$4");
    println!("{}", result);
}
```


## ✅ 9. CSV-like 문자열 분리
```rust
fn main() {
    let text = "Word, Excel  ,PowerPoint   , Outlook,OneNote";
    let re = Regex::new(r"\s*,\s*").unwrap();

    let parts: Vec<&str> = re.split(text).collect();
    for part in parts {
        println!("Split: {}", part);
    }
}
```


## ✅ 10. 특정 패턴의 식별자 추출
```rust
fn main() {
    let text = "a123456 b123 z12345 AX98765";
    let re = Regex::new(r"\b[a-zA-Z][0-9]{5,}\b").unwrap();

    for mat in re.find_iter(text) {
        println!("ID: {}", mat.as_str());
    }
}
```


## ✅ 11. 한글 + 숫자 조합 추출
```rust
fn main() {
    let text = "삼겹살-84-58433, 상추-95838-488";
    let re = Regex::new(r"[가-힣]+-[0-9]{2,3}-[0-9]+").unwrap();

    for mat in re.find_iter(text) {
        println!("Matched: {}", mat.as_str());
    }
}
```


## ✅ 12. XML 태그 이름 추출
```rust
fn main() {
    let text = "<person><name>JungHwan Jeong</name><age>22</age></person>";
    let re = Regex::new(r"<(\w+?)>").unwrap();

    for cap in re.captures_iter(text) {
        println!("Tag: {}", &cap[1]);
    }
}
```


## ✅ 13. 반복 문자 감지
```rust
fn main() {
    let text = "도로를 지나가는 차들이 뛰뛰하고 경적을 울리면 반대쪽 차들이 빵빵하고 울렸다. bb";
    let re = fancy_regex::Regex::new(r"(\w)\1").unwrap();

    for mat in re.find_iter(text) {
        println!("Repeated: {}", mat.as_str());
    }
}
```


## ✅ 14. 회문 구조 감지 (ABA)
```rust
fn main() {
    let text = "기러기 펠리컨 pop";
    let re = fancy_regex::Regex::new(r"\b(\w)\w\1\b").unwrap();

    for mat in re.find_iter(text) {
        println!("Palindrome-like: {}", mat.as_str());
    }
}
```

---

