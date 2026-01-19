## 📄 NURBS 곡선/곡면의 도메인 재설정 (Mathematical Documentation)
### 1. NURBS 곡선 정의
- `차수` $p$, `제어점` $\{ P_i\} _{i=0}^n$ , `Knot Vector` $\{ u_i\} _{i=0}^m (m=n+p+1)$ 가 주어졌을 때,
- NURBS 곡선은

$$
C(u)=\frac{\sum _{i=0}^nN_{i,p}(u)\, w_iP_i}{\sum _{i=0}^nN_{i,p}(u)\, w_i},\quad u\in [u_p,u_{n+1}]
$$

- 여기서:
  - $N_{i,p}(u)$: B-spline basis function of degree p
  - $w_i$: 제어점 가중치
  - 유효 도메인: $[u_p,u_{n+1}]$

### 2. 도메인 재설정의 목적
- 원래 곡선은 $[u_p,u_{n+1}]$ 구간에서 정의됩니다.  
- 이를 새로운 구간 $[t_0,t_1]$ 으로 바꾸고 싶을 때, 형상은 그대로 유지하면서 파라미터만 선형 변환합니다.

### 3. Knot Vector 선형 변환
- 모든 Knot $u_i$ 를 다음과 같이 변환합니다:  
- 즉, Knot Vector 전체를 새로운 구간으로 선형 매핑합니다.

### 4. 형상 불변성 증명 아이디어
- Basis Function $N_{i,p}(u)$ 는 Knot Vector의 상대적 위치에 의해 정의됩니다.
- Knot Vector 전체를 선형 변환하면, Basis Function의 형상은 동일하고 단지 정의역만 바뀝니다.
- 따라서 곡선 C(u)의 기하학적 형상은 변하지 않고, 파라미터 구간만 $[t_0,t_1]$ 으로 바뀝니다.

- 즉,

$$
C(u)\equiv C'(u'),\quad u'\in [t_0,t_1]
$$


![Knot Reprameter](/image/knot_reparameter.png)


### 5. NURBS 곡면 확장
- NURBS 곡면은 두 방향(U,V)의 Knot Vector를 가집니다:  

$$
S(u,v)=\frac{\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)\, M_{j,q}(v)\, w_{ij}P_{ij}}{\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)\, M_{j,q}(v)\, w_{ij}}
$$

- U 방향 도메인: $[u_p,u_{n+1}]$
- V 방향 도메인: $[v_q,v_{m+1}]$
- 각 방향의 Knot Vector를 동일한 방식으로 선형 변환하면,  
  곡면 형상은 그대로 유지되고 도메인만 $[t_0^u,t_1^u]$, $[t_0^v,t_1^v]$ 로 바뀝니다.

## 6. 결론
- 곡선/곡면의 형상은 바뀌지 않는다.
- 바뀌는 것은 파라미터 도메인뿐이다.
- 이는 Piegl & Tiller의 **set_domain** 정의와 동일하다.

---

## 소스 코드
### NurbsCurve
```rust
pub fn domain(&self) -> Interval {
    self.domain
}
```
```rust
fn set_domain(&mut self, t0: Real, t1: Real) -> bool {
    let kv_len = self.kv.knots.len();
    let degree = self.degree as usize;

    if kv_len == 0 || self.degree < 1 || kv_len <= degree || t0 >= t1 {
        return false;
    }

    let k0 = self.kv.knots[degree];
    let k1 = self.kv.knots[kv_len - degree - 1]; // Piegl 기준 내부 도메인

    if (k0 - t0).abs() < 1e-12 && (k1 - t1).abs() < 1e-12 {
        return true; // 이미 도메인이 맞음
    }

    if k0 >= k1 {
        return false;
    }

    let d = (t1 - t0) / (k1 - k0);
    let km = 0.5 * (k0 + k1);

    for knot in &mut self.kv.knots {
        if *knot <= km {
            *knot = (*knot - k0) * d + t0;
        } else {
            *knot = (*knot - k1) * d + t1;
        }
    }

    true
}
```

### NurbsSurface
```rust
impl NurbsSurface {
    /// U 또는 V 방향 도메인 설정
    /// dir = 0 → U, dir = 1 → V
    pub fn set_domain(&mut self, dir: usize, t0: Real, t1: Real) -> bool {
        let (degree, kv) = if dir == 0 {
            (self.pu as usize, &mut self.ku.knots)
        } else {
            (self.pv as usize, &mut self.kv.knots)
        };

        let kv_len = kv.len();
        if kv_len == 0 || degree < 1 || kv_len <= degree || t0 >= t1 {
            return false;
        }

        let k0 = kv[degree];
        let k1 = kv[kv_len - degree - 1]; // Piegl 기준 내부 도메인

        if (k0 - t0).abs() < 1e-12 && (k1 - t1).abs() < 1e-12 {
            return true; // 이미 도메인이 맞음
        }

        if k0 >= k1 {
            return false;
        }

        let d = (t1 - t0) / (k1 - k0);
        let km = 0.5 * (k0 + k1);

        for knot in kv.iter_mut() {
            if *knot <= km {
                *knot = (*knot - k0) * d + t0;
            } else {
                *knot = (*knot - k1) * d + t1;
            }
        }

        true
    }
```
```rust
    /// 현재 도메인 반환
    pub fn domain(&self, dir: usize) -> Option<(Real, Real)> {
        if dir == 0 {
            let degree = self.pu as usize;
            let kv_len = self.ku.knots.len();
            if kv_len > degree {
                Some((self.ku.knots[degree], self.ku.knots[kv_len - degree - 1]))
            } else { None }
        } else {
            let degree = self.pv as usize;
            let kv_len = self.kv.knots.len();
            if kv_len > degree {
                Some((self.kv.knots[degree], self.kv.knots[kv_len - degree - 1]))
            } else { None }
        }
    }
}
```
---

