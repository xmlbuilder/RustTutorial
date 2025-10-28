# JointHandle
JoinHandle은 Rust에서 스레드의 실행 결과를 받아오거나,  
스레드가 끝날 때까지 기다릴 수 있게 해주는 핸들 객체.  
Java의 Thread.join()과 유사하지만, Rust는 타입 안전성과 소유권 모델을 기반으로 훨씬 정교하게 설계돼 있음.

## 🧠 JoinHandle이란?

| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| `std::thread::spawn()`  | 새 스레드를 생성하고 `JoinHandle<T>`를 반환함                         |
| `.join()`               | 해당 스레드가 종료될 때까지 기다리고 결과를 반환함                    |
| `JoinHandle<T>`         | 스레드 핸들 객체. `T`는 스레드가 반환하는 값의 타입                   |
| `.join()의 반환 타입`   | `Result<T, Box<dyn Any + Send + 'static>>` → 성공 시 `T`, 실패 시 panic 정보 |


## 📦 기본 샘플 예제
```rust
use std::thread;

fn main() {
    let handle: thread::JoinHandle<i32> = thread::spawn(|| {
        println!("스레드 실행 중...");
        42 // 스레드가 반환할 값
    });

    // 스레드가 끝날 때까지 기다리고 결과 받기
    match handle.join() {
        Ok(result) => println!("스레드 결과: {}", result),
        Err(e) => println!("스레드 에러 발생!"),
    }
}
```

- spawn()으로 새 스레드를 만들고
- join()으로 결과를 기다림
- 스레드가 panic하면 Err로 반환됨

## 🛠 실전 예제: 병렬 계산기
```rust
use std::thread;

fn main() {
    let nums = vec![10, 20, 30, 40];
    let mut handles = vec![];

    for num in nums {
        let handle = thread::spawn(move || {
            println!("계산 중: {}", num);
            num * num
        });
        handles.push(handle);
    }

    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap_or(-1))
        .collect();

    println!("계산 결과: {:?}", results);
}
```

- 여러 스레드를 동시에 실행
- 각각의 계산 결과를 `JoinHandle` 로 받아옴
- unwrap_or(-1)로 panic 시 기본값 처리

## ✨ 요약: Rust JoinHandle 흐름

| 단계        | 설명                                                   |
|-------------|--------------------------------------------------------|
| spawn()     | 새 스레드를 생성하고 `JoinHandle<T>`를 반환함           |
| JoinHandle  | 스레드 핸들 객체. 스레드의 실행 결과를 담고 있음        |
| join()      | 스레드가 끝날 때까지 기다리고 결과를 반환함             |
| Result      | `Ok(T)` 또는 `Err(Box<dyn Any + Send + 'static>)` 형태 |

---



