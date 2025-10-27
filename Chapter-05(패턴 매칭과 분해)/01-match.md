# match 실전 코드
Rust의 match는 단순한 조건 분기문이 아니라 패턴 매칭 기반의 강력한 제어 흐름 도구.  
특히 @ 바인딩을 활용하면 패턴을 매칭하면서 동시에 값을 변수로 사용할 수 있음.

## 🎯 기본 구조: match
```
match 값 {
    패턴1 => 실행문1,
    패턴2 => 실행문2,
    _ => 기본 실행문,
}
```

- match는 switch와 비슷하지만 훨씬 더 강력하고 안전해요.
- 모든 가능한 경우를 완전히 처리해야 하며, 그렇지 않으면 컴파일 에러가 발생합니다.
- _는 "그 외 모든 경우"를 의미하는 wildcard 패턴이에요.

## 전체 예제
```rust
//You can also use @ to give a name to the value of a match expression, and then you can use it.
fn match_number(input: i32) {
    match input {
    number @ 4 => println!("{} is an unlucky number in Korea (sounds close to 死)!", number),
    number @ 13 => println!("{} is unlucky in North America, lucky in Italy! In bocca al lupo!", number),
    _ => println!("Looks like a normal number"),
    }
}

fn main() {
    match_number(50);
    match_number(13);
    match_number(4);
}
// Looks like a normal number
// 13 is unlucky in North America, lucky in Italy! In bocca al lupo!
// 4 is an unlucky number in China (sounds close to 死)!
```

## 🔍 예제 분석
```rust
fn match_number(input: i32) {
    match input {
        number @ 4 => println!("{} is an unlucky number in Korea (sounds close to 死)!", number),
        number @ 13 => println!("{} is unlucky in North America, lucky in Italy! In bocca al lupo!", number),
        _ => println!("Looks like a normal number"),
    }
}
```


### 💡 핵심 포인트: @ 바인딩
- number @ 4는 4라는 값을 매칭하면서 동시에 number라는 변수에 바인딩합니다.
- 즉, input이 4일 때 number는 4가 되고, 그 값을 출력에 사용할 수 있어요.
number @ 4 => println!("{} is unlucky!", number)


#### ➡️ number는 4로 바인딩됨

## 🧠 실행 흐름
```rust
fn main() {
    match_number(50); // _ 패턴에 매칭됨
    match_number(13); // number @ 13에 매칭됨
    match_number(4);  // number @ 4에 매칭됨
}
```

## 🖨️ 출력 결과
```
Looks like a normal number
13 is unlucky in North America, lucky in Italy! In bocca al lupo!
4 is an unlucky number in Korea (sounds close to 死)!
```


## 🧩 @ 바인딩의 활용 예
```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("id {} is in range 3 to 7", id_variable);
        }
        Message::Hello { id } => {
            println!("id is {}", id);
        }
    }
}
```

- id_variable @ 3..=7은 id가 범위에 속할 때 그 값을 id_variable로 바인딩합니다.

## 📋 요약 표 (Markdown)
| 문법 요소     | 설명                                                                 |
|---------------|----------------------------------------------------------------------|
| `match`       | 패턴 기반 조건 분기                                                  |
| `_`           | 모든 나머지 경우를 처리하는 와일드카드 패턴                          |
| `@` 바인딩    | 패턴을 매칭하면서 해당 값을 변수로 바인딩                            |
| 안전성        | 모든 경우를 반드시 처리해야 하므로 컴파일 타임에 오류 방지 가능       |
| 표현식        | `match`는 표현식이므로 값을 반환할 수 있음                           |

Rust의 match는 단순한 조건문을 넘어서 열거형 처리, 패턴 분해, 바인딩, 조건 분기까지 모두 커버하는 강력한 도구예요.
---

# 실전 예제

Rust의 match는 enum, Option, Result와 함께 사용할 때 진가를 발휘.

## 🧭 1. enum과 match
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn navigate(dir: Direction) {
    match dir {
        Direction::North => println!("You head north into the mountains."),
        Direction::South => println!("You walk south toward the beach."),
        Direction::East => println!("You travel east into the forest."),
        Direction::West => println!("You go west into the desert."),
    }
}

fn main() {
    navigate(Direction::East);
}
```


### ✅ 설명
- Direction은 사용자 정의 열거형입니다.
- match를 통해 각 방향에 따른 행동을 분기합니다.
- 실전에서는 게임, 로봇 제어, UI 방향 처리 등에 자주 쓰여요.

## 🎁 2. Option<T>과 match
```rust
fn greet(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {n}!"),
        None => println!("Hello, stranger!"),
    }
}

fn main() {
    greet(Some("JungHwan"));
    greet(None);
}
```

### ✅ 설명
- Option은 값이 있을 수도 없을 수도 있는 타입.
- Some(value)일 때는 값을 꺼내서 사용하고, None일 때는 기본 처리.
- 실전에서는 사용자 입력, 설정 값, 검색 결과 등에 자주 등장합니다.

## ⚠️ 3. Result<T, E>과 match
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result is {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let fail = divide(5.0, 0.0);
    match fail {
        Ok(value) => println!("Result is {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

### ✅ 설명
- Result는 성공(Ok) 또는 실패(Err)를 나타내는 열거형.
- match를 통해 성공/실패를 명확하게 처리.
- 실전에서는 파일 입출력, 네트워크, 계산, DB 작업 등에서 필수적으로 사용됩니다.

## 🧠 요약 표
| 타입        | 설명                                | 대표 패턴                  |
|-------------|-------------------------------------|-----------------------------|
| `enum`      | 사용자 정의 타입, 여러 상태 표현     | `match MyEnum::Variant`     |
| `Option<T>` | 값이 있거나 없을 수 있음             | `Some(val)`, `None`         |
| `Result<T,E>`| 성공 또는 실패를 표현               | `Ok(val)`, `Err(error)`     |

---


