# Curvaute Tensor (C++ 코드)
- Parasolid rust 버전은 아직 없음
- C++로 부터 코드 생성

## Curvaute Tensor Matrix
- C(0,0), C(0,1), C(1,1)은 모두 **두 번째 미분 벡터와 법선 벡터의 내적(dot product)** 이다.
- 즉,
- $C_{00}=\langle P_{uu},N\rangle$ 
- $C_{01}=\langle P_{uv},N\rangle$ 
- $C_{11}=\langle P_{vv},N\rangle$ 
- 여기서
  - $P_{uu} = \frac{\partial ^2P}{\partial u^2}$
  - $P_{uv} = \frac{\partial ^2P}{\partial u\partial v}$
  - $P_{vv} = \frac{\partial ^2P}{\partial v^2}$
  - N = 단위 법선 벡터

## 📘 왜 dot product인가?
- 곡면의 **Shape Operator(곡률 텐서)** 는 두 번째 기본형식(Second Fundamental Form)의 계수로 구성됨.

- 두 번째 기본형식은:

```math
\begin{aligned}e&=\langle P_{uu},N\rangle \\ f&=\langle P_{uv},N\rangle \\ g&=\langle P_{vv},N\rangle \end{aligned}
```

- 그리고 `Surface::get_curvatures()` 가 반환하는 C 행렬은:

```math
C=\left[ \begin{matrix}e&f\\ f&g\end{matrix}\right] =\left[ \begin{matrix}\langle P_{uu},N\rangle &\langle P_{uv},N\rangle \\ \langle P_{uv},N\rangle &\langle P_{vv},N\rangle \end{matrix}\right]
``` 
- 즉, 두 번째 미분 벡터를 법선 방향으로 투영한 값이 곡률 텐서의 성분이 된다.

## 🎯 직관적으로 설명하면
- $P_{uu}$ 는 u 방향으로 얼마나 “휘는지”
- $P_{vv}$ 는 v 방향으로 얼마나 “휘는지”
- $P_{uv}$ 는 u와 v가 섞여서 휘는 정도
- 이걸 법선 방향으로 투영(dot product) 하면 곡면의 실제 곡률 성분이 된다.

## Parasolid PK_SURF_eval_with_normal
- surf            → Parasolid surface handle
- uv              → (u, v) 파라미터
- n_u_derivs      → 2
- n_v_derivs      → 2
- triangular      → PK_LOGICAL_false   (직사각형 배열이 더 다루기 쉬움)
- p[]             → 결과 벡터 배열 (길이 = (n_u+1)*(n_v+1) = 9)


### 📘 PK_SURF_eval_with_normal()이 반환하는 p[] 배열 구조 (triangular=false)
- n_u_derivs = 2, n_v_derivs = 2 이므로:
```
index = i + (n_u_derivs+1) * j = i + 3*j
```

| i (u-deriv) | j (v-deriv) | 의미     | index |
|-------------|-------------|----------|--------|
| 0           | 0           | P(u,v)   | 0      |
| 1           | 0           | Pᵤ       | 1      |
| 2           | 0           | Pᵤᵤ      | 2      |
| 0           | 1           | Pᵥ       | 3      |
| 1           | 1           | Pᵤᵥ      | 4      |
| 2           | 1           | Pᵤᵤᵥ     | 5      |
| 0           | 2           | Pᵥᵥ      | 6      |
| 1           | 2           | Pᵤᵥᵥ     | 7      |
| 2           | 2           | Pᵤᵤᵥᵥ    | 8      |

- 곡률 텐서에 필요한 건:
  - Pᵤᵤ → p[2]
  - Pᵤᵥ → p[4]
  - Pᵥᵥ → p[6]

## C++ 코드
```cpp
bool GeomFace::evalCurvauteTensor(double u, double v, double tensor[4])
{
	PK_SURF_t surf;
	if (PK_FACE_ask_surf(m_hItem, &surf) != PK_ERROR_no_errors)
	{
		return false;
	}
	
	PK_UV_t uv;
	uv.param[0] = u;
	uv.param[1] = v;

	PK_VECTOR_t p[12];
	PK_VECTOR1_t normal;
	if (PK_SURF_eval_with_normal(surf, uv, 2, 2, PK_LOGICAL_false, p, &normal) != PK_ERROR_no_errors)
	{
		return false;
	}

	PK_VECTOR_t Pu = p[1];
	PK_VECTOR_t Pv = p[3];
	PK_VECTOR_t Puu = p[2];
	PK_VECTOR_t Puv = p[4];
	PK_VECTOR_t Pvv = p[6];

	tensor[0] = VEC_DOT(Puu.coord, normal.coord);
	tensor[1] = VEC_DOT(Puv.coord, normal.coord);
	tensor[2] = VEC_DOT(Puv.coord, normal.coord);
	tensor[3] = VEC_DOT(Pvv.coord, normal.coord);
	return true;
}
```

## CM2 Input 코드
```cpp
int get_curvatures(const cm2::DoubleMat& pos2D, cm2::DoubleMat& H) const
{
    const size_t         NODS = pos2D.cols();
    double               u, v;
    if (pos2D.cols() != H.cols())       return -1;
    if (pos2D.empty())                  return 0;
    if (H.rows() != 3)                  return -1;     // Error.
    if (pos2D.rows() != 2)              return -1;     // Error.
    for (size_t j = 0; j < NODS; ++j)
    {
        u = pos2D(0, j);
        v = pos2D(1, j);
        double tensor[4] = { 0.0, 0.0, 0.0, 0.0 };
        m_pFace->evalCurvauteTensor(u, v, tensor);
        H(0, j) = tensor[0];
        H(1, j) = tensor[1];
        H(2, j) = tensor[3];
    }
    return 0;
}
```
---


