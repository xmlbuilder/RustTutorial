# Radar / Marching Cube

특히 3D 레이더 데이터, 시뮬레이션 기반 공간 해석, Doppler volume 처리 같은 고급 응용에서 등장합니다.

## 🧠 Radar에서 Marching Cubes가 쓰일 수 있는 경우
### 1️⃣ 3D Radar Volume → Isosurface 추출
  - 일부 고해상도 레이더는 **3D voxel 형태의 반사 강도(volume reflectivity)** 를 제공합니다.
  - 이 데이터를 기반으로 특정 임계값에서 표면 추출 → Marching Cubes로 mesh 생성
  - 예: 기상 레이더에서 구름, 강우, 우박의 경계 추출
### 2️⃣ Doppler Velocity Field 시각화
  - Doppler 레이더는 속도 필드를 생성 → 이를 3D로 표현할 때 Marching Cubes 사용
  - 예: 항공기 주변 공기 흐름, 회전하는 물체의 속도 경계 추출
### 3️⃣ Radar Simulation → AI 학습용 Mesh 생성
  - 시뮬레이션에서 생성된 레이더 반사체를 voxel로 표현 → Marching Cubes로 mesh 생성
  - AI가 레이더 반사체의 형태, 경계, 움직임을 학습할 수 있도록 전처리
### 4️⃣ SAR 시계열 → 3D Stack → Surface 추출
  - SAR 이미지를 시간 축으로 쌓아 3D volume 생성 → 변화 경계 추출에 Marching Cubes 사용
  - 예: 지형 변화, 구조물 붕괴 감지
### 5️⃣ AI 기반 Radar Occupancy Field
  - AI가 레이더 데이터를 기반으로 occupancy probability field를 예측
  - Marching Cubes로 존재 확률이 높은 영역의 경계 추출

## ✅ 결론
  Marching Cubes는 Radar의 2D 이미지를 직접 처리하진 않지만,
  3D volume으로 확장되거나 AI가 생성한 필드를 mesh로 표현할 때 매우 유용하게 쓰입니다.
  특히 기상, 항공, 자율주행, 시뮬레이션 기반 AI 학습에서 그 역할이 점점 커지고 있어요

---

## 🛰️ 위성 데이터를 이용한 3D 지형 생성 과정
- 데이터 획득
  - 위성 영상(예: Sentinel-2, Landsat) → 광학영상
  - 레이더 위성(SAR) → 지형 반사 특성
  - 위성 고도계(Altimetry) → 해수면 높이 변화로 해저 지형 추정
  - LiDAR 항공 데이터 → 고해상도 DEM(디지털 고도 모델)
- DEM/DSM 생성
  - GeoTIFF 같은 포맷으로 저장된 고도값을 불러옴
  - DEM(지형 표면), DSM(건물·식생 포함 표면)으로 변환
- Voxel Volume 변환
  - DEM을 voxel grid로 변환 → 3D 필드 생성
  - 이 필드를 AI 학습용 입력으로 사용 가능
- Marching Cubes 적용
  - voxel 필드에서 isosurface 추출
  - 삼각형 mesh로 변환 → 3D 지형 모델 완성
- AI 학습 활용
  - 자율주행, 항공 시뮬레이션, 군사 훈련 등에서 지형 인식 학습
  - 예: 미사일 궤적 시뮬레이션, 드론 항로 최적화, 레이더 반사체 모델링

---
## 소스
```rust
// marching.rs
// Port of ON_MarchingSquares / ON_MarchingCubes to Rust
// Uses existing project structs: TMatrix, Point2D, Point3D, Mesh

use crate::core::geom::Point2D;
use crate::core::mesh::{Mesh, on_tri_list_to_mesh};
use crate::core::prelude::Point3D;
use crate::core::tmatrix::TMatrix;
use crate::core::types::SQRT_EPSILON;
use std::f64;
use std::sync::Arc;

pub type ScalarField2D = Arc<dyn Fn(f64, f64) -> f64 + Send + Sync>;
pub type ScalarField3D = Arc<dyn Fn(f64, f64, f64) -> f64 + Send + Sync>;

// ========================= Marching Squares =========================

pub struct MarchingSquares {
    cells: TMatrix<f64>,
    grid_origin: Point2D,
    cells_in_x: usize,
    cells_in_y: usize,
    cell_size_x: f64,
    cell_size_y: f64,
    iso_level: f64,
    byte_data: Option<TMatrix<u8>>,
    func: Option<ScalarField2D>,
}
```
```rust
impl MarchingSquares {
    pub fn new(
        cells_x: usize,
        cell_sz_x: f64,
        cells_y: usize,
        cell_sz_y: f64,
        func: Option<ScalarField2D>,
    ) -> Self {
        Self::with_origin(
            Point2D { x: 0.0, y: 0.0 },
            cells_x,
            cell_sz_x,
            cells_y,
            cell_sz_y,
            func,
        )
    }
```
```rust
    pub fn from_byte_matrix(original: TMatrix<u8>, cell_sz_x: usize, cell_sz_y: usize) -> Self {
        let rows = original.rows();
        let cols = original.cols();
        let cells_x = (rows as f64 / cell_sz_x as f64).floor() as usize;
        let cells_y = (cols as f64 / cell_sz_y as f64).floor() as usize;
        let mut ms = Self::with_origin(
            Point2D { x: 0.0, y: 0.0 },
            cells_x,
            cell_sz_x as f64,
            cells_y,
            cell_sz_y as f64,
            None,
        );
        ms.byte_data = Some(original);
        // build func from byte_data
        let ptr = std::sync::Mutex::new(());
        let bd_ref = ms.byte_data.as_ref().unwrap().clone();
        ms.func = Some(Arc::new(move |gx: f64, gy: f64| {
            // Note: grid aligned sample (nearest neighbor)
            let _lock = ptr.lock().ok();
            let fx = (gx - 0.0) / (cell_sz_x as f64);
            let fy = (gy - 0.0) / (cell_sz_y as f64);
            let mut ix = (fx + 0.5).floor() as isize;
            let mut iy = (fy + 0.5).floor() as isize;
            let rmax = bd_ref.rows() as isize - 1;
            let cmax = bd_ref.cols() as isize - 1;
            ix = ix.clamp(0, rmax);
            iy = iy.clamp(0, cmax);
            *bd_ref.get(ix as usize, iy as usize).unwrap_or(&0) as f64
        }));
        ms.fill_grid();
        ms
    }
```
```rust
    pub fn with_origin(
        origin: Point2D,
        cells_x: usize,
        cell_sz_x: f64,
        cells_y: usize,
        cell_sz_y: f64,
        func: Option<ScalarField2D>,
    ) -> Self {
        let mut s = Self {
            cells: TMatrix::with_size(cells_x, cells_y, 0.0f64),
            grid_origin: origin,
            cells_in_x: cells_x,
            cells_in_y: cells_y,
            cell_size_x: cell_sz_x,
            cell_size_y: cell_sz_y,
            iso_level: 0.0,
            byte_data: None,
            func,
        };
        if s.func.is_some() {
            s.fill_grid();
        }
        s
    }
```
```rust
    pub fn iso_level(&self) -> f64 {
        self.iso_level
    }
```
```rust
    pub fn set_iso_level(&mut self, v: f64) {
        self.iso_level = v;
    }
```
```rust
    fn fill_grid(&mut self) {
        if let Some(f) = &self.func {
            for iy in 0..self.cells_in_y {
                for ix in 0..self.cells_in_x {
                    let gx = self.grid_origin.x + ix as f64 * self.cell_size_x;
                    let gy = self.grid_origin.y + iy as f64 * self.cell_size_y;
                    if let Some(cell) = self.cells.get_mut(ix, iy) {
                        *cell = f(gx, gy);
                    }
                }
            }
        }
    }
```
```rust
    fn compute_coords(&self, col: usize, row: usize) -> [[f64; 2]; 4] {
        let x0 = self.grid_origin.x + col as f64 * self.cell_size_x;
        let x1 = self.grid_origin.x + (col as f64 + 1.0) * self.cell_size_x;
        let y0 = self.grid_origin.y + row as f64 * self.cell_size_y;
        let y1 = self.grid_origin.y + (row as f64 + 1.0) * self.cell_size_y;
        [[x0, y1], [x1, y1], [x1, y0], [x0, y0]]
    }
```
```rust
    pub fn run(&self) -> Vec<[Point2D; 2]> {
        let mut segments: Vec<[Point2D; 2]> = Vec::new();
        let mut coords: [[f64; 2]; 4];
        let mut data = [0.0f64; 4];
        for row in 0..(self.cells_in_y.saturating_sub(1)) {
            for col in 0..(self.cells_in_x.saturating_sub(1)) {
                coords = self.compute_coords(col, row);
                data[0] = *self.cells.get(col, row + 1).unwrap_or(&0.0);
                data[1] = *self.cells.get(col + 1, row + 1).unwrap_or(&0.0);
                data[2] = *self.cells.get(col + 1, row).unwrap_or(&0.0);
                data[3] = *self.cells.get(col, row).unwrap_or(&0.0);
                Self::calc_cell(&coords, &data, self.iso_level, &mut segments);
            }
        }
        segments
    }
```
```rust
    fn cut_edge(iso: f64, c1: (f64, f64), v1: f64, c2: (f64, f64), v2: f64) -> (f64, f64) {
        const EPS: f64 = SQRT_EPSILON;
        if (iso - v1).abs() < EPS {
            return c1;
        }
        if (iso - v2).abs() < EPS {
            return c2;
        }
        if (v1 - v2).abs() < EPS {
            return c1;
        }
        let t = (iso - v1) / (v2 - v1);
        (c1.0 + t * (c2.0 - c1.0), c1.1 + t * (c2.1 - c1.1))
    }
```
```rust
    fn calc_cell(coords: &[[f64; 2]; 4], data: &[f64; 4], iso: f64, out: &mut Vec<[Point2D; 2]>) {
        let mut idx = 0usize;
        for i in 0..4 {
            if data[i] < iso {
                idx |= 1 << i;
            }
        }
        let mask = QUAD_TABLE_INDEX[idx];
        if mask == 0 {
            return;
        }
        let mut cut = [[0.0f64; 2]; 4];
        if (mask & 1) != 0 {
            // 0-1
            let p = Self::cut_edge(
                iso,
                (coords[0][0], coords[0][1]),
                data[0],
                (coords[1][0], coords[1][1]),
                data[1],
            );
            cut[0] = [p.0, p.1];
        }
        if (mask & 2) != 0 {
            // 1-2
            let p = Self::cut_edge(
                iso,
                (coords[1][0], coords[1][1]),
                data[1],
                (coords[2][0], coords[2][1]),
                data[2],
            );
            cut[1] = [p.0, p.1];
        }
        if (mask & 4) != 0 {
            // 2-3
            let p = Self::cut_edge(
                iso,
                (coords[2][0], coords[2][1]),
                data[2],
                (coords[3][0], coords[3][1]),
                data[3],
            );
            cut[2] = [p.0, p.1];
        }
        if (mask & 8) != 0 {
            // 3-0
            let p = Self::cut_edge(
                iso,
                (coords[3][0], coords[3][1]),
                data[3],
                (coords[0][0], coords[0][1]),
                data[0],
            );
            cut[3] = [p.0, p.1];
        }
        let table = &QUAD_TABLES[idx];
        let mut i = 0;
        while i + 1 < table.len() && table[i] != -1 && table[i + 1] != -1 {
            let a = table[i] as usize;
            let b = table[i + 1] as usize;
            out.push([
                Point2D {
                    x: cut[a][0],
                    y: cut[a][1],
                },
                Point2D {
                    x: cut[b][0],
                    y: cut[b][1],
                },
            ]);
            i += 2;
        }
    }
}
```
```rust
// ========================= Marching Cubes =========================
pub struct MarchingCubes {
    grid: Vec<f64>,
    grid_origin: Point3D,
    cells_in_x: usize,
    cells_in_y: usize,
    cells_in_z: usize,
    cell_size_x: f64,
    cell_size_y: f64,
    cell_size_z: f64,
    iso_level: f64,
    scalar_func: Option<ScalarField3D>,
}
```
```rust
impl MarchingCubes {
    pub fn new(
        cells_x: usize,
        cell_sz_x: f64,
        cells_y: usize,
        cell_sz_y: f64,
        cells_z: usize,
        cell_sz_z: f64,
        func: Option<ScalarField3D>,
    ) -> Self {
        Self::with_origin(
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            cells_x,
            cell_sz_x,
            cells_y,
            cell_sz_y,
            cells_z,
            cell_sz_z,
            func,
        )
    }
```
```rust
    pub fn with_origin(
        origin: Point3D,
        cells_x: usize,
        cell_sz_x: f64,
        cells_y: usize,
        cell_sz_y: f64,
        cells_z: usize,
        cell_sz_z: f64,
        func: Option<ScalarField3D>,
    ) -> Self {
        let n = cells_x * cells_y * cells_z;
        let mut s = Self {
            grid: vec![0.0; n],
            grid_origin: origin,
            cells_in_x: cells_x,
            cells_in_y: cells_y,
            cells_in_z: cells_z,
            cell_size_x: cell_sz_x,
            cell_size_y: cell_sz_y,
            cell_size_z: cell_sz_z,
            iso_level: 0.0,
            scalar_func: func,
        };
        if s.scalar_func.is_some() {
            s.fill_grid();
        }
        s
    }
```
```rust
    #[inline]
    fn idx(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.cells_in_y + y) * self.cells_in_x + x
    }
```
```rust
    pub fn iso_level(&self) -> f64 {
        self.iso_level
    }
```
```rust
    pub fn set_iso_level(&mut self, v: f64) {
        self.iso_level = v;
    }
```
```rust
    pub fn fill_grid(&mut self) {
        if let Some(f) = &self.scalar_func {
            for z in 0..self.cells_in_z {
                for y in 0..self.cells_in_y {
                    for x in 0..self.cells_in_x {
                        let gx = self.grid_origin.x + x as f64 * self.cell_size_x;
                        let gy = self.grid_origin.y + y as f64 * self.cell_size_y;
                        let gz = self.grid_origin.z + z as f64 * self.cell_size_z;
                        let v = f(gx, gy, gz);
                        let id = self.idx(x, y, z);
                        self.grid[id] = v;
                    }
                }
            }
        }
    }
```
```rust
    fn compute_coords(&self, col: usize, row: usize, z: usize) -> [[f64; 3]; 8] {
        let x0 = self.grid_origin.x + col as f64 * self.cell_size_x;
        let x1 = self.grid_origin.x + (col as f64 + 1.0) * self.cell_size_x;
        let y0 = self.grid_origin.y + row as f64 * self.cell_size_y;
        let y1 = self.grid_origin.y + (row as f64 + 1.0) * self.cell_size_y;
        let z0 = self.grid_origin.z + z as f64 * self.cell_size_z;
        let z1 = self.grid_origin.z + (z as f64 + 1.0) * self.cell_size_z;
        [
            [x0, y1, z0],
            [x1, y1, z0],
            [x1, y0, z0],
            [x0, y0, z0],
            [x0, y1, z1],
            [x1, y1, z1],
            [x1, y0, z1],
            [x0, y0, z1],
        ]
    }
```
```rust
    fn cut_edge3(
        iso: f64,
        c1: (f64, f64, f64),
        v1: f64,
        c2: (f64, f64, f64),
        v2: f64,
    ) -> (f64, f64, f64) {
        const EPS: f64 = 1e-3;
        if (iso - v1).abs() < EPS {
            return c1;
        }
        if (iso - v2).abs() < EPS {
            return c2;
        }
        if (v1 - v2).abs() < EPS {
            return c1;
        }
        let t = (iso - v1) / (v2 - v1);
        (
            c1.0 + t * (c2.0 - c1.0),
            c1.1 + t * (c2.1 - c1.1),
            c1.2 + t * (c2.2 - c1.2),
        )
    }
```
```rust
    fn calc_cell(
        coord: &[[f64; 3]; 8],
        data: &[f64; 8],
        iso: f64,
        out_tris: &mut Vec<[Point3D; 3]>,
    ) {
        let mut idx = 0usize;
        for i in 0..8 {
            if data[i] < iso {
                idx |= 1 << i;
            }
        }
        let mask = CUBE_TABLE_INDEX[idx];
        if mask == 0 {
            return;
        }
        let mut cut = [[0.0f64; 3]; 12];
        // 12 edges in mc order: (0-1,1-2,2-3,3-0, 4-5,5-6,6-7,7-4, 0-4,1-5,2-6,3-7)
        if (mask & 1) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[0][0], coord[0][1], coord[0][2]),
                data[0],
                (coord[1][0], coord[1][1], coord[1][2]),
                data[1],
            );
            cut[0] = [p.0, p.1, p.2];
        }
        if (mask & 2) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[1][0], coord[1][1], coord[1][2]),
                data[1],
                (coord[2][0], coord[2][1], coord[2][2]),
                data[2],
            );
            cut[1] = [p.0, p.1, p.2];
        }
        if (mask & 4) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[2][0], coord[2][1], coord[2][2]),
                data[2],
                (coord[3][0], coord[3][1], coord[3][2]),
                data[3],
            );
            cut[2] = [p.0, p.1, p.2];
        }
        if (mask & 8) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[3][0], coord[3][1], coord[3][2]),
                data[3],
                (coord[0][0], coord[0][1], coord[0][2]),
                data[0],
            );
            cut[3] = [p.0, p.1, p.2];
        }
        if (mask & 16) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[4][0], coord[4][1], coord[4][2]),
                data[4],
                (coord[5][0], coord[5][1], coord[5][2]),
                data[5],
            );
            cut[4] = [p.0, p.1, p.2];
        }
        if (mask & 32) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[5][0], coord[5][1], coord[5][2]),
                data[5],
                (coord[6][0], coord[6][1], coord[6][2]),
                data[6],
            );
            cut[5] = [p.0, p.1, p.2];
        }
        if (mask & 64) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[6][0], coord[6][1], coord[6][2]),
                data[6],
                (coord[7][0], coord[7][1], coord[7][2]),
                data[7],
            );
            cut[6] = [p.0, p.1, p.2];
        }
        if (mask & 128) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[7][0], coord[7][1], coord[7][2]),
                data[7],
                (coord[4][0], coord[4][1], coord[4][2]),
                data[4],
            );
            cut[7] = [p.0, p.1, p.2];
        }
        if (mask & 256) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[0][0], coord[0][1], coord[0][2]),
                data[0],
                (coord[4][0], coord[4][1], coord[4][2]),
                data[4],
            );
            cut[8] = [p.0, p.1, p.2];
        }
        if (mask & 512) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[1][0], coord[1][1], coord[1][2]),
                data[1],
                (coord[5][0], coord[5][1], coord[5][2]),
                data[5],
            );
            cut[9] = [p.0, p.1, p.2];
        }
        if (mask & 1024) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[2][0], coord[2][1], coord[2][2]),
                data[2],
                (coord[6][0], coord[6][1], coord[6][2]),
                data[6],
            );
            cut[10] = [p.0, p.1, p.2];
        }
        if (mask & 2048) != 0 {
            let p = Self::cut_edge3(
                iso,
                (coord[3][0], coord[3][1], coord[3][2]),
                data[3],
                (coord[7][0], coord[7][1], coord[7][2]),
                data[7],
            );
            cut[11] = [p.0, p.1, p.2];
        }

        let table = &CUBE_TABLES[idx];
        let mut i = 0;
        while i + 2 < table.len() && table[i] != -1 {
            let a = table[i] as usize;
            let b = table[i + 2] as usize;
            let c = table[i + 1] as usize;
            out_tris.push([
                Point3D {
                    x: cut[a][0],
                    y: cut[a][1],
                    z: cut[a][2],
                },
                Point3D {
                    x: cut[b][0],
                    y: cut[b][1],
                    z: cut[b][2],
                },
                Point3D {
                    x: cut[c][0],
                    y: cut[c][1],
                    z: cut[c][2],
                },
            ]);
            i += 3;
        }
    }
```
```rust
    pub fn run(&self) -> Mesh {
        let mut tris: Vec<[Point3D; 3]> = Vec::new();
        let mut coord: [[f64; 3]; 8];
        let mut data = [0.0f64; 8];
        for z in 0..(self.cells_in_z.saturating_sub(1)) {
            for row in 0..(self.cells_in_y.saturating_sub(1)) {
                for col in 0..(self.cells_in_x.saturating_sub(1)) {
                    coord = self.compute_coords(col, row, z);
                    // gather scalar values
                    data[0] = self.grid[self.idx(col, row + 1, z)];
                    data[1] = self.grid[self.idx(col + 1, row + 1, z)];
                    data[2] = self.grid[self.idx(col + 1, row, z)];
                    data[3] = self.grid[self.idx(col, row, z)];
                    data[4] = self.grid[self.idx(col, row + 1, z + 1)];
                    data[5] = self.grid[self.idx(col + 1, row + 1, z + 1)];
                    data[6] = self.grid[self.idx(col + 1, row, z + 1)];
                    data[7] = self.grid[self.idx(col, row, z + 1)];
                    Self::calc_cell(&coord, &data, self.iso_level, &mut tris);
                }
            }
        }
        // build mesh using helper
        let mut vertices: Vec<Point3D> = Vec::with_capacity(tris.len() * 3);
        let mut indices: Vec<[u32; 3]> = Vec::with_capacity(tris.len());
        for (i, t) in tris.iter().enumerate() {
            let base = (i * 3) as u32;
            vertices.push(t[0]);
            vertices.push(t[1]);
            vertices.push(t[2]);
            indices.push([base, base + 1, base + 2]);
        }
        on_tri_list_to_mesh(vertices, indices)
    }
}
```
```rust
// ========================= Lookup Tables =========================

const QUAD_TABLE_INDEX: [i32; 16] = [0, 9, 3, 10, 6, 15, 5, 12, 12, 5, 15, 6, 10, 3, 9, 0];

const QUAD_TABLES: [[i32; 5]; 16] = [
    [-1, -1, -1, -1, -1],
    [3, 0, -1, -1, -1],
    [0, 1, -1, -1, -1],
    [1, 3, -1, -1, -1],
    [1, 2, -1, -1, -1],
    [0, 1, 2, 3, -1],
    [0, 2, -1, -1, -1],
    [2, 3, -1, -1, -1],
    [2, 3, -1, -1, -1],
    [0, 2, -1, -1, -1],
    [1, 2, 3, 0, -1],
    [1, 2, -1, -1, -1],
    [1, 3, -1, -1, -1],
    [0, 1, -1, -1, -1],
    [3, 0, -1, -1, -1],
    [-1, -1, -1, -1, -1],
];
```
```rust
// 256 index and 256x16 triangle table (shortened for brevity would be incorrect;
// include full tables below exactly as in C++)
const CUBE_TABLE_INDEX: [i32; 256] = [
    0, 265, 515, 778, 1030, 1295, 1541, 1804, 2060, 2309, 2575, 2822, 3082, 3331, 3593, 3840, 400,
    153, 915, 666, 1430, 1183, 1941, 1692, 2460, 2197, 2975, 2710, 3482, 3219, 3993, 3728, 560,
    825, 51, 314, 1590, 1855, 1077, 1340, 2620, 2869, 2111, 2358, 3642, 3891, 3129, 3376, 928, 681,
    419, 170, 1958, 1711, 1445, 1196, 2988, 2725, 2479, 2214, 4010, 3747, 3497, 3232, 1120, 1385,
    1635, 1898, 102, 367, 613, 876, 3180, 3429, 3695, 3942, 2154, 2403, 2665, 2912, 1520, 1273,
    2035, 1786, 502, 255, 1013, 764, 3580, 3317, 4095, 3830, 2554, 2291, 3065, 2800, 1616, 1881,
    1107, 1370, 598, 863, 85, 348, 3676, 3925, 3167, 3414, 2650, 2899, 2137, 2384, 1984, 1737,
    1475, 1226, 966, 719, 453, 204, 4044, 3781, 3535, 3270, 3018, 2755, 2505, 2240, 2240, 2505,
    2755, 3018, 3270, 3535, 3781, 4044, 204, 453, 719, 966, 1226, 1475, 1737, 1984, 2384, 2137,
    2899, 2650, 3414, 3167, 3925, 3676, 348, 85, 863, 598, 1370, 1107, 1881, 1616, 2800, 3065,
    2291, 2554, 3830, 4095, 3317, 3580, 764, 1013, 255, 502, 1786, 2035, 1273, 1520, 2912, 2665,
    2403, 2154, 3942, 3695, 3429, 3180, 876, 613, 367, 102, 1898, 1635, 1385, 1120, 3232, 3497,
    3747, 4010, 2214, 2479, 2725, 2988, 1196, 1445, 1711, 1958, 170, 419, 681, 928, 3376, 3129,
    3891, 3642, 2358, 2111, 2869, 2620, 1340, 1077, 1855, 1590, 314, 51, 825, 560, 3728, 3993,
    3219, 3482, 2710, 2975, 2197, 2460, 1692, 1941, 1183, 1430, 666, 915, 153, 400, 3840, 3593,
    3331, 3082, 2822, 2575, 2309, 2060, 1804, 1541, 1295, 1030, 778, 515, 265, 0,
];

const CUBE_TABLES: [[i32; 16]; 256] = include!("mc_tri_table.inc");
```


