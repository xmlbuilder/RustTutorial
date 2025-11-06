# Hermite Surface
Hermite Surface는 네 개의 꼭짓점 위치, 각 꼭짓점에서의 접선 벡터,  
그리고 꼬임 벡터(twist vector)를 기반으로 정의되는 곡면으로,  
곡선의 Hermite 보간을 2차원으로 확장한 형태입니다.  
주로 곡면 설계나 CAD에서 경계 조건을 명확히 제어할 수 있는 장점 때문에 사용됩니다.

## 📐 Hermite Surface란?
Hermite Surface는 두 개의 매개변수 u와 v를 기반으로 정의되는 이차원 보간 곡면입니다.  
이 곡면은 다음과 같은 16개의 기하 정보로 구성됩니다:
- 4개의 꼭짓점 위치:
$P_{00}$, $P_{01}$, $P_{10}$, $P_{11}$

- 8개의 접선 벡터 (각 꼭짓점에서 u, v 방향):
$P_{00}^u$, $P_{00}^v$, $P_{01}^u$, $P_{01}^v$, $P_{10}^u$, $P_{10}^v$, $P_{11}^u$, $P_{11}^v$,

- 4개의 꼬임 벡터 (혼합 방향 uv):
$P_{00}^{uv}$, $P_{01}^{uv}$, $P_{10}^{uv}$, $P_{11}^{uv}$

이러한 정보를 통해 곡면의 형태와 경계 조건을 직접 제어할 수 있습니다.

## 📊 수식 구조
Hermite Surface는 다음과 같은 형태의 이중 Hermite 보간식으로 표현됩니다:  
$S(u, v) = \sum_{i=0}^{3} \sum_{j=0}^{3} h_i(u) \cdot h_j(v) \cdot C_{ij}$  
### 여기서:
- $h_i(u)$, $h_j(v)$: Hermite basis functions
- $C_{ij}$: 16개의 기하 정보 (위치, 접선, 꼬임 벡터)
- Hermite basis function은 다음과 같이 정의됩니다:
```
h₀(t) =  2t³ − 3t² + 1  
h₁(t) = −2t³ + 3t²  
h₂(t) =  t³ − 2t² + t  
h₃(t) =  t³ − t²
```
따라서 전체 곡면은 다음과 같이 행렬 형태로 표현할 수 있습니다:  
$S(u, v) = [u^3 \quad u^2 \quad u \quad 1] \cdot M_u \cdot G \cdot M_v^T \cdot [v^3 \quad v^2 \quad v \quad 1]^T$
### 여기서:
- `$M_u$`, `$M_v$` : Hermite basis 행렬
- `$G$` : 4×4 기하 정보 행렬 (16개의 벡터)

## 🧠 특징 요약
- 정의 요소: 4 위치 + 8 접선 + 4 꼬임 = 16 벡터
- 연속성: 각 패치 내부는 $C^2$, 패치 간 연결은 $C^1$ 가능
- 제어력: 경계 조건을 직접 지정 가능
- 용도: CAD, 곡면 모델링, 물리 기반 시뮬레이션

  ---

## 소스 코드
```rust
use crate::math::point3d::Point3D;
use crate::math::prelude::Vector3D;

fn strictly_increasing(xs: &[f64]) -> bool {
    if xs.len() < 2 { return false; }
    for w in xs.windows(2) {
        if !w[0].is_finite() || !w[1].is_finite() { return false; }
        if !(w[0] < w[1]) { return false; }
    }
    true
}
```
```rust
#[derive(Clone, Debug)]
pub struct HermiteSurface {
    u_count: usize,
    v_count: usize,
    u_parameters: Vec<f64>,
    v_parameters: Vec<f64>,
    grid_points: Vec<Vec<Point3D>>,   // [u][v]
    u_tangents: Vec<Vec<Vector3D>>,   // [u][v]  (∂S/∂u at grid)
    v_tangents: Vec<Vec<Vector3D>>,   // [u][v]  (∂S/∂v at grid)
    twists:     Vec<Vec<Vector3D>>,   // [u][v]  (∂²S/∂u∂v at grid)
}
```
```rust
impl Default for HermiteSurface {
    fn default() -> Self {
        Self {
            u_count: 0,
            v_count: 0,
            u_parameters: Vec::new(),
            v_parameters: Vec::new(),
            grid_points: Vec::new(),
            u_tangents: Vec::new(),
            v_tangents: Vec::new(),
            twists:     Vec::new(),
        }
    }
}
```
```rust
impl HermiteSurface {
    pub fn new() -> Self { Self::default() }

    /// Constructs an empty grid (values must be filled later).
    pub fn with_counts(u_count: usize, v_count: usize) -> Self {
        let mut s = Self::default();
        s.create(u_count, v_count);
        s
    }

    /// (Re)allocates all arrays to the requested size and clears existing data.
    /// You must later set parameters, points, tangents, and twists.
    pub fn create(&mut self, u_count: usize, v_count: usize) -> bool {
        if u_count < 2 || v_count < 2 {
            *self = Self::default();
            return false;
        }

        self.u_count = u_count;
        self.v_count = v_count;

        self.u_parameters = vec![f64::NAN; u_count];
        self.v_parameters = vec![f64::NAN; v_count];

        self.grid_points = vec![vec![Point3D::new(f64::NAN, f64::NAN, f64::NAN); v_count]; u_count];
        self.u_tangents  = vec![vec![Vector3D::new(f64::NAN, f64::NAN, f64::NAN); v_count]; u_count];
        self.v_tangents  = vec![vec![Vector3D::new(f64::NAN, f64::NAN, f64::NAN); v_count]; u_count];
        self.twists      = vec![vec![Vector3D::new(f64::NAN, f64::NAN, f64::NAN); v_count]; u_count];

        true
    }

    #[inline]
    pub fn u_count(&self) -> usize { self.u_count }
    #[inline]
    pub fn v_count(&self) -> usize { self.v_count }

    #[inline]
    fn in_bounds(&self, u: usize, v: usize) -> bool {
        u < self.u_count && v < self.v_count
    }

    pub fn u_parameter_at(&self, u: usize) -> Option<f64> {
        self.u_parameters.get(u).copied()
    }
    pub fn set_u_parameter_at(&mut self, u: usize, param: f64) {
        if let Some(p) = self.u_parameters.get_mut(u) { *p = param; }
    }

    pub fn v_parameter_at(&self, v: usize) -> Option<f64> {
        self.v_parameters.get(v).copied()
    }
    pub fn set_v_parameter_at(&mut self, v: usize, param: f64) {
        if let Some(p) = self.v_parameters.get_mut(v) { *p = param; }
    }

    pub fn u_parameters(&self) -> &[f64] { &self.u_parameters }
    pub fn v_parameters(&self) -> &[f64] { &self.v_parameters }

    pub fn point_at(&self, u: usize, v: usize) -> Option<Point3D> {
        self.in_bounds(u, v).then(|| self.grid_points[u][v])
    }
    pub fn set_point_at(&mut self, u: usize, v: usize, p: Point3D) {
        if self.in_bounds(u, v) { self.grid_points[u][v] = p; }
    }

    pub fn u_tangent_at(&self, u: usize, v: usize) -> Option<Vector3D> {
        self.in_bounds(u, v).then(|| self.u_tangents[u][v])
    }
    pub fn set_u_tangent_at(&mut self, u: usize, v: usize, t: Vector3D) {
        if self.in_bounds(u, v) { self.u_tangents[u][v] = t; }
    }

    pub fn v_tangent_at(&self, u: usize, v: usize) -> Option<Vector3D> {
        self.in_bounds(u, v).then(|| self.v_tangents[u][v])
    }
    pub fn set_v_tangent_at(&mut self, u: usize, v: usize, t: Vector3D) {
        if self.in_bounds(u, v) { self.v_tangents[u][v] = t; }
    }

    pub fn twist_at(&self, u: usize, v: usize) -> Option<Vector3D> {
        self.in_bounds(u, v).then(|| self.twists[u][v])
    }
    pub fn set_twist_at(&mut self, u: usize, v: usize, t: Vector3D) {
        if self.in_bounds(u, v) { self.twists[u][v] = t; }
    }

    pub fn grid_points(&self) -> &Vec<Vec<Point3D>> { &self.grid_points }
    pub fn u_tangents(&self) -> &Vec<Vec<Vector3D>> { &self.u_tangents }
    pub fn v_tangents(&self) -> &Vec<Vec<Vector3D>> { &self.v_tangents }
    pub fn twists(&self)     -> &Vec<Vec<Vector3D>> { &self.twists }


    /// Light structural validation + finite checks + strictly-increasing params.
    /// If you want deeper Hermite compatibility checks, plug your own validator.
    pub fn is_valid(&self) -> bool {
        if self.u_count < 2 || self.v_count < 2 { return false; }
        if self.u_parameters.len() != self.u_count { return false; }
        if self.v_parameters.len() != self.v_count { return false; }
        if self.grid_points.len() != self.u_count { return false; }
        if self.u_tangents.len()  != self.u_count { return false; }
        if self.v_tangents.len()  != self.u_count { return false; }
        if self.twists.len()      != self.u_count { return false; }

        if !strictly_increasing(&self.u_parameters) { return false; }
        if !strictly_increasing(&self.v_parameters) { return false; }

        for u in 0..self.u_count {
            if self.grid_points[u].len() != self.v_count { return false; }
            if self.u_tangents[u].len()  != self.v_count { return false; }
            if self.v_tangents[u].len()  != self.v_count { return false; }
            if self.twists[u].len()      != self.v_count { return false; }

            for v in 0..self.v_count {
                let p  = self.grid_points[u][v];
                let tu = self.u_tangents[u][v];
                let tv = self.v_tangents[u][v];
                let tw = self.twists[u][v];
                if !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite()) { return false; }
                if !(tu.x.is_finite() && tu.y.is_finite() && tu.z.is_finite()) { return false; }
                if !(tv.x.is_finite() && tv.y.is_finite() && tv.z.is_finite()) { return false; }
                if !(tw.x.is_finite() && tw.y.is_finite() && tw.z.is_finite()) { return false; }
            }
        }
        true
    }

    pub fn to_nurbs<T, F>(&self, mut builder: F) -> Option<T>
    where
        F: FnMut(&[f64], &[f64],
            &Vec<Vec<Point3D>>,
            &Vec<Vec<Vector3D>>,
            &Vec<Vec<Vector3D>>,
            &Vec<Vec<Vector3D>>) -> Option<T>
    {
        if !self.is_valid() { return None; }
        builder(
            &self.u_parameters,
            &self.v_parameters,
            &self.grid_points,
            &self.u_tangents,
            &self.v_tangents,
            &self.twists,
        )
    }

    pub fn for_each_grid<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize, &Point3D, &Vector3D, &Vector3D, &Vector3D),
    {
        for u in 0..self.u_count {
            for v in 0..self.v_count {
                f(
                    u,
                    v,
                    &self.grid_points[u][v],
                    &self.u_tangents[u][v],
                    &self.v_tangents[u][v],
                    &self.twists[u][v],
                );
            }
        }
    }

    /// 점만 필요할 때 간단 버전
    pub fn for_each_point<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize, &Point3D),
    {
        for u in 0..self.u_count {
            for v in 0..self.v_count {
                f(u, v, &self.grid_points[u][v]);
            }
        }
    }
}
```
```rust
/// Bézier 패치 (v-행, u-열 순서로 4x4 제어점)
#[derive(Clone, Debug)]
pub struct BezierPatch {
    pub ctrl: [[Point3D; 4]; 4], // ctrl[v][u]
}

impl HermiteSurface {
    /// 전체 셀을 bicubic Bézier 패치들로 변환합니다.
    /// 반환 길이 = (u_count-1) * (v_count-1)
    pub fn to_bezier_patches(&self) -> Vec<BezierPatch> {
        assert!(self.is_valid(), "Hermite dataset must be valid before conversion");
        let mut out = Vec::with_capacity((self.u_count - 1) * (self.v_count - 1));

        for i in 0..self.u_count - 1 {
            for j in 0..self.v_count - 1 {
                out.push(self.cell_to_bezier(i, j));
            }
        }
        out
    }

    /// 하나의 셀 (i..i+1, j..j+1)을 bicubic Bézier 4x4로 변환
    fn cell_to_bezier(&self, i: usize, j: usize) -> BezierPatch {
        let u0 = self.u_parameters[i];
        let u1 = self.u_parameters[i + 1];
        let v0 = self.v_parameters[j];
        let v1 = self.v_parameters[j + 1];

        let du = u1 - u0;
        let dv = v1 - v0;
        debug_assert!(du.is_finite() && du > 0.0);
        debug_assert!(dv.is_finite() && dv > 0.0);

        // 코너의 데이터 별칭
        let p00 = self.grid_points[i][j];
        let p10 = self.grid_points[i + 1][j];
        let p01 = self.grid_points[i][j + 1];
        let p11 = self.grid_points[i + 1][j + 1];

        let pu00 = self.u_tangents[i][j];
        let pu10 = self.u_tangents[i + 1][j];
        let pu01 = self.u_tangents[i][j + 1];
        let pu11 = self.u_tangents[i + 1][j + 1];

        let pv00 = self.v_tangents[i][j];
        let pv10 = self.v_tangents[i + 1][j];
        let pv01 = self.v_tangents[i][j + 1];
        let pv11 = self.v_tangents[i + 1][j + 1];

        let puv00 = self.twists[i][j];
        let puv10 = self.twists[i + 1][j];
        let puv01 = self.twists[i][j + 1];
        let puv11 = self.twists[i + 1][j + 1];

        // 스케일 계수
        let s_u  = du / 3.0;
        let s_v  = dv / 3.0;
        let s_uv = (du * dv) / 9.0;

        // 4x4 배열(행=v, 열=u)
        let mut b = [[Point3D::new(0.0, 0.0, 0.0); 4]; 4];

        // 하단 행 (v = 0)
        b[0][0] = p00;
        b[0][1] = p00 + (pu00 * s_u);
        b[0][2] = p10 - (pu10 * s_u);
        b[0][3] = p10;

        // v = 1 (아래에서 두번째 행)
        b[1][0] = p00 + (pv00 * s_v);
        b[1][1] = p00 + (pu00 * s_u) + (pv00 * s_v) + (puv00 * s_uv);
        b[1][2] = p10 - (pu10 * s_u) + (pv10 * s_v) - (puv10 * s_uv);
        b[1][3] = p10 + (pv10 * s_v);

        // v = 2 (위에서 두번째 행) — 주의: 위쪽 코너에서 v-접선은 반대방향 보정
        b[2][0] = p01 - (pv01 * s_v);
        b[2][1] = p01 - (pv01 * s_v) + (pu01 * s_u) - (puv01 * s_uv);
        b[2][2] = p11 - (pv11 * s_v) - (pu11 * s_u) + (puv11 * s_uv);
        b[2][3] = p11 - (pv11 * s_v);

        // 상단 행 (v = 3)
        b[3][0] = p01;
        b[3][1] = p01 + (pu01 * s_u);
        b[3][2] = p11 - (pu11 * s_u);
        b[3][3] = p11;

        BezierPatch { ctrl: b }
    }
}
```
```rust
impl std::fmt::Display for HermiteSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HermiteSurface {{ u_count: {}, v_count: {} }}", self.u_count, self.v_count)?;
        writeln!(f, "  U params: {:?}", self.u_parameters)?;
        writeln!(f, "  V params: {:?}", self.v_parameters)?;
        writeln!(f, "  Grid points: {}x{}", self.u_count, self.v_count)?;
        Ok(())
    }
}
```
