# Euler 변환
Euler 관련 변환(RotationZYX, RotationZYZ, GetYawPitchRoll, GetEulerZYZ, KML Orientation)들의 수학적 의미를 정리.

## 1️⃣ Euler 회전 (ZYX, ZYZ)
### ZYX (Yaw–Pitch–Roll)
- 정의:

$$
R=R_z(\mathrm{yaw})\cdot R_y(\mathrm{pitch})\cdot R_x(\mathrm{roll})
$$

- 의미:
  - 항공/로봇 분야에서 흔히 쓰이는 Yaw–Pitch–Roll 회전.
  - Z축(yaw): 좌우 회전
  - Y축(pitch): 위아래 회전
  - X축(roll): 좌우 기울기
  - 순서: Roll → Pitch → Yaw 순으로 적용.

### ZYZ (Euler ZYZ)
- 정의:

$$
R=R_z(\alpha )\cdot R_y(\beta )\cdot R_z(\gamma )
$$

- 의미:
  - 수학/기하학에서 자주 쓰이는 Euler ZYZ 회전.
  - 두 번의 Z축 회전과 한 번의 Y축 회전으로 모든 3D 회전을 표현 가능.
  - 구면 좌표계와 밀접한 관련.

## 2️⃣ 각 추출 (Yaw–Pitch–Roll, Euler ZYZ)
### Yaw–Pitch–Roll 추출
- 공식:
  - 행렬 M에서
  
$$
\begin{aligned}\mathrm{yaw}=\arctan 2(M_{10},M_{00}) & \mathrm{pitch}=\arcsin (-M_{20}) & \mathrm{roll}=\arctan 2(M_{21},M_{22})\end{aligned}
$$

- 의미: 
  - 회전 행렬을 다시 Yaw–Pitch–Roll 각도로 복원.
  - 단, $pitch = ±90°$ 일 때 짐벌락(gimbal lock) 발생 → yaw와 roll이 구분 불가.
### Euler ZYZ 추출
- 공식:
  - 행렬 M에서

$$
\begin{aligned}\beta =\arccos (M_{22}) & \alpha =\arctan 2(M_{12},M_{02}) & \gamma =\arctan 2(M_{21},-M_{20})\end{aligned}
$$

- 의미:
  - 회전 행렬을 Euler ZYZ 각도로 복원.
  - 역시 특수한 경우(β=0 또는 π)에서는 α와 γ가 구분되지 않음.

## 3️⃣ KML Orientation (Heading–Tilt–Roll)
- KML 정의:
  - Heading: Z축 기준 회전 (지구 위에서 북쪽 기준)
  - Tilt: X축 기준 회전 (동쪽 기준)
  - Roll: Y축 기준 회전 (북쪽 기준)
- 주의:
  - KML은 **시계 방향(clockwise)** 을 양수로 정의 → 일반적인 **우손 좌표계(right-hand rule)** 와 반대.
  - 따라서 변환 시 각도를 부호 반전해야 함.

## 📊 요약
- RotationZYX: 항공/로봇에서 쓰이는 yaw–pitch–roll 회전.
- RotationZYZ: 수학적으로 모든 회전을 표현 가능한 Euler ZYZ 회전.
- GetYawPitchRoll / GetEulerZYZ: 회전 행렬에서 다시 각도를 추출하는 공식. 짐벌락 주의.
- KML Orientation: 구글 KML 규격에 맞춘 heading–tilt–roll. 시계 방향을 양수로 정의하므로 일반 수학적 정의와 부호가 반대.

## 👉 즉, Euler 관련 함수들은 회전 행렬 ↔ Euler 각도 간의 변환을 담당하며,
- ZYX는 실용적(항공, 로봇),
- ZYZ는 수학적(모든 회전 표현),
- KML은 **특수 규격(시계 방향)** 을 따른다는 차이가 있습니다.


## 📄 Rust 변환 코드
```rust
impl Xform {
    /// ZYX 오일러 회전 (yaw, pitch, roll)
    pub fn rotation_zyx(yaw: f64, pitch: f64, roll: f64) -> Self {
        let rx = Xform::rotation_axis(roll, &Vector3D::new(1.0, 0.0, 0.0));
        let ry = Xform::rotation_axis(pitch, &Vector3D::new(0.0, 1.0, 0.0));
        let rz = Xform::rotation_axis(yaw, &Vector3D::new(0.0, 0.0, 1.0));
        rz * ry * rx
    }
```
```rust
    /// ZYZ 오일러 회전 (alpha, beta, gamma)
    pub fn rotation_zyz(alpha: f64, beta: f64, gamma: f64) -> Self {
        let rz1 = Xform::rotation_axis(alpha, &Vector3D::new(0.0, 0.0, 1.0));
        let ry  = Xform::rotation_axis(beta,  &Vector3D::new(0.0, 1.0, 0.0));
        let rz2 = Xform::rotation_axis(gamma, &Vector3D::new(0.0, 0.0, 1.0));
        rz1 * ry * rz2
    }
```
```rust
    /// Yaw-Pitch-Roll 추출
    pub fn get_yaw_pitch_roll(&self) -> Option<(f64, f64, f64)> {
        if !self.is_rotation() {
            return None;
        }
        let m = &self.m;
        let (yaw, pitch, roll);
        if (m[1][0] == 0.0 && m[0][0] == 0.0)
            || (m[2][1] == 0.0 && m[2][2] == 0.0)
            || (m[2][0].abs() >= 1.0)
        {
            pitch = if m[2][0] > 0.0 { -std::f64::consts::FRAC_PI_2 } else { std::f64::consts::FRAC_PI_2 };
            yaw   = ( -m[0][1] ).atan2(m[1][1]);
            roll  = 0.0;
        } else {
            yaw   = m[1][0].atan2(m[0][0]);
            roll  = m[2][1].atan2(m[2][2]);
            pitch = (-m[2][0]).asin();
        }
        Some((yaw, pitch, roll))
    }
```
```rust
    /// Euler ZYZ 추출
    pub fn get_euler_zyz(&self) -> Option<(f64, f64, f64)> {
        if !self.is_rotation() {
            return None;
        }
        let m = &self.m;
        let (alpha, beta, gamma);
        if (m[2][2].abs() >= 1.0)
            || (m[1][2] == 0.0 && m[0][2] == 0.0)
            || (m[2][1] == 0.0 && m[2][0] == 0.0)
        {
            beta  = if m[2][2] > 0.0 { 0.0 } else { std::f64::consts::PI };
            alpha = (-m[0][1]).atan2(m[1][1]);
            gamma = 0.0;
        } else {
            beta  = m[2][2].acos();
            alpha = m[1][2].atan2(m[0][2]);
            gamma = m[2][1].atan2(-m[2][0]);
        }
        Some((alpha, beta, gamma))
    }
```
```rust
    /// KML Orientation (라디안)
    pub fn get_kml_orientation_angles_radians(&self) -> Option<(f64, f64, f64)> {
        if !self.is_rotation() {
            return None;
        }
        let m = &self.m;
        let mut heading = f64::NAN;
        let mut tilt    = f64::NAN;
        let mut roll    = f64::NAN;

        // 간단화 버전: OpenNURBS와 동일한 수학적 처리
        // 여기서는 핵심만 구현 (세부 branch는 필요시 확장)
        let h = m[1][0].atan2(m[0][0]);
        let t = m[2][1].asin();
        let r = (-m[2][0]).atan2(m[2][2]);

        heading = -h;
        if heading < 0.0 { heading += 2.0 * std::f64::consts::PI; }
        tilt = -t;
        roll = -r;

        Some((heading, tilt, roll))
    }
```
```rust
    /// KML Orientation (도 단위)
    pub fn get_kml_orientation_angles_degrees(&self) -> Option<(f64, f64, f64)> {
        self.get_kml_orientation_angles_radians().map(|(h,t,r)| {
            (Self::radians_to_pretty_kml_degrees(h, 0.0),
             Self::radians_to_pretty_kml_degrees(t, -180.0),
             Self::radians_to_pretty_kml_degrees(r, -180.0))
        })
    }
```
```rust
    fn radians_to_pretty_kml_degrees(r: f64, min_degrees: f64) -> f64 {
        let mut d = r.to_degrees();
        let mut f = d.floor();
        if d - f > 0.5 { f += 1.0; }
        let half_sec = 0.5 / (60.0 * 60.0);
        if (d - f).abs() < half_sec { d = f; }
        if d < min_degrees { d += 360.0; }
        if d.abs() < half_sec { d = 0.0; }
        d
    }
```
```rust
    /// KML 각도로부터 회전 변환 생성 (라디안)
    pub fn rotation_from_kml_angles_radians(heading: f64, tilt: f64, roll: f64) -> Self {
        // KML은 시계 방향이므로 부호 반전 필요
        let h = Xform::rotation_axis(-heading, &Vector3D::new(0.0, 0.0, 1.0));
        let t = Xform::rotation_axis(-tilt,    &Vector3D::new(1.0, 0.0, 0.0));
        let r = Xform::rotation_axis(-roll,    &Vector3D::new(0.0, 1.0, 0.0));
        let mut kml = h * t * r;

        // 행렬 값 클린업
        for i in 0..4 {
            for j in 0..4 {
                let x = kml.m[i][j];
                if x.abs() <= 1e-12 {
                    kml.m[i][j] = 0.0;
                } else if (x - 1.0).abs() <= 1e-12 {
                    kml.m[i][j] = 1.0;
                } else if (x + 1.0).abs() <= 1e-12 {
                    kml.m[i][j] = -1.0;
                }
            }
        }
        kml
    }
```
```rust
    /// KML 각도로부터 회전 변환 생성 (도 단위)
    pub fn rotation_from_kml_angles_degrees(heading: f64, tilt: f64, roll: f64) -> Self {
        Self::rotation_from_kml_angles_radians(
            heading.to_radians(),
            tilt.to_radians(),
            roll.to_radians()
        )
    }
}
```
## 📊 설명
- rotation_zyx / rotation_zyz: 오일러 회전 행렬 생성.
- get_yaw_pitch_roll / get_euler_zyz: 행렬에서 오일러 각 추출.
- get_kml_orientation_angles_*: KML 규격에 맞는 heading/tilt/roll 추출. (KML은 시계 방향이므로 부호 반전 필요)
- rotation_from_kml_angles_*: KML 각도로부터 변환 행렬 생성.


---

# Bryant Angle
지금까지는 Euler ZYX, ZYZ 같은 전통적인 오일러 회전 변환을 구현했는데,  
Bryant는 사실상 항공역학에서 쓰이는 Euler-Bryant 각도 체계(일명 Bryant angles 또는 Cardan angles)를 의미합니다.  
이건 Yaw–Pitch–Roll과 유사하지만, 특정 축 순서를 고정해서 3차원 회전을 표현하는 또 다른 방식.  

## 📐 Bryant 각도의 정의
- Bryant 각도는 세 개의 직교 축에 대한 연속 회전으로 정의됩니다.
- 가장 흔한 형태는 X–Y–Z 순서:

$$
R=R_x(\alpha )\cdot R_y(\beta )\cdot R_z(\gamma )
$$
- 여기서:
  - \alpha : X축 회전 (roll)
  - \beta : Y축 회전 (pitch)
  - \gamma : Z축 회전 (yaw)
- 즉, ZYX 오일러와 달리 회전 순서가 X → Y → Z로 고정된 표현입니다.

### 📄 Rust 함수 예시
```rust
impl Xform {
    /// Bryant (Cardan) 각도 회전: X → Y → Z 순서
    pub fn rotation_bryant(alpha: f64, beta: f64, gamma: f64) -> Self {
        let rx = Xform::rotation_axis(alpha, &Vector3D::new(1.0, 0.0, 0.0));
        let ry = Xform::rotation_axis(beta,  &Vector3D::new(0.0, 1.0, 0.0));
        let rz = Xform::rotation_axis(gamma, &Vector3D::new(0.0, 0.0, 1.0));
        rx * ry * rz
    }

    /// Bryant 각도 추출 (X→Y→Z 순서)
    pub fn get_bryant_angles(&self) -> Option<(f64, f64, f64)> {
        if !self.is_rotation() {
            return None;
        }
        let m = &self.m;
        let beta = m[0][2].asin(); // pitch
        let alpha = (-m[1][2]).atan2(m[2][2]); // roll
        let gamma = (-m[0][1]).atan2(m[0][0]); // yaw
        Some((alpha, beta, gamma))
    }
}
```


### 📊 의미
- RotationBryant: X→Y→Z 순서로 회전 행렬을 생성.
- GetBryantAngles: 행렬에서 다시 Bryant 각도를 추출.
- Bryant 각도는 Cardan angles라고도 불리며, 항공역학·로봇공학에서 오일러 각도의 또 다른 표현 방식입니다.
- ZYX 오일러와 달리, Bryant는 고정 축(fixed axes) 회전 순서를 사용합니다.


### 📊 Euler 회전 방식 비교표
| 방식             | 회전 순서         | 축 방향 변화 예시 (ψ=45°, θ=30°, φ=60°) |
|------------------|-------------------|-----------------------------------------|
| Intrinsic ZYX    | Z → Y → X         |       Z↑                                |
|                  |                   |      /                                  |
|                  |                   |     Y→                                  |
|                  |                   |    /                                    |
|                  |                   |   X (몸체 기준으로 회전)                  |

| 방식             | 회전 순서         | 축 방향 변화 예시 (ψ=45°, θ=30°, φ=60°) |
|------------------|-------------------|-----------------------------------------|
| Intrinsic XYZ    | X → Y → Z         |   X→                                    |
|                  |                   |    \                                    |
|                  |                   |     Y↑                                  |
|                  |                   |      \                                  |
|                  |                   |       Z (몸체 기준, 순서 다름)         |

| 방식             | 회전 순서         | 축 방향 변화 예시 (ψ=45°, θ=30°, φ=60°) |
|------------------|-------------------|-----------------------------------------|
| Extrinsic ZYX    | X → Y → Z (월드)  |   X→                                    |
|                  |                   |    ↑Y                                   |
|                  |                   |     Z→ (월드 기준, 축 고정)            |


| 방식                  | 회전 순서                     | 행렬 공식                                      | 특징 / 적용 분야                                  |
|----------------------|-------------------------------|------------------------------------------------|---------------------------------------------------|
| Euler ZYX (Yaw–Pitch–Roll) | Z(야오) → Y(피치) → X(롤)       | R = Rz(ψ) · Ry(θ) · Rx(ϕ)                       | 항공, 로봇, 게임 분야에서 흔히 사용. 직관적. 짐벌락 발생 가능 |
| Euler ZYZ            | Z(α) → Y(β) → Z(γ)             | R = Rz(α) · Ry(β) · Rz(γ)                       | 수학적으로 모든 회전 표현 가능. 구면 좌표계, 물리학, 기하학 |
| Bryant (Cardan) XYZ  | X(α) → Y(β) → Z(γ)             | R = Rx(α) · Ry(β) · Rz(γ)                       | 고정 축 기준 회전. 기계공학, 로봇 팔 제어 등에서 사용       |


📐 각 축 회전 행렬 정의
- X축 회전 (roll, α):

$$
R_x(\alpha )=\left[ \begin{matrix}1&0&0 ;& 0&\cos \alpha &-\sin \alpha ;& 0&\sin \alpha &\cos \alpha \end{matrix}\right]
$$ 

- Y축 회전 (pitch, β):

$$
R_y(\beta )=\left[ \begin{matrix}\cos \beta &0&\sin \beta ;& 0&1&0 ;& -\sin \beta &0&\cos \beta \end{matrix}\right] 
$$

- Z축 회전 (yaw, γ):

$$
R_z(\gamma )=\left[ \begin{matrix}\cos \gamma &-\sin \gamma &0 ;& \sin \gamma &\cos \gamma &0 ;& 0&0&1\end{matrix}\right] 
$$

### ⚠️ 주의사항
- 짐벌락(Gimbal lock): 특정 각도(예: pitch=±90°)에서 두 축이 겹쳐져 자유도가 줄어드는 문제 발생.
- 축 순서 중요: 같은 각도라도 회전 순서가 다르면 결과가 완전히 달라짐.
- 적용 분야 차이:
- ZYX: 직관적, 항공/로봇 자세 표현
- ZYZ: 수학적, 모든 회전 표현 가능
- Bryant: 고정 축 회전, 기계공학적 제어에 적합

- 👉 이렇게 보면, 세 방식은 모두 3차원 회전을 표현하지만 축 순서와 적용 분야가 다르다는 점이 핵심입니다.


## 축 도식 비교 (동일 각도: ψ=45°, θ=30°, φ=60°)
```
Intrinsic ZYX (R = Rz · Ry · Rx)
Start axes:
Z:   ^
     |
     +----> Y
    /
   X (out of screen)

After Rx(φ=60°):
Z:   ^
     |
     +----> Y
   / 
  X' (tilted)

After Ry(θ=30°):
Z:    ^ 
      |
      +----> Y'
    / 
   X'' (yawed)

After Rz(ψ=45°):
Z:     ^ 
       |
       +----> Y''
     / 
    X''' 

Result: body axes rotated; final forward differs strongly from other orders.
```

### Intrinsic XYZ (R = Rx · Ry · Rz)
```
Start axes same.

After Rx(φ=60°):
Y' tilted, Z' tilted around X.

After Ry(θ=30°):
Z'' yawed around new Y', X unchanged.

After Rz(ψ=45°):
X''' and Y''': spun around new Z''.

Result: final axes differ from ZYX due to order change.
```

### Extrinsic ZYX (world-fixed)
```
World axes fixed:
Z^, Y→, X⊙ (out of screen)

Apply Rx_world(φ=60°):
Rotate around world X.

Apply Ry_world(θ=30°):
Rotate around world Y.

Apply Rz_world(ψ=45°):
Rotate around world Z.

Result equals intrinsic ZYX mathematically.
```

![Euler Comparision](/image/euler_comparison.png)



### 핵심 차이 한눈에
- 동일한 각도라도 회전 “순서”와 “축이 몸체 기준인지/월드 기준인지”에 따라 최종 방향이 달라진다.
- Intrinsic ZYX ≡ Extrinsic XYZ, Intrinsic XYZ ≡ Extrinsic ZYX처럼 짝이 있다.
- 시뮬레이션/렌더링에서 축 순서가 다르면 카메라/오브젝트의 최종 자세가 완전히 달라진다.

간단 수식 요약
- Rz(ψ):

$$
\left[ \begin{matrix}\cos \psi &-\sin \psi &0 ; & \sin \psi &\cos \psi &0 ;& 0&0&1\end{matrix}\right] 
$$

- Ry(θ):

$$
\left[ \begin{matrix}\cos \theta &0&\sin \theta ; & 0&1&0 ; & -\sin \theta &0&\cos \theta \end{matrix}\right] 
$$

- Rx(φ):

$$
\left[ \begin{matrix}1&0&0 ; & 0&\cos \phi &-\sin \phi ; & 0&\sin \phi &\cos \phi \end{matrix}\right]
$$

- 이 각도들을 다른 순서로 곱하면 서로 다른 최종 R이 된다.


--- 

# Z축 방향

OpenGL의 좌표계와 Z축 방향에 대한 혼란은 정말 흔한데, 핵심을 정리.

## 🎯 OpenGL 좌표계 기본
- 우손 좌표계 (Right-handed)
- 카메라 좌표계 기준:
  - 원점 (0,0,0) → 카메라 위치
  - –Z 방향 → 카메라가 바라보는 방향 (모니터 안쪽)
  - +Z 방향 → 카메라 뒤쪽 (모니터 밖)
- 즉, OpenGL에서 기본적으로 Z축은 화면 안쪽으로 향합니다.

## 🧭 물체가 Z축이 하늘을 보는 쪽으로 그려지는 이유
- 그건 바로 **모델링 좌표계(Model Space)** 에서 Z축을 위로 설정했기 때문입니다.
- 즉:
  - 모델링 툴 (예: Blender, Maya 등)에서는 Z-up을 쓰는 경우가 많고
  - OpenGL에서는 Y-up이 일반적이지만, 우리가 사용하는 모델의 transform matrix에서 Z축을 위로 돌려놓았기 때문에 그렇게 보이는 것임.

## 🧱 Matrix 조작의 역할
- **행렬(Matrix)** 을 조작해서:
  - 모델의 축 방향을 재배열하거나
  - 회전/스케일/위치를 적용하면
  - 결과적으로 Z축이 위로 향하는 것처럼 보일 수 있어요
- 즉, **있는 그대로 그리면 Z축은 화면 안쪽(–Z 방향)** 으로 향하지만,  
  행렬을 조작하면 Z축이 위로 향하도록 보이게 만들 수 있습니다.

## 📐 예시
```
// 기본적으로 OpenGL은 카메라가 -Z 방향을 바라봄
gl_Position = projection * view * model * vec4(position, 1.0);
```
- 여기서 model 행렬이 Z축을 위로 돌려놓으면
  - 물체가 Z-up처럼 보임
- 하지만 실제로는 카메라는 –Z 방향을 보고 있고,
  - Z축은 화면 안쪽으로 향하고 있음



## ✅ 요약
| 구분        | 좌표계 기준        | Z축 방향         | 특징                          |
|-------------|-------------------|------------------|-------------------------------|
| OpenGL      | 우손(Right-handed) | -Z (화면 안쪽)   | 카메라가 -Z 방향을 바라봄     |
| Unity       | 좌손(Left-handed)  | +Z (앞쪽), Y↑    | Y축이 위(하늘), Z축이 전방    |
| 모델링 툴   | 보통 Z-up          | Z (위쪽)         | Blender/Maya 등은 Z를 위로 둠 |
| 변환 행렬   | 사용자 정의        | 축 재배열 가능   | Matrix 조작으로 방향 맞춤     |

- 👉 그래서 **Z축이 하늘을 보는 쪽으로 그려진다** 는 건 행렬 조작의 결과이고, 있는 그대로 그리면 Z축은 화면 안쪽으로 향하는 게 맞습니다.


---


