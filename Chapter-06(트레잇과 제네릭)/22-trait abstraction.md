# trait abstraction (triat impl)

## 코드
```rust
pub trait HomogeneousPoint {
    fn to_h4(&self) -> Point4D;
    fn from_h4(p: Point4D) -> Self;
}


impl HomogeneousPoint for Point4D {
    #[inline] fn to_h4(&self) -> Point4D { *self }
    fn from_h4(p: Point4D) -> Self {
        p
    }
}

impl HomogeneousPoint for Point3D {
    #[inline] fn to_h4(&self) -> Point4D { Point4D::new(self.x, self.y, self.z, 1.0) }
    fn from_h4(p: Point4D) -> Self {
        let w = if p.w.abs() < 1e-15 { 1.0 } else { p.w };
        Self {
            x: p.x / w,
            y: p.y / w,
            z: p.z / w,
        }
    }
}
```

"HomogeneousPoint 트레이트를 Point3D와 Point4D에 동시에 적용"하는  
구조는 Rust에서 trait abstraction 또는 trait-based polymorphism이라고 부릅니다.

## ✅ 정확한 용어 정리
| 개념              | 설명                                     | 예시 코드 또는 표현                          |
|-------------------|------------------------------------------|----------------------------------------------|
| Trait             | 공통 인터페이스 정의                      | HomogeneousPoint                             |
| Trait 구현        | 특정 타입이 해당 trait를 따름             | impl HomogeneousPoint for Point3D            |
| Trait Object      | 여러 타입을 하나처럼 다루는 동적 디스패치 | Box<dyn HomogeneousPoint>                    |
| Trait Bound       | 제네릭 타입에 trait 제약을 거는 방식      | T: HomogeneousPoint                          |
| Trait Polymorphism| 여러 타입을 동일한 방식으로 처리          | T::to_h4() → Point3D 또는 Point4D            |


## 💡 자꾸 구조체(structure)처럼 느껴지는 이유
- HomogeneousPoint는 구조체처럼 필드를 가진 게 아니라,
**행동(behavior)**을 정의하는 인터페이스예요
- Rust는 클래스가 없기 때문에
trait가 OOP의 인터페이스 + 추상 클래스 역할을 동시에 합니다
- 그래서 처음엔 struct처럼 느껴지지만
실제로는 행동을 추상화하는 도구입니다

## ✨ 쉽게 기억하는 방법
struct는 데이터를 담는 그릇
trait는 그릇이 할 수 있는 행동을 정의

## 예시:
```rust
struct Point3D { x: f64, y: f64, z: f64 } // 데이터
trait HomogeneousPoint { fn to_h4(&self) -> Point4D } // 행동
```
Point3D와 Point4D를 하나의 공통 인터페이스로 추상화해서 DSL 시스템에 통합한 것이에요.
이건 Rust에서 가장 강력한 설계 패턴 중 하나입니다


##  왜 자꾸 구조처럼 느껴질까?
- C++에서는 “클래스 = 구조 + 기능”이었죠
- Rust는 “struct = 데이터”, “trait = 기능”으로 분리되어 있어서
처음엔 구조체에 기능을 넣고 싶어지는 게 당연
- 그런데 Rust는 trait로 기능을 먼저 정의하고,
그걸 여러 타입에 동적으로 적용하는 방식이 더 강력

## ✅ 사고 방식 전환 팁
| C++ 스타일 사고           | Rust 스타일 사고                     | 설명                                                   |
|---------------------------|--------------------------------------|--------------------------------------------------------|
| 구조에 기능을 넣자        | 기능을 trait로 추상화하자            | struct 중심 → trait 중심 사고로 전환                   |
| 타입이 메서드를 가져야 해 | trait을 구현하면 메서드를 쓸 수 있어 | 타입이 아니라 trait이 행동을 정의                      |
| 구조 먼저 만들자          | 기능 먼저 정의하자                   | 데이터보다 행동을 먼저 설계                            |
| 클래스처럼 생각하자       | 인터페이스처럼 생각하자              | Rust에는 클래스가 없고, trait이 인터페이스 역할 수행   |
| 상속으로 확장하자         | trait으로 조합하자                   | Rust은 상속 대신 trait 조합으로 확장성 확보            |

## ✨ 예시로 풀어볼게요
```rust
trait HomogeneousPoint {
    fn to_h4(&self) -> Point4D;
    fn from_h4(p: Point4D) -> Self;
}
```

→ 이건 “동차 좌표로 변환할 수 있는 타입”이라는 기능을 정의한 것
→ Point3D, Point4D, Vertex, ControlPoint 등 구조는 다양해도 기능은 같음

## 💬 내 생각
Rust의 철학을 진짜로 이해하려는 과정.
구조가 아니라 기능 중심으로 사고를 전환

---

# trait / impl
Rust의 impl을 비유로 설명하면, 가장 직관적인 비유는 **“도구 상자에 기능을 넣는 행위”**입니다.

## 🧰 비유: impl은 도구 상자에 기능을 채워 넣는 것

- struct는 빈 상자예요 → 데이터만 담겨 있음
- trait는 도구 설명서예요 → 어떤 기능이 있어야 하는지 정의
- impl은 도구를 상자에 실제로 넣는 행위예요 → 기능을 구현해서 사용할 수 있게 만듦
```rust
struct Point3D { x: f64, y: f64, z: f64 } // 상자
trait HomogeneousPoint { fn to_h4(&self) -> Point4D } // 설명서
impl HomogeneousPoint for Point3D { ... } // 도구를 넣음
```

## 비교 설명
| 비유                     | trait 설명                         | impl 설명                                  |
|--------------------------|-------------------------------------|---------------------------------------------|
| 배우와 대본              | 대본은 역할을 정의한다              | 배우가 대본대로 연기한다                    |
| 콘센트와 플러그          | 콘센트는 어떤 전기가 필요한지 정의  | 플러그를 꽂아 실제로 전기를 흐르게 한다     |
| 계약서와 서명            | 계약 조건을 정의한다                | 서명하면 책임지고 실행한다                  |
| 레고 블럭과 기능 부품    | 어떤 기능이 필요한지 정의한다       | 기능 부품을 블럭에 붙여서 작동하게 만든다   |
| 구조체와 기능 추가       | 구조체는 데이터만 가진다            | impl로 기능을 추가해서 행동하게 만든다      |


## 💬 내 생각
impl은 단순히 “코드 구현”이 아니라
타입에게 능력을 부여하는 행위.
Rust는 이걸 통해 구조와 기능을 분리하면서도
아주 강력한 추상화를 가능하게 합니다.


```mermaid
classDiagram
    class HomogeneousPoint {
        +to_h4() Point4D
        +from_h4(p: Point4D) Self
    }

    class Point3D {
        +x: f64
        +y: f64
        +z: f64
    }

    class Point4D {
        +x: f64
        +y: f64
        +z: f64
        +w: f64
    }

    HomogeneousPoint <|.. Point3D
    HomogeneousPoint <|.. Point4D
  ```

---
