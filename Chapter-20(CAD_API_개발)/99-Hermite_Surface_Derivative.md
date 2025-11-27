# Hermite Surface Derivative

Hermite basis 함수와 그 도함수들을 이용해 n차 도함수를 계산.
이 함수는 (u,v)에서 원하는 차수 d까지의 모든 편도함수를 2차원 벡터 배열로 반환합니다.

## 🛠 구현 예시
```rust
impl Surface for HermiteSurface {
    fn domain_u(&self) -> Interval {
        Interval { t0: 0.0, t1: 1.0 }
    }
```
```rust
    fn domain_v(&self) -> Interval {
        Interval { t0: 0.0, t1: 1.0 }
    }
```
```rust
    fn eval_point(&self, u: Real, v: Real) -> Point3D {
        self.point_at_param(u, v).unwrap_or(Point3D::zero())
    }
```
```rust
    fn eval_ders_padded(&self, u: Real, v: Real, d: usize) -> Vec<Vec<Vector3D>> {
        if !self.is_valid() {
            return vec![vec![Vector3D::zero(); d + 1]; d + 1];
        }
```
```rust
        // 1) 현재 (u,v)가 속한 셀과 로컬 파라미터(s,t), 구간폭(hu,hv)
        let (iu, s, hu) = Self::locate_param(&self.u_parameters, u);
        let (jv, t, hv) = Self::locate_param(&self.v_parameters, v);
```
```rust
        // 2) Hermite basis와 도함수들 계산
        fn hermite_basis_and_ders(s: f64) -> [[f64; 4]; 4] {
            // [차수][basis index]
            let s2 = s * s;
            let s3 = s2 * s;
            let mut out = [[0.0; 4]; 4];
            // 0차
            out[0] = [
                2.0 * s3 - 3.0 * s2 + 1.0,
                s3 - 2.0 * s2 + s,
                -2.0 * s3 + 3.0 * s2,
                s3 - s2,
            ];
            // 1차
            out[1] = [
                6.0 * s2 - 6.0 * s,
                3.0 * s2 - 4.0 * s + 1.0,
                -6.0 * s2 + 6.0 * s,
                3.0 * s2 - 2.0 * s,
            ];
            // 2차
            out[2] = [
                12.0 * s - 6.0,
                6.0 * s - 4.0,
                -12.0 * s + 6.0,
                6.0 * s - 2.0,
            ];
            // 3차
            out[3] = [12.0, 6.0, -12.0, 6.0];
            out
        }
```
```rust
        let hu_basis = hermite_basis_and_ders(s);
        let hv_basis = hermite_basis_and_ders(t);
```
```rust
        // 3) 코너 데이터 꺼내기
        let p00 = self.grid_points[iu][jv];
        let p10 = self.grid_points[iu + 1][jv];
        let p01 = self.grid_points[iu][jv + 1];
        let p11 = self.grid_points[iu + 1][jv + 1];

        let pu00 = self.u_tangents[iu][jv];
        let pu10 = self.u_tangents[iu + 1][jv];
        let pu01 = self.u_tangents[iu][jv + 1];
        let pu11 = self.u_tangents[iu + 1][jv + 1];

        let pv00 = self.v_tangents[iu][jv];
        let pv10 = self.v_tangents[iu + 1][jv];
        let pv01 = self.v_tangents[iu][jv + 1];
        let pv11 = self.v_tangents[iu + 1][jv + 1];

        let tw00 = self.twists[iu][jv];
        let tw10 = self.twists[iu + 1][jv];
        let tw01 = self.twists[iu][jv + 1];
        let tw11 = self.twists[iu + 1][jv + 1];
```
```rust
        // 4) 결과 배열 초기화
        let mut ders = vec![vec![Vector3D::zero(); d + 1]; d + 1];
```
```rust
        // 5) 모든 (p,q) 도함수 계산
        for p in 0..=d {
            for q in 0..=(d - p) {
                let mut sum = Vector3D::zero();

                // corner (i,j)
                sum += p00.to_vec() * (hu_basis[p][0] * hv_basis[q][0]);
                sum += pu00 * (hu_basis[p][1] * hv_basis[q][0] * hu);
                sum += pv00 * (hu_basis[p][0] * hv_basis[q][1] * hv);
                sum += tw00 * (hu_basis[p][1] * hv_basis[q][1] * hu * hv);

                // corner (i+1,j)
                sum += p10.to_vec() * (hu_basis[p][2] * hv_basis[q][0]);
                sum += pu10 * (hu_basis[p][3] * hv_basis[q][0] * hu);
                sum += pv10 * (hu_basis[p][2] * hv_basis[q][1] * hv);
                sum += tw10 * (hu_basis[p][3] * hv_basis[q][1] * hu * hv);

                // corner (i,j+1)
                sum += p01.to_vec() * (hu_basis[p][0] * hv_basis[q][2]);
                sum += pu01 * (hu_basis[p][1] * hv_basis[q][2] * hu);
                sum += pv01 * (hu_basis[p][0] * hv_basis[q][3] * hv);
                sum += tw01 * (hu_basis[p][1] * hv_basis[q][3] * hu * hv);

                // corner (i+1,j+1)
                sum += p11.to_vec() * (hu_basis[p][2] * hv_basis[q][2]);
                sum += pu11 * (hu_basis[p][3] * hv_basis[q][2] * hu);
                sum += pv11 * (hu_basis[p][2] * hv_basis[q][3] * hv);
                sum += tw11 * (hu_basis[p][3] * hv_basis[q][3] * hu * hv);

                ders[p][q] = sum;
            }
        }

        ders
    }
}
```

## ✅ 요약
- `hermite_basis_and_ders` 함수로 Hermite basis의 0~3차 도함수를 계산.
- eval_ders_padded에서 (p,q) 조합으로 n차 도함수를 모두 구함.
- 각 항은 corner point, u tangent, v tangent, twist와 Hermite basis 도함수의 곱으로 계산.
- 스케일링(hu, hv, hu*hv)을 반드시 곱해줘야 올바른 값이 나옵니다.

- 👉 이렇게 하면 eval_ders_padded(u,v,d)가 (u,v)에서 원하는 차수까지의 모든 도함수를 반환하게 됩니다.

---

# 벡터 성분

eval_ders_padded 함수는 (u,v)에서 원하는 차수까지의 **도함수 벡터(Vector3D)** 를 계산해 반환합니다.  
여기서 결과로 나오는 Vector3D 성분은 각각 x, y, z 방향의 편도함수 값을 의미합니다.  

## 📘 결과 구조
```rust
eval_ders_padded(u,v,d) → Vec<Vec<Vector3D>>
```
- ders[p][q] : 

$$
\frac{\partial ^{p+q}S}{\partial u^p\partial v^q}(u,v)
$$

- 각 Vector3D는 (x, y, z) 성분을 가짐

🔎 예시 출력 (d=2일 때)
만약 d=2라면 결과는 다음과 같이 구성됩니다:
| Index      | Meaning                          | Components (Vector3D) |
|------------|----------------------------------|-----------------------|
| ders[0][0] | S(u,v)                           | (x, y, z)             |
| ders[1][0] | ∂S/∂u                            | (dx/du, dy/du, dz/du) |
| ders[0][1] | ∂S/∂v                            | (dx/dv, dy/dv, dz/dv) |
| ders[2][0] | ∂²S/∂u²                          | (d²x/du², d²y/du², d²z/du²) |
| ders[1][1] | ∂²S/∂u∂v                         | (d²x/(du dv), d²y/(du dv), d²z/(du dv)) |
| ders[0][2] | ∂²S/∂v²                          | (d²x/dv², d²y/dv², d²z/dv²) |


## 🛠 Rust에서 성분 표시하기

원하는 성분을 직접 출력하려면 예를 들어:

```rust
let ders = surface.eval_ders_padded(u, v, 2);

// 위치 벡터
println!("Position: ({}, {}, {})",
    ders[0][0].x, ders[0][0].y, ders[0][0].z);

// u 방향 1차 도함수
println!("d/du: ({}, {}, {})",
    ders[1][0].x, ders[1][0].y, ders[1][0].z);

// v 방향 1차 도함수
println!("d/dv: ({}, {}, {})",
    ders[0][1].x, ders[0][1].y, ders[0][1].z);

// 혼합 도함수
println!("d²/(du dv): ({}, {}, {})",
    ders[1][1].x, ders[1][1].y, ders[1][1].z);
```


## ✅ 요약
- 결과 벡터는 항상 (x, y, z) 성분을 포함.
- ders[p][q]는 해당 차수의 편도함수 벡터.
- 성분을 출력하려면 .x, .y, .z 필드를 사용.



