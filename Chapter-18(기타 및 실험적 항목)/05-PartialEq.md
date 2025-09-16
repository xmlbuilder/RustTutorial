# PartialEq, Eq, Hash
Rust에서 PartialEq, Eq, Hash 트레이트를 활용한 값 비교와 해시 기반 자료구조 사용에 대해 체계적으로 정리.

## 🧠 핵심 개념: PartialEq, Eq, Hash 트레이트

| 트레이트     | 설명 또는 특징                                      |
|--------------|-----------------------------------------------------|
| `PartialEq`  | `==`, `!=` 연산 가능. 예: `NaN != NaN`도 허용됨       |
| `Eq`         | `PartialEq`의 상위. 완전한 동등성 보장 (`x == x` 항상 참) |
| `Hash`       | 해시값 생성 가능. `HashMap`, `HashSet`의 키로 사용됨 |

### 문자열 명령을 받는 프로그램 예
```rust
use std::io;
pub enum Command {
    Help,
    Quit,
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input)
    {
        Ok(_) => match input.as_str().strip_suffix('\n') {
                Some("h") => return Ok(Command::Help),
                Some("q") => return Ok(Command::Quit),
                Some("r") => return Ok(Command::Run),
                _ => return Err(format!("Wrong input {}", input)),
        },
        Err(error) => return Err(format!("Wrong input {}", error)),
    }
}

fn main(){
    let com = user_input().expect("Wrong Input");
    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
    }
}

```
### string enum 추가
```rust
use std::io;
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r/e:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input)
    {
        Ok(_) => match input.as_str().strip_suffix('\n') {
                Some("h") => return Ok(Command::Help),
                Some("q") => return Ok(Command::Quit),
                Some("r") => return Ok(Command::Run),
                Some("e") => return Ok(Command::Execute(format!("debug"))),
                _ => return Err(format!("Wrong input {}", input)),
        },
        Err(error) => return Err(format!("Wrong input {}", error)),
    }
}

fn main(){
    let com = user_input().expect("Wrong Input");
    let p = String::new();
   
    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
        Command::Execute(path)  => println!("execute {path}")
    }
}
```
#### 에러 발생 → binary operation `==` cannot be applied to type `Command`

### 🧪 오류 분석
```rust
let p: String = String::new();
assert_ne!(p, Command::Execute(p));
```

- Command::Execute(p)는 Command 타입이고, p는 String 타입
- assert_ne!는 내부적으로 != 연산을 수행
- Rust는 서로 다른 타입 간 비교를 허용하지 않음
- 따라서 == 연산을 String과 Command 사이에 적용할 수 없어 컴파일 에러 발생
→ binary operation '==' cannot be applied to type 'Command'

## ✅ 해결 방법: Command에 PartialEq 트레이트를 구현하거나 #[derive(PartialEq, Eq)]를 선언해야 함

### 🧾 문자열 기반 명령 처리 예제
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}
```

- PartialEq, Eq를 derive하면 Command::Execute("abc".to_string()) == Command::Execute("abc".to_string()) 같은 비교가 가능해짐
- assert_eq!, match, if 조건문에서 유용하게 사용됨

### 🧭 사용자 입력 처리 흐름
```rust
fn user_input() -> Result<Command, String> {
    // 입력 받고 문자열로 변환
    // 문자열에 따라 Command enum 반환
}
```

- strip_suffix('\n')로 줄바꿈 제거
- match로 문자열 → enum 변환
- Command::Execute(String)처럼 데이터를 포함하는 variant도 처리 가능

### 🧰 HashMap에서 사용자 정의 키 사용 예제
```rust
#[derive(PartialEq, Eq, Hash)]
struct MyKey {
    x: i32,
    y: i32,
}
```

- PartialEq, Eq, Hash를 모두 구현해야 HashMap의 키로 사용 가능
- map.get(&MyKey { x: 3, y: 4 })처럼 키 조회 가능

### 전체 코드
```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct MyKey{
    x: i32,
    y: i32
}

struct MyVal {
    distance : f32
}

fn main(){
    let mut map: HashMap<MyKey, MyVal> = HashMap::new();

    map.insert(MyKey { x: 3, y: 4 }, MyVal{
        distance: 5.0,
    });

    let find = map.get(&MyKey{
        x: 3, y:4
    });

    if find.is_some() {
        println!("find {:?}", find.unwrap().distance);
    }
}

```

⚠️ 주의 사항 요약

| 상황 또는 조건             | 설명 또는 주의할 점                          |
|----------------------------|----------------------------------------------|
| 타입 비교 시               | `PartialEq` 없으면 `==`, `!=` 사용 불가       |
| 해시 기반 자료구조 사용 시 | `PartialEq`, `Eq`, `Hash` 모두 필요           |
| 서로 다른 타입 비교        | 컴파일 에러 발생. 타입이 반드시 일치해야 함   |
| `HashMap` 키로 사용        | `PartialEq`, `Eq`, `Hash` 트레이트가 필수     |

---


