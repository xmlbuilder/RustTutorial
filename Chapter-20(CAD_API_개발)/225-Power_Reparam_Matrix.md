# Power Reparam Matrix
🧩 두 함수의 핵심 차이 (한 줄 요약)
| Function                          | Purpose         | Reparam Formula          | Typical Use Case      |
|-----------------------------------|---------------------------------|-------------------------|-----------------------------|
|on_power_reparam_matrix_unit|Simple affine reparameterization |u = a + (b - a) * t|Normalize [0,1] → [a,b]|
|on_power_reparam_matrix     |General domain reparameterization|u = α·up + β     |Map [u0,u1] → [a,b] (full reparam) |

즉:
- affine 버전은 “t ∈ [0,1]” 기준의 단순한 치환
- 일반 버전은 임의의 old domain → new domain 변환

## 🧠 1) on_power_reparam_matrix_unit — 단순 affine 치환 전용
- ✔ 치환식
```math
u=a+(b-a)t
```
- 여기서 t ∈ [0,1].
- 즉, old domain이 [a,b]이고 new domain은 항상 [0,1].
- ✔ 공식
```math
d_i=\sum _{k=i}^pc_k{k \choose i}a^{k-i}(b-a)^i
```
- ✔ 행렬 형태
```math
R[i,k]={k \choose i}a^{k-i}(b-a)^i
```
- ✔ 용도
    - Bézier subdivision
    - 구간을 [0,1]로 정규화(normalization)
    - 단순한 affine mapping
    - trimming curve를 0~1 구간으로 맞출 때
    - power basis polynomial을 canonical domain으로 바꿀 때
- 즉, **old domain이 [a,b]로 고정되어 있고, new domain은 항상 [0,1]** 일 때 사용.

## 🧠 2) on_power_reparam_matrix — 일반적인 domain reparameterization
- ✔ 치환식
```math
u=\alpha up+\beta
``` 
- 조건:
    - up = a → u = u0
    - up = b → u = u1
- 따라서:
```math
\alpha =\frac{u_1-u_0}{b-a},\quad \beta =u_0-\alpha a
```
- ✔ 공식
```math
A'_j=\sum _{i=j}^pA_i{i \choose j}\beta ^{i-j}\alpha ^j
```
- ✔ 용도
    - 임의의 old domain [u0,u1] → new domain [a,b]
    - surface reparameterization
    - trimming domain remapping
    - NURBS patch normalization
    - power basis surface의 u,v domain을 바꿀 때
- 즉, 일반적인 domain 변환을 모두 처리할 수 있는 범용 버전.

## 🎯 결정적 차이 정리
- ✔ 차이 1 — 치환식이 다르다
- unit 버전:
```math
u=a+(b-a)t
```
- 항상 old domain = [a,b], new domain = [0,1]

- 일반 버전:
```math
u=\alpha up+\beta
``` 
- old domain = [u0,u1], new domain = [a,b]

- ✔ 차이 2 — 사용 목적이 다르다
    - unit 버전: 정규화(normalization)
    - 일반 버전: 임의 domain 재파라미터라이즈

- ✔ 차이 3 — 공식이 다르다
    - unit 버전은 β가 없고, α = (b−a) 고정
    - 일반 버전은 α, β 둘 다 존재

## 🧱 언제 어떤 함수를 써야 하나?
- ✔ trimming, surface reparameterization, patch domain 변경
    - on_power_reparam_matrix (일반 버전)
- ✔ Bézier subdivision, [a,b] → [0,1] 정규화
    - on_power_reparam_matrix_unit

---
## 소스 코드
```rust
fn on_power_reparam_matrix_unit(p: usize, a: Real, b: Real) -> Result<Matrix> {
    let n = p + 1;
    let s = b - a;
    if s == 0.0 {
        return Err(NurbsError::InvalidInput { msg : "degenerate interval (b-a)==0".into()});
    }

    let mut r = Matrix::with_dims(n, n);
    r.zero();

    for i in 0..=p {
        let si = s.powi(i as i32);
        for k in i..=p {
            let cki =  on_binomial_usize(k, i) as Real;
            let ap = a.powi((k - i) as i32);
            *r.at_mut(i as i32, k as i32) = cki * ap * si;
        }
    }
    Ok(r)
}
```
```rust
pub fn on_power_reparam_matrix(p: usize, u0: Real, u1: Real, a: Real, b: Real) -> Result<Matrix> {
    let denom = b - a;
    if denom.abs() <= 0.0 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_power_reparam_matrix: invalid new bounds (b-a == 0)".into(),
        });
    }

    let alpha = (u1 - u0) / denom;
    let beta  = u0 - alpha * a;

    // M: (p+1)x(p+1), row=j (new power), col=i (old power)
    let mut m = Matrix::with_dims(p + 1, p + 1);

    for j in 0..=p {
        let aj = alpha.powi(j as i32);
        for i in j..=p {
            let comb = on_binomial_usize(i, j) as Real;
            let bp = beta.powi((i - j) as i32);
            *m.at_mut(j as i32, i as i32) = comb * bp * aj;
        }
    }
    Ok(m)
}
```
---
