## on_surface_normal 함수 설명
- 목적
- NURBS 곡면 S(u,v)에서 다음 네 가지 기하 정보를 계산한다.
- 곡면 점 P=S(u,v)
- 정규화된 첫 번째 방향 접선 벡터 $\hat {S}_u$
- 정규화된 두 번째 방향 접선 벡터 $\hat {S}_v$
- 단위 법선 벡터 $\hat {N}$
- 이 값들은 CAD/CAE에서 매우 기본적인 기하 연산이며,  
  offset surface, projection, tangent inversion, curvature 계산 등 다양한 알고리즘의 기반이 된다.

## 수식 설명
### 1. 곡면과 1차 미분
- NURBS 곡면 S(u,v)의 1차 편미분은 다음과 같다.
```math
S_u(u,v)=\frac{\partial S}{\partial u},\qquad S_v(u,v)=\frac{\partial S}{\partial v}
```
- 코드에서는:
```rust
let su0 = ders[1][0]; // S_u
let sv0 = ders[0][1]; // S_v
```


### 2. 접선 벡터 정규화
```math
\hat {S}_u=\frac{S_u}{\| S_u\| },\qquad \hat {S}_v=\frac{S_v}{\| S_v\| }
```
코드:
```rust
let su = su0.unitize();
let sv = sv0.unitize();
```
## 3. 법선 벡터 계산
- 곡면의 법선 벡터는 두 접선 벡터의 외적이다.
```math
N=S_u\times S_v
```
- 정규화된 단위 법선은:
```math
\hat {N}=\frac{N}{\| N\| }
```
- 코드:
```rust
let n0 = su.cross(&sv);
let n = n0.unitize();
```

- 법선 크기가 너무 작으면(거의 퇴화된 경우) 오류를 반환한다.

- 코드 동작 요약
- 아래는 함수가 수행하는 단계별 요약이다.
- 곡면에서 0차(점) + 1차(편미분) 도함수 계산
```rust
let ders = sur.eval_ders_nder(u, v, 1);
```
- 곡면 점 P = S(u,v) 추출
```rust
let p = Point3D::new(s.x, s.y, s.z);
```
- 편미분 벡터 Su, Sv 추출
```rust
let su0 = ders[1][0];
let sv0 = ders[0][1];
```
- 접선 벡터 정규화
```rust
let su = su0.unitize();
let sv = sv0.unitize();
```
- 법선 벡터 계산 및 정규화
```rust
let n0 = su.cross(&sv);
let n = n0.unitize();
```
- 법선 크기가 너무 작으면 오류 처리
```rust
if mag < 0.01 * m_tol { ... }
- (P, Su, Sv, N) 반환
Ok((p, su, sv, n))
```


## 문서용 최종 요약
- on_surface_normal(sur, u, v, m_tol)
- NURBS 곡면 S(u,v)에서 다음을 계산한다:
  - P=S(u,v)
  - $\hat {S}_u=\frac{S_u}{\| S_u\| }$
  - $\hat {S}_v=\frac{S_v}{\| S_v\| }$
  - $\hat {N}=\frac{S_u\times S_v}{\| S_u\times S_v\| }$
- 법선 크기가 너무 작아 퇴화된 경우 오류를 반환한다.

--- 
## 소스 코드
```rust
/// (u,v)에서 P, unit Su, unit Sv, unit N
pub fn on_surface_normal(
    sur: &NurbsSurface,
    u: Real,
    v: Real,
    m_tol: Real,
) -> Result<(Point3D, Vector3D, Vector3D, Vector3D)> 
{
    
    let ders = sur.eval_ders_nder(u, v, 1);
    if ders.len() < 2 || ders[0].len() < 2 {
        return Err(NurbsError::InvalidInput { msg :
          "on_surface_normal: eval_ders_nder returned insufficient derivative data".into() });
    }

    let s = ders[0][0];
    let p = Point3D::new(s.x, s.y, s.z);

    let su0 = ders[1][0];
    let sv0 = ders[0][1];

    let su = su0.unitize();
    let sv = sv0.unitize();

    let n0 = su.cross(&sv);
    let mag = n0.length();
    if mag < 0.01 * m_tol {
        return Err(NurbsError::InvalidInput { msg
          : "on_surface_normal: normal magnitude too small".into()});
    }
    let n = n0.unitize();
    Ok((p, su, sv, n))
}
```
---
## 테스트 코드
```rust
#[cfg(test)]
mod test {
    use nurbslib::core::geom::Vector3D;
    use nurbslib::core::nurbs_surface::{on_surface_normal, NurbsSurface};
    #[test]
    fn test_surface_normal_plane_xy() {

        let sur = NurbsSurface::from_plane_xy().unwrap();
        let (_p, _su, _sv, n) = on_surface_normal(
            &sur, 0.25, 0.75,
            1e-9
        ).unwrap();

        assert!((n - Vector3D::new(0.0, 0.0, 1.0)).length() < 1e-10);
    }
}
```
---

