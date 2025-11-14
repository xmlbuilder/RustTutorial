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


