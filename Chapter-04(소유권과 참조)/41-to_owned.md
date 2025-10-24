# to_owned
`to_owned()`는 참조된 데이터를 복사하여 소유권을 가진 값으로 변환하는 메서드입니다.  
ToOwned 트레잇을 구현한 타입에서 사용되며, 특히 `&str` → `String`, `&[T]` → `Vec<T>` 같은 변환에 자주 쓰입니다.

## 🧠 to_owned()란?
- to_owned()는  트레잇의 메서드입니다.
- 이 트레잇은 **불변 참조(&T)** 를 **소유된 값(T::Owned)** 으로 바꾸는 데 사용됩니다.
- Clone과 비슷하지만, `Clone` 은 `T` → `T` 복사이고, ToOwned는 `&T` → `T::Owned` 복사입니다.

## 📦 주요 구현 예시

| 참조 타입 | 반환 타입  |
|-----------|-------------|
| &str      | String      |
| &[T]      | Vec<T>      |
| &Path     | PathBuf     |
| &OsStr    | OsString    |


✅ 샘플 예제
```rust
fn main() {
    let s: &str = "hello";
    let owned: String = s.to_owned(); // &str → String

    let arr: &[i32] = &[1, 2, 3];
    let vec: Vec<i32> = arr.to_owned(); // &[i32] → Vec<i32>

    println!("String: {}", owned);
    println!("Vec: {:?}", vec);
}
```

- to_owned()는 참조된 데이터를 복사해서 소유권을 가진 타입으로 변환합니다.
- String과 Vec은 힙에 데이터를 저장하므로 복사가 발생합니다.

## 🛠️ 실전 예제: 파일 경로 처리
```rust
use std::path::Path;

fn main() {
    let path_ref: &Path = Path::new("/home/user/data.txt");
    let path_buf = path_ref.to_owned(); // &Path → PathBuf

    // PathBuf은 소유권을 가지므로 수정하거나 보존 가능
    println!("Owned path: {:?}", path_buf);
}
```

- PathBuf은 Path의 소유 버전으로, 파일 경로를 조작하거나 보존할 때 유용합니다.

## 🔍 to_owned() vs to_string()
- to_owned()는 단순 복사에 최적화되어 있어 to_string()보다 빠르고 효율적입니다.
- to_string()은 Display 트레잇을 통해 포맷팅을 거치므로 추가 연산이 발생합니다.

## 📌 요약
- to_owned()는 참조 → 소유 변환에 특화된 메서드입니다.
- Clone보다 목적이 명확하고, to_string()보다 효율적입니다.
- 실전에서는 문자열, 배열, 경로, OS 문자열 등 다양한 참조 타입을 소유로 바꿀 때 사용됩니다.

---
