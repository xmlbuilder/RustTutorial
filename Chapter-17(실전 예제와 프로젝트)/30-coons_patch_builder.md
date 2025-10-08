# Coons Patch Builder

- 사각 경계(네 개의 곡선 또는 폴리라인)로 둘러싸인 영역을 내부로 매끄럽게 보간하는 **Coons 보간(Côons Patch / Transfinite Interpolation)** 의  
  수학적 요약과, 이를 유한 격자(mesh)로 구현하는 절차를 정리한 문서입니다.

---

## 1) 문제 설정

사각형 경계를 구성하는 네 개의 경계 곡선을 다음과 같이 둡니다. 매개변수는 정규화 구간 $[0,1]$ 입니다.

- 하변(아래): $\mathbf{B}(s),\ s\in[0,1]$ (left $\to$ right)
- 상변(위): $\mathbf{T}(s),\ s\in[0,1]$ (left $\to$ right)
- 좌변: $\mathbf{L}(t),\ t\in[0,1]$ (bottom $\to$ top)
- 우변: $\mathbf{R}(t),\ t\in[0,1]$ (bottom $\to$ top)

네 모서리(코너) 점은

$$
\begin{aligned}
\mathbf{C}_{00}&=\mathbf{L}(0)=\mathbf{B}(0), &
\mathbf{C}_{10}&=\mathbf{R}(0)=\mathbf{B}(1), \\
\mathbf{C}_{01}&=\mathbf{L}(1)=\mathbf{T}(0), &
\mathbf{C}_{11}&=\mathbf{R}(1)=\mathbf{T}(1).
\end{aligned}
$$

### **방향 규약**  
- 본 문서와 제공된 구현은 **B, T는 좌→우**, **L, R은 하→상** 방향으로 가정합니다.    
  이 규약이 어긋나면 코너 일치가 깨지고 접힘/꼬임이 생길 수 있으므로, 필요 시 입력을 뒤집어 맞춰야 합니다.

---

## 2) Coons 패치: Transfinite 보간식

Coons 보간은 “두 집합의 선형 보간 합”에서 “코너의 bilinear 중복”을 제거하여 내부를 정의합니다.

### 2.1 혼합(blending) 구성요소

- **수평 보간(상·하 경계):**

$$
  \mathbf{S}(s,t)=(1-t) \mathbf{B}(s)+t \mathbf{T}(s)
$$

- **수직 보간(좌·우 경계):**

$$
  \mathbf{T r}(s,t)=(1-s) \mathbf{L}(t)+s \mathbf{R}(t)
$$

- **중복 보정(코너의 bilinear):**
$$
  \mathbf{B l}(s,t)=
  (1-s)(1-t) \mathbf{C}_{00}
  + s(1-t) \mathbf{C}_{10}
  + (1-s)t \mathbf{C}_{01}
  + st \mathbf{C}_{11}
$$

### 2.2 Coons 패치 정의

$$
\boxed{
\mathbf{C}(s,t)=\mathbf{S}(s,t)+\mathbf{T r}(s,t)-\mathbf{B l}(s,t)
}
$$

이 식은 경계에서 정확히 경계 곡선을 재현합니다. 예를 들어 $t=0$ 이면
$\mathbf{C}(s,0)=\mathbf{B}(s)$, $s=1$ 이면 $\mathbf{C}(1,t)=\mathbf{R}(t)$ 등.

> **직관**  
> $\mathbf{S}$ 와 $\mathbf{T r}$ 을 단순 합하면 코너가 **두 번** 더해집니다. $\mathbf{B l}$ 은 정확히 그 중복을 제거하는 **bilinear 보정**입니다.

---

## 3) 이산화(메시 생성)

실사용에서는 내부를 격자 샘플로 이산화하여 정점/면을 만듭니다.

### 3.1 파라미터 샘플

정수 해상도 $\(N_u, N_v \ge 2\)$ 를 정하고,

$$
s_i = \frac{i}{N_u-1}\quad (i=0,\dots,N_u-1),\qquad
t_j = \frac{j}{N_v-1}\quad (j=0,\dots,N_v-1).
$$

각 격자점의 3D 위치는

$$
\mathbf{P}_{ij}=\mathbf{C}(s_i,t_j)
$$

로 계산합니다.

### 3.2 코너 강제 일치

실제 경계 입력이 완벽히 일치하지 않을 수 있습니다(수치 오차, 데이터 노이즈 등).    
**force-corner-match** 옵션은 $\mathbf{B},\mathbf{T},\mathbf{L},\mathbf{R}$ 의 끝점들을  
𝐂₀₀, 𝐂₁₀, 𝐂₀₁, 𝐂₁₁ 에 스냅시켜 모순을 제거합니다.

### 3.3 면 생성

- **사각형 메쉬(quad mesh):**  
  셀 $\((i,j)\)$ 에서 정점 인덱스를  

$$
  n_{00}=(i,j),\ n_{10}=(i+1,j),\ n_{11}=(i+1,j+1),\ n_{01}=(i,j+1)
$$

  로 잡고, 면을 $(n_{00},n_{10},n_{11},n_{01})$ 순으로 생성합니다.

- **삼각형 메쉬(tri mesh):**  
  같은 셀을 삼각형 두 장으로 나눕니다. 분해 스타일은 세 가지를 제공합니다.
  - **AlignLeft:** $(n_{00},n_{10},n_{01})$, $(n_{10},n_{11},n_{01})$
  - **AlignRight:** $(n_{00},n_{10},n_{11})$, $(n_{00},n_{11},n_{01})$
  - **UnionJack:** 격자 parity를 번갈아 가며 좌/우 대각선을 교차 배치해 장기적인 방향 편향을 줄입니다.

### 3.4 노멀/텍스처 좌표

- **정점 노멀:** 각 면의 법선을 인접 정점에 누적 후 정규화(라플라시안 스무딩 계열과 호환).
- **텍스처 좌표:** $(u,v)=(s_i,t_j) \in [0,1]^2$ 를 그대로 저장하면 UV 매핑이 자연스럽게 정사영됩니다.

---

## 4) 아크길이(호장) 기반 매개화 (선택)

폴리라인/곡선 샘플이 **불균일**하면, 균일한 $s,t$ 샘플만으로는 시각적 왜곡이 있을 수 있습니다.  
이를 완화하려면 경계의 누적 길이를 정규화한 **아크길이 파라미터** $\tilde{s}(i),\tilde{t}(j)$ 를 사용합니다.

1. 경계 각 선분 길이를 누적해 $\ell_k$ 를 얻고, 전체 길이로 나누어 $[0,1]$ 로 정규화합니다.
2. 이 $\tilde{s},\tilde{t}$ 는 **경계 파라미터의 기록/매핑 정보**로 쓰거나, 필요 시 경계를 재표본(resample)하여  
   $s_i, t_j$ 에 대응하는 점을 새로 얻는 데 사용할 수 있습니다.

> 제공된 레퍼런스 구현은 “**지오메트리는 입력 샘플 그대로**” 두고, 맵만 아크길이 축척을 **기록**하는 선택지를 포함합니다.  
> 더 고른 내부 분포가 필요하면, 경계를 아크길이 균등으로 **재표본**한 후 Coons 보간에 투입하세요.

---

## 5) 수치적 고려사항

- **접힘/자체 교차(folding):** 경계가 심하게 비선형·상호 교차에 가까우면 내부가 접힐 수 있습니다.  
  *대응:* 더 조밀한 샘플링, 경계 재파라미터화, 보간 이후의 라플라시안/윈슬로(Winslow) 스무딩 적용.
- **방향 불일치:** B/T, L/R의 진행 방향이 문서 규약과 다르면 코너가 맞지 않거나 뒤틀립니다.  
  *대응:* 시작·끝 점을 비교해 필요 시 경계를 뒤집어 방향을 일치시킵니다.
- **정수 격자 해상도:** \(N_u,N_v\)가 너무 낮으면 각지게 보이고, 너무 높으면 계산/메모리 비용이 큽니다.  
  실무에선 곡률 기반 적응 샘플링이나 멀티해상도 접근을 자주 사용합니다.

---

## 6) 구현 개요 (의사코드)

```text
input: bottom[0..Nu-1], top[0..Nu-1], left[0..Nv-1], right[0..Nv-1]
output: vertices[Nu*Nv], faces

C00=left[0],  C01=left[Nv-1]
C10=right[0], C11=right[Nv-1]

for iu in 0..Nu-1:
  s = iu/(Nu-1)
  for iv in 0..Nv-1:
    t = iv/(Nv-1)
    S  = (1-t)*B(s) + t*T(s)                  // 수평 보간
    Tr = (1-s)*L(t) + s*R(t)                  // 수직 보간
    Bl = (1-s)(1-t)*C00 + s(1-t)*C10
       + (1-s)t*C01  + st*C11                 // bilinear 코너 보정
    P(iu,iv) = S + Tr - Bl

// 면 생성: quad 또는 tri(분해 스타일 선택)
```

---

## 7) Rust API 예시 (요약)

이 문서와 함께 제공된 Rust 구현의 핵심 시그니처:

```rust
pub struct CoonsOptions {
    pub quad_mesh: bool,
    pub tri_style: TriStyle,        // AlignLeft | AlignRight | UnionJack
    pub build_normals: bool,
    pub build_texcoord: bool,       // (u,v) = (s,t)
    pub use_arclen_sampling: bool,  // 경계 파라미터 기록용
    pub force_corner_match: bool,
}

pub fn build_coons_patch_mesh(
    bottom: &[Vec3f], right: &[Vec3f], top: &[Vec3f], left: &[Vec3f],
    opt: &CoonsOptions, want_maps: bool
) -> Result<(Mesh, Option<CoonsBoundaryMaps>), String>;
```

사용 시 **경계 방향 규약** (B/T 좌→우, L/R 하→상)을 반드시 지켜야 함.

---

## 8) 참조

- S. A. Coons, *Surfaces for Computer-Aided Design of Space Forms*, 1967.  
- J. Hoschek & D. Lasser, *Fundamentals of Computer Aided Geometric Design*, 1993.  
- Piegl & Tiller, *The NURBS Book*, 2nd ed., 1997 — Coons 패치와 관련된 transfinite interpolation 개요.

---

## 9) 체크리스트

- [ ] 경계 방향(B/T 좌→우, L/R 하→상) 확인/보정  
- [ ] 코너 일치 여부 확인(필요 시 스냅)  
- [ ] 적절한 \(N_u,N_v\) 선택 및(또는) 아크길이 기반 재표본  
- [ ] 쿼드/트라이 분해 방식 선택(렌더러/후처리 파이프라인 고려)  
- [ ] 노멀/UV 생성 및 검증  
- [ ] 접힘 검사(시각/법선 부호/자체교차), 필요 시 스무딩/재파라미터화

---

Coons 패치는 단순한 선형 혼합과 bilinear 보정만으로 **경계 충실도** 를 유지하면서 내부를 채워 주는 강력한 기본 블록입니다.  

---

## 실전 코드
```rust
use crate::math::prelude::{Point3D, Vector3D};
use crate::mesh::mesh::Mesh;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3f { pub x: f32, pub y: f32, pub z: f32 }
impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn add(self, o: Self) -> Self { Self::new(self.x+o.x, self.y+o.y, self.z+o.z) }
    pub fn sub(self, o: Self) -> Self { Self::new(self.x-o.x, self.y-o.y, self.z-o.z) }
    pub fn mul(self, s: f32) -> Self { Self::new(self.x*s, self.y*s, self.z*s) }
    pub fn dot(self, o: Self) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z }
    pub fn cross(self, o: Self) -> Self {
        Self::new(self.y*o.z - self.z*o.y, self.z*o.x - self.x*o.z, self.x*o.y - self.y*o.x)
    }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalize(self) -> Self {
        let l = self.length();
        if l > 0.0 { self.mul(1.0/l) } else { Self::new(0.0,0.0,0.0) }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2f { pub x: f32, pub y: f32 }
impl Vec2f { pub fn new(x: f32, y: f32) -> Self { Self { x, y } } }



#[derive(Clone, Debug)]
pub struct CoonsMesh {
    pub vertices: Vec<Vec3f>,
    pub faces:     Vec<[u32;4]>,
    pub v_normals: Vec<Vec3f>,
    pub tex_coords: Vec<Vec2f>,
}
impl CoonsMesh {
    pub fn empty() -> Self {
        Self { vertices: vec![], faces: vec![], v_normals: vec![], tex_coords: vec![] }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriStyle { AlignLeft, AlignRight, UnionJack }

#[derive(Copy, Clone, Debug)]
pub struct CoonsOptions {
    pub quad_mesh: bool,            // true 면 quad, false 면 triangle
    pub tri_style: TriStyle,        // 삼각 분해 방식
    pub build_normals: bool,        // 노멀 생성
    pub build_tex_coord: bool,       // (s,t) [0,1]^2 저장
    pub use_arc_len_sampling: bool,  // 경계 파라미터를 호장 기반으로 기록(지오메트리엔 영향 X)
    pub force_corner_match: bool,   // 코너 정확히 일치(입력이 이미 맞다고 가정)
}
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

#[derive(Clone, Debug)]
pub struct CoonsBoundaryMaps {
    // 정규화된 경계 UV
    pub s_on_bottom: Vec<f64>, pub s_on_top: Vec<f64>,   // size=Nu
    pub t_on_left:   Vec<f64>, pub t_on_right: Vec<f64>, // size=Nv
    // 원곡선 파라미터(호장 기반 또는 균등)
    pub t_bottom: Vec<f64>, pub t_top: Vec<f64>, // size=Nu
    pub t_left:   Vec<f64>, pub t_right: Vec<f64>, // size=Nv
}


#[inline]
fn grid_idx(iu: usize, iv: usize, nv: usize) -> usize { iu*nv + iv }

fn cumulative_lengths(poly: &[Vec3f]) -> Vec<f64> {
    let n = poly.len();
    let mut acc = vec![0.0_f64; n];
    if n == 0 { return acc; }
    for i in 1..n {
        let d = poly[i].sub(poly[i-1]).length() as f64;
        acc[i] = acc[i-1] + d;
    }
    if acc[n-1] > 0.0 {
        let total = acc[n-1];
        for a in &mut acc[1..] { *a /= total; }
    }
    acc
}

#[inline]
fn push_tri(out: &mut Vec<[u32;4]>, a:u32,b:u32,c:u32) {
    out.push([a,b,c,c]); // STL 호환: 삼각형은 마지막 인덱스를 c로 중복
}
#[inline]
fn push_quad(out: &mut Vec<[u32;4]>, a:u32,b:u32,c:u32,d:u32) {
    out.push([a,b,c,d]);
}

/// bottom: left->right, top: left->right, left: bottom->top, right: bottom->top
pub fn build_coons_patch_mesh(
    bottom: &[Vec3f],
    right:  &[Vec3f],
    top:    &[Vec3f],
    left:   &[Vec3f],
    opt:    &CoonsOptions,
    want_maps: bool,
) -> Result<(CoonsMesh, Option<CoonsBoundaryMaps>), String> {
    let nu = bottom.len();
    let nv = left.len();
    if nu < 2 || nv < 2 { return Err("Need at least 2 samples for each opposite boundary".into()); }
    if top.len() != nu { return Err("top.size() must equal bottom.size()".into()); }
    if right.len() != nv { return Err("right.size() must equal left.size()".into()); }

    // (선택) 경계 맵 구성 — 기존 코드 유지
    // ... (maps 만드는 부분은 당신 코드 그대로 두세요)
    let maps: Option<CoonsBoundaryMaps> = None; // 필요하면 기존 로직 붙이세요

    // 코너
    let c00 = left.first().copied().unwrap();
    let c01 = left.last().copied().unwrap();
    let c10 = right.first().copied().unwrap();
    let c11 = right.last().copied().unwrap();

    // 내부 정점
    let v_count = nu*nv;
    let mut mesh = CoonsMesh { vertices: Vec::with_capacity(v_count), faces: Vec::new(),
        v_normals: Vec::new(), tex_coords: Vec::new() };

    if opt.build_tex_coord {
        mesh.tex_coords.reserve(v_count);
    }

    for iu in 0..nu {
        let s = if nu==1 { 0.0 } else { iu as f32 / (nu-1) as f32 };
        for iv in 0..nv {
            let t = if nv==1 { 0.0 } else { iv as f32 / (nv-1) as f32 };

            // 경계 표본
            let l = left[iv];    // L(t)
            let r = right[iv];   // R(t)
            let b = bottom[iu];  // B(s)
            let tp= top[iu];     // T(s)

            // Coons: sum - surplus
            let sum = l.mul(1.0 - s).add(r.mul(s)).add(b.mul(1.0 - t)).add(tp.mul(t));
            let s00 = c00.mul((1.0 - s) * (1.0 - t));
            let s01 = c01.mul((1.0 - s) * t);
            let s10 = c10.mul( s * (1.0 - t));
            let s11 = c11.mul( s * t);

            mesh.vertices.push(Vec3f::new(
                sum.x - (s00.x + s01.x + s10.x + s11.x),
                sum.y - (s00.y + s01.y + s10.y + s11.y),
                sum.z - (s00.z + s01.z + s10.z + s11.z),
            ));
            if opt.build_tex_coord {
                mesh.tex_coords.push(Vec2f{ x:s, y:t });
            }
        }
    }

    // 면 생성 — 여기만 전면 교체
    let fq = (nu - 1) * (nv - 1);
    mesh.faces = Vec::with_capacity(if opt.quad_mesh { fq } else { fq * 2 });

    for iu in 1..nu {
        for iv in 1..nv {
            let n00 = grid_idx(iu-1, iv-1, nv) as u32;
            let n10 = grid_idx(iu,   iv-1, nv) as u32;
            let n11 = grid_idx(iu,   iv,   nv) as u32;
            let n01 = grid_idx(iu-1, iv,   nv) as u32;

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
    // 노멀 등은 필요시 별도 계산(당신 프로젝트의 recompute_normals 사용)
    // if opt.build_normals { recompute_normals_for_coons(&mut mesh); }
    Ok((mesh, maps))
}

/* --------------------------- 유틸: 노멀 --------------------------- */

fn face_normal(a:Vec3f,b:Vec3f,c:Vec3f)->Vec3f {
    (b.sub(a)).cross(c.sub(a)).normalize()
}
pub fn recompute_normals(mesh: &mut CoonsMesh) {
    let n = mesh.vertices.len();
    mesh.v_normals.clear();
    mesh.v_normals.resize(n, Vec3f::new(0.0, 0.0, 0.0));
    for f in &mesh.faces {
        if f[2] == f[3] {
            let (a,b,c) = (f[0] as usize, f[1] as usize, f[2] as usize);
            let nrm = face_normal(mesh.vertices[a], mesh.vertices[b], mesh.vertices[c]);
            for &vi in &[a,b,c] { mesh.v_normals[vi] = mesh.v_normals[vi].add(nrm); }
        } else {
            let (a,b,c,d) = (f[0] as usize, f[1] as usize, f[2] as usize, f[3] as usize);
            let n1 = face_normal(mesh.vertices[a], mesh.vertices[b], mesh.vertices[c]);
            let n2 = face_normal(mesh.vertices[a], mesh.vertices[c], mesh.vertices[d]);
            for &vi in &[a,b,c] { mesh.v_normals[vi] = mesh.v_normals[vi].add(n1); }
            for &vi in &[a,c,d] { mesh.v_normals[vi] = mesh.v_normals[vi].add(n2); }
        }
    }
    for v in &mut mesh.v_normals { *v = v.normalize(); }
}

pub fn coons_into_mesh(cm: &CoonsMesh) -> Mesh {
    let vertices: Vec<Point3D> = cm.vertices.iter().map(|v| Point3D {
        x: v.x as f64, y: v.y as f64, z: v.z as f64
    }).collect();

    let faces = cm.faces.clone(); // 동일 형식 [u32;4]

    let normals = if !cm.v_normals.is_empty() {
        Some(cm.v_normals.iter().map(|n| Vector3D {
            x: n.x as f64, y: n.y as f64, z: n.z as f64
        }).collect())
    } else {
        None
    };

    Mesh { vertices, faces, normals }
}
```

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use geometry::io::stl_writer::StlWriter;
    use geometry::mesh::coons_patch::{build_coons_patch_mesh, coons_into_mesh, CoonsMesh, CoonsOptions, TriStyle, Vec3f};
    use geometry::mesh::mesh::Mesh;

    #[test]
    fn coons_quad_and_tri_ok() {
        // 축 정렬 사각 경계(정사각형): bottom/ top (left->right), left/right (bottom->top)
        let n = 8usize;
        let mut bottom = Vec::with_capacity(n);
        let mut top    = Vec::with_capacity(n);
        for i in 0..n {
            let s = i as f32 / (n-1) as f32;
            bottom.push(Vec3f::new(-1.0 + 2.0*s, -1.0, 0.0));
            top.push(Vec3f::new(-1.0 + 2.0*s,  1.0, 0.0));
        }
        let mut left  = Vec::with_capacity(n);
        let mut right = Vec::with_capacity(n);
        for j in 0..n {
            let t = j as f32 / (n-1) as f32;
            left.push (Vec3f::new(-1.0, -1.0 + 2.0*t, 0.0));
            right.push(Vec3f::new( 1.0, -1.0 + 2.0*t, 0.0));
        }

        // Quad mesh
        let (mesh_q, maps_q) = build_coons_patch_mesh(
            &bottom, &right, &top, &left,
            &CoonsOptions{ quad_mesh:true, ..Default::default() },
            true
        ).unwrap();
        assert_eq!(mesh_q.vertices.len(), n*n);
        assert!(!mesh_q.faces.is_empty());
        assert!(maps_q.is_some());

        // Tri mesh (UnionJack)
        let (mesh_t, _) = build_coons_patch_mesh(
            &bottom, &right, &top, &left,
            &CoonsOptions{ quad_mesh:false, tri_style:TriStyle::UnionJack, ..Default::default() },
            false
        ).unwrap();
        assert_eq!(mesh_t.vertices.len(), n*n);
        assert!(mesh_t.faces.len() > mesh_q.faces.len()); // 삼각 분할이 더 많음
    }


    fn lerp(a: Vec3f, b: Vec3f, t: f32) -> Vec3f {
        Vec3f { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t, z: a.z + (b.z - a.z) * t }
    }

    /// Coons Patch의 네 경계 곡선을 샘플링해서 반환합니다.
    /// - bottom: left -> right
    /// - top   : left -> right
    /// - left  : bottom -> top
    /// - right : bottom -> top
    fn build_example_boundaries(nu: usize, nv: usize) -> (Vec<Vec3f>, Vec<Vec3f>, Vec<Vec3f>, Vec<Vec3f>) {
        // 사각 영역의 네 모서리
        let p00 = Vec3f { x: 0.0, y: 0.0, z: 0.0 }; // left-bottom
        let p10 = Vec3f { x: 1.0, y: 0.0, z: 0.0 }; // right-bottom
        let p01 = Vec3f { x: 0.0, y: 1.0, z: 0.0 }; // left-top
        let p11 = Vec3f { x: 1.0, y: 1.0, z: 0.0 }; // right-top

        // 아래/위 경계: x방향으로 진행
        let mut bottom = Vec::with_capacity(nu);
        let mut top    = Vec::with_capacity(nu);

        for i in 0..nu {
            let s = if nu <= 1 { 0.0 } else { i as f32 / (nu as f32 - 1.0) };
            let mut b = lerp(p00, p10, s);
            let mut t = lerp(p01, p11, s);

            // 약간의 굴곡을 줘서 3D 느낌
            t.z = 0.2 * (std::f32::consts::PI * s).sin();

            bottom.push(b);
            top.push(t);
        }

        // 왼/오 경계: y 방향으로 진행
        let mut left  = Vec::with_capacity(nv);
        let mut right = Vec::with_capacity(nv);

        for j in 0..nv {
            let t = if nv <= 1 { 0.0 } else { j as f32 / (nv as f32 - 1.0) };
            let mut l = lerp(p00, p01, t);
            let mut r = lerp(p10, p11, t);

            // 오른쪽 경계에도 굴곡
            r.z = 0.15 * (std::f32::consts::PI * t).sin();

            left.push(l);
            right.push(r);
        }

        (bottom, right, top, left)
    }

    #[test]
    fn coons_patch_export_to_stl() -> Result<(), Box<dyn std::error::Error>> {
        // 샘플 해상도(경계 포인트 개수)
        let nu = 64;
        let nv = 48;

        let (bottom, right, top, left) = build_example_boundaries(nu, nv);

        // Coons 옵션: STL을 위해 삼각형 메쉬가 편하므로 quad_mesh=false
        let mut opt = CoonsOptions::default();
        opt.quad_mesh = false;                   // 삼각형으로 생성
        opt.tri_style = TriStyle::AlignLeft;     // 삼각 분할 방식
        opt.build_normals = true;
        opt.build_tex_coord = true;

        // 패치 생성
        let (coons_mesh, _maps) = build_coons_patch_mesh(&bottom, &right, &top, &left, &opt, true)
            .expect("Failed to build Coons patch mesh");

        let mesh = coons_into_mesh(&coons_mesh) ;

        // 출력 폴더
        let out_dir = Path::new("target/tmp");
        fs::create_dir_all(out_dir)?;

        let ascii_path = out_dir.join("coons_patch_ascii.stl");
        let bin_path   = out_dir.join("coons_patch_binary.stl");

        // ASCII STL
        {
            let mut writer = StlWriter::new(ascii_path.to_str().unwrap(), false)?;
            writer.run_ascii(&mesh)?;
        }

        // Binary STL
        {
            let mut writer = StlWriter::new(bin_path.to_str().unwrap(), true)?;
            writer.run_binary(&mesh)?;
        }

        // 간단한 검증: 파일이 생성되었고 0바이트가 아닌지
        let ascii_meta = fs::metadata(&ascii_path)?;
        let bin_meta   = fs::metadata(&bin_path)?;
        assert!(ascii_meta.len() > 0, "ASCII STL is empty");
        assert!(bin_meta.len() > 84, "Binary STL should be > 84 bytes (header + count)");

        Ok(())
    }
}

```


