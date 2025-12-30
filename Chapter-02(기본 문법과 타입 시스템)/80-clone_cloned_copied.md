# clone / cloned/ copied

- Rust에서 iter().clone(), cloned(), copied()는  모두 **이터레이터(iterator)** 를 다룰 때 사용하는 메서드들이며,    
    각각의 차이점은 소유권, 복사 가능성, 성능과 관련이 있습니다.  
- 아래에 각각을 비교해서 설명.

## 🌀 iter().clone()
- 의미: 이터레이터 자체를 복제합니다.
- 즉, 이터레이터를 두 개 만들고 싶을 때 사용합니다.
- 예시:
```rust
let v = vec![1, 2, 3];
let iter1 = v.iter();
let iter2 = iter1.clone(); // iter1과 같은 이터레이터 복제
```
- 조건: 이터레이터 타입이 Clone을 구현하고 있어야 합니다.
- 용도: 동일한 이터레이터를 여러 번 순회하고 싶을 때.

## 🧬 cloned()
- 의미: 이터레이터가 참조(&T)를 반환할 때, 각 요소를 .clone()해서 소유권을 가진 값(T)으로 바꿔줍니다.
- 예시:
```rust
let v = vec![String::from("hi"), String::from("bye")];
for s in v.iter().cloned() {
    println!("{}", s); // s는 String (소유권 있음)
}
```
- 조건: 요소 타입 T가 Clone을 구현해야 합니다.
- 용도: 참조된 값을 복제해서 소유권을 얻고 싶을 때.

## 📋 copied()
- 의미: cloned()와 비슷하지만, Copy 트레잇을 구현한 타입에만 사용 가능하며, .clone() 대신 .copy()를 수행합니다.
- 예시:
```rust
let v = vec![1, 2, 3];
for n in v.iter().copied() {
    println!("{}", n); // n은 i32 (Copy 타입)
}
```
- 조건: 요소 타입 T가 Copy를 구현해야 함 (예: 정수, 부동소수점, bool 등).
- 장점: cloned()보다 성능이 더 좋음 (heap 할당 없음).

## 🔍 요약 비교표
| 메서드           | 반환 타입 | 요구 조건      | 사용 목적                          |
|------------------|-----------|----------------|-----------------------------------|
| `iter().clone()` | 이터레이터 | `Clone`        | 이터레이터 자체를 복제             |
| `cloned()`       | `T`       | `T: Clone`     | 참조된 값을 복제하여 소유권 획득   |
| `copied()`       | `T`       | `T: Copy`      | 참조된 값을 복사하여 소유권 획득   |

- 필요한 상황에 따라 적절한 메서드를 선택하면 됩니다.  
- 예를 들어 i32, bool 같은 단순 타입이면 copied()가 가장 효율적이고, String 같은 복잡한 타입이면 cloned()를 써야 함.


## ✅ clone().collect 기본 개념
```rust
let v = vec![1, 2, 3];
let collected = v.iter().clone().collect::<Vec<_>>();
```
- 이 코드는 다음과 같은 과정을 거칩니다:
    - v.iter() → &v의 이터레이터를 생성 (&i32 타입 요소)
    - .clone() → 이터레이터 자체를 복제 (즉, v.iter()를 복사해서 또 하나 만듦)
    - .collect() → 복제된 이터레이터를 수집해서 Vec<&i32>로 만듦
    - 즉, iter().clone().collect()는 `Vec<&T>` 를 생성합니다. `clone()` 은 `이터레이터 자체를 복제` 할 뿐, `요소를 복제하지 않습니다`.

## 🔍 예시로 비교
```rust
let v = vec![10, 20, 30];
```
```rust
// iter().clone().collect()
let a = v.iter().clone().collect::<Vec<_>>(); // Vec<&i32>
```
```rust
// iter().cloned().collect()
let b = v.iter().cloned().collect::<Vec<_>>(); // Vec<i32>
```
```rust
// iter().copied().collect()
let c = v.iter().copied().collect::<Vec<_>>(); // Vec<i32>
```

- a는 &i32 참조들의 벡터
- b, c는 i32 값들의 벡터 (소유권 있음)

### ⚠️ 주의할 점
- iter().clone()은 이터레이터를 복제하는 것이지, 요소를 복제(clone)하는 것과는 다릅니다.
- 요소를 복제하고 싶다면 .cloned() 또는 .copied()를 사용해야 합니다.

- 필요하다면 .collect()의 타입 추론을 명시적으로 지정해주는 것도 좋습니다.
- 예를 들어:
```rust
let refs: Vec<&i32> = v.iter().clone().collect();
```

## 🔍 self.ctrl.iter().cloned().collect() 에러 핵심 원인 요약
```rust
pub ctrl: Vec<Point4D>,
let mut cp : Vec<Point4D> = self.ctrl.iter().cloned().collect();
```

- self.ctrl의 타입: Vec<Point4D>
- iter()의 타입: IntoIter<Point4D> → Item = Point4D
- .cloned()은 Iterator<Item = &T>만 지원 → 여기선 Item = Point4D라서 불가능

## ✅ 해결 방법
### 🔧 1. iter() → iter().cloned() (참조 기반 복사)
```rust
let mut cp: Vec<Point4D> = self.ctrl.iter().cloned().collect();
```
- 이건 self.ctrl: Vec<&Point4D>일 때만 동작합니다.
- 즉, Vec<Point4D>가 아니라 Vec<&Point4D>일 때만 .cloned() 가능

### 🔧 2. self.ctrl.clone() 또는 .to_vec() 사용 (가장 안전)
```rust
let mut cp: Vec<Point4D> = self.ctrl.clone();
```

#### 또는
```rust
let mut cp: Vec<Point4D> = self.ctrl.to_vec();
```
- 이건 Vec<Point4D>를 전체 복제하는 방식
- Point4D가 Clone을 구현하고 있어야 함

### ✅ 추천 방식
```rust
let mut cp = self.ctrl.clone();
```

- 가장 간단하고 안전하며, 타입 오류 없이 동작합니다
- Vec<T>는 Clone만 구현되어 있으면 .clone()으로 전체 복제 가능

## ✍️ 결론
- .cloned()는 참조를 값으로 복제할 때만 사용 가능하고,
- Vec<Point4D>처럼 값 자체를 반복하는 경우엔 **.clone() 또는 .to_vec()** 을 써야 합니다.

---

# cloned가 동작하는 조건

## ✅ 먼저: cloned()가 동작하는 조건
```rust
let v = vec![1, 2, 3];
let copied: Vec<i32> = v.iter().cloned().collect();
```

- v.iter() → Iterator<Item = &i32>
- .cloned() → &i32 → i32로 복제
- i32는 Copy + Clone이므로 .cloned()가 잘 동작함

## ❌ 그런데 왜 Point4D는 안 되냐?
```rust
let ctrl: Vec<Point4D> = ...;
let cp: Vec<Point4D> = ctrl.iter().cloned().collect(); // ❌ 에러 발생
```

- ctrl.iter() → Iterator<Item = &Point4D>
- .cloned() → &Point4D → Point4D로 복제 시도
- 그런데 Point4D가 Clone을 구현하지 않았거나,
- 혹은 iter()가 값이 아닌 참조를 반환하지 않는 경우
    - .cloned()는 컴파일 에러 발생
- 문법은 맞지만 타입 추론을 못하는 경우도 있음

## 🔍 핵심 차이 요약
| 표현                          | iter()의 Item 타입 | .cloned() 가능 여부 | 비고                        |
|-------------------------------|---------------------|----------------------|-----------------------------|
| `vec![1, 2, 3].iter()`        | `&i32`              | ✅ 가능              | `i32`는 `Copy + Clone`      |
| `vec![Point4D].iter()`        | `&Point4D`          | ✅ 가능              | `Point4D`가 `Clone` 필요     |
| `vec![Point4D].into_iter()`   | `Point4D`           | ❌ 불가능            | `.cloned()`는 `&T`만 지원   |
| `vec![Point4D].iter().cloned()` | `Point4D` (❌ 오류) | ❌ 불가능            | `iter()`가 참조를 반환해야 함 |


## ✅ 해결 방법
- Point4D에 #[derive(Clone)] 추가
#[derive(Clone)]
struct Point4D {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

- 복제 방식 변경
```rust
let cp = self.ctrl.clone(); // 가장 안전하고 직관적
```

### 또는
```rust
let cp: Vec<Point4D> = self.ctrl.iter().map(|p| p.clone()).collect();
```

## ✍️ 결론
- .cloned()는 참조 타입을 값으로 복제할 때만 동작하며, 그 값이 Clone을 구현하고 있어야 합니다.
- Point4D가 Clone을 구현하지 않았거나, iter()가 참조를 반환하지 않으면 에러가 납니다.
- 문법적으로 다 맞더라고 타입 추론을 못해서 안되는 경우도 있습니다.

---




