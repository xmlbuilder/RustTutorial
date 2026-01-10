# Conic Bézier Arc

- 3D 공간에서 **원뿔형 베지어 곡선(Conic Bézier Arc)** 을 구성하기 위한 수학적 절차를 구현한 것으로,  
 수식적으로 매우 타당한 구조를 갖고 있습니다.  
- 아래에 전체 흐름을 수식 기반으로 해석, 단계별 설명, 그리고 핵심 수식 요약으로 정리.

## 📘 전체 목적
주어진 세 점과 접선 벡터를 기반으로, Conic Bézier Curve의 중간 제어점 $P_1$ 과 가중치 $w_1$ 를 계산합니다.  
### 입력:
- 시작점 $P_0$, 끝점 $P_2$, 중간점 $P$
- 접선 벡터 $T_0$, $T_2$

### 이미지

![Conic Bezier Arc](/image/conic_Bezier_arc.png)


### 출력:
- 제어점 $P_1$
- 가중치 $w_1$

## 🧠 단계별 수식 해석
### ① 평면 프레임 생성 (on_make_frame_plane)
- 벡터 정의:

$$
\vec {a}=P_2-P_0,\quad \vec {b}=P-P_0
$$

- 법선 벡터:

$$
\vec {z}=\vec {a}\times \vec {b},\quad \vec {z}\leftarrow \frac{\vec {z}}{|\vec {z}|}
$$

- 기준 축:

$$
\vec {x}=\frac{\vec {a}}{|\vec {a}|},\quad \vec {y}=\vec {z}\times \vec {x}
$$

### ② 3D → 2D 투영 (on_project_point_2d)
- 투영된 점:

$$
P_i^{2D}=\left( (P_i-O)\cdot \vec {x},\  (P_i-O)\cdot \vec {y}\right)
$$

- 투영된 벡터:

$$
T_i^{2D}=\left( T_i\cdot \vec {x},\  T_i\cdot \vec {y}\right)
$$ 

### ③ 접선 교차점 계산 (on_intersect_lines_2d)
- 두 직선:

$$
L_0(t)=P_0^{2D}+t\cdot T_0^{2D},\quad L_2(s)=P_2^{2D}+s\cdot T_2^{2D}
$$

- 교차점 $P_1^{2D}$ 

#### 계산:

$$
\mathrm{det}=T_0^{2D}\times T_2^{2D}\neq 0\Rightarrow \mathrm{교차\  가능}
$$


### ④ 중간점과 교차점 사이의 위치 비율 계산
- 선분 $P_0^{2D}\rightarrow P_2^{2D}$ 와 직선 $P_1^{2D}\rightarrow P^{2D}$ 의 교차 비율:

$$
t_{\mathrm{seg}}=\mathrm{교차\  위치\  비율}\quad a=\sqrt{\frac{t_{\mathrm{seg}}}{1-t_{\mathrm{seg}}}},\quad u=\frac{a}{1+a}
$$


### ⑤ 가중치 계산
- 벡터 정의:

$$
\vec {v}_0=P-P_0,\quad \vec {v}_1=P_1-P,\quad \vec {v}_2=P-P_2
$$

- 내적:

$$
\alpha =\vec {v}_0\cdot \vec {v}_1,\quad \beta =\vec {v}_1\cdot \vec {v}_2,\quad \gamma =\vec {v}_1\cdot \vec {v}_1
$$

- 가중치:

$$
w_1=\frac{(1-u)^2\cdot \alpha +u^2\cdot \beta }{2u(1-u)\cdot \gamma }
$$

### ⑥ 평면 복원
- 2D 제어점 $P_1^{2D}$ 을 3D로 복원:

$$
P_1=O+P_1^{2D}.x\cdot \vec {x}+P_1^{2D}.y\cdot \vec {y}
$$

### ⑦ 평행 접선 처리 (Parabola Branch)
- 접선이 평행할 경우, $w_1=0$
- $P_1$ 은 $T_0$ 방향으로 스케일된 벡터로 표현됨

## ✅ 수식적 타당성 검토
- 기하학적으로 완전 타당: 평면 생성 → 투영 → 교차점 → 위치 비율 → 가중치 계산 → 복원
- 수치적 안정성 고려: eps로 병렬성, 분모 0, 경계값 처리
- 분기 처리 명확: 일반형 vs. 평행형
- 벡터 연산과 내적 기반: Bézier 곡선의 정의에 부합

---
## 소스 크드
```rust
pub fn on_make_frame_plane(
    p0: Point,
    p2: Point,
    p: Point,
) -> Option<(Point, Vector, Vector, Vector)> {
    let origin = p0;

    // a = P0->P2, b = P0->P
    let a = (p2 - p0).to_vector();
    let b = (p - p0).to_vector();

    // z = a × b
    let mut z_axis = a.cross(&b);
    z_axis = z_axis.unitize();

    // x = unit(a)
    let mut x_axis = a;
    x_axis = x_axis.unitize();

    // y = z × x
    let mut y_axis = z_axis.cross(&x_axis);
    y_axis = y_axis.unitize();

    Some((origin, x_axis, y_axis, z_axis))
}
```
```rust
// ---- Intersect two 2D lines: A + t*U and B + s*V ----
// returns None if parallel
pub fn on_intersect_lines_2d(
    a: Point2,
    u: Point2,
    b: Point2,
    v: Point2,
) -> Option<(f64, f64, Point2)> {
    let det = u.x * v.y - u.y * v.x;
    let eps = 1e-15;
    if det.abs() <= eps {
        return None;
    }
    let w = b + a;
    let t = (w.x * v.y - w.y * v.x) / det;
    let s = (w.x * u.y - w.y * u.x) / det;
    let int_pt = a + u * t;
    Some((t, s, int_pt))
}
```
```rust
fn on_project_vec_2d(v: Vector, x_axis: Vector, y_axis: Vector) -> Point2 {
    Point2::new(v.dot(&x_axis), v. dot(&y_axis))
}
```
```rust
fn on_project_point_2d(p: Point, origin: Point, x_axis: Vector, y_axis: Vector) -> Point2 {
    let v = (p - origin).to_vector();
    Point2::new(v.dot(&x_axis), v.dot(&y_axis))
}
```
```rust
pub fn on_make_bezier_conic_arc(
    p0: Point,
    t0: Vector,
    p2: Point,
    t2: Vector,
    p: Point,
) -> Option<(Point, Real)> {
    // 1) build a local plane frame
    let (o, x_axis, y_axis, _z_axis) = on_make_frame_plane(p0, p2, p)?;

    // 2) project to 2D
    let p0_2 = on_project_point_2d(p0, o, x_axis, y_axis);
    let p2_2 = on_project_point_2d(p2, o, x_axis, y_axis);
    let pp_2 = on_project_point_2d(p, o, x_axis, y_axis);

    let t0_2 = on_project_vec_2d(t0, x_axis, y_axis);
    let t2_2 = on_project_vec_2d(t2, x_axis, y_axis);

    // 3) try intersection of tangents (non-parallel case)
    if let Some((_tau0, _tau2, p1_2)) = on_intersect_lines_2d(p0_2, t0_2, p2_2, t2_2) {
        // Intersect segment p0-p2 with line (p1 -- p)
        let seg = p2_2 - p0_2;
        let dir = pp_2 - p1_2;

        if let Some((tseg, _tl, _m)) = on_intersect_lines_2d(p0_2, seg, p1_2, dir) {
            let eps = 1e-15;
            if tseg < -1e-12 || tseg > 1.0 + 1e-12 {
                return None;
            }
            if (1.0 - tseg).abs() <= eps {
                return None;
            }

            let a = (tseg / (1.0 - tseg)).sqrt();
            let u = a / (1.0 + a);

            // vectors for dot products
            let v0 = pp_2 - p0_2;
            let v1 = p1_2 - pp_2;
            let v2 = pp_2 - p2_2;

            let alf = v0.dot(&v1);
            let bet = v1.dot(&v2);
            let gam = v1.dot(&v1);

            let a_ = (1.0 - u) * (1.0 - u);
            let b_ = u * u;
            let c_ = 2.0 * u * (1.0 - u);

            let num = a_ * alf + b_ * bet;
            let den = c_ * gam;
            if den.abs() <= eps {
                return None;
            }
            let w1 = num / den;

            // lift p1 back to 3D
            let p1 = o + (x_axis * p1_2.x + y_axis * p1_2.y).to_point();
            return Some( (p1, w1 ));
        }
        return None;
    }

    // 4) parallel tangents → parabola branch
    // Intersect line L = (P, T0) with segment S = (P0 -> P2)
    {
        let a = pp_2;
        let u = t0_2;
        let b = p0_2;
        let v = p2_2 - p0_2;

        if let Some((tt, ts, _x)) = on_intersect_lines_2d(a, u, b, v) {
            let eps = 1e-15;
            if (1.0 - ts).abs() <= eps {
                return None;
            }
            if ts < -1e-12 || ts > 1.0 + 1e-12 {
                return None;
            }

            let aa = (ts / (1.0 - ts)).sqrt();
            let u = aa / (1.0 + aa);
            let b = 2.0 * u * (1.0 - u);

            let num = -tt * (1.0 - b);
            if b.abs() <= eps {
                return None;
            }
            let scale = num / b;

            // w1 = 0, and p1 encodes a 3D vector along T0 (no origin)
            let mut t0u = t0;
            if t0u.length_squared() > 0.0 {
                // keep original scale (do not normalize)
                let v3 = t0u * scale;
                let p1_as_vec = Point::new(v3.x, v3.y, v3.z);
                return Some((p1_as_vec, 0.0 ));
            } else {
                return Some((Point::new(0.0, 0.0, 0.0), 0.0 ));
            }
        }
    }

    None
}
```
---

