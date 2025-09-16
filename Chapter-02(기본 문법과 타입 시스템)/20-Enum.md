# enum
Rust의 enum은 단순한 열거형을 넘어서 패턴 매칭, 데이터 포함, 타입 안정성까지 제공하는 강력한 기능입니다.

## 🧠 Rust의 enum이란?
Rust에서 enum은 여러 가지 중 하나의 값을 표현하는 타입입니다. 예를 들어:
```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

하지만 Rust의 enum은 단순한 값 나열을 넘어서 각 variant에 데이터를 포함할 수 있고, **패턴 매칭(match)**과 함께 사용하면 매우 강력한 표현력을 가집니다.

## 🔍 다른 언어와의 차이점
| 비교 항목                          | Rust                             | C/C++                           | Java                            | TypeScript                     |
|-----------------------------------|----------------------------------|---------------------------------|----------------------------------|--------------------------------|
| 기본 선언                         | `enum Language { Rust, Go }`     | `enum Language { Rust, Go }`    | `enum Language { RUST, GO }`    | `enum Language { Rust, Go }`   |
| 데이터 포함                       | ✅ `enum Message { Text(String) }` | ❌ 불가능                        | ✅ 필드와 생성자 가능             | ❌ 불가능                      |
| 패턴 매칭                         | ✅ `match message { ... }`        | ❌ `switch`만 가능               | ❌ `switch` 사용 가능             | ❌ `switch` 사용 가능           |
| 타입 안전성                       | ✅ 컴파일 시 타입 체크            | ❌ 정수 기반, 타입 안전성 낮음   | ✅ 클래스 기반으로 타입 안전함     | ❌ 런타임 오류 가능             |
| 트레이트 및 기능 확장             | ✅ `#[derive(Debug, PartialEq)]` | ❌ 불가능                        | ✅ 메서드 정의 가능               | ❌ 제한적                      |
| 구조체 형태의 variant             | ✅ `enum Msg { Win { who: String } }` | ❌ 불가능                    | ❌ 불가능                        | ❌ 불가능                      |
| null-safe 표현 (`Option`)         | ✅ `Option<T>`                    | ❌ `NULL` 사용                   | ❌ `null` 사용                    | ❌ `null` 사용                  |


Rust의 enum은 Algebraic Data Type(ADT) 개념을 구현하며, Option, Result 같은 표준 라이브러리의 핵심 타입도 모두 enum으로 구현되어 있습니다.

- #[derive(Debug)]
→ Debug 트레이트를 자동으로 구현해줘서 println!("{:?}", ...)로 출력 가능.

- #[allow(dead_code)]
→ 사용되지 않는 코드라도 컴파일 에러 없이 허용해주는 속성.


## 🧪 예제 요약
### 1. 기본 열거형과 match
```rust
enum Language { Python, Rust, JavaScript, Go }

match language {
    Language::Rust => println!("I love Rust"),
    ...
}
```
- match 문으로 각 variant에 대해 분기 처리 가능
- Debug 트레이트 덕분에 출력 가능

### 실전 예제
```rust
#[allow(dead_code)]
#[derive(Debug)]
enum Language {
    Python,
    Rust,
    JavaScript,
    Go,
}

impl Language {
    fn echo(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let language = Language::Rust;
    language.echo();
    //Rust

    match language {
        Language::Rust => println!("I love Rust"),
        Language::JavaScript => println!("I love JavaScript"),
        Language::Go => println!("I love Go"),
        Language::Python => println!("I love Python"),


}
```

### 2. 비교 가능한 enum
```rust
#[derive(Debug, PartialEq)]
enum Color { Red, Green, Blue }

println!("{}", red == green); // false
```

- PartialEq 트레이트를 추가하면 == 비교 가능

#### 실전 예제
```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

fn main() {

    let red = Color::Red;
    let green = Color::Green;
    println!("red = {:?} green = {:?}", red, green); //red = Red green = Green
}
//PartialEq를 쓰면 enum 끼리 비교가 가능하다.
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    let red = Color::Red;
    let green = Color::Green;
    println!("red == green => {}", red == green); //red == green => false
}
```


### 3. 데이터 포함하는 enum
```rust
enum Job {
    Student(Grade, String),
    Developer(String),
}
```

- 각 variant가 구조체처럼 데이터를 포함할 수 있음
- match 문으로 분해 가능


#### 실전 예제
```rust
#[derive(Debug)]
enum Grade {
    A,
    B,
    C
}

enum Job {
    Student(Grade, String),
    Developer(String)
}
fn main() {
    let indo = Job::Student(Grade::A, String::from("jeong"));

    match indo {
        Job::Student(grade, name) => {
            println!("Grade: {:?}, name: {:?}", grade, name);
        },
        Job::Developer(name) => {
            println!("Developer: {:?}", name);
        },
    }
    //Grade: A, name: "jeong"
}
```

### 4. 구조체 형태의 variant
```rust
enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}
```

- WinPoint는 필드가 있는 구조체 형태
- ChangePlayerName은 튜플 형태

#### 실전 예제
```rust
#[derive(Debug)]
enum Messsage {
    StartGame,
    WinPoint {who: String},
    ChangePlayerName(String)
}

fn main() {
    let m1 = Messsage::StartGame;
    let m2 = Messsage::WinPoint { who: String::from("Jhjeong") };
    let m3 = Messsage::ChangePlayerName(String::from("Model"));

    println!("m1 = {:?}", m1); //m1 = StartGame
    println!("m2 = {:?}", m2); //m2 = WinPoint { who: "Jhjeong" }
    println!("m3 = {:?}", m3); //m3 = ChangePlayerName("Model")
    
}


```

### 5. Option enum 활용
```rust
let some_number = Some(2);
let absent_number: Option<i32> = None;

```
- Option<T>는 Some(T) 또는 None을 표현
- 안전한 null 대체 방식
- match 또는 if let으로 처리해야 함

#### 실전 예제
```rust
//enum Option을 어떻게 할용할까?
fn main() {
    let some_number = Some(2);
    let absent_number: Option<i32> = None;
    //some_number + 1; Compile Error 
}
```

## ✅ 핵심 정리
| 개념                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| enum 기본형            | 여러 선택지 중 하나를 표현                                           |
| 데이터 포함 가능       | 각 variant에 값 또는 구조체 형태의 데이터 포함 가능                   |
| 패턴 매칭              | `match` 문으로 각 variant에 대해 분기 처리 가능                       |
| 트레이트 추가          | `#[derive(Debug, PartialEq)]` 등으로 기능 확장 가능                   |
| 비교 가능              | `PartialEq` 트레이트로 `==` 비교 가능                                |
| Option 활용            | `Some`, `None`으로 null-safe한 값 표현 가능                          |




