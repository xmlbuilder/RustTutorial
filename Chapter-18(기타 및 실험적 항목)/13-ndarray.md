## 🔍 비교 요약: ndarray (Rust) vs numpy.ndarray (Python)
| 항목             | ndarray (Rust)                          | numpy.ndarray (Python)                   |
|------------------|------------------------------------------|------------------------------------------|
| 언어             | Rust                                     | Python                                   |
| 메모리 안전성    | ✅ 소유권 기반, 컴파일 타임 안전성 보장     | ❌ 런타임 오류 가능, null 참조 위험         |
| 성능             | ✅ 제로 비용 추상화, 고성능                 | ✅ C 기반으로 빠름                         |
| 다차원 지원      | ✅ 1D, 2D, nD 모두 지원                    | ✅ 1D, 2D, nD 모두 지원                    |
| 슬라이싱/뷰      | ✅ `view()`, `slice()` 등                  | ✅ `[:]`, `.reshape()` 등                  |
| 연산 지원        | ✅ `+`, `*`, `dot`, `mapv` 등              | ✅ `+`, `*`, `dot`, `apply` 등             |
| 병렬 처리        | ✅ `ndarray-parallel`으로 가능             | ✅ `numba`, `dask` 등으로 확장 가능        |
| 생태계 통합      | ✅ `nalgebra`, `ndarray-linalg`, `ndarray-stats` 등 | ✅ `scipy`, `pandas`, `matplotlib` 등과 통합 |


## 🧠 핵심 차이
- ndarray는 Rust답게 소유권, borrowing, lifetime을 따르기 때문에  
    → 메모리 안전성과 성능을 동시에 확보
- numpy는 Python답게 간결하고 직관적이지만  
    → 런타임 오류나 메모리 누수 가능성 있음

## 🔧 예시: Rust의 ndarray
```rust
use ndarray::Array2;
let a = Array2::<f64>::zeros((3, 3));
let b = a.mapv(|x| x + 1.0);
```
- Array2는 2차원 배열
- mapv는 각 요소에 함수 적용

## 🔧 ndarray 주요 함수 목록 + 예제

### 1. Array::from_vec / Array::from_shape_vec
```rust
use ndarray::Array;
let arr1 = Array::from_vec(vec![1, 2, 3, 4, 5]); // 1D
let arr2 = Array::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap(); // 2D
```

### 2. zeros, ones, eye
```rust
use ndarray::{Array2, Array};
let zeros = Array2::<f64>::zeros((3, 3));
let ones = Array2::<f64>::ones((2, 2));
let identity = Array::eye(3);
```

### 3. mapv, mapv_inplace
```rust
let arr = Array::from_vec(vec![1, 2, 3]);
let doubled = arr.mapv(|x| x * 2);

let mut arr2 = Array::from_vec(vec![1, 2, 3]);
arr2.mapv_inplace(|x| x + 1);
```

### 4. dot (내적)
```rust
use ndarray::Array1;

let a = Array1::from_vec(vec![1., 2., 3.]);
let b = Array1::from_vec(vec![4., 5., 6.]);
let result = a.dot(&b); // 1*4 + 2*5 + 3*6 = 32
```

### 5. slice, view, slice_mut
```rust
use ndarray::{Array2, s};
let arr = Array2::from_shape_vec((3, 3), (0..9).collect()).unwrap();
let slice = arr.slice(s![0..2, 1..3]); // 부분 배열
```

### 6. into_shape
```rust
let arr = Array::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
let reshaped = arr.into_shape((3, 2)).unwrap();
```

### 7. stack, concatenate
```rust
use ndarray::{Array1, Axis, stack};
let a = Array1::from_vec(vec![1, 2]);
let b = Array1::from_vec(vec![3, 4]);
let stacked = stack![Axis(0), a, b]; // [1, 2, 3, 4]
```

### 8. zip, azip!, zip_mut_with
```rust
use ndarray::{Array1, Zip};

let a = Array1::from_vec(vec![1, 2, 3]);
let b = Array1::from_vec(vec![4, 5, 6]);
let mut result = Array1::zeros(3);

Zip::from(&mut result)
    .and(&a)
    .and(&b)
    .apply(|r, &x, &y| *r = x + y);
```

### 9. sum, mean, std
```rust
use ndarray_stats::SummaryStatisticsExt;

let arr = Array::from_vec(vec![1., 2., 3., 4.]);
let mean = arr.mean().unwrap(); // 2.5
let std = arr.std(0.0); // 표준편차
```

### 10. par_mapv, par_azip! (병렬 처리)
```rust
use ndarray::Array1;
use ndarray::parallel::prelude::*;

let arr = Array1::from_vec(vec![1, 2, 3, 4]);
let doubled = arr.par_mapv(|x| x * 2);
```


### 11. linspace: 일정 간격으로 값 생성
```rust
use ndarray::Array1;

fn main() {
    let a = Array1::linspace(0.0, 1.0, 5);
    println!("{:?}", a);
}
```

#### 📌 출력:
```
[0.0, 0.25, 0.5, 0.75, 1.0]
```
- 시작값: 0.0
- 끝값: 1.0
- 총 5개의 값 생성 (양 끝 포함)

### 12. range: 시작값부터 일정 간격으로 값 생성
```rust
use ndarray::Array1;

fn main() {
    let a = Array1::range(0.0, 10.0, 2.0);
    println!("{:?}", a);
}
```

#### 📌 출력:
```
[0.0, 2.0, 4.0, 6.0, 8.0]
```

- 시작값: 0.0
- 끝값: 10.0 (포함되지 않음)
- 간격: 2.0

## 📦 ndarray 배열 생성 함수 요약표
| 함수       | 설명                                 | 예제 코드                                      |
|------------|--------------------------------------|------------------------------------------------|
| zeros      | 모든 요소가 0인 배열 생성             | let a = Array2::<f64>::zeros((3, 3));          |
| ones       | 모든 요소가 1인 배열 생성             | let a = Array2::<f64>::ones((2, 2));           |
| eye        | 단위 행렬 (Identity matrix) 생성      | let a = Array::eye(4);                         |
| from_vec   | Vec로부터 배열 생성                   | let a = Array1::from_vec(vec![1, 2, 3]);       |
| from_shape_vec | shape와 Vec으로 배열 생성         | let a = Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap(); |
| linspace   | 일정 간격의 값으로 1D 배열 생성       | let a = Array1::linspace(0., 1., 5);           |
| range      | 범위 기반 1D 배열 생성                | let a = Array1::range(0., 10., 2.);            |


---


# ndarray에서 데이터 꺼낸 쓰는 방법
Rust의 ndarray에서 데이터를 꺼내 쓰는 방법은 Python의 numpy와 비슷하면서도 Rust의 안전성과 명확성을 따릅니다.


## 🔍 ndarray에서 데이터 꺼내는 방법
| 방식              | 설명                                | 예제 코드                                           |
|-------------------|-------------------------------------|-----------------------------------------------------|
| `[[i, j]]`        | 인덱스로 직접 접근 (panic 가능)     | let val = arr[[1, 2]];                              |
| `.get(i)`         | 안전한 접근 (Option 반환)           | if let Some(v) = arr.get(3) { println!("{}", v); }  |
| `.slice(s![..])`  | 슬라이싱 (부분 배열 추출)           | let sub = arr.slice(s![0..2, 1..3]);                |
| `.iter()`         | 전체 요소 반복                      | for x in arr.iter() { println!("{}", x); }          |
| `.rows()`         | 행 단위 반복                        | for row in arr.rows() { println!("{:?}", row); }    |
| `.to_vec()`       | Vec로 복사                          | let v = arr.to_vec();                               |
| `.as_slice()`     | &[T] 참조 (연속 메모리일 때만)      | let slice = arr.as_slice().unwrap();               |

## 🔧 예제: 2D 배열에서 값 꺼내기
```rust
use ndarray::{array, s};

fn main() {
    let arr = array![[1, 2, 3], [4, 5, 6]];

    // 인덱스로 접근
    let val = arr[[1, 2]];
    println!("arr[1][2] = {}", val); // 6

    // 슬라이싱
    let sub = arr.slice(s![0..2, 1..]);
    println!("Slice: {:?}", sub); // [[2, 3], [5, 6]]

    // 반복자
    for x in arr.iter() {
        println!("Value: {}", x);
    }

    // Vec로 변환
    let flat = arr.to_vec();
    println!("Flat Vec: {:?}", flat); // [1, 2, 3, 4, 5, 6]
}
```


## 💡 참고
- [[i, j]]는 panic 가능 → 안전하게 하려면 .get([i, j])
- .as_slice()는 연속 메모리일 때만 가능 (예: Array1, Array2의 기본 생성)
- .to_vec()은 항상 가능하지만 복사 비용 있음


---


## 📦 추가 기능들
- 선형대수: ndarray-linalg로 solve, inv, eig 등 가능
- 통계: ndarray-stats로 quantile, histogram, zscore 등
- 병렬 처리: rayon과 함께 par_mapv, par_azip! 사용



## 📐 선형대수: ndarray-linalg 주요 함수 + 예제
| 함수       | 기능 설명                            | 예제 코드                                 |
|------------|---------------------------------------|--------------------------------------------|
| solve      | 선형 방정식 Ax = b 해법               | let x = a.solve(&b)?;                      |
| inv        | 행렬의 역행렬                         | let inv_a = a.inv()?;                      |
| eig        | 고유값 및 고유벡터 계산               | let (eigvals, eigvecs) = a.eig()?;         |
| svd        | 특이값 분해                           | let (u, s, vt) = a.svd(true, true)?;       |
| qr         | QR 분해                               | let (q, r) = a.qr()?;                      |
| cholesky   | Cholesky 분해 (양의 정부호 행렬)      | let l = a.cholesky()?;                     |
| lu         | LU 분해                               | let (l, u, p) = a.lu()?;                   |


## 🔧 예제:
```rust
use ndarray::array;
use ndarray_linalg::Solve;

let a = array![[3., 2.], [1., 2.]];
let b = array![5., 5.];
let x = a.solve(&b).unwrap();
println!("Solution: {}", x);
```


## 📊 통계: ndarray-stats 주요 함수 + 예제
| 함수       | 기능 설명                          | 예제 코드                                 |
|------------|-------------------------------------|--------------------------------------------|
| mean       | 평균값 계산                         | let m = arr.mean()?;                       |
| std        | 표준편차 계산                       | let s = arr.std(0.0);                      |
| quantile   | 분위수 계산                         | let q = arr.quantile(0.75)?;              |
| zscore     | Z-점수 정규화                       | let z = arr.zscore(0.0)?;                 |
| histogram  | 히스토그램 생성                     | let h = arr.histogram(10)?;               |
| minmax     | 최솟값 / 최댓값 계산                | let min = arr.min()?;                     |


### 🔧 예제:
```rust
use ndarray::Array1;
use ndarray_stats::SummaryStatisticsExt;

let arr = Array1::from(vec![1., 2., 3., 4., 5.]);
let mean = arr.mean().unwrap();
let std = arr.std(0.0);
println!("Mean: {}, Std: {}", mean, std);
```

### ✅ 1. solve: 선형 방정식 Ax = b
```rust
use ndarray::array;
use ndarray_linalg::Solve;

let a = array![[3., 2.], [1., 2.]];
let b = array![5., 5.];
let x = a.solve(&b)?;
println!("x = {}", x);
```


### ✅ 2. inv: 행렬의 역행렬
```rust
use ndarray::array;
use ndarray_linalg::Inverse;

let a = array![[1., 2.], [3., 4.]];
let inv_a = a.inv()?;
println!("Inverse = {}", inv_a);

```

### ✅ 3. eig: 고유값 및 고유벡터
```rust
use ndarray::array;
use ndarray_linalg::Eig;

let a = array![[2., 0.], [0., 3.]];
let (eigvals, eigvecs) = a.eig()?;
println!("Eigenvalues = {}", eigvals);
println!("Eigenvectors = {}", eigvecs);
```


### ✅ 4. svd: 특이값 분해
```rust
use ndarray::array;
use ndarray_linalg::SVD;

let a = array![[1., 0.], [0., -2.]];
let (u, s, vt) = a.svd(true, true)?;
println!("U = {}\nS = {}\nVᵗ = {}", u.unwrap(), s, vt.unwrap());
```


### ✅ 5. qr: QR 분해
```rust
use ndarray::array;
use ndarray_linalg::QR;

let a = array![[1., 2.], [3., 4.]];
let (q, r) = a.qr()?;
println!("Q = {}\nR = {}", q, r);
```


### ✅ 6. cholesky: Cholesky 분해
```rust
use ndarray::array;
use ndarray_linalg::Cholesky;

let a = array![[4., 2.], [2., 3.]];
let l = a.cholesky()?;
println!("L = {}", l);
```


### ✅ 7. lu: LU 분해
```rust
use ndarray::array;
use ndarray_linalg::LU;

let a = array![[4., 3.], [6., 3.]];
let (l, u, p) = a.lu()?;
println!("L = {}\nU = {}\nP = {}", l, u, p);
```


## 📦 설치 방법 (Cargo.toml)
```
ndarray = "0.15"
ndarray-linalg = { version = "0.16", features = ["openblas"] }
```
- openblas 또는 intel-mkl 백엔드가 필요합니다. 
- 시스템에 따라 설치 필요.

---




