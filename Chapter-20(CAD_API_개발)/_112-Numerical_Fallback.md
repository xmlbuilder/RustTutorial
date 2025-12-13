# Fallback
- 이 함수의 목적은 주어진 파라미터 위치 (u,v)에서 곡면의 부분 도함수 벡터 $S_u$, $S_v$ 를 수치적으로 근사하는 것입니다.

## 동작 원리
- 곡면은 일반적으로 파라메트릭 함수 $S(u,v)$ 로 표현됩니다.
- $S_u=\frac{\partial S}{\partial u},\quad S_v=\frac{\partial S}{\partial v}$ 를 직접 계산하기 어려울 때, 수치 미분(finite difference) 방법을 사용합니다.
- 이 함수는 **fallback** 버전으로, 보다 정교한 국소 스텝 계산이 실패했을 때 전역적으로 작은 스텝 du, dv를 사용하여 근사합니다.
- 구현 흐름
  - 파라미터 범위 확인 및 클램프
  - u,v가 도메인 경계를 벗어나지 않도록 umin..umax, v_min..v_max 범위 안에서 제한합니다.
    - u_plus = min(u+du, umax), u_minus = max(u-du, umin)
    - v_plus = min(v+dv, v_max), v_minus = max(v-dv, v_min)
  - 중앙 차분 근사
    - $S_u\approx \frac{S(u+du,v)-S(u-du,v)}{(u+du)-(u-du)}$
    - $S_v\approx \frac{S(u,v+dv)-S(u,v-dv)}{(v+dv)-(v-dv)}$
  - Vector3D 구성
    - 각 좌표(x,y,z)에 대해 차분을 계산하여 벡터로 반환합니다.
## 반환값
- (su, sv) :
  - su → u 방향의 접선 벡터 (부분 도함수)
  - sv → v 방향의 접선 벡터 (부분 도함수)

- 즉, 이 함수는 곡면의 접선 벡터를 안전하게 근사하기 위한 기본 수치 미분 루틴입니다.
- 정교한 국소 스텝 계산이 불가능할 때, 전역적으로 작은 du, dv를 사용한 fallback 방식으로 동작.

---

## NurbsSurface 미분 함수
```rust
 fn numeric_partials_fallback(
      &self,
      u: Real,
      v: Real,
      du: Real,
      dv: Real,
  ) -> (Vector3D, Vector3D) {
      let (umin, umax) = self.param_range_u();
      let (v_min, v_max) = self.param_range_v();
      let u_plus = (u + du).min(umax);
      let u_minus = (u - du).max(umin);
      let v_plus = (v + dv).min(v_max);
      let v_minus = (v - dv).max(v_min);

      let p_u_plus = self.point_at(u_plus, v);
      let p_u_minus = self.point_at(u_minus, v);
      let su = Vector3D {
          x: (p_u_plus.x - p_u_minus.x) / (u_plus - u_minus),
          y: (p_u_plus.y - p_u_minus.y) / (u_plus - u_minus),
          z: (p_u_plus.z - p_u_minus.z) / (u_plus - u_minus),
      };

      let p_v_plus = self.point_at(u, v_plus);
      let p_v_minus = self.point_at(u, v_minus);
      let sv = Vector3D {
          x: (p_v_plus.x - p_v_minus.x) / (v_plus - v_minus),
          y: (p_v_plus.y - p_v_minus.y) / (v_plus - v_minus),
          z: (p_v_plus.z - p_v_minus.z) / (v_plus - v_minus),
      };
      (su, sv)
  }
```
