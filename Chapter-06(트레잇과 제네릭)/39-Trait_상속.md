# Trait 상속
## trait 상속 문법과 의미
- trait B: A { ... }는 **B가 A를 상속한다(supertrait)** 는 의미입니다.
- 이 문법은 **B를 구현하려면 A도 반드시 구현되어 있어야 한다** 는 제약을 선언합니다.
- 즉, 어떤 타입 T가 B를 구현하려면 동일한 타입 T가 A도 구현해야 합니다.

## 핵심 개념 요약
- 상속 관계: B는 A의 **슈퍼트레이트(supertrait)** 를 갖습니다. B: A는 **B에는 A가 필요하다** 는 선언입니다.
- 구현 제약: impl B for T를 작성하기 전에, impl A for T가 성립해야 합니다. 둘을 동시에 구현해도 되지만, 논리적으로 A가 선행 조건입니다.
- 메서드 이용: B의 디폴트 메서드 구현 안에서 A의 메서드를 직접 사용할 수 있습니다. 왜냐면 Self: A가 항상 참이기 때문입니다.
- 다중 슈퍼트레이트: trait B: A + C { ... }처럼 여러 개를 나열할 수 있습니다.
- 호출자 관점: 제네릭 바운드에서 T: B가 주어지면, 자동으로 T: A도 참입니다.
- 따라서 A의 메서드도 안전하게 사용할 수 있습니다.

### 예제로 보는 동작 방식
```rust
trait A {
    fn a(&self) -> String;
}
```
```rust
trait B: A {
    fn b(&self) -> String;

    // B의 디폴트 메서드에서 A의 메서드를 활용
    fn ab(&self) -> String {
        format!("{} + {}", self.a(), self.b())
    }
}
```
```rust
struct X;

impl A for X {
    fn a(&self) -> String { "from A".into() }
}

impl B for X {
    fn b(&self) -> String { "from B".into() }
}

fn use_b<T: B>(t: T) {
    // T: B 이므로 T: A 도 자동으로 만족
    println!("{}", t.ab()); // "from A + from B"
}
```

- 포인트: B의 디폴트 메서드 ab가 self.a()를 호출합니다. 이는 B: A 덕분에 항상 유효합니다.
- 제네릭 바운드: use_b<T: B> 안에서는 T: A가 추가로 필요하지 않습니다. 이미 B가 A를 요구하기 때문입니다.

### trait b가 갖는 제약과 함의
- 구현 제약: B를 구현하는 타입은 반드시 A도 구현해야 합니다. 이는 컴파일러가 강제합니다.
- API 계약 강화: B를 요구하는 함수나 모듈은, 자연스럽게 A의 기능도 이용 가능한 계약을 갖습니다. 설계 관점에서 계층적인 능력(기능)을 쌓아가는 데 유용합니다.
- 디폴트 구현의 의존성: B 안의 디폴트 메서드들은 A의 메서드에 의존할 수 있습니다. 덕분에 상위 행동(공통 인터페이스) 위에 확장 행동을 깔끔히 정의 가능합니다.
- 다중 상속 시의 일관성: trait B: A + C처럼 여러 슈퍼트레이트를 요구하면, 구현 타입은 모두 충족해야 합니다. 각 슈퍼트레이트의 객체 안전성, 연관 타입, 제네릭 요구사항도 함께 고려됩니다.
- 객체 안전성과 트레이트 오브젝트: dyn B를 사용할 때 B와 그 슈퍼트레이트(A, C 등)가 객체 안전(object-safe)해야 합니다. 객체 안전이 깨지면 dyn B 타입으로 사용할 수 없습니다.
- orphan rule은 동일: 외부 타입에 외부 트레이트를 구현할 수 없는 고유 규칙(고아 규칙)은 그대로 적용됩니다. B: A 관계가 이를 완화하지 않습니다.

### 이 문법을 어떻게 부르나
- 정식 명칭: **슈퍼트레이트(supertrait) 문법** 또는 **트레이트 상속(trait inheritance)**.
- 설명 방식: **B는 A를 슈퍼트레이트로 요구한다** 또는 **B는 A를 상속한다** 라고 부릅니다.

### 설계 시 팁
- 기능 계층화: 공통 핵심 행동을 A로, 그 위에 확장/조합된 행동을 B로 정의하면 API가 선명해집니다.
- 다중 요구는 신중히: A + C + D처럼 요구가 많아질수록 구현 난이도와 결합도가 올라갑니다. 최소한으로 유지 필요.
- 객체 안전 체크: 런타임 폴리모르피즘(&dyn B)이 필요하면, B와 슈퍼트레이트의 객체 안전성을 먼저 점검.
- 디폴트 구현 적극 활용: B의 디폴트 메서드에서 A를 활용해 중복을 줄이고 일관된 동작을 제공.


---

## override

- trait의 메서드는 기본 구현(default implementation)을 가질 수 있는데, 이를 구현체에서 다시 정의(override) 할 수 있습니다.
- 즉, trait B: A처럼 슈퍼트레이트 관계가 있더라도, 각 구현체는 trait에 정의된 기본 메서드를 그대로 쓰거나 자신만의 버전으로 덮어쓸 수 있습니다.

### 기본 구현과 override 동작 방식
```rust
trait A {
    fn a(&self) -> String {
        "default A".into()
    }
}
```
```rust
trait B: A {
    fn b(&self) -> String {
        "default B".into()
    }

    fn ab(&self) -> String {
        format!("{} + {}", self.a(), self.b())
    }
}
```
```rust
struct X;

// A의 기본 구현을 override
impl A for X {
    fn a(&self) -> String {
        "X's A".into()
    }
}

// B의 기본 구현을 override
impl B for X {
    fn b(&self) -> String {
        "X's B".into()
    }
}

fn main() {
    let x = X;
    println!("{}", x.ab()); // "X's A + X's B"
}
```
- trait 안에서 제공한 default 메서드는 선택적으로 override 가능합니다.
- impl 블록에서 같은 시그니처로 메서드를 정의하면, 기본 구현 대신 그 버전이 사용됩니다.
- 슈퍼트레이트(A)의 메서드도 동일하게 override 가능합니다.

### 제약 사항
- 필수 메서드: trait에서 기본 구현이 없는 메서드는 반드시 구현해야 합니다.
- 선택적 메서드: 기본 구현이 있는 메서드는 구현하지 않아도 되지만, 필요하다면 override 가능합니다.
- 호출 시점: trait 바운드로 호출할 때는 override된 버전이 실행됩니다.
- 명시적 호출: A::a(&x)처럼 trait 이름을 붙여 호출하면, override된 버전이 아니라 특정 trait의 구현을 직접 호출할 수도 있습니다.

- 👉 정리하면, Rust trait의 기본 메서드는 override 가능하며, trait B: A 관계에서도 동일하게 적용됩니다.  
  즉, 슈퍼트레이트의 메서드든, 서브트레이트의 메서드든 구현체에서 자유롭게 덮어쓸 수 있습니다.



---

## trait 간 override (안됨)

- Rust에서는 **trait B: A** 라고 선언했을 때, B가 A의 메서드를 override(덮어쓰기)하는 것은 **불가능** 합니다.

### 이유
- trait B: A는 **상속처럼 보이지만, 실제로는 "슈퍼트레이트(supertrait) 요구"** 일 뿐입니다.
- 즉, B를 구현하려면 A도 반드시 구현해야 한다는 제약을 뜻하지, B가 A의 메서드 정의를 바꿀 수 있다는 의미는 아닙니다.
- Rust의 trait 시스템은 **구현체(impl)** 에서만 메서드 override가 가능합니다.
- 따라서 trait B 안에서 trait A의 메서드를 다시 정의하거나 덮어쓸 수는 없습니다.

### 예시로 확인
```rust
trait A {
    fn a(&self) -> String {
        "default A".into()
    }
}
```
```rust
trait B: A {
    // 여기서 fn a(&self) { ... } 를 다시 정의할 수 없음
    fn b(&self) -> String {
        "default B".into()
    }
}
```
```rust
struct X;

impl A for X {
    fn a(&self) -> String {
        "X's A".into()
    }
}
```
```rust
impl B for X {
    fn b(&self) -> String {
        "X's B".into()
    }
}
```
- trait B 안에서 fn a(&self)를 다시 작성하려고 하면 컴파일 에러가 납니다.
- impl A for X에서만 a를 override할 수 있습니다.

## 정리
- 가능한 override 위치: impl 블록에서만 가능
- 불가능한 override 위치: 다른 trait(B)에서 슈퍼트레이트(A)의 메서드를 덮어쓰는 것
- 이 문법의 이름: **슈퍼트레이트(supertrait)** 문법
- 결론: trait B는 trait A의 메서드를 override할 수 없고, 단지 A를 요구할 뿐입니다.

- 👉 그러니까 trait B는 A의 기능을 확장하거나 새로운 메서드를 추가할 수는 있지만, A의 메서드를 바꿀 수는 없습니다.

---






