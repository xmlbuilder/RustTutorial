# Speckle Noise Reduction
**Speckle Noise Reduction(스펙클 노이즈 제거)** 는 영상이나 이미지에서 발생하는 **스펙클 노이즈(speckle noise)** 를 줄이는 기법을 말합니다.

## 📌 스펙클 노이즈(Speckle Noise)란?
- 정의: 영상에 나타나는 얼룩, 점, 잡음 패턴으로, 특히 **의료 영상(초음파, MRI)** 이나 **레이더 영상(SAR: Synthetic Aperture Radar)** 에서 자주 발생합니다.
- 원인:
  - 초음파나 레이더 같은 파동 기반 영상에서 여러 산란체(scatterer)들이 간섭(interference)하면서 생김
  - 결과적으로 영상이 거칠고 얼룩덜룩하게 보임

## 📊 Speckle Noise Reduction의 의미
- 영상 품질 향상: 잡음을 줄여서 더 선명한 영상을 얻음
- 해석 정확도 개선: 의료 영상에서는 병변 탐지, 레이더 영상에서는 표적 탐지 성능을 높임
- 후처리 안정화: 딥러닝이나 컴퓨터 비전 알고리즘이 더 정확하게 동작하도록 데이터 품질을 개선

## 🎯 주요 기법
- 필터링 기반
  - Lee Filter, Frost Filter, Kuan Filter: SAR 영상에서 많이 쓰이는 전통적 방법
  - Median Filter, Gaussian Filter: 단순한 공간 필터링
- 변환 기반
  - Wavelet Transform: 주파수 영역에서 노이즈 억제
- 딥러닝 기반
  - CNN, Autoencoder 등을 활용해 노이즈와 실제 신호를 구분하여 제거
- Adaptive 방법
  - 영상의 지역적 특성을 고려해 필터 크기나 강도를 조절

## 📌 요약
- 👉 Speckle Noise Reduction은 초음파·레이더 같은 파동 영상에서 생기는 얼룩 잡음을 줄여, 영상의 선명도와 분석 정확도를 높이는 과정입니다.
- 의료 영상에서는 병변 탐지, 레이더 영상에서는 표적 탐지에 직접적인 영향을 주는 중요한 단계예요.

---

# Real Time 적용

## 📊 소스 코드 구현 예시 (Python, OpenCV/Numpy 기반)

```python
import cv2
import numpy as np

# Gray 이미지 불러오기
img = cv2.imread("speckle_image.png", cv2.IMREAD_GRAYSCALE)

# Median Filter 적용
median_filtered = cv2.medianBlur(img, 3)

# Gaussian Filter 적용
gaussian_filtered = cv2.GaussianBlur(img, (5, 5), 1.0)

# 결과 저장
cv2.imwrite("median_result.png", median_filtered)
cv2.imwrite("gaussian_result.png", gaussian_filtered)
```

## 📌 요약
- 👉 실시간 영상 처리에서는 딥러닝 대신 필터 기반 Speckle Noise Reduction 알고리즘이 주로 사용됩니다.

---

# 실전 분야

유도 미사일이나 전투기 같은 실시간 무기 시스템에서는 딥러닝보다는 필터 기반 신호·영상 처리 기법이 주로 사용됩니다.  
이유는 단순합니다: 실시간성, 안정성, 검증 가능성이 절대적으로 중요하기 때문이에요.  

## 📌 왜 필터를 쓰는가?
- 실시간 처리 요구
  - 미사일 유도나 전투기 레이더는 밀리초 단위로 반응해야 합니다.
  - 딥러닝은 연산량이 많아 지연(latency)이 발생할 수 있어 치명적입니다.
- 안정성과 예측 가능성
  - 군사 시스템은 예측 가능한 성능이 필수 → 필터는 수학적으로 검증된 방식이라 신뢰성이 높음.
- 하드웨어 제약
  - 전투기·미사일 탑재 컴퓨터는 제한된 연산 자원에서 동작 → 필터 기반 알고리즘이 더 적합.

## 📊 실제 적용되는 필터 예시
- Kalman Filter (칼만 필터)
  - 목표 추적, 센서 융합, 항법 시스템에서 핵심적으로 사용
  - 레이더/IR 센서 데이터를 융합해 표적 위치를 추정
- Particle Filter (입자 필터)
  - 비선형/비가우시안 환경에서 목표 추적
- Speckle Noise Reduction Filters
  - SAR 레이더 영상에서 표적 탐지 성능 향상 (Lee, Frost, Kuan 필터)
- Adaptive Filters
  - 환경 변화에 따라 동적으로 잡음을 줄이는 방식

## 🎯 요약
- 👉 유도 미사일과 전투기에서는 필터 기반 신호 처리(칼만 필터, 스펙클 노이즈 필터 등)가 핵심 기술입니다.
- 딥러닝은 연구 단계나 오프라인 분석에는 쓰일 수 있지만, 실시간 무기 시스템에서는 검증된 필터 방식이 훨씬 더 많이 활용됩니다.

---


