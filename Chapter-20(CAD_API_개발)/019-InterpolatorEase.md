# InterpolatorEase
- InterpolatorEase 및 관련 보간 함수들의 수식, 원리, 설명을 포함한 문서와 함께 표 형식 요약도 정리한 것입니다.  
- 이 문서는 easing 함수와 보간 로직을 이해하고 활용하는 데 도움이 됩니다.

## 📘 Interpolation & Easing Function Summary
### 🎯 핵심 개념
- 보간(interpolation): 두 값 사이의 중간값을 계산하는 것
- 이징(easing): 시간 또는 진행률에 따라 보간 속도를 조절하는 함수
- Hermite, Bézier, Bounce, Elastic 등 다양한 곡선 형태를 통해 부드러운 애니메이션 효과 구현

## 📐 주요 수식 정리

| 함수 이름               | 수식 또는 원리                              | 입력 / 설명                      |
|------------------------|---------------------------------------------|----------------------------------|
| `on_lerp_f64`          | $z = (1 - t) \cdot x + t \cdot y$       | 선형 보간 (f64)                  |
| `on_lerp_i32`          | `on_lerp_f64` 후 `round()` → `i32` 변환     | 정수형 선형 보간                 |
| `on_factorial_u64`     | $n! = \prod_{i=1}^{n} i$                | 반복 기반 팩토리얼 계산         |
| `on_hermite_spline_2d` | Hermite basis: $P(t) = h_{00}p_0 + h_{10}t_0 + h_{01}p_1 + h_{11}t_1$ | 2D 위치 + 속도 기반 곡선 보간   |
| `on_hermite_color_rgb` | Hermite 곡선의 y값을 RGB 보간 인자로 사용  | 부드러운 색상 전환               |


## 🎨 Easing 함수 요약표
| Function               | Formula / Behavior                          | Description                          |
|------------------------|---------------------------------------------|--------------------------------------|
| `smooth_step`          | $t^2 (3 - 2t)$                          | Hermite 기반 부드러운 전환          |
| `quadratic_in`         | $t^2$                                   | 느린 시작                            |
| `quadratic_out`        | $t(2 - t)$                              | 빠른 시작                            |
| `quadratic_in_out`     | $2t^2$ or $-1 + (4 - 2t)t$          | 양쪽 부드러운 전환                  |
| `cubic_in`             | $t^3$                                   | 매우 느린 시작                       |
| `cubic_out`            | $(t - 1)^3 + 1$                         | 빠른 시작, 느린 끝                   |
| `cubic_in_out`         | $4t^3$ or $0.5(2t - 2)^3 + 1$       | 양쪽 부드러운 전환                  |
| `exponential_in`       | $2^{10(t - 1)}$                         | 매우 느린 시작                       |
| `exponential_out`      | $1 - 2^{-10t}$                          | 매우 빠른 시작                       |
| `exponential_in_out`   | 복합 exponential                            | 양쪽 극단적 전환                    |
| `bounce_out`           | piecewise quadratic                         | 튕기는 효과                          |
| `elastic_out`          | $2^{-10t} \cdot \sin(2\pi t / p) + 1$   | 스프링처럼 튀는 효과                |
| `back_out`             | overshoot then settle                       | 살짝 넘었다가 돌아오는 효과         |



## 🧪 Hermite 보간 수식
```
on_hermite_spline_2d(p0, t0, p1, t1, t)
```

$$
\begin{aligned}h_{00}=2t^3-3t^2+1 ,& h_{10}=t^3-2t^2+t ,& h_{01}=-2t^3+3t^2 ,& h_{11}=t^3-t^2 ,& P(t)=h_{00}p_0+h_{10}t_0+h_{01}p_1+h_{11}t_1\end{aligned}
$$

## 🌈 RGB Hermite 보간
```
on_hermite_color_rgb(c1, c2, t)
```

- 내부적으로 $y(t) = -2t^3 + 3t^2$ 형태의 Hermite 곡선을 사용
- y(t)를 보간 인자로 사용하여 (r, g, b) 각각을 on_lerp_i32로 보간

## 🧩 animated_value 함수
```rust
pub fn animated_value<F>(start: f64, end: f64, t: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let eased = f(t);
    start.lerp(&end, &eased)
}
```

- f(t)는 easing 함수
- start와 end 사이를 eased 값에 따라 보간

---

## 소스 코드
```rust
#[inline]
pub fn on_lerp_f64(t: f64, x: f64, y: f64) -> f64 {
    // If x == y, return x as-is (even if t is NaN)

    if x == y && t == x {
        return x;
    }
    let mut z = (1.0 - t) * x + t * y;

    // If x and y are not NaN and t is within [0, 1], clamp t to that range
    if x < y {
        if z < x && t >= 0.0 {
            z = x;
        } else if z > y && t <= 1.0 {
            z = y;
        }
    } else if x > y {
        if z < y && t >= 0.0 {
            z = y;
        } else if z > x && t <= 1.0 {
            z = x;
        }
    }
    z
}
```
```rust
#[inline]
pub fn on_lerp_i32(t: f64, a: i32, b: i32) -> i32 {
    let z = on_lerp_f64(t, a as f64, b as f64);
    z.round() as i32
}
```
```rust
#[inline]
pub fn on_factorial_u64(n: u32) -> u64 {
    // 반복형(스택 안전)
    (1..=n as u64).product::<u64>().max(1)
}
```
```rust
/// 2D Hermite: p0, p1 (points), t0, t1 (tangents/velocities), parameter t ∈ [0, 1]
pub fn on_hermite_spline_2d(
    p0: Point2,
    t0: Vector2,
    p1: Point2,
    t1: Vector2,
    t: f64,
) -> Point2 {
    let t2 = t * t;
    let t3 = t2 * t;

    // Hermite basis
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    let x = h00 * p0.x + h10 * t0.x + h01 * p1.x + h11 * t1.x;
    let y = h00 * p0.y + h10 * t0.y + h01 * p1.y + h11 * t1.y;

    Point2 { x, y }
}
```
```rust
/// RGB interpolation using Hermite easing (smooth stepfamily).
/// Internally uses a 2D Hermite curve from (0, 0) to (1, 1), with the y-value as the interpolation factor.
pub fn on_hermite_color_rgb(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    // p0=(0,0), p1=(1,1), t0=(1,0), t1=(1,0) → y(t)= -2t^3 + 3t^2
    let p0 = Point2 { x: 0.0, y: 0.0 };
    let p1 = Point2 { x: 1.0, y: 1.0 };
    let v0 = Vector2 { x: 1.0, y: 0.0 };
    let v1 = Vector2 { x: 1.0, y: 0.0 };
    let h = on_hermite_spline_2d(p0, v0, p1, v1, t).y.clamp(0.0, 1.0);

    let r = on_lerp_i32(h, c1.0 as i32, c2.0 as i32).clamp(0, 255) as u8;
    let g = on_lerp_i32(h, c1.1 as i32, c2.1 as i32).clamp(0, 255) as u8;
    let b = on_lerp_i32(h, c1.2 as i32, c2.2 as i32).clamp(0, 255) as u8;
    (r, g, b)
}
```
```rust
pub struct InterpolatorEase;
impl InterpolatorEase {
    /// Smooth step easing (Hermite)
    pub fn smooth_step(t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
```
```rust
    /// Quadratic easing in
    pub fn quadratic_in(t: f64) -> f64 {
        t * t
    }
```
```rust
    /// Quadratic easing out
    pub fn quadratic_out(t: f64) -> f64 {
        t * (2.0 - t)
    }
```
```rust
    /// Quadratic easing in-out
    pub fn quadratic_in_out(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
```
```rust
    /// Clamp t to [0.0, 1.0]
    fn clamp01(t: f64) -> f64 {
        t.max(0.0).min(1.0)
    }
```
```rust
    /// Cubic easing in: slow start
    pub fn cubic_in(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        t * t * t
    }
```
```rust
    /// Cubic easing out: fast start, slow end
    pub fn cubic_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        let p = t - 1.0;
        p * p * p + 1.0
    }
```
```rust
    /// Cubic easing in-out: smooth both ends
    pub fn cubic_in_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let p = 2.0 * t - 2.0;
            0.5 * p * p * p + 1.0
        }
    }
```
```rust
    /// Exponential easing in: very slow start
    pub fn exponential_in(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        if t == 0.0 {
            0.0
        } else {
            2f64.powf(10.0 * (t - 1.0))
        }
    }
```
```rust
    /// Exponential easing out: very fast start
    pub fn exponential_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2f64.powf(-10.0 * t)
        }
    }
```
```rust
    /// Exponential easing in-out
    pub fn exponential_in_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            0.5 * 2f64.powf(20.0 * t - 10.0)
        } else {
            1.0 - 0.5 * 2f64.powf(-20.0 * t + 10.0)
        }
    }
```
```rust
    /// Bounce easing out: bouncy end
    pub fn bounce_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }
```
```rust
    /// Elastic easing out: springy end
    pub fn elastic_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let p = 0.3;
            2f64.powf(-10.0 * t) * (std::f64::consts::PI * 2.0 * t / p).sin() + 1.0
        }
    }
```
```rust
    /// Back easing out: overshoot then settle
    pub fn back_out(t: f64) -> f64 {
        let t = InterpolatorEase::clamp01(t);
        let s = 1.70158;
        let p = t - 1.0;
        p * p * ((s + 1.0) * p + s) + 1.0
    }
```
```rust
    pub fn animated_value<F>(start: f64, end: f64, t: f64, f: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let eased = f(t); // Replace with desired easing function
        start.lerp(&end, &eased)
    }
}
```
---


