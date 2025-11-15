# fit_geom_tests

아래는 fit_geom_tests 테스트 함수들과 관련된 평면, 직선, 원 적합 알고리즘을 수학적으로 정리한 문서입니다.  
각 함수가 어떤 수학적 원리에 기반하는지, 단계별로 어떤 계산을 수행하는지를 명확하게 설명합니다.

## 📐 1. 평면 적합: Plane::fit_from_points
- 목적
    - 3D 점군으로부터 최소제곱 방식으로 평면을 적합.
    - 법선 벡터는 산포 행렬의 최소 고유벡터(PCA 방식).

### 📐 기하 객체 적합 관련 수식 요약

| 단계 | 수식 표현                                                                 | 설명 |
|------|----------------------------------------------------------------------------|------|
| ① 중심점 계산 | $\mathbf{c} = \frac{1}{n} \sum_{i=1}^{n} \mathbf{p}_i$             | 점군의 평균값 (centroid) |
| ② 산포 행렬 구성  <br> 공분산 기반 산포 행렬 |                                        |
| ③ SVD 분해 | $S = U \Sigma V^T$                                                  | 고유벡터 추출 (PCA 방식) |
| ④ 법선 벡터 | $\vec{n} = \mathrm{eigenvector}_{\min}$                              | 최소 고유값에 대응하는 고유벡터 |
| ⑤ 평면 생성 | $\mathrm{Plane}(\mathbf{c}, \vec{n})$   

#### 산포 행렬 구성

$$
S = \sum_{i=1}^{n} (\mathbf{p}_i - \mathbf{c})(\mathbf{p}_i - \mathbf{c})^T \quad \text{(3×3 대칭 행렬)}
$$


$\mathbf{u}i = \mathrm{project\{plane}}(\mathbf{p}_i)$

#### 공분산 기반 산포 행렬   
- ③ SVD 분해  $S=U\Sigma V^T$  최소 고유값에 대응하는 고유벡터 선택   
- ④ 법선 벡터  $\vec {n}=\mathrm{eigenvector_{\mathnormal{\min }}}$  평면의 법선 벡터   
- ⑤ 평면 생성  $\mathrm{Plane}(\mathbf{c},\vec {n})$  중심과 법선으로 평면 정의 

### 수학적 검토 ✅
- PCA 기반으로 정확한 평면 추정.
- SVD 안정성 확보.
- 테스트에서 평균 거리 < 1e-10 → 수치적 정합성 우수.

## 📏 2. 직선 적합: Line::fit_from_points
- 목적
    - 3D 점군으로부터 최소자승 방식으로 직선 적합.
    - 주성분 방향을 직선 방향으로 사용.

### 단계별 수식 정리

| 단계 | 수식 표현                                                                 | 설명 |
|------|----------------------------------------------------------------------------|------|
| 중심점 계산 | $\mathbf{c} = \frac{1}{n} \sum_{i=1}^{n} \mathbf{p}_i$             | 점군의 평균값 (centroid) |
| 산포 행렬 구성 <br> 공분산 기반 산포 행렬  |                                                                   |                                                                   
| SVD 분해 | $S = U \Sigma V^T$                                                  | 고유벡터 추출 (PCA 방식) |
| 직선 방향 벡터 | $\vec{d} = \mathrm{eigenvector}_{\max}$                          | 가장 큰 고유값에 대응하는 방향 |
| 점 투영값 계산 | $t_i = (\mathbf{p}_i - \mathbf{c}) \cdot \vec{d}$                 | 각 점을 직선 방향으로 투영한 스칼라 값 |
| 직선 끝점 계산 <br> 투영값 범위로부터 직선 양 끝점 계산 |                                                                    |

### 산포 행렬 구성

$$
S = \sum_{i=1}^{n} (\mathbf{p}_i - \mathbf{c})(\mathbf{p}_i - \mathbf{c})^T \quad \text{(3×3 대칭 행렬)}
$$

### 직선 끝점 계산
$$
\mathbf{start} = \mathbf{c} + t_{\min} \cdot \vec{d} \\ \quad \mathbf{end} = \mathbf{c} + t_{\max} \cdot \vec{d}
$$

- 직선 양 끝점 정의 

### 수학적 검토 ✅
- PCA 기반 직선 추정.
- 방향 벡터 정합도 > 0.999999 → 오차 < 1e-6 라디안.
- 노이즈 허용 범위 내에서 안정적.

## 🎯 3. 원 적합: Circle::fit_from_points
- 목적
    - 3D 점군으로부터 원 적합.
    - 평면 적합 → 2D 투영 → 원 적합 → 3D 복원.
### 단계별 수식 정리
| 단계 | 수식 표현                                                                 | 설명 |
|------|----------------------------------------------------------------------------|------|
| ① 중심점 계산 | $\mathbf{c} = \frac{1}{n} \sum_{i=1}^{n} \mathbf{p}_i$             | 점군의 평균값 (centroid) |
| ② 산포 행렬 구성 <br>  PCA 기반 평면 적합을 위한 산포 행렬 |                                                     |
| ③ 평면 법선 추출 | $\vec{n} = \mathrm{eigenvector}_{\min}(S)$                        | 최소 고유값에 대응하는 고유벡터 = 평면 법선 |
| ④ 평면 정의 | $\text{Plane}(\mathbf{c}, \vec{n})$                                 | 중심과 법선으로 평면 정의 |
| ⑤ 2D 투영 | $\mathbf{u}_i = \mathrm{project\\_{plane}}(\mathbf{p}_i)$               | 3D 점을 평면 좌표계로 투영 (2D 점 생성) |
| ⑥ 2D 원 적합 (Kåsa 방식) 2D 원 중심 <br> $(a, b)$ 및 반지름 $r$ 계산 |                                                |
| ⑦ 3D 중심 복원 | $\mathbf{c}_{3D} = \text{plane.point\\_at}(a, b)$                     | 2D 중심을 평면 위 3D 점으로 복원 |
| ⑧ 평면 재정의 | $\text{Plane}(\mathbf{c}_{3D}, \vec{n})$                             | 중심 기준으로 평면 재정의 |
| ⑨ 원 생성 | $\text{Circle}(\text{plane}, r)$                                         |                       |                                  

#### ② 산포 행렬 구성 

$$
S = \sum_{i=1}^{n} (\mathbf{p}_i - \mathbf{c})(\mathbf{p}_i - \mathbf{c})^T \quad \text{(3×3 대칭 행렬)}
$$

#### 2D 원 적합 (Kåsa 방식)

$$
\left[ \begin{matrix}S_{xx}&S_{xy}\\ S_{xy}&S_{yy}\end{matrix}\right] \left[ \begin{matrix}a\\ b\end{matrix}\right] =\frac{1}{2}\left[ \begin{matrix}S_{x^3}+S_{xy^2}\\ S_{x^2y}+S_{y^3}\end{matrix}\right]
$$

#### 중심 및 반지름 계산 
 
- ④ 3D 복원  $\mathbf{c_{\mathnormal{3D}}}=\mathrm{plane.point_at}(a,b)$  2D 중심을 3D로 복원   
- ⑤ 평면 재정의  $\mathrm{Plane}(\mathbf{c_{\mathnormal{3D}}},\vec {n})$  중심 기준으로 평면 재정의   
- ⑥ 원 생성  $\mathrm{Circle}(\mathrm{plane},r)$  최종 원 생성 

### 수학적 검토 ✅
- 평면 법선 정합도 > 0.999999
- 중심 거리 오차 < 1e-6
- 반지름 오차 < 1e-6 → 고정밀 적합


## 📊 기하 객체 적합 알고리즘 요약

| 대상     | 알고리즘 방식 | 핵심 수식 또는 원리                     | 수학적 정합성     |
|----------|----------------|------------------------------------------|--------------------|
| Plane    | PCA            | 최소 고유벡터 = 법선                    | ✅ 매우 정확함     |
| Line     | PCA            | 최대 고유벡터 = 방향                    | ✅ 매우 정확함     |
| Circle   | 평면 + Kåsa    | 2D 원 적합 후 3D 복원                   | ✅ 매우 정확함     |

---


