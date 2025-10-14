# std::slice::IterMut

std::slice::IterMut 는 Rust에서 가변 참조를 반복(iterate)할 수 있게 해주는 반복자 타입.  
즉, Vec<T>나 [T] 같은 슬라이스에서 각 요소를 &mut T 형태로 하나씩 꺼내서 수정할 수 있게 해주는 도구.

## 🔧 핵심 개념
```rust
let mut data = [1, 2, 3];
for x in data.iter_mut() {
    *x *= 2;
}
```

- iter_mut()은 IterMut<'_, T> 타입을 반환
- 각 요소는 `&mut T` 로 접근 가능 → 직접 수정 가능
- for 루프, map, enumerate, zip 등과 함께 사용 가능

## 🧠 언제 쓰나?
| 상황                             | 설명 또는 이유                                      |
|----------------------------------|-----------------------------------------------------|
| 슬라이스의 요소를 직접 수정할 때 | `&mut T`로 접근해야 하므로 `IterMut` 필요           |
| 조건부로 수정할 때               | `if let`, `match`와 함께 `&mut T`를 안전하게 처리   |
| 인덱스 기반으로 수정할 때        | `enumerate()`와 함께 사용하면 인덱스 + 값 수정 가능 |
| 구조체 필드 내부를 반복 수정할 때| `Vec<Struct>`에서 필드 값을 직접 변경 가능          |
| 반복자 조합으로 필터링 후 수정   | `filter_map`, `zip` 등과 함께 사용 가능             |
| 복사 없이 참조로 처리하고 싶을 때| `clone()` 없이 `&mut`로 직접 수정 가능             

## ✨ 예시: 구조체 필드 수정
```rust
struct Point {
    x: f64,
    y: f64,
}

let mut points = vec![
    Point { x: 1.0, y: 2.0 },
    Point { x: 3.0, y: 4.0 },
];

for p in points.iter_mut() {
    p.x += 10.0;
}
```

→ points의 각 x 값을 직접 수정

## ⚠️ 주의할 점
- iter_mut()은 가변 참조를 반환하므로, 해당 슬라이스는 반복 중에 다른 곳에서 접근하면 컴파일 에러가 발생
- 반복 중에 슬라이스를 변경하거나 재할당하면 불안정한 상태가 될 수 있음

---
## Tuple 구조체와 결합 하면
KnotVector가 Vec<T>를 튜플 구조체로 감싼 다음 std::slice::IterMut를 재정의하면,  
그건 단순한 반복 이상의 도메인 제어, API 안정성, 타입 안전성을 의도한 구조적 설계를 할 수 있음.

## 🔍 구조 예시 (추정)
```rust
pub struct KnotVector<T>(pub Vec<T>);
impl<T> KnotVector<T> {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}
```

---
# std::slice::IterMut 재정의 하는 필요성

## 🧠 왜 이렇게 감싸고 iter_mut()를 재정의했을까?
| 목적 또는 상황                  | 설명 또는 의도                                                  |
|-------------------------------|------------------------------------------------------------------|
| Vec<T> → KnotVector<T> 감싸기 | 도메인 의미 부여: 단순한 벡터가 아니라 NURBS의 Knot Vector임을 명시 |
| Vec<T> 직접 노출 피하기        | 내부 구조를 감추고 API 안정성 확보                              |
| Vec<T> 제어                    | 반복자나 접근 방식을 제한하거나 커스터마이징 가능               |
| 도메인 기능 추가               | `validate()`, `is_uniform()`, `insert_knot()` 등 기능 내장 가능  |
| Trait 구현 준비                | `IntoIterator`, `Deref`, `Index` 등 반복자/참조 관련 trait 구현 가능 |
| 반복자 제어                    | `iter()`, `iter_mut()`를 직접 정의해 안전한 접근 방식 제공       |

## ✨ 실전에서의 장점
- KnotVector가 단순한 Vec<f64>가 아니라 NURBS 수학적 의미를 가진 구조체라는 걸 명확히 표현
- 반복자(iter_mut)를 직접 노출함으로써 사용자는 안전하게 수정만 가능하고, 내부 구조는 보호됨
- 나중에 KnotVector에 정렬 검사, 중복 제거, 범위 제한 같은 기능을 추가해도 외부 API는 그대로 유지 가능

## ✅ 결론
- KnotVector(Vec<T>) + iter_mut() 재정의는 도메인 명확화 + API 안정성 + 확장성 확보를 위한 전략적 설계
- 특히 NURBS처럼 수학적 구조가 중요한 분야에서는 이런 감싸기가 오히려 필수적인 안전 장치
- KnotVector<T>처럼 Vec<T>를 감싸는 구조는 단순한 래핑을 넘어서, **도메인 중심 설계(Domain-Oriented Design)** 의 대표적인 사례.
- NURBS에서 Knot Vector는 단순한 숫자 배열이 아니라, 수학적 의미와 규칙을 갖는 핵심 구조이기 때문에 그에 맞는 타입으로 분리하는 게 훨씬 더 나은 선택.

## 🧠 왜 KnotVector<T>가 Vec<T>보다 더 나은가?
| 목적 또는 상황                  | 설명 또는 의도                                                  |
|--------------------------------|------------------------------------------------------------------|
| Vec<f64> → KnotVector<f64>     | 도메인 의미 부여: 단순 숫자 배열이 아니라 NURBS의 Knot Vector임을 명시 |
| 도메인 기능 내장               | `insert_knot()`, `is_uniform()`, `normalize()`, `validate()` 등 기능 집중 |
| 내부 구조 유연화               | `Vec<T>` → `SmallVec<T>` 등으로 교체해도 외부 API는 `KnotVector` 유지 |
| 타입 안전성 강화               | 실수로 일반 `Vec<T>`와 혼용되는 걸 방지                         |
| Trait 구현 가능                | `IntoIterator`, `Index`, `Deref`, `From<Vec<T>>` 등으로 유연한 인터페이스 제공 |
| 확장 가능성 확보               | `KnotSpan`, `Multiplicity`, `Clamping` 등 도메인 구조로 확장 가능       |

## ✨ 실전에서의 장점
- curve.knot_vector.insert_knot(u)처럼 자연스러운 메서드 체인이 가능
- curve.knot_vector.is_clamped() 같은 도메인 지식이 녹아든 API 제공
- iter_mut()를 재정의함으로써 가변 반복자도 안전하게 노출

## ✨ 예시: KnotVector<T>의 확장 가능성
```rust
pub struct KnotVector<T>(pub Vec<T>);

impl<T: Float> KnotVector<T> {
    pub fn insert_knot(&mut self, u: T) {
        // 도메인 로직 내장
    }
    pub fn is_uniform(&self) -> bool {
        // 수학적 검사
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}
```
→ 단순한 Vec<T>가 아니라, NURBS의 수학적 의미를 가진 객체로 진화

## ✅ 결론
- 단순한 Vec<T>보다 KnotVector<T>처럼 의미 있는 타입으로 감싸는 것이 설계적으로 더 안전하고, 유지보수와 확장성 면에서도 훨씬 유리
- 특히 CAD/CAE처럼 수학적 구조가 중요한 분야에서는 이런 타입 분리가 전문성과 신뢰성을 높이는 핵심 전략
- struct로 한 번 감싸는 순간, 그 타입은 단순한 데이터가 아니라 **의미 있는 객체** 가 됩니다.
- 그리고 그 객체에 impl을 붙일 수 있다는 건, 행위(behavior)를 부여하고, 도메인 로직을 집중시킬 수 있음 .
- 이건 Rust의 철학 : **타입은 의미를 담고, 구조는 행위를 담는다.**

---

## 🧠 단일 값 감싸기의 힘: struct MyFloat(f64);
| 목적 또는 상황               | 설명 또는 의도                                                  |
|-----------------------------|------------------------------------------------------------------|
| f64 → KnotValue, Weight 등  | 도메인 의미 부여: 단순 숫자가 아니라 의미 있는 타입으로 분리       |
| impl → normalize(), clamp() | 기능 부여: 메서드를 통해 값에 행위(behavior)를 추가               |
| ParameterU ≠ ParameterV     | 타입 안전성: 서로 다른 의미의 값이 혼용되지 않도록 방지           |
| PointID, CurveID 등         | 식별자 역할: 구조 내에서 명확한 역할을 갖는 타입으로 사용 가능     |
| Display, PartialEq 등 trait | trait 구현으로 인터페이스 확장 가능 (`From<f64>`, `Into<f64>` 등) |
| 내부 접근은 .0              | 내부에서는 `.0`로 값 접근, 외부는 메서드로만 접근하게 설계 가능     |

## ✨ 예시: ParameterU 구조체
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParameterU(pub f64);

impl ParameterU {
    pub fn clamp(self) -> Self {
        ParameterU(self.0.clamp(0.0, 1.0))
    }

    pub fn is_valid(self) -> bool {
        self.0 >= 0.0 && self.0 <= 1.0
    }
}
```
→ 단순한 f64가 아니라 NURBS의 파라메터 공간을 표현하는 의미 있는 타입으로 진화

---




