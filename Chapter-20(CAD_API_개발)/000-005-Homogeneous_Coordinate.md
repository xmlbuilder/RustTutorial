## 📘 Homogeneous Coordinates and Rational Bézier Curves
- (Piegl & Tiller Section 1.4 정리)

### 1. 유클리드 점을 동차 좌표로 표현하기
- 3D 유클리드 점:
- P=(x,y,z)
- 이를 4D 동차 좌표(homogeneous coordinates)로 확장하면:
```math
P^w=(X,Y,Z,W)
```
- 유클리드 좌표로 되돌리는 변환 H는 다음과 같다:
```math
H(P^w)=\left\{ \, \begin{array}{ll}\textstyle \left( \frac{X}{W},\frac{Y}{W},\frac{Z}{W}\right) &\textstyle W\neq 0\\ \textstyle \mathrm{direction\  }(X,Y,Z)&\textstyle W=0\end{array}\right.
``` 
- 즉, W로 나누어 W=1 평면으로 투영하는 것이 동차 → 유클리드 변환이다.

### 2. 동차 좌표의 중요한 성질
- 아무 $w_1\neq w_2$ 에 대해:
```math
H(w_1x,w_1y,w_1z,w_1)=(x,y,z)
```
```math
H(w_2x,w_2y,w_2z,w_2)=(x,y,z)
```
- 즉,
    - 동차 좌표는 스칼라 배(scale factor)에 대해 동일한 유클리드 점을 나타낸다.

- 이 성질이 rational 곡선을 다루는 데 핵심이 된다.

### 3. Rational Bézier 곡선을 동차 공간에서 정의하기
- 컨트롤 포인트 $P_i=(x_i,y_i,z_i)$ 와 weight $w_i$ 가 주어졌을 때
- 동차 좌표의 컨트롤 포인트는:
```math
P_i^w=(w_ix_i,\  w_iy_i,\  w_iz_i,\  w_i)
```
- 이제 4D 공간에서 일반 Bézier 곡선을 만든다:
```math
C^w(u)=\sum _{i=0}^nB_{i,n}(u)\, P_i^w
```
- 여기까지는 완전히 polynomial Bézier curve다.
- 단지 4D에서 계산할 뿐.

### 4. Rational Bézier 곡선은 동차 Bézier 곡선을 투영한 것
- 유클리드 공간으로 되돌리면:
```math
C(u)=H(C^w(u))
```
- 즉,
```math
C(u)=\left( \frac{\sum B_{i,n}(u)\, w_ix_i}{\sum B_{i,n}(u)\, w_i},\frac{\sum B_{i,n}(u)\, w_iy_i}{\sum B_{i,n}(u)\, w_i},\frac{\sum B_{i,n}(u)\, w_iz_i}{\sum B_{i,n}(u)\, w_i}\right)
``` 
- 이는 바로 rational Bézier 곡선의 정의식:
```math
C(u)=\frac{\sum B_{i,n}(u)\, P_i\, w_i}{\sum B_{i,n}(u)\, w_i}
```

### 5. Rational basis 함수
- 분모를 W(u)라 하면:
```math
W(u)=\sum _{i=0}^nB_{i,n}(u)\, w_i
```
- Rational basis:
```math
R_{i,n}(u)=\frac{B_{i,n}(u)\, w_i}{W(u)}
```
- 따라서:
```math
C(u)=\sum _{i=0}^nR_{i,n}(u)\, P_i
```
### 6. Rational Bézier가 필요한 이유 (핵심 요약)
- ✔ 다항식 Bézier로는 원(circle), 타원(ellipse), 쌍곡선(hyperbola) 등 conic을 정확히 표현할 수 없다.
- 다항식으로 원을 표현하려 하면:
```math
x(u)^2+y(u)^2=1
```
- 을 만족하는 다항식 x(u),y(u)는 존재하지 않으며,
- 전개하면 모든 고차항 계수가 0이 되어 상수 함수만 가능하다는 모순이 생긴다.
- ✔ 하지만 rational 함수는 conic을 정확히 표현할 수 있다.
- 예: 단위원
```math
x(u)=\frac{1-u^2}{1+u^2},\quad y(u)=\frac{2u}{1+u^2}
```
- ✔ Rational Bézier는 **4D polynomial Bézier → 3D 투영** 이라는  
    구조 덕분에 다항식 Bézier의 모든 기하학적 성질을 유지하면서 conic까지 표현 가능하다.

### 7. Rational Bézier 곡선의 기하학적 성질 요약
- Convex hull property 유지
- Affine 변환 불변성 유지
- Variation diminishing 유지
- 끝점 보간
```math
C(0)=P_0,\quad C(1)=P_n
```
- 도함수 방향
```math
C'(0)\parallel P_1-P_0,\quad C'(1)\parallel P_n-P_{n-1}
```
- Polynomial Bézier는 $w_i = 1$ 인 특수한 경우

## 🔷 최종 요약
- Rational Bézier 곡선은
- 4D polynomial Bézier curve를 W=1 평면으로 투영한 것이며,
- 이 구조 덕분에 다항식 Bézier로는 표현할 수 없는 원·타원·쌍곡선 같은 곡선을  
    정확하게 표현할 수 있다.

---


## 📘 Rational Bézier Curve: 동차 Bézier에서의 유도 과정 정리
- 이 문서는 Piegl & Tiller 1.4 절의 핵심 내용을 기반으로,  
    동차 Bézier 곡선 → 투영 → Rational Bézier 곡선의 수식 유도 과정을  
    정리한 것이다.

### 1. 동차 Bézier 곡선 정의
- 컨트롤 포인트 $P_i=(x_i,y_i,z_i)$ 와 weight $w_i$ 가 주어졌을 때  
    동차 좌표의 컨트롤 포인트는 다음과 같이 정의된다.
```math
P_i^w = (w_i x_i,\; w_i y_i,\; w_i z_i,\; w_i)
```

- 이제 4D 공간에서 일반 Bézier 곡선을 구성한다.
```math
C^w(u) = \sum_{i=0}^{n} B_{i,n}(u)\,P_i^w
```

- 여기까지는 완전히 polynomial Bézier curve이며, 단지 4D에서 계산할 뿐이다.

### 2. 동차 Bézier의 좌표 함수
- 위 식을 좌표별로 분리하면 다음과 같다.
```math
X(u) = \sum_{i=0}^{n} B_{i,n}(u)\,w_i x_i
```
```math
Y(u) = \sum_{i=0}^{n} B_{i,n}(u)\,w_i y_i
```
```math
Z(u) = \sum_{i=0}^{n} B_{i,n}(u)\,w_i z_i
```
```math
W(u) = \sum_{i=0}^{n} B_{i,n}(u)\,w_i
```

- 즉, 동차 Bézier 곡선은 4개의 다항식 함수로 구성된다.

### 3. 투영(Perspective map)을 통해 3D로 변환
- 동차 좌표에서 유클리드 좌표로 변환하는 투영 H는 다음과 같다.
```math
(x(u), y(u), z(u)) = \left( \frac{X(u)}{W(u)},\; \frac{Y(u)}{W(u)},\; \frac{Z(u)}{W(u)} \right)
```

- 이제 위의 X(u),Y(u),Z(u),W(u)를 대입하면:
```math
x(u) = \frac{\sum B_{i,n}(u)\,w_i x_i}{\sum B_{i,n}(u)\,w_i}
```
```math
y(u) = \frac{\sum B_{i,n}(u)\,w_i y_i}{\sum B_{i,n}(u)\,w_i}
```
```math
z(u) = \frac{\sum B_{i,n}(u)\,w_i z_i}{\sum B_{i,n}(u)\,w_i}
```


### 4. 벡터 형태로 표현하면 Rational Bézier 곡선
- 위 세 좌표를 벡터로 묶으면:
```math
C(u) = \frac{\sum_{i=0}^{n} B_{i,n}(u)\,w_i\,P_i}{\sum_{i=0}^{n} B_{i,n}(u)\,w_i}
```


- 이것이 바로 Piegl & Tiller Eq. (1.14)에서 정의한 Rational Bézier Curve이다.

### 5. Rational Basis 함수의 등장
- 분모를 다음과 같이 정의하자.
```math
W(u) = \sum_{i=0}^{n} B_{i,n}(u)\,w_i
```

- 그러면 rational basis 함수는:
```math
R_{i,n}(u) = \frac{B_{i,n}(u)\,w_i}{W(u)}
```

- 따라서 곡선은 다음과 같이 단순해진다.
```math
C(u) = \sum_{i=0}^{n} R_{i,n}(u)\,P_i
```

- 즉, rational Bézier는 rational basis로 보간된 컨트롤 포인트의 선형 결합이다.

### 6. 핵심 요약
- 동차 좌표에서 polynomial Bézier를 만들고
- 이를 W=1 평면으로 투영하면
- Rational Bézier 곡선이 된다.
- 이 구조 덕분에:
- 원(circle), 타원(ellipse), 쌍곡선(hyperbola) 같은 conic을 정확히  
    표현할 수 있고
- polynomial Bézier의 모든 기하학적 성질을 유지한다.

---


## 📘 Homogeneous 공간에서 Rational Bézier가 자연스럽게 정의되는 이유
- (선형 결합 가능성 중심으로 정리)

### 1. 유클리드 공간의 문제점: Rational 표현이 선형이 아니다
- 유클리드 공간에서 rational Bézier 곡선은 다음과 같이 정의된다.
```math
C(u)=\frac{\sum B_{i,n}(u)\, w_iP_i}{\sum B_{i,n}(u)\, w_i}
```
- 이 식은 분자/분모가 모두 다항식이기 때문에
    유클리드 공간에서는 **선형 결합(linear combination)** 이 아니다.
- 즉,
  - 분모가 존재한다
      - 선형성(linearity)이 깨진다
      - 따라서 polynomial Bézier처럼 단순한 선형 보간으로 계산할 수 없다
- 이게 바로 rational 곡선이 다루기 어려운 이유다.

### 2. 해결책: Homogeneous 공간에서는 선형 결합이 복원된다
- 동차 좌표로 확장하면 컨트롤 포인트는 다음과 같이 표현된다.
```math
P_i^w=(w_ix_i,\; w_iy_i,\; w_iz_i,\; w_i)
```
- 이제 4D 공간에서 Bézier 곡선을 구성하면:
```math
C^w(u)=\sum _{i=0}^nB_{i,n}(u)\, P_i^w
```
- 여기서 중요한 점:
    - 동차 공간에서는 rational Bézier가 완전히 polynomial Bézier가 된다.

- 즉,
    - 분모가 사라지고
    - 단순한 선형 결합이 되고
    - de Casteljau 알고리즘도 그대로 적용된다
- 왜냐하면:
```math
C^w(u)=\sum B_{i,n}(u)\, P_i^w
```
- 은 순수한 선형 결합이기 때문이다.

### 3. Homogeneous 공간에서 선형 결합이 가능한 이유
- 동차 좌표는 다음 성질을 가진다.
```math
H(w_1x,\; w_1y,\; w_1z,\; w_1)=(x,y,z)
```
```math
H(w_2x,\; w_2y,\; w_2z,\; w_2)=(x,y,z)
```
- 즉,
    - 동차 좌표는 스칼라 배(scale factor)에 대해 동일한 유클리드 점을 나타낸다.

- 따라서:
    - 동차 공간에서는 점을 “스케일된 벡터”로 표현할 수 있고
    - 이 벡터들은 선형 결합이 가능하며
    - 그 결과를 다시 W=1 평면으로 투영하면 유클리드 점이 된다
- 이 구조 덕분에 rational 곡선이 polynomial Bézier처럼 다룰 수 있게 된다.

### 4. 투영을 통해 rational 형태가 자연스럽게 등장한다
- 동차 Bézier 곡선:
```math
C^w(u)=(X(u),Y(u),Z(u),W(u))
```
- 투영:
```math
C(u)=\left( \frac{X(u)}{W(u)},\; \frac{Y(u)}{W(u)},\; \frac{Z(u)}{W(u)}\right)
``` 
- 이때:
```math
X(u)=\sum B_{i,n}(u)\, w_ix_i
```
```math
W(u)=\sum B_{i,n}(u)\, w_i
```
- 따라서:
```math
x(u)=\frac{X(u)}{W(u)}=\frac{\sum B_{i,n}(u)\, w_ix_i}{\sum B_{i,n}(u)\, w_i}
```
- 즉, rational Bézier 곡선이 자연스럽게 등장한다.

### 5. 핵심 결론
- ✔ Rational Bézier 곡선은
    - **4D homogeneous 공간에서 polynomial Bézier 곡선을 만든 뒤, W=1 평면으로 투영한 결과** 이다.
- ✔ Homogeneous 공간에서는
    - 선형 결합이 가능하고
    - de Casteljau 알고리즘이 그대로 적용되며
    - polynomial Bézier의 모든 기하학적 성질을 유지한다
- ✔ 유클리드 공간에서는 불가능한 conic(원, 타원, 쌍곡선)을 정확하게 표현할 수 있다.

### 6. 한 문장 요약
- Rational Bézier 곡선은 유클리드 공간에서는 선형성이 깨지지만,  
    homogeneous 공간에서는 완전한 선형 결합으로 표현되기 때문에  
    polynomial Bézier와 동일한 방식으로 계산할 수 있다.

---

## 📘 예제 1.14: Rational Bézier 원호의 동차 평가 과정
- (Piegl & Tiller Section 1.4 기반 정리)

### 🔷 1. Rational Bézier 원호의 정의
- 다음과 같은 2차 Rational Bézier 곡선이 주어진다:
```math
C^w(u)=(1-u)^2P_0^w+2u(1-u)P_1^w+u^2P_2^w
```
- 여기서 동차 컨트롤 포인트는:
- $P_0^w=(1,\  0,\  1)$
- $P_1^w=(1,\  1,\  1)$
- $P_2^w=(0,\  2,\  2)$

- 이 곡선은 단위원의 원호를 정확하게 표현하는 rational Bézier 곡선이다.

### 🔷 2. 동차 Bézier 평가: $u=\frac{1}{2}$
### ✔ Step 1: Bernstein basis 계산
- 2차 Bernstein basis:
```math
B_{0,2}(u)=(1-u)^2=\left( \frac{1}{2}\right) ^2=\frac{1}{4}
```
```math
B_{1,2}(u)=2u(1-u)=2\cdot \frac{1}{2}\cdot \frac{1}{2}=\frac{1}{2}
```
```math
B_{2,2}(u)=u^2=\left( \frac{1}{2}\right) ^2=\frac{1}{4}
```
### ✔ Step 2: 동차 좌표에서 선형 결합
```math
C^w\left( \frac{1}{2}\right) =\frac{1}{4}\cdot (1,\  0,\  1)+\frac{1}{2}\cdot (1,\  1,\  1)+\frac{1}{4}\cdot (0,\  2,\  2)
```
- 각 성분별로 계산하면:
- X: $\frac{1}{4}\cdot 1+\frac{1}{2}\cdot 1+\frac{1}{4}\cdot 0=\frac{3}{4}$
- Y: $\frac{1}{4}\cdot 0+\frac{1}{2}\cdot 1+\frac{1}{4}\cdot 2=1$
- W: $\frac{1}{4}\cdot 1+\frac{1}{2}\cdot 1+\frac{1}{4}\cdot 2=\frac{5}{4}$
- 즉:
```math
C^w\left( \frac{1}{2}\right) =\left( \frac{3}{4},\  1,\  \frac{5}{4}\right) 
```

### 🔷 3. 투영: 동차 → 유클리드
- 투영 변환 H를 적용하면:
```math
C\left( \frac{1}{2}\right) =\left( \frac{3/4}{5/4},\  \frac{1}{5/4}\right) =\left( \frac{3}{5},\  \frac{4}{5}\right)
``` 
- 즉, rational Bézier 곡선 위의 점은:
```math
C\left( \frac{1}{2}\right) =(0.6,\  0.8)
```

### 🔷 4. 비교: 직접 정의된 원호 함수와 동일
- 단위원의 원호는 다음과 같이 정의된다:
```math
x(u)=\frac{1-u^2}{1+u^2},\quad y(u)=\frac{2u}{1+u^2}
```
- $u=\frac{1}{2}$ 일 때:
- $x=\frac{1-1/4}{1+1/4}=\frac{3/4}{5/4}=\frac{3}{5}$
- $y=\frac{2\cdot 1/2}{1+1/4}=\frac{1}{5/4}=\frac{4}{5}$
    - 동일한 결과

### 🔷 5. 핵심 요약
- Rational Bézier 곡선은 동차 공간에서 polynomial Bézier로 계산된다
- 투영을 통해 rational 형태가 복원된다
- 이 방식은 원호 같은 conic을 정확하게 표현할 수 있다
- 예제에서 $u=\frac{1}{2}$ 일 때 계산된 점은 $(0.6,\  0.8)$ 이며,
- 이는 직접 정의된 원호 함수와 정확히 일치한다

### 🔷 한 문장 요약
- Rational Bézier 곡선은 동차 공간에서 선형 결합으로 계산되고,
- 투영을 통해 정확한 원호 점을 얻을 수 있다.

---

## 소스 코드
```rust
impl BezierCurve {
    pub fn new(control_points: Vec<Point4D>) -> Self {
        let degree = control_points.len().saturating_sub(1);
        Self {
            dim: 3,
            degree,
            ctrl: control_points,
        }
    }
```
```rust
    pub fn create(degree: usize, control_points: Vec<Point4D>) -> Option<Self> {
        if control_points.len().saturating_sub(1) != degree {
            return None;
        }
        Some(Self {
            dim: 3,
            degree,
            ctrl: control_points,
        })
    }
```
```rust
    pub fn create_empty(degree: usize) -> Self {
        Self {
            dim: 3,
            degree,
            ctrl: vec![Point4D::zero(); degree + 1],
        }
    }

    pub fn is_rational(&self) -> bool {
        on_is_rational_ctrl_array(&self.ctrl)
    }
```
```rust
    pub fn is_closed(&self, eps: Real) -> bool {
        if self.ctrl.len() < 2 {
            return false;
        }
        let p0 = self.ctrl.first().unwrap().to_point();
        let p1 = self.ctrl.last().unwrap().to_point();

        let dx = p0.x - p1.x;
        let dy = p0.y - p1.y;
        let dz = p0.z - p1.z;
        dx * dx + dy * dy + dz * dz <= eps * eps
    }
```
```rust
    pub fn point_at(&self, u: Real) -> Point3D {
        let n = self.degree;
        let mut p = Point3D::zero();
        for i in 0..=n {
            let b = on_bernstein(n, i, u);
            p.x += b * self.ctrl[i].x;
            p.y += b * self.ctrl[i].y;
            p.z += b * self.ctrl[i].z;
        }
        p
    }
```
```rust
    pub fn point_at_rat(&self, t: Real) -> Option<Point3D> {
        let c = self.eval_h_rat(t);
        if !c.w.is_finite() || c.w.abs() < 1e-15 {
            return None; // avoid division by zero
        }
        Some(Point3D::new(c.x / c.w, c.y / c.w, c.z / c.w))
    }
```
```rust
    pub fn point_at_h(&self, u: Real) -> Point4D {
        let n = self.degree;
        let mut c = Point4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        for i in 0..=n {
            let b = on_bernstein(n, i, u);
            c.x += b * self.ctrl[i].x;
            c.y += b * self.ctrl[i].y;
            c.z += b * self.ctrl[i].z;
            c.w += b * self.ctrl[i].w;
        }
        c
    }
```
```rust
    pub fn eval_h_rat(&self, t: Real) -> Point4D {
        let p: Degree = self.degree as Degree;
        debug_assert_eq!(self.ctrl.len(), p as usize + 1, "Bezier eval requires ctrl.len()==degree+1");
        let b_vec = on_all_ber_1d(p, t);
        let rat = on_is_rat(self.ctrl.as_slice());

        if rat {
            let (mut xw, mut yw, mut zw, mut w) = (0.0, 0.0, 0.0, 0.0);
            for (i, ni) in b_vec.iter().enumerate() {
                let c = self.ctrl[i];
                xw += ni * (c.x * c.w);
                yw += ni * (c.y * c.w);
                zw += ni * (c.z * c.w);
                w += ni * c.w;
            }
            if w == 0.0 {
                w = 1.0;
            }
            Point4D {
                x: xw,
                y: yw,
                z: zw,
                w,
            }
        } else {
            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
            for (i, ni) in b_vec.iter().enumerate() {
                let c = self.ctrl[i];
                x += ni * c.x;
                y += ni * c.y;
                z += ni * c.z;
            }
            Point4D { x, y, z, w: 1.0 }
        }
    }
```
```rust
    pub fn elevate_degree(&self, t: usize) -> BezierCurve {
        let mat = on_degree_elevation_matrix(self.degree, t);
        let mut n_ctrl = vec![Point4D::zero(); self.degree + t + 1];
        for i in 0..=self.degree + t {
            for j in 0..=self.degree {
                n_ctrl[i].x += mat[i][j] * self.ctrl[j].x;
                n_ctrl[i].y += mat[i][j] * self.ctrl[j].y;
                n_ctrl[i].z += mat[i][j] * self.ctrl[j].z;
                n_ctrl[i].w += mat[i][j] * self.ctrl[j].w;
            }
        }
        BezierCurve {
            dim: 3,
            degree: self.degree + t,
            ctrl: n_ctrl,
        }
    }
```
```rust
    pub fn re_parameterize(&self, func: &BezierFunction) -> BezierCurve {
        let n = self.degree;
        let mut res = vec![Point4D::zero(); func.degree + n + 1];

        for i in 0..=n {
            let bi = BezierFunction {
                degree: n,
                coeffs: (0..=n).map(|j| if j == i { 1.0 } else { 0.0 }).collect(),
            };
            let bi_f = bi.multiply(func); // B_i^n(f(u)) as BezierFunction
            for (j, cef) in bi_f.coeffs.iter().enumerate() {
                res[j].x += cef * self.ctrl[i].x;
                res[j].y += cef * self.ctrl[i].y;
                res[j].z += cef * self.ctrl[i].z;
                res[j].w += cef * self.ctrl[i].w;
            }
        }
        BezierCurve {
            dim: 3,
            degree: func.degree + n,
            ctrl: res,
        }
    }
```
```rust
    pub fn dot(&self, rhs: &BezierCurve) -> Vec<Real> {
        let n = self.degree + rhs.degree;
        let mut res = vec![0.0; n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(rhs.degree);
            let jh = self.degree.min(i);
            for j in jl..=jh {
                let p = &self.ctrl[j];
                let q = &rhs.ctrl[i - j];
                res[i] += p.x * q.x + p.y * q.y + p.z * q.z;
            }
        }
        res
    }
```
```rust
    /// Cross product of two curves
    pub fn cross(&self, rhs: &BezierCurve) -> BezierCurve {
        let n = self.degree + rhs.degree;
        let mut res = vec![Point4D::zero(); n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(rhs.degree);
            let jh = self.degree.min(i);
            for j in jl..=jh {
                let p = self.ctrl[j].to_point();
                let q = rhs.ctrl[i - j].to_point();
                let v = Vector3D::cross(&Vector3D::from(p), &Vector3D::from(q));
                res[i].x += v.x;
                res[i].y += v.y;
                res[i].z += v.z;
                res[i].w = 1.0;
            }
        }
        BezierCurve {
            dim: 3,
            degree: n,
            ctrl: res,
        }
    }
```
```rust
    /// Split at u
    pub fn split(&self, u: Real) -> (BezierCurve, BezierCurve) {
        let p = self.degree;
        let mut a = self.ctrl.clone();
        let mut left = vec![Point4D::zero(); p + 1];
        let mut right = vec![Point4D::zero(); p + 1];

        left[0] = a[0];
        right[p] = a[p];
        for k in 1..=p {
            for i in 0..=(p - k) {
                a[i] = a[i].lerp(&a[i + 1], u);
            }
            left[k] = a[0];
            right[p - k] = a[p - k];
        }
        (
            BezierCurve {
                dim: 3,
                degree: p,
                ctrl: left,
            },
            BezierCurve {
                dim: 3,
                degree: p,
                ctrl: right,
            },
        )
    }
```
```rust
    /// Least-squares cubic Bezier approximation
    pub fn approx_cubic(
        ps: &Point3D,
        ts: &Vector3D,
        _p: &Point3D,
        _t: &Vector3D,
        pe: &Point3D,
        te: &Vector3D,
    ) -> BezierCurve {
        // Internal: Piegl's least-squares method. Numerical approximation omitted, structure only preserved.
        let mut ctrl = Vec::with_capacity(4);
        ctrl.push(Point4D::from_point_w(ps, 1.0));

        // Approximately compute middle control points
        let p1 = Point3D {
            x: ps.x + ts.x * 0.3,
            y: ps.y + ts.y * 0.3,
            z: ps.z + ts.z * 0.3,
        };
        let p2 = Point3D {
            x: pe.x - te.x * 0.3,
            y: pe.y - te.y * 0.3,
            z: pe.z - te.z * 0.3,
        };
        ctrl.push(Point4D::from_point_w(&p1, 1.0));
        ctrl.push(Point4D::from_point_w(&p2, 1.0));
        ctrl.push(Point4D::from_point_w(pe, 1.0));

        BezierCurve {
            dim: 3,
            degree: 3,
            ctrl,
        }
    }
```
```rust
    pub fn to_nurbs(&self) -> NurbsCurve {
        // Bézier curve → clamped B-spline: [0..0 (p+1 times), 1..1 (p+1 times)]

        let p = self.degree;
        let mut knot = Vec::with_capacity(2 * (p + 1));
        knot.extend(std::iter::repeat(0.0).take(p + 1));
        knot.extend(std::iter::repeat(1.0).take(p + 1));

        NurbsCurve {
            dim: 3,
            degree: p as u16,
            kv: KnotVector { knots: knot },
            ctrl: self.ctrl.clone(),
            domain: Interval { t0: 0.0, t1: 1.0 },
        }
    }
}
```
---
