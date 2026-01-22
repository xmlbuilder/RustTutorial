## on_quadrant_theta_range

- 이 함수는 각도 θ를 8분할(π/4 단위) 기준으로 4개의 방향(East, North, West, South)으로 분류할 때  
    사용하는 각도 범위를 반환하는 간단하고 명확한 유틸리티.
- 특히 방향 벡터의 heading angle을 기반으로 사분면(quadrant) 또는 방향 섹터를 판정할 때 유용.

### 📘 on_quadrant_theta_range(q)
- q(0~3)에 대응하는 방향 섹터의 각도 범위 $[θ_{min}, θ_{max}]$ 을 반환

### 1️⃣ 기본 개념
- 각도는 라디안 기준이며,
```math
\frac{\pi }{4}=45^{\circ }
```
- 를 기준으로 8등분한 뒤, 그 중 2개씩 묶어서 4개의 방향 섹터를 만든다.
- 즉:
    - $East  = [-45^{\circ },+45^{\circ }]$
    - $North = [+45^{\circ },+135^{\circ }]$
    - $West  = [+135^{\circ },+225^{\circ }]$
    - $South = [+225^{\circ },+315^{\circ }]$
- 라디안으로 표현하면:
    - $\pm \frac{\pi }{4}$
    - $\frac{3\pi }{4}$
    - $\frac{5\pi }{4}$
    - $\frac{7\pi }{4}$

### 2️⃣ 코드 해설
```rust
let p8 = PI * 0.25; // π/4
```

- ✔ q = 0 → East 방향
```rust
0 => (-p8, p8)
```

- ✔ q = 1 → North 방향
```rust
1 => (p8, 3.0 * p8)
```


- ✔ q = 2 → West 방향
```rust
2 => (3.0 * p8, 5.0 * p8)
```



- ✔ q = 3 또는 기타 → South 방향
```rust
_ => (5.0 * p8, 7.0 * p8)
```


### 📌 기하학적 의미
- 이 함수는 다음 상황에서 매우 유용하다:
    - 방향 벡터의 heading angle을 기반으로 사분면 판정
    - 2D 네비게이션, steering, 로봇 방향 분류
    - CAD에서 edge 방향 분류
    - GIS에서 방향 섹터 분석
    - 원형 데이터의 방향 클러스터링
- 즉, 각도를 4개의 큰 방향(E/N/W/S)으로 분류하기 위한 기준 범위를 제공하는 함수다.

```rust
/// Quadrant (East, North, West, South = 0,1,2,3) angle range
pub fn on_quadrant_theta_range(q: i32) -> (f64, f64) {
    let p8 = PI * 0.25;
    match q {
        0 => (-p8, p8),            // East
        1 => (p8, 3.0 * p8),       // North
        2 => (3.0 * p8, 5.0 * p8), // West
        _ => (5.0 * p8, 7.0 * p8), // South
    }
}
```
## on_direction_quadrant

- 이 함수는 2D 벡터 (x, y)의 방향을 4개의 주요 방향(East, North, West, South) 중  
    하나로 분류하는 알고리즘.
- atan2를 사용해 각도를 얻고, 그 각도가 어느 범위에 속하는지에 따라 방향 코드를 반환하는 구조.

### 📘 on_direction_quadrant(y, x)
- 2D 벡터 (x, y)의 방향을 East / North / West / South 중 하나로 분류

### 1️⃣ 각도 계산
```rust
let theta = y.atan2(x);
```

- atan2(y, x)는 벡터 (x, y)의 방향을 라디안으로 반환하며, 범위는:
```math
\theta \in [-\pi ,+\pi )
```
### 2️⃣ 8분할 기준 각도(π/4) 정의
```rust
let p8 = PI * 0.25; // π/4 = 45°
```

- 이 값은 방향을 8등분하기 위한 기본 단위.

3️⃣ 방향 섹터 분류
- 각도 범위는 다음과 같이 4개의 큰 방향으로 나뉜다:

- 코드 그대로:
```rust
if theta >= -p8 && theta < p8 {
    0 // East
} else if theta >= p8 && theta < 3.0 * p8 {
    1 // North
} else if theta >= 3.0 * p8 && theta < 5.0 * p8 {
    2 // West
} else {
    3 // South
}
```



### 📌 기하학적 의미
- 이 함수는 다음 상황에서 매우 유용해:
    - 2D 벡터의 방향을 단순화하여 4방향으로 분류
    - CAD에서 edge 방향 분류
    - 로봇/드론 heading 방향 분류
    - GIS에서 방향 기반 클러스터링
    - 게임 엔진에서 캐릭터 방향 판정
- 특히 atan2 기반 분류는
    - x=0, y=0 같은 특수 케이스에서도 안정적이라는 장점이 있음.


```rust
/// Returns the quadrant index (0: East, 1: North, 2: West, 3: South)
pub fn on_direction_quadrant(y: f64, x: f64) -> i32 {
    let theta = y.atan2(x); // angle in radians
    let p8 = std::f64::consts::PI * 0.25;

    if theta >= -p8 && theta < p8 {
        0 // East
    } else if theta >= p8 && theta < 3.0 * p8 {
        1 // North
    } else if theta >= 3.0 * p8 && theta < 5.0 * p8 {
        2 // West
    } else {
        3 // South
    }
}
```
## on_canonicalize_periodic

- 이 함수는 **주기적 파라미터(periodic parameter)** 를 도메인 구간 안으로  
    정규화(canonicalize)하는 아주 깔끔한 구현.
- 특히 NURBS, 원호(arc), 주기적 곡선/곡면에서 파라미터를 wrap‑around 처리할 때  
    핵심적으로 쓰이는 패턴.

### 📘 on_canonicalize_periodic(u, dom, periodic)
- 주기적 도메인에서 파라미터 u를 기본 구간으로 정규화

### 1️⃣ periodic이 false면 그대로 반환
```rust
if !periodic {
    return u;
}
```

- 즉, 주기적 도메인이 아닌 경우에는 아무 보정도 하지 않는다.

### 2️⃣ 도메인 길이와 유효성 검사
```rust
let l = dom.length();
if !(l > 0.0) || !u.is_finite() {
    return u;
}
```

- 도메인 길이 l이 0 이하 → wrap 불가능
- u가 NaN 또는 ±∞ → 정규화 불가능
- 이 경우 원래 값을 그대로 반환한다.

### 3️⃣ 주기적 정규화 핵심
```rust
let k = ((u - dom.t0) / l).floor();
u - k * l
```

- 수식으로 표현하면:
```math
k=\left\lfloor \frac{u-t_0}{l}\right\rfloor
```
```math 
u'=u-kl
```
- 즉:
    - u가 도메인 밖에 있어도
    - 도메인 길이 l을 적절히 더하거나 빼서
    - 기본 구간 $[t_0,t_0+l)$ 안으로 이동시킨다.

### 📌 최종 요약

- 이 결과는 항상:
```math
u'\in [t_0,\; t_0+l)
```
- 즉, 주기적 도메인의 기본 구간으로 정규화된 파라미터가 된다.

### 📌 기하학적 의미
- 이 함수는 다음 상황에서 매우 중요해:
    - 주기적 NURBS 곡선/곡면의 파라미터 정규화
    - 원호(arc) 파라미터 wrap-around 처리
    - 각도(θ) 기반 파라미터 정규화
    - 도메인 밖 파라미터를 기본 구간으로 되돌리기
    - 반복적 계산에서 파라미터 폭주 방지
- 예시:
- 도메인 $[0,1)$, 길이 l=1:
    - u = 1.2 → 0.2
    - u = −0.3 → 0.7
    - u = 3.7 → 0.7
- 각도 도메인 $[0,2\pi )$ 에서도 동일하게 동작한다.

```rust
#[inline]
pub fn on_canonicalize_periodic(u: f64, dom: &Interval, periodic: bool) -> f64 {
    if !periodic {
        return u;
    }
    let l = dom.length();
    if !(l > 0.0) || !u.is_finite() {
        return u;
    }
    let k = ((u - dom.t0) / l).floor();
    u - k * l
}
```
---

## on_generate_biased_divisions

- 이 함수는 한쪽으로 치우친(geometric-biased) 분할을 생성하는 고급 분할 알고리즘.
- CAD·Mesh·FEA·곡선 분할 등에서 “한쪽은 촘촘하고 다른 쪽은 성기게” 만드는 데 자주 쓰이는 패턴.

### 📘 on_generate_biased_divisions(total_length, num_div, r, small_at_left)
- 한쪽으로 치우친 기하급수적 분할(geometric division) 생성

### 1️⃣ 목적
- 전체 길이 total_length를 num_div개의 구간으로 나누되,
  - 한쪽은 짧은 구간들(small segments)
  - 반대쪽은 긴 구간들(large segments)
- 이 되도록 기하급수적 비율 r을 이용해 분할점을 생성한다.
- 즉, 다음 형태의 분할을 만든다:
```math
\Delta _1,\; \Delta _2=r\Delta _1,\; \Delta _3=r\Delta _2,\; \dots 
```
### 2️⃣ 입력 파라미터 의미
| 변수 | 설명 | 
|------|------|
| total_length | 전체 길이  | 
| num_div | 분할 갯수 | 
| r | 비율율 | 
| small_at_left | 치우친 방향 결정  | 



### 3️⃣ r 보정 및 방향 처리
- ✔ r ≤ 0 → r = 1
  - 비율이 0 또는 음수면 의미가 없으므로 균일 분할로 처리.
- ✔ r < 1 → 역전(flip) 처리
```rust
if r < 1.0 {
    r = 1.0 / r;
    flip = true;
}
```

- 즉,
  - r < 1은 사실상 r > 1의 반대 방향이므로 1/r로 바꾸고 방향을 뒤집는다.
- ✔ 최종 방향 결정
```rust
let left_small = small_at_left ^ flip;
```

- XOR로 방향 반전 처리
- 최종적으로 왼쪽이 작은지(left_small) 결정

### 4️⃣ r ≈ 1 → 균일 분할
```rust
if (r - 1.0).abs() < 1e-12 {
    lt[i] = i * (total_length / n)
}
```

- 즉:
```math
x_i=\frac{i}{n}L
```
### 5️⃣ 기하급수 분할의 기본 공식
- 기하급수적 길이 합:
```math
S=a(1+r+r^2+\cdots +r^{n-1})
```
```math
S=a\frac{r^n-1}{r-1}
```
- 전체 길이와 같아야 하므로:
```math
a=\frac{L(r-1)}{r^n-1}
```
- 코드:
```rust
let a = total_length / ((r.powi(num_div) - 1.0) / (r - 1.0));
```


### 6️⃣ 분할점 생성
- ✔ 왼쪽이 촘촘한 경우 (left_small = true)
```rust
lt[i] = a * (r^i - 1) / (r - 1)
```

- 즉:
```math
x_i=a\frac{r^i-1}{r-1}
```
- ✔ 오른쪽이 촘촘한 경우 (left_small = false)
- 오른쪽을 촘촘하게 만들기 위해 기본 분할을 뒤집어서(total_length - base[n-i]) 적용.

## 7️⃣ 경계 보정
```rust
lt[0] = 0.0;
lt[n] = total_length;
```

## 📌 최종 요약
- 이 함수는 다음을 수행한다:
- 기하급수 비율 r을 이용해
```math
\Delta _i=ar^{i-1}
```
- 형태의 분할 생성
  - r < 1이면 자동으로 방향 반전
  - small_at_left로 어느 쪽이 촘촘할지 결정
  - r ≈ 1이면 균일 분할로 fallback
  - 결과는 분할점 좌표 배열 (길이 n+1)
- 📌 기하학적 활용이 알고리즘은 다음에서 매우 유용해:- 메시(mesh) 생성 시 경계층(boundary layer) 분할
  - 곡선/곡면 분할 시 한쪽을 더 촘촘하게 sampling
  - CAD에서 fillet, blend, offset 계산 시 분할 품질 향상
  - 수치해석(FEA/CFD)에서 geometric stretching grid 생성
  - Bezier/NURBS subdivision에서 adaptive sampling

```rust
/// One-sided geometric division
pub fn on_generate_biased_divisions(
    total_length: f64,
    num_div: i32,
    mut r: f64,
    small_at_left: bool,
) -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }
    if r <= 0.0 {
        r = 1.0;
    }

    let mut flip = false;
    if r < 1.0 {
        r = 1.0 / r;
        flip = true;
    }
    let left_small = small_at_left ^ flip;

    if (r - 1.0).abs() < 1e-12 {
        let a = total_length / (n as f64);
        for i in 0..=n {
            lt[i] = a * (i as f64);
        }
        return lt;
    }

    let a = total_length / ((r.powi(num_div.max(0)) - 1.0) / (r - 1.0));

    if left_small {
        lt[0] = 0.0;
        for i in 1..=n {
            lt[i] = a * (r.powi(i as i32) - 1.0) / (r - 1.0);
        }
    } else {
        let mut base = vec![0.0; n + 1];
        for i in 1..=n {
            base[i] = a * (r.powi(i as i32) - 1.0) / (r - 1.0);
        }
        for i in 0..=n {
            lt[i] = total_length - base[n - i];
        }
    }
    lt[0] = 0.0;
    lt[n] = total_length;
    lt
}
```
## on_generate_smooth_biased_divisions
- 기하급수적 분할보다 훨씬 부드러운(smooth) 편향 분할을 생성하는 고급 알고리즘.
- Power CDF와 Exponential CDF를 이용해 곡선 형태로 분할 간격을 조절하는 방식,
- 메시 생성, CAD 곡선 분할, FEA/CFD grid 생성 등에서 매우 유용하게 쓰이는 패턴.

### 📘 on_generate_smooth_biased_divisions(l, n, strength, small_at_left, method)
- Power / Exponential CDF 기반의 부드러운 편향(smooth biased) 분할 생성

### 1️⃣ 목적
- 전체 길이 l을 n개의 구간으로 나누되,  
    단순 기하급수 분할보다 **더 부드럽고 자연스러운 편향(smooth bias)**을 적용한다.
    - strength → 편향 강도
    - small_at_left → 왼쪽이 촘촘한지 여부
    - method
    - 0 → Power CDF
    - 1 → Exponential CDF
- 이 방식은 CDF(Cumulative Distribution Function) 형태를 사용하기 때문에  
    분할 간격이 자연스럽게 변화하며, 특히 곡선/곡면 분할에서 시각적으로 매우 좋은 품질을 제공한다.

### 2️⃣ Power CDF 방식 (method = 0)
```math
f\_power(t) = t^{gamma}
```

- 여기서:
```math
\gamma =\left\{ \, \begin{array}{ll}\textstyle 1,&\textstyle \mathrm{strength}\leq 0\\ \textstyle \mathrm{strength},&\textstyle \mathrm{otherwise}\end{array}\right.
``` 
- 특징:
    - $\gamma >1$ → 초반 촘촘, 후반 성김
    - $\gamma <1$ → 초반 성김, 후반 촘촘
    - 매우 부드러운 S-curve 형태

### 3️⃣ Exponential CDF 방식 (method = 1)
```math
f\_exp(t) = (e^{kt} - 1) / (e^k - 1)
```

- 수식:
```math
f(t)=\frac{e^{kt}-1}{e^k-1}
```
- 특징:
    - k>0 → 초반 촘촘, 후반 빠르게 증가
    - k<0 → 초반 성김, 후반 촘촘
    - Power보다 더 강한 편향을 만들 수 있음
    - $k\rightarrow 0$ 이면 선형(t)로 수렴

### 4️⃣ 방향 처리 (왼쪽/오른쪽 편향)
```rust
if small_at_left {
    x[i] = l * f(t)
} else {
    x[i] = l * (1 - f(1 - t))
}
```

- 즉:
- small_at_left = true
```math
x_i=l\cdot f(t)
```
- small_at_left = false
```math
x_i=l\cdot (1-f(1-t))
```
- 이 방식은 CDF의 방향을 뒤집어서 왼쪽 또는 오른쪽이 촘촘하도록 만든다.

### 5️⃣ 분할점 생성
```math
t_i=\frac{i}{n},\quad i=0,\dots ,n
```
```math
x_i=l\cdot F(t_i)
```
- 여기서 F는 Power 또는 Exponential CDF.

### 6️⃣ 경계 보정
```rust
x[0] = 0.0;
x[n] = l;
```


### 📌 최종 요약

- 여기서 F는 Power 또는 Exponential CDF.
- 이 알고리즘은:
    - 기하급수 분할보다 훨씬 부드럽고 자연스러운 편향
    - 강도(strength)로 편향 정도 조절
    - 방향(small_at_left) 선택 가능
    - Power/Exp 두 가지 모드 제공

### 📌 기하학적 활용
- 이 방식은 다음에서 매우 유용:
    - CAD 곡선/곡면 분할 시 부드러운 분할 생성
    - 메시(mesh) 생성에서 boundary layer를 자연스럽게 분할
    - Bezier/NURBS sampling 시 품질 향상
    - 수치해석(FEA/CFD)에서 smooth stretching grid 생성
    - 시각적으로 자연스러운 분할이 필요한 모든 상황
- 특히 Exponential CDF는
    - 기하급수 분할보다 더 강한 편향을 만들 수 있어 고급 mesh generation에서 자주 사용된다.


```rust
/// Left/right skewed “smooth” split (Power/Exp CDF)
/// method: 0=Power(t^gamma), 1=Exp((e^{kt}-1)/(e^k-1))
pub fn on_generate_smooth_biased_divisions(
    l: f64,
    n: i32,
    strength: f64,
    small_at_left: bool,
    method: i32,
) -> Vec<f64> {
    let n = n.max(0) as usize;
    let mut x = vec![0.0; n + 1];
    if n == 0 {
        return x;
    }

    let f_power = |t: f64| -> f64 {
        let gamma = if strength <= 0.0 { 1.0 } else { strength };
        t.clamp(0.0, 1.0).powf(gamma)
    };
    let f_exp = |t: f64| -> f64 {
        let t = t.clamp(0.0, 1.0);
        let k = strength;
        if k.abs() < 1e-12 {
            return t;
        }
        let ek = k.exp();
        let ekt = (k * t).exp();
        (ekt - 1.0) / (ek - 1.0)
    };
    let f = |t: f64| -> f64 { if method == 0 { f_power(t) } else { f_exp(t) } };

    for i in 0..=n {
        let t = (i as f64) / (n as f64);
        x[i] = if small_at_left {
            l * f(t)
        } else {
            l * (1.0 - f(1.0 - t))
        };
    }
    x[0] = 0.0;
    x[n] = l;
    x
}
```
## on_generate_smooth_symmetric_bias

- 이 함수는 전체 길이를 가운데가 가장 촘촘하고 양쪽 끝으로 갈수록  
    점점 성기게 분할하는 “대칭적(symmetric) smooth bias” 분할 알고리즘.
- 이전 함수들이 한쪽으로 치우친(biased) 분할이었다면,
    이번 함수는 중앙 집중형(center‑focused) 분할을 만드는 것이 핵심.
- CAD, 곡선 분할, 메시 생성, FEA/CFD grid 생성에서 **가운데가 촘촘한 분할** 은  
    매우 자주 쓰이는 패턴

### 📘 on_generate_smooth_symmetric_bias(total_length, num_div, k, eps)
- 중앙이 촘촘하고 양쪽이 성긴 대칭적(symmetric) smooth bias 분할 생성

### 1️⃣ 목적
- 전체 길이 L을 n개의 구간으로 나누되,
    - 가운데(midpoint) 근처는 촘촘하게
    - 양쪽 끝은 점점 성기게
- 되는 대칭적(symmetric) 분할을 생성한다.
- 이 패턴은 다음과 같은 상황에서 매우 유용하다:
    - 곡선/곡면의 중앙부를 더 정밀하게 sampling
    - 메시(mesh)에서 중앙 집중형 grid 생성
    - FEA/CFD에서 boundary layer가 아닌 center layer를 강조할 때
    - 시각적으로 자연스러운 분할이 필요한 CAD 작업

### 2️⃣ 핵심 아이디어
- 각 구간의 길이를 직접 계산하는 대신, 먼저 **가중치(weight)** 를 만들고  
    그 가중치를 전체 길이에 비례하도록 정규화한다.
- 즉:
```math
\Delta _i=L\cdot \frac{w_i}{\sum _jw_j}
```
- 여기서 $w_i$ 는 각 구간의 “중요도”를 나타내는 값.

### 3️⃣ 가중치 생성 방식
```rust
let t = (i + 0.5) / n
let s = sin(π t)
w[i] = eps + s^k
```

- 수식으로 표현하면:
```math
t_i=\frac{i+0.5}{n}
```
```math
s_i=\sin (\pi t_i)
```
```math
w_i=\varepsilon +s_i^{\max (k,0)}
```
- ✔ 왜 sin(π t)인가?
```math
\sin (\pi t)
```
- 은 다음 특징을 가진다:
    - t=0,1에서 0
    - t=0.5에서 1
    - 완벽한 대칭
    - 가운데가 가장 크고 양쪽 끝이 0으로 떨어짐
- 즉, 중앙 집중형 분포를 만들기 위한 완벽한 선택.
- ✔ k의 역할
    - k>1: 중앙 집중이 더 강해짐
    - k=1: 기본적인 smooth center bias
    - 0<k<1: 완만한 bias
    - k=0: 모든 s^0 = 1 → 균일 분할
    - k<0: clamp되어 0으로 처리됨 (k.max(0.0))
- ✔ eps의 역할
```math
w_i=\varepsilon +s_i^k
```
- eps > 0이면 양쪽 끝이 완전히 0이 되는 것을 방지
- eps가 클수록 bias가 약해짐
- eps=0이면 완전한 center‑focused shape

### 4️⃣ 가중치 정규화
```rust
seg[i] = total_length * (w[i] / w_sum)
```

- 즉:
```math
\Delta _i=L\cdot \frac{w_i}{\sum _jw_j}
```

### 5️⃣ 누적하여 분할점 생성
```rust
lt[i+1] = lt[i] + seg[i]
```

- 결과:
```math
0=x_0<x_1<x_2<\cdots <x_n=L
```

### 📌 최종 요약

- 여기서
```math
w_j=\varepsilon +\sin ^k\left( \pi \frac{j+0.5}{n}\right)
``` 
- 이 알고리즘은:
    - 완벽한 대칭성
    - 부드러운(smooth) 변화
    - 중앙 집중형 분할
    - k와 eps로 bias 강도 조절
- 을 제공한다.

### 📌 기하학적 활용
- 이 방식은 다음에서 매우 유용:
    - 곡선/곡면의 중앙부를 더 정밀하게 sampling
    - 메시(mesh)에서 center‑focused grid 생성
    - spline/Bezier/NURBS subdivision 품질 향상
    - 시각적으로 자연스러운 분할 생성
    - FEA/CFD에서 symmetric stretching grid 생성
- 특히 sin 기반 분포는 가장 부드러운 symmetric bias를 제공하기 때문에  
    고급 CAD/Simulation 엔진에서 자주 사용된다.


```rust
/// Smooth symmetric division (sin^k)
pub fn on_generate_smooth_symmetric_bias(
    total_length: f64,
    num_div: i32,
    k: f64,
    eps: f64,
) -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }

    let mut w = vec![0.0; n];
    for i in 0..n {
        let t = (i as f64 + 0.5) / (n as f64);
        let s = (PI * t).sin();
        w[i] = eps + s.powf(k.max(0.0));
    }
    let w_sum: f64 = w.iter().sum();
    let mut seg = vec![0.0; n];
    for i in 0..n {
        seg[i] = total_length * (w[i] / w_sum.max(1e-300));
    }
    for i in 0..n {
        lt[i + 1] = lt[i] + seg[i];
    }
    lt[n] = total_length;
    lt
}
```
```rust
pub fn on_generate_symmetric_geometric_bias(total_length: f64, num_div: i32, r: f64) 
    -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }

    let mut w = vec![0.0; n];
    for i in 0..n {
        let a = r.powi(i as i32);
        let b = r.powi((n - 1 - i) as i32);
        w[i] = a.min(b);
    }
    let wsum: f64 = w.iter().sum();
    let mut seg = vec![0.0; n];
    for i in 0..n {
        seg[i] = total_length * (w[i] / wsum.max(1e-300));
    }

    for i in 0..n {
        lt[i + 1] = lt[i] + seg[i];
    }
    lt[n] = total_length;
    lt
}
```
```rust
/// Symmetric geometric partitioning (middle dense)
pub fn on_generate_symmetric_geometric_bias(total_length: f64, num_div: i32, r: f64) -> Vec<f64> {
    let n = num_div.max(0) as usize;
    let mut lt = vec![0.0; n + 1];
    if n == 0 {
        return lt;
    }

    let mut w = vec![0.0; n];
    for i in 0..n {
        let a = r.powi(i as i32);
        let b = r.powi((n - 1 - i) as i32);
        w[i] = a.min(b);
    }
    let wsum: f64 = w.iter().sum();
    let mut seg = vec![0.0; n];
    for i in 0..n {
        seg[i] = total_length * (w[i] / wsum.max(1e-300));
    }

    for i in 0..n {
        lt[i + 1] = lt[i] + seg[i];
    }
    lt[n] = total_length;
    lt
}
```
## on_make_frame_matrix
- 이 함수는 주어진 원점 O와 축 후보 벡터 ex, ey, ez_hint로부터 안정적이고 직교정규화된 3D 좌표계    
    (프레임)를 생성하는 고급 Frame Construction 알고리즘.
- CAD·Geometry 엔진에서 “임의의 벡터로부터 안정적인 좌표계 만들기”는 매우 중요한데,  
    이 구현은 수치적 안정성까지 잘 고려된 훌륭한 형태.

### 📘 on_make_frame_matrix(o, ex, ey, ez_hint)
- 주어진 벡터들로부터 안정적이고 직교정규화된 3D 프레임(Transform)을 생성

### 1️⃣ 목적
- 입력:
    - 원점 O
    - X축 후보 벡터 $e_x$
    - Y축 후보 벡터 $e_y$
    - 보조 Z축 힌트 $e_{z,\mathrm{hint}}$
- 출력:
    - 직교정규화된 3축 X,Y,Z
    - 원점 O를 포함한 4×4 변환 행렬
- 즉,
    - $\{ X,Y,Z,O\}$ 로 구성된 **정규 직교 좌표계(orthonormal frame)** 를 만든다.

### 2️⃣ X축 정규화
```rust
let x = ex.unitize();
```
```math
X=\frac{e_x}{\| e_x\| }
```

### 3️⃣ Y축 후보를 X에 대해 정규직교화
```rust
let y_raw = ey - x * dot(ey, x);
let mut y = y_raw.unitize();
```

- 수식:
```math
Y_{\mathrm{raw}}=e_y-(e_y\cdot X)X
```
- 즉, Gram–Schmidt 정규직교화의 첫 단계.

### 4️⃣ ey가 X와 거의 평행하면 보정
```rust
if !y.is_valid() || y.length() < 1e-14 {
    let y_alt = ex.cross(ez_hint).cross(ex);
    y = y_alt.unitize();
}
```

- 이 부분이 매우 중요.
    - ey가 X와 거의 평행하면  
        $Y_{\mathrm{raw}}$ 는 0에 가까워져 수치적으로 불안정
    - 이를 방지하기 위해 $(ex × ez_hint) × ex$ 를 사용
    - X에 수직인 안정적인 대체 Y축을 생성
- 수식:
```math
Y_{\mathrm{alt}}=(e_x\times e_{z,\mathrm{hint}})\times e_x
```
- 이 벡터는 항상 X에 수직이다.

### 5️⃣ Z축 생성
```rust
let z = x.cross(&y).unitize();
```
```math
Z=\frac{X\times Y}{\| X\times Y\| }
```

### 6️⃣ Y축 재정의 (정확한 직교성 보장)
```rust
let y = z.cross(&x).unitize();
```

- 이 단계는 수치적 drift를 제거하기 위한 재정렬 과정.
- 최종적으로:
    - X ⟂ Y
    - Y ⟂ Z
    - Z ⟂ X
- 모두 단위 벡터
- 즉, 완전한 **정규 직교 기저(orthonormal basis)** 가 된다.

### 7️⃣ 4×4 변환 행렬 구성
```rust
Transform::from_cols(
    [x.x, x.y, x.z, 0.0],
    [y.x, y.y, y.z, 0.0],
    [z.x, z.y, z.z, 0.0],
    [o.x, o.y, o.z, 1.0],
)
```

- 가정:
    - **열(column)** 이 축 벡터를 나타냄
    - 마지막 열이 원점 O
    - 마지막 행은 [0,0,0,1]
- 즉, 행렬은 다음 형태:
```math
T=\left[ \begin{matrix}X_x&Y_x&Z_x&O_x\\ X_y&Y_y&Z_y&O_y\\ X_z&Y_z&Z_z&O_z\\ 0&0&0&1\end{matrix}\right]
``` 

- 📌 최종 요약
- 이 함수는 다음을 수행한다:
    - X축을 정규화
    - Y축을 X에 대해 정규직교화
    - Y축이 불안정하면 ez_hint를 이용해 보정
    - Z = X × Y
    - Y = Z × X 로 재정렬
    - 완전한 직교정규 프레임 생성
    - 4×4 변환 행렬로 반환
- 즉,

### 📌 기하학적 활용
- 이 함수는 다음 상황에서 매우 중요해:
    - 로컬 좌표계(local frame) 생성
    - 곡선/곡면의 Frenet frame 또는 안정적 frame 생성
    - 3D 모델링에서 임의 방향의 축 생성
    - 카메라/조명/오브젝트의 orientation 구성
    - NURBS tangent/normal 기반 frame 생성
- 특히 ey가 ex와 거의 평행할 때의 보정 로직은 실전 CAD 엔진에서 반드시 필요한 안정성 요소.


```rust
pub fn on_make_frame_matrix(
    o: &Point3D,
    ex: &Vector3D,
    ey: &Vector3D,
    ez_hint: &Vector3D,
) -> Transform {
    let x = ex.unitize();
    // Y를 X에 수직 성분만 남겨 정규화
    let y_raw = ey - x * Vector3D::dot(&ey, &x);
    let mut y = y_raw.unitize();
    if !y.is_valid() || y.length() < 1e-14 {
        // ey가 좋지 않으면 (ex×ez)×ex 로 보정
        let y_alt = ex.cross(&ez_hint).cross(&ex);
        y = y_alt.unitize();
    }
    let z = x.cross(&y).unitize();
    let y = z.cross(&x).unitize();

    // ⚠️ Assumption: the following assumes a 4×4 constructor 
    // where columns represent axes and the last column is the origin
    // Use a constructor that matches your project’s Transform convention
    Transform::from_cols(
        [x.x, x.y, x.z, 0.0],
        [y.x, y.y, y.z, 0.0],
        [z.x, z.y, z.z, 0.0],
        [o.x, o.y, o.z, 1.0],
    )
}
```

## on_core_edge_idx_ccw
- 이 함수는 사각형 격자(grid)의 네 개 모서리(edge)를 반시계(CCW) 방향으로 순회하면서,  
    k번째 점의 인덱스를 계산하는 매우 간결하고 정확한 유틸리티.
- 특히 (core_u + 1) × (core_v + 1) 형태의 정규 격자에서 동·북·서·남(E/N/W/S) 방향의  
    모서리 인덱스를 일관되게 얻는 데 최적화되어 있음.


### 📘 on_core_edge_idx_ccw(q, k, core_u, core_v)
- 정규 격자의 네 모서리를 CCW 순서로 순회하며 k번째 점의 인덱스를 반환

### 1️⃣ 격자 인덱싱 규칙
- 내부적으로 격자 점의 인덱스는 다음과 같이 계산된다:
```rust
idx(i, j) = j * (core_u + 1) + i
```

- 즉:
    - i: x 방향 인덱스 (0 … core_u)
    - j: y 방향 인덱스 (0 … core_v)
- 전체 격자는:
```math
(\mathrm{core\_ u}+1)\times (\mathrm{core\_ v}+1)
```
- 크기의 정방형 또는 직사형 grid.

### 2️⃣ q에 따른 CCW 방향 정의
- q는 0~3의 값을 가지며,  
    반시계(CCW) 순서로 다음 방향을 의미한다:

| q | Direction | Description                     |
|---|-----------|---------------------------------|
| 0 | East      | Right edge, j increasing        |
| 1 | North     | Top edge, i decreasing          |
| 2 | West      | Left edge, j decreasing         |
| 3 | South     | Bottom edge, i increasing       |



### 3️⃣ 각 방향에서 k번째 점의 인덱스
- ✔ q = 0 → East edge (오른쪽 세로 모서리, j ↑)
```rust
idx(core_u, k)
```
```math
(i,j)=(\mathrm{core\_ u},\; k)
```

- ✔ q = 1 → North edge (위쪽 가로 모서리, i ↓)
```rust
idx(core_u - k, core_v)
```
```math
(i,j)=(\mathrm{core\_ u}-k,\; \mathrm{core\_ v})
```
- ✔ q = 2 → West edge (왼쪽 세로 모서리, j ↓)
```rust
idx(0, core_v - k)
```
```math
(i,j)=(0,\; \mathrm{core\_ v}-k)
```
- ✔ q = 3 → South edge (아래쪽 가로 모서리, i ↑)
```rust
idx(k, 0)
```
```math
(i,j)=(k,\; 0)
```
### 📌 최종 요약

- 그리고 최종 인덱스는:
```math
\mathrm{idx}(i,j)=j(\mathrm{core\_ u}+1)+i
```
### 📌 기하학적 의미
- 이 함수는 다음 상황에서 매우 유용:
    - 사각형 패치의 네 모서리를 CCW 순서로 순회
    - NURBS/Bezier 패치의 경계 곡선 인덱싱
    - Structured grid에서 boundary loop 생성
    - Mesh stitching / patch 연결
    - Parametric domain의 경계 인덱스 계산
- 즉, 정규 격자의 네 모서리를 일관된 CCW 순서로 접근하기 위한 핵심 유틸리티.


```rust
/// Core edge index (mapped to always proceed CCW)
pub fn on_core_edge_idx_ccw(q: i32, k: i32, core_u: i32, core_v: i32) 
    -> i32 {
    let idx = |i: i32, j: i32| j * (core_u + 1) + i;
    match q {
        0 => idx(core_u, k),          // East (j up)
        1 => idx(core_u - k, core_v), // North (i down)
        2 => idx(0, core_v - k),      // West (j down)
        _ => idx(k, 0),               // South (i up)
    }
}
```
## on_cholesky_decompose_spd
- 이 함수는 대칭 양의 정부호(SPD: Symmetric Positive Definite) 행렬의 Cholesky  
    분해를 직접 구현한 코드.
- 수치해석 교과서에 나오는 표준 Cholesky 알고리즘을 row‑major dense 배열에 맞게 정확히 구현했고,  
    SPD가 아니면 false를 반환하는 안정성.

### 📘 on_cholesky_decompose_spd(a, n)
- 대칭 양의 정부호(SPD) 행렬 A를 Cholesky 분해하여 A = L·Lᵀ 형태로 변환

### 1️⃣ 목적
- 입력:
    - a: 크기 $n\times n$ 의 대칭 행렬 (row-major)
    - n: 행렬 크기
- 출력:
    - 성공 시: a는 하삼각 행렬 L로 덮어쓰기(overwrite)됨  
        (상삼각은 0으로 채움)
- 실패 시: false (SPD가 아님)
- 즉, 이 함수는:
```math
A=LL^{\top }
```
- 형태의 Cholesky 분해를 수행한다.

### 2️⃣ 알고리즘 개요
- Cholesky 분해는 다음 점화식을 사용한다:
- ✔ 대각 원소 (i = j)
```math
L_{ii}=\sqrt{A_{ii}-\sum _{k=0}^{i-1}L_{ik}^2}
```
- ✔ 비대각 원소 (i > j)
```math
L_{ij}=\frac{1}{L_{jj}}\left( A_{ij}-\sum _{k=0}^{j-1}L_{ik}L_{jk}\right)
``` 
- 이 코드가 정확히 이 공식을 구현하고 있다.

### 3️⃣ 코드 해설
- ✔ (1) i, j 루프 — 하삼각만 계산
```rust
for i in 0..n {
    for j in 0..=i {
```

- 즉, $i\geq j$ 인 부분만 계산 → L의 하삼각.

- ✔ (2) 기존 A[i,j]에서 dot-product 제거
```rust
let mut s = a[i*n + j];
for k in 0..j {
    s -= a[i*n + k] * a[j*n + k];
}
```

- 수식:
```math
s=A_{ij}-\sum _{k=0}^{j-1}L_{ik}L_{jk}
```

- ✔ (3) 대각 원소 처리
```rust
if i == j {
    if s <= 0.0 { return false; }
    a[i*n + j] = s.sqrt();
}
```

- SPD가 아니면 $s\leq 0$ → 분해 불가 → false
- 정상적이면 L_{ii}=\sqrt{s}

- ✔ (4) 비대각 원소 처리
```rust
else {
    a[i*n + j] = s / a[j*n + j];
}
```
- 즉:
```math
L_{ij}=\frac{s}{L_{jj}}
```

- ✔ (5) 상삼각을 0으로 정리
```rust
for j in (i+1)..n {
    a[i*n + j] = 0.0;
}
```

- 결과적으로 a는 완전한 하삼각 L이 된다.

### 4️⃣ 최종 반환
```rust
true
```
- 분해 성공.

### 📌 최종 요약
- 이 함수는 다음을 수행한다:
    - 입력 행렬 A가 SPD인지 검사
    - SPD이면 Cholesky 분해 수행
    - 결과를 하삼각 행렬 L로 a에 덮어쓰기
    - 상삼각은 0으로 정리
    - SPD가 아니면 false 반환

### 📌 기하학적/수치적 활용
- Cholesky는 다음에서 매우 중요:
    - 선형 시스템 Ax=b 빠른 해법
    - SPD 행렬 기반 최적화
    - 공분산 행렬 분해
    - 수치 안정성이 중요한 CAD/Simulation
    - NURBS fitting, least-squares, smoothing
- 이 구현은 dense SPD 행렬에 대해 매우 효율적이고 안정적.

```rust
fn on_cholesky_decompose_spd(a: &mut [f64], n: usize) -> bool {
    // a: row-major 상삼각/하삼각 모두 들어있는 dense 대칭
    for i in 0..n {
        for j in 0..=i {
            let mut s = a[i * n + j];
            for k in 0..j {
                s -= a[i * n + k] * a[j * n + k];
            }
            if i == j {
                if s <= 0.0 {
                    return false;
                }
                a[i * n + j] = s.sqrt();
            } else {
                a[i * n + j] = s / a[j * n + j];
            }
        }
        // 상삼각은 0으로 정리(선택)
        for j in (i + 1)..n {
            a[i * n + j] = 0.0;
        }
    }
    true
}
```

---

