# Slab 알고리즘
Slab 알고리즘은 **선분(또는 광선)과 AABB(Axis-Aligned Bounding Box, 축 정렬 박스)의 교차 여부** 를 빠르게 판정하는 방법입니다.

## 📌 핵심 아이디어
- AABB는 3차원 공간에서 x, y, z 축 방향으로 정렬된 직육면체입니다.
- 각 축에 대해 두 평행한 평면이 존재합니다. 예를 들어 x축에는 $x_{\min }$, $x_{\max }$ 평면이 있고, 그 사이 영역을 **slab(슬랩)** 이라고 부릅니다.
- 박스 전체는 세 축의 slab 교집합입니다:

$$
\mathrm{Box}=Slab_x\cap Slab_y\cap Slab_z
$$

- 따라서, 광선(ray)이 박스와 교차하는지 확인하려면:
  - 각 축별로 광선이 slab과 교차하는 구간 $[t_i^{low},t_i^{high}]$ 을 계산
  - 세 축의 구간을 모두 교집합
  - 교집합이 비어 있지 않으면 박스와 교차

## 🧮 수식 정리
- 광선(또는 선분)을 매개변수 형태로 표현:

$$
p(t)=o+t\cdot r
$$

  - $o=(o_x,o_y,o_z)$: 시작점(origin)
  - $r=(r_x,r_y,r_z)$: 방향 벡터(direction)
  - $t$: 매개변수 (선분이면 $0\leq t\leq 1$, 광선이면 $t\geq 0$)
- 각 축 i에 대해:

$$
t_i^{low}=\frac{l_i-o_i}{r_i},\quad t_i^{high}=\frac{h_i-o_i}{r_i}
$$

- l_i,h_i: 박스의 최소/최대 좌표

### 정리:

$$
t_i^{close}=\min (t_i^{low},t_i^{high}),\quad t_i^{far}=\max (t_i^{low},t_i^{high})
$$

- 최종 교집합:

$$
t^{close}=\max _i\{ t_i^{close}\} ,\quad t^{far}=\min _i\{ t_i^{far}\}
$$

- 조건:

$$  
t^{close}\leq t^{far}
$$

- 이면 교차 존재.

## ✨ 특징
- 빠름: 단순한 사칙연산만으로 교차 여부 판정 가능
- 안정적: 분기(branch)가 거의 없어 CPU에서 효율적
- 활용 분야:
- 컴퓨터 그래픽스(레이 트레이싱)
- 충돌 판정(게임, 로보틱스)
- R‑Tree 같은 공간 인덱스에서 선분/광선 검색

- 👉 정리하면, **Slab 알고리즘은 “광선을 각 축 slab과 교차시켜 구간을 얻고, 세 구간의 교집합을 검사하는 방식** 입니다.

---


