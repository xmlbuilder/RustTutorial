## on_reparam_power_basis_curve
- on_reparam_power_basis_curve 은 Power Basis Polynomial Curve 재파라미터화 
    (reparametrization) 함수다.
- 즉,
```math
u\in [a,b]\quad \mathrm{에서\  정의된}\quad C(u)
```
- 을
```math
t\in [ap,bp]\quad \mathrm{에서\  정의된}\quad \tilde {C}(t)
```
- 로 바꾸는 것.
- 이때 새로운 계수는 다음 변환으로 얻어진다:
```math
\tilde {C}(t)=C\left( \frac{(b-a)t+(apb-bpa)}{bp-ap}\right)
```

- 이를 행렬 곱으로 표현한 것이 C의 on_compute_reparam_matrix + on_apply_reparam_matrix.

- 같은 기하학적 곡선 C를, 다른 파라미터 구간에서 다시 표현하는 것.

- 예를 들어:
    - 원래 곡선:
```math
C(u),\quad u\in [a,b]
```

- 새 곡선:
```math
\tilde {C}(t),\quad t\in [a',b']
```
- 이때 기하학적으로는 완전히 같은 곡선.
- 즉, 어떤 대응 관계 $\phi (t)$ 를 통해:
```math
\tilde {C}(t)=C(\phi (t))
```
- 이 되도록 새로운 계수(새 power basis) 를 구하는 게 목적.

## 2. 파라미터 매핑 수식부터 제대로
- 우리가 원하는 건:
```math
t=a'\Rightarrow u=a
```
```math
t=b'\Rightarrow u=b
```
- 이 두 조건을 만족하는 선형 함수 $\phi (t)$ 를 찾는 것.
```math
u=\phi (t)=\alpha t+\beta
``` 
- 조건 대입:
- t=a' 일 때 u=a:
```math
a=\alpha a'+\beta
``` 
- t=b' 일 때 u=b:
```math
b=\alpha b'+\beta
``` 
- 두 식을 빼면:
```math
b-a=\alpha (b'-a')\Rightarrow \alpha =\frac{b-a}{b'-a'}
```
- 이제 $\beta$  는:
```math
a=\alpha a'+\beta \Rightarrow \beta =a-\alpha a'
```
- 정리하면:
```math
u=\phi (t)=\frac{b-a}{b'-a'}t+\left( a-\frac{b-a}{b'-a'}a'\right)
``` 
- 코드에서는 이걸:
```rust
c = (b-a)/(bp-ap);
d = (bp*a - ap*b)/(bp-ap);
u = c * t + d;
```

- 즉:
```math
u=ct+d
```

## 3. 곡선에 이 매핑을 적용하면?
- 원래 곡선은 power basis로:
```math
C(u)=\sum _{i=0}^pa_iu^i
```
- 여기서 $a_i$ 는 벡터 계수 (Point3D 같은 것).
- 이제 $u=ct+d$ 를 대입하면:
```math
\tilde {C}(t)=C(ct+d)=\sum _{i=0}^pa_i(ct+d)^i
```
- $(ct+d)^i$ 를 전개하면:
```math
(ct+d)^i=\sum _{j=0}^i{i \choose j}c^jd^{i-j}t^j
```
- 따라서:
```math
\tilde {C}(t)=\sum _{i=0}^pa_i\sum _{j=0}^i{i \choose j}c^jd^{i-j}t^j=\sum _{j=0}^p\left( \sum _{i=j}^pa_i{i \choose j}c^jd^{i-j}\right) t^j
```
- 여기서:
```math
\tilde {a}_j=\sum _{i=j}^pa_i{i \choose j}c^jd^{i-j}
```
- 이게 바로 새로운 power basis 계수.

- 코드의 on_compute_reparam_matrix + on_apply_reparam_matrix 는 이걸 행렬 곱으로 구현,

## 4. 이게 어디에 쓰이냐? (사용처)
- 이 재파라미터화는 생각보다 자주 쓰인다:
    - 곡선 도메인 정규화
    - 예: 곡선을 항상 [0,1] 에서 정의하고 싶을 때
    - 기존 곡선이 [a,b] 에서 정의되어 있으면 → [0,1] 로 재파라미터화
    - 두 곡선의 파라미터를 맞추고 싶을 때
    - 예: 곡선 A는 [0,1], 곡선 B는 [2,5] 에서 정의되어 있는데
- 이 둘을 같은 파라미터 t에서 비교/블렌딩/보간하고 싶을 때
`- 둘 중 하나를 재파라미터화해서 도메인을 맞춘다.
- 수치 안정성 / 알고리즘 요구사항
    - 어떤 알고리즘은 항상 [0,1] 또는 [-1,1] 도메인을 가정한다.
    - 이때 기존 곡선을 그 도메인으로 옮겨줘야 한다.
- 세그먼트 분할 후 재정의
    - 곡선을 잘라서 부분 구간만 사용할 때,
잘린 구간을 다시 [0,1] 로 정규화해서 쓰고 싶을 때.

## 5. Rust 테스트 코드
- 재파라미터화된 곡선이 원래 곡선과 기하학적으로 같은지를 확인하는 것.

### 5.1. 먼저 우리가 만든 함수들 정리
```rust
pub fn on_eval_power_basis_horner(aw: &[Point3D], u: f64) -> Point3D {
    let n = aw.len();
    if n == 0 {
        return Point3D { x: 0.0, y: 0.0, z: 0.0 };
    }
    let mut c = aw[n - 1];
    for i in (0..n - 1).rev() {
        c = c * u + aw[i];
    }
    c
}
```
```rust
pub fn on_compute_reparam_matrix(p: usize, a: f64, b: f64, ap: f64, bp: f64) -> Vec<Vec<f64>> {
    let mut rm = vec![vec![0.0; p + 1]; p + 1];

    let c = (b - a) / (bp - ap);
    let d = (bp * a - ap * b) / (bp - ap);

    rm[0][0] = 1.0;
    for i in 1..=p {
        rm[0][i] = d * rm[0][i - 1];
        rm[i][i] = c * rm[i - 1][i - 1];
    }

    let bin = on_pascal_triangle(p);

    for i in 1..p {
        let mut fact = rm[i][i];
        for j in (i + 1)..=p {
            fact *= d;
            rm[i][j] = bin[j][i] as f64 * fact;
        }
    }

    rm
}
```
```rust
fn on_pascal_triangle(n: usize) -> Vec<Vec<usize>> {
    let mut bin = vec![vec![0; n + 1]; n + 1];
    for i in 0..=n {
        bin[i][0] = 1;
        bin[i][i] = 1;
        for j in 1..i {
            bin[i][j] = bin[i - 1][j - 1] + bin[i - 1][j];
        }
    }
    bin
}
```
```rust
pub fn on_apply_reparam_matrix(
    rm: &[Vec<f64>],
    aw: &[Point3D],
) -> Vec<Point3D> {
    let p = aw.len() - 1;
    let mut bw = vec![Point3D { x: 0.0, y: 0.0, z: 0.0 }; p + 1];

    for i in 0..=p {
        let mut acc = Point3D { x: 0.0, y: 0.0, z: 0.0 };
        for j in i..=p {
            acc = acc + aw[j] * rm[i][j];
        }
        bw[i] = acc;
    }

    bw
}
```
```rust
pub fn on_reparam_power_basis_curve(
    coeffs: &[Point3D],
    old_domain: (f64, f64),
    new_domain: (f64, f64),
) -> Vec<Point3D> {
    let p = coeffs.len() - 1;
    let (a, b) = old_domain;
    let (ap, bp) = new_domain;

    let rm = on_compute_reparam_matrix(p, a, b, ap, bp);
    on_apply_reparam_matrix(&rm, coeffs)
}
```

### 5.2. 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(p: &Point3D, q: &Point3D, tol: f64) -> bool {
        (p.x - q.x).abs() <= tol &&
        (p.y - q.y).abs() <= tol &&
        (p.z - q.z).abs() <= tol
    }
```
```rust
    #[test]
    fn test_reparam_power_basis_curve_equivalence() {
        // 원래 곡선: C(u) = (1, u, u^2), u ∈ [0, 1]
        let coeffs = vec![
            Point3D { x: 1.0, y: 0.0, z: 0.0 }, // a0
            Point3D { x: 0.0, y: 1.0, z: 0.0 }, // a1
            Point3D { x: 0.0, y: 0.0, z: 1.0 }, // a2
        ];
        let old_domain = (0.0, 1.0);

        // 새 도메인: t ∈ [2, 5]
        let new_domain = (2.0, 5.0);

        // 재파라미터화된 계수
        let new_coeffs = on_reparam_power_basis_curve(&coeffs, old_domain, new_domain);

        // u = c t + d
        let (a, b) = old_domain;
        let (ap, bp) = new_domain;
        let c = (b - a) / (bp - ap);
        let d = (bp * a - ap * b) / (bp - ap);

        let tol = 1e-10;

        // 여러 t 샘플에서 C(c t + d)와 C̃(t)가 같은지 확인
        for i in 0..=10 {
            let t = ap + (bp - ap) * (i as f64) / 10.0;
            let u = c * t + d;

            let p_old = on_eval_power_basis_horner(&coeffs, u);
            let p_new = on_eval_power_basis_horner(&new_coeffs, t);

            assert!(
                approx_eq(&p_old, &p_new, tol),
                "Mismatch at t = {}: old={:?}, new={:?}",
                t, p_old, p_new
            );
        }
    }
}
```

- 이 테스트는 정말로 중요한 걸 확인:
    - **재파라미터화된 곡선이, 원래 곡선을 단지 파라미터만 바꿔서 표현한 것인지**

- 즉, 기하학적으로 완전히 동일한지를 검증.

## 6. 정리
- reparam_power_basis_curve 는  
    같은 곡선을 다른 파라미터 구간에서 다시 표현하는 함수다.
    - 수식의 핵심은:
    - 파라미터 매핑: $u=ct+d$
    - 곡선 대입: $\tilde {C}(t)=C(ct+d)$
    - 전개 후 새로운 계수 $\tilde {a}_j$ 를 얻는 것
   - Rust에서 on_compute_reparam_matrix + on_apply_reparam_matrix 로 구현.
- 사용처는:
    - 도메인 정규화
    - 곡선 간 파라미터 맞추기
    - 알고리즘 요구 도메인에 맞추기
    - 세그먼트 재정의 등
---
