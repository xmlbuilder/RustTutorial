# 📘 Marching Squares & Cubes: 수식 기반 원리 정리
## 🟩 1. Marching Squares (2D 등고선 추출)
### 🔧 목적
2차원 스칼라 필드에서 등고선(iso-contour)을 추출하여 선분으로 표현
### 📐 셀 좌표 계산
- 셀의 네 꼭짓점 좌표:

$$
\begin{aligned}P_0&=(x_0,y_1)\\ 
P_1&=(x_1,y_1)\\ 
P_2&=(x_1,y_0)\\ 
P_3&=(x_0,y_0)\end{aligned}
$$

- 여기서:

$$
x_0=x_{\mathrm{origin}}+i\cdot \Delta x,\quad x_1=x_0+\Delta x\\ 
y_0=y_{\mathrm{origin}}+j\cdot \Delta y,\quad y_1=y_0+\Delta y
$$

### ✂️ 엣지 절단 (등고선과의 교차점 계산)
-두 점 $c_1$,$c_2$ 사이에서 등고선 값 $\mathrm{iso}$ 와 교차하는 위치:

$$
t=\frac{\mathrm{iso}-v_1}{v_2-v_1}
$$

$$
\mathrm{cut\  point}=c_1+t\cdot (c_2-c_1)
$$

### 🧠 셀 분류 인덱스
각 꼭짓점의 스칼라 값 v_i가 iso보다 작으면 1, 크면 0으로 비트 마스크 생성:  

$$
\mathrm{index}=\sum _{i=0}^3\left[ v_i<\mathrm{iso}\right] \cdot 2^i
$$

- 이 인덱스를 통해 `QUAD_TABLES[index]` 에서 선분 연결 정보를 가져옴

## 🧊 2. Marching Cubes (3D 등치면 추출)
### 🔧 목적
3차원 스칼라 필드에서 등치면(iso-surface)을 추출하여 삼각형으로 표현
### 📐 셀 좌표 계산
큐브의 8개 꼭짓점:


### ✂️ 엣지 절단 (3D 교차점 계산)

$$
t=\frac{\mathrm{iso}-v_1}{v_2-v_1}
$$

$$
\mathrm{cut\  point}=c_1+t\cdot (c_2-c_1)
$$

- 각 큐브에는 12개의 엣지가 있으며, 교차점은 $cut[0..11]$ 에 저장됨


### 🧠 셀 분류 인덱스

$$
\mathrm{index}=\sum _{i=0}^7\left[ v_i<\mathrm{iso}\right] \cdot 2^i
$$

- 이 인덱스를 통해 `CUBE_TABLES[index]` 에서 삼각형 연결 정보를 가져옴

### 🔺 삼각형 생성
- 각 셀에서 최대 5개의 삼각형이 생성될 수 있으며, 각 삼각형은 3개의 교차점으로 구성:

$$
\mathrm{Triangle}=\left( \mathrm{cut}[a],\mathrm{cut}[b],\mathrm{cut}[c]\right) 
$$

## 📌 핵심 수식 요약
| 설명                         | 수식 |
|------------------------------|------|
| 등치선/등치면 보간 계수      | $t = \frac{\mathrm{iso} - v_1}{v_2 - v_1}$ |
| 교차점 좌표 보간             | $c = c_1 + t \cdot (c_2 - c_1)$ |
| 셀 인덱스 계산 (비트 마스크) | $\mathrm{index} = \sum_i [v_i < \mathrm{iso}] \cdot 2^i$ |
| 격자 좌표 계산               | $x = x_0 + i \cdot \Delta x,\quad y = y_0 + j \cdot \Delta y,\quad z = z_0 + k \cdot \Delta z$ 

## 🧊 Marching Squares

### 🔧 주요 함수 목록
| 함수 이름         | 역할 설명 |
|------------------|-----------|
| `new`            | 기본 생성자. 원점 (0,0 또는 0,0,0) 기준으로 격자와 필드 초기화 |
| `with_origin`    | 사용자 지정 원점 기준으로 격자와 필드 초기화 |
| `from_byte_matrix` | 바이트 행렬로부터 스칼라 필드 생성. 픽셀 기반 필드로 변환 가능 |
| `fill_grid`      | 스칼라 필드 함수로부터 각 셀의 값을 계산하여 격자에 채움 |
| `compute_coords` | 셀의 꼭짓점 좌표 계산 (2D 또는 3D) |
| `run`            | 전체 셀을 순회하며 등고선(2D) 또는 등치면(3D) 추출 |
| `cut_edge`       | 두 꼭짓점 사이에서 iso 값과의 교차점 보간 계산 |
| `calc_cell`      | 셀 하나에 대해 교차점 계산 및 선분(2D) 또는 삼각형(3D) 생성 |



## 🧊 Marching Cubes
### 🔧 주요 함수 목록
| 함수 이름       | 역할 설명 |
|----------------|-----------|
| `new`          | 기본 생성자. 원점 (0,0,0) 기준으로 3D 격자와 스칼라 필드 초기화 |
| `with_origin`  | 사용자 지정 원점 기준으로 격자와 필드 초기화 |
| `fill_grid`    | 스칼라 필드 함수로부터 각 셀의 값을 3D 격자에 채움 |
| `compute_coords` | 셀의 8개 꼭짓점 좌표 계산 (큐브 형태) |
| `cut_edge3`    | 두 꼭짓점 사이에서 iso 값과의 교차점 보간 계산 (3D) |
| `calc_cell`    | 셀 하나에 대해 교차점 계산 및 삼각형 생성 |
| `run`          | 전체 셀을 순회하며 등치면 삼각형 추출 후 Mesh로 변환 |



## 📌 전체 알고리즘 흐름 요약
- Marching Squares
    - 2D 스칼라 필드 정의
    - 셀마다 4개 꼭짓점의 값을 기준으로 인덱스 생성
    - 교차점 보간 → 선분 생성
    - 선분 리스트 반환
- Marching Cubes
    - 3D 스칼라 필드 정의
    - 셀마다 8개 꼭짓점의 값을 기준으로 인덱스 생성
    - 교차점 보간 → 삼각형 생성
    - 삼각형 리스트 → Mesh로 변환


```rust
use crate::core::tmatrix::TMatrix;
use crate::math::math_extra::ON_SQRT_EPSILON;
use crate::math::point2d::Point2D;
use crate::math::point3d::Point3D;
use crate::mesh::mesh::{Mesh, tri_list_to_mesh};
use std::f64;
use std::sync::Arc;

pub type ScalarField2D = Arc<dyn Fn(f64, f64) -> f64 + Send + Sync>;
pub type ScalarField3D = Arc<dyn Fn(f64, f64, f64) -> f64 + Send + Sync>;
```
```rust
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
        const EPS: f64 = ON_SQRT_EPSILON;
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
        tri_list_to_mesh(vertices, indices)
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
```
```rust

const CUBE_TABLES: [[i32; 16]; 256] = include!("mc_tri_table.inc");
```
---

## 🟩 Marching Squares 사용 예제
```rust
use nurbslib::core::marching::MarchingSquares;
use nurbslib::math::point2d::Point2D;
use std::sync::Arc;

fn main() {
    // 2D 스칼라 필드 정의: 원형 함수
    let field = Arc::new(|x: f64, y: f64| {
        let cx = 5.0;
        let cy = 5.0;
        let r = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        10.0 - r // 중심에서 멀어질수록 값 감소
    });

    // MarchingSquares 객체 생성
    let mut ms = MarchingSquares::new(20, 0.5, 20, 0.5, Some(field));
    ms.set_iso_level(5.0); // 등고선 기준값 설정

    // 등고선 선분 추출
    let segments = ms.run();
    println!("Extracted {} contour segments", segments.len());
}
```

## 🧊 Marching Cubes 사용 예제
```rust
use nurbslib::core::marching::MarchingCubes;
use nurbslib::math::point3d::Point3D;
use std::sync::Arc;

fn main() {
    // 3D 스칼라 필드 정의: 구 형태
    let field = Arc::new(|x: f64, y: f64, z: f64| {
        let cx = 5.0;
        let cy = 5.0;
        let cz = 5.0;
        let r = ((x - cx).powi(2) + (y - cy).powi(2) + (z - cz).powi(2)).sqrt();
        10.0 - r
    });

    // MarchingCubes 객체 생성
    let mut mc = MarchingCubes::new(20, 0.5, 20, 0.5, 20, 0.5, Some(field));
    mc.set_iso_level(5.0); // 등치면 기준값 설정

    // 등치면 삼각형 추출
    let mesh = mc.run();
    println!("Generated mesh with {} triangles", mesh.triangles.len());
}
```

## ✅ 핵심 흐름 요약
- 스칼라 필드 정의: Arc<dyn Fn> 형태로 2D 또는 3D 함수 정의
- 객체 생성: MarchingSquares 또는 MarchingCubes 생성
- iso_level 설정: 등고선/등치면 기준값 지정
- run() 호출: 선분 또는 삼각형 추출

---

## 📘 Marching Squares & Cubes 테스트 문서
### 🟩 Marching Squares 테스트
#### 1️⃣ ms_segments_from_scalar_field
- 목적: 스칼라 필드 $f(x,y)=x+y$ 에서 iso=2.5 등고선 추출
- 격자: 4×4, 셀 크기 1.0
- 수식 원리:

$$
t=\frac{\mathrm{iso}-v_1}{v_2-v_1},\quad c=c_1+t\cdot (c_2-c_1), \quad \mathrm{index}=\sum _{i=0}^3[v_i<\mathrm{iso}]\cdot 2^i
$$
- 기대 결과:
    - segs.len() > 0 → 등고선이 존재
    - segs.len() <= 12 → 과도한 분할 없음
    - 모든 점이 격자 내에 있음: 0.0\leq x,y\leq 4.0

#### 2️⃣ ms_from_byte_matrix
- 목적: 바이트 행렬 기반 필드에서 수직 등고선 추출
- 입력: 6×6 행렬, 왼쪽 절반 0, 오른쪽 절반 255
    - iso level: 127.5 → 경계에서 등고선 생성
- 기대 결과:
    - segs.len() > 0 → 경계에서 등고선 존재
    - 모든 점이 격자 내에 있음: 0.0\leq x,y\leq 6.0

### 🧊 Marching Cubes 테스트
#### 3️⃣ mc_empty_when_iso_out_of_range
- 목적: 스칼라 필드 없음, iso=1.0 → 모든 값 0 → 교차 없음
- 원리:

$$
\mathrm{index}=\sum _{i=0}^7[v_i<\mathrm{iso}]\cdot 2^i=255 \quad \mathrm{CUBE\\_TABLE\\_INDEX}[255]=0\Rightarrow \mathrm{삼각형\  없음}
$$

- 기대 결과: mesh.triangle_count() == 0

#### 4️⃣ mc_filled_grid_but_iso_empty
- 목적: 스칼라 필드 있음 $f(x,y,z)=x+y+z$, iso 매우 작음 → 교차 없음
- 기대 결과: mesh.triangle_count() == 0

#### 5️⃣ mc_generates_triangles_when_table_present
- 목적: 구 형태 스칼라 필드에서 iso-surface 추출
- 스칼라 필드:

$$
f(x,y,z)=4.0-\left[ (x-2)^2+(y-2)^2+(z-2)^2\right] 
$$

- → 중심 (2,2,2) 반지름 2인 구
- iso level: 0.0 → 구 표면
- 기대 결과: mesh.triangle_count() > 0

### 📌 핵심 수식 요약
| 설명                         | 수식 |
|------------------------------|------|
| 등치선/등치면 보간 계수      | $t = \frac{\mathrm{iso} - v_1}{v_2 - v_1}$ |
| 교차점 좌표 보간             | $c = c_1 + t \cdot (c_2 - c_1)$ |
| 셀 인덱스 계산 (비트 마스크) | $\mathrm{index} = \sum_i [v_i < \mathrm{iso}] \cdot 2^i$ |
| 격자 좌표 계산               | $x = x_0 + i \cdot \Delta x,\quad y = y_0 + j \cdot \Delta y,\quad z = z_0 + k \cdot \Delta z$ |

```rust
#[cfg(test)]
mod marching_test {
    use std::sync::Arc;
    use nurbslib::core::marching::{MarchingCubes, MarchingSquares, ScalarField2D, ScalarField3D};
    use nurbslib::core::tmatrix::TMatrix;
```
```rust
    #[test]
    fn ms_segments_from_scalar_field() {
        // f(x,y) = x + y over 4x4 grid, iso = 2.5 will produce several short segments
        let f2: ScalarField2D = Arc::new(|x, y| x + y);
        let mut ms = MarchingSquares::new(4, 1.0, 4, 1.0, Some(f2));
        ms.set_iso_level(2.5);
        let segs = ms.run();
        // We don't assert exact geometry here (depends on table winding),
        // but we do expect at least one segment and not an excessive number.
        assert!(!segs.is_empty(), "expected some contour segments");
        assert!(
            segs.len() <= 12,
            "unexpectedly many segments: {}",
            segs.len()
        );
        // sanity-check segment points are within grid bounds
        for s in &segs {
            for p in s {
                assert!(p.x >= 0.0 && p.x <= 4.0);
                assert!(p.y >= 0.0 && p.y <= 4.0);
            }
        }
    }
```
```rust
    #[test]
    fn ms_from_byte_matrix() {
        // Build a 6x6 byte field with a simple step: left half 0, right half 255
        let rows = 6usize;
        let cols = 6usize;
        let mut m = TMatrix::<u8>::with_size(rows, cols, 0u8);
        for r in 0..rows {
            for c in 0..cols {
                let val = if c >= cols / 2 { 255 } else { 0 };
                m.set(r, c, val);
            }
        }
        // cell size 1x1; iso between 0 and 255 -> vertical contour near c=3
        let ms = MarchingSquares::from_byte_matrix(m, 1, 1);
        // default iso = 0.0; set to mid
        let mut ms = ms;
        ms.set_iso_level(127.5);
        let segs = ms.run();
        assert!(
            !segs.is_empty(),
            "expected vertical contour from byte field"
        );
        // Expect segments x around 3.0..4.0 (center between 2 and 3 or 3 and 4 depending on rounding)
        // Just check they're inside the grid bounds.
        for s in &segs {
            for p in s {
                assert!(p.x >= 0.0 && p.x <= 6.0);
                assert!(p.y >= 0.0 && p.y <= 6.0);
            }
        }
    }
```
```rust
    #[test]
    fn mc_empty_when_iso_out_of_range() {
        // No scalar func => grid defaults to 0. iso=1.0 => all vertices below iso => idx=0xFF
        // CUBE_TABLE_INDEX[255] == 0 -> early exit, no triangles, so we avoid reading the large tri table.
        let mut mc = MarchingCubes::new(4, 1.0, 4, 1.0, 4, 1.0, None);
        mc.set_iso_level(1.0);
        let mesh = mc.run();
        assert_eq!(
            mesh.triangle_count(),
            0,
            "expected empty mesh when iso > all values"
        );
    }
```
```rust
    #[test]
    fn mc_filled_grid_but_iso_empty() {
        // Provide a scalar field but set iso very negative to ensure mask=0 and still no triangles.
        let f3: ScalarField3D = Arc::new(|x, y, z| x + y + z); // positive in our domain
        let mut mc = MarchingCubes::new(5, 1.0, 5, 1.0, 5, 1.0, Some(f3));
        mc.set_iso_level(-1e6);
        let mesh = mc.run();
        assert_eq!(mesh.triangle_count(), 0);
    }
```
```rust
    // Optional: small smoke test that should create some triangles IF the big table is included.
    // This test is ignored by default to avoid requiring the 256x16 table during early bring-up.
    #[test]
    fn mc_generates_triangles_when_table_present() {
        let f3: ScalarField3D = Arc::new(|x, y, z| {
            let cx = 2.0;
            let cy = 2.0;
            let cz = 2.0;
            let r2 = (x - cx) * (x - cx) + (y - cy) * (y - cy) + (z - cz) * (z - cz);
            4.0 - r2 // sphere radius 2
        });
        let mut mc = MarchingCubes::new(8, 1.0, 8, 1.0, 8, 1.0, Some(f3));
        mc.set_iso_level(0.0);
        let mesh = mc.run();
        assert!(
            mesh.triangle_count() > 0,
            "expected some triangles from sphere iso-surface"
        );
    }
}
```
---

