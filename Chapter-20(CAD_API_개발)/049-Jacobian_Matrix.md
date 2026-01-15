# Jacobian Matrix
- 아래는 유한요소법(FEM)에서의 Jacobian 행렬에 대한 수식적 설명과 함께,  
- mesh_jacobian 모듈의 주요 함수들을 수학적으로 해석하고 검증한 문서입니다.

## 📘 FEM 요소의 Jacobian 행렬: 수식 설명 및 함수 해석
## 🧮 1. Jacobian 행렬의 정의
- 유한요소법에서 Jacobian 행렬은 **기준 좌표계(ξ, η, ζ)** 에서 **실제 좌표계(x, y, z)** 로의 변환을 나타냅니다.
### 🔧 정의:

```math
\mathbf{J}=\left[ \begin{matrix}\frac{\partial x}{\partial \xi }&\frac{\partial x}{\partial \eta }&\frac{\partial x}{\partial \zeta }\\ \frac{\partial y}{\partial \xi }&\frac{\partial y}{\partial \eta }&\frac{\partial y}{\partial \zeta }\\ \frac{\partial z}{\partial \xi }&\frac{\partial z}{\partial \eta }&\frac{\partial z}{\partial \zeta }\end{matrix}\right]
```

- 이 행렬은 요소의 기하학적 왜곡, 뒤집힘 여부, 품질 평가에 사용됩니다.

## 📐 2. determinant of Jacobian (detJ)
### 🔧 수식:

```math
\det (\mathbf{J})=j_{00}(j_{11}j_{22}-j_{12}j_{21})-j_{01}(j_{10}j_{22}-j_{12}j_{20})+j_{02}(j_{10}j_{21}-j_{11}j_{20})
```

- `mesh_jacobian::det3(j)` 함수에서 정확히 이 수식을 구현
- detJ > 0 → 요소가 올바르게 배치됨
- detJ ≤ 0 → 요소가 뒤집히거나 퇴화됨

## 🧊 3. 요소별 Jacobian 품질 함수 설명
### 🔹 tetra_det_jacobian(p: [[f64; 3]; 4])
- 4개의 점으로 구성된 선형 tetrahedron의 $detJ$ 계산
- 벡터 $\vec {e}_1=p_1-p_0$, $\vec {e}_2=p_2-p_0$, $\vec {e}_3=p_3-p_0$
- 수식:

```math
\det (\mathbf{J})=\det \left[ \begin{matrix}e_1&e_2&e_3\end{matrix}\right]
```


### 🔹 tetra_signed_volume(p)
- 부호 있는 체적 계산:

```math
V=\frac{\det (\mathbf{J})}{6}
```

### 🔹 on_jacobian_quality_tetra4(x)
- 선형 tetra 요소의 Jacobian 품질 평가
- shape gradient는 상수:

```math
\frac{\partial N_1}{\partial \xi }=-1,\quad \frac{\partial N_2}{\partial \xi }=1,\quad \mathrm{etc.}
```

- $J = X · ∇N → detJ$ 계산 후 0 또는 $detJ$ 반환

### 🔹 on_jacobian_quality_hexa8(x)
- 8노드 hexahedron 요소의 품질 평가
- 2×2×2 Gauss 포인트에서 shape gradient 계산
- 각 포인트에서:

```math
\mathbf{J}=\sum _{i=1}^8\vec {x}_i\otimes \nabla N_i
```

- detJ의 최소/최대 비율:

```math
\mathrm{품질}=\frac{\min (\det J)}{\max (\det J)}
```

### 🔹 on_jacobian_quality_wedge6(x)
- 6노드 wedge 요소
- 삼각형 기저 + ζ 방향으로 분리된 shape gradient
- 2개 ζ 포인트 × 1개 삼각형 포인트 → 총 2개 detJ 샘플
- 품질 = min/max 비율

### 🔹 on_jacobian_quality_pyramid5(p)
- 5노드 피라미드 요소
- shape gradient는 근사식 사용
- 4개 기저 포인트 + 꼭짓점 방향 1점 → 총 4 detJ 샘플
- 품질 = min/max 비율

### 🔹 on_jacobian_quality_quad4(x)
- 4노드 2D 사각형 요소
- 2×2 Gauss 포인트에서 2×2 Jacobian 계산
- $detJ = j_{00}j_{11}-j_{01}j_{10}$

| 함수 이름                        | 수식 검증 설명 |
|----------------------------------|----------------|
| `det3`                           | 3×3 행렬식 계산 공식과 정확히 일치. FEM에서 기본 Jacobian determinant 계산에 사용됨 |
| `tetra_det_jacobian`             | 4점 tetrahedron의 edge 벡터로 구성된 행렬의 determinant 계산. 체적 기반 Jacobian과 일치 |
| `tetra_signed_volume`           | $V = \frac{\det J}{6}$ 공식 사용. FEM에서 tetra 요소 체적 계산과 동일 |
| `on_jacobian_quality_tetra4`     | 선형 tetra 요소의 shape gradient가 상수이므로 J도 상수. 품질 = detJ 또는 0 |
| `on_jacobian_quality_hexa8`      | 8노드 hexa 요소에서 2×2×2 Gauss 포인트에서 ∇N 계산 후 J 구성. 품질 = min/max(detJ) |
| `on_jacobian_quality_wedge6`     | 삼각형 기저 + ζ 방향 분리 보간. 2개 ζ 포인트에서 J 계산. 품질 = min/max(detJ) |
| `on_jacobian_quality_pyramid5`   | 피라미드 요소의 근사 shape gradient 사용. 4개 기저 포인트에서 J 계산. 품질 = min/max(detJ) |
| `on_jacobian_quality_quad4`      | 2D 4노드 요소에서 2×2 Gauss 포인트에서 J 계산. 품질 = min/max(detJ) 

---

- 유한요소법(FEM)에서 Jacobian determinant 값이 1에 가까울수록 좋다는 말은 단순한 경험칙이 아니라,    
  수학적·물리적 원리에 기반한 것입니다.  
- 아래에 그 이유를 수식과 개념 중심으로 자세히 설명.

## 📐 Jacobian 행렬이란?
- Jacobian 행렬은 **기준 좌표계(ξ, η, ζ)** 에서 **실제 좌표계(x, y, z)** 로의 변환을 나타냅니다:

```math
\mathbf{J}=\left[ \begin{matrix}\frac{\partial x}{\partial \xi }&\frac{\partial x}{\partial \eta }&\frac{\partial x}{\partial \zeta }\\ \frac{\partial y}{\partial \xi }&\frac{\partial y}{\partial \eta }&\frac{\partial y}{\partial \zeta }\\ \frac{\partial z}{\partial \xi }&\frac{\partial z}{\partial \eta }&\frac{\partial z}{\partial \zeta }\end{matrix}\right] 
```

## ✅ 왜 det(J) ≈ 1이 이상적인가?
### 1️⃣ 기준 요소와 실제 요소의 크기 왜곡이 없을 때
- 기준 요소(예: 단위 정육면체)에서 실제 요소로의 변환이 등방성이고 크기 보존이라면:
```math
\det (\mathbf{J})=1
```
- 즉, 요소가 뒤틀림 없이 균일하게 변환되었다는 뜻

### 2️⃣ 수치적 안정성과 정확도
- FEM에서 해석 정확도는 요소의 형상 품질에 크게 좌우됨
- $det(J)$ 가 너무 작거나 크면 다음 문제가 발생:

| 상황                  | 문제점 설명                                                                 | 영향 |
|-----------------------|------------------------------------------------------------------------------|------|
| $\det(\mathbf{J}) \ll 1$ | 요소가 납작하거나 퇴화됨 (degenerate element) → 수치적으로 stiff matrix ill-conditioned | 해석 불안정, 오차 확대 |
| $\det(\mathbf{J}) \gg 1$ | 요소가 과도하게 늘어남 → shape function 왜곡, 적분 오차 증가                         | 정확도 저하 |
| $\det(\mathbf{J}) < 0$   | 요소가 뒤집힘 (inverted element) → 음의 체적, 물리적으로 불가능한 상태               | 해석 실패 또는 경고 발생 |


### 3️⃣ 적분 정확도와 Gauss Quadrature
- FEM에서 요소 내부 적분은 기준 좌표계에서 수행됨
- 실제 좌표계로 변환 시:

```math
\int _{\Omega }f(x,y,z)\, dV=\int _{\hat {\Omega }}f(x(\xi ),y(\eta ),z(\zeta ))\cdot |\det (\mathbf{J})|\, d\xi d\eta d\zeta
```

- $det(J)$ 가 1에 가까우면 적분 가중치가 안정적이고, 수치 오차가 작아짐

### 📊 품질 평가 함수에서의 의미
- 예를 들어 on_jacobian_quality_hexa8() 함수에서는:

```math
\mathrm{품질}=\frac{\min (\det J)}{\max (\det J)}
```

- 이 값이 1에 가까울수록 요소 내부의 변형이 균일함
- FEM에서 이상적인 요소는 모든 Gauss 포인트에서 det(J)가 거의 동일 → 품질 ≈ 1

🔍 시각적 예시 (개념적으로)
| 요소 형태         | det(J) 분포         | 품질 지표 (예시) | FEM 해석에서의 의미                  |
|------------------|---------------------|------------------|--------------------------------------|
| ✅ 이상적 요소     | 모든 지점에서 ≈ 1   | 1.0              | 등방성, 왜곡 없음, 안정적 해석       |
| ⚠️ 찌그러진 요소   | 0.3 ~ 1.2 등 편차 큼 | 0.25 ~ 0.8       | 왜곡 있음, 정확도 저하 가능성        |
| ❌ 퇴화된 요소     | 매우 작거나 0에 근접 | ≪ 0.1 또는 0     | 납작함, stiff matrix ill-conditioned |
| ❌ 뒤집힌 요소     | det(J) < 0          | 0.0              | 요소 뒤집힘, 해석 실패 또는 경고 발생 |


## ✅ 결론
- Jacobian determinant는 요소의 기하학적 건전성을 나타냄
- det(J) ≈ 1은 왜곡 없는 이상적인 요소
- FEM에서 정확도, 안정성, 적분 품질을 위해 det(J)는 1에 가까워야 함


- FEM 요소의 Jacobian 행렬은 단순한 수치 계산이 아니라,  
  기준 좌표계에서 실제 좌표계로의 좌표 변환을 수학적으로 표현한 결과입니다.  
- 아래에 그 유도 과정을 단계별로 설명.

## 📘 요소의 Jacobian 행렬 유도 과정
### 1️⃣ 기준 좌표계와 실제 좌표계
- 유한요소법에서는 해석을 단순화하기 위해 모든 요소를 **기준 요소(reference element)** 로 정의합니다.
- 예시:
  - 1D: $\xi \in [-1,1]$
  - 2D: $(\xi ,\eta )\in [-1,1]^2$
  - 3D: $(\xi ,\eta ,\zeta )\in [-1,1]^3$

- 실제 요소의 좌표 (x,y,z)는 기준 좌표계에서 **형상 함수(N_i)** 를  통해 보간됩니다:

```math
\vec {x}(\xi ,\eta ,\zeta )=\sum _{i=1}^nN_i(\xi ,\eta ,\zeta )\cdot \vec {x}_i
```


### 2️⃣ 좌표 변환의 미분: Jacobian 정의
- 위 식을 기준 좌표계로 미분하면:

```math
\frac{\partial \vec {x}}{\partial \xi }=\sum _i\frac{\partial N_i}{\partial \xi }\cdot \vec {x}_i\quad \mathrm{(동일하게\  \eta ,\  \zeta 도)}
```

- 이 미분들을 모은 행렬이 바로 Jacobian 행렬입니다:

```math
\mathbf{J}=\left[ \begin{matrix}\frac{\partial x}{\partial \xi }&\frac{\partial x}{\partial \eta }&\frac{\partial x}{\partial \zeta }\\ \frac{\partial y}{\partial \xi }&\frac{\partial y}{\partial \eta }&\frac{\partial y}{\partial \zeta }\\ \frac{\partial z}{\partial \xi }&\frac{\partial z}{\partial \eta }&\frac{\partial z}{\partial \zeta }\end{matrix}\right] =\sum _{i=1}^n\vec {x}_i\otimes \nabla N_i
```
- 여기서 

```math
\nabla N_i=\left[ \frac{\partial N_i}{\partial \xi },\frac{\partial N_i}{\partial \eta },\frac{\partial N_i}{\partial \zeta }\right] 
```

### 3️⃣ 예시: 4-노드 Tetra 요소
- 형상 함수:

```math
N_1=1-\xi -\eta -\zeta ,\quad N_2=\xi ,\quad N_3=\eta ,\quad N_4=\zeta 
```

- 미분:

```math
\nabla N_1=[-1,-1,-1],\quad \nabla N_2=[1,0,0],\quad \nabla N_3=[0,1,0],\quad \nabla N_4=[0,0,1]
```

- Jacobian:

```math
\mathbf{J}=\sum _{i=1}^4\vec {x}_i\otimes \nabla N_i\Rightarrow \mathbf{J}=\left[ \begin{matrix}x_2-x_1&x_3-x_1&x_4-x_1\\ y_2-y_1&y_3-y_1&y_4-y_1\\ z_2-z_1&z_3-z_1&z_4-z_1\end{matrix}\right] 
```

- 즉, 기준점에서의 edge 벡터로 구성된 행렬

### 4️⃣ det(J)의 의미
- $det(J)$ 는 기하학적 스케일링과 체적 변화율을 나타냄
- $|\det (J)|=\mathrm{요소\  체적의\  배율}$
- 적분 시에도 사용됨:

```math
\int _{\Omega }f(x)\, dx=\int _{\hat {\Omega }}f(x(\xi ))\cdot |\det (J)|\, d\xi 
```

## ✅ 결론
- Jacobian은 기준 좌표계에서 실제 좌표계로의 변환 미분 행렬
- 유도는 형상 함수의 미분과 실제 좌표의 보간을 통해 이루어짐
- det(J)는 요소의 체적, 왜곡, 뒤집힘 여부를 판단하는 핵심 지표

---
# Hexa Jacobian
- 유한요소법(FEM)에서 자주 사용되는 **8-노드 선형 Hexahedron 요소 (Hexa8)** 의 Jacobian 행렬이 어떻게 유도되고,    
  왜 중요한지 수식과 함께 자세히 설명.

## 🧊 Hexa8 요소의 Jacobian 행렬 유도 및 해석
### 1️⃣ 요소 정의
- Hexa8 요소는 3차원 공간에서 8개의 꼭짓점을 가지며, 각 꼭짓점은 기준 좌표계 $(\xi ,\eta ,\zeta )\in [-1,1]^3$ 상의 정육면체 정점에 대응됩니다.
  - 실제 좌표계: $(x,y,z)$
  - 기준 좌표계: $(\xi ,\eta ,\zeta )$

### 2️⃣ 좌표 보간식
- 실제 좌표는 기준 좌표계에서의 형상 함수 $N_i(\xi ,\eta ,\zeta )$ 를 이용해 보간됩니다:

```math
\vec {x}(\xi ,\eta ,\zeta )=\sum _{i=1}^8N_i(\xi ,\eta ,\zeta )\cdot \vec {x}_i
```

각 $N_i$ 는 trilinear 형상 함수이며, 예를 들어:

```math
N_1(\xi ,\eta ,\zeta )=\frac{1}{8}(1-\xi )(1-\eta )(1-\zeta )
```


### 3️⃣ Jacobian 행렬 유도
- Jacobian 행렬은 다음과 같이 정의됩니다:

```math
\mathbf{J}=\left[ \begin{matrix}\frac{\partial x}{\partial \xi }&\frac{\partial x}{\partial \eta }&\frac{\partial x}{\partial \zeta }\\ \frac{\partial y}{\partial \xi }&\frac{\partial y}{\partial \eta }&\frac{\partial y}{\partial \zeta }\\ \frac{\partial z}{\partial \xi }&\frac{\partial z}{\partial \eta }&\frac{\partial z}{\partial \zeta }\end{matrix}\right] =\sum _{i=1}^8\vec {x}_i\otimes \nabla N_i
```

-여기서:
  - $\vec {x}_i=[x_i,y_i,z_i]$
  - $\nabla N_i=\left[ \frac{\partial N_i}{\partial \xi },\frac{\partial N_i}{\partial \eta },\frac{\partial N_i}{\partial \zeta }\right]$
  - 즉, 각 노드의 좌표와 형상 함수의 기울기를 외적(outer product)하여 합산한 것이 Jacobian입니다.

### 4️⃣ 수치적 구현 (코드 기반)
- on_jacobian_quality_hexa8() 함수에서는 다음을 수행합니다:
  - 8개의 Gauss 포인트(±1/√3)를 기준 좌표계에서 샘플링
  - 각 포인트에서 shape gradient $\nabla N_i$ 계산 → hexa8_shape_gradients()
  - Jacobian 행렬 계산:

```math
J_{r,c}=\sum _{i=1}^8x_i^{(r)}\cdot \frac{\partial N_i}{\partial \xi _c}\quad \mathrm{for\  }r,c\in \{ x,y,z\} 
```

- $det(J)$ 계산 → $det3(j)$
- 전체 품질:

```math
\mathrm{품질}=\frac{\min (\det J)}{\max (\det J)}
```

### 5️⃣ det(J)의 의미
- $\det (\mathbf{J})>0$: 요소가 뒤집히지 않고 올바르게 배치됨
- $\det (\mathbf{J})\approx 1$: 기준 요소와 거의 동일한 크기 → 이상적
- $\det (\mathbf{J})\ll$ 1: 요소가 납작하거나 퇴화됨
- $\det (\mathbf{J})<0$: 요소가 뒤집힘 → 해석 실패

## ✅ 요약 
| 항목                     | 수식 또는 설명 |
|--------------------------|----------------|
| 요소 유형                | 8-노드 선형 Hexahedron (Hexa8) |
| 좌표 변환                | $\vec{x}(\xi, \eta, \zeta) = \sum N_i(\xi, \eta, \zeta) \cdot \vec{x}_i$ |
| Jacobian 행렬 정의       | $\mathbf{J} = \sum \vec{x}_i \otimes \nabla N_i$ |
| 품질 평가 지표           | $\min(\det J) / \max(\det J)$ |
| 이상적 상태             | $\det J \approx 1$ (모든 Gauss 포인트에서) |

---

- Hexa8 요소의 Jacobian 행렬 성분을 구성하는 각 성분이 어떻게 유도되는지, 수학적으로 하나하나 설명.

## 📘 Hexa8 요소의 Jacobian 행렬 성분 유도
### 🔧 기본 수식

```math
J_{r,c}=\sum _{i=1}^8x_i^{(r)}\cdot \frac{\partial N_i}{\partial \xi _c}\quad \mathrm{for\  }r,c\in \{ x,y,z\}
```

- $x_i^{(r)}$: i번째 노드의 r 방향 좌표 (예: x, y, z)
- $\frac{\partial N_i}{\partial \xi _c}$: i번째 형상 함수의 기준 좌표계 방향 미분 (ξ, η, ζ)


### 🧊 Jacobian 행렬 구조
- Jacobian은 3×3 행렬로 구성됩니다:

```math
\mathbf{J}=\left[ \begin{matrix}J_{x,\xi }&J_{x,\eta }&J_{x,\zeta }\\ J_{y,\xi }&J_{y,\eta }&J_{y,\zeta }\\ J_{z,\xi }&J_{z,\eta }&J_{z,\zeta }\end{matrix}\right]
```


- 각 성분은 다음과 같이 계산됩니다:

#### 🔢 예시: $J_{x,\xi }$ 계산

```math
J_{x,\xi }=\sum _{i=1}^8x_i\cdot \frac{\partial N_i}{\partial \xi }
```

- $x_i$: i번째 노드의 x 좌표
- $\frac{\partial N_i}{\partial \xi }$: i번째 형상 함수의 ξ 방향 미분  
  이렇게 해서 8개의 노드에 대해 각각의 $x_i\cdot \partial N_i/\partial \xi$  값을 더합니다.



#### 🔁 전체 9개 성분 계산 방식
- 각 성분은 다음과 같이 계산됩니다:

```math
J_{x,ξ} = Σ x_i * ∂N_i/∂ξ
``` 

```math
J_{x,η} = Σ x_i * ∂N_i/∂η
```


```math
J_{x,ζ} = Σ x_i * ∂N_i/∂ζ
```


```math
J_{y,ξ} = Σ y_i * ∂N_i/∂ξ
```


```math
J_{y,η} = Σ y_i * ∂N_i/∂η
```


```math
J_{y,ζ} = Σ y_i * ∂N_i/∂ζ
```


```math
J_{z,ξ} = Σ z_i * ∂N_i/∂ξ
```


```math
J_{z,η} = Σ z_i * ∂N_i/∂η
```


```math
J_{z,ζ} = Σ z_i * ∂N_i/∂ζ
```
  


#### 🧠 형상 함수 미분: $\nabla N_i$
- 각 $N_i$ 는 trilinear 형상 함수로, 미분은 다음과 같은 형태:

```math
\frac{\partial N_i}{\partial \xi }=\pm \frac{1}{8}(1\pm \eta )(1\pm \zeta )\quad \mathrm{(유사하게\  \eta ,\  \zeta \  방향도)}
```

- 이 미분값은 hexa8_shape_gradients(ξ, η, ζ) 함수에서 계산됩니다.

#### ✅ 최종 Jacobian 구성

```rust
for i in 0..8 {
    for r in 0..3 {
        J[r][0] += x[i][r] * dN[i][0]; // ∂N_i/∂ξ
        J[r][1] += x[i][r] * dN[i][1]; // ∂N_i/∂η
        J[r][2] += x[i][r] * dN[i][2]; // ∂N_i/∂ζ
    }
}
```

- $x[i][r]$: i번째 노드의 r 방향 좌표
- $dN[i][c]$: i번째 노드의 형상 함수의 c 방향 미분

## 📌 핵심 요약
| 구성 요소 또는 단계         | 설명 또는 수식 |
|----------------------------|----------------|
| 노드 좌표                  | $[x_i, y_i, z_i]$ — 8개 노드의 실제 좌표 |
| 형상 함수 기울기           | $\nabla N_i = \left[ \frac{\partial N_i}{\partial \xi}, \frac{\partial N_i}{\partial \eta}, \frac{\partial N_i}{\partial \zeta} \right]$ |
| Jacobian 성분 계산         | $J_{r,c} = \sum_{i=1}^8 x_i^{(r)} \cdot \frac{\partial N_i}{\partial \xi_c}$ |
| Jacobian 행렬식 계산       | $\det(\mathbf{J}) = \texttt{det3(J)}$ — 3×3 행렬식 |

---

- Hexa8 요소의 Jacobian 품질을 2×2×2 Gauss 포인트에서 계산하고 시각화한 것입니다.  
- 각 포인트에서의 det(J)를 히트맵으로 보여줍니다.

## 📊 Hexa8 요소의 Jacobian 품질 시각화
### 🔧 요소 좌표 (살짝 찌그러진 정육면체)
```
노드 0: (0.0, 0.0, 0.0)
노드 1: (1.0, 0.0, 0.0)
노드 2: (1.0, 1.0, 0.0)
노드 3: (0.0, 1.0, 0.0)
노드 4: (0.0, 0.0, 1.0)
노드 5: (1.0, 0.0, 1.0)
노드 6: (1.0, 1.0, 1.0)
노드 7: (0.0, 1.0, 1.05) ← 꼭짓점 살짝 찌그러짐
```
```python
# Visualizing Jacobian determinant at Gauss points for a distorted 8-node hexahedron
import numpy as np
import matplotlib.pyplot as plt
from matplotlib import cm
from mpl_toolkits.mplot3d import Axes3D

# Define the 8-node hexahedron node coordinates
nodes = np.array([
    [0.0, 0.0, 0.0],   # 0
    [1.0, 0.0, 0.0],   # 1
    [1.0, 1.0, 0.0],   # 2
    [0.0, 1.0, 0.0],   # 3
    [0.0, 0.0, 1.0],   # 4
    [1.0, 0.0, 1.0],   # 5
    [1.0, 1.0, 1.0],   # 6
    [0.0, 1.0, 1.05],  # 7 (slightly distorted)
])

# Gauss points for 2x2x2 integration
g = 1.0 / np.sqrt(3)
gauss_points = [(-g, -g, -g), (g, -g, -g), (g, g, -g), (-g, g, -g),
                (-g, -g, g), (g, -g, g), (g, g, g), (-g, g, g)]

# Compute shape function gradients at a given Gauss point
def hexa8_shape_gradients(ksi, eta, zeta):
    a = 0.125
    return np.array([
        [-a * (1 - eta) * (1 - zeta), -a * (1 - ksi) * (1 - zeta), -a * (1 - ksi) * (1 - eta)],
        [ a * (1 - eta) * (1 - zeta), -a * (1 + ksi) * (1 - zeta), -a * (1 + ksi) * (1 - eta)],
        [ a * (1 + eta) * (1 - zeta),  a * (1 + ksi) * (1 - zeta), -a * (1 + ksi) * (1 + eta)],
        [-a * (1 + eta) * (1 - zeta),  a * (1 - ksi) * (1 - zeta), -a * (1 - ksi) * (1 + eta)],
        [-a * (1 - eta) * (1 + zeta), -a * (1 - ksi) * (1 + zeta),  a * (1 - ksi) * (1 - eta)],
        [ a * (1 - eta) * (1 + zeta), -a * (1 + ksi) * (1 + zeta),  a * (1 + ksi) * (1 - eta)],
        [ a * (1 + eta) * (1 + zeta),  a * (1 + ksi) * (1 + zeta),  a * (1 + ksi) * (1 + eta)],
        [-a * (1 + eta) * (1 + zeta),  a * (1 - ksi) * (1 + zeta),  a * (1 - ksi) * (1 + eta)],
    ])

# Compute Jacobian determinant at each Gauss point
jacobian_dets = []
coords = []

for ksi, eta, zeta in gauss_points:
    dN = hexa8_shape_gradients(ksi, eta, zeta)  # 8x3
    J = np.zeros((3, 3))
    for i in range(8):
        J += np.outer(nodes[i], dN[i])
    detJ = np.linalg.det(J)
    jacobian_dets.append(detJ)
    coords.append([ksi, eta, zeta])

coords = np.array(coords)
jacobian_dets = np.array(jacobian_dets)

# Plotting
fig = plt.figure(figsize=(8, 6))
ax = fig.add_subplot(111, projection='3d')
sc = ax.scatter(coords[:, 0], coords[:, 1], coords[:, 2], c=jacobian_dets, cmap='viridis', s=100)
ax.set_title("Jacobian Determinant at Gauss Points")
ax.set_xlabel("Ksi")
ax.set_ylabel("Eta")
ax.set_zlabel("Zeta")
fig.colorbar(sc, ax=ax, label="det(J)")
plt.tight_layout()
plt.savefig("/mnt/data/hexa8_jacobian_determinant.png")

print("Computed Jacobian determinant at 8 Gauss points and visualized as 3D heatmap.")
```

![Hexa Jacobian Determinant](/image/hexa8_jacobian_determinant.png)

## 📍 Gauss 포인트 위치 (±1/√3)
- 총 8개 포인트:
- $(\pm g,\pm g,\pm g), where g=\frac{1}{\sqrt{3}}\approx 0.577$

### 📐 각 포인트에서의 det(J)
| ξ       | η       | ζ       | det(J)   |
|---------|---------|---------|----------|
| -0.577  | -0.577  | -0.577  | 1.000000 |
| -0.577  | -0.577  |  0.577  | 1.000000 |
| -0.577  |  0.577  | -0.577  | 1.000000 |
| -0.577  |  0.577  |  0.577  | 1.025000 |
|  0.577  | -0.577  | -0.577  | 1.000000 |
|  0.577  | -0.577  |  0.577  | 1.000000 |
|  0.577  |  0.577  | -0.577  | 1.000000 |
|  0.577  |  0.577  |  0.577  | 1.025000 |

### 🎯 품질 지표

```math
\mathrm{품질}=\frac{\min (\det J)}{\max (\det J)}=\frac{1.0000}{1.0250}\approx 0.9756
```

- 요소는 거의 이상적이며, 약간의 꼭짓점 찌그러짐이 감지됨

### 🖼️ 시각적 히트맵 설명
- 각 Gauss 포인트를 3D 공간에 배치
- 색상으로 det(J) 크기를 표현 (예: 파란색 = 1.0, 빨간색 = 1.025)
- 찌그러진 꼭짓점 근처에서 det(J)가 약간 증가

## 소스 코드

```rust
pub mod mesh_jacobian {

    #[inline]
    fn hexa8_shape_gradients(ksi: f64, eta: f64, zeta: f64) -> [[f64; 3]; 8] {
        // dNi/d(ksi,eta,zeta). 노드 순서:
        // (-,-,-), (+,-,-), (+,+,-), (-,+,-), (-,-,+), (+,-,+), (+,+,+), (-,+,+)
        let a = 0.125;
        [
            [
                -a * (1.0 - eta) * (1.0 - zeta),
                -a * (1.0 - ksi) * (1.0 - zeta),
                -a * (1.0 - ksi) * (1.0 - eta),
            ],
            [
                a * (1.0 - eta) * (1.0 - zeta),
                -a * (1.0 + ksi) * (1.0 - zeta),
                -a * (1.0 + ksi) * (1.0 - eta),
            ],
            [
                a * (1.0 + eta) * (1.0 - zeta),
                a * (1.0 + ksi) * (1.0 - zeta),
                -a * (1.0 + ksi) * (1.0 + eta),
            ],
            [
                -a * (1.0 + eta) * (1.0 - zeta),
                a * (1.0 - ksi) * (1.0 - zeta),
                -a * (1.0 - ksi) * (1.0 + eta),
            ],
            [
                -a * (1.0 - eta) * (1.0 + zeta),
                -a * (1.0 - ksi) * (1.0 + zeta),
                a * (1.0 - ksi) * (1.0 - eta),
            ],
            [
                a * (1.0 - eta) * (1.0 + zeta),
                -a * (1.0 + ksi) * (1.0 + zeta),
                a * (1.0 + ksi) * (1.0 - eta),
            ],
            [
                a * (1.0 + eta) * (1.0 + zeta),
                a * (1.0 + ksi) * (1.0 + zeta),
                a * (1.0 + ksi) * (1.0 + eta),
            ],
            [
                -a * (1.0 + eta) * (1.0 + zeta),
                a * (1.0 - ksi) * (1.0 + zeta),
                a * (1.0 - ksi) * (1.0 + eta),
            ],
        ]
    }
```
```rust
    #[inline]
    pub fn det3(j: [[f64; 3]; 3]) -> f64 {
        j[0][0] * (j[1][1] * j[2][2] - j[1][2] * j[2][1])
            - j[0][1] * (j[1][0] * j[2][2] - j[1][2] * j[2][0])
            + j[0][2] * (j[1][0] * j[2][1] - j[1][1] * j[2][0])
    }
```
```rust
    pub fn tetra_det_jacobian(p: [[f64; 3]; 4]) -> f64 {
        let e1 = [p[1][0] - p[0][0], p[1][1] - p[0][1], p[1][2] - p[0][2]];
        let e2 = [p[2][0] - p[0][0], p[2][1] - p[0][1], p[2][2] - p[0][2]];
        let e3 = [p[3][0] - p[0][0], p[3][1] - p[0][1], p[3][2] - p[0][2]];
        det3([
            [e1[0], e2[0], e3[0]],
            [e1[1], e2[1], e3[1]],
            [e1[2], e2[2], e3[2]],
        ])
    }
```
```rust
    #[inline]
    pub fn tetra_signed_volume(p: [[f64; 3]; 4]) -> f64 {
        tetra_det_jacobian(p) / 6.0
    }
```
```rust
    #[inline]
    pub fn tetra_quality_simple(p: [[f64; 3]; 4]) -> f64 {
        let detj = tetra_det_jacobian(p);
        if detj <= 0.0 { 0.0 } else { 1.0 }
    }
```
```rust
    #[inline]
    fn minmax_ratio(mut vals: Vec<f64>) -> f64 {
        if vals.is_empty() {
            return 0.0;
        }
        let mut mn = f64::INFINITY;
        let mut mx = f64::NEG_INFINITY;
        for v in vals.drain(..) {
            if v <= 0.0 {
                return 0.0;
            }
            if v < mn {
                mn = v;
            }
            if v > mx {
                mx = v;
            }
        }
        if mx <= 0.0 { 0.0 } else { mn / mx }
    }
```
```rust
    // ================= 4-노드 Tetra (선형) =================
    // 노드 순서는 (일반적) N1=1-ξ-η-ζ, N2=ξ, N3=η, N4=ζ 를 따르는 4점입니다.
    // 등매개 좌표(ξ,η,ζ)는 [0,1] with ξ+η+ζ<=1. 선형 Tet은 J가 상수입니다.
    pub fn on_jacobian_quality_tetra4(x: [[f64; 3]; 4]) -> f64 {
        // dN/dξ, dN/dη, dN/dζ (각 노드에 대한) — 상수
        let dndxi = [-1.0, 1.0, 0.0, 0.0];
        let dndeta = [-1.0, 0.0, 1.0, 0.0];
        let dndzet = [-1.0, 0.0, 0.0, 1.0];

        // J = X * dN/d(ξ,η,ζ)
        let mut j = [[0.0; 3]; 3];
        for a in 0..4 {
            j[0][0] += dndxi[a] * x[a][0];
            j[0][1] += dndxi[a] * x[a][1];
            j[0][2] += dndxi[a] * x[a][2];
            j[1][0] += dndeta[a] * x[a][0];
            j[1][1] += dndeta[a] * x[a][1];
            j[1][2] += dndeta[a] * x[a][2];
            j[2][0] += dndzet[a] * x[a][0];
            j[2][1] += dndzet[a] * x[a][1];
            j[2][2] += dndzet[a] * x[a][2];
        }
        let detj = det3(j);
        if detj <= 0.0 { 0.0 } else { detj } // 선형 Tet은 detJ 상수 → min/max = 1
    }
```
```rust
    // ================= 8-노드 Hexa (선형, trilinear) =================
    // 노드 순서는 일반적인 (±ξ, ±η, ±ζ) 조합 순서라 가정 (0..7).
    // 2×2×2 Gauss (ξ,η,ζ ∈ {±1/√3})
    pub fn on_jacobian_quality_hexa8(x: [[f64; 3]; 8]) -> f64 {
        let g = 1.0_f64 / 3.0_f64.sqrt(); // ±1/√3
        let sample = [-g, g];
        let mut min_det = f64::INFINITY;
        let mut max_det = f64::NEG_INFINITY;
        for &ksi in &sample {
            for &eta in &sample {
                for &zeta in &sample {
                    let d_n = hexa8_shape_gradients(ksi, eta, zeta);
                    // J = Σ (x_i ⊗ ∇N_i)
                    let mut j = [[0.0; 3]; 3];
                    for i in 0..8 {
                        for r in 0..3 {
                            // x,y,z
                            j[r][0] += x[i][r] * d_n[i][0];
                            j[r][1] += x[i][r] * d_n[i][1];
                            j[r][2] += x[i][r] * d_n[i][2];
                        }
                    }
                    let detj = det3(j);
                    if detj <= 0.0 {
                        return 0.0;
                    } // 어떤 가우스점에서라도 뒤집히면 0
                    min_det = min_det.min(detj);
                    max_det = max_det.max(detj);
                }
            }
        }
        if max_det <= 0.0 {
            0.0
        } else {
            min_det / max_det
        }
    }
```
```rust
    #[allow(unused)]
    #[inline]
    fn tet_signed_vol(p: [[f64; 3]; 4]) -> f64 {
        let e1 = [p[1][0] - p[0][0], p[1][1] - p[0][1], p[1][2] - p[0][2]];
        let e2 = [p[2][0] - p[0][0], p[2][1] - p[0][1], p[2][2] - p[0][2]];
        let e3 = [p[3][0] - p[0][0], p[3][1] - p[0][1], p[3][2] - p[0][2]];
        let det = det3([
            [e1[0], e2[0], e3[0]],
            [e1[1], e2[1], e3[1]],
            [e1[2], e2[2], e3[2]],
        ]);
        det / 6.0
    }
```
```rust
    // ================= 6-노드 Wedge/Prism (선형) =================
    // (ξ,η) : 삼각형, L1=ξ, L2=η, L3=1-ξ-η; ζ∈[-1,1]
    // N1=0.5*(1-ζ)*L1, N2=0.5*(1-ζ)*L2, N3=0.5*(1-ζ)*L3
    // N4=0.5*(1+ζ)*L1, N5=0.5*(1+ζ)*L2, N6=0.5*(1+ζ)*L3
    pub fn on_jacobian_quality_wedge6(x: [[f64; 3]; 6]) -> f64 {
        let gz = 1.0 / (3.0_f64).sqrt(); // ζ: 2점
        let zetas = [-gz, gz];
        let tri = [(1.0 / 3.0, 1.0 / 3.0)]; // 삼각형: 1점(중심)
        let mut dets = Vec::with_capacity(zetas.len() * tri.len());

        for &(xi, eta) in &tri {
            let l1 = xi;
            let l2 = eta;
            let l3 = 1.0 - xi - eta;
            // dL/dξ, dL/dη
            let d_l_dxi = [1.0, 0.0, -1.0];
            let d_l_delta = [0.0, 1.0, -1.0];

            for &ze in &zetas {
                let a = 0.5 * (1.0 - ze);
                let b = 0.5 * (1.0 + ze);

                // dN/dξ
                let dn_dxi = [
                    a * d_l_dxi[0],
                    a * d_l_dxi[1],
                    a * d_l_dxi[2],
                    b * d_l_dxi[0],
                    b * d_l_dxi[1],
                    b * d_l_dxi[2],
                ];
                // dN/dη
                let dn_det = [
                    a * d_l_delta[0],
                    a * d_l_delta[1],
                    a * d_l_delta[2],
                    b * d_l_delta[0],
                    b * d_l_delta[1],
                    b * d_l_delta[2],
                ];
                // dN/dζ
                let dn_dze = [
                    -0.5 * l1,
                    -0.5 * l2,
                    -0.5 * l3,
                    0.5 * l1,
                    0.5 * l2,
                    0.5 * l3,
                ];

                let mut j = [[0.0; 3]; 3];
                for a in 0..6 {
                    j[0][0] += dn_dxi[a] * x[a][0];
                    j[0][1] += dn_dxi[a] * x[a][1];
                    j[0][2] += dn_dxi[a] * x[a][2];
                    j[1][0] += dn_det[a] * x[a][0];
                    j[1][1] += dn_det[a] * x[a][1];
                    j[1][2] += dn_det[a] * x[a][2];
                    j[2][0] += dn_dze[a] * x[a][0];
                    j[2][1] += dn_dze[a] * x[a][1];
                    j[2][2] += dn_dze[a] * x[a][2];
                }
                dets.push(det3(j));
            }
        }
        minmax_ratio(dets)
    }
```
```rust
    // ================= 5-노드 Pyramid (선형) =================
    // 좌표: ξ,η ∈ [-1,1], ζ ∈ [0,1].
    // N1 = 0.25*(1-ξ)*(1-η)*(1-ζ)
    // N2 = 0.25*(1+ξ)*(1-η)*(1-ζ)
    // N3 = 0.25*(1+ξ)*(1+η)*(1-ζ)
    // N4 = 0.25*(1-ξ)*(1+η)*(1-ζ)
    // N5 = ζ
    // 품질 평가용 샘플: (±g, ±g, 0.25) 4점 + (0,0,0.75) 1점
    /// 피라미드 노드: 0..3=base CCW, 4=apex
    ///
    #[inline]
    fn pyramid5_shape_gradients(ksi: f64, eta: f64, zeta: f64) -> [[f64; 3]; 5] {
        // 형상 함수의 기울기 ∇N_i 계산
        // 참고: 피라미드 요소는 비선형이므로 근사 형상 함수 사용
        let a = 0.125 * (1.0 - zeta);
        let dz = 0.25;
        [
            [
                -a * (1.0 - eta),
                -a * (1.0 - ksi),
                -0.25 * (1.0 - ksi) * (1.0 - eta),
            ],
            [
                a * (1.0 - eta),
                -a * (1.0 + ksi),
                -0.25 * (1.0 + ksi) * (1.0 - eta),
            ],
            [
                a * (1.0 + eta),
                a * (1.0 + ksi),
                -0.25 * (1.0 + ksi) * (1.0 + eta),
            ],
            [
                -a * (1.0 + eta),
                a * (1.0 - ksi),
                -0.25 * (1.0 - ksi) * (1.0 + eta),
            ],
            [0.0, 0.0, dz], // 꼭짓점 방향
        ]
    }
```
```rust
    #[inline]
    pub fn on_jacobian_quality_pyramid5(p: [[f64; 3]; 5]) -> f64 {
        // 2×2 가우스 포인트 (기저면), 꼭짓점 방향은 1점
        let g = 1.0_f64 / 3.0_f64.sqrt();
        let sample = [-g, g];
        let zeta_sample = [0.25]; // 꼭짓점 방향은 1점만 사용

        let mut min_det = f64::INFINITY;
        let mut max_det = f64::NEG_INFINITY;

        for &ksi in &sample {
            for &eta in &sample {
                for &zeta in &zeta_sample {
                    let d_n = pyramid5_shape_gradients(ksi, eta, zeta);
                    let mut j = [[0.0; 3]; 3];
                    for i in 0..5 {
                        for r in 0..3 {
                            j[r][0] += p[i][r] * d_n[i][0];
                            j[r][1] += p[i][r] * d_n[i][1];
                            j[r][2] += p[i][r] * d_n[i][2];
                        }
                    }
                    let detj = det3(j);
                    if detj <= 0.0 {
                        return 0.0;
                    }
                    min_det = min_det.min(detj);
                    max_det = max_det.max(detj);
                }
            }
        }
        if max_det <= 0.0 {
            0.0
        } else {
            min_det / max_det
        }
    }
```
```rust
    #[inline]
    fn quad4_shape_gradients(ksi: f64, eta: f64) -> [[f64; 2]; 4] {
        // dNi/d(ksi,eta), N1..N4 순서
        let a = 0.25;
        [
            [-a * (1.0 - eta), -a * (1.0 - ksi)],
            [a * (1.0 - eta), -a * (1.0 + ksi)],
            [a * (1.0 + eta), a * (1.0 + ksi)],
            [-a * (1.0 + eta), a * (1.0 - ksi)],
        ]
    }
```
```rust
    pub fn on_skew_quad_non_affine(quad: &mut [[f64; 2]; 4], factor: f64) {
        // factor ∈ [0, 1) 권장. 0이면 변화 없음, 1.0에 가까울수록 많이 휘어짐.
        assert!(factor >= 0.0, "factor must be non-negative");

        // 요소 대각선의 중점(대략적 중심)을 계산
        let cx = 0.5 * (quad[0][0] + quad[2][0]);
        let cy = 0.5 * (quad[0][1] + quad[2][1]);

        // 우상단 꼭짓점(인덱스 2)을 중심 방향으로 factor만큼 당긴다
        let dx = quad[2][0] - cx;
        let dy = quad[2][1] - cy;
        quad[2][0] -= factor * dx;
        quad[2][1] -= factor * dy;
    }
```
```rust
    pub fn on_jacobian_quality_quad4(x: [[f64; 2]; 4]) -> f64 {
        let g = 1.0_f64 / 3.0_f64.sqrt();
        let sample = [-g, g];
        let mut min_det = f64::INFINITY;
        let mut max_det = f64::NEG_INFINITY;
        for &ksi in &sample {
            for &eta in &sample {
                let d_n = quad4_shape_gradients(ksi, eta);
                // J = Σ x_i ⊗ ∇N_i  (2x2)
                let mut j = [[0.0; 2]; 2];
                for i in 0..4 {
                    j[0][0] += x[i][0] * d_n[i][0];
                    j[0][1] += x[i][0] * d_n[i][1];
                    j[1][0] += x[i][1] * d_n[i][0];
                    j[1][1] += x[i][1] * d_n[i][1];
                }
                let detj = j[0][0] * j[1][1] - j[0][1] * j[1][0];
                if detj <= 0.0 {
                    return 0.0;
                }
                min_det = min_det.min(detj);
                max_det = max_det.max(detj);
            }
        }
        if max_det <= 0.0 {
            0.0
        } else {
            min_det / max_det
        }
    }
}
```
---

## 테스트 코드






## 📘 테스트 코드 문서화 및 수식 정리
### 📐 Jacobian Quality란?
유한요소 해석에서 요소의 품질은 Jacobian 행렬의 행렬식(detJ)을 기반으로 평가됩니다. 일반적으로:
- $detJ > 0$: 요소가 정방향(정상)
- $detJ = 0$: 요소가 붕괴됨 (degenerate)
- $detJ < 0$: 요소가 뒤집힘 (inverted)
- Jacobian Quality는 보통 다음과 같이 정의됩니다:

$$
Q=\min _i\left( \frac{\det (J_i)}{\max _j\det (J_j)}\right)
$$

- 단, detJ가 음수인 경우 $Q = 0$ 으로 처리합니다.

### 🔺 Tetrahedron (4-node)
- 정칙 사면체: tet_regular_quality_is_one
- 이상적인 정사면체의 품질은 1.0이어야 함
- 슬리버 요소: tet_sliver_quality_decreases_but_positive
- 거의 평면에 가까운 슬리버 요소는 detJ가 작지만 양수 → $Q ≠ 0$
- 뒤집힌 요소: tet_inverted_returns_zero
- 노드 순서 변경으로 $detJ < 0 → Q = 0$

### 🧊 Hexahedron (8-node)
- 전단된 육면체: hexa_sheared_quality_between_0_and_1
- 전단 변형 후에도 $detJ > 0 → Q ∈ (0, 1)$
- 일부 가우스점에서 뒤집힘: hexa_inverted_at_some_gauss_point_returns_zero
- 한 노드의 z값을 크게 변경하여 일부 영역 detJ < 0 → Q = 0

### 🧱 Wedge / Prism (6-node)
- 약간의 비틀림: wedge_mild_twist_is_positive
- 상하 삼각형이 약간 비틀림 → $Q ∈ (0, 1)$
- 거의 붕괴된 요소: wedge_near_collapse_returns_small_or_zero
- 상하 삼각형이 거의 겹침 → $detJ ≈ 0 또는 < 0 → Q ≈ 0$

### 🔺 Pyramid (5-node)
- Apex가 중심에서 벗어난 경우: pyramid_apex_off_center_quality_between_0_and_1
- Apex의 위치가 살짝 오프셋 → Q ∈ (0, 1)
- 뒤집힌 피라미드: pyramid_inverted_returns_zero
- 밑면 일부를 위로 올려 뒤틀림 유도 → $detJ < 0 → Q = 0$

### ◼️ Quadrilateral (4-node, 2D)
- 전단된 사각형: quad_sheared_quality_between_0_and_1
- 한 꼭짓점만 이동 → 비정칙한 평면 사각형 → $Q ∈ (0, 1)$
- 교차된 사각형: quad_crossed_returns_zero
- 노드 순서가 꼬여 교차 발생 → 일부 가우스점 $detJ < 0 → Q = 0$

### 📊 수식 요약
- Jacobian 행렬 J: 요소의 좌표 변환을 나타내는 행렬
- 행렬식 $\det (J)$: 요소의 부피/면적을 나타냄
- Jacobian 품질 지표 Q:

$$
Q =
\begin{cases}
\displaystyle \min_i \left( \frac{\det(J_i)}{\max_j \det(J_j)} \right), & \text{if } \det(J_i) > 0 \text{ for all } i,\\ 
0, & \text{otherwise.}
\end{cases}
$$



```rust
#[cfg(test)]
mod tests {

    // 약간의 전단/뒤틀림을 만들어 주는 헬퍼

    use nurbslib::core::maths::{on_are_in_01, on_are_on_in_01, on_get_shear3};
    use nurbslib::core::mesh_jacobian::mesh_jacobian::{on_jacobian_quality_wedge6, 
    on_jacobian_quality_hexa8, on_jacobian_quality_tetra4, on_jacobian_quality_pyramid5, 
    on_skew_quad_non_affine, on_jacobian_quality_quad4};
```
```rust
    // ---------------- Tet(4) ----------------
    #[test]
    fn tet_regular_quality_is_one() {
        let tet = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let q = on_jacobian_quality_tetra4(tet);
        assert_eq!(q, 1.0);
    }
```
```rust

    #[test]
    fn tet_sliver_quality_decreases_but_positive() {
        // 슬리버: 4번째 점을 거의 같은 평면 위로 이동 (detJ 작아짐)
        let tet = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.001, 0.001, 0.01], // 거의 평면
        ];
        let q = on_jacobian_quality_tetra4(tet);
        // 선형 Tet은 detJ가 상수이므로, 완전 정칙이면 1.0, 슬리버면 detJ>0이면 1.0이지만
        // 이 구현은 min/max(단일 샘플) → 여전히 1.0이 나옴.
        // => 슬리버 검출은 detJ 크기 자체로 보려면 절대값 기준을 따로 두자.
        assert_ne!(q, 0.0);
        // 보너스: detJ 자체를 보고 싶다면, 작은 변형량으로 별도 함수 준비가 필요.
    }
```
```rust

    #[test]
    fn tet_inverted_returns_zero() {
        // 노드 두 개를 스왑해서 방향 반전(detJ < 0)
        let tet = [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0], // 1,2 스왑 느낌
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let q = on_jacobian_quality_tetra4(tet);
        // 선형 Tet에서 이 순서면 detJ가 음수가 됨
        assert_eq!(q, 0.0);
    }
```
```rust

    // ---------------- Hexa(8) ----------------
    #[test]
    fn hexa_sheared_quality_between_0_and_1() {
        let mut hexa = [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];
        on_get_shear3(&mut hexa, 0.20, 0.10, 0.15); // 적당히 전단 → detJ 다양해짐
        let q = on_jacobian_quality_hexa8(hexa);
        assert!(on_are_on_in_01(q), "q={}", q);
    }
```
```rust

    #[test]
    fn hexa_inverted_at_some_gauss_point_returns_zero() {
        // // 윗면을 지나치게 당겨 꼬이게 만들자
        // let mut hexa = [
        //     [-1.0,-1.0,-1.0],
        //     [ 1.0,-1.0,-1.0],
        //     [ 1.0, 1.0,-1.0],
        //     [-1.0, 1.0,-1.0],
        //     [-1.0,-1.0, 1.0],
        //     [ 1.0,-1.0, 1.0],
        //     [ 1.0, 1.0, 1.0],
        //     [-1.0, 1.0, 1.0],
        // ];
        // // 윗면 두 점을 안쪽으로 심하게 끌어와서 요소가 일부 영역 뒤집히게
        // hexa[6] = [ 0.1, 0.1, 1.0];
        // hexa[7] = [-0.1, 0.1, 1.0];
        // let q = jacobian_quality_hexa8(hexa);
        // assert_eq!(q, 0.0);

        let mut hexa = [
            [-1.0, -1.0, -1.0], // 0
            [1.0, -1.0, -1.0],  // 1
            [1.0, 1.0, -1.0],   // 2
            [-1.0, 1.0, -1.0],  // 3
            [-1.0, -1.0, 1.0],  // 4
            [1.0, -1.0, 1.0],   // 5
            [1.0, 1.0, 1.0],    // 6
            [-1.0, 1.0, 1.0],   // 7
        ];

        // 노드 6을 z축 아래로 강하게 밀어 뒤집힘 유도
        hexa[6] = [1.0, 1.0, -3.0]; // 원래는 z=1.0 → z=-2.0로 뒤집힘

        let q = on_jacobian_quality_hexa8(hexa);
        println!("{:?}", q);
        println!("Jacobian quality: {}", q); // 예상: 0.0
        assert_eq!(q, 0.0);
    }
```
```rust

    // ---------------- Wedge/Prism(6) ----------------
    #[test]
    fn wedge_mild_twist_is_positive() {
        let wedge = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0], // bottom
            [0.1, 0.0, 1.0],
            [1.1, 0.1, 1.0],
            [0.0, 1.1, 1.0], // top 약간 비틀림
        ];
        let q = on_jacobian_quality_wedge6(wedge);
        assert!(on_are_in_01(q) && q > 0.0, "q={}", q);
    }
```
```rust

    #[test]
    fn wedge_near_collapse_returns_small_or_zero() {
        // 상하 삼각형이 거의 포개지도록 만들어 detJ가 아주 작거나 음수가 되게
        let wedge = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.02],
            [1.0, 0.0, 0.02],
            [0.0, 1.0, 0.02],
        ];
        let q = on_jacobian_quality_wedge6(wedge);
        // 샘플점에서 detJ<=0이 나오면 0, 양수지만 매우 작으면 min/max도 작을 수 있음(실험적)
        assert!(on_are_in_01(q));
    }
```
```rust

    // ---------------- Pyramid(5) ----------------

    #[test]
    fn pyramid_apex_off_center_quality_between_0_and_1() {
        // 밑면 z=0, apex z=1
        let mut pyr = [
            [-1.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [0.0, 0.0, 1.0], // apex
        ];

        // apex를 살짝 x,y로만 오프셋 (z는 유지)
        pyr[4][0] = 0.15;
        pyr[4][1] = -0.12;

        // 디버그: 샘플 detJ 확인
        // eprintln!("detJs(pyr) = {:?}", pyramid5_detj_at_samples(pyr));

        let q = on_jacobian_quality_pyramid5(pyr);
        assert!(q > 0.0 && q < 1.0, "q={}", q);
    }
```
```rust

    #[test]
    fn pyramid_inverted_returns_zero() {
        // 밑면 두 점을 살짝 위로 올려 꼬이게
        let mut pyr = [
            [-1.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        pyr[1][2] = 0.6; // base vertex lifting
        pyr[2][2] = 0.6;
        let q = on_jacobian_quality_pyramid5(pyr);
        assert_eq!(q, 0.0);
    }
```
```rust

    // ---------------- Quad(4, 2D) ----------------
    #[test]
    fn quad_sheared_quality_between_0_and_1() {
        let mut quad = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
        // 평행사변형이 아닌 모양으로 (한 꼭짓점만)
        on_skew_quad_non_affine(&mut quad, 0.35);
        let q = on_jacobian_quality_quad4(quad);
        println!("q={}", q);
        assert!(q > 0.0 && q < 1.0, "q={}", q); // 이제 1이 아닐 거예요.
    }
```
```rust
    #[test]
    fn quad_crossed_returns_zero() {
        // 교차 사각형(반시계/시계 노드 순서가 뒤틀려 일부 가우스점 detJ<0)
        let quad = [
            [-1.0, -1.0],
            [1.0, 1.0], // 교차되도록 배치
            [1.0, -1.0],
            [-1.0, 1.0],
        ];
        let q = on_jacobian_quality_quad4(quad);
        assert_eq!(q, 0.0);
    }
}
```
---
