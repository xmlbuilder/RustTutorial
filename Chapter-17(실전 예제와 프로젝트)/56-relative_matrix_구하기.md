# Relative Matrix
두 비행기의 위치와 자세(orientation)를 글로벌 좌표계에서 알고 있을 때,  
한 비행기 기준 좌표계에서 다른 비행기의 상대 움직임을 표현하는 변환 행렬을 구하는 문제입니다.

## 📌 개념 정리
- 각 비행기는 글로벌 좌표계에서 위치 벡터와 **자세 행렬(회전 행렬)** 을 가짐
- $T_1=[R_1|P_1]$ : 비행기 1의 변환 행렬 (회전 $R_1$, 위치 $P_1$ )
- $T_2=[R_2|P_2]$ : 비행기 2의 변환 행렬
- 비행기 1 기준에서 비행기 2를 표현하려면:

$$
T_{1\rightarrow 2}=T_1^{-1}\cdot T_2
$$

- 반대로 비행기 2 기준에서 비행기 1을 표현하려면:

$$
T_{2\rightarrow 1}=T_2^{-1}\cdot T_1
$$

## 📊 Rust 코드 예시
아래는 **4x4 변환 행렬(회전 + 평행이동)** 을 사용해 상대 변환을 구하는 함수입니다.
```rust
use nalgebra::{Matrix4};

/// 상대 변환 행렬을 구하는 함수
/// T_a: 비행기 A의 글로벌 좌표계 변환 행렬
/// T_b: 비행기 B의 글로벌 좌표계 변환 행렬
/// 반환값: A 기준에서 본 B의 변환 행렬
fn relative_transform(T_a: Matrix4<f64>, T_b: Matrix4<f64>) -> Matrix4<f64> {
    // A의 변환 행렬 역행렬을 구함
    let T_a_inv = T_a.try_inverse().expect("Matrix not invertible");
    // 상대 변환 = A^-1 * B
    T_a_inv * T_b
}
```
```rust
fn main() {
    // 예시: 단순한 변환 행렬 (여기서는 단위 행렬로 가정)
    let T1 = Matrix4::<f64>::identity(); // 비행기 1
    let T2 = Matrix4::<f64>::new_translation(&nalgebra::Vector3::new(10.0, 0.0, 0.0)); // 비행기 2

    // 비행기 1 기준에서 본 비행기 2
    let rel_1_to_2 = relative_transform(T1, T2);
    println!("비행기 1 기준에서 본 비행기 2 변환 행렬:\n{}", rel_1_to_2);

    // 비행기 2 기준에서 본 비행기 1
    let rel_2_to_1 = relative_transform(T2, T1);
    println!("비행기 2 기준에서 본 비행기 1 변환 행렬:\n{}", rel_2_to_1);
}
```

## 📌 요약
- 👉 글로벌 좌표계에서 두 비행기의 변환 행렬을 알고 있다면, 상대 변환 행렬은 단순히 한쪽의 역행렬과 다른 쪽의 행렬을 곱해서 구할 수 있습니다.
- 👉 Rust에서는 nalgebra 같은 선형대수 라이브러리를 활용하면 쉽게 구현할 수 있습니다.

---

## Global Matrix

비행기 1의 글로벌(절대) 좌표계 변환 행렬과 비행기 1 기준에서 본 비행기 2의 상대 변환 행렬을 알고 있을 때,  
비행기 2의 글로벌 좌표계 변환 행렬을 구하는 문제입니다.

## 📌 수학적 관계
- $T_1$: 비행기 1의 글로벌 좌표계 변환 행렬 (4x4, 회전+평행이동)
- $T_{1\rightarrow 2}$: 비행기 1 기준에서 본 비행기 2의 상대 변환 행렬
- $T_2$: 비행기 2의 글로벌 좌표계 변환 행렬
- 관계식은 다음과 같습니다:

$$  
T_2=T_1\cdot T_{1\rightarrow 2}
$$

- 즉, 비행기 2의 절대 좌표계 변환 행렬은 비행기 1의 절대 좌표계 변환 행렬과 상대 변환 행렬의 곱으로 구할 수 있습니다.

## 📊 Rust 코드 예시 (nalgebra 사용)
```rust
use nalgebra::{Matrix4};

/// 비행기 2의 글로벌 좌표계 변환 행렬을 구하는 함수
/// T1: 비행기 1의 글로벌 좌표계 변환 행렬
/// T_rel: 비행기 1 기준에서 본 비행기 2의 상대 변환 행렬
/// 반환값: 비행기 2의 글로벌 좌표계 변환 행렬
fn global_transform_plane2(T1: Matrix4<f64>, T_rel: Matrix4<f64>) -> Matrix4<f64> {
    T1 * T_rel
}
```
```rust
fn main() {
    // 예시: 비행기 1은 원점에 있고, 단위 행렬로 표현
    let T1 = Matrix4::<f64>::identity();

    // 예시: 비행기 1 기준에서 비행기 2는 x축으로 10m 이동
    let T_rel = Matrix4::<f64>::new_translation(&nalgebra::Vector3::new(10.0, 0.0, 0.0));

    // 비행기 2의 글로벌 좌표계 변환 행렬 계산
    let T2 = global_transform_plane2(T1, T_rel);

    println!("비행기 2의 글로벌 좌표계 변환 행렬:\n{}", T2);
}
```

## 📌 요약
- 👉 비행기 2의 절대 좌표계 변환 행렬은 비행기 1의 절대 좌표계 변환 행렬 × 비행기 1 기준 상대 변환 행렬로 계산합니다.
- 👉 Rust에서는 nalgebra 라이브러리를 활용해 간단히 구현할 수 있습니다.


```mermaid
flowchart LR
    G[Global Coordinate System] --> T1[Plane1 Transform - T1]
    T1 --> Rel[Relative Transform  - T1→2]
    Rel --> T2[Plane2 Global Transform - T2]
```

## 📌 설명
- Global Coordinate System (G): 절대 좌표계 기준
- Plane1 Transform (T1): 비행기 1의 글로벌 좌표계 변환 행렬
- Relative Transform (T1→2): 비행기 1 기준에서 본 비행기 2의 상대 변환
- Plane2 Global Transform (T2): 최종적으로 계산된 비행기 2의 글로벌 좌표계 변환

---


##  1. relative_transform($T_a$, $T_b$) = $T_a⁻¹ * T_b$ 검증
```rust
fn relative_transform(T_a: Matrix4<f64>, T_b: Matrix4<f64>) -> Matrix4<f64> {
    let T_a_inv = T_a.try_inverse().expect("Matrix not invertible");
    T_a_inv * T_b
}
```

- 가정:

- $T_a$ : 세계좌표계(World)에서 A 좌표계로 가는 변환
  - 보통 표기로는 $T_{WA}$ (A-frame → World-frame)
- 즉, $P_W = T_{WA} * P_A$

- $T_b$ : 세계좌표계에서 B 좌표계로 가는 변환 $T_{WB}$

- 이때 B를 A 기준에서 보고 싶으면:

$$
P_W = T_{WA} * P_A
$$

$$
P_W = T_{WB} * P_B
$$

- 따라서

$$
T_{WA} * P_A = T_{WB} * P_B
$$


$$
P_A = T_{WA}⁻¹ * T_{WB} * P_B
$$

- 여기서

$$
T_{rel} = T_{A B} = T_{WA}⁻¹ * T_{WB}
$$

- 이고, 이게 바로 코드에서 계산한 

$$
T_{a_inv} * T_b
$$ 

- 입니다.

- 즉

**A 기준에서 본 B의 변환** = $T_a⁻¹ * T_b$ → ✔ 맞음

- 예제 코드도 논리적으로 타당한지
```rust
let T1 = Matrix4::<f64>::identity(); // 비행기 1 (원점)
let T2 = Matrix4::<f64>::new_translation(&nalgebra::Vector3::new(10.0, 0.0, 0.0)); // 비행기 2
```
- $T1$: 원점
- $T2$: x축으로 +10m 이동

- 상대 변환:
```rust
let rel_1_to_2 = relative_transform(T1, T2); // = I⁻¹ * T2 = T2
let rel_2_to_1 = relative_transform(T2, T1); // = T2⁻¹ * I = T2⁻¹
```
- `rel_1_to_2` 는 “1 기준에서 본 2” → (10,0,0) → OK
- `rel_2_to_1` 는 “2 기준에서 본 1” → (-10,0,0) → OK

- 즉, 의도한 상대 위치/자세 해석과 일치합니다.

## 2. $T_2 = T_1 * T_{rel}$ 검증

- 두 번째 수식은:
- “비행기 1의 글로벌 변환 T1과 비행기 1 기준에서 비행기 2로 가는 상대 변환 T_rel이 있을 때, 비행기 2의 글로벌 변환 T2를 구하는 식”
- 코드:
```rust
fn global_transform_plane2(T1: Matrix4<f64>, T_rel: Matrix4<f64>) -> Matrix4<f64> {
    T1 * T_rel
}
```
- 여기서:

$T1 = T_{W1}$ : 1-frame → World-frame

$T_{rel} = T_{1→2} = T_{12}$ : 2-frame → 1-frame (좌표를 2에서 1로 표현하는 변환)

그럼 2의 World 변환은:

$$
P_W = T_{W1} * P_1
$$

$$
P_1 = T_{12} * P_2
$$

따라서

$$
P_W = T_{W1} * T_{12} * P_2
$$

즉

$T_{W2} = T_{W1} * T_{12}$

→ 코드의 T2 = T1 * T_rel 과 정확히 일치합니다.
그리고 앞에서 만든 상대 변환이

$$
T_{rel} = T_{1→2} = {T1}⁻¹ * T2
$$

였다면,
```
T2' = T1 * T_rel
    = T1 * (T1⁻¹ * T2)
    = T2
```
로 정확히 원래 T2를 복원합니다. 그래서 두 수식은 서로 호환되고, 일관된 정의입니다.

## 정리

- 상대 변환
  - 코드의 relative_transform 구현은 이 정의와 일치.

$$
T_{rel}(A 기준에서 본 B) = T_A⁻¹ * T_B
$$


- 글로벌 변환 복원
  - 코드의 $global\\_transform\\_plane2(T1, T_{rel}) = T1 * T_{rel}$ 도 일치.

$$
T_B = T_A * T_{rel}
$$

---

## 소스 코드

```rust
use nalgebra::Matrix4;

/// 좌표계(Frame) 사이의 관계를 다루는 유틸 모음.
/// 모든 행렬은 다음 의미를 가진다고 가정:
///   t_wa : A-frame 에서 World-frame 으로 가는 변환 (p_w = t_wa * p_a)
pub struct FrameTransform;

impl FrameTransform {
    /// A 기준에서 본 B 의 변환 (t_a_b)를 계산
    ///
    /// 입력:
    ///   - t_wa : A-frame → World-frame
    ///   - t_wb : B-frame → World-frame
    ///
    /// 출력:
    ///   - T_a_b : B-frame → A-frame
    ///
    /// 수식:
    ///   t_a_b = t_wa^{-1} * t_wb
    pub fn relative_b_in_a(t_wa: &Matrix4<f64>, t_wb: &Matrix4<f64>) -> Matrix4<f64> {
        let t_aw = t_wa
            .try_inverse()
            .expect("t_wa is not invertible (singular transform)");
        t_aw * t_wb
    }

    /// A의 World 변환과 "A 기준에서 본 B" 변환으로 B의 World 변환을 계산
    ///
    /// 입력:
    ///   - t_wa : A-frame → World-frame
    ///   - t_a_b : B-frame → A-frame
    ///
    /// 출력:
    ///   - t_wb : B-frame → World-frame
    ///
    /// 수식:
    ///   t_wb = t_wa * t_a_b
    pub fn world_from_relative(t_wa: &Matrix4<f64>, t_a_b: &Matrix4<f64>) -> Matrix4<f64> {
        t_wa * t_a_b
    }

    /// 단순히 "A 기준에서 본 B" 를 알고 있을 때,
    /// 그 반대인 "B 기준에서 본 A" 를 구하고 싶을 때 사용.
    ///
    /// 입력:
    ///   - t_a_b : B-frame → A-frame
    /// 출력:
    ///   - t_b_a : A-frame → B-frame
    ///
    /// 수식:
    ///   t_b_a = (t_a_b)^{-1}
    pub fn invert_relative(t_a_b: &Matrix4<f64>) -> Matrix4<f64> {
        t_a_b
            .try_inverse()
            .expect("t_a_b is not invertible (singular transform)")
    }

    /// World 기준에서 본 두 물체(예: 비행기 1, 비행기 2)의 변환을 주었을 때,
    /// - 1 기준에서 본 2 (T_1_2)
    /// - 2 기준에서 본 1 (T_2_1)
    /// 를 둘 다 반환하는 헬퍼
    pub fn mutual_relative(
        t_w1: &Matrix4<f64>,
        t_w2: &Matrix4<f64>,
    ) -> (Matrix4<f64>, Matrix4<f64>) {
        let t_1_2 = Self::relative_b_in_a(t_w1, t_w2); // 1 기준에서 본 2
        let t_2_1 = Self::relative_b_in_a(t_w2, t_w1); // 2 기준에서 본 1
        (t_1_2, t_2_1)
    }
}
```

### 테스트 코드
```rust
use nalgebra::{Matrix4, Vector3};

/// 비행기 2의 글로벌 좌표계 변환 행렬을 구하는 함수
fn global_transform_t2(t1: Matrix4<f64>, t_rel: Matrix4<f64>) -> Matrix4<f64> {
    t1 * t_rel
}

#[cfg(test)]
mod tests_case1 {
    use super::*;
    use nalgebra::Translation3;

    #[test]
    fn test_t2_global_transform() {
        // 비행기 1: 원점에 위치 (단위 행렬)
        let t1 = Matrix4::<f64>::identity();

        // 비행기 1 기준에서 비행기 2: x축으로 10m 이동
        let t_rel = Translation3::new(10.0, 0.0, 0.0).to_homogeneous();

        // 비행기 2의 글로벌 좌표계 변환 계산
        let t2 = global_transform_t2(t1, t_rel);

        // 기대값: 글로벌 좌표계에서 (10,0,0) 위치
        let expected = Translation3::new(10.0, 0.0, 0.0).to_homogeneous();

        assert_eq!(t2, expected);
    }
```
```rust
    #[test]
    fn test_t2_with_rotation() {
        // 비행기 1: y축으로 90도 회전
        let rotation = nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::y_axis(), std::f64::consts::FRAC_PI_2);
        let t1 = rotation.to_homogeneous();

        // 비행기 1 기준에서 비행기 2: x축으로 10m 이동
        let t_rel = Translation3::new(10.0, 0.0, 0.0).to_homogeneous();

        // 비행기 2의 글로벌 좌표계 변환 계산
        let t2 = global_transform_t2(t1, t_rel);

        println!("비행기 2 글로벌 좌표계 변환 행렬:\n{}", t2);
    }
}

#[cfg(test)]
mod tests_case2 {
    use nalgebra::{Isometry3, Matrix4, Translation3, Unit, UnitQuaternion, Vector3};
    use nurbslib::core::frame_transform::FrameTransform;

    /// 부동소수 비교용 헬퍼
    fn approx_eq_matrix4(a: &Matrix4<f64>, b: &Matrix4<f64>, tol: f64) {
        for i in 0..4 {
            for j in 0..4 {
                let diff = (a[(i, j)] - b[(i, j)]).abs();
                assert!(
                    diff <= tol,
                    "mismatch at ({}, {}): {} vs {}, diff={} > tol={}",
                    i, j, a[(i, j)], b[(i, j)], diff, tol
                );
            }
        }
    }
```
```rust
    /// 1) 둘 다 단위행렬이면,
    ///   - 1 기준에서 본 2 = I
    ///   - 2 기준에서 본 1 = I
    #[test]
    fn relative_identity_frames() {
        let t_w1 = Matrix4::<f64>::identity();
        let t_w2 = Matrix4::<f64>::identity();

        let (t_1_2, t_2_1) = FrameTransform::mutual_relative(&t_w1, &t_w2);

        approx_eq_matrix4(&t_1_2, &Matrix4::identity(), 1e-12);
        approx_eq_matrix4(&t_2_1, &Matrix4::identity(), 1e-12);
    }
```
```rust
    /// 2) 1은 원점, 2는 x=10에 있을 때:
    ///   - 1 기준에서 본 2 의 translation 은 (10,0,0)
    ///   - 2 기준에서 본 1 의 translation 은 (-10,0,0)
    #[test]
    fn relative_simple_translation() {
        // 비행기 1: World 기준 단위
        let iso1 = Isometry3::translation(0.0, 0.0, 0.0);
        let t_w1 = iso1.to_homogeneous();

        // 비행기 2: World 기준 x=10
        let iso2 = Isometry3::translation(10.0, 0.0, 0.0);
        let t_w2 = iso2.to_homogeneous();

        let (t_1_2, t_2_1) = FrameTransform::mutual_relative(&t_w1, &t_w2);

        // t_1_2: 1 기준에서 본 2 → (10,0,0)
        let t_1_2 = Vector3::new(t_1_2[(0, 3)], t_1_2[(1, 3)], t_1_2[(2, 3)]);
        assert!((t_1_2.x - 10.0).abs() < 1e-12);
        assert!(t_1_2.y.abs() < 1e-12);
        assert!(t_1_2.z.abs() < 1e-12);

        // t_2_1: 2 기준에서 본 1 → (-10,0,0)
        let t_2_1 = Vector3::new(t_2_1[(0, 3)], t_2_1[(1, 3)], t_2_1[(2, 3)]);
        assert!((t_2_1.x + 10.0).abs() < 1e-12);
        assert!(t_2_1.y.abs() < 1e-12);
        assert!(t_2_1.z.abs() < 1e-12);
    }
```
```rust
    /// 3) t_w1 과 (1 기준에서 본 2 = t_1_2)를 가지고
    ///    다시 t_w2 를 재구성하면 원래와 같아야 한다:
    ///    t_w2 = t_w1 * t_1_2
    #[test]
    fn world_from_relative_recovers_global() {
        // 비행기 1: (1, 2, 3)에 위치 + z축으로 30도 회전 같은 복합 변환
        let axis = Vector3::z_axis();
        let angle = 30f64.to_radians();
        let rot1 = nalgebra::UnitQuaternion::from_axis_angle(&axis, angle);
        let trans1 = Vector3::new(1.0, 2.0, 3.0);
        let iso1 = Isometry3::from_parts(trans1.into(), rot1);
        let t_w1 = iso1.to_homogeneous();

        // 비행기 2: (5, -1, 0.5)에 위치 + x축으로 45도 회전
        let axis2 = Vector3::x_axis();
        let angle2 = 45f64.to_radians();
        let rot2 = nalgebra::UnitQuaternion::from_axis_angle(&axis2, angle2);
        let trans2 = Vector3::new(5.0, -1.0, 0.5);
        let iso2 = Isometry3::from_parts(trans2.into(), rot2);
        let t_w2 = iso2.to_homogeneous();

        // 1 기준에서 본 2
        let T_1_2 = FrameTransform::relative_b_in_a(&t_w1, &t_w2);

        // t_w1, T_1_2 로부터 다시 t_w2 를 복원
        let t_w2_recovered = FrameTransform::world_from_relative(&t_w1, &T_1_2);

        approx_eq_matrix4(&t_w2, &t_w2_recovered, 1e-10);
    }
```
```rust
    /// 4) 상대변환의 역행렬이 "반대 방향 상대변환" 이 되는지 확인
    ///
    ///   t_a_b 를 구한 후, invert_relative(t_a_b) = t_b_a 와 같은지 비교
    #[test]
    fn invert_relative_matches_swapped_relative() {
        // A, B 를 임의의 pose로 설정
        let axis_a = Unit::new_normalize(Vector3::new(0.3, 0.4, 0.5));
        let rot_a = UnitQuaternion::from_axis_angle(&axis_a, 0.7);
        let trans_a = Vector3::new(2.0, -3.0, 1.0);
        let iso_a = Isometry3::from_parts(Translation3::from(trans_a), rot_a);
        let t_wa = iso_a.to_homogeneous();

        let axis_b = Unit::new_normalize(Vector3::new(-0.2, 1.0, 0.1));
        let rot_b = UnitQuaternion::from_axis_angle(&axis_b, -0.4);
        let trans_b = Vector3::new(-1.0, 0.5, 4.0);
        let iso_b = Isometry3::from_parts(Translation3::from(trans_b), rot_b);
        let t_wb = iso_b.to_homogeneous();

        // A 기준에서 본 B
        let t_a_b = FrameTransform::relative_b_in_a(&t_wa, &t_wb);
        // B 기준에서 본 A
        let t_b_a_from_swap = FrameTransform::relative_b_in_a(&t_wb, &t_wa);
        // A 기준에서 본 B 의 역행렬
        let t_b_a_from_inv = FrameTransform::invert_relative(&t_a_b);

        approx_eq_matrix4(&t_b_a_from_swap, &t_b_a_from_inv, 1e-10);

        // 보너스: t_a_b * T_B_A ≈ I 여야 함
        let ident = t_a_b * t_b_a_from_inv;
        approx_eq_matrix4(&ident, &Matrix4::identity(), 1e-10);
    }
}
```
---





