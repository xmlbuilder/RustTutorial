# Frame Transformation

## 🎯 문제 정의: 두 좌표계(Frame1 → Frame2) 변환
- 두 개의 직교좌표계가 있다고 하자.
- Frame1
    - 원점: $O_1$
    - 단위축: $X_1,Y_1,Z_1$
- Frame2
    - 원점: $O_2$
    - 단위축: $X_2,Y_2,Z_2$
- 우리가 만들고 싶은 변환 행렬 A 는:
- Frame1의 모든 점과 벡터를 Frame2 기준으로 표현하도록 바꾸는 변환

- 즉,
```math
A\cdot [O_1\  X_1\  Y_1\  Z_1]=[O_2\  X_2\  Y_2\  Z_2]
```


## 🧠 핵심 아이디어: 좌표계 변환은 “기준을 바꾸는 것”
- Frame1에서 Frame2로 가는 변환은 다음 4단계로 분해.

### 1️⃣ Frame1의 원점을 원점으로 이동 (T(-O1))
- Frame1의 원점 $O_1$을 (0,0,0)으로 옮겨야 회전이 제대로 적용될 수 있음.
```math
T_1=T(-O_1)
```
### 2️⃣ Frame1의 축을 세계좌표계 기준으로 정렬 (R1ᵀ)
- Frame1의 축은:
```math
R_1=[X_1\  Y_1\  Z_1]
```
- 이 행렬은 Frame1 → World 변환이므로,  
    World → Frame1 변환은 전치:
```math
R_1^{-1}=R_1^T
```

### 3️⃣ World 좌표계를 Frame2 축으로 회전 (R2)
- Frame2의 축은:
```math
R_2=[X_2\  Y_2\  Z_2]
```
- 이 행렬은 World → Frame2 변환.

### 4️⃣ Frame2의 원점으로 이동 (T(O2))
- 마지막으로 Frame2의 원점으로 이동:
```math
T_2=T(O_2)
```

## 🧩 최종 변환식
- 이 네 단계를 순서대로 적용하면:
```math
A=T_2\cdot R_2\cdot R_1^T\cdot T_1
```

## 🔥 왜 이 식이 맞는가?
- Frame1의 점 P를 Frame2로 보내려면:
    - Frame1 기준 좌표 → World 기준 좌표
    - World 기준 좌표 → Frame2 기준 좌표
- 이 두 단계를 합친 것이 바로 위의 식.

- 이게 바로 from_orthonormal_frames 본질.

## 🎁 요약
| Step                 | Matrix  |
|----------------------|---------|
| 1. Translate O1 → 0  | T(-O1)  |
| 2. Align Frame1 axes | R1^T    |
| 3. Align to Frame2   | R2      |
| 4. Translate 0 → O2  | T(O2)   |

- **Final Transform:** 
```math 
A = T(O2) · R2 · R1^T · T(-O1)
```

---

## 🧩 1) 기본 원리:
- 변환은 항상 “행렬 × 벡터” 형태로 적용된다
- 즉,
```math
P'=M\cdot P
```
- 여기서
    - P = 원래 점 (동차좌표 4×1)
    - M = 4×4 변환 행렬
    - P' = 변환된 점
- Rust 코드에서도:
```rust
let (x, y, z, w) = Xform::mult_homogeneous(&rhs, self.x, self.y, self.z, 1.0);
```

- 이게 바로 M × P 연산.

## 🧩 2) from_orthonormal_frames 핵심도 동일하다
- 우리가 만든 최종 변환식:
```math
A=T_2\cdot R_2\cdot R_1^T\cdot T_1
```
- 이 행렬 A는 왼쪽에서 곱하는 변환이므로:
```math
P'=A\cdot P
```

## 🧩 3) 왜 “왼쪽 곱”인가?
- 너의 Xform 구조는 행렬이 왼쪽, 벡터가 오른쪽에 오는 방식으로 설계되어 있어.
- 즉,
```rust
let q = xf.multi_point_left(&p);
```

- 이 함수는 내부적으로:
```math
P'=M\cdot P
```
- 을 계산한다.
- 그래서 from_orthonormal_frames 에서 만든 A도 왼쪽 곱으로 적용해야 한다.

## 🧩 4) 오른쪽 곱 방식과의 차이
- 일부 그래픽스 엔진(OpenGL, DirectX 등)은  
    행렬을 오른쪽에 두고 벡터를 왼쪽에 두는 방식도 쓴다:
```math
P'=P\cdot M
```
- 하지만 너의 Xform은 전형적인 CAD/CAE 스타일로 
    행렬이 왼쪽, 벡터가 오른쪽에 오는 방식.

## 🧩 5) from_orthonormal_frames 에서의 적용 예시
- Frame1의 점 P를 Frame2로 보내려면:
```rust
let xf = Xform::from_orthonormal_frames(o1, x1, y1, z1, o2, x2, y2, z2);
let p2 = xf.multi_point_left(&p1);
```
---
## 소스 코드
```rust
impl Xform {
    /// Transform that maps orthonormal frame (O1, X1, Y1, Z1)
    /// into another orthonormal frame (O2, X2, Y2, Z2).
    pub fn from_orthonormal_frames(
        o1: Point3D,
        x1: Vector3D,
        y1: Vector3D,
        z1: Vector3D,
        o2: Point3D,
        x2: Vector3D,
        y2: Vector3D,
        z2: Vector3D,
    ) -> Xform {
        // R1: frame1 rotation matrix
        let r1 = Xform {
            m: [
                [x1.x, y1.x, z1.x, 0.0],
                [x1.y, y1.y, z1.y, 0.0],
                [x1.z, y1.z, z1.z, 0.0],
                [0.0,  0.0,  0.0,  1.0],
            ],
        };

        // R2: frame2 rotation matrix
        let r2 = Xform {
            m: [
                [x2.x, y2.x, z2.x, 0.0],
                [x2.y, y2.y, z2.y, 0.0],
                [x2.z, y2.z, z2.z, 0.0],
                [0.0,  0.0,  0.0,  1.0],
            ],
        };

        // T1 = translation(-O1)
        let t1 = Xform::translation(-o1.x, -o1.y, -o1.z);

        // T2 = translation(O2)
        let t2 = Xform::translation(o2.x, o2.y, o2.z);

        // A = T2 * R2 * R1ᵀ * T1
        let r1_t = r1.transpose();
        t2 * r2 * r1_t * t1
    }
}
```
---

