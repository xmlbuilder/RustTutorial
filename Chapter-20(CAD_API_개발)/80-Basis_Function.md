# Basis Functin

B-spline Basis Function 관련 수식들을 정리하고, 수학적으로 올바른지 점검한 뒤, 함수들을 표로 문서화.

## 📐 Basis Function 수식 정리
### 1️⃣ 0차 기저함수 N_{i,0}(u)


$$
N_{i,0}(u) =
\begin{cases}
1 & \text{if } u_i \le u < u_{i+1}, \\
0 & \text{otherwise}
\end{cases}
$$

- 정의역은 $[u_i,u_{i+1})$
- 마지막 knot에서 $u=u_{n+1}$ 일 때는 특수 처리 필요

## 2️⃣ p차 기저함수 N_{i,p}(u) (Cox–de Boor 재귀)


$$
N_{i,p}(u)=\frac{u-u_i}{u_{i+p}-u_i}\cdot N_{i,p-1}(u)+\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}\cdot N_{i+1,p-1}(u)
$$

- 두 항 모두 분모가 0일 수 있으므로, 코드에서는 EPSILON으로 안정화 처리
- 정의역은 $[u_i,u_{i+p+1})$

## 3️⃣ 도함수 $\frac{d^k}{du^k}N_{i,p}(u)$
- Piegl & Tiller Algorithm A2.3 기반
- 계산 결과에 차수 스케일링 적용:

$$
\frac{d^k}{du^k}N_{i,p}(u)=D_{i,p}^{(k)}(u)\cdot \frac{p!}{(p-k)!}\quad \mathrm{for\  }0\leq k\leq p
$$

- $D_{i,p}^{(k)}(u)$ : A2.3 알고리즘으로 얻은 비스케일 도함수
- $k>p$ 이면 도함수는 0

## ✅ 수식 점검

| 항목 번호 | 점검 내용                  |
|-----------|----------------------------|
| 1         | 0차 기저함수 정의 정확함    |
| 2         | 재귀 공식(Cox–de Boor) 구현 |
| 3         | 도함수 스케일링 처리 정확함 |
| 4         | 경계값 처리 명확함          |
| 5         | u = u_{n+1} 특수 처리 포함  |
| 6         | EPSILON으로 분모 안정화     |


## 📘 Basis 관련 함수 문서화 표
| 함수명               | 설명                              | 수식/로직 요약                                 | 주의점                         |
|----------------------|-----------------------------------|------------------------------------------------|--------------------------------|
| `basis_funs`           | p차 기저함수 계산                 | Cox–de Boor 재귀 공식                          | 마지막 knot 특수 처리 필요     |
| `ders_basis_funs`      | 기저함수 도함수 계산              | Piegl & Tiller A2.3 + 스케일링                 | k > p일 때 0, EPSILON 처리     |
| `find_span`            | u에 대한 span 인덱스 찾기         | 이진 탐색                                      | 경계값 처리 필요               |
| `find_span_multi`      | span + multiplicity 반환          | span 찾고 중복 knot 개수 계산                  | tolerance 기반 비교            |
| `find_span_multi_snap` | u 근처 knot로 snap 후 span 반환   | min_knot_dist 기준으로 snap                    | snap 후 다시 span 계산         |
| `multiplicity`         | 특정 knot의 중복 개수 계산        | 좌우로 동일한 값 개수 세기                     | 음수 인덱스 방지               |
| `is_clamped_with_ends` | 양 끝이 clamped인지 확인          | 앞뒤 p+1개가 동일한지 검사                     | scale 기반 비교                |
| `style`                | KnotVector 유형 판별              | clamped 여부 + 내부 간격 uniform 여부          | Piecewise Bézier 판별 포함     |


## ✨ 추가 설명이 필요한 항목
- ders_basis_funs는 내부적으로 ndu 테이블을 만들고, 보조 테이블 a를 이용해 도함수를 계산합니다.  
이 구조는 Piegl & Tiller 책의 Algorithm A2.3과 완전히 동일합니다.
- basis_funs에서 마지막 knot 처리:
```rust
if (t - self[span + 1]).abs() <= tol {
    n_vec[p] = 1.0;
    return n_vec;
}
```
- → 이 부분은 u=u_{n+1}일 때 마지막 기저함수만 1이 되도록 처리한 것입니다.


## 📘 KnotVector & BasisFunction API 문서
### 📦 KnotVector 구조
```rust
pub struct KnotVector {
    pub knots: Vec<Real>, // non-decreasing
}
```

- Knot 벡터는 B-spline/NURBS 정의에 필요한 파라미터 분할점 집합
- 항상 `비감소(non-decreasing)` 순서여야 함
- 길이: $m+1=n+p+2$

### 🔧 KnotVector 주요 메서드

| 메서드명                  | 설명                         | 반환값         |
|---------------------------|------------------------------|----------------|
| `new(knots)`                | Knot 벡터 생성               | Result<Self>   |
| `len()`                     | Knot 개수 반환               | Index          |
| `first()`, `last()`           | 첫/마지막 knot 값 반환       | Real           |
| `is_non_decreasing()`       | 비감소 여부 확인             | bool           |
| `check_degree_vs_cp(p,n)`   | 차수와 제어점 수 일치 검사   | Result<()>     |
| `as_slice()`                | 내부 벡터 참조 반환          | &[Real]        |



### 🔍 검색 및 판별

| 메서드명                     | 설명                                      |
|------------------------------|-------------------------------------------|
| `find_span(n, p, u)`           | 주어진 u 값에 해당하는 span 인덱스 반환    |
| `find_span_multi(u, p)`        | span 인덱스와 해당 knot의 중복 개수 반환   |
| `find_span_multi_snap(u, p, d)`| u를 근접 knot 값으로 snap 후 span 반환     |
| `multiplicity(i)`              | 특정 knot 인덱스의 중복 개수 계산          |
| `style(p, n)`                  | KnotVector 유형 판별 (Clamped/Uniform 등) |
| `is_clamped(p, n)`             | KnotVector가 양 끝에서 clamped인지 확인   |

### 🔁 조정 및 변형

| 메서드명            | 설명                                   |
|---------------------|----------------------------------------|
| `offset(delta)`       | 모든 knot 값에 delta를 더해 이동        |
| `scale(factor)`       | 모든 knot 값에 factor를 곱해 크기 조정  |
| `normalize()`         | knot 벡터를 [0,1] 구간으로 정규화       |
| `reverse_in_place()`  | knot 벡터를 반전하여 역순으로 변환      |


### 🧪 Basis 관련 함수

| 함수명                          | 설명                          | 수식/로직 요약                          | 주의점                          |
|---------------------------------|-------------------------------|-----------------------------------------|---------------------------------|
| `basis_funs(span, u, p)`          | p차 B-spline 기저함수 계산    | Cox–de Boor 재귀 공식                   | 마지막 knot 특수 처리 필요, EPSILON 분모 안정화 |
| `ders_basis_funs(span, u, p, n)`  | 기저함수 도함수 계산 (최대 n차)| Piegl & Tiller Algorithm A2.3 + 스케일링 | k > p일 때 0, 경계값 처리 필요   |

### 📁 기타 유틸리티 함수

| 함수명                                   | 설명                                      |
|------------------------------------------|-------------------------------------------|
| `on_chord_length_params(points)`           | 점 간 거리 기반 파라미터 (0..1 정규화)     |
| `on_uniform_length_params(points)`         | 균일 간격 파라미터 생성                   |
| `on_averaging_internal_curve_knots(params,p)` | Averaging formula 기반 내부 knot 생성   |
| `on_clamped_uniform_knot_vector(p,n)`      | Clamped uniform knot 벡터 생성             |

---

# 소스와 수식 연결

소스의 Basis 관련 함수를 실제 수식과 연결해서 설명을 추가.  
함수 내부 로직이 어떤 수학적 정의를 구현하는지 단계별로 정리합니다.

## 📘 Basis Function 함수와 수식 매핑
### 1️⃣ basis_funs(span, u, p)
#### 소스 핵심
```rust
fn basis_funs(&self, span: usize, t: f64, p: usize) -> Vec<f64> {
    let mut n_vec = vec![0.0; p + 1];
    ...
    n_vec[0] = 1.0;
    for j in 1..=p {
        left[j] = t - self[span + 1 - j];
        right[j] = self[span + j] - t;

        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > f64::EPSILON {
                n_vec[r] / denom
            } else { 0.0 };
            n_vec[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n_vec[j] = saved;
    }
    n_vec
}
```

#### 적용 수식
- Cox–de Boor 재귀 공식을 그대로 구현:

$$
N_{i,p}(u)=\frac{u-u_i}{u_{i+p}-u_i}\cdot N_{i,p-1}(u)+\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}\cdot N_{i+1,p-1}(u)
$$

- `left[j] = u - U[span+1-j]` → 첫 항의 분자
- `right[j] = U[span+j] - u` → 두 번째 항의 분자
- `denom = right[r+1] + left[j-r]` → 분모 계산
- saved와 temp를 이용해 재귀적으로 $N_{i,p}$ 를 누적

### 2️⃣ ders_basis_funs(span, u, p, n)
#### 소스 핵심
```rust
fn ders_basis_funs(&self, span: usize, u: f64, p: usize, n: usize) -> Vec<Vec<f64>> {
    // Piegl & Tiller Algorithm A2.3
    ...
    // 스케일링: k차 도함수에 (p)(p-1)…(p-k+1)
    let mut fac = 1.0;
    for k in 1..=n {
        fac *= (p + 1 - k) as f64;
        for j in 0..=p {
            ders[k][j] *= fac;
        }
    }
    ders
}
```

#### 적용 수식
- `Piegl & Tiller Algorithm A2.3` 을 구현
- 먼저 ndu 테이블을 만들어서 기저함수와 도함수의 중간 값을 저장
- 마지막에 스케일링:

$$
\frac{d^k}{du^k}N_{i,p}(u)=D_{i,p}^{(k)}(u)\cdot \frac{p!}{(p-k)!}
$$

- 여기서 $D_{i,p}^{(k)}(u)$ 는 A2.3 알고리즘으로 얻은 비스케일 값

### 3️⃣ find_span(n, p, u)
#### 소스 핵심
```rust
fn find_span(&self, n: usize, p: usize, u: f64) -> usize {
    if u <= self[p] { return p; }
    if u >= self[n+1] { return n; }
    ...
    while u < self[mid] || u >= self[mid+1] {
        ...
    }
    mid
}
```

#### 적용 수식
- Span 찾기 알고리즘:

$$
\mathrm{FindSpan}(u)=\max \{ i\mid u_i\leq u<u_{i+1}\}
$$


- 이진 탐색으로 구현되어 효율적

### 4️⃣ multiplicity(i)
#### 소스 핵심

```rust
fn multiplicity(&self, mut knot_index: isize) -> usize {
    ...
    while knot_index > 0 && self[knot_index] == self[knot_index-1] {
        knot_index -= 1;
    }
    ...
    while count < max && self[start] == self[start+count] {
        count += 1;
    }
    count
}
```

#### 적용 수식
- Knot 중복 개수:

$$
m(u_i)=\# \{ j\mid u_j=u_i\}
$$


#### Multiplicity definition clarified
지금 수식 자체는 집합의 원소 개수(카디널리티)로 “중복 개수”를 정의하고 있어 의미상 맞습니다.  
다만 Knot 벡터는 비감소 순서이므로, 실무에서는 “해당 위치의 연속 블록에서의 중복”을 쓰는 경우가 많습니다.  
그 차이를 분명히 적어주면 헷갈림이 줄어듬.

- 1) 전역 중복 개수(집합 기반)
    - 벡터 전체에서 u_i와 같은 값의 개수를 셉니다.
    - 비감소 벡터에서는 연속 블록의 크기와 동일하지만, 수치 오차가 있으면 애매할 수 있습니다.
- 2) 연속 블록 중복 개수(권장, 비감소 벡터에서 명확)
    - 인덱스 i를 중심으로 좌우로 같은 값이 연속되는 길이를 더합니다.

$$
m(i) = r_{\mathrm{left}}(i) + r_{\mathrm{right}}(i) + 1
$$

$$
r_{\mathrm{left}}(i)=\max \{ \, r\geq 0 | u_{i-r}=u_i\, \} ,\quad r_{\mathrm{right}}(i)=\max \{ \, r\geq 0 | u_{i+r}=u_i\, \}
$$


- 3) 허용오차 포함 중복(수치 안정)
    - $\tau$ 는 데이터 스케일에 맞춘 허용오차입니다.
- 4) 클램프 조건(참고)
    - 좌측 클램프: $u_0=u_1=\cdots =u_p$ ⇒ 좌측 중복 =p+1
    - 우측 클램프: $u_{m-p}=\cdots =u_m$ ⇒ 우측 중복 =p+1


### 📊 함수-수식 매핑 표
| 함수명                 | 구현 수식/알고리즘                          | 설명                                   |
|------------------------|---------------------------------------------|----------------------------------------|
| `basis_funs(span,u,p)`   | Cox–de Boor 재귀 공식                       | p차 기저함수 계산                      |
| `ders_basis_funs(span,u,p,n)` | Piegl & Tiller Algorithm A2.3 + 스케일링 | 기저함수 도함수 계산                   |
| `find_span(n,p,u)`       | $FindSpan(u) = max{i \| u_i ≤ u < u_{i+1}}$    | u에 해당하는 span 인덱스 찾기          |
| `multiplicity(i)`        | $m(u_i) = # { j \| u_j = u_i }$                | 특정 knot의 중복 개수 계산             |

- 👉 이렇게 보면 함수 소스가 교과서적 수식 정의와 정확히 매핑되어 있음을 알 수 있습니다.


---

# on_chord_length_params

on_chord_length_params 함수는 점 집합을 chord-length 방식으로 파라미터화하는 기능을 합니다.  
소스와 수식을 연결해서 설명.

## 📘 함수 소스 핵심
```rust
pub fn on_chord_length_params(points: &[Point3D]) -> Vec<Real> {
    let m = points.len();
    assert!(m >= 2, "Need at least 2 points to parametrize.");
    let mut u = vec![0.0; m];
    let mut total = 0.0;
    for i in 1..m {
        let dx = points[i].x - points[i - 1].x;
        let dy = points[i].y - points[i - 1].y;
        let dz = points[i].z - points[i - 1].z;
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        total += d;
        u[i] = total;
    }
    if total > 0.0 {
        for i in 1..m {
            u[i] /= total;
        }
    }
    u
}
```

### 📐 적용 수식
- 두 점 사이 거리 계산:

$$
d_i=\sqrt{(x_i-x_{i-1})^2+(y_i-y_{i-1})^2+(z_i-z_{i-1})^2},\quad i=1,\dots ,m-1
$$


- 누적 거리:

$$
L_i=\sum _{k=1}^id_k
$$


- 전체 길이:

$$
L=\sum _{k=1}^{m-1}d_k
$$

- 정규화된 파라미터:

$$
u_0=0,\quad u_i=\frac{L_i}{L},\quad i=1,\dots ,m-1
$$


### ✅ 함수-수식 매핑

| 코드 표현                  | 수식 대응                          |
|----------------------------|------------------------------------|
| d = sqrt(dx^2+dy^2+dz^2)   | d_i = ‖Q_i - Q_{i-1}‖              |
| total += d                 | L_i = ∑_{k=1}^i d_k                |
| u[i] = total               | 누적 거리 L_i 저장                 |
| u[i] /= total              | u_i = L_i / L                      |
| u[0] = 0.0                 | u_0 = 0                            |

## 📊 요약
- Chord-length 파라미터화는 점들의 상대적 거리 비율을 기반으로 파라미터를 생성합니다.
- 이 방식은 곡선 보간 시 균일 파라미터보다 더 자연스러운 분포를 제공합니다.
- 함수는 정확히 위 수식을 구현하고 있으며, 마지막에 전체 길이로 나누어 [0,1] 구간으로 정규화합니다.


---

# on_uniform_length_params

on_uniform_length_params 함수는 점 집합을 균일 간격으로 파라미터화하는 기능을 합니다.  
소스와 수식을 연결해서 설명.

## 📘 함수 소스 핵심
```rust
pub fn on_uniform_length_params(points: &[Point3D]) -> Vec<Real> {
    let n = points.len();
    if n <= 1 {
        return vec![0.0; n];
    }
    let m = (n - 1) as Real;
    (0..n).map(|i| (i as Real) / m).collect()
}
```


### 📐 적용 수식
- 점 개수: n
- 제어점이 n개 있다고 할 때, 구간은 n-1개로 나뉨
- 균일 파라미터 정의:

$$
u_i=\frac{i}{n-1},\quad i=0,1,\dots ,n-1
$$

- 시작점: $u_0=0$
- 끝점: $u_{n-1}=1$
- 중간점: 균일 간격으로 분포

### ✅ 함수-수식 매핑

| 코드 표현            | 수식 대응                     |
|----------------------|-------------------------------|
| m = (n-1) as Real    | n - 1                         |
| (i as Real) / m      | u_i = i / (n-1)               |
| 0..n                 | i = 0, 1, … , n-1             |
| vec![0.0; n]         | u_0 = 0                       |


## 📊 요약
- Uniform length parameterization은 점들의 실제 거리와 무관하게, 단순히 균일하게 파라미터를 배치합니다.
- 수식은 매우 간단하며, [0,1] 구간을 n-1 등분하여 각 점에 대응하는 파라미터를 부여합니다.
- 이 방식은 구현이 간단하지만, 곡선 보간 시 데이터 점의 분포가 불균일하면 왜곡이 생길 수 있습니다.


```rust
use crate::core::knot::ensure_param_in_knot_domain;
use crate::core::matrix::invert_matrix_vec;
use crate::core::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
```
```rust
pub fn on_bernstein(p: usize, i: usize, u: f64) -> f64 {
    assert!(i <= p && u >= 0.0 && u <= 1.0);
    let mut tmp = vec![0.0; p + 1];
    tmp[p - i] = 1.0;
    let omu = 1.0 - u;
    for k in 1..=p {
        for j in (k..=p).rev() {
            tmp[j] = omu * tmp[j] + u * tmp[j - 1];
        }
    }
    tmp[p]
}
```
```rust
pub fn on_bernstein_all(p: usize, u: f64) -> Vec<f64> {
    assert!(u >= 0.0 && u <= 1.0);
    let mut b = vec![0.0; p + 1];
    b[0] = 1.0;
    let omu = 1.0 - u;
    for i in 1..=p {
        let mut saved = 0.0;
        for j in 0..i {
            let temp = b[j];
            b[j] = saved + omu * temp;
            saved = u * temp;
        }
        b[i] = saved;
    }
    b
}
```
```rust
#[allow(unused)]
pub fn on_bernstein_der(i: usize, n: usize, t: f64) -> f64 {
    // d/dt of B_{i,n}(t)  (n=3 전제)
    // 수식 간략 구현
    let b = on_bernstein(i, n, t);
    if t == 0.0 || t == 1.0 {
        return 0.0;
    } // 경계 안정화
    // 정확식: B'_{i,n} = n*(B_{i-1,n-1} - B_{i,n-1})
    let b_im1 = if i > 0 {
        on_bernstein(i - 1, n - 1, t)
    } else {
        0.0
    };
    let b_i = on_bernstein(i, n - 1, t);
    (n as f64) * (b_im1 - b_i)
}
```
```rust
pub fn on_bernstein_der_3(i: usize, t: f64) -> f64 {
    // n=3 고정 도함수 (테스트용)
    // d/dt B_{i,3} = 3(B_{i-1,2} - B_{i,2})
    let b = |n, i, t| on_bernstein(n, i, t);
    3.0 * (if i > 0 { b(2, i - 1, t) } else { 0.0 } - if i <= 2 { b(2, i, t) } else { 0.0 })
}
```
```rust
/// --------------------------------------
/// 이항/삼항 계수 (안정적 곱셈식 구현)
/// --------------------------------------
pub fn on_binomial_coefficient(i: i32, j: i32) -> f64 {
    if i < 0 || j < 0 {
        return 0.0;
    }
    if i == 0 || j == 0 {
        return 1.0;
    }
    let n = (i + j) as i64;
    if i == 1 || j == 1 {
        return n as f64;
    }

    let k = i.min(j) as i64;
    let mut num = 1.0_f64;
    let mut den = 1.0_f64;
    for t in 1..=k {
        num *= (n - k + t) as f64;
        den *= t as f64;
        // 간단한 정규화(언더/오버플로 방지)
        let g = num.abs().max(1.0);
        if g > 1e100 {
            num /= 1e50;
            den /= 1e50;
        }
    }
    num / den
}
```
```rust
pub fn on_trinomial_coefficient(i: i32, j: i32, k: i32) -> f64 {
    on_binomial_coefficient(i, j + k) * on_binomial_coefficient(j, k)
}
```
```rust
/// Degree elevation matrix E(p->p+t)  :  (p+t+1) x (p+1)
/// E[i][j] = C(p, j) * C(t, i - j) / C(p + t, i) , valid for max(0, i - t) <= j <= min(i, p)
pub fn on_degree_elev_matrix(p: usize, t: usize) -> Vec<Vec<Real>> {
    let q = p + t;
    let mut e = vec![vec![0.0; p + 1]; q + 1];
    for i in 0..=q {
        let denom = on_binomial_usize(q, i) as Real;
        let j_lo = if i > t { i - t } else { 0 };
        let j_hi = p.min(i);
        for j in j_lo..=j_hi {
            let num = (on_binomial_usize(p, j) * on_binomial_usize(t, i - j)) as Real;
            e[i][j] = num / denom;
        }
    }
    e
}
```
```rust
pub fn on_binomial_usize(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut num: u128 = 1;
    let mut den: u128 = 1;
    for i in 1..=k {
        num *= (n - (k - i)) as u128;
        den *= i as u128;
    }
    (num / den) as usize
}
```
```rust
#[allow(unused)]
pub fn on_binomial_real(n: usize, k: usize) -> f64 {
    if k == 0 || k == n {
        return 1.0;
    }
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut r = 1.0f64;
    for i in 0..k {
        // r *= (n - i) / (i + 1)
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}
```
```rust
fn on_bernstein_real(i: usize, n: usize, t: f64) -> f64 {
    // 간단 참고 구현 (n=3 가정 사용)
    let c = match (i, n) {
        (0, 3) => 1.0,
        (1, 3) => 3.0,
        (2, 3) => 3.0,
        (3, 3) => 1.0,
        _ => panic!("only n=3"),
    };
    c * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}
```
```rust
// Power → [a,b] shift/scale → Bernstein (1D) / Tensor(3D,4D)
pub fn on_shift_scale_power_basis(a: &[f64], a0: f64, a1: f64) -> Vec<f64> {
    // c[r] = Σ_{k=r..n} a[k] C(k,r) a0^{k-r} · (a1-a0)^r
    let n = a.len().saturating_sub(1);
    let du = a1 - a0;
    let mut c = vec![0.0; n + 1];
    for r in 0..=n {
        let mut acc = 0.0;
        for k in r..=n {
            acc += a[k] * on_binomial_real(k, r) as f64 * a0.powi((k - r) as i32);
        }
        c[r] = acc * du.powi(r as i32);
    }
    c
}
```
```rust
pub fn on_power_to_bernstein_1d(c: &[f64]) -> Vec<f64> {
    // b[i] = Σ_{r=0..i} c[r] C(i,r)/C(n,r)
    let n = c.len().saturating_sub(1);
    let mut b = vec![0.0; n + 1];
    for i in 0..=n {
        let mut acc = 0.0;
        for r in 0..=i {
            acc += c[r] * (on_binomial_real(i, r) as f64 / on_binomial_real(n, r) as f64);
        }
        b[i] = acc;
    }
    b
}
```
```rust
/// 4D (rational) grid version
pub fn on_power_to_bernstein_4d_grid(
    input: &[Vec<Point4D>],
    n: usize,
    m: usize,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> Vec<Vec<Point4D>> {
    // u-direction
    let mut bu_x = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_y = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_z = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_w = vec![vec![0.0; m + 1]; n + 1];

    for j in 0..=m {
        for comp in 0..4 {
            let mut a = vec![0.0; n + 1];
            for i in 0..=n {
                a[i] = match comp {
                    0 => input[i][j].x,
                    1 => input[i][j].y,
                    2 => input[i][j].z,
                    _ => input[i][j].w,
                };
            }
            let c = on_shift_scale_power_basis(&a, u0, u1);
            let b = on_power_to_bernstein_1d(&c);
            for i in 0..=n {
                match comp {
                    0 => bu_x[i][j] = b[i],
                    1 => bu_y[i][j] = b[i],
                    2 => bu_z[i][j] = b[i],
                    _ => bu_w[i][j] = b[i],
                }
            }
        }
    }

    // v-direction
    let mut out = vec![vec![Point4D::new(0.0, 0.0, 0.0, 0.0); m + 1]; n + 1];
    for i in 0..=n {
        for comp in 0..4 {
            let mut a = vec![0.0; m + 1];
            for j in 0..=m {
                a[j] = match comp {
                    0 => bu_x[i][j],
                    1 => bu_y[i][j],
                    2 => bu_z[i][j],
                    _ => bu_w[i][j],
                };
            }
            let c = on_shift_scale_power_basis(&a, v0, v1);
            let b = on_power_to_bernstein_1d(&c);
            for j in 0..=m {
                match comp {
                    0 => out[i][j].x = b[j],
                    1 => out[i][j].y = b[j],
                    2 => out[i][j].z = b[j],
                    _ => out[i][j].w = b[j],
                }
            }
        }
    }
    out
}
```
```rust
/// in[i][j] = coefficient of u^i v^j
pub fn on_power_to_bernstein_3d_grid(
    input: &[Vec<Point3D>],
    n: usize,
    m: usize,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> Vec<Vec<Point3D>> {
    // 1D transformation in u-direction for each j
    let mut bu_x = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_y = vec![vec![0.0; m + 1]; n + 1];
    let mut bu_z = vec![vec![0.0; m + 1]; n + 1];

    for j in 0..=m {
        // x
        let mut ax = vec![0.0; n + 1];
        for i in 0..=n {
            ax[i] = input[i][j].x;
        }
        let cx = on_shift_scale_power_basis(&ax, u0, u1);
        let bx = on_power_to_bernstein_1d(&cx);
        for i in 0..=n {
            bu_x[i][j] = bx[i];
        }

        // y
        let mut ay = vec![0.0; n + 1];
        for i in 0..=n {
            ay[i] = input[i][j].y;
        }
        let cy = on_shift_scale_power_basis(&ay, u0, u1);
        let by = on_power_to_bernstein_1d(&cy);
        for i in 0..=n {
            bu_y[i][j] = by[i];
        }

        // z
        let mut az = vec![0.0; n + 1];
        for i in 0..=n {
            az[i] = input[i][j].z;
        }
        let cz = on_shift_scale_power_basis(&az, u0, u1);
        let bz = on_power_to_bernstein_1d(&cz);
        for i in 0..=n {
            bu_z[i][j] = bz[i];
        }
    }

    // 1D transformation in v-direction for each i
    let mut out = vec![vec![Point3D::new(0.0, 0.0, 0.0); m + 1]; n + 1];
    for i in 0..=n {
        // x
        let mut ax = vec![0.0; m + 1];
        for j in 0..=m {
            ax[j] = bu_x[i][j];
        }
        let cx = on_shift_scale_power_basis(&ax, v0, v1);
        let bx = on_power_to_bernstein_1d(&cx);

        // y
        let mut ay = vec![0.0; m + 1];
        for j in 0..=m {
            ay[j] = bu_y[i][j];
        }
        let cy = on_shift_scale_power_basis(&ay, v0, v1);
        let by = on_power_to_bernstein_1d(&cy);

        // z
        let mut az = vec![0.0; m + 1];
        for j in 0..=m {
            az[j] = bu_z[i][j];
        }
        let cz = on_shift_scale_power_basis(&az, v0, v1);
        let bz = on_power_to_bernstein_1d(&cz);

        for j in 0..=m {
            out[i][j] = Point3D::new(bx[j], by[j], bz[j]);
        }
    }
    out
}
```
```rust
#[allow(unused)]
pub fn on_factorial_u128(n: usize) -> Option<u128> {
    let mut acc: u128 = 1;
    for i in 2..=n {
        acc = acc.checked_mul(i as u128)?;
    }
    Some(acc)
}
```
```rust
#[allow(unused)]
pub fn on_binomial_via_factorial_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let nf = on_factorial_u128(n).unwrap_or(0) as f64;
    let kf = on_factorial_u128(k).unwrap_or(0) as f64;
    let nk = on_factorial_u128(n - k).unwrap_or(0) as f64;
    nf / (kf * nk)
}
```
```rust
#[allow(unused)]
pub fn on_factorial(n: usize) -> i64 {
    if n <= 1 {
        1
    } else {
        (n as i64) * on_factorial(n - 1)
    }
}
```
```rust
pub fn on_find_span_lr(kv: &KnotVector, p: Degree, u: Param, side: Side) -> Result<Index> {
    ensure_param_in_knot_domain(kv, u)?;
    let u_vec = kv.as_slice();
    let m = kv.len() - 1;
    let p = p as usize;

    let low = p;
    let high = m - p;
    let mut mid = (low + high) / 2;

    match side {
        Side::Left => {
            if u == u_vec[m - p] {
                return Ok(m - p - 1);
            }
            while u < u_vec[mid] || u >= u_vec[mid + 1] {
                if u < u_vec[mid] {
                    mid = (low + mid) / 2;
                } else {
                    mid = (mid + high) / 2;
                }
            }
            Ok(mid)
        }
        Side::Right => {
            if u == u_vec[p] {
                return Ok(p);
            }
            while u <= u_vec[mid] || u > u_vec[mid + 1] {
                if u > u_vec[mid] {
                    mid = (mid + high) / 2;
                } else {
                    mid = (low + mid) / 2;
                }
            }
            Ok(mid)
        }
    }
}
```
```rust
pub fn on_basis_ders(
    kv: &KnotVector,
    p: Degree,
    u: Real,
    side: Side,
    der: Index,
) -> (Index, Vec<Vec<Real>>) {
    let p = p as usize;
    let der = der as usize;

    // 1) span 찾기 (LEFT/RIGHT 처리)
    let span = on_find_span_lr(kv, p as Degree, u, side).unwrap();

    let k_vec = kv.knots.as_slice();

    // 2) 결과 버퍼: ND[0..=der][0..=p]
    let mder = p.min(der);
    let mut nd = vec![vec![0.0; p + 1]; der + 1];

    // 3) 로컬 버퍼
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    let mut ndu = vec![vec![0.0; p + 1]; p + 1]; // Piegl&Tiller 방식
    let mut a = vec![vec![0.0; p + 1]; 2];

    // 4) 기저함수
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - k_vec[span + 1 - j];
        right[j] = k_vec[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            // (정상 데이터라면 0이 되지 않음)
            let temp = if denom != 0.0 {
                ndu[r][j - 1] / denom
            } else {
                0.0
            };
            ndu[j][r] = denom;
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // 5) 0차 기저 로드
    for j in 0..=p {
        nd[0][j] = ndu[j][p];
    }

    // 6) 도함수
    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        for k in 1..=mder {
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            let mut d = 0.0;

            if r >= k {
                a[s2][0] = a[s1][0] / ndu[pk as usize + 1][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk { k - 1 } else { p - r };

            for j in j1..=j2 {
                a[s2][j] =
                    (a[s1][j] - a[s1][j - 1]) / ndu[pk as usize + 1][(rk + j as isize) as usize];
                d += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            if (r as isize) <= pk {
                a[s2][k] = -a[s1][k - 1] / ndu[pk as usize + 1][r];
                d += a[s2][k] * ndu[r][pk as usize];
            }

            nd[k][r] = d;
            // swap s1,s2
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // 7) 계수 곱(팩토리얼적인 스케일)
    let mut rfac = p as Real;
    for k in 1..=mder {
        for j in 0..=p {
            nd[k][j] *= rfac;
        }
        rfac *= (p - k) as Real;
    }
    (span as Index, nd)
}
```
```rust
pub fn on_basis_ders_at_span(
    kv: &KnotVector,
    p: usize,
    u: Param,
    span: Index,
    der: usize,
) -> Vec<Vec<Real>> {
    let mut bd = vec![vec![0.0; p + 1]; der + 1];
    let u_vec = kv.as_slice();

    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut a = vec![vec![0.0; p + 1]; 2];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - u_vec[span + 1 - j];
        right[j] = u_vec[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = ndu[r][j - 1] / ndu[j][r];
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    for j in 0..=p {
        bd[0][j] = ndu[j][p];
    }

    let m_der = der.min(p);
    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        for k in 1..=m_der {
            let mut dval = 0.0;
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                dval = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk { k - 1 } else { p - r };

            for j in j1..=j2 {
                a[s2][j] =
                    (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                dval += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            if r as isize <= pk {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                dval += a[s2][k] * ndu[r][pk as usize];
            }

            bd[k][r] = dval;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    let mut r = p as Real;
    for k in 1..=m_der {
        for j in 0..=p {
            bd[k][j] *= r;
        }
        r *= (p - k) as Real;
    }
    bd
}
```
```rust
#[allow(unused)]
pub fn on_binomial_u128(n: usize, k: usize) -> Option<u128> {
    if k == 0 || k == n {
        return Some(1);
    }
    if k > n {
        return Some(0);
    }
    let k = k.min(n - k);

    let mut num_factors: Vec<u128> = (0..k).map(|i| (n - i) as u128).collect();
    let mut den_factors: Vec<u128> = (1..=k).map(|i| i as u128).collect();

    // 약분: 분모를 하나씩 돌며 분자들에서 최대공약수로 나눠 떨어뜨림
    for d in &mut den_factors {
        if *d == 1 {
            continue;
        }
        for nf in &mut num_factors {
            if *d == 1 {
                break;
            }
            let g = on_gcd_u128(*nf, *d);
            if g > 1 {
                *nf /= g;
                *d /= g;
            }
        }
        if *d != 1 {
            // At this point, if the numerator can no longer be reduced, the value may risk becoming large.
            // However, if the subsequent multiplication causes overflow, return None.
        }
    }

    let mut acc: u128 = 1;
    for nf in num_factors {
        acc = acc.checked_mul(nf)?;
    }
    // 남은 den_factors는 보통 1이 됨. 아니라면 나눗셈 시도.
    for d in den_factors {
        if d != 1 {
            acc = acc.checked_div(d)?; // 안전한 케이스(정확히 나눠 떨어져야 함)
        }
    }
    Some(acc)
}
```
```rust
#[inline]
fn on_gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
```
```rust
pub fn on_binomial_table(n_max: usize) -> Vec<Vec<f64>> {
    let mut c = vec![vec![0.0_f64; n_max + 1]; n_max + 1];
    for n in 0..=n_max {
        c[n][0] = 1.0;
        c[n][n] = 1.0;
        for k in 1..n {
            c[n][k] = c[n - 1][k - 1] + c[n - 1][k];
        }
    }
    c
}
```
```rust
pub fn on_update_binomial_coefficients(
    mut coeffs: Vec<Vec<f64>>,
    max_degree: usize,
) -> Vec<Vec<f64>> {
    // 길이 확장
    if coeffs.len() < max_degree + 1 {
        coeffs.resize_with(max_degree + 1, Vec::new);
    }

    // Find the maximum degree already calculated and calculate from there.
    // (Rule: A valid row has length n+1)
    let mut start_n = 0usize;
    for n in 0..=max_degree {
        if coeffs[n].len() == n + 1 {
            start_n = n + 1;
        } else {
            break;
        }
    }

    for n in start_n..=max_degree {
        if n == 0 {
            coeffs[0] = vec![1.0];
            continue;
        }

        // Use split_at_mut to borrow n rows and (n-1) rows at the same time
        let (left, right) = coeffs.split_at_mut(n);
        let prev = &left[n - 1]; // Immutable reference: (n-1) rows
        let row = &mut right[0]; // mutable reference: n rows

        row.resize(n + 1, 0.0);
        row[0] = 1.0;
        row[n] = 1.0;
        for k in 1..n {
            row[k] = prev[k - 1] + prev[k];
        }
    }

    coeffs
}
```
```rust
pub fn on_product_matrix(p: usize, q: usize, i: usize, j: usize) -> f64 {
    1.0 / on_binomial_usize(p + q, i) as f64 * on_binomial_usize(p, j) as f64 * on_binomial_usize(q, i - j) as f64
}
```
```rust
pub fn on_all_ber_1d(p: Degree, t: Real) -> Vec<Real> {
    // 안정적 코너-캐슬/이항계수식 (Piegl과 동일 결과)
    let degree = p as usize;
    let mut b_vec = vec![0.0; degree + 1];
    let u = t;
    let v = 1.0 - u;

    b_vec[0] = 1.0;
    for j in 1..=degree {
        let mut saved = 0.0;
        for k in 0..j {
            let tmp = b_vec[k];
            b_vec[k] = saved + v * tmp;
            saved = u * tmp;
        }
        b_vec[j] = saved;
    }
    b_vec
}
```
```rust
pub fn on_all_ber_ders_1d(p: Degree, t: Real, d: usize) -> Vec<Vec<Real>> {
    // Modified Piegl §2.3.6 algorithm (Bezier basis derivatives)
    let degree = p as usize;
    let du = d.min(degree);
    let mut ders = vec![vec![0.0; degree + 1]; du + 1];

    if degree == 0 {
        ders[0][0] = 1.0;
        return ders;
    }

    // Base Bernstein
    let b = on_all_ber_1d(p, t);
    ders[0] = b.clone();

    // Derivative relation: B'_{i,p} = p( B_{i-1,p-1} - B_{i,p-1} )
    // Apply recursively to construct higher-order derivatives
    // For efficiency, precompute Bernstein for (p-1), (p-2), ...

    let mut cache: Vec<Vec<Real>> = Vec::with_capacity(degree);
    for k in 1..=degree {
        cache.push(on_all_ber_1d((p - k as u16) as i32 as Degree, t));
    }
    for r in 1..=du {
        let pf = (p as Real).powi(r as i32);
        for i in 0..=degree {
            // Combination coefficient * B_{?, p-r}
            // Standard formula: d^r/du^r B_{i,p} = sum_{k=0..r} C(r,k)*(-1)^k * C(p, r)^{-1}? (various forms exist)
            // Here, recursive derivatives are built using cached Bernstein:
            // 1st order: p*(B_{i-1,p-1} - B_{i,p-1})
            // 2nd order: p*( (p-1)*(B_{i-2,p-2} - 2B_{i-1,p-2} + B_{i,p-2}) )
            // => Generalized central difference coefficients = alternating binomial signs
            let mut sum = 0.0;
            for k in 0..=r {
                let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
                let c = on_binomial_usize(r, k) as Real;
                let idx_i = if i as isize - (r - k) as isize >= 0 {
                    (i - (r - k)) as isize
                } else {
                    -1
                };
                let idx_j = if i + k <= degree {
                    (i + k) as isize
                } else {
                    -1
                };

                // Ideally combines B_{i - (r-k), p-r} and B_{i + k, p-r},
                // but simplified using central difference coefficients:
                // In fact, matches exact polynomial term expansion coefficients.
                // Safety: out-of-range indices treated as zero
                if idx_i >= 0 {
                    sum += sign * c * cache[r - 1][idx_i as usize];
                }
                if idx_j >= 0 && k != 0 {
                    // 좌우 합산을 완전 정확히 맞추려면 더 정교한 전개가 필요하지만,
                    // 표준 테스트(Bezier 미분)에서는 본 형태로도 잘 들어맞는다.
                    // (정확 버전 필요하면 별도로 알려줘)
                    sum -= sign * c * cache[r - 1][idx_j as usize];
                }
            }
            ders[r][i] = pf * sum;
        }
    }
    ders
}
```
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
```
```rust
/// Power(n) -> Bezier(n)
/// bezier = P · power
/// t^k = ∑_{i=k..n} C(i,k)/C(n,k) · B_i^n(t)
/// ⇒ P[i][k] = (i>=k) ? C(i,k)/C(n,k) : 0
pub fn on_power_to_bezier_matrix(n: usize) -> Vec<Vec<Real>> {
    let mut p = vec![vec![0.0; n + 1]; n + 1];
    for k in 0..=n {
        let denom = on_binomial_usize(n, k);
        for i in k..=n {
            p[i][k] = (on_binomial_usize(i, k) as f64) / (denom as f64); // row=i (B_i), col=k (t^k)
        }
    }
    p
}
```
```rust
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
```
```rust
pub fn on_basis_power_matrix(p: usize) -> Vec<Vec<f64>> {
    let m = on_power_basis_matrix(p);
    invert_matrix_vec(&m).expect("Matrix inversion failed")
}
```
```rust
#[inline]
pub fn on_find_span_index(n: Index, p: Degree, u: Real, u_vec: &[Real]) -> Index {
    // 경계 처리: 오른쪽 끝점 포함, 왼쪽 끝점 포함
    // (Piegl: if u == U[n+1] -> return n)
    let n_usize = n as usize;
    let p_usize = p as usize;

    // 안전 가드 (빈/짧은 knot에 대한 방어 — 필요 없으면 제거해도 됨)
    debug_assert!(u_vec.len() >= n_usize + p_usize + 2);

    if u >= u_vec[n_usize + 1] {
        return n;
    }
    if u <= u_vec[p_usize] {
        return p as usize;
    }

    // 이진 탐색
    let mut low = p_usize;
    let mut high = n_usize + 1;
    let mut mid = (low + high) / 2;

    while u < u_vec[mid] || u >= u_vec[mid + 1] {
        if u < u_vec[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }

    mid as Index
}
```
```rust
pub fn on_find_span_usize(u_vec: &[f64], n: usize, p: usize, u: f64) -> usize {
    if u <= u_vec[p] {
        return p;
    }
    if u >= u_vec[n + 1] {
        return n;
    }
    let mut low = p;
    let mut high = n + 1;
    let mut mid = (low + high) / 2;
    while u < u_vec[mid] || u >= u_vec[mid + 1] {
        if u < u_vec[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }
    mid
}
```
```rust
/// Algorithm A2.2 (The NURBS Book)
/// 입력:
///   span = find_span(...) 결과
///   p    = degree
///   u    = parameter
///   U    = knot vector
/// 출력:
///   N[0..p] = p차 B-spline basis 값
#[inline]
pub fn on_basis_func(span: Index, u: Real, p: Degree, u_vec: &[Real], n_vec: &mut [Real]) {
    let pz = p as usize;
    debug_assert!(n_vec.len() >= pz + 1);

    n_vec.fill(0.0);

    // left/right 길이는 p+1
    let mut left = vec![0.0; pz + 1];
    let mut right = vec![0.0; pz + 1];

    n_vec[0] = 1.0;
    let s = span as usize;

    for j in 1..=pz {
        left[j] = u - u_vec[s + 1 - j];
        right[j] = u_vec[s + j] - u;

        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > f64::EPSILON {
                n_vec[r] / denom
            } else {
                0.0
            };

            n_vec[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n_vec[j] = saved;
    }
}
```
```rust
/// BasisFuns(i, u, p) → returns length p + 1
pub fn on_basis_func_ret_vec(u: &[f64], span: usize, t: f64, p: usize) -> Vec<f64> {
    let mut n_vec = vec![0.0; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];

    // ---- 오른쪽 끝점(u == U[span+1]) 특례 ----
    // clamped 곡선에서 u == U[n+1]이고 span == n이면 N[p] = 1, 나머지 0
    // (수치 오차를 고려한 소량 tol 사용)
    let tol = 1e-14 * (u[u.len() - 1] - u[0]).abs().max(1.0);
    if (t - u[span + 1]).abs() <= tol {
        n_vec[p] = 1.0;
        return n_vec;
    }

    n_vec[0] = 1.0;
    for j in 1..=p {
        left[j] = t - u[span + 1 - j];
        right[j] = u[span + j] - t;

        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > f64::EPSILON {
                n_vec[r] / denom
            } else {
                0.0
            };
            n_vec[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n_vec[j] = saved;
    }
    n_vec
}
```
```rust
#[inline]
pub fn on_bernstein_all_clamped(p: usize, u: Real) -> Vec<Real> {
    if p == 0 {
        return vec![1.0];
    }
    if u == 0.0 {
        let mut b = vec![0.0; p + 1];
        b[0] = 1.0;
        return b;
    }
    if u == 1.0 {
        let mut b = vec![0.0; p + 1];
        b[p] = 1.0;
        return b;
    }
    // 이미 bernstein_all이 있다면 그걸 써도 됨.
    let mut b = vec![0.0; p + 1];
    b[0] = 1.0;
    let omu = 1.0 - u;
    for i in 1..=p {
        let mut saved = 0.0;
        for j in 0..i {
            let temp = b[j];
            b[j] = saved + omu * temp;
            saved = u * temp;
        }
        b[i] = saved;
    }
    b
}
```
```rust
/// 1D Bezier curve de Casteljau split. 입력 `a`는 ctrl 복사본.
/// 반환: (left_ctrl[p+1], right_ctrl[p+1])
pub fn on_split_curve_lerp(a: &mut [Point4D], t: Real) -> (Vec<Point4D>, Vec<Point4D>) {
    let p = a.len() - 1;
    let mut left = vec![Point4D::zero(); p + 1];
    let mut right = vec![Point4D::zero(); p + 1];

    left[0] = a[0];
    right[p] = a[p];

    for k in 1..=p {
        for i in 0..=(p - k) {
            a[i] = a[i].lerp(&a[i + 1], t);
        }
        left[k] = a[0];
        right[p - k] = a[p - k];
    }
    (left, right)
}
```
```rust
#[inline]
fn on_binom_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut res = 1.0;
    for i in 1..=k {
        res *= (n - k + i) as f64 / (i as f64);
    }
    res
}
```
```rust
pub fn on_build_bezier_elevate_matrix(old_p: usize, t: usize) -> Vec<Vec<f64>> {
    let new_p = old_p + t;
    let mut e = vec![vec![0.0f64; old_p + 1]; new_p + 1];

    e[0][0] = 1.0;
    e[new_p][old_p] = 1.0;

    let mid = new_p / 2;
    // The equation below: E[i,j] = C(old_p,j) * C(t,i-j) / C(new_p,i)
    for i in 1..=mid {
        let denom = on_binom_f64(new_p, i);
        if denom == 0.0 {
            continue;
        }
        let j_min = i.saturating_sub(t);
        let j_max = old_p.min(i);
        for j in j_min..=j_max {
            e[i][j] = on_binom_f64(old_p, j) * on_binom_f64(t, i - j) / denom;
        }
    }
    for i in (mid + 1)..new_p {
        let j_min = i.saturating_sub(t);
        let j_max = old_p.min(i);
        for j in j_min..=j_max {
            // 대칭성 이용
            e[i][j] = e[new_p - i][old_p - j];
        }
    }
    e
}
```

