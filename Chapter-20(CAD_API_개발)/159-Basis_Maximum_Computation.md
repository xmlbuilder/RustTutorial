# 📘 B‑Spline Basis Maximum Computation: Valid & Invalid Cases
- (NURBS Basis Function Maximum – Newton Iteration Convergence Guide)

## 1. 개요
- on_compute_basis_maximum() 함수는 B‑spline basis function $N_{i,p}(u)$ 의 최대값과  
  그 위치를 찾기 위해 Newton iteration을 수행한다.
- 그러나 모든 basis function이 내부 최대값을 가지는 것은 아니며, 수학적으로 최대값이 존재하지 않는 경우  
  Newton iteration은 절대로 수렴할 수 없다.
- 따라서 이 함수는 특정 조건에서만 정상적으로 동작하며, 조건을 벗어나면  
  **Newton iteration did not converge** 오류가 발생한다.
- 이 문서는 **수렴하는 케이스(되는 경우)** 와 **수렴하지 않는 케이스(안 되는 경우)** 를 명확히 정리한다.

## Basis Functions
![Basis Functions](/image/basis.png)


## 2. 되는 케이스 (Newton iteration이 정상적으로 수렴하는 경우)
- ✔ Case 1. 충분한 control point 수를 가진 `B‑spline (n ≥ p)`
- degree p 에 대해 `control point` 수가 최소 `p+1` 이상일 때
- 즉, basis index 범위가 정상적으로 형성될 때
- 예시
    - degree p = 2
    - knots = [0,0,0,1,1,1]
    - control points = 3개 → n = 2 ≥ p
- 이 경우 basis function은 **종 모양(bell-shaped)** 이며 내부에 최대값이 존재한다.
- Newton iteration은 항상 수렴한다.

- ✔ Case 2. 내부 knot가 존재하여 basis function이 완전한 형태를 가질 때
- 예:
    - knots = [0,0,0.3,0.7,1,1]
    - degree = 2
- 이 경우 각 basis function은 다음을 만족한다:
    - support 구간이 충분히 넓고
    - 내부에서 1차 미분이 0이 되는 지점이 존재 따라서 Newton iteration이 정상적으로 작동한다.

- ✔ Case 3. 특수 케이스: p‑fold interior knot (full multiplicity)
- 내부 knot가 degree만큼 반복되면 basis function은 해당 knot에서 최대값 1을 가진다.
- 예:
    - knots = [0,0,0.5,0.5,0.5,1,1]
    - degree = 2
- 이 경우 Newton iteration 없이도 즉시 최대값을 결정할 수 있다.

## 3. 안 되는 케이스 (Newton iteration이 절대로 수렴하지 않는 경우)
- ❌ Case 1. control point 수가 부족한 경우 (n < p)
- 이 경우 basis function은 정상적인 degree p 형태를 갖지 못한다.
- 예:
    - knots = [0,0,0.5,1,1]
    - degree = 2

- 여기서:
    - m = 4
    - n = m - p - 1 = 4 - 2 - 1 = 1
    - n < p (1 < 2)
- 이 경우 basis function은 bell shape이 아니라 기울어진 선형 함수가 된다.
    - 즉, 내부에서 미분이 0이 되는 지점이 없다.
    -  Newton iteration은 절대로 수렴할 수 없다.
    -  **Newton iteration did not converge** 오류가 정상 동작이다.

- ❌ Case 2. basis function이 plateau(평탄 구간)를 가지는 경우
- 예:
    - knots = [0,0,0,0.5,1,1,1]
    - degree = 2
- 특정 basis는 내부에서 완전히 평평한 plateau를 가질 수 있다.
- 이 경우 미분이 0이 되는 구간이 연속적이므로 Newton iteration이 불안정해진다.

- ❌ Case 3. support 구간이 너무 짧아 미분이 0이 되는 지점이 존재하지 않는 경우
- 예:
    - knots = [0,0,0.5,1,1]
    - degree = 2
    - i = 1
- 이 경우 basis function은 삼각형 형태가 아니라 한쪽으로 치우친 선형 함수가 된다.
- 내부 최대값이 존재하지 않으므로 Newton iteration은 실패한다.

- ❌ Case 4. knot vector가 비정상적이거나 domain이 잘못된 경우
    - knot vector가 비내림차순이 아님
    - domain이 잘못 설정됨
    - span이 유효하지 않음
    - 이 경우 Newton iteration이 시작조차 할 수 없다.

## 4. 해결 방법
- ✔ 방법 1. n < p 인 경우 early return 처리
- OpenNURBS도 내부적으로 이렇게 처리한다.
```rust
if n < p {
    // basis function is not bell-shaped → maximum occurs at boundary
    return Ok((1.0, knots[i+1]));
}
```

- ✔ 방법 2. 테스트 케이스를 올바른 knot vector로 변경
    - degree 2 basis를 테스트하려면 최소 3 control point가 필요하다.

## 5. 결론
- Newton iteration이 실패하는 것은 코드 오류가 아니라 수학적 특성 때문이다.
    - 되는 케이스: basis function이 bell-shaped이고 내부 최대값이 존재
    - 안 되는 케이스: basis function이 선형/편향/plateau 형태라 내부 최대값이 없음
- 따라서 on_compute_basis_maximum()은 특정 조건에서만 사용 가능한 함수이며,
- 그 외의 경우에는 early return 또는 boundary 처리로 우회해야 한다.

---

## on_compute_basis_maximum 알고리즘 설명
- (B‑spline basis $N_{i,p}(u)$ 최대값 찾기)
## 1. 목적
- on_compute_basis_maximum(kv, i, p, tol) 의 목적은:
    - 주어진 knot vector kv
    - basis index i
    - degree p
- 에 대해,
    - basis function $N_{i,p}(u)$ 의
    - 최대값 $N_{\max }$
    - 그때의 위치 $u_{\max }$
- 를 찾는 것.
- 즉,
```math
u_{\max }=\arg \max _uN_{i,p}(u),\quad N_{\max }=N_{i,p}(u_{\max })
```
- 를 구하는 함수다.

## 2. 입력과 기본 검증
- 초반부 코드:
```rust
let u = kv.as_slice();
let p_us = p as usize;
let i_us = i as usize;
let m = u.len() - 1;
```

- 기본 검증 내용:
    - degree > 0 (p == 0 이면 에러)
    - knots.len() >= p+2 (기본적인 basis가 정의되려면 최소한 필요)
    - knots가 non-decreasing인지 확인
    - n = m - p - 1 이 0 이상인지 확인
    - basis index i가 [0..=n] 범위인지 확인
- 이 부분에서 불완전한 knot vector나 잘못된 index를 사전에 걸러낸다.

## 3. 특수 케이스 처리
### 3.1 degree 1 (선형 basis)
```rust
if p_us == 1 {
    return Ok((1.0, u[i_us + 1]));
}
```

- degree 1인 B‑spline basis는 사실상 선형 **삼각형** 이 아니라, 구간에서 한 번만 1이 되는 hat 함수 형태라,
- 특정 index의 basis 최대값은 항상 1이고 그 위치는 가운데 knot다.
- 그래서:
```math
N_{i,1}(u_{\max })=1,\quad u_{\max }=U_{i+1}
```
- 을 바로 반환한다.

### 3.2 내부 p‑fold knot (full multiplicity) 존재
```rust
let mut s = 1usize;
for k in (i_us + 1)..(i_us + p_us) {
    if on_are_equal(u[k], u[k + 1], ON_TOL9) {
        s += 1;
    }
}
if s == p_us {
    // max = 1, 위치 = u[i+1]
}
```

- 여기서 하는 일:
- $U[i+1..i+p]$ 구간에서 연속 knot가 모두 같은 값인지 확인
- multiplicity s가 degree p 와 같다면,
- 즉 내부에 degree만큼 반복된 knot가 있다면
- 그 위치에서 basis가 1을 달성한다.
- 이건 Piegl & Tiller / OpenNURBS에서 쓰는 전형적인 규칙으로:
    - 내부 full-multiplicity knot → 그 위치에서 basis 하나가 딱 1이 되고, 좌우가 0이 된다.
    - 그래서 더 이상 Newton이나 다른 계산은 필요 없다.
- 이때:
    - 모든 span k에서 max[k] = 1.0
    - 대부분 span에서 min[k] = 1.0 or 0.0
    - global maximum 위치는 u[i+1]
- 으로 처리하고 바로 반환.

## 4. 일반 케이스: Newton으로 dN/du = 0 찾기
- 이제 진짜 핵심 부분.
### 4.1 지원 구간(support interval)
- basis $N_{i,p}(u)$ 의 support 는:
- $[u_i,u_{i+p+1}]$
- 코드에서는:
```rust
let span = u[i_us + p_us + 1] - u[i_us];
if span <= 0.0 { ... }
let mut du = span / (nos as Real);
```
- nos = p * nok (nok=10) → 샘플링 세기.
- 여기서 span은basis가 non-zero인 전체 길이.

### 4.2 브래킷팅(Bracketing): “최대값이 있을 것 같은 구간” 찾기
- Newton은 1차 미분이 0이 되는 지점을 찾는 알고리즘이다.
- 여기서는 basis의 최대값 → dN/du = 0 을 쓰는 것.
- 그런데 Newton은 초기값이 중요하므로 먼저 도함수 sign이 바뀌는 구간을 찾아야 한다.
- 코드:
```rust
let mut ul = u[i_us];
let dom_l = u[p_us];
let dom_r = u[m - p_us];
// dom_l, dom_r는 전체 유효 도메인.

let mut ur = ul;

let right_support = u[i_us + p_us + 1].min(dom_r);

while ur < right_support {
    ul = ur;
    ur += du;
    if ur > right_support {
        ur = right_support;
    }

    on_compute_basis_and_derivatives(kv, i, p, ur, Side::Left, 1, &mut nd)?;
    if nd[1] < 0.0 {
        break;
    }
}
```

- 해석:
    - support 왼쪽 끝에서 오른쪽으로 조금씩 ur 을 증가시키며
    - 1차 미분 N'(ur) 의 부호가 음수로 바뀌는 지점을 찾는다.
- 이때 ul, ur는:
    - N'(ul)≥0
    - N'(ur)<0
- 가 되도록 만드는 게 목적.
- 즉, **“좌측에서 오르다가, 우측에서 내리는 구간”**을 찾는 것 → 그 사이에 최대값이 있다.

### 4.3 Newton iteration
- 브래킷팅이 끝나면 Newton 반복:
```rust
let mut conv = false;
let mut u0: Param = 0.5 * (ul + ur);

let mut it = 0usize;
while it < itl {
    u0 = 0.5 * (ul + ur);

    let mut k = 0usize;
    while k < itl {
        on_compute_basis_and_derivatives(kv, i, p, u0, Side::Left, 2, &mut nd)?;

        if nd[1].abs() < tol && nd[0] > tol {
            conv = true;
            break;
        }

        if nd[2].abs() <= ON_TOL9 {
            return Err(NumericError: "division by near-zero second derivative");
        }

        u0 = u0 - nd[1] / nd[2];

        let left_support = u[i_us].max(dom_l);
        let right_support = u[i_us + p_us + 1].min(dom_r);
        if u0 <= left_support || u0 >= right_support {
            break;
        }

        k += 1;
    }

    if conv { break; }

    // 수렴 실패 → bracket refine, 다시 시도
    ...
}
```

- 여기서:
    - nd[0] = N(u0), nd[1] = N'(u0), nd[2] = N''(u0)
- Newton step:
```math
u_{k+1}=u_k-\frac{N'(u_k)}{N''(u_k)}
```
- 수렴 조건:
    - $|N'(u_k)|<tol$
    - 동시에 $N(u_k)>tol$ (0 근처의 flat한 점은 무시)
- 주의할 점:
    - 2차 미분이 너무 작으면 (flat), 나눗셈이 수치적으로 불안정 → 에러 반환
    - Newton으로 구한 u0가 support 구간 밖으로 튀어나가면 → break 후 bracket refine

### 4.4 Bracket refine
- Newton이 수렴하지 않으면:
```rust
du = (ur - ul) / (nok as Real);
ur = ul;

for _ in 1..=nok {
    ul = ur;
    ur += du;

    on_compute_basis_and_derivatives(..., ur, ...)?
    if nd[1] < 0.0 { break; }
}
```

- 즉,
    - 기존 [ul, ur] 구간을 더 잘게 쪼개서
    - 다시 N'(ur) < 0 를 만족하는 구간을 찾고
    - 새로운 중간값에서 Newton을 다시 시도
- 최대 itl번 (20번) 반복.
- 그래도 수렴 안 되면:
```rust
if !conv {
    return Err(NumericError: "Newton iteration did not converge");
}
```

## 5. 최종 결과 세팅
- 수렴에 성공하면:
    - $nd[0] = N_{i,p}(u_0)$
    - $u0 = u_{\max }$
- return 값:
```rust
Ok((nd[0], u0))
```
- 이 함수는 “global maximum (전역 최대)”만 반환하고,
- 각 span별 min/max는 별도 로직 (on_kntmma_minmax_per_span)에서 더 다룬다.

### 6. 왜 수렴 안 하는 케이스가 있는가?
- 이제 겪었던 에러:
```
NumericError { msg: "compute_basis_maximum: Newton iteration did not converge" }
```
- 이건 다음과 같은 경우에 발생한다:
    - basis가 내부 최대값을 가지지 않는 구조 (예: n < p, 선형에 가까운 형태)
    - support 구간이 너무 짧거나 degenerate
    - 브래킷팅 과정에서 N'(u)가 0이 되는 지점을 찾지 못함
    - 2차 미분이 거의 0이라 Newton step이 안정적으로 계산되지 않음
- 즉, 코드가 잘못된 게 아니라, 수학적으로 최대값이 내부에 존재하지 않아서 Newton이 못 찾는 것임.


## 7. 요약 (한 번에 정리)
- on_compute_basis_maximum의 전체 흐름을 한 줄씩 정리하면:
    - knot vector / degree / index 유효성 체크
    - degree 1 → 최대값은 항상 1, 위치는 U[i+1]
    - 내부 p-fold multiplicity → 최대값 1, 위치는 knot block
- 일반 케이스:
    - support interval [U[i], U[i+p+1]] 에서  
        dN/du가 음수로 바뀌는 구간 [ul, ur] 을 샘플링으로 찾음
    - 그 구간에서 Newton으로 dN/du = 0, 즉 최대값 후보 찾음
    - 안 되면 bracket을 refine하고 다시 시도
    - 끝까지 안 되면 **Newton iteration did not converge** 에러


---

## 📌 왜 “한 개만 최대값 1을 갖는가?”
- B‑spline basis $N_{i,p}(u)$ 의 성질:
    - 각 basis는 p+1개의 knot interval에서만 non‑zero
    - 같은 span에서는 p+1개의 basis만 non‑zero
    - 이 p+1개 basis의 합은 항상 1
```math
\sum _{j=0}^pN_{i+j,p}(u)=1
```
- 이 중 하나는 항상 1에 도달한다  
    (특히 full multiplicity knot에서)
- 즉, 어떤 u에서든:
    - p+1개 basis만 살아 있고
    - 그 중 하나는 “peak”를 가지며
    - 그 peak는 대부분 1.0이다.

### 📌 on_compute_basis_maximum()이 하는 일
- 이 함수는:
    - basis index i
    - degree p
    - knot vector U
- 에 대해
```math
\max _uN_{i,p}(u)
```
- 을 찾는다.
- 즉, 그 basis가 1이 되는 위치를 찾는 것이다.
- ✔ 특수 케이스에서는 바로 1 반환
    - 내부 knot가 p번 반복되면
        - 그 knot에서 basis가 정확히 1
        - Newton 필요 없음
- ✔ 일반 케이스에서는 Newton으로 dN/du = 0 찾기
    - basis가 종 모양(bell shape)일 때
    - 내부에서 미분이 0이 되는 지점이 존재
    - 그 지점에서 최대값을 찾는다

### 📌 그런데 왜 어떤 케이스는 수렴하지 않는가?
- 아주 중요한 포인트:
- ❌ n < p 인 경우
    - basis function은 bell shape이 아니다.
    - 즉, 내부 최대값이 없다.
- 예:
    - knots = [0,0,0.5,1,1]
    - degree = 2

- 이 경우 basis는 “기울어진 선형 함수”라서 내부에서 미분이 0이 되는 지점이 없다.
- 그래서 Newton iteration이 절대로 수렴할 수 없다.
- 이건 오류가 아니라 수학적으로 최대값이 존재하지 않기 때문이다.

### 📌 다시 질문에 대한 답
- 구간에서만 n... n + p + 1 개의 basis는 한개만이 최대값 1을 갖지? 지금 그것 계산하는 거야?
    - ✔ 맞아.
    - ✔ 정확히 그걸 계산하는 함수가 on_compute_basis_maximum()이다.
    - ✔ 단, basis가 bell shape일 때만 Newton이 수렴한다.
    - ✔ bell shape이 아닌 경우(예: n < p)는 최대값이 내부에 없어서 Newton이 실패한다.


---

## u가 왜 중요한가
- 왜 basis function의 최대값을 찾을 때 u(파라미터)가 그렇게 중요한가?
- 왜 basis function은 u에 따라 값이 변하고, 그 최대값을 찾는 과정이 필요한가?
- 왜 NURBS는 u라는 파라미터 공간을 이렇게 중요하게 다루는가?

### 🎯 1. NURBS/B‑spline은 “u-parameter space”에서 정의되는 함수다
- NURBS 곡선은 이렇게 정의:
```math
C(u)=\frac{\sum _iN_{i,p}(u)w_iP_i}{\sum _iN_{i,p}(u)w_i}
```
- 여기서 u가 없으면 곡선 자체가 존재할 수 없다.
    - basis function $N_{i,p}(u)$ 는 u에 따라 값이 변한다
    - weight도 u에 따라 영향을 준다
    - 최종 점 C(u)도 u에 따라 변한다
    - 즉, u는 곡선의 “시간” 또는 “좌표계” 같은 것이다.

### 🎯 2. basis function의 최대값은 “해당 control point가 곡선에 가장 크게 기여하는 지점”을 의미한다
- basis function $N_{i,p}(u)$ 는 control point $P_i$ 가 곡선에 얼마나 영향을 주는지를 나타내는 함수다.
    - $N_{i,p}(u)$ 가 크면 → 그 u에서 $P_i$ 의 영향력이 크다
    - $N_{i,p}(u)$ 가 1이면 → 그 u에서 곡선은 사실상 $P_i$ 방향으로 가장 강하게 끌린다
    - $N_{i,p}(u)$ 가 0이면 → 그 u에서는 $P_i$ 가 곡선에 전혀 기여하지 않는다
- 따라서 basis function의 최대값이 어디서 발생하는지(u 값) 는 매우 중요한 의미를 가진다.

### 🎯 3. 왜 “u에서 최대값 1이 되는 basis가 딱 하나”인가?
- 어떤 u에서든:
    - p+1개의 basis만 non-zero
    - 그 중 하나는 항상 가장 크다
    - full multiplicity knot에서는 그 값이 정확히 1이 된다
- 즉, u는 어떤 control point가 곡선을 지배하는지를 결정하는 좌표다.

### 🎯 4. on_compute_basis_maximum()이 하는 일은 결국 “control point 영향력의 peak 위치”를 찾는 것이다
- 이 함수는:
    - basis function $N_{i,p}(u)$ 의 최대값을 찾고
    - 그 최대값이 발생하는 u를 찾는다
- 이 u는 의미가 크다:
    - 곡선에서 $P_i$ 가 가장 강하게 작용하는 지점
    - 곡률 분석, offset 안정화, knot insertion, refinement 등에 사용
    - NURBS 곡선의 local behavior를 이해하는 핵심 정보
- 즉, u는 단순한 숫자가 아니라 곡선의 구조를 결정하는 핵심 좌표다.

### 🎯 5. “왜 u가 중요하냐”에 대한 직관적 비유
- 📌 비유 1: 음악에서 시간 t
    - 곡선 C(u)는 음악의 waveform 같은 것.
    - u는 “시간”.
        - 시간 t가 없으면 음악이 존재할 수 없음
        - 특정 t에서 어떤 악기가 가장 크게 들리는지 → basis maximum과 동일한 개념
- 📌 비유 2: 애니메이션에서 frame index
    - u는 애니메이션의 frame 번호 같은 것.
        - frame이 없으면 움직임이 없음
        - 특정 frame에서 어떤 bone이 가장 크게 움직이는지 → basis maximum
- 📌 비유 3: 지도에서 GPS 좌표
    - 곡선은 길이고, u는 길 위의 GPS 좌표.
        - GPS 좌표가 없으면 길을 따라 움직일 수 없음
        - 특정 GPS에서 어떤 control point가 길을 얼마나 당기는지 → basis maximum

### 🎯 6. 결론: u는 NURBS의 “좌표계”이자 “곡선의 DNA”다
- 그래서 u가 중요한 이유는:
    - 곡선의 모든 점은 u로 정의된다
    - basis function은 u에 따라 값이 변한다
    - control point의 영향력도 u에 따라 달라진다
    - 최대값을 찾는 것도 결국 u에서 일어난다
    - offset, curvature, refinement 등 모든 고급 알고리즘이 u 기반으로 동작한다
- 즉,
    - ⭐ “u는 NURBS의 세계에서 x좌표보다 더 중요한 좌표다.”
    - ⭐ “u가 없으면 NURBS는 존재할 수 없다.”

--

## 🎯 B‑spline Basis Function의 형태 변화: degree에 따른 특징
### ✅ Degree 1: 선형 basis (piecewise linear)
- 모양: 삼각형(hat function)
    - 정의역: $[u_i,u_{i+2}]$
- 특징:
    - 두 개의 선형 구간으로 구성
    - 가운데 $u_{i+1}$ 에서 최대값 1
- 가장 단순한 형태
- 예시:
```math
N_{i,1}(u)=\left\{ \, \begin{array}{ll}\textstyle \frac{u-u_i}{u_{i+1}-u_i},&\textstyle u\in [u_i,u_{i+1})\\ \textstyle \frac{u_{i+2}-u}{u_{i+2}-u_{i+1}},&\textstyle u\in [u_{i+1},u_{i+2})\\ \textstyle 0,&\textstyle \mathrm{otherwise}\end{array}\right.
```

### ✅ Degree 2: 2차 basis (piecewise quadratic)
- 모양: 종 모양(bell-shaped)
    - 정의역: $[u_i,u_{i+3}]$
- 특징:
    - 3개의 구간으로 구성된 2차 함수
    - 가운데에서 최대값 1
    - 부드럽고 곡률이 생김

### ✅ Degree 3: 3차 basis (piecewise cubic)
- 모양: 더 넓고 부드러운 종 모양
    - 정의역: $[u_i,u_{i+4}]$
- 특징:
    - 4개의 구간으로 구성된 3차 함수
    - 더 완만한 상승/하강
    - 곡선의 부드러움이 증가

### ✅ Degree p: 일반적인 특징
- 정의역: $[u_i,u_{i+p+1}]$
- 구성: p+1 개의 구간에서 piecewise polynomial
- 연속성: $C^{p-1}$ (단, knot multiplicity에 따라 감소 가능)
- 최대값: 대부분 내부에서 발생, full multiplicity knot에서는 해당 knot에서 1

### 📌 Basis 분포와 non-zero 영역
- 어떤 u에서든:
    - p+1개의 basis만 non-zero
    - 그 basis들은 해당 span에서만 살아 있음
    - 그 중 하나는 최대값 1에 도달
    - 나머지는 0 < N < 1
- 즉, basis 함수는:
    - local support를 가지며
    - 곡선의 국소 제어를 가능하게 함

### 📌 Degree가 올라갈수록 생기는 변화
| Degree | Basis 형태            | 정의역 길이      | 부드러움(연속성) | non-zero basis 개수 | 최대값 위치 |
|--------|------------------------|-------------------|-------------------|----------------------|--------------|
| 1      | 삼각형(hat function)  | 2 knot interval   | C⁰                | 2개                 | 가운데 knot |
| 2      | 둥근 종(bell shape)   | 3 knot interval   | C¹                | 3개                 | 내부        |
| 3      | 넓고 부드러운 종      | 4 knot interval   | C²                | 4개                 | 내부        |
| p      | p차 다항식 basis      | p+1 interval      | $C^{p−1}$           | p+1개               | 내부 or knot |


## 🎯 결론
- degree가 1이면 단순한 삼각형
- degree가 올라갈수록 basis 함수는 더 넓고 부드러워지고
- 정의역도 길어지고
- 내부에서 최대값을 갖는 위치(u)가 더 중요해진다
- 이 u를 찾는 게 바로 on_compute_basis_maximum()의 역할이고,  
    그 basis가 곡선에서 어느 구간을 지배하는지를 알려주는 핵심 정보.

---

## Degree & Basis
- “degree가 올라갈수록 basis가 어떻게 바뀌는지”를 직접 그림 그리듯이 머릿속에 그릴 수 있도록 정리.
- 설명을 단순하게 하기 위해, 공통으로 이런 uniform open knot를 쓴다고 가정:
    - degree 1: [0,0,1,2,3]
    - degree 2: [0,0,0,1,2,3]
    - degree 3: [0,0,0,0,1,2,3]
- 여기서 우리는 항상 같은 index i의 basis를 봄: $N_{i,p}(u)$  
    (대략 가운데 basis라고 생각).

### 1. degree 1: 선형(B‑spline) basis — “삼각형”
- degree 1 (선형) basis는 진짜 말 그대로 삼각형(모자 함수, hat function) 모양.
- 정의역: $[u_i,u_{i+2}]$
- 가운데 knot $u_{i+1}$ 에서 최대값 1
- 좌/우는 linear로 올라갔다 내려감
- 대략적인 그래프(모양 느낌):
```
값
1.0      /\
        /  \
0.0 ___/    \____
 u_i  u_{i+1}  u_{i+2}
       (최댓값)
```

- 수식으로 보면:
```math
N_{i,1}(u)=\left\{ \, \begin{array}{ll}\textstyle \frac{u-u_i}{u_{i+1}-u_i},&\textstyle u\in [u_i,u_{i+1})\\ \textstyle \frac{u_{i+2}-u}{u_{i+2}-u_{i+1}},&\textstyle u\in [u_{i+1},u_{i+2})\\ \textstyle 0,&\textstyle \mathrm{otherwise}\end{array}\right. 
```
### 2. degree 2: 2차 basis — “조금 더 둥근 종(bell)”
- degree 2가 되면, basis는 삼각형이 아니라 살짝 둥근 종(bell-shaped) 으로 바뀐다.
- 정의역: $[u_i,u_{i+3}]$
- 3개의 구간에서 각기 다른 2차 다항식
- 가운데 어딘가에서 최대값 (보통 1, full multiplicity 아니면 1보다 약간 작을 수도 있음)
- 모양 느낌:
```
값
1.0         /\
           /  \
0.5      _/    \_
0.0 ____/        \______
     u_i   ...   u_{i+3}
```

- 삼각형보다:
    - 좌우가 더 완만하게 올라가고 내려간다
    - 곡선이 “각”이 아니라 부드럽게 꺾인다
    - 연속성이 C^1로 올라간다 (기울기가 연속)

### 3. degree 3: 3차 basis — “더 넓고 부드러운 종”
- degree 3이 되면, 우리가 CAD에서 가장 많이 보는 cubic B‑spline basis가 된다.
- 정의역: $[u_i,u_{i+4}]$
- 4개의 구간에 걸친 3차 다항식
- 훨씬 더 부드럽고, “봉우리”도 더 넓게 퍼져 있다
- 연속성이 C^2
- 모양 느낌:
```
값
1.0            __
              /  \
0.7         _/    \_
0.4       _/        \_
0.0 _____/            \_______
     u_i      ...     u_{i+4}
```

- 느낌적으로:
    - degree 1: 날카로운 삼각형
    - degree 2: 조금 둥근 언덕
    - degree 3: 넓고 부드러운 언덕
- degree가 올라갈수록:
    - support(살아있는 구간)가 길어지고
    - 기여 범위가 넓어지고
    - 각 control point의 영향이 더 멀리 퍼져 나간다
    - 대신 곡선이 훨씬 더 부드러워진다

### 4. 하나의 구간(span)에서 basis 분포 비교
- 예를 들어, 어떤 u 가 하나의 span $[u_j,u_{j+1}]$ 안에 있다고 해보자.
    - degree 1: 이 span에서 non-zero basis는 2개
    - degree 2: non-zero basis는 3개
    - degree 3: non-zero basis는 4개
- 각각의 모양을 겹쳐놓으면 (겉보기):
- degree 1 (간단한 삼각형들)
```
값
1.0      /\    /\
        /  \  /  \
0.0 ___/    \/    \____
      i    i+1   i+2
```

- degree 2 (겹쳐지는 종들)
```
값
1.0        /\ 
          /  \ 
0.5     _/    \_
0.0 ___/        \____
       i   i+1   i+2  i+3
```

- degree 3 (더 많이 겹쳐지는 종들)
```
값
1.0          __
            /  \
0.7       _/    \_
0.3     _/        \_
0.0 ___/            \______
       i   i+1  i+2  i+3  i+4
```

- 각 u에서 이 basis들이 다 더해지면 항상:
```math
\sum _kN_{k,p}(u)=1
```
- 이게 “convex combination”을 만들어서 곡선이 안정적으로 동작하도록 해준다.

### 5. on_compute_basis_maximum과의 연결
- 이제 on_compute_basis_maximum은 뭘 하냐면:
    - 위와 같은 종 모양들의 각각에 대해
    - 그 basis가 가장 높이 올라가는 봉우리 위치 u
    - 그때의 값 (보통 1에 근접 혹은 1)
- 를 찾는 함수.
즉:
- degree 1: 가운데 knot (해석적으로 바로 알 수 있음)
- degree 2,3,...:
- 종 모양 안에서 dN/du = 0 이 되는 지점을 Newton으로 찾아가는 것
- 그 u가 바로 “이 basis가 최대로 기여하는 위치”

### 6. 한 장짜리 요약
- degree 1:
    - 모양: 삼각형
    - non-zero 구간: 2 interval
    - non-zero basis 개수: 2개
    - 최대값: 가운데 knot에서 1
- degree 2:
    - 모양: 둥근 언덕 (piecewise quadratic)
    - non-zero 구간: 3 interval
    - non-zero basis 개수: 3개
    - 최대값: 내부 어딘가 (보통 거의 1)
- degree 3:
    - 모양: 넓고 부드러운 종 (piecewise cubic)
    - non-zero 구간: 4 interval
    - non-zero basis 개수: 4개
    - 최대값: 내부 어딘가 (보통 거의 1)

---

## 소스 코드
```rust
/// Compute the global maximum of a B-spline basis function N_{i,p}(u).
///
/// Returns (max_value, u_at_max)
pub fn on_compute_basis_maximum(
    kv: &KnotVector,
    i: Index,
    p: Degree,
    tol: Real,
) -> Result<(Real, Param), NurbsError> {
    let u = kv.as_slice();
    if u.is_empty() {
        return Err(NurbsError::EmptyKnots);
    }

    // ---- normalize types / basic checks (avoid usize underflow) ----
    let p_us = p as usize;
    let i_us = i as usize;
    let m = u.len() - 1;

    if p_us == 0 {
        return Err(NurbsError::InvalidArgument {
            msg: "compute_basis_maximum: degree p must be > 0".into(),
        });
    }

    // Need indices i+p+1, i+p, i+1, etc.
    if u.len() < p_us + 2 {
        return Err(NurbsError::InvalidArgument {
            msg: "compute_basis_maximum: knot vector too short for degree".into(),
        });
    }

    // n = m - p - 1  (basis index range: 0..=n)
    if m < p_us + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "compute_basis_maximum: invalid knot/degree (m < p+1)".into(),
        });
    }
    let n = m - p_us - 1;

    if i_us > n {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "compute_basis_maximum: index i={} out of range [0, {}]",
                i_us, n
            ),
        });
    }

    // ---- Special case: linear basis ----
    if p_us == 1 {
        return Ok((1.0, u[i_us + 1]));
    }

    // ---- Check p-fold inner multiple knot exists ----
    // C: exact equality (U[k]==U[k+1]). Here we use on_are_equal to be robust.
    let mut s = 1usize;
    for k in (i_us + 1)..(i_us + p_us) {
        if on_are_equal(u[k], u[k + 1], ON_TOL9) {
            s += 1;
        }
    }
    if s == p_us {
        return Ok((1.0, u[i_us + 1]));
    }

    // ---------------------------------------------------------
    // Bracketing step (find ul, ur such that dN/du changes sign)
    // ---------------------------------------------------------
    let nok: usize = 10;
    let itl: usize = 20;

    let dom_l = u[p_us];
    let dom_r = u[m - p_us];

    // In C code, ul=ur=U[i]. But Rust derivative routine checks domain [U[p], U[m-p]].
    // So clamp start point into valid domain to avoid early Err.
    let mut ul = u[i_us];
    if ul < dom_l {
        ul = dom_l;
    }
    if ul > dom_r {
        ul = dom_r;
    }
    let mut ur = ul;

    let nos = p_us * nok;
    let span = u[i_us + p_us + 1] - u[i_us];

    if span <= 0.0 {
        return Err(NurbsError::InvalidArgument {
            msg: "compute_basis_maximum: degenerate basis support (U[i+p+1] <= U[i])".into(),
        });
    }
    let mut du = span / (nos as Real);
    if du <= 0.0 {
        return Err(NurbsError::InvalidArgument {
            msg: "compute_basis_maximum: invalid du".into(),
        });
    }

    let mut nd = [0.0_f64; 3];

    // Move ur forward until dN/du < 0 (bracket around the max where dN/du=0)
    let right_support = u[i_us + p_us + 1].min(dom_r);

    while ur < right_support {
        ul = ur;
        ur += du;
        if ur > right_support {
            ur = right_support;
        }

        on_compute_basis_and_derivatives(kv, i, p, ur, Side::Left, 1, &mut nd)?;
        if nd[1] < 0.0 {
            break;
        }
    }

    // ---------------------------------------------------------
    // Newton iteration
    // ---------------------------------------------------------
    let mut conv = false;
    let mut u0: Param = 0.5 * (ul + ur);

    let mut it = 0usize;
    while it < itl {
        u0 = 0.5 * (ul + ur);

        // Do Newton with guess parameter
        let mut k = 0usize;
        while k < itl {
            on_compute_basis_and_derivatives(kv, i, p, u0, Side::Left, 2, &mut nd)?;

            // C: |ND[1]| < tol && ND[0] > tol  => accept
            if nd[1].abs() < tol && nd[0] > tol {
                conv = true;
                break;
            }

            // C: M_chkfop(ND[1], ND[2], DIVISION)
            // Use project epsilon (ON_TOL9) to prevent division blow-ups.
            if nd[2].abs() <= ON_TOL9 {
                return Err(NurbsError::NumericError {
                    msg: "compute_basis_maximum: division by near-zero second derivative".into(),
                });
            }

            u0 = u0 - nd[1] / nd[2];

            // If Newton jumps outside support interval, break to refine bracket
            let left_support = u[i_us].max(dom_l);
            let right_support = u[i_us + p_us + 1].min(dom_r);
            if u0 <= left_support || u0 >= right_support {
                break;
            }

            k += 1;
        }

        if conv {
            break;
        }

        // No convergence -> refine [ul, ur] and get better guess
        du = (ur - ul) / (nok as Real);
        ur = ul;

        for _ in 1..=nok {
            ul = ur;
            ur += du;

            on_compute_basis_and_derivatives(kv, i, p, ur, Side::Left, 1, &mut nd)?;
            if nd[1] < 0.0 {
                break;
            }
        }

        it += 1;
    }

    if !conv {
        return Err(NurbsError::NumericError {
            msg: "compute_basis_maximum: Newton iteration did not converge".into(),
        });
    }

    Ok((nd[0], u0))
}
```
```rust
/// Compute one basis function N_{i,p}(u) and its derivatives up to order `der`.
///
/// - `kv`   : knot vector (clamped, end knots repeated p+1)
/// - `i`    : basis index (0 <= i <= n)
/// - `p`    : degree
/// - `u`    : parameter
/// - `side` : LEFT  → u ∈ [U[j],U[j+1])  (RIGHT derivative required)
///            RIGHT → u ∈ (U[j],U[j+1]] (LEFT  derivative required)
/// - `der`  : highest derivative to compute
/// - `nd`   : output array, length >= der+1
///
/// ND[0] = N_{i,p}(u), ND[1] = d/du N_{i,p}(u), ...
pub fn on_compute_basis_and_derivatives(
    kv: &KnotVector,
    i: Index,
    p: Degree,
    u: Param,
    side: Side,
    der: Index,
    nd: &mut [Real],
) -> Result<()> {
    let knots = kv.as_slice();
    let m = knots.len() - 1;
    let p = p as usize;
    let i = i as usize;
    let der = der as usize;

    if p == 0 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_compute_basis_and_derivatives: degree p must be > 0".into(),
        });
    }

    if nd.len() < der + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "on_compute_basis_and_derivatives: ND length {} < der+1 {}",
                nd.len(),
                der + 1
            ),
        });
    }

    // n = #basis - 1
    let n = m - p - 1;

    // parameter/domain check (C: E_parval)
    if u < knots[p] || u > knots[m - p] {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "parameter {} out of knot domain [{}, {}]",
                u,
                knots[p],
                knots[m - p]
            ),
        });
    }

    if i > n {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "on_compute_basis_and_derivatives: index i={} out of range [0, {}]",
                i, n
            ),
        });
    }

    // Allocate local arrays:
    // nt: (p+1) x (p+1) (triangular table for basis)
    // nd_tmp: (p+1) (for derivative computation)
    let mut nt = vec![vec![0.0_f64; p + 1]; p + 1];
    let mut nd_tmp = vec![0.0_f64; p + 1];

    // ------------------------------------------------------------
    // 1) Compute degree-zero B-splines into nt[*][0]
    // ------------------------------------------------------------

    match side {
        Side::Left => {
            // Special case: u == U[m-p] and i >= n-p
            if u == knots[m - p] && i >= n - p {
                for j in 0..=p {
                    let uj0 = knots[i + j];
                    let uj1 = knots[i + j + 1];
                    if u > uj0 && u <= uj1 {
                        nt[j][0] = 1.0;
                    } else {
                        nt[j][0] = 0.0;
                    }
                }
            } else {
                // Outside support: all derivatives zero
                if u < knots[i] || u >= knots[i + p + 1] {
                    for j in 0..=der {
                        nd[j] = 0.0;
                    }
                    return Ok(());
                }

                // Standard LEFT: u ∈ [U[i+j], U[i+j+1])
                for j in 0..=p {
                    let uj0 = knots[i + j];
                    let uj1 = knots[i + j + 1];
                    if u >= uj0 && u < uj1 {
                        nt[j][0] = 1.0;
                    } else {
                        nt[j][0] = 0.0;
                    }
                }
            }
        }

        Side::Right => {
            // Special case: u == U[p] and i <= p
            if u == knots[p] && i <= p {
                for j in 0..=p {
                    let uj0 = knots[i + j];
                    let uj1 = knots[i + j + 1];
                    if u >= uj0 && u < uj1 {
                        nt[j][0] = 1.0;
                    } else {
                        nt[j][0] = 0.0;
                    }
                }
            } else {
                // Outside support: all derivatives zero
                if u <= knots[i] || u > knots[i + p + 1] {
                    for j in 0..=der {
                        nd[j] = 0.0;
                    }
                    return Ok(());
                }

                // Standard RIGHT: u ∈ (U[i+j], U[i+j+1]]
                for j in 0..=p {
                    let uj0 = knots[i + j];
                    let uj1 = knots[i + j + 1];
                    if u > uj0 && u <= uj1 {
                        nt[j][0] = 1.0;
                    } else {
                        nt[j][0] = 0.0;
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------
    // 2) Full triangular array for basis up to degree p
    //    (de Boor–Cox style, but using the C algorithm)
    // ------------------------------------------------------------

    for k in 1..=p {
        let mut saved = if nt[0][k - 1] == 0.0 {
            0.0
        } else {
            ((u - knots[i]) * nt[0][k - 1]) / (knots[i + k] - knots[i])
        };

        for j in 0..=(p - k) {
            let ur = knots[i + j + k + 1];
            let ul = knots[i + j + 1];

            if nt[j + 1][k - 1] == 0.0 {
                nt[j][k] = saved;
                saved = 0.0;
            } else {
                let temp = nt[j + 1][k - 1] / (ur - ul);
                nt[j][k] = saved + (ur - u) * temp;
                saved = (u - ul) * temp;
            }
        }
    }

    // ------------------------------------------------------------
    // 3) Derivatives
    // ------------------------------------------------------------

    nd[0] = nt[0][p];

    let mder = std::cmp::min(p, der);

    // ND[k] = 0 for k > p
    for k in (p + 1)..=der {
        nd[k] = 0.0;
    }

    for k in 1..=mder {
        // Load appropriate column into nd_tmp
        // C: nd[j] = nt[j][p-k]
        for j in 0..=k {
            nd_tmp[j] = nt[j][p - k];
        }

        // Triangular table (width = k)
        for l in 1..=k {
            let mut saved = if nd_tmp[0] == 0.0 {
                0.0
            } else {
                nd_tmp[0] / (knots[i + p - k + l] - knots[i])
            };

            for j in 0..(k - l + 1) {
                let ur = knots[p - k + l + i + j + 1];
                let ul = knots[i + j + 1];

                if nd_tmp[j + 1] == 0.0 {
                    nd_tmp[j] = (p - k + l) as Real * saved;
                    saved = 0.0;
                } else {
                    let temp = nd_tmp[j + 1] / (ur - ul);
                    nd_tmp[j] = (p - k + l) as Real * (saved - temp);
                    saved = temp;
                }
            }
        }

        nd[k] = nd_tmp[0];
    }

    Ok(())
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::basis::Side;
    use nurbslib::core::knot::{KnotVector, on_compute_basis_and_derivatives};
    use nurbslib::core::knots_extensions::on_compute_basis_maximum;
    use nurbslib::core::types::{Degree, Index, NurbsError, ON_TOL9, Real};

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn test_basis_maximum_linear() {
        // p = 1 → 항상 max = 1 at U[i+1]
        let kv = KnotVector::new(vec![0.0, 0.0, 1.0, 2.0, 2.0]).unwrap();

        let p = 1;
        let i = 1;

        let (max, u) = on_compute_basis_maximum(&kv, i, p, 1e-8).unwrap();

        println!("max = {}, u = {}", max, u);
        assert!(approx(max, 1.0));
        assert!(approx(u, kv.knots[i + 1]));
    }

    #[test]
    fn test_basis_maximum_p_fold_inner_knot() {
        // p = 3, and U[i+1] = U[i+2] = U[i+3]
        // → max = 1 at U[i+1]
        let kv = KnotVector::new(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0,
        ])
        .unwrap();

        let p = 3;
        let i = 3; // basis with full multiplicity inside

        let (max, u) = on_compute_basis_maximum(&kv, i, p, 1e-8).unwrap();

        println!("max = {}, u = {}", max, u);

        assert!(approx(max, 1.0));
        assert!(approx(u, kv.knots[i + 1]));
    }

    #[test]
    fn test_basis_maximum_quadratic() {
        // Quadratic B-spline (p=2)
        // U = {0,0,0,1,2,3,3,3}
        let kv = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0]).unwrap();

        let p = 2;
        let i = 2;

        let (max, u) = on_compute_basis_maximum(&kv, i, p, 1e-8).unwrap();
        println!("max = {}, u = {}", max, u);

        // finite difference check
        let f = |x: f64| {
            let mut nd = [0.0; 3];
            on_compute_basis_and_derivatives(&kv, i, p, x, Side::Left, 0, &mut nd).unwrap();
            nd[0]
        };

        let h = 1e-6;
        let numeric_deriv = (f(u + h) - f(u - h)) / (2.0 * h);

        // derivative at maximum must be ~0
        assert!(numeric_deriv.abs() < 1e-4);

        // basis value must be positive
        assert!(max > 0.0);
    }

    #[test]
    fn test_basis_maximum_cubic() {
        // Cubic B-spline (p=3)
        let kv = KnotVector::new(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0, 4.0,
        ])
        .unwrap();

        let p = 3;
        let i = 3;

        let (max, u) = on_compute_basis_maximum(&kv, i, p, 1e-8).unwrap();

        println!("max = {}, u = {}", max, u);
        // finite difference check
        let f = |x: f64| {
            let mut nd = [0.0; 3];
            on_compute_basis_and_derivatives(&kv, i, p, x, Side::Left, 0, &mut nd).unwrap();
            nd[0]
        };

        let h = 1e-6;
        let numeric_deriv = (f(u + h) - f(u - h)) / (2.0 * h);

        assert!(numeric_deriv.abs() < 1e-4);
        assert!(max > 0.0);
    }

    fn brute_force_max(
        kv: &KnotVector,
        i: Index,
        p: Degree,
        samples: usize,
    ) -> Result<(Real, Real), NurbsError> {
        let u = kv.as_slice();
        let p_us = p as usize;
        let m = u.len() - 1;

        let dom_l = u[p_us];
        let dom_r = u[m - p_us];

        let left_support = u[i as usize].max(dom_l);
        let right_support = u[i as usize + p_us + 1].min(dom_r);

        let mut nd = [0.0_f64; 3];

        let mut best_val = -1.0;
        let mut best_u = left_support;

        for s in 0..=samples {
            let t = s as Real / samples as Real;
            let uu = left_support + (right_support - left_support) * t;
            on_compute_basis_and_derivatives(kv, i, p, uu, Side::Left, 0, &mut nd)?;
            if nd[0] > best_val {
                best_val = nd[0];
                best_u = uu;
            }
        }

        Ok((best_val, best_u))
    }

    #[test]
    fn kntmax_linear_special_case() -> Result<(), NurbsError> {
        // p=1이면 max=1, u=U[i+1]
        let kv = KnotVector::from_vec(vec![0.0, 0.0, 1.0, 2.0, 2.0]);
        let (mx, uu) = on_compute_basis_maximum(&kv, 1 as Index, 1 as Degree, 1e-12)?;
        assert!((mx - 1.0).abs() < 1e-14);
        assert!((uu - kv.as_slice()[2]).abs() < 1e-14);
        Ok(())
    }

    #[test]
    fn kntmax_multiple_knot_special_case() -> Result<(), NurbsError> {
        // p-fold inner multiple knot: 예시로 p=3, 내부에 U[i+1]==U[i+2]==U[i+3]
        // i=0: U[1]==U[2]==U[3]이면 s==p 이므로 max=1, u=U[i+1]
        let kv = KnotVector::from_vec(vec![0.0, 1.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0]);
        let (mx, uu) = on_compute_basis_maximum(&kv, 0 as Index, 3 as Degree, 1e-12)?;
        assert!((mx - 1.0).abs() < 1e-14);
        assert!((uu - kv.as_slice()[1]).abs() < 1e-14);
        Ok(())
    }

    #[test]
    fn kntmax_matches_bruteforce_clamped_like() -> Result<(), NurbsError> {
        // degree 3, 일반적인 knot (clamped-like)
        // m=10, p=3 => n = m-p-1 = 6 (basis 0..6)
        let kv = KnotVector::from_vec(vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0]);

        let i: Index = 2;
        let p: Degree = 3;
        let tol = 1e-12;

        let (mx, umx) = on_compute_basis_maximum(&kv, i, p, tol)?;

        // brute max
        let (mx_b, _u_b) = brute_force_max(&kv, i, p, 20000)?;

        // 1) 값이 brute-force 최대에 가깝다
        assert!((mx_b - mx).abs() < 5e-7, "mx={}, brute={}", mx, mx_b);

        // 2) 근방에서 도함수 거의 0
        let mut nd = [0.0_f64; 3];
        on_compute_basis_and_derivatives(&kv, i, p, umx, Side::Left, 2, &mut nd)?;
        assert!(nd[1].abs() < 1e-8, "dN/du too large: {}", nd[1]);

        // 3) 값이 0 이상
        assert!(mx >= -ON_TOL9);

        Ok(())
    }
}
```
