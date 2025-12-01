# CFAR
CFAR(Cell-Averaging CFAR)는 레이더 신호 처리에서 목표 탐지를 위한 적응형 임계값 결정 알고리즘입니다.  
즉, 잡음 환경이 변해도 **거짓 경보율(False Alarm Rate)** 을 일정하게 유지하면서 목표를 검출할 수 있도록 설계된 방법.

## 📌 기본 개념
- CFAR (Constant False Alarm Rate):
  - 레이더 수신 신호에서 잡음과 간섭이 시간·공간적으로 변할 때, 탐지 임계값을 자동으로 조정해 거짓 경보율을 일정하게 유지하는 기법.
- Cell-Averaging CFAR (CA-CFAR):
  - 가장 널리 쓰이는 CFAR 방식. 탐지 대상 셀(CUT, Cell Under Test) 주변의 **훈련 셀(training cells)** 에서 잡음 파워를 추정하고,  
    이를 평균 내어 임계값을 설정.

## 🧮 알고리즘 절차
- Cell Under Test (CUT): 현재 탐지하려는 셀.
- Guard Cells: CUT 바로 옆의 몇 개 셀은 제외(목표 신호가 섞이지 않도록).
- Training Cells: Guard Cells 바깥의 셀들을 사용해 잡음 파워 추정.

$$
P_n=\frac{1}{N}\sum _{m=1}^Nx_m
$$

- $N$: 훈련 셀 개수
- $x_m$: 각 훈련 셀의 신호 값
- $P_n$: 추정된 잡음 파워
- $Threshold$ 계산:

$$
T=\alpha \cdot P_n
$$

- $\alpha$ : 임계값 스케일링 계수 (False Alarm Rate을 일정하게 유지하도록 설정)
- 검출 조건:

$$
\mathrm{CUT\  값}>T\quad \Rightarrow \quad \mathrm{Target\  Detected}
$$

## ✨ 특징
- 장점: 잡음 환경이 변해도 일정한 거짓 경보율 유지 → 실전 레이더에서 안정적 탐지 가능.
- 단점: 주변에 강한 클러터(clutter)나 목표가 있으면 잡음 추정이 왜곡될 수 있음.
- 변형 기법:
  - CAGO-CFAR (Greatest Of)
  - CASO-CFAR (Smallest Of)
  - OS-CFAR (Order Statistic)
    - 다양한 환경에서 성능을 보완하기 위해 개발된 변형들.

## 📌 활용 분야
- 군사용 레이더, 항공 관제 레이더
- 해상 레이더(파도 클러터 억제)
- 자동차 레이더(ADAS, 자율주행)
- 위성 원격탐사, 항공기 감시 시스템
- 👉 정리하면, Cell-Averaging CFAR는 레이더에서 주변 셀의 평균 잡음 파워를 기반으로 탐지 임계값을 동적으로 조정해,  
  거짓 경보율을 일정하게 유지하는 탐지 알고리즘입니다.

---
