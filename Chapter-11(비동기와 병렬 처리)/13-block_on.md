# block_on
이제 비동기 처리의 핵심인 block_on에 대해서 분석  
아래에 block_on의 개념과 샘플 코드의 흐름을 단계적으로 구조적으로 설명.

## 🧩 block_on이란?
block_on(future)는
비동기 함수(async fn)를 동기적으로 실행하는 함수입니다.
- 비동기 함수는 Future를 반환하지만, 직접 실행되지 않고 대기 상태에 있음
- block_on은 그 Future를 완전히 실행될 때까지 기다림
- 즉, 비동기 코드를 동기 코드처럼 실행할 수 있게 해줌

## 샘플 코드
```rust

use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("수: {i}개!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    block_on(async_main(10));
}
```
## 출력
```
수: 1개!
수: 2개!
수: 3개!
수: 4개!
수: 5개!
수: 6개!
수: 7개!
수: 8개!
수: 9개!
수: 10개!
```

## 🔍 샘플 코드 단계별 설명
```rust
use futures::executor::block_on;
```
- futures 크레이트의 executor에서 block_on을 가져옴
- 이 executor는 간단한 싱글 스레드 런타임을 제공

### 1. async fn count_to(count: i32)
```rust
async fn count_to(count: i32) {
    for i in 1..=count {
        println!("수: {i}개!");
    }
}
```

- async fn이므로 이 함수는 `Future<Output = ()>` 를 반환
- 하지만 실제로 실행되지는 않음 → await 필요

### 2. async_main(count) 정의
```rust
async fn async_main(count: i32) {
    count_to(count).await;
}
```

- async_main도 Future를 반환
- 내부에서 count_to(count)를 await함으로써
→ count_to의 Future를 실행함

### 3. main()에서 block_on
```rust
fn main() {
    block_on(async_main(10));
}
```

- async_main(10)은 Future를 반환
- block_on(...)은 그 Future를 완전히 실행될 때까지 기다림
- 결과적으로 count_to(10)가 실행되고
→ 수: 1개!부터 수: 10개!까지 출력됨

## ✅ 흐름 요약
```
main()
└── block_on(async_main(10))
     └── await count_to(10)
          └── println! 수: 1~10개!
```

→ block_on은 비동기 흐름을 동기적으로 실행하는 진입점  
→ await는 비동기 함수 내부에서 다른 Future를 실행하는 연결점  

---

이 코드에서 10개의 작업이 비동기적으로 실행되는 것처럼 보이지만,  
실제로는 비동기 함수 내부에서 순차적으로 실행되고,  
block_on은 **그 전체 Future가 끝날 때까지 메인 스레드를 “블로킹”** 합니다.

## 🔍 코드 흐름 다시 보기
```rust
async fn count_to(count: i32) {
    for i in 1..=count {
        println!("수: {i}개!");
    }
}
```

- 이 함수는 async fn이지만, 내부에 await나 tokio::sleep() 같은 비동기 작업이 없음
- 그래서 비동기적으로 실행되지는 않고, 동기적으로 반복문이 실행됨

## ✅ 핵심 정리
| 구성 요소       | 역할 설명                          |
|----------------|-------------------------------------|
| count_to()     | `async fn` → Future 반환             |
| .await         | `count_to` Future를 실행             |
| block_on(...)  | `async_main(10)` Future를 동기 실행  |


- 즉, block_on은 전체 Future가 완료될 때까지 기다리는 구조적 진입점
- 내부 작업이 진짜 비동기적이라면, await마다 스케줄링이 발생하지만 지금은 동기 반복문이 Future 안에 들어감

---
# await / block_on

await는 비동기 작업을 “시작”하게 하고,  
block_on은 그 전체 작업이 “끝날 때까지 기다리는” 역할을 합니다.

## 🧩 핵심 흐름 정리
- async fn은 Future를 반환할 뿐, 실행은 안 됨
- .await는 그 Future를 실행 시작하게 함
- 단, 내부에 진짜 비동기 작업이 있어야 스케줄링이 발생함
- block_on(future)는
- 메인 스레드를 멈추고,
- 그 Future가 완전히 끝날 때까지 기다림

## 🔍 예시로 보면
```rust
async fn do_work() {
    println!("작업 시작");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("작업 완료");
}

fn main() {
    block_on(do_work());
}
```

- do_work()는 Future를 반환
- sleep().await는 진짜 비동기 작업 → 스케줄링 발생
- block_on(...)은 메인 스레드를 멈추고, do_work()가 끝날 때까지 기다림



