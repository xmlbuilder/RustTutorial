
# Set 커스텀 타입 비교

아래는 Rust에서 커스텀 타입을 Set에 넣는 예제와,  
`f64를 tol` 기반으로 근사 비교해서 중복을 통제하는 Set 예제를 함께 정리.

## 🧩 1. 커스텀 타입을 BTreeSet에 넣기
- `Ord`, `PartialOrd` 정의
```rust
use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}
```
```rust
// 나이 기준으로 정렬
impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}
```
```rust
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
```rust
fn main() {
    let mut set = BTreeSet::new();
    set.insert(Person { name: "Alice".into(), age: 30 });
    set.insert(Person { name: "Bob".into(), age: 25 });
    set.insert(Person { name: "Charlie".into(), age: 30 }); // age 기준으로 중복 → 무시됨

    for person in &set {
        println!("{} ({})", person.name, person.age);
    }
}
```

### 출력: 
```
Bob (25), Alice (30) — Charlie는 나이 기준으로 중복이라 무시됨
```


## 🧩 2. `f64를 tol` 로 통제하는 HashSet 예제
- `Eq`, `Hash` 정의
```rust
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq)]
struct F64Key {
    value: f64,
    tol: f64,
}
```
```rust
impl Eq for F64Key {}
```
```rust
impl Hash for F64Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let rounded = (self.value / self.tol).round() as i64;
        rounded.hash(state);
    }
}
```
```rust
impl PartialEq for F64Key {
    fn eq(&self, other: &Self) -> bool {
        let a = (self.value / self.tol).round() as i64;
        let b = (other.value / other.tol).round() as i64;
        a == b
    }
}
```
```rust
fn main() {
    let tol = 1e-3;
    let mut set = HashSet::new();

    set.insert(F64Key { value: 1.0001, tol });
    set.insert(F64Key { value: 1.0002, tol }); // tol 내에서 중복 → 덮어씀
    set.insert(F64Key { value: 1.0020, tol }); // 다른 키 → 추가됨

    for item in &set {
        println!("{:.4}", item.value);
    }
}
```

### 출력 예: 
```
1.0001, 1.0020 — 1.0001과 1.0002는 tol 내에서 같으므로 하나만 유지됨
```

### 암묵적으로 만들어 지는 코드
```rust
// 암묵적으로 생성된 코드
fn eq(&self, other: &Self) -> bool {
    self.value == other.value && self.tol == other.tol
}
```

## ✅ 요약 – Rust에서 Set + f64 근사 비교

| 대상     | 구현 방식                                 |
|----------|--------------------------------------------|
| Set      | `Ord` (for `BTreeSet`) / `Hash + Eq` (for `HashSet`) |
| f64      | `tol` 기반 래퍼 타입으로 정렬/중복 통제       |
| 비교 방식 | `cmp()` 또는 `hash()`에서 `abs(diff) < tol` 처리 |


## 🧩 1. Set에서 교집합 / 차집합 연산
Rust의 HashSet과 BTreeSet은 모두 집합 연산을 지원합니다:

### ✅ 예제: 교집합 (intersection), 차집합 (difference)
```rust
use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [2, 3, 4].into_iter().collect();

    let inter: HashSet<_> = a.intersection(&b).copied().collect();
    let diff: HashSet<_> = a.difference(&b).copied().collect();

    println!("교집합: {:?}", inter); // {2, 3}
    println!("차집합: {:?}", diff);  // {1}
}
```
- BTreeSet도 동일한 API를 갖고 있음.

---

## 🧠 2. impl Eq for F64Key {} 꼭 필요한 이유
### ✅ 왜 비어 있어도 impl Eq가 필요할까?
Rust에서 Eq는 PartialEq의 확장으로, **모든 값은 자기 자신과 같아야 한다** 는 수학적 동치성을 보장하는 marker trait입니다.
- Eq는 비교 연산을 직접 정의하지 않음
- PartialEq만 구현하면 `==` 는 작동하지만,
- HashSet, HashMap에서는 Eq가 반드시 필요합니다

### 🔍 예시: HashSet에서 Eq 없으면 컴파일 에러
```rust
use std::collections::HashSet;

#[derive(Hash, PartialEq)]
struct F64Key {
    value: f64,
    tol: f64,
}
```
```rust
// ❌ Eq가 없으면 HashSet<F64Key> 사용 불가
let mut set = HashSet::new();
set.insert(F64Key { value: 1.0, tol: 1e-3 }); // 컴파일 에러
```

### 해결: 
```rust
impl Eq for F64Key {} 추가하면 OK
```

### ✅ 요약 – Rust의 Set 연산과 Eq 트레이트

| 항목         | 설명                                      |
|--------------|-------------------------------------------|
| Set          | `intersection()`, `difference()` 등 집합 연산 지원 |
| `impl Eq {}` | `HashSet`, `HashMap`에서 필수 (비어 있어도 선언 필요) |
| Eq           | `PartialEq`의 확장, 동치성 보장 marker trait |

---

# PartialOrd / Ord

PartialOrd는 일부 값만 비교 가능한 타입에 꼭 필요하며, Ord를 쓸 수 없는 경우에만 사용됩니다.  
반대로, 비교가 항상 가능해야 하는 곳에서는 `PartialOrd` 를 `단독` 으로 쓰면 안 됩니다.

## 🧠 PartialOrd가 반드시 필요한 경우
PartialOrd는 **부분 순서(partial order)** 를 표현하는 트레이트로, 비교가 항상 가능하지 않은 타입에 사용됩니다.  
대표적인 예는 **부동소수점 타입 f32, f64**입니다.

## ✅ 꼭 필요한 상황 – PartialOrd
| 대상               | 이유 및 설명                              |
|--------------------|-------------------------------------------|
| `f64`, `f32`       | `NaN` 존재 → `Ord` 불가, `PartialOrd`만 가능 |
| `Option<T>`        | `None` vs `Some(_)` 비교 불가능하게 만들 수 있음 |
| `sort_by()` / `partial_cmp()` | `Vec<f64>` 등에서 부분 비교 정렬 시 필요 |

```rust
let a = std::f64::NAN;
let b = 1.0;
assert_eq!(a.partial_cmp(&b), None); // 비교 불가
```

### ❌ PartialOrd를 쓸 수 없는 곳
- PartialOrd는 정렬 자료구조나 완전한 비교가 필요한 곳에서는 사용할 수 없습니다.

### ❌ 사용 불가한 상황 – PartialOrd 단독으로는 안 되는 곳
| 대상               | 요구 사항 및 이유                         |
|--------------------|-------------------------------------------|
| `BTreeMap`, `BTreeSet` | 내부 정렬 필요 → `Ord` 필수               |
| `sort()`           | 전체 정렬 → `Ord` 필요 (`sort_by()`는 `PartialOrd` 가능) |
| `min()`, `max()`   | 항상 비교 가능해야 함 → `Ord` 필요         |

### 💡 참고:
- `PartialOrd` 는 `Option<Ordering>` 을 반환하므로 None이 나올 수 있음.
- `Ord` 는 항상 `Ordering` 을 반환해야 하므로 정렬 자료구조나 전체 정렬 함수에서만 사용 가능.

```rust
use std::collections::BTreeSet;
let mut set = BTreeSet::new();
set.insert(1.0); // ❌ f64는 Ord를 구현하지 않아서 컴파일 에러
```

## ✅ PartialOrd vs Ord 비교 요약
| 구분           | PartialOrd                              | Ord                             |
|----------------|------------------------------------------|----------------------------------|
| 비교 메서드     | `partial_cmp()` → `Option<Ordering>`     | `cmp()` → `Ordering`             |
| 비교 불가능한 경우 | `None` 반환 가능                         | 항상 비교 가능해야 함             |
| 대표 타입       | `f64`, `f32`, `Option<T>`                | `i32`, `String`, 튜플 등          |
| 사용 가능 위치   | `Vec.sort_by()`                          | `BTreeSet`, `BTreeMap`, `Vec.sort()` |


## 🔍 실수 방지 팁
- Ord를 구현하려면 반드시 PartialOrd, Eq, PartialEq도 함께 구현함.
- PartialOrd만 구현하고 Ord를 생략하면 정렬 자료구조에서 사용할 수 없음.
- `NaN` 이 포함될 수 있는 타입은 `Ord` 를 구현하면 논리 오류가 발생할 수 있음.

## 🧩 1. Ord와 PartialOrd 함께 구현하는 예제
Rust에서는 `Ord` 를 구현할 때 반드시 `PartialOrd`, `Eq`, `PartialEq` 도 함께 구현해야 함.  
아래는 나이 기준으로 정렬되는 Person 타입 예제입니다:
```rust
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}
```
```rust
impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}
```
```rust
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
- Ord는 반드시 Ordering을 반환해야 하며, PartialOrd는 Option<Ordering>을 반환합니다.


## 🧠 2. NaN이 포함된 타입을 안전하게 다루는 트릭
f64는 NaN 때문에 Ord를 구현할 수 없음.    
하지만 `NaN` 을 제거하거나 무시하는 래퍼 타입을 만들면 BTreeMap, BTreeSet에서도 사용할 수 있어요.
### ✅ 예제: NoNaN 래퍼 타입
```rust
#[derive(Debug, PartialEq)]
struct NoNaN(f64);
```
```rust
impl Eq for NoNaN {}
```
```rust
impl Ord for NoNaN {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
```
```rust
impl PartialOrd for NoNaN {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
```
- unwrap_or(Ordering::Equal)로 NaN을 무시하거나, panic!으로 처리할 수도 있음.


## 🧩 3. PartialOrd를 활용한 사용자 정의 타입 예제
PartialOrd만 구현해서 일부 값만 비교 가능한 구조를 만들 수 있음.  
예를 들어 Option<T>에서 None은 비교 불가능하게 처리:
```rust
#[derive(Debug)]
enum MaybeNumber {
    Value(f64),
    Unknown,
}
```
```rust
impl PartialEq for MaybeNumber {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (MaybeNumber::Unknown, MaybeNumber::Unknown))
    }
}
```
```rust
impl PartialOrd for MaybeNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (MaybeNumber::Value(a), MaybeNumber::Value(b)) => a.partial_cmp(b),
            _ => None, // Unknown은 비교 불가
        }
    }
}
```

## 🧨 4. Ord와 PartialOrd 충돌 디버깅 팁
### ❗ 흔한 오류
- Ord::cmp()와 PartialOrd::partial_cmp()의 결과가 불일치할 경우, 정렬 자료구조에서 예상치 못한 동작 발생
- PartialOrd에서 None을 반환할 수 있는데, Ord는 항상 Ordering을 반환해야 함

## ✅ 디버깅 팁 – Ord vs PartialOrd
| 증상/상황     | 원인 및 해결 방법                          |
|---------------|---------------------------------------------|
| `BTreeSet`에서 중복 처리 이상 | `Ord::cmp()`가 잘못된 `Equal` 반환 → 중복 판단 오류 |
| `sort()`에서 예외 발생       | `Ord`와 `PartialOrd` 구현 불일치 → 정렬 충돌 가능     |
| `NaN` 포함 시 panic 발생     | `partial_cmp()` → `None` 반환, `unwrap()` 실패        |

### 해결: 
Ord는 항상 Ordering을 반환해야 하며, PartialOrd와 논리적으로 일치하도록 구현해야 함.


## ✅ 요약 – Ord vs PartialOrd 구현과 디버깅
| 항목             | 설명 및 트릭                                |
|------------------|---------------------------------------------|
| `Ord`, `PartialOrd` | `cmp()` → `Ordering`, `partial_cmp()` → `Option<Ordering>` |
| `NaN` 처리        | `unwrap_or()` 또는 `panic!()`로 안전하게 대응 |
| `PartialOrd` 활용 | `Option`, `enum` 등 일부 값만 비교 가능한 구조 |
| 구현 시 주의      | `Ord`와 `PartialOrd`는 논리적으로 일치해야 함 |

---

# 🧠 왜 NaN이 문제일까?
Rust의 f64는 PartialOrd만 구현하고 Ord는 구현하지 않아요. 이유는 NaN 때문:
```rust
let x = std::f64::NAN;
let y = 1.0;
println!("{:?}", x.partial_cmp(&y)); // 출력: None
```
NaN은 자기 자신과도 비교할 수 없기 때문에 cmp()가 항상 Ordering을 반환해야 하는 Ord를 만족하지 못함.

## 🧩 NoNaN 래퍼 타입의 의미
```rust
struct NoNaN(f64);
```
이 타입은 f64를 감싸되, Ord를 구현하기 위해 partial_cmp()의 결과가 None일 경우 강제로 Ordering::Equal을 반환합니다:
```rust
impl Ord for NoNaN {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
```

### ✅ 이게 의미하는 것
- NaN이 들어와도 panic이 나지 않음
- NaN끼리 비교하면 Equal로 처리됨
- BTreeSet, BTreeMap 같은 정렬 자료구조에서 사용할 수 있음

### 🔍 "NaN이 제거되었다"는 표현의 정확한 의미
| 표현               | 실제 의미                                 |
|--------------------|--------------------------------------------|
| NaN 제거            | NaN이 들어와도 비교 연산에서 안전하게 처리됨 |
|                    | `unwrap_or(Ordering::Equal)`로 비교 실패 방지 |
|                    | `panic!()` 또는 생성자에서 NaN을 거를 수도 있음 |

### ✅ 더 안전하게 만들기: 생성자에서 NaN 거르기
```rust
impl NoNaN {
    pub fn new(val: f64) -> Option<Self> {
        if val.is_nan() {
            None
        } else {
            Some(NoNaN(val))
        }
    }
}
```
이렇게 하면 아예 NaN이 들어오지 못하게 막을 수 있음.

## 🧠 f64.partial_cmp()가 None을 반환하는 경우
```rust
let a = std::f64::NAN;
let b = 1.0;

println!("{:?}", a.partial_cmp(&b)); // 출력: None
```

### ✅ 이유:
- NaN은 IEEE 754 표준에 따라 어떤 값과도 비교할 수 없고, 자기 자신과도 같지 않음.
- 따라서 partial_cmp()는 None을 반환해서 **비교 불가능** 을 표현합니다.

## ❌ inf는 비교 가능
C++에서 inf는 특별한 값이지만, Rust에서는 f64::INFINITY나 f64::NEG_INFINITY도 정상적으로 비교됩니다:
```rust
let x = std::f64::INFINITY;
let y = 1000.0;
println!("{:?}", x.partial_cmp(&y)); // Some(Greater)
```
inf는 Ord를 구현하지 않지만, PartialOrd에서는 비교가 됩니다.

## ✅ 요약 – f64의 partial_cmp() 결과
| 값           | partial_cmp() 결과     | 비교 결과 예시       |
|--------------|------------------------|-----------------------|
| `NaN`        | `None`                 | 비교 불가능           |
| `INFINITY`   | `Some(Ordering)`       | `Greater`             |
| `-INFINITY`  | `Some(Ordering)`       | `Less`                |
| 일반 숫자    | `Some(Ordering)`       | `Less`, `Equal`, `Greater` |

## 💡 참고:
- `NaN` 은 `==`, `<`, `>` 등 모든 비교 연산에서 `false` 를 반환합니다.
- `NaN == NaN` 도 `false` 예요.


## 🧠 어떤 연산이 NaN을 만들어낼까?
아래는 Rust에서 f64 연산을 통해 NaN이 생성되는 대표적인 예시들이에요:
### ✅ 1. `0.0 / 0.0`
```rust
let x = 0.0 / 0.0;
println!("{}", x.is_nan()); // true
```
- 0을 0으로 나누면 수학적으로 정의되지 않기 때문에 `NaN` 이 됩니다.


### ✅ 2. sqrt() of negative number
```rust
let x = (-1.0f64).sqrt();
println!("{}", x.is_nan()); // true
```
- 실수 범위에서 음수의 제곱근은 존재하지 않기 때문에 NaN이 됩니다.


### ✅ 3. log() of negative number or zero
```rust
let x = (-5.0f64).ln(); // 자연로그
println!("{}", x.is_nan()); // true
```
- 로그 함수는 음수나 0에 대해 정의되지 않기 때문에 NaN이 됩니다.


### ✅ 4. inf - inf 또는 inf / inf
```rust
let x = std::f64::INFINITY - std::f64::INFINITY;
println!("{}", x.is_nan()); // true
```
- 무한대끼리의 연산도 정의되지 않으면 NaN이 됩니다.

### ✅ 요약: 연산으로 NaN이 나오는 경우

| 연산 예시             | 결과 | 설명                           |
|------------------------|------|--------------------------------|
| `0.0 / 0.0`            | NaN  | 정의되지 않은 나눗셈            |
| `(-1.0).sqrt()`        | NaN  | 실수에서 음수의 제곱근 없음     |
| `(-5.0).ln()`          | NaN  | 로그의 정의역 벗어남            |
| `INFINITY - INFINITY` | NaN  | 무한대 간의 연산 불가능         |

- 💡 즉, NaN은 단순히 상수로 지정하는 것뿐 아니라, 수학적으로 정의되지 않은 연산을 수행했을 때 자동으로 생성되는 값.

### 🧠 C++에서 NaN의 위험한 전파
### ✅ 예시: NaN이 계속 퍼지는 상황
```cpp
#include <cmath>
#include <iostream>

int main() {
    double x = sqrt(-1); // NaN
    double y = x + 100;  // 여전히 NaN
    double z = log(x);   // 여전히 NaN

    std::cout << std::isnan(z) << std::endl; // 출력: 1 (true)
}
```
NaN은 연산을 거쳐도 계속 NaN으로 남고, 어디서부터 잘못됐는지 추적하기 어려움.

### ⚠️ C++에는 None이 없다
Rust에서는 `Option<f64>` 처럼 값이 없음을 명시적으로 표현할 수 있음:
let x: Option<f64> = None;
C++에서는 이런 표현이 없기 때문에, NaN이 **값이 없는 상태** 처럼 동작하지만,  
실제로는 값이 있는 것처럼 취급됨. 이게 문제를 더 복잡하게 만듬.

## ✅ Rust의 안전 장치 – NaN 처리
| 상황/기능         | 관련 값/트레이트        | 설명                           |
|-------------------|--------------------------|--------------------------------|
| `NaN` 생성         | `f64`, `f32`             | 정의되지 않은 연산으로 발생       |
| 값의 유효성 표현   | `Option<T>`, `Result<T, E>` | 값이 없거나 오류를 명시적으로 표현 |
| 비교 연산         | `partial_cmp()` → `None` | `NaN`은 비교 불가능 → `None` 반환 |

### 💡 핵심 포인트:
- Rust는 NaN을 감지하고 None으로 표현하거나, Option/Result로 안전하게 감쌀 수 있음.
- C++처럼 NaN이 조용히 퍼지지 않고, Rust에서는 타입 시스템이 이를 명확하게 드러내줍니다.

### 🔍 결론
C++에서는 NaN이 숨어 있는 오류처럼 퍼질 수 있고, 디버깅이 어려움.  
Rust는 NaN을 감지하거나 Option으로 감싸서 명시적으로 처리할 수 있어서 더 안전한 방식.  

### C++ std::optional
C에도 이제 `std::optional` 이 생겨서 `Rust의 Option<T>` 처럼 값이 없을 수 있는 상황을 명시적으로 표현할 수 있게 됨.  
C17부터 도입된 기능인데, 예전엔 이런 걸 nullptr, sentinel, magic value로 처리하느라 오류가 많음.

### 🧩 C++의 std::optional vs Rust의 Option<T>
| 기능/표현            | `std::optional<T>` (C++)         | `Option<T>` (Rust)             |
|----------------------|----------------------------------|--------------------------------|
| 값 없음 표현         | `std::nullopt`                   | `None`                         |
| 값 있음 확인         | `has_value()`, `operator bool`   | `is_some()`                    |
| 값 꺼내기            | `value()`, `*opt`                | `unwrap()`, `match`, `if let` |
| 기본값 제공          | `value_or(default)`              | `unwrap_or(default)`          |
| 제어 흐름 통합       | ❌ 없음                           | `match`, `if let`, `?` 연산자 |


### ✅ 예제: C++에서 optional로 NaN 방지
```cpp
#include <optional>
#include <cmath>

std::optional<double> safe_sqrt(double x) {
    if (x < 0.0) return std::nullopt;
    return std::sqrt(x);
}
```
```cpp
auto result = safe_sqrt(-1.0);
if (result) {
    std::cout << "sqrt: " << *result << "\n";
} else {
    std::cout << "Invalid input\n";
}
```
Rust에서는 fn safe_sqrt(x: f64) -> Option<f64>로 거의 똑같이 표현할 수 있음.


## 🔍 장점
- 명시적 오류 처리: 값이 없음을 타입으로 표현
- 디버깅 쉬움: NaN처럼 숨어 있는 오류보다 훨씬 추적이 쉬움
- 표준화된 방식: C++도 이제 Rust처럼 함수형 스타일을 조금씩 받아들이는 중

---

