## point_in_polygon_composite

### 📘 point_in_polygon_composite(p, loops)
- 구멍(hole)을 포함한 복합 다각형에서 점 포함 여부 판정
### 📌 문제 정의
- 복합 다각형은 다음과 같이 구성된다:
    - loops[0] → 바깥 경계(outer boundary)
    - loops[1..] → 내부 구멍(inner holes)
- 점 p가 복합 다각형 내부에 있으려면:
    - 바깥 경계 내부에 있어야 하고
    - 어떤 구멍 내부에도 포함되면 안 된다
- 즉:
```math
p\in \mathrm{Outer}\quad \mathrm{AND}\quad p\notin \mathrm{any\  Hole}
```
### 📌 알고리즘 설명
- ✔ 1) 루프가 비어 있으면 false
```rust
if loops.is_empty() {
    return false;
}
```
- 다각형이 없으므로 점이 포함될 수 없다.

- ✔ 2) 바깥 경계에 포함되는지 검사
```rust
if !point_in_polygon_simple(p, &loops[0].points) {
    return false;
}
```

- 즉:
```math
p\notin \mathrm{Outer}\quad \Rightarrow \quad \mathrm{false}
```

- ✔ 3) 모든 구멍에 대해 검사
```rust
for i in 1..loops.len() {
    if point_in_polygon_simple(p, &loops[i].points) {
        return false;
    }
}
```

- 구멍 내부에 포함되면:
```math
p\in \mathrm{Hole_{\mathnormal{i}}}\quad \Rightarrow \quad \mathrm{false}
```
- ✔ 4) 모든 조건을 통과하면 true
```
true
```

- 즉:
```math
p\in \mathrm{Outer}\quad \mathrm{AND}\quad p\notin \mathrm{any\  Hole}
```

### 📌 수학적 의미
- 복합 다각형 P는 다음과 같이 정의된다:
```math
P=\mathrm{Outer}\setminus \bigcup _{i=1}^{n-1}\mathrm{Hole_{\mathnormal{i}}}
```
- 점 포함 여부는:
    - 이 함수는 정확히 이 논리를 구현한다.

### 📌 기하학적 의미
- 이 알고리즘은 다음과 같은 복합 도형에서 사용된다:
    - 도넛 모양(annulus)
    - CAD의 복합 면(face with holes)
    - GIS의 다중 경계 폴리곤
    - 메쉬의 복합 셀
즉- , 구멍을 가진 다각형에서 점이 실제 내부인지 판정하는 표준 방식이다.

### 📌 요약
- 이 함수는 다음을 판정한다:
- 즉, 구멍을 포함한 복합 폴리곤에서 점이 내부에 있는지 판정하는 함수이다.
```rust
/// 외곽-홀 복합 다각형 포함 판정(간단판): 첫 루프는 외곽, 그 외는 홀
fn point_in_polygon_composite(p: &Point2D, loops: &[Polygon2D]) -> bool {
    if loops.is_empty() {
        return false;
    }
    if !point_in_polygon_simple(p, &loops[0].points) {
        return false;
    }
    for i in 1..loops.len() {
        if point_in_polygon_simple(p, &loops[i].points) {
            return false;
        }
    }
    true
}
```
# dot, add, sub, scalar mul, length, cross, normalize

- 각 함수는 선형대수학에서 매우 표준적인 연산을 수행하고 있고,  
    서로가 수학적으로 어떻게 연결되는지도 명확.

## 📘 3D 벡터 연산 모음
- (dot, add, sub, scalar mul, length, cross, normalize)
- 이 함수들은 모두 3차원 벡터
```math
\mathbf{v}=(v_x,v_y,v_z)
```
- 를 배열 [f64; 3] 형태로 표현하여 기본적인 선형대수 연산을 수행한다.

### 1️⃣ vec_dot(a, b) — 벡터 내적 (Dot Product)
```math
\mathbf{a}\cdot \mathbf{b}=a_xb_x+a_yb_y+a_zb_z
```
- 기하학적 의미:
    - 두 벡터의 각도와 관련
    - $\mathbf{a}\cdot \mathbf{b}=\| \mathbf{a}\| \| \mathbf{b}\| \cos \theta$
    - 직교 여부 판단 가능

### 2️⃣ vec_sub(a, b) — 벡터 뺄셈
```math
\mathbf{a}-\mathbf{b}=(a_x-b_x,\; a_y-b_y,\; a_z-b_z)
```
- 기하학적 의미:
    - 두 점의 차 → 방향 벡터
    - 변위(displacement) 계산

### 3️⃣ vec_add(a, b) — 벡터 덧셈
```math
\mathbf{a}+\mathbf{b}=(a_x+b_x,\; a_y+b_y,\; a_z+b_z)
```
- 기하학적 의미:
    - 평행이동
    - 힘/속도 등 물리량의 합성

### 4️⃣ vec_mul_s(a, s) — 스칼라 곱 (Scalar Multiplication)
```math
s\mathbf{a}=(sa_x,\; sa_y,\; sa_z)
```
- 기하학적 의미:
    - 벡터의 크기(scale) 조절
    - 방향은 유지, 길이만 변함

### 5️⃣ vec_len2(v) — 벡터 길이의 제곱 (Squared Norm)
```math
\| \mathbf{v}\| ^2=\mathbf{v}\cdot \mathbf{v}
```
- 기하학적 의미:
    - 거리 비교 시 sqrt 없이 빠르게 사용
    - 충돌 감지 등에서 자주 쓰임

### 6️⃣ vec_len(a) — 벡터 길이 (Norm)
```math
\| \mathbf{v}\| =\sqrt{v_x^2+v_y^2+v_z^2}
```
- 기하학적 의미:
    - 벡터의 실제 크기
    - 거리 계산의 기본 요소

### 7️⃣ vec_cross(a, b) — 벡터 외적 (Cross Product)
```math
\mathbf{a}\times \mathbf{b}=\left[ \begin{matrix}a_yb_z-a_zb_y\\ a_zb_x-a_xb_z\\ a_xb_y-a_yb_x\end{matrix}\right]
``` 
- 기하학적 의미:
    - 두 벡터에 수직인 벡터 생성
    - 크기는 평행사변형의 면적
    - 법선 벡터(normal) 계산에 필수

### 8️⃣ vec_normalize(v) — 벡터 정규화 (Normalization)
```math
\hat {\mathbf{v}}=\frac{\mathbf{v}}{\| \mathbf{v}\| }
```
- 단, $\| \mathbf{v}\|$ =0이면:
```math
\hat {\mathbf{v}}=(0,0,0)
```
- 기하학적 의미:
    - 방향만 유지하고 길이를 1로 만듦
    - 법선(normal), 방향 벡터(direction) 계산에 필수
    - 0벡터 처리로 수치 안정성 확보

### 📌 전체 요약
- 이 함수들은 3D 벡터 연산의 기본 집합으로, 다음을 모두 포함한다:
    - 내적
    - 덧셈/뺄셈
    - 스칼라 곱
    - 길이 / 길이 제곱
    - 외적
    - 정규화
- 즉, 3D 기하 알고리즘의 모든 기반이 되는 핵심 벡터 연산 패키지다.


```rust
#[inline]
fn vec_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
```
```rust
#[inline]
fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
```
```rust
#[inline]
fn vec_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
```
```rust
#[inline]
fn vec_mul_s(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
```
```rust
#[inline]
fn vec_len2(v: [f64; 3]) -> f64 {
    vec_dot(v, v)
}
```
```rust
#[inline]
#[allow(unused)]
fn vec_len(a: [f64; 3]) -> f64 {
    vec_dot(a, a).sqrt()
}
```
```rust
#[inline]
fn vec_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
```
```rust
fn vec_normalize(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / l, v[1] / l, v[2] / l]
    }
}
```

## on_are_coplanar

- 이 함수는 네 점 A, B, C, D가 같은 평면(coplanar)에 있는지 판정하는 고전적 3D 기하 알고리즘.
- 특히 **세 점이 일직선(collinear)** 인 특수 상황까지 정확히 처리하고 있어서 CAD/Geometry 엔진에서  
    매우 중요한 안정성 있는 구현.

### 📘 on_are_coplanar(a, b, c, d, tol)
- 네 점이 동일한 평면 위에 있는지 판정하는 함수
### 📌 문제 정의
- 3D 공간의 네 점
```math
A,B,C,D\in \mathbb{R^{\mathnormal{3}}}
```
- 이 주어졌을 때, 이 점들이 같은 평면 위에 있는지(coplanar) 판정한다.

### 1️⃣ 기본 원리: 평면의 법선 벡터(normal vector)
- 세 점 A,B,C가 평면을 정의한다고 가정하면,
- 평면의 법선 벡터는 다음과 같다:
```math
\mathbf{n}=(B-A)\times (C-A)
```
### 2️⃣ 점 D가 평면에 있는 조건
- 점 D가 평면에 있으려면:
```math
\mathbf{n}\cdot (D-A)=0
```
- 즉, 법선 벡터와 D-A의 내적이 0이면 평면 위에 있다.
- 수치 오차를 고려하여:
```math
\frac{|\mathbf{n}\cdot (D-A)|}{\| \mathbf{n}\| }\leq \mathrm{tol}
```
- 이 값은 점 D의 평면으로부터의 거리이다.

### 3️⃣ 특수 케이스: A, B, C가 일직선(collinear)
```math
\mathbf{n}=(B-A)\times (C-A)=\mathbf{0}
```
- 즉, 세 점이 평면을 정의하지 못한다.
- 이 경우, 네 점이 coplanar이려면:
- D도 A–B 직선 위에 있어야 한다.
- 이를 위해:
```math
\mathbf{cr}=(B-A)\times (D-A)
```
```math
\| \mathbf{cr}\| \leq \mathrm{tol}\cdot \| B-A\|
``` 
- 즉, D가 A–B 직선에서 벗어난 정도가 허용 오차 이하여야 한다.

### 📌 코드와 수식의 정확한 대응
- ✔ 법선 계산
```rust
let ab = vec_sub(b, a);
let ac = vec_sub(c, a);
let n = vec_cross(ab, ac);
let len = |n|;
```

- ✔ Case 1: 세 점이 일직선
```rust
if len == 0.0 {
    let ad = d - a;
    let cr = ab × ad;
    cr_len <= tol * |ab|
}
```

- 즉:
```math
\| \, (B-A)\times (D-A)\, \| \leq \mathrm{tol}\cdot \| B-A\| 
```

- ✔ Case 2: 일반적인 coplanar 판정
```rust
let dist = |n · (d - a)| / |n|;
dist <= tol
```

- 즉:
```math
\frac{|\mathbf{n}\cdot (D-A)|}{\| \mathbf{n}\| }\leq \mathrm{tol}
```

### 📌 기하학적 의미
- 이 함수는 다음 상황에서 매우 중요하다:
    - 네 점이 하나의 평면을 이루는지 검사
    - 삼각형 두 개가 같은 평면인지 판정
    - 메쉬의 면(face) 정합성 검사
    - CAD에서 coplanar edge/face 병합
    - 기하적 특수 케이스 처리 (collinear, degenerate)
- 특히 collinear 처리가 들어간 것은 고급 CAD 엔진에서 매우 중요한 안정성 요소다.

### 📌 최종 요약
- 이 함수는 다음 조건을 검사한다:
    - 즉, 네 점이 동일한 평면 위에 있는지 수치적으로 안정하게 판정하는 함수이다.
```rust
fn on_are_coplanar(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3], tol: f64) -> bool {
    let ab = vec_sub(b, a);
    let ac = vec_sub(c, a);
    let n = vec_cross(ab, ac);
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len == 0.0 {
        // 세 점이 일직선이면, D도 그 직선에 있어야 coplanar 취급
        let ad = vec_sub(d, a);
        let cr = vec_cross(ab, ad);
        let cr_len = (cr[0] * cr[0] + cr[1] * cr[1] + cr[2] * cr[2]).sqrt();
        let ab_len = (ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2])
            .sqrt()
            .max(1.0);
        return cr_len <= tol * ab_len;
    }
    let dist = vec_dot(n, vec_sub(d, a)).abs() / len;
    dist <= tol
}
```

## project_uv

- 이 함수는 3차원 점을 2차원 좌표계(u,v)로 투영하는 기본적인 선형대수 연산.
- 특히, 기준점 a를 원점으로 삼고, 두 개의 기저 벡터 $e_1,e_2$ 를 사용해  
    **국소 좌표계(local coordinate system)** 를 만드는 매우 중요한 기하 알고리즘.

### 📘 project_uv(a, e1, e2, p)
- 3D 점을 (a, e₁, e₂)로 정의된 2D 좌표계로 투영
### 📌 목적
- 3차원 점 p를, 기준점 a를 원점으로 하고  
    벡터 e_1,e_2를 기저로 하는 2차원 좌표계로 투영하여
- (u,v) 좌표를 구한다.
- 즉:
```math
u=(p-a)\cdot e_1
```
```math
v=(p-a)\cdot e_2
```

### 📌 수학적 의미
#### 1) 기준점 이동
- 먼저 점 p를 기준점 a로 이동:
```math
\mathbf{ap}=p-a
```
#### 2) 기저 벡터에 대한 내적
```math
u=\mathbf{ap}\cdot e_1
```
```math
v=\mathbf{ap}\cdot e_2
```
- 이는 벡터 $\mathbf{ap}$ 를 기저 $\{ e_1,e_2\}$ 에 **정사영(projection)** 한 값이다.

### 📌 기하학적 의미
- 이 함수는 다음과 같은 상황에서 매우 중요하다:
    - 3D 평면 위의 점을 2D 평면 좌표로 변환
    - 폴리곤을 2D로 펼쳐서(point-in-polygon 등) 계산
    - 텍스처 좌표 생성 (UV mapping)
    - 평면 기반의 보간, 삼각분할, 클리핑
    - CAD에서 국소 좌표계(local frame) 구성
- 즉, 3D → 2D 평면 투영의 가장 기본적인 형태다.

### 📌 코드와 수식의 대응
```rust
let ap = vec_sub(p, a);
Point2D::new(vec_dot(ap, e1), vec_dot(ap, e2))
```

- 정확히 다음을 의미한다:
```math
(u,v)=\left( (p-a)\cdot e_1,\; (p-a)\cdot e_2\right)
``` 

### 📌 요약
- 즉, 3D 점을 기준점 a와 기저 e₁,e₂로 정의된 2D 좌표계로 투영하는 함수이다.

```rust
// a를 원점으로 두고 e1,e2 기저로 투영 (u = (p-a)·e1, v = (p-a)·e2)
fn project_uv(a: [f64; 3], e1: [f64; 3], e2: [f64; 3], p: [f64; 3]) -> Point2D {
    let ap = vec_sub(p, a);
    Point2D::new(vec_dot(ap, e1), vec_dot(ap, e2))
}
```
## check_diagonal_intersections

- 이 함수는 3차원 공간의 네 점 A–B–C–D가 주어졌을 때, 두 대각선 쌍 중  
    하나라도 교차하는지 판정하는 고급 기하 알고리즘.
- 특히 3D → 2D 평면 투영, coplanar 검사, 대각선 교차 검사가 모두 포함된 꽤 정교한 로직.

### 📘 check_diagonal_intersections(a, b, c, d)
- 네 점 A–B–C–D가 있을 때, 두 대각선 쌍 중 하나라도 교차하면 true
### 📌 문제 정의
- 네 점 A,B,C,D가 주어졌을 때 다음 두 선분 쌍 중 하나라도 교차하면 true:
- 대각선 쌍 1:
```math
(A-C)\quad \mathrm{vs}\quad (B-D)
```
- 대각선 쌍 2:
```math
(A-D)\quad \mathrm{vs}\quad (B-C)
```
- 즉, 네 점이 어떤 사변형을 이루고 있을 때 대각선이 서로 교차하는지 판정하는 함수다.

### 1️⃣ Coplanar 검사
- 3D에서 선분 교차를 정확히 판단하려면 네 점이 동일한 평면 위에 있어야 한다.
```rust
if !are_coplanar(a, b, c, d, 1e-12) {
    return false;
}
```

- 즉:
```math
A,B,C,D\mathrm{가\  coplanar가\  아니면\  교차\  불가}
```

### 2️⃣ 평면 기저(orthonormal basis) 구성
- 평면 위의 3D 점을 2D로 투영하기 위해
    - 다음과 같은 기저 벡터를 만든다:
        - e_1: 평면 내 첫 번째 방향
        - e_2: 평면 내 두 번째 방향
        - n: 평면의 법선 벡터
- 수식:
```math
n=\frac{(B-A)\times (C-A)}{\| (B-A)\times (C-A)\| }
```
```math
e_1=\frac{B-A}{\| B-A\| }\quad \mathrm{(degenerate\  시\  C-A\  사용)}
```
```math
e_2=\frac{n\times e_1}{\| n\times e_1\| }
```

- 이렇게 하면 $\{ e_1,e_2\}$ 는 평면 위의 직교 기저가 된다.

### 3️⃣ 3D → 2D 투영
- 각 점 P를 다음과 같이 투영한다:
```math
u=(P-A)\cdot e_1
```
```math
v=(P-A)\cdot e_2
```
- 즉:
```math
P\mapsto (u,v)
```
- 코드:
```rust
let au = project_uv(a, e1, e2, a);
let bu = project_uv(a, e1, e2, b);
let cu = project_uv(a, e1, e2, c);
let du = project_uv(a, e1, e2, d);
```


### 4️⃣ 2D에서 선분 교차 검사
- 이제 3D 문제는 완전히 2D 문제로 변환되었다.
- 검사해야 할 두 쌍:
- ✔ 쌍 1
```math
(A-C)\quad \mathrm{vs}\quad (B-D)
```
- ✔ 쌍 2
```math
(A-D)\quad \mathrm{vs}\quad (B-C)
```
- 각각에 대해:
```rust
let (ty, _, _) = Segment2D::intersection(...);
if is_hit(ty) { return true; }
```

- 여기서 is_hit은 다음과 같은 교차 타입을 “교차로 인정”한다:
    - Cross (진짜 교차)
    - Touch (접점)
    - EndPointTouch (끝점 접촉)
    - OverlapInSegment (부분 겹침)
    - CollinearEndPointTouch (일직선 + 끝점 접촉)
- 즉, 겹치거나 닿기만 해도 true.

### 📌 기하학적 의미
- 이 함수는 다음과 같은 상황에서 매우 중요하다:
    - 사변형의 대각선 교차 여부 판정
    - 메쉬에서 edge flip 가능 여부 검사
    - 폴리곤 self-intersection 검사
    - CAD에서 planar face의 edge consistency 검사
    - 3D 선분 교차를 안정적으로 판정
- 특히 3D → 2D 투영 후 교차 검사는 수치적으로 가장 안정적인 표준 기법이다.

### 📌 요약
- 이 함수는 다음을 수행한다:
    - 네 점이 coplanar인지 검사
    - 평면 기저(e₁, e₂) 구성
    - 3D 점을 2D로 투영
    - 두 대각선 쌍의 교차 여부 검사
    - 하나라도 교차하면 true
- 즉, 이면 true.

```rust
/// A-B 와 D-C, 그리고 A-D 와 B-C 두 쌍 중 하나라도 교차하면 true
/// A-B-C-D 네 점이 주어졌을 때,
/// (A–C) vs (B–D) 혹은 (A–D) vs (B–C) 중 하나라도 교차하면 true
pub fn check_diagonal_intersections(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> bool {
    if !are_coplanar(a, b, c, d, 1e-12) {
        return false;
    }

    // 평면 기저 구성
    let ab = vec_sub(b, a);
    let ac = vec_sub(c, a);
    let n = vec_normalize(vec_cross(ab, ac));
    let mut e1 = vec_normalize(ab);
    if e1 == [0.0, 0.0, 0.0] {
        e1 = vec_normalize(ac);
    }
    let mut e2 = vec_normalize(vec_cross(n, e1));
    if e2 == [0.0, 0.0, 0.0] {
        let tmp = if n[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        e2 = vec_normalize(vec_cross(n, tmp));
    }

    // (u,v)로 투영
    let au = project_uv(a, e1, e2, a);
    let bu = project_uv(a, e1, e2, b);
    let cu = project_uv(a, e1, e2, c);
    let du = project_uv(a, e1, e2, d);

    // domain_size = 2D AABB 대각선 길이
    let minx = au.x.min(bu.x).min(cu.x).min(du.x);
    let maxx = au.x.max(bu.x).max(cu.x).max(du.x);
    let miny = au.y.min(bu.y).min(cu.y).min(du.y);
    let maxy = au.y.max(bu.y).max(cu.y).max(du.y);
    let domain_size = ((maxx - minx).hypot(maxy - miny)).max(1.0);

    // 교차로 인정할 타입
    let is_hit = |ty: SegmentIntersectionType| {
        matches!(
            ty,
            SegmentIntersectionType::Cross
                | SegmentIntersectionType::Touch
                | SegmentIntersectionType::EndPointTouch
                | SegmentIntersectionType::OverlapInSegment
                | SegmentIntersectionType::CollinearEndPointTouch
        )
    };

    // 쌍1: (A–C) vs (B–D)
    {
        let s1 = Segment2D::new(au, cu);
        let s2 = Segment2D::new(bu, du);
        let (t12, _, _) = Segment2D::intersection(&s1, &s2, domain_size);
        if is_hit(t12) {
            return true;
        }
    }

    // 쌍2: (A–D) vs (B–C)
    {
        let s3 = Segment2D::new(au, du);
        let s4 = Segment2D::new(bu, cu);
        let (t34, _, _) = Segment2D::intersection(&s3, &s4, domain_size);
        if is_hit(t34) {
            return true;
        }
    }

    false
}
```
```rust
pub fn measure_twist(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let len = Vector3D::new(p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]).length();
    let tol = 1e-9; // Utility.TOL9 대응
    let len_tol2 = (len * tol) * (len * tol);

    // b = p0->p1 (fallback: p2->p3)
    let mut b = Vector3D::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);
    if b.length_squared() < len_tol2 {
        b = Vector3D::new(p3[0] - p2[0], p3[1] - p2[1], p3[2] - p2[2]);
    }
    // a = p0->p2 (fallback: p1->p3)
    let mut a = Vector3D::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
    if a.length_squared() < len_tol2 {
        a = Vector3D::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
    }

    // 평면 법선
    let mut n = Vector3D::cross(&a, &b);
    if !n.normalize() {
        return 0.0;
    }
    // 평면 n·X = D
    let d = n.x * p0[0] + n.y * p0[1] + n.z * p0[2];
    (n.x * p3[0] + n.y * p3[1] + n.z * p3[2] - d).abs()
}
```
```rust
pub fn measure_twist(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let len = Vector3D::new(p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]).length();
    let tol = 1e-9; // Utility.TOL9 대응
    let len_tol2 = (len * tol) * (len * tol);

    // b = p0->p1 (fallback: p2->p3)
    let mut b = Vector3D::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);
    if b.length_squared() < len_tol2 {
        b = Vector3D::new(p3[0] - p2[0], p3[1] - p2[1], p3[2] - p2[2]);
    }
    // a = p0->p2 (fallback: p1->p3)
    let mut a = Vector3D::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
    if a.length_squared() < len_tol2 {
        a = Vector3D::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
    }

    // 평면 법선
    let mut n = Vector3D::cross(&a, &b);
    if !n.normalize() {
        return 0.0;
    }
    // 평면 n·X = D
    let d = n.x * p0[0] + n.y * p0[1] + n.z * p0[2];
    (n.x * p3[0] + n.y * p3[1] + n.z * p3[2] - d).abs()
}
```
## point_in_polygon_simple

- 이 함수는 **2D 단일 루프(simple polygon)** 에서 점이 내부에 있는지 판정하는  
    가장 고전적이고 널리 쓰이는 알고리즘인 **짝수-홀수 교차법(Even–Odd Rule, Ray Casting Method)**   
    을 구현한 것.
## 📘 point_in_polygon_simple(p, poly)
- 단일 루프(simple polygon)에서 점 포함 여부를 짝수-홀수 교차법으로 판정
### 📌 알고리즘 개요 (Even–Odd Rule)
- 점 p에서 **오른쪽 방향으로 반직선(ray)** 을 쏘고,
    이 반직선이 다각형의 변과 교차하는 횟수를 센다.
    - 교차 횟수가 홀수 → 점은 내부
    - 교차 횟수가 짝수 → 점은 외부
- 즉:

### 1️⃣ 루프 순회
```rust
for i in 0..n - 1 {
    let a = poly[i];
    let b = poly[i + 1];
```
- 다각형의 모든 변 (a,b)에 대해 검사한다.

### 2️⃣ y-좌표 조건: 반직선이 변을 가로지르는지 검사
```rust
((a.y > p.y) != (b.y > p.y))
```

- 이는 다음을 의미한다:
    - 점 p의 y좌표를 기준으로 변의 양 끝점이 서로 다른 쪽에 있는 경우만 교차 가능
- 즉:
```math
(a_y-p_y)(b_y-p_y)<0
```
### 3️⃣ x-좌표 조건: 실제로 오른쪽 반직선과 교차하는지 검사
```rust
p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y + 1e-30) + a.x
```

- 이 식은 변 (a,b) 위에서 점 p와 같은 y좌표를 갖는 지점의 x좌표를 계산하는 것이다.
- 수식으로 표현하면:
```math
x_{\mathrm{intersect}}=a_x+(b_x-a_x)\frac{p_y-a_y}{b_y-a_y}
```
- 그리고:
```math
p_x<x_{\mathrm{intersect}}
```
- 이면 반직선이 변과 교차한다.
- 1e-30은 0으로 나누기 방지용.

### 4️⃣ 교차 시 inside 상태 토글
```rust
if intersect {
    inside = !inside;
}
```

- 즉:
    - 교차할 때마다 inside 값을 뒤집는다
    - 최종적으로 inside가 true면 내부

### 📌 수학적 의미 요약
- 점 p가 다각형 내부에 있으려면:
### 📌 기하학적 의미
- 이 알고리즘은 다음과 같은 특징을 가진다:
    - 단순 다각형(simple polygon)에 대해 안정적으로 동작
    - 오목/볼록 다각형 모두 처리 가능
    - 매우 빠르고 구현이 간단
    - CAD, GIS, 게임 엔진 등에서 표준적으로 사용

### 📌 최종 요약
- 즉, 점에서 오른쪽으로 쏜 반직선이 다각형 변과 교차하는 횟수가 홀수면 내부, 짝수면 외부이다.
```rust
/// 단일 루프 점 포함 (짝수 교차법)
fn point_in_polygon_simple(p: &Point2D, poly: &[Point2D]) -> bool {
    let mut inside = false;
    let n = poly.len();
    for i in 0..n - 1 {
        let a = poly[i];
        let b = poly[i + 1];
        let intersect = ((a.y > p.y) != (b.y > p.y))
            && (p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y + 1e-30) + a.x);
        if intersect {
            inside = !inside;
        }
    }
    inside
}
```
## classify_patch_polygon

- CAD/NURBS 엔진에서 **패치(patch)** 와 **트림(trim)** 의 관계를 판정하는 핵심 로직으로,  
    패치가 트림 영역에 대해 In / Out / On / Over 중 어디에 속하는지를 결정한다.

### 📘 classify_patch_polygon(patch, trim_polygons)
- 패치 사각형이 트림 다각형(외곽 + 홀)과 어떤 관계인지 판정하는 함수
### 📌 반환값 의미 (PolygonStatus)
    - Out : 패치가 트림 영역 밖
    - In : 패치가 트림 영역 안
    - On : 경계 접촉 또는 부분적으로 걸침
    - Over : 트림 외곽이 패치 내부에 포함됨 (특수 케이스)

### 1️⃣ 빠른 거절(Fast Reject): AABB 검사
- 트림 다각형들의 AABB 전체를 합쳐서:
```math
[t_{\min },t_{\max }]
````
- 패치의 AABB와 겹치지 않으면:
```math
\mathrm{Out}
```
- 즉, 영역이 겹치지 않으면 더 볼 필요도 없음.

### 2️⃣ domain_size 계산
- 패치와 트림 전체의 AABB를 합쳐서 그 대각선 길이를 domain_size로 사용한다.
- 이는 선분 교차 알고리즘의 수치 안정성을 위한 스케일링 값이다.

### 3️⃣ 변-변 교차 검사 (Edge–Edge Intersection)
- 패치의 모든 변과
    - 트림 다각형들의 모든 변을 비교하여 교차 여부를 검사한다.
- ✔ 교차 타입이 Cross이면 즉시 On
```math
\mathrm{Cross}\Rightarrow \mathrm{On}
```
- ✔ Touch / EndPointTouch / Overlap 등은 edge_touch_count 증가
    - 이는 경계 접촉을 의미하며 최종 판정에서 중요한 역할을 한다.

### 4️⃣ 패치 꼭짓점이 트림 복합 영역 내부인지 검사
- 트림 다각형은 다음 구조:
    - trim_polygons[0] → 외곽(outer loop)
    - trim_polygons[1..] → 홀(inner holes)
- 복합 포함 판정:
```math
\mathrm{inside\_ composite}=\# \{ p_i\in \mathrm{CompositeTrim}\}
``` 

### 5️⃣ 외곽이 패치 내부에 있는 경우 → Over
- 패치 꼭짓점이 하나도 트림 내부에 없고:
    - 경계 접촉 없음
    - 트림 외곽의 첫 점이 패치 내부
- 이면:
```math
\mathrm{Over}
```
- 즉, 패치가 트림을 완전히 덮는 경우.

### 6️⃣ 패치가 특정 홀 내부에 완전히 들어간 경우 → On
- 패치 꼭짓점 4개가 동일한 홀 내부에 있으면:
```math
\mathrm{On}
```
- 이는 CAD에서 매우 중요한 보완 로직이다.

### 7️⃣ 패치가 트림 내부에 완전히 포함된 경우
```math
\mathrm{inside\_ composite}=4
```
- 이면:
    - 경계 접촉 있음 → On
    - 경계 접촉 없음 → In
- 즉:
```math
\mathrm{In\  or\  On}
```
### 8️⃣ 중앙점(midpoint) 검사로 최종 보정
- 패치의 대각선 중간점:
```math
m=\frac{p_0+p_2}{2}
```
- 이 점이 트림 외부이고
    edge_touch_count > 0
    inside_composite = 0
- 이면:
```math
\mathrm{Out}
```
- 즉, 겉만 스치고 실제로는 외부인 경우를 걸러낸다.

### 9️⃣ 최종 판정
    - inside_composite ≤ 0 → Out
    - 그 외 → On
- 즉, 부분적으로 걸쳐 있으면 On.

### 📌 전체 알고리즘 요약
- 이 함수는 다음 조건들을 종합하여 패치의 상태를 판정한다:
    - AABB 빠른 거절
    - 변-변 교차 검사
    - 복합 다각형 포함 여부
    - 외곽 포함 여부 (Over)
    - 홀 내부 완전 포함 (On)
    - 패치 전체 포함 (In/On)
    - 중앙점 보정
    - 최종 Out / In / On / Over 결정


```rust
pub fn classify_patch_polygon(patch: &Polygon2D, trim_polygons: &[Polygon2D]) -> PolygonStatus {
    if trim_polygons.is_empty() {
        return PolygonStatus::Out;
    }

    // --- AABB 합집합으로 빠른 거절 ---
    let mut tmin = trim_polygons[0].min;
    let mut tmax = trim_polygons[0].max;
    for tr in &*trim_polygons {
        tmin.x = tmin.x.min(tr.min.x);
        tmin.y = tmin.y.min(tr.min.y);
        tmax.x = tmax.x.max(tr.max.x);
        tmax.y = tmax.y.max(tr.max.y);
    }
    if !PolyRegion2d::overlap_2d(&patch.min, &patch.max, &tmin, &tmax) {
        return PolygonStatus::Out;
    }

    // domain_size: 패치와 트림 전체의 AABB를 합친 대각선 길이
    let umin = Point2D::new(patch.min.x.min(tmin.x), patch.min.y.min(tmin.y));
    let umax = Point2D::new(patch.max.x.max(tmax.x), patch.max.y.max(tmax.y));
    let domain_size = ((umax.x - umin.x).hypot(umax.y - umin.y)).max(1.0);

    // --- 2) 변-변 교차 검사 ---
    let mut edge_touch_count = 0usize;
    for i in 0..(patch.points.len() - 1) {
        let pe = Segment2D::new(patch.points[i], patch.points[i + 1]);
        for tr in trim_polygons {
            for j in 0..(tr.points.len() - 1) {
                let se = Segment2D::new(tr.points[j], tr.points[j + 1]);
                let (itype, _, _) = Segment2D::intersection(&se, &pe, domain_size);
                match itype {
                    SegmentIntersectionType::Cross => return PolygonStatus::On,
                    SegmentIntersectionType::Touch
                    | SegmentIntersectionType::EndPointTouch
                    | SegmentIntersectionType::OverlapInSegment
                    | SegmentIntersectionType::CollinearEndPointTouch => {
                        edge_touch_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    // --- 3) 복합(외곽-홀) 포함 여부로 패치 꼭짓점 카운트 ---
    let inside_composite = patch.points[..(patch.points.len() - 1)]
        .iter()
        .filter(|p| point_in_polygon_composite(p, trim_polygons))
        .count();

    // --- 4) 외곽의 첫 점이 패치 내부면 Over ---
    if inside_composite == 0 {
        if edge_touch_count > 0 {
            return PolygonStatus::On; // ← 경계 접촉은 On
        }
        if point_in_polygon_simple(&trim_polygons[0].points[0], &patch.points) {
            return PolygonStatus::Over; // ← 접촉은 없지만 외곽이 패치에 포함될 때 Over
        }
    }

    // --- 5) (핵심 보완) 어느 '홀' 내부에 패치가 통째로 들어가면 On 처리 ---
    //     패치 꼭짓점 4개가 동일 홀 내부(단순 폴리곤 기준)라면, 트림과 충돌/포함 관계로 본다.
    for i in 1..trim_polygons.len() {
        let in_hole_cnt = patch.points[..(patch.points.len() - 1)]
            .iter()
            .filter(|p| point_in_polygon_simple(p, &trim_polygons[i].points))
            .count();
        if in_hole_cnt == 4 {
            // 원한다면 Over로 바꿔도 됩니다:
            // return PolygonStatus::Over;
            return PolygonStatus::On;
        }
    }

    // --- 6) 최종 판별 ---
    if inside_composite == 4 {
        return if edge_touch_count > 0 {
            PolygonStatus::On
        } else {
            PolygonStatus::In
        };
    }

    // 중앙점으로 한 번 더 거르기(바깥이면 Out)
    let mid = Point2D::new(
        0.5 * (patch.points[0].x + patch.points[2].x),
        0.5 * (patch.points[0].y + patch.points[2].y),
    );
    let mid_out = !point_in_polygon_composite(&mid, trim_polygons);
    if edge_touch_count > 0 && inside_composite == 0 && mid_out {
        return PolygonStatus::Out;
    }

    if inside_composite <= 0 {
        PolygonStatus::Out
    } else {
        PolygonStatus::On
    }
}
```
## project_point_onto_line

- 이 함수는 **3D 점을 주어진 선분(origin → target) 위로 정사영(orthogonal projection)**  
    하는 매우 기본적이면서도 중요한 기하 알고리즘.
- 특히 CAD·Geometry 엔진에서 점-선 거리 계산, foot point 계산, closest point on line 등에  
    쓰이는 표준 공식 그대로 구현.

### 📘 project_point_onto_line(origin, target, point_to_project)
- 3D 점을 선분 origin→target 위로 정사영하여 좌표를 덮어쓰기
### 📌 목적
- 점 P를 선분 $O\rightarrow T$ 가 정의하는 직선(line) 위로 정사영하여  
    그 결과점을 P'로 덮어쓴다.
- 즉:
```math
P'=O+t(T-O)
```
- 여기서:
```math
t=\frac{(P-O)\cdot (T-O)}{\| T-O\| ^2}
```
### 1️⃣ 방향 벡터와 투영 벡터 계산
```rust
dir = target - origin
vec = point_to_project - origin
```

- 수식:
```math
\mathbf{d}=T-O
```
```math
\mathbf{v}=P-O
```

### 2️⃣ t 값 계산 (정사영 스칼라)
```rust
t = dot(dir, vec) / |dir|²
```

- 수식:
```math
t=\frac{\mathbf{d}\cdot \mathbf{v}}{\| \mathbf{d}\| ^2}
```
- 단, $\| \mathbf{d}\| ^2<10^{-15}$ 이면
- 선분이 너무 짧아 방향이 정의되지 않으므로:
```
t=0
```
- 즉, origin을 그대로 사용.

### 3️⃣ 정사영된 점 계산
```rust
point_to_project = origin + t * dir
```

- 수식:
```math
P'=O+t\mathbf{d}
```
### 📌 기하학적 의미
- 이 연산은 다음을 의미한다:
    - 점 P에서 직선 OT로 수직으로 내린 발(foot point)
    - P와 직선 사이의 최단 거리 계산의 핵심
    - 선분 파라미터 t는 다음을 의미:
    - $t<0$: origin 방향으로 연장선
    - $0\leq t\leq 1$: 선분 내부
    - $t>1$: target 방향으로 연장선
- 즉, 직선 위의 가장 가까운 점을 찾는 표준 공식이다.

- 즉,
    - 3D 점을 origin→target 직선 위로 정사영하여 그 좌표로 덮어쓰는 함수이다.

```rust
/// `point_to_project`는 "선분 origin->target" 위로 정사영된 좌표로 덮어씁니다.
pub fn project_point_onto_line(origin: &Point3D, target: &Point3D, point_to_project: &mut Point3D) {
    let dir = Vector3D::new(
        target.x - origin.x,
        target.y - origin.y,
        target.z - origin.z,
    );
    let vec = Vector3D::new(
        point_to_project.x - origin.x,
        point_to_project.y - origin.y,
        point_to_project.z - origin.z,
    );

    let len_sq = dir.length_squared();
    let t = if len_sq < 1.0e-15 {
        0.0
    } else {
        Vector3D::dot(&dir, &vec) / len_sq
    };

    point_to_project.x = origin.x + t * dir.x;
    point_to_project.y = origin.y + t * dir.y;
    point_to_project.z = origin.z + t * dir.z;
}
```
---

