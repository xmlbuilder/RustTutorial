# Pin
이번 주제는 Rust 비동기에서의 핵심 개념인 Pin입니다.  
특히 Future가 자기참조(self-referential) 구조를 가질 수 있기 때문에  
메모리 위치가 고정되어야만 안전하게 poll할 수 있다는 원리를 이해하는 게 핵심입니다.

## ✅ 핵심 원리 요약
| 개념 요소           | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| Future 내부 구조    | async fn은 상태 머신으로 변환됨 → 내부에 지역 변수와 참조가 포함됨         |
| 이동(Move)의 위험   | Future가 이동되면 내부 참조가 무효화됨 → UB 가능성                         |
| Pin의 역할          | 메모리 위치를 고정시켜 이동을 방지함 → poll은 Pin<&mut Self>로만 가능       |
| Box::pin 사용 이유  | 힙에 Future를 고정하고 &mut로 


## 📌 Pin이 왜 필요한가?
Rust는 기본적으로 **값을 자유롭게 이동(move)** 시킬 수 있음.  
하지만 어떤 타입은 자기 자신을 참조하거나, 메모리 위치가 바뀌면 안 되는 구조를 가질 수 있음:
```rust
struct SelfRef {
    data: String,
    ptr: *const String,
}
```

이 구조는 data를 가리키는 포인터를 내부에 가지고 있음.  
그런데 이 값이 move되면 data의 메모리 주소가 바뀌고, ptr은 dangling pointer가 되어버림.  
Rust는 이런 걸 컴파일 타임에 막을 수 없기 때문에, Pin이라는 개념을 도입함.


## 🧠 Pin의 핵심 개념
- Pin<T>는 T를 고정된 메모리 위치에 "핀"으로 박아둔 것처럼 만들어서, 이동 불가능하게 만듦
- Pin<&mut T> 또는 Pin<Box<T>> 형태로 사용
- Unpin 트레잇을 구현한 타입은 이동 가능 → Pin이 필요 없음
- Unpin을 구현하지 않은 타입은 Pin을 통해 이동 금지를 보장해야 함

## 🧪 실제 사용 예: async/await
비동기 함수는 내부적으로 자기 참조 구조를 만들 수 있음. 예를 들어:
```rust
async fn example() {
    // 내부적으로 Future가 자기 자신을 참조할 수 있음
}
```


이런 경우 Rust는 Future를 Pin<Box<dyn Future>>로 감싸서 메모리 위치가 고정되도록 강제함.  
그래야 안전하게 실행할 수 있음.

## 🔍 Pin을 사용하는 코드 예시
```rust
use std::pin::Pin;
use std::future::Future;

fn poll_future(fut: Pin<&mut dyn Future<Output = ()>>) {
    // 안전하게 Future를 실행할 수 있음
}
```

- `Pin<&mut dyn Future>`는 Future가 이동되지 않도록 보장
- poll() 같은 함수는 반드시 Pin을 요구함

## 🧩 Unpin이란?
- 대부분의 타입은 Unpin을 자동으로 구현함 → 자유롭게 이동 가능
- 자기 참조 구조를 가진 타입은 Unpin을 구현하지 않음 → Pin이 필요함
- Unpin은 “이 타입은 안전하게 이동할 수 있다”는 표시야

## ✅ 요약: Rust의 `Pin`과 `Unpin`

| 개념     | 설명                                                                 |
|----------|----------------------------------------------------------------------|
| `Pin<T>` | T의 메모리 위치를 고정시켜 이동을 방지함. 자기 참조 구조나 async에서 사용됨. |
| `Unpin`  | 타입이 안전하게 이동 가능함을 나타내는 트레잇. 대부분의 타입은 자동 구현됨. |
| 사용처   | Future, Stream, 자기 참조 구조체, 비동기 안전성 확보 등                  |
| 목적     | 메모리 안전성과 동작 안정성을 컴파일 타임에 보장                         |


## 🔚 결론
Pin은 Rust의 메모리 안전성 철학을 비동기와 자기 참조 구조까지 확장하기 위한 도구. 
처음엔 생소하지만, 이걸 통해 Rust는 C/C++에서도 어려운 문제를 안전하게 해결할 수 있음.


## 실전 예제
아래는 Pin을 사용하는 실전 예제 전체 코드.  
async와 Pin이 실제로 어떻게 연결되는지 보여주는 간단한 구조로, Future를 직접 구현하고 Pin을 통해 안전하게 실행하는 예제.

### 📦 예제: Pin을 사용한 커스텀 Future 실행
```rust
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::task::Waker;

// 커스텀 Future 구조체
struct MyFuture {
    shared_state: Arc<Mutex<bool>>,
}

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();
        if *state {
            Poll::Ready("완료!")
        } else {
            println!("아직 준비 안 됨, waker 저장");
            let waker = cx.waker().clone();
            let shared = self.shared_state.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(2));
                let mut state = shared.lock().unwrap();
                *state = true;
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn main() {
    use futures::executor::block_on;

    let shared_state = Arc::new(Mutex::new(false));
    let future = MyFuture { shared_state };

    // Pin으로 Future를 고정
    let pinned_future = Box::pin(future);

    // 실행
    let result = block_on(pinned_future);
    println!("결과: {}", result);
}

```

## 🔍 핵심 포인트
- MyFuture는 Future 트레잇을 직접 구현함
- poll() 메서드는 Pin<&mut Self>를 받음 → 이동 금지 보장
- block_on()은 futures 크레이트의 실행기
- Pin<Box<T>>를 통해 Future를 안전하게 고정하고 실행

## 🧠 왜 Pin이 필요한가?
- poll()은 Future 내부 상태를 참조하거나 수정할 수 있음
- Future가 자기 자신을 참조하거나, 내부에서 비동기 흐름을 만들 경우 이동되면 위험함
- Pin은 그런 Future를 고정된 위치에서 안전하게 실행할 수 있도록 보장해줌

---

# 샘플 코드 분석

## 샘플 코드
```rust
use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};

// 작업 항목. 이 경우 지정된 시간 동안 절전 모드이고
// `respond_on` 채널의 메시지로 응답합니다.
#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

// 큐에서 작업을 수신 대기하고 실행하는 worker입니다.
async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut iterations = 0;
    loop {
        tokio::select! {
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // 작업하는 척합니다.
                work.respond_on
                    .send(work.input * 1000)
                    .expect("응답을 보내지 못했습니다.");
                iterations += 1;
            }
            // TODO: 100밀리초마다 반복 횟수를 보고합니다.
        }
    }
}

// 작업을 요청하고 작업이 완료되기를 기다리는 요청자입니다.
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue
        .send(Work { input, respond_on: tx })
        .await
        .expect("작업 큐에서 전송하지 못했습니다.");
    rx.await.expect("응답 대기 실패")
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("반복 작업 결과 {i}: {resp}");
    }
}
```

## 🔍 코드 절차적 설명
### 1. 문제 상황: sleep()을 select! 안에 직접 넣음
```rust
tokio::select! {
    _ = sleep(Duration::from_millis(100)) => println!("타임아웃"),
}
```
- sleep()은 Future를 반환
- select!는 내부적으로 move 시멘틱을 요구
- sleep()은 자기참조 구조일 수 있음 → 이동되면 불안정

### 2. 해결 시도: loop 외부에 Future를 만들고 재사용
```rust
let mut timeout_fut = sleep(Duration::from_millis(100));
loop {
    tokio::select! {
        _ = timeout_fut => println!("타임아웃"),
    }
}
```
- 컴파일러 오류 발생: timeout_fut는 이동될 수 있음
- 해결하려면 Pin으로 고정해야 함

### 3. 해결 방법: Box::pin으로 고정
```rust
let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
loop {
    tokio::select! {
        _ = &mut timeout_fut => println!("타임아웃"),
    }
}
```

- Box::pin은 Future를 힙에 고정
- &mut로 접근하면 Pin<&mut T>로 변환됨 → poll 가능
- select!는 내부적으로 poll을 호출하므로 Pin이 필수

### 4. 추가 개선: 타임아웃 후 Future 재생성
``` rust
loop {
    let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
    tokio::select! {
        _ = &mut timeout_fut => println!("타임아웃"),
        // 다른 작업 ...
    }
}
```

- sleep()은 융합된(fused) Future가 아니므로
→ 완료되면 다시 poll하면 Ready만 반환됨
- 그래서 매번 새로 생성해야 정상 동작

📊 절차도: Pin을 사용한 select! 흐름
sequenceDiagram
    participant Main
    participant TimeoutFuture
    participant Worker

    Main->>TimeoutFuture: Box::pin(sleep(100ms))
    loop 루프 반복
        Main->>TimeoutFuture: &mut timeout_fut → poll
        alt 타임아웃 발생
            TimeoutFuture-->>Main: Ready
            Main->>Main: println!("타임아웃")
        else 작업 수신
            Worker-->>Main: 응답 처리
        end
    end

---

- Rust의 Future는 자기참조 구조를 가질 수 있기 때문에, 메모리 위치가 바뀌면 내부 참조가 무효화된다.
- poll은 반드시 Pin<&mut Self>로 호출해야 하고, Box::pin을 통해 Future를 힙에 고정하고 안전하게 poll하는 구조가 필요하다.
- 결국은 ‘이동을 막아야 안전하다’는 원칙이 Rust의 async 시스템 전체에 적용된다.”



  
