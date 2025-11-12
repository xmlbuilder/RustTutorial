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

### 소스
```rust
use std::cmp::Ordering;
use interpolation::Lerp;
use crate::core::geom::{Point2, Vector2};
use crate::core::prelude::{Point, Real, Vector};

/// Linear interpolation from sorted (x, y) sample points (double precision)
#[derive(Default, Clone)]
pub struct DataInterpolatord {
    src: Vec<(f64, f64)>, // Maintain ascending order of x values
}
impl DataInterpolatord {
    pub fn new() -> DataInterpolatord {
        DataInterpolatord { src: Vec::new() }
    }

    pub fn add_source(&mut self, x: f64, y: f64) {
        let p = (x, y);
        match self
            .src
            .binary_search_by(|(xx, _)| xx.partial_cmp(&x).unwrap_or(Ordering::Less))
        {
            Ok(pos) => self.src.insert(pos, p),
            Err(pos) => self.src.insert(pos, p),
        }
    }
    pub fn get_value(&self, x: f64) -> f64 {
        if self.src.is_empty() {
            return 0.0;
        }
        if x <= self.src[0].0 {
            return self.src[0].1;
        }
        if x >= self.src[self.src.len() - 1].0 {
            return self.src[self.src.len() - 1].1;
        }

        let i = match self
            .src
            .binary_search_by(|(xx, _)| xx.partial_cmp(&x).unwrap_or(Ordering::Less))
        {
            Ok(pos) => return self.src[pos].1, // 정확히 일치
            Err(pos) => pos,
        };
        let (x1, y1) = self.src[i - 1];
        let (x2, y2) = self.src[i];
        let t = ((x - x1) / (x2 - x1)).clamp(0.0, 1.0);
        y1 * (1.0 - t) + y2 * t
    }
    pub fn sources(&self) -> &[(f64, f64)] {
        &self.src
    }
}
```
```rust
/// Linear interpolation from sorted (x, y) sample points (float)
#[derive(Default, Clone)]
pub struct DataInterpolatorf {
    src: Vec<(f32, f32)>, // Maintain ascending order of x values
}
impl DataInterpolatorf {
    pub fn new() -> Self {
        DataInterpolatorf { src: Vec::new() }
    }
    pub fn add_source(&mut self, x: f32, y: f32) {
        let p = (x, y);
        match self
            .src
            .binary_search_by(|(xx, _)| xx.partial_cmp(&x).unwrap_or(Ordering::Less))
        {
            Ok(pos) => self.src.insert(pos, p),
            Err(pos) => self.src.insert(pos, p),
        }
    }
    pub fn get_value(&self, x: f32) -> f32 {
        if self.src.is_empty() {
            return 0.0;
        }
        if x <= self.src[0].0 {
            return self.src[0].1;
        }
        if x >= self.src[self.src.len() - 1].0 {
            return self.src[self.src.len() - 1].1;
        }

        let i = match self
            .src
            .binary_search_by(|(xx, _)| xx.partial_cmp(&x).unwrap_or(Ordering::Less))
        {
            Ok(pos) => return self.src[pos].1, // Exact match
            Err(pos) => pos,
        };
        let (x1, y1) = self.src[i - 1];
        let (x2, y2) = self.src[i];
        let t = ((x - x1) / (x2 - x1)).clamp(0.0, 1.0);
        y1 * (1.0 - t) + y2 * t
    }
    pub fn sources(&self) -> &[(f32, f32)] {
        &self.src
    }
}
```

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
### 📌 수식

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
### 소스 코드
```rust
/// Scalar sequence interpolation (uniformly spaced index domain)
#[derive(Default, Clone)]
pub struct Interpolator {
    values: Vec<f64>,
    cyclical: bool,
}
```
```rust
impl Interpolator {
    pub fn new(values: Vec<f64>, cyclical: bool) -> Self {
        Self { values, cyclical }
    }
    pub fn set_values(&mut self, v: Vec<f64>) {
        self.values = v;
    }
    pub fn set_cyclical(&mut self, c: bool) {
        self.cyclical = c;
    }
```
```rust
    #[inline]
    fn map_index(&self, idx: i32) -> usize {
        let n = self.values.len() as i32;
        if self.cyclical {
            let m = if n == 0 { 1 } else { n };
            let k = ((idx % m) + m) % m;
            k as usize
        } else {
            idx.clamp(0, n.saturating_sub(1)) as usize
        }
    }
```
```rust
    fn solve_param(&self, t: f64) -> (usize, f64) {
        let n = self.values.len();
        assert!(n >= 2, "Interpolator needs at least 2 values");
        let last = (n - 1) as f64;
        if self.cyclical {
            let base = t.floor();
            let i = self.map_index(base as i32);
            let local = (t - base).clamp(0.0, 1.0);
            (i, local)
        } else {
            if t <= 0.0 {
                return (0, 0.0);
            }
            if t >= last {
                return (n - 2, 1.0);
            }
            let base = t.floor();
            let i = base as usize;
            let local = (t - base).clamp(0.0, 1.0);
            (i, local)
        }
    }
```
```rust
    pub fn nearest(&self, t: f64) -> f64 {
        let (i, local) = self.solve_param(t);
        let j = self.map_index((i as i32) + 1);
        if local >= 0.5 {
            self.values[j]
        } else {
            self.values[i]
        }
    }
```
```rust
    pub fn linear(&self, t: f64) -> f64 {
        let (i, local) = self.solve_param(t);
        let j = self.map_index((i as i32) + 1);
        self.values[i] * (1.0 - local) + self.values[j] * local
    }
```
```rust
    pub fn cosine(&self, t: f64) -> f64 {
        let (i, local) = self.solve_param(t);
        let j = self.map_index((i as i32) + 1);
        let s = 0.5 * (1.0 - (local * std::f64::consts::PI).cos());
        self.values[i] * (1.0 - s) + self.values[j] * s
    }
```
```rust
    pub fn cubic(&self, t: f64) -> f64 {
        let (i, local) = self.solve_param(t);
        let i0 = self.map_index((i as i32) - 1);
        let i1 = i;
        let i2 = self.map_index((i as i32) + 1);
        let i3 = self.map_index((i as i32) + 2);

        let p0 = self.values[i0];
        let p1 = self.values[i1];
        let p2 = self.values[i2];
        let p3 = self.values[i3];

        let a = -0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3;
        let b = p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3;
        let c = -0.5 * p0 + 0.5 * p2;
        let d = p1;

        ((a * local + b) * local + c) * local + d
    }
```
```rust
    pub fn catmull_rom(&self, t: f64) -> f64 {
        let (i, local) = self.solve_param(t);
        let i0 = self.map_index((i as i32) - 1);
        let i1 = i;
        let i2 = self.map_index((i as i32) + 1);
        let i3 = self.map_index((i as i32) + 2);

        let p0 = self.values[i0];
        let p1 = self.values[i1];
        let p2 = self.values[i2];
        let p3 = self.values[i3];

        let t2 = local * local;
        let t3 = t2 * local;

        let a = -0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3;
        let b = p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3;
        let c = -0.5 * p0 + 0.5 * p2;
        let d = p1;

        a * t3 + b * t2 + c * local + d
    }
```
```rust
    pub fn linear_multiple(values: &[f64], t: f64) -> f64 {
        let n = values.len();
        if n == 0 {
            return 0.0;
        }
        if n == 1 || t <= 0.0 {
            return values[0];
        }
        if t >= 1.0 {
            return values[n - 1];
        }
        let scaled = t * (n as f64 - 1.0);
        let i = scaled.floor() as usize;
        let local = scaled - i as f64;
        let i = i.min(n - 2);
        values[i] * (1.0 - local) + values[i + 1] * local
    }
```
```rust
    pub fn cubic_multiple(values: &[f64], t: f64) -> f64 {
        let n = values.len();
        if n < 4 {
            return Self::linear_multiple(values, t);
        }
        let scaled = t * (n as f64 - 1.0);
        let mut i = scaled.floor() as i32;
        let local = scaled - (i as f64);
        i = i.clamp(1, (n as i32) - 3);
        let p0 = values[(i - 1) as usize];
        let p1 = values[i as usize];
        let p2 = values[(i + 1) as usize];
        let p3 = values[(i + 2) as usize];

        let t2 = local * local;
        let t3 = t2 * local;

        0.5 * (2.0 * p1
            + (p2 - p0) * local
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
            + (3.0 * p1 - p0 - 3.0 * p2 + p3) * t3)
    }
```
```rust
    pub fn lagrange_interp(xs: &[f64], ys: &[f64], x: f64) -> f64 {
        let n = xs.len();
        let mut y = 0.0;
        for i in 0..n {
            let mut li = 1.0;
            for j in 0..n {
                if i != j {
                    li *= (x - xs[j]) / (xs[i] - xs[j]);
                }
            }
            y += ys[i] * li;
        }
        y
    }
```
```rust
    pub fn catmull_rom4(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
        let t2 = t * t;
        let t3 = t2 * t;
        0.5 * (2.0 * p1
            + (-p0 + p2) * t
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
            + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
    }
```
```rust
    #[allow(unused)]
    fn bi_linear(f: [[f64; 2]; 2], tx: f64, ty: f64) -> f64 {
        // f[0][0] = (0,0), f[0][1]=(0,1), f[1][0]=(1,0), f[1][1]=(1,1)
        let a = f[0][0] * (1.0 - tx) + f[1][0] * tx;
        let b = f[0][1] * (1.0 - tx) + f[1][1] * tx;
        a * (1.0 - ty) + b * ty
    }
```
```rust
    #[allow(unused)]
    fn tri_linear(f: [[[f64; 2]; 2]; 2], tx: f64, ty: f64, tz: f64) -> f64 {
        let c00 = f[0][0][0] * (1.0 - tx) + f[1][0][0] * tx;
        let c01 = f[0][0][1] * (1.0 - tx) + f[1][0][1] * tx;
        let c10 = f[0][1][0] * (1.0 - tx) + f[1][1][0] * tx;
        let c11 = f[0][1][1] * (1.0 - tx) + f[1][1][1] * tx;

        let c0 = c00 * (1.0 - ty) + c10 * ty;
        let c1 = c01 * (1.0 - ty) + c11 * ty;
        c0 * (1.0 - tz) + c1 * tz
    }
}
```
---

