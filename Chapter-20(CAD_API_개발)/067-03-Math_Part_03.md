
## on_solve_tridiagonal_inplace

- 이 함수는 **삼중 대각선(tridiagonal) 선형 시스템을 푸는 Thomas 알고리즘(Tridiagonal Matrix Algorithm, TDMA)** 의 정석적인 구현.
- 수치해석(Numerical Analysis)에서 매우 널리 쓰이는 알고리즘이고, 특히 스플라인(Spline) 계산,  
    PDE 해석, 선형 보간/보정, NURBS 곡선/곡면 계산 등에서 핵심 역할을 한다.

### 📘 on_solve_tridiagonal_inplace(a, b, c, d)
- 삼중 대각선 선형 시스템을 Thomas 알고리즘으로 직접(in-place) 해결
### 📌 문제 정의
- 삼중 대각선(tridiagonal) 선형 시스템:
```math
\left[ \begin{matrix}b_0&c_0&0&\cdots &0\\ a_1&b_1&c_1&\cdots &0\\ 0&a_2&b_2&\cdots &0\\ \vdots &\vdots &\vdots &\ddots &c_{n-2}\\ 0&0&0&a_{n-1}&b_{n-1}\end{matrix}\right] \left[ \begin{matrix}x_0\\ x_1\\ x_2\\ \vdots \\ x_{n-1}\end{matrix}\right] =\left[ \begin{matrix}d_0\\ d_1\\ d_2\\ \vdots \\ d_{n-1}\end{matrix}\right]
```
여기서:
- a[i] = subdiagonal (아래 대각, 단 a[0]은 사용되지 않음)
- b[i] = diagonal (주대각)
- c[i] = superdiagonal (위 대각, 단 c[n-1]은 사용되지 않음)
- d[i] = RHS (해가 저장될 배열)
- Thomas 알고리즘은 이 시스템을 O(n) 시간에 해결한다.

### 1️⃣ Forward Elimination (전진 소거)
- 각 단계에서:
```math
m_i=\frac{a_i}{b_{i-1}}
```
```math
b_i\leftarrow b_i-m_ic_{i-1}
```
```math
d_i\leftarrow d_i-m_id_{i-1}
```
- 즉, 아래 대각 요소를 제거하여 상삼각 행렬로 만든다.
- 코드:
```rust
let m = a[i] / b[i - 1];
b[i] -= m * c[i - 1];
d[i] -= m * d[i - 1];
```

- 여기서 b[i-1]이 너무 작으면(≈0) 분모가 되므로 실패 처리.

### 2️⃣ Back Substitution (후진 대입)
- 마지막 해:
```math
x_{n-1}=\frac{d_{n-1}}{b_{n-1}}
```
- 그 다음:
```math
x_i=\frac{d_i-c_ix_{i+1}}{b_i}
```
- 코드:
```rust
d[n - 1] /= b[n - 1];
for i in (0..n - 1).rev() {
    d[i] = (d[i] - c[i] * d[i + 1]) / b[i];
}
```

- 결과적으로 d 배열이 해 벡터 x로 덮어쓰기된다.

### 📌 수학적 의미 요약
- Thomas 알고리즘은 다음을 수행한다:
- 전진 소거
```math
A\rightarrow U\quad (\mathrm{상삼각\  행렬})
```

- 후진 대입
```math
Ux=d\Rightarrow x\mathrm{\  계산}
```
- 시간 복잡도:
- $O(n)$
- 이는 일반적인 Gaussian elimination의 $O(n^3)$ 보다 훨씬 빠르다.

### 📌 안정성 및 실패 조건
- 다음 조건에서 false를 반환한다:
    - 배열 길이가 일치하지 않음
    - b[i]가 너무 작아(≈0) 분모가 될 수 없음
    - 시스템이 특이(singular)하거나 수치적으로 불안정한 경우


```rust
/// Tridiagonal (Thomas) algorithm. Length n.
/// a: subdiagonal (a[0] is unused), b: diagonal,
/// c: superdiagonal (c[n-1] is unused), d: RHS → overwritten with solution.
/// Returns: success status
pub fn on_solve_tridiagonal_inplace(
    a: &mut [f64],
    b: &mut [f64],
    c: &mut [f64],
    d: &mut [f64],
) -> bool {
    let n = b.len();
    if n == 0 || a.len() != n || c.len() != n || d.len() != n {
        return false;
    }

    // Forward elimination
    for i in 1..n {
        if b[i - 1].abs() < 1e-30 {
            return false;
        }
        let m = a[i] / b[i - 1];
        b[i] -= m * c[i - 1];
        d[i] -= m * d[i - 1];
        // Treat a[i] as zero
    }
    // Back substitution
    if b[n - 1].abs() < 1e-30 {
        return false;
    }
    d[n - 1] /= b[n - 1];
    for i in (0..n - 1).rev() {
        if b[i].abs() < 1e-30 {
            return false;
        }
        d[i] = (d[i] - c[i] * d[i + 1]) / b[i];
    }
    true
}
```
## on_adaptive_simpson

- 이 함수는 **적응형 심프슨 적분(Adaptive Simpson’s Rule)**의 정석적인 구현.
- 수치적분에서 가장 널리 쓰이는 알고리즘 중 하나고, CAD·물리·시뮬레이션·곡선 길이 계산 등에서  
    핵심적으로 사용.
아래는 문서에 그대로 넣을 수 있는 완전한 수학적 설명 + 단계별 해설이다.

### 📘 on_adaptive_simpson(f, a, b, tol, depth)
- Adaptive Simpson’s Rule로 구간 [a, b]의 적분을 원하는 정밀도(tol)까지 계산

### 1️⃣ 기본 Simpson 공식
- 구간 [a,b]에서 심프슨 근사값은:
```math
S(a,b)=\frac{b-a}{6}\left[ f(a)+4f\left( \frac{a+b}{2}\right) +f(b)\right]
``` 
- 코드의 simpson()이 정확히 이 수식을 구현한다.

### 2️⃣ 적응형(Adaptive) 전략
- 구간 [a,b]을 반으로 나누어:
    - 왼쪽 구간 [a,c]
    - 오른쪽 구간 [c,b]
- 에 대해 각각 Simpson 값을 계산:
```math
S_1=S(a,c),\quad S_2=S(c,b)
```
- 전체 구간의 Simpson 값:
```math
S=S(a,b)
```
- 오차 추정값은:
```math
E=|S_1+S_2-S|
```
- Simpson 규칙의 오차 보정 계수는 15이므로:
```math
E<15\cdot \mathrm{tol}
```
- 이면 원하는 정밀도에 도달했다고 판단한다.

### 3️⃣ 오차가 충분히 작으면 보정된 값 반환
```math
\mathrm{result}=S_1+S_2+\frac{S_1+S_2-S}{15}
```
- 즉, Richardson extrapolation을 적용한 보정된 적분값.
- 코드:
```rust
return s1 + s2 + (s1 + s2 - s) / 15.0;
```


### 4️⃣ 오차가 크면 재귀적으로 두 구간을 더 세분화
```rust
on_adaptive_simpson(f, a, c, tol * 0.5, depth - 1)
+ on_adaptive_simpson(f, c, b, tol * 0.5, depth - 1)
```

- tolerance를 절반으로 나누어 두 구간에 배분
- depth는 재귀 깊이 제한 (무한 분할 방지)

### 5️⃣ depth == 0이면 강제 종료
- 더 이상 분할할 수 없으면 현재 근사값을 그대로 반환한다.

### 📌 전체 알고리즘 요약
- 전체 구간의 Simpson 값 $S$ 계산
    - 두 하위 구간의 Simpson 값 $S_1,S_2$ 계산
- 오차 추정
    - 오차가 작으면 보정된 값 반환
    - 오차가 크면 두 구간으로 나누어 재귀
    - depth 제한 도달 시 현재 근사값 반환

### 📌 기하학적/수치적 의미
- 이 알고리즘은 다음 상황에서 매우 유용하다:
    - 곡선 길이 계산
    - 에너지/물리량 적분
    - NURBS 곡선/곡면의 파라미터 적분
    - PDF/CDF 계산
    - 부드럽지 않은 함수의 적분
    - 오차 제어가 필요한 모든 수치적분
- Adaptive Simpson은 부드러운 구간은 크게, 변화가 심한 구간은 촘촘히 분할하기 때문에  
    효율성과 정확도를 동시에 만족한다.


- Simpson 적분의 오차는 4차 항을 가진다.
- 즉,
```math
E(h)=Ch^4+O(h^6)
```
- 여기서
    - h = 구간 폭
    - C = 어떤 상수
- Simpson을 두 번 계산하면:
    - S_1=S(h)
    - S_2=S(h/2)
- 이 둘의 오차는 다음과 같다:
```math
S_1=I+Ch^4
```
```math
S_2=I+C\left( \frac{h}{2}\right) ^4=I+\frac{Ch^4}{16}
```
- 여기서 I 는 실제 적분값.
- 이제 두 식을 조합해서 오차항을 제거하면:
```math
I=\frac{16S_2-S_1}{15}
```
- 여기서 15가 정확히 등장한다.

### 🎯 식과 연결해보면
- 식:
```math
\mathrm{result}=S_1+S_2+\frac{S_1+S_2-S}{15}
```
- 이건 구조만 다를 뿐, 본질은 같은 Richardson 보정식이야.
- 여기서
    - S 는 coarse/fine 조합으로 얻은 예측값
    - S_1+S_2-S 는 잔여 오차(residual error)
    - 그 오차를 정확히 제거하기 위해 1/15 를 곱하는 것
- 즉,
- Simpson 적분의 오차가 4차이기 때문에
- Richardson 보정 계수는
```math
\frac{1}{2^4-1}=\frac{1}{15}
```
- 로 수학적으로 반드시 15가 된다.


### 🎯 왜 2⁴ - 1 이냐?
- Simpson은 4차 정확도이므로, 스텝을 절반으로 줄이면 오차가 16배 감소한다.
- 즉:
    - coarse step: 오차 = Ch^4
    - half step: 오차 = Ch^4/16
- 이 둘을 조합해 오차를 제거하려면:
```math
\frac{1}{16-1}=\frac{1}{15}
```
- 이게 바로 15의 정체.

### 🎯 Adaptive Simpson에서 이게 어떻게 쓰이냐?
- Adaptive Simpson은 다음을 계산:
    - 큰 구간에서 Simpson → S
    - 두 개의 작은 구간에서 Simpson → S_1+S_2
- 두 값의 차이로 오차 추정:
```math
\mathrm{error}=\frac{S_1+S_2-S}{15}
```
- 오차가 허용 범위보다 작으면:
```math
\mathrm{result}=S_1+S_2+\mathrm{error}
```
- 즉,
    - 오차를 1/15 만큼 보정해서 더 정확한 적분값을 얻는 것.

### 🎯 요약
- Simpson 적분의 오차는 4차 → 스텝 절반이면 오차 1/16
- Richardson 보정 계수 = 1/(16-1)=1/15
- 그래서 Adaptive Simpson에서는
```math
\frac{S_1+S_2-S}{15}
```
- 이 정확히 등장한다.
- 즉, 15는 수학적으로 필연적으로 등장하는 값.

```rust
pub fn on_adaptive_simpson<F>(f: &F, a: f64, b: f64, tol: f64, depth: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    fn simpson<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64) -> f64 {
        let c = 0.5 * (a + b);
        (b - a) * (f(a) + 4.0 * f(c) + f(b)) / 6.0
    }

    let c = 0.5 * (a + b);
    let s = simpson(f, a, b);
    let s1 = simpson(f, a, c);
    let s2 = simpson(f, c, b);
    if depth == 0 || (s1 + s2 - s).abs() < 15.0 * tol {
        return s1 + s2 + (s1 + s2 - s) / 15.0;
    }
    on_adaptive_simpson(f, a, c, tol * 0.5, depth - 1)
        + on_adaptive_simpson(f, c, b, tol * 0.5, depth - 1)
}
```

## 수치적 동등성

- 이 묶음은 **수치적 동등성(numerical equality)** 을 판단하는 함수들의 패키지.
- CAD·Geometry·Simulation 엔진에서는 부동소수점 오차 때문에 “같다”를 직접 비교할 수 없기 때문에,  
    이런 함수들이 필수적으로 존재.

### 📘 수치적 동등성 검사 함수 모음
- (절대 오차, 상대 오차, 벡터/점 비교, zero 판정)

### 1️⃣ on_are_equal(a, b, tol)
- 절대 오차 기반 비교
```math
|a-b|<\mathrm{tol}
```
즉, 두 실수의 차이가 tol보다 작으면 “같다”고 본다.
- 작은 값 비교에 적합
- 좌표, 길이, 파라미터 비교 등에서 사용

### 2️⃣ on_are_equal_rel_tol(a, b)
- 상대 오차 기반 비교 (relative tolerance)
```rust
let scale = max(|a|, |b|, 1)
|a - b| ≤ ON_TOL9 * scale
```

- 수식:
```math
|a-b|\leq \mathrm{tol}\cdot \max (|a|,|b|,1)
```
- 특징:
    - 값의 크기에 비례한 오차 허용
    - 큰 값 비교에 안정적
    - 0 근처에서도 최소 스케일 1을 사용해 안정성 확보

### 3️⃣ on_are_vector_equal(a, b, tol)
- f64 배열(벡터)의 원소별 비교
```math
a_i\approx b_i\quad \forall i
```
- 즉:
    - 길이가 같고
    - 모든 원소가 on_are_equal을 만족하면 true

### 4️⃣ on_are_vec_equal(u, v, eps)
- 3D 벡터(Vector3D)의 성분별 비교
```math
|u_x-v_x|<\epsilon ,\; |u_y-v_y|<\epsilon ,\; |u_z-v_z|<\epsilon 
```
- 즉, 성분별 절대 오차 비교.

### 5️⃣ on_are_vec2_equal(u, v, eps)
- 2D 벡터(Vector2D)의 성분별 비교
```math
|u_x-v_x|<\epsilon ,\; |u_y-v_y|<\epsilon 
```

### 6️⃣ on_are_pt_equal(p, q, eps)
- 3D 점(Point3D)의 좌표 비교
```math
|p_x-q_x|<\epsilon ,\; |p_y-q_y|<\epsilon ,\; |p_z-q_z|<\epsilon 
```
- 즉, 두 점이 **같은 위치** 인지 판정.

### 7️⃣ on_are_pt2_equal(p, q, eps)
- 2D 점(Point2D)의 좌표 비교
```math
|p_x-q_x|<\epsilon ,\; |p_y-q_y|<\epsilon 
```
### 8️⃣ on_is_zero(a, tol)
- 0인지 판정하는 절대 오차 비교
```math
|a|<\mathrm{tol}
```
- 즉, 부동소수점 오차를 고려한 **0 판정**.

- 📌 전체 요약
이 함수들은 모두 다음 개념을 기반으로 한다:
    - 절대 오차 비교
    - 상대 오차 비교
    - 벡터/점의 성분별 비교
    - zero 판정
- CAD·Geometry 엔진에서 좌표 비교, 교차 판정, 평면성 검사, 정규화 안정성 등  
    거의 모든 곳에서 사용되는 핵심 유틸리티다.

```rust
#[inline]
pub fn on_are_equal(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}
```
```rust
#[inline]
pub fn on_are_equal_rel_tol(a: f64, b: f64) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0);
    (a - b).abs() <= ON_TOL9 * scale
}
```
```rust
#[inline]
pub fn on_are_vector_equal(a: &[f64], b: &[f64], tol: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| on_are_equal(*x, *y, tol))
}
```
```rust
#[inline]
pub fn on_are_vec_equal(u: Vector3D, v: Vector3D, eps: f64) -> bool {
    on_are_equal(u.x, v.x, eps) && on_are_equal(u.y, v.y, eps) && on_are_equal(u.z, v.z, eps)
}
```
```rust
#[inline]
pub fn on_are_vec2_equal(u: Vector2D, v: Vector2D, eps: f64) -> bool {
    on_are_equal(u.x, v.x, eps) && on_are_equal(u.y, v.y, eps)
}
```
```rust
#[inline]
pub fn on_are_pt_equal(p: Point3D, q: Point3D, eps: f64) -> bool {
    on_are_equal(p.x, q.x, eps) && on_are_equal(p.y, q.y, eps) && on_are_equal(p.z, q.z, eps)
}
```
```rust
#[inline]
pub fn on_are_pt2_equal(p: Point2D, q: Point2D, eps: f64) -> bool {
    on_are_equal(p.x, q.x, eps) && on_are_equal(p.y, q.y, eps)
}
```
```rust
#[inline]
pub fn on_is_zero(a: f64, tol: f64) -> bool {
    a.abs() < tol
}
```
## on_compare 

- 이 묶음은 부동소수점 비교, 범위 판정, 무한대 검사, 클램프, 상대 오차 계산 등  
    CAD·Geometry 엔진에서 거의 매 프레임마다 쓰이는 핵심적인 수치 유틸리티들.

### 📘 수치 비교 / 범위 판정 / 오차 계산 유틸리티 모음

### 1️⃣ on_compare(a, b, tol)
- tol 기반의 3-way 비교 함수
```math
\mathrm{if\  }a<b-\mathrm{tol}\Rightarrow -1
```
```math
\mathrm{if\  }a>b+\mathrm{tol}\Rightarrow 1
```
```math
\mathrm{else\  }0
```
- 즉,
    - -1 → a < b (확실히 작음)
    - 1 → a > b (확실히 큼)
    - 0 → a ≈ b (tol 이내)
- 부동소수점 비교에서 매우 자주 쓰이는 패턴.

### 2️⃣ on_are_between(a, b, c)
- c가 a와 b 사이에 있는지 판정
```math
(a<c<b)\; \; \mathrm{or}\; \; (b<c<a)
```
- 즉, 순서에 상관없이 열린 구간 (a,b) 안에 c가 있는지 검사.

### 3️⃣ on_greater_than(a, b, tol)
- a > b 를 tol 고려하여 판정
```math
a>b\; \wedge \; |a-b|>\mathrm{tol}
```
- 즉,
    - a가 b보다 크고
    - 단순 오차가 아니라 확실히 큰 경우
- NaN이 포함되면 false.

### 4️⃣ on_greater_equal_than(a, b, tol)
- a ≥ b 를 tol 고려하여 판정
```math
a>b\; \; \mathrm{or}\; \; |a-b|\leq \mathrm{tol}
```
- 즉,
    - a가 b보다 크거나
    - 거의 같은 경우

### 5️⃣ on_less_than(a, b, tol)
- a < b 를 tol 고려하여 판정
```math
a<b\; \wedge \; |a-b|>\mathrm{tol}
```
- 즉, 확실히 작은 경우만 true.

### 6️⃣ on_less_equal_than(a, b, tol)
- a ≤ b 를 tol 고려하여 판정
```math
a<b\; \; \mathrm{or}\; \; |a-b|\leq \mathrm{tol}
```
- 즉,
    - a가 b보다 작거나
    - 거의 같은 경우

### 7️⃣ on_is_infinite(v)
- v가 ±∞인지 판정
```rust
!(-max <= v && v <= max)
```

- 즉:
```math
v<-\mathrm{MAX}\; \; \mathrm{or}\; \; v>\mathrm{MAX}
```
- 이는 사실상:
```math
v=\pm \infty 
```
- 를 의미한다.

### 8️⃣ on_clamp(x, lo, hi)
- 값을 [lo, hi] 범위로 제한
```math
\mathrm{clamp}(x)=\left\{ \, \begin{array}{ll}\textstyle lo,&\textstyle x<lo\\ \textstyle hi,&\textstyle x>hi\\ \textstyle x,&\textstyle \mathrm{otherwise}\end{array}\right. 
```
- 모든 수치 알고리즘에서 필수적인 안정성 도구.

### 9️⃣ on_rel_err(a, b)
- 상대 오차(relative error) 계산
```math
\mathrm{rel\_ err}(a,b)=\frac{|a-b|}{\max (|a|,|b|,10^{-16})}
```
- 특징:
    - 값의 크기에 비례한 오차 측정
    - 0 근처에서도 안정적 (denom 최소값 1e-16)

## 📌 전체 요약
- 이 함수들은 모두 다음 목적을 가진다:
    - 절대/상대 오차 기반 비교
    - 크기 비교 (>, <, ≥, ≤)
    - 범위 포함 검사
    - 무한대 검사
    - 값 클램프
    - 상대 오차 계산
- CAD·Geometry 엔진에서 좌표 비교, 파라미터 비교, 경계 판정, 교차 검사, 수치 안정성  
    확보 등 거의 모든 곳에서 사용되는 핵심 도구들이다.

```rust
#[inline]
pub fn on_compare(a: f64, b: f64, tol: f64) -> i32 {
    if a <= b - tol {
        -1
    } else if a >= b + tol {
        1
    } else {
        0
    }
}
```
```rust
pub fn on_are_between(a: f64, b: f64, c: f64) -> bool {
    ((a < c) && (c < b)) || ((b < c) && (c < a))
}
```
```rust
pub fn on_greater_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a > b) && !on_are_equal(a, b, tol)
}
```
```rust
pub fn on_greater_eqaul_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a > b) || on_are_equal(a, b, tol)
}
```
```rust
pub fn on_less_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a < b) && !on_are_equal(a, b, tol)
}
```
```rust
pub fn on_less_equal_than(a: f64, b: f64, tol: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a < b) || on_are_equal(a, b, tol)
}
```
```rust
pub fn on_is_infinite(v: f64) -> bool {
    let max = f64::MAX;
    !(-max <= v && v <= max)
}
```
```rust
#[inline]
pub fn on_clamp<T: PartialOrd>(x: T, lo: T, hi: T) -> T {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}
```
```rust
pub fn on_rel_err(a: f64, b: f64) -> f64 {
    let denom = a.abs().max(b.abs()).max(1e-16);
    (a - b).abs() / denom
}
```
```rust
pub fn on_get_sampling_2d(datas: &[Point2D]) -> Vec<Point2D> {
    let count = datas.len() as i32;
    if count == 0 {
        return vec![];
    }

    let mut length = 4 + (count as f64).cbrt() as i32;
    if length > count {
        length = count;
    }

    let mut r = count / length;
    if r == 0 {
        r = 1;
    }

    let mut out = Vec::with_capacity(length as usize);
    for i in 0..length {
        out.push(datas[(i * r) as usize]);
    }
    out
}
```
## on_get_sampling_3d

- 이 함수는 3D 점 배열에서 균일하게 샘플링된(subsampled) 점들을  
    추출하는 간단한 휴리스틱 알고리즘.
- 특히 데이터가 많을 때 전체를 다 쓰지 않고 **대표점** 만 뽑아 쓰는 용도로 적합.

### 📘 on_get_sampling_3d(datas)
- 3D 점 배열에서 균일 간격으로 샘플링된 점들을 반환
### 📌 목적
- 입력된 3D 점 배열 datas에서  전체 개수에 비례하여 
    적당한 수의 점을 균일하게 선택해 반환한다.
- 즉,
    - 데이터가 많을수록 더 많이 뽑고
    - 데이터가 적으면 그대로 반환하며
    - 전체를 고르게 대표하는 점들을 선택한다.

### 1️⃣ 샘플 개수 결정
```rust
let mut length = 4 + (count as f64).cbrt() as i32;
if length > count {
    length = count;
}
```

- 수식:
```math
\mathrm{length}=4+\lfloor \sqrt[3]{N}\rfloor 
```
- 여기서 $N=\mathrm{datas.len()}$.
- 특징:
    - 데이터가 많아질수록 천천히 증가
    - 최소 4개
    - 전체 개수보다 많아지지 않음

### 2️⃣ 샘플링 간격 r 계산
```rust
let mut r = count / length;
if r == 0 { r = 1; }
```

- 즉:
```math
r=\left\lfloor \frac{N}{\mathrm{length}}\right\rfloor 
```
- r은 샘플링 간격
- r이 0이면 1로 보정 (데이터가 매우 적을 때)

### 3️⃣ 균일 샘플링 수행
```rust
for i in 0..length {
    out.push(datas[(i * r) as usize]);
}
```

- 즉:
```math
\mathrm{output}[i]=\mathrm{datas}[i\cdot r]
```
이는 **등간격 샘플링(uniform subsampling)** 이다.

### 📌 기하학적 의미
- 이 함수는 다음 상황에서 유용하다:
    - 3D 점군(point cloud)에서 대표점만 추출
    - 곡선/곡면 샘플링에서 빠른 프리뷰 생성
    - 대규모 데이터의 다운샘플링
    - 시각화/디버깅용 간단한 샘플링
    - 복잡한 기하 알고리즘의 초기 guess 생성
- 즉, 데이터의 전체 분포를 유지하면서 빠르게 대표점을 뽑는 휴리스틱이다.

```rust
pub fn on_get_sampling_3d(datas: &[Point3D]) -> Vec<Point3D> {
    let count = datas.len() as i32;
    if count == 0 {
        return vec![];
    }

    let mut length = 4 + (count as f64).cbrt() as i32;
    if length > count {
        length = count;
    }

    let mut r = count / length;
    if r == 0 {
        r = 1;
    }

    let mut out = Vec::with_capacity(length as usize);
    for i in 0..length {
        out.push(datas[(i * r) as usize]);
    }
    out
}
```

## on_is_finite
- 이 묶음은 **수치 안정성(safety)과 값 제한(clamping)** 을  
    위한 아주 기본적이면서도 필수적인 유틸리티들.
- CAD·Geometry·Simulation 엔진에서 거의 모든 계산의  
    전처리·후처리에 등장하는 함수들

### 📘 수치 안정성 / 값 제한 유틸리티 모음

### 1️⃣ on_is_finite(v)
- 유한한 실수인지 검사
```rust
v.is_finite()
```

- 수학적으로:
```math
v\in \mathbb{R}\quad \mathrm{(NaN,\  +\infty ,\  -\infty \  제외)}
```
- 즉, 다음을 모두 제외한다:
    - NaN
    - +∞
    - -∞
- 기하 알고리즘에서 계산 폭주나 분모 0 문제를 조기에 감지하는 데 매우 중요하다.

### 2️⃣ on_clamp01(t)
- 값을 [0, 1] 구간으로 제한
```math
\mathrm{clamp_{\mathnormal{[0,1]}}}(t)=\left\{ \, \begin{array}{ll}\textstyle 0,&\textstyle t<0\\ \textstyle 1,&\textstyle t>1\\ \textstyle t,&\textstyle \mathrm{otherwise}\end{array}\right.
``` 
- 주로 다음에 사용된다:
    - 보간 파라미터 t (lerp, spline, easing)
    - UV 좌표 제한
    - 확률 값 제한
    - 정규화된 범위(0~1)에서 벗어난 값 보정

### 3️⃣ on_clamp11(x)
- 값을 [-1, 1] 구간으로 제한
```math
\mathrm{clamp_{\mathnormal{[-1,1]}}}(x)=\left\{ \, \begin{array}{ll}\textstyle -1,&\textstyle x<-1\\ \textstyle 1,&\textstyle x>1\\ \textstyle x,&\textstyle \mathrm{otherwise}\end{array}\right.
``` 
- 특히 다음에서 매우 중요하다:
    - acos, asin 입력값 보정  
        (부동소수점 오차로 인해 ±1.0000000002 같은 값이 들어오는 문제 방지)
- 방향 코사인/사인 값 안정화
- 노멀라이즈된 값의 범위 보정
- 예:
```math
\cos \theta =\mathrm{clamp_{\mathnormal{[-1,1]}}}(\cos \theta )
```

### 📌 전체 요약
- 이 함수들은 모두 수치적 안정성 확보를 위한 핵심 도구다.
    - on_is_finite → 값이 정상적인 실수인지 검사
    - on_clamp01 → [0,1] 범위로 제한
    - on_clamp11 → [-1,1] 범위로 제한
- 즉,
- CAD·Geometry 엔진에서 acos/asin 안정화, 파라미터 보정, 보간 안정성,  
    NaN/Inf 방지 등 수많은 곳에서 필수적으로 사용된다.

```rust
#[inline]
pub fn on_is_finite(v: f64) -> bool {
    v.is_finite()
}
```
```rust
#[inline]
pub fn on_clamp01(t: f64) -> f64 {
    if t < 0.0 {
        0.0
    } else if t > 1.0 {
        1.0
    } else {
        t
    }
}
```
```rust
#[inline]
pub fn on_clamp11(x: f64) -> f64 {
    if x < -1.0 {
        -1.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}
```
## on_poly_fit / on_poly_val

- 이 묶음은 다항식 회귀(polynomial fitting), 다항식 평가(Horner),  
    Cholesky 분해 기반 SPD(대칭 양의 정부호) 선형 시스템 해법까지 포함한,  
    수치해석 패키지의 핵심 기능들.
### 📘 다항식 회귀 + 다항식 평가 + Cholesky SPD Solver 모음

### 1️⃣ on_poly_fit(t, v, order)
- 최소제곱법(Least Squares) 기반 다항식 회귀
- ✔ 문제 정의
- 주어진 데이터:
```math
(t_i,v_i),\quad i=0,\dots ,m-1
```
- 에 대해,
- 다음 형태의 다항식:
```math
p(t)=c_0+c_1t+c_2t^2+\cdots +c_{\mathrm{order}}t^{\mathrm{order}}
```
- 을 최소제곱 오차로 fitting한다.

- ✔ 설계 행렬 T 구성
- 코드:
```rust
t_vec[i*n + j] = t[i]^j
```

- 수식:
```math
T=\left[ \begin{matrix}1&t_0&t_0^2&\cdots &t_0^n\\ 1&t_1&t_1^2&\cdots &t_1^n\\ \vdots &\vdots &\vdots &&\vdots \\ 1&t_{m-1}&t_{m-1}^2&\cdots &t_{m-1}^n\end{matrix}\right] 
```
- ✔ Normal Equation 구성
```math
(T^TT)c=T^Tv
```
- 코드에서:
- $ata = TᵀT$
- $atv = Tᵀv$

- ✔ SPD 시스템을 Cholesky로 해결
```math
c=(T^TT)^{-1}T^Tv
```
- 코드:
```rust
assert!(on_cholesky_solve_spd(&mut ata, &mut c, n));
```

- ✔ 결과
- 반환값 c는 다항식 계수:
```math
[c_0,c_1,\dots ,c_n]
```

### 2️⃣ on_poly_val(coeff, xs)
- Horner 방식으로 다항식 평가
- Horner 알고리즘:
```math
p(x)=a_0+a_1x+a_2x^2+\cdots 
```
- 을 다음처럼 계산:
```math
p(x)=a_n+x(a_{n-1}+x(a_{n-2}+\cdots ))
```
- 코드:
```rust
for &a in coeff.iter().rev() {
    y = a + x * y;
}
```

- 장점:
    - 곱셈/덧셈 최소화
    - 수치적으로 안정적
    - 매우 빠름

### 3️⃣ on_cholesky_solve_spd(g, b, n)
- 대칭 양의 정부호(SPD) 행렬을 Cholesky 분해로 직접(in-place) 해결
- ✔ Cholesky 분해
- 행렬 A를:
```math
A=LL^T
```
- 형태로 분해.
- 코드에서:
    - g가 입력 SPD 행렬
    - 분해 후 g는 L로 덮어쓰기됨

- ✔ Forward substitution
```math
Ly=b
```

- ✔ Back substitution
```math
L^Tx=y
```
- 결과는 b에 덮어쓰기된다.

- ✔ SPD 조건 실패 시 false
```rust
if diag <= ON_TOL14 { return false; }
```

- 즉,
    - 행렬이 SPD가 아니거나
    - 수치적으로 불안정하면 실패

### 4️⃣ on_nalgebra_cholesky_solve_spd(...)
- nalgebra 라이브러리를 이용한 Cholesky 기반 SPD 해법
    - 입력 슬라이스를 DMatrix, DVector로 변환
    - Cholesky::new(a)로 분해
    - chol.solve(b)로 해 계산
    - 결과를 b에 덮어쓰기
- 기본 Cholesky 구현보다 안정적이며, nalgebra의 고성능 최적화를 활용할 수 있다.

### 📌 전체 요약
- 이 네 함수는 다음 수치해석 파이프라인을 완성한다:

- ✔ 1) 다항식 회귀 (Least Squares)
```math
(T^TT)c=T^Tv
```
- ✔ 2) SPD 시스템을 Cholesky로 해결
```math
c=(T^TT)^{-1}T^Tv
```
- ✔ 3) 다항식 평가 (Horner)
```math
p(x)=a_n+x(a_{n-1}+\cdots )
```
- ✔ 4) nalgebra 기반 고성능 Cholesky Solver

### 📌 기하학적/수치적 활용
- 곡선 fitting
- NURBS 파라미터 fitting
- smoothing / filtering
- 물리 시뮬레이션에서 회귀 기반 보정
- CAD에서 곡선 approximation
- 데이터 분석에서 polynomial regression

```rust
pub fn on_poly_fit(t: &[f64], v: &[f64], order: usize) -> Vec<f64> {
    assert_eq!(t.len(), v.len());
    assert!(t.len() >= order + 1);
    let m = t.len();
    let n = order + 1;

    let mut t_vec = vec![0.0; m * n];
    for i in 0..m {
        let mut xpow = 1.0;
        for j in 0..n {
            t_vec[i * n + j] = xpow;
            xpow *= t[i];
        }
    }

    // Normal equations: (T^T T) c = T^T v
    let mut ata = vec![0.0; n * n];
    let mut atv = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..m {
                s += t_vec[k * n + i] * t_vec[k * n + j];
            }
            ata[i * n + j] = s;
        }
        let mut s = 0.0;
        for k in 0..m {
            s += t_vec[k * n + i] * v[k];
        }
        atv[i] = s;
    }

    let mut c = atv.clone();
    assert!(on_cholesky_solve_spd(&mut ata, &mut c, n));
    c
}
```
```rust
pub fn on_poly_val(coeff: &[f64], xs: &[f64]) -> Vec<f64> {
    let mut ys = vec![0.0; xs.len()];
    for (i, &x) in xs.iter().enumerate() {
        // Horner
        let mut y = 0.0;
        for &a in coeff.iter().rev() {
            y = a + x * y;
        }
        ys[i] = y;
    }
    ys
}
```
```rust
pub fn on_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    for k in 0..n {
        let mut sum = 0.0;
        for p in 0..k {
            let l = g[k * n + p];
            sum += l * l;
        }
        let diag = g[k * n + k] - sum;
        if diag <= ON_TOL14 {
            return false;
        }
        g[k * n + k] = diag.sqrt();
        for i in (k + 1)..n {
            let mut s = 0.0;
            for p in 0..k {
                s += g[i * n + p] * g[k * n + p];
            }
            g[i * n + k] = (g[i * n + k] - s) / g[k * n + k];
        }
        for j in (k + 1)..n {
            g[k * n + j] = 0.0;
        }
    }
    for i in 0..n {
        let mut s = 0.0;
        for j in 0..i {
            s += g[i * n + j] * b[j];
        }
        b[i] = (b[i] - s) / g[i * n + i];
    }
    for i in (0..n).rev() {
        let mut s = 0.0;
        for j in (i + 1)..n {
            s += g[j * n + i] * b[j];
        }
        b[i] = (b[i] - s) / g[i * n + i];
    }
    true
}
```
```rust
pub fn on_nalgebra_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    // 1. 입력 슬라이스를 nalgebra 행렬/벡터로 변환
    let a = DMatrix::from_row_slice(n, n, g);
    let b_vec = DVector::from_column_slice(b);

    // 2. Cholesky 분해 시도
    if let Some(chol) = Cholesky::new(a) {
        // 3. 시스템 Ax = b 풀기
        let x = chol.solve(&b_vec);

        // 4. 결과를 b 슬라이스에 덮어쓰기
        b.copy_from_slice(x.as_slice());
        true
    } else {
        false
    }
}
```

## on_circle_from_3_points

```rust
pub fn on_circle_from_3_points(a: &Point2D, b: &Point2D, c: &Point2D) -> Option<(Point2D, f64)> {
    let a1 = b.x - a.x;
    let b1 = b.y - a.y;
    let a2 = c.x - a.x;
    let b2 = c.y - a.y;
    let d1 = (b.x * b.x - a.x * a.x + b.y * b.y - a.y * a.y) * 0.5;
    let d2 = (c.x * c.x - a.x * a.x + c.y * c.y - a.y * a.y) * 0.5;
    let det = a1 * b2 - a2 * b1;
    let scale = a1.abs().max(b1.abs()).max(a2.abs()).max(b2.abs()).max(1.0);
    let eps = ON_TOL12 * scale;

    if det.abs() <= eps {
        return None;
    }
    let ox = (d1 * b2 - d2 * b1) / det;
    let oy = (-d1 * a2 + d2 * a1) / det;
    let o = Point2D::new(ox, oy);
    let r = o.distance(&a);
    Some((o, r))
}
```

## on_get_net_closest_point_leg_index

- **폴리곤/서피스 컨트롤넷 위에서, 점에 가장 가까운 leg(선분) 을 찾는 루틴**.

### 1. on_get_net_closest_point_leg_index 의 역할
```rust
pub fn on_get_net_closest_point_leg_index(
    net: &[Vec<Point3D>], // net[u][v] : (n+1) x (m+1)
    p: Point3D,
) -> Option<(Point3D, usize, usize, SurfaceDir, Real)>
```

- 의미:
    - net[u][v]는 격자형 컨트롤넷 (NURBS surface의 control net 같은 것)
    - 이 net에서 만들어지는 모든 **leg(선분)**:
    - U 방향 leg: <net[k][l], net[k+1][l]>
    - V 방향 leg: <net[k][l], net[k][l+1]>
    - 점 p를 이 모든 leg에 대해 직교 투영해 보고,
    - 투영점이 선분 구간 안에 있을 때만 후보로 인정 (flg == true)
    - 그중 가장 가까운 leg를 찾는다.
- 반환값:
    - q: p를 해당 leg에 투영한 점
    - i, j: leg의 시작점 인덱스 (neti][j])
    - dir: U 방향 leg인지, V 방향 leg인지 (SurfaceDir::UDir / VDir)
    - t: 그 leg 위에서의 파라미터 (domain [0,1] 기준)
- 핵심 요약:
- 컨트롤넷의 모든 U/V 선분 중, 점 p에서 가장 가까운 ‘in-segment projection’을 가지는 leg를 찾고,  
    그 투영점과 leg 인덱스, 방향, 파라미터를 돌려준다.**


### 2. on_project_point_to_line_by_intersect_plane 의 역할
```rust
pub fn on_project_point_to_line_by_intersect_plane(
    line: &Line,
    p: Point3D,
) -> Option<(Point3D, Real, bool)>
```

- 의도:
    - line의 방향 벡터 dir을 구한다.
    - 점 p를 지나고, 법선이 dir인 평면을 만든다.
    - 그 평면과 line의 교점을 구한다 → 이게 “p를 line에 직교 투영한 점”이 된다.
    - dot = line의 start에서부터의 파라미터 (여기서는 domain [0,1]에 대응)
    - flg:
        - dot이 [0,1] 안에 있으면 → 투영점이 선분 구간 안에 있음 → true
        - 아니면 → 선분 밖 → false
- 반환:
    - q: line 위의 투영점
    - t: line.domain에 맞게 변환된 파라미터 (여기선 domain=[0,1]이라 사실상 dot과 동일)
    - flg: “이 투영이 선분 구간 안에 있는가?”
- 즉:
    - “점 p를 line에 직교 투영하고, 그게 선분 안에 있는지까지 알려주는 함수”


### 3. on_get_net_closest_point_leg_index 내부 흐름
- 입력 검증
    - net 비었거나, 직사각형 아니면 → None
    - nu < 2 && nv < 2 → leg 자체가 없음 → None
    - 초기 best 값 설정
    - best_d2 = ∞
    - found = false
    - U 방향 leg 스캔
```rust
for l in 0..nv {
    for k in 0..(nu - 1) {
        let a = net[k][l];
        let b = net[k+1][l];
        // seg: [a,b], domain [0,1]
        let (q, t, flg) = on_project_point_to_line_by_intersect_plane(&seg, p)?;
        if !flg { continue; } // 선분 밖이면 버림
        let d2 = (p - q).length_square();
        if d2 < best_d2 { ... 갱신 ... }
    }
}
```
- V 방향 leg 스캔
```rust
for k in 0..nu {
    for l in 0..(nv - 1) {
        let a = net[k][l];
        let b = net[k][l+1];
        // seg: [a,b], domain [0,1]
        let (q, t, flg) = on_project_point_to_line_by_intersect_plane(&seg, p)?;
        if !flg { continue; }
        let d2 = (p - q).length_square();
        if d2 < best_d2 { ... 갱신 ... }
    }
}
```
- found 여부에 따라 Some / None 반환

### 4. 각 테스트가 “왜” 그런 결과를 내는지
#### 4-1) netleg_picks_right_v_leg_in_2x2_net
- net:
```
(0,1)----(1,1)
  |        |
(0,0)----(1,0)
```
```
u: 0..1, v: 0..1
net[0][0] = (0,0)
net[0][1] = (0,1)
net[1][0] = (1,0)
net[1][1] = (1,1)
```

- U-leg:
    - <(0,0),(1,0)>  (i=0,j=0, UDir)
    - <(0,1),(1,1)>  (i=0,j=1, UDir)
- V-leg:
    - <(0,0),(0,1)>  (i=0,j=0, VDir)
    - <(1,0),(1,1)>  (i=1,j=0, VDir)
점:
- pt = (1.3, 0.25, 0.0)


- 이 점은:
    - x=1.3 → x=1 선분(오른쪽 V-leg)에서 약간 오른쪽
    - y=0.25 → y=0~1 사이
    - 각 leg에 대한 직교 투영을 생각해보면:
        - 오른쪽 V-leg <(1,0),(1,1)>:
        - x=1 고정, y 방향으로 0~1
            - pt를 이 선에 투영하면 q = (1, 0.25, 0)
            - 이건 선분 구간 안 (t=0.25)
            - 거리 = 1.3 - 1 = 0.3
        - 아래 U-leg <(0,0),(1,0)>:
            - y=0 고정, x 방향 0~1
            - pt를 투영하면 x=1.3 → t>1 → 선분 밖 → flg=false → 후보 제외
            - 위 U-leg <(0,1),(1,1)>:
        - y=1 고정, x 방향 0~1
            - y=0.25 → 수직 거리 0.75, x 투영도 t>1 → 선분 밖 or 멀다
            - 왼쪽 V-leg <(0,0),(0,1)>:
            - x=0 고정 → pt와 거리 1.3 이상
- 결국:
- 유효한 in-segment projection 중 가장 가까운 건 오른쪽 V-leg
- 그래서:
```
dir == SurfaceDir::VDir
i == 1
j == 0
q == (1.0, 0.25, 0.0)
t == 0.25
```

- 테스트가 이걸 검증하는 거고, 의도와 정확히 일치.

#### 4-2) netleg_picks_u_leg_when_point_is_near_horizontal_leg
- 같은 2x2 net.
- 점:
```rust
pt = (0.6, -0.4, 0.0)
```
- y=-0.4 → 아래쪽 U-leg <(0,0),(1,0)> 근처
- x=0.6 → 0~1 사이
- 각 leg:
    - 아래 U-leg <(0,0),(1,0)>:
        - y=0, x 0~1
        - pt를 투영하면 q = (0.6, 0.0, 0.0)
        - t = 0.6 (선분 안)
        - 거리 = 0.4
    - 위 U-leg <(0,1),(1,1)>:
        - y=1 → pt와 거리 1.4 이상
        - V-leg 들:
        - x=0, x=1 → pt와 거리 각각 0.6, 0.4 이상
    - 하지만 수직 투영이 선분 안에 들어오더라도, 거리 비교에서 아래 U-leg가 더 유리
- 결과:
```
dir == SurfaceDir::UDir
i == 0
j == 0
q == (0.6, 0.0, 0.0)
t == 0.6
```

- **아래 가로 leg가 가장 가깝다** 는 의도 그대로.

#### 4-3) netleg_returns_none_if_all_projections_outside_segments
- net:
```
(0,0)----(1,0)----(2,0)
nu=3, nv=1
U-leg: [0,0]-[1,0], [1,0]-[2,0]
V-leg: 없음
```

- 점:
```rust
pt = (-10.0, 5.0, 0.0)
```

- 각 U-leg에 대해:
    - 선분 [0,1], [1,2] 위에 직교 투영하면:
    - 방향은 x축, 점은 왼쪽 멀리 (-10,5)
    - 투영 파라미터 t는 둘 다 음수 (선분 왼쪽 바깥)
    - 즉, flg == false → 전부 스킵
- 결과:
    - 어떤 leg도 in-segment projection을 주지 않음
    - found == false → None 반환
- 테스트가 이걸 확인.

#### 4-4) netleg_dense_v_leg_case
- 여기서부터는 더 큰 격자.
```rust
let nu = 7;
let nv = 6;
let dx = 2.0;
let dy = 1.5;
net[u][v] = (u*dx, v*dy, 0)
```

- 즉:
    - u: 0..6 → x = 0,2,4,6,8,10,12
    - v: 0..5 → y = 0,1.5,3,4.5,6,7.5
- 점:
```rust
pt = (6.7, 3.6, 0.0)
```

- 테스트 기대:
```rust
assert_eq!(dir, SurfaceDir::UDir);
assert_eq!(i, 3);
assert_eq!(j, 2);
assert!(approx_pt(q, p(6.7, 3.0, 0.0), 1e-12));
assert!(approx(t, 0.35, 1e-12));
```

- 여기서 약간 주의할 점:
    - dir == UDir 이므로, 선택된 leg는 U 방향:
    - <net[i][j], net[i+1][j]>
    - 즉, y 고정, x가 변하는 수평 선분
    - i=3, j=2 이면:
    - net[3][2] = (3*2.0, 2*1.5) = (6.0, 3.0)
    - net[4][2] = (8.0, 3.0)
        - 선택된 leg: <(6,3),(8,3)> (수평)
    - 점 pt = (6.7, 3.6):
        - 이 수평선에 직교 투영하면:
        - y=3.0, x=6.7 → q = (6.7, 3.0, 0)
        - t = (6.7 - 6.0) / (8.0 - 6.0) = 0.7 / 2.0 = 0.35
- 그래서 테스트가:
    - dir == UDir
    - i == 3, j == 2
    - q == (6.7, 3.0, 0.0)
    - t == 0.35
- 을 기대하는 건 완전히 맞는 의도.

#### 4-5) netleg_dense_u_leg_case
- 같은 net.
- 타겟 leg:
    - <P[4][1], P[5][1]>
    - P[4][1] = (8.0, 1.5)
    - P[5][1] = (10.0, 1.5)
- 점:
```rust
pt = (9.3, 0.9, 0.0)
```

- 이 점은:
    - y=0.9 → y=1.5 수평선 아래
    - x=9.3 → 8~10 사이
- 이 수평 leg에 투영하면:
    - q = (9.3, 1.5, 0.0)
    - t = (9.3 - 8.0) / (10.0 - 8.0) = 1.3 / 2.0 = 0.65
- 테스트:
```rust
assert_eq!(dir, SurfaceDir::UDir);
assert_eq!(i, 4);
assert_eq!(j, 1);
assert!(approx_pt(q, p(9.3, 1.5, 0.0), 1e-12));
assert!(approx(t, 0.65, 1e-12));
```

- 정확히 이 상황을 검증.

### 5. 전체 의도 정리
- on_get_net_closest_point_leg_index:
    - 컨트롤넷의 모든 U/V 선분에 대해
    - 점 p를 직교 투영하고
    - 선분 구간 안에 있는 투영만 후보로 삼아서
    - 그중 가장 가까운 것을 선택한다.
- on_project_point_to_line_by_intersect_plane:
    - “점→직선 직교 투영 + 선분 안/밖 판정”을 깔끔하게 캡슐화한 함수.
    - 테스트들은:
    - 작은 2x2 net에서 “어떤 leg가 선택되어야 하는지”를 직관적으로 검증
    - 더 큰 rect net에서 “정확한 인덱스(i,j), 방향(dir), 투영점(q), 파라미터(t)”까지 수학적으로 맞는지 검증


### 🎯 이 함수가 필요한 이유
- NURBS나 서피스 모델링에서 control net은 단순한 점 배열이 아니라,  
    서피스의 구조를 이루는 격자형 선분 집합.
    - U 방향 선분: <net[u][v], net[u+1][v]>
    - V 방향 선분: <net[u][v], net[u][v+1]>
- 이 선분들을 “leg”라고 부르는 거고,  
    이 함수는 점 p가 이 leg들 중 어느 것에 가장 가깝나를 찾는 기능을 한다.

### 🎯 왜 “선분 구간 안에 들어오는 투영만” 고려할까?
- 직선에 투영하면 항상 투영점이 나오지만, 그 투영점이 선분 밖에 있을 수도 있음.
- 예를 들어:
```
A ---- B
         p

```
- p를 AB 직선에 투영하면 B보다 오른쪽에 떨어지지만, 그건 선분 AB의 범위 밖이기 때문에  
    **AB에 가장 가깝다** 고 말할 수 없음.
- 그래서:
    - 투영 파라미터 t ∈ [0,1] → 선분 안 → 유효한 후보
    - t < 0 또는 t > 1 → 선분 밖 → 무시
    - 이게 flg가 하는 역할.

### 🎯 이 함수가 실제로 해결하는 문제
- 이 함수는 다음과 같은 상황에서 핵심적으로 쓰임:
#### 1) 서피스 위에서 점을 가장 가까운 구조선(iso-curve)에 스냅하고 싶을 때
- 예:
    - CAD에서 점을 서피스의 U/V 방향 iso-curve에 붙이고 싶을 때
        - 가장 가까운 U/V leg를 찾아야 함.
#### 2) 서피스 파라미터 초기값 찾기
- 서피스에서 점 p의 (u,v)를 찾는 알고리즘(예: Newton iteration)은 초기값이 중요.
- 초기값을 찾기 위해:
    - p에 가장 가까운 U-leg 또는 V-leg를 찾고
    - 그 leg의 파라미터 t를 이용해
    - (u,v) 초기값을 추정할 수 있음
- 이 함수가 바로 그 “초기 leg”를 찾는 역할을 한다.
#### 3) 거리 기반 선택 / 하이라이팅
- 마우스 클릭한 점이 어떤 U/V 선분에 가장 가까운지 판단하는 기능.
#### 4) 메시 snapping / feature detection
- 컨트롤넷 기반의 구조선(feature)을 찾을 때 가장 가까운 leg를 찾는 것이 필요함.

### 🎯 반환값이 의미하는 것
- 함수는 다음을 반환:
```rust
Some((q, i, j, dir, t))
```

    - q: p를 해당 leg에 직교 투영한 점
    - i, j: leg의 시작점 net[i][j]
    - dir: U 방향인지 V 방향인지
    - t: 선분 위의 파라미터 (0~1)
- 즉, 이 정보만 있으면:
    - 어떤 leg인지
    - 그 leg 위에서 어디에 투영되는지
    - 그 투영점이 실제로 어디인지
    - 그 leg의 파라미터는 얼마인지 모두 알 수 있어.

### 🎯 결론
- 이 함수의 목적은 아주 명확.
- 컨트롤넷의 모든 U/V 선분 중에서, 점 p에 대해 **선분 구간 안에 들어오는 직교 투영** 을 가지는
    가장 가까운 leg를 찾는 것.

- 그리고 이건 서피스 파라미터 초기화, 스냅핑, 피처 탐지 등   
    수많은 CAD/NURBS 알고리즘에서 핵심적인 기본 연산.

## on_shell_sort_with_index

### 🧩 1. 이 함수가 하는 일
- ✔ a 배열을 제자리(in-place) 에서 오름차순 정렬한다.
- ✔ 동시에 ind 배열을 만들어 정렬 후의 a[i] 가 원래 입력 배열의  
    몇 번째 요소였는지 알려준다.
- 즉,
```rust
a_sorted[i] == a_original[ind[i]]
```
- 이걸 만족하는 인덱스 매핑을 만들어주는 ShellSort.

### 🧩 2. ShellSort란?
- ShellSort는 삽입정렬(insertion sort) 을 개선한 알고리즘으로:
    - 먼저 큰 간격(gap)으로 떨어진 요소들을 비교/정렬하고
    - 점점 gap을 줄여가며
    - 마지막에 gap=1일 때 일반 삽입정렬을 수행한다.
- 이렇게 하면 멀리 떨어진 요소들이 빠르게 제자리로 이동할 수 있어 삽입정렬의 약점을 크게 줄인다.

### 🧩 3. 이 코드에서의 gap(k) 계산 방식
- k = (5*k - 1) / 11


- 이건 ShellSort에서 흔히 쓰는 Knuth sequence 나 Hibbard sequence 가 아니라  
    특정 CAD/수치 라이브러리에서 쓰던 독자적인 gap 감소 규칙이다.
- 초기값:
```rust
n = len - 1
k = n + 1
```

- 즉, 배열 길이가 10이면:
```rust
n = 9
k = 10
```

- 그 다음 반복:
```rust
k = (5*k - 1)/11
```
- 예시:

| step | k 계산                         | k 값 |
|------|--------------------------------|------|
| 초기 | k = n + 1                      | 10   |
| 1    | (5 × 10 − 1) ÷ 11 = 49 ÷ 11    | 4    |
| 2    | k < 5 → k = 1                  | 1    |

- 즉, gap sequence = 10 → 4 → 1

### 🧩 4. gap 정렬 단계 (핵심)
- 이 부분이 ShellSort의 핵심:
```rust
for i in (0..=(n - k)).rev() {
    let b = a[i];
    let bi = ind[i];

    let mut j = i + k;
    while j <= n && b > a[j] {
        a[j - k] = a[j];
        ind[j - k] = ind[j];
        j += k;
    }

    a[j - k] = b;
    ind[j - k] = bi;
}


- 이건 gap 간격 삽입정렬(gap insertion sort) 이다.
- 동작 방식
    - i 위치의 값 b 를 임시로 저장
    - gap 만큼 떨어진 위치들(j = i+k, i+2k, …)과 비교
    - b 보다 큰 값들을 뒤로 밀어냄
    - b 를 올바른 위치에 삽입
- 즉, gap=k 간격으로 이루어진 부분 배열(subsequence) 들을 각각 삽입정렬하는 것.

### 🧩 5. index 배열(ind) 동기화
- 정렬 과정에서 a 의 요소를 이동시키면 그에 대응하는 원래 인덱스도 같이 이동해야 한다.
- 그래서:
```rust
ind[j - k] = ind[j];
```

- 이렇게 ind 도 동일한 방식으로 이동시킨다.
- 마지막에:
```rust
ind[j - k] = bi;
```

- 즉, b 의 원래 인덱스를 그 자리에 넣는다.

### 🧩 6. 최종적으로 ind 배열이 의미하는 것
- 정렬 후:
    - a = [정렬된 값들]
    - ind = [정렬된 값들의 원래 위치]


- 예시:
- 입력:
```rust
a = [50, 20, 40]
```
- 정렬 후:
```rust
a   = [20, 40, 50]
ind = [1, 2, 0]
```

- 즉:
    - a[0] = 20 → 원래 index 1
    - a[1] = 40 → 원래 index 2
    - a[2] = 50 → 원래 index 0
- 이게 바로:
```rust
a[i] == a_in[ind[i]]
```

- 을 만족한다.

### 🧩 7. 전체 알고리즘 요약
- ind = [0,1,2,...] 초기화
- gap = n+1
- gap = (5*gap - 1)/11 로 줄이기
- gap 간격 삽입정렬 수행
- gap=1 될 때까지 반복
- 정렬된 a + 원래 인덱스 매핑 ind 반환

### 🧩 8. 이 구현의 특징
- 안정 정렬(stable)은 아님 (ShellSort 특성)
- index tracking을 정확히 수행
- gap 감소가 빠르기 때문에 성능이 꽤 좋음
- 작은 배열에서는 insertion sort보다 빠름
- 큰 배열에서는 O(n^(3/2)) 정도 성능

### 🎯 결론
- 이 함수는:
    - ShellSort 기반 정렬
    - 원본 인덱스를 추적하는 index array 생성
- 을 수행하는 정렬 알고리즘이다.

### 소스 코드
```rust
///
/// - `a` is sorted in-place ascending.
/// - returns `ind` such that after sorting: `a[i] == a_in[ind[i]]`.
///
/// Note: In the original C, `n` is the *highest index* (so length = n+1).
/// Here we use `a.len()` directly.
pub fn on_shell_sort_with_index(a: &mut [usize]) -> Vec<usize> {
    let len = a.len();
    if len == 0 {
        return Vec::new();
    }

    // ind[i] = original index of the element now at i
    let mut ind: Vec<usize> = (0..len).collect();

    // In C: n = highest index, so count = n+1
    let n = len - 1;

    // k is gap "count" (C starts from n+1)
    let mut k = n + 1;

    while k > 1 {
        if k >= 5 {
            k = (5 * k - 1) / 11; // same gap shrink as C
        } else {
            k = 1;
        }

        // for (i = n-k; i >= 0; --i)
        // Rust descending loop: (0..=n-k).rev()
        if n >= k {
            for i in (0..=(n - k)).rev() {
                let b = a[i];
                let bi = ind[i];

                // j = i+k; while j<=n && b > a[j] { shift } j+=k
                let mut j = i + k;
                while j <= n && b > a[j] {
                    a[j - k] = a[j];
                    ind[j - k] = ind[j];
                    j += k;
                }

                a[j - k] = b;
                ind[j - k] = bi;
            }
        }
    }

    ind
}
```
---

