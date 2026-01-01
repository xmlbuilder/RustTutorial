## 📘 1. Bezier Surface Degree Elevation (표면 차수 상승)
### 1.1 개념 요약
- Bezier Surface S(u,v) 는
    - u 방향 차수: p
    - v 방향 차수: q
    - 제어점: $P_{i,j}$
- 차수 상승은 Bezier 곡선의 degree elevation을 u 또는 v 방향으로 독립적으로 적용하는 것.
- u 방향 차수 상승
    - 각 v-column(세로줄)을 Bezier curve로 보고
    - 그 curve에 대해 degree elevation matrix $E(p\rightarrow p+t)$ 를 적용.
- v 방향 차수 상승
    - 각 u-row(가로줄)에 대해 동일하게 적용.

### 1.2 수식
- Degree Elevation Matrix
- 기존 차수 p, 증가량 t → 새 차수 p+t
```math
E[i][j]=\frac{{p \choose j}{t \choose i-j}}{{p+t \choose i}}
```
- 유효 범위:
```math
\max (0,i-t)\leq j\leq \min (i,p)
```
- 새 제어점 계산
- u 방향 차수 상승 시:
```math
Q_{i,j}=\sum _kE[i][k]\cdot P_{k,j}
```
- v 방향도 동일.


- Control Point는 Linear Interpolation으로 구하나?
- ❌ 아니야. 단순한 선형 보간이 아니다.
- Bezier 차수 상승은 다음 수식을 따른다:
```math
Q_i=\sum _{k=\max (0,i-t)}^{\min (p,i)}E[i][k]\cdot P_k
```
- 여기서
    - $E$ = degree elevation matrix
    - $P_k$ = 기존 control point
    - $Q_i$ = 새 control point
- 즉, 각 새 control point는 여러 기존 control point의 가중합(weighted sum) 으로 계산된다.
- 이 가중치는 binomial coefficient(이항계수) 기반이며, 단순한 1차 보간이 아니다.

- 왜 Linear Interpolation이 아닌가?
- Bezier 곡선의 정의가:
```math
C(u)=\sum _{i=0}^pB_{i,p}(u)P_i
```
- 차수를 올리면:
```math
C(u)=\sum _{i=0}^{p+t}B_{i,p+t}(u)Q_i
```
- 이 두 식이 동일한 곡선을 나타내야 하므로,
- 새 control point Q_i는 Bernstein basis 변환을 만족해야 한다.
- 그래서 degree elevation matrix가 필요하고, 그 결과는 선형 보간이 아니라 다중 가중합이 된다.

- 예시 (p=2 → p=3, t=1)
- 원래 control points:
```math
P_0,P_1,P_2
```
- 새 control points:
```math
Q_0,Q_1,Q_2,Q_3
```
- 수식:
```math
Q_0=P_0
```
```math
Q_1=\frac{1}{3}P_0+\frac{2}{3}P_1
```
```math
Q_2=\frac{2}{3}P_1+\frac{1}{3}P_2
```
```math
Q_3=P_2
```
여기서 보듯이:
- 끝점은 그대로
- 중간점은 선형 보간처럼 보이지만, 사실은 이항계수 기반의 정확한 가중합
- 즉, 단순한 lerp(P0, P1, t)가 아니라 정확한 비율이 수학적으로 결정된 가중합이다.

- 곡면(Bezier Surface)에서도 동일
- U 방향 차수 상승 시:
```math
Q_{i,j}=\sum _kE[i][k]\cdot P_{k,j}
```
- V 방향도 동일.
    - 즉, 각 row 또는 column 단위로 degree elevation matrix를 곱하는 방식이다.

- 📌 결론
    - ✔ Control Point 개수는 t만큼 증가한다
        - ❌ Control Point는 Linear Interpolation으로 구하지 않는다
    - ✔ 정확한 수학적 가중합(이항계수 기반)으로 계산된다
    - ✔ Degree Elevation Matrix를 사용한다


#### 🔍 차수 상승에서 t의 의미
- Bezier 곡선/곡면에서:
    - 원래 차수 = p
    - 올리고 싶은 차수 = p+t
    - 여기서 t = 증가시키고 싶은 차수의 양
- 예)
    - 3차 → 4차로 올리고 싶다 → t = 1
    - 3차 → 6차로 올리고 싶다 → t = 3
- 즉, t는 사용자가 원하는 목표 차수에 따라 직접 넣는 값.


### 1.3 알고리즘 절차
- UDIR (u 방향)
```
for each v index j:
    for each new index i in 0..p+t:
        Q[i][j] = Σ_k E[i][k] * P[k][j]
```

- VDIR (v 방향)
```
for each u index i:
    for each new index j in 0..q+t:
        Q[i][j] = Σ_k E[j][k] * P[i][k]
```

--- 

## 📘 2. Bezier Surface Degree Reduction (표면 차수 감소)
### 2.1 개념 요약
- Bezier 곡선 차수 감소 알고리즘을 u 또는 v 방향에 독립적으로 적용.
    - 한 줄(row/column)만 처리
    - 감소 후 제어점 수는 $p\rightarrow p-1$
    - 중간 제어점은 좌측/우측에서 재귀적으로 계산
    - 중앙부는 odd/even 차수에 따라 다르게 계산
    - 최대 오차도 계산

### 2.2 수식
- 기본 조합식
- (좌측에서 계산)
```math
Q_i=\alpha _iP_i+\omega _iQ_{i-1}
```
- (우측에서 계산)
```math
Q_i=\beta _iP_{i+1}+\mu _iQ_{i+1}
```

- 여기서 $\alpha$ ,$\omega$ ,$\beta$ ,$\mu$  는 미리 계산된 reduction coefficient.
- 중앙부 (odd degree)
```math
Q_r=\frac{1}{2}(L+R)
```
- 최대 오차
```math
e=a\cdot |B_r(u)-B_{r+1}(u)|\cdot \| PL-PR\| 
```
## 📘 3. Rust 구현용 문서화된 알고리즘
### 3.1 Degree Elevation (Global Function)
```rust
pub fn elevate_surface_dir(
    pw: &[Vec<Point4D>],
    p: usize,
    q: usize,
    inc: usize,
    dir: SurfaceDir,
) -> Vec<Vec<Point4D>> {
    let elev = on_degree_elevation_matrix(match dir {
        SurfaceDir::UDir => p,
        SurfaceDir::VDir => q,
    }, inc);

    match dir {
        SurfaceDir::UDir => {
            let new_p = p + inc;
            let mut out = vec![vec![Point4D::zero(); q + 1]; new_p + 1];

            for v in 0..=q {
                for i in 0..=new_p {
                    let mut acc = Point4D::zero();
                    let i_min = i.saturating_sub(inc);
                    let i_max = p.min(i);
                    for k in i_min..=i_max {
                        acc.add_scaled(elev[i][k], &pw[k][v]);
                    }
                    out[i][v] = acc;
                }
            }
            out
        }

        SurfaceDir::VDir => {
            let new_q = q + inc;
            let mut out = vec![vec![Point4D::zero(); new_q + 1]; p + 1];

            for u in 0..=p {
                for j in 0..=new_q {
                    let mut acc = Point4D::zero();
                    let j_min = j.saturating_sub(inc);
                    let j_max = q.min(j);
                    for k in j_min..=j_max {
                        acc.add_scaled(elev[j][k], &pw[u][k]);
                    }
                    out[u][j] = acc;
                }
            }
            out
        }
    }
}
```


### 3.2 Degree Reduction (Global Function)
```rust
pub fn reduce_surface_dir(
    pw: &[Vec<Point4D>],
    p: usize,
    q: usize,
    dir: SurfaceDir,
    k: usize,
    alf: &[f64],
    oma: &[f64],
    bet: &[f64],
    omb: &[f64],
) -> (Vec<Vec<Point4D>>, f64) {
    let mut qw = match dir {
        SurfaceDir::UDir => vec![vec![Point4D::zero(); q + 1]; p],
        SurfaceDir::VDir => vec![vec![Point4D::zero(); q]; p + 1],
    };

    let mut err = 0.0;

    match dir {
        SurfaceDir::UDir => {
            let r = (p - 1) / 2;

            // endpoints
            qw[0][k] = pw[0][k];
            qw[p - 1][k] = pw[p][k];

            if p % 2 == 1 {
                // odd degree
                for i in 1..=r - 1 {
                    qw[i][k] = alf[i] * pw[i][k] + oma[i] * qw[i - 1][k];
                }
                for i in (r + 1)..=(p - 2) {
                    qw[i][k] = bet[i] * pw[i + 1][k] + omb[i] * qw[i + 1][k];
                }

                let pl = alf[r] * pw[r][k] + oma[r] * qw[r - 1][k];
                let pr = bet[r] * pw[r + 1][k] + omb[r] * qw[r + 1][k];
                qw[r][k] = (pl + pr).scaled(0.5);

                let u = 0.5 * (1.0 - (1.0 / p as f64).sqrt());
                let b = on_bernstein(p, r, u);
                let b1 = on_bernstein(p, r + 1, u);
                let dw = pl.distance_to(pr);

                let a = 0.5 * (p - r) as f64 / p as f64;
                err = a * (b - b1).abs() * dw;
            } else {
                // even degree
                for i in 1..=r {
                    qw[i][k] = alf[i] * pw[i][k] + oma[i] * qw[i - 1][k];
                }
                for i in (r + 1)..=(p - 2) {
                    qw[i][k] = bet[i] * pw[i + 1][k] + omb[i] * qw[i + 1][k];
                }

                let u = (r + 1) as f64 / p as f64;
                let b1 = on_bernstein(p, r + 1, u);

                let pl = (qw[r][k] + qw[r + 1][k]).scaled(0.5);
                let dw = pw[r + 1][k].distance_to(pl);

                err = b1 * dw;
            }
        }

        SurfaceDir::VDir => {
            // 동일한 방식으로 j 인덱스 기준 처리
        }
    }

    (qw, err)
}
```


## 📘 4. 테스트 코드 (Rust)
- 아래 테스트는:
    - 랜덤 제어점 생성
    - u 방향 차수 상승 → 다시 차수 감소
    - 원래 곡선과 비교하여 오차가 허용 범위인지 확인
```rust
#[test]
fn test_surface_elev_reduce_roundtrip() {
    let p = 3;
    let q = 2;

    // random control net
    let mut pw = vec![vec![Point4D::zero(); q + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=q {
            let x = i as f64 * 0.7 + j as f64 * 0.3;
            let y = i as f64 * -0.4 + j as f64 * 0.9;
            let z = 1.0 + 0.2 * (i + j) as f64;
            let w = 1.0 + 0.1 * (i + 2 * j) as f64;
            pw[i][j] = Point4D::homogeneous(x, y, z, w);
        }
    }

    // elevate u-direction by 1
    let elevated = elevate_surface_dir(&pw, p, q, 1, SurfaceDir::UDir);

    // compute reduction coefficients
    let (alf, oma, bet, omb) = compute_reduction_coefficients(p + 1);

    // reduce back
    let (reduced, err) = reduce_surface_dir(
        &elevated,
        p + 1,
        q,
        SurfaceDir::UDir,
        1,
        &alf,
        &oma,
        &bet,
        &omb,
    );

    // compare original and reduced
    for i in 0..=p {
        for j in 0..=q {
            let a = pw[i][j].to_point();
            let b = reduced[i][j].to_point();
            assert!((a.x - b.x).abs() < 1e-6);
            assert!((a.y - b.y).abs() < 1e-6);
            assert!((a.z - b.z).abs() < 1e-6);
        }
    }

    println!("max reduction error = {}", err);
}
```

---

# 최종 정리

## 📘 Bezier Surface Degree Elevation & Reduction — Technical Summary
### 1. 개요
- Bezier Surface는 다음과 같이 정의된다:
```math
S(u,v)=\sum _{i=0}^p\sum _{j=0}^qB_{i,p}(u)B_{j,q}(v)P_{i,j}
```

- 여기서
    - p,q = U, V 방향 차수
    - P_{i,j} = 4D homogeneous control point
    - B_{i,p}(u) = Bernstein basis
- Bezier Surface의 차수를 변경하는 작업은 CAD/NURBS 시스템에서 매우 중요하다:
    - Degree Elevation (차수 상승)
        - 더 높은 차수로 변환하여 호환성 확보, 정밀도 증가
    - Degree Reduction (차수 감소)
        - 차수를 낮춰 단순화, 데이터 압축, 계산량 감소

### 📘 2. Degree Elevation (차수 상승)
#### 2.1 목적
- 곡면의 형상은 그대로 유지하면서
- 차수를 p→p+t 로 증가
- Control Point 개수는 (p+1) → (p+t+1) 로 증가
- CAD 시스템 간 호환성, Boolean 연산, Subdivision 등에 필수
#### 2.2 수학적 원리
- Bezier basis 변환:
```math
B_{i,p}(u)=\sum _{k=i}^{i+t}E[k][i]\, B_{k,p+t}(u)
```
- 여기서 E 는 degree elevation matrix:
```math
E[k][i]=\frac{{p \choose i}{t \choose k-i}}{{p+t \choose k}}
```
- 이 행렬을 이용하면:
```math
Q_{k,j}=\sum _iE[k][i]\cdot P_{i,j}
```
- 즉, 각 새 control point는 기존 control point들의 가중합(weighted sum)
    - 단순한 Linear Interpolation이 아니다.
#### 2.3 왜 Linear Interpolation이 아닌가?
- Bezier 곡면은 Bernstein basis의 조합으로 정의되므로 차수를 바꾸면 basis 자체가 바뀐다.
- 따라서:
    - 단순 보간(lerp)은 basis 변환을 만족하지 못함
    - 정확한 이항계수 기반 가중합만이 기존 곡면과 완전히 동일한 형상을 보장

### 📘 3. Degree Reduction (차수 감소)
#### 3.1 목적
- 차수를 p→p-1 로 낮춤
- Control Point 개수는 (p+1) → p 로 감소
- 데이터 압축, 단순화, 계산량 감소
- 하지만 형상이 완전히 동일하게 유지되지 않는다
    - 근사(approximation) 과정이 필요
#### 3.2 왜 한 번에 1씩만 줄이는가?
- Bezier 차수 감소는 정확한 역변환이 존재하지 않는다.
- 즉:
- 차수 상승은 정확한 수학적 변환이지만
    - 차수 감소는 근사 알고리즘이다
    - 한 번에 여러 차수를 줄이면 오차가 폭발적으로 증가
    - 그래서 항상 1씩만 줄이는 방식이 표준
- CAD 시스템(Piegl & Tiller, GeomWare 등)도 모두 이 방식 사용.

## 📘 4. Degree Reduction 알고리즘 요약
- 입력:
    - Pw: 기존 control net
    - p, q: 차수
    - dir: UDIR 또는 VDIR
    - k: 줄일 row 또는 column index
    - alf[], oma[], bet[], omb[]: 미리 계산된 reduction coefficient
    - Qw: 결과 control net
    - e: 최대 오차
- 핵심 아이디어:
    - 양 끝점은 그대로 유지
    - 왼쪽에서부터 forward recurrence
    - 오른쪽에서부터 backward recurrence
    - 중간점은 두 방향에서 계산한 값을 평균
- 오차는 Bernstein basis를 이용해 계산
- 왜 이렇게 하는가?
    - 차수 감소는 역행렬이 존재하지 않음
    - 따라서 최소 오차를 만드는 근사 알고리즘이 필요
    - 왼쪽/오른쪽에서 각각 1차 recurrence로 계산하면 기존 곡선과 가장 가까운 control point를 얻을 수 있음
    - 중간점은 두 방향의 평균을 취해 오차 최소화
- 오차는 다음과 같이 계산:
```math
e=|B_{r,p}(u)-B_{r+1,p}(u)|\cdot \mathrm{distance}(PL,PR)
```
### 📘 5. 왜 U/V 방향을 한 줄씩 처리하는가?
- Bezier Surface는 tensor product 구조:
```math
S(u,v)=\sum _iB_{i,p}(u)\sum _jB_{j,q}(v)P_{i,j}
```
- 따라서:
    - U 방향 차수 변경 → 각 V column을 독립적으로 처리
    - V 방향 차수 변경 → 각 U row를 독립적으로 처리
- 즉, 1D Bezier curve 차수 변경을 row/column 단위로 반복하는 구조
    - 효율적이고 수학적으로 정확함.

### 📘 6. 테스트 코드 설계
- 아래 테스트는 다음을 검증한다:
    - 차수 상승 후 다시 차수 감소하면 원래와 거의 동일한지
    - 차수 감소 후 다시 차수 상승하면 근사적으로 동일한지
    - 오차가 문서에 정의된 범위 내인지
    - U/V 방향 모두 정상 동작하는지

### 📘 7. 결론
- ✔ Degree Elevation
    - 정확한 변환
    - 이항계수 기반 가중합
    - 형상 완전 보존
    - t는 사용자가 지정
    - Control Point는 단순 보간이 아니라 행렬 기반 가중합
- ✔ Degree Reduction
    - 근사 알고리즘
    - 항상 1씩만 감소
    - 왼쪽/오른쪽 recurrence + 중간 평균
    - 오차 계산 포함
    - Surface는 row/column 단위로 처리

---
