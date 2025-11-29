
# Point Cloud, PCA 기반 피팅, KD-Tree 정리

본 문서는 **3D Point Cloud**, **PCA 기반 직선/평면 피팅**, **KD-Tree**, 그리고 관련 수식 기술

---

## 1. Point Cloud 정의

Point Cloud는 다음과 같은 점들의 집합이다.

$$
P = { p_i | p_i = (x_i, y_i, z_i) }
$$

점은 다음 형태로 표현된다.

$$
p_i = (x_i, y_i, z_i)
$$

Rust 구조:

```rust
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

---

## 2. PCA 기반 직선 피팅 (Line Fitting)

### 2.1 평균점 (Centroid)

$$
cx = (1/N) * sum(x_i)
$$

$$
cy = (1/N) * sum(y_i)
$$

$$
cz = (1/N) * sum(z_i)
$$

### 2.2 공분산 행렬 (3x3)

$$
S = (1/N) * sum( (p_i - c) * (p_i - c)^T )
$$

즉:

```
S = | Sxx  Sxy  Sxz |
    | Syx  Syy  Syz |
    | Szx  Szy  Szz |
```

### 2.3 고유값 문제 (Eigen)

직선 방향은 다음을 만족하는 **최대 고유값 λ_max 의 고유벡터 e_max**:

```
S * e_max = λ_max * e_max
```

### 2.4 결과

```
Line point = c  (centroid)
Line direction = e_max (unit vector)
```

---

## 3. PCA 기반 평면 피팅 (Plane Fitting)

평면은 다음 형태로 표현된다:

$$
n ⋅ (x - p0) = 0
$$

여기서:

- `n` = 법선 벡터
- `p0` = centroid

```rust
let mut cx = 0.0;
let mut cy = 0.0;
let mut cz = 0.0;
let n = points.len() as f64;

for p in points {
    cx += p.x;
    cy += p.y;
    cz += p.z;
}

cx /= n;
cy /= n;
cz /= n;
```


### 3.1 법선 벡터 구하기

공분산 행렬 S 의 **최소 고유값 λ_min**에 대응하는 고유벡터가 평면 법선이다.

```
S * n = λ_min * n
```

평면 방정식:

```
n.x * X + n.y * Y + n.z * Z + d = 0
```

```
d = -n ⋅ p0
```


---

## 4. KD-Tree 개념

KD-Tree는 "축 분할(Tree)" 형태의 공간 분할 자료구조.

### 4.1 축 선택

```
axis = depth mod 3
```

### 4.2 재귀 분할

```rust
points.sort_by_key(|p| p[axis])
median = points[N/2]
left  = points[0 .. median]
right = points[median+1 .. end]
```

### 4.3 최근접점(Nearest Neighbor) 검색 조건

거리 제곱:

$$
d2 = (px - qx)^2 + (py - qy)^2 + (pz - qz)^2
$$

분할 평면과의 거리:

```
axis_dist = (q[axis] - p[axis])
```

가지치기 조건:

```
axis_dist^2 < best_distance
```

---

## 5. Radius Search 공식

점 p_i 가 query q 에 대해 radius r 안에 있는지:

$$
d2 = (xi - qx)^2 + (yi - qy)^2 + (zi - qz)^2
$$

$$
if \quad d2 <= r^2 → inside
$$

---

## 6. K-Nearest Neighbor (KNN)

Binary Heap(max-heap) 사용:

```
heap.push( (dist2, point) )
if heap.len > k:
    heap.pop()   // 가장 먼 점 제거
```

최종적으로 오름차순 정렬:

```
sort by dist2 ASC
```

---

## 7. Rust 구현 개요

### 7.1 PointCloud

```rust
pub struct PointCloud {
    points: Vec<Point3D>,
}
```

### 7.2 KD-Tree

```rust
pub struct KdTree<P: Point> {
    pub root: Option<Box<KdNode<P>>>,
}
```

### 7.3 KD-Node

```rust
struct KdNode<P: Point> {
    point: P,
    axis: usize,
    left: Option<Box<KdNode<P>>>,
    right: Option<Box<KdNode<P>>>,
}
```
---

## 8. PCA 기반 피팅의 장점

- 매우 빠름 (O(N))
- 노이즈에 강함
- 직선 / 평면 모두 동일한 공분산 기반 수행

---

## 9. 활용 분야

- Surface Reconstruction Preprocessing  
- LIDAR / SLAM 기본 처리  
- Mesh Normal Estimation  
- Outlier Removal  
- 기하학 기반 클러스터링  

---

## KD Tree Source
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

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

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
    /// Find the nearest neighbor to a query point
    ///
    /// 반환값: `Some(&P)` (최근접 점에 대한 참조)
    pub fn nearest_neighbor(&self, query: &P) -> Option<(usize, f32)> {
        let root = self.root.as_ref()?;

        // 초기값: 루트 노드
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
    /// Recursive helper for nearest neighbor search
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

        // 먼저 탐색할 가지, 나중에 (필요하면) 탐색할 가지
        let (primary, secondary) = if query_pos[axis] < node_pos[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(child) = primary {
            Self::nearest_neighbor_recursive(child, query, best_index, best_distance_squared);
        }

        // 초평면에서의 거리로 가지치기
        let axis_distance = query_pos[axis] - node_pos[axis];
        if axis_distance * axis_distance < *best_distance_squared {
            if let Some(child) = secondary {
                Self::nearest_neighbor_recursive(child, query, best_index, best_distance_squared);
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

## Point Cloud 소스
```rust
use std::collections::HashMap;
use rand::Rng;
use crate::core::circle::Circle;
use crate::core::kd_tree::KdTree;
use crate::core::line::Line;
use crate::core::maths::on_get_sampling_3d;
use crate::core::plane::Plane;
use crate::core::prelude::{Point3D, Vector3D};
use crate::core::transform::Transform;

pub struct PointCloud {
    points: Vec<Point3D>,
}
```
```rust
impl PointCloud {
    pub fn new(pts: &Vec<Point3D>) -> PointCloud {
        let mut points: Vec<Point3D> = vec![];
        pts.iter().for_each(|x| points.push(x.clone()));
        Self { points }
    }
```
```rust
    pub fn from_vec(points: Vec<Point3D>) -> PointCloud {
        PointCloud { points }
    }
```
```rust
    /// Kd_Tree 로 변환 (Point3D 전용)
    pub fn to_kd_tree(&self) -> KdTree<Point3D> {
        KdTree::build(self.points())
    }
```
```rust
    pub fn points(&self) -> &Vec<Point3D> {
        &self.points
    }
```
```rust
    pub fn transform_by(&mut self, t: Transform) {
        for i in 0..self.points.len() {
            let new_origin = t.transform_point3d(&self.points[i]);
            self.points[i] = new_origin;
        }
    }
```
```rust
    pub fn fit_line(&self, pt: &mut Point3D, dir: &mut Vector3D) {
        let array: &[Point3D] = &self.points;

        if let Some(line) = Line::fit_from_points(array) {
            let p = line.start.clone();
            pt.clone_from(&p);
            dir.clone_from(&line.direction());
        }
    }
```
```rust
    pub fn fit_circle(&self, plane: &mut Plane, r: &mut f64) {
        let array: &[Point3D] = &self.points;
        if let Some(circle) = Circle::fit_from_points(array) {
            *r = circle.radius;
            plane.origin = circle.plane.origin.clone();
            plane.x_axis = circle.plane.x_axis.clone();
            plane.y_axis = circle.plane.y_axis.clone();
            plane.z_axis = circle.plane.z_axis.clone();
            plane.equation = circle.plane.equation.clone();
        }
    }
```
```rust
    pub fn fit_plane(&self, plane: &mut Plane) {
        let array: &[Point3D] = &self.points;
        if let Some(pln) = Plane::fit_from_points(array) {
            plane.origin = pln.origin.clone();
            plane.x_axis = pln.x_axis.clone();
            plane.y_axis = pln.y_axis.clone();
            plane.z_axis = pln.z_axis.clone();
            plane.equation = pln.equation.clone();
        }
    }
```
```rust
    pub fn get_sampling(&self) -> Vec<Point3D> {
        let array: &[Point3D] = &self.points;
        on_get_sampling_3d(array)
    }
}
```
```rust
//3-1. 기본 유틸: 길이, bbox
impl PointCloud {
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn bbox(&self) -> Option<(Point3D, Point3D)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min = Point3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for p in &self.points {
            if p.x < min.x { min.x = p.x; }
            if p.y < min.y { min.y = p.y; }
            if p.z < min.z { min.z = p.z; }

            if p.x > max.x { max.x = p.x; }
            if p.y > max.y { max.y = p.y; }
            if p.z > max.z { max.z = p.z; }
        }

        Some((min, max))
    }
}
```
```rust
//3-2. 최근접점 (선형 탐색, ON_GetClosestPointInPointList 대응)
impl PointCloud {
    pub fn closest_point_index_linear(
        &self,
        target: &Point3D,
        max_distance: Option<f64>,
    ) -> Option<usize> {
        if self.points.is_empty() {
            return None;
        }

        let mut best_i: Option<usize> = None;
        let mut best_d2 = max_distance
            .map(|d| d * d)
            .unwrap_or(f64::INFINITY);

        for (i, p) in self.points.iter().enumerate() {
            let dx = p.x - target.x;
            let dy = p.y - target.y;
            let dz = p.z - target.z;
            let d2 = dx * dx + dy * dy + dz * dz;

            if d2 < best_d2 {
                best_d2 = d2;
                best_i = Some(i);
            }
        }

        best_i
    }
}
```
```rust
//3-3. 랜덤 서브샘플링 (Random Sub sample 대응)
impl PointCloud {
    /// 균일 랜덤 서브샘플링
    pub fn random_sub_sample<R: Rng>(
        &self,
        target_count: usize,
        rng: &mut R,
    ) -> PointCloud {
        let n = self.points.len();

        if target_count == 0 || target_count >= n {
            // 그냥 복사해서 반환
            return PointCloud::new(&self.points);
        }

        let mut indices: Vec<usize> = (0..n).collect();
        let mut last = n;

        // ON_PointCloud::RandomSubsample와 같은 패턴
        let to_remove = n - target_count;
        for _ in 0..to_remove {
            let idx = rng.gen_range(0..last);
            indices.swap(idx, last - 1);
            last -= 1;
        }

        // 앞쪽 target_count 개만 사용
        let mut pts = Vec::with_capacity(target_count);
        for &i in &indices[0..target_count] {
            pts.push(self.points[i]);
        }

        PointCloud { points: pts }
    }
}
```
```rust
//3-4. 인덱스 범위 제거 (RemoveRange 대응)
impl PointCloud {
    /// 주어진 index 집합을 제거 (순서는 보장하지 않음)
    pub fn remove_indices(&mut self, indices: &[usize]) -> usize {
        if self.points.is_empty() || indices.is_empty() {
            return 0;
        }

        let n = self.points.len();

        // 복사 후 내림차순 정렬 + 중복 제거
        let mut idxs: Vec<usize> = indices
            .iter()
            .copied()
            .filter(|&i| i < n)
            .collect();

        if idxs.is_empty() {
            return 0;
        }

        idxs.sort_unstable();
        idxs.dedup();
        idxs.reverse(); // 큰 것부터

        let mut last = n;
        let mut removed = 0usize;

        for i in idxs {
            if i < last {
                self.points.swap(i, last - 1);
                removed += 1;
                last -= 1;
                if last == 0 {
                    break;
                }
            }
        }

        self.points.truncate(last);
        removed
    }
}
```
```rust
//3-5. Voxel 기반 다운샘플링 (CentroidDownsampling / CenterDownsampling 대응)
impl PointCloud {
    /// voxel 기반 centroid 다운샘플링
    pub fn voxel_downsample_centroid(&self, voxel_size: f64) -> PointCloud {
        self.voxel_downsample(voxel_size, true)
    }

    /// voxel 기반 center(셀 중심) 다운샘플링
    pub fn voxel_downsample_center(&self, voxel_size: f64) -> PointCloud {
        self.voxel_downsample(voxel_size, false)
    }

    fn voxel_downsample(&self, voxel_size: f64, centroid: bool) -> PointCloud {
        if self.points.is_empty() || voxel_size <= 0.0 {
            return PointCloud::new(&self.points);
        }

        let (min_pt, max_pt) = match self.bbox() {
            Some(bb) => bb,
            None => return PointCloud::new(&self.points),
        };

        // (i,j,k) -> (sum_x, sum_y, sum_z, count)
        type Acc = (f64, f64, f64, u32);
        let mut voxels: HashMap<(i32, i32, i32), Acc> = HashMap::new();

        let inv = 1.0 / voxel_size;

        for p in &self.points {
            let ix = ((p.x - min_pt.x) * inv).floor() as i32;
            let iy = ((p.y - min_pt.y) * inv).floor() as i32;
            let iz = ((p.z - min_pt.z) * inv).floor() as i32;
            let key = (ix, iy, iz);

            let entry = voxels.entry(key).or_insert((0.0, 0.0, 0.0, 0));
            entry.0 += p.x;
            entry.1 += p.y;
            entry.2 += p.z;
            entry.3 += 1;
        }

        let mut result = Vec::with_capacity(voxels.len());

        for ((ix, iy, iz), (sx, sy, sz, cnt)) in voxels {
            let c = cnt as f64;
            if centroid {
                // voxel 내부 포인트의 centroid
                result.push(Point3D::new(sx / c, sy / c, sz / c));
            } else {
                // voxel 중심점
                let cx = min_pt.x + (ix as f64 + 0.5) * voxel_size;
                let cy = min_pt.y + (iy as f64 + 0.5) * voxel_size;
                let cz = min_pt.z + (iz as f64 + 0.5) * voxel_size;
                result.push(Point3D::new(cx, cy, cz));
            }
        }

        PointCloud { points: result }
    }
}
```
```rust
impl PointCloud {
    pub fn centroid(&self) -> Option<Point3D> {
        if self.points.is_empty() {
            return None;
        }
        let mut sx = 0.0;
        let mut sy = 0.0;
        let mut sz = 0.0;
        let n = self.points.len() as f64;

        for p in &self.points {
            sx += p.x;
            sy += p.y;
            sz += p.z;
        }
        Some(Point3D::new(sx / n, sy / n, sz / n))
    }

    pub fn average_radius_from(&self, center: &Point3D) -> Option<f64> {
        if self.points.is_empty() {
            return None;
        }
        let mut sum = 0.0;
        for p in &self.points {
            sum += center.distance(p);
        }
        Some(sum / self.points.len() as f64)
    }
}
```
```rust
impl PointCloud {
    pub fn filter_in_bbox(&self, min: &Point3D, max: &Point3D) -> PointCloud {
        let mut pts = Vec::new();
        for p in &self.points {
            if p.x >= min.x && p.x <= max.x &&
                p.y >= min.y && p.y <= max.y &&
                p.z >= min.z && p.z <= max.z {
                pts.push(*p);
            }
        }
        PointCloud { points: pts }
    }
}
```

---

