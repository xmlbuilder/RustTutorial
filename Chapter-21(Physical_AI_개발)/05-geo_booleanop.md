#  Geo Booleanop

## 📌 AI 활용 사례
### 1. 데이터 전처리
- 이미지나 센서 데이터에서 추출된 영역(Polygon)을 서로 합치거나 빼서 **관심 영역(ROI)** 을 정의
- 예: 카메라/레이더가 감지한 영역을 합집합으로 병합 → 학습용 입력 데이터 생성
### 2. 라벨 정제
- 사람이 만든 라벨과 자동 생성된 라벨을 교집합/차집합으로 비교 → 정확한 Ground Truth 확보
- 예: AI가 탐지한 차량 영역과 실제 라벨의 교집합 → 정답률 계산
### 3. 데이터 증강
- 기존 다각형을 합집합/차집합으로 변형해 새로운 학습 샘플 생성
- 예: 건물 영역과 도로 영역을 XOR → **도로 위 건물 없는 영역** 데이터셋 생성
### 4. 이상 탐지
- 정상 영역과 새로운 입력 영역의 차집합을 계산 → 비정상 패턴 감지
- 예: 교통 흐름에서 차량이 있어서는 안 되는 영역에 들어왔는지 확인
### 5. 멀티센서 융합
- 카메라, 라이다, 레이더 등 서로 다른 센서가 감지한 영역을 Boolean 연산으로 결합
- 예: 라이다 감지 영역 ∩ 카메라 감지 영역 → 신뢰도 높은 학습 데이터

## ✅ 결론
geo_booleanop은 단순한 GIS 도구가 아니라, AI 학습 데이터 전처리와 증강, 라벨 정제, 이상 탐지에 직접 활용될 수 있습니다.  
특히 공간적 패턴을 다루는 AI(자율주행, 드론, 로보틱스, 스마트시티)에서 매우 유용.

**AI 파이프라인에서 geo_booleanop을 활용하는 단계별 예시** 를 그림으로 정리.

---


## 📦 기본 준비
```
# Cargo.toml
[dependencies]
geo = "0.28"
geo-booleanop = "0.4"
```

## 1️⃣ 두 다각형의 합집합 (union)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let union = poly1.union(&poly2);
    println!("Union result: {:?}", union);
}
```


## 2️⃣ 두 다각형의 교집합 (intersection)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let intersection = poly1.intersection(&poly2);
    println!("Intersection result: {:?}", intersection);
}
```


## 3️⃣ 두 다각형의 차집합 (difference)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let difference = poly1.difference(&poly2);
    println!("Difference result: {:?}", difference);
}
```


## 4️⃣ 두 다각형의 배타적 논리합 (xor)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let xor = poly1.xor(&poly2);
    println!("XOR result: {:?}", xor);
}
```
## ✅ 정리
- union → 두 영역 합치기
- intersection → 겹치는 부분만 추출
- difference → 한 영역에서 다른 영역 빼기
- xor → 겹치지 않는 부분만 추출

---

# 단계별 활용


## 단계별 활용 예시
### 1️⃣ 데이터 수집
- 센서(카메라, 라이다, 레이더) 또는 GIS 시스템에서 다각형 영역(Polygon) 데이터를 얻음
- 예: 차량 감지 → 차량의 위치를 다각형으로 표현
```rust
use geo::{polygon, Polygon};

let car_area: Polygon<f64> = polygon![
    (x: 0.0, y: 0.0),
    (x: 2.0, y: 0.0),
    (x: 2.0, y: 2.0),
    (x: 0.0, y: 2.0),
];
```
### 2️⃣ 라벨 데이터와 비교
- 사람이 만든 라벨(정답 영역)과 AI가 감지한 영역을 **교집합(intersection)** 으로 비교
- 겹치는 부분이 많을수록 정확도가 높음
```rust
use geo_booleanop::boolean::BooleanOp;

let ground_truth: Polygon<f64> = polygon![
    (x: 1.0, y: 1.0),
    (x: 3.0, y: 1.0),
    (x: 3.0, y: 3.0),
    (x: 1.0, y: 3.0),
];

let overlap = car_area.intersection(&ground_truth);
println!("Overlap area: {:?}", overlap);
```

### 3️⃣ 데이터 증강
- 기존 영역을 합집합(union) 또는 **차집합(difference)** 으로 변형해 새로운 학습 샘플 생성
- 예: 차량 영역 + 도로 영역 → **차량이 도로 위에 있는 데이터셋**
```rust
let road_area: Polygon<f64> = polygon![
    (x: -1.0, y: -1.0),
    (x: 4.0, y: -1.0),
    (x: 4.0, y: 4.0),
    (x: -1.0, y: 4.0),
];

let car_on_road = car_area.union(&road_area);
println!("Car on road area: {:?}", car_on_road);
```

### 4️⃣ 이상 탐지
- 정상 영역과 새로운 입력 영역의 **차집합(difference)** 을 계산 → 비정상 패턴 감지
- 예: 차량이 도로 밖에 있는 경우
```rust
let abnormal = car_area.difference(&road_area);
println!("Abnormal area (car outside road): {:?}", abnormal);
```

### 5️⃣ 멀티센서 융합
- 카메라와 라이다가 감지한 영역을 **교집합(intersection)** 으로 결합 → 신뢰도 높은 데이터 생성
- AI 학습 시 **센서 융합 데이터셋** 으로 활용
```rust
let camera_area = car_area.clone();
let lidar_area = ground_truth.clone();

let fused = camera_area.intersection(&lidar_area);
println!("Fused sensor area: {:?}", fused);
```

## ✅ 정리
- 교집합(intersection) → 정확도 평가, 센서 융합
- 합집합(union) → 데이터 증강, 영역 병합
- 차집합(difference) → 이상 탐지, 라벨 정제
- XOR → 겹치지 않는 영역 분석

### 흐름도

![GeoBoolean 흐름도](/image/geo_boolean_ai.png)

---

## 소스 코드
```rust
use crate::core::geom::{Point2D, Vector2D};
use crate::core::maths::on_cross_vec_2d;
use crate::core::prelude::Point3D;
use crate::core::segment2d::Segment2D;
use crate::core::transform::Transform;
use crate::core::types::Real;
use earcutr::earcut;
use geo::{BooleanOps, Coord, Polygon};

#[inline]
pub fn do_overlap_2d(min1: Point2D, max1: Point2D, min2: Point2D, max2: Point2D) -> bool {
    !(min2.x > max1.x || max2.x < min1.x || min2.y > max1.y || max2.y < min1.y)
}
```
```rust
#[inline]
pub fn do_overlap_or_touch_2d(min1: Point2D, max1: Point2D, min2: Point2D, max2: Point2D) -> bool {
    do_overlap_2d(min1, max1, min2, max2)
}
```
```rust
#[inline]
pub fn do_overlap_or_touch_2d_with_domain(
    min1: Point2D,
    max1: Point2D,
    min2: Point2D,
    max2: Point2D,
    _domain_diag: f64,
) -> bool {
    do_overlap_2d(min1, max1, min2, max2)
}
```
```rust
// i32 버전
#[inline]
pub fn do_overlap_2d_i32(
    min1: (i32, i32),
    max1: (i32, i32),
    min2: (i32, i32),
    max2: (i32, i32),
) -> bool {
    !(min2.0 > max1.0 || max2.0 < min1.0 || min2.1 > max1.1 || max2.1 < min1.1)
}
```
```rust
// ------------------------ 기본 기하 유틸 ------------------------
#[inline]
fn on_orientation(a: Point2D, b: Point2D, c: Point2D) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
```
```rust
#[inline]
fn on_segment(a: Point2D, b: Point2D, p: Point2D, tol: f64) -> bool {
    // p가 ab 선분 위(박스+수직거리 tol)인지
    if (p.x - a.x).min(b.x - a.x) - tol <= 0.0
        && (p.x - a.x).max(b.x - a.x) + tol >= 0.0
        && (p.y - a.y).min(b.y - a.y) - tol <= 0.0
        && (p.y - a.y).max(b.y - a.y) + tol >= 0.0
    {
        // 수직 거리
        let ab = Vector2D::new(b.x - a.x, b.y - a.y);
        let ap = Vector2D::new(p.x - a.x, p.y - a.y);
        let len2 = ab.x * ab.x + ab.y * ab.y;
        if len2 == 0.0 {
            return (ap.x * ap.x + ap.y * ap.y).sqrt() <= tol;
        }
        let t = (ap.x * ab.x + ap.y * ab.y) / len2;
        let t = t.clamp(0.0, 1.0);
        let proj = Point2D::new(a.x + t * ab.x, a.y + t * ab.y);
        let dx = p.x - proj.x;
        let dy = p.y - proj.y;
        (dx * dx + dy * dy).sqrt() <= tol
    } else {
        false
    }
}
```
```rust
#[inline]
pub fn on_seg_intersects(a1: Point2D, a2: Point2D, b1: Point2D, b2: Point2D, tol: f64) -> bool {
    // 일반적인 2D 선분 교차(+접촉) 판정
    let o1 = on_orientation(a1, a2, b1);
    let o2 = on_orientation(a1, a2, b2);
    let o3 = on_orientation(b1, b2, a1);
    let o4 = on_orientation(b1, b2, a2);

    if (o1 > 0.0 && o2 < 0.0 || o1 < 0.0 && o2 > 0.0)
        && (o3 > 0.0 && o4 < 0.0 || o3 < 0.0 && o4 > 0.0)
    {
        return true;
    }
    // 특수: 일직선/접촉
    (o1.abs() <= tol && on_segment(a1, a2, b1, tol))
        || (o2.abs() <= tol && on_segment(a1, a2, b2, tol))
        || (o3.abs() <= tol && on_segment(b1, b2, a1, tol))
        || (o4.abs() <= tol && on_segment(b1, b2, a2, tol))
}
```
```rust
#[inline]
pub fn on_polygon_area(pts: &[Point2D]) -> f64 {
    if pts.len() < 3 {
        return 0.0;
    }
    let mut s = 0.0;
    let n = pts.len();
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        s += a.x * b.y - b.x * a.y;
    }
    0.5 * s
}
```
```rust
#[inline]
pub fn on_is_polygon_convex(pts: &[Point2D]) -> bool {
    let n = pts.len();
    if n < 5 {
        return true;
    } // 거의 항상 convex 취급(원본 로직과 동일)
    let mut prev_sign = 0.0;
    let mut has_sign = false;
    for i in 0..n {
        let a = pts[(i + n - 1) % n];
        let b = pts[i];
        let c = pts[(i + 1) % n];
        let mut u = Vector2D::new(b.x - a.x, b.y - a.y);
        let mut v = Vector2D::new(c.x - b.x, c.y - b.y);
        let _ = u.normalize();
        let _ = v.normalize();
        let cross = u.x * v.y - u.y * v.x;
        if cross.abs() > 1e-12 {
            if has_sign && cross.signum() != (prev_sign as f64).signum() {
                return false;
            }
            has_sign = true;
            prev_sign = cross;
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_is_polygon_self_intersecting(pts: &[Point2D], tol: f64) -> bool {
    let n = pts.len();
    if n < 4 {
        return false;
    }
    for i in 0..n - 1 {
        let a1 = pts[i];
        let a2 = pts[i + 1];
        for j in i + 2..n - 1 {
            // 인접/공유 꼭짓점 제외
            if i == 0 && j == n - 2 {
                continue;
            }
            let b1 = pts[j];
            let b2 = pts[j + 1];
            if on_seg_intersects(a1, a2, b1, b2, tol) {
                return true;
            }
        }
    }
    false
}
```
```rust
#[inline]
pub fn on_is_polygon_degenerated(pts: &[Point2D], tol: f64) -> bool {
    // "어딘가 자가 교차가 있으면 퇴화"라는 원본 의미에 맞춤
    on_is_polygon_self_intersecting(pts, tol)
}
```
```rust
pub fn on_is_point_on_segment_2d(test: &Point2D, p0: &Point2D, p1: &Point2D) -> bool {
    on_is_point_on_segment_2d_with_domain(test, p0, p1, p0.distance(&p1))
}
```
```rust
// ------------------------ Polygon2D ------------------------
#[derive(Clone, Debug)]
pub struct Polygon2D {
    pub min: Point2D,
    pub max: Point2D,
    pub points: Vec<Point2D>,
    pub diagonal: f64,
}
```
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PointStatus {
    Outside,
    Inside,
    Onto,
}
```
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PolygonStatus {
    Out,
    In,
    On,
    Over,
}
```
```rust
impl Polygon2D {
    pub fn from_points(mut pts: Vec<Point2D>) -> Self {
        // 필요시 닫힌 다각형 보장 (마지막이 처음과 다르면 붙임)
        if pts.len() >= 3 && (pts.first() != pts.last()) {
            pts.push(*pts.first().unwrap());
        }
        let (min, max, diag) = Self::bounding_rect_of(&pts);
        Self {
            min,
            max,
            points: pts,
            diagonal: diag,
        }
    }
```
```rust
    #[inline]
    fn bounding_rect_of(pts: &[Point2D]) -> (Point2D, Point2D, f64) {
        let mut minx = f64::INFINITY;
        let mut miny = f64::INFINITY;
        let mut maxx = f64::NEG_INFINITY;
        let mut maxy = f64::NEG_INFINITY;
        for p in pts {
            if p.x < minx {
                minx = p.x;
            }
            if p.y < miny {
                miny = p.y;
            }
            if p.x > maxx {
                maxx = p.x;
            }
            if p.y > maxy {
                maxy = p.y;
            }
        }
        let dx = maxx - minx;
        let dy = maxy - miny;
        (
            Point2D::new(minx, miny),
            Point2D::new(maxx, maxy),
            (dx * dx + dy * dy).sqrt(),
        )
    }
```
```rust
    pub fn update_bounding_rect(&mut self) {
        let (min, max, diag) = Self::bounding_rect_of(&self.points);
        self.min = min;
        self.max = max;
        self.diagonal = diag;
    }
```
```rust
    #[allow(unused)]
    pub(crate) fn overlap_2d(
        a_min: &Point2D,
        a_max: &Point2D,
        b_min: &Point2D,
        b_max: &Point2D,
    ) -> bool {
        !(a_max.x < b_min.x || a_min.x > b_max.x || a_max.y < b_min.y || a_min.y > b_max.y)
    }
```
```rust
    pub fn is_valid(&self) -> bool {
        if self.points.len() < 4 {
            return false;
        }
        self.points.first() == self.points.last()
    }
```
```rust
    pub fn is_oriented_clockwise(&self) -> bool {
        on_polygon_area(&self.points) < 0.0
    }
```
```rust
    pub fn is_convex(&self) -> bool {
        on_is_polygon_convex(&self.points)
    }
```
```rust
    pub fn is_self_intersecting(&self) -> bool {
        on_is_polygon_self_intersecting(&self.points, 1e-12)
    }
```
```rust
    pub fn is_degenerated(&self) -> bool {
        on_is_polygon_degenerated(&self.points, 1e-12)
    }
```
```rust
    pub fn reverse(&mut self) {
        self.points.reverse();
    }
```
```rust
    pub fn point_inside_aabb(&self, p: &Point2D) -> bool {
        p.x > self.min.x && p.x < self.max.x && p.y > self.min.y && p.y < self.max.y
    }
```
```rust
    pub fn point_inside_or_on_aabb(&self, p: &Point2D) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
```
```rust
    pub fn is_point_inside(&self, p: &Point2D) -> bool {
        if !self.point_inside_aabb(p) {
            return false;
        }
        // ray casting
        let mut cnt = 0;
        let n = self.points.len();
        let mut j = n - 1;
        for i in 0..n {
            let a = self.points[i];
            let b = self.points[j];
            let cond = ((a.y <= p.y && p.y < b.y) || (b.y <= p.y && p.y < a.y))
                && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x;
            if cond {
                cnt += 1;
            }
            j = i;
        }
        (cnt & 1) == 1
    }
```
```rust
    pub fn is_point_inside_with_tol(&self, p: &Point2D, domain_size: f64) -> PointStatus {
        if !self.point_inside_or_on_aabb(p) {
            return PointStatus::Outside;
        }
        // 경계 위?
        for w in self.points.windows(2) {
            if on_is_point_on_segment_2d_with_domain(p, &w[0], &w[1], domain_size) {
                return PointStatus::Onto;
            }
        }
        if self.is_point_inside(p) {
            PointStatus::Inside
        } else {
            PointStatus::Outside
        }
    }
```
```rust
    pub fn is_polygon_inside_with_tol(&self, subj: &Polygon2D, domain_size: f64) -> PolygonStatus {
        // 1) 스케일 → 절대 허용오차로 변환 (필요시 상수 조정 가능)
        //    domain_size 가 0에 가까우면 최소 바닥값을 둬서 안정화
        let scale = domain_size.max(1.0);
        let eps = 1e-9 * scale;

        // 2) AABB 겹침(또는 접촉) 체크 — 여기서는 여전히 스케일 값을 써도 OK
        if !do_overlap_or_touch_2d_with_domain(self.min, self.max, subj.min, subj.max, scale) {
            return PolygonStatus::Out;
        }

        // 3) 에지 교차/접촉 체크 → 교차/접촉 있으면 On
        let n1 = self.points.len();
        let n2 = subj.points.len();
        for i in 0..n2 - 1 {
            let s = (subj.points[i], subj.points[i + 1]);
            for j in 0..n1 - 1 {
                let t = (self.points[j], self.points[j + 1]);
                // 기존에는 domain_size를 그대로 넘겼을 가능성 → eps로 교체
                if on_seg_intersects(s.0, s.1, t.0, t.1, eps) {
                    return PolygonStatus::On;
                }
            }
        }

        // 4) 포함성 판단 (정점/중점 검사에 모두 eps 사용)
        let mut inside_cnt = 0;
        let mut outside_cnt = 0;
        let mut onto_cnt = 0;
        let mut mid_inside_cnt = 0;
        let mut mid_outside_cnt = 0;

        for i in 0..n2 - 1 {
            match self.is_point_inside_with_tol(&subj.points[i], eps) {
                PointStatus::Inside => inside_cnt += 1,
                PointStatus::Outside => outside_cnt += 1,
                PointStatus::Onto => onto_cnt += 1,
            }

            // 중점도 검사
            let a = subj.points[i];
            let b = subj.points[i + 1];
            let mid = Point2D::new(0.5 * (a.x + b.x), 0.5 * (a.y + b.y));
            match self.is_point_inside_with_tol(&mid, eps) {
                PointStatus::Inside => mid_inside_cnt += 1,
                PointStatus::Outside => mid_outside_cnt += 1,
                PointStatus::Onto => {
                    // 중점이 경계 위면, 내부/외부 중간 상태로 취급 → 여기선 outside로 카운트하지 않음
                }
            }
        }

        // 5) 규칙
        //  - 정점 중 Inside/Outside 섞이면 경계 걸침 → On
        if inside_cnt > 0 && outside_cnt > 0 {
            return PolygonStatus::On;
        }
        //  - 모든 정점이 Inside 또는 Onto 이고, 중점도 Outside가 없으면 완전 포함 → In
        if outside_cnt == 0 && mid_outside_cnt == 0 {
            return PolygonStatus::In;
        }

        //  - self가 subj 내부로 들어간 경우 (Over)
        let mut self_inside_subj = false;
        for i in 0..n1 - 1 {
            if subj.is_point_inside_with_tol(&self.points[i], eps) == PointStatus::Inside {
                self_inside_subj = true;
                break;
            }
            let a = self.points[i];
            let b = self.points[i + 1];
            let mid = Point2D::new(0.5 * (a.x + b.x), 0.5 * (a.y + b.y));
            if subj.is_point_inside_with_tol(&mid, eps) == PointStatus::Inside {
                self_inside_subj = true;
                break;
            }
        }
        if outside_cnt + onto_cnt == n2 - 1 && self_inside_subj && mid_inside_cnt == 0 {
            return PolygonStatus::Over;
        }

        //  - 나머지 경계/겹침 케이스
        if (outside_cnt > 0 && mid_inside_cnt > 0)
            || (mid_outside_cnt < n2 - 1 && mid_inside_cnt > 0)
        {
            return PolygonStatus::On;
        }

        if outside_cnt > 0 {
            PolygonStatus::Out
        } else {
            PolygonStatus::In
        }
    }
```
```rust
    pub fn to_geo(&self) -> Polygon<f64> {
        let coords: Vec<Coord<f64>> = self
            .points
            .iter()
            .map(|p| Coord { x: p.x, y: p.y })
            .collect();
        Polygon::new(coords.into(), vec![])
    }
```
```rust
    pub fn union(&self, other: &Polygon2D) -> Vec<Polygon2D> {
        let a = self.to_geo();
        let b = other.to_geo();
        let result = a.union(&b);
        result
            .into_iter()
            .map(|poly| {
                Polygon2D::from_points(
                    poly.exterior()
                        .points()
                        .map(|p| Point2D::new(p.x(), p.y()))
                        .collect(),
                )
            })
            .collect()
    }
```
```rust
    pub fn difference(&self, other: &Polygon2D) -> Vec<Polygon2D> {
        let a = self.to_geo();
        let b = other.to_geo();
        let result = a.difference(&b);
        result
            .into_iter()
            .map(|poly| {
                Polygon2D::from_points(
                    poly.exterior()
                        .points()
                        .map(|p| Point2D::new(p.x(), p.y()))
                        .collect(),
                )
            })
            .collect()
    }
```
```rust
    pub fn xor(&self, other: &Polygon2D) -> Vec<Polygon2D> {
        let a = self.to_geo();
        let b = other.to_geo();
        let result = a.xor(&b);
        result
            .into_iter()
            .map(|poly| {
                Polygon2D::from_points(
                    poly.exterior()
                        .points()
                        .map(|p| Point2D::new(p.x(), p.y()))
                        .collect(),
                )
            })
            .collect()
    }
```
```rust
    pub fn difference_multi(&self, other: &Polygon2D) -> Vec<Vec<Polygon2D>> {
        let a = self.to_geo();
        let b = other.to_geo();
        let result = a.difference(&b);

        let mut out = vec![];

        for poly in result {
            let mut group = vec![];

            // 외곽선
            let exterior = poly
                .exterior()
                .points()
                .map(|p| Point2D::new(p.x(), p.y()))
                .collect();
            group.push(Polygon2D::from_points(exterior));

            // 내부 링들
            for hole in poly.interiors() {
                let pts = hole.points().map(|p| Point2D::new(p.x(), p.y())).collect();
                group.push(Polygon2D::from_points(pts));
            }

            out.push(group);
        }

        out
    }
```
```rust
    pub fn intersection(&self, other: &Polygon2D) -> Vec<Polygon2D> {
        let a = self.to_geo();
        let b = other.to_geo();
        let result = a.intersection(&b);
        result
            .into_iter()
            .map(|poly| {
                Polygon2D::from_points(
                    poly.exterior()
                        .points()
                        .map(|p| Point2D::new(p.x(), p.y()))
                        .collect(),
                )
            })
            .collect()
    }
```
```rust
    pub fn is_polygon_inside_fast(&self, subj: &Polygon2D) -> bool {
        if !do_overlap_2d(self.min, self.max, subj.min, subj.max) {
            return false;
        }
        let m = subj.points.len();
        let mut num = 0;
        for i in 0..m - 1 {
            if self.is_point_inside(&subj.points[i]) {
                num += 1;
            }
        }
        num == m - 1
    }
```
```rust
    pub fn transform_by(&mut self, xf: &Transform) {
        for p in &mut self.points {
            let p3 = Point3D {
                x: p.x,
                y: p.y,
                z: 0.0,
            };
            let q3 = xf.apply_point(&p3);
            p.x = q3.x;
            p.y = q3.y;
        }
        self.update_bounding_rect();
    }
```
```rust
    pub fn intersect_with_segment(&self, seg: &Segment2D) -> bool {
        for w in self.points.windows(2) {
            if on_seg_intersects(w[0], w[1], seg.p0, seg.p1, 1e-12) {
                return true;
            }
        }
        false
    }
```
```rust
    /// 폴리곤과 선분의 교차점(중복 근접 제거)
    pub fn intersections_with(&self, seg: &Segment2D) -> Vec<Point2D> {
        let mut out = Vec::<Point2D>::new();
        let mut last: Option<Point2D> = None;

        for w in self.points.windows(2) {
            if let Some(p) = segment_intersection_point(w[0], w[1], seg.p0, seg.p1) {
                let accept = if let Some(lp) = last {
                    (p.x - lp.x).hypot(p.y - lp.y) > 1e-12
                } else {
                    true
                };
                if accept {
                    out.push(p);
                    last = Some(p);
                }
            }
        }
        if out.len() > 1 {
            if let (Some(first), Some(last_p)) = (out.first(), out.last()) {
                if (first.x - last_p.x).hypot(first.y - last_p.y) < 1e-12 {
                    out.pop();
                }
            }
        }
        out
    }
```
```rust
    fn flatten(points: &[Point2D]) -> Vec<f64> {
        points.iter().flat_map(|p| vec![p.x, p.y]).collect()
    }
```
```rust
    pub fn triangulate_with_holes(&self, holes: &[Polygon2D]) -> Vec<[Point2D; 3]> {
        let exterior_flat = Self::flatten(&self.points);
        let mut holes_flat = vec![];
        let mut hole_indices = vec![];

        let mut offset = exterior_flat.len();
        for hole in holes {
            hole_indices.push(offset / 2); // index 기준은 점 개수
            let flat = Self::flatten(&hole.points);
            offset += flat.len();
            holes_flat.extend(flat);
        }

        let mut all_coords = exterior_flat;
        all_coords.extend(holes_flat);

        let indices = earcut(&all_coords, &hole_indices, 2).expect("Triangulation failed");

        let points: Vec<Point2D> = all_coords
            .chunks(2)
            .map(|chunk| Point2D::new(chunk[0], chunk[1]))
            .collect();

        let outer_sign = Self::polygon_normal(&points[..=3]).signum();

        let triangles: Vec<[Point2D; 3]> = indices
            .chunks(3)
            .map(|idx| {
                let a = points[idx[0]];
                let b = points[idx[1]];
                let c = points[idx[2]];
                let area = Self::triangle_area(a, b, c);
                if area.signum() != outer_sign {
                    [a, c, b] // flip
                } else {
                    [a, b, c]
                }
            })
            .collect();
        triangles
    }
```
```rust
    fn polygon_normal(points: &[Point2D]) -> f64 {
        let mut sum = 0.0;
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];
            sum += (p2.x - p1.x) * (p2.y + p1.y);
        }
        sum // > 0이면 시계, < 0이면 반시계
    }

    fn triangle_area(a: Point2D, b: Point2D, c: Point2D) -> f64 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
}
```
```rust
// 선분 교차점(있으면) 계산: 두 무한직선의 교차를 구해 t∈[0,1] 검사
fn segment_intersection_point(
    a1: Point2D,
    a2: Point2D,
    b1: Point2D,
    b2: Point2D,
) -> Option<Point2D> {
    let r = Vector2D::new(a2.x - a1.x, a2.y - a1.y);
    let s = Vector2D::new(b2.x - b1.x, b2.y - b1.y);
    let denom = r.x * s.y - r.y * s.x;
    if denom.abs() < 1e-15 {
        return None;
    } // 평행/일치 처리 생략
    let qp = Vector2D::new(b1.x - a1.x, b1.y - a1.y);
    let t = (qp.x * s.y - qp.y * s.x) / denom;
    let u = (qp.x * r.y - qp.y * r.x) / denom;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(Point2D::new(a1.x + t * r.x, a1.y + t * r.y))
    } else {
        None
    }
}
```
```rust
impl Default for Polygon2D {
    fn default() -> Self {
        Polygon2D {
            min: Point2D::MAX,
            max: Point2D::MIN,
            points: vec![],
            diagonal: 0.0,
        }
    }
}
```
```rust
pub fn on_is_point_on_segment_2d_with_domain(
    test: &Point2D,
    p0: &Point2D,
    p1: &Point2D,
    domain_diag: f64,
) -> bool {
    let v1x = p1.x - p0.x;
    let v1y = p1.y - p0.y;
    let d = v1x * v1x + v1y * v1y;
    if d == 0.0 {
        return (test.x - p0.x).hypot(test.y - p0.y) <= f64::EPSILON;
    }

    let v2x = test.x - p0.x;
    let v2y = test.y - p0.y;
    let v3x = test.x - p1.x;
    let v3y = test.y - p1.y;

    let num1 = {
        let lhs = v2x * v2x + v2y * v2y;
        let rhs = v3x * v3x + v3y * v3y;
        if lhs >= rhs {
            1.0 + (v3x * v1x + v3y * v1y) / d
        } else {
            (v2x * v1x + v2y * v1y) / d
        }
    };

    let num2 = d.sqrt();
    let num3 = if (num1 * num2).abs() / domain_diag < f64::EPSILON.sqrt() {
        1
    } else {
        0
    };
    let flag = (1.0 - num1).abs() * num2 / domain_diag < f64::EPSILON.sqrt();
    if num3 == 0 && !flag && (num1 < -1e-9 || num1 > 1.000000001) {
        return false;
    }

    let t = (v1x * v2x + v1y * v2y) / d;
    let px = p0.x + t * v1x;
    let py = p0.y + t * v1y;
    let dx = test.x - px;
    let dy = test.y - py;
    (dx * dx + dy * dy).sqrt() / 2.0 < f64::EPSILON.sqrt()
}
```
```rust
pub fn on_point_in_rectangle(test: Point2D, ll: Point2D, ur: Point2D) -> PointStatus {
    // 변 위인지 먼저 체크
    let pts = [
        ll,
        Point2D::new(ur.x, ll.y),
        ur,
        Point2D::new(ll.x, ur.y),
        ll,
    ];
    let diag = ((ur.x - ll.x).hypot(ur.y - ll.y)).abs();
    for w in pts.windows(2) {
        if on_is_point_on_segment_2d_with_domain(&test, &w[0], &w[1], diag) {
            return PointStatus::Onto;
        }
    }
    if test.x > ll.x && test.x < ur.x && test.y > ll.y && test.y < ur.y {
        PointStatus::Inside
    } else {
        PointStatus::Outside
    }
}
```
```rust
pub fn on_point_in_rect_open(test: Point2D, ll: Point2D, ur: Point2D) -> bool {
    test.x > ll.x && test.x < ur.x && test.y > ll.y && test.y < ur.y
}

```
```rust
/* -------- Point-in-Triangle Tests (2D) -------- */
pub fn on_point_in_triangle_2d(test: Point2D, a: Point2D, b: Point2D, c: Point2D) -> bool {
    on_point_in_triangle_2d_scalars(test.x, test.y, a.x, a.y, b.x, b.y, c.x, c.y)
}

```
```rust
pub fn on_point_in_triangle_2d_scalars(
    xp: f64,
    yp: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> bool {
    let a31x = x3 - x1;
    let a31y = y3 - y1;
    let a21x = x2 - x1;
    let a21y = y2 - y1;
    let ap1x = xp - x1;
    let ap1y = yp - y1;
    let dot1 = a31x * a31x + a31y * a31y;
    let dot2 = a31x * a21x + a31y * a21y;
    let dot3 = a31x * ap1x + a31y * ap1y;
    let dot4 = a21x * a21x + a21y * a21y;
    let dot5 = a21x * ap1x + a21y * ap1y;
    let denom = dot1 * dot4 - dot2 * dot2;
    if denom == 0.0 {
        return false;
    }
    let inv = 1.0 / denom;
    let u = (dot4 * dot3 - dot2 * dot5) * inv;
    let v = (dot1 * dot5 - dot2 * dot3) * inv;
    u >= 0.0 && v >= 0.0 && (u + v) <= 1.0
}

```
```rust
fn on_is_left_of(a: &Point2D, b: &Point2D) -> bool {
    a.x < b.x || (a.x == b.x && a.y < b.y)
}

```
```rust
pub fn on_quick_hull_2d(v: Vec<Point2D>) -> Vec<Point2D> {
    if v.len() <= 3 {
        return v;
    }

    let a = *v
        .iter()
        .min_by(|p, q| {
            on_is_left_of(p, q)
                .then_some(std::cmp::Ordering::Less)
                .unwrap_or(std::cmp::Ordering::Greater)
        })
        .unwrap();
    let b = *v
        .iter()
        .max_by(|p, q| {
            on_is_left_of(p, q)
                .then_some(std::cmp::Ordering::Less)
                .unwrap_or(std::cmp::Ordering::Greater)
        })
        .unwrap();
```
```rust
    fn dist(a: Point2D, b: Point2D, p: Point2D) -> f64 {
        ((b.x - a.x) * (a.y - p.y) - (b.y - a.y) * (a.x - p.x)).abs() / ((b - a).length())
    }
```
```rust
    fn farthest(a: Point2D, b: Point2D, vv: &[Point2D]) -> usize {
        let mut idx = 0usize;
        let mut dm = dist(a, b, vv[0]);
        for (i, &pt) in vv.iter().enumerate().skip(1) {
            let d = dist(a, b, pt);
            if d > dm {
                dm = d;
                idx = i;
            }
        }
        idx
    }
```
```rust
    fn side(a: Point2D, b: Point2D, p: Point2D) -> Real {
        on_cross_vec_2d(a, b, p)
    }

    fn recurse(vv: Vec<Point2D>, a: Point2D, b: Point2D, hull: &mut Vec<Point2D>) {
        if vv.is_empty() {
            return;
        }
        let idx = farthest(a, b, &vv);
        let f = vv[idx];

        let mut left = Vec::new();
        for &p in &vv {
            if side(a, f, p) > 0.0 {
                left.push(p);
            }
        }
        recurse(left, a, f, hull);

        hull.push(f);

        let mut right = Vec::new();
        for &p in &vv {
            if side(f, b, p) > 0.0 {
                right.push(p);
            }
        }
        recurse(right, f, b, hull);
    }

    // 좌/우 분리
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &p in &v {
        if side(a, b, p) > 0.0 {
            left.push(p);
        } else {
            right.push(p);
        }
    }

    let mut hull = Vec::new();
    hull.push(a);
    recurse(left, a, b, &mut hull);
    hull.push(b);
    recurse(right, b, a, &mut hull);
    hull
}
```
```rust
pub fn on_monotone_chain_2d(mut v: Vec<Point2D>) -> Vec<Point2D> {
    if v.len() <= 1 {
        return v;
    }
    v.sort_by(|a, b| {
        if on_is_left_of(a, b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut lower: Vec<Point2D> = Vec::new();
    for &p in &v {
        while lower.len() >= 2 {
            let n = lower.len();
            if on_cross_vec_2d(lower[n - 2], lower[n - 1], p) >= 0.0 {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(p);
    }

    let mut upper: Vec<Point2D> = Vec::new();
    for &p in v.iter().rev() {
        while upper.len() >= 2 {
            let n = upper.len();
            if on_cross_vec_2d(upper[n - 2], upper[n - 1], p) >= 0.0 {
                upper.pop();
            } else {
                break;
            }
        }
        upper.push(p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}
```
```rust
pub fn on_is_point_inside_box(
    p: Point3D,
    min: Point3D,
    max: Point3D,
    inflate: f64,
    open: bool,
) -> bool {
    if open {
        return p.x > min.x - inflate
            && p.y > min.y - inflate
            && p.z > min.z - inflate
            && p.x < max.x + inflate
            && p.y < max.y + inflate
            && p.z < max.z + inflate;
    } else {
        return p.x >= min.x - inflate
            && p.y >= min.y - inflate
            && p.z >= min.z - inflate
            && p.x <= max.x + inflate
            && p.y <= max.y + inflate
            && p.z <= max.z + inflate;
    }
}
```
```rust
pub fn on_intersection_polygon2d(a: &Polygon2D, b: &Polygon2D) -> Vec<Polygon2D> {

    a.intersection(b)
}
```
```rust
pub fn on_union_polygon2d(a: &Polygon2D, b: &Polygon2D) -> Vec<Polygon2D> {

    a.union(b)
}
```
```rust
pub fn on_subtract_polygon2d(a: &Polygon2D, b: &Polygon2D) -> Vec<Polygon2D> {

    a.difference(b)
}
```
```rust
pub fn on_xor_polygon2d(a: &Polygon2D, b: &Polygon2D) -> Vec<Polygon2D> {
    a.xor(b)
}
```

---



