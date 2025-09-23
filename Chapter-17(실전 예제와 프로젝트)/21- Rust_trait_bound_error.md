# 🧠 Rust trait bound 에러의 본질
Rust는 정적 타입 시스템을 사용하기 때문에, 컴파일 시점에 모든 메서드 호출이 유효한 타입에 대해 정의되어 있는지를 확인합니다. 
이 과정에서 다음 조건이 충족되지 않으면 에러가 발생합니다:

## 에러 예제
```rust
fn eval_point(&self, u: f64) -> Point3D {
    // 기존의 Basis/Nurbs 평가 함수 사용.
    // 예: self.point_at(u) 같은 것을 호출
    self.point_at(u).euclid()
}

// 24  | pub struct BSplineCurve<T: HomogeneousPoint> {
//     | -------------------------------------------- doesn't satisfy `BSplineCurve<T>: Curve`
// ...

// 770 |         self.point_at(u).euclid()

//     |              ^^^^^^^^ method cannot be called on `&BSplineCurve<T>` due to unsatisfied trait bounds


fn point_at(&self, u: f64) -> Point4D {
        let n = self.n_ctrl();
        assert!(n >= 1 && self.knot.len() >= n + self.degree + 1, "Invalid B-spline data");
        let span = self.find_span(u);
        let mut ar_n = vec![0.0; self.degree + 1];
        self.basis_funs(span, u.clamp(self.knot[self.degree], self.knot[n]), &mut ar_n);
        let mut c = Point4D::new(0.0, 0.0, 0.0, 0.0);
        for j in 0..=self.degree {
            let idx = span - self.degree + j;
            let ph = self.control_points[idx].to_h4();
            c = Point4D::h_add(&c, &Point4D::h_scale(&ph, ar_n[j]));
        }
        c
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

## ✅ 1. 메서드가 정의된 trait을 현재 타입이 구현했는가?
```rust
self.point_at(u)
```

이 호출은 Curve trait의 메서드입니다. 따라서 self가 Curve를 구현한 타입이어야 합니다.  
그런데 self가 BSplineCurve<T> 타입일 때, 컴파일러는 이 타입이 Curve를 구현했다는 사실을 해당 컨텍스트에서 보장받지 못하면 에러를 발생시킵니다.

### ✅ 2. generic 타입 T에 필요한 trait bound가 명시되어 있는가?
예를 들어 Curve trait이 내부적으로 T: Debug + Clone + HomogeneousPoint를 요구한다면, BSplineCurve<T>를 사용할 때도 이 조건을 명시적으로 만족시켜야 합니다.
```rust
impl<T: HomogeneousPoint> BSplineCurve<T> {
    fn eval_point(&self, u: f64) -> Point3D {
        Point3D::from_h4(self.point_at(u)) // ❌ 에러 발생
    }
}
```
→ 여기서 BSplineCurve<T>가 Curve를 구현했다는 사실이 보장되지 않기 때문에 point_at() 호출이 불가능합니다.

## ✅ 해결 원칙: Trait Bound 에러 대응

| 상황 구분         | 해결 전략 또는 코드 방식                                      |
|------------------|-------------------------------------------------------------|
| 타입이 trait 메서드를 호출하려고 할 때 | `where BSplineCurve<T>: Curve` 또는 `impl Curve for BSplineCurve<T>` 내부에서 호출 |
| generic 타입 T가 trait 요구 조건을 만족하지 않을 때 | `T: Debug + Clone + HomogeneousPoint` 등 필요한 trait bound 명시 |
| 메서드가 trait에만 정의되어 있고 struct에는 없음 | trait을 통해 호출하거나 struct에 동일한 메서드 구현 추가 |


## 🔧 실전 해결 예시
```rust
impl<T> BSplineCurve<T>
where
    T: HomogeneousPoint + Debug + Clone + 'static,
    BSplineCurve<T>: Curve,
{
    fn eval_point(&self, u: f64) -> Point3D {
        Point3D::from_h4(self.point_at(u))
    }
}
```

→ 이 방식은 컴파일러에게 "이 타입은 Curve를 구현했으니 point_at()을 호출해도 된다"는 사실을 명확히 알려주는 것입니다.

## 📘 요약
Rust에서 이런 에러가 나는 이유는:
- trait 메서드를 호출하려면 해당 trait이 구현되었음이 명확히 보장되어야 한다
- generic 타입에 필요한 trait bound가 누락되면 컴파일러는 메서드 호출을 막는다
- Rust는 암묵적인 추론보다 명시적인 선언을 선호하기 때문에, trait bound를 직접 써줘야 안전하게 컴파일됩니다


## 🧪 1단계: 에러 유도 예제
```rust
trait Speak {
    fn say(&self);
}

struct Dog;

impl Speak for Dog {
    fn say(&self) {
        println!("멍멍!");
    }
}

fn make_speak<T>(item: T) {
    item.say(); // ❌ 에러 발생
}
```

❌ 컴파일 에러
error[E0599]: no method named `say` found for type `T` in the current scope



### 🔍 원인 분석
- make_speak() 함수는 T 타입을 받지만, T가 Speak trait을 구현했다는 보장이 없음
- 따라서 item.say()를 호출할 수 없다고 컴파일러가 판단함

## ✅ 2단계: trait bound 추가로 해결
```rust
fn make_speak<T: Speak>(item: T) {
    item.say(); // ✅ 정상 작동
}
```

### 🔊 출력 결과
```
멍멍!
```


## 🧪 3단계: 더 복잡한 trait bound 예제
```rust
use std::fmt::Debug;

fn describe<T>(item: T) {
    println!("{:?}", item); // ❌ 에러 발생: T가 Debug를 구현하지 않음
}
```

### ✅ 해결 방법
```rust
fn describe<T: Debug>(item: T) {
    println!("{:?}", item); // ✅ 정상 작동
}

```

## 📘 요약 테이블
| 상황                     | 해결 방법                           |
|--------------------------|--------------------------------------|
| trait 메서드 호출 시 에러 | `T: TraitName` 명시                  |
| `println!("{:?}")` 사용 시 | `T: Debug` 명시                      |
| 여러 trait 필요할 때      | `T: TraitA + TraitB + 'static` 등 조합 |





