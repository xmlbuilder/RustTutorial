# Composite 패턴
“Leaf와 Composite를 동일한 인터페이스로 다루는 구조”를 구현

## 🧠 Composite 패턴 개요
Composite 패턴은 트리 구조의 객체 계층을 만들고,
**Leaf(단일 객체)**와 **Composite(자식들을 포함하는 객체)**를 동일한 인터페이스로 처리할 수 있게 해줍니다.

## 🦀 Rust 예제
```rust
trait Component {
    fn operation(&self);
}

struct Leaf {
    name: String,
}

impl Component for Leaf {
    fn operation(&self) {
        println!("Leaf: {}", self.name);
    }
}

struct Composite {
    name: String,
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    fn new(name: &str) -> Self {
        Composite {
            name: name.to_string(),
            children: vec![],
        }
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }
}

impl Component for Composite {
    fn operation(&self) {
        println!("Composite: {}", self.name);
        for child in &self.children {
            child.operation();
        }
    }
}

fn main() {
    let leaf1 = Box::new(Leaf { name: "Leaf 1".into() });
    let leaf2 = Box::new(Leaf { name: "Leaf 2".into() });

    let mut composite1 = Composite::new("Composite A");
    composite1.add(leaf1);
    composite1.add(leaf2);

    let leaf3 = Box::new(Leaf { name: "Leaf 3".into() });
    let mut root = Composite::new("Root");
    root.add(Box::new(composite1));
    root.add(leaf3);

    root.operation();
}
```

### ✅ Rust 설명
- Component 트레잇은 공통 인터페이스입니다.
- Leaf는 단일 객체, Composite는 자식들을 포함하는 복합 객체입니다.
- Box<dyn Component>를 통해 동적 디스패치로 트리 구조를 구성합니다.
- operation()은 재귀적으로 호출되어 전체 구조를 출력합니다.

## 💠 C++ 예제
```cpp
#include <iostream>
#include <vector>
#include <memory>

class Component {
public:
    virtual void operation() const = 0;
    virtual ~Component() = default;
};

class Leaf : public Component {
    std::string name;
public:
    Leaf(const std::string& n) : name(n) {}
    void operation() const override {
        std::cout << "Leaf: " << name << "\n";
    }
};

class Composite : public Component {
    std::string name;
    std::vector<std::unique_ptr<Component>> children;
public:
    Composite(const std::string& n) : name(n) {}
    void add(std::unique_ptr<Component> child) {
        children.push_back(std::move(child));
    }
    void operation() const override {
        std::cout << "Composite: " << name << "\n";
        for (const auto& child : children) {
            child->operation();
        }
    }
};

int main() {
    auto leaf1 = std::make_unique<Leaf>("Leaf 1");
    auto leaf2 = std::make_unique<Leaf>("Leaf 2");

    auto composite1 = std::make_unique<Composite>("Composite A");
    composite1->add(std::move(leaf1));
    composite1->add(std::move(leaf2));

    auto leaf3 = std::make_unique<Leaf>("Leaf 3");
    auto root = std::make_unique<Composite>("Root");
    root->add(std::move(composite1));
    root->add(std::move(leaf3));

    root->operation();
}
```


## 🧱 C# 예제
```csharp
using System;
using System.Collections.Generic;

interface IComponent {
    void Operation();
}

class Leaf : IComponent {
    private string name;
    public Leaf(string name) => this.name = name;
    public void Operation() => Console.WriteLine($"Leaf: {name}");
}

class Composite : IComponent {
    private string name;
    private List<IComponent> children = new();
    public Composite(string name) => this.name = name;
    public void Add(IComponent component) => children.Add(component);
    public void Operation() {
        Console.WriteLine($"Composite: {name}");
        foreach (var child in children) child.Operation();
    }
}

class Program {
    static void Main() {
        var leaf1 = new Leaf("Leaf 1");
        var leaf2 = new Leaf("Leaf 2");

        var composite1 = new Composite("Composite A");
        composite1.Add(leaf1);
        composite1.Add(leaf2);

        var leaf3 = new Leaf("Leaf 3");
        var root = new Composite("Root");
        root.Add(composite1);
        root.Add(leaf3);

        root.Operation();
    }
}
```


## 🐍 Python 예제

```python
class Component:
    def operation(self):
        raise NotImplementedError

class Leaf(Component):
    def __init__(self, name):
        self.name = name

    def operation(self):
        print(f"Leaf: {self.name}")

class Composite(Component):
    def __init__(self, name):
        self.name = name
        self.children = []

    def add(self, component):
        self.children.append(component)

    def operation(self):
        print(f"Composite: {self.name}")
        for child in self.children:
            child.operation()

def main():
    leaf1 = Leaf("Leaf 1")
    leaf2 = Leaf("Leaf 2")

    composite1 = Composite("Composite A")
    composite1.add(leaf1)
    composite1.add(leaf2)

    leaf3 = Leaf("Leaf 3")
    root = Composite("Root")
    root.add(composite1)
    root.add(leaf3)

    root.operation()

if __name__ == "__main__":
    main()
```


## 🧩 요약 비교
| 언어     | 공통 인터페이스     | Leaf 구현 방식     | Composite 구현 방식              | 특징                         |
|----------|----------------------|---------------------|----------------------------------|------------------------------|
| Rust     | `trait Component`    | `struct + impl`     | `Vec<Box<dyn Component>>`        | 안전한 타입, 명확한 소유권     |
| C++      | `class Component`    | `virtual` override  | `vector<unique_ptr<Component>>` | RAII 기반, 수동 메모리 관리     |
| C#       | `interface IComponent`| `class Leaf`        | `List<IComponent>`               | GC 기반, 명확한 구조           |
| Python   | `class Component`    | `class Leaf`        | `list` 기반 children 관리         | 유연하지만 타입 안전성 낮음     |


---



