# MeshMerger

## 0. MeshMerger의 목적
- MeshMerger는 여러 개의 TessMesh(= NURBS face 하나의 tessellation 결과)를
하나의 GlobalMesh로 병합하는 도구다.
- 병합 과정에서 해결해야 하는 문제는 크게 4가지다:
  - 중복 정점 제거 (vertex weld)
  - T‑junction 제거 (fix T‑junctions)
  - 중복 삼각형 제거 (topological weld)
  - 삼각형 간의 경계 일관성 유지
- 이 네 가지는 서로 영향을 주기 때문에, 정확한 순서와 tolerance 정책이 필요하다.


## 1. 전체 파이프라인 구조
- MeshMerger의 전체 파이프라인은 다음 순서로 동작한다:
```
faces[] → merge_face_meshes()
        → weld_vertices (boundary-only)
        → fix_t_junctions
        → weld_vertices (full)
        → topological_weld
        → result
```

- 각 단계는 다음 의미를 가진다.


## 2. merge_face_meshes()
- 목적
  - 여러 TessMesh를 하나의 GlobalMesh로 합친다.
- 입력
  - faces: Vec<TessMesh>
- 출력
  - GlobalMesh(vertices, tris, face_id)
- 특징
  - 각 face의 vertex index는 base offset을 더해 고유하게 만든다.
  - face_id는 삼각형이 어느 face에서 왔는지 기록한다.
  - 이 단계에서는 정점 병합을 하지 않는다.


## 3. weld_vertices()
- 목적
  - 정점 위치가 eps 이내로 가까운 vertex들을 하나로 묶는다.
- 두 가지 모드
  - boundary-only = true
  - 경계(vertex valence == 1)만 weld
  - interior vertex는 절대 weld하지 않음
  - 이유: interior vertex는 다른 face와 공유될 수 있으므로
- boundary-only 단계에서 interior를 weld하면 T‑junction이 생길 수 있음
  - boundary-only = false
  - 모든 vertex를 weld
- T‑junction fix 이후에 전체를 정리하는 단계
- 알고리즘
  - Spatial Hash(Hash3)로 vertex를 bucket에 넣는다.
  - 같은 bucket + neighbor bucket(3×3×3)에서 거리 ≤ eps이면 union
  - cluster별로 평균 위치로 snap
- 중요한 설계 포인트
  - boundary-only 모드에서 boundary 판정은 vertex valence 기반
    - (삼각형에 1번만 등장하면 boundary vertex)
  - interior vertex끼리는 weld하지 않는다
    - test_weld_vertices_boundary_only가 이 의미를 검증한다.


## 4. fix_t_junctions()
- 목적
- T‑junction을 제거한다.
- T‑junction이란:
```
A ----- B
      |
      P   ← edge 중간에 vertex가 걸쳐 있는 상태
```

- 조건
  - vertex P가 boundary edge AB의 **중간(t ∈ (0,1))** 에 있고
  - 거리 ≤ eps이면 T‑junction
  - 단, endpoint 근처는 T‑junction이 아니다
    - distance(P, A) <= eps 또는 distance(P, B) <= eps이면 skip
    - test_fix_t_junction_ignore_near_endpoint가 이 의미를 검증


- 알고리즘
  - boundary edge 목록 생성
  - boundary vertex 후보를 edge에 투영
  - T‑junction이면 edge를 두 개로 split
  - 삼각형을 (A,P,C), (P,B,C)로 나눈다
  - split 후 edge 목록을 다시 계산하고 반복
  - guard_max(= t_split_passes) 만큼 반복
- 특징
  - fix_t_junctions는 vertex를 추가할 수 있다.
  - 따라서 이후 단계에서 다시 weld가 필요하다.

## 5. second weld (full weld)
- 목적
  - T‑junction split 후 생긴 vertex들을 다시 weld하여 정점 중복을 제거한다.
- 특징
  - boundary-only가 아니라 full weld
  - eps는 stitch_eps 그대로 사용
  - interior vertex도 weld 가능


## 6. topological_weld()
- 목적
  - 정점이 완전히 동일한 삼각형을 제거한다.
- 중복 판단 기준
  - vertex index를 정렬한 (a,b,c) tuple이 같으면 duplicate
  - face_id는 고려하지 않는다
    - 서로 다른 face에서 온 삼각형이라도 vertex weld 후 동일한 삼각형이면 하나만 남긴다
      - topological_weld_removes_duplicate_vertices_and_tris
      - topological_weld_removes_permuted_duplicate_tris
      - 이 두 테스트가 이 의미를 검증한다.
- 특징
- degenerate triangle(a==b 등)은 제거
- duplicate triangle은 제거
- vertex는 DSU로 cluster화하여 재배치


## 7. MeshMergerOptions 의미
### MeshMergerOptions
| Option Name               | Type  | Description                                                   |
|---------------------------|-------|---------------------------------------------------------------|
| stitch_eps                | f64   | Global tolerance. Used for weld, T-fix, and topo_weld.       |
| weld_only_boundary_first  | bool  | First weld only affects boundary vertices (valence == 1).     |
| second_weld_full          | bool  | After T-fix, perform a full weld on all vertices.             |
| enable_topological_weld   | bool  | Remove duplicate triangles in final stage. Default: false.    |
| t_split_passes            | usize | Number of T-junction split iterations (0 = disabled).         |

- 중요한 설계 결정
  - enable_topological_weld 기본값은 false
    - multiple_build_calls 테스트가 이 의미를 기대
    - 기본 옵션에서는 중복 삼각형을 제거하지 않는다
    - topo_weld 관련 테스트는 옵션에서 직접 true로 설정한다

## 8. 테스트들이 기대하는 의미 정리
### 8.1. test_weld_vertices_boundary_only
- boundary-only weld에서는 interior vertex는 weld되지 않아야 한다
- boundary vertex만 weld
### 8.2. test_fix_t_junction_ignore_near_endpoint
- endpoint 근처의 vertex는 T‑junction이 아니다
- split이 일어나지 않아야 한다
- T‑junction count도 0이어야 한다
### 8.3. test_multiple_build_calls
- 기본 옵션(default)에서는 topo_weld가 꺼져 있어야 한다
- 같은 삼각형을 가진 face를 두 번 추가하면 tris=2가 되어야 한다
### 8.4. topological_weld_removes_duplicate_vertices_and_tris
- topo_weld=true일 때
- 서로 다른 face에서 온 삼각형이라도 vertex weld 후 동일하면 1개만 남아야 한다
### 8.5. topological_weld_removes_permuted_duplicate_tris
- 삼각형 정점 순서가 달라도 동일 삼각형이면 1개만 남아야 한다

## 9. 전체 알고리즘 흐름 요약 (최종 버전)
```
faces[]  
  ↓ merge_face_meshes
GlobalMesh(vertices, tris, face_id)

  ↓ weld_vertices(eps, boundary_only=true)
경계 정점만 weld → interior는 그대로

  ↓ fix_t_junctions(eps)
T‑junction split → endpoint 근처는 skip

  ↓ weld_vertices(eps, boundary_only=false)
전체 weld → split 후 생긴 vertex 정리

  ↓ topological_weld(eps)  (옵션)
중복 삼각형 제거 → face_id 무시

  ↓ result
```


## 10. MeshMerger의 철학적 설계 원칙
- Tolerance는 하나(stitch_eps)로 통일
- weld / T‑fix / topo_weld 모두 같은 eps 사용
- eps 스케일링은 하지 않음
- boundary-only weld는 interior를 절대 건드리지 않는다
- interior weld는 T‑junction을 만들 수 있기 때문
- T‑junction split은 endpoint 근처는 무시한다
- numerical noise로 인한 불필요한 split 방지
- topological_weld는 face_id를 고려하지 않는다
- 진짜 중복 삼각형은 face가 달라도 제거해야 한다
- 단, 기본 옵션에서는 topo_weld를 꺼둔다
- build()는 faces를 누적해서 처리한다
- build()를 여러 번 호출해도 faces는 유지
- result만 새로 생성

## 11. 앞으로 유지보수할 때 기억해야 할 핵심
- weld → T‑fix → weld → topo_weld 순서는 절대 바꾸지 말 것
- boundary-only weld는 interior vertex를 건드리면 안 됨
- topo_weld는 face_id를 고려하지 말 것
- default 옵션에서 topo_weld는 꺼져 있어야 함
- T‑junction endpoint 처리 로직은 매우 중요
- build()는 faces를 초기화하지 않는다
- tolerance는 하나만 사용한다

## 12. 문서 끝
- 이 문서는 MeshMerger 전체 구조를 처음부터 다시 설계한 수준으로 정리한 문서.
- 이제 이 문서를 기준으로:
  - 새로운 기능 추가
  - tolerance 정책 변경
  - 성능 최적화
- 테스트 추가


---

🔍 왜 기본값이 true면 문제가 되나?

- **기본 옵션에서 topological_weld 를 false** 인 이유 기본 동작은 절대 삼각형을 삭제하면 안 된다

## 문제 1 — 기본 옵션에서 삼각형이 “몰래” 사라진다
- topological_weld = true일 때는:
  - 서로 다른 face에서 온 삼각형이라도
  - vertex weld 후 동일한 삼각형이 되면
  - 중복으로 판단되어 제거된다
- 즉, 사용자가 의도하지 않아도 삼각형이 사라진다.
- 이게 바로 test_multiple_build_calls에서 발생한 문제:
```rust
mm.add_face(f)
mm.build() → tri 1개

mm.add_face(f)
mm.build() → tri 2개가 되어야 하는데… 1개만 남음
```

- 왜냐면:
  - 두 번째 face의 삼각형이 첫 번째 face와 동일하다고 판단되어
  - topo_weld가 중복 제거해버렸기 때문
- 즉, 기본 옵션에서 데이터 손실이 발생한 것.

## 🔍 문제 2 — 대부분의 사용자는 **중복 삼각형 제거** 를 원하지 않는다
- MeshMerger의 기본 목적은:
  - 여러 face를 하나의 mesh로 합치는 것
  - geometry를 보존하는 것
  - 의도하지 않은 삭제를 하지 않는 것
- 그런데 topological_weld는:
  - geometry를 **정리(clean-up)** 하는 기능
  - 즉, 파괴적(destructive) 기능
- 이런 기능은 기본적으로 꺼져 있어야 한다.

## 🔍 문제 3 — topo_weld는 고급 기능이다
- topological_weld는 다음과 같은 상황에서만 필요하다:
  - CAD tessellation 결과가 중복 삼각형을 생성하는 경우
  - vertex weld 후 동일 삼각형이 여러 개 생기는 경우
  - mesh를 최종적으로 **정리(clean-up)** 하고 싶을 때
- 즉, 특수한 상황에서만 필요한 고급 기능이다.
- 기본 옵션에서 켜져 있으면:
  - 초보 사용자는 이유도 모른 채 삼각형이 사라짐
  - 디버깅이 어려워짐
  - 예측 불가능한 결과가 나옴

## 🔍 문제 4 — 테스트들이 서로 충돌했다
- 두 종류의 테스트가 있었다:
- A) topo_weld 관련 테스트
  - **중복 삼각형은 하나로 합쳐야 한다**
  - → topo_weld = true 필요
- B) multiple_build_calls 테스트
- **face 두 개 넣으면 삼각형 2개가 남아야 한다**
  - topo_weld = false 필요
- 즉, 테스트의 의도가 서로 다르다.
- 기본 동작은 “안전 모드”
  - 삼각형 삭제 없음
  - geometry 보존
- multiple_build_calls 테스트 통과
- ✔ topo_weld 관련 테스트 / 고급 사용
```rust
opt.enable_topological_weld = true;
```
- 중복 삼각형 제거
- vertex weld 후 동일 삼각형은 하나만 남김
- topological_weld_removes_* 테스트 통과

---

## 소스 코드
```rust
use crate::core::types::on_same_object;

#[derive(Debug, Clone, Copy)]
pub struct Index2D {
    pub i : usize,
    pub j : usize,
}

impl Default for Index2D {
    fn default() -> Self {
        Self {
            i: 0,
            j: 0,

        }
    }
}

impl Index2D {
    pub fn new(i : usize, j : usize) -> Self {
        Self {
            i,
            j,
        }
    }

    pub fn compare(&self, other : &Index2D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        0
    }

    pub fn compare_first(&self, other : &Index2D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        0
    }

    pub fn compare_second(&self, other : &Index2D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        0
    }
}
```
```rust
#[derive(Debug, Clone, Copy)]
pub struct Index3D {
    pub i : usize,
    pub j : usize,
    pub k : usize,
}

impl Index3D {
    pub fn new(i : usize, j : usize, k : usize) -> Self {
        Self {
            i,
            j,
            k
        }
    }

    pub fn compare(&self, other : &Index3D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        if self.k < other.k { return -1; }
        if self.k > other.k { return 1; }
        0
    }

    pub fn compare_first(&self, other : &Index3D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        0
    }

    pub fn compare_second(&self, other : &Index3D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        0
    }

    pub fn compare_third(&self, other : &Index3D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.k < other.k { return -1; }
        if self.k > other.k { return 1; }
        0
    }

    pub fn compare_first_second(&self, other : &Index3D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        0
    }
}

impl Default for Index3D {
    fn default() -> Self {
        Self{
            i: 0,
            j: 0,
            k: 0,
        }
    }
}
```
```rust
#[derive(Debug, Clone, Copy)]
pub struct Index4D {
    pub i : usize,
    pub j : usize,
    pub k : usize,
    pub l : usize,
}


impl Index4D {
    pub fn new(i : usize, j : usize, k : usize, l : usize) -> Self {
        Self {
            i,
            j,
            k,
            l
        }
    }

    pub fn compare(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        if self.k < other.k { return -1; }
        if self.k > other.k { return 1; }
        if self.l < other.l { return -1; }
        if self.l > other.l { return 1; }
        0
    }

    pub fn compare_first(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        0
    }

    pub fn compare_second(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        0
    }

    pub fn compare_third(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.k < other.k { return -1; }
        if self.k > other.k { return 1; }
        0
    }

    pub fn compare_fourth(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.l < other.l { return -1; }
        if self.l > other.l { return 1; }
        0
    }

    pub fn compare_first_second(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        0
    }

    pub fn compare_first_second_third(&self, other : &Index4D) -> isize {
        if on_same_object(self, other) { return 0; }
        if self.i < other.i { return -1; }
        if self.i > other.i { return 1; }
        if self.j < other.j { return -1; }
        if self.j > other.j { return 1; }
        if self.k < other.k { return -1; }
        if self.k > other.k { return 1; }
        0
    }
}

impl Default for Index4D {
    fn default() -> Self {
        Self{
            i: 0,
            j: 0,
            k: 0,
            l :0,
        }
    }
}
```
```rust
use crate::core::prelude::Point3D;

/// 로컬 face 단위 메쉬 (NURBS face 하나 tess 결과 같은 느낌)
#[derive(Debug, Clone)]
pub struct TriMesh {
    pub vertices: Vec<Point3D>,
    pub tris: Vec<[u32; 3]>, // (i,j,k)
}

impl TriMesh {
    pub fn new(vertices: Vec<Point3D>, tris: Vec<[u32; 3]>) -> Self {
        Self { vertices, tris }
    }
}
```
```rust
/// 여러 TessMesh를 합친 글로벌 메쉬
#[derive(Debug, Clone)]
pub struct MergedMesh {
    pub vertices: Vec<Point3D>,
    pub tris: Vec<[u32; 3]>,
    /// Optional: 각 tri가 어느 face(TessMesh)에서 왔는지 기록
    pub face_id: Vec<i32>,
}
```
```rust
impl MergedMesh {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            tris: Vec::new(),
            face_id: Vec::new(),
        }
    }
}
```
```rust
/// 메쉬 병합 옵션 (C++ ON_MeshMerger::Options 대응)
#[derive(Debug, Clone)]
pub struct MeshMergerOptions {
    pub eps: f64,
    pub weld_boundary_first: bool,
    pub weld_full_after_tfix: bool,
    pub topo_clean: bool,
    pub tfix_passes: usize,
}

impl Default for MeshMergerOptions {
    fn default() -> Self {
        Self {
            eps: 1e-3,
            weld_boundary_first: true,
            weld_full_after_tfix: true,
            topo_clean: false,
            tfix_passes: 8,
        }
    }
}
```
```rust
#[derive(Clone, Debug)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        Self {
            parent,
            rank: vec![0u8; n],
        }
    }

    #[inline]
    fn find(&mut self, x: usize) -> usize {
        // path compression
        let mut x0 = x;
        while self.parent[x0] != x0 {
            x0 = self.parent[x0];
        }
        let root = x0;
        let mut x1 = x;
        while self.parent[x1] != x1 {
            let p = self.parent[x1];
            self.parent[x1] = root;
            x1 = p;
        }
        root
    }

    #[inline]
    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        let (rka, rkb) = (self.rank[ra], self.rank[rb]);
        if rka < rkb {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        if rka == rkb {
            self.rank[ra] = self.rank[ra].saturating_add(1);
        }
    }
}
```
```rust
/// 메쉬 병합기 (C++ ON_MeshMerger 대응)
pub struct MeshMergerEngine {
    options: MeshMergerOptions,
    faces: Vec<TriMesh>,
    result: Option<MergedMesh>,
}
```
```rust
impl MeshMergerEngine {
    pub fn new(options: MeshMergerOptions) -> Self {
        Self {
            options,
            faces: Vec::new(),
            result: None,
        }
    }

    pub fn clear(&mut self) {
        self.faces.clear();
        self.result = None;
    }

    pub fn options(&self) -> &MeshMergerOptions {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut MeshMergerOptions {
        &mut self.options
    }

    pub fn add_mesh(&mut self, face: TriMesh) {
        self.faces.push(face);
    }

    pub fn add_meshes<I>(&mut self, faces: I)
    where
        I: IntoIterator<Item =TriMesh>,
    {
        self.faces.extend(faces);
    }

    /// 전체 파이프라인 실행.
    /// - faces가 비어 있으면 false
    /// - 성공하면 result()에서 GlobalMesh를 가져올 수 있음
    pub fn build(&mut self) -> bool {
        let g = self.build_impl();
        self.result = g;
        self.result.is_some()
    }

    /// 마지막 build 결과
    pub fn result(&self) -> Option<&MergedMesh> {
        self.result.as_ref()
    }

    /// one-shot static merge
    pub fn merge(faces: Vec<TriMesh>, options: MeshMergerOptions) -> MergedMesh {
        let mut mm = MeshMergerEngine::new(options);
        mm.add_meshes(faces);
        mm.build();
        mm.result.expect("MeshMerger::build() returned None")
    }

    /// 디버그용: 현재 result에 대해 boundary edge 개수
    pub fn count_boundary_edges(&self, _eps: Option<f64>) -> usize {
        if let Some(g) = &self.result {
            Self::count_boundary_edges_impl(g)
        } else {
            0
        }
    }

    /// 디버그용: 현재 result에 대해 T-junction 개수
    pub fn count_t_junctions(&self, eps: f64) -> usize {
        if let Some(g) = &self.result {
            Self::count_t_junctions_impl(g, eps)
        } else {
            0
        }
    }

    // ----------------- 내부 파이프라인 구현 스켈레톤 -----------------

    fn build_impl(&self) -> Option<MergedMesh> {
        if self.faces.is_empty() {
            return None;
        }

        // 1. 모든 TessMesh → 하나의 GlobalMesh로 합치기
        let mut g = self.merge_meshes();

        let eps = self.options.eps;

        // 2. 1차 weld (경계만)
        if self.options.weld_boundary_first {
            self.weld_vertices(&mut g, eps, true);
        }

        // 3. T-junction fix
        if self.options.tfix_passes > 0 {
            self.fix_t_junctions(&mut g, eps, self.options.tfix_passes);
        }

        // 4. 2차 weld (전체)
        if self.options.weld_full_after_tfix {
            // 여기서 eps를 0.5배로 줄 것인지 말 것인지는 정책 문제인데,
            // 테스트/직관에 더 맞게 "동일 eps"를 쓰는 쪽을 추천.
            self.weld_vertices(&mut g, eps, false);
        }

        // 5. topological weld
        if self.options.topo_clean {
            // 마찬가지로 eps 통일: stitch_eps와 같은 기준 사용
            self.topological_weld(&mut g, eps);
        }

        Some(g)
    }


    /// C++ MergeFaceMeshes 대응
    fn merge_meshes(&self) -> MergedMesh {
        let mut g = MergedMesh::empty();

        let mut total_v = 0usize;
        let mut total_t = 0usize;
        for m in &self.faces {
            total_v += m.vertices.len();
            total_t += m.tris.len();
        }

        g.vertices.reserve(total_v);
        g.tris.reserve(total_t);
        g.face_id.reserve(total_t);

        let mut base: u32 = 0;
        let mut face_idx: i32 = 0;

        for m in &self.faces {
            g.vertices.extend_from_slice(&m.vertices);
            for tri in &m.tris {
                g.tris.push([
                    base + tri[0],
                    base + tri[1],
                    base + tri[2],
                ]);
                g.face_id.push(face_idx);
            }
            base += m.vertices.len() as u32;
            face_idx += 1;
        }

        g
    }

    /// edge → 사용 횟수 카운트
    fn build_edge_count(g: &MergedMesh) -> std::collections::HashMap<Edge, usize> {
        use std::collections::HashMap;
        let mut cnt: HashMap<Edge, usize> = HashMap::new();
        for t in &g.tris {
            for &(u, v) in &[(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                let e = Edge::new(u, v);
                *cnt.entry(e).or_insert(0) += 1;
            }
        }
        cnt
    }

    #[inline]
    fn is_boundary_edge(
        cnt: &std::collections::HashMap<Edge, usize>,
        a: u32,
        b: u32,
    ) -> bool {
        let e = Edge::new(a, b);
        match cnt.get(&e) {
            None => true,
            Some(&c) => c == 1,
        }
    }

    pub fn weld_vertices(&self, g: &mut MergedMesh, eps: f64, only_boundary: bool) {
        let n = g.vertices.len();
        if n == 0 {
            return;
        }

        // -----------------------------
        // 1) boundary vertex mask
        // -----------------------------
        let mut use_mask = vec![true; n];
        if only_boundary {
            // vertex valence(삼각형 개수) 기반으로 boundary 판단
            let mut valence = vec![0usize; n];
            for t in &g.tris {
                valence[t[0] as usize] += 1;
                valence[t[1] as usize] += 1;
                valence[t[2] as usize] += 1;
            }

            // valence == 1 인 vertex만 boundary 로 취급
            for i in 0..n {
                use_mask[i] = valence[i] == 1;
            }
        }


        // -----------------------------
        // 2) spatial hash buckets
        // -----------------------------
        use std::collections::HashMap;

        let hash = Hash3::new(eps * 1.5);
        let mut buckets: HashMap<Hash3Key, Vec<usize>> = HashMap::new();

        for (i, p) in g.vertices.iter().enumerate() {
            if !use_mask[i] {
                continue;
            }
            let key = hash.key(p);
            buckets.entry(key).or_default().push(i);
        }

        // -----------------------------
        // 3) union-find
        // -----------------------------
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut [usize], x: usize) -> usize {
            let mut r = x;
            while parent[r] != r {
                let p = parent[r];
                parent[r] = parent[p];
                r = parent[r];
            }
            r
        }

        let unite = |parent: &mut [usize], a: usize, b: usize| {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                parent[rb] = ra;
            }
        };

        // -----------------------------
        // 4) same bucket weld
        // -----------------------------
        for ids in buckets.values() {
            for a in 0..ids.len() {
                for b in (a + 1)..ids.len() {
                    let ia = ids[a];
                    let ib = ids[b];
                    if !use_mask[ia] || !use_mask[ib] {
                        continue;
                    }
                    if g.vertices[ia].distance(&g.vertices[ib]) <= eps {
                        unite(&mut parent, ia, ib);
                    }
                }
            }
        }

        // -----------------------------
        // 5) neighbor bucket weld
        // -----------------------------
        // Rust에서는 bucket iteration 중 neighbor 접근을 위해 key list를 따로 복사
        let bucket_keys: Vec<Hash3Key> = buckets.keys().copied().collect();

        for key in &bucket_keys {
            let Some(ids) = buckets.get(key) else { continue };

            for nb_key in hash.neighbors(*key) {
                if let Some(nb_ids) = buckets.get(&nb_key) {
                    for &ia in ids {
                        if !use_mask[ia] {
                            continue;
                        }
                        for &ib in nb_ids {
                            if !use_mask[ib] {
                                continue;
                            }
                            if g.vertices[ia].distance(&g.vertices[ib]) <= eps {
                                unite(&mut parent, ia, ib);
                            }
                        }
                    }
                }
            }
        }

        // -----------------------------
        // 6) cluster → 평균 위치 snap
        // -----------------------------
        use std::collections::HashMap as Map2;

        let mut clusters: Map2<usize, Vec<usize>> = Map2::new();

        for i in 0..n {
            if !use_mask[i] {
                continue;
            }
            let r = find(&mut parent, i);
            clusters.entry(r).or_default().push(i);
        }

        for (_root, ids) in clusters {
            if ids.is_empty() {
                continue;
            }

            let mut avg = Point3D::new(0.0, 0.0, 0.0);
            for &i in &ids {
                avg.x += g.vertices[i].x;
                avg.y += g.vertices[i].y;
                avg.z += g.vertices[i].z;
            }
            let inv = 1.0 / (ids.len() as f64);
            avg.x *= inv;
            avg.y *= inv;
            avg.z *= inv;

            for &i in &ids {
                g.vertices[i] = avg;
            }
        }
    }
```
```rust
    #[inline]
    fn seg_point_dist(a: Point3D, b: Point3D, p: Point3D) -> (f64, f64) {
        let seg = Segment3D::new(a, b);

        // t ∈ [0,1]
        let t = seg.closest_param_to(p);

        // 거리
        let dist = seg.distance_to_point(p);

        (dist, t)
    }
```
```rust
    fn fix_t_junctions(&self, g: &mut MergedMesh, eps: f64, guard_max: usize) {
        if g.vertices.is_empty() || g.tris.is_empty() {
            return;
        }

        #[derive(Clone, Copy, Debug)]
        struct ERef {
            a: u32,
            b: u32,
            t_idx: usize,
        } // a<b

        // 초기 boundary edge set 구성
        let mut cnt = Self::build_edge_count(g);
        let mut bedges: Vec<ERef> = Vec::with_capacity(g.tris.len());

        let rebuild_bedges = |g: &MergedMesh,
                              cnt: &std::collections::HashMap<Edge, usize>,
                              bedges: &mut Vec<ERef>| {
            bedges.clear();
            for (ti, t) in g.tris.iter().enumerate() {
                for &(u, v) in &[(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                    if Self::is_boundary_edge(cnt, u, v) {
                        let (a, b) = if u < v { (u, v) } else { (v, u) };
                        bedges.push(ERef { a, b, t_idx: ti });
                    }
                }
            }
        };

        rebuild_bedges(g, &cnt, &mut bedges);

        // “초기 boundary vertex”만 대상으로 삼는다
        let mut is_bv = vec![false; g.vertices.len()];
        for e in &bedges {
            let a = e.a as usize;
            let b = e.b as usize;
            if a < is_bv.len() {
                is_bv[a] = true;
            }
            if b < is_bv.len() {
                is_bv[b] = true;
            }
        }

        let mut changed = true;
        let mut guard = 0usize;

        while changed && guard < guard_max {
            changed = false;
            guard += 1;

            // pid: boundary vertex 후보
            'outer: for pid in 0..g.vertices.len() {
                if !is_bv[pid] {
                    continue;
                }
                let p = g.vertices[pid];

                for ei in 0..bedges.len() {
                    let e = bedges[ei];
                    if pid as u32 == e.a || pid as u32 == e.b {
                        continue;
                    }
                    if e.t_idx >= g.tris.len() {
                        continue;
                    }

                    let a = e.a as usize;
                    let b = e.b as usize;
                    if a >= g.vertices.len() || b >= g.vertices.len() {
                        continue;
                    }

                    let (d, tseg) = Self::seg_point_dist(g.vertices[a], g.vertices[b], p);
                    if d > eps {
                        continue;
                    }
                    if tseg <= 1.0e-6 || tseg >= 1.0 - 1.0e-6 {
                        continue; // edge의 양 끝에 너무 가까운 경우는 무시
                    }

                    // 현재 tri가 이 edge를 실제로 가지고 있는지 확인하고 반대 꼭짓점(c) 찾기
                    let tri = g.tris[e.t_idx];
                    let (ua, ub) = (e.a, e.b);
                    let mut c: Option<u32> = None;

                    if (tri[0] == ua && tri[1] == ub) || (tri[0] == ub && tri[1] == ua) {
                        c = Some(tri[2]);
                    } else if (tri[1] == ua && tri[2] == ub) || (tri[1] == ub && tri[2] == ua) {
                        c = Some(tri[0]);
                    } else if (tri[2] == ua && tri[0] == ub) || (tri[2] == ub && tri[0] == ua) {
                        c = Some(tri[1]);
                    }

                    let Some(c) = c else { continue };

                    // pid를 edge 위로 snap
                    let qa = g.vertices[a];
                    let qb = g.vertices[b];
                    g.vertices[pid] = qa + (qb - qa) * tseg;

                    // 삼각형 split: (a,pid,c) 와 (pid,b,c)
                    let pid_u = pid as u32;
                    g.tris[e.t_idx] = [ua, pid_u, c];
                    g.tris.push([pid_u, ub, c]);

                    // face_id 복사
                    if !g.face_id.is_empty() {
                        let fid = g.face_id.get(e.t_idx).copied().unwrap_or(-1);
                        g.face_id.push(fid);
                    }

                    // edge 테이블 다시 만들어서 루프 처음부터 재시작
                    cnt = Self::build_edge_count(g);
                    rebuild_bedges(g, &cnt, &mut bedges);
                    changed = true;
                    break 'outer;
                }
            }
        }
    }
```
```rust
    fn topological_weld(&self, g: &mut MergedMesh, eps: f64) {
        if g.vertices.is_empty() || g.tris.is_empty() {
            return;
        }

        use std::collections::{HashMap, HashSet};

        // eprintln!("=== [topological_weld] start ===");
        // eprintln!("vertex count = {}", g.vertices.len());
        // eprintln!("tri count    = {}", g.tris.len());

        // 1) spatial buckets
        let h = Hash3::new(eps);
        let mut buckets: HashMap<Hash3Key, Vec<usize>> = HashMap::new();
        buckets.reserve(g.vertices.len());

        for (i, p) in g.vertices.iter().enumerate() {
            let key = h.key(p);
            //eprintln!("vertex {} -> bucket {:?}", i, key);
            buckets.entry(key).or_default().push(i);
        }

        // 2) union-find
        let mut dsu = Dsu::new(g.vertices.len());

        for (k, ids) in buckets.iter() {
            //eprintln!("bucket {:?} has {:?}", k, ids);

            // same bucket
            for a in 0..ids.len() {
                for b in (a + 1)..ids.len() {
                    let ia = ids[a];
                    let ib = ids[b];
                    let d2 = Point3D::distance_squared_point(&g.vertices[ia], &g.vertices[ib]);
                    if d2 <= eps * eps {
                        //eprintln!("  UNION same-bucket: {} <-> {} (d2={})", ia, ib, d2);
                        dsu.union(ia, ib);
                    }
                }
            }

            // neighbor buckets
            for kk in h.neighbors(*k) {
                if let Some(nids) = buckets.get(&kk) {
                    //eprintln!("  neighbor bucket {:?} -> {:?}", kk, nids);
                    for &ia in ids {
                        for &ib in nids {
                            let d2 = Point3D::distance_squared_point(&g.vertices[ia], &g.vertices[ib]);
                            if d2 <= eps * eps {
                                //eprintln!("  UNION neighbor: {} <-> {} (d2={})", ia, ib, d2);
                                dsu.union(ia, ib);
                            }
                        }
                    }
                }
            }
        }

        // 3) representative → new index
        let mut rep_to_new: HashMap<usize, u32> = HashMap::new();
        let mut new_vertices: Vec<Point3D> = Vec::with_capacity(g.vertices.len());
        let mut map: Vec<u32> = vec![0; g.vertices.len()];

        //eprintln!("=== DSU cluster mapping ===");
        for i in 0..g.vertices.len() {
            let r = dsu.find(i);
            //eprintln!("vertex {} -> root {}", i, r);

            if let Some(&ni) = rep_to_new.get(&r) {
                map[i] = ni;
            } else {
                let ni = new_vertices.len() as u32;
                rep_to_new.insert(r, ni);
                new_vertices.push(g.vertices[r]);
                map[i] = ni;
            }
        }

        //eprintln!("map = {:?}", map);

        // 4) triangle remap + degenerate 제거 + duplicate 제거
        let mut new_tris: Vec<[u32; 3]> = Vec::with_capacity(g.tris.len());
        let mut new_face_id: Vec<i32> = Vec::with_capacity(g.face_id.len());
        let mut seen: HashSet<(u32, u32, u32)> = HashSet::with_capacity(g.tris.len() * 2);



        //eprintln!("=== triangle remap ===");
        for (ti, t) in g.tris.iter().enumerate() {
            let a = map[t[0] as usize];
            let b = map[t[1] as usize];
            let c = map[t[2] as usize];

            //eprintln!("tri {}: {:?} -> [{},{},{}]", ti, t, a, b, c);

            // degenerate tri 제거
            if a == b || b == c || c == a {
                //eprintln!("  -> degenerate, removed");
                continue;
            }

            // canonical key
            let mut x = [a, b, c];
            x.sort_unstable();
            let fid = *g.face_id.get(ti).unwrap_or(&-1);
            let key = (x[0], x[1], x[2]);

            if !seen.insert(key) {
                // eprintln!("  -> duplicate, removed");
                continue;
            }

            //eprintln!("  -> kept");
            new_tris.push([a, b, c]);
            if !g.face_id.is_empty() {
                let fid = *g.face_id.get(ti).unwrap_or(&-1);
                new_face_id.push(fid);

            }
        }

        g.vertices = new_vertices;
        g.tris = new_tris;
        g.face_id = new_face_id;

        // eprintln!("=== [topological_weld] end ===");
        // eprintln!("final vertex count = {}", g.vertices.len());
        // eprintln!("final tri count    = {}", g.tris.len());
    }

    fn count_boundary_edges_impl(g: &MergedMesh) -> usize {
        let cnt = Self::build_edge_count(g);
        let mut n = 0usize;
        for t in &g.tris {
            for &(u, v) in &[(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                if Self::is_boundary_edge(&cnt, u, v) {
                    n += 1;
                }
            }
        }
        n
    }
```
```rust
    pub fn count_t_junctions_impl(g: &MergedMesh, eps: f64) -> usize {
        if g.vertices.is_empty() || g.tris.is_empty() {
            return 0;
        }

        use std::collections::HashMap;

        // 1) 모든 edge를 세고, boundary edge만 추출
        let mut edge_count: HashMap<(u32, u32), u32> = HashMap::new();

        for t in &g.tris {
            let e = [
                (t[0], t[1]),
                (t[1], t[2]),
                (t[2], t[0]),
            ];

            for &(a, b) in &e {
                let key = if a < b { (a, b) } else { (b, a) };
                *edge_count.entry(key).or_insert(0) += 1;
            }
        }

        let mut boundary_edges = Vec::new();
        for (&(a, b), &cnt) in &edge_count {
            if cnt == 1 {
                boundary_edges.push((a, b));
            }
        }

        if boundary_edges.is_empty() {
            return 0;
        }

        let mut count = 0;
        let eps2 = eps * eps;

        // 2) 모든 vertex를 후보로 본다 (boundary vertex만 제한하지 않음)
        for v_idx in 0..g.vertices.len() {
            let p = &g.vertices[v_idx];

            for &(a, b) in &boundary_edges {
                // edge endpoint는 T‑junction 후보에서 제외
                if v_idx as u32 == a || v_idx as u32 == b {
                    continue;
                }

                let pa = &g.vertices[a as usize];
                let pb = &g.vertices[b as usize];

                let ab = *pb - *pa;
                let ap = *p - *pa;

                let ab_len2 = ab.length_squared();
                if ab_len2 == 0.0 {
                    continue;
                }

                let t = ap.dot(&ab) / ab_len2;

                if t <= 0.0 || t >= 1.0 {
                    continue;
                }

                let proj = *pa + ab * t;
                let d2 = Point3D::distance_squared_point(p, &proj);

                // endpoint 근처는 T‑junction이 아님
                let pa_d2 = Point3D::distance_squared_point(p, pa);
                let pb_d2 = Point3D::distance_squared_point(p, pb);
                if pa_d2 <= eps2 || pb_d2 <= eps2 {
                    continue;
                }


                if d2 <= eps2 {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    pub fn from_mesh(&mut self, mesh: Mesh) {
        let (v, t) = on_mesh_to_tri_list(&mesh);
        self.add_mesh(TriMesh { vertices: v, tris: t });
    }

    pub fn to_mesh(&self) -> Option<Mesh> {
        self.result.as_ref().map(|g| on_tri_list_to_mesh(g.vertices.clone(), g.tris.clone()))
    }

}
```
```rust
// ----------------- 보조 타입들 -----------------

use std::hash::{Hash, Hasher};
use crate::core::mesh::{on_mesh_to_tri_list, on_tri_list_to_mesh, Mesh};
use crate::core::point_ops::PointOps;
use crate::core::segment3d::Segment3D;

/// 무향 edge (a<b)
#[derive(Clone, Copy, Debug, Eq)]
struct Edge {
    a: u32,
    b: u32,
}

impl Edge {
    #[inline]
    fn new(i: u32, j: u32) -> Self {
        if i < j {
            Self { a: i, b: j }
        } else {
            Self { a: j, b: i }
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}
```
```rust
/// 3D 셀 해시 (Weld / TopologicalWeld, T-junction 등에서 사용 예정)
#[derive(Clone, Copy, Debug)]
struct Hash3 {
    cell: f64,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Hash3Key {
    x: i64,
    y: i64,
    z: i64,
}

impl Hash3 {
    fn new(cell: f64) -> Self {
        Self { cell }
    }

    #[inline]
    fn key(&self, p: &Point3D) -> Hash3Key {
        let kx = (p.x / self.cell).floor();
        let ky = (p.y / self.cell).floor();
        let kz = (p.z / self.cell).floor();

        // eprintln!(
        //     "[Hash3.key] p=({:.6},{:.6},{:.6}), cell={}, raw=({:.6},{:.6},{:.6}), floor=({},{},{})",
        //     p.x, p.y, p.z,
        //     self.cell,
        //     p.x / self.cell, p.y / self.cell, p.z / self.cell,
        //     kx, ky, kz
        // );

        Hash3Key {
            x: kx as i64,
            y: ky as i64,
            z: kz as i64,
        }

    }

    #[inline]
    fn neighbors(&self, k: Hash3Key) -> impl Iterator<Item = Hash3Key> {
        (-1..=1).flat_map(move |dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).map(move |dz| Hash3Key {
                    x: k.x + dx,
                    y: k.y + dy,
                    z: k.z + dz,
                })
            })
        })
    }
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod test {
    use nurbslib::core::mesh_merger::{MergedMesh, MeshMergerEngine, MeshMergerOptions, TriMesh};
    use nurbslib::core::prelude::Point3D;

    #[test]
    fn test_empty_merger() {
        let mut mm = MeshMergerEngine::new(MeshMergerOptions::default());
        assert!(!mm.build());
        assert!(mm.result().is_none());
    }
```
```rust
    #[test]
    fn test_single_face_merge() {
        let verts = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        ];
        let tris = vec![[0, 1, 2]];

        let face = TriMesh::new(verts.clone(), tris.clone());

        let mut mm = MeshMergerEngine::new(MeshMergerOptions::default());
        mm.add_mesh(face);

        assert!(mm.build());
        let g = mm.result().unwrap();

        assert_eq!(g.vertices.len(), 3);
        assert_eq!(g.tris.len(), 1);
        assert_eq!(g.tris[0], [0, 1, 2]);
        assert_eq!(g.face_id[0], 0);
    }
```
```rust
    #[test]
    fn test_two_face_merge() {
        // 첫 번째 face
        let f1 = TriMesh::new(
            vec![
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(1.0, 0.0, 0.0),
                Point3D::new(0.0, 1.0, 0.0),
            ],
            vec![[0, 1, 2]],
        );

        // 두 번째 face
        let f2 = TriMesh::new(
            vec![
                Point3D::new(10.0, 0.0, 0.0),
                Point3D::new(11.0, 0.0, 0.0),
                Point3D::new(10.0, 1.0, 0.0),
            ],
            vec![[0, 1, 2]],
        );

        let mut mm = MeshMergerEngine::new(MeshMergerOptions::default());
        mm.add_mesh(f1);
        mm.add_mesh(f2);

        assert!(mm.build());
        let g = mm.result().unwrap();

        // vertex count = 3 + 3
        assert_eq!(g.vertices.len(), 6);

        // triangle count = 1 + 1
        assert_eq!(g.tris.len(), 2);

        // 두 번째 face는 offset 3이 되어야 함
        assert_eq!(g.tris[1], [3, 4, 5]);

        // face_id도 올바르게 기록
        assert_eq!(g.face_id, vec![0, 1]);
    }
```
```rust
    #[test]
    fn test_boundary_edges() {
        // 사각형을 두 삼각형으로 구성
        let verts = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        ];

        let tris = vec![
            [0, 1, 2],
            [0, 2, 3],
        ];

        let face = TriMesh::new(verts, tris);

        let mut mm = MeshMergerEngine::new(MeshMergerOptions::default());
        mm.add_mesh(face);

        assert!(mm.build());
        let g = mm.result().unwrap();

        // 사각형의 boundary edge는 4개
        let be = mm.count_boundary_edges(None);
        assert_eq!(be, 4);
    }
```
```rust
    #[test]
    fn test_multiple_build_calls() {
        let mut mm = MeshMergerEngine::new(MeshMergerOptions::default());

        let f = TriMesh::new(
            vec![
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(1.0, 0.0, 0.0),
                Point3D::new(0.0, 1.0, 0.0),
            ],
            vec![[0, 1, 2]],
        );

        mm.add_mesh(f.clone());
        assert!(mm.build());
        assert!(mm.result().is_some());

        // 다시 face 추가
        mm.add_mesh(f);
        assert!(mm.build());
        assert_eq!(mm.result().unwrap().tris.len(), 2);
    }
```
```rust
    #[test]
    fn test_clear() {
        let mut mm = MeshMergerEngine::new(MeshMergerOptions::default());

        mm.add_mesh(TriMesh::new(
            vec![Point3D::new(0.0, 0.0, 0.0)],
            vec![[0, 0, 0]],
        ));

        assert!(mm.build());
        assert!(mm.result().is_some());

        mm.clear();
        assert!(mm.result().is_none());
        assert!(!mm.build()); // faces가 없으므로 false
    }
```
```rust
    #[test]
    fn test_weld_vertices_basic() {
        let mut g = MergedMesh {
            vertices: vec![
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(0.0004, 0.0, 0.0), // eps=0.001 이내 → weld 대상
                Point3D::new(1.0, 0.0, 0.0),    // 멀리 떨어진 점
            ],
            tris: vec![[0, 1, 2]],
            face_id: vec![0],
        };

        let merger = MeshMergerEngine::new(MeshMergerOptions {
            eps: 0.001,
            weld_boundary_first: false,
            weld_full_after_tfix: false,
            topo_clean: false,
            tfix_passes: 0,
        });

        merger.weld_vertices(&mut g, 0.001, false);

        // 0번과 1번은 weld → 평균값으로 snap
        let avg = Point3D::new(0.0002, 0.0, 0.0);


        println!("avg {:?}", avg);
        println!("g.vertices[0] {:?}", g.vertices[0]);
        println!("g.vertices[1] {:?}", g.vertices[1]);
        println!("g.vertices[2] {:?}", g.vertices[2]);

        assert!((g.vertices[0].x - avg.x).abs() < 1e-12);
        assert!((g.vertices[1].x - avg.x).abs() < 1e-12);

        // 2번은 그대로
        assert_eq!(g.vertices[2], Point3D::new(1.0, 0.0, 0.0));
    }
```
```rust
    #[test]
    fn test_weld_vertices_no_merge_when_far() {
        let mut g = MergedMesh {
            vertices: vec![
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(0.01, 0.0, 0.0), // eps=0.001 보다 멀다 → weld 안 됨
            ],
            tris: vec![[0, 1, 1]],
            face_id: vec![0],
        };

        let merger = MeshMergerEngine::new(MeshMergerOptions::default());
        merger.weld_vertices(&mut g, 0.001, false);

        assert_eq!(g.vertices[0], Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(g.vertices[1], Point3D::new(0.01, 0.0, 0.0));
    }
```
```rust
    #[test]
    fn test_weld_vertices_boundary_only() {
        // 삼각형 2개 → 가운데 점은 boundary가 아니다
        //
        //   0 ----- 1 ----- 2
        //
        let mut g = MergedMesh {
            vertices: vec![
                Point3D::new(0.0, 0.0, 0.0),   // boundary
                Point3D::new(0.5, 0.0, 0.0),   // interior (shared)
                Point3D::new(1.0, 0.0, 0.0),   // boundary
                Point3D::new(0.5004, 0.0, 0.0) // interior duplicate
            ],
            tris: vec![
                [0, 1, 2],
                [1, 3, 2],
            ],
            face_id: vec![0, 1],
        };

        let merger = MeshMergerEngine::new(MeshMergerOptions {
            eps: 0.001,
            weld_boundary_first: true,
            weld_full_after_tfix: false,
            topo_clean: false,
            tfix_passes: 0,
        });

        merger.weld_vertices(&mut g, 0.001, true);

        // boundary-only weld이므로 interior vertex(1,3)는 weld 되지 않아야 한다
        assert_ne!(g.vertices[1], g.vertices[3]);

        // boundary vertex는 그대로
        assert_eq!(g.vertices[0], Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(g.vertices[2], Point3D::new(1.0, 0.0, 0.0));
    }
```
```rust
    #[test]
    fn test_weld_vertices_cluster_average() {
        let mut g = MergedMesh {
            vertices: vec![
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(0.0003, 0.0, 0.0),
                Point3D::new(0.0006, 0.0, 0.0),
            ],
            tris: vec![[0, 1, 2]],
            face_id: vec![0],
        };

        let merger = MeshMergerEngine::new(MeshMergerOptions::default());
        merger.weld_vertices(&mut g, 0.001, false);

        // 평균값 = (0 + 0.0003 + 0.0006) / 3 = 0.0003
        let avg = 0.0003;

        for v in &g.vertices {
            assert!((v.x - avg).abs() < 1e-12);
        }
    }
```
```rust
    #[test]
    fn test_weld_vertices_triangle_indices_unchanged() {
        let mut g = MergedMesh {
            vertices: vec![
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(0.0004, 0.0, 0.0),
                Point3D::new(1.0, 0.0, 0.0),
            ],
            tris: vec![[0, 1, 2]],
            face_id: vec![0],
        };

        let merger = MeshMergerEngine::new(MeshMergerOptions::default());
        merger.weld_vertices(&mut g, 0.001, false);

        // weld는 vertex 위치만 바꾸고 index는 바꾸지 않는다
        assert_eq!(g.tris[0], [0, 1, 2]);
    }
}
```
```rust
#[cfg(test)]
mod fix_tj_tests {
    use nurbslib::core::mesh_merger::{MeshMergerEngine, MeshMergerOptions, TriMesh};
    use nurbslib::core::prelude::Point3D;

    #[inline]
    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }

    /// 기본적인 T‑junction 시나리오:
    /// - 큰 삼각형 ABC
    /// - A–B edge 위에 작은 삼각형의 점 P가 존재
    /// → fix_t_junctions()가 ABC를 split해서 T‑junction을 제거해야 한다.
    #[test]
    fn test_fix_t_junction_basic() {
        let eps = 1e-6;

        // 큰 삼각형 A-B-C
        let a = p(0.0, 0.0, 0.0);
        let b = p(2.0, 0.0, 0.0);
        let c = p(0.0, 2.0, 0.0);

        // A–B 중간점
        let mid = p(1.0, 0.0, 0.0);

        // 작은 삼각형 (mid, b, d)
        let d = p(1.0, -1.0, 0.0);

        let f0 = TriMesh::new(vec![a, b, c], vec![[0, 1, 2]]);
        let f1 = TriMesh::new(vec![mid, b, d], vec![[0, 1, 2]]);

        let mut opt = MeshMergerOptions::default();
        opt.eps = eps;
        opt.weld_boundary_first = false;
        opt.weld_full_after_tfix = false;
        opt.topo_clean = false;
        opt.tfix_passes = 8;

        let mut mm = MeshMergerEngine::new(opt);
        mm.add_meshes([f0, f1]);
        assert!(mm.build());

        let g = mm.result().unwrap();

        // 삼각형 2개 → split 후 3개가 되어야 함
        assert_eq!(
            g.tris.len(),
            3,
            "T‑junction fix should split one triangle into two"
        );

        // T‑junction이 모두 제거되었는지 확인
        let tj = mm.count_t_junctions(eps);
        assert_eq!(tj, 0, "T‑junctions must be fully resolved");
    }
```
```rust
    /// T‑junction이 edge의 거의 끝에 있을 때는 split하지 않아야 한다.
    #[test]
    fn test_fix_t_junction_ignore_near_endpoint() {
        let eps = 1e-6;

        // 큰 삼각형 A-B-C
        let a = p(0.0, 0.0, 0.0);
        let b = p(2.0, 0.0, 0.0);
        let c = p(0.0, 2.0, 0.0);

        // A에 매우 가까운 점 (endpoint 근처 → split 금지)
        let near_a = p(1e-8, 0.0, 0.0);

        let d = p(0.0, -1.0, 0.0);

        let f0 = TriMesh::new(vec![a, b, c], vec![[0, 1, 2]]);
        let f1 = TriMesh::new(vec![near_a, b, d], vec![[0, 1, 2]]);

        let mut opt = MeshMergerOptions::default();
        opt.eps = eps;
        opt.weld_boundary_first = false;
        opt.weld_full_after_tfix = false;
        opt.topo_clean = false;
        opt.tfix_passes = 8;

        let mut mm = MeshMergerEngine::new(opt);
        mm.add_meshes([f0, f1]);
        assert!(mm.build());

        let g = mm.result().unwrap();

        // endpoint 근처 → split 되지 않아야 함 → 삼각형 2개 유지
        assert_eq!(g.tris.len(), 2, "Should NOT split near endpoint");

        // T‑junction은 여전히 0이어야 함 (endpoint는 T‑junction이 아님)
        let tj = mm.count_t_junctions(eps);
        assert_eq!(tj, 0);
    }
```
```rust
    /// 여러 개의 T‑junction이 연속으로 있을 때도 모두 처리되는지 테스트
    #[test]
    fn test_fix_t_junction_multiple() {
        let eps = 1e-6;

        // 큰 삼각형 A-B-C
        let a = p(0.0, 0.0, 0.0);
        let b = p(3.0, 0.0, 0.0);
        let c = p(0.0, 3.0, 0.0);

        // A–B 위에 여러 점
        let p1 = p(1.0, 0.0, 0.0);
        let p2 = p(2.0, 0.0, 0.0);

        let d1 = p(1.0, -1.0, 0.0);
        let d2 = p(2.0, -1.0, 0.0);

        let f0 = TriMesh::new(vec![a, b, c], vec![[0, 1, 2]]);
        let f1 = TriMesh::new(vec![p1, b, d1], vec![[0, 1, 2]]);
        let f2 = TriMesh::new(vec![p2, b, d2], vec![[0, 1, 2]]);

        let mut opt = MeshMergerOptions::default();
        opt.eps = eps;
        opt.weld_boundary_first = false;
        opt.weld_full_after_tfix = false;
        opt.topo_clean = false;
        opt.tfix_passes = 8;

        let mut mm = MeshMergerEngine::new(opt);
        mm.add_meshes([f0, f1, f2]);

        // 초기 삼각형 개수는 3개
        // (검증용으로 찍어두고, build 후에는 증가해야 함)
        let initial_tri_count = 3;

        assert!(mm.build());
        let g = mm.result().unwrap();

        // 최소한 한 번 이상 split은 일어나야 한다 (삼각형이 늘어났는지)
        assert!(
            g.tris.len() > initial_tri_count,
            "Expected triangle count to increase after fixing T-junctions"
        );

        // T‑junction이 모두 제거되었는지 최종적으로 확인
        let tj = mm.count_t_junctions(eps);
        assert_eq!(tj, 0, "All T-junctions should be resolved");
    }
}
```
```rust
#[cfg(test)]
mod topo_weld_tests {
    use nurbslib::core::mesh_merger::{MeshMergerEngine, MeshMergerOptions, TriMesh};
    use nurbslib::core::prelude::Point3D;

    #[inline]
    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }

    /// 두 face가 사실상 같은 삼각형을 갖고 있을 때
    /// topological_weld가 중복 vertex/triangle을 정리하는지 테스트
    #[test]
    fn topological_weld_removes_duplicate_vertices_and_tris() {
        let eps = 1.0e-3;

        // tri ABC
        let a0 = p(0.0, 0.0, 0.0);
        let b0 = p(1.0, 0.0, 0.0);
        let c0 = p(0.0, 1.0, 0.0);

        // 거의 같은 위치의 A'B'C'
        let a1 = p(0.0 + 2e-4, 0.0 - 1e-4, 0.0);
        let b1 = p(1.0 - 1e-4, 0.0 + 1e-4, 0.0);
        let c1 = p(0.0 - 1e-4, 1.0 + 2e-4, 0.0);

        let f0 = TriMesh::new(vec![a0, b0, c0], vec![[0, 1, 2]]);
        let f1 = TriMesh::new(vec![a1, b1, c1], vec![[0, 1, 2]]);

        let mut opt = MeshMergerOptions::default();
        opt.eps = eps;
        opt.weld_boundary_first = false;
        opt.weld_full_after_tfix = false;
        opt.topo_clean = true;
        opt.tfix_passes = 0; // T-fix 끔

        let mut mm = MeshMergerEngine::new(opt);
        mm.add_meshes([f0, f1]);

        assert!(mm.build());
        let g = mm.result().unwrap();

        // 처음에는 vertex 6개, tri 2개
        // topo_weld 후에는 vertex가 3개 근처로 줄고,
        // 완전히 같은 삼각형은 1개만 남아야 한다.
        assert!(
            g.vertices.len() <= 4,
            "expected <=4 vertices after topological_weld, got {}",
            g.vertices.len()
        );
        assert_eq!(
            g.tris.len(),
            1,
            "duplicate triangles should be merged into one"
        );
    }
```
```rust
    #[test]
    fn topological_weld_removes_degenerate_triangles() {
        let eps = 1.0e-3; // 그대로 두되, 실제 topo_weld eps는 0.5 * eps 라고 가정

        // v0와 v1은 "topological_weld 에서 쓰는 eps(=0.0005)" 안에 들어오게 설정
        let v0 = p(0.0, 0.0, 0.0);
        let v1 = p(0.00025, -0.00025, 0.0); // 거리 ≈ 0.000353 < 0.0005
        let v2 = p(1.0, 0.0, 0.0);

        let f0 = TriMesh::new(vec![v0, v1, v2], vec![[0, 1, 2]]);

        let mut opt = MeshMergerOptions::default();
        opt.eps = eps;
        opt.topo_clean = true;
        opt.tfix_passes = 0;

        let mut mm = MeshMergerEngine::new(opt);
        mm.add_mesh(f0);

        assert!(mm.build());
        let g = mm.result().unwrap();

        assert_eq!(g.tris.len(), 0, "degenerate triangles should be removed by topological_weld");
    }
```
```rust
    /// 같은 삼각형이 정점 순서만 다른 형태로 두 번 있을 때
    /// 하나로만 남는지 테스트
    #[test]
    fn topological_weld_removes_permuted_duplicate_tris() {
        let eps = 1.0e-6;

        let a = p(0.0, 0.0, 0.0);
        let b = p(1.0, 0.0, 0.0);
        let c = p(0.0, 1.0, 0.0);

        // 같은 삼각형, 서로 다른 정점 순서
        let f0 = TriMesh::new(vec![a, b, c], vec![[0, 1, 2]]);
        let f1 = TriMesh::new(vec![a, b, c], vec![[2, 0, 1]]);

        let mut opt = MeshMergerOptions::default();
        opt.eps = eps;
        opt.weld_boundary_first = false;
        opt.weld_full_after_tfix = false;
        opt.topo_clean = true;
        opt.tfix_passes = 0;

        let mut mm = MeshMergerEngine::new(opt);
        mm.add_meshes([f0, f1]);

        assert!(mm.build());
        let g = mm.result().unwrap();

        // 정점 순서만 다른 동일 삼각형은 하나만 남아야 한다.
        assert_eq!(
            g.tris.len(),
            1,
            "permuted duplicate triangles should be merged into one"
        );
    }
}
```
```rust
#[cfg(test)]
mod t_junction_tests {
    use nurbslib::core::mesh_merger::{MergedMesh, MeshMergerEngine};
    use nurbslib::core::prelude::Point3D;

    #[inline]
    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }

    // Helper: GlobalMesh 생성
    fn gm(verts: Vec<Point3D>, tris: Vec<[u32; 3]>) -> MergedMesh {
        let cnt = tris.len();
        MergedMesh {
            vertices: verts,
            tris,
            face_id: vec![0; cnt],
        }
    }

    /// 1) 명확한 T‑junction: vertex가 boundary edge의 중간에 정확히 위치
    #[test]
    fn detects_simple_t_junction() {
        // edge: (0,1)
        // vertex 2가 edge 중간에 있음 (어떤 tri에도 포함되지 않음)
        let verts = vec![
            p(0.0, 0.0, 0.0), // 0
            p(1.0, 0.0, 0.0), // 1
            p(0.5, 0.0, 0.0), // 2 -> T-junction
            p(0.0, 1.0, 0.0), // 3 (삼각형용)
        ];

        let tris = vec![
            [0, 1, 3], // boundary edge (0,1)
        ];

        let g = gm(verts, tris);
        let eps = 1e-6;

        let count = MeshMergerEngine::count_t_junctions_impl(&g, eps);
        assert_eq!(count, 1, "should detect 1 T-junction");
    }
```
```rust
    /// 2) vertex가 edge endpoint에 있는 경우 → T‑junction 아님
    #[test]
    fn does_not_count_endpoint_as_t_junction() {
        let verts = vec![
            p(0.0, 0.0, 0.0), // 0
            p(1.0, 0.0, 0.0), // 1
            p(1.0, 0.0, 0.0), // 2 (same as endpoint)
        ];

        let tris = vec![
            [0, 1, 1],
        ];

        let g = gm(verts, tris);
        let eps = 1e-6;

        let count = MeshMergerEngine::count_t_junctions_impl(&g, eps);
        assert_eq!(count, 0, "endpoint vertex must not be counted");
    }
```
```rust
    /// 3) T‑junction이 없는 정상 mesh
    #[test]
    fn no_t_junction_in_clean_mesh() {
        let verts = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(0.0, 1.0, 0.0),
        ];

        let tris = vec![
            [0, 1, 2],
        ];

        let g = gm(verts, tris);
        let eps = 1e-6;

        let count = MeshMergerEngine::count_t_junctions_impl(&g, eps);
        assert_eq!(count, 0, "clean triangle should have no T-junctions");
    }
```
```rust
    /// 4) eps tolerance로 edge 근처에 있는 vertex도 T‑junction으로 잡힘
    #[test]
    fn does_not_count_endpoint_as_t_junction_2() {
        let verts = vec![
            p(0.0, 0.0, 0.0), // 0
            p(1.0, 0.0, 0.0), // 1
            p(1.0, 0.0, 0.0), // 2 (same as endpoint 1)
            p(0.0, 1.0, 0.0), // 3
        ];

        let tris = vec![
            [0, 1, 3],
        ];

        let g = gm(verts, tris);
        let eps = 1e-6;

        let count = MeshMergerEngine::count_t_junctions_impl(&g, eps);
        assert_eq!(count, 0, "endpoint vertex must not be counted");
    }
```
```rust
    #[test]
    fn no_t_junction_in_clean_mesh_2() {
        let verts = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(0.0, 1.0, 0.0),
        ];

        let tris = vec![
            [0, 1, 2],
        ];

        let g = gm(verts, tris);
        let eps = 1e-6;

        let count = MeshMergerEngine::count_t_junctions_impl(&g, eps);
        assert_eq!(count, 0, "clean triangle should have no T-junctions");
    }
```
```rust
    #[test]
    fn detects_t_junction_within_eps2() {
        let eps = 1e-3;

        let verts = vec![
            p(0.0, 0.0, 0.0),       // 0
            p(1.0, 0.0, 0.0),       // 1
            p(0.5, eps * 0.5, 0.0), // 2 -> edge (0,1)에서 eps 안쪽
            p(0.0, 1.0, 0.0),       // 3
        ];

        let tris = vec![
            [0, 1, 3],
        ];

        let g = gm(verts, tris);

        let count = MeshMergerEngine::count_t_junctions_impl(&g, eps);
        assert_eq!(count, 1, "vertex within eps of edge should count as T-junction");
    }
}
```

---





