# World Coordinate / Camera Coordinate

![OpenGL Pipeline](/image/opengl_pipeline.png)


## 🔍 이미지의 핵심 의미
- WC (World Coordinate): 물체가 존재하는 전역 좌표계
  - 축: x_w,y_w,z_w
- EC (Eye Coordinate): 카메라 기준 좌표계 (뷰 공간)
  - 축: x_e,y_e,z_e
- 변환 행렬 M_w: 월드 → 카메라로 좌표를 변환하는 뷰 행렬(view matrix)
- 이 행렬은 카메라의 위치와 방향을 기준으로 좌표계를 재배열하고 이동시켜줍니다

## 🧠 이게 왜 중요한가?
- 우리가 OpenGL에서 물체를 그릴 때는 모델 좌표 → 월드 좌표 → 카메라 좌표 → 클립 공간 → NDC → 화면 순서로 변환됩니다
- 그 중 WC → EC가 바로 카메라가 보는 방향으로 물체를 정렬하는 단계
- 이때 Z축이 –Z 방향으로 향하게 되며, 카메라가 –Z 방향을 바라보는 구조가 만들어집니다

## 📐 요약하면

| 단계         | 좌표계       | Z축 방향       | 특징                          | 비고                         |
|--------------|--------------|----------------|-------------------------------|------------------------------|
| 모델링       | 모델 좌표계   | Z-up (위쪽)    | 물체가 Z축을 따라 위로 세워짐 | Blender, Maya 등에서 사용    |
| 월드         | WC           | 자유           | 물체 배치 기준 좌표계         | Scene 전체 기준              |
| 카메라(View) | EC           | –Z (화면 안쪽) | 카메라가 –Z 방향을 바라봄     | OpenGL 기준                  |
| 투영/클립    | NDC/Clip     | Z ∈ [–1, 1]    | 깊이값으로 변환됨             | 화면에 보이는 영역 판단용    |
| Unity        | Unity 좌표계 | +Z (앞쪽), Y↑ | 좌손 좌표계, Y가 위쪽         | 게임엔진 기준                |


## 실제 행렬 계산 예시
아래 예시는 한 점이 모델→월드→카메라(뷰)→투영을 거쳐 화면에 나타나는 과정을 수치로 보여줍니다.
- 모델 점: $\mathbf{p_{\mathrm{model}}}=(1, 2, 3, 1)$
- 모델 행렬: 단순 병진

$$
M=\left[ \begin{matrix}1&0&0&2 ;& 0&1&0&0 ;& 0&0&1&-1 ;& 0&0&0&1\end{matrix}\right] 
$$

- 결과:

$$
\mathbf{p_{\mathrm{world}}}=M\cdot \mathbf{p_{\mathrm{model}}}=(3, 2, 2, 1)
$$

- 뷰 행렬: 카메라가 (0,0,10)에서 원점(0,0,0)을 바라보고, up=(0,1,0)

$$
V=\left[ \begin{matrix}1&0&0&0 ;& 0&1&0&0 ;& 0&0&1&-10 ;& 0&0&0&1\end{matrix}\right] \quad \Rightarrow \quad \mathbf{p_{\mathrm{cam}}}=V\cdot \mathbf{p_{\mathrm{world}}}=(3, 2, -8, 1)
$$

- 여기서 $z_{\mathrm{cam}}=-8<0$ 이므로 카메라 앞(보이는 방향)에 있음.
- 투영 행렬: 원근투영,

$$
\mathrm{fov_{\mathnormal{y}}}=60^{\circ },\\ \mathrm{aspect}=1.0,\\ \mathrm{near}=0.1, \mathrm{far}=100
$$

$$
P=\left[ \begin{matrix}f&0&0&0 ;& 0&f&0&0 ;& 0&0&\frac{f+n}{n-f}&\frac{2fn}{n-f} ;& 0&0&-1&0\end{matrix}\right] ,\quad f=\frac{1}{\tan (30^{\circ })}\approx 1.732
$$

- 클립 공간:

$$
\mathbf{p_{\mathrm{clip}}}=P\cdot \mathbf{p_{\mathrm{cam}}}=(5.196, 3.464, \approx 1.002, 8)
$$

- NDC:
- 화면 좌표(뷰포트가 $W\times H$ 일 때):

$$
x=\left( \frac{x_{\mathrm{ndc}}+1}{2}\right) W,\quad y=\left( 1-\frac{y_{\mathrm{ndc}}+1}{2}\right) H
$$

- 깊이 0..1:

$$
\mathrm{depth_{\mathnormal{01}}}=\frac{z_{\mathrm{ndc}}+1}{2}\approx 0.5625흐름 정리
$$

- 모델 행렬 M: 로컬(모델) → 월드 좌표로 “물체의 자세/위치/스케일”을 결정.
- 뷰 행렬 V: 월드 → 카메라(뷰) 좌표로 “카메라 관점”을 적용. 관례적으로 카메라는 -Z를 바라보지만,  
  V는 카메라의 위치/방향에 따라 달라집니다.
- 투영 행렬 P: 카메라(뷰) → 클립/NDC로 “시야체(프러스텀)”에 맞춰 정규화.
- 최종 화면 위치는 $P\cdot V\cdot M\cdot \mathbf{p_{\mathrm{model}}}$ 으로 결정됩니다.

---
# three.js / godot
## Cad z-up → godot/three.js y-up 변환 개요
- 가정
  - CAD(Z-up, 오른손): $(x_{\mathrm{cad}},y_{\mathrm{cad}},z_{\mathrm{cad}})$ 에서 Z가 위.
  - Godot/Three.js(Y-up, 오른손, 카메라는 대개 -Z를 전방으로 사용): Y가 위, 전방은 -Z.
- 목표 축 매핑
  - $x_{\mathrm{target}}=x_{\mathrm{cad}}$
  - $y_{\mathrm{target}}=z_{\mathrm{cad}}$
  - $z_{\mathrm{target}}=- y_{\mathrm{cad}}$
- 즉, Z를 Y로 올리고, CAD의 앞(보통 -Y를 “forward”로 쓰는 관례가 많음)을 -Z로 맞추기 위해 z에 부호 반전을 적용합니다.

## 변환 행렬
- 방향/점(동차좌표) 변환

$$
C=\left[ \begin{matrix}1&0&0&0 ;& 0&0&1&0 ;& 0&-1&0&0 ;& 0&0&0&1\end{matrix}\right] \quad \mathrm{such\  that}\quad \left[ \begin{matrix}x_t ;& y_t ;& z_t ;& 1\end{matrix}\right] =C\, \left[ \begin{matrix}x_c ;& y_c ;& z_c ;& 1\end{matrix}\right] 
$$

- 행벡터/열벡터 규칙에 따라 곱셈 순서를 유지하세요. 여기서는 열벡터 기준입니다.
  - 역변환 행렬

$$
C^{-1}=\left[ \begin{matrix}1&0&0&0 ;& 0&0&-1&0 ;& 0&1&0&0 ;& 0&0&0&1\end{matrix}\right] \quad \Rightarrow \quad \left[ \begin{matrix}x_c ;& y_c ;& z_c ;& 1\end{matrix}\right] =C^{-1}\, \left[ \begin{matrix}x_t ;& y_t ;& z_t ;& 1\end{matrix}\right] 
$$
- 모델 행렬 변환
- CAD 모델 행렬 $M_{\mathrm{cad}}$ 를 Godot/Three.js 공간으로 옮길 때:

$$
M_{\mathrm{target}} = C\cdot M_{\mathrm{cad}}
$$

- 반대로 가져올 때:

$$
M_{\mathrm{cad}} = C^{-1}\cdot M_{\mathrm{target}}
$$

이 방식은 회전/스케일/병진을 포함한 전체 모델 변환에 적용됩니다. 정규화 스케일에서 직교성을 유지(오른손→오른손)하므로 행렬의 handedness는 바뀌지 않습니다.


### 수치 예시 1: 점과 벡터
- CAD 점: $(x_c,y_c,z_c)=(1, 2, 3)$

$$
\begin{aligned}x_t&=1,\quad y_t=3,\quad z_t=-2 ;& \Rightarrow &(1, 3, -2)\end{aligned}
$$

- CAD 방향벡터: (0, 1, 0) (CAD의 +Y 방향)

$$
(x_t,y_t,z_t)=(0, 0, -1)
$$

- Godot/Three.js에서 전방 -Z와 정렬됨.


## 수치 예시 2: 기준축(회전 행렬) 변환- CAD 로컬 기준축을 열벡터로 갖는 회전행렬

- Godot/Three.js 기준으로 변환:

$$
R_{\mathrm{target}}=C_{3\times 3} R_{\mathrm{cad}}=\left[ \begin{matrix}1&0&0 ;& 0&0&1 ;& 0&-1&0\end{matrix}\right] 
$$

- 이는 축 매핑 $\hat {x}_t=\hat {x}_c,\  \hat {y}_t=\hat {z}_c,\  \hat {z}_t=-\hat {y}_c$ 와 일치합니다.

### 수치 예시 3: 전체 모델 행렬- CAD 모델 행렬(회전+병진):

$$
M_{\mathrm{cad}}=\left[ \begin{matrix}1&0&0&2 ;& 0&1&0&0 ;& 0&0&1&5 ;& 0&0&0&1\end{matrix}\right]
$$

- 변환:

$$
M_{\mathrm{target}}=C M_{\mathrm{cad}}=\left[ \begin{matrix}1&0&0&2 ;& 0&0&1&5 ;& 0&-1&0&0 ;& 0&0&0&1\end{matrix}\right]
$$

- 병진도 동일 규칙으로 매핑되어 $(x,y,z)=(2, 5, 0)$ 가 됩니다.

### 적용 팁

- 메시/노드의 포지션·회전·스케일을 분해해서 변환하는 경우에도 동일한 축 매핑을 각 성분에 적용하면 됩니다.
- API가 행렬을 행우선/열우선으로 저장하는지, 벡터를 행/열로 해석하는지에 따라 곱셈 순서를 일관되게 유지.
- Three.js는 기본 카메라 전방이 -Z, Godot도 기본적으로 Y-up이므로 위 변환을 그대로 사용하면 시각적 정렬이 자연스럽게 맞습니다.

---


# Rotation before model matrix

좌표계 변환용 축 매핑 행렬을 따로 처리하기보다, 모델 행렬 앞에 **정렬 회전** 매트릭스를 한 번 곱해 두는 게 간단하고 재사용성이 좋습니다.

## Which side to multiply
- 열벡터 기준(대부분 그래픽스: gl_Position = P·V·M·p):
- M_target = R_align · M_cad
- 행벡터 기준(덜 흔함):
- M_target = M_cad · R_align
핵심은 “정렬 회전이 모델의 로컬 축을 바꾼다”는 점. 열벡터/열행렬 관례라면 앞쪽(좌측) 곱입니다.

## The alignment rotation (Z-up → Y-up, right-handed)
- 3×3 회전

$$
R_{\mathrm{align}}=\left[ \begin{matrix}1&0&0 ;& 0&0&1 ;& 0&-1&0\end{matrix}\right] \quad \Rightarrow \quad \hat {x}_t=\hat {x}_c,\  \hat {y}_t=\hat {z}_c,\  \hat {z}_t=-\hat {y}_c
$$

- 4×4 (동차)

$$
R_{\mathrm{align}}^{4\times 4}=\left[ \begin{matrix}1&0&0&0 ;& 0&0&1&0 ;& 0&-1&0&0 ;& 0&0&0&1\end{matrix}\right] 
$$

- 적용:

$$
M_{\mathrm{target}}=R_{\mathrm{align}}\cdot M_{\mathrm{cad}}
$$

- Example
- M_cad =

$$
\left[ \begin{matrix}1&0&0&2 ;& 0&1&0&0 ;& 0&0&1&5 ;& 0&0&0&1\end{matrix}\right] 
$$

- M_target = R_align · M_cad =

$$
\left[ \begin{matrix}1&0&0&2 ;& 0&0&1&5 ;& 0&-1&0&0 ;& 0&0&0&1\end{matrix}\right] 
$$

### Pros and cautions
- 장점
  - 간단: 한 번의 프리-로테이션으로 모든 모델에 적용 가능
  - 재사용: 동일 R_align을 에셋 파이프라인/셰이더 초기화에 공통 적용
  - 일관성: 오른손→오른손 유지, 프러스텀/카메라 규칙과 자연스럽게 맞음
- 주의
  - 곱셈 방향: 엔진의 벡터/행렬 관례 확인(열벡터면 좌곱)
  - 비균일 스케일: S가 포함된 M에 회전을 섞으면 축이 틀어질 수 있음
  - 권장 순서(열벡터): M = T · R_align · R_local · S
  - 피벗: R_align은 객체 로컬 축 기준 회전이므로 피벗(원점) 정의를 고려

### Quick snippets
- GLSL/표준 그래픽 파이프라인:
``` 
gl_Position = P · V · (R_align · M_cad) · vec4(pos,1);
```

- Three.js:
```rust
mesh.matrix = R_align4x4.clone().multiply(mesh.matrix);
```
- 또는
```rust
mesh.quaternion = q_align * mesh.quaternion
```
- Godot:
```rust
transform.basis = R_align3x3 * transform.basis
```
  - 또는 
```rust
transform = Transform3D(R_align3x3, Vector3.ZERO) * transform
```
---
# 📘 3D 그래픽스 좌표계와 변환 정리 문서
## 1. MC (Model Coordinate, 모델 좌표계)
- 정의: obj 등 메쉬 파일을 읽어들여 생성되는 개별 오브젝트의 로컬 좌표계
- 특징: 오브젝트 내부의 정점(vertex), 삼각형, 폴리곤 위치를 정의
  - 비유: 사람 오브젝트라면 팔, 머리, 다리의 위치가 MC 좌표로 표현됨

## 2. WC (World Coordinate, 세상 좌표계)
- 정의: 모든 오브젝트가 배치되는 전역 좌표계
- 특징: 오브젝트 간 상대적 위치를 표현
  - 예시: 손의 MC 좌표가 (-10,0,10), 사람 오브젝트가 WC에서 (100,100,100)에 있으면 손의 WC 좌표는 (90,100,110)

## 3. EC (Eye/Camera Coordinate, 카메라 좌표계)
- 정의: 카메라 위치와 방향을 기준으로 한 좌표계
- 특징: 물체가 카메라에 어떻게 보이는지를 표현
- 역할: WC까지는 “세상을 만드는 과정”, EC부터는 “세상을 2D 이미지로 그려내는 과정”
- 인자: 카메라 위치(position), 방향(orientation, uvn 벡터)

## 4. CC (Clip Coordinate, 클립 좌표계)
- 정의: Vertex Shader의 출력(gl_Position)에 해당하는 좌표계
- 특징: View Volume Clipping 후, 카메라에 보이는 부분만 남김
- 형태: 원근 투영 시 잘린 사각뿔(Frustum) 형태

## 5. NDC (Normalized Device Coordinate, 정규 디바이스 좌표계)
- 정의: CC를 -1~1 범위의 정육면체로 정규화한 좌표계
- 특징:
  - x, y, z ∈ [-1, 1]
  - 원근 나눗셈(Perspective Division)으로 w값을 1로 맞춤
  - OpenGL에서는 이 과정에서 오른손 → 왼손 좌표계로 바뀜
  - z=-1이 near, z=+1이 far

## 6. WdC (Window Coordinate, 윈도우 좌표계)
- 정의: 실제 화면(뷰포트)에 매핑된 2D 좌표계
- 특징:
- NDC를 뷰포트 크기(width, height)와 위치에 맞게 변환
- z는 깊이 버퍼(0~1)로만 사용, 화면에는 x,y만 표시
- 예시: 뷰포트 (100,100)~(300,200)일 때, NDC (0.4,0.3,1)은 WdC (180,130)에 매핑됨


## 📐 변환 과정 요약

| 단계 | 좌표계 | 변환 이름              | 설명                                      | 인자/특징                |
|------|--------|------------------------|-------------------------------------------|---------------------------|
| 1    | MC     | Modeling Transformation | 모델 로컬 → 월드 좌표 변환                 | translate, rotate, scale  |
| 2    | WC     | Viewing Transformation  | 월드 → 카메라 좌표 변환                    | 카메라 위치, 방향(uvn)    |
| 3    | EC     | Projection Transformation| 카메라 → 클립 좌표 변환                    | fov, near, far, frustum   |
| 4    | CC     | Perspective Division    | 클립 → NDC (w로 나눔)                      | 원근 나눗셈, handedness 변화 |
| 5    | NDC    | Viewport Transformation | NDC → 윈도우 좌표 변환                     | viewport width, height     |
| 6    | WdC    | Window Coordinate       | 최종 화면 픽셀 좌표                        | 실제 모니터/윈도우 좌표   |


## 📘 변환 종류
### 1. 3D Affine Transformation (아핀 변환)
- 정의: p'=Ap+v
- 특징: 평행성과 거리 비율 유지
- 예시: rotate, scale, translate, shearing
- 역변환 가능
### 2. Rigid Body Transformation (강체 변환)
- 정의: 길이와 각도 보존, scale 불허
- 특징: 이동과 회전만 허용
- 현실 물체에 대응하는 변환

## ✅ 결론
- MC → WC → EC → CC → NDC → WdC 순으로 좌표계가 변환되며,
- 각 단계는 행렬 연산으로 연결되어 최종적으로 화면 픽셀 좌표가 결정됨.
- 모델링은 “세상을 만드는 과정”, 뷰잉 이후는 **세상을 카메라로 그려내는 과정** 으로 이해하면 직관적임.


## 흐름도
```
   ┌───────────────┐
   │   MC          │
   │ Model Coord.  │
   │ (오브젝트 로컬)│
   └───────┬───────┘
           │  <Modeling Transformation>
           ▼
   ┌───────────────┐
   │   WC          │
   │ World Coord.  │
   │ (세상 좌표계)  │
   └───────┬───────┘
           │  <Viewing Transformation>
           ▼
   ┌───────────────┐
   │   EC          │
   │ Eye/Camera    │
   │ Coord.        │
   └───────┬───────┘
           │  <Projection Transformation>
           ▼
   ┌───────────────┐
   │   CC          │
   │ Clip Coord.   │
   │ (Frustum)     │
   └───────┬───────┘
           │  <Perspective Division>
           ▼
   ┌───────────────┐
   │   NDC         │
   │ Normalized    │
   │ Device Coord. │
   │ (-1 ~ +1 cube)│
   └───────┬───────┘
           │  <Viewport Transformation>
           ▼
   ┌───────────────┐
   │   WdC         │
   │ Window Coord. │
   │ (화면 픽셀)    │
   └───────────────┘
```
