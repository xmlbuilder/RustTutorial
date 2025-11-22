# NURBS 표면 모듈 개요
먼저 전체 소스를 수학 중심으로 구조화해 문서화하고, 핵심 함수들의 수식과 정확성(검증 포인트) 검증.

## 표면 평가와 기초 수학
### 표면 정의와 도메인
- 비균일 클램프 B-스플라인 표면( 차수 $(p,q)$ )은 제어망 $P_{i,j}=(x_{ij},y_{ij},z_{ij},w_{ij})$ 와 매듭벡터 U,V를 가집니다.
- 기저함수는 $N_{i,p}(u)와 M_{j,q}(v)$.
- 합리 NURBS 표면:

$$
\mathbf{S}(u,v)=\frac{\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)M_{j,q}(v)\cdot w_{ij}\cdot \mathbf{P_{\mathnormal{i,j}}^{\mathnormal{E}}}}{\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)M_{j,q}(v)\cdot w_{ij}}
$$


- 여기서 $\mathbf{P_{\mathnormal{i,j}}^{\mathnormal{E}}}=(x_{ij},y_{ij},z_{ij})$.
- 평가 도메인(클램프):

$$
u\in [U[p],U[n+1]],\quad v\in [V[q],V[m+1]]
$$

### 스팬 탐색과 기저함수
- 스팬 탐색: $U[k]\leq u<U[k+1]$ 를 만족하는 k를 이진탐색으로 찾음.
- 기저함수: Piegl & Tiller 알고리즘의 좌/우 재귀로 $N_{k-p,p},\dots ,N_{k,p}$ 를 계산.
- 미분 기저: on_ders_basis_func로 $N_{i,p}(u)$, $N'_{i,p}(u)$ 및 $M$ 방향 동일.

### point_at(u,v)
- 라셔널 처리: 국소 스텐실 $(p+1)\times (q+1)$ 에서

$$
\begin{aligned}x_w&=\sum _i\sum _jN_{i,p}(u) M_{j,q}(v) x_{ij} w_{ij},\\ \quad y_w&=\sum _i\sum _jN_{i,p}(u) M_{j,q}(v) y_{ij} w_{ij},\\ \quad z_w&=\sum _i\sum _jN_{i,p}(u) M_{j,q}(v) z_{ij} w_{ij},\\ \quad w&=\sum _i\sum _jN_{i,p}(u) M_{j,q}(v) w_{ij}.\end{aligned}
$$



- 비라셔널: $\sum N_iM_j (x_{ij},y_{ij},z_{ij})$ 로 단순 합.

### 검증 포인트:
- 합리 표면의 동차합 유도와 일치.
- $w\rightarrow 0$ 근처는 수치 불안정 가능 → 작은 에플실론 가드 권장.

### 1차 미분과 면적 밀도
해석적 도함수 S_u, S_v (현재 구현)
- U 방향:

$$
Q_k(v)=\sum _jM_{j,q}(v) P_{(i=k),j} \quad S_u=\sum _kN'_{k,p}(u) Q_k(v)
$$

- V 방향:

$$
R_{\ell }(u)=\sum _iN_{i,p}(u) P_{i,(j=\ell )} \quad S_v=\sum _{\ell }M'_{\ell ,q}(v) R_{\ell }(u)
$$

### 주의:
- 위 수식은 비라셔널 표면에 대해 정확합니다.  
라셔널에서는 몫의 미분(quotient rule) 필요:  

$$
S=\frac{C}{W}, \quad S_u=\frac{C_uW-CW_u}{W^2}, \quad S_v=\frac{C_vW-CW_v}{W^2}
$$

- 현재 `evaluate_derivative_u/v` 는 가중치 미분항을 포함하지 않아 라셔널에는 근사입니다.  
    정밀 수치가 필요한 경우 라셔널 정확 도함수 경로 추가 권장.

## 수치 미분과 면적
- 수치 미분: 중앙차분(경계 근처는 전/후진차분)으로 $\partial S/\partial u,\partial S/\partial v$ 근사.
- 면적 밀도: $\| \partial S/\partial u\times \partial S/\partial v\|$ .
- 적분: 각 유효 knot span을 직사각형으로 분해하여 3×3 Gauss-Legendre로 적분, 자코비안 스케일링 $du\cdot dv$ 포함.
### 검증 포인트:
- GL3 노드/가중치와 아핀변환 적용이 정확.
- 수치 미분의 스텝 크기는 도메인 비례로 안전하지만, knot가 조밀할 때는 국소 스팬 폭 기반 스텝이 더 안정적.

## 그리드 보간, Skin/Loft, Sweep
### 글로벌 보간(Separable 1D)
- 파라미터화: U/V 방향으로 코드-길이(chord-length) 파라미터를 산출(행/열 평균 혹은 방향별).
- 내부 knot: 평균(averaging) 공식으로 클램프 오픈 knot 구성.
- 선형시스템: 샘플 파라미터마다 스팬-비지 않는 영역에 $A_u$, $A_v$ 를 조립한 뒤 LU/밀집해법으로 해.
- 두 단계: 행(U) 보간으로 중간망 R[i,j] → 열(V) 보간으로 최종 P[i,j].
#### 검증 포인트:
- Piegl & Tiller 유형의 표준 절차. 파라미터 중복/특이성에 주의 필요.
- LU 피벗 임계값과 경계 치환은 수치 안정성을 높임.
## Skin/Loft
- skin_u/v: 스트립을 row-major 그리드로 재배열 후 보간.
- loft: sectionsj][i] → 그리드 구성 후 보간.
- ruled: pv=1로 고정.
- loft 옵션: 섹션 방향 라플라시안 스무딩(끝점 고정), 코너 감지/분할 후 각 구간 Loft.
### 검증 포인트:
- 라플라시안 업데이트:

$$
p_j^{new}=p_j^{old}+\lambda \left( \frac{p_{j-1}^{old}+p_{j+1}^{old}}{2}-p_j^{old}\right)
$$

- $\lambda \in (0,1]$ 이면 안정적이나 수축 유발 → 목적에 맞게 반복/강도 설정.

## Sweep
- 프레임 생성: 경로의 T,N,B를 평행수송(parallel transport)로 구축(퇴화 방어 포함).
- 로컬 좌표: 초기 프레임 기준 섹션을 $(s_x,s_y,s_z)$ 로 투영.
- 재구성: 각 경로 스테이션에서 $P_j+s_xB_j+s_yN_j+s_zT_j$ 로 점 생성 → 그리드 보간.
### 검증 포인트:
- 프레임 안정성이 좋고 Frenet보다 흔들림 적음.

## Knot 편집, 분할, Iso-curve
## 끝 Knot 정리
- trim_excess_end_knots_u/v: 끝단 중복을 p+1로 줄이고 제어망을 잘라 크기 일치. 도메인 일관성 유지.
## Knot 삽입
- insert_knot_u/v: Piegl & Tiller 알고리즘을 2D로 확장. 영향 없는 영역 복사 후 영향 영역을 후방 삽입/동차보간(h_lerp).
- 동차보간은 NURBS 가중치 포함에 대해 올바른 방식.
## 분할 split_u/v
- 목표 t에서 삽입해 목표 다중도를 p(또는 q)로 맞춤 후, 제어망과 knot를 좌/우(혹은 상/하)로 분리, 양쪽 knot를 클램프 세팅.
### 검증 포인트:
- mid 인덱스와 기대 knot 길이 조정이 정확.
## Iso-curve 추출
- iso_curve_u_data(u): 고정 u에서 $N_{i,p}(u)$ 로 동차 제어점 열을 혼합, kv와 pv를 사용.
- iso_curve_v_data(v): 고정 v에서 M 방향으로 혼합, ku와 pu를 사용.
### 검증 포인트:
- 동차 블렌딩으로 라셔널 곡선이 정확하게 생성됨.

## 투영과 최적화(point_inversion)
현재 구현 요지
- 목적함수: $\min \frac{1}{2}\| \mathbf{S}(u,v)-\mathbf{p}\| ^2$.
- 수치 프레임: eval_frame_numeric으로 $S$, $S_u$, $S_v$, $n$ 근사.
- 정규방정식:

$$
(J^TJ)\left[ \begin{matrix}\Delta u\\ \Delta v\end{matrix}\right] =-J^Tr,\quad r=S-p
$$

- 초소량 댐핑: $\lambda I$ 추가(고정값), 퇴화 시 그라디언트 하강으로 폴백.
- 스텝 캡/경계 클램프, 거리/파라미터 변화 기준으로 종료.
### 검증 포인트:
- 개념적으로 타당. 다만 라셔널 정확 도함수를 쓰지 않고 수치 근사/고정 댐핑만으로는 특이/고곡률 영역에서 불안정 가능.

## point_inversion 개선 제안
수렴성과 안정성을 단계적으로 올리는 방법을 구체적으로 제안합니다.
### 1) 라셔널 정확 도함수 추가
- 아이디어: 라셔널일 때 C,W와 그 미분을 계산해 몫의 미분을 적용.

$$
C(u,v)=\sum N_iM_j w_{ij} P_{ij}^E,\quad W(u,v)=\sum N_iM_j w_{ij}C_u=\sum N'_iM_j w_{ij} P_{ij}^E,\quad W_u=\sum N'_iM_j w_{ij}- S_u=(C_uW-CW_u)/W^2, 
$$

$S_v$ 도 동일.
- 효과: 수치잡음 감소, 뉴턴 스텝 품질 개선.
### 2) Levenberg–Marquardt형 적응 댐핑
- $\lambda =\alpha \cdot \mathrm{trace}(J^TJ)$ 로 시작 $(\alpha \approx 10^{-6})$.
- 적응: 스텝 후 목적값 감소 시 $\alpha \leftarrow 0.2\alpha$ , 실패 시 $\alpha \leftarrow 5\alpha$ 로 키움.
- 효과: 특이/비선형 구간에서 안정.
### 3) 백트래킹 라인서치
- (du,dv) 스텝 시도 후 f가 증가하면 절반씩 줄여 최대 5회 재시도.
- 효과: 과도한 스텝 방지, 수렴 가속.
### 4) 파라미터 허용오차 스케일링- 고정 1e-10 대신

$$
\mathrm{param\_ tol}=\epsilon _p\cdot \max (1,|U_{\max }-U_{\min }|,|V_{\max }-V_{\min }|)\cdot \frac{\| J^Tr\| }{\| J^TJ\| +\delta }
$$

- $\epsilon _p\sim 10^{-6}$.
- 효과: 스케일과 난이도에 맞춰 조정.
### 5) 초기값 개선(멀티스타트)
- 힌트 없으면 5×5 격자 샘플에서 최근접을 초기값으로 선택.
- 효과: 로컬 최소/경계 트랩 회피.
#### 6) 도메인 외부 허용 단계
- 초기 3회는 allow_outside=true로 시도, 그 후 클램프.
- 효과: 경계 근접 문제에서 진입 허용.
### 7) 수치 미분 스텝 안정화
- eval_frame_numeric 스텝을 가장 가까운 유효 knot span 폭을 기준으로 설정(전역 도메인 대신).
- 효과: knot 클러스터 영역에서 잡음 감소.
### 8) 종료조건 보강
- 최근 M회(예: 5) 동안 개선량 < $\epsilon _f$ 면 plateau로 종료.
- 조건수 악화 감지 시 종료/댐핑 증가.

---

## 소스 코드

```rust
use crate::core::basis::{on_basis_func, on_find_span_index};
use crate::core::curve_utils::is_rational_ctrl;
use crate::core::geom::Point4D;
use crate::core::integrator::Integrator;
use crate::core::knot::{
    KnotVector, on_averaging_internal_surface_knots, on_chord_length_params,
    on_chord_length_params_in_u, on_chord_length_params_in_v, on_ders_basis_func,
    on_ral_c2d_row_major, on_uniform_params_in_open_range, on_uniform_params_in_range,
};
use crate::core::lucmp::{on_lu_solve, on_m_lu_decmp_full};
use crate::core::matrix::on_solve_linear_system_dense;
use crate::core::nurbs_curve::NurbsCurve;
use crate::core::prelude::{Interval, Point3D, Vector3D};
use crate::core::segment3d::Segment3D;
use crate::core::sfun::{SFun, ensure_sfun_shape};
use crate::core::surface_patch::{KnotSpan, SurfacePatch};
use crate::core::types::{
    Degree, Index, MAX_DEGREE, NONE, NurbsError, Param, Real, Rectangle, Result, SurfaceDir,
};
use crate::core::write_format::fmt_slice;
use enterpolation::Curve;
use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::fs::File;
use std::io::Write;
```
```rust
#[derive(Debug, Clone)]
pub struct NurbsIsoCurveData {
    pub degree: Degree,
    pub ctrl: Vec<Point4D>, // NURBS control points (x,y,z,w)
    pub knot: KnotVector,
}
```
```rust
#[derive(Debug, Clone, Copy)]
pub struct LoftOptions {
    /// Loft(섹션 방향) smoothing 수행 여부
    pub smooth_loft_dir: bool,
    /// smoothing 반복 횟수 (예: 5 ~ 30)
    pub smooth_iterations: usize,
    /// smoothing 강도 (0 < lambda < 1, 보통 0.1 ~ 0.5)
    pub smooth_lambda: Real,

    /// 섹션 방향 corner split 수행 여부
    pub corner_split: bool,
    /// 섹션 방향에서 corner 로 간주할 각도 (deg, 예: 45.0)
    pub corner_angle_deg: Real,
    /// corner 판정 시 무시할 최소 segment 길이 (예: 모델 스케일의 1e-3 정도)
    pub corner_min_edge_length: Real,
}
```
```rust
impl Default for LoftOptions {
    fn default() -> Self {
        LoftOptions {
            smooth_loft_dir: false,
            smooth_iterations: 10,
            smooth_lambda: 0.25,
            corner_split: false,
            corner_angle_deg: 45.0,
            corner_min_edge_length: 1e-3,
        }
    }
}
```
```rust
#[derive(Debug, Clone)]
pub struct SurfaceMesh {
    pub vertices: Vec<Point3D>,
    pub normals: Vec<Vector3D>,
    pub uvs: Vec<(Real, Real)>, // (u, v)
    pub indices: Vec<[u32; 3]>, // 삼각형 인덱스
}
```
```rust
impl SurfaceMesh {
    pub fn new() -> Self {
        SurfaceMesh {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }
}
```
```rust
#[derive(Clone, Copy, Debug)]
struct ParamQuad {
    // UV corners: (u,v) in order [ (u0,v0), (u1,v0), (u1,v1), (u0,v1) ]
    uv: [(Real, Real); 4],
}
```
```rust
impl ParamQuad {
    fn uv_bbox(&self) -> (Real, Real, Real, Real) {
        let mut umin = self.uv[0].0;
        let mut umax = self.uv[0].0;
        let mut vmin = self.uv[0].1;
        let mut vmax = self.uv[0].1;
        for &(u, v) in &self.uv {
            if u < umin {
                umin = u;
            }
            if u > umax {
                umax = u;
            }
            if v < vmin {
                vmin = v;
            }
            if v > vmax {
                vmax = v;
            }
        }
        (umin, umax, vmin, vmax)
    }
}
```
```rust
// UV 좌표 한 점
pub type UV = (Real, Real);

/// UV 상의 피처/경계 곡선 (trim loop, feature edge 등)
#[derive(Debug, Clone)]
pub struct FeatureCurveUV {
    pub points: Vec<UV>, // 순서대로 이어지는 polyline
}
```
```rust
/// Adaptive Tessellation 파라미터
#[derive(Debug, Clone)]
pub struct AdaptiveTessellationOptions {
    /// 허용 최대 편차 (패치의 실제 중앙점 vs corner 평면/bi-linear 근사)
    pub max_deviation: Real,

    /// 허용 최대 법선 각도 (radian; 예: 10도 ≈ 10.0_f64.to_radians())
    pub max_normal_angle: Real,

    /// 너무 큰 삼각형을 막기 위한 최대 edge 길이 (모델 스케일 기반)
    pub max_edge_length: Real,

    /// 너무 잘게 쪼개지 않도록 하는 최소 edge 길이 (이보다 작으면 더 이상 subdivision 안 함)
    pub min_edge_length: Real,

    /// 최대 subdivision depth (0 = 전체를 한 번에 / 6~8 정도 권장)
    pub max_depth: usize,

    /// 경계(Trim loop) 근처를 추가로 촘촘하게 할 것인지
    pub refine_boundaries: bool,

    /// 경계/Trim curve들의 UV polyline (있는 경우)
    pub boundary_curves: Vec<FeatureCurveUV>,

    /// 보존하고 싶은 feature edge (UV polyline)
    pub feature_curves: Vec<FeatureCurveUV>,
}
```
```rust
impl Default for AdaptiveTessellationOptions {
    fn default() -> Self {
        AdaptiveTessellationOptions {
            max_deviation: 1e-3,
            max_normal_angle: 10.0_f64.to_radians(),
            max_edge_length: 0.0, // 0이면 비활성
            min_edge_length: 0.0, // 0이면 비활성
            max_depth: 6,
            refine_boundaries: true,
            boundary_curves: Vec::new(),
            feature_curves: Vec::new(),
        }
    }
}
```
```rust
#[derive(Debug, Clone)]
pub struct NurbsSurface {
    pub pu: Degree,
    pub pv: Degree,
    pub nu: Index,
    pub nv: Index,
    pub ctrl: Vec<Point4D>, // row-major (u + nu*v)
    pub ku: KnotVector,
    pub kv: KnotVector,
}
```
```rust
impl NurbsSurface {
    pub fn new(
        pu: Degree,
        pv: Degree,
        nu: Index,
        nv: Index,
        ctrl: Vec<Point4D>,
        ku: KnotVector,
        kv: KnotVector,
    ) -> Result<Self> {
        if (pu as usize) > MAX_DEGREE || (pv as usize) > MAX_DEGREE {
            return Err(NurbsError::InvalidDegreeCurve { got: pu.max(pv) });
        }
        if ctrl.len() != nu * nv {
            return Err(NurbsError::DimensionMismatch("ctrl != nu*nv"));
        }
        ku.check_degree_vs_cp(pu, nu)?;
        kv.check_degree_vs_cp(pv, nv)?;
        Ok(Self {
            pu,
            pv,
            nu,
            nv,
            ctrl,
            ku,
            kv,
        })
    }
```
```rust
    #[inline]
    pub fn idx(&self, u: Index, v: Index) -> Index {
        v * self.nu + u
    }
```
```rust
    #[inline]
    pub fn idx_row_major(nu: usize, i: usize, j: usize) -> usize {
        i + nu * j
    }
```
```rust
    #[inline]
    fn infer_is_rational(ctrl: &[Point4D]) -> bool {
        ctrl.iter().any(|c| c.w.is_finite())
    }
```
```rust
    #[inline]
    fn idx_from_u(nu: usize, i: usize, j: usize) -> usize {
        i + nu * j
    }
```
```rust
    pub fn deg(&self) -> (Degree, Degree) {
        (self.pu, self.pv)
    }
```
```rust
    #[inline]
    pub fn indices(&self) -> (Index, Index, Index, Index) {
        let n = self.nu - 1; // control net in U : last index
        let m = self.nv - 1; // control net in V : last index
        let r = self.ku.len() as Index - 1; // knot U : last index
        let s = self.kv.len() as Index - 1; // knot V : last index
        (n, m, r, s)
    }
```
```rust
    #[inline]
    pub fn ctrl_ref(&self, i: usize, j: usize) -> &Point4D {
        &self.ctrl[self.idx(i, j)]
    }
```
```rust
    #[inline]
    pub fn ctrl_mut(&mut self, i: usize, j: usize) -> &mut Point4D {
        let k = self.idx(i, j);
        &mut self.ctrl[k]
    }
```
```rust
    pub fn euclidean_in_place(&mut self) {
        let (n, m, _, _) = self.indices();
        for i in 0..=n as usize {
            for j in 0..=m as usize {
                let (xw, yw, zw, w) = {
                    let c = &self.ctrl_ref(i, j);
                    (c.x, c.y, c.z, c.w)
                };
                let (x, y, z) = if w.is_finite() && w != 0.0 {
                    (xw / w, yw / w, zw / w)
                } else {
                    (xw, yw, zw)
                };
                let pw = self.ctrl_mut(i, j);
                pw.x = x;
                pw.y = y;
                pw.z = z;
                pw.w = NONE;
            }
        }
    }
```
```rust
    /// Knot 들을 사각형 [ul,ur] x [vb,vt]에 재매핑
    pub fn rescale_knots(&mut self, rect: Rectangle, dir: SurfaceDir) {
        let (p, q) = self.deg();
        let (_, _, r, s) = self.indices();
        let (ul, ur, vb, vt) = (rect.ul, rect.ur, rect.vb, rect.vt);
        let u_knots = &mut self.ku.knots;
        let v_knots = &mut self.kv.knots;

        match dir {
            SurfaceDir::UDir => {
                if !u_knots.is_empty() && (ul != u_knots[0] || ur != u_knots[r as usize]) {
                    let u0 = u_knots[0];
                    let u1 = u_knots[r as usize];
                    let len = u1 - u0;
                    let fac = if len != 0.0 { (ur - ul) / len } else { 0.0 };

                    for i in 0..=p as usize {
                        u_knots[i] = ul;
                    }
                    let first_in = (p as usize).saturating_add(1);
                    let last_in = (r as usize).saturating_sub(p as usize + 1);
                    if first_in <= last_in && fac != 0.0 {
                        for i in first_in..=last_in {
                            u_knots[i] = fac * (u_knots[i] - u0) + ul;
                        }
                    }
                    for i in (r as usize - p as usize)..=r as usize {
                        u_knots[i] = ur;
                    }
                }
            }
            SurfaceDir::VDir => {
                if !v_knots.is_empty() && (vb != v_knots[0] || vt != v_knots[s as usize]) {
                    let v0 = v_knots[0];
                    let v1 = v_knots[s as usize];
                    let len = v1 - v0;
                    let fac = if len != 0.0 { (vt - vb) / len } else { 0.0 };

                    for j in 0..=q as usize {
                        v_knots[j] = vb;
                    }
                    let first_in = (q as usize).saturating_add(1);
                    let last_in = (s as usize).saturating_sub(q as usize + 1);
                    if first_in <= last_in && fac != 0.0 {
                        for j in first_in..=last_in {
                            v_knots[j] = fac * (v_knots[j] - v0) + vb;
                        }
                    }
                    for j in (s as usize - q as usize)..=s as usize {
                        v_knots[j] = vt;
                    }
                }
            }
        }
    }
```
```rust
    /// 분자/분모 추출: num = (xw, yw, zw, NONE), den = w(u,v)
    pub fn numerator_denominator(&self, out_num: &mut NurbsSurface, out_den: &mut SFun) {
        let (n, m, r, s) = self.indices();
        let (p, q) = self.deg();
        ensure_surface_shape(out_num, n, m, p, q, r, s);
        ensure_sfun_shape(out_den, n, m, p, q, r, s);

        let rat = Self::infer_is_rational(&self.ctrl);

        // control net 복사 (row-major)
        for j in 0..=m as usize {
            for i in 0..=n as usize {
                let k = Self::idx_from_u(self.nu as usize, i, j);
                let c = self.ctrl[k];
                let k_out = Self::idx_from_u(out_num.nu as usize, i, j);
                out_num.ctrl[k_out].x = c.x;
                out_num.ctrl[k_out].y = c.y;
                out_num.ctrl[k_out].z = c.z;
                out_num.ctrl[k_out].w = NONE;

                if rat {
                    let k_den = Self::idx_from_u(out_den.nu as usize, i, j);
                    out_den.values[k_den] = c.w;
                }
            }
        }
        // knots 복사
        out_num.ku.copy_from(&self.ku);
        out_num.kv.copy_from(&self.kv);
        if rat {
            out_den.ku.copy_from(&self.ku);
            out_den.kv.copy_from(&self.kv);
        }
    }
```
```rust
    pub fn coords_as_sfun(
        &self,
        wx: &mut SFun,
        wy: &mut SFun,
        wz: &mut SFun,
        mut w: Option<&mut SFun>, // <-- 가변 Option으로 받아서 내부에서 ref mut로 꺼냄
    ) {
        let (n, m, r, s) = self.indices();
        let (p, q) = self.deg();

        // 타깃 버퍼들 모양 맞추기
        ensure_sfun_shape(wx, n, m, p, q, r, s);
        ensure_sfun_shape(wy, n, m, p, q, r, s);
        ensure_sfun_shape(wz, n, m, p, q, r, s);

        let is_rat = Self::infer_is_rational(&self.ctrl);
        if is_rat {
            if let Some(ref mut wf) = w {
                ensure_sfun_shape(*wf, n, m, p, q, r, s);
            }
        }

        // 값 채우기 (row-major)
        for j in 0..=m as usize {
            for i in 0..=n as usize {
                let k = Self::idx_from_u(self.nu as usize, i, j);
                let c = self.ctrl[k];

                let kx = Self::idx_from_u(wx.nu as usize, i, j);
                let ky = Self::idx_from_u(wy.nu as usize, i, j);
                let kz = Self::idx_from_u(wz.nu as usize, i, j);
                wx.values[kx] = c.x;
                wy.values[ky] = c.y;
                wz.values[kz] = c.z;

                if is_rat {
                    if let Some(ref mut wf) = w {
                        let kw = Self::idx_from_u(wf.nu as usize, i, j);
                        wf.values[kw] = c.w;
                    }
                }
            }
        }

        // knots 동기화
        wx.ku.copy_from(&self.ku);
        wy.ku.copy_from(&self.ku);
        wz.ku.copy_from(&self.ku);
        wx.kv.copy_from(&self.kv);
        wy.kv.copy_from(&self.kv);
        wz.kv.copy_from(&self.kv);

        if is_rat {
            if let Some(ref mut wf) = w {
                wf.ku.copy_from(&self.ku);
                wf.kv.copy_from(&self.kv);
            }
        }
    }
```
```rust
    /// U<->V 스왑 (degree / knot / net 전부 교환)
    pub fn swap_uv(&mut self) {
        let (n, m, _r, _s) = self.indices();
        let old_nu = self.nu as usize;
        let old_nv = self.nv as usize;

        // net 전치 (row-major 1D -> 전치 후 1D)
        let mut new_ctrl = vec![
            Point4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: NONE
            };
            old_nu * old_nv
        ];
        for j in 0..=m as usize {
            for i in 0..=n as usize {
                let k_old = Self::idx_from_u(old_nu, i, j); // old: i + nu * j
                let k_new = Self::idx_from_u(old_nv, j, i); // new nu' = old_nv
                new_ctrl[k_new] = self.ctrl[k_old];
            }
        }
        self.ctrl = new_ctrl;

        // degree / knot 교환
        std::mem::swap(&mut self.pu, &mut self.pv);
        std::mem::swap(&mut self.ku, &mut self.kv);

        // nu, nv 교환
        std::mem::swap(&mut self.nu, &mut self.nv);
    }
```
```rust
    /// 압축(메모리 재적재)
    pub fn compact(&mut self) {
        // 1D 버퍼이므로 clone으로 충분
        self.ctrl = self.ctrl.clone();
        self.ku.knots = self.ku.knots.clone();
        self.kv.knots = self.kv.knots.clone();
    }
```
```rust
    /// 깊은 복사 into dst (dst 크기 자동 보정)
    pub fn copy_into(&self, dst: &mut NurbsSurface) {
        let (n, m, r, s) = self.indices();
        let (p, q) = self.deg();
        ensure_surface_shape(dst, n, m, p, q, r, s);

        // ctrl 복사
        dst.ctrl.clone_from(&self.ctrl);

        // knots 복사
        dst.ku.copy_from(&self.ku);
        dst.kv.copy_from(&self.kv);
    }
}
```
```rust
impl NurbsSurface {
    #[inline]
    fn param_range_u(&self) -> (Real, Real) {
        let p = self.pu as usize;
        let knots = &self.ku.knots;
        let umin = knots[p];
        let umax = knots[knots.len() - p - 1];
        (umin, umax)
    }
```
```rust
    /// V 방향 파라미터 범위 [vmin, vmax]
    #[inline]
    fn param_range_v(&self) -> (Real, Real) {
        let q = self.pv as usize;
        let knots = &self.kv.knots;
        let vmin = knots[q];
        let vmax = knots[knots.len() - q - 1];
        (vmin, vmax)
    }
```
```rust
    #[inline]
    fn clamp_u(&self, u: Real) -> Real {
        let (umin, umax) = self.param_range_u();
        if u < umin {
            umin
        } else if u > umax {
            umax
        } else {
            u
        }
    }
```
```rust
    #[inline]
    fn clamp_v(&self, v: Real) -> Real {
        let (vmin, vmax) = self.param_range_v();
        if v < vmin {
            vmin
        } else if v > vmax {
            vmax
        } else {
            v
        }
    }
```
```rust
    /// (u,v)에서 수치 미분으로 S_u, S_v 를 계산한다.
    /// - du, dv 는 적당히 작은 값 (도메인 길이 비례)
    fn numeric_partials(&self, u: Real, v: Real, du: Real, dv: Real) -> (Vector3D, Vector3D) {
        // 파라미터 범위 확인 및 클램프
        let (umin, umax) = self.param_range_u();
        let (vmin, vmax) = self.param_range_v();

        let du = du.max((umax - umin) * 1e-9);
        let dv = dv.max((vmax - vmin) * 1e-9);

        let u_plus = self.clamp_u(u + du);
        let u_minus = self.clamp_u(u - du);
        let v_plus = self.clamp_v(v + dv);
        let v_minus = self.clamp_v(v - dv);

        // Su ≈ (S(u+du, v) - S(u-du, v)) / (2du)
        let p_u_plus = self.point_at(u_plus, v);
        let p_u_minus = self.point_at(u_minus, v);

        let su = Vector3D {
            x: (p_u_plus.x - p_u_minus.x) / (2.0 * du),
            y: (p_u_plus.y - p_u_minus.y) / (2.0 * du),
            z: (p_u_plus.z - p_u_minus.z) / (2.0 * du),
        };

        // Sv ≈ (S(u, v+dv) - S(u, v-dv)) / (2dv)
        let p_v_plus = self.point_at(u, v_plus);
        let p_v_minus = self.point_at(u, v_minus);

        let sv = Vector3D {
            x: (p_v_plus.x - p_v_minus.x) / (2.0 * dv),
            y: (p_v_plus.y - p_v_minus.y) / (2.0 * dv),
            z: (p_v_plus.z - p_v_minus.z) / (2.0 * dv),
        };
        (su, sv)
    }
```
```rust
    /// (u,v)에서의 U 방향 접선 벡터 ∂S/∂u (수치)
    pub fn tangent_u_numeric(&self, u: Real, v: Real) -> Vector3D {
        let (u_min, u_max) = self.u_domain();
        let du_hint = (u_max - u_min) * 1e-4;
        let (su, _) = self.numeric_partials(u, v, du_hint, du_hint);
        su
    }
```
```rust
    /// (u,v)에서의 V 방향 접선 벡터 ∂S/∂v (수치)
    pub fn tangent_v_numeric(&self, u: Real, v: Real) -> Vector3D {
        let (v_min, v_max) = self.v_domain();
        let dv_hint = (v_max - v_min) * 1e-4;
        let (_, sv) = self.numeric_partials(u, v, dv_hint, dv_hint);
        sv
    }
```
```rust
    /// (u,v)에서의 면적 밀도: || S_u x S_v ||.
    ///
    /// C# GSurface.ComputeSurfaceAreaDensity 의 알고리즘적 의도를
    /// 그대로 가져와서 Rust 용으로 재구성한 함수.
    pub fn area_density_numeric(&self, u: Real, v: Real) -> Real {
        let (umin, umax) = self.param_range_u();
        let (vmin, vmax) = self.param_range_v();
        let du = (umax - umin) * 1e-4;
        let dv = (vmax - vmin) * 1e-4;

        let (su, sv) = self.numeric_partials(u, v, du, dv);

        // cross product
        let cx = su.y * sv.z - su.z * sv.y;
        let cy = su.z * sv.x - su.x * sv.z;
        let cz = su.x * sv.y - su.y * sv.x;

        (cx * cx + cy * cy + cz * cz).sqrt()
    }
```
```rust
    pub fn area_gauss_legendre(&self) -> Real {
        let p = self.pu as usize;
        let q = self.pv as usize;

        let u_knots = &self.ku.knots;
        let v_knots = &self.kv.knots;

        let nu_ctrl = self.nu as usize;
        let nv_ctrl = self.nv as usize;

        if nu_ctrl == 0 || nv_ctrl == 0 {
            return 0.0;
        }

        // 3-point Gauss-Legendre on [-1,1]
        const GL3_X: [f64; 3] = [-0.7745966692414834, 0.0, 0.7745966692414834];
        const GL3_W: [f64; 3] = [0.5555555555555556, 0.8888888888888888, 0.5555555555555556];

        let mut area = 0.0;

        // U 방향 span: i = p .. (nu-1)
        // B-spline 기본 식: 유효한 span 은 [U[p], ..., U[n+1]]
        let n = nu_ctrl - 1;
        let m = nv_ctrl - 1;

        for i in p..=n {
            let u0 = u_knots[i];
            let u1 = u_knots[i + 1];
            if (u1 - u0).abs() == 0.0 {
                continue;
            }

            for j in q..=m {
                let v0 = v_knots[j];
                let v1 = v_knots[j + 1];
                if (v1 - v0).abs() == 0.0 {
                    continue;
                }

                // 사각형 [u0,u1]x[v0,v1] 에서 3x3 Gauss-Legendre 적분
                let du = (u1 - u0) * 0.5;
                let dv = (v1 - v0) * 0.5;
                let u_mid = (u0 + u1) * 0.5;
                let v_mid = (v0 + v1) * 0.5;

                let mut local_area = 0.0;

                for (xu, wu) in GL3_X.iter().zip(GL3_W.iter()) {
                    let u = u_mid + du * (*xu as Real);
                    for (xv, wv) in GL3_X.iter().zip(GL3_W.iter()) {
                        let v = v_mid + dv * (*xv as Real);
                        let density = self.area_density_numeric(u, v);
                        local_area += density * (*wu as Real) * (*wv as Real);
                    }
                }

                // 자코비안 (du*dv) 곱하기
                area += local_area * du * dv;
            }
        }
        area
    }
```
```rust
    #[inline]
    pub fn u_domain(&self) -> (Real, Real) {
        let p = self.pu as usize;
        let knots = &self.ku.knots;
        debug_assert!(knots.len() >= 2 * p + 2);
        (knots[p], knots[knots.len() - p - 1])
    }
```
```rust
    /// V 방향 파라미터 범위 [vmin, vmax]
    #[inline]
    pub fn v_domain(&self) -> (Real, Real) {
        let q = self.pv as usize;
        let knots = &self.kv.knots;
        debug_assert!(knots.len() >= 2 * q + 2);
        (knots[q], knots[knots.len() - q - 1])
    }
```
```rust
    /// (u,v)에서 U 방향 속도: ||∂S/∂u||
    pub fn speed_u_numeric(&self, u: Real, v: Real) -> Real {
        let t = self.evaluate_derivative_u(u, v);
        (t.x * t.x + t.y * t.y + t.z * t.z).sqrt()
    }
```
```rust
    /// (u,v)에서 V 방향 속도: ||∂S/∂v||
    pub fn speed_v_numeric(&self, u: Real, v: Real) -> Real {
        let t = self.evaluate_derivative_v(u, v);
        (t.x * t.x + t.y * t.y + t.z * t.z).sqrt()
    }
```
```rust
    fn multiplicity(knot: &[f64], t: f64, tol: f64) -> usize {
        knot.iter().filter(|&&x| (x - t).abs() <= tol).count()
    }
```
```rust
    pub fn find_span(knot: &[f64], p: usize, u: f64, n: usize) -> usize {
        if u <= knot[p] {
            return p;
        }
        if u >= knot[n + 1] {
            return n;
        }
        let (mut low, mut high) = (p, n + 1);
        let mut mid = (low + high) / 2;
        while u < knot[mid] || u >= knot[mid + 1] {
            if u < knot[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }
```
```rust
    pub fn basis_funs(knot: &[f64], p: usize, span: usize, u: f64, out: &mut [f64]) {
        debug_assert!(out.len() >= p + 1);
        out.fill(0.0);
        let mut left = vec![0.0; p + 1];
        let mut right = vec![0.0; p + 1];
        out[0] = 1.0;
        for j in 1..=p {
            left[j] = u - knot[span + 1 - j];
            right[j] = knot[span + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                let denom = right[r + 1] + left[j - r];
                let temp = if denom.abs() > f64::EPSILON {
                    out[r] / denom
                } else {
                    0.0
                };
                out[r] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            out[j] = saved;
        }
    }
```
```rust
    pub fn find_span_u(&self, u: f64) -> usize {
        Self::find_span(&self.ku.knots, self.pu as usize, u, self.nu - 1)
    }
```
```rust
    pub fn find_span_v(&self, v: f64) -> usize {
        Self::find_span(&self.kv.knots, self.pv as usize, v, self.nv - 1)
    }
```
```rust
    pub fn basis_functions_u(&self, span: usize, u: f64) -> Vec<f64> {
        let mut out = vec![0.0; self.pu as usize + 1];
        Self::basis_funs(&self.ku.knots, self.pu as usize, span, u, &mut out);
        out
    }
```
```rust
    pub fn basis_functions_v(&self, span: usize, v: f64) -> Vec<f64> {
        let mut out = vec![0.0; self.pv as usize + 1];
        Self::basis_funs(&self.kv.knots, self.pv as usize, span, v, &mut out);
        out
    }
```
```rust
    pub fn basis_functions_derivative_u(&self, span: usize, u: f64) -> Vec<Vec<f64>> {
        on_ders_basis_func(&self.ku.knots, span, u, self.pu as usize, 1)
    }
```
```rust
    pub fn basis_functions_derivative_v(&self, span: usize, v: f64) -> Vec<Vec<f64>> {
        on_ders_basis_func(&self.kv.knots, span, v, self.pv as usize, 1)
    }
```
```rust
    pub fn evaluate_derivative_u(&self, u: f64, v: f64) -> Vector3D {
        let su = self.find_span_u(u);
        let sv = self.find_span_v(v);
        let nu = self.basis_functions_derivative_u(su, u); // [0]=N, [1]=N'
        let mv = self.basis_functions_v(sv, v); // M

        let p = self.pu as usize;
        let q = self.pv as usize;

        // Q_i(v) = Σ_j M_j(v) P_{ij}
        let mut qv = vec![Vector3D::zero(); p + 1];
        for k in 0..=p {
            let i = su - p + k;
            let mut acc = Vector3D::zero();
            for l in 0..=q {
                let j = sv - q + l;
                let m = mv[l];
                let pij = &self.ctrl[Self::idx_row_major(self.nu, j, i)]; // [v][u] = [j][i]
                acc += m * Vector3D::new(pij.x, pij.y, pij.z);
            }
            qv[k] = acc;
        }

        // S_u = Σ_i N'_i(u) Q_i(v)
        let mut du = Vector3D::zero();
        for k in 0..=p {
            du += nu[1][k] * qv[k];
        }
        du
    }
```
```rust
    pub fn evaluate_derivative_v(&self, u: f64, v: f64) -> Vector3D {
        let su = self.find_span_u(u);
        let sv = self.find_span_v(v);
        let nu = self.basis_functions_u(su, u); // N
        let mv = self.basis_functions_derivative_v(sv, v); // [0]=M, [1]=M'

        let p = self.pu as usize;
        let q = self.pv as usize;

        // R_j(u) = Σ_i N_i(u) P_{ij}
        let mut ru = vec![Vector3D::zero(); q + 1];
        for l in 0..=q {
            let j = sv - q + l;
            let mut acc = Vector3D::zero();
            for k in 0..=p {
                let i = su - p + k;
                let n = nu[k];
                let pij = &self.ctrl[Self::idx_row_major(self.nu, j, i)]; // [v][u] = [j][i]
                acc += n * Vector3D::new(pij.x, pij.y, pij.z);
            }
            ru[l] = acc;
        }

        // S_v = Σ_j M'_j(v) R_j(u)
        let mut dv = Vector3D::zero();
        for l in 0..=q {
            dv += mv[1][l] * ru[l];
        }
        dv
    }
```
```rust
    /// V = v_fixed 에서 U 방향으로 전체 길이(근사)를 구한다.
    ///
    /// - segments: 적분 구간 분할 수 (짝수로 자동 맞춰짐)
    pub fn length_along_u_numeric(&self, v_fixed: Real, segments: usize) -> Real {
        let (umin, umax) = self.u_domain();
        on_simpson_uniform(|u| self.speed_u_numeric(u, v_fixed), umin, umax, segments)
    }
```
```rust
    /// U = u_fixed 에서 V 방향으로 전체 길이(근사)를 구한다.
    pub fn length_along_v_numeric(&self, u_fixed: Real, segments: usize) -> Real {
        let (vmin, vmax) = self.v_domain();
        on_simpson_uniform(|v| self.speed_v_numeric(u_fixed, v), vmin, vmax, segments)
    }
```
```rust
    /// U/V 방향 open 구간의 균일 파라미터
    pub fn uniform_params_u(&self, n_seg: usize) -> Vec<Real> {
        on_uniform_params_in_open_range(&self.ku.knots, self.pu, n_seg)
    }
```
```rust
    pub fn uniform_params_v(&self, n_seg: usize) -> Vec<Real> {
        on_uniform_params_in_open_range(&self.kv.knots, self.pv, n_seg)
    }
```
```rust
    /// NURBS 표면 평가: (u,v) -> 3D 점
    pub fn point_at(&self, u: Real, v: Real) -> Point3D {
        let pu = self.pu as usize;
        let pv = self.pv as usize;

        let nu_pts = self.nu as usize; // # of ctrl in U
        let nv_pts = self.nv as usize; // # of ctrl in V
        let nu = (nu_pts - 1) as usize;
        let nv = (nv_pts - 1) as usize;

        let su = on_find_span_index(nu, pu as u16, u, &self.ku.knots);
        let sv = on_find_span_index(nv, pv as u16, v, &self.kv.knots);

        let mut n_u = vec![0.0; pu as usize + 1];
        let mut n_v = vec![0.0; pv as usize + 1];
        on_basis_func(su, u, pu as u16, &self.ku.knots, &mut n_u);
        on_basis_func(sv, v, pv as u16, &self.kv.knots, &mut n_v);

        let iu0 = (su - pu) as usize;
        let jv0 = (sv - pv) as usize;

        let rat = is_rational_ctrl(&self.ctrl);
        let nu_usize = self.nu as usize;

        if rat {
            let (mut xw, mut yw, mut zw, mut w) = (0.0, 0.0, 0.0, 0.0);
            for k in 0..n_u.len() {
                for l in 0..n_v.len() {
                    let i = iu0 + k;
                    let j = jv0 + l;
                    let cp = self.ctrl[Self::idx_row_major(nu_usize, i, j)];
                    let nkl = n_u[k] * n_v[l];
                    xw += nkl * (cp.x * cp.w);
                    yw += nkl * (cp.y * cp.w);
                    zw += nkl * (cp.z * cp.w);
                    w += nkl * cp.w;
                }
            }
            if w != 0.0 {
                Point3D {
                    x: xw / w,
                    y: yw / w,
                    z: zw / w,
                }
            } else {
                Point3D {
                    x: xw,
                    y: yw,
                    z: zw,
                }
            }
        } else {
            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
            for k in 0..n_u.len() {
                for l in 0..n_v.len() {
                    let i = iu0 + k;
                    let j = jv0 + l;
                    let cp = self.ctrl[Self::idx_row_major(nu_usize, i, j)];
                    let nkl = n_u[k] * n_v[l];
                    x += nkl * cp.x;
                    y += nkl * cp.y;
                    z += nkl * cp.z;
                }
            }
            Point3D { x, y, z }
        }
    }
```
```rust
    /// 균일 (u_count x v_count) 그리드 샘플 (row-major: iu + u_count * jv)
    pub fn sample_grid3d(&self, u_count: usize, v_count: usize) -> Vec<Point3D> {
        if u_count == 0 || v_count == 0 {
            return vec![];
        }
        let up = self.uniform_params_u(u_count - 1);
        let vp = self.uniform_params_v(v_count - 1);

        let mut out = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            u_count * v_count
        ];
        for j in 0..v_count {
            for i in 0..u_count {
                out[i + u_count * j] = self.point_at(up[i], vp[j]);
            }
        }
        out
    }
```
```rust
    pub fn get_interval(&self) -> (Interval, Interval) {
        (self.get_interval_u(), self.get_interval_v())
    }
```
```rust
    pub fn get_interval_u(&self) -> Interval {
        let n = self.ku.len() - 1;
        Interval {
            t0: self.ku.knots[0],
            t1: self.ku.knots[n],
        }
    }
```
```rust
    pub fn get_interval_v(&self) -> Interval {
        let m = self.kv.len() - 1;
        Interval {
            t0: self.kv.knots[0],
            t1: self.kv.knots[m],
        }
    }
```
```rust
    pub fn surface_area(&self) -> f64 {
        let (u_dom, v_dom) = self.get_interval();

        Integrator::gauss_legendre_2d(
            |u, v| {
                let du = self.evaluate_derivative_u(u, v); // Vector3D
                let dv = self.evaluate_derivative_v(u, v); // Vector3D
                println!(
                    "du={:?}, dv={:?}, |du×dv|={}",
                    du,
                    dv,
                    du.cross(&dv).length()
                );
                du.cross(&dv).length() // 면적 요소
            },
            u_dom.t0,
            u_dom.t1,
            v_dom.t0,
            v_dom.t1,
        )
    }
}
```
```rust
fn ensure_surface_shape(
    out_surf: &mut NurbsSurface,
    n: Index,
    m: Index, // 마지막 인덱스 (과거 관례) => 크기 = n+1, m+1
    p: Degree,
    q: Degree,
    r: Index,
    s: Index, // knot 마지막 인덱스 => 길이 = r+1, s+1
) {
    // 1) 컨트롤넷 크기 (row-major: u + nu * v)
    let nu = n + 1;
    let nv = m + 1;
    let need_ctrl_len = (nu as usize) * (nv as usize);

    if out_surf.nu != nu || out_surf.nv != nv || out_surf.ctrl.len() != need_ctrl_len {
        out_surf.nu = nu;
        out_surf.nv = nv;
        out_surf.ctrl.resize(
            need_ctrl_len,
            Point4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: NONE,
            },
        );
    }

    // 2) 차수 갱신
    out_surf.pu = p;
    out_surf.pv = q;

    // 3) Knot 길이 보장
    let rr = (r as usize) + 1;
    let ss = (s as usize) + 1;

    if out_surf.ku.len() != rr {
        out_surf.ku.resize_len(rr, 0.0);
    }
    if out_surf.kv.len() != ss {
        out_surf.kv.resize_len(ss, 0.0);
    }
}
```
```rust
impl NurbsSurface {
    /// C의 U_KILSUR: 표면을 초기 상태로 리셋 (메모리 해제 효과)
    pub fn clear(&mut self) {
        self.ctrl.clear(); // 컨트롤넷 비움
        self.ku.knots.clear(); // U knot 비움
        self.kv.knots.clear(); // V knot 비움
        self.pu = 0;
        self.pv = 0; // 차수 0
        self.nu = 0;
        self.nv = 0; // 사이즈 0
    }
```
```rust
    /// U_OUTSUR: 표면 하나를 텍스트 파일로 덤프
    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut f = File::create(filename)?;
        let is_rat = self.ctrl.iter().any(|c| c.w.is_finite());

        writeln!(f, "pu {}", self.pu)?;
        writeln!(f, "pv {}", self.pv)?;
        writeln!(f, "nu {}", self.nu)?;
        writeln!(f, "nv {}", self.nv)?;
        writeln!(f, "rational {}", if is_rat { 1 } else { 0 })?;

        // ctrl (row-major)
        for j in 0..(self.nv as usize) {
            for i in 0..(self.nu as usize) {
                let k = i + (self.nu as usize) * j;
                let c = self.ctrl[k];
                if is_rat {
                    writeln!(f, "{:.16} {:.16} {:.16} {:.16}", c.x, c.y, c.z, c.w)?;
                } else {
                    writeln!(f, "{:.16} {:.16} {:.16}", c.x, c.y, c.z)?;
                }
            }
        }
        // knots
        for u in &self.ku.knots {
            writeln!(f, "{:.16}", u)?;
        }
        for v in &self.kv.knots {
            writeln!(f, "{:.16}", v)?;
        }
        Ok(())
    }
```
```rust
    /// U_MAKSU4 스타일: 스칼라장(wx, wy, wz, (옵션) w) + 공유 knot로 표면 생성
    pub fn make_surface_from_sfun(
        wx: &SFun,
        wy: &SFun,
        wz: &SFun,
        wopt: Option<&SFun>, // Some(w)면 rational로 생성
        u_knots: &[Real],
        v_knots: &[Real],
        pu: Degree,
        pv: Degree,
    ) -> Self {
        assert_eq!(wx.nu, wy.nu);
        assert_eq!(wx.nu, wz.nu);
        assert_eq!(wx.nv, wy.nv);
        assert_eq!(wx.nv, wz.nv);

        let nu = wx.nu;
        let nv = wx.nv;
        let mut ctrl = Vec::with_capacity((nu as usize) * (nv as usize));
        let idx = |i: usize, j: usize| -> usize { i + (nu as usize) * j };

        match wopt {
            Some(wf) => {
                for j in 0..(nv as usize) {
                    for i in 0..(nu as usize) {
                        ctrl.push(Point4D {
                            x: wx.values[idx(i, j)],
                            y: wy.values[idx(i, j)],
                            z: wz.values[idx(i, j)],
                            w: wf.values[idx(i, j)],
                        });
                    }
                }
            }
            None => {
                for j in 0..(nv as usize) {
                    for i in 0..(nu as usize) {
                        ctrl.push(Point4D {
                            x: wx.values[idx(i, j)],
                            y: wy.values[idx(i, j)],
                            z: wz.values[idx(i, j)],
                            w: f64::NAN, // 비유리
                        });
                    }
                }
            }
        }

        NurbsSurface {
            ctrl,
            pu,
            pv,
            ku: KnotVector {
                knots: u_knots.to_vec(),
            },
            kv: KnotVector {
                knots: v_knots.to_vec(),
            },
            nu,
            nv,
        }
    }
```
```rust
    /// U_MAKSU1 — 이미 존재하는 구조체 포인터를 연결
    pub fn make_surface_linked(
        net_ctrl: Vec<Point4D>,
        pu: Degree,
        pv: Degree,
        ku: KnotVector,
        kv: KnotVector,
        nu: Index,
        nv: Index,
    ) -> Self {
        NurbsSurface {
            ctrl: net_ctrl,
            pu,
            pv,
            ku,
            kv,
            nu,
            nv,
        }
    }
```
```rust
    /// U_MAKSU2 — 제어점과 knots로부터 새 Surface를 생성
    pub fn make_surface_from_points(
        pw: &[Vec<Point4D>],
        pu: Degree,
        pv: Degree,
        u_knots: &[Real],
        v_knots: &[Real],
    ) -> Self {
        let nu = pw.len() as Index;
        let nv = if nu > 0 { pw[0].len() as Index } else { 0 };
        let mut ctrl = Vec::with_capacity((nu * nv) as usize);

        for j in 0..nv as usize {
            for i in 0..nu as usize {
                ctrl.push(pw[i][j]);
            }
        }

        NurbsSurface {
            ctrl,
            pu,
            pv,
            ku: KnotVector {
                knots: u_knots.to_vec(),
            },
            kv: KnotVector {
                knots: v_knots.to_vec(),
            },
            nu,
            nv,
        }
    }
```
```rust
    /// U_MAKSU3 — wx, wy, wz, w로부터 Surface를 생성
    pub fn make_surface_from_coords(
        wx: &[Vec<Real>],
        wy: &[Vec<Real>],
        wz: &[Vec<Real>],
        ww: &[Vec<Real>],
        pu: Degree,
        pv: Degree,
        u_knots: &[Real],
        v_knots: &[Real],
    ) -> Self {
        let nu = wx.len() as Index;
        let nv = if nu > 0 { wx[0].len() as Index } else { 0 };
        let mut ctrl = Vec::with_capacity((nu * nv) as usize);

        for j in 0..nv as usize {
            for i in 0..nu as usize {
                ctrl.push(Point4D {
                    x: wx[i][j],
                    y: wy[i][j],
                    z: wz[i][j],
                    w: ww[i][j],
                });
            }
        }

        NurbsSurface {
            ctrl,
            pu,
            pv,
            ku: KnotVector {
                knots: u_knots.to_vec(),
            },
            kv: KnotVector {
                knots: v_knots.to_vec(),
            },
            nu,
            nv,
        }
    }
```
```rust
    pub fn resize_ctrl(&mut self, new_nu: usize, new_nv: usize) {
        on_ral_c2d_row_major(
            &mut self.ctrl,
            self.nu as usize,
            self.nv as usize,
            new_nu,
            new_nv,
            Point4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: f64::NAN,
            },
        );
        self.nu = new_nu as Index;
        self.nv = new_nv as Index;
    }
}
```
```rust
#[derive(Debug, Clone, Default)]
pub struct SurfaceBasisCache {
    pub last_uv: Option<(Param, Param)>,
    pub basis_u: Vec<Real>,
    pub basis_v: Vec<Real>,
    pub ders_u: Option<Vec<Vec<Real>>>,
    pub ders_v: Option<Vec<Vec<Real>>>,
}
```
```rust
pub fn write_surface_grid(
    surface: &NurbsSurface,
    u_count: usize,
    v_count: usize,
    path: &str,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    let up = surface.uniform_params_u(u_count.saturating_sub(1));
    let vp = surface.uniform_params_v(v_count.saturating_sub(1));
    let pts = surface.sample_grid3d(u_count, v_count);

    let mut f = File::create(path)?;
    for j in 0..v_count {
        for i in 0..u_count {
            let k = i + u_count * j;
            let p = pts[k];
            writeln!(f, "{} {} {:.16} {:.16} {:.16}", i, j, p.x, p.y, p.z)?;
        }
    }
    // (원하면 up/vp도 추가로 덤프 가능)
    Ok(())
}
```
```rust
pub fn write_surface_array_to_file(srfs: &[NurbsSurface], filename: &str) -> std::io::Result<()> {
    let mut f = File::create(filename)?;
    writeln!(f, "count {}", srfs.len())?;
    for (idx, s) in srfs.iter().enumerate() {
        s.write_to_file(&format!("{}#{}", filename, idx))?;
    }
    Ok(())
}
```
```rust
#[inline]
fn idx_nu(nu: usize, i: usize, j: usize) -> usize {
    i + nu * j
}
```
```rust
impl fmt::Display for NurbsSurface {
    /// 기본 출력(요약): 차수/사이즈/knots 일부/ctrl 일부
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = 6;
        let max_knots = 10;
        let max_u = 4;
        let max_v = 4;

        let (p, q) = (self.pu, self.pv);
        let (nu, nv) = (self.nu as usize, self.nv as usize);

        writeln!(f, "Surface {{")?;
        writeln!(f, "  degree: (pu={}, pv={})", p, q)?;
        writeln!(f, "  size  : (nu={}, nv={})", nu, nv)?;

        // knots
        write!(f, "  ku    : ")?;
        fmt_slice(f, &self.ku.knots, max_knots, precision)?;
        writeln!(f)?;

        write!(f, "  kv    : ")?;
        fmt_slice(f, &self.kv.knots, max_knots, precision)?;
        writeln!(f)?;

        // ctrl net 일부
        writeln!(
            f,
            "  ctrl  : row-major (i + nu*j), showing up to {}x{} entries",
            max_u, max_v
        )?;
        let mu = nu.min(max_u);
        let mv = nv.min(max_v);
        for j in 0..mv {
            write!(f, "    v[{j}] ")?;
            for i in 0..mu {
                let k = idx_nu(nu, i, j);
                let c = self.ctrl[k];
                write!(
                    f,
                    "[{:.*}, {:.*}, {:.*}, w={}] ",
                    precision,
                    c.x,
                    precision,
                    c.y,
                    precision,
                    c.z,
                    if c.w.is_finite() {
                        format!("{:.*}", precision, c.w)
                    } else {
                        "NaN".to_string()
                    }
                )?;
            }
            if mu < nu {
                write!(f, "...")?;
            }
            writeln!(f)?;
        }
        if mv < nv {
            writeln!(f, "    ...")?;
        }

        write!(f, "}}")
    }
}
```
```rust
/// 상세 제어 가능한 Surface 덤프(표준 출력)
pub fn dump_surface_limited(
    s: &NurbsSurface,
    max_u: usize,
    max_v: usize,
    knot_max: usize,
    prec: usize,
) {
    // knots
    println!("Surface:");
    println!("  degree: (pu={}, pv={})", s.pu, s.pv);
    println!("  size  : (nu={}, nv={})", s.nu, s.nv);

    // U knots
    {
        let mut buf = String::new();
        let _ = fmt_slice(&mut buf, &s.ku.knots, knot_max, prec);
        println!("  ku    : {}", buf);
    }
    // V knots
    {
        let mut buf = String::new();
        let _ = fmt_slice(&mut buf, &s.kv.knots, knot_max, prec);
        println!("  kv    : {}", buf);
    }

    // ctrl
    let nu = s.nu as usize;
    let nv = s.nv as usize;
    let mu = nu.min(max_u);
    let mv = nv.min(max_v);
    println!(
        "  ctrl  : row-major (i + nu*j), showing up to {}x{}",
        mu, mv
    );
    for j in 0..mv {
        print!("    v[{j}] ");
        for i in 0..mu {
            let k = idx_nu(nu, i, j);
            let c = s.ctrl[k];
            print!(
                "[{:.*}, {:.*}, {:.*}, w={}] ",
                prec,
                c.x,
                prec,
                c.y,
                prec,
                c.z,
                if c.w.is_finite() {
                    format!("{:.*}", prec, c.w)
                } else {
                    "NaN".to_string()
                }
            );
        }
        if mu < nu {
            print!("...");
        }
        println!();
    }
    if mv < nv {
        println!("    ...");
    }
}
```
```rust
/// 간단 덤프(요약)
#[inline]
pub fn dump_surface(s: &NurbsSurface) {
    // 기본 요약 파라미터
    dump_surface_limited(s, 4, 4, 10, 6);
}
```
```rust
/// Surface의 ctrl/ku/kv를 (n,m,r,s, p,q) 관례로 리사이즈
/// (n,m,r,s) = 마지막 인덱스 → 크기/길이는 +1
pub fn surface_ensure_shape(
    srf: &mut NurbsSurface,
    n: Index,
    m: Index,
    p: Degree,
    q: Degree,
    r: Index,
    s: Index,
    fill: Point4D,
) {
    let new_nu = (n as usize) + 1;
    let new_nv = (m as usize) + 1;

    // ctrl (row-major 1D)
    on_ral_c2d_row_major(
        &mut srf.ctrl,
        srf.nu as usize,
        srf.nv as usize,
        new_nu,
        new_nv,
        fill,
    );
    srf.nu = new_nu as Index;
    srf.nv = new_nv as Index;

    // degree
    srf.pu = p;
    srf.pv = q;

    // knots
    let ru = (r as usize) + 1;
    let rv = (s as usize) + 1;
    srf.ku.knots.resize(ru, 0.0);
    srf.kv.knots.resize(rv, 0.0);
}
```
```rust
fn on_eval_vector_surface(s: &NurbsSurface, u: Real, v: Real) -> Point3D {
    // (x,y,z)만 쓰는 벡터 필드로 가정 (w는 무시)
    s.point_at(u, v)
}
```
```rust
fn on_simpson_uniform<F>(mut f: F, a: Real, b: Real, mut n: usize) -> Real
where
    F: FnMut(Real) -> Real,
{
    if n < 2 {
        n = 2;
    }
    // Simpson 법칙은 구간 수가 짝수여야 함
    if n % 2 == 1 {
        n += 1;
    }

    let h = (b - a) / (n as Real);
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + h * (i as Real);
        let fx = f(x);
        if i % 2 == 0 {
            sum += 2.0 * fx;
        } else {
            sum += 4.0 * fx;
        }
    }

    sum * h / 3.0
}
```
```rust
impl NurbsSurface {
    /// 로컬 B-spline 보간으로 격자 점 Q[i,j] 를 통과하는 NurbsSurface 생성.
    ///
    /// - pts  : row-major (i + nu * j), 크기 nu * nv
    /// - nu   : U 방향 데이터/제어점 개수
    /// - nv   : V 방향 데이터/제어점 개수
    /// - pu,pv: 차수 (보통 3,3 사용)
    ///
    /// 원본 C# GSurface.LocalInterpolation(Point3D[,]) 의 의도에 맞춰
    /// chord-length 파라미터 + averaging formula + 2단계 1D 보간으로 구현.
    pub fn local_interpolation_from_grid(
        pts: &[Point3D],
        nu: usize,
        nv: usize,
        pu: Degree,
        pv: Degree,
    ) -> Result<Self> {
        if nu == 0 || nv == 0 {
            return Err(NurbsError::InvalidSize);
        }
        if pts.len() != nu * nv {
            return Err(NurbsError::InvalidSize);
        }
        if (pu as usize) >= nu || (pv as usize) >= nv {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, pv),
                max: MAX_DEGREE as Degree,
            });
        }

        let nu_i = nu as Index;
        let nv_i = nv as Index;

        // -----------------------------
        // 1) U, V 방향 chord-length 파라미터 계산
        //    - 각 행/열 별로 chord-length, 전체 평균 사용
        // -----------------------------
        let mut u_params = vec![0.0; nu];
        let mut v_params = vec![0.0; nv];

        // U 방향: j 고정(각 행)을 따라 i를 증가시키며 chord-length
        for j in 0..nv {
            let mut row = Vec::with_capacity(nu);
            for i in 0..nu {
                let idx = i + nu * j;
                row.push(pts[idx]);
            }
            if row.len() >= 2 {
                let u_row = on_chord_length_params(&row); // 0..1
                for i in 0..nu {
                    u_params[i] += u_row[i];
                }
            }
        }
        if nv > 0 {
            for i in 0..nu {
                u_params[i] /= nv as Real;
            }
        }

        // V 방향: i 고정(각 열)을 따라 j를 증가시키며 chord-length
        for i in 0..nu {
            let mut col = Vec::with_capacity(nv);
            for j in 0..nv {
                let idx = i + nu * j;
                col.push(pts[idx]);
            }
            if col.len() >= 2 {
                let v_col = on_chord_length_params(&col); // 0..1
                for j in 0..nv {
                    v_params[j] += v_col[j];
                }
            }
        }
        if nu > 0 {
            for j in 0..nv {
                v_params[j] /= nu as Real;
            }
        }

        println!("u_params {:?}", u_params);
        println!("v_params {:?}", v_params);

        // -----------------------------
        // 2) U, V 방향 보간용 knot + LU 준비
        // -----------------------------
        let (u_knots, lu_u) = on_build_bspline_interpolation_lu(&u_params, pu)
            .ok_or(NurbsError::LinearSolveFailed)?;
        let (v_knots, lu_v) = on_build_bspline_interpolation_lu(&v_params, pv)
            .ok_or(NurbsError::LinearSolveFailed)?;

        println!("u_knots {:?}", u_knots);
        println!("v_knots {:?}", v_knots);

        // -----------------------------
        // 3) 1단계: U 방향 보간 → 중간 제어망 R[i,j]
        // -----------------------------
        let mut r_grid = vec![Point3D::default(); nu * nv];

        for j in 0..nv {
            // RHS: 데이터 점 (행 j)
            let mut bx = vec![0.0; nu];
            let mut by = vec![0.0; nu];
            let mut bz = vec![0.0; nu];

            for i in 0..nu {
                let idx = i + nu * j;
                let p = pts[idx];
                bx[i] = p.x;
                by[i] = p.y;
                bz[i] = p.z;
            }

            // Ax = b 해 풀기 (x: 제어점 좌표)
            if !on_lu_solve(&lu_u, &mut bx)
                || !on_lu_solve(&lu_u, &mut by)
                || !on_lu_solve(&lu_u, &mut bz)
            {
                return Err(NurbsError::LinearSolveFailed);
            }

            // 결과를 중간 제어망 R[i,j]에 저장
            for i in 0..nu {
                let idx = i + nu * j;
                r_grid[idx] = Point3D {
                    x: bx[i],
                    y: by[i],
                    z: bz[i],
                };
            }
        }

        // -----------------------------
        // 4) 2단계: V 방향 보간 → 최종 제어망 P[i,j]
        // -----------------------------
        let mut ctrl: Vec<Point4D> = vec![
            Point4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0
            };
            nu * nv
        ];

        for i in 0..nu {
            // RHS: 중간 제어망의 열 i를 데이터로 사용
            let mut bx = vec![0.0; nv];
            let mut by = vec![0.0; nv];
            let mut bz = vec![0.0; nv];

            for j in 0..nv {
                let idx = i + nu * j;
                let p = r_grid[idx];
                bx[j] = p.x;
                by[j] = p.y;
                bz[j] = p.z;
            }

            if !on_lu_solve(&lu_v, &mut bx)
                || !on_lu_solve(&lu_v, &mut by)
                || !on_lu_solve(&lu_v, &mut bz)
            {
                return Err(NurbsError::LinearSolveFailed);
            }

            // 최종 제어점 P[i,j] 저장 (row-major: i + nu * j)
            for j in 0..nv {
                let idx = i + nu * j;
                ctrl[idx] = Point4D {
                    x: bx[j],
                    y: by[j],
                    z: bz[j],
                    w: 1.0,
                };
            }
        }

        // -----------------------------
        // 5) NurbsSurface 생성
        // -----------------------------
        let ku = KnotVector::new(u_knots)?;
        let kv = KnotVector::new(v_knots)?;

        NurbsSurface::new(pu, pv, nu_i, nv_i, ctrl, ku, kv)
    }
```
```rust
    pub fn local_interpolation_from_grid_chord_base(
        pts: &[Point3D],
        nu: usize,
        nv: usize,
        pu: Degree,
        pv: Degree,
    ) -> Result<Self> {
        if pts.len() != nu * nv {
            return Err(NurbsError::InvalidInput(
                "local_interpolation_from_grid: pts.len() != nu*nv".to_string(),
            ));
        }
        if nu < (pu as usize + 1) || nv < (pv as usize + 1) {
            return Err(NurbsError::InvalidInput(
                "local_interpolation_from_grid: not enough points for given degrees".to_string(),
            ));
        }

        // 1) U, V 방향 chord-length 파라미터
        let u_params = on_chord_length_params_in_u(pts, nu, nv);
        let v_params = on_chord_length_params_in_v(pts, nu, nv);

        // 2) U, V 방향 knot vector 생성 (1D 보간용)
        let u_knots = on_averaging_internal_surface_knots(&u_params, pu);
        let v_knots = on_averaging_internal_surface_knots(&v_params, pv);

        // 3) U 방향 보간용 행렬 A_u 구성 (nu x nu)
        let m_u = nu;
        let n_u = nu - 1;
        let p_u = pu;
        let mut a_u = vec![vec![0.0; m_u]; m_u];

        for (row, &u) in u_params.iter().enumerate() {
            let span = on_find_span_index(n_u as Index, p_u, u, &u_knots);
            let mut n_vec = vec![0.0; p_u as usize + 1];
            on_basis_func(span, u, p_u, &u_knots, &mut n_vec);

            let first = span as usize - p_u as usize;
            for a in 0..n_vec.len() {
                a_u[row][first + a] = n_vec[a];
            }
        }

        // 4) V 방향 보간용 행렬 A_v 구성 (nv x nv)
        let m_v = nv;
        let n_v = nv - 1;
        let p_v = pv;
        let mut a_v = vec![vec![0.0; m_v]; m_v];

        for (row, &v) in v_params.iter().enumerate() {
            let span = on_find_span_index(n_v as Index, p_v, v, &v_knots);
            let mut n_vec = vec![0.0; p_v as usize + 1];
            on_basis_func(span, v, p_v, &v_knots, &mut n_vec);

            let first = span as usize - p_v as usize;
            for a in 0..n_vec.len() {
                a_v[row][first + a] = n_vec[a];
            }
        }

        // 5) 1단계: 각 V=j 고정, U 방향으로 보간 → 중간 격자 R(u_i, v_j)
        let mut r_grid = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            nu * nv
        ];

        for j in 0..nv {
            // RHS: Q[i,j]
            let mut bx = vec![0.0; nu];
            let mut by = vec![0.0; nu];
            let mut bz = vec![0.0; nu];

            for i in 0..nu {
                let q = &pts[i + nu * j];
                bx[i] = q.x;
                by[i] = q.y;
                bz[i] = q.z;
            }

            let cx =
                on_solve_linear_system_dense(&a_u, &bx).ok_or(NurbsError::LinearSolveFailed)?;
            let cy =
                on_solve_linear_system_dense(&a_u, &by).ok_or(NurbsError::LinearSolveFailed)?;
            let cz =
                on_solve_linear_system_dense(&a_u, &bz).ok_or(NurbsError::LinearSolveFailed)?;

            for i in 0..nu {
                r_grid[i + nu * j] = Point3D {
                    x: cx[i],
                    y: cy[i],
                    z: cz[i],
                };
            }
        }

        // 6) 2단계: 각 U=i 고정, V 방향으로 보간 → 최종 control grid P(i,j)
        let mut ctrl = Vec::with_capacity(nu * nv);

        for j in 0..nv {
            // 나중에 j-index를 안에서 돌려도 되고, 여기서는 row-major로 쌓기
        }

        for i in 0..nu {
            let mut bx = vec![0.0; nv];
            let mut by = vec![0.0; nv];
            let mut bz = vec![0.0; nv];

            for j in 0..nv {
                let r = &r_grid[i + nu * j];
                bx[j] = r.x;
                by[j] = r.y;
                bz[j] = r.z;
            }

            let cx =
                on_solve_linear_system_dense(&a_v, &bx).ok_or(NurbsError::LinearSolveFailed)?;
            let cy =
                on_solve_linear_system_dense(&a_v, &by).ok_or(NurbsError::LinearSolveFailed)?;
            let cz =
                on_solve_linear_system_dense(&a_v, &bz).ok_or(NurbsError::LinearSolveFailed)?;

            for j in 0..nv {
                ctrl.push(Point4D {
                    x: cx[j],
                    y: cy[j],
                    z: cz[j],
                    w: 1.0,
                });
            }
        }

        // 7) NurbsSurface 생성
        Ok(NurbsSurface {
            // 여기 필드 이름은 네가 쓰는 struct에 맞게 조정해줘
            // 예시:
            pu,
            pv,
            nu: nu as Index,
            nv: nv as Index,
            ctrl,
            ku: KnotVector { knots: u_knots },
            kv: KnotVector { knots: v_knots },
        })
    }
```
```rust
    /// U, V 양 끝에서 degree+1 개를 초과해서 중복된 knot 들을
    /// 제거하고, 그에 맞게 control net 도 잘라내는 함수입니다.
    ///
    /// - `tol` 은 knot 비교에 쓰는 허용 오차입니다. 보통 ON_TOL12 정도를 쓰시면 됩니다.
    pub fn trim_excess_end_knots(&mut self, tol: Real) {
        self.trim_excess_end_knots_u(tol);
        self.trim_excess_end_knots_v(tol);
    }
    /// U 방향 끝단에서 불필요한 중복 knot 제거 + control point 행(열) 정리
    pub fn trim_excess_end_knots_u(&mut self, tol: Real) {
        let p = self.pu as usize;
        let old_nu = self.nu as usize;
        if old_nu == 0 {
            return;
        }

        let knot_count = self.ku.knots.len();
        if knot_count == 0 {
            return;
        }

        // 왼쪽 끝 multiplicity
        let left_mult = {
            let mut cnt = 1usize;
            let u0 = self.ku.knots[0];
            for &u in &self.ku.knots[1..] {
                if (u - u0).abs() <= tol {
                    cnt += 1;
                } else {
                    break;
                }
            }
            cnt
        };

        // 오른쪽 끝 multiplicity
        let right_mult = {
            let mut cnt = 1usize;
            let un = self.ku.knots[knot_count - 1];
            for &u in self.ku.knots[..knot_count - 1].iter().rev() {
                if (u - un).abs() <= tol {
                    cnt += 1;
                } else {
                    break;
                }
            }
            cnt
        };

        let allowed = p + 1;
        let left_remove = if left_mult > allowed {
            left_mult - allowed
        } else {
            0
        };
        let right_remove = if right_mult > allowed {
            right_mult - allowed
        } else {
            0
        };

        // 제거할 것이 없으면 종료
        if left_remove == 0 && right_remove == 0 {
            return;
        }

        // 컨트롤 포인트를 전부 날려 버리는 상황은 방지
        if left_remove + right_remove >= old_nu {
            return;
        }

        // 새로운 U-knot 벡터 구성
        let new_knot_count = knot_count - left_remove - right_remove;
        let mut new_knots = Vec::with_capacity(new_knot_count);
        new_knots.extend_from_slice(&self.ku.knots[left_remove..knot_count - right_remove]);
        self.ku.knots = new_knots;

        // 새로운 control net 구성 (row-major: i + nu * j)
        let nv = self.nv as usize;
        let new_nu = old_nu - left_remove - right_remove;
        let mut new_ctrl = Vec::with_capacity(new_nu * nv);

        for j in 0..nv {
            for i_new in 0..new_nu {
                let i_old = i_new + left_remove;
                let idx_old = i_old + old_nu * j;
                new_ctrl.push(self.ctrl[idx_old]);
            }
        }

        self.nu = new_nu as Index;
        self.ctrl = new_ctrl;
    }
```
```rust
    /// V 방향 끝단에서 불필요한 중복 knot 제거 + control point 행(열) 정리
    pub fn trim_excess_end_knots_v(&mut self, tol: Real) {
        let p = self.pv as usize;
        let old_nv = self.nv as usize;
        if old_nv == 0 {
            return;
        }

        let knot_count = self.kv.knots.len();
        if knot_count == 0 {
            return;
        }

        // 아래(시작) 쪽 multiplicity
        let bottom_mult = {
            let mut cnt = 1usize;
            let v0 = self.kv.knots[0];
            for &v in &self.kv.knots[1..] {
                if (v - v0).abs() <= tol {
                    cnt += 1;
                } else {
                    break;
                }
            }
            cnt
        };

        // 위(끝) 쪽 multiplicity
        let top_mult = {
            let mut cnt = 1usize;
            let vn = self.kv.knots[knot_count - 1];
            for &v in self.kv.knots[..knot_count - 1].iter().rev() {
                if (v - vn).abs() <= tol {
                    cnt += 1;
                } else {
                    break;
                }
            }
            cnt
        };

        let allowed = p + 1;
        let bottom_remove = if bottom_mult > allowed {
            bottom_mult - allowed
        } else {
            0
        };
        let top_remove = if top_mult > allowed {
            top_mult - allowed
        } else {
            0
        };

        if bottom_remove == 0 && top_remove == 0 {
            return;
        }

        if bottom_remove + top_remove >= old_nv {
            return;
        }

        // 새로운 V-knot 벡터
        let new_knot_count = knot_count - bottom_remove - top_remove;
        let mut new_knots = Vec::with_capacity(new_knot_count);
        new_knots.extend_from_slice(&self.kv.knots[bottom_remove..knot_count - top_remove]);
        self.kv.knots = new_knots;

        // 새로운 control net 구성
        let nu = self.nu as usize;
        let new_nv = old_nv - bottom_remove - top_remove;
        let mut new_ctrl = Vec::with_capacity(nu * new_nv);

        for j_new in 0..new_nv {
            let j_old = j_new + bottom_remove;
            for i in 0..nu {
                let idx_old = i + nu * j_old;
                new_ctrl.push(self.ctrl[idx_old]);
            }
        }

        self.nv = new_nv as Index;
        self.ctrl = new_ctrl;
    }
}
```
```rust
impl NurbsSurface {
    /// U 방향 스킨:
    ///   - strips[k] : k번째 U-방향 프로파일 곡선을 따라 샘플링한 점들
    ///   - strips.len() = nu (U 방향 곡선 개수)
    ///   - strips[k].len() = nv (각 곡선의 V 방향 샘플 개수, 모두 같아야 함)
    ///
    /// 내부적으로는 point grid 를 만들어
    ///   `local_interpolation_from_grid` 을 호출한다.
    pub fn skin_u_from_point_strips(
        strips: &[Vec<Point3D>],
        pu: Degree,
        pv: Degree,
    ) -> Result<Self> {
        if strips.is_empty() {
            return Err(NurbsError::InvalidSize);
        }

        let nu = strips.len();
        let nv = strips[0].len();
        if nv == 0 {
            return Err(NurbsError::InvalidSize);
        }

        // 모든 스트립의 길이가 동일한지 확인
        for (k, strip) in strips.iter().enumerate() {
            if strip.len() != nv {
                return Err(NurbsError::InvalidSize); // 서로 다른 샘플 수
            }
        }

        if (pu as usize) >= nu || (pv as usize) >= nv {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, pv),
                max: MAX_DEGREE as Degree,
            });
        }

        // LocalInterpolation 이 기대하는 row-major (i + nu*j) 형태의 그리드로 재배치:
        //   i: U 인덱스 (strip index)
        //   j: V 인덱스 (strip 안에서의 샘플 index)
        let mut pts: Vec<Point3D> = Vec::with_capacity(nu * nv);
        pts.resize(nu * nv, Point3D::default());

        for j in 0..nv {
            for i in 0..nu {
                let p = strips[i][j];
                let idx = i + nu * j;
                pts[idx] = p;
            }
        }

        // 이미 구현한 LocalInterpolation 이용
        NurbsSurface::local_interpolation_from_grid_chord_base(&pts, nu, nv, pu, pv)
    }
```
```rust
    /// V 방향 스킨:
    ///   - strips[k] : k번째 V-방향 프로파일 곡선을 따라 샘플링한 점들
    ///   - strips.len() = nv (V 방향 곡선 개수)
    ///   - strips[k].len() = nu (각 곡선의 U 방향 샘플 개수, 모두 같아야 함)
    ///
    /// 내부적으로는 point grid 를 만들어
    ///   `local_interpolation_from_grid` 을 호출한다.
    pub fn skin_v_from_point_strips(
        strips: &[Vec<Point3D>],
        pu: Degree,
        pv: Degree,
    ) -> Result<Self> {
        if strips.is_empty() {
            return Err(NurbsError::InvalidSize);
        }

        let nv = strips.len();
        let nu = strips[0].len();
        if nu == 0 {
            return Err(NurbsError::InvalidSize);
        }

        // 모든 스트립의 길이가 동일한지 확인
        for strip in strips.iter() {
            if strip.len() != nu {
                return Err(NurbsError::InvalidSize);
            }
        }

        if (pu as usize) >= nu || (pv as usize) >= nv {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, pv),
                max: MAX_DEGREE as Degree,
            });
        }

        // LocalInterpolation 의 row-major (i + nu*j) 에 맞게 그리드 구성:
        // 여기서 i: U 인덱스, j: V 인덱스.
        //
        // strips[j][i] 가 (i,j)에 해당하는 점이므로
        // pts[i + nu*j] = strips[j][i] 로 채운다.
        let mut pts: Vec<Point3D> = Vec::with_capacity(nu * nv);
        pts.resize(nu * nv, Point3D::default());

        for j in 0..nv {
            for i in 0..nu {
                let p = strips[j][i];
                let idx = i + nu * j;
                pts[idx] = p;
            }
        }

        NurbsSurface::local_interpolation_from_grid_chord_base(&pts, nu, nv, pu, pv)
    }
```
```rust
    pub fn sample_curves_to_point_strips<C, FDom, FEval>(
        curves: &[C],
        sample_count: usize,
        get_domain: FDom,
        eval_point: FEval,
    ) -> Vec<Vec<Point3D>>
    where
        FDom: Fn(&C) -> (Real, Real),
        FEval: Fn(&C, Real) -> Point3D,
    {
        let n_curves = curves.len();
        if n_curves == 0 || sample_count == 0 {
            return Vec::new();
        }

        let n = sample_count.max(2); // 1개 샘플은 너무 의미가 없으니 최소 2로 올림

        let mut strips: Vec<Vec<Point3D>> = Vec::with_capacity(n_curves);

        for c in curves {
            let (t0, t1) = get_domain(c);
            let dt = (t1 - t0) / ((n - 1) as Real);

            let mut pts: Vec<Point3D> = Vec::with_capacity(n);
            for k in 0..n {
                let t = if k + 1 == n {
                    // 마지막 샘플은 정확히 t1 이 되도록
                    t1
                } else {
                    t0 + dt * (k as Real)
                };
                pts.push(eval_point(c, t));
            }

            strips.push(pts);
        }

        strips
    }
}
```
```rust
fn on_build_bspline_interpolation_lu(
    params: &[Real],
    p: Degree,
) -> Option<(Vec<Real>, crate::core::lucmp::LU)> {
    let m_pts = params.len();
    if m_pts == 0 {
        return None;
    }
    let n_ctrl = m_pts; // 보간이므로 #ctrl == #data
    let n = (n_ctrl - 1) as Index;

    let pz = p as usize;
    if pz == 0 || n_ctrl <= pz {
        return None;
    }

    // clamped open non-uniform knot vector (0..1)
    let knots = on_averaging_internal_surface_knots(params, p);

    println!("knots {:?}", knots);

    // 시스템 행렬 A (m_pts x m_pts)
    let mut a = vec![vec![0.0; m_pts]; m_pts];

    let mut n_vec = vec![0.0; pz + 1];

    for (row, &u) in params.iter().enumerate() {
        // span 위치
        let span = on_find_span_index(n, p, u, &knots);
        let span_us = span as usize;

        // basis N_{i,p}(u)
        n_vec.fill(0.0);
        on_basis_func(span, u, p, &knots, &mut n_vec);

        // non-zero 지원 범위: i = span-p .. span
        let first = span_us - pz;
        for k in 0..=pz {
            let col = first + k;
            if col < m_pts {
                a[row][col] = n_vec[k];
            }
        }
    }

    // LU 분해
    // ludt: pivot이 이 값보다 작으면 실패 처리
    let ludt = 1e-12;
    let lu = on_m_lu_decmp_full(a, ludt)?;
    Some((knots, lu))
}
```
```rust
pub fn on_sample_nurbs_curves_uniform(
    curves: &[NurbsCurve],
    sample_count: usize,
) -> Vec<Vec<Point3D>> {
    NurbsSurface::sample_curves_to_point_strips(
        curves,
        sample_count,
        // get_domain
        |c: &NurbsCurve| {
            // 예시: NurbsCurve 가 이런 식 API 라고 가정
            //   fn domain(&self) -> Interval;
            let Interval { t0, t1 } = c.domain;
            (t0, t1)
        },
        // eval_point
        |c: &NurbsCurve, t: Real| {
            // 예시: NurbsCurve 가 이런 식 API 라고 가정
            //   fn eval_point(&self, t: Real) -> Point3D;
            c.eval_point(t)
        },
    )
}
```
```rust
impl NurbsSurface {
    /// U 방향으로 여러 개의 knot 를 삽입한다.
    /// new_knots 는 비감소 순서라고 가정한다.
    /// 리턴값은 실제 삽입된 knot 개수 (new_knots.len()).
    pub fn insert_knot_u(&mut self, new_knots: &[Real]) -> usize {
        if new_knots.is_empty() {
            return 0;
        }

        let p = self.pu as usize;
        let old_nu = self.nu as usize;
        let old_nv = self.nv as usize;

        debug_assert!(old_nu >= p + 1);

        let n = old_nu - 1; // 마지막 U 인덱스
        let m = n + p + 1; // 마지막 knot 인덱스 (ku.len()-1)
        let r = new_knots.len() - 1; // 반복문에서 쓰는 r (Piegl 스타일)

        let original = self.clone();
        let orig_u = &original.ku.knots;

        // 새 control net 크기 : (old_nu + (r+1)) x old_nv
        let new_nu = n + r + 2; // = old_nu + r + 1
        let mut new_ctrl = vec![Point4D::new(0.0, 0.0, 0.0, 0.0); new_nu * old_nv];

        // 새 knot 벡터 U'
        let mut new_u = vec![0.0; m + r + 2];

        // new_knots[0] 과 new_knots[r] 이 들어갈 span 구하기
        let first = on_find_span_index(n as Index, self.pu, new_knots[0], orig_u) as usize;
        let last = on_find_span_index(n as Index, self.pu, new_knots[r], orig_u) as usize + 1;

        // 1) 영향 없는 control points 복사 (왼쪽 영역)
        for v in 0..old_nv {
            for i in 0..=(first - p) {
                let src = original.ctrl[i + old_nu * v];
                new_ctrl[i + new_nu * v] = src;
            }
        }
        // 2) 영향 없는 control points 복사 (오른쪽 영역)
        for v in 0..old_nv {
            for i in (last - 1)..=n {
                let src = original.ctrl[i + old_nu * v];
                new_ctrl[i + r + 1 + new_nu * v] = src;
            }
        }

        // 3) 영향 없는 knot 들 복사
        for i in 0..=first {
            new_u[i] = orig_u[i];
        }
        for i in (last + p)..=m {
            new_u[i + r + 1] = orig_u[i];
        }

        // 4) 영향을 받는 구간 안에서, 뒤에서부터 knot 삽입
        let mut current = last + p - 1;
        let mut target = last + p + r;

        for ins_idx in (0..=r).rev() {
            let u_insert = new_knots[ins_idx];

            // 원래 knot 가 새로 삽입되는 knot보다 크고, first 보다 오른쪽이면
            // 기존 control point / knot 를 한 칸씩 뒤로 밀어냄
            while u_insert <= orig_u[current] && current > first {
                for v in 0..old_nv {
                    let src = original.ctrl[(current - p - 1) + old_nu * v];
                    new_ctrl[(target - p - 1) + new_nu * v] = src;
                }
                new_u[target] = orig_u[current];
                target -= 1;
                current -= 1;
            }

            // 한 줄 복사 (초기값)
            for v in 0..old_nv {
                let tmp = new_ctrl[(target - p) + new_nu * v];
                new_ctrl[(target - p - 1) + new_nu * v] = tmp;
            }

            // 차수 p 만큼 블렌딩해서 새로운 control point 생성
            for j in 1..=p {
                let idx = target - p + j;
                let denom = new_u[target + j] - u_insert;
                if denom.abs() < 1e-12 {
                    // degenerate → 그냥 오른쪽 점 복사
                    for v in 0..old_nv {
                        let tmp = new_ctrl[idx + new_nu * v];
                        new_ctrl[idx - 1 + new_nu * v] = tmp;
                    }
                } else {
                    let denom2 = new_u[target + j] - orig_u[current - p + j];
                    let alpha = if denom2.abs() > 1e-12 {
                        denom / denom2
                    } else {
                        0.0
                    };
                    for v in 0..old_nv {
                        let a = new_ctrl[idx - 1 + new_nu * v];
                        let b = new_ctrl[idx + new_nu * v];
                        new_ctrl[idx - 1 + new_nu * v] = Point4D::h_lerp(&a, &b, 1.0 - alpha);
                    }
                }
            }

            new_u[target] = u_insert;
            target -= 1;
        }

        self.ctrl = new_ctrl;
        self.nu = new_nu as Index;
        self.ku =
            KnotVector::new(new_u).expect("insert_knot_u: new knot vector is not non-decreasing");

        r + 1 // 삽입된 knot 개수 = new_knots.len()
    }
```
```rust
    /// V 방향으로 여러 개의 knot 를 삽입한다.
    /// new_knots 는 비감소 순서라고 가정한다.
    pub fn insert_knot_v(&mut self, new_knots: &[Real]) -> usize {
        if new_knots.is_empty() {
            return 0;
        }

        let q = self.pv as usize;
        let old_nu = self.nu as usize;
        let old_nv = self.nv as usize;

        debug_assert!(old_nv >= q + 1);

        let n = old_nv - 1;
        let m = n + q + 1;
        let r = new_knots.len() - 1;

        let original = self.clone();
        let orig_v = &original.kv.knots;

        // 새 control net 크기 : old_nu x (old_nv + (r+1))
        let new_nv = n + r + 2; // = old_nv + r + 1
        let mut new_ctrl = vec![Point4D::new(0.0, 0.0, 0.0, 0.0); old_nu * new_nv];
        let mut new_v = vec![0.0; m + r + 2];

        let first = on_find_span_index(n as Index, self.pv, new_knots[0], orig_v) as usize;
        let last = on_find_span_index(n as Index, self.pv, new_knots[r], orig_v) as usize + 1;

        // 1) 영향 없는 control points 복사 (아래쪽 영역)
        for u in 0..old_nu {
            for j in 0..=(first - q) {
                let src = original.ctrl[u + old_nu * j];
                new_ctrl[u + old_nu * j] = src;
            }
        }
        // 2) 영향 없는 control points 복사 (위쪽 영역)
        for u in 0..old_nu {
            for j in (last - 1)..=n {
                let src = original.ctrl[u + old_nu * j];
                new_ctrl[u + old_nu * (j + r + 1)] = src;
            }
        }

        // 3) 영향 없는 knot 들 복사
        for j in 0..=first {
            new_v[j] = orig_v[j];
        }
        for j in (last + q)..=m {
            new_v[j + r + 1] = orig_v[j];
        }

        // 4) V 방향 knot 삽입
        let mut current = last + q - 1;
        let mut target = last + q + r;

        for ins_idx in (0..=r).rev() {
            let v_insert = new_knots[ins_idx];

            while v_insert <= orig_v[current] && current > first {
                for u in 0..old_nu {
                    let src = original.ctrl[u + old_nu * (current - q - 1)];
                    new_ctrl[u + old_nu * (target - q - 1)] = src;
                }
                new_v[target] = orig_v[current];
                target -= 1;
                current -= 1;
            }

            // 한 줄 복사 (초기값)
            for u in 0..old_nu {
                let tmp = new_ctrl[u + old_nu * (target - q)];
                new_ctrl[u + old_nu * (target - q - 1)] = tmp;
            }

            // 차수 q 만큼 블렌딩
            for j in 1..=q {
                let idx = target - q + j;
                let denom = new_v[target + j] - v_insert;
                if denom.abs() < 1e-12 {
                    for u in 0..old_nu {
                        let tmp = new_ctrl[u + old_nu * idx];
                        new_ctrl[u + old_nu * (idx - 1)] = tmp;
                    }
                } else {
                    let denom2 = new_v[target + j] - orig_v[current - q + j];
                    let alpha = if denom2.abs() > 1e-12 {
                        denom / denom2
                    } else {
                        0.0
                    };
                    for u in 0..old_nu {
                        let a = new_ctrl[u + old_nu * (idx - 1)];
                        let b = new_ctrl[u + old_nu * idx];
                        new_ctrl[u + old_nu * (idx - 1)] = Point4D::h_lerp(&a, &b, 1.0 - alpha);
                    }
                }
            }

            new_v[target] = v_insert;
            target -= 1;
        }

        self.ctrl = new_ctrl;
        self.nv = new_nv as Index;
        self.kv =
            KnotVector::new(new_v).expect("insert_knot_v: new knot vector is not non-decreasing");

        r + 1
    }
}
```
```rust
impl NurbsSurface {
    /// U 방향 분할: 현재 row-major ctrl(u + nu * v) 형식에 맞춘 버전
    pub fn split_u(&self, t: Real) -> (NurbsSurface, NurbsSurface) {
        let p = self.pu as usize;
        let old_nu = self.nu as usize;
        let old_nv = self.nv as usize;

        let mut temp = self.clone();
        let curr_mult = Self::multiplicity(&temp.ku.knots, t, 1e-12);
        let insert_needed = p.saturating_sub(curr_mult);

        if insert_needed > 0 {
            let rep = std::iter::repeat(t).take(insert_needed).collect::<Vec<_>>();
            temp.insert_knot_u(&rep);
        }

        // 갱신된 nu, ctrl, ku 이후 다시 구함
        let nu = temp.nu as usize;
        let nv = temp.nv as usize;

        let span = on_find_span_index((nu - 1) as Index, temp.pu, t, &temp.ku.knots) as usize;

        let min_s = p as isize;
        let max_s = (nu as isize) - (p as isize) - 1;
        let mut s = (span as isize - p as isize).clamp(min_s, max_s);
        if s < min_s {
            s = min_s;
        }
        let s = s as usize;

        assert!(s + 1 >= p + 1);
        assert!(nu - s >= p + 1);

        // ========== 3) Control point split (row-major) ==========
        let mut ctrl_left = Vec::with_capacity((s + 1) * nv);
        let mut ctrl_right = Vec::with_capacity((nu - s) * nv);

        for v in 0..nv {
            let row_start = v * nu;
            // left rows : u = 0..=s
            for u in 0..=s {
                ctrl_left.push(temp.ctrl[row_start + u]);
            }
            // right rows : share u = s
            for u in s..nu {
                ctrl_right.push(temp.ctrl[row_start + u]);
            }
        }

        let left_cols = s + 1;
        let right_cols = nu - s;

        // ========== 4) Knot-split ==========
        let mid = s + p + 1;

        let expected_left = left_cols + p + 1;
        let expected_right = right_cols + p + 1;

        // left knot vector
        let mut knot_u_left = temp.ku.knots[..=mid].to_vec();
        if knot_u_left.len() < expected_left {
            knot_u_left.resize(expected_left, t);
        }
        for i in (expected_left - (p + 1))..expected_left {
            knot_u_left[i] = t;
        }

        // right knot vector
        let mut knot_u_right = temp.ku.knots[mid..].to_vec();
        if knot_u_right.len() < expected_right {
            let last = *temp.ku.knots.last().unwrap();
            knot_u_right.resize(expected_right, last);
        }
        let cap = (p + 1).min(knot_u_right.len());
        for i in 0..cap {
            knot_u_right[i] = t;
        }

        // ========== 5) Construct new surfaces ==========
        let mut left = temp.clone();
        left.nu = left_cols as Index;
        left.ctrl = ctrl_left;
        left.ku = KnotVector::new(knot_u_left).unwrap();

        let mut right = temp.clone();
        right.nu = right_cols as Index;
        right.ctrl = ctrl_right;
        right.ku = KnotVector::new(knot_u_right).unwrap();

        (left, right)
    }
```
```rust
    /// V 방향 분할 (현재 row-major ctrl 배열 기준)
    pub fn split_v(&self, t: Real) -> (NurbsSurface, NurbsSurface) {
        let q = self.pv as usize;
        let old_nu = self.nu as usize;
        let old_nv = self.nv as usize;

        let mut temp = self.clone();
        let curr_mult = Self::multiplicity(&temp.kv.knots, t, 1e-12);
        let insert_needed = q.saturating_sub(curr_mult);

        if insert_needed > 0 {
            let rep = std::iter::repeat(t).take(insert_needed).collect::<Vec<_>>();
            temp.insert_knot_v(&rep);
        }

        let nu = temp.nu as usize;
        let nv = temp.nv as usize;

        let span = on_find_span_index((nv - 1) as Index, temp.pv, t, &temp.kv.knots) as usize;

        let min_s = q as isize;
        let max_s = (nv as isize) - (q as isize) - 1;
        let mut s = (span as isize - q as isize).clamp(min_s, max_s);
        if s < min_s {
            s = min_s;
        }
        let s = s as usize;

        assert!(s + 1 >= q + 1);
        assert!(nv - s >= q + 1);

        // ========== 3) Control point split (row-major) ==========
        let mut ctrl_bottom = Vec::with_capacity(nu * (s + 1));
        let mut ctrl_top = Vec::with_capacity(nu * (nv - s));

        for v in 0..=s {
            let row_start = v * nu;
            for u in 0..nu {
                ctrl_bottom.push(temp.ctrl[row_start + u]);
            }
        }
        for v in s..nv {
            let row_start = v * nu;
            for u in 0..nu {
                ctrl_top.push(temp.ctrl[row_start + u]);
            }
        }

        let bottom_rows = s + 1;
        let top_rows = nv - s;

        // ========== 4) Knot split ==========
        let mid = s + q + 1;

        let expected_bottom = bottom_rows + q + 1;
        let expected_top = top_rows + q + 1;

        // bottom knot vector
        let mut knot_v_bottom = temp.kv.knots[..=mid].to_vec();
        if knot_v_bottom.len() < expected_bottom {
            knot_v_bottom.resize(expected_bottom, t);
        }
        for i in (expected_bottom - (q + 1))..expected_bottom {
            knot_v_bottom[i] = t;
        }

        // top knot vector
        let mut knot_v_top = temp.kv.knots[mid..].to_vec();
        if knot_v_top.len() < expected_top {
            let last = *temp.kv.knots.last().unwrap();
            knot_v_top.resize(expected_top, last);
        }
        let cap = (q + 1).min(knot_v_top.len());
        for i in 0..cap {
            knot_v_top[i] = t;
        }

        // ========== 5) Construct new surfaces ==========
        let mut bottom = temp.clone();
        bottom.nv = bottom_rows as Index;
        bottom.ctrl = ctrl_bottom;
        bottom.kv = KnotVector::new(knot_v_bottom).unwrap();

        let mut top = temp.clone();
        top.nv = top_rows as Index;
        top.ctrl = ctrl_top;
        top.kv = KnotVector::new(knot_v_top).unwrap();

        (bottom, top)
    }
}
```
```rust
impl NurbsSurface {
    pub fn new_basic_patch() -> Self {
        let pu = 3;
        let pv = 3;

        let nu = 4;
        let nv = 4;

        // flat row-major ctrl
        let mut ctrl = Vec::with_capacity(nu * nv);
        for v in 0..nv {
            for u in 0..nu {
                ctrl.push(Point4D {
                    x: u as f64 / 3.0,
                    y: v as f64 / 3.0,
                    z: 0.0,
                    w: 1.0,
                });
            }
        }

        let ku = KnotVector::new(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]).unwrap();
        let kv = KnotVector::new(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]).unwrap();

        let mut s = NurbsSurface {
            pu,
            pv,
            nu: nu as Index,
            nv: nv as Index,
            ctrl,
            ku,
            kv,
        };
        s
    }
}
```
```rust
impl NurbsSurface {
    /// SkinU + splitAtCorners:
    ///   - strips: U 방향 프로파일 곡선 샘플들
    ///   - pu,pv : 차수
    ///   - angle_deg_threshold: 코너 감지 각도 (deg)
    ///   - min_edge_length    : 코너 감지에 사용할 최소 segment 길이
    ///
    /// 결과:
    ///   - 각 corner 구간마다 하나씩 NurbsSurface를 만들어 Vec으로 반환
    pub fn skin_u_with_corner_split_from_point_strips(
        strips: &[Vec<Point3D>],
        pu: Degree,
        pv: Degree,
        angle_deg_threshold: Real,
        min_edge_length: Real,
    ) -> Result<Vec<NurbsSurface>> {
        if strips.is_empty() {
            return Ok(Vec::new());
        }

        // 1) 모든 strip 기준으로 corner index 수집
        let corner_indices =
            on_collect_corner_indices_from_strips(strips, angle_deg_threshold, min_edge_length);

        // corner 없으면 한 번에 만든 surface 하나만 반환
        if corner_indices.is_empty() {
            let srf = NurbsSurface::skin_u_from_point_strips(strips, pu, pv)?;
            return Ok(vec![srf]);
        }

        // 2) corner index 로 전체 strip 을 segment 별로 분할
        let segments = on_split_point_strips_by_indices(strips, &corner_indices);

        // 3) 각 segment 에 대해 SkinU 수행
        let mut result = Vec::with_capacity(segments.len());
        for seg_strips in segments {
            if seg_strips.is_empty() {
                continue;
            }
            // 너무 짧은 segment는 건너뛴다
            if seg_strips[0].len() < 2 {
                continue;
            }
            let srf = NurbsSurface::skin_u_from_point_strips(&seg_strips, pu, pv)?;
            result.push(srf);
        }

        Ok(result)
    }
```
```rust
    /// SkinV + splitAtCorners:
    ///   - strips: V 방향 프로파일 곡선 샘플들
    pub fn skin_v_with_corner_split_from_point_strips(
        strips: &[Vec<Point3D>],
        pu: Degree,
        pv: Degree,
        angle_deg_threshold: Real,
        min_edge_length: Real,
    ) -> Result<Vec<NurbsSurface>> {
        if strips.is_empty() {
            return Ok(Vec::new());
        }

        let corner_indices =
            on_collect_corner_indices_from_strips(strips, angle_deg_threshold, min_edge_length);

        if corner_indices.is_empty() {
            let srf = NurbsSurface::skin_v_from_point_strips(strips, pu, pv)?;
            return Ok(vec![srf]);
        }

        let segments = on_split_point_strips_by_indices(strips, &corner_indices);

        let mut result = Vec::with_capacity(segments.len());
        for seg_strips in segments {
            if seg_strips.is_empty() {
                continue;
            }
            if seg_strips[0].len() < 2 {
                continue;
            }
            let srf = NurbsSurface::skin_v_from_point_strips(&seg_strips, pu, pv)?;
            result.push(srf);
        }

        Ok(result)
    }
}
```
```rust
impl NurbsSurface {
    /// Loft surface 생성:
    ///
    /// - sections[j][i]:
    ///     j = 0..nv-1 : 단면 곡선 index (Loft 방향, V 방향)
    ///     i = 0..nu-1 : 단면 곡선 위의 파라미터 샘플 (U 방향)
    ///
    /// - pu : U 방향 차수 (단면 곡선 파라미터 방향)
    /// - pv : V 방향 차수 (Loft 방향)
    ///
    /// 내부적으로는 point grid 를 만들어
    ///   local_interpolation_from_grid(&pts, nu, nv, pu, pv)
    /// 을 호출한다.
    pub fn loft_from_point_sections(
        sections: &[Vec<Point3D>],
        pu: Degree,
        pv: Degree,
    ) -> Result<Self> {
        if sections.is_empty() {
            return Err(NurbsError::InvalidSize);
        }

        let nv = sections.len(); // Loft 방향(섹션 개수)
        let nu = sections[0].len(); // 각 섹션의 샘플 수 (U 방향)
        if nu == 0 {
            return Err(NurbsError::InvalidSize);
        }

        // 모든 섹션의 샘플 수가 동일한지 확인
        for sec in sections.iter() {
            if sec.len() != nu {
                return Err(NurbsError::InvalidSize);
            }
        }

        if (pu as usize) >= nu || (pv as usize) >= nv {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, pv),
                max: MAX_DEGREE as Degree,
            });
        }

        // LocalInterpolation 이 기대하는 row-major (i + nu*j) 형태:
        //   i: U 인덱스 (섹션 위의 파라미터 샘플)
        //   j: V 인덱스 (섹션 index)
        //
        // sections[j][i] 가 (i,j)에 해당.
        let mut pts: Vec<Point3D> = Vec::with_capacity(nu * nv);
        pts.resize(nu * nv, Point3D::default());

        for j in 0..nv {
            for i in 0..nu {
                let p = sections[j][i];
                let idx = i + nu * j;
                pts[idx] = p;
            }
        }

        NurbsSurface::local_interpolation_from_grid(&pts, nu, nv, pu, pv)
    }
```
```rust
    pub fn ruled_from_two_point_sections(
        sec0: &[Point3D],
        sec1: &[Point3D],
        pu: Degree,
    ) -> Result<Self> {
        if sec0.len() == 0 || sec0.len() != sec1.len() {
            return Err(NurbsError::InvalidSize);
        }

        let nu = sec0.len();
        if (pu as usize) >= nu {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, 1),
                max: MAX_DEGREE as Degree,
            });
        }

        // sections[0] = 첫 단면, sections[1] = 두 번째 단면
        let sections = vec![sec0.to_vec(), sec1.to_vec()];

        // V 방향 차수는 1 (선형) 로 고정 → Ruled
        let pv: Degree = 1;

        NurbsSurface::loft_from_point_sections(&sections, pu, pv)
    }
}
```
```rust
impl NurbsSurface {
    /// Loft + 옵션(평활화, end tangent 자연 정렬, corner split) 지원 버전.
    ///
    /// - sections[j][i]:
    ///     j = Loft 방향 섹션 index
    ///     i = 섹션 위 파라미터 샘플 index
    ///
    /// - pu, pv : U, V 차수
    /// - opts   : LoftOptions
    ///
    /// corner_split 이 true 이면, 결과는 여러 개 surface 로 쪼개져서 반환된다.
    /// corner_split 이 false 이면, 길이 1 의 Vec 에 하나만 들어간다.
    pub fn loft_with_options_from_point_sections(
        sections: &[Vec<Point3D>],
        pu: Degree,
        pv: Degree,
        opts: &LoftOptions,
    ) -> Result<Vec<NurbsSurface>> {
        if sections.is_empty() {
            return Ok(Vec::new());
        }

        // 원본을 보존하기 위해 로컬 복사
        let mut work_sections = sections.to_vec();

        let nv = work_sections.len();
        let nu = work_sections[0].len();
        if nu == 0 {
            return Err(NurbsError::InvalidSize);
        }
        for s in work_sections.iter() {
            if s.len() != nu {
                return Err(NurbsError::InvalidSize);
            }
        }
        if (pu as usize) >= nu || (pv as usize) >= nv {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, pv),
                max: MAX_DEGREE as Degree,
            });
        }

        // 1) Loft 방향 smoothing (end tangent 자연 정렬 포함)
        if opts.smooth_loft_dir {
            on_smooth_sections_along_loft_direction(
                &mut work_sections,
                opts.smooth_iterations,
                opts.smooth_lambda,
            );
        }

        // 2) corner split 여부 확인
        if !opts.corner_split {
            // split 없이 한 번에 Loft
            let srf = NurbsSurface::loft_from_point_sections(&work_sections, pu, pv)?;
            return Ok(vec![srf]);
        }

        // 3) corner index 수집
        let corner_indices = on_collect_corner_indices_along_sections(
            &work_sections,
            opts.corner_angle_deg,
            opts.corner_min_edge_length,
        );

        if corner_indices.is_empty() {
            // corner 없음 → 한 번에 Loft
            let srf = NurbsSurface::loft_from_point_sections(&work_sections, pu, pv)?;
            return Ok(vec![srf]);
        }

        // 4) 섹션 분할 후, 각 segment 에 대해 Loft
        let segments = on_split_sections_by_indices(&work_sections, &corner_indices);
        let mut surfaces = Vec::with_capacity(segments.len());

        for seg_secs in segments {
            if seg_secs.is_empty() {
                continue;
            }
            if seg_secs[0].len() < 2 {
                continue;
            }
            let srf = NurbsSurface::loft_from_point_sections(&seg_secs, pu, pv)?;
            surfaces.push(srf);
        }

        Ok(surfaces)
    }
}
```
```rust
impl NurbsSurface {
    /// 4개의 코너 점으로부터 bilinear planar NURBS 표면 생성
    ///
    /// p00 ---- p10
    ///  |        |
    /// p01 ---- p11
    ///
    /// - U, V 차수 = 1
    /// - U, V knot = [0, 0, 1, 1]
    pub fn planar_from_corners(p00: Point3D, p10: Point3D, p01: Point3D, p11: Point3D) -> Self {
        let pu: Degree = 1;
        let pv: Degree = 1;

        let nu: Index = 2; // control count in U
        let nv: Index = 2; // control count in V

        // row-major: idx = i + nu * j
        let mut ctrl = Vec::with_capacity((nu * nv) as usize);

        // w=1인 비유리 NURBS
        ctrl.push(Point4D::new(p00.x, p00.y, p00.z, 1.0)); // (0,0)
        ctrl.push(Point4D::new(p10.x, p10.y, p10.z, 1.0)); // (1,0)
        ctrl.push(Point4D::new(p01.x, p01.y, p01.z, 1.0)); // (0,1)
        ctrl.push(Point4D::new(p11.x, p11.y, p11.z, 1.0)); // (1,1)

        // clamped uniform [0,0,1,1]
        let ku = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };
        let kv = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };

        NurbsSurface {
            pu,
            pv,
            nu,
            nv,
            ctrl,
            ku,
            kv,
        }
    }
}
```
```rust
impl NurbsSurface {
    /// (u,v)에서의 점, U/V 방향 수치 미분 tangent, 법선 벡터를 계산
    fn eval_frame_numeric(&self, u: Real, v: Real) -> (Point3D, Vector3D, Vector3D, Vector3D) {
        let ku = &self.ku.knots;
        let kv = &self.kv.knots;

        let umin = ku[0];
        let umax = ku[ku.len() - 1];
        let vmin = kv[0];
        let vmax = kv[kv.len() - 1];

        // domain 길이에 비례하는 step
        let du_base = (umax - umin).abs().max(1.0) * 1e-4;
        let dv_base = (vmax - vmin).abs().max(1.0) * 1e-4;

        let mut du = du_base;
        let mut dv = dv_base;

        // 경계 근처이면 한쪽 차분으로 처리
        let u_left = (u - du).max(umin);
        let u_right = (u + du).min(umax);
        du = (u_right - u_left).max(1e-8);

        let v_bottom = (v - dv).max(vmin);
        let v_top = (v + dv).min(vmax);
        dv = (v_top - v_bottom).max(1e-8);

        let p = self.point_at(u, v);

        // U 방향 수치 미분
        let p_u1 = self.point_at(u_right, v);
        let p_u0 = self.point_at(u_left, v);
        let su = Vector3D::new(
            (p_u1.x - p_u0.x) / (u_right - u_left),
            (p_u1.y - p_u0.y) / (u_right - u_left),
            (p_u1.z - p_u0.z) / (u_right - u_left),
        );

        // V 방향 수치 미분
        let p_v1 = self.point_at(u, v_top);
        let p_v0 = self.point_at(u, v_bottom);
        let sv = Vector3D::new(
            (p_v1.x - p_v0.x) / (v_top - v_bottom),
            (p_v1.y - p_v0.y) / (v_top - v_bottom),
            (p_v1.z - p_v0.z) / (v_top - v_bottom),
        );

        // 법선 = Su × Sv
        let mut n = su.cross(&sv);
        if !n.normalize() {
            // 퇴화된 경우 대충 (0,0,1) 같은 기본 방향 사용
            n = Vector3D::world_z();
        }

        (p, su, sv, n)
    }
```
```rust
    /// Offset Surface용 U/V 파라미터 리스트 생성
    ///
    /// - knot: KnotVector.knots (superfluous 포함)
    /// - degree: 해당 방향 차수
    /// - (d0, d1): param domain (보통 knot[p]..knot[n+1])
    fn build_offset_param_vector(knot: &[Real], degree: usize, d0: Real, d1: Real) -> Vec<Real> {
        let mut params = Vec::new();
        params.push(d0);

        let n_knots = knot.len();

        // 내부 knot들을 돌면서 "길이가 거의 0이 아닌" 구간의 시작점 사용
        let eps = 1e-12;
        let start = degree + 1;
        let end = n_knots.saturating_sub(degree + 1);

        for i in start..end {
            let delta = knot[i + 1] - knot[i];
            if delta.abs() > eps {
                params.push(knot[i]);

                // 같은 값이 반복되는 구간은 한 번만 사용
                let mut j = i + 1;
                while j + 1 < n_knots && (knot[j + 1] - knot[j]).abs() < eps {
                    j += 1;
                }
            }
        }

        // 내부 구간이 너무 적으면 3등분 샘플 추가
        if params.len() < 3 {
            let len = d1 - d0;
            let step = len / 3.0;
            params.push(d0 + step);
            params.push(d0 + 2.0 * step);
        }

        // 끝점 추가 및 정렬
        params.push(d1);
        params.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        params
    }
```
```rust
    /// Offset NURBS Surface 생성
    ///
    /// - `amount` : offset 거리 (법선 방향으로 +amount)
    /// - `tol`    : (향후 모양 보정/리파인에 쓸 수 있는 허용 오차, 현재는 인터페이스용)
    ///
    /// 내부 동작:
    ///   1) U/V 방향 offset용 파라미터 그리드 생성 (knot 기반)
    ///   2) 각 (u,v)에서 점+법선을 수치적으로 구해 p_off = p + amount * n
    ///   3) 해당 point grid를 global interpolation 로 다시 NURBS surface 로 근사
    pub fn build_offset_surface(&self, amount: Real, _tol: Real) -> Result<NurbsSurface> {
        // 1) domain 추출
        let u_knots = &self.ku.knots;
        let v_knots = &self.kv.knots;

        if u_knots.len() < 2 || v_knots.len() < 2 {
            return Err(NurbsError::InvalidSize);
        }

        let umin = u_knots[0];
        let umax = u_knots[u_knots.len() - 1];
        let vmin = v_knots[0];
        let vmax = v_knots[v_knots.len() - 1];

        // 2) U/V 파라미터 리스트 생성
        let u_params = Self::build_offset_param_vector(u_knots, self.pu as usize, umin, umax);
        let v_params = Self::build_offset_param_vector(v_knots, self.pv as usize, vmin, vmax);

        let nu_s = u_params.len();
        let nv_s = v_params.len();
        if nu_s < 2 || nv_s < 2 {
            return Err(NurbsError::InvalidSize);
        }

        // 3) offset point grid 계산 (row-major: i + nu_s * j)
        let mut pts: Vec<Point3D> = Vec::with_capacity(nu_s * nv_s);

        for j in 0..nv_s {
            let v = v_params[j];
            for i in 0..nu_s {
                let u = u_params[i];

                let (p, _su, _sv, n) = self.eval_frame_numeric(u, v);
                let p_off = Point3D {
                    x: p.x + amount * n.x,
                    y: p.y + amount * n.y,
                    z: p.z + amount * n.z,
                };
                pts.push(p_off);
            }
        }

        // 4) grid 보간으로 NURBS offset surface 생성
        //
        //   nu_s: U 방향 샘플 개수
        //   nv_s: V 방향 샘플 개수
        //
        // 차수는 원본과 동일하게 사용 (원하시면 여기서 따로 지정 가능)
        let pu = self.pu;
        let pv = self.pv;

        // 이미 구현해 둔 "global interpolation from grid" 함수 사용
        // (이름은 예시입니다. 실제 구현 함수 이름에 맞춰 수정하세요)
        let srf = NurbsSurface::local_interpolation_from_grid(
            &pts,
            nu_s as Index,
            nv_s as Index,
            pu,
            pv,
        )?;

        Ok(srf)
    }

    pub fn offset(&self, amount: Real, tol: Real) -> Result<NurbsSurface> {
        self.build_offset_surface(amount, tol)
    }
}
```
```rust
impl NurbsSurface {
    /// Sweep Surface:
    ///
    /// - `section[i]` : 초기 단면 곡선의 샘플 점들 (U 방향)
    /// - `path[j]`    : sweep 경로의 샘플 점들      (V 방향)
    ///
    /// - `pu` : U 방향 차수  (단면 파라미터 방향)
    /// - `pv` : V 방향 차수  (경로 방향)
    ///
    /// 내부적으로:
    ///   1) path 에 대한 프레임(T,N,B) 생성
    ///   2) 단면을 초기 프레임 기준 로컬 좌표(sx,sy,sz)로 투영
    ///   3) 각 path station j 에서:
    ///        P_j + sx_i * B_j + sy_i * N_j + sz_i * T_j
    ///      로 grid 점 생성
    ///   4) 해당 grid 를 LocalInterpolation 으로 보간해서 NURBS surface 생성
    pub fn sweep_from_section_and_path(
        section: &[Point3D],
        path: &[Point3D],
        pu: Degree,
        pv: Degree,
    ) -> Result<NurbsSurface> {
        let nu = section.len();
        let nv = path.len();

        if nu < 2 || nv < 2 {
            return Err(NurbsError::InvalidSize);
        }

        if (pu as usize) >= nu || (pv as usize) >= nv {
            return Err(NurbsError::InvalidDegreeSurface {
                got: (pu, pv),
                max: crate::core::types::MAX_DEGREE as Degree,
            });
        }

        // 1) path 프레임들 구축
        let frames = on_build_sweep_frames(path).ok_or(NurbsError::InvalidSize)?;

        // 2) 초기 프레임 (j=0) 기준 단면 로컬 좌표 계산
        let (p0, t0, n0, b0) = frames[0];
        let local_section = on_build_section_local_coords(section, p0, t0, n0, b0);

        // 3) path 따라 sweep된 point grid 생성 (row-major: i + nu * j)
        let mut pts: Vec<Point3D> = Vec::with_capacity(nu * nv);

        for j in 0..nv {
            let (pj, tj, nj, bj) = frames[j];

            for i in 0..nu {
                let (sx, sy, sz) = local_section[i];

                // P_j + sx * B_j + sy * N_j + sz * T_j
                let offset_vec = bj * sx + nj * sy + tj * sz;

                let p = pj + offset_vec.to_point();
                pts.push(p);
            }
        }

        // 4) LocalInterpolation 으로 NURBS surface 근사
        //
        //    U 방향: section 파라미터
        //    V 방향: path 파라미터
        //
        // local_interpolation_from_grid(
        //   pts: &[Point3D], nu: usize, nv: usize, pu: Degree, pv: Degree
        // )
        let srf = NurbsSurface::local_interpolation_from_grid(&pts, nu, nv, pu, pv)?;

        Ok(srf)
    }
}
```
```rust
impl NurbsSurface {
    /// knot 벡터에서 "길이가 있는" 구간(span)들을 뽑아낸다.
    ///
    /// - `knots`: KnotVector.knots
    /// - `eps`  : 0으로 간주할 최소 길이 (보통 1e-12 정도)
    ///
    /// 예:
    ///   knots = [0,0,0, 0.5, 1,1,1]
    ///   → span 0: [0, 0.5], span 1: [0.5, 1]
    fn collect_knot_spans(knots: &[Real], eps: Real) -> Vec<KnotSpan> {
        let mut spans = Vec::new();
        let n = knots.len();
        if n < 2 {
            return spans;
        }

        let mut i = 0usize;
        while i + 1 < n {
            let t0 = knots[i];
            let mut j = i + 1;

            // t0 와 거의 같은 knot들은 건너뛰고,
            // 처음으로 다른 값이 나오는 위치를 knot_index1 로 사용
            while j < n && (knots[j] - t0).abs() <= eps {
                j += 1;
            }
            if j >= n {
                break;
            }

            let t1 = knots[j];
            if (t1 - t0).abs() > eps {
                spans.push(KnotSpan {
                    knot_index0: i,
                    knot_index1: j,
                    t0,
                    t1,
                });
            }

            i = j;
        }

        spans
    }
```
```rust
    /// 현재 NurbsSurface를 U/V knot span 에 따라 Patch 들로 분해한다.
    ///
    /// - `eps` : knot 구간 길이를 판단할 때 사용할 tolerance
    ///
    /// 반환:
    ///   Vec<SurfacePatch>  (각 Patch 는 UV 사각형 + span/인덱스 정보 포함)
    pub fn decompose_into_patches(&self, eps: Real) -> Vec<SurfacePatch> {
        let u_knots = &self.ku.knots;
        let v_knots = &self.kv.knots;

        if u_knots.len() < 2 || v_knots.len() < 2 {
            return Vec::new();
        }

        // U, V 방향 span 수집
        let u_spans = Self::collect_knot_spans(u_knots, eps);
        let v_spans = Self::collect_knot_spans(v_knots, eps);

        if u_spans.is_empty() || v_spans.is_empty() {
            return Vec::new();
        }

        let mut patches = Vec::with_capacity(u_spans.len() * v_spans.len());

        for (iu, su) in u_spans.iter().enumerate() {
            for (iv, sv) in v_spans.iter().enumerate() {
                let rect = Rectangle {
                    ul: su.t0,
                    ur: su.t1,
                    vb: sv.t0,
                    vt: sv.t1,
                };

                patches.push(SurfacePatch {
                    rect,
                    u_span_index: iu,
                    v_span_index: iv,
                    u_knot_index0: su.knot_index0,
                    u_knot_index1: su.knot_index1,
                    v_knot_index0: sv.knot_index0,
                    v_knot_index1: sv.knot_index1,
                });
            }
        }

        patches
    }
```
```rust
    /// Patch 의 로컬 파라미터 (0..1, 0..1) 를 global UV 로 맵핑해서 평가하는 편의 함수
    pub fn eval_on_patch(&self, patch: &SurfacePatch, u_hat: Real, v_hat: Real) -> Point3D {
        let u = patch.rect.ul + (patch.rect.ur - patch.rect.ul) * u_hat;
        let v = patch.rect.vb + (patch.rect.vt - patch.rect.vb) * v_hat;
        self.point_at(u, v)
    }
}
```
```rust
impl NurbsSurface {
    /// U 방향 파라미터 도메인 (현재 구현에서는 knot[0]..knot[last])
    pub fn domain_u(&self) -> (Real, Real) {
        let ku = &self.ku.knots;
        (ku[0], *ku.last().unwrap())
    }

    /// V 방향 파라미터 도메인
    pub fn domain_v(&self) -> (Real, Real) {
        let kv = &self.kv.knots;
        (kv[0], *kv.last().unwrap())
    }
```
```rust
    /// Point inversion: 주어진 3D 점 p 에 가장 가까운 (u,v) 를 찾는다.
    ///
    /// - p            : 대상 점
    /// - u0, v0       : 초기 추정값 (대략적인 위치)
    /// - allow_outside: true 이면 도메인 바깥으로 나가는 것도 허용,
    ///                  false 이면 매 반복마다 도메인으로 clamp
    /// - tol          : 거리 허용 오차 (예: 1e-9 ~ 1e-6)
    /// - max_iter     : 최대 반복 횟수
    ///
    /// 반환:
    ///   (성공 여부, u, v, closest_point, deviation_vec)
    ///   deviation_vec = closest_point - p
    pub fn point_inversion(
        &self,
        p: Point3D,
        mut u0: Real,
        mut v0: Real,
        allow_outside: bool,
        tol: Real,
        max_iter: usize,
    ) -> (bool, Real, Real, Point3D, Vector3D) {
        let (u_min, u_max) = self.domain_u();
        let (v_min, v_max) = self.domain_v();

        // 초기값 도메인 클램프
        if !allow_outside {
            u0 = u0.clamp(u_min, u_max);
            v0 = v0.clamp(v_min, v_max);
        }

        let mut u = u0;
        let mut v = v0;

        // 도메인 크기에 비례해서 파라미터 변화량 기준을 잡기 위해
        let du_range = (u_max - u_min).abs().max(1.0);
        let dv_range = (v_max - v_min).abs().max(1.0);
        let param_tol = 1e-10 * (du_range + dv_range + 1.0);

        let mut last_q = self.point_at(u, v);
        let mut last_dev = last_q - p;

        for _iter in 0..max_iter {
            // 1) 현재 (u,v) 에서 위치, 1차 도함수 평가
            let (s, su, sv, _) = self.eval_frame_numeric(u, v);
            let r = s - p;
            let dist2 = r.length_squared();

            // 2) 거리 기반 수렴 체크
            if dist2 <= tol * tol {
                return (true, u, v, s, r.to_vector());
            }

            // 3) J^T J * [du dv]^T = - J^T r  (2x2 normal equation)
            let a11 = su.dot(&su);
            let a12 = su.dot(&sv);
            let a22 = sv.dot(&sv);
            let b1 = -su.dot(&r.to_vector());
            let b2 = -sv.dot(&r.to_vector());

            // 아주 약한 damping (Levenberg-Marquardt 스타일)
            let lambda = 1e-12 * (a11 + a22 + 1.0);
            let a11d = a11 + lambda;
            let a22d = a22 + lambda;

            let det = a11d * a22d - a12 * a12;

            let (du, dv) = if det.abs() <= 1e-20 {
                // Jacobian 이 너무 퇴화 -> -grad 방향으로 작은 스텝 (gradient descent fallback)
                let grad_u = b1;
                let grad_v = b2;
                let grad_norm = (grad_u * grad_u + grad_v * grad_v).sqrt();
                if grad_norm <= 1e-30 {
                    // 더 이동할 의미 없음
                    break;
                }
                let step_len = 0.1 * (du_range.max(dv_range));
                let scale = step_len / grad_norm;
                (grad_u * scale, grad_v * scale)
            } else {
                // 표준 2x2 해
                let inv_det = 1.0 / det;
                let du = (a22d * b1 - a12 * b2) * inv_det;
                let dv = (-a12 * b1 + a11d * b2) * inv_det;
                (du, dv)
            };

            // 4) 너무 큰 스텝은 줄이기 (도메인의 절반 이상은 안 가도록)
            let step_norm = (du * du + dv * dv).sqrt();
            if step_norm > 0.5 * (du_range.max(dv_range)) {
                let scale = 0.5 * (du_range.max(dv_range)) / step_norm;
                u += du * scale;
                v += dv * scale;
            } else {
                u += du;
                v += dv;
            }

            if !allow_outside {
                u = u.clamp(u_min, u_max);
                v = v.clamp(v_min, v_max);
            }

            // 5) 파라미터 변화량 기반 수렴 체크
            if du.abs() <= param_tol && dv.abs() <= param_tol {
                // 한 번 더 위치를 계산해보고 거리도 확인
                let q = self.point_at(u, v);
                let d = q - p;
                if d.length_squared() <= tol * tol {
                    return (true, u, v, q, d.to_vector());
                } else {
                    last_q = q;
                    last_dev = d;
                    break;
                }
            }

            last_q = s;
            last_dev = r;
        }

        // 여기까지 오면 수렴 실패. 그래도 마지막 상태를 돌려준다.
        (false, u, v, last_q, last_dev.to_vector())
    }
}
```
```rust
impl NurbsSurface {
    /// U = const 인 Iso curve 를 V 방향으로 샘플링
    ///
    /// - u         : 고정할 U 파라미터
    /// - n_samples : 샘플 개수 (>= 2)
    ///
    /// 반환: Vec<Point3D>  (V 방향으로 늘어선 점들)
    pub fn sample_iso_curve_u(&self, mut u: Real, n_samples: usize) -> Vec<Point3D> {
        if n_samples < 2 {
            return Vec::new();
        }

        let (umin, umax) = self.domain_u();
        u = u.clamp(umin, umax);

        let (vmin, vmax) = self.domain_v();

        // n_samples 개의 점 → n_samples-1 개의 세그먼트
        let v_params = on_uniform_params_in_range(vmin, vmax, n_samples - 1);

        let mut pts = Vec::with_capacity(n_samples);
        for v in v_params {
            pts.push(self.point_at(u, v));
        }
        pts
    }
```
```rust
    /// V = const 인 Iso curve 를 U 방향으로 샘플링
    pub fn sample_iso_curve_v(&self, mut v: Real, n_samples: usize) -> Vec<Point3D> {
        if n_samples < 2 {
            return Vec::new();
        }

        let (vmin, vmax) = self.domain_v();
        v = v.clamp(vmin, vmax);

        let (umin, umax) = self.domain_u();

        let u_params = on_uniform_params_in_range(umin, umax, n_samples - 1);

        let mut pts = Vec::with_capacity(n_samples);
        for u in u_params {
            pts.push(self.point_at(u, v));
        }
        pts
    }
}
```
```rust
impl NurbsSurface {
    /// U = const 인 NURBS IsoCurve 데이터 (V 방향 곡선)
    ///
    /// - u : 고정할 U 파라미터
    ///
    /// 반환:
    ///   degree = pv
    ///   ctrl   = V 방향 제어점들 (길이 = nv)
    ///   knot   = kv (V 방향 knot 벡터 그대로)
    pub fn iso_curve_u_data(&self, mut u: Real) -> Result<NurbsIsoCurveData> {
        let pu = self.pu as usize;
        let pv = self.pv as usize;

        let nu_pts = self.nu as usize;
        let nv_pts = self.nv as usize;
        if nu_pts == 0 || nv_pts == 0 {
            return Err(NurbsError::InvalidSize);
        }

        // U 도메인 clamp
        let ku = &self.ku.knots;
        let (umin, umax) = (ku[0], *ku.last().unwrap());
        u = u.clamp(umin, umax);

        // V 방향은 곡선의 parameter 방향 → degree = pv, knot = kv
        let degree_v = self.pv;
        let knot_v = self.kv.clone();

        // U 방향에서 basis N_i^p(u) 계산
        let nu = nu_pts - 1;
        let su = on_find_span_index(nu, pu as u16, u, &self.ku.knots);

        let mut n_u = vec![0.0; pu + 1];
        on_basis_func(su, u, pu as u16, &self.ku.knots, &mut n_u);

        let iu0 = (su - pu) as usize;
        let nu_usize = nu_pts;

        let mut ctrl_iso: Vec<Point4D> = Vec::with_capacity(nv_pts);

        // j = 0..nv_pts-1: 각 V index에 대해 control point 계산
        for j in 0..nv_pts {
            let mut xw = 0.0;
            let mut yw = 0.0;
            let mut zw = 0.0;
            let mut w = 0.0;

            // U 방향 basis N_i^p(u) 를 사용해서 homogeneous control point 합성
            for k in 0..n_u.len() {
                let i = iu0 + k;
                if i >= nu_usize {
                    continue;
                }
                let cp = self.ctrl[Self::idx_row_major(nu_usize, i, j)];
                let Nk = n_u[k];

                // homogeneous : (x*w, y*w, z*w, w)
                xw += Nk * (cp.x * cp.w);
                yw += Nk * (cp.y * cp.w);
                zw += Nk * (cp.z * cp.w);
                w += Nk * cp.w;
            }

            if w.abs() > 0.0 {
                let x = xw / w;
                let y = yw / w;
                let z = zw / w;
                ctrl_iso.push(Point4D { x, y, z, w });
            } else {
                // 완전히 퇴화된 경우: 비합리로 간주 (w=1) 하고 좌표만 사용
                ctrl_iso.push(Point4D {
                    x: xw,
                    y: yw,
                    z: zw,
                    w: 1.0,
                });
            }
        }

        Ok(NurbsIsoCurveData {
            degree: degree_v,
            ctrl: ctrl_iso,
            knot: knot_v,
        })
    }
```
```rust
    pub fn iso_curve_v_data(&self, mut v: Real) -> Result<NurbsIsoCurveData> {
        // 1) V 도메인 클램프
        let (vmin, vmax) = self.domain_v();
        if v < vmin {
            v = vmin;
        } else if v > vmax {
            v = vmax;
        }

        let nu = self.nu as usize;
        let nv = self.nv as usize;
        if nu == 0 || nv == 0 {
            return Err(NurbsError::InvalidArgument(
                "iso_curve_v_curve: surface has no control points".to_string(),
            ));
        }

        // 2) V 방향 basis 함수 계산 (degree = pv, knots = kv)
        let q: Degree = self.pv;
        let span = on_find_span_index((nv - 1) as Index, q, v, &self.kv.knots);
        let qz = q as usize;

        let mut n_vec = vec![0.0; qz + 1];
        on_basis_func(span, v, q, &self.kv.knots, &mut n_vec);

        let span_usize = span as usize;

        // 3) 각 U 인덱스에 대해, V 방향으로 블렌딩해서 4D control point 생성
        let mut curve_ctrl: Vec<Point4D> = Vec::with_capacity(nu);

        for iu in 0..nu {
            let mut pw = Point4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            };

            for k in 0..=qz {
                let jv = span_usize - qz + k; // V 인덱스
                let idx = iu + nu * jv; // row-major: u + nu * v
                let cp = self.ctrl[idx];

                let w = n_vec[k];
                pw.x += w * cp.x;
                pw.y += w * cp.y;
                pw.z += w * cp.z;
                pw.w += w * cp.w;
            }

            curve_ctrl.push(pw);
        }

        // 4) 곡선 degree / knot / domain 은 U 방향 것을 사용
        let curve_degree = self.pu;
        let curve_kv = self.ku.clone();
        let kv = &curve_kv.knots;

        // 표준 clamped B-spline 도메인: [U[p], U[m-p]]
        let pz = curve_degree as usize;
        if kv.len() < 2 * pz + 2 {
            return Err(NurbsError::InvalidArgument(
                "iso_curve_v_curve: invalid U knot vector".to_string(),
            ));
        }
        let t0 = kv[pz];
        let t1 = kv[kv.len() - pz - 1];

        Ok(NurbsIsoCurveData {
            degree: curve_degree,
            ctrl: curve_ctrl,
            knot: curve_kv,
        })
    }
```
```rust
    pub fn from_iso_u(surface: &NurbsSurface, u: Real) -> Result<NurbsCurve> {
        let data = surface.iso_curve_u_data(u)?;
        // 여기에서 실제 프로젝트의 생성자 시그니처에 맞게 수정
        NurbsCurve::new(data.degree, data.ctrl, data.knot)
    }
```
```rust
    pub fn from_iso_v(surface: &NurbsSurface, v: Real) -> Result<NurbsCurve> {
        let data = surface.iso_curve_v_data(v)?;
        NurbsCurve::new(data.degree, data.ctrl, data.knot)
    }
}
```
```rust
impl NurbsSurface {
    /// 점 p 를 Surface 위로 Project.
    ///
    /// - u_hint, v_hint 가 Some 이면 그대로 초기값으로 사용
    /// - None 이면 도메인 중앙값으로 초기값 설정
    /// - allow_outside = false 이면 (u,v)를 항상 도메인 안으로 clamp
    ///
    /// 반환:
    ///   Ok((u,v,q,n))  : 성공 (q = S(u,v), n = 법선 벡터)
    ///   Err(...)       : 수렴 실패
    pub fn project_point_to_surface(
        &self,
        p: Point3D,
        u_hint: Option<Real>,
        v_hint: Option<Real>,
        allow_outside: bool,
        tol: Real,
        max_iter: usize,
    ) -> Result<(Real, Real, Point3D, Vector3D)> {
        let (u_min, u_max) = self.domain_u();
        let (v_min, v_max) = self.domain_v();

        let mut u0 = u_hint.unwrap_or((u_min + u_max) * 0.5);
        let mut v0 = v_hint.unwrap_or((v_min + v_max) * 0.5);

        println!("u0={:.5}, v0={:.5}", u0, v0);

        if !allow_outside {
            u0 = u0.clamp(u_min, u_max);
            v0 = v0.clamp(v_min, v_max);
        }

        println!("u0={:.5}, v0={:.5}", u0, v0);

        let (ok, u, v, q, _dev) = self.point_inversion(p, u0, v0, allow_outside, tol, max_iter);

        println!(" ok = {}, u={:.5}, v={:.5}", ok, u, v);

        if !ok {
            return Err(NurbsError::NoConvergence);
        }

        let (_p, _su, _sv, n) = self.eval_frame_numeric(u, v);
        Ok((u, v, q, n))
    }
}
```
```rust
impl NurbsSurface {
    /// 선분을 Surface 위에 Project 해서, Surface 위 polyline(점 + uv)을 반환.
    ///
    /// - seg        : 원래 3D 선분
    /// - n_samples  : 샘플 개수 (>= 2)
    pub fn project_segment_to_surface(
        &self,
        seg: &Segment3D,
        n_samples: usize,
        allow_outside: bool,
        tol: Real,
        max_iter: usize,
    ) -> Result<Vec<(Real, Real, Point3D)>> {
        if n_samples < 2 {
            return Err(NurbsError::InvalidSize);
        }

        let mut result = Vec::with_capacity(n_samples);

        let dir = seg.p1 - seg.p0;
        let mut u_hint: Option<Real> = None;
        let mut v_hint: Option<Real> = None;

        for i in 0..n_samples {
            // 0..1 사이를 균등 분할
            let t = if n_samples == 1 {
                0.0
            } else {
                i as Real / (n_samples as Real - 1.0)
            };

            let p = seg.p0 + dir * t;

            // 이전 점의 (u,v)를 다음 점의 초기값으로 사용
            // (None 이면 project_point_to_surface 내부에서 기본값을 사용하게 둔다)
            let u0 = u_hint;
            let v0 = v_hint;

            let (u, v, q, _n) =
                self.project_point_to_surface(p, u0, v0, allow_outside, tol, max_iter)?;

            result.push((u, v, q));

            // 다음 샘플을 위한 힌트 업데이트
            u_hint = Some(u);
            v_hint = Some(v);
        }

        Ok(result)
    }
}
```
```rust
impl NurbsSurface {
    /// 균일 Tessellation (U×V 그리드)
    ///
    /// - nu_div, nv_div : U,V 방향 분할 개수 (>= 1)
    ///
    /// 반환: SurfaceMesh
    pub fn tessellate_uniform(&self, nu_div: usize, nv_div: usize) -> Result<SurfaceMesh> {
        if nu_div == 0 || nv_div == 0 {
            return Err(NurbsError::InvalidSize);
        }

        let (u_min, u_max) = self.domain_u();
        let (v_min, v_max) = self.domain_v();

        let mut mesh = SurfaceMesh::new();

        // 1) vertex / uv / normal 생성
        for j in 0..=nv_div {
            let v = if nv_div == 0 {
                v_min
            } else {
                v_min + (v_max - v_min) * (j as Real / nv_div as Real)
            };

            for i in 0..=nu_div {
                let u = if nu_div == 0 {
                    u_min
                } else {
                    u_min + (u_max - u_min) * (i as Real / nu_div as Real)
                };

                let (p, _su, _sv, n) = self.eval_frame_numeric(u, v);

                mesh.vertices.push(p);
                mesh.normals.push(n);
                mesh.uvs.push((u, v));
            }
        }

        // 2) 인덱스 생성 (각 격자를 두 개의 삼각형으로)
        let stride = nu_div + 1;
        for j in 0..nv_div {
            for i in 0..nu_div {
                let i0 = (j * stride + i) as u32;
                let i1 = (j * stride + i + 1) as u32;
                let i2 = ((j + 1) * stride + i + 1) as u32;
                let i3 = ((j + 1) * stride + i) as u32;

                // 삼각형 두 개: (i0, i1, i2), (i0, i2, i3)
                mesh.indices.push([i0, i1, i2]);
                mesh.indices.push([i0, i2, i3]);
            }
        }

        Ok(mesh)
    }
}
```
```rust
impl NurbsSurface {
    /// 한 Quad가 충분히 평탄한지 체크.
    ///
    /// - quad: ParamQuad (4개 corner uv)
    /// - tol : 거리 허용 오차
    ///
    /// 반환: (is_flat, corner_points[4], center_point)
    fn eval_quad_flatness(&self, quad: &ParamQuad, tol: Real) -> (bool, [Point3D; 4], Point3D) {
        let mut pts = [Point3D::default(); 4];
        for k in 0..4 {
            let (u, v) = quad.uv[k];
            pts[k] = self.point_at(u, v);
        }

        // 중앙점 uv : 평균
        let u_c = (quad.uv[0].0 + quad.uv[1].0 + quad.uv[2].0 + quad.uv[3].0) * 0.25;
        let v_c = (quad.uv[0].1 + quad.uv[1].1 + quad.uv[2].1 + quad.uv[3].1) * 0.25;
        let p_c = self.point_at(u_c, v_c);

        // 평면 근사: 4 corner 의 평균 (혹은 bilinear 평균) 사용
        let p_avg = Point3D {
            x: (pts[0].x + pts[1].x + pts[2].x + pts[3].x) * 0.25,
            y: (pts[0].y + pts[1].y + pts[2].y + pts[3].y) * 0.25,
            z: (pts[0].z + pts[1].z + pts[2].z + pts[3].z) * 0.25,
        };

        let dx = p_c.x - p_avg.x;
        let dy = p_c.y - p_avg.y;
        let dz = p_c.z - p_avg.z;
        let dist2 = dx * dx + dy * dy + dz * dz;

        (dist2 <= tol * tol, pts, p_c)
    }
}
```
```rust
impl NurbsSurface {
    /// Quad 하나에 대해:
    ///  - corner 4점과 center 점 평가
    ///  - center vs corner 평균(평면/bi-linear) 거리
    ///  - corner/center 법선 각도
    ///  - edge 길이
    ///
    /// 반환:
    ///   (deviation, max_normal_angle, max_edge_length, corner_points, center_point)
    fn eval_quad_error(&self, quad: &ParamQuad) -> (Real, Real, Real, [Point3D; 4], Point3D) {
        let mut pts = [Point3D::default(); 4];
        let mut ns = [Vector3D::default(); 4];

        for k in 0..4 {
            let (u, v) = quad.uv[k];
            let (p, _su, _sv, n) = self.eval_frame_numeric(u, v);
            pts[k] = p;
            ns[k] = n;
        }

        // center uv / point
        let u_c = (quad.uv[0].0 + quad.uv[1].0 + quad.uv[2].0 + quad.uv[3].0) * 0.25;
        let v_c = (quad.uv[0].1 + quad.uv[1].1 + quad.uv[2].1 + quad.uv[3].1) * 0.25;
        let (p_c, _su_c, _sv_c, n_c) = self.eval_frame_numeric(u_c, v_c);

        // bi-linear 평면 근사: corner 평균 사용
        let p_avg = Point3D {
            x: (pts[0].x + pts[1].x + pts[2].x + pts[3].x) * 0.25,
            y: (pts[0].y + pts[1].y + pts[2].y + pts[3].y) * 0.25,
            z: (pts[0].z + pts[1].z + pts[2].z + pts[3].z) * 0.25,
        };

        let dx = p_c.x - p_avg.x;
        let dy = p_c.y - p_avg.y;
        let dz = p_c.z - p_avg.z;
        let deviation = (dx * dx + dy * dy + dz * dz).sqrt();

        // corner vs center 법선 각도의 최대값
        let mut max_angle = 0.0;
        for k in 0..4 {
            let dot = ns[k].dot(&n_c).clamp(-1.0, 1.0);
            let angle = dot.acos();
            if angle > max_angle {
                max_angle = angle;
            }
        }

        // edge 길이 (3D)
        let mut max_edge_len = 0.0;
        let order = [0, 1, 2, 3, 0];
        for w in order.windows(2) {
            let p0 = pts[w[0]];
            let p1 = pts[w[1]];
            let ex = p1.x - p0.x;
            let ey = p1.y - p0.y;
            let ez = p1.z - p0.z;
            let len = (ex * ex + ey * ey + ez * ez).sqrt();
            if len > max_edge_len {
                max_edge_len = len;
            }
        }

        (deviation, max_angle, max_edge_len, pts, p_c)
    }
```
```rust
    fn subdivide_quad(quad: &ParamQuad) -> [ParamQuad; 4] {
        let (u0, v0) = quad.uv[0];
        let (u1, v1) = quad.uv[2];

        let um = (u0 + u1) * 0.5;
        let vm = (v0 + v1) * 0.5;

        let q0 = ParamQuad {
            uv: [(u0, v0), (um, v0), (um, vm), (u0, vm)],
        };
        let q1 = ParamQuad {
            uv: [(um, v0), (u1, v0), (u1, vm), (um, vm)],
        };
        let q2 = ParamQuad {
            uv: [(um, vm), (u1, vm), (u1, v1), (um, v1)],
        };
        let q3 = ParamQuad {
            uv: [(u0, vm), (um, vm), (um, v1), (u0, v1)],
        };
        [q0, q1, q2, q3]
    }
}
```
```rust
impl NurbsSurface {
    fn should_subdivide_quad(
        &self,
        quad: &ParamQuad,
        depth: usize,
        opts: &AdaptiveTessellationOptions,
    ) -> bool {
        if depth >= opts.max_depth {
            return false;
        }

        let (deviation, angle, max_edge, _pts, _pc) = self.eval_quad_error(quad);

        // 기준값들
        let mut max_dev = opts.max_deviation;
        let mut max_angle = opts.max_normal_angle;

        // 경계/feature 가까우면 기준을 더 엄격하게 (더 많이 subdivide)
        if !opts.boundary_curves.is_empty() || !opts.feature_curves.is_empty() {
            let near_boundary = on_quad_near_feature(quad, &opts.boundary_curves);
            let near_feature = on_quad_near_feature(quad, &opts.feature_curves);
            if near_boundary || near_feature {
                max_dev *= 0.5;
                max_angle *= 0.5;
            }
        }

        // 편차 조건
        if deviation > max_dev {
            return true;
        }

        // 곡률(법선 각도) 조건
        if angle > max_angle {
            return true;
        }

        // edge 길이 조건
        if opts.max_edge_length > 0.0 && max_edge > opts.max_edge_length {
            return true;
        }

        // 너무 작은 edge 길이는 더 이상 subdivide 하지 않도록
        if opts.min_edge_length > 0.0 && max_edge < opts.min_edge_length {
            return false;
        }

        false
    }
```
```rust
    pub fn tessellate_adaptive_with_options(
        &self,
        opts: &AdaptiveTessellationOptions,
    ) -> Result<SurfaceMesh> {
        let (umin, umax) = self.domain_u();
        let (vmin, vmax) = self.domain_v();

        // 초기 Quad = 전체 도메인
        let root = ParamQuad {
            uv: [(umin, vmin), (umax, vmin), (umax, vmax), (umin, vmax)],
        };

        let mut queue: VecDeque<(ParamQuad, usize)> = VecDeque::new();
        queue.push_back((root, 0));

        let mut mesh = SurfaceMesh::new();

        // vertex 중복 허용 (나중에 필요하면 weld 가능)
        while let Some((quad, depth)) = queue.pop_front() {
            if self.should_subdivide_quad(&quad, depth, opts) {
                let children = Self::subdivide_quad(&quad);
                let next_depth = depth + 1;
                for c in children.into_iter() {
                    queue.push_back((c, next_depth));
                }
                continue;
            }

            // leaf quad → 실제 삼각형 2개로 출력
            let base_index = mesh.vertices.len() as u32;

            // corner 순서: (0,0), (1,0), (1,1), (0,1)
            for k in 0..4 {
                let (u, v) = quad.uv[k];
                let (p, _su, _sv, n) = self.eval_frame_numeric(u, v);
                mesh.vertices.push(p);
                mesh.normals.push(n);
                mesh.uvs.push((u, v));
            }

            // 삼각형 두 개: (0,1,2), (0,2,3)
            mesh.indices
                .push([base_index, base_index + 1, base_index + 2]);
            mesh.indices
                .push([base_index, base_index + 2, base_index + 3]);
        }

        Ok(mesh)
    }
```
```rust
    /// 기존 간단버전 tessellate_adaptive 를 대체/래핑해서 기본 옵션으로 호출
    pub fn tessellate_adaptive(
        &self,
        max_deviation: Real,
        max_angle_deg: Real,
        max_depth: usize,
    ) -> Result<SurfaceMesh> {
        let mut opts = AdaptiveTessellationOptions::default();
        opts.max_deviation = max_deviation;
        opts.max_normal_angle = max_angle_deg.to_radians();
        opts.max_depth = max_depth;
        // edge length 기준은 모델 스케일에 따라 필요 시 호출부에서 세팅
        self.tessellate_adaptive_with_options(&opts)
    }
}
```
```rust
fn on_build_skin_u_from_curves<C, FDom, FEval>(
    curves: &[C],
    sample_count: usize,
    pu: Degree,
    pv: Degree,
    get_domain: FDom,
    eval_point: FEval,
) -> crate::core::types::Result<NurbsSurface>
where
    FDom: Fn(&C) -> (Real, Real),
    FEval: Fn(&C, Real) -> Point3D,
{
    let strips =
        NurbsSurface::sample_curves_to_point_strips(curves, sample_count, get_domain, eval_point);
    NurbsSurface::skin_u_from_point_strips(&strips, pu, pv)
}
```
```rust
pub fn on_detect_corners_in_strip(
    points: &[Point3D],
    angle_deg_threshold: Real,
    min_edge_length: Real,
) -> Vec<usize> {
    let n = points.len();
    if n < 3 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let angle_rad_threshold = angle_deg_threshold.to_radians();

    for i in 1..(n - 1) {
        let p_prev = points[i - 1];
        let p_cur = points[i];
        let p_next = points[i + 1];

        let v_prev = Vector3D::new(p_cur.x - p_prev.x, p_cur.y - p_prev.y, p_cur.z - p_prev.z);
        let v_next = Vector3D::new(p_next.x - p_cur.x, p_next.y - p_cur.y, p_next.z - p_cur.z);

        let l_prev = v_prev.length();
        let l_next = v_next.length();

        // 너무 짧은 segment 들은 노이즈로 무시
        if l_prev < min_edge_length || l_next < min_edge_length {
            continue;
        }

        let angle = Vector3D::angle_between(&v_prev, &v_next);
        if angle >= angle_rad_threshold {
            result.push(i);
        }
    }

    result
}
```
```rust
/// 여러 strip 에서 corner index 를 모아서
/// 전체에서 공통으로 쓸 수 있는 "분할 인덱스 집합"을 구한다.
///
/// - `strips[k][i]` : k번째 strip의 i번째 샘플
/// - 반환되는 인덱스는 [1, len-2] 범위 (양 끝점은 제외)
pub fn on_collect_corner_indices_from_strips(
    strips: &[Vec<Point3D>],
    angle_deg_threshold: Real,
    min_edge_length: Real,
) -> Vec<usize> {
    if strips.is_empty() {
        return Vec::new();
    }

    let n_samples = strips[0].len();
    if n_samples < 3 {
        return Vec::new();
    }

    // 모든 strip 의 길이가 동일한지 확인
    for s in strips.iter() {
        if s.len() != n_samples {
            // 길이가 다르면 여기서는 corner 정렬이 애매해지므로,
            // 가장 안전하게는 빈 결과를 반환한다.
            return Vec::new();
        }
    }

    let mut indices = BTreeSet::new();

    for strip in strips {
        let corners = on_detect_corners_in_strip(strip, angle_deg_threshold, min_edge_length);
        for idx in corners {
            indices.insert(idx);
        }
    }

    indices.into_iter().collect()
}
```
```rust
pub fn on_split_point_strips_by_indices(
    strips: &[Vec<Point3D>],
    corner_indices: &[usize],
) -> Vec<Vec<Vec<Point3D>>> {
    if strips.is_empty() {
        return Vec::new();
    }

    let n_samples = strips[0].len();
    if n_samples < 2 {
        return Vec::new();
    }

    // 모든 strip 길이 동일한지 체크
    for s in strips.iter() {
        if s.len() != n_samples {
            return Vec::new();
        }
    }

    // corner_indices 를 정렬 + 중복 제거 + 유효 범위로 클램프
    let mut idxs: Vec<usize> = corner_indices
        .iter()
        .copied()
        .filter(|&i| i > 0 && i < n_samples - 1) // 0과 마지막은 제외
        .collect();
    idxs.sort_unstable();
    idxs.dedup();

    // 분할 경계 인덱스: [0, corner..., n_samples-1]
    let mut boundaries = Vec::with_capacity(idxs.len() + 2);
    boundaries.push(0);
    boundaries.extend(idxs.iter().copied());
    boundaries.push(n_samples - 1);

    let n_segments = boundaries.len().saturating_sub(1);
    let n_strips = strips.len();

    let mut segments: Vec<Vec<Vec<Point3D>>> = Vec::with_capacity(n_segments);

    for seg_id in 0..n_segments {
        let start = boundaries[seg_id];
        let end = boundaries[seg_id + 1];

        // [start ..= end] 구간을 잘라내므로, 최소 2개 이상이어야 의미 있음
        if end <= start {
            continue;
        }

        let mut seg_strips: Vec<Vec<Point3D>> = Vec::with_capacity(n_strips);
        for strip in strips {
            let slice = &strip[start..=end];
            seg_strips.push(slice.to_vec());
        }

        segments.push(seg_strips);
    }

    segments
}
```
```rust
pub fn on_loft_from_nurbs_curves(
    curves: &[NurbsCurve],
    sample_count: usize,
    pu: Degree,
    pv: Degree,
) -> Result<NurbsSurface> {
    let strips = NurbsSurface::sample_curves_to_point_strips(
        curves,
        sample_count,
        |c: &NurbsCurve| {
            let dom = c.domain(); // (t0, t1) 이라고 가정
            (dom.t0, dom.t1)
        },
        |c: &NurbsCurve, t: Real| c.eval_point(t),
    );

    // strips: Vec<Vec<Point3D>>
    //   strips[j][i] : j = 섹션 index, i = 섹션 위 파라미터 샘플
    NurbsSurface::loft_from_point_sections(&strips, pu, pv)
}
```
```rust
// NurbsCurve 두 개 → Ruled surface
pub fn on_ruled_from_two_nurbs_curves(
    c0: &NurbsCurve,
    c1: &NurbsCurve,
    sample_count: usize,
    pu: Degree,
) -> Result<NurbsSurface> {
    let curves = vec![c0.clone(), c1.clone()];
    let strips = NurbsSurface::sample_curves_to_point_strips(
        &curves,
        sample_count,
        |c: &NurbsCurve| (c.domain.t0, c.domain.t1),
        |c: &NurbsCurve, t: Real| c.eval_point(t),
    );

    // strips.len() == 2
    // strips[0], strips[1] 가 sec0, sec1 에 해당
    if strips.len() != 2 {
        return Err(NurbsError::InvalidSize);
    }

    NurbsSurface::ruled_from_two_point_sections(&strips[0], &strips[1], pu)
}
```
```rust
fn on_smooth_sections_along_loft_direction(
    sections: &mut [Vec<Point3D>],
    iterations: usize,
    lambda: Real,
) {
    let nv = sections.len();
    if nv < 3 || iterations == 0 {
        return;
    }
    let nu = sections[0].len();
    if nu == 0 {
        return;
    }

    // 모든 섹션 길이 동일한지 확인 (다르면 그냥 패스)
    for s in sections.iter() {
        if s.len() != nu {
            return;
        }
    }

    let lambda = lambda.clamp(0.0, 1.0);

    // 반복 smoothing
    for _ in 0..iterations {
        // 복사본에서 읽고, 원본에 쓴다
        let prev = sections.to_vec();

        // 각 단면 샘플 index i 에 대해, 섹션 방향으로 smoothing
        for i in 0..nu {
            // j = 0, nv-1 은 끝단이므로 그대로 유지 → end tangent match 용
            for j in 1..(nv - 1) {
                let p_im1 = prev[j - 1][i];
                let p_i = prev[j][i];
                let p_ip1 = prev[j + 1][i];

                // Laplacian = 평균 - 현재
                let avg_x = 0.5 * (p_im1.x + p_ip1.x);
                let avg_y = 0.5 * (p_im1.y + p_ip1.y);
                let avg_z = 0.5 * (p_im1.z + p_ip1.z);

                sections[j][i].x = p_i.x + lambda * (avg_x - p_i.x);
                sections[j][i].y = p_i.y + lambda * (avg_y - p_i.y);
                sections[j][i].z = p_i.z + lambda * (avg_z - p_i.z);
            }
        }
    }
}
```
```rust
/// 섹션 리스트(sections[j][i]) 에 대해,
/// Loft(섹션) 방향에서의 corner 섹션 index 집합을 찾는다.
///
/// - sections.len() = nv
/// - sections[j].len() = nu (모두 동일하다고 가정)
fn on_collect_corner_indices_along_sections(
    sections: &[Vec<Point3D>],
    angle_deg_threshold: Real,
    min_edge_length: Real,
) -> Vec<usize> {
    if sections.is_empty() {
        return Vec::new();
    }
    let nv = sections.len();
    let nu = sections[0].len();
    if nv < 3 || nu < 1 {
        return Vec::new();
    }
    for s in sections.iter() {
        if s.len() != nu {
            return Vec::new();
        }
    }

    let mut indices = BTreeSet::new();

    // i 고정하고 j 방향으로 polyline 구성 → corner 탐지
    for i in 0..nu {
        let mut strip: Vec<Point3D> = Vec::with_capacity(nv);
        for j in 0..nv {
            strip.push(sections[j][i]);
        }
        let corners = on_detect_corners_in_strip(&strip, angle_deg_threshold, min_edge_length);
        for idx in corners {
            indices.insert(idx);
        }
    }

    indices.into_iter().collect()
}
```
```rust
/// corner index 집합을 기준으로 섹션 리스트를 분할한다.
///
/// 반환:
///   segments[seg_idx][sec_idx][sample_idx]
fn on_split_sections_by_indices(
    sections: &[Vec<Point3D>],
    corner_indices: &[usize],
) -> Vec<Vec<Vec<Point3D>>> {
    if sections.is_empty() {
        return Vec::new();
    }
    let nv = sections.len();
    let nu = sections[0].len();
    if nv < 2 || nu == 0 {
        return Vec::new();
    }
    for s in sections.iter() {
        if s.len() != nu {
            return Vec::new();
        }
    }

    // corner index 정리: 1..nv-2 범위만
    let mut idxs: Vec<usize> = corner_indices
        .iter()
        .copied()
        .filter(|&j| j > 0 && j < nv - 1)
        .collect();
    idxs.sort_unstable();
    idxs.dedup();

    // 분할 경계: [0, corner..., nv-1]
    let mut boundaries = Vec::with_capacity(idxs.len() + 2);
    boundaries.push(0);
    boundaries.extend(idxs.iter().copied());
    boundaries.push(nv - 1);

    let n_segments = boundaries.len().saturating_sub(1);
    let mut segments: Vec<Vec<Vec<Point3D>>> = Vec::with_capacity(n_segments);

    for s in 0..n_segments {
        let start = boundaries[s];
        let end = boundaries[s + 1];
        if end <= start {
            continue;
        }

        // [start ..= end] 섹션들을 잘라서 하나의 segment 로 만든다.
        let mut seg_secs: Vec<Vec<Point3D>> = Vec::with_capacity(end - start + 1);
        for j in start..=end {
            seg_secs.push(sections[j].clone());
        }

        segments.push(seg_secs);
    }
    segments
}
```
```rust
fn on_build_sweep_frames(path: &[Point3D]) -> Option<Vec<(Point3D, Vector3D, Vector3D, Vector3D)>> {
    let n = path.len();
    if n < 2 {
        return None;
    }

    // 1) 각 지점의 tangent T_j 계산
    let mut tangents: Vec<Vector3D> = Vec::with_capacity(n);
    let mut v;
    for j in 0..n {
        let t = if j == 0 {
            v = path[1] - path[0];
            v
        } else if j + 1 == n {
            v = path[n - 1] - path[n - 2];
            v
        } else {
            v = path[j + 1] - path[j - 1];
            v
        };

        let mut t_norm = Vector3D::from(v);
        if !t_norm.normalize() {
            // 너무 짧으면 이전 tangent를 사용
            if let Some(prev) = tangents.last().cloned() {
                t_norm = prev;
            } else {
                // 완전히 퇴화
                t_norm = Vector3D::new(1.0, 0.0, 0.0);
            }
        }
        tangents.push(t_norm);
    }

    // 2) 초기 프레임 (j = 0) 설정
    let mut frames: Vec<(Point3D, Vector3D, Vector3D, Vector3D)> = Vec::with_capacity(n);

    let t0 = tangents[0];
    let mut up = Vector3D::new(0.0, 0.0, 1.0);
    // up 이 T와 거의 평행이면 다른 축 사용
    if t0.cross(&up).length() < 1e-6 {
        up = Vector3D::new(0.0, 1.0, 0.0);
    }

    let mut b0 = t0.cross(&up);
    if !b0.normalize() {
        b0 = Vector3D::new(1.0, 0.0, 0.0);
    }
    let mut n0 = b0.cross(&t0);
    if !n0.normalize() {
        n0 = Vector3D::new(0.0, 1.0, 0.0);
    }

    frames.push((path[0], t0, n0, b0));

    // 3) parallel transport 식으로 나머지 프레임들 계산
    for j in 1..n {
        let tj = tangents[j];
        let (_, prev_t, prev_n, _) = frames[j - 1].clone();

        // 이전 normal을 새로운 tangent에 투영 제거 → 평행 이동
        let mut nj = prev_n - tj * prev_n.dot(&tj);
        if !nj.normalize() {
            // 완전히 퇴화하면 초기 up 사용
            nj = up - tj * up.dot(&tj);
            if !nj.normalize() {
                // 그래도 안 되면 임의 축
                nj = Vector3D::new(0.0, 1.0, 0.0);
            }
        }
        let mut bj = tj.cross(&nj);
        if !bj.normalize() {
            bj = Vector3D::new(1.0, 0.0, 0.0);
        }

        frames.push((path[j], tj, nj, bj));
    }

    Some(frames)
}
```
```rust
fn on_build_section_local_coords(
    section: &[Point3D],
    origin: Point3D,
    t0: Vector3D,
    n0: Vector3D,
    b0: Vector3D,
) -> Vec<(Real, Real, Real)> {
    let mut local: Vec<(Real, Real, Real)> = Vec::with_capacity(section.len());

    for p in section {
        let q = *p - origin;
        let v = Vector3D::from(q);

        let sx = v.dot(&b0);
        let sy = v.dot(&n0);
        let sz = v.dot(&t0);

        local.push((sx, sy, sz));
    }

    local
}
```
```rust
pub fn on_sweep_from_nurbs_curves(
    section_curve: &NurbsCurve,
    path_curve: &NurbsCurve,
    sample_section: usize,
    sample_path: usize,
    pu: Degree,
    pv: Degree,
) -> Result<NurbsSurface> {
    // section curve 샘플
    let section_pts = {
        let curves = [section_curve.clone()];
        let strips = NurbsSurface::sample_curves_to_point_strips(
            &curves,
            sample_section,
            |c: &NurbsCurve| c.domain_tuple(), // (t0,t1)
            |c: &NurbsCurve, t: Real| c.eval_point(t),
        );
        strips.into_iter().next().unwrap_or_default()
    };

    // path curve 샘플
    let path_pts = {
        let curves = [path_curve.clone()];
        let strips = NurbsSurface::sample_curves_to_point_strips(
            &curves,
            sample_path,
            |c: &NurbsCurve| c.domain_tuple(),
            |c: &NurbsCurve, t: Real| c.eval_point(t),
        );
        strips.into_iter().next().unwrap_or_default()
    };

    NurbsSurface::sweep_from_section_and_path(&section_pts, &path_pts, pu, pv)
}
```
```rust
// polyline vs quad UV bbox 의 대략적인 교차 판정 (단순 bounding box overlap)
fn on_uv_polyline_intersects_quad(poly: &FeatureCurveUV, quad: &ParamQuad) -> bool {
    if poly.points.is_empty() {
        return false;
    }

    let (qu, qU, qv, qV) = quad.uv_bbox();

    for w in poly.points.windows(2) {
        let (u0, v0) = w[0];
        let (u1, v1) = w[1];

        let pu_min = u0.min(u1);
        let pu_max = u0.max(u1);
        let pv_min = v0.min(v1);
        let pv_max = v0.max(v1);

        if pu_max < qu || pu_min > qU || pv_max < qv || pv_min > qV {
            continue;
        }
        // bbox 가 겹치면 일단 교차 가능성이 있다고 본다 (보수적으로 refinement)
        return true;
    }

    false
}
```
```rust
/// 이 Quad 가 어떤 boundary / feature curve 에 "근접"해 있는지 대략 검사
fn on_quad_near_feature(quad: &ParamQuad, curves: &[FeatureCurveUV]) -> bool {
    curves
        .iter()
        .any(|c| on_uv_polyline_intersects_quad(c, quad))
}
```
