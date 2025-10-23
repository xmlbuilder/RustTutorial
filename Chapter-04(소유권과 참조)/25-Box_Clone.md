# clone_box

Rust에서 `Box<dyn Trait>` 를 복제하려면 일반적인 `.clone()` 으로는 안 되고, trait object cloning 패턴을 따라야 함.  
아래에 clone_box 문법을 정리하고, 전체 수식 타입별로 어떻게 적용되는지도 깔끔하게 정리.

## 🧠 clone_box 문법 정리
Rust에서 trait object (Box<dyn Exp>)를 복제하려면 다음 3단계를 따라야 합니다:

### ✅ 1. Trait에 clone_box() 메서드 추가
```rust
pub trait Exp: std::fmt::Debug {
    fn evaluate(&self, vars: &HashMap<String, f64>) -> f64;
    fn differentiate(&self, var: &str) -> Box<dyn Exp>;
    fn to_string(&self) -> String;
    fn clone_box(&self) -> Box<dyn Exp>;
}
```


### ✅ 2. Clone 트레잇을 Box<dyn Exp>에 구현
```rust
impl Clone for Box<dyn Exp> {
    fn clone(&self) -> Box<dyn Exp> {
        self.clone_box()
    }
}
```
- 이제 Box<dyn Exp>도 .clone()을 사용할 수 있게 됩니다


### ✅ 3. 각 수식 타입에서 clone_box() 구현
#### 예: Const
```rust
impl Exp for Const {
    // ...
    fn clone_box(&self) -> Box<dyn Exp> {
        Box::new(Const { value: self.value })
    }
}
```

#### 예: Param
```rust
#[derive(Clone)]
pub struct Param {
    pub name: String,
    pub value: f64,
}

impl Exp for Param {
    // ...
    fn clone_box(&self) -> Box<dyn Exp> {
        Box::new(self.clone())
    }
}
```

#### 예: Add
```rust
impl Exp for Add {
    // ...
    fn clone_box(&self) -> Box<dyn Exp> {
        Box::new(Add {
            left: self.left.clone(),
            right: self.right.clone(),
        })
    }
}
```

## 🧩 전체 수식 타입별 clone_box() 구현 요약
| 타입      | clone_box() 구현 방식                                      | 비고 / 확장 코멘트                          |
|-----------|------------------------------------------------------------|---------------------------------------------|
| Const     | `Box::new(Const { value })`                                | 상수는 그대로 복사                          |
| Param     | `Box::new(self.clone())` ← `#[derive(Clone)]` 필요         | 변수는 이름과 값 복사, 공유 가능            |
| Add       | `Box::new(Add { left.clone(), right.clone() })`            | 이항 연산자: 좌우 복제                      |
| Sub       | `Box::new(Sub { left.clone(), right.clone() })`            | 동일 구조                                   |
| Mul       | `Box::new(Mul { left.clone(), right.clone() })`            | 동일 구조                                   |
| Div       | `Box::new(Div { left.clone(), right.clone() })`            | 동일 구조                                   |
| Pow       | `Box::new(Pow { base.clone(), exponent.clone() })`         | 지수 연산자                                 |
| Sin       | `Box::new(Sin { arg.clone() })`                            | 단항 함수 구조                              |
| Cos       | `Box::new(Cos { arg.clone() })`                            | 단항 함수 구조                              |
| Log       | `Box::new(Log { arg.clone() })`                            | 단항 함수 구조                              |
| Sqrt      | `Box::new(Sqrt { arg.clone() })`                           | 단항 함수 구조                              |
| Compare   | `Box::new(Compare { left.clone(), right.clone(), op.clone() })` | 연산자 문자열도 복제 필요              |
| Vector    | `Box::new(Vector { x.clone(), y.clone(), z.clone() })`     | 3D 벡터 구성                                |
| Matrix    | `Box::new(Matrix { m[i][j].clone() })` ← 2중 루프 필요     | 4×4 행렬 등 고차원 구조                    |


## ✅ 요약
- clone_box()는 trait object 복제를 위한 핵심 메서드
- Box<dyn Exp>에 Clone을 구현하면 .clone() 사용 가능
- 각 수식 타입은 내부 필드를 복제해서 새 객체 생성
- Param은 `#[derive(Clone)]` 으로 간단하게 처리 가능

이제 Rust에서도 Exp 트리를 자유롭게 복제할 수 있고,  
파서, 미분, 솔버 등에서 안전하게 수식 객체를 재사용할 수 있습니다.


## 🧠 clone_box()의 동작 요약
```rust
fn clone_box(&self) -> Box<dyn Exp> {
    Box::new(self.clone())
}
```

### 이 코드는 다음을 의미합니다:
- self.clone() → 기존 객체의 복제본을 생성
- Box::new(...) → 복제본을 힙에 할당
- 반환값은 Box<dyn Exp> → 소유권을 가진 스마트 포인터
| 단계            | 설명                                           |
|-----------------|------------------------------------------------|
| Box<dyn Exp>    | 기존 수식 트리의 루트 노드. 힙에 객체 A가 존재 |
| clone_box()     | 객체 A를 복제하여 객체 B를 생성                |
| Box::new(B)     | 복제된 객체 B를 힙에 새로 할당                 |
| 반환            | 새 Box<dyn Exp>가 객체 B의 소유권을 가짐       |

```
[Box<dyn Exp>] ──▶ [clone_box()] ──▶ [Box::new(B)] ──▶ [Box<dyn Exp> (복제본)]
     (A)               (A → B)           (B 힙에 생성)         (B의 소유권)
```
- 즉, 힙에 객체가 하나 더 생기고, 기존 객체와는 독립적인 복제본이 만들어짐.

## 🔐 소유권과 안전성
Rust의 소유권 시스템 덕분에:
- 원본과 복제본은 명확히 분리된 메모리
- 복제본은 독립적으로 수정 가능
- Box<dyn Exp>는 Drop 시 자동 해제
- clone_box()는 안전하게 트리 구조를 복제하는 데 필수

## ✅ 요약
| 단계           | 설명                                                   |
|----------------|--------------------------------------------------------|
| clone_box()    | 기존 객체를 복제하여 새로운 힙 객체를 생성             |
| Box::new(...)  | 복제된 객체를 힙에 할당하고 Box 스마트 포인터로 감쌈   |
| 반환           | 새 Box<dyn Exp>가 복제된 객체의 소유권을 갖고 반환됨   |
| 결과           | 원본과 독립적인 복제본이 생기며 안전하게 사용 가능     |

---

# 🧠 핵심 이유: Box<T>는 T: Clone일 때만 clone() 가능
## 🔍 왜 clone이 안 될 수 있는가?
- Box<T>는 T를 힙에 저장하고 그 포인터를 관리합니다.
- Box<T>를 clone()하려면 내부의 T를 복제해서 새로운 Box에 넣어야 합니다.
- 따라서 T가 Clone을 구현하지 않으면 Box<T>도 Clone을 구현할 수 없습니다.

```rust
struct NotClone;

let b = Box::new(NotClone);
// let b2 = b.clone(); // ❌ 컴파일 에러: NotClone doesn't implement Clone
```


## ✅ 언제 clone이 되는가?
```rust
#[derive(Clone)]
struct MyData(i32);

let b1 = Box::new(MyData(42));
let b2 = b1.clone(); // ✅ OK: MyData implements Clone
```
- 여기서 Box<MyData>는 MyData가 Clone을 구현했기 때문에 clone() 가능
- b2는 b1의 복제본으로, 새로운 힙 메모리에 MyData(42)를 복사해서 저장함

## 🔍 Box는 포인터 복사 아님
- Box::clone()은 단순히 포인터를 복사하는 게 아니라, 힙에 있는 데이터를 복제합니다
- 즉, Box<T>는 얕은 복사가 아닌 깊은 복사를 수행하려고 하기 때문에 T: Clone이 필수

## ✅ 정리: Box<T>는 언제 clone 가능한가?

| 조건        | 내부 타입 T의 특성 | Box<T>의 clone 가능 여부 |
|-------------|--------------------|---------------------------|
| `T: Clone`  | T가 `Clone`을 구현함 | ✅ `Box<T>`도 `Clone` 가능 |
| `T` not Clone | T가 `Clone`을 구현하지 않음 | ❌ `Box<T>`는 `Clone` 불가능 |


## 🧠 box_clone()이 되는 이유: Trait Object를 복제하는 패턴
Rust에서는 Box<dyn Trait>을 직접 clone()할 수 없습니다. 왜냐하면:
- dyn Trait는 크기가 고정되지 않은 타입 (DST) 이고
- Rust는 dyn Trait이 Clone을 구현했는지 알 수 없기 때문  
그래서 Rust에서는 이런 상황을 위해 명시적인 복제 메서드를 정의하는 패턴을 사용합니다.

## ✅ 예시: box_clone() 패턴
```rust
trait MyTrait {
    fn box_clone(&self) -> Box<dyn MyTrait>;
}

impl Clone for Box<dyn MyTrait> {
    fn clone(&self) -> Box<dyn MyTrait> {
        self.box_clone()
    }
}
```

### 🔍 설명
- box_clone()은 MyTrait에 정의된 자기 자신을 Box로 감싸서 복제하는 메서드입니다
- Box<dyn MyTrait>는 Clone을 직접 구현할 수 없기 때문에,
- box_clone()을 통해 내부 타입을 복제하고
- 새로운 Box<dyn MyTrait>를 만들어 반환하는 방식으로 해결합니다

## 📦 핵심 요약
| 항목               | 조건 또는 설명           | 복제 가능 여부 또는 방식         |
|--------------------|--------------------------|----------------------------------|
| `Box<T>: Clone`    | `T: Clone`               | ✅ 가능 (`Box<T>`도 `Clone`)     |
| `Box<dyn Trait>: Clone` | `dyn Trait`은 DST라서 직접 `Clone` 구현 불가 | ❌ 불가능 (직접 clone 불가)       |
| `box_clone()`      | Trait에 복제 메서드 정의 | ✅ 가능 (`Box<dyn Trait>` 복제용) |

## ✨ 결론
- box_clone()은 Trait Object 복제를 위한 사용자 정의 메서드입니다
- Rust는 dyn Trait의 크기를 알 수 없기 때문에 Clone을 직접 구현할 수 없고,
- 대신 box_clone()을 통해 간접적으로 복제하는 방식이 널리 사용됩니다
- box_clone()은 깊은 복사를 수행합니다
- 힙에 있는 데이터까지 새롭게 복제해서 새로운 Box를 반환합니다
- 이는 trait object를 안전하게 복제하기 위한 Rust의 일반적인 패턴입니다





