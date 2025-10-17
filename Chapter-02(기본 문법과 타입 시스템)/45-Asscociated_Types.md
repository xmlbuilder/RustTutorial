# Associated Types

아래는 Rust에서의 **공유 타입(Associated Types)** 에 대한 핵심 정리입니다.  
실무에서 trait를 설계하거나 추상화를 할 때 꼭 알아야 할 개념.

## 🔍 공유 타입 (Associated Types)란?
- trait 내부에서 타입을 추상화할 수 있는 방식
- type Output;처럼 placeholder 타입을 선언하고  
    → 실제 타입은 impl에서 정의함
- 제네릭 파라미터보다 더 명확하고 제한적인 타입 연결을 제공

## ✅ 예시 분석
```rust
#[derive(Debug)]
struct Meters(i32);

#[derive(Debug)]
struct MetersSquared(i32); // ✅ 반드시 정의 필요

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}
```

- Multiply trait은 multiply() 메서드를 정의하면서 결과 타입을 Self::Output으로 추상화
- Output은 구현체가 결정하는 타입

```rust
impl Multiply for Meters {
    type Output = MetersSquared;

    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}
```

- Meters끼리 곱하면 MetersSquared가 나오는 구조
- Output 타입이 MetersSquared로 고정됨

## 🧠 왜 Associated Types를 쓰는가?
| 목적 또는 장점       | 설명 또는 예시                              |
|----------------------|----------------------------------------------|
| 추상화               | 타입을 trait 내부에서 placeholder로 선언 가능 |
| 명확한 관계 표현     | `trait Multiply<T> -> U`보다 `type Output`이 더 직관적 |
| 제네릭보다 간결함    | 타입 파라미터 없이 trait 내에서 타입 연결 가능 |
| 인터페이스 설계에 적합 | `Iterator`, `Future`, `Add`, `Mul` 등에서 사용됨 |


## 🔧 실무에서 자주 보는 패턴
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- Item은 각 iterator가 반환하는 타입
- Vec<i32>의 iterator는 Item = i32

## 🔚 요약: Associated Types

| 항목             | 설명 또는 예시                          |
|------------------|------------------------------------------|
| 선언 방식         | `type Output;` → trait 내부에서 타입 추상화 |
| 구현 방식         | `type Output = ConcreteType;` → 실제 타입 지정 |
| 사용 목적         | 타입 관계 명확화, 제네릭보다 간결한 표현 |

단순히 **타입을 추상화한다** 가 아니라  
**“타입 간의 관계를 trait 수준에서 명확히 고정하고, 구현체마다 다르게 정의할 수 있다”** 는 점을 정확히 이해.

---
# 반환할 타입 고정의 잇점

Associated Type을 쓰는 순간, 그 trait의 구현체가 반환할 타입을 명확히 고정하게 됩니다.  
즉, Meters.multiply(&Meters)는 반드시 MetersSquared를 반환해야 하고,  
다른 타입이 끼어들 여지가 없습니다.

## 🔒 타입 관계를 강하게 고정하는 방식
```rust
trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}
```

- 이 구조는 **Multiply를 구현한 타입은 반드시 Output을 정의해야 한다** 는 강제 조건
- impl Multiply for Meters에서 type Output = MetersSquared;라고 명시하면    
    → Meters.multiply()는 무조건 MetersSquared를 반환해야 함

## ✅ 실무에서 이게 왜 중요한가?
| 목적 또는 효과        | 설명 또는 예시                                           |
|-----------------------|----------------------------------------------------------|
| API 안정성            | 반환 타입이 고정되어 있어 인터페이스가 흔들리지 않음       |
| 오용 방지             | 다른 타입이 끼어들 수 없도록 설계 단계에서 차단됨           |
| 타입 추론 명확화      | 컴파일러가 정확한 타입을 추론할 수 있어 오류 감소            |
| 유지보수 용이         | 타입 관계가 trait에 고정되어 있어 변경 시 영향 범위가 명확함 |
| 테스트 신뢰성         | 반환 타입이 예측 가능하므로 테스트 코드가 안정적으로 동작함  |

## 🧠 비교: 제네릭 방식과의 차이
```rust
trait Multiply<T> {
    fn multiply(&self, other: &Self) -> T;
}
```
- 이 방식은 T가 외부에서 주입되므로  
    → 타입 관계가 느슨하고, 오용 가능성 있음
- 반면 Associated Type은 타입 관계를 trait 내부에서 고정함

---

