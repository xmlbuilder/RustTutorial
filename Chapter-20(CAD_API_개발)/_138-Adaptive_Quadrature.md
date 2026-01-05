

# Adaptive Quadrature 

- 이 알고리즘은 대칭 필터(Symmetric Filter) 기반의 Adaptive Quadrature다.
- 즉, Wavelet + FFT-like 변환을 이용해서 적분 가중치를 만드는 방식이다.
- 이 방식은 일반적인 Simpson, Gauss-Legendre보다:
  - 더 빠르고
  - 더 안정적이고
  - 고차 NURBS에서도 정확도가 높다

## 📘 전체 알고리즘의 핵심 아이디어 (수식 기반)
### 1) 적분 문제 정의
- 우리가 구하고 싶은 것은:
```math
I=\int _{u_0}^{u_1}f(u)\, du
```
- OpenNURBS에서는:
```math
f(u)=\| C'(u)\|
``` 
- 즉, 곡선 길이 적분.

### 2) 필터 기반 적분 공식
- 다음 형태의 적분 공식을 사용한다:
```math
I\approx (u_1-u_0)\sum _{k=0}^Nw_k\, f(u_k)
```
- 여기서:
  - $w_k$ = 필터 계수 (filter coefficients)
  - $u_k$ = 샘플링 위치
- 즉, 가중치 기반 적분이다.
- 문제는 이 $w_k$ 를 어떻게 구하느냐인데, 대칭 필터(symmetric filter) 를 사용한다.

### 📘 3) 필터 계수 생성의 수학적 의미
- 필터 계수 생성은 다음 과정을 따른다:
#### (1) 초기 half-symmetric weight
```math
h_i=\frac{1}{2^{s+1}(1-4i^2)}
```
- 여기서 s는 스케일 단계.
#### (2) 대칭 변환 (Symmetric Transform)
- 이 변환은 다음 형태의 선형 변환이다:
```math
\begin{aligned}x_i'&=x_i-x_{n-i}\\ x_{n-i}'&=x_i+x_{n-i}\end{aligned}
```
- 이건 Discrete Cosine Transform(DCT) 와 매우 유사한 구조다.
#### (3) 파라미터 업데이트
- 다음 recurrence를 사용한다:
```math
A_{k+1}=\sqrt{2+A_k}
```
```math
B_{k+1}=\frac{B_k}{A_{k+1}}
```
```math
\mathrm{scale_{\mathnormal{k+1}}}=\frac{\mathrm{scale_{\mathnormal{k}}}}{2+A_k}
```
- 이 recurrence는 Chebyshev 다항식 기반 필터 안정화와 동일하다.
#### (4) 필터 계수 재배열 (Rearrangement)
- 이건 FFT의 bit-reversal permutation과 동일한 구조다.
- 즉:
  - 필터 계수는 대칭 구조를 가지며
  - FFT-like 방식으로 재배열된다
  - 최종적으로 적분 가중치 $w_k$ 가 된다

### 📘 4) 최종 적분 공식
- 최종 적분은 다음과 같다:
```math
I\approx (u_1-u_0)\sum _{k=0}^Nw_kf(u_k)
```
- 여기서:
  - $w_k$ = 필터 계수
  - $u_k$ = 필터가 지정한 샘플링 위치
- 샘플링 위치는 다음과 같이 생성된다:
```math
u_k=u_0+(u_1-u_0)\cdot t_k
```
- 여기서 $t_k$ 는 필터 내부에서 생성되는 대칭 샘플링 위치.

### 📘 5) Adaptive Refinement (오차 제어)
- 다음 조건을 만족할 때까지 refinement 한다:
```math
|I_n-I_{n-1}|<\epsilon
``` 
- 또는
```math
\mathrm{error\  estimate}<\mathrm{abs\_ tol}
```
- 이 과정에서:
  - 샘플링 개수 n이 2배씩 증가
  - 필터 계수는 미리 생성되어 있으므로 빠름
  - 오차가 충분히 작아지면 종료

### 📘 6) 전체 알고리즘 요약 (수식 기반)
- 필터 계수 $w_k$ 생성
  - 대칭 필터 기반
  - FFT-like 변환
  - Chebyshev recurrence
- 초기 샘플링
```math
f(u_0),f(u_1),f(u_0+h),f(u_1-h),f(u_0+h/2)
```
- 적분 근사
```math
I_0=(u_1-u_0)\sum w_kf(u_k)
```
- refinement
- 샘플링 개수 2배 증가
- 새로운 샘플링 위치 추가
- 오차 추정
- 반복
- 종료
```math
|I_n-I_{n-1}|<\epsilon
``` 

### 📘 7) 이 알고리즘이 왜 강력한가?
- 고차 NURBS에서도 안정적
- 곡률 급변 구간에서 자동 refinement
- 필터 계수는 미리 생성되므로 빠름
- FFT-like 구조라 계산량이 적음
- Simpson/Gauss보다 오차 제어가 뛰어남



