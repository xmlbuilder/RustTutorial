# RefCell / Rc<RefCell<T>>
RefCell<T>를 단독으로 쓸 때와 Rc<RefCell<T>>로 함께 쓸 때는 용도와 공유 방식에서 중요한 차이가 있음.

## 🧠 핵심 차이
| 구조               | 내부 가변성 | 공유 가능성     | 사용 환경            |
|--------------------|-------------|------------------|-----------------------|
| RefCell<T>         | ✅ 있음     | ❌ 단일 소유자   | 단일 소유자, 구조체 내부 |
| Rc<RefCell<T>>     | ✅ 있음     | ✅ 다중 소유자   | 단일 스레드, 공유 + 변경 |

## 🔍 요약 설명
- RefCell<T>: 내부 값을 런타임에 변경할 수 있지만, 소유자는 하나뿐. 구조체 내부에서만 값 변경이 필요할 때 적합.
- Rc<RefCell<T>: 여러 곳에서 참조하면서도 값을 바꿔야 할 때 사용. 단일 스레드 환경에서 공유와 변경을 동시에 가능하게 해주는 조합.

## ✅ RefCell<T> 단독 사용 예시
```rust
use std::cell::RefCell;

struct Counter {
    value: RefCell<u32>,
}

fn main() {
    let counter = Counter {
        value: RefCell::new(0),
    };

    *counter.value.borrow_mut() += 1;
    println!("Count: {}", counter.value.borrow());
}
```

- Counter는 단일 소유자
- 내부 값만 변경 가능하면 충분 → RefCell<T>만 사용

## ✅ Rc<RefCell<T>> 함께 쓰는 예시
```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { value: 1, next: None }));
    let b = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&a)) }));

    // a를 여러 곳에서 참조하면서 내부 값도 변경 가능
    a.borrow_mut().value += 10;

    println!("a: {:?}", a.borrow());
    println!("b: {:?}", b.borrow());
}
```
- Rc로 여러 곳에서 Node를 공유
- RefCell로 내부 값 변경 가능
- 이 조합은 공유 + 변경이 동시에 필요한 경우에 필수

## 🔍 언제 어떤 걸 써야 할까?
| 상황                         | 추천 구조             |
|------------------------------|------------------------|
| 단일 소유자, 내부 값만 변경     | RefCell<T>             |
| 여러 소유자, 단일 스레드에서 변경 | Rc<RefCell<T>>         |
| 여러 소유자, 멀티스레드에서 변경 | Arc<Mutex<T>>          |

## 🧠 설명 추가
- RefCell<T>: 내부 가변성만 필요할 때. 구조체 안에서 값만 바꾸고 외부 공유가 필요 없을 때.
- Rc<RefCell<T>: 여러 곳에서 참조하면서도 값을 바꿔야 할 때. 단일 스레드 환경에서만 안전.
- Arc<Mutex<T>: 멀티스레드 환경에서 공유 + 변경이 필요할 때. Mutex로 동기화, Arc로 참조 카운트.

## ✅ 요약
RefCell<T>는 내부 가변성만,  
Rc<RefCell<T>>는 공유 + 내부 가변성을 동시에 제공.  
상황에 따라 조합해서 쓰면 Rust의 안전성과 유연성을 모두 살릴 수 있음.  

---



