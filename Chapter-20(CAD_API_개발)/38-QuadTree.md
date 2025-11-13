# QuadTree
## 📐 수학적 배경: Quadtree의 개념
Quadtree는 2차원 공간을 재귀적으로 4분할하여 데이터를 효율적으로 저장하고 탐색하는 공간 분할 자료구조입니다.  
특히 다음과 같은 작업에 유용합니다:  
- 범위 질의 (Range Query): 특정 영역 내의 점 찾기
- 최근접 질의 (Nearest Neighbor): 특정 위치에 가장 가까운 점 찾기
- 공간 인덱싱 및 시각화: 대규모 2D 데이터의 구조화

## 📊 수학적 정의
### 1. 사각형 포함 여부
- 점 P(x,y)가 사각형 B(x_c,y_c,w,h)에 포함되는 조건:  

$$
x_c-w\leq x\leq x_c+w,\quad y_c-h\leq y\leq y_c+h
$$

### 2. 사각형 교차 여부 (AABB)
- 두 사각형 B_1과 B_2가 교차하는 조건:

$$
\neg \left( B_2.x_{\min }>B_1.x_{\max }\vee B_2.x_{\max }<B_1.x_{\min }\vee B_2.y_{\min }>B_1.y_{\max }\vee B_2.y_{\max }<B_1.y_{\min }\right) 
$$


### 3. 거리 계산
- 점 P(x,y)와 기준점 Q(x_0,y_0) 사이의 거리:

$$
d(P,Q)=\sqrt{(x-x_0)^2+(y-y_0)^2}
$$

## 🧩 소스 구조 요약

### 📦 주요 구조체

| 구조체 이름           | 핵심 필드/의존성     | 설명 |
|----------------------|----------------------|------|
| `QuadPoint`          | `id`, `x`, `y`, `name`, `kind` | 공간상의 점 정보 (ID, 좌표, 이름, 종류) |
| `QuadBox`            | `x`, `y`, `half_w`, `half_h`   | 사각형 경계 영역 (중심 + 반폭/반높이) |
| `QuadTree`           | `boundary`, `points`, `children` | Quadtree 노드: 점 저장, 분할, 질의 |
| `QuadEntityDataNode` | `Point2D` 기반 `min_pt`, `max_pt` | 엔티티 기반 사각형 노드 (인덱스 저장) |
| `QuadTreeEntity`     | `Vec<Point2D>`, `root`, `limit` | 전체 포인트 관리 및 트리 구축 로직 |



## 🧠 핵심 함수 단계별 설명
### 1. QuadTree::insert(p)
- 입력: QuadPoint
- 단계:
    - 현재 노드의 boundary가 점을 포함하는지 확인
    - 용량(capacity) 미만이면 points에 추가
    - 용량 초과 시 subdivide() 호출 → 4개 자식 생성
    - 자식 노드에 재귀적으로 삽입 시도

### 2. QuadTree::query(range, found)
- 입력: QuadBox 범위, 결과 벡터
- 단계:
    - 현재 노드가 범위와 교차하지 않으면 종료
    - 현재 노드의 점들 중 범위 내 점을 found에 복사
    - 자식 노드가 있으면 재귀적으로 질의

### 3. QuadTree::query_by_type(range, kind, found)
- query와 동일하지만 kind 필터 추가

### 4. QuadTree::remove(id)
- 입력: 삭제할 점의 ID
- 단계:
    - 현재 노드의 points에서 ID 검색 후 제거
    - 자식 노드가 있으면 재귀적으로 삭제 시도

### 5. QuadTree::find_nearest_in_range(range, x, y)
- 입력: 범위, 기준 좌표
- 단계:
    - query()로 범위 내 점 수집
    - 각 점과 기준점 사이 거리 계산
    - 가장 가까운 점 반환

### 6. QuadTree::visualize(depth)
- 입력: 깊이 (들여쓰기용)
- 단계:
    - 현재 노드의 경계와 점 출력
    - 자식 노드가 있으면 재귀적으로 출력

### 7. QuadTreeEntity::build()
- 역할: QuadEntityDataNode 기반 트리 구축
- 단계:
    - 전체 포인트의 AABB 계산
    - 루트 노드 생성
    - spatial_subdivision() 호출로 재귀 분할

### 8. QuadTreeEntity::spatial_subdivision(...)
- 입력: 부모 노드, 인덱스 집합, 최대 개수
- 단계:
    - 인덱스 수가 limit 이하이면 리프 처리
    - 자식 노드 생성
    - 각 자식에 포함되는 인덱스 분배
    - 분할 효과 없으면 자식 제거 후 리프 처리
    - 자식 노드에 대해 재귀 호출

### 9. on_quad_entity_query_node(...)
- 역할: QuadEntityDataNode 기반 범위 질의
- 단계:
    - 노드와 질의 범위가 교차하는지 확인
    - 리프면 인덱스 검사 후 결과에 추가
    - 자식 노드가 있으면 재귀 질의

## 📌 요약
- Quadtree는 공간 분할 기반으로 효율적인 질의와 삭제, 최근접 탐색을 지원
- 수학적 조건은 AABB 포함/교차, 거리 계산에 기반
- Rust 구현은 안전성과 재귀적 구조를 잘 활용하며, Box와 Option을 통해 트리 구조를 표현



QuadTree 구현을 기반으로 수학적 정의와 알고리즘의 정확성을 검토.  
아래는 주요 연산들에 대한 수학적 정당성과 그에 대한 코드 구현의 일치 여부를 분석한 내용입니다.

## ✅ 1. 사각형 포함 검사 (QuadBox::contains)
### 수학적 정의:
- 점 P(x,y)가 사각형 B(x_c,y_c,w,h)에 포함되려면:

$$
x_c-w\leq x\leq x_c+w,\quad y_c-h\leq y\leq y_c+h
$$


- 코드 구현:
```rust
p.x >= self.x - self.half_w &&
p.x <= self.x + self.half_w &&
p.y >= self.y - self.half_h &&
p.y <= self.y + self.half_h
```
- ✅ 정확히 일치합니다.

## ✅ 2. 사각형 교차 검사 (QuadBox::intersects)
### 수학적 정의 (AABB 교차):
- 두 사각형 A와 B가 교차하지 않으려면:

$$
B_{\min x}>A_{\max x}\vee B_{\max x}<A_{\min x}\vee B_{\min y}>A_{\max y}\vee B_{\max y}<A_{\min y}
$$

- 이를 부정하면 교차 조건이 됩니다.
- 코드 구현:
```rust
!(range.x - range.half_w > self.x + self.half_w ||
  range.x + range.half_w < self.x - self.half_w ||
  range.y - range.half_h > self.y + self.half_h ||
  range.y + range.half_h < self.y - self.half_h)
```
- ✅ 수학적으로 정확한 AABB 교차 판정입니다.

## ✅ 3. 거리 계산 (QuadPoint::distance_to)
### 수학적 정의:

$$
d=\sqrt{(x-x_0)^2+(y-y_0)^2}
$$


- 코드 구현:
```rust
let dx = self.x - px;
let dy = self.y - py;
(dx * dx + dy * dy).sqrt()
```
- ✅ 정확히 일치합니다.

## ✅ 4. 최근접 점 찾기 (find_nearest_in_range)
### 알고리즘:
- query로 범위 내 점 수집
- 각 점과 기준점 사이 거리 계산
- 최소 거리 점 반환
### 수학적 정당성:
- 유클리드 거리 기반 최근접 탐색
- 최솟값 비교 방식은 정확함
- ✅ 수학적으로 타당하며 구현도 정확합니다.

## ✅ 5. 사분할 (subdivide)
### 수학적 정의:
- 중심점 (x,y)를 기준으로 4개의 사각형 생성:
    - NE: (x+w/2,y-h/2)
    - NW: (x-w/2,y-h/2)
    - SE: (x+w/2,y+h/2)
    - SW: (x-w/2,y+h/2)
- 코드 구현:
```rust
let hw = self.boundary.half_w * 0.5;
let hh = self.boundary.half_h * 0.5;
...
QuadBox::new(x ± hw, y ± hh, hw, hh)
```
- ✅ 정확한 사분할 수학적 정의와 일치합니다.

## ✅ 6. QuadTreeEntity의 공간 분할 (spatial_subdivision)
- AABB 기반 포함 검사
- 자식 노드에 점 분배
- 분할 효과 없으면 리프 처리
- 이 방식은 공간 분할 알고리즘의 일반적인 기준과 일치하며, 수학적으로도 타당합니다.

## 🔍 수학적 검증 결과

| 항목                         | 수학적 정의와 일치 | 코드 구현의 정확성 |
|------------------------------|---------------------|---------------------|
| 점의 사각형 포함 검사        | ✅ 일치              | ✅ 정확함            |
| 사각형 간의 교차 검사        | ✅ 일치              | ✅ 정확함            |
| 유클리드 거리 계산           | ✅ 일치              | ✅ 정확함            |
| 최근접 점 탐색               | ✅ 일치              | ✅ 정확함            |
| 사분할 좌표 계산             | ✅ 일치              | ✅ 정확함            |
| AABB 기반 공간 분할          | ✅ 일치              | ✅ 정확함            |
| 리프 노드 판정 및 처리       | ✅ 일치              | ✅ 정확함            |

---

## 소스 코드

```rust
use crate::core::geom::Point2D;
use std::collections::HashSet;
use std::fmt;
// -----------------------------
// 1) QuadPoint / QuadBox / QuadTree
// -----------------------------

#[derive(Clone, PartialEq)]
pub struct QuadPoint {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub kind: String, // C++의 "type"은 러스트 키워드라서 kind로 변경
}
```
```rust
impl fmt::Debug for QuadPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuadPoint")
            .field("id", &self.id)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("name", &self.name)
            .field("kind", &self.kind)
            .finish()
    }
}
```
```rust
impl QuadPoint {
    pub fn new(id: i32, x: f64, y: f64, name: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            id,
            x,
            y,
            name: name.into(),
            kind: kind.into(),
        }
    }
    #[inline]
    pub fn distance_to(&self, px: f64, py: f64) -> f64 {
        let dx = self.x - px;
        let dy = self.y - py;
        (dx * dx + dy * dy).sqrt()
    }
}
```
```rust
#[derive(Copy, Clone, Debug)]
pub struct QuadBox {
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
}
```
```rust
impl QuadBox {
    pub fn new(x: f64, y: f64, half_w: f64, half_h: f64) -> Self {
        Self {
            x,
            y,
            half_w,
            half_h,
        }
    }
    #[inline]
    pub fn contains(&self, p: &QuadPoint) -> bool {
        p.x >= self.x - self.half_w
            && p.x <= self.x + self.half_w
            && p.y >= self.y - self.half_h
            && p.y <= self.y + self.half_h
    }
    #[inline]
    pub fn intersects(&self, range: &QuadBox) -> bool {
        !(range.x - range.half_w > self.x + self.half_w
            || range.x + range.half_w < self.x - self.half_w
            || range.y - range.half_h > self.y + self.half_h
            || range.y + range.half_h < self.y - self.half_h)
    }
}
```
```rust
pub struct QuadTree {
    boundary: QuadBox,
    capacity: usize,
    points: Vec<QuadPoint>,
    divided: bool,
    northeast: Option<Box<QuadTree>>,
    northwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
}
```
```rust
impl fmt::Debug for QuadTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("QuadTree");
        s.field("boundary", &self.boundary)
            .field("capacity", &self.capacity)
            .field("points_len", &self.points.len())
            .field("divided", &self.divided);
        s.finish()
    }
}
```
```rust
impl QuadTree {
    pub fn new(boundary: QuadBox) -> Self {
        Self {
            boundary,
            capacity: 4,
            points: Vec::new(),
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let hw = self.boundary.half_w * 0.5;
        let hh = self.boundary.half_h * 0.5;
        self.northeast = Some(Box::new(QuadTree::new(QuadBox::new(
            x + hw,
            y - hh,
            hw,
            hh,
        ))));
        self.northwest = Some(Box::new(QuadTree::new(QuadBox::new(
            x - hw,
            y - hh,
            hw,
            hh,
        ))));
        self.southeast = Some(Box::new(QuadTree::new(QuadBox::new(
            x + hw,
            y + hh,
            hw,
            hh,
        ))));
        self.southwest = Some(Box::new(QuadTree::new(QuadBox::new(
            x - hw,
            y + hh,
            hw,
            hh,
        ))));
        self.divided = true;
    }
```
```rust
    /// Insert와 동일한 로직. 성공 시 true.
    pub fn insert(&mut self, p: QuadPoint) -> bool {
        if !self.boundary.contains(&p) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push(p);
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        if let Some(ne) = self.northeast.as_mut() {
            if ne.insert(p.clone()) {
                return true;
            }
        }
        if let Some(nw) = self.northwest.as_mut() {
            if nw.insert(p.clone()) {
                return true;
            }
        }
        if let Some(se) = self.southeast.as_mut() {
            if se.insert(p.clone()) {
                return true;
            }
        }
        if let Some(sw) = self.southwest.as_mut() {
            if sw.insert(p) {
                return true;
            }
        }

        false
    }
```
```rust
    /// AABB 범위 질의. found에 **복사본**을 넣는다.
    pub fn query(&self, range: &QuadBox, found: &mut Vec<QuadPoint>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for p in &self.points {
            if range.contains(p) {
                found.push(p.clone());
            }
        }

        if self.divided {
            if let Some(ne) = &self.northeast {
                ne.query(range, found);
            }
            if let Some(nw) = &self.northwest {
                nw.query(range, found);
            }
            if let Some(se) = &self.southeast {
                se.query(range, found);
            }
            if let Some(sw) = &self.southwest {
                sw.query(range, found);
            }
        }
    }
```
```rust
    /// 타입 필터 질의
    pub fn query_by_type(&self, range: &QuadBox, kind: &str, found: &mut Vec<QuadPoint>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for p in &self.points {
            if range.contains(p) && p.kind == kind {
                found.push(p.clone());
            }
        }

        if self.divided {
            if let Some(ne) = &self.northeast {
                ne.query_by_type(range, kind, found);
            }
            if let Some(nw) = &self.northwest {
                nw.query_by_type(range, kind, found);
            }
            if let Some(se) = &self.southeast {
                se.query_by_type(range, kind, found);
            }
            if let Some(sw) = &self.southwest {
                sw.query_by_type(range, kind, found);
            }
        }
    }
```
```rust
    /// id로 1개 삭제. 발견 시 true.
    pub fn remove(&mut self, id: i32) -> bool {
        if let Some(pos) = self.points.iter().position(|p| p.id == id) {
            self.points.remove(pos);
            return true;
        }

        if self.divided {
            if let Some(ne) = self.northeast.as_mut() {
                if ne.remove(id) {
                    return true;
                }
            }
            if let Some(nw) = self.northwest.as_mut() {
                if nw.remove(id) {
                    return true;
                }
            }
            if let Some(se) = self.southeast.as_mut() {
                if se.remove(id) {
                    return true;
                }
            }
            if let Some(sw) = self.southwest.as_mut() {
                if sw.remove(id) {
                    return true;
                }
            }
        }
        false
    }
```
```rust
    /// 범위 내 최근접 포인트 1개 찾기. 있으면 Some(QuadPoint)
    pub fn find_nearest_in_range(
        &self,
        range: &QuadBox,
        target_x: f64,
        target_y: f64,
    ) -> Option<QuadPoint> {
        let mut found = Vec::new();
        self.query(range, &mut found);

        let mut best: Option<(f64, QuadPoint)> = None;
        for p in found {
            let d = p.distance_to(target_x, target_y);
            match &mut best {
                None => best = Some((d, p)),
                Some((bd, bp)) => {
                    if d < *bd {
                        *bd = d;
                        *bp = p;
                    }
                }
            }
        }
        best.map(|(_, p)| p)
    }
```
```rust
    pub fn visualize(&self, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!(
            "{indent}Boundary Center: ({:.3}, {:.3}) Half=({:.3}, {:.3}) Points: {}",
            self.boundary.x,
            self.boundary.y,
            self.boundary.half_w,
            self.boundary.half_h,
            self.points.len()
        );
        for p in &self.points {
            println!(
                "{indent}  - ID: {} ({} / {}) at ({:.3}, {:.3})",
                p.id, p.name, p.kind, p.x, p.y
            );
        }
        if self.divided {
            if let Some(ne) = &self.northeast {
                ne.visualize(depth + 1);
            }
            if let Some(nw) = &self.northwest {
                nw.visualize(depth + 1);
            }
            if let Some(se) = &self.southeast {
                se.visualize(depth + 1);
            }
            if let Some(sw) = &self.southwest {
                sw.visualize(depth + 1);
            }
        }
    }
}
```
```rust
// -----------------------------
// 2) ON_QuadEntityDataNode / ON_QuadTreeEntity
// -----------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb2 {
    pub min: Point2D,
    pub max: Point2D,
}
```
```rust
impl Aabb2 {
    #[inline]
    pub fn contains(&self, p: &Point2D) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}
```
```rust
pub struct QuadEntityDataNode {
    pub min_pt: Point2D,
    pub max_pt: Point2D,
    pub element_indices: Vec<usize>,
    pub children: [Option<Box<QuadEntityDataNode>>; 4], // NE, NW, SE, SW
}
```
```rust
impl QuadEntityDataNode {
    pub fn new(min_pt: Point2D, max_pt: Point2D) -> Self {
        Self {
            min_pt,
            max_pt,
            element_indices: Vec::new(),
            children: [None, None, None, None],
        }
    }

    #[inline]
    pub fn contains(&self, p: &Point2D) -> bool {
        p.x >= self.min_pt.x && p.x <= self.max_pt.x && p.y >= self.min_pt.y && p.y <= self.max_pt.y
    }

    #[inline]
    pub fn bbox(&self) -> (Point2D, Point2D) {
        (self.min_pt, self.max_pt)
    }

    pub fn create_children(&mut self) {
        let cx = (self.min_pt.x + self.max_pt.x) * 0.5;
        let cy = (self.min_pt.y + self.max_pt.y) * 0.5;
        let center = Point2D { x: cx, y: cy };

        // NE
        self.children[0] = Some(Box::new(QuadEntityDataNode::new(center, self.max_pt)));
        // NW
        self.children[1] = Some(Box::new(QuadEntityDataNode::new(
            Point2D {
                x: self.min_pt.x,
                y: center.y,
            },
            Point2D {
                x: center.x,
                y: self.max_pt.y,
            },
        )));
        // SE
        self.children[2] = Some(Box::new(QuadEntityDataNode::new(
            Point2D {
                x: center.x,
                y: self.min_pt.y,
            },
            Point2D {
                x: self.max_pt.x,
                y: center.y,
            },
        )));
        // SW
        self.children[3] = Some(Box::new(QuadEntityDataNode::new(self.min_pt, center)));
    }
```
```rust
    pub fn clear_children(&mut self) {
        self.children = [None, None, None, None];
    }

    #[inline]
    fn is_leaf(&self) -> bool {
        self.children.iter().all(|c| c.is_none())
    }
}
```
```rust
pub struct QuadTreeEntity {
    all_points: Vec<Point2D>, // 입력을 보관(복사). 필요시 Arc<[Point2D]>로 바꿔도 OK
    root: Option<Box<QuadEntityDataNode>>,
    elements_to_analyze: HashSet<usize>,
    limit: usize,
}
```
```rust
impl QuadTreeEntity {
    pub fn new(all_points: Vec<Point2D>, limit: Option<usize>) -> Self {
        let mut root = None;
        let mut elements = HashSet::new();

        let limit_val = if let Some(l) = limit {
            l
        } else {
            1 + all_points.len() / 100
        };

        if !all_points.is_empty() {
            let mut minp = all_points[0];
            let mut maxp = all_points[0];
            for p in &all_points {
                if p.x < minp.x {
                    minp.x = p.x;
                }
                if p.y < minp.y {
                    minp.y = p.y;
                }
                if p.x > maxp.x {
                    maxp.x = p.x;
                }
                if p.y > maxp.y {
                    maxp.y = p.y;
                }
            }
            root = Some(Box::new(QuadEntityDataNode::new(minp, maxp)));
            for i in 0..all_points.len() {
                elements.insert(i);
            }
        }

        Self {
            all_points,
            root,
            elements_to_analyze: elements,
            limit: limit_val,
        }
    }

    pub fn build(&mut self) {
        let Some(mut root) = self.root.take() else {
            return;
        };

        // self 를 불변으로 빌릴 값/복사만 미리 꺼내두기
        let set = self.elements_to_analyze.clone();
        let limit = self.limit;
        let mut tri_count = 0;

        // 이제 self 에는 root 가 없으니(=None) 아래 호출에서 self 를 불변 대여해도 충돌 없음
        self.spatial_subdivision(&mut root, &set, limit, &mut tri_count);

        // 작업 끝나면 되돌려놓기
        self.root = Some(root);
    }

    pub fn root(&self) -> Option<&QuadEntityDataNode> {
        self.root.as_deref()
    }

    fn spatial_subdivision(
        &self,
        parent: &mut QuadEntityDataNode,
        index_set: &HashSet<usize>,
        max_count: usize,
        _tri_count: &mut i32,
    ) {
        if index_set.len() <= max_count {
            parent.element_indices = index_set.iter().copied().collect();
            return;
        }

        parent.create_children();

        let mut child_sets: [HashSet<usize>; 4] = [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];

        for &idx in index_set {
            let p = self.all_points[idx];
            for i in 0..4 {
                if let Some(child) = parent.children[i].as_ref() {
                    if child.contains(&p) {
                        child_sets[i].insert(idx);
                        break;
                    }
                }
            }
        }

        // 분할 효과 없으면(0 or 1개 자식만 사용) 취소하고 리프 처리
        let non_empty = child_sets.iter().filter(|s| !s.is_empty()).count();
        if non_empty <= 1 {
            parent.clear_children();
            parent.element_indices = index_set.iter().copied().collect();
            return;
        }

        for i in 0..4 {
            if let Some(child) = parent.children[i].as_mut() {
                if !child_sets[i].is_empty() {
                    self.spatial_subdivision(child, &child_sets[i], max_count, _tri_count);
                }
            }
        }
    }
}
```
```rust
// -----------------------------
// 3) 보조 함수: 교차/질의
// -----------------------------

#[inline]
pub fn on_box_intersects(min_b: Point2D, max_b: Point2D, min_q: Point2D, max_q: Point2D) -> bool {
    !(min_q.x > max_b.x || max_q.x < min_b.x || min_q.y > max_b.y || max_q.y < min_b.y)
}
```
```rust
/// 사각형 질의: 쿼드트리 빌더 노드(인덱스 보관)에서 범위 내 인덱스들을 찾는다.
pub fn on_quad_entity_query_node(
    node: &QuadEntityDataNode,
    min_query: Point2D,
    max_query: Point2D,
    all_points: &[Point2D],
    found: &mut Vec<usize>,
) {
    let node_min = node.min_pt;
    let node_max = node.max_pt;
    if !on_box_intersects(node_min, node_max, min_query, max_query) {
        return;
    }

    if node.is_leaf() {
        for &idx in &node.element_indices {
            let p = all_points[idx];
            if p.x >= min_query.x && p.x <= max_query.x && p.y >= min_query.y && p.y <= max_query.y
            {
                found.push(idx);
            }
        }
        return;
    }

    for i in 0..4 {
        if let Some(child) = node.children[i].as_ref() {
            on_quad_entity_query_node(child, min_query, max_query, all_points, found);
        }
    }
}
```
```rust

QuadTree API를 익히기 위한 초보자용 샘플 코드 여러 개와 그에 대응하는 테스트 코드 예시들입니다.  
Rust 초보자도 이해하기 쉽도록 간단한 예제부터 시작해 점차 기능을 확장해갑니다.


## 🧪 1. 샘플 코드: 기본 삽입과 시각화
```rust
fn basic_insert_and_visualize() {
    let boundary = QuadBox::new(0.0, 0.0, 10.0, 10.0);
    let mut qt = QuadTree::new(boundary);

    let p1 = QuadPoint::new(1, 1.0, 1.0, "A", "type1");
    let p2 = QuadPoint::new(2, -2.0, -3.0, "B", "type2");

    qt.insert(p1);
    qt.insert(p2);

    qt.visualize(0);
}
```


## 🧪 2. 샘플 코드: 범위 질의
```rust
fn range_query_example() {
    let mut qt = QuadTree::new(QuadBox::new(0.0, 0.0, 10.0, 10.0));
    for i in 0..10 {
        let p = QuadPoint::new(i, i as f64, i as f64, format!("P{i}"), "typeA");
        qt.insert(p);
    }

    let query_box = QuadBox::new(5.0, 5.0, 2.0, 2.0);
    let mut found = Vec::new();
    qt.query(&query_box, &mut found);

    println!("Found {} points in range", found.len());
}
```


## 🧪 3. 샘플 코드: 타입 필터 질의
```rust
fn type_filtered_query() {
    let mut qt = QuadTree::new(QuadBox::new(0.0, 0.0, 10.0, 10.0));
    qt.insert(QuadPoint::new(1, 1.0, 1.0, "A", "tree"));
    qt.insert(QuadPoint::new(2, 2.0, 2.0, "B", "rock"));
    qt.insert(QuadPoint::new(3, 3.0, 3.0, "C", "tree"));

    let query_box = QuadBox::new(2.0, 2.0, 5.0, 5.0);
    let mut found = Vec::new();
    qt.query_by_type(&query_box, "tree", &mut found);

    println!("Found {} tree points", found.len());
}
```


## 🧪 4. 샘플 코드: 최근접 점 찾기
```rust
fn nearest_point_example() {
    let mut qt = QuadTree::new(QuadBox::new(0.0, 0.0, 10.0, 10.0));
    qt.insert(QuadPoint::new(1, 1.0, 1.0, "A", "type"));
    qt.insert(QuadPoint::new(2, 5.0, 5.0, "B", "type"));
    qt.insert(QuadPoint::new(3, 9.0, 9.0, "C", "type"));

    let query_box = QuadBox::new(5.0, 5.0, 5.0, 5.0);
    let nearest = qt.find_nearest_in_range(&query_box, 4.5, 4.5);

    if let Some(p) = nearest {
        println!("Nearest point: {} at ({}, {})", p.name, p.x, p.y);
    }
}
```


## ✅ 테스트 코드 예시들

```rust
#[cfg(test)]
mod quad_tree_tests {
    use nurbslib::core::quadtree::{QuadBox, QuadPoint, QuadTree};


    fn p(id: i32, x: f64, y: f64, name: &str, kind: &str) -> QuadPoint {
        // QuadPoint 필드명이 `kind`가 아니라 `r#type`이라면 .kind -> .r#type 로 바꾸세요.
        QuadPoint {
            id,
            x,
            y,
            name: name.to_string(),
            kind: kind.to_string(),
        }
    }

    fn boundary_full() -> QuadBox {
        // 중심 (0,0), 반폭/반높이 10 → x,y ∈ [-10,10]
        QuadBox::new(0.0, 0.0, 10.0, 10.0)
    }

    fn sort_ids(v: &Vec<QuadPoint>) -> Vec<i32> {
        let mut ids: Vec<i32> = v.iter().map(|q| q.id).collect();
        // 정렬하고
        ids.sort_unstable(); // 안정 정렬이 필요하면 sort() 사용
        // 반환
        ids
    }

    #[allow(unused)]
    fn sort_points_in_place(v: &mut [QuadPoint]) {
        v.sort_by_key(|q| q.id);
    }
```
```rust
    #[test]
    fn insert_and_query_all() {
        let mut qt = QuadTree::new(boundary_full()); // new(boundary) 또는 new(boundary, capacity)
        assert!(qt.insert(p(1, -5.0, -5.0, "a", "A")));
        assert!(qt.insert(p(2, 5.0, -5.0, "b", "B")));
        assert!(qt.insert(p(3, -5.0, 5.0, "c", "A")));
        assert!(qt.insert(p(4, 5.0, 5.0, "d", "B")));
        // 경계 밖 → false
        assert!(!qt.insert(p(99, 30.0, 0.0, "out", "X")));

        let mut found = Vec::new();
        qt.query(&boundary_full(), &mut found);
        assert_eq!(sort_ids(&found), vec![1, 2, 3, 4]);
    }
```
```rust
    #[test]
    fn query_subrange() {
        let mut qt = QuadTree::new(boundary_full());
        qt.insert(p(1, -8.0, -8.0, "sw", "A"));
        qt.insert(p(2, -8.0, 1.0, "nw-ish", "B"));
        qt.insert(p(3, 2.0, -2.0, "se-ish", "A"));
        qt.insert(p(4, 7.0, 8.0, "ne", "A"));

        // x ∈ [-10,-2], y ∈ [-10,2] 범위만
        let range = QuadBox::new(-6.0, -4.0, 4.0, 6.0);
        let mut found = Vec::new();
        qt.query(&range, &mut found);
        assert_eq!(sort_ids(&found), vec![1, 2]);
    }
```
```rust
    #[test]
    fn query_by_type_filter() {
        let mut qt = QuadTree::new(boundary_full());
        qt.insert(p(1, -5.0, -5.0, "a", "A"));
        qt.insert(p(2, 5.0, -5.0, "b", "B"));
        qt.insert(p(3, -5.0, 5.0, "c", "A"));
        qt.insert(p(4, 5.0, 5.0, "d", "B"));

        let range = boundary_full();
        let mut found = Vec::new();
        qt.query_by_type(&range, "A", &mut found);
        assert_eq!(sort_ids(&found), vec![1, 3]);

        found.clear();
        qt.query_by_type(&range, "B", &mut found);
        assert_eq!(sort_ids(&found), vec![2, 4]);

        found.clear();
        qt.query_by_type(&range, "Z", &mut found);
        assert!(found.is_empty());
    }
```
```rust
    #[test]
    fn remove_point() {
        let mut qt = QuadTree::new(boundary_full());
        for i in 0..8 {
            assert!(qt.insert(p(
                i,
                (i as f64) - 3.5,
                (i as f64) - 3.5,
                "n",
                if i % 2 == 0 { "E" } else { "O" }
            )));
        }
        assert!(qt.remove(3));
        assert!(!qt.remove(3)); // 이미 삭제됨

        let mut found = Vec::new();
        qt.query(&boundary_full(), &mut found);
        let ids = sort_ids(&found);
        assert_eq!(ids, vec![0, 1, 2, 4, 5, 6, 7]);
    }
```
```rust
    #[test]
    fn find_nearest_in_range() {
        let mut qt = QuadTree::new(boundary_full());
        qt.insert(p(10, -4.0, -1.0, "p10", "A"));
        qt.insert(p(11, -3.0, 1.0, "p11", "A"));
        qt.insert(p(12, 2.0, 2.0, "p12", "B"));
        qt.insert(p(13, 8.0, 8.0, "p13", "B"));

        let range = QuadBox::new(0.0, 0.0, 5.0, 5.0); // x,y ∈ [-5,5]
        let _nearest = QuadPoint {
            id: -1,
            x: 0.0,
            y: 0.0,
            name: "".into(),
            kind: "".into(),
        };
        let nearest = qt
            .find_nearest_in_range(&range, 1.0, 1.0)
            .expect("Nearest not found");
        // (1,1)에서 가장 가까운 건 (2,2)=id 12 여야 함 (거리 √2)
        assert_eq!(nearest.id, 12);
    }
```
```rust
    #[test]
    fn heavy_insert_and_query() {
        // 많은 포인트를 넣어도 정확히 걸러지는지
        let mut qt = QuadTree::new(QuadBox::new(0.0, 0.0, 100.0, 100.0));
        let n = 2_000;
        for i in 0..n {
            let x = (i as f64 % 200.0) - 100.0;
            let y = ((i * 7) as f64 % 200.0) - 100.0;
            assert!(qt.insert(p(
                i as i32,
                x,
                y,
                "bulk",
                if i % 3 == 0 { "A" } else { "B" }
            )));
        }
        // 중심 근처 10×10 박스
        let range = QuadBox::new(0.0, 0.0, 5.0, 5.0);
        let mut found = Vec::new();
        qt.query(&range, &mut found);
        // 대략 100개 격자 중 일부—적어도 한두 개는 있어야 함
        assert!(!found.is_empty());
        // 타입 필터 함께 확인
        let mut found_a = Vec::new();
        qt.query_by_type(&QuadBox::new(0.0, 0.0, 5.0, 5.0), "A", &mut found_a);
        assert!(found_a.len() <= found.len());
    }
```
```rust
    #[test]
    fn reject_outside_inserts() {
        let mut qt = QuadTree::new(boundary_full());
        // 경계 살짝 밖
        assert!(!qt.insert(p(1, 10.1, 0.0, "o", "X")));
        assert!(!qt.insert(p(2, 0.0, -10.5, "o", "X")));

        // 내부는 OK
        assert!(qt.insert(p(3, 9.999, -9.999, "i", "X")));
        let mut found = Vec::new();
        qt.query(&boundary_full(), &mut found);
        assert_eq!(sort_ids(&found), vec![3]);
    }
```
```rust
    #[test]
    fn find_nearest_in_range2() {
        let mut qt = QuadTree::new(boundary_full());
        qt.insert(p(10, -4.0, -1.0, "p10", "A"));
        qt.insert(p(11, -3.0, 1.0, "p11", "A"));
        qt.insert(p(12, 2.0, 2.0, "p12", "B"));
        qt.insert(p(13, 8.0, 8.0, "p13", "B"));

        let range = boundary_full();
        let nearest = qt
            .find_nearest_in_range(&range, 1.0, 1.0)
            .expect("no point found");
        assert_eq!(nearest.id, 12);
    }

}
```
