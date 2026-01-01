


# 📘 Bezier Surface Dot Product
- bezier_surface_dot_product_with_matrices
    - Mathematical Definition, Meaning, and Applications

## 1. Overview
- bezier_surface_dot_product_with_matrices 는 두 개의 Bezier 곡면
```math
P(u,v),\quad Q(u,v)
```
- 의 dot product surface 를 계산하여 다시 Bezier basis로 표현한 스칼라 Bezier surface를 생성하는 함수이다.
- 이 연산은 NURBS/Bezier 기반 CAD 커널에서 매우 중요한 기본 연산이며,  
    법선(normal), 곡률(curvature), implicitization, FEM 등 다양한 곳에서 사용된다.

## 2. Mathematical Definition
### 2.1 Non‑rational Bezier Surfaces
- 두 Bezier surface가 다음과 같이 주어졌다고 하자:
```math
P(u,v)=\sum _{i=0}^p\sum _{j=0}^qP_{i,j}B_{i,p}(u)B_{j,q}(v)
```
```math
Q(u,v)=\sum _{k=0}^r\sum _{l=0}^sQ_{k,l}B_{k,r}(u)B_{l,s}(v)
```
- dot product:
```math
S(u,v)=P(u,v)\cdot Q(u,v)
```
- 이 함수는 다시 Bezier basis로 표현될 수 있다:
```math
S(u,v)=\sum _{i=0}^{p+r}\sum _{j=0}^{q+s}\mathrm{num_{\mathnormal{i,j}}}\, B_{i,p+r}(u)B_{j,q+s}(v)
```
- 여기서 numerator 계수는 다음과 같다:


### 2.2 Rational Bezier Surfaces
- 합리 Bezier surface는 다음과 같이 표현된다:
```math
P=\frac{N_1}{D_1},\quad Q=\frac{N_2}{D_2}
```
- dot product:
```math
P\cdot Q=\frac{N_1\cdot N_2}{D_1\cdot D_2}
```
- 따라서:
- numerator:
```math
\mathrm{num_{\mathnormal{i,j}}}=\sum U[i][k]V[j][l](N_1\cdot N_2)
```
- denominator:
```math
\mathrm{den_{\mathnormal{i,j}}}=\sum U[i][k]V[j][l](D_1\cdot D_2)
```
- 여기서
```math
D_1=w_{k,l},\quad D_2=w_{i-k,j-l}
```
- 즉:
    - dot = xyz dot
    - dow = weight product

## 3. Product Matrix (pmu, pmv)
- Bezier basis의 곱은 다시 Bezier basis로 표현할 수 있다:
```math
B_{k,p}(u)B_{i-k,r}(u)=\sum _{i=0}^{p+r}U[i][k]B_{i,p+r}(u)
```
- 여기서 product matrix는 다음과 같다:
```math
U[i][k]=\frac{{p \choose k}{r \choose i-k}}{{p+r \choose i}}
```
- pmu: u 방향 product matrix
- pmv: v 방향 product matrix
- 이 행렬은 Bezier basis product → Bezier basis 변환을 수행하는 핵심 요소이다.

## 4. Why pmu / pmv Are Necessary
- Bezier basis는 단순한 다항식이 아니며,
- 두 Bernstein basis의 곱은 다시 Bernstein basis의 선형 결합으로 표현된다.
- 즉:
```math
B_p(u)\cdot B_r(u)=B_{p+r}(u)\cdot pmu
```
- 이 변환을 하지 않으면:
    - dot product
    - cross product
    - function product
    - implicitization
- 같은 연산을 정확하게 수행할 수 없다.

## 5. Applications
- ✔ 1) Surface Normal Computation
- 법선:
```math
N=S_u\times S_v
```
- 그리고:
```math
\| N\| ^2=N\cdot N
```
- 여기서 dot product surface가 필요하다.

- ✔ 2) Curvature Computation
    - 곡률 텐서:
        - 1st fundamental form
        - 2nd fundamental form
    - 모두 dot product surface로 구성된다.

- ✔ 3) Implicitization
    - Bezier surface를 implicit surface로 변환할 때 항들의 곱을 모두 Bezier basis로 변환해야 한다.
    - dot product surface가 필수.

- ✔ 4) FEM / CFD
    - Finite Element Method에서 shape function product는
    - Bezier basis product로 변환해야 한다.

- ✔ 5) Rational NURBS Derivatives
    - 예:
    ```math
    S_uW-SW_u
    ```
    - 여기서 dot product + function product가 모두 필요하다.

- ✔ 6) Surface Blending (Coons, Gordon, T-spline)
    - 블렌딩 함수끼리의 곱이 필요할 때 dot product surface가 등장한다.

## 6. Product Matrix Generator
```rust
pub fn on_build_product_matrix(deg_a: usize, deg_b: usize) -> Vec<Vec<f64>> {
    let new_deg = deg_a + deg_b;
    let mut m = vec![vec![0.0; deg_a + 1]; new_deg + 1];

    for i in 0..=new_deg {
        for k in 0..=deg_a {
            m[i][k] = on_product_matrix(deg_a, deg_b, i, k);
        }
    }
    m
}
```

- on_product_matrix() 는 다음을 계산한다:
```math
M[i][k]=\frac{{p \choose k}{r \choose i-k}}{{p+r \choose i}}
```
## 7. Summary

| 항목     | 설명                                                         |
|----------|--------------------------------------------------------------|
| 기능     | 두 Bezier surface의 dot product를 Bezier basis로 변환       |
| 결과     | numerator (항상), denominator (rational일 때)               |
| 핵심 수식 | 2D convolution + dot(P,Q)                                   |
| pmu/pmv  | Bernstein basis product matrix                               |
| 용도     | 법선, 곡률, implicitization, FEM, blending 등               |


---

# 자세한 수식


## 1. 기본 설정: Bezier surface 정의
- 먼저 비유리(Non-rational) Bezier surface부터 잡고 가는 게 좋습니다.
### 1.1. 1차원 Bezier 곡선
- 차수 m인 Bezier 곡선:
```math
C(u)=\sum _{i=0}^mB_i^m(u)P_i
```
- 여기서 Bernstein basis는
```math
B_i^m(u)={m \choose i}u^i(1-u)^{m-i}
```
### 1.2. 2차원 Bezier surface
- 차수 (m,n)인 Bezier surface:
```math
S(u,v)=\sum _{i=0}^m\sum _{j=0}^nB_i^m(u)\, B_j^n(v)\, P_{ij}
```
- 여기서 $P_{ij}\in \mathbb{R^{\mathnormal{d}}}$ (보통 d=3)는 제어점입니다.

## 2. 두 Bezier surface의 dot product 구조
- 비유리 Bezier surface 두 개:
```math
S_1(u,v)=\sum _{i=0}^{m_1}\sum _{j=0}^{n_1}B_i^{m_1}(u)\, B_j^{n_1}(v)\, P_{ij}^{(1)}
```
```math
S_2(u,v)=\sum _{k=0}^{m_2}\sum _{l=0}^{n_2}B_k^{m_2}(u)\, B_l^{n_2}(v)\, P_{kl}^{(2)}
```
- 두 표면의 점별(dot) 내적은
```math
F(u,v)=S_1(u,v)\cdot S_2(u,v)
```
- 이제 이걸 **또 하나의 Bezier surface(스칼라 값)** 로 표현.

## 3. Bernstein 곱의 핵심 수식 (중요 포인트)
- 여기서 많은 구현 설명이 생략해 버리는 핵심 수식이 이겁니다.
### 3.1. 1D에서 Bernstein 곱
```math
B_i^{m_1}(u)\, B_k^{m_2}(u)={m_1 \choose i}{m_2 \choose k}u^{i+k}(1-u)^{(m_1-i)+(m_2-k)}
```
- 이는 다시 차수 m_1+m_2인 Bernstein basis 하나로 묶을 수 있습니다:
```math
B_i^{m_1}(u)\, B_k^{m_2}(u)=\frac{{m_1 \choose i}{m_2 \choose k}}{{m_1+m_2 \choose i+k}}\, B_{i+k}^{m_1+m_2}(u)
```


### 3.2. 2D에서 Bernstein 곱
- u, v 각각에 대해 위 성질을 독립적으로 사용하면
```math
B_i^{m_1}(u)B_j^{n_1}(v)\cdot B_k^{m_2}(u)B_l^{n_2}(v)=\left[ \frac{{m_1 \choose i}{m_2 \choose k}}{{m_1+m_2 \choose i+k}}B_{i+k}^{m_1+m_2}(u)\right] \cdot \left[ \frac{{n_1 \choose j}{n_2 \choose l}}{{n_1+n_2 \choose j+l}}B_{j+l}^{n_1+n_2}(v)\right]
``` 
- 따라서
```math
B_i^{m_1}(u)B_j^{n_1}(v)\cdot B_k^{m_2}(u)B_l^{n_2}(v)=\alpha _{i,k}^{(u)}\, \beta _{j,l}^{(v)}\, B_{i+k}^{m_1+m_2}(u)\, B_{j+l}^{n_1+n_2}(v)
```
- 여기서
```math
\alpha _{i,k}^{(u)}=\frac{{m_1 \choose i}{m_2 \choose k}}{{m_1+m_2 \choose i+k}},\qquad \beta _{j,l}^{(v)}=\frac{{n_1 \choose j}{n_2 \choose l}}{{n_1+n_2 \choose j+l}}
```
- 이 수식이 Bezier × Bezier = 또 다른 Bezier가 되는 핵심입니다.

## 4. Dot product를 Bezier 형태로 정리하기
- 이제 실제로 $F(u,v)=S_1(u,v)\cdot S_2(u,v)$ 를 전개해 보겠습니다.
```math
F(u,v)=\left( \sum _{i,j}B_i^{m_1}(u)B_j^{n_1}(v)P_{ij}^{(1)}\right) \cdot \left( \sum _{k,l}B_k^{m_2}(u)B_l^{n_2}(v)P_{kl}^{(2)}\right)
``` 
- 전개하면
```math
F(u,v)=\sum _{i,j}\sum _{k,l}\left( B_i^{m_1}(u)B_j^{n_1}(v)B_k^{m_2}(u)B_l^{n_2}(v)\right) \left( P_{ij}^{(1)}\cdot P_{kl}^{(2)}\right)
``` 
- 여기서 앞에서 얻은 Bernstein 곱 수식을 넣으면:
```math
F(u,v)=\sum _{i,j}\sum _{k,l}\alpha _{i,k}^{(u)}\beta _{j,l}^{(v)}B_{i+k}^{m_1+m_2}(u)B_{j+l}^{n_1+n_2}(v)\left( P_{ij}^{(1)}\cdot P_{kl}^{(2)}\right)
``` 
- 이제 인덱스를
```math
r=i+k,\quad s=j+l
```
- 로 바꾸면, 결과를 새로운 Bezier 형태로 쓸 수 있습니다:
```math
F(u,v)=\sum _{r=0}^{m_1+m_2}\sum _{s=0}^{n_1+n_2}B_r^{m_1+m_2}(u)\, B_s^{n_1+n_2}(v)\, Q_{rs}
```
- 여기서 새로운 스칼라 제어점 $Q_{rs}$ 는
```math
Q_{rs}=\sum _{i,k\, :\, i+k=r}\sum _{j,l\, :\, j+l=s}\alpha _{i,k}^{(u)}\beta _{j,l}^{(v)}\left( P_{ij}^{(1)}\cdot P_{kl}^{(2)}\right)
``` 
- 이게 바로 중간에 생략되면 다시 설명하기 빡센 핵심 수식입니다.
- 한 줄로 말하면:
    - **제어점끼리 dot product 한 뒤, 이항계수 비율(α, β)로 가중치 주면서 인덱스가 합이 일정한 쌍만 모아서 새로운 제어점 $Q_{rs}$ 를 만든다.**


## 5. Rational Bezier surface까지 확장
- 이제 rational일 때, 문서에 나왔던
```math
S_1(u,v)=\frac{N_1(u,v)}{D_1(u,v)},\qquad S_2(u,v)=\frac{N_2(u,v)}{D_2(u,v)}
```
- 이걸 단순히 “형태”만 쓴 게 아니라, 사실은 이렇게 됩니다.
- 각 표면이 homogeneous 좌표로
```math
\tilde {S}_1(u,v)=\sum _{i,j}B_i^{m_1}(u)B_j^{n_1}(v)\tilde {P}_{ij}^{(1)},\quad \tilde {P}_{ij}^{(1)}=(w_{ij}^{(1)}X_{ij}^{(1)},\; w_{ij}^{(1)})
```
```math
\tilde {S}_2(u,v)=\sum _{k,l}B_k^{m_2}(u)B_l^{n_2}(v)\tilde {P}_{kl}^{(2)},\quad \tilde {P}_{kl}^{(2)}=(w_{kl}^{(2)}X_{kl}^{(2)},\; w_{kl}^{(2)})
```
- 이때
    - 분자 $N_1(u,v)$, $N_2(u,v)$ 는 “좌표 × weight”의 Bezier surface
    - 분모 $D_1(u,v)$, $D_2(u,v)$ 는 weight만의 Bezier surface
- dot product는
```math
S_1\cdot S_2=\frac{N_1}{D_1}\cdot \frac{N_2}{D_2}=\frac{N_1\cdot N_2}{D_1D_2}
```
- 여기서
    - N_1\cdot N_2 : 위에서 설명한 Bezier × Bezier dot product (새로운 Bezier 분자)
    - D_1D_2 : 스칼라 Bezier × Bezier 곱 (새로운 Bezier 분모)
- 둘 다 **방금 설명한 “Bernstein 곱 → 차수 증가 → 새로운 제어점 Q 계산”**을 그대로 사용합니다.
- 그래서 문서에
```math
S_1S_2=\frac{N_1}{D_1}\frac{N_2}{D_2}=\frac{\mathrm{num}(u,v)}{\mathrm{den}(u,v)}
```


## 6. 정리
- 그래서 실제로 구현/설명에 필요한 핵심은 이 세 가지라고 보면 됩니다:
- Bernstein 곱 공식
```math
B_i^{m_1}(u)B_k^{m_2}(u)=\frac{{m_1 \choose i}{m_2 \choose k}}{{m_1+m_2 \choose i+k}}B_{i+k}^{m_1+m_2}(u)
```
- 2D에서 dot product 후 새로운 제어점 수식
```math
F(u,v)=\sum _{r,s}B_r^{m_1+m_2}(u)B_s^{n_1+n_2}(v)Q_{rs}Q_{rs}=\sum _{i+k=r}\sum _{j+l=s}\alpha _{i,k}^{(u)}\beta _{j,l}^{(v)}\left( P_{ij}^{(1)}\cdot P_{kl}^{(2)}\right) 
```
- Rational의 경우는 “분자와 분모 각각 위 과정을 적용한 것”



