# Coons Patch Builder

- 사각 경계(네 개의 곡선 또는 폴리라인)로 둘러싸인 영역을 내부로 매끄럽게 보간하는 **Coons 보간(Côons Patch / Transfinite Interpolation)** 의  
  수학적 요약과, 이를 유한 격자(mesh)로 구현하는 절차를 정리한 문서입니다.

---

## 1) 문제 설정

사각형 경계를 구성하는 네 개의 경계 곡선을 다음과 같이 둡니다. 매개변수는 정규화 구간 $[0,1]$ 입니다.

- 하변(아래): $\mathbf{B}(s),\ s\in[0,1]$ (left $\to$ right)
- 상변(위): $\mathbf{T}(s),\ s\in[0,1]$ (left $\to$ right)
- 좌변: $\mathbf{L}(t),\ t\in[0,1]$ (bottom $\to$ top)
- 우변: $\mathbf{R}(t),\ t\in[0,1]$ (bottom $\to$ top)

네 모서리(코너) 점은

$$
\begin{aligned}
\mathbf{C}_{00}&=\mathbf{L}(0)=\mathbf{B}(0), &
\mathbf{C}_{10}&=\mathbf{R}(0)=\mathbf{B}(1), \\
\mathbf{C}_{01}&=\mathbf{L}(1)=\mathbf{T}(0), &
\mathbf{C}_{11}&=\mathbf{R}(1)=\mathbf{T}(1).
\end{aligned}
$$

### **방향 규약**  
- 본 문서와 제공된 구현은 **B, T는 좌→우**, **L, R은 하→상** 방향으로 가정합니다.    
  이 규약이 어긋나면 코너 일치가 깨지고 접힘/꼬임이 생길 수 있으므로, 필요 시 입력을 뒤집어 맞춰야 합니다.

---

## 2) Coons 패치: Transfinite 보간식

Coons 보간은 “두 집합의 선형 보간 합”에서 “코너의 bilinear 중복”을 제거하여 내부를 정의합니다.

### 2.1 혼합(blending) 구성요소

- **수평 보간(상·하 경계):**

$$
  \mathbf{S}(s,t)=(1-t) \mathbf{B}(s)+t \mathbf{T}(s)
$$

- **수직 보간(좌·우 경계):**

$$
  \mathbf{T r}(s,t)=(1-s) \mathbf{L}(t)+s \mathbf{R}(t)
$$

- **중복 보정(코너의 bilinear):**
$$
  \mathbf{B l}(s,t)=
  (1-s)(1-t) \mathbf{C}_{00}
  + s(1-t) \mathbf{C}_{10}
  + (1-s)t \mathbf{C}_{01}
  + st \mathbf{C}_{11}
$$

### 2.2 Coons 패치 정의

$$
\boxed{
\mathbf{C}(s,t)=\mathbf{S}(s,t)+\mathbf{T r}(s,t)-\mathbf{B l}(s,t)
}
$$

이 식은 경계에서 정확히 경계 곡선을 재현합니다. 예를 들어 $t=0$ 이면
$\mathbf{C}(s,0)=\mathbf{B}(s)$, $s=1$ 이면 $\mathbf{C}(1,t)=\mathbf{R}(t)$ 등.

> **직관**  
> $\mathbf{S}$ 와 $\mathbf{T r}$ 을 단순 합하면 코너가 **두 번** 더해집니다. $\mathbf{B l}$ 은 정확히 그 중복을 제거하는 **bilinear 보정**입니다.

---

## 3) 이산화(메시 생성)

실사용에서는 내부를 격자 샘플로 이산화하여 정점/면을 만듭니다.

### 3.1 파라미터 샘플

정수 해상도 $\(N_u, N_v \ge 2\)$ 를 정하고,

$$
s_i = \frac{i}{N_u-1}\quad (i=0,\dots,N_u-1),\qquad
t_j = \frac{j}{N_v-1}\quad (j=0,\dots,N_v-1).
$$

각 격자점의 3D 위치는

$$
\mathbf{P}_{ij}=\mathbf{C}(s_i,t_j)
$$

로 계산합니다.

### 3.2 코너 강제 일치

실제 경계 입력이 완벽히 일치하지 않을 수 있습니다(수치 오차, 데이터 노이즈 등).    
**force-corner-match** 옵션은 $\mathbf{B},\mathbf{T},\mathbf{L},\mathbf{R}$ 의 끝점들을  
𝐂₀₀, 𝐂₁₀, 𝐂₀₁, 𝐂₁₁ 에 스냅시켜 모순을 제거합니다.

### 3.3 면 생성

- **사각형 메쉬(quad mesh):**  
  셀 $\((i,j)\)$ 에서 정점 인덱스를  

$$
  n_{00}=(i,j),\ n_{10}=(i+1,j),\ n_{11}=(i+1,j+1),\ n_{01}=(i,j+1)
$$

  로 잡고, 면을 $(n_{00},n_{10},n_{11},n_{01})$ 순으로 생성합니다.

- **삼각형 메쉬(tri mesh):**  
  같은 셀을 삼각형 두 장으로 나눕니다. 분해 스타일은 세 가지를 제공합니다.
  - **AlignLeft:** $(n_{00},n_{10},n_{01})$, $(n_{10},n_{11},n_{01})$
  - **AlignRight:** $(n_{00},n_{10},n_{11})$, $(n_{00},n_{11},n_{01})$
  - **UnionJack:** 격자 parity를 번갈아 가며 좌/우 대각선을 교차 배치해 장기적인 방향 편향을 줄입니다.

### 3.4 노멀/텍스처 좌표

- **정점 노멀:** 각 면의 법선을 인접 정점에 누적 후 정규화(라플라시안 스무딩 계열과 호환).
- **텍스처 좌표:** $(u,v)=(s_i,t_j) \in [0,1]^2$ 를 그대로 저장하면 UV 매핑이 자연스럽게 정사영됩니다.

---

## 4) 아크길이(호장) 기반 매개화 (선택)

폴리라인/곡선 샘플이 **불균일**하면, 균일한 $s,t$ 샘플만으로는 시각적 왜곡이 있을 수 있습니다.  
이를 완화하려면 경계의 누적 길이를 정규화한 **아크길이 파라미터** $\tilde{s}(i),\tilde{t}(j)$ 를 사용합니다.

1. 경계 각 선분 길이를 누적해 $\ell_k$ 를 얻고, 전체 길이로 나누어 $[0,1]$ 로 정규화합니다.
2. 이 $\tilde{s},\tilde{t}$ 는 **경계 파라미터의 기록/매핑 정보**로 쓰거나, 필요 시 경계를 재표본(resample)하여  
   $s_i, t_j$ 에 대응하는 점을 새로 얻는 데 사용할 수 있습니다.

> 제공된 레퍼런스 구현은 “**지오메트리는 입력 샘플 그대로**” 두고, 맵만 아크길이 축척을 **기록**하는 선택지를 포함합니다.  
> 더 고른 내부 분포가 필요하면, 경계를 아크길이 균등으로 **재표본**한 후 Coons 보간에 투입하세요.

---

## 5) 수치적 고려사항

- **접힘/자체 교차(folding):** 경계가 심하게 비선형·상호 교차에 가까우면 내부가 접힐 수 있습니다.  
  *대응:* 더 조밀한 샘플링, 경계 재파라미터화, 보간 이후의 라플라시안/윈슬로(Winslow) 스무딩 적용.
- **방향 불일치:** B/T, L/R의 진행 방향이 문서 규약과 다르면 코너가 맞지 않거나 뒤틀립니다.  
  *대응:* 시작·끝 점을 비교해 필요 시 경계를 뒤집어 방향을 일치시킵니다.
- **정수 격자 해상도:** \(N_u,N_v\)가 너무 낮으면 각지게 보이고, 너무 높으면 계산/메모리 비용이 큽니다.  
  실무에선 곡률 기반 적응 샘플링이나 멀티해상도 접근을 자주 사용합니다.

---

## 6) 구현 개요 (의사코드)

```text
input: bottom[0..Nu-1], top[0..Nu-1], left[0..Nv-1], right[0..Nv-1]
output: vertices[Nu*Nv], faces

C00=left[0],  C01=left[Nv-1]
C10=right[0], C11=right[Nv-1]

for iu in 0..Nu-1:
  s = iu/(Nu-1)
  for iv in 0..Nv-1:
    t = iv/(Nv-1)
    S  = (1-t)*B(s) + t*T(s)                  // 수평 보간
    Tr = (1-s)*L(t) + s*R(t)                  // 수직 보간
    Bl = (1-s)(1-t)*C00 + s(1-t)*C10
       + (1-s)t*C01  + st*C11                 // bilinear 코너 보정
    P(iu,iv) = S + Tr - Bl

// 면 생성: quad 또는 tri(분해 스타일 선택)
```

---

## 7) Rust API 예시 (요약)

이 문서와 함께 제공된 Rust 구현의 핵심 시그니처:

```rust
pub struct CoonsOptions {
    pub quad_mesh: bool,
    pub tri_style: TriStyle,        // AlignLeft | AlignRight | UnionJack
    pub build_normals: bool,
    pub build_texcoord: bool,       // (u,v) = (s,t)
    pub use_arclen_sampling: bool,  // 경계 파라미터 기록용
    pub force_corner_match: bool,
}

pub fn build_coons_patch_mesh(
    bottom: &[Vec3f], right: &[Vec3f], top: &[Vec3f], left: &[Vec3f],
    opt: &CoonsOptions, want_maps: bool
) -> Result<(Mesh, Option<CoonsBoundaryMaps>), String>;
```

사용 시 **경계 방향 규약** (B/T 좌→우, L/R 하→상)을 반드시 지켜야 함.

---

## 8) 참조

- S. A. Coons, *Surfaces for Computer-Aided Design of Space Forms*, 1967.  
- J. Hoschek & D. Lasser, *Fundamentals of Computer Aided Geometric Design*, 1993.  
- Piegl & Tiller, *The NURBS Book*, 2nd ed., 1997 — Coons 패치와 관련된 transfinite interpolation 개요.

---

## 9) 체크리스트

- [ ] 경계 방향(B/T 좌→우, L/R 하→상) 확인/보정  
- [ ] 코너 일치 여부 확인(필요 시 스냅)  
- [ ] 적절한 \(N_u,N_v\) 선택 및(또는) 아크길이 기반 재표본  
- [ ] 쿼드/트라이 분해 방식 선택(렌더러/후처리 파이프라인 고려)  
- [ ] 노멀/UV 생성 및 검증  
- [ ] 접힘 검사(시각/법선 부호/자체교차), 필요 시 스무딩/재파라미터화

---

Coons 패치는 단순한 선형 혼합과 bilinear 보정만으로 **경계 충실도** 를 유지하면서 내부를 채워 주는 강력한 기본 블록입니다.  

