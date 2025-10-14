# Vector 비교 함수

vector cmp라는 표현은 문맥에 따라 다르게 해석될 수 있는데,
Rust나 C++ 같은 언어에서 벡터(Vector) 타입의 비교 함수를 의미.

## 🧠 1. Rust에서 Vec<T> 비교 (cmp 메서드)
Rust에서는 Vec<T>가 PartialEq, Eq, PartialOrd, Ord를 구현하고 있어서 벡터 간 비교가 가능합니다.
```rust
let a = vec![1, 2, 3];
let b = vec![1, 2, 4];
assert!(a < b); // 내부적으로 cmp 호출됨
```

- 내부적으로는 slice::cmp()가 호출되어 요소를 순서대로 비교합니다.
- 즉, cmp는 벡터의 내부 비교 함수라고 볼 수 있어요.

## 🧠 2. C++에서 std::vector 비교
C++에서도 std::vector<T>는 ==, <, > 등의 연산자를 오버로드해서 요소 단위로 비교합니다.
```cpp
std::vector<int> a = {1, 2, 3};
std::vector<int> b = {1, 2, 4};
bool result = a < b; // 내부적으로 lexicographical_compare
```
- 내부적으로는 std::lexicographical_compare가 사용됨
- 역시 내부 비교 함수가 존재한다고 볼 수 있음

## ✅ 벡터 비교 함수 요약

| 언어   | 비교 방식         | 내부 함수                  |
|--------|------------------|----------------------------|
| Rust   | 요소 순서 비교     | `slice::cmp()`             |
| Rust   | 부분 순서 비교     | `slice::partial_cmp()`     |
| Rust   | 동등성 비교        | `slice::eq()` / `slice::ne()` |
| C++    | 사전식 비교        | `std::lexicographical_compare` |
| C++    | 동등성 비교        | `operator==`, `operator!=` |


Rust의 Vec<T>는 다음 trait들을 구현해서 비교 기능을 제공합니다:
## 🧠 Vec<T>의 비교 관련 trait 요약
| Trait        | 지원되는 연산자/메서드 | 내부 동작 방식               |
|--------------|------------------------|------------------------------|
| PartialEq    | `==`, `!=`             | 요소를 순서대로 비교 (`slice::eq`, `slice::ne`) |
| Eq           | 완전한 동등성 비교      | `PartialEq`을 기반으로 구현됨 |
| PartialOrd   | `<`, `>`, `<=`, `>=`   | 부분 순서 비교 (`slice::partial_cmp`) |
| Ord          | `cmp()`                | 전체 순서 비교 (`slice::cmp`) |

## 🔍 내부 비교 함수: cmp, partial_cmp, eq, ne
### 1. cmp(&self, other: &Self) -> Ordering
- 전체 순서를 비교 (Ordering::Less, Equal, Greater)
- 요소를 사전식(lexicographical) 으로 비교
```rust
use std::cmp::Ordering;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 4];
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Equal => println!("a == b"),
        Ordering::Greater => println!("a > b"),
    }
}
```

### 2. partial_cmp(&self, other: &Self) -> Option<Ordering>
- T가 `PartialOrd` 만 구현한 경우 사용
- None이 나올 수 있음 (예: NaN 포함된 float 비교)
```rust
let a = vec![1.0, 2.0];
let b = vec![1.0, f64::NAN];

println!("{:?}", a.partial_cmp(&b)); // None
```


### 3. eq(&self, other: &Self) -> bool
- 요소를 순서대로 비교해서 완전히 같으면 true
```rust
let a = vec![1, 2, 3];
let b = vec![1, 2, 3];

assert!(a == b); // 내부적으로 eq 호출됨

```

### 4. ne(&self, other: &Self) -> bool
- eq의 반대
```rust
let a = vec![1, 2, 3];
let b = vec![1, 2, 4];

assert!(a != b); // 내부적으로 ne 호출됨
```


## ✅ 비교 연산자와 내부 함수 연결
| 연산자 | 내부 함수 호출           |
|--------|---------------------------|
| ==     | `eq()`                    |
| !=     | `ne()`                    |
| <      | `cmp()` 또는 `partial_cmp()` |
| >      | `cmp()` 또는 `partial_cmp()` |
| <=     | `cmp()` 또는 `partial_cmp()` |
| >=     | `cmp()` 또는 `partial_cmp()` |



## 💡 참고: Vec<T>가 비교 가능하려면?
- T가 `PartialEq`, `Eq`, `PartialOrd`, `Ord` 를 구현해야 함
- 예: Vec<f64>는 PartialOrd만 구현 → cmp() 불가, partial_cmp()만 가능


## 🧠 기본 비교 연산자 (==, <, cmp())는 고정된 trait 기반
```rust
let a = vec![1, 2, 3];
let b = vec![1, 2, 4];

assert!(a < b); // 내부적으로 a.cmp(&b) → 요소별 Ord 비교
```

- 이때 cmp()는 T: Ord가 구현한 비교 방식만 사용
- 외부 함수를 인자로 넣을 수 없음

## ✅ 외부 비교 함수를 쓰고 싶다면?
### 1. sort_by() 또는 sort_by_key()처럼 명시적으로 비교 함수를 넣는 API를 사용해야 합니다.
```rust
let mut v = vec!["apple", "banana", "pear"];
v.sort_by(|a, b| a.len().cmp(&b.len())); // 길이 기준 비교
```

- 이건 Vec<T>의 정렬에서 외부 비교 함수를 인자로 넣는 대표적인 예
- sort_by()는 Fn(&T, &T) -> Ordering을 받음

### 2. 직접 비교 함수로 벡터를 비교하려면 Iterator 기반으로 구현
```rust
fn vec_cmp_by<T, F>(a: &[T], b: &[T], cmp_fn: F) -> std::cmp::Ordering
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    for (x, y) in a.iter().zip(b.iter()) {
        let ord = cmp_fn(x, y);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
    }
    a.len().cmp(&b.len())
}
```

- 이렇게 하면 외부 비교 함수로 Vec<T>를 비교할 수 있음
- 예: 길이 비교, 사용자 정의 우선순위 등

## 💡 왜 trait 기반으로 설계했는가?
Rust의 trait 시스템은 비교 로직을 타입에 따라 커스터마이즈할 수 있도록 설계.

- Ord, PartialOrd를 직접 구현하면
해당 타입의 비교 방식 자체를 바꿀 수 있음
- 예: struct Point에 대해 x 좌표 기준으로만 비교하도록 Ord 구현
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}
```

## 🔚 Vec<T> 비교 방식 요약
| 비교 방식             | 외부 함수 사용 가능 | 내부 동작 방식             |
|----------------------|---------------------|----------------------------|
| `==`, `<`, `cmp()`   | ❌ 불가능            | `Ord`, `PartialOrd` trait 기반 (`slice::cmp`) |
| `sort_by`, `vec_cmp_by` | ✅ 가능             | 사용자 정의 비교 함수 인자로 전달 |
| `Ord` trait 구현      | ✅ 가능 (타입 수준)  | 타입 자체에 비교 로직 내장 (`impl Ord for T`) |

----
# float 정렬
float 값은 정렬할 때 항상 까다로움.  
특히 NaN, -0.0, +0.0, 정밀도 문제 때문에 기본 비교 연산자만으로는 원하는 결과를 얻기 어려울 때가 많습니다.  
그래서 외부 비교 함수를 넣어서 정렬 기준을 직접 정의하는 게 핵심 전략입니다.  

## 🧠 Rust에서 float 정렬 시 외부 비교 함수 넣는 샘플
### 🔹 기본 정렬은 `partial_cmp()` 사용
```rust
let mut data = vec![3.2, f64::NAN, 1.5, 2.8];
data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
println!("{:?}", data); // NaN은 맨 뒤로
```

- `partial_cmp()` 는 `Option<Ordering>` 을 반환하므로 `unwrap_or()` 로 처리
- NaN은 비교 불가 → 기본적으로 맨 뒤로 보내는 전략

### 🔹 사용자 정의 기준: 절댓값 기준 정렬
```rust
let mut data = vec![-3.2, 1.5, -2.8, 0.0];
data.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
println!("{:?}", data); // [0.0, 1.5, -2.8, -3.2]
```

- abs() 기준으로 정렬
- partial_cmp()는 여전히 필요

### 🔹 사용자 정의 기준: NaN 우선 정렬
```rust
let mut data = vec![3.2, f64::NAN, 1.5, f64::NAN, 2.8];
data.sort_by(|a, b| {
    match (a.is_nan(), b.is_nan()) {
        (true, true) => std::cmp::Ordering::Equal,
        (true, false) => std::cmp::Ordering::Less,  // NaN 먼저
        (false, true) => std::cmp::Ordering::Greater,
        (false, false) => a.partial_cmp(b).unwrap(),
    }
});
println!("{:?}", data); // NaN이 앞에 옴
```

- NaN을 우선 정렬하거나 뒤로 보내는 전략은 실전에서 자주 쓰입니다


## ✅ 요약: 외부 비교 함수로 float 정렬할 때 핵심

| 항목            | 설명 또는 역할                          |
|-----------------|------------------------------------------|
| `partial_cmp()` | `f64`는 `Ord`를 구현하지 않음 → `cmp()` 불가 |
| `unwrap_or()`   | `NaN` 처리 시 fallback 필요 (`Ordering::Greater` 등) |
| 사용자 기준 정의 | `abs()`, `log()`, `custom weight` 등으로 비교 기준 설정 |
| `sort_by()`     | 외부 비교 함수를 인자로 넣는 핵심 API |


### 🔍 핵심 예시
```rust
let mut data = vec![3.2, f64::NAN, -1.5, 0.0];

data.sort_by(|a, b| {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater)
});
```

또는 절댓값 기준:
```rust
data.sort_by(|a, b| {
    a.abs().partial_cmp(&b.abs()).unwrap()
});
```

---

# closure 사용

Rust에서는 **클로저(closure)** 를 아주 강력하게 지원하고,  
특히 Vec<T>의 정렬이나 필터링, 매핑 같은 고차 함수에서  
외부 비교 함수로 클로저를 직접 인자로 넣는 방식이 자주 쓰입니다.


## ✅ 클로저를 외부 비교 함수로 쓰는 예시
### 🔹 sort_by()에 클로저 넣기
```rust
let mut data = vec![3.2, f64::NAN, 1.5, 2.8];

data.sort_by(|a, b| {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater)
});
```

- 여기서 |a, b| ...가 바로 클로저
- `partial_cmp()` 는 `Option<Ordering>` 을 반환하므로 `unwrap_or()` 로 처리

### 🔹 절댓값 기준 정렬
```rust
let mut data = vec![-3.2, 1.5, -2.8, 0.0];

data.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
```
- a.abs()와 b.abs()를 비교하는 클로저

### 🔹 filter()나 map()에도 클로저 사용
```rust
let data = vec![1, 2, 3, 4, 5];
let even: Vec<_> = data.into_iter().filter(|x| x % 2 == 0).collect();
```
- |x| x % 2 == 0 → 짝수만 필터링

## 🔍 클로저의 특징 요약
| 항목             | 설명 또는 역할                          |
|------------------|------------------------------------------|
| 환경 캡처        | 외부 변수 참조 가능 (`move` 키워드로 소유권 이동 가능) |
| 타입 추론        | 대부분의 경우 매개변수 타입 생략 가능 |
| 고차 함수 활용    | `sort_by`, `filter`, `map`, `fold` 등에서 자주 사용 |
| 익명 함수 형태    | `|x| x + 1` 같은 간단한 문법으로 작성 가능 |


---

# trait 사용

비교 로직을 일반화하거나 재사용하려면 trait가 훨씬 낫습니다.  
클로저는 강력하지만, 일회성, 지역성, 타입 추론 의존성 때문에  
복잡한 비교 로직이나 여러 곳에서 쓰이는 비교 기준에는 trait 기반 설계가 더 안정적이고 확장성도 좋습니다.  

## 🔍 클로저 vs Trait 비교 요약
| 항목             | 클로저                                      | Trait                                       |
|------------------|----------------------------------------------|---------------------------------------------|
| 사용 편의성       | 간단하고 빠르게 작성 가능 (`|a, b| ...`)         | 초기 설정은 복잡하지만 구조화된 설계 가능       |
| 재사용성         | 지역적, 일회성 사용에 적합                      | 전역적, 여러 곳에서 재사용 가능                |
| 타입 안정성       | 타입 추론에 의존 → 복잡한 경우 오류 가능         | 명시적 타입 선언으로 안정적 컴파일 가능         |
| 성능 최적화       | 런타임 캡처 → 성능 저하 가능성 있음              | 컴파일 타임 최적화 가능 → 성능 우위             |
| 구조화된 설계     | 즉흥적 로직에 적합                              | 인터페이스 기반 설계에 적합                    |
| 고차 함수 활용     | `sort_by`, `filter`, `map`, `fold` 등에서 사용   | `Ord`, `Compare`, `Fn` trait 등으로 확장 가능   |


### ✅ Trait 기반 비교 예시
```rust
use std::cmp::Ordering;

trait Compare<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering;
}

struct AbsCompare;

impl Compare<f64> for AbsCompare {
    fn compare(&self, a: &f64, b: &f64) -> Ordering {
        a.abs().partial_cmp(&b.abs()).unwrap()
    }
}
```

- 이 구조는 여러 곳에서 재사용 가능
- Vec<T> 정렬, 해시, 필터링 등 다양한 곳에 적용 가능

### 🧠 클로저는 언제 쓰면 좋은가?
- 간단한 정렬 기준이 필요한 경우
- 한 번만 쓰는 비교 로직일 때
- 외부 환경을 캡처해야 할 때
let weight = vec![1.0, 0.5, 2.0];
data.sort_by(|a, b| {
    let wa = weight[*a as usize];
    let wb = weight[*b as usize];
    wa.partial_cmp(&wb).unwrap()
});

---

# `partial_cmp` / `cmp`

`partial_cmp`와 `cmp` 는 둘 다 비교를 위한 메서드지만, 동작 방식과 적용 대상이 다릅니다.  
특히 float 같은 값에서 차이가 확실히 드러나죠.

## 🔍 핵심 차이 요약
| 항목             | cmp()                   | partial_cmp()             |
|------------------|--------------------------|----------------------------|
| 기반 Trait       | `Ord`                    | `PartialOrd`               |
| 반환 타입        | `Ordering`               | `Option<Ordering>`         |
| 비교 실패 가능성 | ❌ 없음                  | ✅ `None` 가능 (`NaN` 등)   |
| 대표 사용 타입   | `i32`, `String`, `bool` | `f64`, `f32`               |

## 🧠 예시로 비교해보기
### ✅ cmp() 사용 (정수 비교)
```rust
let a = 3;
let b = 5;

let result = a.cmp(&b); // Ordering::Less
```

- 항상 Ordering 반환
- Ord trait이 구현된 타입만 사용 가능

###  ⚠️ partial_cmp() 사용 (float 비교)
```rust
let a = 3.2;
let b = f64::NAN;
let result = a.partial_cmp(&b); // None
```

- NaN은 비교 불가 → None 반환
- cmp()는 사용할 수 없음 → 컴파일 에러

## ✅ 실무에서의 판단 기준
| 상황 또는 조건         | 추천 방식                          |
|------------------------|------------------------------------|
| 완전한 비교 가능 타입   | `cmp()` 사용 (`Ord` 기반)           |
| 비교 실패 가능성 있음   | `partial_cmp().unwrap_or()` 사용 (`PartialOrd` 기반) |
| 사용자 정의 타입 비교   | `Ord` 또는 `PartialOrd` 직접 구현   |

---
# partial_cmp / cmd 결과값

Vec<T>를 비교할 때는 하나씩 순서대로 비교하지만,
결과는 최종적으로 하나의 Ordering 값만 나옵니다.

## 🔍 동작 방식: 요소별 순차 비교 → 하나의 결과
``` rust
let a = vec![1, 2, 3];
let b = vec![1, 2, 4];

let result = a.cmp(&b); // Ordering::Less
```

- 내부적으로는 slice::cmp()가 호출됨
- a[0] == b[0] → 계속 비교
a[1] == b[1] → 계속 비교
a[2] < b[2] → 여기서 Ordering::Less 반환
- 즉, 요소를 하나씩 비교하지만 결과는 하나의 Ordering 값

### ✅ 예시: 비교 흐름
| 인덱스 | a[i] | b[i] | 비교 결과     | 진행 여부     |
|--------|------|------|----------------|----------------|
| 0      | 1    | 1    | Equal          | 다음 요소 비교 |
| 1      | 2    | 2    | Equal          | 다음 요소 비교 |
| 2      | 3    | 4    | Less           | 비교 종료      |

- 이후 요소는 비교하지 않음
- 마치 문자열 비교처럼 사전식(lexicographical) 비교

## 🧠 실무에서의 의미
- Vec<T> 비교는 정렬 기준으로도 사용 가능
- cmp()는 전체 벡터의 순서를 판단하는 하나의 값만 반환
- 요소별 비교 결과를 따로 보고 싶다면 zip() + map()으로 직접 처리해야 함
```rust
let diffs: Vec<_> = a.iter().zip(b.iter()).map(|(x, y)| x.cmp(y)).collect();
```
Vec<T>를 비교할 때는 앞에서부터 순차적으로 요소를 하나씩 비교하고, 처음으로 다른 값이 나오는 순간 바로 결과를 반환합니다.

## 🔍 예시 흐름 다시 정리
```rust
let a = vec![1, 2, 3];
let b = vec![9, 2, 3];
let result = a.cmp(&b); // Ordering::Less
```
## ✅ Vec<T> 비교 흐름 예시
| 인덱스 | a[i] | b[i] | 비교 결과 | 진행 여부     | 최종 결과       |
|--------|------|------|------------|----------------|------------------|
| 0      | 1    | 9    | Less       | 비교 종료      | Ordering::Less   |
| 1      | —    | —    | —          | 비교 안 함     | —                |
| 2      | —    | —    | —          | 비교 안 함     | —                |

- a[0] < b[0] → 바로 Ordering::Less 반환
- 이후 요소는 비교하지 않음

## ✅ 실무에서 이걸 왜 중요하게 봐야 하나?
- 성능 최적화: 큰 벡터라도 앞부분에서 차이가 나면 빠르게 비교 종료
- 정렬 기준 설계: 앞쪽 요소가 우선순위가 높다는 뜻 → 정렬 기준을 앞에 배치
- 디버깅 시 주의: “왜 이 벡터가 작다고 나왔지?” → 앞 요소 먼저 확인해야 함

---
# cmp None이 들어 오면

핵심부터 말씀드리면: cmp()는 Ord trait 기반이기 때문에, 비교할 수 없는 값(None)이 들어가 있으면 컴파일 자체가 안 됩니다.

## 🔍 상황 정리
예: `Vec<Option<i32>>` 를 cmp()로 비교하려고 하면?
```rust
let a = vec![Some(1), None];
let b = vec![Some(1), Some(2)];

let result = a.cmp(&b); // ✅ 컴파일 에러 아님!
```

- 이유: Option<i32>는 Ord를 구현하지만,
None과 Some(_) 사이의 비교는 정의되어 있어야 함
- 다행히 Option<T>는 T: Ord일 때 Ord를 구현하므로
위 코드는 정상적으로 비교됩니다
→ None < Some(_)으로 간주됨

## ✅ Option<T>의 Ord 구현 방식
```rust
None < Some(_) // 항상 참
```

- 즉, cmp()는 None을 가장 작은 값으로 처리함
- Some(…)끼리는 내부 값으로 비교

### ⚠️ 그런데 f64처럼 Ord가 없는 타입을 Option<f64>로 감싸면?
```rust
let a = vec![Some(1.0), None];
let b = vec![Some(2.0), Some(3.0)];

let result = a.cmp(&b); // ❌ 컴파일 에러!
```

- 이유: f64는 Ord를 구현하지 않음 → Option<f64>도 Ord 없음
- 따라서 cmp()는 사용할 수 없음

## ✅ 해결 방법: partial_cmp() 사용
```rust
let result = a.partial_cmp(&b); // → Option<Ordering>
```

- None을 포함한 비교도 가능
- unwrap_or()로 fallback 처리 필요

## 🔚 요약: Option<T>와 Vec<Option<T>> 비교 방식
| 타입               | cmp() 가능 여부 | 관련 trait 설명                     |
|--------------------|------------------|--------------------------------------|
| Option<i32>        | ✅ 가능           | `i32: Ord`, `Option<T>: Ord`         |
| Option<f64>        | ❌ 불가능         | `f64: PartialOrd`, `Ord` 미구현      |
| Vec<Option<f64>>   | ❌ cmp() 불가     | ✅ `partial_cmp()` 사용 가능          |


Option<T>와 float을 함께 다루려면 단순히 “비교된다”가 아니라  
**타입 조합에 따라 비교 가능 여부가 어떻게 달라지는가** 를 정확히 알고 있어야 함.

---



