# std::fs
아래는 std::fs 모듈에서 자주 사용되는 파일 시스템 함수들을 정리한 표와 각 함수에 대한 Rust 샘플 코드입니다.  
이 함수들은 파일 생성, 복사, 삭제, 디렉토리 조작 등 실전에서 매우 유용하게 쓰입니다.

## 📦 std::fs 주요 함수 정리
| 함수명               | 설명                                      |
|----------------------|-------------------------------------------|
| `create_dir`           | 새 디렉토리 생성                          |
| `create_dir_all`       | 상위 디렉토리 포함 전체 생성              |
| `remove_file`          | 파일 삭제                                 |
| `remove_dir`           | 빈 디렉토리 삭제                          |
| `remove_dir_all`       | 디렉토리 및 하위 내용 전체 삭제           |
| `copy`                 | 파일 복사                                 |
| `rename`               | 파일 또는 디렉토리 이름 변경              |
| `read_to_string`       | 파일 내용을 문자열로 읽기                 |
| `write`                | 문자열을 파일에 쓰기                      |
| `metadata`             | 파일 메타데이터 조회                      |
| `canonicalize`         | 경로를 절대 경로로 변환                   |



## 🧪 각 함수별 샘플 예제
### 1. create_dir
```rust
use std::fs;

fn main() {
    fs::create_dir("new_folder").expect("디렉토리 생성 실패");
}
```


### 2. create_dir_all
```rust
use std::fs;

fn main() {
    fs::create_dir_all("parent/child/grandchild").expect("전체 디렉토리 생성 실패");
}
```


### 3. remove_file
```rust
use std::fs;

fn main() {
    fs::remove_file("old.txt").expect("파일 삭제 실패");
}
```


### 4. remove_dir
```rust
use std::fs;

fn main() {
    fs::remove_dir("empty_folder").expect("빈 디렉토리 삭제 실패");
}
```


### 5. remove_dir_all
```rust
use std::fs;

fn main() {
    fs::remove_dir_all("project_backup").expect("전체 디렉토리 삭제 실패");
}
```


### 6. copy
```rust
use std::fs;

fn main() {
    fs::copy("source.txt", "destination.txt").expect("복사 실패");
}
```


### 7. rename
```rust
use std::fs;

fn main() {
    fs::rename("draft.txt", "final.txt").expect("이름 변경 실패");
}
```


### 8. read_to_string
```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("notes.txt").expect("읽기 실패");
    println!("내용: {}", content);
}
```


### 9. write
```rust
use std::fs;

fn main() {
    fs::write("output.txt", "Hello, Rust!").expect("쓰기 실패");
}
```


### 10. metadata
```rust
use std::fs;

fn main() {
    let meta = fs::metadata("output.txt").expect("메타데이터 조회 실패");
    println!("크기: {} bytes", meta.len());
}
```


### 11. canonicalize
```rust
use std::fs;
use std::path::Path;

fn main() {
    let abs_path = fs::canonicalize(Path::new("src/main.rs")).expect("경로 변환 실패");
    println!("절대 경로: {:?}", abs_path);
}
```

---
# fs::metadata

파일 메타데이터는 파일 자체의 내용이 아닌, 파일에 대한 정보를 말합니다.  
쉽게 말해, "파일에 붙어 있는 속성들".

## 🧠 파일 메타데이터란?
| 메서드         | 설명                                      |
|----------------|-------------------------------------------|
| len()          | 파일 크기 (바이트 단위)                   |
| created()      | 파일이 생성된 시간                        |
| modified()     | 마지막으로 수정된 시간                    |
| accessed()     | 마지막으로 읽힌 시간                      |
| permissions()  | 읽기/쓰기 권한 정보                       |
| is_file()      | 일반 파일인지 여부                        |
| is_dir()       | 디렉토리인지 여부                         |


## ✅ Rust 예제: std::fs::metadata
```rust
use std::fs;
use std::time::SystemTime;

fn main() {
    let meta = fs::metadata("example.txt").expect("파일 정보 조회 실패");

    println!("크기: {} bytes", meta.len());
    println!("파일인가요? {}", meta.is_file());
    println!("디렉토리인가요? {}", meta.is_dir());

    if let Ok(modified) = meta.modified() {
        println!("수정된 시간: {:?}", modified);
    }
}
```

- metadata()는 std::fs::Metadata 객체를 반환하며, 다양한 속성에 접근할 수 있게 해줍니다.
- 시간 관련 정보는 SystemTime 타입으로 반환되며, 비교나 포맷이 가능합니다.

## 📦 실전 활용 예시
- 로그 파일 크기 확인: 너무 커지면 삭제하거나 압축
- 백업 시점 판단: 마지막 수정 시간 기준으로 백업 여부 결정
- 파일 정리 도구: 오래된 파일 자동 삭제

---

