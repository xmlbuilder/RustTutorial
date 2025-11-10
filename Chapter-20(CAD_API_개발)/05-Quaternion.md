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
각각의 수학적 의미와 구현을 함께 설명드릴게요.

### ✅ from_euler_angles (XYZ 순서)
```rust
pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
    let (sx, cx) = (0.5 * roll).sin_cos();   // X축 회전
    let (sy, cy) = (0.5 * pitch).sin_cos();  // Y축 회전
    let (sz, cz) = (0.5 * yaw).sin_cos();    // Z축 회전

    Quaternion {
        w: cx * cy * cz - sx * sy * sz,
        x: sx * cy * cz + cx * sy * sz,
        y: cx * sy * cz - sx * cy * sz,
        z: cx * cy * sz + sx * sy * cz,
    }
}
```
#### 📐 수학적 의미
- 회전 순서: X → Y → Z
- 쿼터니언 조합: $q=q_z\cdot q_y\cdot q_x$

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

## 🎯 목적: 쿼터니언을 이용한 벡터 회전
## ✅ 수학적 정의
- 벡터 \vec {v}를 쿼터니언 q로 회전시키는 공식은:

$$
\vec {v}_{\mathrm{rotated}}=q\cdot \vec {v}\cdot q^{-1}
$$

- 여기서 \vec {v}는 쿼터니언으로 표현된 순허수 쿼터니언:

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

