# toml parser

## 1. Cargo.toml에 의존성 추가
```
[dependencies]
toml = "0.5"
serde = { version = "1.0", features = ["derive"] }
```

## 2. 구조체 정의 및 TOML 파싱
```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn main() {
    let toml_str = r#"
        host = "localhost"
        port = 8080
    "#;

    let config: ServerConfig = toml::from_str(toml_str).unwrap();
    println!("Host: {}, Port: {}", config.host, config.port);
}
```
- TOML은 JSON보다 사람이 읽기 쉬운 설정 포맷으로, Rust에서 serde를 통해 구조체로 쉽게 매핑할 수 있습니다.

---

