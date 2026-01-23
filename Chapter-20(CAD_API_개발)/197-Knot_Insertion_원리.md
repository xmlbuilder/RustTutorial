# Knot Insertion (NURBS) 정리 문서

- 이 문서는 **NURBS Knot Insertion(매듭 삽입)** 을 수식, 개념, 알고리즘, 그리고 단계별 예제로 정리한 문서이다.  
- 곡선(Curve)과 곡면(Surface) 모두에 대해 **형상을 보존하면서 제어 구조만 변경** 하는 핵심 기법을 설명한다.

---

## 1. Knot Insertion이란?

- Knot Insertion은 NURBS의 **knot vector에 새로운 매듭 값을 삽입** 하는 연산이다.

- 중요한 특징은 다음과 같다.

  - **기하 형상은 변하지 않는다**
  - 제어점(control point) 개수만 증가한다
  - 국소 제어(local control)가 향상된다
  - 연속성(continuity)을 조절할 수 있다
  - Bezier 분해, Trim, Boolean, Intersection의 기초 연산이다

---

## 2. NURBS 곡선 정의

- 차수(degree) `p`인 NURBS 곡선은 다음과 같이 정의된다.

```math
C(u) = ( Σ N_{i,p}(u) w_i P_i ) / ( Σ N_{i,p}(u) w_i )
```

- $`P_i`$ : control point
- $`w_i`$ : weight
- $`N_{i,p}(u)`$ : B-spline basis function
- Knot vector: `U = [u_0, u_1, ..., u_m]`
- 관계식: `m = n + p + 1`

---

## 3. Knot multiplicity와 연속성

- 어떤 knot 값 `u`의 중복도(multiplicity)를 `s`라 하면,    
  그 knot에서의 연속성은 다음과 같다.

```math
C^{p - s}
```

- 예시 (p = 3):

| multiplicity s | continuity |
|---------------|------------|
| 1             | C²         |
| 2             | C¹         |
| 3             | C⁰         |
| 4             | 분리됨     |

---

## 4. Knot Insert의 핵심 아이디어

- 삽입할 파라미터 `u`가 들어가는 span을 찾는다
- 영향을 받는 제어점 구간만 **선형 결합(blending)** 으로 재계산
- 나머지 제어점은 그대로 복사
- Knot vector에 `u`를 하나 추가

---

## 5. Span과 multiplicity

- 삽입할 값 `u`에 대해:

```math
U[k] ≤ u < U[k+1]
```

- `k` : span index
- `s` : `u`의 현재 multiplicity
- 삽입 가능 횟수: `r ≤ p - s`

---

## 6. 단일 Knot Insertion 수식 (1회 삽입)

### 영향을 받는 제어점 범위

```
i = k - p + 1 ... k
```

### alpha 계수

```math
\alpha _i=\frac{u^*-U_i}{U_{i+p}-U_i}
```

- $\alpha _i$: 제어점 $P_i$ 와 $P_{i-1}$ 사이의 보간 계수
- $u^*$: 삽입할 knot 값
- $U_i$: 기존 knot vector의 i번째 값
- p: B-spline의 degree

### 새 제어점 계산

```math
Q_i = (1 - \alpha_i)  P_{i-1} + \alpha_i  P_i
```

- 분모가 0인 경우 alpha = 0으로 처리

---

## 7. 단계별 숫자 예제 (Curve)

### 조건

- Degree: p = 2
- Knot vector:
  ```
  U = [0,0,0, 1, 2, 3,3,3]
  ```
- Control points:
  ```
  P0=(0,0)
  P1=(1,0)
  P2=(2,1)
  P3=(3,0)
  P4=(4,0)
  ```

### 삽입할 knot (k)

```
u = 1.5
```

Span:

```
U[3] = 1 ≤ 1.5 < 2 = U[4]  → k = 3
```

Multiplicity `s = 0`

---

### 새 Knot vector

```
U' = [0,0,0, 1, 1.5, 2, 3,3,3]
```

---

### 영향 범위 (k - p + 1)

```
i = k - p + 1 ... k
  = 2 ... 3
```

---

### alpha 계산

```
alpha_2 = (1.5 - 0) / (2 - 0) = 0.75
alpha_3 = (1.5 - 1) / (3 - 1) = 0.25
```


- For i=2, p=2, $u^*=1.5$
```math
\alpha _2=\frac{1.5-U_2}{U_{2+2}-U_2}=\frac{1.5-0}{U_4-0}=\frac{1.5}{2}=0.75
```
- For i=3, p=2, $u^*=1.5$
```math
\alpha _3=\frac{1.5-U_3}{U_{3+2}-U_3}=\frac{1.5-1}{U_5-1}=\frac{0.5}{2}=0.25
```

---

### 새 Control points

```
Q2 = 0.25*P1 + 0.75*P2 = (1.75, 0.75)
Q3 = 0.75*P2 + 0.25*P3 = (2.25, 0.75)
```

#### 1. Q_2 계산
```math
Q_2=(1-\alpha _2)P_1+\alpha _2P_2
```
- 주어진 값:
- $\alpha _2=0.75$
- $P_1=(1,0)$
- $P_2=(2,1)$
- 계산:
```math
Q_2=0.25\cdot (1,0)+0.75\cdot (2,1)=(0.25,0)+(1.5,0.75)=(1.75,0.75)
```
### 2. Q_3 계산
```math
Q_3=(1-\alpha _3)P_2+\alpha _3P_3
```
- 주어진 값:
  - $\alpha _3=0.25$
  - $P_2=(2,1)$
  - $P_3=(3,0)$
- 계산:
```math
Q_3=0.75\cdot (2,1)+0.25\cdot (3,0)=(1.5,0.75)+(0.75,0)=(2.25,0.75)
```

---

### 결과 Control net

```
Q0 = P0
Q1 = P1
Q2 = (1.75, 0.75)
Q3 = (2.25, 0.75)
Q4 = P3
Q5 = P4
```

- 형상은 **완전히 동일**하다.

---

## 8. Multiple Knot Insertion

- 같은 `u`를 여러 번 삽입하면 multiplicity가 증가한다.

- 허용 삽입 횟수: `r ≤ p - s`
- 보통:
  - 반복 삽입
  - 또는 Piegl & Tiller A5.1 알고리즘 사용

---

## 9. Surface Knot Insertion
- NURBS Surface:

```
S(u,v) = ΣΣ N_{i,p}(u) M_{j,q}(v) w_{ij} P_{ij} / ΣΣ ...
```

### U 방향 삽입

- 각 v-index에 대해 **u 방향 곡선**으로 생각
- Curve Knot Insert를 모든 row에 적용
- 결과:
  - U knot 증가
  - control points (u 방향) 증가

### V 방향도 동일

---

## 10. Bezier Decomposition과의 관계
- Bezier 분해(`decompose_to_bezier_patches_no_refine_grid`)는:

  - 실제 knot refinement를 하지 않고
  - Knot Insert에서 사용되는 **alpha blending**만 사용
  - 필요한 control patch만 추출
 
즉, **Knot Insert의 부분 적용(partial application)** 이다.

---

## 11. 구현 시 주의사항 (매우 중요)

- span 계산 시 경계(u == U[n+1]) 처리
- multiplicity 계산 정확성
- 분모 0 방어
- 인덱스 underflow/overflow 방지
- Surface의 경우:
  ```
  r == n + p + 1
  s == m + q + 1
  ```
- 반드시 검증

---

## 12. 요약

- Knot Insertion은 **형상 보존 + 제어 구조 변경** 연산
- 모든 CAD 커널의 핵심 기초 연산
- Bezier 분해, Trim, Boolean, Intersection의 기반
- 구현은 단순하지만 **인덱스/범위 실수에 매우 취약**

---

