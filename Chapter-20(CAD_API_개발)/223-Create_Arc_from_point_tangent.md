# on_make_circular_arc_from_endpoints_tangents
## 1. 함수의 목적
```rust
on_make_circular_arc_from_endpoints_tangents(
    p1: Point3D,  // 시작점
    t1u: Vector3D,// 시작점에서의 단위 접선
    p2: Point3D,  // 끝점
    t2u: Vector3D,// 끝점에서의 단위 접선
    ...
) -> Result<(Vec<Point4D>, usize), NurbsError>
```

### 목표:
- 점 $P_1$, $P_2$ 와
- 각 점에서의 단위 접선 $T_1$, $T_2$ 가 주어졌을 때
- 이 조건을 만족하는 원호(circular arc) 를 만들고
- 그 원호를 표현하는 NURBS control points (Pw, weight 포함) 를 생성한다.
- 경우에 따라:
    - 직선
    - 1개의 원호
    - 2개의 원호(연속된 두 arc) 로 나뉜다.

## 2. 기본 준비: 시작점, 끝점, chord, 방향
```rust
let cw = 0.5 * (2.0_f64).sqrt(); // = sqrt(2)/2

pw.push(on_weight_point3d(p1, 1.0));

let d = (p2 - p1).length();      // chord length
let v_hat = (p2 - p1).unitize(); // chord 방향 단위벡터
```

- $d=\| P_2-P_1\|$ : 두 점 사이 거리 (chord 길이)
- v_hat : chord 방향 단위벡터
- cw = sqrt(2)/2 는 90도(π/2) 원호를 quadratic NURBS로 표현할 때 쓰는 표준 weight 값.

## 3. 접선 방향으로 만든 직선 두 개의 교점
```rust
let l1 = on_make_line_seg(p1, t1u, false);
let l2 = on_make_line_seg(p2, t2u, false);

let inter = on_intersect_lines_least_squares(l1, l2, l_tol)?;
let its = inter.ok;
let r = inter.midpoint;
let t1 = inter.t1;
let t2 = inter.t2;
```

- $L_1$: 점 $P_1$, 방향 $T_1$ 를 지나는 직선
- $L_2$ : 점 $P_2$, 방향 $T_2$ 를 지나는 직선
- 이 두 직선의 최소제곱 교점을 구한다:
    - its == true → 두 직선이 “사실상” 교차한다고 볼 수 있음
    - r → 두 직선의 최근접점의 중점 (실질적인 교점 역할)
    - t1, t2 → 각각 $L_1$, $L_2$ 상에서의 파라미터
- 수식으로 쓰면:
```math
L_1(s)=P_1+sT_1,\quad L_2(t)=P_2+tT_2
```
- r는 $L_1(t_1)$ 와 $L_2(t_2)$ 의 중간점.

## 4. 교점이 없을 때: 특수 케이스 처리
```rust
if !its {
    let dot1 = t1u.dot_pt(&v_hat);
    let dot2 = t2u.dot_pt(&v_hat);
```
- 여기서 dot1 = $T_1$ · v_hat, dot2 = $T_2$ · v_hat.
### 4-1. 두 접선이 chord 방향과 나란한 경우 (collinear)
```rust
if (dot1 - 1.0).abs() < p_tol && (dot2 - 1.0).abs() < p_tol {
    let c = on_combination_point3d(0.5, p1, 0.5, p2);
    pw.push(on_weight_point3d(c, 1.0));
    pw.push(on_weight_point3d(p2, 1.0));
    ...
}
```

- 조건:
    - $T_1\approx \hat {v}$
    - $T_2\approx \hat {v}$
- 즉, 두 접선이 chord 방향과 같은 방향이고, 사실상 직선 구간이다.
- 이때:
    - 중간점 $C=\frac{1}{2}P_1+\frac{1}{2}P_2$
    - control points: $[P_1,C,P_2]$, weight = 1 → 그냥 직선 segment.

### 4-2. 두 접선이 chord에 수직인 경우 (perpendicular)
```rust
if dot1.abs() < p_tol && dot2.abs() < p_tol {
    let c = (p1 + p2)/2;
    let dot = t1u.dot(&t2u);
```

- 조건:
    - $T_1\perp \hat {v}$
    - $T_2\perp \hat {v}$
- 즉, chord에 수직인 접선들.
- 여기서 다시 두 경우로 나뉜다.
#### (a) 접선이 서로 반대 방향 (anti-parallel)
```rust
if dot < 0.0 {
    // single 180-degree arc
    let rr = 0.5 * d;
    let p1u = t1u.to_point();

    let a1 = p1 + rr * p1u;
    let a2 = c + rr * p1u;
    let a3 = p2 + rr * p1u;

    // weights: [1, cw, 1, cw, 1] 형태
}
```

- $T_1$ 과 $T_2$ 가 서로 반대 방향 → 180도(π) 원호 하나.
- 반지름 r=d/2
- $P_1$ 에서 접선 방향으로 r만큼 이동한 점,
- 중간점 C에서 같은 방향으로 r,
- $P_2$ 에서도 같은 방향으로 r 이동한 점들을 control point로 사용.
- weight:
    - 양 끝점: 1
    - 중간 control: cw = sqrt(2)/2 → 90도 원호의 표준 weight
- 이 구성으로 하나의 cubic NURBS로 180도 원호를 만든다.
#### (b) 접선이 같은 방향 (parallel)
```rust
else {
    // two arcs (split at midpoint)
    let rr = 0.25 * d;
    ...
}
```

- $T_1$ 과 $T_2$ 가 같은 방향 → 90도 + 90도 같은 식으로 두 개의 원호로 나누어 표현.
- 반지름 r=d/4 를 사용해서
- $P_1$ – 중간점 – C 구간
- C – 중간점 – $P_2$ 구간 을 각각 하나의 원호로 만든다.
- 각 구간마다:
    - 끝점 weight = 1
    - 중간 control weight = cw = sqrt(2)/2

### 4-3. 그 외: 평행/반평행인데 위 특수 케이스가 아닌 경우
```rust
return Err(NurbsError::InvalidArgument { ... });
```

- 즉, 기하적으로 일관된 원호를 만들 수 없는 경우.

## 5. 교점이 있을 때: 일반적인 원호 구성
```rust
if its {
    if t1 * t2 > 0.0 { ... 에러 ... }
    if t1.abs() < p_tol || t2.abs() < p_tol { ... 에러 ... }
```

- 조건:
    - t_1t_2<0: 교점이 두 점 사이에 위치해야 한다는 의미.
    - |t_1|,|t_2| 가 너무 작으면 기하적으로 불안정 → 에러.
### 5-1. 원의 중심과 관련된 값 계산
```rust
let al = 1.0 / t1.abs();
let ar = 1.0 / t2.abs();
let mut w = 0.25 * d * (al + ar);
```

- 여기서 w는 원호의 기하와 관련된 weight 계수로 쓰인다.
- 두 접선이 만드는 원의 중심 위치와 원호의 크기를 반영한다.

### 5-2. t1 > 0: 한 개의 원호
```rust
if t1 > 0.0 {
    pw.push(on_weight_point3d(r, w));
    pw.push(on_weight_point3d(p2, 1.0));
    ...
}
```

- 이 경우는 $P_1\rightarrow P_2$ 를 잇는 단일 원호로 표현 가능.
- control points:
    - $P_1$ (이미 push됨, weight=1)
    - R (교점 근처, weight = w)
    - $P_2$ (weight=1)
- 이 구조는 quadratic NURBS 원호 표현과 유사한 형태를 cubic 구조에 맞게 확장한 것.

### 5-3. t1 < 0: 두 개의 원호로 분할
```rust
else {
    w = -w;
    let bet = w / (1.0 + w);
    let omb = 1.0 - bet;

    let a = omb*P1 + bet*R;
    let b = bet*R + omb*P2;
    let c = 0.5*a + 0.5*b;

    let mut ww = 0.5 * (1.0 + w);
    ww = ww.sqrt();

    // P1 (1)
    // A (ww)
    // C (1)
    // B (ww)
    // P2 (1)
}
```

- 여기서:
    - w를 부호 반전
    - $\omega =1-\beta$
- 점들:
```math
A=\omega P_1+\beta R \quad B=\beta R+\omega P_2 \quad C=\frac{1}{2}A+\frac{1}{2}B
```
- A,B는 $P_1$, $P_2$ 와 교점 R 사이를 적절히 분할한 점들
- C는 그 중간점
- weight:
```math
w_w=\sqrt{\frac{1+w}{2}}
```
- $P_1$, C, $P_2$ : weight = 1
- A, B: weight = $w_w$
- 이 구성은 두 개의 원호를 C¹로 매끄럽게 이어붙인 구조다.

## 6. 전체적으로 보면
- 이 함수는:
    - 기하 조건 (두 점 + 두 접선)으로부터
    - 가능한 경우:
    - 직선
    - 1개의 원호
    - 2개의 원호 를 판별하고
    - 각 경우에 맞는 NURBS control points + weights 를 계산해서
    - 결과로 (Vec<Point4D>, n) 을 반환한다.
- Point4D = homogeneous 좌표 (xw, yw, z*w, w)
- n = 마지막 control point index

## 7. 이 함수가 커널에서 갖는 의미
- 이 함수는 사실상:
- **두 점과 두 접선으로 정의된 원호 생성기**
    - CAD에서 자주 쓰이는:
    - fillet
    - blend
    - circular arc fitting
    - tangent arc construction 같은 기능의 핵심 primitive다.

---

## 소스 코드
```rust
pub fn on_make_circular_arc_from_endpoints_tangents(
    p1: Point3D,
    t1u: Vector3D, // unit tangent
    p2: Point3D,
    t2u: Vector3D, // unit tangent
    p_tol: Real,   // use P_TOL_DEFAULT if you don't have global P_TOL
    l_tol: Real,   // use your global L_TOL equivalent for line intersection
) -> Result<(Vec<Point4D>, usize), NurbsError> {
    // C: cw = 0.5*sqrt(2)
    let cw = 0.5 * (2.0_f64).sqrt();

    // Pw[0] = weight(P1,1)
    let mut pw: Vec<Point4D> = Vec::with_capacity(9);
    pw.push(on_weight_point3d(p1, 1.0));

    // V = normalize(P2-P1), d = chord length
    let d = (p2 - p1).length();
    let v_hat = (p2 - p1).unitize();


    // Make unbounded lines at endpoints along tangents
    let l1 = on_make_line_seg(p1, t1u, false);
    let l2 = on_make_line_seg(p2, t2u, false);

    let inter = on_intersect_lines_least_squares(l1, l2, l_tol)?;
    let its = inter.ok;
    let r = inter.midpoint;
    let t1 = inter.t1;
    let t2 = inter.t2;

    if !its {
        let dot1 = t1u.dot_pt(&v_hat);
        let dot2 = t2u.dot_pt(&v_hat);

        // Collinear tangents (both point along chord)
        if (dot1 - 1.0).abs() < p_tol && (dot2 - 1.0).abs() < p_tol {
            let c = on_combination_point3d(0.5, p1, 0.5, p2);
            pw.push(on_weight_point3d(c, 1.0));
            pw.push(on_weight_point3d(p2, 1.0));
            let n = pw.len() - 1;
            return Ok((pw, n));
        }

        // Tangents are perpendicular to chord
        if dot1.abs() < p_tol && dot2.abs() < p_tol {
            let c = on_combination_point3d(0.5, p1, 0.5, p2);
            let dot = t1u.dot(&t2u);

            if dot < 0.0 {
                // anti-parallel tangents: single 180-degree arc -> 1 cubic (4 ctrl) with weights cw
                let rr = 0.5 * d;

                let p1u = t1u.to_point();

                let a1 = p1 + rr * p1u;
                pw.push(on_weight_point3d(a1, cw));

                let a2 = c + rr * p1u;
                pw.push(on_weight_point3d(a2, 1.0));

                let a3 = p2 + rr * p1u;
                pw.push(on_weight_point3d(a3, cw));

                pw.push(on_weight_point3d(p2, 1.0));
                let n = pw.len() - 1;
                return Ok((pw, n));
            } else {
                // parallel tangents: two arcs (split at midpoint)
                let rr = 0.25 * d;

                let p1u = t1u.to_point();
                // first half
                let b = on_combination_point3d(0.5, p1, 0.5, c);
                let a1 = p1 + rr * p1u;
                pw.push(on_weight_point3d(a1, cw));

                let a2 = b + rr * p1u;
                pw.push(on_weight_point3d(a2, 1.0));

                let a3 = c + rr * p1u;
                pw.push(on_weight_point3d(a3, cw));

                pw.push(on_weight_point3d(c, 1.0));


                let p2u = t2u.to_point();

                // second half
                let b2 = on_combination_point3d(0.5, c, 0.5, p2);
                let a4 = c - rr * p2u;
                pw.push(on_weight_point3d(a4, cw));

                let a5 = b2 -rr * p2u;
                pw.push(on_weight_point3d(a5, 1.0));

                let a6 = p2 -rr * p2u;
                pw.push(on_weight_point3d(a6, cw));

                pw.push(on_weight_point3d(p2, 1.0));

                let n = pw.len() - 1;
                return Ok((pw, n));
            }
        }

        // Anti-collinear or parallel tangents
        return Err(NurbsError::InvalidArgument {
            msg: "on_make_circular_arc_from_endpoints_tangents: tangents are parallel/anti-collinear but not a handled special case".into(),
        });
    }

    // its == TRUE
    if t1 * t2 > 0.0 {
        return Err(NurbsError::InvalidInput {
            msg: "on_make_circular_arc_from_endpoints_tangents: intersection parameters have same sign (t1*t2>0)".into(),
        });
    }
    if t1.abs() < p_tol || t2.abs() < p_tol {
        return Err(NurbsError::InvalidInput {
            msg: "on_make_circular_arc_from_endpoints_tangents: intersection parameter too small".into(),
        });
    }

    // Compute the circle
    let al = 1.0 / t1.abs();
    let ar = 1.0 / t2.abs();
    let mut w = 0.25 * d * (al + ar);

    if t1 > 0.0 {
        // One arc: P1 (already), R(w), P2
        pw.push(on_weight_point3d(r, w));
        pw.push(on_weight_point3d(p2, 1.0));
        let n = pw.len() - 1;
        Ok((pw, n))
    } else {
        // Two arcs
        w = -w;
        let bet = w / (1.0 + w);
        let omb = 1.0 - bet;

        let a = on_combination_point3d(omb, p1, bet, r);
        let b = on_combination_point3d(bet, r, omb, p2);
        let c = on_combination_point3d(0.5, a, 0.5, b);

        let mut ww = 0.5 * (1.0 + w);
        if ww < 0.0 {
            return Err(NurbsError::InvalidInput {
                msg: "on_make_circular_arc_from_endpoints_tangents: negative weight encountered".into(),
            });
        }
        ww = ww.sqrt();

        // P1 already
        pw.push(on_weight_point3d(a, ww));
        pw.push(on_weight_point3d(c, 1.0));
        pw.push(on_weight_point3d(b, ww));
        pw.push(on_weight_point3d(p2, 1.0));

        let n = pw.len() - 1;
        Ok((pw, n))
    }
}
```
```rust
pub fn exact_cubic(
    ps: &Point3D,
    ts: &Vector3D,
    pe: &Point3D,
    te: &Vector3D,
) -> Result<Vec<BezierCurve>, NurbsError> {
    let (pw, n_hi) = on_make_circular_arc_from_endpoints_tangents(
        *ps, *ts, *pe, *te,
        ON_TOL9, ON_TOL9
    )?;

    if pw.len() != n_hi + 1 {
        return Err(NurbsError::InvalidState {
            msg: format!("arc builder returned pw.len()={} but n_hi={} (expected len=n_hi+1)", pw.len(), n_hi),
        });
    }

    let seg2 = Self::split_quadratic_rational_segments(&pw)?;
    let mut out = Vec::with_capacity(seg2.len());

    for s in seg2 {
        let c3 = Self::elevate_rational_bezier_deg2_to_deg3(&s);
        out.push(BezierCurve {
            dim: 3,
            degree: 3,
            ctrl: c3.to_vec(),
        });
    }

    Ok(out)
}
```
```rust
fn split_quadratic_rational_segments(pw: &[Point4D]) 
    -> Result<Vec<[Point4D; 3]>, NurbsError> {
    match pw.len() {
        3 => Ok(vec![[pw[0], pw[1], pw[2]]]),
        5 => Ok(vec![
            [pw[0], pw[1], pw[2]],
            [pw[2], pw[3], pw[4]],
        ]),
        9 => Ok(vec![
            [pw[0], pw[1], pw[2]],
            [pw[2], pw[3], pw[4]],
            [pw[4], pw[5], pw[6]],
            [pw[6], pw[7], pw[8]],
        ]),
        _ => Err(NurbsError::InvalidArgument {
            msg: format!("unexpected Pw length from circular-arc builder: {}", pw.len()),
        }),
    }
}
```
---
