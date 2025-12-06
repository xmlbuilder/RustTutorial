# Decorator
Decorator 패턴을 Rust, C++, C#, Python 네 가지 언어로 구현한 예제.

## 🧠 Decorator 패턴 개요
Decorator 패턴은 객체에 기능을 동적으로 추가할 수 있게 해주는 구조적 디자인 패턴입니다.  
기존 객체를 변경하지 않고, **래핑(wrapping)** 을 통해 기능을 확장합니다.

## 🦀 Rust 예제
```rust
trait Beverage {
    fn cost(&self) -> f32;
    fn description(&self) -> String;
}

// 기본 객체
struct Coffee;
impl Beverage for Coffee {
    fn cost(&self) -> f32 {
        2.0
    }
    fn description(&self) -> String {
        "Coffee".to_string()
    }
}

// 데코레이터: Milk
struct Milk<'a> {
    beverage: &'a dyn Beverage,
}

impl<'a> Beverage for Milk<'a> {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.5
    }
    fn description(&self) -> String {
        format!("{} + Milk", self.beverage.description())
    }
}

// 데코레이터: Sugar
struct Sugar<'a> {
    beverage: &'a dyn Beverage,
}

impl<'a> Beverage for Sugar<'a> {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.3
    }
    fn description(&self) -> String {
        format!("{} + Sugar", self.beverage.description())
    }
}

fn main() {
    let coffee = Coffee;
    let with_milk = Milk { beverage: &coffee };
    let with_milk_and_sugar = Sugar { beverage: &with_milk };

    println!("Order: {}", with_milk_and_sugar.description());
    println!("Cost: ${}", with_milk_and_sugar.cost());
}
```

## ✅ Rust 설명
- trait Beverage는 공통 인터페이스입니다.
- Coffee는 기본 객체이며, Milk와 Sugar는 데코레이터입니다.
- 각 데코레이터는 &dyn Beverage를 참조하여 기능을 확장합니다.
- Rust에서는 상속 대신 트레잇과 참조 기반 래핑으로 데코레이터를 구현합니다.
- main()에서는 데코레이터를 중첩하여 기능을 누적합니다.

### 💠 C++ 예제
```cpp
#include <iostream>
#include <memory>
#include <string>

class Beverage {
public:
    virtual std::string description() const = 0;
    virtual float cost() const = 0;
    virtual ~Beverage() = default;
};

class Coffee : public Beverage {
public:
    std::string description() const override { return "Coffee"; }
    float cost() const override { return 2.0f; }
};

class Decorator : public Beverage {
protected:
    std::unique_ptr<Beverage> beverage;
public:
    Decorator(std::unique_ptr<Beverage> b) : beverage(std::move(b)) {}
};

class Milk : public Decorator {
public:
    Milk(std::unique_ptr<Beverage> b) : Decorator(std::move(b)) {}
    std::string description() const override {
        return beverage->description() + " + Milk";
    }
    float cost() const override {
        return beverage->cost() + 0.5f;
    }
};

class Sugar : public Decorator {
public:
    Sugar(std::unique_ptr<Beverage> b) : Decorator(std::move(b)) {}
    std::string description() const override {
        return beverage->description() + " + Sugar";
    }
    float cost() const override {
        return beverage->cost() + 0.3f;
    }
};

int main() {
    std::unique_ptr<Beverage> coffee = std::make_unique<Coffee>();
    coffee = std::make_unique<Milk>(std::move(coffee));
    coffee = std::make_unique<Sugar>(std::move(coffee));

    std::cout << "Order: " << coffee->description() << "\n";
    std::cout << "Cost: $" << coffee->cost() << "\n";
    return 0;
}
```


## 🧱 C# 예제
```csharp
using System;

interface IBeverage {
    string Description();
    float Cost();
}

class Coffee : IBeverage {
    public string Description() => "Coffee";
    public float Cost() => 2.0f;
}

abstract class Decorator : IBeverage {
    protected IBeverage beverage;
    public Decorator(IBeverage b) => beverage = b;
    public abstract string Description();
    public abstract float Cost();
}

class Milk : Decorator {
    public Milk(IBeverage b) : base(b) {}
    public override string Description() => beverage.Description() + " + Milk";
    public override float Cost() => beverage.Cost() + 0.5f;
}

class Sugar : Decorator {
    public Sugar(IBeverage b) : base(b) {}
    public override string Description() => beverage.Description() + " + Sugar";
    public override float Cost() => beverage.Cost() + 0.3f;
}

class Program {
    static void Main() {
        IBeverage coffee = new Coffee();
        coffee = new Milk(coffee);
        coffee = new Sugar(coffee);

        Console.WriteLine($"Order: {coffee.Description()}");
        Console.WriteLine($"Cost: ${coffee.Cost()}");
    }
}
```


## 🐍 Python 예제
```python
class Beverage:
    def description(self):
        raise NotImplementedError
    def cost(self):
        raise NotImplementedError

class Coffee(Beverage):
    def description(self):
        return "Coffee"
    def cost(self):
        return 2.0

class Decorator(Beverage):
    def __init__(self, beverage):
        self.beverage = beverage

class Milk(Decorator):
    def description(self):
        return self.beverage.description() + " + Milk"
    def cost(self):
        return self.beverage.cost() + 0.5

class Sugar(Decorator):
    def description(self):
        return self.beverage.description() + " + Sugar"
    def cost(self):
        return self.beverage.cost() + 0.3

def main():
    coffee = Coffee()
    coffee = Milk(coffee)
    coffee = Sugar(coffee)

    print(f"Order: {coffee.description()}")
    print(f"Cost: ${coffee.cost()}")

if __name__ == "__main__":
    main()

```

## 🧩 요약 비교
| 언어     | 공통 인터페이스     | 데코레이터 구조         | 객체 래핑 방식         | 특징                         |
|----------|----------------------|--------------------------|--------------------------|------------------------------|
| Rust     | `trait Beverage`     | `struct + impl`          | `&dyn Trait` 참조 기반   | 안전한 타입, 명확한 소유권     |
| C++      | `class Beverage`     | `Decorator` 상속 구조     | `unique_ptr`             | RAII 기반, 수동 메모리 관리     |
| C#       | `interface IBeverage`| `abstract Decorator` 클래스 | 참조 기반                | GC 기반, 명확한 구조           |
| Python   | `class Beverage`     | `Decorator` 클래스 상속   | 동적 타입으로 자유롭게 래핑 | 유연하지만 타입 안전성 낮음     |



## 🧠 C로 구현한 데코레이터 패턴 예제
커피 주문 시스템을 기반으로, Milk와 Sugar 데코레이터를 추가하는 구조입니다.
### 🔧 인터페이스 정의
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Beverage {
    float (*cost)(struct Beverage*);
    const char* (*description)(struct Beverage*);
    void* data;
} Beverage;
```

- Beverage는 인터페이스처럼 동작하는 구조체입니다.
- data는 실제 객체(Coffee, Milk 등)의 포인터를 담고 있고,
- cost와 description은 해당 객체의 동작을 가리키는 함수 포인터입니다.

### ☕ 기본 객체: Coffee
```c
typedef struct {
    const char* name;
    float base_cost;
} CoffeeData;

float coffee_cost(Beverage* b) {
    CoffeeData* data = (CoffeeData*)b->data;
    return data->base_cost;
}

const char* coffee_description(Beverage* b) {
    CoffeeData* data = (CoffeeData*)b->data;
    return data->name;
}

Beverage* create_coffee(const char* name, float cost) {
    Beverage* b = malloc(sizeof(Beverage));
    CoffeeData* data = malloc(sizeof(CoffeeData));
    data->name = name;
    data->base_cost = cost;
    b->data = data;
    b->cost = coffee_cost;
    b->description = coffee_description;
    return b;
}
```


### 🥛 데코레이터: Milk
```c
typedef struct {
    Beverage* base;
    const char* extra;
    float extra_cost;
} MilkData;

float milk_cost(Beverage* b) {
    MilkData* data = (MilkData*)b->data;
    return data->base->cost(data->base) + data->extra_cost;
}

const char* milk_description(Beverage* b) {
    MilkData* data = (MilkData*)b->data;
    static char buffer[128];
    snprintf(buffer, sizeof(buffer), "%s + %s", data->base->description(data->base), data->extra);
    return buffer;
}

Beverage* add_milk(Beverage* base) {
    Beverage* b = malloc(sizeof(Beverage));
    MilkData* data = malloc(sizeof(MilkData));
    data->base = base;
    data->extra = "Milk";
    data->extra_cost = 0.5f;
    b->data = data;
    b->cost = milk_cost;
    b->description = milk_description;
    return b;
}
```


### 🍬 데코레이터: Sugar
```c
typedef struct {
    Beverage* base;
    const char* extra;
    float extra_cost;
} SugarData;

float sugar_cost(Beverage* b) {
    SugarData* data = (SugarData*)b->data;
    return data->base->cost(data->base) + data->extra_cost;
}

const char* sugar_description(Beverage* b) {
    SugarData* data = (SugarData*)b->data;
    static char buffer[128];
    snprintf(buffer, sizeof(buffer), "%s + %s", data->base->description(data->base), data->extra);
    return buffer;
}

Beverage* add_sugar(Beverage* base) {
    Beverage* b = malloc(sizeof(Beverage));
    SugarData* data = malloc(sizeof(SugarData));
    data->base = base;
    data->extra = "Sugar";
    data->extra_cost = 0.3f;
    b->data = data;
    b->cost = sugar_cost;
    b->description = sugar_description;
    return b;
}
```


### 🧪 main() 함수
```c
int main() {
    Beverage* coffee = create_coffee("Coffee", 2.0f);
    Beverage* with_milk = add_milk(coffee);
    Beverage* with_milk_and_sugar = add_sugar(with_milk);

    printf("Order: %s\n", with_milk_and_sugar->description(with_milk_and_sugar));
    printf("Cost: $%.2f\n", with_milk_and_sugar->cost(with_milk_and_sugar));

    // 메모리 해제 생략 (실전에서는 반드시 free 필요)
    SugarData* sugar = (SugarData*)with_milk_and_sugar->data;
    MilkData* milk = (MilkData*)sugar->base->data;
    CoffeeData* coffee_data = (CoffeeData*)milk->base->data;

    free(coffee_data);
    free(milk->base);
    free(milk);
    free(sugar->base);
    free(sugar);
    free(with_milk_and_sugar);

    return 0;
}
```


### 🧩 하드웨어 업체에서의 활용
- 드라이버 확장: 기본 드라이버에 로깅, 보안, 캐싱 기능을 데코레이터처럼 추가
- 통신 프로토콜: 기본 송수신 구조에 암호화/압축 기능을 래핑
- UI 구성: 버튼에 테마, 애니메이션, 이벤트 핸들러를 데코레이터처럼 추가

---
