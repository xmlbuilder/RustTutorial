# Curve Direction
- 이 코드는 두 개의 B-스플라인 곡선(BSplineCurve)의 방향 관계를 판별하는 유틸리티입니다.  
- 즉, 곡선 A와 곡선 B가 서로 어떤 방향으로 이어져 있는지를 검사해서 CurveDirection 열거형으로 반환합니다.

## 🧩 코드의 주요 흐름
### 1. CurveDirection 열거형
```rust
pub enum CurveDirection {
    Forward,  // -> ->
    Backward, // <- <-
    Facing,   // -> <-
    Opposite, // <- ->
}
```

- 두 곡선의 끝점 방향 관계를 네 가지 경우로 정의:
- Forward: A 끝점 → B 시작점 (같은 방향)
- Backward: A 시작점 → B 끝점 (같은 방향, 반대쪽에서 연결)
- Facing: A 끝점 → B 끝점 (서로 마주봄)
- Opposite: A 시작점 → B 시작점 (서로 반대 방향에서 출발)

### 2. 도메인 검사
```rust
let pa = a.degree;
let na = a.control_points.len().saturating_sub(1);
let pb = b.degree;
let nb = b.control_points.len().saturating_sub(1);
if na + pa + 1 > a.knots.len() || nb + pb + 1 > b.knots.len() {
    return None;
}
```

- 곡선의 차수(degree)와 제어점 개수(control_points)를 이용해 knot 벡터가 유효한지 검사
- 잘못된 곡선이면 None 반환

### 3. 시작점과 끝점 계산
```rust
let (au0, au1) = (a.knots[pa], a.knots[na + 1]);
let (bu0, bu1) = (b.knots[pb], b.knots[nb + 1]);
```

- 각 곡선의 파라미터 구간 시작/끝을 가져옴
- open clamped knot이면 제어점의 첫/마지막을 그대로 사용
- 아니면 eval_point로 실제 곡선 좌표를 계산

### 4. 길이 스케일 기반의 허용 오차 설정
```rust
let la = (a1 - a0).length();
let lb = (b1 - b0).length();
let scale = la.max(lb).max(1.0);
let eps = (1e-10 * scale).max(epsilon_abs);
let eps2 = eps * eps;
```

- 곡선 길이를 기반으로 상대/절대 오차를 설정
- 작은 수치 오차에도 견고하게 동작하도록 스케일링

### 5. 네 가지 후보 관계 계산
```rust
let candidates = [
    (d2(a1, b0), CurveDirection::Forward),  // -> ->
    (d2(a0, b1), CurveDirection::Backward), // <- <-
    (d2(a1, b1), CurveDirection::Facing),   // -> <-
    (d2(a0, b0), CurveDirection::Opposite), // <- ->
];
```

- 각 경우에 대해 두 점 사이의 거리 제곱(d2)을 계산
- 예: A 끝점과 B 시작점이 가까우면 Forward

### 6. 가장 가까운 관계 선택
```rust
let best = candidates
    .iter()
    .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(Ordering::Equal))?;
```

- 거리 제곱이 가장 작은 후보를 선택
- 그 거리가 허용 오차 이하이면 해당 방향 반환, 아니면 None

## 🎯 요약
- 역할: 두 B-스플라인 곡선의 끝점 방향 관계를 판별
- 출력: CurveDirection (Forward, Backward, Facing, Opposite) 또는 None
- 특징:
- open clamped knot 여부에 따라 끝점 계산 방식 달라짐
- 길이 기반 오차 스케일링으로 안정성 확보
- 네 가지 후보 관계 중 가장 가까운 것을 선택
- 👉 쉽게 말하면, 이 함수는 두 곡선이 서로 이어질 때 방향이 같은지, 반대인지,
    마주보는지를 판별하는 도우미.


---

# Topology 연결

- Topology를 구성할 때 곡선(Curve) 연결은 핵심적인 단계.  
- 지금까지 만든 CurveDirection::detect 같은 함수는 바로 이 곡선 간의 연결 관계를 판별해서
    토폴로지 그래프를 만들 때 활용할 수 있습니다.

## 🧩 Topology 구성에서 Curve 연결의 역할
- 곡선의 끝점 매칭
    - 두 곡선이 같은 점에서 만나면 연결 가능 후보가 됨
    - CurveDirection::detect는 이때 방향까지 판별해서 "같은 방향으로 이어지는지", "반대 방향인지"를 알려줌
- Edge 정의
    - Topology에서는 곡선을 Edge로 보고, 끝점은 Vertex로 정의
    - 곡선 연결은 결국 Vertex 공유 여부로 결정됨
    - 예: a1 ≈ b0이면 Edge(A)와 Edge(B)가 같은 Vertex를 공유
- Graph 구조화
    - Vertex(Node)와 Edge(곡선)를 연결해서 그래프를 구성
    - 이 그래프가 Topology의 기본 뼈대가 됨
    - 방향 정보(Forward, Backward, Facing, Opposite)는 Edge의 방향성을 결정하는 데 사용

## ✅ 활용 예시 (개념 코드)
```rust
pub struct Vertex {
    pub point: Point3D,
}
```
```rust
pub struct Edge {
    pub curve: BSplineCurve,
    pub start: Vertex,
    pub end: Vertex,
}
```
```rust
pub struct Topology {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}
```
```rust
impl Topology {
    pub fn connect_curves(&mut self, a: &BSplineCurve, b: &BSplineCurve) {
        if let Some(direction) = CurveDirection::detect(a, b, 1e-6) {
            match direction {
                CurveDirection::Forward => {
                    // a 끝점 == b 시작점 → 같은 Vertex 공유
                }
                CurveDirection::Backward => {
                    // a 시작점 == b 끝점
                }
                CurveDirection::Facing => {
                    // a 끝점 == b 끝점
                }
                CurveDirection::Opposite => {
                    // a 시작점 == b 시작점
                }
            }
        }
    }
}
```


## 🎯 요약
- Topology에서 곡선 연결은 Vertex와 Edge를 정의하는 핵심
- CurveDirection::detect는 곡선 간의 방향 관계를 판별해 연결 여부와 방향성을 결정
- 이를 기반으로 **그래프 구조(Topology)** 를 만들면, 이후 Surface나 Solid 같은 상위 구조로 확장 가능

---

## 소스 코드
```rust
use std::cmp::Ordering;
use crate::core::basis::on_is_open_clamped_knot;
use crate::core::point_ops::CPointOps;
use crate::core::prelude::{Degree, NurbsCurve, Point3D};

#[derive(Clone, Copy, Debug)]
pub enum CurveDirection {
    Forward,  // -> ->
    Backward, // <- <-
    Facing,   // -> <-
    Opposite, // <- ->
}
```
```rust
impl CurveDirection {
    pub fn detect(a: &NurbsCurve, b: &NurbsCurve, epsilon_abs: f64) -> Option<Self> {
        // 도메인
        let pa = a.degree as usize;
        let na = a.ctrl.len().saturating_sub(1);
        let pb = b.degree as usize;
        let nb = b.ctrl.len().saturating_sub(1);
        if na + pa + 1 > a.kv.knots.len() || nb + pb + 1 > b.kv.knots.len() {
            return None;
        }

        let (au0, au1) = (a.kv.knots[pa], a.kv.knots[na + 1]);
        let (bu0, bu1) = (b.kv.knots[pb], b.kv.knots[nb + 1]);

        // clamped 면 끝점은 control poin t로 스냅
        let a0 = if on_is_open_clamped_knot(&a.kv.knots, a.degree as usize, 1e-12) {
            a.ctrl.first().unwrap().to_point()
        } else {
            a.eval_point(au0)
        };
        let a1 = if on_is_open_clamped_knot(&a.kv.knots, a.degree as usize, 1e-12) {
            a.ctrl.last().unwrap().to_point()
        } else {
            a.eval_point(au1)
        };

        let b0 = if on_is_open_clamped_knot(&b.kv.knots, b.degree as usize, 1e-12) {
            b.ctrl.first().unwrap().to_point()
        } else {
            b.eval_point(bu0)
        };
        let b1 = if on_is_open_clamped_knot(&b.kv.knots, b.degree as usize, 1e-12) {
            b.ctrl.last().unwrap().to_point()
        } else {
            b.eval_point(bu1)
        };

        // 스케일 인식 절대/상대 tol (길이 스케일 기반)
        let la = (a1 - a0).length();
        let lb = (b1 - b0).length();
        let scale = la.max(lb).max(1.0);
        let eps = (1e-10 * scale).max(epsilon_abs); // 상대 tol + 절대 하한
        let eps2 = eps * eps;

        let d2 = |p: Point3D, q: Point3D| -> f64 { (p - q).length_squared() };

        let candidates = [
            (d2(a1, b0), CurveDirection::Forward),  // -> ->
            (d2(a0, b1), CurveDirection::Backward), // <- <-
            (d2(a1, b1), CurveDirection::Facing),   // -> <-
            (d2(a0, b0), CurveDirection::Opposite), // <- ->
        ];

        let best = candidates
            .iter()
            .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(Ordering::Equal))?;

        if best.0 <= eps2 { Some(best.1) } else { None }
    }
}
```

---

## 🛠 테스트 코드 예시
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::curve_direction::CurveDirection;
    use nurbslib::core::geom::Point4D;
    use nurbslib::core::knot::{on_clamped_uniform_knot_vector, KnotVector};
    use nurbslib::core::prelude::{Degree, Interval, NurbsCurve, Point3D};
```
```rust
    fn make_line_curve(p0: Point3D, p1: Point3D) -> NurbsCurve {
        // 단순 직선 NURBS (degree=1, open clamped)
        let degree = 1;
        let ctrl = vec![Point4D::new(p0.x, p0.y, p0.z, 1.0), Point4D::new(p1.x, p1.y, p1.z, 1.0)];
        let kv = on_clamped_uniform_knot_vector(degree, ctrl.len());
        println!("kv {:?}", kv);
        NurbsCurve { dimension: 3, degree : degree as Degree ,
            ctrl, kv: KnotVector {knots : kv}, domain: Interval {t0: 0.0, t1: 1.0} }
    }
```
```rust
    #[test]
    fn test_forward_direction() {
        let a = make_line_curve(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let b = make_line_curve(Point3D::new(1.0, 0.0, 0.0), Point3D::new(2.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-6);
        assert_eq!(dir, Some(CurveDirection::Forward));
    }
```
```rust
    #[test]
    fn test_backward_direction() {
        let a = make_line_curve(Point3D::new(1.0, 0.0, 0.0), Point3D::new(0.0, 0.0, 0.0));
        let b = make_line_curve(Point3D::new(0.0, 0.0, 0.0), Point3D::new(-1.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-6);
        assert_eq!(dir, Some(CurveDirection::Backward));
    }
```
```rust
    #[test]
    fn test_facing_direction() {
        let a = make_line_curve(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let b = make_line_curve(Point3D::new(2.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-6);
        assert_eq!(dir, Some(CurveDirection::Facing));
    }
```
```rust
    #[test]
    fn test_opposite_direction() {
        let a = make_line_curve(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let b = make_line_curve(Point3D::new(0.0, 0.0, 0.0), Point3D::new(-1.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-6);
        assert_eq!(dir, Some(CurveDirection::Opposite));
    }
```
```rust
    #[test]
    fn test_no_connection() {
        let a = make_line_curve(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let b = make_line_curve(Point3D::new(10.0, 0.0, 0.0), Point3D::new(11.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-6);
        assert_eq!(dir, None);
    }
}
```


### ✅ 테스트 시나리오
- Forward: A 끝점 == B 시작점
- Backward: A 시작점 == B 끝점
- Facing: A 끝점 == B 끝점
- Opposite: A 시작점 == B 시작점
- None: 연결되지 않음

---

## 실전 테스트 코드
```rust
#[cfg(test)]
mod curve_direction_tests {
    use nurbslib::core::curve_direction::CurveDirection;
    use nurbslib::core::prelude::{NurbsCurve, Point4D};

    // 직선 한 세그먼트: degree=1, clamped knots [0,0,1,1]
    fn line(p0: (f64, f64, f64), p1: (f64, f64, f64)) -> NurbsCurve {
        let cp = vec![
            Point4D {
                x: p0.0,
                y: p0.1,
                z: p0.2,
                w: 1.0,
            },
            Point4D {
                x: p1.0,
                y: p1.1,
                z: p1.2,
                w: 1.0,
            },
        ];
        // 네 구조에 있는 헬퍼 사용 (clamped uniform knot 자동 생성)
        NurbsCurve::from_ctrl_clamped_uniform(1, cp)
    }
```
```rust
    #[test]
    fn detect_forward() {
        // A: 0→1,  B: 1→2  (연결점 A 끝 == B 시작)
        let a = line((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let b = line((1.0, 0.0, 0.0), (2.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-12);
        assert!(matches!(dir, Some(CurveDirection::Forward)));
    }
```
```rust
    #[test]
    fn detect_backward() {
        // A: 0→1,  B: -1→0  (연결점 A 시작 == B 끝)
        let a = line((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let b = line((-1.0, 0.0, 0.0), (0.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-12);
        assert!(matches!(dir, Some(CurveDirection::Backward)));
    }
```
```rust
    #[test]
    fn detect_facing() {
        // A: 0→1,  B: 2→1  (연결점 A 끝 == B 끝) → 서로 마주봄
        let a = line((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let b = line((2.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-12);
        assert!(matches!(dir, Some(CurveDirection::Facing)));
    }
```
```rust
    #[test]
    fn detect_opposite() {
        // A: 0→1,  B: 0→-1  (연결점 A 시작 == B 시작) → 반대 방향으로 출발
        let a = line((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let b = line((0.0, 0.0, 0.0), (-1.0, 0.0, 0.0));
        let dir = CurveDirection::detect(&a, &b, 1e-12);
        assert!(matches!(dir, Some(CurveDirection::Opposite)));
    }

    #[test]
    fn detect_none_when_far() {
        // 연결점이 조금 떨어져 있으면 (epsilon 보다 큼) → None
        let a = line((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let b = line((1.0, 1e-6, 0.0), (2.0, 1e-6, 0.0)); // y에 오프셋
        let dir = CurveDirection::detect(&a, &b, 1e-9);
        assert!(dir.is_none());
    }
}
```
---



