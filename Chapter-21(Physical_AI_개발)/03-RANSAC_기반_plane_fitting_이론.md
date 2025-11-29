# RANSAC 기반 plane fitting 이론
RANSAC(Random Sample Consensus)는 outlier가 많은 데이터에서 신뢰할 수 있는 모델을 찾기 위한 반복적 추정 알고리즘입니다.   
평면 피팅에서는 포인트 클라우드에 섞여 있는 잡음·이상치(outlier) 속에서도 **가장 많은 점이 지지하는 하나의 평면** 을 찾아냅니다.
- 모델: 평면 방정식은  $ax+by+cz+d=0$  입니다.   
  여기서 $\mathbf{n}=(a,b,c)$ 는 단위 법선 벡터이고, d는 오프셋입니다.
- 핵심 아이디어:
- 데이터에서 최소 샘플 크기 s만큼 무작위로 뽑아(평면은 3점) 후보 평면을 추정합니다.
- 전체 점들에 대해 후보 평면까지의 거리(점–평면 거리)를 계산하고, 임계값 $\tau$  이하인 점들을 inlier로 분류합니다.
- inlier 개수가 최대인 후보를 유지하며 반복합니다.
- 최종적으로 **최대 inlier 집합** 으로 평면을 재추정(최소제곱/PCA)하면 더 안정적인 결과를 얻습니다.
- 점–평면 거리: 점 $\mathbf{p}=(x,y,z)$, 법선 $\mathbf{n}$ 이 단위벡터일 때

$$
d(\mathbf{p},\pi ) = \left|  \mathbf{n}\cdot \mathbf{p}+d \right|
$$

- 법선이 단위가 아니라면 분모로 $\| \mathbf{n}\|$ 를 나눠 정규화합니다.
- 반복 횟수 추정: 전체 중 inlier 비율을 w, 최소 샘플 크기를 s라고 하면, 신뢰도 p로 **적어도 한 번은 전부 inlier인 샘플을 뽑을**
  - 반복 횟수 N은

$$
N = \frac{\log (1-p)}{\log \left( 1-w^s\right) }
$$

- 보통 $p\in [0.95,0.99]$ 로 설정하고, w는 대략 추정값에서 시작해 반복 중 동적으로 갱신합니다.
- 파라미터 선택 가이드:
  - 거리 임계값 $\tau$ : 센서 노이즈(라이다 정확도)와 포인트 밀도를 고려해 설정합니다. 예: 수 cm–수십 cm.
  - 최소 inlier 수: 너무 적은 inlier로 결정하지 않도록 하한선을 둡니다(예: 전체의 5–20%).
  - 최대 반복: 실시간/오프라인 처리에 맞춰 상한(예: 500–5000)을 설정합니다.

## RANSAC plane fitting Rust 구현
아래 구현은 PointCloud 구조를 그대로 활용하는 형태입니다. 핵심 흐름은:
- 3점 샘플로 후보 평면 생성
- 전체에 대해 거리 기준으로 inlier 집합 계산
- 베스트 inlier 집합 유지
- 최종 inlier로 평면 재추정(PCA 기반 법선 + centroid로 오프셋)

```rust
/// 평면 하나를 RANSAC으로 추정
pub struct RansacPlaneParams {
    pub distance_threshold: f64, // τ: 점-평면 거리 임계값
    pub max_iterations: usize,   // 최대 반복
    pub min_inliers: usize,      // 허용 최소 inlier 수
}
```
```rust
pub struct RansacPlaneResult {
    pub plane: Option<Plane>,
    pub inlier_indices: Vec<usize>,
    pub iterations: usize,
}
```
```rust
/// 유틸: 세 점으로부터 평면 후보를 만든다 (비공선 체크 포함)
fn plane_from_three_points(a: &Point3D, b: &Point3D, c: &Point3D) -> Option<Plane> {
    // 벡터
    let ab = Vector3D::new(b.x - a.x, b.y - a.y, b.z - a.z);
    let ac = Vector3D::new(c.x - a.x, c.y - a.y, c.z - a.z);
    // 법선 = ab × ac
    let nx = ab.y * ac.z - ab.z * ac.y;
    let ny = ab.z * ac.x - ab.x * ac.z;
    let nz = ab.x * ac.y - ab.y * ac.x;

    let norm = (nx*nx + ny*ny + nz*nz).sqrt();
    if norm < 1e-12 {
        return None; // 거의 공선/퇴화
    }
    let n = Vector3D::new(nx / norm, ny / norm, nz / norm);
    // 오프셋 d: n·p0 + d = 0 → d = - n·a
    let d = -(n.x * a.x + n.y * a.y + n.z * a.z);

    // Plane 구성 (Plane 필드 정의에 맞게 세팅)
    let mut plane = Plane::default(); // Plane에 Default가 없다면 직접 생성자 사용
    plane.origin = a.clone();
    plane.z_axis = n.clone();                 // 법선을 z_axis로 두고
    // 임의의 x_axis, y_axis를 직교로 구성(간단한 그래머-슈미트)
    let tmp = if n.x.abs() < 0.9 { Vector3D::new(1.0, 0.0, 0.0) } else { Vector3D::new(0.0, 1.0, 0.0) };
    let x = cross(&tmp, &n).normalize();
    let y = cross(&n, &x).normalize();
    plane.x_axis = x;
    plane.y_axis = y;
    plane.equation = (n.x, n.y, n.z, d);
    Some(plane)
}
```
```rust
/// 벡터 외적/정규화 유틸
fn cross(a: &Vector3D, b: &Vector3D) -> Vector3D {
    Vector3D::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    )
}
```
```rust
trait Normalize {
    fn normalize(self) -> Vector3D;
}
```
```rust
impl Normalize for Vector3D {
    fn normalize(self) -> Vector3D {
        let n = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        if n < 1e-12 { self } else { Vector3D::new(self.x / n, self.y / n, self.z / n) }
    }
}
```
```rust
/// 점-평면 거리 (법선이 단위벡터일 때)
fn point_plane_distance(p: &Point3D, plane: &Plane) -> f64 {
    let (a, b, c, d) = plane.equation;
    (a * p.x + b * p.y + c * p.z + d).abs() // plane.z_axis를 단위로 유지
}
```
```rust
/// PCA 기반 재추정: inlier 점들로 centroid + 공분산 행렬의 최소 고유벡터를 법선으로
fn refit_plane_pca(points: &[Point3D]) -> Option<Plane> {
    if points.len() < 3 { return None; }

    // centroid
    let mut cx = 0.0; let mut cy = 0.0; let mut cz = 0.0;
    let n = points.len() as f64;
    for p in points { cx += p.x; cy += p.y; cz += p.z; }
    cx /= n; cy /= n; cz /= n;
    let c = Point3D::new(cx, cy, cz);

    // 공분산 3x3 (행렬 원소를 직접 계산)
    let mut sxx = 0.0; let mut sxy = 0.0; let mut sxz = 0.0;
    let mut syy = 0.0; let mut syz = 0.0; let mut szz = 0.0;
    for p in points {
        let x = p.x - cx;
        let y = p.y - cy;
        let z = p.z - cz;
        sxx += x * x; sxy += x * y; sxz += x * z;
        syy += y * y; syz += y * z; szz += z * z;
    }
    // 평균 공분산
    sxx /= n; sxy /= n; sxz /= n;
    syy /= n; syz /= n; szz /= n;

    // 최소 고유벡터 구하기(간단한 power iteration으로 최대 고유벡터를 찾고,
    // Gram-Schmidt로 직교 벡터 중 하나를 법선 후보로 쓰거나,
    // 혹은 자코비법/특정 라이브러리를 쓰는 게 정확하지만 여기선 간단화)
    // 여기서는 SVD/고유분해 없이 "평면 법선 ≈ 공분산 행렬의 최소 고유벡터"를
    // 근사하기 위해, 최대 고유벡터를 구한 뒤 임의 벡터와의 외적으로 직교 벡터를 만들고
    // 다시 잔차를 줄이는 간단한 휴리스틱을 사용합니다.

    // power iteration for max eigenvector
    let mut v = Vector3D::new(1.0, 0.3, 0.2);
    for _ in 0..30 {
        let x = sxx * v.x + sxy * v.y + sxz * v.z;
        let y = sxy * v.x + syy * v.y + syz * v.z;
        let z = sxz * v.x + syz * v.y + szz * v.z;
        v = Vector3D::new(x, y, z).normalize();
    }
    // v는 최대 고유벡터(데이터의 주방향) 근사.
    // 임의의 w와 v에 직교한 u를 얻고, u를 공분산에 투영-정규화하여 법선 후보로 사용.
    let w = if v.x.abs() < 0.9 { Vector3D::new(1.0, 0.0, 0.0) } else { Vector3D::new(0.0, 1.0, 0.0) };
    let u0 = cross(&v, &w).normalize();
    let mut u = u0;
    for _ in 0..10 {
        let x = sxx * u.x + sxy * u.y + sxz * u.z;
        let y = sxy * u.x + syy * u.y + syz * u.z;
        let z = sxz * u.x + syz * u.y + szz * u.z;
        u = Vector3D::new(x, y, z);
        // v와 직교 성분만 남기기
        let dot_v = u.x * v.x + u.y * v.y + u.z * v.z;
        u = Vector3D::new(u.x - dot_v * v.x, u.y - dot_v * v.y, u.z - dot_v * v.z).normalize();
    }
    // u는 v에 직교하는 주성분으로, 남은 한 축이 법선에 해당.
    // 마지막 축 n = v × u
    let nvec = cross(&v, &u).normalize();
    let d = -(nvec.x * cx + nvec.y * cy + nvec.z * cz);

    let mut plane = Plane::default();
    plane.origin = c;
    plane.z_axis = nvec.clone();
    plane.x_axis = u;
    plane.y_axis = cross(&nvec, &plane.x_axis).normalize();
    plane.equation = (nvec.x, nvec.y, nvec.z, d);
    Some(plane)
}
```
```rust
/// 메인 RANSAC
impl PointCloud {
    pub fn ransac_fit_plane(&self, params: RansacPlaneParams) -> RansacPlaneResult {
        use rand::{thread_rng, Rng};

        let n = self.len();
        if n < 3 {
            return RansacPlaneResult { plane: None, inlier_indices: vec![], iterations: 0 };
        }
        let pts = self.points();

        let mut rng = thread_rng();
        let mut best_inliers: Vec<usize> = Vec::new();
        let mut best_plane: Option<Plane> = None;

        let mut iterations = 0;
        for it in 0..params.max_iterations {
            iterations = it + 1;
            // 3점 랜덤 샘플(중복 없이)
            let i1 = rng.gen_range(0..n);
            let mut i2 = rng.gen_range(0..n);
            while i2 == i1 { i2 = rng.gen_range(0..n); }
            let mut i3 = rng.gen_range(0..n);
            while i3 == i1 || i3 == i2 { i3 = rng.gen_range(0..n); }

            let a = &pts[i1];
            let b = &pts[i2];
            let c = &pts[i3];

            // 후보 평면
            let Some(candidate) = plane_from_three_points(a, b, c) else { continue; };

            // inlier 계산
            let mut inliers: Vec<usize> = Vec::new();
            for (idx, p) in pts.iter().enumerate() {
                let dist = point_plane_distance(p, &candidate);
                if dist <= params.distance_threshold {
                    inliers.push(idx);
                }
            }

            // 베스트 갱신
            if inliers.len() > best_inliers.len() {
                best_inliers = inliers;
                best_plane = Some(candidate);
                // 조기 종료: 매우 좋은 inlier 집합이면 종료(옵션)
                if best_inliers.len() >= params.min_inliers && best_inliers.len() > n * 3 / 4 {
                    break;
                }
            }
        }

        // 충분한 inlier가 없다면 실패 처리
        if best_inliers.len() < params.min_inliers {
            return RansacPlaneResult { plane: None, inlier_indices: best_inliers, iterations };
        }

        // 최종 재추정: best_inliers로 PCA 기반 정교화
        let inlier_points: Vec<Point3D> = best_inliers.iter().map(|&i| pts[i]).collect();
        let refined = refit_plane_pca(&inlier_points).or(best_plane);

        RansacPlaneResult {
            plane: refined,
            inlier_indices: best_inliers,
            iterations,
        }
    }
}
```
```rust

## 사용 예
```rust
let pc = PointCloud::from_vec(my_points);
let params = RansacPlaneParams {
    distance_threshold: 0.05, // 5cm (센서/현장에 맞게 조정)
    max_iterations: 2000,
    min_inliers: 500,
};
let res = pc.ransac_fit_plane(params);

match res.plane {
    Some(pln) => {
        println!("RANSAC plane found with {} inliers in {} iters", res.inlier_indices.len(), res.iterations);
        // 필요하면 해당 inlier만 남기거나, outlier 제거
        // let mut pc2 = pc.clone();
        // pc2.remove_indices(&outliers);
    }
    None => {
        println!("Plane not found. Inliers: {}, iterations: {}", res.inlier_indices.len(), res.iterations);
    }
}
```

## 튜닝 팁
- distance_threshold: 포인트 밀도와 센서 정확도를 기준으로 탐색하세요. 라이다는 수 cm–수십 cm가 일반적입니다.
- max_iterations: inlier 비율이 낮을수록 반복을 늘려야 합니다. 실시간이면 상한을 제한하고, 오프라인이면 신뢰도 p를 높게 잡아 더 돌립니다.
- min_inliers: 의미 있는 평면만 선택되도록 하한선을 둡니다(장면 규모에 따라 상대값/절대값 병행).

---

## 테스트 코드
```rust
#[cfg(test)]
mod tests_ransac {
    use rand::thread_rng;
    use rand::Rng;
    use nurbslib::core::geom::Vector3D;
    use nurbslib::core::point_cloud::{PointCloud, RansacPlaneParams};
    use nurbslib::core::prelude::Point3D;

    #[test]
    fn test_ransac_plane_fitting() {
        let mut rng = thread_rng();
        let mut points = Vec::new();

        // 평면: z = 0.5x + 0.2y + 1.0
        for _ in 0..500 {
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(-10.0..10.0);
            let z = 0.5 * x + 0.2 * y + 1.0 + rng.gen_range(-0.01..0.01); // 약간의 노이즈
            points.push(Point3D::new(x, y, z));
        }

        // 잡음 추가
        for _ in 0..100 {
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(-10.0..10.0);
            let z = rng.gen_range(-10.0..10.0); // 완전 랜덤
            points.push(Point3D::new(x, y, z));
        }

        let pc = PointCloud::from_vec(points);
        let params = RansacPlaneParams {
            distance_threshold: 0.05,
            max_iterations: 1000,
            min_inliers: 400,
        };

        let result = pc.ransac_fit_plane(params);
        assert!(result.plane.is_some(), "평면 추정 실패");

        let plane = result.plane.unwrap();
        let (a, b, c, d) = plane.equation.to_tuple();

        println!("추정된 평면 방정식: {:.3}x + {:.3}y + {:.3}z + {:.3} = 0", a, b, c, d);
        println!("inlier 개수: {}", result.inlier_indices.len());
        println!("반복 횟수: {}", result.iterations);

        // 법선 벡터가 원래 (0.5, 0.2, -1.0)과 유사한지 확인
        let norm = Vector3D::new(a, b, c).unitize();
        let expected = Vector3D::new(0.5, 0.2, -1.0).unitize();
        let dot = norm.x * expected.x + norm.y * expected.y + norm.z * expected.z;
        assert!(dot.abs() > 0.99, "법선 벡터가 예상과 다름");
    }
}
```

