# Reverse 패턴 매칭
우리가 패턴 매칭을 통해 직접 벗겨내는 것입니다.  
Reverse도 마찬가지로 `구조체의 필드` 를 꺼내기 위해 `패턴 매칭` 을 사용합니다.

## 🧩 예시: Reverse를 벗겨내는 방식
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(Reverse(5));
heap.push(Reverse(1));
heap.push(Reverse(3));

while let Some(Reverse(val)) = heap.pop() {
    println!("{}", val);
}
```

## 🔍 여기서 중요한 부분
```rust
while let Some(Reverse(val)) = heap.pop()
```

- heap.pop()의 반환 타입은 Option<Reverse<i32>>
- Some(...)은 Option을 벗겨내는 패턴
- Reverse(val)은 Reverse 구조체를 벗겨내는 구조체 패턴 매칭
- 이 두 패턴을 중첩해서 사용한 것

## 🧠 구조체 패턴 매칭이란?
```rust
struct Point(i32, i32);

let p = Point(3, 7);

let Point(x, y) = p;
println!("x = {}, y = {}", x, y); // x = 3, y = 7
```

- Point(x, y)는 Point 구조체의 필드를 꺼내는 패턴
- Reverse(val)도 같은 방식으로 동작

## ✅ 정리

| 표현                      | 설명                                |
|---------------------------|-------------------------------------|
| `Some(x)`                  | `Option<T>`에서 값 꺼내기           |
| `Reverse(val)`             | `Reverse<T>` 구조체에서 값 꺼내기   |
| `Some(Reverse(val))`       | `Option<Reverse<T>>`에서 중첩된 값 꺼내기 |
| `while let Some(Reverse(x))` | 반복적으로 꺼내면서 구조체 벗기기   |


## 🧠 핵심 요점
- Reverse는 구조체이므로 패턴 매칭으로 필드를 꺼낼 수 있음
- Option과 Reverse가 중첩된 경우, Some(Reverse(val))처럼 중첩 패턴 매칭을 사용
- while let은 반복적으로 값을 꺼내며 조건부 실행을 가능하게 함

---
