# 왜 테스트에서 약 0.900316 값이 나오는가?

```rust
/// Approximative nodal curvatures from tangents (parametric, dT/du).
///
/// pos1D       : 1D parameter at each node (length = N)
/// connect_e   : 2-node edges (CM2_EDGE2 only), used here mainly for validation
/// local_bases : T(u) at each node, size = N, each [Tx, Ty, Tz]
/// curvatures  : output H[i] = || dT/du || at node i
///
/// return:
///   >= 0 : success (0)
///   <  0 : error
pub fn parametric_curvatures(
    pos1D: &[f64],
    connect_e: &[[usize; 2]],
    local_bases: &[[f64; 3]],
    curvatures: &mut Vec<f64>,
) -> i32 {
    let n = pos1D.len();

    if local_bases.len() != n {
        return -1; // rows == 3 은 구조로 보장, cols == pos1D.size()
    }

    // connectivity 범위 체크
    for &e in connect_e {
        let i = e[0];
        let j = e[1];
        if i >= n || j >= n {
            return -2;
        }
    }

    curvatures.clear();
    curvatures.resize(n, 0.0);

    if n == 0 {
        return 0;
    }
    if n == 1 {
        curvatures[0] = 0.0;
        return 0;
    }
    if n == 2 {
        // 두 점인 경우, 간단히 0 또는 한 값만 줄 수도 있지만
        // 여기서는 0 으로 둘게요.
        curvatures[0] = 0.0;
        curvatures[1] = 0.0;
        return 0;
    }

    // (u, index) 를 정렬해서 라인 상의 순서를 얻는다.
    let mut order: Vec<(f64, usize)> = pos1D
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, u)| (u, idx))
        .collect();

    order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // 정렬된 u, T 를 별도 배열로
    let mut u_sorted = Vec::with_capacity(n);
    let mut t_sorted = Vec::with_capacity(n);
    let mut idx_sorted = Vec::with_capacity(n);

    for (u, idx) in &order {
        u_sorted.push(*u);
        t_sorted.push(local_bases[*idx]);
        idx_sorted.push(*idx);
    }

    // 중앙차분 H_k = || (T_{k+1} - T_{k-1}) / (u_{k+1} - u_{k-1}) ||
    let eps = 1.0e-15;
    let mut H_sorted = vec![0.0_f64; n];

    for k in 1..(n - 1) {
        let u_prev = u_sorted[k - 1];
        let u_next = u_sorted[k + 1];
        let du = u_next - u_prev;

        if du.abs() < eps {
            H_sorted[k] = 0.0;
            continue;
        }

        let t_prev = t_sorted[k - 1];
        let t_next = t_sorted[k + 1];

        let dt = [
            t_next[0] - t_prev[0],
            t_next[1] - t_prev[1],
            t_next[2] - t_prev[2],
        ];
        let dt_norm = (dt[0] * dt[0] + dt[1] * dt[1] + dt[2] * dt[2]).sqrt();
        H_sorted[k] = dt_norm / du.abs();
    }

    // endpoints: neighbor 값으로
    H_sorted[0] = H_sorted[1];
    H_sorted[n - 1] = H_sorted[n - 2];

    // 정렬을 풀어서 원래 node index 순으로 curvatures 채우기
    for (k, node_idx) in idx_sorted.iter().enumerate() {
        curvatures[*node_idx] = H_sorted[k];
    }

    0
}
```
```rust
#[test]
fn test_parametric_curvatures_circle() {
    use std::f64::consts::{FRAC_PI_2, PI};

    let u0 = 0.0;
    let u1 = FRAC_PI_2 * 0.5; // π/4
    let u2 = FRAC_PI_2;       // π/2

    let pos1D = vec![u0, u1, u2];

    // tangent T(u) = (-sin u, cos u, 0)
    fn T(u: f64) -> [f64; 3] {
        [-u.sin(), u.cos(), 0.0]
    }

    let local_bases = vec![T(u0), T(u1), T(u2)];
    let connect_e = vec![[0usize, 1usize], [1usize, 2usize]];

    let mut H = Vec::<f64>::new();
    let err = parametric_curvatures(&pos1D, &connect_e, &local_bases, &mut H);
    assert!(err >= 0, "parametric_curvatures() returned error {}", err);

    println!("[test_parametric_curvatures_circle] H = {:?}", H);
    assert_eq!(H.len(), 3);

    // 이 central difference 셋업의 이론값은 sin(h)/h
    let h = PI / 4.0;
    let k_theoretical = h.sin() / h; // ≈ 0.9003163
    let eps = 1.0e-6;

    for i in 0..3 {
        assert!(
            (H[i] - k_theoretical).abs() < eps,
            "H[{}] = {}, expected ~{}",
            i,
            H[i],
            k_theoretical
        );
    }
}
```
---


본 문서는 Rust 테스트 함수 `test_parametric_curvatures_circle()` 실행 시  
곡률 이론값 1이 아닌 **약 0.900316** 이 출력되는 이유를 수학적으로 정리한 것입니다.

## 1. 테스트에서 사용한 곡선과 접선

테스트는 단위 원(circle)을 사용합니다.

- 곡선  

$$ 
P(u) = (\cos u,\; \sin u) 
$$

- 접선  

$$ 
T(u) = \frac{dP}{du} = (-\sin u,  \cos u) 
$$

단위 원의 실제 곡률은 어디서나 **1** 입니다.

---

## 2. 테스트에서 사용된 u 값

테스트는 아래와 같은 **3개의 점만** 사용합니다.

- $u_0 = 0$
- $u_1 = \frac{\pi}{4}$
- $u_2 = \frac{\pi}{2}$

즉,

```
0 -------- π/4 -------- π/2
```

이렇게 **두 구간으로만** 원을 표현하고 차분을 수행하게 됩니다.  
→ 이 때문에 중앙 차분의 근사 오차가 커집니다.

---

## 3. 중앙 차분으로 dT/du 계산

코드에서 근사한 곡률은 다음과 같습니다.

$$
H \approx \left\| \frac{T(u_2) - T(u_0)}{u_2 - u_0} \right\|
$$

이를 직접 계산해보면,

### 3.1 접선 계산

- $T(0) = (0,\; 1,\; 0)$  
- $T(\frac{\pi}{2}) = (-1, 0,  0)$

차이 벡터:

$$
T(u_2) - T(u_0) = (-1, -1, 0)
$$

### 3.2 노름

$$
\|T(u_2)-T(u_0)\| = \sqrt{(-1)^2 + (-1)^2} = \sqrt{2}
$$

### 3.3 분모

$$
u_2 - u_0 = \frac{\pi}{2}
$$

---

## 4. 최종 근사값

$$
H = \frac{\sqrt{2}}{\pi/2}
  = \frac{2\sqrt{2}}{\pi}
  \approx 0.900316316
$$

따라서 테스트 출력:

```
H = [0.9003163, 0.9003163, 0.9003163]
```

은 수학적으로 완전히 정상입니다.

---

## 5. 왜 1보다 작은가?

### ✔ 이유: **차분 간격이 너무 크기 때문**

중앙 차분이 정확하려면 **간격 h가 매우 작아야** 합니다.  
그러나 테스트에서는:

- $h = \frac{\pi/4}{}$ (아주 큼)
- 노드가 3개뿐임
- 곡률 근사는 2차 미분이기 때문에 특히 민감함

---

## 6. 정확한 이론 근사식: sin(h)/h

단위 원에서 중앙 차분으로 $dT/du$ 를 근사하면:

$$
H(h) = \frac{\sin h}{h}
$$

여기서 

$$ 
h = \frac{\pi}{4} 
$$

이므로:

$$
H = \frac{\sin(\pi/4)}{\pi/4}
  = \frac{\frac{\sqrt{2}}{2}}{\pi/4}
  = \frac{2\sqrt{2}}{\pi}
  = 0.9003163
$$

즉, **지금 코드의 결과는 이론식과 완벽하게 일치** 합니다.

---

## 7. 결론

| 항목 | 값 |
|------|----------------------|
| 실제 곡률 | 1 |
| 근사 곡률 | 약 0.900316 |
| 발생 원인 | 차분 간격이 너무 큼 (노드 3개) |
| 해결 | 더 촘촘한 u-grid 사용 |

---

## 8. 추가 결론

> ❗ 즉, 문제는 **코드가 아니라 테스트 구성이 너무 조악** 해서 발생한 자연스러운 근사 오차임.

더 많은 노드를 사용하면 H가 1에 점점 가까워집니다.

---

