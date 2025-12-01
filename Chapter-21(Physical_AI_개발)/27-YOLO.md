# YOLO

YOLO(You Only Look Once)는 이미지나 영상 속 객체를 빠르고 정확하게 탐지하는 딥러닝 기반 객체 탐지 알고리즘입니다.  
이름처럼 이미지를 한 번만 전체적으로 살펴서 객체의 위치와 종류를 동시에 예측하는 것이 특징.

## 📌 YOLO의 핵심 아이디어
- 단일 신경망으로 전체 이미지 처리
  - 이전 방식(R-CNN 등)은 이미지의 여러 영역을 잘라서 각각 분류해야 했습니다.
  - YOLO는 이미지를 한 번에 전체적으로 처리하여 속도를 크게 높였습니다.
- 그리드 기반 탐지
  - 이미지를 S×S 격자로 나누고, 각 격자가 객체의 존재 여부와 bounding box(좌표), 클래스 확률을 예측합니다.
- 실시간 성능
  - 한 번의 forward pass로 결과를 내기 때문에 실시간 객체 탐지가 가능하며, 자율주행·CCTV·로봇 등에 적합합니다.

## 📊 YOLO의 발전 과정
- YOLOv1 (2015): 최초 버전, 이미지 전체를 한 번에 처리하는 혁신적 접근
- YOLOv2, v3: 정확도와 속도 개선, COCO 데이터셋에서 80개 클래스 탐지 가능
- YOLOv4, v5: 더 깊은 네트워크와 최적화 기법 적용, 오픈소스 커뮤니티에서 널리 사용
- YOLOv8, v11 (Ultralytics): 최신 버전, 탐지·분할·포즈 추정까지 지원
- YOLOv12 (2025): 최신 릴리스, 더 높은 정확도와 다양한 응용 지원

## 🎯 활용 분야
- 자율주행 자동차: 보행자, 차량, 신호등 탐지
- 보안/감시: CCTV에서 사람·물체 실시간 탐지
- 의료 영상: 병변이나 장기 영역 탐지
- 산업 검사: 불량품 자동 검출
- 스마트 기기: AR/VR, 로봇 비전 등

## 📌 요약
- 👉 YOLO는 이미지를 한 번만 전체적으로 처리하여 객체의 위치와 종류를 동시에 예측하는 실시간 객체 탐지 알고리즘입니다.
- 빠른 속도와 높은 정확도로 인해 컴퓨터 비전 분야에서 가장 널리 쓰이는 기술 중 하나.

---

## 📌 Python에서 YOLO 지원 방식
- 대표 라이브러리
  - Ultralytics YOLO: 최신 YOLOv8, YOLOv11까지 지원하는 공식 Python 패키지 (pip install ultralytics)
  - Darknet (Python 바인딩): 원래 C로 작성된 YOLO 프레임워크, Python 인터페이스 제공
  - OpenCV + YOLO: OpenCV의 DNN 모듈을 통해 YOLO 모델을 Python에서 실행 가능
- 사용 방법
  - 모델 불러오기 → 이미지/영상 입력 → 객체 탐지 결과(bounding box, 클래스, confidence) 반환
  - 예시:

```python
from ultralytics import YOLO

# 모델 로드 (YOLOv8)
model = YOLO("yolov8n.pt")

# 이미지에서 객체 탐지
results = model("image.jpg")

# 결과 출력
results.show()
```

- 장점
- Python 생태계와 잘 맞음 (NumPy, Pandas, OpenCV, PyTorch 등과 연동)
- 연구/실험 속도가 빠름
- 커뮤니티와 문서가 풍부해서 학습 자료 확보가 쉬움

---



