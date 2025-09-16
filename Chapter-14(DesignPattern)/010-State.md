# 🧠 State 패턴 개요
State 패턴은 객체의 내부 상태에 따라 행동이 달라지는 구조를 만들기 위한 디자인 패턴입니다.
조건문 대신 상태 객체에 행동을 위임함으로써,
코드를 더 깔끔하고 확장 가능하게 유지할 수 있습니다.

## 🦀 Rust 예제: 간단한 문서 워크플로우
```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;
struct PendingReview;
struct Published;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview)
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct Post {
    state: Box<dyn State>,
    content: String,
}

impl Post {
    fn new() -> Self {
        Post {
            state: Box::new(Draft),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn content(&self) -> &str {
        self.state.content(self)
    }

    fn request_review(&mut self) {
        self.state = self.state.request_review();
    }

    fn approve(&mut self) {
        self.state = self.state.approve();
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("Rust는 멋진 언어입니다.");
    println!("초기 상태: {}", post.content()); // Draft 상태 → 출력 없음

    post.request_review();
    println!("리뷰 요청 후: {}", post.content()); // PendingReview 상태 → 출력 없음

    post.approve();
    println!("승인 후: {}", post.content()); // Published 상태 → 내용 출력
}
```


## ✅ Rust 설명
- State 트레잇은 각 상태가 구현해야 할 인터페이스입니다.
- Draft, PendingReview, Published는 각각의 상태를 나타내는 구조체입니다.
- Post는 상태를 Box<dyn State>로 보관하며, 상태 전환은 request_review()와 approve()를 통해 이루어집니다.
- content()는 상태에 따라 다르게 동작합니다. Published 상태에서만 실제 내용을 반환합니다.
Rust에서는 **상속 대신 트레잇과 동적 디스패치(Box<dyn Trait>)**를 활용해
상태 객체를 유연하게 교체할 수 있습니다.

## 💠 C++ 예제
```cpp
#include <iostream>
#include <memory>
#include <string>

class Post;

class State {
public:
    virtual void requestReview(Post& post) = 0;
    virtual void approve(Post& post) = 0;
    virtual std::string content(const Post& post) const { return ""; }
    virtual ~State() = default;
};

class Draft;
class PendingReview;
class Published;

class Post {
    std::unique_ptr<State> state;
    std::string text;
public:
    Post();
    void addText(const std::string& t) { text += t; }
    std::string getContent() const { return state->content(*this); }
    void requestReview() { state->requestReview(*this); }
    void approve() { state->approve(*this); }
    void setState(std::unique_ptr<State> newState) { state = std::move(newState); }
    const std::string& getText() const { return text; }
};

class Draft : public State {
public:
    void requestReview(Post& post) override;
    void approve(Post& post) override {}
};

class PendingReview : public State {
public:
    void requestReview(Post& post) override {}
    void approve(Post& post) override;
};

class Published : public State {
public:
    void requestReview(Post& post) override {}
    void approve(Post& post) override {}
    std::string content(const Post& post) const override { return post.getText(); }
};

Post::Post() : state(std::make_unique<Draft>()) {}

void Draft::requestReview(Post& post) {
    post.setState(std::make_unique<PendingReview>());
}

void PendingReview::approve(Post& post) {
    post.setState(std::make_unique<Published>());
}

int main() {
    Post post;
    post.addText("C++도 멋진 언어입니다.");
    std::cout << "초기 상태: " << post.getContent() << "\n";

    post.requestReview();
    std::cout << "리뷰 요청 후: " << post.getContent() << "\n";

    post.approve();
    std::cout << "승인 후: " << post.getContent() << "\n";
}
```


## 🧱 C# 예제
```csharp
using System;

interface IState {
    void RequestReview(Post post);
    void Approve(Post post);
    string Content(Post post) => "";
}

class Draft : IState {
    public void RequestReview(Post post) => post.State = new PendingReview();
    public void Approve(Post post) { }
}

class PendingReview : IState {
    public void RequestReview(Post post) { }
    public void Approve(Post post) => post.State = new Published();
}

class Published : IState {
    public void RequestReview(Post post) { }
    public void Approve(Post post) { }
    public string Content(Post post) => post.Text;
}

class Post {
    public IState State { get; set; } = new Draft();
    public string Text { get; private set; } = "";

    public void AddText(string text) => Text += text;
    public string Content() => State.Content(this);
    public void RequestReview() => State.RequestReview(this);
    public void Approve() => State.Approve(this);
}

class Program {
    static void Main() {
        var post = new Post();
        post.AddText("C#도 멋진 언어입니다.");
        Console.WriteLine("초기 상태: " + post.Content());

        post.RequestReview();
        Console.WriteLine("리뷰 요청 후: " + post.Content());

        post.Approve();
        Console.WriteLine("승인 후: " + post.Content());
    }
}
```


## 🧩 요약 비교
| 언어   | 상태 인터페이스     | 상태 객체 관리 방식         | 상태 전환 방식                   | 특징 |
|--------|----------------------|------------------------------|----------------------------------|------|
| Rust   | `trait State`        | `Box<dyn State>`             | `self: Box<Self> → Box<dyn>`     | 안전한 소유권, 명확한 전환 |
| C++    | `class State`        | `unique_ptr<State>`          | `setState(std::move(...))`       | 수동 메모리 관리, 명시적 전환 |
| C#     | `interface IState`   | `IState` 참조                | `post.State = new State()`       | GC 기반, 간결한 구조 |


## 🧠 왜 Rust가 더 어렵게 느껴질까?
| 개념 구분         | C++ 중심 사고                      | Rust의 대응 방식                    |
|------------------|------------------------------------|------------------------------------|
| 복사/이동         | `copy constructor`, `move`         | `Copy`, `Clone`, `move semantics` |
| 메모리 해제       | `delete`, `free`                   | `drop()` 자동 호출                 |
| 스마트 포인터     | `unique_ptr`, `shared_ptr`         | `Box`, `Rc`, `Arc`                |

--- 

# destruction

소멸(destruction) 관련 대화를 핵심 개념 중심으로 정리.
C++, Rust, 그리고 스마트 포인터의 소멸 흐름까지 비교하며 문서처럼 구성했습니다.

## 🧠 소멸(Drop/Destruction) 개념 정리
### 1. C++에서의 소멸자 호출 원리
- unique_ptr<T>는 RAII 객체로, 스택에서 벗어날 때 자동으로 소멸자 호출
- 소멸자는 내부의 T* 포인터를 delete하여 메모리 해제
- std::move()로 소유권을 넘기면, 이전 unique_ptr은 nullptr 상태가 되고
→ 새 객체가 스코프를 벗어날 때 소멸자 호출됨
{
    std::unique_ptr<Foo> ptr = std::make_unique<Foo>();
} // ptr 소멸자 호출 → Foo delete



### 2. Rust에서의 drop 호출 원리
- Box<T>는 힙에 있는 T를 소유
- 스코프가 끝나면 Box<T>가 자동으로 drop() 호출
- Rust는 소유권을 추적하여 drop 타이밍을 컴파일 타임에 결정
{
    let b = Box::new(Foo);
} // drop(b) 호출 → Foo 해제


- Drop 트레잇을 구현하면 커스텀 소멸 동작도 가능

### 3. 상태(State) 객체의 소유권 교체와 소멸
- Rust에서 Box<dyn State>를 사용하면 상태 객체의 소유권을 가짐
- 상태 전환 시 self.state = self.state.request_review();처럼 교체하면
→ 기존 상태 객체는 자동으로 drop됨
- C++에서도 unique_ptr<State>를 std::move()로 교체하면
→ 이전 객체는 자동으로 delete됨

### 4. 스마트 포인터 비교 요약
| 개념 구분         | C++ 중심 사고                      | Rust의 대응 방식                    |
|------------------|------------------------------------|------------------------------------|
| 복사/이동         | `copy constructor`, `move`         | `Copy`, `Clone`, `move semantics` |
| 메모리 해제       | `delete`, `free`                   | `drop()` 자동 호출                 |
| 스마트 포인터     | `unique_ptr`, `shared_ptr`         | `Box`, `Rc`, `Arc`                |



### 5. 핵심 철학 차이
| 항목               | `unique_ptr` (C++)           | `Box<T>` (Rust)           |
|--------------------|------------------------------|---------------------------|
| 자동 소멸 방식     | 스코프 종료 시 소멸자 호출     | 스코프 종료 시 `drop()` 호출 |
| 수동 소멸 방식     | `reset()`, `release()` 사용   | `drop(x)` 호출 가능        |



### 6. 결론
- C++은 RAII 기반으로 소멸자를 자동 호출하지만,
개발자가 new/delete를 직접 관리할 수도 있음
- Rust는 소유권과 drop을 언어 차원에서 강제하여
dangling, double free, 메모리 누수를 컴파일 타임에 차단
- 둘 다 스마트 포인터를 통해 자원 해제를 자동화하지만
Rust는 더 엄격하고 안전한 구조를 제공함


#### 🧠 Rc<T>의 동작 원리
- Rc는 Reference Counted의 약자
- 내부적으로 참조 카운터를 유지하면서
→ Rc::clone()을 통해 참조가 늘어나면 카운터 증가
→ Rc 인스턴스가 스코프를 벗어나면 카운터 감소
- 카운터가 0이 되는 순간 → T가 drop되고 메모리 해제
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("count = {}", Rc::strong_count(&a)); // 3

    drop(b);
    println!("count = {}", Rc::strong_count(&a)); // 2

    drop(c);
    println!("count = {}", Rc::strong_count(&a)); // 1

    drop(a); // 이제 count = 0 → String drop됨
}
```


### 🔍 특징 요약

| 조합                  | 설명                                      |
|-----------------------|-------------------------------------------|
| `Rc<T>`               | 단일 스레드에서 참조 카운트 기반 공유         |
| `Rc<T>` (복제)        | `Rc::clone()`으로 참조 수 증가, drop 시 감소  |
| `Rc<T>` vs `Arc<T>`   | `Rc`는 싱글 스레드용, `Arc`는 멀티 스레드용  |
| `Rc<RefCell<T>>`      | 공유 + 내부 가변성 허용 (`borrow_mut()` 사용) |


### 🧠 C++ vs Rust: 스마트 포인터 철학 비교

| 개념 구분           | C++                             | Rust                              |
|----------------------|----------------------------------|------------------------------------|
| 단일 소유권          | `unique_ptr<T>`                 | `Box<T>`                           |
| 공유 소유권          | `shared_ptr<T>`                 | `Rc<T>`, `Arc<T>`                  |
| 약한 참조            | `weak_ptr<T>`                   | `Weak<T>`                          |
| 내부 가변성          | `mutable`, `const_cast`         | `RefCell<T>`, `Cell<T>`            |
| 자동 소멸            | (스코프 종료 시 소멸자 호출)    | `drop()` 자동 호출                 |
| 순환 참조 방지       | `weak_ptr`                      | `Weak<T>`                          |


### 💡 요점:
- Rust는 C++의 스마트 포인터 개념을 더 엄격하고 안전하게 재해석한 구조입니다.
- C++은 개발자가 선택적으로 스마트 포인터를 사용하지만,
Rust는 언어 차원에서 소유권과 안전성을 강제합니다.
- 두 언어를 함께 이해하면 메모리 모델과 설계 철학을 더 깊이 파악할 수 있어요.

---

# 🧠 Rust에서의 리플렉션과 메타프로그래밍
## ❌ 런타임 리플렉션은 없음
- Rust는 **RTTI (Run-Time Type Information)**를 제공하지 않아요
- 즉, Java처럼 object.getClass().getFields() 같은 건 불가능
- 대신 컴파일 타임에 타입 정보를 활용하거나 생성하는 방식으로 접근

### ✅ 컴파일 타임 리플렉션 대체 도구

| 기능 구분             | Rust 도구                                 | 설명                                      |
|----------------------|--------------------------------------------|-------------------------------------------|
| 매크로 기반 코드 생성 | `macro_rules!`, `proc_macro`               | 반복되는 코드 생성, AST 분석 및 변형 가능  |
| 메타 정보 부여        | `#[derive(...)]`, `#[attribute]`           | 타입에 메타 정보 부여, 자동 트레잇 구현    |
| 타입 식별             | `Any`, `TypeId`                            | 런타임에 타입 비교 가능 (제한적 리플렉션)  |
| 구조체 리플렉션       | `Reflect` (Bevy 등에서 사용)               | 필드 접근, 직렬화, 타입 다운캐스팅 지원    |


### 🔧 예시: Procedural Macro로 Attribute 구현
```rust
#[derive(HelloMacro)]
struct MyType;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // AST 분석 후 코드 생성
}
```

- #[derive(...)]는 Rust의 컴파일 타임 코드 생성 도구
- proc_macro를 사용하면 커스텀 어트리뷰트나 DSL도 구현 가능

## 🎮 Bevy 엔진에서의 리플렉션 활용
- Bevy는 Reflect 트레잇을 통해 런타임에서 필드 접근, 직렬화, 타입 다운캐스팅을 지원
- #[derive(Reflect)]를 사용하면 타입 정보를 등록하고 다룰 수 있음
- 이는 Rust의 철학을 유지하면서도 게임 엔진이나 동적 시스템에 필요한 유연성을 제공

### ✅ 핵심 요약

| 기능 구분         | Rust의 대응 방식                         |
|------------------|------------------------------------------|
| 런타임 타입 정보   | RTTI 없음 (런타임 리플렉션 미지원)         |
| 컴파일 타임 메타   | `#[derive(...)]`, `#[attribute]`         |
| 타입 식별         | `Any`, `TypeId`                          |
| 구조체 리플렉션   | `Reflect` 트레잇 (Bevy 등에서 활용 가능)   |

----


