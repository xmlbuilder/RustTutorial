# NonNull

Rust의 `NonNull<T>` 는 null이 아닌 포인터를 안전하게 다루기 위한 타입입니다.  
일반적인 `raw pointer (*mut T, *const T)` 는 null일 수 있기 때문에 사용 시 항상 null 체크가 필요하지만,  
`NonNull<T>` 는 null이 아님을 보장하므로 이러한 체크 없이도 안전하게 사용할 수 있습니다.

## 🧠 핵심 개념
- `NonNull<T>` 는 *mut T를 래핑한 구조체입니다.
- null 포인터를 허용하지 않기 때문에, Option<NonNull<T>>는 null과 non-null을 구분하는 안전한 방식으로 사용됩니다.
- `NonNull<T>` 는 Send, Sync, Copy 등의 trait을 구현하지 않으며, 수정 가능한 포인터로서의 역할에 집중합니다.

## 🛠 주요 메서드와 사용 예시
```rust
use std::ptr::NonNull;

fn main() {
    let mut value = 42;
    let ptr = NonNull::new(&mut value as *mut i32).expect("Pointer is null");
    
    unsafe {
        *ptr.as_mut() += 1;
    }

    println!("Updated value: {}", value); // 43
}
```

- NonNull::new(ptr) → null이 아닌 경우 Some(NonNull)을 반환
- NonNull::new_unchecked(ptr) → null 체크 없이 생성 (⚠️ unsafe)
- `as_ptr()` → 내부 raw pointer 반환
- `as_mut()` /` as_ref()` → mutable 또는 immutable 참조 반환 (unsafe)

### ⚠️ 주의사항
- NonNull은 null이 아님을 보장하지만, 유효한 메모리인지까지는 보장하지 않습니다.
- NonNull::dangling()은 잘 정렬된 **dangling** 포인터를 생성하지만, 실제로 접근하면 undefined behavior가 발생할 수 있습니다.
- NonNull을 통해 참조를 생성할 때는 반드시 해당 메모리가 초기화되어 있고 접근 가능한 상태인지 확인해야 합니다.


### 📦 활용 예시
- 스마트 포인터 구현: Box, Rc, Arc 등 내부적으로 NonNull을 사용해 null 체크를 제거하고 성능을 향상시킵니다.
- 옵션 포인터 표현: Option<NonNull<T>>는 Option<*mut T>보다 메모리 효율적이며 안전합니다.
- FFI 또는 저수준 시스템 프로그래밍에서 null이 아닌 포인터를 다룰 때 유용합니다.

---

# NonNull<T>라는 타입을 따로 도입했을까요? 
그 이유는 성능, 표현력, 그리고 안전성을 모두 만족시키기 위한 설계적 선택 때문입니다.

## 🧩 Option<NonNull<T>> vs Option<*mut T>
### 1. ✅ 메모리 최적화 (null pointer optimization)
- Rust는 Option<NonNull<T>>를 내부적으로 NonNull<T>와 동일한 크기로 최적화합니다.
- 반면 Option<*mut T>는 null을 표현하기 위해 추가적인 메모리가 필요할 수 있습니다.
- 즉, NonNull<T>를 쓰면 Option을 써도 추가 오버헤드 없이 null/non-null을 표현할 수 있음.

###  2. 🔒 안전성 강화
- NonNull<T>는 null이 아님을 타입 수준에서 보장합니다.
- *mut T는 언제든지 null일 수 있으므로, 사용 시 항상 null 체크가 필요하고 실수로 dereference하면 undefined behavior가 발생할 수 있음.
- NonNull<T>를 사용하면 컴파일러가 null 관련 버그를 사전에 방지할 수 있습니다.

### 3. 🧠 의도 표현
- NonNull<T>를 사용하면 **이 포인터는 절대 null이 아니어야 한다** 는 개발자의 의도를 명확히 표현할 수 있음.
- 이는 코드의 가독성과 유지보수성에도 큰 도움이 됩니다.

### 📦 실제 사용 예시
```rust
use std::ptr::NonNull;

struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        let boxed = Box::new(value);
        MyBox {
            ptr: NonNull::from(Box::leak(boxed)),
        }
    }

    fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}
```

- 위 예시처럼 Box, Rc, Arc 같은 스마트 포인터 구현에서 NonNull<T>는 null이 아님을 보장하면서도 raw pointer처럼 유연하게 사용됩니다.

## 🔍 요약
| 항목             | *mut T       | NonNull<T>   | Option<NonNull<T>> |
|------------------|--------------|--------------|---------------------|
| Null 허용        | ✅ 가능       | ❌ 불가능     | ✅ 가능              |
| 안전성           | ⚠️ 낮음       | ✅ 높음       | ✅ 높음              |
| 메모리 최적화    | ❌ 없음       | ❌ 없음       | ✅ 있음              |
| 의도 표현        | ❌ 불명확     | ✅ 명확       | ✅ 명확              |
| 사용 시 null 체크 | 필요함        | 불필요        | `is_some()`로 체크   |


결국 NonNull<T>는 Option<T>와 경쟁하는 게 아니라, raw pointer를 안전하게 감싸면서 Option과 함께 쓸 수 있도록 설계된 타입이에요.  


## 🔍 코드 분석
```rust
ptr: NonNull::from(Box::leak(boxed))
```

이 줄은 boxed라는 Box<T>를 NonNull<T>로 변환하는 과정입니다.  
각 단계는 다음과 같습니다:

## 🧱 1. Box::leak(boxed)
- Box<T>는 Rust의 힙 할당 스마트 포인터입니다.
- Box::leak(boxed)는 Box<T>를 누수(leak) 시켜서 `'static 수명의 &mut T` 참조를 반환합니다.
- 이 참조는 Box가 drop되지 않고 프로그램이 끝날 때까지 살아있음을 의미합니다.
```rust
let boxed = Box::new(42);
let leaked: &mut i32 = Box::leak(boxed); // 이제 leaked는 'static 수명
```


## 🧱 2. `NonNull::from(...)`
- NonNull::from(&mut T)는 &mut T를 null이 아닌 포인터로 감싸는 NonNull<T>로 변환합니다.
- 이 포인터는 null이 아님을 타입 수준에서 보장합니다.
```rust
let ptr: NonNull<i32> = NonNull::from(leaked);
```

## 🔗 전체 의미
```rust
ptr: NonNull::from(Box::leak(boxed))
```

- Box<T>를 힙에 할당하고,
- 그 값을 누수시켜서 'static 참조로 만들고,
- 그 참조를 NonNull<T>로 감싸서 null이 아닌 안전한 포인터로 저장하는 것입니다.

## 📦 왜 이렇게 쓰는가?
- 스마트 포인터 구현에서 내부적으로 NonNull<T>를 사용하면 null 체크 없이 안전하게 포인터를 다룰 수 있습니다.
- Box::leak을 쓰면 drop을 방지하고 'static 수명을 얻을 수 있어 수명 관리가 단순해집니다.
- 이 패턴은 Rc, Arc, Vec, HashMap 같은 표준 라이브러리에서도 자주 사용됩니다.

## ⚠️ 주의사항
- Box::leak은 메모리 누수를 일으킬 수 있으므로, 정말로 프로그램 종료까지 살아있어야 할 경우에만 사용해야 합니다.
- NonNull은 null이 아님을 보장하지만, 유효한 메모리인지까지는 보장하지 않으므로 unsafe 블록에서 사용 시 주의가 필요합니다.

---


## ✅ 실용적인 예제: NonNull을 사용한 intrusive linked list
Rust의 안전한 Option<Box<Node>> 기반 리스트는 간단하지만, intrusive 구조나 raw pointer 최적화가 필요할 때 NonNull이 유용합니다.  
### 🔹 예제: 단방향 링크드 리스트 노드
```rust
use std::ptr::NonNull;

struct Node {
    value: i32,
    next: Option<NonNull<Node>>,
}

impl Node {
    fn new(value: i32) -> Box<Self> {
        Box::new(Node { value, next: None })
    }

    fn set_next(&mut self, next: &mut Node) {
        self.next = Some(NonNull::from(next));
    }

    fn next(&self) -> Option<&Node> {
        self.next.map(|ptr| unsafe { ptr.as_ref() })
    }
}

fn main() {
    // 노드 생성
    let mut node1 = Node::new(10);
    let mut node2 = Node::new(20);
    let mut node3 = Node::new(30);

    // 연결: node1 → node2 → node3
    node1.set_next(&mut node2);
    node2.set_next(&mut node3);

    // 순회하며 값 출력
    let mut current = Some(&*node1);
    while let Some(node) = current {
        println!("Node value: {}", node.value);
        current = node.next();
    }
}

```
### 출력 결과
```
Node value: 10
Node value: 20
Node value: 30
```

## 🔍 설명
- NonNull<Node>는 *mut Node와 같지만 null이 아님을 보장합니다
- Option<NonNull<T>>는 Option<*mut T>보다 안전하고, null 체크 없이도 None 표현 가능
- unsafe는 as_ref()에서만 필요하고, 나머지는 안전하게 유지 가능

## 🧠 왜 이게 실용적인가?
- 이 구조는 GC 없는 시스템, 커널 수준 자료구조, 또는 게임 엔진의 ECS에서 자주 사용됩니다
- Box::leak() 없이도 Box<Node>를 만들고, &mut Node를 NonNull로 변환해 연결 가능
- Rc, Arc, 또는 Vec 기반 구조보다 더 낮은 수준의 제어가 필요할 때 적합

## ⚠️ 주의할 점
- NonNull은 null이 아님을 보장하지만, 여전히 dangling pointer 위험이 있음
- unsafe 블록은 꼭 필요한 최소 범위로 제한해야 함
- Box::leak()은 메모리 해제를 못하므로, 일반 로직에서는 피하는 게 좋아요

## ✨ 요약: NonNull과 관련 포인터 비교
| 항목                     | 설명                                       |
|--------------------------|--------------------------------------------|
| `NonNull<T>`             | null이 아님을 보장하는 안전한 raw 포인터   |
| `*mut T`                 | null 가능, unsafe에서 자주 사용됨          |
| `Option<NonNull<T>>`     | null 여부를 안전하게 표현 가능 (`None`)    |
| `Option<*mut T>`         | null 표현 가능하지만 더 위험하고 덜 안전함 |




