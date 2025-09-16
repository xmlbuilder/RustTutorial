# Trait Debug
Display가 사람이 읽기 좋은 출력이라면, Debug는 디버깅에 최적화된 출력을 위한 트레이트입니다.

## 🐞 Debug 트레이트란?
Rust에서 Debug는 구조체나 열거형의 내부 상태를 디버깅용으로 출력할 수 있게 해주는 트레이트입니다.  
{:?} 포맷을 사용하면 Debug 구현을 기반으로 출력됩니다.
```rust
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

- fmt 함수는 출력 형식을 정의
- Formatter는 출력 대상
- Result<(), Error>는 출력 성공 여부

## 📚 예제: Book 구조체에 Debug 구현
```rust
impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title, self.author, self.published
        )
    }
}
```

## 전체 소스
```rust
use std::fmt::{self, Formatter};
pub trait Debug {
    fn fmt(&self, f:&mut Formatter<'_>)->Result<(), Error>;
}

use std::fmt;
struct Book {
    title: String,
    author: String,
    published: u32
}

impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title, self.author, self.published
        )
    }
}

fn main(){
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klanbnik and Carol Nicholas".to_string(),
        published: 20230228
    };
    println!("{:?}", book);
}


```

- println!("{:?}", book)으로 출력하면 우리가 정의한 형식대로 출력됨
- 디버깅 시 구조체 내부를 빠르게 확인할 수 있음

### ✅ 왜 `Debug`를 구현할까?

| 기능 목적         | 설명                                          |
|------------------|-----------------------------------------------|
| 디버깅 지원       | 구조체 내부 상태를 빠르게 확인 가능 (`{:?}`)     |
| 자동 파생 가능     | `#[derive(Debug)]`로 간편하게 구현 가능         |
| 로깅 및 테스트 활용 | 로그 출력, 단위 테스트 등에서 유용하게 사용됨     |



### 🆚 `Display` vs `Debug` 트레이트 비교

| 트레이트 | 목적               | 출력 형식 예시                        | 포맷팅 매크로 |
|----------|--------------------|--------------------------------------|----------------|
| Display  | 사용자 친화적 출력 | `Title: Rust`                        | `{}`           |
| Debug    | 디버깅용 출력      | `Book { title: "Rust", ... }` 또는 커스텀 | `{:?}`         |



### ✨ 팁: 자동 파생하기
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}


- derive를 사용하면 직접 구현하지 않아도 자동으로 Debug 트레이트가 붙어요
- 단, 모든 필드 타입도 Debug를 구현하고 있어야 함

---

