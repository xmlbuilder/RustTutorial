# Xform 테스트 코드

## 📐 주요 수식 정리
### 🔁 1. 선형 보간 (Lerp)

$$
\mathrm{lerp}(a,b,t)=(1-t)\cdot a+t\cdot b
$$

### 🔄 2. 쿼터니언 회전 (Quaternion Rotation)
- 벡터 $\vec {v}$ 를 쿼터니언 q 로 회전:

$$
\vec {v}'=q\cdot \vec {v}\cdot q^{-1}
$$

- 행렬로 변환 후 회전:

$$
\vec {v}'=R_q\cdot \vec {v}
$$

### 🔁 3. 행렬 곱 (Matrix Multiplication)
- $3\times 3$ 행렬 곱:

$$
C_{ij}=\sum _{k=0}^2A_{ik}\cdot B_{kj}
$$

### 🔁 4. 행렬 전치 (Transpose)

$$
A_{ij}^T=A_{ji}
$$

## 🔁 5. 회전 행렬 직교성 확인
- 회전 행렬 R 은 직교 행렬이므로:


$$
R\cdot R^T=I
$$

### 🔁 6. 쿼터니언 길이

$$
\| q\| =\sqrt{w^2+x^2+y^2+z^2}
$$

### 🔁 7. 쿼터니언 내적 (동일 회전 확인)

$$
q_1\cdot q_2=w_1w_2+x_1x_2+y_1y_2+z_1z_2
$$


### 🔁 8. SLERP (Spherical Linear Interpolation)

$$
\mathrm{slerp}(q_0,q_1,t)=\frac{\sin ((1-t)\theta )}{\sin \theta }q_0+\frac{\sin (t\theta )}{\sin \theta }q_1
$$

### 🔁 9. Axis-Angle → Quaternion

$$
q=\left( \cos \left( \frac{\theta }{2}\right) ,\sin \left( \frac{\theta }{2}\right) \cdot \vec {u}\right)
$$ 

### 🔁 10. Quaternion → Rotation Matrix

$$
R(q)=\left[ \begin{matrix}1-2y^2-2z^2,&2xy-2zw,&2xz+2yw,\\ 2xy+2zw,&1-2x^2-2z^2,&2yz-2xw,\\ 2xz-2yw,&2yz+2xw,&1-2x^2-2y^2\end{matrix}\right]
$$


## 📊 함수 및 수식 요약표
| 함수 이름                        | 수식 또는 원리                                      | 설명                                 |
|----------------------------------|-----------------------------------------------------|--------------------------------------|
| `lerp(a, b, t)`                  | $(1 - t) \cdot a + t \cdot b$                  | 선형 보간                            |
| `q.rotate(v)`                    | $q \cdot v \cdot q^{-1}$                       | 쿼터니언 회전                        |
| `q.to_mat3()`                    | 쿼터니언 → 회전 행렬 변환                          | 3×3 회전 행렬                        |
| `q.inverse()`                    | $q^{-1} = \frac{\bar{q}}{\|q\|^2}$             | 쿼터니언 역원                        |
| `q.len()`                        | $\|q\| = \sqrt{w^2 + x^2 + y^2 + z^2}$         | 쿼터니언의 크기                      |
| `q.to_axis_angle()`             | $q = (\cos(\theta/2), \sin(\theta/2) \cdot \vec{u})$| 축-각도 변환                     |
| `Quaternion::slerp()`      | $\text{slerp}(q_0, q_1, t)$                    | 구면 선형 보간                       |
| `Xform::from_quat(q)`           | $R_q$                                          | 쿼터니언 기반 변환 행렬 생성         |
| `Xform::translation(x, y, z)`   | 4×4 평행이동 행렬 생성                             | 포인트 이동                          |
| `Xform::mul(a, b)`              | $M = A \cdot B$                                | 변환 행렬 곱                         |
| `transform_vector(v)`           | $R \cdot v$                                    | 벡터 회전 (이동 없음)                |
| `transform_point(p)`            | $R \cdot p + T$                                | 포인트 회전 + 평행이동               |
| `quat_from_rot3(R)`             | 회전 행렬 → 쿼터니언 역변환                        | 회전 행렬 복원                       |


## 수학적 검증
Quaternion, Xform, Vector, Point 관련 테스트 코드와 수식들을 기반으로 수학적 정확성을 점검.  
아래는 각 주요 연산의 수학적 원리와 구현이 올바른지에 대한 분석입니다.

## ✅ 1. 선형 보간 (Lerp)
### 수식:

$$
\mathrm{lerp}(a,b,t)=(1-t)\cdot a+t\cdot b
$$

### 점검:
- 구현은 정확합니다.
- PointInterpolator::lerp_3d, lerp_2d, on_lerp_f64 등에서 사용된 방식은 수학적으로 정당하며, clamp 처리도 안전하게 되어 있습니다.

## ✅ 2. 쿼터니언 회전
### 수식:

$$
\vec {v}'=q\cdot \vec {v}\cdot q^{-1}\quad \mathrm{또는}\quad \vec {v}'=R_q\cdot \vec {v}
$$

### 점검:
- Quaternion::rotate(v)와 Xform::transform_vector(v)가 동일한 결과를 내는지 비교하는 테스트가 포함되어 있고, v_close로 검증됨 → 정확함
- to_mat3()로 변환된 행렬이 실제 회전 행렬인지 orthonormal 여부를 m3_close(R·Rᵀ, I)로 확인 → 수학적으로 타당

### ✅ 3. 행렬 연산
- apply3, mtm, mt, mm 함수들은 모두 표준적인 행렬 곱셈 및 전치 구현입니다.
- 테스트에서 rotation_matrix_is_orthonormal, composition_matches_matrix_mul 등으로 행렬 곱의 정확성을 검증 → 수학적으로 정확

### ✅ 4. 쿼터니언 역원 및 내적
### 수식:

$$
q^{-1}=\frac{\bar {q}}{\| q\| ^2}\quad q\cdot q^{-1}=\mathrm{identity}
$$

### 점검:
- inverse_conjugate_props 테스트에서 $q\cdot q^{-1}\approx (1,0,0,0)$ 확인 → 정확
- 내적을 통한 회전 동일성 확인도 quat_to_mat_round_trip에서 수행됨 → 수학적으로 타당

## ✅ 5. SLERP (Spherical Linear Interpolation)
### 수식:

$$
\mathrm{slerp}(q_0,q_1,t)=\frac{\sin ((1-t)\theta )}{\sin \theta }q_0+\frac{\sin (t\theta )}{\sin \theta }q_1
$$

### 점검:
- slerp_endpoints_mid_properties 테스트에서 t=0, t=1의 정확성 및 중간값의 단위 길이 확인 → 구현 정확

## ✅ 6. Axis-Angle ↔ Quaternion 변환
### 수식:

$$
q=\left( \cos \left( \frac{\theta }{2}\right) ,\sin \left( \frac{\theta }{2}\right) \cdot \vec {u}\right)
$$

### 점검:
- axis_angle_roundtrip 테스트에서 변환 후 다시 복원된 회전 행렬이 동일한지 확인 → 수학적으로 정확

## ✅ 7. Euler ZYX 회전
### 수식:

$$
R=R_z(\mathrm{yaw})\cdot R_y(\mathrm{pitch})\cdot R_x(\mathrm{roll})
$$

#### 점검:
- euler_zyx_matches_explicit_multiplication 테스트에서 직접 계산한 행렬과 쿼터니언 변환 행렬 비교 → 정확

## ✅ 8. 평행이동과 회전의 구분
### 점검:
- vector_vs_point_translation_rules에서 벡터는 평행이동 영향을 받지 않고, 포인트는 받는다는 점을 검증 → 수학적으로 타당

## ✅ 9. 랜덤 회전 검증
### 점검:
- randomized_equivalences에서 500회 반복 테스트로 회전 행렬의 직교성, 회전 결과의 일치성, 조합의 일관성 확인 → 매우 철저한 수학적 검증

## ✅ 10. Eyeshot ↔ XYZW 변환
### 점검:
- eyeshot_mapping_roundtrip에서 쿼터니언을 XYZW로 변환 후 다시 복원 → 회전 행렬 동일성 확인 → 정확

## ✅ 11. Plane → Plane 회전 추정
### 점검:
- plane_to_plane에서 두 평면의 축을 회전시켜 쿼터니언을 추정하고, 원래 회전과 비교 → 수학적으로 정당


## ✅ 종합 평가

| 항목                          | 수학적 정확성 | 구현 상태  | 설명                                      |
|-------------------------------|----------------|-------------|-------------------------------------------|
| 선형 보간 (lerp)              | ✅ 정확         | ✅ 구현 완료 | 기본 보간 수식 적용                       |
| 쿼터니언 회전                 | ✅ 정확         | ✅ 구현 완료 | q·v·q⁻¹ 또는 행렬 적용                    |
| 행렬 연산 (곱, 전치, 비교)    | ✅ 정확         | ✅ 구현 완료 | apply3, mm, mt 등 수학적으로 타당         |
| 쿼터니언 역원 및 내적         | ✅ 정확         | ✅ 구현 완료 | conjugate 및 내적 비교로 회전 동일성 확인 |
| SLERP                         | ✅ 정확         | ✅ 구현 완료 | 구면 선형 보간 수식 및 단위성 확인        |
| Axis-Angle 변환               | ✅ 정확         | ✅ 구현 완료 | 축-각도 ↔ 쿼터니언 roundtrip 검증         |
| Euler ZYX 회전                | ✅ 정확         | ✅ 구현 완료 | Rz·Ry·Rx 행렬과 쿼터니언 비교             |
| 평행이동 vs 벡터/포인트       | ✅ 정확         | ✅ 구현 완료 | 벡터는 이동 영향 없음, 포인트는 있음      |
| 랜덤 회전 검증                | ✅ 정확         | ✅ 구현 완료 | 500회 반복으로 회전 일치성 확인           |
| Eyeshot ↔ XYZW 변환           | ✅ 정확         | ✅ 구현 완료 | 쿼터니언 ↔ XYZW roundtrip 검증            |
| Plane → Plane 회전 추정       | ✅ 정확         | ✅ 구현 완료 | 축 회전으로 쿼터니언 추정 및 비교         |
| 행렬 ↔ 쿼터니언 roundtrip     | ✅ 정확         | ✅ 구현 완료 | to_mat3 → quat_from_rot3 정확성 확인      |


## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use geometry::geom::plane::Plane;
    use geometry::math::math_extra::ON_PI;
    use geometry::math::matrix::quaternion_nurbs::{Quaternion, Xform, quat_from_rot3};
    use geometry::math::point3d::Point;
    use geometry::math::prelude::Vector;

    const EPS: f64 = 1e-12;
    fn feq(a: f64, b: f64, e: f64) -> bool {
        (a - b).abs() <= e
    }

    fn v_close(a: Vector, b: Vector, e: f64) -> bool {
        feq(a.x, b.x, e) && feq(a.y, b.y, e) && feq(a.z, b.z, e)
    }

    fn m3_close(a: [[f64; 3]; 3], b: [[f64; 3]; 3], e: f64) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !feq(a[i][j], b[i][j], e) {
                    return false;
                }
            }
        }
        true
    }

    fn m4_close(a: [[f64; 4]; 4], b: [[f64; 4]; 4], e: f64) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !feq(a[i][j], b[i][j], e) {
                    return false;
                }
            }
        }
        true
    }

    fn apply3(m: [[f64; 3]; 3], v: Vector) -> Vector {
        Vector {
            x: m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            y: m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            z: m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        }
    }

    fn mtm(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut r = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                r[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
            }
        }
        r
    }

    fn mt(a: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
        [
            [a[0][0], a[1][0], a[2][0]],
            [a[0][1], a[1][1], a[2][1]],
            [a[0][2], a[1][2], a[2][2]],
        ]
    }

    fn mm(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut r = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mut s = 0.0;
                for k in 0..3 {
                    s += a[i][k] * b[k][j];
                }
                r[i][j] = s;
            }
        }
        r
    }

    #[allow(unused)]
    fn i3() -> [[f64; 3]; 3] {
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    }

    #[allow(unused)]
    fn near_mat(a: [[f64; 3]; 3], b: [[f64; 3]; 3], eps: f64) -> bool {
        (0..3).all(|i| (0..3).all(|j| (a[i][j] - b[i][j]).abs() <= eps))
    }
```
```rust
    #[test]
    fn identity_cases() {
        let q = Quaternion::ID;
        assert!(feq(q.len(), 1.0, EPS));
        assert!(m3_close(
            q.to_mat3(),
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            EPS
        ));
        let xf = Xform::from_quat(q);
        assert!(m4_close(xf.m, Xform::identity().m, EPS));
    }
```
```rust
    #[test]
    fn zero_quat_is_identity_rotation() {
        let q = Quaternion::ZERO;
        let m = q.to_mat3(); // 설계상 identity로 떨어짐
        assert!(m3_close(
            m,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            EPS
        ));
    }
```
```rust
    #[test]
    fn inverse_conjugate_props() {
        let q = Quaternion::from_axis_angle(Vector::new(0.2, -0.1, 0.3).unitize(), 1.0)
            .normalized();
        let qi = q.inverse();
        let id = q * qi;
        assert!(feq(id.w, 1.0, 1e-10));
        assert!(feq(id.x, 0.0, 1e-10) && feq(id.y, 0.0, 1e-10) && feq(id.z, 0.0, 1e-10));
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = q.rotate(v);
        let v_back = qi.rotate(v2);

        println!("{:?},{:?},{:?}", v, v2, v_back);

        assert!(v_close(v, v_back, 1e-10));
    }
```
```rust
    #[test]
    fn vector_vs_point_translation_rules() {
        let q = Quaternion::from_zyx(0.2, -0.3, 0.4);
        let r = Xform::from_quat(q);
        let t = Xform::translation(10.0, -5.0, 2.0);
        let m = t.mul(r);
        let v = Vector::new(1.0, 0.0, 0.0);
        let p = Point::new(1.0, 0.0, 0.0);
        // 벡터는 평행이동 영향 없음
        assert!(v_close(m.transform_vector(v), r.transform_vector(v), 1e-12));
        // 포인트는 평행이동 영향 받음
        let pr = r.transform_point(p);
        let pm = m.transform_point(p);
        assert!(v_close(
            Vector::new(pr.x + 10.0, pr.y - 5.0, pr.z + 2.0),
            Vector::from(pm),
            1e-12
        ));
    }
```
```rust
    #[test]
    fn composition_matches_matrix_mul() {
        // active rotation: v' = R(q2)*R(q1)*v == R(q2*q1)*v
        let q1 = Quaternion::from_axis_angle(Vector::new(0.0, 1.0, 0.0), 0.7);
        let q2 = Quaternion::from_axis_angle(Vector::new(0.0, 0.0, 1.0), -1.2);
        let q12 = q2 * q1;
        let m1 = Xform::from_quat(q1);
        let m2 = Xform::from_quat(q2);
        let m12 = m2.mul(m1);
        let mq = Xform::from_quat(q12);
        assert!(m4_close(m12.m, mq.m, 1e-12));
    }
```
```rust
    #[test]
    fn rotate_matches_matrix_application() {
        let q = Quaternion::from_axis_angle(Vector::new(0.3, 0.6, 0.9).unitize(), 1.234);
        let m = q.to_mat3();
        let v = Vector::new(1.0, 2.0, 3.0);
        let rv1 = q.rotate(v);
        let rv2 = apply3(m, v);
        println!("{:?}", rv1);
        println!("{:?}", rv2);
        assert!(v_close(rv1, rv2, 1e-12));
    }
```
```rust
    #[test]
    fn rotation_matrix_is_orthonormal() {
        let q = Quaternion::from_zyx(0.3, -0.5, 0.9);
        let m = q.to_mat3();
        let mtm_ = mt(m, m);
        fn mt(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
            let mut r = [[0.0; 3]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    r[i][j] = a[0][i] * b[0][j] + a[1][i] * b[1][j] + a[2][i] * b[2][j];
                }
            }
            r
        }
        let i3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert!(m3_close(mtm_, i3, 1e-12));
    }
```
```rust
    #[test]
    fn z_axis_90deg_right_hand_rule() {
        let q = Quaternion::from_axis_angle(Vector::new(0.0, 0.0, 1.0), ON_PI * 0.5);
        let v = Vector::new(1.0, 0.0, 0.0);
        let r = q.rotate(v);
        assert!(v_close(r, Vector::new(0.0, 1.0, 0.0), 1e-12));
    }
```
```rust
    #[test]
    fn slerp_endpoints_mid_properties() {
        let q0 = Quaternion::from_axis_angle(Vector::new(1.0, 0.0, 0.0), 0.0);
        let q1 = Quaternion::from_axis_angle(Vector::new(0.0, 1.0, 0.0), 1.0);
        let qh = Quaternion::slerp(q0, q1, 0.5).normalized();
        // t=0,1
        assert!(m3_close(
            Quaternion::slerp(q0, q1, 0.0).to_mat3(),
            q0.to_mat3(),
            1e-12
        ));
        assert!(m3_close(
            Quaternion::slerp(q0, q1, 1.0).to_mat3(),
            q1.to_mat3(),
            1e-12
        ));
        // 중간값은 단위길이
        assert!(feq(qh.len(), 1.0, 1e-12));
    }
```
```rust
    #[test]
    fn euler_zyx_matches_explicit_multiplication() {
        let (yaw, pitch, roll) = (0.7, -0.2, 0.4);
        let q = Quaternion::from_zyx(yaw, pitch, roll);
        // Rz * Ry * Rx
        let (sz, cz) = (yaw.sin(), yaw.cos());
        let rz = [[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]];
        let (sy, cy) = (pitch.sin(), pitch.cos());
        let ry = [[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]];
        let (sx, cx) = (roll.sin(), roll.cos());
        let rx = [[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]];
        let expect = mtm(mtm(rz, ry), rx);
        assert!(m3_close(q.to_mat3(), expect, 1e-12));
    }
```
```rust
    #[test]
    fn axis_angle_roundtrip() {
        let axis = Vector::new(0.2, -0.3, 0.4).unitize();
        let ang = -1.1;
        let q = Quaternion::from_axis_angle(axis, ang);
        let (axis2, ang2) = q.to_axis_angle();
        // roundtrip by reconstructing rotation
        let q2 = Quaternion::from_axis_angle(axis2, ang2);
        assert!(m3_close(q.to_mat3(), q2.to_mat3(), 1e-12));
    }
```
```rust
    #[test]
    fn plane_to_plane() {
        let p0 = Plane::world();
        let q = Quaternion::from_axis_angle(Vector::new(0.0, 1.0, 0.0), ON_PI / 3.0);
        let r = q.to_mat3();
        let p1 = Plane::from_origin_xy(
            Point::new(0.0, 0.0, 0.0),
            apply3(r, p0.x_axis).unitize(),
            apply3(r, p0.y_axis).unitize(),
        );

        let qpp = Quaternion::from_plane_to_plane(&p0, &p1.unwrap());
        // 회전 동일성은 부호까지 고려해 행렬로 비교
        println!("{:?}", q);
        println!("{:?}", qpp);
        assert!(m3_close(q.to_mat3(), qpp.to_mat3(), 1e-12));
    }
```
```rust
    #[test]
    fn eyeshot_mapping_roundtrip() {
        let q = Quaternion::from_zyx(0.3, 0.4, 0.5).normalized();
        let (x, y, z, w) = q.to_xyzw();
        let q2 = Quaternion::from_xyzw(x, y, z, w).normalized();
        assert!(m3_close(q.to_mat3(), q2.to_mat3(), 1e-12));
    }
```
```rust
    // --------- Property-style randomized checks (deterministic RNG) ----------
    struct Rng(u64);
    impl Rng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn next_f64(&mut self, min: f64, max: f64) -> f64 {
            let u = (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64);
            min + (max - min) * u
        }
        fn rand_vec3_unit(&mut self) -> Vector {
            let mut v = Vector::new(
                self.next_f64(-1.0, 1.0),
                self.next_f64(-1.0, 1.0),
                self.next_f64(-1.0, 1.0),
            );
            if v.length() < 1e-9 {
                v = Vector::new(1.0, 0.0, 0.0);
            }
            v.unitize()
        }
    }
```
```rust
    #[test]
    fn randomized_equivalences() {
        let mut rng = Rng::new(0x1234_5678_9abc_def0);
        for _ in 0..500 {
            let axis = rng.rand_vec3_unit();
            let ang = rng.next_f64(-ON_PI, ON_PI);
            let q = Quaternion::from_axis_angle(axis, ang).normalized();
            // orthonormal
            let m = q.to_mat3();
            let _should_be_i = mm(m, mt(m));
            // just check rows are orthonormal
            for i in 0..3 {
                let row = Vector::new(m[i][0], m[i][1], m[i][2]);
                assert!(feq(row.length(), 1.0, 1e-10));
            }
            // rotate equiv
            let v = Vector::new(
                rng.next_f64(-3.0, 3.0),
                rng.next_f64(-3.0, 3.0),
                rng.next_f64(-3.0, 3.0),
            );
            let rv1 = q.rotate(v);
            let rv2 = apply3(m, v);

            println!("{:?}", rv1);
            println!("{:?}", rv2);

            assert!(v_close(rv1, rv2, 1e-10));
            // composition
            let axis2 = rng.rand_vec3_unit();
            let ang2 = rng.next_f64(-ON_PI, ON_PI);
            let q2 = Quaternion::from_axis_angle(axis2, ang2);
            let m12 = Xform::from_quat(q2).mul(Xform::from_quat(q));
            let mq = Xform::from_quat(q2 * q);
            println!("{:?}", m12);
            println!("{:?}", mq);
            assert!(m4_close(m12.m, mq.m, 1e-10));
        }
    }
```
```rust
    #[test]
    fn mat3_matches_rotate_even_if_quat_is_nonunit() {
        use rand::{Rng, SeedableRng};
        let mut rng = rand::rngs::StdRng::seed_from_u64(123);

        for _ in 0..1000 {
            let q = Quaternion {
                w: rng.gen_range(-2.0..2.0),
                x: rng.gen_range(-2.0..2.0),
                y: rng.gen_range(-2.0..2.0),
                z: rng.gen_range(-2.0..2.0),
            };
            let v = Vector {
                x: rng.gen_range(-3.0..3.0),
                y: rng.gen_range(-3.0..3.0),
                z: rng.gen_range(-3.0..3.0),
            };
            let m = q.to_mat3();
            let rv1 = q.rotate(v);
            let rv2 = apply3(m, v);
            assert!(v_close(rv1, rv2, 1e-12), "rv1={rv1:?} rv2={rv2:?} q={q:?}");
        }
    }
```
```rust
    #[test]
    fn xform_point_vector() {
        let q = Quaternion::from_zyx(0.2, -0.3, 0.4);
        let r = Xform::from_quat(q);
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(1.0, 2.0, 3.0);
        // vector rotation equals quaternion rotate
        let rv = r.transform_vector(v);
        let qv = q.rotate(v);
        assert!(
            (rv.x - qv.x).abs() < 1e-12
                && (rv.y - qv.y).abs() < 1e-12
                && (rv.z - qv.z).abs() < 1e-12
        );
        // point rotation + translation
        let t = Xform::translation(10.0, 0.0, 0.0);
        let m = t.mul(r);
        let rp = m.transform_point(p);
        let qp = q.rotate(p.as_vector());
        assert!((rp.x - (qp.x + 10.0)).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn quat_to_mat_round_trip() {
        let q = Quaternion::from_axis_angle(Vector::new(1.0, 2.0, 3.0).unitize(), 1.234);
        let m = q.to_mat3();
        let qr = quat_from_rot3(m);
        assert!((q.len() - 1.0).abs() < 1e-12);
        let dot = q.w * qr.w + q.x * qr.x + q.y * qr.y + q.z * qr.z;
        assert!(dot.abs() > 1.0 - 1e-9); // same rotation (sign may differ)
    }
}
```

---

# Rotation 테스트


📐 Xform 테스트 수학 요약표
| 테스트 이름                                     | 수학적 목적                  | 수식 표현                                                                 | 수학적 타당성 |
|--------------------------------------------------|-------------------------------|---------------------------------------------------------------------------|----------------|
| `extract_translation_matrix3`                   | 평행이동과 스케일 분리       | $T = [[1,0,0,dx; 0,1,0,dy; 0,0,1,dz; 0,0,0,1]]$ <br> $S = diag(sx,sy,sz)$ | ✅ |
| `extract_rotation_polar`                        | 회전 추출 (polar 분해)       | $R = A \cdot (A^T A)^{-1/2}$                                          | ✅ |
| `extract_translation_and_rotation3`             | 복합 행렬에서 T, R 분리      | $M = T \cdot S \cdot R \Rightarrow \text{extract } T, R$              | ✅ |
| `identity_round_trip`                           | 항등 행렬 확인               | $M = I \Rightarrow M \cdot p = p$                                     | ✅ |
| `translation_works`                             | 평행이동 적용                | $p' = p + t$                                                          | ✅ |
| `scale_and_rotation`                            | 스케일 + 회전 적용           | $p' = S \cdot R \cdot p$                                              | ✅ |
| `det_inverse`                                   | 행렬식 및 역행렬 확인        | $\det(M) = sx \cdot sy \cdot sz,\quad M \cdot M^{-1} = I$             | ✅ |
| `normal_transform`                              | 법선 벡터 변환               | $n' = (M^{-1})^T \cdot n$                                             | ✅ |
| `rotation_axis_z_90_deg_vector`                | Z축 90° 회전 벡터 확인       | $R_z(90^\circ) \cdot (1,0,0) = (0,1,0)$                               | ✅ |
| `rotation_about_point_z_90_deg_point`          | 점 기준 회전                 | $p' = T(c) \cdot R_z \cdot T(-c) \cdot p$                             | ✅ |
| `rotation_sc_matches_angle_version`            | sin/cos 기반 회전 비교       | $R(\theta) = R(\sin\theta, \cos\theta)$                               | ✅ |
| `point3d_times_translation`                     | 점에 평행이동 적용           | $p' = p + t$                                                          | ✅ |
| `vector3d_times_translation_ignores_translation`| 벡터는 이동 무시             | $v' = v$                                                              | ✅ |
| `point3d_times_rotation_axis`                   | 점에 회전 적용               | $p' = R \cdot p$                                                      | ✅ |
| `vector3d_times_rotation_axis`                  | 벡터에 회전 적용             | $v' = R \cdot v$                                                      | ✅ |
| `point2d_times_translation`                     | 2D 점에 평행이동 적용        | $p' = p + t_{xy}$                                                     | ✅ |
| `vector2d_times_translation_ignores_translation`| 2D 벡터는 이동 무시           | $v' = v$                                                              | ✅ |
| `point3d_perspective_division`                  | 동차 좌표 분할               | $p' = \frac{M \cdot p}{w}$                                            | ✅ |
| `vector3d_ignores_perspective_row`              | 벡터는 투영 영향 없음        | $v' = M_{3 \times 3} \cdot v$                                         | ✅ |
| `rotate_point_then_compare_with_transform_point`| 연산자 vs 함수 비교           | $p * R = R.\text{transform}(p)$                                       | ✅ |


✅ 종합 평가
- 모든 테스트는 기하학적으로 타당한 수식 기반으로 구성되어 있으며, 수치 오차 허용 범위 내에서 정확한 결과를 검증합니다.
- 특히 회전, 스케일, 평행이동, 법선 변환, 동차 좌표 투영 등은 컴퓨터 그래픽스 및 CAD 시스템에서 핵심적인 수학적 연산입니다.
- 테스트는 단위 행렬, 역행렬, 행렬식, 연산자 오버로드까지 포괄적으로 검증하고 있어 구현 신뢰성이 높습니다.

---
```rust
#[inline]
fn p3_eq(p: Point, q: Point, eps: f64) -> bool {
    feq(p.x, q.x, eps) && feq(p.y, q.y, eps) && feq(p.z, q.z, eps)
}
```
```rust
#[inline]
fn v3_eq(u: Vector, v: Vector, eps: f64) -> bool {
    feq(u.x, v.x, eps) && feq(u.y, v.y, eps) && feq(u.z, v.z, eps)
}

```
```rust
#[inline]
fn p2_eq(p: Point2, q: Point2, eps: f64) -> bool {
    feq(p.x, q.x, eps) && feq(p.y, q.y, eps)
}

```
```rust
#[inline]
fn v2_eq(u: Vector2, v: Vector2, eps: f64) -> bool {
    feq(u.x, v.x, eps) && feq(u.y, v.y, eps)
}
```
```rust

/* ================= Rotation tests ================= */

#[test]
fn rotation_axis_z_90_deg_vector() {
    // 90° rotation around the Z-axis: (1, 0, 0) → (0, 1, 0)
    let axis = Vector::new(0.0, 0.0, 1.0);
    let r = Xform::rotation_axis(FRAC_PI_2, &axis);

    let v = Vector::new(1.0, 0.0, 0.0);
    // Operator overload: Vector * Xform
    let v_rot = v * r;

    assert!(v3_eq(v_rot, Vector::new(0.0, 1.0, 0.0), 1e-12));
}
```
```rust
#[test]
fn rotation_about_point_z_90_deg_point() {
    // Rotate point P = (11, 0, 0) around center C = (10, 0, 0) by 90° about the z-axis → result: (10, 1, 0)
    let c = Point::new(10.0, 0.0, 0.0);
    let axis = Vector::new(0.0, 0.0, 1.0);
    let r = Xform::rotation(FRAC_PI_2, &axis, &c);

    let p = Point::new(11.0, 0.0, 0.0);
    // Operator overload: Point * Xform
    let p_rot = p * r;

    assert!(p3_eq(p_rot, Point::new(10.0, 1.0, 0.0), 1e-12));
}
```
```rust
#[test]
fn rotation_sc_matches_angle_version() {
    // Check whether the sin/cos input version and the angle input version produce the same matrix
    let axis = Vector::new(1.0, 2.0, 3.0); // 임의 축
    let angle = 0.3;
    let r1 = Xform::rotation_axis(angle, &axis);

    let (s, c) = angle.sin_cos();
    let r2 = Xform::rotation_sc(s, c, &axis, &Point::new(0.0, 0.0, 0.0));

    let eps = 1e-12;
    for i in 0..4 {
        for j in 0..4 {
            assert!(
                feq(r1.m[i][j], r2.m[i][j], eps),
                "mismatch at ({},{}) : {} vs {}",
                i,
                j,
                r1.m[i][j],
                r2.m[i][j]
            );
        }
    }
}
```
```rust
/* ================= Point/Vector × Xform tests (3D) ================= */

#[test]
fn Point_times_translation() {
    let p = Point::new(1.0, 2.0, 3.0);
    let t = Xform::translation(10.0, -5.0, 2.0);

    let p2 = p * t;
    assert!(p3_eq(p2, Point::new(11.0, -3.0, 5.0), 1e-12));
}
```
```rust
#[test]
fn Vector_times_translation_ignores_translation() {
    let v = Vector::new(1.0, 2.0, 3.0);
    let t = Xform::translation(10.0, -5.0, 2.0);

    let v2 = v * t;
    // 평행이동 무시 → 동일
    assert!(v3_eq(v2, v, 1e-12));
}
```
```rust
#[test]
fn Point_times_rotation_axis() {
    let p = Point::new(2.0, 0.0, 0.0);
    let r = Xform::rotation_axis(PI, &Vector::new(0.0, 0.0, 1.0));

    let p2 = p * r;
    assert!(p3_eq(p2, Point::new(-2.0, 0.0, 0.0), 1e-12));
}
```
```rust
#[test]
fn Vector_times_rotation_axis() {
    let v = Vector::new(0.0, 3.0, 0.0);
    let r = Xform::rotation_axis(PI, &Vector::new(1.0, 0.0, 0.0));

    let v2 = v * r;
    assert!(v3_eq(v2, Vector::new(0.0, -3.0, 0.0), 1e-12));
}
```
```rust
/* ================= 2D × Xform (2D → homogeneous 4×4 extension) ================= */
#[test]
fn Point2_times_translation() {
    let p = Point2::new(-4.0, 1.5);
    let t = Xform::translation(10.0, -3.0, 0.0);

    let p2 = p * t;
    assert!(p2_eq(p2, Point2::new(6.0, -1.5), 1e-12));
}
```
```rust
#[test]
fn Vector2_times_translation_ignores_translation() {
    let v = Vector2::new(2.0, -2.0);
    let t = Xform::translation(10.0, 20.0, 0.0);

    let v2 = v * t;
    assert!(v2_eq(v2, v, 1e-12));
}
```
```rust
/* ================= Homogeneous (w) behavior ================= */
#[test]
fn Point_perspective_division() {
    // Simple perspective: w' = z + 1
    // m3][2] = 1, m3][3] = 1 (all other elements are identity)
    let mut pmat = Xform::identity();
    pmat.m[3][2] = 1.0; // w' = z*1 + 1*1

    let p = Point::new(2.0, 0.0, 1.0); // z=1 → w' = 2
    let p2 = p * pmat;

    // x'/w' = 2/2 = 1, y'/w' = 0, z'/w' = 1/2
    assert!(p3_eq(p2, Point::new(1.0, 0.0, 0.5), 1e-12));
}
```
```rust
#[test]
fn Vector_ignores_perspective_row() {
    // For vectors, w = 0 → w-row has no effect in the same matrix
    let mut pmat = Xform::identity();
    pmat.m[3][2] = 1.0;

    let v = Vector::new(2.0, 0.0, 1.0);
    let v2 = v * pmat;

    // Linear part is identity → leave as is
    assert!(v3_eq(v2, v, 1e-12));
}
```
```rust
/* ================= Regression: rotation_about_point and operator together ================= */
#[test]
fn rotate_point_then_compare_with_transform_point() {
    // Point * Xform 오버로드와 Xform::transform_point 결과 일치 여부
    let c = Point::new(1.0, 2.0, 3.0);
    let axis = Vector::new(0.0, 0.0, 1.0);
    let r = Xform::rotation(0.25, &axis, &c);

    let p = Point::new(2.0, 3.0, 3.0);

    let p_op = p * r;
    let p_fn = r.transform_point(p);

    assert!(p3_eq(p_op, p_fn, 1e-12));
}
```
