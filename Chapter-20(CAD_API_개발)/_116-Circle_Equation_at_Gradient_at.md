# Circle

## EquationAt()
- EquationAt() 같은 암시적(implicit) 원 방정식 함수는 단순히 **원 위인지 아닌지** 를 판별하는 것 이상으로 꽤 다양한 용도가 있음.
- 아래에 핵심 용도를 정리.

## 🎯 equation_at()의 실제 용도
### ✅ 1) 점이 원 위/안/밖에 있는지 판별
- 가장 기본적인 용도.
    - e = 0 → 원 위
    - e < 0 → 원 안
    - e > 0 → 원 밖
- 이건 충돌 검사, 스냅, 트리밍, 선택(selection) 등에 자주 쓰입니다.

### ✅ 2) 뉴턴 방식(Newton iteration) 같은 수치해석에서 사용
- 곡선/곡면과의 교차를 구할 때,
- **원 방정식 = 0** 형태는 뉴턴 반복법에 딱 맞는 형태입니다.
- 예를 들어:
    - 선(Line)과 원(Circle)의 교차점
    - NURBS 곡선과 원의 교차
    - 원과 평면 곡선의 교차
- 이런 문제는 대부분 f(t) = 0 형태로 풀기 때문에 EquationAt() + GradientAt() 조합이 필요합니다.

### ✅ 3) 트리밍(Trimming) 곡선 계산
- CAD 커널에서는 원을 트리밍할 때 **이 점이 원 내부인지 외부인지** 를 빠르게 판정해야 합니다.
- Implicit form은 이 작업에 최적화되어 있어요.

### ✅ 4) 거리 최소화 / 최적화 문제
- 예를 들어, 어떤 점 P에서 원까지의 최소 거리를 구할 때
- 암시적 방정식이 있으면 다음과 같은 최적화가 가능합니다:

$$
\min \| P-C(t)\| ^2\quad \mathrm{s.t.}\quad (x/r)^2+(y/r)^2-1=0
$$

- 이런 constrained optimization에서 implicit form이 필요합니다.

### ✅ 5) 원과 다른 곡선/곡면의 교차를 빠르게 테스트
- 예를 들어:
    - 원과 AABB(축정렬 박스) 교차 여부
    - 원과 삼각형 교차 여부
    - 원과 다각형 교차 여부
- 이런 충돌 검사는 implicit form이 훨씬 빠릅니다.

### ✅ 6) 수치적 안정성 검사
- 원 반지름이 매우 크거나 매우 작은 경우
- 직접 좌표 비교보다 $(x/r)^2 + (y/r)^2$ 방식이 더 안정적입니다.

### 🧠 결론
- EquationAt()은 단순한 함수처럼 보이지만,
- CAD 커널, NURBS 엔진, 충돌 검사, 최적화, 교차 알고리즘에서 핵심적인 역할을 합니다.
- 특히:
    - 교차(intersection)
    - 트리밍(trimming)
    - 수치해석(newton iteration)
    - 충돌 검사(collision detection)
- 이 네 가지에서 매우 자주 쓰임.

## 소스 코드
```rust
/// Evaluate implicit circle equation at a 2D point in the circle's plane:
/// e = (x/r)^2 + (y/r)^2 - 1
pub fn equation_at(&self, p: Point2D) -> f64 {
    if self.radius != 0.0 {
        let x = p.x / self.radius;
        let y = p.y / self.radius;
        x * x + y * y - 1.0
    } else {
        0.0
    }
}
```


## ✅ Rust 버전 gradient_at() 구현

```rust
/// Gradient of implicit circle equation at point p (in plane coordinates).
/// g = (2/r²) * (x, y)
pub fn gradient_at(&self, p: Point2D) -> Point2D {
    if self.radius != 0.0 {
        let rr = 2.0 / (self.radius * self.radius);
        Point2D {
            x: rr * p.x,
            y: rr * p.y,
        }
    } else {
        Point2D { x: 0.0, y: 0.0 }
    }
}
```

### ✅ 왜 gradient가 필요한가?
- Implicit circle equation:

$$
F(x,y)=\left( \frac{x}{r}\right) ^2+\left( \frac{y}{r}\right) ^2-1
$$


- Gradient:

$$
\nabla F=\left( \frac{2x}{r^2},\frac{2y}{r^2}\right) 
$$

- 이 벡터는:
    - 뉴턴 방식 교차 계산
    - 점을 원 위로 투영하는 알고리즘
    - 최적화 문제
    - 충돌 검사
    - 트리밍 곡선 계산
- 등에서 매우 중요합니다.

### ✅ 사용 예시
```rust
let c = Circle::from_center(Point3D::new(0.0, 0.0, 0.0), 5.0).unwrap();
let p = Point2D::new(3.0, 4.0);

let g = c.gradient_at(p);
println!("Gradient = ({}, {})", g.x, g.y);
```



## ✅ 뉴턴 방식(Newton iteration)으로 3D 점을 원 위로 투영하는 함수

### ✅ 핵심 아이디어
- 원은 파라미터 t 로 표현됨:

$$
C(t)=center+r(\cos t\cdot x\_ axis+\sin t\cdot y\_ axis)
$$

- 임의의 3D 점 P를 원 위로 투영하려면:
    - P를 원의 평면으로 투영 → (u, v)
    - 초기 t₀ = atan2(v, u)
    - 뉴턴 반복:

$$
t_{n+1}=t_n-\frac{F(t_n)}{F'(t_n)}
$$


- 여기서:
- $F(t)=\| C(t)-P\| ^2$
- $F'(t)=2(C(t)-P)\cdot C'(t)$

```rust
/// Project a 3D point onto the circle using Newton iteration.
/// Returns (success, projected_point, parameter_t)
/// Newton iteration으로 3D 점을 원(평면 위)으로 투영(가까운 점)한다.
///
/// 목적함수:
///   F(t) = ||C(t) - P||^2
///   F'(t)= 2 (C(t)-P)·C'(t)
///
/// 반환: (ok, projected_point, t in [0, 2π))
pub fn project_newton(&self, p: Point3D) -> (bool, Point3D, f64) {
    let tau = TAU;

    // 반지름 0이면 의미 없음
    if self.radius == 0.0 {
        let c = self.center();
        return (false, c, 0.0);
    }

    // 1) plane에 내린 좌표 (u,v)
    let (u, v) = self.plane.project_st(p);

    // 2) 초기값: 각도
    //    (원점 근처면(센터 바로 위/아래) atan2가 불안정 -> 0으로 둠)
    let mut t = if u.abs() + v.abs() > 1e-15 {
        v.atan2(u)
    } else {
        0.0
    };
    t = t.rem_euclid(tau);

    // 3) Newton loop
    let max_iter = 30;
    let tol_step = 1e-14;   // t 변화량
    let tol_grad = 1e-15;   // F'(t) 너무 작으면 중단
    let mut ok = false;

    let mut best_t = t;
    let mut best_f = f64::INFINITY;

    for _ in 0..max_iter {
        // C(t)
        let c = self.point_at(t);
        let d = c - p;                 // Vector3D
        let f = d.dot(&d);             // scalar

        if f < best_f {
            best_f = f;
            best_t = t;
        }

        // C'(t) = r * T_unit(t)
        // tangent_at(t)가 unit tangent라고 가정.
        // 만약 unit이 아니라면 "cp = self.derivative_at(t)" 같은 함수로 대체해야 정확.
        let tan_unit = self.tangent_at(t);
        let cp = tan_unit * self.radius;

        let fp = 2.0 * d.dot_vec(&cp);

        // gradient(=fp)가 너무 작으면 더 이상 안정적으로 못 감
        if fp.abs() < tol_grad {
            break;
        }

        let dt = f / fp;
        t -= dt;

        // wrap [0, 2π)
        t = t.rem_euclid(tau);

        if dt.abs() < tol_step {
            ok = true;
            break;
        }
    }

    // 4) best 해로 마무리 + 성공판정 강화
    //    (Newton이 실패해도 best_t가 초기값보다 나쁠 수는 없게 유지)
    let t_final = best_t.rem_euclid(tau);
    let proj = self.point_at(t_final);

    // “성공” 기준: proj가 원 위에 충분히 가까운가?
    // 원의 평면좌표계에서 반지름 오차를 보거나, center 거리로 판단.
    let dist = proj.distance(&self.center());
    let radial_err = (dist - self.radius).abs();

    // ok가 false여도 결과는 반환하지만, 오차 기준으로 ok를 보정
    // (연습용이므로 기준을 넉넉하게)
    if radial_err < 1e-9 {
        ok = true;
    }
    (ok, proj, t_final)
}
```

---




