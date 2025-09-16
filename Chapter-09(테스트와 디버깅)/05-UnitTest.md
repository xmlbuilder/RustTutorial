# Unit Test

## 🧪 Unit Test란?
**단위 테스트(Unit Test)**는 프로그램의 가장 작은 단위인 함수나 메서드가
기대한 대로 동작하는지 검증하는 테스트입니다.
Rust는 테스트 기능을 언어에 내장하고 있어 매우 직관적으로 작성할 수 있습니다.

## 🧠 기본 구조
Rust의 단위 테스트는 일반적으로 같은 파일 내에 테스트 모듈을 작성합니다.
### ✅ 예시 구조 (lib.rs)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert_eq!(2 + 2, 4);
    }
}
```

### ✅ Rust 테스트 구성 요약

| 코드/명령어           | 설명                                      |
|------------------------|-------------------------------------------|
| `#[cfg(test)]`         | 테스트 시에만 컴파일되는 조건부 모듈 설정       |
| `cargo test`           | 모든 테스트를 실행하는 기본 명령어             |
| `mod tests`            | 테스트 전용 모듈 선언                         |
| `#[test]`              | 테스트 함수 어노테이션                        |
| `assert_eq!`           | 값 비교를 통해 테스트 결과 검증                |


## 🧩 다양한 테스트 예시
### 🎮 가위바위보 게임 테스트
```rust
#[derive(PartialEq)]
pub enum Cards {
    Rock,
    Scissors,
    Paper,
}

pub fn play(card1: Cards, card2: Cards) -> Option<bool> {
    if card1 == card2 {
        return None;
    }
    match (card1, card2) {
        (Cards::Rock, Cards::Scissors) => Some(true),
        (Cards::Scissors, Cards::Paper) => Some(true),
        (Cards::Paper, Cards::Rock) => Some(true),
        _ => Some(false),
    }
}
```

### ✅ 테스트 모듈
```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_win() {
        assert_eq!(play(Cards::Paper, Cards::Rock), Some(true));
    }

    #[test]
    fn test_draw() {
        assert_eq!(play(Cards::Rock, Cards::Rock), None);
    }

    #[test]
    fn test_loose() {
        assert_eq!(play(Cards::Rock, Cards::Paper), Some(false));
    }

    #[test]
    #[should_panic(expected = "stop!")]
    fn test_stop() {
        stop();
    }
}
```

- #[should_panic]: 해당 테스트는 **panic!**이 발생해야 성공
- expected = "...": panic 메시지까지 검증 가능

### 👤 구조체 테스트 예시
```rust
pub mod person {
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    impl Person {
        pub fn new(name: &str, age: u8) -> Person {
            Person {
                name: name.to_string(),
                age,
            }
        }

        pub fn hi(&self) -> String {
            format!("Hi, I'm {}, I am {} years old.", self.name, self.age)
        }

        pub fn age(&self) -> u8 {
            self.age
        }
    }
}
```

### ✅ 테스트 코드
```rust
#[cfg(test)]
mod test {
    use super::person;

    #[test]
    fn test_hi() {
        let person = person::Person::new("John", 30);
        assert_eq!(
            person.hi(),
            format!("Hi, I'm {}, I am {} years old.", person.name, person.age())
        );
    }
}
```


### 🧩 테스트 실행 옵션 정리
| 명령어                             | 설명                                      |
|------------------------------------|-------------------------------------------|
| `cargo test`                       | 모든 테스트를 병렬로 실행 (기본 설정)       |
| `cargo test -- --test-threads=1`   | 병렬성 제거, 테스트를 하나의 스레드에서 순차 실행 |
| `cargo test -- --nocapture`        | `println!` 출력 캡처 해제, 콘솔에 로그 표시     |
| `cargo test test_hi`               | 특정 테스트 함수만 실행                     |
| `cargo test -- --ignored`          | `#[ignore]`가 붙은 테스트만 실행             |



### ✅ 요약: Rust 단위 테스트 구성 및 실행
| 항목               | 설명                                                              |
|--------------------|-------------------------------------------------------------------|
| `#[cfg(test)] mod tests` | 테스트 전용 모듈 선언. 테스트 시에만 컴파일됨                         |
| `#[test]`          | 테스트 함수 어노테이션. `cargo test` 실행 시 자동으로 호출됨         |
| `assert_eq!`, `assert!`, `assert_ne!`, `should_panic` | 테스트 결과 검증을 위한 매크로들. 실패 시 테스트 실패 처리 |
| `cargo test`       | 모든 테스트를 실행하는 기본 명령어                                 |
|                    |                                                                   |



