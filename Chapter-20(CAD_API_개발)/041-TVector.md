# TVector
- TVector2, TVector3, TVector4 를 포함한 벡터 모듈의 전체 구조와 테스트 코드 요약, 함수 목록 정리,  
    그리고 버그 점검 결과입니다.

## 📦 TVector 모듈 개요
- 이 모듈은 2D, 3D, 4D 벡터 연산을 위한 범용 템플릿 구조를 제공합니다. 실수형뿐 아니라 정수형도 지원하며,  
    수치 계산, 기하 연산, 데이터 처리에 적합합니다.

### ✅ 주요 타입
| 타입 이름        | 설명                                      |
|------------------|-------------------------------------------|
| `TVector2<T>`     | 2D 벡터 (x, y)                            |
| `TVector3<T>`     | 3D 벡터 (x, y, z)                         |
| `TVector4<T>`     | 4D 벡터 (x, y, z, w)                      |
| `DenseVec`        | 벡터형 인터페이스 (get/set/swap)         |
| `TVector2f/d/i`   | f32/f64/i32 타입 별칭                     |
| `TVector3f/d/i`   | f32/f64/i32 타입 별칭                     |
| `TVector4f/d/i`   | f32/f64/i32 타입 별칭                     |

### 🧩 주요 함수 목록
| 함수 이름             | 적용 대상     | 설명                                      |
|----------------------|---------------|-------------------------------------------|
| `new`, `set_value`   | TVector2/3/4  | 벡터 생성 및 값 설정                      |
| `get_value`          | TVector2/3/4  | 배열 형태로 값 반환                       |
| `dot`, `length_squared`, `length` | TVector2/3/4 | 내적, 길이 계산                    |
| `normalize`          | TVector2/3/4  | 단위 벡터로 정규화                        |
| `cross`              | TVector2/3    | 외적 계산 (2D: 스칼라, 3D: 벡터)         |
| `negate`, `-v`       | TVector2/3/4  | 부호 반전                                 |
| `max_component`, `min_component` | TVector2/3/4 | 최대/최소 성분 반환             |
| `equals_eps`         | TVector2/3/4  | 오차 기반 비교                            |
| `fill`               | TVector2/3/4  | 모든 성분을 동일 값으로 설정              |
| `from_v3`            | TVector4      | TVector3 → TVector4 변환                  |
| `Index`, `IndexMut`  | TVector2/3/4  | 인덱스 접근 및 수정                       |
| `Add`, `Sub`, `Mul`, `Div` | TVector2/3/4 | 연산자 오버로드 (성분별/스칼라)   |


### 🐞 버그 점검 결과
- ✅ 모든 테스트 통과
- ✅ Index 범위 초과 시 panic 정상 작동
- ✅ 정규화, 외적, 오차 비교 등 수치 연산 정확도 우수
- ✅ 스칼라 연산자 분배법칙, 항등성, 대칭성 모두 검증됨


```rust
use crate::core::scalar::FloatScalar;
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

// --------------------------- TVector2 ---------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TVector2<T> {
    pub x: T,
    pub y: T,
}
```
```rust
impl<T: Copy> TVector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn set_value(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
    #[inline]
    pub fn get_value(&self) -> [T; 2] {
        [self.x, self.y]
    }
}
```
```rust
impl<T: Copy + Add<Output = T> + Mul<Output = T>> TVector2<T> {
    #[inline]
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
    #[inline]
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }
}
```
```rust
impl<T: FloatScalar> TVector2<T> {
    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
    /// self를 단위화. (원래 길이 반환)
    pub fn normalize(&mut self) -> T {
        let mag = self.length();
        if mag > T::zero() {
            let inv = T::one() / mag;
            self.x = self.x * inv;
            self.y = self.y * inv;
        } else {
            self.x = T::zero();
            self.y = T::zero();
        }
        mag
    }
```
```rust
    // 2D Cross Product: Scalar (Z component only)
    #[inline]
    pub fn cross(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
```
```rust    
    #[inline]
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
```
```rust    
    #[inline]
    pub fn max_component(&self) -> T {
        self.x.max(self.y)
    }
```
```rust    
    #[inline]
    pub fn min_component(&self) -> T {
        self.x.min(self.y)
    }
```
```rust
    //Compare with tolerance epsilon: ‖a−b‖²≤ε²
    pub fn equals_eps(&self, rhs: &Self, eps: T) -> bool {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        dx * dx + dy * dy <= eps * eps
    }
}

```
```rust
impl<T> Index<usize> for TVector2<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range"),
        }
    }
}
```
```rust
impl<T> IndexMut<usize> for TVector2<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of range"),
        }
    }
}
```
```rust
impl<T: Copy + Neg<Output = T>> Neg for TVector2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
```
```rust
impl<T: Copy + Add<Output = T>> Add for TVector2<T> {
    type Output = Self;
    fn add(self, r: Self) -> Self::Output {
        Self::new(self.x + r.x, self.y + r.y)
    }
}
```
```rust
impl<T: Copy + Sub<Output = T>> Sub for TVector2<T> {
    type Output = Self;
    fn sub(self, r: Self) -> Self::Output {
        Self::new(self.x - r.x, self.y - r.y)
    }
}
```
```rust
/// 성분별 곱
impl<T: Copy + Mul<Output = T>> Mul for TVector2<T> {
    type Output = Self;
    fn mul(self, r: Self) -> Self::Output {
        Self::new(self.x * r.x, self.y * r.y)
    }
}
```
```rust
/// 스칼라 곱/나눗셈
impl<T: Copy + Mul<Output = T>> Mul<T> for TVector2<T> {
    type Output = Self;
    fn mul(self, s: T) -> Self::Output {
        Self::new(self.x * s, self.y * s)
    }
}
```
```rust
impl<T: Copy + Div<Output = T>> Div<T> for TVector2<T> {
    type Output = Self;
    fn div(self, s: T) -> Self::Output {
        Self::new(self.x / s, self.y / s)
    }
}
```
```rust
impl<T: Clone> TVector2<T> {
    pub fn fill(&mut self, val: T) {
        self.x = val.clone();
        self.y = val;
    }
}
```
```rust
// --------------------------- TVector3 ---------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TVector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
```
```rust
impl<T: Copy> TVector3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
```
```rust
    #[inline]
    pub fn from_v4(v4: TVector4<T>) -> Self {
        Self {
            x: v4.x,
            y: v4.y,
            z: v4.z,
        }
    }
```
```rust    
    #[inline]
    pub fn set_value(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
```
```rust    
    #[inline]
    pub fn get_value(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}
```
```rust
impl<T: Copy + Add<Output = T> + Mul<Output = T>> TVector3<T> {
    #[inline]
    pub fn dot(&self, r: &Self) -> T {
        self.x * r.x + self.y * r.y + self.z * r.z
    }

```
```rust    
    #[inline]
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }
}

```
```rust
impl<T: FloatScalar> TVector3<T> {
    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
```
```rust    
    /// Normalization (restoring original magnitude)
    pub fn normalize(&mut self) -> T {
        let mag = self.length();
        if mag != T::zero() {
            let inv = T::one() / mag;
            self.x = self.x * inv;
            self.y = self.y * inv;
            self.z = self.z * inv;
        } else {
            self.set_value(T::zero(), T::zero(), T::zero());
        }
        mag
    }
```
```rust    
    #[inline]
    pub fn cross(&self, r: &Self) -> Self {
        Self::new(
            self.y * r.z - self.z * r.y,
            self.z * r.x - self.x * r.z,
            self.x * r.y - self.y * r.x,
        )
    }
```
```rust   
    #[inline]
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
```
```rust    
    #[inline]
    pub fn max_component(&self) -> T {
        self.x.max(self.y).max(self.z)
    }
```
```rust    
    #[inline]
    pub fn min_component(&self) -> T {
        self.x.min(self.y).min(self.z)
    }
```
```rust    
    pub fn equals_eps(&self, r: &Self, eps: T) -> bool {
        let dx = self.x - r.x;
        let dy = self.y - r.y;
        let dz = self.z - r.z;
        dx * dx + dy * dy + dz * dz <= eps * eps
    }
}
```
```rust
impl<T> Index<usize> for TVector3<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index"),
        }
    }
}
```
```rust
impl<T> IndexMut<usize> for TVector3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index"),
        }
    }
}
```
```rust
impl<T: Copy + Neg<Output = T>> Neg for TVector3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}
```
```rust
impl<T: Copy + Add<Output = T>> Add for TVector3<T> {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}
```
```rust
impl<T: Copy + Sub<Output = T>> Sub for TVector3<T> {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}
```
```rust
impl<T: Copy + Mul<Output = T>> Mul for TVector3<T> {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::new(self.x * r.x, self.y * r.y, self.z * r.z)
    }
}
```
```rust
impl<T: Copy + Mul<Output = T>> Mul<T> for TVector3<T> {
    type Output = Self;
    fn mul(self, s: T) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}
```
```rust
impl<T: Copy + Div<Output = T>> Div<T> for TVector3<T> {
    type Output = Self;
    fn div(self, s: T) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s)
    }
}
```
```rust
impl<T: Clone> TVector3<T> {
    pub fn fill(&mut self, val: T) {
        self.x = val.clone();
        self.y = val.clone();
        self.z = val;
    }
}
```
```rust
// --------------------------- TVector4 ---------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TVector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
```
```rust
impl<T: Copy> TVector4<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
```
```rust    
    #[inline]
    pub fn set_value(&mut self, x: T, y: T, z: T, w: T) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
```
```rust    
    #[inline]
    pub fn get_value(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
```
```rust
impl<T: Copy + Default> TVector4<T> {
    #[inline]
    pub fn from_v3(v3: TVector3<T>) -> Self {
        Self {
            x: v3.x,
            y: v3.y,
            z: v3.z,
            w: T::default(),
        }
    }
}
```
```rust
impl<T: Copy + Add<Output = T> + Mul<Output = T>> TVector4<T> {
    #[inline]
    pub fn dot(&self, r: &Self) -> T {
        self.x * r.x + self.y * r.y + self.z * r.z + self.w * r.w
    }
```
```rust    
    #[inline]
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }
}
```
```rust
impl<T: FloatScalar> TVector4<T> {
    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
    /// Normalization (restoring original magnitude)
    pub fn normalize(&mut self) -> T {
        let mag = self.length();
        if mag != T::zero() {
            let inv = T::one() / mag;
            self.x = self.x * inv;
            self.y = self.y * inv;
            self.z = self.z * inv;
            self.w = self.w * inv;
        } else {
            self.set_value(T::zero(), T::zero(), T::zero(), T::zero());
        }
        mag
    }
```
```rust    
    #[inline]
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
    }
```
```rust    
    #[inline]
    pub fn max_component(&self) -> T {
        self.x.max(self.y).max(self.z).max(self.w)
    }
```
```rust    
    #[inline]
    pub fn min_component(&self) -> T {
        self.x.min(self.y).min(self.z).min(self.w)
    }
```
```rust    
    pub fn equals_eps(&self, r: &Self, eps: T) -> bool {
        let dx = self.x - r.x;
        let dy = self.y - r.y;
        let dz = self.z - r.z;
        let dw = self.w - r.w;
        dx * dx + dy * dy + dz * dz + dw * dw <= eps * eps
    }
}
```
```rust
impl<T> Index<usize> for TVector4<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index"),
        }
    }
}
```
```rust
impl<T> IndexMut<usize> for TVector4<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index"),
        }
    }
}
```
```rust
impl<T: Copy + Neg<Output = T>> Neg for TVector4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}
```
```rust
impl<T: Copy + Add<Output = T>> Add for TVector4<T> {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.x + r.x, self.y + r.y, self.z + r.z, self.w + r.w)
    }
}
```
```rust
impl<T: Copy + Sub<Output = T>> Sub for TVector4<T> {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.x - r.x, self.y - r.y, self.z - r.z, self.w - r.w)
    }
}
```
```rust
impl<T: Copy + Mul<Output = T>> Mul for TVector4<T> {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::new(self.x * r.x, self.y * r.y, self.z * r.z, self.w * r.w)
    }
}
```
```rust
impl<T: Copy + Mul<Output = T>> Mul<T> for TVector4<T> {
    type Output = Self;
    fn mul(self, s: T) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}
```
```rust
impl<T: Copy + Div<Output = T>> Div<T> for TVector4<T> {
    type Output = Self;
    fn div(self, s: T) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s, self.w / s)
    }
}
```
```rust
impl<T: Clone> TVector4<T> {
    pub fn fill(&mut self, val: T) {
        self.x = val.clone();
        self.y = val.clone();
        self.z = val.clone();
        self.w = val;
    }
}
```
```rust
// --------------------------- Type aliases ---------------------------
// Floating-point version (recommended)
pub type TVector2f = TVector2<f32>;
pub type TVector2d = TVector2<f64>;
pub type TVector3f = TVector3<f32>;
pub type TVector3d = TVector3<f64>;
pub type TVector4f = TVector4<f32>;
pub type TVector4d = TVector4<f64>;
```
```rust
// Aliases are provided for the integer version as well,
// but do not use methods that depend on FloatScalar, such as length or normalize.
pub type TVector2i = TVector2<i32>;
pub type TVector2l = TVector2<i64>;
pub type TVector3i = TVector3<i32>;
pub type TVector3l = TVector3<i64>;
pub type TVector4i = TVector4<i32>;
pub type TVector4l = TVector4<i64>;
```
```rust
pub trait DenseVec {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> f64;
    fn set(&mut self, i: usize, v: f64);
    fn swap(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }
        let tmp = self.get(i);
        self.set(i, self.get(j));
        self.set(j, tmp);
    }
}
```
```rust
impl DenseVec for TVector3d {
    fn len(&self) -> usize {
        3
    }
    fn get(&self, i: usize) -> f64 {
        self[i]
    }
    fn set(&mut self, i: usize, v: f64) {
        self[i] = v;
    }
}
```

---

# 테스트 코드

## ✅ 테스트 코드 요약
| 테스트 이름                          | 검증 내용                                      | 통과 여부 |
|-------------------------------------|-----------------------------------------------|-----------|
| `tv2_basics`                        | TVector2 기본 생성, 내적, 길이 계산           | ✅        |
| `tv2_ops_scalar_and_componentwise` | TVector2 연산자 오버로드                      | ✅        |
| `tv2_cross_and_normalize`          | TVector2 외적 및 정규화                       | ✅        |
| `tv2_min_max_negate_equals_eps`    | TVector2 성분 비교 및 오차 비교               | ✅        |
| `tv2_index_oob_panics`             | TVector2 인덱스 범위 초과 확인                | ✅ (panic)|
| `tv3_basics_and_dot_length`        | TVector3 기본 생성, 내적, 길이 계산           | ✅        |
| `tv3_cross_right_hand_rule`        | TVector3 외적 및 직교성 확인                  | ✅        |
| `tv3_normalize_zero_and_minmax`    | TVector3 정규화 및 성분 비교                  | ✅        |
| `tv3_scalar_and_componentwise_ops` | TVector3 연산자 오버로드                      | ✅        |
| `tv3_equals_eps`                   | TVector3 오차 기반 비교                       | ✅        |
| `tv3_index_oob_panics`             | TVector3 인덱스 범위 초과 확인                | ✅ (panic)|
| `tv4_basics_and_norm`              | TVector4 생성, 정규화, negate, 성분 비교      | ✅        |
| `tv4_ops`                          | TVector4 연산자 오버로드                      | ✅        |
| `tv4_equals_eps`                   | TVector4 오차 기반 비교                       | ✅        |
| `tv4_index_oob_panics`             | TVector4 인덱스 범위 초과 확인                | ✅ (panic)|
| `tv2i_tv3i_tv4i_basic_ops`         | 정수형 벡터 연산자 오버로드                   | ✅        |
| `scalar_distributivity_vec2_vec3_vec4` | 스칼라 분배법칙 확인                     | ✅        |
| `tv4_from_v3_sets_w_default`       | TVector3 → TVector4 변환                      | ✅        |
| `tv2_tv3_tv4_normalize_unit_length`| 정규화 후 길이 1 확인                         | ✅        |
| `tv2_tv3_tv4_equals_eps_exact_match`| 동일 벡터 오차 비교                          | ✅        |
| `tv2_tv3_tv4_neg_vs_negate`        | negate vs - 연산자 비교                      | ✅        |
| `tv2_tv3_tv4_index_mut_sets_value` | 인덱스 수정 확인                             | ✅        |
| `tv2_tv3_tv4_dot_symmetry`         | 내적의 대칭성 확인                           | ✅        |
| `tv3_cross_antisymmetry_and_self_zero` | 외적 반대성 및 자기 외적 0 확인         | ✅        |
| `tv2_tv3_tv4_mul_div_identity`     | 스칼라 곱/나눗셈 항등 확인                   | ✅        |
| `tv2_tv3_tv4_fill_sets_all_components` | fill 함수 확인                           | ✅        |


```rust
#[cfg(test)]
mod tests_tvector {
    use nurbslib::approx_f64;
    use nurbslib::core::tarray::TArray;
    use nurbslib::core::tvector::{TVector2, TVector3, TVector4};

    #[test]
    fn tarray_test() {
        let mut a = TArray::with_size(3); // [T::default(); 3]
        println!("{:?}", a);

        a.fill(1i32);
        println!("{:?}", a);
        a.append(5);
        println!("{:?}", a);
        let _ = a.insert_at(1, 9); // [1,9,1,1,5]
        println!("{:?}", a);
        a.rotate_left(); // [9,1,1,5,1]
        println!("{:?}", a);
        let _ = a.remove_at(2); // [9,1,5,1]
        println!("{:?}", a);
        a.reverse(); // [1,5,1,9]
        println!("{:?}", a);

        let b = TArray::from([10, 20, 30].as_slice());
        println!("{:?}", b);
        a += &b; // in-place append
        assert_eq!(a.len(), 7);
        println!("{:?}", a);

        let s = a.sum_f64(); // f64 합계
        println!("{:?}", a);
        let ks = a.kahan_sum_f64(); // 카한합 (f64)
        println!("a = {a} (sum={s}, kahan={ks})");
    }
```
```rust
    #[test]
    fn tv2_basics() {
        let mut v = TVector2::<f64>::new(3.0, -4.0);
        assert_eq!(v[0], 3.0);
        assert_eq!(v[1], -4.0);

        v.set_value(1.5, 2.5);
        assert_eq!(v.get_value(), [1.5, 2.5]);

        let d = v.dot(&TVector2::new(-2.0, 1.0)); // 1.5*(-2)+2.5*1 = -3+2.5 = -0.5
        assert_eq!(d, -0.5);

        assert_eq!(v.length_squared(), 1.5 * 1.5 + 2.5 * 2.5);
        assert!(approx_f64!(
            v.length(),
            ((1.5 * 1.5 + 2.5 * 2.5) as f64).sqrt(),
            1e-12_f64
        ));
    }
```
```rust
    #[test]
    fn tv2_ops_scalar_and_componentwise() {
        let a = TVector2::<f64>::new(2.0, -3.0);
        let b = TVector2::<f64>::new(4.0, 5.0);

        // 성분별 +,-,*, / (스칼라)
        let c = a + b;
        assert_eq!(c, TVector2::new(6.0, 2.0));

        let d = a - b;
        assert_eq!(d, TVector2::new(-2.0, -8.0));

        let e = a * b; // 성분별 곱
        assert_eq!(e, TVector2::new(8.0, -15.0));

        let f = a * 2.0; // 스칼라 곱
        assert_eq!(f, TVector2::new(4.0, -6.0));

        let g = b / 2.0; // 스칼라 나눗셈
        assert_eq!(g, TVector2::new(2.0, 2.5));
    }
```
```rust
    #[test]
    fn tv2_cross_and_normalize() {
        let mut v = TVector2::<f64>::new(3.0, 4.0);
        let len = v.normalize();
        assert!(approx_f64!(len, 5.0, 1e-12));
        assert!(approx_f64!(v.length(), 1.0, 1e-12));

        // 2D cross 는 z-스칼라
        let x = TVector2::new(1.0, 0.0);
        let y = TVector2::new(0.0, 1.0);
        assert_eq!(x.cross(&y), 1.0); // 오른손 법칙: +z
        assert_eq!(y.cross(&x), -1.0); // 반대
    }
```
```rust
    #[test]
    fn tv2_min_max_negate_equals_eps() {
        let mut v = TVector2::<f64>::new(-2.0, 1.0);
        assert_eq!(v.max_component(), 1.0);
        assert_eq!(v.min_component(), -2.0);

        v.negate();
        assert_eq!(v, TVector2::new(2.0, -1.0));

        let a = TVector2::new(1.0, 1.0);
        let b = TVector2::new(1.0 + 1e-6, 1.0 - 1e-6);
        assert!(a.equals_eps(&b, 2e-6));
        assert!(!a.equals_eps(&b, 1e-9));
    }
```
```rust
    #[test]
    #[should_panic]
    fn tv2_index_oob_panics() {
        let v = TVector2::<f64>::new(0.0, 0.0);
        let _ = v[2]; // panic
    }
```
```rust
    // ---------------- TVector3<f64> ----------------
    #[test]
    fn tv3_basics_and_dot_length() {
        let mut v = TVector3::<f64>::new(1.0, 2.0, 2.0);
        assert_eq!(v.get_value(), [1.0, 2.0, 2.0]);
        assert_eq!(v.length_squared(), 1.0 + 4.0 + 4.0);
        assert!(approx_f64!(v.length(), 3.0, 1e-12));

        v.set_value(-1.0, 0.5, 2.0);
        let u = TVector3::new(2.0, 0.0, -1.0);
        assert_eq!(v.dot(&u), -1.0 * 2.0 + 0.5 * 0.0 + 2.0 * (-1.0)); // -4
    }
```
```rust
    #[test]
    fn tv3_cross_right_hand_rule() {
        let i = TVector3::<f64>::new(1.0, 0.0, 0.0);
        let j = TVector3::<f64>::new(0.0, 1.0, 0.0);
        let k = TVector3::<f64>::new(0.0, 0.0, 1.0);

        assert_eq!(i.cross(&j), k);
        assert_eq!(j.cross(&k), i);
        assert_eq!(k.cross(&i), j);

        // 직교성: (a×b)·a = 0, (a×b)·b = 0
        let a = TVector3::new(3.0, -1.0, 2.0);
        let b = TVector3::new(0.5, 4.0, -1.0);
        let c = a.cross(&b);
        assert!(approx_f64!(c.dot(&a), 0.0, 1e-12));
        assert!(approx_f64!(c.dot(&b), 0.0, 1e-12));
    }
```
```rust
    #[test]
    fn tv3_normalize_zero_and_minmax() {
        let mut z = TVector3::<f64>::new(0.0, 0.0, 0.0);
        let len = z.normalize();
        assert_eq!(len, 0.0);
        assert_eq!(z, TVector3::new(0.0, 0.0, 0.0));

        let v = TVector3::<f64>::new(-2.0, 5.0, 1.0);
        assert_eq!(v.max_component(), 5.0);
        assert_eq!(v.min_component(), -2.0);
    }
```
```rust
    #[test]
    fn tv3_scalar_and_componentwise_ops() {
        let a = TVector3::<f64>::new(2.0, -3.0, 4.0);
        let b = TVector3::<f64>::new(5.0, 6.0, -1.0);

        assert_eq!(a + b, TVector3::new(7.0, 3.0, 3.0));
        assert_eq!(a - b, TVector3::new(-3.0, -9.0, 5.0));
        assert_eq!(a * b, TVector3::new(10.0, -18.0, -4.0)); // 성분곱
        assert_eq!(a * 2.0, TVector3::new(4.0, -6.0, 8.0));
        assert_eq!(b / 2.0, TVector3::new(2.5, 3.0, -0.5));
    }
```
```rust
    #[test]
    fn tv3_equals_eps() {
        let a = TVector3::<f64>::new(1.0, 2.0, 3.0);
        let b = TVector3::<f64>::new(1.0 + 1e-6, 2.0 - 1e-6, 3.0 + 5e-7);
        assert!(a.equals_eps(&b, 2e-6));
        assert!(!a.equals_eps(&b, 1e-9));
    }
```
```rust
    #[test]
    #[should_panic]
    fn tv3_index_oob_panics() {
        let v = TVector3::<f64>::new(0.0, 0.0, 0.0);
        let _ = v[3]; // panic
    }
```
```rust
    // ---------------- TVector4<f64> ----------------
    #[test]
    fn tv4_basics_and_norm() {
        let dist: f64 = 3.1622776601683795_f64;
        let mut v = TVector4::<f64>::new(1.0, -2.0, 2.0, -1.0);
        assert_eq!(v.get_value(), [1.0, -2.0, 2.0, -1.0]);
        assert!(approx_f64!(
            v.length_squared(),
            1.0 + 4.0 + 4.0 + 1.0,
            1e-12
        ));
        let len = v.normalize();
        assert!(approx_f64!(len, dist, 1e-12_f64));
        assert!(approx_f64!(v.length(), 1.0_f64, 1e-12_f64));

        v.negate();
        assert!(approx_f64!(v.x, -1.0 / dist, 1e-12));
        assert!(approx_f64!(v.y, 2.0 / dist, 1e-12));
        assert!(approx_f64!(v.z, -2.0 / dist, 1e-12));
        assert!(approx_f64!(v.w, 1.0 / dist, 1e-12));

        assert!(approx_f64!(v.max_component(), 2.0 / dist, 1e-12));
        assert!(approx_f64!(v.min_component(), -2.0 / dist, 1e-12));
    }
```
```rust
    #[test]
    fn tv4_ops() {
        let a = TVector4::<f64>::new(1.0, 2.0, 3.0, 4.0);
        let b = TVector4::<f64>::new(-2.0, 1.0, 0.5, 8.0);
        assert_eq!(a + b, TVector4::new(-1.0, 3.0, 3.5, 12.0));
        assert_eq!(a - b, TVector4::new(3.0, 1.0, 2.5, -4.0));
        assert_eq!(a * b, TVector4::new(-2.0, 2.0, 1.5, 32.0)); // 성분곱
        assert_eq!(a * 2.0, TVector4::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(b / 2.0, TVector4::new(-1.0, 0.5, 0.25, 4.0));
    }
```
```rust
    #[test]
    fn tv4_equals_eps() {
        let a = TVector4::<f64>::new(1.0, 2.0, 3.0, 4.0);
        let b = TVector4::<f64>::new(1.0 + 5e-7, 2.0 - 5e-7, 3.0, 4.0 + 5e-7);
        assert!(a.equals_eps(&b, 2e-6));
        assert!(!a.equals_eps(&b, 1e-9));
    }
```
```rust
    #[test]
    #[should_panic]
    fn tv4_index_oob_panics() {
        let v = TVector4::<f64>::new(0.0, 0.0, 0.0, 0.0);
        let _ = v[4]; // panic
    }
```
```rust
    // ---------------- 정수형 기본 연산 ----------------
    #[test]
    fn tv2i_tv3i_tv4i_basic_ops() {
        // TVector2i
        let a2 = TVector2::<i32>::new(2, -3);
        let b2 = TVector2::<i32>::new(5, 1);
        assert_eq!(a2 + b2, TVector2::new(7, -2));
        assert_eq!(a2 - b2, TVector2::new(-3, -4));
        assert_eq!(a2 * b2, TVector2::new(10, -3)); // 성분곱

        // TVector3i
        let a3 = TVector3::<i32>::new(1, 2, 3);
        let b3 = TVector3::<i32>::new(-1, 4, 0);
        assert_eq!(a3 + b3, TVector3::new(0, 6, 3));
        assert_eq!(a3 - b3, TVector3::new(2, -2, 3));
        assert_eq!(a3 * b3, TVector3::new(-1, 8, 0));

        // TVector4i
        let a4 = TVector4::<i32>::new(1, -2, 3, -4);
        let b4 = TVector4::<i32>::new(0, 5, 2, 1);
        assert_eq!(a4 + b4, TVector4::new(1, 3, 5, -3));
        assert_eq!(a4 - b4, TVector4::new(1, -7, 1, -5));
        assert_eq!(a4 * b4, TVector4::new(0, -10, 6, -4));
    }
```
```rust
    // ---------------- 분배법칙(스칼라) ----------------
    #[test]
    fn scalar_distributivity_vec2_vec3_vec4() {
        let s = 2.5_f64;

        let a2 = TVector2::new(1.0, -2.0);
        let b2 = TVector2::new(3.0, 4.0);
        assert_eq!((a2 + b2) * s, (a2 * s) + (b2 * s));

        let a3 = TVector3::new(2.0, -1.0, 0.5);
        let b3 = TVector3::new(-3.0, 4.0, 2.0);
        assert_eq!((a3 + b3) * s, (a3 * s) + (b3 * s));

        let a4 = TVector4::new(1.0, 2.0, 3.0, 4.0);
        let b4 = TVector4::new(0.5, -1.0, 6.0, -2.0);
        assert_eq!((a4 + b4) * s, (a4 * s) + (b4 * s));
    }
}
```
```rust
#[cfg(test)]
mod tests_tvector_extra {
    use rand::Rng;
    use nurbslib::approx_f64;
    use nurbslib::core::tvector::{TVector2, TVector3, TVector4};

    #[test]
    fn tv4_from_v3_sets_w_default() {
        let v3 = TVector3::<f64>::new(1.0, 2.0, 3.0);
        let v4 = TVector4::from_v3(v3);
        assert_eq!(v4.get_value(), [1.0, 2.0, 3.0, 0.0]);
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_normalize_unit_length() {
        let mut v2 = TVector2::new(3.0, 4.0);
        let mut v3 = TVector3::new(1.0, 2.0, 2.0);
        let mut v4 = TVector4::new(1.0, 2.0, 2.0, 1.0);

        v2.normalize();
        v3.normalize();
        v4.normalize();

        assert!(approx_f64!(v2.length(), 1.0, 1e-12));
        assert!(approx_f64!(v3.length(), 1.0, 1e-12));
        assert!(approx_f64!(v4.length(), 1.0, 1e-12));
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_equals_eps_exact_match() {
        let a2 = TVector2::new(1.0, 2.0);
        let a3 = TVector3::new(1.0, 2.0, 3.0);
        let a4 = TVector4::new(1.0, 2.0, 3.0, 4.0);

        assert!(a2.equals_eps(&a2, 1e-12));
        assert!(a3.equals_eps(&a3, 1e-12));
        assert!(a4.equals_eps(&a4, 1e-12));
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_neg_vs_negate() {
        let mut v2 = TVector2::new(1.0, -2.0);
        let mut v3 = TVector3::new(1.0, -2.0, 3.0);
        let mut v4 = TVector4::new(1.0, -2.0, 3.0, -4.0);

        let neg2 = -v2;
        let neg3 = -v3;
        let neg4 = -v4;

        v2.negate();
        v3.negate();
        v4.negate();

        assert_eq!(v2, neg2);
        assert_eq!(v3, neg3);
        assert_eq!(v4, neg4);
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_index_mut_sets_value() {
        let mut v2 = TVector2::new(0.0, 0.0);
        let mut v3 = TVector3::new(0.0, 0.0, 0.0);
        let mut v4 = TVector4::new(0.0, 0.0, 0.0, 0.0);

        v2[1] = 2.0;
        v3[2] = 3.0;
        v4[3] = 4.0;

        assert_eq!(v2[1], 2.0);
        assert_eq!(v3[2], 3.0);
        assert_eq!(v4[3], 4.0);
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_dot_symmetry() {
        let a2 = TVector2::new(1.0, 2.0);
        let b2 = TVector2::new(3.0, 4.0);
        assert_eq!(a2.dot(&b2), b2.dot(&a2));

        let a3 = TVector3::new(1.0, 2.0, 3.0);
        let b3 = TVector3::new(4.0, 5.0, 6.0);
        assert_eq!(a3.dot(&b3), b3.dot(&a3));

        let a4 = TVector4::new(1.0, 2.0, 3.0, 4.0);
        let b4 = TVector4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a4.dot(&b4), b4.dot(&a4));
    }
```
```rust
    #[test]
    fn tv3_cross_antisymmetry_and_self_zero() {
        let a = TVector3::new(1.0, 2.0, 3.0);
        let b = TVector3::new(4.0, 5.0, 6.0);
        let c1 = a.cross(&b);
        let c2 = b.cross(&a);
        assert_eq!(c1, -c2);

        let self_cross = a.cross(&a);
        assert_eq!(self_cross, TVector3::new(0.0, 0.0, 0.0));
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_mul_div_identity() {
        let v2 = TVector2::new(1.0, -2.0);
        let v3 = TVector3::new(3.0, 0.0, -1.0);
        let v4 = TVector4::new(2.0, -2.0, 4.0, -4.0);

        assert_eq!(v2 * 1.0, v2);
        assert_eq!(v3 * 1.0, v3);
        assert_eq!(v4 * 1.0, v4);

        assert_eq!(v2 / 1.0, v2);
        assert_eq!(v3 / 1.0, v3);
        assert_eq!(v4 / 1.0, v4);
    }
```
```rust
    #[test]
    fn tv2_tv3_tv4_fill_sets_all_components() {
        let mut v2 = TVector2::new(0.0, 0.0);
        let mut v3 = TVector3::new(0.0, 0.0, 0.0);
        let mut v4 = TVector4::new(0.0, 0.0, 0.0, 0.0);

        v2.fill(1.5);
        v3.fill(-2.0);
        v4.fill(3.0);

        assert_eq!(v2.get_value(), [1.5, 1.5]);
        assert_eq!(v3.get_value(), [-2.0, -2.0, -2.0]);
        assert_eq!(v4.get_value(), [3.0, 3.0, 3.0, 3.0]);
    }
}
```
---
