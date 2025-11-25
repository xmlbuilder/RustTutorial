# 📘 Mesh Quality Metrics Documentation
## 1. Area (면적)
- 정의: 요소의 실제 기하학적 면적
- 계산식:
- 삼각형:
- 사각형: 두 삼각형으로 분할 후 합산
- 의미: 요소 크기 확인, 음수/0이면 퇴화(degenerate) 요소.

## 2. Aspect Ratio (종횡비)
- 정의: 요소의 가장 긴 변 대비 면적 비율
- 계산식:
- 삼각형:

$$
AR=\frac{0.433\cdot (L_{max})^2}{A}
$$

- 사각형: 대각선 중점 연결 길이 비율

$$
AR=\frac{L_{max\\_ diag}}{L_{min\\_ diag}}
$$

- 의미: 값이 클수록 요소가 길쭉하거나 왜곡됨.

## 3. Warp (비틀림)
- 정의: 사각형 요소의 두 대각선 삼각형 법선 벡터 간 각도
- 계산식:

$$
Warp=\max (\angle (n_{013},n_{023}),\angle (n_{123},n_{023}))
$$


- 의미: 평면 사각형은 0°, 비틀림이 심할수록 값 증가.

## 4. Skew (기울기)
- 정의: 사각형 요소 대각선 교차 각도
- 계산식:
- 대각선 벡터 v_1,v_2의 각도
- 90°가 이상적, 0° 또는 180°에 가까우면 퇴화
- 의미: 직각성이 깨질수록 값이 낮아짐.

## 5. Stretch (신장율)
- 정의: 최소 변 길이 대비 최대 대각선 길이 비율
- 계산식:

$$
Stretch=\sqrt{\frac{\sqrt{2}\cdot L_{min\\_ side}}{L_{max\\_ diag}}}
$$

- 의미: 값이 1에 가까울수록 이상적, 0에 가까우면 요소가 찌그러짐.

## 6. Jacobian Quality
- 정의: 요소의 Jacobian 행렬을 이용한 품질 평가
- 계산식:
- 샘플 지점에서 Jacobian 행렬 J 계산
- det(J) 최소/최대 비율:

$$
Q_{det}=\frac{\min (\det J)}{\max (\det J)}
$$

- Condition number:

$$
Q_{cond}=\| J\| \cdot \| J^{-1}\| 
$$

- 의미:
- $Q_{det}\approx 1.0$ → 요소가 잘 정의됨
- $Q_{cond}$ 값이 클수록 왜곡 심함 (정사각형 Quad4는 Frobenius norm 기준 cond=2.0)

## 7. Vector Angle (벡터 각도)
- 정의: 두 벡터 간의 각도
- 계산식:

$$
\theta =\arccos \left( \frac{\vec {a}\cdot \vec {b}}{|a||b|}\right) 
$$

- 의미: Warp, Skew 계산에 활용.
- 직교 → 90°
- 같은 방향 → 0°
- 반대 방향 → 180°

## 📊 종합 활용
- MeshQuality 구조체에 area, aspect_ratio, warp, skew, stretch, jacobian_quality를 포함
- evaluate_quality(face_index) → 각 face별 품질 지표 계산
- evaluate_all_quality() → 전체 mesh 품질 리스트 반환
- 테스트 코드로 정삼각형/정사각형 등 이상적 요소에서 기대값 검증

## ✅ 결론
- Area: 요소 크기
- Aspect Ratio: 길쭉함/왜곡
- Warp: 평면성
- Skew: 직각성
- Stretch: 균일성
- Jacobian Quality: 수치적 안정성
- Vector Angle: 기하학적 관계



## 📐 Jacobian 행렬 계산 함수
```rust
/// 2x2 행렬의 determinant
#[inline]
fn det2(j: [[f64; 2]; 2]) -> f64 {
    j[0][0] * j[1][1] - j[0][1] * j[1][0]
}
```
```rust
/// 2x2 행렬의 Frobenius norm
#[inline]
fn norm2(j: [[f64; 2]; 2]) -> f64 {
    (j[0][0] * j[0][0] + j[0][1] * j[0][1] +
     j[1][0] * j[1][0] + j[1][1] * j[1][1]).sqrt()
}
```
```rust
/// 2x2 행렬의 역행렬
#[inline]
fn inv2(j: [[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let d = det2(j);
    if d.abs() < 1e-12 {
        return None;
    }
    Some([
        [ j[1][1] / d, -j[0][1] / d],
        [-j[1][0] / d,  j[0][0] / d],
    ])
}
```
```rust
/// Jacobian 품질 지표: det(J), condition number
pub struct JacobianQuality {
    pub det: f64,
    pub cond: f64,
}
```
## 🧩 Quad4 Jacobian 품질
```rust
pub fn jacobian_quality_quad4(x: [[f64; 2]; 4]) -> JacobianQuality {
    let g = 1.0_f64 / 3.0_f64.sqrt();
    let sample = [-g, g];
    let mut min_det = f64::INFINITY;
    let mut max_det = f64::NEG_INFINITY;
    let mut worst_cond = 0.0;

    for &ksi in &sample {
        for &eta in &sample {
            let d_n = quad4_shape_gradients(ksi, eta);
            let mut j = [[0.0; 2]; 2];
            for i in 0..4 {
                j[0][0] += x[i][0] * d_n[i][0];
                j[0][1] += x[i][0] * d_n[i][1];
                j[1][0] += x[i][1] * d_n[i][0];
                j[1][1] += x[i][1] * d_n[i][1];
            }
            let detj = det2(j);
            if detj <= 0.0 {
                return JacobianQuality { det: 0.0, cond: f64::INFINITY };
            }
            min_det = min_det.min(detj);
            max_det = max_det.max(detj);

            // condition number = ||J|| * ||J^-1||
            if let Some(invj) = inv2(j) {
                let cond = norm2(j) * norm2(invj);
                worst_cond = worst_cond.max(cond);
            }
        }
    }

    JacobianQuality {
        det: min_det / max_det, // det 비율
        cond: worst_cond,
    }
}
```


## 🧩 Tri3 Jacobian 품질
```rust
pub fn jacobian_quality_tri3(x: [[f64; 2]; 3]) -> JacobianQuality {
    let d_n = tri3_shape_gradients();
    let mut j = [[0.0; 2]; 2];
    for i in 0..3 {
        j[0][0] += x[i][0] * d_n[i][0];
        j[0][1] += x[i][0] * d_n[i][1];
        j[1][0] += x[i][1] * d_n[i][0];
        j[1][1] += x[i][1] * d_n[i][1];
    }
    let detj = det2(j);
    if detj <= 0.0 {
        return JacobianQuality { det: 0.0, cond: f64::INFINITY };
    }
    // 선형 삼각형은 detJ가 상수 → det 품질은 항상 1.0
    let cond = if let Some(invj) = inv2(j) {
        norm2(j) * norm2(invj)
    } else {
        f64::INFINITY
    };
    JacobianQuality { det: 1.0, cond }
}
```


## ✅ 사용 예시
```rust
let quad = [
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
];
```
```rust
let q = jacobian_quality_quad4(quad);
println!("Quad4 det quality = {:.3}, cond = {:.3}", q.det, q.cond);

let tri = [
    [0.0, 0.0],
    [1.0, 0.0],
    [0.0, 1.0],
];
```
```rust
let t = jacobian_quality_tri3(tri);
println!("Tri3 det quality = {:.3}, cond = {:.3}", t.det, t.cond);
```

- cond 값이 2.0으로 나오는 이유는, 지금 계산한 condition number가 단위 정사각형의 Jacobian에서 정확히 2가 되기 때문입니다.

## 📐 왜 cond=2.0인가?
- Quad4 요소의 Jacobian은 **기준 좌표계(ksi, eta)** 에서 shape function gradient를 곱해 얻습니다.
- 단위 정사각형 (0,0)-(1,0)-(1,1)-(0,1) 을 넣으면, 샘플 지점에서 Jacobian은 대략 다음과 같이 나옵니다:

$$
J=\left[ \begin{matrix}0.5&0.0\\ ; \quad 0.0&0.5\end{matrix}\right]
$$

- 이 행렬의 norm은 $||J||=\sqrt{0.5^2+0.5^2}=\sqrt{0.5}\approx 0.707$.
- 역행렬은

$$
J^{-1}=\left[ \begin{matrix}2.0&0.0\\ ; \quad 0.0&2.0\end{matrix}\right]
$$

- 역행렬의 norm은 $||J^{-1}||=\sqrt{2^2+2^2}=\sqrt{8}\approx 2.828$.
- 따라서 condition number = $||J||\cdot ||J^{-1}||\approx 0.707\times 2.828=2.0$.
- 즉, 정사각형 Quad4 요소의 Jacobian은 스케일링 때문에 cond=2.0이 정상적인 값입니다.


## 참고 이미지
![Jacobian Quad](/image/jacobian_quad.png)

## 🔍 참고
- 보통 선형 사각형 요소의 Jacobian은 **등방성(isotropic)** 일 때 cond=2.0 이 나옵니다.
- cond=1.0은 완전히 직교/정규화된 경우(예: 단위 행렬)에서만 나옵니다.
- FEM에서는 cond 값이 커질수록 요소가 왜곡되었다는 뜻이고, cond=2.0은 **정상적인 정사각형** 상태입니다.

## ✅ 결론
- cond=2.0은 버그가 아니라 올바른 결과입니다.
- 만약 cond=1.0을 기대했다면, norm 정의를 바꿔야 합니다.
- 현재는 Frobenius norm을 쓰고 있어서 2.0이 나오고,
- Spectral norm(최대 고유값)으로 바꾸면 cond=1.0이 나옵니다.

---

# Quality Check

Rust의 Mesh 구조에 맞춰서 Mesh Quality Check 기능으로 통합.  
각 face에 대해 품질 지표를 계산하는 방식으로 구현합니다.

## 🧩 품질 지표 목록

| 지표 이름       | 정의 / 의미                                      | 좋은 값 범위 (권장)       |
|-----------------|--------------------------------------------------|---------------------------|
| Area (면적)     | 요소의 실제 기하학적 면적                        | > 0 (양수, 퇴화 방지)     |
| Aspect Ratio    | 가장 긴 변 대비 면적 비율 (삼각형/사각형)         | ~1.0 (정삼각형/정사각형)  |
| Warp (비틀림)   | 사각형 대각선 삼각형 법선 벡터 간 각도            | 0° (평면), 작을수록 좋음  |
| Skew (기울기)   | 사각형 대각선 교차 각도                          | ~90° (직각), 60–120° 허용 |
| Stretch (신장율)| 최소 변 길이 대비 최대 대각선 길이 비율           | ~1.0 (균일), 0에 가까우면 불량 |
| Jacobian Quality| det(J) 최소/최대 비율, cond(J) 수치적 안정성      | det≈1.0, cond 낮을수록 좋음 |


## 🛠️ 품질 체크 함수 추가: MeshQuality
```rust
#[derive(Debug, Clone)]
pub struct MeshQuality {
    pub area: f64,
    pub aspect_ratio: f64,
    pub warp: f64,
    pub skew: f64,
    pub stretch: f64,
}
```


## 🧠 계산 함수: Mesh::evaluate_quality(face_index: usize)
```rust
impl Mesh {
    pub fn evaluate_quality(&self, face_index: usize) -> Option<MeshQuality> {
        if face_index >= self.faces.len() {
            return None;
        }

        let f = self.faces[face_index];
        let v0 = self.vertices[f[0] as usize];
        let v1 = self.vertices[f[1] as usize];
        let v2 = self.vertices[f[2] as usize];
        let v3 = self.vertices[f[3] as usize];

        let is_tri = f[2] == f[3];

        // Area
        let area = if is_tri {
            let e1 = (v1 - v0).to_vector();
            let e2 = (v2 - v0).to_vector();
            e1.cross(&e2).length()
        } else {
            let e1 = (v1 - v0).to_vector();
            let e2 = (v2 - v0).to_vector();
            let e3 = (v3 - v2).to_vector();
            let e4 = (v0 - v2).to_vector();
            let a1 = e1.cross(&e2).length();
            let a2 = e3.cross(&e4).length();
            a1 + a2
        };
```
```rust
        // Aspect Ratio
        let aspect_ratio = if is_tri {
            let a = (v1 - v0).length();
            let b = (v2 - v1).length();
            let c = (v0 - v2).length();
            let s = (a + b + c) / 2.0;
            let tri_area = (s * (s - a) * (s - b) * (s - c)).sqrt();
            let max_side = a.max(b).max(c);
            if tri_area > 0.0 {
                0.4330127018922193 * max_side * max_side / tri_area
            } else {
                f64::MAX
            }
        } else {
            let mut mid = [Vector3D::zero(); 4];
            for i in 0..4 {
                mid[i] = (self.vertices[f[i] as usize].to_vector()
                    + self.vertices[f[(i + 1) % 4] as usize].to_vector())
                    * 0.5;
            }
            let a = (mid[0] - mid[2]).length();
            let b = (mid[1] - mid[3]).length();
            let (max_diag, min_diag) = if a > b { (a, b) } else { (b, a) };
            if min_diag > 0.0 {
                max_diag / min_diag
            } else {
                f64::MAX
            }
        };
```
```rust
        // Warp
        let warp = if is_tri {
            0.0
        } else {
            let mut normals = [Vector3D::zero(); 4];
            for i in 0..4 {
                let a = self.vertices[f[i] as usize];
                let b = self.vertices[f[(i + 1) % 4] as usize];
                let c = self.vertices[f[(i + 2) % 4] as usize];
                let n = (b - a).to_vector().cross(&(c - a).to_vector());
                if n.length() == 0.0 {
                    return Some(MeshQuality {
                        area,
                        aspect_ratio,
                        warp: 180.0,
                        skew: 180.0,
                        stretch: 0.0,
                    });
                }
                normals[i] = n.unitize();
            }
            let ang1 = normals[0].angle(&normals[2]);
            let ang2 = normals[1].angle(&normals[3]);
            ang1.max(ang2)
        };
```
```rust
        // Skew
        let skew = if is_tri {
            0.0
        } else {
            let mut mid = [Vector3D::zero(); 4];
            for i in 0..4 {
                mid[i] = (self.vertices[f[i] as usize].to_vector()
                    + self.vertices[f[(i + 1) % 4] as usize].to_vector())
                    * 0.5;
            }
            let v1 = mid[0] - mid[2];
            let v2 = mid[1] - mid[3];
            if v1.length() == 0.0 || v2.length() == 0.0 {
                180.0
            } else {
                let a = v1.unitize().angle(&v2.unitize());
                if a > 90.0 {
                    180.0 - a
                } else {
                    a
                }
            }
        };
```
```rust
        // Stretch
        let stretch = if is_tri {
            let a = (v1 - v0).length();
            let b = (v2 - v1).length();
            let c = (v0 - v2).length();
            let min_side = a.min(b).min(c);
            let max_diag = (v0 - v1).length().max((v1 - v2).length()).max((v2 - v0).length());
            if max_diag > 0.0 {
                ((1.41421356237 * min_side) / max_diag).sqrt()
            } else {
                0.0
            }
        } else {
            let mut side_len = [0.0; 4];
            let mut min_side = f64::MAX;
            for i in 0..4 {
                side_len[i] = (self.vertices[f[i] as usize] - self.vertices[f[(i + 1) % 4] as usize])
                    .length();
                min_side = min_side.min(side_len[i]);
            }
            let diag1 = (v0 - v2).length();
            let diag2 = (v1 - v3).length();
            let max_diag = diag1.max(diag2);
            if max_diag > 0.0 {
                ((1.41421356237 * min_side) / max_diag).sqrt()
            } else {
                0.0
            }
        };

        Some(MeshQuality {
            area,
            aspect_ratio,
            warp,
            skew,
            stretch,
        })
    }
}
```

## 📊 전체 품질 체크 함수
```rust
impl Mesh {
    pub fn evaluate_all_quality(&self) -> Vec<MeshQuality> {
        let mut out = Vec::with_capacity(self.faces.len());
        for i in 0..self.faces.len() {
            if let Some(q) = self.evaluate_quality(i) {
                out.push(q);
            }
        }
        out
    }
}
```


## ✅ 사용 예시
```rust
let mesh = ...; // Mesh 객체
let qualities = mesh.evaluate_all_quality();

for (i, q) in qualities.iter().enumerate() {
    println!("Face {}: Area = {:.3}, Aspect = {:.2}, Warp = {:.1}, Skew = {:.1}, Stretch = {:.2}",
        i, q.area, q.aspect_ratio, q.warp, q.skew, q.stretch);
}
```


---


