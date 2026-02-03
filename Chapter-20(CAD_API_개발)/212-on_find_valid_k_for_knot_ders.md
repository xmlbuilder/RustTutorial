# on_find_valid_k_for_knot_ders / on_find_span_left_right

- 이 두 함수의 차이는 겉으로 보면 비슷하지만, 목적과 동작 방식이 완전히 다르다.

## 📘 두 함수의 역할 차이 요약
| 함수 이름                         | 역할                                   | 입력                                   | 출력                     | 핵심 목적                                      |
|-----------------------------------|-----------------------------------------|-----------------------------------------|---------------------------|------------------------------------------------|
| on_find_valid_k_for_knot_ders     | 미분(ders) 가능한 k 인덱스 찾기         | knot vector, degree p, side             | Option<k>                | knot 불연속 검사 후, 미분 가능한 k 위치 찾기   |
| on_find_span_left_right           | 주어진 u가 속한 knot span 찾기          | knot vector, degree p, u, side          | Result<span index>       | basis function 평가를 위한 span index 찾기     |


- 즉:
    - 첫 번째 함수는 **미분 가능한 k 위치 찾기**
    - 두 번째 함수는 **u가 속한 span 찾기**

- 둘은 완전히 다른 목적을 가진 함수다.

### 🔍 1) on_find_valid_k_for_knot_ders
- ✔ 목적
- NURBS 미분(ders) 계산 시 사용할 수 있는 유효한 k 인덱스를 찾는 함수
- 왜냐하면 knot vector에서
    - knot이 반복되면 (multiplicity)
    - 그 지점에서는 미분이 불가능하거나 정의되지 않기 때문
- 그래서 조건이 이렇게 생긴다:
- ✔ 유효 조건
    - p < k <= m - p - 1
    - LEFT  → U[k] != U[k-1]
    - RIGHT → U[k] != U[k+1]


- 즉:
    - knot이 변하는 지점(불연속이 아닌 곳)
    - basis function 미분이 가능한 지점
- 만 반환한다.
- ✔ 반환
    - Some(k) → 유효한 k
    - None → 유효한 k 없음
- ✔ 사용처
    - NURBS 곡선/표면 미분
    - tangent, curvature 계산
    - basis derivative 계산

### 🔍 2) on_find_span_left_right
- ✔ 목적
    - 주어진 u가 knot vector에서 어느 span에 속하는지 찾는 함수
    - 즉:
        - U[k]\leq u<U[k+1]
    - 을 만족하는 k를 찾는 것.
- ✔ side에 따른 경계 처리
    - LEFT → u가 knot 끝에 닿으면 마지막 span
    - RIGHT → u가 knot 시작에 닿으면 첫 span
- ✔ 반환
    - Ok(span index)
    - Err → u가 domain 밖이거나 knot vector가 잘못됨
- ✔ 사용처
    - basis function 평가
    - surface/curve evaluator
    - interpolation, projection

## 🎯 두 함수의 본질적 차이
- ✔ 목적이 완전히 다르다
- on_find_valid_k_for_knot_ders
    -  **미분 가능한 k를 찾는 함수**
- on_find_span_left_right
    - **u가 속한 span을 찾는 함수**

- ✔ 입력/출력 의미가 다르다
    - 첫 번째 함수의 k는 미분용 인덱스
    - 두 번째 함수의 k는 basis span index

- ✔ side 의미도 다르다
    - 첫 번째 함수의 side
        - **knot 변화 방향(왼쪽/오른쪽)에서 유효한 k 찾기**
    - 두 번째 함수의 side
        - **u가 경계에 있을 때 span을 어떻게 선택할지**

- ✔ 내부 조건도 완전히 다르다
- on_find_valid_k_for_knot_ders
    - knot 불연속 검사
    - p < k <= m - p - 1
    - U[k] != U[k-1] 또는 U[k] != U[k+1]
- on_find_span_left_right
    - u가 knot domain 안에 있는지 검사
    - binary search 기반 span 찾기
    - 경계 처리

- 두 함수 모두:
    - knot vector를 다루고
    - degree p를 사용하고
    - side(Left/Right)를 사용하고
    - k 또는 span index를 반환한다
- 그래서 표면적으로는 비슷해 보인다.
- 하지만 실제로는:
    - 하나는 미분용 k 찾기
    - 하나는 basis span 찾기

- 즉, 목적 자체가 완전히 다르다.

## 🎯 최종 요약
- ✔ on_find_valid_k_for_knot_ders
    - 미분 가능한 k 찾기
    - knot 불연속 검사
    - basis derivative 계산용
- ✔ on_find_span_left_right
    - u가 속한 span 찾기
    - evaluator에서 basis function 계산용


## 🎯 핵심 요약
- on_find_span_left_right
    - u가 어느 구간(span)에 속하는지만 찾으면 되므로
    - **knot 반복(multiplicity)** 이 있어도 전혀 문제 없음
- on_find_valid_k_for_knot_ders
    - 미분(ders) 계산을 위해서는
    - U[k] ≠ U[k−1] (LEFT)
    - U[k] ≠ U[k+1] (RIGHT)
    - 즉, knot이 변하는 지점만 유효
- 그래서 span은 찾을 수 있지만, 미분은 불가능한 knot가 존재한다.

## 📌 예시 1 — “span은 OK, 미분은 불가능한 knot vector”
- Knot vector:
    - U = [0, 0, 0, 1, 2, 2, 2, 3, 4, 4, 4]

- Degree:
    - p = 2



- ✔ on_find_span_left_right(u) → 정상 동작
- 예를 들어 u = 2.0 이라고 하자.
```rust
U = [0,0,0,1,2,2,2,3,4,4,4]
                ^^^
```

- u = 2.0 은
    - U[4] = 2
    - U[5] = 2
    - U[6] = 2
    - U[7] = 3
    - 즉, span은 4, 5, 6 중 아무거나 side 규칙에 따라 선택된다.
        - span 찾기는 문제 없음

- ❌ 하지만 미분용 k는 절대 사용 불가
- 미분용 k 조건:
```
LEFT:
U[k] != U[k-1]
```
```
RIGHT:
U[k] != U[k+1]
```

- 그런데 2.0 구간은:
    - U[4] = 2
    - U[5] = 2
    - U[6] = 2


- 즉:
    - U[4] == U[3]? → 2 != 1 → OK
    - U[5] == U[4]? → 2 == 2 → 불가
    - U[6] == U[5]? → 2 == 2 → 불가
- RIGHT도 마찬가지:
    - U[4] == U[5]? → 2 == 2 → 불가
    - U[5] == U[6]? → 2 == 2 → 불가
    - U[6] == U[7]? → 2 != 3 → OK
- 즉:
    - ✔ 미분 가능한 k는 4 또는 6 뿐
    - ✔ 하지만 span은 4,5,6 모두 가능
- 즉:
    - span은 찾을 수 있지만, 미분은 특정 k에서만 가능하다.


## 📌 예시 2 — 더 극단적인 경우
- Knot vector:
    - U = [0, 0, 0, 1, 1, 1, 2, 3, 4, 4, 4]


- 여기서 1.0 구간은 완전히 flat:
    - U[3] = 1
    - U[4] = 1
    - U[5] = 1


- span 찾기
    - u = 1.0 → span = 3,4,5 중 하나
    - 정상
- 미분용 k 찾기
- LEFT 조건: U[k] != U[k−1]
- RIGHT 조건: U[k] != U[k+1]
- 하지만:
    - U[3] == U[4]
    - U[4] == U[5]
    - U[5] == U[6] (1 != 2 → OK)
- 즉:
    - k=3 → LEFT OK, RIGHT 불가
    - k=4 → LEFT 불가, RIGHT 불가
    - k=5 → LEFT 불가, RIGHT OK
- 결론:
- span은 3,4,5 모두 가능하지만
    - 미분은 k=3 또는 k=5에서만 가능하고
    - k=4에서는 절대 불가능


## ⭐ 최종 정리
- ✔ on_find_span_left_right
    - u가 어느 구간에 속하는지만 찾으면 됨
    - knot 반복(multiplicity) 있어도 문제 없음
    - 평가(evaluation)에는 OK
- ✔ on_find_valid_k_for_knot_ders
    - 미분(ders) 계산을 위해서는 knot이 변하는 지점만 유효
    - 반복 knot 구간에서는 미분 불가능
    - 곡률, tangent, normal 계산 시 반드시 필요

## 🎯 한 줄 요약
- span 찾기는 knot 반복이 있어도 되지만, 미분은 knot이 변하는 지점에서만 가능하다.

---


