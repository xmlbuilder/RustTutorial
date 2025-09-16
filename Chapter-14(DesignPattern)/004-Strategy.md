# Strategy Pattern

**전략 패턴(Strategy Pattern)**을 Rust, C++, C# 그리고 다시 Rust로 비교 구현한 예제.


## 🧠 전략 패턴 핵심 구조
- Strategy: 알고리즘 인터페이스 (trait / interface / abstract class)
- ConcreteStrategy: 실제 알고리즘 구현체
- Context: 전략을 사용하는 클래스, 전략을 교체 가능

### 🦀 Rust 예제 (Trait Object 기반)
```rust
trait Strategy {
    fn execute(&self);
}

struct StrategyA;
impl Strategy for StrategyA {
    fn execute(&self) {
        println!("Executing Strategy A");
    }
}

struct StrategyB;
impl Strategy for StrategyB {
    fn execute(&self) {
        println!("Executing Strategy B");
    }
}

struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    fn execute(&self) {
        self.strategy.execute();
    }
}

fn main() {
    let mut context = Context::new(Box::new(StrategyA));
    context.execute();

    context.set_strategy(Box::new(StrategyB));
    context.execute();
}
```


### ✅ Rust 설명
- trait Strategy는 C#의 interface, C++의 abstract class 역할
- Box<dyn Strategy>는 런타임에 전략을 교체할 수 있게 해주는 동적 디스패치
- Context는 전략을 받아서 실행하며, set_strategy()로 교체 가능

### 💠 C++ 예제 (unique_ptr 기반)
```cpp
#include <iostream>
#include <memory>

class Strategy {
public:
    virtual void execute() = 0;
    virtual ~Strategy() = default;
};

class StrategyA : public Strategy {
public:
    void execute() override {
        std::cout << "Executing Strategy A\n";
    }
};

class StrategyB : public Strategy {
public:
    void execute() override {
        std::cout << "Executing Strategy B\n";
    }
};

class Context {
    std::unique_ptr<Strategy> strategy;

public:
    Context(std::unique_ptr<Strategy> s) : strategy(std::move(s)) {}

    void setStrategy(std::unique_ptr<Strategy> s) {
        strategy = std::move(s);
    }

    void execute() {
        strategy->execute();
    }
};

int main() {
    Context context(std::make_unique<StrategyA>());
    context.execute();

    context.setStrategy(std::make_unique<StrategyB>());
    context.execute();

    return 0;
}
```


### 🧱 C# 예제
```csharp
using System;

interface IStrategy {
    void Execute();
}

class StrategyA : IStrategy {
    public void Execute() => Console.WriteLine("Executing Strategy A");
}

class StrategyB : IStrategy {
    public void Execute() => Console.WriteLine("Executing Strategy B");
}

class Context {
    private IStrategy strategy;

    public Context(IStrategy strategy) {
        this.strategy = strategy;
    }

    public void SetStrategy(IStrategy strategy) {
        this.strategy = strategy;
    }

    public void Execute() {
        strategy.Execute();
    }
}

class Program {
    static void Main() {
        var context = new Context(new StrategyA());
        context.Execute();

        context.SetStrategy(new StrategyB());
        context.Execute();
    }
}
```


## 🦀 Rust 예제 (제네릭 기반)
```rust
trait Strategy {
    fn execute(&self);
}

struct StrategyA;
impl Strategy for StrategyA {
    fn execute(&self) {
        println!("Executing Strategy A");
    }
}

struct StrategyB;
impl Strategy for StrategyB {
    fn execute(&self) {
        println!("Executing Strategy B");
    }
}

struct Context<T: Strategy> {
    strategy: T,
}

impl<T: Strategy> Context<T> {
    fn new(strategy: T) -> Self {
        Context { strategy }
    }

    fn execute(&self) {
        self.strategy.execute();
    }
}

fn main() {
    let context_a = Context::new(StrategyA);
    context_a.execute();

    let context_b = Context::new(StrategyB);
    context_b.execute();
}
```


🧠 Rust의 두 전략 방식 비교
| 방식              | 장점                                 | 단점                                 | 전략 교체 가능 여부 |
|-------------------|--------------------------------------|--------------------------------------|----------------------|
| `impl<T>`         | 빠른 성능 (정적 디스패치)              | 타입 고정으로 유연성 부족              | ❌ 불가능             |
| `Box<dyn Trait>`  | 런타임 전략 교체 가능 (동적 디스패치)   | 약간의 런타임 비용, object safety 필요 | ✅ 가능               |



## ✅ Rust 제네릭 설명
- Context<T: Strategy>는 정적 디스패치 방식
- 성능은 좋지만 런타임에 전략을 교체하려면 enum이나 Box<dyn Trait> 필요

🧩 요약 비교
| 언어     | 전략 인터페이스     | 전략 교체 방식           | 메모리 / 표현 방식          |
|----------|----------------------|---------------------------|-----------------------------|
| Rust     | `trait Strategy`     | `Box<dyn Strategy>`       | `Box`, 또는 `enum` 기반 표현 |
| C++      | `class Strategy`     | `unique_ptr<Strategy>`    | RAII, 스마트 포인터 사용     |
| C#       | `interface IStrategy`| `SetStrategy()` 메서드로 교체 | GC 기반 참조                 |

---



