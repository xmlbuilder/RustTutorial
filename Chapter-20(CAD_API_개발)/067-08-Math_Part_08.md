## on_rad_to_deg
- 두 함수 모두 가장 정석적이고 안전한 라디안 ↔ 도(degree) 변환.
- Rust 표준 라이브러리의 std::f64::consts::PI를 사용


### 📘 on_rad_to_deg
- 라디안을 도(degree)로 변환
```rust
pub fn on_rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}
```

- 공식:
```math
\mathrm{deg}=\mathrm{rad}\times \frac{180}{\pi }
```

### 📘 on_deg_to_rad
- 도를 라디안으로 변환
```rust
pub fn on_deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
```

- 공식:
```math
\mathrm{rad}=\mathrm{deg}\times \frac{\pi }{180}
```
- ✔️ 정확성
    - PI는 IEEE 754 double precision에서 가능한 가장 정확한 π 상수
    - 곱셈/나눗셈 순서도 안정적
    - 음수/큰 값/NaN/∞ 모두 자연스럽게 처리됨


```rust
pub fn on_rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}
```
```rust
pub fn on_deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
```

## on_determinant3_vectors

- 이 함수는 **3×3 행렬의 determinant(스칼라 삼중곱, scalar triple product)** 계산

### 📘 on_determinant3_vectors(v1, v2, v3)
- 3개의 3D 벡터를 행으로 갖는 3×3 행렬의 determinant 계산
```rust
pub fn on_determinant3_vectors(v1: Point3D, v2: Point3D, v3: Point3D) -> f64 {
    v1.x * v2.y * v3.z - v1.x * v2.z * v3.y
        - v1.y * v2.x * v3.z + v1.y * v2.z * v3.x
        + v1.z * v2.x * v3.y - v1.z * v2.y * v3.x
}
```


### 1️⃣ 수학적 의미
- 이 값은 다음과 동일하다:
```math
\det [v_1,v_2,v_3]=v_1\cdot (v_2\times v_3)
```
- 즉:
    - 세 벡터가 이루는 평행육면체의 부피
    - 오리엔테이션(부호 있는 부피)
    - 양수 → 오른손 좌표계
    - 음수 → 왼손 좌표계
    - 0 → 세 벡터가 선형종속

### 2️⃣ 행렬 형태로 보면
```math
\left| \begin{matrix}v_{1x}&v_{1y}&v_{1z}\\ v_{2x}&v_{2y}&v_{2z}\\ v_{3x}&v_{3y}&v_{3z}\end{matrix}\right|
``` 
- 이 determinant를 전개한 것이 바로 코드다.

### 3️⃣ 코드 검증
- 전개식:
```math
v_{1x}(v_{2y}v_{3z}-v_{2z}v_{3y})-v_{1y}(v_{2x}v_{3z}-v_{2z}v_{3x})+v_{1z}(v_{2x}v_{3y}-v_{2y}v_{3x})
```
- 코드와 1:1로 대응한다.


```rust
pub fn on_determinant3_vectors(v1: Point3D, v2: Point3D, v3: Point3D) -> f64 {
    v1.x * v2.y * v3.z - v1.x * v2.z * v3.y - v1.y * v2.x * v3.z
        + v1.y * v2.z * v3.x
        + v1.z * v2.x * v3.y
        - v1.z * v2.y * v3.x
}
```

## on_intersect_3d_lines_option

- 이 함수는 **3D에서 두 직선의 최근접점(또는 교점)**을 구하는 정석적인 해법 사용.

##3 📘 on_intersect_3d_lines_option
- 두 3D 직선 L1, L2의 파라미터 (s, t)와 최근접점 ip를 계산
- 직선 정의:
    - L1(s) = p1 + s·d1
    - L2(t) = p2 + t·d2
- 반환:
    - Some((s, t, ip))
    - ip = L1(s) = L2(t) (교점 또는 최근접점)
    - None
    - 두 직선이 거의 평행 → 해가 안정적으로 정의되지 않음

### 1️⃣ 수학적 배경
- 두 직선의 최근접점 조건은 다음 선형 시스템을 푸는 것과 같다:
```math
\begin{aligned}(d_1\cdot d_1)s-(d_1\cdot d_2)t&=d_1\cdot (p_2-p_1)\\ -(d_1\cdot d_2)s+(d_2\cdot d_2)t&=d_2\cdot (p_2-p_1)\end{aligned}
```
- 이를 행렬로 쓰면:
```math
\left[ \begin{matrix}a&-b\\ -b&c\end{matrix}\right] \left[ \begin{matrix}s\\ t\end{matrix}\right] =\left[ \begin{matrix}e\\ f\end{matrix}\right]
``` 
- 여기서:
    - a = d1·d1
    - b = d1·d2
    - c = d2·d2
    - e = d1·(p2−p1)
    - f = d2·(p2−p1)
- 해는:
```math
s=\frac{ec-fb}{ac-b^2},\quad t=\frac{eb-fa}{ac-b^2}
```
- 코드가 바로 이 공식을 구현한 것이다.

### 2️⃣ 코드 분석
```rust
let a = d1.dot(&d1);
let b = d1.dot(&d2);
let c = d2.dot(&d2);
let delta = p2 - p1;
let e = d1.dot(&delta);
let f = d2.dot(&delta);
let denom = a * c - b * b;
```
- denom = ac − b²
- denom ≈ 0 → 두 직선이 평행 또는 거의 평행
```rust
if denom.abs() <= 1e-15 * a.max(1.0) {
    return None;
}
```

- 이 조건은 수치적으로 평행한 경우를 안전하게 감지한다.

### 3️⃣ 해 계산
```rust
let s = (e * c - f * b) / denom;
let t = (e * b - f * a) / denom;
let ip = p1 + d1.scale(s);
```
- ip = L1(s)
- L1(s)와 L2(t)는 이론적으로 동일한 점  
    (수치 오차 때문에 약간 다를 수 있지만 충분히 근접)

### 4️⃣ 반환
```rust
Some((s, t, ip))
```

- s, t: 각 직선에서의 파라미터
- ip: 최근접점 또는 교점

### 5️⃣ 함수의 성격
- 이 함수는 두 직선이 실제로 교차하는지 여부를 판단하는 함수가 아니다.
    - denom ≠ 0 → skew lines(엇갈린 직선)도 최근접점이 존재
    - denom = 0 → 평행 → None
- 즉, 이 함수는:
    - 교점이 있으면 그 교점을 반환
    - 교점이 없어도 최근접점을 반환
    - 평행하면 None
- CAD/Geometry에서 매우 일반적인 패턴이다.


```rust
pub fn on_intersect_3d_lines_option(
    p1: Vector3D,
    d1: Vector3D,
    p2: Vector3D,
    d2: Vector3D,
) -> Option<(f64, f64, Vector3D)> {
    let a = d1.dot(&d1);
    let b = d1.dot(&d2);
    let c = d2.dot(&d2);
    let delta = p2 - p1;
    let e = d1.dot(&delta);
    let f = d2.dot(&delta);
    let denom = a * c - b * b;
    if denom.abs() <= 1e-15 * a.max(1.0) {
        return None;
    }
    let s = (e * c - f * b) / denom;
    let t = (e * b - f * a) / denom;
    let ip = p1 + d1.scale(s);
    Some((s, t, ip))
}
```
---


