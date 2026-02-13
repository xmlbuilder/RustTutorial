# 🔹 B-spline Basis Function의 도함수 정리
## 1. 1차 도함수 공식 (기본 형태)
```math
N_{i,p}(u)=\frac{u-u_i}{u_{i+p}-u_i}N_{i,p-1}(u)+\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(u)
```
- 이를 미분하면:


## 2. 도함수 재귀 대입 후 정리
```math
\begin{aligned}N'_{i,p}(u)&=\frac{1}{u_{i+p}-u_i}N_{i,p-1}(u)-\frac{1}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(u)\\ \\ &\quad +\frac{p-1}{u_{i+p}-u_i}\left( \frac{u-u_i}{u_{i+p-1}-u_i}N_{i,p-2}(u)-\frac{u-u_i}{u_{i+p}-u_{i+1}}N_{i+1,p-2}(u)\right) \\ \\ &\quad +\frac{p-1}{u_{i+p+1}-u_{i+1}}\left( \frac{u_{i+p+1}-u}{u_{i+p}-u_{i+1}}N_{i+1,p-2}(u)-\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+2}}N_{i+2,p-2}(u)\right) \end{aligned}
```


## 3. k차 도함수의 일반화 표현 (계수 $a_{k,j}$ 사용)

- 계수 초기값:
```math
a_{0,0}=1
```
```math
a_{1,0}=\frac{1}{u_{i+p}-u_i},\quad a_{1,1}=\frac{-1}{u_{i+p+1}-u_{i+1}}
```
- 일반적인 계수 재귀식:
```math
a_{k,j}=\frac{a_{k-1,j}}{u_{i+p-j}-u_{i-j}}-\frac{a_{k-1,j-1}}{u_{i+p+1-j}-u_{i+1-j}},\quad j=1,\dots ,k-1
```
```math
a_{k,0}=\frac{a_{k-1,0}}{u_{i+p}-u_i},\quad a_{k,k}=\frac{-a_{k-1,k-1}}{u_{i+p+1-k}-u_{i+1-k}}
```
---
# on_ders_basis_funs
## 0. 이 함수가 계산하는 것
- 입력:
    - knot vector knots
    - span index span
    - 파라미터 u
    - 차수 p
    - 최대 도함수 차수 nder
- 출력:
```math
\mathrm{ders}[k][r]\approx \frac{d^k}{du^k}N_{i-p+r,\; p}(u),\quad k=0,\dots ,\mathrm{nder},\; r=0,\dots ,p
```
- 여기서 $i=\mathrm{span}$ 이고,
- r번째 항은 해당 span에서 non‑vanishing인 p-차 B‑spline basis에 대응.

## 1. ndu 테이블: Cox–de Boor로 $N_{i,p}(u)$ 계산
- 코드:
```rust
ndu[0][0] = 1.0;

for j in 1..=p {
    left[j] = u - knots[span + 1 - j];
    right[j] = knots[span + j] - u;

    let mut saved = 0.0;
    for r in 0..j {
        let denom = right[r + 1] + left[j - r];
        ndu[j][r] = denom;

        let temp = if denom.abs() > 1e-15 {
            ndu[r][j - 1] / denom
        } else {
            0.0
        };

        ndu[r][j] = saved + right[r + 1] * temp;
        saved = left[j - r] * temp;
    }
    ndu[j][j] = saved;
}
```

- 이 부분은 Algorithm A2.2로, 사실상 다음을 구현하고 있어:
- left[j] = $u - u_{i+1-j}$
- right[j] = $u_{i+j} - u$
- 그리고
```math
\mathrm{ndu}[r][j]\equiv N_{i-p+r,\; j}(u)
```
- 이 되도록 재귀적으로 채워 넣는 구조야.
- Cox–de Boor 재귀식:
```math
N_{i,p}(u)=\frac{u-u_i}{u_{i+p}-u_i}N_{i,p-1}(u)+\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(u)
```
- 이걸 테이블 형태로 풀어쓴 게 바로 ndu 빌드 루프.

## 2. 0차 도함수: 그냥 basis 값
```rust
for r in 0..=p {
    ders[0][r] = ndu[r][p];
}
```

- 여기서
```math
\mathrm{ders}[0][r]=N_{i-p+r,\; p}(u)
```
- 이 되고,
    - ndu[r][p]가 바로 p-차 B‑spline 값.

## 3. 도함수 계산: Algorithm A2.3 → $N_{i,p}^{(k)}(u)$
- 핵심 루프:
```rust
let mut a = vec![vec![0.0; p + 1]; 2];

for r in 0..=p {
    let mut s1 = 0usize;
    let mut s2 = 1usize;
    a[0][0] = 1.0;

    for k in 1..=nder {
        let mut d = 0.0;
        rk = r as isize - k as isize;
        pk = p as isize - k as isize;

        // 첫 항 (j=0)
        if rk >= 0 {
            let denom = ndu[(pk + 1) as usize][rk as usize];
            a[s2][0] = ...
            d = a[s2][0] * ndu[rk as usize][pk as usize];
        }

        // 중간 항 (j1..j2)
        for j in j1..=j2 {
            let idx = (rk + j as isize) as usize;
            let denom = ndu[(pk + 1) as usize][idx];
            a[s2][j] = ...
            d += a[s2][j] * ndu[idx][pk as usize];
        }

        // 마지막 항 (j=k)
        if r as isize <= pk {
            let idx = (rk + k as isize) as usize; // = r
            let denom = ndu[(pk + 1) as usize][idx];
            a[s2][k] = ...
            d += a[s2][k] * ndu[idx][pk as usize];
        }

        ders[k][r] = d;
        std::mem::swap(&mut s1, &mut s2);
    }
}
```

- 이 부분이 바로 B‑spline 도함수의 일반식을 구현.
- 이론적으로는:
```math
N_{i,p}^{(k)}(u)=\frac{p!}{(p-k)!}\sum _{j=0}^ka_{k,j}\, N_{i+k-j,\; p-k}(u)
```
- 여기서:
    - a[s1][j], a[s2][j] → 계수 $a_{k,j}$
    - ndu[idx][pk] → $N_{*,\, p-k}(u)$
- 즉, 각 k에 대해:
```math
\mathrm{ders}[k][r]=\sum _{j=0}^ka_{k,j}\, N_{i-p+r+k-j,\; p-k}(u)
```
- 을 계산.

## 4. 마지막 스케일링:  곱하기
```rust
let mut r_fact = p as f64;
for k in 1..=nder {
    for r in 0..=p {
        ders[k][r] *= r_fact;
    }
    r_fact *= (p - k) as f64;
}
```

- 이 부분이 바로 위 일반식의 계수
```math
\frac{p!}{(p-k)!}
```
를 곱해주는 단계.
- 처음 r_fact = p
- 그 다음 p(p-1)
- 그 다음 p(p-1)(p-2)
- 이렇게 누적되면서
```math
r\_ fact=\frac{p!}{(p-k)!}
```
- 이 되어, 각 ders[k][r]에 곱해진다.

## 5. 전체적으로 정리하면
이 함수는 다음 수식을 그대로 구현하고 있어:
- ndu로 $N_{i-p+r,j}(u) (모든 차수 j\leq p)$ 를 Cox–de Boor 재귀로 계산
- ders[0][r] = $N_{i-p+r,p}(u)$
- Algorithm A2.3로 계수 $a_{k,j}$ 를 재귀적으로 계산
```math
N_{i-p+r,\; p}^{(k)}(u)=\frac{p!}{(p-k)!}\sum _{j=0}^ka_{k,j}\, N_{i-p+r+k-j,\; p-k}(u)
```
- 를 코드로 구현
- 그 결과가 ders[k][r]에 저장됨

---
## 소스 코드
```rust
pub fn on_ders_basis_funs(
    knots: &[Real],
    span: usize,
    u: Real,
    p: usize,
    nder: usize,
) -> Vec<Vec<Real>> {
    let nder = nder.min(p);
    let mut ders = vec![vec![0.0; p + 1]; nder + 1];

    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    ndu[0][0] = 1.0;

    // Algorithm A2.2: ndu table
    for j in 1..=p {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;

        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            ndu[j][r] = denom;

            let temp = if denom.abs() > 1e-15 {
                ndu[r][j - 1] / denom
            } else {
                0.0
            };

            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // 0차 도함수 = basis 값
    for r in 0..=p {
        ders[0][r] = ndu[r][p];
    }

    // Algorithm A2.3: 도함수 계산
    let mut a = vec![vec![0.0; p + 1]; 2];

    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        let mut rk: isize;
        let mut pk: isize;

        for k in 1..=nder {
            let mut d = 0.0;
            rk = r as isize - k as isize;
            pk = p as isize - k as isize;

            // 첫 항
            if rk >= 0 {
                let denom = ndu[(pk + 1) as usize][rk as usize];
                a[s2][0] = if denom.abs() > 1e-15 {
                    a[s1][0] / denom
                } else {
                    0.0
                };
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            } else {
                a[s2][0] = 0.0;
            }

            // 중간 항
            let j1 = if rk >= 0 { 1 } else { (-rk) as usize };
            let j2 = if r as isize - 1 <= pk { k - 1 } else { p - r };

            for j in j1..=j2 {
                let idx = (rk + j as isize) as usize;
                let denom = ndu[(pk + 1) as usize][idx];
                a[s2][j] = if denom.abs() > 1e-15 {
                    (a[s1][j] - a[s1][j - 1]) / denom
                } else {
                    0.0
                };
                d += a[s2][j] * ndu[idx][pk as usize];
            }

            // 마지막 항
            if r as isize <= pk {
                let idx = (rk + k as isize) as usize; // = r
                let denom = ndu[(pk + 1) as usize][idx];
                a[s2][k] = if denom.abs() > 1e-15 {
                    -a[s1][k - 1] / denom
                } else {
                    0.0
                };
                d += a[s2][k] * ndu[idx][pk as usize];
            } else {
                a[s2][k] = 0.0;
            }

            ders[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // 스케일링
    let mut r_fact = p as f64;
    for k in 1..=nder {
        for r in 0..=p {
            ders[k][r] *= r_fact;
        }
        r_fact *= (p - k) as f64;
    }
    ders
}
```
---
