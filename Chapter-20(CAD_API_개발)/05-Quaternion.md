# 쿼터니언을 이용한 벡터 회전

## 🎯 목적: 쿼터니언을 이용한 벡터 회전
## ✅ 수학적 정의
- 벡터 \vec {v}를 쿼터니언 q로 회전시키는 공식은:

$$
\vec {v}_{\mathrm{rotated}}=q\cdot \vec {v}\cdot q^{-1}
$$

- 여기서 \vec {v}는 쿼터니언으로 표현된 순허수 쿼터니언:

$$
v_q=(0,v_x,v_y,v_z)
$$

- $q^{-1}$ 는 q의 역원 (conjugate if normalized)
- 수 쿼터니언:

$$
v_q=(0,v_x,v_y,v_z)
$$

- $q^{-1}$ 는 q의 역원 (conjugate if normalized)

## ✅ 구현 예시
```rust
impl Quaternion {
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotate_vector(&self, v: Vector) -> Vector {
        let qv = Quaternion { w: 0.0, x: v.x, y: v.y, z: v.z };
        let q_conj = self.conjugate();
        let r = *self * qv * q_conj;
        Vector::new(r.x, r.y, r.z)
    }
}
```

- 이 구조는 $q\cdot v_q\cdot q^{-1}$ 를 정확히 구현
- 단, q는 **단위 쿼터니언(normalized)** 이어야 회전만 수행됨

## 📐 Quaternion-based Vector Rotation Summary

| Step            | Formula                                      | Description                         |
|-----------------|----------------------------------------------|-------------------------------------|
| Vector as Quaternion | $v_q = (0, v_x, v_y, v_z)$               | Represent vector as pure quaternion |
| Rotation        | $v' = q \cdot v_q \cdot q^{-1}$           | Rotate using quaternion conjugation |
| Extract result  | $\vec{v}' = (v'_x, v'_y, v'_z)$           | Final rotated vector components     |

---

