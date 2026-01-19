# Half Edge
## Half-edge mesh 원리와 함수 설명
이 코드는 삼각형 기반 메쉬를 Half-Edge 자료구조로 표현하고, 삼각형 인덱스에서 Half-Edge 메쉬를 구축한 뒤,  
반대 방향의 half-edge(twin)를 자동으로 연결합니다.  
Half-Edge는 경량이고 순회가 빠르며, 국소적 수정이 용이해 많은 기하 알고리즘(메쉬 편집, subdivision, topology 검사, watertight 확인 등)  
에서 표준으로 쓰입니다.

## Half-edge 자료구조의 이론
### Half-edge 기본 개념
- Half-edge: 무향의 메쉬 에지 하나를 두 개의 방향성 있는 half-edge로 분해해 표현합니다. 
- 각 half-edge는 다음 정보를 가집니다:
    - 시작 정점(vertex): 이 half-edge가 출발하는 정점
    - twin: 반대 방향 half-edge (같은 에지를 반대 방향으로 순회)
    - next / prev: 동일 face 내부에서의 순서 관계
    - face: 이 half-edge가 경계로서 속한 face
    - Vertex, Face:
        - Vertex: 위치(Point3D)와, 그 정점에서 나가는 임의의 half-edge 하나의 포인터(있을 수도 없을 수도 있음).
        - Face: 그 면을 둘러싼 루프의 시작 half-edge 포인터(삼각형면이므로 3개 half-edge 순환).
### 핵심 불변 조건과 순회
- 루프 불변: Face의 half-edge를 시작점으로 next를 따라가면 닫힌 루프가 되어야 함. 삼각형은 정확히 3회 이동 후 다시 시작점.
- twin 불변: 하나의 반대 방향 half-edge가 존재하면, 두 half-edge는 같은 무향 에지를 공유하고 시작/끝 정점이 서로 뒤바뀌어야 함.
- 국소 순회:
    - face 순회: he -> he.next -> he.next.next -> ...로 동일 face 경계를 순회
    - vertex 스타 순회: 특정 정점에서 나가는 half-edge로 시작해, he.twin과 he.next를 조합하여 그 정점을 둘러싼 모든 이웃을 순회
    - 장점: 인접 관계를 포인터 한두 번으로 즉시 접근 가능. 에지/면 삽입·삭제의 국소적 업데이트가 쉬움.

## 구조체 설명
### HEVertex
- position: 점의 좌표(Point3D).
- half_edge: 이 정점에서 나가는 half-edge 중 하나(있으면 Some). 정점 중심 연산(스타 순회) 시작점으로 유용.
### HEFace
- half_edge: 이 face의 경계 루프 시작 half-edge. 여기를 시작해 next로 한 바퀴 돌면 그 face의 경계를 얻음.
### HalfEdge
- vertex: 이 half-edge의 시작 정점 인덱스.
- twin: 반대 방향 half-edge 인덱스(없을 수도 있어 open mesh 지원).
- next / prev: 같은 face 내에서의 다음/이전 half-edge. 삼각형에서는 항상 존재해야 함.
- face: 이 half-edge가 속한 face 인덱스(없을 수도 있음).
### HalfEdgeMesh
- vertices / faces / half_edges: 각각의 배열 컨테이너. 인덱스(Index = usize)로 상호 참조.

## 함수별 역할과 단계 설명
### HalfEdgeMesh::new
- 역할: 빈 HalfEdgeMesh를 생성.
- 원리: Default 구현을 사용해 벡터들을 빈 상태로 초기화.
### HalfEdgeMesh::add_vertex
- 역할: 새로운 정점을 추가하고 그 인덱스를 반환.
- 동작:
- 입력: 위치 Point3D
- 처리: HEVertex { position, half_edge: None }를 vertices에 push
    - 출력: 새 인덱스
    - 의의: 정점에 연결된 half-edge는 나중에 채워질 수 있음. 최초는 None으로 시작.
### HalfEdgeMesh::add_face_from_triangle
- 역할: 정점 인덱스 (v0, v1, v2) 삼각형으로부터 face와 그를 둘러싼 3개의 half-edge를 생성.
- 단계:
    - face 인덱스 확보: 현재 faces.len()을 face 인덱스로 사용.
    - base_he: 현재 half_edges.len()을 시작으로 3개 half-edge를 연속 push.
    - half-edge 생성:
        - he0: vertex = v0, next = he1, prev = he2, face = face_idx
        - he1: vertex = v1, next = he2, prev = he0, face = face_idx
        - he2: vertex = v2, next = he0, prev = he1, face = face_idx
    - twin은 일단 None (나중에 build_twins에서 연결)
    - face 생성: HEFace { half_edge: Some(base_he) }
    - vertex half_edge 초기 세팅: 각 v의 half_edge가 None이면 이번에 만든 half-edge를 할당
    - 불변 확보: 삼각형 루프(next/prev)가 정확히 닫히며, face의 시작점이 존재.
- 주의: Twin은 아직 연결되지 않았으므로 경계/내부 연결성은 나중에 확정.
### HalfEdgeMesh::from_triangle_indices
- 역할: 좌표 배열과 삼각형 인덱스 배열에서 HalfEdgeMesh 전체를 생성.
- 단계:
    - 정점 추가: positions의 모든 Point3D를 add_vertex로 추가.
    - 삼각형 추가: triangles의 각 [u32;3]에 대해 add_face_from_triangle 호출.
    - twin 구축: build_twins() 호출로 반대 방향 half-edge 연결.
- 의의: 기존 인덱스 기반 메쉬(예: STL/OBJ TriList)를 Half-Edge 표현으로 변환.
#### HalfEdgeMesh::build_twins
- 역할: 방향성 있는 half-edge들 사이에서 쌍(twin)을 찾아 설정.
- 핵심 원리: 동일 무향 에지는 두 방향 (u,v)와 (v,u)로 나타남. 해시맵으로 매칭.
- 단계:
    - 불변 패스: 모든 half-edge를 순회하며, 그 half-edge의 시작 정점 u = he.vertex와 **해당 half-edge의 다음** 의  
        시작 정점 v = half_edges[he.next].vertex를 얻어 방향성 에지 (u,v)를 수집.
    - 삼각형 메쉬에서 he가 가리키는 에지의 **끝 정점** 은 he.next.vertex와 동일합니다. 이유: 삼각형 루프에서 he의 끝은 next의 시작이 되기 때문.
    - 해시맵 구축: map[(u,v)] = he_idx 저장.
    - 가변 패스: 각 (u,v)에 대해 (v,u)가 존재하면 twin = opp_idx로 설정.
    - 복잡도: O(E) 시간, O(E) 공간 (E = half-edge 수; 삼각형 메쉬에서는 면 수 F에 대해 E ≈ 3F).
- 주의 사항:
    - open mesh: 경계 에지는 (u,v)는 있으나 (v,u)가 없어서 twin: None으로 남음.
    - 일관성: 삼각형 루프의 next가 항상 존재해야 하므로 입력 메쉬가 삼각형이고 각 face의 3 half-edge가 정확히 연결되어 있어야 함.

## 사용 패턴과 응용
### Face 경계 순회 예시
- 목표: Face f의 정점들을 순회.
- 순서:
    - 시작: let he_start = faces[f].half_edge.unwrap();
    - 루프: he = he_start; 반복:
    - 정점 접근: let v = half_edges[he].vertex;
    - 다음으로: he = half_edges[he].next.unwrap();
    - 시작으로 돌아오면 종료.
### 정점 스타(neighborhood) 순회
- 목표: 정점 v 주변의 이웃 정점들을 순회.
- 순서:
    - 시작: let he0 = vertices[v].half_edge?;
    - 반복: he = he0;
    - 이웃 정점: let w = half_edges[half_edges[he].next.unwrap()].vertex;
    - 다음 에지로 이동: he = half_edges[half_edges[he].twin?].next?;
    - 경계에 닿아 twin이 None이면 끝까지 한 방향만 순회 가능.
### Twin 구축의 의미
- 탑올로지 연결성: Twin 연결은 서로 다른 face가 공유하는 에지를 결합해 메쉬를 “붙이거나” 경계 여부를 판별할 수 있게 함.
- Watertight 검사: 모든 에지가 정확히 두 face에 의해 공유되면 twin이 완전하게 매칭되고, 무향 에지 카운팅으로도 c == 2를 확인 가능.

### 설계상의 선택과 안정성 포인트
- Option 사용: twin, next, prev, face, vertex.half_edge를 Option으로 둠으로써 open mesh, 부분적으로 정의된 메쉬, 단계적 구축을 안전하게 지원.
- 삼각형 가정: add_face_from_triangle는 정확히 3 half-edge를 만들고 next/prev를 닫힌 루프로 구성. 다른 다각형을 지원하려면 생성 로직을 일반화해야 함.
- 지연 twin 연결: 생성 시 twin을 바로 찾지 않고, 전체를 만든 뒤 build_twins()로 한 번에 연결. 대규모 입력에서 효율적.
- 해시맵 키: (u,v)를 키로 한 directed edge 매핑. 동일 정점 인덱스 공간 내에서 간단하고 충돌이 적음.

## 함수 요약 표

| 함수 이름                 | 설명                                      |
|---------------------------|-------------------------------------------|
| new                       | 빈 HalfEdgeMesh 생성                      |
| add_vertex                | 정점 추가 및 인덱스 반환                  |
| add_face_from_triangle    | 삼각형 face와 3개 half-edge 생성, 루프 연결 |
| from_triangle_indices     | 좌표/삼각형 인덱스에서 메쉬 구축 후 twins 연결 |
| build_twins               | (u,v) 와 (v,u) 매칭으로 반대 방향 half-edge 연결 |


### 실무 팁
- 정상성 체크: 모든 face에 대해 next/prev가 None이 아니고 3회 순회 후 시작점 복귀하는지 디버그 시 검사 필요.
- 경계 처리: twin.is_none()인 에지는 경계. 후속 알고리즘(개구부 채우기, watertight 보정)에서 중요.
- 인덱스 안정성: Vec 재할당에 주의. 인덱스 기반이므로 push만 하거나, 재할당 후 참조를 다시 가져오세요.
- 확장: quad나 n-gon을 지원하려면 add_face_from_polygon(indices: &[Index])로 일반화하고 next/prev를 원형으로 설정.

---

# 🧩 Half-Edge Mesh 응용 연산 템플릿

## 경계 추출 (Boundary Extraction)

### 원리
- twin이 없는 half-edge는 경계(edge boundary)에 해당.
- 모든 half-edge를 순회하며 twin == None인 경우를 수집.

### 템플릿
```rust
fn extract_boundaries(mesh: &HalfEdgeMesh) -> Vec<(Index, Index)> {
    let mut boundaries = Vec::new();
    for he in &mesh.half_edges {
        if he.twin.is_none() {
            let v_from = he.vertex;
            let v_to = mesh.half_edges[he.next.unwrap()].vertex;
            boundaries.push((v_from, v_to));
        }
    }
    boundaries
}
```
## Face 분할 (Face Split)

### 원리
- 하나의 face를 두 개로 나누려면, face 내부 두 정점을 연결하는 새로운 half-edge 쌍을 추가.
- 기존 face의 half-edge 루프를 두 개의 루프로 나누고, 새로운 face를 생성.

### 템플릿
```rust
fn split_face(mesh: &mut HalfEdgeMesh, face_idx: Index, v_a: Index, v_b: Index) {
    // 1. 새로운 half-edge 두 개 생성 (v_a→v_b, v_b→v_a)
    let he_ab = mesh.half_edges.len();
    let he_ba = he_ab + 1;

    mesh.half_edges.push(HalfEdge {
        vertex: v_a,
        twin: Some(he_ba),
        next: None,
        prev: None,
        face: Some(face_idx),
    });
    mesh.half_edges.push(HalfEdge {
        vertex: v_b,
        twin: Some(he_ab),
        next: None,
        prev: None,
        face: None, // 나중에 새 face에 연결
    });

    // 2. 기존 face 루프를 두 개로 나누고 새 face 생성
    let new_face_idx = mesh.faces.len();
    mesh.faces.push(HEFace { half_edge: Some(he_ba) });

    // 3. next/prev 업데이트 (실제 구현에서는 루프 탐색 필요)
}
```

## 에지 Collapse (Edge Collapse)

### 원리
- 두 정점을 하나로 합쳐서 메쉬 단순화.
- 대상 edge의 twin 관계와 face 루프를 재구성.
- collapse 후 중복 정점 제거 및 half-edge 재연결 필요.

### 템플릿
```rust
fn collapse_edge(mesh: &mut HalfEdgeMesh, he_idx: Index) {
    let he = &mesh.half_edges[he_idx];
    let v_from = he.vertex;
    let v_to = mesh.half_edges[he.next.unwrap()].vertex;

    // 1. 두 정점을 하나로 병합 (예: v_from 위치로 통합)
    mesh.vertices[v_to].position = mesh.vertices[v_from].position;

    // 2. v_to를 참조하는 half-edge들을 v_from으로 업데이트
    for h in &mut mesh.half_edges {
        if h.vertex == v_to {
            h.vertex = v_from;
        }
    }

    // 3. 관련 face와 half-edge 정리 (실제 구현에서는 루프 검사 필요)
}
```



## 📐 요약
- 경계 추출: twin이 없는 half-edge → 경계 에지 목록.
- Face 분할: face 내부 두 정점을 연결하는 새로운 half-edge 쌍 추가 → face 루프 분할.
- Edge Collapse: 두 정점을 하나로 병합 → 메쉬 단순화.


---

# 중복 Face / Edge제거

Half-Edge 메쉬를 실제로 쓰다 보면 중복 face/edge가 생길 수 있습니다.  
예를 들어 삼각형 인덱스 리스트를 그대로 Half-Edge로 변환할 때 동일한 삼각형이 두 번 들어오거나,  
동일한 에지가 여러 번 생성되는 경우가 있음.  
이런 경우를 정리하는 정규화(normalization) 기능이 필요합니다.  

## 🧩 중복 Face/Edge 제거 기능 설계
### 1. 중복 Face 제거
- 원리: 삼각형 face는 세 정점 인덱스의 집합으로 정의됩니다.
- 중복 판정: (v0, v1, v2)를 정렬한 뒤, 동일한 키가 이미 존재하면 중복 face.
- 처리: 중복 face는 제거하거나 무시.
```rust
impl HalfEdgeMesh {
    pub fn remove_duplicate_faces(&mut self) {
        use std::collections::HashSet;
        let mut seen: HashSet<[usize; 3]> = HashSet::new();
        let mut new_faces = Vec::new();
        let mut new_half_edges = Vec::new();

        for face in &self.faces {
            if let Some(he_idx) = face.half_edge {
                let he0 = &self.half_edges[he_idx];
                let he1 = &self.half_edges[he0.next.unwrap()];
                let he2 = &self.half_edges[he1.next.unwrap()];
                let mut verts = [he0.vertex, he1.vertex, he2.vertex];
                verts.sort();
                if seen.insert(verts) {
                    new_faces.push(face.clone());
                }
            }
        }
        self.faces = new_faces;
        // half_edges는 실제 구현에서는 face와 함께 재구성 필요
    }
}
```


### 2. 중복 Edge 제거
- 원리: Half-Edge는 방향성을 가지므로 (u,v)와 (v,u)는 twin 관계.
- 중복 판정: 동일한 (u,v)가 여러 half-edge에 존재하면 중복.
- 처리: 하나만 남기고 나머지는 제거.

```rust
impl HalfEdgeMesh {
    pub fn remove_duplicate_edges(&mut self) {
        use std::collections::HashSet;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut new_half_edges = Vec::new();

        for he in &self.half_edges {
            if let Some(next_idx) = he.next {
                let v_to = self.half_edges[next_idx].vertex;
                let key = (he.vertex, v_to);
                if seen.insert(key) {
                    new_half_edges.push(he.clone());
                }
            }
        }
        self.half_edges = new_half_edges;
    }
}
```

### 3. 테스트 코드 예시
```rust
#[test]
fn test_remove_duplicates() {
    let positions = vec![
        Point3D { x: 0.0, y: 0.0, z: 0.0 },
        Point3D { x: 1.0, y: 0.0, z: 0.0 },
        Point3D { x: 0.0, y: 1.0, z: 0.0 },
    ];
    // 같은 삼각형 두 번 추가
    let triangles = vec![[0, 1, 2], [0, 1, 2]];

    let mut mesh = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);
    assert_eq!(mesh.faces.len(), 2);

    mesh.remove_duplicate_faces();
    assert_eq!(mesh.faces.len(), 1);

    mesh.remove_duplicate_edges();
    // 중복 edge 제거 후 half_edges 수가 줄어듦
    assert!(mesh.half_edges.len() <= 3);
}
```


## 📐 요약
- 중복 Face 제거: 정점 인덱스 집합을 키로 중복 판정.
- 중복 Edge 제거: (u,v) 방향성 에지 키로 중복 판정.
- 테스트 코드: 중복 삼각형을 입력해 제거 기능 검증.

---

## 소스 코드
```rust
use crate::core::prelude::Point3D;

pub type Index = usize;

#[derive(Debug, Clone)]
pub struct HEVertex {
    pub position: Point3D,
    /// 이 정점에서 시작하는 half-edge 하나 (없을 수도 있음)
    pub half_edge: Option<Index>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct HEFace {
    /// 이 face 를 둘러싸는 half-edge 하나 (loop 순회 시작점)
    pub half_edge: Option<Index>,
}
```
```rust
#[derive(Debug, Clone)]
pub struct HalfEdge {
    /// 이 half-edge 의 시작 정점
    pub vertex: Index,

    /// 반대편 half-edge (없을 수도 있음 – open mesh)
    pub twin: Option<Index>,

    /// 같은 face 안에서의 다음 half-edge
    pub next: Option<Index>,
    /// 같은 face 안에서의 이전 half-edge
    pub prev: Option<Index>,

    /// 이 half-edge 가 속한 face
    pub face: Option<Index>,
}
```
```rust
#[derive(Debug, Default, Clone)]
pub struct HalfEdgeMesh {
    pub vertices: Vec<HEVertex>,
    pub faces: Vec<HEFace>,
    pub half_edges: Vec<HalfEdge>,
}
```
```rust
impl HalfEdgeMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, p: Point3D) -> Index {
        let idx = self.vertices.len();
        self.vertices.push(HEVertex {
            position: p,
            half_edge: None,
        });
        idx
    }

    pub fn add_face_from_triangle(
        &mut self,
        v0: Index,
        v1: Index,
        v2: Index,
    ) -> Index {
        let face_idx = self.faces.len();
        let base_he = self.half_edges.len();

        // half-edges 3개
        self.half_edges.push(HalfEdge {
            vertex: v0,
            twin: None,
            next: Some(base_he + 1),
            prev: Some(base_he + 2),
            face: Some(face_idx),
        });
        self.half_edges.push(HalfEdge {
            vertex: v1,
            twin: None,
            next: Some(base_he + 2),
            prev: Some(base_he + 0),
            face: Some(face_idx),
        });
        self.half_edges.push(HalfEdge {
            vertex: v2,
            twin: None,
            next: Some(base_he + 0),
            prev: Some(base_he + 1),
            face: Some(face_idx),
        });

        self.faces.push(HEFace {
            half_edge: Some(base_he),
        });

        // 정점 쪽에서 나가는 half_edge 초기 세팅 (비어 있으면 채움)
        for (v, he_i) in [(v0, base_he), (v1, base_he + 1), (v2, base_he + 2)] {
            if self.vertices[v].half_edge.is_none() {
                self.vertices[v].half_edge = Some(he_i);
            }
        }

        face_idx
    }
```
```rust
    /// 모든 삼각형 faces 목록에서 HalfEdgeMesh 생성 (twin 은 나중에 연결)
    pub fn from_triangle_indices(
        positions: &[Point3D],
        triangles: &[[u32; 3]],
    ) -> Self {
        let mut m = HalfEdgeMesh::new();

        for &p in positions {
            m.add_vertex(p);
        }

        for tri in triangles {
            m.add_face_from_triangle(
                tri[0] as usize,
                tri[1] as usize,
                tri[2] as usize,
            );
        }

        m.build_twins();

        m
    }
```
```rust
    /// (u,v) 와 (v,u)를 twin 으로 연결
    pub fn build_twins(&mut self) {
        use std::collections::HashMap;

        // -------------------------------
        // 1) immutable pass: half-edge 방향 정보만 수집
        // -------------------------------
        let mut directed_edges = Vec::with_capacity(self.half_edges.len());

        for (he_idx, he) in self.half_edges.iter().enumerate() {
            let v_from = he.vertex;

            let v_to = {
                let next = he.next.expect("triangle mesh must have next");
                self.half_edges[next].vertex
            };

            directed_edges.push((he_idx, v_from, v_to)); // (i, u, v)
        }

        // -------------------------------
        // 2) 해시맵 구축
        // -------------------------------
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();

        for &(he_idx, u, v) in &directed_edges {
            map.insert((u, v), he_idx);
        }

        // -------------------------------
        // 3) mutable pass: twin 채우기
        // -------------------------------
        for (he_idx, u, v) in directed_edges {
            if let Some(&opp_idx) = map.get(&(v, u)) {
                self.half_edges[he_idx].twin = Some(opp_idx);
            }
        }
    }
}
```
```rust
impl HalfEdgeMesh {
    pub fn remove_duplicate_faces(&mut self) {
        use std::collections::HashSet;
        let mut seen: HashSet<[usize; 3]> = HashSet::new();
        let mut new_faces = Vec::new();

        for face in &self.faces {
            if let Some(he_idx) = face.half_edge {
                let he0 = &self.half_edges[he_idx];
                let he1 = &self.half_edges[he0.next.unwrap()];
                let he2 = &self.half_edges[he1.next.unwrap()];
                let mut verts = [he0.vertex, he1.vertex, he2.vertex];
                verts.sort();
                if seen.insert(verts) {
                    new_faces.push(face.clone());
                }
            }
        }
        self.faces = new_faces;
        // half_edges는 실제 구현에서는 face와 함께 재구성 필요
    }
}
```
```rust
impl HalfEdgeMesh {
    pub fn remove_duplicate_edges(&mut self) {
        use std::collections::HashSet;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut new_half_edges = Vec::new();

        for he in &self.half_edges {
            if let Some(next_idx) = he.next {
                let v_to = self.half_edges[next_idx].vertex;
                let key = (he.vertex, v_to);
                if seen.insert(key) {
                    new_half_edges.push(he.clone());
                }
            }
        }
        self.half_edges = new_half_edges;
    }
}
```
```rust
pub fn on_extract_boundaries(mesh: &HalfEdgeMesh) -> Vec<(Index, Index)> {
    let mut boundaries = Vec::new();
    for he in &mesh.half_edges {
        if he.twin.is_none() {
            let v_from = he.vertex;
            let v_to = mesh.half_edges[he.next.unwrap()].vertex;
            boundaries.push((v_from, v_to));
        }
    }
    boundaries
}
```
```rust
pub fn on_split_face(mesh: &mut HalfEdgeMesh, face_idx: Index, v_a: Index, v_b: Index) {
    // 1. 새로운 half-edge 두 개 생성 (v_a→v_b, v_b→v_a)
    let he_ab = mesh.half_edges.len();
    let he_ba = he_ab + 1;

    mesh.half_edges.push(HalfEdge {
        vertex: v_a,
        twin: Some(he_ba),
        next: None,
        prev: None,
        face: Some(face_idx),
    });
    mesh.half_edges.push(HalfEdge {
        vertex: v_b,
        twin: Some(he_ab),
        next: None,
        prev: None,
        face: None, // 나중에 새 face에 연결
    });

    // 2. 기존 face 루프를 두 개로 나누고 새 face 생성
    let new_face_idx = mesh.faces.len();
    mesh.faces.push(HEFace { half_edge: Some(he_ba) });

    // 3. next/prev 업데이트 (실제 구현에서는 루프 탐색 필요)
}
```
```rust
pub fn on_collapse_edge(mesh: &mut HalfEdgeMesh, he_idx: Index) {
    let he = &mesh.half_edges[he_idx];
    let v_from = he.vertex;
    let v_to = mesh.half_edges[he.next.unwrap()].vertex;

    // 1. 두 정점을 하나로 병합 (예: v_from 위치로 통합)
    mesh.vertices[v_to].position = mesh.vertices[v_from].position;

    // 2. v_to를 참조하는 half-edge들을 v_from으로 업데이트
    for h in &mut mesh.half_edges {
        if h.vertex == v_to {
            h.vertex = v_from;
        }
    }

    // 3. 관련 face와 half-edge 정리 (실제 구현에서는 루프 검사 필요)
}
```

---

## 테스트 코드
```rust
#[cfg(test)]
mod tests_basic {
    use nurbslib::core::half_edge::HalfEdgeMesh;
    use nurbslib::core::prelude::Point3D;

    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
```
```rust
    /// 단일 삼각형: twin 이 없는 open mesh 검증
    #[test]
    fn single_triangle_topology() {
        // 정점 3개
        let positions = vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0)];
        let triangles = vec![[0u32, 1u32, 2u32]];

        let m = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);

        assert_eq!(m.vertices.len(), 3);
        assert_eq!(m.faces.len(), 1);
        assert_eq!(m.half_edges.len(), 3);

        // face 의 half_edge 를 따라 한 바퀴 돌 수 있어야 한다.
        let f0 = &m.faces[0];
        let he0_idx = f0.half_edge.expect("face must have a starting half-edge");
        let he0 = &m.half_edges[he0_idx];

        let he1_idx = he0.next.unwrap();
        let he2_idx = m.half_edges[he1_idx].next.unwrap();
        let back_idx = m.half_edges[he2_idx].next.unwrap();

        assert_eq!(back_idx, he0_idx, "half-edge loop must close");

        // twin 은 없는 open triangle
        assert!(m.half_edges[he0_idx].twin.is_none());
        assert!(m.half_edges[he1_idx].twin.is_none());
        assert!(m.half_edges[he2_idx].twin.is_none());
    }
```
```rust
    /// 두 삼각형으로 이루어진 사각형: 공유 edge 의 twin 관계 검증
    #[test]
    fn quad_two_triangles_twins() {
        // 정사각형을 두 개의 삼각형으로 나눔
        //
        // v2 ---- v3
        // |     / |
        // |   /   |
        // | /     |
        // v0 ---- v1
        //
        let positions = vec![
            p(0.0, 0.0, 0.0), // v0
            p(1.0, 0.0, 0.0), // v1
            p(0.0, 1.0, 0.0), // v2
            p(1.0, 1.0, 0.0), // v3
        ];

        let triangles = vec![
            [0u32, 1u32, 2u32], // 아래 삼각형
            [2u32, 1u32, 3u32], // 위 삼각형
        ];

        let m = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);

        assert_eq!(m.vertices.len(), 4);
        assert_eq!(m.faces.len(), 2);
        assert_eq!(m.half_edges.len(), 6);

        // 두 삼각형은 edge (1,2) 를 공유한다.
        // 그 edge 에 해당하는 half-edge 쌍의 twin 이 서로를 가리키는지 확인
        let mut shared_edges = Vec::new();

        for (he_idx, he) in m.half_edges.iter().enumerate() {
            let v_from = he.vertex;
            let v_to = {
                let next_idx = he.next.unwrap();
                m.half_edges[next_idx].vertex
            };

            // (1 -> 2) 또는 (2 -> 1) 이면 공유 엣지
            if (v_from == 1 && v_to == 2) || (v_from == 2 && v_to == 1) {
                shared_edges.push(he_idx);
            }
        }

        assert_eq!(
            shared_edges.len(),
            2,
            "shared edge should appear twice as half-edges"
        );

        let he_a = shared_edges[0];
        let he_b = shared_edges[1];

        // 쌍방 twin
        assert_eq!(m.half_edges[he_a].twin, Some(he_b));
        assert_eq!(m.half_edges[he_b].twin, Some(he_a));

        // 각 half-edge 의 face 는 서로 다른 face 여야 한다.
        let fa = m.half_edges[he_a].face.unwrap();
        let fb = m.half_edges[he_b].face.unwrap();
        assert_ne!(fa, fb, "two half-edges of shared edge must belong to different faces");
    }
}
```
```rust
#[cfg(test)]
mod tests_practical {
    use nurbslib::core::half_edge::HalfEdgeMesh;
    use nurbslib::core::prelude::Point3D;

    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    /// 정육면체 하나를 HalfEdgeMesh로 만들고,
    /// - 모든 half-edge 에 twin 이 있는지 (watertight)
    /// - 각 face 에서 한 바퀴 도는 loop 이 잘 닫히는지
    #[test]
    fn cube_halfedge_topology_is_watertight() {
        // 단위 정육면체 8개 정점
        //
        //      v7 -------- v6
        //     / |         / |
        //   v4 -------- v5  |
        //   |  |        |   |
        //   |  v3 ------|-- v2
        //   | /         |  /
        //   v0 -------- v1
        //
        let positions = vec![
            p(0.0, 0.0, 0.0), // v0
            p(1.0, 0.0, 0.0), // v1
            p(1.0, 1.0, 0.0), // v2
            p(0.0, 1.0, 0.0), // v3
            p(0.0, 0.0, 1.0), // v4
            p(1.0, 0.0, 1.0), // v5
            p(1.0, 1.0, 1.0), // v6
            p(0.0, 1.0, 1.0), // v7
        ];

        // 각 면을 두 삼각형으로 나눈 정육면체 (12 triangles)
        let triangles: Vec<[u32; 3]> = vec![
            // 아래(z=0) 면
            [0, 1, 2],
            [0, 2, 3],
            // 위(z=1) 면
            [4, 6, 5],
            [4, 7, 6],
            // 앞(y=0) 면
            [0, 5, 1],
            [0, 4, 5],
            // 뒤(y=1) 면
            [3, 2, 6],
            [3, 6, 7],
            // 왼(x=0) 면
            [0, 3, 7],
            [0, 7, 4],
            // 오른(x=1) 면
            [1, 5, 6],
            [1, 6, 2],
        ];

        let m = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);

        assert_eq!(m.vertices.len(), 8);
        assert_eq!(m.faces.len(), 12);
        assert_eq!(m.half_edges.len(), 12 * 3);

        // 1) 모든 half-edge 에 twin 이 있어야 하는 완전 폐체
        for (i, he) in m.half_edges.iter().enumerate() {
            assert!(
                he.twin.is_some(),
                "half-edge {} has no twin (non-manifold or boundary)",
                i
            );
        }

        // 2) 각 face 가 half-edge loop 로 잘 닫히는지
        for (fi, face) in m.faces.iter().enumerate() {
            let start_he = face
                .half_edge
                .unwrap_or_else(|| panic!("face {} has no starting half-edge", fi));

            let mut visited = 0;
            let mut current = start_he;

            loop {
                visited += 1;
                let next = m.half_edges[current]
                    .next
                    .unwrap_or_else(|| panic!("face {} has broken next pointer", fi));
                if next == start_he {
                    break;
                }
                if visited > 10 {
                    panic!("face {} loop too long or not closed", fi);
                }
                current = next;
            }

            // 정육면체의 각 면은 삼각형이므로 3번이면 돌아와야 한다.
            assert_eq!(
                visited, 3,
                "face {}: expected loop length 3, got {}",
                fi, visited
            );
        }
    }
```
```rust
    /// 하나의 정점을 기준으로 주변 half-edge/face ring 을 순회해 보는 예제
    /// (실제로는 mesh 분석/리메싱 등에 활용 가능한 패턴)
    #[test]
    fn around_vertex_ring_walk() {
        let positions = vec![
            p(0.0, 0.0, 0.0), // v0
            p(1.0, 0.0, 0.0), // v1
            p(1.0, 1.0, 0.0), // v2
            p(0.0, 1.0, 0.0), // v3
            p(0.0, 0.0, 1.0), // v4
            p(1.0, 0.0, 1.0), // v5
            p(1.0, 1.0, 1.0), // v6
            p(0.0, 1.0, 1.0), // v7
        ];
        let triangles: Vec<[u32; 3]> = vec![
            [0, 1, 2],
            [0, 2, 3],
            [4, 6, 5],
            [4, 7, 6],
            [0, 5, 1],
            [0, 4, 5],
            [3, 2, 6],
            [3, 6, 7],
            [0, 3, 7],
            [0, 7, 4],
            [1, 5, 6],
            [1, 6, 2],
        ];

        let m = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);

        // v0 주변의 face ring 을 순회해보자.
        let v0 = 0usize;
        let start_he = m.vertices[v0]
            .half_edge
            .expect("vertex 0 must have an outgoing half-edge");

        let mut current_he = start_he;
        let mut visited_faces = Vec::new();

        loop {
            let face_idx = m.half_edges[current_he]
                .face
                .expect("half-edge must belong to a face");
            visited_faces.push(face_idx);

            // 다음 half-edge:
            //   current_he 의 prev 의 twin 으로 넘어가면
            //   같은 정점 v0 를 공유하는 이웃 face 로 이동
            let prev_he = m.half_edges[current_he]
                .prev
                .expect("half-edge must have prev in triangle");
            let twin_he = m.half_edges[prev_he]
                .twin
                .expect("closed cube mesh must have twin");
            let next_around = twin_he;

            if next_around == start_he {
                break;
            }
            current_he = next_around;

            // 안전장치
            if visited_faces.len() > 16 {
                panic!("vertex ring walk does not close properly");
            }
        }

        // v0 는 cube 의 모서리이므로, 3개의 면이 만난다.
        // (하, 전, 좌) = 3 faces
        visited_faces.sort();
        visited_faces.dedup();
        assert_eq!(
            visited_faces.len(),
            6,
            "vertex 0 should have 3 incident faces, got {}",
            visited_faces.len()
        );
    }
}
```
```rust
#[cfg(test)]
mod tests_advanced {
    use nurbslib::core::half_edge::{on_extract_boundaries, HalfEdgeMesh};
    use nurbslib::core::prelude::Point3D;

    fn sample_triangle_mesh() -> HalfEdgeMesh {
        // 정점 3개 (삼각형 하나)
        let positions = vec![
            Point3D { x: 0.0, y: 0.0, z: 0.0 },
            Point3D { x: 1.0, y: 0.0, z: 0.0 },
            Point3D { x: 0.0, y: 1.0, z: 0.0 },
        ];
        let triangles = vec![[0, 1, 2]];

        HalfEdgeMesh::from_triangle_indices(&positions, &triangles)
    }
```
```rust
    #[test]
    fn test_add_vertex_and_face() {
        let mut mesh = HalfEdgeMesh::new();
        let v0 = mesh.add_vertex(Point3D { x: 0.0, y: 0.0, z: 0.0 });
        let v1 = mesh.add_vertex(Point3D { x: 1.0, y: 0.0, z: 0.0 });
        let v2 = mesh.add_vertex(Point3D { x: 0.0, y: 1.0, z: 0.0 });

        let f0 = mesh.add_face_from_triangle(v0, v1, v2);

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.half_edges.len(), 3);

        // face의 half_edge 루프가 닫혀 있는지 확인
        let he0 = mesh.faces[f0].half_edge.unwrap();
        let he1 = mesh.half_edges[he0].next.unwrap();
        let he2 = mesh.half_edges[he1].next.unwrap();
        assert_eq!(mesh.half_edges[he2].next.unwrap(), he0);
    }
```
```rust
    #[test]
    fn test_build_twins_on_two_triangles() {
        // 정점 4개, 삼각형 2개 (사각형을 두 삼각형으로 분할)
        let positions = vec![
            Point3D { x: 0.0, y: 0.0, z: 0.0 }, // v0
            Point3D { x: 1.0, y: 0.0, z: 0.0 }, // v1
            Point3D { x: 1.0, y: 1.0, z: 0.0 }, // v2
            Point3D { x: 0.0, y: 1.0, z: 0.0 }, // v3
        ];
        let triangles = vec![[0, 1, 2], [0, 2, 3]];

        let mesh = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);

        // twin 이 제대로 연결되었는지 확인
        let mut twin_count = 0;
        for he in &mesh.half_edges {
            if he.twin.is_some() {
                twin_count += 1;
            }
        }
        // 두 삼각형이 공유하는 에지 (0-2, 2-0) twin 연결됨
        assert!(twin_count >= 2);
    }
```
```rust
    #[test]
    fn test_boundary_extraction() {
        let mesh = sample_triangle_mesh();
        let boundaries = on_extract_boundaries(&mesh);

        // 삼각형 하나만 있으므로 모든 에지가 경계
        assert_eq!(boundaries.len(), 3);
        assert!(boundaries.contains(&(0, 1)));
        assert!(boundaries.contains(&(1, 2)));
        assert!(boundaries.contains(&(2, 0)));
    }
```
```rust
    #[test]
    fn test_remove_duplicates() {
        let positions = vec![
            Point3D { x: 0.0, y: 0.0, z: 0.0 },
            Point3D { x: 1.0, y: 0.0, z: 0.0 },
            Point3D { x: 0.0, y: 1.0, z: 0.0 },
        ];
        // 같은 삼각형 두 번 추가
        let triangles = vec![[0, 1, 2], [0, 1, 2]];

        let mut mesh = HalfEdgeMesh::from_triangle_indices(&positions, &triangles);
        assert_eq!(mesh.faces.len(), 2);

        mesh.remove_duplicate_faces();
        assert_eq!(mesh.faces.len(), 1);

        mesh.remove_duplicate_edges();
        // 중복 edge 제거 후 half_edges 수가 줄어듦
        assert!(mesh.half_edges.len() <= 3);
    }
}
```
