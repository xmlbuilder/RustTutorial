# 🔍 RefCell<T>란?
- **내부 가변성(Interior Mutability)** 을 제공하는 타입
- Rc<T>와 함께 자주 사용됨 (공유 + 변경 가능)
- 컴파일 시점이 아닌 런타임에 borrow 규칙을 검사
- 규칙 위반 시 panic 발생

## borrow() / borrow_mut()
RefCell<T>의 borrow()와 borrow_mut()은 Rust에서 런타임에 안전하게 내부 가변성을 제공하는 핵심 도구입니다.  
borrow()는 불변 참조를, borrow_mut()는 가변 참조를 반환하며, 컴파일 시점이 아닌 런타임에 대여 규칙을 검사합니다.


## 📌 borrow()와 borrow_mut() 차이
| 메서드         | 반환 타입   | 용도           | 동시 사용 가능 여부 | 런타임 검사 |
|----------------|-------------|----------------|----------------------|-------------|
| borrow()       | Ref<T>      | 불변 참조       | 여러 개 가능          | ✅           |
| borrow_mut()   | RefMut<T>   | 가변 참조       | 단 하나만 가능        | ✅           |


## ✅ 기본 샘플
```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    // 불변 참조
    let r1 = data.borrow();
    println!("r1: {}", r1);
    // 가변 참조
    let mut r2 = data.borrow_mut();
    *r2 += 1;
    println!("r2: {}", r2);
}
```

- r1은 Ref<i32> 타입
- r2는 RefMut<i32> 타입
- 동시에 r1과 r2를 만들면 panic 발생

## 🧩 실전 샘플: 트리 구조에서 부모 참조 변경
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

    // 부모 참조 확인
    if let Some(p) = child.parent.borrow().upgrade() {
        println!("Child's parent value: {}", p.value);
    }

    // 자식 목록 변경
    parent.children.borrow_mut().clear();
}
```

- RefCell을 통해 children과 parent를 런타임에 안전하게 변경
- borrow_mut()으로 자식 목록을 수정
- borrow()로 부모 참조를 읽음

## ⚠️ 주의사항
- RefCell은 단일 스레드 전용입니다. 멀티스레드에서는 Mutex<T>를 사용.
- borrow_mut() 중복 호출 시 panic 발생 → 반드시 스코프 관리 필요

---


