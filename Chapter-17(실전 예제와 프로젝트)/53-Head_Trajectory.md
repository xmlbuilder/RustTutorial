# Trajectory
Trajectory calculation from measured accelerations and angular velocities

## Steps for calculating trajectories
- Calculate rotation for each time step that rotates local (measurement coordinate system) to global coordinate system.
- Calculate local accelerations at COG (if they are measured outside of COG).
- Apply g-correction (if g_correction is set to true) to local accelerations at COG.
- Rotate local accelerations at COG to global coordinate system with rotations calculated in step 1.
- Calculate global velocities and trajectory at COG by numerically integrating (trapezoidal rule) global accelerations at COG.

### 📌 Step 1: 회전 계산
- Calculate rotation for each time step that rotates local (measurement coordinate system) to global coordinate system.
- Input: measured angular velocity, initial rotation
- 각속도 
- 적분된 각도 변화량:
- 회전 행렬 업데이트 (소각 근사):
- 3차원 벡터 $\mathbf{v}=[v_x,v_y,v_z]^T$ 에 대해, 그 벡터의 외적을 행렬 곱으로 표현하기 위해 사용하는 것이 스큐-대칭 행렬입니다.  

$$
[\mathbf{v}]_{\times }=\left[ \begin{matrix}0&-v_z&v_y\\ v_z&0&-v_x\\ -v_y&v_x&0\end{matrix}\right]
$$
- 이 행렬은 다음과 같은 성질을 가집니다:

$$
[\mathbf{v}]_{\times }\cdot \mathbf{w}=\mathbf{v}\times \mathbf{w}
$$

- 즉, 외적을 행렬 곱으로 표현한 형태입니다.

- 설명 이미지

![step1](/image/step1.png)

### 📌 Step 2: COG 가속도 보정
- Calculate local accelerations at COG (if they are measured outside of COG).
- Input: measured accelerations, measured angular velocities, offset measurement to COG
- 센서가 COG 외부에 있을 경우, 원심 가속도 보정:  

$$
\begin{aligned}a_x^{\mathrm{COG}}&=a_x+(\omega _y^2+\omega _z^2)\cdot \Delta x\\ a_y^{\mathrm{COG}}&=a_y+(\omega _x^2+\omega _z^2)\cdot \Delta y\\ a_z^{\mathrm{COG}}&=a_z+(\omega _x^2+\omega _y^2)\cdot \Delta z\end{aligned}
$$


- 설명 이미지
![step2](/image/step2.png)

### 📌 Step 3: g 보정
- Apply g-correction (if g_correction is set to true) to local accelerations at COG.
- Input: local accelerations at COG (output step 2)
- 초기 회전 행렬 R_0 기준으로 중력 방향 변화량 보정:

$$
\begin{aligned}a_x'&=a_x+g\cdot (R_k[2,0]-R_0[2,0])\\ a_y'&=a_y+g\cdot (R_k[2,1]-R_0[2,1])\\ a_z'&=a_z+g\cdot (R_k[2,2]-R_0[2,2])\end{aligned}
$$

- 설명 이미지

![step3](/image/step3.png)

### 📌 Step 4: 글로벌 가속도 변환
- Rotate local accelerations at COG to global coordinate system with rotations calculated in step 1.
- Input: g-corrected local accelerations at COG (output step 3)
- 목적: 로컬 좌표계에서 측정된 가속도를 글로벌 좌표계로 변환
- 수식:

$$
\mathbf{a_{\mathnormal{k}}^{\mathrm{global}}}=\mathbf{R_{\mathnormal{k}}}\cdot \mathbf{a_{\mathnormal{k}}^{\mathrm{local}}}
$$

- 여기서:
- $\mathbf{a_{\mathnormal{k}}^{\mathrm{local}}}$: 센서 기준 가속도 벡터
- $\mathbf{R_{\mathnormal{k}}}$: 시간 k에서의 회전 행렬
- $\mathbf{a_{\mathnormal{k}}^{\mathrm{global}}}$: 글로벌 좌표계 기준 가속도

- 설명 이미지

![step4](/image/step4.png)

#### 📌 Step 5: 속도 및 위치 적분
- Calculate global velocities and trajectory at COG by numerically integrating (trapezoidal rule) global accelerations at COG.
- Input: accelerations in global coordinate system (output step 4), initial position, initial velocity
- 속도 적분 (사다리꼴):

$$
\Delta v_k=\frac{\Delta t}{2}(a_k+a_{k+1})
$$

- 위치 적분 (반-명시적):

$$
p_{k+1}=p_k+v_k\cdot \Delta t+\frac{1}{2}\Delta v_k\cdot \Delta t
$$

- 설명 이미지

![step5](/image/step5.png)


## 구현 소스 (Python)
```python
import numpy as np
from scipy.spatial.transform import Rotation as R


def integration_trapezoid(data_list, dt):
    """
    Trapezoidal rule for two consecutive data points.
    https://en.wikipedia.org/wiki/Trapezoidal_rule
    :param data_list: List of arrays data
    :param dt: Time step
    :return: List of "integrated" arrays (length of result arrays is length - 1 of input arrays)
    """
    data_list_result = []
    for data in data_list:
        trapezoid = (data[1:] + data[:-1]) * dt * 0.5
        data_list_result.append(trapezoid)
    return data_list_result


class Trajectory:
    def __init__(self, ac_x_measured, ac_y_measured, ac_z_measured,
                 av_x_measured, av_y_measured, av_z_measured,
                 dt,
                 angle_init, velocity_init, position_init, offset,
                 g=-9.81, g_correction=True, axes_rotations_input='xyz', axes_rotations_output='xyz'):
        """
        Inputs are expected to be in SAE coordinate system. Therefore g is set negative as default (for g correction).
        Other consistent coordinate systems (including g) should work as well.
        :param ac_x_measured: Measured acceleration in x direction
        :param ac_y_measured: Measured acceleration in y direction
        :param ac_z_measured: Measured acceleration in z direction
        :param av_x_measured: Measured angular velocity in x direction
        :param av_y_measured: Measured angular velocity in y direction
        :param av_z_measured: Measured angular velocity in z direction
        :param dt: Time step
        :param angle_init: Initial angle array / list (xyz) in radians
        :param velocity_init: Initial velocity array / list (xyz)
        :param position_init: Initial position (of COG) array / list (xyz)
        :param offset: Vector from COG to acceleration sensor(s) given in coordinate system of measurements
        :param g: Gravitational acceleration
        :param g_correction: Perform g correction or not
        :param axes_rotations_input: Type and sequence of axes for input rotations (see also link and text below)
        :param axes_rotations_output: Type and sequence of axes for output rotations (see also link and text below)
         https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.transform.Rotation.from_euler.html
        The three rotations can either be in a global frame of reference (extrinsic) or in a body centred frame
        of reference (intrinsic), which is attached to, and moves with, the object under rotation.
        Specifies sequence of axes for rotations. Up to 3 characters belonging to the set {‘X’, ‘Y’, ‘Z’} for
        intrinsic rotations, or {‘x’, ‘y’, ‘z’} for extrinsic rotations. Extrinsic and intrinsic rotations cannot
        be mixed in one function call.
        """
        assert len(ac_x_measured) == len(ac_y_measured) == len(ac_z_measured) == len(av_x_measured) == len(
            av_y_measured) == len(av_z_measured)
        assert len(angle_init) == len(velocity_init) == len(position_init) == len(offset) == 3
        self._ac_x_measured = ac_x_measured
        self._ac_y_measured = ac_y_measured
        self._ac_z_measured = ac_z_measured
        self._av_x_measured = av_x_measured
        self._av_y_measured = av_y_measured
        self._av_z_measured = av_z_measured
        self._dt = dt
        self._angle_init = angle_init
        self._velocity_init = velocity_init
        self._position_init = position_init
        self._offset = offset
        self._g = g
        self._g_correction = g_correction
        self._axes_rotations_input = axes_rotations_input
        self._axes_rotations_output = axes_rotations_output
        #
        # List of scipy.spatial.transform Rotations
        self.rotation_list = None
        self.angle_list = None
        self.vel_x_global = None
        self.vel_y_global = None
        self.vel_z_global = None
        self.pos_x_global = None
        self.pos_y_global = None
        self.pos_z_global = None

    def calculate_rotations_and_angles(self):
        """
        Calculate rotations and angles.
        Rotations are written to self.rotation_list.
        Angles are written to self.angle_list.
        :return:
        """
        rotation_list = []
        angle_list = []
        rotation_init = R.from_euler(self._axes_rotations_input, self._angle_init)
        rotation_list.append(rotation_init)
        angle_list.append(rotation_init.as_euler(self._axes_rotations_output))
        d_an_x, d_an_y, d_an_z = integration_trapezoid([self._av_x_measured, self._av_y_measured,
                                                        self._av_z_measured], self._dt)

        for idx in range(len(self._av_x_measured) - 1):
            d_an_x_global, d_an_y_global, d_an_z_global = rotation_list[idx].apply(
                [d_an_x[idx], d_an_y[idx], d_an_z[idx]])
            rotation_matrix = rotation_list[idx].as_matrix()
            for row in range(3):
                rotation_matrix[:, row] += np.cross(np.array([d_an_x_global, d_an_y_global, d_an_z_global]),
                                                    rotation_matrix[:, row])
                # norm of row of rotation_matrix should not be zero
                rotation_matrix[:, row] /= np.linalg.norm(rotation_matrix[:, row])
            rotation = R.from_matrix(rotation_matrix)
            rotation_list.append(rotation)
            angle_list.append(rotation.as_euler(self._axes_rotations_output))
        self.rotation_list = rotation_list
        self.angle_list = angle_list

    def apply_g_correction(self, ac_x, ac_y, ac_z):
        """
        Apply g correction to accelerations.
        :param ac_x: Acceleration in x direction
        :param ac_y: Acceleration in y direction
        :param ac_z: Acceleration in z direction
        :return: g corrected accelerations (x, y, z)
        """
        # Script professor Greimel page 5.
        ac_x_g = np.copy(ac_x)
        ac_y_g = np.copy(ac_y)
        ac_z_g = np.copy(ac_z)
        rotation_matrix_start = self.rotation_list[0].as_matrix()
        for idx, rotation in enumerate(self.rotation_list):
            rotation_matrix = rotation.as_matrix()
            ac_x_g[idx] += self._g * (rotation_matrix[2, 0] - rotation_matrix_start[2, 0])
            ac_y_g[idx] += self._g * (rotation_matrix[2, 1] - rotation_matrix_start[2, 1])
            ac_z_g[idx] += self._g * (rotation_matrix[2, 2] - rotation_matrix_start[2, 2])
        return ac_x_g, ac_y_g, ac_z_g

    def calculate_acceleration_cog_v1(self):
        """
        Calculate accelerations in COG (when they are measured outside of COG).
        :return: Accelerations in COG
        """
        # Script professor Greimel page 22.
        ac_x_cog = self._ac_x_measured + (self._av_y_measured ** 2 + self._av_z_measured ** 2) * self._offset[0]
        ac_y_cog = self._ac_y_measured + (self._av_x_measured ** 2 + self._av_z_measured ** 2) * self._offset[1]
        ac_z_cog = self._ac_z_measured + (self._av_x_measured ** 2 + self._av_y_measured ** 2) * self._offset[2]
        return ac_x_cog, ac_y_cog, ac_z_cog

    def calculate_acceleration_cog_v2(self):
        # Version if each of the acceleration sensors are at different positions.
        # Not yet implemented.
        # Script professor Greimel page 22.
        pass

    def calculate_trajectory(self):
        """
        Calculate trajectory / position and velocity.
        Velocities are written to self.vel_x_global, self.vel_y_global and self.vel_z_global
        Positions are written to self.pos_x_global, self.pos_y_global and self.pos_z_global
        :return:
        """
        self.calculate_rotations_and_angles()
        vel_x_global = np.zeros_like(self._ac_x_measured)
        vel_x_global[0] = self._velocity_init[0]
        vel_y_global = np.zeros_like(self._ac_x_measured)
        vel_y_global[0] = self._velocity_init[1]
        vel_z_global = np.zeros_like(self._ac_x_measured)
        vel_z_global[0] = self._velocity_init[2]
        pos_x_global = np.zeros_like(self._ac_x_measured)
        pos_x_global[0] = self._position_init[0]
        pos_y_global = np.zeros_like(self._ac_x_measured)
        pos_y_global[0] = self._position_init[1]
        pos_z_global = np.zeros_like(self._ac_x_measured)
        pos_z_global[0] = self._position_init[2]
        # accelerations in cog
        ac_x_measured_cog, ac_y_measured_cog, ac_z_measured_cog = self.calculate_acceleration_cog_v1()
        if self._g_correction:
            # g corrected accelerations in cog
            ac_x_measured_cog_g, ac_y_measured_cog_g, ac_z_measured_cog_g = self.apply_g_correction(ac_x_measured_cog,
                                                                                                    ac_y_measured_cog,
                                                                                                    ac_z_measured_cog)
        else:
            ac_x_measured_cog_g, ac_y_measured_cog_g, ac_z_measured_cog_g = ac_x_measured_cog, ac_y_measured_cog, \
                                                                            ac_z_measured_cog
        # (g corrected) accelerations in cog rotated to global coordinate system
        ac_x_global = np.zeros_like(self._ac_x_measured)
        ac_y_global = np.zeros_like(self._ac_x_measured)
        ac_z_global = np.zeros_like(self._ac_x_measured)
        for idx, rotation in enumerate(self.rotation_list):
            ac_x_global[idx], ac_y_global[idx], ac_z_global[idx] = rotation.apply(
                [ac_x_measured_cog_g[idx], ac_y_measured_cog_g[idx], ac_z_measured_cog_g[idx]])
        # delta integrated (g corrected) accelerations in cog rotated to global coordinate system
        d_vel_x_global, d_vel_y_global, d_vel_z_global = integration_trapezoid([ac_x_global, ac_y_global, ac_z_global],
                                                                               self._dt)
        for idx in range(len(self._ac_x_measured) - 1):
            vel_x_global[idx + 1] = vel_x_global[idx] + d_vel_x_global[idx]
            vel_y_global[idx + 1] = vel_y_global[idx] + d_vel_y_global[idx]
            vel_z_global[idx + 1] = vel_z_global[idx] + d_vel_z_global[idx]
            pos_x_global[idx + 1] = pos_x_global[idx] + vel_x_global[idx] * self._dt + 0.5 * d_vel_x_global[
                idx] * self._dt
            pos_y_global[idx + 1] = pos_y_global[idx] + vel_y_global[idx] * self._dt + 0.5 * d_vel_y_global[
                idx] * self._dt
            pos_z_global[idx + 1] = pos_z_global[idx] + vel_z_global[idx] * self._dt + 0.5 * d_vel_z_global[
                idx] * self._dt
        self.vel_x_global = vel_x_global
        self.vel_y_global = vel_y_global
        self.vel_z_global = vel_z_global
        self.pos_x_global = pos_x_global
        self.pos_y_global = pos_y_global
        self.pos_z_global = pos_z_global
```

## result to rust
```python

N = 100
dt = 0.01
t = np.linspace(0, (N - 1) * dt, N)

# 단순한 진동 + 선형 증가
ac_x = 0.1 * np.sin(2 * np.pi * 1.0 * t)
ac_y = 0.2 * np.cos(2 * np.pi * 1.0 * t)
ac_z = 0.0 * t

# 일정한 z축 회전
av_x = np.zeros(N)
av_y = np.zeros(N)
av_z = np.ones(N) * 0.1  # rad/s

# 초기 조건
angle_init = [0.0, 0.0, 0.0]
velocity_init = [0.0, 0.0, 0.0]
position_init = [0.0, 0.0, 0.0]
offset = [0.0, 0.0, 0.0]

traj = Trajectory(ac_x, ac_y, ac_z, av_x, av_y, av_z,
                  dt, angle_init, velocity_init, position_init, offset,
                  g=-9.81, g_correction=True)

traj.calculate_trajectory()

# 결과 저장
np.savez("trajectory_result.npz",
         pos_x=traj.pos_x_global,
         pos_y=traj.pos_y_global,
         pos_z=traj.pos_z_global)

```
---

## ✅ 1. Python → Rust 이식 점검

| 기능 항목                     | Trajectory (Python)             | HeadTrajectory (Rust)             | 비고               |
|------------------------------|----------------------------------|-----------------------------------|--------------------|
| 사다리꼴 적분                | `integration_trapezoid()`       | `integration_trapezoid()`         | 동일 구현          |
| 회전 및 각도 계산            | `calculate_rotations_and_angles()` | `calculate_rotations_and_angles()` | `Rotation3` 사용   |
| 중력 보정                    | `apply_g_correction()`          | `apply_g_correction()`            | 동일 수식 적용     |
| COG 가속도 보정              | `calculate_acceleration_cog_v1()` | `calculate_acceleration_cog_v1()` | 동일 수식 적용     |
| 전체 궤적 계산               | `calculate_trajectory()`        | `calculate_trajectory()`          | 전체 흐름 동일     |

- ✅ 전반적으로 Python 코드가 정확하게 Rust로 이식되었으며, 수치 해석 및 회전 수식도 잘 반영되어 있습니다.


## 🧩 3. 주요 함수 요약
| 함수명                          | 설명                                           |
|--------------------------------|------------------------------------------------|
| integration_trapezoid          | 사다리꼴 적분으로 속도 및 각도 변화량 계산     |
| calculate_rotations_and_angles | 각속도 적분을 통해 회전 행렬 및 Euler 각 계산 |
| propagate_small_angle          | 소각 근사 기반 회전 행렬 업데이트 및 정규화    |
| calculate_acceleration_cog_v1  | 센서 위치 오프셋을 고려한 COG 기준 가속도 계산 |
| apply_g_correction             | 초기 회전 기준으로 중력 보정 수행              |
| calculate_trajectory           | 전체 궤적 계산: 회전, 가속도, 속도, 위치 적분 포함 |


## 소스 코드
```rust
use nalgebra::{Matrix3, Rotation3, Vector3};

/// 단순 사다리꼴 적분: 각 데이터 시퀀스에 대해
/// result[k] = 0.5 * dt * (x[k] + x[k+1]), 길이는 입력-1
fn integration_trapezoid(data_list: &[Vec<f64>], dt: f64) -> Vec<Vec<f64>> {
    let mut out = Vec::with_capacity(data_list.len());
    for data in data_list {
        let mut tr = Vec::with_capacity(data.len().saturating_sub(1));
        for k in 0..data.len().saturating_sub(1) {
            tr.push(0.5 * dt * (data[k] + data[k + 1]));
        }
        out.push(tr);
    }
    out
}
```
```rust
pub struct HeadTrajectory {
    // 측정값 (가속도/각속도)
    ac_x_measured: Vec<f64>,
    ac_y_measured: Vec<f64>,
    ac_z_measured: Vec<f64>,
    av_x_measured: Vec<f64>,
    av_y_measured: Vec<f64>,
    av_z_measured: Vec<f64>,

    dt: f64,

    angle_init: [f64; 3],    // (xyz) rad
    velocity_init: [f64; 3], // (vx, vy, vz)
    position_init: [f64; 3], // (px, py, pz)
    offset: [f64; 3],        // COG->센서 위치(측정 좌표계)

    g: f64,
    g_correction: bool,

    // 현재는 'xyz'만 지원
    axes_rot_in: String,
    axes_rot_out: String,

    // 결과
    pub rotation_list: Vec<Rotation3<f64>>,
    pub angle_list: Vec<[f64; 3]>,
    pub vel_x_global: Vec<f64>,
    pub vel_y_global: Vec<f64>,
    pub vel_z_global: Vec<f64>,
    pub pos_x_global: Vec<f64>,
    pub pos_y_global: Vec<f64>,
    pub pos_z_global: Vec<f64>,
}
```
```rust
impl HeadTrajectory {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ac_x_measured: Vec<f64>,
        ac_y_measured: Vec<f64>,
        ac_z_measured: Vec<f64>,
        av_x_measured: Vec<f64>,
        av_y_measured: Vec<f64>,
        av_z_measured: Vec<f64>,
        dt: f64,
        angle_init: [f64; 3],
        velocity_init: [f64; 3],
        position_init: [f64; 3],
        offset: [f64; 3],
        g: f64,
        g_correction: bool,
        axes_rotations_input: &str,
        axes_rotations_output: &str,
    ) -> Self {
        assert_eq!(ac_x_measured.len(), ac_y_measured.len());
        assert_eq!(ac_x_measured.len(), ac_z_measured.len());
        assert_eq!(ac_x_measured.len(), av_x_measured.len());
        assert_eq!(ac_x_measured.len(), av_y_measured.len());
        assert_eq!(ac_x_measured.len(), av_z_measured.len());

        Self {
            ac_x_measured,
            ac_y_measured,
            ac_z_measured,
            av_x_measured,
            av_y_measured,
            av_z_measured,
            dt,
            angle_init,
            velocity_init,
            position_init,
            offset,
            g,
            g_correction,
            axes_rot_in: axes_rotations_input.to_string(),
            axes_rot_out: axes_rotations_output.to_string(),
            rotation_list: Vec::new(),
            angle_list: Vec::new(),
            vel_x_global: Vec::new(),
            vel_y_global: Vec::new(),
            vel_z_global: Vec::new(),
            pos_x_global: Vec::new(),
            pos_y_global: Vec::new(),
            pos_z_global: Vec::new(),
        }
    }
```
```rust
    fn rot_from_euler_xyz(ang: [f64; 3]) -> Rotation3<f64> {
        Rotation3::from_euler_angles(ang[0], ang[1], ang[2])
    }
```
```rust
    fn euler_xyz_from_rot(r: &Rotation3<f64>) -> [f64; 3] {
        let (rx, ry, rz) = r.euler_angles();
        [rx, ry, rz]
    }
```
```rust
    /// 작은 각도 dθ(글로벌)로 다음 회전을 업데이트: R_{k+1} ≈ (I + [dθ]_x) * R_k
    /// 이후 열(또는 행) 직교정규화.
    fn propagate_small_angle(current: &Rotation3<f64>, dtheta_global: Vector3<f64>) -> Rotation3<f64> {
        let r = current.matrix();
        let skew = |v: Vector3<f64>| -> Matrix3<f64> {
            Matrix3::new(
                0.0, -v.z,  v.y,
                v.z,  0.0, -v.x,
                -v.y,  v.x,  0.0,
            )
        };
        let m_next = (Matrix3::identity() + skew(dtheta_global)) * r;

        // 행(또는 열) 정규화/직교화 (여기서는 열 벡터 기반)
        let mut c0 = m_next.column(0).into_owned();
        let mut c1 = m_next.column(1).into_owned();
        let mut c2 = m_next.column(2).into_owned();

        // Gram-Schmidt 간단 버전
        c0 = c0 / c0.norm();
        c1 = c1 - c0 * c0.dot(&c1);
        c1 = c1 / c1.norm();
        c2 = c0.cross(&c1);

        Rotation3::from_matrix_unchecked(Matrix3::from_columns(&[c0, c1, c2]))
    }
```
```rust
    /// 회전/오일러 각도 계산 (rotation_list / angle_list 채움)
    pub fn calculate_rotations_and_angles(&mut self) {
        assert!(self.axes_rot_in == "xyz" && self.axes_rot_out == "xyz",
                "현재 구현은 'xyz' 오일러 순서만 지원합니다.");

        let mut rotation_list = Vec::with_capacity(self.av_x_measured.len());
        let mut angle_list = Vec::with_capacity(self.av_x_measured.len());

        let r0 = Self::rot_from_euler_xyz(self.angle_init);
        rotation_list.push(r0);
        angle_list.push(Self::euler_xyz_from_rot(&r0));

        // 각속도 적분 (사다리꼴) → 각 증분(스칼라) 시퀀스
        let d_an = integration_trapezoid(
            &[
                self.av_x_measured.clone(),
                self.av_y_measured.clone(),
                self.av_z_measured.clone(),
            ],
            self.dt,
        );
        let (dax, day, daz) = (&d_an[0], &d_an[1], &d_an[2]);

        // 스텝 전파
        for k in 0..self.av_x_measured.len().saturating_sub(1) {
            // body 측정 각증분을 현재 회전으로 global로 변환
            let d_body = Vector3::new(dax[k], day[k], daz[k]);
            let d_global = rotation_list[k] * d_body;

            // 작은각 근사로 행렬 업데이트 + 정규화
            let r_next = Self::propagate_small_angle(&rotation_list[k], d_global);
            rotation_list.push(r_next);
            angle_list.push(Self::euler_xyz_from_rot(&r_next));
        }

        self.rotation_list = rotation_list;
        self.angle_list = angle_list;
    }
```
```rust
    /// g 보정 (Greimel 스크립트 p.5)
    fn apply_g_correction(
        &self,
        ac_x: &[f64],
        ac_y: &[f64],
        ac_z: &[f64],
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut gx = ac_x.to_vec();
        let mut gy = ac_y.to_vec();
        let mut gz = ac_z.to_vec();

        let r_start = self.rotation_list[0].matrix();
        for (i, r) in self.rotation_list.iter().enumerate() {
            let m = r.matrix();
            gx[i] += self.g * (m[(2, 0)] - r_start[(2, 0)]);
            gy[i] += self.g * (m[(2, 1)] - r_start[(2, 1)]);
            gz[i] += self.g * (m[(2, 2)] - r_start[(2, 2)]);
        }
        (gx, gy, gz)
    }
```
```rust
    /// COG 가속도 (센서가 COG 외부에 있을 때), Greimel p.22 (단순 버전)
    fn calculate_acceleration_cog_v1(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut ax = Vec::with_capacity(self.ac_x_measured.len());
        let mut ay = Vec::with_capacity(self.ac_y_measured.len());
        let mut az = Vec::with_capacity(self.ac_z_measured.len());

        for i in 0..self.ac_x_measured.len() {
            ax.push(self.ac_x_measured[i] + (self.av_y_measured[i].powi(2) + self.av_z_measured[i].powi(2)) * self.offset[0]);
            ay.push(self.ac_y_measured[i] + (self.av_x_measured[i].powi(2) + self.av_z_measured[i].powi(2)) * self.offset[1]);
            az.push(self.ac_z_measured[i] + (self.av_x_measured[i].powi(2) + self.av_y_measured[i].powi(2)) * self.offset[2]);
        }
        (ax, ay, az)
    }
```
```rust
    /// 전체 궤적 계산 (속도/위치 적분)
    pub fn calculate_trajectory(&mut self) {
        self.calculate_rotations_and_angles();

        let n = self.ac_x_measured.len();
        let mut vx = vec![0.0; n];
        let mut vy = vec![0.0; n];
        let mut vz = vec![0.0; n];
        let mut px = vec![0.0; n];
        let mut py = vec![0.0; n];
        let mut pz = vec![0.0; n];

        vx[0] = self.velocity_init[0];
        vy[0] = self.velocity_init[1];
        vz[0] = self.velocity_init[2];
        px[0] = self.position_init[0];
        py[0] = self.position_init[1];
        pz[0] = self.position_init[2];

        // 1) COG 가속도
        let (mut ax_cog, mut ay_cog, mut az_cog) = self.calculate_acceleration_cog_v1();

        // 2) g 보정
        if self.g_correction {
            let (gx, gy, gz) = self.apply_g_correction(&ax_cog, &ay_cog, &az_cog);
            ax_cog = gx; ay_cog = gy; az_cog = gz;
        }

        // 3) 글로벌 좌표계로 회전 적용
        let mut ax_g = vec![0.0; n];
        let mut ay_g = vec![0.0; n];
        let mut az_g = vec![0.0; n];
        for i in 0..n {
            let v = Vector3::new(ax_cog[i], ay_cog[i], az_cog[i]);
            let gvec = self.rotation_list[i] * v;
            ax_g[i] = gvec.x; ay_g[i] = gvec.y; az_g[i] = gvec.z;
        }

        // 4) 가속도 적분(사다리꼴) → Δv, 그리고 위치 적분(반-명시적)
        let dv = integration_trapezoid(&[ax_g.clone(), ay_g.clone(), az_g.clone()], self.dt);
        let (dvx, dvy, dvz) = (&dv[0], &dv[1], &dv[2]);

        for k in 0..n.saturating_sub(1) {
            vx[k + 1] = vx[k] + dvx[k];
            vy[k + 1] = vy[k] + dvy[k];
            vz[k + 1] = vz[k] + dvz[k];

            px[k + 1] = px[k] + vx[k] * self.dt + 0.5 * dvx[k] * self.dt;
            py[k + 1] = py[k] + vy[k] * self.dt + 0.5 * dvy[k] * self.dt;
            pz[k + 1] = pz[k] + vz[k] * self.dt + 0.5 * dvz[k] * self.dt;
        }

        self.vel_x_global = vx;
        self.vel_y_global = vy;
        self.vel_z_global = vz;
        self.pos_x_global = px;
        self.pos_y_global = py;
        self.pos_z_global = pz;
    }
}
```

---

# 테스트

##  테스트 목적
- Python의 Trajectory 클래스와 Rust의 HeadTrajectory 구조체가 동일한 입력에 대해 동일한 결과를 내는지 검증
- 데이터 수가 충분히 많아야 수치 적분의 의미가 있음 → 예: 100개 이상의 샘플

## 📊 테스트 데이터 생성 (공통)
- 샘플 수: 100
- 샘플링 주기: dt = 0.01 (100Hz)
- 가속도: 단순 진동 또는 선형 증가
- 각속도: 일정한 회전 속도 (예: z축 기준 회전)
### Python 예시 (테스트 입력 생성)

```python
import numpy as np

N = 100
dt = 0.01
t = np.linspace(0, (N - 1) * dt, N)

# 단순한 진동 + 선형 증가
ac_x = 0.1 * np.sin(2 * np.pi * 1.0 * t)
ac_y = 0.2 * np.cos(2 * np.pi * 1.0 * t)
ac_z = 0.0 * t

# 일정한 z축 회전
av_x = np.zeros(N)
av_y = np.zeros(N)
av_z = np.ones(N) * 0.1  # rad/s

# 초기 조건
angle_init = [0.0, 0.0, 0.0]
velocity_init = [0.0, 0.0, 0.0]
position_init = [0.0, 0.0, 0.0]
offset = [0.0, 0.0, 0.0]

traj = Trajectory(ac_x, ac_y, ac_z, av_x, av_y, av_z,
                  dt, angle_init, velocity_init, position_init, offset,
                  g=-9.81, g_correction=True)

traj.calculate_trajectory()

# 결과 저장
np.savez("trajectory_result.npz",
         pos_x=traj.pos_x_global,
         pos_y=traj.pos_y_global,
         pos_z=traj.pos_z_global)

```

## 🦀 Rust 테스트 코드 (Python 결과 비교)
```rust
#[test]
fn test_head_trajectory_against_python() {
    use std::fs::File;
    use ndarray_npy::NpzReader;
    use approx::assert_abs_diff_eq;

    // Python 결과 불러오기
    let mut npz = NpzReader::new(File::open("trajectory_result.npz").unwrap()).unwrap();
    let pos_x_py: Vec<f64> = npz.by_name("pos_x.npy").unwrap().to_vec().unwrap();
    let pos_y_py: Vec<f64> = npz.by_name("pos_y.npy").unwrap().to_vec().unwrap();
    let pos_z_py: Vec<f64> = npz.by_name("pos_z.npy").unwrap().to_vec().unwrap();

    // 동일한 입력 생성
    let n = 100;
    let dt = 0.01;
    let t: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
    let ac_x: Vec<f64> = t.iter().map(|&ti| 0.1 * (2.0 * std::f64::consts::PI * ti).sin()).collect();
    let ac_y: Vec<f64> = t.iter().map(|&ti| 0.2 * (2.0 * std::f64::consts::PI * ti).cos()).collect();
    let ac_z = vec![0.0; n];
    let av_x = vec![0.0; n];
    let av_y = vec![0.0; n];
    let av_z = vec![0.1; n];

    let mut traj = HeadTrajectory::new(
        ac_x, ac_y, ac_z, av_x, av_y, av_z,
        dt,
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        -9.81,
        true,
        "xyz",
        "xyz",
    );

    traj.calculate_trajectory();

    // 비교
    for i in 0..n {
        assert_abs_diff_eq!(traj.pos_x_global[i], pos_x_py[i], epsilon = 1e-6);
        assert_abs_diff_eq!(traj.pos_y_global[i], pos_y_py[i], epsilon = 1e-6);
        assert_abs_diff_eq!(traj.pos_z_global[i], pos_z_py[i], epsilon = 1e-6);
    }
}
```

## ✅ 비교 요약
| 항목                         | Python                          | Rust                         | 비고                  |
|------------------------------|----------------------------------|------------------------------|-----------------------|
| 회전 표현                    | `scipy.spatial.transform.Rotation` | `nalgebra::Rotation3`        | 동일한 회전 행렬 기반 |
| 결과 저장 형식               | `.npz`                           | `ndarray-npy`                | NumPy 호환 포맷 사용  |
| 수치 비교 방식               | -                                | `assert_abs_diff_eq!`        | 부동소수점 비교 매크로 |

---


## 📦 ndarray_npy란?
- Rust에서 NumPy 형식의 배열 파일(.npy, .npz)을 읽고 쓸 수 있게 해주는 크레이트입니다.
- NumPy는 Python에서 과학 계산에 널리 쓰이는 배열 라이브러리인데, Rust에서도 그 데이터를 직접 다룰 수 있게 해줍니다.

## 🧾 NpzReader란?
- ndarray_npy 크레이트에 포함된 구조체
- .npz는 여러 .npy 배열을 압축한 파일인데, NpzReader는 이 파일을 열고 내부 배열을 읽는 역할을 합니다.

### 예시 코드
```rust
use std::fs::File;
use ndarray_npy::NpzReader;

let file = File::open("trajectory_result.npz").unwrap();
let mut npz = NpzReader::new(file).unwrap();

// 특정 배열 불러오기
let pos_x: ndarray::Array1<f64> = npz.by_name("pos_x.npy").unwrap();
```


## ✅ 언제 쓰면 좋을까?
| 상황 또는 목적                          | Python                    | Rust                      |
|----------------------------------------|---------------------------|---------------------------|
| NumPy 배열 저장/불러오기                | `.npy`, `.npz`            | `ndarray-npy`             |
| Python 결과를 Rust에서 비교/검증할 때   | `np.savez()`              | `NpzReader::by_name()`    |
| 다차원 배열을 공유하거나 테스트할 때     | `numpy.ndarray`           | `ndarray::Array1/2/3`     |
| 부동소수점 수치 비교                    | `np.allclose()`           | `assert_abs_diff_eq!`     |

---

## 📊 Python에서 사용되는 입력 데이터
### 1. 측정된 가속도 (Acceleration)
- ac_x_measured: x축 방향 가속도
- ac_y_measured: y축 방향 가속도
- ac_z_measured: z축 방향 가속도
예: 센서가 머리의 움직임을 측정할 때, 각 축 방향으로 얼마나 빠르게 움직이는지를 나타냄


### 2. 측정된 각속도 (Angular Velocity)
- av_x_measured: x축 회전 속도
- av_y_measured: y축 회전 속도
- av_z_measured: z축 회전 속도
예: 머리가 얼마나 빠르게 회전하고 있는지를 각 축 기준으로 측정


### 3. 시간 정보
- dt: 샘플 간 시간 간격 (예: 0.01초)

### 4. 초기 조건
- angle_init: 초기 회전 각도 (Euler 각, rad)
- velocity_init: 초기 속도 (m/s)
- position_init: 초기 위치 (m)

### 5. 센서 위치 오프셋
- offset: 센서가 COG(질량 중심)에서 얼마나 떨어져 있는지 (측정 좌표계 기준)

### 6. 중력 보정 관련
- g: 중력 가속도 (기본값 -9.81 m/s²)
- g_correction: 중력 보정 여부 (True면 보정 수행)

### 🧮 Python이 계산하는 출력 데이터
- rotation_list: 각 시간 스텝별 회전 행렬
- angle_list: 각 시간 스텝별 Euler 각도
- vel_x_global, vel_y_global, vel_z_global: 글로벌 좌표계 기준 속도
- pos_x_global, pos_y_global, pos_z_global: 글로벌 좌표계 기준 위치

### 📁 저장되는 결과
Python에서는 계산된 궤적 데이터를 .npz 파일로 저장합니다:
```python
np.savez("trajectory_result.npz",
         pos_x=traj.pos_x_global,
         pos_y=traj.pos_y_global,
         pos_z=traj.pos_z_global)

```

---

## ✅ C++ 이식 검토 요약

| 항목                         | Python / Rust                      | C++ (CalcHeadTrajectory)       | 비고             |
|------------------------------|------------------------------------|--------------------------------|------------------|
| 사다리꼴 적분                | `integration_trapezoid()`          | `integrateTrapezoid()`         | 동일 구현        |
| 회전 및 각도 계산            | `calculate_rotations_and_angles()` | `calculateRotationsAndAngles()`| 동일 수식 적용   |
| COG 가속도 보정              | `calculate_acceleration_cog_v1()`  | `calculateAccelerationCOG_v1()`| 동일 수식 적용   |
| 중력 보정                    | `apply_g_correction()`             | `applyGCorrection()`           | 동일 수식 적용   |
| 회전 적용                    | `rotation.apply(acc)`              | `R * acc_local`                | 동일 의미        |
| 결과 비교 또는 출력          | `.npz` + `assert_eq!`              | `std::cout`                    | 수동 비교 방식   |


## 소스 코드
```cpp
#ifndef CALCHEADTRAJECTORY_H
#define CALCHEADTRAJECTORY_H

#include <Eigen/Dense>
#include <Eigen/Geometry>

class CalcHeadTrajectory
{

public:
    CalcHeadTrajectory(
        const std::vector<double>& ac_x,
        const std::vector<double>& ac_y,
        const std::vector<double>& ac_z,
        const std::vector<double>& av_x,
        const std::vector<double>& av_y,
        const std::vector<double>& av_z,
        double dt,
        const Eigen::Vector3d& angle_init,
        const Eigen::Vector3d& velocity_init,
        const Eigen::Vector3d& position_init,
        const Eigen::Vector3d& offset,
        double g = -9810.0,
        bool g_correction = true);

    void calculateRotationsAndAngles();

    void calculateAccelerationCOG_v1(
        std::vector<double>& ac_x_cog,
        std::vector<double>& ac_y_cog,
        std::vector<double>& ac_z_cog);

    void applyGCorrection(
        std::vector<double>& ac_x,
        std::vector<double>& ac_y,
        std::vector<double>& ac_z);

    void calculateTrajectory();

    const std::vector<Eigen::Vector3d>& getPositions() const;
    const std::vector<Eigen::Vector3d>& getVelocities() const;
    const std::vector<Eigen::Vector3d>& getAngleList() const;

private:
    std::vector<double> ac_x_measured, ac_y_measured, ac_z_measured;
    std::vector<double> av_x_measured, av_y_measured, av_z_measured;
    double dt;
    Eigen::Vector3d angle_init;
    Eigen::Vector3d velocity_init;
    Eigen::Vector3d position_init;
    Eigen::Vector3d offset;
    double g;
    bool g_correction;

    std::vector<Eigen::Matrix3d> rotation_list;
    std::vector<Eigen::Vector3d> angle_list;
    std::vector<Eigen::Vector3d> velocities;
    std::vector<Eigen::Vector3d> positions;

    std::vector<double> integrateTrapezoid(
        const std::vector<double>& data,
        double dt);

    std::vector<double> extractComponent(
        const std::vector<Eigen::Vector3d>& vecs,
        int axis);
};


// Trapezoidal integration for scalar vector
static std::vector<double> integrateTrapezoid1D(const std::vector<double>& data, double dt) {
    std::vector<double> result(data.size(), 0.0);
    for (size_t i = 0; i < data.size() - 1; ++i) {
        result[i] = 0.5 * (data[i] + data[i + 1]) * dt;
    }
    return result;
}

// 누적합
static std::vector<double> cumulativeSum(const std::vector<double>& data) {
    std::vector<double> result(data.size(), 0.0);
    if (!data.empty()) result[0] = data[0];
    for (size_t i = 1; i < data.size(); ++i) {
        result[i] = result[i - 1] + data[i];
    }
    return result;
}
#endif // CALCHEADTRAJECTORY_H
```
```cpp
#include "calcheadtrajectory.h"
#include <iostream>

CalcHeadTrajectory::CalcHeadTrajectory(
    const std::vector<double>& ac_x,
    const std::vector<double>& ac_y,
    const std::vector<double>& ac_z,
    const std::vector<double>& av_x,
    const std::vector<double>& av_y,
    const std::vector<double>& av_z,
    double dt,
    const Eigen::Vector3d& angle_init,
    const Eigen::Vector3d& velocity_init,
    const Eigen::Vector3d& position_init,
    const Eigen::Vector3d& offset,
    double g,
    bool g_correction)
    : ac_x_measured(ac_x), ac_y_measured(ac_y), ac_z_measured(ac_z),
        av_x_measured(av_x), av_y_measured(av_y), av_z_measured(av_z),
        dt(dt), angle_init(angle_init), velocity_init(velocity_init),
        position_init(position_init), offset(offset),
        g(g), g_correction(g_correction) {

}
```
```cpp
void CalcHeadTrajectory::calculateRotationsAndAngles() {
    Eigen::Matrix3d rotation_init =
        Eigen::AngleAxisd(angle_init[0], Eigen::Vector3d::UnitX()) *
        Eigen::AngleAxisd(angle_init[1], Eigen::Vector3d::UnitY()) *
        Eigen::AngleAxisd(angle_init[2], Eigen::Vector3d::UnitZ()).toRotationMatrix();

    rotation_list.push_back(rotation_init);
    angle_list.push_back(angle_init);

    std::vector<double> d_an_x = integrateTrapezoid(av_x_measured, dt);
    std::vector<double> d_an_y = integrateTrapezoid(av_y_measured, dt);
    std::vector<double> d_an_z = integrateTrapezoid(av_z_measured, dt);

    for (size_t idx = 0; idx < d_an_x.size() - 1; ++idx) {
        Eigen::Vector3d d_an_local(d_an_x[idx], d_an_y[idx], d_an_z[idx]);
        Eigen::Matrix3d R_prev = rotation_list[idx];
        Eigen::Vector3d d_an_global = R_prev * d_an_local;

        Eigen::Matrix3d R_new = R_prev;
        for (int row = 0; row < 3; ++row) {
            Eigen::Vector3d col = R_prev.col(row);
            col += d_an_global.cross(col);
            col.normalize();
            R_new.col(row) = col;
        }

        rotation_list.push_back(R_new);
        Eigen::Vector3d euler_angles = R_new.eulerAngles(0, 1, 2);
        angle_list.push_back(euler_angles);
    }
}
```
```cpp
void CalcHeadTrajectory::calculateAccelerationCOG_v1(
    std::vector<double>& ac_x_cog,
    std::vector<double>& ac_y_cog,
    std::vector<double>& ac_z_cog)
{
    size_t N = ac_x_measured.size();
    ac_x_cog.resize(N);
    ac_y_cog.resize(N);
    ac_z_cog.resize(N);

    for (size_t i = 0; i < N; ++i) {
        ac_x_cog[i] = ac_x_measured[i] +
                      (av_y_measured[i] * av_y_measured[i] + av_z_measured[i] * av_z_measured[i]) * offset[0];
        ac_y_cog[i] = ac_y_measured[i] +
                      (av_x_measured[i] * av_x_measured[i] + av_z_measured[i] * av_z_measured[i]) * offset[1];
        ac_z_cog[i] = ac_z_measured[i] +
                      (av_x_measured[i] * av_x_measured[i] + av_y_measured[i] * av_y_measured[i]) * offset[2];
    }
}
```
```cpp
void CalcHeadTrajectory::applyGCorrection(
    std::vector<double>& ac_x,
    std::vector<double>& ac_y,
    std::vector<double>& ac_z)
{
    if (rotation_list.empty()) return;
    Eigen::Matrix3d R_start = rotation_list[0];

    for (size_t idx = 0; idx < rotation_list.size(); ++idx) {
        const Eigen::Matrix3d& R = rotation_list[idx];
        ac_x[idx] += g * (R(2, 0) - R_start(2, 0));
        ac_y[idx] += g * (R(2, 1) - R_start(2, 1));
        ac_z[idx] += g * (R(2, 2) - R_start(2, 2));
    }
}
```
```cpp
void CalcHeadTrajectory::calculateTrajectory() {
    calculateRotationsAndAngles();

    size_t N = ac_x_measured.size();
    velocities.resize(N, Eigen::Vector3d::Zero());
    positions.resize(N, Eigen::Vector3d::Zero());
    velocities[0] = velocity_init;
    positions[0] = position_init;

    std::vector<double> ac_x_cog, ac_y_cog, ac_z_cog;
    calculateAccelerationCOG_v1(ac_x_cog, ac_y_cog, ac_z_cog);

    if (g_correction) {
        applyGCorrection(ac_x_cog, ac_y_cog, ac_z_cog);
    }

    std::vector<Eigen::Vector3d> ac_global(N);
    for (size_t i = 0; i < N; ++i) {
        Eigen::Vector3d ac_local(ac_x_cog[i], ac_y_cog[i], ac_z_cog[i]);
        ac_global[i] = rotation_list[i] * ac_local;
    }

    std::vector<double> d_vx = integrateTrapezoid(extractComponent(ac_global, 0), dt);
    std::vector<double> d_vy = integrateTrapezoid(extractComponent(ac_global, 1), dt);
    std::vector<double> d_vz = integrateTrapezoid(extractComponent(ac_global, 2), dt);

    for (size_t i = 0; i < N - 1; ++i) {
        Eigen::Vector3d dv(d_vx[i], d_vy[i], d_vz[i]);
        velocities[i + 1] = velocities[i] + dv;
        positions[i + 1] = positions[i] + velocities[i] * dt + 0.5 * dv * dt;
    }
}
```
```cpp
const std::vector<Eigen::Vector3d>& CalcHeadTrajectory::getPositions() const { return positions; }
const std::vector<Eigen::Vector3d>& CalcHeadTrajectory::getVelocities() const { return velocities; }
const std::vector<Eigen::Vector3d>& CalcHeadTrajectory::getAngleList() const { return angle_list; }

```
```cpp
std::vector<double> CalcHeadTrajectory::integrateTrapezoid(const std::vector<double>& data, double dt) {
    std::vector<double> result(data.size(), 0.0);
    for (size_t i = 0; i < data.size() - 1; ++i) {
        result[i] = 0.5 * (data[i] + data[i + 1]) * dt;
    }
    return result;
}
```
```cpp
std::vector<double> CalcHeadTrajectory::extractComponent(const std::vector<Eigen::Vector3d>& vecs, int axis) {
    std::vector<double> result(vecs.size());
    for (size_t i = 0; i < vecs.size(); ++i) {
        result[i] = vecs[i][axis];
    }
    return result;
}

```
```cpp
void test()
{
    size_t N = 1000;
    double dt = 0.01;
    double dummy_y_position = 50.0;

    // 예시 입력값
    std::vector<double> acc_x(N, 0.0), acc_y(N, 0.0), acc_z(N, 0.0);
    std::vector<double> vel_x(N, 0.01), vel_y(N, 0.02), vel_z(N, 0.015);
    std::vector<double> b_acc(N, 0.005); // 기준선 가속도
    std::vector<double> time(N);
    for (size_t i = 0; i < N; ++i) time[i] = i * dt;

    Eigen::Vector3d angle_init = Eigen::Vector3d(0, 0, 0);
    Eigen::Vector3d velocity_init = Eigen::Vector3d(0, 0, 0);
    const Eigen::Vector3d& position_init = Eigen::Vector3d(0, 0, 0);
    Eigen::Vector3d offset = Eigen::Vector3d(0, 0, 0);

    CalcHeadTrajectory traj(
        acc_x, acc_y, acc_z,
        vel_x, vel_y, vel_z,
        dt,
        angle_init,
        velocity_init,
        position_init,
        offset,
        -9810.0,
        true
        );

    traj.calculateTrajectory();

    // 궤적 결과
    const auto& positions = traj.getPositions();
    std::vector<double> y_head(N), z_head(N);
    for (size_t i = 0; i < N; ++i) {
        y_head[i] = positions[i][1];
        z_head[i] = positions[i][2];
    }

    // 기준선 적분
    std::vector<double> dvb = integrateTrapezoid1D(b_acc, dt);
    std::vector<double> vel_b = cumulativeSum(dvb);
    vel_b.insert(vel_b.begin(), 0.0); vel_b.pop_back();

    std::vector<double> dpb = integrateTrapezoid1D(vel_b, dt);
    std::vector<double> disp_b = cumulativeSum(dpb);
    disp_b.insert(disp_b.begin(), 0.0); disp_b.pop_back();

    // 비교 및 최대 편차 계산
    size_t L = (std::min)(disp_b.size(), y_head.size());
    std::vector<double> diff(L);
    for (size_t i = 0; i < L; ++i) {
        diff[i] = std::abs(disp_b[i]) - std::abs(y_head[i]);
    }

    auto max_it = std::max_element(diff.begin(), diff.end(), [](double a, double b) {
        return std::abs(a) < std::abs(b);
    });
    size_t idx = std::distance(diff.begin(), max_it);
    double tms = time[idx] * 1000.0;
    double iso_end_time = tms * 1.2;

    // 결과 출력
    std::cout << "Max Excursion (Test Data): " << std::abs(diff[idx]) << " mm @ " << tms << " ms\n";
    std::cout << ">> ISO-18571 End Time (ms): " << iso_end_time << " ms\n";

    // 시각화용 데이터 출력 (예시)
    std::cout << "\nExcursion vs Z Head Position (first 10 points):\n";
    for (size_t i = 0; i < (std::min)(size_t(10), L); ++i) {
        double excursion = -diff[i] + std::abs(dummy_y_position);
        double z = -z_head[i];
        std::cout << "X: " << excursion << " mm, Z: " << z << " mm\n";
    }

}
```

## 🧠 수치 처리 수식 정리
### 1. 회전 행렬 업데이트 (소각 근사)

- C++에서는 col += d_an_global.cross(col); col.normalize();로 구현됨

### 2. COG 보정

$$
a_x^{\mathrm{COG}}=a_x+(\omega _y^2+\omega _z^2)\cdot \Delta x\quad \mathrm{(y,\  z도\  동일)}
$$

### 3. g 보정

$$
a_x'=a_x+g\cdot (R_k[2,0]-R_0[2,0])\quad \mathrm{(y,\  z도\  동일)}
$$


### 4. 글로벌 회전 적용

$$
\mathbf{a_{\mathnormal{k}}^{\mathrm{global}}}=\mathbf{R_{\mathnormal{k}}}\cdot \mathbf{a_{\mathnormal{k}}^{\mathrm{local}}}
$$

- 여기서:
- $\mathbf{a_{\mathnormal{k}}^{\mathrm{local}}}$: 센서 기준 가속도 벡터
- $\mathbf{R_{\mathnormal{k}}}$: 시간 k에서의 회전 행렬
- $\mathbf{a_{\mathnormal{k}}^{\mathrm{global}}}$: 글로벌 좌표계 기준 가속도


### 5. 속도 및 위치 적분

$$
v_{k+1}=v_k+\Delta v_k\quad p_{k+1}=p_k+v_k\cdot \Delta t+\frac{1}{2}\Delta v_k\cdot \Delta t
$$


## 📊 테스트 코드 검토
- test() 함수에서:
- 입력 벡터 생성 (acc_x, vel_x, b_acc 등)
- 기준선 적분 (disp_b)과 실제 궤적 (y_head) 비교
- 최대 편차 및 ISO 기준 시간 계산
- 시각화용 데이터 출력
- ✅ Python에서 .npz로 저장하던 결과를 C++에서는 std::cout으로 직접 출력하는 방식으로 변경되어 목적에 맞게 잘 이식되었습니다.

## ✅ integrateTrapezoid 함수 검토
- ✔ 기능
    - 사다리꼴 적분 (trapezoidal integration)
    - 입력: data[i] 시퀀스, 시간 간격 dt
    - 출력: result[i] = 0.5 * dt * (data[i] + data[i+1])
### ⚠️ 주의점
- result의 마지막 원소는 항상 0.0으로 남습니다. 왜냐하면 루프가 i < data.size() - 1까지만 돌기 때문입니다.
- 이 구현은 **"미분량"** 을 구하는 데 적합합니다 (예: 가속도 → 속도 변화량)

### ✅ 개선 제안 (선택)
- 마지막 원소를 제거하거나, result의 길이를 data.size() - 1로 맞추는 것이 더 명확할 수 있습니다:
```cpp
std::vector<double> result;
result.reserve(data.size() - 1);
for (size_t i = 0; i < data.size() - 1; ++i) {
    result.push_back(0.5 * (data[i] + data[i + 1]) * dt);
}
return result;
```


## ✅ cumulativeSum 함수 검토
### ✔ 기능
    - 누적합 계산
    - result[i] = data[0] + data[1] + ... + data[i]
### ⚠️ 주의점
- result[0] = 0.0으로 시작하고, data[0]는 누적되지 않음
- 이건 의도적으로 **"변화량 누적"**을 표현할 때는 맞지만, 전체 누적합을 원한다면 result[0] = data[0]으로 시작해야 합니다

### ✅ 개선 예시 (전체 누적합을 원할 경우)
```rust
std::vector<double> result(data.size(), 0.0);
if (!data.empty()) result[0] = data[0];
for (size_t i = 1; i < data.size(); ++i) {
    result[i] = result[i - 1] + data[i];
}
return result;
```

## 🧪 결론

| 함수명            | 목적                         | 입력 형태         | 출력 형태         | 비고                     |
|------------------|------------------------------|-------------------|-------------------|--------------------------|
| integrateTrapezoid | 사다리꼴 적분으로 변화량 계산 | `Vec<f64>` + `dt` | `Vec<f64>`        | 마지막 항 처리 주의 필요 |
| cumulativeSum     | 누적합 계산                  | `Vec<f64>`        | `Vec<f64>`        | 초기값 처리 방식 선택 가능 |

---



