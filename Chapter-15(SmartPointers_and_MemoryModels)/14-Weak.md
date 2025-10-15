# Weak
Rust의 Weak<T>는 Rc<T>나 Arc<T>와 함께 사용되어 참조 사이클을 방지하고 메모리 누수를 막기 위한 비소유 참조입니다.

## 🔍 Weak<T>란?
- Weak<T>는 Rc<T> 또는 Arc<T>가 관리하는 메모리 블록에 대한 비소유(non-owning) 참조입니다.
- Weak는 참조 카운트를 증가시키지 않기 때문에, 해당 값이 drop되는 것을 막지 않습니다.
- Weak는 `.upgrade()` 를 통해 Option<Rc<T>> 또는 Option<Arc<T>>로 변환할 수 있으며, 원본이 이미 drop되었다면 None을 반환합니다.

## 🧠 언제 사용하나?
- 참조 사이클을 끊기 위해: 예를 들어, 트리 구조에서 부모는 자식을 Rc로, 자식은 부모를 Weak로 참조하면 순환 참조를 피할 수 있습니다.
- 캐시나 임시 참조: 객체가 살아있을 때만 접근하고 싶을 때 Weak를 사용하면 안전하게 접근 가능합니다.

## ✅ 기본 예제
```rust
use std::rc::{Rc, Weak};
fn main() {
    let strong = Rc::new(42);
    let weak = Rc::downgrade(&strong);

    println!("Strong count: {}", Rc::strong_count(&strong)); // 1
    println!("Weak count: {}", Rc::weak_count(&strong));     // 1

    if let Some(upgraded) = weak.upgrade() {
        println!("Upgraded value: {}", upgraded);
    } else {
        println!("Value has been dropped");
    }
}
```

- `Rc::downgrade()` 로 Weak 생성
- `weak.upgrade()` 로 Rc로 복원 가능

## 🧩 실전 예제: 부모-자식 트리 구조
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Rc::downgrade(&parent)),
        children: RefCell::new(vec![]),
    });

    parent.children.borrow_mut().push(Rc::clone(&child));

    // 접근 예시
    if let Some(p) = child.parent.borrow().upgrade() {
        println!("Child's parent value: {}", p.value);
    }
}
```

- 부모는 자식을 Rc로 소유
- 자식은 부모를 Weak로 참조 → 순환 참조 없음
- upgrade()로 부모에 안전하게 접근

## 📌 요약
| 항목           | 설명                                         |
|----------------|----------------------------------------------|
| `Weak<T>`        | Rc<T> 또는 Arc<T>의 비소유 참조                |
| `.upgrade()`     | Option<Rc<T>>로 변환 → 원본이 살아있으면 Some |
| 참조 카운트 증가 없음 | drop 시점에 영향을 주지 않음 → 순환 참조 방지     |

---


