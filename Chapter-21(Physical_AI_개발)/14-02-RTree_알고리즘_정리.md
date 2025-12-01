## 📘 R‑Tree 구조 및 알고리즘 정리
### 1. 전체 개요
- 차원: 3D (x, y, z)
- 자식 수 제한:
- 최소: $RTREE\\_MIN\\_NODE\\_COUNT=2$
- 최대: $RTREE\\_MAX\\_NODE\\_COUNT=6$
- Leaf 노드: 실제 데이터 id 저장
- Internal 노드: 자식 포인터 + 자식들을 감싸는 AABB 저장
- 삽입: 영역 증가량 최소 branch 선택, 초과 시 split
- 삭제: underflow 발생 시 재삽입(reinsert)
- 검색: AABB, Sphere, Capsule, Line, Plane, Pairwise 지원

### 2. 기본 데이터 구조
#### 2.1 AABB (Bounding Box)

$$
\mathrm{min}=(x_{\min },y_{\min },z_{\min }),\quad \mathrm{max}=(x_{\max },y_{\max },z_{\max })
$$

- 조건: $\mathrm{min}[i]\leq \mathrm{max}[i]$

#### 2.2 Sphere

$$
\mathrm{Sphere}(p,r)\quad \mathrm{중심\  }p\in \mathbb{R^{\mathnormal{3}}}, \quad \mathrm{반지름\  }r
$$

- 검색 조건:

$$
\mathrm{dist}(p,\mathrm{AABB})\leq r
$$

#### 2.3 Capsule

$$
\mathrm{Capsule}(p_0,p_1,r,[t_0,t_1])
$$

- 선분: $p(t)=p_0+(p_1-p_0)\cdot t, \quad t\in [t_0,t_1]$
- 반지름 $r$
- 캡슐 = 선분 + 원기둥 + 양 끝 반구

### 3. 핵심 유틸리티 함수
#### 3.1 AABB 합치기

$$
r_{\min }[i]=\min (a_{\min }[i],b_{\min }[i]),\quad r_{\max }[i]=\max (a_{\max }[i],b_{\max }[i])
$$

#### 3.2 Heuristic (Volume-like)

$$
h(r)=(x_{\max }-x_{\min })^2+(y_{\max }-y_{\min })^2+(z_{\max }-z_{\min })^2
$$

#### 3.3 Overlap 판정

$$
a_{\max }[i]\geq b_{\min }[i]\quad \wedge \quad a_{\min }[i]\leq b_{\max }[i]\quad \forall i\in \{ x,y,z\}
$$

#### 3.4 점–박스 거리 (근사)

$$
d[i] =
\begin{cases}
\mathrm{min}[i] - pt[i], & pt[i] < \mathrm{min}[i] \\
pt[i] - \mathrm{max}[i], & pt[i] > \mathrm{max}[i] \\
0, & \text{otherwise}
\end{cases}
$$

- 최종 근사:

$$
d\approx d[0]\cdot \sqrt{1+\left( \frac{d[1]}{d[0]}\right) ^2+\left( \frac{d[2]}{d[0]}\right) ^2}
$$

#### 3.5 Capsule–AABB 거리

$$
t=\frac{v\cdot (c-p_0)}{v\cdot v},\quad t\in [t_0,t_1]
$$

$$
\mathrm{proj}=p_0+v\cdot t
$$

$$
\mathrm{dist}=\| c-\mathrm{proj}\|
$$

3.6 선분 vs AABB (Slab Test)

$$
p(t)=a+d\cdot t,\quad 0\leq t\leq 1
$$

- 각 축별로 t 범위를 계산하고, 교집합이 존재하면 겹침.

#### 3.7 박스 vs 평면
- 평면식:

$$
f(x,y,z)=ax+by+cz+d
$$

AABB 8개 코너에서 $f(corner)$ 계산 → $[minv,maxv]$ 구간과 겹치면 true.

### 4. 삽입 알고리즘
#### 4.1 Branch 선택

$$
\mathrm{incr}=h(\mathrm{combine}(rect,cur))-h(cur)
$$

증가량 최소 branch 선택, 동률이면 area 작은 쪽 선택.

#### 4.2 Split
- Seed pair: waste가 가장 큰 두 branch 선택

$$
\mathrm{waste}(a,b)=h(\mathrm{combine}(a,b))-h(a)-h(b)
$$

- 나머지 branch는 증가량 비교 후 배치

### 5. 삭제 알고리즘
- 루트부터 overlap 검사 → leaf에서 id 일치 branch 삭제
- underflow 발생 시 서브트리 잘라 재삽입
- root가 internal이고 자식 1개뿐이면 root 교체

### 6. 검색 알고리즘
- AABB 검색: overlap 검사
- Sphere 검색: $\mathrm{dist}(sphere.center,rect)\leq r$
- Capsule 검색: $\mathrm{dist}(capsule.axis,rect)\leq r$
- Line 검색: slab test
- Plane 검색: box_plane_overlap

### 7. Pairwise 검색
#### 7.1 두 트리 간

$$
dx=\max (0,b_{\min }[x]-a_{\max }[x],a_{\min }[x]-b_{\max }[x])
$$

$$
dy,dz\mathrm{도\  동일}
$$

$$
\mathrm{dist}=\sqrt{dx^2+dy^2+dz^2}
$$

$$
\mathrm{dist}\leq tolerance \quad \Rightarrow \quad 겹침
$$

#### 7.2 자기 자신
- (a==b) && (i>=j) skip
- 나머지는 동일하게 pair 생성

### 8. Mesh Face Tree
- 각 face의 vertex bounding box 계산 → RTree 삽입
- 전체 mesh의 bounding box = root cover

---


# RTree 구조와 알고리즘 정리 (rtree.md)

이 문서는 `src/rtree/mod.rs` 에 구현된 R-Tree 자료구조의 구조, 사용된 수식,  
주요 함수와 용도 등을 설명하기 위한 문서입니다.  
GitHub / Markdown 환경에서 그대로 복사해서 사용할 수 있도록,  
수식은 모두 ASCII 형태로 작성했습니다.

---

## 1. 전체 개요

R-Tree 는 축 정렬 AABB(Axis-Aligned Bounding Box)를 노드의 키로 사용하는 계층적 공간 분할 트리입니다.  
이 구현은 다음 특징을 가집니다.

- 차원: 3차원 (x, y, z)
- 최소/최대 자식 수:
  - `RTREE_MIN_NODE_COUNT = 2`
  - `RTREE_MAX_NODE_COUNT = 6`
- leaf 노드: 실제 사용자 데이터(id)를 저장
- internal 노드: 자식 노드 포인터와 그 자식을 모두 감싸는 AABB 를 저장
- 삽입: 영역 증가량이 최소가 되도록 branch 선택, 초과 시 분할(splitting)
- 삭제: underflow 발생 시, 서브트리를 잘라서 다시 삽입(reinsert)
- 검색: bounding box / sphere / capsule / line / bounded plane / pairwise (tree 간, 자기 자신) 검색

---
# 소스 친화적 설명

## 2. 기본 데이터 구조

### 2.1 축정렬 박스: `RTreeBBox`

```rust
pub struct RTreeBBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}
```

```
min = [xmin, ymin, zmin]
max = [xmax, ymax, zmax]
```
- 항상 min[i] <= max[i] 이어야 합니다.
- 트리의 모든 노드와 leaf 는 자신의 영역을 AABB 로 표현합니다.

### 2.2 구(sphere) / 캡슐(capsule)
```rust
코드 복사
pub struct RTreeSphere {
    pub point: [f64; 3],
    pub radius: f64,
}
```
- 중심 point 와 반지름 radius 로 표현되는 3D 구입니다.
- 검색 시, "구와 AABB 간의 최소 거리 <= radius" 조건으로 겹침 여부를 판정합니다.

```rust
pub struct RTreeCapsule {
    pub point: [[f64; 3]; 2],  // segment endpoints
    pub radius: f64,           // capsule radius
    pub domain: [f64; 2],      // segment param restriction [t0, t1]
}
```
- 선분 (p0, p1) 의 양 끝점과, 선분을 중심으로 하는 원기둥 + 양 끝 반구의 합(캡슐)을 표현합니다.
- $domain = [t0, t1]$ 은 축 선분에서 허용되는 param 범위입니다. (보통 [0,1])

### 2.3 브랜치와 노드
```rust
pub enum BranchChildOrId {
    Child(*mut RTreeNode),
    Id(usize),
}
```
- internal 노드:
  - Child(ptr) 를 사용, 자식 노드 포인터를 저장

- leaf 노드:
  - Id(usize) 를 사용, 사용자 데이터 id 를 저장

```rust
pub struct RTreeBranch {
    pub rect: RTreeBBox,
    pub child_or_id: BranchChildOrId,
}
```
- 한 노드 안의 하나의 자식 엔트리
- rect 는 자식 or leaf 가 커버하는 전체 영역을 나타냄

```rust
pub struct RTreeNode {
    pub level: i32,                      // 0: leaf, >0: internal
    pub count: i32,                      // 실제 사용중인 branch 수
    pub branch: [RTreeBranch; 6],        // RTREE_MAX_NODE_COUNT
}
```
- level == 0 이면 leaf
- level > 0 이면 internal
- count 개의 branch 만 유효, 나머지는 쓰레기 값일 수 있음

### 2.4 RTree 본체 및 NodeArena
```rust
pub struct RTree {
    root: *mut RTreeNode,
    arena: NodeArena,
}
```
- root: 루트 노드 포인터 (null 이면 비어 있음)
- arena: 노드 메모리 관리용 간단한 allocator

```rust
pub struct NodeArena {
    nodes: Vec<*mut RTreeNode>,
    list_nodes: Vec<*mut RTreeListNode>,
    heap: usize,
}
```
- alloc_node() / free_node() 를 통해 RTreeNode 를 생성/해제
- alloc_list_node() / free_list_node() 는 remove 과정에서 쓰는 재삽입 리스트용
- deallocate_all() : 모든 노드 해제 후 arena 초기화

### 2.5 RTreeIterator
```rust
pub struct RTreeIterator {
    stack: [StackElement; MAX_STACK],
    sp: i32,
    root: *const RTreeNode,
}
```
- DFS 방식으로 leaf 의 branch 들을 순회하는 iterator
- stack: 루트에서 현재 leaf 까지의 경로를 저장
- sp: 현재 스택 포인터 ( -1 이면 유효하지 않은 상태 / end 상태)
- value() 로 현재 leaf 의 RTreeBranch 를 얻고, next() 로 다음 leaf branch 로 이동

## 3. 핵심 유틸리티 함수와 수식
### 3.1 사각형(박스) 합치기: combine_rect
```rust
pub fn combine_rect(a: &RTreeBBox, b: &RTreeBBox) -> RTreeBBox {
    let mut r = *a;
    for j in 0..3 {
        if r.min[j] > b.min[j] { r.min[j] = b.min[j]; }
        if r.max[j] < b.max[j] { r.max[j] = b.max[j]; }
    }
    r
}
```
- 두 AABB a, b 를 모두 포함하는 최소 AABB 를 계산합니다.
- 좌표별로 min/max 를 갱신하는 단순한 연산입니다.
- 수식으로 쓰면:
```
r.min[i] = min(a.min[i], b.min[i])
r.max[i] = max(a.max[i], b.max[i])
```
### 3.2 박스의 크기를 재는 heuristic: rect_volume_heuristic
```rust
pub fn rect_volume_heuristic(r: &RTreeBBox) -> f64 {
    let dx = r.max[0] - r.min[0];
    let dy = r.max[1] - r.min[1];
    let dz = r.max[2] - r.min[2];
    dx*dx + dy*dy + dz*dz
}
```
- 실제 volume(dx * dy * dz)을 쓰지 않고, 간단한 제곱합을 사용합니다:
- volume_like = dx^2 + dy^2 + dz^2
- 이유: 비교만 하면 되므로, 더 간단하고 overflow 가능성이 낮고 계산이 가벼운 값 사용

### 3.3 AABB overlap 판정: overlap
```rust
pub fn overlap(a: &RTreeBBox, b: &RTreeBBox) -> bool {
    a.max[0] >= b.min[0] && a.min[0] <= b.max[0] &&
    a.max[1] >= b.min[1] && a.min[1] <= b.max[1] &&
    a.max[2] >= b.min[2] && a.min[2] <= b.max[2]
}
```
- 각 축별로 projection 이 겹치는지 검사합니다.
- 조건 (x, y, z 에 대해 모두 성립):
```
a.max[i] >= b.min[i]
a.min[i] <= b.max[i]
``
- 세 축 모두 겹치면 AABB가 겹친다고 판단.

### 3.4 박스와 점 사이 거리: distance_to_box
```rust
fn distance_to_box(pt: &[f64; 3], r: f64, rect: &RTreeBBox) -> f64 {
    let mut d = [0.0; 3];
    for i in 0..3 {
        if pt[i] < rect.min[i] {
            d[i] = rect.min[i] - pt[i];
            if d[i] > r { return d[i]; }
        } else if pt[i] > rect.max[i] {
            d[i] = pt[i] - rect.max[i];
            if d[i] > r { return d[i]; }
        } else {
            d[i] = 0.0;
        }
    }
    if d[0] > 0.0 {
        let mut d1 = d[1] / d[0];
        let mut d2 = d[2] / d[0];
        d[0] *= (1.0 + d1*d1 + d2*d2).sqrt();
    }
    d[0]
}
```
- 점 pt 에서 AABB 까지의 최소 거리를 근사적으로 계산합니다.
- 각 축 i 에 대해:
- pt[i] < min[i] 이면 밖에 있으므로 d[i] = min[i] - pt[i]
- pt[i] > max[i] 이면 d[i] = pt[i] - max[i]
- 그 외에는 축 i 방향으로는 박스 안이므로 d[i] = 0
- 첫 번째 축 d[0] 을 기준으로 상대 비율(d1 = d[1]/d[0] 등)을 써서 3D 거리로 변환
- 목적: 정확한 거리보다 "반지름 r 보다 큰지 작은지" 를 빠르게 판단

### 3.5 캡슐 축과 AABB 거리: distance_to_capsule_axis
```rust
fn distance_to_capsule_axis(c: &RTreeCapsule, rect: &RTreeBBox) -> f64 {
    let p0 = c.point[0];
    let p1 = c.point[1];
    let center = [
        (rect.min[0] + rect.max[0]) * 0.5,
        (rect.min[1] + rect.max[1]) * 0.5,
        (rect.min[2] + rect.max[2]) * 0.5,
    ];
    let v = [p1[0]-p0[0], p1[1]-p0[1], p1[2]-p0[2]];
    let w = [center[0]-p0[0], center[1]-p0[1], center[2]-p0[2]];
    let c1 = v[0]*w[0] + v[1]*w[1] + v[2]*w[2];
    let c2 = v[0]*v[0] + v[1]*v[1] + v[2]*v[2];
    let t = if c2 > 0.0 { c1/c2 } else { 0.0 };
    let t = t.clamp(c.domain[0], c.domain[1]);
    let proj = [p0[0] + v[0]*t, p0[1] + v[1]*t, p0[2] + v[2]*t];

    let dx = center[0] - proj[0];
    let dy = center[1] - proj[1];
    let dz = center[2] - proj[2];
    (dx*dx + dy*dy + dz*dz).sqrt()
}
```
- 선분 p(t) = p0 + v * t, t in [t0, t1]
- 박스 중심점 center 를 선분에 투영:
  - c1 = v · (center - p0)
  - c2 = v · v
  - t = c1 / c2 (c2 == 0이면 선분 degenerate)
  - t 를 [domain[0], domain[1]] 로 clamp
  - proj = p0 + v * t
- 거리 = |center - proj|
- 이 거리가 capsule.radius 이하이면 AABB 가 캡슐과 겹친다고 판단

### 3.6 선분 vs AABB: bbox_line_overlap
```rust
fn bbox_line_overlap(line: &Line, rect: &RTreeBBox, infinite: bool) -> bool {
    let (bmin, bmax) = (rect.min, rect.max);
    let a = line.p0;
    let b = line.p1;

    if infinite {
        // 무한 직선에 대한 단순 heuristic
        ...
    } else {
        // 선분 vs AABB slab test
        let mut tmin = 0.0;
        let mut tmax = 1.0;
        let d = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
        for i in 0..3 {
            if d[i].abs() < 1e-15 {
                if a[i] < bmin[i] || a[i] > bmax[i] { return false; }
                continue;
            }
            let inv = 1.0/d[i];
            let t0 = (bmin[i] - a[i]) * inv;
            let t1 = (bmax[i] - a[i]) * inv;
            let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };
            if t0 > tmin { tmin = t0; }
            if t1 < tmax { tmax = t1; }
            if tmax < tmin { return false; }
        }
        true
    }
}
```
- 표준적인 "slab" 알고리즘:
- 선분 p(t) = a + d * t, 0 <= t <= 1
- 축별로 AABB 와 교차하는 t 범위를 구해서, 세 축 모두 공통 교집합이 존재하면 겹침

3.7 박스 vs 평면 구간: box_plane_overlap (bounded plane search 내부)
rust
코드 복사
fn box_plane_overlap(plane: [f64; 4], b: &RTreeBBox, minv: f64, maxv: f64) -> bool {
    let corners = [... 8개 코너 ...];
    let mut below = false;
    let mut above = false;
    for c in corners.iter() {
        let v = plane[0]*c[0] + plane[1]*c[1] + plane[2]*c[2] + plane[3];
        if v < minv { below = true; }
        if v > maxv { above = true; }
        if v >= minv && v <= maxv { return true; }
    }
    below && above
}
평면식: f(x,y,z) = ax + by + c*z + d

AABB 의 8개 코너에서 f(corner) 값 계산

목표: f 값이 [minv, maxv] 구간과 겹치는지

코너들 중 하나라도 minv <= f <= maxv 이면 바로 true

그렇지 않으면:

일부 코너들은 f < minv

다른 코너들은 f > maxv

즉, 박스가 평면 양쪽에 걸쳐 있으면 true

4. 삽입 알고리즘
4.1 브랜치 선택: pick_branch
rust
코드 복사
fn pick_branch(rect: &RTreeBBox, node: &RTreeNode) -> i32 {
    let mut best = -1;
    let mut best_incr = f64::MAX;
    let mut best_area = f64::MAX;

    for i in 0..(node.count as usize) {
        let cur = &node.branch[i].rect;
        let area = rect_volume_heuristic(cur);
        let temp = combine_rect(rect, cur);
        let incr = rect_volume_heuristic(&temp) - area;

        if incr < best_incr || (incr == best_incr && area <= best_area) {
            best = i as i32;
            best_area = area;
            best_incr = incr;
        }
    }
    best
}
새 rect 를 어떤 child 에 넣을지 선택하는 함수

각 child 에 대해:

기존 영역 "area" = rect_volume_heuristic(cur)

새 rect 를 포함했을 때 영역 "temp_area" = rect_volume_heuristic(combine_rect(rect, cur))

증가량 "incr" = temp_area - area

incr 가 최소인 브랜치 선택

동률이면 기존 area 작은 쪽 선택 (더 compact 한 쪽)

4.2 노드에 브랜치 추가: add_branch
rust
코드 복사
unsafe fn add_branch(
    branch: RTreeBranch,
    node: *mut RTreeNode,
    new_node_out: &mut Option<*mut RTreeNode>,
    arena: &mut NodeArena
) -> bool {
    let node_ref = &mut *node;
    if node_ref.count < RTREE_MAX_NODE_COUNT as i32 {
        node_ref.branch[node_ref.count as usize] = branch;
        node_ref.count += 1;
        return false;          // split 필요 없음
    }
    // 꽉 찼으면 split
    split_node(node, branch, new_node_out, arena);
    true                       // split 발생
}
4.3 노드 분할: split_node
RTREE_MAX_NODE_COUNT + 1 개의 브랜치를 모아두고, 두 그룹으로 나눕니다.

알고리즘 개략:

기존 브랜치들 + 새 extra branch 를 buffer 에 저장

seed 선택:

두 브랜치 (a,b) 에 대해:

합친 rect 의 "waste" = heuristic(combine_rect) - heuristic(a) - heuristic(b)

waste 가 가장 큰 pair 를 seed 로 선택 (가장 멀리 떨어진 pair)

seed0 를 원래 노드 A 에, seed1 을 새 노드 B 에 배치

나머지 브랜치들을 하나씩 A/B 에 할당:

최소 채움(min_fill) 조건 강제

그렇지 않으면 A에 넣을 때 증가량과 B에 넣을 때 증가량을 비교해서 더 좋은 쪽으로 배치

rust
코드 복사
let waste = rect_volume_heuristic(&comb)
          - rect_volume_heuristic(&buffer[a].rect)
          - rect_volume_heuristic(&buffer[b].rect);
waste 가 클수록 두 rect 가 서로 떨어져 있으므로 좋은 seed pair 후보.

4.4 재귀 삽입: insert_rec
전체 삽입 순서:

text
코드 복사
insert_rect()
  └── insert_rec()
          └── internal or leaf 처리
               - internal: pick_branch -> child 로 재귀 -> split 발생시 상위에 반영
               - leaf: add_branch
leaf 에서 split 이 발생하면 insert_rec 은 true 를 리턴하여 상위에 새 노드를 전달

root 에서도 split 이 발생하면, 새 root 를 만들어 둘을 child 로 가지고 시작

5. 삭제 알고리즘
5.1 remove_rect
삭제의 기본 아이디어:

루트부터 내려가면서 AABB overlap 이 가능한 노드만 탐색

leaf 층에서 id 가 일치하는 branch 를 찾으면 삭제

그 과정에서 어떤 internal child 가 RTREE_MIN_NODE_COUNT 보다 적은 branch 를 갖게 되면 underflow

underflow 노드는 통째로 잘라서 재삽입 리스트에 넣고, 해당 child branch 는 부모에서 제거

모든 삭제가 끝나면 재삽입 리스트의 서브트리 노드들에 들어있는 leaf 들을 다시 트리에 삽입

root 가 internal 이고 child 가 1개 뿐이면 root 를 child 로 교체 (트리 높이 감소)

6. 검색 알고리즘
6.1 AABB 검색
rust
코드 복사
pub fn search_bbox_collect_ids(&self, rect: &RTreeBBox) -> Vec<usize>
입력: 검색 AABB

internal 노드: branch.rect 와 overlap 이면 자식으로 재귀

leaf 노드: overlap 인 leaf 의 id 를 결과에 push

6.2 구 검색: search_sphere_callback
distance_to_box(sphere.point, sphere.radius, rect) <= sphere.radius 인 노드에 대해서만 재귀

leaf 층에서는 callback(ctx, id) 호출

6.3 캡슐 검색: search_capsule_callback
distance_to_capsule_axis(capsule, rect) <= capsule.radius 인 node 만 골라서 재귀 / leaf 처리

6.4 선 / 무한선 검색
rust
코드 복사
pub fn search_line_callback(...)
pub fn search_infinite_line_callback(...)
bbox_line_overlap(line, rect, infinite) 가 true 인 branch 만 재귀

leaf 층에서 겹치는 id에 대해 callback 호출

6.5 경계가 있는 평면 검색
rust
코드 복사
pub fn search_bounded_plane_callback(
    plane_eqn: [f64; 4],
    min_val: f64,
    max_val: f64,
    ...
)
박스 vs 평면 구간 overlap:

코너 8개에서 평면식 값을 계산하고, [min_val, max_val] 구간과 겹치는지 검사

internal/leaf 모두 이 체크를 통과하는 branch 에 대해서만 재귀/leaf 처리

7. Pairwise 검색 (트리 간 / 자기 자신)
7.1 두 트리 간 pair 검색: pair_search_collect
rust
코드 복사
pub fn pair_search_collect(a: *const RTreeNode, b: *const RTreeNode, tolerance: f64)
    -> Vec<(usize, usize)>
각 박스쌍에 대해 pair_overlap(a_rect, b_rect, tol) 판정:

축 별 최소 간격 dx, dy, dz 계산

거리 = sqrt(dx^2 + dy^2 + dz^2)

거리 <= tolerance 이면 겹치는 것으로 봄

AABB 가 실제로 겹치면 dx,dy,dz 모두 0 이므로 항상 0 <= tol

둘 다 internal 이면 하위 노드 쌍으로 재귀

한쪽만 internal 이면, internal 쪽의 서브트리와 leaf 쪽 id 를 매칭

둘 다 leaf 이면 둘의 id 를 pair 로 결과에 push

7.2 자기 자신 안에서 pair 검색: single_tree_pairs_collect
rust
코드 복사
pub fn single_tree_pairs_collect(root: *const RTreeNode, tolerance: f64)
    -> Vec<(usize, usize)>
루트를 (root, root) 쌍으로 시작

(a == b) && (i >= j) 인 경우는 skip 해서 중복/자기자신 pair 제거

나머지는 pair_search_collect 와 유사한 구조

leaf 층에서 Id 쌍을 만들어 결과에 push

8. Mesh face RTree 빌더
rust
코드 복사
pub fn create_mesh_face_tree(tree: &mut RTree, mesh: &Mesh) -> bool
입력: Mesh (vertices + faces)

각 face 에 대해:

face 를 구성하는 vertex 들의 bounding box 계산

그 bbox 를 RTree 에 삽입 (id = face index)

성공하면 tree.bounding_box() 는 mesh 전체의 AABB 를 반환

9. Iterator 사용 예
rust
코드 복사
#[test]
fn test_iterator() {
    let mut rt = RTree::new(0);
    rt.insert([0.0,0.0,0.0], [1.0,1.0,1.0], 10);
    rt.insert([2.0,2.0,2.0], [3.0,3.0,3.0], 20);

    let mut it = rt.iter();
    let mut ids = Vec::new();
    while let Some(branch) = it.value() {
        if let BranchChildOrId::Id(id) = branch.child_or_id {
            ids.push(id);
        }
        it.next();
    }
    assert!(ids.contains(&10));
    assert!(ids.contains(&20));
}
iter() 로 iterator 생성

value() 가 Some 인 동안:

현재 leaf branch 의 id 를 읽고

next() 로 다음 branch 로 이동

next() 가 더 이상 갈 곳이 없으면 내부적으로 sp = -1 로 바뀌어,
value() 가 자동으로 None 을 반환

10. 정리
이 RTree 구현은:

3D AABB 기반 R-Tree

삽입: 영역 증가량 기반 branch 선택 + linear split

삭제: underflow 서브트리 재삽입

검색: AABB, sphere, capsule, segment/infinite line, bounded plane

pairwise 검색: 두 트리 간 충돌 후보 / 자기 자신 내 후보 쌍 탐색

메모리: NodeArena 로 raw pointer 를 관리

순회: RTreeIterator 로 leaf branch 를 순서대로 접근

실제 geometry/mesh boolean, 충돌 검출, 근접 탐색 등에 재사용하기 좋도록
일반적인 RTree 패턴을 유지하면서도,
sphere/capsule/line/plane 같은 다양한 검색 프리미티브를 지원하도록 설계되어 있습니다.


