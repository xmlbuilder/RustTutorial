## ✅ 1. Rational NURBS 도함수 공식과 비교 (Piegl & Tiller 2nd Ed. Section 3.6)
- NURBS surface는 동차 좌표로 먼저 계산하고:
```math
S=\frac{S_w}{W}
```

```math
S_u=\frac{S_{w,u}W-S_wW_u}{W^2}
```
```math
S_v=\frac{S_{w,v}W-S_wW_v}{W^2}
```
- 2차 미분은:
```math
S_{uu}=\frac{S_{w,uu}W^2-2S_{w,u}WW_u-S_wW_{uu}W+2S_wW_u^2}{W^3}
```
```math
S_{vv}=\frac{S_{w,vv}W^2-2S_{w,v}WW_v-S_wW_{vv}W+2S_wW_v^2}{W^3}
```
```math
S_{uv}=\frac{S_{w,uv}W^2-S_{w,u}WW_v-S_{w,v}WW_u-S_wW_{uv}W+2S_wW_uW_v}{W^3}
```
- 코드가 계산하는 식을 보면:
- ✔ 1차 도함수
  ```rust
  ders[1][0] = (s_u * w00 - s * w_u) / w2;
  ders[0][1] = (s_v * w00 - s * w_v) / w2;
  ```
  - 정확히 공식과 동일

- ✔ 2차 도함수
  ```rust
  ders[2][0] = (s_uu * w2 - (s_u * (2.0 * w00 * w_u)) - (s * (w_uu * w00))
                  + (s * (2.0 * w_u * w_u))) / w3;
  ```
  - Piegl 공식과 완전히 동일

  ```rust
  ders[1][1] = (s_uv * w2 - (s_u * (w00 * w_v)) - (s_v * (w00 * w_u))
                  - (s * (w_uv * w00)) + (s * (2.0 * w_u * w_v))) / w3;
  ```
  - Piegl 공식과 동일

  ```rust
  ders[0][2] = (s_vv * w2 - (s_v * (2.0 * w00 * w_v)) - (s * (w_vv * w00))
                  + (s * (2.0 * w_v * w_v))) / w3;
  ```
  
  - 역시 동일

## 🎯 결론: Rational 처리 100% 정석 구현
- 동차 좌표 Sw(du,dv) 계산 OK
- weight W(du,dv) 계산 OK
- 몫의 미분 공식 0·1·2차 모두 정확
- Piegl & Tiller 공식과 완벽히 일치
- singular weight fallback도 안전하게 처리
- 즉,
    - 이 함수는 rational NURBS surface의 analytic derivative를 정확하게 계산하고 있다.


## 🎯 Control Point 주의 사항
- Control Point가 이미 (wx, wy, wz, w):
- $\mathbf{P_{\mathnormal{w}}}=(wx,wy,wz)$
- $w=w$
- 그럼 동차 좌표 합은:
```math
\mathbf{S_{\mathnormal{w}}^{\mathnormal{(du,dv)}}}=\sum N_i^{(du)}(u)M_j^{(dv)}(v)\cdot \mathbf{P_{\mathnormal{w,ij}}}
```
```math
W^{(du,dv)}=\sum N_i^{(du)}(u)M_j^{(dv)}(v)\cdot w_{ij}
```
- 즉, 코드로는 이렇게 되어야 맞다:
```rust
sw[du][dv] += pw * n_uv;        // 이미 pw = (w x, w y, w z)
w[du][dv]  += w_ij * n_uv;      // w_ij = w
```
- 여기서 절대 w_ij를 pw에 다시 곱하면 안 된다.


## 🎉 최종 결론
- ✔ Rational NURBS analytic derivative 계산
- ✔ 0·1·2차 모두 정확
- ✔ Piegl & Tiller 공식과 100% 일치
- ✔ 실전 CAD 커널 수준의 구현
---

## 소스 코드
```rust

impl NurbsSurface {
    /// 최대 2차까지 analytic 도함수 평가
    /// ders[du][dv]:
    ///   - [0][0] = S
    ///   - [1][0] = S_u, [0][1] = S_v
    ///   - [2][0] = S_uu, [1][1] = S_uv, [0][2] = S_vv
    /// 라셔널인 경우 몫의 미분 공식(1차/2차)을 적용.
    pub fn eval_ders_nder(&self, u: Real, v: Real, n_der: usize) -> Vec<Vec<Vector3D>> {
        let pu = self.pu as usize;
        let pv = self.pv as usize;

        let nu_pts = self.nu as usize;
        let nv_pts = self.nv as usize;
        if nu_pts == 0 || nv_pts == 0 {
            // 빈 출력 (0차 한 개만)
            return vec![vec![Vector3D::zero(); 1]; 1];
        }

        // span index
        let su = on_find_span_index((nu_pts - 1) as Index, pu as u16, u, &self.ku.knots) as usize;
        let sv = on_find_span_index((nv_pts - 1) as Index, pv as u16, v, &self.kv.knots) as usize;

        // 최대 d차 도함수 (여기서는 2차까지 지원)
        let d = n_der.min(2);

        // 0차 B-spline basis
        let mut nu0 = vec![0.0; pu + 1];
        let mut mv0 = vec![0.0; pv + 1];
        on_basis_funs(su as Index, u, pu as u16, &self.ku.knots, &mut nu0);
        on_basis_funs(sv as Index, v, pv as u16, &self.kv.knots, &mut mv0);

        // 도함수 basis: ders[order][i or j]
        let ders_u = on_ders_basis_funs(&self.ku.knots, su, u, pu, d);
        let ders_v = on_ders_basis_funs(&self.kv.knots, sv, v, pv, d);

        // U/V 방향 basis 테이블 구성 (order 0..d)
        // nu_tab[du][a], mv_tab[dv][b]
        let mut nu_tab = vec![vec![0.0; pu + 1]; d + 1];
        let mut mv_tab = vec![vec![0.0; pv + 1]; d + 1];

        nu_tab[0] = nu0;
        mv_tab[0] = mv0;
        if d >= 1 {
            if ders_u.len() > 1 {
                nu_tab[1] = ders_u[1].clone();
            }
            if ders_v.len() > 1 {
                mv_tab[1] = ders_v[1].clone();
            }
        }
        if d >= 2 {
            if ders_u.len() > 2 {
                nu_tab[2] = ders_u[2].clone();
            }
            if ders_v.len() > 2 {
                mv_tab[2] = ders_v[2].clone();
            }
        }

        // stencil 시작 인덱스
        let iu0 = su - pu;
        let jv0 = sv - pv;

        // 출력 ders[du][dv]
        let mut ders = vec![vec![Vector3D::zero(); d + 1]; d + 1];

        // 라셔널 여부 (현재는 정보용; 공식은 rational/general 케이스 동일하게 사용 가능)
        let _is_rat = Self::check_is_rational(&self.ctrl);

        // 동차 좌표 합 Sw(du,dv), 스칼라 weight 합 W(du,dv)
        let mut sw = vec![vec![Vector3D::zero(); d + 1]; d + 1];
        let mut w = vec![vec![0.0; d + 1]; d + 1];

        // Sw(du,dv) = Σ Σ N_i^(du)(u) M_j^(dv)(v) w_ij P_ij
        // W(du,dv)  = Σ Σ N_i^(du)(u) M_j^(dv)(v) w_ij
        for a in 0..=pu {
            for b in 0..=pv {
                let i = iu0 + a;
                let j = jv0 + b;
                if i >= nu_pts || j >= nv_pts {
                    continue;
                }

                let cp = self.ctrl[Self::idx_row_major(self.nu, i, j)];
                let ph = Vector3D::new(cp.x, cp.y, cp.z);
                let w_ij = cp.w;

                for du in 0..=d {
                    let n_u = nu_tab[du][a];
                    for dv in 0..=d {
                        let n_v = mv_tab[dv][b];
                        let n_uv = n_u * n_v;
                        sw[du][dv] += ph * n_uv;
                        w[du][dv] += w_ij * n_uv;
                    }
                }
            }
        }

        // 몫의 미분 공식 적용
        let w00 = w[0][0];
        let eps = 1e-14;
        if w00.abs() < eps {
            // weight가 0에 가까우면 fallback: 동차 벡터 그대로 사용 (실제로는 퇴화 케이스)
            for du in 0..=d {
                for dv in 0..=d {
                    ders[du][dv] = sw[du][dv];
                }
            }
            return ders;
        }

        let w2 = w00 * w00;
        let w3 = w2 * w00;

        let s = sw[0][0];

        // 0차
        ders[0][0] = s / w00;

        // d == 0 이면 여기서 종료 (위에서 Pos만 채운 상태)
        if d == 0 {
            return ders;
        }

        // -------- 1차 도함수 준비 (d >= 1 임)
        let s_u = sw[1][0];
        let s_v = sw[0][1];
        let w_u = w[1][0];
        let w_v = w[0][1];

        ders[1][0] = (s_u * w00 - s * w_u) / w2;
        ders[0][1] = (s_v * w00 - s * w_v) / w2;

        // -------- 2차 도함수 (d >= 2 임)
        if d >= 2 {
            let s_uu = sw[2][0];
            let s_uv = sw[1][1];
            let s_vv = sw[0][2];

            let w_uu = w[2][0];
            let w_uv = w[1][1];
            let w_vv = w[0][2];

            // c_uu = [S_uu*W^2 - 2 S_u W W_u - S W_uu W + 2 S W_u^2] / W^3
            ders[2][0] = (s_uu * w2 - (s_u * (2.0 * w00 * w_u)) - (s * (w_uu * w00))
                + (s * (2.0 * w_u * w_u)))
                / w3;

            // c_vv
            ders[0][2] = (s_vv * w2 - (s_v * (2.0 * w00 * w_v)) - (s * (w_vv * w00))
                + (s * (2.0 * w_v * w_v)))
                / w3;

            // c_uv = [S_uv*W^2 - S_u W W_v - S_v W W_u - S W_uv W + 2 S W_u W_v] / W^3
            ders[1][1] =
                (s_uv * w2 - (s_u * (w00 * w_v)) - (s_v * (w00 * w_u)) - (s * (w_uv * w00))
                    + (s * (2.0 * w_u * w_v)))
                    / w3;
        }

        ders
    }
}
```
```rust

impl NurbsSurface {
    /// Compute gradient of f(u,v) = 0.5 * ||S(u,v) - target||^2 in UV.
    ///   df/du = dot(S - P, Su)
    ///   df/dv = dot(S - P, Sv)
    /// - clamp_uv=true 이면 (u,v)를 surface domain에 clamp합니다.
    /// - 도함수 평가가 실패하면 None 반환(또는 프로젝트 스타일에 맞게 Result로 바꿔도 됨).
    pub fn distance_gradient_uv(
        &self,
        target: &Point3D,
        uv: &mut [Real; 2],
        clamp_uv: bool,
    ) -> Option<[Real; 2]> {
        let mut u = uv[0];
        let mut v = uv[1];

        if clamp_uv {
            u = self.clamp_u(u);
            v = self.clamp_v(v);
            uv[0] = u;
            uv[1] = v;
        }

        // 1차 도함수까지 평가: ders[0][0]=S, ders[1][0]=Su, ders[0][1]=Sv
        let ders = self.eval_ders_nder(u, v, 1);
        let s = ders.get(0)?.get(0)?.clone();
        let su = ders.get(1)?.get(0)?.clone();
        let sv = ders.get(0)?.get(1)?.clone();

        // diff = S - P
        let diff = s - target.to_vector();

        let gu = diff.dot(&su);
        let gv = diff.dot(&sv);

        on_dbg!("[dist_grad] u={:.6} v={:.6} gu={:.6e} gv={:.6e}", u, v, gu, gv);

        Some([gu, gv])
    }


    pub fn squared_distance_and_grad_uv(
        &self,
        u: Real,
        v: Real,
        target: &Point3D,
    ) -> Option<(Real, [Real; 2])> {
        let ders = self.eval_ders_nder(u, v, 1);
        let s = ders[0][0];
        let su = ders[1][0];
        let sv = ders[0][1];

        let diff = s - target.to_vector(); // 또는 (s.as_point - target) 형태로 맞추세요

        let f = 0.5 * diff.dot(&diff);
        let gu = diff.dot(&su);
        let gv = diff.dot(&sv);
        Some((f, [gu, gv]))
    }
}
```

---

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::nurbs_surface::NurbsSurface;
    use nurbslib::core::geom::{Point3D};
    use nurbslib::core::param_surface::ParamSurface;
    use nurbslib::core::point_ops::PointOps;
    use nurbslib::core::types::{Real};
    fn f_half_dist2(srf: &NurbsSurface, u: Real, v: Real, p: &Point3D) -> Real {
        let s = srf.point_at(u, v);
        let d = s - *p;
        0.5 * d.dot(&d)
    }

    fn finite_diff_grad(srf: &NurbsSurface, u: Real, v: Real, p: &Point3D, h: Real) -> [Real; 2] {
        // 중앙차분 (도메인 밖으로 나갈 수 있으니 clamp 사용)
        let um = srf.clamp_u(u - h);
        let up = srf.clamp_u(u + h);
        let vm = srf.clamp_v(v - h);
        let vp = srf.clamp_v(v + h);

        let fu = (f_half_dist2(srf, up, v, p) - f_half_dist2(srf, um, v, p)) / (up - um);
        let fv = (f_half_dist2(srf, u, vp, p) - f_half_dist2(srf, u, vm, p)) / (vp - vm);
        [fu, fv]
    }

    fn assert_near(a: Real, b: Real, tol: Real, msg: &str) {
        let e = (a - b).abs();
        assert!(e <= tol, "{}: |{}-{}|={}", msg, a, b, e);
    }

    #[test]
    fn distance_gradient_matches_finite_difference_on_dummy_surface() {
        let srf = NurbsSurface::dummy_surface();

        println!("{:?}", srf);

        // target point (임의)
        let p = Point3D::new(0.3, 0.8, 0.2);

        // 내부 파라미터
        let (u, v) = (0.37, 0.63);

        let mut uv = [u, v];
        let g = srf.distance_gradient_uv(&p, &mut uv, true).expect("grad failed");

        let h = 1e-7;
        let gd = finite_diff_grad(&srf, uv[0], uv[1], &p, h);

        println!("gd[0] = {:?},  g[0] = {:?}", gd[0], g[0]);
        println!("gd[1] = {:?},  g[1] = {:?}", gd[1], g[1]);

        // 허용오차는 surface 평가/유리함수에 따라 조절
        assert_near(g[0], gd[0], 1e-5, "du gradient");
        assert_near(g[1], gd[1], 1e-5, "dv gradient");
    }

    #[test]
    fn distance_gradient_clamps_uv_when_requested() {
        let srf = NurbsSurface::dummy_surface();
        let p = Point3D::new(10.0, -10.0, 0.0);

        // 일부러 밖으로 던짐
        let mut uv = [-100.0, 100.0];

        let _g = srf.distance_gradient_uv(&p, &mut uv, true).expect("grad failed");

        // dummy_surface는 도메인이 [0,1]로 동작하는 구조였으니
        assert!(uv[0] >= 0.0 && uv[0] <= 1.0);
        assert!(uv[1] >= 0.0 && uv[1] <= 1.0);
    }


    fn fd_grad_sqdist(srf: &NurbsSurface, u: f64, v: f64, p: &Point3D, h: f64) -> [f64; 2] {
        let f = |uu: f64, vv: f64| -> f64 {
            let s = srf.point_at(uu, vv); // surface point
            let d = s - *p;
            0.5 * d.dot(&d)
        };

        let du = (f(u + h, v) - f(u - h, v)) / (2.0 * h);
        let dv = (f(u, v + h) - f(u, v - h)) / (2.0 * h);
        [du, dv]
    }

    #[test]
    fn distance_gradient_matches_finite_difference_on_rational_surface() {
        let srf = NurbsSurface::dummy_surface(); // rational이라면 그대로
        let p = Point3D::new(0.3, 0.8, 0.2);

        let (u, v) = (0.37, 0.63);

        // analytic
        let ders = srf.eval_ders_nder(u, v, 1);
        let s = ders[0][0];
        let su = ders[1][0];
        let sv = ders[0][1];

        let diff = s - p.to_vector();
        let g = [diff.dot(&su), diff.dot(&sv)];

        // finite diff (same objective!)
        let h = 1e-7;
        let gd = fd_grad_sqdist(&srf, u, v, &p, h);

        assert_near(g[0], gd[0], 1e-5, "du gradient");
        assert_near(g[1], gd[1], 1e-5, "dv gradient");
    }
}
```
---

