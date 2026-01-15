# TensorProduct

## 🔎 물리적 의미 설명
### 1. TensorProduct (추상 클래스)
- 역할: 두 개의 매개변수 공간 A,B에서 정의된 점을 받아서 결과 공간 C의 점을 계산하는 **텐서 곱** 연산을 정의하는 인터페이스.
- 수학적으로는 함수

$$
T:\mathbb{R^{\mathrm{dimA}}}\times \mathbb{R^{\mathrm{dimB}}}\rightarrow \mathbb{R^{\mathrm{dimC}}}
$$

- 를 의미합니다.
- 선형성 조건을 만족해야 합니다:
  - $T(aA_0+(1-a)A_1,B)=aT(A_0,B)+(1-a)T(A_1,B)$
  - $T(A,bB_0+(1-b)B_1)=bT(A,B_0)+(1-b)T(A,B_1)$
- 즉, 곡선과 곡선을 결합해 곡면을 만드는 규칙을 정의합니다.

### 2. RevolutionTensor
- 역할: 어떤 곡선을 축 주위로 회전시켜서 곡면을 생성하는 텐서.
- 입력:
  - ArcPoint → 회전 각도에 해당하는 ($\cos \theta$, $\sin \theta$ ) 값
  - ShapePoint → 회전시킬 원래 곡선의 점
- 출력:
  - SrfPoint → 회전 후 3D 공간상의 점
- 물리적 의미:
  - 원점 O와 축 벡터들 X, Y, Z를 기준으로, 곡선을 회전시켜 **회전체 곡면(surface of revolution)** 을 만듭니다.
  - 예: 원을 회전시키면 구, 직선을 회전시키면 원기둥.

3. SumTensor
- 역할: 두 곡선을 단순히 더해서 곡면을 생성.
- 입력:
  - CurveA, CurveB → 두 곡선의 점
- 출력:
- SrfPoint → 두 곡선의 선형 결합 + 기준점(basepoint)
- 물리적 의미:
  - 두 곡선을 더해서 **합성 곡면** 을 만듭니다.
  - 예: 두 곡선이 각각 x축, y축 방향이면, 합치면 평면이 됩니다.

## 🦀 Rust 코드 변환
- 아래는 위 OpenNURB C++ 클래스를 Rust trait + struct로 옮긴 버전입니다.
```rust
/// Trait: TensorProduct
/// Defines a mapping T: R^dimA x R^dimB -> R^dimC
pub trait TensorProduct {
    fn dimension_a(&self) -> usize;
    fn dimension_b(&self) -> usize;
    fn dimension_c(&self) -> usize;

    fn evaluate(&self, a: f64, A: &[f64], b: f64, B: &[f64]) -> Vec<f64>;
}
```
```rust
/// 3D Point/Vector
#[derive(Clone, Copy)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```
```rust
pub type Vector3 = Point3;
```
```rust
/// RevolutionTensor: generates surface of revolution
pub struct RevolutionTensor {
    pub O: Point3,
    pub X: Vector3,
    pub Y: Vector3,
    pub Z: Vector3,
}
```
```rust
impl TensorProduct for RevolutionTensor {
    fn dimension_a(&self) -> usize { 2 }
    fn dimension_b(&self) -> usize { 3 }
    fn dimension_c(&self) -> usize { 3 }

    fn evaluate(&self, a: f64, arc_point: &[f64], b: f64, shape_point: &[f64]) -> Vec<f64> {
        let mut A = [arc_point[0], arc_point[1]];
        if a != 1.0 {
            A[0] *= a;
            A[1] *= a;
        }

        let mut B = [shape_point[0], shape_point[1], shape_point[2]];
        if b != 1.0 {
            B[0] *= b;
            B[1] *= b;
            B[2] *= b;
        }

        let x = (B[0] - self.O.x) * self.X.x + (B[1] - self.O.y) * self.X.y + (B[2] - self.O.z) * self.X.z;
        let y = (B[0] - self.O.x) * self.Y.x + (B[1] - self.O.y) * self.Y.y + (B[2] - self.O.z) * self.Y.z;
        let z = (B[0] - self.O.x) * self.Z.x + (B[1] - self.O.y) * self.Z.y + (B[2] - self.O.z) * self.Z.z;

        let c = A[0];
        let s = A[1];

        let rx = c * x - s * y;
        let ry = s * x + c * y;

        vec![
            self.O.x + rx * self.X.x + ry * self.Y.x + z * self.Z.x,
            self.O.y + rx * self.X.y + ry * self.Y.y + z * self.Z.y,
            self.O.z + rx * self.X.z + ry * self.Y.z + z * self.Z.z,
        ]
    }
}
```
```rust
/// SumTensor: adds two curves to form a surface
pub struct SumTensor {
    pub dim: usize,
    pub basepoint: Point3,
}
```
```rust
impl TensorProduct for SumTensor {
    fn dimension_a(&self) -> usize { self.dim }
    fn dimension_b(&self) -> usize { self.dim }
    fn dimension_c(&self) -> usize { self.dim }

    fn evaluate(&self, a: f64, curve_a: &[f64], b: f64, curve_b: &[f64]) -> Vec<f64> {
        vec![
            a * curve_a[0] + b * curve_b[0] + self.basepoint.x,
            a * curve_a[1] + b * curve_b[1] + self.basepoint.y,
            a * curve_a[2] + b * curve_b[2] + self.basepoint.z,
        ]
    }
}
```
---


## ✨ 요약
- RevolutionTensor: 곡선을 회전시켜서 곡면을 만드는 연산 → 회전체 곡면
- SumTensor: 두 곡선을 더해서 곡면을 만드는 연산 → 합성 곡면
- Rust에서는 trait TensorProduct로 인터페이스를 정의하고, struct RevolutionTensor, struct SumTensor로 구현했습니다.
