## 1. 함수의 목적 한 줄 요약
- Bezier surface로 표현된 **경계 방향 미분 필드(derivative surface)** 를,  
    실제로 사용할 경계 방향/위치에 맞게 회전(swap)·연장(extend strip)해서 재배치하는 함수.

- 즉, 이 함수는:
- 이미 만들어진 derivative Bezier surface를
- **어느 방향으로 만들었는지(cst)** 와
- **어느 경계에서 쓸 건지(usg)** 에 따라
- 적절히:
    - 그대로 쓰거나
    - u↔v를 바꾸거나
    - 한쪽으로 Bezier strip을 연장하거나
    - 둘 다 조합해서
    - 원하는 경계에 맞는 derivative field로 변환해준다.

## 2. 입력 파라미터 의미
```rust
pub fn on_adjust_derivative_field_bezier(
    sur_p: &BezierSurface,
    dir: SurfaceDir,   // 이 derivative surface가 "어느 방향"으로 만들어졌는지 (U or V)
    cst: SideFlag,     // constructed along: 어떤 경계를 기준으로 만들어졌는지
    usg: SideFlag,     // used along: 실제로 어디에 쓸 건지
) -> Result<BezierSurface, NurbsError>
```

- sur_p
    - BezierSurface 형태의 **derivative surface**
    - 보통: 어떤 패치의 한 경계에서, 그 경계를 따라 정의된 미분 필드
        - dir (SurfaceDir::UDir / VDir)
        - 이 derivative surface가 어느 파라미터 방향으로 strip 형태인지
    - 예: UDir이면 **u 방향으로 Bezier strip**
        - cst (SideFlag::Left/Right/Bottom/Top)
        - 이 derivative field가 어느 경계를 기준으로 만들어졌는지
        - 예: Left면 **원래는 왼쪽 경계를 따라 만든 derivative field**
- usg
    - 이 derivative field를 실제로 어느 경계에 적용해서 쓸 건지
    - 예: cst=Left, usg=Right → **왼쪽에서 만든 걸 오른쪽에서 쓰고 싶다**
- 이 함수는:
    - **어디에서 만든 걸(cst), 어디에서 쓸 건지(usg)** 에 따라
    - swap/extend를 조합해서 필요한 위치/방향으로 derivative field를 옮겨준다.


## 3. 내부에서 쓰는 핵심 연산 2개
### 3-1. swap_uv (on_swap_uv_bezier_surface_in_place)
- 수식적으로:
```math
S(u,v)\mapsto S'(u,v)=S(v,u)
```
- control net에서:
    - P[i][j] ↔ P[j][i]
- 기하적으로:
    - u, v 축을 바꿔서
    - **왼쪽/오른쪽** ↔ **아래/위** 를 서로 교환하는 효과
이걸로:
    - **Left/Right”에 있던 걸 “Bottom/Top**  쪽으로 돌려 쓸 수 있게 된다.

### 3-2. extend_strip_with_same_derivatives(dir, side, reverse_param)
- 이게 진짜 핵심.
- Bezier surface가 있다고 하자:
```math
S(u,v)=\sum _{i=0}^p\sum _{j=0}^qB_i^p(u)B_j^q(v)P_{ij}
```
- 여기서 dir이 UDir이고, side가 Start라면:
    - u=0 쪽 경계에서의 **값 + 미분(derivatives)** 를 유지하면서
    - u<0 방향으로 Bezier strip을 하나 더 “붙이는” 연산이다.
- 수식적으로는:
    - Bezier 곡선/패치의 끝에서의 미분을 유지하면서 연장하는 공식
    - 보통 Pascal triangle(이항계수)을 써서
    - 기존 control point들로부터
    - 새 control point들을 선형결합으로 만들어낸다.
- 핵심은:
- 경계에서의 값/미분(derivative field)을 그대로 유지하면서  
    바깥쪽으로 strip을 확장하는 Bezier extension.
- reverse_param=false이므로, 파라미터 방향은 뒤집지 않고 그대로.

## 4. 전체 알고리즘 흐름
- sdr 계산
```rust
let sdr = if dir == UDir { VDir } else { UDir };
```
- dir의 반대 방향
    - swap 후에 strip을 연장할 때 필요
    - Pascal triangle 준비
```rust
let pq = max(u_degree, v_degree);
let _bin = on_pascal_triangle_u128(pq);
```
- Bezier extension에서 쓰일 이항계수 테이블
- 여기서는 extend 함수 내부에서 사용
- sur_q = sur_p.clone()
- 원본은 건드리지 않고, 결과를 새 BezierSurface로 반환
- match cst { ... match usg { ... } }
- “어디에서 만들었고(cst), 어디에서 쓸 건지(usg)”에 따라
- 그대로 쓰거나
- swap하거나
- extend하거나
- 둘 다 한다.

## 5. 케이스별 의미 정리
- 여기서 진짜 중요한 건 “각 조합이 기하적으로 무슨 뜻인지”야.
### 5-1. cst = Left
- 이 derivative field는 원래 왼쪽 경계 기준으로 만들어진 것.
```rust
usg = Left
```
#### **왼쪽에서 만든 걸 왼쪽에서 쓴다**
- 아무 것도 안 함 (그대로 사용)
```rust
usg = Right
```
####  **왼쪽에서 만든 걸 오른쪽에서 쓰고 싶다**
```rust
extend_strip_with_same_derivatives(dir, Start)
```
- UDir strip이라면:
    - u=0 쪽에서의 derivative field를 유지한 채
    - u<0 방향으로 strip을 확장
- 이걸 오른쪽 경계에서 쓰면:
    - “왼쪽에서의 derivative field를 반대쪽으로 가져온 것”과 같은 효과
```rust
usg = Bottom
```
#### **왼쪽에서 만든 걸 아래쪽에서 쓰고 싶다**
```rust
swap_uv
```
- Left ↔ Bottom, Right ↔ Top으로 매핑
    - 방향을 돌려서 아래쪽 경계에 맞게 재배치
```rust
usg = Top
```
#### **왼쪽에서 만든 걸 위쪽에서 쓰고 싶다**
```rust
swap_uv
extend_strip_with_same_derivatives(sdr, Start)
```
- 먼저 swap해서 Left→Bottom으로 보낸 뒤
- sdr 방향(원래의 반대 방향)으로 strip을 Start 쪽으로 확장
- 결과적으로 “왼쪽에서 만든 걸 위쪽으로 가져와서, 위쪽 바깥으로 연장한 derivative field”가 된다.

### 5-2. cst = Right
- 원래 오른쪽 경계 기준으로 만든 derivative field.
- 패턴은 Left의 좌우 반전 버전.
    - usg = Left → extend(dir, End)
    - usg = Right → 그대로
    - usg = Bottom → swap + extend(sdr, End)
    - usg = Top → swap

### 5-3. cst = Bottom
- 원래 아래쪽 경계 기준.
    - usg = Left → swap
    - usg = Right → swap + extend(sdr, Start)
    - usg = Bottom → 그대로
    - usg = Top → extend(dir, Start)

### 5-4. cst = Top
- 원래 위쪽 경계 기준.
    - usg = Left → swap + extend(sdr, End)
    - usg = Right → swap
    - usg = Bottom → extend(dir, End)
    - usg = Top → 그대로

## 6. 이 함수의 용도 (실전 맥락)
- 이 함수는 보통 이런 파이프라인에서 등장:
    - 어떤 base surface가 있고
    - 그 경계에서 cross-boundary derivative field를 만들었다고 치자
    - 예: blend surface, fillet, transition surface 만들기 전 단계
- 그런데:
    - 그 derivative field를
    - 다른 경계에서 쓰고 싶거나
    - 반대쪽에서 쓰고 싶거나
    - 위/아래/좌/우를 바꿔서 쓰고 싶을 수 있다.
- 그때:
- 이미 만들어진 derivative Bezier surface를  
    **어디에서 만들었고(cst), 어디에서 쓸 건지(usg)** 에 맞게
    swap + strip extension으로 재배치하는 도구가 바로 이 함수다.

- 즉:
    - 새로운 derivative field를 매번 다시 계산하는 대신
    - 한 번 만든 걸 이 함수로 회전/연장해서 재사용하는 구조.

## 7. 한 줄로 정리하면
- on_adjust_derivative_field_bezier는  
    어느 경계에서 만든 Bezier derivative surface를,  
    다른 경계/방향에서 쓰기 위해 swap_uv와 Bezier strip extension을 조합해서  
    올바른 위치와 방향으로 옮겨주는 스위치 함수.


---


## 1️⃣ 기본 상황: Bezier strip 형태의 “미분장(surface)”
- dir = UDir 인 경우
- u 방향이 Bezier
- v 방향으로 여러 개의 column curve가 있음

- 각 column은 “한 개의 Bezier curve(동차 미분장)”
```
v ↑
  |
  |   |   |   |   |      ← 각 세로 줄 = Bezier curve (u-방향)
  |   |   |   |   |
  |   |   |   |   |
  +--------------------→ u
      0           1
```

- 이 surface는 보통 이런 의미:

- **이 미분장은 u = 0 (LEFT) 경계를 기준으로 만들어졌다**

- 즉,
- 각 column curve의 t = 0 쪽 미분값이 중요함
- 이게 단면 설계 / sweep / loft에서 쓰일 “경계 미분장”

## 2️⃣ 문제 상황: 반대쪽 경계에서 이 미분장을 쓰고 싶다

- 예를 들어:
- 미분장은 LEFT(u=0) 기준으로 만들어졌는데
- 실제 CAD 연산에서는 RIGHT(u=1) 경계에서 필요함

- ❌ 그대로 쓰면 안 됨
    - u=1에서의 미분값은 전혀 다른 값이기 때문

## 3️⃣ 핵심 아이디어: “곡선 확장으로 미분장을 옮긴다”

- 여기서 쓰이는 핵심 연산이 바로:
- Bezier curve extension with same derivatives
- START 확장
- 각 column curve에 대해:
- 입력 curve (in):
```
t=0 ---------------- t=1
 ^ 중요한 미분값
```
- 출력 curve (out):
```
t=0 ---------------- t=1
                      ^ 여기에 in(t=0)의 미분값을 복제
```

- 수식으로:
```math
out^{(k)}(1)=in^{(k)}(0)(k=0..p)
```
- 즉,
- 미분장 자체를 **뒤집는 게 아니라**
- 미분 데이터를 반대쪽 끝으로 옮긴 새 Bezier curve를 만드는 것

## 4️⃣ surface 전체에서 보면 이렇게 된다 (dir = UDir)
- 원본 미분장 (surP, LEFT 기준)
```
v ↑
  |   |   |   |   |
  |   |   |   |   |
  |   |   |   |   |
  +--------------------→ u
   t=0 (중요)      t=1
```
- 변환된 미분장 (surQ, RIGHT에서 사용 가능)
```
v ↑
  |   |   |   |   |
  |   |   |   |   |
  |   |   |   |   |
  +--------------------→ u
   t=0          t=1 (중요)
```

- 👉 이게 cst=LEFT, usg=RIGHT 케이스

## 5️⃣ 왜 swap(u,v)가 필요한 경우가 있나?

- 어떤 경우엔:
    - 미분장은 u 방향 Bezier인데
    - 사용하려는 경계는 v = 0 / v = 1
- 예: usg = BOTTOM or TOP
- 이때는:
    - u/v를 바꿔서
    - “v 방향 Bezier strip”으로 만든 뒤
    - 동일한 extend 논리를 적용

- swap 개념 그림
- 원본 (u-Bezier):
    - column들이 Bezier

- swap 후 (v-Bezier):
    - row들이 Bezier


- 개념 흐름 요약:
```
[입력 미분장 surP]
   |
   |  (필요 시) swap u/v
   |
   |  (필요 시) Bezier curve extension
   |      - START : in(0) → out(1)
   |      - END   : in(1) → out(0)
   |
[출력 미분장 surQ]
```
- 원하는 경계(LEFT/RIGHT/BOTTOM/TOP)에서  
    동일한 동차 미분값을 제공

## 7️⃣ 이 함수의 “정확한 용도 한 문장 요약”
- on_adjust_derivative_field_bezier는 Bezier strip으로 표현된 동차 미분장을
    **다른 경계에서도 같은 미분 조건을 만족하도록** swap + extension을 조합해 재배치하는 함수다.

- ❌ geometry를 보존하는 함수 아님
- ❌ 같은 u,t에서 같은 값 보장 안 함
- ✅ “경계에서의 미분 조건”만 정확히 보존

## 8️⃣ 왜 CAD 커널에서 중요하냐?

- 이게 없으면:
- sweep / loft / cross-section에서
- 한쪽 면에서 계산한 derivative field를
- 다른 면에서 재사용할 수 없음

- 즉, 이 함수는:
    - **미분장의 좌표계 어댑터 + 경계 변환기**

---
## 소스 코드
```rust

/// - 입력 surP는 "한 방향(dir)으로 Bezier strip"인 derivative surface(4D/homogeneous)라고 가정.
/// - cst(constructed along) 와 usg(used along) 조합에 따라:
///   - 그대로 복사
///   - swap_uv
///   - Bezier strip extension(B_sextsd) 수행
///   - swap + extension 조합 수행
///
/// ✅ 여기서는 이미 네가 가진:
/// - BezierSurface::extend_strip_with_same_derivatives(dir, side, reverse_param)
/// - on_pascal_triangle_* (extend 내부에서 필요하면 사용)
/// - on_swap_uv_* (Bezier용으로는 on_swap_uv_bezier_surface_in_place 사용)
/// 주의:
/// - 결과를 새 BezierSurface로 반환하는 형태가 안전함.
pub fn on_adjust_derivative_field_bezier(
    sur_p: &BezierSurface,
    dir: SurfaceDir,
    cst: SideFlag,
    usg: SideFlag,
) -> Result<BezierSurface, NurbsError> {
    let sdr = match dir {
        SurfaceDir::UDir => SurfaceDir::VDir,
        SurfaceDir::VDir => SurfaceDir::UDir,
    };

    let pq = std::cmp::max(sur_p.u_degree, sur_p.v_degree);
    let _bin_u128 = on_pascal_triangle_u128(pq);

    // 작업 대상( clone으로 처리)
    let mut sur_q = sur_p.clone();

    // 원본 switch(cst) / switch(usg) 그대로 매핑
    match cst {
        SideFlag::Left => match usg {
            SideFlag::Left => {
                // 그대로 (copy)
            }
            SideFlag::Right => {
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    dir,
                    ExtendSide::Start,
                    false,
                );
            }
            SideFlag::Bottom => {
                // copy + swapuv
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
            }
            SideFlag::Top => {
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    sdr,
                    ExtendSide::Start,
                    false,
                );
            }
        },

        SideFlag::Right => match usg {
            SideFlag::Left => {
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    dir,
                    ExtendSide::End,
                    false,
                );
            }
            SideFlag::Right => {
                // 그대로 (copy)
            }
            SideFlag::Bottom => {
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    sdr,
                    ExtendSide::End,
                    false,
                );
            }
            SideFlag::Top => {
                // copy + swapuv
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
            }
        },

        SideFlag::Bottom => match usg {
            SideFlag::Left => {
                // copy + swapuv
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
            }
            SideFlag::Right => {
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    sdr,
                    ExtendSide::Start,
                    false,
                );
            }
            SideFlag::Bottom => {
                // 그대로 (copy)
            }
            SideFlag::Top => {
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    dir,
                    ExtendSide::Start,
                    false,
                );
            }
        },

        SideFlag::Top => match usg {
            SideFlag::Left => {
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    sdr,
                    ExtendSide::End,
                    false,
                );
            }
            SideFlag::Right => {
                // copy + swapuv
                on_swap_uv_bezier_surface_in_place(&mut sur_q);
            }
            SideFlag::Bottom => {
                sur_q = sur_q.extend_strip_with_same_derivatives(
                    dir,
                    ExtendSide::End,
                    false,
                );
            }
            SideFlag::Top => {
                // 그대로 (copy)
            }
        },
    }

    Ok(sur_q)
}
```
---
