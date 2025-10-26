# Rust 파일 조작

## ✅ Rust에서 파일인지 디렉토리인지 확인하고, 파일 속성을 읽는 방법  
`std::fs::metadata` 와 `std::fs::symlink_metadata` 를 통해 수행할 수 있습니다.  
아래에 주요 메서드와 샘플 코드를 표와 함께 정리.

## 📦 Rust 파일 관련 주요 메서드 요약
| 메서드/속성                      | 설명                                           | 반환 타입             |
|----------------------------------|------------------------------------------------|------------------------|
| `Path::exists()`                | 경로 존재 여부 확인                            | `bool`                 |
| `fs::metadata(path)`            | 파일/디렉토리의 메타데이터 조회 (심볼릭 링크 해석) | `Result<Metadata>`     |
| `fs::symlink_metadata(path)`    | 심볼릭 링크 자체의 메타데이터 조회             | `Result<Metadata>`     |
| `metadata.is_file()`            | 일반 파일인지 확인                             | `bool`                 |
| `metadata.is_dir()`             | 디렉토리인지 확인                              | `bool`                 |
| `metadata.len()`                | 파일 크기 (바이트 단위)                         | `u64`                  |
| `metadata.permissions()`        | 권한 정보 (`readonly` 여부 등)                 | `Permissions`          |
| `metadata.modified()`           | 마지막 수정 시간                               | `Result<SystemTime>`   |
| `metadata.created()`            | 생성 시간 (일부 OS에서만 지원)                 | `Result<SystemTime>`   |
| `metadata.accessed()`           | 마지막 접근 시간 (일부 OS에서만 지원)           | `Result<SystemTime>`   |

## 🧪 샘플 코드: 파일/디렉토리 구분 및 속성 출력
```rust
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let path = Path::new("/Users/jeongjunghwan/sample.txt");

    if path.exists() {
        println!("✅ 경로 존재함: {}", path.display());

        let metadata = fs::metadata(path)?;

        if metadata.is_file() {
            println!("📄 일반 파일입니다.");
        } else if metadata.is_dir() {
            println!("📁 디렉토리입니다.");
        }

        println!("크기: {} bytes", metadata.len());
        println!("읽기 전용 여부: {}", metadata.permissions().readonly());

        if let Ok(modified) = metadata.modified() {
            println!("마지막 수정 시간: {:?}", modified);
        }

        if let Ok(created) = metadata.created() {
            println!("생성 시간: {:?}", created);
        }

        if let Ok(accessed) = metadata.accessed() {
            println!("마지막 접근 시간: {:?}", accessed);
        }
    } else {
        println!("❌ 경로가 존재하지 않습니다.");
    }

    Ok(())
}
```


## 🧠 보충 팁
- `Path::exists()` 는 내부적으로 `fs::metadata()` 를 호출하여 경로 존재 여부를 확인합니다.
- `fs::metadata()` 는 심볼릭 링크를 따라가며, 링크 자체를 확인하려면 `fs::symlink_metadata()` 를 사용합니다.
- 시간 관련 메서드(modified, created, accessed)는 일부 OS에서만 지원되며, `Result를 반환` 하므로 match 또는 if let으로 처리해야 합니다.

---



