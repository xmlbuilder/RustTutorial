# Generic/Trait/Lifetime
## 🧠 핵심 개념 간단 설명
| 개념       | 핵심 역할                                      |
|------------|------------------------------------------------|
| Generics   | 여러 타입(`T`, `U`)에 대해 공통 코드를 작성할 수 있게 함 |
| Trait      | 다양한 타입 중 특정 기능을 구현한 타입만 사용하도록 제약 |
| Lifetime   | 참조값의 유효 기간을 명시하여 메모리 안전성을 보장     |


## 📚 학습 흐름 요약
### ① Generics
- 함수, 구조체, 열거형에 T, U 등 타입 매개변수 사용
- 제약 없는 일반 타입부터 시작 → Trait bound로 확장
- 예: fn add<T: Add>(a: T, b: T) -> T
### ② Trait
- trait 정의 → impl로 구현
- dyn Trait vs impl Trait 차이
- Trait object, default method, supertrait 등 고급 개념도 있음
- 예: trait Drawable { fn draw(&self); }
### ③ Lifetime
- 'a, 'static 같은 lifetime 명시
- 함수 간 참조 전달 시 충돌 방지
- 구조체에 lifetime 붙이기, 함수 시그니처에 명시하기
- 예: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

## 🧭 추천 학습 순서
- Generics 기본 문법 → 구조체/함수에 적용
- Trait 정의 및 구현 → Trait bound와 함께 사용
- Trait + Generics 조합 → 실전 함수 설계
- Lifetime 기본 개념 → 함수에서 참조값 다루기
- Lifetime + 구조체 → 복잡한 참조 관계 이해
- 실전 예제: API 설계, Iterator, 클로저 등에서 활용

----

# Generic Trait 실전 문제

## Rust Generic Type 맞추기

Rust에서 제네릭 타입을 사용하는 구조에서 함수 인자 타입이 예상과 다를 때 발생하는 컴파일.
핵심은:
함수가 &[T]를 받도록 정의되어 있는데, 호출 시 trait가 구현 안된 &[Point4D]를 넘기면 타입 불일치가 발생한다는 것.

아래에 아주 간단한 샘플을 만들어서, 문제 발생 → 해결까지 흐름을 보여줌.

## 🧪 문제 재현 샘플
```rust
pub trait HomogeneousPoint {
    fn to_h(&self) -> Point4D;
}

#[derive(Copy, Clone, Debug)]
pub struct Point4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn h_lerp(a: &Self, b: &Self, t: f64) -> Self {
        Self::new(
            a.x * (1.0 - t) + b.x * t,
            a.y * (1.0 - t) + b.y * t,
            a.z * (1.0 - t) + b.z * t,
            a.w * (1.0 - t) + b.w * t,
        )
    }
}

fn de_casteljau_row<T: HomogeneousPoint>(points: &[T], t: f64) -> Point4D {
    let mut tmp: Vec<Point4D> = points.iter().map(|p| p.to_h()).collect();
    let n = tmp.len();
    for r in 1..n {
        for i in 0..(n - r) {
            tmp[i] = Point4D::h_lerp(&tmp[i], &tmp[i + 1], t);
        }
    }
    tmp[0]
}

fn main() {
    let pts = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 1.0, 0.0, 1.0),
    ];

    // ❌ 오류 발생: expected &[T], found &[Point4D]
    let result = de_casteljau_row(&pts, 0.5);
    println!("{:?}", result);
}
```

🔥 컴파일 오류
```
error[E0308]: mismatched types
  --> main.rs:35:32
   |
35 |     let result = de_casteljau_row(&pts, 0.5);
   |                                ^^^^^ expected type parameter `T`, found struct `Point4D`

```

### ✅ 해결 방법 1: de_casteljau_row를 Point4D 전용으로 바꾸기
```rust
fn de_casteljau_row_4d(points: &[Point4D], t: f64) -> Point4D {
    let mut tmp = points.to_vec();
    let n = tmp.len();
    for r in 1..n {
        for i in 0..(n - r) {
            tmp[i] = Point4D::h_lerp(&tmp[i], &tmp[i + 1], t);
        }
    }
    tmp[0]
}
```

→ 호출부:
```rust
let result = de_casteljau_row_4d(&pts, 0.5);
```

#### ✅ 컴파일 성공!

### ✅ 해결 방법 2: pts를 Vec<T>로 유지하고 T: HomogeneousPoint 제약을 만족시키기
예를 들어 Point3D 타입을 만들고 HomogeneousPoint를 구현한 뒤,
Vec<Point3D>를 넘기면 de_casteljau_row(&[T])가 타입 일치하게 됩니다.

```rust
// HomogeneousPoint 트레잇 정의
pub trait HomogeneousPoint {
    fn to_h(&self) -> Point4D;
}

// Point4D 타입 정의
#[derive(Copy, Clone, Debug)]
pub struct Point4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl HomogeneousPoint for Point4D {
    fn to_h(&self) -> Point4D {
        *self
    }
}

impl Point4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn h_lerp(a: &Self, b: &Self, t: f64) -> Self {
        Self::new(
            a.x * (1.0 - t) + b.x * t,
            a.y * (1.0 - t) + b.y * t,
            a.z * (1.0 - t) + b.z * t,
            a.w * (1.0 - t) + b.w * t,
        )
    }
}
fn de_casteljau_row<T: HomogeneousPoint>(points: &[T], t: f64) -> Point4D {
    let mut tmp: Vec<Point4D> = points.iter().map(|p| p.to_h()).collect();
    let n = tmp.len();
    for r in 1..n {
        for i in 0..(n - r) {
            tmp[i] = Point4D::h_lerp(&tmp[i], &tmp[i + 1], t);
        }
    }
    tmp[0]
}

fn main() {
    let pts = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 1.0, 0.0, 1.0),
    ];
    let result = de_casteljau_row(&pts, 0.5);
    println!("{:?}", result);
}
```

## 💬 핵심 요약

| 함수 정의                      | 호출 시 인자 타입         | 문제 원인                          |
|-------------------------------|----------------------------|------------------------------------|
| `de_casteljau_row(&[T])`      | `&[Point4D]`               | `T`는 제네릭인데 `Point4D`를 넘김 |
| 기대 타입: `&[T]`             | 실제 타입: `&[Point4D]`    | 타입 불일치     



## 🔍 핵심 개념: 트레잇은 “정의”가 아니라 “구현”된다
Rust에서 trait HomogeneousPoint는 **행동(인터페이스)**만 정의합니다.
즉, “이 트레잇을 만족하려면 to_h()를 구현해야 한다”는 규칙만 정하는 거예요.
```rust
pub trait HomogeneousPoint {
    fn to_h(&self) -> Point4D;
}
```

이건 “Point4D가 이 트레잇을 반드시 가져야 한다”는 뜻이 아니라,
“원한다면 Point4D에 이 트레잇을 구현할 수 있다”는 뜻이에요.

### ✅ 실제 구현은 따로 해야 함
```rust
impl HomogeneousPoint for Point4D {
    fn to_h(&self) -> Point4D {
        *self
    }
}
```

이렇게 해야 Point4D가 HomogeneousPoint를 만족하게 되고,
그 이후에 Point4D를 T: HomogeneousPoint로 사용하는 제네릭 함수에 넘길 수 있게 됩니다.

## 💡 비유로 이해하기
- trait HomogeneousPoint는 “운전할 수 있는 사람”이라는 자격 요건
- impl HomogeneousPoint for Point4D는 “JungHwan은 운전할 수 있다”는 증명
→ 트레잇은 자격을 정의하고,
→ impl은 그 자격을 특정 타입에 부여하는 거예요

## 💬 결론
HomogeneousPoint는 Point4D를 포함하거나 정의하지 않습니다.
대신, 우리가 impl HomogeneousPoint for Point3D를 작성해야
Point4D가 그 트레잇을 “가지게 되는” 거예요.


## 💡 요약 테이블

| 상황 또는 타입                  | 요구 조건 또는 해결 방법             |
|-------------------------------|-------------------------------------|
| `de_casteljau_row<T: HomogeneousPoint>` | T는 `HomogeneousPoint`를 구현해야 함 |
| `Point4D` 사용 시              | `impl HomogeneousPoint for Point4D` 필요 |
| `Point3D` 사용 시              | (선택 사항) `impl HomogeneousPoint for Point3D` 가능 |



## 🔍 왜 이 한 줄이 핵심이었는가?
당신의 함수는 이렇게 정의되어:
```rust
fn de_casteljau_row<T: HomogeneousPoint>(points: &[T], t: f64) -> Point4D
```

- 이 함수는 제네릭 타입 T를 받는데, T는 HomogeneousPoint를 구현해야 함
- 그런데 main()에서 넘긴 타입은 Point4D
- Rust는 “Point4D가 HomogeneousPoint를 구현했는지”를 확인함
- 구현이 없으면 컴파일러는 타입 불일치 오류를 발생시킴

### ✅ 해결: Point4D가 HomogeneousPoint를 구현하면 타입이 일치
```rust
let result = de_casteljau_row(&pts, 0.5);
```

→ 여기서 T = Point4D
→ Point4D: HomogeneousPoint가 만족되므로
→ points: &[T]와 &pts가 타입 일치
→ ✅ 컴파일 성공

## 💡 핵심 요약 테이블
| 상황                                | 설명                                       |
|-------------------------------------|--------------------------------------------|
| `de_casteljau_row<T: HomogeneousPoint>` | T는 반드시 `to_h()`를 구현해야 함         |
| `Point4D`를 넘김                     | `Point4D`가 `HomogeneousPoint`를 구현해야 함 |
| 구현이 없으면                       | ❌ 타입 불일치 오류 발생                   |
| 구현이 있으면                       | ✅ 타입 일치, 컴파일 성공                  |


---


