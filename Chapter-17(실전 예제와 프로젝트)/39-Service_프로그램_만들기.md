#  서비스 프로그램 만들기
Rust에서 서비스 프로그램을 만들려면 std::thread, tokio,  
또는 async-std 같은 런타임을 활용해 백그라운드 작업을 지속적으로 실행하고,  
OS에 따라 데몬 등록이나 서비스 관리 스크립트를 구성해야 합니다.

## 🧠 Rust로 서비스 프로그램 만드는 기본 흐름
Rust는 Java처럼 setDaemon(true) 같은 API는 없지만,  
서비스처럼 동작하는 프로그램을 만드는 방법은 명확하게 존재합니다.  
아래는 일반적인 구성 방식입니다:

## 1. 백그라운드 루프 구성
``` rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("서비스 작업 실행 중...");
        thread::sleep(Duration::from_secs(5));
    }
}
```

- 이 코드는 종료되지 않고 계속 실행됨
- systemd, launchd, Windows 서비스에 등록하면 서비스처럼 동작

## 2. 비동기 서비스 (Tokio 기반)
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    loop {
        println!("비동기 서비스 작업 실행 중...");
        sleep(Duration::from_secs(5)).await;
    }
}
```

- tokio는 고성능 비동기 런타임
- 웹 서버, 메시지 큐, 이벤트 핸들러 등에 적합

## 3. OS에 서비스 등록
| 운영체제     | 등록 방식 예시                                  |
|--------------|--------------------------------------------------|
| Linux        | `systemd` → 유닛 파일 생성 후 `/etc/systemd/system/`에 배치 |
| macOS        | `launchd` → `.plist` 파일 작성 후 `~/Library/LaunchAgents/`에 등록 |
| Windows      | `sc create` 명령어 또는 `nssm` 도구로 서비스 등록 가능         |

### 예: Linux systemd 유닛 파일
```
[Unit]
Description=Rust 서비스 예제

[Service]
ExecStart=/usr/local/bin/my_rust_service
Restart=always

[Install]
WantedBy=multi-user.target
```


### 4. 실전 예제: 로그 감시 서비스
```rust
use std::{fs::File, io::{BufRead, BufReader}, thread, time::Duration};

fn main() {
    loop {
        if let Ok(file) = File::open("/var/log/syslog") {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if line.contains("ERROR") {
                    println!("에러 감지: {}", line);
                }
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
```

- 시스템 로그를 주기적으로 감시
- 에러 발생 시 출력 또는 알림 가능

## ✅ 요약: Rust 서비스 프로그램 구성 요소
| 항목                     | 설명                                                              |
|--------------------------|-------------------------------------------------------------------|
| 실행 구조                | `loop {}` 또는 `tokio::spawn`으로 지속 실행 루프 구성              |
| OS 서비스 등록           | `systemd`, `launchd`, `Windows 서비스`로 등록 가능                  |
| 에러 및 로그 처리        | `Result`, `log`, `env_logger` 등으로 안정적인 로깅 및 에러 관리     |
| 자동 재시작 및 감시      | `Restart=always`, `watchdog`, `supervisor`로 장애 복구 및 감시 설정 |


---

# 서비스 프로그램과 통신
Rust로 만든 서비스 프로그램이 다른 프로그램(언어 무관)과 통신하려면,  
언어 중립적이고 안정적인 IPC(프로세스 간 통신) 방식을 선택하는 게 중요.  
아래는 상황별로 추천할 수 있는 통신 방식입니다:

## 🧠 Rust 서비스 ↔ 외부 프로그램 통신 방식
| 방식               | Rust 기술 예시                          | 특징                          | 언어 호환성 | 활용 예시                          |
|--------------------|------------------------------------------|-------------------------------|--------------|-------------------------------------|
| HTTP REST API      | `reqwest`, `hyper`, `actix-web`          | 가장 범용적, 언어 중립적       | ✅ 매우 높음   | 웹 서버, 모바일 앱, 마이크로서비스 |
| Unix Domain Socket | `tokio::net::UnixListener`               | 로컬 전용, 빠른 통신           | ✅ 높음       | 데스크탑 앱, 로컬 서비스           |
| CLI 파이프         | `std::process::Command`                  | 간단한 입출력 기반 통신        | ✅ 높음       | 자동화 도구, 스크립트 호출         |
| 공유 메모리        | `mmap`, `shmem`, `memfd`                 | 고속 통신, 구현 복잡            | ⚠️ 낮음       | 실시간 처리, 고성능 시스템         |


## ✅ 추천 기준
- 언어가 다를 경우 → TCP/HTTP/gRPC가 가장 안전하고 호환성 높음
- 같은 머신에서만 통신 → Unix Socket 또는 파일 기반도 가능
- 속도가 중요 → gRPC 또는 shared memory
- 간단한 구조 → REST API 또는 stdin/stdout

### 🚀 Rust에서 HTTP 서버 예시 (actix-web)
```rust
use actix_web::{get, App, HttpServer, Responder};

#[get("/status")]
async fn status() -> impl Responder {
    "서비스 정상 작동 중"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(status))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```
- 외부 프로그램은 http://localhost:8080/status로 요청하면 응답 받음
- Python, JavaScript, C#, Java 등 어떤 언어든 HTTP 요청 가능

## ✨ 요약: Rust 서비스 ↔ 외부 프로그램 통신 방식

| 상황                     | 추천 방식                         |
|--------------------------|-----------------------------------|
| 언어가 다름               | ✅ HTTP, TCP, gRPC                 |
| 같은 머신, 빠른 통신 필요 | ✅ Unix Domain Socket              |
| 간단한 CLI 통신          | ✅ stdin/stdout 파이프             |
| 이벤트 기반 처리         | ✅ 메시지 큐 (Redis, RabbitMQ 등)  |

---

## 🧠 Rust 웹 서비스 구조: 핵심 구성 요소

| 구성 요소         | 기술 스택 예시                                         |
|------------------|--------------------------------------------------------|
| 웹 프레임워크     | `actix-web`, `axum`, `warp`                            |
| 비동기 런타임     | `tokio`                                                |
| 데이터베이스 ORM  | `sqlx`, `diesel`, `sea-orm`                            |
| 서비스 등록       | `systemd`, `launchd`, `Windows 서비스`                 |
| 로깅 및 모니터링  | `tracing`, `log`, `env_logger`, `prometheus`          |


## 🚀 실전 예시: actix-web + PostgreSQL
```rust
use actix_web::{get, App, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;

#[get("/user_count")]
async fn user_count(db: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db.get_ref())
        .await
        .unwrap_or((0,));
    format!("사용자 수: {}", count.0)
}
```
```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_pool = PgPoolOptions::new()
        .connect("postgres://user:pass@localhost/dbname")
        .await
        .expect("DB 연결 실패");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .service(user_count)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

- /user_count로 요청하면 DB에서 사용자 수를 조회
- sqlx는 비동기 DB 클라이언트로 성능이 뛰어남
- 이 서버는 systemd에 등록하면 서비스처럼 항상 실행 가능

## ✅ Rust 서비스 구조 요약

| 구성 단계         | 기술 스택 예시                                      |
|------------------|-----------------------------------------------------|
| 웹 프레임워크     | `actix-web`, `axum`                                 |
| 데이터베이스 연동 | `sqlx`, `diesel`                                    |
| 서비스 등록       | `systemd`, `launchd`, `Windows 서비스`              |
| 로깅 및 모니터링  | `log`, `tracing`, `prometheus`                      |

----
# Windows 에서 서비스 들록 
Windows 서비스로 등록하려면 nssm 또는 sc create 명령어를 사용하는 것이 가장 일반적이며,  
실행 파일을 백그라운드에서 자동으로 실행되도록 설정할 수 있습니다.

## 🧠 Windows에서 Rust 프로그램을 서비스로 등록하는 방법
### 1. ✅ 실행 파일 준비
Rust 프로젝트를 빌드하여 .exe 파일을 생성합니다:
```
cargo build --release
```

- 결과 파일: target\release\your_program.exe

### 2. ✅ nssm으로 서비스 등록 (추천)
nssm (Non-Sucking Service Manager)는 GUI 기반으로 Rust 프로그램을 쉽게 서비스로 등록할 수 있게 해줍니다.
#### 설치 및 등록 절차:
- nssm 다운로드
- 압축 해제 후 nssm.exe 실행
- Path에 Rust 실행 파일 경로 입력 (your_program.exe)
- Arguments, Startup directory 등 설정
- Install service 클릭
#### 서비스 등록 후:
```
nssm start your_service_name
```

### 3. ✅ sc create 명령어로 등록 (CLI 방식)
```
sc create MyRustService binPath= "C:\RustApp\target\release\your_program.exe" start= auto
```

- MyRustService: 서비스 이름
- binPath: 실행 파일 경로
- start= auto: 시스템 부팅 시 자동 시작
#### 서비스 시작:
```
sc start MyRustService
```

#### 서비스 삭제:
```
sc delete MyRustService
```


### 4. ✅ 로그 및 재시작 설정
- nssm은 로그 파일 경로, 재시작 정책 등을 GUI로 설정 가능
- sc create는 기본 기능만 제공하므로 고급 설정은 PowerShell 또는 외부 도구 필요

## ✨ 요약: Rust 프로그램을 Windows 서비스로 등록하는 흐름
| 단계             | 설명                                               |
|------------------|----------------------------------------------------|
| 실행 파일 생성    | `cargo build --release` → `.exe` 파일 준비         |
| GUI 등록 방식     | `nssm`으로 서비스 등록 및 로그/재시작 설정 가능     |
| CLI 등록 방식     | `sc create` 명령어로 서비스 등록 가능              |
| 자동 시작 설정    | `start= auto` 또는 `nssm`에서 설정 가능             |
| 고급 설정 관리    | `nssm`에서 로그 경로, 재시작 정책 등 세부 설정 가능 |


----

# 서비스 등록하는 절차
아래는 Rust 프로그램을 Windows 서비스로 등록할 때 사용할 수 있는 두 가지 예시입니다:

## 🖼️ NSSM 설정 스크린샷 예시 설명

| 설정 탭       | 예시 값 및 설명                                                  |
|---------------|-------------------------------------------------------------------|
| Application   | **Path**: `C:\RustApp\my_service.exe`  
|               | **Startup directory**: `C:\RustApp`                               |
| Details       | **Display name**: `My Rust Service`  
|               | **Description**: `Rust 백엔드 서비스`                             |
| I/O           | **stdout**: `C:\RustApp\logs\stdout.log`  
|               | **stderr**: `C:\RustApp\logs\stderr.log`                          |
| Exit Actions  | **Default action

- 실제 스크린샷은 NSSM 공식 사이트에서 확인 가능: https://nssm.cc/usage


## ⚙️ PowerShell 자동 설치 스크립트 예시
```
# Rust 서비스 등록 스크립트
$serviceName = "MyRustService"
$exePath = "C:\RustApp\my_service.exe"
$logDir = "C:\RustApp\logs"

# 로그 디렉토리 생성
New-Item -ItemType Directory -Path $logDir -Force | Out-Null

# NSSM 경로 (nssm.exe가 있는 위치)
$nssmPath = "C:\nssm\nssm.exe"

# 서비스 등록
& $nssmPath install $serviceName $exePath
& $nssmPath set $serviceName AppDirectory "C:\RustApp"
& $nssmPath set $serviceName AppStdout "$logDir\stdout.log"
& $nssmPath set $serviceName AppStderr "$logDir\stderr.log"
& $nssmPath set $serviceName Start "SERVICE_AUTO_START"
& $nssmPath set $serviceName AppExit Default Restart

# 서비스 시작
& $nssmPath start $serviceName
```

## ✅ 요약: Rust 프로그램 Windows 서비스 등록

| 단계             | 설명                                                       |
|------------------|------------------------------------------------------------|
| 실행 파일 생성    | `cargo build --release`로 `.exe` 파일 준비                 |
| GUI 등록 방식     | `nssm`으로 서비스 등록, 로그 경로 및 재시작 정책 설정 가능 |
| CLI 등록 방식     | `sc create` 명령어로 서비스 등록, `start= auto`로 자동 시작 |


---
# sc

Windows에서 Rust 프로그램을 서비스로 등록할 때 sc create는 가볍고  
스크립트화하기 쉬운 CLI 방식이라 자동화와 배포에 아주 유리합니다.

## 🧠 sc create 명령어 구조
```
sc create [서비스이름] binPath= "[실행파일 경로]" start= auto
```

- 서비스이름: 원하는 서비스 이름
- binPath=: Rust 실행 파일의 전체 경로
- start= auto: 시스템 부팅 시 자동 시작
### ⚠️ binPath=와 start= 사이에는 반드시 공백이 있어야 합니다!


## 🚀 예시: Rust 서비스 등록
```
sc create MyRustService binPath= "C:\RustApp\my_service.exe" start= auto
```
- MyRustService: 서비스 이름
- my_service.exe: Rust로 빌드한 실행 파일
- 자동 시작 설정 포함
### 서비스 시작:
```
sc start MyRustService
```

### 서비스 중지:
```
sc stop MyRustService
```

### 서비스 삭제:
```
sc delete MyRustService
```

## 🔧 고급 설정 (선택 사항)

| 설정 항목     | 설명                             | 명령어 예시                                                              |
|---------------|----------------------------------|---------------------------------------------------------------------------|
| DisplayName   | 서비스 표시 이름 설정             | `sc config MyRustService DisplayName= "Rust 백엔드"`                      |
| Description   | 서비스 설명 추가                  | `sc description MyRustService "Rust로 만든 API 서버"`                     |
| obj           | 실행 계정 지정 (기본은 LocalSystem) | `sc config MyRustService obj= "NT AUTHORITY\LocalService"`               |
| depend        | 의존 서비스 지정                  | `sc config MyRustService depend= Tcpip`                                   |


## ✅ 요약: Rust 프로그램 Windows 서비스 등록

| 단계             | 설명                                                       |
|------------------|------------------------------------------------------------|
| 실행 파일 생성    | `cargo build --release` → `.exe` 파일 준비                 |
| 서비스 등록       | `sc create` 명령어로 CLI 기반 서비스 등록                  |
| 자동 시작 설정    | `start= auto` 옵션으로 시스템 부팅 시 자동 실행             |
| 서비스 관리       | `sc start`, `stop`, `delete`, `config`, `description` 등으로 제어 가능 |

## 🔐 관리자 권한이 필요한 작업들

| 명령/작업        | 설명                                | 관리자 권한 필요 여부 |
|------------------|-------------------------------------|------------------------|
| `sc create`      | 서비스 등록                          | ✅ 필요                |
| `sc start/stop`  | 서비스 시작 및 중지                  | ✅ 필요                |
| `sc delete`      | 서비스 제거                          | ✅ 필요                |
| `nssm install`   | NSSM을 통한 서비스 등록 및 설정      | ✅ 필요                |


## 🧠 실행 팁
- PowerShell 또는 CMD를 “관리자 권한으로 실행” 해야 명령어가 정상 작동합니다.
- .exe를 직접 실행할 때도 서비스 등록용이면 관리자 권한이 필요할 수 있어요.

## ✅ 관리자 권한 요구 사항 요약

| 상황                   | 권한 요구 사항               |
|------------------------|------------------------------|
| 서비스 등록/삭제       | ✅ 관리자 권한 필수           |
| 서비스 시작/중지       | ✅ 관리자 권한 필수           |
| 일반 실행 (테스트용)   | ❌ 관리자 권한 없어도 가능    |


--- 

# Setup시에 sc 설정

서비스 등록은 관리자 권한이 필요한 작업이기 때문에,  
Rust 프로그램을 설치하거나 초기 설정하는 setup 단계에서 sc create를 호출하는 게 가장 깔끔한 방식입니다.

## 🛠️ 설치 흐름 예시
- cargo build --release로 .exe 생성
- 설치 스크립트 또는 인스톨러 실행 (관리자 권한으로)
- 그 안에서 sc create 호출 → 서비스 등록
- sc start로 서비스 시작
- 필요 시 sc config, sc description 등으로 추가 설정

## ⚠️ 관리자 권한 체크 팁 (PowerShell 예시)
```
# 관리자 권한 확인
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "⚠️ 관리자 권한으로 실행해주세요."
    exit
}

# 서비스 등록
sc.exe create MyRustService binPath= "C:\RustApp\my_service.exe" start= auto
```
이렇게 하면 설치 스크립트가 권한을 확인하고, 필요한 경우 사용자에게 관리자 실행을 유도할 수 있습니다.

---

