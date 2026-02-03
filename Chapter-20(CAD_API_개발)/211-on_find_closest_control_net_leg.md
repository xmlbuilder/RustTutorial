# 📘 on_find_closest_control_net_leg 
- 용도 및 사용 가이드 문서
## 1. 함수 목적
- on_find_closest_control_net_leg()는 주어진 점 P가 NURBS 표면의 control net(격자)에서
    가장 가까운 **leg(선분)** 을 찾아주는 함수다.
- 여기서 leg란:
    - U 방향 leg: ⟨Pi][j], Pi+1][j]⟩
    - V 방향 leg: ⟨Pi][j], Pi][j+1]⟩
- 즉, control net을 구성하는 격자선(grid line) 중 하나다.
- 이 함수는 다음을 수행한다:
    - 모든 U-leg을 검사
    - 모든 V-leg을 검사
    - 각 leg에 대해 점 P를 선분에 투영
    - 투영점이 선분 내부에 있을 때만 후보로 인정
    - 그중 가장 가까운 leg을 선택
    - 투영점 Q, leg의 시작 index (i,j), 방향(UDir/VDir), parameter t ∈ [0,1] 반환

## 2. 왜 필요한가?
- 이 함수는 CAD/NURBS 커널에서 다음과 같은 상황에서 매우 중요하다.
### ✔ 2.1. 표면 위의 점을 control net 기반 으로 근사할 때
- 예:
    - Newton iteration 초기값
    - closest point on surface (CPS) 알고리즘의 coarse 단계
    - surface parameterization 초기 guess
    - trimming curve projection의 초기 anchor
- control net은 표면의 coarse shape을 나타내므로,  
    control net leg에 대한 최근접점은 표면의 실제 최근접점의 좋은 초기값이 된다.

### ✔ 2.2. control net 기반 feature detection
- ridge/valley 근사
- iso-curve 추정
- control polygon 기반 curvature 분석
- mesh-like 분석

### ✔ 2.3. 디버깅 및 시각화
- **점이 어느 control leg 근처에 있는지** 를 빠르게 확인
- surface fitting 과정에서 residual 분석
- control net과 입력 데이터의 관계 파악

## 3. 반환값 의미
- 함수는 다음을 반환한다:
```rust
Ok(Some((Q, i, j, dir, t)))
```

각 항목의 의미는 다음과 같다:
| 항목 | 의미 |
|------|------|
| Q    | 점 P를 해당 control net leg에 투영한 실제 3D 점 |
| i    | leg의 시작 control point의 u-index |
| j    | leg의 시작 control point의 v-index |
| dir  | UDir 또는 VDir (어느 방향의 leg인지) |
| t    | 선분 파라미터 값 (0 = 시작점, 1 = 끝점) |


- 즉, leg은 다음과 같이 표현된다:
    - UDir: ⟨Pi][j], Pi+1][j]⟩
    - VDir: ⟨Pi][j], Pi][j+1]⟩
- 그리고 Q는:
    - Q=(1-t)A+tB

## 4. 내부 알고리즘 요약
### ✔ 4.1. 모든 U-leg 검사
- k = 0..n-1
- l = 0..m
- leg = ⟨Pk][l], Pk+1][l]⟩
- 점 P를 선분에 투영
- 투영점이 선분 내부일 때만 후보로 인정
- 거리 최소값 갱신
### ✔ 4.2. 모든 V-leg 검사
- k = 0..n
- l = 0..m-1
- leg = ⟨Pk][l], Pk][l+1]⟩
- 동일한 방식으로 검사
### ✔ 4.3. U-leg vs V-leg 중 더 가까운 것 선택

## 5. 언제 쓰면 좋은가?
### ✔ 5.1. CPS(closest point on surface) 초기값
- 표면에서 점 P의 최근접점을 찾는 알고리즘은 보통 Newton iteration을 사용한다.
- 초기값이 나쁘면:
    - 발산
    - 잘못된 local minimum
    - parameter domain 밖으로 튀기
- 이 함수는 control net 기반의 robust한 초기값을 제공한다.

### ✔ 5.2. surface fitting / reverse engineering
- 입력 점이 어떤 control net leg 근처에 있는지 알면:
    - control point 이동 방향
    - knot insertion 위치
    - local refinement 위치
- 등을 결정할 수 있다.

### ✔ 5.3. trimming curve projection
- 곡선을 표면 위로 투영할 때:
    - 먼저 control net에서 가장 가까운 leg을 찾고
    - 그 leg의 parameter를 기반으로
    - surface parameter (u,v)의 초기 guess를 만든다

### ✔ 5.4. 디버깅
- 표면이 이상하게 보일 때:
    - 어떤 control leg이 문제인지
    - 입력 점이 어느 영역에 몰려 있는지
    - control net과 데이터의 관계
- 이걸 빠르게 파악할 수 있다.

## 6. 장점
- robust: 모든 leg을 검사하므로 놓치는 경우가 없다
- bounded projection: 선분 내부에만 투영하므로 잘못된 후보 제거
- surface-independent: NURBS surface evaluator 없이도 동작
- 빠름: control net은 보통 매우 coarse하므로 leg 수가 적다

## 7. 단점 / 주의사항
- control net이 매우 dense하면 O(n·m) 스캔 비용이 커질 수 있다
- 표면의 실제 geometry와 control net이 크게 다르면 초기값 품질이 떨어질 수 있다
- rational 여부와 무관하게 Euclidean net에서만 동작한다  
    (즉, homogeneous 좌표 기반의 정확한 geometry는 반영되지 않음)

## 8. 예시 사용 코드
```rust
let net = EuclideanNet { n_idx: 3, m_idx: 2, p: ctrl_points };
let p = Point3D::new(1.2, 0.5, 0.8);

if let Some((q, i, j, dir, t)) = on_find_closest_control_net_leg(&net, p)? {
    println!("Closest leg starts at ({}, {})", i, j);
    println!("Direction: {:?}", dir);
    println!("Parameter t = {}", t);
    println!("Projected point Q = {:?}", q);
} else {
    println!("Point is outside all control net legs");
}
```


## ⭐ 최종 요약
- on_find_closest_control_net_leg()는:
    - control net의 모든 선분(leg)에 대해 점 P의 투영을 검사하고
    - 투영점이 선분 내부에 있는 leg만 후보로 삼아
    - 가장 가까운 leg과 그 위의 파라미터 t를 반환하는 함수
- 즉:
- NURBS surface에서 점 P의 parameter 초기값을 찾기 위한  
    가장 기본적이고 중요한 geometric primitive
---
## 소스 코드
```rust

/// Return:
/// - Ok(Some(...)) : found a leg where projection falls inside at least one segment
/// - Ok(None)      : projection is outside of every leg (C: *flg = FALSE)
/// Output tuple:
/// (Q, i, j, dir, t)
/// - dir=UDir => the closest leg is <P[i][j], P[i+1][j]>, with parameter t in [0,1]
/// - dir=VDir => the closest leg is <P[i][j], P[i][j+1]>, with parameter t in [0,1]
pub fn on_find_closest_control_net_leg(
    net: &EuclideanNet,
    p: Point3D,
) -> Result<Option<(Point3D, usize, usize, SurfaceDir, Real)>, NurbsError> {
    // C의 BIGD
    let bigd: Real = 1.0e300;

    let n = net.n_idx;
    let m = net.m_idx;

    // ---- Scan U direction legs: (k,l) -> (k+1,l) ----
    let mut best_u_dist = bigd;
    let mut best_u_i = 0usize;
    let mut best_u_j = 0usize;
    let mut best_u_t = 0.0;

    // note: u legs exist for k=0..n-1 and l=0..m
    for l in 0..=m {
        for k in 0..n {
            let a = net.at(k, l);
            let b = net.at(k + 1, l);

            // BOUNDED segment, domain [0,1]
            let line = Line {
                dimension: 3,
                start: a,
                end: b,
                domain: Interval { t0: 0.0, t1: 1.0 },
            };

            // infinite=false => segment projection only (outside => inside=false)
            let (s, t, inside) = on_project_point_onto_line_via_plane(&line, p, false)?;

            if !inside {
                continue;
            }

            let d = p.distance(&s);
            if d < best_u_dist {
                best_u_dist = d;
                best_u_i = k;
                best_u_j = l;
                best_u_t = t;
            }
        }
    }

    // ---- Scan V direction legs: (k,l) -> (k,l+1) ----
    let mut best_v_dist = bigd;
    let mut best_v_i = 0usize;
    let mut best_v_j = 0usize;
    let mut best_v_t = 0.0;

    // note: v legs exist for k=0..n and l=0..m-1
    for k in 0..=n {
        for l in 0..m {
            let a = net.at(k, l);
            let b = net.at(k, l + 1);

            let line = Line {
                dimension: 3,
                start: a,
                end: b,
                domain: Interval { t0: 0.0, t1: 1.0 },
            };

            let (s, t, inside) = on_project_point_onto_line_via_plane(&line, p, false)?;
            if !inside {
                continue;
            }

            let d = p.distance(&s);
            if d < best_v_dist {
                best_v_dist = d;
                best_v_i = k;
                best_v_j = l;
                best_v_t = t;
            }
        }
    }

    // ---- Pick closest leg ----
    let found_u = best_u_dist < bigd;
    let found_v = best_v_dist < bigd;

    if !found_u && !found_v {
        // C: *flg = FALSE
        return Ok(None);
    }

    if found_u && (!found_v || best_u_dist < best_v_dist) {
        // dir = UDIR, leg <P[i][j], P[i+1][j]>
        let a = net.at(best_u_i, best_u_j);
        let b = net.at(best_u_i + 1, best_u_j);

        // C는 A_compoi(1-t, A, t, B)로 Q를 다시 만든다.
        // (project 함수가 Q를 줘도, 여기서 동일하게 유지)
        let q = on_combination_point3d(1.0 - best_u_t, a, best_u_t, b);

        return Ok(Some((q, best_u_i, best_u_j, SurfaceDir::UDir, best_u_t)));
    } else {
        // dir = VDIR, leg <P[i][j], P[i][j+1]>
        let a = net.at(best_v_i, best_v_j);
        let b = net.at(best_v_i, best_v_j + 1);

        let q = on_combination_point3d(1.0 - best_v_t, a, best_v_t, b);

        return Ok(Some((q, best_v_i, best_v_j, SurfaceDir::VDir, best_v_t)));
    }
}
```
---
