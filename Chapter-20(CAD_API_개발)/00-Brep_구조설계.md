# B-Rep Topology 구조 정리 (Rust 기반)

- 이 문서는 본 프로젝트에서 사용 중인 **B-Rep (Boundary Representation) Topology 구조** 를 정리한 문서이다.
- 본 구조는 전통적인 CAD 커널(Parasolid/ACIS 계열)의 개념을 기반으로 하되, Rust 언어 특성에 맞게 **명시적 ID 참조 + 불변식 기반** 으로 재설계되었다.

---

## 1. B-Rep Topology 개요

- B-Rep은 **형상(Geometry)** 과 **연결 관계(Topology)** 를 분리한다.
  - Geometry
    - 점, 곡선, 면의 **수학적 정의**
  - Topology
    - 어떤 점/곡선/면이 **어떻게 연결되어 있는지**
- 이 문서는 **Topology**에 집중한다.

---

## 2. Topology 전체 계층 구조

- 최상위에서 최하위까지의 계층은 다음과 같다.

```
Region
 └─ Shell
     └─ FaceUse
         └─ LoopUse
             └─ EdgeUse
                 └─ VertexUse
```

- 각 계층은 다음 목적을 가진다.

| 계층 | 역할 |
|----|----|
| Region | 공간 구분 (solid / void) |
| Shell | 하나의 연속된 경계 집합 |
| FaceUse | Face의 방향 포함 사용 상태 |
| LoopUse | Face 경계 루프 (외곽/구멍) |
| EdgeUse | Edge의 방향 포함 사용 상태 |
| VertexUse | Vertex의 사용 맥락 |

---

## 3. Solid / Face / Edge / Vertex (기본 토폴로지)

### 3.1 Vertex

```rust
Vertex {
  id,
  position,
  uses: Vec<VertexUseId>
}
```

- 3D 공간의 점
- 하나의 Vertex는 여러 VertexUse를 가질 수 있음

---

### 3.2 Edge

```rust
Edge {
  id,
  curve: Option<CurveId>,
  interval,
  uses: Vec<EdgeUseId>
}
```
- 3D 곡선 위의 **구간**
- 여러 Face가 같은 Edge를 공유할 수 있음 (non-manifold 가능)

---

### 3.3 Face

```rust
Face {
  id,
  surface: Option<SurfaceId>,
  primary: FaceUseId,
  mate: FaceUseId
}
```

- 하나의 수학적 Surface
- 항상 **2개의 FaceUse (앞/뒤)** 를 가짐

---

## 4. Use 계층 (핵심 개념)

- B-Rep의 핵심은 **Use 객체** 이다.
- Use는 **방향** 과 **맥락** 을 포함한 토폴로지 사용 단위이다.

---

### 4.1 FaceUse

```rust
FaceUse {
  face,
  orientation,   // Same / Opposite
  loops: Vec<LoopUseId>
}
```

- Face의 한쪽 방향 사용
- FaceUse는 LoopUse들을 소유
- 일반 규칙:
  - 첫 LoopUse → outer boundary
  - 이후 LoopUse → inner holes

---

### 4.2 LoopUse

```rust
LoopUse {
  face_use,
  orientation,   // Same = outer, Opposite = inner
  start: LoopUseStart
}
```

- Face 경계 루프
- 실제 경계는 EdgeUse들의 CCW 순환으로 표현됨

#### LoopUseStart

```rust
enum LoopUseStart {
  Edge(EdgeUseId),
  Vertex(VertexUseId)
}
```

- 일반적인 경우: Edge loop
- 특수한 경우: vertex-only loop (singularity)

---

### 4.3 EdgeUse (가장 중요한 객체)

```rust
EdgeUse {
  edge,
  vertex_use,
  orientation,
  loop_use,
  next_ccw,
  prev_cw,
  radial_next,
  mate,
  pcurve
}
```

- EdgeUse는 사실상 **half-edge / co-edge** 역할을 한다.

#### EdgeUse가 관리하는 3가지 연결

- 1. Loop 연결 (경계)
```
next_ccw / prev_cw
```

- 2. Radial 연결 (같은 Edge를 공유하는 fan)
```
radial_next
```

- 3. Mate 연결 (반대 방향 짝)
```
mate
```

---

### 4.4 VertexUse

```rust
VertexUse {
  vertex,
  attach: VertexUseAttach
}
```

```rust
enum VertexUseAttach {
  Shell(ShellId),
  Edge(EdgeUseId),
  Loop(LoopUseId)
}
```

- Vertex가 **어디에서 쓰이고 있는지** 를 표현
- 하나의 Vertex가 여러 위치에서 사용 가능

---

## 5. Loop / Radial / Mate 순회 개념

### 5.1 Loop 순회 (Face boundary)

```
start EdgeUse
  -> next_ccw
  -> next_ccw
  -> ...
  -> start
```

- Face 경계를 한 바퀴 도는 순회
- 면적 계산, 트림, 테셀레이션에 사용

---

### 5.2 Radial 순회 (Edge fan)

```
EdgeUse
  -> radial_next
  -> radial_next
  -> ...
```

- 하나의 Edge를 공유하는 모든 Face 방향 순회
- non-manifold 지원의 핵심

---

### 5.3 Mate 연결

- 같은 기하를 반대 방향으로 쓰는 짝
- manifold edge에서는 보통 1:1
- non-manifold에서는 mate가 없거나 불완전할 수 있음

**중요**  
- 알고리즘은 mate보다 **radial ring** 에 더 의존해야 안전하다.

---

## 6. Geometry와의 관계

Topology는 Geometry를 직접 포함하지 않고 **ID로 참조** 한다.

| Topology | Geometry |
|--------|---------|
| Vertex | Point3D |
| Edge | ModelCurve (3D) |
| EdgeUse | PCurve (UV trim) |
| Face | Surface |

- Geometry는 언제든 교체/확장 가능
- Topology 구조는 변하지 않음

---

## 7. 이 구조로 가능한 것들

### 지원 가능
- 일반 solid / sheet body
- 다중 hole face
- non-manifold edge
- wire / vertex shell
- Boolean / Intersection / Tessellation 기반

### 추가 고려 필요
- Periodic surface seam (pcurve 다중)
- Radial prev 포인터 (성능 개선용)
- Singular face (cone apex 등)

---

## 8. 공부 포인트 요약

- 이 구조를 이해할 때 반드시 익혀야 할 3가지:
  - 1. Loop CCW 순회
  - 2. Edge Radial fan 순회
  - 3. FaceUse / LoopUse / EdgeUse 관계

- 이 3가지를 이해하면 B-Rep Topology의 80%는 이해한 것이다.

---

## 9. 결론

- 현재 구조는:
  
  - 대부분의 CAD B-Rep 모델을 수용 가능
  - Rust에 안전하게 맞춘 명시적 구조
  - 이후 Boolean / Intersection / Meshing 확장에 적합

- 본 문서는 코드와 함께 **Topology 사고의 기준 문서** 로 사용한다.

---

### topo_ids.rs
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShellId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionId(pub u32);
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexUseId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeUseId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopUseId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceUseId(pub u32);
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CurveId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PCurveId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u32);
```

### topo_flags.rs
```rust
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct TopoFlags {
    pub mark1: Cell<u64>,
    pub mark2: Cell<u64>,
}
```
```rust
impl Default for TopoFlags {
    fn default() -> Self {
        Self {
            mark1: Cell::new(0),
            mark2: Cell::new(0),
        }
    }
}
```

### geom_kernel.rs
```rust
// GeometryStore + 최소 Trait/Enum
// - Edge: 3D ModelCurve
// - EdgeUse: Face 위에 붙으면 UV p-curve (2D in UV)
// - Face: Surface

use crate::brep::{CurveId, PCurveId, SurfaceId};
use crate::core::prelude::{Interval, Real};
use crate::core::geom::{Point3D, Vector3D}; // 네 geom.rs 경로에 맞춰 조정
use crate::nurbs_curve::NurbsCurve;
use crate::nurbs_surface::NurbsSurface;

/// 3D 모델 곡선(Edge가 참조)
pub trait ModelCurve3D: std::fmt::Debug + Send + Sync {
    fn domain(&self) -> Interval;
    fn eval_point(&self, t: Real) -> Point3D;
    fn eval_tangent(&self, _t: Real) -> Option<Vector3D> { None }
}
```
```rust
/// UV p-curve(EdgeUse가 참조)
pub trait PCurve2D: std::fmt::Debug + Send + Sync {
    fn domain(&self) -> Interval;
    fn eval_uv(&self, t: Real) -> Point2D;
}
```
```rust
/// Face의 surface
pub trait SurfaceGeom: std::fmt::Debug + Send + Sync {
    fn domain_u(&self) -> Interval;
    fn domain_v(&self) -> Interval;
    fn eval_point(&self, u: Real, v: Real) -> Point3D;
}
```
```rust
/// 지원하는 3D curve 종류를 enum으로 고정(추가 확장 가능)
#[derive(Debug, Clone)]
pub enum ModelCurve {
    Nurbs(NurbsCurve),
}
```
```rust
impl ModelCurve3D for ModelCurve {
    fn domain(&self) -> Interval {
        match self {
            ModelCurve::Nurbs(c) => c.domain(),
        }
    }
    fn eval_point(&self, t: Real) -> Point3D {
        match self {
            ModelCurve::Nurbs(c) => c.eval_point(t),
        }
    }
    fn eval_tangent(&self, t: Real) -> Option<Vector3D> {
        match self {
            ModelCurve::Nurbs(c) => c.tangent_at(t),
        }
    }
}
```
```rust
/// p-curve는 “2D Nurbs”가 따로 없다면,
/// 임시로 NurbsCurve의 (x,y)를 (u,v)로 해석하는 방식으로 시작.
/// (나중에 NurbsCurve2D를 만들면 여기만 바꾸면 됨)
#[derive(Debug, Clone)]
pub enum PCurve {
    NurbsXY(NurbsCurve), // z는 무시
}
```
```rust
impl PCurve2D for PCurve {
    fn domain(&self) -> Interval {
        match self {
            PCurve::NurbsXY(c) => c.domain(),
        }
    }
    fn eval_uv(&self, t: Real) -> Point2D {
        match self {
            PCurve::NurbsXY(c) => {
                let p = c.eval_point(t);
                Point2D::new(p.x, p.y)
            }
        }
    }
}
```
```rust
#[derive(Debug, Clone)]
pub enum Surface {
    Nurbs(NurbsSurface),
}
```
```rust
impl SurfaceGeom for Surface {
    fn domain_u(&self) -> Interval {
        match self {
            Surface::Nurbs(s) => s.domain_u(),
        }
    }
    fn domain_v(&self) -> Interval {
        match self {
            Surface::Nurbs(s) => s.domain_v(),
        }
    }
    fn eval_point(&self, u: Real, v: Real) -> Point3D {
        match self {
            Surface::Nurbs(s) => s.eval_point(u, v),
        }
    }
}
```
```rust
/// Geometry 저장소: Topology는 ID로만 참조
#[derive(Debug, Default, Clone)]
pub struct GeometryStore {
    pub curves: Vec<ModelCurve>,
    pub pcurves: Vec<PCurve>,
    pub surfaces: Vec<Surface>,
}
```
```rust
impl GeometryStore {
    #[inline]
    pub fn add_curve(&mut self, c: ModelCurve) -> CurveId {
        let id = self.curves.len() as u32;
        self.curves.push(c);
        CurveId(id)
    }
```
```rust
    #[inline]
    pub fn add_pcurve(&mut self, c: PCurve) -> PCurveId {
        let id = self.pcurves.len() as u32;
        self.pcurves.push(c);
        PCurveId(id)
    }
```
```rust
    #[inline]
    pub fn add_surface(&mut self, s: Surface) -> SurfaceId {
        let id = self.surfaces.len() as u32;
        self.surfaces.push(s);
        SurfaceId(id)
    }
```
```rust
    #[inline]
    pub fn curve(&self, id: CurveId) -> &ModelCurve {
        &self.curves[id.0 as usize]
    }
```
```rust
    #[inline]
    pub fn pcurve(&self, id: PCurveId) -> &PCurve {
        &self.pcurves[id.0 as usize]
    }
```
```rust
    #[inline]
    pub fn surface(&self, id: SurfaceId) -> &Surface {
        &self.surfaces[id.0 as usize]
    }
}
```

### topology.rs
```rust
use crate::brep::*;
use crate::core::prelude::{Interval, Real};
use crate::core::geom::Point3D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Same,
    Opposite,
}
```
```rust
#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: VertexId,
    pub flags: TopoFlags,
    pub tolerance: Real,
    pub position: Point3D,
    pub uses: Vec<VertexUseId>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct Edge {
    pub id: EdgeId,
    pub flags: TopoFlags,
    pub curve: Option<CurveId>,        // 3D model curve (wire도 존재 가능)
    pub interval: Interval,            // curve param interval
    pub tolerance: Real,

    // Edge를 공유하는 edgeuse fan(=radial ring)을 만들기 위한 목록
    pub uses: Vec<EdgeUseId>,

    // edge 방향 기준 "대표" edgeuse (선택)
    pub primary: Option<EdgeUseId>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct Face {
    pub id: FaceId,
    pub flags: TopoFlags,
    pub surface: Option<SurfaceId>,
    pub uv_domain: (Interval, Interval),
    pub tolerance: Real,

    // Face는 항상 2개의 faceuse(mate pair)
    pub primary: FaceUseId,
    pub mate: FaceUseId,
}
```
```rust
#[derive(Debug, Clone)]
pub struct Loop {
    pub id: LoopId,
    pub flags: TopoFlags,
    // mate pair의 loopuse (없을 수도 있음)
    pub uses: [Option<LoopUseId>; 2],
}
```
```rust
#[derive(Debug, Clone)]
pub struct Shell {
    pub id: ShellId,
    pub flags: TopoFlags,
    pub faceuses: Vec<FaceUseId>,     // 일반 solid/lamina shell

    // wire shell을 위한 시작점(선택)
    pub wire_start: Option<EdgeUseId>,
    pub vertex_start: Option<VertexUseId>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct Region {
    pub id: RegionId,
    pub flags: TopoFlags,
    pub shells: Vec<ShellId>,
    pub is_void: bool,
}
```
```rust
/// -------------------- USE 계층 --------------------
#[derive(Debug, Clone)]
pub enum VertexUseAttach {
    Shell(ShellId),
    Edge(EdgeUseId),
    Loop(LoopUseId),
}
```
```rust
#[derive(Debug, Clone)]
pub struct VertexUse {
    pub id: VertexUseId,
    pub flags: TopoFlags,
    pub vertex: VertexId,
    pub attach: VertexUseAttach,
}
```
```rust
/// EdgeUse는 B-Rep의 “실제 경계 방향/연결” 핵심.
/// - loop 순회: next_ccw / prev_cw
/// - edge fan:  radial_next
/// - mate:      반대 방향 pairing
/// - pcurve:    face 위 UV trim curve(붙는 경우만)
#[derive(Debug, Clone)]
pub struct EdgeUse {
    pub id: EdgeUseId,
    pub flags: TopoFlags,

    pub edge: EdgeId,
    pub vertex_use: VertexUseId, // 이 EU가 시작하는 vertexuse
    pub orientation: Orientation, // edge 기준 방향

    pub loop_use: Option<LoopUseId>,
    pub shell: Option<ShellId>,

    pub next_ccw: Option<EdgeUseId>,
    pub prev_cw: Option<EdgeUseId>,

    pub radial_next: Option<EdgeUseId>,
    pub mate: Option<EdgeUseId>,

    pub pcurve: Option<PCurveId>,
}
```
```rust
#[derive(Debug, Clone)]
pub enum LoopUseStart {
    Edge(EdgeUseId),
    Vertex(VertexUseId),
}
```
```rust
#[derive(Debug, Clone)]
pub struct LoopUse {
    pub id: LoopUseId,
    pub flags: TopoFlags,
    pub loop_topo: LoopId,
    pub face_use: FaceUseId,
    pub orientation: Orientation, // outer=Same, inner=Opposite (빌더에서 강제)
    pub start: LoopUseStart,
    pub mate: Option<LoopUseId>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct FaceUse {
    pub id: FaceUseId,
    pub flags: TopoFlags,
    pub face: FaceId,
    pub orientation: Orientation,
    pub mate: Option<FaceUseId>,
    pub loops: Vec<LoopUseId>, // ★ edgeuse 목록은 loop CCW로 유도
}
```
```rust
/// -------------------- Brep --------------------
#[derive(Debug, Default, Clone)]
pub struct Brep {
    pub geom: GeometryStore,

    pub regions: Vec<Region>,
    pub shells: Vec<Shell>,
    pub faces: Vec<Face>,
    pub faceuses: Vec<FaceUse>,
    pub loops: Vec<Loop>,
    pub loopuses: Vec<LoopUse>,
    pub edges: Vec<Edge>,
    pub edgeuses: Vec<EdgeUse>,
    pub vertices: Vec<Vertex>,
    pub vertexuses: Vec<VertexUse>,

    pub tolerance: Real,
}
```

### builder.rs
```rust
use crate::brep::*;
use crate::core::prelude::{Interval, Real};
use crate::core::geom::Point3D;

/// Brep 생성 전용 빌더.
/// - 여기서 "불변식"을 만들고
/// - validate에서 무결성 검사
#[derive(Debug, Default)]
pub struct BrepBuilder {
    pub brep: Brep,
}

impl BrepBuilder {
    pub fn new(tol: Real) -> Self {
        let mut b = BrepBuilder::default();
        b.brep.tolerance = tol;
        b
    }

    #[inline] fn vid(&self) -> VertexId { VertexId(self.brep.vertices.len() as u32) }
    #[inline] fn eid(&self) -> EdgeId { EdgeId(self.brep.edges.len() as u32) }
    #[inline] fn fid(&self) -> FaceId { FaceId(self.brep.faces.len() as u32) }
    #[inline] fn luid(&self) -> LoopId { LoopId(self.brep.loops.len() as u32) }
    #[inline] fn shid(&self) -> ShellId { ShellId(self.brep.shells.len() as u32) }
    #[inline] fn rgid(&self) -> RegionId { RegionId(self.brep.regions.len() as u32) }

    #[inline] fn vuid(&self) -> VertexUseId { VertexUseId(self.brep.vertexuses.len() as u32) }
    #[inline] fn euid(&self) -> EdgeUseId { EdgeUseId(self.brep.edgeuses.len() as u32) }
    #[inline] fn luseid(&self) -> LoopUseId { LoopUseId(self.brep.loopuses.len() as u32) }
    #[inline] fn fuseid(&self) -> FaceUseId { FaceUseId(self.brep.faceuses.len() as u32) }
```
```rust
    // -------------------- geometry add --------------------
    pub fn add_curve(&mut self, c: ModelCurve) -> CurveId {
        self.brep.geom.add_curve(c)
    }
```
```rust
    pub fn add_surface(&mut self, s: Surface) -> SurfaceId {
        self.brep.geom.add_surface(s)
    }
```
```rust
    pub fn add_pcurve(&mut self, c: PCurve) -> PCurveId {
        self.brep.geom.add_pcurve(c)
    }
```
```rust
    // -------------------- topo add --------------------
    pub fn add_region(&mut self, is_void: bool) -> RegionId {
        let id = self.rgid();
        self.brep.regions.push(Region {
            id,
            flags: Default::default(),
            shells: vec![],
            is_void,
        });
        id
    }
```
```rust
    pub fn add_shell(&mut self) -> ShellId {
        let id = self.shid();
        self.brep.shells.push(Shell {
            id,
            flags: Default::default(),
            faceuses: vec![],
            wire_start: None,
            vertex_start: None,
        });
        id
    }
```
```rust
    pub fn add_shell_to_region(&mut self, region: RegionId, shell: ShellId) {
        self.brep.regions[region.0 as usize].shells.push(shell);
    }
```
```rust
    pub fn add_vertex(&mut self, p: Point3D, tol: Real) -> VertexId {
        let id = self.vid();
        self.brep.vertices.push(Vertex {
            id,
            flags: Default::default(),
            tolerance: tol,
            position: p,
            uses: vec![],
        });
        id
    }
```
```rust
    pub fn add_vertexuse(&mut self, v: VertexId, attach: VertexUseAttach) -> VertexUseId {
        let id = self.vuid();
        self.brep.vertexuses.push(VertexUse {
            id,
            flags: Default::default(),
            vertex: v,
            attach,
        });
        self.brep.vertices[v.0 as usize].uses.push(id);
        id
    }
```
```rust
    pub fn add_edge(&mut self, curve: Option<CurveId>, interval: Interval, tol: Real) -> EdgeId {
        let id = self.eid();
        self.brep.edges.push(Edge {
            id,
            flags: Default::default(),
            curve,
            interval,
            tolerance: tol,
            uses: vec![],
            primary: None,
        });
        id
    }
```
```rust
    pub fn add_edgeuse(
        &mut self,
        edge: EdgeId,
        vertex_use: VertexUseId,
        orientation: Orientation,
        loop_use: Option<LoopUseId>,
        shell: Option<ShellId>,
        pcurve: Option<PCurveId>,
    ) -> EdgeUseId {
        // 불변식: loop_use와 shell 중 하나만 Some을 권장.
        // 필요하면 여기서 debug_assert로 강제 가능.
        let id = self.euid();
        self.brep.edgeuses.push(EdgeUse {
            id,
            flags: Default::default(),
            edge,
            vertex_use,
            orientation,
            loop_use,
            shell,
            next_ccw: None,
            prev_cw: None,
            radial_next: None,
            mate: None,
            pcurve,
        });

        // edge -> uses
        self.brep.edges[edge.0 as usize].uses.push(id);
        if self.brep.edges[edge.0 as usize].primary.is_none() {
            self.brep.edges[edge.0 as usize].primary = Some(id);
        }

        id
    }
```
```rust
    /// Face 생성: surface + primary/mate faceuse 2개를 자동 생성
    pub fn add_face(&mut self, surface: Option<SurfaceId>, uv_domain: (Interval, Interval), tol: Real) -> FaceId {
        let face_id = self.fid();

        // faceuse pair
        let fu0 = self.fuseid();
        let fu1 = FaceUseId(fu0.0 + 1);

        self.brep.faceuses.push(FaceUse {
            id: fu0,
            flags: Default::default(),
            face: face_id,
            orientation: Orientation::Same,
            mate: Some(fu1),
            loops: vec![],
        });
        self.brep.faceuses.push(FaceUse {
            id: fu1,
            flags: Default::default(),
            face: face_id,
            orientation: Orientation::Opposite,
            mate: Some(fu0),
            loops: vec![],
        });

        self.brep.faces.push(Face {
            id: face_id,
            flags: Default::default(),
            surface,
            uv_domain,
            tolerance: tol,
            primary: fu0,
            mate: fu1,
        });

        face_id
    }
```
```rust
    pub fn add_faceuse_to_shell(&mut self, shell: ShellId, faceuse: FaceUseId) {
        self.brep.shells[shell.0 as usize].faceuses.push(faceuse);
    }
```
```rust
    pub fn add_loop(&mut self) -> LoopId {
        let id = self.luid();
        self.brep.loops.push(Loop {
            id,
            flags: Default::default(),
            uses: [None, None],
        });
        id
    }
```
```rust
    /// LoopUse 생성 (Edge loop)
    pub fn add_loopuse_edge(
        &mut self,
        loop_topo: LoopId,
        face_use: FaceUseId,
        orientation: Orientation,
        start_edgeuse: EdgeUseId,
        mate: Option<LoopUseId>,
    ) -> LoopUseId {
        let id = self.luseid();
        self.brep.loopuses.push(LoopUse {
            id,
            flags: Default::default(),
            loop_topo,
            face_use,
            orientation,
            start: LoopUseStart::Edge(start_edgeuse),
            mate,
        });
        id
    }
```
```rust
    /// LoopUse 생성 (Vertex-only loop)
    pub fn add_loopuse_vertex(
        &mut self,
        loop_topo: LoopId,
        face_use: FaceUseId,
        orientation: Orientation,
        start_vertexuse: VertexUseId,
        mate: Option<LoopUseId>,
    ) -> LoopUseId {
        let id = self.luseid();
        self.brep.loopuses.push(LoopUse {
            id,
            flags: Default::default(),
            loop_topo,
            face_use,
            orientation,
            start: LoopUseStart::Vertex(start_vertexuse),
            mate,
        });
        id
    }
```
```rust
    /// FaceUse에 loopuse 추가
    pub fn add_loopuse_to_faceuse(&mut self, faceuse: FaceUseId, loopuse: LoopUseId) {
        self.brep.faceuses[faceuse.0 as usize].loops.push(loopuse);
    }
```
```rust
    /// Edge loop를 만드는 헬퍼:
    /// - edgeuses: loop 경계를 이루는 edgeuse들의 순서 (CCW 순서로 넣어야 함)
    /// - 각 edgeuse.loop_use를 loopuse로 세팅
    /// - next/prev를 원형으로 세팅
    pub fn build_loop_ccw_links(&mut self, loopuse: LoopUseId, edgeuses_ccw: &[EdgeUseId]) {
        let n = edgeuses_ccw.len();
        assert!(n >= 1);

        for (i, &eu) in edgeuses_ccw.iter().enumerate() {
            let next = edgeuses_ccw[(i + 1) % n];
            let prev = edgeuses_ccw[(i + n - 1) % n];

            let e = &mut self.brep.edgeuses[eu.0 as usize];
            e.loop_use = Some(loopuse);
            e.next_ccw = Some(next);
            e.prev_cw = Some(prev);
        }
    }
```
```rust
    /// Edge의 radial ring 구축:
    /// - 해당 edge의 uses를 한 바퀴 원형으로 연결(radial_next)
    /// - mate는 별도로 set_mate_* 로 세팅
    pub fn rebuild_edge_radial_ring(&mut self, edge: EdgeId) {
        let uses = self.brep.edges[edge.0 as usize].uses.clone();
        if uses.is_empty() {
            return;
        }
        let n = uses.len();
        for i in 0..n {
            let a = uses[i];
            let b = uses[(i + 1) % n];
            self.brep.edgeuses[a.0 as usize].radial_next = Some(b);
        }
    }
```
```rust
    /// mate pairing (양방향)
    pub fn set_edgeuse_mate(&mut self, a: EdgeUseId, b: EdgeUseId) {
        self.brep.edgeuses[a.0 as usize].mate = Some(b);
        self.brep.edgeuses[b.0 as usize].mate = Some(a);
    }
}
```

### validate.rs
```rust
use crate::brep::*;

#[derive(Debug)]
pub enum BrepValidateError {
    BadId(&'static str),
    EdgeUseLoopLinkBroken { eu: EdgeUseId },
    EdgeUseRadialBroken { eu: EdgeUseId },
    EdgeUseMateBroken { eu: EdgeUseId },
    LoopUseStartInvalid { lu: LoopUseId },
    FaceUseLoopOrientationRule { fu: FaceUseId },
}
```
```rust
pub fn validate_brep(b: &Brep) -> Result<(), BrepValidateError> {
    // 1) EdgeUse의 loop 링크가 있으면 next/prev가 서로 맞아야 함
    for eu in &b.edgeuses {
        if eu.loop_use.is_some() {
            let n = eu.next_ccw.ok_or(BrepValidateError::EdgeUseLoopLinkBroken { eu: eu.id })?;
            let p = eu.prev_cw.ok_or(BrepValidateError::EdgeUseLoopLinkBroken { eu: eu.id })?;
            let n_prev = b.edgeuses[n.0 as usize].prev_cw;
            let p_next = b.edgeuses[p.0 as usize].next_ccw;
            if n_prev != Some(eu.id) || p_next != Some(eu.id) {
                return Err(BrepValidateError::EdgeUseLoopLinkBroken { eu: eu.id });
            }
        }
    }
```
```rust
    // 2) radial ring: edge가 uses를 가지고 있으면, 각 eu.radial_next는 같은 edge여야 함
    for e in &b.edges {
        for &eu_id in &e.uses {
            let eu = &b.edgeuses[eu_id.0 as usize];
            if eu.edge != e.id {
                return Err(BrepValidateError::BadId("edge.uses contains eu with different edge"));
            }
            if let Some(rn) = eu.radial_next {
                let eu2 = &b.edgeuses[rn.0 as usize];
                if eu2.edge != e.id {
                    return Err(BrepValidateError::EdgeUseRadialBroken { eu: eu.id });
                }
            }
        }
    }
```
```rust
    // 3) mate는 쌍방향이어야 함
    for eu in &b.edgeuses {
        if let Some(m) = eu.mate {
            let back = b.edgeuses[m.0 as usize].mate;
            if back != Some(eu.id) {
                return Err(BrepValidateError::EdgeUseMateBroken { eu: eu.id });
            }
        }
    }
```
```rust
    // 4) LoopUse start 체크
    for lu in &b.loopuses {
        match lu.start {
            LoopUseStart::Edge(eu) => {
                // edge loop면 해당 EU는 loop_use == Some(lu.id) 여야 정상(빌더가 세팅)
                let e = &b.edgeuses[eu.0 as usize];
                if e.loop_use != Some(lu.id) {
                    return Err(BrepValidateError::LoopUseStartInvalid { lu: lu.id });
                }
            }
            LoopUseStart::Vertex(vu) => {
                // vertex loop면 vertexuse 존재만 체크
                let _ = b.vertexuses.get(vu.0 as usize).ok_or(BrepValidateError::LoopUseStartInvalid { lu: lu.id })?;
            }
        }
    }
```
```rust
    // 5) FaceUse loop orientation 규칙(선택):
    // - 첫 루프는 outer(=Same), 나머지는 inner(=Opposite)로 쓰는 걸 권장.
    //   네가 이 규칙을 강제하기 싫으면 이 블록을 주석 처리해도 됨.
    for fu in &b.faceuses {
        if fu.loops.is_empty() {
            continue;
        }
        let lu0 = &b.loopuses[fu.loops[0].0 as usize];
        if lu0.orientation != Orientation::Same {
            return Err(BrepValidateError::FaceUseLoopOrientationRule { fu: fu.id });
        }
        for &lid in fu.loops.iter().skip(1) {
            let lu = &b.loopuses[lid.0 as usize];
            if lu.orientation != Orientation::Opposite {
                return Err(BrepValidateError::FaceUseLoopOrientationRule { fu: fu.id });
            }
        }
    }

    Ok(())
}
```
```rust
use crate::brep::*;
pub fn smoke_build_one_face_loop(mut builder: BrepBuilder) -> Brep {
    // region/shell
    let r = builder.add_region(false);
    let sh = builder.add_shell();
    builder.add_shell_to_region(r, sh);

    // face
    let face = builder.add_face(None, (Interval{t0:0.0,t1:1.0}, Interval{t0:0.0,t1:1.0}), 1e-6);
    let fu = builder.brep.faces[face.0 as usize].primary;
    builder.add_faceuse_to_shell(sh, fu);

    // loop topology + loopuse
    let loop_topo = builder.add_loop();

    // (edge/vertex/edgeuse) 3개를 만들어 triangle loop 흉내
    let v0 = builder.add_vertex(Point3D { x:0.0, y:0.0, z:0.0 }, 1e-9);
    let v1 = builder.add_vertex(Point3D { x:1.0, y:0.0, z:0.0 }, 1e-9);
    let v2 = builder.add_vertex(Point3D { x:0.0, y:1.0, z:0.0 }, 1e-9);

    // loopuse는 start edgeuse가 필요하니 일단 placeholder 생성 후,
    // edgeuse 만들고 start를 세팅하는 방식도 가능하지만,
    // 여기선 edgeuse들을 만들고 loopuse를 만든 다음 loop 링크를 세팅.
    // (loop_use 필드가 builder.build_loop_ccw_links에서 세팅됨)
    let dummy_vu = builder.add_vertexuse(v0, VertexUseAttach::Shell(sh));
    let dummy_e = builder.add_edge(None, Interval{t0:0.0,t1:1.0}, 1e-9);
    let dummy_eu = builder.add_edgeuse(dummy_e, dummy_vu, Orientation::Same, None, None, None);

    let lu = builder.add_loopuse_edge(loop_topo, fu, Orientation::Same, dummy_eu, None);
    builder.add_loopuse_to_faceuse(fu, lu);

    // 이제 실제 edgeuse 3개 생성
    let vu0 = builder.add_vertexuse(v0, VertexUseAttach::Loop(lu));
    let vu1 = builder.add_vertexuse(v1, VertexUseAttach::Loop(lu));
    let vu2 = builder.add_vertexuse(v2, VertexUseAttach::Loop(lu));

    let e01 = builder.add_edge(None, Interval{t0:0.0,t1:1.0}, 1e-9);
    let e12 = builder.add_edge(None, Interval{t0:0.0,t1:1.0}, 1e-9);
    let e20 = builder.add_edge(None, Interval{t0:0.0,t1:1.0}, 1e-9);

    let eu01 = builder.add_edgeuse(e01, vu0, Orientation::Same, None, None, None);
    let eu12 = builder.add_edgeuse(e12, vu1, Orientation::Same, None, None, None);
    let eu20 = builder.add_edgeuse(e20, vu2, Orientation::Same, None, None, None);

    // loop start를 교체하고 싶으면: lu.start를 eu01로 바꾸는 코드 추가 가능
    builder.brep.loopuses[lu.0 as usize].start = LoopUseStart::Edge(eu01);

    // CCW links 세팅
    builder.build_loop_ccw_links(lu, &[eu01, eu12, eu20]);

    // radial ring 세팅
    builder.rebuild_edge_radial_ring(e01);
    builder.rebuild_edge_radial_ring(e12);
    builder.rebuild_edge_radial_ring(e20);

    let brep = builder.brep;
    let _ = validate_brep(&brep);
    brep
}
```
### 테스트 코드
```rust
use crate::brep::*;
use crate::core::prelude::{Interval, Real};
use crate::core::geom::Point3D;

// -------------------- helpers --------------------

fn iv(a: Real, b: Real) -> Interval {
    Interval { t0: a, t1: b }
}

fn p(x: Real, y: Real, z: Real) -> Point3D {
    Point3D { x, y, z }
}

/// CCW edge loop 생성 유틸
/// - FaceUse + Loop + LoopUse
/// - vertex / vertexuse / edge / edgeuse 자동 생성
/// - CCW 링크 + radial 링까지 구성
fn make_ccw_loop(
    b: &mut BrepBuilder,
    fu: FaceUseId,
    loop_topo: LoopId,
    orientation: Orientation,
    pts_ccw: &[Point3D],
) -> (LoopUseId, Vec<EdgeUseId>) {
    assert!(pts_ccw.len() >= 3);

    // shell dummy (vertexuse attach 용)
    let sh = b.add_shell();

    // dummy edgeuse → loopuse 생성용
    let vd = b.add_vertex(pts_ccw[0], 1e-9);
    let vud = b.add_vertexuse(vd, VertexUseAttach::Shell(sh));
    let ed = b.add_edge(None, iv(0.0, 1.0), 1e-9);
    let eu_dummy = b.add_edgeuse(ed, vud, Orientation::Same, None, None, None);

    let lu = b.add_loopuse_edge(loop_topo, fu, orientation, eu_dummy, None);
    b.add_loopuse_to_faceuse(fu, lu);

    // 실제 vertexuses
    let mut vus = Vec::new();
    for &pt in pts_ccw {
        let v = b.add_vertex(pt, 1e-9);
        let vu = b.add_vertexuse(v, VertexUseAttach::Loop(lu));
        vus.push(vu);
    }

    // edges + edgeuses
    let n = pts_ccw.len();
    let mut eus = Vec::new();
    for i in 0..n {
        let e = b.add_edge(None, iv(0.0, 1.0), 1e-9);
        let eu = b.add_edgeuse(e, vus[i], Orientation::Same, None, None, None);
        eus.push(eu);
    }

    // loop start 교체
    b.brep.loopuses[lu.0 as usize].start = LoopUseStart::Edge(eus[0]);

    // CCW 링크 구성
    b.build_loop_ccw_links(lu, &eus);

    // radial ring 구성
    for &eu in &eus {
        let edge = b.brep.edgeuses[eu.0 as usize].edge;
        b.rebuild_edge_radial_ring(edge);
    }

    (lu, eus)
}
```
```rust
// -------------------- tests --------------------
#[test]
fn topo_face_single_outer_loop_ok() {
    let mut b = BrepBuilder::new(1e-6);

    let r = b.add_region(false);
    let sh = b.add_shell();
    b.add_shell_to_region(r, sh);

    let face = b.add_face(None, (iv(0.0, 1.0), iv(0.0, 1.0)), 1e-6);
    let fu = b.brep.faces[face.0 as usize].primary;
    b.add_faceuse_to_shell(sh, fu);

    let loop_topo = b.add_loop();
    let pts = [
        p(0.0, 0.0, 0.0),
        p(2.0, 0.0, 0.0),
        p(0.0, 2.0, 0.0),
    ];

    let (_lu, eus) = make_ccw_loop(&mut b, fu, loop_topo, Orientation::Same, &pts);

    let brep = b.brep.clone();
    validate_brep(&brep).expect("validate_brep failed");

    // CCW 순회 닫힘 확인
    let start = eus[0];
    let mut cur = start;
    for _ in 0..eus.len() {
        cur = brep.edgeuses[cur.0 as usize].next_ccw.unwrap();
    }
    assert_eq!(cur, start);
}
```
```rust
#[test]
fn topo_face_outer_inner_loops_ok() {
    let mut b = BrepBuilder::new(1e-6);

    let r = b.add_region(false);
    let sh = b.add_shell();
    b.add_shell_to_region(r, sh);

    let face = b.add_face(None, (iv(0.0, 1.0), iv(0.0, 1.0)), 1e-6);
    let fu = b.brep.faces[face.0 as usize].primary;
    b.add_faceuse_to_shell(sh, fu);

    // outer
    let l0 = b.add_loop();
    let outer_pts = [
        p(-5.0, -5.0, 0.0),
        p( 5.0, -5.0, 0.0),
        p( 5.0,  5.0, 0.0),
        p(-5.0,  5.0, 0.0),
    ];
    make_ccw_loop(&mut b, fu, l0, Orientation::Same, &outer_pts);

    // inner
    let l1 = b.add_loop();
    let inner_pts = [
        p(-1.0, -1.0, 0.0),
        p( 1.0, -1.0, 0.0),
        p( 1.0,  1.0, 0.0),
        p(-1.0,  1.0, 0.0),
    ];
    make_ccw_loop(&mut b, fu, l1, Orientation::Opposite, &inner_pts);

    let brep = b.brep.clone();
    validate_brep(&brep).expect("validate_brep failed");

    assert_eq!(brep.faceuses[fu.0 as usize].loops.len(), 2);
}
```
```rust
#[test]
fn topo_edgeuse_mate_and_radial_ok() {
    let mut b = BrepBuilder::new(1e-6);

    let e = b.add_edge(None, iv(0.0, 1.0), 1e-9);

    let r = b.add_region(false);
    let sh = b.add_shell();
    b.add_shell_to_region(r, sh);

    // face A/B는 “존재만” 시키고 (loop 링크는 안 만든다)
    let f0 = b.add_face(None, (iv(0.0,1.0), iv(0.0,1.0)), 1e-6);
    let fu0 = b.brep.faces[f0.0 as usize].primary;
    b.add_faceuse_to_shell(sh, fu0);

    let f1 = b.add_face(None, (iv(0.0,1.0), iv(0.0,1.0)), 1e-6);
    let fu1 = b.brep.faces[f1.0 as usize].primary;
    b.add_faceuse_to_shell(sh, fu1);

    // loopuse를 억지로 만들 필요 없음 (mate/radial 테스트 목적상)
    // 공유 edge의 EU 2개만 만들고 mate/radial 검사
    let va = b.add_vertex(p(0.0,0.0,0.0), 1e-9);
    let vua = b.add_vertexuse(va, VertexUseAttach::Shell(sh));
    let eu_a = b.add_edgeuse(e, vua, Orientation::Same, None, None, None);

    let vb = b.add_vertex(p(1.0,0.0,0.0), 1e-9);
    let vub = b.add_vertexuse(vb, VertexUseAttach::Shell(sh));
    let eu_b = b.add_edgeuse(e, vub, Orientation::Opposite, None, None, None);

    b.set_edgeuse_mate(eu_a, eu_b);
    b.rebuild_edge_radial_ring(e);

    let brep = b.brep.clone();
    validate_brep(&brep).expect("validate_brep failed (mate/radial)");

    assert_eq!(brep.edgeuses[eu_a.0 as usize].mate, Some(eu_b));
    assert_eq!(brep.edgeuses[eu_b.0 as usize].mate, Some(eu_a));

    let rn = brep.edgeuses[eu_a.0 as usize].radial_next.unwrap();
    assert_eq!(brep.edgeuses[rn.0 as usize].edge, e);
}
```
```rust
#[test]
fn topo_validate_detects_broken_loop() {
    let mut b = BrepBuilder::new(1e-6);

    let r = b.add_region(false);
    let sh = b.add_shell();
    b.add_shell_to_region(r, sh);

    let face = b.add_face(None, (iv(0.0, 1.0), iv(0.0, 1.0)), 1e-6);
    let fu = b.brep.faces[face.0 as usize].primary;
    b.add_faceuse_to_shell(sh, fu);

    let loop_topo = b.add_loop();
    let pts = [
        p(0.0, 0.0, 0.0),
        p(2.0, 0.0, 0.0),
        p(0.0, 2.0, 0.0),
    ];
    let (_lu, eus) = make_ccw_loop(&mut b, fu, loop_topo, Orientation::Same, &pts);

    // 링크 일부러 파손
    b.brep.edgeuses[eus[1].0 as usize].prev_cw = None;

    let brep = b.brep.clone();
    let err = validate_brep(&brep).unwrap_err();

    match err {
        BrepValidateError::EdgeUseLoopLinkBroken { .. } => {}
        _ => panic!("unexpected error: {:?}", err),
    }
}
```

---

# B-Rep Topology 구조 정리 (Rust 기반, 확장판)

- 본 문서는 현재 프로젝트에서 사용하는 **B-Rep Topology 구조** 를  설명을 확장 한다.
- 기본 구조 설명에 더해 다음을 포함한다.
  - ASCII 다이어그램 기반 구조 설명
  - Boolean / Intersection 관점의 Topology 동작
  - Edge Split / Face Split 시 Topology 변화 정리


## 1. 전체 Topology 계층

```
Region
 └─ Shell
     └─ FaceUse (Same / Opposite)
         └─ LoopUse (outer / inner)
             └─ EdgeUse (half-edge / coedge)
                 └─ VertexUse
```

- Region은 공간, Shell은 연속된 경계, 나머지는 모두 **Use** 기반 방향 객체이다.

---

## 2. Face Boundary Loop

- 하나의 FaceUse는 하나 이상의 LoopUse를 가진다.

```
FaceUse
 └─ LoopUse (outer)
     ┌─ eu0 -> eu1 -> eu2 -> eu3 ┐
     └───────────────────────────┘
```

- LoopUse.start 에서 시작
- EdgeUse.next_ccw 로 순회
- 반드시 원형으로 닫혀야 함

- Inner loop (hole)는 orientation = Opposite 로 구분한다.

---

## 3. Edge Radial Fan

하나의 Edge는 여러 Face에서 사용될 수 있다.

```
           Face A
             |
           eu0
             |
           eu1
             |
           eu2
             |
           eu0   (radial_next)
```

- EdgeUse.radial_next 로 fan 순회
- non-manifold 모델의 핵심 구조
- Boolean / Intersection에서 가장 중요한 순회 경로

---

## 4. Mate 관계

Manifold edge의 일반적인 형태:

```
 Face A        Face B
   eu0 <----> eu1
        mate
```

- mate는 **반대 방향 쌍** 의미
- non-manifold에서는 mate가 없거나 불완전할 수 있음
- 알고리즘은 mate보다 radial fan을 우선 사용해야 안전

---

## 5. Boolean 연산 관점의 Topology 흐름

### 5.1 Boolean 연산의 핵심 단계

1. Face–Face Intersection
2. Intersection Curve 생성
3. Edge / Face 분할
4. 새로운 Topology 재연결
5. 내부/외부 분류

- Topology 관점에서 중요한 것은 **2, 3, 4 단계** 이다.

---

### 5.2 Intersection Curve 생성

- 두 Surface의 교선은 하나 이상의 Curve 조각
- 이 Curve는 기존 Edge와 일치하거나,
  새로운 Edge로 Topology에 삽입됨

```
Face A      Face B
  |   \    |
  |    \   |
  |-----X---|  <-- intersection curve
```

---

### 5.3 Intersection 후 EdgeUse 생성

- 교선이 Face 경계를 가로지르면:

```
기존:  eu0 ---------- eu1

분할:  eu0 ---- e_new ---- eu1
```

- Edge split 발생
- 기존 EdgeUse → 2개 EdgeUse로 분해
- Radial fan 재구성 필수

---

## 6. Edge Split 시 Topology 변화

### 6.1 Edge Split 전

```
Edge E
 ├─ euA (FaceUse A)
 └─ euB (FaceUse B)
```

### 6.2 Edge Split 후

```
Edge E0        Edge E1
 ├─ euA0       ├─ euA1
 └─ euB0       └─ euB1
```

- 변경 규칙:
  - Edge → 두 Edge로 분할
  - 기존 EdgeUse → 2개의 EdgeUse
  - mate 관계 유지
  - radial ring 재구성

**주의**
- LoopUse 경계에 포함된 경우,  
  CCW/CW 링크도 반드시 업데이트해야 함

---

## 7. Face Split 시 Topology 변화

### 7.1 Face Split 전

```
FaceUse
 └─ LoopUse (outer)
```

### 7.2 Face Split 후

```
FaceUse A           FaceUse B
 └─ LoopUse A        └─ LoopUse B
      \ intersection //
```

- 변경 규칙:
  - Face → 2개의 Face
  - 각 Face는 자신의 FaceUse 쌍을 가짐
  - 기존 LoopUse는 분할되어 재배치
  - Intersection curve는 새 LoopUse의 일부가 됨

---

## 8. Intersection / Boolean 구현 시 핵심 불변식

- 항상 유지되어야 할 규칙:
  - 1. LoopUse는 반드시 닫힌 CCW 링
  - 2. EdgeUse.loop_use != None 이면 next/prev 존재
  - 3. EdgeUse.radial fan은 원형
  - 4. mate는 선택적, radial은 필수
  - 5. Topology 수정 후 validate_brep 통과

---

## 9. 구현 전략 요약

- Geometry 계산 → Curve / Surface 레벨
- Topology 변경 → EdgeUse / LoopUse / FaceUse 중심
- Boolean/Intersection은 **Topology 재구성 문제**

---

## 10. 결론

- 이 Topology 구조는:
  - 일반 CAD B-Rep의 대부분을 수용
  - Boolean / Intersection 구현에 충분한 표현력 제공
  - Rust에서 안전하게 관리 가능
- 본 문서와 `validate_brep`를 기준으로 Topology를 수정하면 구조적 오류를 방지할 수 있다.

---

# B-Rep Topology 심화 문서 (Validate / Boolean / 비교 / Traversal)

본 문서는 다음 네 가지 주제를 하나로 정리한 **심화 학습용 문서**이다.

1. validate_brep 규칙 정리
2. Boolean / Intersection 구현 순서 (pseudo-code)
3. Parasolid / ACIS 개념 비교
4. Topology Traversal 유틸 설계 가이드

---

## 1. validate_brep 규칙 정리

- validate_brep는 Topology 구조의 **불변식(invariant)** 을 강제하기 위한 최종 방어선이다.

### 1.1 Loop 관련 규칙

- LoopUse.start는 반드시 유효해야 한다
- LoopUse.start가 EdgeUse라면:
  - next_ccw, prev_cw가 모두 존재해야 함
  - CCW 순회 시 반드시 원형으로 닫혀야 함

```
eu0 -> eu1 -> eu2 -> eu0   (OK)
eu0 -> eu1 -> null         (ERROR)
```
---

### 1.2 EdgeUse Loop Link 규칙

- EdgeUse.loop_use != None 이면:
  - next_ccw != None
  - prev_cw != None

- 이를 어길 경우:
```
EdgeUseLoopLinkBroken
```

---

### 1.3 Radial Ring 규칙

- EdgeUse.radial_next는 None이거나,  
  동일 Edge를 공유하는 EdgeUse로 원형 연결되어야 함

```
eu0 -> eu1 -> eu2 -> eu0
```

- 자기 자신만 가리켜도 허용 (fan size = 1)

---

### 1.4 Mate 규칙

- mate는 선택 사항
- mate가 있다면:
  - 쌍방향이어야 함
  - 동일 Edge를 참조해야 함
---

## 2. Boolean / Intersection 구현 순서 (Pseudo-code)

### 2.1 전체 흐름

```
boolean(A, B):
  faces = intersect_faces(A, B)
  curves = build_intersection_curves(faces)
  split_edges(curves)
  split_faces(curves)
  classify_regions()
  rebuild_shells()
```

---

### 2.2 Face-Face Intersection
```
for faceA in A.faces:
  for faceB in B.faces:
    if bbox_overlap(faceA, faceB):
      curves += intersect_surface(faceA, faceB)
```

결과:
- 0개 이상의 intersection curve 조각

---

### 2.3 Edge Split

```
for curve in intersection_curves:
  for edge in crossed_edges(curve):
    split_edge(edge, curve.t)
```

split_edge 내부:

```
edge -> edge0 + edge1
edgeuses -> eu0 + eu1
fix_loop_links()
fix_radial_ring()
```
---

### 2.4 Face Split

```
for face in faces_crossed_by_curve:
  split_face(face, curve)
```

결과:
- 새로운 Face 2개
- LoopUse 재배치
- Intersection curve는 새 loop의 일부

---

## 3. Parasolid / ACIS 개념 비교

| 개념 | Parasolid | ACIS | 본 구조 |
|----|----|----|----|
| Half-edge | coedge | coedge | EdgeUse |
| Edge fan | radial | radial | radial_next |
| Face dir | face | face | FaceUse |
| Loop | loop | loop | LoopUse |
| Validation | check | audit | validate_brep |

### 핵심 차이
- Parasolid/ACIS는 포인터 기반
- 본 구조는 ID + 불변식 기반
- Rust에서 훨씬 안전
---

## 4. Topology Traversal 유틸 설계

- Traversal은 알고리즘 구현의 핵심이다.

### 4.1 Loop Traversal

```
fn walk_loop(start_eu):
  cur = start_eu
  do:
    visit(cur)
    cur = cur.next_ccw
  while cur != start_eu
```

---

### 4.2 Edge Radial Traversal

```
fn walk_radial(eu):
  cur = eu
  do:
    visit(cur)
    cur = cur.radial_next
  while cur != eu
```

---

### 4.3 Face Boundary Traversal

```
for lu in faceuse.loops:
  if lu.start is Edge:
    walk_loop(lu.start)
```

---

### 4.4 Boolean에서의 Traversal 활용

- 교선 추적 → radial traversal
- 영역 분류 → faceuse/loopuse traversal
- shell 재구성 → adjacency traversal

---

## 5. 결론

- 이 문서까지 포함하면:
  - Topology 구조 이해
  - Validation 기준 명확화
  - Boolean / Intersection 설계 가능
  - 상용 커널과의 개념 대응 완료

- 이 문서를 기준으로 Topology 구현을 확장하면 구조적 오류 없이 고급 CAD 기능을 구현할 수 있다.

---


# B-Rep Boolean & Intersection 심화 문서
## (Region Classification · Euler Operator · Intersection Tracing)

본 문서는 기존 B-Rep Topology 구조를 기반으로,
다음 두 가지 고급 주제를 **이론 + 구현 관점**에서 정리한다.

1. Boolean Region Classification (Euler Operator 관점 재해석)
2. Intersection Curve Tracing 전용 설계 문서

본 문서는 Boolean 커널 구현의 **마지막 퍼즐**에 해당한다.

---

## Brep Boolean

## 1. Euler Operator 관점에서 본 Topology

### 1.1 Euler-Poincaré 공식

- Solid B-Rep는 다음 공식을 만족해야 한다.

```
V - E + F - (L - H) = 2 (1 - G)
```

  - V: Vertex 수
  - E: Edge 수
  - F: Face 수
  - L: Loop 수
  - H: Hole 수
  - G: Genus

**핵심**
- Boolean 연산은 Topology를 변경하지만
- Euler characteristic은 보존되어야 함

---

## 1.2 Euler Operator란 무엇인가

- Euler Operator는 **Topology를 국소적으로 변경하면서** 전역 Euler 특성을 유지하는 연산이다.

- 대표 연산:

| Operator | 의미 |
|--------|----|
| MEV | Make Edge Vertex |
| MEF | Make Edge Face |
| KEV | Kill Edge Vertex |
| KEF | Kill Edge Face |
| MFK | Make Face Kill loop |
| KFM | Kill Face Make loop |

---

## 1.3 현재 구조에서의 Euler Operator 대응

| Euler Operator | 본 구조에서의 의미 |
|--------------|------------------|
| MEV | Vertex + Edge + EdgeUse 생성 |
| MEF | Edge split + Face split |
| KEV | VertexUse 제거 + Edge 병합 |
| KEF | Face 제거 + Shell 재연결 |
| MFK | LoopUse 생성 |
| KFM | LoopUse 제거 |

**중요**
- EdgeUse / LoopUse / FaceUse가 Euler 연산의 실제 대상
- Solid/Face/Edge는 결과 컨테이너에 가깝다

---

## 2. Boolean Region Classification

### 2.1 Region Classification의 본질

- Boolean 연산의 마지막 단계는:

```
이 Face가 결과 Solid에 포함되는가?
```

- 이는 **Geometry 문제가 아니라 Topology + Orientation 문제** 다.

---

## 2.2 Classification 기준

- FaceUse 단위로 판단한다.

| 조건 | 의미 |
|----|----|
| normal outward | outside |
| normal inward | inside |
| on intersection | ambiguous |

- 판정은 다음 조합으로 결정:
  - FaceUse.orientation
  - EdgeUse orientation
  - Intersection curve 진행 방향

---

## 2.3 Region Flood Fill 방식

- 가장 안정적인 방식은 **Topology 기반 flood fill** 이다.

### 알고리즘

```
mark all regions unknown

pick seed face outside A/B

queue.push(seed)

while queue not empty:
  fu = queue.pop
  for each adjacent faceuse via radial:
    if crossing intersection:
      flip inside/outside
    mark neighbor
    queue.push(neighbor)
```

**adjacent 탐색의 핵심**
- EdgeUse.radial fan traversal

---

## 2.4 validate_brep와 Classification

- Classification 이후에도 반드시:

```
validate_brep()
```

  - loop 깨짐
  - radial 끊김
  - mate 불일치

- 이 중 하나라도 발생하면 classification 결과는 신뢰 불가.

---

## 3. Intersection Curve Tracing

### 3.1 Intersection Curve란

- 두 Face의 교차 결과
- 하나 이상의 Curve segment
- Face 경계 / 내부를 자유롭게 통과

- Topology 관점에서는:

```
EdgeUse들의 연결된 chain
```

---

## 3.2 Tracing의 핵심 문제

- 1. 시작점 선택
- 2. 방향 유지
- 3. 종료 조건 판정

---

## 3.3 Seed Point 선택

Seed는 다음 중 하나:

- Face-Face intersection point
- Edge-Face intersection
- Vertex intersection

각 Seed는 다음 정보를 가진다:

```
Seed {
  point3d
  faceA, uvA
  faceB, uvB
}
```

---

## 3.4 Tracing 알고리즘 (개념)

```
trace(seed):
  cur = seed
  while true:
    step along tangent
    project to both faces
    if hit boundary:
      switch face via radial
    record segment
    if close to seed:
      break
```

**Topology가 개입하는 지점**
- face 전환: radial traversal
- loop 진입: LoopUse.start 업데이트

---

## 3.5 Tracing 결과의 Topology 반영

Tracing 완료 후:

- Curve → Edge 생성
- EdgeUse 생성
- 기존 Edge split
- LoopUse 분할 또는 생성

이 단계는 Euler Operator MEF에 해당한다.

---

## 4. Boolean + Tracing 통합 관점

Boolean 커널은 다음 3단계로 요약된다.

```
1. Intersection Tracing  (Geometry heavy)
2. Topology Reconstruction (Euler heavy)
3. Region Classification (Traversal heavy)
```

이 구조는 이 3단계를 **명확히 분리**할 수 있도록 설계되었다.

---

## 5. 결론

- Euler Operator는 Topology 변경의 이론적 기반
- 본 구조는 Euler 연산을 EdgeUse / LoopUse 중심으로 자연스럽게 표현
- Region classification은 flood fill + radial traversal 문제
- Intersection tracing은 Geometry + Topology의 경계 영역

- 이 문서까지 포함하면 B-Rep Boolean 커널의 전체 사고 흐름이 완성된다.


---
