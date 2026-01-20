# Scale Control Point With Respect To a Point
## 소스 코드
```rust
pub fn on_scale_point4d(pw : &Point4D, c : &Point3D, f : &Vector3D)
    -> Point4D
{
    // 1) weight 결정
    let w = if pw.w != 1.0 { pw.w } else { 1.0 };

    // 2) g = 1 - f
    let g = Vector3D::new(1.0 - f.x, 1.0 - f.y, 1.0 - f.z);

    // 3) Cw = w * C
    let cw = Point4D::non_homogeneous(w * c.x, w * c.y, w * c.z, w);

    // 4) 최종 스케일
    Point4D {
        x: f.x * pw.x + g.x * cw.x,
        y: f.y * pw.y + g.y * cw.y,
        z: f.z * pw.z + g.z * cw.z,
        w: pw.w,
    }

}
```
## 📘 먼저 코드가 하는 일을 정확히 수식으로 정리
### 1. 입력 
- $w = (Pw.x, Pw.y, Pw.z, Pw.w)$ : homogeneous control point
- $C = (Cx, Cy, Cz)$ : 스케일 중심점
- $f = (fx, fy, fz)$ : 축별 scale factor
### 2. 보조 벡터 
```math
g=(1-f_x,\; 1-f_y,\; 1-f_z)
```
### 3. 중심점 C를 Pw의 weight로 확장한 
```math
Cw=Pw.w\quad (\mathrm{또는\  }1.0)
```
```math
C_w=(wC_x,\; wC_y,\; wC_z,\; w)
```

4. 최종 스케일된 점 Qw각 좌표별로:
```math
Q_x=f_x\cdot Pw_x+g_x\cdot Cw_x
```
```math
Q_y=f_y\cdot Pw_y+g_y\cdot Cw_y
```
```math
Q_z=f_z\cdot Pw_z+g_z\cdot Cw_z
```
- weight는 그대로 유지:
```math
Q_w=Pw_w
```

- 📘 수식적으로 이 함수가 하는 일이 함수는 어떤 중심점 C를 기준으로  
    control point Pw를 축별로 scale 하는 변환이다.
- 즉,
```math
Pw'=C+f\odot (Pw-C)
```
여기서 $\odot$ 는 축별 곱(element-wise multiply).
- Homogeneous 좌표에서는:
```math
Pw'=f\cdot Pw+(1-f)\cdot C_w
```
- 이 스케일 함수가 왜 이렇게 **복잡하게** 보이는지, 사실 이유가 아주 명확.
  - 우리가 다루는 대상이 일반적인 3D 점이 아니라 NURBS의 homogeneous control point(4D 점) 이기 때문.
- 즉,
  - NURBS control point는 단순한 3D 좌표가 아니라 (wx, wy, wz, w) 형태의 4D 공간에서 움직여야 한다.
  - 그래서 스케일도 일반적인 3D 스케일과 완전히 다르게 처리된다.

- 아래에서 단계별로 풀이.

### 1) 왜 단순한 “Pw = C + f·(Pw − C)” 스케일이 아닌가?
- 일반적인 3D 점이라면 스케일은 이렇게 하면 끝:
```math
P'=C+f\cdot (P-C)
```
- 하지만 NURBS control point는 homogeneous 좌표:
```math
Pw=(wx,\; wy,\; wz,\; w)
```
- 즉,
    - 실제 3D 위치는 (x,y,z)
    - 하지만 control point는 (wx,wy,wz,w) 로 저장됨
    - 이 4D 공간에서의 연산이 NURBS 곡선의 모양을 결정함
- 그래서 스케일도 homogeneous 공간에서 수행해야 한다.

### 2) 왜 C(스케일 중심점)도 w를 곱해서 Cw로 변환하는가?
- 코드:
```rust
Cw.x = w * C.x;
Cw.y = w * C.y;
Cw.z = w * C.z;
Cw.w = w;
```

- 이건 수식으로 보면:
```math
C_w=(wC_x,\; wC_y,\; wC_z,\; w)
```
- 즉,
- 스케일 중심점도 Pw와 동일한 weight 공간으로 올려야 한다.
- 왜냐면:
    - Pw는 4D 공간에 있음
    - C는 3D 공간에 있음
    - 둘을 직접 섞으면 안 됨
    - 반드시 같은 공간(4D homogeneous)에서 연산해야 함
- 그래서 C를 4D로 “승격”시키는 과정이 필요함.

### 3) 왜 g = 1 − f 를 사용하나?
- 코드:
```math
g = 1.0 - f;
```

- 수식으로 보면:
```math
P'=f\cdot Pw+(1-f)\cdot C_w
```
- 이건 사실 스케일의 일반식을 homogeneous 공간에서 다시 쓴 것.
- 일반 스케일:
```math
P'=C+f(P-C)
```
- 전개하면:
```math
P'=fP+(1-f)C
```
- 이걸 homogeneous 공간에서 그대로 구현한 것이:
```math
Pw'=f\cdot Pw+(1-f)\cdot C_w
```
- 즉, g는 단순히 **1 − f** 일 뿐이고, 스케일 공식을 그대로 homogeneous 공간에 적용한 것.

### 4) 왜 이렇게 복잡해 보이지만 사실은 단순한가?
- 정리하면:
- ✔ 일반 3D 스케일
```math
P'=fP+(1-f)C
```
- ✔ NURBS control point 스케일 (homogeneous)
```math
Pw'=f\cdot Pw+(1-f)\cdot C_w
```
- 여기서
```math
C_w=(wC_x,\; wC_y,\; wC_z,\; w)
```
- 즉, 동일한 스케일 공식을 4D 공간에서 적용한 것뿐이다.

### 5) 왜 weight(w)를 그대로 유지하나?
- 코드:
```rust
Qw->w = Pw.w;
```

- 이유:
  - 스케일은 위치만 바꾸는 변환
  - weight는 **control point의 영향력** 을 나타내는 값
  - 스케일과는 무관
  - 따라서 w는 그대로 유지하는 것이 맞다

### 6) 결론 — 왜 복잡해 보이는가?
- 정리하면:

| 이유 | 설명 |
|------|------|
| NURBS control point는 4D homogeneous 공간에 존재 | 단순한 3D 점이 아니라 (wx, wy, wz, w) 형태라서 스케일도 4D에서 수행해야 함 |
| 중심점 C도 4D로 변환해야 함 | Pw와 동일한 weight 공간에서 연산해야 하므로 C → Cw 변환 필요 |
| 스케일 공식이 homogeneous 공간에서 적용됨 | Pw' = f·Pw + (1−f)·Cw 형태로 계산됨 |
| weight(w)는 스케일과 무관 | 스케일은 위치 변환이므로 w는 그대로 유지해야 함 |
| 축별 스케일(fx, fy, fz)이 독립적으로 적용됨 | 각 좌표가 서로 다른 스케일 팩터를 가지므로 계산식이 길어짐 |


- 즉,
  - 복잡해 보이지만 사실은 **3D 스케일 공식을 4D homogeneous 공간에서 그대로 적용한 것** 일 뿐이다.

---
### 테스트 코드
```rust
#[cfg(test)]
mod tests_scale_point4d {
    use nurbslib::core::geom::on_scale_point4d;
    use nurbslib::core::prelude::{Point3D, Point4D, Vector3D};

    fn p4(x: f64, y: f64, z: f64, w: f64) -> Point4D {
        Point4D { x, y, z, w }
    }

    fn p3(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    fn v3(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }
```
```rust
    #[test]
    fn test_scale_identity() {
        // f = (1,1,1) → Pw 그대로
        let pw = p4(3.0, 4.0, 5.0, 2.0);
        let c = p3(10.0, 20.0, 30.0);
        let f = v3(1.0, 1.0, 1.0);

        let q = on_scale_point4d(&pw, &c, &f);

        assert_eq!(q.x, pw.x);
        assert_eq!(q.y, pw.y);
        assert_eq!(q.z, pw.z);
        assert_eq!(q.w, pw.w);
    }
```
```rust
    #[test]
    fn test_scale_zero() {
        // f = (0,0,0) → Pw = Cw
        let pw = p4(3.0, 4.0, 5.0, 2.0);
        let c = p3(10.0, 20.0, 30.0);
        let f = v3(0.0, 0.0, 0.0);

        let q = on_scale_point4d(&pw, &c, &f);

        assert_eq!(q.x, 2.0 * 10.0);
        assert_eq!(q.y, 2.0 * 20.0);
        assert_eq!(q.z, 2.0 * 30.0);
        assert_eq!(q.w, pw.w);
    }
```
```rust
    #[test]
    fn test_scale_half() {
        // f = (0.5, 0.5, 0.5)
        // Pw' = 0.5 Pw + 0.5 Cw
        let pw = p4(4.0, 6.0, 8.0, 2.0);
        let c = p3(10.0, 20.0, 30.0);
        let f = v3(0.5, 0.5, 0.5);

        let q = on_scale_point4d(&pw, &c, &f);

        let cw = p4(2.0 * 10.0, 2.0 * 20.0, 2.0 * 30.0, 2.0);

        assert!((q.x - (0.5 * pw.x + 0.5 * cw.x)).abs() < 1e-12);
        assert!((q.y - (0.5 * pw.y + 0.5 * cw.y)).abs() < 1e-12);
        assert!((q.z - (0.5 * pw.z + 0.5 * cw.z)).abs() < 1e-12);
        assert_eq!(q.w, pw.w);
    }
```
```rust
    #[test]
    fn test_axis_independent_scaling() {
        // f = (2, 0.5, 1)
        let pw = p4(3.0, 4.0, 5.0, 1.0);
        let c = p3(0.0, 0.0, 0.0);
        let f = v3(2.0, 0.5, 1.0);

        let q = on_scale_point4d(&pw, &c, &f);

        assert_eq!(q.x, 2.0 * pw.x);
        assert_eq!(q.y, 0.5 * pw.y);
        assert_eq!(q.z, 1.0 * pw.z);
        assert_eq!(q.w, pw.w);
    }
}
```

---
