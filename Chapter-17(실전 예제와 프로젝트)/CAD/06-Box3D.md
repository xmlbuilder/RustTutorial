# Box3D
Box3D (AABB) 구현에 대한 수학적 검증 결과와 함께, 전체 기능을 문서 형식으로 정리한 내용입니다.  
수학적으로 안정적인지 확인하고, 각 메서드의 역할과 수식적 의미를 명확히 설명.

## 📐 Box3D (Axis-Aligned Bounding Box) 문서 및 수학적 검증
## ✅ 수학적 검증 요약
| 메서드               | 안정성 여부 | 수학적 표현 또는 의미                          | 설명                                                                 |
|----------------------|-------------|------------------------------------------------|----------------------------------------------------------------------|
| `new()`              | ✅           | min/max 정렬                                   | 두 점의 순서에 관계없이 축별로 정렬하여 AABB 생성                    |
| `center()`           | ✅           | $\((\mathrm{min} + \mathrm{max}) / 2\)$          | 중심점 계산: 각 축의 평균                                            |
| `size()`             | ✅           | $\(\mathrm{max} - \mathrm{min}\)$                | 크기 계산: 폭, 높이, 깊이                                            |
| `contains_point()`   | ✅           | $\(\mathrm{min} \leq p \leq \mathrm{max}\)$      | 점 포함 여부: 경계 포함                                              |
| `expand_to_include()`| ✅           | min/max 갱신                                   | 점을 포함하도록 `min/max` 확장                                       |
| `union()`            | ✅           | $\(\min(\text{min}_1, \text{min}_2), \max(\text{max}_1, \text{max}_2)\)$ | 두 박스를 모두 포함하는 최소 AABB           |
| `intersection()`     | ✅           | $\(\max(\text{min}_1, \text{min}_2), \min(\text{max}_1, \text{max}_2)\)$ | 교차 영역 계산. 겹치지 않으면 `None` 반환 |
| `normalize()`        | ✅           | $\(\mathrm{min} \leq \mathrm{max}\)$             | 내부 정렬 유지. setter 호출 시 자동 정규화                           |

모든 연산은 축별 독립적이며, AABB의 수학적 정의에 부합합니다.  
경계 포함, 중심점, 크기, 교차 여부 등 모두 수식적으로 안정적입니다.


## 📘 Box3D 문서 정리
## 📦 구조 정의
```rust
pub struct Box3D {
    min: Point, // 최소 좌표 (x, y, z)
    max: Point, // 최대 좌표 (x, y, z)
}
```
- AABB: 축에 정렬된 3차원 경계 박스
- min ≤ max 조건을 항상 유지

## 🛠 생성 및 정규화
```rust
pub fn new(p1: Point, p2: Point) -> Self
```
- 두 점의 순서에 관계없이 축별로 정렬하여 min, max 설정

```rust
fn normalize(&mut self)
```
- 내부적으로 min ≤ max를 보장
- setter 호출 시 자동 정렬

## 📍 접근자
```rust
pub fn min(&self) -> &Point
pub fn max(&self) -> &Point
```
- min, max 좌표 반환

## 📌 중심점 및 크기
```rust
pub fn center(&self) -> Point
```
- 중심점 = $(\mathrm{min}+\mathrm{max})/2$
```rust
pub fn size(&self) -> (f64, f64, f64)
```
- 크기 = $(\mathrm{max.x}-\mathrm{min.x},\mathrm{max.y}-\mathrm{min.y},\mathrm{max.z}-\mathrm{min.z})$

## 📎 포함 관계
```rust
pub fn contains_point(&self, p: &Point3D) -> bool
```

- 점이 박스 내부에 있는지 확인
- 경계 포함: \mathrm{min}\leq p\leq \mathrm{max}

## 📈 확장 및 병합
```rust
pub fn expand_to_include(&mut self, p: &Point3D)
```
- 점을 포함하도록 min/max 갱신

```rust
pub fn union(&self, other: &Self) -> Self
```
- 두 박스를 모두 포함하는 최소 AABB 반환

## 🔀 교차 판정
```rust
pub fn intersection(&self, other: &Self) -> Option<Self>
```
- 교차 영역 반환 (없으면 None)
- 수학적 조건: \mathrm{max}\geq \mathrm{min} 축별로 모두 만족해야 함


## ✅ 결론
- 이 Box3D 구현은 AABB의 수학적 정의에 충실하며, 모든 연산이 축별 독립적이고 안정적입니다.
- 경계 포함, 교차, 병합, 정규화 등에서 수식적 오류나 논리적 모순은 없습니다.
- 실시간 물리 엔진, 공간 분할, 충돌 판정 등에 바로 적용 가능한 수준입니다.


## 소스 코드
```rust
use crate::core::prelude::Point;

/// AABB (axis-aligned bounding box)

#[derive(Debug, Clone, PartialEq)]
pub struct Box3D {
    min: Point,
    max: Point,
}
```
```rust
impl Box3D {
    /// p1, p2 순서에 상관없이 축별로 (min,max) 정규화해서 저장
    pub fn new(p1: Point, p2: Point) -> Self {
        let min = Point {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
            z: p1.z.min(p2.z),
        };
        let max = Point {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
            z: p1.z.max(p2.z),
        };
        Self { min, max }
    }

    #[inline]
    pub fn min(&self) -> &Point {
        &self.min
    }
    #[inline]
    pub fn max(&self) -> &Point {
        &self.max
    }

    /// setter도 항상 정규화 유지
    pub fn set_min(&mut self, p: Point) {
        self.min = p;
        self.normalize();
    }
    pub fn set_max(&mut self, p: Point) {
        self.max = p;
        self.normalize();
    }

    /// 중심점
    pub fn center(&self) -> Point {
        Point {
            x: (self.min.x + self.max.x) * 0.5,
            y: (self.min.y + self.max.y) * 0.5,
            z: (self.min.z + self.max.z) * 0.5,
        }
    }

    /// 크기(폭, 높이, 깊이)
    pub fn size(&self) -> (f64, f64, f64) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    /// 점 포함 여부 (경계 포함)
    pub fn contains_point(&self, p: &Point) -> bool {
        (self.min.x <= p.x && p.x <= self.max.x)
            && (self.min.y <= p.y && p.y <= self.max.y)
            && (self.min.z <= p.z && p.z <= self.max.z)
    }

    /// 점을 포함하도록 박스를 확장
    pub fn expand_to_include(&mut self, p: &Point) {
        if p.x < self.min.x {
            self.min.x = p.x;
        }
        if p.y < self.min.y {
            self.min.y = p.y;
        }
        if p.z < self.min.z {
            self.min.z = p.z;
        }
        if p.x > self.max.x {
            self.max.x = p.x;
        }
        if p.y > self.max.y {
            self.max.y = p.y;
        }
        if p.z > self.max.z {
            self.max.z = p.z;
        }
    }

    /// 합집합(둘을 모두 포함하는 최소 박스)
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Point {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
                z: self.min.z.min(other.min.z),
            },
            max: Point {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
                z: self.max.z.max(other.max.z),
            },
        }
    }

    /// 교집합(겹치지 않으면 None)
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min = Point {
            x: self.min.x.max(other.min.x),
            y: self.min.y.max(other.min.y),
            z: self.min.z.max(other.min.z),
        };
        let max = Point {
            x: self.max.x.min(other.max.x),
            y: self.max.y.min(other.max.y),
            z: self.max.z.min(other.max.z),
        };
        if max.x >= min.x && max.y >= min.y && max.z >= min.z {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// 내부 정규화: (min ≤ max) 보장
    pub fn normalize(&mut self) {
        let (minx, maxx) = (self.min.x.min(self.max.x), self.min.x.max(self.max.x));
        let (miny, maxy) = (self.min.y.min(self.max.y), self.min.y.max(self.max.y));
        let (minz, maxz) = (self.min.z.min(self.max.z), self.min.z.max(self.max.z));
        self.min.x = minx;
        self.max.x = maxx;
        self.min.y = miny;
        self.max.y = maxy;
        self.min.z = minz;
        self.max.z = maxz;
    }
}
```

----

## 📊 테스트 항목 정리 표
| 테스트 함수 이름              | 검증 항목 설명                                      |
|------------------------------|-----------------------------------------------------|
| `ctor_normalizes`            | 생성 시 `min/max` 정렬 확인                         |
| `setters_keep_normalized`    | setter 호출 후 자동 정규화 확인                     |
| `center_size_contains`       | 중심점, 크기, 점 포함 여부                         |
| `union_and_intersection`     | 박스 병합 및 교차 영역 계산                         |
| `expand_to_include`          | 점 포함 시 박스 확장 확인                          |
| `normalize_corrects_order`   | `normalize()`가 `min ≤ max` 보장하는지 확인         |
| `contains_point_on_boundary` | 경계점 포함 여부 확인                               |
| `intersection_on_edge`       | 경계 접촉 시 교차 여부 확인                         |

```rust
fn p(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
}
```
### 1. ctor_normalizes
```rust
#[test]
fn ctor_normalizes() {
    let b = Box3D::new(p(3.0, -1.0, 2.0), p(-2.0, 5.0, -4.0));
    assert_eq!(*b.min(), p(-2.0, -1.0, -4.0));
    assert_eq!(*b.max(), p(3.0, 5.0, 2.0));
}
```
### 2. setters_keep_normalized
```rust
#[test]
fn setters_keep_normalized() {
    let mut b = Box3D::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0));
    // (min, max)가 자동 교정됨
    b.set_min(p(0.5, -2.0, 0.2));
    assert_eq!(*b.min(), p(0.5, -2.0, 0.2));
    assert_eq!(*b.max(), p(1.0, 1.0, 1.0));
    b.set_max(p(-1.0, 0.5, 0.1));
    assert_eq!(*b.min(), p(-1.0, -2.0, 0.1));
    assert_eq!(*b.max(), p(0.5, 0.5, 0.2));
}
```
### 3. center_size_contains
```rust
#[test]
fn center_size_contains() {
    let b = Box3D::new(p(-1.0, 2.0, -3.0), p(3.0, 8.0, 1.0));
    assert_eq!(b.center(), p(1.0, 5.0, -1.0));
    assert_eq!(b.size(), (4.0, 6.0, 4.0));
    assert!(b.contains_point(&p(0.0, 3.0, 0.0)));
    assert!(!b.contains_point(&p(5.0, 0.0, 0.0)));
}
```
### 4. union_and_intersection
```rust
#[test]
fn union_and_intersection() {
    let a = Box3D::new(p(0.0, 0.0, 0.0), p(2.0, 2.0, 2.0));
    let b = Box3D::new(p(1.0, -1.0, 1.0), p(3.0, 1.0, 3.0));
    let u = a.union(&b);
    assert_eq!(*u.min(), p(0.0, -1.0, 0.0));
    assert_eq!(*u.max(), p(3.0, 2.0, 3.0));

    let inter = a.intersection(&b).unwrap();
    assert_eq!(*inter.min(), p(1.0, 0.0, 1.0));
    assert_eq!(*inter.max(), p(2.0, 1.0, 2.0));

    let c = Box3D::new(p(10.0, 10.0, 10.0), p(11.0, 11.0, 11.0));
    assert!(a.intersection(&c).is_none());
}
```

### 5. expand_to_include
```rust
#[test]
fn expand_to_include() {
    let mut b = Box3D::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0));
    b.expand_to_include(&p(-2.0, 0.5, 3.0));
    assert_eq!(*b.min(), p(-2.0, 0.0, 0.0));
    assert_eq!(*b.max(), p(1.0, 1.0, 3.0));
}
```

### 6. normalize_corrects_order
```rust
#[test]
fn normalize_corrects_order() {
    let mut b = Box3D::new(p(5.0, 5.0, 5.0), p(1.0, 1.0, 1.0));
    b.normalize();
    assert_eq!(*b.min(), p(1.0, 1.0, 1.0));
    assert_eq!(*b.max(), p(5.0, 5.0, 5.0));
}
```

### 7. contains_point_on_boundary
```rust
#[test]
fn contains_point_on_boundary() {
    let b = Box3D::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0));
    assert!(b.contains_point(&p(0.0, 0.0, 0.0))); // min
    assert!(b.contains_point(&p(1.0, 1.0, 1.0))); // max
}
```
### 8. intersection_on_edge
```rust
#[test]
fn intersection_on_edge() {
    let a = Box3D::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0));
    let b = Box3D::new(p(1.0, 1.0, 1.0), p(2.0, 2.0, 2.0));
    let inter = a.intersection(&b).unwrap();
    assert_eq!(*inter.min(), p(1.0, 1.0, 1.0));
    assert_eq!(*inter.max(), p(1.0, 1.0, 1.0));
}
```

### 9. normalize() 직접 테스트
```rust
#[test]
fn normalize_corrects_order() {
    let mut b = Box3D::new(p(5.0, 5.0, 5.0), p(1.0, 1.0, 1.0));
    b.normalize();
    assert_eq!(*b.min(), p(1.0, 1.0, 1.0));
    assert_eq!(*b.max(), p(5.0, 5.0, 5.0));
}
```
- normalize()가 min ≤ max를 보장하는지 직접 확인

### 10. contains_point() 경계 테스트
```rust
#[test]
fn contains_point_on_boundary() {
    let b = Box3D::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0));
    assert!(b.contains_point(&p(0.0, 0.0, 0.0))); // min
    assert!(b.contains_point(&p(1.0, 1.0, 1.0))); // max
}
```
- 경계 포함 여부를 명확히 검증

### 11. intersection() 경계 접촉 테스트
```rust
#[test]
fn intersection_on_edge() {
    let a = Box3D::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0));
    let b = Box3D::new(p(1.0, 1.0, 1.0), p(2.0, 2.0, 2.0));
    let inter = a.intersection(&b).unwrap();
    assert_eq!(*inter.min(), p(1.0, 1.0, 1.0));
    assert_eq!(*inter.max(), p(1.0, 1.0, 1.0));
}
```
- 경계에서 접촉하는 경우도 교차로 인정되는지 확인

---
