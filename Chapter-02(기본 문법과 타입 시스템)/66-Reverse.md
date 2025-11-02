# Reverse
Reverse는 함수가 아니라 구조체(struct)입니다.  
정확히는 std::cmp::Reverse<T>라는 제네릭 래퍼 타입으로, Ord 트레잇의 비교 방향을 반전시키는 역할을 합니다.

## 🔍 Reverse는 무엇인가?
- 정의: Reverse<T>는 Rust 표준 라이브러리에 정의된 **구조체(struct)** 입니다.
- 역할: Ord 트레잇을 구현한 타입을 감싸서 정렬 기준을 반대로 적용합니다.
- 위치: std::cmp::Reverse 모듈에 포함되어 있습니다.
```rust
pub struct Reverse<T>(pub T);
```
- Reverse(5)는 5라는 값을 감싸는 구조체이며, Ord 비교 시 작은 값이 우선이 되도록 만듭니다.

## ✅ 사용 예시
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(10));
min_heap.push(Reverse(3));
min_heap.push(Reverse(7));

while let Some(Reverse(val)) = min_heap.pop() {
    println!("{}", val); // 3, 7, 10 순으로 출력
}
```
- Reverse(val)은 구조체의 패턴 매칭을 통해 내부 값을 꺼냅니다.
- BinaryHeap<Reverse<T>>는 min-heap처럼 작동합니다.

## 🧩 왜 함수가 아닌가?
- Reverse는 값을 감싸는 타입이지, 어떤 동작을 수행하는 함수가 아닙니다.
- Reverse(5)는 Reverse 구조체의 인스턴스를 생성하는 표현식입니다.
- 마치 Vec::new()가 Vec 타입을 생성하듯, Reverse(5)는 Reverse<i32> 타입을 생성합니다.

## 📌 정리

| 항목             | 설명                                |
|------------------|-------------------------------------|
| 타입             | `struct` (구조체)                   |
| 트레잇 기반      | `Ord` 비교 기준을 반전함             |
| 사용처           | `BinaryHeap`에서 min-heap 구현       |
| 동작 방식        | 내부 값을 감싸서 비교 방향을 반전     |
| 생성 방법        | `Reverse(value)`로 인스턴스 생성     |

## 🧩 1. Reverse를 직접 구현하기
Rust 표준 라이브러리의 Reverse<T>는 단순한 래퍼 타입입니다. 직접 구현해보면 다음과 같습니다:
```rust
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct MyReverse<T>(T);

impl<T: Ord> PartialOrd for MyReverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0) // 비교 방향 반전
    }
}

impl<T: Ord> Ord for MyReverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0) // 비교 방향 반전
    }
}
```

- MyReverse(5)는 5를 감싸고, 비교 시 작은 값이 더 높은 우선순위가 되도록 만듭니다.
- BinaryHeap<MyReverse<T>>를 사용하면 min-heap처럼 작동합니다.

## 🧩 2. 커스텀 정렬 기준 만들기 (Ord 직접 구현)
예를 들어, Task라는 구조체가 있고 priority가 낮을수록 우선순위가 높다고 가정해봅시다:
```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Task {
    name: &'static str,
    priority: u32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // 낮은 priority가 높은 우선순위가 되도록 반전
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

### ✅ 사용 예시
```rust
fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(Task { name: "A", priority: 3 });
    heap.push(Task { name: "B", priority: 1 });
    heap.push(Task { name: "C", priority: 2 });

    while let Some(task) = heap.pop() {
        println!("Task {} with priority {}", task.name, task.priority);
    }
}
```

- 출력 순서: B, C, A → priority가 낮은 순서대로 처리됨

## 📌 정리

| 항목           | 설명                                      |
|----------------|-------------------------------------------|
| Reverse<T>     | 표준 라이브러리의 래퍼 타입, 비교 방향 반전 |
| MyReverse<T>   | 사용자 정의 래퍼 타입, `Ord` 직접 구현     |
| Ord            | 정렬 기준을 정의하는 트레잇               |

## 🧠 핵심 요점
- `Reverse<T>` 는 `Ord` 를 반전시켜 min-heap을 구현할 수 있게 해주는 표준 래퍼 타입
- `MyReverse<T>` 는 Ord를 직접 구현하여 사용자 정의 정렬 기준을 만들 수 있음
- Ord는 Rust에서 정렬과 우선순위 결정의 핵심 트레잇

---





