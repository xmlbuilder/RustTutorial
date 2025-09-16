# 🧠 Flyweight 패턴이란?
- 목적: 동일한 데이터를 가진 객체들을 공유하여 메모리 사용을 최소화
- 핵심 개념:
- Intrinsic state: 공유 가능한 내부 상태
- Extrinsic state: 개별 객체마다 다른 외부 상태


## 🦀 Rust 버전 (싱글 스레드 기준)
```rust
use std::collections::HashMap;
use std::rc::Rc;

struct TreeType {
    name: String,
    texture: String,
}

impl TreeType {
    fn draw(&self, x: i32, y: i32) {
        println!("Drawing {} with texture {} at ({}, {})", self.name, self.texture, x, y);
    }
}

struct TreeFactory {
    types: HashMap<String, Rc<TreeType>>,
}

impl TreeFactory {
    fn new() -> Self {
        TreeFactory {
            types: HashMap::new(),
        }
    }

    fn get_tree_type(&mut self, name: &str, texture: &str) -> Rc<TreeType> {
        self.types
            .entry(name.to_string())
            .or_insert_with(|| Rc::new(TreeType {
                name: name.to_string(),
                texture: texture.to_string(),
            }))
            .clone()
    }
}

fn main() {
    let mut factory = TreeFactory::new();
    let tree1 = factory.get_tree_type("Oak", "Green");
    let tree2 = factory.get_tree_type("Oak", "Green");

    tree1.draw(10, 20);
    tree2.draw(30, 40);

    println!("Same instance? {}", Rc::ptr_eq(&tree1, &tree2)); // true
}
```


### ✅ Rust 설명 요약
- Rc<TreeType>: 공유 객체를 참조 카운트 기반으로 관리
- HashMap: 캐시 역할, 동일한 키에 대해 객체 재사용
- Rc::ptr_eq(): 실제로 같은 인스턴스를 공유하는지 확인 가능
- Rust는 소유권과 안전성을 강제하므로, Flyweight 패턴이 매우 자연스럽게 구현됨

## 💻 C++ 버전
```cpp
#include <iostream>
#include <memory>
#include <unordered_map>

class TreeType {
public:
    TreeType(std::string name, std::string texture)
        : name(name), texture(texture) {}

    void draw(int x, int y) {
        std::cout << "Drawing " << name << " with texture " << texture
                  << " at (" << x << ", " << y << ")\n";
    }

private:
    std::string name;
    std::string texture;
};

class TreeFactory {
public:
    std::shared_ptr<TreeType> getTreeType(const std::string& name, const std::string& texture) {
        std::string key = name + texture;
        if (types.find(key) == types.end()) {
            types[key] = std::make_shared<TreeType>(name, texture);
        }
        return types[key];
    }

private:
    std::unordered_map<std::string, std::shared_ptr<TreeType>> types;
};

int main() {
    TreeFactory factory;
    auto tree1 = factory.getTreeType("Oak", "Green");
    auto tree2 = factory.getTreeType("Oak", "Green");

    tree1->draw(10, 20);
    tree2->draw(30, 40);
}

```

🧩 C# 버전
```cpp
using System;
using System.Collections.Generic;

class TreeType {
    public string Name { get; }
    public string Texture { get; }

    public TreeType(string name, string texture) {
        Name = name;
        Texture = texture;
    }

    public void Draw(int x, int y) {
        Console.WriteLine($"Drawing {Name} with texture {Texture} at ({x}, {y})");
    }
}

class TreeFactory {
    private Dictionary<string, TreeType> types = new();

    public TreeType GetTreeType(string name, string texture) {
        string key = name + texture;
        if (!types.ContainsKey(key)) {
            types[key] = new TreeType(name, texture);
        }
        return types[key];
    }
}

class Program {
    static void Main() {
        var factory = new TreeFactory();
        var tree1 = factory.GetTreeType("Oak", "Green");
        var tree2 = factory.GetTreeType("Oak", "Green");

        tree1.Draw(10, 20);
        tree2.Draw(30, 40);
    }
}

```

## 🐍 Python 버전
```python
class TreeType:
    def __init__(self, name, texture):
        self.name = name
        self.texture = texture

    def draw(self, x, y):
        print(f"Drawing {self.name} with texture {self.texture} at ({x}, {y})")

class TreeFactory:
    def __init__(self):
        self.types = {}

    def get_tree_type(self, name, texture):
        key = name + texture
        if key not in self.types:
            self.types[key] = TreeType(name, texture)
        return self.types[key]

factory = TreeFactory()
tree1 = factory.get_tree_type("Oak", "Green")
tree2 = factory.get_tree_type("Oak", "Green")

tree1.draw(10, 20)
tree2.draw(30, 40)

```


### 🔍 다른 언어들과의 비교

| 언어       | Flyweight 구현 방식                     | 특징 요약                                  |
|------------|------------------------------------------|--------------------------------------------|
| **Rust**   | `Rc` + `HashMap`                         | 안전한 공유, 참조 카운트 기반, 소유권 명확 |
| **C++**    | `shared_ptr` + `unordered_map`           | 수동 관리, 메모리 누수 주의 필요           |
| **C#**     | `Dictionary` + `FlyweightFactory` 클래스 | GC 기반, 구조적 구현 쉬움                  |
| **Python** | `dict` + `__new__` 오버라이딩            | 유연하지만 안전성 낮음, 동적 타입 기반     |





### 🎯 마무리 요약
- Rust는 Rc, HashMap, 소유권 모델 덕분에 Flyweight 패턴이 안전하고 자연스럽게 구현됨
- C++은 shared_ptr과 unordered_map으로 유사하게 구현 가능하지만, 메모리 안전은 개발자 책임
- C#과 Python은 GC 기반이라 구현은 간단하지만, 객체 공유 여부를 명확히 추적하기 어려움
- Rust는 성능 + 안전성 + 명확한 구조를 동시에 만족시키는 Flyweight 구현에 매우 적합한 언어
