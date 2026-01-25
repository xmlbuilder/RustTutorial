# Bezier Degree Elevation
## 1. 시작점: Bezier 곡선의 두 가지 표현
- 원래 p차 Bezier 곡선:
```math
C(u)=\sum _{j=0}^pP_jB_j^{(p)}(u)
```
- 여기서
- $B_j^{(p)}(u)$ 는 p차 Bernstein basis:
```math
B_j^{(p)}(u)={p \choose j}u^j(1-u)^{p-j}
```
- 이제 같은 곡선을 더 높은 차수 q=p+t의 Bezier로도 표현하고 싶다:
```math
C(u)=\sum _{i=0}^q\tilde {P}_iB_i^{(q)}(u)
```
- 여기서 $\tilde {P}_i$ 가 새 control point들이고,
- 우리가 만든 행렬이 바로
```math
\tilde {P}_i=\sum _{j=0}^pE_{i,j}P_j
```
- 에서의 $E_{i,j}$ 다.

## 2. 핵심 아이디어: 같은 곡선이므로 basis끼리 선형 결합 관계가 있다
- 같은 함수 C(u)를 두 방식으로 썼으니,  
    basis끼리 이런 관계가 성립해야 한다:
```math
B_j^{(p)}(u)=\sum _{i=0}^qE_{i,j}B_i^{(q)}(u)
```
- 즉, 낮은 차수 Bernstein 하나가 높은 차수 Bernstein들의 선형 결합으로 표현된다.
- 이 $E_{i,j}$ 가 바로 degree elevation matrix의 원소다.

## 3. Bernstein basis를 전개해서 계수 비교하기
- 양쪽을 다 다항식 형태로 전개해보자.
- 왼쪽:
```math
B_j^{(p)}(u)={p \choose j}u^j(1-u)^{p-j}
```
- 오른쪽:
```math
\sum _{i=0}^qE_{i,j}B_i^{(q)}(u)=\sum _{i=0}^qE_{i,j}{q \choose i}u^i(1-u)^{q-i}
```
- 이 두 표현이 모든 u에 대해 같아야 하므로,
- 결국 각 단항 $u^k(1-u)^{q-k}$ 에 대한 계수가 일치해야 한다.
- 하지만 이걸 직접 계수 비교로 밀어붙이면 꽤 지저분해진다.

## 4. 조합론적 해석: Bernstein basis의 degree elevation 정리
- Bezier/ Bernstein 이론에서 잘 알려진 정리 하나가 있다:
```math
B_j^{(p)}(u)=\sum _{i=j}^{j+t}\frac{{p \choose j}{t \choose i-j}}{{p+t \choose i}}B_i^{(p+t)}(u)
```
- 여기서 q=p+t라 두면:
```math
B_j^{(p)}(u)=\sum _{i=j}^{j+t}\frac{{p \choose j}{t \choose i-j}}{{q \choose i}}B_i^{(q)}(u)
```
- 이게 바로 코드에 있는 공식:
```math
E_{i,j}=\frac{{p \choose j}{t \choose i-j}}{{q \choose i}}
```
- 이고,
- i의 유효 범위가
```math
j\leq i\leq j+t
```
- 인데, p와 q 경계까지 고려하면 코드에서 쓰는 것처럼
- $j\in [\max (0,i-t),\min (p,i)]$
- 이렇게 정리된다.

## 5. 이 공식이 왜 맞는지 직관적인 조합론적 설명
- 핵심은 이거다:
- p차 Bernstein $B_j^{(p)}$ 는 “j번 성공, p-j번 실패”인 이항 분포 형태
- q = p+t 차로 올리면, “총 q번 중 i번 성공”인 형태로 다시 써야 함
- 기존의 j번 성공을 유지하면서, 추가 t번 중에서 (i-j)번을 성공으로 채우는 경우의 수가 ${t \choose i-j}$
- 전체 q번 중 i번 성공인 경우의 수는 ${q \choose i}$
- 기존 계수 ${p \choose j}$ 까지 곱하면
- **j에서 i로 올라가는 weight** 가
```math
\frac{{p \choose j}{t \choose i-j}}{{q \choose i}}
```
- 로 자연스럽게 나온다.
- 즉,
- **p번 중 j번 성공** 을  
    **p+t번 중 i번 성공** 으로 재분배하는 조합론적 weight가 바로 $E_{i,j}$ 다.

## 6. 그래서 control point 변환이 왜 이 행렬로 되는가
- 원래 곡선:
```math
C(u)=\sum _{j=0}^pP_jB_j^{(p)}(u)
```
- 각 $B_j^{(p)}$ 를 위 공식으로 치환:
```math
B_j^{(p)}(u)=\sum _{i=0}^qE_{i,j}B_i^{(q)}(u)
```
- 대입하면:
```math
C(u)=\sum _{j=0}^pP_j\left( \sum _{i=0}^qE_{i,j}B_i^{(q)}(u)\right) =\sum _{i=0}^q\left( \sum _{j=0}^pE_{i,j}P_j\right) B_i^{(q)}(u)
```
- 따라서 새 control point는:
```math
\tilde {P}_i=\sum _{j=0}^pE_{i,j}P_j
```
- 이게 바로 네 함수가 만들어주는 행렬 dm의 의미다.

## 7. 코드와 수식의 1:1 대응
- bin[p][j] → ${p \choose j}$
- bin[t][i-j] → ${t \choose i-j}$
- bin[q][i] → ${q \choose i}$
- dm[i][j] = inv * bin[p][j] * bin[t][i-j]

```math
E_{i,j}=\frac{{p \choose j}{t \choose i-j}}{{q \choose i}}
```
- 그리고 대칭성:
```math
E_{i,j}=E_{q-i,\, p-j}
```
- 코드의:
```math
dm[i][j] = dm[q - i][p - j];
```

- 이게 그대로다.

- 한 줄로 정리하면
- 이 행렬은  
    **p차 Bernstein basis를 q=p+t차 Bernstein basis의 선형 결합으로 표현할 때의 계수** 이고,  
- 그 계수는 조합론적으로
```math
E_{i,j}=\frac{{p \choose j}{t \choose i-j}}{{q \choose i}}
```
- 로 유도된다.
- 그래서 이 행렬을 control point에 곱하면 곡선 형상을 유지한 채로 차수만 올릴 수 있다.

- 함수는 Bezier 곡선의 차수 상승(degree elevation) 을 수행할 때 필요한  
    선형 변환 행렬(Elevation Matrix) 을 만드는 함수.
- 즉, p차 Bezier 곡선의 control point들을 q = p+t 차로 올릴 때,
    새로운 control point들을 계산하기 위한 계수 행렬(dm) 을 만드는 역할을 한다.
- 아주 중요한 함수고, NURBS/Bezier 커널에서 자주 쓰이는 핵심 수학 도구.

## 🎯 이 함수가 정확히 하는 일
- ✔ 입력
    - p: 원래 Bezier 곡선의 차수
    - t: 증가시킬 차수
    - q = p + t: 새 차수
- ✔ 출력
    - 크기 (q+1) × (p+1) 의 행렬 dm
    - 이 행렬을 원래 control point 벡터에 곱하면  
        새로운 control point 벡터가 나온다.
- 즉:
```math
\mathbf{P^{\mathnormal{(q)}}}=E\cdot \mathbf{P^{\mathnormal{(p)}}}
```
- 여기서 E가 바로 이 함수가 만드는 행렬이다.

## 🎯 왜 이런 행렬이 필요한가?
- Bezier 곡선의 차수 상승은 다음을 만족해야 한다:
    - 곡선의 형상은 그대로 유지
    - control point 개수만 증가
    - basis function이 바뀌므로 control point도 재계산 필요
- 차수 상승 공식은 다음과 같다:
```math
E_{i,j}=\frac{{p \choose j}{t \choose i-j}}{{q \choose i}}
```
- 이 행렬을 이용하면:
    - p차 control point → q차 control point
    - 선형 변환이므로 매우 빠르고 안정적
    - CAD 커널에서 필수 기능

## 🎯 코드가 하는 일 요약
### 1) Pascal triangle로 binomial coefficient 준비
```rust
let bin = on_pascal_triangle_f64(q);
```

### 2) 행렬 크기 할당
```
(q+1) × (p+1)
```
### 3) 양 끝값은 항상 1
```rust
dm[0][0] = 1.0;
dm[q][p] = 1.0;
```

### 4) 절반만 계산하고 나머지는 대칭으로 채움
- Bezier degree elevation matrix는 다음 대칭성을 가진다:
```math
E_{i,j}=E_{q-i,\, p-j}
```
- 그래서:
    - i = 0..r (r = q/2) 까지만 직접 계산
    - 나머지는 대칭으로 복사

## 🎯 이 행렬을 어디에 쓰는가?
- ✔ 1) Bezier 곡선 차수 상승
- 예:
    - p=3 → q=5
    - control point 4개 → 6개
- 새 control point는:
```math
P_i^{(q)}=\sum _{j=0}^pE_{i,j}P_j^{(p)}
```
- ✔ 2) NURBS 곡선 차수 상승
- NURBS는 Bezier 조각으로 분해한 뒤 각 조각에 이 행렬을 적용한다.
- ✔ 3) 곡면 차수 상승 (tensor product)
    - U 방향, V 방향 각각 이 행렬을 사용한다.
- ✔ 4) Knot insertion과도 관련
    - Bezier degree elevation은 knot insertion의 핵심 구성 요소다.

## 🎯 한 줄 요약
- 이 함수는 p차 Bezier 곡선을 q=p+t 차로 올릴 때
    control point 변환에 필요한 **차수 상승 행렬(Elevation Matrix)** 을 계산한다.


```rust
/// - original degree = p
/// - increment = t  (new degree q = p + t)
/// - returns matrix `dm` of size (q+1) x (p+1)
///
/// Formula:
///   E_{i,j} = C(p, j) * C(t, i-j) / C(q, i)
/// with bounds j in [max(0, i-t) .. min(p, i)].
///
/// C 구현의 대칭 최적화(절반 계산 후 RM[i][j] = RM[q-i][p-j])를 그대로 유지.
/// binomial은 `on_pascal_triangle_f64`를 이용.
pub fn on_bezier_curve_degree_elevation_matrix(p: usize, t: usize) -> Vec<Vec<Real>> {
    let q = p + t;

    // Allocate dm[q+1][p+1] filled with 0
    let mut dm = vec![vec![0.0 as Real; p + 1]; q + 1];

    // Binomial coefficients up to q
    let bin = on_pascal_triangle_f64(q);

    // Base corners
    dm[0][0] = 1.0;
    dm[q][p] = 1.0;

    if q == 0 {
        return dm;
    }

    // r = q/2 (integer)
    let r = q / 2;

    // First half (i=1..r)
    for i in 1..=r {
        let inv = 1.0 / bin[q][i];

        let k = if i > t { i - t } else { 0 };
        let l = if i > p { p } else { i };

        for j in k..=l {
            // inv * C(p,j) * C(t, i-j)
            dm[i][j] = inv * bin[p][j] * bin[t][i - j];
        }
    }

    // Second half via symmetry (i=r+1..q-1)
    // C code uses i<q (exclude q)
    for i in (r + 1)..q {
        let k = if i > t { i - t } else { 0 };
        let l = if i > p { p } else { i };

        for j in k..=l {
            dm[i][j] = dm[q - i][p - j];
        }
    }

    dm
}
```
## 테스트 코드

- 두 테스트는 Bezier 차수 상승 행렬(Elevation Matrix) 이
    수학적으로 “정상적인” 성질을 만족하는지 검증하는 매우 중요한 테스트.

### 1️⃣ test_degree_elevation_matrix_identity_when_t0
- ✔ 목적
- t = 0 (차수 증가 없음) 일 때,  
    degree elevation matrix는 항등행렬(identity matrix) 이어야 한다는 것을 확인.
- ✔ 왜 그래야 하나?
- 차수를 0만큼 올린다는 것은:
    - 원래 p차 → 새 차수도 p차
    - control point가 그대로여야 함
    - 즉, 변환 행렬이 항등행렬이어야 함
- 수학적으로도:
```math
E_{i,j}=\frac{{p \choose j}{0 \choose i-j}}{{p \choose i}}
```
- 여기서 ${0 \choose i-j}$ 는
    - i=j일 때만 1
    - 나머지는 0
- 따라서:
```math
E_{i,j}=\delta _{ij}
```
- 즉, 항등행렬.
- ✔ 테스트 내용 해석
```rust
let p = 5;
let dm = on_bezier_curve_degree_elevation_matrix(p, 0);
```

- p=5, t=0 → 6×6 행렬 생성
```rust
assert_eq!(dm.len(), p + 1);
assert_eq!(dm[0].len(), p + 1);
```

- 행렬 크기가 6×6인지 확인
```rust
for i in 0..=p {
    for j in 0..=p {
        let expect = if i == j { 1.0 } else { 0.0 };
        assert!((dm[i][j] - expect).abs() < 1e-12);
    }
}
```

- 모든 원소가 항등행렬인지 확인
    - 즉, i=j일 때만 1, 나머지는 0
- ✔ 결론
    - 차수 상승이 없을 때 행렬이 항등행렬인지 검증하는 테스트.

### 2️⃣ test_degree_elevation_matrix_row_sum_is_one
- ✔ 목적
    - degree elevation matrix의 각 행(row)의 합이 1인지 확인.
- ✔ 왜 row sum = 1이어야 하나?
- 행렬의 각 행은 다음을 의미한다:
```math
\tilde {P}_i=\sum _{j=0}^pE_{i,j}P_j
```
- 즉, 새 control point $\tilde {P}_i$ 는
    - 원래 control point들의 convex combination(볼록 조합) 이어야 한다.
- 볼록 조합의 조건:
    - 모든 계수 $E_{i,j}\geq 0$
    - 각 행의 합이 1
- 이 조건이 만족되면:
    - 곡선의 형상이 절대 변하지 않음
    - control point가 convex hull 안에 유지됨
    - Bezier 곡선의 기본 성질이 보존됨
- 그래서 row sum = 1은 매우 중요한 성질이다.
- ✔ 테스트 내용 해석
```rust
let p = 4;
let t = 3;
let dm = on_bezier_curve_degree_elevation_matrix(p, t);
let q = p + t;
```

- p=4 → 원래 5 control point
- t=3 → 새 차수 q=7 → 8 control point
- 행렬 크기 8×5
```rust
for i in 0..=q {
    let s: f64 = dm[i].iter().copied().sum();
    assert!((s - 1.0).abs() < 1e-12, "row {} sum {}", i, s);
}
```
- 각 행의 합을 계산
- 1.0과 거의 동일해야 함
- 오차 허용 1e-12
- ✔ 결론
    - degree elevation matrix가 convex combination을 유지하는지 검증하는 테스트.

## 🎯 전체 요약

| Test Name                                   | What It Checks                     | Why It Matters                               |
|---------------------------------------------|------------------------------------|-----------------------------------------------|
| test_degree_elevation_matrix_identity_when_t0 | Degree elevation with t=0 yields identity matrix | No degree change → control points must remain identical |
| test_degree_elevation_matrix_row_sum_is_one | Each row of elevation matrix sums to 1 | Ensures convex combination → curve shape is preserved |


- 이 두 테스트는
    - Bezier 차수 상승 알고리즘이 수학적으로 올바르게 구현되었는지  
        확인하는 핵심 테스트야.

# elevate_degree / reduce_degree / re_parameterize

- 핵심은:
    - elevate_degree → 정확한 차수 상승 (형상 불변)
    - reduce_degree → 근사적인 차수 감소 (형상 근사)
    - re_parameterize → Bezier 곡선의 합성 C(f(u))

## 1. elevate_degree 
    - degree elevation 행렬을 control point에 적용하는 방식
```rust
pub fn elevate_degree(&self, t: usize) -> BezierCurve {
    let mat = on_degree_elevation_matrix(self.degree, t);
    let mut n_ctrl = vec![Point4D::zero(); self.degree + t + 1];
    for i in 0..=self.degree + t {
        for j in 0..=self.degree {
            n_ctrl[i].x += mat[i][j] * self.ctrl[j].x;
            n_ctrl[i].y += mat[i][j] * self.ctrl[j].y;
            n_ctrl[i].z += mat[i][j] * self.ctrl[j].z;
            n_ctrl[i].w += mat[i][j] * self.ctrl[j].w;
        }
    }
    BezierCurve {
        dim: 3,
        degree: self.degree + t,
        ctrl: n_ctrl,
    }
}
```

### 1-1. 수학적 배경
- 원래 p차 Bezier 곡선:
```math
C(u)=\sum _{j=0}^pP_jB_j^{(p)}(u)
```
- 차수를 q=p+t로 올린 뒤에도 같은 곡선을 유지하려면:
```math
C(u)=\sum _{i=0}^q\tilde {P}_iB_i^{(q)}(u)
```
- 여기서 $\tilde {P}_i$ 는 새 control point.
- Bernstein basis 사이에는 다음 관계가 있다:
```math
B_j^{(p)}(u)=\sum _{i=0}^qE_{i,j}B_i^{(q)}(u)
```
- 그래서:
```math
C(u)=\sum _{j=0}^pP_jB_j^{(p)}(u)=\sum _{j=0}^pP_j\left( \sum _{i=0}^qE_{i,j}B_i^{(q)}(u)\right) =\sum _{i=0}^q\left( \sum _{j=0}^pE_{i,j}P_j\right) B_i^{(q)}(u)
```
- 따라서:
```math
\tilde {P}_i=\sum _{j=0}^pE_{i,j}P_j
```
- 여기서 $E_{i,j}$ 가 바로 on_degree_elevation_matrix(p, t)가  
    만들어주는 mat[i][j].
- 공식은:
```math
E_{i,j}=\frac{{p \choose j}{t \choose i-j}}{{p+t \choose i}}
```
- (유효한 j 범위는 $j\in [\max (0,i-t),\min (p,i)]$)
### 1-2. 코드와 수식의 1:1 대응
- mat[i][j] → $E_{i,j}$
- self.ctrl[j] → 원래 control point $P_j=(x_j,y_j,z_j,w_j)$
- n_ctrl[i] → 새 control point $\tilde {P}_i$
코드:
```rust
n_ctrl[i].x += mat[i][j] * self.ctrl[j].x;
...
```

- 수식:
```math
\tilde {P}_i.x=\sum _{j=0}^pE_{i,j}P_j.x
```
- 4D (x,y,z,w)를 모두 같은 계수로 선형 결합하니까,  
    rational Bezier (동차 좌표)에서도 형상이 정확히 보존된다.

## 2. reduce_degree — 차수 감소 (근사)
```rust
pub fn reduce_degree(&mut self, target_deg: Degree) -> Vec<Point4D> {
    let p = (self.ctrl.len() - 1) as i32;
    if target_deg >= p as u16 {
        return self.ctrl.to_vec();
    }

    let q = target_deg as usize;
    let mut new_ctrl = vec![
        Point4D { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        q + 1
    ];

    for i in 0..=q {
        let t = i as Real / q as Real;
        new_ctrl[i] = self.evaluate_cpoint_rational(t);
    }
    new_ctrl
}
```

### 2-1. 수학적 의미
- 이건 정확한 degree reduction이 아니라, 샘플링 기반 근사야.
    - 원래 곡선 C(u)를 가지고 있고
    - 목표 차수 q를 정한 뒤
    - $u_i=\frac{i}{q}$ 에서의 점 C(u_i)를 그대로 새 control point로 사용
- 즉:
```math
\tilde {P}_i\approx C\left( \frac{i}{q}\right) 
```
- 이건 **새 Bezier control point들이 곡선 위의 점을 직접 잡는 방식** 이라서:
- 형상은 근사
    - convex hull 성질은 유지되지 않을 수도 있음
- 하지만 구현이 매우 간단하고 직관적
- 정확한 degree reduction은 보통 최소제곱(least-squares) 문제로 풀어야 하고,  
    그건 elevation matrix의 역문제에 가까운 형태가 된다.
- 여기 코드는 **간단한 practical 버전** 이라고 보면 돼.

## 3. reduce_degree_curve — 위 결과를 곡선으로 감싸기
```rust
pub fn reduce_degree_curve(&mut self, target_deg: Degree) -> Self {
    Self {
        dim: 3,
        degree: target_deg as usize,
        ctrl: self.reduce_degree(target_deg),
    }
}
```

- 그냥:
- degree만 낮춘 새 BezierCurve를 만들어주는 래퍼.

## 4. re_parameterize — Bezier 곡선의 재파라미터화 C(f(u))
```rust
pub fn re_parameterize(&self, func: &BezierFunction) -> BezierCurve {
    let n = self.degree;
    let mut result = vec![Point4D::zero(); func.degree + n + 1];

    for i in 0..=n {
        let bi = BezierFunction {
            degree: n,
            coeffs: (0..=n).map(|j| if j == i { 1.0 } else { 0.0 }).collect(),
        };
        let bi_f = bi.multiply(func); // B_i^n(f(u)) as BezierFunction
        for (j, coeff) in bi_f.coeffs.iter().enumerate() {
            result[j].x += coeff * self.ctrl[i].x;
            result[j].y += coeff * self.ctrl[i].y;
            result[j].z += coeff * self.ctrl[i].z;
            result[j].w += coeff * self.ctrl[i].w;
        }
    }
    BezierCurve {
        dim: 3,
        degree: func.degree + n,
        ctrl: result,
    }
}
```

### 4-1. 수학적 의미
- 원래 곡선:
```math
C(u)=\sum _{i=0}^nP_iB_i^{(n)}(u)
```
- 재파라미터화 함수 f(u) (BezierFunction):
```math
f(u)=\sum _{k=0}^ma_kB_k^{(m)}(u)
```
- 만들고 싶은 건:
```math
\tilde {C}(u)=C(f(u))=\sum _{i=0}^nP_iB_i^{(n)}(f(u))
```
- 여기서 각 $B_i^{(n)}(f(u))$ 는 다시 Bezier basis의 선형 결합으로 쓸 수 있다:
```math
B_i^{(n)}(f(u))=\sum _jc_{i,j}B_j^{(n+m)}(u)
```
- 코드에서 하는 게:
```rust
let bi = BezierFunction { degree: n, coeffs: ... } // B_i^n(u)
let bi_f = bi.multiply(func); // B_i^n(f(u)) as BezierFunction
```

- 즉, bi_f.coeffs[j] = c_{i,j}.
- 그러면:
```math
\tilde {C}(u)=\sum _{i=0}^nP_i\left( \sum _jc_{i,j}B_j^{(n+m)}(u)\right) =\sum _j\left( \sum _{i=0}^nc_{i,j}P_i\right) B_j^{(n+m)}(u)
```
- 따라서 새 control point는:
```math
\tilde {P}_j=\sum _{i=0}^nc_{i,j}P_i
```
- 코드에서:
```rust
for (j, coeff) in bi_f.coeffs.iter().enumerate() {
    result[j].x += coeff * self.ctrl[i].x;
    ...
}
```

- 여기서 coeff가 $c_{i,j}$ 역할을 하고,  
    $result[j]$ 가 $\tilde {P}_j$ 가 된다.
### 4-2. 이게 의미하는 것
- re_parameterize는 Bezier 곡선의 파라미터를 Bezier 함수로 치환하는 일반적인 재파라미터화
- 즉, $u\mapsto f(u)$ 를 곡선에 적용한 것
- degree는 n + func.degree로 증가 (합성의 결과)

## 5. 전체적으로 보면
- elevate_degree
    - 정확한 선형 변환
    - elevation matrix $E_{i,j}$ 로 control point를 선형 결합
    - 형상 완전 보존
- reduce_degree
    - 샘플링 기반 근사
    - u_i=i/q에서의 점을 새 control point로 사용
    - 형상 근사, 간단하지만 정확한 reduction은 아님
- re_parameterize
    - Bezier 곡선의 합성 C(f(u))
    - basis 함수 $B_i^{(n)}(f(u))$ 를 다시 Bezier basis로 전개
    - 그 계수로 control point를 다시 선형 결합
- 이 세 개는:
- control point를 어떻게 선형 결합하면  
    수식에서 원하는 변환(차수 상승, 재파라미터화, 근사)을 구현할 수 있는가
    를 그대로 코드로 옮긴 것임.
---
## 소스 코드
```rust
/// Elevate the degree of a Bezier curve
pub fn elevate_degree(&self, t: usize) -> BezierCurve {
    let mat = on_degree_elevation_matrix(self.degree, t);
    let mut n_ctrl = vec![Point4D::zero(); self.degree + t + 1];
    for i in 0..=self.degree + t {
        for j in 0..=self.degree {
            n_ctrl[i].x += mat[i][j] * self.ctrl[j].x;
            n_ctrl[i].y += mat[i][j] * self.ctrl[j].y;
            n_ctrl[i].z += mat[i][j] * self.ctrl[j].z;
            n_ctrl[i].w += mat[i][j] * self.ctrl[j].w;
        }
    }
    BezierCurve {
        dim: 3,
        degree: self.degree + t,
        ctrl: n_ctrl,
    }
}

pub fn reduce_degree(&mut self, target_deg: Degree) -> Vec<Point4D> {
    let p = (self.ctrl.len() - 1) as i32;
    if target_deg >= p as u16 {
        return self.ctrl.to_vec();
    }

    let q = target_deg as usize;
    let mut new_ctrl = vec![
        Point4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        };
        q + 1
    ];
    // Simple proportional interpolation basis (can also be done with least-squares)
    for i in 0..=q {
        let t = i as Real / q as Real;
        // Obtain the point at parameter (t) using De Casteljau’s algorithm,
        // and use it directly as a control point
        new_ctrl[i] = self.evaluate_cpoint_rational(t);
    }
    new_ctrl
}

pub fn reduce_degree_curve(&mut self, target_deg: Degree) -> Self {
    Self {
        dim: 3,
        degree: target_deg as usize,
        ctrl: self.reduce_degree(target_deg),
    }
}

pub fn re_parameterize(&self, func: &BezierFunction) -> BezierCurve {
    let n = self.degree;
    let mut res = vec![Point4D::zero(); func.degree + n + 1];

    for i in 0..=n {
        let bi = BezierFunction {
            degree: n,
            coeffs: (0..=n).map(|j| if j == i { 1.0 } else { 0.0 }).collect(),
        };
        let bi_f = bi.multiply(func); // B_i^n(f(u)) as BezierFunction
        for (j, cef) in bi_f.coeffs.iter().enumerate() {
            res[j].x += cef * self.ctrl[i].x;
            res[j].y += cef * self.ctrl[i].y;
            res[j].z += cef * self.ctrl[i].z;
            res[j].w += cef * self.ctrl[i].w;
        }
    }
    BezierCurve {
        dim: 3,
        degree: func.degree + n,
        ctrl: res,
    }
}
```
---
