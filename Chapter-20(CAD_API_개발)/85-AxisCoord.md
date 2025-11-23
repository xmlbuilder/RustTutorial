# AxisCoord
AxisCoord 단순한 좌표계 저장용이 아니라 좌표계 간 변환, 미러링, 행렬 생성, 포인트/벡터 변환, 회전/이동까지 다 지원.

## 🎯 용도
### 1. CAD/모델링 시스템의 로컬 좌표계
- 객체마다 고유한 좌표계(Origin + X/Y/Z 축)를 갖고 있을 때
- 예: 기계 부품, 도면 요소, 블록, 인스턴스 등
- TransformPoint, TransformVector, TransformAxis2Placement은
    - 다른 좌표계 기준으로 위치/방향을 변환할 때 사용
### 2. STEP/IGES 등 CAD 파일 포맷 대응
- SetSTEPCanonical은 STEP의 axis2_placement_3d 구조와 거의 동일
- crOrigin, crZAxis, crXRefDirection → STEP의 기준축 정의 방식
- 즉, 이 클래스는 STEP 파일의 좌표계 정보를 받아서 내부적으로 변환하는 데 쓰임
### 3. 미러링/대칭 처리
- MirrorPoint, GetMirrorMatrix는 평면 기준으로 점/좌표계를 대칭시키는 기능
- CAD에서 좌우 대칭 부품 만들거나, 대칭 복사할 때 사용
### 4. 좌표계 기반 트랜스폼 행렬 생성
- GetMatrix, Load4x4 → 4×4 행렬로 변환 가능
- OpenGL/DirectX 렌더링, 또는 내부 트랜스폼 계산에 사용
### 5. 디버깅/시각화용 좌표계 추적
- Dump, Draw, AssertValid → 좌표계 상태를 로그로 출력하거나 시각화
- 디버깅 중 좌표계가 꼬였을 때 확인용

## 🧠 요약하면
CAD/모델링 시스템에서 객체의 로컬 좌표계와 그 변환을 다루기 위한 핵심 유틸리티.

```rust
use crate::math::prelude::{Point3D, Vector3D};
use crate::math::matrix::{Matrix4x4, on_mul_mat4_mat4, on_copy_mat, on_point_project_to_plane};

#[derive(Clone, Debug)]
pub struct AxisCoord {
    pub origin: Point3D,
    pub x_axis: Vector3D,
    pub y_axis: Vector3D,
}
```
```rust
impl AxisCoord {
    pub fn new() -> Self {
        Self {
            origin: Point3D::new(0.0, 0.0, 0.0),
            x_axis: Vector3D::new(1.0, 0.0, 0.0),
            y_axis: Vector3D::new(0.0, 1.0, 0.0),
        }
    }
```
```rust
    pub fn z_axis(&self) -> Vector3D {
        Vector3D::cross(&self.x_axis, &self.y_axis).unitize()
    }
```
```rust
    pub fn set_canonical(&mut self, origin: Point3D, x_axis: Vector3D, y_axis: Vector3D) -> bool {
        if Vector3D::dot(&x_axis, &y_axis).abs() > 1e-9 { return false; }
        if (x_axis.length() - 1.0).abs() > 1e-9 { return false; }
        if (y_axis.length() - 1.0).abs() > 1e-9 { return false; }
        self.origin = origin;
        self.x_axis = x_axis;
        self.y_axis = y_axis;
        true
    }
```
```rust
    pub fn get_matrix(&self) -> Matrix4x4 {
        let z = self.z_axis();
        let p = self.origin;
        [
            [self.x_axis.x, self.y_axis.x, z.x, p.x],
            [self.x_axis.y, self.y_axis.y, z.y, p.y],
            [self.x_axis.z, self.y_axis.z, z.z, p.z],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
```
```rust
    pub fn transform_point(&self, input: Point3D) -> Point3D {
        let z = self.z_axis();
        Point3D::new(
            self.origin.x + input.x * self.x_axis.x + input.y * self.y_axis.x + input.z * z.x,
            self.origin.y + input.x * self.x_axis.y + input.y * self.y_axis.y + input.z * z.y,
            self.origin.z + input.x * self.x_axis.z + input.y * self.y_axis.z + input.z * z.z,
        )
    }
```
```rust
    pub fn transform_vector(&self, input: Vector3D) -> Vector3D {
        let z = self.z_axis();
        Vector3D::new(
            input.x * self.x_axis.x + input.y * self.y_axis.x + input.z * z.x,
            input.x * self.x_axis.y + input.y * self.y_axis.y + input.z * z.y,
            input.x * self.x_axis.z + input.y * self.y_axis.z + input.z * z.z,
        )
    }
```
```rust
    pub fn mirror_point(&self, pt: Point3D) -> Point3D {
        let z = self.z_axis();
        let mut projected = Point3D::new(0.0, 0.0, 0.0);
        on_point_project_to_plane(pt, self.origin, z, &mut projected);
        let v = projected - pt;
        projected + v
    }
```
```rust
    pub fn translate(&mut self, delta: Vector3D) {
        self.origin += delta;
    }
```
```rust
    pub fn invert(&self) -> AxisCoord {
        let z = self.z_axis();
        let x = Vector3D::new(self.x_axis.x, self.y_axis.x, z.x);
        let y = Vector3D::new(self.x_axis.y, self.y_axis.y, z.y);
        let zv = Vector3D::new(self.x_axis.z, self.y_axis.z, z.z);
        let inv_origin = Point3D::new(
            -Vector3D::dot(&self.x_axis, &self.origin),
            -Vector3D::dot(&self.y_axis, &self.origin),
            -Vector3D::dot(&z, &self.origin),
        );
        AxisCoord {
            origin: inv_origin,
            x_axis: x,
            y_axis: y,
        }
    }
```
```rust
    pub fn transform_axis_placement(&self, input: &AxisCoord) -> AxisCoord {
        let in_z = input.z_axis();
        let x = self.transform_vector(input.x_axis);
        let y = self.transform_vector(input.y_axis);
        let o = self.transform_point(input.origin);
        AxisCoord {
            origin: o,
            x_axis: x,
            y_axis: y,
        }
    }
```
```rust
    pub fn decompose_to_angles(&self) -> (f64, f64, f64) {
        let x = self.x_axis;
        let y = self.y_axis;
        let z = self.z_axis();
        let y_rot = (-x.z).asin();
        if y_rot.cos().abs() > 1e-9 {
            let x_rot = y.z.atan2(z.z);
            let z_rot = x.y.atan2(x.x);
            (x_rot, y_rot, z_rot)
        } else {
            let x_rot = 0.0;
            let z_rot = (-y.x).atan2(y.y);
            (x_rot, y_rot, z_rot)
        }
    }
```
```rust
    pub fn load_4x4(&self) -> [f64; 16] {
        let z = self.z_axis();
        [
            self.x_axis.x, self.x_axis.y, self.x_axis.z, 0.0,
            self.y_axis.x, self.y_axis.y, self.y_axis.z, 0.0,
            z.x, z.y, z.z, 0.0,
            self.origin.x, self.origin.y, self.origin.z, 1.0,
        ]
    }
}
```
--- 

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::axis_coord::AxisCoord;
    use nurbslib::core::math_extensions::{on_copy_mat, on_point_project_to_plane};
    use nurbslib::core::prelude::{Point3D, Vector3D};
    use nurbslib::core::types::Matrix4x4;

    #[test]
    fn test_on_copy_mat() {

        let src: Matrix4x4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];
        let mut dst: Matrix4x4 = [[0.0; 4]; 4];

        on_copy_mat(&src, &mut dst);

        assert_eq!(src, dst);
    }
```
```rust
    #[test]
    fn test_on_point_project_to_plane() {

        let point = Point3D::new(1.0, 2.0, 3.0);
        let plane_origin = Point3D::new(0.0, 0.0, 0.0);
        let plane_normal = Vector3D::new(0.0, 0.0, 1.0); // XY 평면

        let mut projected = Point3D::new(0.0, 0.0, 0.0);
        let ok = on_point_project_to_plane(point, plane_origin, plane_normal, &mut projected);

        assert!(ok);
        assert!((projected.z - 0.0).abs() < 1e-9); // Z=0으로 투영됨
        assert!((projected.x - point.x).abs() < 1e-9);
        assert!((projected.y - point.y).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_axis_coord_transform_point() {

        // 기본 좌표계 (원점, X=(1,0,0), Y=(0,1,0))
        let ac = AxisCoord::new();

        // 로컬 좌표 (1,2,3)을 월드 좌표로 변환
        let local = Point3D::new(1.0, 2.0, 3.0);
        let world = ac.transform_point(local);

        // 기본 좌표계에서는 동일하게 나옴
        assert_eq!(world, Point3D::new(1.0, 2.0, 3.0));
    }
```
```rust
    #[test]
    fn test_axis_coord_translate() {

        let mut ac = AxisCoord::new();
        ac.translate(Vector3D::new(10.0, 0.0, 0.0)); // 원점을 X축 방향으로 10 이동

        let local = Point3D::new(0.0, 0.0, 0.0);
        let world = ac.transform_point(local);

        // 원점이 (10,0,0)으로 이동했으므로 결과도 (10,0,0)
        assert_eq!(world, Point3D::new(10.0, 0.0, 0.0));
    }
```
```rust
    #[test]
    fn test_axiscoord_rotate_about_axis() {
        let mut ac = AxisCoord::new();

        // Z축을 기준으로 90도 회전
        ac.rotate_about_axis(std::f64::consts::FRAC_PI_2, Vector3D::new(0.0, 0.0, 1.0));

        // 로컬 좌표 (1,0,0)은 월드에서 (0,1,0)으로 변환되어야 함
        let local = Point3D::new(1.0, 0.0, 0.0);
        let world = ac.transform_point(local);

        assert!((world.x - 0.0).abs() < 1e-12);
        assert!((world.y - 1.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_axis_coord_mirror_point() {

        let ac = AxisCoord::new(); // 기본 좌표계 (XY 평면)

        let pt = Point3D::new(1.0, 2.0, 3.0);
        let mirrored = ac.mirror_point(pt);

        // XY 평면 기준 대칭 → z 좌표가 반전됨
        assert_eq!(mirrored, Point3D::new(1.0, 2.0, -3.0));
    }

}
```
