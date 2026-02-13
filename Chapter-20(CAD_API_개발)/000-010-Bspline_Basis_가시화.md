# Bspline Basis
## 직접 눈으로 확인할 수 있는 그래프 시각화
- 여기서는 우리가 앞에서 사용한 동일한 예제를 사용한다:
- degree p = 2
- knot vector
```
U = {0, 0, 0, 1, 2, 3, 3, 3}
```
- basis:
```math
N_{0,2}, N_{1,2}, N_{2,2}, N_{3,2}
```
- 그리고 u ∈ [0, 3] 구간에서 각 basis가 어떻게 생기는지 가시화

![Quadratic B‑spline basis](/image/bspline_p2.png)


### 📘 1. Quadratic B‑spline basis


- ✔ N₀,₂(u)
  - support: 0, 2)
  - 0에서 시작해서 1까지 증가
  - 1에서 2까지 감소
  - 2 이후는 0

- ✔ N₁,₂(u)
  - support: 0, 3)
  - 가장 넓은 support
  - 전체 곡선의 “중심” 역할
  - degree가 높아질수록 이런 basis가 더 부드럽고 넓어진다

- ✔ N₂,₂(u)
  - support: 1, 3)
  - 1에서 시작
  - 2에서 peak
  - 3에서 0

- ✔ N₃,₂(u)
  - support: 2, 3)
  - 오른쪽 끝에서만 작동하는 basis
  - open knot 때문에 끝에서 support가 짧다

## 📘 2. 전체 basis를 한눈에 보기
- 이 그래프는 다음을 보여준다:
  - 각 basis는 local support를 가진다
  - basis들은 겹치면서도 partition of unity를 만족한다
  - degree가 높을수록 basis가 더 넓고 부드러워진다
  - knot 중복 때문에 일부 basis는 support가 짧아지거나 죽는다

## 📘 3. 왜 어떤 basis는 “죽는가”?
- 이 예제에서 0차 basis 중:
```math
N_{0,0}, N_{1,0}, N_{5,0}, N_{6,0}
```

- 은 모두 항상 0이다.
- 이유는 간단하다:
- ✔ 이유 1 — knot 중복으로 support가 0
  - 예:
```
u0 = u1 = u2 = 0
```

- 0,0) 구간은 길이가 0
- N_{0,0}, N_{1,0} = 0
- ✔ 이유 2 — 재귀식 분모가 0
  - 예:
```
u_i = u_{i+p}
```

- (u - u_i)/(u_{i+p} - u_i) = 0/0 → 0 처리
- basis가 살아날 기회가 없음
- ✔ 이유 3 — support가 비어 있음
  - 예:
```
u5 = u6 = u7 = 3
```

- 3,3) 구간은 길이가 0
- $N_{5,0}, N_{6,0}$ = 0

## 📘 4. 전체 생성 과정 요약
- 0차 basis 생성
    - knot 구간이 0 길이면 basis는 죽음
- 1차 basis 생성
    - 0차 basis가 0이면 higher degree도 0
    - 분모가 0이면 해당 항은 0
- 2차 basis 생성
    - 살아남은 basis만 재귀적으로 조합
    - 결국 4개의 quadratic basis가 생성됨
- 결과
    - B‑spline basis는 knot 구조에 따라 살아나거나 죽는다
    - degree가 높아질수록 basis는 넓고 부드러워진다
    - local support, partition of unity, $C^{p−1}$ continuity를 만족

- degree p = 2
- knot vector U = [0, 0, 0, 1, 2, 3, 3, 3]
- basis: $N_{0,2}, N_{1,2}, N_{2,2}, N_{3,2}$

```python
import numpy as np
import matplotlib.pyplot as plt

# Knot vector (u notation)
U = np.array([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0])
p = 2  # degree
n = len(U) - p - 1  # number of basis functions = 4 (indices 0..3)


def N_ip(u, i, p, U):
    """Evaluate B-spline basis N_{i,p}(u) using Cox–de Boor recursion."""
    if p == 0:
        return np.where((u >= U[i]) & (u < U[i+1]), 1.0, 0.0)

    left_denom = U[i+p] - U[i]
    right_denom = U[i+p+1] - U[i+1]

    left = 0.0
    right = 0.0

    if left_denom != 0:
        left = (u - U[i]) / left_denom * N_ip(u, i, p-1, U)
    if right_denom != 0:
        right = (U[i+p+1] - u) / right_denom * N_ip(u, i+1, p-1, U)

    return left + right


# 샘플링 구간 (전체 유효 구간: [U[p], U[-p-1]] = [0, 3])
u_vals = np.linspace(0.0, 3.0, 400)

# 각 basis N_{i,2}(u) 계산
basis_vals = []
for i in range(n):  # i = 0,1,2,3
    basis_vals.append(N_ip(u_vals, i, p, U))

# 플롯
plt.figure(figsize=(8, 4))
colors = ["tab:blue", "tab:orange", "tab:green", "tab:red", "tab:purple"]

for i in range(n):
    plt.plot(u_vals, basis_vals[i], label=f"N_{{{i},2}}(u)", color=colors[i])

# knot 위치 표시 (세로선)
for uk in U:
    plt.axvline(uk, color="gray", linestyle="--", linewidth=0.5)

plt.title("Quadratic B-spline Basis Functions (p = 2)\nU = {0,0,0,1,2,3,3,3}")
plt.xlabel("u")
plt.ylabel("N_{i,2}(u)")
plt.legend()
plt.grid(True, alpha=0.3)
plt.tight_layout()
plt.show()
```
- 이 스크립트는:
- 동일한 knot vector
```
U = {0, 0, 0, 1, 2, 3, 3, 3}
```
- degree = 1, 2, 3
- 각 degree에 대해 모든 basis $N_{i,p}(u)$ 를 계산
- subplot 3개로 한 화면에 비교 까지 완벽하게 포함한다.

📘 p=1, p=2, p=3
```python
import numpy as np
import matplotlib.pyplot as plt

# Knot vector
U = np.array([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0])

def N_ip(u, i, p, U):
    """Evaluate B-spline basis N_{i,p}(u) using Cox–de Boor recursion."""
    u = np.asarray(u)

    if p == 0:
        # 기본은 [U[i], U[i+1]) 에서 1
        out = np.where((u >= U[i]) & (u < U[i+1]), 1.0, 0.0)
        # 맨 끝 u == U[-1] 포함(마지막 basis가 1 되도록)
        if i == len(U) - 2:
            out = np.where(u == U[-1], 1.0, out)
        return out

    left_denom  = U[i + p]     - U[i]
    right_denom = U[i + p + 1] - U[i + 1]

    # IMPORTANT: 항상 u와 같은 shape의 0 배열로 초기화
    left  = np.zeros_like(u, dtype=float)
    right = np.zeros_like(u, dtype=float)

    if left_denom != 0.0:
        left = (u - U[i]) / left_denom * N_ip(u, i, p - 1, U)
    if right_denom != 0.0:
        right = (U[i + p + 1] - u) / right_denom * N_ip(u, i + 1, p - 1, U)

    return left + right


def plot_basis(p, U, ax):
    """Plot all basis functions of degree p on given axis."""
    n = len(U) - p - 1  # number of basis functions
    u_vals = np.linspace(U[p], U[-p-1], 400)

    for i in range(n):
        vals = N_ip(u_vals, i, p, U)
        ax.plot(u_vals, vals, label=f"N_{{{i},{p}}}(u)")

    for uk in U:
        ax.axvline(uk, color="gray", linestyle="--", linewidth=0.5)

    ax.set_title(f"B-spline Basis (degree p = {p})")
    ax.set_xlabel("u")
    ax.set_ylabel(f"N_{{i,{p}}}(u)")
    ax.grid(True, alpha=0.3)
    ax.legend()


fig, axes = plt.subplots(3, 1, figsize=(8, 10), sharex=True)
plot_basis(1, U, axes[0])  # linear
plot_basis(2, U, axes[1])  # quadratic
plot_basis(3, U, axes[2])  # cubic
plt.tight_layout()
plt.show()
```

![Quadratic B‑spline basis](/image/bspline_p3.png)


## 📘 이 스크립트가 보여주는 핵심 비교
- ✔ p = 1 (linear)
    - 각 basis는 삼각형 모양
    - C⁰ 연속
    - support 길이 짧음
- ✔ p = 2 (quadratic)
    - 각 basis는 포물선 모양
    - C¹ 연속
    - support가 더 넓어짐
- ✔ p = 3 (cubic)
    - 각 basis는 매끄러운 종 모양
    - C² 연속
    - support가 더 넓고 부드러움
    - 곡선 품질이 가장 좋음

## 📘 이 스크립트의 장점
- 같은 knot vector에서 degree만 바꿔서 비교
- basis가 degree에 따라 어떻게 변하는지 직관적으로 확인
- knot 중복 때문에 어떤 basis가 죽는지 한눈에 보임
- B‑spline의 핵심 개념인
    - local support
    - continuity
    - smoothness
    - partition of unity
- 를 시각적으로 이해 가능

- $N_{0,2}(u)$ 는 [0,1] 구간에서만 살아있고
- N_${1,2}(u)$ 는 [0,2]
- $N_{2,2}(u)$ 는 [0,3] (가운데 봉우리)
- $N_{3,2}(u)$ 는 [1,3]
- $N_{4,2}(u)$ 는 [2,3]


## 📘 B‑spline Basis의 Support(지지구간) — 수식 정리
### 1) 0차 basis의 support
- 0차 B‑spline basis는 단순한 indicator function:
```math
N_{i,0}(u) =
\begin{cases}
1, & \text{if } U_i \le u < U_{i+1} \\
0, & \text{otherwise}
\end{cases}
``` 
- 따라서 support는:
```math
supp(N_{i,0}) = [U_i, U_{i+1})
```
### 2) 재귀식 (Cox–de Boor)
```math
N_{i,p}(u)=\frac{u-U_i}{U_{i+p}-U_i}\, N_{i,p-1}(u)+\frac{U_{i+p+1}-u}{U_{i+p+1}-U_{i+1}}\, N_{i+1,p-1}(u)
(분모가 0이면 해당 항은 0으로 정의)
```
### 3) Support inclusion 성질
- 위 재귀식에서 $N_{i,p}(u)$ 가 0이 아닌 경우는
    두 항 중 하나라도 0이 아닌 경우뿐이다.
- 즉:
```math
supp(N_{i,p}) ⊆ supp(N_{i,p-1}) ∪ supp(N_{i+1,p-1})
```
- 이 포함 관계를 p번 반복하면:
```math
supp(N_{i,p}) ⊆ [U_i, U_{i+p+1})
```
- 경계에서 0이 되는 경우만 제외하면 사실상 equality로 봐도 된다.

### 📘 4) 너의 knot vector에 적용
- knot:
```math
U=[0,0,0,1,2,3,3,3],\quad p=2
```
- 여기서 i=0일 때:
```math
U_0=0,\quad U_{0+2+1}=U_3=1
```
- 따라서 support 정리에 의해:
```math
supp(N_{0,2}) = [0, 1)
```
- 즉:
    - $u<0 → 0$
    - $u\geq 1 → 0$
    - 오직 [0,1)에서만 nonzero

### 📘 5) 실제 식으로도 확인 가능
- 이 knot에서는 첫 span 길이가 1이므로
- $N_{0,2}(u)$ 는 사실상 “첫 Bernstein” 형태가 된다.
- 직접 전개하면:
```math
N_{0,2}(u)=\left\{ \, \begin{array}{ll}\textstyle (1-u)^2,&\textstyle 0\leq u<1\\ \textstyle 0,&\textstyle \mathrm{otherwise}\end{array}\right. 
```
- 즉, support 정리와 실제 계산이 완전히 일치한다.

### 📘 6) 요약
- 0차 basis의 support는 $[U_i,U_{i+1})$
- 재귀식으로 인해 support는 점점 넓어지지만
- 최종 support는 항상 $[U_i,U_{i+p+1})$ 로 제한됨
- knot에서는
```math
supp(N_{0,2}) = [0,1)
```
- 실제 전개한 식도
```math
N_{0,2}(u)=(1-u)^2
```
- 로 동일한 support를 가진다.

- 이제 진짜로 Cox–de Boor 재귀식으로부터
- $N_{0,2}(u)=(1-u)^2\quad (0\leq u<1)$
- 가 어떻게 나오는지 단계별로 유도.

#### 0. 설정 다시 확인
- knot vector:
```
U=[0,0,0,1,2,3,3,3]
```
- degree:
p=2
- 우리가 구하고 싶은 것:
```math
N_{0,2}(u)
```
- 유효 구간은 0,3) 이지만,
- support 정리]:
```math
supp(N_{0,2}) = [U_0, U_{0+2+1}) = [0,1)
```
- 그래서 실제로는 0 ≤ u < 1 에서만 값이 의미 있고,
- 그 구간에서만 계산하면 된다.

#### 1단계: 0차 basis $N_{i,0}(u)$
- 정의:
```math
N_{i,0}(u) = {
  1,  if U_i ≤ u < U_{i+1}
  0,  otherwise
}
```
- 우리 knot에서:
    - $U_0=0,U_1=0$ → [0,0): 항상 0
    - $U_1=0,U_2=0$ → [0,0): 항상 0
    - $U_2=0,U_3=1$ → [0,1): 여기서만 1
- 따라서 0 ≤ u < 1 에서:
    - $N_{0,0}(u)=0$
    - $N_{1,0}(u)=0$
    - $N_{2,0}(u)=1$

#### 2단계: 1차 basis $N_{0,1}(u), N_{1,1}(u)$
- 재귀식 (p=1):
```math
N_{i,1}(u)=\frac{u-U_i}{U_{i+1}-U_i}N_{i,0}(u)+\frac{U_{i+2}-u}{U_{i+2}-U_{i+1}}N_{i+1,0}(u)
```
##### (1) $N_{0,1}(u)$
- $i=0, U_0=0,U_1=0,U_2=0$
- 분모들:
    - $U_1-U_0=0-0=0$ → 첫 항은 0
    - $U_2-U_1=0-0=0$ → 두 번째 항도 0
- 따라서:
```math
N_{0,1}(u)=0\quad (\forall u)
````
##### (2) $N_{1,1}(u)$

- $i=1, U_1=0,U_2=0,U_3=1$
- 분모들:
- $U_2-U_1=0-0=0$ → 첫 항 0
- $U_3-U_2=1-0=1$ → 두 번째 항만 남음
- 따라서:
```math
N_{1,1}(u)=\frac{U_3-u}{U_3-U_2}N_{2,0}(u)=(1-u)\, N_{2,0}(u)
```
- 그리고 0 ≤ u < 1 에서 $N_{2,0}(u)=1$ 이므로:
```math
N_{1,1}(u)=1-u\quad (0\leq u<1)
```
### 3단계: 2차 basis $N_{0,2}(u)$
- 이제 진짜 목표.
- 재귀식 (p=2):
```math
N_{i,2}(u)=\frac{u-U_i}{U_{i+2}-U_i}N_{i,1}(u)+\frac{U_{i+3}-u}{U_{i+3}-U_{i+1}}N_{i+1,1}(u)
```
- 여기서 i=0:
    - $U_0=0$
    - $U_2=0$
    - $U_3=1$
    - $U_1=0$
- 분모들:
- $U_{0+2}-U_0=U_2-U_0=0-0=0$ → 첫 항 0
- $U_{0+3}-U_1=U_3-U_1=1-0=1$ → 두 번째 항만 남음
- 따라서:
```math
N_{0,2}(u)=\frac{U_3-u}{U_3-U_1}N_{1,1}(u)=(1-u)\, N_{1,1}(u)
```
- 그리고 위에서 구한 대로:
```math
N_{1,1}(u)=1-u\quad (0\leq u<1)
```
- 이걸 대입하면:
```math
N_{0,2}(u)=(1-u)\, (1-u)=(1-u)^2\quad (0\leq u<1)
```
- 그리고 support 밖에서는 0이므로 최종적으로:
```math
N_{0,2}(u)=\left\{ \, \begin{array}{ll}\textstyle (1-u)^2,&\textstyle 0\leq u<1\\ \textstyle 0,&\textstyle \mathrm{otherwise}\end{array}\right. 
```

## 0. 기본 설정 정리
- knot vector
U=[0,0,0,1,2,3,3,3]
- degree
p=2
- Cox–de Boor 재귀식
```math
N_{i,p}(u)=\alpha _{i,p}(u)\, N_{i,p-1}(u)+\beta _{i,p}(u)\, N_{i+1,p-1}(u)
```
- 여기서
```math
\alpha _{i,p}(u)=\frac{u-U_i}{U_{i+p}-U_i},\quad \beta _{i,p}(u)=\frac{U_{i+p+1}-u}{U_{i+p+1}-U_{i+1}}
``` 
- (분모가 0이면 해당 항은 0으로 취급)

- 핵심은: $\alpha$, $\beta$ 는 단지 “스칼라 함수”이고,
- 곱해지는 $N_{i,p-1},N_{i+1,p-1}$ 가 0이면 결과도 0이다.
- 그래서 support는 항상
```math
supp(N_{i,p}) ⊆ supp(N_{i,p-1}) ∪ supp(N_{i+1,p-1})
```
- 이 구조를 계속 쓰면 된다.
## 1. p = 0 (indicator)에서 support 확정정의:
```math
N_{i,0}(u) = {
  1,  if U_i ≤ u < U_{i+1}
  0,  otherwise
}
```
- 각 i에 대해:
    - $N_{0,0}$: [U_0,U_1)=[0,0) → 빈 구간 → 사실상 0
    - $N_{1,0}$: [U_1,U_2)=[0,0) → 0
    - $N_{2,0}$: [U_2,U_3)=[0,1)
    - $N_{3,0}$: [U_3,U_4)=[1,2)
    - $N_{4,0}$: [U_4,U_5)=[2,3)
    - $N_{5,0}$: [U_5,U_6)=[3,3) → 0
- 따라서 “살아 있는” 0차 basis의 support는:
```math
supp(N_{2,0}) = [0, 1),
```
```math
supp(N_{3,0}) = [1, 2),
```
```math
supp(N_{4,0}) = [2, 3)
``` 
## 2. p = 1에서 support를 union으로 얻기재귀:
```math
N_{i,1}(u)=\alpha _{i,1}(u)\, N_{i,0}(u)+\beta _{i,1}(u)\, N_{i+1,0}(u)
```
- 따라서
```math
supp(N_{i,1}) ⊆ supp(N_{i,0}) ∪ 
```
```math 
supp(N_{i+1,0})
```
#### 2.1 $N_{1,1}$
```math
supp(N_{1,1}) ⊆ supp(N_{1,0}) ∪ supp(N_{2,0})
             = ∅ ∪ [0,1)
             = [0,1)
```
실제로도 그 구간에서만 nonzero이므로:
supp(N_{1,1}) = [0, 1)
#### 2.2 $N_{2,1}$
```math
supp(N_{2,1}) ⊆ supp(N_{2,0}) ∪ supp(N_{3,0})
             = [0,1) ∪ [1,2)
             = [0,2)
```
따라서:
```math
supp(N_{2,1}) = [0, 2)
```
#### 2.3 N_{3,1}
```math
supp(N_{3,1}) ⊆ supp(N_{3,0}) ∪ supp(N_{4,0})
             = [1,2) ∪ [2,3)
             = [1,3)
```
- 따라서:
```math
supp(N_{3,1}) = [1, 3)   (경계에서 값이 0인 점들은 있지만, support 구간은 이렇게 잡는 것이 표준적.)
```
### 3. p = 2에서 support (핵심)
- 이제:
```math
N_{i,2}(u)=\alpha _{i,2}(u)\, N_{i,1}(u)+\beta _{i,2}(u)\, N_{i+1,1}(u)
```
- 따라서:
```math
supp(N_{1,2}) ⊆ supp(N_{1,1}) ∪ supp(N_{2,1})
             = [0,1) ∪ [0,2)
             = [0,2)
```
#### 3.1 $N_{1,2}$ 의 support
```math
supp(N_{1,2}) ⊆ supp(N_{1,1}) ∪ supp(N_{2,1})
             = [0,1) ∪ [0,2)
             = [0,2)
````
실제로 이 구간에서만 nonzero이므로:
```math
supp(N_{1,2}) = [0, 2)
```
- 또한 “정리 형태”로도:
```math
[U_1,U_{1+p+1})=[U_1,U_4)=[0,2)
```
- 와 일치한다.
#### 3.2 $N_{2,2}$ 의 support
```math
supp(N_{2,2}) ⊆ supp(N_{2,1}) ∪ supp(N_{3,1})
             = [0,2) ∪ [1,3)
             = [0,3)
```
- 따라서:
```math
supp(N_{2,2}) = [0, 3)
```
- 정리 형태로도:
```math
[U_2,U_{2+p+1})=[U_2,U_5)=[0,3)
```
- 와 정확히 일치.
### 4. 왜 항상 $[U_i,U_{i+p+1})$ 가 되는가? 
- 재귀 구조를 보면:
- $N_{i,p}$ 는 항상 $N_{i,p-1}$ 와 $N_{i+1,p-1}$ 의 선형 결합
- support는 매 단계마다 왼쪽은 그대로 $U_i$, 오른쪽은 한 칸씩 더 멀리 $U_{i+p+1}$ 까지 확장
- 그래서 귀납적으로:
```math
supp(N_{i,p}) = [U_i, U_{i+p+1})
```
- 이 된다.
- 중복 knot가 있으면 그 안에서 **일부 span에서 값이 0** 인 구간이 생길 수는 있지만, 
- support의 전체 경계는 항상 이 형태를 따른다.

---

