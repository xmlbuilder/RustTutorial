# svdcmp: Jacobi-Eigen 기반 SVD 정리

- 이 문서는 현재 구현한 `svdcmp`(Rust) 함수가 사용하는 **AᵀA 고유분해 → SVD** 절차를 수식으로 정리한 문서입니다.
- 세 개로 분해
  -  입력 축을 “정렬”하는 회전/직교변환(𝑉ᵀ),
  -  각 축을 늘이거나 줄이는 스케일(특이값) 대각행렬(Σ),
  -  결과를 최종 위치로 “되돌리는” 회전/직교변환(U)
- 최적 저랭크 근사, 안정적 해 구하기, 차원축소/잡음제거 등 실무에 바로 먹히는 이점을 준다.

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

$$
\tau = \frac{b_{qq} - b_{pp}}{2\,b_{pq}},\qquad
t =
\begin{cases}
  \dfrac{1}{2\tau}, & \text{if } |\tau| \text{ is very large (approximation)} \\
  \dfrac{\text{sgn}(\tau)}{|\tau| + \sqrt{1 + \tau^2}}, & \text{otherwise}
\end{cases}
$$

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

  즉 $\(V\)$ 의 열 $\(p,q\)$ 에 동일 회전을 적용한다.

- 종료 조건 예시:

$$
\sqrt{\sum_{p \lt q} b_{pq}^{2}} \lt \varepsilon
\quad \text{or sweep-count limit reached}
$$

회전이 수렴하면 $\(B \approx V\,\Lambda\,V^\top\)$ 가 되며, $\(\Lambda\)$ 는 대각 $(\(\lambda_i\)$ ).

### 3) 특이값/정렬/부호

$$
\sigma_i = \sqrt{\max(\lambda_i,\,0)}.
$$

$\(\sigma\)$ 를 내림차순 정렬하고, 동일한 순서로 $\(V\)$ 의 열도 재정렬한다.
( $\(U,V\)$ 의 각 열벡터는 부호 반전이 허용된다. 구현상 $\(\sigma_i \ge 0\)$ 를 강제하고 필요 시 열 부호를 반전하는 관례를 따른다.)

### 4) $\(U\)$ 계산 및 정규화

$$
U[:,i] =
\begin{cases}
\displaystyle \frac{A\,V[:,i]}{\sigma_i}, & \sigma_i > \varepsilon \\
\text{0 또는 직교 보강}, & \sigma_i \le \varepsilon
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

---

## 소스
```rust
use std::f64::EPSILON;
use crate::core::tarray::TArray;
use crate::math::prelude::matrix::Matrix;

#[inline]
fn hypot2(a: f64, b: f64) -> f64 { a.hypot(b) }

/// 대칭행렬 B (n×n)를 야코비 회전으로 고유분해.
/// 결과: B는 대각(고유값), v는 열-고유벡터(정규직교).
fn jacobi_symmetric_eigen(b: &mut Matrix, vals: &mut Vec<f64>, v: &mut Matrix) -> bool {
    let n = b.row_count();
    if n == 0 || b.col_count() != n { return false; }

    // v <- I
    if !v.create(n as i32, n as i32) { return false; }
    for i in 0..n { for j in 0..n { *v.at_mut(i as i32, j as i32) = if i==j {1.0} else {0.0}; } }

    // 반복 파라미터
    let max_sweeps = 50 * n * n;
    let tol = 1e-14_f64;

    // 도움: 합 오프대각의 제곱합
    let mut off2 = |m: &Matrix| -> f64 {
        let mut s=0.0;
        for p in 0..n { for q in 0..n {
            if p!=q { let x=*m.at(p as i32,q as i32); s+=x*x; }
        }}
        s
    };

    // 반복
    let mut sweep = 0usize;
    loop {
        let mut changed = false;

        for p in 0..n {
            for q in (p+1)..n {
                let app = *b.at(p as i32, p as i32);
                let aqq = *b.at(q as i32, q as i32);
                let apq = *b.at(p as i32, q as i32);
                if apq.abs() <= tol * hypot2(app.abs(), aqq.abs()) { continue; }

                // 회전계수 (NR 방식)
                let tau = (aqq - app) / (2.0 * apq);
                let t = if tau.abs() + 1.0 == 1.0 {
                    1.0 / (2.0 * tau)
                } else {
                    let sgn = if tau >= 0.0 { 1.0 } else { -1.0 };
                    sgn / (tau.abs() + (1.0 + tau*tau).sqrt())
                };
                let c = 1.0 / (1.0 + t*t).sqrt();
                let s = t * c;

                // B <- Jᵀ B J  (대칭 유지)
                // 행/열 p,q 업데이트
                let bpp = app - t*apq;
                let bqq = aqq + t*apq;
                *b.at_mut(p as i32, p as i32) = bpp;
                *b.at_mut(q as i32, q as i32) = bqq;
                *b.at_mut(p as i32, q as i32) = 0.0;
                *b.at_mut(q as i32, p as i32) = 0.0;

                for r in 0..n {
                    if r != p && r != q {
                        let arp = *b.at(r as i32, p as i32);
                        let arq = *b.at(r as i32, q as i32);
                        let nrp = c*arp - s*arq;
                        let nrq = s*arp + c*arq;
                        *b.at_mut(r as i32, p as i32) = nrp;
                        *b.at_mut(p as i32, r as i32) = nrp;
                        *b.at_mut(r as i32, q as i32) = nrq;
                        *b.at_mut(q as i32, r as i32) = nrq;
                    }
                }

                // V <- V J (열-고유벡터)
                for r in 0..n {
                    let vrp = *v.at(r as i32, p as i32);
                    let vrq = *v.at(r as i32, q as i32);
                    *v.at_mut(r as i32, p as i32) = c*vrp - s*vrq;
                    *v.at_mut(r as i32, q as i32) = s*vrp + c*vrq;
                }

                changed = true;
            }
        }

        sweep += 1;
        if !changed { break; }
        if sweep > max_sweeps { break; } // 안전 탈출
        if off2(b) < tol { break; }
    }

    // 고유값 추출
    vals.clear();
    vals.resize(n, 0.0);
    for i in 0..n { vals[i] = *b.at(i as i32, i as i32); }
    true
}

/// SVD via Jacobi-eigen on AᵀA
/// 입력:  a (m×n)  — 변경 후 U 저장 (m×n)
/// 출력:  w (n)    — 특이값
///        v (n×n)  — 우직교 행렬
pub fn svdcmp(a: &mut Matrix, w: &mut TArray<f64>, v: &mut Matrix) -> bool {
    let m = a.row_count();
    let n = a.col_count();
    if m == 0 || n == 0 { return false; }

    // A 보존
    let a0 = a.clone();

    // B = AᵀA (n×n)
    let mut at = a0.clone(); at.transpose();           // n×m
    let mut b  = &at * &a0;                            // (n×m)*(m×n) = n×n

    // 대칭 수치화(미세한 비대칭 제거)
    for i in 0..n {
        for j in 0..n {
            let x = 0.5 * (*b.at(i as i32,j as i32) + *b.at(j as i32,i as i32));
            *b.at_mut(i as i32,j as i32) = x;
        }
    }

    // 고유분해
    let mut evals: Vec<f64> = Vec::new();
    if !jacobi_symmetric_eigen(&mut b, &mut evals, v) { return false; }

    // 고유값↓ 정렬 + V 열 재정렬
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i,&j| evals[j].partial_cmp(&evals[i]).unwrap());

    let mut wvec = vec![0.0f64; n];
    let mut v_sorted = Matrix::with_dims(n, n);
    for (col, &k) in idx.iter().enumerate() {
        wvec[col] = evals[k].max(0.0).sqrt();
        for r in 0..n {
            *v_sorted.at_mut(r as i32, col as i32) = *v.at(r as i32, k as i32);
        }
    }
    *v = v_sorted;
    w.set_size(n);
    for i in 0..n { w[i] = wvec[i]; }

    // U = A * V * Σ^{-1}  (σ_i > 0만)
    if !a.create(m as i32, n as i32) { return false; }
    let eps = 1e-12_f64;
    for j in 0..n {
        let sigma = w[j];
        if sigma > eps {
            for r in 0..m {
                let mut s = 0.0;
                for k in 0..n { s += *a0.at(r as i32, k as i32) * *v.at(k as i32, j as i32); }
                *a.at_mut(r as i32, j as i32) = s / sigma;
            }
        } else {
            // σ=0: 임의의 직교 완성 (여기서는 0 벡터로 두고, 필요하면 그람-슈미트로 보강 가능)
            for r in 0..m { *a.at_mut(r as i32, j as i32) = 0.0; }
        }
    }

    // 선택: U 열 정규화(수치 안정)
    for j in 0..n {
        let mut s=0.0; for r in 0..m { let x=*a.at(r as i32,j as i32); s+=x*x; }
        let nrm = s.sqrt();
        if nrm > EPSILON { for r in 0..m { *a.at_mut(r as i32,j as i32) /= nrm; } }
    }

    true
}

```

## 테스트 코드
```rust

#[cfg(test)]
mod tests {
    use geometry::core::tarray::TArray;
    use geometry::math::prelude::matrix::Matrix;
    use geometry::math::svd::{solve_least_squares_svd, svdcmp};

    // 외부 크레이트 없이 작성.
    // -----------------------------------------------------------------------------
    // 헬퍼들
    // -----------------------------------------------------------------------------
    fn approx_eq(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() <= tol }

    fn diff_fro_norm(a: &Matrix, b: &Matrix) -> f64 {
        assert_eq!(a.row_count(), b.row_count());
        assert_eq!(a.col_count(), b.col_count());
        let (r, c) = (a.row_count(), a.col_count());
        let mut s = 0.0;
        for i in 0..r { for j in 0..c {
            let v = *a.at(i as i32, j as i32) - *b.at(i as i32, j as i32);
            s += v * v;
        }}
        s.sqrt()
    }

    // Σ (n×n, n = w.len())
    fn make_sigma_square(w: &[f64]) -> Matrix {
        let n = w.len();
        let mut s = Matrix::with_dims(n, n);
        s.zero();
        for i in 0..n { *s.at_mut(i as i32, i as i32) = w[i]; }
        s
    }

    fn mat_t_mat(m: &Matrix, n: &Matrix) -> Matrix {
        let mut mt = m.clone();
        mt.transpose();
        &mt * n
    }

    fn has_orthonormal_cols(u: &Matrix, tol: f64) -> bool {
        // U: m×n → UᵀU: n×n
        let utu = mat_t_mat(u, u);
        let n = u.col_count();
        for i in 0..n {
            for j in 0..n {
                let want = if i==j {1.0} else {0.0};
                if !approx_eq(*utu.at(i as i32, j as i32), want, tol) { return false; }
            }
        }
        true
    }

    fn is_orthonormal(v: &Matrix, tol: f64) -> bool {
        let vtv = mat_t_mat(v, v);
        let n = v.row_count();
        for i in 0..n {
            for j in 0..n {
                let want = if i==j {1.0} else {0.0};
                if !approx_eq(*vtv.at(i as i32, j as i32), want, tol) { return false; }
            }
        }
        true
    }

    // Â = U Σ Vᵀ  (U: m×n, Σ: n×n, V: n×n)  — NR 스타일(m≥n)
    fn sorted_desc(mut xs: Vec<f64>) -> Vec<f64> {
        xs.sort_by(|a,b| b.partial_cmp(a).unwrap());
        xs
    }

    fn assert_all_nonneg(ws: &[f64], tol: f64) {
        for &x in ws { assert!(x >= -tol, "singular value is negative: {}", x); }
    }

    // ------------------- tests -------------------

    #[test]
    fn svd_identity_3x3() {
        let mut a = Matrix::with_dims(3,3);
        a.set_diagonal_scalar(1.0);
        let a0 = a.clone();

        let mut w = TArray::<f64>::new();
        let mut v = Matrix::new();
        assert!(svdcmp(&mut a, &mut w, &mut v));

        assert!(has_orthonormal_cols(&a, 1e-12), "UᵀU ≉ I");
        assert!(is_orthonormal(&v, 1e-12), "VᵀV ≉ I");

        let got = sorted_desc(w.data.clone());
        let expect = vec![1.0, 1.0, 1.0];
        for (g,e) in got.iter().zip(expect.iter()) {
            assert!(approx_eq(*g, *e, 1e-12), "σ mismatch: {g} vs {e}");
        }

        let a_rec = reconstruct(&a, &w.data, &v);
        let err = diff_fro_norm(&a0, &a_rec);
        assert!(err <= 1e-12, "reconstruction error = {}", err);
    }

    #[test]
    fn svd_diagonal_rect_3x2() {
        // A = diag(3,2) in 3x2 (m≥n)
        let mut a = Matrix::with_dims(3,2);
        a.zero();
        *a.at_mut(0,0) = 3.0;
        *a.at_mut(1,1) = 2.0;
        let a0 = a.clone();

        let mut w = TArray::<f64>::new();
        let mut v = Matrix::new();
        assert!(svdcmp(&mut a, &mut w, &mut v));

        assert_all_nonneg(&w.data, 1e-12);
        let got = sorted_desc(w.data.clone());
        let expect = vec![3.0, 2.0];
        for (g,e) in got.iter().zip(expect.iter()) {
            assert!(approx_eq(*g, *e, 1e-10), "σ mismatch: {g} vs {e}");
        }

        assert!(has_orthonormal_cols(&a, 1e-12));
        assert!(is_orthonormal(&v, 1e-12));

        let a_rec = reconstruct(&a, &w.data, &v);
        let err = diff_fro_norm(&a0, &a_rec);
        assert!(err <= 1e-12, "reconstruction error = {}", err);
    }

    fn reconstruct(u: &Matrix, w: &[f64], v: &Matrix) -> Matrix {
        let n = w.len();
        let mut s = Matrix::with_dims(n, n);
        s.zero();
        for i in 0..n { *s.at_mut(i as i32, i as i32) = w[i]; }
        let mut vt = v.clone(); vt.transpose();
        &(*&u * &s) * &vt
    }
    fn fro(a: &Matrix) -> f64 {
        let (r,c) = (a.row_count(), a.col_count());
        let mut s=0.0;
        for i in 0..r { for j in 0..c { let x=*a.at(i as i32,j as i32); s+=x*x; } }
        s.sqrt()
    }
    fn fro_diff(a: &Matrix, b: &Matrix) -> f64 {
        assert_eq!(a.row_count(), b.row_count());
        assert_eq!(a.col_count(), b.col_count());
        let (r,c) = (a.row_count(), a.col_count());
        let mut s=0.0;
        for i in 0..r { for j in 0..c { let x=*a.at(i as i32,j as i32)-*b.at(i as i32,j as i32); s+=x*x; } }
        s.sqrt()
    }

    #[test]
    fn dbg_rank1_rect_3x2() {
        // A = u vᵀ (랭크 1) → σ = [9, 0]
        let u = [1.0, 2.0, 2.0];
        let v2 = [0.0, 3.0];
        let mut a = Matrix::with_dims(3, 2);
        for i in 0..3 { for j in 0..2 { *a.at_mut(i as i32,j as i32)=u[i]*v2[j]; } }
        let a0 = a.clone();

        let mut w = TArray::<f64>::new();
        let mut v = Matrix::new();
        let ok = svdcmp(&mut a, &mut w, &mut v);
        println!("\n[rank1 3x2] ok={ok}, w={:?}", w.data);

        assert!(ok, "svdcmp failed");

        let mut ws = w.data.clone();
        ws.sort_by(|a,b| b.partial_cmp(a).unwrap());
        println!("sorted σ = {:?}", ws);

        let a_rec = reconstruct(&a, &w.data, &v);
        let err = fro_diff(&a0, &a_rec);
        println!("reconstruct error (fro) = {:.6e}", err);
        println!("‖A‖_F = {:.6},  ‖UΣVᵀ‖_F = {:.6}", fro(&a0), fro(&a_rec));

        assert!((ws[0]-9.0).abs()<1e-8 && ws[1].abs()<1e-10, "σ = {:?} (expected [9,0])", ws);
        assert!(err < 1e-8, "reconstruction error too large");
    }

    #[test]
    fn dbg_constructed_answer_4x3() {
        // Σ = diag(7,3,1) 를 인위적으로 구성한 4×3 케이스
        let mut u0 = Matrix::with_dims(4, 3); u0.zero();
        *u0.at_mut(0,0)=1.0; *u0.at_mut(1,1)=1.0; *u0.at_mut(2,2)=1.0;

        let sigma = [7.0,3.0,1.0];
        let mut s = Matrix::with_dims(3,3); s.zero();
        for i in 0..3 { *s.at_mut(i as i32,i as i32)=sigma[i]; }

        let (c, s_) = ((std::f64::consts::PI/7.0).cos(), (std::f64::consts::PI/7.0).sin());
        let mut v0 = Matrix::with_dims(3,3);
        *v0.at_mut(0,0)=c;  *v0.at_mut(0,1)=-s_; *v0.at_mut(0,2)=0.0;
        *v0.at_mut(1,0)=s_; *v0.at_mut(1,1)= c;  *v0.at_mut(1,2)=0.0;
        *v0.at_mut(2,0)=0.0;*v0.at_mut(2,1)=0.0; *v0.at_mut(2,2)=1.0;

        let mut v0t = v0.clone(); v0t.transpose();
        let a0 = &(&u0 * &s) * &v0t;

        let mut a = a0.clone();
        let mut w = TArray::<f64>::new();
        let mut v = Matrix::new();
        let ok = svdcmp(&mut a, &mut w, &mut v);
        println!("\n[constructed 4x3] ok={ok}, w={:?}", w.data);

        assert!(ok, "svdcmp failed");

        let mut ws = w.data.clone();
        ws.sort_by(|a,b| b.partial_cmp(a).unwrap());
        println!("sorted σ = {:?}", ws);

        let a_rec = reconstruct(&a, &w.data, &v);
        let err = fro_diff(&a0, &a_rec);
        println!("reconstruct error (fro) = {:.6e}", err);
        println!("‖A‖_F = {:.6},  ‖UΣVᵀ‖_F = {:.6}", fro(&a0), fro(&a_rec));

        let mut ex = sigma.to_vec(); ex.sort_by(|a,b| b.partial_cmp(a).unwrap());
        for (g,e) in ws.iter().zip(ex.iter()) {
            assert!((g-e).abs()<1e-8, "σ mismatch: got {}, expect {}", g, e);
        }
        assert!(err < 1e-8, "reconstruction error too large");
    }

    #[test]
    fn svd_constructed_answer_4x3() {
        // Σ = diag(7,3,1) 를 인위적으로 구성한 4×3
        let mut u0 = Matrix::with_dims(4, 3); u0.zero();
        *u0.at_mut(0,0)=1.0; *u0.at_mut(1,1)=1.0; *u0.at_mut(2,2)=1.0; // 직교 열 3개(간단)

        let sigma = [7.0,3.0,1.0];
        let mut s = Matrix::with_dims(3,3); s.zero();
        for i in 0..3 { *s.at_mut(i as i32,i as i32)=sigma[i]; }

        let (c, s_) = ((std::f64::consts::PI/7.0).cos(), (std::f64::consts::PI/7.0).sin());
        let mut v0 = Matrix::with_dims(3,3);
        *v0.at_mut(0,0)=c;  *v0.at_mut(0,1)=-s_; *v0.at_mut(0,2)=0.0;
        *v0.at_mut(1,0)=s_; *v0.at_mut(1,1)= c;  *v0.at_mut(1,2)=0.0;
        *v0.at_mut(2,0)=0.0;*v0.at_mut(2,1)=0.0; *v0.at_mut(2,2)=1.0;

        let mut v0t = v0.clone(); v0t.transpose();
        let a0 = &(&u0 * &s) * &v0t;

        let mut a = a0.clone();
        let mut w = TArray::<f64>::new();
        let mut v = Matrix::new();
        let ok = svdcmp(&mut a, &mut w, &mut v);
        assert!(ok, "svdcmp failed");

        let mut ws = w.data.clone();
        ws.sort_by(|a,b| b.partial_cmp(a).unwrap());
        println!("constructed σ = {:?}", ws);
        let mut ex = sigma.to_vec(); ex.sort_by(|a,b| b.partial_cmp(a).unwrap());
        for (g,e) in ws.iter().zip(ex.iter()) {
            assert!((g-e).abs()<1e-10, "σ mismatch: got {}, expect {}", g, e);
        }

        let a_rec = reconstruct(&a, &w.data, &v);
        let err = fro_diff(&a0, &a_rec);
        println!("recon err = {:.3e}", err);
        assert!(err < 1e-10, "reconstruction error too large");
    }
}
```
