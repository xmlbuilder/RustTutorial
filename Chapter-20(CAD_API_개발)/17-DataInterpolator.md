# DataInterpolator
## ✨ 개요
이 라이브러리는 다양한 형태의 보간(interpolation)을 지원합니다.  
주요 기능은 다음과 같습니다:
- 데이터 기반 보간: DataInterpolatord, DataInterpolatorf
- 스칼라 시퀀스 보간: Interpolator
- 다차원 보간: bilinear, trilinear
- 고급 보간 기법: cosine, cubic, Catmull-Rom, Lagrange

## 📐 1. Linear Interpolation (선형 보간)
### 📌 수식

$$
y=y_1\cdot (1-t)+y_2\cdot t\quad \mathrm{where}\quad t=\frac{x-x_1}{x_2-x_1}
$$

### 📦 구현
```rust
let t = ((x - x1) / (x2 - x1)).clamp(0.0, 1.0);
y1 * (1.0 - t) + y2 * t
```

### 🧠 설명
- x1, x2는 기준 점
- y1, y2는 해당 점의 값
- t는 보간 비율로, x가 x1과 x2 사이 어디에 위치하는지를 나타냄

## 🎚️ 2. Cosine Interpolation (코사인 보간)
### 📌 수식

$$
s=0.5\cdot (1-\cos (\pi \cdot t))\\ y=y_1\cdot (1-s)+y_2\cdot s
$$

### 🧠 설명
- t는 0~1 사이의 보간 비율
- cos 함수를 통해 부드러운 전환을 구현

## 🧮 3. Cubic Interpolation (3차 보간)
### 📌 수식

$$
y(t)=at^3+bt^2+ct+d
$$

$$
\begin{aligned}a=-0.5p_0+1.5p_1-1.5p_2+0.5p_3\quad b=p_0-2.5p_1+2p_2-0.5p_3\quad c=-0.5p_0+0.5p_2\quad d=p_1\end{aligned}
$$

### 🧠 설명
- p0~p3는 주변 4개의 샘플 값
- t는 보간 비율
- 부드러운 곡선 형태를 생성

## 🧬 4. Catmull-Rom Spline
### 📌 수식

$$
y(t)=0.5\cdot \left( 2p_1+(-p_0+p_2)t+(2p_0-5p_1+4p_2-p_3)t^2+(-p_0+3p_1-3p_2+p_3)t^3\right)
$$

### 🧠 설명
- 자연스러운 곡선 형태를 생성
- p1과 p2 사이를 보간하며, p0과 p3는 곡률 제어

## 🧠 5. Lagrange Interpolation
###📌 수식

$$
L_i(x)=\prod _{j=0,j\neq i}^n\frac{x-x_j}{x_i-x_j}\\ y(x)=\sum _{i=0}^ny_i\cdot L_i(x)
$$

### 🧠 설명
- 다항식 기반 보간
- 모든 점을 정확히 통과하는 곡선 생성
- 계산량이 많고, 진동 현상(Runge's phenomenon)이 발생할 수 있음

## 🧭 6. Bilinear & Trilinear Interpolation
### 📌 Bilinear 수식

$$
f(x,y)=(1-x)(1-y)f_{00}+x(1-y)f_{10}+(1-x)yf_{01}+xyf_{11}
$$

##📌 Trilinear 수식
$$
f(x,y,z)=\mathrm{선형\  보간을\  3축에\  대해\  반복}
$$

### 🧠 설명
- 2D 또는 3D 격자에서의 보간
- 주로 이미지 처리, 볼륨 렌더링 등에 사용

## 🧩 Interpolation Class Summary

| Name                   | Input Type             | Description                                      |
|------------------------|------------------------|--------------------------------------------------|
| `DataInterpolatord`    | `(f64, f64)`           | Double precision (64-bit) sample-based linear interpolation |
| `DataInterpolatorf`    | `(f32, f32)`           | Float precision (32-bit) sample-based linear interpolation |
| `Interpolator`         | `[f64]`                | Scalar sequence interpolator with multiple modes |
| `linear_multiple`      | `[f64], t: f64`        | Linear interpolation over a value array         |
| `cubic_multiple`       | `[f64], t: f64`        | Cubic interpolation over a value array          |
| `lagrange_interp`      | `[f64], [f64], x: f64` | Lagrange polynomial interpolation                |
| `catmull_rom4`         | `4×f64, t: f64`        | Catmull-Rom interpolation using 4 control points |
| `bi_linear` / `tri_linear` | `grid + t`         | 2D / 3D grid-based interpolation                 |

---


