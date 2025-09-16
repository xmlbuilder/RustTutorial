# Math

## 🧮 기본 수학 함수 (표준 라이브러리)

Rust는 f64 타입에 다양한 메서드를 제공합니다:

```rust
let x = 2.0_f64;

let sqrt = x.sqrt();       // 제곱근
let pow = x.powf(3.0);     // 거듭제곱
let sin = x.sin();         // 사인
let cos = x.cos();         // 코사인
let ln = x.ln();           // 자연로그
let exp = x.exp();         // 지수 함수
```

대부분의 수학 함수는 f64 또는 f32 타입의 메서드로 제공돼요.


### ✅ 사용 예시
```rust
let y = 1.0;
let x = 1.0;

let angle = y.atan2(x); // 결과: π/4 ≈ 0.785
```

- atan2(y, x)는 (x, y) 좌표의 각도를 계산할 때 사용돼요
- 반환값은 라디안 단위이며, 범위는 -π부터 π까지

### ⚠️ 자주 하는 실수
- std::f64::atan2(y, x)처럼 함수처럼 호출하려고 하면 에러가 납니다
- atan2는 메서드이기 때문에 반드시 y.atan2(x)처럼 써야 해요
- 정수 타입(i32, u32)에서는 사용할 수 없고, as f64로 형 변환해야 합니다:
```rust
let y = 1;
let x = 2;
let angle = (y as f64).atan2(x as f64);
```



## 📐 수학 상수
```rust
use std::f64::consts;

let pi = consts::PI;
let e = consts::E;
```

std::f64::consts에는 PI, E, TAU, LN_2, SQRT_2 등 다양한 상수가 있어요.


## 📦 외부 크레이트 활용
### 1. libm (C 스타일 수학 함수)
```
# Cargo.toml
libm = "0.2"
```

```rust
use libm::atan2;

let angle = atan2(1.0, 1.0); // y, x
```

no_std 환경에서도 사용 가능해서 임베디드 개발에 적합해요.


### 2. num 크레이트 (정수/복소수/범용 수학)
```
num = "0.4"
```

```
use num::integer::gcd;

let result = gcd(12, 18); // 최대공약수
```
num은 정수 연산, 복소수, 범용 수학 타입을 다룰 때 유용해요.


## ✨ 고급: 클로저 기반 수학 처리
Rust는 클로저를 활용해 수학 함수를 동적으로 다룰 수도 있어요:
```rust
let square = |x: f64| x * x;
println!("{}", square(3.0)); // 9.0
```


## 🔧 기본 원칙: 타입이 다르면 연산 불가
예를 들어 i32와 f64를 더하려고 하면 컴파일 에러가 납니다:
```rust
let a: i32 = 10;
let b: f64 = 2.5;

// let result = a + b; // ❌ 에러: 서로 다른 타입
```


## ✅ 형 변환 방법
### 1. as 키워드로 명시적 캐스팅
```rust
let a: i32 = 10;
let b: f64 = 2.5;

let result = (a as f64) + b; // ✅ OK
```

as는 기본적인 형 변환 방법이에요.
단, 정밀도 손실이나 오버플로우에 주의해야 해요.


### 2. TryFrom / Into 트레이트 (안전한 변환)
```rust
use std::convert::TryFrom;

let a: i32 = 256;
let b = u8::try_from(a); // Result<u8, _>
```

이 방식은 런타임 체크가 들어가서 안전하게 변환할 수 있어요.
특히 u8, i8, usize 같은 범위 제한 타입에 유용해요.


## 🧮 수학 함수와 형 변환
Rust의 수학 함수는 대부분 f64 또는 f32 타입을 요구하므로,
정수 타입을 사용할 경우 형 변환이 필수입니다:
```rust
let x: i32 = 3;
let y = (x as f64).sqrt(); // OK: sqrt는 f64 메서드
```


## 📦 예시: 다양한 타입 연산
```rust
let a: u32 = 5;
let b: i64 = -3;
let c: f64 = 2.5;

// 형 변환 후 연산
let result = (a as f64) + (b as f64) + c;
```


### ⚠️ 주의사항
- as는 단순 캐스팅이므로 오버플로우나 손실을 감지하지 않음
- TryFrom은 안전하지만 번거로움이 있음
- f64 → i32로 변환할 때는 소수점이 잘림 (floor나 round를 먼저 써야 할 수도 있음)


## 🧮 i32에서 사용 가능한 주요 메서드 목록
| 메서드 이름             | 예시 및 설명                                                  |
|---------------------------|---------------------------------------------------------------|
| `abs()`                   | `-5.abs()` → `5`                                              |
| `signum()`                | `-5.signum()` → `-1`, `0.signum()` → `0`, `5.signum()` → `1` |
| `checked_add(x)`          | `i32::MAX.checked_add(1)` → `None`, `5.checked_add(3)` → `Some(8)` |
| `checked_sub(x)`          | `5.checked_sub(10)` → `None`, `10.checked_sub(5)` → `Some(5)` |
| `checked_mul(x)`          | `i32::MAX.checked_mul(2)` → `None`, `3.checked_mul(4)` → `Some(12)` |
| `checked_div(x)`          | `1.checked_div(0)` → `None`, `10.checked_div(2)` → `Some(5)` |
| `wrapping_add(x)`         | `i32::MAX.wrapping_add(1)` → `i32::MIN`                       |
| `wrapping_sub(x)`         | `i32::MIN.wrapping_sub(1)` → `i32::MAX`                       |
| `wrapping_mul(x)`         | `i32::MAX.wrapping_mul(2)` → 값 래핑됨                        |
| `overflowing_add(x)`      | `i32::MAX.overflowing_add(1)` → `(i32::MIN, true)`            |
| `saturating_add(x)`       | `i32::MAX.saturating_add(1)` → `i32::MAX`                     |
| `count_ones()`            | `0b1011.count_ones()` → `3`                                   |
| `count_zeros()`           | `0b1011.count_zeros()` → `29`                                 |
| `leading_zeros()`         | `0b0000_0001.leading_zeros()` → `31`                          |
| `trailing_zeros()`        | `0b1000.trailing_zeros()` → `3`                               |
| `rotate_left(n)`          | `0b0001.rotate_left(1)` → `0b0010`                            |
| `rotate_right(n)`         | `0b0001.rotate_right(1)` → `0b1000_0000_0000_0000_0000_0000_0000_0000` |
| `swap_bytes()`            | `0x12345678.swap_bytes()` → `0x78563412`                      |
| `reverse_bits()`          | `0b0001.reverse_bits()` → `0b1000...`                         |
| `is_positive()`           | `5.is_positive()` → `true`                                    |
| `is_negative()`           | `-3.is_negative()` → `true`                                   |
| `ilog2()` / `ilog10()`    | `1024.ilog2()` → `10`, `1000.ilog10()` → `3`                  |



## 🔧 예시 코드
```rust
let x: i32 = -42;

println!("abs: {}", x.abs());
println!("signum: {}", x.signum());
println!("count_ones: {}", x.count_ones());
println!("reverse_bits: {:032b}", x.reverse_bits());
println!("checked_add: {:?}", x.checked_add(100000));
```


### ⚠️ 주의사항
- i32는 **부동소수점 함수(sqrt, sin, exp 등)**를 직접 지원하지 않음
→ 이런 경우엔 x as f64로 형 변환 후 사용해야 해요
- checked_*, wrapping_*, saturating_*은 오버플로우 안전성을 다르게 처리하므로 상황에 맞게 선택해야 해요


## 사용 샘플
Rust에서 i32 배열의 평균과 표준편차를 계산하려면, 먼저 i32 값을 f64로 변환한 뒤 수학 연산을 수행. 
Rust는 타입이 엄격해서 정수와 부동소수점 연산을 섞을 수 없기 때문에 형 변환이 핵심입니다.

### 🧮 평균 계산
```rust
fn mean(data: &[i32]) -> f64 {
    let sum: i32 = data.iter().sum();
    sum as f64 / data.len() as f64
}
```

- iter().sum()으로 총합 계산
- as f64로 형 변환 후 평균 계산

### 📊 표준편차 계산
```rust
fn std_dev(data: &[i32]) -> f64 {
    let mean = mean(data);
    let variance = data.iter()
        .map(|x| {
            let diff = *x as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / (data.len() as f64);

    variance.sqrt()
}
```

- 각 값에서 평균을 뺀 뒤 제곱
- 제곱값들의 평균 → 분산
- sqrt()로 표준편차 계산

### ✅ 전체 예시
```rust
fn main() {
    let data = [1, 2, 3, 4, 5];

    let avg = mean(&data);
    let std = std_dev(&data);

    println!("평균: {:.2}", avg);
    println!("표준편차: {:.2}", std);
}
```

출력:
```
평균: 3.00
표준편차: 1.41
```


###  ⚠️ 주의사항
- f64로 변환하지 않으면 sqrt, powf 같은 함수 사용 불가
- 표준편차 계산 시 **자유도(n-1)**를 고려하려면 분모를 data.len() - 1로 바꿔야 해요

## ✅ sum()
Rust의 Iterator 트레이트는 sum() 메서드를 제공합니다.
f64 타입의 값들을 담은 Vec<f64>나 슬라이스에서 사용할 수 있어요:
```rust
let data = vec![1.0, 2.0, 3.0];
let total: f64 = data.iter().sum();
println!("합계: {}", total); // 6.0
```

iter()는 &f64를 반환하므로 타입 추론이 안 될 경우 sum::<f64>()처럼 명시해줘야 할 수도 있어요.


## 🧮 f64 수학 메서드 목록
| 메서드 이름           | 예시 및 설명                                |
|--------------------------|---------------------------------------------|
| `abs()`                  | `(-3.5).abs()` → `3.5`                      |
| `signum()`               | `-3.5.signum()` → `-1.0`, `0.0.signum()` → `0.0`, `3.5.signum()` → `1.0` |
| `sqrt()`                 | `9.0.sqrt()` → `3.0`                        |
| `cbrt()`                 | `27.0.cbrt()` → `3.0`                       |
| `powf(exp)`              | `2.0.powf(3.0)` → `8.0`                     |
| `powi(n)`                | `2.0.powi(4)` → `16.0`                      |
| `exp()`                  | `1.0.exp()` → `e ≈ 2.718`                  |
| `exp2()`                 | `3.0.exp2()` → `8.0`                        |
| `ln()`                   | `e.ln()` → `1.0`                            |
| `log(base)`              | `10.0.log(2.0)` → `≈ 3.32`                  |
| `log2()`                 | `8.0.log2()` → `3.0`                        |
| `log10()`                | `1000.0.log10()` → `3.0`                    |
| `floor()`                | `3.7.floor()` → `3.0`                       |
| `ceil()`                 | `3.2.ceil()` → `4.0`                        |
| `round()`                | `3.6.round()` → `4.0`, `3.4.round()` → `3.0` |
| `trunc()`                | `3.9.trunc()` → `3.0`                       |
| `fract()`                | `3.9.fract()` → `0.9`                       |
| `hypot(x)`               | `3.0.hypot(4.0)` → `5.0` (`√(3² + 4²)`)     |
| `sin()`                  | `PI.sin()` → `≈ 0.0`                        |
| `cos()`                  | `0.0.cos()` → `1.0`                         |
| `tan()`                  | `PI/4.tan()` → `1.0`                        |
| `asin()`                 | `1.0.asin()` → `PI/2`                       |
| `acos()`                 | `1.0.acos()` → `0.0`                        |
| `atan()`                 | `1.0.atan()` → `≈ 0.785`                    |
| `atan2(y, x)`            | `atan2(1.0, 1.0)` → `PI/4`                  |
| `sinh()`                 | `1.0.sinh()` → `≈ 1.175`                    |
| `cosh()`                 | `1.0.cosh()` → `≈ 1.543`                    |
| `tanh()`                 | `1.0.tanh()` → `≈ 0.761`                    |
| `asinh()`                | `1.0.asinh()` → `≈ 0.881`                   |
| `acosh()`                | `2.0.acosh()` → `≈ 1.317`                   |
| `atanh()`                | `0.5.atanh()` → `≈ 0.549`                   |
| `is_nan()`               | `(0.0 / 0.0).is_nan()` → `true`            |
| `is_infinite()`          | `(1.0 / 0.0).is_infinite()` → `true`       |
| `is_finite()`            | `3.14.is_finite()` → `true`                |
| `is_sign_positive()`     | `3.0.is_sign_positive()` → `true`          |
| `is_sign_negative()`     | `-3.0.is_sign_negative()` → `true`         |
| `copysign(sign)`         | `3.0.copysign(-1.0)` → `-3.0`              |
| `max(y)` / `min(y)`      | `3.0.max(5.0)` → `5.0`, `3.0.min(5.0)` → `3.0` |
| `clamp(min, max)`        | `5.0.clamp(0.0, 4.0)` → `4.0`              |
| `mul_add(a, b)`          | `2.0.mul_add(3.0, 4.0)` → `10.0` (`2×3 + 4`) |
| `rem_euclid(div)`        | `-5.5.rem_euclid(2.0)` → `0.5`             |
| `div_euclid(div)`        | `-5.5.div_euclid(2.0)` → `-3.0`            |



## 📐 std::f64::consts 수학 상수
| 상수 이름         | 값 (대략)           | 설명                          |
|----------------------|---------------------|-------------------------------|
| `PI`                 | 3.1415926535…       | 원주율 π                      |
| `E`                  | 2.7182818284…       | 자연로그 밑 e                 |
| `TAU`                | 6.2831853071…       | 2π (원 전체 각도)             |
| `FRAC_PI_2`          | 1.5707963267…       | π/2                           |
| `FRAC_PI_3`          | 1.0471975511…       | π/3                           |
| `FRAC_PI_4`          | 0.7853981633…       | π/4                           |
| `FRAC_PI_6`          | 0.5235987755…       | π/6                           |
| `FRAC_PI_8`          | 0.3926990816…       | π/8                           |
| `LN_2`               | 0.6931471805…       | ln(2)                         |
| `LN_10`              | 2.3025850930…       | ln(10)                        |
| `LOG2_E`             | 1.4426950408…       | log₂(e)                       |
| `LOG10_E`            | 0.4342944819…       | log₁₀(e)                      |
| `SQRT_2`             | 1.4142135623…       | √2                            |
| `FRAC_1_PI`          | 0.3183098861…       | 1/π                           |
| `FRAC_2_PI`          | 0.6366197723…       | 2/π                           |
| `FRAC_2_SQRT_PI`     | 1.1283791670…       | 2/√π                          |

전체 상수 목록은 Rust 공식 문서의 f64::consts 페이지에서 확인할 수 있어요.


---

# libm
## 🧠 libm 크레이트의 주요 장점
### ✅ 1. no_std 환경에서 사용 가능
- libm은 표준 라이브러리 없이도 작동하는 순수 Rust 구현이에요
- 임베디드 시스템, OS 커널, WebAssembly 등 제한된 환경에서 수학 함수가 필요할 때 유용
### ✅ 2. C의 libm 없이도 동작
- 일반적으로 sqrt, atan2 같은 함수는 C의 libm에 의존하지만,
- libm 크레이트는 Rust로 직접 구현되어 있어서 외부 C 라이브러리 없이도 빌드 가능
### ✅ 3. 이식성과 호환성
- 다양한 플랫폼에서 동일한 결과와 동작 보장
- 크로스 컴파일 시에도 추가 설정 없이 안정적으로 작동
### ✅ 4. 경량성과 단순성
- libm은 의존성이 거의 없고 가볍기 때문에 작은 바이너리나 빠른 빌드에 적합
- 특히 no_std + libm 조합은 최소한의 런타임 환경을 구성할 때 강력함

## 📦 사용 예시
```
# Cargo.toml
libm = "0.2"
```

```rust
use libm::atan2;

fn main() {
    let angle = atan2(1.0, 1.0); // π/4
    println!("angle: {}", angle);
}
```

libm은 f64뿐 아니라 f32용 함수도 제공합니다 (libm::atan2f 등)


### ⚠️ 주의할 점
- libm은 정밀도나 성능 면에서 표준 라이브러리보다 느릴 수 있음
- 일부 함수는 표준 라이브러리보다 구현이 제한적일 수 있어요 (예: 특수 케이스 처리)

## ✨ 언제 쓰면 좋을까?
| 상황        | `libm` 사용 추천 여부 |
|-----------------------------|------------------------|
| `no_std` 환경               | ✅ 매우 유용함         |
| 크로스 컴파일               | ✅ 안정적 선택         |
| WebAssembly 개발            | ✅ C 라이브러리 없이 가능 |
| 임베디드 시스템             | ✅ 경량 수학 연산 가능  |
| OS 커널 / 런타임 개발       | ✅ 표준 라이브러리 없이 작동 |
| 일반 데스크탑 앱            | ❌ 표준 라이브러리 사용 권장 |
| 고정밀 수치 계산            | ⚠️ 제한적 (정밀도 낮을 수 있음) |
| 빠른 빌드 / 작은 바이너리   | ✅ 의존성 최소화 가능   |

결국 libm은 Rust의 순수 수학 엔진이라고 볼 수 있음.
표준 라이브러리 없이도 돌아가는 수학 연산이 필요할 때,
libm은 가장 믿을 수 있는 선택지 중 하나.



## 🧮 libm 지원 함수 목록 (정리된 대표 함수들)
| 함수 이름         | 설명 및 지원 타입       |
|------------------|--------------------------|
| `sqrt`, `sqrtf`  | 제곱근 (`f64`, `f32`)     |
| `cbrt`, `cbrtf`  | 세제곱근                  |
| `sin`, `sinf`    | 사인                      |
| `cos`, `cosf`    | 코사인                    |
| `tan`, `tanf`    | 탄젠트                    |
| `asin`, `asinf`  | 아크사인                  |
| `acos`, `acosf`  | 아크코사인                |
| `atan`, `atanf`  | 아크탄젠트                |
| `atan2`, `atan2f`| 아크탄젠트(y/x)           |
| `sinh`, `sinhf`  | 쌍곡 사인                 |
| `cosh`, `coshf`  | 쌍곡 코사인               |
| `tanh`, `tanhf`  | 쌍곡 탄젠트               |
| `asinh`, `asinhf`| 쌍곡 아크사인             |
| `acosh`, `acoshf`| 쌍곡 아크코사인           |
| `atanh`, `atanhf`| 쌍곡 아크탄젠트           |
| `exp`, `expf`    | 자연지수 함수             |
| `exp2`, `exp2f`  | 2의 거듭제곱              |
| `log`, `logf`    | 자연로그                  |
| `log2`, `log2f`  | 밑 2 로그                 |
| `log10`, `log10f`| 밑 10 로그                |
| `log1p`, `log1pf`| `ln(1 + x)`               |
| `expm1`, `expm1f`| `e^x - 1`                 |
| `pow`, `powf`    | 거듭제곱 (`x^y`)          |
| `fabs`, `fabsf`  | 절댓값                    |
| `floor`, `floorf`| 내림                      |
| `ceil`, `ceilf`  | 올림                      |
| `trunc`, `truncf`| 소수점 제거               |
| `round`, `roundf`| 반올림                    |
| `copysign`, `copysignf` | 부호 복사          |
| `fmod`, `fmodf`  | 나머지 (`x % y`)          |
| `rem_euclid`, `rem_euclidf` | 유클리드 나머지 |
| `hypot`, `hypotf`| 피타고라스 계산           |
| `ldexp`, `ldexpf`| `x * 2^exp`               |
| `frexp`, `frexpf`| `x = mantissa * 2^exp` 분해 |
| `is_nan`, `is_infinite`, `is_finite` | 특수값 판별 |
| `tgamma`, `tgammaf` | 감마 함수              |
| `lgamma`, `lgammaf` | 감마 함수의 로그        |
| `j0`, `j0f`      | 베셀 함수 (0차)           |
| `j1`, `j1f`      | 베셀 함수 (1차)           |
| `jn`, `jnf`      | 베셀 함수 (n차)           |



## 🔧 사용 예시
```rust
use libm::{atan2, sqrt};

fn main() {
    let angle = atan2(1.0, 1.0); // π/4
    let root = sqrt(9.0);        // 3.0
    println!("angle: {}, sqrt: {}", angle, root);
}
```

---


## ✅ 대안: 평균/분산 계산을 위한 크레이트
### 1.  크레이트
- Mean, Variance, Skewness, Kurtosis 등 다양한 통계량 계산 가능
- no_std 환경에서도 작동
- 예시:
```rust
use average::Mean;

let mut mean = Mean::new();
mean.add(1.0);
mean.add(2.0);
mean.add(3.0);

println!("평균: {}", mean.mean()); // 2.0
```

### 2. 직접 구현하는 방법
```rust
fn avg(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}
```

libm과 함께 쓰려면 sqrt, pow 등은 libm::sqrt, libm::pow로 호출하고
sum과 avg는 직접 구현하거나 표준 라이브러리의 Iterator를 활용해야 해요.


## ✨ 요약
- libm은 단일 값 수학 함수 전용 → sum, avg는 포함되지 않음
- 평균이나 분산 계산은 직접 구현하거나 average 크레이트 사용
- libm은 no_std 환경에서 삼각함수, 로그, 제곱근 등을 안전하게 제공하는 게 핵심

## ✅ 설치 방법
### 1. Cargo.toml에 추가
```
[dependencies]
average = "0.13"
```

최신 버전은 crates.io의 average 페이지에서 확인할 수 있어요.


### 2. 설치 및 빌드
```
cargo build
```

이 명령어를 실행하면 average 크레이트가 자동으로 다운로드되고 프로젝트에 포함됩니다.



---



##  🧠 핵심 개념: num은 "메타 크레이트"
- num은 여러 하위 크레이트를 묶은 통합 패키지입니다
- 필요에 따라 개별 크레이트만 선택해서 사용할 수도 있어요

📦 주요 하위 크레이트 구성
| 하위 크레이트      | 주요 기능 및 타입 지원                              |
|------------------------|-----------------------------------------------------|
| `num-traits`           | `Zero`, `One`, `ToPrimitive`, `FromPrimitive` 등 수학 트레이트 |
| `num-integer`          | `gcd`, `lcm`, `div_floor`, `mod_floor` 등 정수 전용 연산     |
| `num-bigint`           | `BigInt`, `BigUint` → 임의 정밀도 정수 계산               |
| `num-rational`         | `Ratio<T>` → 유리수(분수) 타입 지원                      |
| `num-complex`          | `Complex<T>` → 복소수 계산                             |
| `num-iter`             | `range`, `range_step`, `range_inclusive` 등 반복자 생성     |
| `num-derive`           | 수학 트레이트 자동 구현용 `#[derive(...)]` 매크로 지원     |



## ✨ 사용 예시
### 1. BigInt로 큰 수 계산
```rust
use num::bigint::BigInt;
use num::FromPrimitive;

let a = BigInt::from_u64(12345678901234567890).unwrap();
let b = BigInt::from(2);
let result = &a * &b;
```

### 2. Ratio로 유리수 계산
```rust
use num::rational::Ratio;

let r = Ratio::new(3, 4); // 3/4
let s = Ratio::new(2, 5); // 2/5
let sum = r + s;          // 23/20
```

### 3. Complex로 복소수 계산
```rust
use num::complex::Complex;

let z = Complex::new(2.0, 3.0);
let w = Complex::new(1.0, -1.0);
let result = z * w; // 복소수 곱셈
```


## ✅ 장점 요약
- 정수/실수/복소수/유리수 모두 지원
- 제네릭 수학 트레이트로 범용 코드 작성 가능
- no_std 지원 (일부 크레이트는 alloc 필요)
- 정확한 수학 계산을 위한 타입 안전성 확보

## ⚠️ 주의사항
- num 전체를 가져오면 크레이트 크기가 커질 수 있음
→ 필요한 하위 크레이트만 선택하는 게 좋음
- 일부 기능은 std가 필요하거나 alloc을 요구함 (num-bigint 등)


## 🧮 1. 미분 (수치적 미분)
### ✅ 예제: 중심 차분법 (Central Difference Method)
```rust
fn derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}

fn main() {
    let f = |x: f64| x.powi(2); // f(x) = x²
    let x = 3.0;
    let h = 1e-5;
    let df = derivative(f, x, h);
    println!("f'({}) ≈ {}", x, df); // 결과: f'(3) ≈ 6.0
}
```

이 방식은 오차가 있지만 간단하고 빠르며, 대부분의 실시간 계산에 적합해요.


## 📐 2. 적분 (수치적 적분)
### ✅ 예제: 사다리꼴 적분법 (Trapezoidal Rule)
```rust
fn integrate<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += f(x);
    }
    sum * h
}

fn main() {
    let f = |x: f64| x.powi(2); // f(x) = x²
    let result = integrate(f, 0.0, 3.0, 1000);
    println!("∫₀³ x² dx ≈ {}", result); // 결과: ≈ 9.0
}
```


