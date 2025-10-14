# 📦 Base64 날짜 디코더
Base64로 인코딩된 날짜 문자열을 디코딩하여, 현재 날짜가 특정 범위 내에 있는지를 검사하는 간단한 Rust 예제입니다.  
현재 날짜가 특정 범위 내에 있는지를 검사하는 간단한 Rust 예제입니다.

## 🧩 주요 기능
- Base64로 인코딩된 날짜 문자열을 디코딩
- chrono::NaiveDate로 파싱
- 현재 날짜가 주어진 범위 내에 있는지 확인
- 유효성 검사 실패 시 경고 메시지 출력

## 📁 예제 코드
```rust
use chrono::{NaiveDate, Local};
use base64::{engine::general_purpose, Engine as _};

fn decode_date(encoded: &str) -> Option<NaiveDate> {
    let decoded = general_purpose::STANDARD.decode(encoded).ok()?;
    let date_str = String::from_utf8(decoded).ok()?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
}

fn main() {
    // 예시: base64로 인코딩된 날짜 문자열
    let encoded_start = "MjAxNS0xMi0yMw=="; // "2015-12-23"
    let encoded_end = "MjAyNS0xMi0zMQ==";   // "2025-12-31"

    let start_date = decode_date(encoded_start);
    let end_date = decode_date(encoded_end);
    let today = Local::now().date_naive();

    match (start_date, end_date) {
        (Some(start), Some(end)) => {
            if today >= start && today <= end {
                println!("✅ 성공: 오늘은 유효한 날짜 범위 안에 있습니다.");
            } else {
                println!("❌ 실패: 오늘은 유효한 날짜 범위 밖에 있습니다.");
            }
        }
        _ => println!("⚠️ 날짜 디코딩 실패: 입력 형식이 잘못되었거나 base64 오류입니다."),
    }
}
```
## 📦 Cargo.toml
```
[dependencies]
once_cell = "1.17"
chrono = "0.4"
base64 = "0.21"
```


## 🖥️ 실행 예시
```
$ cargo run
```

```
✅ 성공: 오늘은 유효한 날짜 범위 안에 있습니다.
```

## 📌 참고
- 날짜 형식은 반드시 "YYYY-MM-DD" 형식이어야 하며, Base64로 인코딩되어야 합니다.
- 디코딩 실패 시 Option::None을 반환하여 안전하게 처리됩니다.

---

# 암호화

Base64는 **암호화(encryption)** 가 아니라 인코딩(encoding) 방식이에요.  
즉, 데이터를 사람이 읽을 수 있는 문자열로 바꾸는 것이지, 보안이나 키 기반 보호는 제공하지 않음.

## 🔍 Base64는 암호화가 아니다
| 항목         | 설명                                         |
|--------------|----------------------------------------------|
| 목적         | 데이터를 사람이 읽을 수 있는 문자열로 변환   |
| 보안 기능    | ❌ 없음 — 누구나 디코딩 가능                  |
| 키 사용 여부 | ❌ 없음 — 암호화 키를 사용하지 않음           |
| 용도         | 전송/저장 편의성 (이진 → 텍스트)              |
| 예시         | 이미지, 날짜, 토큰 등을 문자열로 인코딩       |


## ✅ 암호화처럼 키를 쓰고 싶다면?
Base64 대신 암호화 알고리즘을 사용해야 함.    
예를 들어:
## 1️⃣ AES (대칭키 암호화)
```
[dependencies]
aes = "0.8"
block-modes = "0.8"
hex = "0.4"
```

- 키와 IV(초기화 벡터)를 사용해 데이터를 암호화/복호화
- 암호화된 결과를 Base64로 인코딩하면 전송 가능
- 
## 2️⃣ RSA (공개키 암호화)
- 공개키로 암호화하고, 개인키로 복호화
- 인증서 기반 보안에 사용됨

## 🧩 예시 흐름 (AES + Base64)
```rust
let encrypted_bytes = aes_encrypt(data, key, iv);
let encoded = base64::encode(encrypted_bytes);

let decoded = base64::decode(&encoded).unwrap();
let decrypted = aes_decrypt(decoded, key, iv);
```

- Base64는 단순히 암호화된 바이트를 문자열로 바꾸는 용도
- 보안은 AES가 담당, Base64는 전송 편의성만 담당

---

## 🔐 실전 예제: 암호화된 날짜 범위 검사기
### 📦 Cargo.toml
```
[dependencies]
aes = "0.8"
block-modes = "0.8"
block-padding = "0.3"
base64 = "0.21"
chrono = "0.4"
hex = "0.4"
```

### 🧪 코드 예시
```rust
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use chrono::{NaiveDate, Local};
use base64::{engine::general_purpose, Engine as _};

// 타입 정의
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// 16바이트 키와 IV (실전에서는 안전하게 관리해야 함)
const KEY: &[u8; 16] = b"0123456789abcdef";
const IV: &[u8; 16] = b"abcdef9876543210";

// 날짜를 암호화하고 Base64로 인코딩
fn encode_date(date: &str) -> String {
    let cipher = Aes128Cbc::new_from_slices(KEY, IV).unwrap();
    let encrypted = cipher.encrypt_vec(date.as_bytes());
    general_purpose::STANDARD.encode(encrypted)
}

// Base64 디코딩 후 AES 복호화
fn decode_date(encoded: &str) -> Option<NaiveDate> {
    let cipher = Aes128Cbc::new_from_slices(KEY, IV).ok()?;
    let decoded = general_purpose::STANDARD.decode(encoded).ok()?;
    let decrypted = cipher.decrypt_vec(&decoded).ok()?;
    let date_str = String::from_utf8(decrypted).ok()?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
}

fn main() {

    let encoded_start = encode_date("2015-12-23");
    let encoded_end = encode_date("2025-12-31");

    let start_date = decode_date(&encoded_start);
    let end_date = decode_date(&encoded_end);
    let today = Local::now().date_naive();

    match (start_date, end_date) {
        (Some(start), Some(end)) => {
            if today >= start && today <= end {
                println!("✅ 성공: 오늘은 유효한 날짜 범위 안에 있습니다.");
            } else {
                println!("❌ 실패: 오늘은 유효한 날짜 범위 밖에 있습니다.");
            }
        }
        _ => println!("⚠️ 날짜 디코딩 실패: 암호화 키 또는 형식 오류입니다."),
    }
}

### ✅ 특징
- AES128 + CBC + PKCS7 방식으로 안전하게 암호화
- Base64를 통해 암호화된 바이트를 문자열로 전송 가능
- 날짜 형식은 "YYYY-MM-DD"로 고정
- 키와 IV는 반드시 16바이트여야 함

### 🔐 예시 출력 (고정된 키와 IV 사용 시)
```rust
let encoded_start = encode_date("2015-12-23");
let encoded_end = encode_date("2025-12-31");

println!("Start: {}", encoded_start);
println!("End:   {}", encoded_end);
```

###  출력 예시 (고정 키/IV 기준):
```rust
Start: 3c4k0qZzK3u3vQz6Z3k9xw==
End:   6z+zZ2Qz9Zz9Z2Qz9Zz9Zg==
```

#### ⚠️ 위 값은 예시일 뿐이며, 실제 키와 IV에 따라 달라집니다.

### 📌 왜 Base64로 인코딩하나요?
- 암호화된 바이트는 바이너리 데이터이므로, 문자열로 안전하게 전송하거나 저장하려면 Base64로 인코딩해야 합니다.
- Base64는 보안 기능은 없지만, 암호화된 데이터를 텍스트로 표현하는 데 유용합니다.

---

