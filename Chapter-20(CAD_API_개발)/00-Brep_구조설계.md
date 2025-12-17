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

