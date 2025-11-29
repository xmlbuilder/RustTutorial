# PointNet 계열 네트워크 개요 (후속 장)

## 1. 서론
PointNet 계열 네트워크는 3D 포인트클라우드를 직접 입력으로 받아 분류(Classification),  
세그멘테이션(Segmentation), 회귀(Regression) 등을 수행하는 딥러닝 모델입니다.  
포인트클라우드는 순서가 없는 점들의 집합(set)이며, 구조적 정렬(grid)이 존재하지 않아 CNN처럼 2D·3D convolution을 그대로 적용할 수 없습니다.  
PointNet 시리즈는 이러한 문제를 해결하기 위해 다음 두 가지 원칙을 기반으로 합니다:

1. **Permutation Invariance (순서 불변성)**  
   포인트클라우드의 점 순서를 어떻게 섞어도 출력이 동일해야 한다.
2. **Local-Global Feature Fusion (국소 + 전역 특징 융합)**  
   주변점의 구조를 표현하는 local feature 와 전체 형상을 나타내는 global feature 를 함께 사용해야 한다.

---

## 2. PointNet (2017)

### 2.1 핵심 아이디어
PointNet의 핵심은 개별 점에 동일한 MLP를 적용하여 feature vector로 변환하고,  
전체 점들에 대해 Max Pooling을 수행하여 **전역(Global) feature** 를 얻는 것입니다.

- 각 점:  
  `MLP -> 64 -> 64 -> 128 -> 1024`
- 모든 점에 대해 Max Pooling  
  → permutation invariant global feature 취득
- global feature 를 다시 concat 하여 segmentation task 수행 가능

수식으로 표현하면:

$$
Input: {p1, p2, ..., pn}
$$

$$
h_i = MLP(p_i)
$$


$$
g = max_pool(h_1, h_2, ..., h_n)
$$

### 2.2 특징
- 정렬 불필요  
- 매우 빠르고 경량  
- Geometry-aware 한 구조는 부족 → 근처점 관계를 충분히 잡지 못함

---

## 3. PointNet++ (2017)

PointNet의 한계를 극복하기 위해 **local region sampling + hierarchy** 구조를 도입.

### 3.1 핵심 구성 요소
1. **FPS (Farthest Point Sampling)**  
   균일한 지역 대표점을 선택한다.
2. **Grouping (Ball Query)**  
   해당 중심점 주변의 점을 radius 내에서 군집화한다.
3. **Local PointNet**  
   각 지역에 대해 미니-PointNet 적용하여 지역 표현 학습.

이 과정을 여러 단계(Hierarchical Layer)로 반복하여 **멀티 스케일 특징** 을 획득한다.

---

## 4. DGCNN (2019)

PointNet++ 보다 더 강력한 local structure 추출을 위해 그래프 기반 CNN을 도입.

### 4.1 EdgeConv 핵심 개념
점 집합을 k-NN 그래프로 연결하고, 점 간의 상대적 위치를 edge 특징으로 사용.

- EdgeConv:
 
$$
h_i' = max_j( MLP([x_i, x_j - x_i]) )
$$

이 방식은:
- local geometric relationships 학습
- 동적으로 k-NN graph를 재구성 → Dynamic Graph CNN
- rotation·scaling 변화에 더 강한 성능

---

## 5. 각 모델 간 비교

| 모델 | 장점 | 단점 | 적용 분야 |
|------|------|------|-----------|
| PointNet | 빠름, 간단, 전역 특징 우수 | 지역 형태 파악 약함 | 분류, 위치 회귀 |
| PointNet++ | 지역+전역 통합 구조 | 느림, 구현 복잡 | 세그멘테이션, 복잡한 기하 |
| DGCNN | 강력한 local geometric 표현 | GPU 메모리 요구량 큼 | 고정밀 세그멘테이션, 인식 |

---

## 6. 향후 확장 방향
- **RANSAC + PointNet** 혼합형: 노이즈 제거 후 학습
- **Voxelization + Transformer 기반 모델**
- **Large-Scale LIDAR Dataset (Waymo, KITTI)용 확장**

---

## 7. 마무리
PointNet 계열은 현재 대부분의 3D Deep Learning 모델의 토대이며  
CAD, Mesh Processing, LIDAR, SLAM 등 전 영역에 응용 가능하다.  
앞서 작성한 PointCloud 및 KD-tree 모듈과도 자연스럽게 연계된다.

---


