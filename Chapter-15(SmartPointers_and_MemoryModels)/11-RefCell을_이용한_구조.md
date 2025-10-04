# 코드 분석

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, ..Node::default() }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

fn main() {
    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(5));
    let subtree = Node::new(10);
    subtree.borrow_mut().children.push(Node::new(11));
    subtree.borrow_mut().children.push(Node::new(12));
    root.borrow_mut().children.push(subtree);

    println!("그래프: {root:#?}");
    println!("그래프 합계: {}", root.borrow().sum());
}
```

이 코드는 단순한 트리 구조를 Rc<RefCell<Node>>로 표현하면서,  
사용자에게 내부 구현을 감추고 안전한 생성과 순회만 노출하는 구조적 의도가 아주 잘 드러난 예.  
아래에 핵심 포인트를 중심으로 분석.  

## 🧩 핵심 구조 요약
```rust
#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}
```

- Node는 트리 구조의 한 노드
- children은 Rc<RefCell<Node>>로 감싸서 공유 + 내부 가변성을 동시에 제공
- Default를 derive해서 `..Node::default()` 를 사용할 수 있게 함

## 🔍 Node::new(value)의 의도
```rust
fn new(value: i64) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node { value, ..Node::default() }))
}
```

### ✅ 의도 1: Wrapper로 감싸서 내부 구조 감춤
- 사용자는 Node::new(10)만 호출하면 됨  
- 내부적으로 Rc<RefCell<Node>>가 생성되지만, 사용자는 몰라도 됨    
    → API를 단순화하고, 내부 구현을 은닉
### ✅ 의도 2: ..Node::default()로 필드 자동 채움
- Node { value, ..Node::default() }는  
    → value만 지정하고 나머지 필드는 Default 값으로 채움
- children은 Vec::new()가 들어감  
  → 필드가 많아질 경우에도 간결하게 초기화 가능

## 🔍 sum() 메서드의 의도
```rust
fn sum(&self) -> i64 {
    self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
}
```

- 트리 전체를 순회하면서 value를 합산
- borrow()를 통해 RefCell 내부에 접근  
  → 불변 순회지만 내부 구조는 가변 가능성 유지


## ✅ 전체 설계 의도 요약
| 구성 요소             | 설계 의도 설명                                                                 |
|----------------------|----------------------------------------------------------------------------------|
| `Rc<RefCell<Node>>`  | 트리 구조에서 노드를 여러 곳에서 공유하면서도 내부 상태를 안전하게 수정 가능하게 함 |
| `Node::new()`        | 생성 과정을 감추고, 사용자에게 간단한 API만 제공하여 내부 구현 복잡성을 숨김       |
| `..Node::default()`  | 필요한 필드만 명시하고 나머지는 기본값으로 채워 초기화를 간결하게 처리함           |
| `sum()`              | 트리 전체를 순회하며 값을 합산하는 기능을 제공, 구조적 순회 로직을 캡슐화함         |
| `main()`             | 트리 구성과 사용 예시를 통해 API 사용법을 직관적으로 보여줌                        |



### 🔍 그런데 왜 ..Node::default()를 썼을까?
이건 확장성과 유지보수성을 고려한 설계 습관입니다.

### ✅ 1. 필드가 늘어날 가능성 대비
```rust
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
    label: Option<String>, // 나중에 추가될 수도 있음
}
```

- 이런 식으로 필드가 늘어나면 → new() 함수도 수정해야 함
- 하지만 ..Node::default()를 쓰면 → value만 지정하고 나머지는 자동 처리  
    → 코드 수정 범위를 최소화
### ✅ 2. 초기화 일관성 확보
- 모든 필드를 Default로 초기화하면
→ 구조체 생성 시 예상 가능한 상태를 보장
- 특히 테스트나 디버깅에서 유용
### ✅ 3. Rust의 구조적 철학에 부합
- Rust는 명시적이면서도 유연한 초기화를 지향
- ..default()는 그 철학을 반영한 문법
