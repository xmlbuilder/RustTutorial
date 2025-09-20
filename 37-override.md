# override

Box<dyn Action>으로 가지고 있으면, 그 안에 어떤 타입이 들어있든 해당 타입이 구현한 run() 메서드가 자동으로 실행. 
이게 바로 Rust의 동적 디스패치(dynamic dispatch) 기능이에요.

## 🔍 작동 방법
```rust
trait Action {
    fn run(&self);
}

struct A;
struct B;

impl Action for A {
    fn run(&self) {
        println!("A runs fast");
    }
}

impl Action for B {
    fn run(&self) {
        println!("B runs slow");
    }
}

fn main() {
    let a: Box<dyn Action> = Box::new(A);
    let b: Box<dyn Action> = Box::new(B);

    a.run(); // prints "A runs fast"
    b.run(); // prints "B runs slow"
}
```

- Box<dyn Action>은 heap에 저장된 trait 객체
- .run()을 호출하면 실제 타입에 맞는 구현이 자동으로 실행
- 이건 C++의 virtual function이나 Java의 override와 거의 같은 개념

## ✅ 요약
| 개념               | 설명                                               |
|--------------------|----------------------------------------------------|
| `Box<dyn Trait>`   | 다양한 타입을 하나의 trait 객체로 통합 가능         |
| 런타임 다형성       | 실행 중에도 실제 타입에 맞는 메서드가 자동 실행됨   |
| 메서드 override    | 각 타입이 trait 메서드를 자신만의 방식으로 구현 가능 |
| swap 가능          | `Box<dyn Trait>`끼리는 타입이 같아 안전하게 교환 가능 |


Rust에서도 OOP 스타일의 다형성을 안전하고 성능 좋게 구현할 수 있다.

## 🔍 왜 바로 호출이 가능할까?
let a: Box<dyn Action> = Box::new(A);
a.run(); // 바로 실행됨!

- Box<dyn Action>는 내부적으로 **vtable(가상 메서드 테이블)**을 가지고 있음
- .run()을 호출하면 Rust는 vtable을 통해 실제 타입(A)의 run 구현을 찾아서 실행
- 이건 C++의 virtual function, Java의 override와 거의 같은 방식

## 🧠 메모리 관점에서 보면
| 구성 요소         | 설명                                               |
|------------------|----------------------------------------------------|
| `Box<dyn Trait>` | heap에 저장된 trait 객체의 참조                     |
| `dyn Trait`      | 런타임에 실제 타입을 추상화하는 trait 인터페이스    |
| `.run()`         | vtable을 통해 실제 타입의 구현된 메서드가 실행됨    |


## ✅ 요약
| 개념               | 설명                                               |
|--------------------|----------------------------------------------------|
| 참조 기반 호출      | `Box`는 heap 참조지만 `.run()`은 바로 호출 가능     |
| 동적 디스패치       | 런타임에 실제 타입에 맞는 메서드가 자동 실행됨     |
| 안전한 추상화       | 타입을 몰라도 trait만 알면 행동을 호출할 수 있음   |

---
