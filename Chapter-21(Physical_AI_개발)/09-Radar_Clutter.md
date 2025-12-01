# Radar clutter

## Radar clutter removal, noise filtering, and anomaly detection
레이더 데이터에서 성능을 좌우하는 세 축은 배경 반사(클러터) 제거, 잡음 필터링, 그리고 이상 탐지입니다.  
아래에 각 항목을 실무 관점에서 깊게 정리했습니다.

## Clutter 제거
레이더의 클러터는 지면, 바다, 건물, 비/눈 등 **의도하지 않은 지속적 반사** 입니다. 
핵심은 목표물(이동체)의 도플러·공간 특성을 이용해 배경을 분리하는 것입니다.
### MTI/MTE (Moving Target Indication/Extraction)
- 아이디어: 정지/저속 반사(도플러 0 근처)를 억제하여 이동 목표만 남기기.
- 방식: 펄스 간 위상 차를 이용해 고역통과처럼 동작.

$$
x_k=s_k+c_k+n_k,\quad y_k=x_k-x_{k-1}
$$

- `c_k`: 정적 클러터(연속성 높음), `s_k`: 목표, `n_k`: 잡음
- 2펄스 MTI의 클러터 억제는 단순하지만, 저속 목표 손실과 잔여 클러터가 남을 수 있음.
- 확장: 3펄스 MTI, IIR 고역통과, 도플러 필터뱅크로 저속/정지 대역을 정밀 차단.

### STAP (Space-Time Adaptive Processing)
- 아이디어: 안테나 배열(공간) + 펄스(시간) 공동 최적화로 클러터를 노치(Null)화.
- 최적 필터: 최소 출력 전력(MVDR)

$$
\min _{\mathbf{w}}\  \mathbf{w^{\mathrm{H}}Rw}\quad \mathrm{s.t.}\quad \mathbf{w^{\mathrm{H}}v}=1
$$

- $\mathbf{R}$: 공분산(클러터+잡음), $\mathbf{v}$ : 목표 방향 벡터
- 해는 $\mathbf{w}\propto \mathbf{R^{\mathnormal{-1}}v}$ 로 귀결; 샘플 수 부족 시 샘플 부족(STAP Snapshot shortage) 문제 발생.
- 실무 팁:
    - 차수 축소(DoF reduction): 도플러-방위 하위공간 선택, Toeplitz/블록 Toeplitz 가정.
    - 훈련 영역: 목표 없는 영역에서 $\mathbf{R}$ 추정(스케이트 트레이닝/리디아 트레이닝).
    - 가속: QR, CG, 근사 역행렬; 일부는 FPGA/GPU로 구현.
### Clutter 맵/지도 기반 억제
- 아이디어: 위치·도플러별 배경 파워를 장기 평균으로 저장해 “동적 바탕”을 빼기.
- 알고리즘:
- 지수 이동 평균(EMA):

$$
C_t(r,\theta ,f_d)=\alpha \, C_{t-1}+(1-\alpha )\, |X_t|^2
$$

- 이후 X_t-\sqrt{C_t} 또는 비율로 정규화.
- 해양/강우: 바탕 분포가 빠르게 변하면 $\alpha$ 를 작게(빠르게 추종) 설정.
- 주의: 목표가 장시간 머무르면 바탕에 흡수(whitening)될 수 있으므로 Guard/Mask로 보호.

### CFAR과의 결합
- Cell-Averaging CFAR (CA-CFAR):

$$
T=\gamma \cdot \frac{1}{N}\sum _{i\in \mathrm{Ref}}|X_i|^2,\quad |X_0|^2>T\Rightarrow \mathrm{det}
$$

- 레퍼런스 윈도우에 클러터가 섞이면 문턱이 높아져 목표 누락. 해법: OS-CFAR(순위 기반), GOCA/SOCA(측면 선택), VI-CFAR(가변 윈도우).
- Clutter 지도 + CFAR: 바탕 정규화 후 CFAR 적용하면 FPR 제어가 쉬워짐.
- Noise filtering 잡음은 감지 성능과 false alarm을 좌우합니다. 스펙트럼·공간·시간 도메인에서 적절히 필터링합니다.

### Matched filtering (Pulse compression)
- 아이디어: 수신신호를 송신 신호의 복켤레로 필터링해 SNR·거리 분해능 향상.
- LFM(선형 주파수 변조) 신호라면 사다리꼴 또는 켤레 응답으로 압축.
- 실무: 창함수(Chebyshev, Kaiser)로 사이드로브 억제 vs. 해상도 손실 트레이드오프.
Wiener/Least-Squares 필터- Wiener 필터: 평균제곱오차(MSE) 최소

$$
H(\omega )=\frac{S_{xx}(\omega )}{S_{xx}(\omega )+S_{nn}(\omega )}
$$

- 신호/잡음 PSD 추정 필요. 스펙트럼 평활화와 결합.
- Savitzky–Golay/LS: 시간-거리 축에서 곡선 근사로 노이즈 완화, 피크 보존.

### 공간·시간 필터
- Median 필터: 스파이크형 잡음/글린트를 제거(경계 보존).
- Bilateral/Non-Local Means: 레인지-도플러 이미지에서 텍스처 보존형 평활화.
- Kalman 필터: 트래킹 시 상태 추정으로 측정 노이즈 저감.
- 상태: 위치/속도, 측정: 레인지·도플러·방위
- 공정잡음/측정잡음 공분산 Q,R 튜닝이 성능 핵심.
### Beamforming/Array processing
- DAS/MVDR/LCMV: 빔폭 내 노이즈 최소화를 통해 SNR 개선.
- Adaptive beamforming: 간섭원 방향을 노치(Null)화.
- Anomaly detection레이더에서 “이상”은 비정상 반사 패턴, 스푸핑/재밍, 센서 고장, 새/비행체 등입니다. 
- 통계·기계학습·딥러닝 접근이 있습니다.

### 통계적/규칙 기반
- Z-score/Robust statistics: 각 셀의 파워가 주변 평균 대비 크게 벗어나면 이상

$$
z=\frac{x-\mu }{\sigma },\quad |z|>\tau \Rightarrow \mathrm{anomaly}
$$

- 주변 통계는 방향성 있게(레인지 또는 도플러 축별) 추정.
- 변화점 탐지 (CUSUM/GLR): 급격한 분포 변화 포착.

### 전통 ML
- One-Class SVM/Isolation Forest: 정상 패턴 학습 후 외곽값 탐지.
- HMM/GMM: 시간적 전이 패턴 모델링(트랙 붕괴, 신호 급변 감지).
- Feature: 스펙트럼 엔트로피, 도플러 확산, 텍스처(Gray-Level Co-occurrence).

### 딥러닝 기반

- 오토인코더/변분 오토인코더(VAE): 정상 레인지–도플러 맵을 재구성; 재구성 오차로 이상 판정.

$$
\mathcal{L}=\| X-\hat {X}\| _2^2+\beta \, \mathrm{KL}(q(z|X)\| p(z))
$$

- CNN/3D CNN: 거리–도플러–시간을 입력으로 이상 패턴 이미지 탐지.
- Temporal models (LSTM/Transformer): 트랙 또는 시퀀스의 이상 행태(가속 급변, 도플러 점프) 감지.
- Self-supervised: 정상 데이터로 프리텍스트 과제(패치 복원, 미래 프레임 예측)를 학습 후 오차 기반 이상 탐지.
### 실무 운용 포인트
- Thresholding: ROC 곡선으로 운영점 선택(환경별 재튜닝).
- Class imbalance: 이상은 희귀 → FPR 제어가 핵심. Focal loss, hard negative mining.
- Robustness: 날씨/지형/해상 상태 변화 대응을 위해 도메인 적응(Domain Adaptation), 데이터 증강(speckle, attenuation, micro-Doppler)을 활용.

### 통합 전략과 운영 팁
- 파이프라인 구성: Matched filter → MTI/STAP → Clutter map 정규화 → CFAR → Tracking → Anomaly
- 온라인/오프라인 분리: STAP/CFAR 파라미터는 온라인, 모델 재학습(오토인코더 등)은 오프라인 배치로.
- 센서 융합: 레이더 단독 이상 신호는 오경보가 있을 수 있음. 카메라/ADS-B/LiDAR와 late fusion으로 신뢰도 결합.
- 모델 경량화: 엣지 운용 시 quantization(8-bit), pruning으로 지연(latency) 최소화.
- 모니터링: 데이터 분포가 바뀌면 성능 급락 → Data drift 탐지로 재튜닝 트리거.

### 📌 Python 예제 코드
```python
import numpy as np
import matplotlib.pyplot as plt
from scipy.signal import butter, lfilter, medfilt

# -----------------------------
# 1. 가짜 레이더 신호 생성
# -----------------------------
np.random.seed(42)
N = 1024
t = np.linspace(0, 1, N)

# 목표물 (sin 파형)
target = np.sin(2 * np.pi * 50 * t)

# 클러터 (저주파 성분)
clutter = 0.5 * np.sin(2 * np.pi * 5 * t)

# 잡음 (Gaussian noise)
noise = 0.3 * np.random.randn(N)

# 전체 신호
signal = target + clutter + noise

# -----------------------------
# 2. Clutter 제거 (고역통과 필터)
# -----------------------------
def highpass_filter(data, cutoff=20, fs=1024, order=4):
    nyq = 0.5 * fs
    normal_cutoff = cutoff / nyq
    b, a = butter(order, normal_cutoff, btype='high', analog=False)
    return lfilter(b, a, data)

signal_clutter_removed = highpass_filter(signal)

# -----------------------------
# 3. Noise filtering (Median filter)
# -----------------------------
signal_denoised = medfilt(signal_clutter_removed, kernel_size=5)

# -----------------------------
# 4. Anomaly detection (Threshold)
# -----------------------------
threshold = 2.0
anomalies = np.where(np.abs(signal_denoised) > threshold)[0]

# -----------------------------
# 5. 결과 시각화
# -----------------------------
plt.figure(figsize=(12,6))
plt.plot(t, signal, label="Original signal (target+clutter+noise)", alpha=0.5)
plt.plot(t, signal_clutter_removed, label="After clutter removal", alpha=0.8)
plt.plot(t, signal_denoised, label="After noise filtering", linewidth=2)

# 이상치 표시
plt.scatter(t[anomalies], signal_denoised[anomalies], color='red', label="Anomalies")

plt.legend()
plt.title("Radar Signal Processing: Clutter Removal, Noise Filtering, Anomaly Detection")
plt.xlabel("Time")
plt.ylabel("Amplitude")
plt.show()
```


### 🧠 코드 설명
- 신호 생성
- 목표물: 50Hz sine
- 클러터: 5Hz sine (저주파)
- 잡음: Gaussian noise
- Clutter 제거
    - Butterworth 고역통과 필터로 저주파 성분 제거
- Noise filtering
    - Median filter로 스파이크성 잡음 제거
- Anomaly detection
    - 임계값(threshold) 기반으로 이상치 탐지
- 시각화
    - 원래 신호, 클러터 제거 후, 잡음 제거 후, 이상치 표시

- 이 코드를 실행하면 원래 신호 → 클러터 제거 → 잡음 제거 → 이상 탐지 과정을 한눈에 볼 수 있습니다.

---

**필터 통과 전(original)** 과 필터 통과 후(processed) 결과를 비교해서 시각화할 수 있습니다.  
제가 앞서 드린 Python 코드에선 signal이 필터 전 신호, signal_denoised가 클러터 제거 + 노이즈 필터링 후 신호입니다.  
이를 그래프로 나란히 그리면 이렇게 됩니다:  

### 📌 Python 예제 (전/후 비교)
```python
import numpy as np
import matplotlib.pyplot as plt
from scipy.signal import butter, lfilter, medfilt

# 신호 생성
np.random.seed(42)
N = 1024
t = np.linspace(0, 1, N)
target = np.sin(2 * np.pi * 50 * t)          # 목표물
clutter = 0.5 * np.sin(2 * np.pi * 5 * t)    # 클러터
noise = 0.3 * np.random.randn(N)             # 잡음
signal = target + clutter + noise            # 원래 신호

# 고역통과 필터로 클러터 제거
def highpass_filter(data, cutoff=20, fs=1024, order=4):
    nyq = 0.5 * fs
    normal_cutoff = cutoff / nyq
    b, a = butter(order, normal_cutoff, btype='high', analog=False)
    return lfilter(b, a, data)

signal_clutter_removed = highpass_filter(signal)

# Median 필터로 노이즈 제거
signal_denoised = medfilt(signal_clutter_removed, kernel_size=5)

# 전/후 비교 시각화
plt.figure(figsize=(12,6))
plt.subplot(2,1,1)
plt.plot(t, signal, label="Original (Before filtering)", color="gray")
plt.legend()
plt.title("Radar Signal - Before Filtering")

plt.subplot(2,1,2)
plt.plot(t, signal_denoised, label="Processed (After filtering)", color="blue")
plt.legend()
plt.title("Radar Signal - After Clutter Removal + Noise Filtering")

plt.xlabel("Time")
plt.ylabel("Amplitude")
plt.tight_layout()
plt.show()

```

### 🧠 결과 해석
- 위 그래프 (Before filtering): 목표물 + 클러터 + 잡음이 섞여 있어 파형이 흐릿하고 불안정합니다.
- 아래 그래프 (After filtering): 저주파 클러터가 제거되고, Median 필터로 잡음이 줄어들어 목표물 파형이 더 뚜렷하게 나타납니다.

### 🎯 식별에 주는 도움
- 1. 클러터 제거 (Clutter Removal)
    - 효과: 배경 반사(지면, 바다, 건물 등)를 줄여서 목표물 신호 대비 잡음을 낮춤.
    - 식별 도움:
    - 목표물이 배경에 묻혀 잘 안 보이는 상황에서 **대비(contrast)**를 높여줌.
    - 예: 바다 위 작은 보트 → 파도 반사(Sea Clutter)를 제거하면 보트가 더 뚜렷하게 탐지됨.

- 2. 노이즈 필터링 (Noise Filtering)
    - 효과: 랜덤 잡음이나 순간적인 스파이크를 줄여 신호를 매끄럽게 만듦.
    - 식별 도움:
    - 목표물의 형태/패턴을 더 정확히 추출 가능.
    - 예: 도플러 스펙트럼에서 잡음이 줄어들면 목표의 속도 성분이 더 선명하게 드러남.
    - 추적 알고리즘(Kalman Filter 등)이 안정적으로 동작 → 목표 분류 정확도 향상.

- 3. 이상 탐지 (Anomaly Detection)
    - 효과: 정상적인 레이더 반사 패턴과 다른 신호를 자동으로 표시.
    - 식별 도움:
    - 비정상 목표(스푸핑, 재밍, 드론, 새 등)를 빠르게 구분.
    - 정상적인 항적과 다른 움직임을 가진 물체를 조기 경보.
    - 예: 항공기 트랙 중 갑자기 속도/방향이 급변하면 이상 탐지로 식별 → 위협 판단에 도움.


## 📌 종합적으로
- 클러터 제거 → 목표가 배경에서 분리됨
- 노이즈 필터링 → 목표의 특징이 선명해짐
- 이상 탐지 → 정상과 다른 패턴을 자동으로 알려줌
- 👉 이 세 가지가 결합되면 레이더는 **무엇이 목표인지, 무엇이 배경/잡음인지** 를 더 정확히 구분할 수 있고,  
    결과적으로 식별 정확도와 신뢰도가 크게 향상됩니다.

---
## 주의점

**레이더 클러터(clutter)** 가 주로 저주파(도플러 0 근처) 성분으로 나타난다는 점.  
즉, 지면·바다·건물 같은 정적 반사는 속도가 거의 없으니 도플러 주파수가 낮거나 0에 집중됩니다.   
그래서 MTI나 STAP 같은 기법이 저주파 대역을 억제하는 방식으로 클러터를 제거.  
하지만 **Anomaly(이상 탐지)** 는 꼭 저주파만 의미하지 않습니다:
- 저주파 영역 이상
    - 예: 바다 클러터가 갑자기 강해지거나, 정지 목표가 비정상적으로 크게 반사될 때.
- 고주파 영역 이상
    - 예: 드론이나 새처럼 빠르게 움직이는 작은 목표가 갑자기 나타나는 경우.
- 재밍/스푸핑 신호는 특정 고주파 대역에 강하게 삽입되기도 합니다.
- 시간적 이상
    - 정상적인 트랙이 갑자기 속도·방향을 급변 → 도플러 스펙트럼 전역에서 이상 패턴 발생.
### 👉 정리하면:
- 클러터 → 주로 저주파(정적/저속 반사).
- 노이즈 → 전 주파수 대역에 랜덤하게 분포.
- Anomaly → 저주파에도, 고주파에도, 시간적 패턴에도 나타날 수 있음.
- 즉, “반사=저주파, 이상=저주파”로 단순히 대응시키기는 어렵고, 클러터는 저주파에 집중되지만
- 이상(anomaly)은 주파수 전역에서 발생할 수 있다고 이해하는 게 맞습니다.

---
