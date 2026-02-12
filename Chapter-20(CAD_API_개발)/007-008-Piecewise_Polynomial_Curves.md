# 📘 Piecewise Polynomial Curves 정리
- (Piegl & Tiller Section 2.1 기반)

## 1. 왜 Piecewise Polynomial인가?
- 단일 고차 다항식으로 긴 곡선을 표현하면:
    - 진동(oscillation)이 커지고
    - 제어가 어렵고
    - 지역 수정(local modification)이 불가능하다
- 그래서 CAD에서는 여러 개의 낮은 차수(polynomial) 곡선을 이어 붙인 형태,  
    즉 piecewise polynomial curve를 사용한다.

## 2. Piecewise Cubic Polynomial Curve
- Figure 2.1은 3개의 구간(segment)으로 구성된 조각별 3차 곡선을 보여준다.
- 각 구간은:
```math
C_k(u)=\sum _{i=0}^3B_{i,3}(u)\, P_{i,k}
```
- 처럼 독립적인 cubic Bézier segment로 구성된다.

## 3. 구간 연결 시 필요한 연속성 조건
두 구간 $C_1(u)$, $C_2(u)$ 가 u=u_1에서 만난다고 하자.
### ✔ C⁰ 연속성 (위치 연속)
```math
C_1(u_1)=C_2(u_1)
```
- 즉, 두 곡선이 “붙어 있어야” 한다.

### ✔ C¹ 연속성 (접선 연속)
```math
C_1'(u_1)=C_2'(u_1)
```
- Bézier 곡선의 도함수는:
```math
C'(u)=3(P_1-P_0)(1-u)^2+6(P_2-P_1)u(1-u)+3(P_3-P_2)u^2
```
- 특히 끝점에서:
- $C'(0)=3(P_1-P_0)$
- $C'(1)=3(P_3-P_2)$
- 따라서 C¹ 연속 조건은:
```math
3(P_1^3-P_1^2)=3(P_2^1-P_2^2)
```
- 이게 바로 책의 식 (2.1)이다.

## 4. 식 (2.1) 정리
- 식 (2.1):
```math
\frac{3}{u_1-u_0}(P_1^3-P_1^2)=\frac{3}{u_2-u_1}(P_2^1-P_2^2)
```
- 정리하면:
```math
P_1^3=\frac{(u_2-u_1)P_1^2+(u_1-u_0)P_2^1}{u_2-u_0}
```
- 즉:
- 두 segment의 내부 control point 하나는
- 양쪽 segment의 접선 조건을 만족하도록 자동으로 결정된다.
- 이게 piecewise Bézier에서 C¹ 연속성을 유지하는 핵심 공식이다.

## 5. 일반적인 Piecewise Polynomial Curve의 표현
- 곡선 전체는 다음과 같이 표현된다:

- 여기서:
- $P_i$: 전체 곡선을 구성하는 control point
- $f_i(u)$: piecewise polynomial basis function  
    (각 구간에서만 nonzero)
- 이 구조가 바로 B-spline basis의 일반 형태다.

## 6. Rational 버전
- 동차 좌표로 확장하면:
- 투영하면 rational curve가 된다.

## 7. Surface로 확장 (Tensor Product)
- 곡면은 두 방향 basis를 곱해서 만든다:

- 동차 버전:
```math
S^w(u,v)=\sum _{i=0}^n\sum _{j=0}^mf_i(u)\, g_j(v)\, P_{i,j}^w
```
- 이 구조는 이후 B-spline surface, NURBS surface의 기본 틀이 된다.

## 8. 핵심 요약
- Piecewise polynomial curve는 여러 개의 낮은 차수 곡선을 이어 붙인 형태
- 구간 연결 시 C⁰, C¹ 연속 조건이 필요
- 식 (2.1)은 C¹ 연속성을 만족시키기 위한 control point 계산식
- 전체 곡선은
- $C(u)=\sum f_i(u)P_i$ - 형태로 표현됨
- 이는 곧 B-spline basis의 일반 형태
- Surface는 tensor product로 확장됨
---

# 📘 Piecewise Polynomial Continuity Test — 수식 기반 문서화
- 이 문서는 제공된 Rust 테스트 코드가 검증하는 이론적 근거를 정리한 것이다.
- 테스트는 다음 두 가지 핵심을 검증한다:
  - Piecewise Bézier 곡선의 C⁰ 연속성
  - Global‑u 기준 C¹ 연속성
    (즉, 파라미터 구간 길이가 다를 때도 접선이 정확히 이어지는지)

## 1. Quadratic Bézier (degree 2)
### 1.1 곡선 정의
- 2차 Bézier 곡선:
```math
C(t) = (1−t)^2 * P0 + 2t(1−t) * P1 + t^2 * P2
```

- 테스트 코드:
```rust
fn bezier2(p0, p1, p2, t)
```


### 1.2 1차 도함수
```math
C'(t) = 2(1−t)(P1−P0) + 2t(P2−P1)
```
- 테스트 코드:
```rust
fn bezier2_dt(...)
```
- 1.3 C¹ 연속 조건 (global‑u 기준)
- 두 구간의 파라미터가:
```
[u0, u1]  (segment 1)
[u1, u2]  (segment 2)
```

- 일 때, global‑u 기준 C¹ 조건은:
```math
C1'(1) / (u1 − u0)  ==  C2'(0) / (u2 − u1)
```

- 즉:
```math
(P2 − P1) / (u1 − u0)  ==  (Q1 − Q0) / (u2 − u1)
```

- 여기서 C⁰ 조건 때문에:
```math
Q0 = P2
```

- 따라서 Q1은 다음과 같이 결정된다:
```math
Q1 = Q0 + (u2 − u1)/(u1 − u0) * (P2 − P1)
```

- 테스트 코드:
```rust
fn enforce_c0_c1_quadratic(...)
```
## 2. Cubic Bézier (degree 3)
### 2.1 곡선 정의
```math
C(t) = (1−t)^3 P0
     + 3t(1−t)^2 P1
     + 3t^2(1−t) P2
     + t^3 P3
```

- 테스트 코드:
```rust
fn bezier3(...)
```


### 2.2 1차 도함수
```rust
C'(t) = 3(1−t)^2 (P1−P0)
      + 6t(1−t) (P2−P1)
      + 3t^2 (P3−P2)
```

- 테스트 코드:
```rust
fn bezier3_dt(...)
```


### 2.3 C¹ 연속 조건 (global‑u 기준)
- 두 구간의 파라미터가:
```
[u0, u1]  (segment 1)
[u1, u2]  (segment 2)
```

- 일 때:
```rust
C1'(1)/(u1 − u0) == C2'(0)/(u2 − u1)
```

- Bézier 끝점 도함수는:
```math
C1'(1) = 3(P3 − P2)
```
```math
C2'(0) = 3(Q1 − Q0)
```

- C⁰ 조건 때문에:
```math
Q0 = P3
```

- 따라서 Q1은:
```math
Q1 = Q0 + (u2 − u1)/(u1 − u0) * (P3 − P2)
```

- 테스트 코드:
```rust
fn enforce_c0_c1_cubic(...)
```

## 3. 테스트가 검증하는 것
### ✔ Test 1: piecewise_quadratic_c0_c1_continuity
- C⁰:
```math
C1(1) == C2(0)
```
- C¹(global‑u):
```math
C1'(1)/(u1−u0) == C2'(0)/(u2−u1)
```

- 또한 연결부 근처에서 점프가 없는지 샘플링으로 확인.

### ✔ Test 2: piecewise_cubic_c0_c1_continuity
- Cubic 버전의 동일한 C⁰, C¹ 조건을 검증.

## ✔ Test 3: piecewise_cubic_breaks_c1_if_not_scaled
- 이 테스트는 스케일링 없이 C¹을 맞추면 왜 틀리는지 보여준다.
- 잘못된 방식:
```math
Q1_{wrong} = Q0 + (P3 − P2)
```

- 즉, 파라미터 span 비율을 고려하지 않음.
- 결과:
```math
C1'(1)/(u1−u0) != C2'(0)/(u2−u1)
```

- 테스트는 이를 의도적으로 실패시키며,  
  식 (2.1) 류의 스케일링이 반드시 필요함을 증명한다.

## 4. 전체 요약
- 이 테스트 스위트는 다음 이론을 검증한다:
- Piecewise Bézier curve continuity:
```
C0:  end of segment 1 == start of segment 2
C1:  (dC1/dt)/(u1−u0) == (dC2/dt)/(u2−u1)
```

- 그리고 이를 만족시키기 위한 control point 조건:
- Quadratic:
```
Q0 = P2
Q1 = Q0 + (u2−u1)/(u1−u0) * (P2−P1)
```
- Cubic:
```
Q0 = P3
Q1 = Q0 + (u2−u1)/(u1−u0) * (P3−P2)
```

- 즉:
- 파라미터 구간 길이가 다르면, C¹ 연속성을 위해 control point를 스케일링해야 한다.
- 이 테스트는 그 사실을 수치적으로 검증한다.
---
### 테스트 코드
```rust
use nurbslib::core::geom::Point2D;
use nurbslib::core::types::{Real};

fn approx_pt(a: Point2D, b: Point2D, eps: Real) -> bool {
    (a - b).length() <= eps
}

/* ---------- Quadratic Bezier (degree 2) ---------- */

fn bezier2(p0: Point2D, p1: Point2D, p2: Point2D, t: Real) -> Point2D {
    // (1-t)^2 p0 + 2t(1-t) p1 + t^2 p2
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * t * u) + p2 * (t * t)
}

fn bezier2_dt(p0: Point2D, p1: Point2D, p2: Point2D, t: Real) -> Point2D {
    // d/dt = 2(1-t)(p1-p0) + 2t(p2-p1)
    let u = 1.0 - t;
    (p1 - p0) * (2.0 * u) + (p2 - p1) * (2.0 * t)
}

// global-u에서 C¹: (dC/dt)/(span) 연속
// quadratic: q0 = p2, q1 = q0 + ((u2-u1)/(u1-u0))*(p2-p1)
fn enforce_c0_c1_quadratic(
    u0: Real,
    u1: Real,
    u2: Real,
    p0: Point2D,
    p1: Point2D,
    p2: Point2D,
    q2: Point2D,
) -> (Point2D, Point2D, Point2D, Point2D, Point2D, Point2D) {
    let q0 = p2;
    let s = (u2 - u1) / (u1 - u0);
    let q1 = q0 + (p2 - p1) * s;
    (p0, p1, p2, q0, q1, q2)
}

/* ---------- Cubic Bezier (degree 3) ---------- */

fn bezier3(p0: Point2D, p1: Point2D, p2: Point2D, p3: Point2D, t: Real) -> Point2D {
    // (1-t)^3 p0 + 3t(1-t)^2 p1 + 3t^2(1-t) p2 + t^3 p3
    let u = 1.0 - t;
    p0 * (u * u * u)
        + p1 * (3.0 * t * u * u)
        + p2 * (3.0 * t * t * u)
        + p3 * (t * t * t)
}

fn bezier3_dt(p0: Point2D, p1: Point2D, p2: Point2D, p3: Point2D, t: Real) -> Point2D {
    // d/dt = 3(1-t)^2(p1-p0) + 6t(1-t)(p2-p1) + 3t^2(p3-p2)
    let u = 1.0 - t;
    (p1 - p0) * (3.0 * u * u) + (p2 - p1) * (6.0 * t * u) + (p3 - p2) * (3.0 * t * t)
}

// global-u에서 C¹: (P3-P2)/(u1-u0) == (Q1-Q0)/(u2-u1)
// => Q0 = P3, Q1 = Q0 + ((u2-u1)/(u1-u0))*(P3-P2)
fn enforce_c0_c1_cubic(
    u0: Real,
    u1: Real,
    u2: Real,
    p0: Point2D,
    p1: Point2D,
    p2: Point2D,
    p3: Point2D,
    q2: Point2D,
    q3: Point2D,
) -> (Point2D, Point2D, Point2D, Point2D, Point2D, Point2D, Point2D, Point2D) {
    let q0 = p3;
    let s = (u2 - u1) / (u1 - u0);
    let q1 = q0 + (p3 - p2) * s;
    (p0, p1, p2, p3, q0, q1, q2, q3)
}
```
```rust
#[test]
fn piecewise_quadratic_c0_c1_continuity() {
    let eps = 1e-12;

    // 파라미터 span을 일부러 다르게: C¹ 스케일 테스트
    let (u0, u1, u2) = (0.0, 2.0, 5.0);

    // seg1 controls
    let p0 = Point2D::new(0.0, 0.0);
    let p1 = Point2D::new(1.0, 2.0);
    let p2 = Point2D::new(3.0, 1.0);

    // seg2: q0,q1은 enforce에서 결정. q2만 자유롭게 둠.
    let q2 = Point2D::new(6.0, 0.0);

    let (_p0, _p1, _p2, q0, q1, q2) = enforce_c0_c1_quadratic(u0, u1, u2, p0, p1, p2, q2);

    // C0: end == start
    let c1_end = bezier2(p0, p1, p2, 1.0);
    let c2_start = bezier2(q0, q1, q2, 0.0);
    assert!(approx_pt(c1_end, c2_start, eps));

    // C1 in global-u
    let d1_du = bezier2_dt(p0, p1, p2, 1.0) * (1.0 / (u1 - u0));
    let d2_du = bezier2_dt(q0, q1, q2, 0.0) * (1.0 / (u2 - u1));
    assert!(approx_pt(d1_du, d2_du, eps));

    // sanity: 연결부 근처 샘플링(점프 방지)
    let tiny = 1e-6;
    let left = bezier2(p0, p1, p2, 1.0 - tiny);
    let right = bezier2(q0, q1, q2, 0.0 + tiny);
    assert!(left.is_valid() && right.is_valid());
    assert!(left.distance(&right) < 1e-3);
}
```
```rust
#[test]
fn piecewise_cubic_c0_c1_continuity() {
    let eps = 1e-12;

    // span이 매우 다르도록: 첫 구간 짧고, 둘째 구간 김
    let (u0, u1, u2) = (10.0, 11.0, 14.0);

    // seg1 cubic
    let p0 = Point2D::new(0.0, 0.0);
    let p1 = Point2D::new(1.0, 3.0);
    let p2 = Point2D::new(3.0, 3.0);
    let p3 = Point2D::new(4.0, 0.0);

    // seg2: q0,q1은 enforce에서 결정. q2,q3는 자유.
    let q2 = Point2D::new(6.0, -2.0);
    let q3 = Point2D::new(8.0, 0.0);

    let (_p0, _p1, _p2, _p3, q0, q1, q2, q3) =
        enforce_c0_c1_cubic(u0, u1, u2, p0, p1, p2, p3, q2, q3);

    // C0
    let c1_end = bezier3(p0, p1, p2, p3, 1.0);
    let c2_start = bezier3(q0, q1, q2, q3, 0.0);
    assert!(approx_pt(c1_end, c2_start, eps));

    // C1 in global-u
    let d1_du = bezier3_dt(p0, p1, p2, p3, 1.0) * (1.0 / (u1 - u0));
    let d2_du = bezier3_dt(q0, q1, q2, q3, 0.0) * (1.0 / (u2 - u1));
    assert!(approx_pt(d1_du, d2_du, eps));
}
```
```rust
#[test]
fn piecewise_cubic_breaks_c1_if_not_scaled() {
    // “구간 span을 무시하면” 왜 식(2.1)류 스케일이 필요한지 보여주는 테스트
    let eps = 1e-9;
    let (u0, u1, u2) = (0.0, 1.0, 4.0); // 두 번째 구간이 훨씬 김

    let p0 = Point2D::new(0.0, 0.0);
    let p1 = Point2D::new(1.0, 0.0);
    let p2 = Point2D::new(2.0, 0.0);
    let p3 = Point2D::new(3.0, 0.0);

    // C0만 맞추고, C1을 "스케일 없이" 맞춘(잘못된) q1
    let q0 = p3;
    let q1_wrong = q0 + (p3 - p2); // span 비율을 안 곱함 (잘못)
    let q2 = Point2D::new(6.0, 0.0);
    let q3 = Point2D::new(9.0, 0.0);

    let d1_du = bezier3_dt(p0, p1, p2, p3, 1.0) * (1.0 / (u1 - u0));
    let d2_du = bezier3_dt(q0, q1_wrong, q2, q3, 0.0) * (1.0 / (u2 - u1));

    // C1이 깨지는 것을 확인(일부러 assert false 형태)
    assert!(!approx_pt(d1_du, d2_du, eps));
}
```
---



