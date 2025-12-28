# 📘 ND – Rust용 NumPy 스타일 N차원 배열 라이브러리
- ND는 ndarray를 내부적으로 사용하지만 완전히 은닉하고,
- 사용자는 Rust 기본 타입(Vec, slice 등)만으로 NumPy처럼 배열을 다룰 수 있다.
  - 다차원 배열 생성
  - broadcasting
  - boolean mask
  - slicing / take
  - reshape
  - matmul / inner / outer
  - sort / unique / concatenate
  - 통계(sum/mean/std)
  - pretty print
  - NaN 처리
- 까지 모두 지원한다.

## 🧱 ND 기본 구조
```rust
pub struct ND {
    data: ArrayD<f64>,   // ndarray는 완전히 숨김
}
```

- 사용자는 내부 ndarray 타입을 전혀 볼 필요가 없다.

### 📌 1. 생성 함수군 (NumPy 스타일)
- from_vec(Vec<f64>)
  - 1D 배열 생성
```rust
let a = ND::from_vec(vec![1.,2.,3.]);
```

- from_vec2(Vec<Vec<f64>>)
  - 2D 배열 생성
```rust
let a = ND::from_vec2(vec![
    vec![1.,2.],
    vec![3.,4.],
]);
```

- from_slice2(&[&[f64]])
```rust
let a = ND::from_slice2(&[
    &[1.,2.],
    &[3.,4.],
]);
```

- from_flat_and_shape(flat, shape)
```rust
let a = ND::from_flat_and_shape(vec![1.,2.,3.,4.], &[2,2]);
```

- from_iter(iter)
```rust
let a = ND::from_iter(0..5); // 자동 f64 변환 필요하면 map
```

- array(&[f64])
```rust
let a = ND::array(&[1.,2.,3.]);
```


## 📌 2. NumPy 스타일 생성
- arange(n)
```rust
let a = ND::arange(5); // [0,1,2,3,4]
```

- linspace(start, end, n)
```rust
let a = ND::linspace(0., 1., 5);
```

- zeros(shape) / ones(shape) / full(shape, v)
```rust
let z = ND::zeros(&[2,3]);
let o = ND::ones(&[2,3]);
let f = ND::full(&[2,3], 7.);
```
- empty(shape)
  - Rust에서는 안전성 때문에 zeros로 초기화.
- eye(n)
```rust
let e = ND::eye(3);
```
- random(shape)
```rust
let r = ND::random(&[2,3]);
```

- from_function(shape, |idx| …)
```rust
let a = ND::from_function(&[3,3], |i| (i[0] + i[1]) as f64);
```

## 📌 3. 형태 변환 / 슬라이싱
- shape()
```rust
println!("{:?}", a.shape());
```
- reshape(shape)
```rust
let b = a.reshape(&[3,2]);
```
- slice2(rows, cols) (2D 전용)
```rust
let s = a.slice2(0..2, 1..3);
```
- slice_axis(axis, range)
```rust
let s = a.slice_axis(0, 1..4);
```

- take(axis, idx)
```rust
let s = a.take(1, &[0,2]);
```

## 📌 4. Boolean Mask
- gt / ge / lt / le / eq_val
```rust
let mask = a.gt(10.);
```
- eq_nd(other)
```rust
let mask = a.eq_nd(&b);
```

- select(mask)
  - NumPy: a[mask]
```
let filtered = a.select(&mask);
```

- where_mask(cond, x, y)
  - NumPy: np.where(cond, x, y)
```rust
let out = ND::where_mask(&mask, &x, &y);
```

## 📌 5. 통계
- a.sum()
- a.mean()
- a.std()
- a.min()
- a.max()


- 축 연산:
```rust
a.sum_axis(0)
a.mean_axis(1)
```

## 📌 6. 선형대수
- matmul (2D × 2D)
```rust
let c = a.matmul(&b);
```

- inner (1D dot)
```rust
let d = a.inner(&b);
```

- outer (1D outer product)
```rust
let o = a.outer(&b); // shape [m,n]
```

## 📌 7. 반복 / 출력
- iter()
```rust
for v in a.iter() {
    println!("{}", v);
}
print()
a.print();
```

## 📌 8. 유틸리티 연산 (ufunc 스타일)
- a.abs()
- a.sqrt()
- a.exp()
- a.clip(0., 1.)

## 📌 9. 정렬 / 집합
- sort()
```rust
let s = a.sort();
```
- unique()
```rust
let u = a.unique();
```

## 📌 10. concatenate
- NumPy: np.concatenate([a,b], axis=0)
```rust
let c = ND::concatenate(&a, &b, 0);
```

## 📌 11. argmax / argmin
```rust
let i = a.argmax();
let j = a.argmin();
```
## 📌 12. 2D 전용 편의 함수
- a.as_2d()
- a.at2(r,c)
- a.row2(r)
- a.col2(c)
- a.row_vec2(r)
- a.col_vec2(c)
- a.to_vec2_row_major()

## 📌 13. NaN 처리
- a.is_nan()
- a.fill_nan(0.)

## 📌 14. argsort / permute_rows2
```rust
let idx = a.argsort1();
let sorted_rows = a.permute_rows2(&idx);
```

## 🎉 마무리
이 문서로 ND의 전체 기능을 한눈에 볼 수 있음:
  - NumPy 스타일 생성
  - broadcasting
  - boolean mask
  - slicing
  - reshape
  - 선형대수
  - 정렬/unique
  - NaN 처리
  - pretty print
- 이 정도면 Rust에서 완전한 수치 계산 엔진.

---

## 소스 코드
```rust
use ndarray::{Array, Array1, Array2, ArrayD, Axis, Ix1, Ix2, IxDyn, s, Dimension};
use rand::distributions::Uniform;
use rand::Rng;
use std::ops::{Add, Div, Mul, Sub};

use crate::core::types::Real;
```
```rust
/// Boolean mask type (NumPy의 boolean array와 유사)
#[derive(Clone, Debug)]
pub struct Mask {
    pub data: ArrayD<bool>,
}
```
```rust
impl Mask {
    fn from_array(data: ArrayD<bool>) -> Self {
        Self { data }
    }
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    pub fn any(&self) -> bool {
        self.data.iter().any(|&b| b)
    }

    pub fn all(&self) -> bool {
        self.data.iter().all(|&b| b)
    }
}
```
```rust
/// NumPy 느낌의 N차원 배열 래퍼
#[derive(Clone, Debug)]
pub struct ND {
    data: ArrayD<f64>, // 완전 은닉
}
```
```rust
impl ND {
    /// 내부 ndarray를 숨기고 ND로 감싸는 생성자
    pub fn from_array(data: ArrayD<f64>) -> Self {
        Self { data }
    }
```
```rust
    /// 1D vec로부터 ND 생성 (shape = [len])
    pub fn from_vec(v: Vec<f64>) -> Self {
        let arr = Array1::from(v).into_dyn();
        Self { data: arr }
    }
```
```rust
    pub fn from_vec_2d(i: usize, j: usize, v: Vec<f64>) -> Self {
        Self::from_array(
            Array2::from_shape_vec((i, j), v)
                .expect("from_vec_2d: shape mismatch")
                .into_dyn(),
        )
    }
```
```rust
    pub fn data(&self) -> &ArrayD<f64> {
        &self.data
    }
```
```rust
    pub fn data_clone(&self) -> ArrayD<f64> {
        self.data.clone()
    }
```
```rust
    /// contiguous(표준 레이아웃)일 때만 1D Vec로 복사
    pub fn to_vec(&self) -> Vec<f64> {
        self.data
            .as_slice()
            .expect("to_vec: array is not contiguous (not standard layout)")
            .to_vec()
    }
```
```rust
    /// 어떤 레이아웃이든 flat Vec로 복사 (fallback)
    pub fn to_vec_fallback(&self) -> Vec<f64> {
        self.data.iter().copied().collect::<Vec<f64>>()
    }
```
```rust
    pub fn to_flat_vec_with_shape(&self) -> (Vec<f64>, Vec<usize>) {
        let shape = self.data.shape().to_vec();
        let data = self
            .data
            .as_slice()
            .map(|s| s.to_vec())
            .unwrap_or_else(|| self.data.iter().copied().collect::<Vec<f64>>());
        (data, shape)
    }
```
```rust
    /// NumPy: np.array([...]) 비슷한 느낌
    pub fn array(slice: &[f64]) -> Self {
        let arr = Array1::from(slice.to_vec()).into_dyn();
        Self { data: arr }
    }

    /// shape 정보 반환
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }
```
```rust
    #[inline]
    fn assert_same_shape(&self, other: &ND, ctx: &str) {
        assert_eq!(
            self.shape(),
            other.shape(),
            "{ctx}: shape mismatch {:?} vs {:?}",
            self.shape(),
            other.shape()
        );
    }
```
```rust
    // --------------------
    // 생성 함수군 (NumPy 스타일)
    // --------------------

    /// np.arange(n)
    pub fn arange(n: usize) -> Self {
        let arr = Array1::from_iter((0..n).map(|x| x as f64)).into_dyn();
        Self { data: arr }
    }
```
```rust
    /// np.linspace(start, end, n)
    pub fn linspace(start: f64, end: f64, n: usize) -> Self {
        let arr = Array1::linspace(start, end, n).into_dyn();
        Self { data: arr }
    }
```
```rust
    /// np.zeros(shape)
    pub fn zeros(shape: &[usize]) -> Self {
        Self {
            data: ArrayD::<f64>::zeros(IxDyn(shape)),
        }
    }
```
```rust
    /// np.ones(shape)
    pub fn ones(shape: &[usize]) -> Self {
        Self {
            data: ArrayD::<f64>::ones(IxDyn(shape)),
        }
    }
```
```rust
    /// np.full(shape, value)
    pub fn full(shape: &[usize], value: f64) -> Self {
        Self {
            data: ArrayD::<f64>::from_elem(IxDyn(shape), value),
        }
    }
```
```rust
    /// np.empty(shape)
    /// Rust에서는 안전성 때문에 실제로는 zeros로 초기화
    pub fn empty(shape: &[usize]) -> Self {
        Self {
            data: ArrayD::<f64>::zeros(IxDyn(shape)),
        }
    }
```
```rust
    /// np.eye(n)
    pub fn eye(n: usize) -> Self {
        let mut arr = Array2::<f64>::zeros((n, n));
        for i in 0..n {
            arr[(i, i)] = 1.0;
        }
        Self { data: arr.into_dyn() }
    }
```
```rust
    pub fn diag_sum(n: usize) -> Real {
        let eye = ND::eye(n);
        let diag_sum = ND::from_array(
            eye.data
                .into_dimensionality::<Ix2>()
                .expect("diag_sum: eye not 2D")
                .diag()
                .to_owned()
                .into_dyn(),
        )
            .sum();
        diag_sum
    }
```
```rust
    /// np.random.random(shape)
    pub fn random(shape: &[usize]) -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(0.0, 1.0);
        let arr = Array::from_shape_simple_fn(IxDyn(shape), || rng.sample(dist));
        Self { data: arr }
    }
```
```rust
    // --------------------
    // 형태/뷰 관련
    // --------------------

    /// np.reshape(shape)
    pub fn reshape(&self, shape: &[usize]) -> Self {
        let reshaped = self
            .data
            .clone()
            .into_shape(IxDyn(shape))
            .expect("reshape: incompatible shape (element count mismatch)");
        Self { data: reshaped }
    }
```
```rust
    /// 2D 전용: a[r0..r1, c0..c1] slicing (view → 새 ND)
    pub fn slice2(&self, rows: std::ops::Range<usize>, cols: std::ops::Range<usize>) -> Self {
        let a2 = self
            .data
            .view()
            .into_dimensionality::<Ix2>()
            .expect("slice2: array is not 2D");
        let view = a2.slice(s![rows, cols]).to_owned().into_dyn();
        Self { data: view }
    }
```
```rust
    // --------------------
    // 비교 연산: Boolean Mask 생성
    // --------------------

    /// a > v  → Mask
    pub fn gt(&self, v: f64) -> Mask {
        let arr = self.data.map(|x| *x > v);
        Mask::from_array(arr)
    }
```
```rust
    /// a >= v
    pub fn ge(&self, v: f64) -> Mask {
        let arr = self.data.map(|x| *x >= v);
        Mask::from_array(arr)
    }
```
```rust
    /// a < v
    pub fn lt(&self, v: f64) -> Mask {
        let arr = self.data.map(|x| *x < v);
        Mask::from_array(arr)
    }
```
```rust
    /// a <= v
    pub fn le(&self, v: f64) -> Mask {
        let arr = self.data.map(|x| *x <= v);
        Mask::from_array(arr)
    }
```
```rust
    /// a == v
    pub fn eq_val(&self, v: f64) -> Mask {
        let arr = self.data.map(|x| *x == v);
        Mask::from_array(arr)
    }
```
```rust
    /// 두 배열 비교: a == b (shape 유지)
    pub fn eq_nd(&self, other: &ND) -> Mask {
        self.assert_same_shape(other, "eq_nd");
        let out = ndarray::Zip::from(&self.data)
            .and(&other.data)
            .map_collect(|a, b| a == b);
        Mask::from_array(out)
    }
```
```rust
    // --------------------
    // Boolean mask 적용 (NumPy: a[mask])
    // --------------------

    /// NumPy: a[mask] → 1D 결과 (numpy와 동일하게 flat)
    pub fn select(&self, mask: &Mask) -> ND {
        assert_eq!(self.shape(), mask.shape(), "select: shape mismatch");
        let mut out = Vec::new();
        for (m, v) in mask.data.iter().zip(self.data.iter()) {
            if *m {
                out.push(*v);
            }
        }
        ND::from_vec(out)
    }
```
```rust
    // --------------------
    // np.where(cond, x, y) 스타일
    // cond: Mask, x,y: ND
    // --------------------

    pub fn where_mask(cond: &Mask, x: &ND, y: &ND) -> ND {
        assert_eq!(cond.shape(), x.shape(), "where_mask: cond/x shape mismatch");
        x.assert_same_shape(y, "where_mask: x/y");

        let out = ndarray::Zip::from(&cond.data)
            .and(&x.data)
            .and(&y.data)
            .map_collect(|c, xv, yv| if *c { *xv } else { *yv });

        ND::from_array(out)
    }
```
```rust
    // --------------------
    // 인덱싱/샘플링
    // --------------------

    /// np.take(a, indices, axis=0) 1D 가정
    pub fn take1(&self, idx: &[usize]) -> ND {
        let arr1 = self
            .data
            .view()
            .into_dimensionality::<Ix1>()
            .expect("take1: array is not 1D");
        let selected = arr1.select(Axis(0), idx);
        ND::from_array(selected.into_dyn())
    }
```
```rust
    // --------------------
    // 통계
    // --------------------

    pub fn sum(&self) -> f64 {
        self.data.sum()
    }
```
```rust
    pub fn mean(&self) -> f64 {
        self.data.mean().expect("mean: empty array")
    }
```
```rust
    pub fn std(&self) -> f64 {
        let m = self.mean();
        let var = self.data.map(|x| (x - m).powi(2)).mean().expect("std: empty array");
        var.sqrt()
    }
```
```rust
    // --------------------
    // 행렬 곱 (2D 전용)
    // --------------------

    /// np.matmul(a, b) (2D 전용)
    pub fn matmul(&self, other: &ND) -> ND {
        let a = self
            .data
            .view()
            .into_dimensionality::<Ix2>()
            .expect("matmul: left not 2D");
        let b = other
            .data
            .view()
            .into_dimensionality::<Ix2>()
            .expect("matmul: right not 2D");

        assert_eq!(a.shape()[1], b.shape()[0], "matmul: inner dim mismatch");
        let out = a.dot(&b);
        ND::from_array(out.into_dyn())
    }
```
```rust
    // --------------------
    // 반복
    // --------------------

    /// NumPy의 nditer 비슷하게 single iterator 제공
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }
```
```rust
    // 디버깅용 출력
    pub fn show(&self) {
        println!("{:?}", self.data);
    }
```
```rust
    // --------------------
    // NumPy 유사 ufunc / 축연산 (자주 쓰는 것)
    // --------------------

    /// element-wise map (numpy ufunc 느낌)
    pub fn mapv<F>(&self, f: F) -> ND
    where
        F: Fn(f64) -> f64 + Copy,
    {
        ND::from_array(self.data.mapv(f))
    }
```
```rust
    pub fn abs(&self) -> ND {
        self.mapv(|x| x.abs())
    }
```
```rust
    pub fn sqrt(&self) -> ND {
        self.mapv(|x| x.sqrt())
    }
```
```rust
    pub fn exp(&self) -> ND {
        self.mapv(|x| x.exp())
    }
```
```rust
    pub fn clip(&self, lo: f64, hi: f64) -> ND {
        self.mapv(|x| x.max(lo).min(hi))
    }
```
```rust
    pub fn min(&self) -> f64 {
        self.data.iter().copied().fold(f64::INFINITY, f64::min)
    }
```
```rust
    pub fn max(&self) -> f64 {
        self.data
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }
```
```rust
    /// sum over axis (keeps remaining axes)
    pub fn sum_axis(&self, axis: usize) -> ND {
        ND::from_array(self.data.sum_axis(Axis(axis)).into_dyn())
    }
```
```rust
    /// mean over axis (simple definition: sum/len)
    pub fn mean_axis(&self, axis: usize) -> ND {
        let denom = self.shape()[axis] as f64;
        ND::from_array((self.data.sum_axis(Axis(axis)) / denom).into_dyn())
    }
}
```
```rust
// --------------------
// 사칙연산 (element-wise, broadcasting은 ndarray에 위임)
// --------------------

impl Add for &ND {
    type Output = ND;
    fn add(self, rhs: Self) -> ND {
        ND::from_array((&self.data + &rhs.data).into_dyn())
    }
}
```
```rust
impl Sub for &ND {
    type Output = ND;
    fn sub(self, rhs: Self) -> ND {
        ND::from_array((&self.data - &rhs.data).into_dyn())
    }
}
```
```rust
impl Mul for &ND {
    type Output = ND;
    fn mul(self, rhs: Self) -> ND {
        ND::from_array((&self.data * &rhs.data).into_dyn())
    }
}
```
```rust
impl Div for &ND {
    type Output = ND;
    fn div(self, rhs: Self) -> ND {
        ND::from_array((&self.data / &rhs.data).into_dyn())
    }
}
```
```rust
// --------------------
// scalar ops (numpy처럼 a + 3.0, a * 2.0)
// --------------------

impl Add<f64> for &ND {
    type Output = ND;
    fn add(self, rhs: f64) -> ND {
        ND::from_array(self.data.mapv(|x| x + rhs))
    }
}
```
```rust
impl Sub<f64> for &ND {
    type Output = ND;
    fn sub(self, rhs: f64) -> ND {
        ND::from_array(self.data.mapv(|x| x - rhs))
    }
}
```
```rust
impl Mul<f64> for &ND {
    type Output = ND;
    fn mul(self, rhs: f64) -> ND {
        ND::from_array(self.data.mapv(|x| x * rhs))
    }
}
```
```rust
impl Div<f64> for &ND {
    type Output = ND;
    fn div(self, rhs: f64) -> ND {
        ND::from_array(self.data.mapv(|x| x / rhs))
    }
}
```
```rust
impl ND {
    /// NumPy: np.fromfunction(func, shape)
    /// func: |idx: &[usize]| -> f64
    pub fn from_function<F>(shape: &[usize], f: F) -> Self
    where
        F: Fn(&[usize]) -> f64,
    {
        let mut arr = ArrayD::<f64>::zeros(IxDyn(shape));
        for (idx, v) in arr.indexed_iter_mut() {
            *v = f(idx.slice());
        }
        ND { data: arr }
    }
}
```
```rust
impl ND {
    /// Create 2D ND from Vec<Vec<f64>>
    pub fn from_vec2(v: Vec<Vec<f64>>) -> Self {
        assert!(!v.is_empty(), "from_vec2: empty rows");
        let rows = v.len();
        let cols = v[0].len();
        assert!(cols > 0, "from_vec2: empty cols");
        assert!(
            v.iter().all(|r| r.len() == cols),
            "from_vec2: ragged rows"
        );

        let flat: Vec<f64> = v.into_iter().flatten().collect();
        let arr = Array2::from_shape_vec((rows, cols), flat)
            .expect("from_vec2: shape mismatch")
            .into_dyn();

        ND { data: arr }
    }
}
```
```rust
impl ND {
    pub fn from_slice2(v: &[&[f64]]) -> Self {
        assert!(!v.is_empty(), "from_slice2: empty rows");
        let rows = v.len();
        let cols = v[0].len();
        assert!(cols > 0, "from_slice2: empty cols");
        assert!(
            v.iter().all(|r| r.len() == cols),
            "from_slice2: ragged rows"
        );

        let mut flat = Vec::with_capacity(rows * cols);
        for row in v {
            flat.extend_from_slice(row);
        }

        let arr = Array2::from_shape_vec((rows, cols), flat)
            .expect("from_slice2: shape mismatch")
            .into_dyn();

        ND { data: arr }
    }
}
```
```rust
impl ND {
    pub fn from_flat_and_shape(flat: Vec<f64>, shape: &[usize]) -> Self {
        let arr = ArrayD::from_shape_vec(IxDyn(shape), flat)
            .expect("from_flat_and_shape: shape mismatch");
        ND { data: arr }
    }
}
```
```rust
impl ND {
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let v: Vec<f64> = iter.into_iter().collect();
        ND::from_vec(v)
    }
}
```
```rust
impl ND {
    pub fn print(&self) {
        // 1D
        if self.data.ndim() == 1 {
            let v: Vec<String> = self.data.iter().map(|x| format!("{}", x)).collect();
            println!("[{}]", v.join(", "));
            return;
        }

        // 2D
        if self.data.ndim() == 2 {
            let arr = self
                .data
                .view()
                .into_dimensionality::<Ix2>()
                .expect("print: not 2D");
            println!("[");
            for row in arr.rows() {
                let v: Vec<String> = row.iter().map(|x| format!("{}", x)).collect();
                println!("  [{}],", v.join(", "));
            }
            println!("]");
            return;
        }

        // fallback
        println!("{:?}", self.data);
    }
}
```
```rust
impl ND {
    pub fn inner(&self, other: &ND) -> f64 {
        let a = self
            .data
            .view()
            .into_dimensionality::<Ix1>()
            .expect("inner: left not 1D");
        let b = other
            .data
            .view()
            .into_dimensionality::<Ix1>()
            .expect("inner: right not 1D");

        if a.len() != b.len() {
            println!(
                "inner(): vectors must have the same length ({} vs {})",
                a.len(),
                b.len()
            );
            return 0.0;
        }

        a.dot(&b)
    }
}
```
```rust
impl ND {
    pub fn outer(&self, other: &ND) -> ND {
        let a = self.data.view().into_dimensionality::<Ix1>().unwrap();
        let b = other.data.view().into_dimensionality::<Ix1>().unwrap();

        let m = a.len();
        let n = b.len();

        // shape [m, 1], [1, n]
        let a2 = a.insert_axis(Axis(1));
        let b2 = b.insert_axis(Axis(0));

        // ndarray는 numpy처럼 자동 broadcasting 곱이 항상 되지 않으니
        // broadcast를 명시하고 Zip으로 element-wise 곱을 만든다.
        let a_bc = a2.broadcast((m, n)).expect("outer: broadcast a failed");
        let b_bc = b2.broadcast((m, n)).expect("outer: broadcast b failed");

        let out = ndarray::Zip::from(a_bc)
            .and(b_bc)
            .map_collect(|x, y| x * y);

        ND::from_array(out.into_dyn())
    }
}
```
```rust
impl ND {
    pub fn transpose(&self) -> ND {
        let t = self.data.clone().reversed_axes();
        ND { data: t }
    }
}
```
```rust
impl ND {
    pub fn flatten(&self) -> ND {
        let flat: Vec<f64> = self.data.iter().copied().collect();
        ND::from_vec(flat)
    }
}
```
```rust
impl ND {
    pub fn sort(&self) -> ND {
        let mut v: Vec<f64> = self.data.iter().copied().collect();
        // NaN 정책: NaN이 있으면 정렬 기준이 정의되지 않으므로 panic (A안: 명확하게)
        assert!(!v.iter().any(|x| x.is_nan()), "sort: contains NaN");
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ND::from_flat_and_shape(v, self.shape())
    }
}
```
```rust
impl ND {
    pub fn unique(&self) -> ND {
        let mut v: Vec<f64> = self.data.iter().copied().collect();
        // NaN 정책: (A안) NaN 있으면 panic
        assert!(!v.iter().any(|x| x.is_nan()), "unique: contains NaN");
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v.dedup_by(|a, b| a == b);
        ND::from_vec(v)
    }
}
```
```rust
impl ND {
    pub fn concatenate(a: &ND, b: &ND, axis: usize) -> ND {
        let ad = a.data.view();
        let bd = b.data.view();

        let out =
            ndarray::concatenate(Axis(axis), &[ad, bd]).expect("concatenate: shape mismatch");

        ND { data: out.into_dyn() }
    }
}
```
```rust
impl ND {
    pub fn argmax(&self) -> usize {
        let mut max = f64::NEG_INFINITY;
        let mut idx = 0;

        for (i, v) in self.data.iter().enumerate() {
            if *v > max {
                max = *v;
                idx = i;
            }
        }
        idx
    }
```
```rust
    pub fn argmin(&self) -> usize {
        let mut min = f64::INFINITY;
        let mut idx = 0;

        for (i, v) in self.data.iter().enumerate() {
            if *v < min {
                min = *v;
                idx = i;
            }
        }
        idx
    }
}
```
```rust
impl ND {
    pub fn ndim(&self) -> usize { self.data.ndim() }
```
```rust
    pub fn as_2d(&self) -> ndarray::ArrayView2<'_, f64> {
        self.data.view().into_dimensionality::<ndarray::Ix2>()
            .expect("as_2d: not 2D")
    }
```
```rust
    pub fn at2(&self, r: usize, c: usize) -> f64 {
        let a = self.as_2d();
        a[(r, c)]
    }
```
```rust
    pub fn row2(&self, r: usize) -> ND {
        let a = self.as_2d();
        ND::from_array(a.row(r).to_owned().into_dyn())
    }
```
```rust
    pub fn col2(&self, c: usize) -> ND {
        let a = self.as_2d();
        ND::from_array(a.column(c).to_owned().into_dyn())
    }
}
```
```rust

impl ND {
    pub fn take(&self, axis: usize, idx: &[usize]) -> ND {
        match self.ndim() {
            1 => {
                assert_eq!(axis, 0, "take: 1D axis must be 0");
                let a = self.data.view().into_dimensionality::<ndarray::Ix1>().unwrap();
                ND::from_array(a.select(Axis(0), idx).into_dyn())
            }
            2 => {
                let a = self.as_2d();
                match axis {
                    0 => ND::from_array(a.select(Axis(0), idx).into_dyn()),
                    1 => ND::from_array(a.select(Axis(1), idx).into_dyn()),
                    _ => panic!("take: invalid axis {axis} for 2D"),
                }
            }
            _ => panic!("take: only 1D/2D supported for DataFrame"),
        }
    }
```
```rust
    pub fn slice_axis(&self, axis: usize, r: std::ops::Range<usize>) -> ND {
        match self.ndim() {
            1 => {
                assert_eq!(axis, 0, "slice_axis: 1D axis must be 0");
                let a = self.data.view().into_dimensionality::<ndarray::Ix1>().unwrap();
                ND::from_array(a.slice(s![r]).to_owned().into_dyn())
            }
            2 => {
                let a = self.as_2d();
                match axis {
                    0 => ND::from_array(a.slice(s![r, ..]).to_owned().into_dyn()),
                    1 => ND::from_array(a.slice(s![.., r]).to_owned().into_dyn()),
                    _ => panic!("slice_axis: invalid axis {axis} for 2D"),
                }
            }
            _ => panic!("slice_axis: only 1D/2D supported for DataFrame"),
        }
    }
}
```
```rust
impl ND {
    pub fn mask_rows(&self, mask: &[bool]) -> ND {
        let a = self.as_2d();
        assert_eq!(a.nrows(), mask.len(), "mask_rows: len mismatch");
        let idx: Vec<usize> = mask.iter().enumerate().filter_map(|(i,&b)| b.then_some(i)).collect();
        ND::from_array(a.select(Axis(0), &idx).into_dyn())
    }
}
```
```rust
impl ND {
    pub fn is_nan(&self) -> Mask {
        Mask::from_array(self.data.mapv(|x| x.is_nan()))
    }
    pub fn fill_nan(&self, v: f64) -> ND {
        ND::from_array(self.data.mapv(|x| if x.is_nan() { v } else { x }))
    }
}
```
```rust
impl ND {
    pub fn argsort1(&self) -> Vec<usize> {
        let a = self.data.view().into_dimensionality::<ndarray::Ix1>()
            .expect("argsort1: not 1D");
        let mut idx: Vec<usize> = (0..a.len()).collect();
        assert!(!a.iter().any(|x| x.is_nan()), "argsort1: contains NaN");
        idx.sort_by(|&i, &j| a[i].partial_cmp(&a[j]).unwrap());
        idx
    }

    pub fn permute_rows2(&self, perm: &[usize]) -> ND {
        let a = self.as_2d();
        ND::from_array(a.select(Axis(0), perm).into_dyn())
    }
}
```
```rust

impl ND {
    /// 2D에서 r번째 row를 Vec로 복사
    pub fn row_vec2(&self, r: usize) -> Vec<f64> {
        self.as_2d().row(r).iter().copied().collect()
    }

    /// 2D에서 c번째 col을 Vec로 복사
    pub fn col_vec2(&self, c: usize) -> Vec<f64> {
        self.as_2d().column(c).iter().copied().collect()
    }

    /// 2D 전체를 row-major flat Vec로 복사
    pub fn to_vec2_row_major(&self) -> Vec<f64> {
        let a = self.as_2d();
        a.iter().copied().collect()
    }
}
```
---

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::nd::ND;
    #[test]
    fn test_boolean_mask_indexing() {
        // a = [10, 20, 30, 40]
        let a = ND::array(&[10.0, 20.0, 30.0, 40.0]);

        // mask = a > 25
        let mask = a.gt(25.0);

        // a[mask] -> [30, 40]
        let filtered = a.select(&mask);

        assert_eq!(filtered.shape(), &[2]);
        // ND::from_vec 사용했으니 1D에 대해 이렇게 비교 가능
        let expected = ND::array(&[30.0, 40.0]);
        // 간단하게 sum / mean으로도 검증할 수 있음
        assert!((filtered.sum() - expected.sum()).abs() < 1e-12);
        let data = filtered.to_vec();
        println!("{:?}", data);
    }
```
```rust
    #[test]
    fn test_zeros_ones_full_eye() {
        let z = ND::zeros(&[2, 3]);
        assert_eq!(z.shape(), &[2, 3]);
        assert!((z.sum() - 0.0).abs() < 1e-12);

        let o = ND::ones(&[2, 2]);
        assert_eq!(o.shape(), &[2, 2]);
        assert!((o.sum() - 4.0).abs() < 1e-12); // 2x2, all 1 → sum=4

        let f = ND::full(&[2, 2], 7.0);
        assert!((f.sum() - 28.0).abs() < 1e-12);

        let diag_sum = ND::diag_sum(3);
        assert!((diag_sum - 3.0).abs() < 1e-12);
    }
    #[test]
    fn test_arange_reshape_add() {
        let a = ND::arange(6).reshape(&[2, 3]);
        assert_eq!(a.shape(), &[2, 3]);
        // a = [[0,1,2],[3,4,5]] → sum=15
        assert!((a.sum() - 15.0).abs() < 1e-12);

        let b = ND::ones(&[2, 3]);
        let c = &a + &b; // element-wise add
        // c = a + 1 → sum=21
        assert!((c.sum() - 21.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_where_mask() {
        // a = [10, 20, 30]
        let a = ND::array(&[10.0, 20.0, 30.0]);
        let mask = a.gt(15.0); // [false, true, true]

        let x = ND::full(&[3], 1.0);
        let y = ND::full(&[3], -1.0);

        let out = ND::where_mask(&mask, &x, &y);
        // 기대: [-1, 1, 1]
        let expected = ND::array(&[-1.0, 1.0, 1.0]);
        assert!((out.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_take1() {
        let a = ND::array(&[10.0, 20.0, 30.0, 40.0]);
        let out = a.take1(&[1, 3]);
        let expected = ND::array(&[20.0, 40.0]);
        assert_eq!(out.shape(), &[2]);
        assert!((out.sum() - expected.sum()).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_case2 {
    use ndarray::{Ix2};
    use nurbslib::core::nd::*;
```
```rust
    // -----------------------------
    // 생성 함수 테스트
    // -----------------------------
    #[test]
    fn test_zeros_ones_full_empty_eye() {
        let z = ND::zeros(&[2, 3]);
        assert_eq!(z.shape(), &[2, 3]);
        assert!((z.sum() - 0.0).abs() < 1e-12);

        let o = ND::ones(&[2, 2]);
        assert_eq!(o.shape(), &[2, 2]);
        assert!((o.sum() - 4.0).abs() < 1e-12);

        let f = ND::full(&[2, 3], 7.0);
        assert_eq!(f.shape(), &[2, 3]);
        assert!((f.sum() - 42.0).abs() < 1e-12);

        let e = ND::empty(&[3, 3]);
        assert_eq!(e.shape(), &[3, 3]);
        assert!((e.sum() - 0.0).abs() < 1e-12);
        let diag_sum = ND::diag_sum(4);
         assert!((diag_sum - 4.0).abs() < 1e-12);
     }
```
```rust
    // -----------------------------
    // arange / linspace
    // -----------------------------
    #[test]
    fn test_arange_linspace() {
        let a = ND::arange(5);
        assert_eq!(a.shape(), &[5]);
        assert!((a.sum() - 10.0).abs() < 1e-12);

        let l = ND::linspace(0.0, 1.0, 5);
        assert_eq!(l.shape(), &[5]);
        assert!((l.sum() - 2.5).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // reshape
    // -----------------------------
    #[test]
    fn test_reshape() {
        let a = ND::arange(6).reshape(&[2, 3]);
        assert_eq!(a.shape(), &[2, 3]);
        assert!((a.sum() - 15.0).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // slice2 (2D slicing)
    // -----------------------------
    #[test]
    fn test_slice2() {
        let a = ND::arange(9).reshape(&[3, 3]);
        let s = a.slice2(0..2, 1..3);
        assert_eq!(s.shape(), &[2, 2]);
        assert!((s.sum() - (1.0 + 2.0 + 4.0 + 5.0)).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // Boolean mask indexing
    // -----------------------------
    #[test]
    fn test_boolean_mask_indexing() {
        let a = ND::array(&[10.0, 20.0, 30.0, 40.0]);
        let mask = a.gt(25.0);
        let out = a.select(&mask);

        let expected = ND::array(&[30.0, 40.0]);
        assert_eq!(out.shape(), &[2]);
        assert!((out.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // where_mask
    // -----------------------------
    #[test]
    fn test_where_mask() {
        let a = ND::array(&[10.0, 20.0, 30.0]);
        let mask = a.gt(15.0); // [false, true, true]

        let x = ND::full(&[3], 1.0);
        let y = ND::full(&[3], -1.0);

        let out = ND::where_mask(&mask, &x, &y);
        let expected = ND::array(&[-1.0, 1.0, 1.0]);

        assert_eq!(out.shape(), &[3]);
        assert!((out.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // take1
    // -----------------------------
    #[test]
    fn test_take1() {
        let a = ND::array(&[10.0, 20.0, 30.0, 40.0]);
        let out = a.take1(&[1, 3]);
        let expected = ND::array(&[20.0, 40.0]);

        assert_eq!(out.shape(), &[2]);
        assert!((out.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // 사칙연산 + broadcasting
    // -----------------------------
    #[test]
    fn test_elementwise_and_broadcast() {
        let a = ND::full(&[2, 3], 2.0);
        let b = ND::ones(&[2, 3]);

        println!("a {:?}", a);
        println!("b {:?}", b);

        let c = &a + &b;
        assert!((c.sum() - 18.0).abs() < 1e-12);

        let d = ND::array(&[1.0, 2.0, 3.0]).reshape(&[1, 3]);
        let e = &a + &d; // broadcasting
        assert_eq!(e.shape(), &[2, 3]);
        assert!((e.sum() - (2.0+1.0 + 2.0+2.0 + 2.0+3.0)*2.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_inner_outer()
    {

        let a = ND::array(&[1.,2.,3.]);
        let b = ND::array(&[4.,5.]);

        let inner = a.inner(&b); // 1*4 + 2*5 = 14
        let outer = a.outer(&b); // 3x2 matrix

        println!("inner {:?}", inner);
        println!("outer {:?}", outer);
    }
```
```rust
    #[test]
    fn test_matmul() {

        let a = ND::from_vec_2d(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = ND::from_vec_2d(2, 2, vec![2.0, 0.0, 1.0, 2.0]);

        let c = a.matmul(&b);
        let data = c.data();

        // c = [[1*2+2*1, 1*0+2*2],
        //      [3*2+4*1, 3*0+4*2]]
        assert!((data[[0, 0]] - 4.0).abs() < 1e-12);
        assert!((data[[0, 1]] - 4.0).abs() < 1e-12);
        assert!((data[[1, 0]] - 10.0).abs() < 1e-12);
        assert!((data[[1, 1]] - 8.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_from_function() {
        let f = ND::from_function(&[3, 3], |idx| (idx[0] + idx[1]) as f64);

        assert_eq!(f.shape(), &[3, 3]);
        assert!((f.sum() - 18.0).abs() < 1e-12);

        // spot check
        let arr = f.data().clone().into_dimensionality::<Ix2>().unwrap();
        assert!((arr[[0, 0]] - 0.0).abs() < 1e-12);
        assert!((arr[[1, 2]] - 3.0).abs() < 1e-12);
        assert!((arr[[2, 2]] - 4.0).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // sum / mean / std
    // -----------------------------
    #[test]
    fn test_statistics() {
        let a = ND::array(&[1.0, 2.0, 3.0, 4.0]);

        assert!((a.sum() - 10.0).abs() < 1e-12);
        assert!((a.mean() - 2.5).abs() < 1e-12);

        let std = a.std();
        assert!((std - 1.11803398875).abs() < 1e-9);
    }
```
```rust
    // -----------------------------
    // iter
    // -----------------------------
    #[test]
    fn test_iter() {
        let a = ND::array(&[1.0, 2.0, 3.0]);
        let mut sum = 0.0;
        for v in a.iter() {
            sum += *v;
        }
        assert!((sum - 6.0).abs() < 1e-12);
    }
```
```rust
    // -----------------------------
    // eq_nd
    // -----------------------------
    #[test]
    fn test_eq_nd() {
        let a = ND::array(&[1.0, 2.0, 3.0]);
        let b = ND::array(&[1.0, 2.0, 4.0]);

        let mask = a.eq_nd(&b);
        let out = a.select(&mask);

        assert_eq!(out.shape(), &[2]);
        assert!((out.sum() - 3.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_unique() {
        let a = ND::array(&[3., 1., 2., 3., 2.]);
        let u = a.unique();
        let expected = ND::array(&[1., 2., 3.]);
        assert_eq!(u.sum(), expected.sum());
    }
```
```rust
    #[test]
    fn test_transpose() {
        let a = ND::from_vec2(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);

        let t = a.transpose();

        assert_eq!(t.shape(), &[3, 2]);

        let expected = ND::from_vec2(vec![
            vec![1.0, 4.0],
            vec![2.0, 5.0],
            vec![3.0, 6.0],
        ]);

        assert!((t.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_flatten() {
        let a = ND::from_vec2(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ]);

        let f = a.flatten();

        assert_eq!(f.shape(), &[4]);
        assert_eq!(f.sum(), 10.0);
    }
```
```rust
    #[test]
    fn test_sort() {
        let a = ND::array(&[3.0, 1.0, 4.0, 2.0]);
        let s = a.sort();

        let expected = ND::array(&[1.0, 2.0, 3.0, 4.0]);

        assert_eq!(s.shape(), &[4]);
        assert!((s.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_concatenate_1d() {
        let a = ND::array(&[1.0, 2.0, 3.0]);
        let b = ND::array(&[4.0, 5.0]);

        let c = ND::concatenate(&a, &b, 0);

        let expected = ND::array(&[1.0, 2.0, 3.0, 4.0, 5.0]);

        assert_eq!(c.shape(), &[5]);
        assert!((c.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_concatenate_2d() {
        let a = ND::from_vec2(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ]);

        let b = ND::from_vec2(vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ]);

        let c = ND::concatenate(&a, &b, 0);

        assert_eq!(c.shape(), &[4, 2]);
        assert!((c.sum() - 36.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_argmax() {
        let a = ND::array(&[3.0, 1.0, 5.0, 2.0]);
        assert_eq!(a.argmax(), 2);
    }
```
```rust
    #[test]
    fn test_argmin() {
        let a = ND::array(&[3.0, 1.0, 5.0, 2.0]);
        assert_eq!(a.argmin(), 1);
    }
```
```rust
    #[test]
    fn test_outer() {
        let a = ND::array(&[1.0, 2.0, 3.0]);
        let b = ND::array(&[4.0, 5.0]);

        let out = a.outer(&b);

        let expected = ND::from_vec2(vec![
            vec![4.0, 5.0],
            vec![8.0, 10.0],
            vec![12.0, 15.0],
        ]);

        assert_eq!(out.shape(), &[3, 2]);
        assert!((out.sum() - expected.sum()).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_inner_mismatch() {
        let a = ND::array(&[1.0, 2.0, 3.0]);
        let b = ND::array(&[4.0, 5.0]);
        let _ = a.inner(&b); // panic expected
    }

}
```
```rust
#[cfg(test)]
mod tests_nd_for_pandas {
    use nurbslib::core::nd::ND;
    use nurbslib::core::pandas::{CsvOptions, DataFrame};
    use super::*;

    fn nd2() -> ND {
        // 3x4:
        // [ 0  1  2  3
        //   4  5  6  7
        //   8  9 10 11 ]
        ND::from_vec_2d(3, 4, (0..12).map(|x| x as f64).collect())
    }

    fn assert_vec_eq(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert!(
                (a[i] - b[i]).abs() <= 0.0,
                "mismatch at {i}: {} vs {}",
                a[i],
                b[i]
            );
        }
    }
```
```rust
    #[test]
    fn nd_as_2d_at2_row2_col2() {
        let a = nd2();
        assert_eq!(a.ndim(), 2);

        // at2
        assert_eq!(a.at2(0, 0), 0.0);
        assert_eq!(a.at2(0, 3), 3.0);
        assert_eq!(a.at2(2, 0), 8.0);
        assert_eq!(a.at2(2, 3), 11.0);

        // row2 -> 1D ND
        let r1 = a.row2(1);
        assert_eq!(r1.ndim(), 1);
        assert_vec_eq(&r1.to_vec_fallback(), &[4.0, 5.0, 6.0, 7.0]);

        // col2 -> 1D ND
        let c2 = a.col2(2);
        assert_eq!(c2.ndim(), 1);
        assert_vec_eq(&c2.to_vec_fallback(), &[2.0, 6.0, 10.0]);

        // as_2d shape
        let v = a.as_2d();
        assert_eq!(v.nrows(), 3);
        assert_eq!(v.ncols(), 4);
    }
```
```rust
    #[test]
    fn nd_take_2d_axis0_axis1() {
        let a = nd2();

        // take rows: [2,0] -> rows 2 then 0
        let t0 = a.take(0, &[2, 0]);
        let v0 = t0.as_2d();
        assert_eq!(v0.nrows(), 2);
        assert_eq!(v0.ncols(), 4);
        assert_vec_eq(v0.row(0).as_slice().unwrap(), &[8.0, 9.0, 10.0, 11.0]);
        assert_vec_eq(v0.row(1).as_slice().unwrap(), &[0.0, 1.0, 2.0, 3.0]);

        // take cols: [3,1]
        let t1 = a.take(1, &[3, 1]);
        let v1 = t1.as_2d();
        assert_eq!(v1.nrows(), 3);
        assert_eq!(v1.ncols(), 2);
        // col 0 = old col 3, col 1 = old col 1
        assert_vec_eq(v1.column(0).as_slice().unwrap(), &[3.0, 7.0, 11.0]);
        assert_vec_eq(v1.column(1).as_slice().unwrap(), &[1.0, 5.0, 9.0]);
    }
```
```rust
    fn assert_iter_eq<I>(it: I, expected: &[f64])
    where
        I: IntoIterator<Item = f64>,
    {
        let got: Vec<f64> = it.into_iter().collect();
        assert_eq!(got.len(), expected.len());
        for i in 0..got.len() {
            assert!(
                (got[i] - expected[i]).abs() <= 0.0,
                "mismatch at {i}: {} vs {}",
                got[i],
                expected[i]
            );
        }
    }
```
```rust
    #[test]
    fn nd_slice_axis_2d_rows_cols() {
        let a = nd2();

        // rows 1..3 => rows [1,2]
        let sr = a.slice_axis(0, 1..3);
        let vr = sr.as_2d();
        assert_eq!(vr.nrows(), 2);
        assert_eq!(vr.ncols(), 4);

        assert_iter_eq(vr.row(0).iter().copied(), &[4.0, 5.0, 6.0, 7.0]);
        assert_iter_eq(vr.row(1).iter().copied(), &[8.0, 9.0, 10.0, 11.0]);

        // cols 0..2 => cols [0,1]
        let sc = a.slice_axis(1, 0..2);
        let vc = sc.as_2d();
        assert_eq!(vc.nrows(), 3);
        assert_eq!(vc.ncols(), 2);

        assert_iter_eq(vc.column(0).iter().copied(), &[0.0, 4.0, 8.0]);
        assert_iter_eq(vc.column(1).iter().copied(), &[1.0, 5.0, 9.0]);
    }
```
```rust
    #[test]
    fn nd_mask_rows_basic() {
        let a = nd2();

        // keep rows 0 and 2
        let mask = vec![true, false, true];
        let m = a.mask_rows(&mask);
        let v = m.as_2d();
        assert_eq!(v.nrows(), 2);
        assert_eq!(v.ncols(), 4);
        assert_vec_eq(v.row(0).as_slice().unwrap(), &[0.0, 1.0, 2.0, 3.0]);
        assert_vec_eq(v.row(1).as_slice().unwrap(), &[8.0, 9.0, 10.0, 11.0]);
    }
```
```rust
    #[test]
    fn nd_is_nan_and_fill_nan() {
        // 2x3
        let mut v = vec![1.0, f64::NAN, 3.0, 4.0, 5.0, f64::NAN];
        let a = ND::from_vec_2d(2, 3, v.clone());

        let nan_mask = a.is_nan();
        assert_eq!(nan_mask.shape(), &[2, 3]);

        // mask 내용 검사 (flatten order)
        let bm: Vec<bool> = nan_mask.data.iter().copied().collect();
        assert_eq!(bm, vec![false, true, false, false, false, true]);

        let b = a.fill_nan(-9.0);
        let out = b.as_2d();
        assert_eq!(out[(0, 1)], -9.0);
        assert_eq!(out[(1, 2)], -9.0);
        assert_eq!(out[(0, 0)], 1.0);
        assert_eq!(out[(1, 1)], 5.0);
    }
```
```rust
    #[test]
    fn nd_argsort1_and_permute_rows2() {
        // argsort1
        let s = ND::from_vec(vec![3.0, -1.0, 2.0, 2.0, 10.0]);
        let idx = s.argsort1();
        // stable 보장 안 해도 되지만, 값 기준으로 맞는지만 확인
        // expected sorted values: [-1,2,2,3,10]
        let sorted: Vec<f64> = idx.iter().map(|&i| s.data().iter().copied().nth(i).unwrap()).collect();
        assert_vec_eq(&sorted, &[-1.0, 2.0, 2.0, 3.0, 10.0]);

        // permute_rows2
        let a = nd2(); // 3x4
        let p = vec![2usize, 0usize, 1usize];
        let b = a.permute_rows2(&p);
        let vb = b.as_2d();
        assert_vec_eq(vb.row(0).as_slice().unwrap(), &[8.0, 9.0, 10.0, 11.0]); // old row2
        assert_vec_eq(vb.row(1).as_slice().unwrap(), &[0.0, 1.0, 2.0, 3.0]);   // old row0
        assert_vec_eq(vb.row(2).as_slice().unwrap(), &[4.0, 5.0, 6.0, 7.0]);   // old row1
    }
}
```
---
