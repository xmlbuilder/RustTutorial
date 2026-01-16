## 🎯 1. Bezier Patch (Bezier Surface)
- 정의식
- 차수 p,q 의 Bezier 패치는 다음과 같이 정의됩니다.
```math
S(u,v)=\sum _{i=0}^p\sum _{j=0}^qB_i^p(u)\, B_j^q(v)\, P_{ij}
```
- 여기서
    - $B_i^p(u)$ = Bernstein basis
    - $B_i^p(u)={p \choose i}u^i(1-u)^{p-i}$
    - $P_{ij}$ = 제어점
    - $u,v\in [0,1]$
- 특징
    - 제어점이 직관적이다. (Bezier curve와 동일한 의미)
    - 항상 convex hull 안에 존재한다.
    - 로컬 수정 불가 (전체 패치가 움직임)
    - 파라미터 구간은 항상 [0,1] × [0,1]

## 🎯 2. Power Basis Patch (Power Basis Surface)
- 정의식
- Power basis(=Monomial basis) 패치는 다음과 같이 정의됩니다.
```math
S(u,v)=\sum _{i=0}^p\sum _{j=0}^qa_{ij}\, u^iv^j
```
- 여기서
  - $a_{ij}$ = power basis coefficient
  - u,v 는 일반적으로 구간 $[u0,u1]$, $[v0,v1]$
- 특징
    - 제어점이 기하학적 의미가 없다  
        (단순히 다항식의 계수)
    - Convex hull 보장 없음
    - 미분/적분이 매우 쉬움
```math
\frac{\partial S}{\partial u}=\sum i\, a_{ij}\, u^{i-1}v^j
```
- CAD 커널 내부에서 **곡면 연산(교차, 트리밍, implicit 변환)** 에 유리

## 3. Bezier Patch 와 Power-Basis Patch 의 본질적 차이

| 항목 | Bezier Patch (Bernstein Basis) | Power-Basis Patch (Monomial Basis) |
|------|--------------------------------|------------------------------------|
| 정의 | Bernstein 다항식 Bᵢ,ₚ(u)·Bⱼ,ₚ(v) 의 선형결합 | uⁱ vʲ 의 단순한 모노미얼 조합 |
| 표면식 | S(u,v)=ΣΣ Pᵢⱼ Bᵢ,ₚ(u) Bⱼ,ₚ(v) | S(u,v)=ΣΣ bᵢⱼ uⁱ vʲ |
| 제어점 의미 | 기하학적 의미가 강함 (Convex hull, variation diminishing) | 단순 계수(coefficient), 기하학적 의미 약함 |
| 안정성 | 수치적으로 매우 안정적 | 고차에서 수치적으로 불안정 |
| 로컬성 | 제어점 하나가 전체에 영향을 주지만 Bernstein 특성으로 예측 가능 | 계수 하나가 전체 모양에 강하게 영향 (글로벌) |
| 변환 | Affine 변환 시 제어점만 변환하면 됨 | 계수 전체가 다시 계산되어야 함 |
| 재파라미터화 | 비교적 쉬움 (Bezier subdivision 등) | 매우 어려움 (모노미얼 재정렬 필요) |
| CAD 사용성 | NURBS/Bezier 표준 → 산업 표준 | 거의 사용되지 않음 (수치적 문제) |
| 장점 | 안정성, 직관성, CAD 친화적 | 수식 유도/해석적 계산에 편리 |
| 단점 | 계수 직접 해석은 어려움 | 고차에서 폭발적 수치 불안정 |

## 🎯 4. 왜 Power Basis → Bezier 변환이 필요한가?
- Bezier는 수치적으로 안정적이고 제어점이 기하학적 의미를 가지므로  
    렌더링, 메싱, 평가에 적합합니다.
- 반면 Power basis는
    - 트리밍 곡선 계산
    - 곡면 교차
    - implicitization
    - Newton iteration
- 등에서 유리합니다.
- 그래서 CAD 커널은 다음 흐름을 자주 사용합니다.
```
Power Basis Patch  →  Bezier Patch  →  평가/메싱
```

## 🎯 5. 변환 수식
- Bezier basis는 다음과 같이 Power basis의 선형 결합입니다.
```math
B_i^p(u)=\sum _{k=0}^pc_{ik}u^k
```
- 따라서
```math
S(u,v)=\sum _{i,j}P_{ij}B_i^p(u)B_j^q(v)
```
- 을 전개하면
```math
S(u,v)=\sum _{k,l}a_{kl}u^kv^l
```
- 즉,
```
Bezier 제어점 → Power basis 계수
```
- 또는
```
Power basis 계수 → Bezier 제어점
```
- 은 모두 선형 변환입니다.

## 🎯 6. 직관적 요약
- ✔ Bezier Patch
    - 모델링에 적합
    - 안정적
    - 제어점이 의미 있음
    - 렌더링/메싱에 최적
- ✔ Power Basis Patch
    - 수학적 계산에 적합
    - 미분/적분/교차가 쉬움
    - 제어점은 의미 없음
    - 수치적으로 불안정

## 🎯 7. Rust에서 이 두 패치를 어떻게 다루면 좋은가?
- 코드에는:
    - BezierSurface
    - NurbsSurface
    - Power basis 변환 함수 (on_power_to_bernstein_4d_grid)
- 이 모두 존재합니다.
- 즉, Bezier ↔ Power basis 변환은 이미 구현되어 있음.


### 1. Power basis 패치의 정의와 b[i][j]의 의미
- 우리가 쓰는 패치는 power basis 형태:
```math
S(u,v)=\sum _{i=0}^p\sum _{j=0}^pb[i][j]\, u^i\, v^j
```
- 여기서
- b[i][j] 는 3D 벡터:
```math
b[i][j]=(b[i][j].x,\; b[i][j].y,\; b[i][j].z)
```
- 따라서 각 좌표 함수는 이렇게 풀린다:
```math
\begin{aligned}x(u,v)&=\sum _{i=0}^p\sum _{j=0}^pb[i][j].x\, u^i\, v^j\\ y(u,v)&=\sum _{i=0}^p\sum _{j=0}^pb[i][j].y\, u^i\, v^j\\ z(u,v)&=\sum _{i=0}^p\sum _{j=0}^pb[i][j].z\, u^i\, v^j\end{aligned}
```

- 즉, 각 좌표마다 “2변수 다항식”의 계수 행렬을 따로 가지고 있는데,  
    그걸 하나의 3D 벡터 그리드 b[i][j]로 묶어놓은 것이라고 보면 된다.

### 2. 인덱스 규칙과 4×4 그리드 구조
- Rust 코드에서 b는 이렇게 선언:
```rust
let mut b = vec![vec![Point3D::zero(); 4]; 4]; // [u][v]
```
- 바깥 인덱스: u 방향 → i
- 안쪽 인덱스: v 방향 → j
- 즉,
```math
b[i][j]\leftrightarrow u^i\, v^j\mathrm{의\  계수\  벡터}
```
- 이를 **행렬처럼** 그리면, 보통 이렇게 볼 수 있음:
```
      j=3       j=2       j=1       j=0
   v^3 term  v^2 term  v^1 term  v^0 term
i
=3  b[3][3]  b[3][2]  b[3][1]  b[3][0]   ← u^3
=2  b[2][3]  b[2][2]  b[2][1]  b[2][0]   ← u^2
=1  b[1][3]  b[1][2]  b[1][1]  b[1][0]   ← u^1
=0  b[0][3]  b[0][2]  b[0][1]  b[0][0]   ← u^0
```

- 하지만 너가 적어준 식은 **j를 행으로** 본 형태:
```
j=3  [0][3]  [1][3]  [2][3]  [3][3]
j=2  [0][2]  [1][2]  [2][2]  [3][2]
j=1  [0][1]  [1][1]  [2][1]  [3][1]
j=0  [0][0]  [1][0]  [2][0]  [3][0]
             ↑ 여기 b[1][0].x = 1
```

- 이건 단지 **어느 축을 행으로 볼 거냐** 의 차이일 뿐이고,  
    핵심은 b[i][j]가 항상 $u^iv^j$ 의 계수라는 것만 기억하면 된다.

### 3. 예제: x(u,v) = u, y(u,v) = v, z(u,v) = u + v
- 테스트 코드:
```rust
// x(u,v) = u
b[1][0].x = 1.0;
// y(u,v) = v
b[0][1].y = 1.0;
// z(u,v) = u + v
b[1][0].z = 1.0;
b[0][1].z = 1.0;
```

- 이걸 수식으로 풀어보면:
#### 3.1. x(u,v) = u
- 원하는 함수:
```math
x(u,v)=u
```
- power basis로 쓰면:
```math
x(u,v)=1\cdot u^1\cdot v^0
```
- 즉,
    - i=1, j=0인 항의 계수만 1이고,
    - 나머지 모든 b[i][j].x는 0이어야 한다.
- 그래서:
```rust
b[1][0].x = 1.0; // u^1 v^0 항의 x 계수
```

- 이 한 줄이 의미하는 건:
```math
x(u,v)=\sum _{i,j}b[i][j].x\, u^iv^j=1\cdot u^1v^0=u
```
#### 3.2. y(u,v) = v
- 원하는 함수:
```math
y(u,v)=v=1\cdot u^0v^1
```
- 따라서:
```rust
b[0][1].y = 1.0; // u^0 v^1 항의 y 계수
```

- 즉,
```math
y(u,v)=\sum _{i,j}b[i][j].y\, u^iv^j=1\cdot u^0v^1=v
```
#### 3.3. z(u,v) = u + v
- 원하는 함수:
```math
z(u,v)=u+v=1\cdot u^1v^0+1\cdot u^0v^1
```
- 그래서:
```rust
b[1][0].z = 1.0; // u^1 v^0 항의 z 계수
b[0][1].z = 1.0; // u^0 v^1 항의 z 계수
```

- 즉,
```math
z(u,v)=\sum _{i,j}b[i][j].z\, u^iv^j=1\cdot u^1v^0+1\cdot u^0v^1=u+v
```

### 4. x, y, z 각각의 “계수 행렬”을 그림으로 표현
- 이제 이 예제에서 실제로 채워지는 계수들을 좌표별로 정리해보자. 
    (나머지 안 쓰는 항은 전부 0)
#### 4.1. x(u,v) 계수 행렬
```math
x(u,v) = Σ_i Σ_j b[i][j].x u^i v^j
```
```
j=3    0        0        0        0
j=2    0        0        0        0
j=1    0        0        0        0
j=0    0       (1)       0        0  --> u
        i=0     i=1      i=2      i=3
```

- 유일하게 b[1][0].x = 1 → $u^1v^0$ 항만 남는다.
#### 4.2. y(u,v) 계수 행렬
```math
y(u,v) = Σ_i Σ_j b[i][j].y u^i v^j
```
```
j=3    0        0        0        0   
j=2    0        0        0        0    v  
j=1   (1)       0        0        0    ^   
j=0    0        0        0        0    |
      i=0     i=1      i=2      i=3    |
```

- b[0][1].y = 1 → $u^0v^1$ 항만 남는다.
#### 4.3. z(u,v) 계수 행렬
```math
z(u,v) = Σ_i Σ_j b[i][j].z u^i v^j
```
```
j=3    0        0        0        0
j=2    0        0        0        0
j=1   (1)       0        0        0   ← u^0 v^1
j=0    0      (1)       0         0    ← u^1 v^0
        i=0     i=1      i=2      i=3
```

- b[1][0].z = 1 → $u^1v^0$
- b[0][1].z = 1 → $u^0v^1$
- 그래서:
```math
z(u,v)=1\cdot u^1v^0+1\cdot u^0v^1=u+v
```

### 5. “결국 matrix를 어떻게 채우는가?”에 대한 요약 규칙
- 기본 규칙:
```math
b[i][j]\mathrm{는\  }u^iv^j\mathrm{\  항의\  3D\  계수\  벡터}
```
- 원하는 좌표 함수가 있으면, 그걸 power basis로 분해한다.
- 예:
- $x(u,v)=2u^3-5uv^2$ 이라면
    - b[3][0].x=2  ( $u^3v^0$ )
    - b[1][2].x=-5 ( $u^1v^2$ )
    - 나머지 b[i][j].x = 0
- 각 좌표(x, y, z)에 대해 같은 방식으로 계수를 채운다.
    - b[i][j].x → x(u,v)의 계수
    - b[i][j].y → y(u,v)의 계수
    - b[i][j].z → z(u,v)의 계수
- Rust 코드에서의 인덱스 대응:
    - b[i][j]는 [u][v]
    - 즉, i는 u의 차수, j는 v의 차수에 해당.

### 6. Bezier vs Power basis 패치의 차이
요청했던 “복사 가능한 md 아스키 표” 버전으로 정리하면:
| 구분              | Power basis patch                                  | Bezier patch (Bernstein basis)                          |
|-------------------|----------------------------------------------------|---------------------------------------------------------|
| 표현식            | $S(u,v) = Σ_i Σ_j b[i][j] u^i v^j$            | $S(u,v) = Σ_i Σ_j P[i][j] B_i^p(u) B_j^p(v)$            |
| 계수/제어점 의미  | b[i][j] = $u^i v^j$ 항의 계수 벡터              | P[i][j] = 제어점 (control point)                        |
| 기하학적 직관     | 계수는 직접적인 기하학 의미가 약함               | 제어망(control net)이 곡면의 형태를 직관적으로 제어    |
| 경계 곡선         | 일반 다항식의 경계 (특별한 구조 없음)            | 경계는 Bezier 곡선, 제어점 일부와 정확히 일치          |
| 변환              | Power→Bezier: 선형 변환(행렬) 필요               | Bezier→Power: 역변환 가능하지만 잘 쓰진 않음           |
| NURBS와의 연결    | 보통 내부 표현으로 바로 쓰진 않음                | Bezier 패치는 clamped NURBS의 특수한 경우로 자연스럽게 연결 |

--- 

### 1. 기본 전제 다시 정리
- 우리가 쓰는 파워 베이시스 패치는 이렇게 생김:
```math
S(u,v)=\sum _{i=0}^3\sum _{j=0}^3b[i][j]\cdot u^i\cdot v^j
```
- 여기서 각 계수는 3D 벡터:
```math
b[i][j]=(b[i][j].x,\; b[i][j].y,\; b[i][j].z)
```
- 그래서 성분별로 보면:
```math
x(u,v)=\sum _{i,j}b[i][j].x\cdot u^i\cdot v^j
```
```math
y(u,v)=\sum _{i,j}b[i][j].y\cdot u^i\cdot v^j
```
```math
z(u,v)=\sum _{i,j}b[i][j].z\cdot u^i\cdot v^j
```

### 2. 새 예제: 살짝 더 복잡한 곡면 하나 정의해보자
- 이번엔 이런 곡면을 만들자:
- x(u,v) = 2u + v
- y(u,v) = u²
- z(u,v) = 3v²
- 모두 $(u,v)\in [0,1]\times [0,1]$ 에서 정의된다고 생각하자.

### 3. 각 성분을 파워 베이시스로 전개
#### 3.1. x(u,v) = 2u + v
```math
x(u,v)=2u+v=2\cdot u^1v^0+1\cdot u^0v^1
```
- 즉, 계수는:
    - $u^1v^0$ 항의 계수 → b[1][0].x=2
    - $u^0v^1$ 항의 계수 → b[0][1].x=1
    - 나머지 b[i][j].x는 전부 0

#### 3.2. y(u,v) = u²
```math
y(u,v)=u^2=1\cdot u^2v^0
```
- $u^2v^0$ 항의 계수 → b[2][0].y=1
- 나머지 b[i][j].y=0
#### 3.3. z(u,v) = 3v²
```math
z(u,v)=3v^2=3\cdot u^0v^2
```
- $u^0v^2$ 항의 계수 → b[0][2].z=3
- 나머지 b[i][j].z=0

### 4. 실제 코드로 $b[i][j]$ 채우기
- Rust 테스트 코드 스타일로 쓰면:
```rust
let degree = 3;
let mut b = vec![vec![Point3D::zero(); 4]; 4];

// x(u,v) = 2u + v
b[1][0].x = 2.0; // 2 * u^1 * v^0
b[0][1].x = 1.0; // 1 * u^0 * v^1

// y(u,v) = u^2
b[2][0].y = 1.0; // 1 * u^2 * v^0

// z(u,v) = 3v^2
b[0][2].z = 3.0; // 3 * u^0 * v^2

let surf = NurbsSurface::from_power_basis_patch(degree, 1.0, 1.0, b);

// 예: u=0.5, v=0.25 에서 값 확인
let p = surf.eval_point(0.5, 0.25);

// 이론값:
// x = 2u + v = 2*0.5 + 0.25 = 1.25
// y = u^2      = 0.5^2       = 0.25
// z = 3v^2     = 3*(0.25^2)  = 3*0.0625 = 0.1875

assert!((p.x - 1.25).abs() < 1e-12);
assert!((p.y - 0.25).abs() < 1e-12);
assert!((p.z - 0.1875).abs() < 1e-12);
```


### 5. b[i][j] 그리드에서의 위치 감각
- 지금 구조는 이렇게 보는 게 편함:
- 인덱스 의미: b[i][j]에서
- i = u 방향 차수 $(u^i)$
- j = v 방향 차수 $(v^j)$
- 그래서 4×4 그리드를 j 기준으로 위에서 아래로, i 기준으로 왼쪽에서 오른쪽으로 보면:
```
      i=0      i=1      i=2      i=3
j=3  b[0][3]  b[1][3]  b[2][3]  b[3][3]
j=2  b[0][2]  b[1][2]  b[2][2]  b[3][2]
j=1  b[0][1]  b[1][1]  b[2][1]  b[3][1]
j=0  b[0][0]  b[1][0]  b[2][0]  b[3][0]
```

- 이번 예제에서 우리가 채운 항들을 표시하면:
```
      i=0           i=1              i=2              i=3
j=3  (0,0,0)      (0,0,0)          (0,0,0)          (0,0,0)

j=2  (0,0,3)      (0,0,0)          (0,0,0)          (0,0,0)
     ↑ b[0][2].z=3  (z 성분만 3)

j=1  (1,0,0)      (0,0,0)          (0,0,0)          (0,0,0)
     ↑ b[0][1].x=1  (x 성분만 1)

j=0  (0,0,0)      (2,0,0)          (0,1,0)          (0,0,0)
                 ↑ b[1][0].x=2   ↑ b[2][0].y=1
```

- 이렇게 보면:
    - **x(u,v)** 는 b[1][0].x와 b[0][1].x 두 칸만 쓰고,
    - **y(u,v)** 는 b[2][0].y 한 칸만,
    - **z(u,v)** 는 b[0][2].z 한 칸만 쓰는 구조.


---
## 소스 코드
```rust
impl NurbsSurface {
    /// ------------------------------------------------------------------------
    /// - b[u][v] is POWER-BASIS coefficient grid:
    ///     S(u,v) = Σ_{i=0..p} Σ_{j=0..p} b[i][j] * u^i * v^j
    /// - output is a BEZIER patch represented as clamped NURBS
    /// - knot domains are [0..u_span] × [0..v_span]
    /// - ctrl flatten order: u + nu * v   (✅ your Idx__row_idx)
    /// ------------------------------------------------------------------------
    pub fn from_power_basis_patch(
        degree: usize,
        u_span: Real,
        v_span: Real,
        b: Vec<Vec<Point3D>>, // [u][v], (degree+1)×(degree+1)
    ) -> Self {
        assert!(degree >= 1);
        assert!(u_span.is_finite() && u_span > 0.0);
        assert!(v_span.is_finite() && v_span > 0.0);
        assert_eq!(b.len(), degree + 1);
        assert_eq!(b[0].len(), degree + 1);

        // 1) Power-basis(계수) -> Bernstein(Bezier control) with span scaling included.
        //    This matches the matrix pipeline (bezierMatrix*diag(span^i))*B*(diag(span^j)*bezierMatrix^T)
        let bez_ctrl_3d = on_power_to_bernstein_3d_grid(b.as_slice(), degree, degree, 0.0, u_span, 0.0, v_span);

        // 2) Pack into homogeneous Point4D (w=1) and flatten (u + nu*v)
        let nu = degree + 1;
        let nv = degree + 1;

        let mut ctrl = vec![Point4D::zero(); nu * nv];
        for v in 0..nv {
            for u in 0..nu {
                let p = bez_ctrl_3d[u][v];
                let idx = u + nu * v; // ✅ Idx__row_idx
                ctrl[idx] = Point4D::homogeneous(p.x, p.y, p.z, 1.0);
            }
        }

        // 3) knot vectors: [0..0, uSpan..uSpan] (p+1 times each)
        //    Use interval-based clamped generator (works for general too).
        let dom_u = Interval { t0: 0.0, t1: u_span };
        let dom_v = Interval { t0: 0.0, t1: v_span };

        let ku = on_clamped_uniform_knot_from_interval(degree, nu, &dom_u);
        let kv = on_clamped_uniform_knot_from_interval(degree, nv, &dom_v);

        NurbsSurface {
            dim: 3,
            pu: degree as Degree,
            pv: degree as Degree,
            nu: nu as Index,
            nv: nv as Index,
            ctrl,
            ku: KnotVector { knots: ku },
            kv: KnotVector { knots: kv },
            domain_u: dom_u,
            domain_v: dom_v,
        }
    }
```
```rust
    /// ------------------------------------------------------------------------
    /// If you *already have* a 4×4 Bezier control net on parameter (t,s)∈[0,1]×[0,1],
    /// and you just want the *same geometry* but with domain [0,u_span]×[0,v_span],
    /// you do NOT need any basis conversion.
    /// Just scale knot/domain. Control points stay the same.
    /// ------------------------------------------------------------------------
    pub fn from_bezier_patch_with_span(
        degree: usize,
        u_span: Real,
        v_span: Real,
        ctrl_bez: Vec<Vec<Point4D>>, // [u][v], (degree+1)×(degree+1)
    ) -> Self {
        assert!(degree >= 1);
        assert!(u_span.is_finite() && u_span > 0.0);
        assert!(v_span.is_finite() && v_span > 0.0);
        assert_eq!(ctrl_bez.len(), degree + 1);
        assert_eq!(ctrl_bez[0].len(), degree + 1);

        let nu = degree + 1;
        let nv = degree + 1;

        // flatten (u + nu*v)
        let mut ctrl = vec![Point4D::zero(); nu * nv];
        for v in 0..nv {
            for u in 0..nu {
                let idx = u + nu * v; // ✅ Idx__row_idx
                ctrl[idx] = ctrl_bez[u][v];
            }
        }

        let dom_u = Interval { t0: 0.0, t1: u_span };
        let dom_v = Interval { t0: 0.0, t1: v_span };

        let ku = on_clamped_uniform_knot_from_interval(degree, nu, &dom_u);
        let kv = on_clamped_uniform_knot_from_interval(degree, nv, &dom_v);

        NurbsSurface {
            dim: 3,
            pu: degree as Degree,
            pv: degree as Degree,
            nu: nu as Index,
            nv: nv as Index,
            ctrl,
            ku: KnotVector { knots: ku },
            kv: KnotVector { knots: kv },
            domain_u: dom_u,
            domain_v: dom_v,
        }
    }
}
```

## 테스트 코드
```rust
#[cfg(test)]
mod test {
    use nurbslib::core::geom::{Point3D, Point4D};
    use nurbslib::core::prelude::{NurbsSurface};
    #[test]
    fn test_power_basis_patch_linear_plane() {
        let degree = 3;
        let mut b = vec![vec![Point3D::zero(); 4]; 4];

        // x(u,v) = u
        b[1][0].x = 1.0;
        // y(u,v) = v
        b[0][1].y = 1.0;
        // z(u,v) = u + v
        b[1][0].z = 1.0;
        b[0][1].z = 1.0;

        let surf = NurbsSurface::from_power_basis_patch(degree, 1.0, 1.0, b);

        let p = surf.eval_point(0.5, 0.5);
        assert!((p.x - 0.5).abs() < 1e-12);
        assert!((p.y - 0.5).abs() < 1e-12);
        assert!((p.z - 1.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_bezier_patch_simple_grid() {
        let degree = 3;
        let mut ctrl = vec![vec![Point4D::zero(); 4]; 4];

        for u in 0..4 {
            for v in 0..4 {
                ctrl[u][v] = Point4D::homogeneous(u as f64, v as f64, (u + v) as f64, 1.0);
            }
        }

        let surf = NurbsSurface::from_bezier_patch_with_span(degree, 1.0, 1.0, ctrl);

        let p = surf.eval_point(0.5, 0.5);

        // 여기서는 "Bezier control net" 기반 곡면이라 결과가 power처럼 2.57이 나오지 않음.
        // 대신 단조/범위 sanity check를 하자.

        println!("p {:?}", p);

        let p00 = surf.eval_point(0.0, 0.0);
        let p10 = surf.eval_point(1.0, 0.0);
        let p01 = surf.eval_point(0.0, 1.0);
        let p11 = surf.eval_point(1.0, 1.0);

        assert!((p00.x - 0.0).abs() < 1e-12 && (p00.y - 0.0).abs() < 1e-12);
        assert!((p10.x - 3.0).abs() < 1e-12 && (p10.y - 0.0).abs() < 1e-12);
        assert!((p01.x - 0.0).abs() < 1e-12 && (p01.y - 3.0).abs() < 1e-12);
        assert!((p11.x - 3.0).abs() < 1e-12 && (p11.y - 3.0).abs() < 1e-12);

        println!("p11 : {:?}", p11);

        assert!(p.x >= 0.0 && p.x <= 3.0);
        assert!(p.y >= 0.0 && p.y <= 3.0);


        let p = surf.eval_point(0.37, 0.81);
        assert!((p.z - (p.x + p.y)).abs() < 1e-12);
    }
}
```
---

## 📌 1. 목표: Bezier Basis → Power Basis
- Bezier 곡선(차수 n)은 다음과 같이 정의됨:
```math
B_i^n(t)={n \choose i}t^i(1-t)^{n-i}
```
- Power basis는 단순히:
```math
t^0,\  t^1,\  t^2,\  \dots ,\  t^n
```
- Bezier basis를 power basis로 바꾸려면,  
    각 Bezier basis $B_i^n(t)$ 를 $t^k$ 항으로 전개해야 한다.

## 📌 2. 핵심 전개식
```math
B_i^n(t)={n \choose i}t^i(1-t)^{n-i}
```
- 여기서
```math
(1-t)^{n-i}=\sum _{m=0}^{n-i}{n-i \choose m}(-1)^mt^m
```
- 따라서 전체는:
```math
B_i^n(t)={n \choose i}\sum _{m=0}^{n-i}{n-i \choose m}(-1)^mt^{i+m}
```
- 여기서 k=i+m 로 치환하면:
```math
B_i^n(t)=\sum _{k=i}^n{n \choose i}{n-i \choose k-i}(-1)^{k-i}t^k
```



- 즉, Bezier basis의 i번째 함수는 power basis의 $t^k$ 항들의 선형 결합이다.



## 📌 3. 변환 행렬 T의 의미
- 우리는 다음을 만족하는 행렬 T를 만들고 싶다:


```math
\mathrm{power}=T\cdot \mathrm{bezier}
```

```math
\text{power\_coeffs}[k] = \sum_i T[k][i] \cdot \text{bezier\_coeffs}[i]
```


- 즉,
- 그런데 위에서 구한 전개식은:
```math
t^k=\sum _{i=k}^nP[i][k]\, B_i^n(t)
```
- k: power basis 항의 지수
- i: Bezier basis 인덱스


- 따라서 변환 행렬의 원소는:
```math
P[i][k]=\frac{{i \choose k}}{{n \choose k}},\quad i\geq k
```
- 그리고 k < i 이면 항이 존재하지 않으므로 0.

## 📌 4. Rust 코드가 하는 일
```rust
pub fn on_bezier_to_power_matrix(n: usize) -> Vec<Vec<Real>> {
    let mut t = vec![vec![0.0; n + 1]; n + 1];

    for i in 0..=n {                 // Bezier index i
        let cni = on_binomial_usize(n, i);

        for k in i..=n {             // Power index k (k >= i)
            let sign = if ((k - i) & 1) == 1 { -1.0 } else { 1.0 };

            t[k][i] = (cni * on_binomial_usize(n - i, k - i)) as Real * sign;
            // row = k (t^k), col = i (Bezier basis B_i)
        }
    }
    t
}
```

- ✔ t[k][i] = power basis의 $t^k$ 항에서 Bezier basis $B_i$ 의 계수
- ✔ 즉, 행(row)은 power basis 지수 k, 열(column)은 Bezier index i

## 📌 5. 왜 “행렬 곱”이 아닌가?
- 이 함수는 행렬을 계산하는 것이 아니라,  
    Bezier basis를 power basis로 전개한 계수를 행렬에 채워 넣는 과정이다.
- 즉,
    - 입력은 Bezier 계수(컨트롤 포인트)
    - 출력은 Power basis 계수
- 변환은:
```math
\mathbf{P_{\mathnormal{power}}}=T\cdot \mathbf{P_{\mathnormal{bezier}}}
```
- 이때 T는 위 수식으로 “직접 만들어진 행렬”이다.

## 📌 6. 예시: n = 3 일 때 T 행렬
- Bezier basis:
```math
[B_0^3,B_1^3,B_2^3,B_3^3]
```
- Power basis:
```math
[t^0,t^1,t^2,t^3]
```
- 전개하면:
```math
\begin{aligned}B_0^3&=1-3t+3t^2-t^3\\ B_1^3&=3t-6t^2+3t^3\\ B_2^3&=3t^2-3t^3\\ B_3^3&=t^3\end{aligned}
```
- 따라서 T는:
    - T (Bezier → Power), degree = 3

| k \ i |   0   |    1    |    2    |   3   |
|-------|-------|---------|---------|-------|
|   0   |   1   |   0     |   0     |   0   |
|   1   |  -3   |   3     |   0     |   0   |
|   2   |   3   |  -6     |   3     |   0   |
|   3   |  -1   |   3     |  -3     |   1   |


## 🎯 Bezier(3) → Power(3) 변환행렬 T (ASCII / Markdown)
- Bezier basis
```math
B_0^3,\; B_1^3,\; B_2^3,\; B_3^3
```
- 을 Power basis
```math
1,\; t,\; t^2,\; t^3
```
- 로 바꾸는 행렬은 다음과 같아.
- 즉,
```math
\left[ \begin{matrix}1\\ t\\ t^2\\ t^3\end{matrix}\right] =T\cdot \left[ \begin{matrix}B_0^3\\ B_1^3\\ B_2^3\\ B_3^3\end{matrix}\right] 
```
## 📌 이 표가 의미하는 것
- 각 행 k는 t^k 항의 계수,
- 각 열 i는 Bezier basis Bᵢ³(t).
- 즉,
```math
t^0 =  1·B0 + 0·B1 + 0·B2 + 0·B3
```
```math
t^1 = -3·B0 + 3·B1
```
```math
t^2 =  3·B0 - 6·B1 + 3·B2
```
```math
t^3 = -1·B0 + 3·B1 - 3·B2 + 1·B3
```

## 📘 왜 이런 값이 나오는가?
- Bezier basis:
```math
B_i^3(t)={3 \choose i}t^i(1-t)^{3-i}
```
- 이를 전개하면:
- $B_0^3=(1-t)^3=1-3t+3t^2-t^3$
- $B_1^3=3t(1-t)^2=3t-6t^2+3t^3$
- $B_2^3=3t^2(1-t)=3t^2-3t^3$
- $B_3^3=t^3$
- 이걸 행렬 형태로 정리하면 위의 T가 된다.


## 🎯 1. T와 P는 서로 역행렬 관계
- T: Bezier basis → Power basis
```math
B_i^n(t)=\sum _{k=i}^nT[k][i]\cdot t^k\mathrm{power}=T\cdot \mathrm{bezier}
```
- P: Power basis → Bezier basis
```math
t^k=\sum _{i=k}^nP[i][k]\cdot B_i^n(t)
```
```math
\mathrm{bezier}=P\cdot \mathrm{power}
```
즉,
```math
T=P^{-1},\quad P=T^{-1}
```


- ✔️ Rust 함수와 정확히 일치하는가?
- Rust 코드:
```rust
t[k][i] = C(n,i) * C(n-i, k-i) * (-1)^(k-i)
```

- degree 3이면 정확히 위 표가 나온다.

- 이 행렬을 코드가 그대로 만들어낸다.

## 📌 7. 결론
- 이 함수는 Bezier basis를 power basis로 전개한 계수를 행렬에 채워 넣는 함수이다.
- 실제로 **행렬 곱** 을 수행하는 것이 아니라,  
    전개식의 계수를 행렬 형태로 정리한 것이다.
- 이후 변환은
```math
\mathrm{power}=T\cdot \mathrm{bezier}
```
- 로 수행된다.


### 1. 우리가 이미 알고 있는 것
- ✔ Power → Bezier 변환
- 코드:
```rust
/// bezier = P · power
/// t^k = ∑_{i=k..n} C(i,k)/C(n,k) · B_i^n(t)
/// ⇒ P[i][k] = (i>=k) ? C(i,k)/C(n,k) : 0
```

- 즉,
```math
B_i^n(t)=\sum _{k=0}^iP[i][k]\, t^k
```
### 2. 반대 변환: Bezier → Power
- 우리가 원하는 것은:
```math
t^k=\sum _{i=0}^nT[k][i]\, B_i^n(t)
```
- 즉,
```
power = T · bezier
```

- 이 T 행렬을 구해야 한다.

### 3. 핵심 아이디어
- P 는 상삼각 행렬(upper triangular) 이다.
    - 행(row) = i (Bezier index)
    - 열(col) = k (power index)
    - i < k 이면 0
- 즉,
```
P =
| P00 P01 P02 ... P0n |
|  0  P11 P12 ... P1n |
|  0   0  P22 ... P2n |
| ...                 |
|  0   0   0  ... Pnn |
```

- 이런 형태.
- 따라서 $T = P⁻¹$ 이다.
- 상삼각 행렬의 역행렬도 상삼각이다.

### 4. T 를 직접 구하는 공식
- Bezier basis의 정의:
```math
B_i^n(t)={n \choose i}t^i(1-t)^{n-i}
```
- 이를 전개하면:
```math
B_i^n(t)={n \choose i}\sum _{j=0}^{n-i}{n-i \choose j}(-1)^jt^{i+j}
```
- 여기서 k=i+j 로 치환하면:
```math
B_i^n(t)=\sum _{k=i}^n{n \choose i}{n-i \choose k-i}(-1)^{k-i}t^k
```
- 따라서:
```math
T[k][i]={n \choose i}{n-i \choose k-i}(-1)^{k-i}
```
- 단, k < i 이면 0.

### 5. Rust 코드 (Bezier → Power)
```rust
/// Bezier(n) -> Power(n)
/// power = T · bezier
/// B_i^n(t) = Σ_{k=i..n} C(n,i) C(n-i, k-i) (-1)^{k-i} t^k
/// ⇒ T[k][i] = C(n,i) C(n-i, k-i) (-1)^{k-i},  k<i 이면 0
pub fn on_bezier_to_power_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut t = vec![vec![0.0; n + 1]; n + 1];

    for i in 0..=n {
        let cni = on_binomial_usize(n, i) as f64;
        for k in i..=n {
            let sign = if ((k - i) & 1) == 1 { -1.0 } else { 1.0 };
            let c = on_binomial_usize(n - i, k - i) as f64;
            t[k][i] = cni * c * sign;
        }
    }
    t
}
```

### 6. 두 행렬의 관계
- power = T · bezier
- bezier = P · power


- 따라서:
```
T = P⁻¹
P = T⁻¹
```

- 둘 다 상삼각이므로 역행렬도 상삼각.

### 7. 행렬 ASCII 형태 (복사 가능)
- ✔ P (Power → Bezier)
```math
P[i][k] = C(i,k)/C(n,k)   (i>=k)
```

- 예: n=3
```
P =
i\k |   0        1        2        3
---------------------------------------
 0  |   1        0        0        0
 1  |   1        1        0        0
 2  |   1      3/2        1        0
 3  |   1        3        3        1
```


- ✔ T (Bezier → Power)
```math
T[k][i] = C(n,i) C(n-i, k-i) (-1)^{k-i}
```

- 예: n=3
```
T =
k\i |   0        1        2        3
---------------------------------------
 0  |   1        0        0        0
 1  |  -3        3        0        0
 2  |   3       -6        3        0
 3  |  -1        3       -3        1
```


### 8. 검증: T · P = I
- n=3 에 대해 직접 곱하면 단위행렬이 나온다.

### 9. 결론
- P 는 Power → Bezier 변환
- T 는 Bezier → Power 변환
- 둘은 서로 역행렬
- 둘 다 상삼각
- 공식은 모두 조합수(binomial) 기반

## 소스 코드
```rust

/// Bezier(n) -> Power(n)
/// power = T · bezier
/// T[k][i] = ∑_{j} coeff, 여기서는
///   B_i^n(t) = ∑_{k=i..n} C(n,i) C(n-i, k-i) (-1)^{k-i} t^k
/// ⇒ T[k][i] = C(n,i) C(n-i, k-i) (-1)^{k-i}, k<i 이면 0
pub fn on_bezier_to_power_matrix(n: usize) -> Vec<Vec<Real>> {
    let mut t = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..=n {
        let cni = on_binomial_usize(n, i);
        for k in i..=n {
            let sign = if ((k - i) & 1) == 1 { -1.0 } else { 1.0 };
            t[k][i] = (cni * on_binomial_usize(n - i, k - i)) as Real * sign; // row=k (t^k), col=i (B_i)
        }
    }
    t
}

/// Power(n) -> Bezier(n)
/// bezier = P · power
/// t^k = ∑_{i=k..n} C(i,k)/C(n,k) · B_i^n(t)
/// ⇒ P[i][k] = (i>=k) ? C(i,k)/C(n,k) : 0
pub fn on_power_to_bezier_vec(n: usize) -> Vec<Vec<Real>> {
    let mut p = vec![vec![0.0; n + 1]; n + 1];
    for k in 0..=n {
        let denom = on_binomial_usize(n, k);
        for i in k..=n {
            p[i][k] = (on_binomial_usize(i, k) as f64) / (denom as f64); // row=i (B_i), col=k (t^k)
        }
    }
    p
}

pub fn on_power_basis_matrix(p: usize) -> Vec<Vec<Real>> {
    let mut m = vec![vec![0.0; p + 1]; p + 1];
    m[0][0] = 1.0;
    m[p][p] = 1.0;
    m[p][0] = if p % 2 == 1 { -1.0 } else { 1.0 };
    let mut sign = -1.0;
    for i in 1..p {
        m[i][i] = on_binomial_usize(p, i) as f64;
        m[i][0] = sign * m[i][i];
        m[p][p - i] = m[i][0];
        sign = -sign;
    }
    m
}

pub fn on_basis_power_matrix(p: usize) -> Vec<Vec<f64>> {
    let m = on_power_basis_matrix(p);
    on_invert_matrix_vec(&m).expect("Matrix inversion failed")
}
```

---

## Bezier Power Matrix

### 1. on_build_blend_coefficients() — 이항계수 Pascal 삼각형 생성기
```
blend[n][k] = C(n,k)
```

- 즉, Pascal Triangle을 만드는 함수입니다.
- 왜 필요한가?
- Bezier ↔ Power 변환식은 모두 이항계수 C(n,k) 를 사용합니다.
- 매번 factorial 계산하면 느리므로, 미리 캐싱해두는 것이 훨씬 빠릅니다.
- 동작 방식
```
n=0: 1
n=1: 1 1
n=2: 1 2 1
n=3: 1 3 3 1
...
```

- Rust 코드가 정확히 이 구조를 만듭니다.

### 2. on_get_blend_coefficient() — C(n,k) 안전 조회
```
on_get_blend_coefficient(blend, n, k) = C(n,k)
```

- 범위 밖이면 0을 반환하도록 안전하게 처리합니다.

### 3. 핵심: Bezier → Power 변환 행렬 생성
- on_to_power_matrix_from_bezier(p, blend)
- 이 함수가 가장 중요합니다.

#### 3.1 변환식 복습
- Bezier basis:
```math
B_i^p(t)=C(p,i)t^i(1-t)^{p-i}
```
- Power basis:
```math
t^k
```
- Bezier → Power 변환은:
```math
B_i^p(t)=\sum _{k=i}^pC(p,i)C(p-i,k-i)(-1)^{k-i}t^k
```
- 따라서 행렬 T는:
```
power[k] = Σ_i  T[k][i] * bezier[i]
```
```math
T[k][i] = C(p,i) C(p-i, k-i) (-1)^{k-i}
```


#### 3.2 Rust 코드가 하는 일
- (1) 첫 행과 마지막 행 고정
```rust
power.set(0,0,1.0);
power.set(p,p,1.0);
power.set(p,0, (-1)^p );
```

- 이는 다음을 의미합니다:
    - $B_0^p(t)=(1-t)^p=\sum (-1)^kC(p,k)t^k$
    - $B_p^p(t)=t^p$
- 즉, 첫/마지막 Bezier basis는 power basis로 변환할 때 구조가 단순합니다.

#### 3.3 대각선(diagonal) 채우기
```rust
for i in 1..p {
    let cpi = C(p,i);
    power.set(i,i, cpi);
    power.set(i,0, s*cpi);
    power.set(p, p-i, s*cpi);
    s = -s;
}
```

- 이 부분은 다음 항을 반영합니다:
```math
B_i^p(t)=C(p,i)t^i(1-t)^{p-i}
```
- 여기서 $t^i$ 항의 계수는 $C(p,i)$ 이므로 대각선에 들어갑니다.
- 또한 $(1−t)^{p−i}$ 전개에서 alternating sign이 나오므로 s = -s 로 번갈아 부호를 바꿉니다.

#### 3.4 내부 삼각형(lower triangular) 채우기
```rust
for col in 1..half {
    let mut sign = -1.0;
    for row in (col+1)..=mirror_row {
        let a = C(p, col);
        let b = C(p-col, row-col);
        let v = sign * a * b;
        ...
        sign = -sign;
    }
}
```

- 이 부분이 바로:
```math
C(p,i)C(p-i,k-i)(-1)^{k-i}
```
- 을 구현한 것입니다.
    - col = i
    - row = k
    - a = C(p,i)
    - b = C(p-i, k-i)
    - sign = (-1)^{k-i}
- 그리고 Bezier basis는 대칭성이 있으므로:
```
power[row][col] = v
power[mirror_row][p-row] = v
```
- 이렇게 좌우 대칭으로 채웁니다.

### 4. Power → Bezier 변환
- on_to_bezier_matrix_from_power(p, M)
- 이 함수는 위에서 만든 행렬 M의 역행렬(inverse) 을 같은 패턴으로 계산합니다.
- 즉:
```
bezier = inverse(power)

```
- 하지만 일반 행렬 역행렬이 아니라, 삼각형 구조를 이용한 빠른 역변환입니다.

#### 4.1 대각선 역수 채우기
```rust
bez.set(i,i, 1.0 / m.get(i,i));
```
- 대각선은 항상 non-zero이므로 역수가 존재합니다.

#### 4.2 내부 삼각형 역계산
```rust
num2 -= m[row][k] * bez[k][col];
v = num2 / m[row][row];
```

- 이는 전형적인 하삼각 행렬 역행렬 공식입니다.


### 5. 전체 요약
- ✔ blend[n][k] = C(n,k)
    - Pascal 삼각형 캐시.
- ✔ Bezier → Power
    ```math
    T[k][i]=C(p,i)C(p-i,k-i)(-1)^{k-i}
    ```
    - 이를 대칭성 + 삼각형 구조로 빠르게 채움.
- ✔ Power → Bezier
    - 위 행렬의 역행렬을 같은 구조로 빠르게 계산.
- ✔ 성능
    - 일반 행렬 곱셈보다 훨씬 빠르고 정확함.

### 소스 코드
```rust
/// ------------------------------------------------------------
/// Binomial(blend) cache builder
/// ------------------------------------------------------------
/// blend[n][k] = C(n,k)
pub fn on_build_blend_coefficients(max_degree: usize) -> Vec<Vec<f64>> {
    let mut blend = vec![Vec::<f64>::new(); max_degree + 1];
    for n in 0..=max_degree {
        blend[n] = vec![0.0; n + 1];
        blend[n][0] = 1.0;
        blend[n][n] = 1.0;
        for k in 1..n {
            blend[n][k] = blend[n - 1][k - 1] + blend[n - 1][k];
        }
    }
    blend
}
```
```rust
#[inline]
pub fn on_get_blend_coefficient(blend: &[Vec<f64>], degree: usize, 
    index: isize) -> f64 {
    if index < 0 || degree >= blend.len() {
        return 0.0;
    }
    let k = index as usize;
    if k > degree {
        return 0.0;
    }
    blend[degree][k]
}
```
```rust
/// ------------------------------------------------------------
/// on_to_power_matrix_from_bezier(p)
/// powerMatrix is (p+1)x(p+1)
/// ------------------------------------------------------------
pub fn on_to_power_matrix_from_bezier(p: usize, blend: &[Vec<f64>]) -> Matrix {
    let n = p + 1;
    let mut power = Matrix::with_dims(n, n);

    power.set(0, 0, 1.0);
    power.set(p, p, 1.0);

    // power[p,0] = (-1)^p
    power.set(p, 0, if (p % 2) == 0 { 1.0 } else { -1.0 });

    // main diagonal + first/last column fills
    let mut s = -1.0;
    for i in 1..p {
        let cpi = on_get_blend_coefficient(blend, p, i as isize);
        power.set(i, i, cpi);
        power.set(i, 0, s * cpi);
        power.set(p, p - i, s * cpi);
        s = -s;
    }

    // ✅ guard: p < 2이면 여기서 끝
    if p < 2 {
        return power;
    }

    // internal lower-triangular terms (and symmetric mirror)
    let half = (p + 1) / 2;
    let mut mirror_row = p - 1;

    for col in 1..half {
        let mut sign = -1.0;
        for row in (col + 1)..=mirror_row {
            let a = on_get_blend_coefficient(blend, p, col as isize);
            let b = on_get_blend_coefficient(blend, p - col, (row - col) as isize);
            let v = sign * a * b;

            power.set(row, col, v);
            power.set(mirror_row, p - row, v);

            sign = -sign;
        }
        mirror_row -= 1;
    }

    power
}
```
```rust
/// ------------------------------------------------------------
/// on_to_bezier_matrix_from_power(p, M)
/// - assumes M is BezierToPowerMatrix(p)
/// - returns bezierMatrix = inverse(M) using the same triangular fill
/// ------------------------------------------------------------
pub fn on_to_bezier_matrix_from_power(p: usize, m: &Matrix) -> Matrix {
    let n = p + 1;
    let mut bez = Matrix::with_dims(n, n);

    for i in 0..=p {
        bez.set(i, 0, 1.0);
        bez.set(p, i, 1.0);

        let diag = m.get(i, i);
        bez.set(i, i, 1.0 / diag);
    }

    // ✅ guard: p < 2이면 여기서 끝
    if p < 2 {
        return bez;
    }

    let half = (p + 1) / 2;
    let mut mirror_row = p - 1;

    for col in 1..half {
        for row in (col + 1)..=mirror_row {
            let mut num2 = 0.0;
            for k in col..row {
                num2 -= m.get(row, k) * bez.get(k, col);
            }
            let v = num2 / m.get(row, row);

            bez.set(row, col, v);
            bez.set(mirror_row, p - row, v);
        }
        mirror_row -= 1;
    }

    bez
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::basis::{on_build_blend_coefficients,
        on_to_bezier_matrix_from_power, on_to_power_matrix_from_bezier};
    use nurbslib::core::matrix::Matrix;

    fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
        Matrix::get_multiply(a, b).expect("multiply failed")
    }

    fn assert_identity(m: &Matrix, tol: f64) {
        let n = m.rows();
        assert_eq!(n, m.cols());
        for r in 0..n {
            for c in 0..n {
                let v = m.get(r, c);
                let e = if r == c { 1.0 } else { 0.0 };
                assert!(
                    (v - e).abs() <= tol,
                    "not identity at ({},{}) = {}, expected {}",
                    r, c, v, e
                );
            }
        }
    }
```
```rust
    #[test]
    fn test_bezier_power_matrices_inverse_property_p0_to_p10() {
        let tol = 1e-12;
        for p in 0..=10 {
            let blend = on_build_blend_coefficients(p);
            let m = on_to_power_matrix_from_bezier(p, &blend);
            let b = on_to_bezier_matrix_from_power(p, &m);

            // M * B == I
            let mb = mat_mul(&m, &b);
            assert_identity(&mb, tol);

            // B * M == I
            let bm = mat_mul(&b, &m);
            assert_identity(&bm, tol);
        }
    }
```
```rust
    #[test]
    fn test_bezier_to_power_known_degree3_row0_col0_signs() {
        // 작은 스모크 테스트: p=3에서 몇 개 항의 부호/대칭이 C#과 일치하는지 확인
        let p = 3;
        let blend = on_build_blend_coefficients(p);
        let m = on_to_power_matrix_from_bezier(p, &blend);

        // diag: C(3,1)=3, C(3,2)=3
        assert!((m.get(1,1) - 3.0).abs() < 1e-12);
        assert!((m.get(2,2) - 3.0).abs() < 1e-12);

        // first col alternating: (-1)^i * C(p,i)
        assert!((m.get(1,0) + 3.0).abs() < 1e-12); // i=1 => -3
        assert!((m.get(2,0) - 3.0).abs() < 1e-12); // i=2 => +3
        assert!((m.get(3,0) + 1.0).abs() < 1e-12); // i=3 => -1
    }
}
```

---
