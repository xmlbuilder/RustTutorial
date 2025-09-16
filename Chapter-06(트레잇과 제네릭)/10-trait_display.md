# Trait Display
이 트레이트는 Rust에서 사용자 정의 타입을 사람이 읽기 쉬운 형식으로 출력할 수 있도록 도와주는 핵심 기능.

## 🖨️ Display 트레이트란?
Display는 std::fmt 모듈에 정의된 트레이트로, println!, format!, to_string() 등에서 사용자 정의 타입을 보기 좋게 출력할 수 있게 해줍니다.
```rust
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

- fmt 함수는 출력 형식을 정의하는 곳
- Formatter는 출력 대상 (예: 콘솔, 문자열 버퍼 등)
- Result<(), Error>는 출력 성공 여부를 나타냄

## 📚 예제: Book 구조체에 Display 구현
```rust
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Title: {}\nAuthor: {}\nPublished {}\n",
            self.title, self.author, self.published
        )
    }
}

fn main(){
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228
    };
    println!("{}", book);
}
```

 write! 매크로를 사용해 Formatter에 문자열을 씀
- println!("{}", book)처럼 출력하면 우리가 정의한 형식대로 출력됨

### ✅ 왜 `Display`를 구현할까?

| 기능 목적         | 설명                                  |
|------------------|---------------------------------------|
| `to_string()`     | `Display`를 구현하면 `.to_string()` 자동 지원 |
| 출력 포맷팅       | `format!`, `println!`에서 `{}` 사용 가능       |
| 사용자 친화적 출력 | 사람이 읽기 쉬운 형식으로 출력 가능           |
| API 일관성        | 표준 라이브러리와 호환되는 출력 방식 제공     |


### 🆚 참고: `Display` vs `Debug` 트레이트 비교

| 트레이트 | 목적               | 출력 형식 예시 | 포맷팅 매크로 |
|----------|--------------------|----------------|----------------|
| Display  | 사용자 친화적 출력 | `Title: Rust`  | `{}`           |
| Debug    | 디버깅용 출력      | `Book { title: "Rust", ... }` | `{:?}`         |

---
