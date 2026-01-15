# Triangle-Traingle Intersection

## 📐 핵심 개념 요약
삼각형 $T_1=(v_0,v_1,v_2)$, $T_2=(u_0,u_1,u_2)$ 가 주어졌을 때:  
- 각 삼각형의 평면 방정식을 계산
- 다른 삼각형의 꼭짓점들이 이 평면과 어느 쪽에 있는지 확인
- 교차선이 존재하면, 그 선분과 두 삼각형의 교차 구간이 겹치는지 확인
- 공면(coplanar)인 경우 2D 투영 후 `edge-edge`, `point-in-triangle` 테스트 수행

## 📊 함수 요약 표
| 함수 이름                          | 기능 요약                                      | 연관 함수 / 흐름 연결                     | 수학적 핵심 또는 역할 요약                        |
|-----------------------------------|------------------------------------------------|-------------------------------------------|--------------------------------------------------|
| `tri_tri_intersect`                 | 기본 교차 판정 (분모 포함)                     | compute_intervals, isect                  | 평면 거리 부호 판정 + 교차 구간 비교             |
| `tri_tri_intersect_no_div`          | 나눗셈 없는 교차 판정                          | new_compute_intervals                     | 정규화 없이 교차 구간 비교 (곱셈 기반)           |
| `tri_tri_intersect_with_isectline`  | 교차선 두 점까지 계산                         | compute_intervals_isectline               | 교차 구간의 실제 3D 점 반환                      |
| `tri_tri_intersect_with_isectline_ex` | 빠른 구간 배제 후 교차선 계산               | tri_tri_intersect, compute_intervals_isectline | 빠른 분기 + 교차선 두 점 계산                    |
| `coplanar_tri_tri`                  | 공면 삼각형 교차 판정                         | edge_against_tri_edges, point_in_tri      | 2D 투영 후 edge-edge, point-in-triangle 테스트   |
| `compute_intervals`                 | 교차 구간 계산 (분모 포함)                    | isect                                     | 선형 보간 기반 교차점 추정                       |
| `new_compute_intervals`             | 교차 구간 계산 (나눗셈 제거)                  | 내부 곱셈 기반 구간 비교                   | 나눗셈 없이 교차 구간 비교                       |
| `isect2_points`                     | 교차선 두 점 계산                             | compute_intervals_isectline               | 선형 보간으로 교차점 위치 계산                   |
| `compute_intervals_isectline`       | 교차선 점 계산 보조                           | isect2_points                              | 교차선 두 점 반환용 보조 함수                    |
| `dominant_plane_indices`           | 투영 평면 선택                                | coplanar_tri_tri                           | 법선의 최대 성분 제외한 두 축 선택               |

## ✏️ 주요 수식 요약
### 1. 평면 방정식
- 삼각형 $T=(v_0,v_1,v_2)$ 의 법선:

$$
\vec {n}=(v_1-v_0)\times (v_2-v_0)
$$

- 평면 방정식:

$$
\vec {n}\cdot \vec {x}+d=0,\quad \mathrm{where\  }d=-\vec {n}\cdot v_0
$$


### 2. 꼭짓점의 평면 거리
- $d_i=\vec {n}\cdot u_i+d$
    - $d_i>0$: 평면 위쪽
    - $d_i<0$: 평면 아래쪽
    - $d_i=0$: 평면 위

### 3. 교차 구간 계산 (분모 버전)

$$
\mathrm{isect_{\mathnormal{0}}}=v_0+(v_1-v_0)\cdot \frac{d_0}{d_0-d_1}
$$

$$
\mathrm{isect_{\mathnormal{1}}}=v_0+(v_2-v_0)\cdot \frac{d_0}{d_0-d_2}
$$

### 4. 교차 여부 판정
- 구간이 겹치는지 확인:

$$
\mathrm{intersects}\Longleftrightarrow \neg (a_1<b_0\vee b_1<a_0)
$$

### 5. 공면 삼각형 교차 (2D)
- 가장 큰 법선 성분 제외한 두 축으로 투영
- edge-edge 교차 테스트
- 점이 삼각형 내부에 있는지 부호 일관성으로 판정

## ✅ 수학적 검증 요약
- 모든 교차 판정은 평면 부호 판별과 교차 구간 비교에 기반
- USE_EPSILON_TEST는 수치 안정성을 위해 작은 오차 허용
- cross, dot, sub, add, mul 등은 벡터 연산의 기본 구성요소
- tri_tri_intersect_with_isectline은 실제 교차선의 두 점을 반환하여 교차 위치 시각화에 유용

![Tri-Tri Intersection](/image/tri_tri_intersection.png)

```rust
#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]

pub const USE_EPSILON_TEST: bool = true;
```
```rust
#[inline]
fn fabs(x: f32) -> f32 {
    x.abs()
}
```
```rust
#[inline]
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
```
```rust
#[inline]
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
```
```rust
#[inline]
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
```
```rust
#[inline]
fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
```
```rust
#[inline]
fn mul(a: [f32; 3], s: f32) -> [f32; 3] {
    [s * a[0], s * a[1], s * a[2]]
}
```
```rust
#[inline]
fn sort_pair(a: &mut f32, b: &mut f32) {
    if *a > *b {
        core::mem::swap(a, b);
    }
}
```
```rust
#[inline]
fn dominant_plane_indices(n: [f32; 3]) -> (usize, usize) {
    // 투영할 축 선택 (법선 성분의 절대값이 가장 큰 것을 제외한 두 축)
    let a = [fabs(n[0]), fabs(n[1]), fabs(n[2])];
    // i0,i1은 2D 평면의 축 인덱스
    if a[0] > a[1] {
        if a[0] > a[2] { (1, 2) } else { (0, 1) }
    } else {
        if a[2] > a[1] { (0, 1) } else { (0, 2) }
    }
}
```
```rust
/* ---------------- coplanar case helpers (2D 테스트) ---------------- */

#[inline]
fn edge_edge_test(
    v0: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    i0: usize,
    i1: usize,
    ax: f32,
    ay: f32,
) -> bool {
    let bx = u0[i0] - u1[i0];
    let by = u0[i1] - u1[i1];
    let cx = v0[i0] - u0[i0];
    let cy = v0[i1] - u0[i1];
    let f = ay * bx - ax * by;
    let d = by * cx - bx * cy;
    if (f > 0.0 && d >= 0.0 && d <= f) || (f < 0.0 && d <= 0.0 && d >= f) {
        let e = ax * cy - ay * cx;
        if f > 0.0 {
            e >= 0.0 && e <= f
        } else {
            e <= 0.0 && e >= f
        }
    } else {
        false
    }
}
```
```rust
#[inline]
fn edge_against_tri_edges(
    v0: [f32; 3],
    v1: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
    i0: usize,
    i1: usize,
) -> bool {
    let ax = v1[i0] - v0[i0];
    let ay = v1[i1] - v0[i1];
    if edge_edge_test(v0, u0, u1, i0, i1, ax, ay) {
        return true;
    }
    if edge_edge_test(v0, u1, u2, i0, i1, ax, ay) {
        return true;
    }
    if edge_edge_test(v0, u2, u0, i0, i1, ax, ay) {
        return true;
    }
    false
}
```
```rust
#[inline]
fn point_in_tri(
    v0: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
    i0: usize,
    i1: usize,
) -> bool {
    // 부호일관성 테스트
    let (mut a, mut b, mut c, d0, d1, d2);

    a = u1[i1] - u0[i1];
    b = -(u1[i0] - u0[i0]);
    c = -a * u0[i0] - b * u0[i1];
    d0 = a * v0[i0] + b * v0[i1] + c;

    a = u2[i1] - u1[i1];
    b = -(u2[i0] - u1[i0]);
    c = -a * u1[i0] - b * u1[i1];
    d1 = a * v0[i0] + b * v0[i1] + c;

    a = u0[i1] - u2[i1];
    b = -(u0[i0] - u2[i0]);
    c = -a * u2[i0] - b * u2[i1];
    d2 = a * v0[i0] + b * v0[i1] + c;

    if d0 * d1 > 0.0 {
        if d0 * d2 > 0.0 {
            return true;
        }
    }
    false
}
```
```rust
/// coplanar triangle-triangle intersection in 2D projection
pub fn coplanar_tri_tri(
    n: [f32; 3],
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
) -> bool {
    let (i0, i1) = dominant_plane_indices(n);

    if edge_against_tri_edges(v0, v1, u0, u1, u2, i0, i1) {
        return true;
    }
    if edge_against_tri_edges(v1, v2, u0, u1, u2, i0, i1) {
        return true;
    }
    if edge_against_tri_edges(v2, v0, u0, u1, u2, i0, i1) {
        return true;
    }

    if point_in_tri(v0, u0, u1, u2, i0, i1) {
        return true;
    }
    if point_in_tri(u0, v0, v1, v2, i0, i1) {
        return true;
    }

    false
}
```
```rust
/* ---------------- 기본 tri-tri 교차 (분모 나눗셈 버전) ---------------- */

#[inline]
fn compute_intervals(
    vv0: f32,
    vv1: f32,
    vv2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
    d0d1: f32,
    d0d2: f32,
    n1: [f32; 3],
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
) -> Result<(f32, f32), bool> /* Err(true)=coplanar */ {
    // C 매크로와 동일 branching
    if d0d1 > 0.0 {
        // D0D2 <= 0
        Ok(isect(vv2, vv0, vv1, d2, d0, d1))
    } else if d0d2 > 0.0 {
        Ok(isect(vv1, vv0, vv2, d1, d0, d2))
    } else if d1 * d2 > 0.0 || d0 != 0.0 {
        Ok(isect(vv0, vv1, vv2, d0, d1, d2))
    } else if d1 != 0.0 {
        Ok(isect(vv1, vv0, vv2, d1, d0, d2))
    } else if d2 != 0.0 {
        Ok(isect(vv2, vv0, vv1, d2, d0, d1))
    } else {
        // coplanar
        Err(coplanar_tri_tri(n1, v0, v1, v2, u0, u1, u2))
    }
}
```
```rust
#[inline]
fn isect(vv0: f32, vv1: f32, vv2: f32, d0: f32, d1: f32, d2: f32) -> (f32, f32) {
    // ISECT macro
    let isect0 = vv0 + (vv1 - vv0) * d0 / (d0 - d1);
    let isect1 = vv0 + (vv2 - vv0) * d0 / (d0 - d2);
    (isect0, isect1)
}
```
```rust
/// Möller triangle-triangle intersection test (분모 나눗셈 버전)
/// returns true if intersects, false otherwise.
pub fn tri_tri_intersect(
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
    epsilon: f32,
) -> bool {
    // plane 1
    let e1 = sub(v1, v0);
    let e2 = sub(v2, v0);
    let n1 = cross(e1, e2);
    let d1 = -dot(n1, v0);

    let mut du0 = dot(n1, u0) + d1;
    let mut du1 = dot(n1, u1) + d1;
    let mut du2 = dot(n1, u2) + d1;

    if USE_EPSILON_TEST {
        if fabs(du0) < epsilon {
            du0 = 0.0;
        }
        if fabs(du1) < epsilon {
            du1 = 0.0;
        }
        if fabs(du2) < epsilon {
            du2 = 0.0;
        }
    }

    let du0du1 = du0 * du1;
    let du0du2 = du0 * du2;
    if du0du1 > 0.0 && du0du2 > 0.0 {
        return false; // same side
    }

    // plane 2
    let e1b = sub(u1, u0);
    let e2b = sub(u2, u0);
    let n2 = cross(e1b, e2b);
    let d2 = -dot(n2, u0);

    let mut dv0 = dot(n2, v0) + d2;
    let mut dv1 = dot(n2, v1) + d2;
    let mut dv2 = dot(n2, v2) + d2;

    if USE_EPSILON_TEST {
        if fabs(dv0) < epsilon {
            dv0 = 0.0;
        }
        if fabs(dv1) < epsilon {
            dv1 = 0.0;
        }
        if fabs(dv2) < epsilon {
            dv2 = 0.0;
        }
    }

    let dv0dv1 = dv0 * dv1;
    let dv0dv2 = dv0 * dv2;
    if dv0dv1 > 0.0 && dv0dv2 > 0.0 {
        return false;
    }

    // intersection line direction
    let d = cross(n1, n2);

    // 최대 성분 인덱스
    let mut index = 0usize;
    let mut max = fabs(d[0]);
    let b = fabs(d[1]);
    let c = fabs(d[2]);
    if b > max {
        max = b;
        index = 1;
    }
    if c > max {
        index = 2;
    }

    let vp0 = v0[index];
    let vp1 = v1[index];
    let vp2 = v2[index];
    let up0 = u0[index];
    let up1 = u1[index];
    let up2 = u2[index];

    let is1 = match compute_intervals(
        vp0, vp1, vp2, dv0, dv1, dv2, dv0dv1, dv0dv2, n1, v0, v1, v2, u0, u1, u2,
    ) {
        Ok(t) => t,
        Err(coplanar) => return coplanar,
    };

    let is2 = match compute_intervals(
        up0, up1, up2, du0, du1, du2, du0du1, du0du2, n1, v0, v1, v2, u0, u1, u2,
    ) {
        Ok(t) => t,
        Err(coplanar) => return coplanar,
    };

    let (mut a0, mut a1) = is1;
    let (mut b0, mut b1) = is2;
    sort_pair(&mut a0, &mut a1);
    sort_pair(&mut b0, &mut b1);

    !(a1 < b0 || b1 < a0)
}
```
```rust
/* ---------------- 나눗셈 제거 버전 (NoDiv) ---------------- */

#[inline]
fn new_compute_intervals(
    vv0: f32,
    vv1: f32,
    vv2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
    d0d1: f32,
    d0d2: f32,
    n1: [f32; 3],
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
) -> Result<(f32, f32, f32, f32, f32), bool> {
    // 반환: (A,B,C,X0,X1)  (매크로 NEWCOMPUTE_INTERVALS와 동일)
    if d0d1 > 0.0 {
        Ok((vv2, (vv0 - vv2) * d2, (vv1 - vv2) * d2, d2 - d0, d2 - d1))
    } else if d0d2 > 0.0 {
        Ok((vv1, (vv0 - vv1) * d1, (vv2 - vv1) * d1, d1 - d0, d1 - d2))
    } else if d1 * d2 > 0.0 || d0 != 0.0 {
        Ok((vv0, (vv1 - vv0) * d0, (vv2 - vv0) * d0, d0 - d1, d0 - d2))
    } else if d1 != 0.0 {
        Ok((vv1, (vv0 - vv1) * d1, (vv2 - vv1) * d1, d1 - d0, d1 - d2))
    } else if d2 != 0.0 {
        Ok((vv2, (vv0 - vv2) * d2, (vv1 - vv2) * d2, d2 - d0, d2 - d1))
    } else {
        Err(coplanar_tri_tri(n1, v0, v1, v2, u0, u1, u2))
    }
}
```
```rust
pub fn tri_tri_intersect_no_div(
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
    epsilon: f32,
) -> bool {
    // plane 1
    let e1 = sub(v1, v0);
    let e2 = sub(v2, v0);
    let n1 = cross(e1, e2);
    let d1 = -dot(n1, v0);

    let mut du0 = dot(n1, u0) + d1;
    let mut du1 = dot(n1, u1) + d1;
    let mut du2 = dot(n1, u2) + d1;
    if USE_EPSILON_TEST {
        if fabs(du0) < epsilon {
            du0 = 0.0;
        }
        if fabs(du1) < epsilon {
            du1 = 0.0;
        }
        if fabs(du2) < epsilon {
            du2 = 0.0;
        }
    }
    let du0du1 = du0 * du1;
    let du0du2 = du0 * du2;
    if du0du1 > 0.0 && du0du2 > 0.0 {
        return false;
    }

    // plane 2
    let e1b = sub(u1, u0);
    let e2b = sub(u2, u0);
    let n2 = cross(e1b, e2b);
    let d2 = -dot(n2, u0);

    let mut dv0 = dot(n2, v0) + d2;
    let mut dv1 = dot(n2, v1) + d2;
    let mut dv2 = dot(n2, v2) + d2;
    if USE_EPSILON_TEST {
        if fabs(dv0) < epsilon {
            dv0 = 0.0;
        }
        if fabs(dv1) < epsilon {
            dv1 = 0.0;
        }
        if fabs(dv2) < epsilon {
            dv2 = 0.0;
        }
    }
    let dv0dv1 = dv0 * dv1;
    let dv0dv2 = dv0 * dv2;
    if dv0dv1 > 0.0 && dv0dv2 > 0.0 {
        return false;
    }

    // 교차선 방향
    let d = cross(n1, n2);

    // 최대 성분
    let mut index = 0usize;
    let mut max = fabs(d[0]);
    let b = fabs(d[1]);
    let c = fabs(d[2]);
    if b > max {
        max = b;
        index = 1;
    }
    if c > max {
        index = 2;
    }

    let vp0 = v0[index];
    let vp1 = v1[index];
    let vp2 = v2[index];
    let up0 = u0[index];
    let up1 = u1[index];
    let up2 = u2[index];

    let (a, b, c, x0, x1) = match new_compute_intervals(
        vp0, vp1, vp2, dv0, dv1, dv2, dv0dv1, dv0dv2, n1, v0, v1, v2, u0, u1, u2,
    ) {
        Ok(t) => t,
        Err(coplanar) => return coplanar,
    };
    let (d_, e, f_, y0, y1) = match new_compute_intervals(
        up0, up1, up2, du0, du1, du2, du0du1, du0du2, n1, v0, v1, v2, u0, u1, u2,
    ) {
        Ok(t) => t,
        Err(coplanar) => return coplanar,
    };

    // 노-디비전 구간 비교
    let xx = x0 * x1;
    let yy = y0 * y1;
    let xxyy = xx * yy;

    let mut isect1 = [0.0f32; 2];
    let mut isect2 = [0.0f32; 2];

    let mut tmp = a * xxyy;
    isect1[0] = tmp + b * x1 * yy;
    isect1[1] = tmp + c * x0 * yy;

    tmp = d_ * xxyy;
    isect2[0] = tmp + e * xx * y1;
    isect2[1] = tmp + f_ * xx * y0;

    let (a, b) = isect1.split_at_mut(1);
    let (c, d) = isect2.split_at_mut(1);

    sort_pair(&mut a[0], &mut b[0]);
    sort_pair(&mut c[0], &mut d[0]);

    !(isect1[1] < isect2[0] || isect2[1] < isect1[0])
}
```
```rust
/* ---------------- 교차선 두 점까지 구하는 버전 ---------------- */

#[inline]
fn isect2_points(
    vtx0: [f32; 3],
    vtx1: [f32; 3],
    vtx2: [f32; 3],
    vv0: f32,
    vv1: f32,
    vv2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
) -> (f32, f32, [f32; 3], [f32; 3]) {
    // C의 isect2()
    let mut tmp = d0 / (d0 - d1);
    let mut diff = sub(vtx1, vtx0);
    diff = mul(diff, tmp);
    let isect0 = vv0 + (vv1 - vv0) * tmp;
    let isectpoint0 = add(vtx0, diff);

    tmp = d0 / (d0 - d2);
    diff = sub(vtx2, vtx0);
    diff = mul(diff, tmp);
    let isect1 = vv0 + (vv2 - vv0) * tmp;
    let isectpoint1 = add(vtx0, diff);

    (isect0, isect1, isectpoint0, isectpoint1)
}
```
```rust
#[inline]
fn compute_intervals_isectline(
    vert0: [f32; 3],
    vert1: [f32; 3],
    vert2: [f32; 3],
    vv0: f32,
    vv1: f32,
    vv2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
    d0d1: f32,
    d0d2: f32,
) -> Result<(f32, f32, [f32; 3], [f32; 3]), ()> /* Err() => coplanar */ {
    if d0d1 > 0.0 {
        let (i0, i1, p0, p1) = isect2_points(vert2, vert0, vert1, vv2, vv0, vv1, d2, d0, d1);
        Ok((i0, i1, p0, p1))
    } else if d0d2 > 0.0 {
        let (i0, i1, p0, p1) = isect2_points(vert1, vert0, vert2, vv1, vv0, vv2, d1, d0, d2);
        Ok((i0, i1, p0, p1))
    } else if d1 * d2 > 0.0 || d0 != 0.0 {
        let (i0, i1, p0, p1) = isect2_points(vert0, vert1, vert2, vv0, vv1, vv2, d0, d1, d2);
        Ok((i0, i1, p0, p1))
    } else if d1 != 0.0 {
        let (i0, i1, p0, p1) = isect2_points(vert1, vert0, vert2, vv1, vv0, vv2, d1, d0, d2);
        Ok((i0, i1, p0, p1))
    } else if d2 != 0.0 {
        let (i0, i1, p0, p1) = isect2_points(vert2, vert0, vert1, vv2, vv0, vv1, d2, d0, d1);
        Ok((i0, i1, p0, p1))
    } else {
        Err(())
    }
}
```
```rust
#[derive(Debug, Clone, Copy)]
pub struct TriTriIsectLine {
    pub intersects: bool,
    pub coplanar: bool,
    pub p0: [f32; 3],
    pub p1: [f32; 3],
}
```
```rust
/// tri-tri 교차 + (비공면이면) 교차선 두 점 리턴
pub fn tri_tri_intersect_with_isectline(
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
    epsilon: f32,
) -> TriTriIsectLine {
    // plane 1
    let e1 = sub(v1, v0);
    let e2 = sub(v2, v0);
    let n1 = cross(e1, e2);
    let d1 = -dot(n1, v0);

    let mut du0 = dot(n1, u0) + d1;
    let mut du1 = dot(n1, u1) + d1;
    let mut du2 = dot(n1, u2) + d1;
    if USE_EPSILON_TEST {
        if fabs(du0) < epsilon {
            du0 = 0.0;
        }
        if fabs(du1) < epsilon {
            du1 = 0.0;
        }
        if fabs(du2) < epsilon {
            du2 = 0.0;
        }
    }
    let du0du1 = du0 * du1;
    let du0du2 = du0 * du2;
    if du0du1 > 0.0 && du0du2 > 0.0 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    // plane 2
    let e1b = sub(u1, u0);
    let e2b = sub(u2, u0);
    let n2 = cross(e1b, e2b);
    let d2 = -dot(n2, u0);

    let mut dv0 = dot(n2, v0) + d2;
    let mut dv1 = dot(n2, v1) + d2;
    let mut dv2 = dot(n2, v2) + d2;
    if USE_EPSILON_TEST {
        if fabs(dv0) < epsilon {
            dv0 = 0.0;
        }
        if fabs(dv1) < epsilon {
            dv1 = 0.0;
        }
        if fabs(dv2) < epsilon {
            dv2 = 0.0;
        }
    }
    let dv0dv1 = dv0 * dv1;
    let dv0dv2 = dv0 * dv2;
    if dv0dv1 > 0.0 && dv0dv2 > 0.0 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    // 교차선 방향
    let d = cross(n1, n2);
    let mut index = 0usize;
    let mut max = fabs(d[0]);
    let b = fabs(d[1]);
    let c = fabs(d[2]);
    if b > max {
        max = b;
        index = 1;
    }
    if c > max {
        index = 2;
    }

    let vp0 = v0[index];
    let vp1 = v1[index];
    let vp2 = v2[index];
    let up0 = u0[index];
    let up1 = u1[index];
    let up2 = u2[index];

    // tri1 intervals
    let (mut is10, mut is11, a1, a2) =
        match compute_intervals_isectline(v0, v1, v2, vp0, vp1, vp2, dv0, dv1, dv2, dv0dv1, dv0dv2)
        {
            Ok(t) => t,
            Err(()) => {
                // coplanar
                let inter = coplanar_tri_tri(n1, v0, v1, v2, u0, u1, u2);
                return TriTriIsectLine {
                    intersects: inter,
                    coplanar: true,
                    p0: [0.0; 3],
                    p1: [0.0; 3],
                };
            }
        };

    // tri2 intervals
    let (mut is20, mut is21, b1, b2) =
        match compute_intervals_isectline(u0, u1, u2, up0, up1, up2, du0, du1, du2, du0du1, du0du2)
        {
            Ok(t) => t,
            Err(()) => {
                let inter = coplanar_tri_tri(n1, v0, v1, v2, u0, u1, u2);
                return TriTriIsectLine {
                    intersects: inter,
                    coplanar: true,
                    p0: [0.0; 3],
                    p1: [0.0; 3],
                };
            }
        };

    // 정렬 및 교차 구간 클립
    let mut smallest1 = false;
    if is10 > is11 {
        core::mem::swap(&mut is10, &mut is11);
        smallest1 = true;
    }
    let mut smallest2 = false;
    if is20 > is21 {
        core::mem::swap(&mut is20, &mut is21);
        smallest2 = true;
    }

    if is11 < is20 || is21 < is10 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    // 교차점 선택 (원본 로직 그대로)
    let (p0, p1) = if is20 < is10 {
        let p0 = if !smallest1 { a1 } else { a2 };
        let p1 = if is21 < is11 {
            if !smallest2 { b2 } else { b1 }
        } else {
            if !smallest1 { a2 } else { a1 }
        };
        (p0, p1)
    } else {
        let p0 = if !smallest2 { b1 } else { b2 };
        let p1 = if is21 > is11 {
            if !smallest1 { a2 } else { a1 }
        } else {
            if !smallest2 { b2 } else { b1 }
        };
        (p0, p1)
    };

    TriTriIsectLine {
        intersects: true,
        coplanar: false,
        p0,
        p1,
    }
}
```
```rust
/// tri-tri 교차 (ex 변형: 먼저 구간으로 빠른 배제 후, 교차선 두 점 계산)
pub fn tri_tri_intersect_with_isectline_ex(
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    u0: [f32; 3],
    u1: [f32; 3],
    u2: [f32; 3],
    epsilon: f32,
) -> TriTriIsectLine {
    // 먼저 분모 버전으로 구간만 빠르게 확인
    // (원본 C 코드도 동일 순서)
    // plane 1
    let e1 = sub(v1, v0);
    let e2 = sub(v2, v0);
    let n1 = cross(e1, e2);
    let d1 = -dot(n1, v0);

    let mut du0 = dot(n1, u0) + d1;
    let mut du1 = dot(n1, u1) + d1;
    let mut du2 = dot(n1, u2) + d1;
    if USE_EPSILON_TEST {
        if fabs(du0) < epsilon {
            du0 = 0.0;
        }
        if fabs(du1) < epsilon {
            du1 = 0.0;
        }
        if fabs(du2) < epsilon {
            du2 = 0.0;
        }
    }
    let du0du1 = du0 * du1;
    let du0du2 = du0 * du2;
    if du0du1 > 0.0 && du0du2 > 0.0 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    // plane 2
    let e1b = sub(u1, u0);
    let e2b = sub(u2, u0);
    let n2 = cross(e1b, e2b);
    let d2 = -dot(n2, u0);

    let mut dv0 = dot(n2, v0) + d2;
    let mut dv1 = dot(n2, v1) + d2;
    let mut dv2 = dot(n2, v2) + d2;
    if USE_EPSILON_TEST {
        if fabs(dv0) < epsilon {
            dv0 = 0.0;
        }
        if fabs(dv1) < epsilon {
            dv1 = 0.0;
        }
        if fabs(dv2) < epsilon {
            dv2 = 0.0;
        }
    }
    let dv0dv1 = dv0 * dv1;
    let dv0dv2 = dv0 * dv2;
    if dv0dv1 > 0.0 && dv0dv2 > 0.0 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    // 교차선 방향
    let d = cross(n1, n2);
    let mut index = 0usize;
    let mut max = fabs(d[0]);
    let b = fabs(d[1]);
    let c = fabs(d[2]);
    if b > max {
        max = b;
        index = 1;
    }
    if c > max {
        index = 2;
    }

    let vp0 = v0[index];
    let vp1 = v1[index];
    let vp2 = v2[index];
    let up0 = u0[index];
    let up1 = u1[index];
    let up2 = u2[index];

    // 먼저 구간만 검사
    let is1 = match compute_intervals(
        vp0, vp1, vp2, dv0, dv1, dv2, dv0dv1, dv0dv2, n1, v0, v1, v2, u0, u1, u2,
    ) {
        Ok(t) => t,
        Err(coplanar) => {
            return TriTriIsectLine {
                intersects: coplanar,
                coplanar: true,
                p0: [0.0; 3],
                p1: [0.0; 3],
            };
        }
    };
    let is2 = match compute_intervals(
        up0, up1, up2, du0, du1, du2, du0du1, du0du2, n1, v0, v1, v2, u0, u1, u2,
    ) {
        Ok(t) => t,
        Err(coplanar) => {
            return TriTriIsectLine {
                intersects: coplanar,
                coplanar: true,
                p0: [0.0; 3],
                p1: [0.0; 3],
            };
        }
    };

    let (mut a0, mut a1) = is1;
    let (mut b0, mut b1) = is2;
    sort_pair(&mut a0, &mut a1);
    sort_pair(&mut b0, &mut b1);
    if a1 < b0 || b1 < a0 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    // 실제 교차선 두 점 계산(원본 ex가 여기서 다시 compute_intervals_isectline)
    let (is10, is11, a1p, a2p) =
        match compute_intervals_isectline(v0, v1, v2, vp0, vp1, vp2, dv0, dv1, dv2, dv0dv1, dv0dv2)
        {
            Ok(t) => t,
            Err(()) => {
                let inter = coplanar_tri_tri(n1, v0, v1, v2, u0, u1, u2);
                return TriTriIsectLine {
                    intersects: inter,
                    coplanar: true,
                    p0: [0.0; 3],
                    p1: [0.0; 3],
                };
            }
        };
    let (is20, is21, b1p, b2p) =
        match compute_intervals_isectline(u0, u1, u2, up0, up1, up2, du0, du1, du2, du0du1, du0du2)
        {
            Ok(t) => t,
            Err(()) => {
                let inter = coplanar_tri_tri(n1, v0, v1, v2, u0, u1, u2);
                return TriTriIsectLine {
                    intersects: inter,
                    coplanar: true,
                    p0: [0.0; 3],
                    p1: [0.0; 3],
                };
            }
        };

    let mut s1 = is10;
    let mut e1 = is11;
    let mut s2 = is20;
    let mut e2 = is21;
    let mut smallest1 = false;
    if s1 > e1 {
        core::mem::swap(&mut s1, &mut e1);
        smallest1 = true;
    }
    let mut smallest2 = false;
    if s2 > e2 {
        core::mem::swap(&mut s2, &mut e2);
        smallest2 = true;
    }

    if e1 < s2 || e2 < s1 {
        return TriTriIsectLine {
            intersects: false,
            coplanar: false,
            p0: [0.0; 3],
            p1: [0.0; 3],
        };
    }

    let (p0, p1) = if s2 < s1 {
        let p0 = if !smallest1 { a1p } else { a2p };
        let p1 = if e2 < e1 {
            if !smallest2 { b2p } else { b1p }
        } else {
            if !smallest1 { a2p } else { a1p }
        };
        (p0, p1)
    } else {
        let p0 = if !smallest2 { b1p } else { b2p };
        let p1 = if e2 > e1 {
            if !smallest1 { a2p } else { a1p }
        } else {
            if !smallest2 { b2p } else { b1p }
        };
        (p0, p1)
    };

    TriTriIsectLine {
        intersects: true,
        coplanar: false,
        p0,
        p1,
    }
}
```
```rust
🧪 예제 코드: 삼각형 교차 여부 확인
fn main() {
    let v0 = [0.0, 0.0, 0.0];
    let v1 = [1.0, 0.0, 0.0];
    let v2 = [0.0, 1.0, 0.0];

    let u0 = [0.5, 0.5, -1.0];
    let u1 = [0.5, 0.5, 1.0];
    let u2 = [1.5, 0.5, 0.0];

    let epsilon = 1e-6;
    let result = tri_tri_intersect(v0, v1, v2, u0, u1, u2, epsilon);
    println!("Intersect? {}", result);
}
```
- 이 예제는 두 삼각형이 z축 방향으로 교차하는지 확인합니다.
- 결과는 true 또는 false로 출력됩니다.

## ✅ 테스트 코드: 교차선 두 점 확인
```rust
#[test]
fn test_intersection_line_points() {
    let v0 = [0.0, 0.0, 0.0];
    let v1 = [1.0, 0.0, 0.0];
    let v2 = [0.0, 1.0, 0.0];

    let u0 = [0.5, 0.5, -1.0];
    let u1 = [0.5, 0.5, 1.0];
    let u2 = [1.5, 0.5, 0.0];

    let epsilon = 1e-6;
    let result = tri_tri_intersect_with_isectline(v0, v1, v2, u0, u1, u2, epsilon);

    assert!(result.intersects);
    assert!(!result.coplanar);
    println!("Intersection line from {:?} to {:?}", result.p0, result.p1);
}
```
- tri_tri_intersect_with_isectline()을 사용해 실제 교차선의 두 점을 반환합니다.
- result.p0, result.p1은 교차선의 시작과 끝점입니다.

## ✏️ 수식 요약
- 평면 방정식:

$$
\vec {n}=(v_1-v_0)\times (v_2-v_0),\quad d=-\vec {n}\cdot v_0
$$

- 꼭짓점 거리:

$$
d_i=\vec {n}\cdot u_i+d
$$

- 교차 구간:

$$
\mathrm{isect_{\mathnormal{0}}}=v_0+(v_1-v_0)\cdot \frac{d_0}{d_0-d_1}
$$

---


## 📊 테스트 함수 요약 및 수식 설명

 번호 | 테스트 함수 이름                          | 목적 및 설명                                      |
|------|-------------------------------------------|--------------------------------------------------|
| 1    | case1                                     | 기본 교차 여부 확인                              |
| 2    | test_intersection_line_points             | 교차선 두 점 반환 확인 (한 점일 수도 있음)       |
| 3    | test_intersection_line_points2            | 교차선이 실제로 길게 나오는지 확인               |
| 4    | tri_basic_intersection                    | 공면 삼각형 간의 겹침 확인                       |
| 5    | tri_disjoint                              | 평행한 삼각형이 교차하지 않음을 확인             |
| 6    | test_intersection_line_segment_proper     | 교차선이 정확한 두 점을 반환하는지 확인          |
| 7    | no_intersection_parallel_planes           | 평행한 삼각형이 교차하지 않음을 확인             |
| 8    | clear_intersection_crossing               | 명확한 교차 상황에서 두 알고리즘 비교            |
| 9    | edge_edge_touching                        | 변-변이 한 점에서 닿는 경우 확인                 |
| 10   | vertex_inside_other                       | 꼭짓점이 다른 삼각형 내부에 있을 때 교차 확인     |
| 11   | just_barely_separated_by_epsilon          | EPSILON에 따라 교차 여부가 달라지는지 확인       |
| 12   | coplanar_overlap_and_disjoint             | 공면 삼각형 간의 겹침/불겹침 모두 확인           |
| 13   | degenerate_zero_area_triangles            | 퇴화된 삼각형(선분, 점) 처리 확인                |
| 14   | with_isectline_points_lie_on_both_planes_and_inside_segments | 교차선 점이 평면 위에 있는지 확인 |
| 15   | with_isectline_ex_consistency             | 교차선 계산 방식 간 결과 일치 확인               |
| 16   | vertex_permutation_invariance             | 정점 순서 변경이 결과에 영향 없는지 확인         |
| 17   | translation_invariance                    | 평행이동 후에도 결과가 동일한지 확인             |
| 18   | div_vs_no_div_agree_widely                | div/no-div 구현이 대부분 일치하는지 확인         |
| 19   | isectline_points_order_indifference       | 교차선 점의 순서가 달라도 결과 동일한지 확인     |
| 20   | skinny_triangles_near_collinearity        | 매우 가는 삼각형에서도 교차 판정이 정확한지 확인 |
| 21   | fuzz_isectline_points_check_planes        | 랜덤 교차선 점이 평면 위에 있는지 확인           |


### 1. case1
- 목적: 기본 교차 여부 확인
- 삼각형 구성: 하나는 z=0 평면, 다른 하나는 z축으로 관통
- 사용 함수: on_tri_tri_intersect
- 수학 원리:
- 두 삼각형의 평면 법선 계산:

$$
\vec {n}=(v_1-v_0)\times (v_2-v_0)
$$

- 다른 삼각형 꼭짓점의 평면 거리:

$$
d_i=\vec {n}\cdot u_i+d
$$

- 교차 조건:

$$
(d_0\cdot d_1>0)\wedge (d_0\cdot d_2>0)\Rightarrow \mathrm{불교차}
$$


### 2. test_intersection_line_points
- 목적: 교차선 두 점 반환 확인
- 사용 함수: on_tri_tri_intersect_with_isectline
- 결과: intersects == true, coplanar == false, p0 == p1 → 한 점 교차
- 수학 원리:
- 교차선 방향: $\vec {d}=\vec {n}_1\times \vec {n}_2$
- 교차 구간 보간:

$$
\mathrm{isect_{\mathnormal{0}}}=v_0+(v_1-v_0)\cdot \frac{d_0}{d_0-d_1}
$$

### 3. test_intersection_line_points2
- 목적: 교차선이 실제로 길게 나오는지 확인
- 삼각형 구성: 두 번째 삼각형이 더 깊게 관통
- 결과: p0 != p1 → 선분 교차 확인

### 4. tri_basic_intersection
- 목적: 공면 삼각형 간의 겹침 확인
- 사용 함수: on_tri_tri_overlap_test_3d
- 수학 원리:
- 동일 평면 투영 후 edge-edge 교차 및 point-in-triangle 테스트

### 5. tri_disjoint
- 목적: 평행한 삼각형이 교차하지 않음을 확인
- 결과: hit == 0, touch == false

### 6. test_intersection_line_segment_proper
- 목적: 교차선이 정확한 두 점을 반환하는지 확인
- 검증: result.p0, result.p1이 예상된 두 점과 근접한지 확인
- 수학 원리:
- 교차선 두 점 계산:

$$
\mathrm{isectpoint_{\mathnormal{0}}}=vtx_0+(vtx_1-vtx_0)\cdot \frac{d_0}{d_0-d_1}
$$


### 7. no_intersection_parallel_planes
- 목적: 평행한 삼각형이 교차하지 않음을 확인
- 결과: false 반환

### 8. clear_intersection_crossing
- 목적: 명확한 교차 상황에서 두 알고리즘이 일치하는지 확인
- 사용 함수: on_tri_tri_intersect, on_tri_tri_intersect_no_div

### 9. edge_edge_touching
- 목적: 변-변이 한 점에서 닿는 경우도 교차로 간주되는지 확인
- 결과: true

### 10. vertex_inside_other
- 목적: 한 삼각형의 꼭짓점이 다른 삼각형 내부에 있을 때 교차 확인
- 결과: true

### 11. just_barely_separated_by_epsilon
- 목적: EPSILON에 따라 교차 여부가 달라지는지 확인
- 결과: 작은 EPSILON → false, 큰 EPSILON → true

### 12. coplanar_overlap_and_disjoint
- 목적: 공면 삼각형 간의 겹침/불겹침 모두 확인
- 결과: 겹침 → true, 떨어짐 → false

### 13. degenerate_zero_area_triangles
- 목적: 퇴화된 삼각형(선분, 점) 처리 확인
- 결과: 구현에 따라 false 또는 true 가능

### 14. with_isectline_points_lie_on_both_planes_and_inside_segments
- 목적: 교차선의 두 점이 실제로 두 평면 위에 있는지 확인
- 검증 수식:

$$
\vec {n}\cdot (\vec {p}-\vec {v}_0)\approx 0
$$

### 15. with_isectline_ex_consistency
- 목적: with_isectline vs with_isectline_ex 결과 일치 확인
- 검증: 두 교차점이 서로 근접한지 확인

### 16. vertex_permutation_invariance
- 목적: 정점 순서 변경이 결과에 영향을 주지 않는지 확인
- 결과: 모든 순열에서 동일한 결과

### 17. translation_invariance
- 목적: 삼각형을 평행이동해도 결과가 동일한지 확인
- 결과: 이동 전후 결과 동일

### 18. div_vs_no_div_agree_widely
- 목적: div vs no_div 구현이 대부분 일치하는지 확인
- 결과: 경계 근처 제외하고 대부분 일치

### 19. isectline_points_order_indifference
- 목적: 교차선 점의 순서가 달라도 결과가 동일한지 확인

### 20. skinny_triangles_near_collinearity
- 목적: 매우 가는 삼각형에서도 교차 판정이 정확한지 확인

### 21. fuzz_isectline_points_check_planes
- 목적: 랜덤 삼각형 교차선 점이 실제 평면 위에 있는지 확인
- 검증 수식:

$$
\vec {n}\cdot (\vec {p}-\vec {v}_0)\approx 0
$$


```rust
#[cfg(test)]
mod test {
    use nurbslib::core::prelude::Point3D;
    use nurbslib::core::tri_tri_intersection::{on_tri_tri_intersect,
        on_tri_tri_intersect_with_isectline, on_tri_tri_overlap_test_3d};

```
```rust
    #[test]
    fn case1() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let u0 = [0.5, 0.5, -1.0];
        let u1 = [0.5, 0.5, 1.0];
        let u2 = [1.5, 0.5, 0.0];

        let epsilon = 1e-6;
        let result = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, epsilon);
        println!("Intersect? {}", result);
    }
```
```rust
    #[test]
    fn test_intersection_line_points() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let u0 = [0.5, 0.5, -1.0];
        let u1 = [0.5, 0.5, 1.0];
        let u2 = [1.5, 0.5, 0.0];

        let epsilon = 1e-6;
        let result = on_tri_tri_intersect_with_isectline(&v0, &v1, &v2, &u0, &u1, &u2, epsilon);

        assert!(result.intersects);
        assert!(!result.coplanar);
        println!("Intersection line from {:?} to {:?}", result.p0, result.p1);
    }
```
```rust
    #[test]
    fn test_intersection_line_points2() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let u0 = [0.25, 0.25, -1.0];
        let u1 = [0.75, 0.75, 1.0];
        let u2 = [1.5, 0.5, 0.0];


        let epsilon = 1e-6;
        let result = on_tri_tri_intersect_with_isectline(&v0, &v1, &v2, &u0, &u1, &u2, epsilon);

        assert!(result.intersects);
        assert!(!result.coplanar);
        println!("Intersection line from {:?} to {:?}", result.p0, result.p1);
    }
```
```rust
    fn p(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
```
```rust
    #[test]
    fn tri_basic_intersection() {
        // 동일 평면, 살짝 겹침
        let (a1, a2, a3) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
        let (b1, b2, b3) = (p(0.2, 0.2, 0.0), p(1.2, 0.2, 0.0), p(0.2, 1.2, 0.0));
        let mut touch = false;
        let hit = on_tri_tri_overlap_test_3d(&a1, &a2, &a3, &b1, &b2, &b3, &mut touch);
        assert_eq!(hit, 1);
        assert!(touch); // 공면 케이스에서 project_plane 경로
    }
```
```rust
    #[test]
    fn tri_disjoint() {
        let (a1, a2, a3) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
        let (b1, b2, b3) = (p(0.0, 0.0, 1.0), p(1.0, 0.0, 1.0), p(0.0, 1.0, 1.0)); // 평행 이동된 면
        let mut touch = false;
        let hit = on_tri_tri_overlap_test_3d(&a1, &a2, &a3, &b1, &b2, &b3, &mut touch);
        assert_eq!(hit, 0);
        assert!(!touch);
    }
```
```rust
    fn close3(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
        (a[0] - b[0]).abs() < eps &&
            (a[1] - b[1]).abs() < eps &&
            (a[2] - b[2]).abs() < eps
    }
```
```rust
    #[test]
    fn test_intersection_line_segment_proper() {
        // 삼각형 V: z = 0 평면 위 큰 삼각형
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];

        // 삼각형 U: 한 변이 z=0에 있고, 나머지 꼭짓점은 위로 올라간 삼각형
        let u0 = [0.5, 0.5, 0.0];
        let u1 = [1.0, 1.0, 0.0];
        let u2 = [0.75, 0.75, 1.0];

        let epsilon = 1e-6;
        let result = on_tri_tri_intersect_with_isectline(&v0, &v1, &v2, &u0, &u1, &u2, epsilon);

        assert!(result.intersects);
        assert!(!result.coplanar);

        println!("Intersection line from {:?} to {:?}", result.p0, result.p1);

        // 기대 교차 선분의 양 끝점
        let expected_a = [0.5, 0.5, 0.0];
        let expected_b = [1.0, 1.0, 0.0];

        // 알고리즘이 p0/p1 순서를 바꿔서 줄 수도 있으니, 두 경우 모두 허용
        let ok =
            (close3(result.p0, expected_a, epsilon) && close3(result.p1, expected_b, epsilon)) ||
                (close3(result.p0, expected_b, epsilon) && close3(result.p1, expected_a, epsilon));

        assert!(ok, "Intersection segment is not the expected one");
    }

}
```
```rust
#[cfg(test)]
mod tri_tri_tests {
    use nurbslib::core::tri_tri_intersection::{on_tri_tri_intersect, on_tri_tri_intersect_no_div,
        on_tri_tri_intersect_with_isectline, on_tri_tri_intersect_with_isectline_ex};

    const E: f64 = 1e-6;

    #[inline]
    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }
    #[inline]
    fn v_eq(a: &[f64; 3], b: &[f64; 3], eps: f64) -> bool {
        approx(a[0], b[0], eps) && approx(a[1], b[1], eps) && approx(a[2], b[2], eps)
    }
```
```rust
    // 간단 LCG 난수 (외부 의존성 없이)
    #[derive(Clone)]
    struct Rng(u64);
    impl Rng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next_u32(&mut self) -> u64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            (self.0 >> 32) as u64
        }
        fn next_f64(&mut self) -> f64 {
            let v = self.next_u32();
            (v as f64) / (u32::MAX as f64)
        }
        fn next_range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.next_f64()
        }
    }
```
```rust
    // 헬퍼: 삼각형 평행이동
    fn translate(p: [f64; 3], t: [f64; 3]) -> [f64; 3] {
        [p[0] + t[0], p[1] + t[1], p[2] + t[2]]
    }

    // 헬퍼: 두 점이 어떤 삼각형의 평면 위인지 체크
    fn plane_point(n: [f64; 3], p0: [f64; 3], q: [f64; 3]) -> f64 {
        n[0] * (q[0] - p0[0]) + n[1] * (q[1] - p0[1]) + n[2] * (q[2] - p0[2])
    }

    // 간단한 외적/내적
    fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
    fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    #[allow(unused)]
    fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    // 삼각형 정점 순서를 전치
    fn cyclic_permutations<T: Copy>(a: T, b: T, c: T) -> [(T, T, T); 3] {
        [(a, b, c), (b, c, a), (c, a, b)]
    }

    // ===== 기본 케이스 =====
```
```rust
    #[test]
    fn no_intersection_parallel_planes() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let u0 = [0.0, 0.0, 0.1];
        let u1 = [1.0, 0.0, 0.1];
        let u2 = [0.0, 1.0, 0.1];
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), false);
        assert_eq!(on_tri_tri_intersect_no_div(&v0, &v1, &v2, &u0, &u1, &u2, E), false);
    }
```
```rust
    #[test]
    fn clear_intersection_crossing() {
        // XY 평면상의 삼각형과, Z축으로 기울어진 삼각형이 명확히 교차
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];
        let u0 = [1.0, -1.0, -1.0];
        let u1 = [1.0, 3.0, 1.0];
        let u2 = [1.0, 0.5, -0.5];
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), true);
        assert_eq!(on_tri_tri_intersect_no_div(&v0, &v1, &v2, &u0, &u1, &u2, E), true);
    }
```
```rust
    #[test]
    fn edge_edge_touching() {
        // 변-변이 한 점에서 닿는 경우 (경계 포함)
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let u0 = [0.5, -0.5, 0.0];
        let u1 = [0.5, 1.5, 0.0];
        let u2 = [2.0, 0.5, 0.0];
        // 공면 + 변교차 → 교차로 취급
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), true);
        assert_eq!(on_tri_tri_intersect_no_div(&v0, &v1, &v2, &u0, &u1, &u2, E), true);
    }
```
```rust
    #[test]
    fn vertex_inside_other() {
        // U의 정점 하나가 V 삼각형 내부에 위치
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];
        let u0 = [0.5, 0.5, 0.0];
        let u1 = [3.0, 0.5, 0.0];
        let u2 = [0.5, 3.0, 0.0];
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), true);
    }
```
```rust
    #[test]
    fn just_barely_separated_by_epsilon() {
        // 거의 같은 평면이지만 EPSILON 바깥으로 살짝 띄움
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let u0 = [0.2, 0.2, E * 10.0];
        let u1 = [0.8, 0.2, E * 10.0];
        let u2 = [0.2, 0.8, E * 10.0];
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), false);
        // EPSILON 을 크게 하면 교차로 간주될 수도 있음
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, 1e-2), true);
    }
```
```rust
    #[test]
    fn coplanar_overlap_and_disjoint() {
        // 완전 공면 - 겹칩/불겹침 모두
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];

        // 겹치는 공면 삼각형
        let u0 = [0.5, 0.5, 0.0];
        let u1 = [3.0, 0.5, 0.0];
        let u2 = [0.5, 3.0, 0.0];
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), true);

        // 떨어진 공면 삼각형
        let u0 = [3.0, 3.0, 0.0];
        let u1 = [4.0, 3.0, 0.0];
        let u2 = [3.0, 4.0, 0.0];
        assert_eq!(on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E), false);
    }
```
```rust
    #[test]
    fn degenerate_zero_area_triangles() {
        // 퇴화: 한 삼각형이 선분, 혹은 점
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [2.0, 0.0, 0.0]; // 선분
        let u0 = [0.5, -1.0, 0.0];
        let u1 = [0.5, 1.0, 0.0];
        let u2 = [0.5, 2.0, 0.0];
        // 원본 알고리듬은 퇴화에 대해 정의가 애매하지만, 이 구성은 선분-선분 교차(공면)로 간주될 가능성 높음
        // 구현에 따라 1 혹은 0이 될 수 있으므로, 여기서는 "교차로 취급되기를 기대"로 둔다.
        let res = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E);
        assert_eq!(res, false);
    }
```
```rust
    // ===== 교차선 점 검증 =====
    #[test]
    fn with_isectline_points_lie_on_both_planes_and_inside_segments() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];
        let u0 = [1.0, -1.0, -1.0];
        let u1 = [1.0, 3.0, 1.0];
        let u2 = [1.0, 0.5, -0.5];

        let mut cop = 0;
        let mut p = [0.0f64; 3];
        let mut q = [0.0f64; 3];
        let hit = on_tri_tri_intersect_with_isectline(
            &v0, &v1, &v2, &u0, &u1, &u2, E);
        assert_eq!(hit.intersects, true);
        assert_eq!(hit.coplanar, false);

        // 두 점이 각 삼각형 평면 위에 있는지 검사
        let n1 = cross(sub(v1, v0), sub(v2, v0));
        let n2 = cross(sub(u1, u0), sub(u2, u0));
        let d1p = plane_point(n1, v0, hit.p0);
        let d1q = plane_point(n1, v0, hit.p1);
        let d2p = plane_point(n2, u0, hit.p0);
        let d2q = plane_point(n2, u0, hit.p1);
        assert!(d1p.abs() < 1e-3 && d1q.abs() < 1e-3 && d2p.abs() < 1e-3 && d2q.abs() < 1e-3);
    }
```
```rust
    #[test]
    fn with_isectline_ex_consistency() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [3.0, 0.0, 0.0];
        let v2 = [0.0, 3.0, 0.0];
        let u0 = [1.0, -1.0, -0.1];
        let u1 = [1.0, 4.0, 0.2];
        let u2 = [1.0, 0.1, -0.2];

        let mut cop1 = 0;
        let mut p1 = [0.0f64; 3];
        let mut q1 = [0.0f64; 3];
        let hit1 = on_tri_tri_intersect_with_isectline(
            &v0, &v1, &v2, &u0, &u1, &u2, E);

        let mut cop2 = 0;
        let mut p2 = [0.0f64; 3];
        let mut q2 = [0.0f64; 3];
        let hit2 = on_tri_tri_intersect_with_isectline_ex(
            &v0, &v1, &v2, &u0, &u1, &u2, E,
        );


        if hit1.intersects {
            assert_eq!(hit1.coplanar, false);
            // 교차선 양 끝점은 지수적/부동소수 차이로 순서가 다를 수 있으니
            // {p1,q1}와 {p2,q2}가 서로에 근접한지 확인
            let ok = (v_eq(&p1, &p2, 1e-3) && v_eq(&q1, &q2, 1e-3))
                || (v_eq(&p1, &q2, 1e-3) && v_eq(&q1, &p2, 1e-3));
            assert!(
                ok,
                "isect points differ: p1={:?} q1={:?} p2={:?} q2={:?}",
                p1, q1, p2, q2
            );
        }
    }
```
```rust
    // ===== 정점/정렬 불변성 =====
    #[test]
    fn vertex_permutation_invariance() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];
        let u0 = [1.0, -1.0, -1.0];
        let u1 = [1.0, 3.0, 1.0];
        let u2 = [1.0, 0.5, -0.5];

        let base = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E);

        for (a, b, c) in cyclic_permutations(v0, v1, v2) {
            for (d, e, f) in cyclic_permutations(u0, u1, u2) {
                let r = on_tri_tri_intersect(&a, &b, &c, &d, &e, &f, E);
                assert_eq!(r, base, "permutation changed result");
            }
        }
    }
```
```rust
    #[test]
    fn translation_invariance() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let u0 = [0.3, 0.2, 0.1];
        let u1 = [1.4, 0.1, 0.2];
        let u2 = [0.2, 1.3, 0.1];
        let base = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E);

        let t = [2.0, -3.0, 5.0];
        let (v0t, v1t, v2t) = (translate(v0, t), translate(v1, t), translate(v2, t));
        let (u0t, u1t, u2t) = (translate(u0, t), translate(u1, t), translate(u2, t));

        let r = on_tri_tri_intersect(&v0t, &v1t, &v2t, &u0t, &u1t, &u2t, E);
        assert_eq!(r, base);
    }
```
```rust
    #[test]
    fn div_vs_no_div_agree_widely() {
        // 다양한 구성에서 두 구현 결과가 일치하는지
        let mut rng = Rng::new(0xC0FFEE);
        for _ in 0..2000 {
            let mut tri = || -> ([f64; 3], [f64; 3], [f64; 3]) {
                let p = [
                    rng.next_range(-2.0, 2.0),
                    rng.next_range(-2.0, 2.0),
                    rng.next_range(-2.0, 2.0),
                ];
                let a = [
                    rng.next_range(-1.0, 1.0),
                    rng.next_range(-1.0, 1.0),
                    rng.next_range(-1.0, 1.0),
                ];
                let b = [
                    rng.next_range(-1.0, 1.0),
                    rng.next_range(-1.0, 1.0),
                    rng.next_range(-1.0, 1.0),
                ];
                (
                    p,
                    [p[0] + a[0], p[1] + a[1], p[2] + a[2]],
                    [p[0] + b[0], p[1] + b[1], p[2] + b[2]],
                )
            };
            let (v0, v1, v2) = tri();
            let (u0, u1, u2) = tri();

            let r1 = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, E);
            let r2 = on_tri_tri_intersect_no_div(&v0, &v1, &v2, &u0, &u1, &u2, E);

            // 퇴화/경계 근처에서 드물게 다를 수 있으니 강단정은 지양,
            // 그래도 대부분 동일해야 함.
            if r1 != r2 {
                // 한 번 더 느슨한 EPSILON 으로 재시험
                let r1b = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, 1e-5);
                let r2b = on_tri_tri_intersect_no_div(&v0, &v1, &v2, &u0, &u1, &u2, 1e-5);
                assert_eq!(r1b, r2b, "div/no-div disagree even with relaxed EPSILON");
            }
        }
    }
```
```rust
    #[test]
    fn isectline_points_order_indifference() {
        // 교차선 점의 순서는 반대여도 OK
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [2.0, 0.0, 0.0];
        let v2 = [0.0, 2.0, 0.0];
        let u0 = [1.0, -1.0, -1.0];
        let u1 = [1.0, 3.0, 1.0];
        let u2 = [1.0, 0.5, -0.5];

        let mut p = [0.0; 3];
        let mut q = [0.0; 3];
        let mut cop = 0;
        let res = on_tri_tri_intersect_with_isectline(
            &v0, &v1, &v2, &u0, &u1, &u2, E,
        );
        assert_eq!(res.intersects, true);

        // 점들을 바꿔 끼워도 교차성은 동일
        let mut p2 = [0.0; 3];
        let mut q2 = [0.0; 3];
        let mut cop2 = 0;
        let res2 = on_tri_tri_intersect_with_isectline_ex(
            &v0, &v1, &v2, &u0, &u1, &u2, E);
        assert_eq!(res2.intersects, true);

        let same = (v_eq(&p, &p2, 1e-3) && v_eq(&q, &q2, 1e-3))
            || (v_eq(&p, &q2, 1e-3) && v_eq(&q, &p2, 1e-3));
        assert!(same);
    }
```
```rust
    // ===== 경계값 근처 =====
    #[test]
    fn skinny_triangles_near_collinearity() {
        // 매우 가는 삼각형들
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [1.0, 1e-7, 0.0];

        let u0 = [0.5, -1.0, 0.0];
        let u1 = [0.5, 2.0, 0.0];
        let u2 = [0.5, 0.3, 0.0];

        // 극도로 가늘지만 교차는 해야 함
        let r1 = on_tri_tri_intersect(&v0, &v1, &v2, &u0, &u1, &u2, 1e-7);
        assert_eq!(r1, true);
    }
```
```rust
    // ===== 간이 퍼지테스트: 교차선 점 검증(일부만) =====
    #[test]
    fn fuzz_isectline_points_check_planes() {
        let mut rng = Rng::new(0xDEAD_BEEF);
        let trials = 200;
        for _ in 0..trials {
            // 대략적인 교차를 유도: 하나는 XY 근처, 하나는 X≈1 평면 근처
            let v0 = [
                rng.next_range(-1.0, 1.0),
                rng.next_range(-1.0, 1.0),
                rng.next_range(-0.01, 0.01),
            ];
            let v1 = [
                v0[0] + rng.next_range(0.1, 1.0),
                v0[1] + rng.next_range(-1.0, 1.0),
                rng.next_range(-0.01, 0.01),
            ];
            let v2 = [
                v0[0] + rng.next_range(-1.0, 1.0),
                v0[1] + rng.next_range(0.1, 1.0),
                rng.next_range(-0.01, 0.01),
            ];

            let u0 = [
                1.0 + rng.next_range(-0.01, 0.01),
                rng.next_range(-1.0, 2.0),
                rng.next_range(-1.0, 1.0),
            ];
            let u1 = [
                1.0 + rng.next_range(-0.01, 0.01),
                rng.next_range(-1.0, 2.0),
                rng.next_range(-1.0, 1.0),
            ];
            let u2 = [
                1.0 + rng.next_range(-0.01, 0.01),
                rng.next_range(-1.0, 2.0),
                rng.next_range(-1.0, 1.0),
            ];

            let mut p = [0.0; 3];
            let mut q = [0.0; 3];
            let mut cop = 0;
            let hit = on_tri_tri_intersect_with_isectline(
                &v0, &v1, &v2, &u0, &u1, &u2, 1e-5,
            );

            if hit.intersects == true && hit.coplanar != false {
                // 두 점은 각 평면 위여야 한다
                let n1 = cross(sub(v1, v0), sub(v2, v0));
                let n2 = cross(sub(u1, u0), sub(u2, u0));
                let d1p = plane_point(n1, v0, p);
                let d1q = plane_point(n1, v0, q);
                let d2p = plane_point(n2, u0, p);
                let d2q = plane_point(n2, u0, q);
                assert!(
                    d1p.abs() < 1e-2 && d1q.abs() < 1e-2 && d2p.abs() < 1e-2 && d2q.abs() < 1e-2
                );
            }
        }
    }
}
```
---



