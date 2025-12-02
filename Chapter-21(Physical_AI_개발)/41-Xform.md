## 📘 Xform 문서화
### 1. 행렬 저장 및 기본 규칙
- 저장 방식: Xform.m은 row-major(행 우선) 4×4 행렬입니다.
- 동차 좌표(Homogeneous coordinates):
- 점(Point): $w=1$
- 벡터(Vector): $w=0$
- 점 변환 공식:

$$
\begin{aligned}x'&=m_{00}x+m_{01}y+m_{02}z+m_{03},\end{aligned}
$$

$$
\begin{aligned}y'&=m_{10}x+m_{11}y+m_{12}z+m_{13},,\end{aligned}
$$

$$
\begin{aligned}z'&=m_{20}x+m_{21}y+m_{22}z+m_{23},\end{aligned}
$$

$$
\begin{aligned}w'&=m_{30}x+m_{31}y+m_{32}z+m_{33}.\end{aligned}
$$


- 결과는 $(x'/w',y'/w',z'/w')$ (단, $w'\neq 0$).

- 벡터 변환:
  
$$
v'=\left[ \begin{matrix}m_{00}&m_{01}&m_{02} \quad m_{10}&m_{11}&m_{12} \quad m_{20}&m_{21}&m_{22}\end{matrix}\right] v
$$

- (즉, 평행이동은 무시).
  
## 2. 주요 생성 함수- Identity: 대각선 (1,1,1,1).
- Zero Transformation: 모든 원소 0, 단 m_{33}=1.
- Scaling:
- scale_uniform(s): $\mathrm{diag}(s,s,s,1)$
- scale_xyz(sx,sy,sz): $\mathrm{diag}(sx,sy,sz,1)$
- scale_about_point(p,sx,sy,sz): $T(p)\cdot S\cdot T(-p)$
- Rotation (Rodrigues 공식):

$$
R=I\cos \theta +(1-\cos \theta )uu^T+[u]_{\times }\sin \theta 
$$

- (축 u는 단위 벡터).
## 3. 카메라 관련 함수World → Camera (뷰 행렬)
- 함수: world_to_camera(camera_location, camera_x, camera_y, camera_z)
- 행렬 구성:
- 행 0~2: 카메라 축 (X, Y, Z)
- 마지막 열:

$$
\langle axis,cameraLocation\rangle
$$

- 효과: 월드 좌표를 카메라 기준 좌표계로 변환.
### Camera → World
- 함수: camera_to_world(camera_location, camera_x, camera_y, camera_z)
- 행렬 구성:
- 열 0~2: 카메라 축
- 열 3: 카메라 위치
- 주의: 축이 직교/정규화되어야 world_to_camera의 역행렬이 됨.

### Camera → Clip (투영 행렬)- 직교 투영:
- 원근 투영:
- 근평면: $Z=-n$
- 원평면: $Z=-f$
- 깊이 z는 비선형으로 매핑됨 (추가 LFT 필요).
  
## 4. 카메라 연산 시 주의사항 
- ⚠️ 축 방향 일관성
  - camera_z는 투영 행렬 정의와 맞춰야 함.
  - OpenNURBS는 카메라 Z축이 뒤→앞을 향한다고 정의.
  - 좌/우손 좌표계
  - OpenNURBS는 우손계.
  - DirectX 등 좌손계와 혼용하면 z 부호가 뒤집힘.
- 깊이 매핑
  - 원근 투영 후 z/w는 선형이 아님.
  - 근평면은 clip z ≈ +1, 원평면은 clip z ≈ -1.
  - 테스트 시 단순히 |z| ≤ 1로 확인하면 실패.
- 행렬 저장 vs 의미
  - row-major 저장이지만 수학적으로는 column vector 곱셈을 가정.
- 행/열 혼동 주의.
  - 축 직교성
  - camera_to_world는 축이 직교/정규화된 경우에만 world_to_camera의 역행렬이 됨.
## 5. 테스트 실패 원인 분석
- WorldToCamera 테스트:
  - 카메라 위치 (0,0,10), Z축 (0,0,-1)일 때, 월드 원점은 카메라 좌표에서 z=+10으로 나옴.
  - 근평면 점 (0,0,-1)은 clip z ≈ +1이어야 함.
## 6. 권장 테스트- 뷰 행렬 역변환: world_to_camera * camera_to_world ≈ identity
- 근/원평면 매핑:
  - (0,0,-n) → clip z ≈ +1
  - (0,0,-f) → clip z ≈ -1
- 직교 투영: 박스 경계 점이 clip [-1,1]에 정확히 매핑되는지 확인.
- 스크린 매핑: clip (-1,-1) → (left,bottom), clip (+1,+1) → (right,top).
- 합성 확인: screen_to_clip * clip_to_screen ≈ identity.
- 👉 요약하면, 카메라 축 방향과 깊이 매핑 규칙을 정확히 이해해야 테스트가 통과합니다.

![Camera Coordinate](/image/camera_coordinate.png)


## 카메라 좌표계에서 시선 방향은 Z축입니다
- 원점 (0,0,0): 카메라 위치
- –Z 방향: 카메라가 바라보는 방향 (화면 안쪽)
- +Z 방향: 카메라 뒤쪽 (보이지 않음)
- X축: 화면 좌우
- Y축: 화면 상하

## 📐 왜 Z축이 기준인가?
- 카메라가 바라보는 방향이 –Z 축으로 정의되어 있기 때문에
- 물체가 –Z 방향에 있어야 화면에 보이고,
- +Z 방향에 있으면 카메라 뒤에 있어서 보이지 않습니다


![Camera Visible](/image/camera_visible.png)

---

# Rodrigues
Rodrigues 공식은 3차원 공간에서 벡터를 특정 축(axis)과 각도(angle)만으로 회전시키는 수학 공식입니다.  
즉, 회전축과 회전각만 알면 회전 행렬을 직접 계산하지 않고도 벡터를 회전시킬 수 있는 효율적인 방법.

## 📐 Rodrigues 회전 공식
- 정의:
  - 벡터 $v\in \mathbb{R^{\mathnormal{3}}}$, 단위 회전축 k, 회전각 $\theta$ 가 주어졌을 때,  
    회전된 벡터 $v_{\mathrm{rot}}$ 는 다음과 같이 계산됩니다:

$$
v_{\mathrm{rot}}=v\cos \theta +(k\times v)\sin \theta +k(k\cdot v)(1-\cos \theta )
$$

- 각 항의 의미:
- $v\cos \theta$ : 원래 벡터를 축 방향으로 줄여주는 성분
- $(k\times v)\sin \theta$ : 축에 수직인 방향으로 회전시키는 성분
- $k(k\cdot v)(1-\cos \theta )$: 축 방향 성분을 보존하는 역할

🧩 직관적 이해
- 벡터 $v$ 를 회전축 $k$ 에 대해 평행 성분과 수직 성분으로 분해합니다.
- 평행 성분은 회전해도 변하지 않습니다.
- 수직 성분만 $\theta$ 만큼 회전시키고, 다시 평행 성분을 더하면 회전된 벡터가 됩니다.

## 📊 행렬 형태
Rodrigues 공식은 회전 행렬 R로도 표현됩니다:

$$
R=I\cos \theta +(1-\cos \theta )kk^T+[k]_{\times }\sin \theta
$$

- $I$: 단위 행렬
- $kk^T$: 축 벡터의 외적 행렬
- $[k]_{\times }$: 축 벡터의 스큐 대칭 행렬 (cross product를 행렬 곱으로 표현)

### ⚠️ 카메라/그래픽스에서 주의사항
- 축 방향을 반드시 단위 벡터로 정규화해야 합니다.
- 좌/우손 좌표계에 따라 회전 방향(시계/반시계)이 달라질 수 있습니다.
- 부동소수점 오차 때문에 실제 구현에서는 $\sin \theta$, $\cos \theta$ 를 직접 넣는 경우가 많습니다.

---


