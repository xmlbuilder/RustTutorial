# SAR

## Synthetic Aperture Radar (SAR)

SAR은 **움직이는 플랫폼(위성, 항공기 등)** 이 레이더 펄스를 연속적으로 송신·수신하면서,
플랫폼의 이동 궤적을 이용해 **가상의 대형 안테나(합성 개구)** 를 형성하는 영상 레이더 기법입니다.

- **원리**: 플랫폼 이동 → 여러 펄스 수집 → 위상 누적 → 고해상도 영상 생성
- **좌표계**: 세로축 = Range(거리), 가로축 = Azimuth(방위)
- **영상 의미**: 지상 지형, 건물, 차량 등 반사 강도를 지도처럼 표현
- **핵심 처리 단계**:
  - 1. 펄스 압축 (Range resolution 확보)
  - 2. RCMC (Range Cell Migration Correction, 플랫폼 이동 보정)
  - 3. Azimuth FFT (방위 해상도 확보)
  - 4. 파워 맵 → 영상화

- SAR은 **지형 매핑, 정찰, 지상 목표 탐지** 등에 널리 활용되며,
- ISAR과 달리 표적이 아니라 **플랫폼의 움직임** 을 이용해 영상을 생성합니다.


## 🧠 ISAR vs SAR 코드 구조 차이

| 구분           | ISAR 코드                                   | SAR 코드                                      | 차이 포인트                          |
|----------------|---------------------------------------------|-----------------------------------------------|--------------------------------------|
| 움직임 주체    | 표적(항공기, 선박 등)이 회전/진동           | 플랫폼(위성, 항공기)이 이동                   | 합성 개구 효과를 만드는 주체가 다름  |
| 보정 단계      | Range alignment (피크 위치 맞춤)            | RCMC (Range Cell Migration Correction)        | ISAR은 표적 운동 보정, SAR은 플랫폼 궤적 보정 |
| FFT 방향       | Doppler FFT (슬로우타임 → 도플러)           | Azimuth FFT (슬로우타임 → 방위)               | 도플러 vs 방위 축 처리               |
| 좌표계         | Range vs Doppler (Cross-range)              | Range vs Azimuth                              | 출력 영상의 가로축 의미가 다름       |
| 정규화 방식    | 로그 스케일 (contrast 강조)                 | sqrt 스케일 (포화 줄임)                       | 데이터 특성에 맞게 다른 스케일링 사용 |
| 출력 영상 의미 | 표적 형상(실루엣) 영상                      | 지상 지도 영상                                | 분석 대상이 표적 vs 지형             |


## 📥 ISAR vs SAR Input 차이

| 구분        | ISAR Input                                                   | SAR Input                                                                 |
|-------------|--------------------------------------------------------------|---------------------------------------------------------------------------|
| I/Q 데이터  | `iq[pulse][sample]` (표적 반사 I/Q, 표적 운동 포함)          | `iq[pulse][sample]` (지상 반사 I/Q, 플랫폼 이동 포함)                     |
| 파라미터    | `RadarParams { fs, bandwidth, lambda, range_bins, pulses }`  | `SarParams { fs, bandwidth, lambda, range_bins, pulses, platform_speed, prf }` |
| 메타데이터  | `look_vector`, `target_center` (선택)                        | `scene_center` (선택)                                                     |

### 🎯 요약
- ISAR: 입력은 표적이 움직이며 생긴 도플러를 포함한 I/Q → RadarParams 단순 구조
- SAR: 입력은 플랫폼이 이동하며 생긴 위상 누적을 포함한 I/Q → SarParams에 platform_speed, prf 같은 추가 파라미터 필요


## 📤 ISAR vs SAR Output 차이

| 구분           | ISAR Output                                      | SAR Output                                     | 차이 포인트                  |
|----------------|--------------------------------------------------|------------------------------------------------|------------------------------|
| 영상 크기      | `height = range_bins`, `width = pulses`          | `height = range_bins`, `width = pulses`        | 크기는 동일                  |
| 가로축 의미    | Doppler (Cross-range, 표적 운동 기반)            | Azimuth (플랫폼 이동 기반)                     | 가로축 해석이 다름           |
| 세로축 의미    | Range (거리)                                     | Range (거리)                                   | 동일                         |
| 영상 내용      | 표적 형상(실루엣), 산란점 분포                   | 지상 지도 영상, 지형/건물 반사 강도            | 분석 대상이 표적 vs 지형     |
| 활용 목적      | 표적 인식, 기종 분류, 마이크로-도플러 분석       | 지형 매핑, 정찰, 지상 목표 탐지                | 응용 분야가 다름             |

### 🎯 요약
- ISAR: 출력은 Range–Doppler 영상 → 표적 형상과 산란점 확인
- SAR: 출력은 Range–Azimuth 영상 → 지상 지도와 지형 반사 강도 확인


## 🎯 쉽게 말하면
- ISAR: 표적이 움직여서 생긴 도플러를 이용해 표적 형상 영상을 만든다.
- SAR: 내가 움직여서 생긴 위상 누적을 이용해 지상 지도 영상을 만든다.
- 코드에서도 이 차이가 그대로 반영돼서, ISAR은 range alignment + doppler FFT, SAR은 RCMC + azimuth FFT가 핵심 차이입니다.
