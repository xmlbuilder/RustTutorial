# point_inversion (Curve / Surface Point Inversion)

- 이 문서는 `point_inversion.rs`에 들어있는 **Curve / Surface Point Inversion(점 역변환)** 구조를 “나중에 기능을 계속 추가/교체할 수 있도록” 설계 관점에서 정리한 것입니다.  
  - (코드 내부의 이름/타입은 지금 대화에서 공유된 구조를 기준으로 작성했습니다.)

---

## 1. Point Inversion이란?

### Curve inversion
- 주어진 점 `P`에 대해, 곡선 `C(u)` 위에서 거리

$$
g(u)=\|C(u)-P\|^2
$$

- 를 최소로 만드는 파라미터 `u*`를 찾는 문제입니다.

### Surface inversion
- 주어진 점 `P`에 대해, 서페이스 `S(u,v)` 위에서

$$
g(u,v)=\|S(u,v)-P\|^2
$$

- 를 최소로 만드는 `(u*,v*)`를 찾는 문제입니다.

- 둘 다 본질은 **최소제곱(least-squares) 최적화**이며, **Newton / Gauss-Newton / LineSearch / Fallback** 이 자연스럽게 붙습니다.

---

## 2. 현재 구조가 지향하는 목표

- 1. **Seed(초기값) 만들기** 와 **Local refine(국소 수렴)** 을 분리  
- 2. Domain(클램프/랩) 정책을 분리  
- 3. Newton 실패/불안정 시 **Fallback** 을 쉽게 바꿀 수 있게 분리  
- 4. 추후 **Patch 기반(Decompose) + LineSearch2D** 로 확장 가능  
- 5. **Numeric derivative → Analytic derivative** 로 교체 가능  
   - 이미 Surface 쪽에서 analytic 1차/2차 도함수 루틴이 추가된 상태라면, Refiner는 그것을 우선 사용하도록 바꾸는 것이 핵심

---

## 3. Curve Point Inversion 구조 요약

### 3.1 핵심 타입

- `CurvePointInversionOptions`  
  - 반복 횟수, seed 생성 정책(스캔/스프레드), 라인서치 횟수, 브렌트 fallback 등 전체 튜닝 파라미터.

- `CurveInversionFailReason`  
  - 실패 원인을 통일된 enum으로 기록.

- `CurvePointInversionResult`  
  - 결과: `ok`, `u`, `dist2`, `iters`, `used_seed`, `reason`.

### 3.2 Strategy / Policy 분리

- `CurveDomainPolicy`  
  - `CurveClampDomain`: domain 밖을 clamp  
  - `CurveWrapDomain`: periodic 곡선용 wrap

- `CurveSeedStrategy<C: CurveGeom>`  
  - seed 후보 생성.  
  - 기본 구현:  
    - 1) `u0` 주변 후보  
    - 2) (가능하면) knot span 기반 후보  
    - 3) uniform scan에서 best 후보

- `CurveLocalRefiner<C: CurveGeom>`  
  - seed에서 시작해 국소 수렴하는 알고리즘.  
  - 기본 구현: `CurveNewtonLineSearchRefiner`

- `CurveFallbackMinimizer`  
  - Newton 실패/부정확 시 fallback 최적화.  
  - 기본 구현: `CurveBrentFallback`

### 3.3 기본 알고리즘 흐름

- 1) seeds 생성  
- 2) 각 seed에 대해 Newton+LineSearch 실행, 최저 dist2 선택  
- 3) 필요시 Brent fallback 실행해 더 좋은 최소값이 있으면 교체

---

## 4. Surface Point Inversion 구조 요약

- Curve 구조를 거의 그대로 2D로 확장한 형태입니다.

### 4.1 핵심 타입

- `SurfacePointInversionOptions`  
  `scan_u/scan_v`, `seed_spread_u/v`, `newton_step_rel_u/v`, `gn_fallback` 등

- `SurfaceInversionFailReason`  
  `NoDerivatives` 등이 추가됨.

- `SurfacePointInversionResult`  
  결과: `ok`, `(u,v)`, `dist2`, `iters`, `used_seed`, `reason`.

### 4.2 Policy / Strategy

- `SurfaceDomainPolicy`
  - `ClampSurfaceDomain`: clamp
  - `WrapSurfaceDomain`: periodic surface용 wrap

- `SurfaceSeedStrategy<S: SurfaceGeom>`
  - 기본: `(u0,v0)` 주변 격자 + coarse scan best

- `SurfaceLocalRefiner<S: SurfaceGeom>`
  - **Refiner #1**: `Newton2ndLineSearchRefiner`  
    2차 도함수까지 가능한 경우, 2x2 시스템을 풀어 (du,dv) 업데이트  
  - **Refiner #2**: `GaussNewtonDampedRefiner`  
    1차 도함수만으로도 안정적으로 감소하는 방향을 찾는 fallback

### 4.3 Newton2ndLineSearchRefiner의 수식(요지)

목표 함수: `g(u,v)=||S(u,v)-P||^2`  
그라디언트:
- `g_u = 2 (S-P)·S_u`
- `g_v = 2 (S-P)·S_v`

Hessian 근사(정확 Newton을 흉내내는 형태):
- `uu = S_u·S_u + (S-P)·S_uu`
- `uv = S_u·S_v + (S-P)·S_uv`
- `vv = S_v·S_v + (S-P)·S_vv`

- 2x2 선형계:

```math
\begin{bmatrix}
uu  & uv\\
uv & vv
\end{bmatrix}
\begin{bmatrix}
du\\
dv
\end{bmatrix}
=
-
\begin{bmatrix}
(S-P)\cdot S_u\\
(S-P)\cdot S_v
\end{bmatrix}
```

---

## 5. Numeric derivative → Analytic derivative로 바꾸는 이유/방향

- 수치미분은 step 선택에 민감하고, near singular/고곡률에서 노이즈로 det가 깨지거나 descent가 망가질 수 있습니다.  
- 따라서 `SurfaceGeom`에 Analytic derivative가 있다면 다음 정책이 좋습니다.

  - 1) Analytic 우선  
  - 2) Analytic이 없을 때만 Numeric fallback  
  - 3) 둘 다 없으면 `NoDerivatives`

---

## 6. 왜 Decompose(패치 분해)를 PointInversion에 붙이려 하나?

- 전역적으로 inversion이 어려운 이유:
  - 파라미터 영역이 큰 경우 로컬 최소가 여러 개
  - knot span이 크거나 분포가 불균일
  - singular/near singular 영역 존재

- 패치로 나누면:
  - seed 탐색을 패치 단위로 제한 가능
  - 문제 패치만 fallback 강화 가능
  - 경계 처리(클램프/랩)가 단순해짐

---

## 7. 확장 방향 (추가하면 효과 큰 순)

### 7.1 LineSearch2D trait 분리(강력 추천)

- Refiner 내부 backtracking을 전략으로 분리하면 모든 스텝(Newton/GN/LM)에 재사용 가능합니다.

```rust
pub trait LineSearch2D {
    fn search(
        &self,
        dom_u: Interval,
        dom_v: Interval,
        u: f64, v: f64,
        du: f64, dv: f64,
        g0: f64,
        eval_g: &dyn Fn(f64,f64)->f64,
        dom_pol: &dyn SurfaceDomainPolicy,
        max_iters: usize
    ) -> Option<(f64,f64,f64)>; // (u_new, v_new, g_new)
}
```

### 7.2 Fallback 강화 순서(추천)

- 1) Newton2nd + LineSearch2D  
- 2) GN + LineSearch2D  
- 3) 2D coarse grid 재탐색 후 재시도  
- 4) Decompose patch 기반: 패치별 best seed → global best  
- 5) 마지막: 짧은 2D coordinate descent

### 7.3 Singular 줄이기: LM(Levenberg–Marquardt)

- det가 작을 때 바로 실패 대신 `H + λI`로 안정화하고 λ를 키우며 재시도하면 매우 강해집니다.

---

## 8. 테스트 확장 체크리스트

- 기본 성공: bilinear 같은 analytic 서페이스
- 경계: u/v가 0/1 근처, patch 경계 근처
- 어려움: su,sv가 거의 평행, seed가 멀리 있는 케이스
- 회귀: dist뿐 아니라 gradient 작음, perturb에서 dist 증가 확인

---

## 9. 나중에 내가 기능 추가할 때 실전 체크

- 새 Surface 타입: `eval_point`, `domain_u/v`는 필수  
- 가능하면 `evaluate_1st_ders`, 더 좋으면 `evaluate_2nd_ders` 제공  
- Refiner 추가는 `SurfaceLocalRefiner`만 구현하면 주입 가능  
- Patch 기반은 “패치별 seed + best 선택”으로 단순화

---

## 10. 결론

- 지금 구조는 **확장 가능한 뼈대**로 충분히 쓸만합니다.  
- 다음 3개만 들어가면, 이후 강화(LM/trust region/patch 기반)가 매우 쉬워집니다.
  - 1) `LineSearch2D` trait 추가  
  - 2) Refiner들이 line search를 주입받아 사용  
  - 3) derivative 호출을 “Analytic 우선”으로 통일

---

