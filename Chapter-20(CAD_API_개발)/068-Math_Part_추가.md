# Math 파트 3
미분기하학과 수치해석에서 매우 정당한 원리에 기반하고 있으며, 각 함수는 수학적으로 타당한 방식으로 정의되어 있습니다.  
특히 야코비안, 법선 도함수, 곡률, 보간, 중복 제거 등은 모두 검증된 수치 기법입니다.  
아래에 주요 수식과 함수들을 범주별로 정리하고, 각각의 수학적 의미와 검증을 설명.  

## 🧮 1. 거리 및 중복 제거
### 🔹 on_distance_square(a, b)
- 계산:
  
$$
\sqrt{(x_a-x_b)^2+(y_a-y_b)^2+(z_a-z_b)^2}
$$

- 검증: 유클리드 거리 공식이며, 정확함
### 🔹 on_remove_duplicate_points
- 원리: 인접 점 간 거리 누적 → 상대/절대 오차 기준으로 중복 제거
- 검증: 거리 기반 필터링은 CAD/CG에서 널리 사용됨

## 📐 2. 베지어 곡선 관련
### 🔹 on_bernstein_f64(i, n, u)
- 계산: 베르스타인 기저함수 $B_i^n(u)$
- 검증: 재귀적 디캐슬조 알고리즘과 동일하며, 수치적으로 안정적
### 🔹 on_solve_bezier_t_from_alpha
- 목적: 주어진 $α$ 에 대해 $t$ 역추정
- 방법: 뉴턴-랩슨 방식으로 반복 근사
- 검증: 수렴 조건과 오차 제어가 포함되어 있어 안정적

## 🧭 3. 평면과 삼각형 절단
### 🔹 tri_plane_cut
- 원리: 평면 방정식 $ax+by+cz+d=0$ 에 대해 삼각형의 절단
- 교차점 계산: 선형 보간

$$
r=\frac{f_j}{a\cdot dx+b\cdot dy+c\cdot dz}
$$

- 검증: 절단 알고리즘은 Constructive Solid Geometry(CSG)에서 표준

## 🧠 4. 야코비안 및 법선 도함수
### 🔹 on_ev_jacobian(E, F, G)
- 계산:
  
$$
\det J=EG-F^2
$$

- 검증: 표면의 면적 요소로서 야코비안은 정확한 정의
### 🔹 on_ev_normal_partials(ds, dt, dss, dst, dtt)
- 계산: 단위 법선 $N=\frac{ds\times dt}{|ds\times dt|}$ 의 도함수
- 검증: 벡터 정규화의 도함수 공식에 기반

## 🧭 5. 곡률 및 접선
### 🔹 on_ev_tangent(d1, d2)
- 정상:

$$
  T=\frac{d1}{|d1|}
$$

- 퇴화:

$$
T\approx \pm \frac{d2}{|d2|}
$$

- 검증: 병렬 벡터의 극한 근사로서 L'Hôpital의 원리 적용
### 🔹 on_ev_curvature(d1, d2)
- 계산:

$$
K=\frac{d2-(d2\cdot T)T}{|d1|^2}
$$

- 검증: 곡률 벡터의 표준 정의이며, 정당함

## 🔄 6. 파라미터 언랩 및 공차
### 🔹 on_unwrap_around_center, on_unwrap_to_face_range
- 원리: 주기 함수의 연속성 보존을 위한 언랩
- 검증: 각도/위상 처리에서 널리 사용되는 방식
### 🔹 get_parameter_tolerance(t0, t1, t)
- 계산:

$$
dt=8\sqrt{\epsilon }(t1-t0)+\epsilon (|t0|+|t1|)
$$


- 검증: IEEE-754 기반 수치 안정성 고려

```rust
#[inline]
pub fn on_distance_square(a: &Point3D, b: &Point3D) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
```
```rust
pub fn on_remove_duplicate_points(
    points: &[Point3D],
    add_index: bool,
    rel_tol: f64,
    abs_tol: f64,
) -> (Vec<Point3D>, Option<Vec<usize>>) {
    let count = points.len();
    if count == 0 {
        return (Vec::new(), if add_index { Some(Vec::new()) } else { None });
    }
    if count == 1 {
        let mut np = Vec::with_capacity(1);
        np.push(points[0]);
        let idx = if add_index { Some(vec![0usize]) } else { None };
        return (np, idx);
    }

    // dist[i] = |points[i] - points[i+1]|
    let mut dist = vec![0.0f64; count - 1];
    let mut total_dist = 0.0f64;
    for i in 0..(count - 1) {
        let d = on_distance_square(&points[i], &points[i + 1]);
        dist[i] = d;
        total_dist += d;
    }

    let mut remove_tol = total_dist * rel_tol;
    if remove_tol < abs_tol {
        remove_tol = abs_tol;
    }

    // 결과 컨테이너 준비
    let mut new_points = Vec::with_capacity(count);
    let mut add_indices: Option<Vec<usize>> = if add_index {
        Some(Vec::with_capacity(count))
    } else {
        None
    };

    // 첫 점은 항상 넣음
    new_points.push(points[0]);
    if let Some(ref mut idxs) = add_indices {
        idxs.push(0);
    }

    for i in 0..(count - 1) {
        if dist[i] > remove_tol {
            for j in (i + 1)..(count - 1) {
                if dist[j] > remove_tol {
                    new_points.push(points[i + 1]);
                    if let Some(ref mut idxs) = add_indices {
                        idxs.push(i + 1);
                    }
                    break;
                }
            }
        }
    }

    let dist_first_last = on_distance_square(&points[0], &points[count - 1]);
    let push_last = if new_points.len() > 1 {
        true
    } else {
        // new_points.len()==1 인 경우, 첫-마지막이 tol보다 크면 마지막 추가
        dist_first_last > remove_tol
    };

    if push_last {
        new_points.push(points[count - 1]);
        if let Some(ref mut idxs) = add_indices {
            idxs.push(count - 1);
        }
    }

    (new_points, add_indices)
}
```
```rust
pub fn on_remove_duplicate_points_simple(points: &[Point3D], abs_tol: f64) -> Vec<Point3D> {
    let (np, _) = on_remove_duplicate_points(points, false, 0.0, abs_tol);
    np
}
```
```rust
pub fn on_remove_duplicate_points_with_tangents(
    points: &[Point3D],
    tangents: &[Vector3D],
    rel_tol: f64,
    abs_tol: f64,
) -> (Vec<Point3D>, Vec<Vector3D>) {
    assert_eq!(
        points.len(),
        tangents.len(),
        "points and tangents must have same length"
    );
    let (new_pts, idxs_opt) = on_remove_duplicate_points(points, true, rel_tol, abs_tol);
    let idxs = idxs_opt.expect("indices must exist when add_index=true");

    let mut new_tans = Vec::with_capacity(idxs.len());
    for &i in &idxs {
        new_tans.push(tangents[i]);
    }
    (new_pts, new_tans)
}
```
```rust
/// Bernstein basis function Bᵢⁿ(u)
pub fn on_bernstein_f64(i: usize, n: usize, u: f64) -> f64 {
    let mut coeffs = vec![0.0; n + 1];
    coeffs[n - i] = 1.0;
    let one_minus_u = 1.0 - u;
    for k in 1..=n {
        for j in (k..=n).rev() {
            coeffs[j] = one_minus_u * coeffs[j] + u * coeffs[j - 1];
        }
    }
    coeffs[n]
}
```
```rust
pub fn on_solve_bezier_t_from_alpha(alpha: f64) -> f64 {
    let mut t = 1.0 - alpha;
    let mut iteration = 0;

    while iteration < 8 {
        let b_sum = on_bernstein_f64(0, 3, t) + on_bernstein_f64(1, 3, t);
        let error = b_sum - alpha;

        if error.abs() < 1e-15 {
            break;
        }

        let derivative = on_bernstein_der(0, 3, t) + on_bernstein_der(1, 3, t);
        if derivative.abs() < 1e-30 {
            break;
        }

        let next_t = (t - error / derivative).clamp(0.0, 1.0);
        if (t - next_t).abs() < 1e-14 {
            break;
        }

        t = next_t;
        iteration += 1;
    }

    if t.is_nan() { 0.0 } else { t }
}
```
```rust
#[allow(unused)]
fn on_matrix_swap_at(a: &mut Matrix, i: i32, j: i32, k: i32) {
    let tmp = a.at(i, j).clone();
    *a.at_mut(i, j) = a.at(k, j).clone();
    *a.at_mut(k, j) = tmp;
}
```
```rust
#[allow(unused)]
fn on_swap_array_copy_at<T: Copy>(a: &mut [T], i: usize, j: usize) {
    let tmp = a[i];
    a[i] = a[j];
    a[j] = tmp;
}
```
```rust
#[allow(unused)]
fn on_swap_array_clone_at<T: Clone>(a: &mut [T], i: usize, j: usize) {
    let tmp = a[i].clone();
    a[i] = a[j].clone();
    a[j] = tmp;
}
```
```rust
// 선형 보간
#[inline]
pub fn on_interpolate_line(x: f64, y: f64, r: f64) -> f64 {
    x * r + y * (1.0 - r)
}
```
```rust
// 점 보간
#[inline]
pub fn on_interpolate_vertex(v1: Point3D, v2: Point3D, r: f64) -> Point3D {
    Point3D {
        x: on_interpolate_line(v1.x, v2.x, r),
        y: on_interpolate_line(v1.y, v2.y, r),
        z: on_interpolate_line(v1.z, v2.z, r),
    }
}
```
```rust
// 평면과의 교차 비율 계산
#[inline]
pub fn on_get_ratio(i: usize, j: usize, fres: &[f64; 3], eq: &[f64; 4], vert: &[Point3D; 3]) -> f64 {
    let dx = vert[j].x - vert[i].x;
    let dy = vert[j].y - vert[i].y;
    let dz = vert[j].z - vert[i].z;
    let denom = eq[0] * dx + eq[1] * dy + eq[2] * dz;
    if denom.abs() < 1e-8 {
        0.5 // fallback to midpoint if degenerate
    } else {
        fres[j] / denom
    }
}
```
```rust
// 평면과 삼각형 절단
pub fn tri_plane_cut(
    eq: [f64; 4],
    vert: [Point3D; 3],
    edge: &mut [Point3D; 2],
    vert_new: &mut [Point3D; 6],
    i_tri: &mut usize,
    i_edge: &mut usize,
) -> bool {
    *i_tri = 0;
    *i_edge = 0;

    let mut fres = [0.0; 3];
    for i in 0..3 {
        fres[i] = eq[0] * vert[i].x + eq[1] * vert[i].y + eq[2] * vert[i].z + eq[3];
    }

    let mut flags = 0u32;
    if fres[0] < 0.0 {
        flags |= 0x01;
    }
    if fres[1] < 0.0 {
        flags |= 0x02;
    }
    if fres[2] < 0.0 {
        flags |= 0x04;
    }

    if flags == 0 {
        return false;
    } else if flags == 0x07 {
        vert_new[0] = vert[0];
        vert_new[1] = vert[1];
        vert_new[2] = vert[2];
        *i_tri = 1;
        return true;
    }

    if (flags & 0x03 == 0x01) || (flags & 0x03 == 0x02) {
        flags |= 0x10;
    }
    if (flags & 0x06 == 0x02) || (flags & 0x06 == 0x04) {
        flags |= 0x20;
    }
    if (flags & 0x05 == 0x04) || (flags & 0x05 == 0x01) {
        flags |= 0x40;
    }

    let mut rvert = [Point3D::default(); 6];
    let mut ridx_edge = [0usize; 3];
    let mut ncnt = 0;
    let mut ncnt_edge = 0;

    if flags & 0x01 != 0 {
        rvert[ncnt] = vert[0];
        ncnt += 1;
    }

    if flags & 0x10 != 0 {
        let r = on_get_ratio(0, 1, &fres, &eq, &vert);
        rvert[ncnt] = on_interpolate_vertex(vert[0], vert[1], r);
        ridx_edge[ncnt_edge] = ncnt;
        ncnt_edge += 1;
        ncnt += 1;
    }

    if flags & 0x02 != 0 {
        rvert[ncnt] = vert[1];
        ncnt += 1;
    }

    if flags & 0x20 != 0 {
        let r = on_get_ratio(1, 2, &fres, &eq, &vert);
        rvert[ncnt] = on_interpolate_vertex(vert[1], vert[2], r);
        ridx_edge[ncnt_edge] = ncnt;
        ncnt_edge += 1;
        ncnt += 1;
    }

    if flags & 0x04 != 0 {
        rvert[ncnt] = vert[2];
        ncnt += 1;
    }

    if flags & 0x40 != 0 {
        let r = on_get_ratio(2, 0, &fres, &eq, &vert);
        rvert[ncnt] = on_interpolate_vertex(vert[2], vert[0], r);
        ridx_edge[ncnt_edge] = ncnt;
        ncnt_edge += 1;
        ncnt += 1;
    }

    if ncnt == 3 {
        vert_new[0] = rvert[0];
        vert_new[1] = rvert[1];
        vert_new[2] = rvert[2];
        *i_tri = 1;
    } else if ncnt == 4 {
        vert_new[0] = rvert[0];
        vert_new[1] = rvert[1];
        vert_new[2] = rvert[2];
        vert_new[3] = rvert[0];
        vert_new[4] = rvert[2];
        vert_new[5] = rvert[3];
        *i_tri = 2;
    }

    if ncnt_edge == 2 {
        edge[0] = rvert[ridx_edge[0]];
        edge[1] = rvert[ridx_edge[1]];
        *i_edge = 1;
    }

    true
}
```
```rust
#[inline]
pub fn tri_center_3d(tri: &[[f64; 3]]) -> [f64; 3] {
    [
        (tri[0][0] + tri[1][0] + tri[2][0]) / 3.0,
        (tri[0][1] + tri[1][1] + tri[2][1]) / 3.0,
        (tri[0][2] + tri[1][2] + tri[2][2]) / 3.0,
    ]
}
```
```rust
pub fn tri_center_point_3d(tri: &[Point3D]) -> Point3D {
    Point3D {
        x: (tri[0].x + tri[1].x + tri[2].x) / 3.0,
        y: (tri[0].y + tri[1].y + tri[2].y) / 3.0,
        z: (tri[0].z + tri[1].z + tri[2].z) / 3.0,
    }
}
```
```rust
#[allow(unused)]
const AXIS_PROJ: [[usize; 3]; 6] = [
    [2, 1, 0], // -X  → (x',y',z') = (z,y,x)
    [0, 2, 1], // -Y
    [1, 0, 2], // -Z
    [1, 2, 0], // +X
    [2, 0, 1], // +Y
    [0, 1, 2], // +Z
];
```
```rust
#[allow(unused)]
const AXIS_PROJ_INV: [[usize; 3]; 6] = AXIS_PROJ;
```
```rust
#[allow(unused)]
#[inline]
fn proj2d(dir: usize, v: &[f64; 3]) -> [f64; 3] {
    let m = AXIS_PROJ[dir];
    [v[m[0]], v[m[1]], v[m[2]]]
}
```
```rust
//Hash Index
pub const HASH2_X: i64 = 30;
pub const HASH2_Y: i64 = 30;
pub const HASH2_SIZE: i64 = HASH2_X * HASH2_Y;
```
```rust
#[inline]
fn hash2_axial_index_from_pos(pos: f64, start: f64, divide: i64, size: f64) -> i64 {
    (((pos - start) * ((divide - 1) as f64) / size).floor()) as i64
}
```
```rust
#[allow(unused)]
#[inline]
fn hash2_one_index(ix: i64, iy: i64) -> i64 {
    ix + iy * HASH2_X
}
```
```rust
#[allow(unused)]
#[inline]
fn hash2_ix(idx: i64) -> i64 {
    idx % HASH2_X
}
```
```rust
#[allow(unused)]
#[inline]
fn hash2_iy(idx: i64) -> i64 {
    idx / HASH2_X
}
```
```rust

pub const HASH3_X: i64 = 25;
pub const HASH3_Y: i64 = 25;
pub const HASH3_Z: i64 = 25;
pub const HASH3_SIZE: i64 = HASH3_X * HASH3_Y * HASH3_Z;
```
```rust
#[allow(unused)]
#[inline]
fn hash3_axial_index_from_pos(pos: f64, start: f64, divide: i64, size: f64) -> i64 {
    (((pos - start) * ((divide - 1) as f64) / size).floor()) as i64
}
```
```rust
#[allow(unused)]
#[inline]
fn hash3_one_index(ix: i64, iy: i64, iz: i64) -> i64 {
    ix + iy * HASH3_X + iz * HASH3_X * HASH3_Y
}
```
```rust
#[allow(unused)]
#[inline]
fn hash3_ix(idx: i64) -> i64 {
    idx % HASH3_X
}
```
```rust
#[allow(unused)]
#[inline]
fn hash3_iy(idx: i64) -> i64 {
    (idx % (HASH3_X * HASH3_Z)) / HASH3_X
}
```
```rust
#[allow(unused)]
#[inline]
fn hash3_iz(idx: i64) -> i64 {
    idx / (HASH3_X * HASH3_Y)
}
```
```rust
#[allow(unused)]
#[inline]
fn inv_proj2d(dir: usize, v: &[f64; 3]) -> [f64; 3] {
    let m = AXIS_PROJ_INV[dir];
    [v[m[0]], v[m[1]], v[m[2]]]
}
```
```rust
#[allow(unused)]
#[inline]
fn bucket_bounds(tri2d: &[[f64; 3]; 3], minv: [f64; 3], maxv: [f64; 3]) -> (i64, i64, i64, i64) {
    let (mut minx, mut miny) = (f64::INFINITY, f64::INFINITY);
    let (mut maxx, mut maxy) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for v in tri2d {
        minx = minx.min(v[0]);
        miny = miny.min(v[1]);
        maxx = maxx.max(v[0]);
        maxy = maxy.max(v[1]);
    }
    let sx = maxv[0] - minv[0];
    let sy = maxv[1] - minv[1];
    let mut ix0 = hash2_axial_index_from_pos(minx, minv[0], HASH2_X, sx);
    let mut ix1 = hash2_axial_index_from_pos(maxx, minv[0], HASH2_X, sx);
    let mut iy0 = hash2_axial_index_from_pos(miny, minv[1], HASH2_Y, sy);
    let mut iy1 = hash2_axial_index_from_pos(maxy, minv[1], HASH2_Y, sy);
    if ix0 < 0 {
        ix0 = 0;
    }
    if iy0 < 0 {
        iy0 = 0;
    }
    if ix1 >= HASH2_X {
        ix1 = HASH2_X - 1;
    }
    if iy1 >= HASH2_Y {
        iy1 = HASH2_Y - 1;
    }
    (ix0, ix1, iy0, iy1)
}
```
```rust
#[allow(unused)]
#[inline]
fn bucket_bounds_point3d(
    tri2d: &[Point3D; 3],
    minv: &Point3D,
    maxv: &Point3D,
) -> (i64, i64, i64, i64) {
    let (mut minx, mut miny) = (f64::INFINITY, f64::INFINITY);
    let (mut maxx, mut maxy) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for v in tri2d {
        minx = minx.min(v[0]);
        miny = miny.min(v[1]);
        maxx = maxx.max(v[0]);
        maxy = maxy.max(v[1]);
    }
    let sx = maxv[0] - minv[0];
    let sy = maxv[1] - minv[1];
    let mut ix0 = hash2_axial_index_from_pos(minx, minv[0], HASH2_X, sx);
    let mut ix1 = hash2_axial_index_from_pos(maxx, minv[0], HASH2_X, sx);
    let mut iy0 = hash2_axial_index_from_pos(miny, minv[1], HASH2_Y, sy);
    let mut iy1 = hash2_axial_index_from_pos(maxy, minv[1], HASH2_Y, sy);
    if ix0 < 0 {
        ix0 = 0;
    }
    if iy0 < 0 {
        iy0 = 0;
    }
    if ix1 >= HASH2_X {
        ix1 = HASH2_X - 1;
    }
    if iy1 >= HASH2_Y {
        iy1 = HASH2_Y - 1;
    }
    (ix0, ix1, iy0, iy1)
}
```
```rust
#[allow(unused)]
fn on_are_coincident_vec2d(u: &Vector2D, v: &Vector2D, tol: f64) -> bool {
    (1.0 - Vector2D::dot(u, v)).abs() < tol
}
```
```rust
#[allow(unused)]
fn cmp_f64_key(a: &f64, b: &f64) -> Ordering {
    let da = *a;
    let db = *b;
    let d = (da - db).abs();
    if d <= 1e-12 * (1.0 + da.abs().max(db.abs())) {
        Ordering::Equal
    } else if da < db {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
```
```rust
pub fn on_are_coincident_vec3d(u: &Vector3D, v: &Vector3D, tol: f64) -> bool {
    (1.0 - Vector3D::dot(u, v)).abs() < tol
}
```
```rust
#[allow(unused)]
fn on_are_coincident_point3d(u: &Point3D, v: &Point3D, tol: f64) -> bool {
    (1.0 - Point3D::dot(u, v)).abs() < tol
}
```
```rust
#[allow(unused)]
fn on_are_coincident_point2d(u: &Point2D, v: &Point2D, tol: f64) -> bool {
    (1.0 - Point2D::dot(u, v)).abs() < tol
}
```
```rust
#[inline]
#[allow(unused)]
fn on_signed_area_point2d(poly: &[Point2D]) -> f64 {
    let n = poly.len();
    if n < 3 {
        return 0.0;
    }
    let mut s = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        s += poly[i].x * poly[j].y - poly[j].x * poly[i].y;
    }
    0.5 * s
}
```
```rust
#[inline]
#[allow(unused)]
fn on_any_perp_vec3d(v: &Vector3D) -> Vector3D {
    // Construct a vertical vector avoiding the maximum component
    let mut a = if v.x.abs() <= v.y.abs() && v.x.abs() <= v.z.abs() {
        Vector3D {
            x: 0.0,
            y: -v.z,
            z: v.y,
        }
    } else if v.y.abs() <= v.x.abs() && v.y.abs() <= v.z.abs() {
        Vector3D {
            x: -v.z,
            y: 0.0,
            z: v.x,
        }
    } else {
        Vector3D {
            x: -v.y,
            y: v.x,
            z: 0.0,
        }
    };
    if !a.normalize() {
        // Fallback when extremely close to 0
        a = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }
    a
}
```
```rust
//----------------------------------------------
// 11) Solve Angular Projection Equation
//----------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleSolveResult {
    NoSolution,
    OutOfRange,
    SingleSolution,
    TwoSolutions,
}
```
```rust
pub fn on_solve_angle_projection(
    vec_x: f64,
    vec_y: f64,
    target: f64,
    tol: f64,
) -> (AngleSolveResult, [f64; 2]) {
    let mut res = [0.0, PI / 2.0];
    let vlen = (vec_x * vec_x + vec_y * vec_y).sqrt();
    if vlen < 1e-20 {
        return (
            if target.abs() >= tol {
                AngleSolveResult::OutOfRange
            } else {
                AngleSolveResult::NoSolution
            },
            res,
        );
    }
    if (target - vlen).abs() < tol && (target + vlen).abs() < tol {
        return (AngleSolveResult::NoSolution, res);
    }
    let ratio = target / vlen;
    if ratio.abs() > 1.0 + tol / vlen {
        return (AngleSolveResult::OutOfRange, res);
    }
    res[0] = on_safe_acos(ratio);
    res[1] = 2.0 * PI - res[0];

    let vangle = on_safe_atan2_positive_angle(vec_x / vlen, vec_y / vlen);
    res[0] = on_normalize_angle_to_two_pi(res[0] + vangle);
    res[1] = on_normalize_angle_to_two_pi(res[1] + vangle);

    let mid = 0.5 * (res[0] + res[1]);
    let check = vec_x * mid.cos() + vec_y * mid.sin() - target;
    if check.abs() < tol {
        res[0] = mid;
        res[1] = mid;
        (AngleSolveResult::SingleSolution, res)
    } else {
        if (res[0] - res[1]).abs() > 2.0 * PI {
            (AngleSolveResult::SingleSolution, res)
        } else {
            (AngleSolveResult::TwoSolutions, res)
        }
    }
}
```
```rust
//----------------------------------------------
// 13) Fast Min/Max Update for Boundary Correction & AABB Calculation
//----------------------------------------------
#[inline]
pub fn on_update_minmax_quick_2d(x: f64, y: f64, min: &mut Point2D, max: &mut Point2D) {
    if x < min.x {
        min.x = x;
    } else if x > max.x {
        max.x = x;
    }
    if y < min.y {
        min.y = y;
    } else if y > max.y {
        max.y = y;
    }
}
```
```rust
#[inline]
pub fn on_update_minmax_quick_xyz(x: f64, y: f64, z: f64, min: &mut Point3D, max: &mut Point3D) {
    if x < min.x {
        min.x = x;
    } else if x > max.x {
        max.x = x;
    }
    if y < min.y {
        min.y = y;
    } else if y > max.y {
        max.y = y;
    }
    if z < min.z {
        min.z = z;
    } else if z > max.z {
        max.z = z;
    }
}
```
```rust
#[inline]
pub fn on_update_minmax_quick_point(p: Point3D, min: &mut Point3D, max: &mut Point3D) {
    on_update_minmax_quick_xyz(p.x, p.y, p.z, min, max);
}
```
```rust
pub fn on_compute_bounding_box_xf(xf: &Transform, pts: &[Point3D]) -> Option<(Point3D, Point3D)> {
    if pts.is_empty() {
        return None;
    }
    let p0 = xf.apply_point(pts[0]);
    let mut minp = p0;
    let mut maxp = p0;
    for &p in &pts[1..] {
        let q = xf.apply_point(p);
        on_update_minmax_quick_point(q, &mut minp, &mut maxp);
    }
    Some((minp, maxp))
}
```
```rust
pub fn on_compute_bounding_box(pts: &[Point3D]) -> Option<(Point3D, Point3D)> {
    on_compute_bounding_box_xf(&Transform::identity(), pts)
}
```
```rust

//----------------------------------------------
// 14) Angle / Period Unwrapping (unwrap)
//----------------------------------------------
pub fn on_unwrap_around_center(u_list: &[f64], period: f64) -> Vec<f64> {
    if u_list.is_empty() {
        return vec![];
    }
    let mut out = Vec::with_capacity(u_list.len());
    out.push(u_list[0]);
    for i in 1..u_list.len() {
        let prev = *out.last().unwrap();
        let mut curr = u_list[i];
        let delta = curr - (prev % period);
        if delta < -period * 0.5 {
            curr += period;
        } else if delta > period * 0.5 {
            curr -= period;
        }
        let val = prev + (curr - (prev % period));
        out.push(val);
    }
    out
}
```
```rust
pub fn on_unwrap_to_face_range(u_list: &[f64], period: f64, umin: f64, umax: f64) -> Vec<f64> {
    let mut out = Vec::with_capacity(u_list.len());
    for mut u in u_list.iter().copied() {
        while u < umin {
            u += period;
        }
        while u > umax {
            u -= period;
        }
        let d1 = (u - umin).abs();
        let d2 = (u - umax).abs();
        if d1 > period * 0.5 && d2 > period * 0.5 {
            if u < umin {
                u += period;
            } else if u > umax {
                u -= period;
            }
        }
        out.push(u);
    }
    out
}
```
```rust

//----------------------------------------------
// 15) Distance Tolerance
//----------------------------------------------
#[inline]
pub fn on_dist_normalized_tol(x: f64, y: f64, length: f64, tol: f64) -> bool {
    ((x - y) / length).abs() < tol
}
```
```rust
#[inline]
pub fn on_dist_tol_point_segment(p: Point3D, seg: &Segment3D, tol: f64) -> bool {
    (p.distance_to_segment(seg)).abs() < tol
}
```
```rust
//----------------------------------------------
// 16) Check for Points Near Reference Direction
//----------------------------------------------
pub fn on_is_any_point_near_ref_direction(
    points: &[Point3D],
    dir: Vector3D,
    origin: Point3D,
    tol: f64,
) -> bool {
    let seg = Segment3D::new(origin, origin + dir.to_point());
    points
        .iter()
        .any(|&p| on_dist_tol_point_segment(p, &seg, tol))
}
```
```rust
pub fn on_is_any_point_near_ref_direction_on_plane(
    plane: &Plane,
    dir: Vector3D,
    origin: Point3D,
    uv_points: &[Point2D],
    tol: f64,
) -> bool {
    let seg = Segment3D::new(origin, origin + dir.to_point());
    if uv_points.is_empty() {
        return false;
    }
    let p0 = plane.point_at(uv_points[0].x, uv_points[0].y);
    let p1 = plane.point_at(
        uv_points[uv_points.len() - 1].x,
        uv_points[uv_points.len() - 1].y,
    );
    on_dist_tol_point_segment(p0, &seg, tol) && on_dist_tol_point_segment(p1, &seg, tol)
}
```
```rust
//----------------------------------------------
// 17) Attempt to Find Orthogonal Vector
//----------------------------------------------
pub fn on_try_get_perpendicular_vector(a: Vector3D, b: Vector3D) -> Option<Vector3D> {
    let dot = Vector3D::dot(&a, &b);
    let parallelish = (1.0 - dot.abs()) < ON_TOL6;
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
// 18) Add Point4D to List Without Duplicates (modify if InterPoint-like functionality is needed)
//   Recommended to implement separately based on your project's InterPoint definition
//----------------------------------------------
pub fn on_contains_value_with_tolerance(vec: &[f64], value: f64, tol: f64) -> bool {
    vec.iter().any(|&x| (x - value).abs() <= tol)
}
```
```rust
pub fn on_contains_any_value_with_tolerance(values: &[f64], target: f64, tol: f64) -> bool {
    values.iter().any(|&v| (v - target).abs() <= tol)
}
```
```rust
/// Computes the Jacobian determinant (det) while checking numerical reliability.
/// Inputs:
/// - ds_o_ds = |D_u|² (Du · Du) — squared magnitude of partial derivative w.r.t. u
/// - ds_o_dt = Du · Dv         — dot product of partial derivatives (cross term)
/// - dt_o_dt = |D_v|² (Dv · Dv) — squared magnitude of partial derivative w.r.t. v
///
/// Returns: (is_reliable, det)
/// - det = ds_o_ds * dt_o_dt - ds_o_dt * ds_o_dt = EG - F²
/// - If is_reliable is false, the det value is considered numerically unstable or suspicious.
pub fn on_ev_jacobian(ds_o_ds: f64, ds_o_dt: f64, dt_o_dt: f64) -> (bool, f64) {
    // Same meaning as constants used in the original code
    // a = E * G, b = F² (always ≥ 0)
    let a = ds_o_ds * dt_o_dt;
    let b = ds_o_dt * ds_o_dt;
    let det = a - b;

    // Default: Unreliable (by default, assume distrust)
    let reliable;

    // If one partial derivative is much smaller than the other (≈ 0), the determinant is unreliable
    if ds_o_ds <= dt_o_dt * ON_EPSILON || dt_o_dt <= ds_o_ds * ON_EPSILON {
        reliable = false;
    }
    // If Du and Dv are (numerically) nearly parallel or anti parallel, the determinant is unreliable
    else if det.abs() <= if a > b { a } else { b } * ON_SQRT_EPSILON {
        reliable = false;
    }
    // Otherwise, considered reliable
    else {
        reliable = true;
    }

    (reliable, det)
}
```
```rust
#[inline]
fn jacobian_reliable(ds: Vector3D, dt: Vector3D) -> bool {
    // v2 = |ds × dt|²  (squared area element)
    let v2 = ds.cross(&dt).length_squared();
    // Scale: e * g (e = |ds|², g = |dt|²). If too small, clamp to 1.0.
    let e = ds.dot(&ds);
    let g = dt.dot(&dt);
    let scale = (e * g).max(1.0);
    // 기준: v2가 scale * 1e-12 보다 크면 "신뢰 가능"
    v2 > scale * 1e-12
}
```
```rust
/* ====== 1) N_s, N_t : Partial Derivatives of the Normal in u,v Directions ====== */
/// ON_EvNormalPartials: Computes the u,v partial derivatives (ns, nt) of the unit normal
/// N = (ds × dt) / ||ds × dt|| using the given first and second derivatives (ds, dt, dss, dst, dtt).
/// Returns Some((ns, nt)) on success, or None if the Jacobian is degenerate.
///
/// Formula:
///   V = ds × dt,  N = V / |V|
///   N' = (V' / |V|) - ( (V · V') / |V|³ ) * V  (Derivative of a normalized vector)
pub fn on_ev_normal_partials(
    ds: Vector3D,
    dt: Vector3D,
    dss: Vector3D,
    dst: Vector3D,
    dtt: Vector3D,
) -> Option<(Vector3D, Vector3D)> {
    // Previous: on_ev_jacobian(...) → prone to excessive false negatives
    // Updated: reliability check based directly on cross product norm
    if !jacobian_reliable(ds, dt) {
        return None;
    }

    let v = ds.cross(&dt);
    let len = v.length();
    if len < 1e-15 {
        return None;
    }
    let len3 = len * len * len;

    // Vs = dss×dt + ds×dst, Vt = dst×dt + ds×dtt
    let vs = dss.cross(&dt) + ds.cross(&dst);
    let vt = dst.cross(&dt) + ds.cross(&dtt);

    // N' = Vs/|V| - ((V·Vs)/|V|^3) V  (V=ds×dt)
    let ns = (1.0 / len) * vs - (v.dot(&vs) / len3) * v;
    let nt = (1.0 / len) * vt - (v.dot(&vt) / len3) * v;

    Some((ns, nt))
}
```
```rust
/* ====== 2) Pullback: 3D 벡터를 (u,v) 좌표로 끌어오기 ====== */

/// ON_Pullback3dVector:
/// Computes (α, β) that pulls back a 3D vector `vector` into the surface parameter space.
/// (vector ≈ α * ds + β * dt; if an offset `distance` is provided, ds and dt are adjusted accordingly)
///
/// Formula:
///  - distance = 0:   vector ≈ α * ds + β * dt
///  - distance ≠ 0:   vector ≈ α * (ds + distance * ns) + β * (dt + distance * nt)
pub fn pullback_3d_vector(
    vector: Vector3D,
    distance: f64, // Du×Dv 기준으로 위(+) / 아래(-) 부호
    ds: Vector3D,
    dt: Vector3D,
    dss: Vector3D,
    dst: Vector3D,
    dtt: Vector3D,
) -> Option<(f64, f64)> {
    if distance != 0.0 {
        // ns, nt 계산 후 보정 기저로 분해
        let (ns, nt) = on_ev_normal_partials(ds, dt, dss, dst, dtt)?;
        let bs = ds + ns * distance;
        let bt = dt + nt * distance;
        on_decompose_vector(vector, bs, bt)
    } else {
        // 그냥 ds, dt로 분해
        on_decompose_vector(vector, ds, dt)
    }
}
```
```rust
/* ====== 3) 파라미터 공차 (t 주변 안전범위) ====== */

/// ON_GetParameterTolerance:
/// Returns the numerical tolerance [tminus, tplus] for parameter t over the interval [t0, t1].
/// (If the domain is valid (t0 < t1), returns Some((tminus, tplus)); otherwise, returns None)
///
/// Formula (original logic):
/// dt = 8√ε · (t1 - t0) + ε · (|t0| + |t1|), clamped to dt ≤ 0.5 · (t1 - t0)
pub fn get_parameter_tolerance(t0: f64, t1: f64, mut t: f64) -> Option<(f64, f64)> {
    if !(t0 < t1) {
        return None;
    }
    if t < t0 {
        t = t0;
    } else if t > t1 {
        t = t1;
    }

    let eps = f64::EPSILON;
    let sqrt_eps = eps.sqrt();
    let mut dt = (t1 - t0) * 8.0 * sqrt_eps + (t0.abs() + t1.abs()) * eps;
    if dt >= (t1 - t0) {
        dt = 0.5 * (t1 - t0);
    }

    Some((t - dt, t + dt))
}
```
```rust

/* ====== 4) 단위 법선 N 평가 (퇴화 시 극한 방향으로 근사) ====== */

/// ON_EvNormal:
/// Default: N ∝ Du × Dv. If the Jacobian (EG - F²) degenerates, approximate the normal direction
/// at the singularity using second-order partial derivatives.
/// limit_dir ∈ {1,2,3,4} selects the quadrant of approach in (u,v) space.
///
/// Formula (in degenerate case):
///   A = Du × Duv + Duu × Dv
///   B = Du × Dvv + Duv × Dv
///   a, b = directional signs (determined by limit_dir)
///   Direction vector ≈ a*A + b*B
pub fn on_ev_normal(
    limit_dir: i32,
    du: Vector3D,
    dv: Vector3D,
    duu: Vector3D,
    duv: Vector3D,
    dvv: Vector3D,
) -> Option<Vector3D> {
    // 정상 케이스: 바로 Du×Dv 사용
    if jacobian_reliable(du, dv) {
        let mut n = du.cross(&dv);
        if n.normalize() {
            return Some(n);
        }
        return None;
    }

    // Degenerate case: use second-order partial derivatives to approximate limiting direction
    if !(is_valid(&duu) && is_valid(&duv) && is_valid(&dvv)) {
        return None;
    }
    let (a, b) = match limit_dir {
        2 => (-1.0, 1.0),
        3 => (-1.0, -1.0),
        4 => (1.0, -1.0),
        _ => (1.0, 1.0),
    };

    // aA+bB = Du×(aDuv+bDvv) − Dv×(aDuu+bDuv)
    let v1 = du.cross(&(duv * a + dvv * b));
    let v2 = dv.cross(&(duu * a + duv * b));
    let mut n = v1 - v2;
    if n.normalize() { Some(n) } else { None }
}
```
```rust
#[inline]
fn is_valid(v: &Vector3D) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}
```
```rust
/* ====== 5) 단위 접선 ====== */

/// ON_EvTangent:
/// Normally, T = D1 / |D1|. If D1 = 0 and D2 ≠ 0, apply L'Hôpital's Rule to approximate T = ± D2 / |D2|.
pub fn on_ev_tangent(d1: Vector3D, d2: Vector3D) -> Option<Vector3D> {
    let mut l1 = d1.length();
    if l1 == 0.0 {
        l1 = d2.length();
        if l1 > 0.0 {
            let t = d2 / l1;
            // 부호는 s→0에서 D1·D2의 부호에 따르지만, 여기서는 방향만 반환
            return Some(t);
        } else {
            return None;
        }
    } else {
        let t = d1 / l1;
        return Some(t);
    }
}
```
```rust
/* ====== 6) Unit Tangent and Curvature Vector ====== */
/// ON_EvCurvature:
/// T = D1 / |D1|                      — unit tangent vector
/// K = (D2 − (D2 · T) * T) / (D1 · D1) — curvature vector
pub fn on_ev_curvature(d1: Vector3D, d2: Vector3D) -> Option<(Vector3D, Vector3D)> {
    let mut l1 = d1.length();
    if l1 == 0.0 {
        // If D1 = 0, apply L'Hôpital → T ≈ ± normalized D2, curvature K is 0 (undefined or ambiguous)
        l1 = d2.length();
        if l1 > 0.0 {
            let t = d2 / l1;
            let k = Vector3D::zero();
            return Some((t, k));
        }
        return None;
    }
    let t = d1 / l1;
    let d2_dot_t = d2.dot(&t);
    let k = (d2 - t * d2_dot_t) / (l1 * l1);
    Some((t, k))
}
```
```rust

/* ====== 7) First Derivative of Curvature and Torsion ====== */
/// ON_EvCurvature1Der:
/// T = D1 / |D1|                      — unit tangent vector
/// K = (D2 − (D2 · T) * T) / |D1|²     — curvature vector
/// k = |D1 × D2| / |D1|³              — scalar curvature magnitude
/// k' = d/ds k                        — derivative of curvature (see closed-form below)
/// torsion τ = ((D1 × D2) · D3) / |D1 × D2|² — torsion
pub struct Curvature1Der {
    pub t: Vector3D,
    pub k_vec: Vector3D,
    pub k_prime: Option<f64>,
    pub torsion: Option<f64>,
}
```
```rust

pub fn on_ev_curvature_1der(d1: Vector3D, d2: Vector3D, d3: Vector3D) -> Option<Curvature1Der> {
    let ds_dt = d1.length();
    if ds_dt <= 0.0 {
        return None;
    }

    let t = d1 / ds_dt;
    let q = d1.cross(&d2);
    let q_len2 = q.length_squared();
    let ds_dt2 = ds_dt * ds_dt;
    let k_vec = (d2 - t * d2.dot(&t)) / ds_dt2;

    // k' (first derivative of curvature)
    let kprime = {
        if q_len2 > 0.0 {
            let qprime = d1.cross(&d3);
            let num = (q.dot(&qprime)) * d1.length_squared() - 3.0 * q_len2 * d1.dot(&d2);
            let den = q_len2.sqrt() * d1.length().powf(5.0);
            Some(num / den)
        } else {
            // q=0 이면 k=0 상태 → 근사식
            let qprime = d1.cross(&d3);
            Some(qprime.length() / d1.length().powf(3.0))
        }
    };

    // 토션 τ
    let torsion = if q_len2 > 0.0 {
        Some(q.dot(&d3) / q_len2)
    } else {
        None
    };

    Some(Curvature1Der {
        t,
        k_vec,
        k_prime: kprime,
        torsion,
    })
}
```
```rust

/* ====== 8) Sectional Curvature (Curvature Vector of Surface-Plane Intersection) ====== */

/// ON_EvSectionalCurvature:
/// Computes the curvature vector K of the intersection curve formed by slicing the surface with a plane whose normal is `planeNormal`.
/// Notation: Su = S10, Sv = S01, Suu = S20, Suv = S11, Svv = S02
///
/// Procedure Overview:
/// 1) M = Su × Sv                      — numerator of the surface normal
/// 2) D1 = (Su × Sv) × planeNormal     — direction of the first derivative of the intersection curve
/// 3) Solve D1 = a * Su + b * Sv       — obtain (a, b) by solving a 3×2 system
/// 4) M1 = (a * Suu + b * Suv) × Sv + Su × (a * Suv + b * Svv)
/// 5) D2 = M1 × planeNormal
/// 6) K = (D2 − ((D2 · D1) / (D1 · D1)) * D1) / (D1 · D1)
pub fn on_ev_sectional_curvature(
    s10: Vector3D,
    s01: Vector3D,
    s20: Vector3D,
    s11: Vector3D,
    s02: Vector3D,
    plane_normal: Vector3D,
) -> Option<Vector3D> {
    // M = Su×Sv
    let m = s10.cross(&s01);

    // D1 = (Su×Sv)×planeNormal
    let d1 = m.cross(&plane_normal);

    // D1 = a*Su + b*Sv 풀기 (3×2)
    let (_rank, a, b, _err, _pr) = on_solve_3x2(s10, s01, d1.x, d1.y, d1.z);
    if _rank < 2 {
        return None;
    }

    // M1 = (a*Suu + b*Suv)×Sv + Su×(a*Suv + b*Svv)
    let left = (s20 * a + s11 * b).cross(&s01);
    let right = s10.cross(&(s11 * a + s02 * b));
    let m1 = left + right;

    // D2 = M1 × planeNormal
    let d2 = m1.cross(&plane_normal);

    // K = (D2 − proj_{D1}(D2)) / |D1|^2
    let d1_len2 = d1.length_squared();
    if d1_len2 <= f64::MIN_POSITIVE {
        return None;
    }
    let bcoef = -(d2.dot(&d1)) / d1_len2;
    let k = (d2 + d1 * bcoef) / d1_len2;
    Some(k)
}
```
```rust
/// ------------------------------
/// Enum for Continuity Classification
/// ------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Continuity {
    Unknown,
    C0,
    C0Locus,
    C1,
    C1Locus,
    G1,
    G1Locus,
    C2,
    C2Locus,
    CInfinity,
    G2,
    G2Locus,
    GSmooth,
}
```
```rust
// ----- 작은 헬퍼들 -----
#[allow(unused)]
#[inline]
fn is_tiny_vec(v: Vector3D, tol: f64) -> bool {
    v.length() <= tol
}
```
```rust
#[allow(unused)]
#[inline]
fn approx_unitize(v: Vector3D) -> Option<Vector3D> {
    let mut t = v;
    if t.normalize() { Some(t) } else { None }
}
```
```rust

/// 곡선/곡면에서 쓰는 기본 미분 도구 (이전 제공 함수와 동일 수식)
pub fn ev_tangent(d1: Vector3D, d2: Vector3D) -> Option<Vector3D> {
    let l1 = d1.length();
    if l1 == 0.0 {
        let l2 = d2.length();
        if l2 > 0.0 {
            let mut t = d2;
            t /= l2;
            Some(t)
        } else {
            None
        }
    } else {
        let mut t = d1;
        t /= l1;
        Some(t)
    }
}
```
```rust
pub fn ev_curvature(d1: Vector3D, d2: Vector3D) -> (Option<Vector3D>, Vector3D) {
    // T = D1/|D1|, K = (D2 - (D2·T)T) / |D1|^2
    let l1 = d1.length();
    if l1 == 0.0 {
        // 특이점: 곡률은 0, 접선은 2차 미분으로 추정
        let t = if d2.length() > 0.0 {
            let mut u = d2;
            u.normalize();
            Some(u)
        } else {
            None
        };
        (t, Vector3D::zero())
    } else {
        let mut t = d1;
        t /= l1;
        let neg_d2_dot_t = -d2.dot(&t);
        let inv = 1.0 / (l1 * l1);
        let k = inv * (d2 + (neg_d2_dot_t * t));
        (Some(t), k)
    }
}
```
```rust

/// 두 곡률 벡터의 G2/GSmooth 비교.
/// 에 해당하는 로직을 간단한 기준으로 대체합니다.
/// - 각도 기준: Ka, Kb 방향 코사인 >= cos_angle_tolerance
/// - 크기 기준: | |Ka|-|Kb| | <= curvature_tolerance
fn is_g2_curvature_continuous(
    ka: Vector3D,
    kb: Vector3D,
    cos_angle_tol: f64,
    mag_tol: f64,
) -> bool {
    let la = ka.length();
    let lb = kb.length();
    if la == 0.0 && lb == 0.0 {
        return true;
    }
    if la == 0.0 || lb == 0.0 {
        return false;
    }
    let ca = (ka.dot(&kb)) / (la * lb);
    (ca >= cos_angle_tol) && ((la - lb).abs() <= mag_tol)
}

```
```rust
// GSmooth = G2보다 좀 더 느슨한 크기비 허용이라 가정(예: 상대 오차도 허용)
fn is_gsmooth_curvature_continuous(
    ka: Vector3D,
    kb: Vector3D,
    cos_angle_tol: f64,
    mag_tol: f64,
) -> bool {
    let la = ka.length();
    let lb = kb.length();
    if la == 0.0 && lb == 0.0 {
        return true;
    }
    if la == 0.0 || lb == 0.0 {
        return false;
    }
    let ca = (ka.dot(&kb)) / (la * lb);
    if ca < cos_angle_tol {
        return false;
    }
    let abs_diff = (la - lb).abs();
    let rel = abs_diff / la.max(lb);
    abs_diff <= mag_tol || rel <= 10.0 * mag_tol
}
```
```rust

/// ------------------------------
/// ON_IsContinuous → is_continuous
/// ------------------------------
pub fn is_continuous(
    desired: Continuity,
    pa: Point3D,
    d1a: Vector3D,
    d2a: Vector3D,
    pb: Point3D,
    d1b: Vector3D,
    d2b: Vector3D,
    point_tol: f64,
    d1_tol: f64,
    d2_tol: f64,
    cos_angle_tol: f64,
    curvature_tol: f64,
) -> bool {
    use Continuity::*;
    match desired {
        Unknown => true,

        C0 | C0Locus => (pb - pa).length() <= point_tol,

        C1 | C1Locus => (pb - pa).length() <= point_tol && (d1a - d1b).length() <= d1_tol,

        G1 | G1Locus => {
            let ta = ev_tangent(d1a, d2a).unwrap_or(Vector3D::zero());
            let tb = ev_tangent(d1b, d2b).unwrap_or(Vector3D::zero());
            (pb - pa).length() <= point_tol && ta.dot(&tb) >= cos_angle_tol
        }

        C2 | C2Locus | CInfinity => {
            (pb - pa).length() <= point_tol
                && (d1a - d1b).length() <= d1_tol
                && (d2a - d2b).length() <= d2_tol
        }

        G2 | G2Locus | GSmooth => {
            let (ta, ka) = ev_curvature(d1a, d2a);
            let (tb, kb) = ev_curvature(d1b, d2b);
            let ta = ta.unwrap_or(Vector3D::zero());
            let tb = tb.unwrap_or(Vector3D::zero());
            if (pb - pa).length() > point_tol {
                return false;
            }
            if ta.dot(&tb) < cos_angle_tol {
                return false;
            }
            match desired {
                GSmooth => is_gsmooth_curvature_continuous(ka, kb, cos_angle_tol, curvature_tol),
                _ => is_g2_curvature_continuous(ka, kb, cos_angle_tol, curvature_tol),
            }
        }
    }
}

```
```rust
/// --------------------------------------
/// ON_SearchMonotoneArray → search_monotone_array
/// --------------------------------------
pub fn search_monotone_array(array: &[f64], t: f64) -> i32 {
    if array.is_empty() {
        return -2;
    }
    let length = array.len() as i32 - 1;

    if t < array[0] {
        return -1;
    }
    if t >= array[length as usize] {
        return if t > array[length as usize] {
            length + 1
        } else {
            length
        };
    }
    if array.len() >= 2 && t < array[1] {
        return 0;
    }
    if array.len() >= 2 && t >= array[(length - 1) as usize] {
        return length - 1;
    }

    let mut i0 = 0i32;
    let mut i1 = length;

    while array[i0 as usize] == array[(i0 + 1) as usize] {
        i0 += 1;
    }
    while array[i1 as usize] == array[(i1 - 1) as usize] {
        i1 -= 1;
    }

    while i0 + 1 < i1 {
        let i = (i0 + i1) >> 1;
        if t < array[i as usize] {
            i1 = i;
            while array[i1 as usize] == array[(i1 - 1) as usize] {
                i1 -= 1;
            }
        } else {
            i0 = i;
            while array[i0 as usize] == array[(i0 + 1) as usize] {
                i0 += 1;
            }
        }
    }
    i0
}

```
```rust
/// --------------------------------------
/// 이항/삼항 계수 (안정적 곱셈식 구현)
/// --------------------------------------
pub fn binomial_coefficient(i: i32, j: i32) -> f64 {
    if i < 0 || j < 0 {
        return 0.0;
    }
    if i == 0 || j == 0 {
        return 1.0;
    }
    let n = (i + j) as i64;
    if i == 1 || j == 1 {
        return n as f64;
    }

    let k = i.min(j) as i64;
    let mut num = 1.0_f64;
    let mut den = 1.0_f64;
    for t in 1..=k {
        num *= (n - k + t) as f64;
        den *= t as f64;
        // 간단한 정규화(언더/오버플로 방지)
        let g = num.abs().max(1.0);
        if g > 1e100 {
            num /= 1e50;
            den /= 1e50;
        }
    }
    num / den
}
```
```rust
pub fn trinomial_coefficient(i: i32, j: i32, k: i32) -> f64 {
    binomial_coefficient(i, j + k) * binomial_coefficient(j, k)
}
```
```rust
/// --------------------------------------
/// 포인트/그리드 유효성 검사
/// (stride/rat 고려, 슬라이스 길이 체크)
/// --------------------------------------
pub fn is_valid_point_list_f64(dim: i32, is_rat: bool, count: i32, stride: i32, p: &[f64]) -> bool {
    if dim <= 0 || count < 0 {
        return false;
    }
    let logical_dim = if is_rat { dim + 1 } else { dim };
    if stride < logical_dim {
        return false;
    }
    if count == 0 {
        return true;
    }
    let need = (count as usize - 1) * (stride as usize) + (logical_dim as usize);
    p.len() >= need
}
```
```rust
pub fn is_valid_point_list_f32(dim: i32, is_rat: bool, count: i32, stride: i32, p: &[f32]) -> bool {
    if dim <= 0 || count < 0 {
        return false;
    }
    let logical_dim = if is_rat { dim + 1 } else { dim };
    if stride < logical_dim {
        return false;
    }
    if count == 0 {
        return true;
    }
    let need = (count as usize - 1) * (stride as usize) + (logical_dim as usize);
    p.len() >= need
}
```
```rust

pub fn is_valid_point_grid_f64(
    dim: i32,
    is_rat: bool,
    count0: i32,
    count1: i32,
    stride0: i32,
    stride1: i32,
    p: &[f64],
) -> bool {
    if dim < 1 || count0 < 1 || count1 < 1 {
        return false;
    }
    let logical_dim = if is_rat { dim + 1 } else { dim };
    if stride0 < logical_dim || stride1 < logical_dim {
        return false;
    }
    // 최소 필요 길이 (둘 중 큰 stride를 행/열에 곱해 conservative 하게 계산)
    let need0 = (count0 as usize - 1) * (stride0 as usize)
        + (count1 as usize - 1) * (stride1 as usize)
        + (logical_dim as usize);
    p.len() >= need0
}
```
```rust

/// --------------------------------------
/// 포인트 리스트/그리드 뒤집기 & 좌표 스왑
/// --------------------------------------
pub fn reverse_point_list_f64(
    dim: i32,
    is_rat: bool,
    count: i32,
    stride: i32,
    p: &mut [f64],
) -> bool {
    if !is_valid_point_list_f64(dim, is_rat, count, stride, p) {
        return false;
    }
    if count <= 1 {
        return true;
    }
    let logical_dim = if is_rat { dim + 1 } else { dim } as usize;
    let stride = stride as usize;
    let mut i = 0usize;
    let mut j = (count as usize - 1) * stride;
    while i < j {
        for k in 0..logical_dim {
            p.swap(i + k, j + k);
        }
        i += stride;
        j = j.saturating_sub(stride);
    }
    true
}
```
```rust
pub fn reverse_point_grid_f64(
    dim: i32,
    is_rat: bool,
    count0: i32,
    count1: i32,
    stride0: i32,
    stride1: i32,
    p: &mut [f64],
    dir: i32,
) -> bool {
    if dir == 0 {
        // 0방향은 축 치환해서 동일 함수 호출
        return reverse_point_grid_f64(dim, is_rat, count1, count0, stride1, stride0, p, 1);
    }
    let mut ok = true;
    for row in 0..count0 {
        let start = (row as usize) * (stride0 as usize);
        let slice = &mut p[start..];
        if !reverse_point_list_f64(dim, is_rat, count1, stride1, slice) {
            ok = false;
            break;
        }
    }
    ok
}
```
```rust
pub fn swap_point_list_coords_f64(count: i32, stride: i32, p: &mut [f64], i: i32, j: i32) -> bool {
    if !is_valid_point_list_f64(stride, false, count, stride, p) {
        return false;
    }
    if i < 0 || j < 0 || i >= stride || j >= stride {
        return false;
    }
    if i == j || count == 0 {
        return true;
    }
    let stride = stride as usize;
    let (i, j) = (i as usize, j as usize);
    for idx in 0..(count as usize) {
        let base = idx * stride;
        p.swap(base + i, base + j);
    }
    true
}
```
```rust
pub fn swap_point_list_coords_f32(count: i32, stride: i32, p: &mut [f32], i: i32, j: i32) -> bool {
    if !is_valid_point_list_f32(stride, false, count, stride, p) {
        return false;
    }
    if i < 0 || j < 0 || i >= stride || j >= stride {
        return false;
    }
    if i == j || count == 0 {
        return true;
    }
    let stride = stride as usize;
    let (i, j) = (i as usize, j as usize);
    for idx in 0..(count as usize) {
        let base = idx * stride;
        p.swap(base + i, base + j);
    }
    true
}
```
```rust
pub fn swap_point_grid_coords_f64(
    count0: i32,
    count1: i32,
    stride0: i32,
    stride1: i32,
    p: &mut [f64],
    i: i32,
    j: i32,
) -> bool {
    if p.is_empty() {
        return false;
    }
    let (i, j) = (i as usize, j as usize);
    let (s0, s1) = (stride0 as usize, stride1 as usize);
    for r in 0..(count0 as usize) {
        let mut base = r * s0;
        for _c in 0..(count1 as usize) {
            p.swap(base + i, base + j);
            base += s1;
        }
    }
    true
}
```
```rust

/// --------------------------------------
/// Transform 계열: 포인트/벡터 배열 변환
///  - dim: 1/2/3 이상 지원
///  - is_rat: 동차좌표 여부 (w 포함)
///  - stride: 다음 점(벡터)까지 거리
///  - count: 개수
/// --------------------------------------
pub fn transform_point_list_f32(
    dim: i32,
    is_rat: bool,
    count: i32,
    stride: i32,
    point: &mut [f32],
    xform: &Transform,
) -> bool {
    if !is_valid_point_list_f32(dim, is_rat, count, stride, point) {
        return false;
    }
    if count == 0 {
        return true;
    }
    let m = &xform.m;
    let stride = stride as usize;

    let mut idx = 0usize;
    for _ in 0..count {
        if is_rat {
            match dim {
                1 => {
                    let x = m[0][0] * point[idx] as f64 + m[0][3] * point[idx + 1] as f64;
                    let w = m[3][0] * point[idx] as f64 + m[3][3] * point[idx + 1] as f64;
                    point[idx] = x as f32;
                    point[idx + 1] = w as f32;
                }
                2 => {
                    let x = m[0][0] * point[idx] as f64
                        + m[0][1] * point[idx + 1] as f64
                        + m[0][3] * point[idx + 2] as f64;
                    let y = m[1][0] * point[idx] as f64
                        + m[1][1] * point[idx + 1] as f64
                        + m[1][3] * point[idx + 2] as f64;
                    let w = m[3][0] * point[idx] as f64
                        + m[3][1] * point[idx + 1] as f64
                        + m[3][3] * point[idx + 2] as f64;
                    point[idx] = x as f32;
                    point[idx + 1] = y as f32;
                    point[idx + 2] = w as f32;
                }
                _ => {
                    // dim >= 3, w는 point[idx+dim]
                    let d = dim as usize;
                    let x = m[0][0] * point[idx] as f64
                        + m[0][1] * point[idx + 1] as f64
                        + m[0][2] * point[idx + 2] as f64
                        + m[0][3] * point[idx + d] as f64;
                    let y = m[1][0] * point[idx] as f64
                        + m[1][1] * point[idx + 1] as f64
                        + m[1][2] * point[idx + 2] as f64
                        + m[1][3] * point[idx + d] as f64;
                    let z = m[2][0] * point[idx] as f64
                        + m[2][1] * point[idx + 1] as f64
                        + m[2][2] * point[idx + 2] as f64
                        + m[2][3] * point[idx + d] as f64;
                    let w = m[3][0] * point[idx] as f64
                        + m[3][1] * point[idx + 1] as f64
                        + m[3][2] * point[idx + 2] as f64
                        + m[3][3] * point[idx + d] as f64;
                    point[idx] = x as f32;
                    point[idx + 1] = y as f32;
                    point[idx + 2] = z as f32;
                    point[idx + d] = w as f32;
                }
            }
        } else {
            match dim {
                1 => {
                    let mut w = m[3][0] * point[idx] as f64 + m[3][3];
                    if w == 0.0 {
                        w = 1.0;
                    }
                    let w_inv = 1.0 / w;
                    let x = m[0][0] * point[idx] as f64 + m[0][3];
                    point[idx] = (w_inv * x) as f32;
                }
                2 => {
                    let mut w =
                        m[3][0] * point[idx] as f64 + m[3][1] * point[idx + 1] as f64 + m[3][3];
                    if w == 0.0 {
                        w = 1.0;
                    }
                    let w_inv = 1.0 / w;
                    let x = m[0][0] * point[idx] as f64 + m[0][1] * point[idx + 1] as f64 + m[0][3];
                    let y = m[1][0] * point[idx] as f64 + m[1][1] * point[idx + 1] as f64 + m[1][3];
                    point[idx] = (w_inv * x) as f32;
                    point[idx + 1] = (w_inv * y) as f32;
                }
                _ => {
                    let mut w = m[3][0] * point[idx] as f64
                        + m[3][1] * point[idx + 1] as f64
                        + m[3][2] * point[idx + 2] as f64
                        + m[3][3];
                    if w == 0.0 {
                        w = 1.0;
                    }
                    let w_inv = 1.0 / w;
                    let x = m[0][0] * point[idx] as f64
                        + m[0][1] * point[idx + 1] as f64
                        + m[0][2] * point[idx + 2] as f64
                        + m[0][3];
                    let y = m[1][0] * point[idx] as f64
                        + m[1][1] * point[idx + 1] as f64
                        + m[1][2] * point[idx + 2] as f64
                        + m[1][3];
                    let z = m[2][0] * point[idx] as f64
                        + m[2][1] * point[idx + 1] as f64
                        + m[2][2] * point[idx + 2] as f64
                        + m[2][3];
                    point[idx] = (w_inv * x) as f32;
                    point[idx + 1] = (w_inv * y) as f32;
                    point[idx + 2] = (w_inv * z) as f32;
                }
            }
        }
        idx += stride;
    }
    true
}
```
```rust
pub fn transform_point_list_f64(
    dim: i32,
    is_rat: bool,
    count: i32,
    stride: i32,
    point: &mut [f64],
    xform: &Transform,
) -> bool {
    if !is_valid_point_list_f64(dim, is_rat, count, stride, point) {
        return false;
    }
    if count == 0 {
        return true;
    }
    let m = &xform.m;
    let stride = stride as usize;

    let mut idx = 0usize;
    for _ in 0..count {
        if is_rat {
            match dim {
                1 => {
                    let x = m[0][0] * point[idx] + m[0][3] * point[idx + 1];
                    let w = m[3][0] * point[idx] + m[3][3] * point[idx + 1];
                    point[idx] = x;
                    point[idx + 1] = w;
                }
                2 => {
                    let x =
                        m[0][0] * point[idx] + m[0][1] * point[idx + 1] + m[0][3] * point[idx + 2];
                    let y =
                        m[1][0] * point[idx] + m[1][1] * point[idx + 1] + m[1][3] * point[idx + 2];
                    let w =
                        m[3][0] * point[idx] + m[3][1] * point[idx + 1] + m[3][3] * point[idx + 2];
                    point[idx] = x;
                    point[idx + 1] = y;
                    point[idx + 2] = w;
                }
                _ => {
                    let d = dim as usize;
                    let x = m[0][0] * point[idx]
                        + m[0][1] * point[idx + 1]
                        + m[0][2] * point[idx + 2]
                        + m[0][3] * point[idx + d];
                    let y = m[1][0] * point[idx]
                        + m[1][1] * point[idx + 1]
                        + m[1][2] * point[idx + 2]
                        + m[1][3] * point[idx + d];
                    let z = m[2][0] * point[idx]
                        + m[2][1] * point[idx + 1]
                        + m[2][2] * point[idx + 2]
                        + m[2][3] * point[idx + d];
                    let w = m[3][0] * point[idx]
                        + m[3][1] * point[idx + 1]
                        + m[3][2] * point[idx + 2]
                        + m[3][3] * point[idx + d];
                    point[idx] = x;
                    point[idx + 1] = y;
                    point[idx + 2] = z;
                    point[idx + d] = w;
                }
            }
        } else {
            match dim {
                1 => {
                    let mut w = m[3][0] * point[idx] + m[3][3];
                    if w == 0.0 {
                        w = 1.0;
                    }
                    let w_inv = 1.0 / w;
                    let x = m[0][0] * point[idx] + m[0][3];
                    point[idx] = w_inv * x;
                }
                2 => {
                    let mut w = m[3][0] * point[idx] + m[3][1] * point[idx + 1] + m[3][3];
                    if w == 0.0 {
                        w = 1.0;
                    }
                    let w_inv = 1.0 / w;
                    let x = m[0][0] * point[idx] + m[0][1] * point[idx + 1] + m[0][3];
                    let y = m[1][0] * point[idx] + m[1][1] * point[idx + 1] + m[1][3];
                    point[idx] = w_inv * x;
                    point[idx + 1] = w_inv * y;
                }
                _ => {
                    let mut w = m[3][0] * point[idx]
                        + m[3][1] * point[idx + 1]
                        + m[3][2] * point[idx + 2]
                        + m[3][3];
                    if w == 0.0 {
                        w = 1.0;
                    }
                    let w_inv = 1.0 / w;
                    let x = m[0][0] * point[idx]
                        + m[0][1] * point[idx + 1]
                        + m[0][2] * point[idx + 2]
                        + m[0][3];
                    let y = m[1][0] * point[idx]
                        + m[1][1] * point[idx + 1]
                        + m[1][2] * point[idx + 2]
                        + m[1][3];
                    let z = m[2][0] * point[idx]
                        + m[2][1] * point[idx + 1]
                        + m[2][2] * point[idx + 2]
                        + m[2][3];
                    point[idx] = w_inv * x;
                    point[idx + 1] = w_inv * y;
                    point[idx + 2] = w_inv * z;
                }
            }
        }
        idx += stride;
    }
    true
}
```
```rust

pub fn transform_point_grid_f64(
    dim: i32,
    is_rat: bool,
    count0: i32,
    count1: i32,
    stride0: i32,
    stride1: i32,
    point: &mut [f64],
    xform: &Transform,
) -> bool {
    let mut ok = false;
    for r in 0..count0 {
        let base = (r as usize) * (stride0 as usize);
        let slice = &mut point[base..];
        if !transform_point_list_f64(dim, is_rat, count1, stride1, slice, xform) {
            ok = false;
            break;
        } else if r == 0 {
            ok = true;
        }
    }
    ok
}
```
```rust

pub fn transform_vector_list_f32(
    dim: i32,
    count: i32,
    stride: i32,
    v: &mut [f32],
    x: &Transform,
) -> bool {
    if !is_valid_point_list_f32(dim, false, count, stride, v) {
        return false;
    }
    if count == 0 {
        return true;
    }
    let m = &x.m;
    let stride = stride as usize;
    let mut idx = 0usize;
    for _ in 0..count {
        match dim {
            1 => {
                let nx = m[0][0] * v[idx] as f64;
                v[idx] = nx as f32;
            }
            2 => {
                let nx = m[0][0] * v[idx] as f64 + m[0][1] * v[idx + 1] as f64;
                let ny = m[1][0] * v[idx] as f64 + m[1][1] * v[idx + 1] as f64;
                v[idx] = nx as f32;
                v[idx + 1] = ny as f32;
            }
            _ => {
                let nx = m[0][0] * v[idx] as f64
                    + m[0][1] * v[idx + 1] as f64
                    + m[0][2] * v[idx + 2] as f64;
                let ny = m[1][0] * v[idx] as f64
                    + m[1][1] * v[idx + 1] as f64
                    + m[1][2] * v[idx + 2] as f64;
                let nz = m[2][0] * v[idx] as f64
                    + m[2][1] * v[idx + 1] as f64
                    + m[2][2] * v[idx + 2] as f64;
                v[idx] = nx as f32;
                v[idx + 1] = ny as f32;
                v[idx + 2] = nz as f32;
            }
        }
        idx += stride;
    }
    true
}
```
```rust
pub fn transform_vector_list_f64(
    dim: i32,
    count: i32,
    stride: i32,
    v: &mut [f64],
    x: &Transform,
) -> bool {
    if !is_valid_point_list_f64(dim, false, count, stride, v) {
        return false;
    }
    if count == 0 {
        return true;
    }
    let m = &x.m;
    let stride = stride as usize;
    let mut idx = 0usize;
    for _ in 0..count {
        match dim {
            1 => {
                let nx = m[0][0] * v[idx];
                v[idx] = nx;
            }
            2 => {
                let nx = m[0][0] * v[idx] + m[0][1] * v[idx + 1];
                let ny = m[1][0] * v[idx] + m[1][1] * v[idx + 1];
                v[idx] = nx;
                v[idx + 1] = ny;
            }
            _ => {
                let nx = m[0][0] * v[idx] + m[0][1] * v[idx + 1] + m[0][2] * v[idx + 2];
                let ny = m[1][0] * v[idx] + m[1][1] * v[idx + 1] + m[1][2] * v[idx + 2];
                let nz = m[2][0] * v[idx] + m[2][1] * v[idx + 1] + m[2][2] * v[idx + 2];
                v[idx] = nx;
                v[idx + 1] = ny;
                v[idx + 2] = nz;
            }
        }
        idx += stride;
    }
    true
}

```
```rust
#[inline]
fn is_finite_nonzero(x: f64) -> bool {
    x.is_finite() && x != 0.0
}

```
```rust
/// pointA, pointB 각각이 길이 >= (dim + (is_rat as usize)) 이어야 합니다.
pub fn on_points_are_coincident(
    dim: usize,
    is_rat: bool,
    point_a: &[f64],
    point_b: &[f64],
) -> bool {
    if dim < 1 || point_a.is_empty() || point_b.is_empty() {
        return false;
    }
    if is_rat {
        if point_a.len() < dim + 1 || point_b.len() < dim + 1 {
            return false;
        }
        let wa = point_a[dim];
        let wb = point_b[dim];
        if wa == 0.0 || wb == 0.0 {
            // 둘 다 0이면 비가중(비동차) 비교로 대체
            if wa == 0.0 && wb == 0.0 {
                return on_points_are_coincident(dim, false, point_a, point_b);
            }
            return false;
        }
        for i in 0..dim {
            let a = point_a[i] / wa;
            let b = point_b[i] / wb;
            let d = (a - b).abs();
            if d <= ON_ZERO_TOL {
                continue;
            }
            if d <= (a.abs() + b.abs()) * ON_REL_TOL {
                continue;
            }
            return false;
        }
        true
    } else {
        if point_a.len() < dim || point_b.len() < dim {
            return false;
        }
        for i in 0..dim {
            let a = point_a[i];
            let b = point_b[i];
            let d = (a - b).abs();
            if d <= ON_ZERO_TOL {
                continue;
            }
            if d <= (a.abs() + b.abs()) * ON_REL_TOL {
                continue;
            }
            return false;
        }
        true
    }
}

```
```rust
/// `points`는 최소 길이 >= ((point_count-1)*point_stride + (dim + is_rat?1:0))
pub fn on_points_are_coincident_list(
    dim: usize,
    is_rat: bool,
    point_count: usize,
    point_stride: usize,
    points: &[f64],
) -> bool {
    if points.is_empty() || point_count < 2 {
        return false;
    }
    let need = (point_count - 1) * point_stride + if is_rat { dim + 1 } else { dim };
    if points.len() < need {
        return false;
    }
    // 처음과 마지막 비교
    let a0 = &points[0..(if is_rat { dim + 1 } else { dim })];
    let b0 = &points[(point_count - 1) * point_stride..][..(if is_rat { dim + 1 } else { dim })];
    if !on_points_are_coincident(dim, is_rat, a0, b0) {
        return false;
    }
    if point_count > 2 {
        let mut idx = 0;
        for _ in 0..(point_count - 1) {
            let a = &points[idx..][..(if is_rat { dim + 1 } else { dim })];
            let b = &points[idx + point_stride..][..(if is_rat { dim + 1 } else { dim })];
            if !on_points_are_coincident(dim, is_rat, a, b) {
                return false;
            }
            idx += point_stride;
        }
    }
    true
}

```
```rust
/// 반환값: -1(first<second), 0(==), +1(first>second)
pub fn on_compare_point(dim: usize, is_rat: bool, point_a: &[f64], point_b: &[f64]) -> i32 {
    if is_rat {
        if point_a.len() < dim + 1 || point_b.len() < dim + 1 {
            return 0;
        }
    } else {
        if point_a.len() < dim || point_b.len() < dim {
            return 0;
        }
    }

    // 동차라면 w = 1/weight 로 유클리드 좌표로 환산
    let (w_a, w_b) = if is_rat {
        let wa = point_a[dim];
        let wb = point_b[dim];
        // weight==0일 때는 비교 불능 → 그대로 0 반환(필요시 정책 조정)
        if wa == 0.0 || wb == 0.0 {
            return 0;
        }
        (1.0 / wa, 1.0 / wb)
    } else {
        (1.0, 1.0)
    };

    // 좌표별 비교 (lexicographic, 상대/절대 tol)
    for i in 0..dim {
        let a = w_a * point_a[i];
        let b = w_b * point_b[i];
        let mut tol = (a.abs() + b.abs()) * ON_REL_TOL;
        if tol < ON_ZERO_TOL {
            tol = ON_ZERO_TOL;
        }

        if a < b - tol {
            return -1;
        }
        if b < a - tol {
            return 1;
        }
    }

    // 여기까지 왔으면 유클리드 좌표들은 모두 같음 → 같다고 본다.
    // (이 줄이 핵심 변경점: 가중치(w)로 tie-break 하지 않음)
    0

    // 만약 ‘정렬 안정성’ 때문에 꼭 타이브레이커가 필요하면,
    // 아래를 옵션으로 켜는 형태로 분기하세요.
    //
    // if w_a < w_b - ON_SQRT_EPSILON { -1 }
    // else if w_b < w_a - ON_SQRT_EPSILON { 1 }
    // else { 0 }
}

```
```rust
/// 두 리스트 비교 (stride 포함). 반환: -1/0/+1
pub fn on_compare_point_list(
    dim: usize,
    is_rat: bool,
    point_count: usize,
    stride_a: usize,
    point_a: &[f64],
    stride_b: usize,
    point_b: &[f64],
) -> i32 {
    if point_count == 0 {
        return 0;
    }
    let need_a = (point_count - 1) * stride_a + if is_rat { dim + 1 } else { dim };
    let need_b = (point_count - 1) * stride_b + if is_rat { dim + 1 } else { dim };
    if point_a.len() < need_a || point_b.len() < need_b {
        return 0;
    }

    let mut rc = 0;
    let mut rc1 = 0;

    let b_do_second_check = is_rat
        && dim <= 3
        && point_count > 0
        && (point_a[dim].is_finite())
        && (point_b[dim].is_finite())
        && point_a[dim] != 0.0
        && point_b[dim] != 0.0;

    let mut ia = 0usize;
    let mut ib = 0usize;

    for _ in 0..point_count {
        if rc != 0 {
            break;
        }
        let pa = &point_a[ia..];
        let pb = &point_b[ib..];
        rc = on_compare_point(dim, is_rat, pa, pb);

        if rc != 0 && b_do_second_check && is_finite_nonzero(pa[dim]) && is_finite_nonzero(pb[dim])
        {
            if rc1 == 0 {
                rc1 = rc;
            }
            // 동차좌표를 비동차로 환산해서 한 번 더 비교
            let mut a_xyz = [0.0f64; 3];
            let mut b_xyz = [0.0f64; 3];
            for k in 0..dim {
                a_xyz[k] = pa[k] / pa[dim];
                b_xyz[k] = pb[k] / pb[dim];
            }
            rc = if on_compare_point(dim, false, &a_xyz[..dim], &b_xyz[..dim]) == 0 {
                0
            } else {
                rc1
            };
        }

        ia += stride_a;
        ib += stride_b;
    }

    rc
}

```
```rust
/// 점열이 닫혀있는지(첫/마지막 동일 + 중간에 다른 점 존재) 검사
pub fn on_is_point_list_closed(
    dim: usize,
    is_rat: bool,
    count: usize,
    stride: usize,
    p: &[f64],
) -> bool {
    if count < 4 {
        return false;
    }
    let need = (count - 1) * stride + if is_rat { dim + 1 } else { dim };
    if p.len() < need {
        return false;
    }
    // 첫/마지막 동일?
    if on_compare_point(dim, is_rat, p, &p[stride * (count - 1)..]) != 0 {
        return false;
    }
    // 전부 같은 점이 쌓인 경우는 "닫힘"으로 보지 않음 → 중간에 다른 점이 하나라도 있나 확인
    for i in 1..(count - 1) {
        if on_compare_point(dim, is_rat, p, &p[stride * i..]) != 0 {
            return true;
        }
    }
    false
}

```
```rust
/// 점 격자가 특정 방향(dir)에 대해 닫혀있는지 검사
/// dir=false(0): 0-방향 폐합 검사 (첫 행/마지막 행 비교)
/// dir=true(1) : 1-방향 폐합 검사 (각 행의 첫/마지막 열 비교)
pub fn on_is_point_grid_closed(
    dim: usize,
    is_rat: bool,
    point_count0: usize,
    point_count1: usize,
    point_stride0: usize,
    point_stride1: usize,
    p: &[f64],
    dir: bool,
) -> bool {
    if point_count0 == 0 || point_count1 == 0 || p.is_empty() {
        return false;
    }

    let (count, stride, p0, p1) = if dir {
        // 1-방향: 각 행에서 첫/끝 열을 비교 → 행 수만큼 비교해야 하지만,
        // 원본 C코드는 ON_ComparePointList를 써서 열 전체를 한번에 비교함.
        let count = point_count0;
        let stride = point_stride0;
        let p0 = p;
        let p1 = &p[(point_count1 - 1) * point_stride1..];
        (count, stride, p0, p1)
    } else {
        // 0-방향: 첫 행과 마지막 행 비교 (열 개수만큼)
        let count = point_count1;
        let stride = point_stride1;
        let p0 = p;
        let p1 = &p[(point_count0 - 1) * point_stride0..];
        (count, stride, p0, p1)
    };

    on_compare_point_list(dim, is_rat, count, stride, p0, stride, p1) == 0
}

```
```rust
pub fn slice_view<'a>(buf: &'a [f64], start: usize, len: usize) -> &'a [f64] {
    &buf[start..start + len]
}

```
```rust
// src/alg_utils.rs
use crate::core::tmatrix::TMatrix;
use std::cmp::Ordering;
/* -------------------------------------------------------------------------- */
/*                             Tridiagonal solver                              */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriDiagError {
    IllegalInput, // dim < 1, n < 2, 길이 불일치 등
    ZeroPivot,    // 앞진행/후진행 중 0 pivot
}

```
```rust
/// 삼대각 행렬 Ax = d (단일 RHS)
/// a: subdiag[0..n-2], b: diag[0..n-1], c: superdiag[0..n-2], d: rhs[0..n-1]
pub fn solve_tridiagonal_scalar(
    n: usize,
    a: &[f64],
    b: &[f64],
    c: &[f64],
    d: &[f64],
    x: &mut [f64],
) -> Result<(), &'static str> {
    assert!(n >= 1 && b.len() == n && d.len() == n && x.len() == n);
    assert!(a.len() == n.saturating_sub(1) && c.len() == n.saturating_sub(1));

    if n == 1 {
        if b[0].abs() < f64::EPSILON {
            return Err("singular");
        }
        x[0] = d[0] / b[0];
        return Ok(());
    }

    // modified super diag & RHS
    let mut cp = vec![0.0f64; n - 1];
    let mut dp = vec![0.0f64; n];

    // i=0
    let mut denom = b[0];
    if denom.abs() < f64::EPSILON {
        return Err("singular");
    }
    cp[0] = c[0] / denom;
    dp[0] = d[0] / denom;

    // forward sweep i=1..n-2
    for i in 1..(n - 1) {
        denom = b[i] - a[i - 1] * cp[i - 1];
        if denom.abs() < f64::EPSILON {
            return Err("singular");
        }
        cp[i] = c[i] / denom;
        dp[i] = (d[i] - a[i - 1] * dp[i - 1]) / denom;
    }
    // last row i=n-1
    denom = b[n - 1] - a[n - 2] * cp[n - 2];
    if denom.abs() < f64::EPSILON {
        return Err("singular");
    }
    dp[n - 1] = (d[n - 1] - a[n - 2] * dp[n - 2]) / denom;

    // back substitution
    x[n - 1] = dp[n - 1];
    for i in (0..=(n - 2)).rev() {
        x[i] = dp[i] - cp[i] * x[i + 1];
    }
    Ok(())
}

```
```rust
/// 삼대각 + 다중 RHS (열 수 r)
/// D: (n x r) RHS, X: (n x r) 해
pub fn solve_tridiagonal_multi_rhs(
    n: usize,
    r: usize,
    a: &[f64],
    b: &[f64],
    c: &mut [f64],
    d: &[f64],     // column-major가 아니라 row-major: d[i*r + j]
    x: &mut [f64], // same layout
) -> Result<(), &'static str> {
    assert!(
        n >= 1 && b.len() == n && a.len() == n.saturating_sub(1) && c.len() == n.saturating_sub(1)
    );
    assert!(d.len() == n * r && x.len() == n * r);

    // 각 RHS에 대해 Thomas 수행 (가장 간단/안전)
    let mut tmpx = vec![0.0; n];
    let mut rhs = vec![0.0; n];
    for col in 0..r {
        for i in 0..n {
            rhs[i] = d[i * r + col];
        }
        solve_tridiagonal_scalar(n, a, b, c, &rhs, &mut tmpx)?;
        for i in 0..n {
            x[i * r + col] = tmpx[i];
        }
    }
    Ok(())
}
```
```rust

///
/// - dim == 1:
///   - a.len() = n-1, b.len() = n, c.len() = n-1, d.len() = n, x.len() = n
/// - dim > 1:
///   - a.len() = n-1, b.len() = n, c.len() = n-1,
///   - d.len() = n*dim, x.len() = n*dim
///
/// 주의: `c`는 원본과 동일하게 **in-place로 수정**됩니다.
pub fn solve_tridiagonal(
    dim: usize,
    n: usize,
    a: &[f64],
    b: &[f64],
    c: &mut [f64],
    d: &[f64],
    x: &mut [f64],
) -> Result<(), TriDiagError> {
    if dim < 1 || n < 2 {
        return Err(TriDiagError::IllegalInput);
    }
    if a.len() != n - 1 || b.len() != n || c.len() != n - 1 {
        return Err(TriDiagError::IllegalInput);
    }
    if dim == 1 {
        if d.len() != n || x.len() != n {
            return Err(TriDiagError::IllegalInput);
        }
        // forward sweep
        let mut beta = b[0];
        if beta == 0.0 {
            return Err(TriDiagError::ZeroPivot);
        }
        beta = 1.0 / beta;
        x[0] = d[0] * beta;

        for i in 1..n {
            let g = {
                c[i - 1] *= beta;
                c[i - 1]
            };
            beta = b[i] - a[i - 1] * g;
            if beta == 0.0 {
                return Err(TriDiagError::ZeroPivot);
            }
            beta = 1.0 / beta;
            x[i] = (d[i] - a[i - 1] * x[i - 1]) * beta;
        }

        // backward substitution
        for i in (0..n - 1).rev() {
            x[i] -= c[i] * x[i + 1];
        }
        Ok(())
    } else {
        // vector case
        if d.len() != n * dim || x.len() != n * dim {
            return Err(TriDiagError::IllegalInput);
        }
        let mut beta = b[0];
        if beta == 0.0 {
            return Err(TriDiagError::ZeroPivot);
        }
        beta = 1.0 / beta;

        // X_0 = d_0 * beta
        for j in 0..dim {
            x[j] = d[j] * beta;
        }

        // forward sweep by blocks
        for i in 1..n {
            let g = {
                c[i - 1] *= beta;
                c[i - 1]
            };
            beta = b[i] - a[i - 1] * g;
            if beta == 0.0 {
                return Err(TriDiagError::ZeroPivot);
            }
            beta = 1.0 / beta;

            let q = a[i - 1];
            let base_prev = (i - 1) * dim;
            let base_cur = i * dim;
            for j in 0..dim {
                x[base_cur + j] = (d[base_cur + j] - q * x[base_prev + j]) * beta;
            }
        }

        // backward substitution by blocks
        for i in (0..n - 1).rev() {
            let q = c[i];
            let base_cur = i * dim;
            let base_next = (i + 1) * dim;
            for j in 0..dim {
                x[base_cur + j] -= q * x[base_next + j];
            }
        }
        Ok(())
    }
}

```
```rust
/* -------------------------------------------------------------------------- */
/*                                ON_Sort 계열                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortAlgorithm {
    QuickSort, // 내부적으로 unstable sort 사용
    HeapSort,  // 직접 구현한 힙소트 사용
}

```
```rust
/// 데이터 슬라이스 `data`를 정렬하지 않고 **인덱스만** 정렬해 반환합니다.
/// - C++: ON_Sort(index, data, count, sizeof_element, compar) 에 해당
pub fn sort_indices<T, F>(algo: SortAlgorithm, data: &[T], mut cmp: F) -> Vec<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut idx: Vec<usize> = (0..data.len()).collect();
    match algo {
        SortAlgorithm::QuickSort => {
            idx.sort_unstable_by(|&i, &j| cmp(&data[i], &data[j]));
        }
        SortAlgorithm::HeapSort => {
            // 단순 힙소트 (최대 힙)
            heap_sort_indices(&mut idx, |&i, &j| cmp(&data[i], &data[j]));
        }
    }
    idx
}

```
```rust
fn heap_sort_indices<F>(idx: &mut [usize], mut cmp: F)
where
    F: FnMut(&usize, &usize) -> Ordering,
{
    // 최대 힙으로 만들어서 내림차순 pop → 오름차순 완성
    let n = idx.len();
    if n < 2 {
        return;
    }
    // heapify
    for start in (0..=(n / 2)).rev() {
        sift_down(idx, start, n - 1, &mut cmp);
    }
    // sort
    for end in (1..n).rev() {
        idx.swap(0, end);
        sift_down(idx, 0, end - 1, &mut cmp);
    }
}
```
```rust
fn sift_down<F>(idx: &mut [usize], start: usize, end: usize, cmp: &mut F)
where
    F: FnMut(&usize, &usize) -> Ordering,
{
    let mut root = start;
    loop {
        let left = 2 * root + 1;
        if left > end {
            break;
        }
        let mut swap_i = root;
        if cmp(&idx[swap_i], &idx[left]) == Ordering::Less {
            swap_i = left;
        }
        let right = left + 1;
        if right <= end && cmp(&idx[swap_i], &idx[right]) == Ordering::Less {
            swap_i = right;
        }
        if swap_i == root {
            return;
        } else {
            idx.swap(root, swap_i);
            root = swap_i;
        }
    }
}

```
```rust
pub fn sort_string_array(algo: SortAlgorithm, arr: &mut [String]) {
    match algo {
        SortAlgorithm::QuickSort => arr.sort_unstable(),
        SortAlgorithm::HeapSort => {
            // 인덱스 힙소트 → 재배치
            let idx = sort_indices(SortAlgorithm::HeapSort, arr, |a, b| a.cmp(b));
            reorder_in_place(arr, &idx);
        }
    }
}

```
```rust
// 인덱스에 따라 배열을 제자리 재배치(사이클 분해)
fn reorder_in_place<T: Clone>(arr: &mut [T], idx: &[usize]) {
    // idx[k] = 정렬 후 k 위치에 와야 하는 원소의 원래 인덱스
    // 이를 제자리에서 재배치한다 (안전하게는 임시 벡터가 깔끔)
    let mut tmp: Vec<T> = Vec::with_capacity(arr.len());
    for &i in idx {
        tmp.push(arr[i].clone());
    }
    arr.clone_from_slice(&tmp);
}

```
```rust
/* -------------------------------------------------------------------------- */
/*                             Binary search helpers                           */
/* -------------------------------------------------------------------------- */

pub fn binary_search_i32<'a>(key: i32, a: &'a [i32]) -> Option<&'a i32> {
    a.binary_search(&key).ok().map(|i| &a[i])
}

```
```rust
pub fn binary_search_u32<'a>(key: u32, a: &'a [u32]) -> Option<&'a u32> {
    a.binary_search(&key).ok().map(|i| &a[i])
}

```
```rust
pub fn binary_search_f64<'a>(key: f64, a: &'a [f64]) -> Option<&'a f64> {
    a.binary_search_by(|v| v.partial_cmp(&key).unwrap_or(Ordering::Less))
        .ok()
        .map(|i| &a[i])
}

```
```rust
/// (구조체 슬라이스에서) **정렬된 u32 키 필드를 기준**으로 이진검색
pub fn binary_search_by_u32_key<'a, T, F>(key: u32, a: &'a [T], key_of: F) -> Option<&'a T>
where
    F: Fn(&T) -> u32,
{
    let mut lo = 0;
    let mut hi = a.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        let k = key_of(&a[mid]);
        if key < k {
            hi = mid;
        } else if key > k {
            lo = mid + 1;
        } else {
            return Some(&a[mid]);
        }
    }
    None
}

```
```rust
/// (동일 키가 여러 개일 때) 첫 번째 원소(lower_bound) 반환
pub fn binary_search_first_by_u32_key<'a, T, F>(key: u32, a: &'a [T], key_of: F) -> Option<&'a T>
where
    F: Fn(&T) -> u32,
{
    let mut lo = 0usize;
    let mut hi = a.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if key_of(&a[mid]) < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo < a.len() && key_of(&a[lo]) == key {
        Some(&a[lo])
    } else {
        None
    }
}
```
```rust

/* -------------------------------------------------------------------------- */
/*                            ON_*dex 구조체군 (Rust)                          */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U2 {
    pub i: u32,
    pub j: u32,
}
```
```rust
impl U2 {
    pub const UNSET: Self = Self {
        i: u32::MAX,
        j: u32::MAX,
    };
```
```rust
    pub const ZERO: Self = Self { i: 0, j: 0 };
```
```rust

    pub fn new(i: u32, j: u32) -> Self {
        Self { i, j }
    }
```
```rust
    pub fn as_increasing(self) -> Self {
        if self.j < self.i {
            Self {
                i: self.j,
                j: self.i,
            }
        } else {
            self
        }
    }
```
```rust
    pub fn as_decreasing(self) -> Self {
        if self.i < self.j {
            Self {
                i: self.j,
                j: self.i,
            }
        } else {
            self
        }
    }
```
```rust

    pub fn dict_compare(a: &Self, b: &Self) -> Ordering {
        (a.i, a.j).cmp(&(b.i, b.j))
    }
```
```rust
    pub fn cmp_first(a: &Self, b: &Self) -> Ordering {
        a.i.cmp(&b.i)
    }
```
```rust
    pub fn cmp_second(a: &Self, b: &Self) -> Ordering {
        a.j.cmp(&b.j)
    }
}
```
```rust
impl Ord for U2 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.i, self.j).cmp(&(other.i, other.j))
    }
}
```
```rust
impl PartialOrd for U2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2 {
    pub i: i32,
    pub j: i32,
}
```
```rust
impl I2 {
    pub fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
```
```rust
    pub fn as_increasing(self) -> Self {
        if self.j < self.i {
            Self {
                i: self.j,
                j: self.i,
            }
        } else {
            self
        }
    }
```
```rust
    pub fn as_decreasing(self) -> Self {
        if self.i < self.j {
            Self {
                i: self.j,
                j: self.i,
            }
        } else {
            self
        }
    }
}
```
```rust
impl Ord for I2 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.i, self.j).cmp(&(other.i, other.j))
    }
}
```
```rust
impl PartialOrd for I2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I3 {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}
```
```rust
impl I3 {
    pub const UNSET: Self = Self {
        i: i32::MIN,
        j: i32::MIN,
        k: i32::MIN,
    };
```
```rust
    pub const ZERO: Self = Self { i: 0, j: 0, k: 0 };
```
```rust
    pub fn new(i: i32, j: i32, k: i32) -> Self {
        Self { i, j, k }
    }
}
```
```rust
impl Ord for I3 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.i, self.j, self.k).cmp(&(other.i, other.j, other.k))
    }
}
```
```rust
impl PartialOrd for I3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
```rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U3 {
    pub i: u32,
    pub j: u32,
    pub k: u32,
}
```
```rust
impl U3 {
    pub const UNSET: Self = Self {
        i: u32::MAX,
        j: u32::MAX,
        k: u32::MAX,
    };
```
```rust
    pub const ZERO: Self = Self { i: 0, j: 0, k: 0 };
```
```rust
    pub fn new(i: u32, j: u32, k: u32) -> Self {
        Self { i, j, k }
    }
```
```rust
    pub fn dict_compare(a: &Self, b: &Self) -> Ordering {
        (a.i, a.j, a.k).cmp(&(b.i, b.j, b.k))
    }
```
```rust
    pub fn cmp_first(a: &Self, b: &Self) -> Ordering {
        a.i.cmp(&b.i)
    }
```
```rust
    pub fn cmp_second(a: &Self, b: &Self) -> Ordering {
        a.j.cmp(&b.j)
    }
```
```rust
    pub fn cmp_third(a: &Self, b: &Self) -> Ordering {
        a.k.cmp(&b.k)
    }
```
```rust
    pub fn cmp_first_second(a: &Self, b: &Self) -> Ordering {
        (a.i, a.j).cmp(&(b.i, b.j))
    }
}
```
```rust
impl Ord for U3 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.i, self.j, self.k).cmp(&(other.i, other.j, other.k))
    }
}
```
```rust
impl PartialOrd for U3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I4 {
    pub i: i32,
    pub j: i32,
    pub k: i32,
    pub l: i32,
}
```
```rust
impl I4 {
    pub const UNSET: Self = Self {
        i: i32::MIN,
        j: i32::MIN,
        k: i32::MIN,
        l: i32::MIN,
    };
```
```rust
    pub const ZERO: Self = Self {
        i: 0,
        j: 0,
        k: 0,
        l: 0,
    };
```
```rust

    pub fn new(i: i32, j: i32, k: i32, l: i32) -> Self {
        Self { i, j, k, l }
    }
```
```rust
    pub fn as_increasing(mut self) -> Self {
        // 모든 성분 비내림차순
        if self.j < self.i {
            std::mem::swap(&mut self.i, &mut self.j);
        }
        if self.k < self.i {
            std::mem::swap(&mut self.i, &mut self.k);
        }
        if self.l < self.i {
            std::mem::swap(&mut self.i, &mut self.l);
        }
        if self.k < self.j {
            std::mem::swap(&mut self.j, &mut self.k);
        }
        if self.l < self.j {
            std::mem::swap(&mut self.j, &mut self.l);
        }
        if self.l < self.k {
            std::mem::swap(&mut self.k, &mut self.l);
        }
        self
    }
```
```rust
    pub fn as_pairwise_increasing(mut self) -> Self {
        // (i,k) 먼저 비교, 같으면 (j,l) 비교
        if self.k < self.i {
            std::mem::swap(&mut self.i, &mut self.k);
            std::mem::swap(&mut self.j, &mut self.l);
        } else if self.i == self.k && self.l < self.j {
            std::mem::swap(&mut self.j, &mut self.l);
        }
        self
    }
```
```rust
    pub fn get(&self, idx: usize) -> i32 {
        match idx {
            0 => self.i,
            1 => self.j,
            2 => self.k,
            _ => self.l,
        }
    }
}
```
```rust
impl Ord for I4 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.i, self.j, self.k, self.l).cmp(&(other.i, other.j, other.k, other.l))
    }
}
```
```rust
impl PartialOrd for I4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
```rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U4 {
    pub i: u32,
    pub j: u32,
    pub k: u32,
    pub l: u32,
}
```
```rust
impl U4 {
    pub const UNSET: Self = Self {
        i: u32::MAX,
        j: u32::MAX,
        k: u32::MAX,
        l: u32::MAX,
    };
```
```rust
    pub const ZERO: Self = Self {
        i: 0,
        j: 0,
        k: 0,
        l: 0,
    };

```
```rust
    pub fn new(i: u32, j: u32, k: u32, l: u32) -> Self {
        Self { i, j, k, l }
    }
```
```rust
    pub fn as_increasing(mut self) -> Self {
        if self.j < self.i {
            std::mem::swap(&mut self.i, &mut self.j);
        }
        if self.k < self.i {
            std::mem::swap(&mut self.i, &mut self.k);
        }
        if self.l < self.i {
            std::mem::swap(&mut self.i, &mut self.l);
        }
        if self.k < self.j {
            std::mem::swap(&mut self.j, &mut self.k);
        }
        if self.l < self.j {
            std::mem::swap(&mut self.j, &mut self.l);
        }
        if self.l < self.k {
            std::mem::swap(&mut self.k, &mut self.l);
        }
        self
    }
```
```rust
    pub fn as_pairwise_increasing(mut self) -> Self {
        if self.k < self.i {
            std::mem::swap(&mut self.i, &mut self.k);
            std::mem::swap(&mut self.j, &mut self.l);
        } else if self.i == self.k && self.l < self.j {
            std::mem::swap(&mut self.j, &mut self.l);
        }
        self
    }
}
```
```rust
impl Ord for U4 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.i, self.j, self.k, self.l).cmp(&(other.i, other.j, other.k, other.l))
    }
}
```
```rust
impl PartialOrd for U4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
```rust

/// (n-i, i) 이항계수 = (n)! / ((n-i)! i!) 가 아니라,
/// 여기서는 원식에 맞춰 C(n-i + i, i) = C(n, i)와 동일.
/// 다만 원 C++ 구현은 ON_BinomialCoefficient(n-i, i)를 사용했음.
/// 그 값은 (n-i + i)! / ((n-i)! i!) = C(n, i) 와 동일하므로 이렇게 구현.
```
```rust
#[inline]
fn binom_ni_i(a: usize, b: usize) -> f64 {
    // 반환: C(a+b, b)
    let n = a + b;
    let k = b.min(n - b);
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
#[allow(clippy::many_single_char_names)]
/// 몫 규칙 (1변수) 평가: X(t)/W(t) 의 (최대) n차 도함수들을 입력으로 받아
/// f(t)=X/W 의 도함수들로 제자리 변환합니다.
///
/// # 인자
/// - `dim`: 벡터 차원 (f, X 의 좌표 수)
/// - `der_count`: 최대 미분 차수 n (>=0)
/// - `v_stride`: 각 (도함수) 블록의 stride (>= dim+1)
/// - `v`: 길이 (der_count+1) * v_stride 의 버퍼.
///   - 입력: [ X, W, X', W', X'', W'', ... ] 순으로 저장 (각 블록은 첫 dim 성분이 X, 마지막 1 성분이 W)
///   - 출력: X 블록이 f 로, W 블록은 w 로 나눠진 값이 됩니다.
///
/// # 수식
/// 몫 규칙의 일반형:
/// f = X/W,  n차 도함수는
/// ```text
/// f^{(n)} = (X^{(n)}/W)  -  sum_{i=0}^{n-1} C(n,i) * (W^{(n-i)}/W) * f^{(i)}
/// ```
/// 여기서 `C(n,i)`는 이항계수.
///
/// # 반환
/// - `true`: v[dim] = W != 0 인 경우 정상 동작
/// - `false`: 초기 W == 0 인 경우
pub fn evaluate_quotient_rule(
    dim: usize,
    der_count: usize,
    v_stride: usize,
    v: &mut [f64],
) -> bool {
    // 기본 유효성 체크
    if dim == 0 || v_stride < dim + 1 || v.len() < (der_count + 1) * v_stride {
        return false;
    }

    // 1) 전체를 w로 나누기 (동차 정규화)
    let w = v[dim];
    if w == 0.0 {
        return false;
    }
    let invw = 1.0 / w;
    // (der_count+1)개 블록 전체 스칼링
    for blk in 0..=der_count {
        let base = blk * v_stride;
        for j in 0..v_stride {
            v[base + j] *= invw;
        }
    }

    // 2) 1차 도함수: F' = X'/w + ( -W'/w ) * F
    if der_count >= 1 {
        let f0 = 0 * v_stride; // F
        let f1 = 1 * v_stride; // X'/w, W'/w
        let ws = -v[f1 + dim]; // -W'/w
        for j in 0..dim {
            v[f1 + j] += ws * v[f0 + j];
        }

        // 3) 2차 도함수: F'' = X''/w + ( -W''/w )F + 2( -W'/w )F'
        if der_count >= 2 {
            let f2 = 2 * v_stride; // X''/w, W''/w
            let w2 = -v[f2 + dim]; // -W''/w
            let ws = -v[f1 + dim]; // -W'/w (다시 읽음)
            for j in 0..dim {
                v[f2 + j] += w2 * v[0 * v_stride + j] + 2.0 * ws * v[f1 + j];
            }

            // 4) n>=3 일반식:
            //    F^{(n)} += - sum_{i=0}^{n-1} C(n-i, i) * (W^{(n-i)}/w) * F^{(i)}
            for n in 3..=der_count {
                let f_n = n * v_stride;
                // 누적 임시(대여 충돌 방지 + 수치 안정)
                let mut acc = vec![0.0f64; dim];

                for i in 0..n {
                    let choose = binom_ni_i(n - i, i);
                    let w_term = -choose * v[(n - i) * v_stride + dim]; // -(C(n-i,i)*W^{(n-i)}/w)
                    let f_i = i * v_stride; // F^{(i)}는 앞에서 차례대로 이미 만들어져 있음
                    for j in 0..dim {
                        acc[j] += w_term * v[f_i + j];
                    }
                }
                // 누적 합을 마지막에 적용
                for j in 0..dim {
                    v[f_n + j] += acc[j];
                }
            }
        }
    }

    true
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
pub fn evaluate_quotient_rule2(
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
pub fn evaluate_quotient_rule3(
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
    let invw = 1.0 / w0;
    for off in (0..need).step_by(v_stride) {
        for j in 0..v_stride {
            v[off + j] *= invw;
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
/// 주곡률(Second fundamental form l,m,n 입력형)
///
/// # 수식
/// 제1기본형: e=⟨S_u,S_u⟩, f=⟨S_u,S_v⟩, g=⟨S_v,S_v⟩
/// 제2기본형: l=⟨S_{uu},N⟩, m=⟨S_{uv},N⟩, n=⟨S_{vv},N⟩
/// Shape operator 의 행렬 표현(매개기저 {S_u,S_v}):
/// ```text
/// A = [a b; c d] with
///   a = (g l - f m)/J,  b = (g m - f n)/J
///   c = (e m - f l)/J,  d = (e n - f m)/J
/// J = e g - f^2
/// ```
/// 고유값 k1,k2 가 주곡률. tr(A)=2H, det(A)=K.
/// 고유벡터를 2D에서 구해 3D로 선형결합: Ki = α S_u + β S_v, 정규화 및 N과의 수직성, 상호수직성 보정.
pub fn ev_principal_curvatures_from_second_form(
    ds: Vector3D,
    dt: Vector3D,
    l: f64,
    m: f64,
    n: f64,
    nrm: Vector3D, // 단위법선
) -> Option<(f64, f64, f64, f64, Vector3D, Vector3D)> {
    let e = ds.dot(&ds);
    let f = ds.dot(&dt);
    let g = dt.dot(&dt);
    let jac = e * g - f * f;
    if !(jac != 0.0) {
        return None;
    }
    let inv_j = 1.0 / jac;
    let gauss = (l * n - m * m) * inv_j;
    let trace2 = (g * l - 2.0 * f * m + e * n) * inv_j; // 2H
    let mean = 0.5 * trace2;

    // 특성방정식 k^2 - trace*k + det = 0
    let det = gauss;
    let tr = trace2;
    let disc = tr * tr - 4.0 * det;

    let (k1, k2) = if disc >= 0.0 {
        let s = disc.sqrt();
        // 수치안정: 더 큰 절대값을 k1 로
        let r1 = 0.5 * (tr + s);
        let r2 = 0.5 * (tr - s);
        if r1.abs() >= r2.abs() {
            (r1, if r1 != 0.0 { det / r1 } else { r2 })
        } else {
            (det / r2, r2)
        }
    } else {
        // 수치적으로 매우 근접한 타원점 – 0 가까이 처리
        (0.0, 0.0)
    };

    // 2×2 행렬
    let a = (g * l - f * m) * inv_j;
    let b = (g * m - f * n) * inv_j;
    let c = (e * m - f * l) * inv_j;
    let d = (e * n - f * m) * inv_j;

    // 고유벡터: (A - kI)[x;y]=0  → 평균화 방법
    fn eigen_dir(a: f64, b: f64, c: f64, d: f64, k: f64) -> (f64, f64) {
        // (a-k, b) ~ ( -y, x )
        let x1 = a - k;
        let y1 = b;
        let x2 = c;
        let y2 = d - k;
        // 두 벡터 평균의 수직 벡터
        let x = x1 + x2;
        let y = y1 + y2;
        (-y, x)
    }

    let (x1, y1) = eigen_dir(a, b, c, d, k1);
    let mut k_1 = ds * y1 + dt * x1;
    let len1 = k_1.length();
    if len1 > 0.0 {
        k_1 = k_1 / len1;
    }
    let (x2, y2) = eigen_dir(a, b, c, d, k2);
    let mut k_2 = ds * y2 + dt * x2;
    let len2 = k_2.length();
    if len2 > 0.0 {
        k_2 = k_2 / len2;
    }

    // 수직/직교 보정
    // N에 수직 보정
    let _eps = 1e-6;
    if (k_1.dot(&nrm)).abs() > 1e-4 || (k_2.dot(&nrm)).abs() > 1e-4 {
        // 보정: K1 = normalize(K2 × N), K2 = N × K1
        if len1 < len2 {
            k_1 = (k_2.cross(&nrm)).unitize();
        } else {
            k_2 = (nrm.cross(&k_1)).unitize();
        }
    }
    // 상호 직교
    if (k_1.dot(&k_2)).abs() > 1e-4 {
        k_2 = (nrm.cross(&k_1)).unitize();
    }

    Some((gauss, mean, k1, k2, k_1, k_2))
}
```
```rust

/// 주곡률(두 번째 형식 직접 계산용) – Dss, Dst, Dtt 로부터 l,m,n = ⟨…,N⟩ 산출 뒤 호출
pub fn ev_principal_curvatures(
    ds: Vector3D,
    dt: Vector3D,
    dss: Vector3D,
    dst: Vector3D,
    dtt: Vector3D,
    nrm: Vector3D,
) -> Option<(f64, f64, f64, f64, Vector3D, Vector3D)> {
    let l = nrm.dot(&dss);
    let m = nrm.dot(&dst);
    let n = nrm.dot(&dtt);
    ev_principal_curvatures_from_second_form(ds, dt, l, m, n, nrm)
}

```
```rust
/// 법선곡률 벡터
///
/// 주어진 단위 접선 T 방향의 **곡면 상 곡선**의 2차 도함수를 이용해
/// 곡률벡터 K 를 평가하고, N 방향 성분만 취합니다.
/// 일반적으로
/// ```text
/// γ(t) = S(u0 + a t, v0 + b t)
/// γ'(0) = a S_u + b S_v  ≡  T
/// γ''(0) = a^2 S_{uu} + 2ab S_{uv} + b^2 S_{vv}
/// 곡률벡터 K = proj_{N} ( κ_n N ) = (K·N) N
/// ```
pub fn normal_curvature(
    s10: Vector3D,
    s01: Vector3D,
    s20: Vector3D,
    s11: Vector3D,
    s02: Vector3D,
    unit_normal: Vector3D,
    unit_tangent: Vector3D,
) -> Vector3D {
    // T = a S_u + b S_v 를 푸는 대신, a,b 를 최소제곱으로 계산
    // 여기서는 간단히 Gram 시스템을 푼다.
    // [⟨Su,Su⟩ ⟨Su,Sv⟩][a] = [⟨Su,T⟩]
    // [⟨Sv,Su⟩ ⟨Sv,Sv⟩][b]   [⟨Sv,T⟩]
    let e = s10.dot(&s10);
    let f = s10.dot(&s01);
    let g = s01.dot(&s01);
    let det = e * g - f * f;
    if det == 0.0 {
        return Vector3D::zero();
    }
    let rhs_a = s10.dot(&unit_tangent);
    let rhs_b = s01.dot(&unit_tangent);
    let a = (g * rhs_a - f * rhs_b) / det;
    let b = (-f * rhs_a + e * rhs_b) / det;

    let d2 = s20 * (a * a) + s11 * (2.0 * a * b) + s02 * (b * b);

    // 곡률 벡터 K: K = γ''(0) 의 법선 성분
    let _t_vec = unit_tangent; // 필요 시 반환 확인용
    // N·K
    let kn = d2.dot(&unit_normal);
    unit_normal * kn
}

```
```rust
/// 유리/비유리 점열의 polyline 길이.
///
/// P: 점열 버퍼, 각 점은 앞 dim 성분(좌표) + (is_rat 이면) 마지막 1 성분(weight)
pub fn get_polyline_length(
    dim: usize,
    is_rat: bool,
    count: usize,
    stride: usize,
    p: &[f64],
) -> Option<f64> {
    if dim < 1 || count < 2 {
        return None;
    }
    let need = if is_rat { dim + 1 } else { dim };
    let stride = if stride == 0 { need } else { stride };
    if stride < need {
        return None;
    }
    if p.len() < (count - 1) * stride + need {
        return None;
    }

    let mut length = 0.0;
    if is_rat {
        let mut i = 1;
        let mut p1 = 0usize;
        let mut w1 = p[p1 + dim];
        if w1 == 0.0 {
            return None;
        }
        w1 = 1.0 / w1;

        while i < count {
            let p0 = p1;
            p1 += stride;
            let w0 = w1;
            w1 = p[p1 + dim];
            if w1 == 0.0 {
                return None;
            }
            w1 = 1.0 / w1;

            let mut dd = 0.0;
            for j in 0..dim {
                let d = w0 * p[p0 + j] - w1 * p[p1 + j];
                dd += d * d;
            }
            length += dd.sqrt();
            i += 1;
        }
    } else {
        let mut i = 1;
        let mut p1 = 0usize;
        while i < count {
            let p0 = p1;
            p1 += stride;
            let mut dd = 0.0;
            for j in 0..dim {
                let d = p[p1 + j] - p[p0 + j];
                dd += d * d;
            }
            length += dd.sqrt();
            i += 1;
        }
    }
    Some(length)
}

```
```rust
// src/math_extra/num.rs
pub fn on_max_f64(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else if b > a {
        b
    } else if b != b {
        a
    } else {
        b
    }
}
```
```rust
pub fn on_min_f64(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else if b < a {
        b
    } else if b != b {
        a
    } else {
        b
    }
}
```
```rust
pub fn on_max_f32(a: f32, b: f32) -> f32 {
    if a >= b {
        a
    } else if b > a {
        b
    } else if b != b {
        a
    } else {
        b
    }
}
```
```rust
pub fn on_min_f32(a: f32, b: f32) -> f32 {
    if a <= b {
        a
    } else if b < a {
        b
    } else if b != b {
        a
    } else {
        b
    }
}
```
```rust
pub fn on_max_i32(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}
```
```rust
pub fn on_min_i32(a: i32, b: i32) -> i32 {
    if a <= b { a } else { b }
}

```
```rust
/// 32-bit round with overflow-safe saturation (C 코드 동작과 동일)
pub fn on_round_i32(x: f64) -> i32 {
    if x.abs() < 2_147_483_647.0 {
        if x >= 0.0 {
            (x + 0.5) as i32
        } else {
            -((0.5 - x) as i32)
        }
    } else if x.abs() < 2_147_483_647.5 {
        if x < 0.0 {
            -2_147_483_647
        } else {
            2_147_483_647
        }
    } else {
        // NaN, inf 등은 0, 아니면 포화
        if !x.is_finite() {
            0
        } else if x > 0.0 {
            2_147_483_647
        } else {
            -2_147_483_647
        }
    }
}

```
```rust
/// 보간: z=(1-t)x + t y, [0,1] 내부에선 clamp
pub fn lerp(t: f64, x: f64, y: f64) -> f64 {
    if x == y && t == t {
        return x;
    }
    let mut z = (1.0 - t) * x + t * y;
    if x < y {
        if z < x && t >= 0.0 {
            z = x;
        } else if z > y && t <= 1.0 {
            z = y;
        }
    } else if x > y {
        if z < y && t >= 0.0 {
            z = y;
        } else if z > x && t <= 1.0 {
            z = x;
        }
    }
    z
}

```
```rust
/// 이진 GCD (binary GCD)
pub fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let mut s = 0;
    while a != 0 && b != 0 {
        if a == b {
            return a << s;
        }
        if a & 1 == 1 {
            if b & 1 == 1 {
                if a > b {
                    a = (a - b) >> 1;
                } else {
                    let t = a;
                    a = (b - a) >> 1;
                    b = t;
                }
            } else {
                b >>= 1;
            }
        } else if b & 1 == 1 {
            a >>= 1;
        } else {
            a >>= 1;
            b >>= 1;
            s += 1;
        }
    }
    if a == 0 { b << s } else { a << s }
}

```
```rust
/// LCM with overflow check (0 if overflow)
pub fn lcm_u32(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let g = gcd_u32(a, b);
    let aa = a / g;
    if (aa as u64) * (b as u64) > (u32::MAX as u64) / (g as u64) {
        0
    } else {
        aa * b * g
    }
}

```
```rust
/// 공차 기준 비교/동등
pub fn are_equal(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}
```
```rust
pub fn is_zero(a: f64, tol: f64) -> bool {
    a.abs() < tol
}
```
```rust
pub fn compare(a: f64, b: f64, tol: f64) -> i32 {
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
pub fn are_between(a: f64, b: f64, c: f64) -> bool {
    ((a < c) && (c < b)) || ((b < c) && (c < a))
}
```
```rust
pub fn greater_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a > b) && !are_equal(a, b, tol)
}
```
```rust
pub fn geq(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a > b) || are_equal(a, b, tol)
}
```
```rust
pub fn less_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a < b) && !are_equal(a, b, tol)
}
```
```rust
pub fn leq(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a < b) || are_equal(a, b, tol)
}
```
```rust
pub fn is_infinite(v: f64) -> bool {
    let max = f64::MAX;
    !(-max <= v && v <= max)
}

```
```rust
#[derive(Clone, Copy)]
struct Mat3([[f64; 3]; 3]);
impl Mat3 {
    fn eye() -> Self {
        Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
```
```rust
    fn col(&self, j: usize) -> [f64; 3] {
        [self.0[0][j], self.0[1][j], self.0[2][j]]
    }

```
```rust
    #[allow(unused)]
    fn set_col(&mut self, j: usize, v: [f64; 3]) {
        self.0[0][j] = v[0];
        self.0[1][j] = v[1];
        self.0[2][j] = v[2];
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
pub fn tridiag_ql_implicit(
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
pub fn sym3_eigen(
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
    let mut v = Mat3::eye();
    if !tridiag_ql_implicit(&mut dvals, &mut evals, Some(&mut v.0)) {
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


/* ---------------------------- 샘플링 ---------------------------- */
pub fn get_sampling_2d(datas: &[Point2D]) -> Vec<Point2D> {
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

pub fn get_sampling_3d(datas: &[Point3D]) -> Vec<Point3D> {
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

/* ---------------------------- 기초 유틸 ---------------------------- */

pub fn offset_point(p1: Point3D, p2: Point3D, length: f64) -> Point3D {
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

pub fn sort_doubles(arr: &mut [f64], increasing: bool) {
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

pub fn cull_doubles(arr: &mut Vec<f64>, mut tol: f64) -> usize {
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

/* ---------------------------- 2D 세그먼트/포함 관계 ---------------------------- */

#[allow(unused)]
#[inline]
fn dot2(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * bx + ay * by
}
```
```rust

#[inline]
fn cross2(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}
```
```rust

pub fn is_point_on_segment_2d(test: Point2D, p0: Point2D, p1: Point2D) -> bool {
    is_point_on_segment_2d_with_domain(test, p0, p1, p0.distance(&p1))
}
```
```rust

pub fn is_point_on_segment_2d_with_domain(
    test: Point2D,
    p0: Point2D,
    p1: Point2D,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointStatus {
    Inside,
    Outside,
    Onto,
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
        if is_point_on_segment_2d_with_domain(test, w[0], w[1], diag) {
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

pub fn point_in_rect_open(test: Point2D, ll: Point2D, ur: Point2D) -> bool {
    test.x > ll.x && test.x < ur.x && test.y > ll.y && test.y < ur.y
}

```
```rust
/* -------- Point-in-Triangle Tests (2D) -------- */

pub fn point_in_triangle_2d(test: Point2D, a: Point2D, b: Point2D, c: Point2D) -> bool {
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
    u > 0.0 && v > 0.0 && (u + v) < 1.0
}
```
```rust

/* -------- Polygon Geometry Utilities -------- */

#[inline]
fn on_cross_z(a: Point2D, b: Point2D, c: Point2D) -> f64 {
    let ux = b.x - a.x;
    let uy = b.y - a.y;
    let vx = c.x - a.x;
    let vy = c.y - a.y;
    cross2(ux, uy, vx, vy)
}
```
```rust

pub fn on_is_polygon_convex(vertices: &[Point2D]) -> bool {
    let n = vertices.len();
    if n < 5 {
        return true;
    } // C++ 소스와 동일 정책

    let mut last_sign = 0i32;
    for i in 0..(n - 1) {
        let i2 = if i > 0 { i - 1 } else { n - 2 };
        let i3 = i + 1;
        let mut u = vertices[i2] - vertices[i];
        let mut v = vertices[i] - vertices[i3];
        u.normalize();
        v.normalize();
        let cross = cross2(u.x, u.y, v.x, v.y);
        if cross.abs() > 1e-12 {
            let s = if cross > 0.0 { 1 } else { -1 };
            if last_sign != 0 && s != last_sign {
                return false;
            }
            last_sign = s;
        }
    }
    true
}
```
```rust

/* ---------------------------- 컨벡스 헐 ---------------------------- */

fn is_left_of(a: &Point2D, b: &Point2D) -> bool {
    a.x < b.x || (a.x == b.x && a.y < b.y)
}
```
```rust

pub fn quickhull_2d(v: Vec<Point2D>) -> Vec<Point2D> {
    if v.len() <= 3 {
        return v;
    }

    let a = *v
        .iter()
        .min_by(|p, q| {
            is_left_of(p, q)
                .then_some(std::cmp::Ordering::Less)
                .unwrap_or(std::cmp::Ordering::Greater)
        })
        .unwrap();
    let b = *v
        .iter()
        .max_by(|p, q| {
            is_left_of(p, q)
                .then_some(std::cmp::Ordering::Less)
                .unwrap_or(std::cmp::Ordering::Greater)
        })
        .unwrap();

    fn dist(a: Point2D, b: Point2D, p: Point2D) -> f64 {
        ((b.x - a.x) * (a.y - p.y) - (b.y - a.y) * (a.x - p.x)).abs() / ((b - a).length())
    }
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
    fn side(a: Point2D, b: Point2D, p: Point2D) -> f64 {
        on_cross_z(a, b, p)
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

pub fn monotone_chain_2d(mut v: Vec<Point2D>) -> Vec<Point2D> {
    if v.len() <= 1 {
        return v;
    }
    v.sort_by(|a, b| {
        if is_left_of(a, b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut lower: Vec<Point2D> = Vec::new();
    for &p in &v {
        while lower.len() >= 2 {
            let n = lower.len();
            if on_cross_z(lower[n - 2], lower[n - 1], p) >= 0.0 {
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
            if on_cross_z(upper[n - 2], upper[n - 1], p) >= 0.0 {
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
/* ---------------------------- Color Conversion ---------------------------- */

pub fn int_to_color_bytes(argb: i32) -> [u8; 4] {
    [
        ((argb >> 24) & 0xFF) as u8,
        ((argb >> 16) & 0xFF) as u8,
        ((argb >> 8) & 0xFF) as u8,
        (argb & 0xFF) as u8,
    ]
}
```
```rust

pub fn color_bytes_to_int(a: u8, r: u8, g: u8, b: u8) -> i32 {
    ((a as i32) << 24) | ((r as i32) << 16) | ((g as i32) << 8) | (b as i32)
}
```
```rust

/* ---------------------------- 조합/팩토리얼 ---------------------------- */

#[allow(unused)]
pub fn factorial_u128(n: usize) -> Option<u128> {
    let mut acc: u128 = 1;
    for i in 2..=n {
        acc = acc.checked_mul(i as u128)?;
    }
    Some(acc)
}
```
```rust

#[allow(unused)]
pub fn binomial_via_factorial_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let nf = factorial_u128(n).unwrap_or(0) as f64;
    let kf = factorial_u128(k).unwrap_or(0) as f64;
    let nk = factorial_u128(n - k).unwrap_or(0) as f64;
    nf / (kf * nk)
}
```
```rust

#[allow(unused)]
pub fn factorial(n: usize) -> i64 {
    if n <= 1 {
        1
    } else {
        (n as i64) * factorial(n - 1)
    }
}
```
```rust

// pub fn binomial(n: usize, k: usize) -> f64 {
//     // For large n, a more stable method is recommended (Pascal or log-gamma). This follows the original implementation.
//     (factorial(n) as f64) / ((factorial(k) as f64) * (factorial(n-k) as f64))
// }

#[allow(unused)]
pub fn on_binomial(n: usize, k: usize) -> f64 {
    if k == 0 || k == n {
        return 1.0;
    }
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut r = 1.0f64;
    for i in 0..k {
        // r *= (n - i) / (i + 1)
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}
```
```rust

#[allow(unused)]
pub fn binomial_u128(n: usize, k: usize) -> Option<u128> {
    if k == 0 || k == n {
        return Some(1);
    }
    if k > n {
        return Some(0);
    }
    let k = k.min(n - k);

    let mut num_factors: Vec<u128> = (0..k).map(|i| (n - i) as u128).collect();
    let mut den_factors: Vec<u128> = (1..=k).map(|i| i as u128).collect();

    // 약분: 분모를 하나씩 돌며 분자들에서 최대공약수로 나눠 떨어뜨림
    for d in &mut den_factors {
        if *d == 1 {
            continue;
        }
        for nf in &mut num_factors {
            if *d == 1 {
                break;
            }
            let g = gcd_u128(*nf, *d);
            if g > 1 {
                *nf /= g;
                *d /= g;
            }
        }
        if *d != 1 {
            // At this point, if the numerator can no longer be reduced, the value may risk becoming large.
            // However, if the subsequent multiplication causes overflow, return None.
        }
    }

    let mut acc: u128 = 1;
    for nf in num_factors {
        acc = acc.checked_mul(nf)?;
    }
    // 남은 den_factors는 보통 1이 됨. 아니라면 나눗셈 시도.
    for d in den_factors {
        if d != 1 {
            acc = acc.checked_div(d)?; // 안전한 케이스(정확히 나눠 떨어져야 함)
        }
    }
    Some(acc)
}
```
```rust

#[inline]
fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
```
```rust
/* ---------------------------- 기타 몇 가지 수학 유틸 ---------------------------- */

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}
```
```rust
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

```
```rust
pub fn determinant3_vectors(v1: Point3D, v2: Point3D, v3: Point3D) -> f64 {
    v1.x * v2.y * v3.z - v1.x * v2.z * v3.y - v1.y * v2.x * v3.z
        + v1.y * v2.z * v3.x
        + v1.z * v2.x * v3.y
        - v1.z * v2.y * v3.x
}

```
```rust
pub fn is_point_inside_box(
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

#[allow(clippy::too_many_arguments)]


```
```rust
// Bernstein (3차) – 여러분 구현으로 바꾸세요.
fn on_bernstein_real(i: usize, n: usize, t: f64) -> f64 {
    // 간단 참고 구현 (n=3 가정 사용)
    let c = match (i, n) {
        (0, 3) => 1.0,
        (1, 3) => 3.0,
        (2, 3) => 3.0,
        (3, 3) => 1.0,
        _ => panic!("only n=3"),
    };
    c * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}

```
```rust
pub fn on_add_point_without_duplicate_vec(
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
pub fn on_compute_uv_parameters(
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
pub fn distance_to_bbox(p: Point3D, min: Point3D, max: Point3D) -> f64 {
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
pub fn get_outer_index_loops2d(loops: &[Vec<Point2D>]) -> i32 {
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
pub fn point_coincidence(
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
pub fn on_intersect_3d_lines(
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
    if denom.abs() <= EPS * a {
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
pub fn on_surface_tangent_vector_inversion(
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
/// f(t) = B'_{0,3}(t) + B'_{1,3}(t) - alpha = 0 의 빠른 해.
/// 3차에서는 B'들의 합이 6 t (t-1)이므로 해석해 가능.
/// 기본은 'lower branch'(t ∈ [0, 0.5])를 반환.
/// upper 브랜치가 필요하면 `upper_branch=true`로 호출.
pub fn on_solve_bezier_param_from_alpha_fast(alpha: f64, upper_branch: bool) -> f64 {
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
pub fn on_solve_bezier_param_from_alpha(alpha: f64) -> f64 {
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
pub fn on_uniform_knot_vector(p: i32, n: i32) -> Vec<f64> {
    let m = (n + p) as usize + 1;
    let mut kv = vec![0.0; m];
    for i in 0..=p as usize {
        kv[i] = 0.0;
        kv[(n as usize) + i] = 1.0;
    }
    let step = 1.0 / (n - p) as f64;
    for i in (p as usize)..(n as usize) {
        kv[i] = kv[p as usize] + step * (i - p as usize) as f64;
    }
    kv
}

```
```rust
pub fn on_are_points_close_to_segment(
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
pub fn on_compute_parameter_distribution(
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
pub fn on_chord_length_parameterization_lengths(
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
pub fn is_symmetric(a: &Matrix, n: usize, eps: f64) -> bool {
    for i in 0..n {
        for j in (i + 1)..n {
            if (*a.at(i as i32, j as i32) - *a.at(j as i32, i as i32)).abs() > eps {
                return false;
            }
        }
    }
    true
}

```
```rust
pub fn insert_value_into_sorted_array(value: f64, v: &mut Vec<f64>) -> bool {
    if v.is_empty() {
        return false;
    }
    if value < v[0] || value > v[v.len() - 1] {
        return false;
    }
    // 자리 확보
    v.push(value);
    let mut i = v.len() - 1;
    while i > 0 && v[i - 1] > value {
        v[i] = v[i - 1];
        i -= 1;
    }
    v[i] = value;
    true
}

```
```rust
pub fn insert_value_into_sorted_array_option(mut v: Vec<f64>, value: f64) -> Option<Vec<f64>> {
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
pub fn check_difference_significant(a: f64, b: f64, c: f64, diff_out: &mut f64) -> bool {
    let ac = a * c;
    let b2 = b * b;
    let diff = ac - b2;
    let cond = a > c * EPS && c > a * EPS && diff.abs() > ac.max(b2) * SQRT_EPS;
    *diff_out = diff;
    cond
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
    let mut diff = 0.0;
    if check_difference_significant(a, b, c, &mut diff) {
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
        if k > EPS {
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
#[inline]
pub fn on_normalize_angle_to_2pi(mut a: f64) -> f64 {
    if a >= 0.0 && a <= 2.0 * PI {
        return a;
    }
    let twopi = 2.0 * PI;
    a = a - (a / twopi).floor() * twopi;
    if a < 0.0 {
        0.0
    } else if a > twopi {
        twopi
    } else {
        a
    }
}

```
```rust
#[inline]
pub fn on_copy_sign(x: f64, y: f64) -> f64 {
    if y < 0.0 { -x.abs() } else { x.abs() }
}

```
```rust
#[inline]
pub fn on_update_min_max_quick_xy(x: f64, y: f64, min: &mut Point3D, max: &mut Point3D) {
    if x < min.x {
        min.x = x;
    } else if x > max.x {
        max.x = x;
    }
    if y < min.y {
        min.y = y;
    } else if y > max.y {
        max.y = y;
    }
}
```
```rust
#[inline]
pub fn on_update_min_max_quick_xyz(x: f64, y: f64, z: f64, min: &mut Point3D, max: &mut Point3D) {
    on_update_min_max_quick_xy(x, y, min, max);
    if z < min.z {
        min.z = z;
    } else if z > max.z {
        max.z = z;
    }
}
```
```rust
#[inline]
pub fn on_update_min_max_quick_p(p: Point3D, min: &mut Point3D, max: &mut Point3D) {
    on_update_min_max_quick_xyz(p.x, p.y, p.z, min, max);
}

```
```rust
#[inline]
fn is_valid_f64(x: f64) -> bool {
    x.is_finite()
}

```
```rust
// ===============================================
// 6) 선형-선형 최근접/교차 파라미터 (3D)
//    (두 직선의 파라미터 t, s)
// ===============================================
pub fn on_try_get_line_line_intersection_parameters(
    // line1: start = base + offset_scale * offset_dir
    ln1_offset_scale: f64,
    ln1_offset_dir: Vector3D,
    ln1_dir_scale: f64,
    ln1_dir_base: Vector3D,
    ln1_base_point: Point3D,
    // line2: start = line2_base_point, dir = line2_direction
    ln2_direction: Vector3D,
    ln2_base_point: Point3D,
) -> Option<(f64 /*s on line2*/, f64 /*t on line1*/)> {
    if !is_valid(&ln1_offset_dir) || !is_valid(&ln1_dir_base) {
        return None;
    }
    // line1 start/end 표현
    let l1_off = Vector3D::scale(&ln1_offset_dir, ln1_offset_scale);
    let l1_dir = Vector3D::sub(&Vector3D::scale(&ln1_dir_base, ln1_dir_scale), &l1_off);
    if Vector3D::is_tiny(&l1_dir) || Vector3D::is_tiny(&ln2_direction) {
        return None;
    }

    let ln1_start = Vector3D::add_point_vector(&ln1_base_point, &l1_off);

    // Δ = line1Start - line2BasePoint
    let d = Vector3D::sub_vector_point(&ln1_start, &ln2_base_point);

    // 보조식 (XY/XZ/YZ 평면 반복)
    let det_xy = ln2_direction.x * l1_dir.y - l1_dir.x * ln2_direction.y;
    let nt_xy = d.x * l1_dir.y - l1_dir.x * d.y;

    let det = det_xy;
    let near_zero = |v: f64| v.abs() <= 32.0 * EPS;

    let (numer_t, numer_s, determinant) = if !near_zero(det) {
        (nt_xy, ln2_direction.x * d.x - d.y * ln2_direction.y, det)
    } else {
        let det_xz = ln2_direction.x * l1_dir.z - l1_dir.x * ln2_direction.z;
        if !near_zero(det_xz) {
            let nt_xz = d.x * l1_dir.z - l1_dir.x * d.z;
            (nt_xz, ln2_direction.x * d.x - d.z * ln2_direction.z, det_xz)
        } else {
            let det_yz = ln2_direction.y * l1_dir.z - l1_dir.y * ln2_direction.z;
            if near_zero(det_yz) {
                return None; // 평행/퇴화
            }
            let nt_yz = d.y * l1_dir.z - l1_dir.y * d.z;
            (nt_yz, ln2_direction.y * d.y - d.z * ln2_direction.z, det_yz)
        }
    };

    let t = numer_t / determinant; // line1 param
    let s = -numer_s / determinant; // line2 param
    if !is_valid_f64(t) || !is_valid_f64(s) {
        return None;
    }

    // 잔차 체크
    let p1 = ln1_start + Vector3D::scale(&l1_dir, t);
    let p2 = ln2_base_point + Vector3D::scale(&ln2_direction, s).to_point();
    let r = p1 - p2.to_vector();
    let residual2 = r.length();
    let scale = 1.0 + d.x.abs().max(d.y.abs()).max(d.z.abs());
    let tol = ON_TOL8 * scale;
    if residual2 > tol * tol {
        return None;
    }

    Some((s, t))
}

```
```rust
// ===============================================
// 7) 경계 3변과 교차 여부(라인1 세그먼트 t∈[0,1])
// ===============================================
pub fn on_intersects_line_with_boundary_3edges(
    line1_offset_scale: f64,
    line1_offset_dir: Vector3D,
    line1_dir_scale: f64,
    line1_dir_base: Vector3D,
    line1_base_point: Point3D,
    edge_dir_a: Vector3D,
    corner_p: Point3D,
    edge_dir_b: Vector3D,
) -> bool {
    let in_seg = |v: f64| -> bool { v.is_finite() && v >= -ON_TOL8 && v <= 1.0 + ON_TOL8 };

    // edge A
    let (s1, t1) = on_try_get_line_line_intersection_parameters(
        line1_offset_scale,
        line1_offset_dir,
        line1_dir_scale,
        line1_dir_base,
        line1_base_point,
        edge_dir_a,
        corner_p,
    )
        .unwrap_or_else(|| (f64::NAN, f64::NAN));
    let hit_a_t = in_seg(t1);
    if hit_a_t && in_seg(s1) {
        return true;
    }

    // edge B
    let (s2, t2) = on_try_get_line_line_intersection_parameters(
        line1_offset_scale,
        line1_offset_dir,
        line1_dir_scale,
        line1_dir_base,
        line1_base_point,
        edge_dir_b,
        corner_p,
    )
        .unwrap_or_else(|| (f64::NAN, f64::NAN));
    let hit_b_t = in_seg(t2);
    if (hit_b_t && in_seg(s2))
        || (hit_a_t
        && hit_b_t
        && !s1.is_nan()
        && !s2.is_nan()
        && s2.signum() != s1.signum()
        && s1.min(s2) < 0.0
        && s1.max(s2) > 1.0)
    {
        return true;
    }

    // edge C
    let edge_c_base = Vector3D::add_point_vector(&corner_p, &edge_dir_b);
    let edge_c_dir = Vector3D::sub(&edge_dir_b, &edge_dir_a);
    if Vector3D::is_tiny(&edge_c_dir) {
        return false;
    }

    let (s3, t3) = on_try_get_line_line_intersection_parameters(
        line1_offset_scale,
        line1_offset_dir,
        line1_dir_scale,
        line1_dir_base,
        line1_base_point,
        edge_c_dir,
        edge_c_base.to_point(),
    )
        .unwrap_or_else(|| (f64::NAN, f64::NAN));

    let hit_c_t = in_seg(t3);

    (hit_c_t && in_seg(s3))
        || (hit_a_t
        && hit_c_t
        && !s1.is_nan()
        && !s3.is_nan()
        && s3.signum() != s1.signum()
        && s1.min(s3) < 0.0
        && s1.max(s3) > 1.0)
        || (hit_b_t
        && hit_c_t
        && !s2.is_nan()
        && !s3.is_nan()
        && s2.signum() != s3.signum()
        && s3.min(s2) < 0.0
        && s3.max(s2) > 1.0)
}

```
```rust
// ===============================================
// 8) 점-선분 부근 판정 / 가장 가까운 점
// ===============================================
pub fn on_point_on_edge(p: Point3D, a: Point3D, b: Point3D, tol: f64) -> bool {
    let ab = b - a;
    let ap = p - a;
    let denom = ab.length_squared();
    if denom <= EPS {
        return false;
    }
    let t = Point3D::dot(&ab, &ap) / denom;
    if t < -tol || t > 1.0 + tol {
        return false;
    }
    let closest = a + Point3D::scale(&ab, t);
    let d = p - closest;
    d.length() < tol
}

```
```rust
// ===============================================
// 9) 기준 방향과 실제 방향의 각도 편차
// ===============================================
pub fn on_compute_deviation_angle_from_direction(
    samples: &[Point3D],
    base_point: Point3D,
    ref_point: Point3D,
    dir_origin: Point3D,
    dir_vector: Vector3D,
    max_dist_scale: f64,
) -> f64 {
    if samples.is_empty() || Vector3D::is_tiny(&dir_vector) {
        return 0.0;
    }

    let mut min_base = f64::MAX;
    let mut max_ref = 0.0;
    let mut i_near = 0usize;
    let mut i_far = 0usize;

    for (i, sp) in samples.iter().enumerate() {
        let db = (*sp - base_point).length().abs();
        let dr = (*sp - ref_point).length().abs();
        if db < min_base {
            min_base = db;
            i_near = i;
        }
        if dr > max_ref {
            max_ref = dr;
            i_far = i;
        }
    }
    let max_dref = (samples[i_near] - ref_point)
        .length()
        .abs()
        .max((samples[i_far] - ref_point).length().abs());

    let offset = Vector3D::scale(&dir_vector, max_dref * max_dist_scale);
    let offset_point = dir_origin + offset.to_point();

    let actual_dir = on_unit_point(&(base_point - offset_point));
    let ref_dir = on_unit_point(&(base_point - dir_origin));

    // angle between (0..pi)
    let d = Point3D::dot(&actual_dir, &ref_dir).clamp(-1.0, 1.0);
    d.acos().abs()
}

```
```rust
fn on_unit_point(p: &Point3D) -> Point3D {
    let l = p.length();
    if l <= ON_EPSILON {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    } else {
        Point3D::scale(&p, 1.0 / l)
    }
}


```
```rust
fn on_unit_vector(v: &Vector3D) -> Vector3D {
    let l = v.length();
    if l <= ON_EPSILON {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    } else {
        Vector3D::scale(&v, 1.0 / l)
    }
}

```
```rust
// src/math/extra.rs
#[allow(dead_code)]
// on_calculate_arc_segments
/// Calculates the number of segments needed to divide a circle or arc based on a maximum chord length.
/// Returns (number of segments, arc length per segment)
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
        let _ = n.normalize();
    }
    n
}
```
```rust

// Clamp helpers
#[inline]
pub fn on_clamp01(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
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
// ------------------------------
// 기초 기하 유틸
// ------------------------------

pub fn on_project_to_segment(p: Point3D, a: Point3D, b: Point3D) -> (f64, Point3D, f64) {
    let ab = b - a;
    let len2 = Point3D::dot(&ab, &ab);
    if len2 <= 0.0 {
        let d2 = Point3D::distance_squared(p, a);
        return (0.0, a, d2);
    }
    let t = on_clamp01(Point3D::dot(&(p - a), &ab) / len2);
    let q = a + ab * t;
    let d2 = Point3D::distance_squared(p, q);
    (t, q, d2)
}

```
```rust
// ------------------------------
// 도메인 clamp/wrap
// ------------------------------

pub fn on_clamp_to_domain(mut x: f64, dom: Interval) -> f64 {
    if x < dom.min() {
        x = dom.min();
    }
    if x > dom.max() {
        x = dom.max();
    }
    x
}

```
```rust
pub fn on_wrap_to_domain(x: f64, dom: Interval) -> f64 {
    let l = dom.max() - dom.min();
    if !(l > 0.0) {
        return x;
    }
    let mut t = (x - dom.min()) % l;
    if t < 0.0 {
        t += l;
    }
    dom.min() + t
}

```
```rust
pub fn on_clamp_or_wrap(x: f64, dom: Interval, periodic: bool) -> f64 {
    if periodic {
        on_wrap_to_domain(x, dom)
    } else {
        on_clamp_to_domain(x, dom)
    }
}

```
```rust
// ------------------------------
// 선/평면 이동 (지정 방향으로 교차점 잡기)
// ------------------------------
pub fn on_move_point_to_plane(p: Point3D, pl: &Plane, mut w: Vector3D, eps: f64) -> Option<Point3D> {
    let n = pl.normal();
    let mut den = Vector3D::dot(&n, &w);
    if den.abs() < eps {
        // 폴백: 수직투영
        w = n;
        den = Vector3D::dot(&n, &w);
        if den.abs() < eps {
            return None;
        }
    }
    // τ = - plane_eval(P) / (n·W)
    let tau = -on_plane_eval(pl, p) / den;
    Some(p + w.to_point() * tau)
}

```
```rust
pub fn on_move_point_to_line(
    p: Point3D,
    l0: Point3D,
    l1: Point3D,
    w: Vector3D,
    eps: f64,
) -> Option<Point3D> {
    let d = l1 - l0;
    let den = Vector3D::dot(&w, &d.to_vector());
    if den.abs() < eps {
        // 최단거리 발로
        let l2 = Point3D::dot(&d, &d);
        if l2 <= eps {
            return None;
        }
        let s = Point3D::dot(&(p - l0), &d) / l2;
        return Some(l0 + d * s);
    }
    let s = Point3D::dot(&w.to_point(), &(p - l0)) / den;
    Some(l0 + d * s)
}

```
```rust
// ------------------------------
// PullClamp (가중/특이점 클램프)
// ------------------------------

#[derive(Debug, Clone, Copy)]
pub struct PullClamp {
    pub d_safe: f64,
    pub dmax_w: f64,
    pub dmin_w: f64,
    pub dmax_sing: f64,
}

```
```rust
pub fn on_clamp_pull_step(
    wk: f64,
    r: f64,
    pkp: f64,
    d_req: f64,
    w_min: f64,
    w_max: f64,
    eta: f64,
) -> PullClamp {
    let mut r_pull = PullClamp {
        d_safe: 0.0,
        dmax_w: f64::INFINITY,
        dmin_w: 0.0,
        dmax_sing: 0.0,
    };
    if wk <= 0.0 || r <= 0.0 || pkp <= 0.0 {
        return r_pull;
    }

    let c_up = (w_max / wk) - 1.0; // >=0
    let d_max_w = if c_up > 0.0 {
        (c_up * r * pkp) / (1.0 + c_up * r)
    } else {
        f64::INFINITY
    };

    let c_dn = (w_min / wk) - 1.0; // <=0
    let d_min_w = if c_dn < 0.0 {
        (c_dn * r * pkp) / (1.0 + c_dn * r)
    } else {
        0.0
    };

    let d_max_sing = (1.0 - eta) * pkp;

    r_pull.dmax_w = d_max_w;
    r_pull.dmin_w = d_min_w;
    r_pull.dmax_sing = d_max_sing;

    r_pull.d_safe = if d_req >= 0.0 {
        d_req.min(d_max_w).min(d_max_sing).max(0.0)
    } else {
        d_req.max(d_min_w)
    };
    r_pull
}

```
```rust
pub fn on_dist_to_plane(p: &Point3D, pl: &Plane) -> f64 {
    pl.distance_to_point(p).abs()
}
```
--- 

## 수학적 검증

핵심적인 수식과 알고리즘들을 범주별로 정리하고, 각각의 수학적 정당성을 검증.

## ✅ 1. 거리 및 중복 제거
### 🔹 on_distance_square(a, b)
- 계산: $\sqrt{(x_a-x_b)^2+(y_a-y_b)^2+(z_a-z_b)^2}$
- 검증: 유클리드 거리 공식이며, 3차원 공간에서 두 점 사이의 거리로 정확함
### 🔹 on_remove_duplicate_points
- 원리: 인접 거리 누적 → 상대/절대 오차 기준으로 중복 제거
- 검증: CAD/CG에서 널리 사용되는 거리 기반 필터링 방식

## ✅ 2. 베지어 곡선 관련
### 🔹 on_bernstein_f64(i, n, u)
- 계산: 베르스타인 기저함수 $B_i^n(u)={n \choose i}u^i(1-u)^{n-i}$
- 구현: 디캐슬조 알고리즘 기반의 재귀적 보간
- 검증: 수치적으로 안정적이며, 곡선 생성에 필수적인 기법
### 🔹 on_solve_bezier_t_from_alpha
- 목적: 주어진 α에 대해 t 역추정
- 방법: 뉴턴-랩슨 방식으로 반복 근사
- 검증: 수렴 조건과 오차 제어가 포함되어 있어 안정적

## ✅ 3. 평면과 삼각형 절단
### 🔹 tri_plane_cut
- 평면 방정식:

$$
ax+by+cz+d=0
$$

- 교차점 계산: 선형 보간 비율

$$
r=\frac{f_j}{a\cdot dx+b\cdot dy+c\cdot dz}
$$


- 검증: Constructive Solid Geometry(CSG)에서 표준 절단 알고리즘

## ✅ 4. 야코비안 및 법선 도함수
### 🔹 on_ev_jacobian(E, F, G)
- 계산:

$$
\det J=EG-F^2
$$

- 의미: 표면의 면적 요소, 야코비안 행렬의 행렬식
- 검증: 미분기하학에서 널리 사용되는 정의
### 🔹 on_ev_normal_partials(ds, dt, dss, dst, dtt)
- 계산:

$$
N=\frac{ds\times dt}{|ds\times dt|},\quad N'=\frac{V'}{|V|}-\frac{(V\cdot V')}{|V|^3}V
$$

- 검증: 벡터 정규화의 도함수 공식에 기반한 정확한 수식

## ✅ 5. 접선 및 곡률
### 🔹 on_ev_tangent(d1, d2)
- 정상:

$$
T=\frac{d1}{|d1|}
$$


- 퇴화:

$$
T\approx \pm \frac{d2}{|d2|}
$$


- 검증: 병렬 벡터의 극한 근사로서 L'Hôpital의 원리 적용
### 🔹 on_ev_curvature(d1, d2)
- 계산:

$$
T=\frac{d1}{|d1|},\quad K=\frac{d2-(d2\cdot T)T}{|d1|^2}
$$

- 검증: 곡률 벡터의 표준 정의이며, 정당함

## ✅ 6. 파라미터 언랩 및 공차
### 🔹 on_unwrap_around_center, on_unwrap_to_face_range
- 원리: 주기 함수의 연속성 보존을 위한 언랩
- 검증: 각도/위상 처리에서 널리 사용되는 방식
### 🔹 get_parameter_tolerance(t0, t1, t)
- 계산:

$$
dt=8\sqrt{\epsilon }(t1-t0)+\epsilon (|t0|+|t1|)
$$

- 검증: IEEE-754 기반 수치 안정성 고려

## ✅ 7. 단위 법선 근사
### 🔹 on_ev_normal(limit_dir, du, dv, duu, duv, dvv)
- 퇴화 시 근사:

$$
N\approx a\cdot (du\times duv+duu\times dv)+b\cdot (du\times dvv+duv\times dv)
$$

- 검증: 극한 방향에서의 법선 근사로서 미분기하학적으로 정당함

## ✅ 8. Pullback (벡터를 파라미터 공간으로 끌어오기)
### 🔹 pullback_3d_vector(vector, distance, ds, dt, dss, dst, dtt)
- 계산:

$$
\vec {v}\approx \alpha \cdot ds+\beta \cdot dt
$$

- 검증: 표면 위의 벡터를 파라미터 공간으로 투영하는 표준 방식

## ✅ 결론
모든 수식은 미분기하학, 수치해석, CAD/CG 알고리즘에서 널리 검증된 방식이며, 수학적으로 정당합니다. 특히:
- 야코비안 행렬식
- 법선 도함수
- 곡률 벡터
- 베지어 기저함수
- 평면 절단 및 보간
- 파라미터 공차
이들은 모두 수치적 안정성과 수학적 정확성을 고려하여 구현되어 있습니다.


수학 함수들이 포함되어 있어서, 이를 범주별로 나눠서 단계적으로 테스트 코드를 작성하는 것이 가장 효율적입니다.  
아래는 전체 테스트 계획과 첫 번째 단계의 코드 예시입니다.

| 단계 | 범주                     | 주요 함수 목록                                                                 |
|------|--------------------------|--------------------------------------------------------------------------------|
| 1️⃣   | 거리 및 중복 제거         | on_distance_square, on_remove_duplicate_points, <br> on_remove_duplicate_points_simple, on_remove_duplicate_points_with_tangents |
| 2️⃣   | 베지어 곡선              | on_bernstein_f64, on_solve_bezier_t_from_alpha                                |
| 3️⃣   | 평면 절단                | tri_plane_cut, on_get_ratio, on_interpolate_vertex                            |
| 4️⃣   | 야코비안 및 법선 도함수  | on_ev_jacobian, on_ev_normal_partials, on_ev_normal                           |
| 5️⃣   | 접선 및 곡률             | on_ev_tangent, on_ev_curvature                                                |
| 6️⃣   | 파라미터 공차 및 언랩    | get_parameter_tolerance, on_unwrap_around_center, on_unwrap_to_face_range     |
| 7️⃣   | 기타 유틸리티            | on_update_minmax_quick_xyz, on_try_get_perpendicular_vector, pullback_3d_vector |

---

## 🧪 1단계 테스트 코드: 거리 및 중복 제거
```rust
#[cfg(test)]
mod tests_distance_dedup {
    use super::*;

    #[test]
    fn test_on_distance_square() {
        let a = Point3D::new(0.0, 0.0, 0.0);
        let b = Point3D::new(3.0, 4.0, 0.0);
        let dist = on_distance_square(&a, &b);
        assert!((dist - 5.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_on_remove_duplicate_points_simple() {
        let pts = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.000001, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        ];
        let result = on_remove_duplicate_points_simple(&pts, 1e-4);
        assert_eq!(result.len(), 2);
    }
```
```rust
    #[test]
    fn test_on_remove_duplicate_points_with_tangents() {
        let pts = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.000001, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        ];
        let tans = vec![
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ];
        let (new_pts, new_tans) = on_remove_duplicate_points_with_tangents(&pts, &tans, 0.0, 1e-4);
        assert_eq!(new_pts.len(), 2);
        assert_eq!(new_tans.len(), 2);
    }
}
```

---

# 베지어 곡선 관련 함수들에 대한 테스트 코드를 작성.

## ✅ 2단계 테스트 코드: 베지어 곡선 관련
```rust
#[cfg(test)]
mod tests_bezier {
    use super::*;

    #[test]
    fn test_on_bernstein_f64_basic() {
        let b = on_bernstein_f64(2, 3, 0.5);
        // B₂³(0.5) = 3 * 0.5² * (1 - 0.5) = 3 * 0.25 * 0.5 = 0.375
        assert!((b - 0.375).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_on_bernstein_f64_sum_to_one() {
        let u = 0.3;
        let n = 4;
        let sum: f64 = (0..=n).map(|i| on_bernstein_f64(i, n, u)).sum();
        assert!((sum - 1.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_on_solve_bezier_t_from_alpha_midpoint() {
        let alpha = 0.5;
        let t = on_solve_bezier_t_from_alpha(alpha);
        // t should be close to 0.5 for symmetric cubic Bezier
        assert!((t - 0.5).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_on_solve_bezier_t_from_alpha_edge() {
        let t0 = on_solve_bezier_t_from_alpha(0.0);
        let t1 = on_solve_bezier_t_from_alpha(1.0);
        assert!((t0 - 1.0).abs() < 1e-6);
        assert!((t1 - 0.0).abs() < 1e-6);
    }
}
```

### 🧠 설명
- on_bernstein_f64는 베지어 곡선의 기저함수로, 모든 항의 합이 항상 1이 되어야 합니다.
- on_solve_bezier_t_from_alpha는 주어진 α에 대해 t를 역으로 추정하는 함수로, 뉴턴-랩슨 방식으로 수렴합니다.
- 테스트는 정확한 값, 경계값, 합 조건 등을 검증합니다.


---

평면과 삼각형 절단 관련 함수들에 대한 테스트 코드를 작성.  
이 범주는 기하 알고리즘의 핵심이며, 절단 결과의 정확성과 교차점 계산이 중요합니다.

## ✅ 3단계 테스트 코드: 평면과 삼각형 절단
```rust
#[cfg(test)]
mod tests_plane_cut {
    use super::*;

    #[test]
    fn test_on_get_ratio_midpoint() {
        let eq = [0.0, 0.0, 1.0, -0.5]; // 평면 z = 0.5
        let vert = [
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Point3D::new(0.0, 0.0, 2.0),
        ];
        let fres = [eq[2] * vert[0].z + eq[3], eq[2] * vert[1].z + eq[3], eq[2] * vert[2].z + eq[3]];
        let r = on_get_ratio(0, 1, &fres, &eq, &vert);
        assert!((r - 0.5).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_on_interpolate_vertex_linear() {
        let v1 = Point3D::new(0.0, 0.0, 0.0);
        let v2 = Point3D::new(1.0, 1.0, 1.0);
        let mid = on_interpolate_vertex(v1, v2, 0.5);
        assert_eq!(mid, Point3D::new(0.5, 0.5, 0.5));
    }
```
```rust

    #[test]
    fn test_tri_plane_cut_simple() {
        let eq = [0.0, 0.0, 1.0, -0.5]; // 평면 z = 0.5
        let vert = [
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 1.0),
            Point3D::new(0.0, 1.0, 1.0),
        ];
        let mut edge = [Point3D::default(); 2];
        let mut vert_new = [Point3D::default(); 6];
        let mut i_tri = 0;
        let mut i_edge = 0;
        let success = tri_plane_cut(eq, vert, &mut edge, &mut vert_new, &mut i_tri, &mut i_edge);
        assert!(success);
        assert!(i_tri > 0);
        assert!(i_edge > 0);
    }
}
```

### 🧠 설명
- on_get_ratio는 평면과 선분의 교차 비율을 계산합니다.
- on_interpolate_vertex는 두 점 사이의 보간을 수행합니다.
- tri_plane_cut은 삼각형을 평면으로 절단하고, 절단된 꼭짓점과 교차점을 반환합니다.

---


야코비안 및 법선 도함수 관련 함수들에 대한 테스트 코드를 작성.  
이 범주는 곡면의 기하적 특성을 평가하는 데 핵심적인 역할을 합니다.

## ✅ 4단계 테스트 코드: 야코비안 및 법선 도함수
```rust
#[cfg(test)]
mod tests_jacobian_normal {
    use super::*;

    #[test]
    fn test_on_ev_jacobian_reliable() {
        let ds_o_ds = 1.0;
        let ds_o_dt = 0.0;
        let dt_o_dt = 1.0;
        let (reliable, det) = on_ev_jacobian(ds_o_ds, ds_o_dt, dt_o_dt);
        assert!(reliable);
        assert!((det - 1.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_jacobian_reliable_cross_area() {
        let ds = Vector3D::new(1.0, 0.0, 0.0);
        let dt = Vector3D::new(0.0, 1.0, 0.0);
        assert!(jacobian_reliable(ds, dt));
    }
```
```rust
    #[test]
    fn test_on_ev_normal_partials_valid() {
        let ds = Vector3D::new(1.0, 0.0, 0.0);
        let dt = Vector3D::new(0.0, 1.0, 0.0);
        let dss = Vector3D::new(0.0, 0.0, 0.0);
        let dst = Vector3D::new(0.0, 0.0, 0.0);
        let dtt = Vector3D::new(0.0, 0.0, 0.0);
        let result = on_ev_normal_partials(ds, dt, dss, dst, dtt);
        assert!(result.is_some());
        let (ns, nt) = result.unwrap();
        assert!(ns.length() < 1e-6);
        assert!(nt.length() < 1e-6);
    }
```
```rust
    #[test]
    fn test_on_ev_normal_regular_case() {
        let du = Vector3D::new(1.0, 0.0, 0.0);
        let dv = Vector3D::new(0.0, 1.0, 0.0);
        let duu = Vector3D::new(0.0, 0.0, 0.0);
        let duv = Vector3D::new(0.0, 0.0, 0.0);
        let dvv = Vector3D::new(0.0, 0.0, 0.0);
        let n = on_ev_normal(1, du, dv, duu, duv, dvv);
        assert!(n.is_some());
        let n_vec = n.unwrap();
        assert!((n_vec - Vector3D::new(0.0, 0.0, 1.0)).length() < 1e-6);
    }
}
```

### 🧠 설명
- on_ev_jacobian은 야코비안 행렬식 EG-F^2을 계산하고 신뢰성을 평가합니다.
- jacobian_reliable은 교차면적 기반으로 안정성을 판단합니다.
- on_ev_normal_partials는 단위 법선의 u,v 방향 도함수를 계산합니다.
- on_ev_normal은 퇴화된 경우에도 법선 방향을 근사합니다.

---

접선 및 곡률 관련 함수들에 대한 테스트 코드를 작성.  
이 범주는 곡선의 기하적 특성을 평가하는 데 핵심적인 역할을 합니다.

## ✅ 5단계 테스트 코드: 접선 및 곡률
```rust
#[cfg(test)]
mod tests_tangent_curvature {
    use super::*;

    #[test]
    fn test_on_ev_tangent_regular() {
        let d1 = Vector3D::new(1.0, 1.0, 0.0);
        let d2 = Vector3D::new(0.0, 0.0, 1.0);
        let t = on_ev_tangent(d1, d2);
        assert!(t.is_some());
        let tangent = t.unwrap();
        let expected = Vector3D::new(1.0, 1.0, 0.0).normalize_copy();
        assert!((tangent - expected).length() < 1e-6);
    }
```
```rust
    #[test]
    fn test_on_ev_tangent_degenerate() {
        let d1 = Vector3D::new(0.0, 0.0, 0.0);
        let d2 = Vector3D::new(0.0, 1.0, 0.0);
        let t = on_ev_tangent(d1, d2);
        assert!(t.is_some());
        let tangent = t.unwrap();
        assert!((tangent - Vector3D::new(0.0, 1.0, 0.0)).length() < 1e-6);
    }
```
```rust
    #[test]
    fn test_on_ev_curvature_circle() {
        let d1 = Vector3D::new(1.0, 0.0, 0.0);
        let d2 = Vector3D::new(0.0, 1.0, 0.0);
        let result = on_ev_curvature(d1, d2);
        assert!(result.is_some());
        let (tangent, curvature) = result.unwrap();
        assert!((tangent - Vector3D::new(1.0, 0.0, 0.0)).length() < 1e-6);
        assert!((curvature - Vector3D::new(0.0, 1.0, 0.0)).length() < 1e-6);
    }
```
```rust
    #[test]
    fn test_on_ev_curvature_degenerate() {
        let d1 = Vector3D::new(0.0, 0.0, 0.0);
        let d2 = Vector3D::new(0.0, 0.0, 0.0);
        let result = on_ev_curvature(d1, d2);
        assert!(result.is_none());
    }
}
```
### 🧠 설명
- on_ev_tangent은 1차 도함수를 정규화하여 접선 벡터를 계산하며, 퇴화 시 2차 도함수로 근사합니다.
- on_ev_curvature는 곡률 벡터를 계산하며, 접선 방향과 곡률 방향을 분리해줍니다.
- 테스트는 정상 케이스와 퇴화 케이스 모두를 포함하여 안정성과 정확성을 검증합니


파라미터 공차 및 언랩 관련 함수들에 대한 테스트 코드를 작성.  
이 범주는 주기적 파라미터 처리와 수치 안정성을 위한 공차 계산이 핵심입니다.

## ✅ 6단계 테스트 코드: 파라미터 공차 및 언랩
```rust
#[cfg(test)]
mod tests_parameter_unwrap {
    use super::*;

    #[test]
    fn test_get_parameter_tolerance_valid() {
        let t0 = 0.0;
        let t1 = 1.0;
        let t = 0.5;
        let result = get_parameter_tolerance(t0, t1, t);
        assert!(result.is_some());
        let (tminus, tplus) = result.unwrap();
        assert!(tminus < t && tplus > t);
        assert!((tplus - tminus) < 1.0);
    }
```
```rust
    #[test]
    fn test_get_parameter_tolerance_invalid_domain() {
        let result = get_parameter_tolerance(1.0, 0.0, 0.5);
        assert!(result.is_none());
    }
```
```rust

    #[test]
    fn test_on_unwrap_around_center_basic() {
        let u_list = vec![0.1, 6.2, 0.3];
        let unwrapped = on_unwrap_around_center(&u_list, std::f64::consts::PI * 2.0);
        assert_eq!(unwrapped.len(), 3);
        assert!((unwrapped[1] - unwrapped[0]).abs() < std::f64::consts::PI);
    }
```
```rust
    #[test]
    fn test_on_unwrap_to_face_range_basic() {
        let u_list = vec![-1.0, 0.0, 7.0];
        let period = 6.0;
        let umin = 0.0;
        let umax = 6.0;
        let unwrapped = on_unwrap_to_face_range(&u_list, period, umin, umax);
        assert_eq!(unwrapped.len(), 3);
        for &u in &unwrapped {
            assert!(u >= umin && u <= umax);
        }
    }
}
```

### 🧠 설명
- get_parameter_tolerance는 파라미터 t에 대해 수치적으로 안전한 범위를 계산합니다.
- on_unwrap_around_center는 주기적 파라미터를 중심 기준으로 연속적으로 펼쳐줍니다.
- on_unwrap_to_face_range는 주어진 범위 내로 파라미터를 재정렬합니다.




기타 유틸리티 함수들에 대한 테스트 코드를 작성.  
이 범주는 다양한 보조 기능들을 포함하며, 기하 계산, 경계 갱신, 직교성 판정 등이 포함됩니다.

## ✅ 7단계 테스트 코드: 기타 유틸리티 함수들
```rust
#[cfg(test)]
mod tests_utilities {
    use super::*;

    #[test]
    fn test_on_update_minmax_quick_xyz() {
        let mut min = Point3D::new(1.0, 1.0, 1.0);
        let mut max = Point3D::new(1.0, 1.0, 1.0);
        on_update_minmax_quick_xyz(0.0, 2.0, -1.0, &mut min, &mut max);
        assert_eq!(min, Point3D::new(0.0, 1.0, -1.0));
        assert_eq!(max, Point3D::new(1.0, 2.0, 1.0));
    }
```
```rust
    #[test]
    fn test_on_try_get_perpendicular_vector() {
        let a = Vector3D::new(1.0, 0.0, 0.0);
        let b = Vector3D::new(0.0, 1.0, 0.0);
        let perp = on_try_get_perpendicular_vector(a, b);
        assert!(perp.is_some());
        let v = perp.unwrap();
        assert!(v.length() > 0.99);
    }
```
```rust
    #[test]
    fn test_on_contains_value_with_tolerance() {
        let values = vec![1.0, 2.0, 3.0];
        assert!(on_contains_value_with_tolerance(&values, 2.0000001, 1e-6));
        assert!(!on_contains_value_with_tolerance(&values, 4.0, 1e-6));
    }
```
```rust
    #[test]
    fn test_on_ev_tangent_and_curvature_consistency() {
        let d1 = Vector3D::new(1.0, 0.0, 0.0);
        let d2 = Vector3D::new(0.0, 1.0, 0.0);
        let result = on_ev_curvature(d1, d2);
        assert!(result.is_some());
        let (tangent, curvature) = result.unwrap();
        let dot = tangent.dot(&curvature);
        assert!(dot.abs() < 1e-12); // 곡률은 접선과 수직이어야 함
    }
}
```

### 🧠 설명
- on_update_minmax_quick_xyz는 AABB 계산에 사용되는 빠른 경계 갱신 함수입니다.
- on_try_get_perpendicular_vector는 두 벡터가 평행하지 않을 때 직교 벡터를 생성합니다.
- on_contains_value_with_tolerance는 값이 허용 오차 내에 존재하는지 확인합니다.
- 마지막 테스트는 접선과 곡률 벡터가 수직임을 검증합니다.

---
