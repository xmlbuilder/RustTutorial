# 📘 Geometry Kernel Intersection Documentation
- (Line–Circle, Circle–Circle, Arc–Arc, Plane–Plane, Line–Arc 포함)

## 1. Plane–Plane Intersection
- 두 평면이 교차하면 **직선(Line)** 이 된다.
### 1.1 교차 조건
- 두 평면의 법선 벡터 $n_0$, $n_1$ 에 대해:
- 평행 여부
```math
|n_0\cdot n_1|\approx 1
```
- 평행이 아니면 반드시 한 직선에서 만난다.
### 1.2 교차선 방향
```math
d=n_0\times n_1
```
### 1.3 교차선 위의 한 점
- 두 평면 방정식:
```math
n_0\cdot x=d_0,\quad n_1\cdot x=d_1
```
- 두 평면을 동시에 만족하는 점을 구한다.  
    (보통 2×2 선형 시스템으로 z=0 또는 특정 축을 고정하여 계산)

## 2. Line–Circle Intersection
- 평면 위의 원과 직선의 교차.
### 2.1 직선 파라미터식
```math
L(t)=P+tD
```
### 2.2 원의 방정식
```math
|X-C|^2=r^2
```
### 2.3 대입하여 2차 방정식
```math
|P+tD-C|^2=r^2
```
- 전개하면:
```math
(D\cdot D)t^2+2D\cdot (P-C)t+|P-C|^2-r^2=0
```
### 2.4 해의 개수
- 판별식 $\Delta >0$ → 2점
- $\Delta =0$ → 1점 (tangent)
- $\Delta <0$ → 없음

## 3. Circle–Circle Intersection
- 두 원이 같은 평면에 있을 때의 교차.
### 3.1 중심 거리
```math
d=|C_1-C_0|
```
### 3.2 교차 조건
- 분리됨
```math
d>r_0+r_1
```
- 한 원이 다른 원 안에 있음
```math
d<|r_0-r_1|
```
- 동심원
```math
d=0
```
### 3.3 교차점 계산 공식
```math
a=\frac{r_0^2-r_1^2+d^2}{2d}
```
```math
h=\sqrt{r_0^2-a^2}
```
- 기준점:
```math
P_2=C_0+a\hat {d}
```
- 여기서:
```math
\hat {d}=\frac{C_1-C_0}{d}
```
- 평면 내 수직 벡터:
```math
d_{\perp }=\hat {d}\times n
```
- 최종 교차점:
```math
X_0=P_2+hd_{\perp }
```
```math
X_1=P_2-hd_{\perp }
```
## 4. Arc–Arc Intersection
- Arc는 Circle의 부분 구간이므로:
    - 먼저 Circle–Circle 교차점을 구한다
    - 각 교차점이 Arc의 domain에 포함되는지 검사한다
### 4.1 Arc parameter domain
- Arc는 angle domain [t_0,t_1]을 가진다.
### 4.2 Arc 위에 있는지 검사
- 교차점 Q에 대해:
    - Arc의 circle에서 parameter t 계산
    - t가 domain 안에 있는지 확인
    - 실제 거리도 확인
```math
|Q-A(t)|<\mathrm{abs\_ tol}
```
## 5. Arc Parameter Projection
- Arc에서 점 P의 parameter를 구하는 과정.
### 5.1 Circle projection
- 평면 좌표계 (u, v)로 투영:
```math
t=\mathrm{atan2}(v,u)
```
### 5.2 Arc domain 밖이면 endpoint로 snap
```math
t<t_0\Rightarrow t=t_0
```
```math
t>t_1\Rightarrow t=t_1
```
## 6. Tolerance Model
- 기하학 엔진에서 매우 중요.
### 6.1 절대 허용 오차
```math
\mathrm{abs\_ tol}=\max (\mathrm{scale_{\mathnormal{0}}},\mathrm{scale_{\mathnormal{1}}})\cdot \mathrm{ON\_ REL\_ TOL}
```
### 6.2 점 일치 판단
```math
|P-Q|<\mathrm{abs\_ tol}
```
## 7. Rust Implementation Notes
- Rust에서는 기하학보다 borrow 모델이 더 까다롭다.
### 7.1 split_at_mut 필수
- 같은 배열에서 두 개의 &mut를 얻으면 안 된다.
```rust
let (left, right) = arr.split_at_mut(1);
```

### 7.2 shadowing 금지
- 이 패턴은 절대 쓰면 안 된다:
```rust
let (p0, p1) = slice.split_at_mut(1);
let p0 = &mut p0[0];   // ❌ shadowing
```

### 7.3 올바른 패턴
```rust
let (s0, s1) = slice.split_at_mut(1);
let p0_ref = &mut s0[0];
let p1_ref = &mut s1[0];
```

### 7.4 Arc–Arc는 Circle–Circle이 핵심
- Arc–Arc의 90%는 Circle–Circle이 결정한다.

## 8. 전체 흐름 요약
```
Arc–Arc
 └─ Circle–Circle
      ├─ coplanar → 표준 공식
      └─ non-coplanar → plane-plane → line-circle
 └─ Arc domain check
```

---
## 소스 코드
```rust
/// points_in: 3D 점 목록
/// 반환: 평면 피팅 후 (u,v)로 정렬된 PointUV 목록
pub fn on_sort_3d_points_by_fitted_plane(points_in: &[Point3D]) -> Option<Vec<PointUV>> {
    if points_in.len() < 3 {
        return None;
    }

    // 1) 자동 평면 피팅
    let plane = Plane::fit_from_points(points_in)?;
    if !plane.is_valid() {
        return None;
    }

    // 2) UV 변환
    let mut out: Vec<PointUV> = Vec::with_capacity(points_in.len());

    for p in points_in {
        let (u, v) = plane.project_params(*p);
        out.push(PointUV::new(*p, Point2D::new(u, v)));
    }

    // 3) UV 기준 정렬 (U → V)
    out.sort_by(|a, b| {
        let ax = a.uv.x;
        let bx = b.uv.x;

        if ax == bx {
            a.uv.y
                .partial_cmp(&b.uv.y)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            ax.partial_cmp(&bx).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
    Some(out)
}
```
```rust
pub fn on_intersect_2d_line_circle(
    mut from: Point2D,
    mut to: Point2D,
    r: f64,
    tol: f64,
) -> (i32, f64, f64) {
    let mut t0 = 0.0;
    let mut t1 = 0.0;

    // swap if needed
    let mut rev = false;
    if from.x * from.x + from.y * from.y > to.x * to.x + to.y * to.y {
        std::mem::swap(&mut from, &mut to);
        rev = true;
    }

    let mut dx = to.x - from.x;
    let dy = to.y - from.y;

    // compute d
    let d = if dx.abs() >= dy.abs() {
        if dx == 0.0 {
            return (0, 0.0, 0.0);
        }
        let t = dy / dx;
        dx.abs() * (1.0 + t * t).sqrt()
    } else {
        let t = dx / dy;
        dy.abs() * (1.0 + t * t).sqrt()
    };

    let c = dx / d;
    let s = dy / d;

    // rotate coordinates
    let mut x = from.x;
    let mut y = from.y;
    from.x = c * x + s * y;
    from.y = c * y - s * x;

    x = to.x;
    y = to.y;
    to.x = c * x + s * y;
    to.y = c * y - s * x;

    dx = to.x - from.x;
    if dx == 0.0 {
        return (0, 0.0, 0.0);
    }

    let mut t = -from.x / dx;
    x = (1.0 - t) * from.x + t * to.x;
    y = (1.0 - t) * from.y + t * to.y;

    let mut d2 = y.abs();

    let x_cnt;
    if d2 < r - tol {
        // two intersections
        d2 /= r;
        let d3 = r * (1.0 - d2 * d2).sqrt();

        let mut a = -(d3 + from.x) / dx;
        let mut b = (d3 - from.x) / dx;

        if rev {
            a = 1.0 - a;
            b = 1.0 - b;
        }

        if a <= b {
            t0 = a;
            t1 = b;
        } else {
            t0 = b;
            t1 = a;
        }

        x_cnt = if (a - b).abs() < 1e-14 { 1 } else { 2 };
    } else if d2 > r + tol {
        // closest point
        x_cnt = 3;
        if rev {
            t = 1.0 - t;
        }
        t0 = t;
        t1 = t;
    } else {
        // tangent
        x_cnt = 1;
        if rev {
            t = 1.0 - t;
        }
        t0 = t;
        t1 = t;
    }

    (x_cnt, t0, t1)
}
```
```rust
/// line–cylinder intersection
/// return:
///   0 = no intersection
///   1 = one point (tangent or merged)
///   2 = two intersection points
///   3 = line overlaps cylinder
pub fn on_intersect_line_cylinder(line: &Line, cylinder: &Cylinder) -> (i32, Point3D, Point3D) {
    let mut rc = 0;
    let mut a = Point3D::origin();
    let mut b = Point3D::origin();

    let radius = cylinder.circle.radius.abs();
    let mut tol = radius * ON_SQRT_EPSILON;
    if tol < ON_ZERO_TOL {
        tol = ON_ZERO_TOL;
    }

    // println!(
    //     "[icyl] radius={} tol={} line=({:?} -> {:?})",
    //     radius, tol, line.start, line.end
    // );
    // println!(
    //     "[icyl] cyl.origin={:?} x={:?} y={:?} z={:?} height=[{}, {}]",
    //     cylinder.circle.plane.origin,
    //     cylinder.circle.plane.x_axis,
    //     cylinder.circle.plane.y_axis,
    //     cylinder.circle.plane.z_axis,
    //     cylinder.height[0],
    //     cylinder.height[1],
    // );

    // cylinder axis line
    let origin = cylinder.circle.plane.origin;
    let z = cylinder.circle.plane.z_axis;

    let axis_from = origin + z.to_point() * cylinder.height[0];
    let axis_to = origin + z.to_point() * cylinder.height[1];

    let mut axis = Line::new(axis_from, axis_to);
    let mut finite = true;

    if axis.length() <= tol {
        axis = Line::new(origin, origin + z.to_point());
        finite = false;
    }

    // println!(
    //     "[icyl] axis=({:?}->{:?}) finite={} axis_len={}",
    //     axis.start, axis.end, finite, axis.length()
    // );

    // line–axis intersection (or fallback to closest points)
    let mut line_t = None;
    let mut axis_t = None;

    let hit_ll = on_intersect_line_line_param01(line, &axis, &mut line_t, &mut axis_t);
    // println!(
    //     "[icyl] line-line hit={} line_t={:?} axis_t={:?}",
    //     hit_ll, line_t, axis_t
    // );

    if !hit_ll {
        // IMPORTANT: 원본(OpenNURBS)은 무한 직선 기준 ClosestPointTo.
        // 너의 closest_point_param()이 segment clamp / domain 변환이면 결과가 달라질 수 있음.
        let (_, at) = axis.closest_point_param(origin);
        let (_, lt) = line.closest_point_param(origin);
        axis_t = Some(at);
        line_t = Some(lt);
        // println!(
        //     "[icyl] fallback closest to origin => axis_t={} line_t={}",
        //     at, lt
        // );
    }

    let lt = line_t.unwrap_or(0.0);
    let mut at = axis_t.unwrap_or(0.0);

    let mut line_pt = line.point_at_normalized(lt);
    let mut axis_pt = axis.point_at_normalized(at);

    let mut d = line_pt.distance(&axis_pt);

    // println!(
    //     "[icyl] seed lt={} at={} line_pt={:?} axis_pt={:?} d={}",
    //     lt, at, line_pt, axis_pt, d
    // );

    if finite {
        let at_before = at;
        if at < 0.0 {
            at = 0.0;
        } else if at > 1.0 {
            at = 1.0;
        }
        if (at - at_before).abs() > 0.0 {
            axis_pt = axis.point_at_normalized(at);
            // println!(
            //     "[icyl] clamp axis_t {} -> {} axis_pt={:?}",
            //     at_before, at, axis_pt
            // );
        } else {
            // println!("[icyl] axis_t in range, no clamp");
        }
    }

    // tangent or no hit
    if d >= radius - tol {
        rc = if d <= radius + tol { 1 } else { 0 };

        a = line_pt;

        let mut v = line_pt - axis_pt;
        if finite {
            let proj = v.dot_vec(&z);
            v = v - z.to_point() * proj;
        }
        v = v.unitize();
        b = axis_pt + v * radius;

        // println!(
        //     "[icyl] early branch: d={} radius={} => rc={} A(line_pt)={:?} B(axis+rad*v)={:?}",
        //     d, radius, rc, a, b
        // );

        if rc == 1 {
            // check overlap
            let (p1, _t1) = axis.closest_point_param(line.start);
            let mut d1 = p1.distance(&line.start);

            // println!(
            //     "[icyl] overlap check1: line.start={:?} axis_cp={:?} axis_t={} d1={}",
            //     line.start, p1, t1, d1
            // );

            if (d1 - radius).abs() <= tol {
                let (p2, _t2) = axis.closest_point_param(line.end);
                d1 = p2.distance(&line.end);

                // println!(
                //     "[icyl] overlap check2: line.end={:?} axis_cp={:?} axis_t={} d2={}",
                //     line.end, p2, t2, d1
                // );

                if (d1 - radius).abs() <= tol {
                    rc = 3;
                    a = cylinder.closest_point_to(&line.start);
                    b = cylinder.closest_point_to(&line.end);

                    // println!(
                    //     "[icyl] OVERLAP => rc=3 A={:?} B={:?}",
                    //     a, b
                    // );
                }
            }
        }
        return (rc, a, b);
    }

    // general case: solve quadratic in cylinder local coords
    let px = cylinder.circle.plane.x_axis;
    let py = cylinder.circle.plane.y_axis;

    let lf = line.start - origin;
    let lt_vec = line.end - origin;

    let x0 = lf.dot_vec(&px);
    let y0 = lf.dot_vec(&py);
    let x1 = lt_vec.dot_vec(&px);
    let y1 = lt_vec.dot_vec(&py);

    let dx = x1 - x0;
    let dy = y1 - y0;

    let ax = dx * dx;
    let bx = 2.0 * dx * x0;
    let cx = x0 * x0;

    let ay = dy * dy;
    let by = 2.0 * dy * y0;
    let cy = y0 * y0;

    // IMPORTANT:
    // 네 RealRootSolver는 a=coef[2], b=coef[1], c=coef[0]로 읽으므로 [c,b,a]로 넣는 게 맞음.
    let coef = [
        cx + cy - radius * radius, // c
        bx + by,                   // b
        ax + ay,                   // a
    ];

    // println!(
    //     "[icyl] quad setup: (x0,y0)=({},{}) (x1,y1)=({},{}) (dx,dy)=({},{})",
    //     x0, y0, x1, y1, dx, dy
    // );
    // println!(
    //     "[icyl] quad coef in solver-order [c,b,a]=[{:.17e}, {:.17e}, {:.17e}]",
    //     coef[0], coef[1], coef[2]
    // );

    let mut roots = Vec::new();
    let nroot = RealRootSolver::solve_quadratic(coef, tol, &mut roots);

    // println!(
    //     "[icyl] solve_quadratic => nroot={} roots={:?}",
    //     nroot, roots
    // );

    if nroot == 0 {
        // println!("[icyl] => return rc=0 (no roots)");
        return (0, a, b);
    }

    let t0 = roots[0];
    let t1 = if roots.len() > 1 { roots[1] } else { roots[0] };

    let pa = line.point_at_normalized(t0);
    let pb = line.point_at_normalized(t1);

    // println!(
    //     "[icyl] t0={} t1={} pa={:?} pb={:?}",
    //     t0, t1, pa, pb
    // );

    a = cylinder.closest_point_to(&pa);
    b = cylinder.closest_point_to(&pb);

    d = a.distance(&b);

    // println!(
    //     "[icyl] closest back to cyl: A={:?} B={:?} |A-B|={}",
    //     a, b, d
    // );

    if d <= ON_ZERO_TOL {
        // collapse to single point
        line_pt = line.point_at_normalized(lt);
        let mut v = line_pt - axis_pt;

        if finite {
            let proj = v.dot_vec(&z);
            v = v - z.to_point() * proj;
        }

        let v_u = v.unitize();
        let _ = v_u;

        a = line_pt;
        b = axis_pt + v * radius;
        rc = 1;

        // println!(
        //     "[icyl] collapse => rc=1 A(line_pt)={:?} B(axis+rad*v)={:?}",
        //     a, b
        // );
    } else {
        rc = 2;
        // println!("[icyl] => rc=2 (two points)");
    }
    (rc, a, b)
}
```
```rust
pub fn on_intersect_plane_plane(pln1: &Plane, pln2: &Plane) -> Option<Line> {
    // direction = S.z × R.z
    let d = pln2.z_axis.cross(&pln1.z_axis);
    if d.length() < 1e-12 {
        return None; // 평행 or 동일 평면
    }

    // 중간점
    let p = (pln1.origin + pln2.origin) * 0.5;

    // 세 번째 평면 T(p, d)
    let t = Plane::from_origin_normal(p, d).expect("Invalid Plane");

    // 세 평면의 교차점
    let mut origin = Point3D::origin();
    if !intersect_plane_plane_plane(pln1, pln2, &t, &mut origin) {
        return None;
    }

    // 교차선 = P + t*d
    let line = Line::new(origin, origin + d.to_point());
    Some(line)
}
```
```rust
pub fn intersect_plane_plane_plane(
    pln1: &Plane,
    pln2: &Plane,
    pln3: &Plane,
    out_p: &mut Point3D,
) -> bool {
    // Ax = -d
    let a = [
        [pln1.equation.a, pln1.equation.b, pln1.equation.c],
        [pln2.equation.a, pln2.equation.b, pln2.equation.c],
        [pln3.equation.a, pln3.equation.b, pln3.equation.c],
    ];

    let b = [-pln1.equation.d, -pln2.equation.d, -pln3.equation.d];

    if let Some(sol) = on_solve3x3(a, b) {
        out_p.x = sol[0];
        out_p.y = sol[1];
        out_p.z = sol[2];
        true
    } else {
        false
    }
}
```
```rust
/// Returns:
/// 0 = no intersection
/// 1 = single point (tangent)
/// 2 = circle
pub fn intersect_plane_sphere(
    plane: &Plane,
    sphere: &Sphere,
    circle: &mut Circle,
) -> i32 {
    let mut rc = 0;

    let sphere_radius = sphere.radius.abs();
    let mut tol = sphere_radius * ON_SQRT_EPSILON;
    if tol < ON_ZERO_TOL {
        tol = ON_ZERO_TOL;
    }

    let sphere_center = sphere.center();
    let mut circle_center = plane.closest_point_point(&sphere_center);
    let mut d = circle_center.distance(&sphere_center);

    circle.radius = 0.0;

    if sphere_radius.is_finite() && d.is_finite() && d <= sphere_radius + tol {
        if sphere_radius > 0.0 {
            let mut x = d / sphere_radius;
            x = 1.0 - x * x;

            // d > 4*EPSILON → numerical stability rule from OpenNURBS
            if x > 4.0 * ON_EPSILON {
                circle.radius = sphere_radius * x.sqrt();
            } else {
                circle.radius = 0.0;
            }
        } else {
            circle.radius = 0.0;
        }

        if circle.radius <= ON_ZERO_TOL {
            // tangent → single point
            rc = 1;
            circle.radius = 0.0;

            // adjust point to lie exactly on sphere if closer
            let r = circle_center - sphere_center;
            let r0 = r.length();

            if r0 > 0.0 {
                let r_unit = r.unitize();
                let c1 = sphere_center + r_unit * sphere_radius;
                let r1 = c1.distance(&sphere_center);

                if (sphere.radius - r1).abs() < (sphere.radius - r0).abs() {
                    circle_center = c1;
                }
            }
        } else {
            // proper circle
            rc = 2;
        }
    }

    // update circle plane
    circle.plane = *plane;
    circle.plane.origin = circle_center;
    circle.plane.update_equation();

    rc
}
```
```rust
/// 2D line–circle intersection helper
/// returns: 0,1,2, or 3 (3 means tangent but duplicated)
pub fn on_intersect_2d_line_circle_mut(
    p0: (f64, f64),
    p1: (f64, f64),
    r: f64,
    tol: f64,
    t0: &mut f64,
    t1: &mut f64,
) -> i32 {
    let (x0, y0) = p0;
    let (x1, y1) = p1;

    let dx = x1 - x0;
    let dy = y1 - y0;

    let a = dx * dx + dy * dy;
    if a.abs() < tol {
        return 0;
    }

    let b = 2.0 * (x0 * dx + y0 * dy);
    let c = x0 * x0 + y0 * y0 - r * r;

    let disc = b * b - 4.0 * a * c;
    if disc < -tol {
        return 0;
    }

    if disc.abs() <= tol {
        // tangent
        let t = -b / (2.0 * a);
        *t0 = t;
        *t1 = t;
        return 1;
    }

    let s = disc.sqrt();
    let tt0 = (-b - s) / (2.0 * a);
    let tt1 = (-b + s) / (2.0 * a);

    *t0 = tt0;
    *t1 = tt1;

    2
}
```
```rust
pub fn on_intersect_line_circle(
    line: &Line,
    circle: &Circle,
    mut ln_t0: Option<&mut f64>,
    c_pt0: &mut Point3D,
    mut ln_t1: Option<&mut f64>,
    c_pt1: &mut Point3D,
) -> i32 {
    // 1) Build change-of-basis transform: world → circle plane
    let plane = circle.plane;

    let to_plane = Xform::change_of_basis(
        Point3D::origin(),
        Vector3D::world_x(),
        Vector3D::world_y(),
        Vector3D::world_z(),
        plane.origin,
        plane.x_axis,
        plane.y_axis,
        plane.z_axis,
    ).expect("change_of_basis failed");

    let from_plane = Xform::change_of_basis(
        plane.origin,
        plane.x_axis,
        plane.y_axis,
        plane.z_axis,
        Point3D::origin(),
        Vector3D::world_x(),
        Vector3D::world_y(),
        Vector3D::world_z(),
    ).expect("change_of_basis failed");

    // 2) Transform line into circle-plane coordinates
    let lf = to_plane.multi_point_left(&line.start);
    let lt = to_plane.multi_point_left(&line.end);

    let l2 = Line::from_xyz(lf.x, lf.y, lf.z, lt.x, lt.y, lt.z);

    let r = circle.radius.abs();
    let mut tol = (circle.maximum_coordinate() + r) * ON_ZERO_TOL;
    if tol < ON_ZERO_TOL {
        tol = ON_ZERO_TOL;
    }

    // 3) Special case: line is vertical in plane coords
    let dx = (l2.start.x - l2.end.x).abs();
    let dy = (l2.start.y - l2.end.y).abs();
    let dz = (l2.start.z - l2.end.z).abs();

    let mut t0 = 0.0;
    let mut t1 = 0.0;
    let mut xcnt;

    if dx <= tol && dy <= tol && dz > tol {
        // vertical line: x,y constant
        let x = l2.start.x;
        let y = l2.start.y;
        if (x * x + y * y - r * r).abs() < tol {
            t0 = -l2.start.z / (l2.end.z - l2.start.z);
            xcnt = 1;
        } else {
            xcnt = 0;
        }
    } else {
        // general case: 2D line-circle intersection
        xcnt = on_intersect_2d_line_circle_mut(
            (l2.start.x, l2.start.y),
            (l2.end.x, l2.end.y),
            r,
            tol,
            &mut t0,
            &mut t1,
        );
        if xcnt == 3 {
            xcnt = 1;
        }
    }

    // 4) Validate intersection points in 3D
    let mut rcnt = 0;

    if xcnt > 0 {
        // world-space line points
        let lp0 = line.point_at_normalized(t0);
        *c_pt0 = circle.closest_point_to_point(lp0);
        let x0 = c_pt0.distance(&lp0) <= tol;

        let mut x1 = false;
        let mut lp1 = Point3D::origin();

        if xcnt == 2 {
            lp1 = line.point_at_normalized(t1);
            *c_pt1 = circle.closest_point_to_point(lp1);
            x1 = c_pt1.distance(&lp1) <= tol;
        }

        if x0 {
            rcnt += 1;
        }

        if x1 {
            rcnt += 1;
            if rcnt == 1 {
                *c_pt0 = *c_pt1;
                if let Some(t0_out) = ln_t0.as_deref_mut() {
                    *t0_out = t1;
                }
            }
        }

        if rcnt >= 1 {
            if let Some(t0_out) = ln_t0.as_deref_mut() {
                *t0_out = t0;
            }
        }
        if rcnt == 2 {
            if let Some(t1_out) = ln_t1.as_deref_mut() {
                *t1_out = t1;
            }
        }
    }

    rcnt
}
```
```rust
pub fn on_intersect_line_arc(
    line: &Line,
    arc: &Arc,
    line_t0: Option<&mut f64>,
    arc_point0: &mut Point3D,
    line_t1: Option<&mut f64>,
    arc_point1: &mut Point3D,
) -> i32 {
    let c = arc.circle;
    let mut p = [Point3D::origin(), Point3D::origin()];
    let mut t = [0.0f64; 2];
    let mut a = [0.0f64; 2];
    let mut s: f64;
    let mut b = [false, false];

    let (t0, t1) = t.split_at_mut(1);
    let (p0, p1) = p.split_at_mut(1);


    let mut xcnt = on_intersect_line_circle(line, &c, Some(&mut t0[0]), &mut p0[0], Some(&mut t1[0]), &mut p1[0]);

    if xcnt > 0 {
        let arc_dom = arc.domain();

        for i in 0..xcnt {
            b[i as usize] = c.closest_point_to(&p[i as usize], &mut a[i as usize]);
            if b[i as usize] {
                s = arc_dom.normalized_parameter_at(a[i as usize]);
                if s < 0.0 {
                    if s >= -ON_SQRT_EPSILON {
                        a[i as usize] = arc_dom.t0;
                        p[i as usize] = c.point_at(a[i as usize]);
                        let (cp, tt) = line.closest_point_param(p[i as usize]);
                        p[i as usize] = cp;
                        t[i as usize] = tt;
                        b[i as usize] = true;
                    } else {
                        b[i as usize] = false;
                    }
                } else if s > 1.0 {
                    if s <= 1.0 + ON_SQRT_EPSILON {
                        a[i as usize] = arc_dom.t1;
                        p[i as usize] = c.point_at(a[i as usize]);
                        let (cp, tt) = line.closest_point_param(p[i as usize]);
                        p[i as usize] = cp;
                        t[i as usize] = tt;
                        b[i as usize] = true;
                    } else {
                        b[i as usize] = false;
                    }
                }
            }
        }

        if !b[0] && !b[1] {
            xcnt = 0;
        }

        if xcnt == 2 {
            if !b[1] {
                xcnt = 1;
            }
            if !b[0] {
                xcnt = 1;
                b[0] = b[1];
                t[0] = t[1];
                a[0] = a[1];
                p[0] = p[1];
                b[1] = false;
            }
            if xcnt == 2 && (t[0] - t[1]).abs() <= 0.0 {
                xcnt = 1;
                b[1] = false;
                let q = line.point_at(t[0]);
                if p[0].distance(&q) > p[1].distance(&q) {
                    a[0] = a[1];
                    t[0] = t[1];
                    p[0] = p[1];
                }
            }
        }

        if xcnt == 1 && !b[0] {
            xcnt = 0;
        }

        if xcnt >= 1 {
            if let Some(t0) = line_t0 {
                *t0 = t[0];
            }
            *arc_point0 = p[0];
        }
        if xcnt == 2 {
            if let Some(t1) = line_t1 {
                *t1 = t[1];
            }
            *arc_point1 = p[1];
        }
    }

    xcnt
}
```
```rust
pub fn intersect_plane_arc(
    plane: &Plane,
    arc: &Arc,
    point0: &mut Point3D,
    point1: &mut Point3D,
) -> i32 {
    let mut xline: Line;
    let mut a = 0.0;
    let mut b = 0.0;

    if let Some(l) = on_intersect_plane_plane(&plane, &arc.plane()) {
        xline = l;
        let mut p0 = Point3D::origin();
        let mut p1 = Point3D::origin();
        let rc = on_intersect_line_arc(&xline, arc, Some(&mut a), &mut p0, Some(&mut b), &mut p1);
        *point0 = p0;
        *point1 = p1;
        rc
    } else {
        let d = plane.equation.value_at(arc.start_point());
        if d < ON_ZERO_TOL {
            3
        } else {
            0
        }
    }
}
```
```rust
fn points_are_coincident(p: &Point3D, q: &Point3D, tol: f64) -> bool {
    p.distance(q) <= tol
}
```
```rust
pub fn on_intersect_circle_circle(
    c0: &Circle,
    c1: &Circle,
    p0: &mut Point3D,
    p1: &mut Point3D,
) -> i32 {
    *p0 = Point3D::nan();
    *p1 = Point3D::nan();
    let mut x_cnt = -1;

    let cos_tol = ON_ZERO_TOL;

    let scale0 = c0.maximum_coordinate();
    let mut abs_tol = c1.maximum_coordinate();
    if abs_tol < scale0 {
        abs_tol = scale0;
    }
    abs_tol *= ON_REL_TOL;
    if abs_tol < ON_ZERO_TOL {
        abs_tol = ON_ZERO_TOL;
    }

    let n0 = c0.plane.z_axis;
    let n1 = c1.plane.z_axis;
    let parallel = n0.dot(&n1).abs() > 1.0 - cos_tol;
    let coplanar = parallel && (c0.plane.distance_to(&c1.plane.origin) < abs_tol);

    if coplanar {
        let mut c = [c0, c1];
        if c1.radius >= c0.radius {
            c[0] = c1;
            c[1] = c0;
        }
        let r0 = c[0].radius;
        let r1 = c[1].radius;

        let mut d_vec = c[1].center() - c[0].center();
        let len = d_vec.length();

        if len > abs_tol {
            d_vec.normalize();
            let dperp = d_vec.cross_vec(&c[0].plane_normal());

            if len > r0 + r1 + abs_tol {
                x_cnt = 0;
            } else if len + r1 + abs_tol < r0 {
                x_cnt = 0;
            } else {
                let d1 = (r0 * r0 - r1 * r1 + len * len) / (2.0 * len);
                let mut a1 = r0 * r0 - d1 * d1;
                if a1 < 0.0 {
                    a1 = 0.0;
                }
                a1 = a1.sqrt();

                if a1 < 0.5 * abs_tol {
                    x_cnt = 1;
                    *p0 = c[0].center() + d1 * d_vec;
                } else {
                    x_cnt = 2;
                    *p0 = c[0].center() + d1 * d_vec + a1 * dperp.to_point();
                    *p1 = c[0].center() + d1 * d_vec - a1 * dperp.to_point();
                }
            }
        } else if (r0 - r1).abs() < abs_tol {
            x_cnt = 3;
        } else {
            x_cnt = 0;
        }
    } else if !parallel {
        if let Some(px_line) = on_intersect_plane_plane(&c0.plane, &c1.plane) {
            let mut cxl = [[Point3D::origin(); 2]; 2];
            let (row0, row1) = cxl.split_at_mut(1);
            let mut t0 = 0.0;
            let mut t1 = 0.0;
            let x0;
            let x1;
            {
                let r0 = &mut row0[0];
                let (left, right) = r0.split_at_mut(1);
                let p0 = &mut left[0]; // row0[0]
                let p1 = &mut right[0]; // row0[1]
                x0 = on_intersect_line_circle(&px_line, c0, Some(&mut t0), p0, Some(&mut t1), p1);
            }
            {
                let r1 = &mut row1[0];
                let (left, right) = r1.split_at_mut(1);
                let p0 = &mut left[0]; // row0[0]
                let p1 = &mut right[0]; // row0[1]
                x1 = on_intersect_line_circle(&px_line, c1, Some(&mut t0), p0, Some(&mut t1), p1);
            }

            x_cnt = 0;
            for i in 0..x0 {
                let mut j = 0;
                while j < x1 {
                    if points_are_coincident(&cxl[0][i as usize], &cxl[1][j as usize], abs_tol) {
                        break;
                    }
                    j += 1;
                }
                if j < x1 {
                    if x_cnt == 0 {
                        *p0 = cxl[0][i as usize];
                    } else {
                        *p1 = cxl[0][i as usize];
                    }
                    x_cnt += 1;
                }
            }
        }
    }

    x_cnt
}
```
```rust
pub fn intersect_arc_arc(
    a0: &Arc,
    a1: &Arc,
    p0: &mut Point3D,
    p1: &mut Point3D,
) -> i32 {
    println!("=== intersect_arc_arc START ===");
    println!("a0: center={:?}, r={}, domain={:?}", a0.circle.center(), a0.radius(), a0.domain());
    println!("a1: center={:?}, r={}, domain={:?}", a1.circle.center(), a1.radius(), a1.domain());

    *p0 = Point3D::nan();
    *p1 = Point3D::nan();

    let mut xcnt = 0;
    let mut out = [p0, p1];

    // -----------------------------
    // 1) tolerance 계산
    // -----------------------------
    let mut abs_tol = a0.maximum_coordinate().max(a1.maximum_coordinate());
    println!("maximum_coordinate: a0={}, a1={}", a0.maximum_coordinate(), a1.maximum_coordinate());

    abs_tol *= ON_ZERO_TOL;
    if abs_tol < ON_ZERO_TOL {
        abs_tol = ON_ZERO_TOL;
    }
    println!("abs_tol = {}", abs_tol);

    // -----------------------------
    // 2) circle-circle 교차
    // -----------------------------
    let mut ccx = [Point3D::origin(), Point3D::origin()];
    let (left, right) = ccx.split_at_mut(1);

    let cx_cnt = on_intersect_circle_circle(
        &a0.circle,
        &a1.circle,
        &mut left[0],
        &mut right[0],
    );

    println!("circle-circle cx_cnt = {}", cx_cnt);
    println!("ccx[0] = {:?}", ccx[0]);
    println!("ccx[1] = {:?}", ccx[1]);

    // -----------------------------
    // 3) 일반적인 0,1,2 교차 처리
    // -----------------------------
    if cx_cnt < 3 {
        println!("-- cx_cnt < 3 branch --");

        for i in 0..cx_cnt {
            let q = ccx[i as usize];
            println!("checking q[{}] = {:?}", i, q);

            // arc0 위에 있는지
            let mut t0 = a0.closest_param_to(q);
            let pa = a0.point_at(t0);
            println!("  a0.closest_param_to = {}, pa={:?}, dist={}", t0, pa, q.distance(&pa));

            if q.distance(&pa) < abs_tol {
                // arc1 위에 있는지
                let mut t1 = a1.closest_param_to(q);
                let pb = a1.point_at(t1);
                println!("  a1.closest_param_to = {}, pb={:?}, dist={}", t1, pb, q.distance(&pb));

                if q.distance(&pb) < abs_tol {
                    println!("  -> accepted intersection");
                    *out[xcnt as usize] = q;
                    xcnt += 1;
                } else {
                    println!("  -> rejected by arc1");
                }
            } else {
                println!("  -> rejected by arc0");
            }
        }

        println!("=== intersect_arc_arc END (xcnt={}) ===", xcnt);
        return xcnt;
    }

    // -----------------------------
    // 4) cx_cnt == 3 (동심원 또는 특수 케이스)
    // -----------------------------
    println!("-- cx_cnt == 3 branch --");

    let mut size = [a0, a1];
    if a0.domain().length() > a1.domain().length() {
        println!("swap size[0] <-> size[1]");
        size.swap(0, 1);
    }

    let mut little_end_match = [0.0f64; 2];

    let mut big_interior = size[1].domain();
    println!("big_interior before expand = {:?}", big_interior);

    if !big_interior.expand(-abs_tol / size[1].radius()) {
        println!("expand failed, using singleton");
        big_interior = Interval::singleton(size[1].domain().mid());
    }

    println!("big_interior after expand = {:?}", big_interior);

    for ei in 0..2 {
        let little_end = if ei == 0 {
            size[0].start_point()
        } else {
            size[0].end_point()
        };

        println!("checking little_end[{}] = {:?}", ei, little_end);

        if let (_, mut t) = size[1].circle.closest_point(little_end) {
            println!("  closest_point t(before clamp) = {}", t);

            let clamped = big_interior.clamp_mut(&mut t);
            println!("  clamp result = {}, t(after clamp) = {}", clamped, t);

            little_end_match[ei] = match clamped {
                -1 => {
                    if size[1].start_point().distance(&little_end) < abs_tol {
                        0.0
                    } else {
                        -1.0
                    }
                }
                0 => 0.5,
                1 => {
                    if size[1].end_point().distance(&little_end) < abs_tol {
                        1.0
                    } else {
                        -1.0
                    }
                }
                _ => -1.0,
            };

            println!("  little_end_match[{}] = {}", ei, little_end_match[ei]);
        }
    }

    println!("little_end_match = {:?}", little_end_match);

    if little_end_match[0] == 0.5 || little_end_match[1] == 0.5 {
        println!("-> return 3");
        return 3;
    }

    if little_end_match[0] == -1.0 && little_end_match[1] == -1.0 {
        println!("-> return 0");
        return 0;
    }

    if little_end_match[0] == -1.0 {
        println!("-> return 1 (end_point)");
        *out[xcnt as usize] = size[0].end_point();
        return xcnt + 1;
    }

    if little_end_match[1] == -1.0 {
        println!("-> return 1 (start_point)");
        *out[xcnt as usize] = size[0].start_point();
        return xcnt + 1;
    }

    let orientation_agree =
        a0.plane().normal().dot(&a1.plane().normal()) > 0.0;

    println!("orientation_agree = {}", orientation_agree);

    if (little_end_match[0] - little_end_match[1]).abs() > 0.0 {
        if orientation_agree == (little_end_match[0] == 1.0) {
            println!("-> return 2 (both endpoints)");
            *out[xcnt as usize] = size[0].start_point();
            xcnt += 1;
            *out[xcnt as usize] = size[0].end_point();
            xcnt += 1;
            return xcnt;
        } else {
            println!("-> return 3");
            return 3;
        }
    } else {
        if size[0].start_point().distance(&size[0].end_point()) < abs_tol {
            println!("-> return 1 (collapsed)");
            *out[xcnt as usize] = size[0].start_point();
            return xcnt + 1;
        } else {
            println!("-> return 3");
            return 3;
        }
    }
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::boundingbox::BoundingBox;
    use nurbslib::core::geom::{Point3D, Vector3D};
    use nurbslib::core::intersection::{
        LineLineIntersectionType, LinePlaneIntersectionType, on_intersect_bbox_plane,
        on_intersect_by_line_line, on_intersect_line_line_type, on_intersect_line_plane_type,
        on_intersect_line_point_param01,
    };
    use nurbslib::core::plane::Plane;
```
```rust
    // -----------------------------
    // Line–Point Intersection Test
    // -----------------------------
    #[test]
    fn test_intersect_line_point() {
        let p0 = Point3D::new(0.0, 0.0, 0.0);
        let dir = Vector3D::new(1.0, 0.0, 0.0);
        let test = Point3D::new(5.0, 0.0, 0.0);

        let mut t = 0.0;
        let ok = on_intersect_line_point_param01(p0, dir, test, &mut t);

        assert!(ok);
        assert!((t - 5.0).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // Line–Line Intersection Test
    // -----------------------------
    #[test]
    fn test_intersect_line_line() {
        let p0 = Point3D::new(0.0, 0.0, 0.0);
        let t0 = Vector3D::new(1.0, 0.0, 0.0);

        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let t1 = Vector3D::new(0.0, -1.0, 0.0);

        let mut s = 0.0;
        let mut t = 0.0;
        let mut i = Point3D::origin();

        let code = on_intersect_by_line_line(&p0, &t0, &p1, &t1, &mut s, &mut t, &mut i);

        assert_eq!(code, 0);
        assert!((s - 0.0).abs() < 1e-12);
        assert!((t - 1.0).abs() < 1e-12);
        assert!(i.is_nearly_equal(&Point3D::new(0.0, 0.0, 0.0), 1e-12));
    }
```
```rust
    // -------------------------------------
    // Line–Line Intersection Type Test
    // -------------------------------------
    #[test]
    fn test_intersect_line_line_type() {
        let p0 = Point3D::new(0.0, 0.0, 0.0);
        let t0 = Vector3D::new(1.0, 0.0, 0.0);

        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let t1 = Vector3D::new(0.0, -1.0, 0.0);

        let mut s = 0.0;
        let mut t = 0.0;
        let mut i = Point3D::origin();

        let ty = on_intersect_line_line_type(p0, t0, p1, t1, &mut s, &mut t, &mut i);

        assert_eq!(ty, LineLineIntersectionType::Intersecting);
        assert!(i.is_nearly_equal(&Point3D::new(0.0, 0.0, 0.0), 1e-12));
    }
```
```rust
    // -----------------------------
    // Line–Plane Intersection Test
    // -----------------------------
    #[test]
    fn test_intersect_line_plane() {
        let plane =
            Plane::from_origin_normal(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0))
                .unwrap();

        let p0 = Point3D::new(0.0, 0.0, 5.0);
        let dir = Vector3D::new(0.0, 0.0, -1.0);

        let mut out = Point3D::origin();

        let ty = on_intersect_line_plane_type(plane.z_axis, plane.origin, p0, dir, &mut out);

        assert_eq!(ty, LinePlaneIntersectionType::Intersecting);
        assert!(out.is_nearly_equal(&Point3D::new(0.0, 0.0, 0.0), 1e-12));
    }
```
```rust
    // -----------------------------
    // BoundingBox–Plane Intersection
    // -----------------------------
    #[test]
    fn test_intersect_bbox_plane() {
        let bbox = BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0));

        let plane =
            Plane::from_origin_normal(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0))
                .unwrap();

        let ok = on_intersect_bbox_plane(&bbox, &plane);

        assert!(ok);
    }
}

```
```rust
#[cfg(test)]
mod tests_intersect_2d_line_circle {
    use nurbslib::core::math_extensions::on_intersect_2d_line_circle_mut;
    // helper to call the function more easily
    fn call(
        p0: (f64, f64),
        p1: (f64, f64),
        r: f64,
        tol: f64,
    ) -> (i32, f64, f64) {
        let mut t0 = 0.0;
        let mut t1 = 0.0;
        let cnt = on_intersect_2d_line_circle_mut(p0, p1, r, tol, &mut t0, &mut t1);
        (cnt, t0, t1)
    }
```
```rust
    #[test]
    fn test_two_intersections() {
        // line: (-10,0) → (10,0)
        // circle: radius 5
        let (cnt, t0, t1) = call((-10.0, 0.0), (10.0, 0.0), 5.0, 1e-12);

        assert_eq!(cnt, 2);

        // expected t0 = 0.25, t1 = 0.75
        assert!((t0 - 0.25).abs() < 1e-9);
        assert!((t1 - 0.75).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_tangent_intersection() {
        // line: (-10,5) → (10,5)
        // circle: radius 5
        let (cnt, t0, t1) = call((-10.0, 5.0), (10.0, 5.0), 5.0, 1e-12);

        assert_eq!(cnt, 1);
        assert!((t0 - 0.5).abs() < 1e-9);
        assert!((t1 - 0.5).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_no_intersection() {
        // line: (-10,6) → (10,6)
        // circle: radius 5
        let (cnt, _, _) = call((-10.0, 6.0), (10.0, 6.0), 5.0, 1e-12);

        assert_eq!(cnt, 0);
    }
```
```rust
    #[test]
    fn test_vertical_line_two_points() {
        // line: (0,-10) → (0,10)
        // circle: radius 5
        let (cnt, t0, t1) = call((0.0, -10.0), (0.0, 10.0), 5.0, 1e-12);

        assert_eq!(cnt, 2);

        // expected t0 = 0.25, t1 = 0.75
        assert!((t0 - 0.25).abs() < 1e-9);
        assert!((t1 - 0.75).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_near_tangent_tol() {
        // line: (-10, 5 + tiny) → (10, 5 + tiny)
        let eps = 1e-10;
        let (cnt, _, _) = call((-10.0, 5.0 + eps), (10.0, 5.0 + eps), 5.0, 1e-9);

        // depending on tol, could be 0 or 1
        assert!(cnt == 0 || cnt == 1);
    }
}
```
```rust
#[cfg(test)]
mod tests_intersect_line_circle {
    use nurbslib::core::arc::Arc;
    use nurbslib::core::circle::{Circle};
    use nurbslib::core::plane::Plane;
    use nurbslib::core::line::Line;
    use nurbslib::core::math_extensions::{intersect_arc_arc, on_intersect_circle_circle, on_intersect_line_arc, on_intersect_line_circle};
    use nurbslib::core::prelude::{Point3D, Vector3D};

    fn circle_xy(radius: f64) -> Circle {
        Circle::from_plane_radius(
            Plane::from_origin_normal(
                Point3D::new(0.0, 0.0, 0.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ).expect("Invalid Plane"),
            radius,
        )
    }
```
```rust
    #[test]
    fn test_two_intersections() {
        let circle = circle_xy(5.0);

        // Line: (-10,0,0) → (10,0,0)
        let line = Line::from_xyz(-10.0, 0.0, 0.0, 10.0, 0.0, 0.0);

        let mut cp0 = Point3D::origin();
        let mut cp1 = Point3D::origin();
        let mut t0 = 0.0;
        let mut t1 = 0.0;

        let rc = on_intersect_line_circle(
            &line,
            &circle,
            Some(&mut t0),
            &mut cp0,
            Some(&mut t1),
            &mut cp1,
        );

        assert_eq!(rc, 2);
        assert!((t0 - 0.25).abs() < 1e-9);
        assert!((t1 - 0.75).abs() < 1e-9);

        assert!((cp0.x.abs() - 5.0).abs() < 1e-9 || (cp0.x.abs() - 5.0).abs() < 1e-9);
        assert!(cp0.y.abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_tangent_intersection() {
        let circle = circle_xy(5.0);

        // Line: (-10,5,0) → (10,5,0)
        let line = Line::from_xyz(-10.0, 5.0, 0.0, 10.0, 5.0, 0.0);

        let mut cp0 = Point3D::origin();
        let mut cp1 = Point3D::origin();
        let mut t0 = 0.0;

        let rc = on_intersect_line_circle(
            &line,
            &circle,
            Some(&mut t0),
            &mut cp0,
            None,
            &mut cp1,
        );

        assert_eq!(rc, 1);
        assert!((t0 - 0.5).abs() < 1e-9);
        assert!((cp0.x - 0.0).abs() < 1e-9);
        assert!((cp0.y - 5.0).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_no_intersection() {
        let circle = circle_xy(5.0);

        // Line: (-10,6,0) → (10,6,0)
        let line = Line::from_xyz(-10.0, 6.0, 0.0, 10.0, 6.0, 0.0);

        let mut cp0 = Point3D::origin();
        let mut cp1 = Point3D::origin();

        let rc = on_intersect_line_circle(&line, &circle, None, &mut cp0, None, &mut cp1);

        assert_eq!(rc, 0);
    }
```
```rust
    #[test]
    fn test_3d_slanted_line() {
        let circle = circle_xy(5.0);

        // 3D line passing through circle at an angle
        let line = Line::from_xyz(-10.0, 0.0, -5.0, 10.0, 0.0, 5.0);

        let mut cp0 = Point3D::origin();
        let mut cp1 = Point3D::origin();
        let mut t0 = 0.0;
        let mut t1 = 0.0;

        let rc = on_intersect_line_circle(
            &line,
            &circle,
            Some(&mut t0),
            &mut cp0,
            Some(&mut t1),
            &mut cp1,
        );

        println!("rc {}, cp0 = {:?}, cp1 = {:?}", rc, cp0, cp1);

        assert_eq!(rc, 0);

        assert!(cp0.distance(&Point3D::new(5.0, 0.0, 0.0)) < 1e-6
            || cp1.distance(&Point3D::new(5.0, 0.0, 0.0)) < 1e-6);
    }
```
```rust
    #[test]
    fn test_vertical_line_through_circle_plane() {
        let circle = circle_xy(5.0);

        // Vertical line: (5,0,-10) → (5,0,10)
        let line = Line::from_xyz(5.0, 0.0, -10.0, 5.0, 0.0, 10.0);

        let mut cp0 = Point3D::origin();
        let mut cp1 = Point3D::origin();
        let mut t0 = 0.0;

        let rc = on_intersect_line_circle(
            &line,
            &circle,
            Some(&mut t0),
            &mut cp0,
            None,
            &mut cp1,
        );

        assert_eq!(rc, 1);
        assert!((cp0.x - 5.0).abs() < 1e-9);
        assert!(cp0.y.abs() < 1e-9);
    }
```
```rust
    #[test]
    fn test_near_tangent_tol() {
        let circle = circle_xy(5.0);

        let eps = 1e-10;
        let line = Line::from_xyz(-10.0, 5.0 + eps, 0.0, 10.0, 5.0 + eps, 0.0);

        let mut cp0 = Point3D::origin();
        let mut cp1 = Point3D::origin();

        let rc = on_intersect_line_circle(&line, &circle, None, &mut cp0, None, &mut cp1);

        assert!(rc == 0 || rc == 1);
    }
```
```rust
    #[test]
    fn test_intersect_line_arc_basic() {
        // Line: origin → +X 방향
        let line = Line::from_point_dir(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
        );

        // Circle: center (5,0,0), radius 2, normal +Z
        let circle = Circle::new(
            Plane::from_origin_normal(Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0)).unwrap(),
            Point3D::new(5.0, 0.0, 0.0),
            2.0,
        ).expect("Invalid Circle");

        // Arc: 0° ~ 180°
        let arc = Arc::from_circle_radius(circle, 0.0, std::f64::consts::PI).expect("Invalid Arc");

        let mut t0 = 0.0;
        let mut t1 = 0.0;
        let mut p0 = Point3D::origin();
        let mut p1 = Point3D::origin();

        let count = on_intersect_line_arc(
            &line,
            &arc,
            Some(&mut t0),
            &mut p0,
            Some(&mut t1),
            &mut p1,
        );

        println!("count = {}", count);
        println!("t0 = {}, p0 = {:?}", t0, p0);
        println!("t1 = {}, p1 = {:?}", t1, p1);

        assert!(count >= 0);
    }
```
```rust
    #[test]
    fn test_intersect_circle_circle_basic() {
        // Plane: XY plane
        let plane = Plane::from_origin_normal(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ).expect("Invalid Plane");

        // Circle 0: center (0,0,0), radius 5
        let c0 = Circle::new(
            plane.clone(),
            Point3D::new(0.0, 0.0, 0.0),
            5.0,
        ).expect("Invalid Circle");

        // Circle 1: center (8,0,0), radius 5
        let c1 = Circle::new(
            plane.clone(),
            Point3D::new(8.0, 0.0, 0.0),
            5.0,
        ).expect("Invalid Circle");

        // 결과 저장
        let mut p0 = Point3D::origin();
        let mut p1 = Point3D::origin();

        let count = on_intersect_circle_circle(
            &c0,
            &c1,
            &mut p0,
            &mut p1,
        );

        println!("count = {}", count);
        println!("p0 = {:?}", p0);
        println!("p1 = {:?}", p1);

        // 두 점에서 교차해야 한다
        assert_eq!(count, 2);
    }
```
```rust
    #[test]
    fn test_intersect_arc_arc_basic() {
        // Plane: XY plane
        let plane = Plane::from_origin_normal(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ).expect("Invalid Plane");

        // Arc 0: center (0,0,0), radius 5, angle 0 ~ π
        let circle0 = Circle::new(
            plane.clone(),
            Point3D::new(0.0, 0.0, 0.0),
            5.0,
        ).expect("Invalid Circle");

        let arc0 = Arc::from_circle_radius(
            circle0,
            0.0,
            std::f64::consts::PI,
        ).expect("Invalid Arc");

        // Arc 1: center (8,0,0), radius 5, angle π ~ 2π
        let circle1 = Circle::new(
            plane.clone(),
            Point3D::new(8.0, 0.0, 0.0),
            5.0,
        ).expect("Invalid Circle");

        let arc1 = Arc::from_circle_radius(
            circle1,
            std::f64::consts::PI,
            std::f64::consts::TAU,
        ).expect("Invalid Arc");

        // 결과 저장
        let mut p0 = Point3D::origin();
        let mut p1 = Point3D::origin();

        let count = intersect_arc_arc(
            &arc0,
            &arc1,
            &mut p0,
            &mut p1,
        );

        println!("count = {}", count);
        println!("p0 = {:?}", p0);
        println!("p1 = {:?}", p1);

        assert!(count >= 0);
    }
}
```
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::intersections::on_intersect_line_sphere;
    use nurbslib::core::line::Line;
    use nurbslib::core::prelude::{Point3D, Vector3D};
    use nurbslib::core::sphere::Sphere;

    #[test]
    fn test_no_intersection() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0);
        let line = Line::from_point_dir(Point3D::new(0.0, 0.0, 5.0), Vector3D::new(1.0, 0.0, 0.0));

        let (rc, a, b) = on_intersect_line_sphere(&line, &sphere);

        assert_eq!(rc, 0);
        assert!(a.is_valid());
        assert!(b.is_valid());
    }
```
```rust
    #[test]
    fn test_tangent_intersection() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0);
        let line = Line::from_point_dir(Point3D::new(1.0, -5.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));

        let (rc, a, b) = on_intersect_line_sphere(&line, &sphere);

        assert_eq!(rc, 1);
        assert!((a.distance(&Point3D::new(1.0, 0.0, 0.0))) < 1e-6);
        assert!((b.distance(&Point3D::new(1.0, 0.0, 0.0))) < 1e-6);
    }
```
```rust
    #[test]
    fn test_two_intersections() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0);
        let line = Line::from_point_dir(Point3D::new(-2.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));

        let (rc, a, b) = on_intersect_line_sphere(&line, &sphere);

        assert_eq!(rc, 2);

        // 두 점은 x = -1, x = +1
        assert!((a.x.abs() - 1.0).abs() < 1e-6);
        assert!((b.x.abs() - 1.0).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_line_through_center() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0);
        let line = Line::new(Point3D::new(-5.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));

        let (rc, a, b) = on_intersect_line_sphere(&line, &sphere);

        assert_eq!(rc, 2);
        assert!((a.x.abs() - 2.0).abs() < 1e-6);
        assert!((b.x.abs() - 2.0).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_diagonal_line() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0);
        let line =
            Line::from_point_dir(Point3D::new(-2.0, -2.0, 0.0), Vector3D::new(1.0, 1.0, 0.0));

        let (rc, a, b) = on_intersect_line_sphere(&line, &sphere);

        assert_eq!(rc, 2);
        assert!(a.is_valid());
        assert!(b.is_valid());
    }
}
```
```rust
#[cfg(test)]
mod tests_line_circle {
    use nurbslib::core::circle::Circle;
    use nurbslib::core::geom::Point2D;
    use nurbslib::core::line::Line;
    use nurbslib::core::math_extensions::on_intersect_2d_line_circle;
    use nurbslib::core::plane::Plane;
    use nurbslib::core::prelude::Point3D;
```
```rust
    #[test]
    fn test_line_circle_two_points() {
        // 원: 중심 (0,0,0), 반지름 5
        let circle = Circle::new(Plane::world_xy(), Point3D::new(0.0, 0.0, 0.0), 5.0);

        // 선: (-10,0) → (10,0)
        let line = Line::from_xy(-10.0, 0.0, 10.0, 0.0);

        let (cnt, t0, t1) = on_intersect_2d_line_circle(
            Point2D::new(-10.0, 0.0),
            Point2D::new(10.0, 0.0),
            5.0,
            1e-12,
        );

        assert_eq!(cnt, 2);
        assert!((t0 - 0.25).abs() < 1e-6);
        assert!((t1 - 0.75).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_line_circle_tangent() {
        let circle = Circle::new(Plane::world_xy(), Point3D::new(0.0, 0.0, 0.0), 5.0);
        let line = Line::from_xy(-10.0, 5.0, 10.0, 5.0);

        let (cnt, t0, t1) = on_intersect_2d_line_circle(
            Point2D::new(-10.0, 5.0),
            Point2D::new(10.0, 5.0),
            5.0,
            1e-12,
        );

        assert_eq!(cnt, 1);
        assert!((t0 - 0.5).abs() < 1e-6);
        assert!((t1 - 0.5).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_line_circle_no_intersection() {
        let (cnt, t0, t1) = on_intersect_2d_line_circle(
            Point2D::new(-10.0, 6.0),
            Point2D::new(10.0, 6.0),
            5.0,
            1e-12,
        );

        assert_eq!(cnt, 3); // 교점 없음 + closest point
        assert!((t0 - 0.5).abs() < 1e-12);
        assert!((t1 - 0.5).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_line_cylinder {
    use nurbslib::core::arc::Arc;
    use nurbslib::core::circle::Circle;
    use nurbslib::core::cylinder::Cylinder;
    use nurbslib::core::line::Line;
    use nurbslib::core::math_extensions::{intersect_plane_arc, on_intersect_line_cylinder};
    use nurbslib::core::plane::Plane;
    use nurbslib::core::prelude::{Point3D, Vector3D};
```
```rust
    #[test]
    fn test_line_cylinder_two_points() {
        // 원기둥: 중심 (0,0,0), 반지름 5, 높이 0~10
        let circle = Circle::new(Plane::world_xy(), Point3D::new(0.0, 0.0, 0.0), 5.0);
        let cyl = Cylinder::new(circle.unwrap(), 0.0, 10.0);

        // 선: (-10,0,5) → (10,0,5)  (원기둥을 수평으로 관통)
        let line = Line::from_xyz(-10.0, 0.0, 5.0, 10.0, 0.0, 5.0);

        let (rc, a, b) = on_intersect_line_cylinder(&line, &cyl);

        assert_eq!(rc, 2);
        assert!((a.x.abs() - 5.0).abs() < 1e-6);
        assert!((b.x.abs() - 5.0).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_line_cylinder_tangent() {
        let circle = Circle::new(Plane::world_xy(), Point3D::new(0.0, 0.0, 0.0), 5.0);
        let cyl = Cylinder::new(circle.unwrap(), 0.0, 10.0);

        // 선: (-10,5,5) → (10,5,5)  (원기둥에 접함)
        let line = Line::from_xyz(-10.0, 5.0, 5.0, 10.0, 5.0, 5.0);

        let (rc, a, b) = on_intersect_line_cylinder(&line, &cyl);

        println!("{}, {}, {}", rc, a, b);

        assert_eq!(rc, 1);
        assert!((a.y - 5.0).abs() < 1e-6);
        assert!((b.y - 5.0).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_line_cylinder_no_intersection() {
        let circle = Circle::new(Plane::world_xy(), Point3D::new(0.0, 0.0, 0.0), 5.0);
        let cyl = Cylinder::new(circle.unwrap(), 0.0, 10.0);

        // 선이 원기둥을 완전히 벗어남
        let line = Line::from_xyz(-10.0, 6.0, 5.0, 10.0, 6.0, 5.0);

        let (rc, _, _) = on_intersect_line_cylinder(&line, &cyl);

        assert_eq!(rc, 0);
    }
```
```rust
    #[test]
    fn test_intersect_plane_arc_basic() {
        // Plane: XY plane (z = 0)
        let plane = Plane::from_origin_normal(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ).expect("Invalid Plane");

        // Circle: center (0,0,5), radius 3, normal +Z
        let circle = Circle::new(
            plane.clone(),
            Point3D::new(0.0, 0.0, 5.0),
            3.0,
        ).expect("Invalid Circle");

        // Arc: 0° ~ 360° (full circle)
        let arc = Arc::from_circle_radius(
            circle,
            0.0,
            std::f64::consts::TAU,
        ).expect("Invalid Arc");

        let mut p0 = Point3D::origin();
        let mut p1 = Point3D::origin();

        let count = intersect_plane_arc(
            &plane,
            &arc,
            &mut p0,
            &mut p1,
        );

        println!("count = {}", count);
        println!("p0 = {:?}", p0);
        println!("p1 = {:?}", p1);

        assert!(count >= 0);
    }
```
```rust
    #[test]
    fn test_vector_point_operation1()
    {

        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Vector3D::new(4.0, 5.0, 6.0);

        let r = &p + &v;   // OK!

        println!("{:?}, {:?}", r, p);

    }
```
```rust
    #[test]
    fn test_vector_point_operation2()
    {

        let p = &Point3D::new(1.0, 2.0, 3.0);
        let v = &Vector3D::new(4.0, 5.0, 6.0);

        let r = p + v;   // OK!

        println!("{:?}, {:?}", r, p);

    }
```
```rust
    #[test]
    fn test_vector_point_operation3()
    {
        let p = &Point3D::new(1.0, 2.0, 3.0);
        let v = &Point3D::new(4.0, 5.0, 6.0);

        let r = p - v;   // OK!

        println!("{:?}, {:?}", r, p);
    }
```
```rust
    #[test]
    fn test_vector_point_operation4()
    {

        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Point3D::new(4.0, 5.0, 6.0);

        let r = &p - &v;   // OK!

        println!("{:?}, {:?}", r, p);

    }
}
```

---




