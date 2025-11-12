# Conic Bézier Arc

3D 공간에서 **원뿔형 베지어 곡선(Conic Bézier Arc)** 을 구성하기 위한 수학적 절차를 구현한 것으로,  
수식적으로 매우 타당한 구조를 갖고 있습니다.  
아래에 전체 흐름을 수식 기반으로 해석, 단계별 설명, 그리고 핵심 수식 요약으로 정리.

## 📘 전체 목적
주어진 세 점과 접선 벡터를 기반으로, Conic Bézier Curve의 중간 제어점 $P_1$ 과 가중치 $w_1$ 를 계산합니다.  
### 입력:
- 시작점 $P_0$, 끝점 $P_2$, 중간점 $P$
- 접선 벡터 $T_0$, $T_2$

### 이미지

![Conic Bezier Arc](/image/conic_Bezier_arc.png)


### 출력:
- 제어점 $P_1$
- 가중치 $w_1$

## 🧠 단계별 수식 해석
### ① 평면 프레임 생성 (on_make_frame_plane)
- 벡터 정의:

$$
\vec {a}=P_2-P_0,\quad \vec {b}=P-P_0
$$

- 법선 벡터:

$$
\vec {z}=\vec {a}\times \vec {b},\quad \vec {z}\leftarrow \frac{\vec {z}}{|\vec {z}|}
$$

- 기준 축:

$$
\vec {x}=\frac{\vec {a}}{|\vec {a}|},\quad \vec {y}=\vec {z}\times \vec {x}
$$

### ② 3D → 2D 투영 (on_project_point_2d)
- 투영된 점:

$$
P_i^{2D}=\left( (P_i-O)\cdot \vec {x},\  (P_i-O)\cdot \vec {y}\right)
$$

- 투영된 벡터:

$$
T_i^{2D}=\left( T_i\cdot \vec {x},\  T_i\cdot \vec {y}\right)
$$ 

### ③ 접선 교차점 계산 (on_intersect_lines_2d)
- 두 직선:

$$
L_0(t)=P_0^{2D}+t\cdot T_0^{2D},\quad L_2(s)=P_2^{2D}+s\cdot T_2^{2D}
$$

- 교차점 $P_1^{2D}$ 

#### 계산:

$$
\mathrm{det}=T_0^{2D}\times T_2^{2D}\neq 0\Rightarrow \mathrm{교차\  가능}
$$


### ④ 중간점과 교차점 사이의 위치 비율 계산
- 선분 $P_0^{2D}\rightarrow P_2^{2D}$ 와 직선 $P_1^{2D}\rightarrow P^{2D}$ 의 교차 비율:

$$
t_{\mathrm{seg}}=\mathrm{교차\  위치\  비율}\quad a=\sqrt{\frac{t_{\mathrm{seg}}}{1-t_{\mathrm{seg}}}},\quad u=\frac{a}{1+a}
$$


### ⑤ 가중치 계산
- 벡터 정의:

$$
\vec {v}_0=P-P_0,\quad \vec {v}_1=P_1-P,\quad \vec {v}_2=P-P_2
$$

- 내적:

$$
\alpha =\vec {v}_0\cdot \vec {v}_1,\quad \beta =\vec {v}_1\cdot \vec {v}_2,\quad \gamma =\vec {v}_1\cdot \vec {v}_1
$$

- 가중치:

$$
w_1=\frac{(1-u)^2\cdot \alpha +u^2\cdot \beta }{2u(1-u)\cdot \gamma }
$$

### ⑥ 평면 복원
- 2D 제어점 $P_1^{2D}$ 을 3D로 복원:

$$
P_1=O+P_1^{2D}.x\cdot \vec {x}+P_1^{2D}.y\cdot \vec {y}
$$

### ⑦ 평행 접선 처리 (Parabola Branch)
- 접선이 평행할 경우, $w_1=0$
- $P_1$ 은 $T_0$ 방향으로 스케일된 벡터로 표현됨

## ✅ 수식적 타당성 검토
- 기하학적으로 완전 타당: 평면 생성 → 투영 → 교차점 → 위치 비율 → 가중치 계산 → 복원
- 수치적 안정성 고려: eps로 병렬성, 분모 0, 경계값 처리
- 분기 처리 명확: 일반형 vs. 평행형
- 벡터 연산과 내적 기반: Bézier 곡선의 정의에 부합

---


