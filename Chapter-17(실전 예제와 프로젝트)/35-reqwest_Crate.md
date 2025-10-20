# reqwest Crate
reqwest는 Rust에서 가장 널리 쓰이는 HTTP 클라이언트 라이브러리로,  
GET/POST 요청, 헤더 설정, JSON 처리, multipart 업로드, 인증 등 거의 모든 HTTP 기능을 지원합니다.  
Rust에서도 reqwest를 사용하면 GUI 앱이나 CLI에서 서버와 통신하는 기능을 쉽게 구현할 수 있음.

## 🧠 reqwest의 핵심 특징
- 비동기 및 동기 API 모두 지원 (reqwest::Client vs reqwest::blocking::Client)
- JSON, form, multipart 등 다양한 요청 본문 타입 지원
- TLS, 쿠키, 인증, 프록시 등 고급 기능 내장
- tokio 기반 비동기 처리 가능

## ✅ 1. 설치 방법
```
# Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "multipart", "blocking", "stream"] }
tokio = { version = "1", features = ["full"] }
```

## ✅ 2. 간단한 GET 요청 예제
```rust
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://httpbin.org/get").send()?;
    let body = res.text()?;
    println!("Response: {}", body);
    Ok(())
}
```
- blocking 버전은 CLI나 데스크탑 앱에서 사용하기 좋습니다.


## ✅ 3. JSON POST 요청 예제
```rust
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client
        .post("https://httpbin.org/post")
        .json(&json!({ "key": "value" }))
        .send()
        .await?;

    println!("Status: {}", res.status());
    Ok(())
}
```


## ✅ 4. Multipart 파일 업로드 예제 (실전 스타일)
```rust
use reqwest::multipart;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let form = multipart::Form::new()
        .text("reportType", "summary")
        .text("mainFile", "main.txt")
        .text("workDir", "/tmp")
        .part("dataFiles[]", multipart::Part::file("report1.pdf")?.file_name("report1.pdf"))
        .part("dataFiles[]", multipart::Part::file("report2.pdf")?.file_name("report2.pdf"));

    let client = Client::new();
    let res = client
        .post("http://localhost:8080/data/runUploadReportApi")
        .multipart(form)
        .send()
        .await?;

    println!("Upload status: {}", res.status());
    Ok(())
}
```

## ✅ 확장 예제: Rust GUI 앱에서 파일 업로드 + 인증 + 진행률 추적

| 기능             | 설명                                                                 | 예시 코드 또는 키워드                          |
|------------------|----------------------------------------------------------------------|------------------------------------------------|
| 진행률 추적       | `tokio::fs::File` + `ReaderStream`으로 chunk 단위 전송               | `tokio::fs::File::open(path).await?`          |
|                  |                                                                      | `tokio_util::io::ReaderStream::new(file)`     |
| 인증 처리         | Bearer 토큰 또는 커스텀 헤더로 인증                                 | `.bearer_auth("your_token")`                  |
|                  |                                                                      | `.header("Authorization", "Bearer ...")`      |
| GUI 연동          | GUI 프레임워크에서 버튼 클릭 시 업로드 함수 호출                    | `iced`, `egui`, `gtk-rs`                      |
|                  |                                                                      | `Message::UploadClicked` → `run_upload()`     |
| 에러 처리         | HTTP 오류 감지 및 예외 처리                                          | `.error_for_status()?`                        |


## ✅ 결론
- Rust에서 reqwest를 사용하면 GUI 앱에서 HTTP 요청을 보내고 서버와 연동하는 기능을 완벽하게 구현할 수 있습니다.

---

# 🔐 Spring Boot 인증 시스템 구축 전략
## ✅ 1. 기본 구조: Spring Security + JWT
- Spring Security: 인증/인가를 처리하는 핵심 프레임워크
- JWT (JSON Web Token): 클라이언트-서버 간 인증 정보를 안전하게 전달
- UserDetailsService: 사용자 정보를 로딩하는 서비스
- SecurityFilterChain: 인증 필터 설정

## ✅ 2. 핵심 구성 요소
### 📦 pom.xml 의존성
```
<dependency>
  <groupId>org.springframework.boot</groupId>
  <artifactId>spring-boot-starter-security</artifactId>
</dependency>
<dependency>
  <groupId>com.auth0</groupId>
  <artifactId>java-jwt</artifactId>
  <version>4.4.0</version>
</dependency>
```

### 📦 예시: Spring Boot + JWT + Security (Gradle Kotlin DSL)
```
plugins {
    id("org.springframework.boot") version "3.2.0"
    id("io.spring.dependency-management") version "1.1.4"
    kotlin("jvm") version "1.9.10"
    kotlin("plugin.spring") version "1.9.10"
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-security")
    implementation("com.auth0:java-jwt:4.4.0")
    implementation("org.springframework.boot:spring-boot-starter-web")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}
```

위 설정은 build.gradle.kts 파일에 작성되며, pom.xml 없이도 완전한 Spring Boot 프로젝트를 구성할 수 있습니다.

### 👤 사용자 엔티티
```java
@Entity
public class User implements UserDetails {
  @Id
  private Long id;
  private String username;
  private String password;
  @Enumerated(EnumType.STRING)
  private Role role;

  // UserDetails 인터페이스 구현
}
```

### 🔑 JWT 토큰 생성기
```java
public String generateToken(UserDetails user) {
  return JWT.create()
    .withSubject(user.getUsername())
    .withExpiresAt(new Date(System.currentTimeMillis() + 86400000))
    .sign(Algorithm.HMAC256(secret));
}
```

### 🛡️ JWT 필터
```java
public class JwtAuthFilter extends OncePerRequestFilter {
  protected void doFilterInternal(...) {
    // Authorization 헤더에서 토큰 추출 → 검증 → SecurityContext에 사용자 설정
  }
}
```

### 🔒 Security 설정
```java
@Bean
public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
  http.csrf().disable()
      .authorizeHttpRequests(auth -> auth
          .requestMatchers("/login", "/register").permitAll()
          .anyRequest().authenticated())
      .addFilterBefore(jwtAuthFilter, UsernamePasswordAuthenticationFilter.class);
  return http.build();
}
```


## ✅ 3. 인증 흐름
- 사용자 로그인 요청 → /login
- 서버가 JWT 토큰 발급 → 클라이언트에 반환
- 클라이언트가 Authorization 헤더에 토큰 포함 → 이후 모든 요청에 사용
- 서버는 필터에서 토큰 검증 후 SecurityContext에 사용자 설정

## ✅ 4. 추가 보안 팁
- HTTPS 강제 적용: requiresSecure() 설정
- CSRF 보호 유지: REST API에서는 비활성화 가능하지만, 웹 앱에서는 유지
- 입력 검증: @Valid, @NotEmpty 등으로 DTO 검증
- 권한 기반 인가: @PreAuthorize("hasRole('ADMIN')") 등으로 역할 제어

## ✅ 결론
Spring Boot에서 인증 시스템을 구축할 때는 Spring Security + JWT 조합이 가장 안전하고 유연합니다.  
사용자 인증, 토큰 발급/검증, 권한 제어까지 모두 커스터마이징 가능하며, REST API 또는 웹 앱 모두에 적용할 수 있음

---

# 🔐 Spring Boot + JWT 인증 시스템과 Rust 클라이언트 연동 흐름
## ✅ 1. 로그인 → JWT 토큰 발급 받기
Spring Boot 서버에서 /login 또는 /auth/login 같은 엔드포인트를 통해 사용자 인증 후 JWT 토큰을 반환합니다.

### 📦 Rust 클라이언트 코드
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

async fn login(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("http://localhost:8080/auth/login")
        .json(&LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?
        .error_for_status()?; // HTTP 오류 감지

    let login_response: LoginResponse = res.json().await?;
    Ok(login_response.token)
}
```


## ✅ 2. JWT 토큰을 포함해 인증된 요청 보내기
서버는 Authorization: Bearer <token> 헤더가 포함된 요청만 인증된 사용자로 처리합니다.

### 📦 Rust 클라이언트 코드
```rust
async fn get_user_profile(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .get("http://localhost:8080/api/profile")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?; // 인증 실패 시 에러 발생

    let body = res.text().await?;
    println!("Profile: {}", body);
    Ok(())
}
```


## ✅ 3. 파일 업로드 요청에 토큰 포함
Spring Boot에서 @RequestPart, @RequestParam, @RequestHeader 등을 통해 multipart 요청과 JWT 인증을 함께 처리할 수 있습니다.
### 📦 Rust 클라이언트 코드
```rust
use reqwest::multipart;

async fn upload_file(token: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let part = multipart::Part::file(file_path)?
        .file_name("report.pdf")
        .mime_str("application/pdf")?;

    let form = multipart::Form::new()
        .text("reportType", "summary")
        .part("dataFiles[]", part);

    let client = Client::new();
    let res = client
        .post("http://localhost:8080/api/upload")
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?
        .error_for_status()?; // 인증 실패 또는 업로드 오류 감지

    println!("Upload complete: {}", res.status());
    Ok(())
}
```
---

# 🧠 서버 측 Spring Boot 설정 요약
- /auth/login → 사용자 인증 후 JWT 발급
- /api/** → JWT 필터로 보호
- Authorization 헤더에서 토큰 추출
- @RequestPart로 multipart 파일 처리
- @PreAuthorize("hasRole('USER')")로 권한 제어 가능

## ✅ 결론
Rust 클라이언트는 reqwest를 통해 Spring Boot JWT 인증 시스템과 완벽하게 연동할 수 있습니다.

---


## ✅ 1. Spring Boot 서버
- /auth/login → 사용자 인증 후 JWT 토큰 발급
- /api/upload → multipart 파일 업로드 처리 + JWT 인증 필터 적용
- JWT 필터가 Authorization: Bearer <token> 헤더를 검사하여 인증 여부 판단
- 업로드된 파일은 서버에서 저장하거나 DB에 기록 가능

## ✅ 2. Rust 클라이언트
- login() 함수 → 사용자 정보를 보내고 JWT 토큰을 받아옴
- upload_file() 함수 → 파일을 multipart로 전송하며 JWT 토큰을 헤더에 포함
- reqwest를 통해 HTTP 요청을 보내고, .error_for_status()로 오류 감지
- GUI 앱에서도 버튼 클릭 시 이 흐름을 그대로 호출 가능

## 🔄 전체 연동 흐름
```
[Rust 클라이언트]
    ↓ POST /auth/login
    ← JWT 토큰 수신
    ↓ POST /api/upload (Authorization: Bearer <token>)
    ← 업로드 성공 응답

[Spring Boot 서버]
    → JWT 필터로 인증 처리
    → 파일 저장 및 응답 반환
```


## ✅ 실전에서 필요한 추가 요소
- Spring Boot에서 CORS 설정 (allowedOrigins) → Rust 앱에서 요청 가능하게
- Rust에서 파일 경로, 이름, MIME 타입 등 유효성 검사
- 서버에서 업로드된 파일 저장 위치 및 DB 기록 처리

---

# ✅ 보안 감사에서 지적받지 않기 위한 핵심 체크리스트
## 🔐 1. JWT 토큰 관리
- ✅ 토큰은 HTTPS 환경에서만 전달해야 함 → HTTP는 무조건 지적 대상
- ✅ 토큰은 적절한 만료 시간 설정 (예: 15분~1시간)
- ✅ 토큰에 서명 알고리즘 명시 (HS256, RS256) → none 사용 금지
- ✅ 토큰에 불필요한 정보 포함 금지 (비밀번호, 민감한 ID 등)
## 🧱 2. Spring Security 설정
- ✅ SecurityFilterChain에서 권한별 접근 제어 명확히 설정
- ✅ /api/** 경로는 반드시 인증 필터 적용
- ✅ csrf().disable()은 REST API에서만 허용 → 웹 앱이면 지적 대상
- ✅ CORS 설정 명확히 → allowedOrigins, allowedMethods 제한
## 🧾 3. 입력값 검증
- ✅ 모든 DTO에 @Valid, @NotBlank, @Size 등 적용
- ✅ 파일 업로드 시 파일 크기 제한, MIME 타입 검증, 경로 탐색 방지
- ✅ Rust 클라이언트에서도 사용자 입력값 사전 검증
## 🧰 4. 에러 처리 및 로깅
- ✅ 인증 실패, 업로드 실패 시 명확한 에러 코드 반환
- ✅ 서버 로그에 토큰, 비밀번호, 파일 경로 등 민감 정보 기록 금지
- ✅ Rust 클라이언트에서도 .error_for_status()로 예외 처리
## 🔒 5. 파일 업로드 보안
- ✅ 서버에서 업로드된 파일은 임시 디렉토리 또는 사용자별 디렉토리에 저장
- ✅ 파일 이름은 UUID 또는 해시 기반으로 변경 → 원본 이름 노출 최소화
- ✅ 업로드 후 파일 실행 금지 (noexec 마운트 또는 권한 제한)

## 🧠 보안 감사에서 흔히 지적되는 항목

| 항목                         | 지적 사유                                                   |
|------------------------------|--------------------------------------------------------------|
| HTTP 사용                    | 토큰, 비밀번호 등 민감 정보가 평문으로 전송됨               |
| JWT 만료 없음                | 세션 고정 공격 가능성, 토큰 탈취 시 장기 피해 발생           |
| CSRF 보호 미적용             | 웹 앱에서 요청 위조 가능성 (특히 쿠키 기반 인증 시)          |
| CORS 설정 허술               | 외부 도메인에서 API 호출 가능 → 데이터 유출 위험             |
| 파일 업로드 검증 없음        | 악성 파일 업로드 가능성 (예: .exe, .js, .php 등)             |
| 입력값 유효성 검증 미흡      | SQL 인젝션, XSS, 경로 탐색 공격 가능                         |
| 로그에 민감 정보 포함        | 토큰, 비밀번호, 사용자 정보가 로그에 남아 유출 위험           |


## ✅ 결론
지금까지 구성한 구조는 보안 감사 기준에서 충분히 안전한 기본 구조입니다.

---

# 전체 JWT 흐름

Spring Boot에서 /upload 엔드포인트에 JWT 인증 필터를 적용하는 예제를 아래에 단계별로 완성된 코드로 정리.  
이 예제는 실제로 동작 가능한 구조이며, 보안 감사 기준도 충족합니다.

## ✅ 1. JWT 필터 클래스 (JwtAuthFilter.java)
```java
@Component
public class JwtAuthFilter extends OncePerRequestFilter {

    @Autowired
    private JwtService jwtService;

    @Autowired
    private UserDetailsService userDetailsService;

    @Override
    protected void doFilterInternal(HttpServletRequest request,
                                    HttpServletResponse response,
                                    FilterChain filterChain)
                                    throws ServletException, IOException {

        final String authHeader = request.getHeader("Authorization");
        final String token;
        final String username;

        if (authHeader == null || !authHeader.startsWith("Bearer ")) {
            filterChain.doFilter(request, response);
            return;
        }

        token = authHeader.substring(7);
        username = jwtService.extractUsername(token);

        if (username != null && SecurityContextHolder.getContext().getAuthentication() == null) {
            UserDetails userDetails = userDetailsService.loadUserByUsername(username);

            if (jwtService.isTokenValid(token, userDetails)) {
                UsernamePasswordAuthenticationToken authToken =
                    new UsernamePasswordAuthenticationToken(userDetails, null, userDetails.getAuthorities());

                authToken.setDetails(new WebAuthenticationDetailsSource().buildDetails(request));
                SecurityContextHolder.getContext().setAuthentication(authToken);
            }
        }

        filterChain.doFilter(request, response);
    }
}
```

## ✅ 2. JWT 서비스 클래스 (JwtService.java)
```java
@Service
public class JwtService {

    private final String SECRET_KEY = "your-secret-key";

    public String extractUsername(String token) {
        return JWT.require(Algorithm.HMAC256(SECRET_KEY))
                  .build()
                  .verify(token)
                  .getSubject();
    }

    public boolean isTokenValid(String token, UserDetails userDetails) {
        String username = extractUsername(token);
        return username.equals(userDetails.getUsername());
    }
}
```


## ✅ 3. Security 설정 (SecurityConfig.java)
```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Autowired
    private JwtAuthFilter jwtAuthFilter;

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
        http.csrf().disable()
            .authorizeHttpRequests(auth -> auth
                .requestMatchers("/auth/**").permitAll()
                .requestMatchers("/upload").authenticated()
                .anyRequest().permitAll()
            )
            .sessionManagement(sess -> sess.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
            .addFilterBefore(jwtAuthFilter, UsernamePasswordAuthenticationFilter.class);

        return http.build();
    }
}
```


## ✅ 4. 업로드 엔드포인트 (UploadController.java)
```java
@RestController
public class UploadController {

    @PostMapping("/upload")
    public ResponseEntity<String> upload(@RequestParam("file") MultipartFile file,
                                         Principal principal) {
        String username = principal.getName();
        // 파일 저장 로직
        return ResponseEntity.ok("File uploaded by " + username);
    }
}
```

## ✅ 5. 클라이언트 요청 예시 (Rust 또는 Postman)
```
POST /upload HTTP/1.1
Host: localhost:8080
Authorization: Bearer <your_jwt_token>
Content-Type: multipart/form-data
```


## ✅ 결론
이 예제는 /upload 경로에 JWT 인증 필터를 적용하여, 토큰이 유효한 사용자만 업로드 가능하도록 보호합니다.  
보안 감사 기준도 충족하며, 확장 시 역할 기반 인가 (@PreAuthorize)도 쉽게 추가할 수 있음.  

---

## ✅ 여러 경로에 인증 적용하는 방법
### 1. 체이닝 방식으로 추가
```java
.authorizeHttpRequests(auth -> auth
    .requestMatchers("/upload", "/profile", "/admin").authenticated()
    .requestMatchers("/auth/**", "/public/**").permitAll()
    .anyRequest().denyAll()
)
```

- /upload, /profile, /admin → JWT 인증 필터를 통과해야 접근 가능
- /auth/**, /public/** → 누구나 접근 가능
- 나머지 경로는 모두 차단 (denyAll())

### 2. 배열로 경로 묶기
```java
String[] protectedPaths = { "/upload", "/profile", "/admin", "/settings" };

.authorizeHttpRequests(auth -> auth
    .requestMatchers(protectedPaths).authenticated()
    .anyRequest().permitAll()
)
```
- 이 방식은 경로가 많아질 때 관리하기 편리합니다.


### 3. 정규 표현식 또는 패턴 사용
```java
.requestMatchers("/api/**").authenticated()
```

- /api/로 시작하는 모든 경로에 인증 적용
- 예: /api/upload, /api/user, /api/settings 등

## ✅ 결론
.requestMatchers(...).authenticated()는 인증이 필요한 경로를 지정하는 곳이고,
여러 개의 경로를 계속 추가하거나 배열로 묶어서 관리할 수 있습니다.
보안 감사에서도 이 부분은 명확하게 경로별 인증 정책이 적용되어야 하는 핵심 항목.

---


