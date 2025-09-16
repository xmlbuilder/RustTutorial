# XML Parser

## 📄 XML 파싱 (quick-xml 사용)
### 📦 필요한 의존성 (Cargo.toml)
```
[dependencies]
quick-xml = "0.30"
serde = { version = "1.0", features = ["derive"] }
serde_derive = "1.0"
serde_xml_rs = "0.6"
```

### 💡 quick-xml는 빠르고 안전한 XML 파서이고, serde_xml_rs는 serde 기반으로 XML을 구조체에 매핑해줍니다.

## 🧾 예제 코드 (src/main.rs)
```rust
use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

fn main() {
    let xml_data = r#"
        <Book>
            <title>Rust in Action</title>
            <author>Tim McNamara</author>
            <year>2021</year>
        </Book>
    "#;

    let parsed: Book = from_str(xml_data).expect("Failed to parse XML");

    println!("Parsed Book: {:?}", parsed);
    println!("Title: {}", parsed.title);
}
```

## 🧪 JSON vs XML 파싱 예제 비교
| 항목               | JSON 파싱 예제                            | XML 파싱 예제                             |
|--------------------|-------------------------------------------|-------------------------------------------|
| 사용 크레이트       | `serde`, `serde_json`                     | `quick-xml`, `serde_xml_rs`               |
| 데이터 형식 예시    | `{ "id": 1, "name": "..." }`              | `<Book><title>...</title></Book>`         |
| 파싱 함수           | `serde_json::from_str()`                  | `serde_xml_rs::from_str()`                |
| 구조체 어노테이션   | `#[derive(Deserialize)]`                  | `#[derive(Deserialize)]`                  |


---



