# Factory Method 패턴

Factory Method 패턴을 Rust, C++, C#, Python 네 가지 언어로 각각 구현한 예제.

## 🦀 Rust 예제
```rust
trait Transport {
    fn deliver(&self);
}

struct Truck;
impl Transport for Truck {
    fn deliver(&self) {
        println!("Delivering by land in a box.");
    }
}

struct Ship;
impl Transport for Ship {
    fn deliver(&self) {
        println!("Delivering by sea in a container.");
    }
}

trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;
}

struct RoadLogistics;
impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck)
    }
}

struct SeaLogistics;
impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship)
    }
}

fn main() {
    let road = RoadLogistics;
    let sea = SeaLogistics;

    let t1 = road.create_transport();
    t1.deliver();

    let t2 = sea.create_transport();
    t2.deliver();
}
```


## 💠 C++ 예제
```cpp
#include <iostream>
#include <memory>

class Transport {
public:
    virtual void deliver() = 0;
    virtual ~Transport() = default;
};

class Truck : public Transport {
public:
    void deliver() override {
        std::cout << "Delivering by land in a box.\n";
    }
};

class Ship : public Transport {
public:
    void deliver() override {
        std::cout << "Delivering by sea in a container.\n";
    }
};

class Logistics {
public:
    virtual std::shared_ptr<Transport> createTransport() = 0;
    virtual ~Logistics() = default;
};

class RoadLogistics : public Logistics {
public:
    std::shared_ptr<Transport> createTransport() override {
        return std::make_shared<Truck>();
    }
};

class SeaLogistics : public Logistics {
public:
    std::shared_ptr<Transport> createTransport() override {
        return std::make_shared<Ship>();
    }
};

int main() {
    std::shared_ptr<Logistics> road = std::make_shared<RoadLogistics>();
    std::shared_ptr<Logistics> sea = std::make_shared<SeaLogistics>();

    road->createTransport()->deliver();
    sea->createTransport()->deliver();

    return 0;
}
```


### 🔧 왜 unique_ptr이 적합한가?
| 항목               | 설명                                           |
|--------------------|------------------------------------------------|
| `move`             | 소유권을 명확히 이전하여 객체 생명주기 관리 가능     |
| 자동 `delete`      | 객체 소멸 시 자동으로 메모리 해제 (RAII 원칙 적용)   |
| 예외 안전성        | 예외 발생 시에도 메모리 누수 없이 안전하게 처리       |
| `shared_ptr` 대비  | 참조 카운팅 오버헤드 없음, 단일 소유권에 더 적합      |


## 🧱 예제: Factory Method with unique_ptr
```cpp
#include <iostream>
#include <memory>

class Transport {
public:
    virtual void deliver() = 0;
    virtual ~Transport() = default;
};

class Truck : public Transport {
public:
    void deliver() override {
        std::cout << "Delivering by land in a box.\n";
    }
};

class Ship : public Transport {
public:
    void deliver() override {
        std::cout << "Delivering by sea in a container.\n";
    }
};

class Logistics {
public:
    virtual std::unique_ptr<Transport> createTransport() = 0;
    virtual ~Logistics() = default;
};

class RoadLogistics : public Logistics {
public:
    std::unique_ptr<Transport> createTransport() override {
        return std::make_unique<Truck>();
    }
};

class SeaLogistics : public Logistics {
public:
    std::unique_ptr<Transport> createTransport() override {
        return std::make_unique<Ship>();
    }
};

int main() {
    std::unique_ptr<Logistics> road = std::make_unique<RoadLogistics>();
    std::unique_ptr<Logistics> sea = std::make_unique<SeaLogistics>();

    auto t1 = road->createTransport();
    t1->deliver();

    auto t2 = sea->createTransport();
    t2->deliver();

    return 0;
}
```


### ✅ 결과
```
Delivering by land in a box.
Delivering by sea in a container.
```


## 🧱 C# 예제
```csharp
using System;

interface ITransport {
    void Deliver();
}

class Truck : ITransport {
    public void Deliver() => Console.WriteLine("Delivering by land in a box.");
}

class Ship : ITransport {
    public void Deliver() => Console.WriteLine("Delivering by sea in a container.");
}

abstract class Logistics {
    public abstract ITransport CreateTransport();
}

class RoadLogistics : Logistics {
    public override ITransport CreateTransport() => new Truck();
}

class SeaLogistics : Logistics {
    public override ITransport CreateTransport() => new Ship();
}

class Program {
    static void Main() {
        Logistics road = new RoadLogistics();
        Logistics sea = new SeaLogistics();

        road.CreateTransport().Deliver();
        sea.CreateTransport().Deliver();
    }
}
```


## 🐍 Python 예제
```python
from abc import ABC, abstractmethod

class Transport(ABC):
    @abstractmethod
    def deliver(self):
        pass

class Truck(Transport):
    def deliver(self):
        print("Delivering by land in a box.")

class Ship(Transport):
    def deliver(self):
        print("Delivering by sea in a container.")

class Logistics(ABC):
    @abstractmethod
    def create_transport(self) -> Transport:
        pass

class RoadLogistics(Logistics):
    def create_transport(self) -> Transport:
        return Truck()

class SeaLogistics(Logistics):
    def create_transport(self) -> Transport:
        return Ship()

def main():
    road = RoadLogistics()
    sea = SeaLogistics()

    road.create_transport().deliver()
    sea.create_transport().deliver()

if __name__ == "__main__":
    main()
```


## 🧠 패턴 요약
| 언어     | 추상 제품     | 구체 제품       | 추상 팩토리 | 구체 팩토리                  | 생성 방식 또는 반환 타입         |
|----------|---------------|------------------|--------------|-------------------------------|----------------------------------|
| Rust     | `Transport`   | `Truck`, `Ship`  | `Logistics`  | `RoadLogistics`, `SeaLogistics` | `Box<dyn Transport>`             |
| C++      | `Transport`   | `Truck`, `Ship`  | `Logistics`  | `RoadLogistics`, `SeaLogistics` | `std::shared_ptr<Transport>`     |
| C#       | `ITransport`  | `Truck`, `Ship`  | `Logistics`  | `RoadLogistics`, `SeaLogistics` | `CreateTransport()`              |
| Python   | `Transport`   | `Truck`, `Ship`  | `Logistics`  | `RoadLogistics`, `SeaLogistics` | `create_transport()`             |

---



