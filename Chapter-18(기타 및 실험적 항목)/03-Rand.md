# Rand
Rust에서 rand 크레이트를 활용해 숫자 맞추기 게임을 만드는 예제.  
아래에 rand 사용법과 함께 dependencies 설정, 코드 설명, 그리고 확장 팁까지 정리.  

## 📦 1. rand 크레이트 설정 방법
Rust에서 외부 라이브러리를 사용하려면 Cargo.toml 파일에 dependencies를 추가해야 함.
```
# Cargo.toml
[dependencies]
rand = "0.8"
```

- "0.8"은 현재 안정적인 버전 중 하나예요. 최신 버전은 crates.io의 rand 페이지에서 확인 가능.
- 추가 후 cargo build를 실행하면 자동으로 다운로드되고 프로젝트에 포함됩니다.

## 📚 rand 크레이트 주요 기능
### 1. RNG 생성기
- thread_rng(): 스레드 로컬 RNG, 가장 흔히 쓰이는 기본 생성기
- StdRng, SmallRng: 표준/경량 RNG
- ChaChaRng: 암호학적으로 안전한 RNG (보안용)
```rust
let mut rng = rand::thread_rng();
```

### 2. 기본 난수 생성
- rng.random::<T>(): 타입에 맞는 난수 생성 (i32, f64, bool 등)
- rng.gen_range(a..b): 지정된 범위 내 난수 생성
- rng.gen_bool(p): 확률 p로 true/false 생성
```rust
let x: i32 = rng.random();          // 임의의 i32
let y: f64 = rng.gen_range(0.0..1.0); // 0~1 사이 실수
let coin = rng.gen_bool(0.5);       // 동전 던지기
```

### 3. 분포 샘플링
- Uniform::new(a, b): 균등분포
- Normal::new(mean, stddev): 정규분포
- Alphanumeric: 알파벳+숫자 랜덤 문자
```rust
use rand_distr::{Normal, Distribution};
let normal = Normal::new(0.0, 1.0).unwrap();
let sample = normal.sample(&mut rng); // 평균 0, 표준편차 1 정규분포 샘플
```

### 4. 시퀀스 관련 기능
- shuffle: 벡터 섞기
- choose: 벡터에서 임의의 원소 선택
```rust
let mut nums: Vec<i32> = (1..10).collect();
nums.shuffle(&mut rng);
let pick = nums.choose(&mut rng);
```

### 5. 기타 유틸리티
- SeedableRng: 시드 고정 RNG (재현성 있는 난수)
- fill: 배열이나 버퍼를 난수로 채우기
```rust
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
let mut rng = ChaCha8Rng::seed_from_u64(12345); // 시드 고정
```


### 📊 비교 표
| 구분           | 주요 함수/트레이트             | 설명                          |
|----------------|-------------------------------|-------------------------------|
| RNG 생성기      | thread_rng, StdRng, ChaChaRng | 난수 생성기 초기화             |
| 기본 난수       | random, gen_range, gen_bool   | 기본 타입 난수 생성            |
| 분포 샘플링     | Distribution::sample          | Uniform, Normal 등 분포 샘플링 |
| 시퀀스 처리     | shuffle, choose               | 벡터 섞기, 원소 선택           |
| 시드 고정 RNG   | SeedableRng                   | 재현성 있는 난수 생성          |



### ⚠️ 주의할 점
- 버전 호환성: rand와 rand_distr는 반드시 같은 메이저 버전에 맞춰야 합니다.  
  예: rand 0.8.5 ↔ rand_distr 0.4.
- trait import: use rand::Rng;과 use rand_distr::Distribution;을  
  반드시 추가해야 gen_range, sample 메서드가 인식됩니다.







## 🎮 2. 숫자 맞추기 게임 코드 설명
- 사용하는 함수 `rand::thread_rng().gen_range`
```rust
use std::cmp::Ordering;
use rand::prelude::*; // Rng trait 포함
use std::io;
use rand::Rng;         // Rng trait 명시적 사용

fn main() {
    println!("Guess the number!");

    // 1~100 사이의 랜덤 숫자 생성
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number); // 디버깅용 출력

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);

        let guess_number: u32 = guess.trim().parse().expect("Parse Failed");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### 🔍 주요 포인트
- `rand::thread_rng()` 는 스레드 로컬 난수 생성기를 반환
- `.gen_range(1..101)` 은 1 이상 101 미만의 난수를 생성
- `Ordering` 을 사용해 비교 결과를 처리


## 사용자 입력 받기
### 🎯 사용자 입력 기반 난수 범위 설정 예제
```rust
use rand::Rng;
use std::io;

fn main() {
    println!("Enter the minimum value:");
    let mut min_input = String::new();
    io::stdin().read_line(&mut min_input).expect("Failed to read line");
    let min: u32 = min_input.trim().parse().expect("Please enter a valid number");

    println!("Enter the maximum value:");
    let mut max_input = String::new();
    io::stdin().read_line(&mut max_input).expect("Failed to read line");
    let max: u32 = max_input.trim().parse().expect("Please enter a valid number");

    if min >= max {
        println!("Invalid range: min must be less than max.");
        return;
    }

    let secret_number = rand::thread_rng().gen_range(min..=max);
    println!("Generated number between {} and {}: {}", min, max, secret_number);
}
```

### 🔍 설명
- 사용자에게 **최솟값(min)** 과 **최댓값(max)** 을 입력받음
- gen_range(min..=max)를 사용해 포함 범위에서 난수 생성
- 입력값이 유효한지 확인 (min < max)
- 결과를 출력

## 🧪 3. rand의 주요 기능들
| 함수 이름               | 설명                            | 예시 코드                      |
|------------------------|----------------------------------|--------------------------------|
| `gen()`                | 지정된 타입의 랜덤 값 생성       | `rng.gen::<u8>()`              |
| `gen_range(start..end)`| 지정된 범위 내에서 랜덤 값 생성  | `rng.gen_range(1..10)`         |
| `sample()`             | 분포 기반 샘플링                 | `rng.sample(Alphanumeric)`     |
| `shuffle()`            | 컬렉션의 요소들을 무작위로 섞기  | `vec.shuffle(&mut rng)`        |
| `choose()`             | 컬렉션에서 랜덤 요소 선택        | `vec.choose(&mut rng)`         |

## ✅ 확장 아이디어
- 난수 생성 범위를 사용자 입력으로 바꾸기
- 시도 횟수 제한 추가
- rand_distr를 사용해 정규분포 기반 난수 생성
- Uniform 분포로 주사위 시뮬레이션 만들기

---

## rand_distr
Rust에서 rand_distr 크레이트를 사용하면 정규분포(Gaussian distribution) 기반의 난수를 쉽게 생성.  
이건 단순한 난수보다 훨씬 더 현실적인 시뮬레이션이나 통계적 모델링에 적합.

### 📦 1. rand_distr 크레이트 설정
먼저 Cargo.toml에 다음과 같이 dependencies를 추가해야 합니다:
```
[dependencies]
rand = "0.8"
rand_distr = "0.4"
```

- rand: 기본 난수 생성기
- rand_distr: 다양한 확률 분포 (정규, 포아송, 베르누이 등)를 지원하는 확장 크레이트

## 🧪 2. 정규분포 기반 난수 생성 예제
```rust
use rand::thread_rng;
use rand_distr::{Normal, Distribution};

fn main() {
    // 평균 0.0, 표준편차 1.0인 정규분포 생성
    let normal_dist = Normal::new(0.0, 1.0).unwrap();

    // 난수 생성기
    let mut rng = thread_rng();

    // 정규분포 기반 난수 샘플링
    let sample = normal_dist.sample(&mut rng);

    println!("Normally distributed number: {}", sample);
}
```

## 🔍 설명
- Normal::new(mean, std_dev)는 평균과 표준편차를 지정해 정규분포 객체를 생성
- .sample(&mut rng)는 해당 분포에서 난수를 하나 추출
- 출력되는 값은 정규분포 곡선에 따라 확률적으로 결정된 실수값

## 📊 정규분포란?
- **정규분포(Normal distribution)** 는 평균을 중심으로 대칭적인 종 모양의 분포
- 대부분의 자연현상(키, 온도, 시험 점수 등)이 이 분포를 따름
- 수학적으로는 다음과 같은 밀도 함수로 표현됨:
- $f(x) = \frac{1}{\sqrt{2\pi\sigma^2}} \cdot e^{-\frac{(x - \mu)^2}{2\sigma^2}}$
- 여기서:
- $\mu$ : 평균
- $\sigma$ : 표준편차


## ✅ 핵심 요약
| 항목             | 설명 |
|------------------|------|
| `rand_distr`     | 다양한 확률 분포를 지원하는 크레이트 (정규분포 포함) |
| `Normal::new()`  | 평균과 표준편차를 지정해 정규분포 객체 생성 |
| `.sample()`      | 해당 분포에서 난수를 추출하는 메서드 |

---

# rand 주요 함수별 샘플
## 1. gen() – 지정된 타입의 랜덤 값 생성
```rust
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let random_u8: u8 = rng.r#gen(); // 0~255 사이의 랜덤 u8
    let random_bool: bool = rng.r#gen(); // true 또는 false
    println!("Random u8: {}", random_u8);
    println!("Random bool: {}", random_bool);
}
```
### 출력 결과
```
Random u8: 17
Random bool: true
```


## 2. gen_range(start..end) – 범위 내 랜덤 값 생성
```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..10); // 1 이상 10 미만
    println!("Random number in range 1..10: {}", number);
}
```
## 출력 결과
```
Random number in range 1..10: 4
```

## 3. sample() – 분포 기반 샘플링
```rust
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let c: u8 = rng.sample(Alphanumeric);
    println!("Random alphanumeric character: {}", c);
}
```

### 출력 결과
```
Random alphanumeric character: 75
```

## 4. shuffle() – 컬렉션 무작위 섞기
```rust
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let mut items = vec![1, 2, 3, 4, 5];
    items.shuffle(&mut rng);
    println!("Shuffled items: {:?}", items);
}
```
### 출력 결과
```
Shuffled items: [4, 3, 2, 1, 5]
```

## 5. choose() – 컬렉션에서 랜덤 요소 선택
```rust
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let items = vec!["apple", "banana", "cherry"];
    if let Some(choice) = items.choose(&mut rng) {
        println!("Random choice: {}", choice);
    }
}
```

### 출력 결과
```
Random choice: apple
```

## ✅ 참고 사항
- rand::thread_rng()는 스레드 로컬 RNG를 생성합니다.
- gen()과 gen_range()는 Rng 트레이트를 통해 제공됩니다.
- shuffle()과 choose()는 SliceRandom 트레이트를 import해야 사용 가능합니다.
- sample()은 distributions 모듈에서 제공하는 분포 기반 샘플링입니다.

---

# 테스트 코드

## 🧪 랜덤 테스트 함수 요약

| 테스트 함수       | 기능 설명                                                   | 사용된 API                          |
|------------------|------------------------------------------------------------|-------------------------------------|
| `rand_gen()`     | `u8`와 `bool` 타입의 랜덤 값 생성                           | `rng.gen()`                         |
| `rand_gen_range()` | 지정된 범위 내에서 랜덤 숫자 생성 (`1..10`)               | `rng.gen_range(start..end)`        |
| `rand_sample()`  | 알파벳/숫자 중 하나를 랜덤으로 선택                         | `rng.sample(Alphanumeric)`         |
| `rand_shuffle()` | 벡터의 요소들을 랜덤하게 섞음                               | `items.shuffle(&mut rng)`          |
| `rand_change()`  | 벡터에서 하나의 요소를 랜덤하게 선택                        | `items.choose(&mut rng)`           |


##  🔍 각 함수 설명

### 1️⃣ rand_gen()
- rng.gen()을 사용해 기본 타입의 랜덤 값을 생성
- u8: 0~255 범위
- bool: true 또는 false
  
### 2️⃣ rand_gen_range()
- rng.gen_range(1..10)은 1 이상 10 미만의 정수 생성
- 범위는 start..end 형식으로 지정

### 3️⃣ rand_sample()
- Alphanumeric 타입에서 하나의 u8 값을 샘플링
- 결과는 ASCII 문자 코드로 출력됨

### 4️⃣ rand_shuffle()
- Vec<T>의 요소들을 무작위로 섞음
- shuffle()은 제자리에서 변경

### 5️⃣ rand_change()
- Vec<T>에서 하나의 요소를 무작위로 선택
- choose()는 Option<&T>를 반환
```rust
#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::distributions::Alphanumeric;
    use rand::seq::SliceRandom;
```
```rust
    #[test]
    fn rand_gen() {
        let mut rng = rand::thread_rng();
        let random_u8: u8 = rng.r#gen(); // 0~255 사이의 랜덤 u8
        let random_bool: bool = rng.r#gen(); // true 또는 false
        println!("Random u8: {}", random_u8);
        println!("Random bool: {}", random_bool);
    }
```
```rust
    #[test]
    fn rand_gen_range() {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..10); // 1 이상 10 미만
        println!("Random number in range 1..10: {}", number);
    }
```
```rust
    #[test]
    fn rand_sample() {
        let mut rng = rand::thread_rng();
        let c: u8 = rng.sample(Alphanumeric);
        println!("Random alphanumeric character: {}", c);
    }
```
```rust
    #[test]
    fn rand_shuffle() {
        let mut rng = rand::thread_rng();
        let mut items = vec![1, 2, 3, 4, 5];
        items.shuffle(&mut rng);
        println!("Shuffled items: {:?}", items);
    }
```
```rust
    #[test]
    fn rand_change() {
        let mut rng = rand::thread_rng();
        let items = vec!["apple", "banana", "cherry"];
        if let Some(choice) = items.choose(&mut rng) {
            println!("Random choice: {}", choice);
        }
    }
}
```
---
