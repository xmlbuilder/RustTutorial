# BooleanOp: 삼각형 메쉬 Boolean 연산 정리

이 문서는 **삼각형 메쉬 Boolean 연산기 (`BooleanOp`)** 의 알고리즘과 주요 수식을 정리한 것이다.

## 지원 연산:

- `merge`  : **A ∪ B** (Union)
- `extract`: **A \ B** (Difference)
- `intersect`: **A ∩ B** (Intersection)

현재 구현은 **볼륨이 실제로 겹치는 일반적인 3D 교차** 를 대상으로 설계되어 있다.  
두 메쉬가 **완전히 같은 평면 위에서 coplanar 로 넓게 겹치는 경우** (면 공유, 경계만 맞닿는 경우)는 정식으로 지원하지 않는다.

---

## 1. 핵심 데이터 구조

### 1.1 `Mesh` (요약)

다른 모듈에 정의되어 있으며, 여기서는 다음 필드만 사용된다.

- `vertices: Vec<Point3D>` — 정점 배열
- `faces: Vec<[u32; 4]>`   — 삼각형 인덱스  
  - `faces[i] = [i0, i1, i2, _]`  형식으로 사용
- 주요 메서드
  - `bounding_box() -> BoundingBox`
  - `append(&mut self, other: &Mesh)`
  - `remove_unused_vertices()`
  - `remove_duplicated_vertices()`
  - `remove_duplicated_vertices_and_remap_edge_endpoints(...)`
  - `remove_empty_faces()`
  - `compute_normals()`

### 1.2 `IntersectedEdge`

두 삼각형이 교차할 때, 그 교차 선분을 나타내는 구조체.

```rust
pub struct IntersectedEdge {
    pub vt: [TVertex; 2],      // 교차 선분의 두 끝점 (3D 좌표)
    pub facet_idx: [isize; 2], // 이 선분이 속한 두 삼각형의 인덱스 (main, sub)
    pub vert_ids: [usize; 2],  // 메쉬의 vertex 배열에서의 인덱스
}
```

`BooleanOp` 내부에서 `self.iedges: Vec<IntersectedEdge>` 로 관리한다.

### 1.3 `RayTraceItem`

원래 레이 기반 분류 알고리즘에서 사용하던 구조체로, 현재 Rust 버전에서는 **직접 사용하지 않고** 있다. (보존용)

```rust
struct RayTraceItem {
    idx: usize,        // 대상 face index
    center: [f64; 3],  // 레이 시작점 (투영 좌표계 기준)
    pos: f64,          // 레이 방향 축 기준 교차 위치
    depth: f64,        // 교차 삼각형의 평균 깊이
    sign_norm: i32,    // 레이 기준 방향 부호
    sign_self: i32,    // 삼각형 자체 법선의 부호
    dist: f64,         // 레이 시작점과 가장 가까운 교차까지의 거리
    zn_self: f64,      // 자기 삼각형 법선의 해당 방향 성분
    zn_near: f64,      // 가장 가까운 상대 삼각형의 해당 방향 성분
}
```

나중에 coplanar/복잡한 경우를 확장할 때 재사용할 수 있는 후보이다.

### 1.4 2D 투영 헬퍼

삼각형의 법선 방향에 따라, 좌표축을 재배열해서 **2D 평면상으로 투영**하기 위한 함수들이다.

```rust
fn cnv2dv_x(dir: usize, v: &[f64; 3]) -> f64 { ... }
fn cnv2dv_y(dir: usize, v: &[f64; 3]) -> f64 { ... }
fn cnv2dv_z(dir: usize, v: &[f64; 3]) -> f64 { ... }

fn cnv2dv_inv(dir: usize, v: [f64; 3]) -> [f64; 3] { ... }
```

- `dir` 는 주 방향 인덱스 (3..5) 를 의미한다.
- `(x, y)` 는 CDT 에서 사용할 2D 좌표이며,
- `cnv2dv_inv` 는 다시 3D 좌표로 되돌릴 때 사용한다.

삼각형 법선의 가장 큰 성분 방향은

```rust
fn get_main_direction(norm: [f64; 3]) -> usize
```

에서 결정한다.

### 1.5 삼각형 법선

```rust
fn triangle_normal(a: &Point3D, b: &Point3D, c: &Point3D) -> Vector3D {
    let ax = a.x - c.x;
    let ay = a.y - c.y;
    let az = a.z - c.z;
    let bx = b.x - c.x;
    let by = b.y - c.y;
    let bz = b.z - c.z;
    Vector3D::new(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
}
```

이는 표준적인 **외적** 정의다.

---

## 2. BooleanOp 구조체와 연산 개요

```rust
pub struct BooleanOp {
    iedges: Vec<IntersectedEdge>,
    eps: f64,
}
```

- `eps` 는 교차 검사 및 레이 교차에서 사용할 **허용 오차**.

생성자:

```rust
impl Default for BooleanOp {
    fn default() -> Self {
        Self {
            iedges: Vec::new(),
            eps: 1.0e-5,
        }
    }
}
```

이 연산기는 메쉬 A(main), B(sub)에 대해 다음 3가지 연산을 제공한다.

### 2.1 Union: `merge(main, sub)`

1. 원본 백업
2. 두 메쉬 사이의 교차 edge (`IntersectedEdge`) 계산
3. 교차가 있는 면을 CDT로 재삼각화
4. 서로의 **내부 면**을 제거
5. 두 메쉬를 합치고 클린업

### 2.2 Difference: `extract(main, sub)`  (main \ sub)

1. 원본 백업
2. 교차 edge 계산
3. 교차 면 CDT 재삼각화
4. `main` 에서는 **내부면 제거** (sub 내부)
5. `sub` 에서는 **외부면 제거** (main 외부)
6. `sub` 메쉬를 뒤집어(flip)서 `main`에 붙인다.
7. 클린업

### 2.3 Intersection: `intersect(main, sub)`

1. 원본 백업
2. 교차 edge 계산
3. 교차 면 CDT 재삼각화
4. `main`, `sub` 각각에서 **외부면 제거**
5. 두 메쉬를 합치고 클린업

각 연산에서 **가장 중요한 단계**는:

- (1) 삼각형-삼각형 교차를 통해 **정확한 intersection segment** 를 찾는 것
- (2) 그 segment들에 맞춰 face를 잘게 나누는 것 (CDT)
- (3) 나눠진 면들을 **내부/외부 분류**해서 살릴지 버릴지 결정하는 것

이다.

---

## 3. 1단계: 삼각형-삼각형 교차 수집

### 3.1 `check_intersected_edge`

```rust
fn check_intersected_edge(&mut self, main: &Mesh, sub: &Mesh) -> bool {
    self.iedges.clear();

    let range = main.bounding_box().union(&sub.bounding_box());
    let s = range.size();
    self.eps = s.x.max(s.y).max(s.z) * 1.0e-4;
    ...
}
```

- 두 메쉬의 전체 bounding box를 구하고, 그 크기에서 **상대적인 eps**를 잡는다.

#### 3.1.1 AABB 빠른 검사

각 face 쌍 `(fa, fb)` 에 대해:

1. `fa` 의 3개 정점 `(pa0, pa1, pa2)` 의 **min/max** (x,y,z)를 계산 → `amin`, `amax`
2. `fb` 의 3개 정점 `(pb0, pb1, pb2)` → `bmin`, `bmax`
3. 다음 조건 중 하나라도 참이면 두 삼각형은 절대 교차할 수 없다.

```text
amin.x > bmax.x  or  amax.x < bmin.x
amin.y > bmax.y  or  amax.y < bmin.y
amin.z > bmax.z  or  amax.z < bmin.z
```

이 경우 tri-tri 교차는 생략한다.

#### 3.1.2 tri-tri 교차 + segment 길이 검사

AABB 를 통과한 삼각형 쌍에 대해

```rust
let inter = on_tri_tri_intersect_with_isectline_ex(..., self.eps);

if !inter.intersects || inter.coplanar {
    continue;
}
```

- `inter.intersects == true` 이고
- `inter.coplanar == false` 인 경우만 사용한다.  
  → **완전 coplanar 인 케이스는 여기서 버린다.**

반환된 두 점 `inter.p0`, `inter.p1` 를 `Point3D`로 변환 후, 선분 길이를 검사한다.

```rust
let fdist = (p0.x - p1.x)^2 + (p0.y - p1.y)^2 + (p0.z - p1.z)^2;
if fdist > f64::EPSILON {
    // 유효한 교차 선분으로 간주
}
```

- 너무 짧은 선분은 **수치 잡음**으로 보고 버린다.

유효한 선분은 `temp_edges[j]` 에 임시로 적재하고, `facet_idx` 에 `(i, j)` 를 기록한다.

각 `i` (main의 face) 루프가 끝나면 `temp_edges` 중 `facet_idx`가 유효한 것만
`self.iedges` 에 옮겨 담는다. 이렇게 하면 각 `j`(sub face)에 대해
한 번씩만 기록하게 되어, 중복 우발을 줄인다.

---

## 4. 2단계: 교차 면 재삼각화 (CDT)

### 4.1 개요

```rust
fn triangulate_intersected_surface(&mut self, index: usize, mesh: &mut Mesh) -> bool
```

- `index == 0` : `facet_idx[0]` 기준 (main 메쉬)
- `index == 1` : `facet_idx[1]` 기준 (sub 메쉬)

#### 4.1.1 정렬 및 교차점 정점화

```rust
if index == 0 {
    self.iedges.sort_by(cmp_iedge_a);
} else {
    self.iedges.sort_by(cmp_iedge_b);
}

// 모든 교차 segment 의 끝점을 메쉬 정점 배열에 추가
for e in &mut self.iedges {
    e.vert_ids[0] = mesh.vertices.len();
    mesh.vertices.push(e.vt[0].to_point3d());
    e.vert_ids[1] = mesh.vertices.len();
    mesh.vertices.push(e.vt[1].to_point3d());
}

// 중복 정점 제거 + edge endpoint 리매핑
mesh.remove_duplicated_vertices_and_remap_edge_endpoints(&mut self.iedges[..], ON_ZERO_TOL);
```

- 교차 segment 의 끝점이 모두 `mesh.vertices` 에 들어가고,
- 좌표상 거의 같은 점(거리 < `ON_ZERO_TOL`)은 하나로 합쳐진다.

#### 4.1.2 face 단위로 교차 segment 그룹화

```rust
let mut facet_flags = vec![0u8; mesh.faces.len()];
let mut new_faces: Vec<[u32; 4]> = Vec::new();

let mut j = 0usize;
for i in 0..mesh.faces.len() {
    if j >= self.iedges.len() { break; }

    let start = j;
    let mut cnt_local = 0usize;
    while j < self.iedges.len() && (self.iedges[j].facet_idx[index] as usize) == i {
        cnt_local += 1;
        j += 1;
    }
    if cnt_local == 0 { continue; }

    facet_flags[i] = 1;
    ...
}
```

- `facet_flags[i] == 1` 인 face 는 **교차 segment 를 가진 face** 로,
  재삼각화 대상이 된다.

#### 4.1.3 투영 방향 선택

face `i` 의 삼각형 정점 `va, vb, vc` 에서 법선을 구한 뒤

```rust
let n = triangle_normal(&va, &vb, &vc);
let dir = get_main_direction([n.x, n.y, n.z]);
```

- `dir` 에 따라 `(x, y)` 좌표를 결정한다.
- 예: `dir == 5` 면 Z 성분이 가장 크므로, XY 평면으로 투영하는 식.

#### 4.1.4 CDT (Spade) 생성

```rust
let mut cdt: CDT<Point2<f64>> = CDT::default();

// 삼각형 코너 3개 삽입
let mut handles = Vec::with_capacity(3);
for p in [va, vb, vc] {
    let arr = p.to_array();
    let x = cnv2dv_x(dir, &arr);
    let y = cnv2dv_y(dir, &arr);
    if let Ok(h) = cdt.insert(Point2::new(x, y)) {
        handles.push(h);
    }
}
if handles.len() != 3 {
    facet_flags[i] = 0;
    continue;
}

// 외곽을 제약선으로 묶는다
for k in 0..3 {
    let _ = cdt.add_constraint(handles[k], handles[(k + 1) % 3]);
}
```

- 삼각형 자체의 외곽은 CDT 에 **제약(edge constraint)** 으로 등록되므로,
  분할된 삼각형들이 원래 face 범위를 벗어나지 않는다.

#### 4.1.5 교차 선분의 끝점 삽입 (내부 포인트)

```rust
for k in 0..cnt_local {
    let e = &self.iedges[start + k];
    if e.vert_ids[0] == e.vert_ids[1] {
        continue;
    }

    let p0 = mesh.vertices[e.vert_ids[0]];
    let p1 = mesh.vertices[e.vert_ids[1]];

    let arr0 = p0.to_array();
    let arr1 = p1.to_array();

    let _ = cdt.insert(Point2::new(
        cnv2dv_x(dir, &arr0),
        cnv2dv_y(dir, &arr0),
    ));
    let _ = cdt.insert(Point2::new(
        cnv2dv_x(dir, &arr1),
        cnv2dv_y(dir, &arr1),
    ));

    // 교차 선분 자체를 constraint로 넣지 않는다.
}
```

- 교차 선분을 그대로 constraint로 넣으면, 서로 다른 face에서 온 constraint끼리
  교차해서 **Spade가 panic** 할 수 있다.
- 따라서 여기서는 **내부 포인트만** 추가하여, CDT가 알아서
  적절한 삼각형 분할을 생성하도록 한다.

#### 4.1.6 2D → 3D 역투영 (`back_project_on_plane`)

CDT 의 정점들을 순회하면서, 각 정점에 대응하는 3D 좌표를 계산한다.

```rust
let mut vmap: Vec<Point3D> = Vec::new();
for vtx in cdt.vertices() {
    let p = vtx.position();
    let lifted = back_project_on_plane(dir, p.x, p.y, &va, &vb, &vc);
    vmap.push(lifted);
}
```

`back_project_on_plane` 의 핵심은 **면의 방정식**과 **좌표 축 재배열**이다.

- 삼각형 평면의 법선 `n = (nx, ny, nz)`
- 평면 위의 한 점 `va = (ax, ay, az)`

평면 방정식은

```text
n · X + d = 0
d = - (n · va)
```

2D CDT 결과의 좌표 `(x, y)` 는 `dir` 에 따라 `(X, Y, Z)` 의 일부로 쓰인다.
예를 들어:

- Z가 주 성분이면 `(x, y, z)` 중 `(x,y)`는 원래 `(X,Y)` 에 매핑되고,
- `Z` 는 평면 방정식으로 계산한다.

코드에서는 다음과 같이 구현되어 있다.

```rust
let n = triangle_normal(va, vb, vc);
let d = -(n.x * va.x + n.y * va.y + n.z * va.z);

let mut w = cnv2dv_inv(dir, [x, y, 0.0]);
let eps = 1e-15;
if n.z.abs() > eps {
    w[2] = -(n.x * w[0] + n.y * w[1] + d) / n.z;
} else if n.y.abs() > eps {
    w[1] = -(n.x * w[0] + n.z * w[2] + d) / n.y;
} else {
    w[0] = -(n.y * w[1] + n.z * w[2] + d) / n.x;
}
```

- 즉, `n` 의 가장 큰 성분에 대해 평면 방정식을 풀어, 남은 좌표를 계산한다.

#### 4.1.7 새로운 삼각형 생성

```rust
for fid in cdt.fixed_inner_faces() {
    let [va_h, vb_h, vc_h] = cdt.face(fid).vertices();
    let ia = va_h.fix().index();
    let ib = vb_h.fix().index();
    let ic = vc_h.fix().index();

    let base = mesh.vertices.len() as u32;
    mesh.vertices.push(vmap[ia]);
    mesh.vertices.push(vmap[ib]);
    mesh.vertices.push(vmap[ic]);

    new_faces.push([base, base + 1, base + 2, base + 2]);
}
```

- CDT가 생성한 내부 삼각형들에 대해, 대응하는 3D 좌표를 `mesh.vertices` 에 추가하고,
- 새 face 인덱스를 `new_faces` 에 쌓는다.

#### 4.1.8 재삼각화된 face 교체

```rust
let mut kept: Vec<[u32; 4]> = Vec::with_capacity(mesh.faces.len());
for (idx, f) in mesh.faces.iter().enumerate() {
    if facet_flags[idx] == 0 {
        kept.push(*f);
    }
}
kept.extend(new_faces.into_iter());
mesh.faces = kept;

mesh.compute_normals();
```

- 원래의 `mesh.faces` 중 **재삼각화 대상이 아닌 face** 만 남기고,
- 새로 생성된 CDT face들을 뒤에 붙인다.

이로써 교차 영역이 **교차선에 맞춰 충분히 세분화**된다.

---

## 5. 3단계: 내부/외부 판정과 면 제거

### 5.1 `remove_surface`

```rust
fn remove_surface(
    &mut self,
    main: &mut Mesh,
    sub: &Mesh,
    op: RemoveOp,
    _is_main: bool,
) -> bool {
    if main.faces.is_empty() || sub.faces.is_empty() {
        return false;
    }

    let eps = self.eps.max(1.0e-9);
    let mut kept_faces: Vec<[u32; 4]> = Vec::with_capacity(main.faces.len());

    for f in &main.faces {
        let c = Self::face_centroid(main, f);
        let inside = Self::point_inside_mesh(&c, sub, eps);

        let remove = match op {
            RemoveOp::RemoveInner => inside,
            RemoveOp::RemoveOuter => !inside,
        };

        if !remove {
            kept_faces.push(*f);
        }
    }

    main.faces = kept_faces;
    true
}
```

- 각 face 의 **무게중심(centroid)** 를 구하고,
- 그 점이 `sub` 메쉬 내부인지 밖인지 판정한 뒤,
- Boolean 연산의 종류에 맞게 face 를 제거/보존한다.

### 5.2 face centroid

```rust
fn face_centroid(mesh: &Mesh, f: &[u32; 4]) -> Point3D {
    let a = mesh.vertices[f[0] as usize];
    let b = mesh.vertices[f[1] as usize];
    let c = mesh.vertices[f[2] as usize];

    Point3D::new(
        (a.x + b.x + c.x) / 3.0,
        (a.y + b.y + c.y) / 3.0,
        (a.z + b.z + c.z) / 3.0,
    )
}
```

기본적인 삼각형 무게중심 공식이다.

### 5.3 점이 메쉬 내부인지 검사: `point_inside_mesh`

```rust
fn point_inside_mesh(p: &Point3D, mesh: &Mesh, eps: f64) -> bool {
    let dir = Vector3D::new(1.0, 0.0, 0.0); // +X 방향 레이
    let mut count = 0usize;

    for f in &mesh.faces {
        let v0 = mesh.vertices[f[0] as usize];
        let v1 = mesh.vertices[f[1] as usize];
        let v2 = mesh.vertices[f[2] as usize];

        if let Some(t) = Self::ray_triangle_intersection(p, &dir, &v0, &v1, &v2, eps) {
            if t > eps {
                count += 1;
            }
        }
    }

    (count % 2) == 1
}
```

- 고전적인 **ray casting (even-odd rule)** 을 사용한다.
- 한 점에서 +X 방향으로 레이를 쏘고, 삼각형과 교차하는 횟수를 센다.
- 교차 횟수가 홀수이면 내부, 짝수이면 외부로 판정한다.

### 5.4 ray-triangle 교차 (Möller–Trumbore)

```rust
fn ray_triangle_intersection(
    p: &Point3D,
    dir: &Vector3D,
    v0: &Point3D,
    v1: &Point3D,
    v2: &Point3D,
    eps: f64,
) -> Option<f64> {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;

    let h = dir.cross(&edge2);
    let a = edge1.dot(&h);
    if a.abs() < eps {
        return None; // 레이와 거의 평행
    }

    let f = 1.0 / a;
    let s = p - v0;
    let u = f * s.dot(&h);
    if u < -eps || u > 1.0 + eps {
        return None;
    }

    let q = s.cross(&edge1);
    let v = f * dir.dot(&q);
    if v < -eps || u + v > 1.0 + eps {
        return None;
    }

    let t = f * edge2.dot(&q);
    if t > eps {
        Some(t)
    } else {
        None
    }
}
```

Möller–Trumbore 알고리즘의 표준 형태로, 수식은 다음과 같다.

- `edge1 = v1 - v0`
- `edge2 = v2 - v0`
- `h = dir × edge2`
- `a = edge1 · h`  
  - `a ≈ 0` 이면 레이와 삼각형이 거의 평행 → 교차 없음

- `f = 1 / a`
- `s = p - v0`
- `u = f * (s · h)`  
  - `0 ≤ u ≤ 1` 이 아니면 삼각형 밖

- `q = s × edge1`
- `v = f * (dir · q)`  
  - `0 ≤ v`, `u + v ≤ 1` 이 아니면 삼각형 밖

- `t = f * (edge2 · q)`  
  - `t > eps` 이면 레이 방향으로의 거리 `t` 에서 교차

이 알고리즘은 **레이와 삼각형의 교차점**을 수치적으로 안정적으로 구하기 위해 널리 쓰이는 방식이다.

---

## 6. Boolean 연산별 동작 요약

### 6.1 Union (`merge`)

```rust
pub fn merge(&mut self, main: &mut Mesh, sub: &mut Mesh) -> bool {
    let main_o = main.clone();
    let sub_o = sub.clone();

    self.check_intersected_edge(main, sub);
    self.triangulate_intersected_surface(0, main);
    self.triangulate_intersected_surface(1, sub);

    self.remove_surface(main, &sub_o, RemoveOp::RemoveInner, true);
    self.remove_surface(sub, &main_o, RemoveOp::RemoveInner, false);

    main.append(sub);
    // 정리
}
```

- `main` : `sub` 내부에 있던 면 제거
- `sub`  : `main` 내부에 있던 면 제거
- 두 메쉬를 합쳐서 **합집합 표면**을 만든다.

### 6.2 Difference (`extract`) — main \ sub

```rust
pub fn extract(&mut self, main: &mut Mesh, sub: &mut Mesh) -> bool {
    let main_o = main.clone();
    let sub_o = sub.clone();

    self.check_intersected_edge(main, sub);
    self.triangulate_intersected_surface(0, main);
    self.triangulate_intersected_surface(1, sub);

    self.remove_surface(main, &sub_o, RemoveOp::RemoveInner, true);
    self.remove_surface(sub, &main_o, RemoveOp::RemoveOuter, false);

    self.flip_mesh(sub);
    main.append(sub);
    // 정리
}
```

- `main` : `sub` 내부에 있던 면 제거 → **뚫린 껍데기**
- `sub`  : `main` 외부에 있는 면 제거 → **구멍을 막는 패치**
- `sub` 의 법선을 뒤집어 `main`에 붙이면, 차집합 표면이 완성된다.

### 6.3 Intersection (`intersect`) — main ∩ sub

```rust
pub fn intersect(&mut self, main: &mut Mesh, sub: &mut Mesh) -> bool {
    let main_o = main.clone();
    let sub_o = sub.clone();

    self.check_intersected_edge(main, sub);
    self.triangulate_intersected_surface(0, main);
    self.triangulate_intersected_surface(1, sub);

    self.remove_surface(main, &sub_o, RemoveOp::RemoveOuter, true);
    self.remove_surface(sub, &main_o, RemoveOp::RemoveOuter, false);

    main.append(sub);
    // 정리
}
```

- `main`, `sub` 모두 **외부 면**을 제거한다.
- 두 메쉬를 합쳐서 겹치는 부분만 남도록 만든다.

---

## 7. 한계와 주의점

1. **완전히 coplanar 한 삼각형 쌍**은 tri-tri 단계에서 `inter.coplanar == true` 로 걸러지고,
   교차 segment가 생성되지 않는다.  
   → 이 경우, Boolean 결과는 **정의되지 않은 동작**에 가깝다.
2. `point_inside_mesh`는 단순 +X 방향 레이 하나로 even-odd rule 을 적용한다.
   - 복잡한 self-intersection, coplanar, 레이와 면이 거의 평행한 경우에서는
     판정이 불안정할 수 있다.
3. `eps` 는 전체 bounding box 크기의 약 `1e-4` 로 잡는데,  
   모델의 스케일과 수치 상태에 따라 튜닝이 필요할 수 있다.

실제 CAD 커널 수준의 robust Boolean 을 구현하려면:

- coplanar handle (2D polygon Boolean)
- 다중 레이 / 방향 별 majority voting
- 더 정교한 epsilon 관리
- self-intersection 정리 등

추가 작업이 필요하다.

---

## 8. 향후 개선: Spatial Partition 도입

현재 `check_intersected_edge` 와 `point_inside_mesh` 는 모두

- **O(F_main × F_sub)** face 쌍 검사
- **O(F)** 레이-삼각형 검사

규모가 커지면 매우 느려진다. 하지만 알고리즘 구조는 단순해서,
나중에 **Spatial Partition (공간 분할)** 을 도입하는 것은 비교적 쉽다.

### 8.1 교차 검사 가속

예를 들어 다음과 같이 바꿀 수 있다.

1. `sub` 메쉬에 대해 `AABB Tree` 또는 `R-Tree`/`Grid` 를 만든다.
2. `main` 의 각 삼각형 AABB 에 대해,  
   공간 인덱스에서 **겹치는 sub 삼각형 후보들만** 가져온다.
3. tri-tri 교차는 이 후보들에 대해서만 수행한다.

그러면 시간 복잡도가 사실상 **O(F log F)** 수준으로 내려간다.

### 8.2 레이-삼각형 검사 가속

`point_inside_mesh` 에서도 비슷하게,

1. `mesh` 전체를 대상으로 공간 인덱스를 만든다.
2. 레이 AABB 또는 레이의 bounding slab와 겹치는 삼각형 후보만 검사한다.

이 역시 전체 triangle 수에 대해 선형이 아니라,  
필요한 후보 삼각형에 대해서만 검사하게 되어 속도가 크게 개선된다.

### 8.3 API 레벨에서의 변경 필요성

좋은 점은, Spatial Partition 을 도입해도 **외부 API (`BooleanOp::merge/extract/intersect`)는 그대로** 둘 수 있다는 것이다.

- 변경 지점은 내부 함수
  - `check_intersected_edge`
  - `point_inside_mesh`
- 이 두 위치에만 공간 인덱스를 끼워 넣으면 되므로,
  나중에 `ON_RTree`, `BVH`, `Uniform Grid` 등 어떤 구조를 쓰더라도
  외부 사용 코드는 수정할 필요가 없다.

---

## 9. 요약

- `BooleanOp` 는
  - tri-tri 교차로 정확한 교차 segment 를 구하고
  - CDT 로 면을 교차선에 맞춰 재삼각화한 뒤
  - 레이 캐스팅으로 내부/외부를 판정해서 face 를 필터링하는 구조이다.
- 현재 구현은 **일반적인 3D 교차(볼륨이 겹치는 경우)** 를 안정적으로 처리하는 데 초점을 맞추고 있으며,
  완전 coplanar 케이스는 아직 범위 밖이다.
- 구조가 단순하고 단계가 잘 분리되어 있어서,
  나중에 속도 향상을 위해 **Spatial Partition** 을 도입하는 것도 어렵지 않다
  (교차 검사와 레이 검사 부분만 교체하면 된다).


