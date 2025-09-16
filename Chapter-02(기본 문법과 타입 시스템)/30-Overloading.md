# Overloading

## 🧠 function Overloading 왜 안 되는가?
Rust는 함수 이름만으로는 구분이 불가능하기 때문에,
다음 두 함수는 동일한 이름과 시그니처를 가진 것으로 간주돼요:

```rust
impl Vector3D {
    pub fn dot(a: &Self, b: &Self) -> f64 { ... }
    pub fn dot(a: &Self, b: &Point3D) -> f64 { ... } // ❌ 컴파일 에러
}
```

- Rust는 함수 이름 + 인자 타입으로 오버로드를 구분하지 않음
- 따라서 dot()이라는 이름이 두 번 정의되면 중복 정의 오류 발생

### ✅ 해결 방법 1: 함수 이름을 다르게
```rust
impl Vector3D {
    pub fn dot_vec(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn dot_point(a: &Self, b: &Point3D) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}
```

- 함수 이름을 명시적으로 구분해서 의도도 명확하게 표현

### ✅ 해결 방법 2: Trait으로 추상화
```rust
trait DotProduct<Rhs> {
    fn dot(&self, rhs: &Rhs) -> f64;
}

impl DotProduct<Vector3D> for Vector3D {
    fn dot(&self, rhs: &Vector3D) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl DotProduct<Point3D> for Vector3D {
    fn dot(&self, rhs: &Point3D) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
```

- 이렇게 하면 v.dot(&v2) 또는 v.dot(&p)처럼 같은 이름으로 호출 가능
- Rust의 trait 기반 다형성을 활용한 방식

## 🔍 요약
| 비교 항목                         | 방식 1: 함수 이름 분리 (`dot_vec`, `dot_point`) | 방식 2: Trait 오버로드 (`DotProduct<T>`) |
|-------------------------------|-----------------------------------------------|-------------------------------------------|
| 함수 이름                     | 명시적으로 구분                              | 동일한 이름 `dot()` 사용 가능             |
| 호출 방식                     | `Vector3D::dot_vec(&a, &b)`                   | `a.dot(&b)`                                |
| 확장성                        | 함수마다 이름 추가 필요                      | 타입별로 trait 구현만 추가하면 됨         |
| Rust 스타일 적합성            | 덜 idiomatic                                  | 더 idiomatic (trait 기반 다형성)          |
| 코드 가독성                   | 명확하지만 중복 가능성 있음                  | 간결하고 직관적                            |


---


# Rust의 연산자 오버로딩 철학
Rust는 연산자 오버로딩을 trait 기반으로 명시적으로 구현합니다.
즉, +, -, *, /, +=, -=, *= 등은 각각 대응되는 trait이 있고,
그 trait을 impl 하면 해당 타입에 대해 연산자가 동작하게 됩니다.

## 🔧 주요 연산자 Trait 매핑
|  연산자 | Trait 이름   | 메서드 시그니처               | 사용 예시     |
|-----------|--------------|-------------------------------|---------------|
| `+`       | `Add`        | `add(self, rhs)`              | `a + b`       |
| `-`       | `Sub`        | `sub(self, rhs)`              | `a - b`       |
| `*`       | `Mul`        | `mul(self, rhs)`              | `a * scalar`  |
| `/`       | `Div`        | `div(self, rhs)`              | `a / scalar`  |
| `+=`      | `AddAssign`  | `add_assign(&mut self, rhs)`  | `a += b`      |
| `-=`      | `SubAssign`  | `sub_assign(&mut self, rhs)`  | `a -= b`      |
| `*=`      | `MulAssign`  | `mul_assign(&mut self, rhs)`  | `a *= scalar` |
| `/=`      | `DivAssign`  | `div_assign(&mut self, rhs)`  | `a /= scalar` |
| `-a`      | `Neg`        | `neg(self)`                   | `-a`          |



## ✅ Vector3D에 적용된 예시 설명
### ▶️ Add / AddAssign
```rust
impl Add for Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
```

- a + b가 가능해짐
- AddAssign을 구현하면 a += b도 가능

### ▶️ Mul<f64> / MulAssign<f64>
```rust
impl Mul<f64> for Vector3D {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}
```

- a * 2.0처럼 스칼라 곱이 가능
- MulAssign으로 a *= 2.0도 가능

### ▶️ Neg
```rust
impl Neg for Vector3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
```
- -a가 가능해짐

## 🧠 Rust의 장점
- 명시적이고 안전함: 어떤 타입에 어떤 연산이 가능한지 명확하게 정의됨
- 타입별 오버로딩 가능: Mul<f64> vs Mul<Vector3D>처럼 타입별로 다르게 구현 가능
- IDE 지원이 뛰어남: trait 기반이라 자동 완성, 문서화가 잘 됨

## 🔍 확장 아이디어
- impl Mul<Vector3D> for f64도 구현하면 2.0 * v도 가능
- PartialEq, PartialOrd로 비교 연산자도 구현 가능
- Display로 println!("{:?}", v) 출력 형식도 커스터마이징 가능

---
