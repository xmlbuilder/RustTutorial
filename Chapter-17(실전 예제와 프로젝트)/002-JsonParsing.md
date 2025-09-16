# JSON Parsing

## 🧪 SON 파싱 (serde_json 사용)
### 📦 필요한 의존성 (Cargo.toml)
```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🧾 예제 코드 (src/main.rs)
```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let json_data = r#"
        {
            "id": 1,
            "name": "JungHwan",
            "email": "jung@example.com"
        }
    "#;

    let parsed: User = serde_json::from_str(json_data).expect("Failed to parse JSON");

    println!("Parsed User: {:?}", parsed);
    println!("Name: {}", parsed.name);
}
```
---
