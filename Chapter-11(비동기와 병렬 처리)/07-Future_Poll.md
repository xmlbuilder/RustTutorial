# Future / Poll 
이제 우리가 Rust에서 비동기 프로그래밍을 이해하기 위해 꼭 알아야 할 두 개념인 Future와 Poll에 대해 깊이 있게 설명.  
이 둘은 Rust의 async 시스템의 핵심이며, Pin이 왜 필요한지도 이걸 이해해야 명확해짐.

## 🧠 1. Future란?
Rust에서 Future는 아직 완료되지 않은 비동기 작업을 나타내는 값.  
예를 들어, 네트워크 요청, 파일 읽기, 타이머 등 시간이 걸리는 작업을 Future로 표현할 수 있음.
```rust
async fn fetch_data() -> String {
    // 비동기 작업
    "Hello, JungHwan!".to_string()
}
```

이 함수는 Future<Output = String>을 반환해. 즉, 나중에 await을 통해 결과를 받을 수 있는 비동기 값.

## 🔄 2. Poll이란?
Future는 스스로 실행되지 않음.  
Rust는 poll()이라는 메서드를 통해 Future를 반복적으로 호출하면서 진행 상태를 확인함.  
이게 Rust의 비동기 모델이 pull-based라는 뜻임.

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

- Poll::Pending: 아직 완료되지 않음
- Poll::Ready(val): 완료됨, 결과 반환
- Context: 현재 task의 Waker를 포함 → 나중에 다시 깨울 수 있음

### 🧪 실전 예제: 커스텀 Future 만들기
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 공유 상태
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

// 커스텀 Future
struct TimerFuture {
    state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            Poll::Ready("타이머 완료!")
        } else {
            // 아직 완료되지 않았으면 waker 저장
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn main() {
    use futures::executor::block_on;

    let state = Arc::new(Mutex::new(SharedState {
        completed: false,
        waker: None,
    }));

    let future = TimerFuture {
        state: state.clone(),
    };

    // 타이머 스레드 시작
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        let mut state = state.lock().unwrap();
        state.completed = true;
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    });

    // Future 실행
    let result = block_on(future);
    println!("결과: {}", result);
}

```

## 🔍 이 예제에서 중요한 점

| 개념       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| Future     | 아직 완료되지 않은 비동기 작업을 표현하는 타입. `poll()`을 통해 상태를 확인함. |
| poll()     | Future의 진행 상태를 확인하는 메서드. `Pending` 또는 `Ready`를 반환함.       |
| Waker      | Future가 다시 실행될 수 있도록 깨우는 핸들. `Context`를 통해 전달됨.         |
| Context    | 현재 task의 실행 정보를 담고 있으며, `Waker`를 포함함.                        |
| block_on() | Future를 실제로 실행하는 executor. 완료될 때까지 `poll()`을 반복 호출함.     |


---

# Waker
Waker는 Rust의 비동기 시스템에서 아주 중요한 역할을 함.  
Future와 poll()을 이해했다면, Waker는 그 흐름을 다시 깨우는 알람 장치라고 보면 됨.

## 🔔 Waker란?
Rust의 비동기 시스템은 pull-based임.  
즉, Future는 스스로 실행되지 않고, executor가 poll()을 반복 호출해서 진행시켜.  
그런데 어떤 Future는 외부 이벤트가 발생해야만 진행 가능해.  
예를 들어:
- 타이머가 끝나야 함
- 네트워크 응답이 와야 함
- 파일이 읽혀야 함
이때 poll()은 Poll::Pending을 반환하고, **“나중에 다시 깨워줘”**라고 요청함.  
이걸 가능하게 하는 게 바로 Waker야.

## 🧠 Waker의 역할
- Future가 아직 준비되지 않았을 때, Waker를 Context에 저장함
- 외부 이벤트가 발생하면 Waker.wake()를 호출해서 Future를 다시 실행하도록 알림
- wake()는 executor에게 “이 Future 다시 poll()해줘!”라고 요청하는 것

## 🧪 실전 흐름 예시
```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if !ready {
        // 아직 준비 안 됨 → waker 저장
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    } else {
        Poll::Ready(result)
    }
}
```

그리고 나중에 외부 이벤트가 발생하면:
```rust
if let Some(waker) = state.waker.take() {
    waker.wake(); // Future를 다시 poll하도록 알림
}
```


## 🔍 Waker는 어떻게 만들어지나?
- 보통 executor가 Waker를 생성해서 Context에 넣어줌
- futures 크레이트나 tokio 같은 런타임이 자동으로 처리해줌
- 직접 만들 수도 있지만, unsafe와 raw pointer가 필요해서 고급 주제야

## ✅ 요약: Waker와 관련된 핵심 개념

| 개념       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| Waker      | Future가 다시 실행될 수 있도록 깨우는 핸들. 외부 이벤트 발생 시 `wake()` 호출. |
| poll()     | Future의 상태를 확인하는 메서드. `Pending`이면 `Waker`를 저장함.             |
| wake()     | 저장된 `Waker`를 통해 Future를 다시 `poll()`하도록 알림.                     |
| Context    | `poll()`에 전달되는 실행 정보. `Waker`를 포함하고 있음.                      |
| executor   | Future를 실행하고 `Waker`를 생성/관리하는 런타임. `block_on()` 등이 포함됨.   |


## Waker 구조
Rust의 비동기 시스템은 Waker가 이벤트를 감지하고, 그에 따라 Future를 다시 poll()하도록 트리거하는 구조.  
이걸 조금 더 정리해서 설명해볼게:

### 🔄 Rust 비동기 흐름 요약
- executor가 Future를 poll()함 → 상태 확인
- Future가 아직 준비되지 않으면 → Poll::Pending 반환
- Future는 Context에서 Waker를 꺼내서 저장함
- 외부 이벤트 발생 (예: 타이머, 네트워크 응답 등)
- 저장된 Waker.wake() 호출 → executor에게 “다시 poll()해줘” 요청
- executor가 다시 Future를 poll()함 → 완료되면 Poll::Ready


## 🧠 핵심 구조: Rust 비동기 시스템의 흐름
| 구성 요소   | 역할                                                                 |
|-------------|----------------------------------------------------------------------|
| Future      | 아직 완료되지 않은 비동기 작업을 표현하는 타입. `poll()`로 상태를 확인함.     |
| poll()      | Future의 진행 상태를 확인하는 메서드. `Pending`이면 `Waker`를 저장함.         |
| Waker       | Future가 다시 실행될 수 있도록 깨우는 핸들. 외부 이벤트 발생 시 `wake()` 호출. |
| wake()      | 저장된 `Waker`를 통해 executor에게 Future를 다시 `poll()`하도록 요청함.       |
| Context     | `poll()`에 전달되는 실행 정보. `Waker`를 포함하고 있음.                        |
| executor    | Future를 실행하고 `poll()`을 반복 호출함. `Waker`를 생성하고 관리함.           |


## 🔧 예를 들어보면…
```rust
// Future가 아직 준비 안 됨
Poll::Pending → Waker 저장

// 외부 이벤트 발생
thread::sleep(2초) → Waker.wake()

// executor가 다시 poll()
Poll::Ready("완료!")
```

이 구조는 스레드를 낭비하지 않고, 이벤트 기반으로 Future를 깨워서 실행하는 방식.  
그래서 Rust의 async는 고성능이면서도 안전한 비동기 모델을 제공할 수 있는 것임.

## 🔚 결론
Waker는 조건에 따라 Thread가 소비되고, 그 소비는 결국 Future를 다시 poll()하는 트리거가 됨.  
이게 Rust의 비동기 시스템이 효율적이고 안전하게 작동하는 핵심 메커니즘.


## 🧠 Rust 비동기 시스템 vs C++ 공급자/소비자 패턴 비교 요약

| Rust 개념   | 역할 또는 설명                          | C++ 대응 개념       | 대응 설명                         |
|-------------|------------------------------------------|---------------------|------------------------------------|
| Future      | 비동기 작업을 표현하는 타입              | 소비자 (Consumer)   | 작업 결과를 기다리는 주체         |
| poll()      | Future의 상태를 확인하는 메서드          | 조건 확인 루프      | 소비자가 조건을 확인하는 루프     |
| Waker       | Future를 다시 실행하도록 알림을 보내는 핸들 | notify_one()        | 조건 변수로 소비자를 깨우는 함수 |
| wake()      | Waker를 통해 executor에게 알림            | 공급자의 알림 트리거 | 조건이 만족되었음을 알림          |
| executor    | Future를 실행하고 poll을 반복 호출함      | 스케줄러 / 실행기   | 소비자를 실행하는 주체            |
| Context     | poll에 전달되는 실행 정보 (Waker 포함)     | 공유 상태 구조체    | 조건 변수와 상태를 담는 구조체    |




