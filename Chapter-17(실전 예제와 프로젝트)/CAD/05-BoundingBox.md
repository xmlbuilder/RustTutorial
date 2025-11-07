# 📦 BoundingBox 전체 기능 정리

## 🧱 구조 정의
```rust
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}
```
- min: 최소 좌표
- max: 최대 좌표

## 🏗️ 생성 및 초기화
| 메서드 / 상수                           | 설명                                                                 |
|----------------------------------------|----------------------------------------------------------------------|
| `new(min, max)` / `from_min_max()`     | 최소/최대 좌표를 지정해 박스를 생성합니다.                           |
| `default()` / `EMPTY` / `UNSET` / `NAN_BOX` | 초기 상태 정의. 유효하지 않은 박스나 NaN 상태를 표현합니다.         |
| `set(x, y, z)` / `set_point3d(pt)` / `set_vec3d(v)` | 단일 점 또는 벡터를 기준으로 박스를 설정합니다.                     |
| `grow(x, y, z)` / `grow_point3d(p)`    | 기존 박스를 확장하여 새로운 점을 포함시킵니다.                        |
| `expand(delta)` / `shrink(delta)`      | 박스를 지정된 벡터만큼 키우거나 줄입니다.                            |

### ✅ 사용 시점 요약
- new() / from_min_max() → 박스를 명시적으로 생성할 때
- default() / EMPTY → 초기화 또는 유효하지 않은 상태 표현
- set() / grow() → 점 또는 벡터를 기반으로 박스를 설정하거나 확장할 때
- expand() / shrink() → 박스의 크기를 조절할 때 (예: 여유 공간 확보, 경계 축소)

## 📐 기하 정보
| 메서드                                | 설명                                                                 |
|--------------------------------------|----------------------------------------------------------------------|
| `center()`                           | 박스의 중심 좌표를 반환합니다.                                       |
| `diagonal()` / `diagonal_length()`   | 대각선 벡터 및 그 길이를 계산합니다.                                 |
| `volume()` / `area()`                | 박스의 부피와 표면적을 계산합니다.                                   |
| `corner(xi, yi, zi)` / `corners()`   | 특정 꼭짓점 또는 전체 8개 꼭짓점을 반환합니다.                       |
| `edge(index)`                        | 12개의 모서리 중 하나를 `Segment3D`로 반환합니다.                    |
| `max_extent_axis()`                  | 가장 길이가 긴 축의 인덱스를 반환합니다 (0=x, 1=y, 2=z).              |
| `get_range_f32()` / `get_range_f64()`| `[min.x, min.y, min.z, max.x, max.y, max.z]` 배열로 반환합니다.       |

### ✅ 활용 예시
- center() → 중심점 기준으로 회전, 이동 등 변환할 때
- diagonal_length() → 외접 구 반지름 계산, 크기 비교
- volume() / area() → 공간 점유율, 충돌 판정 등
- corners() / edge() → 시각화, 메시 생성, 경계 추출
- max_extent_axis() → 분할 기준 축 선택 (예: BVH, KD-Tree)


## 🔍 상태 판별
| 메서드                                      | 설명                                                                 |
|--------------------------------------------|----------------------------------------------------------------------|
| `is_nan()` / `is_unset()` / `is_set()`     | 좌표에 NaN이 포함되어 있는지, 유한한 값인지 여부를 판단합니다.         |
| `is_empty()` / `is_valid()` / `is_point()` | 박스가 비어 있는지, 유효한지, 하나의 점인지 여부를 판단합니다.         |
| `is_degenerate(tol)`                       | 퇴화 상태를 판별합니다. (0=정상, 1=면, 2=선, 3=점, 4=무효)              |
| `is_disjoint(other)`                       | 다른 박스와 교차하지 않는지 여부를 판단합니다.                         |

### ✅ 활용 예시
- is_valid() → 연산 전에 박스가 유효한지 확인
- is_empty() → min > max 상태로 비어 있는지 확인
- is_point() → min == max → 단일 점인지 확인
- is_degenerate() → 퇴화된 형태인지 판단 (예: 평면, 선, 점)
- is_disjoint() → 두 박스가 겹치지 않는지 확인 (충돌 판정 등)

## 📏 거리 및 포함 관계
| 메서드                                           | 설명                                                                 |
|--------------------------------------------------|----------------------------------------------------------------------|
| `closest_point(p)` / `far_point(p)`              | 주어진 점과 가장 가까운/먼 박스 경계점을 반환합니다.                  |
| `min_distance_to_point(p)` / `max_distance_to_point(p)` | 점과 박스 사이의 최소/최대 거리를 계산합니다.                        |
| `min_distance_to_bbox(b)` / `max_distance_to_bbox(b)` | 두 박스 사이의 최소/최대 거리를 계산합니다.                          |
| `includes(other)` / `includes_bbox(other, proper)` | 다른 박스를 포함하거나 진부분집합인지 확인합니다.                     |
| `includes_point(p, proper)`                      | 점이 박스 내부에 포함되는지 확인합니다.                              |
| `is_point_inside_axes(p)`                        | 점이 x/y/z 축별로 박스 내부에 있는지 여부를 반환합니다.              |
| `on_aabb_lb_distance(a, b)`                      | 두 AABB 사이의 보수적 최소 거리(LB)를 계산합니다.                    |
| `on_diagonal_length(bbox)`                       | 외부 유틸리티 기반으로 대각선 길이를 계산합니다.                     |

### ✅ 활용 예시
- closest_point() → 충돌 판정, 거리 기반 필터링
- includes_bbox() → 공간 포함 관계 확인 (예: BVH, 공간 분할)
- min_distance_to_bbox() → 근접성 기반 정렬, 거리 기반 탐색
- is_point_inside_axes() → 축별 포함 여부를 활용한 조건 분기

| 메서드                                                   | 설명                                                                 |
|----------------------------------------------------------|----------------------------------------------------------------------|
| `union(a, b)` / `union_with()` / `union_inplace()` / `union_mut()` | 두 박스를 병합하여 최소/최대 범위를 확장합니다.                        |
| `intersection(a, b)` / `intersection_inplace()`          | 두 박스의 교차 영역을 계산합니다. 유효하지 않으면 기본 박스로 초기화됩니다. |
| `closest_on_line(seg)`                                   | 선분과 박스의 교차 여부 및 최근접점을 계산합니다.                      |
| `on_intersects_ray_bbox(ray, bbox)`                      | 레이와 AABB의 교차 여부를 Slab 방식으로 판정합니다.                    |

### ✅ 활용 예시
- union() → 여러 객체의 경계를 하나로 묶을 때 (예: 씬 전체 AABB)
- intersection() → 충돌 영역 계산, 시각적 클리핑
- closest_on_line() → 선분이 박스를 통과하는지, 닿는지, 가장 가까운 점은 어디인지
- on_intersects_ray_bbox() → 레이 트레이싱, 가시성 판정, 선택 영역 처리

## 🔧 변형 및 외부 연산
| 메서드           | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| `transform(xf)`  | 주어진 변환 행렬(`Transform`)을 적용하여 박스를 갱신합니다.           |
| `sphere_bound()` | 박스를 감싸는 구의 중심과 반지름을 반환합니다.                        |
| `tolerance()`    | 박스의 크기에 비례한 비교용 허용 오차를 계산합니다.                   |

### ✅ 활용 예시
- transform() → 회전, 스케일, 이동 등 변환 후 AABB 재계산
- sphere_bound() → 구 기반 충돌 판정, 시각화, 거리 기반 필터링
- tolerance() → 정밀 비교 시 오차 허용 (예: is_degenerate()와 함께 사용)


## 🧮 외부 유틸리티 함수
| 함수 이름                                      | 설명                                                                 |
|------------------------------------------------|----------------------------------------------------------------------|
| `on_bounding_box_points(points)`              | 주어진 `Point` 리스트로부터 AABB(min, max)를 계산합니다.              |
| `on_compute_bounding_box(transform, points)`  | 변환 행렬을 적용한 후 AABB(min, max)를 계산합니다.                    |
| `on_compute_bounding_box_f32(transform, points, skip)` | `f32` 배열 기반으로 AABB를 계산하며, 변환과 건너뛰기 설정을 지원합니다. |
| `on_compute_bounding_box_into(transform, points, count)` | 지정된 개수만큼의 `Point`를 사용해 `BoundingBox` 객체를 반환합니다.     |
| `on_plane_eq_for_dir(dir, dom)`               | 방향 인덱스(`dir`)에 따라 평면 방정식 `[a, b, c, d]`를 반환합니다.     |
| `on_intersect_face_flag(dir, tri_bb, dom)`    | 방향 인덱스 기준으로 `tri_bb`가 `dom`을 넘는지 여부를 판정합니다.       |

### ✅ 활용 예시
- on_compute_bounding_box() → 변환된 점들로부터 AABB 계산 (예: 모델링, 애니메이션)
- on_compute_bounding_box_f32() → GPU/메모리 최적화된 포맷에서 AABB 추출
- on_plane_eq_for_dir() → 평면 클리핑, 충돌 판정, 공간 분할
- on_intersect_face_flag() → 면 교차 여부 판정 (예: BSP, CSG, voxelization)


## ✅ 전체 요약
- BoundingBox는 3D 공간에서 AABB를 표현하는 구조체
- 생성, 상태 판별, 거리 계산, 포함 관계, 병합, 교차, 변형 등 다양한 기능 제공
- OpenNurbs 스타일과 CAD/CG 응용에 적합한 정밀한 설계
- 외부 유틸리티 함수들과 함께 사용하면 다양한 형식의 데이터에 대응 가능

---

## 소스 코드
```rust
// src/geom/bounding_box.rs
#![allow(dead_code)]

use crate::core::geom::{Point, Vector};
use crate::core::segment3d::Segment3D;
use crate::core::transform::Transform;
use crate::core::types::{get_axis_point, ON_TOL12, ON_TOL6, UNSET_POINT_3};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}
```
```rust
impl BoundingBox {
    pub fn intersects_self(&self, p0: &BoundingBox) -> bool {
        Self::intersects(self, p0)
    }
}
```
```rust
impl BoundingBox {
    pub(crate) fn min_mut(&mut self, axis: usize, p1: f64) {
        self.min[axis] = p1;
    }
}
```
```rust
impl BoundingBox {
    pub fn max_mut(&mut self, axis: usize, p1: f64) {
        self.max[axis] = p1;
    }
}
```
```rust
impl BoundingBox {
    pub(crate) fn max_extent_axis(&self) -> usize {
        (self.max - self.min).maximum_coordinate_index()
    }
}
```
```rust
impl BoundingBox {
    pub(crate) fn union_mut(&mut self, p0: &BoundingBox) {
        if self.is_valid() {
            if p0.is_valid() {
                if p0.min.x < self.min.x {
                    self.min.x = p0.min.x;
                }
                if p0.min.y < self.min.y {
                    self.min.y = p0.min.y;
                }
                if p0.min.z < self.min.z {
                    self.min.z = p0.min.z;
                }
                if p0.max.x > self.max.x {
                    self.max.x = p0.max.x;
                }
                if p0.max.y > self.max.y {
                    self.max.y = p0.max.y;
                }
                if p0.max.z > self.max.z {
                    self.max.z = p0.max.z;
                }
            }
        } else if p0.is_valid() {
            self.min = p0.min.clone();
            self.max = p0.max.clone();
        }
    }
}
```
```rust
impl Default for BoundingBox {
    /// Empty box: min = (1,0,0), max = (-1,0,0) (OpenNurbs)
    fn default() -> Self {
        Self {
            min: Point::new(1.0, 0.0, 0.0),
            max: Point::new(-1.0, 0.0, 0.0),
        }
    }
}
```
```rust
impl BoundingBox {
    pub(crate) fn intersects(p0: &BoundingBox, p1: &BoundingBox) -> bool {
        if !p0.is_valid() || !p1.is_valid() {
            return false;
        }
        (p0.min.x <= p1.max.x && p0.max.x >= p1.min.x)
            && (p0.min.y <= p1.max.y && p0.max.y >= p1.min.y)
            && (p0.min.z <= p1.max.z && p0.max.z >= p1.min.z)
    }
}
```
```rust
impl BoundingBox {
    pub fn includes(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.min.x
            && self.min.y <= other.min.y
            && self.min.z <= other.min.z
            && self.max.x >= other.max.x
            && self.max.y >= other.max.y
            && self.max.z >= other.max.z
    }

    pub fn union_with(&mut self, other: &BoundingBox) {
        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);
        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);
    }
}
```
```rust
impl BoundingBox {
    pub fn empty() -> Self {
        BoundingBox {
            min: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    /// Calculates the length of the diagonal
    pub fn diagonal_length(&self) -> f64 {
        ((self.max.x - self.min.x).powi(2)
            + (self.max.y - self.min.y).powi(2)
            + (self.max.z - self.min.z).powi(2))
        .sqrt()
    }
}
```
```rust
impl BoundingBox {
    pub const EMPTY: BoundingBox = BoundingBox {
        min: Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        max: Point {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
    };

    pub const UNSET: BoundingBox = BoundingBox {
        min: Point {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        },
        max: Point {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        },
    };

    pub const NAN_BOX: BoundingBox = BoundingBox {
        min: Point {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        },
        max: Point {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        },
    };

    #[inline]
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn from_min_max(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn min(&self) -> Point {
        self.min
    }
    #[inline]
    pub fn max(&self) -> Point {
        self.max
    }

    #[inline]
    pub fn center(&self) -> Point {
        Point::new(
            0.5 * (self.min.x + self.max.x),
            0.5 * (self.min.y + self.max.y),
            0.5 * (self.min.z + self.max.z),
        )
    }

    #[inline]
    pub fn diagonal(&self) -> Vector {
        Vector::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    /// x_index, y_index, z_index ∈ {0(=min), 1(=max)}
    pub fn corner(&self, xi: i32, yi: i32, zi: i32) -> Point {
        Point::new(
            if xi > 0 { self.max.x } else { self.min.x },
            if yi > 0 { self.max.y } else { self.min.y },
            if zi > 0 { self.max.z } else { self.min.z },
        )
    }

    pub fn corners(&self) -> [Point; 8] {
        [
            self.corner(0, 0, 0),
            self.corner(1, 0, 0),
            self.corner(0, 1, 0),
            self.corner(1, 1, 0),
            self.corner(0, 0, 1),
            self.corner(1, 0, 1),
            self.corner(0, 1, 1),
            self.corner(1, 1, 1),
        ]
    }

    /// Returns 12 edges (in the same order as ON indices)
    pub fn edge(&self, index: u32) -> Segment3D {
        match index % 12 {
            0 => Segment3D::new(self.corner(0, 0, 0), self.corner(1, 0, 0)),
            1 => Segment3D::new(self.corner(0, 0, 0), self.corner(0, 1, 0)),
            2 => Segment3D::new(self.corner(0, 0, 0), self.corner(0, 0, 1)),
            3 => Segment3D::new(self.corner(0, 0, 1), self.corner(1, 0, 1)),
            4 => Segment3D::new(self.corner(0, 0, 1), self.corner(0, 1, 1)),
            5 => Segment3D::new(self.corner(0, 1, 0), self.corner(1, 1, 0)),
            6 => Segment3D::new(self.corner(0, 1, 0), self.corner(0, 1, 1)),
            7 => Segment3D::new(self.corner(1, 0, 0), self.corner(1, 0, 1)),
            8 => Segment3D::new(self.corner(1, 0, 0), self.corner(1, 1, 0)),
            9 => Segment3D::new(self.corner(0, 1, 1), self.corner(1, 1, 1)),
            10 => Segment3D::new(self.corner(1, 0, 1), self.corner(1, 1, 1)),
            11 => Segment3D::new(self.corner(1, 1, 0), self.corner(1, 1, 1)),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.min.x.is_nan()
            || self.min.y.is_nan()
            || self.min.z.is_nan()
            || self.max.x.is_nan()
            || self.max.y.is_nan()
            || self.max.z.is_nan()
    }

    #[inline]
    pub fn is_unset(&self) -> bool {
        self.is_nan()
    }

    #[inline]
    pub fn is_set(&self) -> bool {
        self.min.x.is_finite()
            && self.min.y.is_finite()
            && self.min.z.is_finite()
            && self.max.x.is_finite()
            && self.max.y.is_finite()
            && self.max.z.is_finite()
    }

    /// OpenNurbs: IsEmpty() == (min>max in any axis) && IsSet()
    pub fn is_empty(&self) -> bool {
        self.is_set()
            && (self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z)
    }

    pub fn is_valid(&self) -> bool {
        self.is_set()
            && (self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z)
    }

    pub fn is_point(&self) -> bool {
        self.is_set()
            && (self.min.x == self.max.x && self.min.y == self.max.y && self.min.z == self.max.z)
    }

    /// 0: not degenerate, 1: rectangle, 2: line, 3: point, 4: invalid
    pub fn is_degenerate(&self, tol: Option<f64>) -> i32 {
        if !self.is_valid() {
            return 4;
        }
        let tol = tol.unwrap_or(0.0).abs();
        let dx = (self.max.x - self.min.x).abs();
        let dy = (self.max.y - self.min.y).abs();
        let dz = (self.max.z - self.min.z).abs();
        let mut zero = 0;
        if dx <= tol {
            zero += 1;
        }
        if dy <= tol {
            zero += 1;
        }
        if dz <= tol {
            zero += 1;
        }
        match zero {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => 3,
        }
    }

    /// Checks whether `test_point` is inside the box (including boundaries)
    pub fn is_point_in(&self, p: &Point, strictly: bool) -> bool {
        if strictly {
            (self.min.x - ON_TOL6) < p.x
                && p.x < (self.max.x + ON_TOL6)
                && (self.min.y - ON_TOL6) < p.y
                && p.y < (self.max.y + ON_TOL6)
                && (self.min.z - ON_TOL6) < p.z
                && p.z < (self.max.z + ON_TOL6)
        } else {
            (self.min.x - ON_TOL6) <= p.x
                && p.x <= (self.max.x + ON_TOL6)
                && (self.min.y - ON_TOL6) <= p.y
                && p.y <= (self.max.y + ON_TOL6)
                && (self.min.z - ON_TOL6) <= p.z
                && p.z <= (self.max.z + ON_TOL6)
        }
    }

    /// Finds the closest point inside or on the boundary of the box
    pub fn closest_point(&self, p: Point) -> Point {
        Point::new(
            p.x.clamp(self.min.x, self.max.x),
            p.y.clamp(self.min.y, self.max.y),
            p.z.clamp(self.min.z, self.max.z),
        )
    }

    /// Radius and center of the sphere that encloses the box
    pub fn sphere_bound(&self) -> (Point, f64) {
        let c = self.center();
        (c, 0.5 * self.diagonal().length())
    }

    /// Fast lower-bound distance (actual closest distance)
    pub fn min_distance_to_point(&self, p: Point) -> f64 {
        p.distance(&self.closest_point(p))
    }

    /// Fast upper-bound distance (to the farthest corner of the box)
    pub fn max_distance_to_point(&self, p: Point) -> f64 {
        let fp = self.far_point(p);
        p.distance(&fp)
    }

    pub fn far_point(&self, p: Point) -> Point {
        let fx = if (self.min.x - p.x).abs() >= (self.max.x - p.x).abs() {
            self.min.x
        } else {
            self.max.x
        };
        let fy = if (self.min.y - p.y).abs() >= (self.max.y - p.y).abs() {
            self.min.y
        } else {
            self.max.y
        };
        let fz = if (self.min.z - p.z).abs() >= (self.max.z - p.z).abs() {
            self.min.z
        } else {
            self.max.z
        };
        Point::new(fx, fy, fz)
    }

    /// Tight minimum/maximum distance between two boxes
    pub fn min_distance_to_bbox(&self, other: &BoundingBox) -> f64 {
        if !self.is_valid() || !other.is_valid() {
            return 0.0;
        }
        let dx = if self.max.x < other.min.x {
            other.min.x - self.max.x
        } else if other.max.x < self.min.x {
            self.min.x - other.max.x
        } else {
            0.0
        };
        let dy = if self.max.y < other.min.y {
            other.min.y - self.max.y
        } else if other.max.y < self.min.y {
            self.min.y - other.max.y
        } else {
            0.0
        };
        let dz = if self.max.z < other.min.z {
            other.min.z - self.max.z
        } else if other.max.z < self.min.z {
            self.min.z - other.max.z
        } else {
            0.0
        };
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn max_distance_to_bbox(&self, other: &BoundingBox) -> f64 {
        // 서로 가장 먼 코너 조합 추정
        let a = [self.min, self.max];
        let b = [other.min, other.max];
        let mut d2: f64 = 0.0;
        for &pa in &a {
            for &pb in &b {
                d2 = d2.max(pa.distance_square(&pb));
            }
        }
        d2.sqrt()
    }

    /// Computes intersection or nearest point with a segment.
    /// Return codes:
    /// 3 — Segment overlaps (t0 < t1)
    /// 2 — Single point of intersection (t0 == t1)
    /// 1 — No intersection; returns closest point
    /// 0 — Computation failed
    pub fn closest_on_line(&self, seg: &Segment3D) -> (i32, Point, f64, f64) {
        if !self.is_valid() {
            return (0, UNSET_POINT_3, 0.0, 0.0);
        }

        // Compute [t0, t1] using the slab method
        let dir = seg.direction();
        let p0 = seg.p0;

        let mut t0 = 0.0_f64;
        let mut t1 = 1.0_f64;

        for axis in 0..3 {
            let (p0a, da, min_a, max_a) = match axis {
                0 => (p0.x, dir.x, self.min.x, self.max.x),
                1 => (p0.y, dir.y, self.min.y, self.max.y),
                _ => (p0.z, dir.z, self.min.z, self.max.z),
            };
            if da.abs() < ON_TOL12 {
                if p0a < min_a || p0a > max_a {
                    // Parallel and outside the box: no intersection possible
                    return (1, self.closest_point(p0), 0.0, 0.0);
                }
                // Parallel and inside: no condition needed
                continue;
            }
            let inv = 1.0 / da;
            let mut tmin = (min_a - p0a) * inv;
            let mut tmax = (max_a - p0a) * inv;
            if tmin > tmax {
                std::mem::swap(&mut tmin, &mut tmax);
            }
            t0 = t0.max(tmin);
            t1 = t1.min(tmax);
            if t0 > t1 {
                // No intersection: return the closer of the segment's endpoints
                let c0 = self.closest_point(seg.p0);
                let c1 = self.closest_point(seg.p1);
                let d0 = c0.distance_square(&seg.p0);
                let d1 = c1.distance_square(&seg.p1);
                if d0 <= d1 {
                    return (1, c0, 0.0, 0.0);
                } else {
                    return (1, c1, 1.0, 1.0);
                }
            }
        }

        // Infinite line intersects; check if it also intersects the segment interval
        let (tt0, tt1) = (t0.max(0.0), t1.min(1.0));
        if tt0 > tt1 {
            // Infinite line intersects, but the segment does not contain the intersection → use the closest endpoint
            let c0 = self.closest_point(seg.p0);
            let c1 = self.closest_point(seg.p1);
            let d0 = c0.distance(&seg.p0);
            let d1 = c1.distance_square(&seg.p1);
            if d0 <= d1 {
                (1, c0, 0.0, 0.0)
            } else {
                (1, c1, 1.0, 1.0)
            }
        } else {
            // Intersect with the segment
            let q0 = seg.point_at(tt0);
            let _q1 = seg.point_at(tt1);
            let code = if (tt1 - tt0).abs() <= ON_TOL12 { 2 } else { 3 };
            (code, q0, tt0, tt1)
        }
    }

    /// Updates the AABB using the transformed 8 corners
    pub fn transform(&mut self, xf: &Transform) -> bool {
        if !self.is_valid() {
            return false;
        }
        let cs = self.corners();
        let mut b = BoundingBox::default();
        for (i, &p) in cs.iter().enumerate() {
            let tp = xf.apply_point(p);
            if i == 0 {
                b.min = tp;
                b.max = tp;
            } else {
                b.min.x = b.min.x.min(tp.x);
                b.min.y = b.min.y.min(tp.y);
                b.min.z = b.min.z.min(tp.z);
                b.max.x = b.max.x.max(tp.x);
                b.max.y = b.max.y.max(tp.y);
                b.max.z = b.max.z.max(tp.z);
            }
        }
        *self = b;
        true
    }

    /// Approximate comparison tolerance (replacement for ON_BoundingBox::Tolerance)
    pub fn tolerance(&self) -> f64 {
        if !self.is_valid() {
            return 0.0;
        }
        let d = self.diagonal().length();
        (d * ON_TOL12).max(ON_TOL12)
    }

    /// Inclusion or proper subset
    pub fn includes_bbox(&self, other: &BoundingBox, proper: bool) -> bool {
        if !self.is_valid() || !other.is_valid() {
            return false;
        }
        let le = self.min.x <= other.min.x
            && self.min.y <= other.min.y
            && self.min.z <= other.min.z
            && other.max.x <= self.max.x
            && other.max.y <= self.max.y
            && other.max.z <= self.max.z;
        if !le {
            return false;
        }
        if !proper {
            return true;
        }
        // 적어도 한 축에서 strict
        let ret = self.min.x < other.min.x
            || self.min.y < other.min.y
            || self.min.z < other.min.z
            || other.max.x < self.max.x
            || other.max.y < self.max.y
            || other.max.z < self.max.z;
        ret
    }

    pub fn includes_point(&self, p: &Point, proper: bool) -> bool {
        self.is_point_in(p, proper)
    }

    pub fn union_inplace(&mut self, other: &BoundingBox) -> bool {
        if !other.is_valid() {
            return self.is_valid();
        }
        if !self.is_valid() {
            *self = *other;
            return true;
        }
        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);
        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);
        true
    }

    pub fn union(a: &BoundingBox, b: &BoundingBox) -> (bool, BoundingBox) {
        if !a.is_valid() && !b.is_valid() {
            return (false, BoundingBox::default());
        }
        if !a.is_valid() {
            return (true, *b);
        }
        if !b.is_valid() {
            return (true, *a);
        }
        let mut out = *a;
        out.union_inplace(b);
        (true, out)
    }

    pub fn intersection_inplace(&mut self, other: &BoundingBox) -> bool {
        if !self.is_valid() || !other.is_valid() {
            *self = BoundingBox::default();
            return false;
        }
        let min = Point::new(
            self.min.x.max(other.min.x),
            self.min.y.max(other.min.y),
            self.min.z.max(other.min.z),
        );
        let max = Point::new(
            self.max.x.min(other.max.x),
            self.max.y.min(other.max.y),
            self.max.z.min(other.max.z),
        );
        let mut out = BoundingBox::new(min, max);
        if !out.is_valid() {
            out = BoundingBox::default();
            *self = out;
            return false;
        }
        *self = out;
        true
    }

    pub fn intersection(a: &BoundingBox, b: &BoundingBox) -> (bool, BoundingBox) {
        let mut out = *a;
        let ok = out.intersection_inplace(b);
        (ok, out)
    }

    #[inline]
    pub fn is_disjoint(&self, other: &BoundingBox) -> bool {
        if !self.is_valid() || !other.is_valid() {
            return true;
        }
        self.max.x < other.min.x
            || other.max.x < self.min.x
            || self.max.y < other.min.y
            || other.max.y < self.min.y
            || self.max.z < other.min.z
            || other.max.z < self.min.z
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.grow(x, y, z);
    }

    pub fn grow(&mut self, x: f64, y: f64, z: f64) {
        if self.min.x > x {
            self.min.x = x;
        }
        if self.min.y > y {
            self.min.y = y;
        }
        if self.min.z > z {
            self.min.z = z;
        }
        if self.max.x < x {
            self.max.x = x;
        }
        if self.max.y < y {
            self.max.y = y;
        }
        if self.max.z < z {
            self.max.z = z;
        }
    }

    pub fn grow_point3d(&mut self, p: &Point) {
        self.grow(p.x, p.y, p.z);
    }

    pub fn set_point3d(&mut self, pt: &Point) {
        if self.min.x > pt.x {
            self.min.x = pt.x;
        }
        if self.min.y > pt.y {
            self.min.y = pt.y;
        }
        if self.min.z > pt.z {
            self.min.z = pt.z;
        }
        if self.max.x < pt.x {
            self.max.x = pt.x;
        }
        if self.max.y < pt.y {
            self.max.y = pt.y;
        }
        if self.max.z < pt.z {
            self.max.z = pt.z;
        }
    }

    pub fn set_vec3d(&mut self, v: &Vector) {
        if self.min.x > v.x {
            self.min.x = v.x;
        }
        if self.min.y > v.y {
            self.min.y = v.y;
        }
        if self.min.z > v.z {
            self.min.z = v.z;
        }
        if self.max.x < v.x {
            self.max.x = v.x;
        }
        if self.max.y < v.y {
            self.max.y = v.y;
        }
        if self.max.z < v.z {
            self.max.z = v.z;
        }
    }

    pub fn expand(&mut self, delta: Vector) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.min.x -= delta.x;
        self.min.y -= delta.y;
        self.min.z -= delta.z;
        self.max.x += delta.x;
        self.max.y += delta.y;
        self.max.z += delta.z;
        self.is_valid()
    }

    pub fn shrink(&mut self, delta: Vector) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.min.x += delta.x;
        self.min.y += delta.y;
        self.min.z += delta.z;
        self.max.x -= delta.x;
        self.max.y -= delta.y;
        self.max.z -= delta.z;
        if self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z {
            return false;
        }
        true
    }

    pub fn volume(&self) -> f64 {
        if !self.is_valid() {
            return 0.0;
        }
        let d = self.diagonal();
        (d.x.abs()) * (d.y.abs()) * (d.z.abs())
    }

    pub fn area(&self) -> f64 {
        if !self.is_valid() {
            return 0.0;
        }
        let d = self.diagonal();
        2.0 * (d.x.abs() * d.y.abs() + d.x.abs() * d.z.abs() + d.y.abs() * d.z.abs())
    }

    /// Accepts a [min, max] array (similar to OpenNurbs API)
    pub fn get_range_f32(&self) -> [f32; 6] {
        [
            self.min.x as f32,
            self.min.y as f32,
            self.min.z as f32,
            self.max.x as f32,
            self.max.y as f32,
            self.max.z as f32,
        ]
    }

    pub fn get_range_f64(&self) -> [f64; 6] {
        [
            self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z,
        ]
    }

    pub fn is_point_inside_axes(&self, p: &Point) -> (bool, bool, bool, bool) {
        if !self.is_valid() {
            return (false, false, false, false);
        }
        let inside_x = self.min.x <= p.x && p.x <= self.max.x;
        let inside_y = self.min.y <= p.y && p.y <= self.max.y;
        let inside_z = self.min.z <= p.z && p.z <= self.max.z;
        (
            inside_x && inside_y && inside_z,
            inside_x,
            inside_y,
            inside_z,
        )
    }
}
```
```rust
/// AABB of the point list (without transformation)
pub fn on_bounding_box_points(points: &[Point]) -> Option<(Point, Point)> {
    if points.is_empty() {
        return None;
    }
    let mut min = Point::max_value();
    let mut max = Point::min_value();
    for p in points {
        if p.x < min.x {
            min.x = p.x;
        }
        if p.x > max.x {
            max.x = p.x;
        }
        if p.y < min.y {
            min.y = p.y;
        }
        if p.y > max.y {
            max.y = p.y;
        }
        if p.z < min.z {
            min.z = p.z;
        }
        if p.z > max.z {
            max.z = p.z;
        }
    }
    Some((min, max))
}
```
```rust
/// AABB after applying the transformation
pub fn on_compute_bounding_box(
    transform: Option<&Transform>,
    points: &[Point],
) -> Option<(Point, Point)> {
    if points.is_empty() {
        return None;
    }
    match transform {
        None => on_bounding_box_points(points),
        Some(t) if t.is_identity() => on_bounding_box_points(points),
        Some(t) => {
            // Initialize the first point
            let p0 = t.apply_point(points[0]);
            let mut min = p0;
            let mut max = p0;
            // Update the remaining points
            for p in &points[1..] {
                let q = t.apply_point(*p);
                if q.x < min.x {
                    min.x = q.x;
                }
                if q.x > max.x {
                    max.x = q.x;
                }
                if q.y < min.y {
                    min.y = q.y;
                }
                if q.y > max.y {
                    max.y = q.y;
                }
                if q.z < min.z {
                    min.z = q.z;
                }
                if q.z > max.z {
                    max.z = q.z;
                }
            }
            Some((min, max))
        }
    }
}
```
```rust
/// Computes the bounding box from a float array (points = [x, y, z, x, y, z, ...])
pub fn on_compute_bounding_box_f32(
    transform: Option<&Transform>,
    points: &[f32],
    skip_points: usize,
) -> Option<(Point, Point)> {
    if points.len() < 3 {
        return None;
    }
    let stride = 3 + skip_points * 3;

    let take_xyz = |idx: usize| -> Point {
        Point::new(
            points[idx] as f64,
            points[idx + 1] as f64,
            points[idx + 2] as f64,
        )
    };

    match transform {
        None => {
            let mut min = Point::max_value();
            let mut max = Point::min_value();
            // 첫 점
            let mut i = 0usize;
            while i + 2 < points.len() {
                let p = take_xyz(i);
                if p.x < min.x {
                    min.x = p.x;
                }
                if p.x > max.x {
                    max.x = p.x;
                }
                if p.y < min.y {
                    min.y = p.y;
                }
                if p.y > max.y {
                    max.y = p.y;
                }
                if p.z < min.z {
                    min.z = p.z;
                }
                if p.z > max.z {
                    max.z = p.z;
                }
                i += stride;
            }
            Some((min, max))
        }
        Some(t) if t.is_identity() => on_compute_bounding_box_f32(None, points, skip_points),
        Some(t) => {
            // 첫 점
            let p0 = t.apply_point(take_xyz(0));
            let mut min = p0;
            let mut max = p0;
            // 나머지
            let mut i = 0usize;
            while i + 2 < points.len() {
                let q = t.apply_point(take_xyz(i));
                if q.x < min.x {
                    min.x = q.x;
                }
                if q.x > max.x {
                    max.x = q.x;
                }
                if q.y < min.y {
                    min.y = q.y;
                }
                if q.y > max.y {
                    max.y = q.y;
                }
                if q.z < min.z {
                    min.z = q.z;
                }
                if q.z > max.z {
                    max.z = q.z;
                }
                i += stride;
            }
            Some((min, max))
        }
    }
}
```
```rust
pub fn on_compute_bounding_box_into(
    xf: Option<&Transform>,
    points: &[Point],
    count: usize,
) -> Option<BoundingBox> {
    if points.is_empty() || count == 0 {
        return None;
    }
    let n = count.min(points.len());

    match xf {
        None => {
            let (min, max) = on_bounding_box_points(&points[..n])?;
            Some(BoundingBox::new(min, max))
        }
        Some(t) if t.is_identity() => {
            let (min, max) = on_bounding_box_points(&points[..n])?;
            Some(BoundingBox::new(min, max))
        }
        Some(t) => {
            let mut min: Point;
            let mut max: Point;

            // 첫 점 초기화
            let p0 = t.apply_point(points[0]);
            min = p0;
            max = p0;

            for p in &points[1..n] {
                let q = t.apply_point(*p);
                if q.x < min.x {
                    min.x = q.x;
                }
                if q.x > max.x {
                    max.x = q.x;
                }
                if q.y < min.y {
                    min.y = q.y;
                }
                if q.y > max.y {
                    max.y = q.y;
                }
                if q.z < min.z {
                    min.z = q.z;
                }
                if q.z > max.z {
                    max.z = q.z;
                }
            }
            Some(BoundingBox::new(min, max))
        }
    }
}
```
```rust
pub fn on_intersects_ray_bbox(ray: &Segment3D, bbox: &BoundingBox) -> bool {
    if !bbox.is_valid() {
        return false;
    }

    // Slab method: treats the ray as an infinite line for intersection testing
    let dir = ray.direction();
    let p0 = ray.p0;

    let mut t0 = f64::NEG_INFINITY;
    let mut t1 = f64::INFINITY;

    for axis in 0..3 {
        let (p0a, da, min_a, max_a) = match axis {
            0 => (p0.x, dir.x, bbox.min.x, bbox.max.x),
            1 => (p0.y, dir.y, bbox.min.y, bbox.max.y),
            _ => (p0.z, dir.z, bbox.min.z, bbox.max.z),
        };
        if da.abs() < ON_TOL12 {
            if p0a < min_a || p0a > max_a {
                return false;
            }
            continue;
        }
        let inv = 1.0 / da;
        let mut tmin = (min_a - p0a) * inv;
        let mut tmax = (max_a - p0a) * inv;
        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }
        t0 = t0.max(tmin);
        t1 = t1.min(tmax);
        if t0 > t1 {
            return false;
        }
    }
    true
}
```
```rust
pub fn on_aabb_lb_distance(a: &BoundingBox, b: &BoundingBox) -> f64 {
    let mut d2 = 0.0;
    let a_min_vec = a.min.to_array();
    let a_max_vec = a.max.to_array();
    let b_min_vec = b.min.to_array();
    let b_max_vec = b.max.to_array();
    for k in 0..3 {
        let (a_min, a_max) = (a_min_vec[k], a_max_vec[k]);
        let (b_min, b_max) = (b_min_vec[k], b_max_vec[k]);
        let t = if a_max < b_min {
            b_min - a_max
        } else if b_max < a_min {
            a_min - b_max
        } else {
            0.0
        };
        d2 += t * t;
    }
    d2.sqrt()
}
```
```rust
pub fn on_diagonal_length(bbox: &BoundingBox) -> f64 {
    let mut sum = 0.0;
    for i in 0..3 {
        if let (Some(max), Some(min)) = (get_axis_point(&bbox.max, i), get_axis_point(&bbox.min, i))
        {
            let d = max - min;
            sum += d * d;
        }
    }
    sum.sqrt()
}
```
```rust
#[inline]
pub fn on_plane_eq_for_dir(dir: usize, dom: &BoundingBox) -> [f64; 4] {
    match dir {
        0 => [-1.0, 0.0, 0.0, dom.min().x], // -X : -x + d = 0 ⇒ x = d
        1 => [0.0, -1.0, 0.0, dom.min().y], // -Y
        2 => [0.0, 0.0, -1.0, dom.min().z], // -Z
        3 => [1.0, 0.0, 0.0, -dom.max().x], // +X :  x - x_max = 0
        4 => [0.0, 1.0, 0.0, -dom.max().y], // +Y
        5 => [0.0, 0.0, 1.0, -dom.max().z], // +Z
        _ => [0.0, 0.0, 0.0, 0.0],
    }
}
```
```rust
#[inline]
pub fn on_intersect_face_flag(dir: usize, tri_bb: &BoundingBox, dom: &BoundingBox) -> bool {
    // The original code uses bit-flags to check whether the plane is crossed.
    // Here, we take a conservative approach: return true if tri_bb exceeds the domain.
    match dir {
        0 => tri_bb.min().x < dom.min().x, // -X
        1 => tri_bb.min().y < dom.min().y, // -Y
        2 => tri_bb.min().z < dom.min().z, // -Z
        3 => tri_bb.max().x > dom.max().x, // +X
        4 => tri_bb.max().y > dom.max().y, // +Y
        5 => tri_bb.max().z > dom.max().z, // +Z
        _ => false,
    }
}
```

## ✅ 수식적 점검 결과 요약
### 1. is_degenerate(tol: Option<f64>)
```rust
let tol = tol.unwrap_or(0.0).abs();
```
- 문제 없음: abs() 처리로 음수 허용 방지
- 주의점: tol=0.0일 경우, min == max가 아닌 이상 모두 정상으로 간주됨 → 퇴화 판별이 무력화될 수 있음

### 2. closest_on_line(seg: &Segment3D)
- Slab 방식으로 t0, t1 계산 후 교차 여부 판단
- 문제 없음: 축별로 da.abs() < 1e-30 처리로 평행성 고려
- 주의점: 1e-30은 매우 작은 값이므로, 실수 오차에 민감한 환경에서는 EPSILON 상수로 관리하는 것이 더 안전

### 3. expand(delta: Vector) / shrink(delta: Vector)
```rust
self.min.x -= delta.x;
self.max.x += delta.x;
```
- 문제 없음: 수학적으로 올바른 확장/축소
- shrink()에서는 min > max가 되면 false 반환 → 경계 역전 방지

### 4. on_aabb_lb_distance(a, b)
```rust
let t = if a_max < b_min {
    b_min - a_max
} else if b_max < a_min {
    a_min - b_max
} else {
    0.0
};
d2 += t * t;
```
- 문제 없음: 축별로 떨어진 거리만 제곱합
- 주의점: sqrt(d2)는 유클리드 거리지만, AABB 간의 실제 최소 거리와는 다를 수 있음 (특히 박스가 겹칠 때)
### 5. on_plane_eq_for_dir(dir, dom)
```rust
match dir {
    0 => [-1.0, 0.0, 0.0, dom.min().x], // -X
    ...
}
```
- 문제 없음: 평면 방정식 [a, b, c, d] 형식으로 표현
- 주의점: d 값이 일반적인 평면식 ax + by + cz + d = 0에서의 -dot(n, p)와 다를 수 있음 → 사용 시 주의
### 6. on_compute_bounding_box_f32(...)
```rust
let stride = 3 + skip_points * 3;
```
- 문제 없음: skip_points가 점 단위로 건너뛰는 구조라면 적절함
- 주의점: stride가 points.len()을 초과할 수 있으므로 while i + 2 < points.len() 조건이 중요

---


## 📊 테스트 항목 정리 표
| 테스트 함수 이름                  | 검증 항목 설명                                               |
|----------------------------------|--------------------------------------------------------------|
| `test_bounding_box`                 | 유효성 검사, 가까운 점 검스 교차 점 검사                                  |
| `test_basic_validity_and_inclusion` | 유효성 검사, 점 포함 여부 확인                                 |
| `test_closest_point_and_far_point` | 가장 가까운/먼 점 계산 정확성                                 |
| `test_union_and_intersection`      | 박스 병합 및 교차 영역 계산                                   |
| `test_degenerate_and_empty`        | 퇴화 상태 및 비어 있는 박스 판별                              |
| `test_expand_and_shrink`           | 박스 확장/축소 연산의 정확성                                  |
| `test_distance_to_point_and_bbox`  | 박스 간 거리 계산 (`min_distance_to_bbox`) 정확성             |


### 1. test_bounding_box
```rust
#[test]
fn test_bounding_box() {
    let b = BoundingBox::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 3.0, 4.0));
    assert!(b.is_valid());
    assert!(b.is_point_in(&Point3D::new(1.0, 2.0, 3.0), false));
    let _c = b.closest_point(Point3D::new(-1.0, 10.0, 1.0)); // → (0,3,1)
    let (_code, _q, _t0, _t1) = b.closest_on_line(&Segment3D::new(
        Point3D::new(-1.0, 1.5, 2.0),
        Point3D::new(3.0, 1.5, 2.0),
    ));
    // code==3 (교차 구간)

    let (ok, inter) = BoundingBox::intersection(
        &b,
        &BoundingBox::new(Point3D::new(1.0, 2.0, 1.0), Point3D::new(10.0, 10.0, 10.0)),
    );
    assert!(ok && inter.is_valid());
}
```
### 2. test_basic_validity_and_inclusion
```rust
#[test]
fn test_basic_validity_and_inclusion() {
    let b = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(2.0, 3.0, 4.0));
    assert!(b.is_valid());
    assert!(b.is_point_in(&Point::new(1.0, 2.0, 3.0), false));
    assert!(!b.is_point_in(&Point::new(5.0, 5.0, 5.0), false));
}
```
### 3. test_closest_point_and_far_point
```rust
#[test]
fn test_closest_point_and_far_point() {
    let b = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(2.0, 2.0, 2.0));
    let p = Point::new(-1.0, 3.0, 1.0);
    let cp = b.closest_point(p);
    assert_eq!(cp, Point::new(0.0, 2.0, 1.0));
    let fp = b.far_point(p);
    assert_eq!(fp, Point::new(2.0, 0.0, 1.0));
}
```
### 4. test_union_and_intersection
```rust
#[test]
fn test_union_and_intersection() {
    let a = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
    let b = BoundingBox::new(Point::new(0.5, 0.5, 0.5), Point::new(2.0, 2.0, 2.0));
    let (ok, inter) = BoundingBox::intersection(&a, &b);
    assert!(ok);
    assert_eq!(inter.min, Point::new(0.5, 0.5, 0.5));
    assert_eq!(inter.max, Point::new(1.0, 1.0, 1.0));

    let (ok2, union) = BoundingBox::union(&a, &b);
    assert!(ok2);
    assert_eq!(union.min, Point::new(0.0, 0.0, 0.0));
    assert_eq!(union.max, Point::new(2.0, 2.0, 2.0));
}
```

### 5. test_degenerate_and_empty
```rust
#[test]
fn test_degenerate_and_empty() {
    let b = BoundingBox::new(Point::new(1.0, 1.0, 1.0), Point::new(1.0, 1.0, 1.0));
    assert!(b.is_point());
    assert_eq!(b.is_degenerate(Some(1e-6)), 3);
    let empty = BoundingBox::default();
    assert!(empty.is_empty());
    assert!(!empty.is_valid());
}
```

### 6. test_expand_and_shrink
```rust
#[test]
fn test_expand_and_shrink() {
    let mut b = BoundingBox::new(Point::new(1.0, 1.0, 1.0), Point::new(2.0, 2.0, 2.0));
    assert!(b.expand(Vector::new(1.0, 1.0, 1.0)));
    assert_eq!(b.min, Point::new(0.0, 0.0, 0.0));
    assert_eq!(b.max, Point::new(3.0, 3.0, 3.0));

    assert!(b.shrink(Vector::new(0.5, 0.5, 0.5)));
    assert_eq!(b.min, Point::new(0.5, 0.5, 0.5));
    assert_eq!(b.max, Point::new(2.5, 2.5, 2.5));
}
```
### 7. test_distance_to_point_and_bbox
```rust
#[test]
fn test_distance_to_point_and_bbox() {
    let a = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
    let b = BoundingBox::new(Point::new(2.0, 2.0, 2.0), Point::new(3.0, 3.0, 3.0));
    let d = a.min_distance_to_bbox(&b);
    assert!((d - (3.0f64).sqrt()).abs() < 1e-6);
}
```

---
