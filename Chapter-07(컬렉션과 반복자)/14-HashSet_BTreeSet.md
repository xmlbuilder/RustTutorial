# HashSet / BTreeSet
Rust에도 C++의 std::set처럼 중복 없는 집합(Set) 구조가 있습니다.  
Rust에서는 크게 두 가지 Set 타입이 있어요:

## 🧩 Rust의 Set 구조
| 타입         | 특징                            |
|--------------|----------------------------------|
| HashSet<T>   | 빠른 검색, 해시 기반, 순서 없음     |
| BTreeSet<T>  | 자동 정렬, 트리 기반, 순서 있음     |



## ✅ HashSet 예제
```rust
use std::collections::HashSet;
fn main() {
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("apple"); // 중복 무시

    for item in &set {
        println!("{}", item);
    }
}
```
- 출력 순서는 보장되지 않아요.


## ✅ BTreeSet 예제
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    for item in &set {
        println!("{}", item); // 출력: 1, 2, 3
    }
}
```


## 🧩 HashSet – 빠른 중복 제거와 검색
```rust
use std::collections::HashSet;

fn main() {
    let mut colors = HashSet::new();
    colors.insert("red");
    colors.insert("blue");
    colors.insert("red"); // 중복은 무시됨
    println!("{:?}", colors); // {"blue", "red"}

    if colors.contains("red") {
        println!("red가 포함되어 있습니다.");
    }
    colors.remove("red");
}
```

## 🔧 자주 쓰는 함수 – HashSet / BTreeSet 공통
| 함수명           | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| `insert()`        | 요소를 추가합니다. 이미 존재하면 무시됩니다.                         |
| `contains()`      | 특정 값이 집합에 존재하는지 확인합니다.                              |
| `remove()`        | 특정 값을 제거합니다. 존재하지 않으면 아무 일도 일어나지 않습니다.   |
| `len()`           | 집합의 요소 개수를 반환합니다.                                       |
| `is_empty()`      | 집합이 비어 있는지 여부를 반환합니다.                                |
| `clear()`         | 모든 요소를 제거합니다.                                              |
| `iter()`          | 집합의 모든 요소에 대해 반복자를 반환합니다.                         |
| `retain()`        | 조건을 만족하는 요소만 유지합니다. (`|x| x % 2 == 0` 등)             |
| `union()`         | 두 집합의 합집합을 반환합니다. (`A ∪ B`)                             |
| `intersection()`  | 두 집합의 교집합을 반환합니다. (`A ∩ B`)                             |
| `difference()`    | 한 집합에서 다른 집합을 뺀 차집합을 반환합니다. (`A - B`)            |


## 🧩 HashSet 자주 쓰는 함수 예제 모음
### 1. insert() – 값 추가
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    println!("{:?}", set); // {"apple", "banana"}
}
```


### 2. contains() – 값 존재 여부 확인
```rust
fn main() {
    let set: HashSet<_> = ["apple", "banana"].into_iter().collect();
    if set.contains("apple") {
        println!("사과가 있어요!");
    }
}
```

### 3. remove() – 값 제거
```rust
fn main() {
    let mut set: HashSet<_> = ["apple", "banana"].into_iter().collect();
    set.remove("banana");
    println!("{:?}", set); // {"apple"}
}
```


### 4. iter() – 반복자 사용
```rust
fn main() {
    let set: HashSet<_> = ["apple", "banana"].into_iter().collect();
    for item in &set {
        println!("과일: {item}");
    }
}
```


### 5. len() – 요소 개수 확인
```rust
fn main() {
    let set: HashSet<_> = ["apple", "banana"].into_iter().collect();
    println!("총 {}개", set.len()); // 2
}
```

#### 🔍 코드 분석
```rust
use std::collections::HashSet;
fn main() {
    let set: HashSet<_> = ["apple", "banana"].into_iter().collect();
    println!("총 {}개", set.len()); // 2
}
```

#### ✅ "apple"과 "banana"는 어떤 타입인가?
- 이 문자열 리터럴은 &'static str 타입입니다.
- 'static lifetime은 프로그램 전체 생존 기간을 의미하므로, HashSet<&'static str>로 안전하게 들어갈 수 있어요.
- 즉, lifetime 문제가 발생하지 않습니다.

#### 🧠 왜 String으로 바꾸지 않아도 되는가?
- String은 힙에 저장되는 가변 문자열이고, &str은 불변 참조입니다.
- 위 코드에서는 &str 타입을 그대로 HashSet에 넣고 있기 때문에 복사도 없고, 소유권도 문제 없음.
- 만약 String을 넣고 싶다면 명시적으로 변환해야 해요:
```rust
let set: HashSet<String> = ["apple", "banana"]
    .into_iter()
    .map(|s| s.to_string())
    .collect();
```

#### ✅ 요약 – HashSet에 문자열 넣을 때의 타입 흐름

| 표현                        | 타입 추론 결과             | 설명                                |
|-----------------------------|-----------------------------|-------------------------------------|
| `["apple", "banana"]`       | `&'static str`              | 문자열 리터럴, 프로그램 전체 생존   |
| `.into_iter().collect()`    | `HashSet<&'static str>`     | 참조를 그대로 집합에 저장           |
| `.map(|s| s.to_string())`   | `HashSet<String>`           | 힙에 복사된 `String`을 저장         |

#### 🔍 예시: 문제가 생기는 코드
```rust
use std::collections::HashSet;
fn make_set() -> HashSet<&String> {
    let name = String::from("JungHwan");
    let mut set = HashSet::new();
    set.insert(&name); // ❌ name은 함수 끝나면 drop됨
    set
}
```
- 이 코드는 컴파일되지 않아요. name의 참조가 set에 들어가지만, name은 함수가 끝나면 사라지기 때문.

#### ✅ 해결 방법: 참조 대신 소유권을 넘기기
```rust
fn make_set() -> HashSet<String> {
    let name = String::from("JungHwan");
    let mut set = HashSet::new();
    set.insert(name); // ✅ name의 소유권을 넘김
    set
}
```
- 이 방식은 HashSet<String>이 String을 소유하므로 lifetime 문제가 없습니다.

#### 💡 요약 – HashSet과 소유권 / 참조 / lifetime
| 표현            | 의미 및 특징                                      |
|-----------------|--------------------------------------------------|
| `HashSet<&T>`    | 참조를 저장함 → lifetime 명시 필요, dangling 위험 있음 |
| `HashSet<T>`     | 값을 직접 소유함 → 안전, lifetime 걱정 없음         |
| `'static`        | 프로그램 전체 생존 기간 → 참조로 넣어도 안전         |

#### 🔍 핵심 포인트
- HashSet<&T>는 참조를 저장하므로, 원본 데이터가 살아 있는 동안만 유효해야 해요.
- 지역 변수의 참조를 넣으면 lifetime 오류 발생 가능.
- HashSet<T>는 값 자체를 소유하므로, lifetime 문제 없이 안전하게 사용 가능.
- 'static 참조 (&'static str 등)는 언제나 유효하므로 HashSet<&'static str>도 안전.




### 6. clear() – 전체 제거
```rust
fn main() {
    let mut set: HashSet<_> = ["apple", "banana"].into_iter().collect();
    set.clear();
    println!("비었나요? {}", set.is_empty()); // true
}


### 7. is_empty() – 비어 있는지 확인
```rust
fn main() {
    let set: HashSet<String> = HashSet::new();
    if set.is_empty() {
        println!("아무것도 없어요!");
    }
}
```


### 8. retain() – 조건에 맞는 값만 유지
```rust
fn main() {
    let mut set: HashSet<_> = [1, 2, 3, 4, 5].into_iter().collect();
    set.retain(|x| x % 2 == 0);
    println!("{:?}", set); // {2, 4}
}
```


### 9. union() – 합집합
```rust
fn main() {
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4, 5].into_iter().collect();
    let union: HashSet<_> = a.union(&b).cloned().collect();
    println!("{:?}", union); // {1, 2, 3, 4, 5}
}
```


### 10. intersection() – 교집합
```rust
fn main() {
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4, 5].into_iter().collect();
    let inter: HashSet<_> = a.intersection(&b).cloned().collect();
    println!("{:?}", inter); // {3}
}
```

### 11. difference() – 차집합
```rust
fn main() {
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4, 5].into_iter().collect();
    let diff: HashSet<_> = a.difference(&b).cloned().collect();
    println!("{:?}", diff); // {1, 2}
}
```
----

# 🌳 BTreeSet – 정렬된 집합
```rust
use std::collections::BTreeSet;
fn main() {
    let mut books = BTreeSet::new();
    books.insert("The Odyssey");
    books.insert("To Kill a Mockingbird");
    books.insert("The Great Gatsby");

    for book in &books {
        println!("{book}"); // 정렬된 순서로 출력됨
    }

    if !books.contains("1984") {
        println!("1984는 없습니다.");
    }

    books.remove("The Odyssey");
}
```

## 🔧 자주 쓰는 함수 – HashSet / BTreeSet
| 함수명        | 설명 및 사용 예시                          |
|---------------|---------------------------------------------|
| `insert()`     | 요소 추가: `set.insert("apple")`            |
| `contains()`   | 포함 여부 확인: `set.contains("apple")`     |
| `remove()`     | 요소 제거: `set.remove("apple")`            |
| `iter()`       | 반복자 반환: `for item in &set { ... }`     |
| `len()`        | 요소 개수: `set.len()`                      |
| `range()`      | 범위 검색 (`BTreeSet` 전용): `set.range("a".."z")` |
| `clear()`      | 전체 제거: `set.clear()`                    |
| `is_empty()`   | 비어 있는지 확인: `set.is_empty()`          |


## 🌳 BTreeSet 자주 쓰는 함수 예제 모음
### 1. insert() – 값 추가
```rust
use std::collections::BTreeSet;
fn main() {
    let mut set = BTreeSet::new();
    set.insert("apple");
    set.insert("banana");
    println!("{:?}", set); // {"apple", "banana"} (정렬됨)
}
```

### 2. contains() – 값 존재 여부 확인
```rust
fn main() {
    let set: BTreeSet<_> = ["apple", "banana"].into_iter().collect();
    if set.contains("banana") {
        println!("바나나가 있어요!");
    }
}
```


### 3. remove() – 값 제거
```rust
fn main() {
    let mut set: BTreeSet<_> = ["apple", "banana"].into_iter().collect();
    set.remove("apple");
    println!("{:?}", set); // {"banana"}
}
```


### 4. iter() – 정렬된 반복자
```rust
fn main() {
    let set: BTreeSet<_> = ["cherry", "apple", "banana"].into_iter().collect();
    for item in &set {
        println!("과일: {item}"); // apple, banana, cherry
    }
}
```


### 5. len() – 요소 개수 확인
```rust
fn main() {
    let set: BTreeSet<_> = ["apple", "banana"].into_iter().collect();
    println!("총 {}개", set.len()); // 2
}
```

### 6. range() – 범위 검색
```rust
fn main() {
    let set: BTreeSet<_> = ["apple", "banana", "cherry", "date"].into_iter().collect();
    for item in set.range("banana".."d") {
        println!("범위 내 과일: {item}"); // banana, cherry
    }
}
```

#### 🧠 핵심 개념: range(start..end)는 end를 포함하지 않음
```rust
use std::collections::BTreeSet;

fn main() {
    let set: BTreeSet<_> = ["banana", "carrot", "date", "egg"].into_iter().collect();

    for item in set.range("banana".."d") {
        println!("{item}");
    }
}
```
#### 출력 결과:
```
banana
carrot
```

- "date"는 "d"보다 사전순으로 뒤에 있으므로 포함되지 않음
- "d"는 포함되지 않는 상한값이기 때문에 "date"는 걸리지 않음

#### ✅ 포함시키려면 어떻게?
##### 1. range("banana"..="date") → "date"까지 포함
```rust
for item in set.range("banana"..="date") {
    println!("{item}");
}
```

##### 출력: 
```
banana, carrot, date
```

##### 2. range("banana".."e") → "egg" 직전까지 포함
- "date"는 "e"보다 앞이므로 포함됨

#### 💡 요약 – BTreeSet::range()와 "date" 포함 여부
| 범위 표현              | 상한값 포함 여부 | "date" 포함 여부 |
|------------------------|------------------|------------------|
| `"banana".."d"`        | ❌ 포함 안 됨     | ❌ 포함 안 됨     |
| `"banana"..="d"`       | ✅ 포함됨         | ❌ 포함 안 됨     |
| `"banana"..="date"`    | ✅ 포함됨         | ✅ 포함됨         |
| `"banana".."e"`        | ❌ 포함 안 됨     | ✅ 포함됨         |

HashSet은 정렬되지 않기 때문에 range()를 사용할 수 없어요.

### 7. clear() – 전체 제거
```rust
fn main() {
    let mut set: BTreeSet<_> = ["apple", "banana"].into_iter().collect();
    set.clear();
    println!("비었나요? {}", set.is_empty()); // true
}
```


### 8. is_empty() – 비어 있는지 확인
```rust
fn main() {
    let set: BTreeSet<String> = BTreeSet::new();
    if set.is_empty() {
        println!("아무것도 없어요!");
    }
}

```

### 9. split_off() – 기준값 이후의 요소들을 분리
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set: BTreeSet<_> = [10, 20, 30, 40, 50].into_iter().collect();
    let split = set.split_off(&30);

    println!("원래 집합: {:?}", set);   // {10, 20, 30}
    println!("분리된 집합: {:?}", split); // {40, 50}
}
```
기준값(30)은 원래 집합에 남고, 그보다 큰 값들이 새 집합으로 분리.

#### 🧠 핵심 개념: split_off()는 소유권을 나눔
```rust
let mut set: BTreeSet<_> = [10, 20, 30, 40, 50].into_iter().collect();
let split = set.split_off(&30);
```

- set: {10, 20, 30}만 남음
- split: {40, 50}를 새로운 소유권으로 가짐
- 기준값 30은 원래 집합에 남고, 그보다 큰 값들이 새 집합으로 이동

#### ✅ 소유권이 분리된다는 의미
- split은 새로운 BTreeSet 인스턴스로, 기존 set과는 별개로 동작합니다.
- 두 집합은 서로 독립적인 메모리 공간을 가지며, 이후 각각 수정 가능.
- Rust의 소유권 시스템에 따라 split은 set에서 값을 이동시킨 결과이므로, 복사가 아닌 **이동(move)** 입니다.

#### 🔍 예시로 확인
```rust
let mut set: BTreeSet<_> = [1, 2, 3, 4, 5].into_iter().collect();
let split = set.split_off(&3);

println!("원래: {:?}", set);  // {1, 2, 3}
println!("분리된: {:?}", split); // {4, 5}
```
set과 split은 이후 각각 독립적으로 insert(), remove() 등 사용 가능.


#### 💡 요약:
- split_off()는 기준값을 기준으로 소유권을 나누는 연산입니다.
- 원래 집합과 분리된 집합은 서로 다른 BTreeSet 인스턴스로 동작합니다.

#### 🧩 BTreeSet 합치기 – 방법 2가지
##### ✅ 방법 1: append() 사용 (가장 간단하고 효율적)
```rust
use std::collections::BTreeSet;
fn main() {
    let mut a: BTreeSet<_> = [1, 2, 3].into_iter().collect();
    let mut b: BTreeSet<_> = [4, 5].into_iter().collect();

    a.append(&mut b); // b의 모든 요소를 a로 이동

    println!("합쳐진 a: {:?}", a); // {1, 2, 3, 4, 5}
    println!("b는 비었는가? {}", b.is_empty()); // true
}
```
append()는 소유권을 이동시키며, 중복은 자동 제거됩니다.


##### ✅ 방법 2: union() + cloned().collect() (새로운 집합 생성)
```rust
fn main() {
    let a: BTreeSet<_> = [1, 2, 3].into_iter().collect();
    let b: BTreeSet<_> = [3, 4, 5].into_iter().collect();

    let merged: BTreeSet<_> = a.union(&b).cloned().collect();

    println!("합쳐진 집합: {:?}", merged); // {1, 2, 3, 4, 5}
}
```
union()은 반복자를 반환하므로, cloned().collect()로 새 집합을 만들어야 해요.

#### ✅ 요약 – BTreeSet 합치기 방법
| 목적               | 방법               | 결과 설명           |
|--------------------|--------------------|----------------------|
| 원본에 다른 집합 추가 | `a.append(&mut b)` | `a`에 `b`를 병합, `b`는 비워짐 |
| 새 집합으로 합치기   | `a.union(&b)`       | `a`와 `b`는 유지, `ab` 새로 생성 |


### 10. first() – 가장 작은 값 가져오기
```rust
fn main() {
    let set: BTreeSet<_> = [5, 3, 8, 1].into_iter().collect();
    if let Some(first) = set.first() {
        println!("가장 작은 값: {}", first); // 1
    }
}
```
자동 정렬 덕분에 first()는 항상 가장 작은 값을 반환합니다.


### 11. last() – 가장 큰 값 가져오기
```
fn main() {
    let set: BTreeSet<_> = [5, 3, 8, 1].into_iter().collect();
    if let Some(last) = set.last() {
        println!("가장 큰 값: {}", last); // 8
    }
}
```
last()는 정렬된 집합에서 가장 큰 값을 빠르게 가져올 수 있어요.

### ✅ 요약 – BTreeSet 고급 함수
| 함수명          | 반환값 / 동작 설명         |
|------------------|----------------------------|
| `split_off(&key)`| 기준값 이후의 요소들을 분리 |
| `first()`        | `Option<&T>` – 가장 작은 값 |
| `last()`         | `Option<&T>` – 가장 큰 값   |


## ✅ 선택 기준 요약 – HashSet vs BTreeSet

| 기준/상황               | HashSet                          | BTreeSet                         |
|--------------------------|----------------------------------|----------------------------------|
| 정렬 필요 여부           | ❌ 순서 없음                      | ✅ 자동 정렬                      |
| 검색 속도                | ✅ 매우 빠름 (해시 기반)          | ⚠️ 느림 (트리 기반)              |
| 반복 순서                | ❌ 예측 불가                      | ✅ 정렬된 순서                    |
| 범위 검색 (`range()`)    | ❌ 지원 안 함                     | ✅ 지원                           |
| 메모리 사용량            | ✅ 상대적으로 적음                | ⚠️ 더 많을 수 있음               |
| `Ord` 요구 여부          | ❌ 필요 없음 (`Eq + Hash`)        | ✅ 필요함 (`Ord`)                 |
| 집합 연산 (`union`, 등)  | ✅ 지원                           | ✅ 지원                           |


---

# 소유권 쪼개기

## ✅ 왜 쪼개고 합치는 방식이 좋은가?
### 1. 소유권 충돌 방지
- 하나의 컬렉션을 동시에 여러 곳에서 수정하려면 빌림 규칙에 자주 걸림.
- split_off()로 나누면 각 부분이 독립된 소유권을 가지므로, 자유롭게 수정 가능.
### 2. 병렬 처리에 유리
- 나눈 집합들을 스레드별로 분산 처리한 뒤, append()나 union()으로 다시 병합할 수 있음.
- 예: 대용량 로그를 시간대별로 나눠서 처리한 뒤 다시 합치기
### 3. 정렬된 데이터 유지
- BTreeSet은 정렬된 상태를 유지하므로, 합쳐도 순서가 깨지지 않음.
- append()는 내부적으로 효율적인 병합을 수행합니다.

## 🧠 예시 흐름
```rust
let mut full: BTreeSet<_> = (1..=10).collect();
let mut later = full.split_off(&6); // full: 1~6, later: 7~10

// 각 집합을 독립적으로 수정
full.remove(&2);
later.insert(11);

// 다시 합치기
full.append(&mut later); // full: 1,3,4,5,6,7,8,9,10,11

```

## 💡 팁: split_off() + append()는 정렬된 병합에 최적화
- split_off()는 O(log n)
- append()는 O(n)보다 빠르게 병합 가능 (내부적으로 최적화됨)

결론적으로, 소유권 문제를 우회하면서도 안전성과 성능을 챙길 수 있는 전략.  
Rust의 철학에 맞는 “명시적이고 안전한” 방식이죠.

---
