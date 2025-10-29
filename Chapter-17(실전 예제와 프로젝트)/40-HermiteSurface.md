Hermite Surface는 네 개의 꼭짓점 위치, 각 꼭짓점에서의 접선 벡터, 그리고 꼬임 벡터(twist vector)를 기반으로 정의되는 곡면으로, 곡선의 Hermite 보간을 2차원으로 확장한 형태입니다. 주로 곡면 설계나 CAD에서 경계 조건을 명확히 제어할 수 있는 장점 때문에 사용됩니다.

📐 Hermite Surface란?
Hermite Surface는 두 개의 매개변수 u와 v를 기반으로 정의되는 이차원 보간 곡면입니다. 이 곡면은 다음과 같은 16개의 기하 정보로 구성됩니다:
- 4개의 꼭짓점 위치:
$P_{00}$, $P_{01}$, $P_{10}$, $P_{11}$
- 8개의 접선 벡터 (각 꼭짓점에서 u, v 방향):
$P_{00}^u$, $P_{00}^v$, $P_{01}^u$, $P_{01}^v$,
- 4개의 꼬임 벡터 (혼합 방향 uv):
P_{00}^{uv}, P_{01}^{uv}, P_{10}^{uv}, P_{11}^{uv}
이러한 정보를 통해 곡면의 형태와 경계 조건을 직접 제어할 수 있습니다.

📊 수식 구조
Hermite Surface는 다음과 같은 형태의 이중 Hermite 보간식으로 표현됩니다:
S(u, v) = \sum_{i=0}^{3} \sum_{j=0}^{3} h_i(u) \cdot h_j(v) \cdot C_{ij}
여기서:
- h_i(u), h_j(v): Hermite basis functions
- C_{ij}: 16개의 기하 정보 (위치, 접선, 꼬임 벡터)
Hermite basis function은 다음과 같이 정의됩니다:
$$
\begin{aligned}
h_0(t) &= 2t^3 - 3t^2 + 1 \\
h_1(t) &= -2t^3 + 3t^2 \\
h_2(t) &= t^3 - 2t^2 + t \\
h_3(t) &= t^3 - t^2
\end{aligned}
$$

따라서 전체 곡면은 다음과 같이 행렬 형태로 표현할 수 있습니다:
S(u, v) = [u^3 \quad u^2 \quad u \quad 1] \cdot M_u \cdot G \cdot M_v^T \cdot [v^3 \quad v^2 \quad v \quad 1]^T
여기서:
- M_u, M_v: Hermite basis 행렬
- G: 4×4 기하 정보 행렬 (16개의 벡터)

🧠 특징 요약
- 정의 요소: 4 위치 + 8 접선 + 4 꼬임 = 16 벡터
- 연속성: 각 패치 내부는 C^2, 패치 간 연결은 C^1 가능
- 제어력: 경계 조건을 직접 지정 가능
- 용도: CAD, 곡면 모델링, 물리 기반 시뮬레이션
