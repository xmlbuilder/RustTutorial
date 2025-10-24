# Path

Path에서 자주 사용되는 주요 메서드들을 정리한 표와 각 메서드에 대한 Rust 예제입니다.  
이 표는 경로 검사, 변환, 추출 등 실전에서 자주 쓰이는 기능들을 중심으로 구성.

## 📦 Path 주요 메서드 정리
| 메서드             | 설명                                      |
|--------------------|-------------------------------------------|
| new                | 문자열을 Path로 변환                      |
| exists             | 경로가 실제로 존재하는지 확인             |
| is_file            | 경로가 파일인지 확인                      |
| is_dir             | 경로가 디렉터리인지 확인                  |
| parent             | 상위 디렉터리 반환                        |
| file_name          | 파일 이름 반환                            |
| file_stem          | 확장자 없는 파일 이름 반환                |
| extension          | 파일 확장자 반환                          |
| to_str             | Path를 Option<&str>로 변환 (UTF-8만 가능) |
| to_string_lossy    | Path를 Cow<str>로 변환 (손실 허용)        |
| to_path_buf        | Path를 PathBuf로 복사                     |
| is_absolute        | 절대 경로인지 확인                        |
| is_relative        | 상대 경로인지 확인                        |
| has_root           | 루트가 있는 경로인지 확인                 |



## 🧪 각 메서드별 예제
### 1. Path::new
```rust
use std::path::Path;

fn main() {
    let path = Path::new("/home/user/file.txt");
    println!("{:?}", path);
}
```


### 2. exists
```rust
use std::path::Path;

fn main() {
    let path = Path::new("Cargo.toml");
    println!("Exists: {}", path.exists());
}

```

### 3. is_file / is_dir
```rust
use std::path::Path;

fn main() {
    let path = Path::new("Cargo.toml");
    println!("Is file: {}", path.is_file());
    println!("Is dir: {}", path.is_dir());
}
```


### 4. parent
```rust
use std::path::Path;

fn main() {
    let path = Path::new("/home/user/file.txt");
    if let Some(parent) = path.parent() {
        println!("Parent: {:?}", parent);
    }
}
```


### 5. file_name
```rust
use std::path::Path;

fn main() {
    let path = Path::new("/home/user/file.txt");
    println!("File name: {:?}", path.file_name());
}
```


### 6. file_stem
```rust
use std::path::Path;

fn main() {
    let path = Path::new("report.pdf");
    println!("File stem: {:?}", path.file_stem()); // 출력: "report"

}
```

### 7. extension
```rust
use std::path::Path;

fn main() {
    let path = Path::new("report.pdf");
    println!("Extension: {:?}", path.extension()); // 출력: "pdf"

}
```


### 8. to_str
```rust
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    println!("As str: {:?}", path.to_str());
}
```


### 9. to_string_lossy
```rust
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    println!("Lossy string: {}", path.to_string_lossy());
}
```

### 10. to_path_buf
```rust
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("hello.txt");
    let buf: PathBuf = path.to_path_buf();
    println!("PathBuf: {:?}", buf);
}
```


### 11. is_absolute / is_relative
```rust
use std::path::Path;

fn main() {
    let path = Path::new("/usr/bin");
    println!("Is absolute: {}", path.is_absolute());
    println!("Is relative: {}", path.is_relative());
}
```


## 12. has_root
```rust
use std::path::Path;

fn main() {
    let path = Path::new("/usr/bin");
    println!("Has root: {}", path.has_root());
}
```

---

# 경로 추가

### 🛠️ 실전 샘플: 디렉토리에 파일 경로 추가하기
```rust
use std::path::PathBuf;

fn main() {
    // 기본 디렉토리 경로 생성
    let mut dir = PathBuf::from("/home/junghwan/projects");

    // 파일 이름 추가 (경로 조합)
    dir.push("report.txt");

    println!("전체 파일 경로: {:?}", dir);
}
```

### 🔍 결과
```
전체 파일 경로: "/home/junghwan/projects/report.txt"
```
- PathBuf::from()으로 시작 경로를 만들고
- push()로 하위 파일이나 디렉토리를 추가합니다

## 📦 추가 예제: 확장자 변경 + 경로 복사
```rust
use std::path::{PathBuf, Path};

fn main() {
    let mut path = PathBuf::from("/home/junghwan/data/raw");
    path.push("image");
    path.set_extension("png");

    let readonly: &Path = path.as_path(); // 읽기 전용 참조로 변환

    println!("최종 경로: {:?}", readonly);
}
```
- set_extension()으로 파일 확장자를 변경
- as_path()로 PathBuf를 &Path로 변환해 읽기 전용으로 사용


---

# 절대 / 상대 경로 전환하기

## 🔍 절대 경로로 변환: canonicalize()
- `std::fs::canonicalize()` 는 경로를 절대 경로로 변환해줍니다.
- 이 함수는 실제 파일 시스템을 참조하기 때문에 경로가 존재해야 작동합니다.
### ✅ 예제: 상대 경로 → 절대 경로
```rust
use std::path::Path;
use std::fs;

fn main() {
    let relative = Path::new("src/main.rs");

    match fs::canonicalize(relative) {
        Ok(absolute) => println!("절대 경로: {:?}", absolute),
        Err(e) => println!("오류: {}", e),
    }
}
```

- `"src/main.rs"` 가 실제로 존재하면 절대 경로로 변환됨
- 존재하지 않으면 Err 반환

## 🔄 절대 경로 → 상대 경로?
Rust 표준 라이브러리에는 절대 경로를 상대 경로로 변환하는 직접적인 함수는 없습니다.  
하지만 다음과 같은 방식으로 구현할 수 있음:
### 🛠️ 예제: 절대 경로에서 기준 경로를 기준으로 상대 경로 계산
```rust
use std::path::{Path, PathBuf};

fn main() {
    let base = Path::new("/home/junghwan/projects");
    let target = Path::new("/home/junghwan/projects/src/main.rs");

    if let Ok(relative) = target.strip_prefix(base) {
        println!("상대 경로: {:?}", relative); // 출력: "src/main.rs"
    }
}
```
- `strip_prefix()` 는 기준 경로를 제거해서 상대 경로를 계산합니다
- 경로가 포함 관계일 때만 작동합니다

## 📌 요약

| 변환 방향           | 사용 메서드           | 설명                                      |
|--------------------|------------------------|-------------------------------------------|
| 상대 → 절대 경로   | `fs::canonicalize()`   | 실제 파일 시스템 기준으로 절대 경로 반환 |
| 절대 → 상대 경로   | `strip_prefix(base)`   | 기준 경로를 제거해 상대 경로 계산         |

---


