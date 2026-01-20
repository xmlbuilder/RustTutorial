# Global / Local Approximation
- **전역 최소제곱 근사 + 구간 분할(local approximation) + 짧은 구간 fallback**  
    를 모두 갖춘 작은 NURBS fitting 모듈.


## 1. make_line_curve — 두 점을 지나는 1차 NURBS 직선
- 기능
- 입력: 두 점 $A,B\in \mathbb{R^{\mathnormal{3}}}$
- 출력: degree 1, clamped, 두 점을 정확히 지나는 NURBS 곡선
- 수식
- degree
```math
p=1 
```
- control points:
```math
P_0=A,\quad P_1=B
```
- knot vector:
  - U=[0,0,1,1]
- NURBS (비유리, w_i=1):
```math
C(u)=(1-u)A+uB,\quad u\in [0,1]
```
- 즉, 그냥 선분 보간.

## 2. make_quadratic_through_3_points — 세 점을 지나는 2차 Bezier 곡선
- 목표
    - 주어진 세 점 Q_0,Q_1,Q_2 를 정확히 지나는 quadratic Bezier 곡선 C(u) 구성
- Bezier 곡선 수식

- degree 
```math
p=2
```
- control points $P_0$, $P_1$, $P_2$:
```math
C(u)=(1-u)^2P_0+2u(1-u)P_1+u^2P_2,\quad u\in [0,1]
```
- 조건:
```math
C(0)=Q_0,\quad C(0.5)=Q_1,\quad C(1)=Q_2
```
- 이미:
```math
C(0)=P_0=Q_0,\quad C(1)=P_2=Q_2
```
중간점에서:
```math
C(0.5)=\frac{1}{4}P_0+\frac{1}{2}P_1+\frac{1}{4}P_2=Q_1
```
여기서 P_1 에 대해 풀면:
```math
\frac{1}{2}P_1=Q_1-\frac{1}{4}P_0-\frac{1}{4}P_2\Rightarrow P_1=2Q_1-\frac{1}{2}P_0-\frac{1}{2}P_2
```
- 코드:
```rust
let p1 = Point3D::new(
    2.0 * q1.x - 0.5 * p0.x - 0.5 * p2.x,
    2.0 * q1.y - 0.5 * p0.y - 0.5 * p2.y,
    2.0 * q1.z - 0.5 * p0.z - 0.5 * p2.z,
);
```

- knot vector:
```
U=[0,0,0,1,1,1]
```
- 즉, quadratic Bezier NURBS.

## 3. global_approximation — 전역 최소제곱 NURBS 근사
- 목표
- 데이터 점 $Q_i\in \mathbb{R^{\mathnormal{3}}},\  i=0..m$
- degree p, control point 개수 $n+1=\mathrm{ctrl\\_count}$
- 조건:
    - $C(u_0)=Q_0$
    - $C(u_m)=Q_m$
    - 내부 control point $P_1..P_{n-1}$ 를 최소제곱으로 결정
### 3.1. 파라미터 할당
```rust
let params = Self::uniform_params_ret_vec(m_plus_1);
```
```math
u_i=\frac{i}{m},\quad i=0..m
```
### 3.2. Knot vector 생성
```rust
let kv = Self::build_knot_vector_ret_kv(&params, p, ctrl_count);
```
- clamped + averaged knot 방식 (일반적인 global approximation 패턴)
### 3.3. 곡선 표현
- NURBS (비유리, w_i=1):
```math
C(u)=\sum _{j=0}^nN_{j,p}(u)P_j
```
여기서:
- P_0=Q_0
- P_n=Q_m
- 내부 P_1..P_{n-1} 미지수

### 3.4. 최소제곱 문제 설정
- 각 데이터 점 Q_i 에 대해:
```math
C(u_i)=\sum _{j=0}^nN_{j,p}(u_i)P_j\approx Q_i
```
- 이를 내부 control point에 대해 정리:
```math
\sum _{k=1}^{n-1}N_{k,p}(u_i)P_k=Q_i-N_{0,p}(u_i)P_0-N_{n,p}(u_i)P_n
```
- 코드에서:
    - $n_i[k-1]=N_{k,p}(u_i)$ → 내부 control point에 대한 basis
    - $n_0=N_{0,p}(u_i)$
    - $n_n=N_{n,p}(u_i)$
```math
R_i=Q_i-n_0Q_0-n_nQ_m
```
- 이걸 행렬 형태로 쓰면:
```math
AP_{\mathrm{inner}}\approx R
```
- $A\in \mathbb{R^{\mathnormal{(m+1)\times (n-1)}}}$
- $P_{\mathrm{inner}}\in \mathbb{R^{\mathnormal{(n-1)\times 3}}}$
- $R\in \mathbb{R^{\mathnormal{(m+1)\times 3}}}$

### 3.5. Normal equation
- 최소제곱 해:
```math
A^TAP_{\mathrm{inner}}=A^TR
```
- 코드:
```rust
ata[a][b] += n_i[a] * n_i[b];   // A^T A
atb_x[a]  += n_i[a] * rx;       // A^T R_x
...
```

- 각 좌표별로 독립적인 선형 시스템:
```math
(A^TA)X=A^TR_x
```
```math
(A^TA)Y=A^TR_y
```
```math
(A^TA)Z=A^TR_z
```
- 이를 on_solve_linear_system_gauss 로 푼다.

### 3.6. 최종 control point 구성
```math
P_0=Q_0,\quad P_k=(X_k,Y_k,Z_k),\  k=1..n-1,\quad P_n=Q_m
```
- 이로부터 NURBS 곡선 생성:
```rust
let mut curve = NurbsCurve::new(degree, ctrl, kv)?;
curve.domain.t0 = curve.kv.knots[p];
curve.domain.t1 = curve.kv.knots[n + 1];
```

## 4. local_approximation — 구간 분할 + 전역 근사 + fallback
- 목표
    - 전체 데이터 점 집합을 여러 구간(segment) 으로 나누고
    - 각 구간마다 global_approximation 수행
    - 구간의 양 끝점은 항상 데이터의 실제 점을 지나도록 구성
    - 짧은 구간은 Bezier/직선으로 fallback

### 4.1. 세그먼트 분할
```rust
let step = points_per_segment - 1;
let mut start = 0;

while start < n_data - 1 {
    let mut end = start + points_per_segment - 1;
    if end >= n_data { end = n_data - 1; }

    let seg_points = &points[start..=end];
    let seg_len = seg_points.len();
    ...
    if end == n_data - 1 { break; }
    start += step;
}
```

- 각 세그먼트는 points_per_segment 개의 점 (마지막은 부족할 수 있음)
- 세그먼트 간 겹치는 점 존재 (join continuity 확보)

### 4.2. 세그먼트별 처리 로직
```rust
let curve_opt = if seg_len >= p + 1 {
    let ctrl_cnt = min(ctrl_per_segment, seg_len);
    NurbsCurve::global_approximation(seg_points, degree, ctrl_cnt)
} else if seg_len == 3 {
    Some(Self::make_quadratic_through_3_points(...))
} else if seg_len == 2 {
    Some(Self::make_line_curve(...))
} else {
    None
};
```


- 케이스 1: 충분히 긴 세그먼트
    - $\mathrm{seg\_ len}\geq p+1$
    - 전역 근사 가능
    - global_approximation 호출
- 케이스 2: 점 3개
    - degree=2 Bezier로 정확히 통과시키는 곡선 생성
    - make_quadratic_through_3_points
- 케이스 3: 점 2개
    - 직선 NURBS (make_line_curve)
- 케이스 4: 그 외 (이상 케이스)
    - None → 루프 종료

### 4.3. 전체 곡선 집합의 성질
- 각 세그먼트 곡선은 자기 구간의 첫/마지막 데이터 점을 정확히 지난다.
- 세그먼트 간 겹치는 점에서:
    - 이전 세그먼트의 마지막 점 = 다음 세그먼트의 첫 점
    - 따라서 곡선 집합 전체가 원래 데이터의 첫/마지막 점을 지나고,
- 세그먼트 경계에서도 위치 연속(C⁰ continuity)을 가진다.
    - degree와 control point 수를 세그먼트마다 조절 가능
    - 짧은 구간은 Bezier/직선으로 처리해 수치적 안정성 확보

## 5. 전체 구조 요약
- global_approximation
    - 전역 최소제곱 NURBS fitting (끝점 통과, 내부 least squares)
- local_approximation
    - 데이터를 구간으로 나누고, 각 구간에 global approximation 적용
    - 짧은 구간은 Bezier/직선 fallback
- make_line_curve / make_quadratic_through_3_points
    - degenerate / short segment용 특수 해 analytic 구성

---
# global_approximation
## 1. global_approximation normal equation 유도 과정
### 1.1. 문제 설정
- 데이터 점:
```math
Q_i\in \mathbb{R^{\mathnormal{3}}},\quad i=0,\dots ,m
```
- degree p, control point 개수 n+1 (index j=0..n).
- NURBS (비유리, $w_j=1$ ):
```math
C(u)=\sum _{j=0}^nN_{j,p}(u)\, P_j
```
- 여기서:
    - $P_0=Q_0$
    - $P_n=Q_m$
    - 내부 control point $P_1,\dots ,P_{n-1}$ 를 미지수로 둔다.
- 파라미터:
```math
u_i,\quad i=0..m
```
- (코드에서는 uniform: $u_i=\frac{i}{m}$)

### 1.2. 각 데이터 점에서의 근사 조건
- 각 i 에 대해:
```math
C(u_i)\approx Q_i
```
- 즉,
```math
\sum _{j=0}^nN_{j,p}(u_i)\, P_j\approx Q_i
```
- 이를 내부 control point에 대해 정리:
```math
\sum _{k=1}^{n-1}N_{k,p}(u_i)\, P_k=Q_i-N_{0,p}(u_i)\, P_0-N_{n,p}(u_i)\, P_n
```
- 코드에서:
- $n_i[k-1]=N_{k,p}(u_i)$
- $n_0=N_{0,p}(u_i)$
- $n_n=N_{n,p}(u_i)$
- $R_i=Q_i-n_0Q_0-n_nQ_m$

### 1.3. 행렬 형태로 쓰기
- 내부 control point를 벡터로 모으면:
```math
\mathbf{P_{\mathrm{inner}}}=\left[ \begin{matrix}P_1\\ P_2\\ \vdots \\ P_{n-1}\end{matrix}\right] \in \mathbb{R^{\mathnormal{(n-1)\times 3}}}
```
- 각 i 에 대해:
```math
\sum _{k=1}^{n-1}N_{k,p}(u_i)\, P_k=R_i
```
- 이를 행렬로 쓰면:
```math
A\, \mathbf{P_{\mathrm{inner}}}\approx \mathbf{R}
```
- 여기서:
    - $A\in \mathbb{R^{\mathnormal{(m+1)\times (n-1)}}}$, $A_{ik}=N_{k,p}(u_i)$
    - $\mathbf{R}\in \mathbb{R^{\mathnormal{(m+1)\times 3}}}$, $R_i$ 는 위에서 정의한 residual

### 1.4. 최소제곱 문제
- 목표:
```math
\min _{\mathbf{P_{\mathrm{inner}}}}\| A\mathbf{P_{\mathrm{inner}}}-\mathbf{R}\| ^2
```
- 각 좌표별로 분리해서 보면:
```math
\min _X\| AX-R_x\| ^2
```
```math
\min _Y\| AY-R_y\| ^2
```
```math
\min _Z\| AZ-R_z\| ^2
```
- 여기서:
    - $X,Y,Z\in \mathbb{R^{\mathnormal{n-1}}}$
    - $R_x,R_y,R_z\in \mathbb{R^{\mathnormal{m+1}}}$

### 1.5. Normal equation 유도
- 스칼라로 보면:
```math
E(X)=\| AX-R_x\| ^2=(AX-R_x)^T(AX-R_x)
```
- 미분:
```math
\frac{\partial E}{\partial X}=2A^T(AX-R_x)
```
- 최소값에서:
```math
A^T(AX-R_x)=0\Rightarrow A^TAX=A^TR_x
```
- Y, Z도 동일:
```math
A^TAY=A^TR_y,\quad A^TAZ=A^TR_z
```
- 코드에서:
```rust
ata[a][b] += n_i[a] * n_i[b];   // A^T A
atb_x[a]  += n_i[a] * rx;       // A^T R_x
...
let sol_x = on_solve_linear_system_gauss(&mut ax, &mut bx)?;
```
- 즉, normal equation을 정확히 구현한 것이 맞다.

## 2. local_approximation에서 C¹ 연속성까지 맞추는 확장 설계
- 지금 구현은:
    - 각 세그먼트가 자기 구간의 첫/마지막 데이터 점을 통과
    - 세그먼트 간 위치 연속(C⁰ continuity) 는 확보됨
    - (겹치는 데이터 점을 공유하니까)
- 이제 목표는:
    - 인접 세그먼트 사이에서 C¹ 연속성 (tangent 연속) 까지 맞추는 설계.


### 2.1. 세그먼트 구조 다시 보기
- 각 세그먼트 $S_k$ 에 대해:
- 데이터 점: $Q_{i_k},\dots ,Q_{j_k}$
- 곡선: $C_k(u),\  u\in [0,1]$ (혹은 세그먼트별 domain)
- 현재는:
    - $C_k(0)=Q_{i_k}$
    - $C_k(1)=Q_{j_k}$
- 그리고 인접 세그먼트 $S_k,S_{k+1}$ 는:
    - $Q_{j_k}=Q_{i_{k+1}}$ 를 공유
        - 위치 연속 C⁰

### 2.2. C¹ 연속 조건
- 두 세그먼트 $C_k,C_{k+1}$ 가 join point $P=Q_{j_k}=Q_{i_{k+1}}$ 에서 C¹ 이려면:
    - $C_k(1)=C_{k+1}(0)=P$
    - $C_k'(1)=C_{k+1}'(0)$
- 즉, 접선 벡터가 같아야 한다.

### 2.3. B-spline에서 끝점 접선과 control point 관계
- clamped B-spline (혹은 Bezier)에서:
    - degree p
    - 끝점 근처 control point:
    - 왼쪽 끝: $P_0,P_1$
    - 오른쪽 끝: $P_{n-1},P_n$
- 끝점에서의 도함수는:
```math
C'(0)=p\frac{P_1-P_0}{U_{p+1}-U_p}
```
```math
C'(1)=p\frac{P_n-P_{n-1}}{U_{n+1}-U_n}
```
- Bezier의 경우:
```math
C'(0)=p(P_1-P_0)
```
```math
C'(1)=p(P_n-P_{n-1})
```
- 즉, 끝점 접선은 인접 control point 차이와 비례한다.

### 2.4. C¹ 연속 조건을 control point로 쓰면
- 두 세그먼트 S_k,S_{k+1} 에 대해:
    - $S_k$ 의 마지막 두 control point: $P_{n_k-1}^{(k)},P_{n_k}^{(k)}$
    - $S_{k+1}$ 의 처음 두 control point: $P_0^{(k+1)},P_1^{(k+1)}$
- C¹ 조건:
```math
C_k'(1)=C_{k+1}'(0)
```
- Bezier라 가정하면:
```math
p(P_{n_k}^{(k)}-P_{n_k-1}^{(k)})=p(P_1^{(k+1)}-P_0^{(k+1)})
```
- 즉,
```math
P_{n_k}^{(k)}-P_{n_k-1}^{(k)}=P_1^{(k+1)}-P_0^{(k+1)}
```
- join point에서:
```math
P_{n_k}^{(k)}=P_0^{(k+1)}=P
```
- 따라서:
```math
P-P_{n_k-1}^{(k)}=P_1^{(k+1)}-P\Rightarrow P_1^{(k+1)}=2P-P_{n_k-1}^{(k)}
```
- 즉, 다음 세그먼트의 첫 번째 내부 control point는 이전 세그먼트의 마지막 두 control point로부터 결정된다.

### 2.5. 이걸 설계에 녹이는 방법
- 지금 local_approximation 은 세그먼트별로 완전히 독립적으로 global_approximation 을 호출하고 있음.
- C¹ 연속을 맞추려면:
    - 세그먼트 $S_0$ 를 먼저 global approximation으로 풀고
    - 그 결과로 나온 마지막 두 control point $P_{n_0-1}^{(0)}$,$P_{n_0}^{(0)}$ 를 이용해
    - 세그먼트 $S_1$ 의 초기 조건(tangent constraint) 을 강제로 걸어야 한다.
- 즉, S_1 의 global approximation에서:
- $P_0^{(1)}=Q_{\mathrm{join}}$ (위치 고정)
- $P_1^{(1)}=2Q_{\mathrm{join}}-P_{n_0-1}^{(0)}$ (접선 고정)
- 이렇게 되면:
    - $S_0$ 의 끝점 접선 = $S_1$ 의 시작점 접선
    - join에서 C¹ 연속 확보

### 2.6. global_approximation에 **끝점 tangent 고정** 을 넣으려면?
- 지금은:
    - $P_0=Q_0$
    - $P_n=Q_m$
    - 내부 $P_1..P_{n-1}$ 를 least squares로 푸는 구조
C¹ 확장을 위해:
    - 시작 세그먼트: 지금 구조 그대로 사용 (끝점만 고정)
    - 이후 세그먼트:
    - $P_0=Q_0$
    - $P_1$ 을 이미 알고 있는 값으로 고정
    - 나머지 $P_2..P_{n-1},P_n$ 를 unknown으로 두고 least squares
- 즉, normal equation을 만들 때:
    - basis에서 $N_0$,$N_1$ 에 해당하는 항을 모두 오른쪽으로 넘기고
    - unknown은 $P_2..P_{n-1},P_n$ 만 남긴다.
- 수식 구조는 지금 global_approximation과 완전히 동일하고,
    단지 **고정된 control point의 개수** 가 늘어나는 것뿐.

### 2.7. 요약: C¹ local approximation 설계 플로우
- 첫 세그먼트 $S_0$:
    - 지금의 global_approximation 그대로 사용
    - $P_0^{(0)}=Q_{\mathrm{start}}, P_{n_0}^{(0)}=Q_{\mathrm{join}}$
- 두 번째 세그먼트 $S_1$:
    - $P_0^{(1)}=Q_{\mathrm{join}}$
    - $P_1^{(1)}=2Q_{\mathrm{join}}-P_{n_0-1}^{(0)}$ 로 고정
- 나머지 control point만 unknown으로 두고 least squares
    - 세 번째 세그먼트 $S_2$ 이후도 동일 패턴 반복
이렇게 하면:
    - 전체 곡선 집합이 C⁰ + C¹ 연속
    - 세그먼트별로 여전히 local fitting
    - global system을 한 번에 풀 필요 없이, 앞에서부터 순차적으로 propagate 가능

## 3. 수식 검증 요약
- global_approximation 의 normal equation 유도:
    - 표준 least squares + normal equation과 1:1 대응
    - 코드의 ata, atb_x/y/z 누적 방식은 정확함
- local_approximation 의 C⁰ 연속성:
    - 세그먼트 간 겹치는 데이터 점을 공유하므로 위치 연속 확보
- C¹ 확장 설계:
    - B-spline/Bezier 끝점 도함수 수식
    - 인접 세그먼트 control point 관계
    - $P_1^{(k+1)}=2P_{\mathrm{join}}-P_{n_k-1}^{(k)}$
    - 이를 global_approximation의 **고정 control point** 로 넣으면 수식적으로 일관됨


---

## 코드 검증

### 1. make_line_curve — 직선 NURBS
- 수식 목표
    - degree p=1
    - control points: P_0=A,P_1=B
    - knot: [0,0,1,1]
- 곡선:
```math
C(u)=(1-u)A+uB,\quad u\in [0,1]
```
- 코드
```rust
let degree: Degree = 1;
let ctrl = vec![
    Point4D::homogeneous(a.x, a.y, a.z, 1.0),
    Point4D::homogeneous(b.x, b.y, b.z, 1.0),
];
let kv = KnotVector { knots: vec![0.0, 0.0, 1.0, 1.0] };
let mut c = NurbsCurve::new(degree, ctrl, kv).expect("line curve build failed");
c.domain.t0 = 0.0;
c.domain.t1 = 1.0;
```
- degree, control point 개수, knot vector 모두 수식과 정확히 일치.
- 비유리(w=1)라서 B-spline = 선형 보간.
- 구현이 수식을 정확히 따른다 → ✅ 문제 없음.

### 2. make_quadratic_through_3_points — 3점 통과 2차 Bezier
- 수식 목표
    - 입력: $Q_0,Q_1,Q_2$
    - Bezier degree 2:
```math
C(u)=(1-u)^2P_0+2u(1-u)P_1+u^2P_2
```
- 조건:
```math
C(0)=Q_0,\quad C(0.5)=Q_1,\quad C(1)=Q_2
```
- 따라서:
```math
P_0=Q_0,\quad P_2=Q_2
```

- 코드
```rust
let p: Degree = 2;

let p0 = q0;
let p2 = q2;

let p1 = Point3D::new(
    2.0 * q1.x - 0.5 * p0.x - 0.5 * p2.x,
    2.0 * q1.y - 0.5 * p0.y - 0.5 * p2.y,
    2.0 * q1.z - 0.5 * p0.z - 0.5 * p2.z,
);

let ctrl = vec![
    Point4D::homogeneous(p0.x, p0.y, p0.z, 1.0),
    Point4D::homogeneous(p1.x, p1.y, p1.z, 1.0),
    Point4D::homogeneous(p2.x, p2.y, p2.z, 1.0),
];

// quadratic Bezier knot (clamped): [0,0,0, 1,1,1]
let kv = KnotVector { knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0] };
```

- $P_1$ 계산식이 수식과 완전히 동일.
- knot vector도 정확한 quadratic Bezier 형태.
- 이 구현은 “u=0,0.5,1에서 Q0,Q1,Q2를 통과하는 2차 Bezier”를 정확히 만든다 → ✅ 수식과 일치.

### 3. global_approximation — normal equation
- 핵심은 여기.
- 수식 구조
    - 데이터: $Q_i,i=0..m$
    - control: $P_0=Q_0,P_n=Q_m,P_1..P_{n-1}$ unknown
- 곡선:
```math
C(u)=\sum _{j=0}^nN_{j,p}(u)P_j
```
- 각 u_i 에서:
```math
\sum _{k=1}^{n-1}N_{k,p}(u_i)P_k=Q_i-N_{0,p}(u_i)Q_0-N_{n,p}(u_i)Q_m=R_i
```
- 행렬:
```math
AP_{\mathrm{inner}}\approx R
```
- normal equation:
```math
A^TAX=A^TR_x,\quad A^TAY=A^TR_y,\quad A^TAZ=A^TR_z
```
- 코드 매핑
- 파라미터, knot:
```rust
let params = Self::uniform_params_ret_vec(m_plus_1);
let kv = Self::build_knot_vector_ret_kv(&params, p, ctrl_count);
let u_vec = &kv.knots;
```

- basis 계산:
```rust
let span = Self::find_span_ret_vec(n, p, u, u_vec);
let n_vec = Self::basis_funs_ret_vec(span, u, p, u_vec);
```

- $N_0,N_n,N_k$ 분리:
```rust
let mut n_i = vec![0.0; inner_count];
let mut n_0 = 0.0;
let mut n_n = 0.0;

for j in 0..=p {
    let col = span - p + j;
    let n_j = n_vec[j];
    if col == 0 {
        n_0 = n_j;
    } else if col == n {
        n_n = n_j;
    } else if col < n {
        let idx = col - 1; // 1..n-1 → 0..inner_count-1
        if idx < inner_count {
            n_i[idx] = n_j;
        }
    }
}
```

- residual:
```rust
let rx = qi.x - n_0 * q0.x - n_n * q_last.x;
let ry = qi.y - n_0 * q0.y - n_n * q_last.y;
let rz = qi.z - n_0 * q0.z - n_n * q_last.z;
```

- normal equation 누적:
```rust
for a in 0..inner_count {
    for b in 0..inner_count {
        ata[a][b] += n_i[a] * n_i[b]; // A^T A
    }
    atb_x[a] += n_i[a] * rx;         // A^T R_x
    atb_y[a] += n_i[a] * ry;
    atb_z[a] += n_i[a] * rz;
}
```

- 선형 시스템 풀이:
```rust
let mut ax = ata.clone();
let mut bx = atb_x.clone();
let sol_x = on_solve_linear_system_gauss(&mut ax, &mut bx)?;
...
```

- control point 구성:
```rust
ctrl.push(Point4D::homogeneous(q0.x, q0.y, q0.z, 1.0));
for k in 0..inner_count {
    ctrl.push(Point4D::homogeneous(sol_x[k], sol_y[k], sol_z[k], 1.0));
}
ctrl.push(Point4D::homogeneous(q_last.x, q_last.y, q_last.z, 1.0));
```

- 수식에서 유도한 normal equation과 완전히 동일한 구조로 구현되어 있고,  
    basis 분리, residual 정의, A^TA, A^TR 누적 모두 수식과 1:1 대응한다.
- global_approximation은 수학적으로 정확하게 구현되어 있다.

### 4. local_approximation — 세그먼트 분할 + fallback
- 수식/의도
    - 전체 데이터 $Q_0..Q_M$ 를 구간별로 나누고,
    - 각 구간에 대해 global_approximation 또는 analytic curve (직선/Bezier) 적용.
    - 각 세그먼트는 자기 구간의 첫/마지막 점을 통과.
    - 세그먼트 간 겹치는 점을 공유 → C⁰ 연속.
- 코드
- 세그먼트 분할:
```rust
let step = points_per_segment - 1;
let mut start = 0usize;

while start < n_data - 1 {
    let mut end = start + points_per_segment - 1;
    if end >= n_data { end = n_data - 1; }

    let seg_points = &points[start..=end];
    let seg_len = seg_points.len();
    ...
    if end == n_data - 1 { break; }
    start += step;
}
```

- 세그먼트별 처리:
```rust
let curve_opt = if seg_len >= p + 1 {
    let ctrl_cnt = min(ctrl_per_segment, seg_len);
    NurbsCurve::global_approximation(seg_points, degree, ctrl_cnt)
} else if seg_len == 3 {
    Some(Self::make_quadratic_through_3_points(seg_points[0], seg_points[1], seg_points[2]))
} else if seg_len == 2 {
    Some(Self::make_line_curve(seg_points[0], seg_points[1]))
} else {
    None
};
```

- seg_len ≥ p+1 → global_approximation: 이미 수식 검증 완료.
- seg_len == 3 → 위에서 검증한 quadratic Bezier.
- seg_len == 2 → 위에서 검증한 line NURBS.
- 세그먼트 경계는 데이터 인덱스를 공유하므로, 각 세그먼트의 첫/마지막 control point가 해당 데이터 점을 지나고,  
    그 점이 인접 세그먼트와 공유됨 → C⁰ 연속성 확보.
- 의도와 구현이 일치한다. ✅

## 5. 요약 — “수식 ↔ 코드” 검증 결과
- make_line_curve
    - 직선 보간 수식과 완전히 일치.
- make_quadratic_through_3_points   
    - 3점 통과 quadratic Bezier의 해를 정확히 구현.
- global_approximation
    - least squares + normal equation 유도와 1:1 대응.
    - A^TA, A^TR, 고정 끝점 처리 모두 수식과 정확히 맞음.
- local_approximation
    - 세그먼트 분할 전략, fallback 조건, C⁰ 연속성 확보 방식이 설계 의도와 일치.

---

## 🌏 Global Approximation — 전체 데이터를 한 번에 다룬다
- global_approximation(points, degree, ctrl_count) 는 이름 그대로  
    전체 데이터 세트를 한 번에(global) 처리한다.
- ✔ 데이터 처리 방식
    - 입력 데이터: points[0..m]
    - 미지수(control points): P[0..n]
    - 전체 데이터에 대해 하나의 큰 선형 시스템 A·P = Q 를 만든다.
    - 즉, 모든 데이터 점이 동시에 control point 계산에 영향을 준다.
- ✔ 특징
    - 전체 데이터에 대해 전역적으로 최적화된 곡선이 나온다.
    - 데이터가 많으면 행렬 크기가 커져 계산량이 증가한다.
    - 데이터가 불규칙하거나 길이가 길면 수치적으로 불안정해질 수 있다.
    - 곡선 전체가 하나의 시스템으로 묶여 있기 때문에 한 점이 바뀌면 전체 곡선이 변한다.
- ✔ 그래서 “global”
- 데이터 전체를 한 번에 보고, 전체를 만족하는 하나의 곡선을 만든다.
- 즉, 전역(global) least-squares fitting.

## 🧩 Local Approximation — 데이터를 구간별로 나누어 처리한다
- local_approximation(points, degree, points_per_segment, ctrl_per_segment) 는  
    전체 데이터를 여러 작은 구간(segment) 으로 나누고,  
    각 구간마다 독립적으로(global_approximation을 호출하여) 곡선을 만든다.
- 즉, 전체 데이터를 한 번에 처리하지 않고, 작은 지역(local) 단위로 나누어 처리한다.
- ✔ 데이터 처리 방식
- 예를 들어:
```
points = [p0, p1, p2, p3, p4, p5, p6, p7]
points_per_segment = 4
```

- 세그먼트는 이렇게 나뉜다:
    - Segment 1: p0, p1, p2, p3
    - Segment 2: p3, p4, p5, p6
    - Segment 3: p6, p7
- 각 세그먼트마다:
    - 데이터가 적으므로 작은 행렬로 global_approximation 수행
    - 짧은 구간은 Bezier/직선 fallback
- ✔ 특징
    - 계산량이 작다 (작은 행렬 여러 개)
    - 수치적으로 안정적
    - 데이터가 많아도 빠르다
    - 한 세그먼트의 변화가 다른 세그먼트에 영향을 거의 주지 않는다
    - 세그먼트 경계에서 C⁰ 연속성 확보 (필요하면 C¹도 가능)
- ✔ 그래서 “local”
    - 전체 데이터를 한 번에 처리하지 않고,
    - 지역(local) 단위로 나누어 fitting 하기 때문에 local approximation.

## 🎯 두 방식의 차이를 한 문장으로 요약하면
- Global approximation
    - 전체 데이터를 한 번에 보고, 하나의 큰 시스템을 풀어 전체 곡선을 만든다.

- Local approximation
    - 데이터를 여러 구간으로 나누고, 각 구간을 독립적으로 fitting하여 여러 개의 작은 곡선을 만든다.


## 🔍 Local vs Global Approximation 이름이 중요한가?

- 이름이 단순한 스타일 차이가 아니라, 근사 방식의 철학 자체가 다르기 때문.

| 구분 | Global Approximation | Local Approximation |
|------|------------------------|------------------------|
| 데이터 처리 방식 | 전체 데이터를 한 번에 처리 | 데이터를 여러 구간(segment)으로 나누어 처리 |
| 선형 시스템 | 큰 행렬 1개 (전역 시스템) | 작은 행렬 여러 개 (세그먼트별 독립 시스템) |
| 계산 비용 | 높음 (O(n³)) | 낮음 (여러 개의 작은 O(k³)) |
| 수치 안정성 | 데이터가 많으면 불안정해질 수 있음 | 매우 안정적 (작은 문제 반복) |
| 곡선 변화 영향 | 한 점이 전체 곡선에 영향을 줌 | 한 세그먼트만 영향, 지역적 변화 |
| 연속성 | 자동으로 C⁰, C¹, C² 등 유지 가능 | 기본은 C⁰, 필요하면 C¹ 조건 추가 가능 |
| 용도 | 고정밀 전역 fitting, 데이터가 적을 때 | 대규모 데이터, 실시간 처리, 노이즈 많은 데이터 |
| 장점 | 전체적으로 매끄럽고 균일한 곡선 | 빠르고 안정적이며 지역적 제어 가능 |
| 단점 | 계산량 많고 민감함 | 세그먼트 경계에서 연속성 관리 필요 |

---

## 🎯 Normal Equation이란? 
- Least squares 문제를 “미분해서 0으로 만드는 과정”에서 자연스럽게 등장하는 선형 방정식.
- 즉,
```math
\min _x\| Ax-b\| ^2
```
- 을 풀기 위해
```math
A^TAx=A^Tb
```
- 를 푸는 것.
- 이게 바로 normal equation이야.

## 🧩 왜 이런 식이 나오지?
- 우리가 하고 싶은 건:
    - Ax 는 모델이 예측한 값
    - b 는 실제 데이터
    - 둘의 차이를 최소화하고 싶다
- 즉,
```math
\| Ax-b\| ^2
```
- 이걸 최소화하는 x 를 찾는 게 least squares.
- 이걸 미분해서 0으로 만들면?
```math
A^TAx=A^Tb
```
- 이게 바로 normal equation.

### 🧠 조금 더 수학적으로 보면
- 목표 함수:
```math
E(x)=(Ax-b)^T(Ax-b)
```
- 미분:
```math
\nabla _xE=2A^T(Ax-b)
```
- 최소값에서:
```math
A^T(Ax-b)=0
```
- 정리하면:
```math
A^TAx=A^Tb
```

## 🔥 왜 “normal”이라는 이름이 붙었을까?
- 기하학적으로 보면:
    - Ax-b 는 잔차(residual)
    - normal equation은 잔차가 A의 column space에 수직(normal) 이 되도록 만드는 조건
- 즉,
```math
A^T(Ax-b)=0
```
- 은 **잔차가 A의 모든 column에 대해 직교한다** 는 뜻.
- 그래서 normal equation.

## 🧪 코드에서 normal equation이 어디에 쓰였나?
- global_approximation에서 내부 control point를 구할 때:
    - A = basis function matrix
    - x = 내부 control point
    - b = 데이터 점에서의 residual
- 이렇게 하고 있음:
```rust
ata[a][b] += n_i[a] * n_i[b];   // A^T A
atb_x[a]  += n_i[a] * rx;       // A^T R_x
```

- 즉,
```math
(A^TA)x=A^TR
```
- 을 그대로 구현한 것.

### ⚠️ Normal Equation의 단점도 있다
    - A^TA 는 condition number가 제곱으로 나빠짐
    - 수치적으로 불안정할 수 있음
    - 그래서 실제로는 QR 분해나 SVD를 더 선호함
- 하지만:
- NURBS fitting에서는 행렬이 작고 구조가 단순해서 normal equation + LU로도 충분히 안정적
- pivoting까지 넣었으니 훨씬 안전함

## 🎉 결론
- Least squares → normal equation은 미분해서 0으로 만드는 과정
- AᵀA x = Aᵀb 가 normal equation
- 잔차가 column space에 직교(normal) 하도록 만드는 조건

---

## 소스 코드
```rust
/// Global approximation:
/// - degree = p, number of control points = ctrl_count
/// - Ensure the curve passes exactly through the first and last data points (P0 = Q0, Pn = Qm)
/// - Solve only the internal control points using the least squares
pub fn global_approximation(
    points: &[Point3D],
    degree: Degree,
    ctrl_count: usize,
) -> Option<NurbsCurve> {
    let p = degree as usize;
    let m_plus_1 = points.len();
    if m_plus_1 < 2 {
        return None;
    }
    if ctrl_count < p + 1 {
        return None;
    }

    let n = ctrl_count - 1; // control point index: 0..n
    if n < 1 {
        return None;
    }

    // Parameters: uniformly distributed from 0 to 1
    let params = Self::uniform_params_ret_vec(m_plus_1);

    // knot vector: clamped + averaged
    let kv = Self::build_knot_vector_ret_kv(&params, p, ctrl_count);
    let u_vec = &kv.knots;

    // Number of internal control points: P1..P_{n-1}
    let inner_count = if n >= 2 { n - 1 } else { 0 };
    if inner_count == 0 {
        // Special case with 2 control points: connect as a straight line
        let q0 = points[0];
        let q1 = points[m_plus_1 - 1];
        let ctrl = vec![
            Point4D::homogeneous(q0.x, q0.y, q0.z, 1.0),
            Point4D::homogeneous(q1.x, q1.y, q1.z, 1.0),
        ];
        let mut curve = NurbsCurve::new(degree, ctrl, kv).expect("Nurbs Curve Fail");
        curve.domain.t0 = curve.kv.knots[p];
        curve.domain.t1 = curve.kv.knots[n + 1];
        return Some(curve);
    }

    // Normal equation: (A^T A) * P_inner = A^T R
    // A: (m_plus_1 x inner_count),  R: (m_plus_1 x 3)
    let mut ata = vec![vec![0.0; inner_count]; inner_count];
    let mut atb_x = vec![0.0; inner_count];
    let mut atb_y = vec![0.0; inner_count];
    let mut atb_z = vec![0.0; inner_count];

    let q0 = points[0];
    let q_last = points[m_plus_1 - 1];

    for (i, qi) in points.iter().enumerate() {
        let u = params[i];
        let span = Self::find_span_ret_vec(n, p, u, u_vec);
        let n_vec = Self::basis_funs_ret_vec(span, u, p, u_vec);

        // Collect N0, Nn, and internal Ni
        let mut n_i = vec![0.0; inner_count];
        let mut n_0 = 0.0;
        let mut n_n = 0.0;

        for j in 0..=p {
            let col = span - p + j;
            let n_j = n_vec[j];
            if col == 0 {
                n_0 = n_j;
            } else if col == n {
                n_n = n_j;
            } else if col < n {
                // Internal control point indices: 1..(n-1)
                let idx = col - 1; // 0..(inner_count-1)
                if idx < inner_count {
                    n_i[idx] = n_j;
                }
            }
        }

        // R_i = Q_i - N0*Q0 - Nn*Qm
        let rx = qi.x - n_0 * q0.x - n_n * q_last.x;
        let ry = qi.y - n_0 * q0.y - n_n * q_last.y;
        let rz = qi.z - n_0 * q0.z - n_n * q_last.z;

        // A^T A, A^T R 누적
        for a in 0..inner_count {
            for b in 0..inner_count {
                ata[a][b] += n_i[a] * n_i[b];
            }
            atb_x[a] += n_i[a] * rx;
            atb_y[a] += n_i[a] * ry;
            atb_z[a] += n_i[a] * rz;
        }
    }

    // Solve the linear system for each coordinate
    let mut ax = ata.clone();
    let mut bx = atb_x.clone();
    let sol_x = on_solve_linear_system_gauss(&mut ax, &mut bx)?;

    let mut ay = ata.clone();
    let mut by = atb_y.clone();
    let sol_y = on_solve_linear_system_gauss(&mut ay, &mut by)?;

    let mut az = ata;
    let mut bz = atb_z;
    let sol_z = on_solve_linear_system_gauss(&mut az, &mut bz)?;

    // Control point structure (P0, interior, Pn)
    let mut ctrl: Vec<Point4D> = Vec::with_capacity(ctrl_count);
    ctrl.push(Point4D::homogeneous(q0.x, q0.y, q0.z, 1.0));
    for k in 0..inner_count {
        ctrl.push(Point4D::homogeneous(sol_x[k], sol_y[k], sol_z[k], 1.0));
    }
    ctrl.push(Point4D::homogeneous(q_last.x, q_last.y, q_last.z, 1.0));

    let mut curve = NurbsCurve::new(degree, ctrl, kv).expect("Nurbs Curve Fail");
    curve.domain.t0 = curve.kv.knots[p];
    curve.domain.t1 = curve.kv.knots[n + 1];

    Some(curve)
}
```
```rust
/// seg_len < p+1일 때 쓰는 fallback: degree=1 clamped line curve
fn make_line_curve(a: Point3D, b: Point3D) -> NurbsCurve {
    let degree: Degree = 1;
    let ctrl = vec![
        Point4D::homogeneous(a.x, a.y, a.z, 1.0),
        Point4D::homogeneous(b.x, b.y, b.z, 1.0),
    ];
    // open-clamped for degree 1, n=1 => m = n+p+1 = 3 => 4 knots
    let kv = KnotVector { knots: vec![0.0, 0.0, 1.0, 1.0] };
    let mut c = NurbsCurve::new(degree, ctrl, kv).expect("line curve build failed");
    c.domain.t0 = 0.0;
    c.domain.t1 = 1.0;
    c
}
```
```rust
fn make_quadratic_through_3_points(q0: Point3D, q1: Point3D, q2: Point3D) -> NurbsCurve {
    let p: Degree = 2;

    let p0 = q0;
    let p2 = q2;

    let p1 = Point3D::new(
        2.0 * q1.x - 0.5 * p0.x - 0.5 * p2.x,
        2.0 * q1.y - 0.5 * p0.y - 0.5 * p2.y,
        2.0 * q1.z - 0.5 * p0.z - 0.5 * p2.z,
    );

    let ctrl = vec![
        Point4D::homogeneous(p0.x, p0.y, p0.z, 1.0),
        Point4D::homogeneous(p1.x, p1.y, p1.z, 1.0),
        Point4D::homogeneous(p2.x, p2.y, p2.z, 1.0),
    ];

    // quadratic Bezier knot (clamped): [0,0,0, 1,1,1]
    let kv = KnotVector { knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0] };

    let mut c = NurbsCurve::new(p, ctrl, kv).expect("quad curve build failed");
    c.domain.t0 = 0.0;
    c.domain.t1 = 1.0;
    c
}
```
```rust
/// 4점(0, 1/3, 2/3, 1) 정확히 통과하는 cubic Bezier
fn make_cubic_through_4(q0: Point3D, q1: Point3D, q2: Point3D, q3: Point3D) -> Option<NurbsCurve> {
    let q0 = on_safe_point3(q0)?;
    let q1 = on_safe_point3(q1)?;
    let q2 = on_safe_point3(q2)?;
    let q3 = on_safe_point3(q3)?;

    let degree: Degree = 3;

    // Bezier:
    // B(1/3) = (8/27)P0 + (4/9)P1 + (2/9)P2 + (1/27)P3 = Q1
    // B(2/3) = (1/27)P0 + (2/9)P1 + (4/9)P2 + (8/27)P3 = Q2
    //
    // Unknowns: P1, P2 (vector)
    // Linear system (per coordinate):
    // (4/9) P1 + (2/9) P2 = Q1 - (8/27)P0 - (1/27)P3
    // (2/9) P1 + (4/9) P2 = Q2 - (1/27)P0 - (8/27)P3
    //
    // Multiply by 9:
    // 4 P1 + 2 P2 = 9R1
    // 2 P1 + 4 P2 = 9R2
    // Solve:
    // P1 = (3R1 - R2)
    // P2 = (-R1 + 3R2)

    let r1 = Point3D::new(
        q1.x - (8.0/27.0)*q0.x - (1.0/27.0)*q3.x,
        q1.y - (8.0/27.0)*q0.y - (1.0/27.0)*q3.y,
        q1.z - (8.0/27.0)*q0.z - (1.0/27.0)*q3.z,
    );
    let r2 = Point3D::new(
        q2.x - (1.0/27.0)*q0.x - (8.0/27.0)*q3.x,
        q2.y - (1.0/27.0)*q0.y - (8.0/27.0)*q3.y,
        q2.z - (1.0/27.0)*q0.z - (8.0/27.0)*q3.z,
    );

    let p1 = Point3D::new(3.0*r1.x - r2.x, 3.0*r1.y - r2.y, 3.0*r1.z - r2.z);
    let p2 = Point3D::new(-r1.x + 3.0*r2.x, -r1.y + 3.0*r2.y, -r1.z + 3.0*r2.z);

    if !on_is_finite_point3(&p1) || !on_is_finite_point3(&p2) { return None; }

    let ctrl = vec![
        Point4D::homogeneous(q0.x, q0.y, q0.z, 1.0),
        Point4D::homogeneous(p1.x, p1.y, p1.z, 1.0),
        Point4D::homogeneous(p2.x, p2.y, p2.z, 1.0),
        Point4D::homogeneous(q3.x, q3.y, q3.z, 1.0),
    ];
    let kv = KnotVector { knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0] };

    let mut c = NurbsCurve::new(degree, ctrl, kv).ok()?;
    c.domain.t0 = 0.0;
    c.domain.t1 = 1.0;
    Some(c)
}
```
```rust
/// Local approximation:
/// - Divide points into intervals and perform global_approximation on each interval
/// - Since each interval passes through its first/last data point,
///   the entire curve set passes through the original data's first/last points,
///   and the join points of adjacent intervals also match.
pub fn local_approximation(
    points: &[Point3D],
    degree: Degree,
    points_per_segment: usize,
    ctrl_per_segment: usize,
) -> Vec<NurbsCurve> {
    let mut result = Vec::new();
    let n_data = points.len();
    if n_data < 2 { return result; }

    let p = degree as usize;
    if points_per_segment < 2 { return result; }
    if ctrl_per_segment < p + 1 { return result; }

    let step = points_per_segment - 1;
    let mut start = 0usize;

    while start < n_data - 1 {
        let mut end = start + points_per_segment - 1;
        if end >= n_data { end = n_data - 1; }

        let seg_points = &points[start..=end];
        let seg_len = seg_points.len();

        // -------------------------------
        // ✅ 핵심: 짧은 세그먼트 fallback
        // -------------------------------
        let curve_opt = if seg_len >= p + 1 {
            let ctrl_cnt = (std::cmp::min)(ctrl_per_segment, seg_len);
            NurbsCurve::global_approximation(seg_points, degree, ctrl_cnt)
        }
        else if seg_len == 4 {
            Self::make_cubic_through_4(seg_points[0], seg_points[1], seg_points[2], seg_points[3])
        }
        else if seg_len == 3 {
            Some(Self::make_quadratic_through_3_points(seg_points[0], seg_points[1], seg_points[2]))
        } else if seg_len == 2 {
            Some(Self::make_line_curve(seg_points[0], seg_points[1]))
        } else {
            None
        };

        let mut curve = match curve_opt {
            Some(c) => c,
            None => break, // 실패하면 중단 (NaN 방지 정책)
        };

        // ✅ endpoints hard clamp (안전)
        // 첫/마지막 ctrl은 segment endpoints로 강제
        if !curve.ctrl.is_empty() {
            curve.ctrl[0].x = seg_points[0].x;
            curve.ctrl[0].y = seg_points[0].y;
            curve.ctrl[0].z = seg_points[0].z;

            let last = curve.ctrl.len() - 1;
            curve.ctrl[last].x = seg_points[seg_len - 1].x;
            curve.ctrl[last].y = seg_points[seg_len - 1].y;
            curve.ctrl[last].z = seg_points[seg_len - 1].z;
        }

        // ✅ ctrl에 NaN이 있으면 버리고 중단 (원인 추적을 위해)
        if curve.ctrl.iter().any(|cp| !cp.x.is_finite() || !cp.y.is_finite() || !cp.z.is_finite() || !cp.w.is_finite()) {
            break;
        }
        result.push(curve);
        if end == n_data - 1 { break; }
        start += step;
    }

    result
}
```
---
