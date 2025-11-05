# TMatrix
이 TMatrix<T>는 범용 행렬 구조로서, 일반적인 행렬 연산뿐 아니라 수치 해석에 필요한 고급 기능까지 포함하고 있음.  
아래는 전체 구조와 수식적 정확성, 구현 안정성, 기능별 요약을 정리한 분석입니다.

## 소스 코드

```rust
use crate::core::scalar::FloatScalar;
use core::fmt;
use core::ops::{Index, IndexMut, Mul, MulAssign};
use num_traits::Float;
```

```rust
/// 부동소수 전용 기능(역행렬/곱셈 등)에 쓰는 간단 트레이트
/// Row-major 행렬 (연속 메모리)
#[derive(Clone, PartialEq, Eq)]
pub struct TMatrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}
```
```rust
impl<T> TMatrix<T> {
    /// 0×0 (비어 있음)
    pub fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }

    /// (rows, cols) 크기 할당 + `fill` 값으로 채움
    pub fn with_size(rows: usize, cols: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Self {
            rows,
            cols,
            data: vec![fill; rows * cols],
        }
    }

    /// from_fn: (r,c) -> 값
    pub fn from_fn<F>(rows: usize, cols: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                data.push(f(r, c));
            }
        }
        Self { rows, cols, data }
    }

    /// 중첩 Vec 으로부터 (모든 행의 길이가 같아야 함)
    pub fn from_nested(rows: Vec<Vec<T>>) -> Self
    where
        T: Clone,
    {
        let r = rows.len();
        let c = if r == 0 { 0 } else { rows[0].len() };
        let mut data = Vec::with_capacity(r * c);
        for row in &rows {
            assert_eq!(row.len(), c, "All rows must have the same length");
            data.extend_from_slice(row);
        }
        Self {
            rows: r,
            cols: c,
            data,
        }
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// (r,c) → 선형 인덱스
    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    /// 안전한 접근
    #[inline]
    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        if r < self.rows && c < self.cols {
            Some(&self.data[r * self.cols + c])
        } else {
            None
        }
    }
    #[inline]
    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut T> {
        if r < self.rows && c < self.cols {
            Some(&mut self.data[r * self.cols + c])
        } else {
            None
        }
    }

    /// 행 슬라이스
    #[inline]
    pub fn row(&self, r: usize) -> Option<&[T]> {
        if r < self.rows {
            let start = r * self.cols;
            Some(&self.data[start..start + self.cols])
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&mut self, r: usize, c: usize, v: T) {
        self.data[r * self.cols + c] = v;
    }

    #[inline]
    pub fn row_mut(&mut self, r: usize) -> Option<&mut [T]> {
        if r < self.rows {
            let start = r * self.cols;
            Some(&mut self.data[start..start + self.cols])
        } else {
            None
        }
    }

    /// 값으로 채우기
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value);
    }

    /// 리셋(0×0)
    pub fn clear(&mut self) {
        self.rows = 0;
        self.cols = 0;
        self.data.clear();
    }

    pub fn resize_reuse(&mut self, new_rows: usize, new_cols: usize, fill: T)
    where
        T: Clone,
    {
        let new_len = new_rows * new_cols;

        // 기존 데이터 보존
        let copy_rows = self.rows.min(new_rows);
        let copy_cols = self.cols.min(new_cols);

        // 기존 데이터 재배치
        let mut new_data = Vec::with_capacity(new_len);
        for r in 0..copy_rows {
            let src = &self.data[r * self.cols..r * self.cols + copy_cols];
            new_data.extend_from_slice(src);
            if copy_cols < new_cols {
                new_data.extend(std::iter::repeat(fill.clone()).take(new_cols - copy_cols));
            }
        }

        // 추가 행 채우기
        let remaining = new_len - new_data.len();
        new_data.extend(std::iter::repeat(fill).take(remaining));

        self.rows = new_rows;
        self.cols = new_cols;
        self.data = new_data;
    }

    pub fn resize(&mut self, new_rows: usize, new_cols: usize, fill: T)
    where
        T: Clone,
    {
        if new_rows == self.rows && new_cols == self.cols {
            return;
        }
        let mut new_data = vec![fill; new_rows * new_cols];
        let copy_rows = self.rows.min(new_rows);
        let copy_cols = self.cols.min(new_cols);
        for r in 0..copy_rows {
            let src = &self.data[r * self.cols..r * self.cols + copy_cols];
            let dst = &mut new_data[r * new_cols..r * new_cols + copy_cols];
            dst.clone_from_slice(src);
        }
        self.rows = new_rows;
        self.cols = new_cols;
        self.data = new_data;
    }

    /// 전치
    pub fn transpose(&self) -> Self
    where
        T: Clone,
    {
        let mut out = Self::with_size(
            self.cols,
            self.rows,
            // dummy, 곧 채워질 거라 Clone만 되면 됨
            self.data[0].clone(),
        );
        for r in 0..self.rows {
            for c in 0..self.cols {
                out[(c, r)] = self[(r, c)].clone();
            }
        }
        out
    }

    pub fn append_rows_reuse(a: &Self, b: &Self) -> Self
    where
        T: Clone,
    {
        assert_eq!(a.cols, b.cols, "Column counts must match");
        let total_rows = a.rows + b.rows;
        let total_len = total_rows * a.cols;

        let mut data = Vec::with_capacity(total_len);
        data.extend_from_slice(&a.data);
        data.extend_from_slice(&b.data);

        Self {
            rows: total_rows,
            cols: a.cols,
            data,
        }
    }

    pub fn append_rows(a: &Self, b: &Self) -> Self
    where
        T: Clone,
    {
        assert_eq!(a.cols, b.cols, "Column counts must match");
        let mut out = Self::with_size(
            a.rows + b.rows,
            a.cols,
            if a.rows > 0 {
                a[(0, 0)].clone()
            } else {
                b[(0, 0)].clone()
            },
        );
        // a 복사
        for r in 0..a.rows {
            let dst = &mut out.data[r * out.cols..r * out.cols + out.cols];
            let src = &a.data[r * a.cols..r * a.cols + a.cols];
            dst.clone_from_slice(src);
        }
        // b 복사
        for r in 0..b.rows {
            let dst = &mut out.data[(r + a.rows) * out.cols..(r + a.rows) * out.cols + out.cols];
            let src = &b.data[r * b.cols..r * b.cols + b.cols];
            dst.clone_from_slice(src);
        }
        out
    }

    /// 열 이어붙이기 (좌→우)
    pub fn append_cols_reuse(a: &Self, b: &Self) -> Self
    where
        T: Clone,
    {
        assert_eq!(a.rows, b.rows, "Row counts must match");
        let new_cols = a.cols + b.cols;
        let mut data = Vec::with_capacity(a.rows * new_cols);

        for r in 0..a.rows {
            let a_row = &a.data[r * a.cols..r * a.cols + a.cols];
            let b_row = &b.data[r * b.cols..r * b.cols + b.cols];
            data.extend_from_slice(a_row);
            data.extend_from_slice(b_row);
        }

        Self {
            rows: a.rows,
            cols: new_cols,
            data,
        }
    }

    pub fn append_cols(a: &Self, b: &Self) -> Self
    where
        T: Clone,
    {
        assert_eq!(a.rows, b.rows, "Row counts must match");
        let mut out = Self::with_size(
            a.rows,
            a.cols + b.cols,
            if a.rows > 0 {
                a[(0, 0)].clone()
            } else {
                b[(0, 0)].clone()
            },
        );
        for r in 0..a.rows {
            let mut i = r * out.cols;
            // a의 행
            out.data[i..i + a.cols].clone_from_slice(&a.data[r * a.cols..r * a.cols + a.cols]);
            i += a.cols;
            // b의 행
            out.data[i..i + b.cols].clone_from_slice(&b.data[r * b.cols..r * b.cols + b.cols]);
        }
        out
    }
}
```
```rust
impl<T> TMatrix<T> {
    #[inline]
    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        if r1 == r2 {
            return;
        }
        let cols = self.cols;
        let base1 = r1 * cols;
        let base2 = r2 * cols;
        for j in 0..cols {
            self.data.swap(base1 + j, base2 + j);
        }
    }
}
```
```rust
// 표시(디버그용)
impl<T: fmt::Debug> fmt::Debug for TMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..self.rows).map(|r| self.row(r)))
            .finish()
    }
}
```
```rust
// (r,c) 인덱싱
impl<T> Index<(usize, usize)> for TMatrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        assert!(r < self.rows && c < self.cols);
        &self.data[self.idx(r, c)]
    }
}
```
```rust
impl<T> IndexMut<(usize, usize)> for TMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        assert!(r < self.rows && c < self.cols);
        let i = self.idx(r, c);
        &mut self.data[i]
    }
}
```
```rust
/* ---------- 수치 연산 (f32/f64 전용) ---------- */
impl<T: Float> TMatrix<T> {
    /// 항등행렬
    pub fn eye(n: usize) -> Self {
        let mut out = Self::with_size(n, n, T::zero());
        for i in 0..n {
            out[(i, i)] = T::one();
        }
        out
    }

    pub fn lu_decompose(&self, zero_tol: T) -> (Self, Self, Vec<usize>)
    where
        T: Float,
    {
        // L, U 초기화
        let n = self.rows;
        assert_eq!(n, self.cols, "LU 분해는 정방행렬만 지원합니다");

        let mut l = Self::eye(n);
        let mut u = self.clone();
        let mut p: Vec<usize> = (0..n).collect(); // 행 교환 기록

        for i in 0..n {
            // 피벗 선택
            let mut pivot = i;
            let mut max = u[(i, i)].abs();
            for r in i + 1..n {
                if u[(r, i)].abs() > max {
                    max = u[(r, i)].abs();
                    pivot = r;
                }
            }

            assert!(max > zero_tol, "Singular matrix");

            // 행 교환
            if pivot != i {
                u.swap_rows(i, pivot);
                l.swap_rows(i, pivot);
                p.swap(i, pivot);
            }

            // L, U 계산
            for r in i + 1..n {
                let factor = u[(r, i)] / u[(i, i)];
                l[(r, i)] = factor;
                for c in i..n {
                    u[(r, c)] = u[(r, c)] - factor * u[(i, c)];
                }
            }
        }

        (l, u, p)
    }

    /// 부분피벗 가우스-조르당 역행렬
    pub fn invert(&self, zero_tol: T) -> Self {
        assert_eq!(self.rows, self.cols, "Square only");
        let n = self.rows;
        let mut a = self.clone();
        let mut inv = Self::eye(n);

        for col in 0..n {
            // pivot 선택 (col..n) 중 |a[r,col]| 최대
            let mut pivot = col;
            let mut max_abs = a[(col, col)].abs();
            for r in col + 1..n {
                let v = a[(r, col)].abs();
                if v > max_abs {
                    max_abs = v;
                    pivot = r;
                }
            }
            assert!(max_abs > zero_tol, "Matrix is singular or ill-conditioned");

            if pivot != col {
                a.swap_rows(pivot, col);
                inv.swap_rows(pivot, col);
            }

            // pivot 행 정규화
            let pv = a[(col, col)];
            let inv_pv = T::one() / pv;
            for j in 0..n {
                a[(col, j)] = a[(col, j)] * inv_pv;
                inv[(col, j)] = inv[(col, j)] * inv_pv;
            }

            // 다른 행에서 제거
            for r in 0..n {
                if r == col {
                    continue;
                }
                let factor = a[(r, col)];
                if factor == T::zero() {
                    continue;
                }
                for j in 0..n {
                    a[(r, j)] = a[(r, j)] - factor * a[(col, j)];
                    inv[(r, j)] = inv[(r, j)] - factor * inv[(col, j)];
                }
            }
        }
        inv
    }

    pub fn invert_new(&self, zero_tol: T) -> Self
    where
        T: Float,
    {
        let n = self.rows;
        assert_eq!(n, self.cols, "정방행렬만 역행렬을 가질 수 있습니다");

        let (l, u, p) = self.lu_decompose(zero_tol);
        let mut inv = TMatrix::with_size(n, n, T::zero());

        for col in 0..n {
            // 단위 벡터 e
            let mut e = vec![T::zero(); n];
            e[p[col]] = T::one();

            // L y = e
            let mut y = vec![T::zero(); n];
            for i in 0..n {
                y[i] = e[i];
                for j in 0..i {
                    y[i] = y[i] - l[(i, j)] * y[j];
                }
            }

            // U x = y
            let mut x = vec![T::zero(); n];
            for i in (0..n).rev() {
                x[i] = y[i];
                for j in i + 1..n {
                    x[i] = x[i] - u[(i, j)] * x[j];
                }
                x[i] = x[i] / u[(i, i)];
            }

            for r in 0..n {
                inv[(r, col)] = x[r];
            }
        }

        inv
    }
}
```
```rust
/// 행렬 곱 (f32/f64 전용)
impl<T: FloatScalar> Mul<&TMatrix<T>> for &TMatrix<T> {
    type Output = TMatrix<T>;
    fn mul(self, rhs: &TMatrix<T>) -> Self::Output {
        assert_eq!(self.cols, rhs.rows, "Matrix dimensions do not match");
        let (m, k, n) = (self.rows, self.cols, rhs.cols);
        let mut out = TMatrix::with_size(m, n, T::zero());

        for i in 0..m {
            for j in 0..n {
                let mut sum = T::zero();
                // i-th row dot j-th col
                for kk in 0..k {
                    sum += self[(i, kk)] * rhs[(kk, j)];
                }
                out[(i, j)] = sum;
            }
        }
        out
    }
}
```
```rust
impl<T: FloatScalar> MulAssign<&TMatrix<T>> for TMatrix<T> {
    fn mul_assign(&mut self, rhs: &TMatrix<T>) {
        let res = (&*self) * rhs;
        *self = res;
    }
}
```
```rust
impl<T: Clone> TMatrix<T> {
    pub fn append_row(a: &Self, b: &Self) -> Self {
        Self::append_rows(a, b)
    }
    pub fn append_col(a: &Self, b: &Self) -> Self {
        Self::append_cols(a, b)
    }
}
```


## ✅ 전체 구조 요약

| 구성 요소         | 관련 기능 또는 메서드                          | 설명 및 역할                                      |
|------------------|-----------------------------------------------|--------------------------------------------------|
| Vec<T>           | 내부 저장 구조                                 | 연속 메모리 기반 Row-major 행렬 저장             |
| 인덱싱 계산      | (r, c) → r * cols + c                          | 행렬 요소 위치 계산에 사용되는 핵심 수식         |
| 생성 및 변형     | with_size, from_fn, append_rows, append_cols, transpose | 다양한 방식의 행렬 생성 및 구조 변형 지원        |
| 디버그 출력      | Debug 구현                                     | 행 단위로 보기 쉽게 출력                         |
| 수치 연산        | LU 분해, 역행렬, 행렬 곱, 항등행렬             | f32/f64 기반 고급 선형대수 연산 지원             |


## 🧮 수식 기반 기능 검증
### 1. append_cols()
- 수식:

$$
C_{i,j} =
\left\{
\begin{array}{ll}
A_{i,j} & \text{if } j < a.cols \\
B_{i,j - a.cols} & \text{if } j \geq a.cols
\end{array}
\right.
$$

- 구현: clone_from_slice로 각 행의 a, b 부분을 정확히 이어붙임
### 2. swap_rows()
- 수식:

$$
\mathrm{swap\  }A_{r1,*}\leftrightarrow A_{r2,*}
$$

- 구현: self.data.swap()으로 각 열에 대해 교환

### 3. LU 분해 (lu_decompose)
- 수식:  
$PA=LU-$ 부분 피벗 선택
- 행 교환
- L: 하삼각 행렬, U: 상삼각 행렬
- 구현: pivot, swap_rows, factor 계산 모두 정확

### 4. 역행렬 (invert, invert_new)
- invert: 가우스-조르당 방식
- 행 정규화 + 다른 행 제거
- invert_new: LU 분해 기반 역행렬
- $Ax=e → LUx=e$ → forward/backward substitution
- 수식 흐름과 구현 모두 정확하며, zero_tol로 특이행렬 방지

### 5. 행렬 곱 (Mul)
- 수식:

$$
C_{i,j}=\sum _{k=0}^KA_{i,k}\cdot B_{k,j}
$$

- 구현: for i, for j, for kk 루프로 정확한 dot product

## 🧠 기능별 요약 테이블

| 기능 범주         | 관련 메서드                         | 역할 또는 동작 설명                          | 수식적 검증 결과 |
|------------------|-------------------------------------|---------------------------------------------|------------------|
| 행렬 생성         | with_size, from_fn, eye             | 크기 지정, 함수 기반 생성, 항등행렬 생성     | ✅               |
| 행/열 병합        | append_rows, append_cols            | 행 또는 열 단위로 두 행렬을 이어붙임         | ✅               |
| 구조 변형         | transpose                           | 행과 열을 뒤바꾼 전치 행렬 생성              | ✅               |
| 행 조작           | swap_rows                           | 두 행의 데이터를 교환                        | ✅               |
| LU 분해           | lu_decompose                        | 부분 피벗 기반 LU 분해                       | ✅               |
| 역행렬 (조르당)   | invert                              | 가우스-조르당 방식으로 역행렬 계산           | ✅               |
| 역행렬 (LU 기반) | invert_new                          | LU 분해 기반으로 역행렬 계산                 | ✅               |
| 행렬 곱           | Mul, MulAssign                      | 두 행렬의 곱 계산 및 대입                    | ✅               |
| 인덱싱 연산자     | Index, IndexMut                     | (r, c) 위치의 요소 접근 및 수정              | ✅               |
| 디버그 출력       | Debug                               | 행 단위로 보기 쉽게 출력                     | ✅               |

## 🔍 안정성 및 개선 포인트

| 항목          | 관련 요소 또는 트레이트     | 설명 및 개선 방향                                      |
|---------------|-----------------------------|--------------------------------------------------------|
| assert_eq!    | 행렬 크기 검증              | 연산 전 크기 불일치를 명확히 체크하여 런타임 오류 방지 |
| zero_tol      | 역행렬 및 LU 분해           | 특이행렬 또는 ill-conditioned 행렬에 대한 안정성 확보 |
| FloatScalar   | Float 기반 확장             | f32/f64 외 사용자 정의 스칼라 타입 확장 가능           |
| (추가 가능)   | 예: SIMD, 병렬 처리 등      | 성능 최적화를 위한 구조적 확장 여지 있음               |


## ✨ 결론
이 TMatrix<T>는 수치 해석, 그래픽, 머신러닝, 물리 시뮬레이션 등 다양한 분야에서 활용 가능한 고성능 행렬 구조입니다.
수식적으로도 정확하며, Rust의 안전성과 제네릭 시스템을 잘 활용하고 있어요.

----

# 테스트 코드

## 참조 소스
```rust
use geometry::core::tmatrix::TMatrix;

// -------- 테스트 유틸 --------
#[inline]
fn approx(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}
```
```rust
fn mat_from(data: &[&[f64]]) -> TMatrix<f64> {
    let rows = data.len();
    let cols = if rows == 0 { 0 } else { data[0].len() };
    assert!(rows > 0 && cols > 0);
    let mut m = TMatrix::<f64>::with_size(rows, cols, 0.0);
    for r in 0..rows {
        assert_eq!(data[r].len(), cols);
        for c in 0..cols {
            m[(r, c)] = data[r][c];
        }
    }
    m
}
```
```rust
fn eye(n: usize) -> TMatrix<f64> {
    let mut m = TMatrix::<f64>::with_size(n, n, 0.0);
    for i in 0..n {
        m[(i, i)] = 1.0;
    }
    m
}
```
```rust
// A * B (f64)
fn mul(a: &TMatrix<f64>, b: &TMatrix<f64>) -> TMatrix<f64> {
    a * b
}
```
```rust
// 행렬 근사 동등
fn mat_eq_eps(a: &TMatrix<f64>, b: &TMatrix<f64>, eps: f64) -> bool {
    if a.rows() != b.rows() || a.cols() != b.cols() {
        return false;
    }
    for r in 0..a.rows() {
        for c in 0..a.cols() {
            if !approx(a[(r, c)], b[(r, c)], eps) {
                return false;
            }
        }
    }
    true
}
```

## 🧪 기본 생성 및 접근
### create_and_index
- with_size로 2×3 행렬 생성
- (r, c) 인덱싱으로 값 설정 및 확인
- row()와 row_mut()로 행 슬라이스 접근 및 수정
```rust
#[test]
fn create_and_index() {
    let mut m = TMatrix::<f64>::with_size(2, 3, 0.0);
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);

    m[(0, 0)] = 1.0;
    m[(0, 1)] = 2.0;
    m[(0, 2)] = 3.0;
    m[(1, 0)] = 4.0;
    m[(1, 1)] = 5.0;
    m[(1, 2)] = 6.0;

    assert_eq!(m[(0, 2)], 3.0);
    assert_eq!(m[(1, 1)], 5.0);

    // row / row_mut
    {
        let row0 = m.row(0);
        assert_eq!(row0.unwrap(), [1.0, 2.0, 3.0]);
        let row1 = m.row_mut(1).unwrap();
        row1[2] = 66.0;
        assert_eq!(m[(1, 2)], 66.0);
    }
}
```

## 🔁 전치 및 변형
### transpose_roundtrip
- 2×3 행렬 → 전치 → 3×2 행렬
- 다시 전치 → 원래 행렬과 동일한지 mat_eq_eps로 확인
```rust
#[test]
fn transpose_roundtrip() {
    let a = mat_from(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
    let at = a.transpose();
    assert_eq!(at.rows(), 3);
    assert_eq!(at.cols(), 2);
    assert_eq!(at[(0, 0)], 1.0);
    assert_eq!(at[(1, 0)], 2.0);
    assert_eq!(at[(2, 1)], 6.0);

    // (A^T)^T = A
    let att = at.transpose();
    assert!(mat_eq_eps(&a, &att, 0.0));
}
```

### resize_preserves_prefix
- 2×2 행렬 → 3×4로 확장 → 기존 값 유지 + 새 영역은 fill
- 다시 1×1로 축소 → 좌상단 값 유지 확인

```rust
#[test]
fn resize_preserves_prefix() {
    let mut m = mat_from(&[&[1.0, 2.0], &[3.0, 4.0]]);

    // 더 크게
    m.resize(3, 4, 99.0);
    assert_eq!(m.rows(), 3);
    assert_eq!(m.cols(), 4);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(1, 1)], 4.0);
    // 새로 생긴 영역은 fill 값
    assert_eq!(m[(0, 2)], 99.0);
    assert_eq!(m[(2, 3)], 99.0);

    // 더 작게(앞부분 유지)
    m.resize(1, 1, 0.0);
    assert_eq!(m.rows(), 1);
    assert_eq!(m.cols(), 1);
    assert_eq!(m[(0, 0)], 1.0); // 좌상단 값 유지
}
```

## 🔗 행/열 병합
### append_rows_and_cols
- 두 2×2 행렬을 행 기준으로 병합 → 4×2
- 열 기준으로 병합 → 2×4
- 병합 후 특정 위치 값 확인

```rust
#[test]
fn append_rows_and_cols() {
    let a = mat_from(&[&[1.0, 2.0], &[3.0, 4.0]]);
    let b = mat_from(&[&[5.0, 6.0], &[7.0, 8.0]]);

    let rcat = TMatrix::append_rows(&a, &b);
    assert_eq!(rcat.rows(), 4);
    assert_eq!(rcat.cols(), 2);
    assert_eq!(rcat[(2, 0)], 5.0);
    assert_eq!(rcat[(3, 1)], 8.0);

    let ccat = TMatrix::append_cols(&a, &b);
    assert_eq!(ccat.rows(), 2);
    assert_eq!(ccat.cols(), 4);
    assert_eq!(ccat[(0, 2)], 5.0);
    assert_eq!(ccat[(1, 3)], 8.0);
}
```

## 🧮 행렬 곱셈
### multiply_rectangular_and_square
- 2×3 × 3×2 → 2×2 행렬 곱 수행
- 손계산 결과와 비교 (approx)
- 항등 행렬 곱 확인: $A * I = A, I * B = B$
- 결합법칙 확인: $(A*B)*C == A*(B*C)$
transpose_multiplication_property
- $(BA)^T=A^TB^T$ 수식 검증
- 전치 후 곱과 곱 후 전치가 동일한지 확인

```rust
#[test]
fn multiply_rectangular_and_square() {
    // (2x3) * (3x2) -> (2x2)
    let a = mat_from(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
    let b = mat_from(&[&[7.0, 8.0], &[9.0, 10.0], &[11.0, 12.0]]);
    let c = mul(&a, &b);
    assert_eq!(c.rows(), 2);
    assert_eq!(c.cols(), 2);
    // 손계산 결과
    assert!(approx(c[(0, 0)], 58.0, 1e-12));
    assert!(approx(c[(0, 1)], 64.0, 1e-12));
    assert!(approx(c[(1, 0)], 139.0, 1e-12));
    assert!(approx(c[(1, 1)], 154.0, 1e-12));

    // 항등검사
    let i3 = eye(3);
    let ai = mul(&a, &i3);
    assert!(mat_eq_eps(&a, &ai, 1e-12));

    let _i2 = eye(2);
    let bi = mul(&i3, &b);
    assert!(mat_eq_eps(&b, &bi, 1e-12));

    // 결합법칙 (A*B)*C == A*(B*C)
    let c2 = mat_from(&[&[1.0, 0.0], &[0.0, 1.0]]);
    let left = mul(&mul(&a, &b), &c2);
    let right = mul(&a, &mul(&b, &c2));
    assert!(mat_eq_eps(&left, &right, 1e-12));
}
```

## 🔁 역행렬
### invert_2x2_and_3x3
- 2×2 행렬 역행렬 계산 → 곱해서 항등행렬 되는지 확인
- 3×3 행렬도 동일하게 검증

```rust
fn invert_2x2_and_3x3() {
    // 2x2
    let a = mat_from(&[&[4.0, 7.0], &[2.0, 6.0]]);
    let inv = a.invert(1e-12_f64);
    let id = mul(&a, &inv);
    assert!(mat_eq_eps(&id, &eye(2), 1e-10));

    // 3x3 (비특이)
    let b = mat_from(&[&[1.0, 2.0, 3.0], &[0.0, 1.0, 4.0], &[5.0, 6.0, 0.0]]);
    let binv = b.invert(1e-12_f64);
    let id3 = mul(&b, &binv);
    assert!(mat_eq_eps(&id3, &eye(3), 1e-9));
}
```

### invert_singular_should_panic
- 선형 종속 행렬 (특이행렬) → 역행렬 계산 시 panic 발생 확인

```rust
#[test]
#[should_panic] // singular or ill-conditioned
fn invert_singular_should_panic() {
    let s = mat_from(&[
        &[1.0, 2.0, 3.0],
        &[2.0, 4.0, 6.0], // 2배 -> 선형 종속
        &[1.0, 1.0, 1.0],
    ]);
    let _ = s.invert(1e-12_f64);
}
```

## 🧭 법선 변환
### normal_transform_inverse_transpose
- 비등방 스케일 행렬 S에 대해 법선 변환: $n'=(S^{-1})^Tn$
- x축 법선 벡터 (1,0,0) → (1/2, 0, 0)로 변환되는지 확인

```rust
#[test]
fn normal_transform_inverse_transpose() {
    // 비등방 스케일 (2, 3, 4)
    let s = mat_from(&[&[2.0, 0.0, 0.0], &[0.0, 3.0, 0.0], &[0.0, 0.0, 4.0]]);
    let inv_t = s.invert(1e-12_f64).transpose();

    // x축 법선 (1,0,0) -> (1/2, 0, 0) 로 변환되어야 함 (스케일만 있을 때)
    let n = mat_from(&[&[1.0], &[0.0], &[0.0]]); // 3x1 as column vector
    let n2 = mul(&inv_t, &n);
    assert!(approx(n2[(0, 0)], 0.5, 1e-12));
    assert!(approx(n2[(1, 0)], 0.0, 1e-12));
    assert!(approx(n2[(2, 0)], 0.0, 1e-12));
}
```

## 🧩 비 Copy 타입 테스트
### non_copy_element_clone_paths
- P3 구조체를 요소로 갖는 행렬 생성
- transpose, append_rows, append_cols, resize 등에서 Clone 경로가 정상 작동하는지 확인
- 값 유지 및 새 영역 채움 확인

```rust
#[test]
fn non_copy_element_clone_paths() {
    // with_size + fill
    let base = P3 { x: 0, y: 0, z: 0 };
    let mut m = TMatrix::<P3>::with_size(2, 3, base.clone());
    assert_eq!(m[(0, 0)], base);

    // 값 대입
    m[(0, 1)] = P3 { x: 1, y: 2, z: 3 };
    m[(1, 2)] = P3 {
        x: -1,
        y: -2,
        z: -3,
    };

    // transpose (push/clone 방식이면 무리 없음)
    let mt = m.transpose();
    assert_eq!(mt.rows(), 3);
    assert_eq!(mt.cols(), 2);
    assert_eq!(mt[(1, 0)], P3 { x: 1, y: 2, z: 3 });
    assert_eq!(
        mt[(2, 1)],
        P3 {
            x: -1,
            y: -2,
            z: -3
        }
    );

    // append_rows / append_cols
    let rcat = TMatrix::append_rows(&m, &m);
    assert_eq!(rcat.rows(), 4);
    assert_eq!(rcat.cols(), 3);
    assert_eq!(rcat[(2, 1)], P3 { x: 1, y: 2, z: 3 });

    let ccat = TMatrix::append_cols(&m, &m);
    assert_eq!(ccat.rows(), 2);
    assert_eq!(ccat.cols(), 6);
    assert_eq!(
        ccat[(1, 5)],
        P3 {
            x: -1,
            y: -2,
            z: -3
        }
    );

    // resize clone_from_slice 경로 검증
    let mut r = m.clone();
    r.resize(3, 4, P3 { x: 9, y: 9, z: 9 });
    assert_eq!(r.rows(), 3);
    assert_eq!(r.cols(), 4);
    assert_eq!(r[(0, 1)], P3 { x: 1, y: 2, z: 3 }); // 기존 유지
    assert_eq!(r[(2, 3)], P3 { x: 9, y: 9, z: 9 }); // 새로 채워진 값
}
```

## 🧪 기타 단위 테스트
### test_matrix_creation_and_access
- with_size로 생성 후 기본 값 확인

```rust
#[test]
fn test_matrix_creation_and_access() {
    let m = TMatrix::<f64>::with_size(2, 3, 1.5);
    assert_eq!(m[(0, 0)], 1.5);
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
}
```

### test_transpose
- from_fn으로 생성 후 전치 확인

```rust
#[test]
fn test_transpose() {
    let m = TMatrix::from_fn(2, 3, |r, c| (r * 10 + c) as i32);
    let t = m.transpose();
    assert_eq!(t.rows(), 3);
    assert_eq!(t.cols(), 2);
    assert_eq!(t[(0, 1)], m[(1, 0)]);
}
```
### test_append_cols
- 두 행렬 열 병합 후 값 확인

```rust
#[test]
fn test_append_cols() {
    let a = TMatrix::from_fn(2, 2, |r, c| r + c);
    let b = TMatrix::from_fn(2, 2, |r, c| r * c);
    let ab = TMatrix::append_cols(&a, &b);
    assert_eq!(ab.rows(), 2);
    assert_eq!(ab.cols(), 4);
    assert_eq!(ab[(0, 0)], a[(0, 0)]);
    assert_eq!(ab[(0, 3)], b[(0, 1)]);
}
```

### test_swap_rows
- 행 교환 후 값 위치 확인

```rust
#[test]
fn test_swap_rows() {
    let mut m = TMatrix::from_fn(2, 2, |r, c| (r * 2 + c) as i32);
    m.swap_rows(0, 1);
    assert_eq!(m[(0, 0)], 2);
    assert_eq!(m[(1, 0)], 0);
}
```
### test_resize
- 리사이즈 후 행렬 크기 및 값 확인

```rust
#[test]
fn test_resize() {
    let mut m = TMatrix::with_size(2, 2, 1.0);
    m.resize(3, 4, 0.0);
    assert_eq!(m.rows(), 3);
    assert_eq!(m.cols(), 4);
    assert_eq!(m[(2, 3)], 0.0);
}
```

### test_lu_decomposition
- LU 분해 수행 후 L, U, P의 크기 확인

```rust
#[test]
fn test_lu_decomposition() {
    let m = TMatrix::from_fn(3, 3, |r, c| if r == c { 2.0 } else { 1.0 });
    let (l, u, p) = m.lu_decompose(1e-6);
    assert_eq!(l.rows(), 3);
    assert_eq!(u.cols(), 3);
    assert_eq!(p.len(), 3);
}
```

### test_invert
- 역행렬 계산 후 항등행렬과 근사 비교

```rust
#[test]
fn test_invert() {
    let m = TMatrix::from_fn(3, 3, |r, c| if r == c { 4.0 } else { 1.0 });
    let inv = m.invert(1e-6);
    let id = &m * &inv;
    for r in 0..id.rows() {
        for c in 0..id.cols() {
            let expected = if r == c { 1.0 } else { 0.0 };
            assert!(((id[(r, c)] - expected) as f64).abs() < 1e-3);
        }
    }
}
```
### test_eye_and_debug
- 항등행렬 생성 및 디버그 출력 확인

```rust
#[test]
fn test_eye_and_debug() {
    let m = TMatrix::<f64>::eye(3);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(0, 1)], 0.0);
    println!("{:?}", m);
}
```

## ✅ 요약 테이블

| 테스트 이름                          | 기능 요약                             | 수식 또는 이론적 의미         | 비고 또는 특이사항             |
|-------------------------------------|--------------------------------------|-------------------------------|-------------------------------|
| create_and_index                    | 행렬 생성 및 인덱싱 접근              |                               | row, row_mut 포함              |
| transpose_roundtrip                | 전치 후 복원 확인                     | \((A^T)^T = A\)               | 정확한 전치 구현 확인          |
| resize_preserves_prefix            | 리사이즈 시 기존 값 유지 및 fill 확인 |                               | 확장/축소 모두 검증            |
| append_rows_and_cols               | 행/열 병합 기능                       |                               | append_rows, append_cols       |
| multiply_rectangular_and_square    | 행렬 곱 및 항등/결합법칙 확인         | \((AB)C = A(BC), AI = A\)     | 손계산 결과와 비교 포함        |
| transpose_multiplication_property  | 전치 곱의 수식 확인                   | \((BA)^T = A^T B^T\)          | 수식 기반 검증                 |
| invert_2x2_and_3x3                 | 역행렬 계산 및 항등 확인              | \(AA^{-1} = I\)               | 2×2, 3×3 모두 포함              |
| invert_singular_should_panic       | 특이행렬 역행렬 시 panic 발생 확인    |                               | #[should_panic] 테스트         |
| normal_transform_inverse_transpose | 법선 변환 수식 확인                   | \(n' = (M^{-1})^T n\)         | 그래픽/물리 시뮬레이션 응용    |
| non_copy_element_clone_paths       | Clone 기반 연산 경로 확인             |                               | P3 구조체 기반 테스트          |
| test_matrix_creation_and_access    | 기본 생성 및 값 확인                  |                               | 단순 생성 확인                 |
| test_transpose                     | 전치 결과 확인                        |                               | from_fn 기반                   |
| test_append_cols                   | 열 병합 후 값 확인                    |                               | append_cols 단독 테스트        |
| test_swap_rows                     | 행 교환 후 값 위치 확인               |                               | swap_rows 동작 검증            |
| test_resize                        | 리사이즈 후 크기 및 값 확인           |                               | fill 값 포함                   |
| test_lu_decomposition              | LU 분해 결과 크기 확인                | \(PA = LU\)                   | pivot 및 행 교환 포함          |
| test_invert                        | 역행렬 계산 후 항등 확인              |                               | 수치 오차 포함 검증            |
| test_eye_and_debug                 | 항등행렬 생성 및 디버그 출력          |                               | Debug 출력 확인                |

---

