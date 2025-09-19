# clone_box

ust에서 Box<dyn Trait>를 복제하려면 일반적인 .clone()으로는 안 되고, trait object cloning 패턴을 따라야 함.
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

이제 Box<dyn Exp>도 .clone()을 사용할 수 있게 됩니다


### ✅ 3. 각 수식 타입에서 clone_box() 구현
예: Const
```rust
impl Exp for Const {
    // ...
    fn clone_box(&self) -> Box<dyn Exp> {
        Box::new(Const { value: self.value })
    }
}
```

예: Param
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

예: Add
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


🧩 전체 수식 타입별 clone_box() 구현 요약
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
- Param은 #[derive(Clone)]으로 간단하게 처리 가능

이제 Rust에서도 Exp 트리를 자유롭게 복제할 수 있고,
파서, 미분, 솔버 등에서 안전하게 수식 객체를 재사용할 수 있습니다.


## 🧠 clone_box()의 동작 요약
```rust
fn clone_box(&self) -> Box<dyn Exp> {
    Box::new(self.clone())
}
```

이 코드는 다음을 의미합니다:
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

즉, 힙에 객체가 하나 더 생기고,
기존 객체와는 독립적인 복제본이 만들어지는 거예요.

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


