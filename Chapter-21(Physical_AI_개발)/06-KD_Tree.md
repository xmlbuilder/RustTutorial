# 📌 KD-Tree란?
- K차원 공간을 분할하는 이진 트리 구조
- 각 노드는 특정 축(x, y, z 등)을 기준으로 데이터를 나눔
- 빠른 최근접 이웃 검색(NN), **범위 질의(Range Query)** 에 최적화

## 🧠 AI에서 KD-Tree 활용 사례
### 1️⃣ 최근접 이웃 검색 (Nearest Neighbor Search)
- 예: 이미지 검색, 추천 시스템, 유사한 문장 찾기
- KD-Tree로 벡터 공간에서 가장 가까운 포인트를 빠르게 찾음
- 대안: Ball Tree, Annoy, HNSW (KD-Tree는 중소규모에 적합)
### 2️⃣ 클러스터링 전처리
- K-Means, DBSCAN 등에서 거리 기반 탐색에 사용
- 예: DBSCAN에서 ε 범위 내 이웃 탐색 → KD-Tree로 빠르게 처리
### 3️⃣ 3D 공간 처리 / 컴퓨터 비전
- 포인트 클라우드(Point Cloud)에서 KD-Tree로 공간 인덱싱
- 예: LiDAR 데이터에서 특정 위치 주변 포인트 추출
### 4️⃣ 강화학습 / 로봇 경로 탐색
- 로봇이 이동 가능한 공간을 KD-Tree로 분할 → 빠른 경로 탐색
- 예: RRT(Rapidly-exploring Random Tree)에서 KD-Tree로 노드 연결
### 5️⃣ 자율주행 / 센서 융합
- 레이더/라이다 포인트를 KD-Tree로 인덱싱 → 실시간 객체 탐색
- 예: 차량 주변 3m 이내의 포인트만 빠르게 추출

### 🔧 Rust에서 KD-Tree 라이브러리
- : 고성능 KD-Tree 구현
- : 간단한 KD-Tree 라이브러리
- Python에서는 scikit-learn, scipy.spatial.KDTree가 대표적

## ✅ 결론
- KD-Tree는 AI에서 공간 기반 탐색, 유사도 검색, 센서 데이터 처리에 매우 유용.
- 특히 레이더/라이다 기반 자율주행, 추천 시스템, 클러스터링 전처리에서 핵심 도구로 쓰임.

# KD-Tree for 3D Point Clouds

이 문서는 `nurbslib::core::kd_tree::KdTree` 구현을 기준으로,  
KD-Tree의 **수학적 의미**, **자료구조**, **각 함수의 역할과 사용법** 을 정리한 것이다.

---

## 1. 개요 (Overview)

- KD-Tree(K-dimensional Tree)는 다차원 점 집합에 대해

    - 최근접 이웃 검색 (Nearest Neighbor Search)
    - k-최근접 이웃 검색 (k-NN)
    - 반경 검색 (Radius Search)

- 을 **O(log N)** 수준의 시간 복잡도로 수행하기 위한 공간 분할 자료구조이다.

- 여기서는 주로 **3D 포인트 클라우드** 를 대상으로 하며,  `Point` 트레잇을 통해 좌표를 추상화한다.

---

## 2. Point 트레잇과 Point3D

- KD-Tree는 `Point` 트레잇을 구현하는 타입에 대해 동작한다.

```rust
/// 3D 포인트를 추상화하는 트레잇 (예시)
pub trait Point {
    fn position(&self) -> [f32; 3];

    fn distance_squared_to(&self, other: &Self) -> f32 {
        let a = self.position();
        let b = other.position();
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        let dz = a[2] - b[2];
        dx*dx + dy*dy + dz*dz
    }
}
```

`Point3D` 구조체는 `Point` 트레잇을 다음과 같이 구현한다:

```rust
impl Point for Point3D {
    #[inline]
    fn position(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}
```

- 수학적으로, 두 점 `p = (px,py,pz)`, `q = (qx,qy,qz)` 사이의 **거리 제곱**은:

```
dist2(p, q) = (px - qx)^2 + (py - qy)^2 + (pz - qz)^2
```

- 실제 거리는:

```
dist(p, q) = sqrt(dist2(p, q))
```

KD-Tree 구현에서는 **비교를 빠르게 하기 위해 거리 제곱(dist2)을 일관되게 사용** 한다.

---

## 3. 자료구조 정의

### 3.1 KdNode

- 트리의 각 노드는 다음 정보를 가진다:

```rust
struct KdNode<P: Point> {
    point: P,
    index: usize,            // 원본 배열에서의 인덱스
    axis: usize,             // 분할 축 (0=x, 1=y, 2=z)
    left: Option<Box<KdNode<P>>>,
    right: Option<Box<KdNode<P>>>,
}
```

- `axis` : 이 노드에서 사용된 분할 평면은  
  `position()[axis] = const` 형태의 초평면이다.
- `left` : 해당 축에서 값이 더 작은 쪽 서브트리
- `right`: 해당 축에서 값이 더 큰 쪽 서브트리

### 3.2 KdTree

```rust
pub struct KdTree<P: Point> {
    root: Option<Box<KdNode<P>>>,
    len: usize, // 저장된 포인트 개수
}
```

- `root` : 트리의 루트 노드 (없으면 빈 트리)
- `len` : 전체 포인트 개수

---

## 4. 트리 생성: `build`

### 4.1 인터페이스

```rust
impl<P: Point> KdTree<P> {
    pub fn build(points: &[P]) -> Self
    where
        P: Clone,
    { /* ... */ }
}
```

- 입력: `points` – 포인트 슬라이스
- 출력: KD-Tree
- 제약: `P: Clone` (빌드 과정에서 포인트 복사)

### 4.2 알고리즘 개요

- 각 재귀 단계에서:
    - 1. 현재 서브 배열 `items` 와 깊이 `depth` 가 있을 때,
    - 2. 분할 축 `axis = depth % 3` (0=x, 1=y, 2=z) 를 선택한다.
    - 3. 이 축에 대해 **중앙값(median)** 을 찾기 위해 `select_nth_unstable_by` 로 `median` 위치를 선택한다.
    - 4. 그 중앙 요소를 루트로 하는 노드를 만들고,
    - 5. 왼쪽 절반으로부터 왼쪽 서브트리, 오른쪽 나머지 절반으로부터 오른쪽 서브트리를 재귀적으로 생성한다.

- 의사코드:

```rust
build_recursive(items, depth):
    if items is empty: return None

    axis = depth mod 3
    median = len(items) / 2

    items를 axis 기준으로 median 위치가 올바른 값이 되도록 분할

    left_items  = items[0 .. median]
    mid_item    = items[median]
    right_items = items[median+1 ..]

    node.point = mid_item.point
    node.index = mid_item.index
    node.axis  = axis
    node.left  = build_recursive(left_items, depth+1)
    node.right = build_recursive(right_items, depth+1)

    return node
```

### 4.3 복잡도

- 각 레벨에서 `select_nth_unstable_by` 가 평균 O(n)의 시간에 median을 선택
- 높이는 대략 O(log n) (평균적인 경우 균형)
- 전체 빌드 시간: **O(n log n)**

---

## 5. 최근접 점 탐색: `nearest_neighbor`

### 5.1 인터페이스 (예시)

- 여기서는 “인덱스 + 거리제곱” 을 반환하는 형태를 기준으로 설명한다:

```rust
pub fn nearest_neighbor(&self, query: &P) -> Option<(usize, f32)> {
    // Some((index, dist2))
}
```

- 입력: `query` – 찾고자 하는 기준 점
- 반환: `Some((original_index, distance_squared))` 또는 `None` (빈 트리)

### 5.2 탐색 알고리즘

- KD-Tree에서 최근접 점을 찾는 표준 알고리즘이다.

    - 1. 루트 노드부터 시작하여,    
        - `query.position()[axis]` 와 `node.point.position()[axis]` 를 비교하여  
            **query가 속할 가능성이 높은 가지(primary)** 를 먼저 내려간다.  

    - 2. 내려가는 동안, 각 노드에 대해:
        - `d2 = query.distance_squared_to(node.point)` 계산  
        - `d2 < best_d2` 이면, `best_d2` 와 `best_index` 를 갱신
    - 3. 재귀 호출이 돌아올 때,
        - 분할 초평면과의 축 거리 `axis_distance` 를 계산:
        ```rust
        axis_distance = query_pos[axis] - node_pos[axis]
        axis_d2 = axis_distance * axis_distance
        ```
        - `axis_d2 < best_d2` 인 경우에만 **반대편 서브트리(secondary)** 를 탐색:
            - 이는 최근접 점이 초평면의 다른 쪽에 있을 수 있는 경우를 보장하면서,
            - 필요 없는 서브트리들은 가지치기(pruning) 해서 성능을 높인다.

- 의사코드:

```rust
nearest(node, query, best_index, best_d2):
    if node == None: return

    d2 = dist2(query, node.point)
    if d2 < best_d2:
        best_d2 = d2
        best_index = node.index

    axis = node.axis
    qp = query.position()
    np = node.point.position()

    if qp[axis] < np[axis]:
        primary = node.left
        secondary = node.right
    else:
        primary = node.right
        secondary = node.left

    nearest(primary, query, best_index, best_d2)

    axis_distance = qp[axis] - np[axis]
    if axis_distance^2 < best_d2:
        nearest(secondary, query, best_index, best_d2)
```

### 5.3 거리와 거리제곱

- 구현에서는 항상 **거리제곱(dist2)** 를 비교하여 `sqrt` 연산을 피한다.
- 테스트 코드에서 실제 거리를 비교하고 싶다면:
  ```rust
  let (idx, d2) = tree.nearest_neighbor(&query).unwrap();
  let d = (d2 as f64).sqrt();
  ```

---

## 6. 반경 검색: `radius_search`

### 6.1 인터페이스

```rust
pub fn radius_search(&self, query: &P, radius: f32) -> Vec<(&P, f32)> {
    // Vec of (참조, 거리제곱)
}
```

- 입력:
  - `query` – 기준점
  - `radius` – 검색 반경 (거리)
- 출력:
  - 반경 내에 있는 모든 포인트에 대한 `(point_ref, dist2)` 벡터

### 6.2 알고리즘

- 1. `radius_squared = radius * radius` 를 구한다.
- 2. 재귀적으로 트리를 순회하면서:
    - 현재 노드와의 거리제곱 `d2` 를 계산하고,
    - `d2 <= radius_squared` 이면 결과에 추가.
- 3. 가지치기 조건:
    - 축 기준으로 `query_pos[axis] - radius` 와 `node_pos[axis]` 를 비교하여,
        - 왼쪽 서브트리 방향으로 반경이 겹치는 경우에만 `left` 탐색
    - 마찬가지로 `query_pos[axis] + radius` 와 비교하여 `right` 탐색 여부 결정

- 의사코드:

```rust
radius_search(node, query, radius, radius_squared):
    if node == None: return

    d2 = dist2(query, node.point)
    if d2 <= radius_squared:
        결과에 (node.point, d2) 추가

    axis = node.axis
    qp = query.position()
    np = node.point.position()

    if qp[axis] - radius <= np[axis]:
        radius_search(node.left, query, radius, radius_squared)

    if qp[axis] + radius >= np[axis]:
        radius_search(node.right, query, radius, radius_squared)
```

---

## 7. k-최근접 이웃 검색: `k_nearest`

### 7.1 인터페이스

```rust
pub fn k_nearest<'a>(&'a self, query: &P, k: usize) -> Vec<(&'a P, f32)> {
    // (point_ref, dist2) k개, dist2 오름차순
}
```

- 입력:
  - `query` – 기준점
  - `k` – 찾고자 하는 이웃 개수
- 출력:
  - 거리제곱 기준으로 가장 가까운 k개 포인트

### 7.2 알고리즘 (max-heap 사용)

- k-NN 검색에서는, 현재까지 탐색된 후보 중에서 **가장 먼 것** 을 빠르게 제거하고 싶다.  
    - 이를 위해 `BinaryHeap`(max-heap)을 사용한다.

- 1. 재귀 탐색 중 각 노드에서:
    - `d2 = dist2(query, node.point)` 계산
    - heap에 `(d2, &point)` push
    - heap 크기가 `k`를 초과하면 `heap.pop()` (가장 먼 후보 제거)
- 2. 가지치기:
    - 현재 heap에 k개가 꽉 차 있다면,
        - heap 루트의 `worst_dist2` 가 "현재까지 가장 먼 후보의 dist2"
    - 축 거리 `axis_distance^2` 가 `worst_dist2` 보다 크면,
        - 반대편 서브트리에는 k 최근접 후보가 있을 수 없으므로 탐색 생략

- 의사코드:

```rust
k_nearest(node, query, k, heap):
    if node == None: return

    d2 = dist2(query, node.point)
    heap.push((d2, &node.point))
    if heap.len > k:
        heap.pop()    // 가장 먼 것 제거

    axis = node.axis
    qp = query.position()
    np = node.point.position()

    if qp[axis] < np[axis]:
        primary = node.left
        secondary = node.right
    else:
        primary = node.right
        secondary = node.left

    k_nearest(primary, query, k, heap)

    worst_d2 = heap.peek().dist2 (또는 무한대)
    axis_distance = qp[axis] - np[axis]
    if axis_distance^2 < worst_d2:
        k_nearest(secondary, query, k, heap)
```

- 3. 탐색이 끝난 후 heap의 모든 요소를 꺼내어 `(point_ref, dist2)` 리스트로 만들고,  
    -`dist2` 오름차순으로 정렬하여 반환한다.

---

## 8. 보조 함수와 구현 디테일

### 8.1 `new`, `is_empty`, `len`

```rust
impl<P: Point> KdTree<P> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
```

- `new` : 빈 트리를 만든다.
- `is_empty` : 트리가 비어 있는지 확인
- `len` : 전체 점 개수

### 8.2 HeapEntry

- k-NN에서 사용하는 내부 구조체:

```rust
#[derive(Debug)]
struct HeapEntry<'a, P: Point> {
    dist2: f32,
    point: &'a P,
}

impl<'a, P: Point> Eq for HeapEntry<'a, P> {}
impl<'a, P: Point> PartialEq for HeapEntry<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        self.dist2.to_bits() == other.dist2.to_bits()
    }
}

impl<'a, P: Point> Ord for HeapEntry<'a, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist2.total_cmp(&other.dist2)
    }
}

impl<'a, P: Point> PartialOrd for HeapEntry<'a, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

- `BinaryHeap` 이 최대 힙이므로, `dist2` 가 클수록 “더 큰” 요소가 된다.
- 이 구조 덕분에 k개를 넘어설 때마다 `heap.pop()` 으로 가장 먼 후보를 자동으로 제거할 수 있다.

---

## 9. 수치 안정성과 구현 시 주의점

- 1. **거리 vs 거리제곱**  
    - 비교에는 항상 거리제곱을 사용해서 `sqrt` 호출을 피하고 성능을 높인다.
    - 실제 거리 값이 필요한 경우에만 마지막에 `sqrt(dist2)` 를 호출한다.

- 2. **`partial_cmp` / `total_cmp`**  
    - 좌표 비교, 거리 비교 시 `NaN` 처리가 들어가면 문제가 될 수 있다.
    - 이 구현에서는 좌표가 정상적인 유한 값(`f32`)이라는 가정을 두고 있다.

- 3. **균형도**  
    - median split 을 사용하므로 평균적으로 균형 잡힌 트리가 생성된다.
    - 입력 데이터가 매우 특이하게 정렬되어 있어도, `select_nth_unstable_by` 를 쓰기 때문에 평균적 성능은 유지된다.

- 4. **Point 트레잇 확장**  
    - 2D 용도로도 사용하고 싶다면, `position()` 을 `[f32; 2]` 로 재정의한 2D 전용 Point 트레잇을 만들거나,
    - 현재 3D 전용 트리를 그대로 두고, z=0 을 기본값으로 사용하는 방식 등으로 확장할 수 있다.

---

## 10. 요약

- KD-Tree는 3D 포인트 클라우드에서 **최근접 이웃, k-NN, 반경 검색**을 빠르게 수행하기 위한 자료구조이다.
- 이 구현은:
    - `Point` 트레잇으로 좌표를 추상화하고,
    - `build` 에서 median split으로 트리를 구성하며,
    - `nearest_neighbor`, `radius_search`, `k_nearest` 에서  
        거리제곱 기반의 재귀 탐색 + 가지치기를 사용한다.
- 거리 제곱을 일관되게 사용함으로써, 성능과 구현 단순성을 동시에 얻을 수 있다.



## KD-Treee 시각화
- LiDAR 포인트 클라우드에서 KD-Tree로 주변 탐색한 시각화 예제입니다.
- 기준점 주변 반경 0.2 이내의 이웃 포인트를 탐색하고 색상으로 구분.

![KD Tree Image](/image/kd_tree.png)


### 📌 시각화 설명
- 회색 점: 전체 LiDAR 포인트 (100개, 2D 평면에 무작위 생성)
- 빨간 점: 기준점 (50번째 포인트)
- 파란 점들: KD-Tree로 탐색된 반경 0.2 이내의 이웃 포인트들
- 이 구조는 실제 LiDAR 센서에서 특정 위치 주변의 포인트를 빠르게 찾을 때 사용됩니다.
- 예를 들어 자율주행 차량이 주변 장애물을 탐지하거나, 특정 객체의 경계를 추출할 때 유용.

## 🧠 AI 활용 포인트
- 센서 융합: 카메라/레이더와 함께 주변 포인트를 빠르게 추출
- 전처리: 관심 영역(ROI) 추출 → 학습 데이터 생성
- 이상 탐지: 특정 위치에 비정상적으로 밀집된 포인트 감지
- 클러스터링: DBSCAN 등에서 이웃 탐색에 KD-Tree 활용

---

## 소스 코드
```rust


//! KD-tree implementation for efficient nearest neighbor search
//!
//! This module provides a KD-tree data structure optimized for 3D point cloud
//! nearest neighbor queries.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::core::point::Point;
use crate::core::prelude::Point3D;

/// Point3D 에 대한 Point 트레잇 구현
impl Point for Point3D {
    #[inline]
    fn position(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}
```
```rust
/// KD-tree for efficient spatial queries
///
/// 이 구현은 `select_nth_unstable_by` 를 사용한 median split 으로 트리를 만들고,
/// 최근접/반경/k-NN 탐색을 지원합니다.
pub struct KdTree<P: Point> {
    // public 필드에서 private 타입을 사용하는 것은 금지되기 때문에
    // root 는 private 으로 두는 것이 맞습니다.
    root: Option<Box<KdNode<P>>>,
    len: usize,
}
```
```rust
/// Node in the KD-tree
struct KdNode<P: Point> {
    point: P,
    index: usize, // 원본 배열에서의 인덱스
    axis: usize, // 0=x, 1=y, 2=z
    left: Option<Box<KdNode<P>>>,
    right: Option<Box<KdNode<P>>>,
}
```
```rust
impl<P: Point> KdTree<P> {
    /// Create a new empty KD-tree
    pub fn new() -> Self {
        Self {
            root: None,
            len: 0,
        }
    }
```
```rust
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
```
```rust
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
```
```rust
    /// Build a KD-tree from a slice of points
    pub fn build(points: &[P]) -> Self
    where
        P: Clone,
    {
        // (point, index) 쌍으로 벡터 구성
        let mut items: Vec<(P, usize)> = points
            .iter()
            .cloned()
            .zip(0..points.len())
            .collect();

        let root = Self::build_recursive(&mut items[..], 0);

        Self {
            root,
            len: points.len(),
        }
    }
```
```rust
    /// Recursively build the KD-tree using nth-selection (O(n log n))
    fn build_recursive(items: &mut [(P, usize)], depth: usize) -> Option<Box<KdNode<P>>>
    where
        P: Clone,
    {
        if items.is_empty() {
            return None;
        }

        let axis = depth % 3;
        let median = items.len() / 2;

        // (P, usize) 에서 P의 position으로 비교
        items.select_nth_unstable_by(median, |a, b| {
            a.0.position()[axis]
                .partial_cmp(&b.0.position()[axis])
                .unwrap_or(Ordering::Equal)
        });

        let (left, right) = items.split_at_mut(median);
        let (median_item, right_rest) = right.split_first_mut().unwrap();

        let point = median_item.0.clone();
        let index = median_item.1;

        let left_child = Self::build_recursive(left, depth + 1);
        let right_child = Self::build_recursive(right_rest, depth + 1);

        Some(Box::new(KdNode {
            point,
            index,
            axis,
            left: left_child,
            right: right_child,
        }))
    }
```
```rust
    pub fn nearest_neighbor_with_distance(&self, query: &P) -> Option<(&P, f32)> {
        let root = self.root.as_ref()?;

        let mut best_node: &KdNode<P> = root;
        let mut best_d2: f32 = query.distance_squared_to(&root.point);

        Self::nearest_neighbor_point_recursive(root, query, &mut best_node, &mut best_d2);

        Some((&best_node.point, best_d2))
    }
```
```rust
    /// 인덱스 기반 최근접 탐색용 재귀 함수 (기존 로직 유지)
    fn nearest_neighbor_index_recursive(
        node: &KdNode<P>,
        query: &P,
        best_index: &mut usize,
        best_distance_squared: &mut f32,
    ) {
        let distance_squared = query.distance_squared_to(&node.point);

        if distance_squared < *best_distance_squared {
            *best_distance_squared = distance_squared;
            *best_index = node.index;
        }

        let axis = node.axis;
        let query_pos = query.position();
        let node_pos = node.point.position();

        let (primary, secondary) = if query_pos[axis] < node_pos[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(child) = primary {
            Self::nearest_neighbor_index_recursive(child, query, best_index, best_distance_squared);
        }

        let axis_distance = query_pos[axis] - node_pos[axis];
        if axis_distance * axis_distance < *best_distance_squared {
            if let Some(child) = secondary {
                Self::nearest_neighbor_index_recursive(child, query, best_index, best_distance_squared);
            }
        }
    }
```
```rust
    /// 최근접 점의 **원본 인덱스와 거리제곱**을 반환하는 버전
    /// (지금 구조에서 index 를 유지하고 싶을 때 사용)
    pub fn nearest_neighbor_index(&self, query: &P) -> Option<(usize, f32)> {
        let root = self.root.as_ref()?;

        let mut best_index = root.index;
        let mut best_distance_squared = query.distance_squared_to(&root.point);

        Self::nearest_neighbor_index_recursive(
            root,
            query,
            &mut best_index,
            &mut best_distance_squared,
        );

        Some((best_index, best_distance_squared))
    }
```
```rust
    /// 포인트 기반 최근접 탐색용 재귀 함수
    fn nearest_neighbor_point_recursive<'a>(
        node: &'a KdNode<P>,
        query: &P,
        best_node: &mut &'a KdNode<P>,
        best_distance_squared: &mut f32,
    ) {
        let distance_squared = query.distance_squared_to(&node.point);

        if distance_squared < *best_distance_squared {
            *best_distance_squared = distance_squared;
            *best_node = node;
        }

        let axis = node.axis;
        let query_pos = query.position();
        let node_pos = node.point.position();

        let (primary, secondary) = if query_pos[axis] < node_pos[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(child) = primary {
            Self::nearest_neighbor_point_recursive(child, query, best_node, best_distance_squared);
        }

        let axis_distance = query_pos[axis] - node_pos[axis];
        if axis_distance * axis_distance < *best_distance_squared {
            if let Some(child) = secondary {
                Self::nearest_neighbor_point_recursive(child, query, best_node, best_distance_squared);
            }
        }
    }
```
```rust
    /// Find all points within a given radius of the query point
    ///
    /// 반환값: `(점에 대한 참조, 거리제곱)` 의 벡터
    pub fn radius_search(&self, query: &P, radius: f32) -> Vec<(&P, f32)> {
        let mut results = Vec::with_capacity(64);
        let radius_squared = radius * radius;

        if let Some(ref root) = self.root {
            Self::radius_search_recursive(root, query, radius_squared, &mut results);
        }

        results
    }
```
```rust
    /// Recursive helper for radius search
    fn radius_search_recursive<'a>(
        node: &'a KdNode<P>,
        query: &P,
        radius_squared: f32,
        results: &mut Vec<(&'a P, f32)>,
    ) {
        let distance_squared = query.distance_squared_to(&node.point);

        if distance_squared <= radius_squared {
            results.push((&node.point, distance_squared));
        }

        let axis = node.axis;
        let query_pos = query.position();
        let node_pos = node.point.position();

        if let Some(left) = &node.left {
            if query_pos[axis] - radius_squared.sqrt() <= node_pos[axis] {
                Self::radius_search_recursive(left, query, radius_squared, results);
            }
        }

        if let Some(right) = &node.right {
            if query_pos[axis] + radius_squared.sqrt() >= node_pos[axis] {
                Self::radius_search_recursive(right, query, radius_squared, results);
            }
        }
    }
```
```rust
    /// Find the k nearest neighbors to a query point
    ///
    /// 반환값: `(점에 대한 참조, 거리제곱)` 의 벡터 (가까운 순으로 정렬)
    pub fn k_nearest<'a>(&'a self, query: &P, k: usize) -> Vec<(&'a P, f32)> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap: BinaryHeap<HeapEntry<'a, P>> = BinaryHeap::new();

        if let Some(ref root) = self.root {
            Self::k_nearest_recursive(root, query, k, &mut heap);
        }

        // Heap 은 max-heap 이므로, 작은 거리 순으로 정렬해서 반환
        let mut results: Vec<(&P, f32)> = heap.into_iter().map(|e| (e.point, e.dist2)).collect();

        results.sort_by(|a, b| a.1.total_cmp(&b.1));
        results
    }
```
```rust
    /// Recursive helper for k-nearest search
    fn k_nearest_recursive<'a>(
        node: &'a KdNode<P>,
        query: &P,
        k: usize,
        heap: &mut BinaryHeap<HeapEntry<'a, P>>,
    ) {
        let distance_squared = query.distance_squared_to(&node.point);

        // 후보 추가
        heap.push(HeapEntry {
            dist2: distance_squared,
            point: &node.point,
        });

        // k개 초과하면 가장 먼 것 제거
        if heap.len() > k {
            heap.pop();
        }

        let axis = node.axis;
        let query_pos = query.position();
        let node_pos = node.point.position();

        let (primary, secondary) = if query_pos[axis] < node_pos[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(child) = primary {
            Self::k_nearest_recursive(child, query, k, heap);
        }

        // 현재 heap에서 가장 먼 거리 (max-heap의 루트)
        let worst_dist2 = heap.peek().map(|e| e.dist2).unwrap_or(f32::INFINITY);

        // 초평면과의 거리로 가지치기
        let axis_distance = query_pos[axis] - node_pos[axis];
        if axis_distance * axis_distance < worst_dist2 {
            if let Some(child) = secondary {
                Self::k_nearest_recursive(child, query, k, heap);
            }
        }
    }
```
```rust
    /// 주어진 query 점에서 max_radius 이내에 있는 최근접 이웃을 찾는다.
    ///
    /// 반환값:
    /// - Some((index, dist2)) : dist2 = 거리 제곱, index = 원본 배열 인덱스
    /// - None                 : 반경 내에 어떤 점도 없을 때
    pub fn nearest_neighbor_with_radius(
        &self,
        query: &P,
        max_radius: f32,
    ) -> Option<(usize, f32)> {
        // 트리가 비었으면 바로 None
        let root = self.root.as_ref()?;

        // 반경이 0 이하이면 아무 것도 찾지 않음
        if !(max_radius > 0.0) {
            return None;
        }

        // 무한대 반경이면 기존 nearest_neighbor 를 그대로 사용
        if !max_radius.is_finite() {
            return self.nearest_neighbor(query);
        }

        // best_index 가 한 번도 갱신되지 않으면 "반경 내에 점이 없었다"는 뜻으로 사용
        let mut best_index: usize = usize::MAX;
        let mut best_dist2: f32 = max_radius * max_radius; // 반경 제곱이 초기 상한

        // 재귀 탐색
        Self::nearest_neighbor_with_radius_recursive(
            root,
            query,
            &mut best_index,
            &mut best_dist2,
        );

        if best_index == usize::MAX {
            None
        } else {
            Some((best_index, best_dist2))
        }

    }
```
```rust
    /// nearest_neighbor_with_radius 전용 재귀 함수
    fn nearest_neighbor_with_radius_recursive(
        node: &KdNode<P>,
        query: &P,
        best_index: &mut usize,
        best_dist2: &mut f32,
    ) {
        // 1) 현재 노드와의 거리 제곱 계산
        let d2 = query.distance_squared_to(&node.point);

        // 현재까지의 best_dist2(반경 제곱 이하) 보다 더 작으면 갱신
        if d2 < *best_dist2 {
            *best_dist2 = d2;
            *best_index = node.index;
        }

        // 2) 분할축 기준으로 primary / secondary 가지 결정
        let axis = node.axis;
        let qpos = query.position();
        let npos = node.point.position();

        let (primary, secondary) = if qpos[axis] < npos[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        // 3) primary 서브트리는 항상 먼저 탐색
        if let Some(child) = primary {
            Self::nearest_neighbor_with_radius_recursive(child, query, best_index, best_dist2);
        }

        // 4) secondary 서브트리는 "초평면과 구의 교차" 여부로 가지치기
        //
        //    축 방향 거리의 제곱이 현재 best_dist2 보다 작으면
        //    (즉, 구와 초평면이 교차 가능성이 있으면) secondary 도 탐색
        let axis_diff = qpos[axis] - npos[axis];
        let axis_dist2 = axis_diff * axis_diff;

        if axis_dist2 < *best_dist2 {
            if let Some(child) = secondary {
                Self::nearest_neighbor_with_radius_recursive(child, query, best_index, best_dist2);
            }
        }
    }
```
```rust
    /// 기존 전역 NN -> 무한대 반경에서의 최근접 이웃
    pub fn nearest_neighbor(&self, query: &P) -> Option<(usize, f32)> {
        let root = self.root.as_ref()?;

        let mut best_index = root.index;
        let mut best_distance_squared = query.distance_squared_to(&root.point);

        Self::nearest_neighbor_recursive(
            root,
            query,
            &mut best_index,
            &mut best_distance_squared,
        );

        Some((best_index, best_distance_squared))
    }
```
```rust
    // 기존 nearest_neighbor_recursive 는 그대로 두면 됨
    fn nearest_neighbor_recursive(
        node: &KdNode<P>,
        query: &P,
        best_index: &mut usize,
        best_distance_squared: &mut f32,
    ) {
        let distance_squared = query.distance_squared_to(&node.point);

        if distance_squared < *best_distance_squared {
            *best_distance_squared = distance_squared;
            *best_index = node.index;
        }

        let axis = node.axis;
        let query_pos = query.position();
        let node_pos = node.point.position();

        let (primary, secondary) = if query_pos[axis] < node_pos[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(child) = primary {
            Self::nearest_neighbor_recursive(child, query, best_index, best_distance_squared);
        }

        let axis_distance = query_pos[axis] - node_pos[axis];
        if axis_distance * axis_distance < *best_distance_squared {
            if let Some(child) = secondary {
                Self::nearest_neighbor_recursive(child, query, best_index, best_distance_squared);
            }
        }
    }

}
```
```rust
impl<P: Point> Default for KdTree<P> {
    fn default() -> Self {
        Self::new()
    }
}
```
```rust
/// 내부에서 사용하는 heap entry (k-NN 전용)
#[derive(Debug)]
struct HeapEntry<'a, P: Point> {
    dist2: f32,
    point: &'a P,
}
```
```rust
impl<'a, P: Point> Eq for HeapEntry<'a, P> {}
```
```rust
impl<'a, P: Point> PartialEq for HeapEntry<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        self.dist2.to_bits() == other.dist2.to_bits()
    }
}
```
```rust
impl<'a, P: Point> Ord for HeapEntry<'a, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        // f32 의 total_cmp 로 안정적인 순서화
        self.dist2.total_cmp(&other.dist2)
    }
}
```
```rust
impl<'a, P: Point> PartialOrd for HeapEntry<'a, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```
--- 
## 샘플 코드
```rust
#[cfg(test)]
mod tests_case1 {
    use nurbslib::core::kd_tree::KdTree;
    use nurbslib::core::prelude::Point3D;

    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }

    #[test]
    fn test_kdtree_build_point3d() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(2.0, 2.0, 2.0),
        ];

        let tree = KdTree::build(&points);
        assert!(!tree.is_empty());
    }
```
```rust
    #[test]
    fn test_nearest_neighbor_point3d() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(2.0, 2.0, 2.0),
        ];

        let tree = KdTree::build(&points);
        let query = Point3D::new(0.1, 0.1, 0.1);

        let result = tree.nearest_neighbor(&query);
        assert!(result.is_some());

        let nearest = result.unwrap();
        // Point3D -> f32 position() 비교
        println!("{:?}", nearest);
    }
```
```rust
    #[test]
    fn test_radius_search_point3d() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(10.0, 10.0, 10.0),
        ];

        let tree = KdTree::build(&points);
        let query = Point3D::new(0.0, 0.0, 0.0);

        let results = tree.radius_search(&query, 2.0);
        // (0,0,0) 과 (1,1,1) 두 개는 반경 2.0 이내
        assert_eq!(results.len(), 2);
    }
```
```rust
    #[test]
    fn test_k_nearest_point3d() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
            Point3D::new(3.0, 0.0, 0.0),
        ];

        let tree = KdTree::build(&points);
        let query = Point3D::new(0.9, 0.0, 0.0);

        let results = tree.k_nearest(&query, 2);
        assert_eq!(results.len(), 2);

        // 가장 가까운 두 개는 (1,0,0), (0,0,0) 순
        let (&p0, d0) = results[0];
        let (&p1, d1) = results[1];

        assert!(d0 <= d1);
        assert!(
            (p0.x == 1.0 && p1.x == 0.0) ||
                (p0.x == 0.0 && p1.x == 1.0)
        );
    }
```
```rust
    #[test]
    fn build_non_empty_tree() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 1.0, 1.0),
            p(2.0, 2.0, 2.0),
        ];

        let tree: KdTree<Point3D> = KdTree::build(&points);
        assert!(!tree.is_empty());
    }
```
```rust
    #[test]
    fn build_empty_tree() {
        let points: Vec<Point3D> = Vec::new();
        let tree: KdTree<Point3D> = KdTree::build(&points);
        assert!(tree.is_empty());
    }
```
```rust
    #[test]
    fn nearest_neighbor_empty_tree_returns_none() {
        let points: Vec<Point3D> = Vec::new();
        let tree: KdTree<Point3D> = KdTree::build(&points);

        let query = p(0.0, 0.0, 0.0);
        let nearest = tree.nearest_neighbor(&query);

        assert!(nearest.is_none());
    }
```
```rust
    #[test]
    fn radius_search_basic_two_hits() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 1.0, 1.0),
            p(10.0, 10.0, 10.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(0.0, 0.0, 0.0);
        let results = tree.radius_search(&query, 2.5); // sqrt(3) ~ 1.73

        // (0,0,0), (1,1,1) 두 개가 나와야 함
        assert_eq!(results.len(), 2);

        // 어떤 순서로 나와도 상관 없으니, 좌표로 확인
        let mut coords: Vec<(f64, f64, f64)> = results
            .iter()
            .map(|(pt, _d2)| (pt.x, pt.y, pt.z))
            .collect();
        coords.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        assert_eq!(coords[0], (0.0, 0.0, 0.0));
        assert_eq!(coords[1], (1.0, 1.0, 1.0));
    }
```
```rust
    #[test]
    fn radius_search_zero_radius_hits_exact_match_only() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(1.0, 0.0, 0.0);
        let results = tree.radius_search(&query, 0.0);

        // 반경 0 → 정확히 같은 점만
        assert_eq!(results.len(), 1);
        let (pt, d2) = results[0];
        assert!(d2.abs() < 1e-12);
        assert_eq!(pt.x, 1.0);
        assert_eq!(pt.y, 0.0);
        assert_eq!(pt.z, 0.0);
    }
```
```rust
    #[test]
    fn radius_search_large_radius_hits_all() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 2.0, 3.0),
            p(-4.0, 0.0, 1.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(0.0, 0.0, 0.0);
        let results = tree.radius_search(&query, 1000.0);

        assert_eq!(results.len(), points.len());
    }
```
```rust
    #[test]
    fn radius_search_no_hit() {
        let points = vec![
            p(100.0, 100.0, 100.0),
            p(-100.0, 50.0, 25.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(0.0, 0.0, 0.0);
        let results = tree.radius_search(&query, 1.0);

        assert!(results.is_empty());
    }
```
```rust
    // -------- k-NN 테스트 (k_nearest) --------
    #[test]
    fn k_nearest_basic_ordering() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(2.0, 0.0, 0.0),
            p(3.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(0.9, 0.0, 0.0);
        let results = tree.k_nearest(&query, 2);

        assert_eq!(results.len(), 2);

        // 거리가 증가하는 순으로 나와야 함
        let (p0, d0) = results[0];
        let (p1, d1) = results[1];
        assert!(d0 <= d1 + 1e-6);

        let d0_real = p0.distance(&query);
        let d1_real = p1.distance(&query);

        assert!((d0.sqrt() - d0_real as f32).abs() < 1e-4);
        assert!((d1.sqrt() - d1_real as f32).abs() < 1e-4);
    }
```
```rust
    #[test]
    fn k_nearest_k_larger_than_point_count() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(1.0, 0.0, 0.0);
        let results = tree.k_nearest(&query, 10);

        // 최대 개수는 points.len()
        assert_eq!(results.len(), 2);
    }
```
```rust
    #[test]
    fn k_nearest_k_zero_returns_empty() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 1.0, 1.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(0.0, 0.0, 0.0);
        let results = tree.k_nearest(&query, 0);

        assert!(results.is_empty());
    }
```
```rust
    #[test]
    fn k_nearest_all_same_point() {
        let points = vec![
            p(1.0, 2.0, 3.0),
            p(1.0, 2.0, 3.0),
            p(1.0, 2.0, 3.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(1.0, 2.0, 3.0);
        let results = tree.k_nearest(&query, 2);

        assert_eq!(results.len(), 2);
        for (pt, d2) in results {
            assert!((d2 as f64).abs() < 1e-12);
            assert_eq!(pt.x, 1.0);
            assert_eq!(pt.y, 2.0);
            assert_eq!(pt.z, 3.0);
        }
    }
}
```
```rust

#[cfg(test)]
mod tests_case2 {
    use nurbslib::core::geom::Point3D;
    use nurbslib::core::kd_tree::KdTree;
    // Point 트레잇 메서드(position)를 쓰기 위해 추가
    use nurbslib::core::point::Point;

    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }

    #[test]
    fn test_nearest_neighbor_point3d() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(2.0, 2.0, 2.0),
        ];

        let tree = KdTree::build(&points);
        let query = Point3D::new(0.1, 0.1, 0.1);

        let result = tree.nearest_neighbor_with_distance(&query);
        assert!(result.is_some());

        let nearest = result.unwrap();
        // Point3D -> f32 position() 비교 (Point 트레잇 메서드 사용)
        let pos = Point::position(nearest.0);
        assert_eq!(pos, [0.0f32, 0.0f32, 0.0f32]);
    }
```
```rust
    #[test]
    fn nearest_neighbor_exact_match() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 1.0, 1.0),
            p(2.0, 2.0, 2.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(1.0, 1.0, 1.0);
        let nearest = tree.nearest_neighbor_with_distance(&query).unwrap();

        assert_eq!(nearest.0.x, 1.0);
        assert_eq!(nearest.0.y, 1.0);
        assert_eq!(nearest.0.z, 1.0);
    }
```
```rust
    #[test]
    fn nearest_neighbor_simple_cloud() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(5.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(4.1, 0.0, 0.0);
        let nearest = tree.nearest_neighbor_with_distance(&query).unwrap();

        // 4.1 에서는 (5,0,0)이 가장 가깝다.
        assert_eq!(nearest.0.x, 5.0);
        assert_eq!(nearest.0.y, 0.0);
        assert_eq!(nearest.0.z, 0.0);
    }
```
```rust
    #[test]
    fn nearest_neighbor_symmetric_points() {
        let points = vec![
            p(-1.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(0.0, 0.0, 0.0);
        let nearest = tree.nearest_neighbor(&query).unwrap();

        // 대칭인 두 점 중 하나를 돌려주어야 함 (거리 같음)
        let dist = nearest.1;
        assert!((dist - 1.0).abs() < 1e-6);
    }
```
```rust
    // -------- 다양한 분포 테스트 --------
    #[test]
    fn nearest_neighbor_random_grid_points() {
        // 3x3x3 격자
        let mut points = Vec::new();
        for ix in 0..3 {
            for iy in 0..3 {
                for iz in 0..3 {
                    points.push(p(ix as f64, iy as f64, iz as f64));
                }
            }
        }
        let tree = KdTree::build(&points);

        // 격자 한 가운데에 가까운 점
        let query = p(0.9, 1.1, 0.9);

        // nearest: (뭐든, 거리제곱)
        let nearest = tree.nearest_neighbor(&query).unwrap();

        let expected = p(1.0, 1.0, 1.0);

        // expected 쪽은 "거리"
        let d_expected = expected.distance(&query);

        // nearest.1 은 거리제곱이므로 sqrt 해서 "거리"로 맞춰준다
        let d_nearest = (nearest.1 as f64).sqrt();

        println!("d_expected = {d_expected}, d_nearest = {d_nearest}");

        assert!((d_nearest - d_expected).abs() < 1e-6);
    }
}
```
```rust
#[cfg(test)]
mod tests_case3 {
    use nurbslib::core::geom::Point3D;
    use nurbslib::core::kd_tree::KdTree;

    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }
    #[test]
    fn nearest_neighbor_with_radius_hit() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(5.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(4.2, 0.0, 0.0);

        // 반경 2.0이면 (5,0,0)이 잡혀야 함 (거리 0.8)
        let res = tree.nearest_neighbor_with_radius(&query, 2.0).unwrap();
        let idx = res.0;
        assert_eq!(points[idx].x, 5.0);
    }
```
```rust
    #[test]
    fn nearest_neighbor_with_radius_miss() {
        let points = vec![
            p(0.0, 0.0, 0.0),
            p(5.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
        ];
        let tree = KdTree::build(&points);

        let query = p(4.2, 0.0, 0.0);

        // 반경 0.1이면 아무 점도 안 잡혀야 함
        let res = tree.nearest_neighbor_with_radius(&query, 0.1);
        assert!(res.is_none());
    }
}
```

---
