## 🧰 Rust 유틸리티 함수 정리
| 범주             | 함수 이름                         | 설명                                                             | 반환값                      |
|------------------|-----------------------------------|------------------------------------------------------------------|-----------------------------|
| 문자열 처리      | `trim(s)`                         | 앞뒤 공백 제거                                                   | `String`                   |
|                  | `to_upper(s)`                     | 대문자로 변환                                                    | `String`                   |
|                  | `to_lower(s)`                     | 소문자로 변환                                                    | `String`                   |
| 경로 정보 추출   | `get_file_name(path)`             | 파일 이름 추출                                                   | `Option<String>`           |
|                  | `get_extension(path)`             | 확장자 추출                                                      | `Option<String>`           |
|                  | `get_file_stem(path)`             | 확장자 없는 파일 이름 추출                                       | `Option<String>`           |
|                  | `get_dir_name(path)`              | 상위 디렉터리 경로 추출                                          | `Option<String>`           |
|                  | `get_base_dir()`                  | 실행 파일 기준 루트 디렉터리 반환                                | `Result<PathBuf>`          |
| 파일/디렉터리    | `file_exists(path)`               | 파일 또는 디렉터리 존재 여부 확인                                | `bool`                     |
|                  | `create_directory(path)`          | 디렉터리 생성 (재귀 포함)                                        | `Result<()>`               |
|                  | `delete_file(path)`               | 파일 삭제                                                        | `Result<()>`               |
|                  | `delete_file_all(path)`           | 디렉터리 전체 삭제                                               | `Result<()>`               |
|                  | `copy_file(from, to)`             | 파일 복사                                                        | `Result<u64>`              |
|                  | `move_file(from, to)`             | 파일 이동                                                        | `Result<()>`               |
| 플랫폼 정보      | `get_platform_name()`             | OS 및 아키텍처 기반 플랫폼 이름 반환                             | `&'static str`             |
| 문자열 분할      | `tokenize(input, pattern)`        | 정규식 기반 문자열 분할                                          | `Vec<String>`              |
| 숫자 파싱        | `parse_array(input)`              | `"1,2:5,10:20;2"` 형식의 숫자 배열 파싱                          | `Vec<f64>`                 |
| 파일 검색        | `find_files_with_extension(dir, ext)` | 특정 확장자 파일 검색                                       | `Vec<String>`              |
|                  | `find_recursive_files(dir)`       | 디렉터리 내 모든 파일 재귀 검색                                  | `Vec<String>`              |
|                  | `find_recursive_directories(dir)` | 디렉터리 내 모든 하위 디렉터리 재귀 검색                         | `Vec<String>`              |
| 이름 생성        | `get_new_name(prefix, format, existing)` | 접두어 + 숫자 기반 새 이름 생성                         | `String`                   |
| 임시 경로        | `temp_path(name)`                 | 시스템 임시 디렉터리 기반 경로 생성                              | `String`                   |

### ✅ 활용 예시
- `parse_array("1,2:4;1")` → [1.0, 2.0, 3.0, 4.0]
- `get_new_name("file", "_{}", &["file_1", "file_2"])` → "file_3"
- `tokenize("a,b;c", "[,;]")` → ["a", "b", "c"]
- `get_platform_name()` → "windows_x64" 또는 "linux" 등

---


# 코드 테스트

## 테스트 항목
| 테스트 함수 이름         | 검증 항목 설명                                      |
|--------------------------|-----------------------------------------------------|
| `test_trim_and_case`     | 문자열 트리밍 및 대소문자 변환                     |
| `test_path_functions`    | 파일 이름, 확장자, 스템 추출                       |
| `test_get_dir_name`      | 상위 디렉터리 경로 추출                            |
| `test_tokenize`          | 정규식 기반 문자열 분할                            |
| `test_parse_array`       | 숫자 배열 파싱 (`1,2:4,10:14;2` → `[1,2,3,4,10...]`) |
| `test_platform_name`     | 플랫폼 이름 반환 (`windows_x64`, `linux` 등)       |
| `test_create_directory`  | 디렉터리 생성 (`create_directory`)                 |
| `test_file_exists`       | 파일 존재 여부 확인                                |
| `test_get_new_name`      | 접두어 + 숫자 기반 새 이름 생성                   |
| `test_temp_path`         | 임시 경로 생성 (`temp_dir` 기반)                   |
| `test_regex`             | 정규식 매칭, 치환, 분할 동작 확인                  |

### 1. test_trim_and_case
```rust
#[test]
fn test_trim_and_case() {
    assert_eq!(trim("  hello  "), "hello");
    assert_eq!(to_upper("rust"), "RUST");
    assert_eq!(to_lower("RUST"), "rust");
}
```
### 2. test_path_functions
```rust
#[test]
fn test_path_functions() {
    let path = "C:/Users/JungHwan/report.pdf";
    assert_eq!(get_file_name(path), Some("report.pdf".to_string()));
    assert_eq!(get_extension(path), Some("pdf".to_string()));
    assert_eq!(get_file_stem(path), Some("report".to_string()));
}
```

### 3. test_tokenize
```rust
#[test]
fn test_tokenize() {
    let tokens = tokenize("one, two three", r"[,\s]+");
    assert_eq!(tokens, vec!["one", "two", "three"]);
}
```

### 3. test_parse_array
```rust
#[test]
fn test_parse_array() {
    let parsed = parse_array("1,2:4,10:14;2");
    assert_eq!(parsed, vec![1.0, 2.0, 3.0, 4.0, 10.0, 12.0, 14.0]);
}
```

### 4. test_platform_name
```rust
#[test]
fn test_platform_name() {
    let name = get_platform_name();
    assert!(["windows_x64", "windows_x86", "linux", "osx", "unknown"].contains(&name));
}
```

### 5. test_create_directory
```rust
#[test]
fn test_create_directory() {
    match create_directory("output") {
        Ok(_) => println!("디렉토리 생성 성공"),
        Err(e) => eprintln!("실패: {}", e),
    }
}
```

### 6. test_regex
```rust
#[test]
fn test_regex() {
    let re = Regex::new(r"\d+").unwrap();
    let text = "abc123def456";

    // 숫자 찾기
    for mat in re.find_iter(text) {
        println!("Match: {}", mat.as_str());
    }

    // 치환
    let replaced = re.replace_all(text, "#");
    println!("Replaced: {}", replaced); // abc#def#

    let re = Regex::new(r"[,\s]+").unwrap();
    let tokens: Vec<&str> = re.split("one, two three").collect();
    println!("{:?}", tokens); // ["one", "two", "three"]
}
```

### 7. test_get_dir_name
```rust
#[test]
fn test_get_dir_name() {
    let path = "C:/Users/JungHwan/report.pdf";
    assert_eq!(get_file_stem(path), Some("report".to_string()));
    assert_eq!(get_dir_name(path), Some("C:/Users/JungHwan".to_string()));
}
```

### 8. test_get_dir_name
```rust
#[test]
fn test_file_exists() {
    use std::fs::File;
    let path = "temp_test_file.txt";
    File::create(path).unwrap();
    assert!(file_exists(path));
    std::fs::remove_file(path).unwrap();
}
```

### 9. test_get_new_name
```rust
#[test]
fn test_get_new_name() {
    use nurbslib::core::file_utils::get_new_name;
    let existing = vec!["file001".to_string(), "file002".to_string()];
    let new_name = get_new_name("file", "{}", &existing);
    assert_eq!(new_name, "file3");
}
```

### 10. test_temp_path
```rust
#[test]
fn test_temp_path() {
    use nurbslib::core::file_utils::temp_path;
    let path = temp_path("test.tmp");
    assert!(path.contains("test.tmp"));
}
```

---
