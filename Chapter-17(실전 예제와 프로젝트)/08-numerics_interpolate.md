# 수치적분 · 스플라인 · 기하 유틸 요약 (MD)

**적분기(Integrator)**, **B-spline 기초 루틴**, **간단한 기하 유틸**의 핵심 **정의·수식·구현 주의점**을 요약.  
테스트에 사용된 식과 불변식(invariant)도 함께 정리했습니다.

---

## 표기(Notation)

- 구간: $\([a,b]\)$, 길이 $\(L = b-a\)$, 선형변환 $\(x = \tfrac{a+b}{2} + \tfrac{b-a}{2}\,t\)$, $\(t\in[-1,1]\)$  
  편의로 $\(c_1=\tfrac{b-a}{2},\; c_2=\tfrac{a+b}{2}\)$ 사용.
- 체비셰프–로바토 노드: $\(\theta_k=\frac{k\pi}{n},\; t_k=\cos\theta_k,\; x_k=c_2+c_1 t_k,\; k=0,\dots,n\)$
- B-spline: 차수 $\(p\)$, 컨트롤포인트 개수 $\(n+1\)$, 매듭 개수 $\(m+1\)$  
  **관계식:** $\(m = n + p + 1 \iff |U|= n + p + 2\)$.

---

# 1. 1D/2D 수치적분

## 1.1 Simpson (단일 패널)
- 공식

$$
  \int_a^b f(x)\,dx \approx \frac{b-a}{6}\bigl[f(a)+4f(\tfrac{a+b}{2})+f(b)\bigr].
$$

- 정밀도: 구간 길이 $\(h=b-a\)$ 에서 **오차 $\(O(h^5)\)$**, 복합 규칙을 $\(N\)$ 분할로 쓰면 전체 $\(O(N^{-4})\)$.

### Adaptive Simpson
- 한 구간(패널) 적분값을 $\(S\)$, 반분 후 합을 $\(S_1+S_2\)$ 라 할 때

$$
  \text{err} \approx \frac{S_1+S_2 - S}{15}.
$$

- 허용오차(tol) 기준으로 재귀 분할, 리처드슨 보정 $\(S^\* = S_1+S_2 + \frac{S_1+S_2 - S}{15}\)$.

## 1.2 2D Simpson (텐서곱 3×3)
- u, v 각각 Simpson을 적용한 텐서곱

$$
  \iint_{[u_0,u_1]\times[v_0,v_1]} f\,\mathrm{d}u\,\mathrm{d}v
  \approx \frac{(u_1-u_0)(v_1-v_0)}{36}
  \sum_{i,j\in\{0,1,2\}} w_i w_j\, f(u_i,v_j)
$$

단, 1D 가중치 \(w = (1,4,1)\), 격자점은 \((u_0,u_m,u_1)\times(v_0,v_m,v_1)\).

### 2D Adaptive Simpson
- 사분할 재귀(4 패널)로 $\(S\to S_4\)$, 오차 추정 $\(|S_4-S|\)$, 리처드슨 보정 $\(S^\* = S_4 + \frac{S_4-S}{15}\)$.

## 1.3 Gauss–Legendre (24점 고정)
- 표준구간 $\([-1,1]\)$ 의 노드 $\(\xi_i\)$, 가중치 $\(w_i\)$ 를 사용하여

$$
  \int_a^b f(x)\,dx \approx c_1 \sum_{i=1}^{24} w_i\, f(c_1 \xi_i + c_2).
$$

- 2D는 텐서곱 $\(\sum_i\sum_j w_i w_j f(u(\xi_i),v(\xi_j))\)$ 후 $\(c_u c_v\)$ 배.

## 1.4 Clenshaw–Curtis (Lobatto) — **DCT-I 기반 정석 구현**

### 핵심 아이디어
- $\([-1,1]\)$ 의 Chebyshev–Lobatto 노드 $\(t_k=\cos(k\pi/n)\)$ 에서 $\(v_k=f(c_2+c_1 t_k)\)$ 를 샘플.
- **DCT-I**로 Chebyshev 계수 $\(a_j\)$ 를 계산:

$$
  a_j = \frac{2}{n}\Bigl[\frac{1}{2}v_0 + \sum_{k=1}^{n-1} v_k \cos\!\Bigl(\frac{jk\pi}{n}\Bigr)
          + \frac{1}{2}(-1)^j v_n\Bigr],\quad j=0,\dots,n.
$$

- 표준구간 적분은

$$
  \int_{-1}^1 f(t)\,dt = a_0 + 2\sum_{\substack{j\ge 2\\ j\ \text{even}}}\frac{a_j}{1-j^2}.
$$

- 일반구간은 $\([a,b]\)$ 스케일로 $\(c_1\int_{-1}^1 f(c_2+c_1 t)\,dt\)$.

### **비매끈 \(f\)에 대한 분할(Automatic split)**
- $\(|x|\)$, $\(\max(0,x)\)$ 등 **모서리**가 있는 함수는 단일 구간 CC 수렴이 $\(O(n^{-2})\)$.
- $\([a,b]\)$ 가 0을 가로지르면 **$\[a,0] + [0,b]$** 로 자동 분할하면 각 부분이 매끈(선형 등)이라
  머신 정밀도에 근접.

### 수렴/오차 특성
- 해석적(analytic) $\(f\)$: 지수적/spectral 수렴.  
- $\(|x|\)$ 등 1차 연속·미분불연속: $\(O(n^{-2})\)$ — **분할 권장**.

---

# 2. Chebyshev 관련 보조 루틴

- `chebyshev_series(size)`: 테스트용 계수 시퀀스 생성(원본 알고리즘 포팅).
- `dfct / ddct / bitrv`: 간단 대체 구현. 정확도·성능 민감 시 FFT 기반 DCT 라이브러리로 교체 권장.

---

# 3. ODE 관점의 적분 (누적 적분기)

## 3.1 RK4
- 스텝 $\(h\)$ 에서

$$
  y_{n+1} = y_n + \frac{h}{6}(k_1+2k_2+2k_3+k_4),\quad
  k_1=f(x_n),\ k_2=f(x_n+\tfrac{h}{2}),\ k_3=f(x_n+\tfrac{h}{2}),\ k_4=f(x_n+h).
$$

- 1D 적분 $\(\int_a^b f(x)\,dx\)$ 는 $\(y'(x)=f(x),\,y(a)=0\)$ 로 두고 $\(y(b)\)$ 를 반환.

## 3.2 RK45 (Dormand–Prince, 적응 스텝)
- 5차/4차 쌍으로 국소 오차 추정, 수용 시 스텝 증감:

$$
  h_{\text{new}} = h\cdot \mathrm{safety}\cdot\Bigl(\frac{\text{tol}}{\text{err}}\Bigr)^{1/5},
$$\

  범위 제한($\(0.2\sim5.0\)$) 적용.  
- 코드에 사용된 계수는 표준 Dormand–Prince(5(4)) 계수.

---

# 4. 기하 유틸

## 4.1 평면 방정식(정규화 & 방향 고정)
- 세 점 $\(p_0,p_1,p_2\)$ 에서

$$
  \mathbf{n} = \frac{(p_1-p_0)\times(p_2-p_0)}{\|(p_1-p_0)\times(p_2-p_0)\|},\quad
  d = -\mathbf{n}\cdot p_0.
$$

- 테스트 일관성 위해 $\(n_z<0\)$ 이면 $\((\mathbf{n},d)\to(-\mathbf{n},-d)\)$ 로 뒤집음.  
  기대: $\(a\approx0,\; b\approx0,\; c\approx1\)$.

## 4.2 사면체 부피
- 점 $\(a,b,c,d\)$ 에 대해
$$
  V = \frac{1}{6}\bigl| (a-d)\cdot \bigl((b-d)\times(c-d)\bigr) \bigr|.
$$
- 절대값으로 **꼭지점 순서**에 따른 부호 차이를 제거.

---

# 5. 구현 체크리스트 / 팁

- **실수 비교**: `close(a,b)`는 절대+상대 혼합(스케일) 오차 사용.  
- **Clenshaw–Curtis**: 가중치 공식을 직접 합성하지 않음. **반드시 DCT-I**로 계수 → 적분식 조합.  
- **비매끈 함수**: 0을 가로지르면 **자동 분할**.  
- **B-spline**: 반복 매듭에서 **분모 0 가드**. `find_span`의 경계 정의 엄수.  
- **테스트**:  
  - CC: $\(\int_{-1}^1 x^2 dx = 2/3\)$, $\(\int_{-1}^1 e^x dx=e-1/e\)$, $\(\int_{-1}^1|x|dx=1\)$  
  - NURBS/B-spline: $\(\sum N=1\), \(\sum N'=0\)$ 전 구간 샘플.

---

# 6. 복잡도 요약

- Simpson: 패널 O(1), 적응형은 패널 수에 비례.  
- Gauss–Legendre(24점): O(24) $\(\approx\)$ O(1). 2D는 O(24²).  
- Clenshaw–Curtis(DCT-I 단순 구현): O(n²). 다회 사용 시 $\(\cos(jk\pi/n)\)$ 테이블 캐시로 상수 개선 가능.  

---

## 부록: 코드에서의 주요 매핑

- 선형변환: `x = c2 + c1 * cos(theta)` with `theta = k*PI/n`.
- DCT-I 계수:
  ```text
  a_j = (2/n) * ( 0.5*v0 + sum_{k=1}^{n-1} v_k*cos(j*k*pi/n) + 0.5*(-1)^j * v_n )
  ```
- 적분 조합:
  ```text
  ∫_{-1}^1 f = a0 + 2 * Σ_{even j≥2} a_j / (1 - j^2)
  ∫_{a}^b f = c1 * (위 식)
  ```
- RK4 증분:
  ```text
  y += h/6 * (k1 + 2k2 + 2k3 + k4)
  ```
- Tet volume:
  ```text
  V = | (a-d) · ((b-d)×(c-d)) | / 6
  ```

---

### 참고(교과서)
- Trefethen, *Spectral Methods in MATLAB* — Clenshaw–Curtis 적분과 Chebyshev 계수.


