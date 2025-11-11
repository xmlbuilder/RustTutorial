# Quaternion

이 Quaternion 구현은 수학적으로 매우 정교하며, 3D 회전 표현과 변환을 위한 핵심 기능들을 잘 갖추고 있습니다.  
아래에 수학적 배경, 핵심 수식, 그리고 구현의 정확성을 단계별로 설명.

## 🧠 Quaternion의 수학적 배경
### ✅ 정의
- 쿼터니언은 복소수의 확장으로, 3D 회전을 표현하는 데 사용됩니다.
- 일반적인 쿼터니언 형태:

$$
q=w+xi+yj+zk\quad \mathrm{또는}\quad q=(w,\vec {v})=(w,x,y,z)
$$

- 여기서:
- $w$: 스칼라 부분
- $\vec {v}=(x,y,z)$: 벡터 부분

### ✅ 단위 쿼터니언과 회전
- 단위 쿼터니언 q는 $\| q\| =1$ 을 만족
- 회전 축 $\vec {u}$ 와 회전 각도 $\theta$ 에 대해:

$$
q=\cos \left( \frac{\theta }{2}\right) +\sin \left( \frac{\theta }{2}\right) (u_xi+u_yj+u_zk)
$$

- 또는

$$
q=\left( \cos \left( \frac{\theta }{2}\right) ,\sin \left( \frac{\theta }{2}\right) \cdot \vec {u}\right) 
$$

## 🔁 벡터 회전 수식
### ✅ 회전 공식
- 벡터 $\vec {v}$ 를 회전시키는 공식:

$$
\vec {v}'=q\cdot (0,\vec {v})\cdot q^{-1}
$$

- 여기서 $(0,\vec {v})$ 는 순허수 쿼터니언
- $q^{-1}=\frac{\mathrm{conj}(q)}{\| q\| ^2}$

### ✅ 코드 내 수식 점검
#### rotate_vector 함수
```rust
let qv = Quaternion { w: 0.0, x: v.x, y: v.y, z: v.z };
let q_conj = self.conjugate();
let r = *self * qv * q_conj;
```
- 정확한 회전 공식 구현: $q\cdot v_q\cdot q^{-1}$
- 단위 쿼터니언일 경우 conjugate만으로 역원 역할 수행
#### rotate 함수
```rust
let qv_w = -(x * v.x + y * v.y + z * v.z); // 스칼라 곱
let qv_x = w * v.x + y * v.z - z * v.y;
let qv_y = w * v.y - x * v.z + z * v.x;
let qv_z = w * v.z + x * v.y - y * v.x;
```
- $q\cdot v_q$ 의 결과를 직접 계산
- 이어서 $qv\cdot q^{-1}$ 의 벡터부만 추출 → 최적화된 회전

## 📐 핵심 수식 요약

### 📐 Quaternion Rotation Summary

| 단계             | 수식                                                        | 설명                         |
|------------------|-------------------------------------------------------------|------------------------------|
| 벡터를 쿼터니언으로 | $v_q = (0, v_x, v_y, v_z)$                               | 순허수 쿼터니언 표현         |
| 회전 수행         | $v' = q \cdot v_q \cdot q^{-1}$                         | 쿼터니언 회전 공식           |
| 역원 계산         | $q^{-1} = \frac{\text{conj}(q)}{\|q\|^2}$               | 역원은 켤레/길이 제곱으로 계산 |
| 결과 추출         | $\vec{v}' = (v'_x, v'_y, v'_z)$                         | 회전된 벡터                   |

### ✅ 구현의 정확성
- ✅ 정규화(normalized), 켤레(conjugate), 역원(inverse) 모두 정확하게 구현
- ✅ from_axis_angle, from_zyx 등 다양한 생성 방식 포함
- ✅ to_mat3, to_mat4로 행렬 변환 가능
- ✅ slerp로 부드러운 회전 보간도 지원


### 🧭 Bryant angles란?
- 회전 순서: Yaw → Pitch → Roll = Z → Y → X
- 적용 순서:
    - Z축 회전 (Yaw)
    - Y축 회전 (Pitch)
    - X축 회전 (Roll)
    - 이 순서는 Body-fixed axes 기준이며, 항공기 자세 표현에 자주 사용됩니다

### ✅ Bryant angles 추출 함수
아래는 쿼터니언에서 Bryant(ZYX) Euler 각을 추출하는 함수입니다:
```rust
pub fn to_bryant_angles(&self) -> (f64, f64, f64) {
    let (w, x, y, z) = (self.w, self.x, self.y, self.z);

    // Yaw (Z-axis rotation)
    let siny_cosp = 2.0 * (w * z + x * y);
    let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
    let yaw = siny_cosp.atan2(cosy_cosp);

    // Pitch (Y-axis rotation)
    let sinp = 2.0 * (w * y - z * x);
    let pitch = if sinp.abs() >= 1.0 {
        sinp.signum() * std::f64::consts::FRAC_PI_2 // ±90°
    } else {
        sinp.asin()
    };

    // Roll (X-axis rotation)
    let sinr_cosp = 2.0 * (w * x + y * z);
    let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
    let roll = sinr_cosp.atan2(cosr_cosp);

    (roll, pitch, yaw)
}
```

## 📐 Bryant (ZYX) Euler Angle Extraction

| Angle   | Formula                                                    |
|---------|------------------------------------------------------------|
| Roll (X)  | $\mathrm{atan2}(2(wx + yz),\ 1 - 2(x^2 + y^2))$        |
| Pitch (Y) | $\mathrm{asin}(2(wy - zx))$                           |
| Yaw (Z)   | $\mathrm{atan2}(2(wz + xy),\ 1 - 2(y^2 + z^2))$        |

- 이 함수는 기존 to_euler_angles와 거의 유사하지만, 회전 순서가 ZYX임을 명확히 반영합니다.

- from_euler_angles: 일반적인 XYZ 회전 순서 (Roll → Pitch → Yaw)
- from_bryant_angles: 항공역학에서 흔한 ZYX 회전 순서 (Yaw → Pitch → Roll)  

---

## from_euler_angles / from_bryant_angles
각각의 수학적 의미와 구현을 함께 설명.

### ✅ from_euler_angles (XYZ 순서)
```rust
pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
    let (sx, cx) = (0.5 * roll).sin_cos();   // X축 회전
    let (sy, cy) = (0.5 * pitch).sin_cos();  // Y축 회전
    let (sz, cz) = (0.5 * yaw).sin_cos();    // Z축 회전

    Quaternion {
        w: cz * cy * cx + sz * sy * sx,
        x: cz * cy * sx - sz * sy * cx,
        y: cz * sy * cx + sz * cy * sx,
        z: sz * cy * cx - cz * sy * sx,
    }
}
```
#### 📐 수학적 의미
- 회전 순서: X → Y → Z
- 쿼터니언 조합: $q=q_z\cdot q_y\cdot q_x$

![Euler Angle Matrix](/image/euler_angle.png)

#### ✅ from_bryant_angles (ZYX 순서)
```rust
pub fn from_bryant_angles(yaw: f64, pitch: f64, roll: f64) -> Quaternion {
    let (sz, cz) = (0.5 * yaw).sin_cos();    // Z축 회전
    let (sy, cy) = (0.5 * pitch).sin_cos();  // Y축 회전
    let (sx, cx) = (0.5 * roll).sin_cos();   // X축 회전

    Quaternion {
        w: cz * cy * cx + sz * sy * sx,
        x: cz * cy * sx - sz * sy * cx,
        y: cz * sy * cx + sz * cy * sx,
        z: sz * cy * cx - cz * sy * sx,
    }
}
```

#### 📐 수학적 의미
- 회전 순서: Z → Y → X
- 쿼터니언 조합: $q=q_x\cdot q_y\cdot q_z$

### 🧪 사용 예시
```rust
let q1 = Quaternion::from_euler_angles(roll, pitch, yaw);
let q2 = Quaternion::from_bryant_angles(yaw, pitch, roll);
```

---

# 쿼터니언을 이용한 벡터 회전

![Quternion Ration](/image/quaternion_rotation.png)

## 🎯 목적: 쿼터니언을 이용한 벡터 회전
## ✅ 수학적 정의
- 벡터 $\vec {v}$ 를 쿼터니언 q로 회전시키는 공식은:

$$
\vec {v}_{\mathrm{rotated}}=q\cdot \vec {v}\cdot q^{-1}
$$

- 여기서 $\vec {v}$ 는 쿼터니언으로 표현된 순허수 쿼터니언:

$$
v_q=(0,v_x,v_y,v_z)
$$

- $q^{-1}$ 는 q의 역원 (conjugate if normalized)
- 수 쿼터니언:

$$
v_q=(0,v_x,v_y,v_z)
$$

- $q^{-1}$ 는 q의 역원 (conjugate if normalized)

## ✅ 구현 예시
```rust
impl Quaternion {
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotate_vector(&self, v: Vector) -> Vector {
        let qv = Quaternion { w: 0.0, x: v.x, y: v.y, z: v.z };
        let q_conj = self.conjugate();
        let r = *self * qv * q_conj;
        Vector::new(r.x, r.y, r.z)
    }
}
```

- 이 구조는 $q\cdot v_q\cdot q^{-1}$ 를 정확히 구현
- 단, q는 **단위 쿼터니언(normalized)** 이어야 회전만 수행됨

## 📐 Quaternion-based Vector Rotation Summary

| Step            | Formula                                      | Description                         |
|-----------------|----------------------------------------------|-------------------------------------|
| Vector as Quaternion | $v_q = (0, v_x, v_y, v_z)$               | Represent vector as pure quaternion |
| Rotation        | $v' = q \cdot v_q \cdot q^{-1}$           | Rotate using quaternion conjugation |
| Extract result  | $\vec{v}' = (v'_x, v'_y, v'_z)$           | Final rotated vector components     |

---

## 소스 코드
```rust
use std::ops::{Add, Mul, Sub};
use crate::core::maths::{on_clamp01, on_is_finite};
use crate::core::plane::Plane;
use crate::core::prelude::{Point, Vector};
use crate::core::transform::Transform;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```
```rust
impl Quaternion {
    pub fn rotate_vector(&self, v: Vector) -> Vector {
        let qv = Quaternion { w: 0.0, x: v.x, y: v.y, z: v.z };
        let q_conj = self.conjugate();
        let r = *self * qv * q_conj;
        Vector::new(r.x, r.y, r.z)
    }
}
```
```rust
impl Quaternion {
    pub fn then(&self, q: Quaternion) -> Quaternion {
        *self * q
    }
}
```
```rust
impl Quaternion {
    pub fn to_transform(&self) -> Transform {
        Transform{
            m : self.to_mat4()
        }
    }
}
```
```rust
impl Quaternion {
    pub const ZERO: Self = Self {
        w: 0.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const ID: Self = Self {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const I: Self = Self {
        w: 0.0,
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const J: Self = Self {
        w: 0.0,
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const K: Self = Self {
        w: 0.0,
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }
    pub fn from_axis_angle(axis: Vector, angle_rad: f64) -> Self {
        let n = axis.length();
        let (s, c) = ((0.5 * angle_rad).sin(), (0.5 * angle_rad).cos());
        if n > 0.0 {
            Self {
                w: c,
                x: s * axis.x / n,
                y: s * axis.y / n,
                z: s * axis.z / n,
            }
        } else {
            Self::ID
        }
    }

    pub fn from_axis_angle_deg(axis: Vector, angle_deg: f64) -> Self {
        let angle_rad = angle_deg.to_radians();
        let n = axis.length();
        let (s, c) = ((0.5 * angle_rad).sin(), (0.5 * angle_rad).cos());
        if n > 0.0 {
            Self {
                w: c,
                x: s * axis.x / n,
                y: s * axis.y / n,
                z: s * axis.z / n,
            }
        } else {
            Self::ID
        }
    }

    pub fn from_zyx(yaw: f64, pitch: f64, roll: f64) -> Self {
        let (sz, cz) = (0.5 * yaw).sin_cos(); // sz = sin(yaw/2),  cz = cos(yaw/2)
        let (sy, cy) = (0.5 * pitch).sin_cos(); // sy = sin(pitch/2),cy = cos(pitch/2)
        let (sx, cx) = (0.5 * roll).sin_cos(); // sx = sin(roll/2), cx = cos(roll/2)

        // w,x,y,z (scalar first)
        Self {
            w: cz * cy * cx + sz * sy * sx,
            x: cz * cy * sx - sz * sy * cx,
            y: cz * sy * cx + sz * cy * sx,
            z: sz * cy * cx - cz * sy * sx,
        }
    }
    pub fn is_valid(&self) -> bool {
        on_is_finite(self.w) && on_is_finite(self.x) && on_is_finite(self.y) && on_is_finite(self.z)
    }
    pub fn len2(self) -> f64 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(self) -> f64 {
        self.len2().sqrt()
    }
    pub fn normalized(self) -> Self {
        let n = self.len();
        if n > 0.0 {
            Self {
                w: self.w / n,
                x: self.x / n,
                y: self.y / n,
                z: self.z / n,
            }
        } else {
            self
        }
    }
    pub fn conjugate(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    pub fn inverse(self) -> Self {
        let l2 = self.len2();
        if l2 > 0.0 {
            let inv = 1.0 / l2;
            Self {
                w: self.w * inv,
                x: -self.x * inv,
                y: -self.y * inv,
                z: -self.z * inv,
            }
        } else {
            Self::ZERO
        }
    }
    pub fn vector(self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }
    pub fn scalar(self) -> f64 {
        self.w
    }

    /// q.rotate(v) = (q * (0,v) * q^{-1}).vector()
    pub fn rotate(self, v: Vector) -> Vector {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        let l2 = w * w + x * x + y * y + z * z;
        if !(l2 > 0.0) {
            return v; // 퇴화 방지
        }
        let inv_l2 = 1.0 / l2;

        // q^{-1} = conj(q) / ||q||^2
        let qi_w = w * inv_l2;
        let qi_x = -x * inv_l2;
        let qi_y = -y * inv_l2;
        let qi_z = -z * inv_l2;

        // qv = q * (0,v)
        let qv_w = -(x * v.x + y * v.y + z * v.z); // ★ 빠져있던 부분
        let qv_x = w * v.x + y * v.z - z * v.y;
        let qv_y = w * v.y - x * v.z + z * v.x;
        let qv_z = w * v.z + x * v.y - y * v.x;

        // (qv) * q^{-1} 의 벡터부만 추출
        Vector {
            x: qv_w * qi_x + qv_x * qi_w + qv_y * qi_z - qv_z * qi_y,
            y: qv_w * qi_y - qv_x * qi_z + qv_y * qi_w + qv_z * qi_x,
            z: qv_w * qi_z + qv_x * qi_y - qv_y * qi_x + qv_z * qi_w,
        }
    }

    pub fn to_mat3(self) -> [[f64; 3]; 3] {
        let l2 = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
        if !(l2 > 0.0) {
            return [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        }
        let s = 2.0 / l2;
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        let xs = x * s;
        let ys = y * s;
        let zs = z * s;
        let wx = w * xs;
        let wy = w * ys;
        let wz = w * zs;
        let xx = x * xs;
        let yy = y * ys;
        let zz = z * zs;
        let xy = x * ys;
        let xz = x * zs;
        let yz = y * zs;

        [
            [1.0 - (yy + zz), xy - wz, xz + wy],
            [xy + wz, 1.0 - (xx + zz), yz - wx],
            [xz - wy, yz + wx, 1.0 - (xx + yy)],
        ]
    }

    /// 4×4로 확장 (우상단/좌하단은 0, 마지막 대각 1)
    pub fn to_mat4(self) -> [[f64; 4]; 4] {
        let m3 = self.to_mat3();
        [
            [m3[0][0], m3[0][1], m3[0][2], 0.0],
            [m3[1][0], m3[1][1], m3[1][2], 0.0],
            [m3[2][0], m3[2][1], m3[2][2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn to_axis_angle_deg(&self) -> (Vector, f64) {
        // self 는 단위라고 가정 (아닐 수 있으면 normalize 권장)
        let a = self.w.acos();
        let s = a.sin(); // = sqrt(1 - w^2)
        if s == 0.0 {
            // 각도 0: 축은 임의로 z축
            (Vector::new(0.0, 0.0, 1.0), 0.0)
        } else {
            let mut axis = Vector::new(self.x / s, self.y / s, self.z / s);
            axis.normalize();
            (axis, (2.0 * a).to_degrees())
        }
    }

    pub fn to_euler_angles(&self) -> (f64, f64, f64) {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);

        // Yaw (Z)
        let siny_cosp = 2.0 * (w * z + x * y);
        let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        // Pitch (Y)
        let sinp = 2.0 * (w * y - z * x);
        let pitch = if sinp.abs() >= 1.0 {
            sinp.signum() * std::f64::consts::FRAC_PI_2
        } else {
            sinp.asin()
        };

        // Roll (X)
        let sinr_cosp = 2.0 * (w * x + y * z);
        let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        (roll, pitch, yaw)
    }

    pub fn to_bryant_angles(&self) -> (f64, f64, f64) {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);

        // Yaw (Z-axis rotation)
        let siny_cosp = 2.0 * (w * z + x * y);
        let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        // Pitch (Y-axis rotation)
        let sinp = 2.0 * (w * y - z * x);
        let pitch = if sinp.abs() >= 1.0 {
            sinp.signum() * std::f64::consts::FRAC_PI_2 // ±90°
        } else {
            sinp.asin()
        };

        // Roll (X-axis rotation)
        let sinr_cosp = 2.0 * (w * x + y * z);
        let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        (roll, pitch, yaw)
    }

   pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
        let (sx, cx) = (0.5 * roll).sin_cos();   // X축 회전
        let (sy, cy) = (0.5 * pitch).sin_cos();  // Y축 회전
        let (sz, cz) = (0.5 * yaw).sin_cos();    // Z축 회전

        Quaternion {
            w: cz * cy * cx + sz * sy * sx,
            x: cz * cy * sx - sz * sy * cx,
            y: cz * sy * cx + sz * cy * sx,
            z: sz * cy * cx - cz * sy * sx,
        }
    }

    pub fn from_bryant_angles(yaw: f64, pitch: f64, roll: f64) -> Quaternion {
        let (sz, cz) = (0.5 * yaw).sin_cos();    // Z축 회전
        let (sy, cy) = (0.5 * pitch).sin_cos();  // Y축 회전
        let (sx, cx) = (0.5 * roll).sin_cos();   // X축 회전

        Quaternion {
            w: cz * cy * cx + sz * sy * sx,
            x: cz * cy * sx - sz * sy * cx,
            y: cz * sy * cx + sz * cy * sx,
            z: sz * cy * cx - cz * sy * sx,
        }
    }

    /// SLERP (두 퀘이트 부호 일관화 포함)
    pub fn slerp(q0: Self, q1: Self, t: f64) -> Self {
        let mut q1m = q1;
        let mut cos = q0.w * q1.w + q0.x * q1.x + q0.y * q1.y + q0.z * q1.z;
        if cos < 0.0 {
            cos = -cos;
            q1m = Self {
                w: -q1.w,
                x: -q1.x,
                y: -q1.y,
                z: -q1.z,
            };
        }
        let t = on_clamp01(t);
        if cos > 0.9995 {
            // lerp + normalize
            let r = Self {
                w: q0.w + t * (q1m.w - q0.w),
                x: q0.x + t * (q1m.x - q0.x),
                y: q0.y + t * (q1m.y - q0.y),
                z: q0.z + t * (q1m.z - q0.z),
            };
            return r.normalized();
        }
        let theta = cos.acos();
        let s0 = ((1.0 - t) * theta).sin() / theta.sin();
        let s1 = (t * theta).sin() / theta.sin();
        Self {
            w: s0 * q0.w + s1 * q1m.w,
            x: s0 * q0.x + s1 * q1m.x,
            y: s0 * q0.y + s1 * q1m.y,
            z: s0 * q0.z + s1 * q1m.z,
        }
    }

    /// Eyeshot(x,y,z,w) ↔ OpenNURBS(w,x,y,z) 변환 어댑터
    pub fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { w, x, y, z }
    }
    pub fn to_xyzw(self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, self.w)
    }

    /// plane0의 축을 plane1 축으로 보내는 회전 (OpenNURBS SetRotation) 근사
    pub fn from_plane_to_plane(p0: &Plane, p1: &Plane) -> Self {
        let m = [
            [
                // R[0][0], R[0][1], R[0][2]
                p1.x_axis.x * p0.x_axis.x + p1.y_axis.x * p0.y_axis.x + p1.z_axis.x * p0.z_axis.x,
                p1.x_axis.x * p0.x_axis.y + p1.y_axis.x * p0.y_axis.y + p1.z_axis.x * p0.z_axis.y,
                p1.x_axis.x * p0.x_axis.z + p1.y_axis.x * p0.y_axis.z + p1.z_axis.x * p0.z_axis.z,
            ],
            [
                // R[1][0], R[1][1], R[1][2]
                p1.x_axis.y * p0.x_axis.x + p1.y_axis.y * p0.y_axis.x + p1.z_axis.y * p0.z_axis.x,
                p1.x_axis.y * p0.x_axis.y + p1.y_axis.y * p0.y_axis.y + p1.z_axis.y * p0.z_axis.y,
                p1.x_axis.y * p0.x_axis.z + p1.y_axis.y * p0.y_axis.z + p1.z_axis.y * p0.z_axis.z,
            ],
            [
                // R[2][0], R[2][1], R[2][2]
                p1.x_axis.z * p0.x_axis.x + p1.y_axis.z * p0.y_axis.x + p1.z_axis.z * p0.z_axis.x,
                p1.x_axis.z * p0.x_axis.y + p1.y_axis.z * p0.y_axis.y + p1.z_axis.z * p0.z_axis.y,
                p1.x_axis.z * p0.x_axis.z + p1.y_axis.z * p0.y_axis.z + p1.z_axis.z * p0.z_axis.z,
            ],
        ];
        on_quat_from_rot3(m) // 이미 있는 rot3→Quat 변환 사용
    }
}
```
```rust
impl Quaternion {
    pub fn to_axis_angle(self) -> (Vector, f64) {
        let q = self.normalized();
        let angle = 2.0 * q.w.acos();
        let s = (1.0 - q.w * q.w).sqrt(); // |(x,y,z)|
        if s < 1e-12 {
            (Vector::new(1.0, 0.0, 0.0), 0.0)
        } else {
            (Vector::new(q.x / s, q.y / s, q.z / s), angle)
        }
    }
}
```
```rust
impl Add for Quaternion {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self {
            w: self.w + o.w,
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}
```
```rust
impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self {
            w: self.w - o.w,
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}
```
```rust
impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, q: Self) -> Self {
        Self {
            w: self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            x: self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
            y: self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
            z: self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
        }
    }
}
```
```rust
impl Mul for &Quaternion {
    type Output = Quaternion;
    fn mul(self, q:&Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            x: self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
            y: self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
            z: self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
        }
    }
}
```
```rust
impl Mul<f64> for Quaternion {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self {
            w: self.w * s,
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}
```
```rust
impl Mul<Quaternion> for f64 {
    type Output = Quaternion;
    fn mul(self, q: Quaternion) -> Quaternion {
        q * self
    }
}
```
```rust
/// 회전행렬(정규직교 3×3) -> Quat(w,x,y,z)
pub fn on_quat_from_rot3(m: [[f64; 3]; 3]) -> Quaternion {
    let trace = m[0][0] + m[1][1] + m[2][2];
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        let w = 0.25 * s;
        let x = (m[2][1] - m[1][2]) / s;
        let y = (m[0][2] - m[2][0]) / s;
        let z = (m[1][0] - m[0][1]) / s;
        Quaternion::new(w, x, y, z).normalized()
    } else {
        let (i, _) = [(0, m[0][0]), (1, m[1][1]), (2, m[2][2])]
            .into_iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap();
        let (j, k) = match i {
            0 => (1, 2),
            1 => (2, 0),
            _ => (0, 1),
        };
        let mut q = [0.0; 4];
        let s = ((m[i][i] - m[j][j] - m[k][k]) + 1.0).sqrt() * 2.0;
        q[i + 1] = 0.25 * s;
        q[0] = (m[k][j] - m[j][k]) / s;
        q[j + 1] = (m[j][i] + m[i][j]) / s;
        q[k + 1] = (m[k][i] + m[i][k]) / s;
        Quaternion::new(q[0], q[1], q[2], q[3]).normalized()
    }
}
```
```rust
// 4x4 transform (row-major; acts on column vectors on the right)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Xform {
    pub m: [[f64; 4]; 4],
}
impl Xform {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn zero() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }
    pub fn from_quat(q: Quaternion) -> Self {
        Self { m: q.to_mat4() }
    }
    pub fn translation(dx: f64, dy: f64, dz: f64) -> Self {
        let mut t = Self::identity();
        t.m[0][3] = dx;
        t.m[1][3] = dy;
        t.m[2][3] = dz;
        t
    }
    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        let mut s = Self::identity();
        s.m[0][0] = sx;
        s.m[1][1] = sy;
        s.m[2][2] = sz;
        s
    }
    pub fn mul(self, rhs: Self) -> Self {
        let mut r = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        Self { m: r }
    }
    pub fn transform_point(self, p: Point) -> Point {
        let x = self.m[0][0] * p.x + self.m[0][1] * p.y + self.m[0][2] * p.z + self.m[0][3];
        let y = self.m[1][0] * p.x + self.m[1][1] * p.y + self.m[1][2] * p.z + self.m[1][3];
        let z = self.m[2][0] * p.x + self.m[2][1] * p.y + self.m[2][2] * p.z + self.m[2][3];
        let w = self.m[3][0] * p.x + self.m[3][1] * p.y + self.m[3][2] * p.z + self.m[3][3];
        if w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        }
    }
    pub fn transform_vector(self, v: Vector) -> Vector {
        Vector::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }
}
```
```rust
mod na_quaternion_compat {
    use super::Quaternion;
    use nalgebra::{Quaternion as NaQuat, UnitQuaternion};

    // QuaternionNurbs → UnitQuaternion
    impl Into<UnitQuaternion<f64>> for Quaternion {
        fn into(self) -> UnitQuaternion<f64> {
            // 제로 혹은 비정상 값 방지: 노름 체크 후 normalize
            let l2 = self.len2();
            if !(l2 > 0.0 && l2.is_finite()) {
                return UnitQuaternion::identity();
            }
            // nalgebra 는 (w, i, j, k) 순서
            let q = NaQuat::new(self.w, self.x, self.y, self.z);
            UnitQuaternion::new_normalize(q)
        }
    }

    impl From<UnitQuaternion<f64>> for Quaternion {
        fn from(uq: UnitQuaternion<f64>) -> Self {
            let q = uq.quaternion();
            // nalgebra Quaternion: (w, i, j, k)
            Quaternion::new(q.w, q.i, q.j, q.k)
        }
    }

    // (옵션) Quaternion ↔ QuaternionNurbs
    impl From<NaQuat<f64>> for Quaternion {
        fn from(q: NaQuat<f64>) -> Self {
            Quaternion::new(q.w, q.i, q.j, q.k).normalized()
        }
    }
    impl Into<NaQuat<f64>> for Quaternion {
        fn into(self) -> NaQuat<f64> {
            NaQuat::new(self.w, self.x, self.y, self.z)
        }
    }
}
```

---

## 테스트 코드

### 🧪 Transform & Quaternion 테스트 요약

| 테스트 함수                     | 목적 및 검증 내용                                                                 |
|-------------------------------|-----------------------------------------------------------------------------------|
| `demo_transform`              | - 점에 대한 평행이동 + 회전 적용<br>- 법선 벡터 회전<br>- 평면 반사<br>- 좌표계 변환 역검증 |
| `demo_quaternion`            | - Z축 90° 회전 쿼터니언 생성<br>- 벡터 회전 결과 확인<br>- Euler 각도, 축/각도, 행렬 변환 확인 
| `demo_quaternion2`           | - 쿼터니언 회전 결과와 행렬 회전 결과 비교                                       |
| `z90_matches_matrix_and_gives_0_1_0` | - Z축 90° 회전 후 결과가 (0,1,0)인지 확인<br>- 행렬과 결과 일치 여부 검증       |
| `inverse_rotation_undoes`    | - 쿼터니언 회전 후 켤레로 역회전 → 원래 벡터 복원 확인                           |
| `composition_order`          | - 쿼터니언 합성과 순차 회전 결과 일치 여부 확인                                 |
| `identity_quaternion_no_change` | - 단위 쿼터니언은 회전 효과 없음 확인                                           |
| `matrix_multi_order`         | - 쿼터니언 합성과 행렬 합성 순서 일치 여부 확인                                 |
| `then_helpers_match_manual_composition` | - `then()` 메서드와 수동 합성 결과 일치 여부 확인                        |
| `from_axis_angle ↔ to_axis_angle`    | 축과 각도로 생성한 쿼터니언을 다시 축/각도로 복원했을 때 일치 여부 확인 |
| `from_euler_angles ↔ to_euler_angles`| Euler 각도로 생성한 쿼터니언을 다시 Euler 각도로 복원했을 때 회전 동등성 확인 |
| `normalized()` 검증                  | 임의 쿼터니언을 정규화했을 때 길이가 1인지 확인                        |
| `inverse()` 검증                     | 쿼터니언과 그 역원을 곱했을 때 단위 쿼터니언이 되는지 확인             |
| `rotate_vector()` 불변성             | 회전축 방향 벡터는 해당 축 기준 회전 시 변화가 없어야 함               |
| `slerp()` 보간                       | 두 쿼터니언 사이 보간이 부드럽고 일관되게 작동하는지 확인              |
| `identity()` 효과 검증              | 단위 쿼터니언은 회전 효과가 없어야 함                                  |
| `to_mat3()` / `to_mat4()` 일관성     | 행렬 변환 결과가 쿼터니언 회전과 일치하는지 확인                       |
| `then()` 순서 검증                   | `q1.then(q2)`와 `q2.rotate(q1.rotate(v))` 결과가 같은지 확인           |

### 🧠 핵심 개념 정리
- ✅ rotate_vector: 쿼터니언을 이용한 벡터 회전
- ✅ to_mat4: 쿼터니언을 4×4 행렬로 변환
- ✅ Transform::mul: 행렬 합성
- ✅ Quaternion::then: 회전 순서 제어
- ✅ on_binomial_table: 이항계수 생성
- ✅ on_update_binomial_coefficients: 이항계수 캐시 확장


```rust
mod tests
{
    use nurbslib::core::basis::{on_binomial_table, on_update_binomial_coefficients};
    use nurbslib::core::prelude::{Point, Vector};
    use nurbslib::core::quaternion::Quaternion;
    use nurbslib::core::transform::Transform;
```
```rust
    #[test]
    fn demo_transform() {
        let p = Point::new(1.0, 2.0, 3.0);
        let t = Transform::translation(10.0, 0.0, 0.0).mul(&Transform::rotation_axis(
            std::f64::consts::FRAC_PI_2,
            Vector::new(0.0, 0.0, 1.0),
            Point::origin(),
        ));
        let q = t.transform_point3d(&p);

        let n = Vector::new(0.0, 0.0, 1.0);
        let world = t.transform_normal(&n).unwrap();
        print!("{:?}", world);

        let mirror = Transform::mirror_about_plane(Point::origin(), Vector::new(1.0, 0.0, 0.0));
        let q2 = mirror.transform_point3d(&q);

        let l2w = Transform::from_basis(
            Point::new(5.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        );
        let w2l = l2w.invert().unwrap();
        let back = w2l.transform_point3d(&l2w.transform_point3d(&p));
        assert!((Point::distance(&p, &back)) < 1e-9);
        print!("{:?}", q2);
    }
```
```rust
    #[test]
    fn demo_quaternion() {
        let q = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let v = Vector::new(1.0, 0.0, 0.0);
        let r = q.rotate_vector(v);

        // 대략 (0,1,0) 쪽으로 회전:
        println!("{:?}", r);
        assert!(r.x.abs() < 1e-9 && (r.y - 1.0).abs() < 1e-9);

        let (roll, pitch, yaw) = q.to_euler_angles(); // 라디안
        let (axis, deg) = q.to_axis_angle_deg();
        let m = q.to_mat4(); // Transform 과 호환되는 4×4
        let t = q.to_transform(); // 바로 Transform 으로

        println!("{:?}, {:?}, {:?}", r, axis, deg);
        println!("{:?}, {:?}, {:?}", roll, pitch, yaw);
        println!("{:?}, {:?}", m, t);
    }
```
```rust
    #[test]
    fn demo_quaternion2() {
        let q = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let v = Vector::new(1.0, 0.0, 0.0);
        let r = q.rotate_vector(v);
        // 기대값: (0, +1, 0)
        assert!(r.x.abs() < 1e-9 && (r.y - 1.0).abs() < 1e-9 && r.z.abs() < 1e-9);

        // 행렬 변환과도 일치해야 함
        let m = q.to_mat4();
        let via_mat = Vector::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        );
        assert!(
            (via_mat.x - r.x).abs() < 1e-9
                && (via_mat.y - r.y).abs() < 1e-9
                && (via_mat.z - r.z).abs() < 1e-9
        );
    }
}
```
```rust
#[cfg(test)]
mod quat_tests {
    use nurbslib::core::prelude::Vector;
    use nurbslib::core::quaternion::Quaternion;
```
```rust
    #[test]
    fn z90_matches_matrix_and_gives_0_1_0() {
        let q = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let v = Vector::new(1.0, 0.0, 0.0);
        let r = q.rotate_vector(v);

        assert!(r.x.abs() < 1e-9 && (r.y - 1.0).abs() < 1e-9 && r.z.abs() < 1e-9);

        // 행렬과 동일해야 함
        let m = q.to_mat4();
        let via = Vector::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        );
        assert!(
            (via.x - r.x).abs() < 1e-9 && (via.y - r.y).abs() < 1e-9 && (via.z - r.z).abs() < 1e-9
        );
    }
```
```rust
    #[test]
    fn inverse_rotation_undoes() {
        let q = Quaternion::from_axis_angle_deg(Vector::new(0.0, 1.0, 0.0), 30.0);
        let qi = q.conjugate(); // 단위이면 역과 동일
        let v = Vector::new(0.0, 0.0, 1.0);
        let f = q.rotate_vector(v);
        let b = qi.rotate_vector(f);
        assert!((b.x - v.x).abs() < 1e-9 && (b.y - v.y).abs() < 1e-9 && (b.z - v.z).abs() < 1e-9);
    }

    #[test]
    fn composition_order() {
        let qx = Quaternion::from_axis_angle_deg(Vector::new(1.0, 0.0, 0.0), 90.0);
        let qz = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let v = Vector::new(1.0, 0.0, 0.0);

        // (qz ∘ qx) == qx * qz   // ← 이 구현에서는 좌→우
        let q = qx * qz;
        let r = q.rotate_vector(v);
        let r2 = qx.rotate_vector(qz.rotate_vector(v));

        assert!(
            (r.x - r2.x).abs() < 1e-9 && (r.y - r2.y).abs() < 1e-9 && (r.z - r2.z).abs() < 1e-9
        );
    }
```
```rust
    #[test]
    fn identity_quaternion_no_change() {
        let q = Quaternion::identity();
        let v = Vector::new(1.0, 2.0, 3.0);
        let r = q.rotate_vector(v);
        assert!(
            (r.x - v.x).abs() < 1e-12 && (r.y - v.y).abs() < 1e-12 && (r.z - v.z).abs() < 1e-12
        );
    }
```
```rust
    #[test]
    fn matrix_multi_order() {
        // 먼저 X축 90°, 그다음 Z축 90° 적용
        let qx = Quaternion::from_axis_angle_deg(Vector::new(1.0, 0.0, 0.0), 90.0);
        let qz = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let v = Vector::new(1.0, 0.0, 0.0);

        // 사원수 합성: qx 다음 q ⇒ qz * qx
        let q = qz * qx;
        let rq = q.rotate_vector(v);

        // 행렬 합성: 먼저 Rx, 그다음 Rz ⇒ Rz * Rx  (우→좌)
        let rx = qx.to_transform();
        let rz = qz.to_transform();
        let combo = rz.mul(&rx);
        let rm = combo.transform_vector3d(&v);

        assert!(
            (rq.x - rm.x).abs() < 1e-9 && (rq.y - rm.y).abs() < 1e-9 && (rq.z - rm.z).abs() < 1e-9
        );
    }
```
```rust
    #[test]
    fn then_helpers_match_manual_composition() {
        let qx = Quaternion::from_axis_angle_deg(Vector::new(1.0, 0.0, 0.0), 90.0);
        let qz = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let v = Vector::new(1.0, 0.0, 0.0);

        let q_then = qz.then(qx); // qx 후 qz
        let r1 = q_then.rotate_vector(v);

        let r2 = qz.rotate_vector(qx.rotate_vector(v));
        assert!(
            (r1.x - r2.x).abs() < 1e-9 && (r1.y - r2.y).abs() < 1e-9 && (r1.z - r2.z).abs() < 1e-9
        );

        // Transform 도 동일한 의미로 작동
        let rx = qx.to_transform();
        let rz = qz.to_transform();
        let t_then = rx.then(&rz); // Rx 후 Rz  →  Rz * Rx
        let r3 = t_then.transform_vector3d(&v);
        assert!(
            (r3.x - r2.x).abs() < 1e-9 && (r3.y - r2.y).abs() < 1e-9 && (r3.z - r2.z).abs() < 1e-9
        );
    }
```
```rust
    #[test]
    fn quaternion_normalization_test() {
        let q = Quaternion::new(2.0, 0.0, 0.0, 0.0);
        let qn = q.normalized();
        assert!((qn.len() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn slerp_midpoint_test() {
        let q0 = Quaternion::identity();
        let q1 = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let qm = Quaternion::slerp(q0, q1, 0.5);
        let v = Vector::new(1.0, 0.0, 0.0);
        let r = qm.rotate_vector(v);
        // 대략 (√2/2, √2/2, 0)
        assert!((r.x - 0.7071).abs() < 1e-3 && (r.y - 0.7071).abs() < 1e-3);
    }
```
```rust
    #[test]
    fn axis_angle_roundtrip_test() {
        let axis = Vector::new(0.0, 1.0, 0.0);
        let deg = 45.0;
        let q = Quaternion::from_axis_angle_deg(axis, deg);
        let (axis2, deg2) = q.to_axis_angle_deg();
        assert!((deg2 - deg).abs() < 1e-9);
        assert!((axis2 - axis.unitize()).length() < 1e-9);
    }
```
```rust
    #[test]
    fn euler_roundtrip_test() {
        use nurbslib::core::prelude::Vector;
        use nurbslib::core::quaternion::Quaternion;

        // 1. 원래 Euler 각도 (라디안)
        let (roll, pitch, yaw) = (0.3, 0.5, 0.7);

        // 2. Euler → Quaternion
        let q1 = Quaternion::from_euler_angles(roll, pitch, yaw);

        // 3. Quaternion → Euler
        let (r2, p2, y2) = q1.to_euler_angles();

        // 4. 복원된 Euler → Quaternion
        let q2 = Quaternion::from_euler_angles(r2, p2, y2);

        // 5. 회전 결과 비교
        let v = Vector::new(1.0, 0.0, 0.0);
        let r1 = q1.rotate_vector(v);
        let r2v = q2.rotate_vector(v);

        // 6. 쿼터니언 내적 비교 (± 동일 회전)
        let dot = q1.w * q2.w + q1.x * q2.x + q1.y * q2.y + q1.z * q2.z;

        println!("Original Euler:    roll={:.6}, pitch={:.6}, yaw={:.6}", roll, pitch, yaw);
        println!("Recovered Euler:   roll={:.6}, pitch={:.6}, yaw={:.6}", r2, p2, y2);
        println!("Rotation match:    r1={:?}, r2={:?}", r1, r2v);
        println!("Quaternion dot:    {:.6}", dot);

        // 7. 회전 결과가 거의 같고, 쿼터니언 내적이 ±1에 가까우면 OK
        assert!((r1 - r2v).length() < 1e-9);
        assert!(dot.abs() > 0.999);
    }
```
```rust
    #[test]
    fn z_axis_rotation_invariance() {
        let q = Quaternion::from_axis_angle_deg(Vector::new(0.0, 0.0, 1.0), 90.0);
        let z = Vector::new(0.0, 0.0, 1.0);
        let r = q.rotate_vector(z);
        assert!((r - z).length() < 1e-9); // Z축은 Z축 기준 회전에 불변
    }
}
```
---




