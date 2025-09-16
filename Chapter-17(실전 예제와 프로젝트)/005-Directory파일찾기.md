# Directory
 Rust에서 디렉토리 내 파일 탐색 및 정보 추출을 구현한 예시.
이 구조는 실전에서 파일 필터링, 확장자 검사, 디렉토리 트리 탐색 등에 유용하게 쓰입니다.

## 🧠 핵심 기능 요약
| 기능                  | 설명                                               |
|-----------------------|----------------------------------------------------|
| `fs::read_dir()`      | 지정된 경로의 디렉토리 항목들을 반복자로 반환       |
| `entry.metadata()`    | 해당 항목의 메타데이터 조회 (파일/디렉토리 여부 등) |
| `entry.path()`        | 항목의 전체 경로(`PathBuf`) 반환                    |
| `file_name()`         | 파일 이름 추출 (`OsStr` 타입)                       |
| `extension()`         | 파일 확장자 추출 (`Some("txt")` 등)                |
| `is_file() / is_dir()`| 항목이 파일인지 디렉토리인지 판별                   |


## ✅ 코드 흐름 설명
```rust
const SAMPLE_DIR: &str = "D:/Development/Rust/communicator";
let sample_dir_path = path::Path::new(SAMPLE_DIR);
```

- 문자열 경로를 Path 객체로 변환
- Path는 파일 시스템 경로를 안전하게 다루는 구조체
```rust
for entry in fs::read_dir(sample_dir_path)? {
    let entry = entry?;
    let metadata = entry.metadata()?;
```

- read_dir()는 Result<DirEntry> 반복자를 반환
- 각 항목에 대해 metadata()를 호출해 파일인지 디렉토리인지 확인
```rust
if metadata.is_file() {
    let entry_path = entry.path();
    if let Some(file_name) = entry_path.file_name() {
        println!("File filename : {:?}", file_name);
        if let Some(ext) = entry_path.extension() {
            println!("Extension {:?}", ext);
        }
    }
}
```

- 파일일 경우 file_name()과 extension()을 추출
- OsStr 타입이므로 필요시 .to_str()로 변환 가능

## 🧪 확장 예시: 특정 확장자만 필터링
```rust
if metadata.is_file() {
    if let Some(ext) = entry.path().extension() {
        if ext == "txt" {
            println!("텍스트 파일 발견: {:?}", entry.path());
        }
    }
}
```

## 전체 코드
```rust
use std::{fs, path};
use std::error::Error as StdError;

const SAMPLE_DIR: &str = "D:/Development/Rust/communicator";

fn main() -> Result<(), Box<dyn StdError>>{

    let sample_dir_path = path::Path::new(SAMPLE_DIR);

    println!("{:?}", sample_dir_path); //"D:/Development/Rust/communicator"

    for entry in fs::read_dir(sample_dir_path)? {

        let entry = entry?;

        println!("{:?}", entry);
    
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            
        } else if metadata.is_file() {

            let entry_path = entry.path();

            if let Some(file_name) = entry_path.file_name() {

                println!("File filename : {:?}", file_name);

                let ext = entry_path.extension();
                if ext.is_some() {
                    println!("Extension {:?}", ext.unwrap());
                }
            }
        }
    }
    Ok(())
}

```


## 📁 디렉토리 탐색 실전 활용
- 로그 파일 필터링: .log, .txt 확장자만 추출
- 이미지 처리: .png, .jpg 파일만 대상으로 처리
- 재귀 탐색: walkdir 크레이트를 사용하면 하위 디렉토리까지 탐색 가능

## ✅ 디렉토리 탐색 핵심 API

| API                    | 설명                                      |
|------------------------|-------------------------------------------|
| `fs::read_dir()`       | 디렉토리 항목 반복자 반환                 |
| `entry.metadata()`     | 파일/디렉토리 여부 확인                   |
| `entry.path()`         | 전체 경로(PathBuf) 반환                   |
| `file_name()`          | 파일 이름 추출 (`OsStr`)                  |
| `extension()`          | 확장자 추출 (`Some("txt")`)               |
| `PathBuf::to_str()`    | 경로를 문자열로 변환                      |


---
# 재귀적 탐색

Rust에서 디렉토리 내 파일을 재귀적으로 탐색하거나 특정 패턴으로 필터링하는 두 가지 강력한 방법을 정리해드릴게요:

### 🧭 1. walkdir을 활용한 재귀 디렉토리 탐색
walkdir 크레이트는 하위 디렉토리까지 포함한 전체 트리 탐색을 매우 효율적으로 처리합니다.

#### ✅ 설치
```
# Cargo.toml
[dependencies]
walkdir = "2"
```

#### ✅ 기본 예제
```rust
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("D:/Development/Rust/communicator")
        .into_iter()
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        if path.is_file() {
            println!("File: {:?}", path.display());
        }
    }
}
```


- WalkDir::new()로 시작 디렉토리 지정
- filter_map(|e| e.ok())로 오류 무시
- entry.path()로 PathBuf 반환

#### 🧪 고급 옵션
```rust
WalkDir::new("dir")
    .follow_links(true) // 심볼릭 링크도 따라감
    .max_depth(3)       // 최대 깊이 제한
    .into_iter()
```


📘 자세한 문서는 walkdir 공식 문서에서 확인할 수 있어요.

### 🎯 2. glob 패턴으로 특정 파일명 매칭
glob 크레이트는 Unix 스타일의 와일드카드 패턴으로 파일을 필터링할 수 있게 해줍니다.
#### ✅ 설치
```rust
# Cargo.toml
[dependencies]
glob = "0.3"
```

#### ✅ 기본 예제
```rust
use glob::glob;

fn main() {
    for entry in glob("D:/Development/Rust/communicator/**/*.txt")
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => println!("Matched: {:?}", path.display()),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
```

- **/*.txt: 모든 하위 디렉토리의 .txt 파일
- glob()는 Result<PathBuf> 반복자 반환
#### 🧪 여러 확장자 매칭
```rust
use glob::glob;

fn main() {
    let patterns = vec!["**/*.json", "**/*.jsonc"];
    for pattern in patterns {
        for entry in glob(pattern).unwrap() {
            if let Ok(path) = entry {
                println!("Found: {:?}", path.display());
            }
        }
    }
}
```

📘 고급 패턴은 glob 공식 문서에서 확인할 수 있어요.

## 📦 비교 요약
| 기능        | 크레이트  | 목적                          | 특징                                 |
|-------------|-----------|-------------------------------|--------------------------------------|
| `walkdir`   | walkdir   | 재귀 디렉토리 전체 탐색       | 빠르고 유연한 트리 탐색              |
| `glob`      | glob      | 패턴 기반 파일 필터링         | Unix 스타일 와일드카드 지원          |


### ✅ glob unwrap 요약표
| 표현식                      | 반환 타입                              | 설명                                 |
|-----------------------------|----------------------------------------|--------------------------------------|
| `glob(pattern)`             | `Result<Paths, PatternError>`          | 패턴이 유효한지 검사                 |
| `glob(pattern).unwrap()`    | `Paths`                                | 패턴이 유효하다고 가정하고 반복자 반환 |
| `for entry in Paths`        | `Result<PathBuf, GlobError>`           | 각 파일 경로에 대한 성공/실패 결과   |
| `match entry` 또는 `if let` | `Ok(path)` / `Err(e)`                  | 실제 경로 추출 또는 오류 처리        |


---



