# Cow
Rust의 Cow는 단순한 유틸리티 타입이 아니라, 성능과 유연성을 동시에 잡을 수 있는 강력한 도구.  
특히 복사 비용이 큰 데이터를 다룰 때, 참조로 처리할 수 있으면 그대로 쓰고, 복사가 필요하면 그때 소유권을 가져오는 방식으로 동작.

## 🐄 Cow: Clone-on-Write
### 📦 정의
`Cow<'a, T>`는 Borrowed(&'a T) 또는 Owned(T) 두 가지 상태를 가질 수 있는 열거형 타입.
```rust
enum Cow<'a, T>
where
    T: ToOwned + ?Sized,
{
    Borrowed(&'a T),
    Owned(<T as ToOwned>::Owned),
}
```

- Borrowed: 참조만 함 → 복사 없음
- Owned: 복사해서 소유함 → 필요할 때만 복사
- 'a: 참조의 라이프타임
- T: 참조할 수 있는 타입 (str, Path, [T], 등)

### 🔍 언제 쓰는가?
- 읽기 전용으로 처리하다가, 특정 조건에서만 수정이 필요할 때
- API에서 참조와 소유를 동시에 허용하고 싶을 때
- 복사 비용이 큰 데이터를 조건부로 복사하고 싶을 때

### 🧪 실사용 예제 1: 문자열 조건부 복사
```rust
use std::borrow::Cow;

fn normalize_path<'a>(path: &'a str) -> Cow<'a, str> {
    if path.starts_with("~/") {
        Cow::Owned(path.replacen("~/", "/home/junghwan", 1))
    } else {
        Cow::Borrowed(path)
    }
}

fn main() {
    let raw = "~/projects/rust";
    let normalized = normalize_path(raw);
    println!("경로: {}", normalized);
}
```

- ~/로 시작하면 복사해서 경로를 바꿈
- 아니면 그대로 참조
- 복사 비용은 조건부로 발생 → 효율적!

### 🧪 실사용 예제 2: API에서 유연한 인자 처리
```rust
use std::borrow::Cow;

fn greet(name: Cow<str>) {
    println!("안녕하세요, {}님!", name);
}

fn main() {
    let borrowed = "JungHwan";
    greet(Cow::Borrowed(borrowed)); // 참조 전달

    let owned = String::from("Rustacean");
    greet(Cow::Owned(owned)); // 소유된 값 전달
}
```


- `Cow<str>`를 인자로 받으면 &str과 String 모두 처리 가능
- API가 더 유연해지고, 호출자가 복사 여부를 결정할 수 있음

## 🧠 장점 요약: Rust의 `Cow` (Clone-on-Write)

| 특징             | 설명                                       |
|------------------|--------------------------------------------|
| 성능 최적화      | 필요할 때만 복사 → 메모리 낭비 방지         |
| 유연한 API 설계  | 참조와 소유 모두 허용 (`&T` 또는 `T`)       |
| 표현식 친화적    | `if`와 조합해 삼항 연산자처럼 깔끔하게 사용 |
| 안전성           | 타입 시스템과 함께 안전하게 동작            |


## 🧩 어떤 타입에 쓸 수 있나?
- Cow<str> → &str vs String
- Cow<[T]> → &[T] vs Vec<T>
- Cow<Path> → &Path vs PathBuf

Rust의 Cow는 단순한 편의 타입이 아니라, 성능과 안전성을 동시에 고려한 고급 설계 도구.  
특히 라이브러리나 API를 만들 때, 호출자에게 선택권을 주면서도 내부에서 효율적으로 처리할 수 있는 구조를 만들 수 있음.
---

