# Wavelet denoising

Wavelet denoising은 신호나 영상에서 잡음을 제거하기 위해 웨이블릿 변환을 활용하는 기법입니다.  
핵심은 웨이블릿 변환이 신호의 중요한 특징을 소수의 큰 계수에 집중시키고, 잡음은 작은 계수로 나타난다는 점을 이용해  
작은 계수를 줄이거나 제거한 뒤 역변환으로 복원하는 것입니다.  

## 📌 기본 원리
- Wavelet 변환
  - 입력 신호나 이미지를 웨이블릿 기저로 분해 → 다중 해상도 표현을 얻음.
  - 신호의 중요한 구조(에지, 스파이크)는 큰 계수로 나타남.
  - 잡음은 작은 계수에 분산되어 나타남.
  - Thresholding (임계값 처리)
  - 작은 계수는 잡음으로 간주 → 제거 또는 축소.
- 방식:
  - Hard thresholding: 임계값 이하 계수는 0으로 설정.
  - Soft thresholding: 임계값 이하 계수는 0, 초과 계수는 임계값만큼 줄임.
  - Inverse Wavelet Transform (역변환)
  - 수정된 계수로 원래 신호를 재구성.
- 결과: 잡음은 줄고, 중요한 특징은 보존된 신호/영상.

## 🧮 대표적인 방법
- VisuShrink: 모든 웨이블릿 세부 계수에 동일한 임계값 적용. 단순하지만 과도한 평활화 가능.
- BayesShrink: 각 서브밴드마다 다른 임계값을 적응적으로 설정. 더 정교하고 잡음 억제 성능이 좋음.

## ✨ 장점
- 에지 보존: Fourier 기반 저역통과 필터링은 에지를 흐리게 하지만, Wavelet은 에지를 유지하면서 잡음을 줄임.
- 다중 해상도 분석: 신호의 다양한 스케일에서 잡음을 제거 가능.
- 비균일 잡음 처리: 구간별로 다른 threshold를 적용해 잡음이 불균일한 경우에도 대응 가능.

## 📌 활용 분야
- 영상 처리: 사진, 의료 영상(CT, MRI, 초음파)에서 잡음 제거.
- 신호 처리: 음성, 전자기 신호, 센서 데이터의 노이즈 억제.
- 데이터 분석: 금융 시계열, 지진파 등에서 잡음 제거 후 특징 추출.

- 👉 정리하면, Wavelet denoising은 웨이블릿 변환으로 신호를 분해한 뒤 작은 계수를 제거/축소하여 잡음을 줄이고,  
  역변환으로 복원하는 적응형 잡음 제거 기법입니다.  
  Fourier 기반 필터링보다 에지와 구조를 잘 보존할 수 있어 영상·신호 처리에서 널리 쓰입니다.

---
## 코드 이론 설명

Rust로 작성해드린 Wavelet denoising 코드의 함수와 수식을 전반적으로 정리해서 설명.  
이 문서는 **어떤 함수가 어떤 역할을 하는지** 와 **수식적으로 어떤 처리를 하는지** 를 연결해 이해할 수 있도록 구성했습니다.

## 📘 Wavelet Denoising 코드 정리
### 1. 기본 개념
- Wavelet 변환(DWT): 신호를 저주파(approximation)와 고주파(detail) 성분으로 분해
- Thresholding: detail 성분에서 작은 계수를 제거 → 잡음 억제
- 역변환(IDWT): 수정된 계수로 원래 신호를 복원 → 잡음 줄고 구조 보존

### 2. 주요 함수와 수식
### 2.1 get_wavelet_filters(kind)
- 역할: Haar, Daubechies-2(db2) 필터 계수 반환
- 수식 (Haar 예시):
- Haar (orthonormal)

$$
h=\left[ \frac{1}{\sqrt{2}},\  \frac{1}{\sqrt{2}}\right] ,\quad g=\left[ -\frac{1}{\sqrt{2}},\  \frac{1}{\sqrt{2}}\right]
$$

- 재구성 필터는 동일 계수(직교 정규 기저에서 $h_r=h,\  g_r=g$)입니다.
- Daubechies-2 (db2, 길이 4)

$$
h=\left[ \frac{1+\sqrt{3}}{4\sqrt{2}},\  \frac{3+\sqrt{3}}{4\sqrt{2}},\  \frac{3-\sqrt{3}}{4\sqrt{2}},\  \frac{1-\sqrt{3}}{4\sqrt{2}}\right] \quad g=\left[ h_3,\  -h_2,\  h_1,\  -h_0\right]
$$

  - 직교 정규 db2의 재구성 필터도 동일 계수로 사용합니다: $h_r=h,\  g_r=g$.

### 2.2 conv_periodic(x, f, shift)
- 역할: 주어진 필터 f로 신호 x를 순환 경계 조건(periodic)으로 convolution
- 수식:
  - 입력 신호 길이가 $N$, 필터 길이가 $M$ 일 때, 순환(주기) 경계 조건 하의 컨볼루션 출력

$$ 
y[i] = sum_{k=0}^{M-1} f[k] * x[(i - shift - k) mod N], \quad  i = 0,1,...,N-1
$$

### 2.3 dwt_1d(x, kind)
- 역할: 1D 신호를 저주파(approx)와 고주파(detail)로 분해
- 수식:

$$
a[n]=\sum h[k]\cdot x[2n-k],\quad d[n]=\sum g[k]\cdot x[2n-k]
$$

### 2.4 idwt_1d(approx, detail, kind)
- 역할: 1D 역변환, approximation과 detail을 합쳐 원래 신호 복원
- 수식:

$$
x[i] = sum_{k=0}^{M-1} hr[k] * a[(i - k)/2] + sum_{k=0}^{M-1} gr[k] * d[(i - k)/2]
$$

- 조건:
  - i = 0,1,...,N-1
  - (i - k)/2 는 짝수 인덱스에서만 유효 (upsampling 과정에서 0 삽입)
  - M = 필터 길이

### 2.5 estimate_sigma(detail)
- 역할: detail 계수로 잡음 표준편차 추정 (MAD 기반)
- 수식:

$$
\sigma \approx \frac{\mathrm{median}(|d|)}{0.6745}
$$


### 2.6 threshold_coeffs(detail, thr, kind)
- 역할: detail 계수에 threshold 적용
- Hard threshold:

$$
d_i =
\begin{cases}
d_i, & |d_i| \geq T \\
0,   & |d_i| < T
\end{cases}
$$


- Soft threshold:

$$
d_i=\mathrm{sign}(d_i)\cdot \max (|d_i|-T,0)
$$


### 2.7 denoise_1d(x, kind, tkind)
- 역할: 1D 신호 denoising 전체 파이프라인
- 수식 (Universal threshold):

$$
T=\sigma \cdot \sqrt{2\ln N}
$$

- (N: 신호 길이)

### 2.8 dwt2(img, kind)- 역할: 2D DWT (행 → 열 순서로 separable 변환)
- 출력: LL (저주파), LH (행 detail), HL (열 detail), HH (대각 detail)
### 2.9 idwt2(ll, lh, hl, hh, kind)
- 역할: 2D 역변환, 네 서브밴드로 원래 이미지 복원
#### 2.10 denoise_2d(img, kind, tkind)
- 역할: 2D 영상 denoising 전체 파이프라인
- 수식 (Universal threshold):

$$
T=\sigma \cdot \sqrt{2\ln (MN)}
$$

- (M×N: 영상 크기)
  
## 3. 전체 흐름- Wavelet 변환: DWT로 신호/영상 분해
- Noise level 추정: detail 계수로 σ 추정
- Thresholding: 작은 detail 계수 제거
- 역변환: IDWT로 복원
- 결과: 잡음은 줄고 구조는 보존된 신호/영상
## 4. AI 활용 포인트- 전처리: 잡음 억제된 데이터를 AI 모델에 입력 → 학습 안정성 향상
- Feature engineering: detail 계수 자체를 feature로 활용 → 이상 탐지, 두께 변화 검출
- 실시간성: Rust 구현으로 로봇 센서 데이터 처리에 적합

---

## 소스 코드
```rust
use ndarray::{Array1, Array2, Axis, s};
use ndarray::prelude::*;
use std::f64;

/// Wavelet filters (Haar, Daubechies-2)
#[derive(Clone, Copy)]
pub enum WaveletKind {
    Haar,
    Db2,
}
```
```rust
fn get_wavelet_filters(kind: WaveletKind) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    match kind {
        WaveletKind::Haar => {
            // Haar scaling/low-pass h and wavelet/high-pass g
            let h = vec![1.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)];
            let g = vec![-1.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)];
            // Reconstruction filters for orthonormal Haar are same up to sign
            let hr = h.clone();
            let gr = g.clone();
            (h, g, hr, gr)
        }
        WaveletKind::Db2 => {
            // Daubechies-2 (length 4)
            // scaling low-pass (h)
            let h = vec![
                (1.0 + f64::sqrt(3.0)) / (4.0 * f64::sqrt(2.0)),
                (3.0 + f64::sqrt(3.0)) / (4.0 * f64::sqrt(2.0)),
                (3.0 - f64::sqrt(3.0)) / (4.0 * f64::sqrt(2.0)),
                (1.0 - f64::sqrt(3.0)) / (4.0 * f64::sqrt(2.0)),
            ];
            // wavelet high-pass (g) from QMF relation
            let g = vec![
                h[3], -h[2], h[1], -h[0],
            ];
            // reconstruction filters (for orthonormal db2)
            let hr = h.clone();
            let gr = g.clone();
            (h, g, hr, gr)
        }
    }
}
```
```rust
/// Periodic extension indexing
fn pidx(i: isize, n: usize) -> usize {
    let n_is = n as isize;
    let mut j = i % n_is;
    if j < 0 { j += n_is; }
    j as usize
}
```
```rust
/// 1D convolution with periodic boundary
fn conv_periodic(x: &[f64], f: &[f64], shift: isize) -> Vec<f64> {
    let n = x.len();
    let m = f.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut acc = 0.0;
        for k in 0..m {
            let xi = pidx(i as isize - shift - k as isize, n);
            acc += x[xi] * f[k];
        }
        y[i] = acc;
    }
    y
}
```
```rust
/// MAD-based sigma estimate (detail coefficients)
pub fn estimate_sigma(detail: &[f64]) -> f64 {
    if detail.is_empty() { return 0.0; }
    let mut abs_vals: Vec<f64> = detail.iter().map(|v| v.abs()).collect();
    abs_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = abs_vals.len() / 2;
    let mad = if abs_vals.len() % 2 == 0 {
        (abs_vals[mid - 1] + abs_vals[mid]) / 2.0
    } else {
        abs_vals[mid]
    };
    mad / 0.6745
}
```
```rust
#[derive(Clone, Copy)]
pub enum ThresholdKind {
    Hard,
    Soft,
}
```
```rust
/// Thresholding
pub fn threshold_coeffs(detail: &mut [f64], thr: f64, kind: ThresholdKind) {
    match kind {
        ThresholdKind::Hard => {
            for v in detail.iter_mut() {
                if v.abs() < thr {
                    *v = 0.0;
                }
            }
        }
        ThresholdKind::Soft => {
            for v in detail.iter_mut() {
                let s = v.signum();
                let a = v.abs();
                *v = if a <= thr { 0.0 } else { s * (a - thr) };
            }
        }
    }
}
```
```rust
/// 1D denoise (single-level VisuShrink)
pub fn denoise_1d(x: &[f64], kind: WaveletKind, tkind: ThresholdKind) -> Vec<f64> {
    let (mut a, mut d) = dwt_1d(x, kind);
    let sigma = estimate_sigma(&d);
    let thr = sigma * (2.0 * (x.len() as f64).ln()).sqrt(); // universal threshold
    threshold_coeffs(&mut d, thr, tkind);
    idwt_1d(&a, &d, kind)
}
```
```rust
/// 2D DWT (single-level) on Array2<f64>, periodic boundary
pub fn dwt2(img: &Array2<f64>, kind: WaveletKind)
            -> (Array2<f64>, Array2<f64>, Array2<f64>, Array2<f64>)
{
    let (rows, cols) = img.dim();
    assert!(rows % 2 == 0 && cols % 2 == 0, "even size only for now");

    // 1단계: 각 행에 1D DWT → 앞 절반에 approx, 뒤 절반에 detail
    let mut tmp = Array2::<f64>::zeros((rows, cols));
    for r in 0..rows {
        let row = img.slice(s![r, ..]).to_owned().to_vec();
        let (a, d) = dwt_1d(&row, kind);
        let half = cols / 2;
        for c in 0..half {
            tmp[(r, c)] = a[c];
            tmp[(r, half + c)] = d[c];
        }
    }

    // 2단계: 각 열에 1D DWT → 위 절반 approx, 아래 절반 detail
    let mut coeff = Array2::<f64>::zeros((rows, cols));
    for c in 0..cols {
        let col: Vec<f64> = (0..rows).map(|r| tmp[(r, c)]).collect();
        let (a, d) = dwt_1d(&col, kind);
        let half = rows / 2;
        for r in 0..half {
            coeff[(r, c)] = a[r];
            coeff[(half + r, c)] = d[r];
        }
    }

    let half_r = rows / 2;
    let half_c = cols / 2;

    let ll = coeff.slice(s![0..half_r,       0..half_c]).to_owned();
    let lh = coeff.slice(s![0..half_r,       half_c..cols]).to_owned();
    let hl = coeff.slice(s![half_r..rows,    0..half_c]).to_owned();
    let hh = coeff.slice(s![half_r..rows,    half_c..cols]).to_owned();

    (ll, lh, hl, hh)
}
```
```rust
/// 2D IDWT (single-level) reconstruct Array2<f64>
pub fn idwt2(
    ll: &Array2<f64>,
    lh: &Array2<f64>,
    hl: &Array2<f64>,
    hh: &Array2<f64>,
    kind: WaveletKind,
) -> Array2<f64> {
    let (half_r, half_c) = ll.dim();
    let rows = half_r * 2;
    let cols = half_c * 2;

    // 4개 subband 를 다시 하나의 packed coeff 행렬로 합치기
    let mut coeff = Array2::<f64>::zeros((rows, cols));
    coeff
        .slice_mut(s![0..half_r, 0..half_c])
        .assign(ll);
    coeff
        .slice_mut(s![0..half_r, half_c..cols])
        .assign(lh);
    coeff
        .slice_mut(s![half_r..rows, 0..half_c])
        .assign(hl);
    coeff
        .slice_mut(s![half_r..rows, half_c..cols])
        .assign(hh);

    // 1단계: 각 열에 대해 1D 역변환
    let mut tmp = Array2::<f64>::zeros((rows, cols));
    for c in 0..cols {
        let mut a = Vec::with_capacity(half_r);
        let mut d = Vec::with_capacity(half_r);
        for r in 0..half_r {
            a.push(coeff[(r, c)]);
            d.push(coeff[(half_r + r, c)]);
        }
        let col_rec = idwt_1d(&a, &d, kind);
        for r in 0..rows {
            tmp[(r, c)] = col_rec[r];
        }
    }

    // 2단계: 각 행에 대해 1D 역변환
    let mut img = Array2::<f64>::zeros((rows, cols));
    for r in 0..rows {
        let mut a = Vec::with_capacity(half_c);
        let mut d = Vec::with_capacity(half_c);
        for c in 0..half_c {
            a.push(tmp[(r, c)]);
            d.push(tmp[(r, half_c + c)]);
        }
        let row_rec = idwt_1d(&a, &d, kind);
        for c in 0..cols {
            img[(r, c)] = row_rec[c];
        }
    }

    img
}
```
```rust
/// 2D denoise (single-level): threshold LH, HL, HH bands and reconstruct
pub fn denoise_2d(img: &Array2<f64>, kind: WaveletKind, tkind: ThresholdKind) -> Array2<f64> {
    let (ll, mut lh, mut hl, mut hh) = dwt2(img, kind);

    // Estimate sigma from HH band (most noise-dominant)
    let mut hh_vec: Vec<f64> = hh.iter().copied().collect();
    let sigma = estimate_sigma(&hh_vec);
    let n = (img.dim().0 * img.dim().1) as f64;
    let thr = sigma * (2.0 * n.ln()).sqrt(); // universal threshold for 2D (simple heuristic)

    // Threshold detail subbands
    for v in lh.iter_mut() {
        let s = v.signum();
        let a = v.abs();
        *v = if a <= thr { 0.0 } else { match tkind { ThresholdKind::Soft => s * (a - thr), ThresholdKind::Hard => *v } };
        if let ThresholdKind::Hard = tkind {
            if a < thr { *v = 0.0; }
        }
    }
    for v in hl.iter_mut() {
        let s = v.signum();
        let a = v.abs();
        *v = if a <= thr { 0.0 } else { match tkind { ThresholdKind::Soft => s * (a - thr), ThresholdKind::Hard => *v } };
        if let ThresholdKind::Hard = tkind {
            if a < thr { *v = 0.0; }
        }
    }
    for v in hh.iter_mut() {
        let s = v.signum();
        let a = v.abs();
        *v = if a <= thr { 0.0 } else { match tkind { ThresholdKind::Soft => s * (a - thr), ThresholdKind::Hard => *v } };
        if let ThresholdKind::Hard = tkind {
            if a < thr { *v = 0.0; }
        }
    }

    idwt2(&ll, &lh, &hl, &hh, kind)
}
```
```rust
pub fn dwt_1d(x: &[f64], kind: WaveletKind) -> (Vec<f64>, Vec<f64>) {
    match kind {
        WaveletKind::Haar => {
            let n = x.len();
            assert!(n % 2 == 0, "Haar: length must be even");
            let half = n / 2;
            let mut a = vec![0.0; half];
            let mut d = vec![0.0; half];
            let s = std::f64::consts::SQRT_2;
            for k in 0..half {
                let x0 = x[2 * k];
                let x1 = x[2 * k + 1];
                a[k] = (x0 + x1) / s;
                d[k] = (x0 - x1) / s;
            }
            (a, d)
        }
        WaveletKind::Db2 => {
            let (h, g, _hr, _gr) = get_wavelet_filters(WaveletKind::Db2);
            let n = x.len();
            let lp = conv_periodic(x, &h, 0);
            let hp = conv_periodic(x, &g, 0);
            let mut approx = Vec::with_capacity((n + 1) / 2);
            let mut detail = Vec::with_capacity((n + 1) / 2);
            for i in (0..n).step_by(2) {
                approx.push(lp[i]);
                detail.push(hp[i]);
            }
            (approx, detail)
        }
    }
}
```
```rust
pub fn idwt_1d(approx: &[f64], detail: &[f64], kind: WaveletKind) -> Vec<f64> {
    match kind {
        WaveletKind::Haar => {
            assert_eq!(approx.len(), detail.len());
            let half = approx.len();
            let mut y = vec![0.0; 2 * half];
            let s = std::f64::consts::SQRT_2;
            for k in 0..half {
                let a = approx[k];
                let d = detail[k];
                y[2 * k] = (a + d) / s;
                y[2 * k + 1] = (a - d) / s;
            }
            y
        }
        WaveletKind::Db2 => {
            let (_h, _g, hr, gr) = get_wavelet_filters(WaveletKind::Db2);
            let up_len = approx.len() * 2;
            let mut a_up = vec![0.0; up_len];
            let mut d_up = vec![0.0; up_len];
            for i in 0..approx.len() {
                a_up[2 * i] = approx[i];
                d_up[2 * i] = detail[i];
            }
            let a_rec = conv_periodic(&a_up, &hr, 0);
            let d_rec = conv_periodic(&d_up, &gr, 0);
            a_rec
                .iter()
                .zip(d_rec.iter())
                .map(|(aa, dd)| aa + dd)
                .collect()
        }
    }
}
```

---

