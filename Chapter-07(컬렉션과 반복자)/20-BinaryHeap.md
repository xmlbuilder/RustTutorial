# BinaryHeap
Rust에서 알아두면 좋은 자료구조는 `BinaryHeap` 있음.   
특히 BinaryHeap은 `우선순위 큐` 로 매우 중요합니다.

## 🧠 Rust에서 추가로 알아두면 좋은 자료구조
| 자료구조            | 주요 메서드 또는 특징               |
|---------------------|-------------------------------------|
| BinaryHeap<T>       | pop() → 가장 큰 값 반환             |


## 🧪 실전 예제 몇 가지
### ✔ BinaryHeap: 우선순위 큐
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(5);
    heap.push(1);

    while let Some(top) = heap.pop() {
        println!("가장 큰 값: {}", top);
    }
}
```
### ✔ Rc + RefCell: 다중 소유 + 내부 가변성
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    a.borrow_mut().push(4);
    b.borrow_mut().push(5);

    println!("{:?}", data.borrow()); // [1, 2, 3, 4, 5]
}
```


### ✔ Box: 재귀 구조체
```rust
enum List {
    Node(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
}
```


### ✔ Cow: 읽기 전용 → 쓰기 시 복사
```rust
use std::borrow::Cow;

fn main() {
    let s: Cow<str> = Cow::Borrowed("hello");

    let mut owned = s.into_owned(); // 필요 시 복사
    owned.push_str(" world");

    println!("{}", owned); // "hello world"
}
```


## ✅ 요약
- `BinaryHeap`: 우선순위 큐가 필요할 때
- `Rc + RefCell`: 다중 소유 + 내부 가변성
- `Box`: 힙에 저장하거나 재귀 타입 처리
- `Cow`: 읽기 전용 → 쓰기 시 복사
- `Option/Result`: 안전한 null 처리와 에러 핸들링

---

# 왜 사용 하는가
Rust에서 우선순위 큐를 구현할 때 사용하는 자료구조로,  
가장 큰 값(또는 가장 작은 값)을 빠르게 꺼낼 수 있어 효율적인 작업 스케줄링, 그래프 알고리즘 등에 매우 유용합니다.

## 🧠 왜 BinaryHeap을 쓰는가?
- 우선순위 큐 구현에 최적화: pop() 시 가장 큰 값을 O(log n) 시간에 꺼낼 수 있음
- 정렬된 값의 빠른 접근: peek()으로 가장 큰 값을 확인 가능
- 그래프 알고리즘에 필수: Dijkstra, A* 등에서 핵심 자료구조
- 표준 라이브러리에서 제공: std::collections::BinaryHeap

## 🛠️ 주요 메서드 요약
| 메서드       | 설명                                      |
|--------------|-------------------------------------------|
| `new()`        | 빈 힙 생성                                 |
| `push(x)`      | 요소 추가                                  |
| `pop()`        | 가장 큰 값 제거 및 반환                    |
| `peek()`       | 가장 큰 값 참조 반환 (`Option<&T>`)        |
| `len()`        | 요소 개수 반환                             |
| `is_empty()`   | 비어 있는지 확인                           |
| `clear()`      | 모든 요소 제거                             |
| `from([...])`  | 초기값으로 힙 생성                         |



## 🧪 각 메서드별 예제
### ✔ `new()` + `push()`
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(10);
    heap.push(5);
    heap.push(20);
    println!("{:?}", heap); // 내부 순서는 보장되지 않음
}
```
### ✔ `peek()` → 가장 큰 값 확인
```rust
let mut heap = BinaryHeap::from([3, 1, 4]);
println!("최상위 값: {:?}", heap.peek()); // Some(&4)
```


### ✔ `pop()` → 가장 큰 값 꺼내기
```rust
let mut heap = BinaryHeap::from([3, 1, 4]);
while let Some(top) = heap.pop() {
    println!("꺼낸 값: {}", top); // 4 → 3 → 1
}
```


### ✔ `len()` / `is_empty()`
```rust
let heap = BinaryHeap::from([1, 2, 3]);
println!("길이: {}", heap.len());       // 3
println!("비어 있음?: {}", heap.is_empty()); // false
```


### ✔ `clear()`
```rust
let mut heap = BinaryHeap::from([1, 2, 3]);
heap.clear();
println!("비어 있음?: {}", heap.is_empty()); // true
```


### ✔ from([...]) → 초기값으로 힙 생성
```rust
let heap = BinaryHeap::from([7, 2, 9]);
println!("최상위 값: {:?}", heap.peek()); // Some(&9)
```

### ✔ Min-Heap 만들기 (Reverse 사용)
```rust
use std::cmp::Reverse;

let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(10));
min_heap.push(Reverse(5));
min_heap.push(Reverse(20));

while let Some(Reverse(val)) = min_heap.pop() {
    println!("Min pop: {}", val); // 5 → 10 → 20
}
```

## ✅ 요약
- BinaryHeap은 우선순위 기반 작업에 최적화된 자료구조
- 기본은 Max-Heap, Reverse로 Min-Heap 구현 가능
- push, pop, peek로 빠른 삽입/삭제/조회 가능
- 그래프 알고리즘, 작업 스케줄링, 이벤트 처리 등에 매우 유용

---

# Reverse 

이 코드는 Rust의 BinaryHeap을 **최소 힙(min-heap)** 처럼 사용하기 위한 패턴인데,  
그 핵심은 Reverse 타입을 명시적으로 감싸서 넣는 것.  
아래에 단계적으로 원리를 설명.  

## 🧠 기본 개념: BinaryHeap은 기본적으로 최대 힙(max-heap)
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(10);
heap.push(5);
heap.push(20);

while let Some(val) = heap.pop() {
    println!("{}", val); // 출력: 20 → 10 → 5
}
```
- BinaryHeap은 기본적으로 큰 값이 먼저 나오는 최대 힙입니다
- 내부적으로 Ord를 기준으로 정렬되며, 큰 값이 우선순위가 높다고 판단

## ✅ 목표: 최소 힙으로 만들기
Rust에서는 BinaryHeap을 최소 힙으로 바꾸는 전용 옵션이 없기 때문에  
값을 Reverse로 감싸서 넣는 방식을 사용합니다.

## 🔍 단계별 설명
### 1️⃣ Reverse(10)은 무엇인가?
- Reverse<T>는 std::cmp::Reverse 타입
- Ord 구현이 기존과 반대 방향으로 작동함
- 즉, Reverse(5)는 Reverse(10)보다 작은 값으로 간주됨

### 2️⃣ 왜 감싸서 넣는가?
```rust
use std::cmp::Reverse;

min_heap.push(Reverse(10)); // 실제 저장: Reverse(10)
```
- BinaryHeap은 Ord를 기준으로 정렬하므로
- Reverse를 감싸면 작은 값이 우선순위가 높아짐
- 결과적으로 최소 힙처럼 작동

### 3️⃣ 왜 while let Some(Reverse(val)) = min_heap.pop()인가?
- pop()은 Reverse<T> 타입을 반환함
- Reverse(val)로 패턴 매칭해서 내부 값만 꺼냄

### 🧪 전체 흐름
```rust
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(10)); // 내부적으로 Reverse(10)
min_heap.push(Reverse(5));  // Reverse(5)
min_heap.push(Reverse(20)); // Reverse(20)

while let Some(Reverse(val)) = min_heap.pop() {
    println!("Min pop: {}", val); // 출력: 5 → 10 → 20
}
```

- Reverse로 감싸서 넣으면 작은 값이 먼저 나옴
- 꺼낼 때는 Reverse(val)로 감싸진 값을 해제해서 사용

## ✅ 요약
| 개념               | 설명                                       |
|--------------------|--------------------------------------------|
| BinaryHeap 기본     | 최대 힙 (큰 값이 먼저 나옴)                |
| Reverse<T>         | 정렬 방향을 반대로 바꾸는 래퍼 타입         |
| push(Reverse(x))   | 작은 값이 우선순위가 높게 작동함           |
| pop() → Reverse(x) | 꺼낼 때는 Reverse(val)로 패턴 매칭 필요     |

---

# 실전 응응

## 🧠 1. 그래프 알고리즘: Dijkstra 최단 경로
### ✔ 왜 BinaryHeap이 필요한가?
- Dijkstra 알고리즘은 가장 짧은 거리의 노드부터 처리해야 함
- BinaryHeap을 사용하면 우선순위 큐로 가장 작은 거리 노드를 빠르게 꺼낼 수 있음
- 기본은 Max-Heap이므로 Reverse를 사용해 Min-Heap으로 변환

### 🧪 예제: Dijkstra 알고리즘 (간단한 그래프)

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn dijkstra(graph: &HashMap<&str, Vec<(&str, u32)>>, start: &str) -> HashMap<&str, u32> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if let Some(neighbors) = graph.get(node) {
            for &(next, weight) in neighbors {
                let next_cost = cost + weight;
                if dist.get(next).map_or(true, |&c| next_cost < c) {
                    dist.insert(next, next_cost);
                    heap.push(Reverse((next_cost, next)));
                }
            }
        }
    }

    dist
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert("A", vec![("B", 1), ("C", 4)]);
    graph.insert("B", vec![("C", 2), ("D", 5)]);
    graph.insert("C", vec![("D", 1)]);
    graph.insert("D", vec![]);

    let result = dijkstra(&graph, "A");
    for (node, cost) in result {
        println!("A → {}: {}", node, cost);
    }
}
```

## 🧠 2. 이벤트 처리 시스템
### ✔ 왜 BinaryHeap이 필요한가?
- 이벤트는 우선순위 또는 시간 순서에 따라 처리되어야 함
- BinaryHeap을 사용하면 가장 빠른 이벤트를 효율적으로 꺼낼 수 있음
- Reverse를 사용하면 오름차순 시간 처리 가능

### 🧪 예제: 이벤트 스케줄러
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct Event {
    time: u32,
    name: &'static str,
}

fn main() {
    let mut schedule = BinaryHeap::new();

    schedule.push(Reverse(Event { time: 30, name: "Render Frame" }));
    schedule.push(Reverse(Event { time: 10, name: "Input Polling" }));
    schedule.push(Reverse(Event { time: 20, name: "Physics Update" }));

    while let Some(Reverse(event)) = schedule.pop() {
        println!("Time {} → Handling: {}", event.time, event.name);
    }
}
```

## ✅ 요약
| 분야           | 사용 목적                     | 핵심 메서드 및 전략         |
|----------------|-------------------------------|------------------------------|
| 그래프 알고리즘 | 최단 경로 탐색 (Dijkstra 등)   | push(), pop(), Reverse       |
| 이벤트 처리     | 시간 순서 또는 우선순위 처리   | push(), pop(), Reverse       |


---

# 부동 소수점 사용

BinaryHeap<f64> 또는 BinaryHeap<f32>처럼 부동소수점 타입(double) 을 사용할 수는 있지만,  
주의할 점이 있습니다.

## ⚠️ BinaryHeap<f64>은 기본적으로 동작하지 않음
Rust의 BinaryHeap<T>는 내부적으로 Ord 트레잇을 요구합니다.  
하지만 f64와 f32는 NaN(Not a Number) 때문에 Ord를 완전히 구현하지 않습니다.  
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(3.14); // ❌ 컴파일 오류: f64 doesn't implement Ord
}
```


## ✅ 해결 방법 1: OrderedFloat 사용
크레이트를 사용하면 f64를 안전하게 Ord로 감쌀 수 있습니다.
```
# Cargo.toml
ordered-float = "4"
```

```rust
use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(OrderedFloat(3.14));
    heap.push(OrderedFloat(2.71));
    heap.push(OrderedFloat(1.41));

    while let Some(val) = heap.pop() {
        println!("{}", val);
    }
}
```


## ✅ 해결 방법 2: unsafe
NaN이 없다는 걸 보장할 수 있다면 unsafe하게 강제 구현도 가능하지만, 권장되지 않습니다.  
OrderedFloat이 가장 안전하고 표준적인 방법입니다.

## ✅ 요약
| 항목                     | 설명 또는 키워드                      |
|--------------------------|----------------------------------------|
| BinaryHeap<f64> 불가 이유 | Ord 미구현 (`NaN` 비교 불가)           |
| 해결 방법                | ordered-float 크레이트의 OrderedFloat |
| 핵심 이슈                | NaN은 Ord 트레잇 위반                  |


## 실전 예제
아래는 OrderedFloat<f64>를 활용해 A 알고리즘에서 휴리스틱 비용이 실수일 때*  
우선순위 큐를 안전하게 사용하는 실전 Rust 예제입니다.

### 🧠 A* 알고리즘에서 왜 OrderedFloat이 필요한가?
- A*는 f(n) = g(n) + h(n) 형태의 실수 기반 휴리스틱 비용을 사용
- BinaryHeap은 Ord가 필요한데, f64는 NaN 때문에 Ord를 완전히 구현하지 않음
- OrderedFloat<f64>를 사용하면 NaN 없이 안전하게 정렬 가능

### 🧪 예제: A* 알고리즘 (간단한 그래프)
```rust
use std::collections::{BinaryHeap, HashMap};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq)]
struct Node<'a> {
    name: &'a str,
    cost: OrderedFloat<f64>, // f(n) = g(n) + h(n)
}

// A*는 우선순위 큐에서 가장 작은 f(n)를 먼저 꺼내야 하므로 Reverse 사용
impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Reverse: 작은 cost 우선
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // 그래프: 각 노드에 이웃과 거리
    let graph: HashMap<&str, Vec<(&str, f64)>> = HashMap::from([
        ("A", vec![("B", 1.0), ("C", 4.0)]),
        ("B", vec![("C", 2.0), ("D", 5.0)]),
        ("C", vec![("D", 1.0)]),
        ("D", vec![]),
    ]);

    // 휴리스틱: 각 노드에서 목표까지 예상 거리
    let heuristic: HashMap<&str, f64> = HashMap::from([
        ("A", 7.0),
        ("B", 6.0),
        ("C", 2.0),
        ("D", 0.0),
    ]);

    let mut open = BinaryHeap::new();
    let mut g_score: HashMap<&str, f64> = HashMap::new();

    g_score.insert("A", 0.0);
    open.push(Node {
        name: "A",
        cost: OrderedFloat(0.0 + heuristic["A"]),
    });

    while let Some(Node { name, .. }) = open.pop() {
        println!("확장 중: {}", name);
        if name == "D" {
            println!("목표 도달!");
            break;
        }

        let current_g = g_score[name];
        for &(neighbor, weight) in &graph[name] {
            let tentative_g = current_g + weight;
            let f = tentative_g + heuristic[neighbor];

            if g_score.get(neighbor).map_or(true, |&g| tentative_g < g) {
                g_score.insert(neighbor, tentative_g);
                open.push(Node {
                    name: neighbor,
                    cost: OrderedFloat(f),
                });
            }
        }
    }
}
```


## ✅ 요약
| 요소                  | 설명 또는 키워드                  |
|------------------------|----------------------------------|
| OrderedFloat<f64>      | Ord 구현으로 BinaryHeap에 사용 가능 |
| Reverse + cmp()        | Min-Heap처럼 작동하도록 정렬 반전   |
| f(n)                   | g(n) + h(n) 형태의 휴리스틱 비용   |

---

# HashSet, HashMap, BTreeSet, BTreeMap 사용
ordered-float의 OrderedFloat<T>는 Ord, Eq, Hash를 모두 구현하므로  
HashSet, HashMap, BTreeSet, BTreeMap 등 대부분의 컬렉션에서 문제 없이 사용할 수 있음.

## ✅ OrderedFloat<T> 사용 가능한 컬렉션
| 컬렉션 타입   | 사용 가능 여부 | 요구되는 트레잇         |
|---------------|----------------|--------------------------|
| HashSet       | ✅ 가능         | Hash + Eq                |
| HashMap       | ✅ 가능         | Hash + Eq                |
| BTreeSet      | ✅ 가능         | Ord                      |
| BTreeMap      | ✅ 가능         | Ord                      |
| BinaryHeap    | ✅ 가능         | Ord                      |



## 🧪 예제: OrderedFloat을 HashSet에 사용
```rust
use std::collections::HashSet;
use ordered_float::OrderedFloat;

fn main() {
    let mut set = HashSet::new();
    set.insert(OrderedFloat(3.14));
    set.insert(OrderedFloat(2.71));
    set.insert(OrderedFloat(3.14)); // 중복 제거됨

    for val in &set {
        println!("{}", val);
    }
}
```


## 🧪 예제: OrderedFloat을 BTreeMap에 키로 사용
```rust
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(OrderedFloat(1.0), "one");
    map.insert(OrderedFloat(2.0), "two");

    for (k, v) in &map {
        println!("{} → {}", k, v);
    }
}
```

### ⚠️ 주의할 점
- OrderedFloat::NaN도 존재하지만, 비교 시 NaN < x는 항상 false이므로 NaN을 넣는 건 추천되지 않음.
- 실수 비교가 필요한 경우 항상 OrderedFloat을 사용해 Ord를 보장해야함.


## 허용 오차 없음
Rust의 OrderedFloat<T>는 내부적으로 허용 오차(Tolerance)를 적용하지 않습니다.  
즉, OrderedFloat(3.14)와 OrderedFloat(3.1400001)은 완전히 다른 값으로 취급되며,  
정확한 f64 값의 일치 여부로만 중복을 판단합니다.  

## 🧠 핵심 개념
- OrderedFloat<T>는 PartialEq, Eq, Ord, Hash를 구현합니다
- 비교는 정확한 IEEE-754 부동소수점 값 기준
- Tolerance(오차 범위) 는 사용자가 직접 구현해야 합니다

## ✅ Tolerance를 적용하려면?
### 방법 1: 직접 비교 함수 작성
```rust
fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}
```

### 방법 2: HashSet 대신 Vec + approx_eq로 중복 제거
```rust
let mut values = vec![3.14];

let new = 3.1400001;
let tol = 0.0001;

if !values.iter().any(|&x| approx_eq(x, new, tol)) {
    values.push(new);
}
```


### 방법 3: 커스텀 래퍼 타입 만들기
```rust
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
struct TolerantFloat(f64);

impl PartialEq for TolerantFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.0001
    }
}
impl Eq for TolerantFloat {}

impl Hash for TolerantFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 정규화된 값으로 해시
        (self.0 * 10000.0).round() as i64 .hash(state);
    }
}
```
이 방식은 HashSet<TolerantFloat>처럼 사용할 수 있지만,  
정밀도 손실이나 해시 충돌 가능성에 주의해야 합니다.


## ✅ 요약
| 항목                     | 설명 또는 키워드                          |
|--------------------------|-------------------------------------------|
| 기본 비교 방식           | 정확한 f64 값 비교 (Tolerance 없음)       |
| 중복 판단 기준           | 완전 동일한 값만 중복으로 간주            |
| Tolerance 적용 방법       | 직접 비교 함수 또는 커스텀 타입 구현 필요 |


### ⚠️ 왜 OrderedFloat가 까다로운가?
- OrderedFloat은 Ord, Eq, Hash를 강제로 구현하지만 IEEE-754 부동소수점의 근본적인 불안정성은 해결하지 못함.
- 예를 들어 3.14 * 100.0과 314.0은 같아 보여도 다르게 저장될 수 있음.
- NaN, -0.0, +0.0 같은 특수 값도 비교/정렬/해시에 영향을 줍니다.

## ✅ 언제 OrderedFloat을 써도 괜찮은가?
| 상황                         | 사용 가능 여부 | 이유 또는 조건                      |
|------------------------------|----------------|-------------------------------------|
| 정렬만 필요한 경우           | ✅ 가능         | 정확한 비교만 필요                  |
| 우선순위 큐에서 휴리스틱 사용 | ✅ 가능         | NaN 제거 + Reverse로 정렬만 필요    |
| 정규화된 실수 비교           | ✅ 가능         | 오차가 없거나 무시 가능한 상황      |


## ❌ 언제 사용이 위험한가?
| 상황         | 사용 위험 여부 | 비교 방식       | 주의사항                                 |
|--------------|----------------|------------------|------------------------------------------|
| HashSet      | ❌ 위험         | == (정확한 값 비교) | 오차로 인해 근사값이 중복으로 인식되지 않음 |
| HashMap      | ❌ 위험         | == (정확한 값 비교) | 키 충돌 또는 누락 가능성 있음             |
| 실수 비교     | ❌ 위험         | ==                | 부동소수점 오차로 인해 예측 불가 결과 발생 |

## 🛠️ 대안 전략
- Tolerance 기반 비교 함수 직접 구현 → (a - b).abs() < ε
- 정규화된 정수로 변환 → round(f * 1000.0) 등
- 커스텀 래퍼 타입 → TolerantFloat로 Eq, Hash를 오차 기반으로 구현

## ✅ 요약
| 항목                     | 설명                                      |
|--------------------------|-------------------------------------------|
| OrderedFloat의 한계       | 오차 누적, NaN, 해시 충돌 가능성 있음     |
| 안전한 사용 조건          | 정렬만 필요하거나 오차 무시 가능한 경우   |
| 위험한 사용 조건          | 중복 제거, 키-값 매핑, 정밀 비교 등       |
| 대안                      | Tolerance 비교 함수 또는 정규화 변환       |

---



