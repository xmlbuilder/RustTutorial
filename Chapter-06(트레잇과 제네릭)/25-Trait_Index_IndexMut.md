
# 🧠 Index / IndexMut 트레이트란?
Rust에서 a[i] 문법은 내부적으로 Index 트레이트를 호출합니다:
- Index<Idx>: 읽기 전용 인덱싱 (a[i])
- IndexMut<Idx>: 쓰기 가능한 인덱싱 (a[i] = value)
이 트레이트를 구현하면 구조체를 배열처럼 다룰 수 있음.


## 🔧 트레잇 정의
```rust
pub trait Index<Idx> {
    type Output;

    fn index(&self, index: Idx) -> &Self::Output;
}
```
```rust
pub trait IndexMut<Idx>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```
- Idx: 인덱스 타입 (예: usize, String, 사용자 정의 타입 등)
- Output: 반환되는 참조 타입 (예: T, [T], str 등)

## ✅ 기본 예제: RGB 색상 구조체
```rust
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// 읽기 전용 인덱싱
impl Index<usize> for Color {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Color index out of bounds"),
        }
    }
}

// 쓰기 가능한 인덱싱
impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Color index out of bounds"),
        }
    }
}
```


## ✅ 사용 예시
```rust
fn main() {
    let mut c = Color { r: 255, g: 128, b: 64 };

    println!("Red: {}", c[0]); // 255
    println!("Green: {}", c[1]); // 128
    println!("Blue: {}", c[2]); // 64

    c[1] = 200; // Green 값 수정
    println!("Updated Green: {}", c[1]); // 200
}
```


## ✅ Point3D에 적용하면?
```rust
impl Index<usize> for Point3D {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Point3D index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Point3D {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Point3D index out of bounds"),
        }
    }
}
```

이렇게 하면 point[0], point[1], point[2]로 x, y, z를 자유롭게 다룰 수 있어요.

## ✨ 보너스: 안전한 인덱싱 헬퍼
```rust
pub fn get_axis_point(p: &Point3D, axis: usize) -> Option<f64> {
    match axis {
        0 => Some(p.x),
        1 => Some(p.y),
        2 => Some(p.z),
        _ => None,
    }
}

```

## 🧪 사용 예제: 안전한 축 접근
```rust
fn main() {
    let p = Point3D::new(3.0, 4.0, 5.0);

    for i in 0..4 {
        match get_axis_point(&p, i) {
            Some(val) => println!("Axis {}: {}", i, val),
            None => println!("Axis {} is out of bounds!", i),
        }
    }
}
```

## 🖨️ 출력 결과:
```
Axis 0: 3.0
Axis 1: 4.0
Axis 2: 5.0
Axis 3 is out of bounds!
```


## ✅ 실전 활용 예: BoundingBox 축별 거리 계산
```rust
fn diagonal_length(bbox: &BoundingBox) -> f64 {
    let mut sum = 0.0;
    for i in 0..3 {
        if let (Some(max), Some(min)) = (get_axis_point(&bbox.max, i), get_axis_point(&bbox.min, i)) {
            let d = max - min;
            sum += d * d;
        }
    }
    sum.sqrt()
}
```



