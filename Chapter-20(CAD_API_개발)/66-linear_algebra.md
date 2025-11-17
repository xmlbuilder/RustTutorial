## 📘 LU 분해 기반 선형 방정식 풀이 API 문서
### 📐 수식 정리
LU 분해는 정방행렬 A를 다음과 같이 두 행렬의 곱으로 분해하는 방법입니다:

$$
A=LU
$$

- $L$: 하삼각 행렬 (Lower triangular matrix)
- $U$: 상삼각 행렬 (Upper triangular matrix)
$LU$ 분해를 통해 선형 방정식 Ax=b를 다음 두 단계로 해결할 수 있습니다:
- 전진 대입 (Forward substitution): $Ly=b$
- 후진 대입 (Backward substitution): $Ux=y$

## ⚙️ 핵심 함수 설명
### 1. on_lu_decompose_scaled_partial_pivot
$LU$ 분해를 수행하며 스케일된 부분 피벗 선택을 적용합니다.
- 입력:
  - a: 제자리에서 LU로 변형될 정방행렬
  - aidx: 행 교환 이력을 저장할 인덱스 배열
  - policy: 피벗 정책 (CppCompatible 또는 Strict { tol })
- 동작:
  - 각 행의 최대 절대값으로 스케일링
  - 피벗 선택 시 스케일된 값 기준으로 최대값 행을 선택
  - 피벗이 0일 경우 정책에 따라 처리
  - 반환: true (성공) 또는 false (특이 행렬)

### 2. on_lu_solve_in_place_vec
$LU$ 분해된 행렬과 피벗 인덱스를 사용해 벡터 $b$ 를 해 $x$ 로 교체합니다.
- 입력:
  - a: LU 분해된 행렬
  - indx: 피벗 인덱스
  - b: RHS 벡터 (제자리에서 해로 변경됨)
- 동작:
  - 전진 대입: $Ly=b$
  - 후진 대입: $Ux=y$

### 3. on_solve_equation_vec_cpp / on_solve_equation_vec_strict
$LU$ 분해 + 벡터 해 구하기를 한 번에 수행합니다.
- CppCompatible: pivot이 0이면 EPS로 대체
- Strict: pivot이 작으면 실패 반환

### 4. lu_solve_in_place_mat
$LU$ 분해된 행렬과 피벗 인덱스를 사용해 행렬 $B$ 의 각 열을 해로 교체합니다.
- 입력:
  - a: LU 분해된 행렬
  - index: 피벗 인덱스
  - b: RHS 행렬 (제자리에서 해로 변경됨)
- 동작: 각 열에 대해 lu_solve_inplace_vec와 동일한 방식 적용

### 5. on_solve_equation_mat
#LU# 분해 + 행렬 해 구하기를 한 번에 수행합니다.

## 🧪 API 사용 예제
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::linear_algebra::{solve_equation_mat, solve_equation_vec};
    use nurbslib::core::matrix::DenseMat;
    use nurbslib::core::tvector::DenseVec;

```
```rust
    #[derive(Debug, Clone)]
    pub struct MatOwned {
        rows: usize,
        cols: usize,
        data: Vec<f64>,
    }
```
```rust
    impl MatOwned {
        pub fn new(rows: usize, cols: usize) -> Self {
            Self {
                rows,
                cols,
                data: vec![0.0; rows * cols],
            }
        }

        pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
            assert_eq!(data.len(), rows * cols);
            Self { rows, cols, data }
        }

        pub fn to_vec(&self) -> Vec<f64> {
            self.data.clone()
        }

        pub fn swap_rows(&mut self, i: usize, j: usize) {
            for col in 0..self.cols {
                self.data.swap(i * self.cols + col, j * self.cols + col);
            }
        }
    }
```
```rust
    // DenseMat 트레이트 구현
    impl DenseMat for MatOwned {
        fn n_rows(&self) -> usize {
            self.rows
        }

        fn n_cols(&self) -> usize {
            self.cols
        }

        fn get(&self, row: usize, col: usize) -> f64 {
            self.data[row * self.cols + col]
        }

        fn set(&mut self, row: usize, col: usize, val: f64) {
            self.data[row * self.cols + col] = val;
        }

        fn swap_rows(&mut self, i: usize, j: usize) {
            self.swap_rows(i, j);
        }
    }

```
```rust
    #[derive(Debug, Clone)]
    pub struct VecOwned {
        data: Vec<f64>,
    }
```
```rust
    impl VecOwned {
        /// 생성자: 주어진 Vec<f64>로 초기화
        pub fn from_vec(data: Vec<f64>) -> Self {
            Self { data }
        }

        /// 내부 데이터를 복사해서 반환
        pub fn to_vec(&self) -> Vec<f64> {
            self.data.clone()
        }


    }
```
```rust
    // DenseVec 트레이트 구현
    impl DenseVec for VecOwned {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn get(&self, i: usize) -> f64 {
            self.data[i]
        }

        fn set(&mut self, i: usize, v: f64) {
            self.data[i] = v;
        }
    }
```
```rust

    #[test]
    fn example_solve_vec() {
        // 3x3 행렬 A
        let mut a = MatOwned::from_vec(3, 3, vec![
            2.0, -1.0, 0.0,
            -1.0, 2.0, -1.0,
            0.0, -1.0, 2.0,
        ]);

        // RHS 벡터 b
        let mut b = VecOwned::from_vec(vec![1.0, 2.0, 3.0]);
        let b_ref: &mut dyn DenseVec = &mut b;


        // 방정식 Ax = b 풀기
        let success = solve_equation_vec(&mut a, &mut b);

        if success {
            println!("해 x = {:?}", b.to_vec());
        } else {
            println!("LU 분해 실패");
        }
    }
```
```rust
    #[test]
    fn example_solve_mat() {
        let mut a = MatOwned::from_vec(2, 2, vec![
            3.0, 2.0,
            1.0, 4.0,
        ]);

        let mut b = MatOwned::from_vec(2, 2, vec![
            7.0, 5.0,
            6.0, 3.0,
        ]);

        let success = solve_equation_mat(&mut a, &mut b);

        if success {
            println!("해 행렬 X = {:?}", b.to_vec());
        } else {
            println!("LU 분해 실패");
        }
    }
}
```

## 📌 요약: LU 기반 선형 방정식 풀이 함수

| 함수명                         | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `on_lu_decompose_scaled_partial_pivot` | 스케일된 부분 피벗을 사용하여 제자리 LU 분해 수행, 행 교환 인덱스 기록       |
| `on_lu_solve_in_place_vec`          | LU 분해된 행렬과 피벗 인덱스를 사용해 벡터 RHS를 해로 교체 (제자리 연산)     |
| `on_solve_equation_vec_cpp`        | C++ 스타일 정책으로 LU 분해 후 벡터 방정식 풀이                         |
| `on_solve_equation_vec_strict`    | 엄격한 피벗 기준으로 LU 분해 후 벡터 방정식 풀이                         |
| `on_lu_solve_in_place_mat`          | LU 분해된 행렬과 피벗 인덱스를 사용해 행렬 RHS의 각 열을 해로 교체         |
| `on_solve_equation_mat`            | LU 분해 + 행렬 방정식 풀이를 한 번에 수행                               |
| `on_solve_equation_vec`            | LU 분해 + 벡터 방정식 풀이를 한 번에 수행                               |



## ✅ 수학적 검증: LU 분해 기반 선형 방정식 풀이
### 1. LU 분해의 정의
$LU$ 분해는 정방행렬 $A\in \mathbb{R^{\mathnormal{n\times n}}}$ 를 다음과 같이 두 행렬의 곱으로 분해하는 방법입니다:

$$
A=LU
$$
- $L$: 단위 하삼각 행렬 (diagonal이 모두 1)
- $U$: 상삼각 행렬

단, 일부 경우에는 행 교환이 필요하므로 실제로는 다음과 같은 형태로 분해됩니다:  
$PA=LU$
- $P$: 행 교환을 나타내는 순열 행렬
- 이 코드에서는 P를 명시적으로 사용하지 않고, aidx 배열로 행 교환 이력을 추적합니다.

### 2. 스케일된 부분 피벗 선택
코드에서 사용하는 피벗 선택 방식은 다음과 같습니다:
- 각 행의 최대 절대값 $s_i=\max _j|a_{ij}|$ 를 기준으로 스케일링
- 피벗 선택 시 $|a_{ij}|/s_i$ 가 최대인 행을 선택
이는 스케일된 부분 피벗(Scaled Partial Pivoting) 방식으로, 수치적으로 안정적인 LU 분해를 위한 일반적인 기법입니다.

### 3. LU 분해 알고리즘 검증
코드의 핵심 루프는 다음과 같은 수학적 연산을 수행합니다:
- 상삼각 갱신:

$$  
u_{ji}=a_{ji}-\sum _{k=0}^{j-1}l_{jk}u_{ki}
$$

- 피벗 선택 및 행 교환
- 하삼각 갱신:

$$
l_{ji}=\frac{a_{ji}}{u_{ii}},\quad \mathrm{for\  }j>i

$$
이 연산은 Doolittle 방식의 LU 분해와 동일하며, 수학적으로 정당합니다.

### 4. 전진/후진 대입 검증
$LU$ 분해 후 $Ax=b$ 를 푸는 과정은 다음과 같습니다:
- $Ly=Pb$ — 전진 대입
- $Ux=y$ — 후진 대입
코드에서는 aidx를 통해 Pb를 구현하고, 두 단계의 대입을 정확히 수행합니다:
- 전진 대입:

$$
y_i=b_i-\sum _{j=0}^{i-1}l_{ij}y_j
$$

- 후진 대입:

$$
x_i=\frac{y_i-\sum _{j=i+1}^{n-1}u_{ij}x_j}{u_{ii}}
$$

이 수식은 코드의 on_lu_solve_in_place_vec 및 on_lu_solve_in_place_mat 함수에서 정확히 구현되어 있습니다.

### 5. 특이 행렬 처리
- CppCompatible: pivot이 0이면 $\varepsilon$ 로 대체 → 수치적 안정성 확보
- Strict: pivot이 $\leq \mathrm{tol}$ 이면 실패 → 수학적으로 특이 행렬로 간주
이는 LU 분해의 실패 조건을 수학적으로 올바르게 처리한 것입니다.

## ✅ 결론
이 코드의 LU 분해 및 선형 방정식 풀이 알고리즘은 다음을 만족합니다:
- 수학적으로 정당한 LU 분해 방식 (Doolittle + 스케일된 부분 피벗)
- 전진/후진 대입 방식의 정확한 구현
- 특이 행렬에 대한 예외 처리
- 행 교환을 순열 인덱스로 추적
- 따라서 이 코드는 수학적으로 올바른 방식으로 동작하며, 수치적 안정성까지 고려된 고품질 구현입니다.

---




