## Homogenous Control Point 규약 문서

### 📘 NURBS Homogeneous Control Point 규약 (A안, Piegl & Tiller 표준)
- 프로젝트 전체에서 반드시 일관되게 적용해야 하는 규칙

### 0️⃣ 핵심 요약 (One‑liner)
- 모든 NURBS control point는 항상 homogeneous (X,Y,Z,W) = (xw, yw, z*w, w) 로 저장하고,
- 평가 시에는 4D 선형합을 한 뒤 마지막에만 /W 한다.

### 1️⃣ Control Point 저장 규약 (가장 중요)
- ✔ 내부 저장 형식 (Homogeneous 4D)
    - Point4D = (X, Y, Z, W) = (x*w, y*w, z*w, w)
    - X, Y, Z는 이미 weight가 곱해진 값
    - W는 weight 자체
    - Piegl & Tiller의 표준 방식과 동일
- ✔ 유클리드 좌표로 변환할 때
    - (x, y, z) = (X/W, Y/W, Z/W)


- ❌ 금지되는 방식
    - (x, y, z, w)를 그대로 저장하는 방식
    - cp.x * cp.w 같은 연산
    - **유클리드 좌표 + weight** 라는 개념
    - 평가 중간에 /W 하는 방식
    - 이런 방식은 A안과 B안이 섞여서 w², w가 사라짐 같은 오류를 만든다.

## 2️⃣ Curve / Surface 평가 규약
- ✔ 항상 동일한 4D 선형합
```rust
X += N * cp.X
Y += N * cp.Y
Z += N * cp.Z
W += N * cp.W
```
- rational / non-rational 분기 없음
- non-rational은 단지 W = 1인 특수 케이스일 뿐
- ✔ 유클리드 결과가 필요할 때만
    - (x, y, z) = (X/W, Y/W, Z/W)



## 3️⃣ 입력(테스트/생성기) 규약
- ✔ 반드시 헬퍼 사용
```rust
fn homogenous(x, y, z, w) -> Point4D {
    Point4D::new(x*w, y*w, z*w, w)
}
```

- 예: 원의 중간점
```rust
let w = FRAC_1_SQRT_2;
homogenous(r, r, 0.0, w);   // 유클리드 (r,r), weight = w
```


### 🎯 왜 Point4D::homogeneous() 가 필요한가?
- 현재 API:
```rust
Point4D::new(wx, wy, wz, w)
```

- 여기서 인자는 이미 weight가 곱해진 값.
- 즉:
    - wx = x * w
    - wy = y * w
    - wz = z * w
- 이걸 매번 기억하고 쓰는 건 사람이 실수하기 딱 좋은 구조.
- 예를 들어:
```rust
Point4D::new(r*w, r*w, 0.0, w);
```
- 이건 맞지만, 실수로 이렇게 쓰기 쉽다:
```rust
Point4D::new(r, r, 0.0, w);   // ❌ w를 안 곱함
```

- 또는:
```rust
Point4D::new(r*w*w, r*w*w, 0.0, w); // ❌ w를 두 번 곱함
```

- 이런 실수는 rational curve가 완전히 깨지는 원인이 된다.
- 그래서 API를 명확하게 분리해야 한다.

## ✅ 제안: 명확한 헬퍼 함수 추가
### 1) 유클리드 좌표 + weight → homogeneous 변환
```rust
impl Point4D {
    pub fn homogeneous(x: Real, y: Real, z: Real, w: Real) -> Self {
        Point4D {
            x: x * w,
            y: y * w,
            z: z * w,
            w,
        }
    }
}
```

- 사용법:
```rust
let p = Point4D::homogeneous(r, r, 0.0, w);
```

- 이제 절대 실수할 일이 없다.

### 2) 이미 homogeneous 값이 준비된 경우만 new() 사용
```rust
Point4D::new(wx, wy, wz, w)
```
- 이건 내부 계산에서만 사용하도록 하고,
- 외부 API에서는 절대 new()를 직접 쓰지 않도록 규약을 정하면 좋다.




## 4️⃣ Revolution / Tensor 규약
-  ✔ Tensor 내부 계산은 항상 유클리드 공간에서 수행
    - (기하 연산자이기 때문)
- 하지만:
    - 입력은 homogeneous (X,Y,Z,W)
    - 내부에서 /W 하여 유클리드로 변환
    - 계산 후 다시 homogeneous로 복원
- 즉:
- 입력:  (X, Y, Z, W)
    - Tensor 내부: (x, y, z) = (X/W, Y/W, Z/W)
- 출력: (world * W, W)


- Tensor는 기하 연산자,
- NURBS는 표현/보존 담당.

## 5️⃣ A/B 혼동이 생겼던 이유 (정리)
- A안과 B안은 수식이 비슷해서, **w를 언제 곱하느냐** 만 다르다.
- 둘을 섞으면:
    - w²
    - w가 사라짐
    - rational이 non-rational처럼 보임
    - numeric/analytic mismatch
- 같은 문제가 반드시 발생한다.
- 지금은 다음이 모두 A안으로 통일됨:
    - 내부 표현
    - 평가
    - Revolution
    - Piegl 알고리즘
    - 즉, 완전 일치 상태.

## 6️⃣ 앞으로의 약속 (규약 적용 방식)
- **A안 기준으로**
- **homogeneous 저장**
- **여기서는 /W 하면 안 되고**
- **입력은 homogeneous 넣고**
    - 그리고 A/B라는 말 자체를 거의 쓰지 않음.
    - 그냥 **homogeneous 방식** 하나만 사용.

## 7️⃣ 테스트 에러가 나는 경우 (이제는 두 가지뿐)
- 입력 생성 실수
- homogeneous를 안 썼거나
- 유클리드 좌표를 그대로 넣었거나
- w를 두 번 곱했거나
- 예전 코드에서 남아 있는 w 중복 연산


## 📌 최종 문서 요약
- 이 문서는 프로젝트의 NURBS 내부 표현 규약이며 Piegl & Tiller 표준과 완전히 일치한다.
    - Control point는 항상 homogeneous로 저장
    - 평가 시 4D 선형합 → 마지막에만 /W
    - 입력은 반드시 homogeneous 사용
    - Tensor는 유클리드에서 계산 후 homogeneous로 복원
    - rational/non-rational 분기 없음
    - A/B 혼동 금지
- 이 규약을 지키면:
    - w² 문제 없음
    - analytic/numeric mismatch 없음
    - rational curve/surface가 정확히 동작
    - Piegl 알고리즘과 완전 호환


---

