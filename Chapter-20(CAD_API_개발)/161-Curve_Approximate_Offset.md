## 1. offset_curve_approx 함수의 역할과 전체 흐름
- 이 함수가 하는 일은 한 줄로 요약하면:
    - 주어진 NURBS 곡선 C(u)에 대해, 거리 d만큼의 **오프셋 곡선** 을 샘플링 + 근사 방식으로 만들어서,  
        다시 NURBS 곡선으로 반환한다.

- 구체적으로:
    - 호 길이 기반으로 파라미터 u를 균등 분할
    - 각 u에서 Frenet frame 또는 fallback 프레임을 이용해 법선 방향으로 거리 d만큼 이동한 점들을 생성
    - 그 점 집합을 대상으로 global least-squares NURBS 근사를 수행해 새로운 NURBS 곡선을 만든다

## 2. 수식 관점에서의 정의
### 2.1. 원래 곡선
- 원래 NURBS 곡선 C(u)는 (비가중 좌표계에서) 보통 다음과 같이 쓸 수 있음:
```math
C(u)=\frac{\sum _{i=0}^nN_{i,p}(u)\, w_i\, \mathbf{P_{\mathnormal{i}}}}{\sum _{i=0}^nN_{i,p}(u)\, w_i}
```
- $N_{i,p}(u)$: B-spline basis 함수 (차수 p)
- $\mathbf{P_{\mathnormal{i}}}$: 제어점
- $w_i$: 가중치
- 여기서 eval_point(u)는 사실상 위 수식을 계산하는 함수라고 보면 된다.
### 2.2. Frenet frame과 offset
- 공간 곡선 C(u)의 1차, 2차 도함수를 각각
```math
\mathbf{C}'(u),\quad \mathbf{C}''(u)
```
- 라고 하면, 곡선이 충분히 $regular( \| \mathbf{C}'(u)\| \neq 0 )$ 이고 곡률이 0이 아닌 구간에서 Frenet frame은:
- 단위 접선 벡터
```math
\mathbf{T}(u)=\frac{\mathbf{C}'(u)}{\| \mathbf{C}'(u)\| }
```
- 단위 법선 벡터
```math
\mathbf{N}(u)=\frac{\mathbf{T}'(u)}{\| \mathbf{T}'(u)\| }
```
- 이 둘의 외적
```math
\mathbf{B}(u)=\mathbf{T}(u)\times \mathbf{N}(u)
```

- 이론적인 offset 곡선은:
```math
C_{\mathrm{off}}(u)=C(u)+d\, \mathbf{N}(u)
```
여기서 d가 dist에 해당한다.
하지만 실제 구현에서는:
- frenet_frame_struct(u)에서 $\mathbf{T}$, $\mathbf{N}$, $\mathbf{B}$ 를 구해주고,
- 실패하거나 $\kappa \approx 0$ 인 구간에서는 fallback normal을 사용한다.
### 2.3. fallback normal (Gram–Schmidt)
- Frenet frame을 못 쓰는 경우, 코드에서는:
    - 접선 벡터 $\mathbf{t}$ 를 eval_first_derivative(u)로 얻고 정규화
    - 임의의 up 벡터 $\mathbf{u}$ 를 선택 (기본은 (0,0,1), colinear면 (0,1,0))
- Gram–Schmidt로 $\mathbf{t}$ 에 직교하는 벡터 생성:
```math
\mathbf{n_{\mathrm{temp}}}=\mathbf{u}-(\mathbf{u}\cdot \mathbf{t})\mathbf{t}
```
```math
\mathbf{n_{\mathrm{temp}}}\leftarrow \frac{\mathbf{n_{\mathrm{temp}}}}{\| \mathbf{n_{\mathrm{temp}}}\| }
```
- 이 $\mathbf{n_{\mathrm{temp}}}$ 를 법선 후보로 사용해서
```math
C_{\mathrm{off}}(u)\approx C(u)+d\, \mathbf{n_{\mathrm{temp}}}
```
- 를 만든다.
### 2.4. normal 방향 연속성 보정
- Frenet normal은 부호가 갑자기 flip될 수 있다. 그래서 코드에서는:
```rust
if let Some(pn) = prev_n {
    if pn.dot(&n) < 0.0 {
        n = -n;
    }
}
prev_n = Some(n);
```

- 이렇게 이전 normal과 내적을 보고, 음수면 방향을 뒤집어서 연속적인 normal field를 유지한다.
- 이건 실제로 offset 곡선이 “지그재그”로 튀는 걸 막는 중요한 디테일이다.

## 3. 샘플링 전략: 호 길이 기반 균등 분할
- uniform_params_by_arc_length(...)는 대략 이런 일을 한다고 보면 된다:
    - 곡선의 전체 호 길이 L를 수치적으로 추정
    - $n_{\mathrm{samples}}+1$ 개의 점을 만들기 위해, 호 길이 기준으로
    ```math
    s_k=\frac{k}{n_{\mathrm{samples}}}L,\quad k=0,1,\dots ,n_{\mathrm{samples}}
    ```
    - 각 $s_k$ 에 대응하는 파라미터 $u_k$ 를 역으로 찾아서 u_params에 저장
- 이렇게 하면 곡선의 기하적 분포에 맞춰 샘플링되기 때문에, 단순한 파라미터 균등 분할보다 offset 근사 품질이 좋아진다.

## 4. global least-squares 근사
- 마지막 단계에서:
```rust
let offset_curve = NurbsCurve::global_approximation(
    &offset_points,
    fit_degree,
    n_ctrl,
)
```

- 이 부분은 다음 문제를 푸는 것과 같다:
    - 주어진 점 집합 $\{ \mathbf{Q_{\mathnormal{k}}}\} _{k=0}^m$ (여기서는 offset_points)
    - 차수 $p=\mathrm{fit\_ degree}$
    - 제어점 개수 $n_{\mathrm{ctrl}}$
    - 에 대해, 어떤 NURBS (또는 B-spline) 곡선 $\tilde {C}(u)$ 를 찾아서
    ```math
    \sum _{k=0}^m\| \tilde {C}(u_k)-\mathbf{Q_{\mathnormal{k}}}\| ^2
    ```
    - 를 최소화하는 전역 최소제곱 문제를 푸는 것.
    - 이건 보통
    ```math
    \mathbf{AP}\approx \mathbf{Q}
    ```
    - 형태의 선형 least-squares로 풀고, $\mathbf{P}$ 가 새로운 제어점 벡터가 된다.

## 5. 코드 리뷰: 
- 짧게 핵심만 설명.
- 입력 방어
    - n_samples < 4, n_ctrl < fit_degree+1 체크: 타당.
    - dist == 0 처리
```rust
if dist.abs() <= 0.0 {
    return Ok(self.clone());
}
```
- 여기서 <= 0.0은 사실상 == 0.0과 동일이긴 한데, 보통은
```rust
if dist.abs() < std::f64::EPSILON { ... }
```
- 같은 식으로 쓰면 의도가 더 명확.
- Frenet → fallback → **원래 점 순서**
- 프레임이 안 나오면 tangent 기반 fallback
    - tangent도 안 나오면 그냥 원래 점 사용
        - “최소한 원래 곡선에서 크게 망가지지는 않게” 하는 안전장치로 적절.
- normal flip 방지
- Frenet과 fallback 모두에 대해 동일한 로직 적용.

## 6. 테스트 코드 예시들
- offset은 눈으로도 확인해야 하지만, 자동 테스트로 **기본적인 성질** 을 체크할 수 있음.
### 6.1. 단순 2D 직선 offset (평면 내)
직선 C(u)=(u,0,0)를 생각해보자.
이걸 x–y 평면에 두고, y 방향으로 offset하면, 결과는 또 다른 직선이 된다.
```rust
#[test]
fn test_offset_straight_line() {

    // x축 위의 직선: (0,0) -> (3,0)
    let curve = NurbsCurve::new(
        1 as Degree,
        vec![
            Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(3.0, 0.0, 0.0, 1.0),
        ],
        KnotVector::from_vec(vec![0.0, 0.0, 1.0, 1.0]),
    ).expect("Failed to create line NurbsCurve");

    let dist: Real = 2.0;
    let offset = curve
        .offset_curve_approx(dist, 16, 1 as Degree, 2)
        .expect("offset_curve_approx failed");

    // 몇 개의 파라미터에서 y좌표가 거의 dist인지 확인
    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        let p = offset.eval_point(u);
        println!("p {}, dist {}", p, dist);
        assert!((p.z - dist).abs() < 1.0e-3, "y={} not close to {}", p.y, dist);
    }
}

```

### 6.2. dist == 0일 때 clone 동작 확인
```rust
#[test]
fn test_offset_zero_distance_returns_clone() {


    let curve = make_test_curve_nonrational();
    let offset = curve
        .offset_curve_approx(0.0 as Real, 16, curve.degree, curve.ctrl.len())
        .expect("offset_curve_approx failed");

    // 제어점, knot, degree 등이 동일한지 확인
    assert_eq!(curve.degree, offset.degree);
    assert_eq!(curve.kv.knots, offset.kv.knots);
    assert_eq!(curve.ctrl.len(), offset.ctrl.len());
    for (a, b) in curve.ctrl.iter().zip(offset.ctrl.iter()) {
        assert!((a.x - b.x).abs() < 1e-12);
        assert!((a.y - b.y).abs() < 1e-12);
        assert!((a.z - b.z).abs() < 1e-12);
        assert!((a.w - b.w).abs() < 1e-12);
    }
}
```

### 6.3. Frenet 실패 구간에서 fallback이 잘 동작하는지
- 예를 들어, 일부 구간에서 곡률이 거의 0인 “긴 직선 + 곡선” 조합을 만들어서,  
    offset이 튀지 않고 부드럽게 나오는지 수치적으로 체크.


## 7. KnotRemovalState 구성 요소 테이블

| 필드 이름          | 인덱스 기준 | 의미                                                                 |
|--------------------|------------|----------------------------------------------------------------------|
| `removal_bound[r]` | knot index | knot 인덱스 `r`에서, 그 knot 블록(마지막 인덱스가 `r`)을 1회 제거했을 때의 추정 오차 상한 |
| `accumulated_err[i]` | knot interval index `i` | knot 구간 `[U[i], U[i+1]]`에 대해 지금까지 누적된 제거 오차 (approx knot removal 누적 에러) |
| `multiplicity[r]`  | knot index | knot 값이 같은 블록의 마지막 인덱스가 `r`일 때, 그 블록의 중복도(멀티플리시티)         |


---
## 폐곡선은 Offset은 우선 차단

### 1. 우선: 폐곡선 차단 버전 코드
```rust
impl NurbsCurve {
    /// Offset NURBS curve (approximation)
    pub fn offset_curve_approx(
        &self,
        dist: Real,
        n_samples: usize,
        fit_degree: Degree,
        n_ctrl: usize,
    ) -> Result<NurbsCurve> {
        // --- 0) 기본 방어 ---
        if self.is_closed() {
            return Err(NurbsError::InvalidArgument {
                msg: "offset_curve_approx: closed curves are not
                  supported (use a dedicated closed-offset method)".into(),
            });
        }

        if n_samples < 4 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "offset_curve_approx: n_samples={} too small (need >= 4)",
                    n_samples
                ),
            });
        }
        if n_ctrl < (fit_degree as usize + 1) {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "offset_curve_approx: n_ctrl={} too small for degree {} (need >= p+1)",
                    n_ctrl, fit_degree
                ),
            });
        }

        if dist.abs() <= 0.0 {
            return Ok(self.clone());
        }

        // --- 1) 파라미터 샘플링: 호 길이 기준 균등 분할 ---
        let mut u_params = self.uniform_params_by_arc_length(
            n_samples + 1,
            1e-6,
            20,
            64,
        );

        if u_params.len() < 2 {
            return Err(NurbsError::NumericError {
                msg: "offset_curve_approx: failed to generate arc-length parameters".into(),
            });
        }

        // (이 아래는 네가 마지막에 올린 버전 그대로)
        // closed curve 처리, fallback normal, fit_params, global_approximation_with_params ...
        // ...
        # let _ = (&mut u_params, fit_degree, n_ctrl); // dummy to keep snippet compilable
        unimplemented!()
    }
}
```

- 이렇게 해두면:
    - 열린 곡선(open curve): 지금 구현 그대로 사용
    - 폐곡선(closed curve): 명시적으로 에러를 던져서 **이 함수는 근사 오프셋용이고,폐곡선은 다른 루틴을 써야 한다** 는  
        의도가 코드 레벨에서 드러남

---

## 1. 왜 폐곡선에서 문제가 되는가?
- 지금의 offset_curve_approx는 3D 일반 곡선을 대상으로 한다.
- 이 방식은 **프레임 기반 offset** 인데, 폐곡선에서는 다음과 같은 문제가 필연적으로 발생한다.

### 문제 1) Frenet frame의 불연속성 (normal flip)
- 폐곡선은 시작점과 끝점이 연결되어야 한다.
- 하지만 Frenet normal은 다음과 같은 성질을 가진다:
    - 곡률이 0에 가까운 구간에서 normal 방향이 불안정
    - 곡선이 한 바퀴 돌면 normal이 반대로 뒤집힐 수 있음
    - 폐곡선의 시작점과 끝점에서 normal이 일치하지 않을 수 있음
- 즉:
```
N(0) ≠ N(1)
```
- 이러면 offset curve의 시작점과 끝점이 이어지지 않고 **틈(gap)** 이 생긴다.

### 문제 2) 3D offset은 “정의 자체가 모호함”
- 3D 공간에서 곡선의 offset은 유일하게 정의되지 않는다.
- 왜냐하면:
    - 3D 곡선은 접선 T(u)만 고유하게 정의됨
    - 법선 N(u)은 무한히 많은 선택지가 있음  
        (Frenet normal은 곡률이 0이면 정의 불가)
    - 즉, offset 방향이 **정의된 수학적 개념** 이 아님
- 그래서 3D offset은 프레임 선택에 따라 완전히 다른 곡선이 나옴.
- 폐곡선에서는 이 모호성이 더 크게 드러난다.

### 문제 3) fallback normal이 폐곡선에서 불연속
- fallback normal은:
    - tangent와 가장 직교한 축을 선택
    - Gram–Schmidt로 normal 생성
    - normal flip 방지
- 하지만 폐곡선에서는:
    - 시작점과 끝점에서 tangent가 같아도
    - fallback normal이 다르게 선택될 수 있음
    - normal flip 보정도 “순환 조건”을 만족시키지 못함
- 결과:
```
offset_points[0] ≠ offset_points[last]
```
- 즉, 폐곡선이 깨짐.

### 문제 4) offset curve 자체가 self-intersection을 만들 수 있음
- 폐곡선 offset은 다음과 같은 현상이 생긴다:
    - convex 부분은 바깥 offset이 잘 되지만
    - concave 부분은 offset이 안쪽으로 말려 들어가 self-intersection 발생
    - CAD에서는 이를 **offset loop** 또는 **offset cusp** 라고 부름
- 이건 수학적으로 피할 수 없는 현상이다.

## 2. 그래서 폐곡선 offset은 “2D planar offset”으로 별도 정의해야 한다
- 폐곡선 offset이 안정적으로 정의되는 경우는 단 하나:
    - 곡선이 평면 위에 있을 때(2D)
- 그리고
    - offset은 그 평면에서의 법선 방향으로 정의될 때

- 즉, XY 평면에 있는 폐곡선이라면 offset은 다음처럼 명확하게 정의된다.

## 3. offset_planar_closed_exact — 수학적 정의
- 곡선이 평면(예: XY)에 있다고 하자.
- 곡선:
```rust
C(u)=(x(u),y(u))
```
- 접선:
```math
\mathbf{T}(u)=\frac{C'(u)}{\| C'(u)\| }
```
- 2D 법선은 두 가지 중 하나:
```math
\mathbf{N}(u)=(-T_y(u),T_x(u))
```
- 또는
```math
\mathbf{N}(u)=(T_y(u),-T_x(u))
```
- 이 normal은:
    - Frenet처럼 곡률에 의존하지 않음
    - 항상 정의됨
    - 2D에서는 normal이 유일하게 결정됨
    - 폐곡선에서도 시작점과 끝점이 일치함
- offset curve:
```math
C_{\mathrm{off}}(u)=C(u)+d\cdot \mathbf{N}(u)
```
- 이게 정확한 2D offset의 수학적 정의다.
---

## 소스 코드
```rust
impl NurbsCurve {
    /// Offset NURBS curve (approximation)
    ///
    /// - `dist`        : offset 거리 (양수 = 프레임 법선 방향, 음수 = 반대 방향)
    /// - `n_samples`   : 곡선을 샘플링할 점 개수 (>= 4 권장)
    /// - `fit_degree`  : 근사 곡선의 차수 (보통 원곡선과 동일)
    /// - `n_ctrl`      : 근사 곡선의 제어점 개수 (>= fit_degree+1)
    ///
    /// 알고리즘:
    ///   1) 호길이 기반으로 u 파라미터를 균등 분할
    ///   2) 각 u에서 (가능하면) Frenet frame의 normal 방향으로 dist 이동
    ///      - Frenet 실패 또는 κ≈0 구간에서는 tangent + up 벡터 기반 fallback normal 사용
    ///   3) offset 샘플 점들을 global least-squares로 NURBS 곡선 근사
    ///
    /// 주의:
    ///   - 3D 일반 곡선의 “오프셋”은 정의 자체가 프레임에 의존하며, Frenet은 κ≈0에서 불안정
    ///   - 본 구현은 normal 연속성(부호 flip) 보정을 넣어 안정성을 개선
    pub fn offset_curve_approx(
        &self,
        dist: Real,
        n_samples: usize,
        fit_degree: Degree,
        n_ctrl: usize,
    ) -> Result<NurbsCurve> {
        // --- 0) 기본 방어 ---
        if n_samples < 4 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "offset_curve_approx: n_samples={} too small (need >= 4)",
                    n_samples
                ),
            });
        }
        if n_ctrl < (fit_degree as usize + 1) {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "offset_curve_approx: n_ctrl={} too small for degree {} (need >= p+1)",
                    n_ctrl, fit_degree
                ),
            });
        }

        // dist==0이면 원본 복제
        if dist.abs() <= 0.0 {
            return Ok(self.clone());
        }

        // ✅ 폐곡선은 금지 (주기 제약/periodic fit 없이는 품질 보장 불가)
        if self.is_closed() {
            return Err(NurbsError::InvalidArgument {
                msg: "offset_curve_approx: closed/periodic curves are not supported. \
                  Use offset_closed_curve(...)
                  (periodic fitting) or exact primitives (circle/arc/line)."
                    .into(),
            });
        }

        // --- 1) 파라미터 샘플링: 호 길이 기준 균등 분할 ---
        let mut u_params = self.uniform_params_by_arc_length(
            n_samples + 1, // 양 끝 포함
            1e-6,
            20,
            64,
        );

        if u_params.len() < 2 {
            return Err(NurbsError::NumericError {
                msg: "offset_curve_approx: failed to generate arc-length parameters".into(),
            });
        }

        // closed curve면 시작/끝 파라미터 중복 제거 (u에서)
        if self.is_closed() && u_params.len() >= 2 {
            let last = u_params.last().unwrap();
            let first = u_params[0];
            if (last - first).abs() <= 1e-12 {
                u_params.pop();
            }
        }

        // --- 2) 각 u에서 프레임으로 offset point 생성 ---
        let mut offset_points: Vec<Point3D> = Vec::with_capacity(u_params.len());

        // normal flip 방지(연속성)
        let mut prev_n: Option<Vector3D> = None;

        for &u in &u_params {
            // 2-1) Frenet frame 시도
            if let Some(frame) = self.frenet_frame_struct(u) {
                let p = frame.p;
                let mut n = frame.n;

                if let Some(pn) = prev_n {
                    if pn.dot(&n) < 0.0 {
                        n = -n;
                    }
                }
                prev_n = Some(n);

                offset_points.push(Point3D {
                    x: p.x + dist * n.x,
                    y: p.y + dist * n.y,
                    z: p.z + dist * n.z,
                });
                continue;
            }

            // 2-2) fallback: tangent 기반 normal 구성
            if let Some(tan0) = self.eval_first_derivative(u) {
                let p = self.eval_point(u);

                let mut t = tan0;
                if !t.normalize() {
                    offset_points.push(p);
                    continue;
                }

                // tangent와 가장 직교한 축 선택
                let candidates = [
                    Vector3D::new(1.0, 0.0, 0.0),
                    Vector3D::new(0.0, 1.0, 0.0),
                    Vector3D::new(0.0, 0.0, 1.0),
                ];

                let mut up = candidates[0];
                let mut best_len = up.cross(&t).length();
                for c in &candidates[1..] {
                    let len = c.cross(&t).length();
                    if len > best_len {
                        best_len = len;
                        up = *c;
                    }
                }

                // Gram–Schmidt: up에서 t 성분 제거
                let up_dot_t = up.dot(&t);
                let mut n_temp = up - t * up_dot_t;

                if !n_temp.normalize() {
                    offset_points.push(p);
                    continue;
                }

                if let Some(pn) = prev_n {
                    if pn.dot(&n_temp) < 0.0 {
                        n_temp = -n_temp;
                    }
                }
                prev_n = Some(n_temp);

                offset_points.push(Point3D {
                    x: p.x + dist * n_temp.x,
                    y: p.y + dist * n_temp.y,
                    z: p.z + dist * n_temp.z,
                });
            } else {
                offset_points.push(self.eval_point(u));
            }
        }

        // --- 2.5) closed curve면 points도 중복 제거 (첫/끝이 같은 경우) ---
        // u_params는 이미 pop했지만, frame 계산/수치 오차로 points가 거의 같을 수 있어 제거
        if self.is_closed() && offset_points.len() >= 2 {
            let a = offset_points[0];
            let b = offset_points.last().unwrap();
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dz = a.z - b.z;
            if (dx * dx + dy * dy + dz * dz).sqrt() <= 1e-10 {
                offset_points.pop();
                u_params.pop();
            }
        }

        if offset_points.len() < 2 {
            return Err(NurbsError::NumericError {
                msg: "offset_curve_approx: insufficient offset samples".into(),
            });
        }

        // --- 3) fitting params를 u_params에서 정규화해서 사용 (중요!) ---
        // global_approximation이 내부에서 uniform params를 쓰면 폐곡선/호길이 샘플이 깨진다.
        let u0 = u_params[0];
        let u1 = u_params.last().unwrap();
        let denom = u1 - u0;

        let fit_params: Vec<Real> = if denom.abs() <= 1e-14 {
            // degenerate fallback
            NurbsCurve::uniform_params_ret_vec(u_params.len())
        } else {
            u_params.iter().map(|u| (u - u0) / denom).collect()
        };

        // --- 4) global least-squares 근사 (params 버전 사용) ---
        let offset_curve = NurbsCurve::global_approximation_with_params(
            &offset_points,
            &fit_params,
            fit_degree,
            n_ctrl,
        ).ok_or_else(|| NurbsError::NumericError {
            msg: "offset_curve_approx: global_approximation_with_params failed".into(),
        })?;

        Ok(offset_curve)
    }
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::basis::Side;
    use nurbslib::core::cfun::{cfun_derivatives, CFun};
    use nurbslib::core::knot::KnotVector;
    use nurbslib::core::knots_extensions::{on_compute_basis_maximum, on_evaluate_basis_function,
      on_evaluate_rational_bases_and_derivatives};
    use nurbslib::core::nurbs_curve::NurbsCurve;
    use nurbslib::core::prelude::Point4D;
```
```rust
    #[test]
    fn test_basic_curve_eval() {
        // 직선 NURBS 생성
        let p0 = Point4D::homogeneous(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4D::homogeneous(10.0, 0.0, 0.0, 1.0);

        let mut curve = NurbsCurve::from_ctrl_clamped_uniform(1, vec![p0, p1]);

        // 중간점 평가
        let pt = curve.eval_point(0.5);
        assert!((pt.x - 5.0).abs() < 1e-12);
        assert!((pt.y).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_tangent() {
        let p0 = Point4D::homogeneous(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4D::homogeneous(10.0, 0.0, 0.0, 1.0);

        let curve = NurbsCurve::from_ctrl_clamped_uniform(1, vec![p0, p1]);


        let tan = curve.eval_tangent(0.3).unwrap().unitize();
        println!("{:?}", tan);
        assert!((tan.x - 1.0).abs() < 1e-12);
        assert!(tan.y.abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_basis_function() {
        let kv = KnotVector::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap();
        let curve = NurbsCurve::new(1, vec![
            Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(1.0, 0.0, 0.0, 1.0),
        ], kv.clone()).unwrap();

        let val = on_evaluate_basis_function(&curve, 0, 0.25, Side::Left).unwrap();
        assert!(val > 0.0);
        assert!(val < 1.0);
    }
```
```rust
    #[test]
    fn test_rational_basis_derivatives() {
        let p0 = Point4D::homogeneous(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4D::homogeneous(1.0, 0.0, 0.0, 2.0); // weight 2

        let curve = NurbsCurve::from_ctrl_clamped_uniform(1, vec![p0, p1]);

        let (span, rd) = on_evaluate_rational_bases_and_derivatives(&curve, 0.3, Side::Left, 1).unwrap();

        assert_eq!(rd.len(), 2); // 0차, 1차 도함수
        assert!(rd[0][0] > 0.0);
    }
```
```rust
    #[test]
    fn test_cfun_eval_and_derivatives() {
        let knots = KnotVector::new(vec![0.0, 0.0, 1.0, 1.0]).unwrap();
        let cfn = CFun::new(1, knots, vec![0.0, 10.0]).unwrap();

        let v = cfn.eval(0.5, Side::Left).unwrap();
        assert!((v - 5.0).abs() < 1e-12);

        let d = cfun_derivatives(&cfn, 0.5, Side::Left, 1).unwrap();
        assert!((d[1] - 10.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn test_knot_refinement() {
        let p0 = Point4D::homogeneous(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4D::homogeneous(10.0, 0.0, 0.0, 1.0);

        let mut curve = NurbsCurve::from_ctrl_clamped_uniform(1, vec![p0, p1]);

        curve.refine_knot_vector(&[0.5]);

        assert!(curve.kv.knots.contains(&0.5));
    }
```
```rust
    #[test]
    fn test_basis_maximum() {
        let kv = KnotVector::new(vec![0.0, 0.0, 0.0,1.0, 1.0, 1.0]).unwrap();
        let (max_val, u_at_max) = on_compute_basis_maximum(&kv, 1, 2, 1e-12).unwrap();

        println!("max_val={}, u_at_max={}", max_val, u_at_max);

        assert!(max_val > 0.0);
        assert!(u_at_max >= 0.0 && u_at_max <= 1.0);
    }

}
```
---
