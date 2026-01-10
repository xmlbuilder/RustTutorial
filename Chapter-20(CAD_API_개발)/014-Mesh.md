#  Mesh
- 메시 구조, 정점 최적화, 면 정렬, 평면성 검사, 위상 구조(Topology)까지 포함된 매우 정교한 메시 처리 모듈입니다.  
- 아래에 요청하신 내용을 체계적으로 정리해드릴게요.

## 📘 1. 전체 함수 정리 및 역할 요약
### 🔹 메시 구조 관련 함수
| 함수 이름               | 역할 설명                                                                 |
|------------------------|----------------------------------------------------------------------------|
| `Mesh::new`              | 정점과 면(face) 데이터를 받아 새로운 메시 객체를 생성합니다.               |
| `Mesh::default`          | 빈 메시를 생성합니다. 정점, 면, 법선이 모두 비어 있습니다.                 |
| `Mesh::triangle_count`   | 메시 내 면(face)의 개수를 반환합니다. 삼각형 개수로 간주됩니다.            |
| `Mesh::compute_normals`  | 각 면의 법선 벡터를 계산하여 `normals` 필드에 저장합니다.                  |
| `Mesh::optimize_mesh`    | 중복된 정점을 제거하고 인덱스를 재매핑하여 메시를 최적화합니다.           |
| `Mesh::filter_planar_faces` | 평면성 기준에 따라 메시에서 평면(face)만 필터링하여 인덱스를 반환합니다. |

### 🔹 메시 변환 및 정리 함수
| 함수 이름                   | 역할 설명                                                                 |
|----------------------------|----------------------------------------------------------------------------|
| `on_face_is_tri`             | 면이 삼각형인지 확인 (v2 == v3이면 삼각형으로 간주)                        |
| `on_mesh_to_tri_list`        | 메시를 삼각형 리스트로 변환 (사각형은 두 삼각형으로 분할)                  |
| `on_tri_list_to_mesh`        | 삼각형 리스트를 메시로 변환 (v2 == v3로 설정하여 삼각형 표현)             |
| `on_quad_list_to_mesh`       | 사각형 리스트를 메시로 변환                                                |
| `on_force_triangulate`       | 메시의 모든 사각형 면을 두 개의 삼각형으로 분할                            |
| `on_weld_vertices`           | 좌표 기반으로 정점을 병합 (eps 기준으로 근접 정점 통합)                   |
| `on_compact_vertices`        | 사용되지 않는 정점을 제거하고 인덱스를 재매핑                             |
| `on_apply_transform`         | 메시 정점에 변환 행렬 적용 (법선은 재계산 권장)                           |
| `on_extract_planar_sub_mesh` | 평면성 기준에 따라 평면 면만 추출하여 서브 메시 생성                       |
| `on_dedup_faces`             | 동일한 정점 조합을 가진 중복 면 제거                                      |
| `on_merge_meshes`            | 두 메시를 병합하여 하나의 메시로 생성 (Topology 기반 병합)                |


## 📐 2. 사용된 주요 수식 정리
| 수식 항목                     | 수식 표현                                                                                   | 의미 및 사용 위치                          |
|------------------------------|----------------------------------------------------------------------------------------------|--------------------------------------------|
| 면의 법선 벡터               | 𝑛 = (v₁ − v₀) × (v₂ − v₀)                                                                   | 삼각형 또는 사각형 면의 법선 계산          |
| 평면 방정식                  | ax + by + cz + d = 0                                                                        | 면이 놓인 평면의 방정식                    |
| 평면 편차 조건               | max(hᵢ − h₀) − min(hᵢ − h₀) ≤ tol                                                           | 사각형 면의 평면성 검사                    |
| 코너 노멀 각도 조건          | cos(θ) = 𝑛ᵢ · 𝑛ⱼ ≥ cos(angle_tol)                                                          | 사각형의 대각 코너 노멀 각도 검사          |
| 정점 병합 키                 | key = (⌊x/ε⌋, ⌊y/ε⌋, ⌊z/ε⌋)                                                                 | 정점 좌표 기반 병합 키 (해시)              |
| 테트라 체적 계산             | V = (1/6) · v₁ · (v₂ × v₃)                                                                  | 원점 기준 삼각형 기반 테트라 체적 계산     |


## 🧠 3. Topology 구조 점검
### ✅ 구성 요소
| 필드 이름   | 역할 설명                                                                 |
|-------------|----------------------------------------------------------------------------|
| `topv_map`    | 메시 정점 인덱스 → 위상 정점 인덱스 매핑 테이블                           |
| `topv`        | 위상 정점(TopoVertex) 목록. 메시 정점들과 연결된 엣지 정보 포함           |
| `tope`        | 위상 엣지(TopoEdge) 목록. 두 위상 정점 사이의 엣지 및 연결된 면 정보 포함 |
| `topf`        | 위상 면(TopoFace) 목록. 엣지 인덱스와 방향 정보(reve) 포함                |
| `keymap`      | 좌표 기반 정점 병합용 해시. (x,y,z) → topv 인덱스                         |
| `edge_map`    | 엣지 중복 방지용 해시. (min_vi, max_vi) → tope 인덱스                     |


### ✅ 생성 흐름
- from_mesh()에서 정점 병합 → topv, topv_map, keymap 생성
- 각 면의 엣지 추출 → tope, edge_map 생성
- 면 → topf 생성, 엣지 방향 정보 포함
- 각 정점에 연결된 엣지 → tope_indices로 연결

### ✅ 검증 결과
- 정점 병합: eps 기반 좌표 정규화 → ✅ 정상
- 엣지 생성: (min, max) 정렬로 방향성 유지 → ✅ 정상
- 면 생성: 삼각형/사각형 모두 대응 → ✅ 정상
- compact()로 불필요한 위상 요소 제거 가능 → ✅ 정상
- all_boundary_loops()로 경계 루프 추출 가능 → ✅ 정상
- ✅ Topology는 구조적으로 완전하고, 메시와의 연결도 잘 구성되어 있습니다.

## ✅ 요약 평가
| 단계 | 함수 이름               | 주요 역할 설명                                                                 |
|------|------------------------|--------------------------------------------------------------------------------|
| ①    | Topology::from_mesh    | 메시로부터 위상 구조 생성: 정점 병합, 엣지 추출, 면 구성                        |
| ②    | Topology::from_mesh    | topv_map, topv, tope, topf, keymap, edge_map 초기화 및 연결                     |
| ③    | Topology::compact      | 사용되지 않는 위상 요소 제거: 유효한 face/edge/vertex 필터링                   |
| ④    | Topology::compact      | 새 인덱스 remap 생성: vertex_remap, edge_remap, face_remap                     |
| ⑤    | Topology::compact      | 구조 재작성: new_topv, new_tope, new_topf, new_topv_map                        |
| ⑥    | Topology::compact      | keymap, edge_map 재생성                                                         |
| ⑦    | Topology::compact      | 최종 구조 교체 및 mesh_face_count, mesh_vertex_count 갱신                      |

## ✅ 구성 요소와 흐름 관계
- from_mesh()는 메시 기반 위상 구조를 생성합니다.
- compact()는 위상 구조에서 사용되지 않는 요소를 제거하고 재정렬합니다.
- 두 함수는 함께 사용될 때 메시의 위상 정보를 정확하고 효율적으로 관리할 수 있게 해줍니다.

---
# Topology
- 아래에 TopoVertex, TopoEdge, TopoFace의 필드 요약과 경계 루프 추출 흐름을 표 형식으로 정리.

## 🧩 Topology 구성 요소: 구조체 필드 요약
### 🔹 TopoVertex
| 필드 이름        | 설명                                      |
|------------------|-------------------------------------------|
| `tope_indices`     | 이 정점에 연결된 위상 엣지 인덱스 목록     |
| `mesh_vertices`    | 이 위상 정점에 대응되는 메시 정점 인덱스들 |


### 🔹 TopoEdge
| 필드 이름        | 설명                                      |
|------------------|-------------------------------------------|
| `topv`             | 엣지를 구성하는 두 위상 정점 인덱스        |
| `topf_indices`     | 이 엣지에 연결된 위상 면 인덱스 목록       |


### 🔹 TopoFace
| 필드 이름        | 설명                                      |
|------------------|-------------------------------------------|
| `tope`             | 이 면을 구성하는 4개의 엣지 인덱스         |
| `reve`             | 각 엣지의 방향 반전 여부 (true면 반대방향) |

## 🔁 경계 루프 추출 흐름 요약 (Topology::all_boundary_loops())
### 1. 경계 엣지 탐색
   - 엣지에 연결된 면이 1개뿐이면 경계 엣지로 간주
   - 각 위상 정점(topv)에 연결된 경계 엣지를 기록

### 2. 루프 시작점 선택
   - 아직 방문하지 않은 경계 엣지를 기준으로 루프 탐색 시작

### 3. 루프 추적
   - 현재 정점에서 연결된 경계 엣지를 따라 다음 정점으로 이동
   - 시작 정점으로 되돌아오면 루프 종료

### 4. 루프 저장
   - 닫힌 루프일 경우 정점 시퀀스를 저장
   - 열린 경계는 무시하거나 별도 처리 가능

### 5. 모든 경계 루프 수집 완료


## ✅ 경계 루프 특징
- 경계 엣지는 topf_indices.len() == 1인 엣지
- 루프는 Vec<usize> 형태로 위상 정점 인덱스 시퀀스를 반환
- 복수 루프 지원 (복잡한 메시에서도 안정적)
- 이제 Topology의 핵심 구조와 경계 추출 알고리즘이 명확하게 정리되었습니다.

---

## 소스 코드

```rust
use crate::core::domain::on_fix_rev_angle_interval_2pi;
use crate::core::geom::Point2;
use crate::core::plane::Plane;
use crate::core::plane_equation::PlaneEquation;
use crate::core::point_ops::PointOps;
use crate::core::prelude::{Interval, Point, Vector};
use crate::core::transform::Transform;
use crate::core::types::ON_ZERO_TOL;
use ordered_float::NotNan;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f64::EPSILON;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshFace {
    pub vi: [i32; 4], // tri: vi[2]==vi[3]
}
```
```rust
impl MeshFace {
    #[inline]
    pub fn new_tri(v0: i32, v1: i32, v2: i32) -> Self {
        Self {
            vi: [v0, v1, v2, v2],
        }
    }
```
```rust
    #[inline]
    pub fn new_quad(v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
        Self {
            vi: [v0, v1, v2, v3],
        }
    }
```
```rust
    #[inline]
    pub fn is_triangle(&self) -> bool {
        self.vi[2] == self.vi[3]
    }
```
```rust
    #[inline]
    pub fn is_quad(&self) -> bool {
        self.vi[2] != self.vi[3]
    }
```
```rust
    /// CCW/CW 맞출 때 사용. 삼각형은 (1,2) 스왑, 사각형은 (1,3) 스왑.
    #[inline]
    pub fn flip(&mut self) {
        if self.is_triangle() {
            self.vi.swap(1, 2);
        } else {
            self.vi.swap(1, 3);
        }
    }
```
```rust
    pub fn face_is_tri(&self) -> bool {
        self.vi[2] == self.vi[3]
    }
```
```rust
    pub fn is_valid(&self, vertex_count: usize) -> bool {
        let ok_index = |i: i32| -> bool {
            let u = if i >= 0 { i as usize } else { usize::MAX };
            u < vertex_count
        };
        if !ok_index(self.vi[0])
            || !ok_index(self.vi[1])
            || !ok_index(self.vi[2])
            || !ok_index(self.vi[3])
        {
            return false;
        }
        // 동일 인덱스 중복은 비권장 (삼각형은 v2==v3 허용)
        let v0 = self.vi[0];
        let v1 = self.vi[1];
        let v2 = self.vi[2];
        let v3 = self.vi[3];
        if self.is_triangle() {
            !(v0 == v1 || v1 == v2 || v0 == v2)
        } else {
            !(v0 == v1 || v1 == v2 || v2 == v3 || v3 == v0)
        }
    }
```
```rust
    pub fn compute_face_normal_from_dv(
        face: &MeshFace,
        verts: &[Point],
        out_n: &mut Vector,
    ) -> bool {
        // 인덱스 안전 체크는 호출측 가정 (성능위해 생략 가능)
        let i0 = face.vi[0] as usize;
        let i1 = face.vi[1] as usize;
        let i2 = face.vi[2] as usize;
        let i3 = face.vi[3] as usize;

        let a = verts[i2] - verts[i0];
        let b = verts[i3] - verts[i1];
        if let Some(n) = Vector::normalize_vec(Point::cross_point(&a, &b)) {
            *out_n = n;
            true
        } else {
            *out_n = Vector::ZERO_VECTOR;
            false
        }
    }
```
```rust
    /// face plane equation 계산 (법선이 0이면 None)
    pub fn get_plane_equation(face: &MeshFace, verts: &[Point]) -> Option<PlaneEquation> {
        let mut n = Vector::ZERO_VECTOR;
        if !Self::compute_face_normal_from_dv(face, verts, &mut n) {
            return None;
        }
        PlaneEquation::create(verts[face.vi[0] as usize], n)
    }
```
```rust
    pub fn is_planar(
        face: &MeshFace,
        planar_tolerance: f64,
        angle_tolerance_radians: f64,
        verts: &[Point],
    ) -> (bool, Option<PlaneEquation>) {
        let e = match Self::get_plane_equation(face, verts) {
            Some(pe) => pe,
            None => return (false, Some(PlaneEquation::UNSET)),
        };
        // 삼각형은 바로 OK
        if face.is_triangle() {
            return (true, Some(e));
        }

        // 1) 평면 편차
        if planar_tolerance >= 0.0 {
            // vi[0..3] 점들 plane 값의 max-min
            let mut hmin = 0.0;
            let mut hmax = 0.0;
            let base = e.value_at(verts[face.vi[0] as usize]);
            for k in 1..=3 {
                let h = e.value_at(verts[face.vi[k] as usize]);
                let d = h - base; // base 기준
                if d < hmin {
                    hmin = d;
                }
                if d > hmax {
                    hmax = d;
                }
                if (hmax - hmin) > planar_tolerance {
                    return (false, Some(e));
                }
            }
        }

        // 2) 코너 노멀(quad일 때) 각도 검사
        if angle_tolerance_radians >= 0.0 && !face.is_triangle() {
            // Corner normals: (vi[i]->vi[i+1], diagonal)
            // C++과 동일 개념으로 계산 (Unset 처리 없이 단순 normalize/cross)
            let idx = |k: usize| face.vi[k % 4] as usize;
            let p = |k: usize| verts[idx(k)];
            let mut cn = [Vector::ZERO_VECTOR; 4];

            // 대각 C = v0-v3
            let c = p(0) - p(3);
            let c_unit;
            if let Some(u) = Vector::normalize_pt(c) {
                c_unit = u;
            } else {
                c_unit = Vector::UNSET_VECTOR;
            }

            for i in 0..4 {
                let a = if i == 3 {
                    c_unit
                } else {
                    let e = p(i + 1) - p(i);
                    if let Some(u) = Vector::normalize_pt(e) {
                        u
                    } else {
                        Vector::UNSET_VECTOR
                    }
                };
                let b = if i == 3 {
                    c_unit
                } else {
                    let e = p(i + 1) - p(i);
                    if let Some(u) = Vector::normalize_vec(e.to_vector()) {
                        u
                    } else {
                        Vector::UNSET_VECTOR
                    }
                };
                let n = Vector::cross_vec(&a, &b);
                cn[i] = if let Some(nn) = Vector::normalize_vec(n) {
                    nn
                } else {
                    Vector::UNSET_VECTOR
                };
            }

            let cos_tol = if angle_tolerance_radians < std::f64::consts::PI {
                angle_tolerance_radians.cos()
            } else {
                -1.0
            };

            // 반대 코너 쌍 (0↔2, 1↔3)
            let dot02 = cn[0].x * cn[2].x + cn[0].y * cn[2].y + cn[0].z * cn[2].z;
            let dot13 = cn[1].x * cn[3].x + cn[1].y * cn[3].y + cn[1].z * cn[3].z;
            if dot02 < cos_tol || dot13 < cos_tol {
                return (false, Some(e));
            }
        }
        (true, Some(e))
    }
}
```
```rust
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Point>,
    pub faces: Vec<[u32; 4]>, // triangle: [v0,v1,v2,v2], quad: [v0,v1,v2,v3]
    pub normals: Option<Vec<Vector>>,
}
```
```rust
impl Mesh {
    pub fn triangle_count(&self) -> usize {
        self.faces.len()
    }
}
```
```rust
impl Mesh {
    pub fn default() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            normals: None,
        }
    }
}
```
```rust
impl Mesh {
    pub fn new(vertices: Vec<Point>, faces: Vec<[u32; 4]>) -> Self {
        Self {
            vertices,
            faces,
            normals: None,
        }
    }
```
```rust
    pub fn compute_normals(&mut self) {
        let normals = self.normals.get_or_insert_with(|| vec![]);
        normals.clear();
        normals.resize(self.faces.len(), Vector::new(0.0, 0.0, 0.0));

        for (i, face) in self.faces.iter().enumerate() {
            let v0 = self.vertices[face[0] as usize];
            let v1 = self.vertices[face[1] as usize];
            let v2 = self.vertices[face[2] as usize];

            let edge1 = (v1 - v0).to_vector();
            let edge2 = (v2 - v0).to_vector();
            let normal = edge1.cross(&edge2).unitize();

            normals[i] = normal;
        }
    }
```
```rust
    pub fn optimize_mesh(mesh: &mut Mesh) {
        use std::collections::HashMap;
        let mut unique_map: HashMap<[NotNan<f32>; 3], i32> = HashMap::new();
        let mut new_vertices = Vec::new();
        let mut remap = Vec::new();

        for (_i, v) in mesh.vertices.iter().enumerate() {
            let key: [NotNan<f32>; 3] = [
                NotNan::new(v.x as f32).unwrap(),
                NotNan::new(v.y as f32).unwrap(),
                NotNan::new(v.z as f32).unwrap(),
            ];

            let id = *unique_map.entry(key).or_insert_with(|| {
                let new_id = new_vertices.len() as i32;
                new_vertices.push(Point::new(
                    key[0].into_inner() as f64,
                    key[1].into_inner() as f64,
                    key[2].into_inner() as f64,
                ));
                new_id
            });
            remap.push(id);
        }

        for faces in &mut mesh.faces {
            for vi in &mut *faces {
                *vi = remap[*vi as usize] as u32;
            }
        }

        mesh.vertices = new_vertices;
    }
```
```rust
    pub fn filter_planar_faces(mesh: &Mesh, planar_tol: f64, angle_tol_rad: f64) -> Vec<usize> {
        let mut planar_faces = Vec::new();

        for (i, face) in mesh.faces.iter().enumerate() {
            let mesh_face = if face[2] == face[3] {
                MeshFace::new_tri(face[0] as i32, face[1] as i32, face[2] as i32)
            } else {
                MeshFace::new_quad(
                    face[0] as i32,
                    face[1] as i32,
                    face[2] as i32,
                    face[3] as i32,
                )
            };

            let (is_planar, _) =
                MeshFace::is_planar(&mesh_face, planar_tol, angle_tol_rad, &mesh.vertices);
            if is_planar {
                planar_faces.push(i);
            }
        }
        planar_faces
    }
}
```
```rust
pub fn on_face_is_tri(f: &[u32; 4]) -> bool {
    f[2] == f[3]
}
```
```rust
pub fn on_mesh_to_tri_list(mesh: &Mesh) -> (Vec<Point>, Vec<[u32; 3]>) {
    let mut tris: Vec<[u32; 3]> = Vec::with_capacity(mesh.faces.len() * 2);
    for f in &mesh.faces {
        if on_face_is_tri(f) {
            tris.push([f[0], f[1], f[2]]);
        } else {
            // quad -> two tris (0,1,2) (0,2,3)
            tris.push([f[0], f[1], f[2]]);
            tris.push([f[0], f[2], f[3]]);
        }
    }
    (mesh.vertices.clone(), tris)
}
```
```rust
pub fn on_tri_list_to_mesh(vertices: Vec<Point>, tris: Vec<[u32; 3]>) -> Mesh {
    let mut faces = Vec::<[u32; 4]>::with_capacity(tris.len());
    for t in tris {
        faces.push([t[0], t[1], t[2], t[2]]);
    }
    Mesh::new(vertices, faces)
}
```
```rust
pub fn on_quad_list_to_mesh(vertices: Vec<Point>, quads: Vec<[u32; 4]>) -> Mesh {
    Mesh::new(vertices, quads)
}
```
```rust
pub fn on_force_triangulate(mesh: &mut Mesh) {
    let mut out = Vec::<[u32; 4]>::with_capacity(mesh.faces.len() * 2);
    for f in &mesh.faces {
        if on_face_is_tri(f) {
            out.push(*f);
        } else {
            out.push([f[0], f[1], f[2], f[2]]);
            out.push([f[0], f[2], f[3], f[3]]);
        }
    }
    mesh.faces = out;
}
```
```rust
pub fn on_weld_vertices(mesh: &mut Mesh, eps: f64) {
    let q = |x: f64| (x / eps).round() as i64;
    let mut map: HashMap<(i64, i64, i64), u32> = HashMap::new();
    let mut new_verts: Vec<Point> = Vec::with_capacity(mesh.vertices.len());
    let mut remap: Vec<u32> = vec![u32::MAX; mesh.vertices.len()];

    for (i, p) in mesh.vertices.iter().enumerate() {
        let key = (q(p.x), q(p.y), q(p.z));
        let id = *map.entry(key).or_insert_with(|| {
            let nid = new_verts.len() as u32;
            new_verts.push(*p);
            nid
        });
        remap[i] = id;
    }

    for f in &mut mesh.faces {
        f[0] = remap[f[0] as usize];
        f[1] = remap[f[1] as usize];
        f[2] = remap[f[2] as usize];
        f[3] = remap[f[3] as usize];
    }
    mesh.vertices = new_verts;
}
```
```rust
/// 사용되지 않는 정점 제거 후 인덱스 리매핑
pub fn on_compact_vertices(mesh: &mut Mesh) {
    let mut used = vec![false; mesh.vertices.len()];
    for f in &mesh.faces {
        used[f[0] as usize] = true;
        used[f[1] as usize] = true;
        used[f[2] as usize] = true;
        used[f[3] as usize] = true;
    }
    let mut remap = vec![u32::MAX; mesh.vertices.len()];
    let mut new_v = Vec::<Point>::with_capacity(mesh.vertices.len());
    for (i, p) in mesh.vertices.iter().enumerate() {
        if used[i] {
            remap[i] = new_v.len() as u32;
            new_v.push(*p);
        }
    }
    for f in &mut mesh.faces {
        f[0] = remap[f[0] as usize];
        f[1] = remap[f[1] as usize];
        f[2] = remap[f[2] as usize];
        f[3] = remap[f[3] as usize];
    }
    mesh.vertices = new_v;
}
```
```rust
pub fn on_apply_transform(mesh: &mut Mesh, xf: &Transform) {
    for p in &mut mesh.vertices {
        let q = xf.apply_point(*p);
        *p = q;
    }
    if let Some(normals) = &mut mesh.normals {
        // 회전/스케일 포함 시 법선 재계산을 권장
        normals.clear();
    }
}
```
```rust
pub fn on_extract_planar_sub_mesh(src: &Mesh, planar_tol: f64, angle_tol_rad: f64) -> Mesh {
    let idxes = Mesh::filter_planar_faces(src, planar_tol, angle_tol_rad);
    let mut faces = Vec::with_capacity(idxes.len());
    for i in idxes {
        faces.push(src.faces[i]);
    }
    // 정점은 우선 모두 보존 → 추후 compact_vertices 권장
    let mut m = Mesh::new(src.vertices.clone(), faces);
    on_compact_vertices(&mut m);
    m
}
```
```rust
/// 중복 페이스(같은 정점 조합) 제거
pub fn on_dedup_faces(mesh: &mut Mesh) {
    let mut seen: HashSet<[u32; 4]> = HashSet::new();
    mesh.faces.retain(|f| seen.insert(*f));
}
```
```rust
pub fn extract_contact_faces(mesh: &Mesh) -> Vec<(usize, usize)> {
    let mut vertex_to_faces: HashMap<i32, Vec<usize>> = HashMap::new();

    for (face_idx, face) in mesh.faces.iter().enumerate() {
        for &vi in face.iter() {
            vertex_to_faces.entry(vi as i32).or_default().push(face_idx);
        }
    }

    let mut contact_pairs = Vec::new();
    let mut seen = HashMap::new();

    for faces in vertex_to_faces.values() {
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                let a = faces[i];
                let b = faces[j];
                let key = if a < b { (a, b) } else { (b, a) };
                if seen.insert(key, true).is_none() {
                    contact_pairs.push(key);
                }
            }
        }
    }
    contact_pairs
}
```
```rust
#[derive(Debug, Clone)]
pub struct TopoVertex {
    pub tope_indices: Vec<usize>,
    pub mesh_vertices: Vec<usize>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct TopoEdge {
    pub topv: [usize; 2],
    pub topf_indices: Vec<usize>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct TopoFace {
    pub tope: [usize; 4],
    pub reve: [bool; 4],
}
```
```rust
#[derive(Debug, Clone)]
pub struct Topology {
    pub mesh_vertex_count: usize,
    pub mesh_face_count: usize,
    pub topv_map: Vec<usize>,
    pub topv: Vec<TopoVertex>,
    pub tope: Vec<TopoEdge>,
    pub topf: Vec<TopoFace>,
    pub eps: f64,
    pub keymap: HashMap<(i64, i64, i64), usize>,
    pub edge_map: HashMap<(usize, usize), usize>,
}
```
```rust
impl Topology {
    pub fn compact(&mut self, mesh: &Mesh) {
        use std::collections::HashMap;

        // 1. 유효한 face, edge, vertex 수집
        let valid_face: Vec<_> = self
            .topf
            .iter()
            .enumerate()
            .filter(|(_, f)| f.tope.iter().all(|&ei| ei < self.tope.len()))
            .map(|(i, _)| i)
            .collect();

        let mut used_edges = HashSet::new();
        for &fi in &valid_face {
            for &ei in &self.topf[fi].tope {
                used_edges.insert(ei);
            }
        }

        let mut used_vertices = HashSet::new();
        for &ei in &used_edges {
            let e = &self.tope[ei];
            used_vertices.insert(e.topv[0]);
            used_vertices.insert(e.topv[1]);
        }

        // 2. 새 인덱스 맵 생성
        let edge_remap: HashMap<_, _> = used_edges.iter().enumerate().map(|(i, &ei)| (ei, i)).collect();
        let vertex_remap: HashMap<_, _> = used_vertices.iter().enumerate().map(|(i, &vi)| (vi, i)).collect();
        let face_remap: HashMap<_, _> = valid_face.iter().enumerate().map(|(i, &fi)| (fi, i)).collect();


        // 3. 구조 재작성
        let mut new_topv = vec![
            TopoVertex {
                tope_indices: vec![],
                mesh_vertices: vec![]
            };
            used_vertices.len()
        ];
        for (&old_vi, &new_vi) in &vertex_remap {
            new_topv[new_vi].mesh_vertices = self.topv[old_vi].mesh_vertices.clone();
        }

        let mut new_tope = vec![
            TopoEdge {
                topv: [0, 0],
                topf_indices: vec![]
            };
            used_edges.len()
        ];
        for (&old_ei, &new_ei) in &edge_remap {
            let e = &self.tope[old_ei];
            new_tope[new_ei].topv = [vertex_remap[&e.topv[0]], vertex_remap[&e.topv[1]]];
            new_tope[new_ei].topf_indices = e
                .topf_indices
                .iter()
                .filter_map(|fi| face_remap.get(fi).copied())
                .collect();
        }

        let mut new_topf = vec![
            TopoFace {
                tope: [0; 4],
                reve: [false; 4]
            };
            valid_face.len()
        ];
        for (&old_fi, &new_fi) in &face_remap {
            let f = &self.topf[old_fi];
            new_topf[new_fi].tope = f.tope.map(|ei| edge_remap[&ei]);
            new_topf[new_fi].reve = f.reve;
        }

        // 4. topv_map 재작성
        let mut new_topv_map = vec![usize::MAX; self.mesh_vertex_count];
        for (new_vi, v) in new_topv.iter().enumerate() {
            for &mvi in &v.mesh_vertices {
                new_topv_map[mvi] = new_vi;
            }
        }

        // 5. keymap, edge_map 재생성
        let mut new_keymap = HashMap::new();
        let q = |x: f64| (x / self.eps).round() as i64;
        for (new_vi, v) in new_topv.iter().enumerate() {
            if let Some(&mvi) = v.mesh_vertices.first() {
                let p = mesh.vertices[mvi];
                new_keymap.insert((q(p.x), q(p.y), q(p.z)), new_vi);
            }
        }


        let mut new_edge_map = HashMap::new();
        for (ei, e) in new_tope.iter().enumerate() {
            let key = if e.topv[0] <= e.topv[1] {
                (e.topv[0], e.topv[1])
            } else {
                (e.topv[1], e.topv[0])
            };
            new_edge_map.insert(key, ei);
        }

        // 6. 구조 교체
        self.topv = new_topv;
        self.tope = new_tope;
        self.topf = new_topf;
        self.topv_map = new_topv_map;
        self.keymap = new_keymap;
        self.edge_map = new_edge_map;
        self.mesh_face_count = self.topf.len();
        self.mesh_vertex_count = self.topv_map.len();
    }
}
```
```rust
impl Topology {
    pub fn from_mesh(mesh: &Mesh, eps: f64) -> Self {
        use std::collections::HashMap;

        let n_v = mesh.vertices.len();
        let n_f = mesh.faces.len();
        let q = |x: f64| (x / eps).round() as i64;


        let mut keymap: HashMap<(i64, i64, i64), usize> = HashMap::new();
        let mut topv: Vec<TopoVertex> = Vec::new();
        let mut topv_map = vec![usize::MAX; n_v];

        for (vi, p) in mesh.vertices.iter().enumerate() {
            let key = (q(p.x), q(p.y), q(p.z));
            let topvi = *keymap.entry(key).or_insert_with(|| {
                let idx = topv.len();
                topv.push(TopoVertex {
                    tope_indices: vec![],
                    mesh_vertices: vec![],
                });
                idx
            });
            topv[topvi].mesh_vertices.push(vi);
            topv_map[vi] = topvi;
        }

        let mut edge_map = HashMap::new();
        let mut tope = Vec::new();
        let mut topf = Vec::new();

        for (fi, f) in mesh.faces.iter().enumerate() {
            let fv = [
                topv_map[f[0] as usize],
                topv_map[f[1] as usize],
                topv_map[f[2] as usize],
                topv_map[f[3] as usize],
            ];
            let is_tri = f[2] == f[3];
            let ring = if is_tri {
                vec![(fv[2], fv[0]), (fv[0], fv[1]), (fv[1], fv[2])]
            } else {
                vec![(fv[3], fv[0]), (fv[0], fv[1]), (fv[1], fv[2]), (fv[2], fv[3])]
            };

            let mut eidx = [usize::MAX; 4];
            let mut reve = [false; 4];

            for (k, &(a, b)) in ring.iter().enumerate() {
                let (u, v, rev) = if a <= b { (a, b, false) } else { (b, a, true) };
                let key = (u, v);
                let ei = *edge_map.entry(key).or_insert_with(|| {
                    let idx = tope.len();
                    tope.push(TopoEdge {
                        topv: [u, v],
                        topf_indices: vec![],
                    });
                    idx
                });
                tope[ei].topf_indices.push(fi);
                eidx[k] = ei;
                reve[k] = rev;
            }

            if is_tri {
                eidx[3] = eidx[2];
                reve[3] = reve[2];
            }

            topf.push(TopoFace { tope: eidx, reve });
        }

        let mut incident = vec![HashSet::new(); topv.len()];
        for (ei, e) in tope.iter().enumerate() {
            incident[e.topv[0]].insert(ei);
            incident[e.topv[1]].insert(ei);
        }
        for (i, v) in topv.iter_mut().enumerate() {
            v.tope_indices = incident[i].iter().copied().collect();
        }

        Topology {
            mesh_vertex_count: n_v,
            mesh_face_count: n_f,
            topv_map,
            topv,
            tope,
            topf,
            eps,
            keymap,
            edge_map,
        }
    }
}
```
```rust
fn get_or_create_edge(
    a: usize,
    b: usize,
    tope: &mut Vec<TopoEdge>,
    edge_map: &mut HashMap<(usize, usize), usize>,
    fi: usize,
) -> (usize, bool) {
    let (u, v, rev) = if a <= b { (a, b, false) } else { (b, a, true) };
    let key = (u, v);
    let ei = *edge_map.entry(key).or_insert_with(|| {
        let idx = tope.len();
        tope.push(TopoEdge {
            topv: [u, v],
            topf_indices: vec![],
        });
        idx
    });
    tope[ei].topf_indices.push(fi);
    (ei, rev)
}
```
```rust
impl Topology {
    pub fn add_vertex(&mut self, mesh: &mut Mesh, p: Point) -> usize {
        let vi = mesh.vertices.len();
        mesh.vertices.push(p);
        self.mesh_vertex_count += 1;
        self.topv_map.push(usize::MAX);

        let q = |x: f64| (x / self.eps).round() as i64;
        let key = (q(p.x), q(p.y), q(p.z));

        let topvi = if let Some(&tvi) = self.keymap.get(&key) {
            self.topv[tvi].mesh_vertices.push(vi);
            tvi
        } else {
            let tvi = self.topv.len();
            self.topv.push(TopoVertex {
                tope_indices: vec![],
                mesh_vertices: vec![vi],
            });
            self.keymap.insert(key, tvi);
            tvi
        };

        self.topv_map[vi] = topvi;
        vi
    }
}
```
```rust
impl Topology {
    pub fn add_face(&mut self, mesh: &mut Mesh, f: [u32; 4]) -> usize {
        let fi = mesh.faces.len();
        mesh.faces.push(f);
        self.mesh_face_count += 1;

        let fv = [
            self.topv_map[f[0] as usize],
            self.topv_map[f[1] as usize],
            self.topv_map[f[2] as usize],
            self.topv_map[f[3] as usize],
        ];
        let is_tri = f[2] == f[3];
        let ring = if is_tri {
            vec![(fv[2], fv[0]), (fv[0], fv[1]), (fv[1], fv[2])]
        } else {
            vec![
                (fv[3], fv[0]),
                (fv[0], fv[1]),
                (fv[1], fv[2]),
                (fv[2], fv[3]),
            ]
        };

        let mut eidx = [usize::MAX; 4];
        let mut reve = [false; 4];

        for (k, (a, b)) in ring.iter().enumerate() {
            let key = if *a <= *b { (*a, *b) } else { (*b, *a) };
            let (ei, revflag) = if let Some(&ei) = self.edge_map.get(&key) {
                self.tope[ei].topf_indices.push(fi);
                (ei, a > b)
            } else {
                let ei = self.tope.len();
                self.tope.push(TopoEdge {
                    topv: [key.0, key.1],
                    topf_indices: vec![fi],
                });
                self.edge_map.insert(key, ei);
                self.topv[key.0].tope_indices.push(ei);
                self.topv[key.1].tope_indices.push(ei);
                (ei, a > b)
            };
            eidx[k] = ei;
            reve[k] = revflag;
        }

        if is_tri {
            eidx[3] = eidx[2];
            reve[3] = reve[2];
        }

        self.topf.push(TopoFace { tope: eidx, reve });
        fi
    }
}
```
```rust
impl Topology {
    /// 모든 경계 루프 추출 (복수 가능)
    pub fn all_boundary_loops(&self) -> Vec<Vec<usize>> {
        // 각 topv에 연결된 경계 엣지 목록
        let mut bedges_at: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (ei, e) in self.tope.iter().enumerate() {
            if e.topf_indices.len() == 1 {
                bedges_at.entry(e.topv[0]).or_default().push(ei);
                bedges_at.entry(e.topv[1]).or_default().push(ei);
            }
        }

        let mut visited_e = HashSet::new();
        let mut loops = Vec::new();

        for (&start_v, e_list) in &bedges_at {
            for &e0 in e_list {
                if visited_e.contains(&e0) {
                    continue;
                }

                let mut loop_v = vec![start_v];
                let mut cur_v = start_v;
                let mut prev_e = None;

                loop {
                    let next_e = bedges_at
                        .get(&cur_v)
                        .and_then(|vv| {
                            vv.iter()
                                .find(|&&ei| !visited_e.contains(&ei) && Some(ei) != prev_e)
                        })
                        .copied();

                    if let Some(ei) = next_e {
                        visited_e.insert(ei);
                        let e = &self.tope[ei];
                        let nxt_v = if e.topv[0] == cur_v {
                            e.topv[1]
                        } else {
                            e.topv[0]
                        };

                        if nxt_v == loop_v[0] {
                            break; // 닫힌 루프
                        }

                        loop_v.push(nxt_v);
                        prev_e = Some(ei);
                        cur_v = nxt_v;
                    } else {
                        break; // 열린 경계 (비정상)
                    }
                }

                if loop_v.len() > 1 {
                    loops.push(loop_v);
                }
            }
        }

        loops
    }
}
```
```rust
pub fn on_merge_meshes(mesh1: &Mesh, mesh2: &Mesh, eps: f64) -> Mesh {
    let mut merged = Mesh::new(vec![], vec![]);
    let mut topo = Topology::from_mesh(&merged, eps);

    // 1. mesh1 정점 복사
    for p in &mesh1.vertices {
        topo.add_vertex(&mut merged, *p);
    }

    // 2. mesh1 면 복사
    for f in &mesh1.faces {
        topo.add_face(&mut merged, *f);
    }

    // 3. mesh2 정점 병합
    for p in &mesh2.vertices {
        topo.add_vertex(&mut merged, *p); // 동일 위치면 병합됨
    }

    // 4. mesh2 면 병합
    for f in &mesh2.faces {
        topo.add_face(&mut merged, *f);
    }

    // 5. 경계 루프 분석 (선택적)
    let loops = topo.all_boundary_loops();
    println!("Merged boundary loops: {}", loops.len());

    // 6. 압축
    topo.compact(&merged);

    merged
}
```
```rust
impl Topology {
    pub fn to_mesh(&self, original_mesh: &Mesh) -> Mesh {
        let mut vertices = Vec::new();
        let mut vertex_remap = vec![usize::MAX; self.mesh_vertex_count];

        // 1. top_v → 대표 mesh vertex 선택
        for (_top_vi, top_v) in self.topv.iter().enumerate() {
            if let Some(&vi) = top_v.mesh_vertices.first() {
                let p = original_mesh.vertices[vi];
                vertex_remap[vi] = vertices.len();
                vertices.push(p);
            }
        }

        // 2. face 재구성
        let mut faces = Vec::new();
        for f in &self.topf {
            let mut fv = [0u32; 4];
            for i in 0..4 {
                let ei = f.tope[i];
                let e = &self.tope[ei];
                let vi = e.topv[if f.reve[i] { 1 } else { 0 }];
                let mesh_vi = self.topv[vi].mesh_vertices[0];
                fv[i] = vertex_remap[mesh_vi] as u32;
            }
            faces.push(fv);
        }

        Mesh::new(vertices, faces)
    }
}
```
```rust
impl Topology {
    /// 메시가 watertight 상태인지 점검
    pub fn is_watertight(&self) -> bool {
        let mut boundary_count = 0;
        let mut nonmanifold_count = 0;

        for e in &self.tope {
            match e.topf_indices.len() {
                2 => {} // 정상
                1 => boundary_count += 1,
                0 => boundary_count += 1, // 비정상이지만 경계 취급
                _ => nonmanifold_count += 1,
            }
        }

        boundary_count == 0 && nonmanifold_count == 0
    }
}
```
```rust
#[inline]
fn push_tri4(faces: &mut Vec<[u32; 4]>, a: u32, b: u32, c: u32) {
    faces.push([a, b, c, c]);
}
```
```rust
// === orientation: C++ GetOrientationTransformation 과 동일 ===
fn get_orientation_transform(start: Point, dir: Vector) -> Transform {
    let _ = dir.unitize(); // 단위화
    let angle_in_xy = dir.y.atan2(dir.x);
    let angle_from_xy = dir.z.atan2((dir.x * dir.x + dir.y * dir.y).sqrt());

    // tran * rotZ * rotY(-angle_from_xy)  (C++와 동일한 순서)
    let tran = Transform::translation(start.x, start.y, start.z);
    let rotz = Transform::rotation_axis(
        angle_in_xy,
        Vector::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, 0.0),
    );
    let roty = Transform::rotation_axis(
        -angle_from_xy,
        Vector::new(0.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
    );
    tran.mul(&rotz).mul(&roty)
}
```
```rust
// === (헬퍼) C++ CreateSphereVertex 과 동일한 수식 ===
// _p1: 중심 원 반지름(스프링 중심 반지름), r: 선재(wire) 반지름
// a4, a5: 중심 원의 각도( cosθ, sinθ ) 에 해당
// c, s: 단면 원의 cosφ, sinφ
fn create_sphere_vertex(_p1: f64, r: f64, a4: f64, a5: f64, c: f64, s: f64) -> Point {
    let x = c * a4;
    let y = c * a5;
    Point::new(_p1 * a4 + x * r, _p1 * a5 + y * r, s * r)
}
```
```rust
pub fn create_arrow_geom(
    cyl_radius: f64,
    cyl_length: f64,
    cone_radius: f64,
    cone_length: f64,
    slices: usize,
) -> (Vec<Point>, Vec<[u32; 4]>) {
    assert!(slices >= 3);
    assert!(cyl_radius > 0.0 && cyl_length > 0.0);
    assert!(cone_radius > 0.0 && cone_length > 0.0);

    // 정점: 링 3개 + 바닥/뾰족점 2개
    let mut v = vec![Point::new(0.0, 0.0, 0.0); slices * 3 + 2];

    for i in 0..slices {
        let ang = 2.0 * PI * (i as f64) / (slices as f64);
        let y = ang.cos();
        let z = ang.sin();

        // 원통 시작 링 (x=0)
        v[i] = Point::new(0.0, y * cyl_radius, z * cyl_radius);
        // 원통 끝 링 (x=cyl_length)
        v[i + slices] = Point::new(cyl_length, y * cyl_radius, z * cyl_radius);
        // 원뿔 밑단 링 (x=cyl_length, 반지름 cone_radius)
        v[i + slices * 2] = Point::new(cyl_length, y * cone_radius, z * cone_radius);
    }
    // 원통 바닥 중심, 원뿔 꼭짓점
    v[slices * 3] = Point::new(0.0, 0.0, 0.0);
    v[slices * 3 + 1] = Point::new(cyl_length + cone_length, 0.0, 0.0);

    // 삼각형 수: (원통 측면 2*slices) + (원뿔 측면 2*slices) = 4*slices
    let mut f: Vec<[u32; 4]> = Vec::with_capacity(4 * slices);

    // 원통 측면
    for i in 0..slices {
        let i_next = if i + 1 >= slices { 0 } else { i + 1 };

        // (i at base ring) -> (i_next at base ring) -> (i_next at top ring)
        push_tri4(&mut f, i as u32, i_next as u32, (i_next + slices) as u32);
        // (i at base ring) -> (i_next at top ring) -> (i at top ring)
        push_tri4(
            &mut f,
            i as u32,
            (i_next + slices) as u32,
            (i + slices) as u32,
        );
    }

    // 원뿔 측면 (밑단 링: slices..2*slices-1, 끝점: slices*3+1)
    let tip = (slices * 3 + 1) as u32;
    for i in 0..slices {
        let i_next = if i + 1 >= slices { 0 } else { i + 1 };
        let a = (i + slices) as u32; // 원통 끝 링
        let b = (i_next + slices) as u32;
        let c = (i_next + slices * 2) as u32; // 원뿔 밑단 링
        let d = (i + slices * 2) as u32;

        push_tri4(&mut f, a, b, c);
        push_tri4(&mut f, a, c, d);
        // 원뿔 옆면(밑단 링 -> 꼭짓점)
        push_tri4(&mut f, d, c, tip);
    }

    // 원통 바닥 (삼각팬)
    let base_center = (slices * 3) as u32;
    for i in 0..slices {
        let i_next = if i + 1 >= slices { 0 } else { i + 1 };
        // 바닥은 시계/반시계 중 원하는 방향으로 (여기선 바닥에서 밖을 향해 보이는 방향으로)
        push_tri4(&mut f, base_center, i_next as u32, i as u32);
    }

    (v, f)
}
```
```rust
/// mesh 에 추가 (원점→+X 방향 기본)
pub fn create_arrow(
    mesh: &mut Mesh,
    cyl_radius: f64,
    cyl_length: f64,
    cone_radius: f64,
    cone_length: f64,
    slices: usize,
) {
    let (v, f) = create_arrow_geom(cyl_radius, cyl_length, cone_radius, cone_length, slices);
    let offset = mesh.vertices.len() as u32;
    mesh.vertices.extend(v.into_iter());
    for [a, b, c, _] in f {
        mesh.faces
            .push([offset + a, offset + b, offset + c, offset + c]);
    }
}
```
```rust
/// 시작점과 방향을 반영한 버전
pub fn create_arrow_oriented(
    mesh: &mut Mesh,
    start: Point,
    direction: Vector,
    cyl_radius: f64,
    cyl_length: f64,
    cone_radius: f64,
    cone_length: f64,
    slices: usize,
) {
    let (mut v, f) = create_arrow_geom(cyl_radius, cyl_length, cone_radius, cone_length, slices);
    let xf = get_orientation_transform(start, direction);
    for p in &mut v {
        *p = xf.apply_point(*p);
    }
    let offset = mesh.vertices.len() as u32;
    mesh.vertices.extend(v.into_iter());
    for [a, b, c, _] in f {
        mesh.faces
            .push([offset + a, offset + b, offset + c, offset + c]);
    }
}
```
```rust
// ======================================================
// Spring (원통 코일)
// ======================================================
fn spring_sub(
    count: f64,
    rings: usize,
    count1: usize,
    sides: usize,
    div_ang1: f64, // = 2π/rings (sign 포함, reverse 시 음수)
    pitch: f64,
    radius: f64,
    closed: bool,
    _reverse_twist: bool,
    verts: &mut Vec<Point>,
) {
    // C++ CreateSpringSub 의 동일 계산
    let val1 = pitch / (rings as f64);
    let mut z1 = 0.0;
    let mut ang2 = 0.0;

    let frac = count - count.floor(); // 남은 부분 (0..1)
    let val2 = if frac > f64::EPSILON {
        frac * val1
    } else {
        val1
    };
    let val3 = if frac > f64::EPSILON { frac } else { 1.0 };
    let num5 = val3 * div_ang1;

    // 헬릭스 기울기
    let sign = if div_ang1 >= 0.0 { 1.0 } else { -1.0 };
    let tilt = (pitch / (2.0 * PI * radius)).atan() * sign;

    // 초기 단면을 Z축 기준 tilt 회전
    let rot_tilt =
        Transform::rotation_axis(tilt, Vector::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0));
    for i in 0..sides {
        verts[i] = rot_tilt.apply_point(verts[i]);
    }

    // 중간 segment 반복 복제
    let mut write_idx = sides;
    for _i in 1..(count1 - 1) {
        ang2 += div_ang1;
        z1 += val1;
        let rot =
            Transform::rotation_axis(ang2, Vector::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0));
        let trans = Transform::translation(0.0, 0.0, z1);
        let xf = trans.mul(&rot);
        for j in 0..sides {
            verts[write_idx] = xf.apply_point(verts[j]);
            write_idx += 1;
        }
    }

    // 마지막 조각
    let z2 = z1 + val2;
    let num6 = ang2 + num5;
    let rot_last =
        Transform::rotation_axis(num6, Vector::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0));
    let trans_last = Transform::translation(0.0, 0.0, z2);
    let xf_last = trans_last.mul(&rot_last);
    for j in 0..sides {
        verts[write_idx] = xf_last.apply_point(verts[j]);
        write_idx += 1;
    }

    if closed {
        // 끝 원판(캡)용 보조 정점 2개 추가 (C++과 동일)
        let p_cap0 = create_sphere_vertex(radius, 0.0, 1.0, 0.0, 0.0, 1.0);
        let mut p_cap1 = create_sphere_vertex(radius, 0.0, num6.cos(), num6.sin(), 0.0, 1.0);
        p_cap1.z = z2;
        verts.push(p_cap0);
        verts.push(p_cap1);
    }

    // reverseTwist 면 삼각형 뒤집기는 연결 단계에서 처리
}
```
```rust
pub fn create_spring_geom(
    radius: f64,
    wire_radius: f64,
    sides: usize,
    rings: usize,
    pitch: f64,
    turns: f64,
    reverse_twist: bool,
    closed: bool,
) -> (Vec<Point>, Vec<[u32; 4]>) {
    assert!(wire_radius > 0.0 && radius > 0.0);
    assert!(sides >= 3 && rings >= 2);

    let count = (rings as f64) * turns;
    let count1 = count.ceil() as usize; // 세그먼트 수(정점 링 수)
    let count2 = count.floor() as usize;

    // 정점 수
    let mut v_cap = 0usize;
    if closed {
        v_cap = 2;
    }
    let total_vertices = count1 * sides + v_cap;
    let mut verts = vec![Point::new(0.0, 0.0, 0.0); total_vertices];

    // 링 각(헬릭스 중심)
    let sign = if reverse_twist { -1.0 } else { 1.0 };
    let div_ang1 = sign * 2.0 * PI / (rings as f64);

    // 단면 각
    let mut a2 = 0.0f64;
    let div_ang2 = 2.0 * PI / (sides as f64);

    // 선형(피치) 없는 경우엔 처음 count2개의 링을 채움, 있는 경우엔 첫 링만 채운 뒤 spring_sub에서 복제
    let first_loops = if pitch != 0.0 { 1usize } else { count2.max(1) };

    let mut write_idx = 0usize;
    for i in 0..first_loops {
        let a1 = (i as f64) * div_ang1;
        let (ca1, sa1) = (a1.cos(), a1.sin());
        for _k in 0..sides {
            let (c, s) = (a2.cos(), a2.sin());
            verts[write_idx] = create_sphere_vertex(radius, wire_radius, ca1, sa1, c, s);
            write_idx += 1;
            a2 += div_ang2;
        }
    }

    if pitch != 0.0 {
        // 남은 구간 복제/이동
        // 이미 첫 링(sides) 채웠으니 총 count1 링이 되도록 spring_sub 수행
        spring_sub(
            count,
            rings,
            count1,
            sides,
            div_ang1,
            pitch,
            radius,
            closed,
            reverse_twist,
            &mut verts,
        );
    } else {
        // pitch == 0 → 동일한 링을 차곡
        for i in first_loops..count1 {
            let a1 = (i as f64) * div_ang1;
            let (ca1, sa1) = (a1.cos(), a1.sin());
            a2 = 0.0;
            for _k in 0..sides {
                let (c, s) = (a2.cos(), a2.sin());
                verts[write_idx] = create_sphere_vertex(radius, wire_radius, ca1, sa1, c, s);
                write_idx += 1;
                a2 += div_ang2;
            }
        }
    }

    // 면 연결
    // 링 i와 i+1 사이를 사각 -> 삼각 2장
    let mut faces: Vec<[u32; 4]> = Vec::new();
    let ring_count = count1; // 실제 링 수
    let mut node = 0usize;
    for _i in 0..(ring_count - 1) {
        for k in 0..sides {
            let a = node + k;
            let b = node + ((k + 1) % sides);
            let c = node + sides + ((k + 1) % sides);
            let d = node + sides + k;

            if reverse_twist {
                push_tri4(&mut faces, a as u32, c as u32, b as u32);
                push_tri4(&mut faces, a as u32, d as u32, c as u32);
            } else {
                push_tri4(&mut faces, a as u32, b as u32, c as u32);
                push_tri4(&mut faces, a as u32, c as u32, d as u32);
            }
        }
        node += sides;
    }

    // pitch == 0 인 완전 닫힌 경우, 마지막 링과 첫 링을 연결
    if pitch == 0.0 {
        let base = (ring_count - 1) * sides;
        for k in 0..sides {
            let a = base + k;
            let b = base + ((k + 1) % sides);
            let c = (k + 1) % sides;
            let d = k;

            if reverse_twist {
                push_tri4(&mut faces, a as u32, c as u32, b as u32);
                push_tri4(&mut faces, a as u32, d as u32, c as u32);
            } else {
                push_tri4(&mut faces, a as u32, b as u32, c as u32);
                push_tri4(&mut faces, a as u32, c as u32, d as u32);
            }
        }
    }

    // 닫힌 스프링이면 앞/뒤 캡(삼각팬) 추가
    if closed {
        let cap0 = (ring_count * sides) as u32; // 앞
        let cap1 = cap0 + 1; // 뒤

        // 앞 캡: 첫 링 0..sides-1
        for k in 0..sides {
            let a = ((k + 1) % sides) as u32;
            let b = k as u32;
            push_tri4(&mut faces, cap0, a, b);
        }

        // 뒤 캡: 마지막 링 인덱스 범위
        let base = ((ring_count - 1) * sides) as u32;
        for k in 0..(sides - 1) {
            let a = base + k as u32;
            let b = base + (k as u32 + 1);
            push_tri4(&mut faces, a, b, cap1);
        }
        // 마지막 삼각형
        push_tri4(&mut faces, base + (sides as u32 - 1), base + 0, cap1);
    }

    (verts, faces)
}
```
```rust
pub fn create_spring(
    mesh: &mut Mesh,
    radius: f64,
    wire_radius: f64,
    sides: usize,
    rings: usize,
    pitch: f64,
    turns: f64,
    reverse_twist: bool,
    closed: bool,
) {
    let (v, f) = create_spring_geom(
        radius,
        wire_radius,
        sides,
        rings,
        pitch,
        turns,
        reverse_twist,
        closed,
    );
    let offset = mesh.vertices.len() as u32;
    mesh.vertices.extend(v.into_iter());
    for [a, b, c, _] in f {
        mesh.faces
            .push([offset + a, offset + b, offset + c, offset + c]);
    }
}
```
```rust
/// 헬릭스 파라미터 t (라디안) 에서 중심선 점과 접선 벡터
fn helix_point_tangent(r: f64, pitch: f64, t: f64) -> (Point, Vector) {
    let x = r * t.cos();
    let y = r * t.sin();
    let z = (pitch / (2.0 * std::f64::consts::PI)) * t;

    // d/dt [r cos t, r sin t, (pitch/2π) t] = [-r sin t, r cos t, pitch/2π]
    let tx = -r * t.sin();
    let ty = r * t.cos();
    let tz = pitch / (2.0 * std::f64::consts::PI);

    (Point::new(x, y, z), Vector::new(tx, ty, tz))
}
```
```rust
/// 스프링 끝(t = 2π*turns) 접선 방향으로 화살표 배치
pub fn attach_arrow_at_spring_end(
    mesh: &mut Mesh,
    r: f64,
    pitch: f64,
    turns: f64,
    cyl_r: f64,
    cyl_len: f64,
    cone_r: f64,
    cone_len: f64,
    slices: usize,
) {
    let t = 2.0 * std::f64::consts::PI * turns;
    let (p, tvec) = helix_point_tangent(r, pitch, t);
    // 스프링 단면(와이어 반지름)은 무시하고 중심선 접선으로 정렬
    create_arrow_oriented(mesh, p, tvec, cyl_r, cyl_len, cone_r, cone_len, slices);
}
```
```rust
#[inline]
fn on_unit_vec(v: Vector) -> Vector {
    if let Some(u) = Vector::normalize_vec(v) {
        u
    } else {
        Vector::ZERO_VECTOR
    }
}
```
```rust
#[inline]
fn on_unit_pt(v: Point) -> Vector {
    if let Some(u) = Vector::normalize_pt(v) {
        u
    } else {
        Vector::ZERO_VECTOR
    }
}
```
```rust
pub fn generate_revolved_surface_points(
    profile: &[Point],
    start_angle: f64,
    end_angle: f64,
    axis_dir: Vector,
    axis_origin: Point,
    angle_div_count: usize,
    profile_point_count: usize,
    total_angle_step_count: usize,
    _is_closed: bool, // 내부에선 사용 안 함(프로파일 개수로 처리)
) -> Vec<Point> {
    let dom = Interval::new(start_angle, end_angle);
    let angle_step = dom.length() / (angle_div_count as f64);

    // 각도 테이블
    let mut cos_t = vec![0.0; total_angle_step_count];
    let mut sin_t = vec![0.0; total_angle_step_count];
    let mut ang = dom.t0;
    for i in 0..total_angle_step_count {
        cos_t[i] = ang.cos();
        sin_t[i] = ang.sin();
        ang += angle_step;
    }

    let axis_u = on_unit_vec(axis_dir);
    let axis_line_p = axis_origin; // 축의 한 점
    let axis_line_u = axis_u; // 축 방향

    // 결과: 각도 스텝 x 프로파일 수
    let mut out = vec![Point::new(0.0, 0.0, 0.0); profile_point_count * total_angle_step_count];

    // 축에 대한 각 프로파일 점의 중심, 반경, 방사/직교 벡터
    for j in 0..profile_point_count {
        let sp = profile[j];

        // 축으로의 수선발: P_center = O + dot(SP-O, U)*U
        let op = sp - axis_line_p;
        let proj_len = op.x * axis_line_u.x + op.y * axis_line_u.y + op.z * axis_line_u.z;
        let center = axis_line_p + (axis_line_u * proj_len).to_point();

        let rvec = sp - center; // 반지름 벡터
        let r = rvec.length();
        let rv = if r > 0.0 {
            on_unit_pt(rvec)
        } else {
            Vector::ZERO_VECTOR
        };
        let ov = Vector::cross_vec(&axis_line_u, &rv); // rv와 축에 수직인 방향
        let ov = on_unit_vec(ov);

        for k in 0..total_angle_step_count {
            let idx = k * profile_point_count + j;
            // 회전: center + r*(cos θ * rv + sin θ * ov)
            let x = center.x + r * (cos_t[k] * rv.x + sin_t[k] * ov.x);
            let y = center.y + r * (cos_t[k] * rv.y + sin_t[k] * ov.y);
            let z = center.z + r * (cos_t[k] * rv.z + sin_t[k] * ov.z);
            out[idx] = Point::new(x, y, z);
        }
    }

    out
}
```
```rust
#[inline]
fn add_quad_as_two_tris(faces: &mut Vec<[u32; 4]>, a: usize, b: usize, c: usize, d: usize) {
    faces.push([a as u32, b as u32, c as u32, c as u32]);
    faces.push([c as u32, b as u32, d as u32, d as u32]);
}
```
```rust
pub fn generate_revolved_mesh_core(
    profile: &[Point],
    start_angle: f64,
    end_angle: f64,
    axis_dir: Vector,
    axis_origin: Point,
    angle_div_count: usize,
    is_profile_closed: bool,
) -> (Vec<Point>, Vec<[u32; 4]>) {
    let full_revolution = (end_angle - start_angle).abs() >= 2.0 * PI - f64::EPSILON;

    let profile_count = if is_profile_closed {
        // 마지막 중복점 제외
        profile.len().saturating_sub(1)
    } else {
        profile.len()
    };

    let angle_steps = if full_revolution {
        angle_div_count
    } else {
        angle_div_count + 1
    };

    let verts = generate_revolved_surface_points(
        profile,
        start_angle,
        end_angle,
        axis_dir,
        axis_origin,
        angle_div_count,
        profile_count,
        angle_steps,
        false,
    );

    // faces
    let mut faces: Vec<[u32; 4]> = Vec::with_capacity((profile.len() - 1) * angle_div_count * 2);

    let max_angle_idx = if full_revolution {
        angle_steps
    } else {
        angle_steps - 1
    };
    for ai in 0..max_angle_idx {
        let nai = (ai + 1) % angle_steps;
        for pi in 0..(profile.len() - 1) {
            let cp = pi % profile_count;
            let np = (cp + 1) % profile_count;

            // 그리드 사각형 (ai,cp) (ai,np) (nai,cp) (nai,np)
            let a = ai * profile_count + cp;
            let b = ai * profile_count + np;
            let c = nai * profile_count + cp;
            let d = nai * profile_count + np;
            add_quad_as_two_tris(&mut faces, a, b, c, d);
        }
    }

    (verts, faces)
}
```
```rust
pub fn create_revolved_mesh_to_mesh(
    profile: &[Point],
    start_angle: f64,
    rev_angle: f64,
    axis_dir: Vector,
    axis_origin: Point,
    angle_div_count: usize,
) -> Mesh {
    let itv = on_fix_rev_angle_interval_2pi(start_angle, rev_angle);
    let is_closed_profile = {
        if profile.len() >= 2 {
            // 첫/끝이 같으면 폐곡선으로 취급
            let p0 = profile[0];
            let p1 = profile[profile.len() - 1];
            (p0 - p1).length() < 1e-12
        } else {
            false
        }
    };

    let (v, f) = generate_revolved_mesh_core(
        profile,
        itv.t0,
        itv.t1,
        axis_dir,
        axis_origin,
        angle_div_count,
        is_closed_profile,
    );

    Mesh::new(v, f)
}
```
```rust
/// 토러스 생성 (풀 회전: 0..2π)
/// - major_radius: 중심 원 반경 R
/// - minor_radius: 단면 원 반경 r
/// - rings: 큰 원 방향 분할(θ)
/// - sides: 단면 원 분할(φ)
pub fn create_torus(major_radius: f64, minor_radius: f64, sides: usize, rings: usize) -> Mesh {
    assert!(major_radius > 0.0 && minor_radius > 0.0);
    assert!(sides >= 3 && rings >= 3);

    let mut verts = Vec::<Point>::with_capacity(sides * rings);
    // 파라미터: θ (메이저), φ(마이너)
    for i in 0..rings {
        let theta = 2.0 * PI * (i as f64) / (rings as f64);
        let ct = theta.cos();
        let st = theta.sin();
        for j in 0..sides {
            let phi = 2.0 * PI * (j as f64) / (sides as f64);
            let cp = phi.cos();
            let sp = phi.sin();

            let r = major_radius + minor_radius * cp;
            let x = r * ct;
            let y = r * st;
            let z = minor_radius * sp;
            verts.push(Point::new(x, y, z));
        }
    }

    let mut faces: Vec<[u32; 4]> = Vec::with_capacity(rings * sides * 2);
    let idx = |ri: isize, sj: isize| -> usize {
        let r = ((ri % rings as isize) + rings as isize) % rings as isize;
        let s = ((sj % sides as isize) + sides as isize) % sides as isize;
        (r as usize) * sides + (s as usize)
    };
    for i in 0..rings {
        for j in 0..sides {
            let a = idx(i as isize, j as isize);
            let b = idx(i as isize, j as isize + 1);
            let c = idx(i as isize + 1, j as isize);
            let d = idx(i as isize + 1, j as isize + 1);
            add_quad_as_two_tris(&mut faces, a, b, c, d);
        }
    }

    Mesh::new(verts, faces)
}
```
```rust
pub fn project_to_xy_plane(
    outer_loop_3d: &[Point],
    inner_loops_3d: &[Vec<Point>],
    source_plane: &Plane,
) -> (Vec<Point2>, Vec<Vec<Point2>>) {
    // 월드→평면 로컬 좌표: (P - O) · {xaxis, yaxis}
    fn to_plane_xy(p: Point, pl: &Plane) -> Point2 {
        let v = p - pl.origin;
        Point2::new(v.dot_vec(&pl.x_axis), v.dot_vec(&pl.y_axis))
    }

    let outer2d: Vec<Point2> = outer_loop_3d
        .iter()
        .copied()
        .map(|p| to_plane_xy(p, source_plane))
        .collect();

    if inner_loops_3d.is_empty() {
        return (outer2d, Vec::new());
    }
    let mut inner2d: Vec<Vec<Point2>> = Vec::with_capacity(inner_loops_3d.len());
    for ring in inner_loops_3d {
        inner2d.push(
            ring.iter()
                .copied()
                .map(|p| to_plane_xy(p, source_plane))
                .collect(),
        );
    }
    (outer2d, inner2d)
}

// -------------------------------------------
// Extrude (측면만 / 분할 / 캡 포함)
// -------------------------------------------
```
```rust
/// 삼각형 페이스 푸시 헬퍼
#[inline]
fn push_tri(tris: &mut Vec<[u32; 3]>, a: usize, b: usize, c: usize) {
    tris.push([a as u32, b as u32, c as u32]);
}
```
```rust
/// 사각형 두 삼각형 분해 (a,b; c,d)
#[inline]
fn push_quad_as_tris(tris: &mut Vec<[u32; 3]>, a: usize, b: usize, c: usize, d: usize) {
    push_tri(tris, a, b, c);
    push_tri(tris, b, d, c);
}
```
```rust
/// 폐루프 여부에 따라 유효한 정점 수(마지막=첫점 중복 제거)
#[inline]
fn loop_vertex_count(loop_pts: &[Point], is_closed: bool) -> usize {
    if is_closed {
        loop_pts.len().saturating_sub(1)
    } else {
        loop_pts.len()
    }
}
```
```rust
/// 측면만: baseLoop 와 baseLoop + extrusion 으로 띠 만들기
pub fn create_extruded_mesh_side(
    base_loop: &[Point],
    extrusion_vec: Vector,
    is_closed_loop: bool,
) -> (Vec<Point>, Vec<[u32; 3]>) {
    let n = loop_vertex_count(base_loop, is_closed_loop);
    if n < 2 {
        return (Vec::new(), Vec::new());
    }

    let mut verts = vec![Point::new(0.0, 0.0, 0.0); n * 2];
    for i in 0..n {
        verts[i] = base_loop[i];
        verts[i + n] = base_loop[i] + extrusion_vec.to_point();
    }

    let mut tris = Vec::<[u32; 3]>::with_capacity((base_loop.len() - 1) * 2);
    for i in 0..(base_loop.len() - 1) {
        let v0 = i % n;
        let v1 = (v0 + 1) % n;
        // ON_AddTriangle(triangles, v0, v1, 1, 0, n, idx)
        // → (a,b,c) = (v0@layer1, v1@layer1, v0@layer0)와 동일에 맞춰서
        // 원본 코드(ON_AddTriangle)가 (a,b,c,d) 패턴으로 두 개를 푸시하는데,
        // 여기서는 quad(v0,v1,v0+n,v1+n)을 두 삼각형으로
        push_quad_as_tris(&mut tris, v0, v1, v0 + n, v1 + n);
    }
    (verts, tris)
}
```
```rust
/// 분할 Extrude: 방향/높이/분할수로 여러 링 생성 (측면만)
pub fn create_extruded_mesh_side_divided(
    base_loop: &[Point],
    direction: Vector,
    height: f64,
    divisions: usize,
    is_closed_loop: bool,
) -> (Vec<Point>, Vec<[u32; 3]>) {
    let n = loop_vertex_count(base_loop, is_closed_loop);
    if n < 2 || divisions < 1 {
        return (Vec::new(), Vec::new());
    }

    let mut step = direction;
    let len = step.length();
    if len > 0.0 {
        step = step / len * (height / divisions as f64);
    }

    // (divisions+1)개의 링
    let mut verts = vec![Point::new(0.0, 0.0, 0.0); n * (divisions + 1)];
    for layer in 0..=divisions {
        let off = step * (layer as f64);
        for i in 0..n {
            verts[layer * n + i] = base_loop[i] + off.to_point();
        }
    }

    let mut tris = Vec::<[u32; 3]>::with_capacity(divisions * n * 2);
    for layer in 0..divisions {
        let cur = layer * n;
        let nxt = (layer + 1) * n;
        for i in 0..n {
            let v0 = cur + i;
            let v1 = cur + ((i + 1) % n);
            let v2 = nxt + i;
            let v3 = nxt + ((i + 1) % n);
            push_tri(&mut tris, v0, v1, v2);
            push_tri(&mut tris, v1, v3, v2);
        }
    }
    (verts, tris)
}
```
```rust
/// 캡 포함 Extrude: 위/아래 중심을 만들어 팬캡
pub fn create_extruded_mesh_with_caps(
    base_loop: &[Point],
    direction: Vector,
    height: f64,
    divisions: usize,
    is_closed_loop: bool,
) -> (Vec<Point>, Vec<[u32; 3]>) {
    let n = loop_vertex_count(base_loop, is_closed_loop);
    if n < 3 || divisions < 1 {
        return (Vec::new(), Vec::new());
    }

    let (mut verts, mut tris) =
        create_extruded_mesh_side_divided(base_loop, direction, height, divisions, is_closed_loop);

    // 아래/위 중심 추가
    let bottom_center_idx = verts.len();
    let mut bottom = Point::new(0.0, 0.0, 0.0);
    for i in 0..n {
        bottom = bottom + verts[i];
    }
    bottom = bottom / (n as f64);
    verts.push(bottom);

    let top_center_idx = verts.len();
    let mut top = Point::new(0.0, 0.0, 0.0);
    let top_start = n * divisions;
    for i in 0..n {
        top = top + verts[top_start + i];
    }
    top = top / (n as f64);
    verts.push(top);

    // 캡 삼각형: 아래
    for i in 0..n {
        let i0 = i;
        let i1 = (i + 1) % n;
        push_tri(&mut tris, bottom_center_idx, i1, i0);
    }
    // 위
    for i in 0..n {
        let i0 = top_start + i;
        let i1 = top_start + (i + 1) % n;
        push_tri(&mut tris, top_center_idx, i0, i1);
    }
    (verts, tris)
}
```
```rust
#[inline]
fn rot_z(p: Point, ang: f64) -> Point {
    let (c, s) = (ang.cos(), ang.sin());
    Point::new(c * p.x - s * p.y, s * p.x + c * p.y, p.z)
}
```
```rust
pub fn apply_twisted_transform_to_mesh_vertices(
    mesh: &mut Mesh,
    total_twist_turns: f64,
    angular_segments: usize,
    height_segments: usize,
    vertex_count_per_section: usize,
    twist_angle_per_segment: f64,
    height: f64,
    _radius: f64,
    apply_caps: bool,
    _is_clockwise: bool,
) {
    if vertex_count_per_section == 0 || height_segments < 2 {
        return;
    }

    let height_per_segment = height / angular_segments as f64;

    let frac = total_twist_turns - total_twist_turns.floor();
    let (adj_height_per_seg, height_fraction) = if frac > f64::EPSILON {
        (frac * height_per_segment, frac)
    } else {
        (height_per_segment, 1.0)
    };

    let total_twist_angle = height_fraction * twist_angle_per_segment;

    // section0(첫 링) 전체를 total_twist_angle 만큼 Z회전
    for i in 0..vertex_count_per_section {
        mesh.vertices[i] = rot_z(mesh.vertices[i], total_twist_angle);
    }

    // 중간 섹션 복제/회전/이동
    let mut current_height = 0.0;
    let mut current_twist = 0.0;
    let mut write_idx = vertex_count_per_section;

    for _seg in 1..(height_segments - 1) {
        current_twist += twist_angle_per_segment;
        current_height += height_per_segment;

        for i in 0..vertex_count_per_section {
            let p = rot_z(mesh.vertices[i], current_twist);
            mesh.vertices[write_idx] = Point::new(p.x, p.y, p.z + current_height);
            write_idx += 1;
        }
    }

    // 마지막(Frac 고려)
    let final_height = current_height + adj_height_per_seg;
    let final_twist = current_twist + total_twist_angle;
    for i in 0..vertex_count_per_section {
        let p = rot_z(mesh.vertices[i], final_twist);
        mesh.vertices[write_idx] = Point::new(p.x, p.y, p.z + final_height);
        write_idx += 1;
    }

    if apply_caps {
        // 캡 두 점은 뒤에서 별도 배치가 필요한 경우에만 사용하세요.
        // 여기서는 공간만 맞춰 줍니다(필요 없으면 삭제).
        if mesh.vertices.len() >= write_idx + 2 {
            mesh.vertices[write_idx] = Point::new(mesh.vertices[0].x, mesh.vertices[0].y, 0.0);
            mesh.vertices[write_idx + 1] =
                Point::new(mesh.vertices[0].x, mesh.vertices[0].y, final_height);
        }
    }
}

// -------------------------------------------
// Sweep (프레임/스윕)
// -------------------------------------------
```
```rust
#[derive(Clone, Debug)]
pub struct SweepPathFrame {
    pub position: Point,
    pub tangent: Vector,
    pub normal: Vector,
    pub binormal: Vector,
}
```
```rust
#[inline]
fn safe_unit(v: Vector) -> Vector {
    let l = v.length();
    if l > 0.0 {
        v / l
    } else {
        Vector::new(0.0, 0.0, 0.0)
    }
}
```
```rust
#[inline]
fn cross(a: Vector, b: Vector) -> Vector {
    a.cross(&b)
}
```
```rust
#[inline]
fn dot(a: Vector, b: Vector) -> f64 {
    a.dot(&b)
}
```
```rust
/// Frenet 프레임
pub fn on_sweep_compute_frenet_frames(
    path_points: &[Point],
    tangents: &[Vector],
) -> Vec<SweepPathFrame> {
    let n = path_points.len();
    if n == 0 || tangents.len() != n {
        return Vec::new();
    }

    let mut frames = Vec::with_capacity(n);
    // 초기 normal 후보
    let mut prev_n = Vector::new(0.0, 0.0, 1.0);
    if dot(tangents[0], prev_n).abs() > 0.9 {
        prev_n = Vector::new(0.0, 1.0, 0.0);
    }

    for i in 0..n {
        let t = safe_unit(tangents[i]);
        let b = safe_unit(cross(t, prev_n));
        let n = safe_unit(cross(b, t));
        frames.push(SweepPathFrame {
            position: path_points[i],
            tangent: t,
            normal: n,
            binormal: b,
        });
        prev_n = n;
    }
    frames
}
```
```rust
/// Rotation-minimizing frames (RMF; simple discrete version)
pub fn on_compute_rotation_minimizing_frames(

    points: &[Point],
    tangents: &[Vector],
) -> Vec<SweepPathFrame> {
    let n = points.len();
    if n < 2 || tangents.len() != n {
        return Vec::new();
    }
    let mut frames = vec![
        SweepPathFrame {
            position: points[0],
            tangent: safe_unit(tangents[0]),
            normal: Vector::new(0.0, 1.0, 0.0),
            binormal: Vector::new(0.0, 0.0, 1.0)
        };
        n
    ];

    // 초기 N,B
    let mut n0 = Vector::new(0.0, 1.0, 0.0);
    if dot(tangents[0], n0).abs() > 0.9 {
        n0 = Vector::new(1.0, 0.0, 0.0);
    }
    let t0 = safe_unit(tangents[0]);
    let b0 = safe_unit(cross(t0, n0));
    let n0 = safe_unit(cross(b0, t0));
    frames[0] = SweepPathFrame {
        position: points[0],
        tangent: t0,
        normal: n0,
        binormal: b0,
    };

    for i in 1..n {
        let ti = safe_unit(tangents[i]);
        let v = safe_unit((points[i] - points[i - 1]).to_vector()); // 이동 방향
        let ri = v;
        let mut si = frames[i - 1].normal - ri * dot(ri, frames[i - 1].normal);
        let mut ti_b = frames[i - 1].binormal - ri * dot(ri, frames[i - 1].binormal);
        si = safe_unit(si);
        ti_b = cross(ri, si);

        frames[i] = SweepPathFrame {
            position: points[i],
            tangent: ti,
            normal: si,
            binormal: ti_b,
        };
    }
    frames
}
```
```rust
/// from→to 회전(축/각)으로 벡터 회전
fn on_rotate_from_to(v: Vector, from: Vector, to: Vector) -> Vector {
    let f = safe_unit(from);
    let t = safe_unit(to);
    let c = dot(f, t);

    if c > 0.9999 {
        return v;
    }
    if c < -0.9999 {
        // 180도: f와 직교하는 임의 축
        let mut axis = cross(f, Vector::new(1.0, 0.0, 0.0));
        if axis.length() < 1e-6 {
            axis = cross(f, Vector::new(0.0, 1.0, 0.0));
        }
        let axis = safe_unit(axis);
        return on_rotate_axis_angle(v, axis, PI);
    }
    let axis = safe_unit(cross(f, t));
    let ang = c.acos();
    on_rotate_axis_angle(v, axis, ang)
}
```
```rust
/// 축-각 회전: Rodrigues
fn on_rotate_axis_angle(v: Vector, axis: Vector, ang: f64) -> Vector {
    let (c, s) = (ang.cos(), ang.sin());
    v * c + cross(axis, v) * s + axis * (dot(axis, v) * (1.0 - c))
}
```
```rust
/// 곡률 프레임(간단 버전)
pub fn on_sweep_compute_curvature_frames(
    path_points: &[Point],
    tangents: &[Vector],
    curvatures: &[Vector],
) -> Vec<SweepPathFrame> {
    let n = path_points.len();
    if n < 2 || tangents.len() != n || curvatures.len() != n {
        return Vec::new();
    }

    // 첫 N
    let mut n0 = curvatures[0];
    if n0.length() < 1e-6 {
        n0 = cross(tangents[0], Vector::new(0.0, 0.0, 1.0));
        if n0.length() < 1e-6 {
            n0 = cross(tangents[0], Vector::new(0.0, 1.0, 0.0));
        }
    }
    let mut t_prev = safe_unit(tangents[0]);
    let mut n_prev = safe_unit(n0);
    let b_prev = safe_unit(cross(t_prev, n_prev));
    n_prev = safe_unit(cross(b_prev, t_prev));

    let mut frames = Vec::with_capacity(n);
    frames.push(SweepPathFrame {
        position: path_points[0],
        tangent: t_prev,
        normal: n_prev,
        binormal: b_prev,
    });

    for i in 1..n {
        let t = safe_unit(tangents[i]);
        // t_prev → t 회전으로 N/B 갱신
        let n = on_rotate_from_to(n_prev, t_prev, t);
        let b = safe_unit(cross(t, n));
        let n = safe_unit(cross(b, t));
        frames.push(SweepPathFrame {
            position: path_points[i],
            tangent: t,
            normal: n,
            binormal: b,
        });
        t_prev = t;
        n_prev = n; // b_prev = b
    }
    frames
}
```
```rust
/// 2D 프로파일을 프레임 시퀀스에 따라 스윕 (Frenet/RMF 공용)
pub fn on_sweep_profile_along_path(
    profile2d: &[Point2],
    path_frames: &[SweepPathFrame],
) -> (Vec<Point>, Vec<[u32; 3]>) {
    let m = profile2d.len();
    let k = path_frames.len();
    if m == 0 || k == 0 {
        return (Vec::new(), Vec::new());
    }

    let mut verts = Vec::<Point>::with_capacity(m * k);
    for fr in path_frames {
        for q in profile2d {
            let p = fr.position + (fr.normal * q.x + fr.binormal * q.y).to_point();
            verts.push(p);
        }
    }

    let mut tris = Vec::<[u32; 3]>::with_capacity((k - 1) * m * 2);
    for i in 0..(k - 1) {
        let base0 = i * m;
        let base1 = (i + 1) * m;
        for j in 0..m {
            let next = (j + 1) % m;
            push_tri(&mut tris, base0 + j, base1 + j, base1 + next);
            push_tri(&mut tris, base0 + j, base1 + next, base0 + next);
        }
    }
    (verts, tris)
}
```
```rust
/// 원본 C++ `ClosestPtToEdge`와 동일 개념:
/// 점 P를 선분 AB에 사영해 bary(a,b)를 반환 (a*A + b*B, a+b=1)
pub fn on_closest_pt_to_edge_bary(p: &Point, a: &Point, b: &Point) -> (f64, f64) {
    // 표준 공식: t = ((P-A)·(B-A)) / |B-A|^2,  a=1-t, b=t
    let u = Point::sub_point(b, a);
    let denom = Point::dot_point(&u, &u);
    if denom <= 0.0 {
        return (1.0, 0.0); // A==B 퇴화
    }
    let pa = Point::sub_point(p, a);
    let t = Point::dot_point(&pa, &u) / denom;

    if t <= ON_ZERO_TOL {
        (1.0, 0.0)
    } else if t >= 1.0 - ON_ZERO_TOL {
        (0.0, 1.0)
    } else {
        (1.0 - t, t)
    }
}
```
```rust
/// 내부 헬퍼: 삼각형 R,S,T 평면 내 해(바리센트릭) 추정.
/// 실패(퇴화) 시 None.
fn on_closest_pt_to_triangle_helper(
    r: &Point,
    s: &Point,
    t_: &Point,
    q_in: &Point,
) -> Option<(f64, f64, f64)> {
    // 원본과 동일 변수명/흐름
    let v0 = Point::sub_point(r, t_);
    let v1 = Point::sub_point(s, t_);
    let q = Point::sub_point(q_in, t_);

    let mut a00 = Point::dot_point(&v0, &v0);
    if a00 <= 0.0 {
        return None;
    }
    a00 = 1.0 / a00;

    let mut a11 = Point::dot_point(&v1, &v1);
    if a11 <= 0.0 {
        return None;
    }
    a11 = 1.0 / a11;

    let mut a01 = Point::dot_point(&v0, &v1);
    let a10 = a01 * a11;
    a01 *= a00;

    let b0 = Point::dot_point(&v0, &q) * a00;
    let b1 = Point::dot_point(&v1, &q) * a11;

    let (ss, tt) = if a00 <= a11 {
        // tt 먼저
        let den = 1.0 - a01 * a10;
        if den == 0.0 {
            return None;
        }
        let tt = (b1 - a10 * b0) / den;
        let ss = b0 - a01 * tt;
        (ss, tt)
    } else {
        // ss 먼저
        let den = 1.0 - a01 * a10;
        if den == 0.0 {
            return None;
        }
        let ss = (b0 - a01 * b1) / den;
        let tt = b1 - a10 * ss;
        (ss, tt)
    };

    let uu = 1.0 - ss - tt;
    Some((ss, tt, uu))
}
```
```rust
/// 점 `input`의 삼각형 `tri[3]`에 대한 최단점 `output`과 bary(A,B,C) 반환
pub fn on_closest_pt_to_triangle(input: &Point, tri: &[Point; 3]) -> (Point, f64, f64, f64) {
    let a = tri[0];
    let b = tri[1];
    let c = tri[2];

    // 1) 평면 해 시도
    if let Some((mut bary_a, mut bary_b, mut bary_c)) =
        on_closest_pt_to_triangle_helper(&a, &b, &c, input)
    {
        // 아주 작은 음수/수치진동은 0으로
        if bary_a <= ON_ZERO_TOL {
            bary_a = 0.0;
        }
        if bary_b <= ON_ZERO_TOL {
            bary_b = 0.0;
        }
        if bary_c <= ON_ZERO_TOL {
            bary_c = 0.0;
        }

        // 한 변/꼭짓점으로 스냅되는 케이스: 에지 후보들 비교
        if bary_a == 0.0 || bary_b == 0.0 || bary_c == 0.0 {
            let mut a0 = -1.0;
            let mut b0 = -1.0;
            let mut c0 = -1.0;
            let mut a1 = -1.0;
            let mut b1 = -1.0;
            let mut c1 = -1.0;
            let mut has_second = false;

            if bary_a == 0.0 {
                if bary_b == 0.0 {
                    // A=0,B=0 → AC, BC 두 에지 후보
                    let (aa, cc) = on_closest_pt_to_edge_bary(input, &a, &c);
                    a0 = aa;
                    c0 = cc;
                    b0 = 1.0 - a0 - c0;

                    let (bb, cc2) = on_closest_pt_to_edge_bary(input, &b, &c);
                    b1 = bb;
                    c1 = cc2;
                    a1 = 1.0 - b1 - c1;
                    has_second = true;
                } else if bary_c == 0.0 {
                    // A=0,C=0 → AB, CB 두 후보
                    let (aa, bb) = on_closest_pt_to_edge_bary(input, &a, &b);
                    a0 = aa;
                    b0 = bb;
                    c0 = 1.0 - a0 - b0;

                    let (cc, bb2) = on_closest_pt_to_edge_bary(input, &c, &b);
                    c1 = cc;
                    b1 = bb2;
                    a1 = 1.0 - c1 - b1;
                    has_second = true;
                } else {
                    // A=0만 0 → BC만 후보
                    let (bb, cc) = on_closest_pt_to_edge_bary(input, &b, &c);
                    b0 = bb;
                    c0 = cc;
                    a0 = 1.0 - b0 - c0;
                }
            } else if bary_b == 0.0 {
                if bary_c == 0.0 {
                    // B=0,C=0 → BA, CA 두 후보
                    let (bb, aa) = on_closest_pt_to_edge_bary(input, &b, &a);
                    b0 = bb;
                    a0 = aa;
                    c0 = 1.0 - b0 - a0;

                    let (cc, aa2) = on_closest_pt_to_edge_bary(input, &c, &a);
                    c1 = cc;
                    a1 = aa2;
                    b1 = 1.0 - c1 - a1;
                    has_second = true;
                } else {
                    // B=0만 0 → CA만 후보
                    let (cc, aa) = on_closest_pt_to_edge_bary(input, &c, &a);
                    c0 = cc;
                    a0 = aa;
                    b0 = 1.0 - c0 - a0;
                }
            } else if bary_c == 0.0 {
                // C=0만 0 → AB만 후보
                let (aa, bb) = on_closest_pt_to_edge_bary(input, &a, &b);
                a0 = aa;
                b0 = bb;
                c0 = 1.0 - a0 - b0;
            }

            // 우선 첫 후보 채택
            let mut ba = a0;
            let mut bb_ = b0;
            let mut bc = c0;

            if has_second {
                let p0 = Point {
                    x: ba * a.x + bb_ * b.x + bc * c.x,
                    y: ba * a.y + bb_ * b.y + bc * c.y,
                    z: ba * a.z + bb_ * b.z + bc * c.z,
                };
                let p1 = Point {
                    x: a1 * a.x + b1 * b.x + c1 * c.x,
                    y: a1 * a.y + b1 * b.y + c1 * c.y,
                    z: a1 * a.z + b1 * b.z + c1 * c.z,
                };
                if Point::distance_squared_point(&p0, input)
                    > Point::distance_squared_point(&p1, input)
                {
                    ba = a1;
                    bb_ = b1;
                    bc = c1;
                }
            }

            // 최종 미세 음수는 0으로
            if ba <= ON_ZERO_TOL {
                ba = 0.0;
            }
            if bb_ <= ON_ZERO_TOL {
                bb_ = 0.0;
            }
            if bc <= ON_ZERO_TOL {
                bc = 0.0;
            }

            let out = Point {
                x: ba * a.x + bb_ * b.x + bc * c.x,
                y: ba * a.y + bb_ * b.y + bc * c.y,
                z: ba * a.z + bb_ * b.z + bc * c.z,
            };
            return (out, ba, bb_, bc);
        }

        // 평면 내부 일반 케이스
        let out = Point {
            x: bary_a * a.x + bary_b * b.x + bary_c * c.x,
            y: bary_a * a.y + bary_b * b.y + bary_c * c.y,
            z: bary_a * a.z + bary_b * b.z + bary_c * c.z,
        };
        return (out, bary_a, bary_b, bary_c);
    }

    // 2) 헬퍼 실패(퇴화) → 에지 별로 비교
    let (mut ba, mut bb) = on_closest_pt_to_edge_bary(input, &a, &b);
    let mut bc = 0.0;
    let mut best = Point {
        x: ba * a.x + bb * b.x,
        y: ba * a.y + bb * b.y,
        z: ba * a.z + bb * b.z,
    };
    let mut best_d2 = Point::distance_squared_point(&best, input);

    // Edge BC
    let (b2, c2) = on_closest_pt_to_edge_bary(input, &b, &c);
    let cand = Point {
        x: b2 * b.x + c2 * c.x,
        y: b2 * b.y + c2 * c.y,
        z: b2 * b.z + c2 * c.z,
    };
    let d2 = Point::distance_squared_point(&cand, input);
    if d2 < best_d2 {
        best_d2 = d2;
        ba = 0.0;
        bb = b2;
        bc = c2;
        best = cand;
    }

    // Edge CA
    let (c3, a3) = on_closest_pt_to_edge_bary(input, &c, &a);
    let cand2 = Point {
        x: c3 * c.x + a3 * a.x,
        y: c3 * c.y + a3 * a.y,
        z: c3 * c.z + a3 * a.z,
    };
    let d3 = Point::distance_squared_point(&cand2, input);
    if d3 < best_d2 {
        ba = a3;
        bb = 0.0;
        bc = c3;
        best = cand2;
    }

    (best, ba, bb, bc)
}
```
```rust
pub fn on_closest_pt_to_mesh_face(mesh: &Mesh, fi: usize, pt_in: &Point) -> (Point, [f64; 4]) {
    let face = mesh.faces[fi];
    let v0 = mesh.vertices[face[0] as usize];
    let v1 = mesh.vertices[face[1] as usize];
    let v2 = mesh.vertices[face[2] as usize];
    let v3 = mesh.vertices[face[3] as usize];

    // triangle?
    if face[2] == face[3] {
        let tri = [v0, v1, v2];
        let (q, a, b, c) = on_closest_pt_to_triangle(pt_in, &tri);
        return (q, [a, b, c, 0.0]);
    }

    // quad: 두 대각선 길이 비교
    let d02 = (v0 - v2).length();
    let d13 = (v1 - v3).length();

    // 두 삼각형 후보의 최근접점/바리센트릭
    let (q1, t1) = if d02 <= d13 {
        // 삼각형 (0,1,2)
        let tri012 = [v0, v1, v2];
        let (q, a, b, c) = on_closest_pt_to_triangle(pt_in, &tri012);
        (q, [a, b, c, 0.0])
    } else {
        // 삼각형 (0,1,3)
        let tri013 = [v0, v1, v3];
        let (q, a, b, d) = on_closest_pt_to_triangle(pt_in, &tri013);
        (q, [a, b, 0.0, d])
    };

    let (q2, t2) = if d02 <= d13 {
        // 삼각형 (0,2,3)  → a->t2[0], c->t2[2], d->t2[3], b=0
        let tri023 = [v0, v2, v3];
        let (q, a, c, d) = on_closest_pt_to_triangle(pt_in, &tri023);
        (q, [a, 0.0, c, d])
    } else {
        // 삼각형 (1,2,3)  → b->t2[1], c->t2[2], d->t2[3], a=0
        let tri123 = [v1, v2, v3];
        let (q, b, c, d) = on_closest_pt_to_triangle(pt_in, &tri123);
        (q, [0.0, b, c, d])
    };

    // 더 가까운 후보 선택
    let d1 = (q1 - *pt_in).length();
    let d2 = (q2 - *pt_in).length();
    if d2 < d1 { (q2, t2) } else { (q1, t1) }
}
```
```rust
pub fn on_mesh_face_centers_and_normals(mesh: &mut Mesh) -> (Vec<Point>, Vec<Vector>) {

    // face normals 보장
    mesh.compute_normals();

    let mut centers = Vec::with_capacity(mesh.faces.len());
    let mut normals = Vec::with_capacity(mesh.faces.len());

    for (fi, f) in mesh.faces.iter().enumerate() {
        let v0 = mesh.vertices[f[0] as usize];
        let v1 = mesh.vertices[f[1] as usize];
        let v2 = mesh.vertices[f[2] as usize];
        let is_tri = f[2] == f[3];
        let c = if is_tri {
            // (v0+v1+v2)/3
            Point::new(
                (v0.x + v1.x + v2.x) / 3.0,
                (v0.y + v1.y + v2.y) / 3.0,
                (v0.z + v1.z + v2.z) / 3.0,
            )
        } else {
            let v3 = mesh.vertices[f[3] as usize];
            // (v0+v1+v2+v3)/4
            Point::new(
                (v0.x + v1.x + v2.x + v3.x) / 4.0,
                (v0.y + v1.y + v2.y + v3.y) / 4.0,
                (v0.z + v1.z + v2.z + v3.z) / 4.0,
            )
        };
        // face normal은 mesh.compute_normals()에서 faces.len() 크기로 채워져 있음
        let n = mesh
            .normals
            .as_ref()
            .and_then(|ns| ns.get(fi).copied())
            .unwrap_or(Vector::new(0.0, 0.0, 0.0));

        centers.push(c);
        normals.push(n);
    }

    (centers, normals)
}
```
```rust
pub fn on_append_box_mesh(mesh: &mut Mesh, width: f64, depth: f64, height: f64) {
    let base = mesh.vertices.len() as u32;

    // 8 vertices
    let verts = [
        Point::new(0.0, 0.0, 0.0),
        Point::new(width, 0.0, 0.0),
        Point::new(width, depth, 0.0),
        Point::new(0.0, depth, 0.0),
        Point::new(0.0, 0.0, height),
        Point::new(width, 0.0, height),
        Point::new(width, depth, height),
        Point::new(0.0, depth, height),
    ];

    // helper
    let tri = |a: u32, b: u32, c: u32| -> [u32; 4] { [base + a, base + b, base + c, base + c] };

    // 12 triangles (원본과 동일한 순서)
    let faces: [[u32; 4]; 12] = [
        // bottom (z-)
        tri(3, 2, 1),
        tri(3, 1, 0),
        // front (y-)
        tri(0, 1, 5),
        tri(0, 5, 4),
        // right (x+)
        tri(1, 2, 6),
        tri(1, 6, 5),
        // back (y+)
        tri(2, 3, 7),
        tri(2, 7, 6),
        // left (x-)
        tri(3, 0, 4),
        tri(3, 4, 7),
        // top (z+)
        tri(4, 5, 6),
        tri(4, 6, 7),
    ];

    // append
    mesh.vertices.extend_from_slice(&verts);
    mesh.faces.extend_from_slice(&faces);

    // face normals 갱신
    mesh.compute_normals();
}
```
```rust
/// 필요하면 새 박스 메시를 만들어 반환하는 버전
pub fn on_create_box_mesh(width: f64, depth: f64, height: f64) -> Mesh {
    let mut m = Mesh::new(vec![], vec![]);
    on_append_box_mesh(&mut m, width, depth, height);
    m
}
```
```rust
#[inline]
fn on_get_cos_sin_array(slices: usize) -> (Vec<f64>, Vec<f64>) {
    let inc = 2.0 * std::f64::consts::PI / slices as f64;
    let mut cosines = Vec::with_capacity(slices);
    let mut sines = Vec::with_capacity(slices);
    for i in 0..slices {
        let a = i as f64 * inc;
        cosines.push(a.cos());
        sines.push(a.sin());
    }
    (cosines, sines)
}
```
```rust
#[inline]
fn on_create_cone_vertices(
    radius: f64,
    z: f64,
    slices: usize,
    cosines: &[f64],
    sines: &[f64],
    cap_offset: usize,
    vertices: &mut [Point],
) {
    for i in 0..slices {
        vertices[cap_offset + i] = Point::new(cosines[i] * radius, sines[i] * radius, z);
    }
}
```
```rust
fn on_create_cone_core(
    base_radius: f64,
    top_radius: f64,
    height: f64,
    slices: usize,
) -> (Vec<Point>, Vec<[u32; 4]>, usize, usize) {
    let mut n_caps = 1usize;
    let mut vlen = slices + 2; // 링 + (base_center, top_center)
    let mut flen = slices * 2; // 기본: 옆면(또는 옆면+한쪽 팬)

    if base_radius > EPSILON.sqrt() && top_radius > EPSILON.sqrt() {
        vlen += slices; // 위쪽 링 추가
        flen *= 2; // 반대쪽 팬/옆면 추가
        n_caps = 2;
    }

    let (cosines, sines) = on_get_cos_sin_array(slices);
    let mut vertices = vec![Point::new(0.0, 0.0, 0.0); vlen];

    // 링 세팅
    if base_radius > EPSILON.sqrt() {
        on_create_cone_vertices(base_radius, 0.0, slices, &cosines, &sines, 0, &mut vertices);
    }
    if top_radius > EPSILON.sqrt() {
        let cap_offset = slices * (n_caps - 1);
        on_create_cone_vertices(
            top_radius,
            height,
            slices,
            &cosines,
            &sines,
            cap_offset,
            &mut vertices,
        );
    }

    // 중심점 (base center, top center)
    let first_base_center = slices * n_caps;
    let base_center_idx = first_base_center;
    let top_center_idx = first_base_center + 1;
    vertices[base_center_idx] = Point::new(0.0, 0.0, 0.0);
    vertices[top_center_idx] = Point::new(0.0, 0.0, height);

    // 삼각형 생성
    let mut faces: Vec<[u32; 4]> = Vec::with_capacity(flen);

    let wrap = |i: isize, n: usize| -> usize {
        let n = n as isize;
        let mut v = i % n;
        if v < 0 {
            v += n;
        }
        v as usize
    };

    // base 링이 있으면: 옆면/아랫판 생성
    if base_radius > EPSILON.sqrt() {
        for i in 0..slices {
            let i_next = wrap((i as isize) + 1, slices);
            if n_caps == 2 {
                // 옆면 삼각형 1: (i, i_next, i_next + slices)
                let v1 = i as u32;
                let v2 = i_next as u32;
                let v3 = (i_next + slices) as u32;
                faces.push([v1, v2, v3, v3]); // tri

                // 바닥 팬: (base_center, i_next, i)
                faces.push([base_center_idx as u32, v2, v1, v1]);
            } else {
                // top apex 로 수렴 (top_center)
                // 옆면 삼각형 1: (i, i_next, top_center)
                let v1 = i as u32;
                let v2 = i_next as u32;
                let v3 = top_center_idx as u32;
                faces.push([v1, v2, v3, v3]);

                // 바닥 팬: (base_center, i_next, i)
                faces.push([base_center_idx as u32, v2, v1, v1]);
            }
        }
    }

    // top 링이 없으면 여기서 끝 (C++도 동일하게 return)
    if !(top_radius > EPSILON.sqrt()) {
        return (vertices, faces, first_base_center, n_caps);
    }

    // 위쪽이 존재
    let top_ring_offset = if n_caps == 2 { slices } else { 0 };

    for i in 0..slices {
        let i_next = wrap((i as isize) + 1, slices);

        if n_caps == 2 {
            // 옆면 삼각형 2: (i, i_next+slices, i+slices)
            let v1 = i as u32;
            let v2 = (i_next + slices) as u32;
            let v3 = (i + slices) as u32;
            faces.push([v1, v2, v3, v3]);

            // 윗판 팬: (top_center, i+slices, i_next+slices)
            faces.push([
                top_center_idx as u32,
                (i + slices) as u32,
                (i_next + slices) as u32,
                (i_next + slices) as u32,
            ]);
        } else {
            // base 링 없음: apex = base_center, side = (apex, i_next, i)
            // 이미 위에서 base 가 없으면 이 루프만으로 측면/윗판 모두 생성
            let apex = base_center_idx as u32;
            let v1 = apex;
            let v2 = (i_next + top_ring_offset) as u32;
            let v3 = (i + top_ring_offset) as u32;
            faces.push([v1, v2, v3, v3]);

            // 윗판 팬: (top_center, i, i_next)
            faces.push([
                top_center_idx as u32,
                (i + top_ring_offset) as u32,
                (i_next + top_ring_offset) as u32,
                (i_next + top_ring_offset) as u32,
            ]);
        }
    }

    (vertices, faces, first_base_center, n_caps)
}
```
```rust
pub fn on_append_cone_axis_z(
    mesh: &mut Mesh,
    base_radius: f64,
    top_radius: f64,
    height: f64,
    slices: usize,
) -> bool {
    if (base_radius < EPSILON.sqrt() && top_radius < EPSILON.sqrt())
        || height < EPSILON.sqrt()
        || slices < 3
    {
        return false;
    }
    let (vertices, faces, ..) = on_create_cone_core(base_radius, top_radius, height, slices);

    let base = mesh.vertices.len() as u32;

    // 인덱스 오프셋 후 추가
    mesh.vertices.extend_from_slice(&vertices);
    let mut ofs_faces = Vec::with_capacity(faces.len());
    for f in faces {
        ofs_faces.push([base + f[0], base + f[1], base + f[2], base + f[3]]);
    }
    mesh.faces.extend_from_slice(&ofs_faces);

    // 노멀 갱신
    mesh.compute_normals();
    true
}
```
```rust
#[allow(unused)]
fn on_get_orientation_transformation(position: Point, mut direction: Vector) -> Transform {
    // dir 정규화
    if direction.length_squared() > 0.0 {
        direction.normalize();
    } else {
        direction = Vector::new(1.0, 0.0, 0.0);
    }

    let angle_in_xy = direction.y.atan2(direction.x);
    let len_xy = (direction.x * direction.x + direction.y * direction.y).sqrt();
    let angle_from_xy = direction.z.atan2(len_xy);

    let tran = Transform::translation(position.x, position.y, position.z);
    let rot1 = Transform::rotation_axis(
        angle_in_xy,
        Vector::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, 0.0),
    );
    let rot2 = Transform::rotation_axis(
        -angle_from_xy,
        Vector::new(0.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
    );
    let _ = (rot1, rot2); // silence warn (설명용 주석)
    tran
}
```
```rust
pub fn on_append_cone_between(
    mesh: &mut Mesh,
    base_radius: f64,
    top_radius: f64,
    pt1: Point,
    pt2: Point,
    slices: usize,
) -> bool {
    let dir = Vector::new(pt2.x - pt1.x, pt2.y - pt1.y, pt2.z - pt1.z);
    let height = dir.length();

    if (base_radius < EPSILON.sqrt() && top_radius < EPSILON.sqrt())
        || height < EPSILON.sqrt()
        || slices < 3
    {
        return false;
    }

    // Z 축으로 생성
    let (mut vertices, faces, ..) = on_create_cone_core(base_radius, top_radius, height, slices);

    // C++: rot = 회전(Y, +90°). Z축을 X축으로 보냄
    let rot_y_90 = Transform::rotation_axis(
        std::f64::consts::PI / 2.0,
        Vector::new(0.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
    );

    // C++: total = GetOrientationTransformation(pt1, dir) * rot
    // 안전하게 per-vertex 순차 적용: 먼저 rot(90° Y), 그 다음 X축→dir 정렬 + pt1로 이동
    let angle_in_xy = dir.y.atan2(dir.x);
    let len_xy = (dir.x * dir.x + dir.y * dir.y).sqrt();
    let angle_from_xy = dir.z.atan2(len_xy);

    let rot1 = Transform::rotation_axis(
        angle_in_xy,
        Vector::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, 0.0),
    );
    let rot2 = Transform::rotation_axis(
        -angle_from_xy,
        Vector::new(0.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
    );
    let tran = Transform::translation(pt1.x, pt1.y, pt1.z);

    for v in &mut vertices {
        // rot (Y 90°)
        let p = rot_y_90.apply_point(*v);
        // rot2 → rot1 → tran 순서(원본과 동등 효과)
        let p = rot2.apply_point(p);
        let p = rot1.apply_point(p);
        let p = tran.apply_point(p);
        *v = p;
    }

    // append
    let base = mesh.vertices.len() as u32;
    mesh.vertices.extend_from_slice(&vertices);
    let mut ofs_faces = Vec::with_capacity(faces.len());
    for f in faces {
        ofs_faces.push([base + f[0], base + f[1], base + f[2], base + f[3]]);
    }
    mesh.faces.extend_from_slice(&ofs_faces);

    mesh.compute_normals();
    true
}
```
```rust
/// 원통(축이 z) = (원뿔) base == top
pub fn on_append_cylinder_axis_z(mesh: &mut Mesh, radius: f64, height: f64, slices: usize) -> bool {
    on_append_cone_axis_z(mesh, radius, radius, height, slices)
}
```
```rust
/// 원통(임의 방향) = (원뿔) base == top
pub fn on_append_cylinder_between(
    mesh: &mut Mesh,
    radius: f64,
    pt1: Point,
    pt2: Point,
    slices: usize,
) -> bool {
    on_append_cone_between(mesh, radius, radius, pt1, pt2, slices)
}
```
```rust
/// 구 메쉬 생성 (z-축 기준 스택/슬라이스 파라미터화)
/// - vertices: (stacks-1)개의 위도 링 * slices + 남/북극 2점
/// - faces: 2*slices*(stacks-1) 개의 삼각형
pub fn on_append_sphere(mesh: &mut Mesh, radius: f64, slices: usize, stacks: usize) -> bool {
    if !(radius > 0.0) || slices < 3 || stacks < 2 {
        return false;
    }

    let base = mesh.vertices.len() as u32;

    // 1) 경도 각도 미리 계산
    let mut cos_slice = Vec::with_capacity(slices);
    let mut sin_slice = Vec::with_capacity(slices);
    for j in 0..slices {
        let ang = 2.0 * PI * (j as f64) / (slices as f64);
        cos_slice.push(ang.cos());
        sin_slice.push(ang.sin());
    }

    // 2) 내부 위도 링(남/북극 제외) 정점 추가: i = 1..stacks-1
    mesh.vertices.reserve((stacks - 1) * slices + 2);
    for i in 1..stacks {
        if i == stacks {
            break;
        } // 논리상 실행 안 됨(안전장치)
        let phi = -0.5 * PI + (PI * (i as f64) / (stacks as f64));
        let c = phi.cos();
        let s = phi.sin();
        for j in 0..slices {
            let x = c * cos_slice[j] * radius;
            let y = c * sin_slice[j] * radius;
            let z = s * radius;
            mesh.vertices.push(Point::new(x, y, z));
        }
    }

    // 남/북극 정점
    let south_idx = mesh.vertices.len() as u32;
    mesh.vertices.push(Point::new(0.0, 0.0, -radius));
    let north_idx = mesh.vertices.len() as u32;
    mesh.vertices.push(Point::new(0.0, 0.0, radius));

    // 3) 면(삼각형) 추가
    mesh.faces.reserve(2 * slices * (stacks - 1));

    // 3-1) 내부 벨트(링 사이의 사각형 → 2삼각형), 링 개수 = stacks-1, 벨트 = (stacks-2)
    for r in 0..(stacks.saturating_sub(2)) {
        let row0 = base + (r * slices) as u32;
        let row1 = base + ((r + 1) * slices) as u32;
        for j in 0..slices {
            let jn = ((j + 1) % slices) as u32;
            let j = j as u32;

            let v00 = row0 + j;
            let v01 = row0 + jn;
            let v10 = row1 + j;
            let v11 = row1 + jn;

            // 두 삼각형
            mesh.faces.push([v00, v01, v11, v11]);
            mesh.faces.push([v00, v11, v10, v10]);
        }
    }

    // 3-2) 남극 캡: (south → 첫 링)
    let first_ring = base; // r=0 시작
    for j in 0..slices {
        let jn = ((j + 1) % slices) as u32;
        let j = j as u32;
        mesh.faces
            .push([south_idx, first_ring + jn, first_ring + j, first_ring + j]);
    }

    // 3-3) 북극 캡: (마지막 링 → north)
    let last_ring = base + ((stacks - 2) * slices) as u32; // r=(stacks-2)
    for j in 0..slices {
        let jn = ((j + 1) % slices) as u32;
        let j = j as u32;
        mesh.faces
            .push([last_ring + j, last_ring + jn, north_idx, north_idx]);
    }

    // 안전 체크(디버그)
    debug_assert!(mesh.faces.iter().all(|f| {
        let n = mesh.vertices.len() as u32;
        f[0] < n && f[1] < n && f[2] < n && f[3] < n && f[2] == f[3]
    }));

    mesh.compute_normals();
    true
}
```
```rust
pub fn on_mesh_from_tris(verts: Vec<Point>, tris: Vec<[u32; 3]>) -> Mesh {
    let faces: Vec<[u32; 4]> = tris.into_iter().map(|[a, b, c]| [a, b, c, c]).collect();
    Mesh::new(verts, faces)
}
```
```rust
/// SVG 시각화용 경계 루프 출력
pub fn on_visualize_boundary_loops(mesh: &Mesh, topo: &Topology, filename: &str) {
    use std::fs::File;
    use std::io::Write;

    let loops = topo.all_boundary_loops();
    let mut svg = String::new();
    svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="800">"#);

    for (i, loop_v) in loops.iter().enumerate() {
        let color = format!("hsl({}, 100%, 50%)", (i * 37) % 360);
        svg.push_str(&format!(r#"<polyline fill="none" stroke="{}" stroke-width="1" points=""#, color));

        for &vi in loop_v {
            let mv = topo.topv[vi].mesh_vertices[0];
            let p = &mesh.vertices[mv];
            let x = (p.x * 100.0 + 400.0) as i32;
            let y = (p.y * 100.0 + 400.0) as i32;
            svg.push_str(&format!("{} {},", x, y));
        }

        svg.push_str(r#"" />"#);
    }

    svg.push_str("</svg>");
    let mut file = File::create(filename).expect("Failed to create SVG file");
    file.write_all(svg.as_bytes()).expect("Failed to write SVG");
}
```
----

## 테스트 항목
| 항목                         | 검증 목적                                               | 기대 결과 요약                          |
|------------------------------------|----------------------------------------------------------|-----------------------------------------|
| 정점 병합 정확성                   | keymap 기반 병합이 잘 작동하는지 확인                    | 병합된 정점 수가 기대값과 일치          |
| 엣지 방향성 reve 검증              | 면 방향에 따라 reve가 올바르게 설정되는지 확인          | reve 값이 topv 순서에 따라 정확히 설정 |
| compact 후 구조 보존               | compact 이후에도 구조가 일관성 있는지 확인               | topv, tope, topf 수가 유지됨            |
| edge_map / keymap 재생성 검증      | compact 이후 해시맵이 정확히 재구성되는지 확인           | keymap, edge_map 키가 좌표/정점과 일치  |
| 경계 루프 닫힘 여부                | 경계 루프가 완전히 닫혀 있는지 확인                      | 루프의 시작 정점과 끝 정점이 동일       |


## 테스트 결과
| 항목        | 검증 상태 | 테스트 함수 예시                         | 설명 요약                                      |
|-------------|------------|-------------------------------------------|------------------------------------------------|
| keymap      | ✅ 통과     | test_topology_from_mesh_and_vertex_merge | 좌표 기반 정점 병합이 정확히 작동함            |
| reve        | ✅ 통과     | test_topology_edge_direction_reve        | 엣지 방향성과 면 방향 불일치 여부 정확히 추적 |
| compact     | ✅ 통과     | test_topology_compact_preserves_structure | 사용되지 않는 요소 제거 후 구조 일관성 유지    |
| edge_map    | ✅ 통과     | test_topology_keymap_and_edge_map_after_compact | 엣지 해시맵이 방향성 기준으로 정확히 재생성됨 |
| on_merge    | ✅ 통과     | test_mesh_merge                          | 중복 정점 제거 및 면 병합 정확히 수행됨        |
| is_planar   | ✅ 통과     | meshface_quad_planarity                  | tolerance 기반 면 평면성 판단 정확함           |



## ✅ 전체 테스트 코드 정리표
| 테스트 함수 이름                          | 주요 사용 함수/모듈                          | 검증 목적 및 설명                                      |
|-------------------------------------------|----------------------------------------------|--------------------------------------------------------|
| test_point3d_operations                   | PointOps, Point, Vector                      | Point 연산: 덧셈, 뺄셈, dot, cross                      |
| meshface_triangle_normal                  | MeshFace::compute_face_normal_from_dv        | 삼각형 면의 법선 벡터 계산                             |
| meshface_quad_planarity                   | MeshFace::is_planar, PlaneEquation           | 사각형 면의 평면성 검사                                |
| test_topology_from_mesh_and_vertex_merge | Topology::from_mesh                          | 메시 → 위상 구조 생성 및 정점 병합 확인                |
| test_topology_edge_direction_reve         | Topology::from_mesh, reve                    | 면 방향과 엣지 방향 불일치 여부(reve) 검증             |
| test_topology_compact_preserves_structure| Topology::compact                            | compact 후 구조 보존 여부 확인                         |
| test_topology_keymap_and_edge_map_after_compact | Topology::compact, keymap, edge_map     | compact 후 해시맵(keymap, edge_map) 정확성 검증        |
| test_boundary_loops                       | Topology::all_boundary_loops                 | 경계 루프 추출 기능 확인                               |
| test_boundary_loop_is_closed              | Topology::all_boundary_loops, tope_indices   | 경계 루프가 닫혀 있는지 연결성 기반으로 검증           |
| test_watertight_check                     | Topology::is_watertight                      | 메시 watertight 여부 확인                              |
| test_mesh_merge                           | on_merge_meshes                              | 두 메시 병합 후 중복 제거 및 면 수 확인                |

## 🧠 분류 요약
- 기하 연산: PointOps, Vector, MeshFace
- 위상 구조: Topology::from_mesh, compact, reve, keymap, edge_map
- 경계 처리: all_boundary_loops, is_watertight
- 병합 처리: on_merge_meshes

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::geom::Point;
    use nurbslib::core::maths::on_are_equal;
    use nurbslib::core::mesh::MeshFace;
    use nurbslib::core::plane_equation::PlaneEquation;
    use nurbslib::core::prelude::Vector;
    use nurbslib::core::types::ON_TOL9;
```
```rust
    #[test]
    fn meshface_triangle_normal() {
        let verts = vec![
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        let f = MeshFace::new_tri(0, 1, 2);
        let mut n = Vector::ZERO_VECTOR;
        assert!(MeshFace::compute_face_normal_from_dv(&f, &verts, &mut n));
        // should be +Z
        assert!(on_are_equal(n.x, 0.0, ON_TOL9) && on_are_equal(n.y, 0.0, ON_TOL9) && on_are_equal(n.z, 1.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn meshface_quad_planarity() {
        // planar unit square on XY
        let verts = vec![
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }, //0
            Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }, //1
            Point {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            }, //2
            Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }, //3
        ];
        let f = MeshFace::new_quad(0, 1, 2, 3);
        let pe: PlaneEquation;
        let mut ret = false;
        let some_pln: Option<PlaneEquation>;
        (ret, some_pln) = MeshFace::is_planar(&f, 1e-9, 0.1, &verts);
        assert!(ret);
        pe = some_pln.expect("Invalid plane");
        assert!(pe.a.abs() < 1e-9 && pe.b.abs() < 1e-9 && on_are_equal(pe.c, 1.0, ON_TOL9));
    }
}
```
```rust
#[cfg(test)]
mod mesh_tests {

    use nurbslib::core::geom::Point;
    use nurbslib::core::mesh::{on_merge_meshes, Mesh, Topology};
    use nurbslib::core::point_ops::PointOps;
    use nurbslib::core::prelude::Vector;
```
```rust
    //🧪 1.Point3D 연산 테스트
    #[test]
    fn test_point3d_operations() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = Point::new(4.0, 5.0, 6.0);

        let c = a + b;
        assert_eq!(c, Point::new(5.0, 7.0, 9.0));

        let d = b - a;
        assert_eq!(d.to_vector(), Vector::new(3.0, 3.0, 3.0));

        let dot = a.dot(&b);
        assert_eq!(dot, 32.0);

        let cross = a.cross_pt(&b);
        assert_eq!(cross.to_point(), Point::new(-3.0, 6.0, -3.0));
    }
```
```rust
    //🧪 2.Topology 생성 및 정점 병합 테스트
    #[test]
    fn test_topology_from_mesh_and_vertex_merge() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2], [0, 2, 3, 3]];
        let mesh = Mesh::new(verts, faces);
        let topo = Topology::from_mesh(&mesh, 1e-9);

        assert_eq!(topo.topv.len(), 4);
        assert_eq!(topo.topf.len(), 2);
        assert_eq!(topo.tope.len(), 5); // 5 unique edges
    }
```
```rust
    //🧪 3.경계 루프 추출 테스트
    #[test]
    fn test_boundary_loops() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2], [0, 2, 3, 3]];
        let mesh = Mesh::new(verts, faces);
        let topo = Topology::from_mesh(&mesh, 1e-9);

        let loops = topo.all_boundary_loops();
        assert_eq!(loops.len(), 1);
        assert!(loops[0].len() >= 4);
    }
```
```rust
    //🧪 4.Watertight 점검 테스트
    #[test]
    fn test_watertight_check() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2], [0, 2, 3, 3]];
        let mesh = Mesh::new(verts, faces);
        let topo = Topology::from_mesh(&mesh, 1e-9);

        assert!(!topo.is_watertight()); // 경계가 존재함
    }
```
```rust
    //🧪 5.병합 알고리즘 테스트
    #[test]
    fn test_mesh_merge() {
        let verts1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let faces1 = vec![[0, 1, 2, 2]];
        let mesh1 = Mesh::new(verts1, faces1);

        let verts2 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces2 = vec![[0, 1, 2, 2]];
        let mesh2 = Mesh::new(verts2, faces2);

        let merged = on_merge_meshes(&mesh1, &mesh2, 1e-9);
        assert!(merged.vertices.len() <= 6); // 병합되면 중복 제거됨
        assert_eq!(merged.faces.len(), 2);
    }
}
```
```rust
#[cfg(test)]
mod tests_topology {
    use nurbslib::core::mesh::{Mesh, Topology};
    use nurbslib::core::prelude::Point;
   
```
```rust
    #[test]
    fn test_topology_vertex_merge_and_keymap() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0 + 1e-10, 0.0, 0.0), // 병합 대상
            Point::new(1.0, 0.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2]];
        let mesh = Mesh::new(verts, faces);
        let topo = Topology::from_mesh(&mesh, 1e-9);

        // 병합된 정점 수 확인
        assert_eq!(topo.topv.len(), 2); // 0,1 병합됨
        assert_eq!(topo.topv_map[0], topo.topv_map[1]);
    }
```
```rust
    #[test]
    fn test_topology_edge_direction_reve() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2]];
        let mesh = Mesh::new(verts, faces.clone());
        let topo = Topology::from_mesh(&mesh, 1e-9);

        let face = &topo.topf[0];
        let fv = [
            topo.topv_map[faces[0][0] as usize],
            topo.topv_map[faces[0][1] as usize],
            topo.topv_map[faces[0][2] as usize],
        ];

        let ring = vec![(fv[2], fv[0]), (fv[0], fv[1]), (fv[1], fv[2])];

        for i in 0..3 {
            let ei = face.tope[i];
            let edge = &topo.tope[ei];
            let rev = face.reve[i];
            let (a, b) = ring[i];
            let rev_expected = !(edge.topv[0] == a && edge.topv[1] == b);
            assert_eq!(rev, rev_expected, "reve mismatch at edge {}", i);
        }
    }
```
```rust
    #[test]
    fn test_topology_compact_preserves_structure() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2], [0, 2, 3, 3]];
        let mesh = Mesh::new(verts, faces);
        let mut topo = Topology::from_mesh(&mesh, 1e-9);
        let before = (topo.topv.len(), topo.tope.len(), topo.topf.len());

        topo.compact(&mesh);
        let after = (topo.topv.len(), topo.tope.len(), topo.topf.len());

        assert_eq!(before, after); // 구조 보존
    }
```
```rust
    #[test]
    fn test_topology_keymap_and_edge_map_after_compact() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2], [0, 2, 3, 3]];
        let mesh = Mesh::new(verts, faces);
        let mut topo = Topology::from_mesh(&mesh, 1e-9);
        topo.compact(&mesh);

        for (key, vi) in &topo.keymap {
            let p = mesh.vertices[topo.topv[vi.clone()].mesh_vertices[0]];
            let q = |x: f64| (x / topo.eps).round() as i64;
            assert_eq!(key.clone(), (q(p.x), q(p.y), q(p.z)));
        }

        for (key, ei) in &topo.edge_map {
            let edge = &topo.tope[*ei];
            let sorted = if edge.topv[0] <= edge.topv[1] {
                (edge.topv[0], edge.topv[1])
            } else {
                (edge.topv[1], edge.topv[0])
            };
            assert_eq!(key.clone(), sorted);
        }
    }
```
```rust
    #[test]
    fn test_boundary_loop_is_closed() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![[0, 1, 2, 2], [0, 2, 3, 3]];
        let mesh = Mesh::new(verts, faces);
        let topo = Topology::from_mesh(&mesh, 1e-9);
        let loops = topo.all_boundary_loops();

        for loop_v in loops {
            assert!(loop_v.len() >= 3);

            // 루프가 닫혔는지: 마지막 정점과 첫 정점이 연결되어 있는지 확인
            let first = loop_v[0];
            let last = loop_v[loop_v.len() - 1];
            let connected = topo.topv[first]
                .tope_indices
                .iter()
                .any(|&ei| {
                    let e = &topo.tope[ei];
                    e.topv.contains(&last)
                });

            assert!(
                connected,
                "루프가 닫히지 않았습니다: {:?} → {:?}",
                first,
                last
            );
        }
    }
}
```
