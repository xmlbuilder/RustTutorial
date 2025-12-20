## Async / Await
- 아래 코드는:
  - 비동기 HTTP 요청 병렬 실행
  - 랜덤 포켓몬 10마리 이름 가져오기
  - Python 코드와 동일한 동작
  - Rust async 스트림(async_stream)을 사용해 Python의 async for 느낌을 재현

## ✅ Rust 버전 전체 코드
```rust
use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use tokio::time::Instant;
use async_stream::stream;
use futures::StreamExt;

const MAX_POKEMON: u32 = 898;

#[derive(Deserialize)]
struct Pokemon {
    name: String,
}

async fn fetch_one(client: &Client, url: &str) -> String {
    let resp = client.get(url).send().await.unwrap();
    let pokemon: Pokemon = resp.json().await.unwrap();
    pokemon.name
}

fn fetch(total: usize) -> impl futures::Stream<Item = String> {
    stream! {
        let client = Client::new();

        // URL 리스트 생성
        let urls: Vec<String> = (0..total)
            .map(|_| {
                let id = rand::thread_rng().gen_range(1..=MAX_POKEMON);
                format!("https://pokeapi.co/api/v2/pokemon/{}", id)
            })
            .collect();

        // 비동기 요청 병렬 실행
        let futures = urls.iter().map(|url| fetch_one(&client, url));
        let results = futures::future::join_all(futures).await;

        // Python의 "yield"처럼 하나씩 스트림으로 반환
        for name in results {
            yield name;
        }
    }
}
```
```rust
#[tokio::main]
async fn main() {
    let start = Instant::now();

    let mut stream = fetch(10);

    while let Some(name) = stream.next().await {
        println!("{}", name);
    }

    println!("Time taken: {:.3}s", start.elapsed().as_secs_f64());
}
```


## ✅ Cargo.toml 설정
```
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
rand = "0.8"
async-stream = "0.3"
futures = "0.3"
```

## ✅ Rust 코드 설명
| Python (asyncio)           | Rust (tokio)                         |
|----------------------------|---------------------------------------|
| asyncio                    | tokio                                 |
| aiohttp.ClientSession()    | reqwest::Client                       |
| async def _fetch()         | async fn fetch_one()                  |
| async def fetch(): yield   | async_stream::stream! { yield }       |
| asyncio.gather()           | futures::future::join_all()           |
| async for name in fetch()  | while let Some(name) = stream.next().await |

---

## ✅ 전체 구조 개요
- 이 프로그램은 다음을 수행하는 비동기 Rust 애플리케이션:
  - 여러 개의 URL을 생성한다
  - HTTP 요청을 병렬로 보낸다
  - 응답 JSON에서 필요한 필드를 파싱한다
  - 결과를 비동기 스트림 형태로 하나씩 반환한다
  - main 함수에서 스트림을 소비하며 출력한다
- Rust의 async/await, 스트림, JSON 파싱, HTTP 클라이언트 사용법이 모두 들어간 예제.

## ✅ 1. 의존성 및 import
```rust
use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use tokio::time::Instant;
use async_stream::stream;
use futures::StreamExt;
```

- 각각의 역할은 다음과 같음:
  - rand::Rng
    - 난수 생성기 trait. 포켓몬 ID를 랜덤으로 뽑는 데 사용.
  - reqwest::Client
    - 비동기 HTTP 클라이언트. 요청을 병렬로 보내기 위해 사용.
  - serde::Deserialize
    - JSON 응답을 Rust 구조체로 자동 변환하기 위한 derive 매크로.
  - tokio::time::Instant
    - 시간 측정용. 실행 시간을 재기 위해 사용.
  - async_stream::stream
    - Rust에서 비동기 generator를 구현하기 위한 매크로.
    - yield를 사용할 수 있게 해줌.
  - futures::StreamExt
    - 스트림에서 .next().await 같은 메서드를 사용하기 위한 확장 trait.

## ✅ 2. 상수 및 JSON 구조 정의
```rust
const MAX_POKEMON: u32 = 898;

#[derive(Deserialize)]
struct Pokemon {
    name: String,
}
```

- MAX_POKEMON
  - 랜덤 ID 생성 범위.
- Pokemon 구조체
  - API 응답 JSON 중에서 필요한 필드만 정의.
  - serde가 자동으로 JSON → struct 변환을 해준다.

## ✅ 3. 단일 요청 처리 함수
```rust
async fn fetch_one(client: &Client, url: &str) -> String {
    let resp = client.get(url).send().await.unwrap();
    let pokemon: Pokemon = resp.json().await.unwrap();
    pokemon.name
}
```

- 이 함수는 다음을 수행함:
  - HTTP GET 요청을 보낸다
  - 응답을 JSON으로 파싱한다
  - JSON에서 name 필드를 꺼내서 반환한다
- 여기서 중요한 점:
  - client는 재사용
  - 매 요청마다 새 클라이언트를 만들지 않고 하나를 공유한다.
- async fn
  - 네트워크 I/O를 비동기로 처리한다.
- unwrap()
  - 샘플 코드라 단순하게 에러를 처리.

## ✅ 4. 여러 요청을 병렬로 처리하는 스트림 생성 함수
```rust
fn fetch(total: usize) -> impl futures::Stream<Item = String> {
    stream! {
        let client = Client::new();
```

- 이 함수는 비동기 스트림을 반환한다.
- 즉, 호출자는 .next().await로 하나씩 값을 받을 수 있다.

### ✅ 4-1. URL 리스트 생성
```rust
let urls: Vec<String> = (0..total)
    .map(|_| {
        let id = rand::thread_rng().gen_range(1..=MAX_POKEMON);
        format!("https://pokeapi.co/api/v2/pokemon/{}", id)
    })
    .collect();
```

- total 개수만큼 랜덤 ID를 생성하고
- 각 ID에 대해 URL을 만든다
- Vec<String>으로 모은다

### ✅ 4-2. 비동기 작업 목록 생성
```rust
let futures = urls.iter().map(|url| fetch_one(&client, url));
```
- 각 URL에 대해 fetch_one()을 호출
- 이 시점에서는 Future 객체만 생성
- 아직 실행된 것은 아님

### ✅ 4-3. join_all로 병렬 실행
```rust
let results = futures::future::join_all(futures).await;
```

- 모든 Future를 동시에 실행
- 모든 요청이 끝날 때까지 기다림
- 결과는 Vec<String> 형태로 반환됨

### ✅ 4-4. 스트림으로 값 내보내기
```rust
for name in results {
    yield name;
}
```

- 스트림 소비자가 .next().await 할 때마다 여기서 yield된 값이 하나씩 전달됨

###  ✅ 5. main 함수
```rust
#[tokio::main]
async fn main() {
    let start = Instant::now();

    let mut stream = fetch(10);

    while let Some(name) = stream.next().await {
        println!("{}", name);
    }

    println!("Time taken: {:.3}s", start.elapsed().as_secs_f64());
}
```

- 주요 포인트:
  - #[tokio::main]
    - tokio 런타임을 자동으로 생성하고 async main을 실행.
  - fetch(10)
    - 10개의 요청을 병렬로 처리하는 스트림을 생성.
  - while let Some(name) = stream.next().await
    - 스트림에서 값이 나올 때마다 출력.
    - 스트림이 끝나면 루프 종료.
  - 실행 시간 출력
    - 전체 병렬 요청에 걸린 시간을 측정.

## ✅ 전체 요약
- 이 Rust 코드는 다음 개념들을 모두 포함:
  - tokio 기반 비동기 런타임
  - reqwest 비동기 HTTP 클라이언트
  - serde JSON 파싱
  - async_stream을 이용한 비동기 generator
  - join_all을 이용한 병렬 Future 실행
  - StreamExt::next()로 스트림 소비
- Rust async 생태계의 핵심 요소들이 포함되어 있음.

---



