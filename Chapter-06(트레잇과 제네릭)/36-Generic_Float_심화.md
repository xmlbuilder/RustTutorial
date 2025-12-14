# Float Generic
- 코드가 컴파일되지 않는 이유는 PartialOrd만으로는 뺄셈(-) 연산을 지원하지 않기 때문.
- value - low 같은 표현은 T가 Copy + PartialOrd만 구현했다고 해서 가능한 게 아님.
- T가 f32나 f64 같은 실수형일 때만 의미가 있어요.
- 수정 방법
  - 만약 f32나 f64 범위 제한을 하고 싶다면, 제네릭을 쓸 필요 없이 그냥 실수형으로 고정하는 게 가장 간단합니다:

```rust
const ON_EPSILON: f32 = 0.0001;
pub fn on_limit_range(low: f32, value: &mut f32, high: f32) {
    if *value - low < ON_EPSILON {
        *value = low;
    } else {
        if *value - high <= ON_EPSILON {
            return;
        }
        *value = high;
    }
}
```
---

# Generic으로 구현

- T가 반드시 Sub<Output=T>와 Copy를 구현해야 하고, ON_EPSILON과 비교할 수 있어야 합니다.
- 예를 들어 num_traits 크레이트를 쓰면 더 일반화할 수 있음:
```rust std::ops::Sub;
use num_traits::Float;

pub fn on_limit_range<T>(low: T, value: &mut T, high: T)
where
    T: Float + Sub<Output = T> + Copy,
{
    if *value - low < T::epsilon() {
        *value = low;
    } else {
        if *value - high <= T::epsilon() {
            return;
        }
        *value = high;
        }
```
- 여기서는 T: Float 제약을 걸어서 f32, f64 같은 타입만 받도록 했습니다.

- 👉 정리하면:
  - 그냥- 그냥 f32/f64만 쓸 거라면 제네릭을 없애는 게 깔끔합 니다.
  - 여러 실수 타입을 지원하고 싶다면 num_traits::Float 같은 트레이트를 사용.
    


## What the function is doing
- 이 함수는 값 value를 하한 low와 상한 high 사이로 **가까운 쪽** 에 맞춰주는 로직. 
- 다만 단순한 클램프(clamp)처럼 항상 범위 안으로 자르는 게 아니라, 두 경계 근처에서는 허용 오차 epsilon에 기반해 약간의 여유를 둡니다.

## Trait bounds and imports
- Float: f32, f64 같은 부동소수점 타입에 필요한 메서드(예: epsilon())를 제공합니다.
- Sub<Output = T>: T 타입끼리 뺄셈이 가능하도록 합니다.
- Copy: T가 복사 가능한 타입임을 보장해, 소유권 이동 없이 값 복사가 가능합니다.
- 이렇게 해서 *value - low나 T::epsilon() 같은 연산이 컴파일되도록 제약을 걸고 있습니다.

## Parameters and mutability
- low / high: 하한과 상한입니다.
- value: &mut T: 변경 가능한 참조이므로, 함수 안에서 *value = ...로 실제 값을 바꿀 수 있습니다.
- 역참조: *value는 참조를 값으로 바꿔 사용하는 표준 방식입니다.

## Epsilon and comparison
- T::epsilon(): 부동소수점 연산에서의 **머신 엡실론** 입니다.
- 해당 타입이 표현할 수 있는 가장 작은 상대적 간격으로, 일반적 의미의 허용 오차와는 다릅니다.
- 비교 의미:
  - *value - low < T::epsilon()는 value가 low에 **매우 가깝다** 는 뜻.
  - *value - high <= T::epsilon()는 value가 high보다 작거나 거의 같은(또는 아주 조금 큰) 범위에 있다는 뜻입니다.

## Branching logic step by step
- 아래 경계에 매우 가까우면 하한으로 맞춤
- 조건: *value - low < T::epsilon()
  - 동작: *value = low
  - 해석: value가 low에 충분히 근접하면, 아예 low로 스냅합니다.
  - 그 외의 경우, 상한 근처면 유지
- 조건: *value - high <= T::epsilon()
  - 동작: return (변경 없음)
  - 해석: value가 high보다 작거나, high를 아주 조금 초과한 정도면 그대로 둡니다. 즉 value가 low + epsilon 이상이고 high + epsilon 이하이면 건드리지 않습니다.
  - 상한을 충분히 초과하면 상한으로 맞춤
- 조건: 위 조건들을 모두 통과한 경우(즉 value > high + epsilon)
  - 동작: *value = high
  - 해석: value가 상한을 의미 있게 넘어가면 high로 클램프합니다.
- 결과적으로 로직은:
  - low 근처에서는 아래로 스냅,
  - 범위 안과 high 근처(조금 초과까지)는 유지,
  - high를 의미 있게 초과하면 위로 스냅합니다.

## Caveats and practical notes
- 머신 엡실론 사용의 의미: T::epsilon()은 수학적 허용 오차가 아니라 타입의 표현 정밀도 한계입니다.
- 실무에서는 보통 도메인에 맞는 tolerance 값을 파라미터로 받는 편이 안전합니다.
- 포함/배제 경계: 아래쪽 비교는 <이고 위쪽은 <=라서, low 쪽은 조금 더 보수적으로 스냅됩니다.
- 진짜 clamp와의 차이: 일반적인 clamp(value, low, high)는 범위를 벗어나면 즉시 잘라내지만,  
  이 함수는 high + epsilon까지는 그대로 두는 **유예** 가 있습니다.

## Small example
```rust
use num_traits::Float;

fn main() {
    let low = 0.0f64;
    let high = 10.0f64;
    let mut v1 = 0.0 + f64::epsilon();      // 거의 low
    let mut v2 = 9.999999999999;            // high 근처(아래)
    let mut v3 = 10.0 + f64::epsilon() / 2.0; // high를 아주 조금 초과
    let mut v4 = 12.0;                      // high를 의미 있게 초과

    on_limit_range(low, &mut v1, high); // v1 -> 0.0
    on_limit_range(low, &mut v2, high); // v2 유지
    on_limit_range(low, &mut v3, high); // v3 유지 (epsilon 이하 초과)
    on_limit_range(low, &mut v4, high); // v4 -> 10.0
}
```
- v1: low에 매우 가까워서 0.0으로 스냅.
- v2: 범위 안이므로 유지.
- v3: high를 아주 조금 초과했지만 epsilon 이하라 유지.
- v4: 상한을 의미 있게 초과해서 10.0으로 클램프.

--
## num_traits::Float

- num_traits::Float는 Rust에서 부동소수점 타입(f32, f64)을 일반화해서 다룰 수 있도록 해주는 트레이트입니다.
- 즉, 제네릭 함수 안에서 f32와 f64를 공통된 방식으로 처리할 수 있게 해주는 역할을 합니다.

## 핵심 역할
- 부동소수점 전용 제약
  - T: Float라고 쓰면, T는 반드시 f32나 f64 같은 부동소수점 타입이어야 합니다. 따라서 정수형(i32, u64) 같은 타입은 사용할 수 없습니다.
- 부동소수점 관련 메서드 제공
  - Float 트레이트는 수학적으로 자주 쓰이는 기능들을 제공합니다. 예를 들어:
    - T::epsilon() → 해당 타입의 머신 엡실론(가장 작은 차이값)
    - T::infinity() / T::neg_infinity() → 양/음의 무한대
    - T::nan() → NaN(Not a Number)
    - T::sqrt() → 제곱근
    - T::abs() → 절댓값
    - T::floor(), T::ceil(), T::round() → 내림, 올림, 반올림
    - T::is_nan(), T::is_infinite() → NaN/무한대 판별
    - 제네릭 수학 연산 가능
- Float를 사용하면 f32와 f64를 따로 나누지 않고, 제네릭 함수 하나로 두 타입을 모두 지원할 수 있습니다.

## 간단 예시
```rust
use num_traits::Float;

fn normalize<T: Float>(x: T, min: T, max: T) -> T {
    (x - min) / (max - min)
}

fn main() {
    let a: f32 = normalize(5.0, 0.0, 10.0); // f32로 동작
    let b: f64 = normalize(2.5, 0.0, 5.0);  // f64로 동작
    println!("{}, {}", a, b);
}
```
- 여기서 normalize 함수는 Float 덕분에 f32와 f64 모두에서 잘 작동합니다.

- 👉 정리하면:
- num_traits::Float는 부동소수점 타입을 제네릭으로 다룰 수 있게 해주는 트레이트이며,  
  부동소수점 연산에 필요한 다양한 메서드와 상수를 제공합니다.

---

## 🛠️ num_traits::Float에서 실무에서 자주 쓰이는 기능들
### 1. 상수 관련
- T::epsilon() → 머신 엡실론 (부동소수점에서 구분 가능한 최소 차이값)
- T::infinity() / T::neg_infinity() → 양/음의 무한대
- T::nan() → NaN(Not a Number)
- T::zero() / T::one() → 0과 1 값
### 2. 기본 연산
- abs() → 절댓값
- sqrt() → 제곱근
- powi(n: i32) → 정수 거듭제곱
- powf(n: T) → 부동소수점 거듭제곱
- recip() → 역수 (1/x)
### 3. 반올림 관련
- floor() → 내림
- ceil() → 올림
- round() → 반올림
- trunc() → 소수점 이하 버림
- fract() → 소수점 이하 부분만 추출
### 4. 삼각함수 및 로그
- sin(), cos(), tan() → 삼각함수
- asin(), acos(), atan() → 역삼각함수
- atan2(y, x) → 두 값으로 각도 계산
- exp() → 지수 함수 (e^x)
- ln() → 자연 로그
- log(base) → 임의의 밑 로그
### 5. 상태 체크
- is_nan() → NaN 여부
- is_infinite() → 무한대 여부
- is_finite() → 유한 값 여부
- is_normal() → 정상(normalized) 값 여부

### 📌 실무에서 자주 쓰이는 패턴
- 허용 오차 비교
```rust
if (a - b).abs() < T::epsilon() {
    // a와 b가 거의 같다
}
```
- 정규화(normalization)
```rust
let normalized = (x - min) / (max - min);
- 안전한 값 체크
if value.is_nan() || !value.is_finite() {
    // 잘못된 값 처리
}
```
- 👉 정리하면, Float는 부동소수점 연산을 제네릭으로 통일해주는 트레이트이고, 실무에서는 주로:
  - epsilon, infinity, nan 같은 상수
  - abs, sqrt, pow 같은 기본 연산
  - floor, ceil, round 같은 반올림
  - 삼각함수, 로그
  - is_nan, is_finite 같은 상태 체크
 
---




