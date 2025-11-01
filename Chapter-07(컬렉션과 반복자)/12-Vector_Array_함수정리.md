# Vector / Array 함수 정리

## ✅ Vec / Array에서 자주 쓰는 함수들

| 함수명             | 설명                           | 예시 코드 예시                          |
|--------------------|--------------------------------|-----------------------------------------|
| `iter()`           | 불변 참조 순회                 | `vec.iter().for_each(\|x\| println!("{}", x));` |
| `iter_mut()`       | 가변 참조 순회                 | `vec.iter_mut().for_each(\|x\| *x *= 2);`       |
| `for_each()`       | 클로저 기반 순회               | `vec.iter().for_each(\|x\| println!("{}", x));` |
| `map()`            | 변환 후 수집                   | `vec.iter().map(\|x\| x * 2).collect::<Vec<_>>()` |
| `filter()`         | 조건에 맞는 요소 추출          | `vec.iter().filter(\|x\| *x > 2).collect::<Vec<_>>()` |
| `fold()`           | 누적 계산                      | `vec.iter().fold(0, \|acc, x\| acc + x)`         |
| `find()`           | 조건 만족 첫 요소 찾기         | `vec.iter().find(\|&&x\| x == 3)`                |
| `any()` / `all()`  | 조건 만족 여부 확인            | `vec.iter().any(\|&x\| x > 0)` / `all(\|&x\| x < 10)` |
| `contains()`       | 특정 값 포함 여부 확인         | `vec.contains(&3)`                             |
| `sort()` / `sort_by()` | 정렬                       | `vec.sort()` / `vec.sort_by(\|a, b\| b.cmp(a))`  |
| `push()` / `pop()` | 요소 추가 및 제거              | `vec.push(4)` / `vec.pop()`                    |
| `split_at()`       | 슬라이스 분할                  | `let (a, b) = vec.split_at(2)`                 |
| `chunks()`         | 고정 크기 블록 분할            | `for c in vec.chunks(2) { println!("{:?}", c); }` |
| `windows()`        | 슬라이딩 윈도우                | `for w in vec.windows(2) { println!("{:?}", w); }` |



## 🔧 샘플 코드 모음
### 1. Vec 순회 + for_each
```rust
fn main() {
    let data = vec![1, 2, 3];
    data.iter().for_each(|x| println!("Value: {}", x));
}
```

### 2. Vec 가변 순회 + iter_mut
```rust
fn main() {
    let mut data = vec![1, 2, 3];
    data.iter_mut().for_each(|x| *x *= 2);
    println!("{:?}", data); // [2, 4, 6]
}
```

### 3. Vec map + collect
```rust
fn main() {
    let data = vec![1, 2, 3];
    let squared: Vec<_> = data.iter().map(|x| x * x).collect();
    println!("{:?}", squared); // [1, 4, 9]
}
```


### 4. Vec filter
```rust
fn main() {
    let data = vec![1, 2, 3, 4];
    let even: Vec<_> = data.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even); // [2, 4]
}
```

### 5. Vec fold
```rust
fn main() {
    let data = vec![1, 2, 3];
    let sum = data.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum); // 6
}
```

### 6. Vec find
```rust
fn main() {
    let data = vec![1, 2, 3];
    if let Some(x) = data.iter().find(|&&x| x > 2) {
        println!("Found: {}", x);
    }
}
```

### 7. Vec sort
```rust
fn main() {
    let mut data = vec![3, 1, 2];
    data.sort();
    println!("{:?}", data); // [1, 2, 3]
}
```

### 8. Vec split_at
```rust
fn main() {
    let data = [1, 2, 3, 4];
    let (left, right) = data.split_at(2);
    println!("{:?} {:?}", left, right); // [1, 2] [3, 4]
}
```

### 9. Vec chunks
```rust
fn main() {
    let data = [1, 2, 3, 4, 5];
    for chunk in data.chunks(2) {
        println!("{:?}", chunk);
    }
}
```

## 💬 결론
Rust에서 가장 많이 쓰는 건 결국 Vec, array 입니다.
lifetime은 중요하지만, 데이터를 어떻게 다루는지가 본질입니다.

---


## ✅ Vec / Array에서 자주 쓰는 함수들 — 정리 & 샘플
### 1. iter() — 불변 순회
```rust
fn main() {
    let arr = [1, 2, 3];
    for x in arr.iter() {
        println!("Value: {}", x);
    }
}
```


### 2. iter_mut() — 가변 순회
```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    for x in vec.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", vec); // [2, 4, 6]
}
```


### 3. for_each() — 클로저 기반 순회
```rust
fn main() {
    let vec = vec![1, 2, 3];
    vec.iter().for_each(|x| println!("Value: {}", x));
}
```


### 4. map() — 변환 후 수집
```rust
fn main() {
    let vec = vec![1, 2, 3];
    let squared: Vec<_> = vec.iter().map(|x| x * x).collect();
    println!("{:?}", squared); // [1, 4, 9]
}
```


### 5. filter() — 조건 추출
```rust
fn main() {
    let vec = vec![1, 2, 3, 4];
    let even: Vec<_> = vec.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even); // [2, 4]
}
```


### 6. fold() — 누적 계산
```rust
fn main() {
    let vec = vec![1, 2, 3];
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum); // 6
}
```


### 7. find() — 조건 만족 첫 요소
```rust
fn main() {
    let vec = vec![1, 2, 3];
    if let Some(x) = vec.iter().find(|&&x| x > 2) {
        println!("Found: {}", x);
    }
}
```


### 8. any() / all() — 조건 만족 여부
```rust
fn main() {
    let vec = vec![1, 2, 3];
    println!("Any > 2: {}", vec.iter().any(|&x| x > 2)); // true
    println!("All > 0: {}", vec.iter().all(|&x| x > 0)); // true
}
```


### 9. contains() — 특정 값 포함 여부
```rust
fn main() {
    let vec = vec![1, 2, 3];
    println!("Contains 2? {}", vec.contains(&2)); // true
}
```


### 10. sort() / sort_by() — 정렬
```rust
fn main() {
    let mut vec = vec![3, 1, 2];
    vec.sort();
    println!("Sorted: {:?}", vec); // [1, 2, 3]

    vec.sort_by(|a, b| b.cmp(a));
    println!("Reversed: {:?}", vec); // [3, 2, 1]
}
```


### 11. push() / pop() — 추가 및 제거
```rust
fn main() {
    let mut vec = vec![1, 2];
    vec.push(3);
    println!("{:?}", vec); // [1, 2, 3]

    vec.pop();
    println!("{:?}", vec); // [1, 2]
}
```


### 12. split_at() — 슬라이스 분할
```rust
fn main() {
    let arr = [1, 2, 3, 4];
    let (left, right) = arr.split_at(2);
    println!("Left: {:?}, Right: {:?}", left, right); // [1, 2], [3, 4]
}
```


### 13. chunks() — 고정 크기 블록
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for chunk in arr.chunks(2) {
        println!("Chunk: {:?}", chunk);
    }
}
```


### 14. windows() — 슬라이딩 윈도우
```rust
fn main() {
    let arr = [1, 2, 3, 4];
    for window in arr.windows(2) {
        println!("Window: {:?}", window);
    }
}
```

---

# Vec 소유권 이동

Vec에서 구조체 같은 heap 데이터를 꺼내서 배열에 넣는 순간, **소유권 이동(move)** 이 발생합니다.  
특히 Copy 트레잇이 없는 타입이라면 더 확실함.

## 🔍 예시로 확인
```rust
#[derive(Debug)]
struct Person {
    name: String,
}

fn main() {
    let v = vec![
        Person { name: "Kim".to_string() },
        Person { name: "Lee".to_string() },
        Person { name: "Park".to_string() },
    ];

    // 소유권 이동 발생
    let a: [Person; 3] = [v[0], v[1], v[2]]; // ❌ 컴파일 에러
}
```

### ❌ 에러 이유
- Person은 Copy가 아니고, Clone도 안 했기 때문에 v[0], v[1], v[2]를 꺼내는 순간 move가 발생
- 하지만 Vec은 Index 연산이 &T를 반환하므로 v[0]은 사실 &Person이라서 복사도 안 되고, move도 안 됨

### ✅ 해결 방법

#### 1. clone() 사용
```rust
let a: [Person; 3] = [v[0].clone(), v[1].clone(), v[2].clone()];
```

### 2. into_iter()로 소유권 넘기기
```rust
let v = vec![
    Person { name: "Kim".to_string() },
    Person { name: "Lee".to_string() },
    Person { name: "Park".to_string() },
];

let mut iter = v.into_iter();
let a: [Person; 3] = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
```

---

# &&x, &x, 없는 경우 정리

Rust에서 &&x, &x, 혹은 아무 것도 안 붙는 x는 iterator의 타입과 클로저의 매개변수 패턴이 어떻게 맞물리는지에 따라 달라집니다.  
filter, find, any, all처럼 비슷해 보이는 메서드들이 실제로는 매개변수 타입에 따라 다르게 동작하기 때문에,  
어떤 경우에 &x, &&x가 필요한지 구조적으로 정리.

## 🧩 핵심: iter()는 참조를 반환한다
```rust
let vec = vec![1, 2, 3];
for x in vec.iter() {
    // x는 &i32
}
```
- vec.iter() → Iterator<Item = &i32>
- 즉, x는 &i32 타입이므로
클로저에서 |x|는 &i32, |&x|는 i32, |&&x|는 &i32의 참조

## 🔍 비교: find vs filter
### ✅ find: 클로저가 FnMut(&T) -> bool
```rust
→ Item = &i32이므로 |x|는 &i32, |&x|는 i32
vec.iter().find(|&&x| x > 2)
// x: &&i32 → x는 &i32 → &&x는 i32
```


- &&x는 x: &&i32 → *x: &i32 → **x: i32
- 그래서 x > 2가 성립

### ✅ filter: 클로저가 FnMut(&T) -> bool
```rust
→ Item = &i32이므로 |x| x > &2도 가능
vec.iter().filter(|x| **x > 2)
// 또는
vec.iter().filter(|&x| x > 2)
```

- filter는 Iterator를 반환하므로
이후 .collect() 등으로 결과를 모아야 함
- filter는 find처럼 바로 값을 반환하지 않기 때문에
&&x를 쓸 일이 적음

## ✅ any, all: 클로저가 FnMut(&T) -> bool
```rust
vec.iter().any(|&x| x > 2)
// x: &i32 → |&x| x: i32


- |&x|는 x: i32로 언패킹
- |x|는 x: &i32로 그대로 사용
```
---

# Rust 역 참조 정리

Rust와 C의 참조와 역참조 개념이 겹치면서 생기는 혼란,  
정리해드리면 Rust에서 &&x는 문법적으로 x가 &&T일 때 **x와 같은 의미로 쓰일 수 있습니다.  
즉, &&x는 패턴 매칭에서 “이중 참조를 언패킹”하는 방식이고,  
C의 **x와 비슷한 역할을 하지만 의미와 위치가 다릅니다.

## 🧩 Rust에서 &&x는 어떻게 동작하는가?

### ✅ 상황: vec.iter()는 Iterator<Item = &i32>
```rust
vec.iter().find(|&&x| x > 2)
```

- vec.iter() → Item = &i32
- find()는 FnMut(&T) -> bool → &T = &&i32
- |&&x|는 x: i32로 언패킹하는 패턴
즉, |&&x|는 x: &&i32 → *x: &i32 → **x: i32와 같은 효과

## 🔍 C++과 비교: 참조 vs 역참조

| 개념 구분       | Rust 표현       | C++ 표현        | 설명                                      |
|----------------|------------------|------------------|-------------------------------------------|
| 단일 참조       | `&x`             | `&x`             | 변수 `x`에 대한 참조                      |
| 이중 참조       | `&&x`            | `&(&x)`          | 참조의 참조 (Rust에서는 패턴 언패킹에 사용) |
| 단일 역참조     | `*x`             | `*x`             | 참조를 따라가서 값에 접근                 |
| 이중 역참조     | `**x`            | `**x`            | 이중 참조를 두 번 따라가서 값에 접근       |
| 패턴 언패킹 1단 | `\|&x\|`           | 없음             | `x: &T` → `x: T`로 언패킹                  |
| 패턴 언패킹 2단 | `\|&&x\|`          | 없음             | `x: &&T` → `x: T`로 언패킹                 |

→ Rust의 |&&x|는 패턴 매칭에서 이중 참조를 언패킹하는 문법이고,
C++에서는 이런 식의 패턴 매칭이 없기 때문에 직접 *x로 접근해야 함

### ✅ Rust에서 &&x는 **x처럼 동작하지만, 위치가 다름
- &&x는 패턴 매칭에서 쓰는 문법  
    → |&&x|는 x: i32로 바로 언패킹됨
- **x는 표현식에서 쓰는 역참조 연산자  
    → let val = **x;

즉, &&x는 패턴에서 역참조를 구조적으로 표현하는 방식이고, C++처럼 직접 * 연산자를 쓰는 건 Rust에서는 표현식에서만 사용됨

---
# | |의 의미

## 🧩 | |는 단순한 인자 리스트가 아니라 “패턴 자리”다
### ✅ 예: 단순 인자
```rust
let add = |x, y| x + y;
```

- 여기서 x, y는 그냥 값 → 언패킹 없음
### ✅ 예: 참조 언패킹
```rust
let vec = vec![1, 2, 3];
vec.iter().find(|&&x| x > 2);
```

- vec.iter() → Item = &i32
- find()는 FnMut(&T) → &T = &&i32
- |&&x|는 x: i32로 언패킹하는 패턴 매칭

## 🔍 Rust의 `| |`는 “패턴 매칭 가능한 인자 자리”
| 표현         | 인자 타입        | 언패킹 후 타입 | 설명                      |
|--------------|------------------|----------------|---------------------------|
| `\|x\|`         | `&i32`           | `&i32`         | 참조 그대로 받음           |
| `\|&x\|`        | `&i32`           | `i32`          | 참조를 한 번 언패킹        |
| `\|&&x\|`       | `&&i32`          | `i32`          | 이중 참조를 두 번 언패킹   |
| `\|(a, b)\|`    | `(T1, T2)`       | `T1`, `T2`     | 튜플을 분해하여 각각 추출  |
| `\|Some(x)\|`   | `Option<T>`      | `T`            | 열거형 매칭 후 내부 추출   |

→ 즉, | |는 단순한 인자 리스트가 아니라 패턴 매칭을 통해 구조를 분해하고 언패킹하는 자리입니다

람다 안에서 |x|처럼 단순하게 받아서 표현식에서 직접 역참조하거나 구조를 해석하는 방식은  
복잡한 패턴 매칭을 피하고 명시적으로 처리할 수 있는 실용적인 전략입니다.

## 🧩 예시: 복잡한 패턴 대신 표현식에서 처리
### ✅ 패턴 매칭 방식
``` rust
vec.iter().find(|&&x| x > 2);
```

### ✅ 표현식 처리 방식
```rust
vec.iter().find(|x| **x > 2);
```

- x: &&i32를 그대로 받고
- 표현식에서 **x로 직접 역참조

## 🔍 언제 이렇게 쓰면 좋은가?
| 표현        | 상황 및 용도 설명                                      |
|-------------|--------------------------------------------------------|
| `\|&x\|`       | `x: &T`일 때 참조를 한 번 언패킹하여 값으로 비교할 때 사용 |
| `\|&&x\|`      | `x: &&T`일 때 이중 참조를 언패킹해야 할 때 사용           |
| `\|x\|`        | 참조 그대로 받아서 표현식에서 직접 `*x`, `**x`로 처리할 때 |
| `\|x\| dbg!(x)`| 디버깅이나 타입 확인이 필요할 때, 구조를 직접 확인하고 싶을 때 |


→ 결국은 패턴 매칭은 구조를 드러내는 방식,  
표현식 처리는 제어를 명시적으로 하는 방식입니다

---
## 🧩 C++ vs Rust: 참조/포인터 철학 차이

| 개념 구분       | C++ 표현        | Rust 표현        | 의미 및 설명                                 |
|----------------|------------------|-------------------|-----------------------------------------------|
| 단일 참조       | `int& x`         | `&i32`            | C++: 참조 변수 선언<br>Rust: 값에 대한 참조   |
| 포인터          | `int* x`         | `*const i32`      | C++: 포인터 변수<br>Rust: unsafe 포인터 타입 |
| 이중 포인터/참조| `int** x`        | `&&i32`           | C++: 포인터의 포인터<br>Rust: 참조의 참조     |
| 역참조          | `*x`, `**x`      | `*x`, `**x`       | C++/Rust 모두 참조나 포인터를 따라가서 값 접근 |
| 패턴 언패킹 1단 | 없음             | `|&x|`            | Rust: `x: &T` → `x: T`로 언패킹                |
| 패턴 언패킹 2단 | 없음             | `|&&x|`           | Rust: `x: &&T` → `x: T`로 언패킹               |

## 🔍 Rust는 “참조를 구조로 다루고”, C++은 “포인터를 연산으로 다룬다”
- C++: int* p = &x; *p = 10; → 포인터는 값처럼 쓰고 연산으로 해석
- Rust: let r: &i32 = &x; → 참조는 소유권과 생명주기를 따름  
    → Rust는 메모리 안전성과 구조적 제어를 위해

참조를 언어의 핵심 구조로 설계했고, C++은 퍼포먼스와 유연성을 위해 포인터를 연산의 일부로 설계

## ✅ 그래서 Rust에서는 “참조를 언패킹”하고, C++에서는 “포인터를 역참조”한다
- Rust: |&x| → x: i32로 구조적으로 언패킹
- C++: *x → x가 포인터일 때 값에 접근
    → 같은 *x라도 Rust에서는 안전한 참조 해석, C++에서는 위험한 포인터 연산이 될 수 있음

---

