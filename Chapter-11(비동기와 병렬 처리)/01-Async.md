# 🚀 Rust 비동기 프로그래밍 완전 정리

## 1️⃣ 비동기 모델이란?
- 동기 함수는 작업이 끝날 때까지 기다림 → 실행 흐름이 차단됨
- 비동기 함수는 작업이 끝나길 기다리는 동안 다른 작업을 수행 가능
- Rust에서는 **비동기 함수(async fn)**가 Future 객체를 반환하며, 이를 .await로 기다림

### 비동기 flow
![async_flow](/image/async_flow.png)

## 2️⃣ 협력적 멀티태스킹 (Cooperative Multitasking)
| 항목           | 설명                                       | 예시 또는 적용 언어            |
|----------------|--------------------------------------------|-------------------------------|
| 제어 주체      | 태스크 또는 코루틴이 자발적으로 제어권 반환 | Python (asyncio), Rust (Tokio) |
| 실행 방식      | 비동기 런타임이 유휴 상태일 때 태스크 실행   | `await`를 통해 실행 흐름 제어  |
| 장점           | 데이터 경합 방지, IO 작업에 최적화           | 네트워크 요청, 파일 처리 등     |
| 단점           | 모든 라이브러리가 비동기 지원해야 함         | 일부 서드파티 API는 미지원 가능 |



## 3️⃣ 비동기 함수 기본 구조
```rust
use tokio;

async fn func() {
    println!("async");
}

#[tokio::main]
async fn main() {
    func().await;
}
```

- async fn은 Future를 반환
- #[tokio::main]은 Tokio 런타임을 자동으로 시작

## 4️⃣ Rust의 비동기 실행 방식: Lazy Execution
- JavaScript/C#: Promise 기반 → 호출 즉시 실행
- Python/Rust: Lazy → .await 또는 런타임이 실행할 때까지 대기
- 단순히 .await만 사용하면 동기 실행처럼 보일 수 있음
```rust
hello().await;
bye().await; // 순차 실행
```


## 5️⃣ 병렬 실행: tokio::join!
```rust
use tokio;

async fn give_order(order: u64) -> u64 {
    println!("Processing {order}...");
    tokio::time::sleep(std::time::Duration::from_secs(3 - order)).await;
    println!("Finished {order}...");
    order
}

#[tokio::main]
async fn main() {
    let result = tokio::join!(give_order(3), give_order(2), give_order(1));
    println!("{:?}", result); // (3, 2, 1)
}
```

- join!은 여러 Future를 동시에 실행
- 각 태스크는 병렬적으로 진행되며, 결과는 튜플로 반환

## 6️⃣ 비동기 HTTP 요청: reqwest + serde_json
```
📦 Cargo.toml 설정
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
rand = "0.8.5"
reqwest = { version = "0.11.16", features = ["blocking", "json"] }
serde_json = "1.0.95"
```

###  Crate
| 크레이트       | 주요 기능 또는 트레잇         |
|----------------|-------------------------------|
| `rand`         | `Rng` 트레잇을 통한 난수 생성  |
| `reqwest`      | HTTP 요청 처리 (동기/비동기 지원) |
| `serde_json`   | JSON 직렬화 및 역직렬화       |




## 7️⃣ 동기 vs 비동기 fetch 비교
### 🔒 동기 방식 (blocking)
```rust
fn fetch(total: u32) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let mut names = vec![];

    for _ in 0..total {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", rand::thread_rng().gen_range(1..=898));
        let response = client.get(&url).send().unwrap().json::<serde_json::Value>().unwrap();
        names.push(response["name"].as_str().unwrap().to_string());
    }

    names
}
```

- 모든 요청이 순차적으로 처리됨
- 전체 시간이 요청 수 × 응답 시간

### ⚡ 비동기 방식 (async)
```rust
async fn fetch(total: u32) -> Vec<String> {
    let client = reqwest::Client::new();
    let mut names = vec![];

    for _ in 0..total {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", rand::thread_rng().gen_range(1..=898));
        let response = client.get(&url).send().await.unwrap().json::<serde_json::Value>().await.unwrap();
        names.push(response["name"].as_str().unwrap().to_string());
    }

    names
}
```

- .await를 통해 비동기적으로 요청 처리
- Tokio 런타임이 태스크를 병렬로 스케줄링

## 8️⃣ 성능 측정 예제
```rust
#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    for name in fetch(10).await {
        println!("{}", name);
    }
    println!("Time elapsed: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("{:?}", fetch(10).await);
    println!("Time elapsed: {:?}", start.elapsed());
}
```

- Instant::now()로 시작 시간 측정
- .elapsed()로 총 소요 시간 확인

## 🧠 핵심 요약
| 항목             | 설명 또는 키워드             |
|------------------|------------------------------|
| `async fn`       | 비동기 함수 정의, Future 반환 |
| `.await`         | Future를 기다려 실행          |
| `#[tokio::main]` | Tokio 런타임을 시작하는 매크로 |
| `tokio::join!`   | 여러 Future를 병렬로 실행     |
| `reqwest`        | HTTP 요청을 처리하는 클라이언트 |
| `serde_json`     | JSON 직렬화 및 역직렬화       |
| `rand`           | 난수 생성 유틸리티             |

---


# fetch 기반 확장 패턴

비동기 fetch 구조를 기반으로, 아래는 세 가지 고급 확장 패턴을 소개합니다:

## 🚀 확장 1: 비동기 스트리밍 (Async Streaming)
Rust에서는 Stream을 통해 데이터를 비동기적으로 순차 처리할 수 있습니다.  
예를 들어, API에서 여러 데이터를 받아올 때 하나씩 처리하고 싶다면 futures::stream을 활용할 수 있습니다.

### ✅ 예제: Pokémon 이름을 스트리밍으로 출력
```rust
use futures::{stream, StreamExt};
use rand::Rng;
use reqwest;
use serde_json;

const MAX_POKEMON: u32 = 898;

async fn fetch_name(id: u32) -> String {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.unwrap().json::<serde_json::Value>().await.unwrap();
    response["name"].as_str().unwrap().to_string()
}

#[tokio::main]
async fn main() {
    let ids: Vec<u32> = (0..10).map(|_| rand::thread_rng().gen_range(1..=MAX_POKEMON)).collect();

    let stream = stream::iter(ids)
        .map(|id| async move { fetch_name(id).await })
        .buffer_unordered(5); // 동시에 5개까지 처리

    stream
        .for_each(|name| async {
            println!("Got: {}", name);
        })
        .await;
}
```


- buffer_unordered(n): 최대 n개의 Future를 병렬 실행
- for_each: 스트리밍된 결과를 하나씩 처리

## 🛡️ 확장 2: 에러 핸들링 개선
비동기 코드에서 unwrap()은 위험합니다. 대신 Result를 활용해 에러를 안전하게 처리하는 방식으로 개선할 수 있습니다.
### ✅ 예제: 에러를 반환하는 fetch 함수
```rust
async fn fetch_name(id: u32) -> Result<String, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?.json::<serde_json::Value>().await?;
    Ok(response["name"].as_str().unwrap_or("unknown").to_string())
}

#[tokio::main]
async fn main() {
    match fetch_name(1).await {
        Ok(name) => println!("Fetched: {}", name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

- ? 연산자를 사용해 에러를 전파
- unwrap_or("unknown")으로 JSON 파싱 실패 대비

## ⚡ 확장 3: 병렬 fetch 최적화
tokio::task::JoinSet을 사용하면 수백 개의 비동기 작업을 병렬로 관리할 수 있습니다.
이 방식은 tokio::join!보다 유연하며, 결과를 순차적으로 수집할 수 있습니다.
### ✅ 예제: 병렬 fetch + 실행 시간 측정
```rust
use tokio::task::JoinSet;
use rand::Rng;
use reqwest;
use serde_json;
use std::time::Instant;

const MAX_POKEMON: u32 = 898;

async fn fetch_name(id: u32) -> String {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.unwrap().json::<serde_json::Value>().await.unwrap();
    response["name"].as_str().unwrap_or("unknown").to_string()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut tasks = JoinSet::new();

    for _ in 0..10 {
        let id = rand::thread_rng().gen_range(1..=MAX_POKEMON);
        tasks.spawn(fetch_name(id));
    }

    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(name) => println!("Fetched: {}", name),
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }

    println!("Total time: {:?}", start.elapsed());
}
```

- JoinSet은 태스크를 동적으로 추가/수집 가능
- join_next()로 완료된 태스크부터 처리
- start.elapsed()로 전체 실행 시간 측정

## 🧠 확장 요약
| 확장 방식             | 핵심 기능 또는 API               | 특징                          | 적합한 상황                     |
|----------------------|----------------------------------|-------------------------------|----------------------------------|
| 비동기 스트리밍       | `Stream` + `buffer_unordered`    | 순차 처리 + 병렬 실행 가능     | 대량 데이터 처리, API 반복 호출 |
| 에러 핸들링           | `Result<T, E>` + `?` 연산자      | 안전한 에러 전파 및 처리       | 네트워크 오류, JSON 파싱 실패 등 |
| 병렬 fetch 최적화     | `JoinSet` + `join_next()`        | 태스크 병렬 실행 + 순차 수집   | 대규모 병렬 요청, 성능 측정      |

---


