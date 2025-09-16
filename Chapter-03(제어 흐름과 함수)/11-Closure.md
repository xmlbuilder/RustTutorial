# Closure
Rust의 클로저(Closure) 개념을 정리하고, 왜/어떻게 사용하는지, 그리고 주의할 점까지 정리.

## 📌 1. 클로저(Closure)란?
- 익명 함수: 이름이 없는 함수로, 변수에 저장하거나 다른 함수의 인자로 전달 가능
- 환경 캡처: 선언된 스코프의 변수를 가져와서 사용할 수 있음
- 일급 객체: 함수처럼 호출 가능하며, 값처럼 전달 가능
함수와 달리 클로저는 주변 환경의 변수들을 캡처할 수 있다는 점이 가장 큰 특징입니다.


## 📌 2. 클로저의 환경 캡처 방식
Rust의 클로저는 상황에 따라 세 가지 방식 중 하나로 변수를 캡처합니다.
| 캡처 방식   | 설명                                   | 예시 코드 |
|-------------|----------------------------------------|-----------|
| `&T`        | 불변 참조로 캡처 (읽기 전용)            | `let c = || println!("{}", x);` |
| `&mut T`    | 가변 참조로 캡처 (값 수정 가능)         | `let mut c = || x += 1;` |
| `T`         | 소유권 이동하여 캡처 (`move` 키워드 사용)| `let c = move || println!("{}", x);` |



## 📌 3. 클로저 문법
```rust
let closure_name = |param1, param2| {
    // 실행 코드
    // 마지막 표현식이 반환값
};
```

- | ... | : 파라미터 정의
- { ... } : 실행 블록 (한 줄이면 {} 생략 가능)
- 타입 어노테이션은 생략 가능 (컴파일러가 추론)
- 반환값은 마지막 표현식의 값 (세미콜론이 없을 경우)

## 📌 4. 함수 vs 클로저 문법 비교
```rust
fn add_one_v1(x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x| { x + 1 };
let add_one_v4 = |x| x + 1;
```

- add_one_v1 : 일반 함수
- add_one_v2 : 타입 명시 클로저
- add_one_v3 : 타입 추론 클로저
- add_one_v4 : 한 줄 표현식 클로저

## 📌 5. 클로저 활용 예시 — 중복 연산 제거
### 기존 코드 (중복 호출)
```rust
use std::thread;
use std::time::Duration;
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

```

### 수정된 코드 (불안정)
이 코드를 리팩토링 하여 simulated_expensive_calculation 함수를 단지 한 번만 호출 하도록 하려고 합니다.
if 문에서 불필요하게 이 함수를 여러번 호출하던 문제를 해결 합니다.  
불행하게도, 이제 모든 경우에 대해서 이 함수를 호출하고 결과를 기다리며, 이 결과를 전혀 사용하지 않는 안쪽 if 블럭도 해당됩니다.

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

###  수정된 코드 (Closure)
```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```

### 코드 설명

#### 처음 코드
```rust
if intensity < 25 {
    println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
    println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
}
```

→ simulated_expensive_calculation이 두 번 호출됨

#### 변수로 추출 (문제: 불필요한 호출 발생)

```rust
let expensive_result = simulated_expensive_calculation(intensity);
```
→ 모든 경우에 호출됨 (필요 없는 경우에도 실행)

#### 클로저로 리팩토링

```rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```
→ 필요할 때만 호출 가능, 코드 중복 제거

## 📌 6. 클로저의 타입 추론과 제약
- 클로저는 처음 호출될 때의 인자 타입으로 고정됨
```rust
let example_closure = |x| x;
let s = example_closure(String::from("hello")); // OK
let n = example_closure(5); // ❌ 타입 불일치
```

→ 한 클로저는 하나의 타입 시그니처만 가질 수 있음

## 📌 7. 클로저의 장점
- 코드 간결화: 반복되는 로직을 한 곳에 모아 재사용
- 지연 실행: 필요할 때만 연산 수행
- 환경 캡처: 외부 변수를 쉽게 참조 가능

## 📌 8. 주의할 점
- 타입 고정: 한 번 타입이 정해지면 다른 타입으로 호출 불가
- 캡처 방식에 따른 소유권 변화: move 사용 시 변수 소유권이 클로저로 이동
- 성능: 클로저는 함수 포인터보다 약간의 오버헤드가 있을 수 있음 (특히 환경 캡처 시)

## 💡 정리
Rust의 클로저는 환경을 캡처할 수 있는 익명 함수로, 반복 로직 제거, 지연 실행, 콜백 처리 등에 매우 유용합니다.
특히, Fn, FnMut, FnOnce 트레잇과 함께 사용하면 함수형 스타일의 프로그래밍이 가능해집니다.

---
