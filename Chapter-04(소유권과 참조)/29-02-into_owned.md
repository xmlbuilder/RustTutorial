# into_owned
`into_owned()` 는 Cow 타입에서 참조를 소유 데이터로 변환할 때 사용하는 메서드입니다.  
즉, 참조된 데이터를 복사해서 완전히 소유하는 형태로 바꿔줍니다.

## 🧠 into_owned()란?
Rust의  타입은 "Clone-On-Write"의 줄임말로, **참조(&T) 또는 소유된 값(T)** 을 담을 수 있는 열거형입니다.
```rust
enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

- Cow::Borrowed(&str) → 참조만 가진 상태
- Cow::Owned(String) → 소유권을 가진 상태
- into_owned()는 Cow가 참조 상태일 때 복사해서 소유권을 가진 값으로 변환합니다.

## ✅ 심플 예제
```rust
use std::borrow::Cow;

fn main() {
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    let owned: String = borrowed.into_owned(); // String으로 복사됨

    println!("{}", owned); // 출력: hello
}
```

- borrowed는 &str을 참조하고 있었지만,
- into_owned()를 통해 String으로 복사되어 완전히 소유된 값이 됨

## 🛠️ 실전 예제: XML 태그 이름 처리
quick-xml 라이브러리에서 Name<'a>는 내부적으로 Cow<'a, [u8]>를 사용합니다.  
XML 태그 이름을 처리할 때, 참조된 바이트 배열을 완전히 소유하고 싶을 때 into_owned()를 사용합니다.
```rust
use quick_xml::Reader;
use quick_xml::events::Event;

fn main() {
    let xml = r#"<greeting>Hello</greeting>"#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().into_owned(); // Cow<[u8]> → Vec<u8>
                println!("Start tag: {:?}", String::from_utf8(name));
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error: {:?}", e),
            _ => (),
        }
        buf.clear();
    }
}
```

- e.name()은 Cow<[u8]> 타입을 반환
- into_owned()를 통해 Vec<u8>로 변환하여 소유권을 확보
- 이후 `String::from_utf8()` 으로 사람이 읽을 수 있는 문자열로 변환

## 📌 요약
- into_owned()는 Cow 타입에서 참조를 소유 데이터로 바꾸는 메서드
- 복사 비용이 있지만, 참조가 사라지거나 오래 보존해야 할 때 유용
- XML, JSON, 파일 파싱 등에서 자주 사용됨

---

## 🧠 Cow의 동작 방식과 into_owned()의 처리
### Cow<'a, T>는 두 가지 상태를 가질 수 있어요:
- Borrowed: Cow::Borrowed(&T) — 참조만 가진 상태
- Owned: Cow::Owned(T) — 소유권을 가진 상태

### into_owned()를 호출하면 다음과 같이 동작합니다:

| Cow 상태              | into_owned() 결과         | 동작 방식                  |
|------------------------|----------------------------|-----------------------------|
| Cow::Owned(T)          | T                          | 복사 없이 소유권만 이동     |
| Cow::Borrowed(&T)      | T: ToOwned → 복사 발생     | 참조를 복사해서 소유권 획득 |

### 즉:
- 이미 Owned 상태라면 복사 없이 값만 넘겨줍니다.
- Borrowed 상태라면 복사해서 새로운 소유권을 가진 값을 만들어 반환합니다.

## ✅ 예제로 확인해보기
```rust
use std::borrow::Cow;

fn main() {
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    let owned: Cow<str> = Cow::Owned(String::from("world"));

    let b = borrowed.into_owned(); // 복사 발생 → String
    let o = owned.into_owned();    // 복사 없음 → String 그대로

    println!("borrowed into_owned: {}", b); // hello
    println!("owned into_owned: {}", o);    // world
}
```

- borrowed는 &str이므로 into_owned() 시 String으로 복사됨
- owned는 이미 String이므로 복사 없이 그대로 반환됨

## 🛠️ 실전에서 왜 중요할까?
- 예를 들어 XML 파싱에서 태그 이름을 처리할 때:  
```rust
use quick_xml::events::BytesStart;

fn handle_tag(tag: BytesStart) {
    let name = tag.name().into_owned(); // Cow<[u8]> → Vec<u8>
    // 여기서 name은 완전히 소유된 Vec<u8>이므로 안전하게 보존하거나 가공 가능
}
```

- XML 파서가 내부 버퍼를 재사용하기 때문에, 참조된 데이터는 곧 무효화될 수 있음.
- 그래서 into_owned()로 복사해서 안전하게 보존하는 게 중요합니다.

##  into_owned() / to_owned() / clone()
into_owned(), to_owned(), clone()은 모두 소유권을 가진 값으로 변환하거나 복사할 때 사용하는 메서드들이지만,  
각각의 목적과 동작 방식에는 중요한 차이가 있음. 

### 🧩 into_owned() vs to_owned() vs clone() 비교

| 메서드        | 입력 타입       | 동작 방식                                      | 반환 타입 |
|---------------|------------------|------------------------------------------------|------------|
| into_owned()  | Cow<'a, T>       | Owned이면 그대로 반환, Borrowed면 복사 (T: ToOwned) | T          |
| to_owned()    | &T               | 참조를 복사해서 소유권 있는 값 생성 (T: ToOwned)   | T          |
| clone()       | T                | 값을 복사해서 동일한 새 인스턴스 생성 (T: Clone)   | T          |



## 🔍 각각의 예제
### 1. into_owned() — Cow 전용
```rust
use std::borrow::Cow;

fn main() {
    let cow: Cow<str> = Cow::Borrowed("hello");
    let owned: String = cow.into_owned(); // 복사 발생
}
```

### 2. to_owned() — 참조 → 소유
```rust
fn main() {
    let s: &str = "hello";
    let owned: String = s.to_owned(); // String으로 복사
}
```

### 3. clone() — 값 복사
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 완전한 복사
}
```

## 🧠 언제 어떤 걸 써야 할까?

| 상황 대상 | 추천 메서드   | 조건 또는 설명           |
|------------|----------------|----------------------------|
| Cow        | into_owned()   | 참조면 복사, 소유면 그대로 |
| &T         | to_owned()     | T: ToOwned                 |
| T          | clone()        | T: Clone                   |


## 💡 참고: to_owned() vs clone()
- to_owned()는 Clone보다 더 구체적인 목적을 가진 메서드.
- to_owned()는 주로 참조 → 소유 변환에 쓰이고, clone()은 값 → 값 복사에 사용됩니다.
- 예: `&str.to_owned()` 는 `String` 을 반환하지만, `&str.clone()` 은 여전히 `&str` 입니다.

---
