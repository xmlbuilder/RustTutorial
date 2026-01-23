## on_make_frame_plane

- **세 점(P0, P2, P)** 을 이용해 **평면 위의 로컬 직교 좌표계(frame)** 를 만드는 구현.
- 특히 CAD/Geometry 엔진에서 **평면 기반 로컬 프레임 생성** 은 자주 등장하는 패턴인데, 이 함수는 그 핵심을 정확하게 담고 있음.

### 📘 on_make_frame_plane(p0, p2, p)
- 세 점을 이용해 평면 위의 직교정규 프레임(origin, x, y, z)을 생성

### 1️⃣ 목적
- 입력:
    - p0: 프레임의 원점
    - p2: x축 방향을 정의하는 점
    - p: 평면의 법선 방향을 정의하는 보조 점
- 출력:
    - (origin, x_axis, y_axis, z_axis)
    - 모두 직교정규(orthonormal)
    - 평면 위에서 안정적인 로컬 좌표계 생성

### 2️⃣ 벡터 정의
```rust
let a = (p2 - p0).to_vector(); // x축 후보
let b = (p - p0).to_vector();  // 평면 내 다른 방향
```

- a는 P0→P2
- b는 P0→P
- 두 벡터는 평면을 정의하는 데 사용됨

### 3️⃣ z축 생성 — 평면의 법선
```rust
let mut z_axis = a.cross(&b);
z_axis = z_axis.unitize();
```

- 수식:
```math
Z=\frac{a\times b}{\| a\times b\| }
```
- 두 벡터의 외적 → 평면의 법선
- 단위 벡터로 정규화

### 4️⃣ x축 생성 — P0→P2 방향
```rust
let mut x_axis = a;
x_axis = x_axis.unitize();
```
```math
X=\frac{a}{\| a\| }
```

### 5️⃣ y축 생성 — z × x
```rust
let mut y_axis = z_axis.cross(&x_axis);
y_axis = y_axis.unitize();
```
```math
Y=\frac{Z\times X}{\| Z\times X\| }
```
- 이 순서는 **오른손 좌표계(Right-handed frame)** 를 보장한다.

### 6️⃣ 최종 반환
```rust
Some((origin, x_axis, y_axis, z_axis))
```


### 📌 최종 요약
- 이 함수는 다음을 수행한다:
    - P0을 origin으로 사용
    - P0→P2 방향을 x축으로 설정
    - P0→P와의 외적으로 z축(법선) 생성
    - z × x로 y축 생성
    - 모든 축을 정규화하여 직교정규 프레임 완성


### 📌 개선 포인트(선택)
- 실전에서는 다음을 고려할 수 있음:
- a와 b가 거의 평행하면 cross가 0에 가까워짐 → None 반환 처리
- unitize() 실패 시 체크
- y축 생성 후 x축과 z축을 다시 재정렬해 수치 drift 제거


```rust
pub fn on_make_frame_plane(
    p0: Point3D,
    p2: Point3D,
    p: Point3D,
) -> Option<(Point3D, Vector3D, Vector3D, Vector3D)> {
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
## on_intersect_lines_2d

- 2D 직선 교차 공식은 다음과 같다:
- 두 직선:
```math
A+tU,\quad B+sV
```
- 교차 조건:
```math
A+tU=B+sV
```
- 정리하면:
```math
t=\frac{(B-A)\times V}{U\times V}
```
```math
s=\frac{(B-A)\times U}{U\times V}
```
- 여기서 2D cross는:
```math
(x_1,y_1)\times (x_2,y_2)=x_1y_2-y_1x_2
```
- 즉, w = B - A가 맞다.

```rust
pub fn on_intersect_lines_2d(
    a: Point2D,
    u: Point2D,
    b: Point2D,
    v: Point2D,
) -> Option<(f64, f64, Point2D)> {
    let det = u.x * v.y - u.y * v.x;
    let eps = 1e-15;
    if det.abs() <= eps {
        return None; // parallel or nearly parallel
    }

    // Correct: difference of points
    let w = b - a;

    let t = (w.x * v.y - w.y * v.x) / det;
    let s = (w.x * u.y - w.y * u.x) / det;

    let int_pt = a + u * t;
    Some((t, s, int_pt))
}
```


### 📌 요약
- w = b - a
- det = U×V 로 평행성 체크도 정확함
- t, s 계산식은 w만 고치면 완벽하게 맞음

```rust
// ---- Intersect two 2D lines: A + t*U and B + s*V ----
// returns None if parallel
pub fn on_intersect_lines_2d(
    a: Point2D,
    u: Point2D,
    b: Point2D,
    v: Point2D,
) -> Option<(f64, f64, Point2D)> {
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
## on_project_vec_2d

- 3D 벡터의 2D 평면 투영(project) 함수

### 📘 on_project_vec_2d(v, x_axis, y_axis)
```rust
- 3D 벡터 v를 (x_axis, y_axis)로 정의된 2D 좌표계로 투영
fn on_project_vec_2d(v: Vector3D, x_axis: Vector3D, y_axis: Vector3D) 
    -> Point2D {
    Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
}
```

### 1️⃣ 목적
- 3D 공간의 벡터 v를
    로컬 2D 좌표계 (x_axis,y_axis) 위로 투영하여
- 2D 좌표 (u,v)를 얻는다.
- 즉:
```math
u=v\cdot x\_ axis,\quad v=v\cdot y\_ axis
```
- 이 함수는:
    - 평면 좌표계로의 투영
    - 로컬 프레임 기반 2D 매핑
    - 텍스처 좌표 생성
    - 3D → 2D 변환
    - CAD/Geometry에서의 평면 파라미터화 같은 곳에서 매우 자주 쓰인다.

### 2️⃣ 수학적 의미
- 입력:
    - v: 3D 벡터
    - x_axis: 3D 단위 벡터 (평면의 x축)
    - y_axis: 3D 단위 벡터 (평면의 y축)
- 출력:
```math
(u,v)=(v\cdot x\_ axis,\; v\cdot y\_ axis)
```
- 즉,
    - v를 두 축에 정사영한 길이를 2D 좌표로 사용한다.

### 3️⃣ 전제 조건
- x_axis와 y_axis는 **직교정규(orthonormal)** 이어야 한다.
- 보통 on_make_frame_plane 또는 on_make_frame_matrix 같은 프레임 생성 함수로 만든 축을 사용한다.

### 4️⃣ 예시
- 만약:
    - x축 = (1,0,0)
    - y축 = (0,1,0)
    - v = (3,5,7)
- 이면:
```math
u=3,\quad v=5
```
- 즉, z축 성분은 무시되고 XY 평면으로 투영된다.

### 📌 최종 요약
- 이 함수는:
    - 3D 벡터를 로컬 2D 좌표계로 투영하는 가장 기본적이고 정확한 방식
    - dot product 두 번으로 끝나는 매우 빠른 연산
    - 프레임 기반 기하 알고리즘에서 필수적인 구성 요소

```rust
fn on_project_vec_2d(v: Vector3D, x_axis: Vector3D, y_axis: Vector3D) -> Point2D {
    Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
}
```
## on_project_point_2d
- on_project_vec_2d의 점(point) 버전으로, 3D 점을 로컬 2D 좌표계로 투영하는 가장 정석적인 구현.


### 📘 on_project_point_2d(p, origin, x_axis, y_axis)
- 3D 점 p를 (origin, x_axis, y_axis)로 정의된 2D 평면 좌표계로 투영
```rust
fn on_project_point_2d(
    p: Point3D,
    origin: Point3D,
    x_axis: Vector3D,
    y_axis: Vector3D,
) -> Point2D {
    let v = (p - origin).to_vector();
    Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
}
```


### 1️⃣ 목적
- 3D 공간의 점 p를 로컬 평면 좌표계 
    (origin, x_axis, y_axis) 위로 투영하여 2D 좌표 (u,v)를 얻는다.
- 즉:
```math
u=(p-origin)\cdot x\_ axis,\quad v=(p-origin)\cdot y\_ axis
```
- 이 함수는 다음과 같은 상황에서 매우 자주 쓰인다:
    - 평면 파라미터화
    - 3D → 2D 매핑
    - 텍스처 좌표 생성
    - 로컬 프레임 기반 기하 계산
    - 폴리곤을 평면에 투영해 2D 알고리즘 적용

### 2️⃣ 수학적 의미
- 입력:
    - origin: 2D 좌표계의 원점
    - x_axis: 3D 단위 벡터 (평면의 x축)
    - y_axis: 3D 단위 벡터 (평면의 y축)
    - p: 투영할 3D 점
- 계산:
```rust
let v = (p - origin).to_vector();
```
```rust
v=p-origin
```
- 그다음:
```rust
Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
```
```math
(u,v)=(v\cdot x\_ axis,\; v\cdot y\_ axis)
```
- 즉,
- origin을 기준으로 한 벡터를 두 축에 정사영한 길이가 2D 좌표가 된다.

### 3️⃣ 전제 조건
- x_axis와 y_axis는 **직교정규(orthonormal)** 이어야 한다.  
    (보통 on_make_frame_plane 또는 on_make_frame_matrix로 생성)
- origin은 평면 위의 기준점.

### 4️⃣ 예시
- 평면이 XY 평면이고:
    - origin = (0,0,0)
    - x_axis = (1,0,0)
    - y_axis = (0,1,0)
    - p = (3,5,7)
- 이면:
```math
u=3,\quad v=5
```
- 즉, z축 성분은 무시되고 XY 평면으로 투영된다.

### 📌 최종 요약
- 이 함수는:
    - 3D 점을 로컬 2D 평면 좌표계로 투영하는 가장 기본적이고 정확한 방식
    - dot product 두 번으로 끝나는 매우 빠른 연산
    - CAD/Geometry에서 평면 기반 알고리즘을 적용하기 위한 필수 도구

```rust
fn on_project_point_2d(p: Point3D, origin: Point3D, x_axis: Vector3D, y_axis: Vector3D) -> Point2D {
    let v = (p - origin).to_vector();
    Point2D::new(v.dot(&x_axis), v.dot(&y_axis))
}
```
## on_make_bezier_conic_arc

- 이 함수는 **세 점(P0, P2, P)과 두 접선(T0, T2)** 을 이용해  
    **정확히 그 세 점을 지나는 2차(Conic) Bézier 호(arc)** 를 구성하는 고급 알고리즘.
- CAD 엔진에서 원호·타원호·포물선 조각을 Bézier로 표현할 때 쓰는 바로 그 방식

### 📘 on_make_bezier_conic_arc
- 세 점과 양 끝 접선으로부터 2차 Bézier Conic Arc(가중치 w1 포함)를 구성

### 1️⃣ 목적
- 입력:
    - P0: 시작점
    - T0: 시작점에서의 접선
    - P2: 끝점
    - T2: 끝점에서의 접선
    - P: Bézier 곡선이 지나야 하는 중간 점
- 출력:
    - P1: Bézier 중간 제어점
    - w1: Conic weight (원·타원·쌍곡선·포물선 결정)
- 즉,
```math
B(t) = (1−t)² P0 + 2 * w1 * t(1−t) P1 + t² * P2
```
- 이 곡선이 P0, P, P2를 지나고,
- 양 끝에서의 접선이 T0, T2가 되도록 P1과 w1을 찾는 함수다.

### 2️⃣ 전체 알고리즘 개요
- 세 점(P0, P2, P)으로 평면 프레임 생성
- 3D → 2D 투영
- 두 접선의 교점 P1₂ 찾기
- 접선이 교차하면 일반 conic
- 접선이 평행하면 parabola branch
- P1₂와 weight w1 계산
- P1₂를 다시 3D로 lift
- (P1, w1) 반환

### 3️⃣ 코드 흐름 상세
- ✔ 1) 평면 프레임 생성
```rust
let (o, x_axis, y_axis, _z_axis) = on_make_frame_plane(p0, p2, p)?;
```

- P0을 origin
- P0→P2를 x축
- (P0→P2)×(P0→P)로 z축
- y축은 z × x
- 즉, 세 점이 정의하는 평면 위의 직교정규 프레임

- ✔ 2) 3D → 2D 투영
```rust
let p0_2 = on_project_point_2d(p0, o, x_axis, y_axis);
...
let t0_2 = on_project_vec_2d(t0, x_axis, y_axis);
```

- 모든 점과 접선을 2D로 변환
- 이후 모든 계산은 2D에서 수행 → 안정적이고 단순

- ✔ 3) 접선 교차점 찾기 (일반 conic branch)
```rust
if let Some((_tau0, _tau2, p1_2)) =
    on_intersect_lines_2d(p0_2, t0_2, p2_2, t2_2)
```

- 두 접선이 교차하면 그 교점이 Bézier의 P1 후보
- 이 경우 conic weight w1 ≠ 0
- 이어서 P1₂–P₂₂ 선과 P0₂–P₂₂ 선의 교차로 파라미터 tseg 계산
```rust
let seg = p2_2 - p0_2;
let dir = pp_2 - p1_2;
```

- tseg = P가 Bézier chord(P0–P2)에서 차지하는 비율
- tseg ∈ (0,1) 이어야 유효
- weight 계산
```rust
let a = (tseg / (1.0 - tseg)).sqrt();
let u = a / (1.0 + a);
```

- 이 u는 conic parameterization에서 쓰는 내부 파라미터.
- 그다음 dot product 기반 weight 공식:
```math
w_1=\frac{(1-u)^2(v_0\cdot v_1)+u^2(v_1\cdot v_2)}{2u(1-u)(v_1\cdot v_1)}
```
- 코드:
```rust
let num = a_ * alf + b_ * bet;
let den = c_ * gam;
let w1 = num / den;
```

- ✔ 4) 접선이 평행한 경우 (parabola branch)
```
// parallel tangents → parabola branch
```
- 두 접선이 평행하면 conic weight w1 = 0
- 이 경우 P1은 “벡터 형태”로 반환  
    (즉, P1은 실제 점이 아니라 T0 방향으로의 벡터)
- 이 branch는 포물선 형태의 conic을 의미한다.

- ✔ 5) 2D P1₂를 다시 3D로 lift
```rust
let p1 = o + (x_axis * p1_2.x + y_axis * p1_2.y).to_point();
```

- 평면 프레임을 이용해 3D로 복원
- 최종 Bézier 제어점 P1

### 📌 최종 요약
- 이 함수는:
    - 세 점(P0, P2, P)
    - 두 접선(T0, T2)
- 을 만족하는 정확한 2차 Conic Bézier 호를 구성한다.
- 출력:
    - P1: 중간 제어점
    - w1: conic weight
    - w1 = 1 → 원/타원
    - w1 = 0 → 포물선
    - w1 < 0 → 쌍곡선

```rust
pub fn on_make_bezier_conic_arc(
    p0: Point3D,
    t0: Vector3D,
    p2: Point3D,
    t2: Vector3D,
    p: Point3D,
) -> Option<(Point3D, Real)> {
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
            return Some((p1, w1));
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
            let t0u = t0;
            if t0u.length_squared() > 0.0 {
                // keep original scale (do not normalize)
                let v3 = t0u * scale;
                let p1_as_vec = Point3D::new(v3.x, v3.y, v3.z);
                return Some((p1_as_vec, 0.0));
            } else {
                return Some((Point3D::new(0.0, 0.0, 0.0), 0.0));
            }
        }
    }
    None
}
```

## on_bandec

- 네 개의 함수는 **밴드 행렬(band matrix)** 에 대한 LU 분해 + 전진/후진 대치를  
    완전하게 구현한 고급 수치해석 모듈.
- Numerical Recipes 스타일의 알고리즘을 Rust로 매우 정확하게 재현했고,  
    밴드 구조를 유지하면서 연산량을 최소화하는 최적화까지 잘 되어 있음.


### 📘 Band Matrix LU Decomposition & Solve Module
- (on_bandec / on_banbks + dynamic versions)
- 밴드 행렬은 다음과 같은 구조를 가진다:
    - 하부 밴드 폭: m1
    - 상부 밴드 폭: m2
    - 전체 저장 폭: m1 + m2 + 1
- 즉, 행렬 A는 다음과 같은 형태:
```
      m2
   ┌───────────┐
m1 │  banded A │
   └───────────┘
```

- 이 모듈은 다음을 수행한다:
    - 밴드 행렬 LU 분해 (banded LU decomposition)
    - 전진 대치 (L·y = P·b)
    - 후진 대치 (U·x = y)
    - 밴드 구조를 유지한 상태로 최소 연산량으로 수행

### 1️⃣ on_bandec / on_bandec_dyn
- 밴드 행렬 LU 분해 (Band LU Decomposition)
- 입력
    - a: n × (m1+m2+1) 밴드 행렬 (in-place로 U로 변환됨)
    - al: n × m1, L의 하부 밴드 저장
    - index: pivot index (1-based)
    - d: 행 교환 부호 (+1 또는 -1)
- 출력
    - a → U (상부 삼각 밴드)
    - al → L의 하부 밴드
    - index → pivot 정보
    - d → 행 교환 부호

### ✔ 핵심 알고리즘 요약
- 1) 상부 밴드 정렬(sliding)
    - 밴드 행렬의 첫 m1개 행은 왼쪽에 0이 많기 때문에
    - 밴드 중심을 맞추기 위해 왼쪽으로 슬라이딩한다.
- 2) pivot 선택 (partial pivoting)
- 각 행 i에서:
```rust
pivot = max |a[j][0]|,  j = i..i+m1
```

- 즉, 밴드 내에서만 pivot을 찾는다.
- 3) pivot 행 교환
    - 밴드 폭만큼만 swap.
- 4) L 저장
```rust
al[i][j-i-1] = a[j][0] / a[i][0]
```

- 5) U 업데이트 (forward elimination)
```rust
a[j][k-1] = a[j][k] - r * a[i][k]
```

- 마지막 칸은 0으로 밀어냄.

### 2️⃣ on_banbks / on_banbks_dyn
- 밴드 LU를 이용한 Ax = b 해법
- 입력
    - a: U (banded)
    - al: L의 하부 밴드
    - index: pivot 정보
    - b: n × n_rhs (in-place로 해 x로 변환됨)

- ✔ 전진 대치 (L·y = P·b)
    - pivot index에 따라 b의 행을 swap
    - L의 하부 밴드만 이용해 y 계산
```rust
b[k] -= al[j][k-j-1] * b[j]
```

- ✔ 후진 대치 (U·x = y)
- U는 밴드 구조이므로:
```rust
val -= a[j][k] * b[j+k]
val /= a[j][0]
```

- 여기서 a[j][0]은 U의 대각 원소.

### 📌 전체 데이터 구조 요약

- A (original band matrix)
```
 ┌──────────────────────────────────────────┐
 │ lower band (m1) | diag | upper band (m2) │
 └──────────────────────────────────────────┘
```
- After on_bandec:
    - A → U (upper band)
    - AL → lower band of L
    - INDEX → pivot rows
    - D → sign of row swaps



### 📌 알고리즘 복잡도
- 밴드 폭을 w = m1 + m2 + 1이라 하면:
    - 분해: O(n·w²)
    - 전진/후진 대치: O(n·w)
    - Dense LU의 O(n³)에 비해 매우 빠르다.

### 📌 사용 예시
- 이 모듈은 다음과 같은 문제에서 필수:
    - B‑spline / NURBS smoothing
    - Thin-plate spline
    - PDE discretization (tridiagonal, pentadiagonal)
    - Finite difference / finite element banded systems
    - Cubic spline interpolation (tridiagonal)
    - Large sparse banded systems

### 📌 최종 요약
- 이 네 함수는:
    - 밴드 행렬을 위한 최적화된 LU 분해
    - 밴드 구조를 유지한 전진/후진 대치
    - pivoting 포함
    - dense LU보다 훨씬 빠르고 메모리 효율적
```rust
/// Band matrix LU decomposition
///
/// - a: n x (m1 + m2 + 1) — transformed in-place into U
/// - al: n x m1 — stores the lower band of L
/// - index: length n, pivot indices (stored as 1-based; compatible with original C++ convention)
/// - d: (out) sign of row exchanges (+/-1)
pub fn on_bandec<A: DenseMat, L: DenseMat>(
    a: &mut A,
    m1: usize,
    m2: usize,
    al: &mut L,
    index: &mut [usize],
    d: &mut f64,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1, "al must have at least m1 columns");
    debug_assert_eq!(index.len(), n);

    // 상부로 정렬(슬라이딩) + 왼쪽 0 채우기
    let mut num2 = m1;
    for i in 0..m1 {
        // a[i][0..] ← a[i][(m1-i)..(num1-1)]
        for j in (m1 - i)..num1 {
            let v = a.get(i, j);
            a.set(i, j - num2, v);
        }
        num2 -= 1;
        // 오른쪽 끝쪽을 0으로 채움
        for j in (num1 - num2 - 1)..num1 {
            a.set(i, j, 0.0);
        }
    }

    *d = 1.0;
    let mut num3 = m1;

    for i in 0..n {
        // 피벗 찾기: a[i..min(i+num3-i, n-1)][0] 중 절대값 최대
        let mut val1 = a.get(i, 0);
        let mut imax = i;

        if num3 < n {
            num3 += 1;
        } // 다음 행까지의 밴드 높이 확장

        for j in (i + 1)..num3.min(n) {
            let aj0 = a.get(j, 0);
            if aj0.abs() > val1.abs() {
                val1 = aj0;
                imax = j;
            }
        }

        // 1-based pivot index 저장 (원본 C++과 동일)
        index[i] = imax + 1;

        if val1 == 0.0 {
            // 원본과 동일한 '작은 값' 방어
            a.set(i, 0, 1e-40);
        }

        // 행 교환 (0..num1-1 열까지만)
        if imax != i {
            *d = -*d;
            for j in 0..num1 {
                let tmp = a.get(i, j);
                a.set(i, j, a.get(imax, j));
                a.set(imax, j, tmp);
            }
        }

        // 하부 제거 (forward elimination in band form)
        for j in (i + 1)..num3.min(n) {
            let r = a.get(j, 0) / a.get(i, 0);
            // al[i][j - i - 1] = r;
            al.set(i, j - i - 1, r);

            // a[j][k-1] = a[j][k] - r * a[i][k]
            for k in 1..num1 {
                let new_val = a.get(j, k) - r * a.get(i, k);
                a.set(j, k - 1, new_val);
            }
            // 마지막 칸 0으로
            a.set(j, num1 - 1, 0.0);
        }
    }
}
```
```rust
pub fn on_bandec_dyn(
    a: &mut dyn DenseMat,
    m1: usize,
    m2: usize,
    al: &mut dyn DenseMat,
    index: &mut [usize],
    d: &mut f64,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1, "al must have at least m1 columns");
    debug_assert_eq!(index.len(), n);

    // 상부로 정렬(슬라이딩) + 왼쪽 0 채우기
    let mut num2 = m1;
    for i in 0..m1 {
        // a[i][0..] ← a[i][(m1-i)..(num1-1)]
        for j in (m1 - i)..num1 {
            let v = a.get(i, j);
            a.set(i, j - num2, v);
        }
        num2 -= 1;
        // 오른쪽 끝쪽을 0으로 채움
        for j in (num1 - num2 - 1)..num1 {
            a.set(i, j, 0.0);
        }
    }

    *d = 1.0;
    let mut num3 = m1;

    for i in 0..n {
        // 피벗 찾기: a[i..min(i+num3-i, n-1)][0] 중 절대값 최대
        let mut val1 = a.get(i, 0);
        let mut imax = i;

        if num3 < n {
            num3 += 1;
        } // 다음 행까지의 밴드 높이 확장

        for j in (i + 1)..num3.min(n) {
            let aj0 = a.get(j, 0);
            if aj0.abs() > val1.abs() {
                val1 = aj0;
                imax = j;
            }
        }

        // 1-based pivot index 저장 (원본 C++과 동일)
        index[i] = imax + 1;

        if val1 == 0.0 {
            // 원본과 동일한 '작은 값' 방어
            a.set(i, 0, 1e-40);
        }

        // 행 교환 (0..num1-1 열까지만)
        if imax != i {
            *d = -*d;
            for j in 0..num1 {
                let tmp = a.get(i, j);
                a.set(i, j, a.get(imax, j));
                a.set(imax, j, tmp);
            }
        }

        // 하부 제거 (forward elimination in band form)
        for j in (i + 1)..num3.min(n) {
            let r = a.get(j, 0) / a.get(i, 0);
            // al[i][j - i - 1] = r;
            al.set(i, j - i - 1, r);

            // a[j][k-1] = a[j][k] - r * a[i][k]
            for k in 1..num1 {
                let new_val = a.get(j, k) - r * a.get(i, k);
                a.set(j, k - 1, new_val);
            }
            // 마지막 칸 0으로
            a.set(j, num1 - 1, 0.0);
        }
    }
}
```
```rust
/// Forward/Backward substitution
///
/// - a: Band matrix containing LU decomposition (n x (m1 + m2 + 1)) — result from `bandec`
/// - al: Lower band of L (n x m1) — result from `bandec`
/// - index: 1-based pivot indices obtained from `bandec`
/// - b: n x n_rhs (right-hand side and solution stored in-place)
pub fn on_banbks<A: DenseMat, L: DenseMat, B: DenseMat>(
    a: &A,
    m1: usize,
    m2: usize,
    al: &L,
    index: &[usize],
    b: &mut B,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1);
    debug_assert_eq!(index.len(), n);
    debug_assert_eq!(b.n_rows(), n, "b must have n rows");

    let n_rhs = b.n_cols();

    for col in 0..n_rhs {
        // 전진 대입: L * y = P*b
        let mut num2 = m1;
        for j in 0..n {
            let ip = index[j] - 1; // 1-based → 0-based
            if ip != j {
                let tmp = b.get(j, col);
                b.set(j, col, b.get(ip, col));
                b.set(ip, col, tmp);
            }

            if num2 < n {
                num2 += 1;
            }

            for k in (j + 1)..num2.min(n) {
                let new_val = b.get(k, col) - al.get(j, k - j - 1) * b.get(j, col);
                b.set(k, col, new_val);
            }
        }

        // 후진 대입: U * x = y  (banded back-substitution)
        let mut num4 = 1usize;
        for j in (0..n).rev() {
            let mut val = b.get(j, col);
            for k in 1..num4 {
                // a[j][k]는 U의 상부밴드; b[k+j][col]는 그 위에 해당하는 y/x
                val -= a.get(j, k) * b.get(j + k, col);
            }
            val /= a.get(j, 0);
            b.set(j, col, val);

            if num4 < num1 {
                num4 += 1;
            }
        }
    }
}
```
```rust
pub fn on_banbks_dyn(
    a: &dyn DenseMat,
    m1: usize,
    m2: usize,
    al: &dyn DenseMat,
    index: &[usize],
    b: &mut dyn DenseMat,
) {
    let n = a.n_rows();
    let num1 = m1 + m2 + 1;

    debug_assert_eq!(a.n_cols(), num1, "a must be n x (m1+m2+1)");
    debug_assert_eq!(al.n_rows(), n);
    debug_assert!(al.n_cols() >= m1);
    debug_assert_eq!(index.len(), n);
    debug_assert_eq!(b.n_rows(), n, "b must have n rows");

    let n_rhs = b.n_cols();

    for col in 0..n_rhs {
        // 전진 대입: L * y = P*b
        let mut num2 = m1;
        for j in 0..n {
            let ip = index[j] - 1; // 1-based → 0-based
            if ip != j {
                let tmp = b.get(j, col);
                b.set(j, col, b.get(ip, col));
                b.set(ip, col, tmp);
            }

            if num2 < n {
                num2 += 1;
            }

            for k in (j + 1)..num2.min(n) {
                let new_val = b.get(k, col) - al.get(j, k - j - 1) * b.get(j, col);
                b.set(k, col, new_val);
            }
        }

        // 후진 대입: U * x = y  (banded back-substitution)
        let mut num4 = 1usize;
        for j in (0..n).rev() {
            let mut val = b.get(j, col);
            for k in 1..num4 {
                // a[j][k]는 U의 상부밴드; b[k+j][col]는 그 위에 해당하는 y/x
                val -= a.get(j, k) * b.get(j + k, col);
            }
            val /= a.get(j, 0);
            b.set(j, col, val);

            if num4 < num1 {
                num4 += 1;
            }
        }
    }
}
```
---
## on_tridiag_ql_implicit

- 이 함수는 대칭 삼대각 3×3 행렬에 대한 QL implicit-shift 고유값/고유벡터 알고리즘을 특화 구현

### 📘 함수 개요
```rust
pub fn on_tridiag_ql_implicit(
    d: &mut [f64; 3],
    e: &mut [f64; 3],
    mut v: Option<&mut [[f64; 3]; 3]>,
) -> bool
```

- 입력 행렬 형태:
```math
\left[ \begin{matrix}d_0&e_0&0\\ e_0&d_1&e_1\\ 0&e_1&d_2\end{matrix}\right]
``` 
- d = [d0, d1, d2] → 대각
- e = [e0, e1, _] → 초대각 (e[2]는 dummy, 내부에서 0으로 세팅)
- 출력:
    - d → 고윳값 3개 (정렬은 안 보장)
    - e → 중간 계산용, 의미 없음
    - v = Some(V)이면:
    - V는 3×3
    - V의 k번째 열이 d[k]에 대응하는 고유벡터
- 반환값:
    - true → 30회 이내에 모두 수렴
    - false → 수렴 실패

### 🔧 알고리즘 핵심 흐름
- 고유벡터 요청 시 V를 단위행렬로 초기화
    - e[2] = 0.0으로 마지막 오프대각 제거
    - l = 0..2에 대해:
    - m을 찾음: e[m]가 충분히 작아지는 지점 → 그 블록이 하나의 1×1 또는 2×2 서브문제로 수렴
    - 수렴하면 해당 블록 종료
    - 아니면 implicit shift 계산 후,
- Givens 회전 형태로 QL 스텝 수행
- 각 스텝에서:
    - d와 e 갱신 (삼대각 구조 유지)
    - v가 Some이면, 같은 회전으로 고유벡터 행렬도 갱신
    - 최대 30회 반복

### ✨ 포인트
- 3×3에 특화되어 있어서 일반 QL보다 훨씬 가볍고 빠름
- 대칭 삼대각이기 때문에 고유값은 항상 실수, 알고리즘도 안정적
- v를 Option으로 둔 게 좋다 — 고유값만 필요할 때는 비용 절약
- f64::EPSILON * (|d[m]| + |d[m+1]|) 기준으로 수렴 판단하는 것도 교과서적

```rust
/// 삼대각 대칭 3×3의 QL-implicit. d=대각, e=상부 초대각(e[2] dummy)
/// QL algorithm with implicit shifts for a symmetric tridiagonal 3x3 matrix.
///
/// 입력/출력:
/// - d: 대각 원소 [d0, d1, d2]  →  계산 후 고윳값들(오름차순은 보장하지 않음)
/// - e: 아랫대각 원소 [e0, e1, _] (e[2]는 사용 안함) → 계산 중 덮어씀
/// - v: Some(V) 이면 V는 3×3이고, k번째 열이 d[k]의 고유벡터로 채워짐
///
/// 행렬 형태:
/// [ d[0]  e[0]   0  ]
/// [ e[0]  d[1]  e[1]]
/// [  0    e[1]  d[2]]
pub fn on_tridiag_ql_implicit(
    d: &mut [f64; 3],
    e: &mut [f64; 3],
    mut v: Option<&mut [[f64; 3]; 3]>,
) -> bool {
    // V 초기화(요청 시 단위행렬)
    if let Some(vv) = &mut v {
        for i in 0..3 {
            for j in 0..3 {
                vv[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }

    // 마지막 오프대각은 사용하지 않으므로 0으로
    e[2] = 0.0;

    for l in 0..3 {
        let mut iter = 0;

        'outer: loop {
            // m 찾기: e[m]가 충분히 작아지는 첫 위치 (또는 맨 끝)
            let mut m = l;
            while m < 2 && (e[m].abs() >= f64::EPSILON * (d[m].abs() + d[m + 1].abs())) {
                m += 1;
            }

            // 수렴: 해당 블록 종료
            if m == l {
                break 'outer;
            }

            iter += 1;
            if iter == 30 {
                // 수렴 실패로 간주
                return false;
            }

            // implicit shift 계산
            let g0 = (d[l + 1] - d[l]) / (2.0 * e[l]);
            let mut r = (g0 * g0 + 1.0).sqrt();
            let mut g = d[m] - d[l]
                + e[l]
                    / if g0 >= 0.0 {
                        g0 + r.abs()
                    } else {
                        g0 - r.abs()
                    };

            let mut s = 1.0f64;
            let mut c = 1.0f64;
            let mut p = 0.0f64;

            // i = m-1 down to l
            let mut i = m - 1;
            loop {
                let f = s * e[i];
                let b = c * e[i];
                r = (f * f + g * g).sqrt();
                e[i + 1] = r;

                if r == 0.0 {
                    // 이 경우 원본 구현은 바깥 반복을 다시 시작(continue)한다
                    d[i + 1] -= p;
                    e[m] = 0.0;
                    continue 'outer;
                }

                s = f / r;
                c = g / r;

                let g2 = d[i + 1] - p;
                r = (d[i] - g2) * s + 2.0 * c * b;
                p = s * r;
                d[i + 1] = g2 + p;
                g = c * r - b;

                // 고유벡터 갱신
                if let Some(vv) = &mut v {
                    for k in 0..3 {
                        let f = vv[k][i + 1];
                        vv[k][i + 1] = s * vv[k][i] + c * f;
                        vv[k][i] = c * vv[k][i] - s * f;
                    }
                }

                if i == l {
                    break;
                }
                i -= 1;
            }

            // 한 턴 마감 갱신
            d[l] -= p;
            e[l] = g;
            e[m] = 0.0;
        }
    }

    true
}
```

## on_sym3_eigen

- 이 함수는 대칭 3×3 행렬의 고유값·고유벡터를 매우 효율적으로 구하는 특화 알고리즘.
- 특히 일반적인 3×3 고유분해보다 훨씬 빠르고 안정적인 방식  
    **1회 회전 → 삼대각화 → QL implicit shift**  을 사용하고 있어서,  
    CAD/Geometry 엔진에서 자주 쓰는 정석 패턴.


### 📘 on_sym3_eigen
- 대칭 3×3 행렬의 고유값·고유벡터 계산 (fast tridiagonal reduction + QL)
- 입력 행렬:
```math
M=\left[ \begin{matrix}A&D&F\\ D&B&E\\ F&E&C\end{matrix}\right]
``` 
- 출력:
    - ([d0, d1, d2], V)
    - d0..d2 = 고유값 (정렬은 안 보장)
    - V[:,k] = 고유값 d[k]에 대응하는 고유벡터

### 1️⃣ 1단계 — (1,3) 원소 제거(삼대각화)
- 대칭 3×3을 삼대각으로 만들기 위해 x–z 평면 회전을 수행한다.
```rust
if f != 0.0 {
    let theta = 0.5 * (c - a) / f;
    let t = ...;          // Jacobi-like 회전 파라미터
    cos_phi = 1.0 / sqrt(1+t²)
    sin_phi = t * cos_phi

    aa = a - t*f
    cc = c + t*f
    dd = d - sin_phi*(e + tau*d)
    ee = e + sin_phi*(d - tau*e)
}
```

- 결과:
```math
\left[ \begin{matrix}aa&dd&0\\ dd&bb&ee\\ 0&ee&cc\end{matrix}\right] 
```
- 즉, 대칭 삼대각 행렬이 된다.

### 2️⃣ 2단계 — 삼대각 행렬의 QL implicit shift
```rust
let mut dvals = [aa, bb, cc];
let mut evals = [dd, ee, 0.0];
let mut v = Matrix3x3::eye();

on_tridiag_ql_implicit(&mut dvals, &mut evals, Some(&mut v.0));
```

- dvals = 대각
- evals = 초대각
- v = 고유벡터 누적
- QL 알고리즘은 3×3에서는 매우 빠르게 수렴

- 이 단계가 끝나면:
    - dvals = 고유값
    - v = 삼대각화된 좌표계에서의 고유벡터

### 3️⃣ 3단계 — 원래 좌표계로 고유벡터 회전 복원
- 삼대각화할 때 사용한 x–z 회전을 역으로 적용한다.
```rust
let rot = |col| {
    [
        cos_phi*col[0] + sin_phi*col[2],
        col[1],
        -sin_phi*col[0] + cos_phi*col[2],
    ]
};
```

- 각 고유벡터에 적용:
```rust
let c0 = rot(v.col(0));
let c1 = rot(v.col(1));
let c2 = rot(v.col(2));
```


### 📌 최종 결과
```rust
Some((dvals, [c0, c1, c2]))
```

- dvals: 고유값 3개
- [c0, c1, c2]: 각 고유값에 대응하는 정규직교 고유벡터

### 📌 이 구현의 장점
- 대칭 3×3 전용 최적화
    - 일반적인 3×3 고유분해보다 훨씬 빠름
    - 수치적으로 안정적
- Jacobi-style 회전 + QL implicit shift
    - 고유벡터까지 정확히 복원
    - 분기 없는 깔끔한 구조
    - CAD/Geometry 엔진에서 표준적으로 쓰는 방식

```rust
/// 대칭 3×3: [A D F; D B E; F E C] → (eigenvalues d0<=d1<=d2, eigenvectors V(:,k))
pub fn on_sym3_eigen(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) -> Option<([f64; 3], [[f64; 3]; 3])> {
    // 1단계: (1,3) 제거 회전
    let (mut aa, bb, mut cc, mut dd, mut ee) = (a, b, c, d, e);
    let (mut cos_phi, mut sin_phi) = (1.0, 0.0);
    if f != 0.0 {
        let theta = 0.5 * (c - a) / f;
        let t = if theta.abs() > 1.0 {
            1.0 / (theta.abs() * (1.0 + (1.0 + 1.0 / (theta * theta)).sqrt()))
        } else {
            1.0 / (theta.abs() + (1.0 + theta * theta).sqrt())
        } * if theta < 0.0 { -1.0 } else { 1.0 };
        cos_phi = 1.0 / (1.0 + t * t).sqrt();
        sin_phi = t * cos_phi;

        aa = a - t * f;
        cc = c + t * f;
        let tau = sin_phi / (1.0 + cos_phi);
        dd = d - sin_phi * (e + tau * d);
        ee = e + sin_phi * (d - tau * e);
    }

    // 삼대각의 QL
    let mut dvals = [aa, bb, cc];
    let mut evals = [dd, ee, 0.0];
    let mut v = Matrix3x3::eye();
    if !on_tridiag_ql_implicit(&mut dvals, &mut evals, Some(&mut v.0)) {
        return None;
    }

    // 원 좌표계로 회전 복원 (x-z 회전)
    let rot = |col: [f64; 3]| -> [f64; 3] {
        let x = cos_phi * col[0] + sin_phi * col[2];
        let y = col[1];
        let z = -sin_phi * col[0] + cos_phi * col[2];
        [x, y, z]
    };
    let c0 = rot(v.col(0));
    let c1 = rot(v.col(1));
    let c2 = rot(v.col(2));
    Some((dvals, [c0, c1, c2]))
}
```
---
