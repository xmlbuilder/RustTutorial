# Joint Matrix System

다양한 관절 유형에 대한 변환 행렬을 계산하는 C++ 기반 라이브러리입니다.
다물체 동역학, 인체 모델링, 로봇 시뮬레이션 등에 활용 가능합니다.

## 지원 유형
- 지원 관절 유형
- 자유 관절 (Quaternion, Euler, Bryant)
- 회전 관절 (Revolute)
- 병진 관절 (Translational)
- 구형 관절 (Spherical)
- 유니버설 관절 (Universal)
- 복합 관절 (Cylinder, Planar, Revo+Trans 등)

## 소스
```rust
use num_traits::Float;
use num_traits::real::Real;
use crate::math::matrix::matrix4::identity_matrix4x4;
use crate::math::matrix::Matrix4x4;
use crate::math::prelude::Quaternion;

struct JointMatrix
{
}

impl JointMatrix{

    fn reorder_quad_to_open_nurbs(q: &[f64; 4]) -> [f64; 4] {
        // OpenNURBS: [w, x, y, z] → 내부 계산용 [w, x, y, z]
        [q[3], q[1], q[2], q[0]]
    }

    fn reorder_quad_to_madymo(q: &[f64; 4]) -> [f64; 4] {
        // Madymo: [x, y, z, w] → 내부 계산용 [w, x, y, z]
        [q[0], q[1], q[2], q[3]]
    }

    fn normalized_quaternion(vec: &[f64; 4])->Option<Quaternion>
    {
        let len = (vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2] + vec[3] * vec[3]);
        if len < 1.0E-6
        {
            let mut new_vec = [0.0; 4];
            new_vec[0] = 0.0;
            new_vec[1] = 0.0;
            new_vec[2] = 0.0;
            new_vec[3] = 1.0;
            Some(Quaternion::new(new_vec[0], new_vec[1], new_vec[2], new_vec[3]));
        }
        let identity = (2.0 / len).sqrt();
        let mut new_vec = [0.0; 4];
        new_vec[0] = identity * vec[0];
        new_vec[1] = identity * vec[1];
        new_vec[2] = identity * vec[2];
        new_vec[3] = identity * vec[3];
        Some(Quaternion::new(new_vec[0], new_vec[1], new_vec[2], new_vec[3]))
    }

    fn calc_joint_free(q : &[f64; 4], t: &[f64; 4]) -> Matrix4x4
    {
        let mut mat : Matrix4x4 = [[0.0; 4]; 4];
        let nq = match Self::normalized_quaternion(q) {
            Some(quat) => [quat.w, quat.x, quat.y, quat.z],
            None => [0.0, 0.0, 0.0, 1.0], // fallback
        };

        mat[0][0] = nq[0] * nq[0] + nq[1] * nq[1] - 1.0;
        mat[0][1] = nq[1] * nq[2] - nq[0] * nq[3];
        mat[0][2] = nq[1] * nq[3] + nq[0] * nq[2];
        mat[1][0] = nq[1] * nq[2] + nq[0] * nq[3];
        mat[1][1] = nq[0] * nq[0] + nq[2] * nq[2] - 1.0;
        mat[1][2] = nq[2] * nq[3] - nq[0] * nq[1];
        mat[2][0] = nq[1] * nq[3] - nq[0] * nq[2];
        mat[2][1] = nq[2] * nq[3] + nq[0] * nq[1];
        mat[2][2] = nq[0] * nq[0] + nq[3] * nq[3] - 1.0;
        mat[0][3] = t[0];
        mat[1][3] = t[1];
        mat[2][3] = t[2];
        mat
    }

    fn calc_joint_free_bryant(q : &[f64; 3], t: &[f64; 3]) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[1].cos() * q[2].cos();
        matrix[0][1] = -q[1].cos() * q[2].sin();
        matrix[0][2] = q[1].sin();
        matrix[1][0] = q[0].cos() * q[2].sin() + q[0].sin() * q[1].sin() * q[2].cos();
        matrix[1][1] = q[0].cos() * q[2].cos() - q[0].sin() * q[1].sin() * q[2].sin();
        matrix[1][2] = -q[0].sin() * q[1].cos();
        matrix[2][0] = q[0].sin() * q[2].sin() - q[0].cos() * q[1].sin() * q[2].cos();
        matrix[2][1] = q[0].sin() * q[2].cos() + q[0].cos() * q[1].sin() * q[2].sin();
        matrix[2][2] = q[0].cos() * q[1].cos();
        matrix[0][3] = t[0];
        matrix[1][3] = t[1];
        matrix[2][3] = t[2];
        matrix
    }

    fn calc_joint_free_euler(q : &[f64; 3], t: &[f64; 3]) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[1].cos();
        matrix[0][1] = q[1].sin() * q[2].sin();
        matrix[0][2] = q[1].sin() * q[2].cos();
        matrix[1][0] = q[0].sin() * q[1].sin();
        matrix[1][1] = q[0].cos() * q[2].cos() - q[0].sin() * q[1].cos() * q[2].sin();
        matrix[1][2] = -q[0].cos() * q[2].sin() - q[0].sin() * q[1].cos() * q[2].cos();
        matrix[2][0] = -q[0].cos() * q[1].sin();
        matrix[2][1] = q[0].sin() * q[2].cos() + q[0].cos() * q[1].cos() * q[2].sin();
        matrix[2][2] = -q[0].sin() * q[2].sin() +q[0].cos() * q[1].cos() * q[2].cos();
        matrix[0][3] = t[0];
        matrix[1][3] = t[1];
        matrix[2][3] = t[2];
        matrix
    }

    fn calc_joint_free_rot_disp(q : &[f64; 4], t: &[f64; 3]) -> Matrix4x4
    {
        let mut matrix = identity_matrix4x4();
        let nq = match Self::normalized_quaternion(q) {
            Some(quat) => [quat.w, quat.x, quat.y, quat.z],
            None => [0.0, 0.0, 0.0, 1.0], // fallback
        };
        matrix[0][0] = nq[0] * nq[0] + nq[1] * nq[1] - 1.0;
        matrix[0][1] = nq[1] * nq[2] - nq[0] * nq[3];
        matrix[0][2] = nq[1] * nq[3] + nq[0] * nq[2];
        matrix[1][0] = nq[1] * nq[2] + nq[0] * nq[3];
        matrix[1][1] = nq[0] * nq[0] + nq[2] * nq[2] - 1.0;
        matrix[1][2] = nq[2] * nq[3] - nq[0] * nq[1];
        matrix[2][0] = nq[1] * nq[3] - nq[0] * nq[2];
        matrix[2][1] = nq[2] * nq[3] + nq[0] * nq[1];
        matrix[2][2] = nq[0] * nq[0] + nq[3] * nq[3] - 1.0;

        matrix[0][3] = matrix[0][0] * t[0] + matrix[0][1] * t[1] + matrix[0][2] * t[2];
        matrix[1][3] = matrix[1][0] * t[0] + matrix[1][1] * t[1] + matrix[1][2] * t[2];
        matrix[2][3] = matrix[2][0] * t[0] + matrix[2][1] * t[1] + matrix[2][2] * t[2];
        matrix
    }

    fn calc_joint_revo(q : f64) ->Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[1][1] = q.cos();
        matrix[1][2] = -q.sin();
        matrix[2][1] = q.sin();
        matrix[2][2] = q.cos();
        matrix
    }

    fn calc_joint_sphere(q : &[f64; 4]) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[0] * q[0] + q[1] * q[1] - 1.0;
        matrix[0][1] = q[1] * q[2] - q[0] * q[3];
        matrix[0][2] = q[1] * q[3] + q[0] * q[2];
        matrix[1][0] = q[1] * q[2] + q[0] * q[3];
        matrix[1][1] = q[0] * q[0] + q[2] * q[2] - 1.0;
        matrix[1][2] = q[2] * q[3] - q[0] * q[1];
        matrix[2][0] = q[1] * q[3] - q[0] * q[2];
        matrix[2][1] = q[2] * q[3] + q[0] * q[1];
        matrix[2][2] = q[0] * q[0] + q[3] * q[3] - 1.0;
        matrix
    }

    fn calc_joint_sphere_euler(q : &[f64; 3]) ->Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[1].cos();
        matrix[0][1] = q[1].sin() * q[2].sin();
        matrix[0][2] = q[1].sin() * q[2].cos();
        matrix[1][0] = q[0].sin() * q[1].sin();
        matrix[1][1] = q[0].cos() * q[2].cos() - q[0].sin() * q[1].cos() * q[2].sin();
        matrix[1][2] = -q[0].cos() * q[2].sin() - q[0].sin() * q[1].cos() * q[2].cos();
        matrix[2][0] = -q[0].cos() * q[1].sin();
        matrix[2][1] =q[0].sin() * q[2].cos() + q[0].cos() * q[1].cos() *q[2].sin();
        matrix[2][2] = -q[0].sin() * q[2].sin() + q[0].cos() * q[1].cos() * q[2].cos();
        matrix
    }

    fn calc_joint_sphere_bryant(q : &[f64; 3]) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[1].cos() * q[2].cos();
        matrix[0][1] = -q[1].cos() * q[2].sin();
        matrix[0][2] = q[1].sin();
        matrix[1][0] = q[0].cos() * q[2].sin() + q[0].sin() * q[1].sin() * q[2].cos();
        matrix[1][1] = q[0].cos() * q[2].cos() - q[0].sin() * q[1].sin() * q[2].sin();
        matrix[1][2] = -q[0].sin() * q[1].cos();
        matrix[2][0] = q[0].sin() * q[2].sin() - q[0].cos() * q[1].sin() * q[2].cos();
        matrix[2][1] = q[0].sin() * q[2].cos() + q[0].cos() * q[1].sin() * q[2].sin();
        matrix[2][2] = q[0].cos() * q[1].cos();
        matrix
    }

    fn calc_joint_universal(q : &[f64; 2]) ->Matrix4x4
    {

        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[1].cos();
        matrix[0][1] = 0.0;
        matrix[0][2] = q[1].sin();
        matrix[1][0] = q[0].sin() * q[1].sin();
        matrix[1][1] = q[0].cos();
        matrix[1][2] = -q[0].sin() * q[1].cos();
        matrix[2][0] = -q[0].cos() * q[1].sin();
        matrix[2][1] = q[0].sin();
        matrix[2][2] = q[0].cos() * q[1].cos();
        matrix
    }


    fn calc_joint_trans(t : f64) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][3] = t;
        matrix
    }

    fn calc_joint_cylinder(q : f64, t : f64) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[1][1] = q.cos();
        matrix[1][2] = -q.sin();
        matrix[2][1] = q.sin();
        matrix[2][2] = q.cos();
        matrix[0][3] = t;
        matrix
    }

    fn calc_joint_planar(q : f64, t : [f64; 2]) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[1][1] = q.cos();
        matrix[1][2] = -q.sin();
        matrix[2][1] = q.sin();
        matrix[2][2] = q.cos();
        matrix[1][3] = t[0];
        matrix[2][3] = t[1];
        matrix
    }

    fn calc_joint_trans_universal(q : [f64; 2], t : f64) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[0].cos() * q[1].cos();
        matrix[0][1] = -q[0].cos() * q[1].sin();
        matrix[0][2] = q[0].sin();
        matrix[1][0] = q[1].sin();
        matrix[1][1] = q[1].cos();
        matrix[1][2] = 0.0;
        matrix[2][0] = -q[0].sin() * q[1].cos();
        matrix[2][1] = q[0].sin() * q[1].sin();
        matrix[2][2] = q[0].cos();
        matrix[0][3] = t;
        matrix
    }

    fn calc_joint_universal_trans(q : [f64; 2], t : f64) -> Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q[0].cos() * q[1].cos();
        matrix[0][1] = -q[1].sin();
        matrix[0][2] = q[0].sin() * q[1].cos();
        matrix[1][0] = q[0].cos() * q[1].sin();
        matrix[1][1] = q[1].cos();
        matrix[1][2] = q[0].sin() * q[1].sin();
        matrix[2][0] = -q[0].sin();
        matrix[2][1] = 0.0;
        matrix[2][2] = q[0].cos();
        matrix[0][3] = t * q[0].cos() * q[1].cos();
        matrix[1][3] = t * q[0].cos() * q[1].sin();
        matrix[2][3] = -t * q[0].sin();
        matrix
    }

    fn calc_joint_trans_revo(q : f64, t : f64) ->Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q.cos();
        matrix[0][2] = q.sin();
        matrix[2][0] = -q.sin();
        matrix[2][2] = q.cos();
        matrix[0][3] = t;
        matrix
    }

    fn calc_joint_revo_trans(q : f64, t : f64) ->Matrix4x4
    {
        let mut matrix  = identity_matrix4x4();
        matrix[0][0] = q.cos();
        matrix[0][2] = q.sin();
        matrix[2][0] = -q.sin();
        matrix[2][2] = q.cos();
        matrix[0][3] = t * q.cos();
        matrix[2][3] = -t * q.sin();
        matrix
    }
}
```

## 테스트 코드
```rust
#[test]
fn test_normalized_quaternion(){
    let some = JointMatrix::normalized_quaternion(&[1.0, 0.0, 0.0, 0.0]);
    println!("{:?}", some.is_some());
}

```
### 🧪 샘플 1: 자유 관절 (Quaternion + Translation)
```rust
#[test]
fn test_calc_joint_free() {
    let q = [1.0, 0.0, 0.0, 0.707]; // 90도 회전 (Y축 기준)
    let t = [1.0, 2.0, 3.0, 0.0];     // 위치 이동

    let matrix = JointMatrix::calc_joint_free(&q, &t);
    println!("Joint Free Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}

#[test]
fn test_calc_joint_free_bryant() {
    let q = [0.0_f64.to_radians(), 90.0_f64.to_radians(), 0.0_f64.to_radians()];
    let t = [0.0, 0.0, 0.0];

    let matrix = JointMatrix::calc_joint_free_bryant(&q, &t);
    println!("Bryant Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}

#[test]
fn test_calc_joint_free_rot_disp() {
    let q = [0.0, 0.0, 0.0, 1.0]; // 단위 Quaternion
    let t = [1.0, 0.0, 0.0];      // X축 이동

    let matrix = JointMatrix::calc_joint_free_rot_disp(&q, &t);
    println!("Rotational Displacement Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}

#[test]
fn test_calc_joint_revo() {
    let angle = std::f64::consts::FRAC_PI_2; // 90도
    let matrix = JointMatrix::calc_joint_revo(angle);
    println!("Revolute Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
```



### 🧪 샘플 2: 브라이언트 각도 기반 관절
```
#[test]
fn test_calc_joint_free_bryant() {
    let q = [0.0_f64.to_radians(), 90.0_f64.to_radians(), 0.0_f64.to_radians()];
    let t = [0.0, 0.0, 0.0];

    let matrix = JointMatrix::calc_joint_free_bryant(&q, &t);
    println!("Bryant Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
```


### 🧪 샘플 3: 회전 + 변위 관절
```
#[test]
fn test_calc_joint_free_rot_disp() {
    let q = [1.0, 0.0, 0.0, 0.0]; // 단위 Quaternion
    let t = [1.0, 0.0, 0.0];      // X축 이동

    let matrix = JointMatrix::calc_joint_free_rot_disp(&q, &t);
    println!("Rotational Displacement Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
```


### 🧪 샘플 4: 단순 Revolute 관절
```
#[test]
fn test_calc_joint_revo() {
    let angle = std::f64::consts::FRAC_PI_2; // 90도
    let matrix = JointMatrix::calc_joint_revo(angle);
    println!("Revolute Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
```



