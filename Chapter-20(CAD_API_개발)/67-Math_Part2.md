# 수식 2

```rust
use crate::approx_eq;
use crate::core::basis::{bernstein_der_3, on_basis_func, on_binomial, on_find_span_index};
use crate::core::geom::{Point2D, Point4D, Vector2D};
use crate::core::matrix::{DenseMat, Matrix};
use crate::core::plane::Plane;
use crate::core::point_ops::PointOps;
use crate::core::poly_region2d::PolyRegion2d;
use crate::core::polygon2d::{Polygon2D, PolygonStatus};
use crate::core::prelude::{Degree, Interval, Point3D, Vector3D};
use crate::core::segment2d::{Segment2D, SegmentIntersectionType};
use crate::core::segment3d::Segment3D;
use crate::core::transform::Transform;
use crate::core::types::{Matrix3x3, ON_TOL9, ON_TOL12, ON_TOL14, Real, ON_EPSILON, ON_UNSET_VALUE, ON_RADIANS_TO_DEGREES, ON_UNSET_POSITIVE_VALUE, ON_DEGREES_TO_RADIANS, ON_SQRT2, ON_SQRT3, ON_SQRT_EPSILON, SQRT_EPS};
use nalgebra::{Cholesky, DMatrix, DVector};
use std::f64::consts::{PI, TAU};
use std::mem::swap;
use crate::core::tmatrix::TMatrix;
```
```rust
#[inline]
fn on_hypot2(a: f64, b: f64) -> f64 {
    a.hypot(b)
}
```
```rust
/// 대칭행렬 B (n×n)를 야코비 회전으로 고유분해.
/// 결과: B는 대각(고유값), v는 열-고유벡터(정규직교).
fn on_jacobi_symmetric_eigen(b: &mut Matrix, vals: &mut Vec<f64>, v: &mut Matrix) -> bool {
    let n = b.row_count();
    if n == 0 || b.col_count() != n {
        return false;
    }

    // v <- I
    if !v.create(n, n) {
        return false;
    }
    for i in 0..n {
        for j in 0..n {
            *v.at_mut(i as i32, j as i32) = if i == j { 1.0 } else { 0.0 };
        }
    }

    // 반복 파라미터
    let max_sweeps = 50 * n * n;
    let tol = 1e-14_f64;

    // 도움: 합 오프대각의 제곱합
    let off2 = |m: &Matrix| -> f64 {
        let mut s = 0.0;
        for p in 0..n {
            for q in 0..n {
                if p != q {
                    let x = *m.at(p as i32, q as i32);
                    s += x * x;
                }
            }
        }
        s
    };

    // 반복
    let mut sweep = 0usize;
    loop {
        let mut changed = false;

        for p in 0..n {
            for q in (p + 1)..n {
                let app = *b.at(p as i32, p as i32);
                let aqq = *b.at(q as i32, q as i32);
                let apq = *b.at(p as i32, q as i32);
                if apq.abs() <= tol * on_hypot2(app.abs(), aqq.abs()) {
                    continue;
                }

                // 회전계수 (NR 방식)
                let tau = (aqq - app) / (2.0 * apq);
                let t = if tau.abs() + 1.0 == 1.0 {
                    1.0 / (2.0 * tau)
                } else {
                    let sgn = if tau >= 0.0 { 1.0 } else { -1.0 };
                    sgn / (tau.abs() + (1.0 + tau * tau).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;

                // B <- Jᵀ B J  (대칭 유지)
                // 행/열 p,q 업데이트
                let bpp = app - t * apq;
                let bqq = aqq + t * apq;
                *b.at_mut(p as i32, p as i32) = bpp;
                *b.at_mut(q as i32, q as i32) = bqq;
                *b.at_mut(p as i32, q as i32) = 0.0;
                *b.at_mut(q as i32, p as i32) = 0.0;

                for r in 0..n {
                    if r != p && r != q {
                        let arp = *b.at(r as i32, p as i32);
                        let arq = *b.at(r as i32, q as i32);
                        let nrp = c * arp - s * arq;
                        let nrq = s * arp + c * arq;
                        *b.at_mut(r as i32, p as i32) = nrp;
                        *b.at_mut(p as i32, r as i32) = nrp;
                        *b.at_mut(r as i32, q as i32) = nrq;
                        *b.at_mut(q as i32, r as i32) = nrq;
                    }
                }

                // V <- V J (열-고유벡터)
                for r in 0..n {
                    let vrp = *v.at(r as i32, p as i32);
                    let vrq = *v.at(r as i32, q as i32);
                    *v.at_mut(r as i32, p as i32) = c * vrp - s * vrq;
                    *v.at_mut(r as i32, q as i32) = s * vrp + c * vrq;
                }

                changed = true;
            }
        }

        sweep += 1;
        if !changed {
            break;
        }
        if sweep > max_sweeps {
            break;
        } // 안전 탈출
        if off2(b) < tol {
            break;
        }
    }

    // 고유값 추출
    vals.clear();
    vals.resize(n, 0.0);
    for i in 0..n {
        vals[i] = *b.at(i as i32, i as i32);
    }
    true
}
```
```rust
#[inline]
pub fn on_point_on_circle(c: Point3D, r: f64, ang: f64) -> Point3D {
    Point3D {
        x: c.x + r * ang.cos(),
        y: c.y + r * ang.sin(),
        z: c.z,
    }
}
```
```rust
pub fn on_circle_to_polygon(center: Point2D, radius: f64, segments: usize) -> Polygon2D {
    let mut pts = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let theta = (i as f64) * std::f64::consts::TAU / (segments as f64);
        let x = center.x + radius * theta.cos();
        let y = center.y + radius * theta.sin();
        pts.push(Point2D::new(x, y));
    }
    Polygon2D::from_points(pts)
}
```
```rust
pub fn on_ellipse_to_polygon(center: Point2D, rx: f64, ry: f64, segments: usize) -> Polygon2D {
    let mut pts = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let theta = (i as f64) * std::f64::consts::TAU / (segments as f64);
        let x = center.x + rx * theta.cos();
        let y = center.y + ry * theta.sin();
        pts.push(Point2D::new(x, y));
    }
    Polygon2D::from_points(pts)
}
```
```rust
pub fn on_is_clockwise(points: &[Point2D]) -> bool {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        sum += (p2.x - p1.x) * (p2.y + p1.y);
    }
    sum > 0.0
}
```
```rust
pub fn on_ensure_winding_order(points: &mut Vec<Point2D>, clockwise: bool) {
    if on_is_clockwise(points) != clockwise {
        points.reverse();
    }
}
```
```rust
#[inline]
pub fn on_distance(a: &Point3D, b: &Point3D) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}
```
```rust
#[inline]
pub fn on_eq_pt(a: &Point3D, b: &Point3D) -> bool {
    (a.x == b.x) && (a.y == b.y) && (a.z == b.z)
}
```
```rust
#[inline]
pub fn on_lerp_point(a: &Point3D, b: &Point3D, t: f64) -> Point3D {
    Point3D {
        x: (1.0 - t) * a.x + t * b.x,
        y: (1.0 - t) * a.y + t * b.y,
        z: (1.0 - t) * a.z + t * b.z,
    }
}
```
```rust
#[inline]
pub fn on_unitize(v: Vector3D) -> Vector3D {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len <= ON_TOL12 {
        Vector3D::zero()
    } else {
        Vector3D {
            x: v.x / len,
            y: v.y / len,
            z: v.z / len,
        }
    }
}
```
```rust
#[inline]
pub fn on_dot_vec(a: &Vector3D, b: &Vector3D) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
```
```rust
#[inline]
pub fn on_dot_pt(a: &Point3D, b: &Point3D) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
```
```rust
#[inline]
pub fn on_is_valid_pt(p: &Point3D) -> bool {
    p.x.is_finite() && p.y.is_finite() && p.z.is_finite()
}
```
```rust
pub fn on_get_divide_number(radius: f64, delta_radians: f64, deviation: f64) -> (usize, f64) {
    // 2*acos((r-dev)/r) 각도 당 하나의 chord
    if !(radius > 0.0) || !(deviation > 0.0) || delta_radians.abs() < f64::EPSILON {
        return (2, delta_radians / 2.0);
    }
    let mut t = (radius - deviation) / radius;
    if !t.is_finite() {
        t = 1.0;
    }
    t = t.clamp(-1.0, 1.0);
    let denom = 2.0 * t.acos();
    let div = if denom <= f64::EPSILON {
        2
    } else {
        (delta_radians.abs() / denom).ceil().max(2.0) as usize
    };
    let angle = delta_radians / (div as f64);
    (div, angle)
}
```
```rust
pub fn on_number_of_segments(
    radius: f64,
    mut delta_radians: f64,
    deviation: f64,
    angle_limit: f64,
) -> usize {
    let pi = std::f64::consts::PI;
    let two_pi = 2.0 * pi;
    let mut div;

    if approx_eq!(delta_radians, two_pi, ON_TOL12) {
        delta_radians = pi / 2.0;
        let (mut local_div, a) = on_get_divide_number(radius, delta_radians, deviation);
        if angle_limit > 0.0 && !crate::approx_eq!(a, angle_limit, ON_TOL12) && a > angle_limit {
            local_div = ((pi / 2.0) / angle_limit).ceil() as usize;
        }
        div = local_div * 4;
    } else if approx_eq!(delta_radians, pi, ON_TOL12) {
        delta_radians = pi / 2.0;
        let (mut local_div, a) = on_get_divide_number(radius, delta_radians, deviation);
        if angle_limit > 0.0 && !approx_eq!(a, angle_limit, ON_TOL12) && a > angle_limit {
            local_div = ((pi / 2.0) / angle_limit).ceil() as usize;
        }
        div = local_div * 2;
    } else {
        let (mut d, a) = on_get_divide_number(radius, delta_radians, deviation);
        if angle_limit > 0.0 && !approx_eq!(a, angle_limit, ON_TOL12) && a > angle_limit {
            d = (delta_radians / angle_limit).ceil() as usize;
        }
        div = d;
    }
    if div < 2 {
        div = 2;
    }
    div
}
```
```rust
/// 외곽-홀 복합 다각형 포함 판정(간단판): 첫 루프는 외곽, 그 외는 홀
fn point_in_polygon_composite(p: &Point2D, loops: &[Polygon2D]) -> bool {
    if loops.is_empty() {
        return false;
    }
    if !point_in_polygon_simple(p, &loops[0].points) {
        return false;
    }
    for i in 1..loops.len() {
        if point_in_polygon_simple(p, &loops[i].points) {
            return false;
        }
    }
    true
}
```
```rust
#[inline]
fn vec_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
```
```rust
#[inline]
fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
```
```rust
#[inline]
fn vec_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
```
```rust
#[inline]
fn vec_mul_s(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
```
```rust
#[inline]
fn vec_len2(v: [f64; 3]) -> f64 {
    vec_dot(v, v)
}
```
```rust
#[inline]
#[allow(unused)]
fn vec_len(a: [f64; 3]) -> f64 {
    vec_dot(a, a).sqrt()
}
```
```rust
#[inline]
fn vec_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
```
```rust
fn vec_normalize(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / l, v[1] / l, v[2] / l]
    }
}
```
```rust
fn are_coplanar(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3], tol: f64) -> bool {
    let ab = vec_sub(b, a);
    let ac = vec_sub(c, a);
    let n = vec_cross(ab, ac);
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len == 0.0 {
        // 세 점이 일직선이면, D도 그 직선에 있어야 coplanar 취급
        let ad = vec_sub(d, a);
        let cr = vec_cross(ab, ad);
        let cr_len = (cr[0] * cr[0] + cr[1] * cr[1] + cr[2] * cr[2]).sqrt();
        let ab_len = (ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2])
            .sqrt()
            .max(1.0);
        return cr_len <= tol * ab_len;
    }
    let dist = vec_dot(n, vec_sub(d, a)).abs() / len;
    dist <= tol
}
```
```rust
// a를 원점으로 두고 e1,e2 기저로 투영 (u = (p-a)·e1, v = (p-a)·e2)
fn project_uv(a: [f64; 3], e1: [f64; 3], e2: [f64; 3], p: [f64; 3]) -> Point2D {
    let ap = vec_sub(p, a);
    Point2D::new(vec_dot(ap, e1), vec_dot(ap, e2))
}
```
```rust
/// A-B 와 D-C, 그리고 A-D 와 B-C 두 쌍 중 하나라도 교차하면 true
/// A-B-C-D 네 점이 주어졌을 때,
/// (A–C) vs (B–D) 혹은 (A–D) vs (B–C) 중 하나라도 교차하면 true
pub fn check_diagonal_intersections(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> bool {
    if !are_coplanar(a, b, c, d, 1e-12) {
        return false;
    }

    // 평면 기저 구성
    let ab = vec_sub(b, a);
    let ac = vec_sub(c, a);
    let n = vec_normalize(vec_cross(ab, ac));
    let mut e1 = vec_normalize(ab);
    if e1 == [0.0, 0.0, 0.0] {
        e1 = vec_normalize(ac);
    }
    let mut e2 = vec_normalize(vec_cross(n, e1));
    if e2 == [0.0, 0.0, 0.0] {
        let tmp = if n[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        e2 = vec_normalize(vec_cross(n, tmp));
    }

    // (u,v)로 투영
    let au = project_uv(a, e1, e2, a);
    let bu = project_uv(a, e1, e2, b);
    let cu = project_uv(a, e1, e2, c);
    let du = project_uv(a, e1, e2, d);

    // domain_size = 2D AABB 대각선 길이
    let minx = au.x.min(bu.x).min(cu.x).min(du.x);
    let maxx = au.x.max(bu.x).max(cu.x).max(du.x);
    let miny = au.y.min(bu.y).min(cu.y).min(du.y);
    let maxy = au.y.max(bu.y).max(cu.y).max(du.y);
    let domain_size = ((maxx - minx).hypot(maxy - miny)).max(1.0);

    // 교차로 인정할 타입
    let is_hit = |ty: SegmentIntersectionType| {
        matches!(
            ty,
            SegmentIntersectionType::Cross
                | SegmentIntersectionType::Touch
                | SegmentIntersectionType::EndPointTouch
                | SegmentIntersectionType::OverlapInSegment
                | SegmentIntersectionType::CollinearEndPointTouch
        )
    };

    // 쌍1: (A–C) vs (B–D)
    {
        let s1 = Segment2D::new(au, cu);
        let s2 = Segment2D::new(bu, du);
        let (t12, _, _) = Segment2D::intersection(&s1, &s2, domain_size);
        if is_hit(t12) {
            return true;
        }
    }

    // 쌍2: (A–D) vs (B–C)
    {
        let s3 = Segment2D::new(au, du);
        let s4 = Segment2D::new(bu, cu);
        let (t34, _, _) = Segment2D::intersection(&s3, &s4, domain_size);
        if is_hit(t34) {
            return true;
        }
    }

    false
}
```
```rust
pub fn measure_twist(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let len = Vector3D::new(p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]).length();
    let tol = 1e-9; // Utility.TOL9 대응
    let len_tol2 = (len * tol) * (len * tol);

    // b = p0->p1 (fallback: p2->p3)
    let mut b = Vector3D::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);
    if b.length_squared() < len_tol2 {
        b = Vector3D::new(p3[0] - p2[0], p3[1] - p2[1], p3[2] - p2[2]);
    }
    // a = p0->p2 (fallback: p1->p3)
    let mut a = Vector3D::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
    if a.length_squared() < len_tol2 {
        a = Vector3D::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
    }

    // 평면 법선
    let mut n = Vector3D::cross(&a, &b);
    if !n.normalize() {
        return 0.0;
    }
    // 평면 n·X = D
    let d = n.x * p0[0] + n.y * p0[1] + n.z * p0[2];
    (n.x * p3[0] + n.y * p3[1] + n.z * p3[2] - d).abs()
}
```
```rust
/// 단일 루프 점 포함 (짝수 교차법)
fn point_in_polygon_simple(p: &Point2D, poly: &[Point2D]) -> bool {
    let mut inside = false;
    let n = poly.len();
    for i in 0..n - 1 {
        let a = poly[i];
        let b = poly[i + 1];
        let intersect = ((a.y > p.y) != (b.y > p.y))
            && (p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y + 1e-30) + a.x);
        if intersect {
            inside = !inside;
        }
    }
    inside
}
```
```rust
pub fn classify_patch_polygon(patch: &Polygon2D, trim_polygons: &[Polygon2D]) -> PolygonStatus {
    if trim_polygons.is_empty() {
        return PolygonStatus::Out;
    }

    // --- AABB 합집합으로 빠른 거절 ---
    let mut tmin = trim_polygons[0].min;
    let mut tmax = trim_polygons[0].max;
    for tr in &*trim_polygons {
        tmin.x = tmin.x.min(tr.min.x);
        tmin.y = tmin.y.min(tr.min.y);
        tmax.x = tmax.x.max(tr.max.x);
        tmax.y = tmax.y.max(tr.max.y);
    }
    if !PolyRegion2d::overlap_2d(&patch.min, &patch.max, &tmin, &tmax) {
        return PolygonStatus::Out;
    }

    // domain_size: 패치와 트림 전체의 AABB를 합친 대각선 길이
    let umin = Point2D::new(patch.min.x.min(tmin.x), patch.min.y.min(tmin.y));
    let umax = Point2D::new(patch.max.x.max(tmax.x), patch.max.y.max(tmax.y));
    let domain_size = ((umax.x - umin.x).hypot(umax.y - umin.y)).max(1.0);

    // --- 2) 변-변 교차 검사 ---
    let mut edge_touch_count = 0usize;
    for i in 0..(patch.points.len() - 1) {
        let pe = Segment2D::new(patch.points[i], patch.points[i + 1]);
        for tr in trim_polygons {
            for j in 0..(tr.points.len() - 1) {
                let se = Segment2D::new(tr.points[j], tr.points[j + 1]);
                let (itype, _, _) = Segment2D::intersection(&se, &pe, domain_size);
                match itype {
                    SegmentIntersectionType::Cross => return PolygonStatus::On,
                    SegmentIntersectionType::Touch
                    | SegmentIntersectionType::EndPointTouch
                    | SegmentIntersectionType::OverlapInSegment
                    | SegmentIntersectionType::CollinearEndPointTouch => {
                        edge_touch_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    // --- 3) 복합(외곽-홀) 포함 여부로 패치 꼭짓점 카운트 ---
    let inside_composite = patch.points[..(patch.points.len() - 1)]
        .iter()
        .filter(|p| point_in_polygon_composite(p, trim_polygons))
        .count();

    // --- 4) 외곽의 첫 점이 패치 내부면 Over ---
    if inside_composite == 0 {
        if edge_touch_count > 0 {
            return PolygonStatus::On; // ← 경계 접촉은 On
        }
        if point_in_polygon_simple(&trim_polygons[0].points[0], &patch.points) {
            return PolygonStatus::Over; // ← 접촉은 없지만 외곽이 패치에 포함될 때 Over
        }
    }

    // --- 5) (핵심 보완) 어느 '홀' 내부에 패치가 통째로 들어가면 On 처리 ---
    //     패치 꼭짓점 4개가 동일 홀 내부(단순 폴리곤 기준)라면, 트림과 충돌/포함 관계로 본다.
    for i in 1..trim_polygons.len() {
        let in_hole_cnt = patch.points[..(patch.points.len() - 1)]
            .iter()
            .filter(|p| point_in_polygon_simple(p, &trim_polygons[i].points))
            .count();
        if in_hole_cnt == 4 {
            // 원한다면 Over로 바꿔도 됩니다:
            // return PolygonStatus::Over;
            return PolygonStatus::On;
        }
    }

    // --- 6) 최종 판별 ---
    if inside_composite == 4 {
        return if edge_touch_count > 0 {
            PolygonStatus::On
        } else {
            PolygonStatus::In
        };
    }

    // 중앙점으로 한 번 더 거르기(바깥이면 Out)
    let mid = Point2D::new(
        0.5 * (patch.points[0].x + patch.points[2].x),
        0.5 * (patch.points[0].y + patch.points[2].y),
    );
    let mid_out = !point_in_polygon_composite(&mid, trim_polygons);
    if edge_touch_count > 0 && inside_composite == 0 && mid_out {
        return PolygonStatus::Out;
    }

    if inside_composite <= 0 {
        PolygonStatus::Out
    } else {
        PolygonStatus::On
    }
}
```
```rust
/// `point_to_project`는 "선분 origin->target" 위로 정사영된 좌표로 덮어씁니다.
pub fn project_point_onto_line(origin: &Point3D, target: &Point3D, point_to_project: &mut Point3D) {
    let dir = Vector3D::new(
        target.x - origin.x,
        target.y - origin.y,
        target.z - origin.z,
    );
    let vec = Vector3D::new(
        point_to_project.x - origin.x,
        point_to_project.y - origin.y,
        point_to_project.z - origin.z,
    );

    let len_sq = dir.length_squared();
    let t = if len_sq < 1.0e-15 {
        0.0
    } else {
        Vector3D::dot(&dir, &vec) / len_sq
    };

    point_to_project.x = origin.x + t * dir.x;
    point_to_project.y = origin.y + t * dir.y;
    point_to_project.z = origin.z + t * dir.z;
}
```
```rust
/// Tridiagonal (Thomas) algorithm. Length n.
/// a: subdiagonal (a[0] is unused), b: diagonal, c: superdiagonal (c[n-1] is unused), d: RHS → overwritten with solution.
/// Returns: success status
pub fn on_solve_tridiagonal_inplace(
    a: &mut [f64],
    b: &mut [f64],
    c: &mut [f64],
    d: &mut [f64],
) -> bool {
    let n = b.len();
    if n == 0 || a.len() != n || c.len() != n || d.len() != n {
        return false;
    }

    // Forward elimination
    for i in 1..n {
        if b[i - 1].abs() < 1e-30 {
            return false;
        }
        let m = a[i] / b[i - 1];
        b[i] -= m * c[i - 1];
        d[i] -= m * d[i - 1];
        // Treat a[i] as zero
    }
    // Back substitution
    if b[n - 1].abs() < 1e-30 {
        return false;
    }
    d[n - 1] /= b[n - 1];
    for i in (0..n - 1).rev() {
        if b[i].abs() < 1e-30 {
            return false;
        }
        d[i] = (d[i] - c[i] * d[i + 1]) / b[i];
    }
    true
}
```
```rust
pub fn on_adaptive_simpson<F>(f: &F, a: f64, b: f64, tol: f64, depth: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    fn simpson<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64) -> f64 {
        let c = 0.5 * (a + b);
        (b - a) * (f(a) + 4.0 * f(c) + f(b)) / 6.0
    }

    let c = 0.5 * (a + b);
    let s = simpson(f, a, b);
    let s1 = simpson(f, a, c);
    let s2 = simpson(f, c, b);
    if depth == 0 || (s1 + s2 - s).abs() < 15.0 * tol {
        return s1 + s2 + (s1 + s2 - s) / 15.0;
    }
    on_adaptive_simpson(f, a, c, tol * 0.5, depth - 1)
        + on_adaptive_simpson(f, c, b, tol * 0.5, depth - 1)
}
```
```rust
#[inline]
pub fn on_are_equal(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}
```
```rust
#[inline]
pub fn on_are_equal_rel_tol(a: f64, b: f64) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0);
    (a - b).abs() <= ON_TOL9 * scale
}
```
```rust
#[inline]
pub fn on_are_vector_equal(a: &[f64], b: &[f64], tol: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| on_are_equal(*x, *y, tol))
}
```
```rust
#[inline]
pub fn on_are_vec_equal(u: Vector3D, v: Vector3D, eps: f64) -> bool {
    on_are_equal(u.x, v.x, eps) && on_are_equal(u.y, v.y, eps) && on_are_equal(u.z, v.z, eps)
}
```
```rust
#[inline]
pub fn on_are_vec2_equal(u: Vector2D, v: Vector2D, eps: f64) -> bool {
    on_are_equal(u.x, v.x, eps) && on_are_equal(u.y, v.y, eps)
}
```
```rust
#[inline]
pub fn on_are_pt_equal(p: Point3D, q: Point3D, eps: f64) -> bool {
    on_are_equal(p.x, q.x, eps) && on_are_equal(p.y, q.y, eps) && on_are_equal(p.z, q.z, eps)
}
```
```rust
#[inline]
pub fn on_are_pt2_equal(p: Point2D, q: Point2D, eps: f64) -> bool {
    on_are_equal(p.x, q.x, eps) && on_are_equal(p.y, q.y, eps)
}
```
```rust
#[inline]
pub fn on_is_zero(a: f64, tol: f64) -> bool {
    a.abs() < tol
}
```
```rust
#[inline]
pub fn on_compare(a: f64, b: f64, tol: f64) -> i32 {
    if a <= b - tol {
        -1
    } else if a >= b + tol {
        1
    } else {
        0
    }
}
```
```rust
pub fn on_are_between(a: f64, b: f64, c: f64) -> bool {
    ((a < c) && (c < b)) || ((b < c) && (c < a))
}
```
```rust
pub fn on_greater_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a > b) && !on_are_equal(a, b, tol)
}
```
```rust
pub fn on_greater_eqaul_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a > b) || on_are_equal(a, b, tol)
}
```
```rust
pub fn on_less_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a < b) && !on_are_equal(a, b, tol)
}
```
```rust
pub fn on_less_equal_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a < b) || on_are_equal(a, b, tol)
}
```
```rust
pub fn on_is_infinite(v: f64) -> bool {
    let max = f64::MAX;
    !(-max <= v && v <= max)
}
```
```rust
#[inline]
pub fn on_clamp<T: PartialOrd>(x: T, lo: T, hi: T) -> T {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}
```
```rust
pub fn on_rel_err(a: f64, b: f64) -> f64 {
    let denom = a.abs().max(b.abs()).max(1e-16);
    (a - b).abs() / denom
}
```
```rust
pub fn on_get_sampling_2d(datas: &[Point2D]) -> Vec<Point2D> {
    let count = datas.len() as i32;
    if count == 0 {
        return vec![];
    }

    let mut length = 4 + (count as f64).cbrt() as i32;
    if length > count {
        length = count;
    }

    let mut r = count / length;
    if r == 0 {
        r = 1;
    }

    let mut out = Vec::with_capacity(length as usize);
    for i in 0..length {
        out.push(datas[(i * r) as usize]);
    }
    out
}
```
```rust
pub fn on_get_sampling_3d(datas: &[Point3D]) -> Vec<Point3D> {
    let count = datas.len() as i32;
    if count == 0 {
        return vec![];
    }

    let mut length = 4 + (count as f64).cbrt() as i32;
    if length > count {
        length = count;
    }

    let mut r = count / length;
    if r == 0 {
        r = 1;
    }

    let mut out = Vec::with_capacity(length as usize);
    for i in 0..length {
        out.push(datas[(i * r) as usize]);
    }
    out
}
```
```rust
#[inline]
pub fn on_is_finite(v: f64) -> bool {
    v.is_finite()
}
```
```rust
#[inline]
pub fn on_clamp01(t: f64) -> f64 {
    if t < 0.0 {
        0.0
    } else if t > 1.0 {
        1.0
    } else {
        t
    }
}
```
```rust
#[inline]
pub fn on_clamp11(x: f64) -> f64 {
    if x < -1.0 {
        -1.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}
```
```rust
pub fn on_poly_fit(t: &[f64], v: &[f64], order: usize) -> Vec<f64> {
    assert_eq!(t.len(), v.len());
    assert!(t.len() >= order + 1);
    let m = t.len();
    let n = order + 1;

    let mut t_vec = vec![0.0; m * n];
    for i in 0..m {
        let mut xpow = 1.0;
        for j in 0..n {
            t_vec[i * n + j] = xpow;
            xpow *= t[i];
        }
    }

    // Normal equations: (T^T T) c = T^T v
    let mut ata = vec![0.0; n * n];
    let mut atv = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..m {
                s += t_vec[k * n + i] * t_vec[k * n + j];
            }
            ata[i * n + j] = s;
        }
        let mut s = 0.0;
        for k in 0..m {
            s += t_vec[k * n + i] * v[k];
        }
        atv[i] = s;
    }

    let mut c = atv.clone();
    assert!(on_cholesky_solve_spd(&mut ata, &mut c, n));
    c
}
```
```rust
pub fn on_poly_val(coeff: &[f64], xs: &[f64]) -> Vec<f64> {
    let mut ys = vec![0.0; xs.len()];
    for (i, &x) in xs.iter().enumerate() {
        // Horner
        let mut y = 0.0;
        for &a in coeff.iter().rev() {
            y = a + x * y;
        }
        ys[i] = y;
    }
    ys
}
```
```rust
pub fn on_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    for k in 0..n {
        let mut sum = 0.0;
        for p in 0..k {
            let l = g[k * n + p];
            sum += l * l;
        }
        let diag = g[k * n + k] - sum;
        if diag <= ON_TOL14 {
            return false;
        }
        g[k * n + k] = diag.sqrt();
        for i in (k + 1)..n {
            let mut s = 0.0;
            for p in 0..k {
                s += g[i * n + p] * g[k * n + p];
            }
            g[i * n + k] = (g[i * n + k] - s) / g[k * n + k];
        }
        for j in (k + 1)..n {
            g[k * n + j] = 0.0;
        }
    }
    for i in 0..n {
        let mut s = 0.0;
        for j in 0..i {
            s += g[i * n + j] * b[j];
        }
        b[i] = (b[i] - s) / g[i * n + i];
    }
    for i in (0..n).rev() {
        let mut s = 0.0;
        for j in (i + 1)..n {
            s += g[j * n + i] * b[j];
        }
        b[i] = (b[i] - s) / g[i * n + i];
    }
    true
}
```
```rust
pub fn on_nalgebra_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    // 1. 입력 슬라이스를 nalgebra 행렬/벡터로 변환
    let a = DMatrix::from_row_slice(n, n, g);
    let b_vec = DVector::from_column_slice(b);

    // 2. Cholesky 분해 시도
    if let Some(chol) = Cholesky::new(a) {
        // 3. 시스템 Ax = b 풀기
        let x = chol.solve(&b_vec);

        // 4. 결과를 b 슬라이스에 덮어쓰기
        b.copy_from_slice(x.as_slice());
        true
    } else {
        false
    }
}
```
```rust
pub fn on_circle_from_3_points(a: &Point2D, b: &Point2D, c: &Point2D) -> Option<(Point2D, f64)> {
    let a1 = b.x - a.x;
    let b1 = b.y - a.y;
    let a2 = c.x - a.x;
    let b2 = c.y - a.y;
    let d1 = (b.x * b.x - a.x * a.x + b.y * b.y - a.y * a.y) * 0.5;
    let d2 = (c.x * c.x - a.x * a.x + c.y * c.y - a.y * a.y) * 0.5;
    let det = a1 * b2 - a2 * b1;
    let scale = a1.abs().max(b1.abs()).max(a2.abs()).max(b2.abs()).max(1.0);
    let eps = ON_TOL12 * scale;

    if det.abs() <= eps {
        return None;
    }
    let ox = (d1 * b2 - d2 * b1) / det;
    let oy = (-d1 * a2 + d2 * a1) / det;
    let o = Point2D::new(ox, oy);
    let r = o.distance(&a);
    Some((o, r))
}
```
```rust
#[inline]
pub fn on_arc_half_angle(o: &Point2D, a: &Point2D, b: &Point2D) -> f64 {
    let ux = a.x - o.x;
    let uy = a.y - o.y;
    let vx = b.x - o.x;
    let vy = b.y - o.y;

    let nu = (ux * ux + uy * uy).sqrt();
    let nv = (vx * vx + vy * vy).sqrt();
    if !(nu > 0.0 && nv > 0.0) {
        return 0.0;
    }

    let mut c = (ux * vx + uy * vy) / (nu * nv);
    if c < -1.0 {
        c = -1.0;
    }
    if c > 1.0 {
        c = 1.0;
    }

    0.5 * c.acos()
}
```
```rust
pub fn on_are_equal_param(a: f64, b: f64, tol: f64) -> bool {
    let fa = a.is_finite();
    let fb = b.is_finite();
    if !fa && !fb {
        return true;
    }
    if fa && fb {
        return (a - b).abs() <= tol;
    }
    false
}
```
```rust
pub fn on_linspace(a: f64, b: f64, n: i32) -> Vec<f64> {
    let n = n.max(1);
    if n == 1 {
        return vec![a];
    }
    let h = (b - a) / ((n - 1) as f64);
    (0..n).map(|i| a + h * (i as f64)).collect()
}
```
```rust
#[inline]
pub fn on_angle_2pi_normalize(a: f64) -> f64 {
    if !a.is_finite() {
        return f64::NAN;
    }
    let mut r = a % TAU;
    if r < 0.0 {
        r += TAU;
    }
    r
}
```
```rust
#[inline]
pub fn on_normalize_to_domain(dom: &Interval, t01: f64) -> f64 {
    dom.parameter_at(t01.clamp(0.0, 1.0))
}
```
```rust
pub fn on_modify_param_diff(a: f64, b: f64, dom: &Interval) -> f64 {
    let l = dom.length();
    let mut d = a - b;
    if l > 0.0 {
        while d > 0.5 * l {
            d -= l;
        }
        while d < -0.5 * l {
            d += l;
        }
    }
    d
}
```
```rust
#[inline]
pub fn on_angle_2pi_difference(a: f64, b: f64) -> f64 {
    if !a.is_finite() || !b.is_finite() {
        return f64::NAN;
    }
    let mut d = (a - b) % TAU;
    if d > PI {
        d -= TAU;
    }
    if d < -PI {
        d += TAU;
    }
    d
}
```
```rust
#[inline]
pub fn on_angle_2pi_distance(a: f64, b: f64) -> f64 {
    on_angle_2pi_difference(a, b).abs()
}
```
```rust
/// Quadrant (East, North, West, South = 0,1,2,3) angle range
pub fn on_quadrant_theta_range(q: i32) -> (f64, f64) {
    let p8 = PI * 0.25;
    match q {
        0 => (-p8, p8),            // East
        1 => (p8, 3.0 * p8),       // North
        2 => (3.0 * p8, 5.0 * p8), // West
        _ => (5.0 * p8, 7.0 * p8), // South
    }
}
```
```rust
/// Returns the quadrant index (0: East, 1: North, 2: West, 3: South)
pub fn on_direction_quadrant(y: f64, x: f64) -> i32 {
    let theta = y.atan2(x); // angle in radians
    let p8 = std::f64::consts::PI * 0.25;

    if theta >= -p8 && theta < p8 {
        0 // East
    } else if theta >= p8 && theta < 3.0 * p8 {
        1 // North
    } else if theta >= 3.0 * p8 && theta < 5.0 * p8 {
        2 // West
    } else {
        3 // South
    }
}
```
```rust
#[inline]
pub fn on_canonicalize_periodic(u: f64, dom: &Interval, periodic: bool) -> f64 {
    if !periodic {
        return u;
    }
    let l = dom.length();
    if !(l > 0.0) || !u.is_finite() {
        return u;
    }
    let k = ((u - dom.t0) / l).floor();
    u - k * l
}
```
```rust
/// One-sided geometric division
pub fn on_generate_biased_divisions(
    total_length: f64,
    num_div: i32,
    mut r: f64,
    small_at_left: bool,
) -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }
    if r <= 0.0 {
        r = 1.0;
    }

    let mut flip = false;
    if r < 1.0 {
        r = 1.0 / r;
        flip = true;
    }
    let left_small = small_at_left ^ flip;

    if (r - 1.0).abs() < 1e-12 {
        let a = total_length / (n as f64);
        for i in 0..=n {
            lt[i] = a * (i as f64);
        }
        return lt;
    }

    let a = total_length / ((r.powi(num_div.max(0)) - 1.0) / (r - 1.0));

    if left_small {
        lt[0] = 0.0;
        for i in 1..=n {
            lt[i] = a * (r.powi(i as i32) - 1.0) / (r - 1.0);
        }
    } else {
        let mut base = vec![0.0; n + 1];
        for i in 1..=n {
            base[i] = a * (r.powi(i as i32) - 1.0) / (r - 1.0);
        }
        for i in 0..=n {
            lt[i] = total_length - base[n - i];
        }
    }
    lt[0] = 0.0;
    lt[n] = total_length;
    lt
}
```
```rust
/// Left/right skewed “smooth” split (Power/Exp CDF)
/// method: 0=Power(t^gamma), 1=Exp((e^{kt}-1)/(e^k-1))
pub fn on_generate_smooth_biased_divisions(
    l: f64,
    n: i32,
    strength: f64,
    small_at_left: bool,
    method: i32,
) -> Vec<f64> {
    let n = n.max(0) as usize;
    let mut x = vec![0.0; n + 1];
    if n == 0 {
        return x;
    }

    let f_power = |t: f64| -> f64 {
        let gamma = if strength <= 0.0 { 1.0 } else { strength };
        t.clamp(0.0, 1.0).powf(gamma)
    };
    let f_exp = |t: f64| -> f64 {
        let t = t.clamp(0.0, 1.0);
        let k = strength;
        if k.abs() < 1e-12 {
            return t;
        }
        let ek = k.exp();
        let ekt = (k * t).exp();
        (ekt - 1.0) / (ek - 1.0)
    };
    let f = |t: f64| -> f64 { if method == 0 { f_power(t) } else { f_exp(t) } };

    for i in 0..=n {
        let t = (i as f64) / (n as f64);
        x[i] = if small_at_left {
            l * f(t)
        } else {
            l * (1.0 - f(1.0 - t))
        };
    }
    x[0] = 0.0;
    x[n] = l;
    x
}
```
```rust
/// Smooth symmetric division (sin^k)
pub fn on_generate_smooth_symmetric_bias(
    total_length: f64,
    num_div: i32,
    k: f64,
    eps: f64,
) -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }

    let mut w = vec![0.0; n];
    for i in 0..n {
        let t = (i as f64 + 0.5) / (n as f64);
        let s = (PI * t).sin();
        w[i] = eps + s.powf(k.max(0.0));
    }
    let w_sum: f64 = w.iter().sum();
    let mut seg = vec![0.0; n];
    for i in 0..n {
        seg[i] = total_length * (w[i] / w_sum.max(1e-300));
    }
    for i in 0..n {
        lt[i + 1] = lt[i] + seg[i];
    }
    lt[n] = total_length;
    lt
}
```
```rust
/// Symmetric geometric partitioning (middle dense)
pub fn on_generate_symmetric_geometric_bias(total_length: f64, num_div: i32, r: f64) -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }

    let mut w = vec![0.0; n];
    for i in 0..n {
        let a = r.powi(i as i32);
        let b = r.powi((n - 1 - i) as i32);
        w[i] = a.min(b);
    }
    let wsum: f64 = w.iter().sum();
    let mut seg = vec![0.0; n];
    for i in 0..n {
        seg[i] = total_length * (w[i] / wsum.max(1e-300));
    }

    for i in 0..n {
        lt[i + 1] = lt[i] + seg[i];
    }
    lt[n] = total_length;
    lt
}
```
```rust
pub fn on_make_frame_matrix(
    o: &Point3D,
    ex: &Vector3D,
    ey: &Vector3D,
    ez_hint: &Vector3D,
) -> Transform {
    let x = ex.unitize();
    // Y를 X에 수직 성분만 남겨 정규화
    let y_raw = ey - x * Vector3D::dot(&ey, &x);
    let mut y = y_raw.unitize();
    if !y.is_valid() || y.length() < 1e-14 {
        // ey가 좋지 않으면 (ex×ez)×ex 로 보정
        let y_alt = ex.cross(&ez_hint).cross(&ex);
        y = y_alt.unitize();
    }
    let z = x.cross(&y).unitize();
    let y = z.cross(&x).unitize();

    // ⚠️ Assumption: the following assumes a 4×4 constructor where columns represent axes and the last column is the origin
    // Use a constructor that matches your project’s Transform convention
    Transform::from_cols(
        [x.x, x.y, x.z, 0.0],
        [y.x, y.y, y.z, 0.0],
        [z.x, z.y, z.z, 0.0],
        [o.x, o.y, o.z, 1.0],
    )
}
```
```rust
/// Core edge index (mapped to always proceed CCW)
pub fn on_core_edge_idx_ccw(q: i32, k: i32, core_u: i32, core_v: i32) -> i32 {
    let idx = |i: i32, j: i32| j * (core_u + 1) + i;
    match q {
        0 => idx(core_u, k),          // East (j up)
        1 => idx(core_u - k, core_v), // North (i down)
        2 => idx(0, core_v - k),      // West (j down)
        _ => idx(k, 0),               // South (i up)
    }
}
```
```rust
fn on_cholesky_decompose_spd(a: &mut [f64], n: usize) -> bool {
    // a: row-major 상삼각/하삼각 모두 들어있는 dense 대칭
    for i in 0..n {
        for j in 0..=i {
            let mut s = a[i * n + j];
            for k in 0..j {
                s -= a[i * n + k] * a[j * n + k];
            }
            if i == j {
                if s <= 0.0 {
                    return false;
                }
                a[i * n + j] = s.sqrt();
            } else {
                a[i * n + j] = s / a[j * n + j];
            }
        }
        // 상삼각은 0으로 정리(선택)
        for j in (i + 1)..n {
            a[i * n + j] = 0.0;
        }
    }
    true
}
```
```rust
/// Cholesky로 Ax=b 푸는 전진/후진 대치
fn on_cholesky_solve(a: &[f64], b: &mut [f64], n: usize) {
    // L y = b
    for i in 0..n {
        let mut s = b[i];
        for k in 0..i {
            s -= a[i * n + k] * b[k];
        }
        b[i] = s / a[i * n + i];
    }
    // L^T x = y
    for i in (0..n).rev() {
        let mut s = b[i];
        for k in (i + 1)..n {
            s -= a[k * n + i] * b[k];
        }
        b[i] = s / a[i * n + i];
    }
}
```
```rust
/// 간단 가우스 소거(부분 피벗) – Cholesky 실패 시 폴백
fn on_gaussian_solve(mut a: Vec<f64>, mut b: Vec<f64>, n: usize) -> Option<Vec<f64>> {
    // 증분행렬 [A|b]
    for i in 0..n {
        // pivot
        let mut piv = i;
        let mut maxv = a[i * n + i].abs();
        for r in (i + 1)..n {
            let v = a[r * n + i].abs();
            if v > maxv {
                maxv = v;
                piv = r;
            }
        }
        if maxv <= 1e-30 {
            return None;
        }
        if piv != i {
            for c in i..n {
                a.swap(i * n + c, piv * n + c);
            }
            b.swap(i, piv);
        }
        // eliminate
        let diag = a[i * n + i];
        for r in (i + 1)..n {
            let f = a[r * n + i] / diag;
            if f == 0.0 {
                continue;
            }
            for c in i..n {
                a[r * n + c] -= f * a[i * n + c];
            }
            b[r] -= f * b[i];
        }
    }
    // back-subst
    for i in (0..n).rev() {
        let mut s = b[i];
        for c in (i + 1)..n {
            s -= a[i * n + c] * b[c];
        }
        let d = a[i * n + i];
        if d.abs() <= 1e-30 {
            return None;
        }
        b[i] = s / d;
    }
    Some(b)
}
```
```rust
/// - 첫/끝 제어점은 데이터 양 끝점으로 고정
/// - 내부(m-2) 제어점은 최소제곱으로 추정
/// - 비라셔널(w=1) 가정
pub fn on_least_squares_end_interpolate(
    points: &[Point3D],
    degree: usize,  // p
    n_ctrl: usize,  // m
    params: &[f64], // u_i
    knot: &[f64],   // U
) -> Option<Vec<Point4D>> {
    let n_data = points.len();
    if n_data < 2 || n_ctrl < degree + 1 {
        return None;
    }
    if knot.len() != n_ctrl + degree + 1 {
        return None;
    }
    if params.len() != n_data {
        return None;
    }

    // 내부 제어점 개수 (미지수) = m-2, 첫/끝은 고정
    if n_ctrl < 2 {
        return None;
    }
    let n_unknown = n_ctrl - 2;
    if n_unknown == 0 {
        // 제어점이 2개면 직선 – 첫/끝만 반환
        let mut cps = Vec::with_capacity(2);
        cps.push(Point4D::new(points[0].x, points[0].y, points[0].z, 1.0));
        let pe = points[n_data - 1];
        cps.push(Point4D::new(pe.x, pe.y, pe.z, 1.0));
        return Some(cps);
    }

    // 그람행렬 G = A^T A (n_unknown x n_unknown), RHS_x/y/z = A^T (b)
    // b_i = P_i - N_{i,0}*P0 - N_{i,m-1}*P_{m-1}
    let mut gram_vec = vec![0.0f64; n_unknown * n_unknown];
    let mut rhs_x = vec![0.0f64; n_unknown];
    let mut rhs_y = vec![0.0f64; n_unknown];
    let mut rhs_z = vec![0.0f64; n_unknown];

    let p0 = points[0];
    let pend = points[n_data - 1];

    // 한 데이터 점마다 기저 N(span, u) 누적
    let p = degree;
    for i in 0..n_data {
        let u = params[i];
        // find_span: n = m-1
        let span = on_find_span_index(n_ctrl - 1, p as Degree, u, knot);
        let mut n_vec = vec![0.0; n_ctrl + p + 1];
        on_basis_func(span, u, p as Degree, knot, &mut n_vec);

        // b_i = Pi - N0 * P0 - N_last * Pend
        // (여기서 N0는 실제 0번째 열의 계수인지, N_last는 마지막 열 계수인지
        //  — span-p..span 범위 내에서 해당하는 열(0, m-1)이 있으면 그 계수를 쓰는 개념.
        //  하지만 C# 코드는 Equation을 만들어 pos별로 접근했으므로,
        //  동일하게 처리: 내부에서 0 또는 m-1 열이 포함되어 있으면 그만큼 빼 준다.)

        let pi = points[i];
        let mut bx = pi.x;
        let mut by = pi.y;
        let mut bz = pi.z;

        // span 에 해당하는 전역 열 idx = span-p .. span
        let col0 = if span >= p { span - p } else { 0 };
        for j in 0..=p {
            let col = col0 + j;
            let aij = n_vec[j];
            if col == 0 {
                bx -= aij * p0.x;
                by -= aij * p0.y;
                bz -= aij * p0.z;
            } else if col == n_ctrl - 1 {
                bx -= aij * pend.x;
                by -= aij * pend.y;
                bz -= aij * pend.z;
            }
        }

        // 내부 열(1..m-2)에 대해서만 A와 b를 누적 → G += a^T a, rhs += a^T b
        // 내부 열의 로컬 인덱스 = (col-1) in [0..n_unknown-1]
        for j in 0..=p {
            let colj = col0 + j;
            if colj == 0 || colj == n_ctrl - 1 {
                continue;
            }
            let lj = colj - 1; // 0..n_unknown-1
            let aij = n_vec[j];

            // RHS
            rhs_x[lj] += aij * bx;
            rhs_y[lj] += aij * by;
            rhs_z[lj] += aij * bz;

            // G(=A^T A)
            for k in 0..=p {
                let colk = col0 + k;
                if colk == 0 || colk == n_ctrl - 1 {
                    continue;
                }
                let lk = colk - 1;
                gram_vec[lj * n_unknown + lk] += aij * n_vec[k];
            }
        }
    }

    // 이제 G * X = RHS 를 x,y,z 각각에 대해 풉니다.
    // 우선 Cholesky 시도 → 실패 시 가우스 소거 폴백
    let mut g_chol = gram_vec.clone();
    let chol_ok = on_cholesky_decompose_spd(&mut g_chol, n_unknown);

    let solve_one = |g_dense: &mut [f64], rhs: &mut [f64]| -> Option<Vec<f64>> {
        if chol_ok {
            let a = g_dense.to_vec(); // cholesky_solve는 상삼/하삼 배치로 읽음
            let mut b = rhs.to_vec();
            on_cholesky_solve(&a, &mut b, n_unknown);
            Some(b)
        } else {
            on_gaussian_solve(gram_vec.clone(), rhs.to_vec(), n_unknown)
        }
    };

    let xs = solve_one(&mut g_chol, &mut rhs_x)?;
    let ys = solve_one(&mut g_chol, &mut rhs_y)?;
    let zs = solve_one(&mut g_chol, &mut rhs_z)?;

    // 최종 제어점 구성: C0, C1..C_{m-2}, C_{m-1}
    let mut ctrl = Vec::with_capacity(n_ctrl);
    ctrl.push(Point4D::new(p0.x, p0.y, p0.z, 1.0));
    for i in 0..n_unknown {
        ctrl.push(Point4D::new(xs[i], ys[i], zs[i], 1.0));
    }
    ctrl.push(Point4D::new(pend.x, pend.y, pend.z, 1.0));

    Some(ctrl)
}
```
```rust
#[derive(Copy, Clone, Debug)]
pub struct Solve2x2Result {
    pub rank: i32,
    pub x: f64,
    pub y: f64,
    pub pivot_ratio: f64,
}
```
```rust

pub fn on_solve_2x2(
    mut m00: f64,
    mut m01: f64,
    mut m10: f64,
    mut m11: f64,
    mut d0: f64,
    mut d1: f64,
) -> Solve2x2Result {
    use core::mem;

    // pivot 선택 (최대 절댓값)
    let mut which = 0usize;
    let mut vmax = m00.abs();
    let v01 = m01.abs();
    if v01 > vmax {
        vmax = v01;
        which = 1;
    }
    let v10 = m10.abs();
    if v10 > vmax {
        vmax = v10;
        which = 2;
    }
    let v11 = m11.abs();
    if v11 > vmax {
        vmax = v11;
        which = 3;
    }

    let mut x = 0.0;
    let mut y = 0.0;
    let mut pivot_ratio = 0.0;

    if vmax == 0.0 {
        return Solve2x2Result {
            rank: 0,
            x,
            y,
            pivot_ratio,
        };
    }

    // val5=max pivot, val6=min pivot (초기값은 vmax)
    let mut val5 = vmax;
    let mut val6 = vmax;

    // 열 스왑?
    let mut swapped_cols = false;
    if which % 2 == 1 {
        swapped_cols = true;
        mem::swap(&mut m00, &mut m01);
        mem::swap(&mut m10, &mut m11);
    }
    // 행 스왑?
    if which > 1 {
        mem::swap(&mut d0, &mut d1);
        mem::swap(&mut m00, &mut m10);
        mem::swap(&mut m01, &mut m11);
    }

    // 첫 피벗으로 정규화
    let inv = 1.0 / m00;
    m01 *= inv;
    d0 *= inv;

    // 소거
    if m10 != 0.0 {
        m11 -= m10 * m01;
        d1 -= m10 * d0;
    }

    // 두 번째 피벗 체크 (정확 비교)
    if m11 == 0.0 {
        return Solve2x2Result {
            rank: 1,
            x,
            y,
            pivot_ratio: 0.0,
        };
    }

    // pivot ratio 갱신
    let v = m11.abs();
    if v > val5 {
        val5 = v;
    } else if v < val6 {
        val6 = v;
    }
    pivot_ratio = val6 / val5;

    // back substitution
    d1 /= m11;
    if m01 != 0.0 {
        d0 -= m01 * d1;
    }

    if !swapped_cols {
        x = d0;
        y = d1;
    } else {
        y = d0;
        x = d1;
    }

    Solve2x2Result {
        rank: 2,
        x,
        y,
        pivot_ratio,
    }
}
```
```rust
pub fn on_solve_2x2_tuple(
    mut m00: f64,
    mut m01: f64,
    mut m10: f64,
    mut m11: f64,
    mut d0: f64,
    mut d1: f64,
) -> (i32, f64, f64, f64) {
    let pivot_ratio;

    // choose max-abs pivot in 2x2
    let mut i = 0;
    let mut x = m00.abs();
    let mut y = m01.abs();
    if y > x {
        x = y;
        i = 1;
    }
    y = m10.abs();
    if y > x {
        x = y;
        i = 2;
    }
    y = m11.abs();
    if y > x {
        x = y;
        i = 3;
    }

    if x == 0.0 {
        return (0, 0.0, 0.0, 0.0); // rank 0
    }
    let mut minpiv = x;
    let mut maxpiv = x;

    // track where x/y land if we swap columns
    let mut xy_swapped = false;

    // if pivot column is 1, swap columns 0<->1
    if i % 2 == 1 {
        xy_swapped = true;
        swap(&mut m00, &mut m01);
        swap(&mut m10, &mut m11);
    }
    // if pivot row is 1, swap rows 0<->1 (affects d as well)
    if i > 1 {
        swap(&mut d0, &mut d1);
        swap(&mut m00, &mut m10);
        swap(&mut m01, &mut m11);
    }

    // eliminate
    let inv = 1.0 / m00;
    m01 *= inv;
    d0 *= inv;
    if m10 != 0.0 {
        m11 -= m10 * m01;
        d1 -= m10 * d0;
    }

    if m11 == 0.0 {
        // rank 1
        // x = d0, y will be 0 if m01==0 else free variable; 원 코드와 동일하게 반환만 함
        let (x_ans, y_ans) = if xy_swapped { (0.0, d0) } else { (d0, 0.0) };
        return (1, x_ans, y_ans, 0.0);
    }

    // pivot stats
    let y_abs = m11.abs();
    if y_abs > maxpiv {
        maxpiv = y_abs;
    } else if y_abs < minpiv {
        minpiv = y_abs;
    }

    // back-substitute
    d1 /= m11;
    if m01 != 0.0 {
        d0 -= m01 * d1;
    }

    let (mut x_ans, mut y_ans) = (d0, d1);
    if xy_swapped {
        swap(&mut x_ans, &mut y_ans);
    }

    pivot_ratio = minpiv / maxpiv;
    (2, x_ans, y_ans, pivot_ratio)
}
```
```rust
pub fn on_solve_2x2_fast(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Option<(f64, f64)> {
    let scale = a
        .abs()
        .max(b.abs())
        .max(c.abs())
        .max(d.abs())
        .max(e.abs())
        .max(f.abs())
        .max(1.0);
    let det = a * d - b * c;
    if det.abs() < ON_TOL12 * scale {
        return None;
    }
    let s = (e * d - b * f) / det;
    let t = (a * f - e * c) / det;
    Some((s, t))
}
```
```rust
#[inline]
pub fn on_cross_2d(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}
```
```rust
#[inline]
pub fn on_cross_vec_2d(a: Point2D, b: Point2D, c: Point2D) -> f64 {
    let ux = b.x - a.x;
    let uy = b.y - a.y;
    let vx = c.x - a.x;
    let vy = c.y - a.y;
    on_cross_2d(ux, uy, vx, vy)
}
```
```rust
#[inline]
pub fn on_signed_area(poly: &[Point2D]) -> f64 {
    let n = poly.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        a += poly[j].x * poly[i].y - poly[i].x * poly[j].y;
        j = i;
    }
    0.5 * a
}
```
```rust
#[inline]
pub fn on_is_convex_ccw(poly: &[Point2D], eps: f64) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }
    for i in 0..n {
        let a = &poly[i];
        let b = &poly[(i + 1) % n];
        let c = &poly[(i + 2) % n];
        let cross = on_cross_2d(b.x - a.x, b.y - a.y, c.x - b.x, c.y - b.y);
        if cross < -eps {
            return false;
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_mat3x3_close(a: [[f64; 3]; 3], b: [[f64; 3]; 3], e: f64) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if !on_are_equal(a[i][j], b[i][j], e) {
                return false;
            }
        }
    }
    true
}
```
```rust
#[inline]
pub fn on_mat4x4_close(a: [[f64; 4]; 4], b: [[f64; 4]; 4], e: f64) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if !on_are_equal(a[i][j], b[i][j], e) {
                return false;
            }
        }
    }
    true
}
```
```rust
pub fn on_make_frame_plane(
    p0: Point3D,
    p2: Point3D,
    p: Point3D,
) -> Option<(Point3D, Vector3D, Vector3D, Vector3D)> {
    let origin = p0;

    // a = P0->P2, b = P0->P
    let a = (p2 - p0).to_vector();
    let b = (p - p0).to_vector();

    // z = a × b
    let mut z_axis = a.cross(&b);
    z_axis = z_axis.unitize();

    // x = unit(a)
    let mut x_axis = a;
    x_axis = x_axis.unitize();

    // y = z × x
    let mut y_axis = z_axis.cross(&x_axis);
    y_axis = y_axis.unitize();

    Some((origin, x_axis, y_axis, z_axis))
}
```
```rust
// ---- Intersect two 2D lines: A + t*U and B + s*V ----
// returns None if parallel
pub fn on_intersect_lines_2d(
    a: Point2D,
    u: Point2D,
    b: Point2D,
    v: Point2D,
) -> Option<(f64, f64, Point2D)> {
    let det = u.x * v.y - u.y * v.x;
    let eps = 1e-15;
    if det.abs() <= eps {
        return None;
    }
    let w = b + a;
    let t = (w.x * v.y - w.y * v.x) / det;
    let s = (w.x * u.y - w.y * u.x) / det;
    let int_pt = a + u * t;
    Some((t, s, int_pt))
}
```
```rust
fn on_project_vec_2d(v: Vector3D, x_axis: Vector3D, y_axis: Vector3D) -> Point2D {
    Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
}
```
```rust
fn on_project_point_2d(p: Point3D, origin: Point3D, x_axis: Vector3D, y_axis: Vector3D) -> Point2D {
    let v = (p - origin).to_vector();
    Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
}
```
```rust
pub fn on_make_bezier_conic_arc(
    p0: Point3D,
    t0: Vector3D,
    p2: Point3D,
    t2: Vector3D,
    p: Point3D,
) -> Option<(Point3D, Real)> {
    // 1) build a local plane frame
    let (o, x_axis, y_axis, _z_axis) = on_make_frame_plane(p0, p2, p)?;

    // 2) project to 2D
    let p0_2 = on_project_point_2d(p0, o, x_axis, y_axis);
    let p2_2 = on_project_point_2d(p2, o, x_axis, y_axis);
    let pp_2 = on_project_point_2d(p, o, x_axis, y_axis);

    let t0_2 = on_project_vec_2d(t0, x_axis, y_axis);
    let t2_2 = on_project_vec_2d(t2, x_axis, y_axis);

    // 3) try intersection of tangents (non-parallel case)
    if let Some((_tau0, _tau2, p1_2)) = on_intersect_lines_2d(p0_2, t0_2, p2_2, t2_2) {
        // Intersect segment p0-p2 with line (p1 -- p)
        let seg = p2_2 - p0_2;
        let dir = pp_2 - p1_2;

        if let Some((tseg, _tl, _m)) = on_intersect_lines_2d(p0_2, seg, p1_2, dir) {
            let eps = 1e-15;
            if tseg < -1e-12 || tseg > 1.0 + 1e-12 {
                return None;
            }
            if (1.0 - tseg).abs() <= eps {
                return None;
            }

            let a = (tseg / (1.0 - tseg)).sqrt();
            let u = a / (1.0 + a);

            // vectors for dot products
            let v0 = pp_2 - p0_2;
            let v1 = p1_2 - pp_2;
            let v2 = pp_2 - p2_2;

            let alf = v0.dot(&v1);
            let bet = v1.dot(&v2);
            let gam = v1.dot(&v1);

            let a_ = (1.0 - u) * (1.0 - u);
            let b_ = u * u;
            let c_ = 2.0 * u * (1.0 - u);

            let num = a_ * alf + b_ * bet;
            let den = c_ * gam;
            if den.abs() <= eps {
                return None;
            }
            let w1 = num / den;

            // lift p1 back to 3D
            let p1 = o + (x_axis * p1_2.x + y_axis * p1_2.y).to_point();
            return Some((p1, w1));
        }
        return None;
    }

    // 4) parallel tangents → parabola branch
    // Intersect line L = (P, T0) with segment S = (P0 -> P2)
    {
        let a = pp_2;
        let u = t0_2;
        let b = p0_2;
        let v = p2_2 - p0_2;

        if let Some((tt, ts, _x)) = on_intersect_lines_2d(a, u, b, v) {
            let eps = 1e-15;
            if (1.0 - ts).abs() <= eps {
                return None;
            }
            if ts < -1e-12 || ts > 1.0 + 1e-12 {
                return None;
            }

            let aa = (ts / (1.0 - ts)).sqrt();
            let u = aa / (1.0 + aa);
            let b = 2.0 * u * (1.0 - u);

            let num = -tt * (1.0 - b);
            if b.abs() <= eps {
                return None;
            }
            let scale = num / b;

            // w1 = 0, and p1 encodes a 3D vector along T0 (no origin)
            let t0u = t0;
            if t0u.length_squared() > 0.0 {
                // keep original scale (do not normalize)
                let v3 = t0u * scale;
                let p1_as_vec = Point3D::new(v3.x, v3.y, v3.z);
                return Some((p1_as_vec, 0.0));
            } else {
                return Some((Point3D::new(0.0, 0.0, 0.0), 0.0));
            }
        }
    }

    None
}
```
```rust
/// Band matrix LU decomposition
///
/// - a: n x (m1 + m2 + 1) — transformed in-place into U
/// - al: n x m1 — stores the lower band of L
/// - index: length n, pivot indices (stored as 1-based; compatible with original C++ convention)
/// - d: (out) sign of row exchanges (+/-1)
pub fn on_bandec<A: DenseMat, L: DenseMat>(
    a: &mut A,
    m1: usize,
    m2: usize,
    al: &mut L,
    index: &mut [usize],
    d: &mut f64,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1, "al must have at least m1 columns");
    debug_assert_eq!(index.len(), n);

    // 상부로 정렬(슬라이딩) + 왼쪽 0 채우기
    let mut num2 = m1;
    for i in 0..m1 {
        // a[i][0..] ← a[i][(m1-i)..(num1-1)]
        for j in (m1 - i)..num1 {
            let v = a.get(i, j);
            a.set(i, j - num2, v);
        }
        num2 -= 1;
        // 오른쪽 끝쪽을 0으로 채움
        for j in (num1 - num2 - 1)..num1 {
            a.set(i, j, 0.0);
        }
    }

    *d = 1.0;
    let mut num3 = m1;

    for i in 0..n {
        // 피벗 찾기: a[i..min(i+num3-i, n-1)][0] 중 절대값 최대
        let mut val1 = a.get(i, 0);
        let mut imax = i;

        if num3 < n {
            num3 += 1;
        } // 다음 행까지의 밴드 높이 확장

        for j in (i + 1)..num3.min(n) {
            let aj0 = a.get(j, 0);
            if aj0.abs() > val1.abs() {
                val1 = aj0;
                imax = j;
            }
        }

        // 1-based pivot index 저장 (원본 C++과 동일)
        index[i] = imax + 1;

        if val1 == 0.0 {
            // 원본과 동일한 '작은 값' 방어
            a.set(i, 0, 1e-40);
        }

        // 행 교환 (0..num1-1 열까지만)
        if imax != i {
            *d = -*d;
            for j in 0..num1 {
                let tmp = a.get(i, j);
                a.set(i, j, a.get(imax, j));
                a.set(imax, j, tmp);
            }
        }

        // 하부 제거 (forward elimination in band form)
        for j in (i + 1)..num3.min(n) {
            let r = a.get(j, 0) / a.get(i, 0);
            // al[i][j - i - 1] = r;
            al.set(i, j - i - 1, r);

            // a[j][k-1] = a[j][k] - r * a[i][k]
            for k in 1..num1 {
                let new_val = a.get(j, k) - r * a.get(i, k);
                a.set(j, k - 1, new_val);
            }
            // 마지막 칸 0으로
            a.set(j, num1 - 1, 0.0);
        }
    }
}
```
```rust
pub fn on_bandec_dyn(
    a: &mut dyn DenseMat,
    m1: usize,
    m2: usize,
    al: &mut dyn DenseMat,
    index: &mut [usize],
    d: &mut f64,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1, "al must have at least m1 columns");
    debug_assert_eq!(index.len(), n);

    // 상부로 정렬(슬라이딩) + 왼쪽 0 채우기
    let mut num2 = m1;
    for i in 0..m1 {
        // a[i][0..] ← a[i][(m1-i)..(num1-1)]
        for j in (m1 - i)..num1 {
            let v = a.get(i, j);
            a.set(i, j - num2, v);
        }
        num2 -= 1;
        // 오른쪽 끝쪽을 0으로 채움
        for j in (num1 - num2 - 1)..num1 {
            a.set(i, j, 0.0);
        }
    }

    *d = 1.0;
    let mut num3 = m1;

    for i in 0..n {
        // 피벗 찾기: a[i..min(i+num3-i, n-1)][0] 중 절대값 최대
        let mut val1 = a.get(i, 0);
        let mut imax = i;

        if num3 < n {
            num3 += 1;
        } // 다음 행까지의 밴드 높이 확장

        for j in (i + 1)..num3.min(n) {
            let aj0 = a.get(j, 0);
            if aj0.abs() > val1.abs() {
                val1 = aj0;
                imax = j;
            }
        }

        // 1-based pivot index 저장 (원본 C++과 동일)
        index[i] = imax + 1;

        if val1 == 0.0 {
            // 원본과 동일한 '작은 값' 방어
            a.set(i, 0, 1e-40);
        }

        // 행 교환 (0..num1-1 열까지만)
        if imax != i {
            *d = -*d;
            for j in 0..num1 {
                let tmp = a.get(i, j);
                a.set(i, j, a.get(imax, j));
                a.set(imax, j, tmp);
            }
        }

        // 하부 제거 (forward elimination in band form)
        for j in (i + 1)..num3.min(n) {
            let r = a.get(j, 0) / a.get(i, 0);
            // al[i][j - i - 1] = r;
            al.set(i, j - i - 1, r);

            // a[j][k-1] = a[j][k] - r * a[i][k]
            for k in 1..num1 {
                let new_val = a.get(j, k) - r * a.get(i, k);
                a.set(j, k - 1, new_val);
            }
            // 마지막 칸 0으로
            a.set(j, num1 - 1, 0.0);
        }
    }
}
```
```rust
/// Forward/Backward substitution
///
/// - a: Band matrix containing LU decomposition (n x (m1 + m2 + 1)) — result from `bandec`
/// - al: Lower band of L (n x m1) — result from `bandec`
/// - index: 1-based pivot indices obtained from `bandec`
/// - b: n x n_rhs (right-hand side and solution stored in-place)
pub fn on_banbks<A: DenseMat, L: DenseMat, B: DenseMat>(
    a: &A,
    m1: usize,
    m2: usize,
    al: &L,
    index: &[usize],
    b: &mut B,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1);
    debug_assert_eq!(index.len(), n);
    debug_assert_eq!(b.n_rows(), n, "b must have n rows");

    let n_rhs = b.n_cols();

    for col in 0..n_rhs {
        // 전진 대입: L * y = P*b
        let mut num2 = m1;
        for j in 0..n {
            let ip = index[j] - 1; // 1-based → 0-based
            if ip != j {
                let tmp = b.get(j, col);
                b.set(j, col, b.get(ip, col));
                b.set(ip, col, tmp);
            }

            if num2 < n {
                num2 += 1;
            }

            for k in (j + 1)..num2.min(n) {
                let new_val = b.get(k, col) - al.get(j, k - j - 1) * b.get(j, col);
                b.set(k, col, new_val);
            }
        }

        // 후진 대입: U * x = y  (banded back-substitution)
        let mut num4 = 1usize;
        for j in (0..n).rev() {
            let mut val = b.get(j, col);
            for k in 1..num4 {
                // a[j][k]는 U의 상부밴드; b[k+j][col]는 그 위에 해당하는 y/x
                val -= a.get(j, k) * b.get(j + k, col);
            }
            val /= a.get(j, 0);
            b.set(j, col, val);

            if num4 < num1 {
                num4 += 1;
            }
        }
    }
}
```
```rust
pub fn on_banbks_dyn(
    a: &dyn DenseMat,
    m1: usize,
    m2: usize,
    al: &dyn DenseMat,
    index: &[usize],
    b: &mut dyn DenseMat,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1);
    debug_assert_eq!(index.len(), n);
    debug_assert_eq!(b.n_rows(), n, "b must have n rows");

    let n_rhs = b.n_cols();

    for col in 0..n_rhs {
        // 전진 대입: L * y = P*b
        let mut num2 = m1;
        for j in 0..n {
            let ip = index[j] - 1; // 1-based → 0-based
            if ip != j {
                let tmp = b.get(j, col);
                b.set(j, col, b.get(ip, col));
                b.set(ip, col, tmp);
            }

            if num2 < n {
                num2 += 1;
            }

            for k in (j + 1)..num2.min(n) {
                let new_val = b.get(k, col) - al.get(j, k - j - 1) * b.get(j, col);
                b.set(k, col, new_val);
            }
        }

        // 후진 대입: U * x = y  (banded back-substitution)
        let mut num4 = 1usize;
        for j in (0..n).rev() {
            let mut val = b.get(j, col);
            for k in 1..num4 {
                // a[j][k]는 U의 상부밴드; b[k+j][col]는 그 위에 해당하는 y/x
                val -= a.get(j, k) * b.get(j + k, col);
            }
            val /= a.get(j, 0);
            b.set(j, col, val);

            if num4 < num1 {
                num4 += 1;
            }
        }
    }
}
```
```rust
/// 삼대각 대칭 3×3의 QL-implicit. d=대각, e=상부 초대각(e[2] dummy)
/// QL algorithm with implicit shifts for a symmetric tridiagonal 3x3 matrix.
///
/// 입력/출력:
/// - d: 대각 원소 [d0, d1, d2]  →  계산 후 고윳값들(오름차순은 보장하지 않음)
/// - e: 아랫대각 원소 [e0, e1, _] (e[2]는 사용 안함) → 계산 중 덮어씀
/// - v: Some(V) 이면 V는 3×3이고, k번째 열이 d[k]의 고유벡터로 채워짐
///
/// 행렬 형태:
/// [ d[0]  e[0]   0  ]
/// [ e[0]  d[1]  e[1]]
/// [  0    e[1]  d[2]]
pub fn on_tridiag_ql_implicit(
    d: &mut [f64; 3],
    e: &mut [f64; 3],
    mut v: Option<&mut [[f64; 3]; 3]>,
) -> bool {
    // V 초기화(요청 시 단위행렬)
    if let Some(vv) = &mut v {
        for i in 0..3 {
            for j in 0..3 {
                vv[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }

    // 마지막 오프대각은 사용하지 않으므로 0으로
    e[2] = 0.0;

    for l in 0..3 {
        let mut iter = 0;

        'outer: loop {
            // m 찾기: e[m]가 충분히 작아지는 첫 위치 (또는 맨 끝)
            let mut m = l;
            while m < 2 && (e[m].abs() >= f64::EPSILON * (d[m].abs() + d[m + 1].abs())) {
                m += 1;
            }

            // 수렴: 해당 블록 종료
            if m == l {
                break 'outer;
            }

            iter += 1;
            if iter == 30 {
                // 수렴 실패로 간주
                return false;
            }

            // implicit shift 계산
            let g0 = (d[l + 1] - d[l]) / (2.0 * e[l]);
            let mut r = (g0 * g0 + 1.0).sqrt();
            let mut g = d[m] - d[l]
                + e[l]
                    / if g0 >= 0.0 {
                        g0 + r.abs()
                    } else {
                        g0 - r.abs()
                    };

            let mut s = 1.0f64;
            let mut c = 1.0f64;
            let mut p = 0.0f64;

            // i = m-1 down to l
            let mut i = m - 1;
            loop {
                let f = s * e[i];
                let b = c * e[i];
                r = (f * f + g * g).sqrt();
                e[i + 1] = r;

                if r == 0.0 {
                    // 이 경우 원본 구현은 바깥 반복을 다시 시작(continue)한다
                    d[i + 1] -= p;
                    e[m] = 0.0;
                    continue 'outer;
                }

                s = f / r;
                c = g / r;

                let g2 = d[i + 1] - p;
                r = (d[i] - g2) * s + 2.0 * c * b;
                p = s * r;
                d[i + 1] = g2 + p;
                g = c * r - b;

                // 고유벡터 갱신
                if let Some(vv) = &mut v {
                    for k in 0..3 {
                        let f = vv[k][i + 1];
                        vv[k][i + 1] = s * vv[k][i] + c * f;
                        vv[k][i] = c * vv[k][i] - s * f;
                    }
                }

                if i == l {
                    break;
                }
                i -= 1;
            }

            // 한 턴 마감 갱신
            d[l] -= p;
            e[l] = g;
            e[m] = 0.0;
        }
    }

    true
}
```
```rust
/// 대칭 3×3: [A D F; D B E; F E C] → (eigenvalues d0<=d1<=d2, eigenvectors V(:,k))
pub fn on_sym3_eigen(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) -> Option<([f64; 3], [[f64; 3]; 3])> {
    // 1단계: (1,3) 제거 회전
    let (mut aa, bb, mut cc, mut dd, mut ee) = (a, b, c, d, e);
    let (mut cos_phi, mut sin_phi) = (1.0, 0.0);
    if f != 0.0 {
        let theta = 0.5 * (c - a) / f;
        let t = if theta.abs() > 1.0 {
            1.0 / (theta.abs() * (1.0 + (1.0 + 1.0 / (theta * theta)).sqrt()))
        } else {
            1.0 / (theta.abs() + (1.0 + theta * theta).sqrt())
        } * if theta < 0.0 { -1.0 } else { 1.0 };
        cos_phi = 1.0 / (1.0 + t * t).sqrt();
        sin_phi = t * cos_phi;

        aa = a - t * f;
        cc = c + t * f;
        let tau = sin_phi / (1.0 + cos_phi);
        dd = d - sin_phi * (e + tau * d);
        ee = e + sin_phi * (d - tau * e);
    }

    // 삼대각의 QL
    let mut dvals = [aa, bb, cc];
    let mut evals = [dd, ee, 0.0];
    let mut v = Matrix3x3::eye();
    if !on_tridiag_ql_implicit(&mut dvals, &mut evals, Some(&mut v.0)) {
        return None;
    }

    // 원 좌표계로 회전 복원 (x-z 회전)
    let rot = |col: [f64; 3]| -> [f64; 3] {
        let x = cos_phi * col[0] + sin_phi * col[2];
        let y = col[1];
        let z = -sin_phi * col[0] + cos_phi * col[2];
        [x, y, z]
    };
    let c0 = rot(v.col(0));
    let c1 = rot(v.col(1));
    let c2 = rot(v.col(2));
    Some((dvals, [c0, c1, c2]))
}
```
```rust
pub fn on_offset_point(p1: Point3D, p2: Point3D, length: f64) -> Point3D {
    let v = p2 - p1;
    v.unitize();
    Point3D::new(
        p2.x + length * v.x,
        p2.y + length * v.y,
        p2.z + length * v.z,
    )
}
```
```rust
pub fn on_sort_doubles(arr: &mut [f64], increasing: bool) {
    if arr.len() <= 1 {
        return;
    }
    if increasing {
        arr.sort_by(|a, b| a.total_cmp(b));
    } else {
        arr.sort_by(|a, b| b.total_cmp(a));
    }
}
```
```rust
pub fn on_cull_doubles(arr: &mut Vec<f64>, mut tol: f64) -> usize {
    if arr.len() <= 1 {
        return arr.len();
    }
    arr.sort_by(|a, b| a.total_cmp(b));
    if tol < f64::EPSILON.sqrt() {
        tol = f64::EPSILON.sqrt();
    }
    // 뒤에서 앞으로 스캔하며 근접값 제거
    let mut i = arr.len() - 1;
    let mut d = arr[i];
    while i > 0 {
        let j = i - 1;
        if (d - arr[j]).abs() <= tol {
            arr.remove(j);
        } else {
            d = arr[j];
        }
        i = j;
    }
    arr.shrink_to_fit();
    arr.len()
}
```
```rust
pub fn on_cull_doubles_keep_canonical(arr: &mut Vec<f64>, mut tol: f64) -> usize {
    if arr.len() <= 1 {
        return arr.len();
    }

    if tol < f64::EPSILON.sqrt() {
        tol = f64::EPSILON.sqrt();
    }

    arr.sort_by(|a, b| a.total_cmp(b));

    let mut out = Vec::with_capacity(arr.len());
    let mut last = arr[0];
    out.push(last);

    for &x in arr.iter().skip(1) {
        if (x - last).abs() > tol {
            out.push(x);
            last = x;
        }
    }

    *arr = out;
    arr.shrink_to_fit();
    arr.len()
}
```
```rust
#[allow(unused)]
pub fn on_factorial_u128(n: usize) -> Option<u128> {
    let mut acc: u128 = 1;
    for i in 2..=n {
        acc = acc.checked_mul(i as u128)?;
    }
    Some(acc)
}
```
```rust
#[allow(unused)]
pub fn on_binomial_via_factorial_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let nf = on_factorial_u128(n).unwrap_or(0) as f64;
    let kf = on_factorial_u128(k).unwrap_or(0) as f64;
    let nk = on_factorial_u128(n - k).unwrap_or(0) as f64;
    nf / (kf * nk)
}
```
```rust
#[allow(unused)]
pub fn on_factorial(n: usize) -> i64 {
    if n <= 1 {
        1
    } else {
        (n as i64) * on_factorial(n - 1)
    }
}
```
```rust
pub fn on_rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}
```
```rust
pub fn on_deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
```
```rust
pub fn on_determinant3_vectors(v1: Point3D, v2: Point3D, v3: Point3D) -> f64 {
    v1.x * v2.y * v3.z - v1.x * v2.z * v3.y - v1.y * v2.x * v3.z
        + v1.y * v2.z * v3.x
        + v1.z * v2.x * v3.y
        - v1.z * v2.y * v3.x
}
```
```rust
pub fn on_intersect_3d_lines_option(
    p1: Vector3D,
    d1: Vector3D,
    p2: Vector3D,
    d2: Vector3D,
) -> Option<(f64, f64, Vector3D)> {
    let a = d1.dot(&d1);
    let b = d1.dot(&d2);
    let c = d2.dot(&d2);
    let delta = p2 - p1;
    let e = d1.dot(&delta);
    let f = d2.dot(&delta);
    let denom = a * c - b * b;
    if denom.abs() <= 1e-15 * a.max(1.0) {
        return None;
    }
    let s = (e * c - f * b) / denom;
    let t = (e * b - f * a) / denom;
    let ip = p1 + d1.scale(s);
    Some((s, t, ip))
}
```
```rust
pub fn on_insert_value_into_sorted_array_option(mut v: Vec<f64>, value: f64) -> Option<Vec<f64>> {
    if v.is_empty() {
        return None;
    }
    if value < v[0] || value > *v.last().unwrap() {
        return None;
    }
    v.push(value); // 확장
    let mut i = v.len() - 1;
    while i > 0 && v[i - 1] > value {
        v[i] = v[i - 1];
        i -= 1;
    }
    v[i] = value;
    Some(v)
}
```
```rust
pub fn on_closest_points_of_3d_lines(
    p1: Vector3D,
    d1: Vector3D,
    p2: Vector3D,
    d2: Vector3D,
) -> Result<(f64, f64, Vector3D, Vector3D), &'static str> {
    let a = d1.dot(&d1);
    let b = d1.dot(&d2);
    let c = d2.dot(&d2);
    let w0 = p1 - p2;
    let d = d1.dot(&w0);
    let e = d2.dot(&w0);

    let denom = a * c - b * b;
    if denom.abs() < 1e-30 {
        return Err("parallel or nearly parallel");
    }
    let s = (b * e - c * d) / denom;
    let t = (a * e - b * d) / denom;

    let ps = p1 + d1.scale(s);
    let qt = p2 + d2.scale(t);
    Ok((s, t, ps, qt))
}
```
```rust
pub fn on_calculate_arc_segments(radius: f64, arc_length: f64, chord_length: f64) -> (usize, f64) {
    // Max segment angle = 2 * acos((R - c) / R)
    // Safeguards: zero, negative, or invalid values
    if radius <= 0.0 {
        return (2, arc_length / 2.0);
    }
    let mut ratio = (radius - chord_length) / radius;
    if ratio < -1.0 {
        ratio = -1.0;
    }
    if ratio > 1.0 {
        ratio = 1.0;
    }

    let max_seg_angle = 2.0 * ratio.acos();

    // As max_seg_angle approaches 0, a large number of segments is required.
    // If arc_length == 0, return 2.
    let n = if max_seg_angle > 0.0 {
        ((arc_length.abs() / max_seg_angle).ceil() as i64).max(2) as usize
    } else {
        2
    };
    (n, arc_length / (n as f64))
}
```
```rust
// on_integrate_simpson (적응형 Simpson, 누적 방식)
pub fn on_integrate_simpson<F>(
    mut f: F,
    a: f64,
    b: f64,
    rel_tol: f64,
    max_levels: i32,
    eval_count_out: &mut i32,
    last_delta_out: &mut f64,
) -> f64
where
    F: FnMut(f64) -> f64,
{
    let mut simpson = 0.0_f64;
    let mut prev_simpson = 0.0_f64;
    let mut prev_mid_sum4 = 0.0_f64;
    let mut weighted_sum = 0.0_f64;
    *eval_count_out = 0;
    *last_delta_out = f64::INFINITY;

    for level in 0..=max_levels {
        if level == 0 {
            weighted_sum = f(a) + f(b);
            *eval_count_out = 2;

            // Initial T0 (reference)
            simpson = 0.5 * (b - a) * weighted_sum;
        } else {
            let mid_count = 1 << (level - 1);
            let mid_step = (b - a) / (mid_count as f64);
            let mut x = a + 0.5 * mid_step;
            let mut mid_sum = 0.0;
            for _ in 0..mid_count {
                mid_sum += f(x);
                x += mid_step;
            }
            *eval_count_out += mid_count as i32;

            let mid_sum4 = 4.0 * mid_sum;

            // Add 4 new midpoints and apply correction by subtracting half of the previous midpoint sum (-0.5 * prev_mid_sum4)
            weighted_sum += mid_sum4 - 0.5 * prev_mid_sum4;

            simpson = (b - a) * weighted_sum / ((1 << level) as f64 * 3.0);

            if level >= 5 {
                *last_delta_out = (simpson - prev_simpson).abs();
                if *last_delta_out <= rel_tol * prev_simpson.abs() {
                    return simpson;
                }
            }
            prev_mid_sum4 = mid_sum4;
            prev_simpson = simpson;
        }
    }
    simpson
}
```
```rust
pub fn on_integrate_simpson_simple<F>(f: F, a: f64, b: f64, rel_tol: f64) -> f64
where
    F: FnMut(f64) -> f64,
{
    let mut n = 0;
    let mut d = 0.0;
    on_integrate_simpson(f, a, b, rel_tol, 20, &mut n, &mut d)
}
```
```rust
// ------------------------------------------------------------
// Polynomial f(u) and its derivative f'(u) using Horner's method.
// `a` is the coefficient array.
// If `ascending = true`, the form is: a[0] + a[1] * u + ... + a[n] * u^n
// ------------------------------------------------------------
pub fn on_polynomial_f_df(a: &[f64], u: f64, ascending: bool) -> (f64, f64) {
    let n = a.len().wrapping_sub(1);
    if a.is_empty() {
        return (0.0, 0.0);
    }

    if ascending {
        // Reverse Horner (ascending order of coefficients)
        let mut f = a[n];
        let mut df = 0.0;
        for k in (0..n).rev() {
            df = df * u + f;
            f = f * u + a[k];
        }
        (f, df)
    } else {
        // Stored in descending order: a[0] + a[1] * u + ...
        let mut f = a[0];
        let mut df = 0.0;
        for k in 1..=n {
            df = df * u + f;
            f = f * u + a[k];
        }
        (f, df)
    }
}
```
```rust
/// 외부 API: AX=B -> X
pub fn on_solve_linear_system(a: &Matrix, b: &Matrix) -> Result<Matrix, &'static str> {
    if a.row_count() != a.col_count() {
        return Err("square A required");
    }
    if a.col_count() != b.row_count() {
        return Err("A cols must equal B rows");
    }

    let mut alu = a.clone();
    let piv = on_mat_lu_decompose_partial(&mut alu)?;
    on_mat_lu_solve(&alu, &piv, b)
}
```
```rust
pub fn on_solve_linear_system_vec(a: &mut Matrix, b: &mut [f64]) -> bool {
    let n = a.row_count();
    if a.col_count() != n || b.len() != n {
        return false;
    }

    // LU 분해 (부분 피벗, Doolittle). 행 스왑 시 b도 같이 스왑.
    for k in 0..n {
        // pivot row 찾기 (|a[p,k]| 최대)
        let mut piv = k;
        let mut max_abs = a.at(k as i32, k as i32).abs();
        for i in (k + 1)..n {
            let v = a.at(i as i32, k as i32).abs();
            if v > max_abs {
                max_abs = v;
                piv = i;
            }
        }
        // pivot 이 너무 작으면 실패
        if max_abs <= 1e-30 {
            return false;
        }

        // 행 스왑 (A, b 동시)
        if piv != k {
            for j in 0..n {
                let v_p = { *a.at(piv as i32, j as i32) };
                let v_k = { *a.at(k as i32, j as i32) };
                *a.at_mut(piv as i32, j as i32) = v_k;
                *a.at_mut(k as i32, j as i32) = v_p;
            }
            b.swap(piv, k);
        }
        // 아래 삼각 (L) 갱신 + trailing 상삼각(U) 업데이트
        let akk = a.at(k as i32, k as i32);
        if akk.abs() <= 1e-30 {
            return false;
        }

        let akk = { *a.at(k as i32, k as i32) };

        for i in (k + 1)..n {
            // A[i,k]도 값으로 복사
            let aik = { *a.at(i as i32, k as i32) };
            let lik = aik / akk;

            // 참조가 모두 드롭된 뒤에 쓴다
            {
                *a.at_mut(i as i32, k as i32) = lik;
            }

            // A(i, j) -= L(i,k) * U(k, j)  (j >= k+1)
            for j in (k + 1)..n {
                // 읽기는 전부 값 복사로
                let aij = { *a.at(i as i32, j as i32) };
                let ukj = { *a.at(k as i32, j as i32) };
                let new_val = aij - lik * ukj;
                // 쓰기는 별도 블록에서
                {
                    *a.at_mut(i as i32, j as i32) = new_val;
                }
            }
        }
    }

    // 전진 대입: L y = b  (L의 대각은 1로 가정)
    for i in 0..n {
        let mut s = b[i];
        for j in 0..i {
            s -= a.at(i as i32, j as i32) * b[j];
        }
        b[i] = s; // y 저장(그대로 b에)
    }

    // 후진 대입: U x = y
    for i in (0..n).rev() {
        let mut s = b[i];
        for j in (i + 1)..n {
            s -= a.at(i as i32, j as i32) * b[j];
        }
        let uii = a.at(i as i32, i as i32);
        if uii.abs() <= 1e-30 {
            return false;
        }
        b[i] = s / uii;
    }

    true
}
```
```rust
pub fn on_mat_lu_decompose_partial(a: &mut Matrix) -> Result<Vec<usize>, &'static str> {
    let n = a.row_count() as usize;
    if n == 0 || a.row_count() != a.col_count() {
        return Err("square required");
    }
    let mut piv: Vec<usize> = (0..n).collect();

    for k in 0..n {
        // pivot 선택
        let mut p = k;
        let mut max = a.at(k as i32, k as i32).abs();
        for i in (k + 1)..n {
            let v = a.at(i as i32, k as i32).abs();
            if v > max {
                max = v;
                p = i;
            }
        }
        if max == 0.0 {
            return Err("singular");
        }
        if p != k {
            a.swap_rows(p as i32, k as i32);
            piv.swap(p, k);
        }
        // 아래로 제거
        for i in (k + 1)..n {
            let factor = *a.at(i as i32, k as i32) / *a.at(k as i32, k as i32);
            *a.at_mut(i as i32, k as i32) = factor; // L 저장
            for j in (k + 1)..n {
                let v = *a.at(i as i32, j as i32) - factor * *a.at(k as i32, j as i32);
                *a.at_mut(i as i32, j as i32) = v;
            }
        }
    }
    Ok(piv)
}
```
```rust
/// 전진/후진대입: (PA=LU) 일 때, 다중 RHS B에 대해 X 계산
pub fn on_mat_lu_solve(a_lu: &Matrix, piv: &[usize], b: &Matrix) -> Result<Matrix, &'static str> {
    let n = a_lu.row_count();
    if b.row_count() != n {
        return Err("dim mismatch");
    }
    let r = b.col_count() as usize;

    // 1) PB 만들기
    let mut y = Matrix::with_dims(n, r);
    for i in 0..n {
        let pi = piv[i] as i32;
        for j in 0..r {
            *y.at_mut(i as i32, j as i32) = *b.at(pi, j as i32);
        }
    }

    // 2) L y = P B  (L의 단위대각, 하삼각; L은 a_lu의 하삼각부 + 1.0 on diag)
    for i in 0..n {
        for j in 0..r {
            let mut sum = *y.at(i as i32, j as i32);
            for k in 0..i {
                sum -= *a_lu.at(i as i32, k as i32) * *y.at(k as i32, j as i32);
            }
            *y.at_mut(i as i32, j as i32) = sum;
        }
    }

    // 3) U x = y  (U는 상삼각: a_lu의 상부 포함 대각)
    let mut x = Matrix::with_dims(n, r);
    for j in 0..r {
        for i in (0..n).rev() {
            let mut sum = *y.at(i as i32, j as i32);
            for k in (i + 1)..n {
                sum -= *a_lu.at(i as i32, k as i32) * *x.at(k as i32, j as i32);
            }
            let uii = *a_lu.at(i as i32, i as i32);
            if uii.abs() < f64::EPSILON {
                return Err("singular");
            }
            *x.at_mut(i as i32, j as i32) = sum / uii;
        }
    }
    Ok(x)
}
```
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
```
```rust
pub fn on_line_point_side_2d(a: Point2D, b: Point2D, q: Point2D, eps: f64) -> Side {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    let dx = q.x - a.x;
    let dy = q.y - a.y;
    let z = vx * dy - vy * dx;
    if z >= eps { Side::Left } else { Side::Right }
}
```
```rust
pub fn on_line_point_side_xy(a: Point3D, b: Point3D, q: Point3D, eps: f64) -> Side {
    on_line_point_side_2d(
        Point2D::new(a.x, a.y),
        Point2D::new(b.x, b.y),
        Point2D::new(q.x, q.y),
        eps,
    )
}
```
```rust
/// Projects a 3D input onto a 2D plane defined by an arbitrary reference plane,
/// then determines left/right orientation.
/// If `ref_plane` is `None`, the XY plane is used by default.
pub fn on_line_point_side(
    line: &Segment3D,
    q: Point3D,
    ref_plane: Option<&Plane>,
    eps: f64,
) -> Side {
    if let Some(p) = ref_plane {
        // P의 로컬 (s,t)로 투영
        let proj = |x: Point3D| -> Point2D {
            let d = Vector3D::from_points(&p.origin, &x);
            Point2D::new(Vector3D::dot(&d, &p.x_axis), Vector3D::dot(&d, &p.y_axis))
        };
        let a2 = proj(line.p0);
        let b2 = proj(line.p1);
        let q2 = proj(q);
        return on_line_point_side_2d(a2, b2, q2, eps);
    }
    on_line_point_side_xy(line.p0, line.p1, q, eps)
}
```
```rust
#[inline]
pub fn on_point_distance(a: &Point3D, b: &Point3D) -> f64 {
    a.distance(b)
}
```
```rust
/// Cosine of the rotation angle (cos θ) between vectors a→b and b→c.
/// Returns `None` if any segment has zero length.
pub fn on_turn_cosine(a: &Point3D, b: &Point3D, c: &Point3D) -> Option<f64> {
    let v1 = *b - *a;
    let v2 = *c - *b;
    let d1 = v1.length();
    let d2 = v2.length();
    if d1 <= 0.0 || d2 <= 0.0 {
        return None;
    }
    Some(Point3D::dot(&v1, &v2) / (d1 * d2))
}
```
```rust
// ------------------------------
// Power basis evaluation (curve/surface) + derivatives / rational transformation
// ------------------------------
pub fn on_eval_curve_power3d(a: &[Point3D], degree: usize, t: f64) -> Point3D {
    let mut s = Point3D::new(0.0, 0.0, 0.0);
    let mut tp = 1.0;
    for i in 0..=degree {
        s.x += a[i].x * tp;
        s.y += a[i].y * tp;
        s.z += a[i].z * tp;
        tp *= t;
    }
    s
}
```
```rust
pub fn on_eval_curve_power4d(a: &[Point4D], degree: usize, t: f64) -> Point4D {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    let mut tp = 1.0;
    for i in 0..=degree {
        x += a[i].x * tp;
        y += a[i].y * tp;
        z += a[i].z * tp;
        w += a[i].w * tp;
        tp *= t;
    }
    if w.abs() < 1e-14 {
        Point4D::new(x, y, z, 1.0)
    } else {
        Point4D::new(x, y, z, w)
    }
}
```
```rust
pub fn on_eval_surface_power3d(a: &[Vec<Point3D>], n: usize, m: usize, u: f64, v: f64) -> Point3D {
    let mut s = Point3D::new(0.0, 0.0, 0.0);
    for i in 0..=n {
        let ui = u.powi(i as i32);
        for j in 0..=m {
            let c = ui * v.powi(j as i32);
            s.x += a[i][j].x * c;
            s.y += a[i][j].y * c;
            s.z += a[i][j].z * c;
        }
    }
    s
}
```
```rust
pub fn on_eval_surface_power4d(a: &[Vec<Point4D>], n: usize, m: usize, u: f64, v: f64) -> Point3D {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    for i in 0..=n {
        let ui = u.powi(i as i32);
        for j in 0..=m {
            let c = ui * v.powi(j as i32);
            x += a[i][j].x * c;
            y += a[i][j].y * c;
            z += a[i][j].z * c;
            w += a[i][j].w * c;
        }
    }
    if w.abs() < 1e-14 {
        Point3D::new(x, y, z)
    } else {
        Point3D::new(x / w, y / w, z / w)
    }
}
```
```rust
pub fn on_eval_curve_power3d_deriv(a: &[Point3D], n: usize, t: f64) -> Vector3D {
    let mut d = Vector3D::new(0.0, 0.0, 0.0);
    for i in 1..=n {
        let c = (i as f64) * t.powi((i - 1) as i32);
        d.x += a[i].x * c;
        d.y += a[i].y * c;
        d.z += a[i].z * c;
    }
    d
}
```
```rust
pub fn on_eval_curve_power4d_deriv(a: &[Point4D], n: usize, t: f64) -> Vector3D {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    let mut xd = 0.0;
    let mut yd = 0.0;
    let mut zd = 0.0;
    let mut wd = 0.0;
    for i in 0..=n {
        let ti = t.powi(i as i32);
        x += a[i].x * ti;
        y += a[i].y * ti;
        z += a[i].z * ti;
        w += a[i].w * ti;
        if i >= 1 {
            let c = (i as f64) * t.powi((i - 1) as i32);
            xd += a[i].x * c;
            yd += a[i].y * c;
            zd += a[i].z * c;
            wd += a[i].w * c;
        }
    }
    let w2 = w * w;
    if w2.abs() < 1e-14 {
        return Vector3D::new(0.0, 0.0, 0.0);
    }
    Vector3D::new(
        (xd * w - x * wd) / w2,
        (yd * w - y * wd) / w2,
        (zd * w - z * wd) / w2,
    )
}
```
```rust
pub struct Eval3dD1 {
    pub s: Point3D,
    pub su: Vector3D,
    pub sv: Vector3D,
}
```
```rust
pub struct Eval3dD2 {
    pub s: Point3D,
    pub su: Vector3D,
    pub sv: Vector3D,
    pub suu: Vector3D,
    pub suv: Vector3D,
    pub svv: Vector3D,
}
```
```rust
pub fn on_eval_surface_power3d_d1(
    a: &[Vec<Point3D>],
    n: usize,
    m: usize,
    u: f64,
    v: f64,
) -> Eval3dD1 {
    let mut s = Point3D::new(0.0, 0.0, 0.0);
    let mut su = Vector3D::new(0.0, 0.0, 0.0);
    let mut sv = Vector3D::new(0.0, 0.0, 0.0);

    let mut up = vec![1.0; n + 1];
    let mut vp = vec![1.0; m + 1];
    for i in 1..=n {
        up[i] = up[i - 1] * u;
    }
    for j in 1..=m {
        vp[j] = vp[j - 1] * v;
    }

    let mut dup = vec![0.0; n + 1];
    let mut dvp = vec![0.0; m + 1];
    for i in 1..=n {
        dup[i] = (i as f64) * u.powi((i - 1) as i32);
    }
    for j in 1..=m {
        dvp[j] = (j as f64) * v.powi((j - 1) as i32);
    }

    for i in 0..=n {
        for j in 0..=m {
            let c = up[i] * vp[j];
            let cu = dup[i] * vp[j];
            let cv = up[i] * dvp[j];
            let aij = a[i][j];
            s.x += aij.x * c;
            s.y += aij.y * c;
            s.z += aij.z * c;
            su.x += aij.x * cu;
            su.y += aij.y * cu;
            su.z += aij.z * cu;
            sv.x += aij.x * cv;
            sv.y += aij.y * cv;
            sv.z += aij.z * cv;
        }
    }
    Eval3dD1 { s, su, sv }
}
```
```rust
pub fn on_curve_kappa(d1: Vector3D, d2: Vector3D) -> f64 {
    let c = d1.cross(&d2);
    let n = c.length();
    let s = d1.length();
    if s <= 0.0 { 0.0 } else { n / (s * s * s) }
}
```
```rust
pub fn on_surface_curvature(
    su: Vector3D,
    sv: Vector3D,
    suu: Vector3D,
    suv: Vector3D,
    svv: Vector3D,
) -> Option<(f64, f64, f64, f64)> {
    let e = su.dot(&su);
    let f = su.dot(&sv);
    let g = sv.dot(&sv);
    let n = su.cross(&sv);
    let n_len = n.length();
    if n_len == 0.0 {
        return None;
    }
    let nh = n / n_len;
    let ee = nh.dot(&suu);
    let ff = nh.dot(&suv);
    let gg = nh.dot(&svv);
    let eg_f2 = e * g - f * f;
    if eg_f2 == 0.0 {
        return None;
    }
    let k = (ee * gg - ff * ff) / eg_f2;
    let h = (e * gg - 2.0 * f * ff + g * ee) / (2.0 * eg_f2);
    let disc = (h * h - k).max(0.0);
    let s = disc.sqrt();
    let k1 = h + s;
    let k2 = h - s;
    Some((k, h, k1, k2))
}
```
```rust
// on_compute_bi_normal
pub fn on_compute_bi_normal(t: Vector3D, plane_normal: Vector3D) -> Vector3D {
    // N = plane_normal x T  (Perpendicular to both)
    let mut n = Vector3D::cross(&plane_normal, &t);
    if !n.normalize() {
        // Use auxiliary axis when nearly parallel to T
        let alt = if t.x.abs() < 0.9 {
            Vector3D::new(1.0, 0.0, 0.0)
        } else {
            Vector3D::new(0.0, 1.0, 0.0)
        };
        n = Vector3D::cross(&alt, &t);
        n = n.unitize();
    }
    n
}
```
```rust
// ------------------------------
// 1/4 원호 Bezier(라셔널)용 포인트 + weight
// ------------------------------

pub fn on_make_quarter_arc_bezier(r: f64, z: f64) -> ([Point3D; 3], [f64; 3]) {
    // 0°, 45°, 90° (중간 가중치 = 1/√2)
    let w_mid = (0.5f64).sqrt();
    let row = [
        Point3D::new(r, 0.0, z),
        Point3D::new(r, r, z),
        Point3D::new(0.0, r, z),
    ];
    let w = [1.0, w_mid, 1.0];
    (row, w)
}
```
```rust
pub fn on_plane_eval(pl: &Plane, p: Point3D) -> f64 {
    // Plane 은 이미 equation 을 유지한다고 가정
    pl.equation.value_at_point(p)
}
```
```rust
pub fn on_intersect_line_plane(
    line_from: Point3D,
    line_to: Point3D,
    pl: &Plane,
) -> Option<Point3D> {
    let n = pl.normal();
    let d = -(n.x * pl.origin.x + n.y * pl.origin.y + n.z * pl.origin.z);
    let p0 = line_from;
    let dir = line_to - line_from;
    let denom = Vector3D::dot(&n, &dir.to_vector());
    if denom.abs() < 1e-14 {
        return None;
    }
    let t = -(n.x * p0.x + n.y * p0.y + n.z * p0.z + d) / denom;
    Some(p0 + dir * t)
}
```
```rust
pub fn on_shoot_to_plane(q0: Point3D, w: Vector3D, pl: &Plane) -> Option<Point3D> {
    let n = pl.normal();
    let d = -(n.x * pl.origin.x + n.y * pl.origin.y + n.z * pl.origin.z);
    let denom = Vector3D::dot(&n, &w);
    if denom.abs() < 1e-14 {
        return None;
    }
    let t = -(n.x * q0.x + n.y * q0.y + n.z * q0.z + d) / denom;
    Some(q0 + (w * t).to_point())
}
```
```rust
pub fn on_pass_plane_side(pl: &Plane, p: Point3D, side: Side) -> bool {
    let s = on_plane_eval(pl, p);
    match side {
        Side::Left => s >= 0.0,
        Side::Right => s <= 0.0,
    }
}
```
```rust
// ------------------------------------------------------------
// Newton options
// ------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct PolynomialNewtonOptions {
    pub max_iter: i32,
    pub deriv_eps: f64, // If set to 0, auto value is used: 1e-30 × (1 + |f'|)
    pub ascending: bool,
}
```
```rust
impl Default for PolynomialNewtonOptions {
    fn default() -> Self {
        Self {
            max_iter: 50,
            deriv_eps: 0.0,
            ascending: false,
        }
    }
}
```
```rust
// ------------------------------------------------------------
// polynomial_newton_root
// ------------------------------------------------------------
pub fn on_polynomial_newton_root(
    a: &[f64],
    u0: f64,
    ul: f64,
    ur: f64,
    tol: f64,
    opt: PolynomialNewtonOptions,
) -> Option<f64> {
    if a.is_empty() {
        return None;
    }
    let mut unew = u0.clamp(ul.min(ur), ul.max(ur));

    let maxit = opt.max_iter.max(1);
    for _ in 0..maxit {
        let (f, df) = on_polynomial_f_df(a, unew, opt.ascending);
        if f.abs() < tol {
            return Some(unew);
        }

        let mut deps = opt.deriv_eps;
        if deps <= 0.0 {
            deps = 1e-30 * (1.0 + df.abs());
        }
        if df.abs() <= deps {
            return None;
        } // 나눗셈 불가

        let uold = unew;
        unew = uold - f / df;
        // 구간 보정
        unew = unew.clamp(ul.min(ur), ul.max(ur));

        if ((unew - uold) * df).abs() < tol {
            return Some(unew);
        }
    }
    None
}
```
```rust
// ------------------------------------------------------------
// Bisection bracket root
// ------------------------------------------------------------
pub fn on_bisection_root(
    a: &[f64],
    ul: f64,
    ur: f64,
    tol: f64,
    ascending: bool,
    max_iter: i32,
) -> Option<f64> {
    let (mut f_l, _) = on_polynomial_f_df(a, ul, ascending);
    let (mut f_r, _) = on_polynomial_f_df(a, ur, ascending);

    if f_l == 0.0 {
        return Some(ul);
    }
    if f_r == 0.0 {
        return Some(ur);
    }
    if f_l * f_r > 0.0 {
        return None;
    }

    let (mut a_l, mut a_r) = (ul, ur);
    for _ in 0..max_iter.max(1) {
        let c = 0.5 * (a_l + a_r);
        let (f_c, _) = on_polynomial_f_df(a, c, ascending);
        if f_c.abs() < tol || 0.5 * (a_r - a_l).abs() < tol {
            return Some(c);
        }
        if f_l * f_c <= 0.0 {
            a_r = c;
            f_r = f_c;
        } else {
            a_l = c;
            f_l = f_c;
        }
    }
    Some(0.5 * (a_l + a_r))
}
```
```rust
// ------------------------------------------------------------
// Automatic bracketing + Newton method + fallback to bisection
// ------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct AutoPolynomialNewtonOptions {
    pub samples: i32, // 그리드 샘플 수
    pub ascending: bool,
    pub newton_max_iter: i32,
    pub newton_deriv_eps: f64,
}
```
```rust
impl Default for AutoPolynomialNewtonOptions {
    fn default() -> Self {
        Self {
            samples: 32,
            ascending: false,
            newton_max_iter: 50,
            newton_deriv_eps: 0.0,
        }
    }
}
```
```rust
#[derive(Clone, Copy)]
struct Bracket {
    ul: f64,
    ur: f64,
    fl: f64,
    fr: f64,
}
```
```rust
fn on_find_sign_change_brackets(
    a: &[f64],
    umin: f64,
    umax: f64,
    samples: i32,
    ascending: bool,
) -> Vec<Bracket> {
    let mut out = Vec::new();
    let s = samples.max(2) as usize;
    let du = (umax - umin) / (s as f64);

    let mut u_prev = umin;
    let (mut f_prev, _) = on_polynomial_f_df(a, u_prev, ascending);

    for k in 1..=s {
        let u_cur = if k == s { umax } else { umin + (k as f64) * du };
        let (f_cur, _) = on_polynomial_f_df(a, u_cur, ascending);

        if f_prev == 0.0 || f_cur == 0.0 || f_prev * f_cur < 0.0 {
            let (mut ul, mut ur, mut fl, mut fr) = (u_prev, u_cur, f_prev, f_cur);
            if ul > ur {
                std::mem::swap(&mut ul, &mut ur);
                std::mem::swap(&mut fl, &mut fr);
            }
            out.push(Bracket { ul, ur, fl, fr });
        }
        u_prev = u_cur;
        f_prev = f_cur;
    }
    out
}
```
```rust
fn on_initial_guess_from_bracket(b: Bracket) -> f64 {
    if b.fl == 0.0 {
        return b.ul;
    }
    if b.fr == 0.0 {
        return b.ur;
    }
    let denom = b.fr - b.fl;
    if denom.abs() > 0.0 {
        let u_sec = b.ul - b.fl * (b.ur - b.ul) / denom;
        if u_sec >= b.ul.min(b.ur) && u_sec <= b.ul.max(b.ur) {
            return u_sec;
        }
    }
    0.5 * (b.ul + b.ur)
}
```
```rust
pub fn on_polynomial_auto_bracket_and_newton(
    a: &[f64],
    umin: f64,
    umax: f64,
    tol: f64,
    opt: AutoPolynomialNewtonOptions,
) -> Option<f64> {
    // 1) Sign change bracketing
    let br = on_find_sign_change_brackets(a, umin, umax, opt.samples, opt.ascending);
    if br.is_empty() {
        // Attempt Newton method within a narrow bracket around the grid minimum of |f|
        let samples = opt.samples.max(16);
        let mut best_u = umin;
        let mut best_f = f64::INFINITY;
        for k in 0..=samples {
            let u = umin + (umax - umin) * (k as f64) / (samples as f64);
            let (f, _) = on_polynomial_f_df(a, u, opt.ascending);
            let af = f.abs();
            if af < best_f {
                best_f = af;
                best_u = u;
            }
        }
        let eps = 1e-6_f64.max(tol * 10.0);
        let ul = umin.max(best_u - eps);
        let ur = umax.min(best_u + eps);

        let nopt = PolynomialNewtonOptions {
            max_iter: opt.newton_max_iter,
            deriv_eps: opt.newton_deriv_eps,
            ascending: opt.ascending,
        };
        if let Some(u) = on_polynomial_newton_root(a, best_u, ul, ur, tol, nopt) {
            return Some(u);
        }
        // If a valid bracket is formed, apply bisection
        let (fl, _) = on_polynomial_f_df(a, ul, opt.ascending);
        let (fr, _) = on_polynomial_f_df(a, ur, opt.ascending);
        if fl * fr <= 0.0 {
            return on_bisection_root(a, ul, ur, tol, opt.ascending, 64);
        }
        return None;
    }

    // 2) Select bracket with smallest |f| at midpoint
    let mut best_i = 0usize;
    let mut best_score = f64::INFINITY;
    for (i, b) in br.iter().enumerate() {
        let mid = 0.5 * (b.ul + b.ur);
        let (fm, _) = on_polynomial_f_df(a, mid, opt.ascending);
        let score = fm.abs();
        if score < best_score {
            best_score = score;
            best_i = i;
        }
    }
    let b = br[best_i];

    // 3) Initial guess
    let u0 = on_initial_guess_from_bracket(b);
    let (ul, ur) = (b.ul, b.ur);

    // 4) Newton iteration
    let nopt = PolynomialNewtonOptions {
        max_iter: opt.newton_max_iter,
        deriv_eps: opt.newton_deriv_eps,
        ascending: opt.ascending,
    };
    if let Some(u) = on_polynomial_newton_root(a, u0, ul, ur, tol, nopt) {
        return Some(u);
    }
    // 5) Fallback to bisection
    on_bisection_root(a, ul, ur, tol, opt.ascending, 64)
}
```
```rust
/// C(n,k)
#[inline]
fn binom(n: usize, k_in: usize) -> f64 {
    let k = k_in.min(n - k_in);
    if k == 0 {
        return 1.0;
    }
    let mut num = 1.0;
    let mut den = 1.0;
    for t in 1..=k {
        num *= (n + 1 - t) as f64;
        den *= t as f64;
    }
    num / den
}
```
```rust
/// 2변수 (s,t) 파셜 도함수 블록 인덱스:
/// k = ((i+j)(i+j+1)/2 + j)*v_stride
#[inline]
fn idx2(v_stride: usize, i: usize, j: usize) -> usize {
    let d = i + j;
    (d * (d + 1) / 2 + j) * v_stride
}
```
```rust
/// 3변수 (r,s,t) 파셜 도함수 블록 인덱스:
/// n = v_stride*( d(d+1)(d+2)/6 + (j+k)(j+k+1)/2 + k )
#[inline]
fn idx3(v_stride: usize, i: usize, j: usize, k: usize) -> usize {
    let d = i + j + k;
    let jk = j + k;
    v_stride * (d * (d + 1) * (d + 2) / 6 + jk * (jk + 1) / 2 + k)
}
```
```rust
/// ----- 2변수 버전: F(s,t) = X/W ------------------------------------------
pub fn on_evaluate_quotient_rule2(
    dim: usize,
    der_count: usize,
    v_stride: usize,
    v: &mut [f64],
) -> bool {
    if dim == 0 || v_stride < dim + 1 {
        return false;
    }
    // 총 블록 수 = (der_count+1)(der_count+2)/2
    let need = (der_count + 1) * (der_count + 2) / 2 * v_stride;
    if v.len() < need {
        return false;
    }

    // 1) 전 블록을 w로 나눔
    let w0 = v[dim];
    if w0 == 0.0 {
        return false;
    }
    let invw = 1.0 / w0;
    for off in (0..need).step_by(v_stride) {
        for j in 0..v_stride {
            v[off + j] *= invw;
        }
    }

    if der_count >= 1 {
        // 2) 1차: Fs, Ft
        let f00 = idx2(v_stride, 0, 0);
        let f10 = idx2(v_stride, 1, 0); // Xs/W, Ws/W
        let f01 = idx2(v_stride, 0, 1); // Xt/W, Wt/W
        let ws = -v[f10 + dim];
        let wt = -v[f01 + dim];
        for j in 0..dim {
            v[f10 + j] += ws * v[f00 + j];
            v[f01 + j] += wt * v[f00 + j];
        }

        if der_count >= 2 {
            // 3) 2차: Fss, Fst, Ftt
            let f20 = idx2(v_stride, 2, 0);
            let f11 = idx2(v_stride, 1, 1);
            let f02 = idx2(v_stride, 0, 2);

            let wss = -v[f20 + dim];
            let wst = -v[f11 + dim];
            let wtt = -v[f02 + dim];

            for j in 0..dim {
                let f = v[f00 + j];
                let fs = v[f10 + j];
                let ft = v[f01 + j];

                v[f20 + j] += wss * f + 2.0 * ws * fs; // Dss
                v[f11 + j] += wst * f + wt * fs + ws * ft; // Dst
                v[f02 + j] += wtt * f + 2.0 * wt * ft; // Dtt
            }

            // 4) n>=3 일반식
            // 각 차수 n에서 (i,j) with i+j=n, 블록 F_{i,j} 갱신:
            // F_{i,j} += - sum_{ii=0..i} sum_{jj=0..j} [ (ii||jj)? 1 : 0 ] * C(ii, i-ii)*C(jj, j-jj) * (W_{ii,jj}/W) * F_{i-ii, j-jj}
            for n in 3..=der_count {
                for jtot in 0..=n {
                    let itot = n - jtot;
                    let dst = idx2(v_stride, itot, jtot);
                    let mut acc = vec![0.0f64; dim];

                    for ii in 0..=itot {
                        let c1 = binom(itot, ii);
                        let jstart = if ii == 0 { 1 } else { 0 }; // (ii||jj)?0:1 조건 구현
                        for jj in jstart..=jtot {
                            let c2 = binom(jtot, jj);
                            let wblk = idx2(v_stride, ii, jj);
                            let fsrc = idx2(v_stride, itot - ii, jtot - jj);
                            let wt = -(c1 * c2) * v[wblk + dim]; // -C*C * W_{ii,jj}/W
                            for c in 0..dim {
                                acc[c] += wt * v[fsrc + c];
                            }
                        }
                    }
                    for c in 0..dim {
                        v[dst + c] += acc[c];
                    }
                }
            }
        }
    }

    true
}
```
```rust
/// ----- 3변수 버전: F(r,s,t) = X/W ------------------------------------------
pub fn on_evaluate_quotient_rule3(
    dim: usize,
    der_count: usize,
    v_stride: usize,
    v: &mut [f64],
) -> bool {
    if dim == 0 || v_stride < dim + 1 {
        return false;
    }
    // 총 블록 수 = (d+1)(d+2)(d+3)/6
    let blocks = (der_count + 1) * (der_count + 2) * (der_count + 3) / 6;
    let need = blocks * v_stride;
    if v.len() < need {
        return false;
    }

    // 1) 전 블록을 w로 나눔
    let w0 = v[dim];
    if w0 == 0.0 {
        return false;
    }
    let inv_w = 1.0 / w0;
    for off in (0..need).step_by(v_stride) {
        for j in 0..v_stride {
            v[off + j] *= inv_w;
        }
    }

    if der_count >= 1 {
        // 2) 1차: Fr, Fs, Ft
        let f000 = idx3(v_stride, 0, 0, 0);
        let f100 = idx3(v_stride, 1, 0, 0); // Xr/W, Wr/W
        let f010 = idx3(v_stride, 0, 1, 0); // Xs/W, Ws/W
        let f001 = idx3(v_stride, 0, 0, 1); // Xt/W, Wt/W
        let wr = -v[f100 + dim];
        let ws = -v[f010 + dim];
        let wt = -v[f001 + dim];

        for c in 0..dim {
            let f = v[f000 + c];
            v[f100 + c] += wr * f;
            v[f010 + c] += ws * f;
            v[f001 + c] += wt * f;
        }

        if der_count >= 2 {
            // 3) 2차: Frr, Frs, Frt, Fss, Fst, Ftt
            let f200 = idx3(v_stride, 2, 0, 0);
            let f110 = idx3(v_stride, 1, 1, 0);
            let f101 = idx3(v_stride, 1, 0, 1);
            let f020 = idx3(v_stride, 0, 2, 0);
            let f011 = idx3(v_stride, 0, 1, 1);
            let f002 = idx3(v_stride, 0, 0, 2);

            let wrr = -v[f200 + dim];
            let wrs = -v[f110 + dim];
            let wrt = -v[f101 + dim];
            let wss = -v[f020 + dim];
            let wst = -v[f011 + dim];
            let wtt = -v[f002 + dim];

            for c in 0..dim {
                let f = v[f000 + c];
                let fr = v[f100 + c];
                let fs = v[f010 + c];
                let ft = v[f001 + c];

                v[f200 + c] += wrr * f + 2.0 * wr * fr; // Drr
                v[f110 + c] += wrs * f + wr * fs + ws * fr; // Drs
                v[f101 + c] += wrt * f + wr * ft + wt * fr; // Drt
                v[f020 + c] += wss * f + 2.0 * ws * fs; // Dss
                v[f011 + c] += wst * f + ws * ft + wt * fs; // Dst
                v[f002 + c] += wtt * f + 2.0 * wt * ft; // Dtt
            }

            // 4) n>=3 일반식 (3중합)
            // 각 차수 n, (i,j,k) with i+j+k=n 에 대해:
            // F_{i,j,k} += - sum_{ii=0..i} sum_{jj=0..j} sum_{kk=0..k with !(ii==0&&jj==0&&kk==0)}
            //             C(i,ii) C(j,jj) C(k,kk) * (W_{ii,jj,kk}/W) * F_{i-ii, j-jj, k-kk}
            for n in 3..=der_count {
                for i in (0..=n).rev() {
                    for j in 0..=(n - i) {
                        let k = n - i - j;
                        let dst = idx3(v_stride, i, j, k);
                        let mut acc = vec![0.0f64; dim];

                        for ii in 0..=i {
                            let c1 = binom(i, ii);
                            for jj in 0..=j {
                                let c2 = c1 * binom(j, jj);
                                let k_start = if ii == 0 && jj == 0 { 1 } else { 0 };
                                for kk in k_start..=k {
                                    let wblk = idx3(v_stride, ii, jj, kk);
                                    let f_src = idx3(v_stride, i - ii, j - jj, k - kk);
                                    let wt = -(c2 * binom(k, kk)) * v[wblk + dim];
                                    for c in 0..dim {
                                        acc[c] += wt * v[f_src + c];
                                    }
                                }
                            }
                        }
                        for c in 0..dim {
                            v[dst + c] += acc[c];
                        }
                    }
                }
            }
        }
    }

    true
}
```
```rust
pub fn on_are_in_01(v: f64) -> bool {
    v.is_finite() && 0.0 <= v && v <= 1.0
}
```
```rust
pub fn on_are_on_in_01(v: f64) -> bool {
    v.is_finite() && 0.0 < v && v < 1.0
}
```
```rust
pub fn on_get_shear3(x: &mut [[f64; 3]], sx: f64, sy: f64, sz: f64) {
    for p in x.iter_mut() {
        // 간단한 선형 전단: x' = x + s*y, y' = y + s*z, z' = z + s*x
        p[0] += sx * p[1];
        p[1] += sy * p[2];
        p[2] += sz * p[0];
    }
}
```
```rust
#[allow(unused)]
pub fn on_get_skew2(x: &mut [[f64; 2]], s: f64) {
    for p in x.iter_mut() {
        p[0] += s * p[1];
    }
}
```
```rust
/// 밀집 행렬을 CSR 포맷으로 변환
pub fn on_dense_to_csr(matrix: &[Vec<f64>]) -> (Vec<usize>, Vec<usize>, Vec<f64>) {
    let mut i = Vec::with_capacity(matrix.len() + 1); // 행 포인터
    let mut j = Vec::new(); // 열 인덱스
    let mut a = Vec::new(); // 값

    let mut count = 0;
    i.push(0); // 첫 번째 행 시작은 항상 0

    for row in matrix {
        for (col_idx, &val) in row.iter().enumerate() {
            if val != 0.0 {
                j.push(col_idx);
                a.push(val);
                count += 1;
            }
        }
        i.push(count); // 다음 행 시작 인덱스
    }

    (i, j, a)
}
```
```rust
/// CSR 포맷을 밀집 행렬로 변환
pub fn on_csr_to_dense(i: &[usize], j: &[usize], a: &[f64]) -> Vec<Vec<f64>> {
    let n_rows = i.len() - 1;
    // 열 개수는 j에서 최대값 + 1로 추정
    let n_cols = j.iter().copied().max().unwrap_or(0) + 1;

    let mut dense = vec![vec![0.0; n_cols]; n_rows];

    for row in 0..n_rows {
        let start = i[row];
        let end = i[row + 1];
        for k in start..end {
            let col = j[k];
            let val = a[k];
            dense[row][col] = val;
        }
    }
    dense
}
```
```rust
pub fn on_safe_swap_with_slice<T>(a: &mut [T], b: &mut [T]) {
    // 길이 확인
    assert_eq!(a.len(), b.len(), "슬라이스 길이가 다릅니다.");

    // 메모리 겹침 확인
    let a_start = a.as_ptr() as usize;
    let a_end = unsafe { a.as_ptr().add(a.len()) as usize };
    let b_start = b.as_ptr() as usize;
    let b_end = unsafe { b.as_ptr().add(b.len()) as usize };

    let overlap = a_start < b_end && b_start < a_end;
    assert!(!overlap, "슬라이스가 메모리에서 겹칩니다.");
    // 안전하게 교환
    a.swap_with_slice(b);
}
```
```rust
pub fn on_transport_frame(
    prev_t: Vector3D,
    cur_t: Vector3D,
    x: &mut Vector3D,
    y: &mut Vector3D,
    z: &mut Vector3D,
) {
    let axis = prev_t.cross(&cur_t);
    let s = axis.length();
    let c = Vector3D::dot(&prev_t, &cur_t);
    if s < 1e-9 || c > 1.0 - 1e-12 {
        return;
    } // Nearly parallel: leave as is
    let mut k = axis;
    if !k.normalize() {
        return;
    }
    let angle = on_clamp11(c).acos();
    // Rodrigues Rotation
    let rot = |v: Vector3D| -> Vector3D {
        v * angle.cos()
            + k.cross(&v) * angle.sin()
            + k * (Vector3D::dot(&k, &v) * (1.0 - angle.cos()))
    };
    *x = rot(*x);
    *y = rot(*y);
    *z = rot(*z);
}
```
```rust
// Power → [a,b] shift/scale → Bernstein (1D) / Tensor(3D,4D)
pub fn on_shift_scale_power_basis(a: &[f64], a0: f64, a1: f64) -> Vec<f64> {
    // c[r] = Σ_{k=r..n} a[k] C(k,r) a0^{k-r} · (a1-a0)^r
    let n = a.len().saturating_sub(1);
    let du = a1 - a0;
    let mut c = vec![0.0; n + 1];
    for r in 0..=n {
        let mut acc = 0.0;
        for k in r..=n {
            acc += a[k] * on_binomial(k, r) as f64 * a0.powi((k - r) as i32);
        }
        c[r] = acc * du.powi(r as i32);
    }
    c
}
```
```rust
pub fn on_power_to_bernstein_1d(c: &[f64]) -> Vec<f64> {
    // b[i] = Σ_{r=0..i} c[r] C(i,r)/C(n,r)
    let n = c.len().saturating_sub(1);
    let mut b = vec![0.0; n + 1];
    for i in 0..=n {
        let mut acc = 0.0;
        for r in 0..=i {
            acc += c[r] * (on_binomial(i, r) as f64 / on_binomial(n, r) as f64);
        }
        b[i] = acc;
    }
    b
}
```
```rust
/// in[i][j] = coefficient of u^i v^j
pub fn on_power_to_bernstein_3d_grid(
    input: &[Vec<Point3D>],
    n: usize,
    m: usize,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> Vec<Vec<Point3D>> {
    // 1D transformation in u-direction for each j
    let mut bu_x = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_y = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_z = vec![vec![0.0; m + 1]; n + 1];

    for j in 0..=m {
        // x
        let mut ax = vec![0.0; n + 1];
        for i in 0..=n {
            ax[i] = input[i][j].x;
        }
        let cx = on_shift_scale_power_basis(&ax, u0, u1);
        let bx = on_power_to_bernstein_1d(&cx);
        for i in 0..=n {
            bu_x[i][j] = bx[i];
        }

        // y
        let mut ay = vec![0.0; n + 1];
        for i in 0..=n {
            ay[i] = input[i][j].y;
        }
        let cy = on_shift_scale_power_basis(&ay, u0, u1);
        let by = on_power_to_bernstein_1d(&cy);
        for i in 0..=n {
            bu_y[i][j] = by[i];
        }

        // z
        let mut az = vec![0.0; n + 1];
        for i in 0..=n {
            az[i] = input[i][j].z;
        }
        let cz = on_shift_scale_power_basis(&az, u0, u1);
        let bz = on_power_to_bernstein_1d(&cz);
        for i in 0..=n {
            bu_z[i][j] = bz[i];
        }
    }

    // 1D transformation in v-direction for each i
    let mut out = vec![vec![Point3D::new(0.0, 0.0, 0.0); m + 1]; n + 1];
    for i in 0..=n {
        // x
        let mut ax = vec![0.0; m + 1];
        for j in 0..=m {
            ax[j] = bu_x[i][j];
        }
        let cx = on_shift_scale_power_basis(&ax, v0, v1);
        let bx = on_power_to_bernstein_1d(&cx);

        // y
        let mut ay = vec![0.0; m + 1];
        for j in 0..=m {
            ay[j] = bu_y[i][j];
        }
        let cy = on_shift_scale_power_basis(&ay, v0, v1);
        let by = on_power_to_bernstein_1d(&cy);

        // z
        let mut az = vec![0.0; m + 1];
        for j in 0..=m {
            az[j] = bu_z[i][j];
        }
        let cz = on_shift_scale_power_basis(&az, v0, v1);
        let bz = on_power_to_bernstein_1d(&cz);

        for j in 0..=m {
            out[i][j] = Point3D::new(bx[j], by[j], bz[j]);
        }
    }
    out
}
```
```rust
/// 4D (rational) grid version
pub fn on_power_to_bernstein_4d_grid(
    input: &[Vec<Point4D>],
    n: usize,
    m: usize,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> Vec<Vec<Point4D>> {
    // u-direction
    let mut bu_x = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_y = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_z = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_w = vec![vec![0.0; m + 1]; n + 1];

    for j in 0..=m {
        for comp in 0..4 {
            let mut a = vec![0.0; n + 1];
            for i in 0..=n {
                a[i] = match comp {
                    0 => input[i][j].x,
                    1 => input[i][j].y,
                    2 => input[i][j].z,
                    _ => input[i][j].w,
                };
            }
            let c = on_shift_scale_power_basis(&a, u0, u1);
            let b = on_power_to_bernstein_1d(&c);
            for i in 0..=n {
                match comp {
                    0 => bu_x[i][j] = b[i],
                    1 => bu_y[i][j] = b[i],
                    2 => bu_z[i][j] = b[i],
                    _ => bu_w[i][j] = b[i],
                }
            }
        }
    }

    // v-direction
    let mut out = vec![vec![Point4D::new(0.0, 0.0, 0.0, 0.0); m + 1]; n + 1];
    for i in 0..=n {
        for comp in 0..4 {
            let mut a = vec![0.0; m + 1];
            for j in 0..=m {
                a[j] = match comp {
                    0 => bu_x[i][j],
                    1 => bu_y[i][j],
                    2 => bu_z[i][j],
                    _ => bu_w[i][j],
                };
            }
            let c = on_shift_scale_power_basis(&a, v0, v1);
            let b = on_power_to_bernstein_1d(&c);
            for j in 0..=m {
                match comp {
                    0 => out[i][j].x = b[j],
                    1 => out[i][j].y = b[j],
                    2 => out[i][j].z = b[j],
                    _ => out[i][j].w = b[j],
                }
            }
        }
    }
    out
}

```
```rust
pub fn on_square(x: f64) -> f64 {
    x * x
}

```
```rust
#[inline]
pub fn on_copysign(x: f64, y: f64) -> f64 {
    if y < 0.0 { -x.abs() } else { x.abs() }
}
```
```rust
pub fn on_is_valid_real(x: f64) -> bool {
    x > ON_UNSET_VALUE && x < ON_UNSET_POSITIVE_VALUE
}
```
```rust
#[inline]
pub fn on_degrees_from_radians(angle_in_radians: f64) -> f64 {
    if !on_is_valid_real(angle_in_radians) {
        return angle_in_radians;
    }
    let mut d = angle_in_radians * ON_RADIANS_TO_DEGREES;
    // Attempt snapping to units like 1, 1/2, 1/4, and 1/8
    const SCALE: [f64; 4] = [1.0, 2.0, 4.0, 8.0];
    for &s in &SCALE {
        let ds = d * s;
        let mut f = ds.floor();
        // Round to the "nearest integer" (tie-breaking follows original logic)
        if f + 0.5 < ds {
            f += 1.0;
        }
        // If ds is very close to an integer, snap to that grid
        if (f - ds).abs() < ON_EPSILON * s {
            d = f / s;
            break;
        }
    }
    d
}
```
```rust
#[inline]
pub fn on_radians_from_degrees(angle_in_degrees: f64) -> f64 {
    if on_is_valid_real(angle_in_degrees) {
        angle_in_degrees * ON_DEGREES_TO_RADIANS
    } else {
        angle_in_degrees
    }
}
```
```rust
#[inline]
fn dbg_check_len_f64(dim: usize, slices: &[&[f64]]) {
    debug_assert!(slices.iter().all(|s| s.len() >= dim));
}
```
```rust
#[inline]
fn dbg_check_len_f32(dim: usize, slices: &[&[f32]]) {
    debug_assert!(slices.iter().all(|s| s.len() >= dim));
}
```
```rust
#[inline]
fn dbg_check_len_out_f64(dim: usize, outs: &[&mut [f64]]) {
    debug_assert!(outs.iter().all(|s| s.len() >= dim));
}
```
```rust
#[inline]
fn dbg_check_len_out_f32(dim: usize, outs: &[&mut [f32]]) {
    debug_assert!(outs.iter().all(|s| s.len() >= dim));
}
```
```rust
/// dot = A · B
pub fn on_array_dot_product(dim: usize, a: &[f64], b: &[f64]) -> f64 {
    dbg_check_len_f64(dim, &[a, b]);
    match dim {
        0 => 0.0,
        1 => a[0] * b[0],
        2 => a[0] * b[0] + a[1] * b[1],
        3 => a[0] * b[0] + a[1] * b[1] + a[2] * b[2],
        4 => a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3],
        _ => {
            let mut sum = 0.0;
            for i in 0..dim {
                sum += a[i] * b[i];
            }
            sum
        }
    }
}
```
```rust
/// A · (B - C)
pub fn on_array_dot_difference(dim: usize, a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    dbg_check_len_f64(dim, &[a, b, c]);

    match dim {
        0 => 0.0,
        1 => a[0] * (b[0] - c[0]),
        2 => a[0] * (b[0] - c[0]) + a[1] * (b[1] - c[1]),
        3 => a[0] * (b[0] - c[0]) + a[1] * (b[1] - c[1]) + a[2] * (b[2] - c[2]),
        _ => {
            let mut sum = 0.0;
            for i in 0..dim {
                sum += a[i] * (b[i] - c[i]);
            }
            sum
        }
    }
}
```
```rust
/// ||A - B|| (includes numerically stable branching)
pub fn on_array_distance(dim: usize, a: &[f64], b: &[f64]) -> f64 {
    dbg_check_len_f64(dim, &[a, b]);

    match dim {
        0 => 0.0,
        1 => (b[0] - a[0]).abs(),
        2 => {
            let mut ia = 0usize;
            let mut ib = 0usize;
            let aa = (b[ia] - a[ia]).abs();
            ia += 1;
            ib += 1;
            let bb = (b[ib] - a[ia]).abs();
            if aa > bb {
                let t = bb / aa;
                aa * (1.0 + t * t).sqrt()
            } else if bb > aa {
                let t = aa / bb;
                bb * (1.0 + t * t).sqrt()
            } else {
                aa * ON_SQRT2
            }
        }
        3 => {
            let a0 = (b[0] - a[0]).abs();
            let a1 = (b[1] - a[1]).abs();
            let a2 = (b[2] - a[2]).abs();

            let (m, x, y) = if a0 >= a1 && a0 >= a2 {
                (a0, a1, a2)
            } else if a1 >= a0 && a1 >= a2 {
                (a1, a0, a2)
            } else {
                (a2, a0, a1)
            };

            if m == 0.0 {
                0.0
            } else if m == a0 && a0 == a1 && a0 == a2 {
                m * ON_SQRT3
            } else {
                let rx = x / m;
                let ry = y / m;
                m * (1.0 + (rx * rx + ry * ry)).sqrt()
            }
        }
        _ => {
            let mut sum = 0.0;
            for i in 0..dim {
                let d = b[i] - a[i];
                sum += d * d;
            }
            sum.sqrt()
        }
    }
}
```
```rust
/// ||A - B||^2
pub fn on_array_distance_squared(dim: usize, a: &[f64], b: &[f64]) -> f64 {
    dbg_check_len_f64(dim, &[a, b]);

    let mut sum = 0.0;
    for i in 0..dim {
        let d = b[i] - a[i];
        sum += d * d;
    }
    sum
}
```
```rust
/// ||A||
pub fn on_array_magnitude(dim: usize, a: &[f64]) -> f64 {
    dbg_check_len_f64(dim, &[a]);

    match dim {
        0 => 0.0,
        1 => a[0].abs(),
        2 => {
            let aa = a[0].abs();
            let bb = a[1].abs();
            if aa > bb {
                let t = bb / aa;
                aa * (1.0 + t * t).sqrt()
            } else if bb > aa {
                let t = aa / bb;
                bb * (1.0 + t * t).sqrt()
            } else {
                aa * ON_SQRT2
            }
        }
        3 => {
            let a0 = a[0].abs();
            let a1 = a[1].abs();
            let a2 = a[2].abs();

            let (m, x, y) = if a0 >= a1 && a0 >= a2 {
                (a0, a1, a2)
            } else if a1 >= a0 && a1 >= a2 {
                (a1, a0, a2)
            } else {
                (a2, a0, a1)
            };

            if m == a0 && m == a1 && m == a2 {
                m * ON_SQRT3
            } else {
                let rx = x / m;
                let ry = y / m;
                m * (1.0 + (rx * rx + ry * ry)).sqrt()
            }
        }
        _ => {
            let mut sum = 0.0;
            for i in 0..dim {
                let v = a[i];
                sum += v * v;
            }
            sum.sqrt()
        }
    }
}
```
```rust
/// ||A||^2
pub fn on_array_magnitude_squared(dim: usize, a: &[f64]) -> f64 {
    dbg_check_len_f64(dim, &[a]);

    let mut sum = 0.0;
    for i in 0..dim {
        let v = a[i];
        sum += v * v;
    }
    sum
}
```
```rust
/// sA = s * A
pub fn on_array_scale_f64(dim: usize, s: f64, a: &[f64], s_a: &mut [f64]) {
    dbg_check_len_f64(dim, &[a]);
    dbg_check_len_out_f64(dim, &[s_a]);

    for i in 0..dim {
        s_a[i] = s * a[i];
    }
}
```
```rust
/// aA_plus_B = a*A + B
pub fn on_array_a_a_plus_b_f64(dim: usize, a_s: f64, a: &[f64], b: &[f64], out: &mut [f64]) {
    dbg_check_len_f64(dim, &[a, b]);
    dbg_check_len_out_f64(dim, &[out]);

    for i in 0..dim {
        out[i] = a_s * a[i] + b[i];
    }
}
```
```rust
/*-------- Single-Precision (f32) Implementation --------*/

/// dot = A · B (f32)
pub fn on_array_dot_product_f32(dim: usize, a: &[f32], b: &[f32]) -> f32 {
    dbg_check_len_f32(dim, &[a, b]);

    let mut sum = 0.0f32;
    for i in 0..dim {
        sum += a[i] * b[i];
    }
    sum
}
```
```rust
/// sA = s * A (f32)
pub fn on_array_scale_f32(dim: usize, s: f32, a: &[f32], s_a: &mut [f32]) {
    dbg_check_len_f32(dim, &[a]);
    dbg_check_len_out_f32(dim, &[s_a]);

    for i in 0..dim {
        s_a[i] = s * a[i];
    }
}
```
```rust
/// aA_plus_B = a*A + B (f32)
pub fn on_array_a_a_plus_b_f32(dim: usize, a_s: f32, a: &[f32], b: &[f32], out: &mut [f32]) {
    dbg_check_len_f32(dim, &[a, b]);
    dbg_check_len_out_f32(dim, &[out]);

    for i in 0..dim {
        out[i] = a_s * a[i] + b[i];
    }
}
```
```rust

pub fn on_solve_2x2_optional(a: f64, b: f64, c: f64, d: f64, r1: f64, r2: f64) -> Option<(f64, f64)> {
    let det = a * d - b * c;
    // Threshold for detecting small values considering matrix scale
    let scale = a.abs().max(d.abs()).max(b.abs()).max(c.abs()).max(1.0);
    let tol = f64::EPSILON * scale * 16.0; // 약간 보수적으로
    if det.abs() <= tol {
        return None; // (Nearly singular)
    }
    let du = (r1 * d - b * r2) / det;
    let dv = (a * r2 - c * r1) / det;
    Some((du, dv))
}
```
```rust

/* ========================= 3x2 ========================= */

/// Solve 3x2 with full pivoting.
/// col0, col1: columns (Vector3D)
/// d0,d1,d2: RHS
/// Returns (rank, x, y, err, pivot_ratio) where
/// err = signed distance along (col0 x col1)/|col0 x col1|
pub fn on_solve_3x2(
    col0: Vector3D,
    col1: Vector3D,
    mut d0: f64,
    mut d1: f64,
    mut d2: f64,
) -> (i32, f64, f64, f64, f64) {
    let x_out;
    let y_out;
    let mut err_out = f64::MAX;
    let mut pivot_ratio;

    // find max abs among columns
    let mut i = 0;
    let mut x = col0.x.abs();
    let mut y = col0.y.abs();
    if y > x {
        x = y;
        i = 1;
    }
    y = col0.z.abs();
    if y > x {
        x = y;
        i = 2;
    }
    y = col1.x.abs();
    if y > x {
        x = y;
        i = 3;
    }
    y = col1.y.abs();
    if y > x {
        x = y;
        i = 4;
    }
    y = col1.z.abs();
    if y > x {
        x = y;
        i = 5;
    }
    if x == 0.0 {
        return (0, 0.0, 0.0, err_out, 0.0);
    }
    pivot_ratio = x.abs();

    // possibly swap columns so that first pivot is in col0.x
    let mut c0 = col0;
    let mut c1 = col1;
    let mut xy_swapped = false;
    if i >= 3 {
        // swap columns
        xy_swapped = true;
        c0 = col1;
        c1 = col0;
    }
    // normalize top-left
    match i % 3 {
        1 => {
            // swap rows 0/1
            swap_vals(&mut c0.x, &mut c0.y);
            swap_vals(&mut c1.x, &mut c1.y);
            swap_vals(&mut d0, &mut d1);
        }
        2 => {
            // swap rows 0/2
            swap_vals(&mut c0.x, &mut c0.z);
            swap_vals(&mut c1.x, &mut c1.z);
            swap_vals(&mut d0, &mut d2);
        }
        _ => {}
    }

    // eliminate first column under top pivot (c0.x)
    c1.x /= c0.x;
    d0 /= c0.x;
    let mut t = -c0.y;
    if t != 0.0 {
        c1.y += t * c1.x;
        d1 += t * d0;
    }
    t = -c0.z;
    if t != 0.0 {
        c1.z += t * c1.x;
        d2 += t * d0;
    }

    if c1.y.abs() > c1.z.abs() {
        // pivot at c1.y
        pivot_ratio = if c1.y.abs() > pivot_ratio {
            pivot_ratio / c1.y.abs()
        } else {
            c1.y.abs() / pivot_ratio
        };
        d1 /= c1.y;
        t = -c1.x;
        if t != 0.0 {
            d0 += t * d1;
        }
        t = -c1.z;
        if t != 0.0 {
            d2 += t * d1;
        }

        let (mut x_ans, mut y_ans) = (d0, d1);
        if xy_swapped {
            swap(&mut x_ans, &mut y_ans);
        }
        x_out = x_ans;
        y_out = y_ans;
        err_out = d2;
        return (2, x_out, y_out, err_out, pivot_ratio);
    } else if c1.z == 0.0 {
        return (1, 0.0, 0.0, err_out, pivot_ratio); // rank 1
    } else {
        // pivot at c1.z
        pivot_ratio = if c1.z.abs() > pivot_ratio {
            pivot_ratio / c1.z.abs()
        } else {
            c1.z.abs() / pivot_ratio
        };
        d2 /= c1.z;
        t = -c1.x;
        if t != 0.0 {
            d0 += t * d2;
        }
        t = -c1.y;
        if t != 0.0 {
            d1 += t * d2;
        }

        let (mut x_ans, mut y_ans) = (d0, d2);
        if xy_swapped {
            swap(&mut x_ans, &mut y_ans);
        }
        x_out = x_ans;
        y_out = y_ans;
        err_out = d1;
        return (2, x_out, y_out, err_out, pivot_ratio);
    }
}
```
```rust
#[inline]
fn swap_vals(a: &mut f64, b: &mut f64) {
    let t = *a;
    *a = *b;
    *b = t;
}
```
```rust
/// General NxN solve (Gauss-Jordan to upper-triangular, with optional full pivoting & normalize).
/// - M: mutable rows, each at least n long
/// - B: RHS (length n), overwritten with solution if !bFullPivot, otherwise X에 씀
/// - X: output solution (length n)
/// Returns pivot_ratio (min(|pivot|)/max(|pivot|)) on success, or -rank if degenerate.
pub fn on_solve_nxn(
    b_full_pivot: bool,
    b_normalize: bool,
    n: usize,
    m: &mut [&mut [f64]],
    b: &mut [f64],
    x: &mut [f64],
) -> f64 {
    if n == 0 || m.is_empty() || b.is_empty() || x.is_empty() {
        return 0.0;
    }
    debug_assert!(m.len() >= n && b.len() >= n && x.len() >= n);
    for i in 0..n {
        debug_assert!(m[i].len() >= n);
    }

    // normalize rows (optional)
    if b_normalize {
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += m[i][j] * m[i][j];
            }
            if s > 0.0 {
                let inv = 1.0 / s.sqrt();
                b[i] *= inv;
                for j in 0..n {
                    m[i][j] *= inv;
                }
            }
        }
    }

    // column permutation index for full pivot
    let mut xdex_buf = [0usize; 64];
    let mut xdex_vec = Vec::<usize>::new();
    let xdex: &mut [usize] = if b_full_pivot {
        let dst = if n <= 64 {
            &mut xdex_buf[..n]
        } else {
            xdex_vec.resize(n, 0);
            &mut xdex_vec[..]
        };
        for i in 0..n {
            dst[i] = i;
        }
        dst
    } else {
        &mut []
    };

    let mut minpivot = 0.0;
    let mut maxpivot = 1.0;

    // forward elimination to upper-triangular
    for n0 in 0..n {
        // find max pivot in submatrix
        let mut maxi = n0;
        let mut maxj = n0;
        let mut xabs = 0.0;

        for j in n0..n {
            for i in n0..n {
                let v = m[i][j].abs();
                if v > xabs {
                    xabs = v;
                    maxi = i;
                    maxj = j;
                }
            }
            if !b_full_pivot {
                break;
            }
        }

        if xabs == 0.0 {
            // degenerate: rank = n0
            return -(n0 as f64);
        } else if n0 == 0 {
            minpivot = xabs;
            maxpivot = xabs;
        } else if xabs < minpivot {
            minpivot = xabs;
        } else if xabs > maxpivot {
            maxpivot = xabs;
        }

        // swap rows
        if n0 != maxi {
            m.swap(n0, maxi);
            b.swap(n0, maxi);
        }
        // swap cols (only when full pivot)
        if b_full_pivot && n0 != maxj {
            for i in 0..n {
                m[i].swap(n0, maxj);
            }
            xdex.swap(n0, maxj);
        }

        // unitize pivot row
        let piv_inv = 1.0 / m[n0][n0];
        b[n0] *= piv_inv;
        for j in (n0 + 1)..n {
            m[n0][j] *= piv_inv;
        }

        // eliminate below
        for i in (n0 + 1)..n {
            let coeff = -m[i][n0];
            if coeff != 0.0 {
                b[i] += coeff * b[n0];
                for j in (n0 + 1)..n {
                    m[i][j] += coeff * m[n0][j];
                }
            }
        }
    }

    // back_solve (now upper-triangular with 1s on diagonal)
    for j in (0..n).rev() {
        for i in 0..j {
            let coeff = -m[i][j];
            if coeff != 0.0 {
                b[i] += coeff * b[j];
            }
        }
    }

    // write solution
    if b_full_pivot {
        for i in 0..n {
            x[xdex[i]] = b[i];
        }
    } else {
        x[..n].copy_from_slice(&b[..n]);
    }

    minpivot / maxpivot
}

```
```rust
/* ========================= 4x4 ========================= */

/// 4x4 Gauss–Jordan + full pivoting.
/// Returns (rank, x, y, z, w, pivot_ratio).
pub fn solve_4x4_gaussj(
    row0: [f64; 4],
    row1: [f64; 4],
    row2: [f64; 4],
    row3: [f64; 4],
    d0: f64,
    d1: f64,
    d2: f64,
    d3: f64,
) -> (i32, f64, f64, f64, f64, f64) {
    // assemble augmented rows
    let mut p0 = [row0[0], row0[1], row0[2], row0[3], d0];
    let mut p1 = [row1[0], row1[1], row1[2], row1[3], d1];
    let mut p2 = [row2[0], row2[1], row2[2], row2[3], d2];
    let mut p3 = [row3[0], row3[1], row3[2], row3[3], d3];

    let mut x_idx = 0usize; // 0:x,1:y,2:z,3:w 매핑 추적용
    let mut y_idx = 1usize;
    let mut z_idx = 2usize;
    let mut w_idx = 3usize;

    // find max pivot
    let mut i = 0;
    let mut j = 0;
    let mut x = row0[0].abs();
    let mut y = row0[1].abs();
    if y > x {
        x = y;
        j = 1;
    }
    y = row0[2].abs();
    if y > x {
        x = y;
        j = 2;
    }
    y = row0[3].abs();
    if y > x {
        x = y;
        j = 3;
    }
    y = row1[0].abs();
    if y > x {
        x = y;
        i = 1;
        j = 0;
    }
    y = row1[1].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }
    y = row1[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 2;
    }
    y = row1[3].abs();
    if y > x {
        x = y;
        i = 1;
        j = 3;
    }
    y = row2[0].abs();
    if y > x {
        x = y;
        i = 2;
        j = 0;
    }
    y = row2[1].abs();
    if y > x {
        x = y;
        i = 2;
        j = 1;
    }
    y = row2[2].abs();
    if y > x {
        x = y;
        i = 2;
        j = 2;
    }
    y = row2[3].abs();
    if y > x {
        x = y;
        i = 2;
        j = 3;
    }
    y = row3[0].abs();
    if y > x {
        x = y;
        i = 3;
        j = 0;
    }
    y = row3[1].abs();
    if y > x {
        x = y;
        i = 3;
        j = 1;
    }
    y = row3[2].abs();
    if y > x {
        x = y;
        i = 3;
        j = 2;
    }
    y = row3[3].abs();
    if y > x {
        x = y;
        i = 3;
        j = 3;
    }

    if x == 0.0 {
        return (0, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    let mut minpiv = x;
    let mut maxpiv = x;

    // swap rows so pivot is at p0[0]
    match i {
        1 => swap(&mut p0, &mut p1),
        2 => swap(&mut p0, &mut p2),
        3 => swap(&mut p0, &mut p3),
        _ => {}
    }
    // swap columns (track variable mapping)
    match j {
        1 => {
            swap_cols(&mut p0, &mut p1, &mut p2, &mut p3, 0, 1);
            swap(&mut x_idx, &mut y_idx);
        }
        2 => {
            swap_cols(&mut p0, &mut p1, &mut p2, &mut p3, 0, 2);
            swap(&mut x_idx, &mut z_idx);
        }
        3 => {
            swap_cols(&mut p0, &mut p1, &mut p2, &mut p3, 0, 3);
            swap(&mut x_idx, &mut w_idx);
        }
        _ => {}
    }

    // eliminate column 0
    scale_row(&mut p0, 0);
    elim_col(&mut p1, &p0, 0);
    elim_col(&mut p2, &p0, 0);
    elim_col(&mut p3, &p0, 0);

    // pick next pivot in submatrix (rows 1.., cols 1..)
    let (i2, j2, val2) = max_in_3x3(&p1, &p2, &p3);
    if val2 == 0.0 {
        // rank 1
        let sol = back_collect(&[&p0, &p1, &p2, &p3], x_idx, y_idx, z_idx, w_idx);
        return (1, sol[0], sol[1], sol[2], sol[3], 0.0);
    }
    update_minmax(val2, &mut minpiv, &mut maxpiv);

    if j2 == 1 {
        swap_cols(&mut p0, &mut p1, &mut p2, &mut p3, 1, 2);
        swap(&mut y_idx, &mut z_idx);
    } else if j2 == 2 {
        swap_cols(&mut p0, &mut p1, &mut p2, &mut p3, 1, 3);
        swap(&mut y_idx, &mut w_idx);
    }
    if i2 == 1 {
        swap(&mut p1, &mut p2);
    } else if i2 == 2 {
        swap(&mut p1, &mut p3);
    }

    // eliminate column 1
    scale_row(&mut p1, 1);
    elim_col(&mut p2, &p1, 1);
    elim_col(&mut p3, &p1, 1);

    // next pivot in 2×2 tail
    let (i3, j3, val3) = max_in_2x2(&p2, &p3);
    if val3 == 0.0 {
        // rank 2
        let mut sol = [0.0; 4];
        // y = p2[4] ; x = p0[4] - p0[1]*y (Column swaps are mapped at the final stage)
        let yv = p2[4];
        let xv = p0[4] - p0[1] * yv;
        // mapping
        sol[x_idx] = xv;
        sol[y_idx] = yv;
        return (2, sol[0], sol[1], sol[2], sol[3], 0.0);
    }
    update_minmax(val3, &mut minpiv, &mut maxpiv);

    if j3 == 1 {
        swap_cols(&mut p0, &mut p1, &mut p2, &mut p3, 2, 3);
        swap(&mut z_idx, &mut w_idx);
    }
    if i3 == 1 {
        swap(&mut p2, &mut p3);
    }

    // eliminate column 2
    scale_row(&mut p2, 2);
    elim_col(&mut p3, &p2, 2);

    // final pivot
    let val4 = p3[3].abs();
    if val4 == 0.0 {
        // rank 3
        let mut sol = [0.0; 4];
        let z = p2[4];
        let y = p1[4] - p1[2] * z;
        let x = p0[4] - p0[1] * y - p0[2] * z;
        sol[x_idx] = x;
        sol[y_idx] = y;
        sol[z_idx] = z;
        return (3, sol[0], sol[1], sol[2], 0.0, 0.0);
    }
    update_minmax(val4, &mut minpiv, &mut maxpiv);

    // back_solve
    p3[4] /= p3[3];
    p2[4] -= p2[3] * p3[4];
    p1[4] -= p1[2] * p2[4] + p1[3] * p3[4];
    p0[4] -= p0[1] * p1[4] + p0[2] * p2[4] + p0[3] * p3[4];

    let mut sol = [0.0; 4];
    sol[x_idx] = p0[4];
    sol[y_idx] = p1[4];
    sol[z_idx] = p2[4];
    sol[w_idx] = p3[4];

    (4, sol[0], sol[1], sol[2], sol[3], minpiv / maxpiv)
}
```
```rust
fn swap_cols(
    p0: &mut [f64; 5],
    p1: &mut [f64; 5],
    p2: &mut [f64; 5],
    p3: &mut [f64; 5],
    c1: usize,
    c2: usize,
) {
    p0.swap(c1, c2);
    p1.swap(c1, c2);
    p2.swap(c1, c2);
    p3.swap(c1, c2);
}
```
```rust
fn scale_row(p: &mut [f64; 5], k: usize) {
    let inv = 1.0 / p[k];
    for j in (k + 1)..4 {
        p[j] *= inv;
    }
    p[4] *= inv;
}
```
```rust
fn elim_col(dst: &mut [f64; 5], src: &[f64; 5], k: usize) {
    let co_eff = -dst[k];
    if co_eff != 0.0 {
        for j in (k + 1)..4 {
            dst[j] += co_eff * src[j];
        }
        dst[4] += co_eff * src[4];
        // dst[k]=0 (코멘트)
    }
}
```
```rust
fn max_in_3x3(p1: &[f64; 5], p2: &[f64; 5], p3: &[f64; 5]) -> (usize, usize, f64) {
    // among {p1[1..=3], p2[1..=3], p3[1..=3]} pick max abs -> (row_idx 0..2, col_idx 1..3, val)
    let mut i = 0;
    let mut j = 1;
    let mut x = p1[1].abs();
    let mut y = p1[2].abs();
    if y > x {
        x = y;
        j = 2;
    }
    y = p1[3].abs();
    if y > x {
        x = y;
        j = 3;
    }
    y = p2[1].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }
    y = p2[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 2;
    }
    y = p2[3].abs();
    if y > x {
        x = y;
        i = 1;
        j = 3;
    }
    y = p3[1].abs();
    if y > x {
        x = y;
        i = 2;
        j = 1;
    }
    y = p3[2].abs();
    if y > x {
        x = y;
        i = 2;
        j = 2;
    }
    y = p3[3].abs();
    if y > x {
        x = y;
        i = 2;
        j = 3;
    }
    (i, j - 1, x) // j-1: 우리는 앞에서 1과 2를 각각 (1,2)로 구분했지만 호출부에서 다시 보정
}
```
```rust
fn max_in_2x2(p2: &[f64; 5], p3: &[f64; 5]) -> (usize, usize, f64) {
    // among {p2[2], p2[3], p3[2], p3[3]}
    let mut i = 0;
    let mut j = 0;
    let mut x = p2[2].abs();
    let mut y = p2[3].abs();
    if y > x {
        x = y;
        j = 1;
    }
    y = p3[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 0;
    }
    y = p3[3].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }
    (i, j, x)
}
```
```rust
fn update_minmax(v: f64, minp: &mut f64, maxp: &mut f64) {
    if v > *maxp {
        *maxp = v;
    } else if v < *minp {
        *minp = v;
    }
}
```
```rust
fn back_collect(rows: &[&[f64; 5]; 4], xi: usize, yi: usize, zi: usize, wi: usize) -> [f64; 4] {
    let mut sol = [0.0; 4];
    sol[xi] = rows[0][4];
    sol[yi] = 0.0;
    sol[zi] = 0.0;
    sol[wi] = 0.0;
    sol
}
```
```rust
/// 4x4 LU decomposition (partial pivoting). A and b are updated in-place, and on success, the solution is written into b.
pub fn solve_4x4_lu(a: &mut [[f64; 4]; 4], b: &mut [f64; 4]) -> bool {
    for k in 0..4 {
        // Pivot selection
        let mut max_i = k;
        let mut max_v = a[k][k].abs();
        for i in (k + 1)..4 {
            let v = a[i][k].abs();
            if v > max_v {
                max_v = v;
                max_i = i;
            }
        }
        if max_v <= 0.0 {
            return false;
        } // 특이

        // --- Right here! If it's a full row swap, it's done in a single line ---
        if max_i != k {
            a.swap(k, max_i); // Row swap
            b.swap(k, max_i); // Swap the right-hand side vector along with the matrix row
        }

        // Elimination
        let akk = a[k][k];
        for i in (k + 1)..4 {
            let m = a[i][k] / akk;
            a[i][k] = m; // L 저장
            for j in (k + 1)..4 {
                a[i][j] -= m * a[k][j];
            }
            b[i] -= m * b[k];
        }
    }

    // Back-substitution
    let mut x = [0.0; 4];
    for i in (0..4).rev() {
        let mut s = b[i];
        for j in (i + 1)..4 {
            s -= a[i][j] * x[j];
        }
        x[i] = s / a[i][i];
    }
    b.copy_from_slice(&x);
    true
}
```
```rust
/* ========================= 3x3 ========================= */
/// 3x3 Gauss–Jordan + full pivoting
/// Returns (rank, x, y, z, pivot_ratio)
pub fn solve_3x3_gaussj(
    row0: [f64; 3],
    row1: [f64; 3],
    row2: [f64; 3],
    d0: f64,
    d1: f64,
    d2: f64,
) -> (i32, f64, f64, f64, f64) {
    let mut w = [
        [row0[0], row0[1], row0[2], d0],
        [row1[0], row1[1], row1[2], d1],
        [row2[0], row2[1], row2[2], d2],
    ];
    let (mut x_idx, mut y_idx, mut z_idx) = (0usize, 1usize, 2usize);

    // 1) 첫 피벗 선택 및 행/열 스왑
    let mut i = 0;
    let mut j = 0;
    let mut x = row0[0].abs();
    let mut y = row0[1].abs();
    if y > x {
        x = y;
        j = 1;
    }
    y = row0[2].abs();
    if y > x {
        x = y;
        j = 2;
    }
    y = row1[0].abs();
    if y > x {
        x = y;
        i = 1;
        j = 0;
    }
    y = row1[1].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }
    y = row1[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 2;
    }
    y = row2[0].abs();
    if y > x {
        x = y;
        i = 2;
        j = 0;
    }
    y = row2[1].abs();
    if y > x {
        x = y;
        i = 2;
        j = 1;
    }
    y = row2[2].abs();
    if y > x {
        x = y;
        i = 2;
        j = 2;
    }
    if x == 0.0 {
        return (0, 0.0, 0.0, 0.0, 0.0);
    }
    let mut minpiv = x;
    let mut maxpiv = x;

    if i != 0 {
        w.swap(0, i);
    }
    if j == 1 {
        for r in &mut w {
            r.swap(0, 1);
        }
        std::mem::swap(&mut x_idx, &mut y_idx);
    } else if j == 2 {
        for r in &mut w {
            r.swap(0, 2);
        }
        std::mem::swap(&mut x_idx, &mut z_idx);
    }

    // 2) 열0 소거
    let inv = 1.0 / w[0][0];
    for c in 1..4 {
        w[0][c] *= inv;
    } // pivot=1처럼 사용
    for r in 1..3 {
        let coeff = -w[r][0];
        if coeff != 0.0 {
            for c in 1..4 {
                w[r][c] += coeff * w[0][c];
            }
            w[r][0] = 0.0; // ★ 소거된 자리 0으로!
        }
    }

    // 3) 두 번째 피벗 선택 (submatrix)
    let (mut i2, mut j2, mut v) = (1usize, 1usize, w[1][1].abs());
    let mut t = w[1][2].abs();
    if t > v {
        v = t;
        j2 = 2;
    }
    t = w[2][1].abs();
    if t > v {
        v = t;
        i2 = 2;
        j2 = 1;
    }
    t = w[2][2].abs();
    if t > v {
        v = t;
        i2 = 2;
        j2 = 2;
    }
    if v == 0.0 {
        return (1, w[0][3], 0.0, 0.0, 0.0);
    }
    if v > maxpiv {
        maxpiv = v;
    } else if v < minpiv {
        minpiv = v;
    }

    if j2 == 2 {
        for r in &mut w {
            r.swap(1, 2);
        }
        std::mem::swap(&mut y_idx, &mut z_idx);
    }
    if i2 == 2 {
        w.swap(1, 2);
    }

    // 4) 열1 소거 (위/아래 모두)
    let inv = 1.0 / w[1][1];
    for c in 2..4 {
        w[1][c] *= inv;
    }
    {
        let coeff0 = -w[0][1];
        if coeff0 != 0.0 {
            for c in 2..4 {
                w[0][c] += coeff0 * w[1][c];
            }
            w[0][1] = 0.0; // ★ Must be set to 0!
        }
        let coeff2 = -w[2][1];
        if coeff2 != 0.0 {
            for c in 2..4 {
                w[2][c] += coeff2 * w[1][c];
            }
            w[2][1] = 0.0; // ★ Must be set to 0!
        }
    }

    // 5) 마지막 피벗 (2,2)
    let p = w[2][2];
    if p == 0.0 {
        return (2, w[0][3], w[1][3], 0.0, minpiv / maxpiv);
    }
    let v = p.abs();
    if v > maxpiv {
        maxpiv = v;
    } else if v < minpiv {
        minpiv = v;
    }
    let z = w[2][3] / p;

    // 6) back-sub (상삼각)
    let y = w[1][3] - w[1][2] * z;
    let x = w[0][3] - w[0][2] * z; // w[0][1] is explicitly set to 0

    let mut sol = [0.0; 3];
    sol[x_idx] = x;
    sol[y_idx] = y;
    sol[z_idx] = z;
    (3, sol[0], sol[1], sol[2], minpiv / maxpiv)
}
```
```rust
/// 3x3 Cramer
pub fn solve_3x3_cramer(a: [f64; 9], b: [f64; 3]) -> Option<Vector3D> {
    let det = |m: &[f64; 9]| -> f64 {
        m[0] * (m[4] * m[8] - m[5] * m[7]) - m[1] * (m[3] * m[8] - m[5] * m[6])
            + m[2] * (m[3] * m[7] - m[4] * m[6])
    };
    let d = det(&a);
    if d.abs() < 1e-15 {
        return None;
    }
    let mx = [b[0], a[1], a[2], b[1], a[4], a[5], b[2], a[7], a[8]];
    let my = [a[0], b[0], a[2], a[3], b[1], a[5], a[6], b[2], a[8]];
    let mz = [a[0], a[1], b[0], a[3], a[4], b[1], a[6], a[7], b[2]];
    Some(Vector3D::new(det(&mx) / d, det(&my) / d, det(&mz) / d))
}
```
```rust
/* ========================= Decompose V onto {A,B} ========================= */

/// Solve V = xA + yB and return (x, y). (Project by solving 2×2 system)
pub fn on_decompose_vector(v: Vector3D, a: Vector3D, b: Vector3D) -> Option<(f64, f64)> {
    let aov = Vector3D::dot(&a, &v);
    let bov = Vector3D::dot(&b, &v);
    let aoa = Vector3D::dot(&a, &a);
    let aob = Vector3D::dot(&a, &b);
    let bob = Vector3D::dot(&b, &b);
    let (rank, x, y, _pr) = on_solve_2x2_tuple(aoa, aob, aob, bob, aov, bov);
    if rank == 2 { Some((x, y)) } else { None }
}
```
```rust
//----------------------------------------------
// 1) Closest Parameter on a Line
//----------------------------------------------
pub fn on_line_closest_point(line_p: Point3D, line_dir: Vector3D, test: Point3D) -> Option<f64> {
    let s = Vector3D::new(test.x - line_p.x, test.y - line_p.y, test.z - line_p.z);
    let denom = line_dir.length_squared();
    if denom < ON_EPSILON {
        return None;
    }
    Some(Vector3D::dot(&s, &line_dir) / denom)
}
```
```rust
//----------------------------------------------
// 2) Insert value into sorted vector (returns false if out of bounds)
//----------------------------------------------
pub fn on_insert_value_into_sorted_array(value: f64, arr: &mut Vec<f64>) -> bool {
    if arr.is_empty() {
        return false;
    }
    if value < arr[0] || value > arr[arr.len() - 1] {
        return false;
    }
    let pos = match arr.binary_search_by(|x| x.partial_cmp(&value).unwrap()) {
        Ok(i) => i,  // 동일값 위치
        Err(i) => i, // 들어갈 자리
    };
    arr.insert(pos, value);
    true
}
```
```rust
//----------------------------------------------
// 3) Significance Test for Difference
//----------------------------------------------
pub fn on_check_difference_significant(a: f64, b: f64, c: f64) -> (bool, f64) {
    let ac = a * c;
    let b2 = b * b;
    let diff = ac - b2;
    let cond = a > c * std::f64::EPSILON
        && c > a * std::f64::EPSILON
        && diff.abs() > ac.max(b2) * ON_SQRT_EPSILON;
    (cond, diff)
}
```
```rust
pub fn add_point_without_duplicate_vec(
    point: Point3D,
    points: &mut Vec<Point3D>,
    length: f64,
    add: bool,
) -> bool {
    if add || points.is_empty() {
        points.push(point);
        return true;
    }
    for pt in points.iter() {
        if pt.distance(&point) / length < SQRT_EPS {
            return false;
        }
    }
    points.push(point);
    true
}
```
```rust
pub fn compute_uv_parameters(
    points: &TMatrix<Point3D>,
    u_params: &mut Vec<f64>,
    v_params: &mut Vec<f64>,
) -> bool {
    let rows = points.rows();
    let cols = points.cols();
    u_params.clear();
    u_params.resize(rows, 0.0);
    v_params.clear();
    v_params.resize(cols, 0.0);

    let mut tmp = vec![0.0_f64; rows];
    let mut valid_cols = cols as i32;

    for i in 0..cols {
        let mut total = 0.0;
        for j in 1..rows {
            tmp[j] = points[(j, i)].distance(&points[(j - 1, i)]);
            total += tmp[j];
        }
        if total == 0.0 {
            valid_cols -= 1;
        } else {
            let mut acc = 0.0;
            for j in 1..rows {
                acc += tmp[j];
                u_params[j] += acc / total;
            }
        }
    }
    if valid_cols <= 0 {
        return false;
    }
    for i in 1..rows - 1 {
        u_params[i] /= valid_cols as f64;
    }
    u_params[rows - 1] = 1.0;

    // v
    let mut tmp2 = vec![0.0_f64; cols];
    let mut valid_rows = rows as i32;
    for r in 0..rows {
        let mut total = 0.0;
        for c in 1..cols {
            tmp2[c] = points[(r, c)].distance(&points[(r, c - 1)]);
            total += tmp2[c];
        }
        if total == 0.0 {
            valid_rows -= 1;
        } else {
            let mut acc = 0.0;
            for c in 1..cols {
                acc += tmp2[c];
                v_params[c] += acc / total;
            }
        }
    }
    if valid_rows <= 0 {
        return false;
    }
    for c in 1..cols - 1 {
        v_params[c] /= valid_rows as f64;
    }
    v_params[cols - 1] = 1.0;
    true
}
```
```rust
pub fn on_distance_to_bbox(p: Point3D, min: Point3D, max: Point3D) -> f64 {
    let clamp = |v: f64, lo: f64, hi: f64| {
        if v < lo {
            lo
        } else if v > hi {
            hi
        } else {
            v
        }
    };
    let qx = clamp(p.x, min.x - SQRT_EPS, max.x + SQRT_EPS);
    let qy = clamp(p.y, min.y - SQRT_EPS, max.y + SQRT_EPS);
    let qz = clamp(p.z, min.z - SQRT_EPS, max.z + SQRT_EPS);
    let dx = p.x - qx;
    let dy = p.y - qy;
    let dz = p.z - qz;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
```
```rust
pub fn on_get_outer_index_loops2d(loops: &[Vec<Point2D>]) -> i32 {
    if loops.is_empty() {
        return -1;
    }
    let mut idx = 0;
    let mut loop_poly = loops[idx].clone();

    let mut i = 0usize;
    while i < loops.len() {
        if !loops[i].is_empty() && i != idx && idx < loops.len() - 1 {
            let test_pt = loops[i][0];
            if !on_point_in_polygon_2d(test_pt, &loop_poly) {
                idx += 1;
                loop_poly = loops[idx].clone();
                i = 0;
                continue;
            }
        }
        i += 1;
    }
    if idx >= loops.len() { -1 } else { idx as i32 }
}
```
```rust
pub fn on_point_in_polygon_2d(test: Point2D, poly: &[Point2D]) -> bool {
    let n = poly.len();
    if n < 2 {
        return false;
    }
    let mut cnt = 0;
    let mut j = n - 1;
    for i in 0..n {
        let pi = poly[i];
        let pj = poly[j];
        let cond = ((pi.y <= test.y && test.y < pj.y) || (pj.y <= test.y && test.y < pi.y))
            && test.x < (pj.x - pi.x) * (test.y - pi.y) / (pj.y - pi.y) + pi.x;
        if cond {
            cnt += 1;
        }
        j = i;
    }
    (cnt & 1) == 1
}
```
```rust
pub fn on_point_coincidence(
    p: Vector3D,
    q: Vector3D,
    problem_size: f64,
    r_out: &mut Vector3D,
    r_len_out: &mut f64,
) -> bool {
    *r_out = q - p;
    *r_len_out = r_out.length();
    *r_len_out / problem_size < 1.0e-10
}
```
```rust
pub fn polar_angle(x: f64, y: f64) -> f64 {
    // 원본 로직 유지
    let mut a = if x != 0.0 {
        (y / x).atan()
    } else {
        if y <= 0.0 { 1.5 * PI } else { 0.5 * PI }
    };
    if x < 0.0 {
        a += PI;
    }
    if x > 0.0 && y < 0.0 {
        a += 2.0 * PI;
    }
    a
}
```
```rust
pub fn polar_angle_fast(x: f64, y: f64) -> f64 {
    y.atan2(x)
}
```
```rust
pub fn intersect_3d_lines(
    p1: Point3D,
    d1: Vector3D,
    p2: Point3D,
    d2: Vector3D,
    s_out: &mut f64,
    t_out: &mut f64,
    ip_out: &mut Point3D,
) -> bool {
    let a = Vector3D::dot(&d1, &d1);
    let b = Vector3D::dot(&d1, &d2);
    let c = Vector3D::dot(&d2, &d2);
    let denom = a * c - b * b;
    if denom.abs() <= ON_EPSILON * a {
        *s_out = 0.0;
        *t_out = 0.0;
        *ip_out = Point3D::UNSET; // ← 프로젝트에 맞게
        return false;
    }
    let delta = p2 - p1;
    let e = Vector3D::dot(&d1, &delta.to_vector());
    let f = Vector3D::dot(&d2, &delta.to_vector());
    *s_out = (e * c - f * b) / denom;
    *t_out = (e * b - f * a) / denom;
    *ip_out = p1 + d1.to_point() * *s_out;
    true
}
```
```rust
pub fn surface_tangent_vector_inversion(
    tangent_pt: Vector3D,
    su: Vector3D,
    sv: Vector3D,
    uv_tangent: &mut (f64, f64),
) {
    let a = Vector3D::dot(&su, &su);
    let b = Vector3D::dot(&su, &sv);
    let d = Vector3D::dot(&sv, &sv);
    let r1 = Vector3D::dot(&su, &tangent_pt);
    let r2 = Vector3D::dot(&sv, &tangent_pt);
    if let Some((du, dv)) = on_solve_2x2_optional(a, b, b, d, r1, r2) {
        *uv_tangent = (du, dv);
    }
}
```
```rust
pub fn on_multiply_tmatrix(a: &TMatrix<f64>, b: &TMatrix<f64>) -> TMatrix<f64> {
    assert_eq!(a.cols(), b.rows());
    let (m, k, n) = (a.rows(), a.cols(), b.cols());
    let mut out = TMatrix::with_size(m, n, 0.0);
    for i in 0..m {
        for j in 0..n {
            let mut s = 0.0;
            for kk in 0..k {
                s += a[(i, kk)] * b[(kk, j)];
            }
            out[(i, j)] = s;
        }
    }
    out
}
```
```rust
pub fn on_project_dir(
    len_threshold: f64,
    dir: Vector3D,
    plane_x: Vector3D,
    plane_y: Vector3D,
) -> bool {
    if dir.length() < len_threshold {
        return true;
    }
    if plane_x.is_zero() || plane_y.is_zero() {
        return false;
    }
    let n = Vector3D::cross(&plane_y, &plane_x);
    n.unitize();
    dir.unitize();
    n.is_unit() && (1.0 - Vector3D::dot(&n, &dir).abs()) < 1.0e-5
}
```
```rust
pub fn on_is_point_opposing_from_grid_corner(
    test: Point3D,
    grid: &TMatrix<[f64; 4]>, // ← 4D control point(동차). 필요시 여러분 타입으로.
    idx_min: &mut i32,
) -> bool {
    // 필요한 경우 4d→3d로 투영하는 헬퍼 사용
    let rc = grid.rows();
    let cc = grid.cols();
    let corners = [
        Point4D::on_from_w(&grid[(0, 0)]),
        Point4D::on_from_w(&grid[(0, cc - 1)]),
        Point4D::on_from_w(&grid[(rc - 1, 0)]),
        Point4D::on_from_w(&grid[(rc - 1, cc - 1)]),
    ];
    let mut best = std::f64::MAX;
    let mut best_i = -1;
    let mut closest = corners[0];
    for (i, p) in corners.iter().enumerate() {
        let d2 = test.distance_square(p);
        if d2 < best {
            best = d2;
            best_i = i as i32;
            closest = *p;
        }
    }
    *idx_min = best_i;

    let diff = test - closest;
    for r in 0..rc {
        for c in 0..cc {
            let q = Point4D::on_from_w(&grid[(r, c)]);
            if Vector3D::dot(&diff.to_vector(), &(q - closest).to_vector()) < 0.0 {
                return true;
            }
        }
    }
    false
}
```
```rust
/// f(t) = B'_{0,3}(t) + B'_{1,3}(t) - alpha = 0 의 빠른 해.
/// 3차에서는 B'들의 합이 6 t (t-1)이므로 해석해 가능.
/// 기본은 'lower branch'(t ∈ [0, 0.5])를 반환.
/// upper 브랜치가 필요하면 `upper_branch=true`로 호출.
pub fn solve_bezier_param_from_alpha_fast(alpha: f64, upper_branch: bool) -> f64 {
    // 이 함수의 정의역 상 자연스런 범위: alpha ∈ [-1.5, 0]
    // 살짝 벗어나도 클램프해서 안정적으로 처리
    let a = alpha.clamp(-1.5, 0.0);
    let disc = 1.0 + (2.0 / 3.0) * a; // >= 0 이 보장됨(클램프 덕분)
    let s = disc.sqrt();

    // 두 해: (1 ± s)/2
    let mut t = if upper_branch {
        0.5 * (1.0 + s) // t ∈ [0.5, 1]
    } else {
        0.5 * (1.0 - s) // t ∈ [0, 0.5]
    };

    // 수치적 튜닝: 해석해가 아주 희미하게 틀어지거나 입력이 더 벗어났을 때
    // 뉴턴 2~3스텝으로 미세 교정 (f = 6t(t-1)-alpha, f' = 12t-6)
    // 과도한 변경 방지를 위해 클램프 포함
    for _ in 0..3 {
        let f = 6.0 * t * (t - 1.0) - alpha;
        let fp = 12.0 * t - 6.0;
        if fp.abs() < 1e-14 {
            break;
        }
        let nt = (t - f / fp).clamp(0.0, 1.0);
        if (nt - t).abs() < 1e-14 {
            break;
        }
        t = nt;
    }

    t
}
```
```rust
pub fn solve_bezier_param_from_alpha(alpha: f64) -> f64 {
    let mut t = 0.5;
    let mut lo = 0.0;
    let mut hi = 1.0;
    for _ in 0..20 {
        let f = bernstein_der_3(0, t) + bernstein_der_3(1, t) - alpha;
        if f.abs() < 1e-14 {
            break;
        }
        // 도함수의 도함수 (대충 수치 미분)
        let eps = 1e-8;
        let f2 = {
            let f_p = bernstein_der_3(0, (t + eps).clamp(0.0, 1.0))
                + bernstein_der_3(1, (t + eps).clamp(0.0, 1.0))
                - alpha;
            (f_p - f) / eps
        };
        if f2.abs() < 1e-20 {
            // fallback: 이분
            let mid = 0.5 * (lo + hi);
            if f > 0.0 {
                hi = t;
            } else {
                lo = t;
            }
            t = mid;
        } else {
            let next = (t - f / f2).clamp(0.0, 1.0);
            if (next - t).abs() < 1e-14 {
                break;
            }
            t = next;
        }
    }
    t
}
```
```rust
pub fn are_points_close_to_segment(
    start_idx: usize,
    end_idx: usize,
    pts: &[Point3D],
    tol: f64,
) -> bool {
    if pts[start_idx] == pts[end_idx] && end_idx > start_idx {
        return false;
    }
    let seg = Segment3D::new(pts[start_idx], pts[end_idx]); // ← 여러분 세그먼트
    for i in start_idx + 1..end_idx {
        if pts[i].distance_to_segment(&seg) > tol {
            return false;
        }
    }
    true
}
```
```rust
pub fn compute_parameter_distribution(
    count: usize,
    seg_lengths: &[f64],
    total_len: f64,
    out: &mut Vec<f64>,
) {
    out.clear();
    out.resize(count, 0.0);
    if total_len > 0.0 {
        for i in 1..count - 1 {
            out[i] = out[i - 1] + seg_lengths[i] / total_len;
        }
        out[count - 1] = 1.0;
    } else {
        for i in 1..count - 1 {
            out[i] = i as f64 / (count - 1) as f64;
        }
        out[count - 1] = 1.0;
    }
}
```
```rust
pub fn chord_length_parameterization_lengths(
    from: usize,
    to: usize,
    q: &[Point3D],
    distances: &mut Vec<f64>,
    ub: &mut Vec<f64>,
) -> f64 {
    let len = to - from;
    distances.clear();
    distances.resize(len, 0.0);
    let mut total = 0.0;
    for i in from + 1..to {
        let d = q[i].distance(&q[i - 1]);
        distances[i - from] = d;
        total += d;
    }
    chord_length_parameterization(len, distances, total, ub);
    total
}
```
```rust
pub fn chord_length_parameterization(len: usize, distances: &[f64], chord: f64, ub: &mut Vec<f64>) {
    ub.clear();
    ub.resize(len, 0.0);
    if chord > 0.0 {
        for i in 1..len - 1 {
            ub[i] = ub[i - 1] + distances[i] / chord;
        }
        ub[len - 1] = 1.0;
    } else {
        for i in 1..len - 1 {
            ub[i] = i as f64 / (len - 1) as f64;
        }
        ub[len - 1] = 1.0;
    }
}
```
```rust
pub fn is_point_within_control_points(test: Point3D, boundary: &[[f64; 4]]) -> bool {
    // [x,y,z,w]를 3D로
    let last = boundary.len() - 1;
    let p0 = Point4D::on_from_w(&boundary[0]);
    let p1 = Point4D::on_from_w(&boundary[1]);
    let pn_1 = Point4D::on_from_w(&boundary[last - 1]);
    let pn = Point4D::on_from_w(&boundary[last]);

    let v_st = p0 - test;
    let v_es = pn - p0;

    let dot1 = Point3D::dot(&v_st, &(p1 - p0));
    let dot2 = Point3D::dot(&(pn - test), &(pn_1 - pn));
    let dot3v = Point3D::dot(&v_es, &(pn - test));
    let dot4v = Point3D::dot(&v_es, &v_st);

    if (dot1 >= 0.0 && dot2 >= 0.0)
        || (dot3v * dot4v <= 0.0)
        || dot1.abs() < SQRT_EPS
        || dot2.abs() < SQRT_EPS
    {
        return true;
    }
    false
}
```
```rust
pub fn is_opposite_direction_from_control_points(ref_pt: Point3D, ctrl: &[[f64; 4]]) -> bool {
    let last = ctrl.len() - 1;
    let a = Point4D::on_from_w(&ctrl[0]);
    let b = Point4D::on_from_w(&ctrl[last]);
    let closest = if ref_pt.distance_square(&a) > ref_pt.distance_square(&b) {
        b
    } else {
        a
    };
    let ref_v = closest - ref_pt;
    for c in ctrl {
        let v = Point4D::on_from_w(c) - closest;
        if Point3D::dot(&ref_v, &v) < -1.0e-12 {
            return true;
        }
    }
    false
}
```
```rust
pub fn is_control_points_convex_or_straight(ctrl: &[[f64; 4]]) -> bool {
    if ctrl.len() < 3 {
        return true;
    }
    let last = ctrl.len() - 1;
    let first = Point4D::on_from_w(&ctrl[0]);
    let lastp = Point4D::on_from_w(&ctrl[last]);
    for i in 1..last {
        let prev = Point4D::on_from_w(&ctrl[i - 1]);
        let next = Point4D::on_from_w(&ctrl[i + 1]);
        let seg = Segment3D::new(prev, next);
        let cur = Point4D::on_from_w(&ctrl[i]);
        let dev_v = cur.project_to_segment(&seg) - cur;

        let dp = if i < last / 2 {
            let proj_last = lastp.project_to_segment(&seg);
            Point3D::dot(&dev_v, &(proj_last - lastp))
        } else {
            let proj_first = first.project_to_segment(&seg);
            Point3D::dot(&dev_v, &(proj_first - first))
        };
        if dp > 1.0e-9 {
            return false;
        }
    }
    true
}
```
```rust
pub fn print_matrix(a: &Matrix) {
    // 디버그용
    for r in 0..a.row_count() as usize {
        for c in 0..a.col_count() as usize {
            print!("{:10.4} ", *a.at(r as i32, c as i32));
        }
        println!();
    }
}
```
```rust
pub fn print_vector(v: &[f64]) {
    for x in v {
        println!("{:.6}", x);
    }
}
```
```rust
pub fn get_normal_at_point(
    method: i32,
    du: Vector3D,
    dv: Vector3D,
    duu: Vector3D,
    duv: Vector3D,
    dvv: Vector3D,
    normal_out: &mut Vector3D,
) -> Vector3D {
    let a = du.length_squared();
    let b = Vector3D::dot(&du, &dv);
    let c = dv.length_squared();

    let (ret, _) = on_check_difference_significant(a, b, c);
    if ret  {
        *normal_out = Vector3D::cross(&du, &dv);
    } else {
        let (f1, f2) = match method {
            2 => (-1.0, 1.0),
            3 => (-1.0, -1.0),
            4 => (1.0, -1.0),
            _ => (1.0, 1.0),
        };
        let t1 = du * 0.0 + (f1 * dvv + f2 * duv);
        let c1 = du.cross(&t1);
        let t2 = (f1 * duu + f2 * dvv) + dv * 0.0;
        let c2 = t2.cross(&dv);
        *normal_out = c1 + c2;
    }
    normal_out.unitize()
}
```
```rust
pub fn compute_surface_curvature_and_normals(
    du: Vector3D,
    dv: Vector3D,
    duu: Vector3D,
    duv: Vector3D,
    dvv: Vector3D,
    n: Vector3D,
    gauss: &mut f64,
    mean: &mut f64,
    kappa1: &mut f64,
    kappa2: &mut f64,
    k1: &mut Vector3D,
    k2: &mut Vector3D,
) -> bool {
    let e = Vector3D::dot(&du, &du);
    let f = Vector3D::dot(&du, &dv);
    let g = Vector3D::dot(&dv, &dv);

    let l = Vector3D::dot(&n, &duu);
    let m = Vector3D::dot(&n, &duv);
    let nn = Vector3D::dot(&n, &dvv);

    *gauss = 0.0;
    *mean = 0.0;
    *kappa1 = 0.0;
    *kappa2 = 0.0;

    let det = e * g - f * f;
    if det == 0.0 {
        return false;
    }
    let inv = 1.0 / det;

    let k = (l * nn - m * m) * inv;
    let h2 = (g * l - 2.0 * f * m + e * nn) * inv;

    *gauss = k;
    *mean = 0.5 * h2;

    let disc = h2 * h2 - 4.0 * k;
    let (r1, r2) = if disc < 0.0 {
        if k > ON_EPSILON {
            return false;
        }
        (0.0, 0.0)
    } else if h2 == 0.0 {
        if k > 0.0 {
            return false;
        }
        let v = (-k).sqrt();
        (-v, v)
    } else {
        let srt = (1.0 - 4.0 * k / (h2 * h2)).max(0.0).sqrt();
        let big = 0.5 * h2.abs() * (1.0 + srt);
        let r2l = if h2 < 0.0 { -big } else { big };
        let r1l = k / r2l;
        (r1l, r2l)
    };

    // 순서 큰 절댓값이 kappa1
    if r1.abs() > r2.abs() {
        *kappa1 = r1;
        *kappa2 = r2;
    } else {
        *kappa1 = r2;
        *kappa2 = r1;
    }

    // 주곡률 방향
    let mut final_simple = true;
    if (r2 - r1).abs() > 1e-6 * (r2.abs() + r1.abs()) {
        final_simple = false;

        // 간단한 고유방향 구하기(방정식 (II - k I) * t = 0 의 근사)
        // 여기선 직교 기준벡터를 unitize 해서 반환
        // (실무에선 shape operator 의 고유벡터를 푸는 쪽을 권장)
        let t1 = du;
        let t2 = dv;
        let v1 = (t1 - t2).unitize();
        let v2 = n.cross(&v1).unitize();

        *k1 = v1;
        *k2 = v2;
    }
    if final_simple {
        if e >= g {
            *k1 = du.unitize();
        } else {
            *k1 = dv.unitize();
        }
        *k2 = n.cross(k1).unitize();
    }
    true
}
```
```rust
pub fn on_is_oriented_clockwise_xform(xf: &Transform, pts: &[Point3D]) -> bool {
    if pts.len() < 3 {
        return false;
    }
    let area2 = pts
        .windows(2)
        .chain(std::iter::once(&[pts[pts.len() - 1], pts[0]][..]))
        .map(|w| {
            let p = xf.apply_point(w[0]);
            let q = xf.apply_point(w[1]);
            p.x * q.y - q.x * p.y
        })
        .sum::<f64>();
    area2 < 0.0 // 음수면 시계방향
}
```
```rust
pub fn on_is_oriented_clockwise_plane(pts: &[Point3D], plane: &Plane) -> bool {
    if pts.len() < 3 {
        return false;
    }

    // 한 번만 투영해서 사용 (불필요한 중복 계산 제거)
    let n = pts.len();
    let mut area2 = 0.0;
    let mut prev = plane.project(&pts[n - 1]);
    for i in 0..n {
        let cur = plane.project(&pts[i]);
        area2 += prev.x * cur.y - cur.x * prev.y;
        prev = cur;
    }
    area2 < 0.0 // 음수면 시계방향
}
```
```rust
//----------------------------------------------
// 9) Cross Product of Two Vectors (includes parallel check)
//    On success, returns Some(normalized cross product)
//----------------------------------------------
pub fn on_compute_cross_product(a: Vector3D, b: Vector3D) -> Option<Vector3D> {
    let dot = Vector3D::dot(&a, &b);
    let parallelish = (1.0 - dot.abs()) < ON_SQRT_EPSILON;
    if !a.is_zero() && !b.is_zero() && !parallelish {
        let mut c = Vector3D::cross(&a, &b);
        if c.normalize() {
            return Some(c);
        }
    }
    None
}
```
```rust
//----------------------------------------------
// 10) Angle Utilities
//----------------------------------------------
#[inline]
pub fn on_normalize_angle_to_two_pi(mut angle: f64) -> f64 {
    if (0.0..=2.0 * PI).contains(&angle) {
        return angle;
    }
    angle = angle - (angle / (2.0 * PI)).floor() * (2.0 * PI);
    if angle < 0.0 {
        0.0
    } else if angle > 2.0 * PI {
        2.0 * PI
    } else {
        angle
    }
}
```
```rust
#[inline]
pub fn on_safe_atan2_positive_angle(x: f64, y: f64) -> f64 {
    if x.abs() + y.abs() < 1e-20 {
        return 0.0;
    }
    let mut a = y.atan2(x);
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}
```
```rust
#[inline]
pub fn on_atan2_adjusted(x: f64, y: f64) -> f64 {
    let mut a = y.atan2(x);
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}
```
```rust
#[inline]
pub fn on_normalize_angle(mut a: f64) -> f64 {
    while a >= 2.0 * PI {
        a -= 2.0 * PI;
    }
    while a < 0.0 {
        a += 2.0 * PI;
    }
    a
}
```
```rust
#[inline]
pub fn on_safe_acos(x: f64) -> f64 {
    if x > 1.0 {
        0.0
    } else if x < -1.0 {
        PI
    } else {
        x.acos()
    }
}
```
```rust
#[inline]
pub fn on_vector2d_magnitude(x: f64, y: f64) -> f64 {
    let ax = x.abs();
    let ay = y.abs();
    if ax > ay {
        ax * (1.0 + (ay / ax).powi(2)).sqrt()
    } else {
        if ay != 0.0 {
            ay * (1.0 + (ax / ay).powi(2)).sqrt()
        } else {
            0.0
        }
    }
}
```
```rust
fn adaptive_trapezoidal_integration_test() {
    // 적분할 함수: f(x) = sin(x)
    let f = |x: f64| x.sin();

    // 구간 [0, π]에서 적분
    let a = 0.0;
    let b = std::f64::consts::PI;

    // 상대 오차 허용치와 최대 반복 단계
    let rel_tol = 1e-8;
    let max_level = 20;

    let (result, evals, error) = on_adaptive_trapezoidal_integration(f, a, b, rel_tol, max_level);

    println!("적분 결과: {}", result);
    println!("함수 평가 횟수: {}", evals);
    println!("최종 오차 추정: {}", error);
}
```
### 출력 결과
```
적분 결과: 2.0000000002520055
함수 평가 횟수: 257
최종 오차 추정: 0.000000003780252111340587
```
---

## 📌 1단계: 수치 해석 관련 함수 정리
이 범주는 행렬 분해, 수치 적분, 다항식 근사 등 수학적 계산에 관련된 함수들입니다.
### 1. on_jacobi_symmetric_eigen
- 기능: 대칭 행렬 $B\in \mathbb{R^{\mathnormal{n\times n}}}$ 을 야코비 회전법으로 고유분해
- 수식:
- 목표: $B=V\Lambda V^T$
- $\Lambda$ : 고유값 대각행렬
- $V$: 정규 직교 고유벡터 행렬 (열 기준)
- 사용법:

```rust
let mut b = Matrix::from(...); // 대칭 행렬
let mut vals = vec![0.0; n];
let mut v = Matrix::new();
let success = on_jacobi_symmetric_eigen(&mut b, &mut vals, &mut v);
```

### 2. on_solve_tridiagonal_inplace
- 기능: 삼대각 행렬 시스템 $Ax=d$ 을 Thomas 알고리즘으로 풀이
- 수식:
- A는 삼대각 행렬: a_i,b_i,c_i는 각각 하/중/상 대각
- 전진 제거 + 후진 대입
사용법:
```rust
let mut a = vec![0.0, a1, a2, ...]; // a[0]은 사용 안 함
let mut b = vec![b0, b1, ...];
let mut c = vec![c0, c1, ...];
let mut d = vec![d0, d1, ...]; // RHS → 해로 덮어씀
let success = on_solve_tridiagonal_inplace(&mut a, &mut b, &mut c, &mut d);
```

### 3. on_adaptive_simpson
- 기능: 적응형 심프슨 수치 적분
- 수식:

$$
\int _a^bf(x)dx\approx \frac{b-a}{6}\left[ f(a)+4f\left( \frac{a+b}{2}\right) +f(b)\right]
$$

- 오차가 허용 오차보다 작아질 때까지 재귀적으로 분할
- 사용법:
```rust
let result = on_adaptive_simpson(&|x| x.sin(), 0.0, std::f64::consts::PI, 1e-8, 10);
```


### 4. on_poly_fit
- 기능: 다항식 근사 (최소제곱법)
- 수식:
- $y=a_0+a_1x+a_2x^2+\dots +a_nx^n$
- 정규방정식: $(T^TT)\vec {c}=T^T\vec {y}$
-사용법:
```rust
let coeffs = on_poly_fit(&t_values, &y_values, degree);
```


### 5. on_poly_val
- 기능: 다항식 평가 (Horner 방식)
- 수식:

$$
P(x)=a_0+a_1x+a_2x^2+\dots +a_nx^n
$$

- 사용법:
```rust
let ys = on_poly_val(&coeffs, &x_values);
```


### 6. on_cholesky_solve_spd
- 기능: 대칭 양의 정부호 행렬에 대한 Cholesky 분해 기반 선형 시스템 풀이
- 수식:
- $A=LL^T$
- $LL^Tx=b$ → 전진/후진 대입
- 사용법:
```rust
let mut g = vec![...]; // 행렬 A (row-major)
let mut b = vec![...]; // RHS → 해로 덮어씀
let success = on_cholesky_solve_spd(&mut g, &mut b, n);
```


### 7. on_nalgebra_cholesky_solve_spd
- 기능: nalgebra 라이브러리를 이용한 Cholesky 분해 기반 선형 시스템 풀이
- 사용법:
```rust
let mut g = vec![...];
let mut b = vec![...];
let success = on_nalgebra_cholesky_solve_spd(&mut g, &mut b, n);
```


## 📘 2단계: 기하학 함수 정리
이 범주는 원, 타원, 거리, 투영, 평면 관련 연산 등 공간 기하학에 관련된 함수들입니다.

### 1. on_point_on_circle
- 기능: 중심 $c$, 반지름 $r$, 각도 $\theta$ 에 따라 원 위의 점 계산
- 수식:

$$
P=c+r\cdot (\cos (\theta ),\sin (\theta ),0)
$$

- 사용법:

```rust
let pt = on_point_on_circle(center, radius, angle);
```


### 2. on_circle_to_polygon
- 기능: 원을 다각형으로 근사
- 수식:
- $\theta _i=\frac{2\pi i}{n}$
- $P_i=c+r\cdot (\cos (\theta _i),\sin (\theta _i))$
- 사용법:

```rust
let poly = on_circle_to_polygon(center, radius, segments);
```


### 3. on_ellipse_to_polygon
- 기능: 타원을 다각형으로 근사
- 수식:
- $P_i=c+(r_x\cos (\theta _i),r_y\sin (\theta _i))$
- 사용법:

```rust
let poly = on_ellipse_to_polygon(center, rx, ry, segments);
```


### 4. on_distance
- 기능: 두 3D 점 사이의 거리 계산
- 수식:

$$
\mathrm{dist}(a,b)=\sqrt{(b_x-a_x)^2+(b_y-a_y)^2+(b_z-a_z)^2}
$$

- 사용법:
```rust
let d = on_distance(&a, &b);
```

### 5. on_lerp_point
- 기능: 두 점 사이의 선형 보간
- 수식:

$$
P(t)=(1-t)\cdot a+t\cdot b
$$

- 사용법:

```rust
let pt = on_lerp_point(&a, &b, t);
```


### 6. on_unitize
- 기능: 벡터 정규화
- 수식:

$$
\hat {v}=\frac{v}{\| v\| },\quad \mathrm{단\  }\| v\| >\mathrm{tol}
$$

- 사용법:
```rust
let unit = on_unitize(v);
```

### 7. on_dot_vec, on_dot_pt
- 기능: 내적 계산
- 수식:

$$
a\cdot b=a_xb_x+a_yb_y+a_zb_z
$$

- 사용법:
```rust
let dot = on_dot_vec(&a, &b);
```

### 8. on_project_point_onto_line
- 기능: 3D 점을 선분에 정사영
- 수식:

$$
P'=O+\frac{(P-O)\cdot D}{\| D\| ^2}\cdot D
$$

- 사용법:

```rust
on_project_point_onto_line(&origin, &target, &mut point);
```


### 9. on_circle_from_3_points
- 기능: 2D 세 점으로 원의 중심과 반지름 계산
- 수식:
    - 세 점을 지나는 원의 중심은 선분 수직이등분선의 교점
    - 반지름은 중심과 한 점 사이 거리
- 사용법:
```rust
let result = on_circle_from_3_points(&a, &b, &c);
```


### 10. on_arc_half_angle
- 기능: 원호의 중심과 양 끝점으로 반각 계산
- 수식:

$$
\theta =\cos ^{-1}\left( \frac{u\cdot v}{\| u\| \| v\| }\right) 
$$

- 사용법:
```rust
let angle = on_arc_half_angle(&center, &a, &b);
```


## 📘 3단계: 다각형 및 포함 판정 관련 함수
이 범주는 2D 다각형의 방향, 포함 여부, 교차 판정 등 공간 영역 처리에 관련된 함수들입니다.

### 1. on_is_clockwise
- 기능: 2D 다각형의 winding 방향이 시계방향인지 판정
- 수식:

$$
\mathrm{sum}=\sum _i(x_{i+1}-x_i)(y_{i+1}+y_i)
$$

- sum > 0 → 시계방향

- 사용법:
```rust
let is_cw = on_is_clockwise(&points);
```


### 2. on_ensure_winding_order
- 기능: 다각형의 winding 방향을 지정한 방향으로 맞춤
- 사용법:
```rust
on_ensure_winding_order(&mut points, true); // true → 시계방향
```

### 3. point_in_polygon_simple
- 기능: 단일 루프 다각형에 점이 포함되는지 판정 (짝수 교차법)
- 수식:
    - 수평선과 교차 횟수로 내부 여부 결정
- 사용법:
```rust
let inside = point_in_polygon_simple(&pt, &polygon.points);
```

### 4. point_in_polygon_composite
- 기능: 외곽 + 홀 구조의 복합 다각형 포함 여부 판정
- 로직:
    - 첫 루프는 외곽, 이후 루프는 홀
    - 외곽에 포함되고 홀에 포함되지 않으면 true
- 사용법:
```rust
let inside = point_in_polygon_composite(&pt, &loops);
```

### 5. classify_patch_polygon
- 기능: 패치 다각형과 트림 다각형들 간의 포함/교차 관계 판정
- 반환값: PolygonStatus::In, On, Out, Over
- 사용법:
```rust
let status = classify_patch_polygon(&patch, &trim_polygons);
```


### 6. check_diagonal_intersections
- 기능: 3D 네 점으로 구성된 두 대각선 쌍이 교차하는지 판정
- 로직:
    - 평면 투영 후 2D 선분 교차 판정
- 사용법:
```rust
let hit = check_diagonal_intersections(a, b, c, d);
```


## 📘 벡터 연산 및 수치 유틸리티 함수
이 범주는 벡터 연산, 비교, 정규화, 보간, 클램핑 등 수치 계산을 보조하는 함수들입니다.

## 🔢 벡터 연산 함수
### 1. vec_dot(a, b)
- 기능: 3D 벡터 내적
- 수식:

$$
a\cdot b=a_0b_0+a_1b_1+a_2b_2
$$

### 2. vec_sub(a, b) / vec_add(a, b) / vec_mul_s(a, s)
- 기능: 벡터 뺄셈, 덧셈, 스칼라 곱
- 수식:
- $a-b$, $a+b$, $s\cdot a$

### 3. vec_len(v) / vec_len2(v)
- 기능: 벡터 길이 및 제곱 길이
- 수식:

$$
\| v\| =\sqrt{v_0^2+v_1^2+v_2^2}
$$

### 4. vec_cross(a, b)
- 기능: 3D 벡터 외적
- 수식:

$$
a\times b=\left[ \begin{matrix}a_1b_2-a_2b_1\\ a_2b_0-a_0b_2\\ a_0b_1-a_1b_0\end{matrix}\right] 
$$


### 5. vec_normalize(v)
- 기능: 벡터 정규화
- 수식:

$$
\hat {v}=\frac{v}{\| v\| },\quad \mathrm{단\  }\| v\| >0
$$


### 6. are_coplanar(a, b, c, d, tol)
- 기능: 네 점이 같은 평면에 있는지 판정
- 수식:
- $\mathrm{dist}=\frac{|n\cdot (d-a)|}{\| n\| }$
- $n=(b-a)\times (c-a)$

### 7. on_project_uv(a, e1, e2, p)
- 기능: 3D 점을 평면 기저 e_1,e_2로 투영
- 수식:

$$
u=(p-a)\cdot e_1,\quad v=(p-a)\cdot e_2
$$

## 📏 수치 비교 및 보조 함수

### 8. on_are_equal(a, b, tol) / on_are_equal_rel_tol(a, b)
- 기능: 절대/상대 오차 기반 실수 비교
- 수식:

$$
|a-b|<\mathrm{tol},\quad \mathrm{또는\  }\frac{|a-b|}{\max (|a|,|b|)}<\mathrm{tol}
$$


### 9. on_compare(a, b, tol)
- 기능: 오차 허용 기반 비교 결과 (-1, 0, 1)

### 10. on_clamp(x, lo, hi) / on_clamp01(t) / on_clamp11(x)
- 기능: 값의 범위 제한
- 수식:
- $x\in [lo,hi]$, $t\in [0,1]$, $x\in [-1,1]$

### 11. on_rel_err(a, b)
- 기능: 상대 오차 계산
- 수식:

$$
\mathrm{err}=\frac{|a-b|}{\max (|a|,|b|,\epsilon )}
$$


### 12. on_is_zero(a, tol) / on_is_finite(v) / on_is_infinite(v)
- 기능: 값이 0인지, 유한/무한인지 판정

### 13. on_are_vector_equal(a, b, tol) / on_are_vec_equal(u, v, eps) 등
- 기능: 벡터/점의 성분별 비교


## 📘 데이터 샘플링, 보간, 기타 보조 알고리즘
이 범주는 곡선 근사, 샘플링, 수치 안정성 보장 등을 위한 보조 알고리즘 함수들로 구성되어 있습니다.

## 📊 곡선 근사 및 분할
### 1. on_get_divide_number
- 기능: 원호를 주어진 허용 편차(deviation) 내에서 분할할 세그먼트 수 계산
- 수식:

$$
\theta =2\cdot \arccos \left( \frac{r-\delta }{r}\right) 
$$

- $\delta$ : 허용 편차
- $\theta$ : 한 세그먼트가 커버할 수 있는 최대 각도
- 사용법:
```rust
let (segments, angle_step) = on_get_divide_number(radius, delta_radians, deviation);
```

### 2. on_number_of_segments
- 기능: 전체 원호를 주어진 편차와 최대 각도 제한에 따라 적절한 세그먼트 수로 분할
- 로직:
    - $\delta$ , $\theta$ $_{\mathrm{max}}$ 기준으로 분할 수 결정
    - $\theta =\frac{\Delta \theta }{n}\leq \theta _{\mathrm{max}}$
- 사용법:
```rust
let n = on_number_of_segments(radius, delta_radians, deviation, angle_limit);
```


## 🧪 샘플링 함수
### 3. on_get_sampling_2d / on_get_sampling_3d
- 기능: 2D 또는 3D 점 집합에서 균등하게 샘플링
- 로직:
    - $\mathrm{샘플\  수}=4+\sqrt[3]{N}$
    - 일정 간격으로 점 선택
- 사용법:
```rust
let sampled = on_get_sampling_2d(&points);
```

## 📐 기하 보조 함수
### 4. measure_twist
- 기능: 3D 사변형의 비틀림 정도 측정
- 수식:
    - 사변형 평면에서 네 번째 점의 거리 측정

$$
\mathrm{twist}=\frac{|n\cdot (p_3-p_0)|}{\| n\| },\quad n=(p_2-p_0)\times (p_1-p_0)
$$

- 사용법:
```rust
let twist = measure_twist(p0, p1, p2, p3);
```


## 🧪 1단계: 수치 해석 함수 샘플 코드
### ✅ 1. on_jacobi_symmetric_eigen
```rust
let mut b = Matrix::from_vec(3, 3, vec![
    4.0, 1.0, 1.0,
    1.0, 3.0, 0.0,
    1.0, 0.0, 2.0,
]);
let mut vals = vec![];
let mut v = Matrix::new();
let success = on_jacobi_symmetric_eigen(&mut b, &mut vals, &mut v);
if success {
    println!("Eigenvalues: {:?}", vals);
    println!("Eigenvectors: {:?}", v);
}
```
### ✅ 입력 행렬 확인
```rust
A = [
    [4.0, 1.0, 1.0],
    [1.0, 3.0, 0.0],
    [1.0, 0.0, 2.0]
]
```

이 행렬은 대칭 행렬이며, 야코비 방법의 적용 대상입니다.

### ✅ 출력 결과 검증
#### 🔹 고유값
Eigenvalues ≈ [4.879, 2.653, 1.468]

이 값들은 수치적으로 다음을 만족해야 합니다:
- $A=V\Lambda V^T$
- $\Lambda$ : 대각 고유값 행렬
- $V$: 직교 고유벡터 행렬
### 🔹 고유벡터 행렬 V
```rust
V = [
    [ 0.8440, -0.2931, -0.4491],
    [ 0.4491,  0.8440,  0.2931],
    [ 0.2931, -0.4491,  0.8440]
]
```

이 행렬은 다음을 만족해야 합니다:
- $V^TV=I$ → 직교성
- 각 열은 단위 벡터 → $\| v_i\| =1$
#### 🔹 검증: $A\cdot v_i=\lambda _i\cdot v_i$
예를 들어 첫 번째 고유값과 고유벡터:
- $\lambda _1=4.879$
- $v_1=[0.8440,0.4491,0.2931]$
- 계산:

$$
A\cdot v_1\approx \left[ \begin{matrix}4.0&1.0&1.0\\ 1.0&3.0&0.0\\ 1.0&0.0&2.0\end{matrix}\right] \cdot \left[ \begin{matrix}0.8440\\ 0.4491\\ 0.2931\end{matrix}\right] \approx 4.879\cdot v_1
$$

이 결과는 수치적으로 만족합니다.

## ✅ 결론
- 함수 on_jacobi_symmetric_eigen은 대칭 행렬에 대해 정확한 고유값과 직교 고유벡터를 반환하고 있습니다.
- 출력된 고유값과 고유벡터는 수학적으로 정당하며, 직교성과 고유방정식을 만족합니다.


### ✅ 2. on_solve_tridiagonal_inplace
```rust
let mut a = vec![0.0, 1.0, 1.0]; // subdiagonal (a[0] unused)
let mut b = vec![4.0, 4.0, 4.0]; // diagonal
let mut c = vec![1.0, 1.0, 0.0]; // superdiagonal (c[n-1] unused)
let mut d = vec![5.0, 5.0, 5.0]; // RHS
let success = on_solve_tridiagonal_inplace(&mut a, &mut b, &mut c, &mut d);
if success {
    println!("Solution: {:?}", d);
}
```

on_solve_tridiagonal_inplace 함수의 입력과 출력 결과를 수학적으로 검증해드릴게요.

### ✅ 문제 정의
이 함수는 삼대각 행렬 시스템 Ax=d 을 푸는 함수입니다. 주어진 벡터들은 다음과 같은 행렬을 구성합니다:
#### 🔹 행렬 A 구성
- 주대각선 b=[4.0,4.0,4.0]
- 하부대각선 a=[0.0,1.0,1.0] → a[0]은 사용되지 않음
- 상부대각선 c=[1.0,1.0,0.0] → c[2]은 사용되지 않음
따라서 A는 다음과 같습니다:

$$
A=\left[ \begin{matrix}4.0&1.0&0.0\\ 1.0&4.0&1.0\\ 0.0&1.0&4.0\end{matrix}\right] 
$$

#### 🔹 우변 벡터 d

$$
d=\left[ \begin{matrix}5.0\\ 5.0\\ 5.0\end{matrix}\right] 
$$


#### ✅ 출력 결과
```
Solution: [1.0714285714285714, 0.7142857142857143, 1.0714285714285714]
```

이 벡터를 $x$ 라고 하면, $A\cdot x$ 를 계산해보겠습니다.

####  ✅ 검산: A\cdot x

$$
x=\left[ \begin{matrix}1.0714285714285714\\ 0.7142857142857143\\ 1.0714285714285714\end{matrix}\right] 
$$

- 계산:
- 첫 번째 행:

$$
4.0\cdot 1.0714+1.0\cdot 0.7143=4.2857+0.7143=5.0
$$

- 두 번째 행:

$$
1.0\cdot 1.0714+4.0\cdot 0.7143+1.0\cdot 1.0714=1.0714+2.8572+1.0714=5.0
$$

- 세 번째 행:

$$
1.0\cdot 0.7143+4.0\cdot 1.0714=0.7143+4.2857=5.0
$$

- 모든 결과가 정확히 5.0입니다 ✅

### 🧠 결론
- 함수 on_solve_tridiagonal_inplace는 주어진 삼대각 행렬 시스템을 정확하게 풀고 있습니다.
- 출력된 해는 수치적으로 완벽하게 검증됩니다.
- Thomas 알고리즘 기반의 전진/후진 대입이 올바르게 작동하고 있습니다.




### ✅ 3. on_adaptive_simpson
```rust
let result = on_adaptive_simpson(&|x| x.sin(), 0.0, std::f64::consts::PI, 1e-8, 10);
println!("∫ sin(x) dx from 0 to π ≈ {}", result);
```


### ✅ 4. on_poly_fit
```rust
let t = vec![0.0, 1.0, 2.0, 3.0];
let v = vec![1.0, 2.0, 3.9, 6.1];
let coeffs = on_poly_fit(&t, &v, 2); // 2차 다항식
println!("Fitted coefficients: {:?}", coeffs);
```


#### ✅ 검산: 각 점에서의 근사값과 실제값 비교
|  x  | 실제 y | 근사 y = 0.97 + 0.82x + 0.30x² | 계산 과정                          | 오차  |
|-----|--------|-------------------------------|------------------------------------|--------|
| 0.0 |  1.0   | 0.97                          | 0.97 + 0.00 + 0.00 = 0.97          | 0.03   |
| 1.0 |  2.0   | 2.09                          | 0.97 + 0.82 + 0.30 = 2.09          | 0.09   |
| 2.0 |  3.9   | 3.81                          | 0.97 + 1.64 + 1.20 = 3.81          | 0.09   |
| 3.0 |  6.1   | 6.13                          | 0.97 + 2.46 + 2.70 = 6.13          | 0.03   |

모든 오차가 ±0.1 이내이며, 매우 양호한 근사입니다 ✅

#### ✅ 수학적 정당성
- on_poly_fit 함수는 최소제곱법 기반으로 다항식 계수를 계산합니다.
- Vandermonde 행렬 T를 구성하고, $T^TTc=T^Ty$ 를 풀어 계수 c를 구합니다.
- 결과는 입력 데이터에 대해 평균 제곱 오차를 최소화하는 최적의 2차 다항식입니다.


---

## 🧪 테스트 코드 예시
```rust
#[test]
fn test_poly_fit_accuracy() {
    let t = vec![0.0, 1.0, 2.0, 3.0];
    let v = vec![1.0, 2.0, 3.9, 6.1];
    let coeffs = on_poly_fit(&t, &v, 2);
    let approx = on_poly_val(&coeffs, &t);
    for (y, y_hat) in v.iter().zip(approx.iter()) {
        assert!((y - y_hat).abs() < 0.15);
    }
}
```

## ✅ 5. on_poly_val
```rust
let coeffs = vec![1.0, 2.0, 3.0]; // y = 1 + 2x + 3x²
let xs = vec![0.0, 1.0, 2.0];
let ys = on_poly_val(&coeffs, &xs);
println!("Evaluated values: {:?}", ys);
```


## ✅ 6. on_cholesky_solve_spd
```rust
let mut g = vec![
    4.0, 2.0,
    2.0, 3.0,
]; // SPD matrix (2x2)
let mut b = vec![6.0, 5.0];
let success = on_cholesky_solve_spd(&mut g, &mut b, 2);
if success {
    println!("Solution: {:?}", b);
}
```
```
Solution: [1.0, 0.9999999999999999]
```


## ✅ 7. on_nalgebra_cholesky_solve_spd
```rust
let mut g = vec![
    4.0, 2.0,
    2.0, 3.0,
];
let mut b = vec![6.0, 5.0];
let success = on_nalgebra_cholesky_solve_spd(&mut g, &mut b, 2);
if success {
    println!("Solution via nalgebra: {:?}", b);
}
```
```
Solution via nalgebra: [1.0, 0.9999999999999999]
```

## 📐 2단계: 기하학 함수 샘플 코드
### ✅ 1. on_point_on_circle
```rust
let center = Point3D::new(0.0, 0.0, 0.0);
let radius = 5.0;
let angle = std::f64::consts::FRAC_PI_2; // 90도
let pt = on_point_on_circle(center, radius, angle);
println!("Point on circle: {:?}", pt);
```
```
Point on circle: Point3D { x: 3.061616997868383e-16, y: 5.0, z: 0.0 }
```


### ✅ 2. on_circle_to_polygon
```rust
let center = Point2D::new(0.0, 0.0);
let radius = 3.0;
let segments = 8;
let polygon = on_circle_to_polygon(center, radius, segments);
println!("Circle as polygon: {:?}", polygon.points);
```
```
Circle as polygon: [Point2D { x: 3.0, y: 0.0 }, Point2D { x: 2.121320343559643, y: 2.121320343559643 }, Point2D { x: 1.8369701987210297e-16, y: 3.0 }, Point2D { x: -2.1213203435596424, y: 2.121320343559643 }, Point2D { x: -3.0, y: 3.6739403974420594e-16 }, Point2D { x: -2.121320343559643, y: -2.1213203435596424 }, Point2D { x: -5.51091059616309e-16, y: -3.0 }, Point2D { x: 2.121320343559642, y: -2.121320343559643 }, Point2D { x: 3.0, y: -7.347880794884119e-16 }, Point2D { x: 3.0, y: 0.0 }]
```

### ✅ 3. on_ellipse_to_polygon
```rust
let center = Point2D::new(0.0, 0.0);
let rx = 4.0;
let ry = 2.0;
let segments = 12;
let polygon = on_ellipse_to_polygon(center, rx, ry, segments);
println!("Ellipse as polygon: {:?}", polygon.points);
```
```
Ellipse as polygon: [Point2D { x: 4.0, y: 0.0 }, Point2D { x: 3.464101615137755, y: 0.9999999999999999 }, Point2D { x: 2.0000000000000004, y: 1.7320508075688772 }, Point2D { x: 2.4492935982947064e-16, y: 2.0 }, Point2D { x: -1.9999999999999991, y: 1.7320508075688774 }, Point2D { x: -3.464101615137755, y: 0.9999999999999999 }, Point2D { x: -4.0, y: 2.4492935982947064e-16 }, Point2D { x: -3.4641016151377553, y: -0.9999999999999994 }, Point2D { x: -2.0000000000000018, y: -1.732050807568877 }, Point2D { x: -7.347880794884119e-16, y: -2.0 }, Point2D { x: 2.0000000000000004, y: -1.7320508075688772 }, Point2D { x: 3.4641016151377535, y: -1.0000000000000009 }, Point2D { x: 4.0, y: -4.898587196589413e-16 }, Point2D { x: 4.0, y: 0.0 }]
```


### ✅ 4. on_distance
```rust
let a = Point3D::new(1.0, 2.0, 3.0);
let b = Point3D::new(4.0, 6.0, 3.0);
let dist = on_distance(&a, &b);
println!("Distance between points: {}", dist);
```
```
Distance between points: 5
```

### ✅ 5. on_lerp_point
```rust
let a = Point3D::new(0.0, 0.0, 0.0);
let b = Point3D::new(10.0, 10.0, 0.0);
let pt = on_lerp_point(&a, &b, 0.25);
println!("Interpolated point: {:?}", pt);
```
```
Interpolated point: Point3D { x: 2.5, y: 2.5, z: 0.0 }
```


### ✅ 6. on_unitize
```rust
let v = Vector3D::new(3.0, 4.0, 0.0);
let unit = on_unitize(v);
println!("Unit vector: {:?}", unit);
```
```
Unit vector: Vector3D { x: 0.6, y: 0.8, z: 0.0 }
```



### ✅ 7. on_dot_vec
```rust
let a = Vector3D::new(1.0, 2.0, 3.0);
let b = Vector3D::new(4.0, -5.0, 6.0);
let dot = on_dot_vec(&a, &b);
println!("Dot product: {}", dot);
```
```
Dot product: 12
```


## ✅ 8. project_point_onto_line
```rust
let origin = Point3D::new(0.0, 0.0, 0.0);
let target = Point3D::new(1.0, 0.0, 0.0);
let mut pt = Point3D::new(0.5, 1.0, 0.0);
project_point_onto_line(&origin, &target, &mut pt);
println!("Projected point: {:?}", pt);
```
```
Projected point: Point3D { x: 0.5, y: 0.0, z: 0.0 }
```


## ✅ 9. on_circle_from_3_points
```rust
let a = Point2D::new(0.0, 0.0);
let b = Point2D::new(1.0, 0.0);
let c = Point2D::new(0.0, 1.0);
if let Some((center, radius)) = on_circle_from_3_points(&a, &b, &c) {
    println!("Circle center: {:?}, radius: {}", center, radius);
}
```
```
Circle center: Point2D { x: 0.5, y: 0.5 }, radius: 0.7071067811865476
```



## ✅ 10. on_arc_half_angle
```rust
let o = Point2D::new(0.0, 0.0);
let a = Point2D::new(1.0, 0.0);
let b = Point2D::new(0.0, 1.0);
let angle = on_arc_half_angle(&o, &a, &b);
println!("Half angle of arc: {}", angle);
```
```
Half angle of arc: 0.7853981633974483
```



이 함수들은 2D 다각형의 방향, 포함 여부, 교차 여부 등을 판단하는 데 사용됩니다.

## 🧭 3단계: 다각형 및 포함 판정 함수 샘플 코드
### ✅ 1. on_is_clockwise
```rust
let points = vec![
    Point2D::new(0.0, 0.0),
    Point2D::new(1.0, 0.0),
    Point2D::new(1.0, 1.0),
    Point2D::new(0.0, 1.0),
];
let is_cw = on_is_clockwise(&points);
println!("Is clockwise? {}", is_cw);
```
```
Is clockwise? false
```

### ✅ 2. on_ensure_winding_order
```rust
let mut points = vec![
    Point2D::new(0.0, 0.0),
    Point2D::new(1.0, 0.0),
    Point2D::new(1.0, 1.0),
    Point2D::new(0.0, 1.0),
];
on_ensure_winding_order(&mut points, true); // 시계 방향으로 맞춤
println!("Adjusted winding: {:?}", points);
```
```
Adjusted winding: [Point2D { x: 0.0, y: 1.0 }, Point2D { x: 1.0, y: 1.0 }, Point2D { x: 1.0, y: 0.0 }, 
Point2D { x: 0.0, y: 0.0 }]

```


### ✅ 3. point_in_polygon_simple
```rust
let polygon = vec![
    Point2D::new(0.0, 0.0),
    Point2D::new(2.0, 0.0),
    Point2D::new(2.0, 2.0),
    Point2D::new(0.0, 2.0),
    Point2D::new(0.0, 0.0),
];
let pt = Point2D::new(1.0, 1.0);
let inside = point_in_polygon_simple(&pt, &polygon);
println!("Point inside polygon? {}", inside);
```
```
Point inside polygon? true
```


### ✅ 4. point_in_polygon_composite
```rust
let outer = Polygon2D::from_points(vec![
    Point2D::new(0.0, 0.0),
    Point2D::new(4.0, 0.0),
    Point2D::new(4.0, 4.0),
    Point2D::new(0.0, 4.0),
    Point2D::new(0.0, 0.0),
]);
let hole = Polygon2D::from_points(vec![
    Point2D::new(1.0, 1.0),
    Point2D::new(3.0, 1.0),
    Point2D::new(3.0, 3.0),
    Point2D::new(1.0, 3.0),
    Point2D::new(1.0, 1.0),
]);
let pt = Point2D::new(2.0, 2.0);
let inside = point_in_polygon_composite(&pt, &[outer, hole]);
println!("Point inside composite polygon? {}", inside);
```
```
Point inside composite polygon? false
```


### ✅ 5. classify_patch_polygon
```rust
let patch = Polygon2D::from_points(vec![
    Point2D::new(1.5, 1.5),
    Point2D::new(2.5, 1.5),
    Point2D::new(2.5, 2.5),
    Point2D::new(1.5, 2.5),
    Point2D::new(1.5, 1.5),
]);
let outer = Polygon2D::from_points(vec![
    Point2D::new(0.0, 0.0),
    Point2D::new(4.0, 0.0),
    Point2D::new(4.0, 4.0),
    Point2D::new(0.0, 4.0),
    Point2D::new(0.0, 0.0),
]);
let hole = Polygon2D::from_points(vec![
    Point2D::new(1.0, 1.0),
    Point2D::new(3.0, 1.0),
    Point2D::new(3.0, 3.0),
    Point2D::new(1.0, 3.0),
    Point2D::new(1.0, 1.0),
]);
let status = classify_patch_polygon(&patch, &[outer, hole]);
println!("Patch classification: {:?}", status);
```
```
Patch classification: On
```


### ✅ 6. check_diagonal_intersections
```rust
let a = [0.0, 0.0, 0.0];
let b = [1.0, 0.0, 0.0];
let c = [1.0, 1.0, 0.0];
let d = [0.0, 1.0, 0.0];
let hit = check_diagonal_intersections(a, b, c, d);
println!("Diagonals intersect? {}", hit);
```
```
Diagonals intersect? true
```
```rust
#[test]
fn check_diagonal_intersections_test()
{
    let a = [0.0, 0.0, 0.0];
    let b = [1.0, 0.0, 0.0];
    //let c = [1.0, 1.0, 0.0];
    let c = [0.5, 0.5, 1.0]; // 위로 튀어나온 점
    let d = [0.0, 1.0, 0.0];
    let hit = on_check_diagonal_intersections(a, b, c, d);
    println!("Diagonals intersect? {}", hit);
}
```
```
Diagonals intersect? false
```

### ✅ 입력 점 정의
```rust
a = (0.0, 0.0, 0.0)  // 사각형의 왼쪽 아래
b = (1.0, 0.0, 0.0)  // 오른쪽 아래
c = (1.0, 1.0, 0.0)  // 오른쪽 위
d = (0.0, 1.0, 0.0)  // 왼쪽 위
```

- 이 네 점은 평면 z=0 위에 있는 정사각형을 구성합니다.

### ✅ 대각선 정의
- 대각선 1: a\rightarrow c
- 대각선 2: b\rightarrow d
즉, 두 대각선은 다음과 같습니다:
- $\vec {ac}=c-a=(1.0,1.0,0.0)$
- $\vec {bd}=d-b=(-1.0,1.0,0.0)$

### ✅ 교차 여부 검증
두 선분이 교차하려면 다음 조건을 만족해야 합니다:
- 같은 평면에 존재 → 이미 z=0 평면에 있음
- 선분 교차 판정: 2D에서 선분 ac와 bd가 교차하는지 확인
#### 🔹 방법: CCW (Counter Clockwise) 판정
두 선분 ac와 bd가 교차하려면 다음이 성립해야 합니다:

$$
\mathrm{ccw}(a,c,b)\neq \mathrm{ccw}(a,c,d)\quad \mathrm{AND}\quad \mathrm{ccw}(b,d,a)\neq \mathrm{ccw}(b,d,c)
$$

계산해보면:
- $\mathrm{ccw}(a,c,b)=\mathrm{ccw}((0,0),(1,1),(1,0))=\mathrm{true}$
- $\mathrm{ccw}(a,c,d)=\mathrm{ccw}((0,0),(1,1),(0,1))=\mathrm{false}$
- $\mathrm{ccw}(b,d,a)=\mathrm{ccw}((1,0),(0,1),(0,0))=\mathrm{true}$
- $\mathrm{ccw}(b,d,c)=\mathrm{ccw}((1,0),(0,1),(1,1))=\mathrm{false}$
→ 교차 조건 만족 ✅

## ✅ 결론
- 두 대각선 ac와 bd는 정사각형의 중심에서 정확히 교차합니다.
- `check_diagonal_intersections(a, b, c, d)` 가 true를 반환하는 것은 수학적으로 정확합니다.
- 교차점은 (0.5,0.5,0.0)입니다.


벡터 연산 및 수치 유틸리티 함수 샘플 코드를 제공.  
이 함수들은 벡터 계산, 비교, 클램핑, 정규화 등 수치적 안정성과 정확성을 위한 기초 도구들입니다.

## 🧮 4단계: 벡터 및 수치 유틸리티 함수 샘플 코드
### ✅ 1. on_vec_dot
```rust
let a = [1.0, 2.0, 3.0];
let b = [4.0, 5.0, 6.0];
let dot = on_vec_dot(a, b);
println!("Dot product: {}", dot);
```


### ✅ 2. vec_sub, vec_add, vec_mul_s
```rust
let a = [3.0, 4.0, 5.0];
let b = [1.0, 2.0, 3.0];
let sub = vec_sub(a, b);
let add = vec_add(a, b);
let scaled = vec_mul_s(a, 2.0);
println!("Sub: {:?}, Add: {:?}, Scaled: {:?}", sub, add, scaled);
```
```
Sub: [2.0, 2.0, 2.0], Add: [4.0, 6.0, 8.0], Scaled: [6.0, 8.0, 10.0]
```


### ✅ 3. vec_len, vec_len2
```rust
let v = [3.0, 4.0, 0.0];
let len = vec_len(v);
let len2 = vec_len2(v);
println!("Length: {}, Squared Length: {}", len, len2);
```
```
Length: 5, Squared Length: 25
```

### ✅ 4. vec_cross
```rust
let a = [1.0, 0.0, 0.0];
let b = [0.0, 1.0, 0.0];
let cross = vec_cross(a, b);
println!("Cross product: {:?}", cross);
```
```
Cross product: [0.0, 0.0, 1.0]
```


### ✅ 5. vec_normalize
```rust
let v = [3.0, 0.0, 4.0];
let norm = vec_normalize(v);
println!("Normalized vector: {:?}", norm);
```
```
Cross product: [0.0, 0.0, 1.0]
```


### ✅ 6. are_coplanar
```rust
let a = [0.0, 0.0, 0.0];
let b = [1.0, 0.0, 0.0];
let c = [0.0, 1.0, 0.0];
let d = [1.0, 1.0, 0.0];
let coplanar = are_coplanar(a, b, c, d, 1e-12);
println!("Are coplanar? {}", coplanar);
```
```
Are coplanar? true
```


### ✅ 7. project_uv
```rust
let a = [0.0, 0.0, 0.0];
let e1 = [1.0, 0.0, 0.0];
let e2 = [0.0, 1.0, 0.0];
let p = [2.0, 3.0, 0.0];
let uv = project_uv(a, e1, e2, p);
println!("Projected UV: {:?}", uv);
```
```
Projected UV: Point2D { x: 2.0, y: 3.0 }
```


### ✅ 8. on_are_equal, on_are_equal_rel_tol
```rust
let a = 1.00000001;
let b = 1.00000002;
let eq = on_are_equal(a, b, 1e-7);
let rel_eq = on_are_equal_rel_tol(a, b);
println!("Equal? {}, Relatively equal? {}", eq, rel_eq);
```
```
Equal? true, Relatively equal? false
```


### ✅ 9. on_compare
```rust
let cmp = on_compare(1.0, 1.0001, 1e-4);
println!("Comparison result: {}", cmp); // -1, 0, or 1
```
```
Comparison result: -1
```


### ✅ 10. on_clamp, on_clamp01, on_clamp11
```rust
let x = 1.5;
let clamped = on_clamp(x, 0.0, 1.0);
let clamped01 = on_clamp01(x);
let clamped11 = on_clamp11(x);
println!("Clamp: {}, Clamp01: {}, Clamp11: {}", clamped, clamped01, clamped11);
```
```
Clamp: 1, Clamp01: 1, Clamp11: 1
```

### ✅ 11. on_rel_err
```rust
let err = on_rel_err(100.0, 101.0);
println!("Relative error: {}", err);
```
```
Relative error: 0.009900990099009901
```


### ✅ 12. on_is_zero, on_is_finite, on_is_infinite
```rust
let z = on_is_zero(1e-15, 1e-12);
let f = on_is_finite(1.0);
let inf = on_is_infinite(f64::INFINITY);
println!("Is zero? {}, Is finite? {}, Is infinite? {}", z, f, inf);
```
```
Is zero? true, Is finite? true, Is infinite? true
```

### ✅ 13. on_are_vector_equal, on_are_vec_equal, on_are_vec2_equal, on_are_pt_equal, on_are_pt2_equal
```rust
let a = vec![1.0, 2.0, 3.0];
let b = vec![1.0, 2.0, 3.00000001];
let eq = on_are_vector_equal(&a, &b, 1e-6);
println!("Vectors equal? {}", eq);
```
```
Vectors equal? true
```

데이터 처리 및 보조 알고리즘 함수 샘플 코드를 소개. 
이 함수들은 곡선 분할, 샘플링, 비틀림 측정 등 다양한 보조 연산에 사용됩니다.

## 🧪 5단계: 데이터 처리 및 보조 알고리즘 함수 샘플 코드
### ✅ 1. on_get_divide_number
```rust
let radius = 10.0;
let delta_radians = std::f64::consts::PI / 2.0;
let deviation = 0.1;
let (segments, angle_step) = on_get_divide_number(radius, delta_radians, deviation);
println!("Segments: {}, Angle step: {}", segments, angle_step);
```
```
Segments: 6, Angle step: 0.2617993877991494
```

### ✅ 2. on_number_of_segments
```rust
let radius = 10.0;
let delta_radians = std::f64::consts::PI;
let deviation = 0.05;
let angle_limit = std::f64::consts::PI / 8.0;
let segments = on_number_of_segments(radius, delta_radians, deviation, angle_limit);
println!("Number of segments: {}", segments);
```
```
Number of segments: 16
```

### ✅ 3. on_get_sampling_2d
```rust
let points = (0..20)
    .map(|i| Point2D::new(i as f64, (i * i) as f64))
    .collect::<Vec<_>>();
let sampled = on_get_sampling_2d(&points);
println!("Sampled 2D points: {:?}", sampled);
```
```
Sampled 2D points: [Point2D { x: 0.0, y: 0.0 }, Point2D { x: 3.0, y: 9.0 }, 
Point2D { x: 6.0, y: 36.0 }, Point2D { x: 9.0, y: 81.0 }, Point2D { x: 12.0, y: 144.0 }, 
Point2D { x: 15.0, y: 225.0 }]
```


### ✅ 4. on_get_sampling_3d
```rust
let points = (0..20)
    .map(|i| Point3D::new(i as f64, (i * i) as f64, (i % 3) as f64))
    .collect::<Vec<_>>();
let sampled = on_get_sampling_3d(&points);
println!("Sampled 3D points: {:?}", sampled);
```
```
Sampled 3D points: [Point3D { x: 0.0, y: 0.0, z: 0.0 }, Point3D { x: 3.0, y: 9.0, z: 0.0 }, 
Point3D { x: 6.0, y: 36.0, z: 0.0 }, Point3D { x: 9.0, y: 81.0, z: 0.0 }, Point3D { x: 12.0, y: 144.0, z: 0.0 }, 
Point3D { x: 15.0, y: 225.0, z: 0.0 }]

```

### ✅ 5. measure_twist
```rust
let p0 = [0.0, 0.0, 0.0];
let p1 = [1.0, 0.0, 0.0];
let p2 = [0.0, 1.0, 0.0];
let p3 = [1.0, 1.0, 0.1]; // 살짝 비틀림
let twist = measure_twist(p0, p1, p2, p3);
println!("Twist measure: {}", twist);
```
```
Twist measure: 0.1
```


## ✅ 1. 수학적 검증: 주요 알고리즘 함수
아래는 첨부 코드에서 사용된 핵심 수학 알고리즘에 대한 수학적 정당성 검토입니다.

### 🔷 on_jacobi_symmetric_eigen
- 목적: 대칭 행렬 $A\in \mathbb{R^{\mathnormal{n\times n}}}$ 의 고유값 분해
- 수학적 원리:
- 야코비 회전 J는 직교 행렬이며 J^TAJ는 대칭 유지
- 반복적으로 오프대각 성분을 제거하여 $A\rightarrow \Lambda$
- 결과: $A=V\Lambda V^T$, $V$ 는 고유벡터 열, $\Lambda$ 는 고유값 대각행렬
- 정당성: 고전적인 야코비 방법이며 수치적으로 안정적

### 🔷 on_solve_tridiagonal_inplace
- 목적: 삼대각 행렬 시스템 $Ax=d$ 풀이
- 수학적 원리:
- Thomas 알고리즘: 전진 제거 + 후진 대입
- 시간복잡도 O(n)
- 정당성: A가 비특이 삼대각 행렬일 때 정확한 해를 구함

### 🔷 on_cholesky_solve_spd
- 목적: SPD 행렬 A에 대해 Ax=b 풀이
- 수학적 원리:
- $A=LL^T$ 분해 후 $LL^Tx=b$
- 전진 대입: $Ly=b$, 후진 대입: $L^Tx=y$
- 정당성: SPD 조건 만족 시 항상 분해 가능

### 🔷 on_adaptive_simpson
- 목적: 함수 f(x)의 구간 적분
- 수학적 원리:
- 심프슨 공식: $\int _a^bf(x)dx\approx \frac{b-a}{6}(f(a)+4f(c)+f(b))$
- 오차가 허용 범위보다 크면 재귀적으로 분할
- 정당성: 수치적 적분에서 널리 사용되는 안정적 기법

### 🔷 on_poly_fit
- 목적: 데이터 $(x_i,y_i)$ 를 다항식으로 근사
- 수학적 원리:
- 최소제곱법: $T^TTc=T^Ty$
- T: Vandermonde 행렬
- 정당성: 과적합 방지 및 안정적 근사 가능

## 🧪 2. 테스트 코드: 모든 함수에 대한 단위 테스트 예시
아래는 각 함수에 대해 Rust 스타일의 테스트 함수 예시입니다. 실제 테스트 프레임워크에서는 #[test] 어노테이션을 붙여 사용합니다.

### 예시: on_distance
```rust
#[test]
fn test_on_distance() {
    let a = Point3D::new(0.0, 0.0, 0.0);
    let b = Point3D::new(3.0, 4.0, 0.0);
    let dist = on_distance(&a, &b);
    assert!((dist - 5.0).abs() < 1e-12);
}
```


### 예시: on_are_equal
```rust
#[test]
fn test_on_are_equal() {
    let a = 1.0000001;
    let b = 1.0000002;
    assert!(on_are_equal(a, b, 1e-6));
    assert!(!on_are_equal(a, b, 1e-8));
}
```


### 예시: vec_cross
```rust
#[test]
fn test_vec_cross() {
    let a = [1.0, 0.0, 0.0];
    let b = [0.0, 1.0, 0.0];
    let cross = on_vec_cross(a, b);
    assert_eq!(cross, [0.0, 0.0, 1.0]);
}
```


### 예시: on_poly_val
```rust
#[test]
fn test_on_poly_val() {
    let coeffs = vec![1.0, 2.0]; // y = 1 + 2x
    let xs = vec![0.0, 1.0, 2.0];
    let ys = on_poly_val(&coeffs, &xs);
    assert_eq!(ys, vec![1.0, 3.0, 5.0]);
}
```


### 예시: on_get_sampling_2d
```rust
#[test]
fn test_on_get_sampling_2d() {
    let pts = (0..10).map(|i| Point2D::new(i as f64, i as f64)).collect::<Vec<_>>();
    let sampled = on_get_sampling_2d(&pts);
    assert!(sampled.len() >= 4);
}
```


```rust
#[cfg(test)]
mod test_geom {
    use nurbslib::core::geom::Point2D;
    use nurbslib::core::maths::{on_are_equal, on_compare, on_distance, on_get_sampling_2d, on_point_in_polygon_simple, on_poly_val, on_unitize, on_vec_cross};
    use nurbslib::core::prelude::{Point3D, Vector3D};
    use super::*;

    #[test]
    fn test_on_distance() {
        let a = Point3D::new(0.0, 0.0, 0.0);
        let b = Point3D::new(3.0, 4.0, 0.0);
        let dist = on_distance(&a, &b);
        assert!((dist - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_on_are_equal() {
        let a = 1.0000001;
        let b = 1.0000002;
        assert!(on_are_equal(a, b, 1e-6));
        assert!(!on_are_equal(a, b, 1e-8));
    }

    #[test]
    fn test_vec_cross() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let cross = on_vec_cross(a, b);
        assert_eq!(cross, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_on_poly_val() {
        let coeffs = vec![1.0, 2.0]; // y = 1 + 2x
        let xs = vec![0.0, 1.0, 2.0];
        let ys = on_poly_val(&coeffs, &xs);
        assert_eq!(ys, vec![1.0, 3.0, 5.0]);
    }

    #[test]
    fn test_on_get_sampling_2d() {
        let pts = (0..10).map(|i| Point2D::new(i as f64, i as f64)).collect::<Vec<_>>();
        let sampled = on_get_sampling_2d(&pts);
        assert!(sampled.len() >= 4);
    }

    #[test]
    fn test_on_unitize() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        let unit = on_unitize(v);
        let len = (unit.x * unit.x + unit.y * unit.y + unit.z * unit.z).sqrt();
        assert!((len - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_point_in_polygon_simple() {
        let polygon = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(2.0, 2.0),
            Point2D::new(0.0, 2.0),
            Point2D::new(0.0, 0.0),
        ];
        let pt = Point2D::new(1.0, 1.0);
        assert!(on_point_in_polygon_simple(&pt, &polygon));
    }

    #[test]
    fn test_on_compare() {
        assert_eq!(on_compare(1.0, 1.00001, 1e-4), 0);
        assert_eq!(on_compare(1.0, 1.0002, 1e-4), -1);
        assert_eq!(on_compare(1.0002, 1.0, 1e-4), 1);
    }
}
```
Rust 스타일의 테스트 코드를 범주별로 정리해서 제공해드릴게요.  
이 코드는 #[cfg(test)] 모듈 안에 넣으면 cargo test로 실행 가능합니다.

## 🧪 테스트 코드 모듈: 수치 해석 함수
```rust
#[cfg(test)]
mod tests_math {
    use super::*;

    #[test]
    fn test_on_adaptive_simpson() {
        let result = on_adaptive_simpson(&|x| x.sin(), 0.0, std::f64::consts::PI, 1e-8, 10);
        assert!((result - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_on_poly_fit_and_val() {
        let t = vec![0.0, 1.0, 2.0, 3.0];
        let v = vec![1.0, 2.0, 3.9, 6.1];
        let coeffs = on_poly_fit(&t, &v, 2);
        let ys = on_poly_val(&coeffs, &t);
        for (y, v) in ys.iter().zip(v.iter()) {
            assert!((y - v).abs() < 0.2);
        }
    }

    #[test]
    fn test_on_cholesky_solve_spd() {
        let mut g = vec![4.0, 2.0, 2.0, 3.0];
        let mut b = vec![6.0, 5.0];
        assert!(on_cholesky_solve_spd(&mut g, &mut b, 2));
        assert!((b[0] - 1.0).abs() < 1e-6);
        assert!((b[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_on_solve_tridiagonal_inplace() {
        let mut a = vec![0.0, 1.0, 1.0];
        let mut b = vec![4.0, 4.0, 4.0];
        let mut c = vec![1.0, 1.0, 0.0];
        let mut d = vec![5.0, 5.0, 5.0];
        assert!(on_solve_tridiagonal_inplace(&mut a, &mut b, &mut c, &mut d));
    }
}
```


## 📐 테스트 코드 모듈: 기하학 함수
```rust
#[cfg(test)]
mod tests_geom {
    use super::*;

    #[test]
    fn test_on_point_on_circle() {
        let pt = on_point_on_circle(Point3D::new(0.0, 0.0, 0.0), 1.0, std::f64::consts::FRAC_PI_2);
        assert!((pt.x.abs() < 1e-6) && ((pt.y - 1.0).abs() < 1e-6));
    }

    #[test]
    fn test_on_circle_from_3_points() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(1.0, 0.0);
        let c = Point2D::new(0.0, 1.0);
        let result = on_circle_from_3_points(&a, &b, &c);
        assert!(result.is_some());
    }

    #[test]
    fn test_on_arc_half_angle() {
        let o = Point2D::new(0.0, 0.0);
        let a = Point2D::new(1.0, 0.0);
        let b = Point2D::new(0.0, 1.0);
        let angle = on_arc_half_angle(&o, &a, &b);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
    }
}
```
다각형 포함 판정 및 벡터/유틸리티 함수들에 대한 테스트 모듈을 정리해드릴게요.  
이 코드는 #[cfg(test)] 아래에 넣으면 cargo test로 실행 가능합니다.

## 🧭 테스트 코드 모듈: 다각형 포함 판정 함수
```rust
#[cfg(test)]
mod tests_polygon {
    use super::*;

    #[test]
    fn test_on_is_clockwise() {
        let pts = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];
        assert!(on_is_clockwise(&pts));
    }

    #[test]
    fn test_point_in_polygon_simple() {
        let poly = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(2.0, 2.0),
            Point2D::new(0.0, 2.0),
            Point2D::new(0.0, 0.0),
        ];
        let pt = Point2D::new(1.0, 1.0);
        assert!(point_in_polygon_simple(&pt, &poly));
    }

    #[test]
    fn test_point_in_polygon_composite() {
        let outer = Polygon2D::from_points(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(4.0, 4.0),
            Point2D::new(0.0, 4.0),
            Point2D::new(0.0, 0.0),
        ]);
        let hole = Polygon2D::from_points(vec![
            Point2D::new(1.0, 1.0),
            Point2D::new(3.0, 1.0),
            Point2D::new(3.0, 3.0),
            Point2D::new(1.0, 3.0),
            Point2D::new(1.0, 1.0),
        ]);
        let pt = Point2D::new(0.5, 0.5);
        assert!(point_in_polygon_composite(&pt, &[outer, hole]));
    }

    #[test]
    fn test_check_diagonal_intersections() {
        let a = [0.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];
        let c = [1.0, 1.0, 0.0];
        let d = [0.0, 1.0, 0.0];
        assert!(check_diagonal_intersections(a, b, c, d));
    }
}
```


## 🧮 테스트 코드 모듈: 벡터 및 유틸리티 함수
```rust

#[cfg(test)]
mod tests_vector_utils {
    use super::*;

    #[test]
    fn test_vec_dot_and_len() {
        let a = [3.0, 4.0, 0.0];
        let b = [0.0, 0.0, 1.0];
        assert_eq!(vec_dot(a, b), 0.0);
        assert_eq!(vec_len(a), 5.0);
    }

    #[test]
    fn test_vec_cross() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let cross = vec_cross(a, b);
        assert_eq!(cross, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_vec_normalize() {
        let v = [3.0, 0.0, 4.0];
        let norm = vec_normalize(v);
        let len = vec_len(norm);
        assert!((len - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_on_are_equal_and_compare() {
        let a = 1.0000001;
        let b = 1.0000002;
        assert!(on_are_equal(a, b, 1e-6));
        assert_eq!(on_compare(a, b, 1e-8), -1);
    }

    #[test]
    fn test_on_clamp() {
        assert_eq!(on_clamp(1.5, 0.0, 1.0), 1.0);
        assert_eq!(on_clamp01(-0.5), 0.0);
        assert_eq!(on_clamp11(2.0), 1.0);
    }

    #[test]
    fn test_on_rel_err() {
        let err = on_rel_err(100.0, 101.0);
        assert!(err > 0.009 && err < 0.011);
    }

    #[test]
    fn test_on_is_finite_and_zero() {
        assert!(on_is_finite(1.0));
        assert!(on_is_zero(1e-15, 1e-12));
    }
}
```


