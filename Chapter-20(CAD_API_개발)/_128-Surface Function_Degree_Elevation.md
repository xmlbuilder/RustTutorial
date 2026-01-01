# 📘 Surface Function Degree Elevation
- Bezier Surface Function Degree Elevation은 Bezier Surface의 control point가 아닌,  
    control value(스칼라 값) 그리드에 대해 차수를 올리는 알고리즘이다.
- 즉, 다음과 같은 2D 스칼라 배열:
    - $f_{i,j}$ 에 대해 U 또는 V 방향으로 차수를 올려서 $g_{i,j}$ 를 만드는 과정이다.

## 1. Surface Function이란?
- Bezier Surface는 보통 4D control point $P_{i,j}$ 로 정의되지만,  
    일부 알고리즘에서는 스칼라 값 그리드가 필요하다.
- 예:
    - 곡면의 높이값(height field)
    - 곡면의 스칼라 필드(온도, 압력 등)
    - 곡면의 weight function
    - 곡면의 partial derivative coefficient
    - 곡면의 basis function 변환
- 이런 경우, 각 grid cell은 단순한 실수 값이다:
```math
f_{i,j}\in \mathbb{R}
```
- 이 스칼라 값 배열에 대해 Bezier 차수 상승을 적용하는 것이 바로 Surface Function Degree Elevation이다.

## 2. 문제 정의
- 원래 차수: r
    - 증가량: t
    - 새 차수: r+t
- 원래 스칼라 값:
```math
f_0,f_1,\dots ,f_r
```
- 새 스칼라 값:
```math
g_0,g_1,\dots ,g_{r+t}
```
- 이때, Bezier 곡선 차수 상승과 동일한 방식으로 스칼라 값도 변환해야 한다.

## 3. 핵심 수학
- Bezier function:
```math
F(u)=\sum _{i=0}^rf_iB_{i,r}(u)
```
- 차수 상승 후:
```math
F(u)=\sum _{k=0}^{r+t}g_kB_{k,r+t}(u)
```
- 두 식이 동일한 함수가 되려면:
```math
g_k=\sum _{i=\max (0,k-t)}^{\min (k,r)}E[k][i]\cdot f_i
```
- 여기서 E 는 degree elevation matrix:
```math
E[k][i]=\frac{{r \choose i}{t \choose k-i}}{{r+t \choose k}}
```
## 4. Surface Function에 적용
- Surface Function은 2D grid이므로:
    - UDIR: column 고정, u 방향으로 차수 상승
    - VDIR: row 고정, v 방향으로 차수 상승
- UDIR (u 방향)
```math
g_{i,roc}=\sum _{k=\max (0,i-t)}^{\min (i,r)}E[i][k]\cdot f_{k,roc}
```
- VDIR (v 방향)
```math
g_{roc,j}=\sum _{k=\max (0,j-t)}^{\min (j,r)}E[j][k]\cdot f_{roc,k}
```
- 여기서:
    - roc: row or column index
    - f,l: 계산할 index 범위
    - E: degree elevation matrix

## 5. 왜 Row/Column 단위로 처리하는가?
- Bezier Surface는 tensor product 구조:
```math
S(u,v)=\sum _iB_{i,p}(u)\sum _jB_{j,q}(v)f_{i,j}
```
- 따라서:
    - U 방향 차수 상승 → 각 V column을 독립적인 Bezier function으로 처리
    - V 방향 차수 상승 → 각 U row를 독립적인 Bezier function으로 처리
- 이 방식이:
    - 수학적으로 정확
    - 계산 효율적
    - 메모리 접근이 단순
    - CAD/NURBS 표준 방식

## 6. 알고리즘 절차 (Pseudo Code)
- UDIR
```rust
for i = f..l:
    a = max(0, i - t)
    b = min(i, r)
    g[i][roc] = Σ_{k=a..b} E[i][k] * f[k][roc]
```

- VDIR
```rust
for j = f..l:
    a = max(0, j - t)
    b = min(j, r)
    g[roc][j] = Σ_{k=a..b} E[j][k] * f[roc][k]
```


## 7. Rust 구현과의 연결
- Rust 함수:
```rust
pub fn on_bezier_surface_function_degree_elevate_rowcol(...)
```
- fp → 원래 스칼라 grid
- fq → 새 스칼라 grid
- rm → degree elevation matrix
- dir → UDIR / VDIR
- f,l → 계산 범위
- roc → row/column index

## 8. 예시
- 원래 값:
```
f=[10,20,30]
```
- 차수 상승: r=2,t=1→r+t=3
    - Elevation matrix:
```math
E=\left[ \begin{matrix}1&0&0\\ \frac{2}{3}&\frac{1}{3}&0\\ 0&\frac{1}{3}&\frac{2}{3}\\ 0&0&1\end{matrix}\right]
``` 
- 새 값:
```math
g_0=1\cdot 10=10
```
```math
g_1=\frac{2}{3}10+\frac{1}{3}20=13.33
```
```math
g_2=\frac{1}{3}20+\frac{2}{3}30=26.66
```
```math
g_3=1\cdot 30=30
```

## 9. 결론
- Surface Function Degree Elevation은:
    - ✔ Bezier function의 차수 상승을
    - ✔ Surface의 row/column 단위로 적용하는 알고리즘이다.
    - ✔ 스칼라 값이므로 control point가 아닌 control value에 적용된다.
    - ✔ 정확한 수학적 변환이며, 형상(함수 형태)을 완전히 보존한다.
    - ✔ degree elevation matrix 기반의 가중합으로 계산된다.
    - ✔ CAD/NURBS 시스템에서 매우 중요한 기본 연산이다.

---

## 🔥 1. 단순 보간이 아니다 → “조합 기반의 공학적 보간”이다
- 일반적인 보간(interpolation)은:
    - 두 값 사이를 선형으로 잇거나
    - 스플라인으로 부드럽게 잇거나
    - 단순한 곡선 fitting을 한다
- 즉, 기존 값 사이를 채우는 것이 목적이다.
- 하지만 Bezier degree elevation은 완전히 다르다.
- ✔ 기존 값의 **선형 보간** 이 아니라
- ✔ 기존 값들의 **이항계수 기반 조합(Weighted Combination)** 이다.
- 수식:
```math
g_k=\sum _{i=\max (0,k-t)}^{\min (k,r)}\frac{{r \choose i}{t \choose k-i}}{{r+t \choose k}}f_i
```
- 이건 단순한 보간이 아니라:
    - 확률적 조합
    - Bernstein basis 변환
    - 조합론적 가중합
    - 형상 보존 변환
- 이 네 가지 성질을 동시에 가진다.

## 🔧 2. 왜 공학 보간(Engineering Interpolation)에 유리한가?
- ✔ (1) 형상 보존 (Shape-preserving)
    - 차수를 올려도 원래 함수/곡선/곡면의 형상이 100% 유지된다.
    - 즉,
        - 값은 바뀌지만
        - 함수는 바뀌지 않는다
    - 이건 공학에서 매우 중요하다.
    - 예:
        - FEM shape function
        - CFD boundary interpolation
        - CAD 곡면 refinement
        - NURBS 기반 해석
    - 이런 곳에서는 형상 보존 + 해상도 증가가 필수다.

- ✔ (2) 안정성 (Numerical Stability)
    - Bernstein basis는 수치적으로 매우 안정적이다.
        - 선형 보간보다 안정적
        - 고차 스플라인보다 안정적
        - FEM shape function과 동일한 안정성
    - 그래서 공학 계산에서 선호된다.

- ✔ (3) 부드러운 고차 보간 가능
    - 차수를 올리면:
        - 더 많은 control value 생성
        - 더 부드러운 고차 함수 표현 가능
        - 고차 미분도 안정적으로 계산 가능
    - 이는 FEM/CFD에서 고차 요소(p-refinement) 와 동일한 개념이다.

- ✔ (4) Tensor-product 구조 → 2D/3D 확장 쉬움
    - U/V 방향 독립적으로 차수를 올릴 수 있기 때문에:
        - 2D 스칼라 필드
        - 3D 스칼라 필드
        - Surface function
        - Volume function
    - 모두 동일한 방식으로 확장 가능.
    - 이건 공학에서 mesh refinement 와 동일한 개념이다.

## 3. Difference From Simple Interpolation

| Method                 | Mathematical Nature                     | Behavior                          | Engineering Meaning                     |
|------------------------|------------------------------------------|------------------------------------|------------------------------------------|
| Linear Interpolation   | Simple linear blend                      | Connects two values directly       | Low accuracy, not shape-preserving       |
| Spline Interpolation   | Piecewise polynomial                     | Smooth curve between samples       | Smooth but does NOT preserve original    |
| Bezier Degree Elevation | Binomial-coefficient weighted combination | Generates new control values from all neighbors | Shape-preserving, stable, engineering-grade refinement |

- 즉, Bezier degree elevation은 **보간** 이 아니라 **기능적 고차화(Function Refinement)** 이다.

## 🚀 4. 그래서 Surface Function Degree Elevation이 강력한 이유
- 2D 스칼라 배열에 적용하면:
    - 기존 surface function의 형상은 그대로
    - 해상도만 증가
    - 중간 값은 단순 보간이 아니라 수학적으로 최적의 조합
    - 공학적 해석에 바로 사용 가능
- 예:
    - 곡면의 weight function refinement
    - 곡면의 partial derivative field refinement
    - FEM shape function 고차화
    - CFD boundary condition smoothing
    - CAD surface refinement

## 🎯 5. 결론
- ✔ 단순 보간이 아니다
- ✔ 기존 값들의 조합으로 만들어지는 “공학적 고차 보간”이다
- ✔ 형상 보존 + 안정성 + 고차화
- ✔ 공학 해석(FEM/CFD/CAD)에 매우 유리
- ✔ 2D/3D 스칼라 필드에도 그대로 적용 가능

---

## 소스 코드
```rust
/// 편의 헬퍼:
/// degree elevation matrix를 내부에서 생성해서 한 row/column만 올림.
///
/// - fp  : (old_deg+1) x N (UDir) or N x (old_deg+1) (VDir)
/// - old_deg : r
/// - t   : increment
/// - dir : elev dir
/// - f,l : 범위 (i 또는 j)
/// - roc : row / column index
/// - fq  : (old_deg+t+1) x N or N x (old_deg+t+1)
pub fn on_bezier_surface_function_degree_elevate_rowcol(
    fp: &[Vec<f64>],
    old_deg: usize,
    t: usize,
    dir: SurfaceDir,
    f: usize,
    l: usize,
    roc: usize,
    fq: &mut [Vec<f64>],
) {
    let rm = on_degree_elevation_matrix(old_deg, t);
    on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
        fp, old_deg, t, &rm, dir, f, l, roc, fq,
    );
}

```
