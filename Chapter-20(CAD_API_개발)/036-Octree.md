# Octree
Octree 구현은 공간 분할 기반의 자료구조로, 3차원 공간 내의 점(Point) 또는 삼각형(Triangle)을 효율적으로 저장하고 탐색하는 데 사용됩니다.  
아래에 수학적 원리와 이론을 기반으로 Octree의 핵심 개념을 정리.

🧠 Octree의 수학적 원리
### 1. 📦 공간 분할 (Spatial Partitioning)
Octree는 3차원 공간을 **축 기준으로 8개의 구역(octant)** 으로 재귀적으로 분할합니다.
- 각 노드는 AABB (Axis-Aligned Bounding Box)를 정의:

$$
\mathrm{Box}=[x_{\min },x_{\max }]\times [y_{\min },y_{\max }]\times [z_{\min },z_{\max }]
$$

- 중심점:

$$
c=\left( \frac{x_{\min }+x_{\max }}{2},\  \frac{y_{\min }+y_{\max }}{2},\  \frac{z_{\min }+z_{\max }}{2}\right)
$$

- 각 자식 노드는 이 중심점을 기준으로 8개의 구역으로 나뉨

### 2. 🧮 점 포함 여부 (Containment Test)
점 $p=(x,y,z)$ 가 AABB에 포함되는지 판단:

$$
x_{\min }-\varepsilon \leq x\leq x_{\max }+\varepsilon \\ y_{\min }-\varepsilon \leq y\leq y_{\max }+\varepsilon \\ z_{\min }-\varepsilon \leq z\leq z_{\max }+\varepsilon
$$

- 여기서 $\varepsilon =10^{-12}$ 는 수치 오차 허용 범위 (OCTREE_TOL)

### 3. 🧭 Octant 결정 (Octant Indexing)
점이 어느 자식 노드에 속하는지 결정:
```rust
if x > mid_x { oct |= 4; }
if y > mid_y { oct |= 2; }
if z > mid_z { oct |= 1; }
```

- 결과적으로 oct는 0~7 사이의 정수
- 이는 3비트로 표현된 공간 위치 인덱스

### 4. 🔁 재귀적 삽입 및 분할
- 노드에 점이 너무 많거나 깊이가 최대에 도달하지 않으면 subdivide() 호출
- 기존 점들을 각 octant로 재분배
- 이 과정은 트리 깊이 D에 대해 최대 8^D개의 노드 생성 가능

### 5. 🔍 탐색 및 근접 질의
- AABB vs Sphere 교차 여부:

$$
\mathrm{거리\ 제곱} = \sum_{i=x,y,z} 
\{
  \begin{array}{ll}
    (\min_i - c_i)^2 & \text{if } c_i < \min_i \\
    (c_i - \max_i)^2 & \text{if } c_i > \max_i \\
    0 & \text{otherwise}
  \end{array}
\}.
$$ 
- 이 값이 $r^2$ 이하이면 교차

- 근접 점 찾기:

$$
\mathrm{Euclidean\  distance^{\mathnormal{2}}}=(x-x_c)^2+(y-y_c)^2+(z-z_c)^2\leq r^2
$$

### 6. 🧩 삼각형 기반 Octree
- 삼각형의 AABB를 계산하여 공간에 삽입
- 삼각형이 하나의 octant에 완전히 포함되면 해당 자식에 삽입
- 그렇지 않으면 부모 노드에 유지


## 📌 Octree 파라미터 역할 요약

| 파라미터 이름     | 수학적 의미 또는 역할 설명                                                                 |
|------------------|---------------------------------------------------------------------------------------------|
| `max_depth`      | 깊이 $D$에 대해 최대 $8^D$개의 노드 생성 가능<br>→ 공간 분할의 최대 해상도 제한       |
| `max_points`     | 한 노드에 저장 가능한 최대 점 개수<br>→ 초과 시 자식 노드로 분할하여 공간 밀도에 따라 세분화     |
| `merge_threshold`| 자식 노드들의 총 점 개수가 이 값 이하이면 병합 수행<br>→ 희소한 공간을 단순화하여 트리 깊이 축소 |

```rust
Octree::new(
    OctreePoint::new(0.0, 0.0, 0.0),
    OctreePoint::new(10.0, 10.0, 10.0),
    max_depth = 4,
    max_points = 2,
    merge_threshold = 4,
);
```

- 깊이 4까지 분할 가능
- 각 노드는 최대 2개의 점만 저장
- 자식 노드들의 총 점이 4개 이하이면 병합

## ✅ 수학적 검증 요약

| 항목                     | 수학적 원리 또는 알고리즘 설명                                                                 |
|--------------------------|-----------------------------------------------------------------------------------------------|
| 공간 분할 (Octant 분기)   | 3D 공간을 중심점 기준으로 8개 구역으로 분할 (이진 분할 × 3축) → 정합성 있음                        |
| 포함 판정 (`contains`)   | AABB 내 점 포함 여부를 절대 오차 $\varepsilon = 10^{-12}$ 기준으로 판단 → 수치적 안정성 확보     |
| Octant 인덱스 계산       | 3비트 마스크 방식: $x > c_x, y > c_y, z > c_z$ → 정확한 공간 위치 인코딩                         |
| AABB vs Sphere 교차 판정 | 거리 제곱 기반의 최소 거리 계산 → $d^2 \leq r^2$ 조건으로 정확한 교차 여부 판정                   |
| 삼각형 AABB 계산         | 꼭짓점 좌표의 min/max로 AABB 생성 → 수학적으로 정확한 경계 상자 추정                              |
| 삼각형 Octant 분기       | 삼각형 AABB가 하나의 Octant에 완전히 포함될 경우만 분기 → 보수적이지만 수학적으로 타당               |



## ✅ 수학적으로 타당한 부분
### 1. OctreePoint 비교
- approx_eq는 $\varepsilon =10^{-12}$ 기준으로 좌표 비교 → 수치적 안정성 확보
- PartialEq 구현도 이 기준을 따름 → 정확한 동등성 판단
### 2. Octant 결정
- 3비트 마스크 방식으로 octant 계산:

$$
\mathrm{oct}=(x>x_{\mathrm{mid}})\cdot 4+(y>y_{\mathrm{mid}})\cdot 2+(z>z_{\mathrm{mid}})\cdot 1
$$
- → 공간 분할 논리적으로 정확

### 3. AABB vs Sphere 교차 판정
- 거리 제곱 계산 방식:

$$
d^2 = \sum_{i = x, y, z}
\{
  \begin{array}{ll}
    (\min_i - c_i)^2 & \text{if } c_i < \min_i \\
    (c_i - \max_i)^2 & \text{if } c_i > \max_i \\
    0 & \text{otherwise}
  \end{array}
\}.
$$

- → 정확한 구-박스 교차 판정

### 4. 삼각형 AABB 계산
- 각 꼭짓점의 min/max를 이용한 AABB 생성 → 수학적으로 정확

## ⚠️ 검토 및 개선 포인트
### 1. OctreePoint 비교 기준
- `OCTREE_TOL = 1e-12` 는 매우 작은 값 → 실수 오차에 민감
- 개선 제안: relative tolerance도 함께 고려하면 더 안정적

$$
|x-y|\leq \varepsilon \cdot \max (|x|,|y|,1.0)
$$

### 2. 삼각형 AABB vs Octant 포함 판정
- get_octant()에서 삼각형이 하나의 octant에 완전히 포함되는지 판단
- 현재는 AABB 기준으로만 판단 → 삼각형이 실제로는 걸쳐 있을 수 있음
- 개선 제안: 삼각형의 실제 면적 또는 중심점 기반 분할 고려 가능

### 3. 병합 조건
- try_merge()는 자식 노드가 모두 leaf이고 총 점 개수가 merge_threshold 이하일 때 병합
- 수학적으로는 타당하지만, 병합 후 공간 분포가 왜곡될 수 있음
- 개선 제안: 병합 후 AABB 재조정 또는 분포 기반 병합 조건 추가 고려

### 4. 삼각형 분할 기준
- get_octant()에서 삼각형이 하나의 octant에 완전히 포함되지 않으면 부모에 유지
- 이는 정확하지만, 공간 효율성 측면에서 일부 삼각형이 과도하게 상위 노드에 남을 수 있음
- 개선 제안: 삼각형을 여러 octant에 중복 삽입하거나 BSP 기반 분할 고려 가능

## 📌 결론

| 항목                         | 평가 또는 요약 설명                                                   |
|------------------------------|------------------------------------------------------------------------|
| 공간 분할 구조               | ✅ 3D 공간을 8개 octant로 정확하게 분할함                              |
| 포함 및 교차 판정            | ✅ AABB 기반의 수학적 조건으로 안정적이고 정확한 판정 수행             |
| 삼각형 AABB 처리             | ✅ 꼭짓점 기반 min/max 계산으로 수학적으로 타당한 경계 상자 생성       |
| 병합 조건                    | ⚠️ 자식 노드의 점 개수 기준 병합은 단순하지만 공간 분포 고려는 부족함   |
| 수치 비교 안정성            | ⚠️ 절대 오차만 사용 → 상대 오차 기반 비교 추가 시 더 견고해질 수 있음   |
| 삼각형 분기 전략             | ⚠️ AABB 기준 단일 octant 포함 여부만 판단 → 중복 삽입 전략 고려 가능     |


## 📦 points와 children의 역할과 영향

| 요소            | 기준 또는 상태         | 구조적/성능적 영향 설명                                               |
|-----------------|------------------------|------------------------------------------------------------------------|
| `points.len()`  | `< max_points`         | 현재 노드에 점을 계속 저장 → 분할 없이 leaf 유지                       |
| `points.len()`  | `≥ max_points`         | 분할 트리거 → 자식 노드 생성 및 점 분배 → 트리 깊이 증가               |
| `children.len()`| `Some / None` 상태     | `Some`이면 내부 노드 → 탐색 시 분기 발생<br>`None`이면 leaf 노드 → 병합 가능성 있음 |


### 🔍 핵심 요약
- points.len()은 분할 여부 결정
- children의 상태는 탐색 경로와 병합 가능성 결정
- 이 두 요소는 Octree의 공간 효율성과 탐색 성능을 좌우합니다


### 🧠 수학적 및 구조적 영향
#### 1. points.len()이 max_points보다 작을 때
- 현재 노드에 점을 계속 저장
- 공간 분할 없이 트리 깊이 증가 없음
- 탐색은 빠르지만 공간 밀도가 높아지면 성능 저하 가능
#### 2. points.len()이 max_points 이상일 때
- subdivide() 호출 → 8개의 자식 노드 생성
- 기존 점들을 get_octant() 기준으로 분배
- 트리 깊이 증가 → 탐색 경로 길어짐
#### 3. children이 모두 None일 때
- 현재 노드는 leaf 노드
- 병합 가능성 있음 (→ try_merge() 조건 만족 시)
#### 4. children 중 일부가 Some일 때
- 현재 노드는 내부 노드
- 탐색 시 get_octant()로 자식 노드로 이동
- 점은 자식 노드에 분산됨 → 공간 효율성 증가

## 🔁 병합 조건 (try_merge())
### 병합이 일어나는 조건:
- 모든 children이 Some이고 그 내부는 leaf여야 함
- 자식 노드들의 points.len() 합이 merge_threshold 이하
- 병합 시 자식 노드 제거 + 점들을 현재 노드로 통합
    - 병합 후 children은 모두 None, points는 증가

## 📌 정리

| 조건 또는 상태               | 구조적/수학적 의미 또는 영향 설명                                      |
|-----------------------------|------------------------------------------------------------------------|
| `points.len() < max_points` | 현재 노드에 점을 저장하며 분할하지 않음 → 공간 밀도가 낮거나 초기 상태 |
| `points.len() ≥ max_points` | 자식 노드로 분할 발생 (`subdivide`) → 공간 밀도 높음, 트리 깊이 증가     |
| `children` 모두 `None`      | 현재 노드는 leaf 상태 → 병합 가능성 있음                               |
| `children` 중 일부 `Some`   | 현재 노드는 내부 노드 → 탐색 시 분기 발생, 점은 자식에 분산됨           |
| `merge_threshold` 조건 만족 | 자식 노드들의 총 점 개수가 이 값 이하이면 병합 수행 → 트리 깊이 감소     |


## 📌 Octree 성능 트레이드오프 요약

| 요소       | 시간 복잡도 | 영향 요약                                      |
|------------|-------------|------------------------------------------------|
| `points`   | $O(N)$  | 한 노드에 점이 많으면 선형 탐색 비용 증가 → 탐색/삭제 성능 저하 |
| `children` | $O(D)$  | 트리 깊이가 깊어질수록 탐색 경로 길어짐 → 재귀 호출 비용 증가     |
### 🔍 설명 보충
- N: 한 노드에 저장된 점의 개수
- D: 트리의 깊이 (depth)
- points는 leaf 노드에서 직접 비교하는 대상
- children은 탐색 시 분기 경로를 따라 내려가는 깊이


### 🧠 왜 이런 일이 발생할까?
### 1. points가 많을 때
- 분할이 일어나지 않으면 한 노드에 점이 계속 쌓임
- 탐색 시 points.iter().any(...)로 선형 탐색
- 즉, 공간 분할이 부족하면 탐색이 선형 시간으로 퇴화
### 2. children이 너무 깊을 때
- max_points가 너무 작거나 max_depth가 너무 크면 과도한 분할 발생
- 탐색 시 get_octant()을 따라 재귀적으로 내려감
- 즉, 공간 분할이 과도하면 탐색 경로가 길어져 느려짐

## ⚖️ 성능 균형을 위한 전략

| 파라미터         | 조절 효과 또는 전략 설명                                                                 |
|------------------|------------------------------------------------------------------------------------------|
| `max_points`     | 값을 크게 하면 분할 빈도 감소 → 트리 깊이 얕아짐, leaf당 점 수 증가 → 탐색은 느려질 수 있음 |
| `max_depth`      | 값을 작게 하면 과도한 분할 방지 → 트리 구조 단순화, 탐색 경로 짧아짐                       |
| `merge_threshold`| 값을 크게 하면 병합이 잘 일어남 → 희소한 공간에서 트리 깊이 줄이고 메모리 절약 가능         |

## 🔍 전략 요약
- 데이터가 밀집된 경우 → max_points를 작게 설정해 세밀하게 분할
- 데이터가 희소한 경우 → merge_threshold를 크게 설정해 병합 유도
- 탐색 성능이 중요한 경우 → max_depth를 적절히 제한해 경로 길이 최소화


## ✅ 결론
- points가 많으면 한 노드 내 비교 횟수 증가 → 선형 탐색 비용
- children이 깊으면 탐색 경로 길어짐 → 재귀 호출 비용 증가
- 따라서, 적절한 분할과 병합 기준이 Octree의 성능을 결정합니다


---
# FEM

유한 요소(FEM) 절점이 공간에 고르게 분포되어 있는 경우,  
Octree의 분할/병합 파라미터를 어떻게 설정하면 효율적인지를 수학적 관점에서 아래에 정리.

## 📌 전제 조건: 고르게 분포된 절점
- 절점들이 균일한 격자 또는 등간격으로 배치됨
- 특정 영역에 밀집되지 않음 → 공간 밀도 변화가 적음
- 탐색, 근접 질의, 병합이 자주 발생할 수 있음

## ⚙️ 효율적 파라미터 설정 전략 (고르게 분포된 절점 기준)

| 파라미터         | 추천 설정 범위     | 조정 효과 및 전략 설명                                                                 |
|------------------|--------------------|----------------------------------------------------------------------------------------|
| `max_points`     | 중간~조금 큼 (8~16) | 점이 균일하게 분포되어 있으므로 과도한 분할을 피하고 leaf당 탐색 효율을 유지함           |
| `max_depth`      | 중간 수준 (4~6)     | 공간 해상도는 확보하면서도 트리 깊이를 제한하여 탐색 경로 길이를 최소화함               |
| `merge_threshold`| 작게 (2~4)          | 병합이 자주 일어날 필요가 없으므로 병합 조건을 엄격히 하여 트리 구조의 안정성 유지       |

### 🔍 요약 전략
- 분할은 적절히, 병합은 보수적으로
- leaf 노드에 점이 너무 많지 않도록 하면서도 불필요한 분할은 억제
- 탐색 경로가 너무 깊어지지 않도록 max_depth를 제한


### 🧠 수학적 근거
#### 1. 분할 필요성 ↓
- 고르게 분포된 경우, 각 노드에 점이 균일하게 들어오므로 분할 빈도가 낮아도 충분
- \mathrm{points.len()}<\mathrm{max_points} 상태가 자주 유지됨
#### 2. 병합 필요성 ↓
- 자식 노드들이 고르게 채워지므로 병합 조건을 자주 만족하지 않음
- 병합이 자주 일어나면 오히려 트리 구조가 흔들릴 수 있음
#### 3. 탐색 효율 ↑
- 깊이가 너무 깊으면 탐색 경로가 길어짐 → max_depth는 제한
- leaf당 점 개수가 적당하면 탐색 시 선형 비교 비용도 낮음

### ✅ 예시 설정
```rust
Octree::new(
    OctreePoint::new(0.0, 0.0, 0.0),
    OctreePoint::new(100.0, 100.0, 100.0),
    max_depth = 5,
    max_points = 12,
    merge_threshold = 3,
);
```
- 고르게 분포된 절점에 적합한 설정
- 분할은 적절히 발생하고, 병합은 최소화
- 탐색 성능과 메모리 효율의 균형 유지

----

```rust
use crate::math::boundingbox::BoundingBox;
use crate::math::point3d::Point3D;
use std::f64;
use std::mem;

const OCTREE_TOL: f64 = ON_TOL9;
```

```rust
//-----------------------------
// ON_OctreePoint
//-----------------------------
#[derive(Clone, Copy, Debug)]
pub struct OctreePoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```
```rust
impl OctreePoint {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= OCTREE_TOL
            && (self.y - other.y).abs() <= OCTREE_TOL
            && (self.z - other.z).abs() <= OCTREE_TOL
    }
}
```
```rust
impl PartialEq for OctreePoint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other)
    }
}
impl Eq for OctreePoint {}
```
```rust
//-----------------------------
// ON_OctreeNode
//-----------------------------
pub struct OctreeNode {
    pub min_point: OctreePoint,
    pub max_point: OctreePoint,
    pub points: Vec<OctreePoint>,
    pub children: [Option<Box<OctreeNode>>; 8],

    pub depth: i32,
    pub max_depth: i32,
    pub max_points: i32,
    pub merge_threshold: i32,
}
```
```rust
impl OctreeNode {
    pub fn new(
        min_pt: OctreePoint,
        max_pt: OctreePoint,
        depth: i32,
        max_depth: i32,
        max_points: i32,
        merge_threshold: i32,
    ) -> Self {
        Self {
            min_point: min_pt,
            max_point: max_pt,
            points: Vec::new(),
            children: Default::default(), // [None; 8]
            depth,
            max_depth,
            max_points,
            merge_threshold,
        }
    }
```
```rust
    #[inline]
    pub fn contains(&self, p: &OctreePoint) -> bool {
        let in_x = p.x >= self.min_point.x - OCTREE_TOL && p.x <= self.max_point.x + OCTREE_TOL;
        let in_y = p.y >= self.min_point.y - OCTREE_TOL && p.y <= self.max_point.y + OCTREE_TOL;
        let in_z = p.z >= self.min_point.z - OCTREE_TOL && p.z <= self.max_point.z + OCTREE_TOL;
        in_x && in_y && in_z
    }
```
```rust
    #[inline]
    pub fn get_octant(&self, p: &OctreePoint) -> usize {
        let mid_x = 0.5 * (self.min_point.x + self.max_point.x);
        let mid_y = 0.5 * (self.min_point.y + self.max_point.y);
        let mid_z = 0.5 * (self.min_point.z + self.max_point.z);
        let mut oct = 0usize;
        if p.x > mid_x {
            oct |= 4;
        }
        if p.y > mid_y {
            oct |= 2;
        }
        if p.z > mid_z {
            oct |= 1;
        }
        oct
    }
```
```rust
    pub fn subdivide(&mut self) {
        let mid_x = 0.5 * (self.min_point.x + self.max_point.x);
        let mid_y = 0.5 * (self.min_point.y + self.max_point.y);
        let mid_z = 0.5 * (self.min_point.z + self.max_point.z);

        let mins = [
            OctreePoint::new(self.min_point.x, self.min_point.y, self.min_point.z),
            OctreePoint::new(self.min_point.x, self.min_point.y, mid_z),
            OctreePoint::new(self.min_point.x, mid_y, self.min_point.z),
            OctreePoint::new(self.min_point.x, mid_y, mid_z),
            OctreePoint::new(mid_x, self.min_point.y, self.min_point.z),
            OctreePoint::new(mid_x, self.min_point.y, mid_z),
            OctreePoint::new(mid_x, mid_y, self.min_point.z),
            OctreePoint::new(mid_x, mid_y, mid_z),
        ];
        let maxs = [
            OctreePoint::new(mid_x, mid_y, mid_z),
            OctreePoint::new(mid_x, mid_y, self.max_point.z),
            OctreePoint::new(mid_x, self.max_point.y, mid_z),
            OctreePoint::new(mid_x, self.max_point.y, self.max_point.z),
            OctreePoint::new(self.max_point.x, mid_y, mid_z),
            OctreePoint::new(self.max_point.x, mid_y, self.max_point.z),
            OctreePoint::new(self.max_point.x, self.max_point.y, mid_z),
            OctreePoint::new(self.max_point.x, self.max_point.y, self.max_point.z),
        ];

        for i in 0..8 {
            let child = OctreeNode::new(
                mins[i],
                maxs[i],
                self.depth + 1,
                self.max_depth,
                self.max_points,
                self.merge_threshold,
            );
            self.children[i] = Some(Box::new(child));
        }
    }
```
```rust
    pub fn insert(&mut self, point: OctreePoint) {
        if !self.contains(&point) {
            return;
        }
        if (self.points.len() as i32) < self.max_points || self.depth >= self.max_depth {
            self.points.push(point);
            return;
        }

        if self.children[0].is_none() {
            self.subdivide();
            // 기존 점들을 자식으로 분배
            let mut old = Vec::new();
            mem::swap(&mut old, &mut self.points);
            for p in old {
                let oct = self.get_octant(&p);
                if let Some(ch) = self.children[oct].as_mut() {
                    ch.insert(p);
                }
            }
        }

        let oct = self.get_octant(&point);
        if let Some(ch) = self.children[oct].as_mut() {
            ch.insert(point);
        }
    }
```
```rust
    pub fn remove(&mut self, point: &OctreePoint) -> bool {
        if !self.contains(point) {
            return false;
        }

        // 현재 노드에 있는가?
        if let Some(pos) = self.points.iter().position(|p| p.approx_eq(point)) {
            self.points.swap_remove(pos);
            return true;
        }

        // 자식에서 제거
        for ch in self.children.iter_mut() {
            if let Some(node) = ch {
                if node.remove(point) {
                    self.try_merge();
                    return true;
                }
            }
        }
        false
    }
```
```rust
    fn is_pure_leaf(node: &OctreeNode) -> bool {
        node.children.iter().all(|c| c.is_none())
    }
```
```rust
    pub fn try_merge(&mut self) {
        if self.children[0].is_none() {
            return;
        }
        // 손자 있으면 병합 금지
        for ch in self.children.iter() {
            if let Some(n) = ch {
                if !Self::is_pure_leaf(n) {
                    return;
                }
            }
        }

        let mut total = 0i32;
        for ch in self.children.iter() {
            if let Some(n) = ch {
                total += n.points.len() as i32;
            }
        }

        if total <= self.merge_threshold {
            for ch in self.children.iter_mut() {
                if let Some(n) = ch.take() {
                    self.points.extend(n.points.into_iter());
                }
            }
        }
    }
```
```rust
    pub fn search(&self, point: &OctreePoint) -> bool {
        if !self.contains(point) {
            return false;
        }
        if self.points.iter().any(|p| p.approx_eq(point)) {
            return true;
        }
        for ch in self.children.iter() {
            if let Some(n) = ch {
                if n.search(point) {
                    return true;
                }
            }
        }
        false
    }
```
```rust
    #[inline]
    pub fn aabb_intersects_sphere(&self, c: &OctreePoint, r: f64) -> bool {
        fn axis(v: f64, minv: f64, maxv: f64) -> f64 {
            if v < minv {
                (minv - v) * (minv - v)
            } else if v > maxv {
                (v - maxv) * (v - maxv)
            } else {
                0.0
            }
        }
        let mut d2 = 0.0;
        d2 += axis(c.x, self.min_point.x, self.max_point.x);
        d2 += axis(c.y, self.min_point.y, self.max_point.y);
        d2 += axis(c.z, self.min_point.z, self.max_point.z);
        d2 <= r * r
    }
```
```rust
    pub fn find_nearby_points(
        &self,
        center: &OctreePoint,
        radius: f64,
        out: &mut Vec<OctreePoint>,
    ) {
        if !self.aabb_intersects_sphere(center, radius) {
            return;
        }
        let r2 = radius * radius;
        for p in &self.points {
            let dx = p.x - center.x;
            let dy = p.y - center.y;
            let dz = p.z - center.z;
            if dx * dx + dy * dy + dz * dz <= r2 {
                out.push(*p);
            }
        }
        for ch in self.children.iter() {
            if let Some(n) = ch {
                n.find_nearby_points(center, radius, out);
            }
        }
    }
}
```
```rust
//-----------------------------
// ON_Octree (wrapper)
//-----------------------------
pub struct Octree {
    pub root: Box<OctreeNode>,
}
```
```rust
impl Octree {
    pub fn new(
        min_pt: OctreePoint,
        max_pt: OctreePoint,
        max_depth: i32,
        max_points: i32,
        merge_threshold: i32,
    ) -> Self {
        let root = OctreeNode::new(min_pt, max_pt, 0, max_depth, max_points, merge_threshold);
        Self {
            root: Box::new(root),
        }
    }
```
```rust
    #[inline]
    pub fn insert(&mut self, p: OctreePoint) {
        self.root.insert(p);
    }
 ```
```rust   
    #[inline]
    pub fn remove(&mut self, p: &OctreePoint) -> bool {
        self.root.remove(p)
    }
```
```rust    
    #[inline]
    pub fn search(&self, p: &OctreePoint) -> bool {
        self.root.search(p)
    }
```
```rust
    pub fn find_nearby_points(&self, center: &OctreePoint, radius: f64) -> Vec<OctreePoint> {
        let mut v = Vec::new();
        self.root.find_nearby_points(center, radius, &mut v);
        v
    }
}
```
```rust
//-----------------------------
// Mesh Triangle Octree
//-----------------------------

#[derive(Clone, Copy, Debug)]
pub struct IndexTriangle {
    pub v0: usize,
    pub v1: usize,
    pub v2: usize,
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Point3D,
    pub max: Point3D,
}
```
```rust
impl AABB {
    #[inline]
    pub fn intersects(&self, other: &AABB) -> bool {
        !(other.min.x > self.max.x
            || other.max.x < self.min.x
            || other.min.y > self.max.y
            || other.max.y < self.min.y
            || other.min.z > self.max.z
            || other.max.z < self.min.z)
    }
```
```rust    
    #[inline]
    pub fn contains_point(&self, p: &Point3D) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }
```
```rust    
    #[inline]
    pub fn contains_aabb(&self, b: &AABB) -> bool {
        self.contains_point(&b.min) && self.contains_point(&b.max)
    }
}
```
```rust
pub struct MeshTriangleOctree<'a> {
    vertices: &'a [Point3D],
    triangles: &'a [IndexTriangle],
    max_tris: usize,
    max_depth: usize,
    root: Option<Box<Node>>,
}
```
```rust
pub struct Node {
    pub bbox: AABB,
    pub triangle_indices: Vec<usize>,
    pub children: [Option<Box<Node>>; 8],
    pub is_leaf: bool,
}
```
```rust
impl<'a> MeshTriangleOctree<'a> {
    pub fn new(
        vertices: &'a [Point3D],
        triangles: &'a [IndexTriangle],
        max_triangles_per_node: usize,
        max_depth: usize,
    ) -> Self {
        // 전체 AABB
        let global = if vertices.is_empty() {
            AABB {
                min: Point3D::new(0.0, 0.0, 0.0),
                max: Point3D::new(0.0, 0.0, 0.0),
            }
        } else {
            let mut minx = vertices[0].x;
            let mut miny = vertices[0].y;
            let mut minz = vertices[0].z;
            let mut maxx = minx;
            let mut maxy = miny;
            let mut maxz = minz;
            for v in vertices {
                minx = minx.min(v.x);
                miny = miny.min(v.y);
                minz = minz.min(v.z);
                maxx = maxx.max(v.x);
                maxy = maxy.max(v.y);
                maxz = maxz.max(v.z);
            }
            AABB {
                min: Point3D::new(minx, miny, minz),
                max: Point3D::new(maxx, maxy, maxz),
            }
        };

        let mut all_idx = Vec::with_capacity(triangles.len());
        for (i, _) in triangles.iter().enumerate() {
            all_idx.push(i);
        }

        let mut me = Self {
            vertices,
            triangles,
            max_tris: max_triangles_per_node,
            max_depth,
            root: None,
        };
        me.root = Some(me.build_node(global, &all_idx, 0));
        me
    }
```
```rust
    pub fn query_range(&self, range: &AABB, out_tris: &mut Vec<usize>) {
        fn rec<'b>(
            node: &'b Node,
            range: &AABB,
            verts: &'b [Point3D],
            tris: &'b [IndexTriangle],
            out: &mut Vec<usize>,
            compute_tri_aabb: &dyn Fn(&[Point3D], &IndexTriangle) -> AABB,
        ) {
            if !node.bbox.intersects(range) {
                return;
            }
            for &idx in &node.triangle_indices {
                let tri_box = compute_tri_aabb(verts, &tris[idx]);
                if range.intersects(&tri_box) {
                    out.push(idx);
                }
            }
            if !node.is_leaf {
                for ch in node.children.iter() {
                    if let Some(c) = ch {
                        rec(c, range, verts, tris, out, compute_tri_aabb);
                    }
                }
            }
        }

        if let Some(ref root) = self.root {
            let f = |verts: &[Point3D], tri: &IndexTriangle| -> AABB {
                self.compute_triangle_aabb(tri, verts)
            };
            rec(root, range, self.vertices, self.triangles, out_tris, &f);
        }
    }
```
```rust
    fn build_node(&self, bounds: AABB, tri_indices: &[usize], depth: usize) -> Box<Node> {
        let mut node = Box::new(Node {
            bbox: bounds,
            triangle_indices: Vec::new(),
            children: Default::default(),
            is_leaf: true,
        });

        if tri_indices.len() <= self.max_tris || depth >= self.max_depth {
            node.triangle_indices.extend_from_slice(tri_indices);
            return node;
        }

        // 8 자식 박스
        let cx = 0.5 * (bounds.min.x + bounds.max.x);
        let cy = 0.5 * (bounds.min.y + bounds.max.y);
        let cz = 0.5 * (bounds.min.z + bounds.max.z);

        let mut child_bounds = [bounds; 8];
        for i in 0..8 {
            child_bounds[i].min.x = if (i & 4) != 0 { cx } else { bounds.min.x };
            child_bounds[i].max.x = if (i & 4) != 0 { bounds.max.x } else { cx };
            child_bounds[i].min.y = if (i & 2) != 0 { cy } else { bounds.min.y };
            child_bounds[i].max.y = if (i & 2) != 0 { bounds.max.y } else { cy };
            child_bounds[i].min.z = if (i & 1) != 0 { cz } else { bounds.min.z };
            child_bounds[i].max.z = if (i & 1) != 0 { bounds.max.z } else { cz };
        }

        let mut child_tris: [Vec<usize>; 8] = Default::default();
        let mut remain = Vec::new();

        for &idx in tri_indices {
            let trib = self.compute_triangle_aabb(&self.triangles[idx], self.vertices);
            let oct = self.get_octant(&bounds, &trib);
            if let Some(o) = oct {
                if child_bounds[o].contains_aabb(&trib) {
                    child_tris[o].push(idx);
                    continue;
                }
            }
            remain.push(idx);
        }

        node.triangle_indices = remain;
        node.is_leaf = false;

        for i in 0..8 {
            if !child_tris[i].is_empty() {
                node.children[i] =
                    Some(self.build_node(child_bounds[i], &child_tris[i], depth + 1));
            }
        }

        node
    }
```
```rust
    #[inline]
    fn compute_triangle_aabb(&self, tri: &IndexTriangle, verts: &[Point3D]) -> AABB {
        let p0 = verts[tri.v0];
        let p1 = verts[tri.v1];
        let p2 = verts[tri.v2];
        let minx = p0.x.min(p1.x).min(p2.x);
        let miny = p0.y.min(p1.y).min(p2.y);
        let minz = p0.z.min(p1.z).min(p2.z);
        let maxx = p0.x.max(p1.x).max(p2.x);
        let maxy = p0.y.max(p1.y).max(p2.y);
        let maxz = p0.z.max(p1.z).max(p2.z);
        AABB {
            min: Point3D::new(minx, miny, minz),
            max: Point3D::new(maxx, maxy, maxz),
        }
    }
```
```rust
    /// triBox가 부모 박스의 어느 하나의 옥탄트에 완전히 들어가면 그 옥탄트 인덱스를 Some으로,
    /// 아니면 None.
    #[inline]
    fn get_octant(&self, parent: &AABB, tri_box: &AABB) -> Option<usize> {
        let cx = 0.5 * (parent.min.x + parent.max.x);
        let cy = 0.5 * (parent.min.y + parent.max.y);
        let cz = 0.5 * (parent.min.z + parent.max.z);

        let min_lower_x = tri_box.max.x <= cx;
        let min_lower_y = tri_box.max.y <= cy;
        let min_lower_z = tri_box.max.z <= cz;
        let max_upper_x = tri_box.min.x >= cx;
        let max_upper_y = tri_box.min.y >= cy;
        let max_upper_z = tri_box.min.z >= cz;

        let x_ok = min_lower_x || max_upper_x;
        let y_ok = min_lower_y || max_upper_y;
        let z_ok = min_lower_z || max_upper_z;

        if x_ok && y_ok && z_ok {
            let ix = if max_upper_x { 1 } else { 0 };
            let iy = if max_upper_y { 1 } else { 0 };
            let iz = if max_upper_z { 1 } else { 0 };
            Some((ix << 2) | (iy << 1) | iz)
        } else {
            None
        }
    }
}
```
```rust
#[inline]
pub fn on_is_triangle_fully_inside_box(
    v0: &Point3D,
    v1: &Point3D,
    v2: &Point3D,
    box_: &BoundingBox,
) -> bool {
    box_.includes_point(&v0, false)
        && box_.includes_point(&v1, false)
        && box_.includes_point(&v2, false)
}

```

---

# 테스트

아래는 이 Octree 테스트 코드에 대한 수학적 의미와 테스트 목적을 항목별로 정리한 분석입니다.  
각 테스트가 어떤 공간적 원리와 알고리즘을 검증하는지, 그리고 왜 중요한지를 설명합니다.

## ✅ Octree 테스트 분석 요약

| 테스트 함수                         | 수학적 의미 또는 알고리즘 기반                          | 테스트 목적 또는 검증 항목                         |
|------------------------------------|--------------------------------------------------------|---------------------------------------------------|
| `test_insert_and_search`           | AABB 포함 판정 + Octant 분기                           | 점 삽입 후 정확한 위치 탐색 가능 여부 확인         |
| `test_remove_point`                | 근사 동등성 기반 제거 (`approx_eq`)                    | 삽입된 점을 정확히 제거하고 검색되지 않음을 확인   |
| `test_find_nearby_points`          | 구-박스 교차 판정 + 유클리드 거리 계산                | 중심점 기준 반경 내 점 탐색 정확성 검증            |
| `octree_insert_search_remove`      | 절대 오차 기반 근사 비교 (`approx_eq`)                | 수치적 동등성 테스트 및 제거 후 상태 확인          |
| `octree_subdivide_and_merge`       | 분할 조건: `max_points` 초과 → 병합 조건: `merge_threshold` 이하 | 자동 분할 및 병합 로직의 수학적 조건 충족 여부 검증 |
| `octree_find_nearby_points`        | 격자 기반 유클리드 거리 판정                          | 3D 격자에서 반경 내 점 개수 및 자기 자신 포함 여부 확인 |
| `mesh_triangle_octree_basic_query` | 삼각형 AABB와 범위 박스의 교차 여부                   | 삼각형 기반 Octree에서 공간 질의 정확성 검증        |
| `triangle_fully_inside_box`        | 삼각형의 꼭짓점이 AABB에 포함되는지 여부              | 삼각형이 완전히 박스 내부에 있는지 판정하는 수학적 조건 확인 |
| `insert_search`                    | 기본 AABB 포함 판정                                   | Octree의 최소 기능이 정상 작동하는지 확인          |


## 🧠 수학적 원리 요약
### 1. AABB 포함 판정

$$
x_{\min }\leq x\leq x_{\max },\quad y_{\min }\leq y\leq y_{\max },\quad z_{\min }\leq z\leq z_{\max }
$$

### 2. Octant 결정

$$
\mathrm{octant}=(x>c_x)\cdot 4+(y>c_y)\cdot 2+(z>c_z)\cdot 1
$$

### 3. 근사 동등성

$$
|x_1-x_2|\leq \varepsilon ,\quad \varepsilon =10^{-12}
$$


### 4. 구-박스 교차 판정

$$
\mathrm{거리\ 제곱} = \sum_{i=x,y,z} 
\{
  \begin{array}{ll}
    (\min_i - c_i)^2 & \text{if } c_i < \min_i \\
    (c_i - \max_i)^2 & \text{if } c_i > \max_i \\
    0 & \text{otherwise}
  \end{array}
\}.
$$

### 5. 삼각형 AABB 계산

$$
\mathrm{AABB_{tri}}=\left[ \min (v_0,v_1,v_2),\  \max (v_0,v_1,v_2)\right] 
$$

## 📌 결론
이 테스트셋은 Octree의 핵심 기능을 수학적으로 정합하게 검증하고 있으며, 다음을 만족합니다:
- 공간 분할 및 탐색 정확성
- 수치적 근사 비교 안정성
- 병합 및 분할 조건의 수학적 타당성
- 삼각형 기반 공간 질의의 기하학적 정확성


```rust
#[cfg(test)]
mod octree_tests {
    use nurbslib::core::boundingbox::BoundingBox;
    use nurbslib::core::geom::Point3D;
    use nurbslib::core::octree::{on_is_triangle_fully_inside_box, IndexTriangle, MeshTriangleOctree, Octree, OctreePoint, AABB};
    use super::*;
```
```rust
    #[test]
    fn test_insert_and_search() {
        let mut tree = Octree::new(
            OctreePoint::new(0.0, 0.0, 0.0),
            OctreePoint::new(10.0, 10.0, 10.0),
            4,  // max_depth
            2,  // max_points
            4,  // merge_threshold
        );

        let p1 = OctreePoint::new(1.0, 1.0, 1.0);
        let p2 = OctreePoint::new(9.0, 9.0, 9.0);

        tree.insert(p1);
        tree.insert(p2);

        assert!(tree.search(&p1));
        assert!(tree.search(&p2));
        assert!(!tree.search(&OctreePoint::new(5.0, 5.0, 5.0)));
    }
```
```rust

    #[test]
    fn test_remove_point() {
        let mut tree = Octree::new(
            OctreePoint::new(0.0, 0.0, 0.0),
            OctreePoint::new(10.0, 10.0, 10.0),
            4, 2, 4,
        );

        let p = OctreePoint::new(3.0, 3.0, 3.0);
        tree.insert(p);
        assert!(tree.search(&p));
        assert!(tree.remove(&p));
        assert!(!tree.search(&p));
    }
```
```rust

    #[test]
    fn test_find_nearby_points() {
        let mut tree = Octree::new(
            OctreePoint::new(0.0, 0.0, 0.0),
            OctreePoint::new(10.0, 10.0, 10.0),
            4, 2, 4,
        );

        let center = OctreePoint::new(5.0, 5.0, 5.0);
        let near = OctreePoint::new(5.1, 5.1, 5.1);
        let far = OctreePoint::new(9.0, 9.0, 9.0);

        tree.insert(near);
        tree.insert(far);

        let result = tree.find_nearby_points(&center, 0.5);
        assert!(result.contains(&near));
        assert!(!result.contains(&far));
    }
```
```rust

    fn octree_pt(x: f64, y: f64, z: f64) -> OctreePoint {
        OctreePoint::new(x, y, z)
    }
    fn vtx(x: f64, y: f64, z: f64) -> Point3D {
        Point3D::new(x, y, z)
    }

    #[test]
    fn octree_insert_search_remove() {
        let mut ot = Octree::new(
            octree_pt(0.0, 0.0, 0.0),
            octree_pt(10.0, 10.0, 10.0),
            6,
            /*max_depth*/ 4,
            /*max_points*/ 2, /*merge_threshold*/
        );

        let pts = [
            octree_pt(1.0, 1.0, 1.0),
            octree_pt(2.0, 2.0, 2.0),
            octree_pt(8.0, 8.0, 8.0),
            octree_pt(5.0, 5.0, 5.0),
        ];
        for q in pts {
            ot.insert(q);
        }

        // 존재 검색
        assert!(ot.search(&octree_pt(1.0, 1.0, 1.0)));
        assert!(ot.search(&octree_pt(2.0, 2.0, 2.0)));
        assert!(ot.search(&octree_pt(8.0, 8.0, 8.0)));
        assert!(ot.search(&octree_pt(5.0, 5.0, 5.0)));

        // 허용오차 내 같음(= C++ operator== 근사)
        assert!(ot.search(&octree_pt(1.0 + 1e-13, 1.0, 1.0)));

        // 미존재 검색
        assert!(!ot.search(&octree_pt(9.0, 0.0, 0.0)));

        // 제거
        assert!(ot.remove(&octree_pt(5.0, 5.0, 5.0)));
        assert!(!ot.search(&octree_pt(5.0, 5.0, 5.0)));
        assert!(ot.search(&octree_pt(8.0, 8.0, 8.0)));
    }
```
```rust
    #[test]
    fn octree_subdivide_and_merge() {
        let mut ot = Octree::new(
            octree_pt(0.0, 0.0, 0.0),
            octree_pt(10.0, 10.0, 10.0),
            6,
            /*max_depth*/ 2,
            /*max_points*/ 4, /*merge_threshold*/
        );

        // 한 옥탄트(상위/상위/상위)로 몰아넣기
        let cluster = vec![
            octree_pt(9.0, 9.0, 9.0),
            octree_pt(9.1, 9.1, 9.1),
            octree_pt(9.2, 9.2, 9.2),
            octree_pt(9.3, 9.3, 9.3),
            octree_pt(9.4, 9.4, 9.4),
        ];
        for q in &cluster {
            ot.insert(*q);
        }

        // max_points(=4) 초과 → 분할되어야 함
        assert!(ot.root.children.iter().any(|c| c.is_some()));

        // 다 지우면 merge 발생
        for q in &cluster {
            assert!(ot.remove(q));
        }

        // 필요 시 루트에서 merge 한번 더 시도 (자식 노드가 남아있다가도 조건 만족 시 병합)
        ot.root.try_merge();
        assert!(ot.root.children.iter().all(|c| c.is_none()));
    }
```
```rust
    #[test]
    fn octree_find_nearby_points() {
        let mut ot = Octree::new(
            octree_pt(0.0, 0.0, 0.0),
            octree_pt(6.0, 6.0, 6.0),
            5,
            /*max_depth*/ 4,
            /*max_points*/ 2, /*merge_threshold*/
        );

        // 0..=2 격자 27개
        for ix in 0..=2 {
            for iy in 0..=2 {
                for iz in 0..=2 {
                    ot.insert(octree_pt(ix as f64 * 2.0, iy as f64 * 2.0, iz as f64 * 2.0));
                }
            }
        }

        let center = octree_pt(2.0, 2.0, 2.0);
        let found = ot.find_nearby_points(&center, 2.01); // 반경 2.01 → 6개의 이웃(맨해튼 1스텝 중 유클리드<=2)
        // 실제로는 (2,2,2) 자기 자신 포함해서 7개가 들어옴: 자기자신 + 6 방향
        assert!(found.len() >= 7);
        assert!(found.iter().any(|q| q.approx_eq(&center)));
    }
```
```rust
    #[test]
    fn mesh_triangle_octree_basic_query() {
        // 정육면체 한 면을 두 삼각형으로
        let verts = vec![
            vtx(0.0, 0.0, 0.0), // 0
            vtx(1.0, 0.0, 0.0), // 1
            vtx(1.0, 1.0, 0.0), // 2
            vtx(0.0, 1.0, 0.0), // 3
        ];
        let tris = vec![
            IndexTriangle {
                v0: 0,
                v1: 1,
                v2: 2,
            },
            IndexTriangle {
                v0: 0,
                v1: 2,
                v2: 3,
            },
        ];

        let oct = MeshTriangleOctree::new(&verts, &tris, 1, 8);

        // 박스 (0.25..0.75)^2 x {0}
        let range = AABB {
            min: vtx(0.25, 0.25, -0.1),
            max: vtx(0.75, 0.75, 0.1),
        };

        let mut hits = Vec::<usize>::new();
        oct.query_range(&range, &mut hits);

        // 두 삼각형 모두 범위와 교차해야 함
        assert!(hits.contains(&0));
        assert!(hits.contains(&1));
    }
```
```rust
    #[test]
    fn triangle_fully_inside_box() {
        let bb = BoundingBox {
            min: vtx(0.0, 0.0, 0.0),
            max: vtx(2.0, 2.0, 2.0),
        };
        let a = vtx(0.5, 0.5, 0.5);
        let b = vtx(1.5, 0.5, 0.5);
        let c = vtx(0.5, 1.5, 0.5);
        assert!(on_is_triangle_fully_inside_box(&a, &b, &c, &bb));

        let d = vtx(2.5, 0.5, 0.5);
        assert!(!on_is_triangle_fully_inside_box(&a, &b, &d, &bb));
    }
```
```rust
    #[test]
    fn insert_search() {
        let mut ot = Octree::new(
            OctreePoint::new(0.0, 0.0, 0.0),
            OctreePoint::new(10.0, 10.0, 10.0),
            6,
            4,
            2,
        );
        let q = OctreePoint::new(1.0, 1.0, 1.0);
        ot.insert(q);
        assert!(ot.search(&q));
    }

}
```
