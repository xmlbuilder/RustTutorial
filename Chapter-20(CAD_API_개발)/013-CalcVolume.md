# Calc Volume

## ✅ 사용된 수식 요약 및 검증
### 1. 🔺 삼각형 면적 벡터
```
tri_area_vec(v0, v1, v2) = (v1 - v0) × (v2 - v0)
```

- 수학적 의미:  

```math 
\vec {A}=\vec {v}_1-\vec {v}_0
```

```math
\vec {B}=\vec {v}_2-\vec {v}_0
```

```math
\mathrm{면적벡터}=\vec {A}\times \vec {B}
```

- 크기:  

```math
|\vec {A}\times \vec {B}|=2\cdot \mathrm{Area}
```

- ✅ 정확한 수식입니다.

### 2. 📦 삼각형 기반 체적 (원점 기준)
```
tri_signed_volume_from_origin(a, b, c) = a · (b × c)
```

- 수학적 의미:  

```math
V=\frac{1}{6}\cdot \vec {a}\cdot (\vec {b}\times \vec {c})
```

- 코드에서는 누적 후 $s / 6.0$ 으로 나눔
- ✅ 정확한 수식입니다.

### 3. 🧭 방향 정렬 기준
```
s = n_ref · ((v1 - v0) × (v2 - v0))
```

- 면의 법선 벡터와 기준 법선의 내적
- s<0 이면 CW → flip 필요
- ✅ 정확한 방향 판별 수식입니다.

### 4. 🧮 테트라 체적 (CalcVolume)
```
vol = v1x*v2y*v3z + v1y*v2z*v3x + v2x*v3y*v1z
    - v3x*v2y*v1z - v2x*v1y*v3z - v3y*v2z*v1x
```

- 수학적으로:  

```math
V=\frac{1}{6}\cdot \det \left[ \begin{matrix}v_1\\ v_2\\ v_3\end{matrix}\right] \quad \mathrm{(원점\  기준\  테트라)}
```

-✅ 정확한 체적 수식입니다.

### 5. 🎯 중심점 (Centroid)
```
centroid = (cx, cy, cz) / (4 * m)
```

- 테트라 중심:

```math
\vec {C}=\frac{1}{4}(v_0+v_1+v_2+v_3)
```

- 누적 중심:
```math
\sum V_i\cdot (v_1+v_2+v_3)
```
- ✅ 정확한 중심점 수식입니다.

### 6. 📈 1차 모멘트
```rust
world_x = x / 24, world_y = y / 24, world_z = z / 24
```

- 수학적으로:  

```math
\int _Vx\, dV\approx \sum V_i\cdot \bar {x}_i
```

- 평균화 계수 $\frac{1}{24}$ 는 테트라 중심에 대한 근사치
- ✅ 실무에서 널리 쓰이는 근사 수식입니다.

### 7. 📊 2차 모멘트
```rust
world_xx = xx / 120, world_yy = yy / 120, world_zz = zz / 120
```

- 수학적으로:  

```math
I_{xx}=\int _Vx^2 dV
```

- 테트라 기준 근사 계수 $\frac{1}{120}$

- ✅ 정확한 근사 수식입니다.

### 8. 🔁 제품 모멘트
```rust
world_xy = yx / 120, world_yz = zy / 120, world_zx = zx / 120
```
```math
I_{xy}=\int _Vxy dV
```
- ✅ 정확한 근사 수식입니다.

### 9. 🧮 평행축 정리
```
ccs_xx = world_xx - mass * cx²
```

```math
I_{ccs}=I_{world}-m\cdot d^2
```

- ✅ 정확한 수식입니다.

## 🧠 최종 평가

| 수식 항목             | 수학적 정확성 ✅ | 실무 적합성 ⚙️ | 설명 요약                                               |
|----------------------|------------------|------------------|----------------------------------------------------------|
| 삼각형 면적 벡터     | ✅ 정확           | ✅ 매우 적합      | 외적 기반으로 2×면적 계산, 방향성 포함                   |
| 삼각형 기반 체적     | ✅ 정확           | ✅ 매우 적합      | 원점 기준 테트라 체적 계산: a · (b × c) / 6              |
| 방향 정렬 기준       | ✅ 정확           | ✅ 매우 적합      | 기준 법선과 면적벡터 내적 → CW/CCW 판별                 |
| 테트라 체적 공식     | ✅ 정확           | ✅ 매우 적합      | 행렬식 기반 체적 계산, 누적 후 /6                        |
| 중심점 (Centroid)    | ✅ 정확           | ✅ 매우 적합      | 테트라 중심 평균: (v₁+v₂+v₃)/4, 누적 후 /4m              |
| 1차 모멘트           | ✅ 근사 정확       | ✅ 실무 적합      | 위치 평균 누적 후 /24 → 질량 중심 계산에 사용           |
| 2차 모멘트           | ✅ 근사 정확       | ✅ 실무 적합      | 제곱 거리 누적 후 /120 → 관성 모멘트 계산에 사용         |
| 제품 모멘트          | ✅ 근사 정확       | ✅ 실무 적합      | xy, yz, zx 누적 후 /120 → 회전축 간 상호작용 표현        |
| 평행축 정리          | ✅ 정확           | ✅ 매우 적합      | 중심 기준 관성 모멘트 변환: I = I₀ - m·d²                |

## 📘 사용된 함수 요약표

| 함수 이름                        | 주요 역할 설명                                 | 정의 위치         | 테스트 여부 | 비고                         |
|----------------------------------|--------------------------------------------------|--------------------|--------------|------------------------------|
| on_tri_area_vec                     | 삼각형 면적 벡터 계산 (2×면적 × 법선)           | calc_volume.rs     | ✅ 간접 테스트됨 | 방향 정렬 및 법선 계산에 사용 |
| on_tri_signed_volume_from_origin    | 삼각형 기반 부호 있는 체적 계산                | calc_volume.rs     | ✅ 직접 테스트됨 | total_signed_volume 내부 사용 |
| on_total_signed_volume              | 전체 메시 체적 계산 (원점 기준 테트라 누적)     | calc_volume.rs     | ❌ 미테스트     | 별도 테스트 필요              |
| on_compute_ref_normal               | 메시 참조 법선 계산                             | calc_volume.rs     | ✅ 간접 테스트됨 | ensure_ccw_auto 내부 사용     |
| on_ensure_ccw_auto                  | 면 방향을 CCW로 정렬                            | calc_volume.rs     | ✅ 직접 테스트됨 | 방향 정렬 테스트 포함         |
| CalcVolume::add_triangle         | 삼각형 하나를 체적 누적에 추가                  | CalcVolume         | ✅ 테스트됨     | 다양한 삼각형 입력에 사용     |
| CalcVolume::add_triangles        | 메시 전체 삼각형을 체적 누적에 추가             | CalcVolume         | ✅ 테스트됨     | 사각형 분할 포함              |
| CalcVolume::volume               | 누적된 체적 반환 (6으로 나눈 값)                | CalcVolume         | ✅ 테스트됨     | 정방향/역방향 체적 검증 포함  |
| CalcVolume::centroid             | 체적 기반 중심점 반환                           | CalcVolume         | ❌ 미테스트     | 중심 좌표 검증 필요           |
| CalcVolume::write_result         | MassProperties에 결과 기록                      | CalcVolume         | ❌ 미테스트     | 모든 필드 값 검증 필요        |


## 🧪 테스트 함수 요약표

| 테스트 함수 이름                   | 검증 대상 함수/기능                  | 입력 유형             | 기대 결과 요약                            | 상태 |
|-----------------------------------|--------------------------------------|------------------------|-------------------------------------------|--------|
| tetra_volume_positive_with_ccw    | CalcVolume::add_triangles, volume    | 정방향 테트라          | 체적 = 1/6                                | ✅ 완료 |
| quad_two_tris_ccw_auto            | on_ensure_ccw_auto, on_compute_ref_normal  | CW/CCW 혼합 삼각형     | 모든 면이 CCW 방향으로 정렬됨             | ✅ 완료 |
| degenerate_triangle_zero_volume   | add_triangles, volume                | 퇴화 삼각형            | 체적 = 0                                  | ✅ 완료 |
| tetra_volume_negative_with_cw     | add_triangles, volume                | 역방향 테트라          | 체적 < 0                                  | ✅ 완료 |
| flat_quad_volume_zero             | add_triangles, volume                | 평면 사각형            | 체적 = 0                                  | ✅ 완료 |

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::calc_volume::{on_compute_ref_normal, on_ensure_ccw_auto, on_tri_area_vec, CalcVolume};
    use nurbslib::core::mass_properties::MassProperties;
    use nurbslib::core::maths::on_are_equal;
    use nurbslib::core::mesh::MeshFace;
    use nurbslib::core::prelude::Point;
    use nurbslib::core::types::ON_TOL9;
```
```rust
    #[test]
    fn tetra_volume_positive_with_ccw() {
        // 간단한 테트라 (정방향)
        let v = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        ];
        let tris = vec![
            MeshFace::new_tri(0, 1, 2),
            MeshFace::new_tri(0, 3, 1),
            MeshFace::new_tri(0, 2, 3),
            MeshFace::new_tri(1, 2, 3),
        ];
        let mut cv = CalcVolume::default();
        cv.add_triangles(&v, &tris);
        println!("{:?}", cv.volume());
        assert!(on_are_equal(cv.volume(), 1.0 / 6.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn quad_two_tris_ccw_auto() {
        let v = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        // 하나는 CCW, 하나는 CW 로 섞음
        let mut faces = vec![
            MeshFace::new_tri(0, 1, 2), // +Z
            MeshFace::new_tri(0, 3, 2), // -Z (뒤집어야)
        ];
        on_ensure_ccw_auto(&v, &mut faces, 1e-14);

        // 자동 기준 n_ref 에 대해 모두 s>0 이어야 함
        let n_ref = on_compute_ref_normal(&v, &faces, 1e-14).unwrap();
        for f in &faces {
            let (i0, i1, i2) = (f.vi[0] as usize, f.vi[1] as usize, f.vi[2] as usize);
            let s = n_ref.dot(&on_tri_area_vec(v[i0], v[i1], v[i2]));
            assert!(s > 0.0);
        }
    }
```
```rust
    #[test]
    fn degenerate_triangle_zero_volume() {
        let v = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 0.0), // 동일 선상
        ];
        let tris = vec![MeshFace::new_tri(0, 1, 2)];
        let mut cv = CalcVolume::default();
        cv.add_triangles(&v, &tris);
        assert!(on_are_equal(cv.volume(), 0.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn tetra_volume_negative_with_cw() {
        let v = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        ];
        let tris = vec![
            MeshFace::new_tri(0, 2, 1), // CW
            MeshFace::new_tri(0, 3, 2),
            MeshFace::new_tri(0, 1, 3),
            MeshFace::new_tri(1, 2, 3),
        ];
        let mut cv = CalcVolume::default();
        cv.add_triangles(&v, &tris);
        assert!(cv.volume() < 0.0); // 방향 뒤집힘으로 음수 체적
    }
```
```rust
    #[test]
    fn flat_quad_volume_zero() {
        let v = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let tris = vec![
            MeshFace::new_tri(0, 1, 2),
            MeshFace::new_tri(0, 2, 3),
        ];
        let mut cv = CalcVolume::default();
        cv.add_triangles(&v, &tris);
        assert!(on_are_equal(cv.volume(), 0.0, ON_TOL9)); // 평면이므로 체적 없음
    }
```
```rust
    #[test]
    fn volume_tetrahedron() {
        // Tetrahedron with vertices (0,0,0), (1,0,0), (0,1,0), (0,0,1)
        let v = vec![
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }, //0
            Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }, //1
            Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }, //2
            Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }, //3
        ];
        // 4 faces, oriented outward (any consistent orientation works for this accumulator)
        let tris = vec![
            MeshFace::new_tri(0, 1, 2),
            MeshFace::new_tri(0, 3, 1),
            MeshFace::new_tri(0, 2, 3),
            MeshFace::new_tri(1, 2, 3),
        ];
        let mut cv = CalcVolume::default();
        cv.add_triangles(&v, &tris);

        // Volume of this tetrahedron is 1/6
        assert!(on_are_equal(cv.volume(), 1.0 / 6.0, ON_TOL9));
        let c = cv.centroid(); // centroid should be at (1/4,1/4,1/4)
        assert!(on_are_equal(c.x, 0.25, ON_TOL9) && on_are_equal(c.y, 0.25, ON_TOL9) && on_are_equal(c.z, 0.25, ON_TOL9));

        let mut mp = MassProperties::default();
        assert!(cv.write_result(&mut mp));
        assert!(on_are_equal(mp.mass, 1.0 / 6.0, ON_TOL9));
    }
}
```
---




    
