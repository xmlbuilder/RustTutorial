
# 🦀 Rust에서 정렬 기준 지정 방법
Rust의 BTreeMap과 BTreeSet은 정렬된 자료구조입니다.  
이들은 내부적으로 Ord 트레이트를 기반으로 정렬 순서를 결정합니다.

## ✅ 기본 정렬 기준
Rust의 기본 타입들 (i32, String, 튜플 등)은 이미 Ord 트레이트를 구현하고 있어서 자동으로 정렬됩니다.
```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(3);
set.insert(1);
set.insert(2);
for v in &set {
    println!("{}", v); // 출력: 1, 2, 3
}
```


## 🧩 사용자 정의 타입에 정렬 기준 지정하기
### 1. Ord, PartialOrd, Eq, PartialEq 트레이트 구현
```rust
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}
```
```rust
impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age) // 나이 기준 정렬
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
let mut people = BTreeSet::new();
people.insert(Person { name: "Alice".into(), age: 30 });
people.insert(Person { name: "Bob".into(), age: 25 });

for p in &people {
    println!("{} ({})", p.name, p.age); // Bob (25), Alice (30)
}
```
Rust는 Ord를 구현할 때 반드시 Eq, PartialEq, PartialOrd도 함께 구현해야 함.


## 🔁 커스텀 정렬이 필요할 때
BTreeSet은 정렬 기준을 바꿀 수 없지만, Vec을 사용하고 정렬할 수 있음:
```rust
let mut people = vec![
    Person { name: "Alice".into(), age: 30 },
    Person { name: "Bob".into(), age: 25 },
];

people.sort_by(|a, b| b.age.cmp(&a.age)); // 나이 내림차순

for p in &people {
    println!("{} ({})", p.name, p.age); // Alice (30), Bob (25)
}
```

## ✨ 요약 – C++ vs Rust 정렬 기준

| C++ (`std::map`, `std::set`) | Rust (`BTreeMap`, `BTreeSet`)         |
|------------------------------|----------------------------------------|
| `<` 연산자 오버로드           | `Ord`, `PartialOrd` 트레이트 구현      |
|                              | `cmp()` 메서드로 비교 로직 정의        |
|                              | `BTree*`은 고정 정렬, `Vec + sort_by`로 커스텀 정렬 가능 |

---

# 커스텀 타입 키
Rust의 BTreeMap에서 커스텀 타입을 키로 사용하려면,  
해당 타입이 Ord, PartialOrd, Eq, PartialEq 트레이트를 구현해야 함.  
그리고 정렬 기준을 바꾸는 트릭도 몇 가지 있음.

## 🧩 1. BTreeMap에서 커스텀 타입을 키로 쓰기
```rust
use std::collections::BTreeMap;
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
    let mut map = BTreeMap::new();

    map.insert(Person { name: "Alice".into(), age: 30 }, "Engineer");
    map.insert(Person { name: "Bob".into(), age: 25 }, "Designer");

    for (person, job) in &map {
        println!("{} ({}) → {}", person.name, person.age, job);
    }
}
```
출력은 나이 순으로 정렬돼서 Bob → Alice 순서가 됩니다.


## 🧠 2. 정렬 기준을 바꾸는 트릭
Rust의 BTreeMap은 정렬 기준을 런타임에 바꿀 수는 없지만, 다음과 같은 트릭으로 다양한 정렬을 흉내낼 수 있음:
### ✅ 방법 A: 정렬 기준을 반대로 (내림차순)
```rust
#[derive(Debug, Eq, PartialEq)]
struct Descending(u32);
```
```rust
impl Ord for Descending {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0) // 내림차순
    }
}
```
```rust
impl PartialOrd for Descending {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
```rust
impl PartialEq for Descending {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
```
```rust
impl Eq for Descending {}
```
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(Descending(10), "ten");
    map.insert(Descending(5), "five");
    map.insert(Descending(20), "twenty");

    for (k, v) in &map {
        println!("{} → {}", k.0, v);
    }
}
```

#### 출력: 
```
20 → 10 → 5
```

### ✅ 방법 B: 튜플로 정렬 우선순위 지정

```rust
use std::collections::BTreeMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
```
```rust
fn main() {
    let mut map = BTreeMap::new();

    let p1 = Person { name: "Alice".into(), age: 30 };
    let p2 = Person { name: "Bob".into(), age: 25 };
    let p3 = Person { name: "Alice".into(), age: 20 };

    // (정렬 기준, 원본 객체)
    map.insert((p1.name.clone(), p1.age), p1);
    map.insert((p2.name.clone(), p2.age), p2);
    map.insert((p3.name.clone(), p3.age), p3);

    for ((name, age), person) in &map {
        println!("{} ({})", name, age);
    }
}
```
이름 → 나이 순으로 정렬됨

## ✅ 요약 – Rust에서 정렬 기준 지정 방법

| 목적                     | 방법                                      |
|--------------------------|-------------------------------------------|
| 커스텀 타입 키 사용       | `Ord`, `PartialOrd`, `Eq`, `PartialEq` 구현 |
| 비교 로직 정의           | `Ord::cmp()` 메서드에서 기준 지정         |
| 복합 기준 정렬           | `(String, u32)` 같은 튜플을 키로 사용      |
| 동적 정렬 기준           | `Vec` + `sort_by()`로 정렬 후 사용         |

## 🧠 PartialOrd vs Ord 차이점

| 트레이트       | 특징 및 의미                                      |
|----------------|--------------------------------------------------|
| `PartialOrd`   | 부분 비교 가능 → `a <= b`는 가능하지만 `b <= a`가 항상 보장되진 않음 |
| `Ord`          | 전체 순서 정의 → 모든 값 쌍에 대해 `a <= b`와 `b <= a`가 비교 가능 |


## 🔍 핵심 차이점
### ✅ PartialOrd (부분 순서)
- partial_cmp() 메서드를 구현해야 함 → Option<Ordering> 반환
- 비교 불가능한 경우 None 반환 가능
- 예: f32, f64은 NaN 때문에 Ord를 구현할 수 없고, PartialOrd만 구현
```rust
let a = std::f64::NAN;
let b = 1.0;
assert_eq!(a.partial_cmp(&b), None); // 비교 불가
```
### ✅ Ord (전순서)
- cmp() 메서드를 구현해야 함 → 항상 Ordering (Less, Equal, Greater) 반환
- 모든 값 쌍이 비교 가능해야 함
- BTreeMap, BTreeSet은 내부 정렬을 위해 반드시 Ord가 필요
```rust
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct MyKey(i32); // i32는 항상 비교 가능하므로 OK
```


### 📌 예시: 왜 f64은 Ord를 구현하지 않나?
```rust
use std::collections::BTreeSet;
let mut set = BTreeSet::new();
set.insert(1.0_f64);
set.insert(std::f64::NAN); // ❌ 컴파일 에러: f64는 Ord를 구현하지 않음
```
- NaN은 자기 자신과도 비교할 수 없기 때문에 Ord를 만족하지 못함.


## ✅ 요약 – PartialOrd vs Ord

| 구분             | PartialOrd                          | Ord                          |
|------------------|--------------------------------------|------------------------------|
| 비교 메서드       | `partial_cmp()` → `Option<Ordering>` | `cmp()` → `Ordering`         |
| 비교 불가능한 경우 | `None` 반환 가능                     | 항상 비교 가능해야 함         |
| 대표 타입         | `f64`, `f32`                         | 정수, 문자열, 튜플 등         |

---

# double을 key로 쓰기

C++에서 double을 std::map이나 std::set의 키로 쓸 때, < 연산자에 Tol을 넣어 근사 비교를 하는 경우가 있음.  
Rust에서는 f64가 Ord를 구현하지 않기 때문에 BTreeMap의 키로 직접 사용할 수 없지만,  
정렬 가능한 래퍼 타입을 만들어서 Tol을 적용한 비교를 구현할 수 있어요.

## 🧩 목표: f64를 키로 쓰되, Tol 기반 비교로 정렬
```rust
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64Key {
    pub value: f64,
    pub tol: f64, // 허용 오차
}
```
```rust
impl Eq for F64Key {}
```
```rust
impl Ord for F64Key {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.value - other.value;
        if diff.abs() < self.tol {
            Ordering::Equal
        } else if diff < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
```
```rust
impl PartialOrd for F64Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

### ✅ 사용 예시
```rust
fn main() {
    let tol = 1e-6;
    let mut map = BTreeMap::new();

    map.insert(F64Key { value: 1.000001, tol }, "A");
    map.insert(F64Key { value: 1.000002, tol }, "B"); // tol 내에서 같으므로 덮어씀

    for (k, v) in &map {
        println!("{:.6} → {}", k.value, v);
    }
}
```

#### 출력: 
```
1.000001 → B (두 값이 tol 내에서 같으므로 하나만 저장됨)
```

### 🧠 확장 아이디어
- tol을 const로 고정하거나, impl F64Key에 생성자 추가 가능
- HashMap에서도 비슷한 방식으로 Hash 트레이트를 커스터마이징 가능
- Vec에서 sort_by()로 근사 정렬도 가능

## ✅ 요약 – f64를 근사 비교 키로 쓰는 방법

| 대상      | 구현 방식                         |
|-----------|----------------------------------|
| `f64`     | `Ord` 구현한 래퍼 타입 `F64Key`   |
| 비교 기준 | `cmp()`에서 `abs(diff) < tol`     |
| 정렬 처리 | `Ordering::Equal`로 중복 판단      |


## HashMap 사용 예
- HashMap<F64Key, T>: 근사 비교를 위한 해시 키 사용
- Vec<F64Key>: sort_by()로 tol 기반 정렬

### 🧩 1. HashMap<F64Key, T> 예제
Rust의 HashMap은 `Eq` 와 `Hash` 트레이트를 요구하므로, F64Key에 `Hash` 구현이 필요합니다.  
tol 내에서 같은 값은 동일한 해시로 처리되도록 구현합니다.
```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64Key {
    pub value: f64,
    pub tol: f64,
}
```
```rust
impl Eq for F64Key {}
```
```rust
impl Hash for F64Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // tol 내에서 반올림된 값으로 해시
        let rounded = (self.value / self.tol).round() as i64;
        rounded.hash(state);
    }
}
```
```rust
fn main() {
    let tol = 1e-3;
    let mut map = HashMap::new();

    map.insert(F64Key { value: 1.0001, tol }, "A");
    map.insert(F64Key { value: 1.0002, tol }, "B"); // 같은 키로 간주됨

    for (k, v) in &map {
        println!("{:.4} → {}", k.value, v); // 출력: 1.0001 → B
    }
}
```
1.0001과 1.0002는 tol=1e-3 내에서 같으므로 하나의 키로 처리됨.

### 🧩 2. Vec<F64Key> 정렬 예제
```rust
fn main() {
    let tol = 1e-4;
    let mut vec = vec![
        F64Key { value: 1.0002, tol },
        F64Key { value: 1.0001, tol },
        F64Key { value: 0.9999, tol },
    ];

    vec.sort_by(|a, b| {
        let diff = a.value - b.value;
        if diff.abs() < a.tol {
            std::cmp::Ordering::Equal
        } else if diff < 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    for v in &vec {
        println!("{:.4}", v.value);
    }
}
```

### 출력: 
```
0.9999, 1.0002, 1.0001 (정렬 기준은 tol에 따라 다르게 적용 가능)
```

## ✅ 요약 – f64 근사 비교 키/정렬 in Rust

| 용도     | 구현 방식                          |
|----------|------------------------------------|
| HashMap  | `Hash` + `Eq` 구현, `tol` 기반 해시 |
| Vec      | `sort_by()`에서 `abs(diff) < tol` 비교 |

---

# fn hash<H: Hasher>(&self, state: &mut H) 구조 설명

## 🧠 해시에 영향을 주는 이유
```rust
impl Hash for F64Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // tol 내에서 반올림된 값으로 해시
        let rounded = (self.value / self.tol).round() as i64;
        rounded.hash(state);
    }
}
```

이 줄은 self.value를 tol로 나눈 뒤 반올림해서 정수로 바꿉니다. 즉:
- value = 1.01, tol = 0.1 → 1.01 / 0.1 = 10.1 → round() = 10 → 해시값: 10
- value = 1.04, tol = 0.1 → 1.04 / 0.1 = 10.4 → round() = 10 → 해시값: 10
- value = 1.49, tol = 0.1 → 1.49 / 0.1 = 14.9 → round() = 15 → 해시값: 15  
즉, value가 tol 내에서 조금씩 달라도 같은 해시값을 가지게 됩니다.


## ✅ 왜 이렇게 하는가?
이런 방식은 부동소수점 값의 근사치를 해시로 묶기 위한 전략.  
Rust에서 f64는 Hash를 직접 구현하지 않기 때문에,
- NaN, -0.0, 0.0 같은 값들이 해시 충돌을 일으킬 수 있고
- 아주 미세한 차이로 해시가 달라지는 걸 방지하고 싶을 때  
    → tol을 기준으로 "같은 범위"에 있는 값들을 같은 해시로 묶는 것임.

### 🔍 예시: HashSet<F64Key>에서 근사값 중복 제거
```rust
let mut set = HashSet::new();
set.insert(F64Key { value: 1.01, tol: 0.1 });
set.insert(F64Key { value: 1.04, tol: 0.1 }); // 같은 해시 → 중복으로 간주됨
```
- 이 경우 두 값은 Hash와 Eq가 동일하게 작동하면 중복으로 처리됨.

## 💡 요약 – Hash 구현 흐름

| 단계                  | 설명                                      |
|-----------------------|-------------------------------------------|
| `value / tol`         | 기준 단위로 정규화                        |
| `.round() as i64`     | 근사값으로 정수화                         |
| `rounded.hash(state)` | 해시 상태에 반영 (Hasher 내부 상태 변경)  |
| 결과 해시             | `value`와 `tol` 조합으로 결정됨           |

- 이건 Rust에서 부동소수점 기반 키를 안전하게 해시하는 고급 패턴.
- hash() 함수 안에서 state가 바뀌는 이유는, 우리가 직접 값을 바꾸는 게 아니라, Hasher가 내부적으로 상태를 누적하기 때문.

## 🧠 hash() 함수의 역할
```rust
fn hash<H: Hasher>(&self, state: &mut H) {
    let rounded = (self.value / self.tol).round() as i64;
    rounded.hash(state);
}
```

여기서 일어나는 일:
- rounded는 i64 값입니다. (self.value / self.tol을 반올림한 정수)
- rounded.hash(state)는 내부적으로 Hasher에 이 값을 피드합니다.
- Hasher는 이 값을 받아서 내부 상태를 업데이트합니다.
- 이 상태는 나중에 finish()를 호출할 때 최종 해시값으로 변환됩니다.

### 🔍 핵심: Hasher는 내부 상태를 갖고 있다
- Hasher는 state라는 이름으로 전달되지만, 사실은 해시 누적기예요.
- 우리가 state를 직접 바꾸는 게 아니라, rounded.hash(state)가 내부적으로 state.write(...)를 호출해서 해시 상태를 갱신합니다.
- 이건 마치 println!()이 화면에 출력되지만, 우리가 직접 stdout을 조작하지 않는 것과 비슷한 개념.

### ✅ 예시로 이해해보기
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let mut hasher = DefaultHasher::new();
    42.hash(&mut hasher); // 내부적으로 hasher.write(...) 호출
    let result = hasher.finish(); // 최종 해시값 추출
    println!("해시값: {}", result);
}
```
여기서도 42.hash(&mut hasher)는 값을 리턴하지 않지만,  
hasher의 내부 상태는 변화했고, finish()로 그 결과를 꺼낼 수 있음.

## 💡 요약 – Hash 트레이트와 Hasher 흐름
| 표현              | 의미 및 동작                            |
|-------------------|------------------------------------------|
| `state`           | `Hasher`의 mutable 참조, 해시 상태 저장기 |
| `.hash(state)`    | 내부적으로 `state.write(...)` 호출        |
| `state.finish()`  | 누적된 상태를 기반으로 최종 해시값 생성   |
| `state`           | 값은 직접 안 바뀌지만 내부 상태는 변화함  |

- 겉으로는 변화가 없지만, state는 내부적으로 계속 누적되고 있는 것임.

---

## 🧠 Rust vs C++/Java – 해시 방식의 근본적 차이
| 항목                  | C++ / Java                          | Rust                                |
|-----------------------|--------------------------------------|-------------------------------------|
| 해시 구현 방식         | `hashCode()` / `std::hash`           | `Hash` 트레이트 구현 필요            |
| 비교 연산과의 연계     | `equals()`와 `hashCode()` 분리        | `Eq` + `Hash` 함께 구현 필수         |
| 부동소수점 지원        | `float`, `double` 해시 가능 (NaN 포함) | `f32`, `f64`는 `Hash` 미구현 (직접 구현 필요) |
| 해시 기반 컬렉션       | `HashMap`, `HashSet` 등              | `HashMap`, `HashSet` (Eq + Hash 필요) |
| 해시 알고리즘          | JVM/라이브러리마다 다름               | `SipHash` 기본 (보안 중심)           |
| 해시 대상 제약         | 거의 모든 객체 가능                   | `Hash` 트레이트 구현된 타입만 가능   |

## 🔍 Rust의 해시 설계 특징
### 1. 트레이트 기반
- Hash는 트레이트로 구현해야 하며, Hasher에 값을 피드하는 방식
- 해시 결과는 Hasher.finish()로 추출

### 2. 보안 중심 해시
- 기본 HashMap은 SipHash를 사용 → 해시 충돌 공격에 강함
- C++/Java는 성능 중심 해시가 많음 → 보안은 개발자 책임

### 3. 부동소수점은 기본 해시 불가
- f64, f32는 Hash 미구현 → NaN, -0.0 등 문제 방지
- 커스텀 키(F64Key)로 Eq + Hash를 직접 구현해야 함

### ✅ 예시: Rust에서 해시 구현은 이렇게 다름
```rust
use std::hash::{Hash, Hasher};

struct F64Key {
    value: f64,
    tol: f64,
}
```
```rust
impl Hash for F64Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let rounded = (self.value / self.tol).round() as i64;
        rounded.hash(state);
    }
}
```
C++/Java에서는 그냥 value를 해시하면 끝이지만,  
Rust에서는 정확한 비교 기준과 해시 전략을 명시적으로 설계해야 함.

## 💡 요약
- Rust는 해시를 안전성과 일관성 중심으로 설계함
- C++/Java는 해시를 편의성과 성능 중심으로 설계함
- Rust에서는 해시와 비교(Eq)가 항상 함께 작동해야 하며, 개발자가 의도적으로 설계해야만 안전하게 동작합니다

---

