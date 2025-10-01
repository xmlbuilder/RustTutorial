# 📌 Pin이 왜 필요한가?
Rust는 기본적으로 **값을 자유롭게 이동(move)**시킬 수 있음.  
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
