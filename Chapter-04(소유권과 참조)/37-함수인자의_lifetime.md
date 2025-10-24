# 함수 인자 스코프
Rust에서 XML 파싱 시 발생하는 수명 에러를 스코프 관점에서 부각해 정리한 문서.   
예제 코드, 설명, 핵심 개념, 해결 패턴까지 포함.

## 🧠 Rust XML 파싱에서의 스코프와 수명 문제 — quick-xml 예제 기반 정리
### 📌 문제 상황
Rust에서 quick-xml을 사용해 XML을 파싱할 때 다음과 같은 에러가 발생할 수 있다:
```
error[E0716]: temporary value dropped while borrowed
```

### 예제 코드:
```rust
let name = reader.decoder().decode(e.name().as_ref())?;
out.push(format!("ElemStart: <{}>", name));
```

이 코드는 e.name()이 반환한 임시값을 decode() 함수에 직접 넘기고, 그 결과를 다음 줄에서 사용하려고 한다.  
하지만 Rust는 함수 인자에 들어간 임시값의 수명을 그 줄까지만 보장하기 때문에,  
다음 줄에서 name을 사용하면 이미 참조 대상이 사라져 에러가 발생한다.


### 📌 비유로 쉽게 설명하면
```rust
let s = "hello";       // String
let r = &s;            // r은 참조 (&str)
let wrapper = r;       // wrapper는 r을 담은 구조체라고 생각해봐
```
- wrapper는 r을 담고 있지만, s의 데이터를 소유하지는 않아.
- s가 사라지면 wrapper도 무효가 돼.


## 🎯 핵심 원인: 함수 인자도 하나의 스코프다
Rust는 함수 인자에 들어간 임시값을 그 함수 호출이 끝나는 시점까지만 유지한다.  
즉, 다음 줄에서 그 값을 참조하려 하면 이미 소멸된 상태다.
- ✅ 이 문제의 본질은 **함수 인자도 하나의 스코프로 간주된다** 는 점이다.


## ✅ 해결 방법
### 1. 임시값을 변수에 먼저 바인딩하여 스코프를 끌어올린다
```rust
let tag = e.name(); // 임시값을 변수에 담아 스코프 연장
let name = reader.decoder().decode(tag.as_ref())?.into_owned(); // 안전하게 소유화
out.push(format!("ElemStart: <{}>", name));
```

- tag는 현재 블록 전체에서 살아 있으므로
- name이 그 안의 참조를 빌려도 문제가 없다
- into_owned()을 통해 Reader 버퍼와 분리된 String을 얻는다

### 2. 즉시 사용하고 버리는 경우는 괜찮다
```rust
if e.name().as_ref() == b"book" {
    // 비교 후 바로 버리기 → OK
}
```
- 이 경우는 임시값이 그 줄 안에서만 사용되므로 안전하다

## 🧪 속성 처리 예시
속성도 동일한 원리로 처리해야 한다:
```rust
for a in e.attributes() {
    let a = a?;
    let key = reader.decoder().decode(a.key.as_ref())?.into_owned(); // String
    let val = a.decode_and_unescape_value(&reader.decoder())?.into_owned(); // String
    out.push(format!("Attr: @{}=\"{}\"", key, val));
}
```

## 📚 요약 — Rust XML 파싱에서의 스코프와 수명
| 개념/패턴             | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| `함수 인자도 스코프다`   | 함수 인자로 넘긴 임시값은 해당 줄까지만 살아 있음                     |
| `임시값 바인딩`          | 임시값을 변수에 먼저 담으면 스코프가 블록 전체로 확장됨               |
| `into_owned()`           | Cow<str> 또는 참조 데이터를 String으로 복사해 Reader 버퍼와 수명 분리 |
| `즉시 사용 패턴`         | 비교 등 한 줄에서만 쓰면 임시값 수명 안에서 안전하게 사용 가능         |
| `Reader 버퍼 재사용`     | 다음 이벤트에서 버퍼가 덮어쓰기 되므로 참조는 무효화됨                 |
| `안전한 저장 방식`       | 로그, Vec 등에 저장하려면 반드시 소유화 필요                           |


## 안전한 인자 패턴들
### A. 함수 안에서만 쓰는 도우미(래퍼를 값으로 받기)
```rust
use quick_xml::name::Name;

fn handle_start(name: Name<'_>) {
    // name은 이 함수 본문이 끝날 때까지 유효
    if name.as_ref() == b"book" {
        // ...
    }
}
```

#### 호출부:
```rust
if let Event::Start(e) = ev {
    handle_start(e.name()); // OK: move로 넘겨 함수 스코프에서만 사용
}
```

### B. 슬라이스로 받기(역시 함수 안에서만 사용)
```rust
fn handle_start_bytes(tag: &[u8]) {
    if tag == b"book" { /* ... */ }
}

handle_start_bytes(e.name().as_ref()); // 같은 문장 내에서 빌림 → OK
```

###  C. “보관/반환”이 필요할 땐 소유화해서 반환하기
```rust
use quick_xml::name::Name;
use quick_xml::reader::Reader;

fn name_to_string(reader: &Reader<&[u8]>, name: Name<'_>) -> Result<String, quick_xml::Error> {
    // decode(...)가 Cow<str>을 주므로 into_owned()로 소유화
    Ok(reader.decoder().decode(name.as_ref())?.into_owned())
}

// 호출
let owned = name_to_string(&reader, e.name())?; // String (이벤트/스코프 넘어도 안전)
```
### D. 속성도 동일 원리
```rust
fn attr_kv_owned(
    reader: &Reader<&[u8]>,
    a: quick_xml::events::attributes::Attribute<'_>,
) -> Result<(String, String), quick_xml::Error> {
    let k = reader.decoder().decode(a.key.as_ref())?.into_owned();
    let v = a.decode_and_unescape_value(&reader.decoder())?.into_owned();
    Ok((k, v)) // 둘 다 owned
}
```

### 피해야 할 패턴(컴파일러가 보통 잡아줌)
```rust
// ❌ 래퍼에서 꺼낸 참조를 변수에 넣고 “다음 줄”에서도 쓰기
let s = e.name().as_ref();  // s는 임시 래퍼를 빌림
use_it_later(s);            // 다음 줄 → 임시 소멸로 인해 E0716 가능성
```

---
## 🔍 Name<'_>의 의미
- `Name<'_>` 는 제네릭 `lifetime`을 갖는 타입입니다. 여기서 `'_`는 익명 lifetime을 의미.
- Name<'a>처럼 명시적으로 lifetime을 지정할 수도 있지만, `_` 를 쓰면 컴파일러가 적절한 lifetime을 추론하게 됩니다.

## 📦 소유권과의 관계
- 함수 시그니처에서 name: Name<'_>는 Name 타입의 값을 소유권과 함께 받는다는 뜻.
- 즉, handle_start 함수가 호출되면 name의 소유권은 함수로 이동(move) 합니다.
- 다만, Name 타입이 내부적으로 참조를 포함하고 있다면, 그 참조의 lifetime을 `'_`로 명시.
### 예를 들어:
```rust
struct Name<'a> {
    value: &'a str,
}
```

- 이런 구조체가 있다고 할 때, Name<'_>는 value 필드가 어떤 lifetime `'a`를 갖는 `&str`이라는 걸 의미합니다.
- 이때 `'_` 는 그 lifetime을 명시적으로 쓰지 않고 컴파일러에게 위임.

## ✅ 정리하자면
- `Name<'_>` 는 참조를 포함한 구조체의 lifetime을 명시하는 것이고,
- `name: Name<'_>` 는 그 값을 소유권과 함께 함수로 이동시키는 것임.
- 즉, 소유권 이동 + lifetime 명시가 동시에 일어난다고 볼 수 있음.

---






