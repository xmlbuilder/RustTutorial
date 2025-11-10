# Power basis Bezier basis 변환


이 함수들은 수학적으로 정확하며, Power basis → Bézier basis 변환과 Bézier 곡선 평가를 올바르게 구현하고 있습니다.  
각 단계는 Bernstein 다항식과 이항 계수를 기반으로 하며, 수식적으로도 문제가 없습니다.


## 소스 코드
```rust
#[allow(unused)]
#[inline]
fn on_binom_3(k: i32) -> f64 {
    if k < 0 || k > 3 {
        return 0.0;
    }
    // It's enough to just use the last row of the Pascal table for n=3
    // [1, 3, 3, 1]
    match k {
        0 => 1.0,
        1 => 3.0,
        2 => 3.0,
        _ => 1.0,
    }
}
```
```rust
/// power basis (a0 + a1 t + a2 t^2 + a3 t^3) → cubic Bézier control values [b0..b3].
/// Formula: b_i = Σ_{k=0..i} C(i,k) / C(3,k) * a_k
pub fn on_power_to_bezier_deg3(a: &[f64; 4]) -> [f64; 4] {
    let n = 3usize;
    let mut b = [0.0f64; 4];
    for i in 0..=n {
        let mut s = 0.0;
        for k in 0..=i {
            s += a[k] * on_binom_f64(i, k) / on_binom_f64(n, k);
        }
        b[i] = s;
    }
    b
}
```
```rust
#[inline]
pub fn on_bernstein3(i: usize, t: f64) -> f64 {
    // B0..B3
    match i {
        0 => (1.0 - t).powi(3),
        1 => 3.0 * t * (1.0 - t).powi(2),
        2 => 3.0 * t * t * (1.0 - t),
        3 => t.powi(3),
        _ => 0.0,
    }
}
```
```rust
pub fn on_eval_power_deg3(a: &[f64; 4], t: f64) -> f64 {
    a[0] + a[1] * t + a[2] * t * t + a[3] * t * t * t
}
```
```rust
pub fn on_eval_bezier_deg3(b: &[f64; 4], t: f64) -> f64 {
    b[0] * on_bernstein3(0, t)
        + b[1] * on_bernstein3(1, t)
        + b[2] * on_bernstein3(2, t)
        + b[3] * on_bernstein3(3, t)
}
```
```rust
pub fn on_power_to_bezier_2d_4x2(ax: [[f64; 3]; 5]) -> [[f64; 3]; 5] {
    // 1) u-direction (deg4) transformation
    let mut tmp = [[0.0f64; 3]; 5];
    for j in 0..=2 {
        let in_u = [ax[0][j], ax[1][j], ax[2][j], ax[3][j], ax[4][j]];
        let out_u = on_power_to_bezier_deg4(in_u);
        for i in 0..=4 {
            tmp[i][j] = out_u[i];
        }
    }
    // 2) Transform in v direction (deg2)
    let mut bx = [[0.0f64; 3]; 5];
    for i in 0..=4 {
        let in_v = [tmp[i][0], tmp[i][1], tmp[i][2]];
        let out_v = on_power_to_bezier_deg2(in_v);
        for j in 0..=2 {
            bx[i][j] = out_v[j];
        }
    }
    bx
}
```

## 🧮 수학적 배경
### 1. Power basis vs Bézier basis
- Power basis:

$$
P(t)=a_0+a_1t+a_2t^2+a_3t^3
$$

- Bézier basis (Bernstein basis):

$$
P(t)=\sum _{i=0}^3b_iB_{i,3}(t)\quad \mathrm{where\  }B_{i,3}(t)={3 \choose i}t^i(1-t)^{3-i}
$$

- 변환 공식:

$$
b_i=\sum _{k=0}^i\frac{{i \choose k}}{{3 \choose k}}a_k
$$


## ✅ 함수별 수학적 검토
### on_binom_3(k)
- 고정된 n=3에 대해 이항 계수 반환
- 파스칼 삼각형의 마지막 행: [1,3,3,1]
- 수학적으로 정확하며, 효율적

### on_power_to_bezier_deg3(a)
- 변환 공식 구현:

$$
b_i=\sum _{k=0}^i\frac{{i \choose k}}{{3 \choose k}}a_k
$$

- on_binom_f64(i, k)는 일반 이항 계수 계산
- on_binom_3(k) 대신 on_binom_f64(3, k) 사용 → 일반화 가능
- ✅ 정확한 수식 기반 변환이며, Bézier 제어점 계산에 적합

### on_bernstein3(i, t)
- Bernstein 다항식 정의:

$$
B_{0,3}(t)=(1-t)^3\\ B_{1,3}(t)=3t(1-t)^2\\ B_{2,3}(t)=3t^2(1-t)\\ B_{3,3}(t)=t^3
$$

- ✅ 정확한 Bernstein 다항식 구현

### on_eval_power_deg3(a, t)
- Power basis 다항식 평가:

$$
P(t)=a_0+a_1t+a_2t^2+a_3t^3
$$

- ✅ 정확한 다항식 평가

### on_eval_bezier_deg3(b, t)
- Bézier 곡선 평가:

$$
P(t)=\sum _{i=0}^3b_iB_{i,3}(t)
$$

- ✅ 정확한 Bézier 곡선 평가


## 📌 단계별 요약

| 단계 구분             | 수식                                                                 | 의미 설명                                      |
|----------------------|----------------------------------------------------------------------|------------------------------------------------|
| 이항 계수 정의        | $\binom{n}{k} = \frac{n!}{k!(n-k)!}$                             | Bernstein 다항식과 변환 계수 계산의 기초       |
| Power → Bézier 변환   | $b_i = \sum_{k=0}^{i} \frac{\binom{i}{k}}{\binom{3}{k}} a_k$     | Power 계수 $a_k$를 Bézier 제어점 $b_i$로 변환 |
| Bernstein 다항식 정의 | $B_{i,3}(t) = \binom{3}{i} t^i (1 - t)^{3 - i}$                  | Bézier 곡선의 기저 함수                        |
| Power basis 평가      | $P(t) = a_0 + a_1 t + a_2 t^2 + a_3 t^3$                         | Power basis 다항식의 직접 평가                 |
| Bézier 곡선 평가      | $P(t) = \sum_{i=0}^{3} b_i B_{i,3}(t)$                           | Bézier 제어점과 Bernstein 기저로 곡선 평가     |


## ✅ nalgebra 기반 함수들
```rust
use nalgebra::{DVector};

#[inline]
fn binom_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut res = 1.0;
    for i in 1..=k {
        res *= (n - k + i) as f64 / (i as f64);
    }
    res
}

#[inline]
fn binom_3(k: i32) -> f64 {
    match k {
        0 => 1.0,
        1 | 2 => 3.0,
        3 => 1.0,
        _ => 0.0,
    }
}

pub fn power_to_bezier_deg3(a: &[f64; 4]) -> [f64; 4] {
    let a_vec = DVector::from_column_slice(a);
    let mut b_vec = DVector::zeros(4);
    let n = 3;

    for i in 0..=n {
        let mut s = 0.0;
        for k in 0..=i {
            let c = binom_f64(i, k) / binom_f64(n, k);
            s += c * a_vec[k];
        }
        b_vec[i] = s;
    }

    [b_vec[0], b_vec[1], b_vec[2], b_vec[3]]
}

#[inline]
pub fn on_nalgebra_bernstein3(i: usize, t: f64) -> f64 {
    match i {
        0 => (1.0 - t).powi(3),
        1 => 3.0 * t * (1.0 - t).powi(2),
        2 => 3.0 * t * t * (1.0 - t),
        3 => t.powi(3),
        _ => 0.0,
    }
}

pub fn on_nalgebra_eval_power_deg3(a: &[f64; 4], t: f64) -> f64 {
    let a_vec = DVector::from_column_slice(a);
    let t_vec = DVector::from_column_slice(&[1.0, t, t * t, t * t * t]);
    a_vec.dot(&t_vec)
}

pub fn on_nalgebra_eval_bezier_deg3(b: &[f64; 4], t: f64) -> f64 {
    let b_vec = DVector::from_column_slice(b);
    let b_basis = DVector::from_column_slice(&[
        on_nalgebra_bernstein3(0, t),
        on_nalgebra_bernstein3(1, t),
        on_nalgebra_bernstein3(2, t),
        on_nalgebra_bernstein3(3, t),
    ]);
    b_vec.dot(&b_basis)
}
```


## 📌 유지된 구조

| 항목 구분           | 구조 유지 여부 | 설명                                                   |
|--------------------|----------------|--------------------------------------------------------|
| 입력/출력 타입      | ✅             | `[f64; 4]` 배열 형태 그대로 유지                        |
| 이항 계수 계산      | ✅             | `binom_f64` 함수 그대로 사용                           |
| Bernstein 다항식    | ✅             | `bernstein3(i, t)` 함수 그대로 유지                    |
| 곡선 평가 방식      | ✅             | `nalgebra::DVector::dot()`으로 내적 계산 방식 유지     |

---



# on_power_to_bezier_2d_4x2

이 함수는 2차원 다항식 형태의 데이터를 Power basis → Bézier basis로 변환하는 구조이며, 수학적으로 매우 타당한 방식입니다.  
아래에 목적, 수식, 단계별 설명을 정리.


## 소스 코드
```rust
pub fn on_power_to_bezier_2d_4x2(ax: [[f64; 3]; 5]) -> [[f64; 3]; 5] {
    // 1) u-direction (deg4) transformation
    let mut tmp = [[0.0f64; 3]; 5];
    for j in 0..=2 {
        let in_u = [ax[0][j], ax[1][j], ax[2][j], ax[3][j], ax[4][j]];
        let out_u = on_power_to_bezier_deg4(in_u);
        for i in 0..=4 {
            tmp[i][j] = out_u[i];
        }
    }
    // 2) Transform in v direction (deg2)
    let mut bx = [[0.0f64; 3]; 5];
    for i in 0..=4 {
        let in_v = [tmp[i][0], tmp[i][1], tmp[i][2]];
        let out_v = on_power_to_bezier_deg2(in_v);
        for j in 0..=2 {
            bx[i][j] = out_v[j];
        }
    }
    bx
}
```

## 🎯 함수 목적
- 입력: ax: [[f64; 3]; 5] → 2D 다항식 계수 배열 (degree 4 in u, degree 2 in v)
- 출력: Bézier basis로 변환된 동일 구조의 2D 제어점 배열
- 목적:
### Power basis로 표현된 2D 다항식

$$
f(u,v)=\sum _{i=0}^4\sum _{j=0}^2a_{ij}u^iv^j
$$

- 를 Bézier basis로 표현된 곡면

$$
f(u,v)=\sum _{i=0}^4\sum _{j=0}^2b_{ij}B_{i,4}(u)B_{j,2}(v)
$$


## 🧮 수학적 단계
### ✅ Step 1: u 방향 변환 (degree 4)
- 각 열 j에 대해:

$$
\mathrm{in_{\mathnormal{u}}}=[a_{0j},a_{1j},a_{2j},a_{3j},a_{4j}]\mathrm{out_{\mathnormal{u}}}=\mathrm{Bézier\  변환}(in_u)b_{ij}^{(tmp)}=\mathrm{out_{\mathnormal{u}}}[i]
$$

- 이 단계는 v 고정 상태에서 u-방향 다항식을 Bézier basis로 변환

## ✅ Step 2: v 방향 변환 (degree 2)- 각 행 i에 대해:

$$
\mathrm{in_{\mathnormal{v}}}=[b_{i0}^{(tmp)},b_{i1}^{(tmp)},b_{i2}^{(tmp)}]\mathrm{out_{\mathnormal{v}}}=\mathrm{Bézier\  변환}(in_v)b_{ij}=\mathrm{out_{\mathnormal{v}}}[j]
$$

- 이 단계는 u 고정 상태에서 v-방향 다항식을 Bézier basis로 변환

## 📐 전체 수식 구조
### Power basis 표현:

$$
f(u,v)=\sum _{i=0}^4\sum _{j=0}^2a_{ij}u^iv^j
$$

### Bézier basis 표현:

$$
f(u,v)=\sum _{i=0}^4\sum _{j=0}^2b_{ij}B_{i,4}(u)B_{j,2}(v)\quad \mathrm{where\  }B_{i,n}(t)={n \choose i}t^i(1-t)^{n-i}
$$

## ✅ 수학적 검토 결과
- 정확성: 두 방향 모두 독립적으로 Bézier basis로 변환 → 수학적으로 타당
- 선형성: Bézier 변환은 선형 연산이므로 순차적 적용 가능
- 구조 보존: 입력과 출력 모두 동일한 shape ([[f64; 3]; 5]) 유지

---
