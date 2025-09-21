# Math Utilities — Functions and Formulas

## 1. is_continuous
곡선/곡면의 접합점에서 연속성(continuity)을 판정하는 함수.

- **C0**: 위치만 일치

$$
  P_a = P_b
$$

- **C1**: 위치 + 1차 미분 벡터(접선)도 동일

$$
  P_a = P_b, \quad D1_a = D1_b
$$

- **G1**: 위치 + 접선 방향만 동일(크기는 다를 수 있음)

$$
  \cos \theta = \frac{D1_a \cdot D1_b}{\lVert D1_a\rVert\lVert D1_b\rVert} \approx 1
$$

- **G2**: 위치, 접선 방향, 곡률 벡터 방향까지 동일

### 소스
``` rust
pub fn is_continuous(
    desired: Continuity,
    pa: Point3D, d1a: Vector3D, d2a: Vector3D,
    pb: Point3D, d1b: Vector3D, d2b: Vector3D,
    point_tol: f64, d1_tol: f64, d2_tol: f64,
    cos_angle_tol: f64, curvature_tol: f64,
) -> bool {
    use Continuity::*;
    match desired {
        Unknown => true,

        C0 | C0Locus => {
            (pb - pa).length() <= point_tol
        }

        C1 | C1Locus => {
            (pb - pa).length() <= point_tol &&
                (d1a - d1b).length() <= d1_tol
        }

        G1 | G1Locus => {
            let ta = ev_tangent(d1a, d2a).unwrap_or(Vector3D::zero());
            let tb = ev_tangent(d1b, d2b).unwrap_or(Vector3D::zero());
            (pb - pa).length() <= point_tol &&
                ta.dot(&tb) >= cos_angle_tol
        }

        C2 | C2Locus | CInfinity => {
            (pb - pa).length() <= point_tol &&
                (d1a - d1b).length() <= d1_tol &&
                (d2a - d2b).length() <= d2_tol
        }

        G2 | G2Locus | GSmooth => {
            let (ta, ka) = ev_curvature(d1a, d2a);
            let (tb, kb) = ev_curvature(d1b, d2b);
            let ta = ta.unwrap_or(Vector3D::zero());
            let tb = tb.unwrap_or(Vector3D::zero());
            if (pb - pa).length() > point_tol { return false; }
            if ta.dot(&tb) < cos_angle_tol { return false; }
            match desired {
                GSmooth => is_gsmooth_curvature_continuous(ka, kb, cos_angle_tol, curvature_tol),
                _       => is_g2_curvature_continuous(ka, kb, cos_angle_tol, curvature_tol),
            }
        }
    }
}
```

### 사용법
```rust
#[test]
    fn continuity_c0_c1_g1_g2() {
        // 동일한 점, 같은 접선/곡률
        let pa = Point3D::new(0.0, 0.0, 0.0);
        let pb = Point3D::new(0.0, 0.0, 0.0);
        let d1a = Vector3D::new(1.0, 0.0, 0.0);
        let d1b = Vector3D::new(1.0, 0.0, 0.0);
        let d2a = Vector3D::new(0.0, 0.0, 0.0);
        let d2b = Vector3D::new(0.0, 0.0, 0.0);

        let ok_c0 = is_continuous(
            Continuity::C0, pa, d1a, d2a, pb, d1b, d2b,
            1e-12, 1e-12, 1e-12, 0.999, 1e-12
        );
        assert!(ok_c0);

        let ok_c1 = is_continuous(
            Continuity::C1, pa, d1a, d2a, pb, d1b, d2b,
            1e-12, 1e-12, 1e-12, 0.999, 1e-12
        );
        assert!(ok_c1);

        let ok_g1 = is_continuous(
            Continuity::G1, pa, d1a, d2a, pb, d1b, d2b,
            1e-12, 1e-12, 1e-12, 0.999, 1e-12
        );
        assert!(ok_g1);

        let ok_g2 = is_continuous(
            Continuity::G2, pa, d1a, d2a, pb, d1b, d2b,
            1e-12, 1e-12, 1e-12, 0.999, 1e-12
        );
        assert!(ok_g2);
    }
```

---

## 2. search_monotone_array
단조증가 배열에서 값이 들어갈 위치를 찾는 이진 탐색 함수.

조건:

$$
  a[i] \leq t < a[i+1]
$$

경계값 처리:
- `t < a[0]` → `-1`
- `t >= a[n-1]` → `n-1` 또는 `n`

### 소스
```rust
pub fn search_monotone_array(array: &[f64], t: f64) -> i32 {
    if array.is_empty() { return -2; }
    let mut length = array.len() as i32 - 1;

    if t < array[0] { return -1; }
    if t >= array[length as usize] {
        return if t > array[length as usize] { length + 1 } else { length };
    }
    if array.len() >= 2 && t < array[1] { return 0; }
    if array.len() >= 2 && t >= array[(length - 1) as usize] { return length - 1; }

    let mut i0 = 0i32;
    let mut i1 = length;

    while array[i0 as usize] == array[(i0 + 1) as usize] { i0 += 1; }
    while array[i1 as usize] == array[(i1 - 1) as usize] { i1 -= 1; }

    while i0 + 1 < i1 {
        let i = (i0 + i1) >> 1;
        if t < array[i as usize] {
            i1 = i;
            while array[i1 as usize] == array[(i1 - 1) as usize] { i1 -= 1; }
        } else {
            i0 = i;
            while array[i0 as usize] == array[(i0 + 1) as usize] { i0 += 1; }
        }
    }
    i0
}
```

### 사용법
```rust
#[test]
    fn search_monotone_basic_and_edges() {
        let a = [1.0, 2.0, 2.0, 5.0, 9.0];
        // t < a[0]
        assert_eq!(search_monotone_array(&a, 0.5), -1);
        // t == last
        assert_eq!(search_monotone_array(&a, 9.0), (a.len() as i32) - 1);
        // t > last
        assert_eq!(search_monotone_array(&a, 9.1), (a.len() as i32));
        // inside duplicates: t=2.0 -> returns an i s.t. a[i] <= t < a[i+1]
        let idx = search_monotone_array(&a, 2.0);
        assert!(idx == 1 || idx == 2); // 둘 중 하나면 OK
        // typical
        assert_eq!(search_monotone_array(&a, 4.0), 2);
    }
```
---

## 3. binomial_coefficient(i,j)
이항계수:

$$
  \binom{i+j}{i} = \frac{(i+j)!}{i!\,j!}
$$

예:  
`binomial_coefficient(2,3) = C(5,2) = 10`

### 소스
```rust
pub fn binomial_coefficient(i: i32, j: i32) -> f64 {
    if i < 0 || j < 0 { return 0.0; }
    if i == 0 || j == 0 { return 1.0; }
    let n = (i + j) as i64;
    if i == 1 || j == 1 { return n as f64; }

    let k = i.min(j) as i64;
    let mut num = 1.0_f64;
    let mut den = 1.0_f64;
    for t in 1..=k {
        num *= (n - k + t) as f64;
        den *= t as f64;
        // 간단한 정규화(언더/오버플로 방지)
        let g = num.abs().max(1.0);
        if g > 1e100 { num /= 1e50; den /= 1e50; }
    }
    num / den
}
```

### 사용법
```rust
#[test]
    fn binomial_trinomial_values() {
        // ON_BinomialCoefficient(i,j) == C(i+j, i)
        assert_eq!(binomial_coefficient(2, 3), 10.0); // C(5,2)=10
        assert_eq!(binomial_coefficient(1, 4), 5.0);  // C(5,1)=5
        assert_eq!(binomial_coefficient(0, 7), 1.0);  // C(7,0)=1

        // Trinomial: (i+j+k)!/(i!j!k!)
        // i=j=k=1 => 6
        assert_eq!(trinomial_coefficient(1,1,1), 6.0);
        // 2,1,0 => C(3,2)*C(1,0)=3*1=3
        assert_eq!(trinomial_coefficient(2,1,0), 3.0);
    }

```

---

## 4. trinomial_coefficient(i,j,k)
삼항계수:
$$
  \frac{(i+j+k)!}{i!\,j!\,k!}
$$

예:  
`trinomial_coefficient(1,1,1) = 6`

### 소스
```rust
pub fn trinomial_coefficient(i: i32, j: i32, k: i32) -> f64 {
    binomial_coefficient(i, j + k) * binomial_coefficient(j, k)
}
```
### 사용법
```rust

```
    #[test]
    fn binomial_trinomial_values() {
        // ON_BinomialCoefficient(i,j) == C(i+j, i)
        assert_eq!(binomial_coefficient(2, 3), 10.0); // C(5,2)=10
        assert_eq!(binomial_coefficient(1, 4), 5.0);  // C(5,1)=5
        assert_eq!(binomial_coefficient(0, 7), 1.0);  // C(7,0)=1

        // Trinomial: (i+j+k)!/(i!j!k!)
        // i=j=k=1 => 6
        assert_eq!(trinomial_coefficient(1,1,1), 6.0);
        // 2,1,0 => C(3,2)*C(1,0)=3*1=3
        assert_eq!(trinomial_coefficient(2,1,0), 3.0);
    }
---

## 5. reverse_point_list_f64
포인트 리스트를 **역순(reverse)** 으로 뒤집음.

예:  
입력: `[(0,0,0), (1,2,3), (4,5,6)]`  
출력: `[(4,5,6), (1,2,3), (0,0,0)]`

### 소스
``` rust
pub fn reverse_point_list_f64(dim: i32, is_rat: bool, count: i32, stride: i32, p: &mut [f64]) -> bool {
    if !is_valid_point_list_f64(dim, is_rat, count, stride, p) { return false; }
    if count <= 1 { return true; }
    let logical_dim = if is_rat { dim + 1 } else { dim } as usize;
    let stride = stride as usize;
    let mut i = 0usize;
    let mut j = (count as usize - 1)*stride;
    while i < j {
        for k in 0..logical_dim {
            p.swap(i + k, j + k);
        }
        i += stride;
        j = j.saturating_sub(stride);
    }
    true
}
```

### 사용법
```rust
#[test]
    fn reverse_point_list_3d() {
        // [ (0,0,0), (1,2,3), (4,5,6) ]
        let mut buf = vec![
            0.0,0.0,0.0,
            1.0,2.0,3.0,
            4.0,5.0,6.0
        ];
        assert!(reverse_point_list_f64(3, false, 3, 3, &mut buf));
        // 기대: [ (4,5,6),(1,2,3),(0,0,0) ]
        assert!(approx_p(
            Point3D::new(buf[0],buf[1],buf[2]),
            Point3D::new(4.0,5.0,6.0), 1e-12
        ));
        assert!(approx_p(
            Point3D::new(buf[3],buf[4],buf[5]),
            Point3D::new(1.0,2.0,3.0), 1e-12
        ));
        assert!(approx_p(
            Point3D::new(buf[6],buf[7],buf[8]),
            Point3D::new(0.0,0.0,0.0), 1e-12
        ));
    }
```

---

## 6. reverse_point_grid_f64
2차원 격자(grid) 포인트들을 행/열 단위로 뒤집음.  
보통 **패치(surface patch)** 파라미터화 시 `u`나 `v` 방향을 뒤집을 때 사용.

### 소스
```rust
pub fn reverse_point_grid_f64(
    dim: i32, is_rat: bool,
    count0: i32, count1: i32,
    stride0: i32, stride1: i32,
    p: &mut [f64], dir: i32
) -> bool {
    if dir == 0 {
        // 0방향은 축 치환해서 동일 함수 호출
        return reverse_point_grid_f64(dim, is_rat, count1, count0, stride1, stride0, p, 1);
    }
    let mut ok = true;
    for row in 0..count0 {
        let start = (row as usize) * (stride0 as usize);
        let slice = &mut p[start..];
        if !reverse_point_list_f64(dim, is_rat, count1, stride1, slice) {
            ok = false;
            break;
        }
    }
    ok
}
```

### 사용법
```rust
    #[test]
    fn reverse_point_grid_2x3() {
        // 2행 x 3열, stride0=3*3=9, stride1=3
        // row0: (0,0,0),(1,0,0),(2,0,0)
        // row1: (0,1,0),(1,1,0),(2,1,0)
        let mut buf = vec![
            0.0,0.0,0.0,  1.0,0.0,0.0,  2.0,0.0,0.0,
            0.0,1.0,0.0,  1.0,1.0,0.0,  2.0,1.0,0.0
        ];
        assert!(reverse_point_grid_f64(3, false, 2, 3, 9, 3, &mut buf, 1));
        // 각 행이 좌우로 뒤집힘: row0 -> (2,0,0),(1,0,0),(0,0,0)
        assert!(approx_p(Point3D::new(buf[0],buf[1],buf[2]), Point3D::new(2.0,0.0,0.0), 1e-12));
        assert!(approx_p(Point3D::new(buf[3],buf[4],buf[5]), Point3D::new(1.0,0.0,0.0), 1e-12));
        assert!(approx_p(Point3D::new(buf[6],buf[7],buf[8]), Point3D::new(0.0,0.0,0.0), 1e-12));
    }
```

---

## 7. swap_point_list_coords_f64
포인트 좌표의 특정 축을 서로 교환.

예:  
```
입력: `[(1,2), (-3,4)]`, swap(x,y) →  
출력: `[(2,1), (4,-3)]`
```

### 소스
```rust
pub fn swap_point_list_coords_f64(count: i32, stride: i32, p: &mut [f64], i: i32, j: i32) -> bool {
    if !is_valid_point_list_f64(stride, false, count, stride, p) { return false; }
    if i < 0 || j < 0 || i >= stride || j >= stride { return false; }
    if i == j || count == 0 { return true; }
    let stride = stride as usize;
    let (i, j) = (i as usize, j as usize);
    for idx in 0..(count as usize) {
        let base = idx*stride;
        p.swap(base + i, base + j);
    }
    true
}
```
---

### 사용법
```rust
    fn swap_point_list_coords() {
        // 2D points, swap x<->y
        let mut buf = vec![
            1.0, 2.0,
            -3.0, 4.0,
            0.5, -1.5
        ];
        assert!(swap_point_list_coords_f64(3, 2, &mut buf, 0, 1));
        assert!(approx(buf[0], 2.0, 1e-12) && approx(buf[1], 1.0, 1e-12));
        assert!(approx(buf[2], 4.0, 1e-12) && approx(buf[3], -3.0, 1e-12));
        assert!(approx(buf[4], -1.5, 1e-12) && approx(buf[5], 0.5, 1e-12));
    }
```

## 8. transform_point_list_f64
변환행렬(`Transform`)을 이용해 포인트 리스트를 변환.

- **비가중 (Affine)**:

$$
  p' = T \cdot p
$$

- **가중 (Rational, NURBS용)**:

$$
  [x,y,z,w]' = T \cdot [x,y,z,1]^T,\quad 결과는 다시 w로 나눔
$$

### 소스
```rust
pub fn transform_point_list_f64(
    dim: i32, is_rat: bool, count: i32, stride: i32,
    point: &mut [f64], xform: &Transform
) -> bool {
    if !is_valid_point_list_f64(dim, is_rat, count, stride, point) { return false; }
    if count == 0 { return true; }
    let m = &xform.m;
    let stride = stride as usize;

    let mut idx = 0usize;
    for _ in 0..count {
        if is_rat {
            match dim {
                1 => {
                    let x = m[0][0]*point[idx] + m[0][3]*point[idx+1];
                    let w = m[3][0]*point[idx] + m[3][3]*point[idx+1];
                    point[idx]   = x; point[idx+1] = w;
                }
                2 => {
                    let x = m[0][0]*point[idx] + m[0][1]*point[idx+1] + m[0][3]*point[idx+2];
                    let y = m[1][0]*point[idx] + m[1][1]*point[idx+1] + m[1][3]*point[idx+2];
                    let w = m[3][0]*point[idx] + m[3][1]*point[idx+1] + m[3][3]*point[idx+2];
                    point[idx] = x; point[idx+1] = y; point[idx+2] = w;
                }
                _ => {
                    let d = dim as usize;
                    let x = m[0][0]*point[idx] + m[0][1]*point[idx+1] + m[0][2]*point[idx+2] + m[0][3]*point[idx+d];
                    let y = m[1][0]*point[idx] + m[1][1]*point[idx+1] + m[1][2]*point[idx+2] + m[1][3]*point[idx+d];
                    let z = m[2][0]*point[idx] + m[2][1]*point[idx+1] + m[2][2]*point[idx+2] + m[2][3]*point[idx+d];
                    let w = m[3][0]*point[idx] + m[3][1]*point[idx+1] + m[3][2]*point[idx+2] + m[3][3]*point[idx+d];
                    point[idx] = x; point[idx+1] = y; point[idx+2] = z; point[idx+d] = w;
                }
            }
        } else {
            match dim {
                1 => {
                    let mut w = m[3][0]*point[idx] + m[3][3];
                    if w == 0.0 { w = 1.0; }
                    let w_inv = 1.0 / w;
                    let x = m[0][0]*point[idx] + m[0][3];
                    point[idx] = w_inv * x;
                }
                2 => {
                    let mut w = m[3][0]*point[idx] + m[3][1]*point[idx+1] + m[3][3];
                    if w == 0.0 { w = 1.0; }
                    let w_inv = 1.0 / w;
                    let x = m[0][0]*point[idx] + m[0][1]*point[idx+1] + m[0][3];
                    let y = m[1][0]*point[idx] + m[1][1]*point[idx+1] + m[1][3];
                    point[idx] = w_inv * x; point[idx+1] = w_inv * y;
                }
                _ => {
                    let mut w = m[3][0]*point[idx] + m[3][1]*point[idx+1] + m[3][2]*point[idx+2] + m[3][3];
                    if w == 0.0 { w = 1.0; }
                    let w_inv = 1.0 / w;
                    let x = m[0][0]*point[idx] + m[0][1]*point[idx+1] + m[0][2]*point[idx+2] + m[0][3];
                    let y = m[1][0]*point[idx] + m[1][1]*point[idx+1] + m[1][2]*point[idx+2] + m[1][3];
                    let z = m[2][0]*point[idx] + m[2][1]*point[idx+1] + m[2][2]*point[idx+2] + m[2][3];
                    point[idx] = w_inv * x; point[idx+1] = w_inv * y; point[idx+2] = w_inv * z;
                }
            }
        }
        idx += stride;
    }
    true
}
```

### 사용법
```rust
    #[test]
    fn transform_point_list_affine_3d() {
        // 비가중 3D, translation(1,2,3)
        let t = Transform::translation(1.0, 2.0, 3.0);
        let mut buf = vec![
            0.0,0.0,0.0,
            1.0,2.0,3.0
        ];
        assert!(transform_point_list_f64(3, false, 2, 3, &mut buf, &t));
        assert!(approx_p(Point3D::new(buf[0],buf[1],buf[2]), Point3D::new(1.0,2.0,3.0), 1e-12));
        assert!(approx_p(Point3D::new(buf[3],buf[4],buf[5]), Point3D::new(2.0,4.0,6.0), 1e-12));
    }

```
---

## 9. transform_vector_list_f64
벡터 리스트 변환.  
⚠️ Translation은 무시됨 (벡터는 방향만 의미).

$$
  v' = R \cdot v
$$

### 소스
```rust
pub fn transform_vector_list_f64(dim: i32, count: i32, stride: i32, v: &mut [f64], x: &Transform) -> bool {
    if !is_valid_point_list_f64(dim, false, count, stride, v) { return false; }
    if count == 0 { return true; }
    let m = &x.m;
    let stride = stride as usize;
    let mut idx = 0usize;
    for _ in 0..count {
        match dim {
            1 => {
                let nx = m[0][0]*v[idx];
                v[idx] = nx;
            }
            2 => {
                let nx = m[0][0]*v[idx] + m[0][1]*v[idx+1];
                let ny = m[1][0]*v[idx] + m[1][1]*v[idx+1];
                v[idx] = nx; v[idx+1] = ny;
            }
            _ => {
                let nx = m[0][0]*v[idx] + m[0][1]*v[idx+1] + m[0][2]*v[idx+2];
                let ny = m[1][0]*v[idx] + m[1][1]*v[idx+1] + m[1][2]*v[idx+2];
                let nz = m[2][0]*v[idx] + m[2][1]*v[idx+1] + m[2][2]*v[idx+2];
                v[idx] = nx; v[idx+1] = ny; v[idx+2] = nz;
            }
        }
        idx += stride;
    }
    true
}
```

### 사용법
```rust
    #[test]
    fn transform_vector_list_translation_invariant() {
        // 벡터는 translation의 영향을 받지 않음
        let t = Transform::translation(10.0, -5.0, 2.5);
        let mut v = vec![
            1.0, 0.0, 0.0,
            -2.0, 3.0, 0.5
        ];
        assert!(transform_vector_list_f64(3, 2, 3, &mut v, &t));
        assert!(approx_v(
            Vector3D::new(v[0],v[1],v[2]), Vector3D::new(1.0,0.0,0.0), 1e-12
        ));
        assert!(approx_v(
            Vector3D::new(v[3],v[4],v[5]), Vector3D::new(-2.0,3.0,0.5), 1e-12
        ));
    }
```

---

## 10. transform_point_grid_f64
2차원 격자(grid) 포인트 전체에 변환행렬 적용.  
Surface Patch, Mesh 노드 등 전체 그리드를 한 번에 변환할 때 유용.

### 소스
```rust
pub fn transform_point_grid_f64(
    dim: i32, is_rat: bool,
    count0: i32, count1: i32,
    stride0: i32, stride1: i32,
    point: &mut [f64],
    xform: &Transform
) -> bool {
    let mut ok = false;
    for r in 0..count0 {
        let base = (r as usize)*(stride0 as usize);
        let slice = &mut point[base..];
        if !transform_point_list_f64(dim, is_rat, count1, stride1, slice, xform) {
            ok = false;
            break;
        } else if r == 0 { ok = true; }
    }
    ok
}

```

### 사용법
```rust
    #[test]
    fn transform_point_grid_wrapped() {
        // 2x2 grid, 3D points
        let t = Transform::translation(1.0, 0.0, 0.0);
        let mut buf = vec![
            0.0,0.0,0.0,   1.0,0.0,0.0,
            0.0,1.0,0.0,   1.0,1.0,0.0
        ];
        // count0=2, count1=2, stride0=6, stride1=3
        assert!(transform_point_grid_f64(3, false, 2, 2, 6, 3, &mut buf, &t));
        assert!(approx_p(Point3D::new(buf[0],buf[1],buf[2]), Point3D::new(1.0,0.0,0.0), 1e-12));
        assert!(approx_p(Point3D::new(buf[3],buf[4],buf[5]), Point3D::new(2.0,0.0,0.0), 1e-12));
        assert!(approx_p(Point3D::new(buf[6],buf[7],buf[8]), Point3D::new(1.0,1.0,0.0), 1e-12));
        assert!(approx_p(Point3D::new(buf[9],buf[10],buf[11]), Point3D::new(2.0,1.0,0.0), 1e-12));
    }
```
---
