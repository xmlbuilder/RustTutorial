# SpatialPartition 구조 

## 🧠 전체 구조 의도 요약
| 구성 요소               | 설계 의도 및 역할                                                  |
|------------------------|---------------------------------------------------------------------|
| ITEM 구조체            | 셀에 저장되는 데이터 노드. 링크드 리스트로 여러 개의 데이터를 연결 |
| m_aPool                | 3차원 셀 배열. 각 셀은 ITEM 리스트를 가짐                          |
| m_nCellNum / m_dCellSize | 공간을 균일한 그리드로 나누기 위한 셀 개수, 셀 크기 설정         |
| InsertTriangle/Line/Point | 공간 객체를 셀에 삽입. AABB 기반 충돌 검사로 셀에 데이터 할당  |
| FindBox / FindNearData / FindAxisData | 특정 영역 또는 축 기준으로 셀 내 데이터 검색       |
| ProcessCheck / Range   | 셀 내 데이터 간의 관계를 사용자 정의 방식으로 처리 (중복 제거 포함) |
| ON_FixedSizeMemMgr     | ITEM 객체를 빠르게 생성/소멸하기 위한 메모리 풀                    |
| iterator               | 특정 셀의 데이터 순회용 반복자                                     |
| triBoxOverlap / lineBoxOverlap | 삼각형/선과 셀의 AABB 간 충돌 여부 판단 (SAT 기반)         |
| FloorIndex / GetCellIndex | 좌표를 셀 인덱스로 변환하는 핵심 유틸리티 함수                   |


## 🎯 설계 목적
- 고속 공간 분할 기반 검색: 3D 공간을 균일한 그리드로 나누고, 각 셀에 데이터 삽입 → 빠른 근접 검색 가능
- 메모리 효율성: ITEM을 메모리 풀로 관리하여 할당/해제 비용 최소화
- 확장성: 삼각형, 선, 점 등 다양한 형태의 공간 객체를 처리 가능
- 사용자 정의 처리: ICompPair, ICompSingle 인터페이스로 커스텀 비교/처리 로직 삽입 가능



습니다 JungHwan님! 이제 STL 병합의 핵심인 3단계: SpatialPartition 구조를 Rust로 변환하는 작업을 시작할게요. 

## 🧱 1. 단계: SpatialPartition 구조
이 구조는 정점 병합 시 근접 정점을 빠르게 찾기 위한 공간 분할 인덱스입니다.

### ✅ 1. Item 구조
```rust
#[derive(Debug)]
struct Item {
    data: usize,
    next: Option<Box<Item>>,
}
```


### ✅ 2. SpatialPartition 구조
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

###  ✅ 3. 초기화 함수
```rust
impl SpatialPartition {
    pub fn new(bbox: &BoundingBox, division: usize) -> Self {
        let mut div = if division % 2 == 0 { division + 1 } else { division };
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

### ✅ 4. 셀 인덱스 계산
```rust
impl SpatialPartition {
    fn get_cell_index(&self, xi: usize, yi: usize, zi: usize) -> Option<usize> {
        if xi >= self.cell_counts[0] || yi >= self.cell_counts[1] || zi >= self.cell_counts[2] {
            return None;
        }
        Some((xi * self.cell_counts[1] + yi) * self.cell_counts[2] + zi)
    }

    fn floor_index(&self, coord: f64, min: f64) -> usize {
        ((coord - min) / self.cell_size).floor() as usize
    }
}
```

이제 SpatialPartition의 기본 구조와 초기화, 셀 인덱스 계산까지 Rust로 완성됐습니다.  
다음 단계에서는 insert_point(), begin(), find_data_in_cell() 등 병합에 필요한 메서드를 구현.


## 🧱 2단계: SpatialPartition 메서드 구현
### ✅ 1. insert_point() — 정점 삽입
```rust
impl SpatialPartition {
    pub fn insert_point(&mut self, pt: Point3D, tol: f64, data: usize) {
        let s = [
            Self::floor_index(pt.x - tol, self.pt_min.x),
            Self::floor_index(pt.y - tol, self.pt_min.y),
            Self::floor_index(pt.z - tol, self.pt_min.z),
        ];
        let e = [
            Self::floor_index(pt.x + tol, self.pt_min.x),
            Self::floor_index(pt.y + tol, self.pt_min.y),
            Self::floor_index(pt.z + tol, self.pt_min.z),
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
}
```


### ✅ 2. begin(pt) — 반복자 시작점
```rust
impl SpatialPartition {
    pub fn begin(&self, pt: Point3D) -> SpatialPartitionIterator {
        let xi = Self::floor_index(pt.x, self.pt_min.x);
        let yi = Self::floor_index(pt.y, self.pt_min.y);
        let zi = Self::floor_index(pt.z, self.pt_min.z);

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



### ✅ 3. 반복자 정의
```rust
pub struct SpatialPartitionIterator<'a> {
    current: Option<&'a Item>,
}

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

### ✅ 병합 준비 완료
이제 SpatialPartition은 다음을 지원합니다:
- 정점 삽입 시 tolerance 기반 셀 범위 계산
- 중복 데이터 방지
- 반복자를 통한 근접 셀 탐색



##  정점 병합 및 MeshFace 생성을 Rust로 구현
- 기존 정점과 거리 비교
- tolerance 내에 있으면 병합
- 아니면 새 정점 추가
- MeshFace 생성 및 저장

## 🧱 3단계: 병합 로직 구현
### ✅ 병합 함수 시그니처
```rust
pub fn merge_facets(facets: &RawFacetSet, mesh: &mut MergedMesh) {
```


## ✅ 병합 로직
```rust
pub fn merge_facets(facets: &RawFacetSet, mesh: &mut MergedMesh) {
    if facets.is_empty() {
        return;
    }

    let bbox = facets.bounding_box();
    let tol = bbox.diagonal().length() * std::f64::EPSILON.sqrt();
    let mut sp = SpatialPartition::new(&bbox, 100);

    for facet in &facets.facets {
        let mut face = MeshFace { vi: [-1; 4] };

        for j in 0..3 {
            let pt = facet.pos[j];
            let mut min_dist2 = tol;
            let mut min_idx: Option<i32> = None;

            for idx in sp.begin(pt) {
                let vtx = mesh.vertices[idx];
                let dist2 = pt.distance_square(&vtx);
                if dist2 <= min_dist2 {
                    min_dist2 = dist2;
                    min_idx = Some(idx as i32);
                }
            }

            match min_idx {
                Some(idx) => {
                    face.vi[j] = idx;
                }
                None => {
                    let id = mesh.push_vertex(pt);
                    sp.insert_point(pt, tol, id as usize);
                    face.vi[j] = id;
                }
            }
        }

        if face.is_valid(mesh.vertices.len()) {
            face.vi[3] = face.vi[2]; // 삼각형 처리
            mesh.push_face(face);
        } else {
            mesh.error_faces += 1;
        }
    }
}
```


### ✅ 결과
- mesh.vertices: 병합된 정점 목록
- mesh.faces: 유효한 MeshFace 목록
- mesh.error_faces: 중복 또는 퇴화된 face 수



## 🧱 4단계: 병합 결과를 Mesh에 적용

### ✅ 함수 시그니처
```rust
pub fn apply_merged_mesh(mesh: &mut Mesh, merged: MergedMesh) {
```


### ✅ 적용 로직
```rust
pub fn apply_merged_mesh(mesh: &mut Mesh, merged: MergedMesh) {
    let offset = mesh.vertices.len() as i32;

    // 정점 추가
    for pt in &merged.vertices {
        mesh.vertices.push(pt.to_f32()); // Point3D → [f32; 3]
    }

    // 면 추가 (offset 적용)
    for mut face in merged.faces {
        for i in 0..4 {
            face.vi[i] += offset;
        }
        mesh.faces.push(face);
    }

    // 법선 계산 (선택적으로 구현)
    // 예: mesh.compute_face_normals(); mesh.compute_vertex_normals();
}
```

to_f32()는 Point3D를 [f32; 3]로 변환하는 헬퍼입니다. 필요 시 구현해드릴게요.


### ✅ 전체 병합 흐름 요약
```rust
let raw_facets = read_stl_binary("model.stl")?;
let mut merged = MergedMesh::new();
merge_facets(&raw_facets, &mut merged);
apply_merged_mesh(&mut mesh, merged);
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use geometry::core::spatial_partition::SpatialPartition;
    use geometry::math::boundingbox::BoundingBox;
    use geometry::math::prelude::Point3D;
    use geometry::mesh::mesh::MeshFace;

    //✅ 1. 삼각형 면 유효성 테스트
    #[test]
    fn test_mesh_face_validity() {
        let face = MeshFace::new_tri(0, 1, 2);
        assert!(face.is_triangle());
        assert!(face.is_valid(10));

        let invalid_face = MeshFace::new_tri(1, 1, 2);
        assert!(!invalid_face.is_valid(10));
    }

    //✅ 2. BoundingBox 포함 테스트
    #[test]
    fn test_bbox_includes_point() {
        let bbox = BoundingBox::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(5.0, 5.0, 5.0),
        );
        let p_inside = Point3D::new(2.0, 2.0, 2.0);
        let p_outside = Point3D::new(6.0, 2.0, 2.0);

        assert!(bbox.includes_point(&p_inside, false));
        assert!(!bbox.includes_point(&p_outside, false));
    }

    //✅ 3. SpatialPartition 경계 테스트
    #[test]
    fn test_spatial_partition_bounds() {
        let bbox = BoundingBox::new(
            Point3D::new(-1.0, -1.0, -1.0),
            Point3D::new(1.0, 1.0, 1.0),
        );
        let sp = SpatialPartition::new(&bbox, 5);

        let pt = Point3D::new(0.0, 0.0, 0.0);
        let iter = sp.begin(pt);
        assert!(iter.count() == 0, "초기 상태에서는 데이터 없음");
    }
}
```

---

## 🧱 1. 접촉면 추출: 인접 Face 간 공유 정점 기반
```rust
use std::collections::HashMap;

pub fn extract_contact_faces(mesh: &Mesh) -> Vec<(usize, usize)> {
    let mut vertex_to_faces: HashMap<i32, Vec<usize>> = HashMap::new();

    for (face_idx, face) in mesh.faces.iter().enumerate() {
        for vi in 0..face.len() {
            vertex_to_faces.entry(vi as i32).or_default().push(face_idx);
        }
    }

    let mut contact_pairs = Vec::new();
    let mut seen = HashMap::new();

    for faces in vertex_to_faces.values() {
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                let a = faces[i];
                let b = faces[j];
                let key = if a < b { (a, b) } else { (b, a) };
                if seen.insert(key, true).is_none() {
                    contact_pairs.push(key);
                }
            }
        }
    }
    contact_pairs
}
```

이 함수는 공유 정점을 기준으로 인접한 face 쌍을 추출합니다.  
접촉면 후보로 활용 가능하며, 이후 평면성 판단과 함께 필터링할 수 있음.

## 🧠 2. 평면성 판단: MeshFace::is_planar(...) 활용
```rust
pub fn filter_planar_faces(mesh: &Mesh, planar_tol: f64, angle_tol_rad: f64) -> Vec<usize> {
    let mut planar_faces = Vec::new();

    for (i, face) in mesh.faces.iter().enumerate() {

        let mesh_face = if *face[2] == *face[3] {
            MeshFace::new_tri(*face[0], *face[1], *face[2])
        } else {
            MeshFace::new_quad(*face[0], *face[1], *face[2], *face[3])
        };

        let (is_planar, _) = MeshFace::is_planar(&mesh_face, planar_tol, angle_tol_rad, &mesh.vertices);
        if is_planar {
            planar_faces.push(i);
        }
    }
    planar_faces
}
```

###  🎯 예시: match로 삼각형 vs 사각형 처리
```rust
let mesh_face = match *face {
    [a, b, c, d] if c == d => MeshFace::new_tri(a, b, c, d),
    [a, b, c, d] => MeshFace::new_quad(a, b, c, d),
    _ => panic!("Invalid face format"),
};
```

### 🔍 설명
- match는 패턴 매칭이기 때문에 face가 [u32; 4]일 때만 작동
- if c == d 조건으로 삼각형 판단
- panic!()은 예외 처리용 (필요시 Option이나 Result로 바꿀 수 있음)

## ✨ 확장 가능성
- 나중에 face.len()이 가변적일 경우 match face.len()으로 분기 가능
- 혹은 FaceType::Tri, FaceType::Quad 같은 enum을 도입하면 더 명확하게 관리 가능
```rust
match face_type {
    FaceType::Tri => MeshFace::new_tri(...),
    FaceType::Quad => MeshFace::new_quad(...),
}
```

이 함수는 tolerance 기준으로 평면성을 만족하는 face 인덱스를 반환합니다.  
접촉면 추출과 조합하면 “평면 접촉면” 필터링도 가능해요.


## 🧹 3. 병합 후 최적화: 정점 정렬 및 중복 제거
```rust
pub fn optimize_mesh(mesh: &mut Mesh) {
        use std::collections::HashMap;
        let mut unique_map: HashMap<[NotNan<f32>; 3], i32> = HashMap::new();
        let mut new_vertices = Vec::new();
        let mut remap = Vec::new();

        for (i, v) in mesh.vertices.iter().enumerate() {
            let key: [NotNan<f32>; 3] = [
                NotNan::new(v.x as f32).unwrap(),
                NotNan::new(v.y as f32).unwrap(),
                NotNan::new(v.z as f32).unwrap(),
            ];

            let id = *unique_map.entry(key).or_insert_with(|| {
                let new_id = new_vertices.len() as i32;
                new_vertices.push(Point3D::new(
                    key[0].into_inner() as f64,
                    key[1].into_inner() as f64,
                    key[2].into_inner() as f64,
                ));
              new_id
            });
            remap.push(id);
        }

        for faces in &mut mesh.faces {
            for vi in &mut *faces {
                *vi = remap[*vi as usize] as u32;
            }
        }

        mesh.vertices = new_vertices;
    }
```

이 함수는 동일한 정점을 병합하고 face 인덱스를 재정렬합니다.  
STL 병합 후 중복 정점이 남아있을 경우 유용합니다.


## 🔄 전체 흐름 예시
```python
let contact_pairs = extract_contact_faces(&mesh);
let planar_faces = filter_planar_faces(&mesh, 1e-6, 0.01);
optimize_mesh(&mut mesh);
```



## Spatial Partition 전체 코드
```rust
use crate::math::boundingbox::BoundingBox;
use crate::math::prelude::Point3D;

#[derive(Debug)]
struct Item {
    data: usize,
    next: Option<Box<Item>>,
}

pub struct SpatialPartition {
    cell_counts: [usize; 3],
    cell_size: f64,
    pt_min: Point3D,
    pt_max: Point3D,
    pool: Vec<Option<Box<Item>>>,
    item_count: usize,
}
impl SpatialPartition {
    pub fn new(bbox: &BoundingBox, division: usize) -> Self {
        let mut div = if division % 2 == 0 { division + 1 } else { division };
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

impl SpatialPartition {
    fn get_cell_index(&self, xi: usize, yi: usize, zi: usize) -> Option<usize> {
        if xi >= self.cell_counts[0] || yi >= self.cell_counts[1] || zi >= self.cell_counts[2] {
            return None;
        }
        Some((xi * self.cell_counts[1] + yi) * self.cell_counts[2] + zi)
    }

    fn floor_index(&self, coord: f64, min: f64) -> usize {
        ((coord - min) / self.cell_size).floor() as usize
    }
}

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
}

pub struct SpatialPartitionIterator<'a> {
    current: Option<&'a Item>,
}

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
