# Joint Matrix System

다양한 관절 유형에 대한 변환 행렬을 계산하는 C++ 기반 라이브러리입니다.  
다물체 동역학, 인체 모델링, 로봇 시뮬레이션 등에 활용 가능합니다.

## 📘 Joint Matrix System 개요
- 목적: 다양한 관절 유형에 대해 4×4 변환 행렬(Xform)을 계산하는 Rust 기반 유틸리티
- 활용 분야: 다물체 동역학, 인체 모델링, 로봇 시뮬레이션 등
- 핵심 구조: JointMatrix 구조체와 다양한 calc_joint_* 함수들


## 지원 유형
- 지원 관절 유형
- 자유 관절 (Quaternion, Euler, Bryant)
- 회전 관절 (Revolute)
- 병진 관절 (Translational)
- 구형 관절 (Spherical)
- 유니버설 관절 (Universal)
- 복합 관절 (Cylinder, Planar, Revo+Trans 등)

## 🧮 핵심 수식 예시
### ✅ Quaternion 기반 회전 행렬

$$
R=\left[ \begin{matrix}q_0^2+q_1^2-1&q_1q_2-q_0q_3&q_1q_3+q_0q_2\\ ; \quad q_1q_2+q_0q_3&q_0^2+q_2^2-1&q_2q_3-q_0q_1\\ ; \quad q_1q_3-q_0q_2&q_2q_3+q_0q_1&q_0^2+q_3^2-1\end{matrix}\right]
$$


### ✅ 병진 포함 시

$$
T=\left[ \begin{matrix}R&\vec {t}\\ 0&1\end{matrix}\right] \quad \mathrm{where\  }\vec {t}=[tx,ty,tz]^T
$$

## 소스
```rust
use crate::core::xform::Xform;

/// 여러 형태의 관절(조인트) 파라미터를 4×4 변환 행렬(Xform)로 만들어 주는 유틸리티입니다.
/// - 쿼터니언(오일러 파라미터)
/// - Euler / Bryant 각
/// - 단순 회전/병진 조합
pub struct JointMatrix;
```
```rust
impl JointMatrix {
    /// C++ 코드와 동일한 방식으로 쿼터니언을 정규화합니다.
    ///
    /// 입력: q = [q0, q1, q2, q3], q0 이 스칼라 항
    /// 반환: true  -> 정상 정규화
    ///       false -> 길이가 너무 작아서 단위 쿼터니언으로 리셋
    pub fn normalize_quaternion(q: &mut [f64; 4]) -> bool {
        let len = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3];

        if len < 1.0e-6 {
            // 매우 작은 경우: 단위 쿼터니언으로 초기화
            q[0] = 1.0;
            q[1] = 0.0;
            q[2] = 0.0;
            q[3] = 0.0;
            return false;
        }

        // 원본 코드와 동일하게 |q|^2 = 2 가 되도록 스케일
        let s = (2.0 / len).sqrt();
        q[0] *= s;
        q[1] *= s;
        q[2] *= s;
        q[3] *= s;
        true
    }
```
```rust
    /// 자유 관절 (쿼터니언 회전 + 직교 좌표계에서의 병진)
    ///
    /// q = [q0,q1,q2,q3] (오일러 파라미터, 자동 정규화하지 않음)
    /// t = [tx,ty,tz] (x,y,z 방향 병진)
    pub fn calc_joint_free(q: [f64; 4], t: [f64; 3]) -> Xform {
        let mut m = Xform::identity();

        m.m[0][0] = q[0] * q[0] + q[1] * q[1] - 1.0;
        m.m[0][1] = q[1] * q[2] - q[0] * q[3];
        m.m[0][2] = q[1] * q[3] + q[0] * q[2];

        m.m[1][0] = q[1] * q[2] + q[0] * q[3];
        m.m[1][1] = q[0] * q[0] + q[2] * q[2] - 1.0;
        m.m[1][2] = q[2] * q[3] - q[0] * q[1];

        m.m[2][0] = q[1] * q[3] - q[0] * q[2];
        m.m[2][1] = q[2] * q[3] + q[0] * q[1];
        m.m[2][2] = q[0] * q[0] + q[3] * q[3] - 1.0;

        m.m[0][3] = t[0];
        m.m[1][3] = t[1];
        m.m[2][3] = t[2];

        m
    }
```
```rust
    /// 자유 관절 (Bryant 각, ZYX 순서) + 병진
    ///
    /// q = [rx, ry, rz] : x, y, z 축 회전각
    pub fn calc_joint_free_bryant(q: [f64; 3], t: [f64; 3]) -> Xform {
        let (qx, qy, qz) = (q[0], q[1], q[2]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos() * qz.cos();
        m.m[0][1] = -qy.cos() * qz.sin();
        m.m[0][2] = qy.sin();

        m.m[1][0] = qx.cos() * qz.sin() + qx.sin() * qy.sin() * qz.cos();
        m.m[1][1] = qx.cos() * qz.cos() - qx.sin() * qy.sin() * qz.sin();
        m.m[1][2] = -qx.sin() * qy.cos();

        m.m[2][0] = qx.sin() * qz.sin() - qx.cos() * qy.sin() * qz.cos();
        m.m[2][1] = qx.sin() * qz.cos() + qx.cos() * qy.sin() * qz.sin();
        m.m[2][2] = qx.cos() * qy.cos();

        m.m[0][3] = t[0];
        m.m[1][3] = t[1];
        m.m[2][3] = t[2];

        m
    }
```
```rust
    /// 자유 관절 (Euler 각) + 병진
    ///
    /// q = [rx, ry, rz] : x, y, z 축 회전각
    pub fn calc_joint_free_euler(q: [f64; 3], t: [f64; 3]) -> Xform {
        let (qx, qy, qz) = (q[0], q[1], q[2]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos();
        m.m[0][1] = qy.sin() * qz.sin();
        m.m[0][2] = qy.sin() * qz.cos();

        m.m[1][0] = qx.sin() * qy.sin();
        m.m[1][1] = qx.cos() * qz.cos() - qx.sin() * qy.cos() * qz.sin();
        m.m[1][2] = -qx.cos() * qz.sin() - qx.sin() * qy.cos() * qz.cos();

        m.m[2][0] = -qx.cos() * qy.sin();
        m.m[2][1] = qx.sin() * qz.cos() + qx.cos() * qy.cos() * qz.sin();
        m.m[2][2] = -qx.sin() * qz.sin() + qx.cos() * qy.cos() * qz.cos();

        m.m[0][3] = t[0];
        m.m[1][3] = t[1];
        m.m[2][3] = t[2];

        m
    }
```
```rust
    /// 자유 관절 (쿼터니언 회전 + 회전 좌표계에서의 변위)
    ///
    /// q : in-out, C++ 과 마찬가지로 이 함수 안에서 정규화됩니다.
    /// t : 기준 좌표계에서의 변위 (회전 후 R * t 로 환산되어 저장)
    pub fn calc_joint_free_rot_disp(q: &mut [f64; 4], t: [f64; 3]) -> Xform {
        let _ = Self::normalize_quaternion(q);
        let mut m = Xform::identity();

        m.m[0][0] = q[0] * q[0] + q[1] * q[1] - 1.0;
        m.m[0][1] = q[1] * q[2] - q[0] * q[3];
        m.m[0][2] = q[1] * q[3] + q[0] * q[2];

        m.m[1][0] = q[1] * q[2] + q[0] * q[3];
        m.m[1][1] = q[0] * q[0] + q[2] * q[2] - 1.0;
        m.m[1][2] = q[2] * q[3] - q[0] * q[1];

        m.m[2][0] = q[1] * q[3] - q[0] * q[2];
        m.m[2][1] = q[2] * q[3] + q[0] * q[1];
        m.m[2][2] = q[0] * q[0] + q[3] * q[3] - 1.0;

        // T' = R * T
        m.m[0][3] = m.m[0][0] * t[0] + m.m[0][1] * t[1] + m.m[0][2] * t[2];
        m.m[1][3] = m.m[1][0] * t[0] + m.m[1][1] * t[1] + m.m[1][2] * t[2];
        m.m[2][3] = m.m[2][0] * t[0] + m.m[2][1] * t[1] + m.m[2][2] * t[2];

        m
    }
```
```rust
    /// 회전 관절 (x축 회전)
    pub fn calc_joint_revo(q: f64) -> Xform {
        let mut m = Xform::identity();

        m.m[1][1] = q.cos();
        m.m[1][2] = -q.sin();
        m.m[2][1] = q.sin();
        m.m[2][2] = q.cos();

        m
    }
```
```rust
    /// 구형 관절 (쿼터니언으로부터 회전만)
    pub fn calc_joint_sphere(q: [f64; 4]) -> Xform {
        let mut m = Xform::identity();

        m.m[0][0] = q[0] * q[0] + q[1] * q[1] - 1.0;
        m.m[0][1] = q[1] * q[2] - q[0] * q[3];
        m.m[0][2] = q[1] * q[3] + q[0] * q[2];

        m.m[1][0] = q[1] * q[2] + q[0] * q[3];
        m.m[1][1] = q[0] * q[0] + q[2] * q[2] - 1.0;
        m.m[1][2] = q[2] * q[3] - q[0] * q[1];

        m.m[2][0] = q[1] * q[3] - q[0] * q[2];
        m.m[2][1] = q[2] * q[3] + q[0] * q[1];
        m.m[2][2] = q[0] * q[0] + q[3] * q[3] - 1.0;

        m
    }
```
```rust
    /// 구형 관절 (Euler 각)
    pub fn calc_joint_sphere_euler(q: [f64; 3]) -> Xform {
        let (qx, qy, qz) = (q[0], q[1], q[2]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos();
        m.m[0][1] = qy.sin() * qz.sin();
        m.m[0][2] = qy.sin() * qz.cos();

        m.m[1][0] = qx.sin() * qy.sin();
        m.m[1][1] = qx.cos() * qz.cos() - qx.sin() * qy.cos() * qz.sin();
        m.m[1][2] = -qx.cos() * qz.sin() - qx.sin() * qy.cos() * qz.cos();

        m.m[2][0] = -qx.cos() * qy.sin();
        m.m[2][1] = qx.sin() * qz.cos() + qx.cos() * qy.cos() * qz.sin();
        m.m[2][2] = -qx.sin() * qz.sin() + qx.cos() * qy.cos() * qz.cos();

        m
    }
```
```rust
    /// 구형 관절 (Bryant, ZYX)
    pub fn calc_joint_sphere_bryant(q: [f64; 3]) -> Xform {
        let (qx, qy, qz) = (q[0], q[1], q[2]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos() * qz.cos();
        m.m[0][1] = -qy.cos() * qz.sin();
        m.m[0][2] = qy.sin();

        m.m[1][0] = qx.cos() * qz.sin() + qx.sin() * qy.sin() * qz.cos();
        m.m[1][1] = qx.cos() * qz.cos() - qx.sin() * qy.sin() * qz.sin();
        m.m[1][2] = -qx.sin() * qy.cos();

        m.m[2][0] = qx.sin() * qz.sin() - qx.cos() * qy.sin() * qz.cos();
        m.m[2][1] = qx.sin() * qz.cos() + qx.cos() * qy.sin() * qz.sin();
        m.m[2][2] = qx.cos() * qy.cos();

        m
    }
```
```rust
    /// 유니버설 관절:
    /// x축 회전 후, 새 y축 기준 회전
    pub fn calc_joint_universal(q: [f64; 2]) -> Xform {
        let (qx, qy) = (q[0], q[1]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos();
        m.m[0][1] = 0.0;
        m.m[0][2] = qy.sin();

        m.m[1][0] = qx.sin() * qy.sin();
        m.m[1][1] = qx.cos();
        m.m[1][2] = -qx.sin() * qy.cos();

        m.m[2][0] = -qx.cos() * qy.sin();
        m.m[2][1] = qx.sin();
        m.m[2][2] = qx.cos() * qy.cos();

        m
    }
```
```rust
    /// 순수 병진 (x축 방향)
    pub fn calc_joint_trans(q: f64) -> Xform {
        let mut m = Xform::identity();
        m.m[0][3] = q;
        m
    }
```
```rust
    /// 원통 관절: x축 회전 + x축 병진
    pub fn calc_joint_cylinder(q: f64, t: f64) -> Xform {
        let mut m = Xform::identity();

        m.m[1][1] = q.cos();
        m.m[1][2] = -q.sin();
        m.m[2][1] = q.sin();
        m.m[2][2] = q.cos();

        m.m[0][3] = t;

        m
    }
```
```rust
    /// 평면 관절: x축 회전 + y,z 방향 병진
    pub fn calc_joint_planar(q: f64, t: [f64; 2]) -> Xform {
        let mut m = Xform::identity();

        m.m[1][1] = q.cos();
        m.m[1][2] = -q.sin();
        m.m[2][1] = q.sin();
        m.m[2][2] = q.cos();

        m.m[1][3] = t[0];
        m.m[2][3] = t[1];

        m
    }
```
```rust
    /// 병진 후 유니버설 회전
    /// q = [qy, qz] : y, z 축 회전각
    /// t : x 방향 병진
    pub fn calc_joint_trans_universal(q: [f64; 2], t: f64) -> Xform {
        let (qy, qz) = (q[0], q[1]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos() * qz.cos();
        m.m[0][1] = -qy.cos() * qz.sin();
        m.m[0][2] = qy.sin();

        m.m[1][0] = qz.sin();
        m.m[1][1] = qz.cos();
        m.m[1][2] = 0.0;

        m.m[2][0] = -qy.sin() * qz.cos();
        m.m[2][1] = qy.sin() * qz.sin();
        m.m[2][2] = qy.cos();

        m.m[0][3] = t;

        m
    }
```
```rust
    /// 유니버설 회전 후 x축 병진
    pub fn calc_joint_universal_trans(q: [f64; 2], t: f64) -> Xform {
        let (qy, qz) = (q[0], q[1]);
        let mut m = Xform::identity();

        m.m[0][0] = qy.cos() * qz.cos();
        m.m[0][1] = -qz.sin();
        m.m[0][2] = qy.sin() * qz.cos();

        m.m[1][0] = qy.cos() * qz.sin();
        m.m[1][1] = qz.cos();
        m.m[1][2] = qy.sin() * qz.sin();

        m.m[2][0] = -qy.sin();
        m.m[2][1] = 0.0;
        m.m[2][2] = qy.cos();

        m.m[0][3] = t * qy.cos() * qz.cos();
        m.m[1][3] = t * qy.cos() * qz.sin();
        m.m[2][3] = -t * qy.sin();

        m
    }
```
```rust
    /// 병진 + 회전 (y축 회전 + x축 병진)
    pub fn calc_joint_trans_revo(q: f64, t: f64) -> Xform {
        let mut m = Xform::identity();

        m.m[0][0] = q.cos();
        m.m[0][2] = q.sin();
        m.m[2][0] = -q.sin();
        m.m[2][2] = q.cos();

        m.m[0][3] = t;

        m
    }
```
```rust
    /// 회전 + 병진 (y축 회전 + x축 병진, 회전 후 좌표계 기준)
    pub fn calc_joint_revo_trans(q: f64, t: f64) -> Xform {
        let mut m = Xform::identity();

        m.m[0][0] = q.cos();
        m.m[0][2] = q.sin();
        m.m[2][0] = -q.sin();
        m.m[2][2] = q.cos();

        m.m[0][3] = t * q.cos();
        m.m[2][3] = -t * q.sin();

        m
    }
}
```
---

## 테스트 코드

### 🧪 주요 테스트 목록

| 테스트 함수명                           | 검증 내용                                  |
|----------------------------------------|--------------------------------------------|
| `test_normalize_quaternion_small`      | 길이 0 쿼터니언 → 단위 쿼터니언으로 초기화 |
| `test_normalize_quaternion_length2`    | 길이 2 → |q|^2 = 2로 정규화됨 확인           |
| `test_calc_joint_free_identity_and_translation` | 단위 회전 + 병진 확인                     |
| `test_calc_joint_free_bryant_zero`     | Bryant 각 0 → 단위 행렬 확인               |
| `test_calc_joint_free_euler_zero`      | Euler 각 0 → 단위 행렬 확인                |
| `test_calc_joint_free_rot_disp_identity` | 회전 후 병진 적용 확인                    |
| `test_calc_joint_revo_pi_half`         | x축 회전 90도 → 행렬 확인                  |
| `test_calc_joint_sphere_identity`      | 단위 쿼터니언 → 단위 회전 확인             |
| `test_calc_joint_planar_zero`          | 평면 관절 병진 확인                        |
| `test_calc_joint_universal_trans_zero` | 유니버설 회전 후 병진 확인                |

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::joint_matrix::JointMatrix;
    use nurbslib::core::xform::Xform;

    fn assert_xform_eq(m: &Xform, e: [[f64; 4]; 4]) {
        for i in 0..4 {
            for j in 0..4 {
                let a = m.m[i][j];
                let b = e[i][j];
                assert!(
                    (a - b).abs() < 1.0e-12,
                    "m[{}][{}] = {}, expected {}",
                    i,
                    j,
                    a,
                    b
                );
            }
        }
    }
```
```rust
    #[test]
    fn test_normalize_quaternion_small() {
        let mut q = [0.0, 0.0, 0.0, 0.0];
        let ok = JointMatrix::normalize_quaternion(&mut q);
        assert!(!ok);
        assert!((q[0] - 1.0).abs() < 1.0e-12);
        assert!(q[1].abs() < 1.0e-12);
        assert!(q[2].abs() < 1.0e-12);
        assert!(q[3].abs() < 1.0e-12);
    }
```
```rust
    #[test]
    fn test_normalize_quaternion_length2() {
        let mut q = [1.0, 1.0, 1.0, 1.0];
        let ok = JointMatrix::normalize_quaternion(&mut q);
        assert!(ok);

        let len2 = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3];
        assert!((len2 - 2.0).abs() < 1.0e-12);
    }
```
```rust
    #[test]
    fn test_calc_joint_free_identity_and_translation() {
        let q = [2.0_f64.sqrt(), 0.0, 0.0, 0.0]; // |q|^2 = 2 -> 단위 회전
        let t = [1.0, 2.0, 3.0];
        let m = JointMatrix::calc_joint_free(q, t);

        // 회전은 단위, 평행이동은 그대로
        let mut expected = Xform::identity();
        expected.m[0][3] = 1.0;
        expected.m[1][3] = 2.0;
        expected.m[2][3] = 3.0;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_free_bryant_zero() {
        let q = [0.0, 0.0, 0.0];
        let t = [0.0, 0.0, 0.0];
        let m = JointMatrix::calc_joint_free_bryant(q, t);

        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_free_euler_zero() {
        let q = [0.0, 0.0, 0.0];
        let t = [0.0, 0.0, 0.0];
        let m = JointMatrix::calc_joint_free_euler(q, t);

        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_free_rot_disp_identity() {
        // q는 함수 내부에서 정규화됨
        let mut q = [1.0, 0.0, 0.0, 0.0]; // normalize -> [sqrt(2), 0, 0, 0]
        let t = [1.0, 2.0, 3.0];
        let m = JointMatrix::calc_joint_free_rot_disp(&mut q, t);

        // 단위 회전이므로, R * T = T
        let mut expected = Xform::identity();
        expected.m[0][3] = 1.0;
        expected.m[1][3] = 2.0;
        expected.m[2][3] = 3.0;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_revo_zero() {
        let m = JointMatrix::calc_joint_revo(0.0);
        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_revo_pi_half() {
        let q = std::f64::consts::FRAC_PI_2;
        let m = JointMatrix::calc_joint_revo(q);

        // x축 회전 90도
        let mut expected = Xform::identity();
        expected.m[1][1] = 0.0;
        expected.m[1][2] = -1.0;
        expected.m[2][1] = 1.0;
        expected.m[2][2] = 0.0;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_sphere_identity() {
        let q = [2.0_f64.sqrt(), 0.0, 0.0, 0.0];
        let m = JointMatrix::calc_joint_sphere(q);
        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_sphere_euler_zero() {
        let q = [0.0, 0.0, 0.0];
        let m = JointMatrix::calc_joint_sphere_euler(q);
        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_sphere_bryant_zero() {
        let q = [0.0, 0.0, 0.0];
        let m = JointMatrix::calc_joint_sphere_bryant(q);
        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_universal_zero() {
        let q = [0.0, 0.0];
        let m = JointMatrix::calc_joint_universal(q);
        let expected = Xform::identity();
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_trans() {
        let m = JointMatrix::calc_joint_trans(5.0);
        let mut expected = Xform::identity();
        expected.m[0][3] = 5.0;
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_cylinder_zero() {
        let m = JointMatrix::calc_joint_cylinder(0.0, 2.0);
        let mut expected = Xform::identity();
        expected.m[0][3] = 2.0;
        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_planar_zero() {
        let q = 0.0;
        let t = [1.0, 2.0];
        let m = JointMatrix::calc_joint_planar(q, t);

        let mut expected = Xform::identity();
        expected.m[1][3] = 1.0;
        expected.m[2][3] = 2.0;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_trans_universal_zero() {
        let q = [0.0, 0.0];
        let t = 3.0;
        let m = JointMatrix::calc_joint_trans_universal(q, t);

        let mut expected = Xform::identity();
        expected.m[0][3] = 3.0;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_universal_trans_zero() {
        let q = [0.0, 0.0];
        let t = 4.0;
        let m = JointMatrix::calc_joint_universal_trans(q, t);

        let mut expected = Xform::identity();
        expected.m[0][3] = 4.0;
        expected.m[1][3] = 0.0;
        expected.m[2][3] = 0.0;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_trans_revo_zero() {
        let q = 0.0;
        let t = 2.5;
        let m = JointMatrix::calc_joint_trans_revo(q, t);

        let mut expected = Xform::identity();
        expected.m[0][3] = 2.5;

        assert_xform_eq(&m, expected.m);
    }
```
```rust
    #[test]
    fn test_calc_joint_revo_trans_zero() {
        let q = 0.0;
        let t = 1.5;
        let m = JointMatrix::calc_joint_revo_trans(q, t);

        let mut expected = Xform::identity();
        expected.m[0][3] = 1.5;
        expected.m[2][3] = 0.0;

        assert_xform_eq(&m, expected.m);
    }
}
```
---







