# Trait Objet

- Rust에서 말하는 **객체 안전(object safety)** 과 **트레이트 오브젝트(dyn Trait)** 의 관계를 풀어서 설명.

## 🧩 트레이트 오브젝트란?
- dyn Trait은 런타임에 어떤 타입인지 모르지만 Trait을 만족하는 값을 가리키는 포인터 같은 개념입니다.
- 예:

```rust
trait Draw {
    fn draw(&self);
}

fn render(x: &dyn Draw) {
    x.draw(); // 구체 타입은 모르지만 Draw를 만족하므로 호출 가능
}
```

## 🔒 객체 안전(object safety)이란?
- Rust는 모든 trait을 dyn Trait으로 쓸 수 있게 허용하지 않습니다.
- **이 trait을 트레이트 오브젝트로 안전하게 사용할 수 있는가?** 를 검사하는 규칙이 있습니다.
- 이를 **객체 안전성(object safety)** 이라고 부릅니다.
- 객체 안전성을 깨뜨리는 경우:
  - Self 타입을 반환하는 메서드

```rust      
trait Bad {
    fn make(&self) -> Self; // ❌ dyn Bad에서는 어떤 구체 타입인지 알 수 없음
}
```
  - 제네릭 메서드
```rust
trait Bad {
    fn foo<T>(&self, x: T); // ❌ dyn Bad에서는 T를 알 수 없음
}
```
  - Sized 요구
```rust
trait Bad: Sized { ... } // ❌ dyn Bad는 Sized가 아님
```

- 즉, 트레이트 오브젝트로 쓸 때 컴파일러가 런타임 디스패치(vtable 호출)를 안전하게 할 수 없는 경우는 객체 안전하지 않다고 합니다.

### 🧾 질문 속 문장의 의미
- **dyn B를 사용할 때 B와 그 슈퍼트레이트(A, C 등)가 객체 안전(object-safe)해야 합니다. 객체 안전이 깨지면 dyn B 타입으로 사용할 수 없습니다.**
- 이 말은:
- trait B: A + C라고 했을 때, dyn B를 쓰려면 B뿐 아니라 A, C도 모두 객체 안전해야 한다는 뜻입니다.
- 왜냐면 dyn B는 사실상 "dyn A + dyn C"의 기능도 포함해야 하기 때문입니다.
- 만약 A나 C가 객체 안전하지 않으면, dyn B도 쓸 수 없습니다.

### 🎯 예시
```rust
trait A {
    fn a(&self) -> String;
}
```
```rust     
trait B: A {
    fn b(&self) -> String;
}
```
```rust
fn use_dyn(x: &dyn B) {
    println!("{}", x.a()); // A의 메서드도 호출 가능
    println!("{}", x.b());
}
```
- 여기서는 A와 B 모두 객체 안전하므로 &dyn B 사용 가능.
- 만약 A가 fn make(&self) -> Self 같은 메서드를 가진다면, dyn B는 쓸 수 없음.

## ✅ 정리
- 객체 안전성(object safety): dyn Trait으로 사용할 수 있는지 여부를 결정하는 규칙.
- dyn B 사용 조건: B뿐 아니라 B가 요구하는 모든 슈퍼트레이트(A, C 등)도 객체 안전해야 함.
- 안전하지 않으면: dyn B 타입으로는 쓸 수 없고, 컴파일 에러 발생.

---
## Generic & Trait 안정성

- Rust에서 generic 메서드와 **트레이트 오브젝트(dyn Trait)** 는 서로 성격이 다릅니다.
- 이 차이 때문에 **객체 안전성(object safety)** 문제는 주로 generic 메서드가 trait 안에 있을 때 발생합니다.

### 🔍 왜 generic이 문제를 일으키는가?
- **트레이트 오브젝트(dyn Trait)** 는 런타임에 vtable을 통해 메서드를 호출합니다. 즉, 컴파일 시점에 구체 타입을 몰라도 됩니다.
- 그런데 generic 메서드는 컴파일러가 제네릭 파라미터(T)에 대해 **모든 가능한 타입에 대해 코드 생성(monomorphization)** 을 해야 합니다.
- dyn Trait은 런타임에 **어떤 타입인지 모른다** 는 전제를 가지므로, 제네릭 메서드의 구체 타입을 알 수 없어 vtable에 안전하게 넣을 수 없습니다.
- 따라서 generic 메서드가 trait에 있으면 그 trait은 객체 안전하지 않음 → dyn Trait으로 사용할 수 없음.

### 📌 예시
```rust
trait Bad {
    fn foo<T>(&self, x: T); // generic 메서드
}
```
```rust
fn use_trait(x: &dyn Bad) {
    // ❌ 컴파일 에러: Bad는 객체 안전하지 않음
}
```

- 반면에 generic이 없는 trait은 안전합니다:
```rust
trait Good {
    fn foo(&self, x: i32); // 구체 타입
}
```
```rust
fn use_trait(x: &dyn Good) {
    x.foo(42); // ✅ 가능
}
```


## ✅ 정리
- generic 메서드가 있는 trait → dyn Trait 불가능 (객체 안전성 깨짐).
- generic 없는 trait → dyn Trait 가능.
- 따라서 generic을 많이 쓰는 trait은 트레이트 오브젝트로 사용하기 어렵다.
- 하지만 generic은 컴파일 시점 최적화에 강점이 있고, dyn Trait은 런타임 다형성에 강점이 있습니다.
- 둘은 용도가 다르기 때문에 상황에 맞게 선택하는 것이 중요합니다.

- 👉 결론적으로, generic을 쓰면 문제가 생길 소지가 높다는 건 맞습니다.
- 다만 그 문제는 **트레이트 오브젝트로 쓸 때** 에 한정되고, 일반적인 제네릭 기반 코드에서는 오히려 성능과 타입 안정성을 보장해 주는 장점이 있습니다.

---


