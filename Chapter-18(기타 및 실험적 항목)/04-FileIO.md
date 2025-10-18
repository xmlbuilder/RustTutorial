# Rust 파일 읽고 쓰기
std::fs, std::io, std::path 세 가지 모듈의 역할과 주의사항을 정리 + 보완 설명으로 구성.  
실전에서 자주 마주치는 오류와 그 해결 팁도 함께 담았습니다.

## 📦 Rust에서 파일 입출력을 위한 핵심 모듈 3가지
### 1️⃣ std::fs – 파일 시스템 접근
- 주요 기능: 파일 생성, 열기, 삭제, 복사 등
- 로컬 파일 시스템에 있는 파일을 처리 하기 위한 모듈
- 운영체제에 상관없이 사용 할 수 있는 기능들을 모아좋은 것
- File 구조체가 일반 파일에 접근 할 때 사용
- 대표 타입: File, OpenOptions
- 예시:
```rust
let file = File::open("data.txt")?;
let mut file = File::create("output.txt")?;
```

- 주의사항:
- File::open()은 읽기 전용으로 열기 때문에 쓰기 작업은 불가능
- 파일이 없으면 ErrorKind::NotFound 오류 발생
- 파일을 여러 번 열거나 동시에 접근할 경우 OS에 따라 충돌 가능성 있음


### 2️⃣ std::io – 입출력 트레이트와 도구
- 주요 기능: `Read`, `Write`, `BufReader`, `BufWriter`, 에러 처리
- 입출력을 위한 타입, 라이블러리, 에러 타입등을 모아 놓은 모듈
- 트레이트 기반: File은 Read, Write 트레이트를 구현함
- 트레이트는 추상화 객체이므로 use std::io::Read 라는 트레이트를 사용하지만 실제 구현체는 std::fs::File에 있습니다
- 예시:
```rust
use std::io::Read;
let mut buffer = String::new();
file.read_to_string(&mut buffer)?;
```

- 주의사항:
- `read_to_string()` 은 전체 파일을 메모리에 로드하므로 큰 파일에는 부적합
- 반복적으로 읽을 경우 `BufReader` 를 사용하는 것이 성능에 유리
- `?` 연산자를 사용하면 에러를 자동 전파할 수 있어 코드가 깔끔해짐

### 3️⃣ std::path – 경로 추상화
- 주요 타입: `Path`, `PathBuf`
- 파일을 처리 하기 위해서 파일의 경로를 알아야 합니다.
- PathBuf: 가변 경로 (push 가능), Path: 불변 참조
- 예시:
```rust
let mut path = current_dir()?;
path.push("src/main.rs");
grep(&path, "main")?;
```

- 주의사항:
- PathBuf는 Path로 자동 참조 변환됨 (&PathBuf → &Path)
- 경로가 OS에 따라 다르므로 Path는 플랫폼 독립적인 추상화 제공
- `Path::exists()` 같은 메서드는 없고, `std::fs::metadata()` 로 확인해야 함

## 🧪 에러 처리 흐름
Rust는 에러를 `Result<T, E>` 로 처리하며, `?` 연산자를 통해 간결하게 전파할 수 있습니다.
```rust
fn grep(filename: &Path, word: &str) -> std::io::Result<()> {
    let mut f = File::open(filename)?; // 파일 열기 실패 시 즉시 반환
    let mut text = String::new();
    f.read_to_string(&mut text)?;      // 읽기 실패 시 즉시 반환

    for line in text.lines() {
        if line.contains(word) {
            println!("line - {line}");
        }
    }
    Ok(())
}
```

- `?` 는 Result 타입에서 Err일 경우 즉시 반환
- `unwrap()` 은 테스트나 빠른 프로토타입에만 사용하고, 실전에서는 `?` 또는 match로 처리하는 것이 안전

## ⚠️ 자주 마주치는 오류와 해결 팁

| 오류 메시지                  | 원인 또는 상황 설명                          | 해결 방법 또는 팁                          |
|-----------------------------|----------------------------------------------|--------------------------------------------|
| `No such file or directory` | 지정한 경로에 파일이 없음                    | `println!("{:?}", path)`로 경로 확인<br>`PathBuf.push()` 순서 점검 |
| `Permission denied`         | 파일에 접근할 권한이 없음                    | 관리자 권한으로 실행하거나 파일 권한 수정   |
| `Is a directory`            | 디렉토리를 파일처럼 열려고 시도함            | `std::fs::metadata()`로 타입 확인 후 처리   |


## ✅ 보너스: 파일 존재 여부 확인

use std::fs;

if fs::metadata("src/main.rs").is_ok() {
    println!("파일이 존재합니다.");
} else {
    println!("파일이 없습니다.");
}

## 전체 코드
```rust
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf}
};

fn grep(filename: &Path, word: &str){
    let mut f = File::open(filename).unwrap();
    let mut text_buffer = String::new();
    f.read_to_string(&mut text_buffer).unwrap();

    for line in text_buffer.split('\n') {
        if line.contains(word){
            println!("line - {line}");
        }
    }
}

fn main(){
    let mut filename  = current_dir().unwrap();
    println!("filename = {:?}", filename); //"/Users/jeongjunghwan/Rust/rock-paper-scissors"
    filename.push("src/main.rs");
    println!("filename = {:?}", filename); //"/Users/jeongjunghwan/Rust/rock-paper-scissors/src/main.rs"
    grep(&filename, "main");
}
```

### 에러 처리 포함
```rust
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf}
};

fn grep(filename: &Path, word: &str) -> std::io::Result<()>{

    let mut f = File::open(filename)?;
    let mut text_buffer = String::new();

    f.read_to_string(&mut text_buffer)?;
    for line in text_buffer.split('\n') {
        if line.contains(word){
            println!("line - {line}");
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut filename  = current_dir().unwrap();
    println!("filename = {:?}", filename);
    filename.push("src/main1.rs");
    println!("filename = {:?}", filename);
    grep(&filename, "main")?;
    Ok(())
}

```

### 출력 결과
```
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

---

## 파일 관련 Utility

```rust
use std::{env, fs, io};
use std::path::{Path, PathBuf};
use std::collections::BTreeSet;
use ordered_float::OrderedFloat;
use regex::Regex;

/// String trimming
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// Case conversion (uppercase/lowercase)
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn to_lower(s: &str) -> String {
    s.to_lowercase()
}

/// Extracting path information
pub fn get_file_name(path: &str) -> Option<String> {
    Path::new(path).file_name()?.to_str().map(|s| s.to_string())
}

pub fn get_extension(path: &str) -> Option<String> {
    Path::new(path).extension()?.to_str().map(|s| s.to_string())
}

pub fn get_file_stem(path: &str) -> Option<String> {
    Path::new(path).file_stem()?.to_str().map(|s| s.to_string())
}

pub fn get_dir_name(path: &str) -> Option<String> {
    Path::new(path).parent()?.to_str().map(|s| s.to_string())
}

pub fn get_base_dir() -> io::Result<PathBuf> {
    let base_dir = {
        let exe = env::current_exe()?;
        exe.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no parent for current_exe"))?
            .to_path_buf()
    };
    Ok(base_dir)
}


/// Checking file existence
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Creating a directory
pub fn create_directory(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

/// Deleting a file
pub fn delete_file(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)
}

/// Deleting a directory recursively
pub fn delete_file_all(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}

/// Copying a file
pub fn copy_file(from: &str, to: &str) -> std::io::Result<u64> {
    fs::copy(from, to)
}

/// Moving a file
pub fn move_file(from: &str, to: &str) -> std::io::Result<()> {
    fs::rename(from, to)
}

/// Returning platform name
pub fn get_platform_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        if cfg!(target_arch = "x86_64") {
            "windows_x64"
        } else {
            "windows_x86"
        }
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "macos")]
    {
        "osx"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "unknown"
    }
}

/// Splitting string into tokens
pub fn tokenize(input: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    re.split(input)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Parsing numeric array from string: "1,2:5,10:20;2"
pub fn parse_array(input: &str) -> Vec<f64> {
    let mut result = BTreeSet::new();
    for token in input.split(|c| c == ',' || c == '\n') {
        if let Some((start, rest)) = token.split_once(':') {
            let start: f64 = start.trim().parse().unwrap_or(0.0);
            if let Some((end, step)) = rest.split_once(';') {
                let end: f64 = end.trim().parse().unwrap_or(start);
                let step: f64 = step.trim().parse().unwrap_or(1.0);
                let mut val = start;
                while val <= end {
                    result.insert(OrderedFloat(val));

                    val += step;
                }
            } else {
                let end: f64 = rest.trim().parse().unwrap_or(start);
                for val in (start as usize)..=(end as usize) {
                    result.insert(OrderedFloat(val as f64));
                }
            }
        } else {
            if let Ok(val) = token.trim().parse::<f64>() {
                result.insert(OrderedFloat(val));
            }
        }
    }
    let values: Vec<f64> = result.into_iter().map(|x| x.into_inner()).collect();
    values
}

/// Finding files with specific extension in a directory
pub fn find_files_with_extension(dir: &str, ext: &str) -> Vec<String> {
    let mut result = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == ext) {
                result.push(path.to_string_lossy().to_string());
            }
        }
    }
    result
}

/// Recursively searching for files
pub fn find_recursive_files(dir: &str) -> Vec<String> {
    let mut result = vec![];
    for entry in walkdir::WalkDir::new(dir).into_iter().flatten() {
        let path = entry.path();
        if path.is_file() {
            result.push(path.to_string_lossy().to_string());
        }
    }
    result
}

/// Recursively searching for directories 
pub fn find_recursive_directories(dir: &str) -> Vec<String> {
    let mut result = vec![];
    for entry in walkdir::WalkDir::new(dir).into_iter().flatten() {
        let path = entry.path();
        if path.is_dir() {
            result.push(path.to_string_lossy().to_string());
        }
    }
    result
}

/// Filename generator: prefix + number 
pub fn get_new_name(prefix: &str, format: &str, existing: &[String]) -> String {
    let mut max_num = 0;
    for name in existing {
        if name.starts_with(prefix) {
            let digits = name[prefix.len()..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            if let Ok(num) = digits.parse::<usize>() {
                max_num = max_num.max(num);
            }
        }
    }
    format!("{}{}", prefix, format.replace("{}", &(max_num + 1).to_string()))
}

/// Get Temp Path
pub fn temp_path(name: &str) -> String {
    let mut p = std::env::temp_dir();
    p.push(name);
    p.to_string_lossy().into_owned()
}
```




