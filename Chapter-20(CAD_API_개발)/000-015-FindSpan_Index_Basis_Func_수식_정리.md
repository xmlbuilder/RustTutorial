## 🎯 왜 FindSpan(i)에서 p만큼 뒤로 가는가?
- ✔ 이유 1: B‑spline basis는 국소 지지(local support)를 가진다
  - p차 B‑spline basis $N_{j,p}(u)$ 는 다음 구간에서만 0이 아니다:
  - $N_{j,p}(u)\neq 0\quad \mathrm{iff}\quad u\in [U_j,\; U_{j+p+1})$
  - 즉, 하나의 basis function은 p+1개의 knot span에 걸쳐서만 살아 있다.

-  ✔ 이유 2: FindSpan(i)는 “u가 속한 구간”을 알려준다
  - FindSpan이 반환한 i는:
```math
u\in [U_i,\; U_{i+1})
```
- 즉, u가 이 구간에 있다는 뜻.

- ✔ 이유 3: 이 구간에서 non‑zero인 basis는 정확히 p+1개
  - 그 p+1개의 basis의 인덱스는:
```math
i-p,\; i-p+1,\; \dots ,\; i
```
- 왜냐하면:
  - $N_{i,p}$ 는 $[U_i,U_{i+p+1})$ 에서 non‑zero
  - $N_{i-1,p}$ 는 $[U_{i-1},U_{i+p})$ 에서 non‑zero
  - $N_{i-2,p}$ 는 $[U_{i-2},U_{i+p-1})$ 에서 non‑zero
  - …
  - $N_{i-p,p}$ 는 $[U_{i-p},U_{i+1})$ 에서 non‑zero
  - 즉, u가 $U_i, U_{i+1})$ 에 있으면, 이 p+1개만 non‑zero가 된다.

- ✔ 이유 4: 그래서 basis function 계산은 항상 i‑p부터 시작한다
  - B‑spline은 국소 지지이기 때문에
  - 전체 basis function을 계산할 필요가 없다.
  - FindSpan(i)가 나오면:
```math
\mathrm{non-zero\  basis\  index}=i-p\sim i
```
- 이게 바로 basis 계산의 시작점이 된다.

### 📌 예시로 완전히 이해해보자
- Knot vector:
```math
U=\{ 0,0,0,1,5,6,8,8,8\}
``` 
- degree:
    - p=3
- u = 5.2

#### Step 1: FindSpan(u)
- 5.2는 5,6) 구간 → i = 4
#### Step 2: non-zero basis index
- i-p=4-3=1
- 따라서 non‑zero basis는:
```math
N_{1,3},\; N_{2,3},\; N_{3,3},\; N_{4,3}
```
- 즉, FindSpan이 i를 주면, basis 계산은 i‑p부터 i까지다.

## 🎯 핵심 요약
- ✔ FindSpan(i)는 “u가 속한 knot span”을 알려준다
- ✔ p차 B‑spline은 p+1개의 basis만 non‑zero
- ✔ 그 basis들의 index는 항상 i‑p부터 i까지
- ✔ 그래서 basis 계산은 반드시 i‑p에서 시작한다
- 즉,
    - ⭐ **FindSpan(i) → basis index 범위 = [i‑p .. i]**
    - 이 규칙은 B‑spline의 국소 지지 성질 때문에 절대적으로 고정된 구조.

---

## 🎯 왜 j+p+1 때문에 헷갈렸는가?
- 우리가 알고 있는 B‑spline의 지지구간 공식은:
```math
N_{j,p}(u)\neq 0\quad \mathrm{iff}\quad u\in [U_j,\; U_{j+p+1})
```
- 여기서 중요한 건:
    - basis function $N_{j,p}$ 는 j에서 시작해서 p+1개의 knot span 동안만 non‑zero
    - 즉, 지지구간 길이가 p+1
- 이걸 보면 “앞으로 p+1만큼 가야 하는 것 아닌가?” 하고 착각하기 쉬워.
- 하지만 실제 basis function 계산에서는 반대로 p만큼 뒤로 가야 한다.

### ⭐ FindSpan(i)는 “u가 속한 구간의 오른쪽 끝”을 기준으로 잡기 때문이다
- FindSpan이 반환한 i는:
```math
u\in [U_i,\; U_{i+1})
```
- 즉, u는 $U_i$ 와 $U_{i+1}$ 사이에 있다.
- 이때 non‑zero basis는:
```math
N_{i-p,p},\; N_{i-p+1,p},\; \dots ,\; N_{i,p}
```

### 📌 지지구간을 기준으로 보면 완전히 이해된다
#### 1) N_{i,p}의 지지구간:
```math
[U_i,\; U_{i+p+1})
```
- u는 $U_i, U_{i+1})$ 에 있으므로
    - $N_{i,p}$ 는 non‑zero

#### 2) N_{i-1,p}의 지지구간:
```math
[U_{i-1},\; U_{i+p})
```
- u는 여전히 이 구간 안에 있음
    - N_{i-1,p}도 non‑zero

#### 3) N_{i-2,p}의 지지구간:
```math
[U_{i-2},\; U_{i+p-1})
```
- u는 여전히 이 구간 안에 있음
    - non‑zero

#### 4) … 반복하면 마지막으로 non‑zero가 되는 것은:
```math
N_{i-p,p}
```
- 왜냐하면:
```math
[U_{i-p},\; U_{i+1})
```
- u는 $U_i, U_{i+1})$ 에 있으므로
    - 여기까지는 non‑zero

### 🔥 그런데 N_{i+1,p}는 왜 zero인가?
```math
N_{i+1,p}\neq 0\quad \mathrm{iff}\quad u\in [U_{i+1},\; U_{i+p+2})
```
- 하지만 u는:
```math
u<U_{i+1}
```
- 즉, u는 이 지지구간에 들어가지 않음
    - $N_{i+1,p}(u) = 0$
- 같은 이유로:
    - $N_{i+2,p}$ = 0
    - $N_{i+3,p}$ = 0
    - …
    - $N_{i+p,p}$ = 0
- 전부 0이 된다.

## 🎯 그래서 결론은?
- ✔ 지지구간은 j에서 시작해서 p+1 span
- ✔ 하지만 FindSpan(i)는 u가 $U_i, U_{i+1})$ 에 있다는 뜻
- ✔ 이 구간에서 non‑zero가 되는 basis는 $i‑p$ 부터 $i$ 까지
- ✔ i+1부터 i+p까지는 모두 zero
- 즉,
    - ⭐ 지지구간은 앞으로 p+1이지만, basis index는 뒤로 p만큼 가야 한다
- 지지구간은 p+1이라서 앞으로 가야 할 것 같지만,  
    u가 속한 구간을 기준으로 보면 basis index는 p만큼 뒤로 가야 한다.

---

## ✅ B‑spline FindSpan / BasisFuns 관계 완전 정리

### 1️⃣ FindSpan(i)의 의미
- FindSpan(u)가 반환하는 i는:
```math
U_i\leq u<U_{i+1}
```
- 즉, u가 속한 knot span의 index이다.
- 그리고 이 구간에서 0이 아닌 basis 함수는 정확히 p+1개이며:
```math
N_{i-p,p}(u),\; N_{i-p+1,p}(u),\; \dots ,\; N_{i,p}(u)
```
- 이 p+1개만 살아 있다.
- ✔ 이건 100% 맞는 이해다.

### 2️⃣ 그런데 BasisFuns에서는 왜 i, i+1, i+j가 보일까?
- 이게 헷갈리는 핵심 포인트.
- ✔ BasisFuns는 “전체 basis를 계산하는 함수”가 아니다  
    이미 non‑zero basis만 계산하는 함수다.
- 즉, 입력 i는:
    - basis index가 아니라
    - span index
- 그리고 출력 N[j]는 실제로는:
```math
N[j]=N_{i-p+j,p}(u)
```
- 즉, basis index는 i-p부터 시작하지만, 배열 index는 0부터 시작한다.

### 3️⃣ 코드 안의 i+j는 basis index가 아니다
- A2.2의 핵심 부분:
```math
left[j]  = u - U[i+1-j]
right[j] = U[i+j] - u
```

- 여기서 U[i+j] 때문에 **앞으로 p만큼 가는 것처럼** 보이지만,  
    이건 basis index가 아니라 knot index 접근이다.
- 즉:
    - basis index는 i‑p … i
    - knot index는 i‑p … i+p+1
- 둘은 다르다.

### 4️⃣ 왜 knot은 i+p+1까지 접근하는가?
- p차 B‑spline의 정의:
```math
N_{k,p}(u)\mathrm{는\  }U_k\sim U_{k+p+1}\mathrm{까지\  영향을\  받는다}
```
- 지금 계산하는 가장 오른쪽 basis는:
```math
N_{i,p}(u)
```
- 이 함수는 다음 knot까지 영향을 받는다:
```math
U_i\sim U_{i+p+1}
```
- 따라서 A2.2는 계산을 위해 U[i+p+1]까지 접근해야 한다.
    - ✔ 하지만 이것은 basis index가 i+p까지 있다는 뜻이 아니다.
    - ✔ 단지 basis 계산에 필요한 knot 범위일 뿐이다.

### 5️⃣ 핵심 정리
- ✔ FindSpan(i)는 span index
- ✔ non‑zero basis index는 i‑p … i
- ✔ BasisFuns의 N[j]는 실제로 $N_{i-p+j,p}(u)$
- ✔ 코드에서 i+j가 등장하는 이유는
    - basis index가 아니라
    - 해당 basis를 계산하기 위한 knot 접근 범위이기 때문

### 6️⃣ 관점 차이 때문에 헷갈린다
- 가장 중요한 문장:
    - FindSpan → basis index 관점
    - BasisFuns 내부 → knot 접근 관점
- 관점이 다르다.
    - ✔ FindSpan은 **어떤 basis가 살아 있는가?**
    - ✔ BasisFuns는 **그 basis를 계산하려면 어떤 knot이 필요한가?**
- 이 두 관점이 섞이면 헷갈리는 게 당연하다.

## 🔥 한 줄로 요약
- i는 span index이고, 살아 있는 basis는 i‑p … i.
- i+p+1은 basis index가 아니라 basis 계산을 위한 knot 접근 범위일 뿐이다.

---

# 시각화

## 🎨 1) Knot vector와 FindSpan(i)
- 예시로 p=3 (cubic)이라고 하자.
- knot vector U:
```
|---U0---|---U1---|---U2---|---U3---|---U4---|---U5---|---U6---|---U7---|

u가 여기에 있음
                     ↓
                   [ U_i , U_{i+1} )
```

- FindSpan(u) = i
    - u는 U_i와 U_{i+1} 사이에 있다.

## 🎨 2) 이 구간에서 non‑zero인 basis index
- p=3이면 non‑zero basis는 4개:
- basis index j:
```
i-3     i-2     i-1     i
 |       |       |       |
 N_{i-3,3}  N_{i-2,3}  N_{i-1,3}  N_{i,3}   ← 살아 있음
```

- 즉,
    - non-zero basis index = [i-p ... i]

## 🎨 3) 각 basis의 지지구간(support)
- 각 basis N_{j,p}는 다음 구간에서만 non-zero:
```math
N_{j,p}(u) ≠ 0  iff  u ∈ [ U_j , U_{j+p+1} )
```

- 이를 그림으로 보면:
- $N_{i,3}$      : $[ U_i     ... U_{i+4} )$
- $N_{i-1,3}$    : $[ U_{i-1} ... U_{i+3} )$
- $N_{i-2,3}$    : $[ U_{i-2} ... U_{i+2} )$
- $N_{i-3,3}$    : $[ U_{i-3} ... U_{i+1} )$


- u는 $U_i, U_{i+1})$ 에 있으므로
    - 위 4개만 non-zero
    - i+1, i+2, i+3, i+4는 모두 zero

## 🎨 4) BasisFuns가 왜 i+j, i+1−j를 쓰는가?
- BasisFuns는 basis index를 계산하는 게 아니라,
b- asis 계산에 필요한 knot을 접근하는 것이다.
- 그래서 다음과 같은 knot 접근이 필요하다:
```rust
left[j]  = u - U[i+1-j]
right[j] = U[i+j] - u
```

- 이걸 그림으로 보면:
- knot access range needed:
```
U[i-p] ... U[i] ... U[i+1] ... U[i+p+1]
```

- 즉,
    - basis index는 i-p … i
    - knot index는 i-p … i+p+1
- 둘은 다르다.

## 🎨 5) 전체 흐름을 하나의 그림으로
```
KNOT VECTOR
 ... U[i-3] -- U[i-2] -- U[i-1] -- U[i] -- U[i+1] -- U[i+2] -- U[i+3] -- U[i+4] ...

                              u ∈ [U[i], U[i+1])
```
```
NON-ZERO BASIS INDEX
   i-3        i-2        i-1        i
    |          |          |          |
    +----------+----------+----------+
                p+1개 살아 있음
```
```
KNOTS NEEDED TO COMPUTE THESE BASIS
U[i-3] ... U[i] ... U[i+1] ... U[i+4]
 ^----------------------------------^
         p+1 span (support)
```
- 즉:
    - basis index는 뒤로 p만큼 간다
    - knot index는 앞으로 p+1까지 간다
    - 둘이 다르기 때문에 헷갈렸던 것

## 🎯 최종 요약
- ✔ FindSpan(i)는 span index
- ✔ non‑zero basis index = i‑p … i
- ✔ BasisFuns의 N[j] = N_{i-p+j,p}(u)
- ✔ 코드에서 i+j가 보이는 이유는
    - basis index가 아니라
    - basis 계산에 필요한 knot 접근 때문

---
