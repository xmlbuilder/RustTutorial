# PointCloud
수학적으로 분석하고, 각 함수의 목적과 계산 원리를 정리한 문서 형태로 구성.  
아래는 구조체별로 기능 요약, 수학적 설명, 사용 목적을 포함한 문서입니다.

## 📘 PointCloud, Line, Circle, Plane 수학 기반 문서화

## 🧱 구조체: PointCloud
### 🔧 주요 기능: PointCloud

| 함수 이름         | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| `new()`          | 외부 점 리스트로부터 PointCloud 생성. 내부적으로 복사하여 보관         |
| `points()`       | 내부 점 리스트에 대한 참조 반환. 읽기 전용                             |
| `transform_by()` | 모든 점에 3D 변환 행렬 `Transform` 적용. 위치, 회전, 스케일 가능       |
| `fit_line()`     | 점군에 대해 최적 직선 피팅 수행. PCA 기반 주축 방향으로 선형 근사       |
| `fit_circle()`   | 점군을 평면에 투영 후 원 피팅. 중심, 반지름, 평면 정보 추출             |
| `fit_plane()`    | 점군에 대해 최소제곱 평면 피팅. 공분산 행렬의 최소 고유벡터를 법선으로 사용 |
| `get_sampling()` | 점군에서 샘플링된 점 추출. 분포 기반 또는 간격 기반 샘플링 가능         |



## 📐 Line::fit_from_points()
### 🎯 목적
점군을 가장 잘 설명하는 직선을 계산합니다. 주축 방향으로 점들을 투영하여 시작점과 끝점을 결정합니다.
### 📊 수학적 절차
- 무게중심 계산

$$
\bar {p}=\left( \frac{1}{n}\sum x_i,\  \frac{1}{n}\sum y_i,\  \frac{1}{n}\sum z_i\right) 
$$

- 산포 행렬 (Scatter Matrix)

$$
S=\sum (p_i-\bar {p})(p_i-\bar {p})^T
$$


- SVD 분해
- $S=U\Sigma V^T$
- 최대 singular value에 대응하는 eigenvector가 방향 벡터
- 투영 거리 계산

$$
t_i=(p_i-\bar {p})\cdot \vec {d}
$$

- 시작점과 끝점 계산

$$
A=\bar {p}+t_{\min }\cdot \vec {d},\quad B=\bar {p}+t_{\max }\cdot \vec {d}
$$


## 🎯 Circle::fit_from_points()
### 📊 수학적 절차
- 평면 피팅
    - Plane::fit_from_points()로 점군이 놓인 평면 추정
- 2D 투영
    - 3D 점들을 평면에 투영하여 2D 좌표로 변환
- 2D 원 피팅
    - 최소제곱 방식으로 중심과 반지름 계산
- 3D 복원
    - 2D 중심을 평면 위 3D 점으로 복원
    - 평면 법선 유지하며 원 생성

## 📐 Plane::fit_from_points()
### 🎯 목적
점군을 가장 잘 설명하는 평면을 계산합니다. 최소제곱 방식으로 공분산 행렬의 최소 고유벡터를 법선으로 사용합니다.
### 📊 수학적 절차
- 무게중심 계산

$$
\bar {p}=\left( \frac{1}{n}\sum x_i,\  \frac{1}{n}\sum y_i,\  \frac{1}{n}\sum z_i\right)
$$

- 공분산 행렬

$$
S=\sum (p_i-\bar {p})(p_i-\bar {p})^T
$$


- SVD 분해
- 최소 singular value에 대응하는 eigenvector가 평면 법선

## 📌 수식 요약표

| 항목 설명                     | 수학적 표현                                                                 |
|------------------------------|------------------------------------------------------------------------------|
| 무게중심 (Centroid)          | $\bar{p} = \frac{1}{n} \sum p_i$                                          |
| 산포 행렬 (Scatter Matrix)   | $S = \sum (p_i - \bar{p})(p_i - \bar{p})^T$                               |
| 투영 거리 (Line fitting)     | $t_i = (p_i - \bar{p}) \cdot \vec{d}$                                     |
| 평면 위 점 복원 (Circle)     | $P = \mathrm{plane.point_at}(x, y)$                                       |

이 수식들은 다음과 같은 맥락에서 사용됩니다:
- $\bar {p}$: 점군의 중심 → 직선/평면/원의 기준점
- $S$: 공분산 행렬 → SVD로 방향 벡터 또는 법선 추출
- $t_i$: 직선 방향으로 점 투영 → 시작점/끝점 결정
- $P$: 2D 원 중심을 3D 평면 위로 복원


## 📘 사용 목적 요약

| 구조체 이름   | 사용 목적 설명                                                                 |
|---------------|----------------------------------------------------------------------------------|
| `PointCloud`  | 3D 점군을 표현하고, 기하적 피팅(선, 원, 평면), 변환, 샘플링 등의 고급 처리를 지원 |
| `Line`        | 두 점 사이의 직선 표현. 방향, 길이, 보간, 접선, 투영, 피팅 등 다양한 기하 연산 수행 |
| `Circle`      | 평면상 원 표현. 중심, 반지름, 접선, 오프셋, 점 평가 및 점군 기반 피팅 기능 제공     |
| `Plane`       | 3D 공간의 평면 표현. 법선, 좌표계, 점 투영, 평면 피팅(PCA 기반) 등 공간 분석에 활용 |


---
# 테스트

아래는 주신 테스트 코드에 대한 테스트 함수 요약표와 함께 각 테스트의 목적과 기대 결과를 문서화한 내용입니다.

## 📋 PointCloud 테스트 함수 요약

| 테스트 함수 이름                         | 테스트 대상 함수       | 목적 및 설명                                                                 | 기대 결과                                                  |
|------------------------------------------|------------------------|------------------------------------------------------------------------------|-------------------------------------------------------------|
| `pointcloud_fit_line_should_work`        | `fit_line()`           | 점군을 직선으로 근사. 방향 벡터가 X축 방향인지 확인                          | `dir.x ≈ 1`, `dir.y ≈ 0`                                    |
| `pointcloud_fit_plane_should_work`       | `fit_plane()`          | XY 평면 위 점군을 평면으로 근사. 법선 벡터가 Z축 방향인지 확인              | `plane.z_axis.z ≈ 1`                                        |
| `pointcloud_fit_circle_should_work`      | `fit_circle()`         | 단위 원 형태 점군을 원으로 근사. 반지름이 1에 가까운지 확인                 | `r ≈ 1.0`                                                   |
| `pointcloud_transform_should_move_points`| `transform_by()`       | 점군에 평행 이동 변환 적용. X축으로 1만큼 이동했는지 확인                   | `p.x ≈ 2.0`                                                 |
| `pointcloud_sampling_should_return_subset`| `get_sampling()`      | 점군에서 샘플링된 점 추출. 일부 점만 반환되는지 확인                        | `0 < sampled.len() ≤ pts.len()`                             |

## 📘 테스트 목적 및 결과 요약
### ✅ 목적
- PointCloud의 주요 기능들이 정확하게 동작하는지 검증
- 기하적 피팅 결과가 수학적으로 타당한지 확인
- 변환 및 샘플링 기능이 기대대로 작동하는지 확인
### 🎯 기대 결과
- 직선 피팅 시 방향 벡터가 올바른 축을 향함
- 평면 피팅 시 법선 벡터가 Z축과 일치
- 원 피팅 시 반지름이 정확히 계산됨
- 변환 시 점의 좌표가 정확히 이동됨
- 샘플링 시 점 개수가 유효 범위 내에 있음



## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::plane::Plane;
    use nurbslib::core::point_cloud::PointCloud;
    use nurbslib::core::prelude::{Point, Vector};
    use nurbslib::core::transform::Transform;
```
### 1. pointcloud_fit_line_should_work
```rust
    #[test]
    fn pointcloud_fit_line_should_work() {
        let pts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
        ];
        let cloud = PointCloud::new(&pts);
        let mut pt = Point::origin();
        let mut dir = Vector::zero();
        cloud.fit_line(&mut pt, &mut dir);
        assert!(dir.x.abs() > 0.9 && dir.y.abs() < 1e-12);
    }
```
### 2. pointcloud_fit_plane_should_work
```rust
    #[test]
    fn pointcloud_fit_plane_should_work() {
        let pts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let cloud = PointCloud::new(&pts);
        let mut plane = Plane::xy();
        cloud.fit_plane(&mut plane);
        assert!(plane.z_axis.z.abs() > 0.9);
    }
```
### 3. pointcloud_fit_circle_should_work
```rust
    #[test]
    fn pointcloud_fit_circle_should_work() {
        let pts = vec![
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(0.0, -1.0, 0.0),
        ];
        let cloud = PointCloud::new(&pts);
        let mut plane = Plane::xy();
        let mut r = 0.0;
        cloud.fit_circle(&mut plane, &mut r);
        assert!((r - 1.0).abs() < 1e-6);
    }
```
### 4. pointcloud_transform_should_move_points
```rust
    #[test]
    fn pointcloud_transform_should_move_points() {
        let pts = vec![Point::new(1.0, 2.0, 3.0)];
        let mut cloud = PointCloud::new(&pts);
        let t = Transform::translation(1.0, 0.0, 0.0);
        cloud.transform_by(t);
        let p = cloud.points()[0];
        assert!((p.x - 2.0).abs() < 1e-12);
    }
```
### 5. pointcloud_sampling_should_return_subset
```rust
    #[test]
    fn pointcloud_sampling_should_return_subset() {
        let pts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
            Point::new(3.0, 0.0, 0.0),
        ];
        let cloud = PointCloud::new(&pts);
        let sampled = cloud.get_sampling();
        assert!(sampled.len() > 0 && sampled.len() <= pts.len());
    }
}
```

---

