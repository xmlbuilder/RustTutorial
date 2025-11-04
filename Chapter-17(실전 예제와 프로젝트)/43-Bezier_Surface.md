# Bezier Surface
## ✅ 주요 기능 점검 및 수식 정리
### 1. from_ctrl_grid / with_degrees
- ctrl의 크기에서 차수를 추론하거나 명시적으로 설정
- 직사각형 여부 검증 포함 → 정상

### 2. evaluate(u, v)
기능: 베지어 곡면의 점 평가
수식:

$$
S(u,v)=\frac{\sum _{i=0}^p\sum _{j=0}^qB_i^p(u)B_j^q(v)P_{ij}w_{ij}}{\sum _{i=0}^p\sum _{j=0}^qB_i^p(u)B_j^q(v)w_{ij}}
$$

- bernstein_all_clamped(p, u) → $B_i^p(u)$
- 동차 좌표로 누적 후 유클리드 변환 → 정확

### 3. elevate_u / elevate_v
기능: u 또는 v 방향 차수 상승
수식:

$$
P_i'=\sum _{k=\max (0,i-inc)}^{\min (p,i)}E_{ik}P_k
$$ 

- degree_elev_matrix(p, inc) → 차수 상승 행렬 E
- 각 열 또는 행에 대해 적용 → 정확

### 4. split_u / split_v
기능: u 또는 v 방향 분할 (de Casteljau 알고리즘)
수식:
- 1D 곡선 분할을 각 열/행에 적용
- split_curve_lerp() 사용 → 선형 보간 기반 분할 → 정상

### 5. elevate_degree_dir(dir, inc)
기능: 방향에 따라 차수 상승
- SurfaceDir::UDir 또는 VDir에 따라 elevate_u / elevate_v와 동일한 로직 수행 → 정상

### 6. to_power_basis(a, b, c, d)
기능: 베지어 곡면을 power basis로 변환
수식 흐름:
- 베지어 → power basis 변환 행렬:  

$$
PUM=\mathrm{power\\_basis\\_matrix}(p),\quad PVM=\mathrm{power\\_basis\\_matrix}(q)
$$


- 구간 재매핑:

$$
RUM=\mathrm{reparam\\_matrix}(p,a,b,0,1),\quad RVM=\mathrm{reparam\\_matrix}(q,c,d,0,1)
$$

- 전체 변환 행렬:

$$
CUM=RUM\cdot PUM,\quad CVM=RVM\cdot PVM
$$

- 최종 변환:

$$
BW_{ij}=\sum _{u=0}^p\sum _{v=0}^qCUM_{iu}\cdot CVM_{jv}\cdot P_{uv}
$$

- 동차 좌표로 누적 → 정확하고 수학적으로 타당

## 🧪 테스트 제안
- evaluate()에서 u, v = 0, 1, 0.5 등 경계값 테스트
- elevate_u() 후 evaluate() 결과 비교
- split_u() 후 두 곡면의 evaluate() 합이 원래와 일치하는지 확인
- to_power_basis() 결과를 수치적으로 검증


## ✅ bernstein_all_clamped(p, u)
### 📌 역할
- 차수 p에 대해 클램핑된 Bernstein basis 함수 $B_i^p(u)$ 전체를 계산
- 반환값: Vec<Real> 형태의 $[B_0^p(u),B_1^p(u),...,B_p^p(u)]$

### 📐 수식 정의
Bernstein basis 함수:

$$
B_i^p(u)={p \choose i}u^i(1-u)^{p-i}
$$

- 이 구현은 재귀적 누적 방식으로 계산하여 수치적으로 안정적이고 효율적입니다.

### ⚙️ 코드 검토
- u=0 또는 u=1일 때 특수 처리 → OK
- 누적 방식으로 basis 계산 → OK
- saved와 omu = 1 - u를 활용한 안정적 계산 → OK

## ✅ split_curve_lerp(a, t)
### 📌 역할
- 1D 베지어 곡선을 매개변수 t\in [0,1]에서 분할
- 입력: 제어점 배열 a (복사본)
- 출력: (left, right) → 각각 [0,t], [t,1] 구간의 제어점

### 📐 수식 설명 (de Casteljau 알고리즘)
- 반복적으로 선형 보간:

$$
P_i^{(k)}=(1-t)P_i^{(k-1)}+tP_{i+1}^{(k-1)}
$$

- 최종적으로:

$$
\mathrm{left}[k]=P_0^{(k)},\quad \mathrm{right}[p-k]=P_{p-k}^{(k)}
$$

## ⚙️ 코드 검토
- left[0] = a[0], right[p] = a[p] → 시작/끝점 설정 OK
- 내부 루프에서 a[i] = a[i].lerp(a[i+1], t) → 선형 보간 OK
- left[k] = a[0], right[p-k] = a[p-k] → 누적 결과 저장 OK

---

## 소스 코드
```rust
#[derive(Debug, Clone)]
pub struct BezierSurface {
    pub u_degree: usize,
    pub v_degree: usize,
    pub ctrl: Vec<Vec<CPoint>>, // [u][v] (len = u_degree+1) x (v_degree+1)
}
```
```rust
impl BezierSurface {
    /// ctrl의 크기로부터 degree를 추론하여 생성.
    /// ctrl는 직사각형이어야 함.
    pub fn from_ctrl_grid(ctrl: Vec<Vec<CPoint>>) -> Result<Self, &'static str> {
        if ctrl.is_empty() || ctrl[0].is_empty() {
            return Err("BezierSurface: empty control net");
        }
        let u_len = ctrl.len();
        let v_len = ctrl[0].len();
        for row in &ctrl {
            if row.len() != v_len {
                return Err("BezierSurface: control net must be rectangular");
            }
        }
        Ok(Self {
            u_degree: u_len - 1,
            v_degree: v_len - 1,
            ctrl,
        })
    }

    /// 명시적 degree로 생성 (검증 포함)
    pub fn with_degrees(u_degree: usize, v_degree: usize, ctrl: Vec<Vec<CPoint>>) -> Result<Self, &'static str> {
        if ctrl.len() != u_degree + 1 { return Err("u rows != u_degree+1"); }
        if ctrl.is_empty() { return Err("empty control net"); }
        let vlen = ctrl[0].len();
        if vlen != v_degree + 1 { return Err("v cols != v_degree+1"); }
        for row in &ctrl {
            if row.len() != vlen { return Err("non-rectangular control net"); }
        }
        Ok(Self { u_degree, v_degree, ctrl })
    }

    #[inline] pub fn size(&self) -> (usize, usize) { (self.u_degree + 1, self.v_degree + 1) }
    #[inline] pub fn order_u(&self) -> usize { self.u_degree + 1 }
    #[inline] pub fn order_v(&self) -> usize { self.v_degree + 1 }

    /// 표면 평가: S(u,v) (동차합 → 유클리드)
    pub fn evaluate(&self, u: Real, v: Real) -> Point {
        let p = self.u_degree;
        let q = self.v_degree;
        debug_assert!((0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v));

        // Bernstein 벡터
        let bu = bernstein_all_clamped(p, u); // 아래 helper 사용
        let bv = bernstein_all_clamped(q, v);

        // 동차 합
        let mut x = 0.0; let mut y = 0.0; let mut z = 0.0; let mut w = 0.0;
        for i in 0..=p {
            for j in 0..=q {
                let b = bu[i] * bv[j];
                let c = &self.ctrl[i][j];
                x += b * c.x;
                y += b * c.y;
                z += b * c.z;
                w += b * c.w;
            }
        }
        if w != 0.0 {
            Point { x: x / w, y: y / w, z: z / w }
        } else {
            // 비가중(또는 w=0)도 케이스 방어
            Point { x, y, z }
        }
    }

    /// u-방향 차수 상승 (B_SDEGEL의 u방향 대응, degree_elev_matrix 재사용)
    pub fn elevate_u(&self, inc: usize) -> BezierSurface {
        if inc == 0 { return self.clone(); }
        let p = self.u_degree;
        let q = self.v_degree;
        let e = degree_elev_matrix(p, inc); // (p+inc+1) x (p+1)

        let new_p = p + inc;
        let mut new_ctrl = vec![vec![CPoint::zero(); q + 1]; new_p + 1];

        for j in 0..=q {
            for i in 0..=new_p {
                let mut acc = CPoint::zero();
                let i_min = i.saturating_sub(inc);
                let i_max = p.min(i);
                for k in i_min..=i_max {
                    let a = e[i][k];
                    acc.x += a * self.ctrl[k][j].x;
                    acc.y += a * self.ctrl[k][j].y;
                    acc.z += a * self.ctrl[k][j].z;
                    acc.w += a * self.ctrl[k][j].w;
                }
                new_ctrl[i][j] = acc;
            }
        }
        BezierSurface { u_degree: new_p, v_degree: q, ctrl: new_ctrl }
    }

    /// v-방향 차수 상승
    pub fn elevate_v(&self, inc: usize) -> BezierSurface {
        if inc == 0 { return self.clone(); }
        let p = self.u_degree;
        let q = self.v_degree;
        let e = degree_elev_matrix(q, inc); // (q+inc+1) x (q+1)

        let new_q = q + inc;
        let mut new_ctrl = vec![vec![CPoint::zero(); new_q + 1]; p + 1];

        for i in 0..=p {
            for j in 0..=new_q {
                let mut acc = CPoint::zero();
                let j_min = j.saturating_sub(inc);
                let j_max = q.min(j);
                for k in j_min..=j_max {
                    let a = e[j][k];
                    acc.x += a * self.ctrl[i][k].x;
                    acc.y += a * self.ctrl[i][k].y;
                    acc.z += a * self.ctrl[i][k].z;
                    acc.w += a * self.ctrl[i][k].w;
                }
                new_ctrl[i][j] = acc;
            }
        }
        BezierSurface { u_degree: p, v_degree: new_q, ctrl: new_ctrl }
    }

    /// u방향 split (de Casteljau를 각 v열별로 적용)
    pub fn split_u(&self, u: Real) -> (BezierSurface, BezierSurface) {
        let p = self.u_degree;
        let q = self.v_degree;

        // 각 v 열에 대해 de Casteljau 1D 분할
        let mut left = vec![vec![CPoint::zero(); q + 1]; p + 1];
        let mut right = vec![vec![CPoint::zero(); q + 1]; p + 1];

        for j in 0..=q {
            let mut col = (0..=p).map(|i| self.ctrl[i][j]).collect::<Vec<_>>();
            // 1D split (CPoint::lerp 사용)
            let (l, r) = split_curve_lerp(&mut col, u);
            for i in 0..=p { left[i][j] = l[i]; right[i][j] = r[i]; }
        }

        (
            BezierSurface { u_degree: p, v_degree: q, ctrl: left },
            BezierSurface { u_degree: p, v_degree: q, ctrl: right },
        )
    }

    /// v방향 split
    pub fn split_v(&self, v: Real) -> (BezierSurface, BezierSurface) {
        let p = self.u_degree;
        let q = self.v_degree;

        let mut left = vec![vec![CPoint::zero(); q + 1]; p + 1];
        let mut right = vec![vec![CPoint::zero(); q + 1]; p + 1];

        for i in 0..=p {
            let mut row = (0..=q).map(|j| self.ctrl[i][j]).collect::<Vec<_>>();
            let (l, r) = split_curve_lerp(&mut row, v);
            for j in 0..=q { left[i][j] = l[j]; right[i][j] = r[j]; }
        }

        (
            BezierSurface { u_degree: p, v_degree: q, ctrl: left },
            BezierSurface { u_degree: p, v_degree: q, ctrl: right },
        )
    }

    pub fn elevate_degree_dir(
        &self,
        dir: SurfaceDir,
        inc: usize,
    ) -> BezierSurface {
        match dir {
            SurfaceDir::UDir => {
                let elev_mat = degree_elev_matrix(self.u_degree, inc);
                let new_u = self.u_degree + inc;
                let vsize = self.v_degree + 1;
                let mut new_ctrl = vec![vec![CPoint::zero(); vsize]; new_u + 1];

                for v in 0..vsize {
                    for i in 0..=new_u {
                        let mut q = CPoint::zero();
                        let a = i.saturating_sub(inc);
                        let b = self.u_degree.min(i);
                        for k in a..=b {
                            q.x += elev_mat[i][k] * self.ctrl[k][v].x;
                            q.y += elev_mat[i][k] * self.ctrl[k][v].y;
                            q.z += elev_mat[i][k] * self.ctrl[k][v].z;
                            q.w += elev_mat[i][k] * self.ctrl[k][v].w;
                        }
                        new_ctrl[i][v] = q;
                    }
                }

                BezierSurface { u_degree: new_u, v_degree: self.v_degree, ctrl: new_ctrl }
            }

            SurfaceDir::VDir => {
                let elev_mat = degree_elev_matrix(self.v_degree, inc);
                let new_v = self.v_degree + inc;
                let usize = self.u_degree + 1;
                let mut new_ctrl = vec![vec![CPoint::zero(); new_v + 1]; usize];

                for u in 0..usize {
                    for j in 0..=new_v {
                        let mut q = CPoint::zero();
                        let a = j.saturating_sub(inc);
                        let b = self.v_degree.min(j);
                        for k in a..=b {
                            q.x += elev_mat[j][k] * self.ctrl[u][k].x;
                            q.y += elev_mat[j][k] * self.ctrl[u][k].y;
                            q.z += elev_mat[j][k] * self.ctrl[u][k].z;
                            q.w += elev_mat[j][k] * self.ctrl[u][k].w;
                        }
                        new_ctrl[u][j] = q;
                    }
                }

                BezierSurface { u_degree: self.u_degree, v_degree: new_v, ctrl: new_ctrl }
            }
        }
    }
    pub fn to_power_basis(
        &self,
        a: f64, b: f64,
        c: f64, d: f64,
    ) -> Vec<Vec<CPoint>> {
        let pum = power_basis_matrix(self.u_degree);
        let pvm = power_basis_matrix(self.v_degree);

        let rum = re_param_matrix(self.u_degree, a, b, 0.0, 1.0);
        let rvm = re_param_matrix(self.v_degree, c, d, 0.0, 1.0);

        // cum = rum * pum, cvm = rvm * pvm
        let cum = Matrix::mul(&rum, &pum);
        let cvm = Matrix::mul(&rvm, &pvm);

        // M_rmcprm(cum, Pw, cvm, bw)
        let m = self.u_degree + 1;
        let n = self.v_degree + 1;
        let mut bw = vec![vec![CPoint::zero(); n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut cp = CPoint::zero();
                for u in 0..m {
                    for v in 0..n {
                        let c = cum[i][u] * cvm[j][v];
                        cp.x += c * self.ctrl[u][v].x;
                        cp.y += c * self.ctrl[u][v].y;
                        cp.z += c * self.ctrl[u][v].z;
                        cp.w += c * self.ctrl[u][v].w;
                    }
                }
                bw[i][j] = cp;
            }
        }
        bw
    }

    pub fn to_nurbs(&self) -> Surface {
        let (nu, nv) = self.dims();
        let degree_u = self.u_degree;
        let degree_v = self.v_degree;

        let knots_u = clamped_uniform_knot_vector(degree_u, nu);
        let knots_v = clamped_uniform_knot_vector(degree_v, nv);

        Surface {
            pu: degree_u,
            pv: degree_v,
            nu,
            nv,
            ctrl: self.ctrl.clone(),
            ku: KnotVector {knots: knots_u},
            kv: KnotVector {knots: knots_v},
        }
    }
}
```





