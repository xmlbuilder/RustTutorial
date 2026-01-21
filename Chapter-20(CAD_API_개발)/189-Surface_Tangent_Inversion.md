# Surface Tangent Vector Inversion
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
- 하지만 surface tangent plane에서는 $\mathbf{T} 가 \mathbf{S_{\mathnormal{u}}}, \mathbf{S_{\mathnormal{v}}}$ 로  
    span되므로 least squares 방식으로 정확한 해가 존재함.

### 📐 Metric Tensor 방식으로 정리
- 위 식을 내적 기반으로 정리하면:
```math
\left[ \begin{matrix}\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{u}}}&\mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}}\\ \mathbf{S_{\mathnormal{u}}}\cdot \mathbf{S_{\mathnormal{v}}}&\mathbf{S_{\mathnormal{v}}}\cdot \mathbf{S_{\mathnormal{v}}}\end{matrix}\right] \left[ \begin{matrix}du\\ dv\end{matrix}\right] =\left[ \begin{matrix}\mathbf{T}\cdot \mathbf{S_{\mathnormal{u}}}\\ \mathbf{T}\cdot \mathbf{S_{\mathnormal{v}}}\end{matrix}\right]
``` 
- 이게 바로 surface metric tensor를 이용한 tangent inversion 방식이고,  
    Piegl 책에서도 이 방식으로 설명.

### 🧮 Rust 코드 대응
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
- 즉, 3D tangent → (du, dv) 파라미터 방향으로 역변환하는 과정이다.


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

```math
W=\left[ \begin{matrix}du\\ dv\end{matrix}\right]
```
- 우리가 찾고 싶은 파라미터 방향
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
```math
\min _W\| MW-T\| ^2
```
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

# du, dv의 의미

## ⭐ du, dv는 “surface 1st derivative 값”이 아니다.
- du, dv는 파라미터 공간에서의 변화량(velocity) 이고,  
    Su, Sv가 surface의 1차 미분(기저 벡터) 이다.
- 즉:
    - Su, Sv = surface의 1st derivative (기하학적 basis vectors)
    - du, dv = 그 basis 방향으로 얼마나 움직일지 나타내는 계수
- 둘은 완전히 다른 개념.

## 📘 개념 정리
- ✔ Surface 1st derivatives
```math
S_u=\frac{\partial S}{\partial u},\quad S_v=\frac{\partial S}{\partial v}
```
- 3D 벡터
    - surface의 tangent plane을 구성하는 기저 벡터(basis vectors)
    - 단위 벡터가 아님
    - 곡면의 stretching, bending에 따라 크기와 방향이 달라짐

- ✔ du, dv
```math
T=du\cdot S_u+dv\cdot S_v
```
- 여기서 du, dv는:
- 파라미터 공간(u,v)에서의 변화량
    - 즉, surface 위에서 “얼마나 u 방향으로 움직이고, 얼마나 v 방향으로 움직이는지”
    - 단위는 “parameter units”
    - 3D 벡터가 아니라 스칼라 값

## 📐 왜 du, dv가 1st derivative가 아닌가?
- Surface 1st derivative는 기저 벡터이고,
    du, dv는 그 기저 벡터의 계수(coefficient) 이기 때문.
- 비유하자면:
    - Su, Sv = x축, y축 같은 좌표축
    - du, dv = 그 축 방향으로 얼마나 갈지 나타내는 값
- 즉,
    - Su, Sv는 방향이고
    - du, dv는 그 방향으로의 이동량이다.

## 🧠 Tangent inversion의 의미
- 3D tangent T가 주어졌을 때:
```math
T=du\cdot S_u+dv\cdot S_v
```
- 이 식을 풀어서:
    - du = u 방향으로 얼마나 움직여야 T가 되는가
    - dv = v 방향으로 얼마나 움직여야 T가 되는가
- 를 찾는 과정이 바로 surface tangent inversion.

## 🎯 예시로 보면 더 명확해짐
- 만약 surface가 u 방향으로 늘어나 있다면:
    - Su가 매우 길다
    - 같은 T를 만들기 위해 du는 작아진다
- 반대로 surface가 v 방향으로 좁게 압축되어 있다면:
    - Sv가 짧다
    - 같은 T를 만들기 위해 dv는 커진다
- 즉, du, dv는 surface의 geometry에 따라 달라지는 scaling factor.

## 🧾 요약
| 개념 | 의미 |
|------|------|
| Su, Sv | surface의 1st derivative (tangent basis vectors) |
| du, dv | parameter space에서의 변화량 (basis의 계수) |
| 관계 | T = du·Su + dv·Sv |
| du, dv의 역할 | 3D tangent를 만들기 위해 Su, Sv를 얼마나 섞을지 결정 |
| Su, Sv와의 차이 | Su,Sv는 벡터; du,dv는 스칼라 |

---

## 🌟 1. Surface 위에서 곡선을 따라 이동할 때 (Curve on Surface Integration)
- Surface 위에서 어떤 방향으로 “한 걸음” 움직이고 싶다고 해보자.
    - 3D tangent T는 알고 있음
    - 하지만 surface는 (u,v) 파라미터 공간에서 정의됨
    - 그래서 T를 (du, dv)로 변환해야 surface 위에서 이동할 수 있음
- 즉:
```math
\frac{dS}{dt}=T\quad \Rightarrow \quad \frac{du}{dt},\frac{dv}{dt}\mathrm{\  필요}
```
- 이게 바로 tangent inversion이 필요한 순간.
- 적용 예:
    - Surface trimming curve 생성
    - Surface 위에서 particle 이동
    - Surface flow simulation
    - Surface offset 곡선 생성

## 🌟 2. Surface 위에서 Newton iteration 할 때
- 예를 들어, 어떤 3D 점 P가 있을 때  
    surface 위에서 가장 가까운 점을 찾는 알고리즘을 생각해보자.
- Newton iteration은 이렇게 생김:
```math
\left[ \begin{matrix}\Delta u\\ \Delta v\end{matrix}\right] =-(J^TJ)^{-1}J^Tr
```
- 여기서 J는 Su, Sv로 구성된 Jacobian.
- 즉, Newton step을 계산하려면 tangent inversion이 필수
- 적용 예:
    - Point projection onto surface
    - Surface-surface intersection (SSI)
    - Curve-surface intersection (CSI)
    - Closest point computation

## 🌟 3. Surface parameterization 기반의 곡선 생성
- Surface 위에서 어떤 방향 벡터 field를 따라 곡선을 만들고 싶을 때:
    - 방향 field는 3D 벡터
    - surface는 (u,v) 공간
    - 따라서 방향 field를 (du,dv)로 변환해야 곡선을 적분할 수 있음
- 적용 예:
    - Surface geodesic curve 생성
    - Surface iso-parameter curve 생성
    - Surface flow line 생성

## 🌟 4. Surface offset / normal variation 계산
- Offset surface를 만들 때:
```math
S_{offset}(u,v)=S(u,v)+d\cdot n(u,v)
```
- offset 곡선의 tangent를 계산하려면 surface tangent inversion이 필요.

## 🌟 5. Surface deformation / morphing / animation
- Surface를 변형할 때:
    - 3D 변위 벡터 ΔP가 주어짐
    - 이를 (du,dv)로 변환해야 surface parameter domain에서 변형을 적용할 수 있음

## 🌟 6. Surface parameter optimization / reparameterization
- Surface 위에서 어떤 목적함수를 최소화할 때:
    - gradient는 3D 벡터
    - 하지만 최적화 변수는 (u,v)
    - 따라서 gradient를 (du,dv)로 변환해야 함

## Surface Tangent Vector Inversion — 어디에 쓰는가?

| 용도 | 설명 |
|------|------|
| Surface 위에서 곡선 적분 | 3D tangent → (du,dv) 변환 필요 |
| Newton iteration | point projection, SSI, CSI 등에서 필수 |
| Surface flow / geodesic | 방향 field를 파라미터 공간으로 변환 |
| Offset surface 계산 | offset tangent 계산에 필요 |
| Surface deformation | 3D 변위를 (du,dv)로 변환 |
| Parameter optimization | gradient를 파라미터 공간으로 변환 |


- Surface tangent inversion은  
    **3D 공간에서의 방향을 surface 파라미터 공간으로 변환하는 기술** 이고,
    surface 위에서 움직이거나 계산하는 모든 알고리즘의 핵심이다.

---

## t의 의미
- 미분기하학에서 흔히 쓰는 곡선의 매개변수(parameter) 를 의미.

### 🌱 t는 시간(time)이 아니라 **곡선을 따라 움직이는 매개변수**
- Surface 위에서 어떤 곡선을 가정:
```math
C(t)=S(u(t),v(t))
```
- 여기서 t는 단순히 곡선을 따라 움직이는 인덱스 같은 것.
    - t = 0 → 곡선의 시작점
    - t = 1 → 곡선의 끝점
    - t = 0.5 → 중간
    - t는 시간일 수도 있지만, 보통은 **곡선의 매개변수** 일 뿐
- 즉, t는 곡선의 진행 정도를 나타내는 변수.

### 🌟 그럼 dS/dt = T는 무슨 뜻인가?
- Surface 위의 곡선 C(t)가 있을 때:
```math
\frac{dS}{dt}=\frac{dC}{dt}=T
```
- 이 말은:
    - **곡선을 따라 t가 조금 변할 때, surface 위의 점 S가 어떻게 움직이는지**
    - **그 순간의 3D 방향 벡터가 T이다**

- 즉, T는 surface 위에서 곡선이 진행하는 방향.

### 🧠 Chain Rule로 풀어보면 더 명확
```math
S(u(t),v(t))
```
- 이걸 t로 미분하면:
```math
\frac{dS}{dt}=S_u\frac{du}{dt}+S_v\frac{dv}{dt}
```
- 여기서:
    - $S_u,S_v$ = surface의 1차 미분 (tangent basis)
    - du/dt,dv/dt = 파라미터 공간에서의 속도
    - dS/dt = 3D 공간에서의 속도 = T
- 그래서:
```math
T=\frac{dS}{dt}=S_u\frac{du}{dt}+S_v\frac{dv}{dt}
```
- 이걸 역으로 풀면:
```math
\frac{du}{dt},\frac{dv}{dt}
```
- 즉, du, dv를 구하는 것이 바로 tangent inversion.

## 🎯 정리
| 기호 | 의미 |
|------|------|
| t | 곡선의 매개변수 (시간 아님) |
| dS/dt | surface 위에서 곡선이 움직이는 3D 방향 (tangent) |
| Su, Sv | surface의 1차 미분 (tangent basis vectors) |
| du/dt, dv/dt | 파라미터 공간에서의 속도 |
| T | 주어진 3D tangent vector |


- t는 단순히 곡선을 따라 움직이는 매개변수이고,  
    dS/dt = T는 **surface 위에서 곡선이 진행하는 3D 방향이 T이다** 라는 뜻이다.

- 즉, Su, Sv는 단순한 미분값이 아니라 곡면이 그 지점에서 어떻게 생겼는지를 나타내는 기하학적 벡터.

### 🌄 Su, Sv는 무엇인가?
- Surface가 이렇게 정의되어 있다고 하자:
```math
S(u,v)=(x(u,v),y(u,v),z(u,v))
```
-그러면:
```math
S_u=\frac{\partial S}{\partial u},\quad S_v=\frac{\partial S}{\partial v}
```
- 이 두 벡터는:
    - surface의 tangent plane을 구성하는 기저 벡터
    - surface의 기하학적 성질
    - surface의 stretching, bending, orientation을 모두 담고 있음
    - 단순히 u, v의 함수가 아니라 surface shape 자체의 특성
- 즉, Su, Sv는 surface가 그 지점에서 **어떤 방향으로 늘어나 있는지** 를 나타내는 벡터.
- 🌱 u, v는 단순한 파라미터일 뿐
    - u, v는:
    - surface를 정의하기 위한 좌표계
    - 2D parameter domain의 위치
    - geometry가 아님
    - 단순한 index 같은 것
- 즉, u, v는 **주소** 이고 Su, Sv는 **그 주소에 있는 surface의 기하학적 성질**.

### 🧠 Su, Sv가 왜 surface의 특징인가?
- 예를 들어, 같은 u 방향이라도:
    - surface가 늘어나 있으면 Su가 길어짐
    - surface가 압축되어 있으면 Su가 짧아짐
    - surface가 비틀려 있으면 Su 방향이 회전함
- 즉, Su는 surface의 shape에 따라 달라지는 geometry-dependent vector.
- 반면 u는 단순히 “0.3, 0.5” 같은 숫자일 뿐.

### 🎯 표로 정리
| 개념 | 의미 |
|------|------|
| u, v | surface parameter (주소, 좌표) |
| Su, Sv | surface의 기하학적 특징 (tangent basis vectors) |
| Su, Sv의 역할 | surface가 그 지점에서 어떻게 생겼는지 나타냄 |
| u, v의 역할 | surface 위의 위치를 지정하는 파라미터 |
| 관계 | Su, Sv는 u, v에 따라 계산되지만, geometry를 나타냄 |

### 🔥 한 문장으로 요약하면
- u, v는 단순한 좌표이고, Su, Sv는 그 좌표에서의 surface의 기하학적 성질이다.
- 그래서 tangent inversion에서:
    - Su, Sv는 **surface의 방향축**
    - du, dv는 **그 방향축으로 얼마나 움직일지**
- 이렇게 역할이 나뉘는 것임

---
## 소스 코드
```rust

/// Invert surface curve tangent:
/// Given surface S(u,v) and tangent T in 3D,
/// compute (du, dv) such that
///     T = du * Su + dv * Sv
///
/// Equivalent to solving:
/// [ Su·Su  Su·Sv ] [du] = [ T·Su ]
/// [ Su·Sv  Sv·Sv ] [dv]   [ T·Sv ]
pub fn on_surface_tangent_inverse(
    sur: &NurbsSurface,
    u: Real,
    v: Real,
    tangent: Vector3D,
) -> Result<Vector2D> {

    // --- 1. evaluate first derivatives ---
    // ders[du][dv], we need:
    // Su = ders[1][0], Sv = ders[0][1]
    let ders = sur.eval_ders_nder(u, v, 1);
    if ders.len() < 2 || ders[0].len() < 2 {
        return Err(NurbsError::InvalidDerivative{ msg :
            "surface derivatives unavailable".into() });
    }

    let su = ders[1][0];
    let sv = ders[0][1];

    // --- 2. metric coefficients ---
    let fu = su.dot(&su); // Su·Su
    let fv = su.dot(&sv); // Su·Sv
    let gv = sv.dot(&sv); // Sv·Sv

    let f = tangent.dot(&su); // T·Su
    let g = tangent.dot(&sv); // T·Sv

    // determinant
    let den = fu * gv - fv * fv;
    if den.abs() < 1e-14 {
        return Err(NurbsError::InvalidSurfaceMetric{ msg :
            "degenerate surface metric (Jacobian nearly singular)".into()});
    }

    // --- 3. solve 2x2 system ---
    let du = (f * gv - g * fv) / den;
    let dv = (fu * g - fv * f) / den;

    Ok(Vector2D::new(du, dv))
}
```
---
## 테스트 코드
```rust
use nurbslib::core::nurbs_surface::{on_surface_tangent_inverse, NurbsSurface};
use nurbslib::core::geom::{Point4D, Vector3D};
use nurbslib::core::knot::KnotVector;
```
```rust
#[test]
fn test_surface_tangent_inverse_plane() {
    // S(u,v) = (u, v, 0)
    let sur = NurbsSurface::from_plane_xy().expect("Invalid Nurbs Surface");

    let u = 0.3;
    let v = 0.7;

    let du_true = 2.0;
    let dv_true = -1.5;

    let tangent = Vector3D::new(du_true, dv_true, 0.0);

    let uv = on_surface_tangent_inverse(&sur, u, v, tangent).unwrap();

    assert!((uv.x - du_true).abs() < 1e-12);
    assert!((uv.y - dv_true).abs() < 1e-12);
}

/// Smooth curved surface for testing
/// S(u,v) = (u, v, alpha * u * v)
pub fn some_curved_surface() -> Option<NurbsSurface> {
    let alpha = 0.5;

    let degree_u = 2;
    let degree_v = 2;

    let nu = 3;
    let nv = 3;

    let mut ctrl_flat = Vec::with_capacity(nu * nv);

    for v in 0..nv {
        let vv = v as f64 / (nv as f64 - 1.0);
        for u in 0..nu {
            let uu = u as f64 / (nu as f64 - 1.0);
            ctrl_flat.push(Point4D::new(
                uu,
                vv,
                alpha * uu * vv,
                1.0,
            ));
        }
    }

    let ku = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]).expect("Invalid Knot Vector");
    let kv = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]).expect("Invalid Knot Vector");

    NurbsSurface::new_from_flat_ctrl(
        degree_u,
        degree_v,
        nu,
        nv,
        ctrl_flat,
        ku,
        kv,
    )
}
```
```rust
#[test]
fn test_surface_tangent_inverse_reconstructs_tangent() {

    let sur = some_curved_surface().expect("Invalid Nurbs Surface");

    let u = 0.4;
    let v = 0.6;

    let ders = sur.eval_ders_nder(u, v, 1);
    let su = ders[1][0];
    let sv = ders[0][1];

    let du = 0.8;
    let dv = -0.3;

    let tangent = su * du + sv * dv;

    let uv = on_surface_tangent_inverse(&sur, u, v, tangent).unwrap();

    assert!((uv.x - du).abs() < 1e-10);
    assert!((uv.y - dv).abs() < 1e-10);
}
```
---
