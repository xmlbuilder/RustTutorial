# Physical AI + SVD + SAR(합성개구레이더) 이미지 처리
- 이 조합은 실제로 연구·산업에서 이미 핵심 기술로 쓰이고 있음.
- 그리고 검색 결과에서도 SAR 이미지 처리에서 SVD 기반 기법이 활발히 연구되고 있다는 걸 확인할 수 있음.
- 아래에서 왜 SVD가 Physical AI와 SAR에서 중요한지, 그리고 어디에 쓰이는지 단계별로 정리.

## 🌍 1. Physical AI에서 SVD가 중요한 이유
- Physical AI는 “물리 세계를 이해하고 조작하는 AI”를 의미.
- 즉, 센서 → 물리 신호 → AI 모델 → 행동/판단 흐름 전체를 다루는 분야.
- 여기서 SVD는 다음 두 가지 역할을 한다:
- ✔ 1) 노이즈 제거(denoising)
    - 물리 센서 데이터는 항상 노이즈가 많음.
    - SVD는 작은 특이값 = 노이즈 방향을 제거해주기 때문에 센서 신호를 깨끗하게 만드는 데 최적.
- ✔ 2) 차원 축소(feature extraction)
    - 물리 신호는 고차원인데, 실제 의미 있는 정보는 몇 개의 축에만 존재.
    - SVD는 그 “핵심 방향”을 자동으로 찾음.
- 즉,
    - Physical AI는 SVD를 통해 물리 신호의 본질만 남기고 AI가 처리하기 쉬운 형태로 바꾼다.


## 🛰️ 2. SAR 이미지 처리에서 SVD가 중요한 이유
- SAR(Synthetic Aperture Radar)은 위성·드론에서 쓰는 레이더 기반 이미지 기술.
- 문제는 SAR 이미지가:
    - 스페클 노이즈(speckle noise) 많고
    - 해상도 낮고
    - 구조가 복잡하고
    - 광학 이미지와 달리 “물리적 반사 패턴”을 가진다는 점
- 그래서 SAR 이미지는 전통적인 이미지 처리보다 훨씬 어려워서 SVD가 강력하게 쓰인다.

## 🔥 3. SAR 이미지 처리에서 SVD가 실제로 쓰이는 분야
- ✔ 1) SAR 노이즈 제거 (Speckle Noise Reduction)
    - SAR의 가장 큰 문제는 스페클 노이즈인데 SVD는 작은 특이값을 제거함으로써 노이즈를 효과적으로 줄임.
    - 큰 특이값 → 실제 지형/물체 구조
    - 작은 특이값 → 스페클 노이즈
- 그래서 SVD 기반 필터링은 SAR에서 매우 강력한 기법으로 알려져 있음.

- ✔ 2) SAR 특징 추출 (Feature Extraction)
- SAR 이미지에서 물체(탱크, 건물, 도로 등)를 인식하려면 **특징(feature)** 을 잘 뽑아야 함
- SVD는 다음을 자동으로 해줌:
    - 주요 방향(Principal Axes)
    - 반사 패턴의 주요 성분
    - 물체의 고유 구조
- IEEE 논문에서도 SVD 기반 SAR 특징 재구성 기법이 소개돼 있음.

- ✔ 3) SAR 자동 표적 인식(ATR: Automatic Target Recognition)
- SAR ATR은 군사·위성 분야에서 매우 중요한 기술.
- SVD는 다음에 쓰여:
    - 표적의 signature 분해
    - 배경과 표적 분리
    - 차원 축소 후 딥러닝 입력으로 사용
- 즉, SVD는 SAR 표적 인식의 전처리 핵심 기술.

- ✔ 4) SAR 이미지 압축 및 복원
    - SAR 데이터는 매우 크기 때문에 SVD 기반 압축이 많이 쓰임.
    - 큰 특이값만 남기면 압축 다시 복원하면 원본 구조 유지
    - SAR 데이터 전송·저장에 매우 유용.

- ✔ 5) SAR + AI 융합 모델에서 SVD는 “물리적 의미”를 제공
- SAR은 물리 기반 신호이기 때문에 AI 모델이 단순히 픽셀을 보는 것보다
    물리적 구조를 반영한 특징을 쓰는 게 훨씬 좋음.
- SVD는 그 물리적 구조를 잘 반영.
- MDPI Remote Sensing에서도 AI 기반 SAR 해석에서 SVD 같은 물리 기반 기법이 중요하다고 언급돼 있음.

## 🧠 4. Physical AI + SAR + SVD = 어떤 시너지가 생길까?
- ✔ 1) 물리 기반 특징 + AI 기반 분류
- SVD로 물리적 구조를 추출하고 AI가 그걸 학습하면 훨씬 안정적이고 정확한 모델이 된다.
- ✔ 2) 노이즈가 많은 SAR에서도 안정적인 AI 모델
    - SAR은 노이즈가 심해서 AI가 그대로 학습하면 오히려 성능이 떨어짐.
    - SVD로 노이즈를 제거하면 AI가 훨씬 잘 학습한다.
- ✔ 3) 데이터 부족 문제 해결
- SAR은 데이터가 적은 경우가 많음.
- SVD 기반 특징 추출은 데이터가 적어도 안정적인 모델을 만들 수 있음.
- IEEE 논문에서도 **SVD 기반 특징 재구성은 SAR 데이터 부족 문제를 해결하는 데 유용하다** 고 언급.

## 🌟 한 줄 요약
- Physical AI에서 SVD는 물리 신호의 핵심 구조를 추출하고 노이즈를 제거하는 핵심 도구,
- SAR 이미지 처리에서는 노이즈 제거, 특징 추출, 표적 인식, 압축 등 거의 모든 단계에서 사용된다.

---
## 실전 예제

- 아래 예제는 “SAR 이미지를 2D 배열로 보고 SVD로 노이즈를 줄이는” 가장 기본적인 패턴.

### 1. 기본 아이디어
- SAR 이미지를 2D 배열 I 로 읽는다. (float32 권장)
- I = U Σ Vᵀ 로 SVD 분해한다.
- 큰 특이값 몇 개만 남기고 나머지는 잘라서 I_denoised 를 재구성한다.
    - 큰 특이값 = 구조/형상 정보
    - 작은 특이값 = 노이즈(스페클 포함)
    - 결과를 다시 이미지로 본다.

### 2. 최소 예제 코드 (NumPy + Matplotlib)
```python
import numpy as np
import matplotlib.pyplot as plt

# 1. SAR 이미지를 2D 배열로 준비한다고 가정 (예: H x W)
# 실제 코드에서는 여기서 이미지 파일을 읽어오면 됨
# 예시: I = plt.imread('sar_image.png').astype(np.float32)
# 지금은 가짜 SAR 비슷한 데이터 생성
H, W = 512, 512
np.random.seed(0)
# base 구조 + 노이즈
base = np.zeros((H, W), dtype=np.float32)
base[100:400, 200:300] = 1.0   # 어떤 타겟/건물 같은 구조
noise = 0.5 * np.random.randn(H, W).astype(np.float32)
I = base + noise

# 2. SVD 분해
# full_matrices=False 를 쓰면 메모리/속도 측면에서 훨씬 효율적
U, S, VT = np.linalg.svd(I, full_matrices=False)
# U: (H, r), S: (r,), VT: (r, W), r = min(H, W)

# 3. 상위 k 개 특이값만 사용해서 재구성 (노이즈 제거 효과)
# k를 조절하면서 구조 vs 노이즈 트레이드오프를 볼 수 있음
k = 50  # 예: 상위 50개만 사용
S_k = np.zeros_like(S)
S_k[:k] = S[:k]

# Σ_k 를 대각 행렬 대신 broadcasting 으로 사용
I_denoised = (U * S_k) @ VT

# 4. 결과 시각화
fig, axs = plt.subplots(1, 3, figsize=(12, 4))
axs[0].imshow(I, cmap='gray')
axs[0].set_title('Original (noisy)')
axs[0].axis('off')

axs[1].plot(S)
axs[1].set_title('Singular values')
axs[1].set_yscale('log')

axs[2].imshow(I_denoised, cmap='gray')
axs[2].set_title(f'SVD denoised (k={k})')
axs[2].axis('off')

plt.tight_layout()
plt.show()
```

![SVD Noise Filtering](/image/sar_svd_noisy_filtering.png)

### 3. 실제 SAR 이미지에 적용할 때 팁
- 입력 타입:
- SAR은 보통 float, log-scale, 또는 complex amplitude 형태.
- 위 예제는 real-valued intensity 기준.
- 복소 SAR이라면 |I| 또는 log(|I|) 같은 형태로 먼저 만들어 쓰는 경우가 많음.
- k 선택:
  - 너무 작으면 구조가 과도하게 단순해짐 (디테일 날아감)
  - 너무 크면 노이즈가 그대로 남음
  - 보통 특이값 그래프를 보고 **급격히 꺾이는 지점** 근처를 고른다.
- 블록 단위 처리:
  - 큰 SAR 이미지는 메모리 때문에 전체 SVD가 부담될 수 있음
  - 타일/패치 단위 (예: 128×128, 256×256)로 나눠서 SVD 적용 → 다시 합치기
- 패턴도 많이 쓴다.
- 구조만 보고 싶을 때:
  - k를 아주 작게(예: 5, 10) 잡으면 큰 스케일 구조(지형, 큰 건물, 강, 도로)만 남고 소규모 노이즈/디테일은 많이 사라진다.

### 4. “실제 SAR”에 맞게 바꾸는 형태 예시
```python
import numpy as np
import matplotlib.pyplot as plt

# 예: SAR magnitude 이미지를 읽었다고 가정
I_raw = plt.imread('sar_magnitude.png').astype(np.float32)

# log 스케일로 살짝 눌러줌 (선택사항)
I = np.log1p(I_raw)  # log(1 + x)

U, S, VT = np.linalg.svd(I, full_matrices=False)

k = 80  # SAR 구조에 맞게 조절
S_k = np.zeros_like(S)
S_k[:k] = S[:k]

I_denoised = (U * S_k) @ VT

plt.figure(figsize=(10, 4))
plt.subplot(1, 2, 1)
plt.imshow(I, cmap='gray')
plt.title('Original log SAR')
plt.axis('off')

plt.subplot(1, 2, 2)
plt.imshow(I_denoised, cmap='gray')
plt.title(f'SVD denoised (k={k})')
plt.axis('off')

plt.tight_layout()
plt.show()
```
---

# Rust 코드

- Rust는 NumPy 같은 고수준 SVD 함수가 없기 때문에, 보통 ndarray + ndarray-linalg 조합을 사용.

## 📌 Rust: SAR 이미지에 SVD 적용해 노이즈 제거하기
### 1. Cargo.toml 설정
````
[package]
name = "sar_svd"
version = "0.1.0"
edition = "2021"

[dependencies]
ndarray = "0.15"
ndarray-linalg = { version = "0.16", features = ["openblas"] }
image = "0.24"
```
- openblas 기능을 켜야 SVD가 동작.
  (Windows, Linux, macOS 모두 지원)


### 2. main.rs — SVD 기반 노이즈 제거
```rust
use ndarray::{Array2, ArrayBase, Data, Ix2};
use ndarray_linalg::SVD;
use image::{GrayImage, Luma};

fn main() {
    // 1. SAR 이미지 로드 (grayscale)
    let img = image::open("sar.png").unwrap().to_luma8();
    let (h, w) = img.dimensions();

    // 2. ndarray로 변환 (f64)
    let mut mat = Array2::<f64>::zeros((h as usize, w as usize));
    for y in 0..h {
        for x in 0..w {
            mat[(y as usize, x as usize)] = img.get_pixel(x, y)[0] as f64;
        }
    }

    // 3. SVD 분해
    let (u, s, vt) = mat.svd(true, true).unwrap();

    // 4. 상위 k개 특이값만 사용
    let k = 50;
    let mut s_k = s.clone();
    for i in k..s_k.len() {
        s_k[i] = 0.0;
    }

    // 5. 재구성: U * Σ_k * Vᵀ
    let sigma = Array2::from_diag(&s_k);
    let mat_denoised = u.unwrap().dot(&sigma).dot(&vt.unwrap());

    // 6. 다시 이미지로 변환
    let mut out = GrayImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let v = mat_denoised[(y as usize, x as usize)];
            let v = v.clamp(0.0, 255.0) as u8;
            out.put_pixel(x, y, Luma([v]));
        }
    }

    out.save("sar_denoised.png").unwrap();
    println!("Saved: sar_denoised.png");
}
```
### 📌 코드 설명
- ✔ 1) SAR 이미지 로드
  - Rust의 image crate로 PNG/JPG/TIFF 등 읽기 가능.
- ✔ 2) ndarray로 변환
  - SVD는 f64 타입을 요구하므로 변환.
- ✔ 3) SVD 수행
```rust
let (u, s, vt) = mat.svd(true, true).unwrap();
```
- u: 좌특이벡터
- s: 특이값 벡터
- vt: 우특이벡터 전치
- ✔ 4) 상위 k개 특이값만 남기기
  - SAR 노이즈 제거 핵심.
- ✔ 5) 재구성
  - U * Σ_k * Vᵀ
- ✔ 6) 이미지로 저장
  - 노이즈 제거된 SAR 이미지 출력.

### 📌 결과
- sar.png → 입력 SAR 이미지
- sar_denoised.png → SVD 기반 노이즈 제거 결과

---

