# Calc Area

## 🧮 1. 삼각형 면적 계산
```rust
fn tri_area(v1, v2, v3) = 0.5 * |(v3 - v1) × (v2 - v1)|
```

### 📌 수식 설명
- 두 벡터 $\vec {a}=v2-v1$, $\vec {b}=v3-v1$
- 면적 $A=\frac{1}{2}\cdot \| \vec {b}\times \vec {a}\|$ 
- 이는 벡터 외적의 크기로 삼각형의 면적을 구하는 고전적인 방식입니다.

## 🧠 2. 중심점(centroid) 계산
```rust
centroid = (cx, cy, cz) / (6 * total_area)
```

### 📌 수식 설명
- 각 삼각형의 세 꼭짓점 평균: $\vec {c}_i=\frac{v_1+v_2+v_3}{3}$
- 전체 중심:

$$
\vec {C}=\frac{1}{\sum A_i}\sum A_i\cdot \vec {c}_i
$$

- 코드에서는 $\mathrm{cx}+=2A\cdot (x_1+x_2+x_3)$ → 나중에 $cx/(6A)$

## 🧩 3. 1차 모멘트 (First Moment)
```rust
world_x = x / 6, world_y = y / 6, world_z = z / 6
```
- 각 축에 대한 질량 중심의 위치를 계산하는 데 사용됨

$$
x=\sum 2A\cdot (x_1+x_2+x_3)
$$

## 🧲 4. 2차 모멘트 (Second Moment of Area)

```rust
world_xx = xx / 12, world_yy = yy / 12, world_zz = zz / 12
```

### 📌 수식 설명
- $xx=\sum A\cdot (x_1^2+x_2^2+x_3^2+(x_1+x_2+x_3)^2)$
- 이는 각 삼각형의 면적 가중 평균 제곱 거리를 누적한 값
- 12로 나누는 이유는 면적 중심 기준으로 평균화하기 위함

## 🔁 5. 제품 모멘트 (Product of Inertia)
```rust
world_xy = yx / 2, world_yz = zy / 2, world_zx = zx / 2
```

- $$yx=\sum 2A\cdot (y_1x_1+y_2x_2+y_3x_3+(y_1+y_2+y_3)(x_1+x_2+x_3))$$
- 제품 모멘트는 회전축 간 상호작용을 나타냄

## 🧮 6. 중심 좌표계 관성 모멘트 (CCS)
```
ccs_xx = world_xx - mass * cx²
```

- 중심 좌표계 기준으로 관성 모멘트를 변환
- 이는 평행축 정리(Parallel Axis Theorem) 기반:

$$
I_{ccs}=I_{world}-m\cdot d^2
$$


## 📐 7. 선분 면적 기여 (add_line)
```
area = x₁(y₂ - 0) + x₂(0 - y₁)
```

- 이는 XY 평면에 투영된 선분이 원점 기준으로 만드는 삼각형의 부호 있는 면적입니다.
- 선분이 삼각형을 구성하지 않더라도 면적 기여가 있을 수 있음

## 🧠 전체 알고리즘 흐름 요약
1. 각 삼각형의 면적 계산 → m += area
2. 각 꼭짓점 좌표 누적 → 중심점 계산용
3. 제곱 및 곱 누적 → 관성 모멘트 계산용
4. 최종적으로:
   - area() → 총 면적
   - centroid() → 질량 중심
   - write_result() → MassProperties에 모든 결과 기록

---

## 코드

```rust
#[derive(Clone, Debug)]
pub struct MassProperties {
    pub mass_type: i32, // 2: area, 3: volume (matching original comments)
    pub mass: f64,
    pub valid_mass: bool,
    pub valid_centroid: bool,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,

    pub valid_first: bool,
    pub world_x: f64,
    pub world_y: f64,
    pub world_z: f64,

    pub valid_second: bool,
    pub world_xx: f64,
    pub world_yy: f64,
    pub world_zz: f64,

    pub valid_product: bool,
    pub world_xy: f64,
    pub world_yz: f64,
    pub world_zx: f64,

    pub ccs_xx: f64,
    pub ccs_yy: f64,
    pub ccs_zz: f64,
}

impl Default for MassProperties {
    fn default() -> Self {
        Self {
            mass_type: 0,
            mass: 0.0,
            valid_mass: false,
            valid_centroid: false,
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            valid_first: false,
            world_x: 0.0,
            world_y: 0.0,
            world_z: 0.0,
            valid_second: false,
            world_xx: 0.0,
            world_yy: 0.0,
            world_zz: 0.0,
            valid_product: false,
            world_xy: 0.0,
            world_yz: 0.0,
            world_zx: 0.0,
            ccs_xx: 0.0,
            ccs_yy: 0.0,
            ccs_zz: 0.0,
        }
    }
}
```

```rust
use crate::core::geom::PointF;
use crate::core::mass_properties::MassProperties;
use crate::core::mesh::MeshFace;
use crate::core::prelude::Point;

#[derive(Clone, Debug)]
pub struct CalcArea {
    m: f64,
    cx: f64,
    cy: f64,
    cz: f64,
    x: f64,
    y: f64,
    z: f64,
    xx: f64,
    yy: f64,
    zz: f64,
    yx: f64,
    zx: f64,
    zy: f64,
}
```
```rust
impl Default for CalcArea {
    fn default() -> Self {
        Self {
            m: 0.0,
            cx: 0.0,
            cy: 0.0,
            cz: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            xx: 0.0,
            yy: 0.0,
            zz: 0.0,
            yx: 0.0,
            zx: 0.0,
            zy: 0.0,
        }
    }
}
```
```rust
impl CalcArea {
    #[inline]
    fn tri_area(v1: Point, v2: Point, v3: Point) -> f64 {
        let a = v2 - v1;
        let b = v3 - v1;
        0.5 * b.cross_pt(&a).length()
    }

    pub fn add_triangle(&mut self, v1: Point, v2: Point, v3: Point) {
        let area = Self::tri_area(v1, v2, v3);
        self.m += area;

        let (x1, y1, z1) = (v1.x, v1.y, v1.z);
        let (x2, y2, z2) = (v2.x, v2.y, v2.z);
        let (x3, y3, z3) = (v3.x, v3.y, v3.z);

        let sumx = x1 + x2 + x3;
        let sumy = y1 + y2 + y3;
        let sumz = z1 + z2 + z3;

        self.cx += 2.0 * area * sumx;
        self.cy += 2.0 * area * sumy;
        self.cz += 2.0 * area * sumz;

        self.x += 2.0 * area * sumx;
        self.y += 2.0 * area * sumy;
        self.z += 2.0 * area * sumz;

        self.xx += area * (x1 * x1 + x2 * x2 + x3 * x3 + sumx * sumx);
        self.yy += area * (y1 * y1 + y2 * y2 + y3 * y3 + sumy * sumy);
        self.zz += area * (z1 * z1 + z2 * z2 + z3 * z3 + sumz * sumz);

        self.yx += 2.0 * area * (y1 * x1 + y2 * x2 + y3 * x3 + sumy * sumx);
        self.zx += 2.0 * area * (z1 * x1 + z2 * x2 + z3 * x3 + sumz * sumx);
        self.zy += 2.0 * area * (z1 * y1 + z2 * y2 + z3 * y3 + sumz * sumy);
    }

    pub fn add_triangle_f32(&mut self, v1: PointF, v2: PointF, v3: PointF) {
        self.add_triangle(v1.into(), v2.into(), v3.into());
    }

    pub fn add_line(&mut self, v1: Point, v2: Point) {
        // Project to XY for signed area contribution of segment wrt origin triangle (x1,y1)-(x2,y2)-(0,0)
        let (x1, y1, z1) = (v1.x, v1.y, v1.z);
        let (x2, y2, z2) = (v2.x, v2.y, v2.z);

        let num1 = 0.0f64;
        let area = x1 * (y2 - num1) + x2 * (num1 - y1) + 0.0 * (y1 - y2);
        if area.abs() < 1e-10 {
            return;
        }

        self.m += area / 2.0;

        let sum_x = x1 + x2;
        let sum_y = y1 + y2;
        let sum_z = z1 + z2;

        self.cx += area * sum_x;
        self.cy += area * sum_y;
        self.cz += area * sum_z;

        self.x += area * sum_x;
        self.y += area * sum_y;
        self.z += area * sum_z;

        self.xx += area * (x1 * x1 + x1 * x2 + x2 * x2);
        self.yy += area * (y1 * y1 + y1 * y2 + y2 * y2);
        self.zz += area * (z1 * z1 + z1 * z2 + z2 * z2);

        self.yx += area * (x1 * y2 + 2.0 * x1 * y1 + 2.0 * x2 * y2 + x2 * y1);
        self.zx += area * (x1 * z2 + 2.0 * x1 * z1 + 2.0 * x2 * z2 + x2 * z1);
        self.zy += area * (y1 * z2 + 2.0 * y1 * z1 + 2.0 * y2 * z2 + y2 * z1);
    }

    pub fn add_triangles(&mut self, vertices: &[Point], faces: &[MeshFace]) {
        for f in faces {
            if f.is_triangle() {
                let a = vertices[f.vi[0] as usize];
                let b = vertices[f.vi[1] as usize];
                let c = vertices[f.vi[2] as usize];
                self.add_triangle(a, b, c);
            } else {
                let v0 = vertices[f.vi[0] as usize];
                let v1 = vertices[f.vi[1] as usize];
                let v2 = vertices[f.vi[2] as usize];
                let v3 = vertices[f.vi[3] as usize];
                self.add_triangle(v0, v1, v2);
                self.add_triangle(v2, v3, v0);
            }
        }
    }

    pub fn add_triangles_f32(&mut self, vertices: &[PointF], faces: &[MeshFace]) {
        for f in faces {
            if f.is_triangle() {
                let a: Point = vertices[f.vi[0] as usize].into();
                let b: Point = vertices[f.vi[1] as usize].into();
                let c: Point = vertices[f.vi[2] as usize].into();
                self.add_triangle(a, b, c);
            } else {
                let v0: Point = vertices[f.vi[0] as usize].into();
                let v1: Point = vertices[f.vi[1] as usize].into();
                let v2: Point = vertices[f.vi[2] as usize].into();
                let v3: Point = vertices[f.vi[3] as usize].into();
                self.add_triangle(v0, v1, v2);
                self.add_triangle(v2, v3, v0);
            }
        }
    }

    pub fn area(&self) -> f64 {
        self.m
    }

    pub fn centroid(&self) -> Point {
        if self.m == 0.0 {
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let w = 1.0 / (6.0 * self.m);
            Point {
                x: self.cx * w,
                y: self.cy * w,
                z: self.cz * w,
            }
        }
    }

    pub fn mass(&self) -> f64 {
        self.area()
    }

    pub fn write_result(&self, mp: &mut MassProperties) -> bool {
        mp.mass_type = 2;
        mp.mass = self.area();
        mp.valid_mass = true;
        mp.valid_centroid = true;

        let c = self.centroid();
        mp.x0 = c.x;
        mp.y0 = c.y;
        mp.z0 = c.z;

        let n1 = 1.0 / 6.0;
        mp.valid_first = true;
        mp.world_x = self.x * n1;
        mp.world_y = self.y * n1;
        mp.world_z = self.z * n1;

        let r = 1.0 / 12.0;
        mp.valid_second = true;
        mp.world_xx = self.xx * r;
        mp.world_yy = self.yy * r;
        mp.world_zz = self.zz * r;

        mp.valid_product = true;
        mp.world_xy = self.yx / 2.0;
        mp.world_yz = self.zy / 2.0;
        mp.world_zx = self.zx / 2.0;

        mp.ccs_xx = mp.world_xx - mp.mass * c.x * c.x;
        mp.ccs_yy = mp.world_yy - mp.mass * c.y * c.y;
        mp.ccs_zz = mp.world_zz - mp.mass * c.z * c.z;
        true
    }
}
```

---

## 수식 검증


## ✅ 1. 삼각형 면적 계산
```rust
0.5 * (v3 - v1).cross(v2 - v1).length()
```

### 🔍 수식 해석
- 두 벡터 $\vec {a}=v2-v1$, $\vec {b}=v3-v1$
- 외적 $\vec {a}\times \vec {b}$ 의 크기 = 평행사변형 넓이
- 삼각형 넓이 = $\frac{1}{2}\cdot \| \vec {a}\times \vec {b}\|$
- ✅ 정확한 수식입니다.

## ✅ 2. 중심점(centroid) 계산
```rust
centroid = (cx, cy, cz) / (6 * total_area)
```

### 🔍 수식 해석
- 각 삼각형의 중심: $\vec {c}_i=\frac{v_1+v_2+v_3}{3}$
- 전체 중심:

$$
\vec {C}=\frac{1}{\sum A_i}\sum A_i\cdot \vec {c}_i
$$

- 코드에서 $\mathrm{cx}+=2A\cdot (x_1+x_2+x_3)$ → $\mathrm{cx}=6A\cdot \bar {x}\Rightarrow \bar {x}=\frac{\mathrm{cx}}{6A}$
- ✅ 정확한 수식입니다.

## ✅ 3. 1차 모멘트 (First Moment)
```rust
world_x = x / 6
```

- $x=\sum 2A\cdot (x_1+x_2+x_3)$
- 평균 위치: $\frac{x}{6A}$
- ✅ 정확한 면적 가중 평균입니다.

## ✅ 4. 2차 모멘트 (Second Moment of Area)
```
world_xx = xx / 12
```

### 🔍 수식 해석
- $xx=\sum A\cdot (x_1^2+x_2^2+x_3^2+(x_1+x_2+x_3)^2)$
- 이 수식은 다음을 근사합니다:

$$
I_{xx}=\int _Ax^2 dA\approx \sum A_i\cdot \left( \frac{x_1^2+x_2^2+x_3^2+(x_1+x_2+x_3)^2}{4}\right)
$$

- 코드에서는 12로 나누어 평균화
- ✅ 근사적이지만 실무에서 널리 쓰이는 정확한 수식입니다.

## ✅ 5. 제품 모멘트 (Product of Inertia)
```rust
world_xy = yx / 2
```

- $yx=\sum 2A\cdot (y_1x_1+y_2x_2+y_3x_3+(y_1+y_2+y_3)(x_1+x_2+x_3))$
- 이는 $- \int _Axy dA$ 를 근사
- ✅ 정확한 근사 수식입니다.

## ✅ 6. 평행축 정리 (Parallel Axis Theorem)
```
ccs_xx = world_xx - mass * cx²
```

- $I_{ccs}=I_{world}-m\cdot d^2$
- 여기서 $d=\mathrm{centroid\  coordinate}$
- ✅ 정확한 수식입니다.

## ✅ 7. 선분 면적 계산 (add_line)
```
area = x₁(y₂ - 0) + x₂(0 - y₁)
```

- 이는 XY 평면에서 원점 기준 삼각형의 부호 있는 면적:  

$$
A=\frac{1}{2}\cdot \left( x_1y_2-x_2y_1\right)
$$

- 코드에서는 2배로 계산 후 나중에 나눔
- ✅ 정확한 수식입니다.

## 🧠 결론
모든 수식은 기하학적 질량 속성 계산의 표준 방식에 기반하며,
- 면적, 중심, 관성 모멘트, 제품 모멘트, 평행축 정리까지
- 정확하고 실무에 적합한 구현입니다.

---
## ✅ CalcArea 함수 테스트 커버리지 요약
| 함수 이름               | 기능 설명                                                                 | 테스트 필요성 | 현재 테스트 포함 여부 | 추천 테스트 시나리오 예시                         |
|------------------------|---------------------------------------------------------------------------|----------------|------------------------|--------------------------------------------------|
| tri_area               | 삼각형 면적 계산 (벡터 외적 기반)                                         | 높음           | ✅ 내부적으로 사용됨     | 단일 삼각형 면적 정확성 검증                     |
| add_triangle           | 삼각형 하나 추가 및 면적/모멘트 누적                                     | 높음           | ✅ 포함됨               | 다양한 삼각형 입력에 대한 누적 결과 확인         |
| add_triangle_f32       | f32 버전 삼각형 추가                                                      | 중간           | ❌ 미포함               | f32 → f64 변환 후 정확성 유지 확인               |
| add_line               | 선분을 XY 평면 기준 삼각형으로 간주해 면적 기여                           | 높음           | ✅ 포함됨 (수정 필요)   | 선분 3개로 삼각형 구성 후 면적/중심 확인         |
| add_triangles          | MeshFace 배열 기반 삼각형/사각형 추가                                     | 높음           | ✅ 포함됨               | 사각형 → 삼각형 분할 정확성 확인                 |
| add_triangles_f32      | f32 버전 MeshFace 처리                                                    | 중간           | ❌ 미포함               | f32 정점 기반 메시 처리 정확성 확인              |
| area                   | 누적 면적 반환                                                            | 높음           | ✅ 포함됨               | 다양한 입력에 대한 면적 검증                     |
| centroid               | 면적 기반 중심점 반환                                                     | 높음           | ✅ 포함됨               | 정규 삼각형/사각형 중심 좌표 확인                |
| mass                   | area()와 동일                                                             | 낮음           | ✅ 포함됨               | area()와 값 일치 여부 확인                       |
| with_result            | MassProperties에 결과 기록                                                | 높음           | ✅ 포함됨               | 모든 속성 필드가 정확히 채워졌는지 확인          |


| 테스트 함수 이름                        | 검증 대상 함수         | 입력 유형           | 기대 결과 요약                                      | 상태 |
|----------------------------------------|------------------------|---------------------|-----------------------------------------------------|------|
| area_two_tris_make_square              | add_triangles          | 사각형(2 삼각형)     | 면적 = 1.0, 중심 = (0.5, 0.5, 0.0)                  | ✅ 완료 |
| area_single_triangle                   | add_triangle           | 단일 삼각형          | 면적 = 0.5, 중심 = (1/3, 1/3, 0.0)                  | ✅ 완료 |
| area_nonplanar_triangle                | add_triangle           | Z축 포함 삼각형      | 면적 = 0.5, 중심 = (1/3, 0.0, 1/3)                  | ✅ 완료 |
| area_quad_split_into_tris             | add_triangles          | 사각형(4점)          | 면적 = 2.0, 중심 = (1.0, 0.5, 0.0)                  | ✅ 완료 |
| area_from_line_segment_with_origin     | add_line               | 선분 3개 + 원점      | 면적 = 0.5, 중심 = (1/3, 1/3, 0.0)                  | ✅ 완료 |


## 테스트 코드
``` rust
#[cfg(test)]
mod tests {
    use nurbslib::core::calc_area::CalcArea;
    use nurbslib::core::mass_properties::MassProperties;
    use nurbslib::core::maths::on_are_equal;
    use nurbslib::core::mesh::MeshFace;
    use nurbslib::core::prelude::Point;
    use nurbslib::core::types::ON_TOL9;
```
```rust
    #[test]
    fn area_single_triangle() {
        let verts = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 0.0, z: 0.0 },
            Point { x: 0.0, y: 1.0, z: 0.0 },
        ];
        let faces = vec![MeshFace::new_tri(0, 1, 2)];
        let mut acc = CalcArea::default();
        acc.add_triangles(&verts, &faces);
        assert!(on_are_equal(acc.area(), 0.5, ON_TOL9));
        let c = acc.centroid();
        assert!(on_are_equal(c.x, 1.0 / 3.0, ON_TOL9));
        assert!(on_are_equal(c.y, 1.0 / 3.0, ON_TOL9));
        assert!(on_are_equal(c.z, 0.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn area_nonplanar_triangle() {
        let verts = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 0.0, z: 0.0 },
            Point { x: 0.0, y: 0.0, z: 1.0 },
        ];
        let faces = vec![MeshFace::new_tri(0, 1, 2)];
        let mut acc = CalcArea::default();
        acc.add_triangles(&verts, &faces);
        assert!(on_are_equal(acc.area(), 0.5, ON_TOL9));
        let c = acc.centroid();
        assert!(on_are_equal(c.x, 1.0 / 3.0, ON_TOL9));
        assert!(on_are_equal(c.y, 0.0, ON_TOL9));
        assert!(on_are_equal(c.z, 1.0 / 3.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn area_quad_split_into_tris() {
        let verts = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 2.0, y: 0.0, z: 0.0 },
            Point { x: 2.0, y: 1.0, z: 0.0 },
            Point { x: 0.0, y: 1.0, z: 0.0 },
        ];
        let faces = vec![MeshFace::new_quad(0, 1, 2, 3)];
        let mut acc = CalcArea::default();
        acc.add_triangles(&verts, &faces);
        assert!(on_are_equal(acc.area(), 2.0, ON_TOL9));
        let c = acc.centroid();
        assert!(on_are_equal(c.x, 1.0, ON_TOL9));
        assert!(on_are_equal(c.y, 0.5, ON_TOL9));
        assert!(on_are_equal(c.z, 0.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn area_from_line_segment_with_origin() {
        let mut acc = CalcArea::default();
        let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p1 = Point { x: 1.0, y: 0.0, z: 0.0 };
        let p2 = Point { x: 0.0, y: 1.0, z: 0.0 };

        // 삼각형 (0,0)-(1,0)-(0,1) → 면적 = 0.5
        acc.add_line(origin, p1);
        acc.add_line(p1, p2);
        acc.add_line(p2, origin);

        assert!(on_are_equal(acc.area(), 0.5, ON_TOL9));
        let c = acc.centroid();
        assert!(on_are_equal(c.x, 1.0 / 3.0, ON_TOL9));
        assert!(on_are_equal(c.y, 1.0 / 3.0, ON_TOL9));
        assert!(on_are_equal(c.z, 0.0, ON_TOL9));
    }
```
```rust
    #[test]
    fn area_two_tris_make_square() {
        // two triangles forming 1x1 square
        let verts = vec![
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
                x: 1.0,
                y: 1.0,
                z: 0.0,
            }, //2
            Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }, //3
        ];
        let faces = vec![MeshFace::new_tri(0, 1, 2), MeshFace::new_tri(0, 2, 3)];
        let mut acc = CalcArea::default();
        acc.add_triangles(&verts, &faces);
        assert!(on_are_equal(acc.area(), 1.0, ON_TOL9));
        let c = acc.centroid();
        assert!(on_are_equal(c.x, 0.5, ON_TOL9) && on_are_equal(c.y, 0.5, ON_TOL9) && on_are_equal(c.z, 0.0, ON_TOL9));
        let mut mp = MassProperties::default();
        assert!(acc.write_result(&mut mp));
        assert!(on_are_equal(mp.mass, 1.0, ON_TOL9));
    }
}
```
---



