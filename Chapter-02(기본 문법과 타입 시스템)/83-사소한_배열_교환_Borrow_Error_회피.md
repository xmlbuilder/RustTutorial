## 배열 내용 교환 

```rust
pub fn on_swap_real_array_2d<const N: usize>(
    vec: &mut [[Real; N]],
    i0: usize, j0: usize,
    i1: usize, j1: usize
) {
    let rows = vec.len();
    let cols = vec[0].len();

    if i0 < rows && i1 < rows && j0 < cols && j1 < cols {
        if i0 != i1 {
            // 서로 다른 행일 때는 split_at_mut로 안전하게 두 행을 분리
            let (row_a, row_b) = if i0 < i1 {
                let (left, right) = vec.split_at_mut(i1);
                (&mut left[i0], &mut right[0])
            } else {
                let (left, right) = vec.split_at_mut(i0);
                (&mut right[0], &mut left[i1])
            };
            std::mem::swap(&mut row_a[j0], &mut row_b[j1]);
        } else {
            // 같은 행일 경우는 그냥 swap 가능
            vec[i0].swap(j0, j1);
        }
    }
}
```

---

## 함수 개요와 목적
- 이 함수는 2차원 배열 &mut [[Real; N]]에서 두 좌표 (i0, j0)와 (i1, j1)의 값을 안전하게 교환합니다. 
- 같은 행이면 간단히 swap을 사용하고, 서로 다른 행이면 split_at_mut로 슬라이스를 분리해  
  borrow 충돌 없이 두 행을 동시에 가변 참조로 얻어 교환합니다.

- 시그니처와 기본 개념
```rust
pub fn on_swap_real_array_2d<const N: usize>(
    vec: &mut [[Real; N]],
    i0: usize, j0: usize,
    i1: usize, j1: usize
)
```
- `const N`: 열의 크기(한 행의 길이)를 컴파일 타임 상수로 받습니다.
- `&mut [[Real; N]]`: 행 개수는 동적, 열은 고정된 2차원 배열입니다.
- 의도: 두 위치를 교환할 때 borrow 에러 없이, 안전하게 처리합니다.

- 인덱스 범위 확인
```rust
let rows = vec.len();
let cols = vec[0].len();

if i0 < rows && i1 < rows && j0 < cols && j1 < cols {
    // ...
}
```

- 행 범위 체크: i0, i1가 rows 미만인지 확인.
- 열 범위 체크: j0, j1가 cols 미만인지 확인.
- 주의: `vec.len() == 0` 일 때 `vec[0].len()` 은 패닉이므로, 이 부분은 호출자가 빈 배열을 넘기지 않는다는 전제입니다.
- 안정성을 높이려면 `let cols = N;` 로 대체하는 것이 안전하고 의도가 명확합니다.
- 개선 제안: let cols = N;로 바꾸면 빈 배열에서도 안전하며, 열 크기는 이미 N으로 고정이기 때문에 의미적으로도 더 정확합니다.

- 같은 행일 때 간단 교환
```rust
if i0 != i1 {
    // ...
} else {
    vec[i0].swap(j0, j1);
}
```
- 같은 행: vec[i0]가 동일한 슬라이스이므로 slice::swap으로 두 열을 바로 교환할 수 있습니다.
- 장점: 간단하고 borrow 충돌이 없습니다.

-  서로 다른 행일 때 split_at_mut로 안전 분리
```rust
let (row_a, row_b) = if i0 < i1 {
    let (left, right) = vec.split_at_mut(i1);
    (&mut left[i0], &mut right[0])
} else {
    let (left, right) = vec.split_at_mut(i0);
    (&mut right[0], &mut left[i1])
};
std::mem::swap(&mut row_a[j0], &mut row_b[j1]);
```

- 왜 필요한가: `&mut vec[i0][j0]` 와 `&mut vec[i1][j1]` 를 동시에 만들면,  
  컴파일러는 두 가변 참조가 같은 메모리를 가리킬 위험을 배제하지 못해 에러를 냅니다.
- `split_at_mut` 는 슬라이스를 두 개의 비겹치는 가변 슬라이스로 나누어,  
  컴파일러가 **절대 겹치지 않는다** 는 것을 증명할 수 있게 해줍니다.

- i0 < i1인 경우:
  - vec.split_at_mut(i1)는 [0..i1)를 left, [i1..)를 right로 나눕니다.
  - row_a = &mut left[i0], row_b = &mut right[0]로 서로 다른 행을 안전히 참조합니다.
- i0 > i1인 경우: 동일한 논리로 기준을 i0로 바꿔 나눕니다.
  - 최종 교환: 비겹치는 두 행의 두 열을 std::mem::swap으로 교환합니다.
  - 이 방식은 Copy가 아닌 타입에도 안전하게 동작합니다.

- 전체 코드와 소소한 리팩터링

```rust
use std::mem::swap;

pub fn on_swap_real_array_2d<const N: usize>(
    vec: &mut [[Real; N]],
    i0: usize, j0: usize,
    i1: usize, j1: usize
) {
    let rows = vec.len();
    let cols = N; // vec[0].len() 대신 N을 쓰면 빈 배열에서도 안전

    if i0 < rows && i1 < rows && j0 < cols && j1 < cols {
        if i0 == i1 {
            vec[i0].swap(j0, j1);
        } else if i0 < i1 {
            let (left, right) = vec.split_at_mut(i1);
            let (row_a, row_b) = (&mut left[i0], &mut right[0]);
            swap(&mut row_a[j0], &mut row_b[j1]);
        } else {
            let (left, right) = vec.split_at_mut(i0);
            let (row_a, row_b) = (&mut right[0], &mut left[i1]);
            swap(&mut row_a[j0], &mut row_b[j1]);
        }
    }
}
```
- cols = N: 빈 배열 안전성 및 의미 명확성 향상.
- 분기 정리: 가독성을 위해 i0 == i1, i0 < i1, 그 외로 명확히 나눔.

## 핵심 요약
- 같은 행: slice::swap(j0, j1)로 간단 처리.
- 다른 행: split_at_mut로 슬라이스를 비겹치게 나눈 뒤 std::mem::swap으로 교환.
- 열 크기: const generics N을 직접 사용해 빈 배열에서도 안전하게 처리.

---


