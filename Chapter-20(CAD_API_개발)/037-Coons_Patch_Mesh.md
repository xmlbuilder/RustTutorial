# 🧵 Coons Patch Mesh 생성기
## 📦 구조체 요약

### Vec3f
- 3D 벡터 (x, y, z)
- 벡터 연산 지원: 덧셈, 뺄셈, 스칼라 곱, 내적, 외적, 길이, 정규화
### Vec2f
- 2D 벡터 (x, y)
- 텍스처 좌표용
### CoonsMesh
- Coons 패치 결과를 담는 메쉬 구조체
    - `vertices`: 정점 리스트
    - `faces`: 면 리스트 ([u32; 4] → 삼각형은 마지막 인덱스를 중복)
    - `v_normals`: 정점 노멀
    - `tex_coords`: 텍스처 좌표
### TriStyle
- 삼각형 분할 방식
- `AlignLeft`, `AlignRight`, `UnionJack`
### CoonsOptions
- 패치 생성 옵션
    - `quad_mesh`: 사각형 메쉬 여부
    - `tri_style:` 삼각형 분할 방식
    - `build_normals`: 노멀 생성 여부
    - `build_tex_coord`: 텍스처 좌표 생성 여부
    - `use_arc_len_sampling`: 경계 파라미터를 호장 기반으로 할지 여부
    - `force_corner_match`: 코너 정렬 강제 여부
### CoonsBoundaryMaps
- 경계 파라미터 맵 (UV 및 원곡선 파라미터)

### 🧮 Coons Patch 수식 정리
- Coons Patch는 경계 곡선 4개를 기반으로 내부 보간된 표면을 생성합니다.  
- 수식은 다음과 같습니다:  

```math
P(s, t) = (1 - s) \cdot L(t) + s \cdot R(t) + (1 - t) \cdot B(s) + t \cdot T(s)
          - (1 - s)(1 - t) \cdot C_{00}
          - (1 - s)t \cdot C_{01}
          - s(1 - t) \cdot C_{10}
          - st \cdot C_{11}
```


- $L(t)$ , $R(t)$: 좌/우 경계 곡선
- $B(s)$ , $T(s)$ : 하/상 경계 곡선
- $C_{ij}$: 네 코너 점
- $s,t\in [0,1]$: 정규화된 파라미터
- 이 수식은 경계 보간의 합에서 코너 중복을 제거하는 방식으로 작동합니다.

## ⚙️ 주요 함수 설명
### on_build_coons_patch_mesh(...)
- 입력: 4개의 경계 곡선 (bottom, top, left, right)
- 출력: CoonsMesh와 선택적 CoonsBoundaryMaps
- 내부:
    - 정점 계산: Coons 수식 기반
    - 면 생성: 사각형 또는 삼각형
    - 텍스처 좌표 및 노멀 생성 (옵션에 따라)
### recompute_normals(...)
- 각 면의 노멀을 계산하고 정점 노멀을 누적 후 정규화
### coons_into_mesh(...)
- CoonsMesh를 일반적인 Mesh 타입으로 변환

### 🧩 STL 호환 삼각형 처리
- 삼각형은 [a, b, c, c] 형태로 저장되어 STL 포맷과 호환되도록 구성됩니다.

---

## 소스
```rust
use crate::core::mesh::Mesh;
use crate::core::prelude::{Point3D, Vector3D};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```
```rust
impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    pub fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    pub fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalize(self) -> Self {
        let l = self.length();
        if l > 0.0 {
            self.mul(1.0 / l)
        } else {
            Self::new(0.0, 0.0, 0.0)
        }
    }
}
```
```rust
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}
```
```rust
impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
```
```rust
#[derive(Clone, Debug)]
pub struct CoonsMesh {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<[u32; 4]>,
    pub v_normals: Vec<Vec3f>,
    pub tex_coords: Vec<Vec2f>,
}
```
```rust
impl CoonsMesh {
    pub fn empty() -> Self {
        Self {
            vertices: vec![],
            faces: vec![],
            v_normals: vec![],
            tex_coords: vec![],
        }
    }
}
```
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriStyle {
    AlignLeft,
    AlignRight,
    UnionJack,
}
```
```rust
#[derive(Copy, Clone, Debug)]
pub struct CoonsOptions {
    pub quad_mesh: bool,            // true 면 quad, false 면 triangle
    pub tri_style: TriStyle,        // 삼각 분해 방식
    pub build_normals: bool,        // 노멀 생성
    pub build_tex_coord: bool,      // (s,t) [0,1]^2 저장
    pub use_arc_len_sampling: bool, // 경계 파라미터를 호장 기반으로 기록(지오메트리엔 영향 X)
    pub force_corner_match: bool,   // 코너 정확히 일치(입력이 이미 맞다고 가정)
}
```
```rust
impl Default for CoonsOptions {
    fn default() -> Self {
        Self {
            quad_mesh: false,
            tri_style: TriStyle::AlignLeft,
            build_normals: true,
            build_tex_coord: true,
            use_arc_len_sampling: false,
            force_corner_match: true,
        }
    }
}
```
```rust
#[derive(Clone, Debug)]
pub struct CoonsBoundaryMaps {
    // 정규화된 경계 UV
    pub s_on_bottom: Vec<f64>,
    pub s_on_top: Vec<f64>, // size=Nu
    pub t_on_left: Vec<f64>,
    pub t_on_right: Vec<f64>, // size=Nv
    // 원곡선 파라미터(호장 기반 또는 균등)
    pub t_bottom: Vec<f64>,
    pub t_top: Vec<f64>, // size=Nu
    pub t_left: Vec<f64>,
    pub t_right: Vec<f64>, // size=Nv
}
```
```rust
#[allow(unused)]
#[inline]
fn grid_idx(iu: usize, iv: usize, nv: usize) -> usize {
    iu * nv + iv
}
```
```rust
#[allow(unused)]
fn cumulative_lengths(poly: &[Vec3f]) -> Vec<f64> {
    let n = poly.len();
    let mut acc = vec![0.0_f64; n];
    if n == 0 {
        return acc;
    }
    for i in 1..n {
        let d = poly[i].sub(poly[i - 1]).length() as f64;
        acc[i] = acc[i - 1] + d;
    }
    if acc[n - 1] > 0.0 {
        let total = acc[n - 1];
        for a in &mut acc[1..] {
            *a /= total;
        }
    }
    acc
}
```
```rust
#[inline]
fn push_tri(out: &mut Vec<[u32; 4]>, a: u32, b: u32, c: u32) {
    out.push([a, b, c, c]); // STL 호환: 삼각형은 마지막 인덱스를 c로 중복
}
```
```rust
#[inline]
fn push_quad(out: &mut Vec<[u32; 4]>, a: u32, b: u32, c: u32, d: u32) {
    out.push([a, b, c, d]);
}
```
```rust
/// bottom: left->right, top: left->right, left: bottom->top, right: bottom->top
pub fn on_build_coons_patch_mesh(
    bottom: &[Vec3f],
    right: &[Vec3f],
    top: &[Vec3f],
    left: &[Vec3f],
    opt: &CoonsOptions,
    _want_maps: bool,
) -> Result<(CoonsMesh, Option<CoonsBoundaryMaps>), String> {
    let nu = bottom.len();
    let nv = left.len();
    if nu < 2 || nv < 2 {
        return Err("Need at least 2 samples for each opposite boundary".into());
    }
    if top.len() != nu {
        return Err("top.size() must equal bottom.size()".into());
    }
    if right.len() != nv {
        return Err("right.size() must equal left.size()".into());
    }

    // (선택) 경계 맵 구성 — 기존 코드 유지
    // ... (maps 만드는 부분은 당신 코드 그대로 두세요)
    let maps: Option<CoonsBoundaryMaps> = None; // 필요하면 기존 로직 붙이세요

    // 코너
    let c00 = left.first().copied().unwrap();
    let c01 = left.last().copied().unwrap();
    let c10 = right.first().copied().unwrap();
    let c11 = right.last().copied().unwrap();

    // 내부 정점
    let v_count = nu * nv;
    let mut mesh = CoonsMesh {
        vertices: Vec::with_capacity(v_count),
        faces: Vec::new(),
        v_normals: Vec::new(),
        tex_coords: Vec::new(),
    };

    if opt.build_tex_coord {
        mesh.tex_coords.reserve(v_count);
    }

    for iu in 0..nu {
        let s = if nu == 1 {
            0.0
        } else {
            iu as f32 / (nu - 1) as f32
        };
        for iv in 0..nv {
            let t = if nv == 1 {
                0.0
            } else {
                iv as f32 / (nv - 1) as f32
            };

            // 경계 표본
            let l = left[iv]; // L(t)
            let r = right[iv]; // R(t)
            let b = bottom[iu]; // B(s)
            let tp = top[iu]; // T(s)

            // Coons: sum - surplus
            let sum = l
                .mul(1.0 - s)
                .add(r.mul(s))
                .add(b.mul(1.0 - t))
                .add(tp.mul(t));
            let s00 = c00.mul((1.0 - s) * (1.0 - t));
            let s01 = c01.mul((1.0 - s) * t);
            let s10 = c10.mul(s * (1.0 - t));
            let s11 = c11.mul(s * t);

            mesh.vertices.push(Vec3f::new(
                sum.x - (s00.x + s01.x + s10.x + s11.x),
                sum.y - (s00.y + s01.y + s10.y + s11.y),
                sum.z - (s00.z + s01.z + s10.z + s11.z),
            ));
            if opt.build_tex_coord {
                mesh.tex_coords.push(Vec2f { x: s, y: t });
            }
        }
    }

    // 면 생성 — 여기만 전면 교체
    let fq = (nu - 1) * (nv - 1);
    mesh.faces = Vec::with_capacity(if opt.quad_mesh { fq } else { fq * 2 });

    for iu in 1..nu {
        for iv in 1..nv {
            let n00 = grid_idx(iu - 1, iv - 1, nv) as u32;
            let n10 = grid_idx(iu, iv - 1, nv) as u32;
            let n11 = grid_idx(iu, iv, nv) as u32;
            let n01 = grid_idx(iu - 1, iv, nv) as u32;

            if opt.quad_mesh {
                push_quad(&mut mesh.faces, n00, n10, n11, n01);
            } else {
                match opt.tri_style {
                    TriStyle::AlignRight => {
                        push_tri(&mut mesh.faces, n00, n10, n11);
                        push_tri(&mut mesh.faces, n00, n11, n01);
                    }
                    TriStyle::UnionJack => {
                        let flip = (iu & 1) == (iv & 1);
                        if !flip {
                            push_tri(&mut mesh.faces, n00, n10, n01);
                            push_tri(&mut mesh.faces, n10, n11, n01);
                        } else {
                            push_tri(&mut mesh.faces, n00, n10, n11);
                            push_tri(&mut mesh.faces, n00, n11, n01);
                        }
                    }
                    TriStyle::AlignLeft => {
                        push_tri(&mut mesh.faces, n00, n10, n01);
                        push_tri(&mut mesh.faces, n10, n11, n01);
                    }
                }
            }
        }
    }

    if opt.build_normals { recompute_normals(&mut mesh); }
    Ok((mesh, maps))
}
```
```rust
/* --------------------------- 유틸: 노멀 --------------------------- */
fn face_normal(a: Vec3f, b: Vec3f, c: Vec3f) -> Vec3f {
    (b.sub(a)).cross(c.sub(a)).normalize()
}
```
```rust
pub fn recompute_normals(mesh: &mut CoonsMesh) {
    let n = mesh.vertices.len();
    mesh.v_normals.clear();
    mesh.v_normals.resize(n, Vec3f::new(0.0, 0.0, 0.0));
    for f in &mesh.faces {
        if f[2] == f[3] {
            let (a, b, c) = (f[0] as usize, f[1] as usize, f[2] as usize);
            let nrm = face_normal(mesh.vertices[a], mesh.vertices[b], mesh.vertices[c]);
            for &vi in &[a, b, c] {
                mesh.v_normals[vi] = mesh.v_normals[vi].add(nrm);
            }
        } else {
            let (a, b, c, d) = (f[0] as usize, f[1] as usize, f[2] as usize, f[3] as usize);
            let n1 = face_normal(mesh.vertices[a], mesh.vertices[b], mesh.vertices[c]);
            let n2 = face_normal(mesh.vertices[a], mesh.vertices[c], mesh.vertices[d]);
            for &vi in &[a, b, c] {
                mesh.v_normals[vi] = mesh.v_normals[vi].add(n1);
            }
            for &vi in &[a, c, d] {
                mesh.v_normals[vi] = mesh.v_normals[vi].add(n2);
            }
        }
    }
    for v in &mut mesh.v_normals {
        *v = v.normalize();
    }
}
```
```rust
pub fn coons_into_mesh(cm: &CoonsMesh) -> Mesh {
    let vertices: Vec<Point3D> = cm
        .vertices
        .iter()
        .map(|v| Point3D {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        })
        .collect();

    let faces = cm.faces.clone(); // 동일 형식 [u32;4]

    let normals = if !cm.v_normals.is_empty() {
        Some(
            cm.v_normals
                .iter()
                .map(|n| Vector3D {
                    x: n.x as f64,
                    y: n.y as f64,
                    z: n.z as f64,
                })
                .collect(),
        )
    } else {
        None
    };

    Mesh {
        vertices,
        faces,
        normals,
    }
}
```
---
# 테스트 코드

## 🧪 Coons Patch 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_coons_patch() {
        // 간단한 2x2 경계 정의
        let bottom = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 0.0, 0.0),
        ];
        let top = vec![
            Vec3f::new(0.0, 1.0, 0.0),
            Vec3f::new(1.0, 1.0, 0.0),
        ];
        let left = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
        ];
        let right = vec![
            Vec3f::new(1.0, 0.0, 0.0),
            Vec3f::new(1.0, 1.0, 0.0),
        ];

        let opt = CoonsOptions {
            quad_mesh: true,
            build_normals: true,
            build_tex_coord: true,
            ..Default::default()
        };

        let result = build_coons_patch_mesh(&bottom, &right, &top, &left, &opt, false);
        assert!(result.is_ok());

        let (mesh, _) = result.unwrap();
        assert_eq!(mesh.vertices.len(), 4); // 2x2
        assert_eq!(mesh.faces.len(), 1);    // 1 quad
        assert_eq!(mesh.tex_coords.len(), 4);

        // 노멀 확인
        if opt.build_normals {
            assert_eq!(mesh.v_normals.len(), 4);
            for n in &mesh.v_normals {
                let len = n.length();
                assert!((len - 1.0).abs() < 1e-5, "Normal not unit length");
            }
        }
    }
}
```
## 🧩 테스트 포인트
- 2×2 경계로 최소 Coons 패치 생성
- 정점 수, 면 수, 텍스처 좌표 수 확인
- 노멀 생성 시 단위 벡터 여부 확인

## 🧪 곡선형 Coons Patch 테스트
```rust
#[test]
fn test_curved_coons_patch() {
    use std::f32::consts::PI;

    // 곡선 형태의 경계 정의
    let nu = 10;
    let nv = 8;

    // bottom: 반원 아크 (x축 기준)
    let bottom: Vec<Vec3f> = (0..nu)
        .map(|i| {
            let theta = i as f32 / (nu - 1) as f32 * PI;
            Vec3f::new(theta.cos(), 0.0, theta.sin())
        })
        .collect();

    // top: 위로 볼록한 곡선
    let top: Vec<Vec3f> = (0..nu)
        .map(|i| {
            let x = i as f32 / (nu - 1) as f32;
            Vec3f::new(2.0 * x - 1.0, 1.0, 0.0)
        })
        .collect();

    // left: S자 곡선
    let left: Vec<Vec3f> = (0..nv)
        .map(|i| {
            let t = i as f32 / (nv - 1) as f32;
            Vec3f::new(-1.0, t, (t * 2.0 * PI).sin() * 0.2)
        })
        .collect();

    // right: 직선
    let right: Vec<Vec3f> = (0..nv)
        .map(|i| {
            let t = i as f32 / (nv - 1) as f32;
            Vec3f::new(1.0, t, 0.0)
        })
        .collect();

    let opt = CoonsOptions {
        quad_mesh: false,
        tri_style: TriStyle::UnionJack,
        build_normals: true,
        build_tex_coord: true,
        ..Default::default()
    };

    let result = build_coons_patch_mesh(&bottom, &right, &top, &left, &opt, false);
    assert!(result.is_ok());

    let (mesh, _) = result.unwrap();
    assert_eq!(mesh.vertices.len(), nu * nv);
    assert_eq!(mesh.tex_coords.len(), nu * nv);
    assert!(!mesh.faces.is_empty());

    if opt.build_normals {
        for n in &mesh.v_normals {
            let len = n.length();
            assert!((len - 1.0).abs() < 1e-4, "Normal not unit length");
        }
    }
}
```
## 🧩 테스트 특징
- 곡선 경계: 반원, S자, 위로 볼록한 곡선 등 다양한 형태
- 삼각형 메쉬: UnionJack 스타일로 분할
- 정점 수: 10 × 8 = 80개
- 면 수: 144개 삼각형 (2 × 9 × 7)

---

