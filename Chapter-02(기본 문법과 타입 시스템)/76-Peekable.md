# Peekable
Rust의 `Peekable` 은 반복자(iterator)를 다룰 때 아주 유용한 도구입니다.  
특히 다음 값을 미리 확인(peek)하고 싶을 때 사용됩니다.

## 🔍 Peekable이란?
Peekable<I>는 반복자 I를 감싸서, 다음 값을 소비하지 않고 미리 확인할 수 있게 해주는 어댑터입니다.

```rust
let iter = vec![1, 2, 3].into_iter().peekable();
```

- 이렇게 만들면 `iter.peek()` 으로 `다음 값을 확인` 할 수 있고, `iter.next()` 로 `값을 소비` 할 수 있습니다.

## 📌 Peekable 주요 메서드

| 메서드       | 반환 타입           | 설명                                                   |
|--------------|---------------------|--------------------------------------------------------|
| `.peek()`    | `Option<&Item>`     | 다음 값을 소비하지 않고 미리 확인                      |
| `.next()`    | `Option<Item>`      | 다음 값을 소비하면서 반환                              |
| `.peek_mut()`| `Option<&mut Item>` | 다음 값을 가변 참조로 확인 및 수정 가능               |
| `.clone()`   | `Peekable<I>`       | Peekable을 복제 (peek 상태는 유지되지 않음)           |


## 🧠 사용 예시

```rust
use std::iter::Peekable;

fn parse_tokens<I: Iterator<Item = char>>(mut iter: Peekable<I>) {
    while let Some(&ch) = iter.peek() {
        if ch.is_whitespace() {
            iter.next(); // consume whitespace
        } else {
            println!("Next token starts with: {}", ch);
            break;
        }
    }
}
```
- `peek()` 은 &ch를 반환하므로 Some(&ch)로 패턴 매칭
- `next()` 는 실제로 값을 소비

## ⚠️ 주의할 점
- `peek()` 은 반복자의 상태를 변경하지 않습니다
- `next()` 를 호출해야 실제로 값이 소비됩니다
- `peek()` 의 반환 타입은 Option<&T>입니다.  
- 여기서 `.cloned()`을 사용하면 Option<T>가 되지만,  
- Option<T>는 nested type(Option<Option<T>>)이 아니므로 flatten()이 적용되지 않습니다.
- .clone() Peekable을 복제하며, peek된 상태도 그대로 복사됨 (내부 state도 유지됨)


## ✨ 실전 활용: 파일 라인 파싱
```rust
let file = BufReader::new(File::open("data.txt")?);
let mut lines = file.lines().peekable();

while let Some(line) = lines.next() {
    let line = line?;
    if line.starts_with("GRID*") {
        //let next_line = lines.peek().and_then(|l| l.as_ref().ok());
        let next_line = lines.peek().transpose()?;
        println!("Next line: {:?}", next_line);
    }
}
```
- peek()으로 다음 줄을 미리 확인
- next()로 실제로 소비

## ✅ 요약
Peekable은 반복자에서 다음 값을 미리 확인하고 조건에 따라 처리할 수 있게 해주는 도구입니다.  
파서, 토큰 처리, 파일 읽기 등에서 매우 유용하며, Rust의 반복자 시스템을 더 유연하게 만들어줍니다.

---

## ✅ 언제 peek().cloned()를 써야 할까?
peek()은 반복자의 다음 값을 **참조(&Item)** 로 반환합니다.
하지만 어떤 경우에는 그 값을 복제해서 소유권을 가져와야 할 때가 있습니다.

## 📌 대표적인 경우:
- 값을 복제해서 조건 분기에 사용하고 싶을 때
- 값을 복제해서 다른 함수에 넘기고 싶을 때
- 값을 복제해서 반복자 상태를 유지한 채 처리하고 싶을 때

##  🧪 예제: 수식 파서에서 다음 토큰 미리 복제

```rust
use std::iter::Peekable;

fn parse_expression<I: Iterator<Item = char>>(iter: &mut Peekable<I>) {
    while let Some(ch) = iter.peek().cloned() {
        match ch {
            '+' | '-' => {
                println!("Found operator: {}", ch);
                iter.next(); // consume
            }
            _ => {
                println!("Found operand: {}", ch);
                iter.next(); // consume
            }
        }
    }
}
```

## 🔍 여기서 왜 .cloned()가 필요할까?
- peek()은 Option<&char>을 반환
- match에서 char로 비교하려면 &char → char로 변환 필요
- cloned()는 Option<&T> → Option<T>로 바꿔줌

## 🧠 Peekable 요약

| 상황                          | `peek()` 사용 가능 | `peek().cloned()` 필요 |
|-------------------------------|---------------------|-------------------------|
| 단순히 다음 값 확인           | ✅                  | ❌                      |
| 참조(&Item)로 충분한 경우     | ✅                  | ❌                      |
| 값 복제해서 비교할 경우       | ❌                  | ✅                      |
| 값 소유권이 필요한 경우       | ❌                  | ✅                      |
| `match` / `if let`에서 값 비교| ❌                  | ✅                      |


## ✨ 실전 팁
- peek().cloned()는 반복자 상태를 유지하면서도 값을 안전하게 복제할 수 있는 방법입니다.
- 특히 match, if let, while let에서 값 비교가 필요한 경우 자주 사용됩니다.

---

