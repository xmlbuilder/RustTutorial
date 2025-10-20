# Actix-web
Actix-web은 Rust 생태계에서 Spring Boot와 유사한 역할을 충분히 수행할 수 있습니다.  
다만, Rust와 Java의 철학과 생태계가 다르기 때문에 접근 방식은 조금 다릅니다.

## 🧰 Actix-web vs Spring Boot: 역할 비교
| 항목               | Spring Boot (Java)                          | Actix-web (Rust)                             |
|--------------------|---------------------------------------------|----------------------------------------------|
| 언어               | Java                                        | Rust                                         |
| 웹 프레임워크      | Spring MVC 기반                             | Actix 기반의 actor 모델                      |
| 비동기 처리        | 기본은 동기, WebFlux로 비동기 가능          | 기본이 async-first (Tokio 기반)              |
| 의존성 주입        | 자동 DI (Annotation 기반)                   | 수동 DI 또는 trait 기반 구성                 |
| REST API 구성      | `@RestController`, `@GetMapping` 등         | `#[get("/")]`, `App::new().route(...)`       |
| 미들웨어           | 필터, 인터셉터 등                           | Tower 기반 미들웨어 체인                     |
| 데이터베이스 연동  | JPA, Hibernate                              | sqlx, Diesel 등                              |
| 설정 방식          | `application.yml`, `application.properties` | `.env`, `config` crate 등                    |
| 배포 및 실행       | JVM 기반, fat jar                           | 바이너리 실행 파일 (성능 우수)               |
| 테스트 지원        | JUnit, Mockito 등                           | `actix-rt`, `tokio-test`, `assert_json_diff` 등 |

## ✅ Actix-web이 Spring Boot처럼 사용 가능한 이유
- 고성능: Rust의 성능 덕분에 Actix-web은 매우 빠른 응답 속도를 자랑합니다 (TechEmpower 벤치마크 상위권)
- 비동기 처리 기본 내장: Tokio 기반으로 async-first 설계
- 라우팅, 미들웨어, 상태 관리, 에러 처리 등 웹 프레임워크로서 필수 기능 완비
- sqlx, Diesel, SeaORM 등과 연동하여 DB 처리도 가능
- serde와 함께 JSON 직렬화/역직렬화도 간편하게 처리

## 🧪 간단한 Actix-web REST API 예제
```rust
use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello, Rust!"
}

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    format!("Echo: {}", body)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```


## 📦 Cargo.toml 설정 예시
```
[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```


## 🧠 결론
Actix-web은 Spring Boot처럼 REST API 서버, 웹 서비스, 마이크로서비스 백엔드를 구축하는 데 충분한 기능을 갖추고 있으며,  
Rust의 안전성과 성능까지 더해져 실무에서도 점점 더 많이 채택되고 있음.

---


## 🧱 1. 실전 프로젝트 구조 예시
```
my-actix-app/
├── src/
│   ├── main.rs              # 앱 진입점
│   ├── routes/
│   │   ├── mod.rs           # 라우트 모듈 등록
│   │   ├── auth.rs          # 로그인/회원가입 라우트
│   │   └── user.rs          # 사용자 관련 라우트
│   ├── handlers/
│   │   ├── auth_handler.rs  # 인증 처리 로직
│   │   └── user_handler.rs  # 사용자 처리 로직
│   ├── models/
│   │   └── user.rs          # DB 모델 정의
│   ├── db.rs                # DB 연결 설정
│   └── config.rs            # 환경 설정 로딩
├── .env                     # 환경 변수 (DB URL 등)
├── Cargo.toml               # 의존성 설정
```


## 🔐 2. 인증 처리 예제 (JWT 기반)
### 📦 Cargo.toml 설정
```
[dependencies]
actix-web = "4"
jsonwebtoken = "8"
serde = { version = "1.0", features = ["derive"] }
dotenv = "0.15"
```

### 🧪 JWT 생성 및 검증
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_token(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: chrono::Utc::now().timestamp() as usize + 3600,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret")).unwrap()
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &Validation::default())
        .map(|data| data.claims)
}
```


## 🗄️ 3. DB 연동 예제 (sqlx + PostgreSQL)
### 📦 Cargo.toml 설정
```
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "macros"] }
tokio = { version = "1", features = ["full"] }
dotenv = "0.15"
```


### 🧪 DB 연결 및 모델 정의
- src/db.rs
```rust
use sqlx::postgres::PgPoolOptions;

pub async fn connect_db() -> sqlx::Pool<sqlx::Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
```

- src/models/user.rs
```rust
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
```


## 🚀 4. 라우트 예제: 회원가입
- src/routes/auth.rs
```rust
use actix_web::{post, web, HttpResponse};
use crate::models::user::User;

#[post("/register")]
async fn register(user: web::Json<User>) -> HttpResponse {
    // DB에 저장하는 로직 생략
    HttpResponse::Ok().json(user.into_inner())
}
```


## ✅ 결론
이 구조는 Actix-web으로 RESTful API 서버, JWT 인증, PostgreSQL 연동까지 모두 갖춘 실전 백엔드 프로젝트의 기본 골격입니다.  


---


## 📁 1. 파일 업로드 (multipart/form-data)
### 📦 Cargo.toml
```
actix-multipart = "0.6"
tokio = { version = "1", features = ["full"] }
```

### 🧪 예제: 파일 업로드 핸들러
```rust
use actix_multipart::Multipart;
use actix_web::{post, web, App, HttpServer, Error, HttpResponse};
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.next().await {
        let mut field = field?;
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map(|f| format!("uploads/{}", sanitize_filename::sanitize(f)))
            .unwrap_or("uploads/unknown".to_string());

        let mut f = File::create(&filename)?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }
    }
    Ok(HttpResponse::Ok().body("File uploaded"))
}
```
- sanitize_filename 크레이트를 사용해 파일 이름을 안전하게 처리하세요.




## 🔐 2. OAuth 연동 (예: GitHub 로그인)
### 📦 Cargo.toml
```rust
oauth2 = "4"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
```

### 🧪 예제: GitHub OAuth 흐름
```rust
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
    AuthorizationCode, CsrfToken, Scope,
};
use actix_web::{get, web, HttpResponse};

#[get("/auth/github")]
async fn github_login(oauth_client: web::Data<BasicClient>) -> HttpResponse {
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}
```
- OAuth 콜백 처리, 토큰 교환, 사용자 정보 요청은 추가 구현 필요


## 🧪 3. 비동기 테스트 (actix-rt + tokio-test)
### 📦 Cargo.toml
```
[dev-dependencies]
actix-rt = "2"
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
```

## ✅ 예제: 통합 테스트
```rust
#[actix_rt::test]
async fn test_hello_endpoint() {
    let resp = reqwest::get("http://localhost:8080/hello")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(resp, "Hello, Rust!");
}
```
- 테스트 전 서버가 실행 중이어야 하며, actix_web::test 모듈을 활용하면 인메모리 테스트도 가능


## 🚀 4. CI/CD 구성 (GitHub Actions 예시)
### 📄 .github/workflows/ci.yml
```
name: Rust CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Install dependencies
        run: sudo apt-get update && sudo apt-get install -y libpq-dev
      - name: Build
        run: cargo build --verbose
      - name: Run tests
        run: cargo test --verbose
```

- PostgreSQL, Redis 등 외부 서비스가 필요할 경우 services: 섹션에 추가 가능

---

## 🧠 수십 개 파일 업로드 시 고려사항
### ✅ 1. 파일 하나씩 스트리밍 처리
- Multipart는 스트리밍 방식으로 각 파일을 순차적으로 받아옵니다.
- 메모리에 한꺼번에 올리지 않기 때문에 메모리 폭주를 방지할 수 있음.
```rust
while let Some(mut field) = payload.next().await {
    let mut field = field?;
    // 파일 처리 로직
}
```


### ✅ 2. 파일 저장을 비동기로 처리
- tokio::fs::File을 사용하면 파일 저장도 비동기로 처리 가능
- 여러 파일을 동시에 저장할 수 있도록 tokio::spawn으로 작업을 분리할 수 있음
```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

while let Some(mut field) = payload.next().await {
    let mut field = field?;
    let filename = ...;

    let mut file = File::create(&filename).await?;
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        file.write_all(&data).await?;
    }
}
```


### ✅ 3. 파일 저장을 병렬로 처리하고 싶다면
- futures::stream::FuturesUnordered를 사용하면 병렬로 처리 가능
```rust
use futures::stream::FuturesUnordered;
use futures::StreamExt;

let mut tasks = FuturesUnordered::new();

while let Some(mut field) = payload.next().await {
    let mut field = field?;
    let filename = ...;

    tasks.push(tokio::spawn(async move {
        let mut file = File::create(&filename).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data).await?;
        }
        Ok::<_, std::io::Error>(())
    }));
}

while let Some(result) = tasks.next().await {
    result??;
}
```


### ✅ 개선된 구조: 병렬 파일 저장
```rust
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Error};
use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures::stream::FuturesUnordered;
use futures::StreamExt as FuturesStreamExt;

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut tasks = FuturesUnordered::new();

    while let Some(mut field) = payload.next().await {
        let mut field = field?;
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map(|f| format!("uploads/{}", sanitize_filename::sanitize(f)))
            .unwrap_or_else(|| format!("uploads/unknown_{}", uuid::Uuid::new_v4()));

        // spawn a task for each file
        tasks.push(tokio::spawn(async move {
            let mut f = File::create(&filename).await?;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f.write_all(&data).await?;
            }
            Ok::<_, std::io::Error>(filename)
        }));
    }

    let mut uploaded_files = Vec::new();
    while let Some(result) = tasks.next().await {
        match result {
            Ok(Ok(filename)) => uploaded_files.push(filename),
            Ok(Err(e)) => eprintln!("File write error: {}", e),
            Err(e) => eprintln!("Task join error: {}", e),
        }
    }

    Ok(HttpResponse::Ok().json(uploaded_files))
}
```


## 🧠 핵심 변경점
- FuturesUnordered로 파일 저장을 병렬 처리 → 빠르고 효율적
- tokio::spawn으로 각 파일 저장을 비동기 태스크로 분리
- uuid::Uuid로 파일 이름 중복 방지
- 결과를 Vec<String>으로 수집하여 클라이언트에 반환


