## Isar 영상 합성의 핵심 원리
목표물의 자체 운동을 이용한 ISAR(Inverse Synthetic Aperture Radar)는,  
플랫폼이 움직이는 SAR과 달리 **목표물의 회전·진동·이동** 이 만들어내는  
시간-위치 변화로 합성 개구(synthetic aperture)를 형성하고, 이를 통해 고해상도 2D(거리–도플러) 영상을 얻는 방식입니다.

## 기본 영상 형성 개념
- 반사 신호 모델: 레이더는 시간에 따라 변화하는 목표물 산란원(scatterer)의 거리와 방위에 따른 위상 변화를 측정합니다.
- 위상은 거리 변화에 비례해 변합니다.

$$
\phi (t)\approx \frac{4\pi }{\lambda }R(t)
$$

- 여기서 $\lambda$ 는 파장, $R(t)$ 는 시간 $t$ 에서의 레이더–산란원 간 거리입니다.
- 도플러 주파수: 산란원의 레이더에 대한 방사형 속도 $v_r(t)$ 가 있으면 도플러 편이가 발생합니다.

$$
f_d(t)=\frac{2\, v_r(t)}{\lambda }
$$

- 이 도플러가 **방위(크로스-레인지) 축** 해상도를 만들어주는 핵심입니다.

## 자체 운동 유형과 영상에 미치는 영향
- 회전(Rotation): 항공기/선박이 기준축을 중심으로 회전하면 각 산란원은 시선방향(radial) 속도가 서로 달라져 서로 다른 도플러 서명을 가집니다.
- 시간에 따라 도플러가 변화하므로, 누적 관측으로 크로스-레인지 분해능이 향상됩니다.
- 크로스-레인지 해상도 근사:

$$
\Delta x_{\mathrm{cr}}\approx \frac{\lambda }{2\, \Delta \theta }
$$

- 관측 동안 누적된 시선각 변화 $\Delta$ $\theta$ 가 클수록 방위 해상도가 좋아집니다.
- 진동(Vibration): 구조물의 미세 진동은 마이크로-도플러 성분을 만듭니다.
- 블레이드 회전, 안테나 스윙 등은 특유의 도플러 측대(sub-bands)를 형성해 부품 식별에 유리합니다.
- 이동(Translation): 목표물의 전역 이동은 모든 산란원에 공통으로 작용하는 거리/위상 변화를 만들어 정합 보정이 필요합니다.
- 적절히 보정하면 잔여 상대운동(회전 등)이 크로스-레인지 정보를 제공합니다.

## 표준 ISAR 처리 파이프라인
### 신호 정렬과 보정
- 거리 정렬(Range alignment): 펄스 간 목표물의 전역 거리 변화를 보정해 각 펄스의 거리 프로파일을 정렬합니다.
- 위상 보정/오토포커스: 전역 위상 흔들림을 제거하여 산란원들의 도플러 응집(coherence)을 회복합니다.
- 흔히 Phase Gradient Autofocus(PGA), Map-Drift 등이 사용됩니다.
- RCMC(거리 셀 마이그레이션 보정): 회전으로 인해 산란원이 다른 거리 셀로 이동하는 현상을 보정합니다.
### 영상 형성
- 거리 축 형성: 각 펄스의 압축(상관/매치드 필터)으로 고해상도 거리 프로파일 생성.
- 크로스-레인지 축 형성: 시간 축 또는 도플러 축으로 2D FFT 수행해 거리–도플러(=ISAR) 이미지를 형성합니다.
- 기하 재형성(Polar reformatting): 회전 궤적이 균일하지 않을 때, 데이터는 파수-도메인에서 비균일 샘플링이 되므로  
  키스톤 변환(Keystone transform), Stolt 재표본화 등으로 등간격 그리드로 변환합니다.

### 직관적 사례
- 항공기: 동체는 저도플러, 회전하는 프로펠러/팬 블레이드는 고주파 마이크로-도플러 성분을 보입니다.
- 일정 시간 관측으로 블레이드의 형상/회전수 추정과 동체 윤곽 영상이 형성됩니다.
- 선박: 선체 롤/피칭(회전)으로 방위 해상도가 생성됩니다. 선체 상의 마스트, 데크 구조물 등 산란원이 서로 다른 도플러 궤적을 따라 분리됩니다.

### 성능 요소와 제약
- 관측 시간–운동 균질성: 충분한 시간 동안 **안정적인** 회전(또는 유효 방위 변화)이 있어야 높은 크로스-레인지 해상도를 달성합니다.
- 파장/대역폭: 거리 해상도는 대역폭 B에 의해 결정됩니다.

$$
\Delta R\approx \frac{c}{2B}
$$

- 대역폭이 클수록 거리 축 해상도가 향상됩니다.
- 운동 추정 정확도: 잘못된 운동 보정은 영상 흐림(blur)과 산란원 위치 왜곡을 유발합니다.
- 스페클/다중경로/진동 복잡도: 복잡한 구조물은 마이크로-도플러가 풍부하지만, 분리/해석이 어려울 수 있어 추가적인 분리/정규화가 필요합니다.

### 3D ISAR로의 확장
- 다중 관측각 또는 장시간 회전: 서로 다른 시선에서 얻은 2D ISAR들을 결합해 토모그래피 또는 기하 기반 역산으로 3D voxel을 복원합니다.
- 표면 추출: 복원된 3D 필드에서 임계면을 선택해 $\Rightarrow$  Marching Cubes로 메쉬를 생성합니다.

### 핵심 요약
- ISAR는 목표물의 자체 운동이 만들어내는 시간-위상·도플러 변화를 이용해 합성 개구를 형성한다.
- 정렬/보정(거리, 위상, RCMC) + 2D FFT/재표본화로 거리–도플러 영상을 생성한다.
- 운동이 크로스-레인지 해상도를 결정하며, 안정적 회전·충분한 방위 변화가 성능의 관건이다.

---

## ISAR 기본 원리
- **ISAR (Inverse SAR)** 는 목표물의 **자체 운동(회전, 진동, 이동)** 을 이용해 레이더 영상을 합성하는 방식입니다.
- 일반 SAR은 레이더 플랫폼의 이동을 활용하지만, ISAR은 목표물의 움직임을 활용해 고해상도 이미지를 얻습니다.
- 결과는 보통 **2D 영상(거리-도플러 이미지)** 로 표현됩니다.

## 🤖 3D ISAR 정보 생성 과정
- 데이터 수집
  - 레이더가 목표물(예: 항공기, 선박)을 관측하면서 다중 관측각 또는 시간에 따른 회전 운동을 기록합니다.
  - 수집된 데이터는 **range profile(거리 프로파일)** 과 Doppler 정보를 포함합니다.
- 2D ISAR 영상 생성
  - 거리축(Range) + 도플러축(Cross-range) → 2D ISAR 이미지 형성
  - 이 단계에서 목표물의 평면 투영 이미지가 만들어짐
- 다중 관측 또는 회전 기반 3D 복원
  - 목표물이 회전하거나, 레이더가 여러 각도에서 관측하면 다양한 투영 이미지를 얻을 수 있음
  - Tomography(단층촬영) 기법이나 Fourier 변환 기반 3D 재구성을 적용
  - 여러 2D ISAR 영상을 합성해 3D voxel 데이터 생성
- 표면 추출
  - 3D voxel 데이터에서 Marching Cubes 같은 알고리즘을 적용해 표면 mesh 생성
  - 최종적으로 목표물의 3D 형상을 얻을 수 있음

## 📦 활용 분야
- 군사/방위: 항공기, 선박, 미사일의 3D 형상 인식 및 분류
- 민간: 해양 구조물, 항공기 정비, 우주 탐사에서 물체 형상 복원
- AI 학습: 3D ISAR 데이터를 기반으로 딥러닝 모델이 목표물의 형상을 학습 → 자동 분류/식별

## ✅ 결론
- 3D ISAR는 목표물의 운동이나 다중 관측각을 이용해 2D ISAR 영상을 얻고,  
  이를 합성해 3D voxel 데이터를 만든 뒤 표면 추출 알고리즘으로 mesh를 생성하는 방식입니다.  
- 즉, ISAR → 다중 투영 → 3D 복원 → Mesh 추출의 흐름으로 정보가 만들어 짐


## 📡 SAR vs ISAR의 차이

| 구분           | SAR (Synthetic Aperture Radar)         | ISAR (Inverse Synthetic Aperture Radar) |
|----------------|----------------------------------------|-----------------------------------------|
| 움직임의 주체  | 레이더 플랫폼(위성, 항공기)이 이동     | 목표물(항공기, 선박 등)이 회전·진동·이동 |
| 출력 영상      | 지형, 도시, 바다 등 고정된 대상의 영상 | 움직이는 목표물의 거리–도플러 영상       |
| 주요 활용      | 원격탐사, 지도 제작, 재난 모니터링     | 군사 표적 인식, 선박/항공기 형상 추정    |
| 데이터 성격    | 넓은 지역을 커버하는 정적 영상         | 특정 목표물의 동적 영상, 마이크로-도플러 포함 |

## 📡 전투기 레이더 영상 유형
- SAR (Synthetic Aperture Radar)
  - 전투기가 지상이나 해상 표적을 관측할 때, 자기 자신의 이동을 이용해 합성 개구를 형성합니다.
  - 결과: 지형 지도, 해상 표적 탐지, 고정된 구조물 영상
  - 예: 지상 공격용 표적 탐지, 정밀 지도 제작
- ISAR (Inverse SAR)
  - 전투기가 **움직이는 표적(적 항공기, 선박 등)** 을 관측할 때, 표적의 **자체 운동(회전, 진동, 이동)** 을 이용해 영상을 형성합니다.
  - 결과: 움직이는 표적의 형상 영상(거리–도플러 이미지)
  - 예: 적 항공기 식별, 선박 분류

## ✅ 결론
- 전투기 레이더가 지형/고정 표적을 보면 → SAR 이미지
- 전투기 레이더가 움직이는 표적을 보면 → ISAR 이미지
- 즉, 전투기 레이더는 상황에 따라 SAR과 ISAR을 모두 생성할 수 있고 특히 적 항공기나 선박을 식별할 때는 ISAR 영상을 활용합니다.



## 🕐 리얼타임 처리
- 현대 전투기 레이더나 일부 고성능 레이더 시스템은 실시간 영상 형성을 목표로 설계됩니다.
- 레이더 신호 처리 모듈(DSP, FPGA, GPU)이 펄스 압축, 도플러 처리, 오토포커스 등을 실시간으로 수행 → 조종사 HUD나 콘솔에 바로 영상 표시.
- 예: 전투기 레이더가 적 항공기를 추적할 때 ISAR 이미지를 실시간으로 생성해 형상 식별에 활용.

## 🖥️ 후처리 방식
- 연구용, 정보 분석용, 위성 SAR에서는 보통 **후처리(Post-processing)** 가 많습니다.
- 원시 레이더 데이터(raw I/Q data)를 저장 → 지상에서 대용량 컴퓨터로 정밀 처리 → 고해상도 SAR/ISAR 영상 생성.
- 예: 위성 SAR은 수십~수백 km 지역을 커버하므로, 실시간보다는 후처리로 정밀 지도 제작.

## 📌 차이점 요약
| 구분         | 리얼타임 처리                          | 후처리                               |
|--------------|---------------------------------------|--------------------------------------|
| 목적         | 전술 상황 인식, 즉각 대응              | 정밀 분석, 지도 제작, 연구           |
| 플랫폼       | 전투기, 함정, 일부 드론                | 위성, 연구 레이더, 대형 시스템       |
| 장점         | 즉시 활용 가능                        | 더 높은 해상도, 정밀 보정 가능       |
| 단점         | 계산 자원 제약, 해상도 제한            | 시간 지연, 실시간성 부족             |

## ✅ 결론
- 전투기·군사 레이더 → 실시간 ISAR/SAR 영상 생성 가능
- 위성·연구용 → 후처리로 고해상도 영상 생성
- 즉, 전술 상황에서는 리얼타임, 정밀 분석에서는 후처리라고 이해하면 됩니다

---

# 입력 데이터

ISAR 영상이 만들어질 때 레이더로부터 들어오는 정보는 **시간에 따라 변화하는 산란 신호** 입니다.  
이 신호를 처리해서 2D 이미지(거리–도플러 영상)를 형성하게 됩니다.

## 🧠 ISAR 영상 형성에 들어오는 정보
- 거리 정보 (Range profile)
  - 레이더 펄스가 목표물에 반사되어 돌아오는 시간 → 목표물까지의 거리 계산
  - 각 펄스마다 **거리 프로파일** 이 생성됨 (목표물의 여러 산란점이 서로 다른 거리 셀에 나타남)
- 도플러 정보 (Cross-range / Doppler shift)
  - 목표물의 회전·진동·이동으로 인해 각 산란점이 서로 다른 속도를 가짐
  - 속도 차이가 도플러 주파수로 나타나고, 이것이 방위축(크로스-레인지) 해상도를 만들어줌
- 위상 정보 (Phase history)
  - 거리와 속도 변화가 위상 변화를 일으킴
  - 위상 누적을 통해 합성 개구를 형성 → 해상도 향상

## 📊 처리 과정 요약
- Step 1: 펄스 압축 → 고해상도 거리 프로파일 생성
- Step 2: 시간 축(목표물 운동)으로 FFT → 도플러 스펙트럼 추출
- Step 3: 거리축 + 도플러축 결합 → 2D ISAR 이미지 형성
  - 가로축: 도플러(크로스-레인지)
  - 세로축: 거리(Range)

## 🎯 결과
- ISAR 영상은 결국 거리–도플러 평면에 목표물 산란원의 강도를 매핑한 2D 이미지입니다.
- 이 이미지에는 목표물의 형상, 회전하는 부품의 마이크로-도플러, 구조물의 상대 위치 같은 정보가 담깁니다.

## ✅ 정리하면:
ISAR 영상은 레이더가 수집한 **거리 프로파일 + 도플러 주파수 + 위상 정보** 를 합성해 만든 2D 이미지예요.
즉, 레이더가 보내는 건 단순한 전자파 반사 신호지만, 처리하면 목표물의 형상이 드러나는 영상으로 바뀌는 것임.

![ISAR Image Flow](/image/isar_image_flow.png)


## 📌 실제 대응 방법
- 멀티-어스펙트 관측: 레이더가 여러 각도에서 표적을 관측해 RCS 패턴을 누적 → 식별 정확도 향상
- 라이브러리 매칭: 다양한 각도에서 얻은 RCS/ISAR 데이터베이스와 비교 → 기종 판별
- 멀티센서 융합: 레이더 영상 + IR 탐지 + 전자신호(ESM) 결합 → 종합 식별
- AI 기반 분류: 딥러닝 모델이 단일 ISAR 영상에서도 특징을 추출해 기종을 추정 (최근 연구 방향)

## ✅ 결론
- 특정 각도에서 얻은 RCS만으로는 적기 식별이 쉽지 않습니다.
- 그래서 실제 전술 환경에서는 다중 관측, 데이터베이스 매칭, 센서 융합 같은 기법을 함께 써야 정확한 식별이 가능

---

## AI 활용

- 현재 관측된 RCS/ISAR 영상을 다양한 각도의 데이터베이스와 비교해 적기를 식별하는 과정은 바로 AI가 강점을 발휘하는 분야.

## 🧠 왜 AI가 필요한가?
- 패턴 인식: RCS와 ISAR 영상은 각도·환경·노이즈에 따라 크게 달라지므로, 사람이 직접 비교하기 어렵습니다.
- 대규모 데이터베이스 매칭: 다양한 각도와 조건에서 수집된 방대한 라이브러리와 실시간 영상을 빠르게 비교해야 함.
- 특징 추출: AI는 영상에서 마이크로-도플러, 형상 특징, 반사 패턴 같은 세부 특징을 자동으로 추출 가능.
- 불확실성 처리: 관측 조건이 제한적일 때도 확률적으로 가장 유사한 기종을 추정할 수 있음.

## 📌 AI 적용 방식
- 머신러닝 분류기
  - SVM, Random Forest 등으로 RCS/ISAR 특징 벡터를 학습 → 기종 분류
- 딥러닝 기반 CNN/RNN
  - ISAR 이미지를 이미지 분류 문제로 처리 → 자동 식별
  - 시계열 도플러 데이터를 RNN/LSTM으로 분석 → 마이크로-도플러 특징 활용
- 멀티모달 융합
  - 레이더 영상 + IR 영상 + 전자신호(ESM)를 함께 학습 → 종합 식별 정확도 향상
  - 라이브러리 매칭 + AI 보정
- 기존 데이터베이스와 비교 후, AI가 불확실성을 줄여 최적 후보를 제시

## ✅ 결론
- **현재 관측된 RCS/ISAR ↔ 다양한 각도 데이터베이스 비교** 는 AI의 핵심 응용 분야입니다.
- 실제로 군사·항공 분야에서는 AI 기반 자동 표적 식별(ATR, Automatic Target Recognition) 연구가 활발히 진행되고 있음.

---
## ISAR Source
```rust
// isar.rs
// ISAR 영상 생성 최소 파이프라인 (자급자족: Complex, DFT, 윈도우, 펄스 압축, 정렬, 정규화)
// 최종 출력은 첨부 Image 타입 사용.

use std::f32::consts::PI;
use std::cmp::Ordering;
use ndarray::{Array, Array3};
use crate::core::geom::Point2D;
use crate::core::image::{Image, ImgErr};

// -----------------------------
// 기본 복소 타입 및 유틸
// -----------------------------
#[derive(Clone, Copy, Debug, Default)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}
```
```rust
impl Complex {
    #[inline] pub fn new(re: f32, im: f32) -> Self { Self { re, im } }
    #[inline] pub fn conj(self) -> Self { Self::new(self.re, -self.im) }
    #[inline] pub fn mag2(self) -> f32 { self.re * self.re + self.im * self.im }
    #[inline] pub fn add(self, o: Self) -> Self { Self::new(self.re + o.re, self.im + o.im) }
    #[inline] pub fn sub(self, o: Self) -> Self { Self::new(self.re - o.re, self.im - o.im) }
    #[inline] pub fn mul(self, o: Self) -> Self {
        Self::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
    #[inline] pub fn scale(self, s: f32) -> Self { Self::new(self.re * s, self.im * s) }
}
```
```rust
// e^{-j*2*pi*k/N}
#[inline]
fn twiddle(n: usize, k: usize, N: usize) -> Complex {
    let ang = -2.0 * PI * (k as f32) * (n as f32) / (N as f32);
    Complex::new(ang.cos(), ang.sin())
}
```
```rust
// -----------------------------
// 레이더 파라미터/입력 정의
// -----------------------------
#[derive(Clone, Debug)]
pub struct RadarParams {
    // 표본 주파수, 대역폭 등은 필요 시 사용
    pub fs: f32,          // fast-time sampling rate
    pub bandwidth: f32,   // signal bandwidth
    pub lambda: f32,      // wavelength
    pub range_bins: usize,
    pub pulses: usize,    // slow-time 길이
}
```
```rust
#[derive(Clone, Debug)]
pub struct IsarInput {
    // iq[pulse][sample] 형태의 원시 I/Q
    pub iq: Vec<Vec<Complex>>,
    // 펄스 압축용 기준 신호 (reference chirp 등)
    pub reference: Vec<Complex>,
    pub params: RadarParams,
    // 메타(선택): 시선벡터, 목표물 예상 센터 등
    pub look_vector: Option<Vector2D>,
    pub target_center: Option<Point2D>,
}
```
```rust
// -----------------------------
// 윈도우/정규화 유틸
// -----------------------------
fn hann_window(n: usize) -> Vec<f32> {
    let mut w = vec![0.0f32; n];
    if n <= 1 { return w; }
    let denom = (n - 1) as f32;
    for i in 0..n {
        w[i] = 0.5 - 0.5 * (2.0 * PI * (i as f32) / denom).cos();
    }
    w
}
```
```rust
/// 파워 값 배열을 로그 스케일로 정규화하여 0..255 범위의 u8 픽셀로 변환
fn normalize_to_u8(power: &[f32]) -> Vec<u8> {
    let maxp = power.iter().cloned().fold(0.0, f32::max);
    if maxp <= 0.0 {
        return vec![0u8; power.len()];
    }
    let mut out = Vec::with_capacity(power.len());
    let floor_db = -60.0f32;
    let max_db = 0.0f32;

    for &p in power {
        let db = 10.0 * (p / maxp).max(1e-12).log10(); // -inf 방지
        let db_clamped = db.max(floor_db).min(max_db);
        let t = (db_clamped - floor_db) / (max_db - floor_db); // 0..1
        out.push((t * 255.0).round() as u8);
    }
    out
}
```
```rust
// -----------------------------
// 펄스 압축 (매치드 필터: reference와의 상관)
// -----------------------------
// out[pulse][range_bin]
fn pulse_compress(iq: &[Vec<Complex>], reference: &[Complex], range_bins: usize) -> Vec<Vec<Complex>> {
    let ref_len = reference.len();
    let mut out = vec![vec![Complex::default(); range_bins]; iq.len()];
    // 단순 시간영역 상관 (효율 < FFT, but self-contained)
    for (m, pulse) in iq.iter().enumerate() {
        let plen = pulse.len();
        for r in 0..range_bins {
            // ref를 반전/켤레 상관으로 누적
            let mut acc = Complex::default();
            for k in 0..ref_len {
                let idx = r + k;
                if idx >= plen { break; }
                let ref_idx = ref_len - 1 - k;
                acc = acc.add(pulse[idx].mul(reference[ref_idx].conj()));
            }
            out[m][r] = acc;
        }
    }
    out
}
```
```rust
// -----------------------------
// 거리 정렬 (range alignment)
// 간단: 각 펄스의 파워 피크 위치를 기준 피크에 맞춤
// -----------------------------
fn range_align(profiles: &mut [Vec<Complex>]) {
    if profiles.is_empty() { return; }
    let ref_peak = peak_index(&profiles[0]);
    for m in 1..profiles.len() {
        let cur_peak = peak_index(&profiles[m]);
        match cur_peak.cmp(&ref_peak) {
            Ordering::Equal => {}
            Ordering::Less => {
                let shift = ref_peak - cur_peak; // → 오른쪽으로 shift
                let mut shifted = vec![Complex::default(); profiles[m].len()];
                for r in 0..profiles[m].len() {
                    if r + shift < profiles[m].len() {
                        shifted[r + shift] = profiles[m][r];
                    }
                }
                profiles[m] = shifted;
            }
            Ordering::Greater => {
                let shift = cur_peak - ref_peak; // → 왼쪽으로 shift
                let mut shifted = vec![Complex::default(); profiles[m].len()];
                for r in shift..profiles[m].len() {
                    shifted[r - shift] = profiles[m][r];
                }
                profiles[m] = shifted;
            }
        }
    }
}
```
```rust
fn peak_index(profile: &[Complex]) -> usize {
    let mut maxp = -f32::INFINITY;
    let mut idx = 0usize;
    for (i, &c) in profile.iter().enumerate() {
        let p = c.mag2();
        if p > maxp {
            maxp = p;
            idx = i;
        }
    }
    idx
}
```
```rust
// -----------------------------
// 도플러 DFT (느린 시간축 FFT를 DFT로 대체)
// 입력: profiles[pulse][range_bin] → 출력: spec[range_bin][doppler_bin]
// -----------------------------
fn doppler_dft(profiles: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let pulses = profiles.len();
    if pulses == 0 { return vec![]; }
    let range_bins = profiles[0].len();
    let mut spec = vec![vec![Complex::default(); pulses]; range_bins];
    // 각 range bin마다 느린 시간 축 DFT
    for r in 0..range_bins {
        for k in 0..pulses {
            let mut acc = Complex::default();
            for n in 0..pulses {
                let w = twiddle(n, k, pulses);
                acc = acc.add(profiles[n][r].mul(w));
            }
            spec[r][k] = acc;
        }
    }
    spec
}
```
```rust
// -----------------------------
// 파워 맵 → Image
// spec[range][doppler] → 그레이 영상 (세로=range, 가로=doppler)
// -----------------------------
fn spectrum_to_image(spec: &[Vec<Complex>]) -> Image {
    if spec.is_empty() { return Image::new_gray(1, 1); }
    let height = spec.len() as u32;       // range
    let width = spec[0].len() as u32;     // doppler
    // 파워 벡터로 변환 후, 0..255 정규화
    let mut power = Vec::with_capacity((width * height) as usize);
    for r in 0..height as usize {
        for k in 0..width as usize {
            power.push(spec[r][k].mag2());
        }
    }
    let pix = normalize_to_u8(&power);
    let mut img = Image::new_gray(width, height);
    img.pixels = pix;
    img
}
```
```rust
// -----------------------------
// 실시간 누적 파이프라인
// -----------------------------
pub struct IsarRealtime {
    params: RadarParams,
    reference: Vec<Complex>,
    // 슬로우타임 버퍼 (고정 길이 circular)
    iq_ring: Vec<Vec<Complex>>,
    head: usize,
    filled: usize,
    window_slow: Vec<f32>,
}
```
```rust
impl IsarRealtime {
    pub fn new(params: RadarParams, reference: Vec<Complex>) -> Self {
        let iq_ring = vec![vec![Complex::default(); params.range_bins + reference.len()]; params.pulses];
        let window_slow = hann_window(params.pulses);
        Self { params, reference, iq_ring, head: 0, filled: 0, window_slow }
    }
```
```rust
    // 새로운 펄스 I/Q 샘플을 수신 (fast-time 길이는 적어도 range_bins + ref_len 권장)
    pub fn push_pulse(&mut self, iq_samples: Vec<Complex>) {
        self.iq_ring[self.head] = iq_samples;
        self.head = (self.head + 1) % self.params.pulses;
        if self.filled < self.params.pulses { self.filled += 1; }
    }
```
```rust
    // 충분한 펄스가 채워졌다면 ISAR 프레임 생성
    pub fn generate_frame(&self) -> Option<Image> {
        if self.filled < self.params.pulses { return None; }

        // 슬로우타임 순서를 0..pulses로 재구성
        let mut iq = Vec::with_capacity(self.params.pulses);
        let mut idx = self.head;
        for _ in 0..self.params.pulses {
            iq.push(self.iq_ring[idx].clone());
            idx = (idx + 1) % self.params.pulses;
        }

        // 펄스 압축
        let mut profiles = pulse_compress(&iq, &self.reference, self.params.range_bins);

        // 거리 정렬
        range_align(&mut profiles);

        // 슬로우타임 윈도(크로스-레인지 사이드로브 저감)
        for m in 0..self.params.pulses {
            let w = self.window_slow[m];
            for r in 0..self.params.range_bins {
                profiles[m][r] = profiles[m][r].scale(w);
            }
        }

        // 느린 시간축 DFT로 도플러 스펙트럼
        let spec = doppler_dft(&profiles);

        // 영상화
        Some(spectrum_to_image(&spec))
    }
}
```
```rust
// -----------------------------
// 오프라인 일괄 처리 (배치)
// -----------------------------
pub fn generate_isar_image(input: &IsarInput) -> Result<Image, ImgErr> {
    let pulses = input.params.pulses;
    let range_bins = input.params.range_bins;

    // 1) 펄스 압축
    let mut profiles = pulse_compress(&input.iq, &input.reference, range_bins);

    // 2) 거리 정렬
    range_align(&mut profiles);

    // 3) 슬로우타임 윈도잉
    let w = hann_window(pulses);
    for m in 0..pulses {
        for r in 0..range_bins {
            profiles[m][r] = profiles[m][r].scale(w[m]);
        }
    }

    // 4) 도플러 DFT
    let spec = doppler_dft(&profiles);

    // 5) 영상화
    let img = spectrum_to_image(&spec);

    Ok(img)
}
```
```rust
// -----------------------------
// 헬퍼: 간단한 기준 신호(chirp) 생성 (테스트용)
// s(t) ≈ exp(j*pi*alpha*t^2) 를 이산화하여 reference 생성
// -----------------------------
pub fn make_chirp_reference(len: usize, alpha: f32) -> Vec<Complex> {
    let mut out = Vec::with_capacity(len);
    for n in 0..len {
        let t = n as f32 / (len as f32);
        let phase = PI * alpha * t * t;
        out.push(Complex::new(phase.cos(), phase.sin()));
    }
    out
}
```
```rust
/// Image → CNN 입력 텐서 (1채널, [1, height, width])
pub fn image_to_tensor(img: &Image) -> Array3<f32> {
    let h = img.height as usize;
    let w = img.width as usize;
    let mut arr = Array::zeros((1, h, w));
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let v = img.pixels[idx] as f32 / 255.0;
            arr[[0, y, x]] = v;
        }
    }
    arr
}
```
```rust
/// 밝은 픽셀을 기준으로 산란원 위치 추출
pub fn extract_scatterers(img: &Image, threshold: u8) -> Vec<Point2D> {
    let mut points = Vec::new();
    let w = img.width;
    let h = img.height;
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let v = img.pixels[idx];
            if v >= threshold {
                points.push(Point2D {
                    x: x as f64,
                    y: y as f64,
                });
            }
        }
    }
    points
}
```
---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};
    use nurbslib::core::image::Image;
    use nurbslib::core::isar::{extract_scatterers, generate_isar_image, image_to_tensor, make_chirp_reference, IsarInput, IsarRealtime, RadarParams};
    use nurbslib::core::math_extensions::Complex;

    fn create_isar_image() -> Result<(), Box<dyn std::error::Error>> {
        // 입력 구성
        let pulses = 256;
        let range_bins = 512;
        let params = RadarParams { fs: 20e6, bandwidth: 10e6, lambda: 0.03, range_bins, pulses };
        let reference = make_chirp_reference(128, 0.9);

        // iq[pulse][sample] 준비 (여기서는 예시로 zero)
        let iq = vec![vec![Complex::new(0.0,0.0); range_bins + reference.len()]; pulses];

        let input = IsarInput {
            iq,
            reference,
            params,
            look_vector: None,
            target_center: None,
        };

        let img = generate_isar_image(&input)?;
        img.save("asset/isar_out.png")?;
        Ok(())
    }
```
```rust
    #[test]
    fn create_isar_image_test()
    {
        create_isar_image().expect("Failed to create isar image");
    }
```
```rust
    fn realtime_example() -> Result<(), Box<dyn std::error::Error>> {
        let pulses = 128;
        let range_bins = 256;
        let params = RadarParams { fs: 20e6, bandwidth: 10e6, lambda: 0.03, range_bins, pulses };
        let reference = make_chirp_reference(64, 0.9);
        let mut rt = IsarRealtime::new(params, reference);

        // 실시간으로 펄스 수신
        for _ in 0..pulses {
            let iq_samples = vec![Complex::new(0.0,0.0); range_bins + 64];
            rt.push_pulse(iq_samples);
        }

        if let Some(img) = rt.generate_frame() {
            img.save("asset/isar_realtime.png")?;
        }
        Ok(())
    }
```
```rust
    #[test]
    fn create_isar_realtime_image_test()
    {
        realtime_example().expect("Failed to realtime image");
    }
```
```rust
    #[test]
    fn test_multi_scatterers_isar() {
        use super::*;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let pulses = 128;
        let range_bins = 256;
        let params = RadarParams {
            fs: 20e6,
            bandwidth: 10e6,
            lambda: 0.03,
            range_bins,
            pulses,
        };
        let reference = make_chirp_reference(64, 0.9);

        let mut iq: Vec<Vec<Complex>> = vec![vec![Complex::default(); range_bins + reference.len()]; pulses];

        // 산란원 3개: 서로 다른 거리와 도플러 성분
        let scatterers = vec![
            (60usize, 8usize, 120.0),   // 가까운 거리, 낮은 도플러
            (120usize, 20usize, 100.0), // 중간 거리, 중간 도플러
            (200usize, 40usize, 80.0),  // 먼 거리, 높은 도플러
        ];

        let mut rng = StdRng::seed_from_u64(12345);
        for m in 0..pulses {
            let mut pulse = vec![Complex::default(); range_bins + reference.len()];
            for &(rbin, dbin, amp) in &scatterers {
                let phase = 2.0 * PI * (dbin as f32) * (m as f32) / (pulses as f32);
                let s = Complex::new(phase.cos(), phase.sin()).scale(amp);
                pulse[rbin] = s;
            }
            // 노이즈 추가
            for v in &mut pulse {
                v.re += rng.gen_range(-0.5..0.5);
                v.im += rng.gen_range(-0.5..0.5);
            }
            iq[m] = pulse;
        }

        let input = IsarInput {
            iq,
            reference,
            params,
            look_vector: None,
            target_center: None,
        };

        let img = generate_isar_image(&input).unwrap();
        img.save("asset/isar_multi_scatterers.png").unwrap();
    }
```
```rust
    #[test]
    fn test_isar_pipeline() {
        // 가상 파라미터
        let pulses = 128;
        let range_bins = 256;
        let params = RadarParams {
            fs: 20e6,
            bandwidth: 10e6,
            lambda: 0.03, // X-band ~10GHz
            range_bins,
            pulses,
        };
        // 기준 신호
        let reference = make_chirp_reference(64, 0.9);

        // 가상 타깃: 특정 range_bin에 산란원, 슬로우타임에서 도플러 위상 누적
        let mut iq: Vec<Vec<Complex>> = vec![vec![Complex::default(); range_bins + reference.len()]; pulses];

        let target_range = 90usize;
        let doppler_hz_bin = 12usize; // 슬로우타임 bin

        let mut rng = StdRng::seed_from_u64(42);
        for m in 0..pulses {
            let mut pulse = vec![Complex::default(); range_bins + reference.len()];
            // 타깃 반사 성분 (간단 모델)
            let phase = 2.0 * PI * (doppler_hz_bin as f32) * (m as f32) / (pulses as f32);
            let s = Complex::new(phase.cos(), phase.sin()).scale(100.0);
            pulse[target_range] = s;

            // 노이즈 추가
            for v in &mut pulse {
                v.re += rng.gen_range(-0.5..0.5);
                v.im += rng.gen_range(-0.5..0.5);
            }
            iq[m] = pulse;
        }

        let input = IsarInput {
            iq,
            reference,
            params,
            look_vector: None,
            target_center: None,
        };

        let img = generate_isar_image(&input).unwrap();
        assert_eq!(img.channels, 1);
        assert_eq!(img.width, pulses as u32);
        assert_eq!(img.height, range_bins as u32);
        // 간단 확인: 픽셀 범위
        assert!(img.pixels.iter().all(|&p| p <= 255));
    }
```
```rust
    fn detect_isar() -> Result<(), Box<dyn std::error::Error>> {
        let img = Image::load("asset/isar_multi_scatterers.png")?;

        // CNN 입력 텐서로 변환
        let tensor = image_to_tensor(&img);
        println!("Tensor shape: {:?}", tensor.shape());

        // 산란원 위치 추출
        let scatterers = extract_scatterers(&img, 240);
        println!("Detected scatterers: {:?}", scatterers);

        Ok(())
    }

    #[test]
    fn test_detect_isar() {
        detect_isar().expect("Failed to detect ISAR");
    }

}
```
