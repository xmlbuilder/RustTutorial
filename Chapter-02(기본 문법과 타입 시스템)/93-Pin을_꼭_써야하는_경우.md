# 🔥 Pin이 꼭 필요한 상황 (실무 기준)
## 1) 자기 자신을 참조하는 구조체(self-referential struct)
- Rust는 기본적으로 move 시 메모리 주소가 바뀐다.
- 그런데 구조체 안에 **자기 자신을 가리키는 포인터** 가 있으면?
- 예:
```rust
struct Node {
    data: String,
    slice: *const str, // data 내부를 가리킴
}
```

- 이런 구조체는 move 되는 순간 slice가 무효화된다.
- 그래서 Rust는 이런 구조를 안전하게 만들 수 없다.
- 이때 필요한 게 Pin.
- Pin은:
  - **이 값은 절대 이동시키지 마라. 주소 고정!** 을 보장한다.


## 🔥 Pin 없이 터지는 self‑referential struct 예제
- 아래 코드는 절대 컴파일되지 않는다.
- Rust가 **이건 move 되면 주소가 바뀌니까 slice가 무효화된다** 라고 판단하기 때문이다.
```rust
use std::ptr;

struct Node {
    data: String,
    slice: *const str, // data 내부를 가리킴
}

impl Node {
    fn new(s: &str) -> Self {
        let data = s.to_string();
        let slice = &data[..] as *const str;

        Node { data, slice }
    }

    fn print(&self) {
        unsafe {
            println!("{}", &*self.slice);
        }
    }
}

fn main() {
    let mut node = Node::new("hello");

    // 여기서 move 발생
    let mut v = Vec::new();
    v.push(node); // <-- 여기서 Rust가 막는다
}
```


## 🔥 왜 이 코드가 터지는가?
- Rust는 node가 v.push(node) 되는 순간 move된다고 본다.
- move가 일어나면:
  - node.data의 메모리 주소가 바뀜
  - 그런데 node.slice는 옛날 주소를 가리키고 있음
  - 즉, dangling pointer 발생
- Rust는 이걸 컴파일 타임에 감지하고 다음과 같은 에러를 낸다:
```
error[E0505]: cannot move out of `node` because it is borrowed
```

- 또는
```
error[E0597]: borrowed value does not live long enough
```

- 또는
```
error[E0499]: cannot borrow `node.data` as mutable more than once
```

- Rust 버전에 따라 메시지는 조금 다르지만
- 핵심은 항상 같다:
  - **이 struct는 자기 자신을 참조하고 있으므로 move가 불가능하다.**


## 🔥 왜 Pin이 필요해지는가?
- 이 문제를 해결하려면 Rust에게 이렇게 말해야 한다:
- **이 Node는 절대 move되지 않는다. 주소가 고정되어 있으니 slice는 안전하다.**

- 그걸 보장하는 도구가 Pin이다.

## 🔥 Pin을 사용한 안전한 버전 (참고용)
```rust
use std::pin::Pin;

struct Node {
    data: String,
    slice: *const str,
}

impl Node {
    fn new(s: &str) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(Node {
            data: s.to_string(),
            slice: ptr::null(),
        });

        let slice = &boxed.data[..] as *const str;

        unsafe {
            let mut_ref = Pin::get_unchecked_mut(boxed.as_mut());
            mut_ref.slice = slice;
        }

        boxed
    }

    fn print(self: Pin<&Self>) {
        unsafe {
            println!("{}", &*self.slice);
        }
    }
}

fn main() {
    let node = Node::new("hello");
    node.as_ref().print();
}
```

- 여기서 핵심은:
  - Box::pin → Node의 주소가 절대 바뀌지 않음
  - Pin::get_unchecked_mut → unsafe로 slice를 설정
  - 이후 move 불가 → slice가 항상 유효


## 🔥 1. Pin이 적용된 값은 move가 금지된다
- 예를 들어:
```rust
use std::pin::Pin;

let mut node = Box::pin(Node::new("hello"));
```
- 이제 node의 타입은:
```rust
Pin<Box<Node>>
```
- 이 상태에서:
  
```rust
v.push(node);
```

- 를 하면 Rust는 절대 허용하지 않는다.
- 왜냐하면:
  - v.push(node)는 node를 move하려는 시도
  - 하지만 Pin<Box<Node>>는 move 불가
  - 그래서 Rust가 컴파일 단계에서 막는다
- Rust가 내는 에러는 보통 이런 형태:
```
error[E0277]: `Pin<Box<Node>>` cannot be moved out of
```

- 또는
```
error[E0599]: the method `push` cannot be called because it moves the value
```

- 핵심은:
  - Pin<Box<T>>는 move가 금지된 타입이다.
  - Vec::push는 move를 요구한다.
  - 그래서 둘은 절대 함께 쓸 수 없다.


## 🔥 2. 왜 move가 금지되는가?
- Pin은 **이 값의 메모리 주소는 절대 바뀌면 안 된다** 는 의미.
- 그런데 v.push(node)는:
  - node를 Vec 내부로 move
  - move되면 메모리 주소가 바뀜
  - self-referential struct는 내부 포인터가 깨짐
  - 그래서 Rust가 컴파일 단계에서 차단
- 즉, Pin은 **move 금지** 를 강제하는 장치.

## 🔥 3. 그럼 Pin된 값을 Vec에 넣으려면?
- 방법은 딱 2가지뿐.
- 방법 1) Vec 안에 Pin<Box<Node>>를 직접 넣기
```rust
let mut v: Vec<Pin<Box<Node>>> = Vec::new();
v.push(Box::pin(Node::new("hello")));
```

- 가능한 이유:
  - Vec은 Pin<Box<Node>> 자체를 move하지만
  - Pin<Box<Node>> 내부의 Node는 move되지 않기 때문
- 즉:
  - Pin<Box<Node>>는 move 가능
  - Box<Node> 내부의 Node는 move 불가
  - 그래서 안전

- 방법 2) Pin<&mut Node>를 슬라이스로만 관리
```rust
let mut node = Box::pin(Node::new("hello"));
let mut refs: Vec<Pin<&mut Node>> = vec![node.as_mut()];
```
- 이 경우도 move가 일어나지 않기 때문에 안전해.

## 🔥 4. 정리
| 상황                                   | 결과(설명)                                      |
|----------------------------------------|-------------------------------------------------|
| node: Node → v.push(node)              | OK (Node는 move 가능)                           |
| node: Pin<Box<Node>> → v.push(node)    | ❌ 컴파일 에러 (Pin<Box<Node>>는 move 금지)     |
| v.push(Box::pin(Node))                 | OK (Pin<Box<Node>> 자체는 move 가능)            |
| Vec<Pin<&mut Node>> 에 저장            | OK (참조만 저장, Node 자체는 move되지 않음)     |


## ⭐ 한 문장으로 정리
- Pin이 적용된 값은 move가 금지되므로
- v.push(node) 같은 move 연산은 컴파일 에러가 난다.
- 대신 Vec 안에 Pin<Box<T>>를 저장해야 한다.

---




