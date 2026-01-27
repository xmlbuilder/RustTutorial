# Bezier Reduce Degree
🎯 전체 개념 요약
- Bezier 차수 감소는 다음 문제를 푸는 것이다:
```math
\mathrm{degree\  }p\mathrm{\  Bezier\  curve\  }B_p(t)\quad \Rightarrow \quad \mathrm{degree\  }p-1\mathrm{\  Bezier\  curve\  }B_{p-1}(t)
```
- 형상을 최대한 유지하면서 control point를 줄여야 한다.
- 그런데 Bezier basis는 다음 관계를 가진다:
```math
B_i^{p-1}(t)=\frac{p-i}{p}B_i^p(t)+\frac{i+1}{p}B_{i+1}^p(t)
```
- 이걸 뒤집으면:
```math
B_i^p(t)=\frac{p}{p-i}B_i^{p-1}(t)-\frac{i}{p-i}B_{i-1}^{p-1}(t)
```
- 이게 바로 reduce_coeff_left의 원천이다.
- 또 다른 형태:
```math
B_{i+1}^p(t)=\frac{p}{i+1}B_i^{p-1}(t)-\frac{p-i-1}{i+1}B_{i+1}^{p-1}(t)
```
- 이게 reduce_coeff_right의 원천이다.
- 즉:
    - reduce_coeff_left = 왼쪽에서부터 Q[i]를 계산할 때 필요한 계수
    - reduce_coeff_right = 오른쪽에서부터 Q[i]를 계산할 때 필요한 계수

### 🟥 1. reduce_coeff_left(p, i) 수식 정리
- 코드:
```rust
fn reduce_coeff_left(p: usize, i: usize) -> (Real, Real) {
    let denom = (p - i) as Real;
    ((p as Real) / denom, -((i as Real) / denom))
}
```

### 수식:
```math
Q_i=\alpha _iP_i+\omega _iQ_{i-1}
```
- 여기서:
```math
\alpha _i=\frac{p}{p-i}
```
```math
\omega _i=-\frac{i}{p-i}
```
- 왜 이런 계수가 나오냐?
- ✔ 유도 과정
Bezier basis 관계식:
```math
B_i^p(t)=\frac{p-i}{p}B_i^{p-1}(t)+\frac{i}{p}B_{i-1}^{p-1}(t)
```
- 이걸 뒤집으면:
```math
B_i^{p-1}(t)=\frac{p}{p-i}B_i^p(t)-\frac{i}{p-i}B_{i-1}^p(t)
```
- Bezier curve:
```math
C(t)=\sum _{i=0}^pP_iB_i^p(t)
```
- 차수 감소 후:
```math
C(t)=\sum _{i=0}^{p-1}Q_iB_i^{p-1}(t)
```
- 두 식을 비교하면:
```math
Q_i=\frac{p}{p-i}P_i-\frac{i}{p-i}Q_{i-1}
```
- 즉:
- $\alpha _i=\frac{p}{p-i}$
- $\omega _i=-\frac{i}{p-i}$
- 이게 그대로 코드에 들어간다.

##  🟦 2. reduce_coeff_right(p, i) 수식 정리
- 코드:
```rust
fn coeff_right(p: usize, i: usize) -> (Real, Real) {
    let denom = (i + 1) as Real;
    ((p as Real) / denom, -(((p - i - 1) as Real) / denom))
}
```

- 수식:
```math
Q_i=\beta _iP_{i+1}+\omega _iQ_{i+1}
```
- 여기서:
    - $\beta _i=\frac{p}{i+1}$
    - $\omega _i=-\frac{p-i-1}{i+1}$
- ✔ 유도 과정
- Bezier basis 관계식:
```math
B_{i+1}^p(t)=\frac{i+1}{p}B_i^{p-1}(t)+\frac{p-i-1}{p}B_{i+1}^{p-1}(t)
```
- 뒤집으면:
```math
B_i^{p-1}(t)=\frac{p}{i+1}B_{i+1}^p(t)-\frac{p-i-1}{i+1}B_{i+2}^p(t)
```
- 이걸 control point에 적용하면:
```math
Q_i=\frac{p}{i+1}P_{i+1}-\frac{p-i-1}{i+1}Q_{i+1}
```
- 즉:
    - $\beta _i=\frac{p}{i+1}$
    - $\omega _i=-\frac{p-i-1}{i+1}$
- 이게 그대로 코드에 들어간다.

### 🟩 3. 전체 그림
- Bezier 차수 감소는 다음 두 recurrence를 이용해
- 왼쪽에서 절반, 오른쪽에서 절반을 계산한다.
- 왼쪽 recurrence
```math
Q_i=\frac{p}{p-i}P_i-\frac{i}{p-i}Q_{i-1}
```
- 오른쪽 recurrence
```math
Q_i=\frac{p}{i+1}P_{i+1}-\frac{p-i-1}{i+1}Q_{i+1}
```
- 이 두 recurrence는 수학적으로 동일한 식을 다른 방향에서 푸는 것이다.
- 왜 두 방향을 모두 쓰냐?
    - 차수 감소는 ill-conditioned
    - 한쪽에서만 계산하면 오차가 커짐
    - 양쪽에서 계산해서 중간에서 만나게 해야 안정적
그래서:
    - odd degree → 가운데 점을 left/right 두 방식으로 계산해서 평균
    - even degree → 가운데 두 점을 left/right로 계산해서 error 추정

### 🟧 4. 코드와 수식의 1:1 대응
- reduce_coeff_left
- alf = p/(p-i)
- oma = -i/(p-i)

```math
Q_i=\alpha _iP_i+\omega _iQ_{i-1}
```

- reduce_coeff_right
- bet = p/(i+1)
- omb = -(p-i-1)/(i+1)

```math
Q_i=\beta _iP_{i+1}+\omega _iQ_{i+1}
```

---

## Bezier Reduce  Degree 전체 과정
- Bezier 차수 감소(reduce_degree) 전체 과정 수식과 알고리즘 정리
- 코드(reduce_degree_once)는 **The NURBS Book Algorithm A5.8(Bezier Degree Reduction)** 을 거의 그대로 구현한 형태.

## 🎯 전체 목표
- 입력:
- p차 Bezier 곡선
```math
C(t)=\sum _{i=0}^pP_iB_i^p(t)
```

- 출력:
- p−1차 Bezier 곡선
```math
\tilde {C}(t)=\sum _{i=0}^{p-1}Q_iB_i^{p-1}(t)
```
- 단,
    - 형상을 최대한 보존
    - 오차 최소화
    - 양쪽에서 계산해 중간에서 만나도록 (안정성 확보)

### 🟥 1. 핵심 수식: Bezier basis의 차수 관계식
- Bezier basis는 다음 관계를 가진다:
```math
B_i^p(t)=\frac{p-i}{p}B_i^{p-1}(t)+\frac{i}{p}B_{i-1}^{p-1}(t)
```
- 이걸 뒤집으면:
```math
B_i^{p-1}(t)=\frac{p}{p-i}B_i^p(t)-\frac{i}{p-i}B_{i-1}^p(t)
```
- 이게 왼쪽 recurrence의 원천이다.
- 또 다른 형태:
```math
B_i^{p-1}(t)=\frac{p}{i+1}B_{i+1}^p(t)-\frac{p-i-1}{i+1}B_{i+2}^p(t)
```
- 이게 오른쪽 recurrence의 원천이다.

### 🟧 2. 왼쪽 recurrence (reduce_coeff_left)
- 왼쪽에서부터 Q[i]를 계산하는 식:
```math
Q_i=\alpha _iP_i+\omega _iQ_{i-1}
```
- 계수:
```math
\alpha _i=\frac{p}{p-i},\quad \omega _i=-\frac{i}{p-i}
```
- 코드:
```rust
fn reduce_coeff_left(p, i) -> (alf, oma) {
    alf = p/(p-i)
    oma = -i/(p-i)
}
```

### 🟨 3. 오른쪽 recurrence (reduce_coeff_right)
- 오른쪽에서부터 Q[i]를 계산하는 식:
```math
Q_i=\beta _iP_{i+1}+\omega _iQ_{i+1}
```
- 계수:
```math
\beta _i=\frac{p}{i+1},\quad \omega _i=-\frac{p-i-1}{i+1}
```
- 코드:
```rust
fn reduce_coeff_right(p, i) -> (bet, omb) {
    bet = p/(i+1)
    omb = -(p-i-1)/(i+1)
}
```


### 🟩 4. 왜 양쪽 recurrence를 모두 쓰는가?
- Bezier 차수 감소는 ill-conditioned 문제라서  
    한쪽에서만 계산하면 오차가 커진다.
- 그래서:
    - 왼쪽에서 절반 계산
    - 오른쪽에서 절반 계산
    - 가운데에서 만나게 함
    - odd/even degree에 따라 처리 방식이 다름

### 🟦 5. odd degree (p = 2r + 1)
- 중앙 index = r
    - 왼쪽 recurrence로 Q[1]..Q[r−1] 계산
    - 오른쪽 recurrence로 Q[p−2]..Q[r+1] 계산
    - 중앙 Q[r]은 양쪽에서 계산한 두 점의 평균
```math   
Q_r=\frac{1}{2}(Q_r^{(L)}+Q_r^{(R)})
```
- 오차 계산:
```math
e=a\cdot |B_r^p(u)-B_{r+1}^p(u)|\cdot \| P_L-P_R\| 
```

### 🟫 6. even degree (p = 2r)
- 중앙이 두 개: r, r+1
    - 왼쪽 recurrence로 Q[1]..Q[r] 계산
    - 오른쪽 recurrence로 Q[p−2]..Q[r+1] 계산
    - 중앙 두 점은 그대로 사용
    - 오차는 다음으로 계산:
```math
e=B_{r+1}^p(u)\cdot \| P_{r+1}-\frac{Q_r+Q_{r+1}}{2}\|
``` 

### 🟪 7. 전체 알고리즘 흐름 (reduce_degree_once)
- p차 Bezier인지 확인
- 새 control point 배열 Q 생성 (크기 p)
- Q[0] = P[0], Q[p−1] = P[p]
- 왼쪽 recurrence로 Q[1..r] 계산
- 오른쪽 recurrence로 Q[p−2..r+1] 계산
- odd/even에 따라 중앙 처리
- 오차 계산
- (Q, error) 반환

### 🟦 8. 반복 차수 감소 (reduce_degree)
```rust
while cur.degree > target_deg {
    (next_ctrl, _) = reduce_degree_once()
    cur.ctrl = next_ctrl
    cur.degree -= 1
}
```

- 즉:
    - p → p−1
    - p−1 → p−2
    - …
    - target_deg까지 반복

## 🎯 요약
- Bezier 차수 감소는 basis function의 차수 관계식에서 출발
- 왼쪽 recurrence / 오른쪽 recurrence 두 개를 사용
- 중앙에서 만나게 해서 안정성 확보
- odd/even degree에 따라 중앙 처리 방식이 다름
- reduce_degree는 reduce_degree_once 를 반복 호출하는 구조

---

## 🟨 전체 구조를 그림으로 보면
- p = 2r + 1 (odd degree)
```
왼쪽 recurrence:   Q0  Q1  Q2  ...  Q(r-1)
                                       \
                                        \
                                         →  Q(r)  ← (평균)
                                        /
                                       /
오른쪽 recurrence: Q(p) Q(p-1) ... Q(r+1)
```
- Q(r−1) = 왼쪽에서만 계산
- Q(r+1) = 오른쪽에서만 계산
- Q(r) = 왼쪽 + 오른쪽 → 평균

## 🟩 even degree(p = 2r)에서는?
- even degree에서는 중앙이 두 개라서:
- Q[r]은 왼쪽에서 계산
- Q[r+1]은 오른쪽에서 계산
- 중앙 두 점을 평균하지 않음
- 대신 오차 계산 방식만 달라짐
- 즉, 중앙 두 점은 각각 한 번씩만 계산된다.


## 소스 코드
```rust



/// C의 B_cdegre와 동일한 1-step 차수감소: degree p -> p-1 (Bezier) + max error e
    pub fn reduce_degree_once(&self) -> Result<(Vec<Point4D>, Real), NurbsError> {
        let p = self.degree;
        if p < 1 {
            return Err(NurbsError::InvalidArgument {
                msg: "reduce_degree_once_c: degree < 1".into(),
            });
        }
        if self.ctrl.len() != p + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: "reduce_degree_once_c: ctrl length != degree+1 (not a single Bezier)".into(),
            });
        }

        // new degree = p-1 => new ctrl count = p
        let mut qw = vec![Point4D::zero(); p];

        // endpoints
        qw[0] = self.ctrl[0];
        qw[p - 1] = self.ctrl[p];

        let r = (p - 1) / 2;
        let mut e = 0.0;

        if p % 2 == 1 {
            // -------- Odd degree --------

            // left: i=1..=r-1
            if r >= 2 {
                for i in 1..=r - 1 {
                    let (alf, oma) = Self::reduce_coeff_left(p, i);
                    qw[i] = alf * self.ctrl[i] + oma * qw[i - 1];
                }
            }

            // right: i=p-2 down to r+1
            if p >= 3 {
                let mut i = p - 2;
                while i >= r + 1 {
                    let (bet, omb) = Self::reduce_coeff_right(p, i);
                    qw[i] = bet * self.ctrl[i + 1] + omb * qw[i + 1];
                    if i == 0 { break; }
                    i -= 1;
                }
            }

            // middle control point
            let (alf_r, oma_r) = Self::reduce_coeff_left(p, r);
            let pl = alf_r * self.ctrl[r] + oma_r * qw[r - 1];

            let (bet_r, omb_r) = Self::reduce_coeff_right(p, r);
            let pr = bet_r * self.ctrl[r + 1] + omb_r * qw[r + 1];

            qw[r] = (pl + pr) * 0.5;

            // error (C 그대로)
            let u = 0.5 * (1.0 - (1.0 / (p as Real)).sqrt());
            let b  = on_bernstein(p, r, u);
            let b1 = on_bernstein(p, r + 1, u);
            let dw = pl.distance_de_homogenized(&pr);

            let a = 0.5 * ((p - r) as Real) / (p as Real);
            e = a * (b - b1).abs() * dw;
        } else {
            // -------- Even degree --------

            // left: i=1..=r
            if r >= 1 {
                for i in 1..=r {
                    let (alf, oma) = Self::reduce_coeff_left(p, i);
                    qw[i] = alf * self.ctrl[i] + oma * qw[i - 1];
                }
            }

            // right: i=p-2 down to r+1
            if p >= 3 {
                let mut i = p - 2;
                while i >= r + 1 {
                    let (bet, omb) = Self::reduce_coeff_right(p, i);
                    qw[i] = bet * self.ctrl[i + 1] + omb * qw[i + 1];
                    if i == 0 { break; }
                    i -= 1;
                }
            }

            // error (C 그대로)
            let u = (r as Real + 1.0) / (p as Real);
            let b1 = on_bernstein(p, r + 1, u);

            let pl = (qw[r] + qw[r + 1]) * 0.5;
            let dw = self.ctrl[r + 1].distance_de_homogenized(&pl);
            e = b1 * dw;
        }
        Ok((qw, e))
    }

    pub fn reduce_degree_curve(&mut self, target_deg: Degree) -> Self {
        let ctrl = self.reduce_degree(target_deg);
        Self {
            dim: self.dim,
            degree: target_deg as usize,
            ctrl,
        }
    }

    
    ///  target_deg까지 반복 감소 (내부적으로 1-step B_cdegre를 반복)
    /// - 기존 시그니처 유지(반환 Vec<Point4D>)
    pub fn reduce_degree(&mut self, target_deg: Degree) -> Vec<Point4D> {
        let mut cur = self.clone();

        while cur.degree > target_deg as usize {
            match cur.reduce_degree_once() {
                Ok((next_ctrl, _e)) => {
                    cur.ctrl = next_ctrl;
                    cur.degree -= 1;
                }
                Err(_) => {
                    // 기존 코드 스타일이 Result가 아니라서, 실패 시 현재 상태 반환
                    // (원하면 여기서 panic/로그로 바꿔도 됨)
                    return self.ctrl.to_vec();
                }
            }
        }
        cur.ctrl
    }

    /// BezierCurve : reduce_degree_curve 차수 감소된 새 곡선 생성
    pub fn reduce_degree_curve(&mut self, target_deg: Degree) -> Self {
        let ctrl = self.reduce_degree(target_deg);
        Self {
            dim: self.dim,
            degree: target_deg as usize,
            ctrl,
        }
    }
```
### 테스트 코드
```rust
// tests/bezier_reduce_degree_tests.rs

use nurbslib::core::bezier_curve::BezierCurve;
use nurbslib::core::prelude::{Point4D, Real};

fn max_dev_samples(a: &BezierCurve, b: &BezierCurve, samples: usize) -> Real {
    let mut maxd = 0.0;
    for k in 0..=samples {
        let t = k as Real / samples as Real;
        let pa = a.point_at(t);
        let pb = b.point_at(t);
        let d = pa.distance(&pb);
        if d > maxd {
            maxd = d;
        }
    }
    maxd
}

#[test]
fn reduce_degree_curve_odd_degree_endpoints_and_error_bound() {
    // p=5 (odd), ctrl=6
    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 2.0, 0.0, 1.0),
        Point4D::new(2.0, 3.0, 1.0, 1.0),
        Point4D::new(3.0, 2.0, 2.0, 1.0),
        Point4D::new(4.0, 1.0, 2.0, 1.0),
        Point4D::new(5.0, 0.0, 0.0, 1.0),
    ];
    let c = BezierCurve::new(ctrl);
    assert_eq!(c.degree, 5);

    // 1-step 정석 감소 + e
    let (q_ctrl, e) = c.reduce_degree_once().expect("reduce_degree_once failed");
    let reduced = BezierCurve::new(q_ctrl);

    // degree/ctrl length
    assert_eq!(reduced.degree, 4);
    assert_eq!(reduced.ctrl.len(), 5);

    // endpoints must match exactly
    assert_eq!(reduced.ctrl[0], c.ctrl[0]);
    assert_eq!(reduced.ctrl[reduced.ctrl.len() - 1], c.ctrl[c.ctrl.len() - 1]);

    // sampled deviation should be <= e (약간의 수치 여유)
    let dev = max_dev_samples(&c, &reduced, 64);
    assert!(
        dev <= e * 1.000_001 + 1e-10,
        "dev {} > e {} (odd degree)",
        dev,
        e
    );
}

#[test]
fn reduce_degree_curve_even_degree_endpoints_and_error_bound() {
    // p=4 (even), ctrl=5
    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(0.5, 2.0, 0.0, 1.0),
        Point4D::new(2.0, 4.0, 1.0, 1.0),
        Point4D::new(3.5, 2.0, 2.0, 1.0),
        Point4D::new(4.0, 0.0, 0.0, 1.0),
    ];
    let c = BezierCurve::new(ctrl);
    assert_eq!(c.degree, 4);

    let (q_ctrl, e) = c.reduce_degree_once().expect("reduce_degree_once_c failed");
    let reduced = BezierCurve::new(q_ctrl);

    assert_eq!(reduced.degree, 3);
    assert_eq!(reduced.ctrl.len(), 4);

    assert_eq!(reduced.ctrl[0], c.ctrl[0]);
    assert_eq!(reduced.ctrl[reduced.ctrl.len() - 1], c.ctrl[c.ctrl.len() - 1]);

    let dev = max_dev_samples(&c, &reduced, 64);
    assert!(
        dev <= e * 1.000_001 + 1e-10,
        "dev {} > e {} (even degree)",
        dev,
        e
    );
}

#[test]
fn reduce_degree_curve_multi_step_degree_and_endpoints() {
    // p=6 -> target=3 (multi-step)
    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 3.0, 0.0, 1.0),
        Point4D::new(2.0, 5.0, 1.0, 1.0),
        Point4D::new(3.0, 5.0, 2.0, 1.0),
        Point4D::new(4.0, 3.0, 3.0, 1.0),
        Point4D::new(5.0, 1.0, 2.0, 1.0),
        Point4D::new(6.0, 0.0, 0.0, 1.0),
    ];
    let mut c = BezierCurve::new(ctrl);
    assert_eq!(c.degree, 6);

    // reduce_degree_curve는 Result가 아니라면, 네 구현에 맞춰 호출만 바꿔.
    let reduced = c
        .reduce_degree_curve(3);

    assert_eq!(reduced.degree, 3);
    assert_eq!(reduced.ctrl.len(), 4);

    // endpoints 유지
    assert_eq!(reduced.ctrl[0], c.ctrl[0]);
    assert_eq!(reduced.ctrl[reduced.ctrl.len() - 1], c.ctrl[c.ctrl.len() - 1]);
}

#[test]
fn reduce_degree_curve_rational_keeps_endpoints_and_runs() {
    // rational Bezier (w varies)
    // p=3 -> 2
    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 2.0, 0.0, 0.5),
        Point4D::new(2.0, 2.0, 1.0, 2.0),
        Point4D::new(3.0, 0.0, 0.0, 1.0),
    ];
    let c = BezierCurve::new(ctrl);
    assert_eq!(c.degree, 3);

    let (q_ctrl, _e) = c.reduce_degree_once().expect("reduce_degree_once_c failed");
    let reduced = BezierCurve::new(q_ctrl);

    assert_eq!(reduced.degree, 2);
    assert_eq!(reduced.ctrl.len(), 3);

    // endpoints (homogeneous) must match
    assert_eq!(reduced.ctrl[0], c.ctrl[0]);
    assert_eq!(reduced.ctrl[reduced.ctrl.len() - 1], c.ctrl[c.ctrl.len() - 1]);

    // 추가로 샘플링이 NaN 없이 동작하는지 확인
    let dev = max_dev_samples(&c, &reduced, 64);
    assert!(dev.is_finite(), "rational deviation is not finite");
}

```
---
