# Vector Clock
- 벡터 각도/사분면 판별 유틸리티입니다.  
- 핵심 용도를 정리하면 다음과 같습니다:

## 🧠 코드의 목적
- CircularSector / CircularSector3D
  - 두 벡터(start, end)로 정의되는 **원호 구간(부채꼴)** 을 표현합니다.
  - CentralAngle() → 두 벡터 사이의 중심각 계산
  - Domain() → 시작각~끝각 범위(ON_Interval)
  - Contains(vec) → 특정 벡터가 이 부채꼴 안에 포함되는지 판별
  - 👉 즉, “이 방향 벡터가 특정 각도 범위 안에 들어오나?”를 체크하는 도구입니다.
- VectorClock / VectorClock3D
  - 두 축 벡터(mainAxis, secAxis)를 기준으로 좌표계를 4개 사분면으로 나눔
  - 각 사분면을 CircularSector로 관리
  - Quadrant(pt) → 점이나 벡터가 어느 사분면에 속하는지 판별
  - Locate(pt, quadIndex) → 해당 사분면의 각도 범위 반환
  - 👉 **이 점/벡터가 시계방향으로 몇 번째 사분면에 있나?** 를 알려주는 시계(clock) 같은 구조입니다.
- ON_SignedAngle3D / ON_SignedAngleSafe
  - 두 벡터 사이의 부호 있는 각도를 계산합니다.
  - 법선 벡터(normal)를 기준으로 회전 방향(시계/반시계)을 판별해 ±각도로 반환
  - Safe 버전은 벡터가 0인 경우(퇴화 상황)를 안전하게 처리
  - 👉 **벡터 v1에서 v2로 회전할 때, 법선 방향 기준으로 각도가 +인지 -인지** 를 구하는 함수입니다.
- UnitVectorFromAngle
  - 주어진 법선(normal)에 수직인 기준 벡터를 잡고, angle만큼 회전시켜 단위 벡터를 생성
  - 3D에서 특정 평면 위의 방향 벡터를 각도로부터 얻을 때 사용
## 📌 실제 활용 예시- CAD/Geometry:
- 2D/3D 평면에서 점이나 벡터가 어느 사분면에 있는지 판별
- 원호, 부채꼴, 회전 각도 계산
- 곡선/곡면 분할, 방향성 검사
- 레이더/영상 처리:
- 산란점이나 방향 벡터가 특정 각도 범위에 속하는지 확인
- ISAR/SAR 영상에서 방향성 기반 분류
- 일반 기하 알고리즘:
- 벡터 방향 비교
- 회전 변환
- 각도 기반 조건 분기
## 🎯 결론이 코드의 용도는 벡터 방향을 각도로 환산하고, 이를 사분면/부채꼴 단위로 관리하여 포함 여부나 각도 범위를 판별하는 것입니다.
- 쉽게 말하면, **벡터 시계(clock)를 만들어서 방향을 분류하고 각도 계산을 안전하게 처리하는 유틸리티**.

--- 
## 소스 코드
```rust
use crate::core::geom::Vector2D;
use crate::core::prelude::{Interval, Vector3D};
use std::f64::consts::PI;

pub struct CircularSector {
    start: Vector2D,
    end: Vector2D,
    central_angle: f64,
    domain: Interval,
}
```
```rust
impl CircularSector {
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        let central_angle = Self::adjust_angle(on_signed_angle2d(start, end));
        let t0 = Self::adjust_angle(start.angle());
        let domain = Interval {
            t0,
            t1: t0 + central_angle,
        };
        Self {
            start,
            end,
            central_angle,
            domain,
        }
    }
```
```rust
    pub fn contains(&self, vec: Vector2D) -> bool {
        let angle = Self::adjust_angle(on_signed_angle2d(self.start, vec));
        angle < self.central_angle
    }
```
```rust
    pub fn central_angle(&self) -> f64 {
        self.central_angle
    }
```
```rust
    pub fn domain(&self) -> Interval {
        self.domain
    }
```
```rust
    fn adjust_angle(angle: f64) -> f64 {
        if angle >= 0.0 {
            angle
        } else {
            2.0 * PI + angle
        }
    }
}
```
```rust
/// 2D signed angle
pub fn on_signed_angle2d(v1: Vector2D, v2: Vector2D) -> f64 {
    let dot = v1.x * v2.x + v1.y * v2.y;
    let det = v1.x * v2.y - v1.y * v2.x;
    det.atan2(dot)
}
```
```rust
/// 3D CircularSector
pub struct CircularSector3D {
    start: Vector3D,
    end: Vector3D,
    normal: Vector3D,
    angle: f64,
    domain: Interval,
}
```
```rust
impl CircularSector3D {
    pub fn new(start: Vector3D, end: Vector3D, normal: Vector3D) -> Self {
        let angle = Self::adjust_angle(on_signed_angle3d(start, end, normal));
        let t0 = Self::adjust_angle(on_signed_angle3d(
            Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            start,
            normal,
        ));
        let domain = Interval { t0, t1: t0 + angle };
        Self {
            start,
            end,
            normal,
            angle,
            domain,
        }
    }
```
```rust
    pub fn contains(&self, vec: Vector3D) -> bool {
        let ang = Self::adjust_angle(on_signed_angle3d(self.start, vec, self.normal));
        ang < self.angle
    }
```
```rust
    pub fn central_angle(&self) -> f64 {
        self.angle
    }
```
```rust
    pub fn domain(&self) -> Interval {
        self.domain
    }
```
```rust
    fn adjust_angle(ang: f64) -> f64 {
        if ang >= 0.0 { ang } else { 2.0 * PI + ang }
    }
}
```
```rust
/// 3D signed angle
pub fn on_signed_angle3d(v1: Vector3D, v2: Vector3D, normal: Vector3D) -> f64 {
    let cross = v1.cross(&v2);
    let dot = v1.dot(&v2);
    let angle = cross.length().atan2(dot);
    if cross.dot(&normal) >= 0.0 {
        angle
    } else {
        -angle
    }
}
```
```rust
/// 2D VectorClock
pub struct VectorClock2D {
    main_axis: Vector2D,
    sec_axis: Vector2D,
    swapped: bool,
    start_angle: f64,
    q_angle: f64,
    sectors: [CircularSector; 4],
}
```
```rust
impl VectorClock2D {
    pub fn new(main_axis: Vector2D, sec_axis: Vector2D) -> Self {
        let start_angle = CircularSector::adjust_angle(main_axis.angle());
        let mut q_angle = on_signed_angle2d(main_axis, sec_axis);

        let swapped = q_angle < 0.0;
        if swapped {
            q_angle += PI;
        }

        let end = Vector2D {
            x: -main_axis.x,
            y: -main_axis.y,
        };
        let flipped = Vector2D {
            x: -sec_axis.x,
            y: -sec_axis.y,
        };

        let s0 = CircularSector::new(main_axis, if swapped { flipped } else { sec_axis });
        let s1 = CircularSector::new(s0.end, end);
        let s2 = CircularSector::new(s1.end, if swapped { sec_axis } else { flipped });
        let s3 = CircularSector::new(s2.end, main_axis);

        Self {
            main_axis,
            sec_axis,
            swapped,
            start_angle,
            q_angle,
            sectors: [s0, s1, s2, s3],
        }
    }
```
```rust
    pub fn quadrant(&self, pt: Vector2D) -> usize {
        for (i, sec) in self.sectors.iter().enumerate() {
            if sec.contains(pt) {
                return i;
            }
        }
        0
    }

    pub fn locate(&self, pt: Vector2D) -> (usize, Interval) {
        let quad = self.quadrant(pt);
        (quad, self.sectors[quad].domain())
    }
}
```
```rust
/// 3D VectorClock
pub struct VectorClock3D {
    main_axis: Vector3D,
    sec_axis: Vector3D,
    normal: Vector3D,
    swapped_axis: bool,
    start_angle: f64,
    q_angle: f64,
    sectors: [CircularSector3D; 4],
}
```
```rust
impl VectorClock3D {
    pub fn new(main_axis: Vector3D, sec_axis: Vector3D, normal: Vector3D) -> Self {
        let start_angle = CircularSector3D::adjust_angle(on_signed_angle3d(
            Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            main_axis,
            normal,
        ));
        let mut q_angle = on_signed_angle3d(main_axis, sec_axis, normal);

        let swapped_axis = q_angle < 0.0;
        if swapped_axis {
            q_angle += PI;
        }

        let neg_main = Vector3D {
            x: -main_axis.x,
            y: -main_axis.y,
            z: -main_axis.z,
        };
        let neg_sec = Vector3D {
            x: -sec_axis.x,
            y: -sec_axis.y,
            z: -sec_axis.z,
        };

        let s0 = CircularSector3D::new(
            main_axis,
            if swapped_axis { neg_sec } else { sec_axis },
            normal,
        );
        let s1 =
            CircularSector3D::new(Vector3D::from_angle(s0.domain.t1, normal), neg_main, normal);
        let s2 = CircularSector3D::new(
            Vector3D::from_angle(s1.domain.t1, normal),
            if swapped_axis { sec_axis } else { neg_sec },
            normal,
        );
        let s3 = CircularSector3D::new(
            Vector3D::from_angle(s2.domain.t1, normal),
            main_axis,
            normal,
        );

        Self {
            main_axis,
            sec_axis,
            normal,
            swapped_axis,
            start_angle,
            q_angle,
            sectors: [s0, s1, s2, s3],
        }
    }
```
```rust
    pub fn quadrant(&self, dir: Vector3D) -> usize {
        for (i, sec) in self.sectors.iter().enumerate() {
            if sec.contains(dir) {
                return i;
            }
        }
        0
    }
```
```rust
    pub fn quadrant_math(&self, dir: Vector3D) -> usize {
        match (dir.x >= 0.0, dir.y >= 0.0) {
            (true, true) => 0,
            (false, true) => 1,
            (false, false) => 2,
            (true, false) => 3,
        }
    }
```
```rust
    pub fn locate(&self, dir: Vector3D) -> (usize, Interval) {
        let quad = self.quadrant(dir);
        (quad, self.sectors[quad].domain())
    }
}
```
---
## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::circular_sector::{
        CircularSector, CircularSector3D, on_signed_angle2d, on_signed_angle3d,
    };
    use nurbslib::core::geom::Vector2D;
    use nurbslib::core::prelude::Vector3D;

    #[test]
    fn test_circular_sector_2d() {
        // x축 → y축 방향 부채꼴
        let start = Vector2D { x: 1.0, y: 0.0 };
        let end = Vector2D { x: 0.0, y: 1.0 };
        let sector = CircularSector::new(start, end);

        // 중심각은 90도 (π/2)
        assert!((sector.central_angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-6);

        // 도메인 범위 확인
        let dom = sector.domain();
        assert!(dom.t1 - dom.t0 > 0.0);

        // 벡터 (1,1)은 포함되어야 함
        let vec = Vector2D { x: 1.0, y: 1.0 };
        assert!(sector.contains(vec));

        // 벡터 (-1,0)은 포함되지 않음
        let vec2 = Vector2D { x: -1.0, y: 0.0 };
        assert!(!sector.contains(vec2));
    }
```
```rust
    #[test]
    fn test_circular_sector_3d() {
        // 3D: x축 → y축, 법선은 z축
        let start = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let normal = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        let sector3d = CircularSector3D::new(start, end, normal);

        // 중심각은 90도 (π/2)
        assert!((sector3d.central_angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-6);

        // 벡터 (1,1,0)은 포함되어야 함
        let vec = Vector3D {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };
        assert!(sector3d.contains(vec));

        // 벡터 (-1,0,0)은 포함되지 않음
        let vec2 = Vector3D {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        assert!(!sector3d.contains(vec2));
    }
```
```rust
    #[test]
    fn test_signed_angle_functions() {
        let v1 = Vector2D { x: 1.0, y: 0.0 };
        let v2 = Vector2D { x: 0.0, y: 1.0 };
        let ang2d = on_signed_angle2d(v1, v2);
        assert!((ang2d - std::f64::consts::FRAC_PI_2).abs() < 1e-6);

        let v3 = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v4 = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let normal = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let ang3d = on_signed_angle3d(v3, v4, normal);
        assert!((ang3d - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
    }
}
```
```rust
#[cfg(test)]
mod tests_vector {
    use nurbslib::core::circular_sector::{
        CircularSector, CircularSector3D, VectorClock2D, VectorClock3D,
    };
    use nurbslib::core::geom::Vector2D;
    use nurbslib::core::prelude::Vector3D;

    #[test]
    fn test_circular_sector_2d() {
        // x축 → y축 방향 부채꼴
        let start = Vector2D { x: 1.0, y: 0.0 };
        let end = Vector2D { x: 0.0, y: 1.0 };
        let sector = CircularSector::new(start, end);

        // 중심각은 90도 (π/2)
        assert!((sector.central_angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-6);

        // 벡터 (1,1)은 포함되어야 함
        let vec = Vector2D { x: 1.0, y: 1.0 };
        assert!(sector.contains(vec));

        // 벡터 (-1,0)은 포함되지 않음
        let vec2 = Vector2D { x: -1.0, y: 0.0 };
        assert!(!sector.contains(vec2));
    }
```
```rust
    #[test]
    fn test_vector_clock_2d_quadrants() {
        let main = Vector2D { x: 1.0, y: 0.0 };
        let sec = Vector2D { x: 0.0, y: 1.0 };
        let clock = VectorClock2D::new(main, sec);

        // (1,1)은 0번 사분면
        let pt = Vector2D { x: 1.0, y: 1.0 };
        let q = clock.quadrant(pt);
        assert_eq!(q, 0);

        // (-1,1)은 1번 사분면
        let pt2 = Vector2D { x: -1.0, y: 1.0 };
        let q2 = clock.quadrant(pt2);
        assert_eq!(q2, 1);

        // (-1,-1)은 2번 사분면
        let pt3 = Vector2D { x: -1.0, y: -1.0 };
        let q3 = clock.quadrant(pt3);
        assert_eq!(q3, 2);

        // (1,-1)은 3번 사분면
        let pt4 = Vector2D { x: 1.0, y: -1.0 };
        let q4 = clock.quadrant(pt4);
        assert_eq!(q4, 3);
    }
```
```rust
    #[test]
    fn test_circular_sector_3d() {
        let start = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let normal = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        let sector3d = CircularSector3D::new(start, end, normal);

        // 중심각은 90도 (π/2)
        assert!((sector3d.central_angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-6);

        // 벡터 (1,1,0)은 포함되어야 함
        let vec = Vector3D {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };
        assert!(sector3d.contains(vec));

        // 벡터 (-1,0,0)은 포함되지 않음
        let vec2 = Vector3D {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        assert!(!sector3d.contains(vec2));
    }
```
```rust
    #[test]
    fn test_vector_clock_3d_quadrants() {
        let main = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let sec = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let normal = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let clock3d = VectorClock3D::new(main, sec, normal);

        // (1,1,0)은 0번 사분면
        let dir = Vector3D {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };
        let q = clock3d.quadrant_math(dir);
        assert_eq!(q, 0);

        // (-1,1,0)은 1번 사분면
        let dir2 = Vector3D {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        };
        let q2 = clock3d.quadrant_math(dir2);
        assert_eq!(q2, 1);

        // (-1,-1,0)은 2번 사분면
        let dir3 = Vector3D {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        };
        let q3 = clock3d.quadrant_math(dir3);
        assert_eq!(q3, 2);

        // (1,-1,0)은 3번 사분면
        let dir4 = Vector3D {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        };
        let q4 = clock3d.quadrant_math(dir4);
        assert_eq!(q4, 3);
    }
}
```
---

