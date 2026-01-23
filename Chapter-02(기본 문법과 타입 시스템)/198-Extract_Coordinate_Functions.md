##  extract_coordinate_functions
 ✨ 1. 목적을 수식으로 정리
- NURBS 표면은 다음과 같이 정의:
```math
S(u,v)=\frac{\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)\, M_{j,q}(v)\, w_{ij}\, P_{ij}}{\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)\, M_{j,q}(v)\, w_{ij}}
```
- 여기서
- $P_{ij}=(x_{ij},y_{ij},z_{ij})$
- $w_{ij} = weight$
- $N_{i,p}(u), M_{j,q}(v)$ = B-spline basis functions
- NURBS는 homogeneous 좌표를 사용하므로 제어점은:
```math
P_{ij}^w=(x_{ij}w_{ij},\; y_{ij}w_{ij},\; z_{ij}w_{ij},\; w_{ij})
```

### 📌 extract_coordinate_functions 하는 일
- U_SURCOR는 위의 homogeneous 제어점 배열에서 다음 4개의 스칼라 함수를 추출한다:
```math
\begin{aligned}w_x(u,v)&=\sum _{i,j}N_{i,p}(u)M_{j,q}(v)(x_{ij}w_{ij})\\ w_y(u,v)&=\sum _{i,j}N_{i,p}(u)M_{j,q}(v)(y_{ij}w_{ij})\\ w_z(u,v)&=\sum _{i,j}N_{i,p}(u)M_{j,q}(v)(z_{ij}w_{ij})\\ w(u,v)&=\sum _{i,j}N_{i,p}(u)M_{j,q}(v)w_{ij}\end{aligned}
```
- 즉, homogeneous 좌표의 각 성분을 따로 B-spline surface 형태로 추출하는 함수.
- 이 4개의 함수는 나중에 실제 3D 좌표를 계산할 때 사용:
```math
S(u,v)=\left( \frac{w_x(u,v)}{w(u,v)},\; \frac{w_y(u,v)}{w(u,v)},\; \frac{w_z(u,v)}{w(u,v)}\right)
``` 

### ✨ 2. Rust 코드가 하는 일 요약
- Rust 함수 extract_coordinate_functions()는 정확히 위 수식의 제어점 부분을 꺼내서 SFun 구조에 채워 넣는 역할을 한다.
- ✔ 1) 제어점 접근
```rust
fn ctrl_at(&self, iu, iv) -> &Point4D {
    &self.ctrl[iu + nu * iv]
}
```

- 여기서 Point4D = (xw, yw, zw, w)
- 즉, homogeneous 좌표.

- ✔ 2) wx, wy, wz, w에 제어점 값 채우기
- Rust 코드의 이 부분이 핵심:
```rust
for i in 0..nu {
    for j in 0..nv {
        let cp = self.ctrl_at(i, j);

        wx.set(i, j, cp.x);
        wy.set(i, j, cp.y);
        wz.set(i, j, cp.z);

        if rat {
            if let Some(ref mut ww) = w {
                ww.set(i, j, cp.w);
            }
        }
    }
}
```

### 🔍 전체 흐름 요약 (Rust 버전)
- 이 함수는 C 코드의 extract_coordinate_functions와 동일한 역할을 한다:
    - NURBS surface의 control point $P_{ij}=(Xw,Yw,Zw,w)$ 에서
        $wx(u,v), wy(u,v), wz(u,v), w(u,v)$ 함수들을 추출한다.
    - rational surface면 w까지 채우고
    - non‑rational이면 w는 무시하거나 clear한다.
    - knot vector도 그대로 복사한다.
- 즉, surface를 구성하는 4개의 scalar function을 분리해내는 작업.

### 📘 Rust 스타일로 정리
- ✔ 목적
- NURBS surface S(u,v)는 homogeneous control point로 구성된다:
```math
P_{ij}=(Xw_{ij},Yw_{ij},Zw_{ij},w_{ij})
```
- 이를 다음 4개의 scalar function으로 분리한다:
- $wx(u,v)=Xw(u,v)$
- $wy(u,v)=Yw(u,v)$
- $wz(u,v)=Zw(u,v)$
- $w(u,v)=w(u,v)$ (rational일 때만)
- 이 함수들은 이후:
    - Dehomogenization
    - Surface evaluation
    - Derivative computation
    - Knot refinement
    - Subdivision
- 등에서 사용된다.

### 📐 Rust 코드의 핵심 수식 정리
- 1) control point 접근
```rust
&self.ctrl[iu + nu * iv]
```

- 즉,
```math
P_{ij}=\mathrm{ctrl}[i+(n+1)\cdot j]
```
- 2) 좌표 분리
```rust
wx(i,j) = cp.x
wy(i,j) = cp.y
wz(i,j) = cp.z
w(i,j)  = cp.w   (rational일 때)
```

- 3) knot vector 복사
```rust
wx.ku = self.ku
wy.ku = self.ku
wz.ku = self.ku
w.ku  = self.ku (if rational)
```

- 그리고 v 방향도 동일.

## ensure_sfun_shape
- ensure_sfun_shape()는 다음을 보장하는 함수:
    - value 배열 크기 보장
    - 차수(p, q) 갱신
    - knot vector 길이 보장
- 즉, SFun이 surface의 shape에 맞게 정확한 크기와 구조를 갖도록 강제하는 함수.

### ✅ 1) value 버퍼 크기 보장 부분
```rust
let nu = n + 1;
let nv = m + 1;
let need = (nu as usize) * (nv as usize);

if out.nu != nu || out.nv != nv || out.values.len() != need {
    out.nu = nu;
    out.nv = nv;
    out.values.resize(need, 0.0);
}
```
- nu = n+1, nv = m+1 → NURBS 정의와 정확히 일치
- row-major로 nu * nv
- shape이 다르면 resize → 안전
- 초기값 0.0 → 합리적

- ✔ Rust 관점에서 완벽한 메모리 안전
    - resize는 기존 데이터를 유지하고 부족한 부분만 0.0으로 채움
    - capacity는 자동 관리
    - 슬라이스 인덱싱 안전

### ✅ 2) 차수(p, q) 갱신
```rust
out.pu = p;
out.pv = q;
```
- ✔ 문제 없음
    - SFun은 surface의 degree를 그대로 가져야 하므로 정확함
    - 이 값은 이후 basis function 계산에서 필요

### ✅ 3) knot vector 길이 보장
```rust
let rr = (r as usize) + 1;
let ss = (s as usize) + 1;

if out.ku.len() != rr {
    out.ku.resize_len(rr, 0.0);
}
if out.kv.len() != ss {
    out.kv.resize_len(ss, 0.0);
}
```

- ✔ 정확한 처리
    - knot vector 길이는 마지막 인덱스 + 1
    - C 코드의 U_sfnchk와 동일
    - resize_len()은 너가 만든 고정 길이 vector 타입일 가능성이 높음
    - 0.0으로 초기화 → 이후 copy 단계에서 덮어쓰기 때문에 문제 없음

- Rust에서 SFun 같은 구조를 다룰 때 가장 중요한 건:
    - shape mismatch 방지
    - 메모리 안전성
    - 슬라이스 인덱싱 안전
    - knot vector 길이 보장
    - degree 일관성 유지
- 이 함수는 이 모든 걸 충족하고 있음.

---

## 소스 코드
```rust
/// - wx,wy,wz: 항상 채움
/// - w: surface가 rational일 때만 채움 (비-rational이면 untouched)
/// 반환값: surface가 rational이면 true
pub fn extract_coordinate_functions(
    &self,
    wx: &mut SFun,
    wy: &mut SFun,
    wz: &mut SFun,
    mut w: Option<&mut SFun>,
) -> bool {
    // ---- local notation (C: U_surbre + U_surknp) ----
    let (n, m, r, s) = self.indices();      // last indices
    let (p, q) = self.deg();                // degrees
    let rat = self.is_rational();

    // ---- ensure memory (C: U_sfnchk + U_sfnfuv) ----
    ensure_sfun_shape(wx, n, m, p, q, r, s);
    ensure_sfun_shape(wy, n, m, p, q, r, s);
    ensure_sfun_shape(wz, n, m, p, q, r, s);

    if rat {
        if let Some(ref mut ww) = w {
            ensure_sfun_shape(ww, n, m, p, q, r, s);
        } else {
            // rational인데 w 저장용 버퍼가 안 들어오면,
        }
    }

    let nu = (n + 1) as usize;
    let nv = (m + 1) as usize;

    for i in 0..nu {
        for j in 0..nv {
            let cp = self.ctrl_at(i, j);

            wx.set(i, j, cp.x);
            wy.set(i, j, cp.y);
            wz.set(i, j, cp.z);

            if rat {
                if let Some(ref mut ww) = w {
                    ww.set(i, j, cp.w);
                }
            }
        }
    }

    // ---- copy knots (C: UX/UY/UZ/(UW) and VX/VY/VZ/(VW)) ----
    // SFun의 knot vector는 KnotVector 내부 knots를 그대로 갱신
    // ensure_sfun_shape()가 길이를 맞춰놨기 때문에 인덱스 대입 OK.
    for i in 0..=(r as usize) {
        let ui = self.ku.knots[i];
        wx.ku.knots[i] = ui;
        wy.ku.knots[i] = ui;
        wz.ku.knots[i] = ui;
        if rat {
            if let Some(ref mut ww) = w {
                ww.ku.knots[i] = ui;
            }
        }
    }

    for j in 0..=(s as usize) {
        let vj = self.kv.knots[j];
        wx.kv.knots[j] = vj;
        wy.kv.knots[j] = vj;
        wz.kv.knots[j] = vj;
        if rat {
            if let Some(ref mut ww) = w {
                ww.kv.knots[j] = vj;
            }
        }
    }
    rat
}
```
```rust
/// 편의 함수: w까지 반드시 받고 싶을 때 (rational 아니면 w는 clear해둘 수도 있음)
pub fn extract_coordinate_functions_with_w(
    &self,
    wx: &mut SFun,
    wy: &mut SFun,
    wz: &mut SFun,
    w: &mut SFun,
) -> bool {
    let rat = self.extract_coordinate_functions(wx, wy, wz, Some(w));
    if !rat {
        // 비-rational이면 C처럼 "w를 반환 안 한다"가 원칙이지만
        // Rust에서는 호출자가 실수로 쓰는 걸 막으려면 clear가 안전.
        w.clear();
    }
    rat
}
```

---

## 테스트 코드
```rust
use nurbslib::core::circle::Circle;
use nurbslib::core::geom::{Point3D, Point4D, Vector3D};
use nurbslib::core::nurbs_curve::NurbsCurve;
use nurbslib::core::nurbs_surface::NurbsSurface;
use nurbslib::core::plane::Plane;
use nurbslib::core::sfun::SFun;
use nurbslib::core::tensor_product::RevolutionTensor;

fn dist2(a: Point3D, b: Point3D) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx*dx + dy*dy + dz*dz
}
```
```rust
#[test]
fn extract_coord_func_non_rational_plane() {
    let surf = NurbsSurface::from_plane_xy().unwrap();
    assert!(!surf.is_rational());

    let mut wx = SFun::default();
    let mut wy = SFun::default();
    let mut wz = SFun::default();
    let mut w  = SFun::default(); // 있어도 non-rat이면 안 채워짐

    let rat = surf.extract_coordinate_functions(
        &mut wx, &mut wy, &mut wz, Some(&mut w)
    );
    assert!(!rat);

    // knot vector 복사 확인
    assert_eq!(wx.ku.knots, surf.ku.knots);
    assert_eq!(wx.kv.knots, surf.kv.knots);

    let us = [0.0, 0.25, 0.5, 0.75, 1.0];
    let vs = [0.0, 0.4, 0.8, 1.0];

    for &u in &us {
        for &v in &vs {
            let p = surf.eval_point(u, v);

            let q = Point3D {
                x: wx.eval(u, v),
                y: wy.eval(u, v),
                z: wz.eval(u, v),
            };

            assert!(
                dist2(p, q) < 1e-14,
                "plane mismatch at ({u},{v})"
            );
        }
    }
}
```
```rust
/// rational surface 하나 생성:
/// - arc: XY 평면의 unit circle (rational NurbsCurve)
/// - shape: XZ 평면의 수직 라인(비유리 NurbsCurve)
/// - rho: world axes 기준 회전 텐서
///
/// 결과: "원통류" 형태의 rational NURBS surface (arc가 rational이므로 surface도 rational)
fn make_rational_revolution_surface() -> NurbsSurface {
    // 1) arc: unit circle on world XY
    let circle = Circle::from_plane_radius(Plane::world_xy(), 1.0);
    let arc = NurbsCurve::from_circle(&circle); // ✅ 존재함 (quadratic rational circle)

    // 2) shape: line segment (radius=1, z=0..2)
    let p0 = Point3D::new(1.0, 0.0, 0.0);
    let p1 = Point3D::new(1.0, 0.0, 2.0);
    let shape = NurbsCurve::from_line(p0, p1); // ✅ 존재함

    // 3) rho: world frame (identity)
    let rho = RevolutionTensor {
        origin: Point3D::new(0.0, 0.0, 0.0),
        x_axis: Vector3D::new(1.0, 0.0, 0.0),
        y_axis: Vector3D::new(0.0, 1.0, 0.0),
        z_axis: Vector3D::new(0.0, 0.0, 1.0),
    };

    // 4) build surface
    NurbsSurface::from_revolution_tensor(&arc, &shape, &rho)
}
```
```rust
#[test]
fn extract_coord_func_rational_surface() {
    let surf = make_rational_revolution_surface();
    assert!(surf.is_rational());

    let mut wx = SFun::default();
    let mut wy = SFun::default();
    let mut wz = SFun::default();
    let mut w  = SFun::default();

    let rat = surf.extract_coordinate_functions(
        &mut wx, &mut wy, &mut wz, Some(&mut w)
    );
    assert!(rat);

    // knot 복사
    assert_eq!(wx.ku.knots, surf.ku.knots);
    assert_eq!(wx.kv.knots, surf.kv.knots);
    assert_eq!(w.ku.knots,  surf.ku.knots);
    assert_eq!(w.kv.knots,  surf.kv.knots);

    let us = [0.0, 0.15, 0.33, 0.67, 1.0];
    let vs = [0.0, 0.5, 1.0];

    for &u in &us {
        for &v in &vs {
            let p = surf.eval_point(u, v);

            let ww = w.eval(u, v);
            assert!(ww.abs() > 1e-12);

            let q = Point3D {
                x: wx.eval(u, v) / ww,
                y: wy.eval(u, v) / ww,
                z: wz.eval(u, v) / ww,
            };

            assert!(
                dist2(p, q) < 1e-12,
                "rational mismatch at ({u},{v})"
            );
        }
    }
}
```
```rust
#[test]
fn extract_coord_func_control_net_matches_ctrl_points() {
    let surf = NurbsSurface::from_plane_xy().unwrap();

    let mut wx = SFun::default();
    let mut wy = SFun::default();
    let mut wz = SFun::default();

    surf.extract_coordinate_functions(&mut wx, &mut wy, &mut wz, None);

    let nu = surf.nu;
    let nv = surf.nv;

    for i in 0..nu {
        for j in 0..nv {
            let cp = surf.ctrl[i + nu*j];
            assert_eq!(wx.get(i, j), cp.x);
            assert_eq!(wy.get(i, j), cp.y);
            assert_eq!(wz.get(i, j), cp.z);
        }
    }
}
```
---
