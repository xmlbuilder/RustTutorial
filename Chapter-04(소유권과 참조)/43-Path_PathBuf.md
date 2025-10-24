# Path / PathBuf
- PathBuf는 소유권을 가진 경로 타입이고, Path는 참조 기반의 경로 타입입니다.  
- PathBuf는 경로를 생성·수정할 때 사용하고, Path는 경로를 읽거나 검사할 때 사용합니다.

## 🧠 Path vs PathBuf 개념 정리
| 항목         | Path                         | PathBuf                          |
|--------------|------------------------------|----------------------------------|
| 소유권       | 없음 (참조 타입)             | 있음 (소유 타입)                |
| 가변성       | 불변                         | 가변                             |
| 용도         | 경로 읽기, 검사               | 경로 생성, 수정, 조작            |
| 메모리 위치  | 주로 스택 또는 참조된 힙     | 힙에 저장된 데이터 소유          |
| 변환 가능성  | `to_path_buf()`로 복사 가능   | `as_path()`으로 참조 가능         |
| 예시 타입    | `&Path`                      | `PathBuf`                        |

- Path는 &str처럼 불변 참조이며, 경로 데이터를 소유하지 않습니다.
- PathBuf는 String처럼 소유권을 가진 가변 타입으로, 경로를 동적으로 조작할 수 있습니다.
- PathBuf는 Deref를 통해 Path로 자동 변환되므로 대부분의 Path 메서드를 사용할 수 있습니다.

## ✅ 샘플 예제: PathBuf 생성과 Path 참조
```rust
use std::path::{Path, PathBuf};

fn main() {
    let mut path_buf = PathBuf::from("/home/user");
    path_buf.push("documents");
    path_buf.set_extension("txt");

    let path: &Path = path_buf.as_path(); // PathBuf → &Path

    println!("PathBuf: {:?}", path_buf);
    println!("Path: {:?}", path);
}
```

- push()로 경로를 추가하고, set_extension()으로 확장자를 변경
- as_path()를 통해 PathBuf를 &Path로 변환하여 읽기 전용으로 사용

## 🛠️ 실전 예제: 파일 존재 여부 확인
```rust
use std::path::Path;

fn check_file_exists(path: &Path) {
    if path.exists() {
        println!("파일이 존재합니다: {:?}", path);
    } else {
        println!("파일이 없습니다: {:?}", path);
    }
}

fn main() {
    let path_buf = PathBuf::from("/home/user/data.txt");
    check_file_exists(&path_buf); // PathBuf → &Path
}
```

- Path는 exists(), is_file(), parent() 등 다양한 검사 메서드를 제공합니다.
- PathBuf는 소유권을 가지므로 경로를 생성하거나 수정한 후 Path로 넘겨서 검사할 수 있습니다.

## 🔄 변환 방법

| 변환 방향         | 사용 방법              |
|------------------|------------------------|
| PathBuf → &Path  | `path_buf.as_path()`   |
| &Path → PathBuf  | `path.to_path_buf()`   |


## 📌 언제 어떤 걸 써야 할까?
- 읽기만 할 때: &Path (예: 함수 인자, 경로 검사)
- 경로를 만들거나 수정할 때: PathBuf (예: 파일 생성, 경로 조합)
- 유연한 함수 인자 처리: impl AsRef<Path> 또는 impl Into<PathBuf>를 사용하면 &str, String, Path, PathBuf 모두 받을 수 있음

---

## 🧠 왜 Path가 더 자주 쓰일까?
- ✅ 읽기 전용 작업이 대부분: 파일 경로를 검사하거나 비교할 때는 소유권이 필요 없기 때문에 &Path로 충분합니다.
- ✅ 함수 인자에 적합: 많은 표준 라이브러리 함수들이 AsRef<Path>를 받기 때문에 &str, Path, PathBuf 모두 유연하게 사용할 수 있음.
- ✅ 성능 효율: 참조 기반이라 복사나 할당이 없고, 빠르게 처리할 수 있습니다.
- ✅ 경로 조작은 제한적: 경로를 수정하거나 생성하는 경우가 아니라면 PathBuf까지 필요하지 않음.

## 📌 예시: Path가 자주 쓰이는 함수들
```rust
use std::path::Path;

fn main() {
    let path = Path::new("/home/user/data.txt");

    if path.exists() {
        println!("파일이 존재합니다.");
    }

    if path.is_file() {
        println!("이건 파일입니다.");
    }

    if let Some(parent) = path.parent() {
        println!("상위 디렉터리: {:?}", parent);
    }
}
```

- exists(), is_file(), parent() 등은 모두 Path에서 제공하는 메서드입니다.
- 이처럼 읽기 전용 경로 처리는 대부분 Path로 충분합니다.

## 🔄 언제 PathBuf를 써야 할까?
- 경로를 조합하거나 수정할 때
- 경로를 오래 보존하거나 소유권을 넘겨야 할 때
- 예: push(), set_extension(), canonicalize() 같은 메서드는 PathBuf에서만 사용 가능

---



