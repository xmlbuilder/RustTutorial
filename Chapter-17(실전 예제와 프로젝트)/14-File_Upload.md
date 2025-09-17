# reqwest Crate

C에서 웹 서버에 데이터를 보내려면 socket, write, recv부터 시작해서 HTTP 헤더 수동 작성, Content-Length 계산, 엔코딩 처리, 에러 핸들링까지 전부 직접 구현 필요.
Rust는 이걸 한 줄로 끝낼 수 있습니다.

## 🚀 Rust에서 HTTP 요청 보내기: reqwest 크레이트
```rust

use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client.post("https://example.com/api")
        .header("Authorization", "Bearer my_token")
        .json(&serde_json::json!({
            "key1": "value1",
            "key2": "value2"
        }))
        .send()?;

    println!("응답 코드: {}", res.status());
    Ok(())
}
```


## ✅ 이게 가능한 이유
| 항목 또는 API        | 설명 또는 역할                                               |
|----------------------|--------------------------------------------------------------|
| `reqwest`            | 고수준 HTTP 클라이언트. TLS, 헤더, 바디, 쿠키 등 자동 처리     |
| `serde_json`         | JSON 직렬화/역직렬화 자동 지원. 구조체 → JSON 변환 가능        |
| `.json(&...)`        | Content-Type, Content-Length 자동 설정 + JSON 바디 구성         |
| `.send()`            | 요청 전송 + 응답 수신까지 한 번에 처리                         |
| `.status()`          | HTTP 상태 코드 바로 확인 가능 (`200`, `404`, `500` 등)         |



🔥 비교: C vs Rust
🧠 실무 비교: C vs Rust (`reqwest`)

| 작업 항목               | C 방식 (`socket`, `write`, 등)         | Rust 방식 (`reqwest`)                        |
|--------------------------|-----------------------------------------|----------------------------------------------|
| 연결 설정                | `socket()`, `connect()`                 | 자동 처리 (`Client::new()`)                  |
| 헤더 작성                | 문자열 수동 조합                       | `.header("Key", "Value")`                    |
| JSON 직렬화              | 수동 문자열 또는 외부 라이브러리        | `serde_json::json!({...})`                   |
| 요청 전송                | `write()` + `recv()` 반복               | `.send()` 한 줄로 처리                        |
| 응답 상태 코드 확인      | `recv()` 후 파싱                        | `.status()`로 바로 확인                      |
| 에러 처리                | `errno`, `strerror()`                   | `Result` + `?` 연산자로 깔끔하게 처리         |


## 🧠 실무 팁
- reqwest는 비동기 버전도 지원 (tokio 기반)
- blocking 버전은 CLI 툴이나 데몬에 적합
- multipart, form, cookie, redirect 등도 모두 지원
- hyper를 쓰면 더 저수준 제어도 가능 (C 스타일에 가까움)

요약하자면:

---

# File Upload

Rust에서는  크레이트를 사용하면 POST, GET, 그리고 멀티 파일 첨부. 

## 🚀 1. GET 방식 요청

```rust
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://httpbin.org/get")
        .query(&[("key1", "value1"), ("key2", "value2")])
        .send()?;

    println!("응답 코드: {}", res.status());
    println!("본문: {}", res.text()?);
    Ok(())
}
```

- .get()으로 GET 요청
- .query()로 URL 파라미터 추가
- .text()로 응답 본문 읽기

## 📦 2. POST 방식 요청 (JSON 전송)
```rust
use reqwest::blocking::Client;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.post("https://httpbin.org/post")
        .json(&json!({
            "username": "junghwan",
            "score": 42
        }))
        .send()?;

    println!("응답 코드: {}", res.status());
    println!("본문: {}", res.text()?);
    Ok(())
}
```

- .post()로 POST 요청
- .json()으로 JSON 바디 자동 직렬화
- Content-Type, Content-Length 자동 설정됨

## 📁 3. 파일 첨부 (멀티파트 업로드)

```rust
use reqwest::blocking::{Client};
use reqwest::multipart;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let file1 = fs::read("document1.pdf")?;
    let file2 = fs::read("image.jpg")?;

    let form = multipart::Form::new()
        .part("document1", multipart::Part::bytes(file1)
            .file_name("document1.pdf")
            .mime_str("application/pdf")?)
        .part("image", multipart::Part::bytes(file2)
            .file_name("image.jpg")
            .mime_str("image/jpeg")?)
        .text("description", "멀티 파일 업로드 예제");

    let res = client.post("https://httpbin.org/post")
        .multipart(form)
        .send()?;

    println!("업로드 상태: {}", res.status());
    Ok(())
}
```

- multipart::Form::new()으로 폼 생성
- .part()로 파일 추가
- .text()로 일반 필드도 함께 전송 가능


## ✅ 실무 팁: 웹 요청 핵심 API

| 작업 또는 목적           | 사용 API 또는 메서드                                |
|--------------------------|-----------------------------------------------------|
| GET 요청 + 파라미터 전달 | `.get().query()`                                    |
| POST 요청 + JSON 전송    | `.post().json()`                                    |
| 파일 첨부 (멀티파트)     | `multipart::Form.part().text()`                     |
| 응답 처리                | `.status()`, `.text()`, `.json()`                   |



GET, POST, 멀티파트 파일 업로드까지 하려면 Cargo.toml에서 기능(feature) 설정을 꼭 해줘야 합니다.  
특히 multipart, json, blocking 기능이 필요하죠.

## ⚙️ Cargo.toml 설정 예시
```
[dependencies]
reqwest = { version = "0.11", features = ["json", "multipart", "blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🧠 설명
| 크레이트 또는 기능       | 설명 또는 역할                                      |
|--------------------------|-----------------------------------------------------|
| `reqwest`                | HTTP 클라이언트. 웹 요청을 간단하게 처리할 수 있음     |
| `blocking`               | 동기 방식 요청을 가능하게 함. CLI, 데몬 등에 적합       |
| `json`                   | `.json()` 메서드 사용 가능. JSON 자동 직렬화 지원       |
| `multipart`              | 파일 첨부 및 폼 데이터 전송 기능 활성화                 |
| `serde`, `serde_json`    | 구조체 ↔ JSON 변환. 응답 파싱 및 요청 바디 구성에 필수   |


----
# 비동기 큰 파일 처리

Rust에서는 큰 파일을 비동기 방식으로 업로드한 뒤, 완료 이벤트를 감지하거나 후속 작업을 트리거하는 구조도 충분히 구현 가능합니다.
Rust는 C#이나 Java처럼 “이벤트”라는 개념이 언어에 내장되어 있진 않지만, 비동기 흐름을 제어하는 방식이 매우 강력합니다.

## ✅ Rust에서 비동기 업로드 후 완료 이벤트를 처리하는 방법
### 1. async/.await + .await로 완료 시점 감지
```rust
use reqwest::Client;
use tokio::fs::File;
use reqwest::multipart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let file = File::open("large_file.zip").await?;
    let stream = reqwest::Body::wrap_stream(tokio_util::io::ReaderStream::new(file));

    let part = multipart::Part::stream(stream)
        .file_name("large_file.zip")
        .mime_str("application/zip")?;

    let form = multipart::Form::new().part("file", part);

    let res = client.post("https://example.com/upload")
        .multipart(form)
        .send()
        .await?;

    println!("업로드 완료! 응답 코드: {}", res.status());
    // 여기서 후속 이벤트 트리거 가능
    Ok(())
}
```

- tokio 기반 비동기 런타임 사용
- .await로 업로드 완료 시점 감지
- 이후 로직에서 UI 반영, 알림, 로그 기록 등 가능

### 2. 업로드 완료 후 콜백처럼 처리하는 구조
Rust는 콜백보다는 Future 체이닝을 선호합니다. 예를 들어:
```rust
client.post("https://example.com/upload")
    .multipart(form)
    .send()
    .await?
    .text()
    .await
    .map(|body| {
        println!("서버 응답 본문: {}", body);
        // 여기에 이벤트 처리 로직 삽입
    });
```


### 3. 채널 기반 이벤트 전달 (tokio::sync::mpsc)
업로드 완료 후 다른 스레드나 UI에 알림을 보내고 싶다면, 채널을 통해 이벤트를 전달할 수 있음:
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(1);

// 업로드 작업
tokio::spawn(async move {
    let res = client.post(...).send().await.unwrap();
    tx.send("업로드 완료").await.unwrap();
});

// 이벤트 수신
if let Some(msg) = rx.recv().await {
    println!("이벤트 수신: {}", msg);
}

```


## 🧠 실무 팁: 비동기 이벤트 처리 핵심 기술

| 목적 또는 흐름             | 사용 기술 또는 API                                 |
|----------------------------|----------------------------------------------------|
| 비동기 작업 완료 감지       | `.await`                                           |
| 후속 작업 트리거           | `.map()`, `.then()`                                |
| 이벤트 전달 및 수신        | `tokio::sync::mpsc`, `broadcast`, `watch`          |
| 이벤트 루프 및 다중 처리   | `tokio::select!`, `stream`                         |

## ✨ 예시 흐름
- 파일 업로드 시작 → tokio::spawn으로 비동기 실행
- 업로드 완료 시점 → .await로 감지
- 후속 작업 실행 → .map() 또는 .then()으로 트리거
- 다른 컴포넌트에 알림 전송 → mpsc 채널로 메시지 전달
- 이벤트 루프에서 수신 → tokio::select!로 이벤트 감지 및 처리

요약하자면:
Rust는 “이벤트”라는 키워드 대신, Future의 완료 시점을 기준으로 정확하고 안전하게 후속 작업을 트리거합니다.

----

# Form 정보 보내기

## 🧰 Rust에서 Form 데이터를 서버로 보내는 방법
### 1. reqwest 크레이트 + application/x-www-form-urlencoded
# Cargo.toml
```
[dependencies]
reqwest = { version = "0.11", features = ["json", "multipart"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let mut form = HashMap::new();
    form.insert("username", "junghwan");
    form.insert("email", "junghwan@example.com");

    let res = client
        .post("https://example.com/submit")
        .form(&form)
        .send()
        .await?;

    println!("응답 상태: {}", res.status());
    Ok(())
}
```

- .form(&form) → application/x-www-form-urlencoded 형식으로 전송
- 서버에서는 일반적인 HTML <form> POST와 동일하게 처리 가능

### 2. JSON 형식으로 전송 (application/json)
```rust
let res = client
    .post("https://example.com/api")
    .json(&form)
    .send()
    .await?;

```
- 서버가 REST API라면 이 방식이 더 일반적
- Content-Type: application/json 자동 설정됨

### 3. 파일 포함 Form 전송 (multipart/form-data)
```rust
use reqwest::multipart;

let part = multipart::Part::text("junghwan");
let form = multipart::Form::new()
    .text("username", "junghwan")
    .part("profile", part); // 파일도 가능

let res = client
    .post("https://example.com/upload")
    .multipart(form)
    .send()
    .await?;
```

- HTML <form enctype="multipart/form-data">와 동일한 방식
- 파일 업로드, 이미지 전송 등에 사용

## 🔍 서버 측에서 어떻게 받는가?
- Flask, Express, Spring 등 대부분의 서버 프레임워크는
application/x-www-form-urlencoded, application/json, multipart/form-data 모두 지원합니다


## 🧠 요약: Rust에서 Form 데이터를 서버로 전송하는 방식

| 전송 방식              | Rust 코드 방식           | Content-Type 헤더 값               | 특징 및 용도                          |
|------------------------|--------------------------|------------------------------------|---------------------------------------|
| 일반 HTML Form 스타일   | `.form(&HashMap)`         | `application/x-www-form-urlencoded` | 간단한 키-값 쌍 전송, 로그인/검색 등  |
| REST API 스타일         | `.json(&HashMap)`         | `application/json`                 | 구조화된 데이터 전송, API 호출에 적합 |
| 파일 포함 Form          | `.multipart(Form)`        | `multipart/form-data`              | 이미지, 파일 업로드 등 복합 데이터 전송 |

---

# JSON형식의 폼

## ✅ JSON 형식의 Form 샘플 (Rust 코드)
### 1. HashMap 방식
```rust
use std::collections::HashMap;

let mut form = HashMap::new();
form.insert("username", "junghwan");
form.insert("email", "junghwan@example.com");
form.insert("age", "42");
```
```
→ 전송되는 JSON:
{
  "username": "junghwan",
  "email": "junghwan@example.com",
  "age": "42"
}
```


### 2. serde 구조체 방식 (더 안전하고 명확함)

```rust
use serde::Serialize;

#[derive(Serialize)]
struct FormData {
    username: String,
    email: String,
    age: u32,
}

let form = FormData {
    username: "junghwan".to_string(),
    email: "junghwan@example.com".to_string(),
    age: 42,
};
```
```
→ 전송되는 JSON:
{
  "username": "junghwan",
  "email": "junghwan@example.com",
  "age": 42
}
```

- 이 방식은 타입 안정성이 있고, 서버와의 계약이 명확해져서 유지보수에 유리해요
- serde는 Rust에서 JSON 직렬화/역직렬화의 표준입니다

## 🔧 전송 코드 예시
```rust
let res = client
    .post("https://example.com/api")
    .json(&form)
    .send()
    .await?;
```

서버는 Content-Type: application/json 헤더를 보고 JSON 본문을 파싱합니다.

---

