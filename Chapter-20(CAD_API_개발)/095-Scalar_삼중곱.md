#  Scalar Triple

이 함수의 목적은 **세 벡터 a,b,c의 스칼라 삼중곱(scalar triple product)** 을 계산하는 것입니다.

## 📐 스칼라 삼중곱이란?
- 정의:

$$
[a,b,c]=a\cdot (b\times c)
$$

- 즉, 벡터 b와 c의 **외적(cross product)** 을 구한 뒤, 그 결과 벡터와 a의 **내적(dot product)** 을 계산합니다.
- 코드에서 하는 일:
```rust
let cx = b.y * c.z - b.z * c.y; // cross product x
let cy = b.z * c.x - b.x * c.z; // cross product y
let cz = b.x * c.y - b.y * c.x; // cross product z
a.x * cx + a.y * cy + a.z * cz  // dot product with a
```

## 🎯 함수의 목적
- 부피 계산
  -세 벡터가 만드는 평행육면체(parallelepiped)의 부피를 나타냅니다.

```math
|a\cdot (b\times c)| = 부피.
```

- 선형 독립성 판별
  - 결과가 0이면 세 벡터가 같은 평면에 놓여 있다는 뜻 → 선형 종속.
- 방향성 확인
  - 부호가 양수/음수인지에 따라 오른손 좌표계/왼손 좌표계 여부를 판별할 수 있습니다.
    
## 🧪 예시
```rust
let a = Vector3D { x: 1.0, y: 0.0, z: 0.0 };
let b = Vector3D { x: 0.0, y: 1.0, z: 0.0 };
let c = Vector3D { x: 0.0, y: 0.0, z: 1.0 };

let result = triple(&a, &b, &c);
// result = 1.0 → 단위 정육면체의 부피
```

## ✅ 정리하면:
- 이 함수는 세 벡터가 만드는 공간적 관계(부피, 독립성, 방향성)를 판별하는 핵심 도구.

---
