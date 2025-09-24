# Iterator + Generator

Rust에서도 Python처럼 iterator와 generator 스타일을 결합한 구조를 만들 수 있음.
Rust에는 yield 키워드가 없지만, async/await와 Iterator trait,  
그리고 generator-like 구조를 통해 유사한 패턴을 구현할 수 있습니다.

## 🧪 1. Iterator + Generator 스타일 구현
```rust
struct WordSplitter {
    words: Vec<String>,
    index: usize,
}

impl WordSplitter {
    fn new(text: &str) -> Self {
        let words = text.split_whitespace().map(|s| s.to_string()).collect();
        WordSplitter { words, index: 0 }
    }
}

impl Iterator for WordSplitter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.words.len() {
            let word = self.words[self.index].clone();
            self.index += 1;
            Some(word)
        } else {
            None
        }
    }
}
```

### ✅ 사용 예
```rust
fn main() {
    let splitter = WordSplitter::new("Do today what you could to tomorrow");
    for word in splitter {
        println!("{}", word);
    }
}
```


### 출력:
```rust
Do  
today  
what  
you  
could  
to  
tomorrow
```


## 🧠 2. Generator-like 구조 with async and stream
Rust에는 아직 공식 yield 키워드는 없지만, futures crate의 Stream을 사용하면 비동기 generator처럼 동작할 수 있음.
```rust
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let words = "Do today what you could to tomorrow"
        .split_whitespace()
        .map(|s| s.to_string());

    let word_stream = stream::iter(words);

    word_stream
        .for_each(|word| async move {
            println!("{}", word);
        })
        .await;
}
```

→ stream::iter()는 generator처럼 하나씩 값을 "yield"하는 역할을 합니다.

## ✅ 요약: Rust에서 generator 스타일 구현법
| 목적                     | 방법                                  |
|--------------------------|---------------------------------------|
| 동기 반복자              | `impl Iterator` + `next()` 구현       |
| 비동기 generator-like    | `futures::stream` + `StreamExt` 사용 |
| yield-like 동작          | `stream::iter()` 또는 `async fn` 내부에서 `.await` |



## 🧠 Rust의 비동기 스타일: 정말 Python처럼 간단함
```rust
async fn move_to(target: &str) {
    println!("Moving to {}", target);
    // 실제 이동 로직
}

async fn wait(seconds: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
}

async fn play_animation(name: &str) {
    println!("Playing animation: {}", name);
}

async fn character_behavior() {
    move_to("enemy").await;
    wait(2).await;
    play_animation("attack").await;
}
```

→ 이게 Rust에서 coroutine을 쓰는 방식이에요.
딱 봐도 C++보다 훨씬 간결하고, 읽기 쉽고, 유지보수도 쉬움.

## ✅ Rust가 덜 복잡한 이유
| 특징               | 설명                                                  |
|--------------------|-------------------------------------------------------|
| `async/await` 문법 | 언어 차원에서 고수준 coroutine 지원, 문법 간결         |
| 상태 머신 자동 생성 | 컴파일러가 내부 상태 전환 코드를 자동으로 생성         |
| 안전한 메모리 모델 | borrow checker로 데이터 경합, dangling pointer 방지     |
| 풍부한 생태계       | `tokio`, `async-std`, `futures` 등으로 coroutine 활용 가능 |


## 🔍 비교 요약
| 항목           | C++20 coroutine             | Rust async/await           |
|----------------|-----------------------------|----------------------------|
| 문법 복잡도    | 높음 (직접 상태 머신 구현)   | 낮음 (자동 상태 머신 생성) |
| 사용 편의성    | Wrapper 없으면 불편          | 기본 문법만으로 충분        |
| 안전성         | 수동 관리 필요               | 컴파일러가 보장             |
| 실무 적용      | 라이브러리 의존 많음         | tokio 등으로 바로 사용 가능 |

---
