# 📘 B‑Spline 기저함수의 차원, 연속성, Knot 중복도에 관한 정리
## 1. 구간 분할 ${u_j}$ 위의 조각별 다항식 공간
- 서로 증가하는 분할점 집합
```math
\{ u_j\} ,\quad 0\leq j\leq k
```
- 이 주어졌다고 하자.
- 각 구간에서 차수 p의 조각별 다항식이며, 분할점 $u_j$ 에서
```math
C^{r_j}\quad (0\leq r_j\leq p)
```
- 연속성을 갖는 함수들의 집합을 다음과 같이 정의한다.
```math
V=\{ f:f\mathrm{는\  각\  구간에서\  차수\  }p,\; u_j\mathrm{에서\  }C^{r_j}\}
``` 
- ● 연속성 제약이 없을 때
    - 즉, 모든 $r_j=-1$ (불연속 허용)이라면:
```math
\dim (V)=k(p+1)
```
- ● 연속성 제약이 있을 때
    - 각  $C^{r_j}$ 조건은 자유도 1을 줄인다.
    - 따라서 전체 차원은:

## 2. 연속성과 Knot 중복도의 관계
- B‑spline에서 분할점 u_j에서의 연속성은 knot 중복도로 표현된다.
```math
s_j=p-r_j
```
- 즉,
    - 중복도 $s_j=1$ → $C^{p-1}$
    - 중복도 $s_j=p$ → $C^0$
    - 중복도 $s_j=p+1$ → 불연속
- ● Knot vector 구성
```math
U=\{ u_0,\dots ,u_0\mathrm{\  (}s_0\mathrm{번)},\; \dots ,\; u_k,\dots ,u_k\mathrm{\  (}s_k\mathrm{번)}\}
``` 
- 그리고
```math
m=\left( \sum _{j=0}^ks_j\right) -1
```
## 3. 기저함수 개수 계산
- 0차 함수 개수: m
- 1차 함수 개수: m-1
- …
- p-차 함수 개수:
```math
n=m-p
```
- 이제 Eq. (2.12)에 $s_j=p-r_j$ 를 대입하면:
```math
\begin{aligned}\dim (V)&=k(p+1)-\sum _{j=0}^k(p-s_j+1)\\ &=-p-1+\sum _{j=0}^ks_j\\ &=n\end{aligned}
```
- 즉,
    - ✔ p차 B‑spline 기저함수의 개수 = 공간 V의 차원

## 4. B‑spline 기저함수의 선형 독립성
- 증명은 차수 p에 대한 귀납법으로 진행된다.
- ● 가정
```math
\sum _{i=0}^na_iN_{i,p}(u)=0\quad \forall u
```
- Cox–de Boor 재귀식 적용:
```math
N_{i,p}(u)=\frac{u-u_i}{u_{i+p}-u_i}N_{i,p-1}(u)+\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(u)
```
- 이를 대입하면:
```math
0=\sum _{i=0}^na_i\frac{u-u_i}{u_{i+p}-u_i}N_{i,p-1}(u)+\sum _{i=1}^{n+1}a_{i-1}\frac{u_{i+p}-u}{u_{i+p}-u_i}N_{i,p-1}(u)
```
- 정리하면:
```math
\sum _{i=0}^n\left[ a_i\frac{u-u_i}{u_{i+p}-u_i}+a_{i-1}\frac{u_{i+p}-u}{u_{i+p}-u_i}\right] N_{i,p-1}(u)=0
\{ N_{i,p-1}\} 
```
- 이 선형 독립이므로:
```math
a_i-a_{i-1}=0\Rightarrow a_i=0
```
- 따라서:
- ✔ B‑spline 기저함수는 선형 독립이다

## 5. 클램프된(Nonperiodic) Knot Vector
- 형태:

- 특징:
- ● P2.7
```math
U=\{ 0,\dots ,0,\dots ,1,\dots ,1\}
``` 
- 이면
    - B‑spline 기저함수 = Bernstein 다항식
- ● P2.8
```math
n=m-p-1
```
- 기저함수 개수: n+1
- 양 끝에서 다음 성질을 가짐:
```math
N_{0,p}(a)=1,\quad N_{i,p}(a)=0\  (i\neq 0)
```
```math
N_{n,p}(b)=1,\quad N_{i,p}(b)=0\  (i\neq n)
```
- 이는 B‑spline의 단위 분할과 지지구간(local support) 성질에서 바로 따라온다.

## 📌 결론
- 이 문서에 포함된 수식들은 다음을 계산할 때 핵심적인 역할을 한다:
    - B‑spline 기저함수의 개수
    - 연속성 조건과 knot 중복도의 관계
    - B‑spline 공간의 차원
    - 기저함수의 선형 독립성
    - 클램프된 knot vector의 구조적 성질

---
