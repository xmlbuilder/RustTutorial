
# Hessian vs Weingarten Matrix  

- Weingarten matrix(Shape operator) 와 Hessian matrix 는 **둘 다 2차 미분을 다룬다** 는 점에서 비슷하지만,
  역할·정의·좌표계·기하적 의미가 완전히 다르다.
- 그래도 둘 사이에는 아주 중요한 연결 고리가 존재.
- 이걸 정확히 이해하면 곡률 계산, NURBS surface 미분, FEM 쉘 요소까지 전부 명확해진다.

## 1) Hessian Matrix가 뭔가?
- Hessian은 스칼라 함수 f(x,y) 의 2차 미분을 모아놓은 행렬.
```math
H_f=\left[ \begin{matrix}f_{xx}&f_{xy}\\ f_{yx}&f_{yy}\end{matrix}\right]
``` 
- 즉,
    - 입력: 스칼라 함수
    - 출력: 2차 미분 정보
    - 의미: 함수의 굽음(curvature), convexity, 최적화 등
- Hessian은 스칼라 함수의 곡률을 다루는 도구,
    - 곡면(surface)의 기하학적 곡률을 직접 다루는 도구는 아니야.

## 2) Weingarten Matrix(Shape Operator)는?
- Weingarten matrix는 곡면의 법선벡터 변화율을 나타내는 2×2 선형 변환.
```math
W=-\left[ \begin{matrix}n_u\cdot S_u&n_u\cdot S_v\\ n_v\cdot S_u&n_v\cdot S_v\end{matrix}\right]
``` 
- 여기서:
    - $S(u,v)$: 곡면
    - $S_u$, $S_v$: 1차 미분 (tangent vectors)
    - $n$ : 단위 법선
    - $n_u$, $n_v$ : 법선의 변화율
- Weingarten matrix는 곡면의 곡률 텐서(curvature tensor) 그 자체.
    - 고유값 = 주곡률
    - trace = 평균곡률
    - det = 가우스 곡률
- 즉, 곡면의 기하학적 곡률을 직접적으로 표현하는 행렬이다.

## 3) 둘의 차이 (핵심 요약)

| 항목 | Hessian Matrix | Weingarten Matrix |
|------|----------------|-------------------|
| 다루는 대상 | 스칼라 함수 f(x,y) | 곡면 S(u,v) |
| 의미 | 함수의 굽음(curvature), convexity | 곡면의 기하학적 곡률(curvature tensor) |
| 구성 | 2차 편미분 $f_xx$, $f_xy$, $f_yx$, $f_yy$ | 법선벡터 변화율 $n_u$, $n_v$ |
| 결과 | 함수의 최적화, convexity 분석 | 주곡률, 평균곡률, 가우스곡률 |
| 적용 분야 | 최적화, 머신러닝, PDE | NURBS, CAD, FEM 쉘 요소, 그래픽스 |


- 둘 다 **2차 미분** 을 다루지만,
    - Hessian은 함수의 곡률, Weingarten은 곡면의 기하학적 곡률을 다룬다.

## 4) 그런데… 둘은 연결되어 있다 (중요)
- 곡면이 implicit surface 로 주어졌다고 해보자:
```math
F(x,y,z)=0
```
- 이때 곡면의 법선은:
```math
n=\frac{\nabla F}{\| \nabla F\| }
```
- 여기서 Weingarten matrix는 Hessian으로 표현될 수 있다.
```math
W=-\frac{1}{\| \nabla F\| }\left( I-nn^T\right) H_F
```
- 즉,
    - implicit surface에서는 Weingarten matrix가 Hessian에서 직접 나온다.
- 이게 바로 둘의 “수학적 연결 고리”.

## 5) NURBS에서는 어떤 관계?
- NURBS surface는 implicit이 아니라 parametric 형태:
```math
S(u,v)
```
- 그래서 Hessian이 아니라:
    - $S_u$, $S_v$
    - $S_{uu}$, $S_{uv}$, $S_{vv}$
    - 법선 $n$
    - 법선 미분 $n_u$, $n_v$ 
- 이걸 이용해 Weingarten matrix를 만든다.
- 즉, NURBS에서는 Hessian이 직접 등장하지 않는다,
- 하지만 2차 미분 텐서가 Weingarten matrix를 구성한다는 점에서 개념적으로는 비슷한 구조를 가진다.

## 6) 결론
- ✔ Weingarten matrix는 곡면의 기하학적 곡률을 나타내는 행렬
- ✔ Hessian은 스칼라 함수의 2차 미분 행렬
- ✔ implicit surface에서는 Weingarten = Hessian 기반
- ✔ parametric surface(NURBS)에서는 Weingarten = 2차 미분 + 법선 변화율 기반
- ✔ 둘은 “2차 미분”이라는 공통점이 있지만 역할이 다르다

---

## (Metric 제거 관점에서 본 곡률 이론 정리)

- 이 문서는 표면 곡률 계산에서 자주 혼동되는 **Hessian matrix**와  
    **Weingarten matrix (Shape Operator)** 의 관계를 수식 단계별로 명확히 설명한다.

- 핵심 질문은 다음이다:
    - Hessian과 Weingarten은 전혀 다른 것인가, 아니면 같은 현상을 다른 좌표계에서 표현한 것인가?

- 결론부터 말하면:
    - 같은 기하학적 현상을 다루지만,  
    - Hessian은 **좌표 의존적 표현**,  
    - Weingarten은 **metric이 제거된 기하학적 표현”이다.**

---

## 1. 표면과 도함수 정의

- 표면을 다음과 같이 둔다.

```math
\mathbf{S}(u,v) \in \mathbb{R}^3
```

### 1차 도함수 (접벡터)
```math
\mathbf{S}_u,\quad \mathbf{S}_v
```

### 2차 도함수 (Hessian의 재료)
```math
\mathbf{S}_{uu},\quad \mathbf{S}_{uv},\quad \mathbf{S}_{vv}
```

---

## 2. First Fundamental Form (Metric)
- 좌표계 (u,v)가 공간에서 얼마나 늘어나고 비틀리는지를 나타내는 행렬:

```math
I =
\begin{bmatrix}
E & F \\
F & G
\end{bmatrix}
```

- 여기서

```math
\begin{aligned}
E &= \mathbf{S}_u \cdot \mathbf{S}_u \\
F &= \mathbf{S}_u \cdot \mathbf{S}_v \\
G &= \mathbf{S}_v \cdot \mathbf{S}_v
\end{aligned}
```

### 의미
- **Metric**  
    - 좌표계의 스케일, 왜곡, 비직교성 포함
    - Hessian이 불안정해지는 주 원인

---

## 3. Second Fundamental Form

- 먼저 단위 법선을 정의한다.

```math
\mathbf{N} =
\frac{\mathbf{S}_u \times \mathbf{S}_v}
{\|\mathbf{S}_u \times \mathbf{S}_v\|}
```

- 그 다음 2차 형식:

```math
II =
\begin{bmatrix}
e & f \\
f & g
\end{bmatrix}
```

```math
\begin{aligned}
e &= \mathbf{N} \cdot \mathbf{S}_{uu} \\
f &= \mathbf{N} \cdot \mathbf{S}_{uv} \\
g &= \mathbf{N} \cdot \mathbf{S}_{vv}
\end{aligned}
```

### 주의
- 여전히 (u,v) 좌표계에 묶여 있음
- 아직 **기하학적 불변량** 은 아님

---

## 4. 핵심 단계: Metric 제거 (Weingarten Map)

### 정의

Weingarten map (Shape Operator):

```math
\boxed{
\mathbf{W} = I^{-1} II
}
```

즉,

```math
\mathbf{W} =
\begin{bmatrix}
E & F \\
F & G
\end{bmatrix}^{-1}
\begin{bmatrix}
e & f \\
f & g
\end{bmatrix}
```

---

### 4.1 Metric 역행렬

```math
I^{-1} =
\frac{1}{EG - F^2}
\begin{bmatrix}
G & -F \\
-F & E
\end{bmatrix}
```

---

### 4.2 완전 전개된 Weingarten 행렬

```math
\mathbf{W} =
\frac{1}{EG - F^2}
\begin{bmatrix}
G e - F f & G f - F g \\
- F e + E f & - F f + E g
\end{bmatrix}
```

### 의미
- 좌표계 왜곡(E,F,G)을 제거한 **순수 곡률 연산자**
- 파라미터 재정의(u,v 변경)에 불변

---

## 5. 주곡률 (Principal Curvatures)

- Weingarten 행렬의 고유값:

```math
\det(\mathbf{W} - k I) = 0
```

- 해:

```math
k_1,\; k_2
```

---

## 6. Gaussian / Mean Curvature 공식의 정체

### Gaussian Curvature

```math
K = \frac{eg - f^2}{EG - F^2}
```

### Mean Curvature

```math
H = \frac{E g - 2F f + G e}{2(EG - F^2)}
```

- 👉 이것들은 **Weingarten 고유값의 불변량 표현**

---

## 7. Hessian과 Weingarten의 관계 요약

### Hessian 관점
- $S_{uu}$, $S_{uv}$, $S_{vv}$
- 좌표계(u,v)에 강하게 의존
- Mesh / FEM / 수치 해석에 적합

### Weingarten 관점
- $I^{-1} II$
- 좌표 불변
- CAD / Geometry Kernel에 적합

---

## 8. 직관적 한 줄 요약

-  **Weingarten = Hessian ÷ Metric**

- 또는
    -  **Hessian은 얼마나 변했는지**,  
    -  **Weingarten은 “진짜 얼마나 휘었는지**

---

## 9. 관점 차이 정리

| 관점 | Hessian | Weingarten |
|----|----|----|
| 좌표계 | 절대적(u,v 의존) | 상대적(불변) |
| 수치 | 민감 | 안정 |
| 용도 | Mesh / FEM | CAD / 곡률 |
| 의미 | 변화량 | 형상 |

---

## 10. 최종 핵심 문장

- **Metric을 제거하는 순간, 좌표의 세계에서 기하학의 세계로 넘어간다.**

---


## 🌊 1. 다루는 **곡률** 은 훨씬 깊다
- 지금 다루는 건 곡률의 구조 자체.
- ✔ 곡률을 만드는 텐서
    - First Fundamental Form (metric tensor)
    - Second Fundamental Form (curvature tensor)
    - Weingarten matrix (shape operator)
- ✔ 곡률의 고유값/고유벡터
    - principal curvature k1, k2
    - principal directions
- ✔ 곡률의 조합
    - Gaussian curvature K = k1·k2
    - Mean curvature H = (k1 + k2)/2
- ✔ 곡률의 변화율
    - ∂κ/∂u, ∂κ/∂v
    - ∂²κ/∂u², ∂²κ/∂u∂v, ∂²κ/∂v² (FEM에서 필요)
- ✔ 곡률을 계산하는 방식의 차이
    - parametric surface curvature (NURBS)
    - implicit surface curvature (Hessian 기반)
    - discrete curvature (mesh 기반)


## 🌊 2. 왜 이렇게 깊게 구별해야 하는가?
- ✔ 1) 목적이 다르기 때문
    - CAD/NURBS → 곡률 자체가 필요
    - FEM → 곡률의 변화율이 필요
    - Optimization → 스칼라 함수의 Hessian이 필요
    - Geometry → 법선 변화율이 필요
- 이걸 섞어버리면 계산이 완전히 틀어짐.

- ✔ 2) 같은 **2차 미분** 이라도 의미가 다르기 때문
    - S(u,v)의 2차 미분 → 곡률 텐서
    - κ(u,v)의 2차 미분 → 곡률의 Hessian
    - F(u,v)의 2차 미분 → 최적화 Hessian
- 이걸 구분하지 않으면 **왜 Hessian인데 곡률이 안 나오지?** 같은 혼란이 생김.

- ✔ 3) NURBS는 rational 구조라 더 복잡
    - numerator/denominator
    - rational derivatives
    - symbolic differentiation
    - Weingarten 계산 시 rational term 처리
- 이건 일반적인 미분기하보다 훨씬 난이도가 높음.

## 🎯 핵심 요약 — 단 3줄
- Weingarten = 곡면 자체의 고유한 곡률 텐서
    - metric(좌표계 영향)을 제거해서 순수한 곡률만 남긴 것
- Hessian = 어떤 스칼라 함수의 2차 미분
    - 좌표계 영향 + tangent + normal 성분이 전부 섞여 있음
- FEM은 곡률의 변화율이 필요하므로 곡률 κ(u,v)의 Hessian을 쓴다
    - CAD/NURBS는 곡률 자체가 필요하므로 Weingarten을 쓴다

## 🎯 더 쉽게 말하면
- ✔ Weingarten
    - **곡면이 본질적으로 얼마나 휘었는지**
    - 곡면의 고유한 성질
- ✔ Hessian
    **곡률이 UV 좌표에서 어떻게 변하는지**
    → 좌표계에 따라 달라지는 성질
- 둘 다 2차 미분을 쓰지만 무엇을 미분하느냐가 완전히 다르다.


---

## 📘 곡률 구조
### 1) 곡률의 세 가지 층위
- 곡률 자체 (Weingarten / Shape Operator)
- 곡률의 변화율 (FEM에서의 κ(u,v) Hessian)
- 최적화용 Hessian (point inversion)
- 이 세 개를 분리하면 혼란이 싹 사라진다.

### 2) 곡률 텐서의 구조
- First Fundamental Form (metric)
- Second Fundamental Form (normal projection of 2nd derivatives)
- Weingarten = I⁻¹ II
- 주곡률, 평균곡률, 가우스곡률

### 3) 왜 Weingarten은 “곡면 고유의 성질”인가
- metric 제거
- 좌표계 독립
- intrinsic curvature

### 4) 왜 Hessian은 “곡면 고유 성질이 아닌가”
- Suu, Suv, Svv 전체 벡터가 들어감
- metric + embedding + tangent 성분이 섞임
- UV 좌표계에 따라 값이 달라짐

### 5) FEM에서 왜 Hessian이 필요한가
- 곡률 κ(u,v)의 변화율
- bending energy
- stiffness matrix 구성
- UV 격자 기반의 수치해석

### 6) NURBS에서의 실제 구현 흐름
- Su, Sv
- Suu, Suv, Svv
- normal
- I, II
- Weingarten
- k1, k2, H, K
- principal directions

---

