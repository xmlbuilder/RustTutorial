# Align Curve
## 🎯 이 함수가 하는 일 (핵심 요약)
- on_align_curve_to_frame = **곡선을 특정 좌표계에 정렬(align)하는 함수**
- 즉:
    - 곡선의 Frenet frame (T, N, B)을
    - 사용자가 지정한 좌표계 (X, Y, Z) 에
    - 정확히 일치하도록
    - 곡선 전체를 변환(회전 + 이동) 한다.
- 그리고 이 변환은 in-place, 즉 곡선 자체를 직접 바꾼다.

## 🧠 왜 이런 기능이 필요할까? (용도)
- 이건 CAD/Geometry 엔진에서 엄청 자주 쓰이는 기능.
### ✔ 1) 곡선을 “정렬”해서 작업하기 쉽게 만들기
- 예를 들어:
    - 어떤 곡선이 3D 공간에서 이상한 방향으로 누워있다
    - 이걸 XY 평면에 정렬해서 2D 스케치처럼 쓰고 싶다
    - 또는 특정 방향을 기준으로 회전시키고 싶다
- 이때:
    - 곡선의 Frenet T(접선)를 X축에 맞추고
    - N(법선)을 Y축에 맞추고
    - B(이단법선)을 Z축에 맞추면
    - 곡선이 정확히 원하는 좌표계로 정렬된다.

### ✔ 2) 스윕(sweep), 로프트(loft) 같은 CAD 기능의 기반
- 스윕 프로파일을 만들 때:
    - 프로파일 곡선을
    - 스윕 경로의 Frenet frame에 맞춰서
    - 계속 회전시키면서 따라가야 한다
- 이때 바로 이 함수가 필요함.

### ✔ 3) 곡선의 “기준점”을 특정 좌표계로 옮기기
- 예:
    - 곡선의 u=0.3 지점이
    - 어떤 좌표계의 원점(O)에 오도록 하고 싶다
    - 그리고 그 지점의 접선이 X축을 향하도록 하고 싶다
- 이걸 자동으로 해주는 게 on_align_curve_to_frame.

### ✔ 4) 곡선을 **평탄화(flatten)** 하거나 **정규화(normalize)** 할 때
- 곡선을:
    - XY 평면에 눕히거나
    - Z축을 기준으로 회전시키거나
    - 특정 방향으로 정렬하고 싶을 때
    - 이 함수가 바로 그 역할을 한다.

## 🧩 함수 내부 동작을 단계별로 설명하면
### 1) 곡선이 3D인지 확인
```rust
match self.dimension {
    3 => { /* ok */ }
    2 => {
        for cp in &mut self.ctrl {
            cp.z = 0.0;
        }
        self.dimension = 3;
    }
    _ => panic!("Unsupported curve dimension"),
}
```
- 2D 곡선이면 3D로 확장.

### 2) u에서 Frenet frame 계산
```rust
#[derive(Debug)]
pub struct FrenetFrame {
    pub p: Point3D,
    pub t: Vector3D,
    pub n: Vector3D,
    pub b: Vector3D,
}
```
```rust
pub fn frenet_frame_struct(&self, u: Real) -> Option<FrenetFrame>
```
- p = 곡선의 점
- t = 접선
- n = 법선
- b = 이단법선

### 3) 변환 행렬 생성
```rust
change_of_basis(P,T,N,B,O,X,Y,Z);

pub fn change_of_basis(
    a_o: Point3D,
    a_x: Vector3D,
    a_y: Vector3D,
    a_z: Vector3D,
    b_o: Point3D,
    b_x: Vector3D,
    b_y: Vector3D,
    b_z: Vector3D,
) -> Option<Self> {
    let a2w = Self::from_basis(a_o, a_x, a_y, a_z);
    let b2w = Self::from_basis(b_o, b_x, b_y, b_z);
    let w2b = b2w.inverse()?;
    Some(w2b.mul(a2w))
}
```
```rust
pub fn to_basis(origin: Point3D, x: Vector3D, y: Vector3D, z: Vector3D) -> Option<Self> {
    Self::from_basis(origin, x, y, z).inverse()
}
```
```rust
pub fn from_basis(origin: Point3D, x: Vector3D, y: Vector3D, z: Vector3D) -> Self {
    let mut m = [[0.0; 4]; 4];
    m[0][0] = x.x;
    m[0][1] = y.x;
    m[0][2] = z.x;
    m[0][3] = origin.x;
    m[1][0] = x.y;
    m[1][1] = y.y;
    m[1][2] = z.y;
    m[1][3] = origin.y;
    m[2][0] = x.z;
    m[2][1] = y.z;
    m[2][2] = z.z;
    m[2][3] = origin.z;
    m[3][3] = 1.0;
    Self { m }
}
```
- 이 함수는:
    - (P, T, N, B) → (O, X, Y, Z)
    - 이렇게 매핑하는 정확한 rigid transform을 만든다.
- 즉:
    - 곡선의 Frenet frame을
    - 사용자가 원하는 좌표계로
    - 정확히 회전 + 이동시키는 행렬을 만든다.

### 4) 곡선 전체를 변환
```rust
pub fn transform_in_place(&mut self, mat: &Matrix4)
```

- 이제 곡선 전체가 새 좌표계에 정렬된다.

## 🦀 Rust로 옮기면 구조는 이렇게 된다
- Rust에서는 보통 이런 식으로 구조화한다:
```rust
pub fn on_align_curve_to_frame(
    curve: &mut NurbsCurve,
    u: Real,
    origin: Point3D,
    x_axis: Vector3D,
    y_axis: Vector3D,
    z_axis: Vector3D,
) -> Result<()> {
    // 1) ensure 3D (transform_in_place와 동일한 로직 재사용)
    match curve.dimension {
        3 => { /* ok */ }
        2 => {
            for cp in &mut curve.ctrl {
                cp.z = 0.0;
            }
            curve.dimension = 3;
        }
        _ => {
            return Err(NurbsError::InvalidInput {
                msg: "align_curve_to_frame: unsupported curve dimension".into(),
            })
        }
    }

    // 2) Frenet frame at parameter u
    let (p, t, n, b) = curve
        .frenet_frame(u)
        .ok_or_else(|| NurbsError::InvalidInput {
            msg: "align_curve_to_frame: Frenet frame undefined at given u".into(),
        })?;

    // 3) build transform matrix: current frame (p,t,n,b) -> target frame
    let m = Xform::from_frame_to_frame(p, t, n, b, origin, x_axis, y_axis, z_axis)
        .ok_or_else(|| NurbsError::NumericError {
            msg: "align_curve_to_frame: singular frame transform".into(),
        })?;

    // 4) apply transform to curve control points
    curve.transform_xform(&m);

    Ok(())
}
```

- 이 함수는 고급 기하학 기능.
- 특히:
    - 스윕
    - 로프트
    - 프로파일 정렬
    - 곡선 기반 피처 생성
    - CAD 스케치 정렬

## 🔥 한 문장으로 요약
- on_align_curve_to_frame = **곡선의 Frenet frame을 원하는 좌표계에 정확히 맞춰서 곡선을 회전/이동시키는 함수**

---

##  함수가 하는 일 요약
- frenet_frame는 주어진 파라미터 u에서 NURBS 곡선의 Frenet frame을 계산하는 함수.
- 입력:
    - cur: NURBS 곡선
    - u: 파라미터 값
- 출력:
    - p: 곡선 위의 점
    - t: 단위 접선 벡터 (tangent)
    - n: 단위 법선 벡터 (normal)
    - b: 단위 이단법선 벡터 (binormal)
- 내부에서 하는 일:
    - 곡선 차수 p 확인 (p <= 1이면 Frenet frame 정의 불가 → 에러)
    - eval_ders_padded로 곡선의 0,1,2차 미분 벡터 ders[0], ders[1], ders[2] 계산
    - ders[0] = 위치 P
    - ders[1] = 1차 미분 (속도 벡터)
    - ders[2] = 2차 미분 (가속도 벡터)
    - t = ders[1] 정규화
    - b = ders[1] × ders[2] → 정규화
    - n = b × t
- 즉, 표준 Frenet frame 정의 그대로 구현한 함수.

```rust
#[derive(Debug)]
pub struct FrenetFrame {
    pub p: Point3,
    pub t: Vector3,
    pub n: Vector3,
    pub b: Vector3,
}
```
```rust
/// Frenet frame (p, T, N, B).
/// - 직선(곡률 ≈ 0)인 경우에도 T 는 정의되고,
///   N/B 는 고정 기준축에서 안정적으로 만들어서 Some(...) 을 리턴한다.
pub fn frenet_frame(&self, u: Real) -> Option<(Point3D, Vector3D, Vector3D, Vector3D)> {
    let ders = self.eval_ders_padded(u, 2);
    if ders.len() < 3 {
        return None;
    }

    let p = self.eval_point(u);
    let d1 = ders[1];
    let d2 = ders[2];

    let eps = 1e-12;

    let speed = d1.length();
    if speed <= eps {
        // 접선 자체가 0이면 프레임 정의 불가
        return None;
    }

    // 단위 접선 벡터
    let t = d1 / speed;

    // 곡률이 충분한지 확인
    let cross = d1.cross(&d2);
    let cross_len = cross.length();

    if cross_len <= eps {
        // === 직선 혹은 평면에서 직선 구간: 곡률 ~ 0 ===
        // T 는 정의되지만, N/B 는 임의의 직교 프레임을 만들어야 함.

        // 우선 기본 "위쪽" 벡터를 하나 고른다.
        let mut up = Vector3D::new(0.0, 0.0, 1.0);
        // 만약 T 가 z축과 거의 평행이면, 다른 축을 사용
        if up.cross(&t).length() <= eps {
            up = Vector3D::new(0.0, 1.0, 0.0);
        }

        // up 에서 T 성분을 제거해서 N 후보를 만든다.
        let up_dot_t = up.dot(&t);
        let n_temp = up - t * up_dot_t;
        let n_len = n_temp.length();
        if n_len <= eps {
            // 이론상 거의 안 오지만, 혹시 수치적으로 또 꼬이면 실패 처리
            return None;
        }
        let n = n_temp / n_len;

        // B = T × N
        let b = t.cross(&n);

        return Some((p, t, n, b));
    }

    // === 일반적인 곡선: 표준 Frenet frame ===
    let b = cross / cross_len; // binormal
    let n = b.cross(&t).unitize(); // normal

    Some((p, t, n, b))
}

/// Frenet frame을 FrenetFrame 구조체로 감싸서 반환하는 wrapper.
pub fn frenet_frame_struct(&self, u: Real) -> Option<FrenetFrame> {
    self.frenet_frame(u).map(|(p, t, n, b)| FrenetFrame { p, t, n, b })
}
```

- 이 frenet_frame은:
    - on_align_curve_to_frame 같은 곡선 정렬 함수의 기반
    - 스윕/로프트에서 프로파일을 경로에 따라 회전시키는 기준
    - 곡선 위에서 로컬 좌표계(local frame) 를 만들 때 필수
    - 곡률, 비틀림, 곡선 분석 등에도 사용 가능
- 즉, **곡선 위의 좌표계** 를 만드는 핵심 도구.

---


## 1) frenet_frame — Frenet Frame 계산 (곡선 위의 로컬 좌표계 생성)
- 📌 수학적 정의
- NURBS 곡선 C(u) 에 대해:
    - 0차 미분: C(u) → 곡선 위의 점 P
    - 1차 미분: C'(u) → 접선 벡터 T
    - 2차 미분: C''(u) → 곡률 방향 계산에 사용
- Frenet Frame은 다음과 같이 정의된다:
### 1. 단위 접선
```math
\mathbf{T}(u)=\frac{C'(u)}{\| C'(u)\| }
```
### 2. 이단법선 (binormal)
```math
\mathbf{B}(u)=\frac{C'(u)\times C''(u)}{\| C'(u)\times C''(u)\| }
```

### 3. 법선
```math
\mathbf{N}(u)=\mathbf{B}(u)\times \mathbf{T}(u)
```
- 결과적으로 Frenet Frame은:
```math
\{ \mathbf{T}(u),\mathbf{N}(u),\mathbf{B}(u)\}
``` 
- 이는 곡선 위의 정규 직교 좌표계(orthonormal frame) 이다.

- 📌 기하학적 의미
    - 곡선의 방향(T)
    - 곡선의 곡률 방향(N)
    - 곡선의 비틀림 방향(B)
    - 을 나타내는 곡선 고유의 로컬 좌표계다.
즉, 곡선 위의 **기준 좌표계** 를 만드는 함수.

## 📌 Sweep에서의 역할
- 스윕은 다음을 반복하는 과정이다:
- **프로파일을 곡선의 Frenet Frame에 맞춰서 계속 회전시키며 이동시키는 것**

- 따라서:
    - 스윕 경로의 방향 = T
    - 프로파일의 회전 기준 = N, B
    - 프로파일의 위치 = P
- 즉, 스윕의 핵심 좌표계를 만드는 함수가 바로 frenet_frame.

## 2) on_align_curve_to_frame — 곡선을 특정 좌표계에 정렬(align)
- 📌 수학적 정의
- 곡선의 Frenet Frame (P,T,N,B) 을  
    사용자가 원하는 좌표계 (O,X,Y,Z) 에 정렬시키는 변환 행렬 M 을 만든다.
- 즉:
```math
M\cdot P=O
```
```math
M\cdot T=X
```
```math
M\cdot N=Y
```
```math
M\cdot B=Z
```
- 이 조건을 만족하는 유일한 rigid transform을 계산한다.

- 📌 기하학적 의미
    - 곡선의 특정 지점에서
    - 곡선의 방향(T), 법선(N), 이단법선(B)을
    - 사용자가 원하는 좌표계(X, Y, Z)에 정확히 맞춘다.
- 즉:
    - **곡선을 원하는 좌표계로 회전 + 이동시키는 함수**


- 📌 Sweep에서의 역할
    - 스윕 프로파일을 만들 때:
        - 프로파일을 스윕 경로의 Frenet Frame에 맞춰야 한다
        - 즉, 프로파일의 X,Y,Z 축을 곡선의 T,N,B 축에 맞춰야 한다
    - 이때 필요한 것이 바로 정렬(align) 이다.
- on_align_curve_to_frame는:
    - 프로파일을 곡선의 로컬 프레임에 맞추는 데 사용
    - 또는 곡선을 특정 기준 좌표계로 정렬할 때 사용
- 즉, 스윕의 좌표계 매칭 단계를 담당한다.

## 3) on_align_curve_to_frame — 곡선에 4×4 변환 행렬 적용
- 📌 수학적 정의
    - NURBS 곡선은 동차 좌표로 표현된다:
```math
C^w(u)=\sum N_{i,p}(u)P_i^w
```
- 4×4 행렬 M 을 적용하면:
```math
P_i'^w=MP_i^w
```
- 새 곡선:
```math
C'^w(u)=\sum N_{i,p}(u)P_i'^w=MC^w(u)
```

```rust

pub fn transform_xform(&mut self, xform: &Xform) {
    for p in &mut self.ctrl {
        *p = xform.multi_point4_left(p);
    }
    // 2. Reverse orientation (e.g., when matrix determinant < 0)
    if xform.is_reversed() {
        self.kv.reverse();
    }
}


pub fn multi_point4_left(&self, p: &Point4D) -> Point4D {
    let x = self.m[0][0] * p.x + self.m[0][1] * p.y + self.m[0][2] * p.z + self.m[0][3] * p.w;
    let y = self.m[1][0] * p.x + self.m[1][1] * p.y + self.m[1][2] * p.z + self.m[1][3] * p.w;
    let z = self.m[2][0] * p.x + self.m[2][1] * p.y + self.m[2][2] * p.z + self.m[2][3] * p.w;
    let w = self.m[3][0] * p.x + self.m[3][1] * p.y + self.m[3][2] * p.z + self.m[3][3] * p.w;

    Point4D { x, y, z, w }
}

pub fn is_reversed(&self) -> bool {
    self.determinant3() < 0.0
}

pub fn determinant3(&self) -> Real {
    let num1 = self.m[2][2];
    let val1 = self.m[1][1];
    let val2 = self.m[1][2];
    let val3 = self.m[1][0];
    let val4 = self.m[2][0];
    let val5 = self.m[2][1];
    self.m[0][0] * (val1 * num1 - val5 * val2) - self.m[0][1] * (val3 * num1 - val4 * val2)
        + self.m[0][2] * (val3 * val5 - val4 * val1)
}

```

- 즉:
    - 컨트롤 포인트에 행렬을 곱하는 것은 곡선 전체에 동일한 기하학 변환을 적용하는 것과 완전히 동일하다.

- 📌 기하학적 의미
    - 곡선을 회전
    - 곡선을 이동
    - 곡선을 스케일
    - 곡선을 쉐어
- 등 모든 아핀 변환을 곡선 전체에 적용한다.

- 📌 Sweep에서의 역할
    - 스윕은 다음을 반복한다:
    - 스윕 경로의 Frenet Frame 계산
    - 프로파일을 그 프레임에 맞춰 정렬
    - 그 위치로 이동
    - 변환된 프로파일을 surface로 연결

---

## align_curve_to_frame flow chart
- 함수가 내부에서 어떤 순서로 동작하는지 Flowchart 형태로 표현하면 이렇게 된다.

### 🧩 align_curve_to_frame 
- Flowchart
```
 ┌──────────────────────────────────────────────┐
 │            align_curve_to_frame()            │
 └──────────────────────────────────────────────┘
                      │
                      ▼
        ┌────────────────────────────┐
        │ 1. curve.ensure_3d()       │
        │ - 2D면 z=0으로 승격         │
        └────────────────────────────┘
                      │
                      ▼
        ┌────────────────────────────┐
        │ 2. Frenet frame 계산        │
        │ (p, t, n, b) = frenet(u)   │
        └────────────────────────────┘
                      │
                      ▼
        ┌──────────────────────────────────────────┐
        │ 3. 현재 프레임 A = (p, t, n, b)           │
        │    목표 프레임 B = (origin, x, y, z)      │
        │    A → B 로 보내는 변환행렬 M 생성         │
        │    M = B2W * inverse(A2W)                │
        └──────────────────────────────────────────┘
                      │
                      ▼
        ┌────────────────────────────┐
        │ 4. curve.apply_transform(M) │
        │ - 모든 control point 변환   │
        │ - det<0 이면 knot reverse   │
        └────────────────────────────┘
                      │
                      ▼
        ┌────────────────────────────┐
        │ 5. 완료 (Ok)               │
        └────────────────────────────┘
```

## 🔍 핵심 요약
- Step 1: 곡선이 2D면 3D로 승격
- Step 2: u에서 Frenet frame 계산
- Step 3: 현재 프레임 → 목표 프레임 변환행렬 생성
- Step 4: 변환행렬을 곡선 전체에 적용
- Step 5: det<0 이면 orientation reverse
- Step 6: 완료

---

