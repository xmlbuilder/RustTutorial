# Date / Time

## 🧭 1. 표준 라이브러리: std::time
Rust 기본 라이브러리에서 제공하는 시간 관련 타입은 시스템 시간 기반이에요.
### 🔹 주요 타입
|  타입   | 설명                                      |
|---------------|-------------------------------------------|
| `SystemTime`  | 현재 시스템 시간을 나타냄 (`SystemTime::now()`) |
| `Duration`    | 시간 간격을 나타냄 (초, 밀리초 등)         |
| `Instant`     | 기준점에서 경과 시간을 측정하는 데 사용됨   |

### ✅ 예시
```rust
use std::time::{SystemTime, Duration};
let now = SystemTime::now();
let later = now + Duration::new(5, 0); // 5초 후
```

하지만 SystemTime은 날짜, 시간대, 포맷 처리가 부족해서
실제 날짜/시간을 다루려면 외부 crate가 필요해요.

## 📦 2. 외부 Crate: chrono 또는 time
### 🔹 chrono crate
가장 널리 쓰이는 날짜/시간 라이브러리.
시간대, 포맷, 파싱, 연산까지 거의 모든 기능을 지원해요.
```
# Cargo.toml
chrono = "0.4"
```

### ✅ 주요 타입
|  타입         | 설명                                                             |
|----------------------|------------------------------------------------------------------|
| `NaiveDate`          | 시간대 없는 날짜 (예: 2025-09-13)                                |
| `NaiveTime`          | 시간대 없는 시간 (예: 18:52:00)                                  |
| `NaiveDateTime`      | 시간대 없는 날짜 + 시간                                          |
| `DateTime<Utc>`      | UTC 기준 날짜/시간. 글로벌 서버나 로그 타임스탬프에 적합         |
| `DateTime<Local>`    | 로컬 시스템 시간대 기준 날짜/시간. 사용자 인터페이스에 적합      |


### ✅ 예시
```rust
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Utc};

let date = NaiveDate::from_ymd_opt(2025, 9, 13).unwrap();
let time = NaiveTime::from_hms_opt(18, 50, 0).unwrap();
let dt = NaiveDateTime::new(date, time);

let now = Utc::now();
println!("현재 시간: {}", now);
```

### ✅ 문자열 파싱
```rust
use chrono::NaiveDate;

let date = NaiveDate::parse_from_str("2025-09-13", "%Y-%m-%d").unwrap();
```


🔹 time crate
더 정밀하고 모던한 API를 제공하는 대안 라이브러리.
chrono보다 성능과 정확성에 집중한 설계.
time = "0.3"


### ✅ 예시
```rust
use time::{Date, Time, OffsetDateTime};

let date = Date::from_calendar_date(2025, time::Month::September, 13).unwrap();
let time = Time::from_hms(18, 50, 0).unwrap();
let datetime = date.with_time(time);
```

### 🕒 오늘 날짜 얻기 (time crate 기준)
```
use time::{OffsetDateTime, Date};

fn main() {
    let now = OffsetDateTime::now_local().unwrap(); // 로컬 시간 기준
    let today: Date = now.date(); // 날짜만 추출

    println!("오늘 날짜는: {}", today); // 예: 2025-09-13
}
```
#### 🔍 설명
- OffsetDateTime::now_local() → 현재 로컬 시간 (KST 등)
- .date() → Date 타입으로 날짜만 추출
- unwrap()은 시스템에서 로컬 시간대 정보를 못 가져올 경우를 대비한 처리


## 🧠 언제 어떤 걸 써야 할까?
| 🕒 방법       | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| `std::time`  | 표준 라이브러리. `SystemTime`, `Duration`, `Instant` 등 시간 측정용. 날짜/시간대 없음. |
| `chrono`     | 가장 널리 쓰이는 외부 crate. 날짜/시간, 포맷, 파싱, 시간대 지원. 직관적이고 강력함. |
| `time`       | 고정 시간대, 정밀도 중심의 모던한 라이브러리. 성능과 정확성에 집중. |
| 추천         | 일반적인 날짜/시간 처리에는 `chrono`가 가장 범용적으로 적합함.       |


---

# 날짜 체크 샘플 코드

입력은 "2015-12-23" 형식의 문자열이 encoding 되어 들어온다고 가정하고,
내부에서 decoding → 날짜 파싱 → 오늘 날짜와 비교 → 결과 출력까지 처리합니다.

## 🦀 Rust 샘플 코드: 날짜 범위 검사
```rust
use chrono::{NaiveDate, Local};
use base64::{engine::general_purpose, Engine};

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

### 🔍 설명
- base64 crate를 사용해 인코딩된 날짜 문자열을 디코딩
- chrono crate로 날짜 파싱 및 오늘 날짜 가져오기
- 날짜 범위 비교 후 결과 출력

## 📦 필요한 의존성
```
# Cargo.toml
chrono = "0.4"
base64 = "0.21"
```


## 🧪 실행 예시
```
$ cargo run
```
## 출력 결과
```
✅ 성공: 오늘은 유효한 날짜 범위 안에 있습니다.
```
이 코드는 실제 서비스에서 라이선스 유효 기간, 이벤트 기간 체크, 접근 권한 만료 같은 기능에 바로 응용할 수 있음.

---


## 🔐 키 기반 인코딩/디코딩 방식
### ✅ 1. XOR 방식 (간단한 키 기반 인코딩)
```rust
fn xor_encode(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}

fn xor_decode(encoded: &[u8], key: u8) -> String {
    encoded.iter().map(|&b| (b ^ key) as char).collect()
}
```

- key 값으로 XOR 연산을 수행
- 아주 간단한 방식이지만 보안성은 낮음
- 테스트용이나 임시 인코딩에 적합

### ✅ 2. AES 방식 (진짜 암호화)
```rust
use openssl::symm::{Cipher, Crypter, Mode};

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv)).unwrap();
    let mut ciphertext = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut ciphertext).unwrap();
    let rest = crypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count + rest);
    ciphertext
}

fn decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv)).unwrap();
    let mut plaintext = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut plaintext).unwrap();
    let rest = crypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count + rest);
    plaintext
}
```

- key와 iv를 넣어서 암호화/복호화
- 보안성이 높고 실제 서비스에서도 사용 가능
- openssl crate 필요

## 🧠 언제 어떤 방식?
|  목적    | 방식       | 특징                          | 추천 용도                         |
|------------------------|------------|-------------------------------|-----------------------------------|
| 테스트용 인코딩        | XOR 방식   | 간단하고 빠름, 보안은 없음    | 임시 키 기반 인코딩, 디버깅용     |
| 실제 암호화            | AES 방식   | 키와 IV 필요, 보안성 높음     | 민감 정보 보호, 인증 토큰 처리    |
| 단순 문자열 포맷 변환  | base64     | 키 없음, 인코딩/디코딩만 가능 | 파일 이름, 날짜 문자열 포맷 등    |

---


