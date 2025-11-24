# Viewport

## 🔎 Viewport 모듈 기능 정리
### 1. 기본 구조
- Projection: Parallel / Perspective
- Frustum: left, right, bottom, top, near, far
- ScreenPort: 픽셀 좌표(left, right, bottom, top, near, far)
- Viewport: Camera + Projection + Frustum + ScreenPort + 제약값(min_near_dist, min_near_over_far)

### 2. 주요 함수 및 수식
- Frustum 관련
    - set_frustum(left, right, bottom, top, near, far)
- 조건:

$$
left<right,\quad bottom<top,\quad 0<near<far- → Frustum 설정.
$$

- frustum_aspect()

$$
aspect=\frac{right-left}{top-bottom}
$$

- set_frustum_aspect(aspect)
    - 원하는 aspect에 맞게 width/height 조정.
- ScreenPort 관련
    - set_screen_port(left, right, bottom, top, near, far)
        - 픽셀 좌표 기반 스크린 포트 설정.
- screen_aspect()

$$
aspect=\frac{right-left}{top-bottom}World ↔ Screen 변환
$$

- world_to_screen_scale(frustum_depth)
- Parallel: 단순 비율.
- Perspective:

$$
s=\frac{depth}{near},\quad scale=\frac{screen\_ width}{frustum\_ width\cdot s}
$$

- world_to_screen_scale_at_point(world_point)
- Camera position과 forward 벡터로 depth 계산:

$$
depth=(cam\_ pos-world\_ point)\cdot cam\_ forward
$$

- 위 scale 계산에 적용.
카메라 시야각 / 렌즈
- camera_angle()

$$
angle=\min \left( \arctan \frac{half\_ w}{near},\  \arctan \frac{half\_ h}{near}\right) 
$$

- set_camera_angle(angle)
    -원하는 angle에 맞게 frustum width/height 조정.
- camera_35mm_lens_length()

$$
lens=\frac{near\cdot film\_ r}{view\_ r}
$$

- (film_r = 12mm, view_r = min(half_w, half_h))
    - set_camera_35mm_lens_length(lens_length)

- scale factor로 frustum 크기 조정.
- Near/Far 조정- fit_near_far_to_bbox(bbox)
    - Camera forward 방향으로 bbox corner까지의 depth 계산.
    - min/max depth → near/far로 설정.
    - 보정: near *= 0.9375, far *= 1.0625.

---

## 소스

```rust
use crate::camera::Camera;          // 이미 구현된 camera.rs 사용
use crate::boundingbox::BoundingBox; // 이미 구현된 boundingbox.rs 사용
use crate::math::prelude::Point3D;

/// 투영 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Projection {
    Parallel,
    Perspective,
}
```
```rust
/// 프러스텀 정보
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,
    pub near: f64,
    pub far: f64,
}
```
```rust
/// 스크린 포트 정보 (픽셀 좌표)
#[derive(Debug, Clone, Copy)]
pub struct ScreenPort {
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub top: i32,
    pub near: i32,
    pub far: i32,
}
```
```rust
impl ScreenPort {
    pub fn width(&self) -> i32 {
        let w = self.right - self.left;
        if w >= 0 { w } else { -w }
    }

    pub fn height(&self) -> i32 {
        let h = self.top - self.bottom;
        if h >= 0 { h } else { -h }
    }
}
```
```rust
/// Rhino ON_Viewport를 단순화한 Viewport
///
/// - 카메라는 이미 존재하는 `Camera`를 사용
/// - 여기서는 프러스텀, 스크린포트, 시야각, 렌즈 길이 등을 담당
#[derive(Debug, Clone)]
pub struct Viewport {
    pub camera: Camera,
    pub projection: Projection,

    pub frustum: Option<Frustum>,
    pub screen_port: Option<ScreenPort>,

    /// perspective에서 near/far 제약값 (필요 없다면 나중에 제거 가능)
    pub min_near_dist: f64,
    pub min_near_over_far: f64,
}
```
```rust
impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
```
```rust
impl Viewport {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            projection: Projection::Parallel,
            frustum: None,
            screen_port: None,
            // Rhino 기본값과 비슷하게
            min_near_dist: 1.0e-4,
            min_near_over_far: 1.0e-4,
        }
    }
```
```rust
    // ---------------------------
    // 기본 설정
    // ---------------------------

    pub fn set_projection(&mut self, p: Projection) {
        self.projection = p;
    }
```
```rust
    pub fn set_frustum(
        &mut self,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> bool {
        // Rhino ON_Viewport::SetFrustum 의 조건을 단순화해서 그대로 사용
        if !(left < right
            && bottom < top
            && 0.0 < near
            && near < far
            && far.is_finite())
        {
            return false;
        }

        // 너무 극단적인 perspective near/far는 경고만 남기고, 일단 허용할지 여부는
        // 필요에 따라 조정 가능
        if matches!(self.projection, Projection::Perspective)
            && (near <= 1.0e-8 || far > 1.0001e8 * near)
        {
            // TODO: 필요하다면 log 경고
        }

        self.frustum = Some(Frustum {
            left,
            right,
            bottom,
            top,
            near,
            far,
        });
        true
    }
```
```rust
    pub fn get_frustum(&self) -> Option<Frustum> {
        self.frustum
    }
```
```rust
    pub fn set_screen_port(
        &mut self,
        left: i32,
        right: i32,
        bottom: i32,
        top: i32,
        near: i32,
        far: i32,
    ) -> bool {
        if left == right || bottom == top {
            return false;
        }
        self.screen_port = Some(ScreenPort {
            left,
            right,
            bottom,
            top,
            near,
            far,
        });
        true
    }
```
```rust
    pub fn get_screen_port(&self) -> Option<ScreenPort> {
        self.screen_port
    }
```
```rust
    // ---------------------------
    // Frustum / Screen 비율
    // ---------------------------
    /// frustum aspect = width / height
    pub fn frustum_aspect(&self) -> Option<f64> {
        let f = self.frustum?;
        let w = f.right - f.left;
        let h = f.top - f.bottom;
        if h == 0.0 {
            return None;
        }
        Some(w / h)
    }
```
```rust
    /// 원하는 frustum aspect(width/height)를 맞추도록 frustum을 조정
    /// (카메라 각을 유지하면서 Rhino `SetFrustumAspect`를 단순화)
    pub fn set_frustum_aspect(&mut self, frustum_aspect: f64) -> bool {
        if frustum_aspect <= 0.0 {
            return false;
        }
        let Some(mut f) = self.frustum else {
            return false;
        };

        let mut w = f.right - f.left;
        let mut h = f.top - f.bottom;

        if h.abs() > w.abs() {
            // height가 더 크면 width에 맞추기
            let d = if w >= 0.0 { w.abs() } else { -w.abs() };
            let d = 0.5 * d;
            let c = 0.5 * (f.top + f.bottom);
            f.bottom = c - d;
            f.top = c + d;
            h = f.top - f.bottom;
        } else {
            // width가 더 크면 height에 맞추기
            let d = if h >= 0.0 { h.abs() } else { -h.abs() };
            let d = 0.5 * d;
            let c = 0.5 * (f.left + f.right);
            f.left = c - d;
            f.right = c + d;
            w = f.right - f.left;
        }

        if frustum_aspect > 1.0 {
            // width를 늘림
            let d = 0.5 * w * frustum_aspect;
            let c = 0.5 * (f.left + f.right);
            f.left = c - d;
            f.right = c + d;
        } else if frustum_aspect < 1.0 {
            // height를 늘림
            let d = 0.5 * h / frustum_aspect;
            let c = 0.5 * (f.bottom + f.top);
            f.bottom = c - d;
            f.top = c + d;
        }

        self.frustum = Some(f);
        true
    }
```
```rust
    /// screen aspect = width / height
    pub fn screen_aspect(&self) -> Option<f64> {
        let port = self.screen_port?;
        let w = (port.right - port.left) as f64;
        let h = (port.top - port.bottom) as f64;
        if h == 0.0 {
            return None;
        }
        Some((w / h).abs())
    }
```
```rust
    // ---------------------------
    // World ↔ Screen scale
    // ---------------------------

    /// ON_Viewport::GetWorldToScreenScale(frustum_depth, scale) 의 단순화 버전
    ///
    /// - frustum_depth: 카메라에서의 깊이 (perspective일 때만 사용)
    /// - 반환: Some(scale) 이면 world length * scale = screen pixels
    pub fn world_to_screen_scale(&self, frustum_depth: Option<f64>) -> Option<f64> {
        let f = self.frustum?;
        let port = self.screen_port?;

        // 기본 스케일 계수
        let mut s = 1.0;

        if matches!(self.projection, Projection::Perspective) {
            if let Some(d) = frustum_depth {
                if !(f.near > 0.0) {
                    return None;
                }
                s = d / f.near;
                if !(s.is_finite() && s >= 0.0) {
                    return None;
                }
            }
        }

        let mut fw = (f.right - f.left).abs();
        if !(fw > 0.0) {
            return None;
        }
        fw *= s;

        let sw = (port.right - port.left).abs() as f64;
        if !(sw > 0.0) {
            return None;
        }

        s = sw / fw;
        if !(s.is_finite() && s > 0.0) {
            return None;
        }
        Some(s)
    }
```
```rust
    /// world space의 특정 점에서의 scale (perspective일 때 깊이를 내부에서 계산)
    ///
    /// Camera가 "카메라 위치 + 카메라 Z축" 정보를 제공한다고 가정합니다.
    /// camera.rs API에 맞게 이 부분은 사용자가 약간 손을 봐야 합니다.
    pub fn world_to_screen_scale_at_point(&self, world_point: &Point3D) -> Option<f64> {
        let f = self.frustum?;
        if !matches!(self.projection, Projection::Perspective) {
            return self.world_to_screen_scale(None);
        }

        // 여기서는 camera.rs에 아래와 같은 API가 있다고 가정합니다:
        //
        //   camera.position() -> Point3D
        //   camera.forward()  -> Vector3D  (카메라가 보는 방향, world space)
        //
        // 실제 camera.rs 구조에 맞게 수정해 주세요.
        let cam_pos = self.camera.position();
        let cam_forward = self.camera.forward().normalized();

        let v = Point3D::new(
            cam_pos.x - world_point.x,
            cam_pos.y - world_point.y,
            cam_pos.z - world_point.z,
        );
        let depth = cam_forward.x * v.x + cam_forward.y * v.y + cam_forward.z * v.z;

        if depth <= 0.0 {
            return None; // 카메라 뒤쪽
        }

        // ON_Viewport::GetWorldToScreenScale(world_point) 와 비슷한 로직
        self.world_to_screen_scale(Some(depth))
    }
```
```rust
    // ---------------------------
    // 카메라 시야각 / 렌즈 길이
    // ---------------------------

    /// 전체 대각선이 아니라, Rhino처럼 "가장 작은 각의 절반"을 angle로 사용하는 방식
    pub fn camera_angle(&self) -> Option<f64> {
        let Some(f) = self.frustum else {
            return None;
        };

        let half_w = f.right.abs().max(f.left.abs());
        let half_h = f.top.abs().max(f.bottom.abs());

        if f.near <= 0.0 || !f.near.is_finite() {
            return None;
        }

        let angle_w = (half_w / f.near).atan();
        let angle_h = (half_h / f.near).atan();
        Some(angle_w.min(angle_h))
    }
```
```rust
    pub fn set_camera_angle(&mut self, angle: f64) -> bool {
        if !(angle > 0.0 && angle < 0.5 * std::f64::consts::PI * (1.0 - f64::EPSILON.sqrt())) {
            return false;
        }
        let Some(mut f) = self.frustum else {
            return false;
        };

        let aspect = match self.frustum_aspect() {
            Some(a) if a > 0.0 => a,
            _ => return false,
        };

        let r = f.near * angle.tan();
        let d = r; // 최소 각의 절반에 맞추는 방식
        let (half_w, half_h) = if aspect >= 1.0 {
            // width >= height
            (d * aspect, d)
        } else {
            (d, d / aspect)
        };

        f.left = -half_w;
        f.right = half_w;
        f.bottom = -half_h;
        f.top = half_h;
        self.frustum = Some(f);
        true
    }
```
```rust
    /// 35mm 필름(24x36mm) 기준 렌즈 길이 (mm)
    pub fn camera_35mm_lens_length(&self) -> Option<f64> {
        let f = self.frustum?;
        if f.near <= 0.0 {
            return None;
        }

        let half_w = f.right.abs().max(f.left.abs());
        let half_h = f.top.abs().max(f.bottom.abs());

        // Two-point perspective일 때는 항상 width 기준으로
        let view_r = half_w.min(half_h);
        let film_r = 12.0; // 24mm 높이의 절반

        if view_r <= 0.0 {
            return None;
        }
        Some(f.near * film_r / view_r)
    }
```
```rust
    pub fn set_camera_35mm_lens_length(&mut self, lens_length: f64) -> bool {
        if !(lens_length.is_finite() && lens_length > 0.0) {
            return false;
        }
        let Some(mut f) = self.frustum else {
            return false;
        };
        if f.near <= 0.0 {
            return false;
        }

        let half_w = f.right.abs().max(f.left.abs());
        let half_h = f.top.abs().max(f.bottom.abs());
        let view_r = half_w.min(half_h);
        let film_r = 12.0;

        if view_r <= 0.0 {
            return false;
        }

        let s = (film_r / view_r) * (f.near / lens_length);
        if (s - 1.0).abs() < 1.0e-6 {
            return true;
        }

        f.left *= s;
        f.right *= s;
        f.bottom *= s;
        f.top *= s;
        self.frustum = Some(f);
        true
    }
```
```rust
    // ---------------------------
    // Near/Far 조정 (BBox 기반)
    // ---------------------------

    /// BoundingBox를 전체로 다 보이도록 near/far를 조정
    ///
    /// Rhino `SetFrustumNearFar(box_min, box_max)`를 단순화한 버전입니다.
    /// 카메라의 position/forward 정보가 필요하므로 camera.rs API에 맞게 수정해야 합니다.
    pub fn fit_near_far_to_bbox(&mut self, bbox: &BoundingBox) -> bool {
        let Some(mut f) = self.frustum else {
            return false;
        };

        // camera.rs 에 이런 API가 있다고 가정합니다:
        //   position() -> Point3D
        //   forward()  -> Vector3D (카메라 Z축 반대 방향이 아니라 "보는 방향"이라고 가정)
        let cam_pos = self.camera.position();
        let cam_forward = self.camera.forward().normalized();

        if !bbox.is_valid() {
            return false;
        }

        let corners = bbox.corners(); // [Point3D; 8] 정도라고 가정

        let mut n = 0.0;
        let mut far = 0.0;
        let mut first = true;

        for p in &corners {
            let v = Point3D::new(
                cam_pos.x - p.x,
                cam_pos.y - p.y,
                cam_pos.z - p.z,
            );
            let d = cam_forward.x * v.x + cam_forward.y * v.y + cam_forward.z * v.z;

            if first {
                n = d;
                far = d;
                first = false;
            } else {
                if d < n {
                    n = d;
                }
                if d > far {
                    far = d;
                }
            }
        }

        if !n.is_finite() || !far.is_finite() || far <= 0.0 {
            return false;
        }

        n *= 0.9375;
        far *= 1.0625;

        if n <= 0.0 {
            n = self.min_near_over_far * far;
        }

        if matches!(self.projection, Projection::Perspective) {
            if n < self.min_near_dist {
                n = self.min_near_dist;
            }
        }

        if n <= 0.0 || far <= n {
            return false;
        }

        // 기존 frustum의 width/height는 유지
        f.near = n;
        f.far = far;
        self.frustum = Some(f);
        true
    }
}

```
```rust
use nalgebra::{Matrix4, Vector3, Vector4};

impl Viewport {
    /// Projection 행렬 생성
    pub fn projection_matrix(&self) -> Option<Matrix4<f64>> {
        let f = self.frustum?;
        match self.projection {
            Projection::Parallel => {
                // Orthographic projection
                Some(Matrix4::new_orthographic(
                    f.left, f.right, f.bottom, f.top, f.near, f.far,
                ))
            }
            Projection::Perspective => {
                let aspect = self.frustum_aspect()?;
                let fov = self.camera_angle()?;
                Some(Matrix4::new_perspective(aspect, fov, f.near, f.far))
            }
        }
    }
```
```rust
    /// View 행렬 생성 (카메라 position, forward, up 기반)
    pub fn view_matrix(&self) -> Matrix4<f64> {
        let eye = self.camera.position();
        let target = eye + self.camera.forward();
        let up = self.camera.up();
        Matrix4::look_at_rh(
            &nalgebra::Point3::new(eye.x, eye.y, eye.z),
            &nalgebra::Point3::new(target.x, target.y, target.z),
            &Vector3::new(up.x, up.y, up.z),
        )
    }
```
```rust
    /// World → Screen 좌표 변환
    pub fn world_to_screen_point(&self, world: &Point3D) -> Option<(f64, f64)> {
        let proj = self.projection_matrix()?;
        let view = self.view_matrix();
        let mvp = proj * view;

        let v = mvp * Vector4::new(world.x, world.y, world.z, 1.0);
        if v.w.abs() < f64::EPSILON {
            return None;
        }

        let ndc_x = v.x / v.w;
        let ndc_y = v.y / v.w;

        let port = self.screen_port?;
        let sx = (ndc_x + 1.0) * 0.5 * (port.right - port.left) as f64 + port.left as f64;
        let sy = (1.0 - ndc_y) * 0.5 * (port.top - port.bottom) as f64 + port.bottom as f64;
        Some((sx, sy))
    }
```
```rust
    /// Screen → World Ray (픽킹용)
    pub fn screen_to_world_ray(&self, sx: f64, sy: f64) -> Option<(Point3D, Point3D)> {
        let proj = self.projection_matrix()?;
        let view = self.view_matrix();
        let inv = (proj * view).try_inverse()?;

        let port = self.screen_port?;
        let ndc_x = (2.0 * (sx - port.left as f64) / (port.right - port.left) as f64) - 1.0;
        let ndc_y = 1.0 - (2.0 * (sy - port.bottom as f64) / (port.top - port.bottom) as f64);

        let near = inv * Vector4::new(ndc_x, ndc_y, -1.0, 1.0);
        let far  = inv * Vector4::new(ndc_x, ndc_y,  1.0, 1.0);

        let p_near = Point3D::new(near.x/near.w, near.y/near.w, near.z/near.w);
        let p_far  = Point3D::new(far.x/far.w, far.y/far.w, far.z/far.w);
        Some((p_near, p_far))
    }
}
```

---

## 테스트 코드
```rust
#[cfg(test)]
mod tests_case1 {
    use nurbslib::core::geom::Point3D;
    use nurbslib::core::viewport::{Projection, Viewport};

    fn make_test_viewport() -> Viewport {
        let mut vp = Viewport::new();
        vp.set_projection(Projection::Perspective);
        vp.set_frustum(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
        vp.set_screen_port(0, 800, 0, 600, 0, 1);
        vp
    }
```
```rust
    #[test]
    fn test_frustum_set_get() {
        let mut vp = Viewport::new();
        assert!(vp.set_frustum(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0));
        let f = vp.get_frustum().unwrap();
        assert_eq!(f.left, -1.0);
        assert_eq!(f.right, 1.0);
        assert_eq!(f.near, 0.1);
        assert_eq!(f.far, 100.0);
    }
```
```rust
    #[test]
    fn test_screen_port_set_get() {
        let mut vp = Viewport::new();
        assert!(vp.set_screen_port(0, 800, 0, 600, 0, 1));
        let sp = vp.get_screen_port().unwrap();
        assert_eq!(sp.width(), 800);
        assert_eq!(sp.height(), 600);
    }
```
```rust
    #[test]
    fn test_aspect_ratios() {
        let vp = make_test_viewport();
        let fr_aspect = vp.frustum_aspect().unwrap();
        let sc_aspect = vp.screen_aspect().unwrap();
        assert!((fr_aspect - 1.0).abs() < 1e-12);
        assert!((sc_aspect - (800.0/600.0)).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_projection_and_view_matrix() {
        let vp = make_test_viewport();
        let proj = vp.projection_matrix().unwrap();
        let view = vp.view_matrix();
        // 행렬이 유효한지 확인
        println!("{:?}", proj);
        println!("{:?}", view);
        assert!(proj.determinant().is_finite());
        assert!(view.determinant().is_finite());
    }
```
```rust
    #[test]
    fn test_world_to_screen_point() {
        let vp = make_test_viewport();
        let world = Point3D::new(0.0, 0.0, -1.0);
        if let Some((sx, sy)) = vp.world_to_screen_point(&world) {
            assert!(sx >= 0.0 && sx <= 800.0);
            assert!(sy >= 0.0 && sy <= 600.0);
        } else {
            panic!("world_to_screen_point failed");
        }
    }
```
```rust
    #[test]
    fn test_screen_to_world_ray() {
        let vp = make_test_viewport();
        let (near, far) = vp.screen_to_world_ray(400.0, 300.0).unwrap();
        // 중심 픽셀에서 생성된 Ray가 유효한지 확인
        assert!(near.z.is_finite());
        assert!(far.z.is_finite());
    }
}
```
```rust
#[cfg(test)]
mod tests_viewport2 {
    // tests/viewport_tests.rs

    use nurbslib::core::viewport::{Viewport, Projection};
    use nurbslib::core::prelude::Point3D;

    #[test]
    fn viewport_default_state() {
        let vp = Viewport::new();

        // 기본값 검사
        assert!(matches!(vp.projection, Projection::Parallel));
        assert!(vp.frustum.is_none());
        assert!(vp.screen_port.is_none());
    }
```
```rust
    #[test]
    fn viewport_set_frustum_and_screen_port() {
        let mut vp = Viewport::new();
        vp.set_projection(Projection::Parallel);

        // 대략 [-1,1] x [-1,1], near=1, far=10
        let ok = vp.set_frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0);
        assert!(ok, "set_frustum failed");

        let ok = vp.set_screen_port(0, 800, 0, 600, 0, 1);
        assert!(ok, "set_screen_port failed");

        let fr_aspect = vp.frustum_aspect().expect("frustum aspect");
        let sc_aspect = vp.screen_aspect().expect("screen aspect");

        // frustum 은 2x2 이므로 aspect = 1.0
        assert!((fr_aspect - 1.0).abs() < 1e-12);
        // screen 은 800x600 → 4/3
        assert!((sc_aspect - (4.0 / 3.0)).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn world_to_screen_scale_parallel() {
        let mut vp = Viewport::new();
        vp.set_projection(Projection::Parallel);

        // width = 2.0, height = 2.0
        let ok = vp.set_frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0);
        assert!(ok);

        // screen width = 800
        let ok = vp.set_screen_port(0, 800, 0, 600, 0, 1);
        assert!(ok);

        // parallel에서는 frustum_depth를 쓰지 않음 (None)
        let s = vp
            .world_to_screen_scale(None)
            .expect("world_to_screen_scale failed");

        // world 1.0 단위가 몇 픽셀인지:
        // frustum width = 2 → 800 / 2 = 400
        assert!((s - 400.0).abs() < 1e-10);
    }
```
```rust
    #[test]
    fn world_to_screen_scale_perspective_with_depth() {
        let mut vp = Viewport::new();
        vp.set_projection(Projection::Perspective);

        // width = 2, near=1, far=10
        let ok = vp.set_frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0);
        assert!(ok);
        let ok = vp.set_screen_port(0, 800, 0, 600, 0, 1);
        assert!(ok);

        // depth = near → 평행 투영이랑 동일한 scale
        let s_near = vp
            .world_to_screen_scale(Some(1.0))
            .expect("scale at near failed");
        // depth = 2*near → 같은 물체가 더 멀리 있으니 scale 2배 감소 (대신 여기서는 depth/near 이므로 s_near와 비교)
        let s_far = vp
            .world_to_screen_scale(Some(2.0))
            .expect("scale at 2*near failed");

        // 우리가 정의한 world_to_screen_scale 에서는
        //   effective_width = (right-left) * (depth/near)
        //   scale = screen_width / effective_width
        // → depth=2*near 이면 effective_width 2배, scale 1/2
        assert!((s_far - s_near * 0.5).abs() < 1e-10);
    }
```
```rust
    #[test]
    fn camera_angle_and_lens_length_roundtrip() {
        let mut vp = Viewport::new();
        vp.set_projection(Projection::Perspective);

        // 대략적인 대칭 frustum
        let ok = vp.set_frustum(-1.0, 1.0, -0.75, 0.75, 1.0, 100.0);
        assert!(ok);

        // 1) 현재 frustum에서 시야각 계산
        let angle = vp.camera_angle().expect("camera_angle failed");
        assert!(angle > 0.0 && angle < std::f64::consts::FRAC_PI_2);

        // 2) 그 각도로 다시 세팅 (큰 변화는 없어야 함)
        let ok = vp.set_camera_angle(angle);
        assert!(ok);

        // 3) 35mm 렌즈 길이 계산
        let lens = vp
            .camera_35mm_lens_length()
            .expect("camera_35mm_lens_length failed");
        assert!(lens > 0.0);

        // 4) 같은 렌즈 길이로 다시 세팅했을 때 큰 변화 없어야 함
        let ok = vp.set_camera_35mm_lens_length(lens);
        assert!(ok);
    }
```
```rust
    #[test]
    fn world_to_screen_scale_at_point_parallel_degenerate() {
        let mut vp = Viewport::new();
        vp.set_projection(Projection::Parallel);

        let ok = vp.set_frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0);
        assert!(ok);
        let ok = vp.set_screen_port(0, 800, 0, 600, 0, 1);
        assert!(ok);

        // parallel 모드에서는 깊이를 쓰지 않으므로,
        // world_to_screen_scale_at_point는 world_to_screen_scale(None)과 같은 스케일을 줄 것
        // (내가 작성한 viewport.rs 구현에 맞게 필요 시 수정해서 사용)
        let p = Point3D::new(0.0, 0.0, 0.0);

        // 만약 world_to_screen_scale_at_point 가 perspective 에서만 의미있게 동작하도록
        // 구현되어 있다면, 이 테스트는 그 구현에 맞게 조정해야 함.
        // 지금은 "평행도 동일한 scale"을 가정한 예시.
        let s0 = vp.world_to_screen_scale(None).unwrap();
        let s1 = vp.world_to_screen_scale_at_point(&p).unwrap();
        assert!((s0 - s1).abs() < 1e-10);
    }
}
```

---

