# 📐 BezierSurface 기능 및 수식 정리


## 🧱 구조 개요
```rust
struct BezierSurface {
    u_degree: usize,              // U 방향 차수 (n)
    v_degree: usize,              // V 방향 차수 (m)
    ctrl: Vec<Vec<CPoint>>,       // 제어점 격자 [u][v] (크기: (n+1) x (m+1))
}
```

## 소스 코드
```rust
use crate::core::basis::{bernstein_all_clamped, degree_elev_matrix, power_basis_matrix, split_curve_lerp};
use crate::core::bezier_curve::{on_re_param_matrix};
use crate::core::geom::{CPoint, Point};
use crate::core::knot::{clamped_uniform_knot_vector};
use crate::core::matrix::Matrix;
use crate::core::prelude::{Degree, KnotVector, Real, Surface};
use crate::core::types::SurfaceDir;

#[derive(Debug, Clone)]
pub struct BezierSurface {
    pub u_degree: usize,
    pub v_degree: usize,
    pub ctrl: Vec<Vec<CPoint>>, // [u][v] (len = u_degree+1) x (v_degree+1)
}
```
```rust
impl BezierSurface {
    pub(crate) fn from_ctrl(control_points: Vec<Vec<CPoint>>) -> BezierSurface {
        assert!(
            !control_points.is_empty(),
            "control_points must not be empty"
        );
        let nu = control_points.len();
        let nv = control_points[0].len();
        assert!(nv > 0, "each u-row must have at least one v point");
        for row in &control_points {
            assert_eq!(row.len(), nv, "non-rectangular control net: expected v-count {}, got {}", nv, row.len());
        }
        Self {
            u_degree: nu.saturating_sub(1),
            v_degree: nv.saturating_sub(1),
            ctrl: control_points,
        }
    }
}
```
```rust
impl BezierSurface {
    /// ctrl의 크기로부터 degree를 추론하여 생성.
    /// ctrl는 직사각형이어야 함.
    pub fn from_ctrl_grid(ctrl: Vec<Vec<CPoint>>) -> Result<Self, &'static str> {
        if ctrl.is_empty() || ctrl[0].is_empty() {
            return Err("BezierSurface: empty control net");
        }
        let u_len = ctrl.len();
        let v_len = ctrl[0].len();
        for row in &ctrl {
            if row.len() != v_len {
                return Err("BezierSurface: control net must be rectangular");
            }
        }
        Ok(Self {
            u_degree: u_len - 1,
            v_degree: v_len - 1,
            ctrl,
        })
    }

    /// 명시적 degree로 생성 (검증 포함)
    pub fn with_degrees(u_degree: usize, v_degree: usize, ctrl: Vec<Vec<CPoint>>) -> Result<Self, &'static str> {
        if ctrl.len() != u_degree + 1 { return Err("u rows != u_degree+1"); }
        if ctrl.is_empty() { return Err("empty control net"); }
        let vlen = ctrl[0].len();
        if vlen != v_degree + 1 { return Err("v cols != v_degree+1"); }
        for row in &ctrl {
            if row.len() != vlen { return Err("non-rectangular control net"); }
        }
        Ok(Self { u_degree, v_degree, ctrl })
    }

    #[inline] pub fn size(&self) -> (usize, usize) { (self.u_degree + 1, self.v_degree + 1) }
    #[inline] pub fn order_u(&self) -> usize { self.u_degree + 1 }
    #[inline] pub fn order_v(&self) -> usize { self.v_degree + 1 }

    /// 표면 평가: S(u,v) (동차합 → 유클리드)
    pub fn evaluate(&self, u: Real, v: Real) -> Point {
        let p = self.u_degree;
        let q = self.v_degree;
        debug_assert!((0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v));

        // Bernstein 벡터
        let bu = bernstein_all_clamped(p, u); // 아래 helper 사용
        let bv = bernstein_all_clamped(q, v);

        // 동차 합
        let mut x = 0.0; let mut y = 0.0; let mut z = 0.0; let mut w = 0.0;
        for i in 0..=p {
            for j in 0..=q {
                let b = bu[i] * bv[j];
                let c = &self.ctrl[i][j];
                x += b * c.x;
                y += b * c.y;
                z += b * c.z;
                w += b * c.w;
            }
        }
        if w != 0.0 {
            Point { x: x / w, y: y / w, z: z / w }
        } else {
            // 비가중(또는 w=0)도 케이스 방어
            Point { x, y, z }
        }
    }

    /// u-방향 차수 상승 (B_SDEGEL의 u방향 대응, degree_elev_matrix 재사용)
    pub fn elevate_u(&self, inc: usize) -> BezierSurface {
        if inc == 0 { return self.clone(); }
        let p = self.u_degree;
        let q = self.v_degree;
        let e = degree_elev_matrix(p, inc); // (p+inc+1) x (p+1)

        let new_p = p + inc;
        let mut new_ctrl = vec![vec![CPoint::zero(); q + 1]; new_p + 1];

        for j in 0..=q {
            for i in 0..=new_p {
                let mut acc = CPoint::zero();
                let i_min = i.saturating_sub(inc);
                let i_max = p.min(i);
                for k in i_min..=i_max {
                    let a = e[i][k];
                    acc.x += a * self.ctrl[k][j].x;
                    acc.y += a * self.ctrl[k][j].y;
                    acc.z += a * self.ctrl[k][j].z;
                    acc.w += a * self.ctrl[k][j].w;
                }
                new_ctrl[i][j] = acc;
            }
        }
        BezierSurface { u_degree: new_p, v_degree: q, ctrl: new_ctrl }
    }

    /// v-방향 차수 상승
    pub fn elevate_v(&self, inc: usize) -> BezierSurface {
        if inc == 0 { return self.clone(); }
        let p = self.u_degree;
        let q = self.v_degree;
        let e = degree_elev_matrix(q, inc); // (q+inc+1) x (q+1)

        let new_q = q + inc;
        let mut new_ctrl = vec![vec![CPoint::zero(); new_q + 1]; p + 1];

        for i in 0..=p {
            for j in 0..=new_q {
                let mut acc = CPoint::zero();
                let j_min = j.saturating_sub(inc);
                let j_max = q.min(j);
                for k in j_min..=j_max {
                    let a = e[j][k];
                    acc.x += a * self.ctrl[i][k].x;
                    acc.y += a * self.ctrl[i][k].y;
                    acc.z += a * self.ctrl[i][k].z;
                    acc.w += a * self.ctrl[i][k].w;
                }
                new_ctrl[i][j] = acc;
            }
        }
        BezierSurface { u_degree: p, v_degree: new_q, ctrl: new_ctrl }
    }

    /// u방향 split (de Casteljau를 각 v열별로 적용)
    pub fn split_u(&self, u: Real) -> (BezierSurface, BezierSurface) {
        let p = self.u_degree;
        let q = self.v_degree;

        // 각 v 열에 대해 de Casteljau 1D 분할
        let mut left = vec![vec![CPoint::zero(); q + 1]; p + 1];
        let mut right = vec![vec![CPoint::zero(); q + 1]; p + 1];

        for j in 0..=q {
            let mut col = (0..=p).map(|i| self.ctrl[i][j]).collect::<Vec<_>>();
            // 1D split (CPoint::lerp 사용)
            let (l, r) = split_curve_lerp(&mut col, u);
            for i in 0..=p { left[i][j] = l[i]; right[i][j] = r[i]; }
        }

        (
            BezierSurface { u_degree: p, v_degree: q, ctrl: left },
            BezierSurface { u_degree: p, v_degree: q, ctrl: right },
        )
    }

    /// v방향 split
    pub fn split_v(&self, v: Real) -> (BezierSurface, BezierSurface) {
        let p = self.u_degree;
        let q = self.v_degree;

        let mut left = vec![vec![CPoint::zero(); q + 1]; p + 1];
        let mut right = vec![vec![CPoint::zero(); q + 1]; p + 1];

        for i in 0..=p {
            let mut row = (0..=q).map(|j| self.ctrl[i][j]).collect::<Vec<_>>();
            let (l, r) = split_curve_lerp(&mut row, v);
            for j in 0..=q { left[i][j] = l[j]; right[i][j] = r[j]; }
        }

        (
            BezierSurface { u_degree: p, v_degree: q, ctrl: left },
            BezierSurface { u_degree: p, v_degree: q, ctrl: right },
        )
    }

    pub fn elevate_degree_dir(
        &self,
        dir: SurfaceDir,
        inc: usize,
    ) -> BezierSurface {
        match dir {
            SurfaceDir::UDir => {
                let elev_mat = degree_elev_matrix(self.u_degree, inc);
                let new_u = self.u_degree + inc;
                let vsize = self.v_degree + 1;
                let mut new_ctrl = vec![vec![CPoint::zero(); vsize]; new_u + 1];

                for v in 0..vsize {
                    for i in 0..=new_u {
                        let mut q = CPoint::zero();
                        let a = i.saturating_sub(inc);
                        let b = self.u_degree.min(i);
                        for k in a..=b {
                            q.x += elev_mat[i][k] * self.ctrl[k][v].x;
                            q.y += elev_mat[i][k] * self.ctrl[k][v].y;
                            q.z += elev_mat[i][k] * self.ctrl[k][v].z;
                            q.w += elev_mat[i][k] * self.ctrl[k][v].w;
                        }
                        new_ctrl[i][v] = q;
                    }
                }

                BezierSurface { u_degree: new_u, v_degree: self.v_degree, ctrl: new_ctrl }
            }

            SurfaceDir::VDir => {
                let elev_mat = degree_elev_matrix(self.v_degree, inc);
                let new_v = self.v_degree + inc;
                let usize = self.u_degree + 1;
                let mut new_ctrl = vec![vec![CPoint::zero(); new_v + 1]; usize];

                for u in 0..usize {
                    for j in 0..=new_v {
                        let mut q = CPoint::zero();
                        let a = j.saturating_sub(inc);
                        let b = self.v_degree.min(j);
                        for k in a..=b {
                            q.x += elev_mat[j][k] * self.ctrl[u][k].x;
                            q.y += elev_mat[j][k] * self.ctrl[u][k].y;
                            q.z += elev_mat[j][k] * self.ctrl[u][k].z;
                            q.w += elev_mat[j][k] * self.ctrl[u][k].w;
                        }
                        new_ctrl[u][j] = q;
                    }
                }

                BezierSurface { u_degree: self.u_degree, v_degree: new_v, ctrl: new_ctrl }
            }
        }
    }
    pub fn to_power_basis(
        &self,
        a: f64, b: f64,
        c: f64, d: f64,
    ) -> Vec<Vec<CPoint>> {
        let pum = power_basis_matrix(self.u_degree);
        let pvm = power_basis_matrix(self.v_degree);

        let rum = on_re_param_matrix(self.u_degree, a, b, 0.0, 1.0);
        let rvm = on_re_param_matrix(self.v_degree, c, d, 0.0, 1.0);

        // cum = rum * pum, cvm = rvm * pvm
        let cum = Matrix::mul(&rum, &pum);
        let cvm = Matrix::mul(&rvm, &pvm);

        // M_rmcprm(cum, Pw, cvm, bw)
        let m = self.u_degree + 1;
        let n = self.v_degree + 1;
        let mut bw = vec![vec![CPoint::zero(); n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut cp = CPoint::zero();
                for u in 0..m {
                    for v in 0..n {
                        let c = cum[i][u] * cvm[j][v];
                        cp.x += c * self.ctrl[u][v].x;
                        cp.y += c * self.ctrl[u][v].y;
                        cp.z += c * self.ctrl[u][v].z;
                        cp.w += c * self.ctrl[u][v].w;
                    }
                }
                bw[i][j] = cp;
            }
        }
        bw
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.u_degree+1, self.v_degree+1)
    }

    pub fn to_nurbs(&self) -> Surface {
        let (nu, nv) = self.dims();
        let degree_u = self.u_degree;
        let degree_v = self.v_degree;

        let knots_u = clamped_uniform_knot_vector(degree_u, nu);
        let knots_v = clamped_uniform_knot_vector(degree_v, nv);

        let mut ctrls : Vec<CPoint> = Vec::new();

        for i in 0..self.ctrl.len() {
            for j in 0..self.ctrl[i].len() {
                ctrls.push(self.ctrl[i][j].clone());
            }
        }

        Surface {
            pu: degree_u as Degree,
            pv: degree_v as Degree,
            nu,
            nv,
            ctrl: ctrls,
            ku: KnotVector {knots: knots_u},
            kv: KnotVector {knots: knots_v},
        }
    }
}
```

## 📏 수식 점검
Bezier 곡면은 다음과 같이 정의됩니다:

$$
S(u,v)=\sum _{i=0}^n\sum _{j=0}^mB_i^n(u)\cdot B_j^m(v)\cdot P_{i,j}
$$

- $B_i^n(u)$: U 방향 Bernstein basis
- $B_j^m(v)$: V 방향 Bernstein basis
- $P_{i,j}$: 제어점


## 🛠 기능별 설명 요약
| 메서드                        | 설명                                                             |
|------------------------------|------------------------------------------------------------------|
| from_ctrl(control_points)    | 제어점 격자로부터 BezierSurface 생성                           |
| from_ctrl_grid               | 제어점 격자에서 차수 추론하여 생성                              |
| with_degrees                 | 명시적 차수와 제어점으로 생성                                   |
| evaluate(u, v)               | 곡면 위 점 평가 (유리/비유리 모두 지원)                         |
| elevate_u(inc)               | U 방향 차수 승격                                                 |
| elevate_v(inc)               | V 방향 차수 승격                                                 |
| elevate_degree_dir(dir, inc) | 방향 지정 차수 승격 (SurfaceDir::UDir / VDir)                   |
| split_u(u)                   | U 방향 분할 (de Casteljau 적용)                                 |
| split_v(v)                   | V 방향 분할                                                      |
| to_power_basis(a,b,c,d)      | 주어진 구간으로 재매개화 후 Power basis 계수로 변환             |
| to_nurbs()                   | BezierSurface → NURBS Surface 변환                              |
| size(), dims(), order_u/v()  | 제어점 수 및 차수 정보 반환                                     |

- ctrl은 2차원 격자 형태의 제어점 배열이며, 각 행은 U 방향, 각 열은 V 방향을 나타냅니다.
- u_degree = ctrl.len() - 1, v_degree = ctrl[0].len() - 1로 자동 계산됩니다.
- 모든 행의 길이가 동일해야 하며, 비정사각형 격자는 허용되지 않습니다.


## ✅ 수식 점검 결과
| 항목                     | 수식 표현                                                                 | 설명                                      |
|--------------------------|----------------------------------------------------------------------------|-------------------------------------------|
| Bezier 곡면 정의         | $\( S(u,v) = \sum_{i=0}^n \sum_{j=0}^m B_i^n(u) B_j^m(v) P_{i,j} \)$         | U/V 방향 Bernstein 기저를 통한 곡면 평가 |
| 차수 계산          | $\( n = \mathrm{ctrl.len()} - 1,\quad m = \mathrm{ctrl[0].len()} - 1 \)$    | 제어점 격자 크기로부터 차수 추론         |
| 차수 승격                | $\( Q_i = \sum_k M_{ik} \cdot P_k \)$                                       | degree_elev_matrix 기반 제어점 승격       |
| 재매개화 행렬 요소       | $\( M_{ij} = C(i,j) \cdot \beta^{i-j} \cdot \alpha^j \)$                     | 선형 매핑 기반 계수 변환                  |
| Power basis 표현         | $\( S(u,v) = \sum_{i,j} a_{ij} u^i v^j \)$                                  | 단항식 기반 곡면 표현                     |


---

## ✅ BezierSurface 테스트 기능 요약
| 테스트 함수                          | 검증 내용 요약                                                      |
|-------------------------------------|----------------------------------------------------------------------|
| eval_bilinear_surface_center        | bilinear 곡면의 중심 평가 정확성 확인                               |
| elevate_and_split                   | U 방향 차수 승격 및 split 후 평가 일치 확인                         |
| test_surface_degree_elevation       | SurfaceDir::UDir 방향 차수 승격 확인                                |
| test_surface_to_power_basis         | Power basis 변환 결과의 크기 및 구조 확인                           |
| test_re_param_and_mul               | 재매개화 행렬과 역행렬 곱셈 시 항등 행렬 생성 여부 확인             |
| test_pt_identity                    | Bezier ↔ Power basis 변환 행렬의 상호 역함수 성질 확인              |
| test_re_param_exact_inverse         | 두 방향 재매개화 행렬 곱셈 시 항등 행렬 생성 여부 확인              |
| test_bezier_surface_to_nurbs_conversion | BezierSurface → NURBS 변환 시 차수, 제어점 수, 노트 벡터 검증     |
| test_surface_evaluate_edges         | 곡면의 네 모서리 점 평가 (u,v = 0 또는 1)                           |
| test_surface_split_v_consistency    | V 방향 split 후 평가 일치 확인                                      |
| test_surface_elevate_both_directions| U/V 방향 동시에 차수 승격 후 제어점 수 확인                         |
| test_surface_ctrl_rectangular_check | 비정사각형 제어점 입력 시 오류 발생 확인                            |
| test_surface_zero_weight_handling   | w=0 제어점 포함 시 평가 결과가 안전하게 처리되는지 확인             |

### 1. eval_bilinear_surface_center
```rust
#[test]
fn eval_bilinear_surface_center() {
    // p=q=1 (bilinear)
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let s = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let p = s.evaluate(0.5, 0.5);
    assert!((p.x - 0.5).abs() < 1e-12);
    assert!((p.y - 0.5).abs() < 1e-12);
}
```

### 2. elevate_and_split
```rust
#[test]
fn elevate_and_split() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let s = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let se = s.elevate_u(1);
    assert_eq!(se.u_degree, 2);

    let (sl, sr) = s.split_u(0.5);
    let pl = sl.evaluate(1.0, 0.5);
    let pr = sr.evaluate(0.0, 0.5);
    let pm = s.evaluate(0.5, 0.5);
    assert!((pl.x - pm.x).abs() < 1e-9 && (pl.y - pm.y).abs() < 1e-9);
    assert!((pr.x - pm.x).abs() < 1e-9 && (pr.y - pm.y).abs() < 1e-9);
}
```

### 3. test_surface_degree_elevation
```rust
#[test]
fn test_surface_degree_elevation() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)]
    ];
    let surf = BezierSurface { u_degree: 1, v_degree: 1, ctrl };
    let elevated = surf.elevate_degree_dir(SurfaceDir::UDir, 1);
    assert_eq!(elevated.u_degree, 2);
}
```

### 4. test_surface_to_power_basis
```rust
#[test]
fn test_surface_to_power_basis() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
                CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)]
    ];
    let surf = BezierSurface { u_degree: 1, v_degree: 1, ctrl };
    let power_basis = surf.to_power_basis(0.0, 1.0, 0.0, 1.0);
    assert_eq!(power_basis.len(), 2);
}
```

### 5. test_re_param_and_mul
```rust
#[test]
fn test_re_param_and_mul() {
    // 3차(4x4) 예시
    let m1 = on_re_param_matrix(3, 0.0, 1.0, -1.0, 1.0);
    let m2 = on_re_param_inverse_matrix(3, 0.0, 1.0, -1.0, 1.0);
    let id = Matrix::mul(&m1, &m2);
    // 대각 성분이 ~1에 가깝다 정도 체크
    for i in 0..=3 {
        println!("i = {}", id[i][i]);
        assert!((id[i][i] - 1.0).abs() < 1e-6);
    }
}
```
### 6. test_pt_identity
```rust
#[test]
fn test_pt_identity() {
    for n in 1..=6 {
        let p = power_to_bezier_matrix(n);
        let t = bezier_to_power_matrix(n);
        let id = Matrix::mul(&p, &t); // Bezier -> Power -> Bezier
        for i in 0..=n {
            for j in 0..=n {
                let want = if i == j { 1.0 } else { 0.0 };
                assert!((id[i][j] - want).abs() < 1e-12, "n={}, PT[{},{}]={}", n, i, j, id[i][j]);
            }
        }
        let id2 = Matrix::mul(&t, &p); // Power -> Bezier -> Power
        for i in 0..=n {
            for j in 0..=n {
                let want = if i == j { 1.0 } else { 0.0 };
                assert!((id2[i][j] - want).abs() < 1e-12, "n={}, TP[{},{}]={}", n, i, j, id2[i][j]);
            }
        }
    }
}
```
### 7. test_re_param_exact_inverse
```rust
#[test]
fn test_re_param_exact_inverse() {
    let n = 3;
    let m1 = on_re_param_matrix(n, 0.0, 1.0, -1.0, 1.0);   // [0,1] -> [-1,1]
    let m2 = on_re_param_matrix(n, -1.0, 1.0, 0.0, 1.0);   // [-1,1] -> [0,1]  (== inverse)
    let id = Matrix::mul(&m1, &m2);
    for i in 0..=n {
        for j in 0..=n {
            let want = if i == j { 1.0 } else { 0.0 };
            assert!((id[i][j] - want).abs() < 1e-12, "id[{},{}]={}", i, j, id[i][j]);
        }
    }
}
```

### 8. test_bezier_surface_to_nurbs_conversion
```rust
#[test]
fn test_bezier_surface_to_nurbs_conversion() {

    // 3x3 제어점 그리드 생성 (degree_u = 2, degree_v = 2)
    let ctrl = vec![
        vec![
            CPoint::from_point_w(&Point { x: 0.0, y: 0.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 0.0, y: 1.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 0.0, y: 2.0, z: 0.0 }, 1.0),
        ],
        vec![
            CPoint::from_point_w(&Point { x: 1.0, y: 0.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 1.0, y: 1.0, z: 1.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 1.0, y: 2.0, z: 0.0 }, 1.0),
        ],
        vec![
            CPoint::from_point_w(&Point { x: 2.0, y: 0.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 2.0, y: 1.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 2.0, y: 2.0, z: 0.0 }, 1.0),
        ],
    ];

    let bezier = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let nurbs = bezier.to_nurbs();

    // 차수 확인
    assert_eq!(nurbs.pu, 2);
    assert_eq!(nurbs.pv, 2);

    // 제어점 수 확인
    assert_eq!(nurbs.nu, 3);
    assert_eq!(nurbs.nv, 3);

    // 노트 벡터 길이 확인: n + p + 1 = 3 + 2 + 1 = 6
    assert_eq!(nurbs.ku.knots.len(), 6);
    assert_eq!(nurbs.kv.knots.len(), 6);

    // 클램프 확인
    assert_eq!(&nurbs.ku.knots[..3], &[0.0, 0.0, 0.0]);
    assert_eq!(&nurbs.ku.knots[3..], &[1.0, 1.0, 1.0]);

    dump_surface(&nurbs);
}
```


### 9. test_surface_evaluate_edges
```rust
#[test]
fn test_surface_evaluate_edges() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let s = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let p00 = s.evaluate(0.0, 0.0);
    let p11 = s.evaluate(1.0, 1.0);
    assert_eq!(p00.x, 0.0);
    assert_eq!(p11.x, 1.0);
}
```


### 10. test_surface_split_v_consistency
```rust
#[test]
fn test_surface_split_v_consistency() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let s = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let (sl, sr) = s.split_v(0.5);
    let pl = sl.evaluate(0.5, 1.0);
    let pr = sr.evaluate(0.5, 0.0);
    let pm = s.evaluate(0.5, 0.5);
    assert!((pl.x - pm.x).abs() < 1e-9 && (pr.x - pm.x).abs() < 1e-9);
}
```


### 11. test_surface_elevate_both_directions
```rust
#[test]
fn test_surface_elevate_both_directions() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let s = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let su = s.elevate_u(1);
    let suv = su.elevate_v(1);
    assert_eq!(suv.u_degree, 2);
    assert_eq!(suv.v_degree, 2);
}
```


### 12. test_surface_ctrl_rectangular_check
```rust
#[test]
fn test_surface_ctrl_rectangular_check() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let result = BezierSurface::from_ctrl_grid(ctrl);
    assert!(result.is_err());
}
```


### 13. test_surface_zero_weight_handling
```rust
#[test]
fn test_surface_zero_weight_handling() {
    let ctrl = vec![
        vec![CPoint::from_point_w(&Point::new(0.0,0.0,0.0),0.0),
             CPoint::from_point_w(&Point::new(0.0,1.0,0.0),1.0)],
        vec![CPoint::from_point_w(&Point::new(1.0,0.0,0.0),1.0),
             CPoint::from_point_w(&Point::new(1.0,1.0,0.0),1.0)],
    ];
    let s = BezierSurface::from_ctrl_grid(ctrl).unwrap();
    let p = s.evaluate(0.0, 0.0);
    assert!(p.x.abs() < 1e-6); // w=0 제어점이 있어도 평가가 안전하게 처리되는지 확인
}
```

