# Argument

## 🧠 핵심 개념: env::args()
- std::env::args()는 프로그램 실행 시 전달된 커맨드라인 인자들을 Iterator로 반환합니다.
- 첫 번째 인자는 항상 실행 파일 경로입니다.
- .collect::<Vec<String>>()으로 벡터로 수집하면 인덱스로 접근 가능해져요.

## ✅ 기본 사용 예시
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```


실행 예:
```
cargo run sample1 sample2
```

출력:
```
["target/debug/communicator.exe", "sample1", "sample2"]
```


## 🔍 인자 개수 확인 및 분리
```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());

    if args.len() == 3 {
        let arg1 = &args[1];
        let arg2 = &args[2];
        println!("arg1 for {}", arg1);
        println!("arg2 for {}", arg2);
    }
}
```

- args[0]: 실행 파일 경로
- args[1], args[2]: 사용자 입력 인자

## 📦 설정값 구조체로 그룹화
```rust
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = std::fs::File::open(&config.filename)
        .expect("file not found");
}
```

- Config 구조체로 인자들을 의미 단위로 묶음
- 실전 프로젝트에서 구성값 관리에 매우 유용한 패턴

## 📁 파일 읽기와 인자 연결
```rust
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("sample.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read error");
    println!("Test - contents\n{}", contents);
}
```

- 인자로 받은 파일명을 File::open()에 연결하면 검색 기능, 설정 파일 로딩 등에 활용 가능

## 🧪 실전 확장 아이디어
- args_os()로 유니코드가 깨질 수 있는 인자 처리
- clap, argh, structopt 같은 라이브러리로 인자 파싱 자동화
- Config에 Option<T>나 Result<T>를 활용해 유연한 에러 처리

---

## Rust 실전 프로젝트에서 자주 쓰이는 세 가지 기능을 정리:
- clap 기반 인자 파서 구현
- env::var()로 환경 변수 읽기
- std::path::PathBuf로 경로 처리
각 기능은 Rust의 안전성과 유연함을 실전에서 활용하는 핵심 도구입니다. 아래에 예제와 함께 정리해드릴게요.

### 🧰 1. clap 기반 인자 파서 구현
clap은 Rust에서 가장 널리 쓰이는 커맨드라인 인자 파서 라이브러리입니다.
#### ✅ 설정 방법
```
# Cargo.toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

#### ✅ 기본 예제
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 검색할 문자열
    #[arg(short, long)]
    query: String,

    /// 대상 파일 경로
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    println!("Searching for {}", args.query);
    println!("In file {}", args.filename);
}
```

- cargo run -- --query rust --filename sample.txt
- --help 자동 생성됨
- derive 매크로로 구조체 기반 파싱 가능
📘 자세한 문서는 clap 공식 문서에서 확인할 수 있어요.

### 🌐 2. env::var()로 환경 변수 읽기
Rust의 std::env 모듈을 사용하면 런타임 환경 변수를 읽을 수 있어요.
#### ✅ 기본 예제
```rust
use std::env;

fn main() {
    match env::var("MY_CONFIG") {
        Ok(val) => println!("MY_CONFIG: {}", val),
        Err(e) => println!("환경 변수 없음: {}", e),
    }
}
```

- 환경 변수 설정:
- Linux/macOS: MY_CONFIG=value cargo run
- Windows PowerShell: $env:MY_CONFIG="value"; cargo run
#### ✅ 모든 환경 변수 출력
```rust
for (key, value) in env::vars() {
    println!("{} = {}", key, value);
}
```

📘 더 많은 예제는 Thorsten Hans의 환경 변수 정리에서 확인할 수 있어요.

### 📁 3. std::path::PathBuf로 경로 처리
PathBuf는 가변 경로 객체로, 파일 경로를 안전하게 조작할 수 있게 해줍니다.
#### ✅ 기본 예제
```rust
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();
    path.push("data");
    path.push("config.json");

    println!("Full path: {:?}", path);
}
```

- push()로 경로 조합
- to_str()로 문자열 변환
- exists()로 파일 존재 여부 확인
#### ✅ 현재 디렉토리에서 파일 경로 만들기
```rust
use std::env;
use std::path::PathBuf;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("sample.txt");

    println!("Path: {:?}", path);
}
```

📘 더 많은 예제는 Rust 공식 PathBuf 문서에서 확인할 수 있어요.

### 🧭 요약표
| 기능                  | 설명                                       | 대표 메서드 / 매크로         |
|-----------------------|--------------------------------------------|-------------------------------|
| `clap` 인자 파서      | 구조체 기반 커맨드라인 인자 파싱          | `Args::parse()`               |
| 환경 변수 읽기        | 런타임 설정값 읽기                         | `env::var()`, `env::vars()`   |
| 경로 처리             | 파일 경로 조합 및 검사                     | `PathBuf::push()`, `exists()` |

---



