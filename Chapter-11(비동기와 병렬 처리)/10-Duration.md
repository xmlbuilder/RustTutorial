# 🧠 핵심 개념: Duration
Duration은 Rust에서 시간 간격을 나타내는 타입입니다.
초, 밀리초, 나노초 단위로 표현 가능하며, 시스템 시간과 연산하거나 대기 시간으로 사용할 수 있습니다.

```rust
use std::time::Duration;

let d = Duration::from_secs(2);        // 2초
let d2 = Duration::from_millis(500);   // 500밀리초
let total = d + d2;                    // 2.5초

```

## 🔧 실전 예제 1: 타이머 대기
```rust
use std::time::Duration;
use std::thread::sleep;

fn main() {
    println!("3초 대기 시작...");
    sleep(Duration::from_secs(3));
    println!("대기 완료!");
}
```

- sleep()은 현재 스레드를 지정된 시간만큼 멈춤
- Duration을 통해 대기 시간을 설정

## 🔧 실전 예제 2: 실행 시간 측정
```rust
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // 측정할 작업
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }

    let elapsed = start.elapsed();
    println!("작업 시간: {:.2?}", elapsed);
}
```

- Instant::now()로 시작 시간 기록
- elapsed()으로 경과 시간 측정
- Duration 타입으로 반환됨

## 🔧 실전 예제 3: 반복 작업 제어
```rust
use std::time::{Duration, Instant};

fn main() {

    let interval = Duration::from_secs(1);
    let mut next_tick = Instant::now() + interval;

    for _ in 0..5 {
        let now = Instant::now();
        if now < next_tick {
            std::thread::sleep(next_tick - now);
        }
        println!("Tick at {:?}", Instant::now());
        next_tick += interval;
    }
}
```

- 1초 간격으로 반복 작업 수행
- Duration을 이용해 정확한 간격 유지

## 🔧 실전 예제 4: 사용자 입력 기반 시간 처리
```rust
use std::time::Duration;

fn parse_duration(secs: f32) -> Result<Duration, std::time::TryFromFloatSecsError> {
    Duration::try_from_secs_f32(secs)
}

fn main() {
    match parse_duration(2.5) {
        Ok(dur) => println!("Duration: {:?}", dur),
        Err(e) => println!("변환 실패: {}", e),
    }
}
```

- 사용자 입력이 f32일 경우 안전하게 Duration으로 변환
- try_from_secs_f32를 사용해 예외 처리 가능


## ✅ 요약: Duration 실전 활용

| 항목                     | 설명                                                              |
|--------------------------|-------------------------------------------------------------------|
| sleep(Duration)          | 지정된 시간만큼 현재 스레드를 멈춤 → 타이머, 대기 처리에 사용         |
| Instant::now() + elapsed() | 작업 시작 시점 기록 후 경과 시간 측정 → 성능 분석, 시간 추적에 유용   |
| Duration                 | 시간 간격을 나타내는 타입 → 초, 밀리초, 나노초 단위로 표현 가능       |
| try_from_secs_f32        | f32 값을 안전하게 Duration으로 변환 → 사용자 입력, 외부 API 처리에 적합 |


---

## try_from_secs_f32

try_from_secs_f32는 Rust에서 부동소수점(f32) 값을 안전하게 Duration으로 변환하기 위한 함수입니다.  
아래에 관련 함수들과 함께 개념, 사용법, 주의사항까지 정리.

## 🧠 핵심 개념: try_from_secs_f32
```rust
use std::time::Duration;
let d = Duration::try_from_secs_f32(1.5)?; // 1.5초 → Duration
```

- try_from_secs_f32(secs: f32) -> Result<Duration, FromFloatSecsError>
- f32 타입의 초 단위 값을 Duration으로 변환
- 음수, NaN, 무한대, 오버플로우 같은 값은 변환 실패 → Err(FromFloatSecsError)


## 🔧 관련 함수들: Duration 변환 함수 요약
| 함수                          | 설명                                                              |
|-------------------------------|-------------------------------------------------------------------|
| Duration::try_from_secs_f32(f32) | f32 초 단위를 안전하게 Duration으로 변환 → 실패 시 Result 반환         |
| Duration::try_from_secs_f64(f64) | f64 초 단위를 안전하게 Duration으로 변환 → 더 높은 정밀도 지원          |
| Duration::from_secs_f32(f32)     | f32 초 단위를 Duration으로 변환 → 실패 시 panic 발생 가능               |
| Duration::from_secs_f64(f64)     | f64 초 단위를 Duration으로 변환 → 정밀도 높지만 예외 처리 없음          |

try_from_ 버전은 panic 없이 실패를 처리할 수 있는 안전한 방식입니다.


## ⚠️ 예외 처리 예시
```rust
match Duration::try_from_secs_f32(-1.0) {
    Ok(dur) => println!("Duration: {:?}", dur),
    Err(e) => println!("변환 실패: {}", e),
}
```

- -1.0은 음수이므로 변환 실패
- FromFloatSecsError 타입의 에러가 반환됨

## ✅ 사용 시 주의사항: Duration::try_from_secs_f32

| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| 정밀도 손실 가능성     | f32는 정밀도가 낮아 큰 값이나 소수점 처리에 주의 필요                     |
| 음수/NaN/Infinity     | 이런 값은 변환 실패 → 반드시 Result로 처리해야 안전                      |
| Duration 크기 제한     | 너무 큰 값은 Duration 내부 표현을 초과할 수 있음                         |
| Result 반환            | 실패 시 panic 대신 Err 반환 → 안전한 예외 처리 가능                       |
| try_from_ vs from_    | try_from_은 안전하지만 느릴 수 있음, from_은 빠르지만 실패 시 panic 발생   |


## 💡 결론
try_from_secs_f32는 시간을 부동소수점으로 다룰 때 안전하게 Duration으로 변환하는 도구입니다.  
실시간 시스템, 사용자 입력, 외부 API 등에서 예외가 발생할 수 있는 상황에서는  
반드시 try_from_ 계열을 사용하는 것이 바람직합니다.


