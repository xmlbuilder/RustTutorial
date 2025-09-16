# Match 코드 실전

## 샘플 코드
```rust
pub fn is_planar(
    &self,
    planar_tolerance: f64,
    angle_tolerance_radians: f64,
    verts: &[Point3D],
    out_plane: Option<&mut PlaneEquation>,
) -> bool {
    let plane = match self.get_plane_equation(verts) {
        Some(pe) => pe,
        None => {
            if let Some(o) = out_plane {
                *o = PlaneEquation::UNSET;
            }
            return false;
        }
    };

    // 이후 plane 사용
    ...
}
```

### ✅ 문법적으로 OK인 이유
- let mut plane;은 선언만 하고, 이후 조건문에서 초기화됨
- if let Some(pe) = ... 패턴 매칭도 올바름
- Option<&mut T>에서 if let Some(o)로 꺼내서 *o = ... 하는 것도 적절함
- return false;로 조기 반환하는 흐름도 자연스러움

### ⚠️ 설계적으로 고려할 점
#### 1. plane 초기화가 조건문 안에만 있음
- 이 구조는 plane이 반드시 초기화된다는 보장이 조건문에 의존해요
- Rust는 컴파일 타임에 “모든 경로에서 초기화됨”을 확인해야 하므로
return false가 없으면 컴파일 에러가 날 수 있어요

#### 2. 스코프 중첩이 불필요할 수 있음
```rust
{
    if let Some(pe) = ... { ... } else { ... }
}
```
- 중괄호 {}로 감싼 이유가 없다면 제거해도 돼요
- 혹시 plane을 이후에 더 쓰지 않는다면, let plane = ...을 if let 안에서 선언해도 됨


### ✅ 이 코드의 목적
- get_plane_equation()이 성공하면 → plane에 결과 저장
- 실패하면 → out_plane에 실패 상태 저장하고 → false 반환
즉, 이 코드는 조건에 따라 흐름을 분기하면서 동시에 값을 추출하는 아주 Rust스러운 방식이에요.


## 수정 코드
```rust
let plane = match self.get_plane_equation(verts) {
    Some(pe) => pe,
    None => {
        if let Some(o) = out_plane {
            *o = PlaneEquation::UNSET;
        }
        return false;
    }
};

```
### 1. self.get_plane_equation(verts)
- 이 함수는 Option<PlaneEquation>을 반환한다고 가정해요
- 즉, 결과가 Some(pe)일 수도 있고 None일 수도 있어요

### 2. match 구문으로 분기
| 조건      | 처리 내용                       |
|-----------|----------------------------------|
| `Some(pe)` | `plane`에 `pe` 값을 대입         |
| `None`     | `out_plane`에 `UNSET` 저장 후 `false` 반환 |


match를 쓰면 흐름이 더 명확하고, plane이 초기화되지 않는 경로도 없어서 안전해요


## 💡 요약
| 단계 | 의미            | 문법 요소     | 설명                                      |
|------|-----------------|---------------|-------------------------------------------|
| ①    | 값 선언         | `let`         | 변수 선언만 하고 초기화는 나중에 수행       |
| ②    | 조건 분기       | `{}`          | 중괄호로 로직을 감싸서 스코프를 제한        |
| ③    | 패턴 매칭       | `match`       | `Option` 값을 분기하여 `Some`/`None` 처리   |
| ④    | 값 추출         | `Some(pe)`    | 값이 있으면 꺼내서 `plane`에 저장          |
| ⑤    | 실패 처리       | `None`        | 값이 없으면 `out_plane` 초기화 후 `return` |


## 🔍 구조 다시 보기
```rust
if let Some(o) = out_plane {
    *o = PlaneEquation::UNSET;
}
```

### 해석:
- out_plane는 Option<&mut PlaneEquation> 타입이에요
- Some(o)일 경우, o는 &mut PlaneEquation — 즉, 다른 곳에 있는 PlaneEquation을 가리키는 가변 참조
- *o = ...는 그 참조가 가리키는 실제 PlaneEquation 값을 변경하는 거예요

## ✅ 누가 초기화되는가?
out_plane가 Some(&mut target)일 때,
*o = PlaneEquation::UNSET;는 target을 초기화하는 거예요
즉, 이 함수 바깥에서 out_plane으로 넘겨준 실제 PlaneEquation 인스턴스가 변경됩니다.


##💡 요약
| 표현                     | 의미                                 |
|--------------------------|--------------------------------------|
| `out_plane: Option<&mut PlaneEquation>` | 외부에서 넘겨받은 수정 가능한 참조 |
| `if let Some(o)`         | 참조가 존재할 경우 꺼냄              |
| `*o = PlaneEquation::UNSET` | 참조가 가리키는 실제 값을 변경함     |
