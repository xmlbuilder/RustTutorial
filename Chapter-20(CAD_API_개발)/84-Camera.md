# Camera
##  함수별 수학적 의미와 수식
### 📐 View 관련
- set_look_at(eye, target, up)
    - 카메라 위치 eye, 바라보는 점 target, 기준 벡터 up으로 좌표계를 정의합니다.
    - Forward 벡터:

$$
\mathbf{z}=\frac{\mathbf{eye}-\mathbf{target}}{\| \mathbf{eye}-\mathbf{target}\| }
$$

- Up 벡터를 z에 직교화:

$$
\mathbf{u}=\mathbf{up}-(\mathbf{up}\cdot \mathbf{z})\mathbf{z},\quad \mathbf{u}=\frac{\mathbf{u}}{\| \mathbf{u}\| }
$$

- Right 벡터:

$$
\mathbf{x}=\mathbf{u}\times \mathbf{z}
$$
- View 행렬은 row-major로:

$$
V=\left[ \begin{matrix}x_x&x_y&x_z&-\mathbf{x}\cdot \mathbf{eye}\\ ; \quad y_x&y_y&y_z&-\mathbf{y}\cdot \mathbf{eye}\\ ; \quad z_x&z_y&z_z&-\mathbf{z}\cdot \mathbf{eye}\\ ; \quad 0&0&0&1\end{matrix}\right] 
$$

- update_view_matrix()
    - 위 수식 그대로 카메라 좌표계를 갱신합니다.

### 📐 Projection 관련
- set_perspective(fov_y, aspect, near, far)
    - 수직 FOV = fov_y, 종횡비 = aspect.
    - 투영 행렬:

$$
P=\left[ \begin{matrix}\frac{1}{a\tan (\frac{fov_y}{2})}&0&0&0\\ ; \quad 0&\frac{1}{\tan (\frac{fov_y}{2})}&0&0\\ ; \quad 0&0&\frac{f+n}{n-f}&\frac{2fn}{n-f}\\ ; \quad 0&0&-1&0\end{matrix}\right] 
$$

- set_orthogonal(left,right,bottom,top,near,far)
    - 직교 투영 행렬:

$$
P=\left[ \begin{matrix}\frac{2}{r-l}&0&0&-\frac{r+l}{r-l}\\ ; \quad 0&\frac{2}{t-b}&0&-\frac{t+b}{t-b}\\ ; \quad 0&0&-\frac{2}{f-n}&-\frac{f+n}{f-n}\\ ; \quad 0&0&0&1\end{matrix}\right]
$$


### 📐 이동/조작 관련
- direction()

$$
\mathbf{d}=\frac{\mathbf{target}-\mathbf{eye}}{\| \mathbf{target}-\mathbf{eye}\| }
$$

- distance()

$$
dist=\| \mathbf{target}-\mathbf{eye}\| 
$$

- set_distance(dist)
    - eye를 target에서 dist만큼 떨어진 위치로 이동.
- move_forward(delta)
    - eye와 target을 direction 방향으로 delta만큼 평행 이동.
- pan(dx,dy)
    - right = dir × up
    - upv = right × dir
    - eye, target을 right·dx + upv·dy 만큼 이동.
- dolly_by_factor(f)
    - distance를 배율 f로 조정.
- orbit_around_target(yaw,pitch)
    - eye-target 벡터를 yaw(around up), pitch(around right) 회전.
- Rodrigues 회전 공식 사용:

$$
\mathbf{v}'=\mathbf{v}\cos \theta +(\mathbf{k}\times \mathbf{v})\sin \theta +\mathbf{k}(\mathbf{k}\cdot \mathbf{v})(1-\cos \theta )
$$


### 📐 Project/Unproject
- project_point(world)
    - world → clip → NDC → screen 좌표 변환.
    - depth01 = (ndc_z * 0.5 + 0.5).
- unproject_point(screen, depth01)
    - screen → NDC → clip → world (inv(viewproj) 사용).
- screen_to_ray(sx,sy)
    - near/far unproject 후 방향 벡터 계산.

### 📐 Fit helpers
- fit_from_box_simple(bbox)
    - bounding box를 화면에 맞게 카메라 거리/직교 frustum 조정.
    - 원근: 반지름 / tan(FOV/2).
    - 직교: bbox 크기와 aspect 비율로 half_w, half_h 설정.

## 2️⃣ 수학적 검증
- View 행렬: lookAt 구현은 표준적이며, up 벡터가 forward와 평행할 때 fallback 처리도 있음 → 안정적.
- Projection 행렬: perspective/orthogonal 모두 OpenGL 표준과 동일 → 문제 없음.
- Orbit 회전: Rodrigues 공식 사용 → 정확.
- Project/Unproject: NDC 변환, inv(viewproj) 사용 → 올바름.
- Fit helpers: bbox 반지름 기반 거리 계산은 단순하지만 robust.  
    다만 margin_px를 픽셀 단위로 반영하는 부분은 근사치라서 정확한 field-of-view 기반 margin 계산은 더 정밀하게 개선 가능.
- 👉 큰 수학적 오류는 없음, 다만 margin 처리와 near/far 업데이트는 보수적 근사.

## 3️⃣ 추가 추천 함수
- look_direction(): 현재 카메라 방향 벡터만 반환 (이미 direction() 있음 → alias).
- roll(angle): 카메라 방향 유지한 채 up 벡터를 회전 (카메라 롤링).
- screen_to_ndc(sx,sy,width,height): 스크린 좌표 → NDC 변환 헬퍼.
- world_to_camera(world): world 좌표를 카메라 로컬 좌표계로 변환.
- camera_to_world(local): 카메라 로컬 좌표 → world 변환.
- frustum_corners(): 현재 view+proj에서 near/far plane의 8개 코너 반환 → picking, culling에 유용.
- fit_from_points(points): bbox 대신 점 집합으로부터 카메라 맞춤.

## ✅ 요약:
- 함수별 수학적 의미와 수식은 표준적인 카메라 모델과 일치합니다.
- 수학적으로 큰 문제는 없고 margin 처리만 근사치라 개선 여지 있음.
- 추가로 roll, 좌표계 변환, frustum 코너 추출 같은 함수가 있으면 더 완성도 높은 카메라 유틸리티가 됩니다.


```rust
use crate::math::boundingbox::BoundingBox;
use crate::math::matrix::Matrix4x4;
use crate::math::matrix::matrix4::{mat4_identity, mat4_inverse, mat4_mul, mat4_mul_pt};
use crate::math::prelude::{Point3D, Vector3D};
use crate::math::utils::clamp;
use std::f64::consts::PI;

// ------------------------------------------------------------
// Camera
// ------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub eye: Point3D,
    pub target: Point3D,
    pub up: Vector3D,

    pub fov_y_deg: f64,
    pub aspect: f64,
    pub near_z: f64,
    pub far_z: f64,

    pub is_perspective: bool,

    // Orthogonal params (used when !is_perspective)
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,

    view: Matrix4x4,
    proj: Matrix4x4,
}
```
```rust
impl Default for Camera {
    fn default() -> Self {
        let mut c = Self {
            eye: Point3D::new(0.0, 0.0, 10.0),
            target: Point3D::new(0.0, 0.0, 0.0),
            up: Vector3D::new(0.0, 0.0, 1.0),

            fov_y_deg: 45.0,
            aspect: 1.0,
            near_z: 0.1,
            far_z: 1000.0,

            is_perspective: true,

            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,

            view: mat4_identity(),
            proj: mat4_identity(),
        };
        c.update_view_matrix();
        c.update_projection_matrix();
        c
    }
}
```
```rust
impl Camera {
    pub fn new(eye: Point3D, target: Point3D, up: Vector3D) -> Self {
        let mut c = Self {
            ..Default::default()
        };
        c.set_look_at(eye, target, up);
        c
    }
```
```rust
    #[inline]
    pub fn view_matrix(&self) -> &Matrix4x4 {
        &self.view
    }
```
```rust
    #[inline]
    pub fn proj_matrix(&self) -> &Matrix4x4 {
        &self.proj
    }
```
```rust    
    #[inline]
    pub fn viewproj_matrix(&self) -> Matrix4x4 {
        mat4_mul(&self.proj, &self.view)
    }
```
```rust
    pub fn set_look_at(&mut self, eye: Point3D, target: Point3D, up: Vector3D) {
        self.eye = eye;
        self.target = target;

        // make up orthogonal to forward
        let mut z = Vector3D::new(
            self.eye.x - self.target.x,
            self.eye.y - self.target.y,
            self.eye.z - self.target.z,
        );
        z.normalize();

        let mut u = Vector3D::new(up.x, up.y, up.z);
        u = Vector3D::new(
            u.x - Vector3D::dot(&u, &z) * z.x,
            u.y - Vector3D::dot(&u, &z) * z.y,
            u.z - Vector3D::dot(&u, &z) * z.z,
        );
        u = if u.length() > 0.0 {
            u.unitize()
        } else {
            // fallback axis
            let alt = if z.x.abs() < 0.9 {
                Vector3D::new(1.0, 0.0, 0.0)
            } else {
                Vector3D::new(0.0, 1.0, 0.0)
            };
            let mut uu = Vector3D::new(
                alt.x - Vector3D::dot(&alt, &z) * z.x,
                alt.y - Vector3D::dot(&alt, &z) * z.y,
                alt.z - Vector3D::dot(&alt, &z) * z.z,
            );
            uu.normalize();
            uu
        };

        self.up = u;
        self.update_view_matrix();
    }
```
```rust
    pub fn set_perspective(&mut self, fov_y_deg: f64, aspect: f64, near_z: f64, far_z: f64) {
        self.fov_y_deg = fov_y_deg;
        self.aspect = aspect;
        self.near_z = near_z.max(1e-9);
        self.far_z = far_z.max(self.near_z + 1e-9);
        self.is_perspective = true;
        self.update_projection_matrix();
    }
```
```rust
    pub fn set_orthogonal(
        &mut self,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near_z: f64,
        far_z: f64,
    ) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
        self.near_z = near_z;
        self.far_z = far_z;
        self.is_perspective = false;
        self.update_projection_matrix();
    }
```
```rust
    pub fn direction(&self) -> Vector3D {
        let d = Vector3D::new(
            self.target.x - self.eye.x,
            self.target.y - self.eye.y,
            self.target.z - self.eye.z,
        );
        d.unitize()
    }
```
```rust
    pub fn distance(&self) -> f64 {
        ((self.target.x - self.eye.x).powi(2)
            + (self.target.y - self.eye.y).powi(2)
            + (self.target.z - self.eye.z).powi(2))
        .sqrt()
    }
```
```rust
pub fn set_distance(&mut self, dist: f64) {
        let dist = dist.max(1e-9);
        let dir = self.direction();
        self.eye = Point3D::new(
            self.target.x - dir.x * dist,
            self.target.y - dir.y * dist,
            self.target.z - dir.z * dist,
        );
        self.update_view_matrix();
    }
```
```rust
    pub fn set_location(&mut self, loc: Point3D) {
        let dir = self.direction();
        let dist = self.distance().max(1e-9);
        self.eye = loc;
        self.target = Point3D::new(
            self.eye.x + dir.x * dist,
            self.eye.y + dir.y * dist,
            self.eye.z + dir.z * dist,
        );
        self.update_view_matrix();
    }
```
```rust    
    pub fn set_direction(&mut self, dir: Vector3D) {
        let d = dir.unitize();
        let dist = self.distance().max(1e-9);
        self.target = Point3D::new(
            self.eye.x + d.x * dist,
            self.eye.y + d.y * dist,
            self.eye.z + d.z * dist,
        );

        // re-orthogonal up
        let mut u = Vector3D::new(
            self.up.x - Vector3D::dot(&self.up, &d) * d.x,
            self.up.y - Vector3D::dot(&self.up, &d) * d.y,
            self.up.z - Vector3D::dot(&self.up, &d) * d.z,
        );

        if u.length() == 0.0 {
            let alt = if d.x.abs() < 0.9 {
                Vector3D::new(1.0, 0.0, 0.0)
            } else {
                Vector3D::new(0.0, 1.0, 0.0)
            };
            u = Vector3D::new(
                alt.x - Vector3D::dot(&alt, &d) * d.x,
                alt.y - Vector3D::dot(&alt, &d) * d.y,
                alt.z - Vector3D::dot(&alt, &d) * d.z,
            );
        }
        self.up = u.unitize();
        self.update_view_matrix();
    }
```
```rust    
    pub fn set_up(&mut self, up: Vector3D) {
        let d = self.direction();
        let mut u = Vector3D::new(
            up.x - Vector3D::dot(&up, &d) * d.x,
            up.y - Vector3D::dot(&up, &d) * d.y,
            up.z - Vector3D::dot(&up, &d) * d.z,
        );
        if u.length() == 0.0 {
            // up parallel to dir -> choose a fallback
            let alt = if d.x.abs() < 0.9 {
                Vector3D::new(1.0, 0.0, 0.0)
            } else {
                Vector3D::new(0.0, 1.0, 0.0)
            };
            u = Vector3D::new(
                alt.x - Vector3D::dot(&alt, &d) * d.x,
                alt.y - Vector3D::dot(&alt, &d) * d.y,
                alt.z - Vector3D::dot(&alt, &d) * d.z,
            );
        }
        self.up = u.unitize();
        self.update_view_matrix();
    }
```
```rust
    pub fn move_forward(&mut self, delta: f64) {
        let dir = self.direction();
        self.eye = Point3D::new(
            self.eye.x + dir.x * delta,
            self.eye.y + dir.y * delta,
            self.eye.z + dir.z * delta,
        );
        self.target = Point3D::new(
            self.target.x + dir.x * delta,
            self.target.y + dir.y * delta,
            self.target.z + dir.z * delta,
        );
        self.update_view_matrix();
    }
```
```rust    
    pub fn pan(&mut self, dx: f64, dy: f64) {
        let dir = self.direction();
        let right = Vector3D::cross(&dir, &self.up).unitize();
        let upv = Vector3D::cross(&right, &dir).unitize();
        let delta = Vector3D::new(
            right.x * dx + upv.x * dy,
            right.y * dx + upv.y * dy,
            right.z * dx + upv.z * dy,
        );
        self.eye = Point3D::new(
            self.eye.x + delta.x,
            self.eye.y + delta.y,
            self.eye.z + delta.z,
        );
        self.target = Point3D::new(
            self.target.x + delta.x,
            self.target.y + delta.y,
            self.target.z + delta.z,
        );
        self.update_view_matrix();
    }
```
```rust    
    pub fn dolly_by_factor(&mut self, factor: f64) {
        let f = clamp(factor, 1e-3, 1e3);
        self.set_distance(self.distance() * f);
    }
```
```rust
    pub fn orbit_around_target(&mut self, yaw_deg: f64, pitch_deg: f64) {
        let yaw = yaw_deg * PI / 180.0;
        let pitch = pitch_deg * PI / 180.0;

        let t = self.target;
        let mut v = Vector3D::new(self.eye.x - t.x, self.eye.y - t.y, self.eye.z - t.z);
        let dir = self.direction();
        let right = Vector3D::cross(&dir, &self.up).unitize();

        // yaw around up
        if yaw.abs() > 1e-12 {
            v = rotate_around_axis(&v, &self.up, yaw);
        }
        // pitch around right
        if pitch.abs() > 1e-12 {
            v = rotate_around_axis(&v, &right, pitch);
        }
        self.eye = Point3D::new(t.x + v.x, t.y + v.y, t.z + v.z);
        self.set_direction(Vector3D::new(-v.x, -v.y, -v.z));
    }
```
```rust
    pub fn update_view_matrix(&mut self) {
        // Camera axes
        let mut z = Vector3D::new(
            self.eye.x - self.target.x,
            self.eye.y - self.target.y,
            self.eye.z - self.target.z,
        );
        z.normalize();
        let mut up = Vector3D::new(
            self.up.x - Vector3D::dot(&self.up, &z) * z.x,
            self.up.y - Vector3D::dot(&self.up, &z) * z.y,
            self.up.z - Vector3D::dot(&self.up, &z) * z.z,
        );
        up.normalize();
        let x = Vector3D::cross(&up, &z).unitize();
        let y = Vector3D::cross(&z, &x); // already orthonormal

        // Row-major lookAt
        let ex = -(x.x * self.eye.x + x.y * self.eye.y + x.z * self.eye.z);
        let ey = -(y.x * self.eye.x + y.y * self.eye.y + y.z * self.eye.z);
        let ez = -(z.x * self.eye.x + z.y * self.eye.y + z.z * self.eye.z);

        self.view = [
            [x.x, x.y, x.z, ex],
            [y.x, y.y, y.z, ey],
            [z.x, z.y, z.z, ez],
            [0.0, 0.0, 0.0, 1.0],
        ];
        self.up = up; // keep orthogonalized up
    }
```
```rust
    pub fn update_projection_matrix(&mut self) {
        if self.is_perspective {
            let f = 1.0 / (0.5 * self.fov_y_deg * PI / 180.0).tan();
            let a = if self.aspect > 0.0 { self.aspect } else { 1.0 };
            let n = self.near_z.max(1e-9);
            let fz = self.far_z.max(n + 1e-9);

            self.proj = [
                [f / a, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (fz + n) / (n - fz), (2.0 * fz * n) / (n - fz)],
                [0.0, 0.0, -1.0, 0.0],
            ];
        } else {
            let l = self.left;
            let r = self.right;
            let b = self.bottom;
            let t = self.top;
            let n = self.near_z;
            let fz = self.far_z;
            self.proj = [
                [2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)],
                [0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)],
                [0.0, 0.0, -2.0 / (fz - n), -(fz + n) / (fz - n)],
                [0.0, 0.0, 0.0, 1.0],
            ];
        }
    }
```
```rust
    // -------------------
    // Project / Unproject
    // -------------------
    /// Returns (screen_xy, depth_01). Y grows downward.
    pub fn project_point(
        &self,
        world: Point3D,
        width: i32,
        height: i32,
    ) -> Option<((f64, f64), f64)> {
        if width <= 0 || height <= 0 {
            return None;
        }
        let vp = self.viewproj_matrix();
        let p = [world.x, world.y, world.z, 1.0];
        let clip = mat4_mul_pt(&vp, &p);
        if clip[3].abs() < 1e-12 {
            return None;
        }

        let ndc_x = clip[0] / clip[3];
        let ndc_y = clip[1] / clip[3];
        let ndc_z = clip[2] / clip[3]; // -1..+1

        let sx = (ndc_x * 0.5 + 0.5) * (width as f64);
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * (height as f64); // y-down
        let depth01 = ndc_z * 0.5 + 0.5;
        Some(((sx, sy), depth01))
    }
```
```rust
    /// Unproject screen (px) + depth in [0..1] back to world.
    pub fn unproject_point(
        &self,
        screen: (f64, f64),
        depth01: f64,
        width: i32,
        height: i32,
    ) -> Option<Point3D> {
        if width <= 0 || height <= 0 {
            return None;
        }
        let ndc_x = 2.0 * (screen.0 / (width as f64)) - 1.0;
        let ndc_y = 1.0 - 2.0 * (screen.1 / (height as f64));
        let ndc_z = 2.0 * clamp(depth01, 0.0, 1.0) - 1.0;
        let clip = [ndc_x, ndc_y, ndc_z, 1.0];

        let vp = self.viewproj_matrix();
        let inv = mat4_inverse(&vp)?;
        let world_h = mat4_mul_pt(&inv, &clip);
        if world_h[3].abs() < 1e-12 {
            return None;
        }
        Some(Point3D::new(
            world_h[0] / world_h[3],
            world_h[1] / world_h[3],
            world_h[2] / world_h[3],
        ))
    }
```
```rust
    pub fn screen_to_world(
        &self,
        sx: f64,
        sy: f64,
        z_ndc: f64,
        width: i32,
        height: i32,
    ) -> Option<Point3D> {
        // z_ndc in [0..1] (0=near, 1=far)
        self.unproject_point((sx, sy), z_ndc, width, height)
    }
```
```rust
    pub fn screen_to_ray(
        &self,
        sx: f64,
        sy: f64,
        width: i32,
        height: i32,
    ) -> Option<(Point3D, Vector3D)> {
        let p_near = self.screen_to_world(sx, sy, 0.0, width, height)?;
        let p_far = self.screen_to_world(sx, sy, 1.0, width, height)?;
        let mut dir = Vector3D::new(p_far.x - p_near.x, p_far.y - p_near.y, p_far.z - p_near.z);
        dir.normalize();
        Some((p_near, dir))
    }
```
```rust
    /// Returns camera frame axes (origin=eye, camX, camY, camZ).
    pub fn frame(&self) -> (Point3D, Vector3D, Vector3D, Vector3D) {
        let z = Vector3D::new(
            self.eye.x - self.target.x,
            self.eye.y - self.target.y,
            self.eye.z - self.target.z,
        )
        .unitize();
        let x = Vector3D::cross(&self.up.unitize(), &z).unitize();
        let y = Vector3D::cross(&z, &x);
        (self.eye, x, y, z)
    }
```
```rust
    // -------------------
    // Fit helpers (simple)
    // -------------------

    /// Simple & robust 'zoom to extents' for both perspective & orthogonal.
    /// margin_px is treated approximately (by inflating radius).
    pub fn fit_from_box_simple(
        &mut self,
        bbox: &BoundingBox,
        vp_w: i32,
        vp_h: i32,
        margin_px: i32,
    ) -> bool {
        if !bbox.is_valid() || vp_w <= 0 || vp_h <= 0 {
            return false;
        }
        let c = bbox.center();
        let diag = bbox.diagonal();
        let radius = 0.5 * (diag.x * diag.x + diag.y * diag.y + diag.z * diag.z).sqrt();

        if self.is_perspective {
            let half_fov = 0.5 * self.fov_y_deg * PI / 180.0;
            let aspect = if self.aspect > 0.0 {
                self.aspect
            } else {
                (vp_w as f64) / (vp_h as f64).max(1.0)
            };
            let fov_y_half_tan = half_fov.tan();
            let fov_x_half_tan = fov_y_half_tan * aspect;

            // pick the larger requirement among x/y
            let dist_y = radius / (fov_y_half_tan.max(1e-9));
            let dist_x = radius / (fov_x_half_tan.max(1e-9));
            let mut dist = dist_x.max(dist_y);

            // inflate by margin
            if margin_px > 0 {
                let sx = (vp_w as f64) / ((vp_w - 2 * margin_px).max(1) as f64);
                let sy = (vp_h as f64) / ((vp_h - 2 * margin_px).max(1) as f64);
                dist *= sx.max(sy);
            }

            self.target = c;
            self.set_distance(dist.max(1e-6));
            self.update_near_far_from_box(bbox);
        } else {
            // Ortho: set frustum so the box fits
            let aspect = if self.aspect > 0.0 {
                self.aspect
            } else {
                (vp_w as f64) / (vp_h as f64).max(1.0)
            };
            let rx = 0.5 * (bbox.max.x - bbox.min.x).abs().max(1e-9);
            let ry = 0.5 * (bbox.max.y - bbox.min.y).abs().max(1e-9);

            let mut half_h = (ry).max(rx / aspect);
            if margin_px > 0 {
                let sy = (vp_h as f64) / ((vp_h - 2 * margin_px).max(1) as f64);
                half_h *= sy;
            }
            let half_w = half_h * aspect;
            self.left = -half_w;
            self.right = half_w;
            self.bottom = -half_h;
            self.top = half_h;
            self.target = c;
            // keep distance
            self.update_projection_matrix();
            self.update_near_far_from_box(bbox);
        }
        true
    }
```
```rust
    pub fn update_near_far_from_box(&mut self, bbox: &BoundingBox) {
        // Use direction to measure depth range relative to target
        let dir = self.direction();
        let mut minz = f64::INFINITY;
        let mut maxz = -f64::INFINITY;
        for p in bbox.corners().iter() {
            let v = Vector3D::new(
                p.x - self.target.x,
                p.y - self.target.y,
                p.z - self.target.z,
            );
            let z = Vector3D::dot(&v, &dir);
            if z < minz {
                minz = z;
            }
            if z > maxz {
                maxz = z;
            }
        }
        if self.is_perspective {
            let near_d = (self.distance() + minz - 1.0).max(1e-4);
            let far_d = (self.distance() + maxz + 1.0).max(near_d + 10.0);
            self.near_z = near_d;
            self.far_z = far_d;
            self.update_projection_matrix();
        } else {
            self.near_z = (minz - 100.0).min(-1e5);
            self.far_z = (maxz + 100.0).max(1e5);
            self.update_projection_matrix();
        }
    }
}
```
```rust
// Rotate vector v around axis (unit not required) by angle (rad)
fn rotate_around_axis(v: &Vector3D, axis: &Vector3D, angle: f64) -> Vector3D {
    let k = axis.unitize();
    let c = angle.cos();
    let s = angle.sin();
    // Rodrigues
    let dot_kv = v.x * k.x + v.y * k.y + v.z * k.z;
    Vector3D::new(
        v.x * c + (k.y * v.z - k.z * v.y) * s + k.x * dot_kv * (1.0 - c),
        v.y * c + (k.z * v.x - k.x * v.z) * s + k.y * dot_kv * (1.0 - c),
        v.z * c + (k.x * v.y - k.y * v.x) * s + k.z * dot_kv * (1.0 - c),
    )
}
```

## 추가 코드 구현
앞서 추천드린 7가지 카메라 유틸리티 함수들을 수식과 함께 Rust 코드로 구현.  
기존 Camera 구조체에 그대로 추가할 수 있는 형태입니다.

### 1. look_direction()
- 👉 이미 direction()이 있지만 alias로 제공하면 직관적입니다.

$$
\mathbf{d}=\frac{\mathbf{target}-\mathbf{eye}}{\| \mathbf{target}-\mathbf{eye}\| }
$$

```rust
impl Camera {
    pub fn look_direction(&self) -> Vector3D {
        self.direction()
    }
}
```


### 2. roll(angle_deg)
- 👉 카메라 방향 벡터는 유지하고, up 벡터를 roll 회전.
- 수식: Rodrigues 회전

$$
\mathbf{u}'=\mathbf{u}\cos \theta +(\mathbf{d}\times \mathbf{u})\sin \theta +\mathbf{d}(\mathbf{d}\cdot \mathbf{u})(1-\cos \theta )
$$

```rust
impl Camera {
    pub fn roll(&mut self, angle_deg: f64) {
        let angle = angle_deg.to_radians();
        let dir = self.direction();
        self.up = rotate_around_axis(&self.up, &dir, angle);
        self.update_view_matrix();
    }
}
```


### 3. screen_to_ndc(sx, sy, width, height)
- 👉 스크린 좌표 → NDC 좌표 변환.

$$
x_{ndc}=2\frac{s_x}{W}-1,\quad y_{ndc}=1-2\frac{s_y}{H}
$$

```rust
impl Camera {
    pub fn screen_to_ndc(&self, sx: f64, sy: f64, width: i32, height: i32) -> Option<(f64,f64)> {
        if width <= 0 || height <= 0 { return None; }
        let ndc_x = 2.0 * (sx / width as f64) - 1.0;
        let ndc_y = 1.0 - 2.0 * (sy / height as f64);
        Some((ndc_x, ndc_y))
    }
}
```


### 4. world_to_camera(world)
- 👉 world 좌표를 카메라 로컬 좌표계로 변환.

$$
p_{cam}=V\cdot p_{world}
$$

```rust
impl Camera {
    pub fn world_to_camera(&self, world: Point3D) -> Point3D {
        let p = [world.x, world.y, world.z, 1.0];
        let cam = mat4_mul_pt(&self.view, &p);
        Point3D::new(cam[0], cam[1], cam[2])
    }
}
```

### 5. camera_to_world(local)
- 👉 카메라 좌표 → world 좌표.

$$
p_{world}=V^{-1}\cdot p_{cam}
$$

```rust
impl Camera {
    pub fn camera_to_world(&self, local: Point3D) -> Option<Point3D> {
        let inv = mat4_inverse(&self.view)?;
        let p = [local.x, local.y, local.z, 1.0];
        let w = mat4_mul_pt(&inv, &p);
        Some(Point3D::new(w[0]/w[3], w[1]/w[3], w[2]/w[3]))
    }
}
```

### 6. frustum_corners()
- 👉 현재 view+proj에서 near/far plane의 8개 코너 반환.  


NDC 코너 $(-1,-1,-1),(1,-1,-1),\dots ,(1,1,1)$ → unproject.


```rust
impl Camera {
    pub fn frustum_corners(&self, width: i32, height: i32) -> Option<[Point3D; 8]> {
        let ndc = [
            (-1.0,-1.0,-1.0), (1.0,-1.0,-1.0),
            (1.0, 1.0,-1.0), (-1.0, 1.0,-1.0),
            (-1.0,-1.0, 1.0), (1.0,-1.0, 1.0),
            (1.0, 1.0, 1.0), (-1.0, 1.0, 1.0),
        ];
        let vp = self.viewproj_matrix();
        let inv = mat4_inverse(&vp)?;
        let mut out = [Point3D::new(0.0,0.0,0.0);8];
        for (i,(x,y,z)) in ndc.iter().enumerate() {
            let clip = [*x,*y,*z,1.0];
            let w = mat4_mul_pt(&inv,&clip);
            out[i] = Point3D::new(w[0]/w[3], w[1]/w[3], w[2]/w[3]);
        }
        Some(out)
    }
}
```


### 7. fit_from_points(points)
- 👉 점 집합으로부터 bbox를 만들고 fit_from_box_simple 호출.

$$
bbox=\{ \min (x,y,z),\max (x,y,z)\}
$$

```rust
impl Camera {
    pub fn fit_from_points(&mut self, points: &[Point3D], vp_w: i32, vp_h: i32, margin_px: i32) -> bool {
        if points.is_empty() { return false; }
        let mut minx = f64::INFINITY; let mut miny = f64::INFINITY; let mut minz = f64::INFINITY;
        let mut maxx = -f64::INFINITY; let mut maxy = -f64::INFINITY; let mut maxz = -f64::INFINITY;
        for p in points {
            minx = minx.min(p.x); miny = miny.min(p.y); minz = minz.min(p.z);
            maxx = maxx.max(p.x); maxy = maxy.max(p.y); maxz = maxz.max(p.z);
        }
        let bbox = BoundingBox::new(Point3D::new(minx,miny,minz), Point3D::new(maxx,maxy,maxz));
        self.fit_from_box_simple(&bbox, vp_w, vp_h, margin_px)
    }
}
```


## ✅ 요약
- look_direction: 방향 벡터 alias
- roll: 카메라 롤 회전 (up 벡터 회전)
- screen_to_ndc: 스크린 → NDC 변환
- world_to_camera: world → camera 좌표
- camera_to_world: camera → world 좌표
- frustum_corners: 현재 frustum의 8개 코너 world 좌표
- fit_from_points: 점 집합 기반 카메라 맞춤

---

## 🧪 테스트 코드 예시
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::prelude::{Point3D, Vector3D};
    use crate::math::boundingbox::BoundingBox;

    fn make_test_camera() -> Camera {
        Camera::new(
            Point3D::new(0.0, 0.0, 10.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        )
    }
```
```rust
    #[test]
    fn test_look_direction() {
        let cam = make_test_camera();
        let dir = cam.look_direction();
        assert!((dir.z.abs() - 1.0).abs() < 1e-9); // looking toward -Z
    }
```
```rust
    #[test]
    fn test_roll() {
        let mut cam = make_test_camera();
        let before = cam.up;
        cam.roll(90.0);
        let after = cam.up;
        // up 벡터가 회전했는지 확인
        assert!((before.x - after.x).abs() > 1e-6 || (before.y - after.y).abs() > 1e-6);
    }
```
```rust
    #[test]
    fn test_screen_to_ndc() {
        let cam = make_test_camera();
        let ndc = cam.screen_to_ndc(400.0, 300.0, 800, 600).unwrap();
        assert!((ndc.0.abs() <= 1.0) && (ndc.1.abs() <= 1.0));
    }
```
```rust
    #[test]
    fn test_world_camera_transform() {
        let cam = make_test_camera();
        let world = Point3D::new(0.0, 0.0, 0.0);
        let cam_pt = cam.world_to_camera(world);
        // target이 원점이므로 카메라 좌표계에서 z축 방향으로 음수 값이어야 함
        assert!(cam_pt.z < 0.0);

        let back = cam.camera_to_world(cam_pt).unwrap();
        assert!((back.x - world.x).abs() < 1e-6);
        assert!((back.y - world.y).abs() < 1e-6);
        assert!((back.z - world.z).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_frustum_corners() {
        let cam = make_test_camera();
        let corners = cam.frustum_corners(800, 600).unwrap();
        assert_eq!(corners.len(), 8);
        // near/far plane 코너들이 서로 다른 z값을 가져야 함
        let near_z = corners[0].z;
        let far_z = corners[4].z;
        assert!(near_z != far_z);
    }
```
```rust
    #[test]
    fn test_fit_from_points() {
        let mut cam = make_test_camera();
        let pts = vec![
            Point3D::new(-1.0, -1.0, -1.0),
            Point3D::new(1.0, 1.0, 1.0),
        ];
        let ok = cam.fit_from_points(&pts, 800, 600, 10);
        assert!(ok);
        // target이 bbox 중심으로 이동했는지 확인
        assert!((cam.target.x.abs() < 1e-6) && (cam.target.y.abs() < 1e-6));
    }
}
```

## ✅ 테스트 포인트
- look_direction: 카메라 방향 벡터가 올바른지 확인.
- roll: up 벡터가 회전했는지 검증.
- screen_to_ndc: 스크린 좌표가 NDC 범위 [-1,1]로 변환되는지 확인.
- world_to_camera / camera_to_world: 변환이 역함수 관계인지 확인.
- frustum_corners: near/far plane 코너가 올바르게 계산되는지 확인.
- fit_from_points: 점 집합 기반으로 카메라 target이 bbox 중심으로 이동하는지 확인.

---

# NDC

NDC는 Normalized Device Coordinates의 약자입니다.

## 📐 의미
- 3D 그래픽스 파이프라인에서 **클리핑 좌표(clip space)** 를 동차 좌표 w로 나눈 뒤 얻는 좌표계를 말합니다.
- 즉,

$$
(x_{ndc},y_{ndc},z_{ndc})=\left( \frac{x_{clip}}{w},\frac{y_{clip}}{w},\frac{z_{clip}}{w}\right) 
$$

- 이 좌표는 정규화된 범위를 가집니다:
- $x_{ndc},y_{ndc},z_{ndc}\in [-1,+1]$

## 📌 활용
- NDC 공간은 뷰포트 변환 직전 단계입니다.
- 예:
- x_{ndc}=-1 → 화면의 왼쪽
- x_{ndc}=+1 → 화면의 오른쪽
- y_{ndc}=-1 → 화면의 아래쪽
- y_{ndc}=+1 → 화면의 위쪽
- z_{ndc}=-1 → near plane
- z_{ndc}=+1 → far plane

## ✅ 요약:
NDC = Normalized Device Coordinates → 클리핑 후, 화면 좌표로 매핑하기 직전의 정규화된 좌표 공간입니다.

## 📐 단계별 수식
- World → Clip
- World 좌표를 View 행렬과 Projection 행렬로 변환:

$$
p_{clip}=P\cdot V\cdot p_{world}^{(h)}
$$

- 여기서 $p_{world}^{(h)}=(x,y,z,1)$ 은 동차 좌표.
- Clip → NDC (Normalized Device Coordinates)
- w로 나누어 정규화:

$$
(x_{ndc},y_{ndc},z_{ndc})=\left( \frac{x_{clip}}{w},\frac{y_{clip}}{w},\frac{z_{clip}}{w}\right) 
$$

- 범위: $[-1,1].$
- NDC → Screen
- 화면 픽셀 좌표로 변환:

$$
s_x=(x_{ndc}\cdot 0.5+0.5)\cdot W
$$
- 여기서 W,H는 뷰포트 크기.

## ✅ 요약:
- World → Clip: 행렬 변환
- Clip → NDC: w로 나누어 정규화
- NDC → Screen: 픽셀 좌표로 매핑
- 이 과정을 통해 3D 공간의 점이 최종적으로 화면 픽셀 위치에 대응됩니다.

```mermaid
flowchart LR
    A[World Space<br/>(x,y,z,1)] -->|View * Projection| B[Clip Space<br/>(x_clip,y_clip,z_clip,w)]
    B -->|Divide by w| C[NDC Space<br/>(x_ndc,y_ndc,z_ndc) ∈ [-1,1]]
    C -->|Viewport Transform| D[Screen Space<br/>(s_x, s_y, depth)]

    %% Labels with formulas
    B:::formula
    C:::formula
    D:::formula

classDef formula fill=#f9f,stroke=#333,stroke-width=1px;

%% Extra notes
    subgraph Formulas
    note1["World→Clip: p_clip = P·V·p_world"]
    note2["Clip→NDC: (x_ndc,y_ndc,z_ndc) = (x_clip/w, y_clip/w, z_clip/w)"]
    note3["NDC→Screen: s_x = (x_ndc*0.5+0.5)*W, s_y = (1-(y_ndc*0.5+0.5))*H"]
    end

    A -.-> note1
    B -.-> note2
    C -.-> note3

```

## ✅ 설명
- World Space: 원래 3D 좌표 (x,y,z,1)
- Clip Space: View 행렬과 Projection 행렬 적용 후 $(x_{clip},y_{clip},z_{clip},w)$
- NDC Space: w로 나누어 [-1,1] 범위로 정규화
- Screen Space: NDC를 뷰포트 크기 (W,H)에 맞게 픽셀 좌표로 변환

---

