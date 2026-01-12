# Matrix borrow
- MatrixSliceMut, MatrixBlockMut, BlockMut 구조 설명서

## 기본 Matrix 구조
```Rust
#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
    pub row_offset: isize,
    pub col_offset: isize,
}
```
- rows, cols 베이스로 2D 행렬 쪼개 쓰기

## 1. 왜 이렇게 세 가지 뷰가 필요한가
- 구조는 크게 세 레벨:
- Matrix:
    - 실제 Vec<f64> 메모리를 소유하는 2D 행렬 (row-major)
- MatrixSliceMut:
    - 연속된 row 범위에 대한 연속 메모리 뷰
        - **위/아래로 자른 행 단위 뷰**
        - split_rows_mut으로 만들어짐
        - 내부 메모리는 rows * cols 연속 블록
- MatrixBlockMut:
  - MatrixSliceMut 위에서 strided 2D 블록 뷰 (열 기준으로 잘린 영역)
      - **left/right column block**
      - 메모리는 전체 slice를 바라보지만, (row_stride, offset)으로 실제 블록 위치를 인덱싱
- BlockMut:
    - 완전히 safe한 블록 뷰 (연속 row 영역 + 열 window)
  - block_rows_mut, block_mut로 생성
      - 내부 data는 항상 rows * stride 크기의 연속 구간
      - col window는 col0 오프셋으로 처리
- 요약하면:
    - split_rows_mut → row 방향으로 안전하게 자르기 (완전 safe)
    - split_cols_mut → 한 row-slice를 column 기준으로 나누기 (내부 unsafe, 결과는 safe API)
    - BlockMut → **이 row 범위에서 col0..col0+cols만 다루고 싶다** 는 용도의 safe block
- 이 구조는 NURBS에서:
    - surface control net을 row/col/patch 단위로 나눌 때
    - Bezier patch 분해할 때
    - knot insertion 시 control point block을 각 방향으로 나눌 때 딱 필요한 형태.

## 2. MatrixSliceMut: 
- **연속적인 row 영역** 이라는 게 핵심
```rust
pub struct MatrixSliceMut<'a> {
    pub data: &'a mut [f64], // rows*cols 연속 구간
    pub rows: usize,
    pub cols: usize,         // == row_stride
}
```

- 여기서 중요한 포인트:
    - data는 항상 row * cols 만큼의 연속 메모리
    - cols == row_stride라서, row마다 바로 다음 row가 이어짐
- 그래서 인덱싱이 간단하게 된다:
```rust
pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
    &mut self.data[r * self.cols + c]
}
```

- 이 말은 곧:
    - 이 뷰는 **상하로 자른 행 덩어리** 이고
    - 내부는 여전히 일반적인 row-major 2D 배열과 동일하게 취급 가능
- 그리고 split_rows_mut이 이 성질을 보장해주고 있음.
``` rust
pub fn split_rows_mut(&mut self, row: usize) -> (MatrixSliceMut<'_>, MatrixSliceMut<'_>) {
    assert!(row <= self.rows);

    let split = row * self.cols;
    let (top, bottom) = self.data.split_at_mut(split);

    (
        MatrixSliceMut { data: top,    rows: row,           cols: self.cols },
        MatrixSliceMut { data: bottom, rows: self.rows-row, cols: self.cols },
    )
}
```

- 여기서:
    - split_at_mut는 Rust가 보장하는 완전 safe한 이분할
    - top과 bottom은 절대 겹치지 않는 두 개의 mutable slice
    - 그래서 top과 bottom을 동시에 &mut로 넘길 수 있고,
- 각각에서 마음대로 수정해도 borrow 문제 없음

## 3. MatrixBlockMut: 
- **연속이 아닌 column 블록을 stride+offset으로 표현**
- 열 기준으로 자르기(split_cols_mut)는 row-major 메모리에서 물리적으로 연속이 아님.
  - 그래서 연속 slice를 잘라 주는 것만으로는 block을 표현할 수 없음.
  - 그래서 나온 게 MatrixBlockMut:
```rust
pub struct MatrixBlockMut<'a> {
    pub data: &'a mut [f64], // 부모 slice 전체
    pub rows: usize,
    pub cols: usize,
    pub row_stride: usize,
    pub offset: usize,
}
```

- 여기서 의미는:
  - data: 원래 MatrixSliceMut가 갖고 있던 연속 메모리 전체 (위/아래로만 잘린 상태)
  - row_stride: 한 row에서 다음 row까지 건너뛰어야 하는 칸 수 (원본 cols와 동일)
  - offset: 이 block의 (0,0)이 data에서 시작하는 index
- 인덱싱:
```rust
pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
    let k = self.offset + r * self.row_stride + c;
    &mut self.data[k]
}
```
- 즉:
    - r행 → offset + r * stride
    - c열 → 그 안에서 + c
- 그리고 split_cols_mut은 이렇게 block을 두 개 만듬:
```rust
pub fn split_cols_mut(&mut self, col: usize) -> (MatrixBlockMut<'_>, MatrixBlockMut<'_>) {
    assert!(col <= self.cols);

    let rows = self.rows;
    let cols = self.cols;
    let stride = cols;

    let ptr = self.data.as_mut_ptr();
    let len = self.data.len();

    let mk = |offset: usize, sub_cols: usize| MatrixBlockMut {
        data: unsafe { std::slice::from_raw_parts_mut(ptr, len) },
        rows,
        cols: sub_cols,
        row_stride: stride,
        offset,
    };

    let left  = mk(0, col);
    let right = mk(col, cols - col);

    (left, right)
}
```
- 여기서 핵심:
    - ptr + len으로 원래 slice의 전체 영역을 다시 만들고
    - left: offset = 0, cols = col
    - right: offset = col, cols = cols-col
- 즉, 논리적으로:
    - left는 원래 row에서 0..col)
    - right는 col..end)
    - 두 block이 서로 다른 column 범위를 가진다는 점에서 논리적으로 disjoint
- Rust는 이런 “논리적 불변식”을 이해 못하니까, 내부에서 unsafe로 만들어 주고,  
    바깥에는 at_mut 같은 safe API만 열어둔 구조.
- 테스트가 그걸 증명:
```rust
*left.at_mut(2, 3) = -1.0;   // (2,3)
*right.at_mut(2, 1) = -2.0;  // (2, 4+1) = (2,5)
```
- 이 둘이 같은 메모리를 건드리지 않는다는 걸 수학적으로 보장하고 있는 것임.

## 4. BlockMut: 완전히 safe한 block view
- MatrixBlockMut는 **부모 slice 전체 + offset/stride 기반 view** 라서  
    내부 구조는 유연하지만, unsafe가 들어가 있기 때문에 **일반 사용자가 직접 쓰는 API** 로는 부담이 있음.
- 그래서 네가 따로 둔 게 BlockMut:
```rust
pub struct BlockMut<'a> {
    data: &'a mut [f64], // rows*stride 연속 구간
    pub rows: usize,
    pub cols: usize,     // block width
    stride: usize,       // matrix cols
    col0: usize,         // starting column
}
```
- data는 항상 row-range에 대한 연속 구간
- 열 방향 window만 col0과 cols로 표현
- row stride는 원래 matrix의 cols와 동일
- block_rows_mut:
```rust
pub fn block_rows_mut(&mut self, r0: usize, rows: usize) -> BlockMut<'_> {
    let stride = self.cols;
    let start = r0 * stride;
    let end = start + rows * stride;
    BlockMut { data: &mut self.data[start..end], rows, cols: self.cols, stride, col0: 0 }
}
```
- block_mut:
```rust
pub fn block_mut(&mut self, r0: usize, c0: usize, rows: usize, cols: usize) -> BlockMut<'_> {
    let stride = self.cols;
    let start = r0 * stride;
    let end = start + rows * stride;
    BlockMut { data: &mut self.data[start..end], rows, cols, stride, col0: c0 }
}
```

둘 다:
    - row 범위는 split_at_mut 스타일로 실제 slice를 자르고
    - col window는 인덱싱 계산으로만 처리
- 인덱싱:
```rust
pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
    let idx = r * self.stride + (self.col0 + c);
    &mut self.data[idx]
}
```
- 이 구조의 장점:
    - data는 항상 실제 disjoint slice
    - col window만 인덱싱으로 처리 → Rust 입장에서 완전 safe
    - row/col 둘 다 잘라진 상태를 표현 가능
    - NURBS에서 “특정 patch 영역 (i..i+m, j..j+n)”을 다룰 때 적합

## 5. 테스트들이 보장해주는 것들
- 이 테스트들이 의미하는 건:
    - split_rows_mut
    - top/bottom이 완전히 분리된 구간이라는 것
    - 하나를 수정해도, 나머지 영역은 원본대로 유지된다는 것
    - split_cols_mut
    - 좌/우 block이 논리적으로 다른 열 영역이라는 것
    - 서로 다른 column에만 영향을 준다는 것
    - rows → cols 순서로 4분할

```rust
let (mut top, mut bottom) = m.split_rows_mut(i);
// top: split cols
{
    let (mut tl, mut tr) = top.split_cols_mut(j);
    ...
}
// bottom: split cols
{
    let (mut bl, mut br) = bottom.split_cols_mut(j);
    ...
}
```

- 이건 사실상:
    - A: (0..i, 0..j)
    - B: (0..i, j..end)
    - C: (i..end, 0..j)
    - D: (i..end, j..end)
    - 이라는 4-quadrant 분해가 정확히 작동한다는 것을 보여주고 있음.
    - Surface Bezier patch 분해에서 control net을 4개로 나눌 때 쓰이는 바로 그 패턴.
    - block_mut
    - 임의의 (r0, c0, rows, cols) 영역에 대해
- BlockMut가 정확한 cell을 가리키는지 검증
- 이 테스트들 덕분에:
    - 메모리 disjoint 보장
    - index 계산 정확성
    - Rust borrow-safe 수준에서의 로컬 correctness 까지 확인된 상태.

## 6. NURBS / Bezier patch 관점에서 보면
- 이 구조는 그냥 **행렬 유틸리티** 가 아니라, 딱 NURBS surface control net용:
    - u 방향 split → split_rows_mut
    - U-knot에서 분리된 patch의 row 범위
    - v 방향 split → split_cols_mut
    - V-knot에서 분리된 patch의 column 범위
    - (i,j) 기준 네 개 patch → split_rows_mut 후 split_cols_mut 두 번
    - 정확히 지금 테스트 split_rows_then_cols_quadrants가 검증하는 것
    - 특정 patch 또는 블록 연산 → block_mut
    - knot insertion에서 특정 서브패치 영역만 업데이트
    - Bezier patch basis 변환 시 해당 블록만 처리
---


## 핵심 테스트 코드
```rust
#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
    pub row_offset: isize,
    pub col_offset: isize,
}
```
```rust
// -------------------------
// Row-slice view (SAFE)
// -------------------------
pub struct MatrixSliceMut<'a> {
    pub data: &'a mut [f64], // rows*cols 연속 구간
    pub rows: usize,
    pub cols: usize,         // == row_stride
}
```
```rust
impl<'a> MatrixSliceMut<'a> {
    pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
        debug_assert!(r < self.rows);
        debug_assert!(c < self.cols);
        &mut self.data[r * self.cols + c]
    }

    /// Split by columns (requires unsafe internally, returns 2 block views)
    pub fn split_cols_mut(&mut self, col: usize) -> (MatrixBlockMut<'_>, MatrixBlockMut<'_>) {
        assert!(col <= self.cols);

        let rows = self.rows;
        let cols = self.cols;
        let stride = cols;

        let ptr = self.data.as_mut_ptr();
        let len = self.data.len();

        let mk = |offset: usize, sub_cols: usize| MatrixBlockMut {
            data: unsafe { std::slice::from_raw_parts_mut(ptr, len) },
            rows,
            cols: sub_cols,
            row_stride: stride,
            offset,
        };

        let left  = mk(0, col);
        let right = mk(col, cols - col);

        (left, right)
    }
}
```
```rust
// -------------------------
// Strided block view (for split_cols_mut results)
// -------------------------
#[derive(Debug)]
pub struct MatrixBlockMut<'a> {
    pub data: &'a mut [f64], // 부모 slice
    pub rows: usize,
    pub cols: usize,
    pub row_stride: usize,
    pub offset: usize,
}
```
```rust
impl<'a> MatrixBlockMut<'a> {
    pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
        debug_assert!(r < self.rows);
        debug_assert!(c < self.cols);
        let k = self.offset + r * self.row_stride + c;
        &mut self.data[k]
    }
}
```
```rust
// -------------------------
// Safe block view (recommended usage)
// -------------------------
pub struct BlockMut<'a> {
    data: &'a mut [f64], // rows*stride 연속 구간
    pub rows: usize,
    pub cols: usize,     // block width
    stride: usize,       // matrix cols
    col0: usize,         // starting column
}
```
```rust
impl<'a> BlockMut<'a> {
    pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
        debug_assert!(r < self.rows);
        debug_assert!(c < self.cols);
        let idx = r * self.stride + (self.col0 + c);
        &mut self.data[idx]
    }
}
```
```rust
impl Matrix {
    /// Safe row split (top/bottom have disjoint slices)
    pub fn split_rows_mut(&mut self, row: usize) -> (MatrixSliceMut<'_>, MatrixSliceMut<'_>) {
        assert!(row <= self.rows);

        let split = row * self.cols;
        let (top, bottom) = self.data.split_at_mut(split);

        (
            MatrixSliceMut { data: top,    rows: row,            cols: self.cols },
            MatrixSliceMut { data: bottom, rows: self.rows-row,  cols: self.cols },
        )
    }
```
```rust
    /// Safe block over contiguous row-range; supports col window by (col0, cols)
    pub fn block_rows_mut(&mut self, r0: usize, rows: usize) -> BlockMut<'_> {
        assert!(r0 + rows <= self.rows);
        let stride = self.cols;
        let start = r0 * stride;
        let end = start + rows * stride;
        BlockMut { data: &mut self.data[start..end], rows, cols: self.cols, stride, col0: 0 }
    }
```
```rust
    pub fn block_mut(&mut self, r0: usize, c0: usize, rows: usize, cols: usize) -> BlockMut<'_> {
        assert!(r0 + rows <= self.rows);
        assert!(c0 + cols <= self.cols);
        let stride = self.cols;
        let start = r0 * stride;
        let end = start + rows * stride;
        BlockMut { data: &mut self.data[start..end], rows, cols, stride, col0: c0 }
    }
}
```

---

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::matrix::Matrix;
```
```rust
    fn make_matrix(rows: usize, cols: usize) -> Matrix {
        let mut data = Vec::with_capacity(rows * cols);
        for k in 0..rows * cols {
            data.push(k as f64);
        }
        Matrix { rows, cols, data, row_offset: 0, col_offset: 0 }
    }
```
```rust
    fn idx(cols: usize, r: usize, c: usize) -> usize {
        r * cols + c
    }
```
```rust
    fn assert_cell(m: &Matrix, r: usize, c: usize, v: f64) {
        assert_eq!(m.data[idx(m.cols, r, c)], v, "cell mismatch at ({},{})", r, c);
    }
```
```rust
    fn assert_unchanged_except(m: &Matrix, changed: &[(usize, usize, f64)]) {
        let mut mark = vec![false; m.rows * m.cols];
        for &(r, c, _) in changed {
            mark[idx(m.cols, r, c)] = true;
        }
        for r in 0..m.rows {
            for c in 0..m.cols {
                let k = idx(m.cols, r, c);
                if mark[k] { continue; }
                assert_eq!(m.data[k], k as f64, "unexpected change at ({},{})", r, c);
            }
        }
        for &(r, c, v) in changed {
            assert_cell(m, r, c, v);
        }
    }
```
```rust
    // -------------------------
    // split_rows_mut tests
    // -------------------------
    #[test]
    fn split_rows_mut_writes_top_and_bottom() {
        let mut m = make_matrix(6, 4);
        let cut = 2;

        let (mut top, mut bottom) = m.split_rows_mut(cut);

        *top.at_mut(1, 3) = -10.0;     // matrix (1,3)
        *bottom.at_mut(0, 0) = -20.0;  // matrix (cut+0,0) = (2,0)

        drop(top);
        drop(bottom);

        assert_unchanged_except(&m, &[(1, 3, -10.0), (2, 0, -20.0)]);
    }
```
```rust
    #[test]
    fn split_rows_mut_cut_zero_all_bottom() {
        let mut m = make_matrix(5, 3);
        let (mut top, mut bottom) = m.split_rows_mut(0);

        assert_eq!(top.rows, 0);
        assert_eq!(bottom.rows, 5);

        *bottom.at_mut(4, 2) = 999.0;

        drop(top);
        drop(bottom);

        assert_unchanged_except(&m, &[(4, 2, 999.0)]);
    }
```
```rust
    #[test]
    fn split_rows_mut_cut_end_all_top() {
        let mut m = make_matrix(5, 3);
        let (mut top, mut bottom) = m.split_rows_mut(5);

        assert_eq!(top.rows, 5);
        assert_eq!(bottom.rows, 0);

        *top.at_mut(4, 2) = 777.0;
        drop(top);
        drop(bottom);
        assert_unchanged_except(&m, &[(4, 2, 777.0)]);
    }
```
```rust
    #[test]
    #[should_panic]
    fn split_rows_mut_panics_if_cut_too_big() {
        let mut m = make_matrix(5, 3);
        let _ = m.split_rows_mut(6);
    }
```
```rust
    // -------------------------
    // split_cols_mut tests (via MatrixSliceMut)
    // -------------------------
    #[test]
    fn split_cols_mut_left_right_non_overlapping_writes() {
        let mut m = make_matrix(4, 6);
        let (mut top, _) = m.split_rows_mut(4);

        let cut = 4;
        let (mut left, mut right) = top.split_cols_mut(cut);

        // left covers cols [0..4), right covers [4..6)
        *left.at_mut(2, 3) = -1.0;   // (2,3)
        *right.at_mut(2, 1) = -2.0;  // (2, 4+1) = (2,5)

        drop(left);
        drop(right);
        drop(top);

        assert_unchanged_except(&m, &[(2, 3, -1.0), (2, 5, -2.0)]);
    }
```
```rust
    #[test]
    fn split_cols_mut_cut_zero_all_right() {
        let mut m = make_matrix(3, 5);
        let (mut top, _) = m.split_rows_mut(3);

        let (mut left, mut right) = top.split_cols_mut(0);
        assert_eq!(left.cols, 0);
        assert_eq!(right.cols, 5);

        *right.at_mut(2, 4) = 123.0;

        drop(left);
        drop(right);
        drop(top);

        assert_unchanged_except(&m, &[(2, 4, 123.0)]);
    }
```
```rust
    #[test]
    fn split_cols_mut_cut_end_all_left() {
        let mut m = make_matrix(3, 5);
        let (mut top, _) = m.split_rows_mut(3);

        let (mut left, mut right) = top.split_cols_mut(5);
        assert_eq!(left.cols, 5);
        assert_eq!(right.cols, 0);

        *left.at_mut(1, 2) = 321.0;

        drop(left);
        drop(right);
        drop(top);

        assert_unchanged_except(&m, &[(1, 2, 321.0)]);
    }
```
```rust
    #[test]
    #[should_panic]
    fn split_cols_mut_panics_if_cut_too_big() {
        let mut m = make_matrix(3, 5);
        let (mut top, _) = m.split_rows_mut(3);
        let _ = top.split_cols_mut(6);
    }
```
```rust
    // -------------------------
    // 4-way composition test
    // -------------------------
    #[test]
    fn split_rows_then_cols_quadrants() {
        let mut m = make_matrix(6, 6);
        let i = 2;
        let j = 4;

        let (mut top, mut bottom) = m.split_rows_mut(i);

        // top: split cols
        {
            let (mut tl, mut tr) = top.split_cols_mut(j);
            *tl.at_mut(0, 0) = 1.0;        // (0,0)
            *tr.at_mut(1, 1) = 2.0;        // (1, j+1)
        }

        // bottom: split cols
        {
            let (mut bl, mut br) = bottom.split_cols_mut(j);
            *bl.at_mut(0, 3) = 3.0;        // (i+0, 3)
            *br.at_mut(2, 0) = 4.0;        // (i+2, j+0)
        }

        drop(top);
        drop(bottom);

        assert_unchanged_except(
            &m,
            &[
                (0, 0, 1.0),
                (1, j + 1, 2.0),
                (i + 0, 3, 3.0),
                (i + 2, j + 0, 4.0),
            ],
        );
    }
```
```rust
    // -------------------------
    // block_mut sanity (safe block)
    // -------------------------
    #[test]
    fn block_mut_writes_correct_cell() {
        let mut m = make_matrix(4, 5);

        {
            let mut b = m.block_mut(2, 3, 1, 1);
            *b.at_mut(0, 0) = 1234.0;
        }

        assert_cell(&m, 2, 3, 1234.0);
        assert_unchanged_except(&m, &[(2, 3, 1234.0)]);
    }
}

```
## drop의 의미
- drop(top);
- drop(bottom);
- 을 호출해서:
    - top의 borrow 종료
    - bottom의 borrow 종료
    - 이제 m.data를 다시 읽어도 borrow 충돌 없음

---

## 1. 왜 이렇게 복잡한 구조가 필요했는가
- Rust는 &mut T는 단 하나만 존재해야 한다는 규칙이 있음.
- 그런데 행렬을 row/col/block 단위로 쪼개려면:
    - 위쪽 행들(top)
    - 아래쪽 행들(bottom)
    - 왼쪽 열(left)
    - 오른쪽 열(right)
- 이렇게 여러 개의 서로 다른 부분을 동시에 &mut로 다뤄야 함.
- C++이나 Python에서는 그냥 포인터로 하면 끝인데 Rust는 이걸 허용하지 않음.
- 그래서 필요한 게 바로:
    - ✔ **메모리는 하나지만, 서로 겹치지 않는 부분만 안전하게 가리키는 view 구조**
- 이게 바로
    - MatrixSliceMut, MatrixBlockMut, BlockMut


## 2. MatrixSliceMut — “행 단위로 자른 연속 메모리”
- 이건 완전 safe.
- 왜냐하면:
```rust
let (top, bottom) = self.data.split_at_mut(row * cols);
```
- 이 split_at_mut은 Rust가 보장하는:
    - top과 bottom은 절대 겹치지 않는다
    - 둘 다 &mut로 써도 된다
- 라는 성질을 갖고 있음.
- 그래서 row 방향 split은 완전 safe.

## 3. MatrixBlockMut 
- “열 방향으로 자른 블록 (stride + offset)”
    - 문제는 column split.
- row-major 메모리에서는 열 방향으로 자르면 메모리가 이렇게 생김:
```rust
[0 1 2 3 4 5]
[6 7 8 9 10 11]
[12 13 14 15 16 17]
```
- 여기서 col=2로 split하면:
    - left block: (0,1), (6,7), (12,13)
    - right block: (2,3,4,5), (8,9,10,11), (14,15,16,17)
- 즉, 물리적으로 연속이 아님.
- 그래서 Rust는 split_at_mut을 쓸 수 없음.
- 그래서 나온 게:
- ✔ MatrixBlockMut = **stride + offset으로 논리적 2D 블록을 표현하는 구조**
- 이 구조는:
    - data는 전체 row-slice
    - offset은 block의 시작 위치
    - row_stride는 한 row의 전체 길이
    - cols는 block의 폭
- 그래서 인덱싱:
```
offset + r * stride + c
```

## 4. BlockMut — “완전히 safe한 block view”
- MatrixBlockMut는 내부에 unsafe가 들어가 있어.
- 그래서 일반 사용자에게는 부담이 있음.
- 그래서 나온 게 BlockMut:
    - row 범위는 실제 slice로 잘라서 safe
    - col window는 offset으로만 처리
    - 내부는 완전 safe
- 즉:
    - ✔ BlockMut은 **사용자 친화적인 safe block view**

## 5. 왜 drop을 쓰는가
- 테스트에서만 쓰는 이유는:
    - top과 bottom이 여전히 m.data를 빌리고 있다고 Rust가 판단하기 때문에
    - 원본 Matrix를 다시 읽으려면 borrow를 끝내야 해서
    - drop(top), drop(bottom)으로 명시적으로 borrow를 종료시키는 것
- 실제 라이브러리 코드에서는 필요 없음.

---

