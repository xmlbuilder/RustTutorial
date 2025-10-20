## 🧠 핵심 개념: String의 메모리 주소 변화

### ✅ 기본 구조
- String은 내부적으로 Vec<u8>로 구현되어 있음
- Vec은 **capacity(용량)** 을 초과하면 새로운 메모리 블록을 할당하고 기존 데이터를 복사함

### ✅ push_str 동작
- push_str은 기존 String의 capacity가 충분하면 주소 그대로 유지
- capacity를 초과하면 새로운 메모리 블록으로 이동 → 주소 변경 발생

### 🔍 실전 예제: 주소 추적
```rust
fn main() {
    let mut sentence = String::new();
    println!("초기 주소: {:p}", sentence.as_ptr());

    sentence.push_str("안녕");
    println!("추가 후 주소: {:p}", sentence.as_ptr());

    sentence.push_str("하세요. 반갑습니다!");
    println!("더 추가 후 주소: {:p}", sentence.as_ptr());
}
```

### 🧪 결과 예시
```
초기 주소: 0x12345678
추가 후 주소: 0x12345678   // 주소 유지됨
더 추가 후 주소: 0x87654321 // 주소 변경됨 (capacity 초과)
```

## 💡 결론
Rust의 String은 capacity를 초과하지 않는 한 메모리 주소가 유지되지만,  
초과하면 새로운 메모리 블록으로 이동하면서 주소가 바뀝니다.


## mutable / immutable

Rust는 불변 참조(&T)와 가변 참조(&mut T)가 동시에 존재하는 걸 절대 허용하지 않음.  
이건 Rust의 가장 핵심적인 안전성 원칙 중 하나인 **“데이터 경합 방지”** 를 위한 규칙.

## 🧠 Rust의 참조 규칙 요약
| 참조 유형       | 허용 여부 | 설명                                      |
|----------------|-----------|-------------------------------------------|
| &T             | ✅ 허용    | 여러 개의 불변 참조는 동시에 가능 (읽기만 함) |
| &mut T         | ✅ 허용    | 단 하나의 가변 참조만 허용 (쓰기 가능)       |
| &T + &mut T    | ❌ 금지    | 불변과 가변 참조가 동시에 존재하면 컴파일 에러 |

### 🔥 예제: 에러 발생 상황
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;        // 불변 참조
    let r2 = &mut s;    // 가변 참조

    println!("{}, {}", r1, r2); // ❌ 컴파일 에러
}
```

### 🧨 에러 메시지
```
cannot borrow `s` as mutable because it is also borrowed as immutable
```


### ✅ 해결 방법
```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &s;
        println!("{}", r1); // r1의 생명주기 종료

        // 이후에 가변 참조 가능
        let r2 = &mut s;
        r2.push_str(" world");
        println!("{}", r2);
    }
}
```
---

## 🧠 Rust에서 StringBuilder 역할을 하는 것

### ✅ String 자체가 가변 버퍼
- Rust의 String은 내부적으로 Vec<u8> 기반으로 동작
- push_str, push, insert, extend 등 다양한 메서드로 문자열을 효율적으로 조립 가능
- capacity를 자동으로 관리하면서 성능 손실 없이 문자열을 누적
### ✅ String::with_capacity()로 미리 공간 확보
```rust
let mut builder = String::with_capacity(100);
builder.push_str("Hello");
builder.push_str(", ");
builder.push_str("Rust!");
```
- StringBuilder처럼 미리 메모리를 할당해서 재할당 최소화 가능

### 🔧 실전 예제: StringBuilder 스타일
```rust
fn main() {
    let mut sentence = String::with_capacity(64);

    for word in ["안녕", " ", "세상", "!"] {
        sentence.push_str(word);
    }

    println!("{}", sentence);
}
```

- 이 방식은 Java의 StringBuilder.append()와 거의 동일한 역할
- Rust에서는 String이 이미 가변적이고 효율적인 누적 구조를 갖고 있어서 별도 타입이 필요 없음

## 💡 결론
Rust에서는 StringBuilder가 따로 필요 없음.
String 자체가 가변 버퍼 + 효율적 누적 + 안전한 메모리 관리를 모두 갖춘 구조입니다.

---



