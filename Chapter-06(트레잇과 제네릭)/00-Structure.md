# struct

Rust의 struct는 C++의 struct나 class와 비슷하지만, 안전성과 명확성을 극대화한 설계를 갖고 있음.  
아래에 Rust의 struct에 대해 기본 개념부터 고급 기능까지 자세히 정리.

## 🧱 기본 개념: Rust의 struct
Rust에서 struct는 데이터를 묶는 사용자 정의 타입.  
C++의 struct처럼 필드를 갖지만, 상속은 없고, 구현은 명확하게 분리됩니다.  
```rust
struct Point {
    x: f64,
    y: f64,
}
```

- Point는 x와 y라는 필드를 가진 구조체
- 필드 타입은 명시적으로 지정
- 기본적으로 불변이며, mut 키워드로 가변 구조체로 만들 수 있음

## 🧠 구조체 사용 예시
```rust
fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("({}, {})", p.x, p.y);
}
```

- 구조체 인스턴스는 {}로 초기화
- 필드 접근은 . 연산자 사용

## 🔧 구조체에 메서드 구현
Rust는 구조체에 메서드를 붙일 때 impl 블록을 사용해요.
```rust
impl Point {
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

- self는 C++의 this와 유사
- &self는 불변 참조, &mut self는 가변 참조, self는 소유권 이동

## 🧩 다양한 구조체 형태
### 1. 튜플 구조체
```rust
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);
```

- 필드 이름 없이 순서로 접근 (red.0, red.1)

### 2. 유닛 구조체
```rust
struct Marker;
```

- 필드가 없는 구조체 → 타입 구분용, trait 구현용

### 3. 구조체 업데이트 문법
```rust
let p2 = Point { x: 5.0, ..p };
```

- 기존 인스턴스 p의 나머지 필드를 복사

## 🛡️ Rust struct의 특징
| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| let / let mut     | 구조체는 기본적으로 불변이며, `let mut`으로 가변 인스턴스를 생성할 수 있음 |
| Copy / Clone / Drop | 구조체는 소유권을 따르며, 필요한 경우 `Copy`, `Clone`, `Drop` 트레잇을 구현해 메모리 동작을 제어 |
| trait             | 구조체는 상속 대신 `trait`을 통해 기능을 확장하고 다형성을 구현함         |
| impl              | 구조체에 메서드를 붙일 때는 `impl` 블록을 사용하여 명확하게 구현           |
| match / if let    | 구조체는 패턴 매칭이 가능하여 `match`, `if let` 등을 통해 구조 분해 및 조건 분기 가능 |


## 🎯 고급 기능
### 1. Trait 구현
```rust
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

→ 구조체에 Display, Debug, Clone, PartialEq 등 trait을 붙여서 기능 확장
### 2. Derive 매크로
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: f64, y: f64 }
```

→ 반복적인 trait 구현을 자동으로 생성

## 💬 결론
Rust의 struct는 단순한 데이터 묶음이 아니라,  
안전성, 명확성, 표현력을 모두 갖춘 구조적 도구입니다.  
C++의 class보다 더 예측 가능하고 소유권과 수명까지 통제되는 철학적 구조.  

---
# `struct` / `impl` / `trait`
Rust를 처음 접하는 사람에게 struct, impl, trait를 설명하려면, 각 개념을 현실적인 비유로 연결하고,  
관계와 역할을 명확히 보여주는 방식이 가장 효과적.

## 🎯 핵심 비유: “구조체는 설계도, impl은 기능, trait은 규약”
### 1. struct = 데이터의 설계도
“struct는 어떤 데이터를 담을지 정의하는 설계도.”
```rust
struct Dog {
    name: String,
    age: u8,
}
```

- Dog는 이름과 나이를 담는 구조체
- C++의 struct와 비슷하지만, 상속은 없고 안전성이 높음

### 2. impl = 구조체에 기능을 붙이는 블록
“impl은 구조체에 동작을 붙이는 공간. 메서드를 여기에 정의.”
```rust
impl Dog {
    fn bark(&self) {
        println!("{} says woof!", self.name);
    }
}
```

- Dog에 bark()라는 기능을 추가
- self는 해당 구조체 인스턴스를 가리킴

### 3. trait = 기능의 규약, 인터페이스
“trait는 어떤 기능을 갖추고 있어야 하는지를 정의하는 규약. C++의 인터페이스.”
trait Animal {
    fn speak(&self);
}

- Animal이라는 trait은 speak()라는 기능을 요구함

### 4. impl Trait for Struct = 구조체가 규약을 따르게 만들기
“impl Trait for Struct는 구조체가 그 규약을 따르도록 기능을 구현하는 것임.”
```rust
impl Animal for Dog {
    fn speak(&self) {
        println!("{} says woof!", self.name);
    }
}
```

- 이제 Dog는 Animal trait을 따르므로
speak()를 호출할 수 있음

## 🧠 전체 흐름 요약
```rust
// 1. 데이터 설계
struct Dog { name: String, age: u8 }

// 2. 기능 추가
impl Dog {
    fn bark(&self) { println!("{} says woof!", self.name); }
}

// 3. 규약 정의
trait Animal {
    fn speak(&self);
}

// 4. 규약 구현
impl Animal for Dog {
    fn speak(&self) { println!("{} says woof!", self.name); }
}
```


## 💬 결론
처음 사용자에게는 struct는 “데이터를 담는 틀”,  
impl은 “그 틀에 기능을 붙이는 공간”,  
trait는 “기능의 규약”이라고 설명하면  
Rust의 구조적 설계를 쉽게 이해할 수 있음.

---
# Java / C# 비교

Rust의 struct, impl, trait 개념을 Java나 C#의 객체지향 관점에서 설명하면 훨씬 직관적으로 다가올 수 있음.  
아래에 각 개념을 Java/C#에 대응시켜서 설명.

## 🧱 Rust vs Java/C# 객체 구조 대응표
| Rust 개념               | Java/C# 대응 개념       | 설명 또는 키워드         |
|-------------------------|-------------------------|--------------------------|
| struct                  | class / POJO(Plain Old Java Object)     | 데이터 구조 정의   |
| impl                    | method 정의             | `this` ↔ `self`          |
| trait                   | interface               | 기능 규약, 다형성        |
| impl Trait for Struct   | implements Interface    | 인터페이스 구현          |

## 🎯 예시 비교: Java vs Rust
### Java 스타일
```java
interface Animal {
    void speak();
}

class Dog implements Animal {
    String name;

    Dog(String name) {
        this.name = name;
    }

    public void speak() {
        System.out.println(name + " says woof!");
    }
}
```

### Rust 스타일
```rust
struct Dog {
    name: String,
}

impl Dog {
    fn new(name: String) -> Self {
        Dog { name }
    }
}

trait Animal {
    fn speak(&self);
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says woof!", self.name);
    }
}
```


## 🧠 핵심 차이점
| 개념 비교             | Java/C#                      | Rust                          |
|-----------------------|------------------------------|-------------------------------|
| 클래스 상속           | extends                      | 없음 (trait으로 다형성 구현) |
| 생성자                | 생성자 메서드                | impl 블록 내 new() 메서드    |
| 인스턴스 참조         | this                         | self                          |
| 인터페이스 구현       | implements Interface         | impl Trait for Struct         |
| 메모리 제어           | GC 기반                      | Drop / Clone 트레잇 기반      |


## 💬 결론
Rust의 struct, impl, trait는 Java/C#의 class, method, interface와 유사하지만  
더 안전하고, 더 명시적이며, 더 예측 가능한 방식으로 설계.

---


# 🧠 Rust의 trait = “행동 기반 상속”
Rust는 전통적인 상속(class A extends B)은 없지만,  
trait을 통해 “행동의 공통성”으로 타입을 묶는 방식을 사용.  
예시: 여러 타입이 같은 trait을 구현
```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("Woof!"); }
}

impl Animal for Cat {
    fn speak(&self) { println!("Meow!"); }
}
```

→ Dog와 Cat은 Animal이라는 trait을 구현했기 때문에 “같은 종류의 객체”처럼 다룰 수 있음

## 🎯 trait object = 상속처럼 다형성 구현
```rust
fn make_it_speak(animal: &dyn Animal) {
    animal.speak();
}
```

- &dyn Animal은 trait object
- Dog, Cat 모두 Animal trait을 구현했기 때문에 make_it_speak()에 넘길 수 있음  
→ 이건 Java/C#에서 interface Animal을 구현한 클래스들을 Animal 타입으로 다루는 것과 거의 동일한 개념  

## 🔍 Rust의 철학적 차이
| 개념 비교             | Java/C#                      | Rust                              |
|-----------------------|------------------------------|-----------------------------------|
| 클래스 상속           | extends                      | 없음 (trait 기반 다형성)          |
| 인터페이스 구현       | implements Interface         | impl Trait for Struct             |
| 다형성 인스턴스 표현  | Animal animal = new Dog()    | &dyn Animal / Box<dyn Animal>     |
| 메모리 관리 방식      | GC 기반                      | 소유권 기반 + Drop / Clone 트레잇 |

## 💬 결론
Rust에서 trait은 단순한 인터페이스가 아니라, “같은 행동을 구현한 타입은 같은 종류로 본다”는 상속 개념의 대체물.

---

# runtime dispatch

**런타임 디스패치(runtime dispatch)** 는 객체지향 프로그래밍에서 **“어떤 메서드를 호출할지 실행 중에 결정하는 방식”** 을 말함.

## 🧠 쉽게 말하면…
“코드를 실행할 때, 실제 객체의 타입을 보고 어떤 메서드를 호출할지 결정하는 것”


## 🎯 예시: Java에서의 런타임 디스패치
```java
Animal a = new Dog();
a.speak();  // Dog의 speak()가 실행됨
```

- a는 Animal 타입이지만 실제 객체는 Dog
- 컴파일 시점에는 어떤 speak()가 호출될지 몰라요
- 실행 시점에 JVM이 Dog의 speak()를 찾아서 호출 → 런타임 디스패치

## 🦀 Rust에서의 런타임 디스패치
Rust는 기본적으로 **컴파일 타임 디스패치(static dispatch)**를 사용하지만,
dyn Trait을 쓰면 런타임 디스패치가 발생함.
```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn make_speak(animal: &dyn Animal) {
    animal.speak();  // 런타임에 어떤 speak()를 호출할지 결정됨
}
```

- &dyn Animal은 trait object
- make_speak()는 어떤 타입이 들어올지 모르기 때문에
실행 시점에 Dog의 speak()를 찾아서 호출 → 런타임 디스패치

## 🔍 정리: 런타임 vs 컴파일 타임 디스패치

| 디스패치 방식         | 결정 시점       | Rust 문법 예시       | 특징 및 용도                          |
|------------------------|------------------|-----------------------|----------------------------------------|
| 컴파일 타임 디스패치   | 컴파일 시점      | `impl Trait`          | 빠르고 안전함. 제네릭 기반 최적화 가능 |
| 런타임 디스패치        | 실행 시점        | `dyn Trait`           | 다형성 구현. trait object로 동적 호출   |



## 💬 결론
런타임 디스패치는 **다형성(polymorphism)**을 구현하는 핵심 기술.
Rust에서도 dyn Trait을 통해 객체지향 스타일의 다형성을 안전하게 구현할 수 있음.

---

## 🛠️ Rust에서 오버로딩 없이 비슷한 이름을 쓰는 방법
## 1. Trait으로 추상화
```rust
trait Drawable {
    fn draw_circle(&self);
}

impl Drawable for f32 { /* ... */ }
impl Drawable for f64 { /* ... */ }
```

→ draw_circle()이라는 이름은 유지하면서 타입에 따라 구현을 분리
## 2. enum으로 타입 구분

```rust
enum CircleSize {
    Small(f32),
    Large(f64),
}

fn draw_circle(size: CircleSize) {
    match size {
        CircleSize::Small(radius) => {
            println!("Drawing a small circle with radius: {}", radius);
        }
        CircleSize::Large(radius) => {
            println!("Drawing a large circle with radius: {}", radius);
        }
    }
}

fn main() {
    let c1 = CircleSize::Small(5.0);
    let c2 = CircleSize::Large(12.5);

    draw_circle(c1);
    draw_circle(c2);
}

```
→ 하나의 함수 이름으로 다양한 타입을 처리

---


