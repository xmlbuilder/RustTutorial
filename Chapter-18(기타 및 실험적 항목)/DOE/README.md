
# DOE
Rust에서 구현한 DOE(Taguchi), Response Surface, ANOM 분석 소스

## 📘 Rust 기반 DOE / Response Surface / ANOM 분석 시스템 정리

Rust로 구축한 `실험 설계(Design of Experiments)`, `반응표면(Response Surface) 회귀`,  
`ANOM(Analysis of Means) 분석` 시스템을 요약한 기술 문서이다.  
전체 라이브러리는 다음 3개 축으로 구성된다.

## 1. Taguchi Orthogonal Arrays (정규 직교 배열)

### ✔ 정규 직교 배열 구조체
```rust
pub struct OrthogonalArray {
    pub runs: usize,
    pub factors: usize,
    pub levels: usize,
    pub data: Vec<usize>, // row-major (run x factor)
}
```
### ✔ 대표 OA 하드코딩 생성 함수

- L4(2³)
- L8(2⁷)
- L9(3⁴)
- L12(2¹¹)
- L16(2¹⁵)


#### 예:
```rust
pub fn oa_l8_2_7() -> OrthogonalArray { ... }
```

## 2. Factor Levels 구조화

- DOE에서 각 요인의 레벨 값을 설정하는 구조:
```rust
pub struct FactorLevels {
    pub levels: Vec<f64>,   // e.g., [-1.0, 1.0]
}
```
- Taguchi OA의 0,1,2 레벨 인덱스를 이 구조체가 받아 실제 실험 변수값으로 변환한다.

## 3. Response Surface Quadratic Model (2차 반응표면 모델)
- Rust에서 완전한 2차 회귀(Quadratic Regression) 을 nalgebra 형태로 재구현함.

### ✔ 모델 형태

반응표면은 다음과 같은 구조를 가짐:
2차 반응표면 회귀모형 수식

y = β₀ + ∑ᵢ βᵢ·xᵢ + ∑ᵢ βᵢᵢ·xᵢ² + ∑{ᵢ<ⱼ} βᵢⱼ·xᵢ·xⱼ

#### 🔎 구성 요소 설명
- $\beta _0$: 절편(intercept)
- $\beta _ix_i$: 각 요인의 선형항(linear term)
- $\beta _{ii}x_i^2$: 각 요인의 제곱항(quadratic term)
- $\beta _{ij}x_ix_j$: 요인 간 교호작용항(interaction term)




### ✔ 설계 행렬 구성 (Phi Matrix)

- fit() 내부에서 설계행렬을 구성:

| 항 종류       | 개수            |
|--------------|-----------------|
| 상수항       | 1               |
| 선형항       | $k$           |
| 제곱항       | $k$           |
| 교호작용항   | $\frac{k(k-1)}{2}$ |

- 최종 항수:
- 이 수식은 **요인 수가 k** 일 때, 총 계수 개수는 다음과 같습니다:

$$
1+k+k+\frac{k(k-1)}{2}=1+2k+\frac{k(k-1)}{2}
$$



### ✔ Least Squares Fit (SVD 기반)

- nalgebra QR은 비정방행렬을 풀지 못해 아래 방식으로 해결:
```rust
let svd = phi.svd(true, true);
let beta = svd.solve(&yy, 1e-12);
```

- 이를 통해:
    - 직사각행렬(8×6 등) 안전하게 처리
    - 랭크 부족에도 최소노름해 계산 가능


## 4. ANOM (Analysis of Means)
- ANOM은 그룹별 평균의 차이가 유의한지를 확인하는 통계 기법으로 Rust에서 완전히 구현했다.

### ✔ 주요 단계
- 그룹별 mean 계산
- 전체 grand mean 계산
- pooled within-group variance 계산
- t-기반 또는 Bonferroni 기반 임계값 계산
- UDL / LDL 생성

$$
\mathrm{UDL}=\bar {Y}+M,\quad \mathrm{LDL}=\bar {Y}-M
$$

- 🔎 구성 요소 설명
  - $\bar {Y}$: 전체 평균 (grand mean)
  - $M$: 마진 (margin), 즉 각 그룹 평균이 전체 평균에서 벗어날 수 있는 허용 범위
  - $\mathrm{UDL}$: 상한 결정선 (Upper Decision Limit)
  - $\mathrm{LDL}$: 하한 결정선 (Lower Decision Limit)


- ✔ equal-n / unequal-n 둘 다 지원
    - equal-n: Bonferroni 기반 h 값 사용
    - unequal-n: 그룹별 tcrit × s × sqrt(1/nᵢ)

### ✔ 결과 구조체
```rust
pub struct AnomGroupResult {
    pub name: String,
    pub n: usize,
    pub mean: f64,
    pub margin: f64,
    pub UDL: f64,
    pub LDL: f64,
    pub significant_high: bool,
    pub significant_low: bool,
}
```

### ✔ SVG Chart 자동 생성
- Rust에서 SVG 파일로 직접 ANOM 차트를 출력하는 기능 존재:
```rust
let svg = anom.render_svg();
std::fs::write("anom_chart.svg", svg);
```

## 5. Full DOE Wrapper (Taguchi → RS → ANOM)

- 핵심 기능을 하나로 묶어 **버튼 한 번으로 DOE 전체 분석** 을 수행하는 래퍼:

### ✔ run_doe_full_analysis() 기능
- OA + FactorLevels 조합하여 설계점 생성
- Response data(y) 받아서 반응표면 모델 적합
- 각 Factor에 대해 ANOM 수행
- 전체 결과를 구조화된 형태로 반환

```rust
pub struct DoeFullAnalysis {
    pub rs_model: ResponseSurfaceQuadratic,
    pub factor_anoms: Vec<FactorAnomResult>,
}
```

## 6. Surface Visualization (반응표면 시각화)

- Rust에서 직접 그래프는 그리지 않지만 grid 샘플 CSV 를 내보내는 함수를 제공:
```rust
export_response_surface_csv(rs, x1_min, x1_max, x2_min, x2_max, nx, ny, "surface.csv");
```
- Python/MATLAB에서 읽어서 다음과 같이 그림을 그림:
    - 3D surface plot (plot_surface)
    - Color contour plot (contourf)
    - Response slices

7. 전체 흐름 요약

- Rust에서 DOE → 분석 까지 전체 파이프라인은 아래 형태:
```
Taguchi OA
   ↓
Design Matrix 생성
   ↓
실험 수행 또는 y 데이터 입력
   ↓
Quadratic Response Surface 회귀
   ↓
Factor별 ANOM 분석
   ↓
SVG 차트 출력 / CSV 출력
   ↓
Python/MATLAB에서 그림 시각화
```

- 이제 Rust 프로젝트 하나로 다음이 모두 가능해짐:
    - Taguchi 실험 설계 자동 생성
    - 결과값만 넣으면 RS + ANOM 자동 분석
    - 2차 반응표면 계수 자동 회귀
    - 표 형식 결과 CSV 출력
    - ANOM Chart 자동 SVG 생성
    - surface 예측 값 CSV 출력하여 3D Plot 가능


---
