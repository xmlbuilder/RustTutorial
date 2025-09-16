# 🧠 dyn이란?
dyn Trait은 “어떤 타입이든 해당 트레잇을 구현하고 있으면 참조할 수 있다”는 뜻이에요.
하지만 구체적인 타입은 컴파일 타임에 알 수 없고, 런타임에 vtable을 통해 메서드를 호출합니다.


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

fn make_sound(animal: &dyn Animal) {
    animal.speak(); // 동적 디스패치
}
```

- &dyn Animal은 Dog, Cat, Bird 등 어떤 타입이든 받을 수 있어요.
- dyn을 붙이면 트레잇 객체가 되고, 런타임에 vtable을 통해 메서드가 호출됨

## 🔍 dyn vs impl Trait vs 제네릭
| 방식          | 디스패치 방식     | 성능         | 유연성 / 재사용성       | 특징 및 용도                                      |
|---------------|------------------|--------------|--------------------------|--------------------------------------------------|
| `dyn Trait`   | 동적 디스패치     | 느림 (vtable) | 높음 (다양한 타입 수용) | 런타임에 타입 결정, 트레잇 객체, 컬렉션에 적합     |
| `impl Trait`  | 정적 디스패치     | 빠름         | 낮음 (타입 고정)         | 함수 반환 타입에 주로 사용, 간결하고 직관적        |
| `<T: Trait>`  | 정적 디스패치     | 빠름         | 높음 (제네릭 활용 가능)  | 재사용성 높음, 복잡한 제약 조건 표현 가능          |


## 🧪 메모리 구조
dyn Trait은 두 개의 포인터를 가집니다:
- 데이터 포인터: 실제 객체 (Dog, Cat 등)
- vtable 포인터: 메서드 테이블 (어떤 메서드를 호출할지)
이 구조 덕분에 다양한 타입을 하나의 인터페이스로 다룰 수 있지만, 런타임 비용이 발생합니다.

## 📦 예시: Box<dyn Trait>
```rust
fn get_animal() -> Box<dyn Animal> {
    Box::new(Dog)
}
```

- Box<dyn Trait>은 힙에 저장된 트레잇 객체를 가리킴
- dyn이 없으면 컴파일러는 구체 타입을 요구함

## 🧩 언제 dyn을 써야 할까?
- 여러 타입을 동일한 인터페이스로 처리하고 싶을 때
- 트레잇을 컬렉션(Vec, HashMap 등)에 담고 싶을 때
- 런타임에 타입이 결정되는 구조가 필요할 때

## ✅ 실전 팁
- dyn Trait은 trait object를 만들기 위한 문법
- dyn을 쓰면 동적 디스패치가 발생하므로 성능이 중요할 땐 주의
- Arc<dyn Trait>, Box<dyn Trait>, &dyn Trait 등 다양한 포인터와 함께 사용됨
- dyn Trait을 쓰려면 트레잇이 object-safe해야 함 (예: self: Sized 금지)

--- 

# dyn을 중심으로 object safety, vtable, impl Trait과의 변환

Rust의 dyn을 중심으로 object safety, vtable, impl Trait과의 변환, 그리고 디자인 패턴 적용까지 확장해서 정리.

## 🧠 1. Object Safety란?
Object Safety는 트레잇을 dyn Trait 형태로 사용할 수 있는지 여부를 결정하는 규칙이에요.

### ✅ Object-safe 트레잇이란?
- dyn Trait로 사용할 수 있는 트레잇
- 즉, 트레잇 객체로 변환 가능한 트레잇
### ❌ Object-safe가 아닌 예시
```rust
trait NotSafe {
    fn new() -> Self; // ❌ Self 반환은 object-safe 아님
    fn clone(&self) -> Self; // ❌ Self 반환
}
```

### ✅ Object-safe 예시
```rust
trait Safe {
    fn draw(&self); // ✅ Self를 참조만 하면 OK
}
```

## 🔒 규칙 요약
| 조건                         | Object-safe 여부 | 설명                                      |
|------------------------------|------------------|-------------------------------------------|
| `&self`, `&mut self`         | ✅ 가능           | 트레잇 객체에서 안전하게 호출 가능         |
| `Self` 반환 또는 인자 사용   | ❌ 불가능         | 구체 타입을 요구하므로 트레잇 객체 불가     |
| 제네릭 메서드 (`fn foo<T>()`) | ❌ 불가능         | vtable로 호출할 수 없어 트레잇 객체 불가    |


## 🧪 2. VTable이란?
**VTable (Virtual Method Table)**은 dyn Trait이 런타임에 메서드를 호출할 수 있도록 해주는 구조입니다.

### 🔍 구조
- dyn Trait은 두 개의 포인터를 가짐:
- 데이터 포인터: 실제 객체
- vtable 포인터: 메서드 주소 테이블
```rust
let obj: Box<dyn Trait> = Box::new(MyType);
obj.method(); // vtable을 통해 런타임에 메서드 호출
```

- 이 방식은 동적 디스패치를 가능하게 하지만, 성능 비용이 발생할 수 있어요.

## 🔁 3. impl Trait ↔ dyn Trait 변환
Rust에서는 impl Trait과 dyn Trait을 서로 다른 목적으로 사용합니다.
| 방식         | 디스패치 방식     | 변환 가능 여부         | 설명                                      |
|--------------|------------------|------------------------|-------------------------------------------|
| `impl Trait` | 정적 디스패치     | ✅ → `Box<dyn Trait>` 가능 | 컴파일 타임에 타입 결정, 성능 우수           |
| `dyn Trait`  | 동적 디스패치     | ❌ → `impl Trait` 불가     | 런타임에 타입 결정, 유연하지만 성능 비용 있음 |


예시: impl Trait → Box<dyn Trait>
```rust
fn make_drawer() -> Box<dyn Drawable> {
    Box::new(Circle {})
}
```

- impl Trait은 반환 타입을 숨기지만, dyn Trait은 런타임에 타입을 결정합니다.

## 🧩 4. dyn을 활용한 디자인 패턴
Rust에서 dyn Trait은 다양한 디자인 패턴에 활용됩니다:

### ✅ Observer 패턴
```rust
trait Observer {
    fn update(&self, msg: &str);
}

type ObserverList = Vec<Box<dyn Observer>>;
```

- 다양한 타입의 옵저버를 하나의 리스트에 담을 수 있음

### ✅ Strategy 패턴
```rust
trait Strategy {
    fn execute(&self);
}

struct Context {
    strategy: Box<dyn Strategy>,
}
```

- 런타임에 전략을 바꿀 수 있음
### ✅ Command 패턴
```rust
trait Command {
    fn run(&self);
}

let commands: Vec<Box<dyn Command>> = vec![Box::new(PrintCommand {})];
```

- 다양한 명령을 하나의 인터페이스로 실행

## 🧠 요약
| 개념            | 설명                                      | 변환 가능성                     |
|-----------------|-------------------------------------------|----------------------------------|
| `dyn Trait`     | 런타임에 타입 결정, 동적 디스패치 사용       | ❌ `impl Trait`로 직접 변환 불가 |
| `impl Trait`    | 컴파일 타임에 타입 결정, 정적 디스패치 사용   | ✅ `Box<dyn Trait>`로 감싸면 가능 |
| `Box<dyn Trait>`| 힙에 저장된 트레잇 객체, `dyn Trait`의 대표 형태 | ✅ `impl Trait` → `Box<dyn Trait>`로 변환 가능 |

Rust는 dyn Trait을 통해 유연한 런타임 추상화를 제공하면서도,
object safety와 명시적 타입 제어를 통해 안전성과 성능을 놓치지 않아요.

---

# dyn Trait, impl Trait, <T: Trait>

Rust에서 dyn Trait, impl Trait, <T: Trait> 세 가지 방식의 선언과 사용을 각각 보여주는 예제입니다.
모두 같은 Drawable 트레잇을 기반으로 하되, 디스패치 방식과 유연성이 어떻게 다른지 직접 비교할 수 있게 구성.

## 🎨 공통 트레잇 정의
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
```

### 🧱 1. dyn Trait: 동적 디스패치, 다양한 타입 수용
```rust
fn draw_all(shapes: Vec<Box<dyn Drawable>>) {
    for shape in shapes {
        shape.draw(); // vtable 통해 런타임 호출
    }
}

fn dyn_trait_example() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    draw_all(shapes);
}
```

- 다양한 타입을 하나의 벡터에 담을 수 있음
- 런타임에 타입 결정 → 유연하지만 성능 비용 있음

### ⚡ 2. impl Trait: 정적 디스패치, 간결한 반환 타입
```rust
fn get_shape(use_circle: bool) -> impl Drawable {
    if use_circle {
        Circle
    } else {
        Square
    }
}

fn impl_trait_example() {
    let shape = get_shape(true);
    shape.draw(); // 컴파일 타임에 타입 결정
}
```
- 반환 타입을 숨기면서도 정적 디스패치 사용
- 단일 타입만 반환 가능 (여러 타입은 Box<dyn Trait> 필요)

### 🔧 3. <T: Trait>: 제네릭 기반, 재사용성 높음
```rust
fn draw_generic<T: Drawable>(shape: T) {
    shape.draw(); // 컴파일 타임에 타입 결정
}

fn generic_trait_example() {
    draw_generic(Circle);
    draw_generic(Square);
}
```

- 다양한 타입에 대해 재사용 가능
- 성능 우수, 제약 조건도 세밀하게 표현 가능

## ✅ 요약 비교
| 방식           | 디스패치 방식     | 성능             | 유연성 / 재사용성       | 특징 및 용도                                      |
|----------------|------------------|------------------|--------------------------|--------------------------------------------------|
| `dyn Trait`    | 동적 디스패치     | 느림 (vtable 사용) | 높음 (다양한 타입 수용) | 런타임에 타입 결정, 트레잇 객체, 컬렉션에 적합     |
| `impl Trait`   | 정적 디스패치     | 빠름             | 낮음 (타입 고정)         | 함수 반환 타입에 주로 사용, 간결하고 직관적        |
| `<T: Trait>`   | 정적 디스패치     | 빠름             | 높음 (제네릭 활용 가능)  | 재사용성 높음, 복잡한 제약 조건 표현 가능          |

----

# 🔍 Self: Sized 금지의 의미
## ✅ 기본 개념
- Rust에서 dyn Trait은 트레잇 객체로 사용되며, 런타임에 타입이 결정됩니다.
- 하지만 dyn Trait은 **크기를 알 수 없는 타입(unsized type)**이에요.
- 그런데 트레잇에 where Self: Sized 제약이 걸려 있으면, Rust는 해당 트레잇을 크기가 정해진 타입에서만 사용 가능하다고 판단해요.
📌 즉, Self: Sized가 있으면 dyn Trait으로 사용할 수 없어요 → object-safe가 아님

### ❌ 예시: object-safe가 아닌 트레잇
```rust
trait NotObjectSafe {
    fn clone_self(&self) -> Self; // Self 반환 → object-safe 아님
}
```

- Self를 반환하면 컴파일 타임에 구체 타입을 알아야 하므로 dyn Trait으로 사용할 수 없음
- 이럴 땐 Self: Sized를 명시해서 해당 메서드만 제외시킬 수 있어요:

```rust
trait PartiallySafe {
    fn draw(&self); // object-safe
    fn clone_self(&self) -> Self where Self: Sized; // object-safe에서 제외
}
```

- 이렇게 하면 draw()는 dyn Trait으로 호출 가능하고,
- clone_self()는 Sized 타입에서만 호출 가능해요

## ✅ 왜 중요한가?
| 조건                     | 영향                          | 설명                                                         |
|--------------------------|-------------------------------|--------------------------------------------------------------|
| `Self: Sized` 전체 트레잇에 적용 | ❌ `dyn Trait` 사용 불가         | 트레잇 객체는 크기를 알 수 없기 때문에 Sized 제약과 충돌함     |
| 특정 메서드에만 `Self: Sized` | ✅ `dyn Trait` 일부 사용 가능     | 해당 메서드는 제외되고 나머지 메서드는 트레잇 객체에서 사용 가능 |

이 제약을 잘 활용하면 트레잇 객체로 일부 기능만 노출하고,
구체 타입에서만 사용 가능한 기능은 숨길 수 있어요 → API 설계에 매우 유용!

### 가능 조건
| 조건 또는 조합               | 영향 또는 의미                          | 설명                                                         |
|-----------------------------|----------------------------------------|--------------------------------------------------------------|
| `Self: Sized`               | ❌ `dyn Trait` 사용 불가                 | 트레잇 전체가 크기 고정 타입만 허용되므로 트레잇 객체 불가     |
| `dyn Trait`                 | ✅ object-safe 트레잇만 사용 가능         | `Self` 반환, 제네릭 메서드 등은 포함되면 안 됨                |
| `dyn Trait + Self: Sized`   | ✅ 일부 메서드만 제외 가능               | `Self: Sized`를 특정 메서드에만 적용하면 나머지는 `dyn Trait` 가능 |


## 🔄 Rust vs C# 관점 비교
| 관점               | Rust (`dyn Trait`)             | C# (`interface`)            |
|--------------------|-------------------------------|-----------------------------|
| 추상화 방식         | 트레잇 객체 (Trait Object)      | 인터페이스 참조             |
| 디스패치 방식       | 동적 디스패치 (vtable)          | 동적 디스패치 (virtual call) |
| 대표 컬렉션 예시    | `Vec<Box<dyn Drawable>>`       | `List<IDrawable>`           |
| 목적               | 다양한 타입을 하나로 다루기      | 다형성 구현, 결합도 낮추기    |
| 타입 안전성         | 컴파일러가 object safety 검사    | 런타임 오류 가능성 있음       |
| 성능               | 약간의 런타임 비용 발생          | 일반적으로 최적화됨          |


## ✅ 왜 이렇게 쓰는가?
- 유연성: 다양한 구현체를 하나의 인터페이스로 다룰 수 있음
- 결합도 감소: 구체 타입에 의존하지 않으므로 테스트, 확장, 유지보수가 쉬움
- 추상화 강화: 동작 중심 설계가 가능해짐

## 🔍 Rust에서 dyn Trait을 쓰는 순간
```rust
fn render_all(shapes: Vec<Box<dyn Drawable>>) {
    for shape in shapes {
        shape.draw();
    }
}
```

### 이건 C#에서 다음과 같은 코드와 거의 같은 의미:
```rust
void RenderAll(List<IDrawable> shapes) {
    foreach (var shape in shapes) {
        shape.Draw();
    }
}
```

결국 Rust의 dyn Trait은 C#의 인터페이스 참조와 같은 역할을 하며,
**“구현체를 넘기지 말고 인터페이스를 넘겨라”**는 설계 철학을 그대로 반영하고 있어요.
