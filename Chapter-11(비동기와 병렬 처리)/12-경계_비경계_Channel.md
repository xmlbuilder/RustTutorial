# mpsc::channel / mpsc::sync_channel

Rust의 mpsc::channel()과 mpsc::sync_channel()은 각각 **무경계(unbounded)**와 경계(bounded) 채널을 의미하며,  
실무에서는 생산자-소비자 간의 흐름 제어, 백프레셔(backpressure), 메모리 안정성에 직접적인 영향을 줍니다.  
아래에 두 샘플을 기반으로 구조적으로 정리.

## 🧩 핵심 차이: 무경계 vs 경계 채널
| 구분           | 무경계 채널 (`channel()`)         | 경계 채널 (`sync_channel(n)`)         |
|----------------|------------------------------------|----------------------------------------|
| 버퍼 크기      | 무제한                             | 고정 크기 (n)                          |
| send() 동작    | 즉시 반환 (비동기)                  | 버퍼가 가득 차면 블로킹됨 (동기)       |
| 소비자 속도    | 느려도 상관없음 (메시지 계속 쌓임) | 느리면 생산자가 멈춤 (백프레셔 발생)   |
| 메모리 사용    | 많아질 수 있음                     | 제한적, 안정적                         |
| 실무 용도      | 로그, 이벤트 스트림                | 작업 큐, 병렬 처리, 속도 제어          |


## 🔍 샘플 1: 무경계 채널 (channel())

### 실제 코드
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("메시지 {i}")).unwrap();
            println!("{thread_id:?}: 보낸 메시지 {i}");
        }
        println!("{thread_id:?}: 완료");
    });
    thread::sleep(Duration::from_millis(100));
    for msg in rx.iter() {
        println!("기본: {msg} 받음");
    }
}
```


```rust
let (tx, rx) = mpsc::channel();
```
- tx.send(...)는 즉시 반환됨
- 소비자가 느려도 메시지는 무제한으로 쌓임
- 실무에서는 로그 수집, 이벤트 브로드캐스트 등에 적합
- 단점: 메모리 폭주 가능성 있음

## 🔍 샘플 2: 경계 채널 (sync_channel(3))
### 실제 샘플
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("메시지 {i}")).unwrap();
            println!("{thread_id:?}: 보낸 메시지 {i}");
        }
        println!("{thread_id:?}: 완료");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("기본: {msg} 받음");
    }
}
```

```rust
let (tx, rx) = mpsc::sync_channel(3);
```
- 버퍼 크기 3 → 3개 이상 보내면 send()가 블로킹됨
- 소비자가 느리면 생산자가 멈추게 됨
- 실무에서는 작업 큐, 병렬 처리, 속도 제어에 적합
- 장점: 백프레셔로 흐름 제어 가능

## ✅ 실무적 관점에서 선택 기준

| 상황 또는 용도             | 추천 채널         | 이유 설명                                      |
|----------------------------|-------------------|------------------------------------------------|
| 로그, 이벤트, 알림         | `channel()`       | 빠르게 쌓고 소비는 나중에 해도 됨              |
| 병렬 작업 분배             | `sync_channel(n)` | 소비 속도에 따라 생산 속도 제어 가능           |
| 메모리 안정성 중요         | `sync_channel(n)` | 버퍼 제한으로 폭주 방지                        |
| 실시간 처리, 스트리밍      | `channel()`       | 지연 없이 빠르게 전달                          |
| 작업 큐 + 백프레셔 필요    | `sync_channel(n)` | 느린 소비자에게 압력 걸어 생산자 제어 가능     |



## 🧩 대응 구조: 채널 vs 네트워크 프로토콜
| Rust 채널 구조         | 네트워크 프로토콜 | 구조적 유사점 설명                             |
|------------------------|-------------------|------------------------------------------------|
| `mpsc::channel()`      | UDP / UTP         | 빠른 전송, 무제한 버퍼, 흐름 제어 없음         |
| `mpsc::sync_channel(n)`| TCP               | 제한된 버퍼, 흐름 제어(백프레셔), 안정적 처리  |



## 🔍 구조적 유사점 설명
### ✅ 무경계 채널 ↔ UDP/UTP
- 빠르게 보내고 잊는다 → send()는 즉시 반환
- 수신자가 느려도 계속 쌓인다 → 메모리 폭주 가능
- 흐름 제어 없음 → 생산자가 멈추지 않음
- 실무에서는 로그, 이벤트, 알림처럼
신뢰성보다 속도가 중요한 경우에 적합

### ✅ 경계 채널 ↔ TCP
- 버퍼가 가득 차면 블로킹 → send()가 멈춤
- 수신자가 느리면 생산자도 느려짐 → 백프레셔
- 흐름 제어 있음 → 안정적 처리
- 실무에서는 작업 큐, 병렬 처리, 데이터 파이프라인처럼
속도보다 신뢰성과 제어가 중요한 경우에 적합
