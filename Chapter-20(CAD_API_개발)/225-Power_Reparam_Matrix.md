# Power Reparam Matrix

🧩 두 함수의 핵심 차이 (한 줄 요약)

| Function | Purpose | Reparam Formula | Meaning |
|---|---|---|---|
| on_power_reparam_matrix_unit | Unit-domain reparameterization | u = a + (b-a)t | old polynomial f(u)를 new parameter t ∈ [0,1] 기준으로 재전개 |
| on_power_reparam_matrix | General-domain reparameterization | u = α·up + β | old polynomial f(u)를 new parameter up ∈ [a,b] 기준으로 재전개 |

즉:

- unit 버전은:

```math
f(u)\rightarrow f(a+(b-a)t)
```

- 일반 버전은:

```math
f(u)\rightarrow f(\alpha up+\beta)
```

---

## 🧠 1) on_power_reparam_matrix_unit 
- unit-domain reparameterization

## ✔ 치환식

```math
u = a + (b-a)t
```

여기서:

```math
t \in [0,1]
```

이고, `u`는 기존 polynomial의 변수이다.

즉 기존 함수가:

```math
f(u)=\sum_{k=0}^{p} c_k u^k
```

일 때 새 함수는:

```math
g(t)=f(a+(b-a)t)
```

이다.

따라서:

```math
g(t)=\sum_{i=0}^{p} d_i t^i
```

이고:

```math
d_i=\sum_{k=i}^{p} c_k {k \choose i} a^{k-i}(b-a)^i
```

이다.

---

## ✔ 행렬 형태

```math
R[i,k]={k \choose i}a^{k-i}(b-a)^i
```

단:

```math
i \le k
```

일 때만 값이 있고 나머지는 0이다.

즉 upper-right triangular matrix 구조를 가진다.

---

## ✔ 의미

이 함수는:

```text
old polynomial f(u)
```

를:

```text
new local parameter t ∈ [0,1]
```

기준 polynomial로 다시 표현한다.

즉:

```text
u-domain의 [a,b] 구간을
local parameter [0,1] 위에서 재표현
```

하는 것이다.

---

## ✔ 대표 사용처

- Bézier subdivision
- Bézier segment extraction
- curve segment `[a,b]`를 local `[0,1]` polynomial로 재표현
- trimming curve 일부 구간을 local parameter로 재정의
- power basis polynomial의 local segment 생성
- Bernstein subdivision 내부 처리

---

## 🧠 2) on_power_reparam_matrix 
- 일반 domain reparameterization

## ✔ 치환식

```math
u=\alpha up+\beta
```

조건:

```math
up=a \Rightarrow u=u0
```

```math
up=b \Rightarrow u=u1
```

따라서:

```math
\alpha=\frac{u1-u0}{b-a}
```

```math
\beta=u0-\alpha a
```

---

## ✔ 기존 함수

기존 polynomial:

```math
f(u)=\sum_{i=0}^{p} A_i u^i
```

를 새 변수 `up` 기준으로:

```math
g(up)=f(\alpha up+\beta)
```

로 바꾼다.

즉:

```math
g(up)=\sum_{j=0}^{p} A'_j up^j
```

이며:

```math
A'_j=\sum_{i=j}^{p}A_i{i \choose j}\beta^{i-j}\alpha^j
```

이다.

---

## ✔ 행렬 형태

```math
M[j,i]={i \choose j}\beta^{i-j}\alpha^j
```

단:

```math
j \le i
```

일 때만 값이 있고 나머지는 0이다.

즉 역시 upper-right triangular matrix 구조를 가진다.

---

## ✔ 대표 사용처

- 임의 domain remapping
- surface reparameterization
- trimming domain 변경
- NURBS patch normalization
- arbitrary parameter interval remapping
- power basis surface domain 변경

---

# 🎯 결정적 차이

`on_power_reparam_matrix_unit(p, a, b)` 는 사실상 다음 일반 함수와 동일하다.

```rust
on_power_reparam_matrix(p, a, b, 0.0, 1.0)
```

왜냐하면:

```math
up \in [0,1]
```

일 때:

```math
up=0 \Rightarrow u=a
```

```math
up=1 \Rightarrow u=b
```

이므로:

```math
u=a+(b-a)up
```

가 되기 때문이다.

즉 unit 버전은 일반 버전의 특수한 경우이다.

---

# 🧱 언제 어떤 함수를 써야 하나?

## ✔ on_power_reparam_matrix_unit

이럴 때 사용:

```text
원래 polynomial의 [a,b] 구간을
새 local parameter [0,1] 위에서 표현하고 싶다.
```

예:

```text
Bezier curve의 u=[0.25,0.75] 구간을
새 Bezier [0,1]로 만들기
```

---

## ✔ on_power_reparam_matrix

이럴 때 사용:

```text
새 parameter domain이 [0,1]이 아니라
임의의 [a,b]일 때
```

예:

```text
old u-domain [u0,u1]을
new up-domain [a,b] 기준으로 표현
```

---

# 🎯 한 줄 요약

unit 버전:

```math
f(u)\rightarrow f(a+(b-a)t),\quad t\in[0,1]
```

일반 버전:

```math
f(u)\rightarrow f(\alpha up+\beta),\quad up\in[a,b]
```

즉 둘 다:

```text
polynomial variable substitution matrix
```

이지만,

- unit 버전은 local `[0,1]`
- 일반 버전은 arbitrary domain

을 대상으로 한다.


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
