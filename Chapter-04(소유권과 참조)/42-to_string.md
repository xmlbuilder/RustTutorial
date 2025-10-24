# to_string
to_string()은 Rust에서 거의 모든 타입을 문자열로 변환할 수 있게 해주는 메서드입니다.  
Display 트레잇을 구현한 타입에 자동으로 제공되며, 포맷팅을 통해 사람이 읽을 수 있는 문자열을 생성합니다.

## 🧠 to_string()란?
- to_string()는  트레잇의 메서드입니다.
- 이 트레잇은 Display 트레잇을 구현한 모든 타입에 자동으로 구현됩니다.
- 내부적으로 format!("{}", self)를 호출하여 포맷팅된 문자열을 생성합니다.

## 📌 핵심 특징
- 범용성: 숫자, 구조체, 열거형 등 다양한 타입에 사용 가능
- 포맷팅 기반: Display 구현에 따라 출력 형식이 결정됨
- 비용 있음: to_owned()보다 성능이 낮을 수 있음 (불필요한 포맷팅 발생)

## ✅ 샘플 예제
```rust
fn main() {
    let num = 42;
    let s = num.to_string(); // "42"

    let boolean = true;
    let b = boolean.to_string(); // "true"

    println!("Number: {}", s);
    println!("Boolean: {}", b);
}
```

- 숫자나 불리언 같은 기본 타입도 Display를 구현하고 있어서 to_string() 사용 가능

## 🛠️ 실전 예제: 사용자 정의 타입 출력
```rust
use std::fmt;

struct Person {
    name: String,
    age: u32,
}

// Display 트레잇 구현
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let description = person.to_string(); // Display 기반 문자열 생성
    println!("{}", description); // 출력: Alice is 30 years old
}
```

- Person 구조체에 Display를 구현하면 to_string()으로 사람이 읽을 수 있는 설명을 만들 수 있음.

## 🔍 to_string() vs to_owned()

| 항목        | to_string()             | to_owned()              |
|-------------|--------------------------|--------------------------|
| 트레잇 기반 | Display                  | ToOwned (&T)            |
| 내부 동작   | format!("{}", self)      | Clone                   |
| 대상        | 값 자체 (T)              | 참조 (&T)               |
| 반환 타입   | String                   | T::Owned                |
| 성능        | 느릴 수 있음 (포맷팅 비용) | 더 빠름 (단순 복사)     |
| 사용 목적   | 사람이 읽을 문자열 생성   | 참조를 소유로 변환       |

예: &str.to_string()은 포맷팅을 거치므로 to_owned()보다 느릴 수 있음.

---




