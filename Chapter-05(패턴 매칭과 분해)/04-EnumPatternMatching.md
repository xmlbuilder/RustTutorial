# enum 패턴 매칭(Pattern Matching)
좋Rust의 **enum 패턴 매칭(Pattern Matching)**은 언어의 핵심 기능 중 하나로, 값의 종류에 따라 분기 처리하면서 내부 데이터를 꺼내는 기능까지 제공합니다.  
다른 언어의 switch 문보다 훨씬 강력하고 안전하죠.

## 🧠 패턴 매칭이란?
Rust의 match 문은 값의 형태에 따라 분기 처리하는 구문입니다.
특히 enum과 함께 사용할 때, 각 variant에 대해 정확하게 대응하는 코드를 작성할 수 있습니다.
match 값 {
    패턴1 => 처리1,
    패턴2 => 처리2,
    ...
}


## ✅ 특징 요약
| 특징 항목             | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| 값의 종류 구분        | `enum`은 여러 가지 중 하나의 값을 표현할 수 있음                      |
| 데이터 포함 가능      | 각 variant에 구조체나 튜플 형태의 데이터를 포함할 수 있음              |
| 패턴 매칭 지원        | `match` 문을 통해 각 variant에 대해 분기 처리 가능                     |
| 데이터 추출 가능      | `match`를 통해 variant 내부의 값을 꺼내어 사용할 수 있음               |
| Exhaustive 처리       | 가능한 모든 variant를 반드시 처리해야 하며, 누락 시 컴파일 에러 발생     |
| 타입 안전성           | 컴파일러가 타입을 체크하므로 안전한 분기 처리 가능                      |
| `_` 패턴 지원         | 나머지 모든 경우를 `_`로 처리 가능 (catch-all)                         |
| `Option`, `Result` 활용 | 표준 라이브러리에서 `enum` 기반으로 null-safe, 에러 처리 구현됨         |



## 🎨 예제 ①: 단순 enum 매칭
```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}
```

- 각 variant에 대해 정확히 대응
- RGB 구조체로 변환
- match는 모든 경우를 처리해야 하므로 Red, Green, Blue 모두 있어야 함


### 예제
```rust
enum Color {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
struct RGB(u8, u8, u8);

fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}

fn main() {
    
    let rgb1 = color_to_rgb(Color::Red);
    let rgb2 = color_to_rgb(Color::Green);
    println!("rgb1 = {:?}", rgb1); //rgb1 = RGB(255, 0, 0)
    println!("rgb1 = {:?}", rgb2); //rgb1 = RGB(0, 255, 0)
}

```

## 🧩 예제 ②: 데이터 포함 enum 매칭
```rust
enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}

fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("게임시작"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        Message::ChangePlayerName(name) => println!("Player Name {}", name),
    }
}
```

### 예제
```rust
enum Message {
    StartGame,
    WinPoint{who:String},
    ChangePlayerName(String)
}

fn handle_message(message: &Message){
    match message{
        Message::StartGame => println!("게임시작"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        Message::ChangePlayerName(name) => println!("Player Name {}", name),
    }
}

fn main() {
    
    let message1 = Message::StartGame;
    let message2 = Message::WinPoint { who: String::from("jhjeong") };
    let message3 = Message::ChangePlayerName(String::from("Message"));
    handle_message(&message1); //게임시작
    handle_message(&message2); //jhjeong의 득점
    handle_message(&message3); //jhjeong의 득점
    
}

```

### 🔍 설명
| 패턴                                | 추출되는 값 또는 변수명 |
|-------------------------------------|--------------------------|
| `Message::StartGame`                | 없음                    |
| `Message::WinPoint { who }`         | `who`                   |
| `Message::ChangePlayerName(name)`   | `name`                  |



## 🧪 예제 ③: Option 타입 매칭
fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}


- Option은 enum Option<T> { Some(T), None } 형태
- Some(i)에서 i를 꺼내 연산 가능
- None은 값이 없음을 의미

### 예제
```rust
//Option에 이용하는 방법
fn increment(x : Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}
fn main() {
    let x = Some(2);
    println!("{:?}", increment(x)); //Some(3)
    println!("{:?}", increment(None)); //None
}

```


## 🧼 예제 ④: 값 무시하기 (_)
```rust
match message {
    Message::WinPoint { who: _ } => println!("득점"),
    Message::ChangePlayerName(_) => println!("Player Name"),
}
```

- _는 값을 무시한다는 의미
- 꺼내지 않고 단순히 분기 처리만 할 때 사용

### 예제
```rust
//값이 없을 때
enum Message {
    StartGame,
    WinPoint{who:String},
    ChangePlayerName(String)
}

fn handle_message(message: &Message){
    match message{
        Message::StartGame => println!("게임시작"),
        Message::WinPoint { who : _ } => println!("득점"),
        Message::ChangePlayerName(_) => println!("Player Name"),
    }
}
fn main() {
    
    let message1 = Message::StartGame;
    let message2 = Message::WinPoint { who: String::from("jhjeong") };
    let message3 = Message::ChangePlayerName(String::from("Message"));
    handle_message(&message1); //게임시작
    handle_message(&message2); //득점
    handle_message(&message3); //Player Name
    
}
```


## 🚫 예제 ⑤: 일부만 처리하고 나머지 무시 (_ catch-all)
```rust
match message {
    Message::StartGame => println!("게임시작"),
    Message::WinPoint { who } => println!("{}의 득점", who),
    _ => println!("아직 처리 하지 못합니다"),
}
```

### 예제
```rust
enum Message {
    StartGame,
    WinPoint{who:String},
    ChangePlayerName(String)
}

fn handle_message(message: &Message){
    match message{
        Message::StartGame => println!("게임시작"),
        Message::WinPoint { who } => println!("{}의 득점", who),
       _ => println!("아직 처리 하지 못합니다"),
    }
}

fn main() {
    
    let message1 = Message::StartGame;
    let message2 = Message::WinPoint { who: String::from("jhjeong") };
    let message3 = Message::ChangePlayerName(String::from("Message"));
    handle_message(&message1); //게임시작
    handle_message(&message2); //게임시작
    handle_message(&message3); //아직 처리 하지 못합니다
}

```

- _는 모든 나머지 경우를 처리
- ChangePlayerName을 따로 처리하지 않고 _로 묶음
- match는 모든 경우를 처리해야 하므로 _가 없으면 컴파일 에러

## 🧠 패턴 매칭 정리 표
| 패턴 유형               | 예시                                      | 설명                                      |
|------------------------|-------------------------------------------|-------------------------------------------|
| 단순 variant           | `Message::StartGame`                      | 값만 비교                                 |
| 튜플 variant           | `Message::ChangePlayerName(name)`         | 값 꺼내서 변수에 바인딩                   |
| 구조체 variant         | `Message::WinPoint { who }`               | 필드 이름으로 값 꺼냄                     |
| 값 무시                | `Message::ChangePlayerName(_)`            | 값 꺼내지 않고 무시                       |
| catch-all              | `_`                                       | 나머지 모든 경우 처리                     |
| Option 매칭            | `Some(i)`, `None`                         | 값이 있을 때와 없을 때 분기               |


## ✨ 마무리 팁
- match는 패턴 매칭 + 데이터 추출 + 타입 안전성을 동시에 제공
- if let, while let, matches! 매크로 등으로 더 간결하게 표현 가능
- match는 Rust의 함수형 프로그래밍 스타일을 잘 보여주는 대표 기능

---
