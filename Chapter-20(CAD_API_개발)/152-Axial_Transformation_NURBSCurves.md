# 📘 Axial Transformation for NURBS Curves
- (on_apply_axial_transform_to_curve Documentation)
- 이 문서는 Piegl & Tiller의 The NURBS Book 에서 소개된  
    축 기반 변형(Axial Deformation) 알고리즘을 Rust로 구현한 함수 
    on_apply_axial_transform_to_curve() 의 동작을 설명한다.
- 이 함수는 NURBS 곡선의 모든 컨트롤 포인트를 Euclidean 공간에서 변형한 뒤  
    다시 원래 weight를 적용하여 곡선의 형태를 바꾸는 기능을 수행한다.

## 1. 함수 개요
```rust
pub fn on_apply_axial_transform_to_curve<F>(
    curve: &mut NurbsCurve,
    shape: F,
    a: f64,
    tra: AxialTra,
    dir: AxialDir,
    cor: AxialCoord,
) where
    F: Fn(f64) -> f64 + Copy,
```

- 이 함수는 NURBS 곡선의 모든 control point에 대해 다음을 수행한다:
    - 동차 좌표 → Euclidean 좌표 변환
    - 축 기반 변형(Axial Transform) 적용
    - weight 재적용 후 다시 동차 좌표로 저장

## 2. 수학적 배경
### 2.1 Homogeneous → Euclidean 변환
- NURBS control point는 다음과 같은 4D homogeneous 형태이다:
```math
P_w=(x_w,y_w,z_w,w)
```
- 이를 Euclidean 좌표로 변환하면:
```math
P=(x,y,z)=\left( \frac{x_w}{w},\frac{y_w}{w},\frac{z_w}{w}\right) 
```

### 2.2 Axial Transform
- 축 기반 변형은 다음 네 가지 타입을 지원한다:
    - Pinch
    - Taper
    - Twist
    - Shear
- 각 변형은 다음과 같은 공통 구조를 가진다:
- 변형의 강도는 shape function f(t) 에 의해 결정된다.
- t 는 선택된 축(dir)의 좌표값이다.
- 전체 변형량은 $a\cdot f(t)$ 로 표현된다.
### 2.2.1 Pinch
- 특정 좌표(cor)만 scale:
```math
P'_{cor}=P_{cor}\cdot (a\cdot f(t))
```

### 2.2.2 Taper
- 축(dir)을 제외한 두 좌표를 scale:
- 예: dir = X 일 때
```math
y'=y\cdot (a\cdot f(x)),\quad z'=z\cdot (a\cdot f(x))
```

### 2.2.3 Twist
- 축을 중심으로 회전:
- 회전각:
```math
\alpha =\pi \cdot a\cdot f(t)
```
- 예: dir = Z 일 때
```math
\begin{aligned}x'&=x\cos \alpha -y\sin \alpha \\ y'&=x\sin \alpha +y\cos \alpha \end{aligned}
```

### 2.2.4 Shear
- 특정 좌표(cor)를 평행 이동:
```math
P'_{cor}=P_{cor}+a\cdot f(t)
```
### 2.3 변형 후 다시 weight 적용
- 변형된 Euclidean 좌표 P' 에 대해:
```math
P'_w=(x'w,\; y'w,\; z'w,\; w)
```
- 즉, weight는 변하지 않는다.

## 3. 알고리즘 요약
- 아래는 함수가 수행하는 전체 과정이다.
```
for each control point Pw[i]:
    1) Pw[i] → (P, w)
    2) P ← axial_transform(P)
    3) Pw[i] ← (P * w, w)
```


## 4. Rust 코드
```rust
pub fn on_apply_axial_transform_to_curve<F>(
    curve: &mut NurbsCurve,
    shape: F,
    a: f64,
    tra: AxialTra,
    dir: AxialDir,
    cor: AxialCoord,
) where
    F: Fn(f64) -> f64 + Copy,
{
    let n = curve.ctrl.len();

    for i in 0..n {
        let cpw = curve.ctrl[i];
        let w = cpw.w;

        // 1) Homogeneous → Euclidean
        let mut p = Point3D {
            x: cpw.x / w,
            y: cpw.y / w,
            z: cpw.z / w,
        };

        // 2) Apply axial transform
        on_axial_transform(&mut p, shape, a, tra, dir, cor);

        // 3) Euclidean → Homogeneous
        curve.ctrl[i].x = p.x * w;
        curve.ctrl[i].y = p.y * w;
        curve.ctrl[i].z = p.z * w;
    }
}
```

## 5. 예제
```rust
let shape_fn = |t: f64| t; // linear shape

on_apply_axial_transform_to_curve(
    &mut curve,
    shape_fn,
    0.5,                // amplitude
    AxialTra::Twist,    // twist deformation
    AxialDir::Z,        // twist around Z axis
    AxialCoord::X,      // ignored for twist
);

```

## 6. 특징 및 주의사항
- NURBS의 weight는 변하지 않는다
- 변형은 control point의 Euclidean 위치만 바꾼다
- knot vector, degree, control point 개수는 그대로 유지
- 곡선 전체가 변형되며, 부분 변형은 지원하지 않는다
- shape function은 B-spline 기반이든 analytic function이든 모두 가능

---



## 🎯 이 함수의 목족
- 이 변환은 곡선(또는 곡면)의 형태를 ‘직관적으로’ 조작하기 위한 고급 Shape Editing 도구다.
- 즉,
    - control point를 직접 움직이지 않고
    - weight를 건드리지 않고
    - knot를 건드리지 않고
    - NURBS 구조를 유지한 채
    - 곡선 전체의 형태를 “축 기반으로” 변형하는 기능 이다.
- 이건 인터랙티브 모델링, 디자인 스케치, 형상 최적화, 애니메이션 리깅, 형상 변형 알고리즘에서 매우 중요한 역할을 한다.

## 🧩 왜 이런 변형이 필요한가?
### 1) Control point를 직접 건드리는 건 너무 거칠다
- NURBS는 control point 하나를 움직이면 곡선 전체가 영향을 받는다.
- 하지만 디자이너는 종종 이렇게 원한다:
    - **곡선의 이 부분만 살짝 꼬아줘**
    - **여기서부터 끝까지 점점 가늘어지게 해줘**
    - **이 축을 기준으로 부드럽게 휘어지게 해줘**
    - **곡선 전체를 한쪽 방향으로 taper 시켜줘**
- Control point를 직접 건드리면 이런 걸 정교하게 만들기 어렵다.

### 2) Weight를 직접 조작하는 건 더 어렵다
- NURBS weight는 곡선의 “당김”을 조절하지만 직관적으로 다루기 어렵다.
- 그래서 Piegl & Tiller는 weight를 건드리지 않고도 곡선을 변형하는 방법을 제안했다.
- 그게 바로 axial transform이다.

### 3) Shape function을 이용하면 “부드러운 변형”이 가능하다
- shape(t)는 보통 B-spline function이다.
- 즉:
    - 변형이 시작되는 지점
    - 변형이 강해지는 지점
    - 변형이 사라지는 지점
    - 변형의 부드러움
- 이걸 모두 곡선의 파라미터 t로 제어할 수 있다.
- 이건 control point 조작으로는 절대 불가능한 정교함이다.

## 🔥 그래서 이 변환으로 할 수 있는 것들
### 1) Tapering (점점 가늘어지게)
- 곡선 또는 곡면을 한쪽 방향으로 점점 좁아지게 만들 수 있다.
- 예: 자동차 보닛, 항공기 날개, 병 목 부분.

### 2) Twisting (비틀기)
- 곡선을 축을 기준으로 회전시키는 변형.
- 예: 드릴 비트, 나선형 구조, 로프 형태.

### 3) Pinching (조이기)
- 특정 축을 기준으로 곡선을 조여서
- 곡선의 일부를 날카롭게 만들거나 좁히는 효과.
- 예: 병의 허리 부분, 곡면의 ridge 생성.

### 4) Shearing (전단 변형)
- 곡선을 한쪽 방향으로 밀어내는 변형.
- 예: 애니메이션 squash & stretch, 건축 구조물 변형.

### 5) Shape Optimization / Morphing
- 곡선의 형태를 부드럽게 바꾸면서 원래의 NURBS 구조는 유지.

### 6) Interactive Design Tools
- 디자이너가 슬라이더를 움직이면 곡선이 부드럽게 변형되는 UI를 만들 수 있다.

## 🧠 핵심 요약
- 이 변환은:
    - control point를 직접 건드리지 않고
    - weight도 건드리지 않고
    - knot도 건드리지 않고
    - NURBS 구조를 유지하면서
    - 곡선의 형태를 부드럽고 직관적으로 변형하기 위한 도구다.
- 즉,
- **곡선을 수학적으로 안전하게, 직관적으로 변형하는 고급 Shape Editing 기능**  
    이라고 보면 된다.

---

## 소스 코드
```rust
// Axial transformation (Piegl G_TRAAXL 포팅 버전).
///
/// - `p`  : in/out, 변환할 점
/// - `shape` : f(t)를 주는 shape function (원래 CFUN + N_cfnevn 역할)
/// - `a`  : amplitude (scaling factor)
/// - `tra`: PINCH / TAPER / TWIST / SHEAR
/// - `dir`: shape가 어떤 축의 좌표를 기준으로 평가되는지 (XDIR/YDIR/ZDIR)
/// - `cor`: 어떤 좌표를 변경할지 (XCRD/YCRD/ZCRD, PINCH/SHEAR에서만 의미)
pub fn on_axial_transform<F>(
    p: &mut Point3D,
    shape: F,
    a: f64,
    tra: AxialTra,
    dir: AxialDir,
    cor: AxialCoord,
) where
    F: Fn(f64) -> f64,
{
    // 1) 좌표 추출
    let mut x = p.x;
    let mut y = p.y;
    let mut z = p.z;

    // 2) 방향에 따라 shape function 평가 (N_cfnevn(cfn, x|y|z, left, &f))
    let t = match dir {
        AxialDir::X => x,
        AxialDir::Y => y,
        AxialDir::Z => z,
    };
    let f = shape(t);

    // 3) 변환 적용
    match tra {
        AxialTra::Pinch => match dir {
            AxialDir::X => match cor {
                AxialCoord::Y => y *= a * f,
                AxialCoord::Z => z *= a * f,
                AxialCoord::X => {}
            },
            AxialDir::Y => match cor {
                AxialCoord::X => x *= a * f,
                AxialCoord::Z => z *= a * f,
                AxialCoord::Y => {}
            },
            AxialDir::Z => match cor {
                AxialCoord::X => x *= a * f,
                AxialCoord::Y => y *= a * f,
                AxialCoord::Z => {}
            },
        },

        AxialTra::Taper => match dir {
            AxialDir::X => {
                y *= a * f;
                z *= a * f;
            }
            AxialDir::Y => {
                x *= a * f;
                z *= a * f;
            }
            AxialDir::Z => {
                x *= a * f;
                y *= a * f;
            }
        },

        AxialTra::Twist => {
            let alf = PI * a * f;
            let (s, c) = alf.sin_cos();
            match dir {
                AxialDir::X => {
                    let w = y;
                    y = c * w - s * z;
                    z = s * w + c * z;
                }
                AxialDir::Y => {
                    let w = x;
                    x = c * w + s * z;
                    z = -s * w + c * z;
                }
                AxialDir::Z => {
                    let w = x;
                    x = c * w - s * y;
                    y = s * w + c * y;
                }
            }
        }

        AxialTra::Shear => match dir {
            AxialDir::X => match cor {
                AxialCoord::Y => y += a * f,
                AxialCoord::Z => z += a * f,
                AxialCoord::X => {}
            },
            AxialDir::Y => match cor {
                AxialCoord::X => x += a * f,
                AxialCoord::Z => z += a * f,
                AxialCoord::Y => {}
            },
            AxialDir::Z => match cor {
                AxialCoord::X => x += a * f,
                AxialCoord::Y => y += a * f,
                AxialCoord::Z => {}
            },
        },
    }

    // 4) 결과 되돌려 쓰기 (in-place)
    p.x = x;
    p.y = y;
    p.z = z;
}
```
---

