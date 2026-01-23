# total_cmp
- total_cmp 는 부동소수점(f32/f64)을 **완전한 전순서(total order)** 로 비교하기 위한 Rust 메서드.
- 보통 f64는 Ord를 구현하지 못하는데, NaN 때문에 **모든 값에 대해 항상 비교 가능** 한 순서를 만들 수 없기 때문.
- total_cmp는 이 문제를 해결하기 위해 비트 패턴 기반의 일관된 정렬 순서를 정의해준다.

## 1. 기본 개념
- 메서드 시그니처 (f64 기준):
```rust
impl f64 {
    pub fn total_cmp(&self, other: &f64) -> std::cmp::Ordering
}
```
- 역할:
- NaN, +∞, -∞, -0.0, +0.0까지 포함해서  
  모든 f64 값에 대해 항상 비교 결과를 반환한다.
- 즉, partial_cmp가 None을 반환하는 경우에도  
  total_cmp는 항상 Ordering::Less / Equal / Greater 중 하나를 준다.

## 2. 왜 필요한가?
### 2.1 partial_cmp의 한계
```rust
let a = f64::NAN;
let b = 1.0;

assert_eq!(a.partial_cmp(&b), None); // NaN은 비교 불가
```
- partial_cmp는 IEEE 754 규칙을 그대로 따르기 때문에  
  NaN이 끼어 있으면 비교 결과가 None이 되어버린다.
- 그래서 sort_by(|a, b| a.partial_cmp(b).unwrap()) 같은 코드는  
  NaN이 나오면 패닉이 날 수 있다.
### 2.2 total_cmp의 장점
```rust
let mut v = vec![1.0, f64::NAN, -0.0, 0.0, 5.0];
v.sort_by(|a, b| a.total_cmp(b));
```

- 어떤 값이 들어와도 항상 정렬 가능하다.
- NaN이 있어도 패닉 없이 정렬된다.
- 정렬 결과는 항상 동일한 규칙을 따른다(플랫폼/빌드에 상관없이).

### 3. 정렬 규칙(대략적인 순서)
- Rust 표준 라이브러리 구현은 비트 패턴을 이용해 전순서를 정의한다.
- 대략적인 순서는 다음과 같다 (정확한 내부 구현은 더 세밀하지만 개념적으로):
  - -∞
  - 음수들 (작은 것 → 큰 것)
  - -0.0
  - +0.0
  - 양수들 (작은 것 → 큰 것)
  - NaN들 (비트 패턴 순서대로)
- 즉:
```rust
let mut v = vec![f64::NAN, 1.0, -1.0, 0.0, -0.0];
v.sort_by(|a, b| a.total_cmp(b));
println!("{v:?}");
```
```
[-1.0, -0.0, 0.0, 1.0, NaN]
```
- 이런 식으로 정렬하면:
  - -1.0 < -0.0 < 0.0 < 1.0 < NaN
  - -0.0와 +0.0도 구분해서 순서를 정해준다.

## 4. total_cmp vs partial_cmp vs Ord

| 비교 방식     | NaN 비교 가능 여부 | 항상 비교 가능? | 정렬에 사용 가능? | 설명 |
|---------------|--------------------|------------------|--------------------|------|
| partial_cmp   | 불가능 (`None`)    | 아니오           | 조건부 가능        | IEEE 754 규칙을 따르며 NaN이 포함되면 비교 불가 |
| total_cmp     | 가능               | 예               | 예                 | 비트 패턴 기반 전순서(total order)를 제공하여 모든 f64 비교 가능 |
| Ord (정렬용)  | f64는 구현 불가    | -                | -                  | 전순서를 요구하므로 NaN이 있는 f64에는 직접 구현할 수 없음 |


- total_cmp는 **부동소수점용 의사-Ord** 라고 보면 된다.
- 그래서 보통 이렇게 쓴다:
```rust
v.sort_by(|a, b| a.total_cmp(b));
```


## 5. 자주 쓰는 패턴
### 5.1 정렬
```rust
let mut v = vec![3.0, f64::NAN, 2.0, -0.0, 0.0];
v.sort_by(|a, b| a.total_cmp(b));
```

### 5.2 Ord가 필요한 자료구조에 래핑해서 쓰기
```rust
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct F64Ord(f64);

impl Eq for F64Ord {}

impl Ord for F64Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
```

- 이렇게 하면 BTreeMap<F64Ord, T> 같은 것도 안전하게 쓸 수 있다.

## 6. 언제 total_cmp를 써야 할까?
- 정렬이 필요하고, NaN이 들어올 수도 있을 때
- 플랫폼/빌드에 상관없이 항상 같은 정렬 결과가 필요할 때
- 부동소수점을 키로 쓰는 정렬/맵/셋에서 안정적인 순서가 필요할 때
## 반대로:
- 수학적으로 **NaN은 비교 불가** 라는 의미를 유지하고 싶다면
  - partial_cmp를 그대로 쓰는 게 맞다.

- 한 줄로 정리하면:
- total_cmp는 **부동소수점에 대해 항상 잘 동작하는 정렬용 비교 함수**.
- NaN, -0.0까지 포함해서 완전한 전순서를 만들어준다.

---


