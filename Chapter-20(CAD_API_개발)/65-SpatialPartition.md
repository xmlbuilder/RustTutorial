# Spatial Partition
3D 공간 파티셔너 SpatialPartition의 핵심 수식과 함수들을 먼저 정리하고,  
그 수식들이 수학적으로 타당한지 검증.

## 🧮 핵심 수식 정리
### 1. 셀 인덱스 계산

$$
\mathrm{floor\\_index}(coord,min)=\left\lfloor \frac{coord-min}{cell\\_ size}\right\rfloor
$$

$$
\mathrm{clamp\\_floor\\_index}(coord,min,axis)=\min \left( \max \left( \left\lfloor \frac{coord-min}{cell\\_size}\right\rfloor ,0\right) ,cell\\_counts[axis]-1\right) 
$$


### 2. 셀 풀 인덱스 (3D → 1D)

$$
\mathrm{get\\_ cell\\_ index}(x_i,y_i,z_i)=(x_i\cdot cell\\_ counts[1]+y_i)\cdot cell\\_ counts[2]+z_i
$$

### 3. 셀 중심 좌표

$$
\mathrm{cell\\_ center}(i,j,k)=\left[ i\cdot cell\\_ size+\frac{cell\\_ size}{2}+pt_{min,x},j\cdot cell\\_ size+\frac{cell\\_ size}{2}+pt_{min,y},k\cdot cell\\_ size+\frac{cell\\_ size}{2}+pt_{min,z}\right] 
$$

### 4. 셀 AABB 경계


$$
\mathrm{cell\\_ aabb}(i,j,k,tol)=\left[ \mathrm{min}=i\cdot cell\\_ size+pt_{min}-tol,\mathrm{max}=(i+1)\cdot cell\\_ size+pt_{min}+tol\right] 
$$

### 5. 선분 vs AABB 교차 (slab 방식)

$$
t_{\mathrm{entry}}=\max _{ax}\left( \frac{bmin[ax]-start[ax]}{end[ax]-start[ax]}\right) ,\quad t_{\mathrm{exit}}=\min _{ax}\left( \frac{bmax[ax]-start[ax]}{end[ax]-start[ax]}\right) 
\quad 조건: t_{\mathrm{entry}}\leq t_{\mathrm{exit}}
$$

### 6. 삼각형 vs AABB 교차 (SAT 방식)
- 축: AABB(X,Y,Z) + 삼각형 edge × AABB 축 (총 13개)
- 프로젝션:

$$
\mathrm{rad}=|n_x|\cdot h_x+|n_y|\cdot h_y+|n_z|\cdot h_z
$$

$$
\mathrm{proj_{\mathnormal{i}}}=n\cdot v_i
$$

$$
\mathrm{조건:\  }\min (\mathrm{proj})\leq \mathrm{rad}\wedge \max (\mathrm{proj})\geq -\mathrm{rad}
$$

### 📌 SpatialPartition 주요 함수 요약

| 함수 이름                  | 설명                                                             |
|---------------------------|------------------------------------------------------------------|
| `new`                     | BoundingBox와 division으로 3D 격자 초기화                         |
| `get_cell_index`          | (x,y,z) 셀 좌표 → 1D 풀 인덱스로 변환                            |
| `floor_index`             | 좌표를 셀 인덱스로 변환                                          |
| `clamp_floor_index`       | 셀 인덱스를 경계 내로 보정                                       |
| `cell_center`             | 셀 중심 좌표 계산                                                |
| `cell_aabb`               | 셀의 AABB 경계 계산                                              |
| `index_range_for_point`   | 점 기준 AABB 인덱스 범위 계산                                    |
| `index_range_for_line`    | 선분 기준 AABB 인덱스 범위 계산                                  |
| `index_range_for_triangle`| 삼각형 기준 AABB 인덱스 범위 계산                                |
| `line_box_overlap`        | 선분과 AABB의 교차 여부 slab 방식으로 판정                       |
| `tri_box_overlap`         | 삼각형과 AABB의 교차 여부 SAT 방식으로 판정                      |
| `plane_box_overlap`       | 평면과 AABB의 교차 여부 판정                                    |
| `insert_point`            | 점 삽입. 교차 셀에 중복 없이 삽입                                |
| `insert_line`             | 선분 삽입. 교차 셀에 중복 없이 삽입                              |
| `insert_triangle`         | 삼각형 삽입. 교차 셀에 중복 없이 삽입                            |
| `contains`                | 셀 내에 특정 데이터가 이미 존재하는지 검사                        |
| `push_to_cell`            | 셀에 데이터를 연결 리스트 방식으로 삽입                           |
| `begin`                   | 특정 좌표의 셀에 대한 이터레이터 생성                            |
| `begin_box`               | 박스 범위에 대한 셀 이터레이터 생성                              |
| `collect_box_ids`         | 박스 범위 내의 모든 데이터 ID 수집                               |


### ✅ 수식 검증 결과

| 수식 항목                  | 검증 결과 설명                                      |
|---------------------------|-----------------------------------------------------|
| 셀 인덱스 계산             | ✅ `floor(coord - min) / cell_size`는 안정적이며 경계 보정 포함 |
| 3D → 1D 인덱스 변환        | ✅ `(xi * Y + yi) * Z + zi`는 전형적인 행-열-깊이 순서로 정확 |
| 셀 중심 좌표 계산          | ✅ 셀의 중심은 `pt_min + cell_size * (i + 0.5)`로 정확히 위치함 |
| 셀 AABB 경계 계산          | ✅ `min/max` 경계는 셀 크기 기반으로 정확히 확장됨 (`±tol`)     |
| 선분 vs AABB 교차 판정     | ✅ slab 방식으로 각 축에서 `t_entry`, `t_exit` 계산이 정확함     |
| 삼각형 vs AABB 교차 판정   | ✅ SAT 방식으로 13개 축에 대해 프로젝션 판정이 수학적으로 타당함 |
| 평면 vs AABB 교차 판정     | ✅ dot 계산 기반으로 평면과 박스의 교차 여부를 안정적으로 판정함 |


```rust
use crate::math::boundingbox::BoundingBox;
use crate::math::prelude::Point3D;
use std::collections::HashSet;

#[derive(Debug)]
struct Item {
    data: usize,
    next: Option<Box<Item>>,
}
```
```rust
pub struct SpatialPartition {
    cell_counts: [usize; 3],
    cell_size: f64,
    pt_min: Point3D,
    pt_max: Point3D,
    pool: Vec<Option<Box<Item>>>,
    item_count: usize,
}
```
```rust
impl SpatialPartition {
    // --------------------
    // Getters (읽기)
    // --------------------

    /// 각 축(x,y,z)의 셀 개수
    #[inline]
    pub fn cell_counts(&self) -> [usize; 3] {
        self.cell_counts
    }
```
```rust
    /// 셀 한 변의 길이
    #[inline]
    pub fn cell_size(&self) -> f64 {
        self.cell_size
    }
```
```rust
    #[inline]
    pub fn pt_min(&self) -> &Point3D {
        &self.pt_min
    }
```
```rust
    #[inline]
    pub fn pt_max(&self) -> &Point3D {
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
    /// 전체 셀 개수 (pool 크기)
    #[inline]
    pub fn pool_size(&self) -> usize {
        self.pool.len()
    }
```
```rust
    #[inline]
    pub fn set_pt_min(&mut self, p: Point3D) {
        self.pt_min = p;
    }
```
```rust
    /// 그리드 최대 코너 설정 (주의: 인덱싱 기준이 바뀝니다)
    #[inline]
    pub fn set_pt_max(&mut self, p: Point3D) {
        self.pt_max = p;
    }
```
```rust
    #[inline]
    pub fn set_bounds(&mut self, min: Point3D, max: Point3D) {
        self.pt_min = min;
        self.pt_max = max;
    }
}
```
```rust
pub struct SpatialPartitionBoxIter<'a> {
    part: &'a SpatialPartition,
    s: [usize; 3],
    e: [usize; 3],
    i: usize,
    j: usize,
    k: usize,
    cur: Option<&'a Item>,
}
```
```rust
impl<'a> SpatialPartitionBoxIter<'a> {
    fn new(part: &'a SpatialPartition, s: [usize; 3], e: [usize; 3]) -> Self {
        let mut it = Self {
            part,
            s,
            e,
            i: s[0],
            j: s[1],
            k: s[2],
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
            if self.k > self.e[2] {
                self.j += 1;
                self.k = self.s[2];
                continue;
            }

            if let Some(cell) = self.part.get_cell_index(self.i, self.j, self.k) {
                self.cur = self.part.pool[cell].as_deref();
                self.k += 1;
                if self.cur.is_some() {
                    return;
                }
            } else {
                self.k += 1;
            }
        }
    }
}
```
```rust
impl<'a> Iterator for SpatialPartitionBoxIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.cur {
            self.cur = node.next.as_deref();
            Some(node.data)
        } else {
            self.advance_to_next_nonempty_cell();
            if let Some(node) = self.cur {
                self.cur = node.next.as_deref();
                Some(node.data)
            } else {
                None
            }
        }
    }
}
```
```rust
impl SpatialPartition {
    pub fn new(bbox: &BoundingBox, division: usize) -> Self {
        let div = if division % 2 == 0 {
            division + 1
        } else {
            division
        };
        let mut max_dist = bbox.diagonal().length();
        max_dist += max_dist * 0.001;

        let cell_size = max_dist / div as f64;

        let mut cell_counts = [0usize; 3];
        for i in 0..3 {
            let dist = bbox.max[i] - bbox.min[i];
            let expanded = dist + max_dist * 0.001;
            let count = (expanded / cell_size).ceil() as usize;
            cell_counts[i] = if count % 2 == 0 { count + 1 } else { count };
        }

        let center = bbox.center();
        let pt_min = Point3D::new(
            center.x - cell_size * cell_counts[0] as f64 * 0.5,
            center.y - cell_size * cell_counts[1] as f64 * 0.5,
            center.z - cell_size * cell_counts[2] as f64 * 0.5,
        );
        let pt_max = Point3D::new(
            pt_min.x + cell_size * cell_counts[0] as f64,
            pt_min.y + cell_size * cell_counts[1] as f64,
            pt_min.z + cell_size * cell_counts[2] as f64,
        );

        let pool_size = cell_counts[0] * cell_counts[1] * cell_counts[2];

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
}
```
```rust
impl SpatialPartition {
    #[inline]
    fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
```
```rust    
    #[inline]
    fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }
```
```rust    
    #[inline]
    fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
```
```rust
    pub fn get_cell_index(&self, xi: usize, yi: usize, zi: usize) -> Option<usize> {
        if xi >= self.cell_counts[0] || yi >= self.cell_counts[1] || zi >= self.cell_counts[2] {
            return None;
        }
        Some((xi * self.cell_counts[1] + yi) * self.cell_counts[2] + zi)
    }
```
```rust
    fn floor_index(&self, coord: f64, min: f64) -> usize {
        ((coord - min) / self.cell_size).floor() as usize
    }
```
```rust
    #[inline]
    fn cell_center(&self, i: usize, j: usize, k: usize) -> [f64; 3] {
        let h = self.cell_size * 0.5;
        [
            i as f64 * self.cell_size + h + self.pt_min.x,
            j as f64 * self.cell_size + h + self.pt_min.y,
            k as f64 * self.cell_size + h + self.pt_min.z,
        ]
    }
```
```rust
    #[inline]
    fn cell_aabb(&self, i: usize, j: usize, k: usize, tol: f64) -> ([f64; 3], [f64; 3]) {
        let min = [
            i as f64 * self.cell_size + self.pt_min.x - tol,
            j as f64 * self.cell_size + self.pt_min.y - tol,
            k as f64 * self.cell_size + self.pt_min.z - tol,
        ];
        let max = [
            (i + 1) as f64 * self.cell_size + self.pt_min.x + tol,
            (j + 1) as f64 * self.cell_size + self.pt_min.y + tol,
            (k + 1) as f64 * self.cell_size + self.pt_min.z + tol,
        ];
        (min, max)
    }
```
```rust
    #[inline]
    pub fn clamp_floor_index(&self, coord: f64, min: f64, axis: usize) -> usize {
        // 좌표를 셀 인덱스로 변환하되 0..=cell_counts[axis]-1 로 클램프
        let rel = (coord - min) / self.cell_size;
        let raw = rel.floor();
        if raw.is_nan() {
            return 0;
        }
        if raw <= 0.0 {
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
    #[allow(unused)]
    fn index_range_for_point(&self, pt: &Point3D, tol: f64) -> Option<([usize; 3], [usize; 3])> {
        let s = [
            self.clamp_floor_index(pt.x - tol, self.pt_min.x, 0),
            self.clamp_floor_index(pt.y - tol, self.pt_min.y, 1),
            self.clamp_floor_index(pt.z - tol, self.pt_min.z, 2),
        ];
        let e = [
            self.clamp_floor_index(pt.x + tol, self.pt_min.x, 0),
            self.clamp_floor_index(pt.y + tol, self.pt_min.y, 1),
            self.clamp_floor_index(pt.z + tol, self.pt_min.z, 2),
        ];
        if s[0] > e[0] || s[1] > e[1] || s[2] > e[2] {
            return None;
        }
        Some((s, e))
    }
```
```rust
    /// 두 점 pt1, pt2에 대해 포함 AABB 인덱스 범위 (C++ GetBoxIndexByLine 과 동일 의도)
    fn index_range_for_line(
        &self,
        p1: &Point3D,
        p2: &Point3D,
        tol: f64,
    ) -> Option<([usize; 3], [usize; 3])> {
        let minp = [
            p1.x.min(p2.x) - tol,
            p1.y.min(p2.y) - tol,
            p1.z.min(p2.z) - tol,
        ];
        let maxp = [
            p1.x.max(p2.x) + tol,
            p1.y.max(p2.y) + tol,
            p1.z.max(p2.z) + tol,
        ];
        let s = [
            self.clamp_floor_index(minp[0], self.pt_min.x, 0),
            self.clamp_floor_index(minp[1], self.pt_min.y, 1),
            self.clamp_floor_index(minp[2], self.pt_min.z, 2),
        ];
        let e = [
            self.clamp_floor_index(maxp[0], self.pt_min.x, 0),
            self.clamp_floor_index(maxp[1], self.pt_min.y, 1),
            self.clamp_floor_index(maxp[2], self.pt_min.z, 2),
        ];
        if s[0] > e[0] || s[1] > e[1] || s[2] > e[2] {
            return None;
        }
        Some((s, e))
    }
```
```rust
    /// 세 점 pt1,2,3에 대해 포함 AABB 인덱스 범위 (C++ GetBoxIndexByTriangle 과 동일 의도)
    fn index_range_for_triangle(
        &self,
        p1: &Point3D,
        p2: &Point3D,
        p3: &Point3D,
        tol: f64,
    ) -> Option<([usize; 3], [usize; 3])> {
        let min_p = [
            p1.x.min(p2.x).min(p3.x) - tol,
            p1.y.min(p2.y).min(p3.y) - tol,
            p1.z.min(p2.z).min(p3.z) - tol,
        ];
        let max_p = [
            p1.x.max(p2.x).max(p3.x) + tol,
            p1.y.max(p2.y).max(p3.y) + tol,
            p1.z.max(p2.z).max(p3.z) + tol,
        ];
        let s = [
            self.clamp_floor_index(min_p[0], self.pt_min.x, 0),
            self.clamp_floor_index(min_p[1], self.pt_min.y, 1),
            self.clamp_floor_index(min_p[2], self.pt_min.z, 2),
        ];
        let e = [
            self.clamp_floor_index(max_p[0], self.pt_min.x, 0),
            self.clamp_floor_index(max_p[1], self.pt_min.y, 1),
            self.clamp_floor_index(max_p[2], self.pt_min.z, 2),
        ];
        if s[0] > e[0] || s[1] > e[1] || s[2] > e[2] {
            return None;
        }
        Some((s, e))
    }
```
```rust
    fn line_box_overlap(
        start: [f64; 3],
        end: [f64; 3],
        bmin: [f64; 3],
        bmax: [f64; 3],
    ) -> Option<f64> {
        let mut fst = 0.0;
        let mut fet = 1.0;
        for ax in 0..3 {
            let si = start[ax];
            let ei = end[ax];
            let di = ei - si;

            if di.abs() < f64::EPSILON {
                if si < bmin[ax] || si > bmax[ax] {
                    return None;
                }
                continue;
            }

            let st;
            let et;
            if si < ei {
                if si > bmax[ax] || ei < bmin[ax] {
                    return None;
                }
                st = if si < bmin[ax] {
                    (bmin[ax] - si) / di
                } else {
                    0.0
                };
                et = if ei > bmax[ax] {
                    (bmax[ax] - si) / di
                } else {
                    1.0
                };
            } else {
                if ei > bmax[ax] || si < bmin[ax] {
                    return None;
                }
                st = if si > bmax[ax] {
                    (bmax[ax] - si) / di
                } else {
                    0.0
                };
                et = if ei < bmin[ax] {
                    (bmin[ax] - si) / di
                } else {
                    1.0
                };
            }

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
    /// 평면–AABB overlap 테스트 보조
    fn plane_box_overlap(normal: [f64; 3], d: f64, half: [f64; 3]) -> bool {
        let mut vmin = [0.0; 3];
        let mut vmax = [0.0; 3];
        for q in 0..3 {
            if normal[q] > 0.0 {
                vmin[q] = -half[q];
                vmax[q] = half[q];
            } else {
                vmin[q] = half[q];
                vmax[q] = -half[q];
            }
        }
        let dn1 = Self::dot(normal, vmin) + d;
        if dn1 > 0.0 {
            return false;
        }
        let dn2 = Self::dot(normal, vmax) + d;
        dn2 >= 0.0
    }
```
```rust
    /// 삼각형 vs AABB (Separation Axis Theorem)
    fn tri_box_overlap(box_center: [f64; 3], box_half: [f64; 3], tri: [[f64; 3]; 3]) -> bool {
        // 삼각형 정점 box 중심 기준으로 이동
        let v0 = Self::sub(tri[0], box_center);
        let v1 = Self::sub(tri[1], box_center);
        let v2 = Self::sub(tri[2], box_center);

        let e0 = Self::sub(v1, v0);
        let e1 = Self::sub(v2, v1);
        let e2 = Self::sub(v0, v2);

        // 9개 축(각 edge x {X,Y,Z})
        let fe0 = [e0[0].abs(), e0[1].abs(), e0[2].abs()];
        let fe1 = [e1[0].abs(), e1[1].abs(), e1[2].abs()];
        let fe2 = [e2[0].abs(), e2[1].abs(), e2[2].abs()];

        // 헬퍼: 프로젝션 범위 테스트
        let axis_test = |a: f64,
                         b: f64,
                         va: [f64; 3],
                         vb: [f64; 3],
                         fa: f64,
                         fb: f64,
                         i0: usize,
                         i1: usize|
         -> bool {
            let p0 = a * va[i0] - b * va[i1];
            let p1 = a * vb[i0] - b * vb[i1];
            let (minp, maxp) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
            let rad = fa * box_half[i0] + fb * box_half[i1];
            !(minp > rad || maxp < -rad)
        };

        // X축 관련 (i0=1(Y), i1=2(Z))
        if !axis_test(e0[2], e0[1], v0, v2, fe0[2], fe0[1], 1, 2) {
            return false;
        }
        if !axis_test(e1[2], e1[1], v0, v1, fe1[2], fe1[1], 1, 2) {
            return false;
        }
        if !axis_test(e2[2], e2[1], v0, v1, fe2[2], fe2[1], 1, 2) {
            return false;
        }

        // Y축 관련 (i0=0(X), i1=2(Z)) ; 부호 주의
        let axis_test_y = |a: f64, b: f64, va: [f64; 3], vb: [f64; 3], fa: f64, fb: f64| -> bool {
            let p0 = -a * va[0] + b * va[2];
            let p1 = -a * vb[0] + b * vb[2];
            let (minp, maxp) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
            let rad = fa * box_half[0] + fb * box_half[2];
            !(minp > rad || maxp < -rad)
        };
        if !axis_test_y(e0[2], e0[0], v0, v2, fe0[2], fe0[0]) {
            return false;
        }
        if !axis_test_y(e1[2], e1[0], v0, v1, fe1[2], fe1[0]) {
            return false;
        }
        if !axis_test_y(e2[2], e2[0], v0, v2, fe2[2], fe2[0]) {
            return false;
        }

        // Z축 관련 (i0=0(X), i1=1(Y))
        let axis_test_z = |a: f64, b: f64, va: [f64; 3], vb: [f64; 3], fa: f64, fb: f64| -> bool {
            let p0 = a * va[0] - b * va[1];
            let p1 = a * vb[0] - b * vb[1];
            let (minp, maxp) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
            let rad = fa * box_half[0] + fb * box_half[1];
            !(minp > rad || maxp < -rad)
        };
        if !axis_test_z(e0[1], e0[0], v1, v2, fe0[1], fe0[0]) {
            return false;
        }
        if !axis_test_z(e1[1], e1[0], v0, v1, fe1[1], fe1[0]) {
            return false;
        }
        if !axis_test_z(e2[1], e2[0], v0, v1, fe2[1], fe2[0]) {
            return false;
        }

        // AABB 축(X/Y/Z)
        let minmax = |a: f64, b: f64, c: f64| -> (f64, f64) {
            let mut minv = a;
            let mut maxv = a;
            if b < minv {
                minv = b;
            }
            if b > maxv {
                maxv = b;
            }
            if c < minv {
                minv = c;
            }
            if c > maxv {
                maxv = c;
            }
            (minv, maxv)
        };
        let (minx, maxx) = minmax(v0[0], v1[0], v2[0]);
        if minx > box_half[0] || maxx < -box_half[0] {
            return false;
        }
        let (miny, maxy) = minmax(v0[1], v1[1], v2[1]);
        if miny > box_half[1] || maxy < -box_half[1] {
            return false;
        }
        let (minz, maxz) = minmax(v0[2], v1[2], v2[2]);
        if minz > box_half[2] || maxz < -box_half[2] {
            return false;
        }

        // 평면 vs AABB
        let n = Self::cross(e0, e1);
        let d = -Self::dot(n, v0);
        if !Self::plane_box_overlap(n, d, box_half) {
            return false;
        }

        true
    }
```
```rust
    pub fn begin_box(&self, center: Point3D, tol: f64) -> SpatialPartitionBoxIter<'_> {
        // 인덱스 범위 계산 (clamp 포함)
        let s = [
            self.clamp_floor_index(center.x - tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y - tol, self.pt_min.y, 1),
            self.clamp_floor_index(center.z - tol, self.pt_min.z, 2),
        ];
        let e = [
            self.clamp_floor_index(center.x + tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y + tol, self.pt_min.y, 1),
            self.clamp_floor_index(center.z + tol, self.pt_min.z, 2),
        ];
        SpatialPartitionBoxIter::new(self, s, e)
    }
```
```rust
    pub fn collect_box_ids(&self, center: Point3D, tol: f64) -> Vec<usize> {
        let s = [
            self.clamp_floor_index(center.x - tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y - tol, self.pt_min.y, 1),
            self.clamp_floor_index(center.z - tol, self.pt_min.z, 2),
        ];
        let e = [
            self.clamp_floor_index(center.x + tol, self.pt_min.x, 0),
            self.clamp_floor_index(center.y + tol, self.pt_min.y, 1),
            self.clamp_floor_index(center.z + tol, self.pt_min.z, 2),
        ];
        let mut set = HashSet::new();
        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                for k in s[2]..=e[2] {
                    if let Some(cell) = self.get_cell_index(i, j, k) {
                        let mut cur = self.pool[cell].as_deref();
                        while let Some(node) = cur {
                            set.insert(node.data);
                            cur = node.next.as_deref();
                        }
                    }
                }
            }
        }
        let mut out: Vec<_> = set.into_iter().collect();
        out.sort_unstable();
        out
    }
}
```
```rust
impl SpatialPartition {
    pub fn insert_point(&mut self, pt: Point3D, tol: f64, data: usize) {
        let s = [
            self.floor_index(pt.x - tol, self.pt_min.x),
            self.floor_index(pt.y - tol, self.pt_min.y),
            self.floor_index(pt.z - tol, self.pt_min.z),
        ];
        let e = [
            self.floor_index(pt.x + tol, self.pt_min.x),
            self.floor_index(pt.y + tol, self.pt_min.y),
            self.floor_index(pt.z + tol, self.pt_min.z),
        ];

        for i in s[0]..=e[0].min(self.cell_counts[0] - 1) {
            for j in s[1]..=e[1].min(self.cell_counts[1] - 1) {
                for k in s[2]..=e[2].min(self.cell_counts[2] - 1) {
                    if let Some(index) = self.get_cell_index(i, j, k) {
                        if !self.contains(index, data) {
                            let new_item = Box::new(Item {
                                data,
                                next: self.pool[index].take(),
                            });
                            self.pool[index] = Some(new_item);
                            self.item_count += 1;
                        }
                    }
                }
            }
        }
    }
```
```rust
    fn contains(&self, index: usize, data: usize) -> bool {
        let mut current = self.pool[index].as_ref();
        while let Some(item) = current {
            if item.data == data {
                return true;
            }
            current = item.next.as_ref();
        }
        false
    }
```
```rust
    pub fn insert_line(&mut self, pt1: Point3D, pt2: Point3D, tol: f64, data: usize) -> bool {
        let Some((s, e)) = self.index_range_for_line(&pt1, &pt2, tol) else {
            return false;
        };

        let a = [pt1.x, pt1.y, pt1.z];
        let b = [pt2.x, pt2.y, pt2.z];

        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                for k in s[2]..=e[2] {
                    if let Some(cell) = self.get_cell_index(i, j, k) {
                        let (bmin, bmax) = self.cell_aabb(i, j, k, tol);
                        if Self::line_box_overlap(a, b, bmin, bmax).is_some() {
                            if !self.contains(cell, data) {
                                self.push_to_cell(cell, data);
                            }
                        }
                    }
                }
            }
        }
        true
    }
```
```rust    
    #[inline]
    fn push_to_cell(&mut self, index: usize, data: usize) {
        // 셀 헤드에 새 노드를 스택처럼 붙임
        let new_item = Box::new(Item {
            data,
            next: self.pool[index].take(),
        });
        self.pool[index] = Some(new_item);
        self.item_count += 1;
    }
```
```rust
    /// 삼각형 삽입: (pt1, pt2, pt3, tol)와 교차하는 모든 셀에 data 추가
    pub fn insert_triangle(
        &mut self,
        pt1: Point3D,
        pt2: Point3D,
        pt3: Point3D,
        tol: f64,
        data: usize,
    ) -> bool {
        let Some((s, e)) = self.index_range_for_triangle(&pt1, &pt2, &pt3, tol) else {
            return false;
        };

        let tri = [
            [pt1.x, pt1.y, pt1.z],
            [pt2.x, pt2.y, pt2.z],
            [pt3.x, pt3.y, pt3.z],
        ];
        let half = [
            self.cell_size * 0.5 + tol,
            self.cell_size * 0.5 + tol,
            self.cell_size * 0.5 + tol,
        ];

        for i in s[0]..=e[0] {
            for j in s[1]..=e[1] {
                for k in s[2]..=e[2] {
                    if let Some(cell) = self.get_cell_index(i, j, k) {
                        let center = self.cell_center(i, j, k);
                        if Self::tri_box_overlap(center, half, tri) {
                            if !self.contains(cell, data) {
                                self.push_to_cell(cell, data);
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
```
```rust
pub struct SpatialPartitionIterator<'a> {
    current: Option<&'a Item>,
}
```
```rust
impl<'a> Iterator for SpatialPartitionIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.current {
            self.current = item.next.as_deref();
            Some(item.data)
        } else {
            None
        }
    }
}
```
```rust
impl SpatialPartition {
    pub fn begin(&self, pt: Point3D) -> SpatialPartitionIterator {
        let xi = self.floor_index(pt.x, self.pt_min.x);
        let yi = self.floor_index(pt.y, self.pt_min.y);
        let zi = self.floor_index(pt.z, self.pt_min.z);

        if let Some(index) = self.get_cell_index(xi, yi, zi) {
            SpatialPartitionIterator {
                current: self.pool[index].as_deref(),
            }
        } else {
            SpatialPartitionIterator { current: None }
        }
    }
}
```
---

# 테스트

SpatialPartition의 주요 함수들에 대한 테스트 코드입니다.  
각 테스트는 함수의 핵심 기능을 검증하며, 중복 방지, 셀 인덱싱, 교차 판정 등 다양한 시나리오를 포함합니다.

## ✅ 1. new 생성자 테스트
```rust
#[test]
fn test_new_partition_initialization() {
    let bbox = BoundingBox::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(10.0, 10.0, 10.0));
    let part = SpatialPartition::new(&bbox, 9);

    assert_eq!(part.cell_counts().len(), 3);
    assert!(part.cell_size() > 0.0);
    assert_eq!(part.item_count(), 0);
    assert_eq!(part.pool_size(), part.cell_counts()[0] * part.cell_counts()[1] * part.cell_counts()[2]);
}
```

## ✅ 2. get_cell_index 테스트
```rust
#[test]
fn test_get_cell_index_valid_and_invalid() {
    let bbox = BoundingBox::unit();
    let part = SpatialPartition::new(&bbox, 5);

    let valid = part.get_cell_index(0, 0, 0);
    assert!(valid.is_some());

    let invalid = part.get_cell_index(999, 0, 0);
    assert!(invalid.is_none());
}
```


## ✅ 3. floor_index 및 clamp_floor_index 테스트
```rust
#[test]
fn test_floor_and_clamp_index() {
    let bbox = BoundingBox::unit();
    let part = SpatialPartition::new(&bbox, 5);

    let raw = part.floor_index(0.5, part.pt_min().x);
    let clamped = part.clamp_floor_index(0.5, part.pt_min().x, 0);

    assert!(raw <= clamped);
    assert!(clamped < part.cell_counts()[0]);
}
```


## ✅ 4. insert_point 중복 방지 테스트
```rust
#[test]
fn test_insert_point_no_duplicates() {
    let bbox = BoundingBox::unit();
    let mut part = SpatialPartition::new(&bbox, 5);

    let pt = Point3D::new(0.5, 0.5, 0.5);
    part.insert_point(pt, 0.01, 123);
    let count1 = part.item_count();

    part.insert_point(pt, 0.01, 123); // 중복 삽입 시도
    let count2 = part.item_count();

    assert_eq!(count1, count2);
}
```


## ✅ 5. insert_line 교차 판정 테스트
```rust
#[test]
fn test_insert_line_and_query() {
    let bbox = BoundingBox::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(10.0, 10.0, 10.0));
    let mut part = SpatialPartition::new(&bbox, 9);

    let a = Point3D::new(1.0, 1.0, 1.0);
    let b = Point3D::new(9.0, 1.0, 1.0);
    part.insert_line(a, b, 0.0, 777);

    let center = Point3D::new(5.0, 1.0, 1.0);
    let ids = part.collect_box_ids(center, 1.0);
    assert!(ids.contains(&777));
}
```


## ✅ 6. insert_triangle 교차 판정 테스트
```rust
#[test]
fn test_insert_triangle_and_query() {
    let bbox = BoundingBox::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(10.0, 10.0, 10.0));
    let mut part = SpatialPartition::new(&bbox, 9);

    let p1 = Point3D::new(3.0, 3.0, 3.0);
    let p2 = Point3D::new(7.0, 3.0, 3.0);
    let p3 = Point3D::new(5.0, 7.0, 3.0);
    part.insert_triangle(p1, p2, p3, 0.0, 888);

    let center = Point3D::new(5.0, 4.0, 3.0);
    let ids = part.collect_box_ids(center, 1.0);
    assert!(ids.contains(&888));
}
```


## ✅ 7. begin 및 begin_box 이터레이터 테스트
```rust
#[test]
fn test_begin_and_begin_box_iterator() {
    let bbox = BoundingBox::unit();
    let mut part = SpatialPartition::new(&bbox, 5);

    let pt = Point3D::new(0.5, 0.5, 0.5);
    part.insert_point(pt, 0.01, 321);

    let ids: Vec<_> = part.begin(pt).collect();
    assert!(ids.contains(&321));

    let ids_box: Vec<_> = part.begin_box(pt, 0.1).collect();
    assert!(ids_box.contains(&321));
}
```


## ✅ 8. collect_box_ids 중복 제거 테스트
```rust
#[test]
fn test_collect_box_ids_deduplicated() {
    let bbox = BoundingBox::unit();
    let mut part = SpatialPartition::new(&bbox, 5);

    let pt = Point3D::new(0.5, 0.5, 0.5);
    for _ in 0..3 {
        part.insert_point(pt, 0.01, 999); // 중복 삽입 시도
    }

    let ids = part.collect_box_ids(pt, 0.1);
    let count = ids.iter().filter(|&&id| id == 999).count();
    assert_eq!(count, 1); // 중복 없이 한 번만 존재
}
```
---
# 실전 테스트

STL 절점(Vertices)을 병합하는 작업은 일반적으로 공간 내에서 동일하거나  
매우 가까운 위치에 있는 절점들을 하나로 통합하는 과정입니다.  
SpatialPartition 구조를 활용하면 이 병합을 효율적으로 수행할 수 있습니다.

## 🧠 절점 병합 개요
- 입력: Vec<Point3D> 형태의 절점 리스트
- 기준: 거리 허용 오차 tol 이내에 있는 절점은 동일한 것으로 간주
- 출력: 병합된 절점 리스트와, 원래 절점 → 병합된 절점 인덱스 매핑

## ✅ 병합 코드 예시
```rust
use std::collections::HashMap;

pub fn merge_vertices(points: &[Point3D], tol: f64) -> (Vec<Point3D>, Vec<usize>) {
    let bbox = BoundingBox::from_points(points);
    let mut grid = SpatialPartition::new(&bbox, 51); // 충분한 분할 수

    let mut unique_points = Vec::new();
    let mut remap = vec![0; points.len()]; // 원래 인덱스 → 병합 인덱스
    let mut point_to_index = HashMap::new();

    for (i, pt) in points.iter().enumerate() {
        let mut found = None;

        for j in grid.begin_box(*pt, tol) {
            let candidate = &unique_points[j];
            if pt.distance(candidate) <= tol {
                found = Some(j);
                break;
            }
        }

        match found {
            Some(j) => {
                remap[i] = j;
            }
            None => {
                let new_index = unique_points.len();
                unique_points.push(*pt);
                remap[i] = new_index;
                grid.insert_point(*pt, tol, new_index);
            }
        }
    }

    (unique_points, remap)
}
```

### 📌 결과 설명

| 항목            | 설명                                                                 |
|-----------------|----------------------------------------------------------------------|
| `unique_points` | 병합된 절점 리스트. 중복 또는 근접한 절점들이 하나로 통합된 결과입니다. |
| `remap`         | 원래 절점 인덱스 → 병합된 절점 인덱스 매핑. 즉, `points[i] → unique_points[remap[i]]` 관계를 나타냅니다. |


## 🧪 테스트 예시
```rust
#[test]
fn test_merge_vertices() {
    let pts = vec![
        Point3D::new(1.0, 2.0, 3.0),
        Point3D::new(1.001, 2.001, 3.001),
        Point3D::new(5.0, 5.0, 5.0),
    ];
    let (merged, remap) = merge_vertices(&pts, 0.01);

    assert_eq!(merged.len(), 2); // 첫 두 개는 병합됨
    assert_eq!(remap[0], remap[1]);
    assert_ne!(remap[0], remap[2]);
}
```

---

# 📘 테스트 수식 설명 및 기능 요약
## ✅ 1. 삼각형 면 유효성 테스트
```rust
fn test_mesh_face_validity()
```
### ✅ MeshFace 관련 함수 설명

| 함수 이름                | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| `MeshFace::is_triangle()` | 이 면(face)이 삼각형인지 확인합니다. 보통 3개의 정점 인덱스를 갖고 있어야 합니다. |
| `MeshFace::is_valid(n)`   | 주어진 정점 개수 `n`에 대해 이 면이 유효한지 검사합니다. 중복된 인덱스가 없어야 하며, <br> 각 인덱스는 `0 <= i < n` 범위여야 합니다. |


### 📐 수식 기반 검증

- is_triangle() 조건:

$$
\mathrm{face.indices.len()}=3
$$

- is_valid(n) 조건:

$$
\mathrm{모든\  }i\in \mathrm{face.indices}\Rightarrow 0\leq i<n\quad \mathrm{and}\quad i\neq j\neq k
$$

### 수식 기반:
- 유효성: $0\leq i,j,k<n 이고 i\neq j\neq k$

### 테스트 코드
```rust
//✅ 1. 삼각형 면 유효성 테스트
#[test]
fn test_mesh_face_validity() {
    let face = MeshFace::new_tri(0, 1, 2);
    assert!(face.is_triangle());
    assert!(face.is_valid(10));

    let invalid_face = MeshFace::new_tri(1, 1, 2);
    assert!(!invalid_face.is_valid(10));
}
```

## ✅ 2. BoundingBox 포함 테스트
```rust
fn test_bbox_includes_point()
```

### ✅ BoundingBox::includes_point(p) 함수 설명

| 함수 이름                      | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `BoundingBox::includes_point(p)` | 주어진 점 `p`가 AABB(BoundingBox) 내부에 포함되는지 확인합니다. <br> 기본적으로 경계 포함 여부는 `false`일 경우 경계 제외, `true`일 경우 경계 포함으로 동작합니다. |


### 📐 수식 기반 검증

$$
점:  p=(x,y,z), \quad 박스 경계: min=(x_{\mathrm{min}},y_{\mathrm{min}},z_{\mathrm{min}}), \quad max=(x_{\mathrm{max}},y_{\mathrm{max}},z_{\mathrm{max}})
$$

- 경계 포함 조건 (inclusive = true):

$$
x_{\mathrm{min}}\leq x\leq x_{\mathrm{max}},\quad y_{\mathrm{min}}\leq y\leq y_{\mathrm{max}},\quad z_{\mathrm{min}}\leq z\leq z_{\mathrm{max}}
$$

- 경계 제외 조건 (inclusive = false):

$$
x_{\mathrm{min}} < x < x_{\mathrm{max}} \quad \text{is equivalent to} \quad x \in (x_{\mathrm{min}}, x_{\mathrm{max}})
$$


### 수식 기반:
- 포함 조건:

$$
min_x\leq p_x\leq max_x,\quad min_y\leq p_y\leq max_y,\quad min_z\leq p_z\leq max_z
$$

### 테스트 코드
```rust
//✅ 2. BoundingBox 포함 테스트
#[test]
fn test_bbox_includes_point() {
    let bbox = BoundingBox::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(5.0, 5.0, 5.0));
    let p_inside = Point3D::new(2.0, 2.0, 2.0);
    let p_outside = Point3D::new(6.0, 2.0, 2.0);

    assert!(bbox.includes_point(&p_inside, false));
    assert!(!bbox.includes_point(&p_outside, false));
}
```

## ✅ 3. SpatialPartition 경계 테스트
```rust
fn test_spatial_partition_bounds()
```

### ✅ SpatialPartition 관련 함수 설명

| 함수 이름                    | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| `SpatialPartition::begin(pt)` | 주어진 좌표 `pt`가 속한 셀의 연결 리스트를 순회하는 이터레이터를 반환합니다. 해당 셀에 삽입된 데이터만 조회됩니다. |
| `item_count()`               | 현재 파티션에 삽입된 전체 데이터 수를 반환합니다. 중복 삽입은 카운트되지 않도록 `contains()`로 방지됩니다. |


### 📐 수식 기반 검증
#### begin(pt) 내부 동작:
- 셀 인덱스 계산:

$$
i=\left\lfloor \frac{pt_x-pt_{min,x}}{cell\\_ size}\right\rfloor ,\quad j=\left\lfloor \frac{pt_y-pt_{min,y}}{cell\\_ size}\right\rfloor ,\quad k=\left\lfloor \frac{pt_z-pt_{min,z}}{cell\\_ size}\right\rfloor
$$

- 셀 풀 인덱스:

$$
\mathrm{index}=(i\cdot cell\\_ counts[1]+j)\cdot cell\\_ counts[2]+k
$$

- 반환: 해당 셀의 연결 리스트를 순회하는 SpatialPartitionIterator

- item_count() 의미:
    - 삽입 시마다 item_count += 1
    - 단, contains(index, data)가 false일 때만 증가
    - 즉, 중복된 데이터는 카운트되지 않음

### 수식 기반:
- 셀 인덱스 계산:

$$
\left\lfloor \frac{coord-pt_{min}}{cell\\_ size}\right\rfloor 
$$

### 테스트 코드
```rust
//✅ 3. SpatialPartition 경계 테스트
#[test]
fn test_spatial_partition_bounds() {
    let bbox = BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0));
    let sp = SpatialPartition::new(&bbox, 5);

    let pt = Point3D::new(0.0, 0.0, 0.0);
    let iter = sp.begin(pt);
    assert!(iter.count() == 0, "초기 상태에서는 데이터 없음");
}
```


## ✅ 4. insert_point 및 중복 방지
```rust
fn insert_point_and_find_with_begin()
fn duplicate_point_is_not_duplicated()
```

### ✅ SpatialPartition 핵심 함수 설명

| 함수 이름                   | 설명                                                                 |
|----------------------------|----------------------------------------------------------------------|
| `insert_point(pt, tol, id)` | 주어진 점 `pt`를 허용 오차 `tol` 범위 내 셀들에 삽입합니다. 중복된 `id`는 삽입되지 않으며, 셀마다 연결 리스트로 저장됩니다. |
| `begin(pt)`                | 점 `pt`가 속한 단일 셀의 연결 리스트를 순회하는 이터레이터를 반환합니다. 해당 셀의 데이터만 조회됩니다. |
| `item_count()`             | 현재 파티션에 삽입된 전체 데이터 수를 반환합니다. 중복 삽입은 `contains()`로 차단되므로 실제 삽입된 고유 데이터 수만 카운트됩니다. |

### 📐 수식 기반 동작
#### 🔹 insert_point(pt, tol, id)
- 셀 인덱스 범위 계산:

$$
s_i=\left\lfloor \frac{pt_i-tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor ,\quad e_i=\left\lfloor \frac{pt_i+tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor 
$$

- 각 셀에 대해:
    - contains(cell, id)가 false일 경우:
    - pool[cell] = Some(Box::new(Item { data: id, next: pool[cell] }))
    - item_count += 1

#### 🔹 begin(pt)
- 단일 셀 인덱스 계산:

$$
i=\left\lfloor \frac{pt_x-pt_{\mathrm{min},x}}{cell\\_ size}\right\rfloor ,\quad j=\left\lfloor \frac{pt_y-pt_{\mathrm{min},y}}{cell\\_ size}\right\rfloor ,\quad k=\left\lfloor \frac{pt_z-pt_{\mathrm{min},z}}{cell\\_ size}\right\rfloor 
$$

- 반환: 해당 셀의 연결 리스트를 순회하는 SpatialPartitionIterator

#### 🔹 item_count()
- 삽입 시마다 item_count += 1
- 단, contains(index, id)가 false일 때만 증가
- 즉, 중복된 데이터는 카운트되지 않음


### 수식 기반:
- 셀 범위:

$$
s=\left\lfloor \frac{pt-tol-pt_{min}}{cell\\_ size}\right\rfloor ,\quad e=\left\lfloor \frac{pt+tol-pt_{min}}{cell\\_ size}\right\rfloor 
$$

### 테스트 코드
```rust
fn mk_pt(x: f64, y: f64, z: f64) -> Point3D {
    Point3D { x, y, z }
}
fn mk_bbox() -> BoundingBox {
    // 필요 시 당신 프로젝트의 API 에 맞게 수정
    // (예: BoundingBox::from_min_max(min, max) 등)
    let min = mk_pt(0.0, 0.0, 0.0);
    let max = mk_pt(10.0, 10.0, 10.0);
    BoundingBox::new(min, max)
}

// begin_box()가 있다면 이걸 사용하고,
// 없다면 아래 fallback 스캐너로 동일 동작 수행.
fn collect_in_box(part: &SpatialPartition, center: Point3D, half: [f64; 3]) -> HashSet<usize> {
    let tol = half[0].max(half[1]).max(half[2]);
    part.collect_box_ids(center, tol).into_iter().collect()
}
```
```rust
#[test]
fn insert_point_and_find_with_begin() {
    let bbox = mk_bbox();
    let mut sp = SpatialPartition::new(&bbox, 9);

    let p = mk_pt(1.2, 1.3, 1.4);
    let id = 42usize;
    sp.insert_point(p, 0.05, id);

    // 같은 셀에서 begin()으로 훑으면 id가 나와야 함
    let mut found = false;
    for got in sp.begin(p) {
        if got == id {
            found = true;
            break;
        }
    }
    assert!(found, "insert_point 한 데이터가 begin()에서 안 나옵니다");
}
```
```rust
#[test]
fn duplicate_point_is_not_duplicated() {
    let bbox = mk_bbox();
    let mut sp = SpatialPartition::new(&bbox, 9);

    let p = mk_pt(2.0, 2.0, 2.0);
    let id = 7usize;

    // 첫 삽입
    let before = sp.item_count();
    sp.insert_point(p, 0.01, id);
    let after1 = sp.item_count();
    assert!(after1 > before, "첫 삽입 후 item_count 증가 필요");

    // 동일 포인트/아이디 재삽입 → contains로 막혀야 함
    sp.insert_point(p, 0.01, id);
    let after2 = sp.item_count();
    assert_eq!(after1, after2, "중복 삽입이 차단되지 않았습니다");
}
```


## ✅ 5. insert_line 교차 판정
```rust
fn insert_line_and_find_in_range()
```

### ✅ 선분 삽입 및 박스 스캔 함수 설명

| 함수 이름                        | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `insert_line(a, b, tol, id)`     | 선분 `a → b`와 교차하는 셀들을 판정하여 `id`를 삽입합니다. `tol`은 셀 경계 확장에 사용됩니다. 중복 삽입은 `contains()`로 방지됩니다. |
| `collect_box_ids(center, tol)`   | 중심점 `center`와 반경 `tol`을 기준으로 박스 범위 내의 셀들을 순회하며, 포함된 모든 `id`를 중복 없이 수집합니다. |


- 아래는 insert_line과 collect_box_ids 함수에 대한 설명과 수식 기반 동작을 정리한 표입니다.
### ✅ 선분 삽입 및 박스 스캔 함수 설명

| 함수 이름                        | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `insert_line(a, b, tol, id)`     | 선분 `a → b`와 교차하는 셀들을 판정하여 `id`를 삽입합니다. `tol`은 셀 경계 확장에 사용됩니다. 중복 삽입은 `contains()`로 방지됩니다. |
| `collect_box_ids(center, tol)`   | 중심점 `center`와 반경 `tol`을 기준으로 박스 범위 내의 셀들을 순회하며, 포함된 모든 `id`를 중복 없이 수집합니다. |



### 📐 수식 기반 동작
#### 🔹 insert_line(a, b, tol, id)
- AABB 인덱스 범위 계산:

$$
s_i=\left\lfloor \frac{\min (a_i,b_i)-tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor ,\quad e_i=\left\lfloor \frac{\max (a_i,b_i)+tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor
$$

- 각 셀에 대해:
- 셀 AABB 계산:

$$
bmin_i=i\cdot cell\\_ size+pt_{\mathrm{min},i}-tol,\quad bmax_i=(i+1)\cdot cell\\_ size+pt_{\mathrm{min},i}+tol
$$

- slab 방식 교차 판정:

$$
t_{\mathrm{entry}}\leq t_{\mathrm{exit}}\Rightarrow 교차- 교차 시 contains(cell, id)가 false이면 삽입
$$

#### 🔹 collect_box_ids(center, tol)
- 박스 인덱스 범위 계산:

$$
s_i=\left\lfloor \frac{center_i-tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor ,\quad e_i=\left\lfloor \frac{center_i+tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor
$$


- 각 셀의 연결 리스트 순회 → id 수집
- HashSet으로 중복 제거 후 정렬된 Vec<usize> 반환


### 수식 기반:
- slab 방식 교차 판정:

$$
t_{entry}=\max _{axis}\left( \frac{bmin-start}{end-start}\right) ,\quad t_{exit}=\min _{axis}\left( \frac{bmax-start}{end-start}\right) 
$$


### 테스트 코드
```rust
#[test]
fn insert_line_and_find_in_range() {
    let bbox = mk_bbox();
    let mut sp = SpatialPartition::new(&bbox, 9);

    let a = mk_pt(0.2, 0.2, 0.2);
    let b = mk_pt(6.8, 0.2, 0.2);
    let id = 111usize;

    assert!(sp.insert_line(a, b, 0.0, id), "insert_line 실패");

    // 라인 AABB 근방 박스로 훑어서 id가 있어야 함
    let center = mk_pt((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, (a.z + b.z) * 0.5);
    let half = [(b.x - a.x).abs() * 0.5 + 0.2, 0.5, 0.5];
    let got = collect_in_box(&sp, center, half);
    assert!(
        got.contains(&id),
        "insert_line 한 id를 박스 범위 조회에서 찾지 못함"
    );
}
```

## ✅ 6. insert_triangle 교차 판정
```rust
fn insert_triangle_and_hit_cells()
```

### ✅ 삼각형 삽입 및 박스 스캔 함수 설명

| 함수 이름                          | 설명                                                                 |
|------------------------------------|----------------------------------------------------------------------|
| `insert_triangle(p1, p2, p3, tol, id)` | 삼각형 `p1–p2–p3`와 교차하는 셀들을 판정하여 `id`를 삽입합니다. `tol`은 셀 경계 확장에 사용됩니다. 중복 삽입은 `contains()`로 방지됩니다. |
| `collect_box_ids(center, tol)`     | 중심점 `center`와 반경 `tol`을 기준으로 박스 범위 내의 셀들을 순회하며, 포함된 모든 `id`를 중복 없이 수집합니다. |


### 📐 수식 기반 동작
#### 🔹 insert_triangle(p1, p2, p3, tol, id)
- AABB 인덱스 범위 계산:

$$
s_i=\left\lfloor \frac{\min (p1_i,p2_i,p3_i)-tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor ,\quad e_i=\left\lfloor \frac{\max (p1_i,p2_i,p3_i)+tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor 
$$


- 각 셀에 대해:
- 셀 중심 계산:

$$
center_i=i\cdot cell\\_ size+pt_{\mathrm{min},i}+\frac{cell\\_ size}{2}
$$

- 셀 반경:

$$
half_i=\frac{cell\\_ size}{2}+tol
$$

- 교차 판정: `tri_box_overlap(center, half, triangle)`

- Separation Axis Theorem(SAT) 기반 13개 축에 대해 교차 여부 판정
- 교차 시 contains(cell, id)가 false이면 삽입


#### 🔹 collect_box_ids(center, tol)
- 박스 인덱스 범위 계산:

$$
s_i=\left\lfloor \frac{center_i-tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor ,\quad e_i=\left\lfloor \frac{center_i+tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor
$$


- 각 셀의 연결 리스트 순회 → id 수집
- HashSet으로 중복 제거 후 정렬된 Vec<usize> 반환

### 수식 기반:
- SAT 방식 교차 판정:
- 삼각형 edge × AABB 축 → 13개 축에 대해 프로젝션
- AABB 반경과 삼각형 프로젝션 범위 비교

### 코드
```rust
#[test]
fn insert_triangle_and_hit_cells() {
    let bbox = mk_bbox();
    let mut sp = SpatialPartition::new(&bbox, 9);

    // XY 평면에 비스듬히 놓인 삼각형
    let p1 = mk_pt(3.0, 3.0, 1.0);
    let p2 = mk_pt(7.0, 3.5, 1.0);
    let p3 = mk_pt(4.0, 7.5, 1.0);
    let id = 999usize;

    assert!(
        sp.insert_triangle(p1, p2, p3, 0.0, id),
        "insert_triangle 실패"
    );

    // 삼각형의 AABB 중심/반경으로 수집
    let cx = (p1.x + p2.x + p3.x) / 3.0;
    let cy = (p1.y + p2.y + p3.y) / 3.0;
    let cz = 1.0;
    let center = mk_pt(cx, cy, cz);
    let minx = p1.x.min(p2.x).min(p3.x);
    let maxx = p1.x.max(p2.x).max(p3.x);
    let miny = p1.y.min(p2.y).min(p3.y);
    let maxy = p1.y.max(p2.y).max(p3.y);
    let half = [(maxx - minx) * 0.5 + 0.5, (maxy - miny) * 0.5 + 0.5, 0.75];

    let got = collect_in_box(&sp, center, half);
    assert!(
        got.contains(&id),
        "insert_triangle 한 id를 박스 범위에서 찾지 못함"
    );
}
```

## ✅ 7. begin 단일 셀 조회
```rust
fn begin_returns_only_current_cell_items()
```
### ✅ SpatialPartition::begin(pt) 함수 설명

| 함수 이름       | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| `begin(pt)`    | 주어진 좌표 `pt`가 속한 단일 셀의 연결 리스트를 순회하는 이터레이터를 반환합니다. 해당 셀에 삽입된 데이터만 조회됩니다. |

#### 📐 수식 기반 동작
- 셀 인덱스 계산:

$$
i=\left\lfloor \frac{pt_x-pt_{\mathrm{min},x}}{cell\\_ size}\right\rfloor ,\quad j=\left\lfloor \frac{pt_y-pt_{\mathrm{min},y}}{cell\\_ size}\right\rfloor ,\quad k=\left\lfloor \frac{pt_z-pt_{\mathrm{min},z}}{cell\\_ size}\right\rfloor
$$

- 셀 풀 인덱스:

$$
\mathrm{index}=(i\cdot cell\\_ counts[1]+j)\cdot cell\\_ counts[2]+k
$$

- 반환값: 해당 셀의 연결 리스트를 순회하는 SpatialPartitionIterator

### 수식 기반:
- 셀 인덱스:

$$
i=\left\lfloor \frac{pt_x-pt_{min,x}}{cell\\_ size}\right\rfloor
$$

### 코드
```rust
#[test]
fn begin_returns_only_current_cell_items() {
    let bbox = mk_bbox();
    let mut sp = SpatialPartition::new(&bbox, 9);

    let id_a = 1usize;
    let id_b = 2usize;

    // 서로 다른 셀에 들어가도록 약간 거리 둔다
    let p_a = mk_pt(1.1, 1.1, 1.1);
    let p_b = mk_pt(4.9, 1.1, 1.1);

    sp.insert_point(p_a, 0.01, id_a);
    sp.insert_point(p_b, 0.01, id_b);

    // p_a 위치 셀에서 begin → id_a만 나와야 함
    let got: HashSet<_> = sp.begin(p_a).collect();
    assert!(got.contains(&id_a));
    assert!(!got.contains(&id_b), "begin()은 단일 셀만 순회해야 합니다");
}
```

## ✅ 8. 전체 셀 이터레이터 스캔
```rust
fn iterator_scans_entire_grid_and_finds_everything()
```

### ✅ begin_box 및 HashSet 설명

| 항목                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| `begin_box(center, tol)` | 중심점 `center`와 반경 `tol`을 기준으로 박스 범위 내 셀들을 순회하는 이터레이터를 반환합니다. <br> 각 셀의 연결 리스트를 순회하며 데이터를 반환합니다. |
| `HashSet`                | `begin_box`로 얻은 데이터 중 중복된 ID를 제거하기 위해 사용됩니다. <br>동일한 데이터가 여러 셀에 중복 삽입된 경우에도 하나만 유지됩니다. |


### 📐 수식 기반 동작
#### 🔹 begin_box(center, tol)
- 셀 인덱스 범위 계산:

$$
s_i=\left\lfloor \frac{center_i-tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor ,\quad e_i=\left\lfloor \frac{center_i+tol-pt_{\mathrm{min},i}}{cell\\_ size}\right\rfloor
$$

- 셀 인덱스 범위 [s_0..e_0],[s_1..e_1],[s_2..e_2]를 순회하며 각 셀의 연결 리스트를 순회
- 반환값: SpatialPartitionBoxIter 이터레이터 → Item.data (usize ID)

#### 🔹 HashSet 사용 목적
- begin_box는 여러 셀을 순회하므로, 동일한 id가 여러 셀에 중복 삽입된 경우 중복된 결과가 나올 수 있음
- 따라서:

$$
\mathrm{HashSet}\leftarrow \mathrm{중복\  제거된\  ID\  집합}
$$

- 최종적으로 HashSet을 통해 유일한 ID만 수집하여 정확한 결과 확보

### 수식 기반:
- 전체 박스 범위:

$$
tol=2\cdot \max (pt_{max}-pt_{min})
$$

### 코드
```rust
// --- 유틸 ---
fn mk_pt(x: f64, y: f64, z: f64) -> Point3D {
    Point3D { x, y, z }
}
fn mk_bbox() -> BoundingBox {
    // 프로젝트 API에 맞게 조정하세요 (예: from_min_max 등)
    let min = mk_pt(0.0, 0.0, 0.0);
    let max = mk_pt(10.0, 10.0, 10.0);
    BoundingBox::new(min, max)
}

/// 이터레이터 `begin_box`로 “그리드 전체”를 스캔해서
/// 중복을 제거한 id 집합을 얻는다.
fn scan_all_with_iterator(part: &SpatialPartition) -> HashSet<usize> {
    // 충분히 큰 tol로 전체 커버 (clamp가 있으므로 크게 잡아도 안전)
    let center = mk_pt(
        (part.pt_min().x + part.pt_max().x) * 0.5,
        (part.pt_min().y + part.pt_max().y) * 0.5,
        (part.pt_min().z + part.pt_max().z) * 0.5,
    );
    let tol_all = (part.pt_max().x - part.pt_min().x)
        .max(part.pt_max().y - part.pt_min().y)
        .max(part.pt_max().z - part.pt_min().z)
        * 2.0; // 여유있게 전체 커버

    // begin_box는 셀 단위로 순회하므로 "같은 data"가 여러 셀에 있으면 중복이 나올 수 있음.
    // HashSet으로 중복 제거.
    let mut set = HashSet::new();
    for id in part.begin_box(center, tol_all) {
        set.insert(id);
    }
    set
}

#[test]
fn iterator_scans_entire_grid_and_finds_everything() {
    let bbox = mk_bbox();
    let mut sp = SpatialPartition::new(&bbox, 9);

    // 서로 다른 유형으로 몇 개 삽입
    let id_point = 1usize;
    let id_line = 2usize;
    let id_tri = 3usize;
    let id_point_far = 4usize;

    // point
    sp.insert_point(mk_pt(1.2, 1.3, 1.4), 0.05, id_point);

    // line
    let a = mk_pt(0.2, 0.2, 0.2);
    let b = mk_pt(7.8, 0.2, 0.2);
    assert!(sp.insert_line(a, b, 0.0, id_line));

    // triangle
    let p1 = mk_pt(3.0, 3.0, 1.0);
    let p2 = mk_pt(7.0, 3.5, 1.0);
    let p3 = mk_pt(4.0, 7.5, 1.0);
    assert!(sp.insert_triangle(p1, p2, p3, 0.0, id_tri));

    // far point (다른 영역)
    sp.insert_point(mk_pt(9.1, 9.2, 9.3), 0.05, id_point_far);

    // --- 이터레이터로 "전체 스캔" ---
    let found = scan_all_with_iterator(&sp);

    // 우리가 넣은 모든 id가 나와야 한다 (중복은 HashSet으로 제거)
    for expect in [id_point, id_line, id_tri, id_point_far] {
        assert!(
            found.contains(&expect),
            "iterator 전체 스캔에서 id {expect}를 찾지 못함"
        );
    }
}
```


## 📌 테스트 기능 요약표
| 테스트 이름                                      | 검증 기능 설명                                  | 수식 기반 핵심 |
|--------------------------------------------------|--------------------------------------------------|----------------|
| test_mesh_face_validity                          | 삼각형 면 유효성 확인                            | 인덱스 범위 및 중복 없음 |
| test_bbox_includes_point                         | AABB 포함 여부 확인                              | min ≤ p ≤ max |
| test_spatial_partition_bounds                    | 초기 상태에서 데이터 없음 확인                   | 셀 인덱스 계산 |
| insert_point_and_find_with_begin                 | 점 삽입 후 셀 조회 가능                          | 셀 범위 계산 |
| duplicate_point_is_not_duplicated                | 중복 삽입 방지 확인                              | contains 검사 |
| insert_line_and_find_in_range                    | 선분 삽입 및 박스 범위 조회                      | slab 교차 판정 |
| insert_triangle_and_hit_cells                    | 삼각형 삽입 및 박스 범위 조회                    | SAT 교차 판정 |
| begin_returns_only_current_cell_items            | begin()은 단일 셀만 순회해야 함                  | 셀 인덱스 보정 |
| iterator_scans_entire_grid_and_finds_everything | 전체 셀 순회로 모든 ID 조회                      | 전체 박스 커버 |

---

