## 📘 QuickHull 3D 알고리즘 문서
### 📌 개요
QuickHull 알고리즘은 3차원 공간에서 주어진 점 집합의 볼록 껍질(Convex Hull)을 계산하는 효율적인 기하 알고리즘입니다.  
이 구현은 Half-Edge 구조를 사용하여 메쉬를 구성하고, 점들을 반복적으로 껍질에 추가하면서 볼록 껍질을 확장합니다.

## 📐 수학적 원리
### 1. 볼록 껍질 정의
- `볼록 껍질(Convex Hull)`: 점 집합 $P = \{p_1, p_2, \dots, p_n\}$의 볼록 껍질은 $P$ 를 포함하는 가장 작은 볼록 다면체입니다.
- 수학적으로:

$$
\text{Conv}(P) = \{ \sum_{i=1}^n \lambda_i p_i \quad \mid \quad \lambda_i \geq 0, \quad \sum_{i=1}^n \lambda_i = 1 \}
$$


### 2. 평면의 방정식
- 평면은 법선 벡터 $\vec{n} = (a, b, c)$ 와 점 $p_0 = (x_0, y_0, z_0)$ 를 통해 정의됩니다.
- 평면 방정식:

$$
a(x - x_0) + b(y - y_0) + c(z - z_0) = 0 \quad \Rightarrow \quad ax + by + cz + d = 0
$$

- 여기서 $d = -(ax_0 + by_0 + cz_0)$


### 3. 점과 평면 사이의 부호 거리

- 점 q = (x, y, z)가 평면 $ax + by + cz + d = 0$ 으로부터 떨어진 거리:

$$
\text{dist}(q, \text{plane}) = \frac{ax + by + cz + d}{\sqrt{a^2 + b^2 + c^2}}
$$

- 이 구현에서는 제곱 거리만 사용:

$$
\text{signed\\_distance}(q) = ax + by + cz + d
$$

### 4. 삼각형의 법선 벡터

- 세 점 a, b, c로 이루어진 삼각형의 법선 벡터:

$$
\vec{n} = (b - c) \times (a - c)
$$


#### 5. 점과 직선 사이의 거리 제곱

- 점 p와 직선 r(t) = s + tv 사이의 거리 제곱:

$$
\text{dist}^2(p, r) = \|p - s\|^2 - \left( \frac{(p - s) \cdot v}{\|v\|} \right)^2
$$


## ⚙️ 알고리즘 작동 원리
- 초기 테트라헤드론 구성:
- 극값 점들 중 가장 멀리 떨어진 두 점을 찾고, 이들을 기준으로 삼각형을 만든 뒤,  
    가장 멀리 떨어진 네 번째 점을 찾아 테트라헤드론을 구성합니다.
- 볼록 껍질 확장:
- 각 면에서 가장 멀리 떨어진 점을 찾아, 해당 점에서 볼 수 있는 면들을 제거하고 새로운 면을 생성합니다.
- Horizon Edge 탐색:
- 제거된 면과 남은 면 사이의 경계선을 찾아 새로운 면을 연결합니다.
- 종료 조건:
- 더 이상 볼록 껍질 외부에 있는 점이 없을 때 종료합니다.


## 🧩 주요 구조체 및 함수 정리

| 구조체 이름     | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| ConvexHull     | 최종 결과를 담는 구조체. 정점 목록 (`vertices`)과 삼각형 인덱스 (`indices`) 포함 |
| Plane          | 평면 정의. 법선 벡터 `n`, 거리 상수 `d`, 법선 길이 제곱 `sqr_n_length` 포함     |
| Ray            | 직선 정의. 시작점 `s`, 방향 벡터 `v`, 길이 역제곱 `v_inv_length_squared` 포함   |
| Face           | 삼각형 면 정보. 평면, 가장 먼 점, 가시성, 경계선 정보 등 포함                   |
| HalfEdge       | Half-Edge 메쉬 구성 요소. 끝점, 반대 edge, face, 다음 edge 정보 포함             |
| MeshBuilder    | 메쉬 구성 및 관리. face/edge 추가, 비활성화, 초기 테트라헤드론 구성 등 담당     |
| QuickHull3D    | QuickHull 알고리즘 전체 구현체. 메쉬 구성, 볼록 껍질 생성, 반복 확장 로직 포함   |


## 🧮 핵심 함수 요약

| 함수 이름                    | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| compute_convex_hull         | 입력 점들로부터 볼록 껍질을 계산하는 외부 API 함수                   |
| build_mesh                  | 입력 점들을 기반으로 초기 메쉬를 구성하고 전처리 수행                 |
| setup_initial_tetrahedron   | 초기 테트라헤드론을 구성하여 알고리즘의 시작점을 설정                 |
| create_convex_half_edge_mesh| QuickHull 알고리즘의 핵심 반복 로직을 수행하여 볼록 껍질을 확장         |
| add_point_to_face           | 주어진 점이 특정 면의 positive side에 있는지 판단하고 할당             |
| triangle_normal             | 세


![QuickHull](/image/quick_hull_result.png)

---

## 소스 코드

```rust
#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

use crate::core::geom::{Point3D, Vector3D};
use crate::core::mesh::Mesh;
use crate::core::stl_reader::StlReader;
use crate::core::stl_writer::StlWriter;
use crate::core::types::Real;

// ========================================
// ConvexHull 결과 구조체 (public API)
// ========================================

#[derive(Debug, Clone)]
pub struct ConvexHull {
    pub vertices: Vec<Point3D>,
    pub indices: Vec<usize>, // 3개씩 하나의 삼각형
}
```
```rust
// ========================================
// 내부 유틸: 평면, 레이, 삼각형 법선
// ========================================

#[derive(Debug, Clone, Copy)]
struct Plane {
    n: Vector3D,
    d: Real,
    sqr_n_length: Real,
}
```
```rust
impl Plane {
    fn new(n: Vector3D, p: Point3D) -> Self {
        let d = -(n.x * p.x + n.y * p.y + n.z * p.z);
        let sqr_n_length = n.x * n.x + n.y * n.y + n.z * n.z;
        Plane { n, d, sqr_n_length }
    }
```
```rust
    #[inline]
    fn signed_distance(&self, q: &Point3D) -> Real {
        self.n.x * q.x + self.n.y * q.y + self.n.z * q.z + self.d
    }
```
```rust
    #[inline]
    fn is_point_on_positive_side(&self, q: &Point3D) -> bool {
        self.signed_distance(q) >= 0.0
    }
}
```
```rust
#[inline]
fn triangle_normal(a: &Point3D, b: &Point3D, c: &Point3D) -> Vector3D {
    let ax = a.x - c.x;
    let ay = a.y - c.y;
    let az = a.z - c.z;

    let bx = b.x - c.x;
    let by = b.y - c.y;
    let bz = b.z - c.z;

    Vector3D {
        x: ay * bz - az * by,
        y: az * bx - ax * bz,
        z: ax * by - ay * bx,
    }
}
```
```rust
#[derive(Debug, Clone, Copy)]
struct Ray {
    s: Point3D,
    v: Vector3D,
    v_inv_length_squared: Real,
}
```
```rust
impl Ray {
    fn new(s: Point3D, v: Vector3D) -> Self {
        let len2 = v.length_squared();
        let v_inv_length_squared = if len2 > 0.0 { 1.0 / len2 } else { 0.0 };
        Ray { s, v, v_inv_length_squared }
    }
}
```
```rust
#[inline]
fn squared_distance_point_ray(p: &Point3D, r: &Ray) -> Real {
    let sx = p.x - r.s.x;
    let sy = p.y - r.s.y;
    let sz = p.z - r.s.z;
    let s = Vector3D::new(sx, sy, sz);

    let t = Vector3D::dot_vec(&s, &r.v);
    s.length_squared() - t * t * r.v_inv_length_squared
}
```
```rust
// ========================================
// HalfEdge / Face / MeshBuilder
// ========================================

#[derive(Debug, Clone)]
struct HalfEdge {
    end_vertex: usize,
    opp: usize,
    face: usize,
    next: usize,
}

impl HalfEdge {
    fn disable(&mut self) {
        self.end_vertex = usize::MAX;
    }

    fn is_disabled(&self) -> bool {
        self.end_vertex == usize::MAX
    }
}
```
```rust
#[derive(Debug, Clone)]
struct Face {
    he: usize,
    plane: Plane,
    most_distant_point_dist: Real,
    most_distant_point: usize,
    visibility_checked_on_iteration: usize,
    is_visible_on_current_iteration: bool,
    in_face_stack: bool,
    horizon_edges_bits: u8,
    points_on_positive_side: Vec<usize>,
    disabled: bool,
}
```
```rust
impl Default for Face {
    fn default() -> Self {
        Face {
            he: usize::MAX,
            plane: Plane {
                n: Vector3D::new(0.0, 0.0, 0.0),
                d: 0.0,
                sqr_n_length: 0.0,
            },
            most_distant_point_dist: 0.0,
            most_distant_point: 0,
            visibility_checked_on_iteration: 0,
            is_visible_on_current_iteration: false,
            in_face_stack: false,
            horizon_edges_bits: 0,
            points_on_positive_side: Vec::new(),
            disabled: false,
        }
    }
}
```
```rust
impl Face {
    fn disable(&mut self) {
        self.he = usize::MAX;
        self.disabled = true;
        self.points_on_positive_side.clear();
    }

    fn is_disabled(&self) -> bool {
        self.disabled || self.he == usize::MAX
    }
}
```
```rust
#[derive(Debug, Default, Clone)]
struct MeshBuilder {
    faces: Vec<Face>,
    half_edges: Vec<HalfEdge>,
    disabled_faces: Vec<usize>,
    disabled_half_edges: Vec<usize>,
}
```
```rust
impl MeshBuilder {
    fn new() -> Self {
        MeshBuilder::default()
    }

    fn add_face(&mut self) -> usize {
        if let Some(idx) = self.disabled_faces.pop() {
            let f = &mut self.faces[idx];
            debug_assert!(f.is_disabled());
            f.most_distant_point_dist = 0.0;
            f.disabled = false;
            f.points_on_positive_side.clear();
            idx
        } else {
            self.faces.push(Face::default());
            self.faces.len() - 1
        }
    }
```
```rust
    fn add_half_edge(&mut self) -> usize {
        if let Some(idx) = self.disabled_half_edges.pop() {
            idx
        } else {
            self.half_edges.push(HalfEdge {
                end_vertex: 0,
                opp: 0,
                face: 0,
                next: 0,
            });
            self.half_edges.len() - 1
        }
    }
```
```rust
    fn disable_face(&mut self, face_index: usize) -> Vec<usize> {
        let f = &mut self.faces[face_index];
        let points = std::mem::take(&mut f.points_on_positive_side);
        f.disable();
        self.disabled_faces.push(face_index);
        points
    }
```
```rust
    fn disable_half_edge(&mut self, he_index: usize) {
        let he = &mut self.half_edges[he_index];
        he.disable();
        self.disabled_half_edges.push(he_index);
    }
```
```rust
    /// 초기 tetrahedron ABCD 구성
    fn setup(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self.faces.clear();
        self.half_edges.clear();
        self.disabled_faces.clear();
        self.disabled_half_edges.clear();

        self.faces.reserve(4);
        self.half_edges.reserve(12);

        let mut push_he = |end_vertex, opp, face, next, vec: &mut Vec<HalfEdge>| {
            vec.push(HalfEdge {
                end_vertex,
                opp,
                face,
                next,
            });
        };
```
```rust
        // HalfEdge 12개
        push_he(b, 6, 0, 1, &mut self.half_edges);  // 0: AB
        push_he(c, 9, 0, 2, &mut self.half_edges);  // 1: BC
        push_he(a, 3, 0, 0, &mut self.half_edges);  // 2: CA

        push_he(c, 2, 1, 4, &mut self.half_edges);  // 3: AC
        push_he(d, 11, 1, 5, &mut self.half_edges); // 4: CD
        push_he(a, 7, 1, 3, &mut self.half_edges);  // 5: DA

        push_he(a, 0, 2, 7, &mut self.half_edges);  // 6: BA
        push_he(d, 5, 2, 8, &mut self.half_edges);  // 7: AD
        push_he(b, 10, 2, 6, &mut self.half_edges); // 8: DB

        push_he(b, 1, 3, 10, &mut self.half_edges); // 9: CB
        push_he(d, 8, 3, 11, &mut self.half_edges); // 10: BD
        push_he(c, 4, 3, 9, &mut self.half_edges);  // 11: DC

        // Face 4개
        self.faces.push(Face { he: 0, ..Default::default() }); // ABC
        self.faces.push(Face { he: 3, ..Default::default() }); // ACD
        self.faces.push(Face { he: 6, ..Default::default() }); // BAD
        self.faces.push(Face { he: 9, ..Default::default() }); // CBD
    }
```
```rust
    /// face -> 정점 인덱스 3개
    fn triangle_vertices(&self, face_index: usize) -> [usize; 3] {
        let mut v = [0usize; 3];
        let mut he = &self.half_edges[self.faces[face_index].he];
        v[0] = he.end_vertex;
        he = &self.half_edges[he.next];
        v[1] = he.end_vertex;
        he = &self.half_edges[he.next];
        v[2] = he.end_vertex;
        v
    }

    /// half-edge -> (from, to)
    fn edge_vertices(&self, he: &HalfEdge) -> [usize; 2] {
        [self.half_edges[he.opp].end_vertex, he.end_vertex]
    }
}
```
```rust
// ========================================
// Diagnostics / FaceData / QuickHull3D
// ========================================

#[derive(Debug, Default, Clone)]
pub struct DiagnosticsData {
    pub failed_horizon_edges: usize,
}
```
```rust
#[derive(Debug, Clone, Copy)]
struct FaceData {
    face_index: usize,
    entered_from_half_edge: usize,
}
```
```rust
#[derive(Debug)]
struct QuickHull3D {
    epsilon: Real,
    epsilon_squared: Real,
    scale: Real,
    planar: bool,
    points: Vec<Point3D>,
    mesh: MeshBuilder,
    extreme_values: [usize; 6],
    diagnostics: DiagnosticsData,

    new_face_indices: Vec<usize>,
    new_half_edge_indices: Vec<usize>,
    disabled_face_point_vectors: Vec<Vec<usize>>,
    visible_faces: Vec<usize>,
    horizon_edges: Vec<usize>,
    possibly_visible_faces: Vec<FaceData>,
    face_list: VecDeque<usize>,
}
```
```rust
impl QuickHull3D {
    fn new() -> Self {
        QuickHull3D {
            epsilon: 0.0,
            epsilon_squared: 0.0,
            scale: 1.0,
            planar: false,
            points: Vec::new(),
            mesh: MeshBuilder::new(),
            extreme_values: [0; 6],
            diagnostics: DiagnosticsData::default(),
            new_face_indices: Vec::new(),
            new_half_edge_indices: Vec::new(),
            disabled_face_point_vectors: Vec::new(),
            visible_faces: Vec::new(),
            horizon_edges: Vec::new(),
            possibly_visible_faces: Vec::new(),
            face_list: VecDeque::new(),
        }
    }
```
```rust
    #[inline]
    fn default_eps() -> Real {
        1.0e-7
    }
```
```rust
    fn compute_convex_hull(
        &mut self,
        input_points: &[Point3D],
        ccw: bool,
        use_original_indices: bool,
        eps: Real,
    ) -> ConvexHull {
        self.build_mesh(input_points, eps);
        self.build_convex_hull_result(ccw, use_original_indices)
    }
```
```rust
    fn build_mesh(&mut self, input_points: &[Point3D], epsilon: Real) {
        if input_points.is_empty() {
            self.mesh = MeshBuilder::new();
            self.points.clear();
            return;
        }

        self.points.clear();
        self.points.extend_from_slice(input_points);

        self.extreme_values = self.get_extreme_values();
        self.scale = self.get_scale(&self.extreme_values);

        self.epsilon = epsilon * self.scale;
        self.epsilon_squared = self.epsilon * self.epsilon;

        self.diagnostics = DiagnosticsData::default();
        self.planar = false;

        self.create_convex_half_edge_mesh();

        if self.planar {
            // planar인 경우, 마지막 임시 포인트 제거 및 인덱스 정리
            let extra_point_index = self.points.len() - 1;
            for he in &mut self.mesh.half_edges {
                if he.end_vertex == extra_point_index {
                    he.end_vertex = 0;
                }
            }
            self.points.pop();
        }
    }
```
```rust
    fn build_convex_hull_result(&self, ccw: bool, use_original_indices: bool) -> ConvexHull {
        let mut indices = Vec::<usize>::new();
        let i_ccw = if ccw { 1usize } else { 0usize };

        let final_face_count = self
            .mesh
            .faces
            .iter()
            .filter(|f| !f.is_disabled())
            .count();
        indices.reserve(final_face_count * 3);

        for (fi, face) in self.mesh.faces.iter().enumerate() {
            if face.is_disabled() {
                continue;
            }
            let v = self.mesh.triangle_vertices(fi);
            indices.push(v[0]);
            indices.push(v[1 + i_ccw]);
            indices.push(v[2 - i_ccw]);
        }

        if use_original_indices {
            ConvexHull {
                vertices: self.points.clone(),
                indices,
            }
        } else {
            use std::collections::HashMap;
            let mut map: HashMap<usize, usize> = HashMap::new();
            let mut new_vertices: Vec<Point3D> = Vec::new();

            for idx in &mut indices {
                let v_idx = *idx;
                let entry = map.entry(v_idx).or_insert_with(|| {
                    let new_idx = new_vertices.len();
                    new_vertices.push(self.points[v_idx]);
                    new_idx
                });
                *idx = *entry;
            }

            ConvexHull {
                vertices: new_vertices,
                indices,
            }
        }
    }
```
```rust
    fn get_extreme_values(&self) -> [usize; 6] {
        let mut out = [0usize; 6];
        if self.points.is_empty() {
            return out;
        }

        let p0 = self.points[0];
        let mut extreme_vals = [
            p0.x, p0.x, // max x, min x
            p0.y, p0.y, // max y, min y
            p0.z, p0.z, // max z, min z
        ];

        let v_count = self.points.len();
        for i in 1..v_count {
            let pos = self.points[i];

            if pos.x > extreme_vals[0] {
                extreme_vals[0] = pos.x;
                out[0] = i;
            } else if pos.x < extreme_vals[1] {
                extreme_vals[1] = pos.x;
                out[1] = i;
            }

            if pos.y > extreme_vals[2] {
                extreme_vals[2] = pos.y;
                out[2] = i;
            } else if pos.y < extreme_vals[3] {
                extreme_vals[3] = pos.y;
                out[3] = i;
            }

            if pos.z > extreme_vals[4] {
                extreme_vals[4] = pos.z;
                out[4] = i;
            } else if pos.z < extreme_vals[5] {
                extreme_vals[5] = pos.z;
                out[5] = i;
            }
        }

        out
    }
```
```rust
    fn get_scale(&self, extremes: &[usize; 6]) -> Real {
        let mut s = 0.0;
        for i in 0..6 {
            let v = &self.points[extremes[i]];
            let coord = match i / 2 {
                0 => v.x,
                1 => v.y,
                _ => v.z,
            };
            let a = coord.abs();
            if a > s {
                s = a;
            }
        }
        if s == 0.0 { 1.0 } else { s }
    }
```
```rust
    fn setup_initial_tetrahedron(&mut self) {
        let v_count = self.points.len();

        if v_count <= 4 {
            let v0 = 0usize;
            let v1 = if v_count > 1 { 1 } else { 0 };
            let v2 = if v_count > 2 { 2 } else { v1 };
            let v3 = if v_count > 3 { 3 } else { v2 };

            let p0 = self.points[v0];
            let p1 = self.points[v1];
            let p2 = self.points[v2];
            let p3 = self.points[v3];

            let n = triangle_normal(&p0, &p1, &p2);
            let tri_plane = Plane::new(n, p0);
            let mut verts = [v0, v1, v2, v3];

            if tri_plane.is_point_on_positive_side(&p3) {
                verts.swap(0, 1);
            }

            self.mesh.setup(verts[0], verts[1], verts[2], verts[3]);
            return;
        }

        // 극값들 사이에서 가장 먼 두 점
        let mut max_d = self.epsilon_squared;
        let mut selected = (0usize, 0usize);

        for i in 0..6 {
            for j in (i + 1)..6 {
                let pi = self.points[self.extreme_values[i]];
                let pj = self.points[self.extreme_values[j]];
                let d = Point3D::distance_squared_point(&pi, &pj);
                if d > max_d {
                    max_d = d;
                    selected = (self.extreme_values[i], self.extreme_values[j]);
                }
            }
        }

        if max_d == self.epsilon_squared {
            // 거의 한 점에 모여있는 경우
            let v0 = 0usize;
            let v1 = if v_count > 1 { 1 } else { 0 };
            let v2 = if v_count > 2 { 2 } else { v1 };
            let v3 = if v_count > 3 { 3 } else { v2 };
            self.mesh.setup(v0, v1, v2, v3);
            return;
        }

        // 두 extreme 점과 가장 먼 점으로 base 선분 설정
        let r = Ray::new(
            self.points[selected.0],
            Vector3D::from_points(&self.points[selected.0], &self.points[selected.1]),
        );

        max_d = self.epsilon_squared;
        let mut max_i = usize::MAX;

        for i in 0..v_count {
            let dist = squared_distance_point_ray(&self.points[i], &r);
            if dist > max_d {
                max_d = dist;
                max_i = i;
            }
        }

        if max_d == self.epsilon_squared {
            // 거의 1차원인 경우
            let mut third_point = selected.0;
            let mut fourth_point = selected.0;

            for (idx, _) in self.points.iter().enumerate() {
                if idx != selected.0 && idx != selected.1 {
                    third_point = idx;
                    break;
                }
            }
            for (idx, _) in self.points.iter().enumerate() {
                if idx != selected.0 && idx != selected.1 && idx != third_point {
                    fourth_point = idx;
                    break;
                }
            }

            self.mesh.setup(selected.0, selected.1, third_point, fourth_point);
            return;
        }

        // base triangle
        let base_triangle = [selected.0, selected.1, max_i];
        let base_vertices = [
            self.points[base_triangle[0]],
            self.points[base_triangle[1]],
            self.points[base_triangle[2]],
        ];

        // base 평면에서 가장 멀리 떨어진 네 번째 점
        let n = triangle_normal(&base_vertices[0], &base_vertices[1], &base_vertices[2]);
        let tri_plane = Plane::new(n, base_vertices[0]);

        max_d = self.epsilon;
        max_i = 0;
        for i in 0..v_count {
            let d = tri_plane.signed_distance(&self.points[i]).abs();
            if d > max_d {
                max_d = d;
                max_i = i;
            }
        }

        if max_d == self.epsilon {
            // 2D planar cloud: 평면 밖으로 약간 밀어낸 점 추가
            self.planar = true;
            let n1 = triangle_normal(&base_vertices[1], &base_vertices[2], &base_vertices[0]);
            let extra_point = Point3D {
                x: self.points[0].x + n1.x,
                y: self.points[0].y + n1.y,
                z: self.points[0].z + n1.z,
            };
            self.points.push(extra_point);
            max_i = self.points.len() - 1;
        }

        // 네 번째 점 기준으로 base orientation 조정
        let p4 = self.points[max_i];
        let tri_plane2 = Plane::new(n, base_vertices[0]);
        let mut base = base_triangle;
        if tri_plane2.is_point_on_positive_side(&p4) {
            base.swap(0, 1);
        }

        // tetrahedron 생성
        self.mesh.setup(base[0], base[1], base[2], max_i);

        // 각 face 의 평면 계산
        let face_count = self.mesh.faces.len();
        for fi in 0..face_count {
            let v_idx = self.mesh.triangle_vertices(fi);
            let va = self.points[v_idx[0]];
            let vb = self.points[v_idx[1]];
            let vc = self.points[v_idx[2]];
            let n1 = triangle_normal(&va, &vb, &vc);
            self.mesh.faces[fi].plane = Plane::new(n1, va);
        }

        // tetrahedron 밖에 있는 점들을 face 에 할당
        for i in 0..v_count {
            for face_idx in 0..self.mesh.faces.len() {
                if self.add_point_to_face(face_idx, i) {
                    break;
                }
            }
        }
    }
```
```rust
    fn add_point_to_face(&mut self, face_index: usize, point_index: usize) -> bool {
        let face_plane;
        {
            let face = &self.mesh.faces[face_index];
            if face.is_disabled() {
                return false;
            }
            face_plane = face.plane;
        }

        let d = face_plane.signed_distance(&self.points[point_index]);
        if d > 0.0 && d * d > self.epsilon_squared * face_plane.sqr_n_length {
            let face = &mut self.mesh.faces[face_index];
            face.points_on_positive_side.push(point_index);
            if d > face.most_distant_point_dist {
                face.most_distant_point_dist = d;
                face.most_distant_point = point_index;
            }
            true
        } else {
            false
        }
    }
```
```rust
    fn create_convex_half_edge_mesh(&mut self) {
        self.visible_faces.clear();
        self.horizon_edges.clear();
        self.possibly_visible_faces.clear();

        // base tetrahedron
        self.setup_initial_tetrahedron();
        debug_assert_eq!(self.mesh.faces.len(), 4);

        // face stack 초기화
        self.face_list.clear();
        for (i, f) in self.mesh.faces.iter_mut().enumerate().take(4) {
            if !f.points_on_positive_side.is_empty() {
                self.face_list.push_back(i);
                f.in_face_stack = true;
            }
        }

        let mut iter: usize = 0;

        while let Some(top_face_index) = self.face_list.pop_front() {
            iter = iter.wrapping_add(1);

            // 1) top_face 한 번만 가변 빌려서 정보 꺼내기
            let active_point_index;
            {
                let tf = &mut self.mesh.faces[top_face_index];
                tf.in_face_stack = false;
                if tf.points_on_positive_side.is_empty() || tf.is_disabled() {
                    continue;
                }
                active_point_index = tf.most_distant_point;
            } // 여기서 tf에 대한 &mut borrow 해제

            let active_point = self.points[active_point_index];

            // 2) visible faces / horizon edge 찾기
            self.horizon_edges.clear();
            self.possibly_visible_faces.clear();
            self.visible_faces.clear();

            self.possibly_visible_faces.push(FaceData {
                face_index: top_face_index,
                entered_from_half_edge: usize::MAX,
            });

            while let Some(face_data) = self.possibly_visible_faces.pop() {
                let fi = face_data.face_index;
                let entered_he = face_data.entered_from_half_edge;

                // 먼저 plane / he 인덱스 정보만 불변으로 읽기
                let (plane, he_indices_prev) = {
                    let f = &self.mesh.faces[fi];
                    let he0 = f.he;
                    let he1 = self.mesh.half_edges[he0].next;
                    let he2 = self.mesh.half_edges[he1].next;
                    (f.plane, [he0, he1, he2])
                };

                // 거리 계산
                let d = plane.signed_distance(&active_point);

                let is_visible = d > 0.0;

                // 이제 가변 참조로 상태 업데이트
                {
                    let pvf = &mut self.mesh.faces[fi];
                    if pvf.visibility_checked_on_iteration == iter {
                        // 이미 체크했으면, 이전 결과만 사용
                        if pvf.is_visible_on_current_iteration {
                            continue;
                        }
                    } else {
                        pvf.visibility_checked_on_iteration = iter;
                        if is_visible {
                            pvf.is_visible_on_current_iteration = true;
                            pvf.horizon_edges_bits = 0;
                        } else {
                            pvf.is_visible_on_current_iteration = false;
                        }
                    }
                }

                if is_visible {
                    self.visible_faces.push(fi);

                    // 이 face에서 나가는 half-edge들을 따라 인접 face push
                    for &he_idx in &he_indices_prev {
                        if self.mesh.half_edges[he_idx].opp != entered_he {
                            let opp_face =
                                self.mesh.half_edges[self.mesh.half_edges[he_idx].opp].face;
                            self.possibly_visible_faces.push(FaceData {
                                face_index: opp_face,
                                entered_from_half_edge: he_idx,
                            });
                        }
                    }
                    continue;
                }

                // visible이 아니면 horizon edge 후보
                if entered_he != usize::MAX {
                    self.horizon_edges.push(entered_he);

                    // 어떤 half-edge가 horizon인지 bit로 기록
                    let face_for_he = self.mesh.half_edges[entered_he].face;
                    let (he0, he1, he2) = {
                        let f = &self.mesh.faces[face_for_he];
                        let he0 = f.he;
                        let he1 = self.mesh.half_edges[he0].next;
                        let he2 = self.mesh.half_edges[he1].next;
                        (he0, he1, he2)
                    };
                    let he_indices = [he0, he1, he2];

                    let ind = if he_indices[0] == entered_he {
                        0
                    } else if he_indices[1] == entered_he {
                        1
                    } else {
                        2
                    };

                    let f = &mut self.mesh.faces[face_for_he];
                    f.horizon_edges_bits |= 1 << ind;
                }
            }

            let horizon_edge_count = self.horizon_edges.len();

            // 3) horizon edge 루프 정렬 실패 시 처리
            if !reorder_horizon_edges(&self.mesh, &mut self.horizon_edges) {
                self.diagnostics.failed_horizon_edges += 1;

                let tf = &mut self.mesh.faces[top_face_index];
                if let Some(pos) = tf
                    .points_on_positive_side
                    .iter()
                    .position(|&idx| idx == active_point_index)
                {
                    tf.points_on_positive_side.remove(pos);
                }
                continue;
            }

            // 4) visible faces 삭제 + half-edge 재사용/disable
            self.new_face_indices.clear();
            self.new_half_edge_indices.clear();
            self.disabled_face_point_vectors.clear();

            let mut disable_counter = 0usize;
            for &face_idx in &self.visible_faces {
                // 먼저 불변으로 half-edge 인덱스 / horizon bit 읽기
                let (he_indices, horizon_bits) = {
                    let face = &self.mesh.faces[face_idx];
                    let he0 = face.he;
                    let he1 = self.mesh.half_edges[he0].next;
                    let he2 = self.mesh.half_edges[he1].next;
                    ([he0, he1, he2], face.horizon_edges_bits)
                };

                // 이제 mesh를 가변으로 빌려서 half-edge 재사용/disable
                for j in 0..3 {
                    if (horizon_bits & (1 << j)) == 0 {
                        if disable_counter < horizon_edge_count * 2 {
                            self.new_half_edge_indices.push(he_indices[j]);
                            disable_counter += 1;
                        } else {
                            self.mesh.disable_half_edge(he_indices[j]);
                        }
                    }
                }

                let points = self.mesh.disable_face(face_idx);
                if !points.is_empty() {
                    self.disabled_face_point_vectors.push(points);
                }
            }

            if disable_counter < horizon_edge_count * 2 {
                let needed = horizon_edge_count * 2 - disable_counter;
                for _ in 0..needed {
                    self.new_half_edge_indices.push(self.mesh.add_half_edge());
                }
            }

            // 5) horizon edge 루프를 따라 새로운 face 생성
            for i in 0..horizon_edge_count {
                let ab = self.horizon_edges[i];
                let ab_he = &self.mesh.half_edges[ab];

                let v_ab = self.mesh.edge_vertices(&ab_he);
                let a = v_ab[0];
                let b = v_ab[1];
                let c = active_point_index;

                let new_face_index = self.mesh.add_face();
                self.new_face_indices.push(new_face_index);

                let ca = self.new_half_edge_indices[2 * i + 0];
                let bc = self.new_half_edge_indices[2 * i + 1];

                {
                    let he_ab = &mut self.mesh.half_edges[ab];
                    he_ab.next = bc;
                    he_ab.face = new_face_index;
                }
                {
                    let he_bc = &mut self.mesh.half_edges[bc];
                    he_bc.next = ca;
                    he_bc.face = new_face_index;
                    he_bc.end_vertex = c;
                }
                {
                    let he_ca = &mut self.mesh.half_edges[ca];
                    he_ca.next = ab;
                    he_ca.face = new_face_index;
                    he_ca.end_vertex = a;
                }

                let plane_normal = triangle_normal(&self.points[a], &self.points[b], &active_point);
                {
                    let new_face = &mut self.mesh.faces[new_face_index];
                    new_face.plane = Plane::new(plane_normal, active_point);
                    new_face.he = ab;
                }

                // CA edge 의 반대편 edge
                {
                    let ca_opp = if i > 0 {
                        self.new_half_edge_indices[i * 2 - 1]
                    } else {
                        self.new_half_edge_indices[2 * horizon_edge_count - 1]
                    };
                    self.mesh.half_edges[ca].opp = ca_opp;
                }

                // BC edge 의 반대편 edge
                {
                    let bc_opp =
                        self.new_half_edge_indices[((i + 1) * 2) % (horizon_edge_count * 2)];
                    self.mesh.half_edges[bc].opp = bc_opp;
                }
            }

            // 6) disable된 face 들에서 온 점들을 새 face 로 재할당
            let disabled_vectors = std::mem::take(&mut self.disabled_face_point_vectors);

            // 6-2. new_face_indices 도 따로 복사해서 self와 분리
            let new_faces = self.new_face_indices.clone();

            for points in disabled_vectors {
                for point in points {
                    if point == active_point_index {
                        continue;
                    }
                    // 여기서는 self에 대한 borrow 없음 (new_faces는 로컬 변수)
                    for &new_face_index in &new_faces {
                        if self.add_point_to_face(new_face_index, point) {
                            break;
                        }
                    }
                }
            }

            // 7) 새 face 중에서 positive side 점이 있는 face 만 다시 stack에 push
            for &new_face_index in &self.new_face_indices {
                let new_face = &mut self.mesh.faces[new_face_index];
                if !new_face.points_on_positive_side.is_empty() && !new_face.in_face_stack {
                    self.face_list.push_back(new_face_index);
                    new_face.in_face_stack = true;
                }
            }
        }
    }
}
```
```rust
// ========================================
// horizon edge 재정렬 (자유 함수)
// ========================================

fn reorder_horizon_edges(mesh: &MeshBuilder, horizon_edges: &mut Vec<usize>) -> bool {
    let count = horizon_edges.len();
    if count <= 1 {
        return true;
    }

    for i in 0..(count - 1) {
        let end_vertex = mesh.half_edges[horizon_edges[i]].end_vertex;
        let mut found_next = false;

        for j in (i + 1)..count {
            let he_idx = horizon_edges[j];
            let begin_vertex = mesh.half_edges[mesh.half_edges[he_idx].opp].end_vertex;
            if begin_vertex == end_vertex {
                horizon_edges.swap(i + 1, j);
                found_next = true;
                break;
            }
        }

        if !found_next {
            return false;
        }
    }

    let last_end = mesh.half_edges[*horizon_edges.last().unwrap()].end_vertex;
    let first_begin = mesh.half_edges[mesh.half_edges[horizon_edges[0]].opp].end_vertex;
    debug_assert_eq!(last_end, first_begin);

    true
}
```
```rust
// ========================================
// public API: ConvexHull
// ========================================

impl ConvexHull {
    pub fn from_points(points: &[Point3D], ccw: bool, use_original_indices: bool) -> Self {
        Self::from_points_with_eps(points, ccw, use_original_indices, QuickHull3D::default_eps())
    }

    pub fn from_points_with_eps(
        points: &[Point3D],
        ccw: bool,
        use_original_indices: bool,
        eps: Real,
    ) -> Self {
        let mut qh = QuickHull3D::new();
        qh.compute_convex_hull(points, ccw, use_original_indices, eps)
    }
}
```


---

## 테스트 코드

```rust
// ========================================
// 간단 테스트
// ========================================

#[cfg(test)]
mod tests {
    use super::ConvexHull;
    use crate::core::geom::Point3D;
    use crate::core::types::Real;

    fn p(x: Real, y: Real, z: Real) -> Point3D {
        Point3D { x, y, z }
    }

    #[test]
    fn convex_hull_tetrahedron_basic() {
        let pts = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(0.0, 1.0, 0.0),
            p(0.0, 0.0, 1.0),
        ];

        let hull = ConvexHull::from_points(&pts, true, true);

        assert_eq!(hull.vertices.len(), 4);
        assert_eq!(hull.indices.len(), 12);

        for &idx in &hull.indices {
            assert!(idx < 4, "index {} out of range", idx);
        }
    }

    #[test]
    fn convex_hull_cube_basic() {
        let pts = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.0, 1.0, 0.0),
            p(0.0, 1.0, 0.0),
            p(0.0, 0.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
            p(0.0, 1.0, 1.0),
        ];

        let hull = ConvexHull::from_points(&pts, true, false);

        assert_eq!(hull.vertices.len(), 8);
        assert!(hull.indices.len() >= 36 && hull.indices.len() % 3 == 0);
    }
}
```
```rust
pub fn export_convex_hull_stl(
    input_path: &str,
    output_path: &str,
    binary: bool,
) -> std::io::Result<()> {
    // 1) 원본 STL 읽어서 Mesh 구성
    let mut src_mesh = Mesh::default();
    StlReader::run(input_path, &mut src_mesh)?;

    // 2) Mesh 정점들로 ConvexHull 계산
    //    ConvexHull::from_points 가 Point3D의 슬라이스를 받는다고 가정
    let points = src_mesh.vertices.clone();
    let hull = ConvexHull::from_points(&points, true, false);

    // 3) ConvexHull 결과를 Mesh로 변환
    let mut hull_mesh = Mesh::default();
    hull_mesh.vertices = hull.vertices.clone();

    // hull.indices 는 3개씩 하나의 삼각형
    hull_mesh.faces.reserve(hull.indices.len() / 3);
    for tri in hull.indices.chunks(3) {
        if tri.len() < 3 {
            continue;
        }
        let f = [
            tri[0] as u32,
            tri[1] as u32,
            tri[2] as u32,
            tri[2] as u32, // 기존 Mesh가 [u32;4] 구조라서 마지막은 3번 인덱스를 복사
        ];
        hull_mesh.faces.push(f);
    }

    // 4) 노멀 계산
    hull_mesh.compute_normals();

    println!("is_watertight(&hull_mesh) = {}", is_watertight(&hull_mesh));

    // 5) STL로 내보내기
    StlWriter::run(output_path, &hull_mesh, binary)
}

```
```rust

pub fn is_watertight(mesh: &Mesh) -> bool {
    let mut edge_count: HashMap<(u32, u32), i32> = HashMap::new();

    for f in &mesh.faces {
        let v = [f[0], f[1], f[2]];
        for k in 0..3 {
            let a = v[k];
            let b = v[(k + 1) % 3];

            // 무향 edge로 정규화
            let key = if a < b { (a, b) } else { (b, a) };
            *edge_count.entry(key).or_insert(0) += 1;
        }
    }

    // watertight라면 모든 edge가 정확히 두 번 등장
    edge_count.values().all(|&c| c == 2)
}
```

---


