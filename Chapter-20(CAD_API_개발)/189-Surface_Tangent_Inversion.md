# 📘 개념: Surface Tangent Vector Inversion
- 문제 정의
    - 주어진 surface $\mathbf{S}(u,v)$
    - 특정 파라미터 위치 $(\bar {u},\bar {v})$ 에서의 점 $\mathbf{P}=\mathbf{S}(\bar {u},\bar {v})$
    - 그 점에서의 tangent vector $\mathbf{T}\in \mathbb{R^{\mathnormal{3}}}$ 가 주어졌을 때
    - 이 벡터가 surface의 파라미터 방향으로 어떻게 구성되는지 알고 싶다
    - 즉,
    ```math
    \mathbf{T}=du\cdot \mathbf{S_{\mathnormal{u}}}+dv\cdot \mathbf{S_{\mathnormal{v}}}
    ```
    - 이때 $\mathbf{S_{\mathnormal{u}}},\mathbf{S_{\mathnormal{v}}}$ 는 surface의 partial derivatives.

- 🧠 수식 유도
- 이건 단순한 선형 시스템:
```math
\left[ \begin{matrix}x_u&x_v\\ y_u&y_v\\ z_u&z_v\end{matrix}\right] \left[ \begin{matrix}du\\ dv\end{matrix}\right] =\left[ \begin{matrix}dx\\ dy\\ dz\end{matrix}\right]
``` 
- 이건 3개의 방정식에 2개의 미지수 → 일반적으로 해가 없음.
- 하지만 surface tangent plane에서는 $\mathbf{T} 가 \mathbf{S_{\mathnormal{u}}},\mathbf{S_{\mathnormal{v}}}$ 로  
    span되므로 least squares 방식으로 정확한 해가 존재함.

### 📐 Metric Tensor 방식으로 정리
- 위 식을 내적 기반으로 정리하면:
```math
\left[ \begin{matrix}\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{u}}}&\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}}\\ \mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}}&\mathbf{S_{\mathnormal{v}}}\cdot \mathbf{S_{\mathnormal{v}}}\end{matrix}\right] \left[ \begin{matrix}du\\ dv\end{matrix}\right] =\left[ \begin{matrix}\mathbf{T}\cdot \mathbf{S_{\mathnormal{u}}}\\ \mathbf{T}\cdot \mathbf{S_{\mathnormal{v}}}\end{matrix}\right]
``` 
- 이게 바로 surface metric tensor를 이용한 tangent inversion 방식이고,  
    Piegl 책에서도 이 방식으로 설명.

###🧮 Rust 코드 대응
- 코드:
```rust
let fu = su.dot(&su); // Su·Su
let fv = su.dot(&sv); // Su·Sv
let gv = sv.dot(&sv); // Sv·Sv

let f = tangent.dot(&su); // T·Su
let g = tangent.dot(&sv); // T·Sv
```

- 정확히 위 수식의 좌변 행렬과 우변 벡터를 계산한 것.
- 그리고 2×2 선형 시스템을 직접 해석적으로 푼 부분:
```rust
let den = fu * gv - fv * fv;
let du = (f * gv - g * fv) / den;
let dv = (fu * g - fv * f) / den;
```
- 이건 2×2 시스템의 해석적 해:

```math
\begin{aligned}du&=\frac{(\mathbf{T}\cdot \mathbf{S_{\mathnormal{u}}})\cdot (\mathbf{S_{\mathnormal{v}}}\cdot \mathbf{S_{\mathnormal{v}}})-(\mathbf{T}\cdot \mathbf{S_{\mathnormal{v}}})\cdot (\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}})}{\det }\\ dv&=\frac{(\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{u}}})\cdot (\mathbf{T}\cdot \mathbf{S_{\mathnormal{v}}})-(\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}})\cdot (\mathbf{T}\cdot \mathbf{S_{\mathnormal{u}}})}{\det }\end{aligned}
```
- 여기서:
```math
\det =(\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{u}}})(\mathbf{S_{\mathnormal{v}}}\cdot \mathbf{S_{\mathnormal{v}}})-(\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}})^2
```
- 정확히 den 계산과 일치.

### ⚠️ 왜 det이 작으면 오류를 내는가?
- det이 작다는 건 $\mathbf{S_{\mathnormal{u}}},\mathbf{S_{\mathnormal{v}}}$ 가  
    거의 선형 종속이라는 뜻
    - 즉, tangent plane이 거의 퇴화됨
    - 이 경우 역방향 계산이 불안정해지므로 오류를 내는 게 맞다

### 🎯 적용 방식
이 함수는 다음과 같은 상황에서 쓰인다:
- surface 위에서 어떤 3D 방향으로 움직이고 싶을 때
    - 그 방향이 파라미터 공간에서 어떤 (du, dv) 변화로 대응되는지 알고 싶을 때
    - 예: surface 위에서 곡선을 따라 이동할 때, 곡선의 tangent를 surface 파라미터로 변환
- 즉,
- 3D tangent → (du, dv) 파라미터 방향으로 역변환하는 과정이다.


# 🧾 Surface Tangent Vector Inversion — 정리

| 항목       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| 목적       | 3D tangent vector를 surface 파라미터 방향 (du, dv)로 역변환         |
| 입력       | Surface S(u,v), 위치 (u,v), tangent vector T                         |
| 출력       | 파라미터 방향 변화량 (du, dv)                                        |
| 수식       | T = du·Su + dv·Sv                                                    |
| 방식       | metric tensor 기반 2×2 선형 시스템 해석적 해                         |
| 핵심 행렬  | [Su·Su  Su·Sv] [du] = [T·Su]                                          |
|            | [Su·Sv  Sv·Sv] [dv]   [T·Sv]                                          |
| 조건       | Su × Sv ≠ 0, det ≠ 0                                                  |
| 적용 예시  | surface 위에서 곡선을 따라 이동하거나, tangent 방향을 파라미터로 변환 |


---
## Metric Tensor 유도

- 🎯 문제 정의
- 우리는 다음 선형 시스템을 풀고 싶음:
```mat
MW=T
```
- 여기서:
- $M\in \mathbb{R^{\mathnormal{3\times 2}}}$: surface의 partial derivatives
- $W=\left[ \begin{matrix}du\\ dv\end{matrix}\right]$ : 우리가 찾고 싶은 파라미터 방향
- $T\in \mathbb{R^{\mathnormal{3}}}$: 주어진 tangent vector

- ❌ 왜 이 시스템은 일반적으로 해가 없을 수 있는가?
    - M은 3×2 행렬 → 미지수 2개, 방정식 3개
    - 즉, overdetermined system
    - 일반적으로 T가 M의 column space에 없으면 정확한 해가 존재하지 않음

### ✅ 그런데 surface tangent plane에서는 해가 존재한다
- 왜냐면:
    - T는 surface의 tangent plane 위에 있음
    - tangent plane은 $S_u,S_v$ 로 span됨
    - 즉, $T=du\cdot S_u+dv\cdot S_v$ 형태로 정확히 표현 가능
- 이 경우에는 정확한 해가 존재함

### 🧠 그럼 어떻게 해를 구하나?
- 우리는 다음을 만족하는 W 를 찾고 싶어:
```math
\min _W\| MW-T\| ^2
```
- 즉, 잔차 에너지 최소화
- 이걸 미분해서 0으로 만들면:
```math
M^TMW=M^TT
```
- 이게 바로 normal equation이야.

### 📐 기하학적으로 보면
- MW 는 surface tangent plane 위의 벡터
- T 는 주어진 tangent vector
- 우리는 MW 가 T 와 최대한 가까워지도록 W 를 조정하고 싶다
- 그 차이(잔차)가 최소가 되는 조건이 바로
```math
M^T(MW-T)=0
```
- 정리하면:
```math
M^TMW=M^TT
```
### 🧮 수식 흐름 요약
- 원래 식:
```math
MW=T
```
- 일반적으로 해가 없을 수 있음 → least squares로 접근
$\min _W\| MW-T\| ^2$
- 미분해서 0으로 만들면:
```math
M^TMW=M^TT
```
- 이걸 풀면 잔차가 최소가 되는 W 를 얻는다
    - surface tangent plane에서는 이게 정확한 해가 된다

### 🎉 결론
- 이 식은 least squares 해를 구하기 위한 normal equation
- 이건 단순히 “잔차 에너지 최소화”를 위한 미분 결과
- surface tangent plane에서는 이 식이 정확한 해를 보장함
- 그래서 Piegl 책에서도 이 식을 바로 써서 tangent inversion을 수행
---

