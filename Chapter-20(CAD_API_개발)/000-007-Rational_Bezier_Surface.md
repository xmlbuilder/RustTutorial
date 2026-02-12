# 📘 Rational Bézier Surface 정리
    - (Piegl & Tiller Section 1.5 기반)

## 🔷 1. 정의: Rational Bézier Surface
### ✔ 동차 공간에서의 정의
4- D polynomial Bézier surface:
```math
S^w(u,v) = \sum_{i=0}^{n} \sum_{j=0}^{m} B_{i,n}(u)\,B_{j,m}(v)\,P^w_{i,j}
```

- 여기서:
    - $P_{i,j}^w=(w_{i,j}x_{i,j},\; w_{i,j}y_{i,j},\; w_{i,j}z_{i,j},\; w_{i,j})$: 동차 컨트롤 포인트
    - $B_{i,n}(u),B_{j,m}(v)$: Bernstein basis

### ✔ 투영을 통해 유클리드 공간으로 변환
```math
S(u,v) = H(S^w(u,v)) = \frac{\sum B_{i,n}(u)\,B_{j,m}(v)\,w_{i,j}\,P_{i,j}}{\sum B_{i,n}(u)\,B_{j,m}(v)\,w_{i,j}}
```

- 또는 rational basis로 표현하면:
```math
S(u,v) = \sum_{i=0}^{n} \sum_{j=0}^{m} R_{i,j}(u,v)\,P_{i,j}
```

- 여기서:
```math
R_{i,j}(u,v) = \frac{B_{i,n}(u)\,B_{j,m}(v)\,w_{i,j}}{\sum_{r=0}^{n} \sum_{s=0}^{m} B_{r,n}(u)\,B_{s,m}(v)\,w_{r,s}}
```

- Rational Bézier Surface는 4D polynomial surface를 W=1 평면으로 투영한 것이다.

## 🔷 2. Rational Basis의 기하학적 의미
- 각 $R_{i,j}(u,v)$ 는 곡면 위 점에 대한 영향력(weighted basis)
- 모든 $R_{i,j}(u,v)\geq 0$, 총합은 1 (partition of unity)
- 특정 weight를 키우면 해당 점의 영향력이 커짐
- 예: $w_{0,1}=5 → R_{0,1}(u,v)$ 가 봉처럼 튀어나옴 (Figure 1.26a)

## 🔷 3. 예제: 원호를 연결한 원기둥 곡면
- ✔ 두 원호 곡선 정의 (z=1, z=–1 평면)
```math
C^w_0(u) = \sum_{i=0}^{2} B_{i,2}(u)\,P^w_{i,0}
```
```math
C^w_1(u) = \sum_{i=0}^{2} B_{i,2}(u)\,P^w_{i,1}
```

- 컨트롤 포인트:
```math
P^w_{i,0} = (1,1,0,1),\; (1,1,1,1),\; (2,0,2,2)
```
```math
P^w_{i,1} = (-1,-1,0,1),\; (-1,1,1,1),\; (-2,0,2,2)
```
- 각각 z=1, z=–1 평면에 있는 원호

### ✔ 곡면 정의: 두 원호를 선형 보간
```math
S^w(u,v) = \sum_{i=0}^{2} \sum_{j=0}^{1} B_{i,2}(u)\,B_{j,1}(v)\,P^w_{i,j}
```

- u 방향: 2차 Bézier
- v 방향: 1차 Bézier (선형 보간)
→ 원호들을 z 방향으로 연결한 원기둥 곡면

## 🔷 4. 고정된 파라미터에서의 의미
- ✔ 고정 u=u_0
```math
C^w_{u_0}(v) = \sum_{j=0}^{1} B_{j,1}(v)\,Q^w_j(u_0)
```

- $C_0^w(u_0)$ 와 $C_1^w(u_0)$ 사이의 직선 보간
- z 방향으로 연결된 선분

### ✔ 고정 v=v_0
```math
C^w_{v_0}(u) = \sum_{i=0}^{2} B_{i,2}(u)\,Q^w_i(v_0)
```

- v_0에 따라 x 위치가 바뀌는 원호
- 평면 $x=1-2v_0$ 위의 원호

## 🔷 5. 예제 평가: S(1/2,1/2)
- 알고리즘 A1.7에 따라:
    - n=2>m=1 → 먼저 v 방향 처리
    - 각 행 i에 대해 v=1/2로 보간:
```math
Q^w_0 = (1,1,0,1)
```
```math
Q^w_1 = (0,1,0,1)
```
```math
Q^w_2 = (-1,1,0,1)
```

- 이 세 점으로 u 방향 Bézier 곡선을 평가하면 $S^w(1/2,1/2)$
- 투영하면 S(1/2,1/2)

## 🔷 한 문장 요약
- Rational Bézier Surface는 4D polynomial surface를 투영하여 얻으며,
- rational basis $R_{i,j}(u,v)$ 를 통해 곡면 위 점을 계산한다.
- 예제에서는 두 원호를 선형 보간하여 원기둥 곡면을 구성하며,
- 이는 Bézier surface의 구조와 rational 투영의 결합을 잘 보여준다.

---

## 원기둥 예제 상세 설명

### 1. 예제의 목표: “원기둥 패치” 를 Rational Bézier로 만들기
- 이 예제는 한마디로 말하면:
- 동차 공간에서 두 개의 원호(rational Bézier curve)를 만들고, 그 둘을 선형 보간해서 “원기둥 곡면”을 만드는 과정을 보여준다.


### 2. 첫 번째 단계: 원호 하나 만들기 (yz-평면)
- 먼저 Section 1.4에서 나온 원호:
```math
C^w(u)=\sum _{i=0}^2B_{i,2}(u)\, P_i^w
```
- 여기서
```math
\{ P_i^w\} =\{ (0,1,0,1),\  (0,1,1,1),\  (0,0,2,2)\}
``` 
- 이건 yz-평면(x=0) 위의 원호를 나타내는 rational Bézier 곡선이다.
- 동차 좌표이므로, 투영하면 실제 원 위의 점들이 나온다.

### 3. 두 번째 단계: 이 원호를 x=1, x=–1 평면으로 평행이동
- 이제 이 원호를 x 방향으로 평행이동해서 두 개의 원호를 만든다.
```math
C_0^w(u)=\sum _{i=0}^2B_{i,2}(u)\, P_{i,0}^w
```
```math
C_1^w(u)=\sum _{i=0}^2B_{i,2}(u)\, P_{i,1}^w
```
- 컨트롤 포인트는:
```math
\{ P_{i,0}^w\} =\{ (1,1,0,1),\  (1,1,1,1),\  (2,0,2,2)\}
```
```math 
\{ P_{i,1}^w\} =\{ (-1,-1,0,1),\  (-1,1,1,1),\  (-2,0,2,2)\}
``` 
- $C_0^w(u)$: x=1 평면 위의 원호
- $C_1^w(u)$: x=–1 평면 위의 원호
- 즉, 서로 평행한 두 평면 위에 같은 모양의 원호가 하나씩 있다.

### 4. 세 번째 단계: 두 원호를 v로 선형 보간 → 원기둥 곡면
- 이제 이 두 곡선을 v로 선형 보간해서 곡면을 만든다.
- 동차 Bézier 곡면:
```math
S^w(u,v)=\sum _{i=0}^2\sum _{j=0}^1B_{i,2}(u)\, B_{j,1}(v)\, P_{i,j}^w
```
- u 방향: 2차 Bézier (원호)
- v 방향: 1차 Bézier (직선 보간)
- 고정 u=u_0:
```math
C_{u_0}^w(v)=\sum _{j=0}^1B_{j,1}(v)\, Q_j^w(u_0)
```
여기서 $Q_j^w(u_0)$ 는 u 방향에서 평가된 점들.
- 이건 기하적으로:
- $C_0^w(u_0)$ 와 $C_1^w(u_0)$ 사이를 v로 선형 보간한 직선
  - x축 방향으로 뻗은 선분 → 원기둥의 **생성선**

- 고정 v=v_0:
```math
C_{v_0}^w(u)=S^w(u,v_0)=\sum _{i=0}^2B_{i,2}(u)\, Q_i^w(v_0)
```
- 이건:
x가 $x=(1-v_0)(+1)+v_0(-1)=1-2v_0$ 인 평면 위의 원호

- 즉:
  - v 고정 → 특정 x 평면에서의 원호
  - u 고정 → 두 원호를 잇는 직선
- 전체적으로 “원기둥 곡면”이 된다.

### 5. 네 번째 단계: 실제로 S(1/2,1/2)를 계산해보자
- 우리는 S(1/2,1/2)를 구하고 싶다.
- 알고리즘 A1.7에 따르면, n=2>m=1 이므로 먼저 v 방향을 처리한다.
#### 5.1 v=1/2에서 각 열을 보간
- v 방향은 1차 Bézier이므로:
```math
B_{0,1}\left( \frac{1}{2}\right) =\frac{1}{2},\quad B_{1,1}\left( \frac{1}{2}\right) =\frac{1}{2}
```
- 각 i에 대해:
```math
Q_i^w(v_0)=\sum _{j=0}^1B_{j,1}(1/2)\, P_{i,j}^w=\frac{1}{2}P_{i,0}^w+\frac{1}{2}P_{i,1}^w
```
- 실제로 계산하면:
- i=0:
```math
Q_0^w=\frac{1}{2}(1,1,0,1)+\frac{1}{2}(-1,-1,0,1)=(0,0,0,1)
```
- (책에서는 좌표를 약간 다르게 쓰지만, 구조는 “두 점의 중간”)
- i=1:
```math
Q_1^w=\frac{1}{2}(1,1,1,1)+\frac{1}{2}(-1,1,1,1)=(0,1,1,1)
```
- i=2:
```math
Q_2^w=\frac{1}{2}(2,0,2,2)+\frac{1}{2}(-2,0,2,2)=(0,0,2,2)
```
- 책에서는 이 결과를 정리해서:
- $Q_0^w(v_0)=(0,1,0,1)$
- $Q_1^w(v_0)=(0,1,1,1)$
- $Q_2^w(v_0)=(0,0,2,2)$
- 처럼 “yz-평면(x=0) 위의 원호 컨트롤 포인트”로 다시 표현하고 있다.
- 핵심은:
v=1/2에서 얻은 세 점 $Q_i^w$ 가 yz-평면 위의 원호를 이루는 동차 컨트롤 포인트가 된다는 것.
#### 5.2 이제 u 방향 2차 Bézier로 다시 평가이제:
```math
C_{v_0=1/2}^w(u)=\sum _{i=0}^2B_{i,2}(u)\, Q_i^w(v_0)
```
- 이는 yz-평면(x=0) 위의 원호.
- u=1/2에서:
```math
B_{0,2}\left( \frac{1}{2}\right) =\frac{1}{4},\quad B_{1,2}\left( \frac{1}{2}\right) =\frac{1}{2},\quad B_{2,2}\left( \frac{1}{2}\right) =\frac{1}{4}
```
- 따라서:
```math
S^w\left( \frac{1}{2},\frac{1}{2}\right) =C_{v_0=1/2}^w\left( \frac{1}{2}\right) =\frac{1}{4}Q_0^w+\frac{1}{2}Q_1^w+\frac{1}{4}Q_2^w
```
- 책에서 계산한 결과:
```math
S^w\left( \frac{1}{2},\frac{1}{2}\right) =\left( 0,\  \frac{3}{4},\  1,\  \frac{5}{4}\right)
```
### 6. 마지막 단계: 투영해서 실제 점 S(1/2, 1/2) 얻기투영 H를 적용하면:
```math
S\left( \frac{1}{2},\frac{1}{2}\right) =H\left( 0,\frac{3}{4},1,\frac{5}{4}\right) =\left( 0,\  \frac{3/4}{5/4},\  \frac{1}{5/4}\right) =\left( 0,\  \frac{3}{5},\  \frac{4}{5}\right)
```
- 즉, 원기둥 곡면 위의 한 점이:
```math
S\left( \frac{1}{2},\frac{1}{2}\right) =(0,\  0.6,\  0.8)
```
### 7. 이 예제가 주는 핵심 직관- 곡선 → 곡면:
- u 방향: 원호 (rational Bézier curve)
- v 방향: 두 원호 사이의 직선 보간
  - 원기둥 곡면
- 동차 공간에서의 선형성:
  - 모든 계산은 동차 좌표에서 선형 결합으로 진행
  - 마지막에만 W로 나누어 유클리드 공간으로 투영
- deCasteljau2와 완전히 같은 구조:
  - v 방향 1D de Casteljau
  - u 방향 1D de Casteljau
- Tensor product rational Bézier surface 평가
---
## 원기둥 검증 테스트
```rust
use nurbslib::core::geom::{Point3D, Point4D};
use nurbslib::core::bezier_surface::BezierSurface; // 실제 모듈 경로에 맞게 수정
/// Piegl & Tiller 1.5 cylinder patch example:
/// - u-degree = 2 (quadratic rational Bezier arc)
/// - v-degree = 1 (linear interpolation between two arcs)
///
/// Control points are homogeneous P^w = (Xw, Yw, Zw, W).
pub fn make_rational_cylinder_patch() -> BezierSurface {
    // j=0: arc on plane x=+1
    let p00 = Point4D { x:  1.0, y: 1.0, z: 0.0, w: 1.0 };
    let p10 = Point4D { x:  1.0, y: 1.0, z: 1.0, w: 1.0 };
    let p20 = Point4D { x:  2.0, y: 0.0, z: 2.0, w: 2.0 };

    // j=1: arc on plane x=-1 (NOTE: p01.y = +1.0 for book's S(1/2,1/2))
    let p01 = Point4D { x: -1.0, y: 1.0, z: 0.0, w: 1.0 };
    let p11 = Point4D { x: -1.0, y: 1.0, z: 1.0, w: 1.0 };
    let p21 = Point4D { x: -2.0, y: 0.0, z: 2.0, w: 2.0 };

    // ctrl[u][v]
    let ctrl = vec![
        vec![p00, p01],
        vec![p10, p11],
        vec![p20, p21],
    ];

    BezierSurface::with_degrees(2, 1, ctrl).unwrap()
}

#[test]
fn piegl_cylinder_patch_midpoint_matches_book() {
    let s = make_rational_cylinder_patch();

    let u = 0.5;
    let v = 0.5;

    let p = s.point_at(u, v);

    // expected: (0, 3/5, 4/5)
    let ex = 0.0;
    let ey = 3.0 / 5.0;
    let ez = 4.0 / 5.0;

    let tol = 1e-12;
    assert!((p.x - ex).abs() <= tol, "x={}", p.x);
    assert!((p.y - ey).abs() <= tol, "y={}", p.y);
    assert!((p.z - ez).abs() <= tol, "z={}", p.z);
}
```
---

## 소스 코드
```rust
#[derive(Debug, Clone)]
pub struct BezierSurface {
    pub dim: usize,
    pub u_degree: usize,
    pub v_degree: usize,
    pub ctrl: Vec<Vec<Point4D>>, // [u][v] (len = u_degree+1) x (v_degree+1)
}
```
```rust
impl SurfaceGeom for BezierSurface {
    fn domain_u(&self) -> Interval {
        Interval { t0: 0.0, t1: 1.0 }
    }

    fn domain_v(&self) -> Interval {
        Interval { t0: 0.0, t1: 1.0 }
    }

    fn eval_point(&self, u: Real, v: Real) -> Point3D {
        self.point_at(u, v)
    }

    fn eval_ders_nder(&self, u: Real, v: Real, d: usize) -> Vec<Vec<Vector3D>> {
        let order = d.min(2);

        // Prepare an (order+1) x (order+1) zero vector matrix
        let mut ders = vec![vec![Vector3D::new(0.0, 0.0, 0.0); order + 1]; order + 1];

        // If only the first derivative is needed, set second_order = false
        
        let second_order = order >= 2;
        if let Some(surf_ders) = self.evaluate_with_ders(u, v, second_order) {
            // 0th order: point
            ders[0][0] = Vector3D::new(surf_ders.point.x, surf_ders.point.y, surf_ders.point.z);

            if order >= 1 {
                // First derivative: Su, Sv
                ders[1][0] = surf_ders.du;
                ders[0][1] = surf_ders.dv;
            }

            if order >= 2 {
                if let Some(duu) = surf_ders.duu {
                    ders[2][0] = duu;
                }
                if let Some(duv) = surf_ders.duv {
                    ders[1][1] = duv;
                }
                if let Some(dvv) = surf_ders.dvv {
                    ders[0][2] = dvv;
                }
            }
        }
        ders
    }
}
```
```rust
impl BezierSurface {
    pub fn new(u_degree: usize, v_degree: usize, control_points: Vec<Vec<Point4D>>) -> Self {
        Self {
            dim: 3,
            u_degree,
            v_degree,
            ctrl: control_points,
        }
    }

    pub fn create_only(u_degree: usize, v_degree: usize) -> Self {
        Self {
            dim: 3,
            u_degree,
            v_degree,
            ctrl: vec![vec![Point4D::default(); v_degree + 1]; u_degree + 1],
        }
    }

    pub fn from_ctrl(control_points: Vec<Vec<Point4D>>) -> BezierSurface {
        assert!(
            !control_points.is_empty(),
            "control_points must not be empty"
        );
        let nu = control_points.len();
        let nv = control_points[0].len();
        assert!(nv > 0, "each u-row must have at least one v point");
        for row in &control_points {
            assert_eq!(
                row.len(),
                nv,
                "non-rectangular control net: expected v-count {}, got {}",
                nv,
                row.len()
            );
        }

        Self {
            dim: 3,
            u_degree: nu.saturating_sub(1),
            v_degree: nv.saturating_sub(1),
            ctrl: control_points,
        }
    }
}
```
```rust
impl BezierSurface {
    /// Create by inferring the degree from the size of `ctrl`.
    /// `ctrl` must be rectangular.
    pub fn from_ctrl_grid(ctrl: Vec<Vec<Point4D>>) -> Result<Self, &'static str> {
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
            dim: 3,
            u_degree: u_len - 1,
            v_degree: v_len - 1,
            ctrl,
        })
    }

    /// Create with an explicit degree (including validation)
    pub fn with_degrees(
        u_degree: usize,
        v_degree: usize,
        ctrl: Vec<Vec<Point4D>>,
    ) -> Result<Self, &'static str> {
        if ctrl.len() != u_degree + 1 {
            return Err("u rows != u_degree+1");
        }
        if ctrl.is_empty() {
            return Err("empty control net");
        }
        let v_len = ctrl[0].len();
        if v_len != v_degree + 1 {
            return Err("v cols != v_degree+1");
        }
        for row in &ctrl {
            if row.len() != v_len {
                return Err("non-rectangular control net");
            }
        }
        Ok(Self {
            dim: 3,
            u_degree,
            v_degree,
            ctrl,
        })
    }
```
```rust
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.u_degree + 1, self.v_degree + 1)
    }
```
```rust
    #[inline]
    pub fn order_u(&self) -> usize {
        self.u_degree + 1
    }
```
```rust
    #[inline]
    pub fn order_v(&self) -> usize {
        self.v_degree + 1
    }
```
```rust
    /// Surface evaluation: S(u,v) (homogeneous sum → Euclidean)
    pub fn point_at(&self, u: Real, v: Real) -> Point3D {
        let p = self.u_degree;
        let q = self.v_degree;
        debug_assert!((0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v));

        // Bernstein vector
        let bu = on_bernstein_all_clamped(p, u); // 아래 helper 사용
        let bv = on_bernstein_all_clamped(q, v);

        // Homogeneous sum
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
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
            Point3D {
                x: x / w,
                y: y / w,
                z: z / w,
            }
        } else {
            // Handle the case of unweighted (or w=0) as well
            Point3D { x, y, z }
        }
    }
```
```rust
    /// Degree elevation in the u-direction (apply to u, reuse degree_elev_matrix)
    pub fn elevate_u(&self, inc: usize) -> BezierSurface {
        if inc == 0 {
            return self.clone();
        }
        let p = self.u_degree;
        let q = self.v_degree;
        let e = on_degree_elevation_matrix(p, inc); // (p+inc+1) x (p+1)

        let new_p = p + inc;
        let mut new_ctrl = vec![vec![Point4D::zero(); q + 1]; new_p + 1];

        for j in 0..=q {
            for i in 0..=new_p {
                let mut acc = Point4D::zero();
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
        BezierSurface {
            dim: 3,
            u_degree: new_p,
            v_degree: q,
            ctrl: new_ctrl,
        }
    }
```
```rust
    /// Degree elevation in the v-direction
    pub fn elevate_v(&self, inc: usize) -> BezierSurface {
        if inc == 0 {
            return self.clone();
        }
        let p = self.u_degree;
        let q = self.v_degree;
        let e = on_degree_elevation_matrix(q, inc); // (q+inc+1) x (q+1)

        let new_q = q + inc;
        let mut new_ctrl = vec![vec![Point4D::zero(); new_q + 1]; p + 1];

        for i in 0..=p {
            for j in 0..=new_q {
                let mut acc = Point4D::zero();
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
        BezierSurface {
            dim: 3,
            u_degree: p,
            v_degree: new_q,
            ctrl: new_ctrl,
        }
    }
```
```rust
    /// Split in the u-direction (apply de Casteljau to each v-column)
    pub fn split_u(&self, u: Real) -> (BezierSurface, BezierSurface) {
        let p = self.u_degree;
        let q = self.v_degree;

        // Perform 1D de Casteljau split for each v-column
        let mut left = vec![vec![Point4D::zero(); q + 1]; p + 1];
        let mut right = vec![vec![Point4D::zero(); q + 1]; p + 1];

        for j in 0..=q {
            let mut col = (0..=p).map(|i| self.ctrl[i][j]).collect::<Vec<_>>();
            // 1D split (CPoint::lerp 사용)
            let (l, r) = on_split_curve_lerp(&mut col, u);
            for i in 0..=p {
                left[i][j] = l[i];
                right[i][j] = r[i];
            }
        }

        (
            BezierSurface {
                dim: 3,
                u_degree: p,
                v_degree: q,
                ctrl: left,
            },
            BezierSurface {
                dim: 3,
                u_degree: p,
                v_degree: q,
                ctrl: right,
            },
        )
    }
```
```rust
    /// Split in the v-direction
    pub fn split_v(&self, v: Real) -> (BezierSurface, BezierSurface) {
        let p = self.u_degree;
        let q = self.v_degree;

        let mut left = vec![vec![Point4D::zero(); q + 1]; p + 1];
        let mut right = vec![vec![Point4D::zero(); q + 1]; p + 1];

        for i in 0..=p {
            let mut row = (0..=q).map(|j| self.ctrl[i][j]).collect::<Vec<_>>();
            let (l, r) = on_split_curve_lerp(&mut row, v);
            for j in 0..=q {
                left[i][j] = l[j];
                right[i][j] = r[j];
            }
        }

        (
            BezierSurface {
                dim: 3,
                u_degree: p,
                v_degree: q,
                ctrl: left,
            },
            BezierSurface {
                dim: 3,
                u_degree: p,
                v_degree: q,
                ctrl: right,
            },
        )
    }
```
```rust
    pub fn elevate_degree_dir(&self, dir: SurfaceDir, inc: usize) -> BezierSurface {
        match dir {
            SurfaceDir::UDir => {
                let elev_mat = on_degree_elevation_matrix(self.u_degree, inc);
                let new_u = self.u_degree + inc;
                let v_size = self.v_degree + 1;
                let mut new_ctrl = vec![vec![Point4D::zero(); v_size]; new_u + 1];

                for v in 0..v_size {
                    for i in 0..=new_u {
                        let mut q = Point4D::zero();
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

                BezierSurface {
                    dim: 3,
                    u_degree: new_u,
                    v_degree: self.v_degree,
                    ctrl: new_ctrl,
                }
            }

            SurfaceDir::VDir => {
                let elev_mat = on_degree_elevation_matrix(self.v_degree, inc);
                let new_v = self.v_degree + inc;
                let usize = self.u_degree + 1;
                let mut new_ctrl = vec![vec![Point4D::zero(); new_v + 1]; usize];

                for u in 0..usize {
                    for j in 0..=new_v {
                        let mut q = Point4D::zero();
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

                BezierSurface {
                    dim: 3,
                    u_degree: self.u_degree,
                    v_degree: new_v,
                    ctrl: new_ctrl,
                }
            }
        }
    }
```
```rust
    pub fn to_power_basis_unit_domain(
        &self,
        a: Real,
        b: Real,
        c: Real,
        d: Real,
    ) -> Vec<Vec<Point4D>> {
        let pum = on_power_basis_matrix(self.u_degree);
        let pvm = on_power_basis_matrix(self.v_degree);

        let rum = on_re_param_matrix(self.u_degree, a, b, 0.0, 1.0);
        let rvm = on_re_param_matrix(self.v_degree, c, d, 0.0, 1.0);

        // cum = rum * pum, cvm = rvm * pvm
        let cum = Matrix::mul(&rum, &pum);
        let cvm = Matrix::mul(&rvm, &pvm);

        let m = self.u_degree + 1;
        let n = self.v_degree + 1;
        let mut bw = vec![vec![Point4D::zero(); n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut cp = Point4D::zero();
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
```
```rust
    pub fn to_power_basis_domain(&self, a: Real, b: Real, c: Real, d: Real) -> Vec<Vec<Point4D>> {
        on_bezier_surface_to_power_basis_4d(&self.ctrl, self.u_degree, self.v_degree, a, b, c, d)
    }
```
```rust
    pub fn dims(&self) -> (usize, usize) {
        (self.u_degree + 1, self.v_degree + 1)
    }
```
```rust
    pub fn to_nurbs(&self) -> NurbsSurface {
        let (nu, nv) = self.dims();
        let degree_u = self.u_degree;
        let degree_v = self.v_degree;

        let knots_u = on_clamped_uniform_knot_vector(degree_u, nu);
        let knots_v = on_clamped_uniform_knot_vector(degree_v, nv);

        let mut ctrls: Vec<Point4D> = Vec::new();

        for iv in 0..nv {
            for iu in 0..nu {
                // Assume that self.ctrl[iu][iv] corresponds to (u = i, v = j)
                ctrls.push(self.ctrl[iu][iv].clone());
            }
        }

        NurbsSurface {
            dim: 3,
            pu: degree_u as Degree,
            pv: degree_v as Degree,
            nu,
            nv,
            ctrl: ctrls,
            ku: KnotVector { knots: knots_u },
            kv: KnotVector { knots: knots_v },
            domain_u: Default::default(),
            domain_v: Default::default(),
        }
    }
}
```
```rust
impl BezierSurface {
    /// Internal: compute 0th, 1st, and 2nd order partial derivatives
    /// of the homogeneous Bézier surface S(u,v) = (Xw, Yw, Zw, W) in 4D.
    /// If `second_order = false` → returns S, S_u, S_v.
    /// If `second_order = true`  → additionally returns S_uu, S_uv, S_vv.
    fn eval_h_partials(
        &self,
        u: Real,
        v: Real,
        second_order: bool,
    ) -> Option<(
        Point4D,
        Point4D,
        Point4D,
        Option<(Point4D, Point4D, Point4D)>,
    )> {
        let p = self.u_degree;
        let q = self.v_degree;

        if self.ctrl.is_empty() || self.ctrl[0].is_empty() {
            return None;
        }

        // Bernstein functions and derivatives in u- and v-directions
        let (bu, bu1, bu2) = on_bernstein_0_1_2(p, u);
        let (bv, bv1, bv2) = on_bernstein_0_1_2(q, v);

        // S, Su, Sv, Suu, Suv, Svv (4D)
        let mut s = Point4D::zero();
        let mut su = Point4D::zero();
        let mut sv = Point4D::zero();
        let mut suu = Point4D::zero();
        let mut suv = Point4D::zero();
        let mut svv = Point4D::zero();

        for i in 0..=p {
            for j in 0..=q {
                let cv = self.ctrl[i][j];

                let b = bu[i] * bv[j];
                let bu_ = bu1[i];
                let bv_ = bv1[j];
                let b1u = bu_ * bv[j]; // B'_i(u) B_j(v)
                let b1v = bu[i] * bv_; // B_i(u) B'_j(v)

                s.x += b * cv.x;
                s.y += b * cv.y;
                s.z += b * cv.z;
                s.w += b * cv.w;

                su.x += b1u * cv.x;
                su.y += b1u * cv.y;
                su.z += b1u * cv.z;
                su.w += b1u * cv.w;

                sv.x += b1v * cv.x;
                sv.y += b1v * cv.y;
                sv.z += b1v * cv.z;
                sv.w += b1v * cv.w;

                if second_order {
                    let b2u = bu2[i] * bv[j]; // B''_i(u) B_j(v)
                    let b2v = bu[i] * bv2[j]; // B_i(u) B''_j(v)
                    let buv = bu1[i] * bv1[j]; // B'_i(u) B'_j(v)

                    suu.x += b2u * cv.x;
                    suu.y += b2u * cv.y;
                    suu.z += b2u * cv.z;
                    suu.w += b2u * cv.w;

                    svv.x += b2v * cv.x;
                    svv.y += b2v * cv.y;
                    svv.z += b2v * cv.z;
                    svv.w += b2v * cv.w;

                    suv.x += buv * cv.x;
                    suv.y += buv * cv.y;
                    suv.z += buv * cv.z;
                    suv.w += buv * cv.w;
                }
            }
        }

        if second_order {
            Some((s, su, sv, Some((suu, suv, svv))))
        } else {
            Some((s, su, sv, None))
        }
    }
}
```
```rust
#[derive(Debug, Clone)]
pub struct BezierSurfaceDers {
    pub point: Point3D,
    pub du: Vector3D,
    pub dv: Vector3D,
    pub duu: Option<Vector3D>,
    pub duv: Option<Vector3D>,
    pub dvv: Option<Vector3D>,
}
```
```rust
impl BezierSurface {
    /// Compute the surface value and 1st/2nd order partial derivatives in R³.
    /// If `second_order = false` → returns P, Su, Sv.
    /// If `second_order = true`  → additionally returns Suu, Suv, Svv.
    pub fn evaluate_with_ders(
        &self,
        u: Real,
        v: Real,
        second_order: bool,
    ) -> Option<BezierSurfaceDers> {
        let (s, su, sv, maybe_second) = self.eval_h_partials(u, v, second_order)?;

        let w0 = s.w;
        if !w0.is_finite() || w0.abs() < 1e-15 {
            return None;
        }

        // Separate homogeneous to Euclidean
        let v0 = Vector3D::new(s.x, s.y, s.z);
        let vu = Vector3D::new(su.x, su.y, su.z);
        let vv = Vector3D::new(sv.x, sv.y, sv.z);

        let wu = su.w;
        let wv = sv.w;

        let w0_2 = w0 * w0;

        // First-order partial derivatives (exactly the same pattern as for curves)
        let du = (vu * w0 - v0 * wu) / w0_2;
        let dv = (vv * w0 - v0 * wv) / w0_2;

        let p = Point3D {
            x: s.x / w0,
            y: s.y / w0,
            z: s.z / w0,
        };

        if !second_order {
            return Some(BezierSurfaceDers {
                point: p,
                du,
                dv,
                duu: None,
                duv: None,
                dvv: None,
            });
        }

        // Second-order partial derivatives (for general rational surfaces)
        let (suu, suv, svv) = maybe_second?;

        let vuu = Vector3D::new(suu.x, suu.y, suu.z);
        let vuv = Vector3D::new(suv.x, suv.y, suv.z);
        let vvv = Vector3D::new(svv.x, svv.y, svv.z);

        let wuu = suu.w;
        let wuv = suv.w;
        let wvv = svv.w;

        let w0_3 = w0_2 * w0;

        // x = V / w
        // X_u  = (V_u w - V w_u) / w^2
        // X_uu = (V_uu w - V w_uu) / w^2 - 2 (V_u w - V w_u) w_u / w^3

        let n_u = vu * w0 - v0 * wu;
        let n_uu = vuu * w0 - v0 * wuu;
        let duu = (n_uu / w0_2) - (n_u * (2.0 * wu) / w0_3);

        // X_vv
        let n_v = vv * w0 - v0 * wv;
        let n_vv = vvv * w0 - v0 * wvv;
        let dvv = (n_vv / w0_2) - (n_v * (2.0 * wv) / w0_3);

        // X_uv
        // N = V_u w - V w_u
        // N_v = V_uv w + V_u w_v - V_v w_u - V w_uv
        let n_uv = vuv * w0 + vu * wv - vv * wu - v0 * wuv;
        let duv = (n_uv / w0_2) - (n_u * (2.0 * wv) / w0_3);

        Some(BezierSurfaceDers {
            point: p,
            du,
            dv,
            duu: Some(duu),
            duv: Some(duv),
            dvv: Some(dvv),
        })
    }
```
```rust
    /// Unit normal vector
    pub fn normal(&self, u: Real, v: Real) -> Option<Vector3D> {
        let d = self.evaluate_with_ders(u, v, false)?;
        let n = d.du.cross(&d.dv);
        let len = n.length();
        if len <= 1e-15 {
            return None;
        }
        Some(n / len)
    }
```
```rust
    /// Gaussian curvature K and mean curvature H
    /// (return None in singular or nearly flat cases)
    pub fn curvatures(&self, u: Real, v: Real) -> Option<(Real, Real)> {
        let d = self.evaluate_with_ders(u, v, true)?;
        let du = d.du;
        let dv = d.dv;
        let duu = d.duu?;
        let duv = d.duv?;
        let dvv = d.dvv?;

        let n = du.cross(&dv);
        let n_len = n.length();
        if n_len <= 1e-15 {
            return None;
        }
        let n_unit = n / n_len;

        // First fundamental form
        let e = du.dot(&du);
        let f = du.dot(&dv);
        let g = dv.dot(&dv);

        // Second fundamental form
        let l = duu.dot(&n_unit);
        let m = duv.dot(&n_unit);
        let n2 = dvv.dot(&n_unit);

        let eg_f2 = e * g - f * f;
        if eg_f2.abs() <= 1e-24 {
            return None;
        }

        let k = (l * n2 - m * m) / eg_f2;
        let h = (e * n2 - 2.0 * f * m + g * l) / (2.0 * eg_f2);

        Some((k, h))
    }
}
```
```rust
impl BezierSurface {
    pub fn evaluate_derivs(&self, u: Real, v: Real) -> Option<Point3D> {
        let d = self.evaluate_with_ders(u, v, false)?;
        Some(d.point)
    }

    /// Return results as a Rust-style struct.
    ///
    /// `der_u` and `der_v` specify the maximum derivative order (0–2) in the u and v directions.
    /// - If `der_u >= 1` → include Su
    /// - If `der_v >= 1` → include Sv
    /// - If `der_u >= 2` → include Suu
    /// - If `der_v >= 2` → include Svv
    /// - If `der_u >= 1 && der_v >= 1` → include Suv
    pub fn evaluate(
        &self,
        u: Real,
        v: Real,
        der_u: usize,
        der_v: usize,
    ) -> Option<BezierSurfaceDers> {
        let need_second = der_u > 1 || der_v > 1;
        self.evaluate_with_ders(u, v, need_second)
    }
```
```rust
    pub fn normal_at(&self, u: Real, v: Real) -> Option<Vector3D> {
        self.normal(u, v)
    }
```
```rust
    /// Wrapper for `curvatures(u,v)` returning K (Gaussian curvature) and H (Mean curvature).
    pub fn curvatures_at(&self, u: Real, v: Real) -> Option<(Real, Real)> {
        self.curvatures(u, v)
    }

    /// Helper to compute principal curvatures k1, k2 from K and H.
    /// Returns k1 ≤ k2, using H ± sqrt(H² − K).
    pub fn principal_curvatures_at(&self, u: Real, v: Real) -> Option<(Real, Real)> {
        let (k, h) = self.curvatures(u, v)?;
        let disc = h * h - k;
        if disc < 0.0 {
            return None;
        }
        let s = disc.sqrt();
        let k1 = h - s;
        let k2 = h + s;
        Some((k1, k2))
    }
```
```rust
    /// Sampling-based bounding box
    pub fn bounding_box_point(&self, nu: usize, nv: usize) -> Option<(Point3D, Point3D)> {
        if self.ctrl.is_empty() || nu < 2 || nv < 2 {
            return None;
        }

        let mut min = Point3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        let nu1 = nu - 1;
        let nv1 = nv - 1;

        for i in 0..=nu1 {
            let u = (i as Real) / (nu1 as Real);
            for j in 0..=nv1 {
                let v = (j as Real) / (nv1 as Real);
                if let Some(p) = self.evaluate_derivs(u, v) {
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                    min.z = min.z.min(p.z);

                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                    max.z = max.z.max(p.z);
                }
            }
        }

        Some((min, max))
    }
```
```rust
    pub fn bounding_box(&self, nu: usize, nv: usize) -> Option<BoundingBox> {
        if let Some(min_max) = self.bounding_box_point(nu, nv) {
            return Some(BoundingBox::new(min_max.0, min_max.1));
        }
        None
    }
```
```rust
    /// Support flipping in U / V directions
    pub fn reverse_u(&mut self) {
        // ctrl[i][j], i=0..p, j=0..q
        self.ctrl.reverse(); // Flip indices in the U direction
    }

    pub fn reverse_v(&mut self) {
        for row in &mut self.ctrl {
            row.reverse(); // For each U row, flip the V indices
        }
    }
}
```
```rust
impl BezierSurface {
    /// Number of control points (u_count, v_count)
    pub fn cv_size(&self) -> (usize, usize) {
        (self.u_degree + 1, self.v_degree + 1)
    }
```
```rust
    pub fn cv_count(&self, dir: usize) -> usize {
        if dir == 0 {
            self.u_degree + 1
        } else {
            self.v_degree + 1
        }
    }
```
```rust
    /// (i,j) weight (equivalent to C++ Weight(i,j))
    pub fn weight(&self, i: usize, j: usize) -> Option<Real> {
        self.ctrl.get(i).and_then(|row| row.get(j)).map(|cp| cp.w)
    }
```
```rust
    /// Set (i,j) weight
    pub fn set_weight(&mut self, i: usize, j: usize, w: Real) -> bool {
        if let Some(row) = self.ctrl.get_mut(i) {
            if let Some(cp) = row.get_mut(j) {
                cp.w = w;
                return true;
            }
        }
        false
    }
```
```rust
    /// Set (i,j) control vertex (coordinates + weight)
    pub fn set_cv_w(&mut self, i: usize, j: usize, p: Point3D, w: Real) -> bool {
        if let Some(row) = self.ctrl.get_mut(i) {
            if let Some(cp) = row.get_mut(j) {
                cp.x = p.x * w;
                cp.y = p.y * w;
                cp.z = p.z * w;
                cp.w = w;
                return true;
            }
        }
        false
    }
```
```rust
    pub fn set_cv(&mut self, i: usize, j: usize, p: Point4D) -> bool {
        if let Some(row) = self.ctrl.get_mut(i) {
            if let Some(cp) = row.get_mut(j) {
                cp.x = p.x * p.w;
                cp.y = p.y * p.w;
                cp.z = p.z * p.w;
                cp.w = p.w;
                return true;
            }
        }
        false
    }
```
```rust
    pub fn get_cv(&self, i: usize, j: usize) -> Option<Point4D> {
        if let Some(row) = self.ctrl.get(i) {
            if let Some(cp) = row.get(j) {
                Some(cp.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
```
```rust
    /// Make all control points non‑rational
    pub fn make_non_rational(&mut self) {
        for row in &mut self.ctrl {
            for cp in row {
                if cp.w != 0.0 && cp.w.is_finite() && (cp.w - 1.0).abs() > 1e-15 {
                    let w = cp.w;
                    cp.x /= w;
                    cp.y /= w;
                    cp.z /= w;
                    cp.w = 1.0;
                }
            }
        }
    }
```
```rust
    /// Trim the entire surface to a sub‑patch (equivalent to C++ Trim).
    /// - Requires u0 < u1, v0 < v1
    /// - The resulting surface is re‑mapped to [0,1] × [0,1] as a BezierSurface
    pub fn trim(
        &self,
        mut u0: Real,
        mut u1: Real,
        mut v0: Real,
        mut v1: Real,
    ) -> Option<BezierSurface> {
        // Interval cleanup
        if u0 > u1 {
            std::mem::swap(&mut u0, &mut u1);
        }
        if v0 > v1 {
            std::mem::swap(&mut v0, &mut v1);
        }

        if u1 <= 0.0 || u0 >= 1.0 || v1 <= 0.0 || v0 >= 1.0 {
            return None;
        }

        u0 = u0.clamp(0.0, 1.0);
        u1 = u1.clamp(0.0, 1.0);
        v0 = v0.clamp(0.0, 1.0);
        v1 = v1.clamp(0.0, 1.0);

        if (u1 - u0).abs() < 1e-15 || (v1 - v0).abs() < 1e-15 {
            return None;
        }

        // Trim in the U direction first
        // 1) Up to [0, u1]
        let (su0, _) = self.split_u(u1);

        if u0 > 0.0 {
            // 2) Within that, keep only the [u0/u1, 1] portion
            let local_u = u0 / u1;
            let (_, su) = su0.split_u(local_u);
            // Perform trimming in the V direction using su
            return su.trim_v(v0, v1);
        }

        // When u0 == 0: su0 already covers [0, u1]
        su0.trim_v(v0, v1)
    }
```
```rust
    /// Internal use: V‑direction trim (used in the above trim)
    fn trim_v(&self, mut v0: Real, mut v1: Real) -> Option<BezierSurface> {
        if v0 > v1 {
            std::mem::swap(&mut v0, &mut v1);
        }
        if v1 <= 0.0 || v0 >= 1.0 {
            return None;
        }
        v0 = v0.clamp(0.0, 1.0);
        v1 = v1.clamp(0.0, 1.0);
        if (v1 - v0).abs() < 1e-15 {
            return None;
        }

        // 1) [0, v1]
        let (sv0, _) = self.split_v(v1);

        if v0 > 0.0 {
            let local_v = v0 / v1;
            let (_, sv) = sv0.split_v(local_v);
            Some(sv)
        } else {
            Some(sv0)
        }
    }
```
```rust
    /// Transpose by flipping the U/V axes
    pub fn transpose(&mut self) {
        let (nu, nv) = self.cv_size();
        let mut new_ctrl = vec![vec![Point4D::zero(); nu]; nv];
        for i in 0..nu {
            for j in 0..nv {
                new_ctrl[j][i] = self.ctrl[i][j].clone();
            }
        }
        std::mem::swap(&mut self.ctrl, &mut new_ctrl);
        std::mem::swap(&mut self.u_degree, &mut self.v_degree);
    }
}

impl BezierSurface {
    /// Extract V-iso curve at fixed u ∈ [0,1] as a Bezier curve of degree v_degree.
    pub fn iso_v_curve(&self, u: Real) -> BezierCurve {
        let q = self.v_degree;
        let mut col = vec![Point4D::zero(); q + 1];
        // de Casteljau along U for each V column
        for j in 0..=q {
            let mut a: Vec<Point4D> = (0..=self.u_degree).map(|i| self.ctrl[i][j]).collect();
            for r in 1..=self.u_degree {
                for i in 0..(self.u_degree - r + 1) {
                    a[i] = a[i].lerp(&a[i + 1], u);
                }
            }
            col[j] = a[0];
        }
        BezierCurve::create(q, col).unwrap()
    }
```
```rust
    /// Extract U-iso curve at fixed v ∈ [0,1] as a Bezier curve of degree u_degree.
    pub fn iso_u_curve(&self, v: Real) -> BezierCurve {
        let p = self.u_degree;
        let mut row = vec![Point4D::zero(); p + 1];
        for i in 0..=p {
            let mut a: Vec<Point4D> = (0..=self.v_degree).map(|j| self.ctrl[i][j]).collect();
            for r in 1..=self.v_degree {
                for j in 0..(self.v_degree - r + 1) {
                    a[j] = a[j].lerp(&a[j + 1], v);
                }
            }
            row[i] = a[0];
        }
        BezierCurve::create(p, row).unwrap()
    }
```
```rust
    /// Unit tangents (Sû, Sv̂) at (u,v), if meaningful
    pub fn unit_tangents(&self, u: Real, v: Real) -> Option<(Vector3D, Vector3D)> {
        let d = self.evaluate_with_ders(u, v, false)?;
        let tu = d.du;
        let tv = d.dv;
        let lu = tu.length();
        let lv = tv.length();
        if lu <= 1e-15 || lv <= 1e-15 {
            return None;
        }
        Some((tu / lu, tv / lv))
    }

    /// Unit normal wrapper at (u,v)
    pub fn unit_normal(&self, u: Real, v: Real) -> Option<Vector3D> {
        self.normal(u, v)
    }
}
```
```rust
impl BezierSurface {
    /// Sanitize weights for the entire surface
    pub fn sanitize_weights(&mut self) {
        for row in &mut self.ctrl {
            for cp in row {
                if !cp.w.is_finite() || cp.w == 0.0 {
                    let w_old = if cp.w == 0.0 || !cp.w.is_finite() {
                        1.0
                    } else {
                        cp.w
                    };
                    cp.x /= w_old;
                    cp.y /= w_old;
                    cp.z /= w_old;
                    cp.w = 1.0;
                }
            }
        }
    }
```
```rust
    /// Extract a sub-grid (control net window), indices inclusive
    pub fn subgrid(&self, iu0: usize, iu1: usize, iv0: usize, iv1: usize) -> Option<BezierSurface> {
        let (m, n) = self.cv_size();
        if iu0 > iu1 || iv0 > iv1 || iu1 >= m || iv1 >= n {
            return None;
        }
        let u_deg = iu1 - iu0;
        let v_deg = iv1 - iv0;
        let mut ctrl = vec![vec![Point4D::zero(); v_deg + 1]; u_deg + 1];
        for i in 0..=u_deg {
            for j in 0..=v_deg {
                ctrl[i][j] = self.ctrl[iu0 + i][iv0 + j];
            }
        }
        BezierSurface::with_degrees(u_deg, v_deg, ctrl).ok()
    }
}
```
```rust
impl BezierSurface {
    pub fn sample_grid(&self, nu: usize, nv: usize) -> Vec<Vec<Point3D>> {
        let mut grid = Vec::new();
        for i in 0..=nu {
            let u = i as f64 / nu as f64;
            let mut row = Vec::new();
            for j in 0..=nv {
                let v = j as f64 / nv as f64;
                row.push(self.point_at(u, v));
            }
            grid.push(row);
        }
        grid
    }
}
```
```rust
impl BezierSurface {
    pub fn surface_area(&self, nu: usize, nv: usize) -> Real {
        let grid = self.sample_grid(nu, nv);
        let mut area = 0.0;
        for i in 0..nu {
            for j in 0..nv {
                let p00 = grid[i][j];
                let p10 = grid[i + 1][j];
                let p01 = grid[i][j + 1];
                let p11 = grid[i + 1][j + 1];
                // 두 삼각형으로 근사
                area += (p10 - p00).cross_pt(&(p01 - p00)).length() * 0.5;
                area += (p11 - p10).cross_pt(&(p01 - p10)).length() * 0.5;
            }
        }
        area
    }
}
```
```rust
impl BezierSurface {
    /// Generate wireframe edges from sampled grid
    pub fn wireframe(&self, nu: usize, nv: usize) -> Vec<(Point3D, Point3D)> {
        let grid = self.sample_grid(nu, nv);
        let mut edges = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                edges.push((grid[i][j], grid[i + 1][j])); // u-direction edge
                edges.push((grid[i][j], grid[i][j + 1])); // v-direction edge
            }
        }
        edges
    }
}
```
```rust
impl BezierSurface {
    pub fn elevate_row_col(
        &self,
        dir: SurfaceDir,
        inc: usize,
        f: usize,
        l: usize,
        roc: usize,
    ) -> BezierSurface {
        let (p, q) = (self.u_degree, self.v_degree);

        let r = match dir {
            SurfaceDir::UDir => p,
            SurfaceDir::VDir => q,
        };
        let dm = on_degree_elevation_matrix(r, inc);

        let new_p = if dir == SurfaceDir::UDir { p + inc } else { p };
        let new_q = if dir == SurfaceDir::VDir { q + inc } else { q };

        let mut new_ctrl = vec![vec![Point4D::zero(); new_q + 1]; new_p + 1];
        // 원본 복사
        for i in 0..=p {
            for j in 0..=q {
                new_ctrl[i][j] = self.ctrl[i][j];
            }
        }

        on_elevate_surface_row_col(&self.ctrl, r, inc, &dm, dir, f, l, roc, &mut new_ctrl);

        BezierSurface {
            dim: 3,
            u_degree: new_p,
            v_degree: new_q,
            ctrl: new_ctrl,
        }
    }
}
```
```rust
impl BezierSurface {
    #[inline]
    pub fn ctrl_size_ok(&self) -> bool {
        self.ctrl.len() == self.u_degree + 1
            && self.ctrl.iter().all(|row| row.len() == self.v_degree + 1)
    }

    /// - 이 함수는 "표면 전체 elevation"을 하지 않는다.
    /// - 외부에서 new_ctrl을 미리 할당하고 roc를 반복하며 호출해야 한다.
    pub fn elevate_row_col_kernel(
        &self,
        dir: SurfaceDir,
        inc: usize,
        f: usize,
        l: usize,
        roc: usize,
        dm: &[Vec<Real>],              // (r+inc+1) x (r+1)
        out_ctrl: &mut [Vec<Point4D>], // new ctrl net (already allocated)
    ) {
        let r = match dir {
            SurfaceDir::UDir => self.u_degree,
            SurfaceDir::VDir => self.v_degree,
        };

        elevate_surface_row_col_kernel(&self.ctrl, r, inc, dm, dir, f, l, roc, out_ctrl);
    }

    /// U 방향으로 표면 전체 degree elevation (p -> p+inc)
    pub fn elevate_u_full(&self, inc: usize) -> BezierSurface {
        assert!(self.ctrl_size_ok());
        if inc == 0 {
            return self.clone();
        }

        let p = self.u_degree;
        let q = self.v_degree;

        let dm = on_degree_elevation_matrix(p, inc);
        let new_p = p + inc;

        let mut new_ctrl = vec![vec![Point4D::zero(); q + 1]; new_p + 1];

        // 모든 column(roc = v index)을 반복해서 채움
        for roc in 0..=q {
            elevate_surface_row_col_kernel(
                &self.ctrl,
                p,
                inc,
                &dm,
                SurfaceDir::UDir,
                0,
                new_p,
                roc,
                &mut new_ctrl,
            );
        }

        BezierSurface {
            dim: self.dim,
            u_degree: new_p,
            v_degree: q,
            ctrl: new_ctrl,
        }
    }
```
```rust
    /// V 방향으로 표면 전체 degree elevation (q -> q+inc)
    pub fn elevate_v_full(&self, inc: usize) -> BezierSurface {
        assert!(self.ctrl_size_ok());
        if inc == 0 {
            return self.clone();
        }

        let p = self.u_degree;
        let q = self.v_degree;

        let dm = on_degree_elevation_matrix(q, inc);
        let new_q = q + inc;

        let mut new_ctrl = vec![vec![Point4D::zero(); new_q + 1]; p + 1];

        // 모든 row(roc = u index)을 반복해서 채움
        for roc in 0..=p {
            elevate_surface_row_col_kernel(
                &self.ctrl,
                q,
                inc,
                &dm,
                SurfaceDir::VDir,
                0,
                new_q,
                roc,
                &mut new_ctrl,
            );
        }

        BezierSurface {
            dim: self.dim,
            u_degree: p,
            v_degree: new_q,
            ctrl: new_ctrl,
        }
    }
}
```
```rust
impl BezierSurface {
    pub fn elevate_row(&self, row: usize, inc: usize) -> Vec<Point4D> {
        let p = self.u_degree;
        let elev = on_degree_elevation_matrix(p, inc);
        let new_p = p + inc;

        let mut out = vec![Point4D::zero(); new_p + 1];

        for i in 0..=new_p {
            let mut acc = Point4D::zero();
            let i_min = i.saturating_sub(inc);
            let i_max = p.min(i);

            for k in i_min..=i_max {
                let a = elev[i][k];
                acc.x += a * self.ctrl[k][row].x;
                acc.y += a * self.ctrl[k][row].y;
                acc.z += a * self.ctrl[k][row].z;
                acc.w += a * self.ctrl[k][row].w;
            }

            out[i] = acc;
        }

        out
    }
}
```
```rust
// ------------------------------------------------------------
// Public API on BezierSurface
// ------------------------------------------------------------
impl BezierSurface {
    /// Reduce degree in U-direction by 1 (p -> p-1) for the whole surface.
    /// Returns: (new surface, max error over all columns)
    pub fn reduce_u_full(&self) -> (BezierSurface, f64) {
        let p = self.u_degree;
        let q = self.v_degree;

        assert!(p >= 2, "u_degree must be >= 2");
        debug_assert!(self.ctrl.len() == p + 1);
        debug_assert!(self.ctrl.iter().all(|row| row.len() == q + 1));

        let (alf, oma, bet, omb) = on_degree_reduction_coeffs(p);

        let new_p = p - 1;
        let mut new_ctrl = vec![vec![Point4D::default(); q + 1]; new_p + 1];

        let mut emax = 0.0;
        for k in 0..=q {
            let e = reduce_row_col_kernel(
                &self.ctrl,
                p,
                q,
                SurfaceDir::UDir,
                k,
                &alf,
                &oma,
                &bet,
                &omb,
                &mut new_ctrl,
            );
            if e > emax {
                emax = e;
            }
        }

        (
            BezierSurface {
                dim: self.dim,
                u_degree: new_p,
                v_degree: q,
                ctrl: new_ctrl,
            },
            emax,
        )
    }
```
```rust
    /// Reduce degree in V-direction by 1 (q -> q-1) for the whole surface.
    /// Returns: (new surface, max error over all rows)
    pub fn reduce_v_full(&self) -> (BezierSurface, f64) {
        let p = self.u_degree;
        let q = self.v_degree;

        assert!(q >= 2, "v_degree must be >= 2");
        debug_assert!(self.ctrl.len() == p + 1);
        debug_assert!(self.ctrl.iter().all(|row| row.len() == q + 1));

        let (alf, oma, bet, omb) = on_degree_reduction_coeffs(q);

        let new_q = q - 1;
        let mut new_ctrl = vec![vec![Point4D::default(); new_q + 1]; p + 1];

        let mut emax = 0.0;
        for k in 0..=p {
            let e = reduce_row_col_kernel(
                &self.ctrl,
                p,
                q,
                SurfaceDir::VDir,
                k,
                &alf,
                &oma,
                &bet,
                &omb,
                &mut new_ctrl,
            );
            if e > emax {
                emax = e;
            }
        }

        (
            BezierSurface {
                dim: self.dim,
                u_degree: p,
                v_degree: new_q,
                ctrl: new_ctrl,
            },
            emax,
        )
    }
}
```
```rust
impl BezierSurface {
    /// Reduce degree by 1 along u or v, for a single row/column k.
    ///
    /// - dir = UDir → reduce u-degree, fix column k
    /// - dir = VDir → reduce v-degree, fix row k
    ///
    /// Returns: (new surface after reduction, max error e on that row/column)
    pub fn reduce_degree_row_col(
        &self,
        dir: SurfaceDir,
        k: usize,
        alf: &[Real],
        oma: &[Real],
        bet: &[Real],
        omb: &[Real],
    ) -> (BezierSurface, f64) {
        let p = self.u_degree;
        let q = self.v_degree;

        match dir {
            SurfaceDir::UDir => {
                assert!(p >= 1);
                assert!(k <= q);

                let new_p = p - 1;
                let mut new_ctrl = vec![vec![Point4D::zero(); q + 1]; new_p + 1];

                let e = on_bezier_surface_degree_reduce_row_col(
                    &self.ctrl,
                    p,
                    q,
                    SurfaceDir::UDir,
                    k,
                    alf,
                    oma,
                    bet,
                    omb,
                    &mut new_ctrl,
                );

                (
                    BezierSurface {
                        dim: 3,
                        u_degree: new_p,
                        v_degree: q,
                        ctrl: new_ctrl,
                    },
                    e,
                )
            }

            SurfaceDir::VDir => {
                assert!(q >= 1);
                assert!(k <= p);

                let new_q = q - 1;
                let mut new_ctrl = vec![vec![Point4D::zero(); new_q + 1]; p + 1];

                let e = on_bezier_surface_degree_reduce_row_col(
                    &self.ctrl,
                    p,
                    q,
                    SurfaceDir::VDir,
                    k,
                    alf,
                    oma,
                    bet,
                    omb,
                    &mut new_ctrl,
                );

                (
                    BezierSurface {
                        dim: 3,
                        u_degree: p,
                        v_degree: new_q,
                        ctrl: new_ctrl,
                    },
                    e,
                )
            }
        }
    }
}
```
```rust
impl BezierSurface {
    /// Product of bivariate Bezier scalar function f(u,v) and this Bezier surface.
    ///
    /// f: control values (p+1) x (q+1)
    /// self: surface degree (r,s) = (self.u_degree, self.v_degree)
    /// result: surface degree (p+r, q+s)
    ///
    /// Range version: only fills [su..eu]x[sv..ev] in the output control net.
    pub fn multiply_by_scalar_bezier_function_range(
        &self,
        f: &[Vec<Real>],
        p: usize,
        q: usize,
        su: usize,
        eu: usize,
        sv: usize,
        ev: usize,
    ) -> Result<BezierSurface, NurbsError> {
        // ---- validate f dimensions ----
        if f.len() != p + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "f row count must be p+1 (got {}, expected {})",
                    f.len(),
                    p + 1
                ),
            });
        }
        for (i, row) in f.iter().enumerate() {
            if row.len() != q + 1 {
                return Err(NurbsError::InvalidArgument {
                    msg: format!(
                        "f[{}].len() must be q+1 (got {}, expected {})",
                        i,
                        row.len(),
                        q + 1
                    ),
                });
            }
        }

        let r = self.u_degree;
        let s = self.v_degree;

        let pu = p + r;
        let qv = q + s;

        if su > eu || eu > pu {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}",
                    su, eu, pu
                ),
            });
        }
        if sv > ev || ev > qv {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}",
                    sv, ev, qv
                ),
            });
        }

        // output control net (p+r+1) x (q+s+1)
        let mut qw = vec![vec![Point4D::zero(); qv + 1]; pu + 1];

        // ---- main loops: identical indexing to C B_sfnsur ----
        for i in su..=eu {
            let kl = i.saturating_sub(r);
            let kh = (std::cmp::min)(p, i);

            for j in sv..=ev {
                let ll = j.saturating_sub(s);
                let lh = (std::cmp::min)(q, j);

                let mut acc = Point4D::zero();

                for k in kl..=kh {
                    // U[i][k]
                    let a_u = on_bezier_product_coeff(p as i32, r as i32, i as i32, k as i32);

                    let gu = i - k; // 0..=r
                    for l in ll..=lh {
                        // V[j][l]
                        let a_v = on_bezier_product_coeff(q as i32, s as i32, j as i32, l as i32);

                        let gv = j - l; // 0..=s
                        let scale = f[k][l] * a_u * a_v;

                        // acc += scale * Pw[gu][gv]
                        acc.add_scaled(scale, &self.ctrl[gu][gv]);
                    }
                }

                qw[i][j] = acc;
            }
        }

        Ok(BezierSurface {
            dim: self.dim,
            u_degree: pu,
            v_degree: qv,
            ctrl: qw,
        })
    }

    /// Full-range convenience wrapper.
    pub fn multiply_by_scalar_bezier_function(
        &self,
        f: &[Vec<Real>],
        p: usize,
        q: usize,
    ) -> Result<BezierSurface, NurbsError> {
        let pu = p + self.u_degree;
        let qv = q + self.v_degree;
        self.multiply_by_scalar_bezier_function_range(f, p, q, 0, pu, 0, qv)
    }
}
```
```rust
impl BezierSurface {
    /// Power basis 계수(bw)가 정의된 도메인 [0,1]x[0,1] 을
    /// Bezier control net (Bernstein form) 으로 변환한다.
    ///
    /// bw[i][j] = coefficient of u^i v^j (homogeneous Point4D)
    pub fn from_power_basis_unit_domain_4d(
        bw: &[Vec<Point4D>],
        p: usize,
        q: usize,
    ) -> Result<Self, String> {
        // size check
        if bw.len() != p + 1 {
            return Err(format!(
                "bw row count must be p+1 (got {}, expected {})",
                bw.len(),
                p + 1
            ));
        }
        for (i, row) in bw.iter().enumerate() {
            if row.len() != q + 1 {
                return Err(format!(
                    "bw[{}].len() must be q+1 (got {}, expected {})",
                    i,
                    row.len(),
                    q + 1
                ));
            }
        }

        // basis.rs에 이미 구현되어 있음: (u0,u1,v0,v1) 포함 변환
        let ctrl = on_power_to_bernstein_4d_grid(bw, p, q, 0.0, 1.0, 0.0, 1.0);

        Ok(BezierSurface {
            dim: 3,
            u_degree: p,
            v_degree: q,
            ctrl,
        })
    }
```
```rust
    /// Power basis 계수(bw)가 정의된 도메인 [a,b]x[c,d] 를
    /// Bezier control net (Bernstein form) 으로 변환한다.
    ///
    /// 내부적으로:
    /// 1) u: [a,b] -> [0,1] 로 shift/scale (power 계수 변환)
    /// 2) power -> bernstein (Bezier) 변환
    /// 3) v도 동일
    ///
    /// bw[i][j] = coefficient of u^i v^j (u∈[a,b], v∈[c,d])
    pub fn from_power_basis_4d(
        bw: &[Vec<Point4D>],
        p: usize,
        q: usize,
        a: Real,
        b: Real,
        c: Real,
        d: Real,
    ) -> Result<Self, String> {
        if bw.len() != p + 1 {
            return Err(format!(
                "bw row count must be p+1 (got {}, expected {})",
                bw.len(),
                p + 1
            ));
        }
        for (i, row) in bw.iter().enumerate() {
            if row.len() != q + 1 {
                return Err(format!(
                    "bw[{}].len() must be q+1 (got {}, expected {})",
                    i,
                    row.len(),
                    q + 1
                ));
            }
        }
        if !(a.is_finite() && b.is_finite() && c.is_finite() && d.is_finite()) {
            return Err("domain must be finite".to_string());
        }
        if (b - a) == 0.0 || (d - c) == 0.0 {
            return Err("domain size must be non-zero".to_string());
        }

        let ctrl = on_power_to_bernstein_4d_grid(bw, p, q, a, b, c, d);

        Ok(BezierSurface {
            dim: 3,
            u_degree: p,
            v_degree: q,
            ctrl,
        })
    }
```
```rust
    pub fn from_power_basis_unit_domain(
        bw: &[Vec<Point4D>],
        p: usize,
        q: usize,
        ipu: &[Vec<Real>],
        ipv: &[Vec<Real>],
    ) -> Result<BezierSurface, String> {
        // --- dimension checks ---
        if bw.len() != p + 1 {
            return Err(format!(
                "bw row count must be p+1 (got {}, expected {})",
                bw.len(),
                p + 1
            ));
        }
        for (i, row) in bw.iter().enumerate() {
            if row.len() != q + 1 {
                return Err(format!(
                    "bw[{}].len() must be q+1 (got {}, expected {})",
                    i,
                    row.len(),
                    q + 1
                ));
            }
        }

        if ipu.len() != p + 1 {
            return Err(format!(
                "ipu row count must be p+1 (got {}, expected {})",
                ipu.len(),
                p + 1
            ));
        }
        for (i, row) in ipu.iter().enumerate() {
            if row.len() != p + 1 {
                return Err(format!(
                    "ipu[{}].len() must be p+1 (got {}, expected {})",
                    i,
                    row.len(),
                    p + 1
                ));
            }
        }

        if ipv.len() != q + 1 {
            return Err(format!(
                "ipv row count must be q+1 (got {}, expected {})",
                ipv.len(),
                q + 1
            ));
        }
        for (j, row) in ipv.iter().enumerate() {
            if row.len() != q + 1 {
                return Err(format!(
                    "ipv[{}].len() must be q+1 (got {}, expected {})",
                    j,
                    row.len(),
                    q + 1
                ));
            }
        }

        // ------------------------------------------------------------
        // Step 1: u-direction transform: tmp = ipu * bw
        // tmp[i,j] = sum_k ipu[i,k] * bw[k,j]
        // ------------------------------------------------------------
        let mut tmp = vec![vec![Point4D::zero(); q + 1]; p + 1];

        for i in 0..=p {
            for j in 0..=q {
                let mut acc = Point4D::zero();
                for k in 0..=p {
                    let coef = ipu[i][k];
                    acc += bw[k][j] * coef;
                }
                tmp[i][j] = acc;
            }
        }

        // ------------------------------------------------------------
        // Step 2: v-direction transform: Pw = tmp * ipv^T
        // Pw[i,j] = sum_l tmp[i,l] * ipv[j,l]
        // ------------------------------------------------------------
        let mut pw = vec![vec![Point4D::zero(); q + 1]; p + 1];

        for i in 0..=p {
            for j in 0..=q {
                let mut acc = Point4D::zero();
                for l in 0..=q {
                    let coef = ipv[j][l];
                    acc += tmp[i][l] * coef;
                }
                pw[i][j] = acc;
            }
        }

        Ok(BezierSurface {
            dim: 3,
            u_degree: p,
            v_degree: q,
            ctrl: pw,
        })
    }
}
```
```rust
impl BezierSurface {
    /// Cross product of two Bezier surfaces.
    /// C: B_surcro
    ///
    /// R(u,v) = P(u,v) × Q(u,v)
    /// degree(R) = (p+r, q+s)
    pub fn cross_product_range(
        &self,
        other: &BezierSurface,
        su: usize,
        eu: usize,
        sv: usize,
        ev: usize,
    ) -> Result<BezierSurface, NurbsError> {
        let p = self.u_degree;
        let q = self.v_degree;
        let r = other.u_degree;
        let s = other.v_degree;

        let pu = p + r;
        let qv = q + s;

        if su > eu || eu > pu {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}",
                    su, eu, pu
                ),
            });
        }
        if sv > ev || ev > qv {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}",
                    sv, ev, qv
                ),
            });
        }

        let mut rw = vec![vec![Point4D::zero(); qv + 1]; pu + 1];

        for i in su..=eu {
            let kl = i.saturating_sub(r);
            let kh = (std::cmp::min)(p, i);

            for j in sv..=ev {
                let ll = j.saturating_sub(s);
                let lh = (std::cmp::min)(q, j);

                let mut acc = Point4D::zero();

                for k in kl..=kh {
                    let uik = on_bezier_product_coeff(p as i32, r as i32, i as i32, k as i32);
                    let gu = i - k;

                    for l in ll..=lh {
                        let vjl = on_bezier_product_coeff(q as i32, s as i32, j as i32, l as i32);
                        let gv = j - l;

                        // IMPORTANT: cross는 (x,y,z)만! w는 0이거나 무시되어야 함.
                        let tw = self.ctrl[k][l].cross_p3(&other.ctrl[gu][gv]);
                        acc.add_scaled(uik * vjl, &tw);
                    }
                }

                rw[i][j] = acc;
            }
        }

        Ok(BezierSurface {
            dim: self.dim,
            u_degree: pu,
            v_degree: qv,
            ctrl: rw,
        })
    }
```
```rust
    pub fn cross_product_full(&self, other: &BezierSurface) -> Result<BezierSurface, NurbsError> {
        let pu = self.u_degree + other.u_degree;
        let qv = self.v_degree + other.v_degree;
        self.cross_product_range(other, 0, pu, 0, qv)
    }
}
```
```rust
impl BezierSurface {
    /// Cross product of two Bezier surfaces, using precomputed product matrices.
    pub fn cross_product_with_matrices(
        &self,
        other: &BezierSurface,
        pmu: &[Vec<Real>],
        pmv: &[Vec<Real>],
        su: usize,
        eu: usize,
        sv: usize,
        ev: usize,
    ) -> Result<BezierSurface, String> {
        let p = self.u_degree;
        let q = self.v_degree;
        let r = other.u_degree;
        let s = other.v_degree;

        let rw = bezier_surface_cross_product_with_matrices(
            &self.ctrl,
            p,
            q,
            &other.ctrl,
            r,
            s,
            pmu,
            pmv,
            su,
            eu,
            sv,
            ev,
        )?;

        Ok(BezierSurface {
            dim: 3,
            u_degree: p + r,
            v_degree: q + s,
            ctrl: rw,
        })
    }
```
```rust
    /// full range 버전
    pub fn cross_product_full_with_matrices(
        &self,
        other: &BezierSurface,
        pmu: &[Vec<Real>],
        pmv: &[Vec<Real>],
    ) -> Result<BezierSurface, String> {
        let pu = self.u_degree + other.u_degree;
        let qv = self.v_degree + other.v_degree;

        self.cross_product_with_matrices(other, pmu, pmv, 0, pu, 0, qv)
    }
}
```
```rust
impl BezierSurface {
    pub fn dot_product_with_matrices(
        &self,
        other: &BezierSurface,
        pmu: &[Vec<Real>],
        pmv: &[Vec<Real>],
    ) -> Result<(Vec<Vec<Real>>, Option<Vec<Vec<Real>>>), String> {
        bezier_surface_dot_product_with_matrices(
            &self.ctrl,
            self.u_degree,
            self.v_degree,
            &other.ctrl,
            other.u_degree,
            other.v_degree,
            pmu,
            pmv,
            0,
            self.u_degree + other.v_degree,
            0,
            self.u_degree + other.v_degree,
        )
    }
}
```
```rust
impl BezierSurface {
    /// Extend a Bezier surface strip so that the extension shares
    /// the same homogeneous derivatives as the original strip.
    ///
    /// - dir:
    ///     SurfaceDir::UDir → u 방향이 Bezier, 각 v-column을 curve로 확장
    ///     SurfaceDir::VDir → v 방향이 Bezier, 각 u-row 를 curve로 확장
    ///
    /// - side:
    ///     ExtendSide::Start → u=umin / v=vmin 쪽으로 확장 (B_SEXTSD 의 START)
    ///     ExtendSide::End   → u=umax / v=vmax 쪽으로 확장 (B_SEXTSD 의 END)
    ///
    /// - reverse_param:
    ///     true  → 파라미터 방향 반전 (rev = YES)
    ///     false → 그대로 유지      (rev = NO)
    /// (Piegl) B_sextsd:
    /// Extend a Bezier surface strip so that extension and original share
    /// the same derivatives in homogeneous (4D) space.
    ///
    /// - dir = UDir: treat each v-column as a Bezier curve of degree p
    /// - dir = VDir: treat each u-row    as a Bezier curve of degree q
    ///
    /// side:
    /// - Start: extend at u=0 / v=0
    /// - End  : extend at u=1 / v=1
    ///
    /// reverse_param:
    /// - true  => reverse parametrization (rev=YES)
    /// - false => keep parametrization   (rev=NO)
    pub fn extend_strip_with_same_derivatives(
        &self,
        dir: SurfaceDir,
        side: ExtendSide,
        reverse_param: bool,
    ) -> BezierSurface {
        let p = self.u_degree;
        let q = self.v_degree;

        debug_assert_eq!(self.ctrl.len(), p + 1);
        debug_assert!(self.ctrl.iter().all(|row| row.len() == q + 1));

        let mut n_ctrl = self.ctrl.clone();

        match dir {
            SurfaceDir::UDir => {
                // column buffer size = p+1
                let mut col = vec![Point4D::zero(); p + 1];

                for j in 0..=q {
                    // gather column
                    for i in 0..=p {
                        col[i] = self.ctrl[i][j];
                    }

                    // extend as curve
                    let curve = BezierCurve {
                        dim: self.dim,
                        degree: p,
                        ctrl: col.clone(), // BezierCurve가 Vec를 owning 하므로 여기선 clone 필요
                    };
                    let ext = curve.extend_with_same_derivatives(side, reverse_param);

                    debug_assert_eq!(ext.ctrl.len(), p + 1);

                    // write back
                    for i in 0..=p {
                        n_ctrl[i][j] = ext.ctrl[i];
                    }
                }
            }

            SurfaceDir::VDir => {
                // row buffer size = q+1
                let mut row = vec![Point4D::zero(); q + 1];

                for i in 0..=p {
                    // gather row
                    for j in 0..=q {
                        row[j] = self.ctrl[i][j];
                    }

                    // extend as curve
                    let curve = BezierCurve {
                        dim: self.dim,
                        degree: q,
                        ctrl: row.clone(),
                    };
                    let ext = curve.extend_with_same_derivatives(side, reverse_param);

                    debug_assert_eq!(ext.ctrl.len(), q + 1);

                    // write back
                    for j in 0..=q {
                        n_ctrl[i][j] = ext.ctrl[j];
                    }
                }
            }
        }

        BezierSurface {
            dim: self.dim,
            u_degree: p,
            v_degree: q,
            ctrl: n_ctrl,
        }
    }
}
```
```rust
pub fn on_bezier_tensor_product(cu: &BezierCurve, cv: &BezierCurve) -> BezierSurface {
    let p = cu.degree;
    let q = cv.degree;
    let mut ctrl = vec![vec![Point4D::zero(); q + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=q {
            // tensor product of control points: store as (xyz*w, w)
            let a = cu.ctrl[i];
            let b = cv.ctrl[j];
            ctrl[i][j] = Point4D::from_euclid(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w);
        }
    }
    BezierSurface::with_degrees(p, q, ctrl).unwrap()
}
```
```rust
pub fn on_bezier_surface_dot_coeffs(
    s1: &BezierSurface,
    s2: &BezierSurface,
) -> Option<Vec<Vec<Real>>> {
    if s1.u_degree != s2.u_degree || s1.v_degree != s2.v_degree {
        return None;
    }
    let (m, n) = s1.cv_size();
    let mut out = vec![vec![0.0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let a = &s1.ctrl[i][j];
            let b = &s2.ctrl[i][j];
            out[i][j] = a.x * b.x + a.y * b.y + a.z * b.z; // ignore weights here
        }
    }
    Some(out)
}
```
```rust
pub fn on_bezier_surface_eval_point(
    ctrl: &[Point4D],
    pu: Degree,
    pv: Degree,
    u: Real,
    v: Real,
) -> Point3D {
    let p = pu as usize;
    let q = pv as usize;
    let nu = p + 1;
    let nv = q + 1;
    let bu_vec = on_bernstein_basis(p, u);
    let bv_vec = on_bernstein_basis(q, v);

    let mut nx = 0.0;
    let mut ny = 0.0;
    let mut nz = 0.0;
    let mut dw = 0.0;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let is_rat = ctrl.iter().any(|c| c.w.is_finite());

    for j in 0..nv {
        for i in 0..nu {
            let b = bu_vec[i] * bv_vec[j];
            let c = ctrl[i + nu * j];
            if is_rat && c.w.is_finite() {
                nx += b * c.x * c.w;
                ny += b * c.y * c.w;
                nz += b * c.z * c.w;
                dw += b * c.w;
            } else {
                x += b * c.x;
                y += b * c.y;
                z += b * c.z;
            }
        }
    }
    if dw > 0.0 {
        Point3D {
            x: nx / dw,
            y: ny / dw,
            z: nz / dw,
        }
    } else {
        Point3D { x, y, z }
    }
}
```
```rust
// Homogeneous cross:
// P = p'/w1, Q = q'/w2 => P x Q = (p' x q')/(w1*w2)
// store as (p' x q', w1*w2)
pub fn on_bezier_surf_cross_homogeneous(a: &Point4D, b: &Point4D) -> Point4D {
    let p = Point3D::new(a.x, a.y, a.z);
    let q = Point3D::new(b.x, b.y, b.z);
    let c = p.cross_pt(&q);
    Point4D::from_euclid(c.x, c.y, c.z, a.w * b.w)
}
```
```rust
// cross product of two Bezier surfaces (3D)
pub fn on_bezier_surface_cross_product(
    s1: &BezierSurface,
    s2: &BezierSurface,
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Option<BezierSurface> {
    if s1.dim != 3 || s2.dim != 3 {
        return None;
    }

    let p = s1.u_degree;
    let q = s1.v_degree;
    let r = s2.u_degree;
    let s = s2.v_degree;

    let nu1 = s1.cv_count(0); // = p+1
    let nv1 = s1.cv_count(1); // = q+1
    let nu2 = s2.cv_count(0); // = r+1
    let nv2 = s2.cv_count(1); // = s+1
    if nu1 != p + 1 || nv1 != q + 1 || nu2 != r + 1 || nv2 != s + 1 {
        return None;
    }

    let r_u = p + r;
    let r_v = q + s;

    let mut surf = BezierSurface::create_only(r_u, r_v);

    // init CVs to zero
    for i in 0..=r_u {
        for j in 0..=r_v {
            surf.set_cv(i, j, Point4D::zero());
        }
    }

    let mut iu0 = su;
    let mut iu1 = eu;
    let mut iv0 = sv;
    let mut iv1 = ev;

    iu0 = 0.max(iu0);
    iu1 = r_u.min(iu1);
    iv0 = 0.max(iv0);
    iv1 = r_v.min(iv1);

    if iu0 > iu1 || iv0 > iv1 {
        return None;
    }

    for i in iu0..=iu1 {
        let kl = 0.max(i as i32 - r as i32) as usize;
        let kh = p.min(i);
        for j in iv0..=iv1 {
            let ll = 0.max(j as i32 - s as i32) as usize;
            let lh = q.min(j);

            let mut rij = Point4D::zero();

            for k in kl..=kh {
                let a_u = on_bezier_product_coeff(p as i32, r as i32, i as i32, k as i32);
                for l in ll..=lh {
                    let a_v = on_bezier_product_coeff(q as i32, s as i32, j as i32, l as i32);
                    let pkl = on_bezier_surface_cv_point4d(s1, k, l)?;
                    let qij = on_bezier_surface_cv_point4d(s2, i - k, j - l)?;
                    let t = on_bezier_surf_cross_homogeneous(&pkl, &qij);
                    let a = a_u * a_v;
                    rij.x += a * t.x;
                    rij.y += a * t.y;
                    rij.z += a * t.z;
                    rij.w += a * t.w;
                }
            }
            surf.set_cv(i, j, rij);
        }
    }

    Some(surf)
}
```
```rust
pub fn on_elevate_surface_row_col(
    pw: &Vec<Vec<Point4D>>,     // original control net
    r: usize,                   // original degree
    t: usize,                   // increment
    dm: &Vec<Vec<Real>>,        // degree elevation matrix (size (r+t+1) x (r+1))
    dir: SurfaceDir,            // UDir or VDir
    f: usize,                   // first index
    l: usize,                   // last index
    roc: usize,                 // row or column index
    qw: &mut Vec<Vec<Point4D>>, // output control net
) {
    match dir {
        SurfaceDir::UDir => {
            // Elevate in u-direction: operate on column "roc"
            for i in f..=l {
                let a = if i >= t { i - t } else { 0 };
                let b = r.min(i);

                let mut acc = Point4D::zero();
                for k in a..=b {
                    let w = dm[i][k];
                    acc.x += w * pw[k][roc].x;
                    acc.y += w * pw[k][roc].y;
                    acc.z += w * pw[k][roc].z;
                    acc.w += w * pw[k][roc].w;
                }
                qw[i][roc] = acc;
            }
        }

        SurfaceDir::VDir => {
            // Elevate in v-direction: operate on row "roc"
            for j in f..=l {
                let a = if j >= t { j - t } else { 0 };
                let b = r.min(j);

                let mut acc = Point4D::zero();
                for k in a..=b {
                    let w = dm[j][k];
                    acc.x += w * pw[roc][k].x;
                    acc.y += w * pw[roc][k].y;
                    acc.z += w * pw[roc][k].z;
                    acc.w += w * pw[roc][k].w;
                }
                qw[roc][j] = acc;
            }
        }
    }
}
```
```rust
pub fn on_elevate_row_col_to_surface(
    surf: &BezierSurface,
    dir: SurfaceDir, // UDir or VDir
    inc: usize,      // degree increment
    f: usize,        // first index
    l: usize,        // last index
    roc: usize,      // row or column index
) -> BezierSurface {
    let (p, q) = (surf.u_degree, surf.v_degree);

    // degree elevation matrix
    let r = match dir {
        SurfaceDir::UDir => p,
        SurfaceDir::VDir => q,
    };
    let dm = on_degree_elevation_matrix(r, inc);

    // new control net size
    let new_p = if dir == SurfaceDir::UDir { p + inc } else { p };
    let new_q = if dir == SurfaceDir::VDir { q + inc } else { q };

    // initialize new control net
    let mut new_ctrl = vec![vec![Point4D::zero(); new_q + 1]; new_p + 1];

    // copy original control points first
    for i in 0..=p {
        for j in 0..=q {
            new_ctrl[i][j] = surf.ctrl[i][j];
        }
    }

    // apply row/column elevation
    match dir {
        SurfaceDir::UDir => {
            // elevate column "roc"
            for i in f..=l {
                let a = if i >= inc { i - inc } else { 0 };
                let b = r.min(i);

                let mut acc = Point4D::zero();
                for k in a..=b {
                    let w = dm[i][k];
                    acc.x += w * surf.ctrl[k][roc].x;
                    acc.y += w * surf.ctrl[k][roc].y;
                    acc.z += w * surf.ctrl[k][roc].z;
                    acc.w += w * surf.ctrl[k][roc].w;
                }
                new_ctrl[i][roc] = acc;
            }
        }

        SurfaceDir::VDir => {
            // elevate row "roc"
            for j in f..=l {
                let a = if j >= inc { j - inc } else { 0 };
                let b = r.min(j);

                let mut acc = Point4D::zero();
                for k in a..=b {
                    let w = dm[j][k];
                    acc.x += w * surf.ctrl[roc][k].x;
                    acc.y += w * surf.ctrl[roc][k].y;
                    acc.z += w * surf.ctrl[roc][k].z;
                    acc.w += w * surf.ctrl[roc][k].w;
                }
                new_ctrl[roc][j] = acc;
            }
        }
    }

    // return as BezierSurface
    BezierSurface {
        dim: 3,
        u_degree: new_p,
        v_degree: new_q,
        ctrl: new_ctrl,
    }
}
```
```rust
/// Bernstein(Beziér) -> Power 변환 행렬 (degree p)
/// a[k] = Σ_i M[k][i] * b[i]
pub fn on_bernstein_to_power_matrix(p: usize) -> Vec<Vec<Real>> {
    let mut m = vec![vec![0.0; p + 1]; p + 1];
    for k in 0..=p {
        for i in 0..=k {
            let sign = if ((k - i) & 1) == 1 { -1.0 } else { 1.0 };
            m[k][i] = on_binomial_real(p, i) * on_binomial_real(p - i, k - i) * sign;
        }
    }
    m
}
```
```rust
/// 행렬 곱: C = A * B
fn on_mat_mul_1d(a: &[Vec<Real>], b: &[Vec<Real>]) -> Vec<Vec<Real>> {
    let n = a.len();
    let m = b[0].len();
    let k = b.len();
    let mut c = vec![vec![0.0; m]; n];
    for i in 0..n {
        for kk in 0..k {
            let aik = a[i][kk];
            if aik == 0.0 {
                continue;
            }
            for j in 0..m {
                c[i][j] += aik * b[kk][j];
            }
        }
    }
    c
}
```
```rust
/// 2D 변환: bw = Cu * Pw * Cv^T
/// Pw[u][v]  (u:0..p, v:0..q)
pub fn on_bezier_surface_to_power_basis_4d(
    pw: &[Vec<Point4D>], // (p+1) x (q+1)
    p: usize,
    q: usize,
    a: Real,
    b: Real, // u-domain
    c: Real,
    d: Real, // v-domain
) -> Vec<Vec<Point4D>> {
    assert_eq!(pw.len(), p + 1);
    assert_eq!(pw[0].len(), q + 1);

    // 1) Bernstein->Power (on [0,1]) matrices
    let pum = on_bernstein_to_power_matrix(p);
    let pvm = on_bernstein_to_power_matrix(q);

    // 2) Reparam matrices:
    // 원하는 출력 power basis가 [a,b]에서 u로 직접 평가되는 형태라면
    // t∈[0,1] -> u∈[a,b] 로 치환해야 하므로 (0,1,a,b)
    let rum = on_reparam_matrix_power_basis(p, 0.0, 1.0, a, b).expect("Invalid Rum");
    let rvm = on_reparam_matrix_power_basis(q, 0.0, 1.0, c, d).expect("Invalid Rum");

    // 3) Conversion matrices: Cu = Rum * Pum, Cv = Rvm * Pvm
    let cu = on_mat_mul_1d(&rum, &pum);
    let cv = on_mat_mul_1d(&rvm, &pvm);

    // 4) bw = cu * pw * cv^T
    // 먼저 tmp[u_power][v] = Σ_u cu[u_power][u] * pw[u][v]
    let mut tmp = vec![vec![Point4D::zero(); q + 1]; p + 1];
    for up in 0..=p {
        for u0 in 0..=p {
            let coef = cu[up][u0];
            if coef == 0.0 {
                continue;
            }
            for v in 0..=q {
                tmp[up][v].x += coef * pw[u0][v].x;
                tmp[up][v].y += coef * pw[u0][v].y;
                tmp[up][v].z += coef * pw[u0][v].z;
                tmp[up][v].w += coef * pw[u0][v].w;
            }
        }
    }

    // bw[up][vp] = Σ_v cv[vp][v] * tmp[up][v]
    let mut bw = vec![vec![Point4D::zero(); q + 1]; p + 1];
    for up in 0..=p {
        for vp in 0..=q {
            let mut acc = Point4D::zero();
            for v0 in 0..=q {
                let coef = cv[vp][v0];
                if coef == 0.0 {
                    continue;
                }
                acc.x += coef * tmp[up][v0].x;
                acc.y += coef * tmp[up][v0].y;
                acc.z += coef * tmp[up][v0].z;
                acc.w += coef * tmp[up][v0].w;
            }
            bw[up][vp] = acc;
        }
    }
    bw
}
```
```rust
/// Bezier surface의 한 row/column만 degree elevation.
/// pw  : 원래 control net [u][v]
/// r   : 올릴 방향의 원래 degree (u 또는 v)
/// t   : 증가량 (new degree = r + t)
/// dm  : degree elevation matrix, size = (r+t+1) x (r+1)
/// dir : UDir / VDir
/// f,l : new index range (부분 계산 지원)
/// roc : 다른 방향 index (UDir면 v-index(column), VDir면 u-index(row))
/// qw  : output control net (이미 new size로 할당되어 있어야 함)
pub fn elevate_surface_row_col_kernel(
    pw: &[Vec<Point4D>],
    r: usize,
    t: usize,
    dm: &[Vec<Real>],
    dir: SurfaceDir,
    f: usize,
    l: usize,
    roc: usize,
    qw: &mut [Vec<Point4D>],
) {
    // 기본 방어: 인덱스/크기 최소 체크
    if t == 0 {
        // t==0이면 사실상 복사인데, 이 커널은 "한 줄만 계산"이 목적이라
        // 호출자가 전체 복사/처리를 결정하도록 그냥 리턴해도 된다.
        // (원하면 여기서도 그대로 pw->qw 복사 가능)
    }

    assert!(dm.len() >= r + t + 1);
    for i in 0..=(r + t) {
        assert!(dm[i].len() >= r + 1);
    }

    match dir {
        SurfaceDir::UDir => {
            // u 방향 degree elevation: column `roc`만 계산
            for i in f..=l {
                let a = if i >= t { i - t } else { 0 };
                let b = r.min(i);

                let mut acc = Point4D::zero();
                for k in a..=b {
                    let w = dm[i][k];
                    acc.add_scaled(w, &pw[k][roc]);
                }
                qw[i][roc] = acc;
            }
        }
        SurfaceDir::VDir => {
            // v 방향 degree elevation: row `roc`만 계산
            for j in f..=l {
                let a = if j >= t { j - t } else { 0 };
                let b = r.min(j);

                let mut acc = Point4D::zero();
                for k in a..=b {
                    let w = dm[j][k];
                    acc.add_scaled(w, &pw[roc][k]);
                }
                qw[roc][j] = acc;
            }
        }
    }
}
```
```rust
pub fn bezier_surface_degree_elevate_row(
    pw: &[Vec<Point4D>],
    p: usize,
    t: usize,
    row: usize,
    elev_mat: &[Vec<Real>],
    out: &mut [Vec<Point4D>],
) {
    let new_p = p + t;

    for i in 0..=new_p {
        let mut acc = Point4D::zero();
        let i_min = i.saturating_sub(t);
        let i_max = p.min(i);

        for k in i_min..=i_max {
            let a = elev_mat[i][k];
            acc.x += a * pw[k][row].x;
            acc.y += a * pw[k][row].y;
            acc.z += a * pw[k][row].z;
            acc.w += a * pw[k][row].w;
        }

        out[i][row] = acc;
    }
}
```
```rust
// 관계식: P_i = (i/p) Q_{i-1} + (1 - i/p) Q_i  (i=1..p-1)
//
// forward (left->right):
//   Q_i = (p/(p-i)) P_i + (-(i)/(p-i)) Q_{i-1}
//   => alf[i] = p/(p-i),  oma[i] = -i/(p-i)
//
// backward (right->left):
//   P_{i+1} = ((i+1)/p) Q_i + (1-(i+1)/p) Q_{i+1}
//   Q_i = (p/(i+1)) P_{i+1} + (-(p-i-1)/(i+1)) Q_{i+1}
//   => bet[i] = p/(i+1), omb[i] = -(p-i-1)/(i+1)
pub fn on_degree_reduction_coeffs(p: usize) -> (Vec<Real>, Vec<Real>, Vec<Real>, Vec<Real>) {
    assert!(p >= 2);

    let mut alf = vec![0.0; p];
    let mut oma = vec![0.0; p];
    let mut bet = vec![0.0; p];
    let mut omb = vec![0.0; p];

    let pf = p as f64;

    // forward: i = 1..=p-1 (p-i != 0 except i=p)
    for i in 1..=p - 1 {
        let denom = (p - i) as f64;
        if denom != 0.0 {
            alf[i] = pf / denom;
            oma[i] = -(i as f64) / denom;
        }
    }

    // backward: i = 0..=p-2 (uses P_{i+1})
    for i in 0..=p - 2 {
        let denom = (i + 1) as f64;
        bet[i] = pf / denom;
        omb[i] = -((p - i - 1) as f64) / denom;
    }

    (alf, oma, bet, omb)
}
```
```rust
// ------------------------------------------------------------
// Small helper: linear combination using existing Point4D
// out = wa*A + wb*B
// ------------------------------------------------------------
#[inline]
fn on_lin_comb_point4d(a: &Point4D, wa: Real, b: &Point4D, wb: Real) -> Point4D {
    let mut out = Point4D::default();
    out.add_scaled(wa, a);
    out.add_scaled(wb, b);
    out
}
```
```rust
fn reduce_row_col_kernel(
    pw: &[Vec<Point4D>],
    p: usize,
    q: usize,
    dir: SurfaceDir,
    k: usize,
    alf: &[Real],
    oma: &[Real],
    bet: &[Real],
    omb: &[Real],
    qw: &mut [Vec<Point4D>],
) -> Real {
    let mut e = 0.0;

    match dir {
        SurfaceDir::UDir => {
            // reduce u degree: p -> p-1 on column k (v-index)
            assert!(p >= 2);
            assert!(k <= q);
            assert!(qw.len() == p);
            debug_assert!(qw.iter().all(|row| row.len() == q + 1));

            // endpoints
            qw[0][k] = pw[0][k];
            qw[p - 1][k] = pw[p][k];

            let r = (p - 1) / 2;

            if p % 2 == 1 {
                // odd degree (requires p>=3 => r>=1)
                assert!(r >= 1);

                // left: i=1..r-1
                if r >= 2 {
                    for i in 1..=r - 1 {
                        qw[i][k] = on_lin_comb_point4d(&pw[i][k], alf[i], &qw[i - 1][k], oma[i]);
                    }
                }

                // right: i=p-2..r+1
                if p >= 3 && r + 1 <= p - 2 {
                    for i in (r + 1..=p - 2).rev() {
                        // Q_i = bet[i]*P_{i+1} + omb[i]*Q_{i+1}
                        qw[i][k] =
                            on_lin_comb_point4d(&pw[i + 1][k], bet[i], &qw[i + 1][k], omb[i]);
                    }
                }

                // middle
                let pl = on_lin_comb_point4d(&pw[r][k], alf[r], &qw[r - 1][k], oma[r]);
                let pr = on_lin_comb_point4d(&pw[r + 1][k], bet[r], &qw[r + 1][k], omb[r]);
                qw[r][k] = on_lin_comb_point4d(&pl, 0.5, &pr, 0.5);

                // error
                let u = 0.5 * (1.0 - (1.0 / (p as f64)).sqrt());
                let b = on_bernstein(p, r, u);
                let b1 = on_bernstein(p, r + 1, u);
                let dw = Point4D::distance(&pl, &pr);

                let a = 0.5 * ((p - r) as f64) / (p as f64);
                e = a * (b - b1).abs() * dw;
            } else {
                // even degree
                // left: i=1..r
                for i in 1..=r {
                    qw[i][k] = on_lin_comb_point4d(&pw[i][k], alf[i], &qw[i - 1][k], oma[i]);
                }

                // right: i=p-2..r+1
                if p >= 3 && r + 1 <= p - 2 {
                    for i in (r + 1..=p - 2).rev() {
                        qw[i][k] =
                            on_lin_comb_point4d(&pw[i + 1][k], bet[i], &qw[i + 1][k], omb[i]);
                    }
                }

                // error
                let u = (r as f64 + 1.0) / (p as f64);
                let b1 = on_bernstein(p, r + 1, u);

                let pl = on_lin_comb_point4d(&qw[r][k], 0.5, &qw[r + 1][k], 0.5);
                let dw = Point4D::distance(&pw[r + 1][k], &pl);

                e = b1 * dw;
            }
        }

        SurfaceDir::VDir => {
            // reduce v degree: q -> q-1 on row k (u-index)
            assert!(q >= 2);
            assert!(k <= p);
            assert!(qw.len() == p + 1);
            debug_assert!(qw.iter().all(|row| row.len() == q));

            // endpoints
            qw[k][0] = pw[k][0];
            qw[k][q - 1] = pw[k][q];

            let r = (q - 1) / 2;

            if q % 2 == 1 {
                // odd degree (requires q>=3 => r>=1)
                assert!(r >= 1);

                // left: j=1..r-1
                if r >= 2 {
                    for j in 1..=r - 1 {
                        qw[k][j] = on_lin_comb_point4d(&pw[k][j], alf[j], &qw[k][j - 1], oma[j]);
                    }
                }

                // right: j=q-2..r+1
                if q >= 3 && r + 1 <= q - 2 {
                    for j in (r + 1..=q - 2).rev() {
                        qw[k][j] =
                            on_lin_comb_point4d(&pw[k][j + 1], bet[j], &qw[k][j + 1], omb[j]);
                    }
                }

                // middle
                let pl = on_lin_comb_point4d(&pw[k][r], alf[r], &qw[k][r - 1], oma[r]);
                let pr = on_lin_comb_point4d(&pw[k][r + 1], bet[r], &qw[k][r + 1], omb[r]);
                qw[k][r] = on_lin_comb_point4d(&pl, 0.5, &pr, 0.5);

                // error
                let u = 0.5 * (1.0 - (1.0 / (q as f64)).sqrt());
                let b = on_bernstein(q, r, u);
                let b1 = on_bernstein(q, r + 1, u);
                let dw = Point4D::distance(&pl, &pr);

                let a = 0.5 * ((q - r) as f64) / (q as f64);
                e = a * (b - b1).abs() * dw;
            } else {
                // even degree
                // left: j=1..r
                for j in 1..=r {
                    qw[k][j] = on_lin_comb_point4d(&pw[k][j], alf[j], &qw[k][j - 1], oma[j]);
                }

                // right: j=q-2..r+1
                if q >= 3 && r + 1 <= q - 2 {
                    for j in (r + 1..=q - 2).rev() {
                        qw[k][j] =
                            on_lin_comb_point4d(&pw[k][j + 1], bet[j], &qw[k][j + 1], omb[j]);
                    }
                }

                // error
                let u = (r as f64 + 1.0) / (q as f64);
                let b1 = on_bernstein(q, r + 1, u);

                let pl = on_lin_comb_point4d(&qw[k][r], 0.5, &qw[k][r + 1], 0.5);
                let dw = Point4D::distance(&pw[k][r + 1], &pl);

                e = b1 * dw;
            }
        }
    }

    e
}
```
```rust
/// Bezier surface degree reduction for a single row/column.
///
/// - `pw`: original control net (size (p+1) x (q+1))
/// - `p, q`: degrees in u, v
/// - `dir`: UDir → reduce in u, VDir → reduce in v
/// - `k`: row/column index (depending on dir)
/// - `alf, oma, bet, omb`: precomputed reduction coefficients (from B_degrco-like routine)
/// - `qw`: output control net after reduction
///
/// Returns: maximum reduction error `e` for that row/column.
pub fn on_bezier_surface_degree_reduce_row_col(
    pw: &[Vec<Point4D>],
    p: usize,
    q: usize,
    dir: SurfaceDir,
    k: usize,
    alf: &[Real],
    oma: &[Real],
    bet: &[Real],
    omb: &[Real],
    qw: &mut [Vec<Point4D>],
) -> Real {
    assert!(dir == SurfaceDir::UDir || dir == SurfaceDir::VDir);

    let mut e = 0.0;

    match dir {
        SurfaceDir::UDir => {
            // reduce degree in u-direction, work on column k
            // Pw: [0..=p][0..=q], Qw: [0..=p-1][0..=q]
            // 1) end points
            qw[0][k] = pw[0][k]; // A_initcp
            qw[p - 1][k] = pw[p][k];

            // 2) main reduction
            let r = (p - 1) / 2;

            if p % 2 == 1 {
                // ---- odd degree ----
                // from the left
                if r >= 2 {
                    for i in 1..=r - 1 {
                        let a = alf[i];
                        let b = oma[i];
                        let pw_i = pw[i][k];
                        let qw_im1 = qw[i - 1][k];
                        qw[i][k] = Point4D::linear_comb(pw_i, a, qw_im1, b);
                    }
                }

                // from the right
                if p >= 3 && r + 1 <= p - 2 {
                    for i in (r + 1..=p - 2).rev() {
                        let a = bet[i];
                        let b = omb[i];
                        let pw_ip1 = pw[i + 1][k];
                        let qw_ip1 = qw[i + 1][k];
                        qw[i][k] = Point4D::linear_comb(pw_ip1, a, qw_ip1, b);
                    }
                }

                // middle control point
                let pw_r = pw[r][k];
                let pw_r1 = pw[r + 1][k];
                let qw_rm1 = qw[r - 1][k];
                let qw_rp1 = qw[r + 1][k];

                let pl = Point4D::linear_comb(pw_r, alf[r], qw_rm1, oma[r]);
                let pr = Point4D::linear_comb(pw_r1, bet[r], qw_rp1, omb[r]);
                qw[r][k] = Point4D::linear_comb(pl, 0.5, pr, 0.5);

                // error
                let u = 0.5 * (1.0 - (1.0 / (p as f64)).sqrt());
                let b = on_bernstein(p, r, u);
                let b1 = on_bernstein(p, r + 1, u);
                let dw = Point4D::distance(&pl, &pr);

                let a = 0.5 * (p - r) as f64 / (p as f64);
                let diff = (b - b1).abs();
                e = a * diff * dw;
            } else {
                // ---- even degree ----
                // from the left
                for i in 1..=r {
                    let a = alf[i];
                    let b = oma[i];
                    let pw_i = pw[i][k];
                    let qw_im1 = qw[i - 1][k];
                    qw[i][k] = Point4D::linear_comb(pw_i, a, qw_im1, b);
                }

                // from the right
                if p >= 3 && r + 1 <= p - 2 {
                    for i in (r + 1..=p - 2).rev() {
                        let a = bet[i];
                        let b = omb[i];
                        let pw_ip1 = pw[i + 1][k];
                        let qw_ip1 = qw[i + 1][k];
                        qw[i][k] = Point4D::linear_comb(pw_ip1, a, qw_ip1, b);
                    }
                }

                // error
                let u = (r as f64 + 1.0) / (p as f64);
                let b1 = on_bernstein(p, r + 1, u);

                let pl = Point4D::linear_comb(qw[r][k], 0.5, qw[r + 1][k], 0.5);
                let dw = Point4D::distance(&pw[r + 1][k], &pl);

                e = b1 * dw;
            }
        }

        SurfaceDir::VDir => {
            // reduce degree in v-direction, work on row k
            // Pw: [0..=p][0..=q], Qw: [0..=p][0..=q-1]
            // 1) end points
            qw[k][0] = pw[k][0];
            qw[k][q - 1] = pw[k][q];

            let r = (q - 1) / 2;

            if q % 2 == 1 {
                // ---- odd degree ----
                // from the left
                if r >= 2 {
                    for j in 1..=r - 1 {
                        let a = alf[j];
                        let b = oma[j];
                        let pw_kj = pw[k][j];
                        let qw_kjm1 = qw[k][j - 1];
                        qw[k][j] = Point4D::linear_comb(pw_kj, a, qw_kjm1, b);
                    }
                }

                // from the right
                if q >= 3 && r + 1 <= q - 2 {
                    for j in (r + 1..=q - 2).rev() {
                        let a = bet[j];
                        let b = omb[j];
                        let pw_kjp1 = pw[k][j + 1];
                        let qw_kjp1 = qw[k][j + 1];
                        qw[k][j] = Point4D::linear_comb(pw_kjp1, a, qw_kjp1, b);
                    }
                }

                // middle control point
                let pw_kr = pw[k][r];
                let pw_kr1 = pw[k][r + 1];
                let qw_krm1 = qw[k][r - 1];
                let qw_krp1 = qw[k][r + 1];

                let pl = Point4D::linear_comb(pw_kr, alf[r], qw_krm1, oma[r]);
                let pr = Point4D::linear_comb(pw_kr1, bet[r], qw_krp1, omb[r]);
                qw[k][r] = Point4D::linear_comb(pl, 0.5, pr, 0.5);

                // error
                let u = 0.5 * (1.0 - (1.0 / (q as f64)).sqrt());
                let b = on_bernstein(q, r, u);
                let b1 = on_bernstein(q, r + 1, u);
                let dw = Point4D::distance(&pl, &pr);

                let a = 0.5 * (q - r) as f64 / (q as f64);
                let diff = (b - b1).abs();
                e = a * diff * dw;
            } else {
                // ---- even degree ----
                // from the left
                for j in 1..=r {
                    let a = alf[j];
                    let b = oma[j];
                    let pw_kj = pw[k][j];
                    let qw_kjm1 = qw[k][j - 1];
                    qw[k][j] = Point4D::linear_comb(pw_kj, a, qw_kjm1, b);
                }

                // from the right
                if q >= 3 && r + 1 <= q - 2 {
                    for j in (r + 1..=q - 2).rev() {
                        let a = bet[j];
                        let b = omb[j];
                        let pw_kjp1 = pw[k][j + 1];
                        let qw_kjp1 = qw[k][j + 1];
                        qw[k][j] = Point4D::linear_comb(pw_kjp1, a, qw_kjp1, b);
                    }
                }

                // error
                let u = (r as f64 + 1.0) / (q as f64);
                let b1 = on_bernstein(q, r + 1, u);

                let pl = Point4D::linear_comb(qw[k][r], 0.5, qw[k][r + 1], 0.5);
                let dw = Point4D::distance(&pw[k][r + 1], &pl);

                e = b1 * dw;
            }
        }
    }

    e
}
```
```rust
/// Elevate the degree of a Bezier *surface function* (scalar control values)
/// for a single row/column, using a precomputed degree elevation matrix.
///
/// Parameters:
/// - fp  : original control values, size = (r+1) x N  (UDir) or N x (r+1) (VDir)
/// - r   : original degree in the elevated direction
/// - t   : increment (new degree = r + t)
/// - rm  : degree elevation matrix, size = (r+t+1) x (r+1)
/// - dir : SurfaceDir::UDir or SurfaceDir::VDir
/// - f,l : first and last indices in the elevated direction to compute (inclusive)
/// - roc : row or column index orthogonal to the elevated direction
/// - fq  : output control values, must have size (r+t+1) x N (UDir) or N x (r+t+1) (VDir)
pub fn on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
    fp: &[Vec<Real>],
    r: usize,
    t: usize,
    rm: &[Vec<Real>],
    dir: SurfaceDir,
    f: usize,
    l: usize,
    roc: usize,
    fq: &mut [Vec<Real>],
) {
    let new_deg = r + t;

    debug_assert_eq!(rm.len(), new_deg + 1);
    debug_assert!(rm.iter().all(|row| row.len() == r + 1));
    debug_assert!(f <= l);
    debug_assert!(l <= new_deg);

    match dir {
        SurfaceDir::UDir => {
            // fp: (r+1) x M, fq: (new_deg+1) x M, column=roc 고정
            debug_assert!(fp.len() >= r + 1);
            debug_assert!(fq.len() >= new_deg + 1);
            debug_assert!(fp.iter().all(|row| roc < row.len()));
            debug_assert!(fq.iter().all(|row| roc < row.len()));

            for i in f..=l {
                let a = if i > t { i - t } else { 0 };
                let b = if i > r { r } else { i };

                let mut sum = 0.0;
                for k in a..=b {
                    sum += rm[i][k] * fp[k][roc];
                }
                fq[i][roc] = sum;
            }
        }

        SurfaceDir::VDir => {
            // fp: N x (r+1), fq: N x (new_deg+1), row=roc 고정
            debug_assert!(roc < fp.len());
            debug_assert!(roc < fq.len());
            debug_assert!(fp[roc].len() >= r + 1);
            debug_assert!(fq[roc].len() >= new_deg + 1);

            for j in f..=l {
                let a = if j > t { j - t } else { 0 };
                let b = if j > r { r } else { j };

                let mut sum = 0.0;
                for k in a..=b {
                    sum += rm[j][k] * fp[roc][k];
                }
                fq[roc][j] = sum;
            }
        }
    }
}
```
```rust
/// degree elevation matrix를 내부에서 생성해서 한 row/column만 올림.
///
/// - fp  : (old_deg+1) x N (UDir) or N x (old_deg+1) (VDir)
/// - old_deg : r
/// - t   : increment
/// - dir : elev dir
/// - f,l : 범위 (i 또는 j)
/// - roc : row / column index
/// - fq  : (old_deg+t+1) x N or N x (old_deg+t+1)
pub fn on_bezier_surface_function_degree_elevate_rowcol(
    fp: &[Vec<Real>],
    old_deg: usize,
    t: usize,
    dir: SurfaceDir,
    f: usize,
    l: usize,
    roc: usize,
    fq: &mut [Vec<Real>],
) {
    let rm = on_degree_elevation_matrix(old_deg, t);
    on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
        fp, old_deg, t, &rm, dir, f, l, roc, fq,
    );
}

// product matrix를 (degA+degB+1) x (degA+1)로 만든다.
/// U: (p+r+1) x (p+1), V: (q+s+1) x (q+1)
pub fn on_build_product_matrix(deg_a: usize, deg_b: usize) -> Vec<Vec<Real>> {
    let new_deg = deg_a + deg_b;
    let mut m = vec![vec![0.0; deg_a + 1]; new_deg + 1];
    for i in 0..=new_deg {
        // k range는 실제론 max(0,i-deg_b)..min(deg_a,i)만 유효하지만
        // 행렬은 전체 채워도 됨(0이거나 on_product_matrix가 0 주면 OK)
        for k in 0..=deg_a {
            m[i][k] = on_product_matrix(deg_a, deg_b, i, k);
        }
    }
    m
}
```
```rust
pub fn on_build_product_matrix_u(p: usize, r: usize) -> Vec<Vec<Real>> {
    let pu = p + r;
    let mut pmu = vec![vec![0.0_f64; p + 1]; pu + 1];
    for i in 0..=pu {
        for k in 0..=p {
            pmu[i][k] = on_product_matrix(p, r, i, k);
        }
    }
    pmu
}
```
```rust
pub fn on_build_product_matrix_v(q: usize, s: usize) -> Vec<Vec<Real>> {
    let qv = q + s;
    let mut pmv = vec![vec![0.0_f64; q + 1]; qv + 1];
    for j in 0..=qv {
        for l in 0..=q {
            pmv[j][l] = on_product_matrix(q, s, j, l);
        }
    }
    pmv
}
```
```rust
pub fn on_bezier_surface_function_product_range_with_matrices(
    f: &[Vec<Real>],
    p: usize,
    q: usize,
    g: &[Vec<Real>],
    r: usize,
    s: usize,
    u_mat: &[Vec<Real>], // (p+r+1) x (p+1)
    v_mat: &[Vec<Real>], // (q+s+1) x (q+1)
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<Real>>, NurbsError> {
    // --- dimension checks (네 스타일 유지) ---
    if f.len() != p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "f row count must be p+1 (got {}, expected {})",
                f.len(),
                p + 1
            ),
        });
    }
    for (i, row) in f.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "f[{}].len() must be q+1 (got {}, expected {})",
                    i,
                    row.len(),
                    q + 1
                ),
            });
        }
    }

    if g.len() != r + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "g row count must be r+1 (got {}, expected {})",
                g.len(),
                r + 1
            ),
        });
    }
    for (i, row) in g.iter().enumerate() {
        if row.len() != s + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "g[{}].len() must be s+1 (got {}, expected {})",
                    i,
                    row.len(),
                    s + 1
                ),
            });
        }
    }

    let pu = p + r;
    let qv = q + s;

    if su > eu || eu > pu {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}",
                su, eu, pu
            ),
        });
    }
    if sv > ev || ev > qv {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}",
                sv, ev, qv
            ),
        });
    }

    // matrix size check
    if u_mat.len() != pu + 1 || u_mat.iter().any(|row| row.len() != p + 1) {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "u_mat size must be (p+r+1)x(p+1) = ({})x({})",
                pu + 1,
                p + 1
            ),
        });
    }
    if v_mat.len() != qv + 1 || v_mat.iter().any(|row| row.len() != q + 1) {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "v_mat size must be (q+s+1)x(q+1) = ({})x({})",
                qv + 1,
                q + 1
            ),
        });
    }

    let mut fg = vec![vec![0.0_f64; qv + 1]; pu + 1];

    for i in su..=eu {
        for j in sv..=ev {
            let kl = i.saturating_sub(r);
            let kh = (std::cmp::min)(p, i);
            let ll = j.saturating_sub(s);
            let lh = (std::cmp::min)(q, j);

            let mut val = 0.0;

            // 살짝 미세 최적화: u_mat row를 미리 잡아두기
            let urow = &u_mat[i];
            let vrow = &v_mat[j];

            for k in kl..=kh {
                let uik = urow[k];
                // g_u = i-k 는 0..=r 로 보장됨
                let gu = i - k;

                for l in ll..=lh {
                    let vjl = vrow[l];
                    let gv = j - l; // 0..=s 보장
                    val += uik * vjl * f[k][l] * g[gu][gv];
                }
            }

            fg[i][j] = val;
        }
    }

    Ok(fg)
}
```
```rust
pub fn on_bezier_surface_function_product_range(
    f: &[Vec<Real>],
    p: usize,
    q: usize,
    g: &[Vec<Real>],
    r: usize,
    s: usize,
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<Real>>, NurbsError> {
    let u_mat = on_build_product_matrix(p, r);
    let v_mat = on_build_product_matrix(q, s);

    on_bezier_surface_function_product_range_with_matrices(
        f, p, q, g, r, s, &u_mat, &v_mat, su, eu, sv, ev,
    )
}
```
```rust
/// full range convenience
pub fn on_bezier_surface_function_product(
    f: &[Vec<Real>],
    p: usize,
    q: usize,
    g: &[Vec<Real>],
    r: usize,
    s: usize,
) -> Result<Vec<Vec<Real>>, NurbsError> {
    let pu = p + r;
    let qv = q + s;
    on_bezier_surface_function_product_range(f, p, q, g, r, s, 0, pu, 0, qv)
}
```
```rust
pub fn on_bezier_surface_function_times_surface_range_with_matrices(
    f: &[Vec<Real>],
    p: usize,
    q: usize,
    pw: &[Vec<Point4D>],
    r: usize,
    s: usize,
    pmu: &[Vec<Real>],
    pmv: &[Vec<Real>],
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<Point4D>>, NurbsError> {
    // --- dimension checks (네 기존 스타일 유지) ---
    if f.len() != p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "f row count must be p+1 (got {}, expected {})",
                f.len(),
                p + 1
            ),
        });
    }
    for (i, row) in f.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "f[{}].len() must be q+1 (got {}, expected {})",
                    i,
                    row.len(),
                    q + 1
                ),
            });
        }
    }

    if pw.len() != r + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "pw row count must be r+1 (got {}, expected {})",
                pw.len(),
                r + 1
            ),
        });
    }
    for (i, row) in pw.iter().enumerate() {
        if row.len() != s + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "pw[{}].len() must be s+1 (got {}, expected {})",
                    i,
                    row.len(),
                    s + 1
                ),
            });
        }
    }

    let pu = p + r;
    let qv = q + s;

    if su > eu || eu > pu {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}",
                su, eu, pu
            ),
        });
    }
    if sv > ev || ev > qv {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}",
                sv, ev, qv
            ),
        });
    }

    if pmu.len() != pu + 1 || pmu.iter().any(|row| row.len() != p + 1) {
        return Err(NurbsError::InvalidArgument {
            msg: format!("pmu size must be (p+r+1)x(p+1) = ({})x({})", pu + 1, p + 1),
        });
    }
    if pmv.len() != qv + 1 || pmv.iter().any(|row| row.len() != q + 1) {
        return Err(NurbsError::InvalidArgument {
            msg: format!("pmv size must be (q+s+1)x(q+1) = ({})x({})", qv + 1, q + 1),
        });
    }

    // output: (p+r+1) x (q+s+1)
    let mut qw = vec![vec![Point4D::default(); qv + 1]; pu + 1];

    for i in su..=eu {
        let urow = &pmu[i];
        for j in sv..=ev {
            let vrow = &pmv[j];

            // kl..kh, ll..lh same as C
            let kl = i.saturating_sub(r);
            let kh = (std::cmp::min)(p, i);
            let ll = j.saturating_sub(s);
            let lh = (std::cmp::min)(q, j);

            let mut acc = Point4D::default();

            for k in kl..=kh {
                let uik = urow[k];
                let gu = i - k; // 0..=r
                for l in ll..=lh {
                    let vjl = vrow[l];
                    let gv = j - l; // 0..=s

                    let scale = f[k][l] * uik * vjl;

                    // acc += scale * pw[gu][gv]
                    // (geom.rs에 add_scaled가 있으면 이게 제일 깔끔)
                    acc.add_scaled(scale, &pw[gu][gv]);
                }
            }

            qw[i][j] = acc;
        }
    }

    Ok(qw)
}
```
```rust
/// pmu/pmv를 내부에서 생성해서 full range 계산하는 편의 함수
pub fn on_bezier_surface_function_times_surface(
    f: &[Vec<Real>],
    p: usize,
    q: usize,
    pw: &[Vec<Point4D>],
    r: usize,
    s: usize,
) -> Result<Vec<Vec<Point4D>>, NurbsError> {
    let pu = p + r;
    let qv = q + s;

    let pmu = on_build_product_matrix(p, r);
    let pmv = on_build_product_matrix(q, s);

    on_bezier_surface_function_times_surface_range_with_matrices(
        f, p, q, pw, r, s, &pmu, &pmv, 0, pu, 0, qv,
    )
}
```
```rust
/// Convert a power-basis surface (defined on [0,1] x [0,1]) into a Bezier surface.
/// IMPORTANT WARNING:
/// ------------------------------------------------------------
/// This function ONLY works when the power basis surface is
/// already defined on the unit domain [0,1] x [0,1].
///
/// If your polynomial surface is defined on [a,b] x [c,d] with
/// a != 0 or b != 1 (or c != 0 or d != 1), you MUST first
/// reparameterize it to [0,1] x [0,1] BEFORE calling this function.
/// ------------------------------------------------------------
///
/// INPUT:
///   bw  : (p+1) x (q+1) power basis coefficients (Point4D)
///   p,q : degrees
///   ipu : (p+1) x (p+1) inverse power→Bezier matrix (u-direction)
///   ipv : (q+1) x (q+1) inverse power→Bezier matrix (v-direction)
///
/// OUTPUT:
///   Returns (p+1) x (q+1) Bezier control points (Point4D)
///
/// Mathematically:
///   Pw = ipu * bw * ipv^T
///
/// This is the [0,1] special case of the general B_spobez routine.
pub fn on_power_surface_to_bezier_unit_domain(
    bw: &[Vec<Point4D>],
    p: usize,
    q: usize,
    ipu: &[Vec<Real>],
    ipv: &[Vec<Real>],
) -> Result<Vec<Vec<Point4D>>, String> {
    // --- dimension checks ---
    if bw.len() != p + 1 {
        return Err(format!(
            "bw row count must be p+1 (got {}, expected {})",
            bw.len(),
            p + 1
        ));
    }
    for (i, row) in bw.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(format!(
                "bw[{}].len() must be q+1 (got {}, expected {})",
                i,
                row.len(),
                q + 1
            ));
        }
    }

    if ipu.len() != p + 1 {
        return Err(format!(
            "ipu row count must be p+1 (got {}, expected {})",
            ipu.len(),
            p + 1
        ));
    }
    for (i, row) in ipu.iter().enumerate() {
        if row.len() != p + 1 {
            return Err(format!(
                "ipu[{}].len() must be p+1 (got {}, expected {})",
                i,
                row.len(),
                p + 1
            ));
        }
    }

    if ipv.len() != q + 1 {
        return Err(format!(
            "ipv row count must be q+1 (got {}, expected {})",
            ipv.len(),
            q + 1
        ));
    }
    for (j, row) in ipv.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(format!(
                "ipv[{}].len() must be q+1 (got {}, expected {})",
                j,
                row.len(),
                q + 1
            ));
        }
    }

    // ------------------------------------------------------------
    // Step 1: u-direction transform: tmp = ipu * bw
    // ------------------------------------------------------------
    let mut tmp = vec![vec![Point4D::zero(); q + 1]; p + 1];

    for i in 0..=p {
        for j in 0..=q {
            let mut acc = Point4D::zero();
            for k in 0..=p {
                let coef = ipu[i][k];
                acc += bw[k][j] * coef;
            }
            tmp[i][j] = acc;
        }
    }

    // ------------------------------------------------------------
    // Step 2: v-direction transform: Pw = tmp * ipv^T
    // ------------------------------------------------------------
    let mut pw = vec![vec![Point4D::zero(); q + 1]; p + 1];

    for i in 0..=p {
        for j in 0..=q {
            let mut acc = Point4D::zero();
            for l in 0..=q {
                let coef = ipv[j][l];
                acc += tmp[i][l] * coef;
            }
            pw[i][j] = acc;
        }
    }

    Ok(pw)
}
```
```rust
/// Cross product of two Bezier surfaces using precomputed product matrices.
///
/// P(u,v): degree (p,q), control points Pw[k][l]
/// Q(u,v): degree (r,s), control points Qw[a][b]
///
/// R(u,v) = P(u,v) × Q(u,v)  →  degree (p+r, q+s)
///
/// pmu: (p+r+1) x (p+1)  u-direction product matrix
/// pmv: (q+s+1) x (q+1)  v-direction product matrix
pub fn bezier_surface_cross_product_with_matrices(
    pw: &[Vec<Point4D>],
    p: usize,
    q: usize,
    qw: &[Vec<Point4D>],
    r: usize,
    s: usize,
    pmu: &[Vec<Real>],
    pmv: &[Vec<Real>],
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<Point4D>>, String> {
    // dimension checks
    if pw.len() != p + 1 {
        return Err(format!(
            "Pw row count must be p+1 (got {}, expected {})",
            pw.len(),
            p + 1
        ));
    }
    for (i, row) in pw.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(format!(
                "Pw[{}].len() must be q+1 (got {}, expected {})",
                i,
                row.len(),
                q + 1
            ));
        }
    }

    if qw.len() != r + 1 {
        return Err(format!(
            "Qw row count must be r+1 (got {}, expected {})",
            qw.len(),
            r + 1
        ));
    }
    for (i, row) in qw.iter().enumerate() {
        if row.len() != s + 1 {
            return Err(format!(
                "Qw[{}].len() must be s+1 (got {}, expected {})",
                i,
                row.len(),
                s + 1
            ));
        }
    }

    let pu = p + r;
    let qv = q + s;

    // product matrix checks
    if pmu.len() != pu + 1 {
        return Err(format!(
            "pmu row count must be p+r+1 (got {}, expected {})",
            pmu.len(),
            pu + 1
        ));
    }
    for (i, row) in pmu.iter().enumerate() {
        if row.len() != p + 1 {
            return Err(format!(
                "pmu[{}].len() must be p+1 (got {}, expected {})",
                i,
                row.len(),
                p + 1
            ));
        }
    }

    if pmv.len() != qv + 1 {
        return Err(format!(
            "pmv row count must be q+s+1 (got {}, expected {})",
            pmv.len(),
            qv + 1
        ));
    }
    for (j, row) in pmv.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(format!(
                "pmv[{}].len() must be q+1 (got {}, expected {})",
                j,
                row.len(),
                q + 1
            ));
        }
    }

    if su > eu || eu > pu {
        return Err(format!(
            "u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}",
            su, eu, pu
        ));
    }
    if sv > ev || ev > qv {
        return Err(format!(
            "v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}",
            sv, ev, qv
        ));
    }

    // result surface
    let mut rw = vec![vec![Point4D::zero(); qv + 1]; pu + 1];

    for i in su..=eu {
        let kl = i.saturating_sub(r);
        let kh = (std::cmp::min)(p, i);

        for j in sv..=ev {
            let ll = j.saturating_sub(s);
            let lh = (std::cmp::min)(q, j);

            let mut accum = Point4D::zero();

            for k in kl..=kh {
                let uik = pmu[i][k];
                for l in ll..=lh {
                    let vjl = pmv[j][l];

                    // Piegl 의미(너 구현): xyz 외적 + weight 처리 규칙
                    let tw = pw[k][l].cross_p4(qw[i - k][j - l]);

                    accum += tw * (uik * vjl);
                }
            }

            rw[i][j] = accum;
        }
    }

    Ok(rw)
}
```
```rust
/// Product of two bivariate Bezier functions using precomputed product matrices.
/// (U and V product matrices) are already computed and passed in.
/// f: (p+1) x (q+1)
/// g: (r+1) x (s+1)
/// pmu: (p+r+1) x (p+1)
/// pmv: (q+s+1) x (q+1)
///
/// fg: (p+r+1) x (q+s+1)
pub fn on_bezier_surface_function_product_with_matrices(
    f: &[Vec<Real>],
    p: usize,
    q: usize,
    g: &[Vec<Real>],
    r: usize,
    s: usize,
    pmu: &[Vec<Real>],
    pmv: &[Vec<Real>],
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<Vec<Vec<Real>>, NurbsError> {
    // dimension checks
    if pmu.len() != p + r + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "pmu row count must be p+r+1 (got {}, expected {})",
                pmu.len(),
                p + r + 1
            ),
        });
    }
    for (i, row) in pmu.iter().enumerate() {
        if row.len() != p + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "pmu[{}].len() must be p+1 (got {}, expected {})",
                    i,
                    row.len(),
                    p + 1
                ),
            });
        }
    }

    if pmv.len() != q + s + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "pmv row count must be q+s+1 (got {}, expected {})",
                pmv.len(),
                q + s + 1
            ),
        });
    }
    for (j, row) in pmv.iter().enumerate() {
        if row.len() != q + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "pmv[{}].len() must be q+1 (got {}, expected {})",
                    j,
                    row.len(),
                    q + 1
                ),
            });
        }
    }

    let pu = p + r;
    let qv = q + s;

    if su > eu || eu > pu {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "u-range [su,eu]=[{},{}] must satisfy 0<=su<=eu<=p+r={}",
                su, eu, pu
            ),
        });
    }
    if sv > ev || ev > qv {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "v-range [sv,ev]=[{},{}] must satisfy 0<=sv<=ev<=q+s={}",
                sv, ev, qv
            ),
        });
    }

    // output fg
    let mut fg = vec![vec![0.0_f64; qv + 1]; pu + 1];

    // main computation
    for i in su..=eu {
        for j in sv..=ev {
            let kl = i.saturating_sub(r);
            let kh = p.min(i);
            let ll = j.saturating_sub(s);
            let lh = q.min(j);

            let mut sum = 0.0;

            for k in kl..=kh {
                let uik = pmu[i][k];
                for l in ll..=lh {
                    let vjl = pmv[j][l];
                    sum += uik * vjl * f[k][l] * g[i - k][j - l];
                }
            }

            fg[i][j] = sum;
        }
    }

    Ok(fg)
}
```
```rust
/// Dot product of two Bezier surfaces using precomputed product matrices.
///
/// P(u,v): degree (p,q)
/// Q(u,v): degree (r,s)
///
/// If surfaces are rational:
///   S1 = N1 / D1
///   S2 = N2 / D2
///   S1·S2 = (N1·N2) / (D1·D2)
///
/// num(i,j) = Bezier coefficients of numerator
/// den(i,j) = Bezier coefficients of denominator (only if rational)
///
/// pmu: (p+r+1) x (p+1)
/// pmv: (q+s+1) x (q+1)
pub fn bezier_surface_dot_product_with_matrices(
    pw: &[Vec<Point4D>], // P surface ctrl
    p: usize,
    q: usize,
    qw: &[Vec<Point4D>], // Q surface ctrl
    r: usize,
    s: usize,
    pmu: &[Vec<Real>],
    pmv: &[Vec<Real>],
    su: usize,
    eu: usize,
    sv: usize,
    ev: usize,
) -> Result<(Vec<Vec<Real>>, Option<Vec<Vec<Real>>>), String> {
    // rational 여부 판단
    let wp = pw[0][0].w;
    let wq = qw[0][0].w;
    let rat = !(wp == 1.0 && wq == 1.0);

    let pu = p + r;
    let qv = q + s;

    // 결과 num, den
    let mut num = vec![vec![0.0_f64; qv + 1]; pu + 1];
    let mut den = if rat {
        Some(vec![vec![0.0_f64; qv + 1]; pu + 1])
    } else {
        None
    };

    for i in su..=eu {
        for j in sv..=ev {
            let kl = if i > r { i - r } else { 0 };
            let kh = if i < p { i } else { p };
            let ll = if j > s { j - s } else { 0 };
            let lh = if j < q { j } else { q };

            let mut num_acc = 0.0;
            let mut den_acc = 0.0;

            for k in kl..=kh {
                let uik = pmu[i][k];
                for l in ll..=lh {
                    let vjl = pmv[j][l];

                    // dot product of control points
                    // dot = Pw·Qw (xyz dot)
                    // dow = weight product (w1 * w2)
                    let (dot, dow) = pw[k][l].dot_rational_pair(qw[i - k][j - l]);

                    num_acc += uik * vjl * dot;
                    if rat {
                        den_acc += uik * vjl * dow;
                    }
                }
            }

            num[i][j] = num_acc;
            if let Some(ref mut d) = den {
                d[i][j] = den_acc;
            }
        }
    }

    Ok((num, den))
}
```
```rust
impl fmt::Display for BezierSurface {
    /// Summary-style output similar to NurbsSurface::fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = 6;
        let max_u = 4;
        let max_v = 4;

        let p = self.u_degree;
        let q = self.v_degree;
        let (nu, nv) = (p + 1, q + 1);

        writeln!(f, "BezierSurface {{")?;
        writeln!(f, "  degree: (u={}, v={})", p, q)?;
        writeln!(f, "  size  : (nu={}, nv={})", nu, nv)?;

        // Control net preview
        writeln!(
            f,
            "  ctrl  : [u][v], showing up to {}x{} entries",
            max_u, max_v
        )?;

        let mu = nu.min(max_u);
        let mv = nv.min(max_v);

        for j in 0..mv {
            write!(f, "    v[{j}] ")?;
            for i in 0..mu {
                let c = self.ctrl[i][j];
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
---

