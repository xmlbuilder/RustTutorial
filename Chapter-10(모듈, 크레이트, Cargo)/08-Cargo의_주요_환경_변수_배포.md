# 환경 변수
Rust에서 env! 매크로로 접근 가능한 Cargo의 주요 환경 변수들은 프로젝트 경로, 빌드 설정, 패키지 정보 등을 컴파일 타임에 활용할 수 있게 해줍니다.  
아래는 대표적인 디렉토리 및 경로 관련 환경 변수들의 정리입니다.

## 📦 Cargo 환경 변수 요약 (env! 매크로로 사용 가능)
| 환경 변수                  | 설명                                      |
|----------------------------|-------------------------------------------|
| `CARGO_MANIFEST_DIR`       | 현재 crate의 `Cargo.toml`이 위치한 디렉토리 |
| `CARGO_TARGET_DIR`         | 빌드 결과물이 저장되는 `target/` 디렉토리   |
| `CARGO_HOME`               | Cargo 설정 및 캐시 디렉토리 (`~/.cargo`)    |
| `OUT_DIR`                  | `build.rs`가 생성한 출력 파일 디렉토리      |
| `CARGO`                    | Cargo 실행 파일 경로 (`cargo` 바이너리)     |
| `RUSTC`                    | Rust 컴파일러 경로 (`rustc` 바이너리)       |
| `RUSTC_WRAPPER`            | `rustc` 대신 실행할 래퍼 프로그램 경로      |
| `RUSTC_WORKSPACE_WRAPPER`  | 워크스페이스 전체에 적용되는 `rustc` 래퍼   |


## 🧠 사용 예시
```rust
const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const TARGET: &str = env!("CARGO_TARGET_DIR"); // 설정된 경우에만 사용 가능
```

- env!는 컴파일 타임에 상수로 삽입되므로 const로 사용할 수 있음
- 존재하지 않는 환경 변수를 호출하면 컴파일 오류 발생

## 📁 활용 예시
- `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated.rs"));`
- `std::fs::read_to_string(env!("OUT_DIR").to_owned() + "/config.json")`

이 외에도 CARGO_PKG_NAME, CARGO_PKG_VERSION, CARGO_PKG_AUTHORS 같은 패키지 메타 정보도 env!로 접근 가능해요.

---
# Runtime 시 접근
env! 매크로는 컴파일 타임에만 동작하는 매크로이기 때문에, 배포된 실행 파일에서는 더 이상 CARGO_MANIFEST_DIR 같은 환경 변수에 접근할 수 없음.  
이로 인해 몇 가지 중요한 차이점과 주의사항이 생깁니다.

## ⚠️ 배포된 실행 파일에서의 문제점
| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| `env!` 매크로     | 컴파일 타임에만 동작하며 런타임에서는 변경 불가                      |
| `CARGO_MANIFEST_DIR` | 빌드 시점의 경로가 고정되므로 배포 환경에서는 경로가 달라질 수 있음     |
| 경로 하드코딩 위험 | 실행 파일이 다른 위치에 있을 경우 리소스 접근 실패 가능                 |
| 실행 파일 기준 경로 필요 | 런타임에는 `env::current_exe()`로 실행 파일 위치를 기준으로 경로 계산 필요 |


## ✅ 배포용 실행 파일에서 경로 처리 방법
### 1. 런타임 기준 경로 사용
```rust
use std::env;
use std::path::PathBuf;

fn get_exe_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("실행 파일 경로를 가져올 수 없습니다");
    exe_path.parent().unwrap().to_path_buf()
}
```

- env::current_exe()는 실행 중인 바이너리의 경로를 반환
- 이를 기반으로 리소스 파일 경로를 계산해야 함

### 2. 리소스 파일을 실행 파일과 함께 배포
```rust
let resource_path = get_exe_dir().join("data/config.json");
```

- 실행 파일과 같은 디렉토리에 리소스를 배치
- 상대 경로로 접근 가능

### 3. 리소스를 include_bytes! 또는 include_str!로 컴파일 타임에 포함
- 이 방식은 파일 내용을 실행 파일에 내장하므로 경로 문제 없음
- 단점: 파일 크기가 커지거나 동적으로 교체하기 어려움

## 🧩 요약
| 상황 또는 목적                         | 추천 방식                                      |
|----------------------------------------|------------------------------------------------|
| 개발 중 정적 코드 또는 테이블 삽입       | `env!("CARGO_MANIFEST_DIR")` + `include!`       |
| 배포된 실행 파일에서 리소스 접근         | `env::current_exe()`로 실행 파일 기준 경로 계산 |
| 리소스를 실행 파일에 내장하고 싶을 때    | `include_str!`, `include_bytes!`                |

---

# 리소스 배포
Rust 프로젝트에서 리소스를 다루는 대표적인 전략들을 build.rs 자동 복사, 런타임 로딩, 그리고 실행 파일과 함께 배포하는 방식으로 정리.

## 🛠️ 1. build.rs에서 리소스 자동 복사
### 📁 예시 디렉토리 구조
```
project/
├── build.rs
├── src/
│   └── main.rs
├── assets/
│   └── config.json
```

### 🔧 build.rs 코드 예시
```rust
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("assets/config.json", out_dir.join("config.json"))
        .expect("리소스 복사 실패");
}
```

- OUT_DIR은 Cargo가 빌드 중 생성하는 임시 디렉토리
- 복사된 파일은 env!("OUT_DIR")로 접근 가능
### 📦 main.rs에서 접근
```rust
let config_path = concat!(env!("OUT_DIR"), "/config.json");
let config = std::fs::read_to_string(config_path)?;
```


## 📂 2. std::fs로 런타임 로딩
### 📁 예시 디렉토리 구조
```
project/
├── target/
│   └── release/
│       ├── my_app.exe
│       └── config.json

```
### 🔧 main.rs에서 실행 파일 기준 경로 계산
```rust
use std::env;
use std::fs;
use std::path::PathBuf;

fn load_config() -> std::io::Result<String> {
    let exe_dir = env::current_exe()?.parent().unwrap().to_path_buf();
    let config_path = exe_dir.join("config.json");
    fs::read_to_string(config_path)
}
```

- env::current_exe()는 실행 중인 바이너리의 경로를 반환
- 실행 파일과 같은 디렉토리에 리소스를 배치하면 쉽게 접근 가능

### 🚚 3. 실행 파일과 함께 리소스 배포 전략
| 전략 또는 상황                         | 추천 방식 또는 도구                                      |
|----------------------------------------|----------------------------------------------------------|
| 런타임에 실행 파일 기준으로 리소스 접근 | `std::fs` + `env::current_exe()`                          |
| 실행 파일과 같은 디렉토리에 리소스 배치 | `exe_dir.join("assets/config.json")`                      |
| 리소스를 실행 파일에 내장               | `include_str!`, `include_bytes!`                          |
| GUI/CLI 앱을 설치 가능한 형태로 배포     | `cargo-bundle`, `cargo-deb`, `cargo-dist` 등 패키징 도구 사용 |

## 🧩 요약

| 목적 또는 상황                         | 추천 방식 또는 도구                                 |
|----------------------------------------|-----------------------------------------------------|
| 빌드 시 리소스를 자동 복사             | `build.rs` + `env!("OUT_DIR")`                      |
| 런타임에 실행 파일 기준으로 리소스 접근 | `std::fs` + `env::current_exe()`                    |
| 리소스를 실행 파일에 내장               | `include_str!`, `include_bytes!`                    |
| 설치 가능한 형태로 리소스 포함 배포     | `cargo-bundle`, `cargo-deb`, `cargo-dist` 등 패키징 도구 |


## 🚀 cargo-dist를 활용한 배포 전략
### ✅ 주요 기능
- GitHub Releases 자동 생성
- CI/CD 워크플로우 자동화 (예: GitHub Actions)
- .zip, .tar.gz, .msi, .deb 등 다양한 포맷으로 빌드
- 릴리스 노트 포함 및 버전 태그 기반 릴리스 트리거
### 🛠️ 설정 방법
- 설치:
```
cargo install cargo-dist
```
- 초기화:
```
cargo dist init
```
- Cargo.toml에 [profile.dist] 및 [workspace.metadata.dist] 추가됨
- 릴리스 생성:
```
cargo dist build
cargo dist publish
```

### 📦 배포 흐름
- git tag v1.0.0 → GitHub Actions가 자동으로 빌드 및 릴리스 생성
- 릴리스에 바이너리, 설치 파일, 릴리스 노트 포함

---

# 🖥️ cargo-bundle을 활용한 GUI 앱 패키징
## ✅ 주요 기능
- macOS .app, Linux .deb, Windows .msi 생성
- 앱 아이콘, 이름, 리소스 포함 가능
- Cargo.toml에 메타데이터로 설정
## 🛠️ 설정 방법
- 설치:
```
cargo install cargo-bundle
```
- Cargo.toml에 설정 추가:
```
[package.metadata.bundle]
name = "MyApp"
identifier = "com.example.myapp"
icon = ["assets/icon.png"]
resources = ["assets/config.json"]
```
- 빌드:
```
cargo bundle --release
```

## 🧩 두 도구의 조합 전략
| 목적 또는 상황                         | 추천 도구 또는 조합                          |
|----------------------------------------|----------------------------------------------|
| 릴리스 자동화 및 CI/CD 설정             | `cargo-dist`                                  |
| OS별 GUI 앱 패키지 생성                 | `cargo-bundle`                                |
| GUI 앱을 릴리스 자동화에 포함           | `cargo-dist` + `cargo-bundle` 조합             |
| 아이콘, 리소스 포함된 GUI 앱 패키징     | `cargo-bundle`                                |

## 🎯 배포 전략 요약
- 개발 완료 후 `cargo dist init`으로 릴리스 설정
- `cargo bundle`로 OS별 GUI 앱 패키지 생성
- `cargo dist build` → `cargo dist publish`로 릴리스 자동화
- GitHub Releases에 `.app`, `.msi`, `.deb` 등 포함

이렇게 하면 Rust GUI 앱을 자동화된 릴리스 파이프라인과 함께 OS별 설치 파일로 배포할 수 있음.

---

# 디렉토리 포함
cargo-bundle이나 일반적인 Rust 배포 전략에서 디렉토리 전체를 리소스로 포함하거나 배포하는 방법은 가능합니다.  
다만 방식이 도구마다 조금씩 다름:

## 📦 cargo-bundle에서 디렉토리 전체 리소스 포함하기
### ✅ 방법
resources 필드에 디렉토리 경로를 지정하면 내부 파일들이 모두 포함됩니다.
```
[package.metadata.bundle]
name = "MyApp"
identifier = "com.example.myapp"
resources = ["assets"]
```

- assets/ 디렉토리 전체가 번들에 포함됨
- macOS .app, Windows .msi, Linux .deb 등에서 리소스 디렉토리로 복사됨
### 📁 예시 구조
```
assets/
├── config.json
├── images/
│   └── logo.png
├── data/
│   └── table.csv
```

- 위 디렉토리 전체가 앱 번들에 포함됨

### 🛠️ cargo-dist와 함께 배포할 때
- cargo-dist는 기본적으로 바이너리 중심의 릴리스 자동화 도구
- 리소스 디렉토리를 포함하려면 릴리스 스크립트 또는 CI 설정에서 수동 복사 필요
### ✅ 전략
- cargo bundle로 .app, .msi, .deb 생성 (리소스 포함됨)
- cargo dist build로 릴리스 아티팩트 생성
- GitHub Actions 또는 릴리스 스크립트에서 .zip에 리소스 디렉토리 포함

### GitHub Actions 예시
- name: Copy assets
```
  run: cp -r assets dist/
```
- name: Zip release
```
  run: zip -r my_app.zip dist/
```

## 🧩 요약
| 목적 또는 상황                         | 추천 방식 또는 도구                                 |
|----------------------------------------|-----------------------------------------------------|
| GUI 앱 번들에 디렉토리 전체 포함         | `cargo-bundle` + `resources = ["assets"]`           |
| 릴리스 자동화 및 배포                   | `cargo-dist`                                        |
| 런타임에 실행 파일 기준으로 리소스 접근 | `std::fs` + `env::current_exe()`                    |


---
# .app / .deb 구조

.app과 .deb 패키지는 OS별로 구조가 다르며, cargo-bundle을 통해 생성된 앱은 리소스가 특정 위치에 복사됩니다.  
아래에 각 플랫폼별 내부 구조와 리소스 접근 경로를 정리.

## 🍎 macOS .app 번들 구조
### 📁 기본 구조
```
MyApp.app/
├── Contents/
│   ├── MacOS/              # 실행 파일 위치
│   │   └── my_app
│   ├── Resources/          # 아이콘 및 리소스 파일
│   │   └── config.json
│   └── Info.plist          # 앱 메타 정보
```

- Resources/ 디렉토리에 Cargo.toml의 resources = ["assets"]로 지정한 파일/디렉토리가 복사됨
- 실행 중에는 NSBundle.mainBundle().resourcePath 또는 Rust에서 std::env::current_exe()로 경로 계산 가능
### 📦 Rust에서 리소스 접근 예시
```rust
use std::env;
use std::path::PathBuf;

fn resource_path() -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    exe_path
        .parent().unwrap()               // MacOS/
        .parent().unwrap()               // Contents/
        .join("Resources")
        .join("config.json")
}
```


## 🐧 Linux .deb 패키지 구조
### 📁 기본 설치 경로
```
/usr/
├── bin/                  # 실행 파일
│   └── my_app
├── share/
│   └── my_app/
│       ├── config.json   # 리소스 파일
│       └── assets/
```

- resources = ["assets"]로 지정하면 /usr/share/<app_name>/에 복사됨
- 앱에서 std::env::var("CARGO_BUNDLE_RESOURCE_DIR")로 접근 가능 (환경 변수 설정 시)

### 📦 Rust에서 리소스 접근 예시
```rust
se std::path::PathBuf;

fn resource_path() -> PathBuf {
    PathBuf::from("/usr/share/my_app/config.json")
}
```

실제 배포 시에는 .desktop 파일이나 런처 스크립트에서 환경 변수로 경로를 넘겨주는 방식도 사용됨

## 🧩 요약
| 플랫폼         | 리소스 위치                          | 런타임 접근 방식                          |
|----------------|---------------------------------------|-------------------------------------------|
| macOS `.app`   | `MyApp.app/Contents/Resources/`       | `env::current_exe()` → `.app` 구조 따라 경로 계산 |
| Linux `.deb`   | `/usr/share/<app_name>/`              | `CARGO_BUNDLE_RESOURCE_DIR` 또는 고정 경로 사용   |

---

# LD_LIBRARY_PATH 
LD_LIBRARY_PATH 문제는 **동적 라이브러리(.so 파일)** 를 사용하는 앱에서 자주 발생하는데,  
cargo-bundle이나 cargo-dist로 배포할 때도 이 경로 설정이 중요:

## 🧠 LD_LIBRARY_PATH란?
- Linux에서 **동적 라이브러리(.so)** 를 찾는 경로를 지정하는 환경 변수
- 기본적으로 /usr/lib, /lib 등 시스템 경로를 탐색
- 앱이 커스텀 라이브러리 경로를 사용할 경우 이 변수로 알려줘야 함

## 🚫 문제가 생기는 경우
| 항목         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| `.so`         | 실행 파일이 `.so` 라이브러리를 찾지 못하면 런타임 오류 발생 (`cannot open shared object file`) |
| `.so` + `LD_LIBRARY_PATH` | 경로가 시스템 기본 경로가 아니면 `LD_LIBRARY_PATH` 설정이 필요                     |
| `.deb` 패키지 | `/usr/share/my_app/libfoo.so` 같은 경로는 자동 탐색되지 않음 → 경로 설정 필요              |



## ✅ 문제를 피하는 전략
### 1. 런타임에 LD_LIBRARY_PATH 설정
```
export LD_LIBRARY_PATH=/usr/share/my_app:$LD_LIBRARY_PATH
./my_app
```
- 실행 스크립트(run.sh)로 자동 설정 가능

### 2. 앱 내부에서 경로 설정 (Rust)
```rust
use std::env;
fn setup_library_path() {
    let lib_path = "/usr/share/my_app";
    let current = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    let new_path = format!("{}:{}", lib_path, current);
    env::set_var("LD_LIBRARY_PATH", new_path);
}
```

단, 이 방식은 이미 실행된 프로세스에는 적용되지 않음. 외부에서 설정하는 것이 더 안전함.

### 3. rpath 또는 static linking 사용
- rpath: 실행 파일에 .so 경로를 내장
```
rustc -C link-args="-Wl,-rpath,/usr/share/my_app"
```
- static linking: .so 대신 .a 정적 라이브러리로 빌드 → LD_LIBRARY_PATH 불필요

## 🧩 요약
| 상황 또는 목적                         | 해결 전략 또는 설정 방식                          |
|----------------------------------------|---------------------------------------------------|
| 실행 시 라이브러리 경로 지정            | `LD_LIBRARY_PATH=/path/to/libs ./my_app`          |
| `.deb` 설치 시 시스템 경로에 배치       | `/usr/lib/lib.so`                                 |
| 실행 파일에 라이브러리 경로 내장        | `-Wl,-rpath` 링크 옵션 사용                        |
| 정적 링크로 경로 문제 제거              | `.a` 정적 라이브러리 사용                          |

- 앱 내부에서 LD_LIBRARY_PATH를 설정하는 건 이미 늦습니다.

### ❌ 왜 의미가 없을까?
- .so 동적 라이브러리는 프로그램 시작 전에 로딩되어야 합니다.
- Rust 실행 파일이 시작되기 전에 OS가 필요한 .so를 찾지 못하면 실행 자체가 실패합니다.
- 따라서 main() 함수에서 env::set_var("LD_LIBRARY_PATH", ...)를 호출해도 이미 .so가 없어서 프로세스가 시작되지 못한 상태일 수 있음.

## ✅ 올바른 해결 전략
| 상황 또는 목적                         | 해결 전략 또는 설정 방식                          | 비고 또는 효과                                   |
|----------------------------------------|---------------------------------------------------|--------------------------------------------------|
| 실행 시 라이브러리 경로 지정            | `LD_LIBRARY_PATH=/path/to/libs ./my_app`          | 셸에서 환경 변수 설정 후 실행                     |
| 시스템 경로에 `.so` 배치               | `.so` → `/usr/lib/lib.so`                         | OS가 자동으로 탐색 가능                           |
| 실행 파일에 라이브러리 경로 내장        | `-Wl,-rpath,/path/to/libs`                        | 실행 파일에 경로 포함 → 외부 설정 불필요           |
| 정적 링크로 경로 문제 제거              | `.a` 정적 라이브러리 사용                         | `.so` 불필요 → 배포 간편, 크기 증가 가능성 있음     |

## 🧠 요약
앱 내부에서 경로를 설정하는 건 이미 실행된 이후에만 유효하므로 .so가 필요한 경우에는 실행 전에 경로를 잡아주는 방식이 필수입니다.  
실행 스크립트나 설치 경로 설계가 핵심.

## 💬 Rust가 C++보다 나은 점도 있어요
| 항목 또는 기능                         | Rust에서의 장점 또는 도구                          |
|----------------------------------------|---------------------------------------------------|
| 정적 라이브러리 생성                   | `crate-type = ["staticlib"]` → `.a` 파일 생성 가능 |
| 빌드 자동화                            | `cargo`가 `Makefile` 없이 의존성 및 빌드 관리       |
| rpath 설정                             | `-C link-args="-Wl,-rpath=..."`로 간단하게 설정 가능 |
| 크로스 컴파일                          | `cross`, `cargo-zigbuild`로 플랫폼별 빌드 지원      |

## 🧠 Rust에서 현실적인 선택
- 가능하면 정적 링크로 .so 문제 자체를 제거
- 동적 링크가 필요하다면 rpath 또는 설치 스크립트로 LD_LIBRARY_PATH 설정
- GUI 앱이라면 cargo-bundle로 리소스와 .so를 함께 배포

---

# PathBuf

## 🧭 PathBuf → Path 변환 방법
```rust
use std::path::{Path, PathBuf};

fn main() {
    let path_buf = PathBuf::from("example.txt");

    // &Path로 변환
    let path: &Path = path_buf.as_path();

    // I/O 함수에 사용
    std::fs::read_to_string(path).unwrap();
}
```


## 📌 핵심 메서드 — PathBuf & Path 변환 요약
| 메서드       | 반환 타입           | 설명                                                         |
|--------------|---------------------|--------------------------------------------------------------|
| as_path()    | &Path               | `PathBuf`를 `&Path`로 변환하여 I/O 함수에 사용할 수 있음     |
| to_str()     | Option<&str>        | `Path`를 UTF-8 문자열로 변환 (실패 가능성 있음)              |
| display()    | Display             | `Path`를 출력용으로 포맷팅 (디버깅, 로그에 유용)             |



## 🛠️ 실전 예시: 파일 읽기
```rust
use std::fs;
use std::path::PathBuf;

fn read_file(path_buf: PathBuf) -> String {
    let path = path_buf.as_path(); // 변환
    fs::read_to_string(path).expect("파일을 읽을 수 없습니다")
}
```
---






