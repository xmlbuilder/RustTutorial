# svdcmp: Jacobi-Eigen 기반 SVD 정리

> 이 문서는 현재 구현한 `svdcmp`(Rust) 함수가 사용하는 **AᵀA 고유분해 → SVD** 절차를 수식으로 정리한 문서입니다.

---

## 표기

- 입력 행렬: $\(A \in \mathbb{R}^{m\times n}\)$  $\((m \ge n\)$ 가 일반적
- 특이값 분해(SVD):  

$$
  A = U\,\Sigma\,V^\top
$$

  where

$$
  U \in \mathbb{R}^{m\times n},\quad
  V \in \mathbb{R}^{n\times n},\quad
  \Sigma = \mathrm{diag}(\sigma_1,\dots,\sigma_n),\ \sigma_i \ge 0.
$$

- 전치: $\((\cdot)^\top\)$, 2-노름 $\(\|\cdot\|_2\)$, 프로베니우스 노름 $\(\|\cdot\|_F\)$$.

---

## 핵심 아이디어

1. 대칭 행렬 $\(B = A^\top A \in \mathbb{R}^{n\times n}\)$ 를 만든다.  
   $\(B\)$ 는 **대칭**이고 **양의 준정부호**(PSD):

 $$
   B^\top = B,\qquad x^\top B x = \|Ax\|_2^2 \ge 0.
 $$

2. $\(B\)$ 의 **고유분해**(정규직교)

 $$
   B = V\,\Lambda\,V^\top,\qquad
   \Lambda = \mathrm{diag}(\lambda_1,\dots,\lambda_n),\ \lambda_i \ge 0,\qquad
   V^\top V = I.
 $$


3. **특이값**과 **우특이벡터**:

 $$
   \sigma_i = \sqrt{\lambda_i},\qquad \text{우특이벡터} = v_i\ \ (\text{= }V\text{의 열}).
 $$


4. **좌특이벡터**:

$$
u_i =
\begin{cases}
  \dfrac{A v_i}{\sigma_i}, & \sigma_i > 0 \\
  \text{영공간 보강 (선택)}, & \sigma_i = 0
\end{cases}
$$

   $\(\sigma_i>0\)$ 이면

 $$
   (AA^\top)u_i = \frac{A(A^\top A)v_i}{\sigma_i}
   = \frac{A(\lambda_i v_i)}{\sigma_i} = \sigma_i^2\,u_i,
 $$

   즉 $\(u_i\)$ 는 $\(AA^\top\)$ 의 고유벡터이다.

5. **정리**:  

 $$
   A = U\,\Sigma\,V^\top,\qquad
   U = \big[u_1\ \cdots\ u_n\big],\quad
   V = \big[v_1\ \cdots\ v_n\big].
 $$

---

## 알고리즘(구현 절차)

### 1) $\(B = A^\top A\)$ 생성 및 대칭화
수치 미세오차를 줄이기 위해

$$
B \leftarrow \tfrac12\,(B + B^\top).
$$

### 2) **야코비(Jacobi) 회전**으로 $\(B\)$ 를 대각화
모든 $\(p<q\)$ 쌍에 대해 오프대각 원소 $\(b_{pq}\)$ 를 0으로 만드는 평면 회전 $\(J(p,q,c,s)\)$ 를 반복 적용한다.

- 회전 파라미터 (NR 형식):

\[
\tau = \frac{b_{qq} - b_{pp}}{2\,b_{pq}},\qquad
t =
\begin{cases}
  \dfrac{1}{2\tau}, & \text{if } |\tau| \text{ is very large (approximation)} \\
  \dfrac{\text{sgn}(\tau)}{|\tau| + \sqrt{1 + \tau^2}}, & \text{otherwise}
\end{cases}
\]

$$
  c = \frac{1}{\sqrt{1+t^2}},\qquad s = t\,c.
$$

- 갱신(대칭 유지):

$$
  \begin{aligned}
  b'_{pp} &= b_{pp} - t\,b_{pq},\\
  b'_{qq} &= b_{qq} + t\,b_{pq},\\
  b'_{pq} &= b'_{qp} = 0,\\
  b'_{rp} &= c\,b_{rp} - s\,b_{rq},\quad
  b'_{rq} = s\,b_{rp} + c\,b_{rq}\quad (\forall r\ne p,q).
  \end{aligned}
$$

- 고유벡터 누적:
$$
  V \leftarrow V\,J(p,q,c,s),
$$

  즉 \(V\)의 열 \(p,q\)에 동일 회전을 적용한다.

- 종료 조건 예시:

$$
  \sqrt{\sum_{p<q} b_{pq}^2}\ <\ \varepsilon
  \quad\text{또는}\quad
  \text{sweep 수 상한 도달}.
$$

회전이 수렴하면 $\(B \approx V\,\Lambda\,V^\top\)$ 가 되며, $\(\Lambda\)$ 는 대각 $(\(\lambda_i\)$ ).

### 3) 특이값/정렬/부호
$$\sigma_i = \sqrt{\max(\lambda_i,\,0)}.$$

$\(\sigma\)$ 를 내림차순 정렬하고, 동일한 순서로 $\(V\)$ 의 열도 재정렬한다.
( $\(U,V\)$ 의 각 열벡터는 부호 반전이 허용된다. 구현상 $\(\sigma_i \ge 0\)$ 를 강제하고 필요 시 열 부호를 반전하는 관례를 따른다.)

### 4) $\(U\)$ 계산 및 정규화

$$
U[:,i] =
\begin{cases}
\displaystyle \frac{A\,V[:,i]}{\sigma_i}, & \sigma_i > \varepsilon,\$$10pt]
\text{0 또는 직교 보강}, & \sigma_i \le \varepsilon.
\end{cases}
$$

수치 안정화를 위해 각 열을 $\(\ell_2\)$ 정규화한다.

---

## 정확성 성질(테스트용 체크리스트)

- **정규직교성**

$$
  V^\top V = I,\qquad U^\top U \approx I\quad(\text{유효 열에 대해}).
$$

- **재구성 오차**

$$
  \|A - U\,\Sigma\,V^\top\|_F \ll \|A\|_F
  \quad(\text{double에서 일반적으로 }10^{-12}\sim10^{-14}).
$$

- **비음수/정렬**

$$
  \sigma_i \ge 0,\qquad \sigma_1 \ge \sigma_2 \ge \cdots \ge \sigma_n.
$$

- **랭크-결손 처리**
  $\(\sigma_i \le \varepsilon\)$ 인 열은 영(또는 직교 보강)로 두어도 $\(U\,\Sigma\,V^\top\)$ 재구성에는 영향 없음.

---

## 수치적 고려사항

- $\(B=A^\top A\)$ 는 **조건수가 제곱**된다:  

$$
  \kappa_2(B) = \kappa_2(A)^2.
$$

  매우 ill-conditioned한 문제에선 $\(QR\)$ 기반 SVD(Golub–Reinsch, divide-and-conquer)가 더 안정적일 수 있다.
- 종료 허용오차 $\(\varepsilon\)$ (예: $\(10^{-12}\sim10^{-14}\)$ )는 문제 스케일에 맞게 조정.
- $\(V\)$ 와 $\(U\)$ 의 열벡터 부호는 임의(±)이나, $\(\Sigma\)$ 는 관례적으로 비음수.
- 입력 스케일링(행/열의 단순 스케일)로 수치성을 개선할 수 있다.

---

## 복잡도 & 메모리

- 시간 복잡도(대략):

$$
  \underbrace{O(mn^2)}_{A^\top A,\ U계산}\ +\ \underbrace{O(n^3)}_{\text{Jacobi 대각화}}.
$$

  $\(m\ge n\)$ 에서 중·소형 행렬에 적합.
- 공간: $\(A(m\times n)\)$, $\(B(n\times n)\)$, $\(V(n\times n)\)$, 작업 벡터/임시 버퍼.

---

## 의사코드

```text
Input:  A ∈ R^{m×n}
Output: U (m×n), Σ (diag σ_i, length n), V (n×n)

1: B ← Aᵀ A
2: B ← (B + Bᵀ)/2              # 수치적 대칭화
3: (Λ, V) ← JacobiSymmetricEigen(B)
   # Jacobi: 반복적으로 p<q 쌍에 Givens 회전 J(p,q,c,s) 적용하여 오프대각 제거
4: σ_i ← sqrt(max(Λ_ii, 0))
5: σ 내림차순 정렬, 동일 순서로 V 열 재정렬
6: for i = 1..n:
       if σ_i > eps:
           U[:,i] ← (A · V[:,i]) / σ_i
       else:
           U[:,i] ← 0  # (필요시 직교 보강)
   열 정규화(U)
7: return (U, Σ, V)
```

---

## 선택 사항: $\(\sigma=0\)$ 인 열의 좌특이벡터 보강

랭크-결손이면 $\(i\)$ 에 대해 $\(A v_i = 0\)$. 이때 $\(U[:,i]\)$ 는
- 0 벡터로 두어도 재구성에는 영향 없음(현재 구현 기본),
- 혹은 $\(U\)$ 의 기존 열들과 직교가 되도록 **그람-슈미트**로 $\(\mathcal{N}(A^\top)\)$ 에서 기저를 완성할 수 있다.

---

## Golub–Reinsch SVD와의 비교(요약)

- **본 구현(야코비-고유)**: 간결, 구현 용이, 테스트/중소형 문제에 강함.  
  단점: $\(A^\top A\)$ 로 인해 **조건수 제곱**.
- **Golub–Reinsch(하우스홀더→이중대각→QR)**:  
  수치적으로 가장 안정적(Backward stable), 대형 행렬에서도 효율적.  
  구현 복잡도와 코드 길이가 길다.

---

## 단위 테스트에서 권장 검증 항목

- $\(\|V^\top V - I\|_F < 10^{-10}\)$
- $\(\|U^\top U - I\|_F < 10^{-10}\)$ (유효 열)
- $\(\|A - U\Sigma V^\top\|_F / \|A\|_F < 10^{-12}\)$
- $\(\sigma\)$ 내림차순/비음수
- 랭크-1/대각/무작위/구성된 $\(\Sigma\)$ 케이스

---

## 요약

현재 `svdcmp`는 

$$
B=A^\top A\ \xrightarrow{\ \text{Jacobi}\ }\ B=V\Lambda V^\top
\ \Rightarrow\ 
\sigma_i=\sqrt{\lambda_i},\ 
U[:,i] = \frac{A\,V[:,i]}{\sigma_i}
$$

절차로 $\(A=U\Sigma V^\top\)$ 을 구성합니다. 수치적으로 건전하며, 테스트 가능한 불변식(정규직교, 재구성 오차, $\(\sigma\)$ 정렬/비음수)을 만족하도록 설계되어 있습니다.
