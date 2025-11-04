# Bezier Curve

베지어 곡선 및 함수의 핵심 연산들이 잘 구현되어 있습니다.  
아래에 수식 정리와 함께 reparameterize() 함수의 구현도 제안.

## ✅ 핵심 수식 정리
### 1. 🎯 베지어 곡선 평가

$$
P(u)=\sum _{i=0}^nB_i^n(u)\cdot P_i\quad \mathrm{where}\quad B_i^n(u)={n \choose i}u^i(1-u)^{n-i}
$$

- evaluate_point() → 유클리드 좌표
- evaluate_cpoint() → 동차 좌표
- evaluate_cpoint_rational() → 가중치 포함 rational 평가

### 2. 📈 차수 상승 (degree elevation)

$$
P_i'=\sum _{j=0}^nE_{ij}\cdot P_j\quad \mathrm{where}\quad E_{ij}=\mathrm{degree\  elevation\  matrix}
$$

- elevate_degree() → 정확한 행렬 기반 구현

### 3. 📉 차수 감소 (degree reduction)
- 근사 방식: $P_i'=P(t_i), where \quadt_i=\frac{i}{q}$
- reduce_degree() → rational 평가 기반 보간
- 정확한 방식은 least-squares 기반이지만, 현재는 보간 기반으로 충분

### 4. ✂️ 분할 (de Casteljau)
- split() → de Casteljau 알고리즘 기반
- 수식:

$$
P_i^{(k)}=(1-u)P_i^{(k-1)}+uP_{i+1}^{(k-1)}
$$


### 5. 📐 곡선 곱셈 (dot, cross)
- dot():

$$
(P\cdot Q)_i=\sum _{j=\max (0,i-q)}^{\min (p,i)}P_j\cdot Q_{i-j}
$$

- cross():

$$
(P\times Q)_i=\sum _{j=\max (0,i-q)}^{\min (p,i)}P_j\times Q_{i-j}
$$


### 6. 📏 길이 근사 (arc length)
- bezier_arc_len() → chord vs polygon 길이 비교 후 재귀 분할
- 수식:

$$
\mathrm{length}\approx \sum _{i=0}^{n-1}\| P_{i+1}-P_i\| \quad \mathrm{vs}\quad \| P_n-P_0\|
$$ 

## 🧮 BezierFunction 수식
### 1. 평가

$$
f(u)=\sum _{i=0}^nB_i^n(u)\cdot f_i
$$

### 2. 곱셈

$$
(f\cdot g)_i=\sum _{j=\max (0,i-q)}^{\min (p,i)}M_{i,j}\cdot f_j\cdot g_{i-j}\quad \mathrm{where}\quad M_{i,j}=\mathrm{product\  matrix\  coefficient}
$$

### 3. 차수 상승

$$
f_i'=\sum _{j=0}^p\sum _{k=0}^t\frac{{p \choose j}{t \choose k}}{{p+t \choose i}}f_j\quad \mathrm{where}\quad i=j+k
$$

## 🔧 구현: reparameterize(f: &BezierFunction)
### 🎯 목적
- 기존 베지어 곡선 P(u)를 함수 $f(u)\in [0,1]$ 로 재파라미터화
- 즉, $P(f(u))$ 를 베지어 곡선으로 근사

### 🧠 아이디어
- $P(f(u))=\sum _{i=0}^nP_iB_i^n(f(u))$
- $B_i^n(f(u))$ 를 베지어 함수로 근사 → 곱셈으로 전개
- 최종적으로:

$$
P'(u)=\sum _{i=0}^nP_i\cdot B_i^n(f(u))\quad \mathrm{→\  베지어\  곡선}
$$


## ✅ 구현 예시
```rust
pub fn reparameterize(&self, f: &BezierFunction) -> BezierCurve {
    
    let n = self.degree;
    let mut result = vec![CPoint::zero(); f.degree + n + 1];

    for i in 0..=n {
        let bi = BezierFunction {
            degree: n,
            coeffs: (0..=n).map(|j| if j == i { 1.0 } else { 0.0 }).collect(),
        };
        let bi_f = bi.multiply(f); // B_i^n(f(u)) as BezierFunction
        for (j, coeff) in bi_f.coeffs.iter().enumerate() {
            result[j].x += coeff * self.ctrl[i].x;
            result[j].y += coeff * self.ctrl[i].y;
            result[j].z += coeff * self.ctrl[i].z;
            result[j].w += coeff * self.ctrl[i].w;
        }
    }

    BezierCurve {
        degree: f.degree + n,
        ctrl: result,
    }
}
```





