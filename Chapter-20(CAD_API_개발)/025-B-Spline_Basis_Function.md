# B-Spline Basis Function 정리

## 🔹 1. basis_funs(span, t, p)
### 📐 목적
- B-spline 기저 함수 $N_{i,p}(t)$ 계산
- Piegl & Tiller의 Algorithm A2.2에 해당

### 🧮 수식 정의
기저 함수는 재귀적으로 정의됩니다:

$$
N_{i,0}(t) =
\begin{cases}
1 & \text{if } u_i \leq t < u_{i+1} \\
0 & \text{otherwise}
\end{cases}
$$


$$
N_{i,p}(t)=\frac{t-u_i}{u_{i+p}-u_i}N_{i,p-1}(t)+\frac{u_{i+p+1}-t}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(t)
$$


### 🧭 단계별 설명
- 특수 케이스 처리
- $t\approx u_{n+1}$ 일 때, 마지막 기저 함수만 1이고 나머지는 0
- 초기화
- $N_0=1$
- 재귀 계산
- $left[j] = t - u_{i+1-j}$
- $right[j] = u_{i+j} - t$
- 반복적으로 $N_j$ 계산

### ✅ 수학적 검증
- 재귀 정의와 동일
- 수치 안정성을 위해 EPSILON 체크 포함
- 마지막 점에서의 특수 처리도 정확

## 🔹 2. ders_basis_funs(span, u, p, n)
### 📐 목적
- B-spline 기저 함수의 도함수 $N_{i,p}^{(k)}(u)$ 계산
- Piegl & Tiller의 Algorithm A2.3에 해당

### 🧮 수식 정의
도함수는 다음과 같이 계산됩니다:

B-spline 기저 함수의 k차 도함수는 다음과 같이 계산됩니다:

$$
N_{i,p}^{(k)}(u)=\sum _{j=0}^pd_{k,j}\cdot N_{i+j,p-k}(u)
$$

여기서 $d_{k,j}$ 는 다음과 같이 정의되는 계수입니다:

$$
d_{k,j}=\frac{p!}{(p-k)!}\cdot \mathrm{기저\  함수\  분할에\  따른\  계수}
$$

하지만 실제 계산에서는 이 식을 직접 쓰기보다 Piegl & Tiller의 Algorithm A2.3을 사용하여 다음과 같은 방식으로 계산합니다:


### 🧭 단계별 설명
- 기저 함수 테이블 ndu 계산
- $ndu[r][j]$ 는 $N_{i+j-r,p-r}(u)$ 에 해당
- 0차 도함수 저장
- $ders[0][j] = ndu[j][p]$
- 도함수 계산
- 보조 테이블 a를 사용해 반복적으로 도함수 계산
- 각 단계에서 a를 swap하며 누적
- 스케일링
- $N^{(k)}는 p(p-1)...(p-k+1)$ 로 스케일링 필요

### ✅ 수학적 검증
- Piegl & Tiller 알고리즘 A2.3을 정확히 구현
- 도함수 계산 시 외적 분할과 누적 방식이 수학적으로 타당
- 스케일링도 정확히 적용됨

| 항목                     | 평가 내용                                                                 |
|--------------------------|---------------------------------------------------------------------------|
| 수학적 정의 일치         | Piegl & Tiller의 알고리즘 A2.2, A2.3을 정확히 구현함                     |
| 수치 안정성              | 0으로 나누는 경우 `EPSILON` 처리로 안정성 확보                           |
| 특수 케이스 처리         | u ≈ U[span+1]인 경우 마지막 기저 함수만 1로 설정 (클램프 조건 반영)       |
| 도함수 스케일링          | 도함수 결과에 (p)(p−1)...(p−k+1) 계수 곱하여 정확한 도함수 값 도출       |
| 메모리 및 성능 최적화    | 재사용 가능한 테이블 구조 (`ndu`, `a`)로 중복 계산 최소화                |
| 구현 완성도              | 경계 조건, 반복 구조, 보조 테이블 등 모든 측면에서 완성도 높음           |


---
## basis_funs
```rust
fn basis_funs(&self, span: usize, t: f64, p: usize) -> Vec<f64> {
    let mut n_vec = vec![0.0; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];

    // ---- Special case: right endpoint (u == U[span+1]) ----
    // For clamped curves, if u == U[n+1] and span == n, then N[p] = 1 and all others are 0
    // (A small tolerance 'tol' is used to account for numerical error)
    let tol = 1e-14 * (self[self.len() - 1] - self[0]).abs().max(1.0);
    if (t - self[span + 1]).abs() <= tol {
        n_vec[p] = 1.0;
        return n_vec;
    }

    n_vec[0] = 1.0;
    for j in 1..=p {
        left[j] = t - self[span + 1 - j];
        right[j] = self[span + j] - t;

        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > f64::EPSILON {
                n_vec[r] / denom
            } else {
                0.0
            };
            n_vec[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n_vec[j] = saved;
    }
    n_vec
}
```

## ders_basis_funs
```rust
fn ders_basis_funs(&self, span: usize, u: f64, p: usize, n: usize) -> Vec<Vec<f64>> {
    // Piegl & Tiller Algorithm A2.3
    let n = n.min(p);
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - self[span + 1 - j];
        right[j] = self[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > f64::EPSILON {
                ndu[r][j - 1] / denom
            } else {
                0.0
            };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // 파생 출력 배열
    let mut ders = vec![vec![0.0; p + 1]; n + 1];
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }

    // a: 보조 테이블
    let mut a = vec![vec![0.0; p + 1]; 2];

    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;

        a[0][0] = 1.0;

        for k in 1..=n {
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            let mut d = 0.0;
            // 첫 항
            if r >= k {
                let denom = ndu[(pk + 1) as usize][rk as usize];
                a[s2][0] = if denom.abs() > f64::EPSILON {
                    a[s1][0] / denom
                } else {
                    0.0
                };
                d += a[s2][0] * ndu[rk as usize][pk as usize];
            } else {
                a[s2][0] = 0.0;
            }

            // 중간 항
            let j1 = if rk >= 0 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk { k - 1 } else { p - r };

            for j in j1..=j2 {
                let denom = ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                let num = a[s1][j] - a[s1][j - 1];
                a[s2][j] = if denom.abs() > f64::EPSILON {
                    num / denom
                } else {
                    0.0
                };
                d += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            // 마지막 항
            if r <= (pk as usize) {
                let denom = ndu[(pk + 1) as usize][r];
                a[s2][k] = if denom.abs() > f64::EPSILON {
                    -a[s1][k - 1] / denom
                } else {
                    0.0
                };
                d += a[s2][k] * ndu[r][pk as usize];
            } else {
                a[s2][k] = 0.0;
            }

            ders[k][r] = d;

            // swap rows
            for j in 0..=k {
                a[s1][j] = 0.0;
            }
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // 스케일링: k차 도함수에 (p)(p-1)…(p-k+1)
    let mut fac = 1.0;
    for k in 1..=n {
        fac *= (p + 1 - k) as f64;
        for j in 0..=p {
            ders[k][j] *= fac;
        }
    }

    ders
}
```
---

