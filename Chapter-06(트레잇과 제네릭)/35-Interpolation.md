# interpolation
interpolation crate는 Rust에서 `선형 보간(linear interpolation)` 을 간단하게 구현할 수 있도록 도와주는 경량 라이브러리입니다.  
주로 애니메이션, 그래픽, 물리 시뮬레이션 등에서 값의 중간 지점을 계산할 때 사용됩니다.

## 🧩 주요 기능
- 선형 보간 (Linear Interpolation): lerp 함수로 두 값 사이의 중간 값을 계산
- Trait 기반 확장성: Lerp trait을 구현하여 다양한 타입에 보간 기능 추가 가능
- 간단한 API: start.lerp(end, t) 형태로 직관적인 사용

## ⚙️ 설치 방법
Cargo.toml에 다음을 추가:
```
[dependencies]
interpolation = "0.2"
```

## 🧪 기본 사용 예제
```rust
use interpolation::Lerp;

fn main() {
    let start = 10.0;
    let end = 20.0;
    let t = 0.25;
    let result = start.lerp(end, t);
    println!("Interpolated value: {}", result); // 12.5
}
```
- t는 0.0 ~ 1.0 사이의 값으로, start에서 end까지의 비율을 나타냅니다.
- t = 0.0이면 start, t = 1.0이면 end, 중간값은 그 사이의 위치입니다.

## 🧮 커스텀 타입에 적용하기
Lerp trait을 직접 구현하면 사용자 정의 타입에도 보간을 적용할 수 있습니다:

```rust
use interpolation::Lerp;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl Lerp for Point {
    type Scalar = Real;
    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        Self {
            x: self.x.lerp(&other.x, scalar),
            y: self.y.lerp(&other.y, scalar),
            z: self.z.lerp(&other.z, scalar),
        }
    }
}
```

### 📦 사용 예시
```rust
fn main() {
    let a = Point { x: 0.0, y: 0.0, z: 0.0 };
    let b = Point { x: 10.0, y: 20.0, z: 30.0 };
    let t = 0.5;

    let mid = a.lerp(&b, &t);
    println!("{:?}", mid); // Point { x: 5.0, y: 10.0, z: 15.0 }
}
```


## 🎨 애니메이션에 활용 예시
```rust
fn animated_value<F>(start: f64, end: f64, t: f64, easing: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let eased_t = easing(t);
    start.lerp(end, eased_t)
}
```

- easing 함수로 시간에 따른 속도 조절 가능 (예: ease-in, ease-out)
- interpolation crate은 lerp만 제공하므로 easing은 직접 구현해야 합니다

## 📌 주의사항
- lerp는 f64, f32 등 기본 타입에만 기본적으로 구현되어 있습니다
- t 값이 범위를 벗어나면 extrapolation(외삽)이 발생할 수 있습니다

---
