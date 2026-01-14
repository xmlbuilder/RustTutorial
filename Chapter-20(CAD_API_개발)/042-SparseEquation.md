# Sparse Matrix
- 이 소스는 희소 행렬(Sparse Matrix)을 다루기 위한 선형대수 연산용 데이터 구조 및 알고리즘을 정의한 것입니다.  
- 특히 SparseEquation 희소 벡터(또는 행렬의 한 행)를 표현하며,  
- SparseCoefficient 그 안의 개별 항(열 인덱스와 값)을 나타냅니다.

## 📦 전체 구조 요약
### 1. SparseCoefficient
```rust
#[derive(Debug, Clone, Copy)]
pub struct SparseCoefficient {
    pub pos: usize, // 열 인덱스
    pub val: f64,   // 해당 열의 값
}
```
- 하나의 항을 나타냅니다: A[i][j] = val이면 pos = j, val = A[i][j]
- PartialEq은 pos만 비교 → 정렬/병합 시 유용

### 2. SparseEquation
```rust
#[derive(Debug, Default, Clone)]
pub struct SparseEquation {
    terms: Vec<SparseCoefficient>, // 항상 pos 오름차순 유지
}
```
- 희소 벡터 또는 행렬의 한 행을 표현
- 내부적으로 terms는 정렬된 SparseCoefficient 리스트

## 🧩 주요 메서드 설명

| 메서드 이름               | 설명                                                                 |
|---------------------------|----------------------------------------------------------------------|
| `add(pos, val)`           | 항을 추가하거나 기존 pos가 있으면 값을 누적 (정렬 유지 및 병합)     |
| `remove_at_shift(pos)`    | 해당 열을 제거하고 이후 열 인덱스를 1씩 감소 (열 삭제 시 사용)       |
| `get(pos)`                | 특정 열 인덱스에 해당하는 항을 참조로 반환 (없으면 None)             |
| `terms()`                 | 현재 항 목록을 읽기 전용 슬라이스로 반환                            |
| `dot_dense(x)`            | 밀집 벡터 `x`와의 내적 계산: ∑ valᵢ * x[posᵢ]                        |
| `dot_csr(row, i, j, a, x)`| CSR 포맷의 한 행과 벡터 `x`의 내적 계산 (정적 함수)                  |


## 🧮 주요 연산 함수
### 1. on_merge_sparse_equation(a, b)
- 희소 행렬 A, B의 곱: C = A × B
- 각 행 i에 대해 C[i][j] += A[i][k] * B[k][j] 수행
### 2. on_multiply_diagonal_accum(a, b, n_size)
- 대각선 누적 곱: C[i][k] += A[i][p] * B[p][j]에서 k = j - i + center
- 대각선 기반의 구조적 패턴 분석에 유용 (예: convolution, band matrix)
### 3. on_eqs_to_dense(rows, n_cols)
- 희소 행렬 → 밀집 행렬 변환
- 디버깅, 시각화, 외부 라이브러리 연동 시 사용
### 4. on_dense_mul(a, b)
- 밀집 행렬 곱: C = A × B
- 희소 연산과 비교용 또는 검증용
### 5. on_transpose_sparse_equations(n, m, equations)
- 희소 행렬 전치: Aᵗ[i][j] = A[j][i]
- 열 기반 연산 또는 좌변/우변 전환 시 사용

## 🧪 사용 예시
```rust
let mut eq = SparseEquation::new();
eq.add(0, 2.0);
eq.add(3, -1.0);
let x = vec![1.0, 0.0, 0.0, 4.0];
let result = eq.dot_dense(&x); // 2.0*1.0 + (-1.0)*4.0 = -2.0
```

## 🧠 용도 및 활용 분야
| 분야             | 활용 방식                                                   | 예시 또는 목적                         |
|------------------|-------------------------------------------------------------|----------------------------------------|
| 선형 시스템 해석 | 희소 행렬 곱셈, 내적, 전치, 대각선 누적                     | Ax = b 해법, LU 분해, CG solver        |
| FEM/CFD/해석     | 구조 해석, 행렬 조립, 대각선 기반 패턴 분석                | 유한요소법, 열/유체 시뮬레이션         |
| 머신러닝         | 희소 특성 벡터 내적, 전치 변환                              | 텍스트 분류, 추천 시스템               |
| 그래픽스/시뮬레이션 | 행렬 기반 변환, 희소 연산 최적화                         | 물리 기반 애니메이션, 변환 행렬 적용   |
| 최적화/계획 문제 | 희소 제약 조건 표현 및 연산                                 | 선형계획법, 제약 만족 문제             |
| 데이터 분석      | 희소 행렬 → 밀집 행렬 변환, 구조 시각화                    | PCA, 행렬 시각화, 구조 디버깅          |

---

## 소스

```rust
#[derive(Debug, Clone, Copy)]
pub struct SparseCoefficient {
    pub pos: usize,
    pub val: f64,
}
```
```rust
// Note: If you only want to compare "same column" semantically, use PartialEq based on pos.
// (Use only in sort/merge, and manage it in the public API to avoid confusion.)
impl PartialEq for SparseCoefficient {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
```
```rust
impl Eq for SparseCoefficient {}
```
```rust
#[derive(Debug, Default, Clone)]
pub struct SparseEquation {
    terms: Vec<SparseCoefficient>, // Always keep pos in ascending order
}
```
```rust
impl SparseEquation {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            terms: Vec::with_capacity(n),
        }
    }

    /// Maintain sorting + merge same pos
    pub fn add(&mut self, pos: usize, val: f64) {
        match self.terms.binary_search_by_key(&pos, |c| c.pos) {
            Ok(i) => {
                self.terms[i].val += val;
                // Optionally remove very small values to 0 if necessary
                // if self.terms[i].val.abs() < 1e-20 { self.terms.remove(i); }
            }
            Err(i) => self.terms.insert(i, SparseCoefficient { pos, val }),
        }
    }

    /// Remove the column (pos) and increment all indices of columns greater than it by 1.
    /// (Preserves the original C# RemoveAt intent + off-by-one/fixes omissions)
    pub fn remove_at_shift(&mut self, pos: usize) {
        match self.terms.binary_search_by_key(&pos, |c| c.pos) {
            Ok(i) => {
                // Remove the term corresponding to pos
                self.terms.remove(i);
                // All pos in the back row are -1
                for t in &mut self.terms[i..] {
                    t.pos -= 1;
                }
            }
            Err(_insertion_point) => {
                // Even if the column doesn't exist, all larger columns are -1 (renumber columns by deleting variables)
                for t in &mut self.terms {
                    if t.pos > pos {
                        t.pos -= 1;
                    }
                }
                // Sort is maintained
            }
        }
    }

    /// Get the item corresponding to pos as a reference.
    pub fn get(&self, pos: usize) -> Option<&SparseCoefficient> {
        self.terms
            .binary_search_by_key(&pos, |c| c.pos)
            .ok()
            .map(|i| &self.terms[i])
    }

    /// Expose the current items as a (pos,val) slice (read-only)
    pub fn terms(&self) -> &[SparseCoefficient] {
        &self.terms
    }

    /// Sparse-dense inner product: sum_i val_i * x[pos_i]
    pub fn dot_dense(&self, x: &[f64]) -> f64 {
        let mut acc = 0.0;
        for c in &self.terms {
            acc += c.val * x[c.pos];
        }
        acc
    }

    /// Dot product in a single row in CSR format: for row r, sum_{k=I[r]..I[r+1]-1} A[k]*x[J[k]]
    pub fn dot_csr(row: usize, i: &[usize], j: &[usize], a: &[f64], x: &[f64]) -> f64 {
        assert!(row + 1 < i.len(), "CSR: I must have len >= rows+1");
        let (s, e) = (i[row], i[row + 1]);
        assert!(e <= a.len() && e <= j.len(), "CSR: A/J length mismatch");

        let mut acc = 0.0;
        for k in s..e {
            acc += a[k] * x[j[k]];
        }
        acc
    }
}
```
```rust
pub fn on_merge_sparse_equation(a: &[SparseEquation], b: &[SparseEquation]) -> Vec<SparseEquation> {
    let n = a.len();
    assert_eq!(b.len(), n, "A, B must have same size (square)");
    let mut out = vec![SparseEquation::with_capacity(0); n];
    if n != b.len() {
        return out;
    }

    for i in 0..n {
        let mut row_out = SparseEquation::new();
        // A[i, *]의 항들만 순회
        for c1 in a[i].terms() {
            let brow = &b[c1.pos]; // B의 (row = c1.pos)
            // B의 해당 행의 항들만 순회하여 병합
            for c2 in brow.terms() {
                // C[i, c2.pos] += A[i, c1.pos] * B[c1.pos, c2.pos]
                row_out.add(c2.pos, c1.val * c2.val);
            }
        }
        out[i] = row_out;
    }
    out
}
```
```rust
pub fn on_multiply_diagonal_accum(a: &[SparseEquation], b: &[SparseEquation], n_size: usize) -> Vec<Vec<f64>> {
    let n = a.len();
    assert_eq!(b.len(), n, "A, B must have same size (square)");
    let num = n_size / 2;
    let mut out = vec![vec![0.0; n_size]; n];
    if n != b.len() {
        return out;
    }

    for i in 0..n {
        for c1 in a[i].terms() {
            let brow = &b[c1.pos];
            for c2 in brow.terms() {
                let pos = c2.pos; // B의 열
                // 대각선 오프셋 인덱스
                let k = (pos as isize - i as isize + num as isize) as isize;
                if 0 <= k && (k as usize) < n_size {
                    out[i][k as usize] += c1.val * c2.val;
                }
            }
        }
    }
    out
}
```
```rust
pub fn on_eqs_to_dense(rows: &[SparseEquation], n_cols: usize) -> Vec<Vec<f64>> {
    let n_rows = rows.len();
    let mut m = vec![vec![0.0; n_cols]; n_rows]; // <-- 행은 rows.len(), 열은 ncols
    for (i, eq) in rows.iter().enumerate() {
        for c in eq.terms() {
            if c.pos < n_cols {
                m[i][c.pos] = c.val;
            } else {
                panic!("eqs_to_dense: column {} out of n_cols {}", c.pos, n_cols);
            }
        }
    }
    m
}
```
```rust
pub fn on_dense_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut c = vec![vec![0.0; n]; n];
    for i in 0..n {
        for k in 0..n {
            let aik = a[i][k];
            if aik == 0.0 {
                continue;
            }
            for j in 0..n {
                c[i][j] += aik * b[k][j];
            }
        }
    }
    c
}
```
```rust
pub fn on_transpose_sparse_equations(n: usize, m: usize, equations: &[SparseEquation]) -> Vec<SparseEquation> {
    assert_eq!(
        equations.len(),
        m,
        "transpose: equations.len() must equal m"
    );

    let mut out = (0..n)
        .map(|_| SparseEquation::with_capacity(0))
        .collect::<Vec<_>>();

    for row in 0..m {
        for c in equations[row].terms() {
            // 전치: (row, c.pos, c.val)  →  out[c.pos].add(row, c.val)
            assert!(c.pos < n, "transpose: column {} >= n {}", c.pos, n);
            out[c.pos].add(row, c.val);
        }
    }
    out
}
```

---

# 테스트


## ✅ SparseEquation 전체 테스트 요약

| 테스트 이름                                      | 검증 내용 및 목적                                                   | 희소 연산 의미                          | 통과 여부 |
|--------------------------------------------------|----------------------------------------------------------------------|-----------------------------------------|-----------|
| add_merges_same_pos_and_keeps_sorted             | 동일 pos 항 병합 및 정렬 유지 확인                                  | 희소 벡터 항 추가 및 병합              | ✅        |
| remove_at_shift_existing_pos                     | 존재하는 항 제거 후 이후 pos -1 적용                                | 열 제거 후 희소 구조 재정렬            | ✅        |
| remove_at_shift_missing_pos_still_shifts_bigger | 존재하지 않는 pos 제거 시 이후 항만 이동                            | 열 삭제 시 누락 없이 재정렬            | ✅        |
| get_returns_some_when_present_none_when_absent   | 특정 pos 항 조회 가능 여부 확인                                     | 희소 항 접근 (존재/비존재)             | ✅        |
| dot_dense_is_correct                             | 밀집 벡터와의 내적 계산 정확성 확인                                 | 희소 벡터 · 밀집 벡터 내적             | ✅        |
| dot_dense_matches_csharp_meaning                 | C# 의미와 동일한 내적 결과 확인                                     | 희소 벡터 · 밀집 벡터 내적             | ✅        |
| dot_csr_is_correct                               | CSR 포맷 기반 희소 행렬-벡터 곱 정확성 확인                         | CSR 행 내적                             | ✅        |
| dot_csr_row_matches_standard_csr                 | CSR 포맷과 표준 행렬 곱 결과 비교                                   | CSR 행 내적                             | ✅        |
| test_equation_solver2x2                          | 부분 열만 추출하여 2x2 선형 시스템 해석                              | 희소 행에서 필요한 항만 추출하여 해석  | ✅        |
| test_merge_equation_matches_dense_product        | 희소 행렬 곱셈 결과가 밀집 행렬 곱과 일치하는지 확인               | Sparse × Sparse = Sparse                | ✅        |
| test_multiply_diagonal_accum_matches_folded_dense| 대각선 누적 희소 곱이 밀집 행렬 접기 결과와 일치하는지 확인        | 대각선 기반 희소 누적 곱                | ✅        |
| transpose_square                                 | 정사각 희소 행렬 전치 결과 확인                                     | Sparse Transpose                        | ✅        |
| transpose_rectangular                            | 직사각 희소 행렬 전치 결과 확인                                     | Sparse Transpose (m ≠ n)                | ✅        |



## 🧩 SparseEquation 함수 설명

| 함수 이름                        | 설명                                                                 | 희소 관점에서의 역할                     |
|----------------------------------|----------------------------------------------------------------------|------------------------------------------|
| `add(pos, val)`                  | 항 추가 또는 병합 (동일 pos 존재 시 덧셈)                            | 0이 아닌 항만 저장하는 희소 구조 유지    |
| `remove_at_shift(pos)`          | 해당 열 제거 후 이후 pos 항들 -1                                     | 열 삭제 시 희소 구조 재정렬              |
| `get(pos)`                      | 특정 열 인덱스 항 조회                                               | 희소 벡터에서 항 직접 접근               |
| `terms()`                       | 전체 항 슬라이스 반환 (읽기 전용)                                    | 희소 벡터의 내부 구조 노출               |
| `dot_dense(x)`                  | 밀집 벡터와의 내적 계산                                              | 희소 벡터 · 밀집 벡터 내적               |
| `dot_csr(row, i, j, a, x)`      | CSR 포맷 기반 희소 행렬-벡터 곱 계산                                 | CSR 행 내적                              |
| `merge_sparse_equation(a, b)`   | 희소 행렬 곱셈 A × B                                                 | 희소 행렬 간 곱셈                        |
| `multiply_diagonal_accum(a, b)` | 희소 행렬 곱셈 결과를 대각선 기준으로 누적                          | 구조적 패턴 분석, convolution 등         |
| `eqs_to_dense(rows, n_cols)`    | 희소 행렬 → 밀집 행렬 변환                                          | 디버깅, 시각화, 외부 연동용              |
| `dense_mul(a, b)`               | 밀집 행렬 곱셈                                                       | 희소 연산 결과 검증용                    |
| `transpose_sparse_equations(n,m,equations)` | 희소 행렬 전치 구현                                      | 열 기반 연산, 좌변/우변 전환             |

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::maths::on_solve_2x2;
    use nurbslib::core::sparse_equation::{on_dense_mul, on_eqs_to_dense, on_merge_sparse_equation,
        on_multiply_diagonal_accum, on_transpose_sparse_equations, SparseCoefficient, SparseEquation};

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn add_merges_same_pos_and_keeps_sorted() {
        let mut eq = SparseEquation::new();
        eq.add(3, 2.0);
        eq.add(1, 5.0);
        eq.add(3, 0.5); // 병합되어 pos=3 항의 값이 2.5가 됨
        eq.add(2, -1.0);

        let terms = eq.terms();
        let poses: Vec<_> = terms.iter().map(|c| c.pos).collect();
        let vals: Vec<_> = terms.iter().map(|c| c.val).collect();

        assert_eq!(poses, vec![1, 2, 3], "positions must be sorted and unique");
        assert!(approx(vals[0], 5.0, 1e-12));
        assert!(approx(vals[1], -1.0, 1e-12));
        assert!(approx(vals[2], 2.5, 1e-12));
    }
```
```rust
    #[test]
    fn remove_at_shift_existing_pos() {
        // 초기:  (1:5.0), (2:-1.0), (3:2.5)
        let mut eq = SparseEquation::new();
        eq.add(1, 5.0);
        eq.add(2, -1.0);
        eq.add(3, 2.5);

        // pos=2 제거 → (2:-1.0) 삭제, pos>2 들은 -1 → (1:5.0),(2:2.5)
        eq.remove_at_shift(2);

        let terms = eq.terms();
        let poses: Vec<_> = terms.iter().map(|c| c.pos).collect();
        let vals: Vec<_> = terms.iter().map(|c| c.val).collect();

        assert_eq!(poses, vec![1, 2]);
        assert!(approx(vals[0], 5.0, 1e-12));
        assert!(approx(vals[1], 2.5, 1e-12));
    }
```
```rust
    #[test]
    fn remove_at_shift_missing_pos_still_shifts_bigger() {
        // 초기: (1:1.0), (3:3.0), (5:5.0)
        let mut eq = SparseEquation::new();
        eq.add(1, 1.0);
        eq.add(3, 3.0);
        eq.add(5, 5.0);

        // pos=2 제거: 실제로 pos=2 항은 없지만, pos>2 인 것들(3,5)은 각각 2,4로 -1
        eq.remove_at_shift(2);

        let terms = eq.terms();
        let poses: Vec<_> = terms.iter().map(|c| c.pos).collect();
        let vals: Vec<_> = terms.iter().map(|c| c.val).collect();

        assert_eq!(poses, vec![1, 2, 4]);
        assert!(approx(vals[0], 1.0, 1e-12));
        assert!(approx(vals[1], 3.0, 1e-12));
        assert!(approx(vals[2], 5.0, 1e-12));
    }
```
```rust
    #[test]
    fn get_returns_some_when_present_none_when_absent() {
        let mut eq = SparseEquation::new();
        eq.add(4, 10.0);
        eq.add(7, -2.0);

        let c4 = eq.get(4).copied();
        let c6 = eq.get(6).copied();

        assert!(matches!(c4, Some(SparseCoefficient { pos: 4, val: v }) if approx(v, 10.0, 1e-12)));
        assert!(c6.is_none());
    }
```
```rust
    #[test]
    fn dot_dense_is_correct() {
        // eq: 2*x1 + (-1)*x3 + 0.5*x5
        let mut eq = SparseEquation::new();
        eq.add(1, 2.0);
        eq.add(3, -1.0);
        eq.add(5, 0.5);

        let x = vec![0.0, 10.0, 0.0, 1.5, 0.0, 8.0];
        let y = eq.dot_dense(&x);
        // 2*10 + (-1)*1.5 + 0.5*8 = 20 - 1.5 + 4 = 22.5
        assert!(approx(y, 22.5, 1e-12));
    }
```
```rust
    #[test]
    fn dot_csr_is_correct() {
        // 3x3 예: 행별로 테스트
        // A = [[2, 0, 1],
        //      [0, 3, 0],
        //      [4, 0, 5]]
        // CSR:
        // I = [0, 2, 3, 5]
        // J = [0, 2, 1, 0, 2]
        // V = [2, 1, 3, 4, 5]
        let i = vec![0, 2, 3, 5];
        let j = vec![0, 2, 1, 0, 2];
        let a = vec![2.0, 1.0, 3.0, 4.0, 5.0];
        let x = vec![1.0, 2.0, 3.0];

        // row0: 2*x0 + 1*x2 = 2*1 + 1*3 = 5
        let y0 = SparseEquation::dot_csr(0, &i, &j, &a, &x);
        // row1: 3*x1 = 6
        let y1 = SparseEquation::dot_csr(1, &i, &j, &a, &x);
        // row2: 4*x0 + 5*x2 = 4*1 + 5*3 = 19
        let y2 = SparseEquation::dot_csr(2, &i, &j, &a, &x);

        assert!(approx(y0, 5.0, 1e-12));
        assert!(approx(y1, 6.0, 1e-12));
        assert!(approx(y2, 19.0, 1e-12));
    }
```
```rust
    #[test]
    fn test_equation_solver2x2() {
        // 첫 번째 제약식: -1·x_A + 1·x_B = 10
        let mut eq1 = SparseEquation::new();
        eq1.add(0, -1.0); // x_A
        eq1.add(1, 1.0); // x_B
        let d0 = 10.0;

        // 두 번째 제약식: -1·x_B + 1·x_C = 5
        let mut eq2 = SparseEquation::new();
        eq2.add(1, -1.0); // x_B
        eq2.add(2, 1.0); // x_C
        let d1 = 5.0;

        // 2x2 시스템으로 축소: x_B와 x_C만 해석한다고 가정
        let m00 = eq1.get(1).map_or(0.0, |c| c.val); // x_B in eq1
        let m01 = 0.0; // x_C not in eq1
        let m10 = eq2.get(1).map_or(0.0, |c| c.val); // x_B in eq2
        let m11 = eq2.get(2).map_or(0.0, |c| c.val); // x_C in eq2

        let result = on_solve_2x2(m00, m01, m10, m11, d0, d1);
        println!("해석 결과: {:?}", result);
        // 해석 결과: Solve2x2Result {
        //     rank: 2,
        //     x: 10.0, // x_B
        //     y: 15.0, // x_C
        //     pivot_ratio: 1.0
        // }
    }
```
```rust
    #[test]
    fn dot_dense_matches_csharp_meaning() {
        // eq: 2*x1 + (-1)*x3 + 0.5*x5
        let mut eq = SparseEquation::new();
        eq.add(1, 2.0);
        eq.add(3, -1.0);
        eq.add(5, 0.5);

        let x = vec![0.0, 10.0, 0.0, 1.5, 0.0, 8.0];
        // 2*10 + (-1)*1.5 + 0.5*8 = 22.5
        assert!((eq.dot_dense(&x) - 22.5).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn dot_csr_row_matches_standard_csr() {
        // A = [[2,0,1],
        //      [0,3,0],
        //      [4,0,5]]
        // CSR:
        // I = [0, 2, 3, 5]
        // J = [0, 2, 1, 0, 2]
        // V = [2, 1, 3, 4, 5]
        let i = vec![0, 2, 3, 5];
        let j = vec![0, 2, 1, 0, 2];
        let a = vec![2.0, 1.0, 3.0, 4.0, 5.0];
        let x = vec![1.0, 2.0, 3.0];

        // row0: 2*x0 + 1*x2 = 5
        // row1: 3*x1 = 6
        // row2: 4*x0 + 5*x2 = 19
        assert!((SparseEquation::dot_csr(0, &i, &j, &a, &x) - 5.0).abs() < 1e-12);
        assert!((SparseEquation::dot_csr(1, &i, &j, &a, &x) - 6.0).abs() < 1e-12);
        assert!((SparseEquation::dot_csr(2, &i, &j, &a, &x) - 19.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_merge_equation_matches_dense_product() {
        // A (3x3) 희소
        // [0 2 0]
        // [1 0 0]
        // [0 3 4]
        let mut a0 = SparseEquation::new();
        a0.add(1, 2.0);
        let mut a1 = SparseEquation::new();
        a1.add(0, 1.0);
        let mut a2 = SparseEquation::new();
        a2.add(1, 3.0);
        a2.add(2, 4.0);
        let a = vec![a0, a1, a2];

        // B (3x3) 희소
        // [5 0 0]
        // [0 6 7]
        // [0 0 8]
        let mut b0 = SparseEquation::new();
        b0.add(0, 5.0);
        let mut b1 = SparseEquation::new();
        b1.add(1, 6.0);
        b1.add(2, 7.0);
        let mut b2 = SparseEquation::new();
        b2.add(2, 8.0);
        let b = vec![b0, b1, b2];

        let c_sparse = on_merge_sparse_equation(&a, &b);
        let n = a.len();
        let c_dense_from_sparse = on_eqs_to_dense(&c_sparse, n);

        let a_dense = on_eqs_to_dense(&a, n);
        let b_dense = on_eqs_to_dense(&b, n);
        let c_dense = on_dense_mul(&a_dense, &b_dense);

        for i in 0..n {
            for j in 0..n {
                assert!(
                    approx(c_dense_from_sparse[i][j], c_dense[i][j], 1e-12),
                    "C[{},{}]: sparse={} dense={}",
                    i,
                    j,
                    c_dense_from_sparse[i][j],
                    c_dense[i][j]
                );
            }
        }
    }
```
```rust
    #[test]
    fn test_multiply_diagonal_accum_matches_folded_dense() {
        // 동일 A, B
        let n = 3;

        let mut a0 = SparseEquation::new();
        a0.add(1, 2.0);
        let mut a1 = SparseEquation::new();
        a1.add(0, 1.0);
        let mut a2 = SparseEquation::new();
        a2.add(1, 3.0);
        a2.add(2, 4.0);
        let a = vec![a0, a1, a2];

        let mut b0 = SparseEquation::new();
        b0.add(0, 5.0);
        let mut b1 = SparseEquation::new();
        b1.add(1, 6.0);
        b1.add(2, 7.0);
        let mut b2 = SparseEquation::new();
        b2.add(2, 8.0);
        let b = vec![b0, b1, b2];

        let n_size = 2 * n + 1; // 넉넉히 (대각선 오프셋이 [-n..+n]을 덮도록)
        let acc = on_multiply_diagonal_accum(&a, &b, n_size);

        // Dense C를 만들어서 대각선 오프셋으로 접어(accumulate) 비교
        let ad = on_eqs_to_dense(&a, n);
        let bd = on_eqs_to_dense(&b, n);
        let cd = on_dense_mul(&ad, &bd);

        let num = n_size / 2;
        let mut expect = vec![vec![0.0; n_size]; n];
        for i in 0..n {
            for j in 0..n {
                let k = (j as isize - i as isize + num as isize) as isize;
                if 0 <= k && (k as usize) < n_size {
                    expect[i][k as usize] += cd[i][j];
                }
            }
        }

        for i in 0..n {
            for k in 0..n_size {
                assert!(
                    approx(acc[i][k], expect[i][k], 1e-12),
                    "acc[{},{}]: got={} expect={}",
                    i,
                    k,
                    acc[i][k],
                    expect[i][k]
                );
            }
        }
    }
```
```rust
    #[test]
    fn transpose_square() {
        // A (3x3):
        // [0 2 0]
        // [1 0 3]
        // [0 0 4]
        let mut r0 = SparseEquation::new();
        r0.add(1, 2.0);
        let mut r1 = SparseEquation::new();
        r1.add(0, 1.0);
        r1.add(2, 3.0);
        let mut r2 = SparseEquation::new();
        r2.add(2, 4.0);
        let a = vec![r0, r1, r2];

        let at = on_transpose_sparse_equations(3, 3, &a);

        // A^T 기대값:
        // [0 1 0]
        // [2 0 0]
        // [0 3 4]
        let at_dense = on_eqs_to_dense(&at, 3);
        assert_eq!(at_dense[0], vec![0.0, 1.0, 0.0]);
        assert_eq!(at_dense[1], vec![2.0, 0.0, 0.0]);
        assert_eq!(at_dense[2], vec![0.0, 3.0, 4.0]);
    }
```
```rust
    #[test]
    fn transpose_rectangular() {
        // A (m=2, n=4):
        // [ 5 0 0 7 ]
        // [ 0 6 0 0 ]
        let mut r0 = SparseEquation::new();
        r0.add(0, 5.0);
        r0.add(3, 7.0);
        let mut r1 = SparseEquation::new();
        r1.add(1, 6.0);
        let a = vec![r0, r1];

        // A^T (4x2)
        let at = on_transpose_sparse_equations(4, 2, &a);
        let at_dense = on_eqs_to_dense(&at, 2);

        // 기대:
        // [5 0]
        // [0 6]
        // [0 0]
        // [7 0]
        assert_eq!(at_dense[0], vec![5.0, 0.0]);
        assert_eq!(at_dense[1], vec![0.0, 6.0]);
        assert_eq!(at_dense[2], vec![0.0, 0.0]);
        assert_eq!(at_dense[3], vec![7.0, 0.0]);
    }
}
```

---

# 테스트 코드 분석

- 이 테스트는 희소 행렬의 CSR(Compressed Sparse Row) 포맷을 사용하여 행렬과 벡터의 내적을 계산하는 과정을 검증하는 것입니다.  
- 아래에 단계적으로 상세히 설명.

```rust
#[test]
fn dot_csr_row_matches_standard_csr() {
    // A = [[2,0,1],
    //      [0,3,0],
    //      [4,0,5]]
    // CSR:
    // I = [0, 2, 3, 5]
    // J = [0, 2, 1, 0, 2]
    // V = [2, 1, 3, 4, 5]
    let i = vec![0, 2, 3, 5];
    let j = vec![0, 2, 1, 0, 2];
    let a = vec![2.0, 1.0, 3.0, 4.0, 5.0];
    let x = vec![1.0, 2.0, 3.0];

    // row0: 2*x0 + 1*x2 = 5
    // row1: 3*x1 = 6
    // row2: 4*x0 + 5*x2 = 19
    assert!((SparseEquation::dot_csr(0, &i, &j, &a, &x) - 5.0).abs() < 1e-12);
    assert!((SparseEquation::dot_csr(1, &i, &j, &a, &x) - 6.0).abs() < 1e-12);
    assert!((SparseEquation::dot_csr(2, &i, &j, &a, &x) - 19.0).abs() < 1e-12);
}
```

## 🧠 목적 요약
### 이 테스트는 다음을 검증합니다:
- CSR 포맷으로 표현된 희소 행렬 A와 벡터 x의 곱 y=Ax
- 각 행에 대해 내적 결과가 정확히 계산되는지 확인

## 📦 CSR 포맷 구성 요소
CSR 포맷은 희소 행렬을 다음 3개의 배열로 표현합니다:
| 배열 이름 | 의미                                 | 설명                                                                 |
|-----------|--------------------------------------|----------------------------------------------------------------------|
| `I`       | Row Pointer                          | 각 행의 시작 인덱스를 나타내는 배열 (길이 = 행 개수 + 1)             |
| `J`       | Column Indices                       | 각 항의 열 인덱스를 저장한 배열 (길이 = 비영(非零) 항 개수)         |
| `A`       | Non-zero Values                      | 실제 값이 있는 항들만 저장한 배열 (길이 = 비영 항 개수)             |

### 예시 행렬 A (3×3):
```
A = [
  [2, 0, 1],
  [0, 3, 0],
  [4, 0, 5]
]
```
## CSR 표현:
```
I = [0, 2, 3, 5]       // 행별 시작 위치: row0→0~1, row1→2, row2→3~4
J = [0, 2, 1, 0, 2]    // 각 항의 열 위치
A = [2.0, 1.0, 3.0, 4.0, 5.0] // 각 항의 값
```
## 🧮 계산 단계별 설명
### 1️⃣ row 0: A[0] = [2, 0, 1]
- 시작: I[0] = 0, 끝: I[1] = 2 → 항 인덱스 0~1
- 항들:
    - A[0] = 2.0, J[0] = 0 → x[0] = 1.0 → 2.0 × 1.0 = 2.0
    - A[1] = 1.0, J[1] = 2 → x[2] = 3.0 → 1.0 × 3.0 = 3.0
    - 합계: 2.0 + 3.0 = 5.0
### 2️⃣ row 1: A[1] = [0, 3, 0]
- 시작: I[1] = 2, 끝: I[2] = 3 → 항 인덱스 2
- 항:
    - A[2] = 3.0, J[2] = 1 → x[1] = 2.0 → 3.0 × 2.0 = 6.0
### 3️⃣ row 2: A[2] = [4, 0, 5]
- 시작: I[2] = 3, 끝: I[3] = 5 → 항 인덱스 3~4
- 항들:
    - A[3] = 4.0, J[3] = 0 → x[0] = 1.0 → 4.0 × 1.0 = 4.0
    - A[4] = 5.0, J[4] = 2 → x[2] = 3.0 → 5.0 × 3.0 = 15.0
    - 합계: 4.0 + 15.0 = 19.0

## ✅ 최종 검증
```rust
assert!(approx(y0, 5.0, 1e-12));  // row 0 결과
assert!(approx(y1, 6.0, 1e-12));  // row 1 결과
assert!(approx(y2, 19.0, 1e-12)); // row 2 결과
```
- 모든 결과가 정확히 계산되었음을 확인합니다.

## 📌 요약

| 행 번호 | 계산식                          | 기대 결과 | 설명                                 |
|---------|----------------------------------|------------|--------------------------------------|
| row 0   | 2×x₀ + 1×x₂ = 2×1 + 1×3         | 5.0        | A[0] = [2, 0, 1] · x = [1, 2, 3]     |
| row 1   | 3×x₁ = 3×2                      | 6.0        | A[1] = [0, 3, 0] · x = [1, 2, 3]     |
| row 2   | 4×x₀ + 5×x₂ = 4×1 + 5×3         | 19.0       | A[2] = [4, 0, 5] · x = [1, 2, 3]     |

## 📦 CSR 배열 구성
### 1️⃣ A: 실제 값들 (non-zero values)
0이 아닌 값만 순서대로 나열합니다:
```
A = [2.0, 1.0, 3.0, 4.0, 5.0]
```

### 2️⃣ J: 각 값의 열 인덱스
각 값이 어느 열에 있는지 나열합니다:
```
J = [0, 2, 1, 0, 2]
```

- A[0][0] = 2.0 → 열 0
- A[0][2] = 1.0 → 열 2
- A[1][1] = 3.0 → 열 1
- A[2][0] = 4.0 → 열 0
- A[2][2] = 5.0 → 열 2

### 3️⃣ I: 각 행의 시작 인덱스
A와 J에서 각 행이 어디서 시작하는지 표시합니다:
```
I = [0, 2, 3, 5]
```
- I[0] = 0 → row 0은 A[0]~A[1] (2개 항)
- I[1] = 2 → row 1은 A[2] (1개 항)
- I[2] = 3 → row 2는 A[3]~A[4] (2개 항)
- I[3] = 5 → 마지막 인덱스는 전체 항 개수

## 🔍 시각적 대응
| 행 번호 | 실제 항들        | A 값들 | J 값들 | I 범위     |
|---------|------------------|--------|--------|------------|
| row 0   | A[0][0], A[0][2] | 2.0, 1.0 | 0, 2  | I[0]=0 ~ I[1]=2 |
| row 1   | A[1][1]          | 3.0      | 1      | I[1]=2 ~ I[2]=3 |
| row 2   | A[2][0], A[2][2] | 4.0, 5.0 | 0, 2  | I[2]=3 ~ I[3]=5 |


## ✅ 요약
- A: 0이 아닌 값만 저장
- J: 각 값의 열 위치
- I: 각 행이 A와 J에서 어디서 시작하는지 알려줌

---

# 🧩 Matrix -> CSR 변환 함수
```rust
/// 밀집 행렬을 CSR 포맷으로 변환
pub fn dense_to_csr(matrix: &[Vec<f64>]) -> (Vec<usize>, Vec<usize>, Vec<f64>) {
    let mut i = Vec::with_capacity(matrix.len() + 1); // 행 포인터
    let mut j = Vec::new(); // 열 인덱스
    let mut a = Vec::new(); // 값

    let mut count = 0;
    i.push(0); // 첫 번째 행 시작은 항상 0

    for row in matrix {
        for (col_idx, &val) in row.iter().enumerate() {
            if val != 0.0 {
                j.push(col_idx);
                a.push(val);
                count += 1;
            }
        }
        i.push(count); // 다음 행 시작 인덱스
    }

    (i, j, a)
}
```


## ✅ 사용 예시
```rust
let dense = vec![
    vec![2.0, 0.0, 1.0],
    vec![0.0, 3.0, 0.0],
    vec![4.0, 0.0, 5.0],
];

let (i, j, a) = dense_to_csr(&dense);

assert_eq!(i, vec![0, 2, 3, 5]);
assert_eq!(j, vec![0, 2, 1, 0, 2]);
assert_eq!(a, vec![2.0, 1.0, 3.0, 4.0, 5.0]);
```


## 📌 CSR 포맷 요약
| 배열 이름 | 의미               | 설명                                                         |
|-----------|--------------------|--------------------------------------------------------------|
| I         | Row Pointer        | 각 행의 시작 인덱스를 나타냄 (길이 = 행 개수 + 1)            |
| J         | Column Indices     | 각 값이 위치한 열 인덱스를 저장 (길이 = 비영 항 개수)        |
| A         | Non-zero Values    | 실제 값이 있는 항들만 저장 (길이 = 비영 항 개수)             |

이 함수는 디버깅, 시각화, 외부 라이브러리 연동 시 CSR 포맷을 자동 생성하는 데 매우 유용합니다.

# 🧩 CSR -> Matrix 변환 함수
```rust
/// CSR 포맷을 밀집 행렬로 변환
pub fn csr_to_dense(i: &[usize], j: &[usize], a: &[f64]) -> Vec<Vec<f64>> {
    let n_rows = i.len() - 1;
    // 열 개수는 j에서 최대값 + 1로 추정
    let n_cols = j.iter().copied().max().unwrap_or(0) + 1;

    let mut dense = vec![vec![0.0; n_cols]; n_rows];

    for row in 0..n_rows {
        let start = i[row];
        let end = i[row + 1];
        for k in start..end {
            let col = j[k];
            let val = a[k];
            dense[row][col] = val;
        }
    }

    dense
}
```
---

# CSR 범용성

지금까지 정리하신 방식은 Sparse Solver들이 사용하는 가장 범용적이고 표준적인 데이터 표현 방식입니다.  
특히 CSR(Compressed Sparse Row) 포맷은 희소 행렬 연산에서 다음과 같은 이유로 널리 사용됩니다:

## ✅ 왜 CSR 포맷이 범용적인가?

| 특징            | 설명                                                   |
|-----------------|--------------------------------------------------------|
| 메모리 효율성   | 0이 아닌 항만 저장 → 대규모 희소 행렬에서도 공간 절약 |
| 연산 최적화     | 행 단위 연산에 최적화 → 빠른 내적, 곱셈 가능           |
| 구조 단순함     | 단 3개의 배열(`I`, `J`, `A`)으로 전체 행렬 표현 가능   |
| 범용성          | 대부분의 수치 해석 라이브러리에서 기본 포맷으로 사용   |
| 연산 호환성     | Ax = b, Aᵗ, A·B 등 다양한 연산에 직접 활용 가능        |


## 🧠 실제 사용 예시

| 분야             | 활용 방식                                         | 예시 또는 목적                         |
|------------------|--------------------------------------------------|----------------------------------------|
| FEM/CFD/CAE      | 희소 행렬 조립 및 해석                           | 구조 해석, 열전달, 유체 시뮬레이션     |
| 머신러닝         | 희소 특성 벡터 내적, CSR 기반 모델 입력          | 텍스트 분류, 추천 시스템               |
| 최적화/계획 문제 | 제약 조건 행렬 표현                              | 선형계획법, 제약 만족 문제             |
| 그래픽스/시뮬레이션 | 물리 기반 애니메이션, 변환 행렬 적용         | 강체 운동, 변형 시뮬레이션             |
| 수치 해석        | 선형 시스템 해법 (Ax = b), 전치, 곱셈 등         | CG, BiCGSTAB, GMRES, LU, ILU           |
| 데이터 분석      | 희소 행렬 기반 차원 축소 및 시각화              | PCA, SVD, 구조 시각화                  |


## 🧩 CSR 외에도 존재하는 포맷들

| 포맷 이름 | 구조 예시             | 특징 및 용도                                                  |
|-----------|------------------------|----------------------------------------------------------------|
| COO       | (row, col, val)        | 가장 직관적인 형태. 정렬 필요. 초기 데이터 수집에 적합         |
| CSC       | I, J, A (열 기준)      | CSR의 열 버전. 열 기반 연산에 최적화 (예: 전치, 열 제거)       |
| ELL       | 고정 열 수 배열        | 각 행에 동일한 항 개수 필요. GPU 연산에 적합                   |
| DIA       | 대각선 배열            | 대각선 패턴이 뚜렷한 행렬에 최적화. 메모리 접근 효율 높음       |
| BSR       | 블록 단위 CSR          | 블록 희소 행렬 표현. 구조적 반복이 있는 행렬에 적합            |
- 하지만 범용성과 구현 편의성 측면에서는 CSR이 가장 널리 쓰입니다.

---


