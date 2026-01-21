# Stream Line

## 1️⃣ Streamline의 기본 원리
- Streamline은 **속도장의 방향을 따라가는 곡선**
- 수식으로는:
```math
\frac{d\mathbf{x}}{ds}=\mathbf{v}(\mathbf{x})
```
- 여기서:
    - $\mathbf{x}(s)$ = streamline의 위치
    - $\mathbf{v}(\mathbf{x})$ = 속도장
    - $s$ = streamline을 따라가는 매개변수 (시간 아님)
- 즉, 현재 위치에서 속도장을 읽고 → 그 방향으로 조금 이동 → 반복
- 이게 streamline 생성의 전부.

## 2️⃣ 속도장을 만드느데 필요한 데이터
- 필요한 데이터:
    - 점들이 흩어져 있고
    - 각 점에 속도 벡터가 있고
    - 이걸 interpolation해서
    - 연속적인 velocity field $\mathbf{v}(x)$ 를 만들었음
- 그럼 이제 해야 할 건:
    - ODE 적분으로 streamline을 따라가면 된다.


## 3️⃣ 특정 속도로 따라가고 싶다면?
- 속도장의 크기와 상관없이 **내가 원하는 속도** 로 따라가고 싶다면  
    속도장을 정규화(normalize) 하면 됨.
```math
\hat {\mathbf{v}}(\mathbf{x})=\frac{\mathbf{v}(\mathbf{x})}{\| \mathbf{v}(\mathbf{x})\| }
```
- 그리고 원하는 속도 c 로 이동하려면:
```math
\frac{d\mathbf{x}}{ds}=c\cdot \hat {\mathbf{v}}(\mathbf{x})
```
- 즉:
    - 속도장의 방향은 그대로
    - 크기는 내가 원하는 속도로 맞춤
- 이렇게 하면 균일한 속도로 streamline을 생성할 수 있음.

## 4️⃣ 실제 추적 알고리즘 (RK4 예시)
- 가장 많이 쓰는 방법은 Runge–Kutta 4차 (RK4).
- 현재 위치를 $x_n$ 이라고 하면:
    - $k_1=v(x_n)$
    - $k_2=v(x_n+\frac{h}{2}k_1)$
    - $k_3=v(x_n+\frac{h}{2}k_2)$
    - $k_4=v(x_n+hk_3)$
- 그리고 다음 위치:
```math
x_{n+1}=x_n+\frac{h}{6}(k_1+2k_2+2k_3+k_4)
```
- 여기서:
    - h = step size (너가 원하는 이동 거리)
    - v(x) = 속도장 (정규화했으면 일정 속도)
- 이걸 반복하면 streamline이 생성됨.

## 5️⃣ 전체 알고리즘 요약
- (1). 속도장 v(x)를 interpolation으로 만든다
- (2). 시작점 seed point x0를 정한다
- (3). v(x)를 normalize해서 방향만 남긴다
- (4). 원하는 속도 c를 곱한다
- (5). dx/ds = c * v(x)를 RK4로 적분한다
- (6). x1, x2, x3… 반복해서 streamline을 만든다



## 6️⃣ 왜 이 방식이 CAE에서 표준인가
- 속도장이 meshfree든 mesh-based든 상관 없음
- 점들이 순서 없이 들어와도 interpolation만 되면 OK
- streamline은 시간 개념이 필요 없음
- 원하는 속도로 **균일하게** 따라갈 수 있음
- 수렴 문제 없음 (단순 ODE 적분)
- 그래서 CAE post-processing에서  
    streamline, pathline, streakline 모두 이 방식으로 만든다.

## 🌟 한 문장으로 정리
- 속도장을 만들었으면, streamline은 단순히 그 속도장을 따라 ODE를 적분하는 과정이고,  
    특정 속도로 가고 싶으면 속도장을 정규화한 뒤 원하는 속도를 곱해서 적분하면 된다.

## 소스 코드
```rust
use std::collections::HashMap;
use crate::core::types::Real;
use crate::core::geom::{Point3D, Vector3D};
use crate::core::ode_solver::OdeSolver;
use crate::core::fem_model::{FemModel, ElementType, NodeId};
use crate::core::point_ops::PointOps;
```
```rust
pub trait VelocityField {
    fn velocity(&self, x: Point3D, t: Real) -> Option<Vector3D>;
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct ConstantField {
    pub v: Vector3D,
}
```
```rust
impl VelocityField for ConstantField {
    fn velocity(&self, _x: Point3D, _t: Real) -> Option<Vector3D> { Some(self.v) }
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct RotationField; // v = (-y, x, 0)
impl VelocityField for RotationField {
    fn velocity(&self, x: Point3D, _t: Real) -> Option<Vector3D> {
        Some(Vector3D::new(-x.y, x.x, 0.0))
    }
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub struct PathlineResult {
    pub t1: Real,
    pub x1: Point3D,
    pub steps: usize,
    pub exited_domain: bool,
}
```
```rust
/// dx/dt = v(x,t)를 OdeSolver(internal f) + integrate_rk4 로 적분
pub fn on_integrate_pathline_rk4<'a>(
    solver: &mut OdeSolver<'a>,
    field: &'a dyn VelocityField,
    x0: Point3D,
    t0: Real,
    t1: Real,
    dt: Real,
    samples_out: Option<&mut Vec<PathlineResult>>,
) -> Point3D {
    solver.set_dimension(3);

    // ODE RHS 설정: y = [x,y,z]
    solver.set_function(move |t, y, dy| {
        let p = Point3D::new(y[0], y[1], y[2]);
        let v = field.velocity(p, t).expect("Invalid velocity");
        dy[0] = v.x;
        dy[1] = v.y;
        dy[2] = v.z;
    });

    let y0 = vec![x0.x, x0.y, x0.z];
    let mut y1 = vec![0.0; 3];

    // 선택적으로 궤적 저장
    let mut ts = Vec::<Real>::new();
    let mut ys = Vec::<Vec<Real>>::new();

    let ok = solver.integrate_rk4(t0, &y0, t1, dt, &mut y1, Some(&mut ts), Some(&mut ys));
    debug_assert!(ok);

    if let Some(out) = samples_out {
        out.clear();
        out.reserve(ts.len());
        for (i, &t) in ts.iter().enumerate() {
            let yy = &ys[i];
            out.push(PathlineResult {
                t1: t,
                x1: Point3D::new(yy[0], yy[1], yy[2]),
                steps: 0,
                exited_domain: false,
            });
        }
    }
    let out = Point3D::new(y1[0], y1[1], y1[2]);
    solver.clear_function();
    out
}
```
```rust
/// Integrate dx/dt = v(x,t) from t0 to t1 with fixed dt (simple stepping).
/// - 이 버전은 "가장 단순한 레퍼런스"라서 디버깅/검증용.
/// - 네 프로젝트의 RK45/적응 dt 솔버가 준비되면, 이걸 대체하거나 내부만 교체하면 됨.
pub fn on_integrate_pathline_euler(
    field: &dyn VelocityField,
    x0: Point3D,
    t0: Real,
    t1: Real,
    dt: Real,
) -> Vec<PathlineResult> {
    let mut out = Vec::new();

    if !(dt > 0.0) || !(t1 >= t0) {
        return out;
    }

    let mut t = t0;
    let mut x = x0;
    out.push(PathlineResult { t1: t, x1: x, steps: 0, exited_domain: false });

    // step count guard
    let max_steps = ((t1 - t0) / dt).ceil() as usize + 2;
    for i in 0..max_steps {
        if t >= t1 {
            break;
        }
        let h = (t1 - t).min(dt);
        let v = field.velocity(x, t).expect("Invalid velocity");
        x = Point3D::new(x.x + h * v.x, x.y + h * v.y, x.z + h * v.z);
        t += h;
        out.push(PathlineResult { t1: t, x1: x, steps: i, exited_domain: false });
    }

    out
}
```
```rust

#[derive(Debug)]
pub struct FemVelocityField<'a> {
    pub model: &'a FemModel,
    pub nodal_vel: &'a HashMap<NodeId, Vector3D>,
    pub inside_tol: Real,
}
```
```rust
impl<'a> FemVelocityField<'a> {
    pub fn new(model: &'a FemModel, nodal_vel: &'a HashMap<NodeId, Vector3D>) -> Self {
        Self {
            model,
            nodal_vel,
            inside_tol: 1e-12,
        }
    }

    fn vel_at_node(&self, nid: usize) -> Vector3D {
        self.nodal_vel.get(&(nid as NodeId)).unwrap().clone()
    }

    /// 삼각형 barycentric (3D에서도 동작: 삼각형 평면 위/근처라고 가정)
    fn tri_barycentric(p: Point3D, a: Point3D, b: Point3D, c: Point3D) -> Option<(Real, Real, Real)> {
        // 안정적으로 하려고, (b-a, c-a) 기저에서 p-a를 최소제곱으로 푼다.
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);

        let denom = d00 * d11 - d01 * d01;
        if denom.abs() < 1e-30 {
            return None; // degenerate tri
        }
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        Some((u, v, w))
    }

    fn tri_contains_bary(&self, bary: (Real, Real, Real)) -> bool {
        let (u, v, w) = bary;
        let t = self.inside_tol;
        u >= -t && v >= -t && w >= -t
    }

    fn tri_interp_vel(&self, n0: NodeId, n1: NodeId, n2: NodeId, bary: (Real, Real, Real)) -> Vector3D {
        let (w0, w1, w2) = bary;
        let v0 = self.vel_at_node(n0 as usize);
        let v1 = self.vel_at_node(n1 as usize);
        let v2 = self.vel_at_node(n2 as usize);
        v0 * w0 + v1 * w1 + v2 * w2
    }

    fn quad_interp_vel_split(&self, n0: NodeId, n1: NodeId, n2: NodeId, n3: NodeId, p: Point3D) -> Option<Vector3D> {
        let a = self.model.nodes.get(&n0);
        let b = self.model.nodes.get(&n1);
        let c = self.model.nodes.get(&n2);
        let d = self.model.nodes.get(&n3);

        let pa = Point3D::from_array(&a?.xyz);
        let pb = Point3D::from_array(&b?.xyz);
        let pc = Point3D::from_array(&c?.xyz);
        let pd = Point3D::from_array(&d?.xyz);

        // (a,b,c) 먼저
        if let Some(bary) = Self::tri_barycentric(p, pa, pb, pc) {
            if self.tri_contains_bary(bary) {
                return Some(self.tri_interp_vel(n0, n1, n2, bary));
            }
        }
        // (a,c,d)
        if let Some(bary) = Self::tri_barycentric(p, pa, pc, pd) {
            if self.tri_contains_bary(bary) {
                return Some(self.tri_interp_vel(n0, n2, n3, bary));
            }
        }
        None
    }

    /// 점 p에서 FEM 보간 속도 구하기 (못 찾으면 None)
    pub fn velocity_at(&self, p: Point3D) -> Option<Vector3D> {
        if self.nodal_vel.len() != self.model.nodes.len() {
            return None;
        }

        for e in &self.model.elems {
            match e.1.e_type {
                ElementType::Tri3 => {
                    let n0 = e.1.node_ids[0];
                    let n1 = e.1.node_ids[1];
                    let n2 = e.1.node_ids[2];
                    let a = self.model.nodes.get(&n0);
                    let b = self.model.nodes.get(&n1);
                    let c = self.model.nodes.get(&n2);
                    if !a.is_none() && !b.is_none()  && !c.is_none() {
                        let pa = Point3D::from_array(&a?.xyz);
                        let pb = Point3D::from_array(&b?.xyz);
                        let pc = Point3D::from_array(&c?.xyz);
                        if let Some(bary) = Self::tri_barycentric(p, pa, pb, pc) {
                            if self.tri_contains_bary(bary) {

                                return Some(self.tri_interp_vel(n0, n1, n2, bary));
                            }
                        }
                    }
                }
                ElementType::Quad4 => {
                    let n0 = e.1.node_ids[0];
                    let n1 = e.1.node_ids[1];
                    let n2 = e.1.node_ids[2];
                    let n3 = e.1.node_ids[3];

                    if let Some(v) = self.quad_interp_vel_split(n0, n1, n2, n3, p) {
                        return Some(v);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
```
```rust
/// Robust-ish barycentric in 3D using dot-products (assumes non-degenerate triangle).
fn on_barycentric_tri3_with_tol(a: Point3D, b: Point3D, c: Point3D, p: Point3D, tol: Real)
  -> Option<(Real, Real, Real)> {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let den = d00 * d11 - d01 * d01;
    if den.abs() < 1e-30 {
        return None;
    }

    let v = (d11 * d20 - d01 * d21) / den;
    let w = (d00 * d21 - d01 * d20) / den;
    let u = 1.0 - v - w;

    // inside test with tolerance
    if u >= -tol && v >= -tol && w >= -tol {
        Some((u, v, w))
    } else {
        None
    }
}
```
```rust
impl VelocityField for FemVelocityField<'_> {
    fn velocity(&self, x: Point3D, _t: Real) -> Option<Vector3D> {
        // 1) 아주 단순하게: 모든 elem을 검사 (나중에 bbox/rtree로 가속)
        for e in self.model.elems.values() {
            match e.e_type {
                ElementType::Tri3 => {
                    if e.node_ids.len() < 3 { continue; }
                    let n0 = self.model.nodes.get(&e.node_ids[0])?;
                    let n1 = self.model.nodes.get(&e.node_ids[1])?;
                    let n2 = self.model.nodes.get(&e.node_ids[2])?;

                    let p0 = Point3D::new(n0.xyz[0], n0.xyz[1], n0.xyz[2]);
                    let p1 = Point3D::new(n1.xyz[0], n1.xyz[1], n1.xyz[2]);
                    let p2 = Point3D::new(n2.xyz[0], n2.xyz[1], n2.xyz[2]);

                    if let Some((w0, w1, w2)) = on_barycentric_tri3_with_tol(p0, p1, p2, x,
                      self.inside_tol) {
                        let v0 = *self.nodal_vel.get(&e.node_ids[0])?;
                        let v1 = *self.nodal_vel.get(&e.node_ids[1])?;
                        let v2 = *self.nodal_vel.get(&e.node_ids[2])?;

                        return Some(Vector3D::new(
                            w0 * v0.x + w1 * v1.x + w2 * v2.x,
                            w0 * v0.y + w1 * v1.y + w2 * v2.y,
                            w0 * v0.z + w1 * v1.z + w2 * v2.z,
                        ));
                    }
                }
                ElementType::Quad4 => {
                    if e.node_ids.len() < 4 { continue; }

                    let ids = [e.node_ids[0], e.node_ids[1], e.node_ids[2], e.node_ids[3]];

                    let n0 = self.model.nodes.get(&ids[0])?;
                    let n1 = self.model.nodes.get(&ids[1])?;
                    let n2 = self.model.nodes.get(&ids[2])?;
                    let n3 = self.model.nodes.get(&ids[3])?;

                    let p0 = Point3D::new(n0.xyz[0], n0.xyz[1], n0.xyz[2]);
                    let p1 = Point3D::new(n1.xyz[0], n1.xyz[1], n1.xyz[2]);
                    let p2 = Point3D::new(n2.xyz[0], n2.xyz[1], n2.xyz[2]);
                    let p3 = Point3D::new(n3.xyz[0], n3.xyz[1], n3.xyz[2]);

                    // quad -> tri split: (0,1,2) and (0,2,3)
                    if let Some((w0, w1, w2)) = on_barycentric_tri3_with_tol(p0, p1, p2, x,
                      self.inside_tol) {
                        let v0 = *self.nodal_vel.get(&ids[0])?;
                        let v1 = *self.nodal_vel.get(&ids[1])?;
                        let v2 = *self.nodal_vel.get(&ids[2])?;
                        return Some(Vector3D::new(
                            w0 * v0.x + w1 * v1.x + w2 * v2.x,
                            w0 * v0.y + w1 * v1.y + w2 * v2.y,
                            w0 * v0.z + w1 * v1.z + w2 * v2.z,
                        ));
                    }
                    if let Some((w0, w2, w3)) = on_barycentric_tri3_with_tol(p0, p2, p3, x,
                      self.inside_tol) {
                        let v0 = *self.nodal_vel.get(&ids[0])?;
                        let v2 = *self.nodal_vel.get(&ids[2])?;
                        let v3 = *self.nodal_vel.get(&ids[3])?;
                        return Some(Vector3D::new(
                            w0 * v0.x + w2 * v2.x + w3 * v3.x,
                            w0 * v0.y + w2 * v2.y + w3 * v3.y,
                            w0 * v0.z + w2 * v2.z + w3 * v3.z,
                        ));
                    }
                }

                _ => {
                    // 다음 단계에서 Quad4/Bar2/Tri6 등 확장
                }
            }
        }
        None

    }
}
```
```rust
pub struct FemVelocityFieldFn<'a, F>
where
    F: Fn(u32, Real) -> Option<Vector3D>,
{
    pub model: &'a FemModel,
    pub vel_fn: F,
    pub inside_tol: Real,
}
```
```rust
impl<'a, F> FemVelocityFieldFn<'a, F>
where
    F: Fn(u32, Real) -> Option<Vector3D>,
{
    pub fn new(model: &'a FemModel, vel_fn: F) -> Self {
        Self { model, vel_fn, inside_tol: 1e-12 }
    }
}
```
```rust
impl<'a, F> VelocityField for FemVelocityFieldFn<'a, F>
where
    F: Fn(u32, Real) -> Option<Vector3D>,
{
    fn velocity(&self, x: Point3D, t: Real) -> Option<Vector3D> {
        for e in self.model.elems.values() {
            match e.e_type {
                ElementType::Tri3 => {
                    if e.node_ids.len() < 3 { continue; }
                    let ids = [e.node_ids[0], e.node_ids[1], e.node_ids[2]];

                    let n0 = self.model.nodes.get(&ids[0])?;
                    let n1 = self.model.nodes.get(&ids[1])?;
                    let n2 = self.model.nodes.get(&ids[2])?;

                    let p0 = Point3D::new(n0.xyz[0], n0.xyz[1], n0.xyz[2]);
                    let p1 = Point3D::new(n1.xyz[0], n1.xyz[1], n1.xyz[2]);
                    let p2 = Point3D::new(n2.xyz[0], n2.xyz[1], n2.xyz[2]);

                    if let Some((w0, w1, w2)) = on_barycentric_tri3_with_tol(p0, p1, p2, x,
                      self.inside_tol) {
                        let v0 = (self.vel_fn)(ids[0], t)?;
                        let v1 = (self.vel_fn)(ids[1], t)?;
                        let v2 = (self.vel_fn)(ids[2], t)?;
                        return Some(Vector3D::new(
                            w0*v0.x + w1*v1.x + w2*v2.x,
                            w0*v0.y + w1*v1.y + w2*v2.y,
                            w0*v0.z + w1*v1.z + w2*v2.z,
                        ));
                    }
                }
                _ => {}
            }
        }
        None
    }
}
```
```rust
pub fn on_integrate_pathline_rk4_mesh<F: VelocityField>(
    field: &F,
    x0: Point3D,
    t0: Real,
    t1: Real,
    dt: Real,
    mut out_ts: Option<&mut Vec<Real>>,
    mut out_xs: Option<&mut Vec<Point3D>>,
) -> Point3D {
    let mut solver = OdeSolver::new(3);

    solver.set_function(|t, y, dy| {
        let x = Point3D::new(y[0], y[1], y[2]);
        let v = field.velocity(x, t).expect("Invalid velocity");
        dy[0] = v.x;
        dy[1] = v.y;
        dy[2] = v.z;
    });

    let y0 = vec![x0.x, x0.y, x0.z];
    let mut y1 = vec![0.0; 3];

    // trajectory 저장(원하면)
    let mut ts_local: Vec<Real> = Vec::new();
    let mut ys_local: Vec<Vec<Real>> = Vec::new();

    let ok = solver.integrate_rk4(
        t0,
        &y0,
        t1,
        dt,
        &mut y1,
        Some(&mut ts_local),
        Some(&mut ys_local),
    );

    if ok {
        if let Some(ts) = out_ts.as_mut() {
            ts.clear();
            ts.extend_from_slice(&ts_local);
        }
        if let Some(xs) = out_xs.as_mut() {
            xs.clear();
            for y in &ys_local {
                xs.push(Point3D::new(y[0], y[1], y[2]));
            }
        }
    }

    Point3D::new(y1[0], y1[1], y1[2])
}
```
```rust
/// Fixed-step RK4 pathline integrator.
/// - stops early if field returns None (outside domain)
pub fn on_integrate_pathline_rk4_mesh_with<F: VelocityField>(
    solver: &OdeSolver,
    field: &F,
    x0: Point3D,
    t0: Real,
    t1: Real,
    dt: Real,
    mut out: Option<&mut Vec<Point3D>>,
) -> PathlineResult {
    debug_assert!(dt > 0.0);

    let mut t = t0;
    let mut x = x0;
    let mut steps = 0usize;

    if let Some(p) = out.as_mut() {
        p.clear();
        p.push(x0);
    }

    let mut y = vec![x.x, x.y, x.z];
    let mut y_next = vec![0.0; 3];

    while t < t1 - 1e-15 {
        let h = dt.min(t1 - t);

        // dy/dt = v(x,t)
        let mut deriv = |tt: Real, yy: &[Real], dydt: &mut [Real]| {
            let px = Point3D::new(yy[0], yy[1], yy[2]);
            if let Some(v) = field.velocity(px, tt) {
                dydt[0] = v.x;
                dydt[1] = v.y;
                dydt[2] = v.z;
            } else {
                // outside → mark by NaN (we’ll detect after step)
                dydt[0] = Real::NAN;
                dydt[1] = Real::NAN;
                dydt[2] = Real::NAN;
            }
        };

        solver.step_rk4_with(&mut deriv, t, &y, h, &mut y_next);

        // if outside happened, NaN will propagate
        if y_next.iter().any(|v| v.is_nan()) {
            return PathlineResult {
                x1: Point3D::new(y[0], y[1], y[2]),
                t1: t,
                steps,
                exited_domain: true,
            };
        }

        y.copy_from_slice(&y_next);
        t += h;
        steps += 1;

        x = Point3D::new(y[0], y[1], y[2]);
        if let Some(p) = out.as_mut() {
            p.push(x);
        }
    }

    PathlineResult {
        x1: x,
        t1: t,
        steps,
        exited_domain: false,
    }
}
```
```rust
pub fn on_integrate_pathline_rk4_model_interp_with_tol<F: VelocityField>(
    solver: &OdeSolver,
    field: &F,
    x0: Point3D,
    t0: Real,
    t1: Real,
    dt: Real,
    mut out: Option<&mut Vec<Point3D>>,
) -> PathlineResult {
    let mut t = t0;
    let mut y = vec![x0.x, x0.y, x0.z];
    let mut y_next = vec![0.0; 3];
    let mut steps = 0usize;

    if let Some(p) = out.as_mut() {
        p.clear();
        p.push(x0);
    }

    let mut step_once = |tt: Real, yy: &[Real], h: Real, yout: &mut [Real]| -> bool {
        let mut deriv = |tq: Real, yq: &[Real], dydt: &mut [Real]| {
            let px = Point3D::new(yq[0], yq[1], yq[2]);
            if let Some(v) = field.velocity(px, tq) {
                dydt[0] = v.x; dydt[1] = v.y; dydt[2] = v.z;
            } else {
                dydt[0] = Real::NAN; dydt[1] = Real::NAN; dydt[2] = Real::NAN;
            }
        };
        solver.step_rk4_with(&mut deriv, tt, yy, h, yout);
        !yout.iter().any(|v| v.is_nan())
    };

    while t < t1 - 1e-15 {
        let h_full = (t1 - t).min(dt);

        // 먼저 한 번 시도
        if step_once(t, &y, h_full, &mut y_next) {
            y.copy_from_slice(&y_next);
            t += h_full;
            steps += 1;

            if let Some(p) = out.as_mut() {
                p.push(Point3D::new(y[0], y[1], y[2]));
            }
            continue;
        }

        // 밖으로 나감: [0, h_full]에서 마지막 inside를 이분법으로 찾고 종료
        let mut lo = 0.0;
        let mut hi = h_full;

        // lo는 inside(=0), hi는 outside(=h_full) 상태라고 가정
        let mut y_lo = y.clone();
        let mut y_mid = vec![0.0; 3];

        // 이분법 횟수 (정밀도)
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if mid <= 0.0 { break; }

            if step_once(t, &y, mid, &mut y_mid) {
                lo = mid;
                y_lo.copy_from_slice(&y_mid);
            } else {
                hi = mid;
            }

            if (hi - lo) < 1e-12 * (1.0 + h_full.abs()) {
                break;
            }
        }

        // 마지막 inside에서 종료
        y.copy_from_slice(&y_lo);
        t += lo;

        if let Some(p) = out.as_mut() {
            p.push(Point3D::new(y[0], y[1], y[2]));
        }

        return PathlineResult {
            x1: Point3D::new(y[0], y[1], y[2]),
            t1: t,
            steps,
            exited_domain: true,
        };
    }

    PathlineResult { x1: Point3D::new(y[0], y[1], y[2]), t1: t, steps, exited_domain: false }
}
```
```rust
pub fn on_integrate_pathline_rk45<F: VelocityField>(
    solver: &OdeSolver,
    field: &F,
    x0: Point3D,
    t0: Real,
    t1: Real,
    mut h: Real,
) -> PathlineResult {
    let mut t = t0;
    let mut y = vec![x0.x, x0.y, x0.z];
    let mut y_next = vec![0.0; 3];
    let mut steps = 0usize;

    if !(h > 0.0) { h = (t1 - t0).abs() / 50.0; }
    h = h.max(solver.h_min).min(solver.h_max);

    let mut step_once = |tt: Real, yy: &[Real], hh: Real, yout: &mut [Real], err: &mut Real| -> bool {
        let mut deriv = |tq: Real, yq: &[Real], dydt: &mut [Real]| {
            let px = Point3D::new(yq[0], yq[1], yq[2]);
            if let Some(v) = field.velocity(px, tq) {
                dydt[0]=v.x; dydt[1]=v.y; dydt[2]=v.z;
            } else {
                dydt[0]=Real::NAN; dydt[1]=Real::NAN; dydt[2]=Real::NAN;
            }
        };
        solver.step_rk45_with(&mut deriv, tt, yy, hh, yout, err);
        !yout.iter().any(|v| v.is_nan())
    };

    for _ in 0..1_000_000 {
        if t >= t1 - 1e-15 { break; }
        if t + h > t1 { h = t1 - t; }
        h = h.max(solver.h_min).min(solver.h_max);

        let mut err = 0.0;
        if !step_once(t, &y, h, &mut y_next, &mut err) {
            // 밖으로 나감: 이분법으로 경계 찾고 종료 (오차 기준 무시, inside/outside만 본다)
            let mut lo = 0.0;
            let mut hi = h;
            let mut y_lo = y.clone();
            let mut y_mid = vec![0.0; 3];
            let mut dummy = 0.0;

            for _ in 0..60 {
                let mid = 0.5*(lo+hi);
                if mid <= 0.0 { break; }
                if step_once(t, &y, mid, &mut y_mid, &mut dummy) {
                    lo = mid;
                    y_lo.copy_from_slice(&y_mid);
                } else {
                    hi = mid;
                }
                if (hi-lo) < 1e-12*(1.0 + h.abs()) { break; }
            }

            y.copy_from_slice(&y_lo);
            t += lo;

            return PathlineResult {
                x1: Point3D::new(y[0], y[1], y[2]),
                t1: t,
                steps,
                exited_domain: true,
            };
        }

        // accept/reject
        if err <= 1.0 {
            t += h;
            y.copy_from_slice(&y_next);
            steps += 1;

            // next h
            let mut fac = solver.safety * (1.0f64.max(1.0/err)).powf(1.0/5.0);
            fac = fac.max(solver.fac_min).min(solver.fac_max);
            h = (h*fac).max(solver.h_min).min(solver.h_max);
        } else {
            // reduce h
            let mut fac = solver.safety * (1.0f64.max(1.0/err)).powf(1.0/5.0);
            fac = fac.max(0.1).min(0.5);
            h = (h*fac).max(solver.h_min).min(solver.h_max);
        }
    }

    PathlineResult { x1: Point3D::new(y[0], y[1], y[2]), t1: t, steps, exited_domain: false }
}
```

## 1️⃣ 전체 구조 요약 — 지금 만든 건 “범용 Streamline 엔진”
- 코드는 크게 4개의 구성요소로 나뉜다.
- ① VelocityField (속도장 인터페이스)
```rust
pub trait VelocityField {
    fn velocity(&self, x: Point3D, t: Real) -> Vector3D;
}
```

    - 어떤 속도장이든 이 trait만 구현하면 됨
    - CFD 데이터, RBF 보간된 field, analytic field 모두 가능
    - Streamline 엔진은 field 내부가 어떻게 생겼는지 신경 안 씀
- 즉, 속도장 모듈을 완전히 분리한 설계라서 확장성이 매우 좋다.

- ② ConstantField / RotationField (테스트용 속도장)
    - ConstantField → 직선 Streamline
    - RotationField → 원형 Streamline
- 테스트용으로 아주 적절하고, 나중에 실제 CFD velocity field로 교체하면 그대로 동작한다.

- ③ integrate_pathline_rk4 (Runge–Kutta 4차 적분)
```math
dx/dt = v(x,t)
```
- 을 RK4로 적분하는 함수.
    - 정확도 높음
    - dt 고정
    - samples_out 옵션으로 Streamline 전체를 저장할 수도 있음
    - 마지막 위치만 반환할 수도 있음
- CAE에서 가장 많이 쓰는 방식 그대로다.

- ④ on_integrate_pathline_euler (단순 Euler 적분)
    - 디버깅용
    - RK4와 비교해서 검증하는 용도
- 테스트에서 ConstantField로 정확도 비교하는 구조 포함

## 2️⃣ Streamline을 실제로 어떻게 구하나?
- Streamline은 속도장을 따라가는 곡선이므로 함수는 이렇게 사용하면 된다.

- ✔ Step 1: 속도장 준비
    - 예: ConstantField

```rust
let field = ConstantField { v: Vector3D::new(1.0, 0.5, 0.0) };
```

- 예: RotationField
```rust
let field = RotationField;
```

- 나중에는 CFD 데이터로 VelocityField를 구현하면 된다.

- ✔ Step 2: 시작점(seed point) 선택
- Streamline은 seed point에서 시작한다.
```rust
let x0 = Point3D::new(1.0, 0.0, 0.0);
```

- ✔ Step 3: ODE 적분으로 Streamline 생성
- RK4 버전 (정확한 Streamline)
```rust
let mut solver = OdeSolver::new(3);

let mut samples = Vec::new();
let x_end = integrate_pathline_rk4(
    &mut solver,
    &field,
    x0,
    0.0,     // t0
    5.0,     // t1
    0.01,    // dt
    Some(&mut samples),
);
```    

- samples 안에 Streamline의 모든 점이 들어 있음
- x_end는 마지막 위치
- dt는 Streamline의 해상도 조절

- Euler 버전 (빠르지만 정확도 낮음)
```rust
let samples = on_integrate_pathline_euler(&field, x0, 0.0, 5.0, 0.01);
```

- ✔ Step 4: PathlineResult를 polyline으로 사용

- Result는 이렇게 생김:
```rust
pub struct PathlineResult {
    pub t1: Real,
    pub x1: Point3D,
    pub steps: usize,
    pub exited_domain: bool,
}
```

- 즉:
    - PathlineResult[i].x1 → Streamline의 i번째 점
    - 이걸 polyline으로 그리면 Streamline이 된다
    - CAE post-processing에서는 이걸 그대로 시각화

## 3️⃣ Streamline을 CFD 데이터에 적용하는 방법
- VelocityField를 CFD 데이터로 구현하면 끝
- 예를 들어 RBF 보간된 속도장:
```rust
pub struct RbfVelocityField {
    rbf: RbfInterpolator,
}

impl VelocityField for RbfVelocityField {
    fn velocity(&self, x: Point3D, _t: Real) -> Vector3D {
        self.rbf.eval(x)
    }
}
```

- 이렇게만 하면:
    - CFD 점 데이터 → RBF로 보간
    - VelocityField로 감싸기
    - integrate_pathline_rk4로 Streamline 생성
- 완벽하게 동작한다.

---
# rk4 interpolation 이해 하기

## 1. 먼저, Streamline을 수식으로 다시 보자
- 네가 만들고 싶은 건 아래와 같이 가정:
    - 공간에서 속도장의 방향을 따라가는 곡선
    - 즉, 지금 이 위치에서의 **속도 방향** 으로 조금 가고,
    - 거기서 또 그 위치의 **속도 방향으로 조금 가고…** 를 반복한 궤적
- 수식으로 쓰면:
```math
\frac{d\mathbf{x}}{dt}=\mathbf{v}(\mathbf{x},t)
```
- 여기서:
    - $\mathbf{x}(t)$: 시간 t에서의 위치 (3D 점)
    - $\mathbf{v}(\mathbf{x},t)$: 그 위치에서의 속도 벡터
    - 이 미분방정식을 풀면 → $\mathbf{x}(t)$ 라는 곡선이 나옴
- 이게 바로 pathline/streamline
- 즉, Streamline = 이 미분방정식의 해 곡선.

## 2. 함수가 하는 일: “이 미분방정식을 수치적으로 푸는 것”
- integrate_pathline_rk4를 보자.
```rust
solver.set_function(move |t, y, dy| {
    let p = Point3D::new(y[0], y[1], y[2]);
    let v = field.velocity(p, t);
    dy[0] = v.x;
    dy[1] = v.y;
    dy[2] = v.z;
});
```

- 여기서 의미는 딱 하나야:
    - y = [x, y, z] → 현재 위치
    - dy = [dx/dt, dy/dt, dz/dt]
    - 그런데 dx/dt = v.x, dy/dt = v.y, dz/dt = v.z
- 즉,
```math
\frac{d\mathbf{x}}{dt}=\mathbf{v}(\mathbf{x},t)
```
- 이걸 그대로 코드로 적은 것
- 이걸 OdeSolver에게 넘겨서  
    **이 미분방정식을 t0에서 t1까지 풀게 하는 것임**.

## 3. RK4가 실제로 하는 일: 
- **조금씩 앞으로 나가며 곡선을 그린다** 아주 단순화해서 말하면, RK4는 이걸 반복하는 알고리즘:
    - 현재 위치 $x_n$ 에서 속도 $v(x_n)$ 를 읽는다
    - 그 방향으로 조금(h) 이동한다
    - 새 위치에서 다시 속도를 읽고 보정한다
    - 이걸 반복해서 $x_{n+1}$,$x_{n+2},\dots$  를 만든다
- 즉:
    - 시작점: $x(t_0)=x_0$
    - 한 스텝: $x_{n+1}=x_n+\Delta x$
    - 이걸 t0→t1까지 반복
    - 그 결과: $\{ x(t_0),x(t_1),x(t_2),\dots \}$ 
    - 이 점들을 이어놓으면 → Streamline 곡선
- 코드에서 이 부분:
```rust
let ok = solver.integrate_rk4(t0, &y0, t1, dt, &mut y1, Some(&mut ts), Some(&mut ys));
```
- t0 → 시작 시간
- t1 → 끝 시간
- dt → 한 번에 얼마나 나아갈지 (step size)
- y0 → 시작 위치 [x0, y0, z0]
- ys → 중간에 지나간 모든 위치들
- y1 → 마지막 위치
## 4. 그래서 **Streamline이 만들어진다** 는 말의 정확한 의미이 한 줄이 하는 일:
```rust
let x1 = integrate_pathline_rk4(&mut solver, &field, x0, t0, t1, dt, Some(&mut samples));
```
- 이건 수학적으로는:
- 미분방정식
```math
\frac{d\mathbf{x}}{dt}=\mathbf{v}(\mathbf{x},t),\quad \mathbf{x}(t_0)=x_0
```
- 를 t0에서 t1까지 푼다
- 그 과정에서 지나간 모든 점 $\mathbf{x}(t_i)$ 를 result에 저장한다
- 이 result를 순서대로 이어놓으면 → Streamline 곡선
- 즉,
- Streamline = **속도장을 따라 적분한 궤적** 이고,  
    `on_integrate_pathline_rk4` = 그 적분을 실제로 수행하는 함수.

## 5. 코드 기준으로 한 번 말로 시뮬레이션
- 예를 들어:
    - field = RotationField (v = (-y, x, 0))
    - x0 = (1, 0, 0)
    - t0 = 0, t1 = 2π, dt = 0.001
- 그러면:
    - t=0, x=(1,0,0) → v=(-0, 1, 0) → 위쪽으로 이동 시작
    - t가 조금씩 증가하면서
    - x가 (cos t, sin t, 0) 근처를 따라감
    - t=2π가 되면 다시 (1,0,0) 근처로 돌아옴
    - 이 중간 점들을 다 모으면 → 원형 Streamline
- 테스트 코드에서 이걸 검증:
```rust
assert!((x1.x - x0.x).abs() < 1e-3);
assert!((x1.y - x0.y).abs() < 1e-3);
```
- 즉, 함수가 실제로 **속도장을 따라가는 곡선** 을 잘 만들고 있다는 증거.

## 6. 한 문장으로 다시 정리하면
- on_integrate_pathline_rk4는
    - **dx/dt = v(x,t)** 라는 미분방정식을 수치적으로 풀어서,  
    속도장을 따라가는 궤적 x(t)를 점들의 리스트로 만들어주는 함수이고,  
    그 점들을 이어놓은 것이 바로 Streamline이다.

---
### 테스트 코드
```rust
#[cfg(test)]
mod tests {

    use nurbslib::core::geom::{Point3D, Vector3D};
    use nurbslib::core::pathline::{ConstantField, VelocityField};
```
```rust
    #[test]
    fn constant_field_velocity_is_constant() {
        let f = ConstantField { v: Vector3D::new(1.0, -2.0, 0.5) };
        let v0 = f.velocity(Point3D::new(9.0, 8.0, 7.0), 0.0).expect("Velocity failed");
        let v1 = f.velocity(Point3D::new(-1.0, 3.0, 4.0), 123.0).expect("Velocity failed");
        assert!((v0.x - 1.0).abs() < 1e-12);
        assert!((v0.y + 2.0).abs() < 1e-12);
        assert!((v0.z - 0.5).abs() < 1e-12);
        assert!((v1.x - 1.0).abs() < 1e-12);
        assert!((v1.y + 2.0).abs() < 1e-12);
        assert!((v1.z - 0.5).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_euler_constant {

    use nurbslib::core::geom::{Point3D, Vector3D};
    use nurbslib::core::pathline::{on_integrate_pathline_euler, ConstantField};

    #[test]
    fn pathline_euler_constant_field_is_straight_line() {
        let f = ConstantField { v: Vector3D::new(2.0, 0.0, 0.0) };
        let x0 = Point3D::new(1.0, 3.0, -5.0);

        let sol = on_integrate_pathline_euler(&f, x0, 0.0, 1.0, 0.01);
        let last = sol.last().unwrap();

        // x(t)=x0 + v*t
        let ex = 1.0 + 2.0 * 1.0;
        assert!((last.x1.x - ex).abs() < 5e-2); // Euler라서 오차 허용
        assert!((last.x1.y - 3.0).abs() < 1e-12);
        assert!((last.x1.z + 5.0).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_rk45_constant {
    use nurbslib::core::geom::{Point3D, Vector3D};
    use nurbslib::core::ode_solver::OdeSolver;
    use nurbslib::core::pathline::{on_integrate_pathline_rk4,
      on_integrate_pathline_euler,
      ConstantField, RotationField};
```
```rust
    #[test]
    fn pathline_euler_constant_field_is_straight_line() {
        let f = ConstantField { v: Vector3D::new(2.0, 0.0, 0.0) };
        let x0 = Point3D::new(1.0, 3.0, -5.0);

        let sol = on_integrate_pathline_euler(&f, x0, 0.0, 1.0, 0.01);
        let last = sol.last().unwrap();

        // x(t)=x0 + v*t
        let ex = 1.0 + 2.0 * 1.0;
        assert!((last.x1.x - ex).abs() < 5e-2); // Euler라서 오차 허용
        assert!((last.x1.y - 3.0).abs() < 1e-12);
        assert!((last.x1.z + 5.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn pathline_constant_field_matches_exact() {
        use nurbslib::core::geom::{Point3D, Vector3D};
        use nurbslib::core::ode_solver::OdeSolver;

        let field = ConstantField { v: Vector3D::new(2.0, -1.0, 0.5) };
        let x0 = Point3D::new(1.0, 3.0, -5.0);
        let mut solver = OdeSolver::new(3);

        let t0 = 0.0;
        let t1 = 1.0;
        let dt = 0.01;

        let x1 = on_integrate_pathline_rk4(&mut solver, &field, x0, t0, t1, dt, None);

        let ex = Point3D::new(
            x0.x + field.v.x * (t1 - t0),
            x0.y + field.v.y * (t1 - t0),
            x0.z + field.v.z * (t1 - t0),
        );

        assert!((x1.x - ex.x).abs() < 1e-10);
        assert!((x1.y - ex.y).abs() < 1e-10);
        assert!((x1.z - ex.z).abs() < 1e-10);
    }
```
```rust
    #[test]
    fn pathline_rotation_field_near_returns_after_2pi() {


        let field = RotationField;

        let x0 = Point3D::new(1.0, 0.0, 0.0);

        let mut solver = OdeSolver::new(3);

        let t0 = 0.0;
        let t1 = std::f64::consts::PI * 2.0;
        let dt = 1e-3;

        let x1 = on_integrate_pathline_rk4(&mut solver, &field, x0, t0, t1, dt, None);

        let r0 = (x0.x * x0.x + x0.y * x0.y).sqrt();
        let r1 = (x1.x * x1.x + x1.y * x1.y).sqrt();

        assert!((r1 - r0).abs() < 1e-6, "radius drift too large: r0={} r1={}", r0, r1);
        assert!((x1.x - x0.x).abs() < 1e-3, "x mismatch too large: {}", (x1.x - x0.x).abs());
        assert!((x1.y - x0.y).abs() < 1e-3, "y mismatch too large: {}", (x1.y - x0.y).abs());
        assert!(x1.z.abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_rk45_fem {
    use std::collections::HashMap;
    use nurbslib::core::fem_model::{Element, ElementType, FemModel, Node, NodeId};
    use nurbslib::core::lucmp::{on_lu_solve_with_pivot, on_m_lu_decmp_full_with_pivot};
    use nurbslib::core::types::Real;
    use nurbslib::core::geom::{Point3D, Vector3D};
    use nurbslib::core::ode_solver::OdeSolver;
    use nurbslib::core::pathline::{on_integrate_pathline_rk45, on_integrate_pathline_rk4_mesh,
      on_integrate_pathline_rk4_mesh_with, on_integrate_pathline_rk4_model_interp_with_tol,
      ConstantField, FemVelocityField, FemVelocityFieldFn, PathlineResult, VelocityField};
```
```rust
    #[test]
    fn lu_pivot_solve_matches_known_solution() {

        // A x = b, 정답 x를 정해놓고 b를 만든다.
        let a: Vec<Vec<Real>> = vec![
            vec![ 0.0, 2.0, 3.0],
            vec![ 4.0, 5.0, 6.0],
            vec![ 7.0, 8.0, 10.0],
        ];
        let x_true = vec![1.5, -2.0, 0.25];

        let mut b = vec![0.0; 3];
        for i in 0..3 {
            b[i] = a[i][0]*x_true[0] + a[i][1]*x_true[1] + a[i][2]*x_true[2];
        }

        let lu = on_m_lu_decmp_full_with_pivot(a.clone(), 1e-14).expect("lu failed");
        let mut x = b.clone();
        assert!(on_lu_solve_with_pivot(&lu, &mut x), "solve failed");

        for i in 0..3 {
            assert!((x[i] - x_true[i]).abs() < 1e-10, "i={} x={} true={}", i, x[i], x_true[i]);
        }
    }
```
```rust
    #[test]
    fn pathline_constant_field_is_straight_line() {
        let field = ConstantField { v: Vector3D::new(2.0, -1.0, 0.5) };

        let x0 = Point3D::new(1.0, 2.0, 3.0);
        let t0 = 0.0;
        let t1 = 1.25;
        let dt = 0.01;

        let x1 = on_integrate_pathline_rk4_mesh(&field, x0, t0, t1, dt, None, None);

        // 해석해: x(t)=x0+v*(t1-t0)
        let dtot = t1 - t0;
        let x_true = Point3D::new(
            x0.x + field.v.x * dtot,
            x0.y + field.v.y * dtot,
            x0.z + field.v.z * dtot,
        );

        let err = (x1 - x_true).length();
        assert!(err < 1e-6, "err too big: {}", err);
    }
```
```rust
    fn make_single_tri3_model() -> FemModel {
        let mut model = FemModel::default();

        // --- Nodes ---
        let n0 = Node::new(0, 0.0, 0.0, 0.0);
        let n1 = Node::new(1, 1.0, 0.0, 0.0);
        let n2 = Node::new(2, 0.0, 1.0, 0.0);

        model.nodes.insert(0, n0);
        model.nodes.insert(1, n1);
        model.nodes.insert(2, n2);

        model.max_node_id = 2;

        // --- Element ---
        let e0 = Element::tri3(0, [0, 1, 2]);
        model.elems.insert(0, e0);
        model.max_elem_id = 0;

        // --- node → elems connectivity ---
        model.node_to_elems.insert(0, vec![0]);
        model.node_to_elems.insert(1, vec![0]);
        model.node_to_elems.insert(2, vec![0]);

        // --- bbox ---
        model.bbox = [
            0.0, 0.0, 0.0,  // min
            1.0, 1.0, 0.0,  // max
        ];

        model
    }
```
```rust
    #[test]
    fn fem_velocity_field_tri3_basic() {
        let model = make_single_tri3_model();

        // nodal velocities: v = (x, y, 0)
        let mut nodal_vel = HashMap::<NodeId, Vector3D>::new();
        nodal_vel.insert(0, Vector3D::new(0.0, 0.0, 0.0));
        nodal_vel.insert(1, Vector3D::new(1.0, 0.0, 0.0));
        nodal_vel.insert(2, Vector3D::new(0.0, 1.0, 0.0));

        let field = FemVelocityField::new(&model, &nodal_vel);

        let p = Point3D::new(0.25, 0.25, 0.0);
        let v = field.velocity_at(p).expect("point inside element");

        assert!((v.x - 0.25).abs() < 1e-12);
        assert!((v.y - 0.25).abs() < 1e-12);
        assert!(v.z.abs() < 1e-12);
    }
```
```rust
    #[test]
    fn pathline_fem_tri3_constant_velocity_matches_analytic() {
        // --- 1) FemModel 구성 (Tri3 하나) ---
        let mut model = FemModel::new(); // 너 코드에 new()가 있다고 했으니 사용. 없으면 Default::default()

        // nodes: id 1,2,3 (0도 되지만 HashMap이면 명시가 안전)
        let n1 = 1u32; let n2 = 2u32; let n3 = 3u32;
        model.nodes.insert(n1, Node::new(n1, 0.0, 0.0, 0.0));
        model.nodes.insert(n2, Node::new(n2, 1.0, 0.0, 0.0));
        model.nodes.insert(n3, Node::new(n3, 0.0, 1.0, 0.0));

        let e1 = 10u32;
        model.elems.insert(e1, Element::new(e1, ElementType::Tri3, vec![n1, n2, n3]));

        // (필요하면 bbox/max_id/node_to_elems 채우기)
        model.max_node_id = 3;
        model.max_elem_id = 10;

        // node_to_elems (최소한 이것도 넣어두는 편이 좋음)
        model.node_to_elems.entry(n1).or_default().push(e1);
        model.node_to_elems.entry(n2).or_default().push(e1);
        model.node_to_elems.entry(n3).or_default().push(e1);

        model.bbox = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0];

        // --- 2) nodal velocity: 상수장 ---
        let v = Vector3D::new(2.0, -1.0, 0.5);
        let mut nodal_vel: HashMap<u32, Vector3D> = HashMap::new();
        nodal_vel.insert(n1, v);
        nodal_vel.insert(n2, v);
        nodal_vel.insert(n3, v);

        let field = FemVelocityField::new(&model, &nodal_vel);

        // --- 3) ode solver (dimension=3) ---
        let solver = OdeSolver::new(3);

        // --- 4) integrate ---
        let x0 = Point3D::new(0.2, 0.2, 0.0);
        let t0: Real = 0.0;
        let t1: Real = 0.1;
        let dt: Real = 0.01;

        let res = on_integrate_pathline_rk4_mesh_with(&solver, &field, x0, t0, t1, dt, None);
        assert!(!res.exited_domain, "should stay inside for this short step");

        // analytic: x(t)=x0+v*(t1-t0)
        let exact = Point3D::new(
            x0.x + v.x * (t1 - t0),
            x0.y + v.y * (t1 - t0),
            x0.z + v.z * (t1 - t0),
        );

        let err = (res.x1 - exact).length();
        assert!(err < 1e-10, "err={} x1={:?} exact={:?}", err, res.x1, exact);
    }
```
```rust
    #[test]
    fn pathline_fem_quad4_constant_velocity_matches_analytic() {

        let mut model = FemModel::new();

        let n1=1u32; let n2=2u32; let n3=3u32; let n4=4u32;
        model.nodes.insert(n1, Node::new(n1, 0.0, 0.0, 0.0));
        model.nodes.insert(n2, Node::new(n2, 1.0, 0.0, 0.0));
        model.nodes.insert(n3, Node::new(n3, 1.0, 1.0, 0.0));
        model.nodes.insert(n4, Node::new(n4, 0.0, 1.0, 0.0));

        let e1=10u32;
        model.elems.insert(e1, Element::new(e1, ElementType::Quad4, vec![n1,n2,n3,n4]));
        model.node_to_elems.entry(n1).or_default().push(e1);
        model.node_to_elems.entry(n2).or_default().push(e1);
        model.node_to_elems.entry(n3).or_default().push(e1);
        model.node_to_elems.entry(n4).or_default().push(e1);
        model.bbox = [0.0,0.0,0.0, 1.0,1.0,0.0];

        let v = Vector3D::new(2.0, -1.0, 0.5);
        let mut nodal_vel: HashMap<u32, Vector3D> = HashMap::new();
        nodal_vel.insert(n1, v); nodal_vel.insert(n2, v);
        nodal_vel.insert(n3, v); nodal_vel.insert(n4, v);

        let field = FemVelocityField::new(&model, &nodal_vel);
        let solver = OdeSolver::new(3);

        let x0 = Point3D::new(0.25, 0.25, 0.0);
        let t0 = 0.0;
        let t1 = 0.1;
        let dt = 0.01;

        let res = on_integrate_pathline_rk4_mesh_with(&solver, &field, x0, t0, t1, dt, None);
        assert!(!res.exited_domain);

        let exact = Point3D::new(
            x0.x + v.x*(t1-t0),
            x0.y + v.y*(t1-t0),
            x0.z + v.z*(t1-t0),
        );

        let err = (res.x1 - exact).length();
        assert!(err < 1e-10, "err={}", err);
    }
```
```rust
    #[test]
    fn pathline_clips_at_boundary_when_exiting_domain() {
        let mut model = FemModel::new();
        let n1=1u32; let n2=2u32; let n3=3u32;
        model.nodes.insert(n1, Node::new(n1, 0.0, 0.0, 0.0));
        model.nodes.insert(n2, Node::new(n2, 1.0, 0.0, 0.0));
        model.nodes.insert(n3, Node::new(n3, 0.0, 1.0, 0.0));
        let e1=10u32;
        model.elems.insert(e1, Element::new(e1, ElementType::Tri3, vec![n1,n2,n3]));
        model.node_to_elems.entry(n1).or_default().push(e1);
        model.node_to_elems.entry(n2).or_default().push(e1);
        model.node_to_elems.entry(n3).or_default().push(e1);
        model.bbox = [0.0,0.0,0.0, 1.0,1.0,0.0];

        // 강한 +x 속도
        let v = Vector3D::new(10.0, 0.0, 0.0);
        let mut nodal_vel: HashMap<u32, Vector3D> = HashMap::new();
        nodal_vel.insert(n1, v); nodal_vel.insert(n2, v); nodal_vel.insert(n3, v);

        let field = FemVelocityField::new(&model, &nodal_vel);
        let solver = OdeSolver::new(3);

        let x0 = Point3D::new(0.2, 0.2, 0.0);
        let res = on_integrate_pathline_rk4_model_interp_with_tol(&solver, &field, x0, 0.0, 1.0, 0.1, None);

        assert!(res.exited_domain, "should exit");
        // inside triangle 조건: x>=0,y>=0,x+y<=1
        let x1 = res.x1.x;
        let y1 = res.x1.y;
        assert!(x1 >= -1e-10);
        assert!(y1 >= -1e-10);
        assert!(x1 + y1 <= 1.0 + 1e-10, "point should be on/inside boundary");
        // 사실상 x가 0.8 근처에서 멈춰야 (0.2 + 10*t = 0.8 => t=0.06) 하지만
        // 삼각형 경계는 x+y<=1 이므로 y=0.2면 x<=0.8 맞음
        assert!((x1 - 0.8).abs() < 1e-6, "x1={} expected ~0.8", x1);
    }
```
```rust
    #[test]
    fn pathline_time_dependent_constant_in_space_linear_in_time_matches_analytic() {

        let mut model = FemModel::new();
        let n1=1u32; let n2=2u32; let n3=3u32;
        model.nodes.insert(n1, Node::new(n1, 0.0, 0.0, 0.0));
        model.nodes.insert(n2, Node::new(n2, 1.0, 0.0, 0.0));
        model.nodes.insert(n3, Node::new(n3, 0.0, 1.0, 0.0));
        let e1=10u32;
        model.elems.insert(e1, Element::new(e1, ElementType::Tri3, vec![n1,n2,n3]));
        model.bbox = [0.0,0.0,0.0, 1.0,1.0,0.0];

        let v0 = Vector3D::new(1.0, 2.0, 0.0);
        let a  = Vector3D::new(3.0, -1.0, 0.0);

        // vel_fn: 모든 노드 동일
        let field = FemVelocityFieldFn::new(&model, move |_nid, t| {
            Some(Vector3D::new(v0.x + a.x*t, v0.y + a.y*t, v0.z + a.z*t))
        });

        let solver = OdeSolver::new(3);

        let x0 = Point3D::new(0.2, 0.2, 0.0);
        let t0 = 0.0;
        let t1 = 0.1;
        let dt = 0.001;

        let res = on_integrate_pathline_rk4_model_interp_with_tol(&solver, &field, x0, t0, t1, dt, None);
        assert!(!res.exited_domain);

        let dt_tot = t1 - t0;
        let exact = Point3D::new(
            x0.x + v0.x*dt_tot + 0.5*a.x*dt_tot*dt_tot,
            x0.y + v0.y*dt_tot + 0.5*a.y*dt_tot*dt_tot,
            x0.z + v0.z*dt_tot + 0.5*a.z*dt_tot*dt_tot,
        );

        let err = (res.x1 - exact).length();
        assert!(err < 1e-6, "err={}", err);
    }
```
```rust
    #[test]
    fn pathline_rk45_time_dependent_is_accurate() {
        let mut model = FemModel::new();
        let n1=1u32; let n2=2u32; let n3=3u32;
        model.nodes.insert(n1, Node::new(n1, 0.0, 0.0, 0.0));
        model.nodes.insert(n2, Node::new(n2, 1.0, 0.0, 0.0));
        model.nodes.insert(n3, Node::new(n3, 0.0, 1.0, 0.0));
        let e1=10u32;
        model.elems.insert(e1, Element::new(e1, ElementType::Tri3, vec![n1,n2,n3]));
        model.bbox = [0.0,0.0,0.0, 1.0,1.0,0.0];

        let v0 = Vector3D::new(1.0, 2.0, 0.0);
        let a  = Vector3D::new(3.0, -1.0, 0.0);

        let field = FemVelocityFieldFn::new(&model, move |_nid, t| {
            Some(Vector3D::new(v0.x + a.x*t, v0.y + a.y*t, v0.z + a.z*t))
        });

        let mut solver = OdeSolver::new(3);
        solver.set_tolerances(1e-10, 1e-8);
        solver.set_step_limits(1e-12, 1e-1);

        let x0 = Point3D::new(0.2, 0.2, 0.0);
        let t0 = 0.0;
        let t1 = 0.1;

        // RK4 (coarse)
        let rk4 = on_integrate_pathline_rk4_model_interp_with_tol(&solver, &field, x0, t0, t1, 0.02, None);
        assert!(!rk4.exited_domain);

        // RK45 (adaptive)
        let rk45 = on_integrate_pathline_rk45(&solver, &field, x0, t0, t1, 0.02);
        assert!(!rk45.exited_domain);

        // analytic
        let dt = t1 - t0;
        let exact = Point3D::new(
            x0.x + v0.x*dt + 0.5*a.x*dt*dt,
            x0.y + v0.y*dt + 0.5*a.y*dt*dt,
            x0.z + v0.z*dt + 0.5*a.z*dt*dt,
        );

        let e4  = (rk4.x1  - exact).length();
        let e45 = (rk45.x1 - exact).length();

        println!("{}, {}", e4, e45);

        assert!(e45 <= e4, "rk45 should be better: e45={} e4={}", e45, e4);
        assert!(e45 < 1e-6, "rk45 should be accurate enough: e45={}", e45);
    }
}
```
---
