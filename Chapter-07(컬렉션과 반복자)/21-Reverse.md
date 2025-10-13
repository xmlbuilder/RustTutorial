# Reverse
std::cmp::Reverse는 Rust에서 정렬 방향을 반전시키는 데 특화된 도구로,  
특히 BinaryHeap, sort_by_key, BTreeMap 등에서 매우 유용하게 쓰입니다.

## 🔍 Reverse란?
- Reverse<T>는 Ord, PartialOrd를 반대로 구현한 래퍼 타입
- 즉, Reverse(5)는 Reverse(10)보다 작은 값으로 간주됨
- 원래 Ord가 큰 값 우선이라면 → Reverse는 작은 값 우선

## ✅ Reverse의 주요 사용처
| 사용처        | 목적               | 효과 또는 설명                        | 예시 함수 또는 구조체         |
|---------------|--------------------|---------------------------------------|-------------------------------|
| BinaryHeap    | 최소 힙 구현       | 작은 값이 우선순위가 높게 작동        | `BinaryHeap<Reverse<T>>`     |
| sort_by_key   | 내림차순 정렬      | 큰 값이 먼저 오도록 정렬              | `sort_by_key(|x| Reverse(x))`|
| BTreeMap      | 역순 키 순회       | 큰 키부터 순회                        | `BTreeMap<Reverse<K>, V>`    |
| 사용자 정의 정렬 | 우선순위 반전     | 커스텀 정렬 기준을 반대로 적용 가능    | `Ord` 기반 구조체에 활용      |


## 🧪 샘플 예제 1: BinaryHeap → 최소 힙
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::new();

    min_heap.push(Reverse(10));
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(20));

    while let Some(Reverse(val)) = min_heap.pop() {
        println!("Min pop: {}", val); // 출력: 5 → 10 → 20
    }
}
```


## 🧪 샘플 예제 2: sort_by_key → 내림차순 정렬
```rust
let mut scores = vec![("Alice", 80), ("Bob", 95), ("Carol", 70)];

scores.sort_by_key(|&(_, score)| Reverse(score));

for (name, score) in scores {
    println!("{}: {}", name, score); // Bob → Alice → Carol
}
```


## 🧪 샘플 예제 3: BTreeMap → 큰 키부터 순회
```rust
use std::collections::BTreeMap;
use std::cmp::Reverse;

let mut map = BTreeMap::new();
map.insert(Reverse(1), "one");
map.insert(Reverse(3), "three");
map.insert(Reverse(2), "two");

for (Reverse(k), v) in &map {
    println!("{} → {}", k, v); // 3 → 2 → 1
}
```
## ✅ 요약
| 개념         | 설명                                      |
|--------------|-------------------------------------------|
| `Reverse<T>`   | 정렬 방향을 반전시키는 래퍼 타입          |
| 주요 용도     | 최소 힙, 내림차순 정렬, 역순 키 순회 등    |
| 반환 해제     | `Reverse(val)`로 패턴 매칭해서 값 꺼냄     |

---

# 사용자 정의 구조체

## 🧪 예제: 사용자 정의 구조체 + Reverse 정렬
### 🎯 목표

Task 구조체를 우선순위 기준으로 정렬하되,  
작은 숫자가 높은 우선순위가 되도록 `Reverse` 를 활용합니다.

### ✔ 코드 예제
```rust
use std::cmp::Reverse;

#[derive(Debug)]
struct Task {
    name: String,
    priority: u32,
}

fn main() {
    let mut tasks = vec![
        Task { name: "Write report".into(), priority: 3 },
        Task { name: "Fix bug".into(), priority: 1 },
        Task { name: "Email client".into(), priority: 2 },
    ];

    // 우선순위 낮은 숫자가 먼저 오도록 정렬
    tasks.sort_by_key(|task| Reverse(task.priority));

    for task in tasks {
        println!("{} (priority {})", task.name, task.priority);
    }
}
```
### 🧾 출력 결과
```
Fix bug (priority 1)
Email client (priority 2)
Write report (priority 3)
```

## ✅ 요약
| 요소                  | 역할 또는 설명                     | 예시 또는 효과                  |
|-----------------------|------------------------------------|---------------------------------|
| `Reverse(task.priority)`| 정렬 기준을 반전시키는 래퍼        | 작은 숫자가 높은 우선순위로 작동 |
| `sort_by_key`           | 정렬 기준을 지정하는 메서드        | `Reverse(x)`로 내림차순 정렬     |
| `사용자 정의 구조체`     | 정렬 대상                          | 작업 큐, 랭킹, 스케줄러 등 활용  |

---



