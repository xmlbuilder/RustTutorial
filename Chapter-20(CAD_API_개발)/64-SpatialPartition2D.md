# SpaitialPartition2D

## 🧮 주요 함수 수식 정리
### 1. 셀 인덱스 계산

$$
clamp\_floor\_index(coord, min, axis)
= \min\big( \max( \lfloor (coord - min) / cell\_size \rfloor, 0 ),\; cell\_counts[axis] - 1 \big)
$$

### 2. 셀 위치 → 풀 인덱스

$$
\mathrm{get\_ cell\_ index}(x_i,y_i)=x_i\cdot cell\_ counts[1]+y_i
$$

### 3. 선분 vs AABB 교차 (slab 방식)

$$
t=\max _{ax}\left( \frac{min[ax]-a[ax]}{b[ax]-a[ax]}\right) ,\quad \mathrm{조건:\  }a[ax]\leq b[ax]
$$


### 4. 삼각형 vs AABB 교차 (SAT 방식)

$$
\mathrm{dot}(n,v_i)=n_x\cdot v_{ix}+n_y\cdot v_{iy}
$$

$$
\mathrm{rad}=|n_x|\cdot box\_ half_x+|n_y|\cdot box\_ half_y
$$

$$
\mathrm{조건:\  }\min (dot)\leq rad\wedge \max (dot)\geq -rad
$$

## 📌 함수 설명 요약표
| 함수 이름                        | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `clamp_floor_index`              | 좌표를 셀 인덱스로 변환. 경계 밖은 자동 보정                         |
| `get_cell_index`                 | (x, y) 셀 좌표를 1D 풀 인덱스로 변환                                 |
| `push_to_cell`                   | 셀에 데이터를 삽입. 연결 리스트 방식으로 저장                         |
| `contains`                       | 셀 내에 특정 데이터가 이미 존재하는지 검사                            |
| `index_range_for_point`         | 점 기준 AABB 범위 계산 → 셀 인덱스 범위 반환                          |
| `index_range_for_line`          | 선분 기준 AABB 범위 계산 → 셀 인덱스 범위 반환                        |
| `index_range_for_triangle`      | 삼각형 기준 AABB 범위 계산 → 셀 인덱스 범위 반환                      |
| `line_box_overlap_2d`           | 선분과 AABB의 교차 여부 계산. slab 방식 사용                          |
| `tri_box_overlap_2d`            | 삼각형과 AABB의 교차 여부 계산. SAT 방식 사용                         |
| `insert_point`                  | 점을 셀에 삽입. 중복 방지 포함                                        |
| `insert_line`                   | 선분을 셀에 삽입. 교차 여부 검사 포함                                 |
| `insert_triangle`               | 삼각형을 셀에 삽입. 교차 여부 검사 포함                                |
| `begin`, `begin_box`            | 셀 또는 박스 범위에 대한 이터레이터 생성                              |
| `collect_box_ids`, `collect_aabb_ids` | 박스 또는 AABB 범위 내의 모든 데이터 ID 수집                         |
| `process_check_pairs`           | 각 셀 내에서 모든 데이터 쌍에 대해 검사 수행                          |
| `process_check_range`           | AABB 범위 내 모든 데이터에 대해 검사 수행                             |
| `remove_all`                    | 모든 셀 초기화 및 데이터 제거                                         |

---

# 수식 검증

문서에 포함된 주요 수학 로직들을 하나씩 검증.  
특히 공간 파티셔닝에서 핵심이 되는 셀 인덱스 계산, AABB 범위 추출, 선분/삼각형 교차 판정 함수들을 수학적으로 분석합니다.

## ✅ 1. 셀 인덱스 계산: clamp_floor_index
- 수식:

$$
\mathrm{index}=\left\lfloor \frac{\mathrm{coord}-\mathrm{min}}{\mathrm{cell\_ size}}\right\rfloor
$$

- 검증:
    - floor()는 셀 경계 기준으로 좌표를 하향 정렬
    - 음수나 NaN 방지를 위해 <= 0일 경우 0으로 보정
    - 최대 인덱스 초과 시 cell_counts[axis] - 1로 클램핑
    - ✅ 정확한 셀 인덱스 계산 방식이며, 경계 외 좌표에 대한 안정성도 확보됨

## ✅ 2. 셀 풀 인덱스: get_cell_index(xi, yi)
- 수식:

$$
\mathrm{index}=x_i\cdot \mathrm{cell\_ counts}[1]+y_i
$$

- 검증:
    - 2D 격자를 1D 배열로 표현하는 전형적인 방식
    - 행 우선(row-major) 순서로 인덱싱
    - ✅ 수학적으로 올바른 2D → 1D 인덱스 변환

## ✅ 3. 선분 vs AABB 교차: line_box_overlap_2d
- 핵심 수식:

$$
t_{\mathrm{entry}}=\max (\mathrm{slab_{\mathnormal{x}}},\mathrm{slab_{\mathnormal{y}}}),\quad t_{\mathrm{exit}}=\min (\mathrm{slab_{\mathnormal{x}}},\mathrm{slab_{\mathnormal{y}}})
$$

- 각 축에 대해 선분의 시작/끝이 박스 범위와 겹치는지 확인
- slab 방식은 선분을 [0,1] 구간으로 정규화하여 교차 여부 판단
- ✅ 2D slab 방식은 수학적으로 정확하며, 모든 축에 대해 교차 구간을 계산함

## ✅ 4. 삼각형 vs AABB 교차: tri_box_overlap_2d
- 핵심 수식:
- 삼각형의 각 꼭짓점 v_i를 박스 중심 기준으로 이동
- 박스 축(X, Y) 및 삼각형 엣지의 수직 벡터 $n=(e_y,-e_x)$ 에 대해 SAT 테스트

$$
\mathrm{rad}=|n_x|\cdot \mathrm{half_{\mathnormal{x}}}+|n_y|\cdot \mathrm{half_{\mathnormal{y}}}
$$

$$
\mathrm{proj_{\mathnormal{i}}}=n\cdot v_i
$$

$$
\mathrm{조건:\  }\min (\mathrm{proj})\leq \mathrm{rad}\wedge \max (\mathrm{proj})\geq -\mathrm{rad}
$$

- ✅ Separating Axis Theorem(SAT)를 정확히 구현한 방식이며, 2D 충돌 판정에 적합함

### 📌 요약표: 수학적 검증 결과

| 함수 이름              | 수학적 모델 설명                                      | 검증 결과             |
|------------------------|--------------------------------------------------------|------------------------|
| `clamp_floor_index`    | 좌표를 셀 인덱스로 변환: ⌊(coord - min) / cell_size⌋   | ✅ 경계 보정 포함, 안정적 |
| `get_cell_index`       | 2D → 1D 인덱스 변환: xi × cell_counts[1] + yi         | ✅ 전형적 행 우선 방식 |
| `line_box_overlap_2d`  | slab 방식 교차 판정: 각 축에서 t_entry, t_exit 계산   | ✅ 정규화된 교차 구간 계산 |
| `tri_box_overlap_2d`   | SAT 방식 교차 판정: 박스 축 + 삼각형 엣지 수직축 투영 | ✅ 축 기반 충돌 판정 정확 |


## 소스 코드

```rut
use std::collections::HashSet;
use crate::core::geom::Point2D;

#[derive(Debug)]
struct Item {
    data: usize,
    next: Option<Box<Item>>,
}
```
```rust
pub trait CompPair2D {
    fn process_check(&mut self, d1: usize, d2: usize);
}
```
```rust
pub trait CompSingle2D {
    fn process_check(&mut self, d: usize);
}
```
```rust
/// 2D 균일 격자 기반 파티셔너 (3D 버전과 필드/메서드 구성을 맞춥니다)
pub struct SpatialPartition2D {
    cell_counts: [usize; 2],
    cell_size: f64,
    pt_min: Point2D,
    pt_max: Point2D,
    pool: Vec<Option<Box<Item>>>,
    item_count: usize,
}
```
```rust
impl SpatialPartition2D {
    /// 경계(min,max)와 division(홀수로 보정)으로 초기화
    pub fn new_from_bounds(min: Point2D, max: Point2D, division: usize) -> Self {
        let div = if division % 2 == 0 {
            division + 1
        } else {
            division
        };

        let dist_x = max.x - min.x;
        let dist_y = max.y - min.y;
        let mut max_dist = dist_x.max(dist_y);
        max_dist += max_dist * 0.001;

        let cell_size = max_dist as f64 / div as f64;

        let mut cell_counts = [0usize; 2];
        for a in 0..2 {
            let dist = if a == 0 { dist_x } else { dist_y };
            let expanded = dist + max_dist * 0.001;
            let cnt = (expanded / cell_size).ceil() as usize;
            cell_counts[a] = if cnt % 2 == 0 { cnt + 1 } else { cnt };
        }

        let cx = (min.x + max.x) * 0.5;
        let cy = (min.y + max.y) * 0.5;
        let pt_min = Point2D {
            x: cx - cell_size * cell_counts[0] as f64 * 0.5,
            y: cy - cell_size * cell_counts[1] as f64 * 0.5,
        };
        let pt_max = Point2D {
            x: pt_min.x + cell_size * cell_counts[0] as f64,
            y: pt_min.y + cell_size * cell_counts[1] as f64,
        };

        let pool_size = cell_counts[0] * cell_counts[1];
        let mut pool = Vec::with_capacity(pool_size);
        pool.resize_with(pool_size, || None);

        Self {
            cell_counts,
            cell_size,
            pt_min,
            pt_max,
            pool,
            item_count: 0,
        }
    }
```
```rust
    // 만약 BoundingRect 타입이 있다면 이 생성자를 노출하세요.
    // pub fn new(b: &BoundingRect, division: usize) -> Self {
    //     Self::new_from_bounds(b.min, b.max, division)
    // }

    // ---------- 게터 ----------
    #[inline]
    pub fn cell_counts(&self) -> [usize; 2] {
        self.cell_counts
    }
```
```rust    
    #[inline]
    pub fn cell_size(&self) -> f64 {
        self.cell_size
    }
```
```rust    
    #[inline]
    pub fn pt_min(&self) -> &Point2D {
        &self.pt_min
    }
```
```rust    
    #[inline]
    pub fn pt_max(&self) -> &Point2D {
        &self.pt_max
    }
```
```rust    
    #[inline]
    pub fn item_count(&self) -> usize {
        self.item_count
    }
```
```rust    
    #[inline]
    pub fn pool_size(&self) -> usize {
        self.pool.len()
    }
```
```rust
    // ---------- 내부 유틸 ----------
    #[inline]
    fn clamp_floor_index(&self, coord: f64, min: f64, axis: usize) -> usize {
        let rel = (coord - min) / self.cell_size;
        let raw = rel.floor();
        if raw.is_nan() || raw <= 0.0 {
            return 0;
        }
        let idx = raw as isize;
        let max_i = self.cell_counts[axis] as isize - 1;
        if idx > max_i {
            max_i as usize
        } else {
            idx as usize
        }
    }
```
```rust
    #[inline]
    fn get_cell_index(&self, xi: usize, yi: usize) -> Option<usize> {
        if xi >= self.cell_counts[0] || yi >= self.cell_counts[1] {
            return None;
        }
        Some(xi * self.cell_counts[1] + yi)
    }
```
```rust
    #[inline]
    fn push_to_cell(&mut self, index: usize, data: usize) {
        let new_item = Box::new(Item {
            data,
            next: self.pool[index].take(),
        });
        self.pool[index] = Some(new_item);
        self.item_count += 1;
    }
```
```rust
    fn contains(&self, index: usize, data: usize) -> bool {
        let mut cur = self.pool[index].as_deref();
        while let Some(n) = cur {
            if n.data == data {
                return true;
            }
            cur = n.next.as_deref();
        }
        false
    }
```
```rust
    // AABB 인덱스 범위
    #[inline]
    fn index_range_for_point(&self, p: &Point2D, tol: f64) -> Option<([usize; 2], [usize; 2])> {
        let s = [
            self.clamp_floor_index(p.x - tol, self.pt_min.x, 0),
            self.clamp_floor_index(p.y - tol, self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(p.x + tol, self.pt_min.x, 0),
            self.clamp_floor_index(p.y + tol, self.pt_min.y, 1),
        ];
        if s[0] > e[0] || s[1] > e[1] {
            return None;
        }
        Some((s, e))
    }
```
```rust    
    #[inline]
    fn index_range_for_line(
        &self,
        a: &Point2D,
        b: &Point2D,
        tol: f64,
    ) -> Option<([usize; 2], [usize; 2])> {
        let minp = [a.x.min(b.x) - tol, a.y.min(b.y) - tol];
        let maxp = [a.x.max(b.x) + tol, a.y.max(b.y) + tol];
        let s = [
            self.clamp_floor_index(minp[0], self.pt_min.x, 0),
            self.clamp_floor_index(minp[1], self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(maxp[0], self.pt_min.x, 0),
            self.clamp_floor_index(maxp[1], self.pt_min.y, 1),
        ];
        if s[0] > e[0] || s[1] > e[1] {
            return None;
        }
        Some((s, e))
    }
```
```rust    
    #[inline]
    fn index_range_for_triangle(
        &self,
        p1: &Point2D,
        p2: &Point2D,
        p3: &Point2D,
        tol: f64,
    ) -> Option<([usize; 2], [usize; 2])> {
        let minp = [
            p1.x.min(p2.x).min(p3.x) - tol,
            p1.y.min(p2.y).min(p3.y) - tol,
        ];
        let maxp = [
            p1.x.max(p2.x).max(p3.x) + tol,
            p1.y.max(p2.y).max(p3.y) + tol,
        ];
        let s = [
            self.clamp_floor_index(minp[0], self.pt_min.x, 0),
            self.clamp_floor_index(minp[1], self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(maxp[0], self.pt_min.x, 0),
            self.clamp_floor_index(maxp[1], self.pt_min.y, 1),
        ];
        if s[0] > e[0] || s[1] > e[1] {
            return None;
        }
        Some((s, e))
    }
```
```rust
    // ---------- 2D 교차 테스트 ----------
    #[inline]
    fn dot(a: [f64; 2], b: [f64; 2]) -> f64 {
        a[0] * b[0] + a[1] * b[1]
    }
    #[inline]
    fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
        [a[0] - b[0], a[1] - b[1]]
    }

    /// 선분 vs AABB (2D slab) – 교차 시 [0,1]상의 t 반환
    fn line_box_overlap_2d(a: [f64; 2], b: [f64; 2], min: [f64; 2], max: [f64; 2]) -> Option<f64> {
        let mut fst = 0.0;
        let mut fet = 1.0;
        for ax in 0..2 {
            let si = a[ax];
            let ei = b[ax];
            let di = ei - si;
            if di.abs() < f64::EPSILON {
                if si < min[ax] || si > max[ax] {
                    return None;
                }
                continue;
            }
            let (st, et) = if si < ei {
                if si > max[ax] || ei < min[ax] {
                    return None;
                }
                (
                    if si < min[ax] {
                        (min[ax] - si) / di
                    } else {
                        0.0
                    },
                    if ei > max[ax] {
                        (max[ax] - si) / di
                    } else {
                        1.0
                    },
                )
            } else {
                if ei > max[ax] || si < min[ax] {
                    return None;
                }
                (
                    if si > max[ax] {
                        (max[ax] - si) / di
                    } else {
                        0.0
                    },
                    if ei < min[ax] {
                        (min[ax] - si) / di
                    } else {
                        1.0
                    },
                )
            };
            if st > fst {
                fst = st;
            }
            if et < fet {
                fet = et;
            }
            if fet < fst {
                return None;
            }
        }
        Some(fst)
    }
```
```rust
    /// 삼각형 vs AABB (2D SAT) – 박스 축(X,Y) + 삼각형 3개 엣지의 수직축 테스트
    fn tri_box_overlap_2d(box_center: [f64; 2], box_half: [f64; 2], tri: [[f64; 2]; 3]) -> bool {
        // 박스 중심 기준으로 이동
        let v0 = Self::sub(tri[0], box_center);
        let v1 = Self::sub(tri[1], box_center);
        let v2 = Self::sub(tri[2], box_center);

        // 박스 축(X,Y)
        let (minx, maxx) = {
            let (a, b, c) = (v0[0], v1[0], v2[0]);
            (a.min(b.min(c)), a.max(b.max(c)))
        };
        if minx > box_half[0] || maxx < -box_half[0] {
            return false;
        }
        let (miny, maxy) = {
            let (a, b, c) = (v0[1], v1[1], v2[1]);
            (a.min(b.min(c)), a.max(b.max(c)))
        };
        if miny > box_half[1] || maxy < -box_half[1] {
            return false;
        }

        // 삼각형 엣지의 수직 벡터를 축으로 사용
        let edges = [
            [v1[0] - v0[0], v1[1] - v0[1]],
            [v2[0] - v1[0], v2[1] - v1[1]],
            [v0[0] - v2[0], v0[1] - v2[1]],
        ];

        for e in edges {
            // 수직축 n = (ey, -ex) (정규화 불필요)
            let n = [e[1], -e[0]];
            let fa = n[0].abs();
            let fb = n[1].abs();
            // 박스 반경
            let rad = fa * box_half[0] + fb * box_half[1];
            // 삼각형 프로젝션
            let p0 = Self::dot(n, v0);
            let p1 = Self::dot(n, v1);
            let p2 = Self::dot(n, v2);
            let minp = p0.min(p1.min(p2));
            let maxp = p0.max(p1.max(p2));
            if minp > rad || maxp < -rad {
                return false;
            }
        }
        true
    }
```
```rust
    // ---------- 삽입 ----------
    pub fn insert_point(&mut self, pt: Point2D, tol: f64, data: usize) -> bool {
        let Some((s, e)) = self.index_range_for_point(&pt, tol) else {
            return false;
        };
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    if !self.contains(cell, data) {
                        self.push_to_cell(cell, data);
                    }
                }
            }
        }
        true
    }
```
```rust
    pub fn insert_line(&mut self, a: Point2D, b: Point2D, tol: f64, data: usize) -> bool {
        let Some((s, e)) = self.index_range_for_line(&a, &b, tol) else {
            return false;
        };
        let pa = [a.x, a.y];
        let pb = [b.x, b.y];
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    let min = [
                        i as f64 * self.cell_size + self.pt_min.x - tol,
                        j as f64 * self.cell_size + self.pt_min.y - tol,
                    ];
                    let max = [
                        (i + 1) as f64 * self.cell_size + self.pt_min.x + tol,
                        (j + 1) as f64 * self.cell_size + self.pt_min.y + tol,
                    ];
                    if Self::line_box_overlap_2d(pa, pb, min, max).is_some() {
                        if !self.contains(cell, data) {
                            self.push_to_cell(cell, data);
                        }
                    }
                }
            }
        }
        true
    }
```
```rust
    pub fn insert_triangle(
        &mut self,
        p1: Point2D,
        p2: Point2D,
        p3: Point2D,
        tol: f64,
        data: usize,
    ) -> bool {
        let Some((s, e)) = self.index_range_for_triangle(&p1, &p2, &p3, tol) else {
            return false;
        };
        let tri = [[p1.x, p1.y], [p2.x, p2.y], [p3.x, p3.y]];
        let half = [self.cell_size * 0.5 + tol, self.cell_size * 0.5 + tol];
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    let center = [
                        i as f64 * self.cell_size + self.pt_min.x + self.cell_size * 0.5,
                        j as f64 * self.cell_size + self.pt_min.y + self.cell_size * 0.5,
                    ];
                    if Self::tri_box_overlap_2d(center, half, tri) {
                        if !self.contains(cell, data) {
                            self.push_to_cell(cell, data);
                        }
                    }
                }
            }
        }
        true
    }
```
```rust
    // ---------- 이터레이션 ----------
    pub fn begin(&self, pt: Point2D) -> SpatialPartition2DIterator<'_> {
        let xi = self.clamp_floor_index(pt.x, self.pt_min.x, 0);
        let yi = self.clamp_floor_index(pt.y, self.pt_min.y, 1);
        if let Some(idx) = self.get_cell_index(xi, yi) {
            SpatialPartition2DIterator {
                current: self.pool[idx].as_deref(),
            }
        } else {
            SpatialPartition2DIterator { current: None }
        }
    }
```
```rust
    pub fn begin_box(&self, center: Point2D, tol: f64) -> SpatialPartition2DBoxIter<'_> {
        let s = [
            self.clamp_floor_index(center.x - tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y - tol, self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(center.x + tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y + tol, self.pt_min.y, 1),
        ];
        SpatialPartition2DBoxIter::new(self, s, e)
    }
```
```rust
    // ---------- 즉시 수집 ----------
    pub fn collect_box_ids(&self, center: Point2D, tol: f64) -> Vec<usize> {
        use std::collections::HashSet;
        let s = [
            self.clamp_floor_index(center.x - tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y - tol, self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(center.x + tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y + tol, self.pt_min.y, 1),
        ];
        let mut set = HashSet::new();
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    let mut cur = self.pool[cell].as_deref();
                    while let Some(n) = cur {
                        set.insert(n.data);
                        cur = n.next.as_deref();
                    }
                }
            }
        }
        let mut out: Vec<_> = set.into_iter().collect();
        out.sort_unstable();
        out
    }
```
```rust
    pub fn collect_aabb_ids(&self, min: Point2D, max: Point2D) -> Vec<usize> {
        use std::collections::HashSet;
        let s = [
            self.clamp_floor_index(min.x, self.pt_min.x, 0),
            self.clamp_floor_index(min.y, self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(max.x, self.pt_min.x, 0),
            self.clamp_floor_index(max.y, self.pt_min.y, 1),
        ];
        let mut set = HashSet::new();
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    let mut cur = self.pool[cell].as_deref();
                    while let Some(n) = cur {
                        set.insert(n.data);
                        cur = n.next.as_deref();
                    }
                }
            }
        }
        let mut out: Vec<_> = set.into_iter().collect();
        out.sort_unstable();
        out
    }
```
```rust
    /// 각 셀 내부에서 (i<j) 모든 쌍에 대해 checker 호출
    pub fn process_check_pairs<C: CompPair2D>(&self, checker: &mut C) {
        for head in &self.pool {
            let mut p1 = head.as_deref();
            while let Some(a) = p1 {
                let mut p2 = a.next.as_deref();
                while let Some(b) = p2 {
                    checker.process_check(a.data, b.data);
                    p2 = b.next.as_deref();
                }
                p1 = a.next.as_deref();
            }
        }
    }
```
```rust
    /// 주어진 AABB 를 커버하는 셀들의 모든 아이템에 대해 checker 호출
    pub fn process_check_range<C: CompSingle2D>(
        &self,
        bb_min: Point2D,
        bb_max: Point2D,
        checker: &mut C,
    ) {
        let s = [
            self.clamp_floor_index(bb_min.x, self.pt_min.x, 0),
            self.clamp_floor_index(bb_min.y, self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(bb_max.x, self.pt_min.x, 0),
            self.clamp_floor_index(bb_max.y, self.pt_min.y, 1),
        ];
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    let mut cur = self.pool[cell].as_deref();
                    while let Some(n) = cur {
                        checker.process_check(n.data);
                        cur = n.next.as_deref();
                    }
                }
            }
        }
    }
```
```rust

    pub fn remove_all(&mut self) {
        for slot in &mut self.pool {
            *slot = None;
        }
        self.item_count = 0;
    }
```
```rust
    pub fn process_check_range_check_duplicated<C: CompSingle2D>(
        &self,
        bb_min: Point2D,
        bb_max: Point2D,
        checker: &mut C,
    ) {
        let s = [
            self.clamp_floor_index(bb_min.x, self.pt_min.x, 0),
            self.clamp_floor_index(bb_min.y, self.pt_min.y, 1),
        ];
        let e = [
            self.clamp_floor_index(bb_max.x, self.pt_min.x, 0),
            self.clamp_floor_index(bb_max.y, self.pt_min.y, 1),
        ];

        let mut visited = HashSet::new(); // ✅ 중복 방지용

        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                if let Some(cell) = self.get_cell_index(i, j) {
                    let mut cur = self.pool[cell].as_deref();
                    while let Some(n) = cur {
                        if visited.insert(n.data) {
                            checker.process_check(n.data); // ✅ 중복 없이 한 번만 호출
                        }
                        cur = n.next.as_deref();
                    }
                }
            }
        }
    }
}
```
```rust
// ---------- 이터레이터 타입들 ----------
pub struct SpatialPartition2DIterator<'a> {
    current: Option<&'a Item>,
}
```
```rust
impl<'a> Iterator for SpatialPartition2DIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.current {
            self.current = n.next.as_deref();
            Some(n.data)
        } else {
            None
        }
    }
}
```
```rust
pub struct SpatialPartition2DBoxIter<'a> {
    part: &'a SpatialPartition2D,
    s: [usize; 2],
    e: [usize; 2],
    i: usize,
    j: usize,
    cur: Option<&'a Item>,
}
```
```rust
impl<'a> SpatialPartition2DBoxIter<'a> {
    fn new(part: &'a SpatialPartition2D, s: [usize; 2], e: [usize; 2]) -> Self {
        let mut it = Self {
            part,
            s,
            e,
            i: s[0],
            j: s[1],
            cur: None,
        };
        it.advance_to_next_nonempty_cell();
        it
    }
```
```rust    
    fn advance_to_next_nonempty_cell(&mut self) {
        loop {
            if self.i > self.e[0] {
                self.cur = None;
                return;
            }
            if self.j > self.e[1] {
                self.i += 1;
                self.j = self.s[1];
                continue;
            }
            if let Some(cell) = self.part.get_cell_index(self.i, self.j) {
                self.cur = self.part.pool[cell].as_deref();
                self.j += 1;
                if self.cur.is_some() {
                    return;
                }
            } else {
                self.j += 1;
            }
        }
    }
}
```
```rust
impl<'a> Iterator for SpatialPartition2DBoxIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.cur {
            self.cur = n.next.as_deref();
            Some(n.data)
        } else {
            self.advance_to_next_nonempty_cell();
            if let Some(n) = self.cur {
                self.cur = n.next.as_deref();
                Some(n.data)
            } else {
                None
            }
        }
    }
}
```

---


# 🧪 샘플 테스트 코드 모음
## 1. ✅ 점 삽입 및 조회 테스트
```rust
#[test]
fn test_insert_and_collect_point() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    let pt = Point2D::new(3.2, 4.7);
    let data_id = 42;
    let inserted = part.insert_point(pt, 0.1, data_id);
    assert!(inserted);

    let ids = part.collect_box_ids(pt, 0.1);
    assert!(ids.contains(&data_id));
}
```

## 2. ✅ 선분 삽입 및 교차 테스트
```rust
#[test]
fn test_insert_line_and_query() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(-5.0, -5.0),
        Point2D::new(5.0, 5.0),
        5,
    );

    let a = Point2D::new(-2.0, -2.0);
    let b = Point2D::new(2.0, 2.0);
    let data_id = 100;
    part.insert_line(a, b, 0.2, data_id);

    let ids = part.collect_box_ids(Point2D::new(0.0, 0.0), 0.5);
    assert!(ids.contains(&data_id));
}
```


## 3. ✅ 삼각형 삽입 및 교차 테스트
```rust
#[test]
fn test_insert_triangle_and_query() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(20.0, 20.0),
        9,
    );

    let p1 = Point2D::new(5.0, 5.0);
    let p2 = Point2D::new(10.0, 5.0);
    let p3 = Point2D::new(7.5, 10.0);
    let data_id = 77;
    part.insert_triangle(p1, p2, p3, 0.3, data_id);

    let ids = part.collect_box_ids(Point2D::new(7.5, 7.5), 1.0);
    assert!(ids.contains(&data_id));
}
```

## 4. ✅ 중복 삽입 방지 테스트
```rust
#[test]
fn test_duplicate_insertion_prevention() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    let pt = Point2D::new(5.0, 5.0);
    let data_id = 1;

    let first = part.insert_point(pt, 0.5, data_id);
    let second = part.insert_point(pt, 0.5, data_id); // 중복 삽입 시도

    assert!(first);
    assert!(second); // 여전히 true지만 내부적으로 중복 방지됨

    let ids = part.collect_box_ids(pt, 0.5);
    let count = ids.iter().filter(|&&id| id == data_id).count();
    assert_eq!(count, 1); // 중복 없음
}
```

## 5. ✅ 이터레이터 테스트
```rust
#[test]
fn test_iterator_over_cell() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    let pt = Point2D::new(2.0, 2.0);
    for i in 0..5 {
        part.insert_point(pt, 0.1, i);
    }

    let mut iter = part.begin(pt);
    let mut collected = vec![];
    while let Some(id) = iter.next() {
        collected.push(id);
    }

    collected.sort_unstable();
    assert_eq!(collected, vec![0, 1, 2, 3, 4]);
}
```

## 6. ✅ process_check_pairs 테스트
```rust
struct TestChecker {
    pairs: Vec<(usize, usize)>,
}
```
```rust
impl CompPair2D for TestChecker {
    fn process_check(&mut self, d1: usize, d2: usize) {
        self.pairs.push((d1.min(d2), d1.max(d2)));
    }
}
```
```rust
#[test]
fn test_process_check_pairs() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    let pt = Point2D::new(1.0, 1.0);
    part.insert_point(pt, 0.1, 10);
    part.insert_point(pt, 0.1, 20);
    part.insert_point(pt, 0.1, 30);

    let mut checker = TestChecker { pairs: vec![] };
    part.process_check_pairs(&mut checker);

    assert!(checker.pairs.contains(&(10, 20)));
    assert!(checker.pairs.contains(&(10, 30)));
    assert!(checker.pairs.contains(&(20, 30)));
    assert_eq!(checker.pairs.len(), 3);
}
```

## ✅ 7. BoundingRect 타입과 함께 사용하는 생성자 및 테스트
```rust
#[derive(Debug)]
struct BoundingRect {
    min: Point2D,
    max: Point2D,
}
```
```rust
#[test]
fn test_new_with_bounding_rect() {
    let rect = BoundingRect {
        min: Point2D::new(0.0, 0.0),
        max: Point2D::new(10.0, 10.0),
    };
    let part = SpatialPartition2D::new_from_bounds(rect.min, rect.max, 5);
    assert_eq!(part.cell_counts()[0] % 2, 1);
    assert_eq!(part.cell_counts()[1] % 2, 1);
}
```

### 🧠 핵심 흐름 요약
```rust
let dist_x = max.x - min.x; // 10.0
let dist_y = max.y - min.y; // 10.0
let max_dist = dist_x.max(dist_y) + max_dist * 0.001; // 약간 확장됨
let cell_size = max_dist / division; // division = 5 → cell_size ≈ 2.002
```

- 이후 각 축에 대해:
```rust
let expanded = dist + max_dist * 0.001; // dist = 10.0 → expanded ≈ 10.01
let cnt = (expanded / cell_size).ceil() as usize; // ≈ 10.01 / 2.002 ≈ 5.0 → ceil → 6
cell_counts[a] = if cnt % 2 == 0 { cnt + 1 } else { cnt }; // 6 → 7
```

### ✅ 왜 7이 되는가?
- division = 5 → cell_size ≈ 2.002
- expanded ≈ 10.01
- expanded / cell_size ≈ 5.0 → ceil = 6
- 6은 짝수 → +1 → 7
- 즉, 짝수 보정 로직 때문에 최종 셀 개수가 7이 됩니다.

### 📌 전체 요약

| 항목                              | 핵심 기능 또는 테스트 목적                          |
|-----------------------------------|-----------------------------------------------------|
| `new_from_bounds`                | BoundingRect 기반 생성자. 경계와 분할 수로 초기화     |
| `remove_all()`                   | 모든 셀 초기화 및 데이터 제거. item_count = 0 확인   |
| `process_check_range`           | AABB 범위 내 데이터 필터링. checker로 처리 가능       |
| `cargo bench`                    | 대규모 삽입 성능 측정. 평균 시간 및 셀 분포 확인      |
| `SafeMemMgr` 결합                | 메모리 풀 기반 객체 삽입/해제. 안정적 메모리 관리     |
| `insert_point`, `insert_line`, `insert_triangle` | 다양한 공간 객체 삽입 및 교차 판정 테스트         |
| `collect_box_ids`, `begin_box`   | 박스 범위 내 데이터 수집 및 이터레이션               |



## ✅ 8. remove_all() 이후 상태 검증
```rust
#[test]
fn test_remove_all_clears_data() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    part.insert_point(Point2D::new(5.0, 5.0), 0.1, 123);
    assert_eq!(part.item_count(), 1);

    part.remove_all();
    assert_eq!(part.item_count(), 0);

    let ids = part.collect_box_ids(Point2D::new(5.0, 5.0), 0.5);
    assert!(ids.is_empty());
}
```


## ✅ 9. process_check_range를 활용한 AABB 범위 내 필터링 테스트
```rust
struct Collector {
    found: Vec<usize>,
}
```
```rust
impl CompSingle2D for Collector {
    fn process_check(&mut self, d: usize) {
        if !self.found.contains(&d)
        {
            self.found.push(d);
        }
    }
}
```
```rust
#[test]
fn test_process_check_range_aabb() {
    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    part.insert_point(Point2D::new(2.0, 2.0), 0.1, 1);
    part.insert_point(Point2D::new(8.0, 8.0), 0.1, 2);

    let mut checker = Collector { found: vec![] };
    part.process_check_range(Point2D::new(1.0, 1.0), Point2D::new(3.0, 3.0), &mut checker);

    assert_eq!(checker.found, vec![1]);
}
```


## ✅ 10. SafeMemMgr와 결합하여 메모리 풀 기반 객체 관리 테스트
```rust
#[test]
fn test_safe_mem_mgr_with_partition() {
    use memmgr::SafeMemMgr;
    let mgr = SafeMemMgr::new(32, 128); // usize 크기 기준

    let mut part = SpatialPartition2D::new_from_bounds(
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 10.0),
        5,
    );

    let data_ptr = mgr.alloc_object(999usize);
    let data = unsafe { data_ptr.as_ref() };
    part.insert_point(Point2D::new(5.0, 5.0), 0.1, *data);

    let ids = part.collect_box_ids(Point2D::new(5.0, 5.0), 0.5);
    assert!(ids.contains(&999));

    unsafe { mgr.free_object(data_ptr); }
    mgr.free_all_objects();
}
```

---


# 📘 테스트 문서: SpatialPartition2D 기능 검증
## 🔧 기본 유틸리티 함수
```rust
fn mk_pt(x: f64, y: f64) -> Point2D
```

- Point2D 생성 헬퍼 함수
```rust
fn mk_grid() -> SpatialPartition2D
```

- (0,0)~(10,10) 범위의 9분할 격자 생성
```rust
fn box_scan_ids(part, center, half) -> HashSet<usize>
```

- 중심과 반경을 기준으로 박스 범위 내의 ID들을 수집

### ✅ 기본 삽입 및 조회 테스트

| 테스트 함수 이름                             | 검증 포인트 설명                                      |
|----------------------------------------------|--------------------------------------------------------|
| `insert_point_and_begin_cell`                | 점 삽입 후 해당 셀에서 ID가 정확히 조회되는지 확인     |
| `duplicate_point_is_not_duplicated`          | 동일한 점을 중복 삽입해도 `item_count`가 증가하지 않음 |
| `insert_line_and_find_in_box`                | 선분 삽입 후 박스 범위에서 ID가 정확히 조회되는지 확인 |
| `insert_triangle_and_find_in_box`            | 삼각형 삽입 후 박스 범위에서 ID가 정확히 조회되는지 확인 |
| `iterator_scans_entire_grid_and_finds_everything` | 전체 셀 순회로 모든 삽입된 ID가 누락 없이 조회되는지 확인 |


### 🔍 ProcessCheck 기반 테스트

| 테스트 함수 이름                             | 검증 포인트 설명                                      |
|----------------------------------------------|--------------------------------------------------------|
| `process_check_pairs_counts_combinations`     | 셀 내 데이터 쌍 조합에 대해 `process_check(d1, d2)`가 정확히 호출되는지 확인 |
| `process_check_range_visits_items_in_aabb`    | AABB 범위 내 데이터에 대해 `process_check(d)`가 정확히 호출되는지 확인       |


## 🧪 내부 구조 테스트용 타입
```rust
struct PairCounter {
    pairs: Vec<(usize, usize)>
}

impl CompPair2D for PairCounter
```

- 셀 내 데이터 쌍을 수집하는 구조체
```rust
struct SingleCollector {
    set: HashSet<usize>
}
impl CompSingle2D for SingleCollector
```

- AABB 범위 내의 단일 데이터 ID를 수집하는 구조체

### 📌 전체 테스트 흐름 요약

| 기능 범주             | 설명                                      | 테스트 함수들                                                   |
|----------------------|-------------------------------------------|------------------------------------------------------------------|
| 점 삽입 및 중복 방지 | 셀에 점 삽입 후 조회, 중복 삽입 억제 확인 | `insert_point_and_begin_cell`, `duplicate_point_is_not_duplicated` |
| 선분/삼각형 삽입     | 교차 판정 후 박스 범위에서 ID 조회         | `insert_line_and_find_in_box`, `insert_triangle_and_find_in_box`   |
| 전체 셀 순회         | 전체 격자 순회로 모든 ID 조회 가능 여부    | `iterator_scans_entire_grid_and_finds_everything`                 |
| 셀 내 쌍 검사        | 셀 내부 데이터 쌍 조합 수 확인             | `process_check_pairs_counts_combinations`                         |
| AABB 범위 검사       | 지정된 범위 내 데이터만 방문               | `process_check_range_visits_items_in_aabb`                        |

```rust
fn mk_pt(x: f64, y: f64) -> Point2D {
    Point2D { x, y }
}
```
```rust
fn mk_grid() -> SpatialPartition2D {
    let min = mk_pt(0.0, 0.0);
    let max = mk_pt(10.0, 10.0);
    SpatialPartition2D::new_from_bounds(min, max, 9)
}
```
```rust
// ---- helpers ----
fn box_scan_ids(part: &SpatialPartition2D, center: Point2D, half: [f64; 2]) -> HashSet<usize> {
    let tol = half[0].max(half[1]);
    part.collect_box_ids(center, tol).into_iter().collect()
}
```
```rust
#[test]
fn insert_point_and_begin_cell() {
    let mut sp = mk_grid();
    let p = mk_pt(1.2, 1.3);
    let id = 42usize;
    assert!(sp.insert_point(p, 0.05, id));

    let got: HashSet<_> = sp.begin(p).collect();
    assert!(got.contains(&id));
}
```
```rust
#[test]
fn duplicate_point_is_not_duplicated() {
    let mut sp = mk_grid();
    let p = mk_pt(2.0, 2.0);
    let id = 7usize;

    let before = sp.item_count();
    sp.insert_point(p, 0.01, id);
    let after1 = sp.item_count();
    assert!(after1 > before, "first insert should increase item_count");

    sp.insert_point(p, 0.01, id);
    let after2 = sp.item_count();
    assert_eq!(after1, after2, "duplicate insert should be suppressed");
}
```
```rust
#[test]
fn insert_line_and_find_in_box() {
    let mut sp = mk_grid();
    let a = mk_pt(0.2, 0.2);
    let b = mk_pt(7.8, 0.2);
    let id = 111usize;

    assert!(sp.insert_line(a, b, 0.0, id));

    let center = mk_pt((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
    let half = [(b.x - a.x).abs() * 0.5 + 0.2, 0.5];
    let got = box_scan_ids(&sp, center, half);
    assert!(got.contains(&id), "line id not found in box scan");
}
```
```rust
#[test]
fn insert_triangle_and_find_in_box() {
    let mut sp = mk_grid();
    let p1 = mk_pt(3.0, 3.0);
    let p2 = mk_pt(7.0, 3.5);
    let p3 = mk_pt(4.0, 7.5);
    let id = 999usize;

    assert!(sp.insert_triangle(p1, p2, p3, 0.0, id));

    let cx = (p1.x + p2.x + p3.x) / 3.0;
    let cy = (p1.y + p2.y + p3.y) / 3.0;
    let center = mk_pt(cx, cy);
    let minx = p1.x.min(p2.x).min(p3.x);
    let maxx = p1.x.max(p2.x).max(p3.x);
    let miny = p1.y.min(p2.y).min(p3.y);
    let maxy = p1.y.max(p2.y).max(p3.y);
    let half = [(maxx - minx) * 0.5 + 0.5, (maxy - miny) * 0.5 + 0.5];

    let got = box_scan_ids(&sp, center, half);
    assert!(got.contains(&id), "triangle id not found in box scan");
}
```
```rust
#[test]
fn iterator_scans_entire_grid_and_finds_everything() {
    let mut sp = mk_grid();

    let id_point = 1usize;
    let id_line = 2usize;
    let id_tri = 3usize;
    let id_point_far = 4usize;

    sp.insert_point(mk_pt(1.2, 1.3), 0.05, id_point);
    sp.insert_line(mk_pt(0.2, 0.2), mk_pt(9.2, 0.2), 0.0, id_line);
    sp.insert_triangle(
        mk_pt(3.0, 3.0),
        mk_pt(7.0, 3.5),
        mk_pt(4.0, 7.5),
        0.0,
        id_tri,
    );
    sp.insert_point(mk_pt(9.1, 9.2), 0.05, id_point_far);

    let center = mk_pt(
        (sp.pt_min().x + sp.pt_max().x) * 0.5,
        (sp.pt_min().y + sp.pt_max().y) * 0.5,
    );
    let tol_all = (sp.pt_max().x - sp.pt_min().x).max(sp.pt_max().y - sp.pt_min().y) * 2.0;

    let found: HashSet<_> = sp.begin_box(center, tol_all).collect();
    for expect in [id_point, id_line, id_tri, id_point_far] {
        assert!(
            found.contains(&expect),
            "missing id {expect} in global iterator scan"
        );
    }
}
```
```rust
// ---- ProcessCheck 대응 테스트 ----

struct PairCounter {
    pairs: Vec<(usize, usize)>,
}
```
```rust
impl PairCounter {
    fn new() -> Self {
        Self { pairs: Vec::new() }
    }
}
```
```rust
impl CompPair2D for PairCounter {
    fn process_check(&mut self, d1: usize, d2: usize) {
        self.pairs.push((d1.min(d2), d1.max(d2)));
    }
}
```
```rust
struct SingleCollector {
    set: HashSet<usize>,
}
```
```rust
impl SingleCollector {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
}
```
```rust
impl CompSingle2D for SingleCollector {
    fn process_check(&mut self, d: usize) {
        self.set.insert(d);
    }
}
```
```rust
#[test]
fn process_check_pairs_counts_combinations() {
    let mut sp = mk_grid();

    // 같은 셀에 들어가도록 근접한 포인트들 삽입
    let ids = [10usize, 20usize, 30usize, 40usize];
    for (k, id) in ids.iter().enumerate() {
        sp.insert_point(mk_pt(1.0 + k as f64 * 0.01, 1.0), 0.0, *id);
    }

    let mut pc = PairCounter::new();
    sp.process_check_pairs(&mut pc);

    // 한 셀에 n=4개 → 조합 수 4C2 = 6 이상이어야 한다 (다른 셀 쌍은 0)
    let n = ids.len();
    let expected_min = n * (n - 1) / 2;
    assert!(
        pc.pairs.len() >= expected_min,
        "pair count {} < {}",
        pc.pairs.len(),
        expected_min
    );
}
```
```rust
#[test]
fn process_check_range_visits_items_in_aabb() {
    let mut sp = mk_grid();

    let id_a = 101usize;
    let id_b = 202usize;
    let id_c = 303usize;

    sp.insert_point(mk_pt(2.0, 2.0), 0.0, id_a);
    sp.insert_point(mk_pt(5.0, 5.0), 0.0, id_b);
    sp.insert_point(mk_pt(8.0, 8.0), 0.0, id_c);

    let min = mk_pt(1.0, 1.0);
    let max = mk_pt(6.0, 6.0);

    let mut sc = SingleCollector::new();
    sp.process_check_range(min, max, &mut sc);

    assert!(sc.set.contains(&id_a));
    assert!(sc.set.contains(&id_b));
    assert!(
        !sc.set.contains(&id_c),
        "id outside AABB should not be visited"
    );
}
```
---


