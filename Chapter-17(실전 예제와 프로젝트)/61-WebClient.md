## 📘 WebClient 기능 정리 문서
- Rust 기반 HTTP 통합 유틸리티

### 📌 개요
- WebClient는 Rust의 reqwest, tokio, scraper, serde_json 등을 기반으로 만든 올인원 HTTP 클라이언트 유틸리티다.
- 하나의 구조체로 다음 기능을 모두 처리할 수 있다:
  - HTTP GET / POST
  - JSON 전송 및 파싱
  - Form 전송
  - Multipart 파일 업로드
  - Form + 파일 동시 업로드
  - 대용량 스트리밍 업로드
  - HTML 파싱 + CSS Selector
  - HTML attribute 추출
  - JSON key 접근
  - JSONPath 지원
  - 쿠키/세션 유지
  - 기본 헤더 자동 추가
  - Timeout 설정
- 웹 크롤링, API 호출, 파일 업로드, 자동화 작업 등 다양한 상황에서 활용 가능하다.

### ✅ 1. 구조체 생성
```rust
let client = WebClient::new("https://example.com");
```

#### 옵션: Timeout 설정
```rust
let client = WebClient::new("https://example.com")
    .with_timeout(10); // 10초
```


### ✅ 2. GET 요청
```rust
let client = WebClient::new("https://httpbin.org/get")
    .get()
    .await?;
```

- 응답은 자동으로 JSON 또는 HTML로 파싱된다.

### ✅ 3. POST JSON
```rust
let body = serde_json::json!({
    "name": "jung",
    "age": 20
});
```
```rust
let client = WebClient::new("https://httpbin.org/post")
    .post_json(&body)
    .await?;
```

### ✅ 4. POST Form (HashMap)
```rust
let mut form = HashMap::new();
form.insert("username", "junghwan");
form.insert("email", "test@example.com");

let client = WebClient::new("https://httpbin.org/post")
    .post_form(&form)
    .await?;
```


### ✅ 5. Multipart 파일 업로드
```rust
let client = WebClient::new("https://httpbin.org/post")
    .upload_files(vec![
        ("file1", "test1.txt", "text/plain"),
        ("file2", "test2.txt", "text/plain"),
    ])
    .await?;
```


### ✅ 6. Form + 파일 동시 업로드
```rust
let mut form = HashMap::new();
form.insert("title", "테스트 문서");
form.insert("author", "JungHwan");

let client = WebClient::new("https://httpbin.org/post")
    .upload_form_and_files(
        form,
        vec![("document", "doc.txt", "text/plain")]
    )
    .await?;
```

### ✅ 7. 대용량 스트리밍 업로드
```rust
let client = WebClient::new("https://example.com/upload")
    .upload_stream("file", "large.zip", "application/zip")
    .await?;
```


### ✅ 8. HTML 파싱 (CSS Selector)
```rust
let client = WebClient::new("https://www.rust-lang.org")
    .get()
    .await?;

let titles = client.select("title");
```


### ✅ 9. HTML attribute 추출
```rust
let links = client.attr("a", "href");
```


### ✅ 10. JSON key 접근
```rust
let value = client.json_key("slideshow");
```


### ✅ 11. JSONPath 지원
```rust
let title = client.json_path("slideshow.slides[0].title");
```


### ✅ 12. 쿠키/세션 유지
- WebClient는 내부적으로 다음 설정을 사용한다:
```rust
ClientBuilder::new()
    .cookie_store(true)
```

- 즉:
  - 로그인 후 세션 유지 가능
  - 여러 요청 간 쿠키 자동 전송
  - 인증 기반 사이트 크롤링 가능

### ✅ 13. 기본 헤더 자동 추가
```
User-Agent: WebClient/1.0
Accept: */*
```
- 필요하면 확장 가능.

### ✅ 14. HTML/JSON 자동 판별
- 응답 본문이 {로 시작하면 JSON, 그 외는 HTML로 자동 처리한다.

### ✅ 15. 전체 기능 요약 표
| 기능                         | 지원 |
|------------------------------|------|
| GET 요청                     | ✅   |
| POST JSON                    | ✅   |
| POST Form(HashMap)           | ✅   |
| Multipart 파일 업로드        | ✅   |
| Form + 파일 동시 업로드      | ✅   |
| 대용량 스트리밍 업로드       | ✅   |
| HTML 파싱                    | ✅   |
| CSS Selector                 | ✅   |
| HTML attribute 추출          | ✅   |
| JSON key 접근                | ✅   |
| JSONPath 지원                | ✅   |
| 쿠키/세션 유지               | ✅   |
| 기본 헤더 자동 추가          | ✅   |
| Timeout 설정                 | ✅   |


### ✅ 16. Cargo.toml 설정
```
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "multipart", "cookies", "gzip", "brotli", "deflate", "rustls-tls"] }
scraper = "0.18"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio-util = "0.7"
tokio-stream = "0.1"
rand = "0.8"
```


### ✅ 18. 소스 코드
```rust
use reqwest::{Client, ClientBuilder, multipart, header};
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;
use tokio_util::io::ReaderStream;
use tokio::fs::File;
use std::time::Duration;
```
```rust
pub struct WebClient {
    client: Client,
    url: String,
    html: Option<Html>,
    json: Option<Value>,
}
```
```rust
impl WebClient {
    pub fn new(url: impl Into<String>) -> Self {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .default_headers(Self::default_headers())
            .timeout(Duration::from_secs(30)) // ✅ 기본 timeout 30초
            .build()
            .unwrap();

        Self {
            client,
            url: url.into(),
            html: None,
            json: None,
        }
    }
```
```rust
    // ✅ Timeout 설정 기능
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.client = ClientBuilder::new()
            .cookie_store(true)
            .default_headers(Self::default_headers())
            .timeout(Duration::from_secs(secs))
            .build()
            .unwrap();
        self
    }
```
```rust
    fn default_headers() -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("WebClient/1.0"));
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("*/*"));
        headers
    }
```
```rust
    // ---------------------------
    // GET
    // ---------------------------
    pub async fn get(mut self) -> Result<Self, reqwest::Error> {
        let resp = self.client.get(&self.url).send().await?;
        let text = resp.text().await?;
        self.parse_body(text);
        Ok(self)
    }
```
```rust
    // ---------------------------
    // POST JSON
    // ---------------------------
    pub async fn post_json(mut self, body: &Value) -> Result<Self, reqwest::Error> {
        let resp = self.client.post(&self.url).json(body).send().await?;
        let text = resp.text().await?;
        self.parse_body(text);
        Ok(self)
    }
```
```rust
    // ---------------------------
    // POST Form(HashMap)
    // ---------------------------
    pub async fn post_form(mut self, form: &HashMap<&str, &str>) -> Result<Self, reqwest::Error> {
        let resp = self.client.post(&self.url).form(form).send().await?;
        let text = resp.text().await?;
        self.parse_body(text);
        Ok(self)
    }
```
```rust
    // ---------------------------
    // Multipart 파일 업로드
    // ---------------------------
    pub async fn upload_files(
        mut self,
        files: Vec<(&str, &str, &str)>
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let mut form = multipart::Form::new();

        for (field, path, mime) in files {
            let bytes = tokio::fs::read(path).await?;
            let part = multipart::Part::bytes(bytes)
                .file_name(path.to_string())
                .mime_str(mime)?;
            form = form.part(field.to_string(), part);
        }

        let resp = self.client.post(&self.url).multipart(form).send().await?;
        let text = resp.text().await?;
        self.parse_body(text);
        Ok(self)
    }
```
```rust
    // ---------------------------
    // Multipart: Form + Files 업로드
    // ---------------------------
    pub async fn upload_form_and_files(
        mut self,
        form_fields: HashMap<&str, &str>,
        files: Vec<(&str, &str, &str)>
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let mut form = multipart::Form::new();

        for (key, value) in form_fields {
            form = form.text(key.to_string(), value.to_string());
        }

        for (field, path, mime) in files {
            let bytes = tokio::fs::read(path).await?;
            let part = multipart::Part::bytes(bytes)
                .file_name(path.to_string())
                .mime_str(mime)?;
            form = form.part(field.to_string(), part);
        }

        let resp = self.client.post(&self.url).multipart(form).send().await?;
        let text = resp.text().await?;
        self.parse_body(text);
        Ok(self)
    }
```
```rust
    // ---------------------------
    // 대용량 스트리밍 업로드
    // ---------------------------
    pub async fn upload_stream(
        mut self,
        field: &str,
        path: &str,
        mime: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let file = File::open(path).await?;
        let stream = ReaderStream::new(file);

        let part = multipart::Part::stream(reqwest::Body::wrap_stream(stream))
            .file_name(path.to_string())
            .mime_str(mime)?;

        let form = multipart::Form::new().part(field.to_string(), part);

        let resp = self.client.post(&self.url).multipart(form).send().await?;
        let text = resp.text().await?;
        self.parse_body(text);
        Ok(self)
    }
```
```rust
    // ---------------------------
    // HTML selector
    // ---------------------------
    pub fn select(&self, selector: &str) -> Vec<String> {
        let html = match &self.html {
            Some(h) => h,
            None => return vec![],
        };

        let sel = Selector::parse(selector).unwrap();
        html.select(&sel)
            .map(|e| e.text().collect::<Vec<_>>().join(" "))
            .collect()
    }
```
```rust
    // ✅ HTML attribute 추출
    pub fn attr(&self, selector: &str, attr: &str) -> Vec<String> {
        let html = match &self.html {
            Some(h) => h,
            None => return vec![],
        };

        let sel = Selector::parse(selector).unwrap();
        html.select(&sel)
            .filter_map(|e| e.value().attr(attr).map(|v| v.to_string()))
            .collect()
    }
```
```rust
    // ---------------------------
    // JSON key 접근
    // ---------------------------
    pub fn json_key(&self, key: &str) -> Option<&Value> {
        self.json.as_ref()?.get(key)
    }
```
```rust
    // ✅ JSONPath 지원 (간단 버전)
    pub fn json_path(&self, path: &str) -> Option<&Value> {
        let mut current = self.json.as_ref()?;

        for part in path.split('.') {
            if let Some(idx_start) = part.find('[') {
                // 배열 접근: key[index]
                let key = &part[..idx_start];
                let idx_end = part.find(']').unwrap();
                let idx: usize = part[idx_start + 1..idx_end].parse().ok()?;

                current = current.get(key)?.get(idx)?;
            } else {
                // 일반 key 접근
                current = current.get(part)?;
            }
        }

        Some(current)
    }
```
```rust
    // ---------------------------
    // HTML/JSON 자동 판별
    // ---------------------------
    fn parse_body(&mut self, text: String) {
        if text.trim_start().starts_with('{') {
            self.json = serde_json::from_str(&text).ok();
        } else {
            self.html = Some(Html::parse_document(&text));
        }
    }
}
```

### ✅ 17. 테스트 코드

```rust
use serde_json::json;
use std::collections::HashMap;
use nurbslib::core::web_client::WebClient;
```
```rust
//
// ✅ 1. GET 요청 테스트
//
#[tokio::test]
async fn test_get_request() {
    let client = WebClient::new("https://httpbin.org/get")
        .get()
        .await
        .expect("GET 요청 실패");

    assert!(client.json_key("url").is_some());
}
```
```rust
//
// ✅ 2. POST JSON 테스트
//
#[tokio::test]
async fn test_post_json() {
    let body = json!({
        "name": "jung",
        "age": 20
    });

    let client = WebClient::new("https://httpbin.org/post")
        .post_json(&body)
        .await
        .expect("POST JSON 실패");

    assert_eq!(
        client.json_path("json.name").unwrap(),
        "jung"
    );
}
```
```rust
//
// ✅ 3. POST Form(HashMap) 테스트
//
#[tokio::test]
async fn test_post_form() {
    let mut form = HashMap::new();
    form.insert("username", "junghwan");
    form.insert("email", "test@example.com");

    let client = WebClient::new("https://httpbin.org/post")
        .post_form(&form)
        .await
        .expect("POST Form 실패");

    assert_eq!(
        client.json_path("form.username").unwrap(),
        "junghwan"
    );
}
```
```rust
//
// ✅ 4. Multipart 파일 업로드 테스트
//
#[tokio::test]
async fn test_upload_files() {
    // 테스트용 파일 생성
    tokio::fs::write("test1.txt", "hello").await.unwrap();
    tokio::fs::write("test2.txt", "world").await.unwrap();

    let client = WebClient::new("https://httpbin.org/post")
        .upload_files(vec![
            ("file1", "test1.txt", "text/plain"),
            ("file2", "test2.txt", "text/plain"),
        ])
        .await
        .expect("파일 업로드 실패");

    assert!(client.json_key("files").is_some());
}
```
```rust
//
// ✅ 5. Multipart Form + Files 업로드 테스트
//
#[tokio::test]
async fn test_upload_form_and_files() {
    tokio::fs::write("doc.txt", "document").await.unwrap();

    let mut form = HashMap::new();
    form.insert("title", "테스트 문서");
    form.insert("author", "JungHwan");

    let client = WebClient::new("https://httpbin.org/post")
        .upload_form_and_files(
            form,
            vec![("document", "doc.txt", "text/plain")]
        )
        .await
        .expect("Form + 파일 업로드 실패");

    assert!(client.json_key("form").is_some());
    assert!(client.json_key("files").is_some());
}
```
```rust
//
// ✅ 6. 대용량 스트리밍 업로드 테스트
//
#[tokio::test]
async fn test_upload_stream() {
    // 1MB 테스트 파일 생성
    let data = vec![0u8; 1024 * 1024];
    tokio::fs::write("big.bin", &data).await.unwrap();

    let client = WebClient::new("https://httpbin.org/post")
        .upload_stream("file", "big.bin", "application/octet-stream")
        .await
        .expect("스트리밍 업로드 실패");

    assert!(client.json_key("files").is_some());
}
```
```rust
//
// ✅ 7. HTML selector 테스트
//
#[tokio::test]
async fn test_html_selector() {
    let client = WebClient::new("https://www.rust-lang.org")
        .get()
        .await
        .expect("HTML GET 실패");

    let titles = client.select("title");

    assert!(!titles.is_empty());
}
```
```rust
//
// ✅ 8. HTML attribute 추출 테스트
//
#[tokio::test]
async fn test_html_attribute() {
    let client = WebClient::new("https://www.rust-lang.org")
        .get()
        .await
        .expect("HTML GET 실패");

    let links = client.attr("a", "href");

    assert!(!links.is_empty());
}
```
```rust
//
// ✅ 9. JSONPath 테스트
//
#[tokio::test]
async fn test_json_path() {
    let client = WebClient::new("https://httpbin.org/json")
        .get()
        .await
        .expect("JSON GET 실패");

    let title = client.json_path("slideshow.title");

    assert!(title.is_some());
}
```
```rust
//
// ✅ 10. Timeout 설정 테스트
//
#[tokio::test]
async fn test_timeout() {
    let client = WebClient::new("https://httpbin.org/delay/3")
        .with_timeout(1) // 1초 timeout
        .get()
        .await;

    assert!(client.is_err()); // timeout 발생해야 정상
}
```
----
