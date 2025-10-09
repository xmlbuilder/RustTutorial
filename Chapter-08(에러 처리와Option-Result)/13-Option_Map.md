# Option map

## 🧠 Option<T>::map()의 의미
- Option<T>는 Some(value) 또는 None을 가질 수 있는 열거형(enum)
- map()은 Some(value)일 때만 클로저를 실행해서 값을 변환하고, None이면 그대로 None을 반환
### 예시
```rust
let maybe_num = Some(10);
let squared = maybe_num.map(|x| x * x); // Some(100)
```

- maybe_num이 Some(10)이므로 x * x가 실행됨
- 결과는 Some(100)

## ✅ 컬렉션이 아닌 타입에서도 map()이 되는 이유
| 타입         | map() 지원 여부 | 동작 조건 및 설명                              |
|--------------|------------------|-----------------------------------------------|
| Option<T>    | ✅ 지원           | `Some(value)`일 때만 클로저 실행, `None`은 그대로 |
| Result<T,E>  | ✅ 지원           | `Ok(value)`일 때만 클로저 실행, `Err(e)`는 그대로 |
| Iterator     | ✅ 지원           | 각 요소에 대해 클로저 실행, 새로운 반복자 반환     |
| Future       | ✅ 지원           | 비동기 결과(`Poll::Ready`)에 클로저 적용 가능     |

Rust의 map()은 “값이 있으면 변환하고, 없으면 그대로 둔다”는 함수형 철학을 따르고 있음.

---

## 실전 코드
```rust
#[inline]
pub fn unitize(v: Vector3D) -> Vector3D {
    let len = (v.x*v.x + v.y*v.y + v.z*v.z).sqrt();
    if len <= ON_ZERO_TOLERANCE { Vector3D::zero() } else { Vector3D { x: v.x/len, y: v.y/len, z: v.z/len } }
}

pub fn segment_direction(&self, i: usize) -> Option<Vector3D> {
    self.segment(i).map(|(a,b)| Vector3D { x: b.x - a.x, y: b.y - a.y, z: b.z - a.z })
}

pub fn segment_tangent(&self, i: usize) -> Option<Vector3D> { self.segment_direction(i).map(unitize) }
```

## 🧩 전체 구조 요약
```
segment_direction(i) → (a, b) → b - a → Vector3D  
segment_tangent(i) → unitize(segment_direction(i))
```


### 1️⃣ segment_direction(i) — 선분의 방향 벡터 구하기
```rust
pub fn segment_direction(&self, i: usize) -> Option<Vector3D> {
    self.segment(i).map(|(a,b)| Vector3D {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z
    })
}
```


- self.segment(i)는 i번째 선분을 (a, b) 형태로 반환한다고 가정
- a, b는 시작점과 끝점
- b - a를 계산해서 방향 벡터를 구함
- 결과는 Vector3D 타입으로 반환

### 2️⃣ segment_tangent(i) — 방향 벡터를 단위 벡터로 정규화
```rust
pub fn segment_tangent(&self, i: usize) -> Option<Vector3D> {
    self.segment_direction(i).map(unitize)
}
```

- segment_direction(i)로 방향 벡터를 구함
- unitize() 함수를 통해 길이가 1인 단위 벡터로 변환
- 즉, 방향만 유지하고 크기를 1로 맞춤

### 3️⃣ unitize(v) — 벡터 정규화 함수
```rust
pub fn unitize(v: Vector3D) -> Vector3D {
    let len = (v.x*v.x + v.y*v.y + v.z*v.z).sqrt();
    if len <= ON_ZERO_TOLERANCE {
        Vector3D::zero()
    } else {
        Vector3D {
            x: v.x / len,
            y: v.y / len,
            z: v.z / len
        }
    }
}

```
- 벡터의 길이 len을 계산: √(x² + y² + z²)
- 길이가 너무 작으면(0에 가까우면) → Vector3D::zero() 반환
- 그렇지 않으면 각 성분을 len으로 나눠서 단위 벡터 생성

### 🔄 실행 흐름 예시
```rust
let tangent = segment_tangent(3);
```
- segment(3) → (a, b) 반환
- b - a → 방향 벡터 계산
- unitize() → 길이 1로 정규화
- 최종 결과: Option<Vector3D> 타입의 단위 벡터

---


