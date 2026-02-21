# Conic Arc 만들기

## 🧠 on_comp_weight_closest_conic_arc 하는 일 (요약)
- **세 점 P0–P1–P2로 정의되는 원뿔곡선(conic arc)** 이 있을 때,  
  이 conic arc가 원(circle)에 가장 가까운 형태가 되도록 하는 weight w를 계산한다.
- 즉:
  - P0–P2는 끝점
  - P1은 중간 제어점
  - 이 세 점으로 conic arc를 만들 때    
    원에 가장 가까운 conic weight를 구하는 공식.


## 📐 수학적 의미
- 원에 가장 가까운 conic arc는 weight w 가 다음과 같을 때 얻어진다:
```math
w=\frac{s}{1-s}
```
- 여기서 s 는 다음과 같이 정의된다:
- $d=|P_0-P_2|/2$ (끝점 거리의 절반)
- $f_l=|P_0-P_1|$
- $f_r=|P_2-P_1|$
```math
s_l=\frac{d}{d+f_l},\quad s_r=\frac{d}{d+f_r}
```
```math
s=\frac{s_l+s_r}{2}
```

## 🔍 코드 단계별 해석
```rust
let mut d = p0.distance(&p2);
let fl = p0.distance(&p1);
let fr = p2.distance(&p1);
```
- d = 끝점 거리
- fl = 왼쪽 끝점–중간점 거리
- fr = 오른쪽 끝점–중간점 거리

### 1) 특수 케이스 처리
```rust
if(d == 0.0 && fl == 0.0) || (d == 0.0 && fr == 0.0){
    return None;
}
```
- P0 = P2 이고 P1도 같은 위치 → 의미 없는 conic
- weight 계산 불가 → None

### 2) 끝점 거리 절반
```rust
d = d * 0.5;
```

- 수식의 $d=|P_0-P_2|/2$ 를 구현.

### 3) s_l, s_r 계산
```rust
let sl = d / (d + fl);
let sr = d / (d + fr);
```

### 4) 평균 s
```rust
let s = 0.5 * (sl + sr);
```
### 5) weight 계산
```rust
if s == 1.0 { return None; }

Some(s / (1.0 - s))
```

- 즉:
```math
w=\frac{s}{1-s}
```

## 소스 코드
```rust
pub fn on_comp_weight_closest_conic_arc(
    p0 : Point3D,
    p1 : Point3D,
    p2 : Point3D
) -> Option<Real>
{

    let mut d = p0.distance(&p2);
    let fl = p0.distance(&p1);
    let fr = p2.distance(&p1);

    if(d == 0.0 && fl == 0.0) || (d == 0.0 && fr == 0.0){
        return None;
    }

    d = d * 0.5;
    let sl = d / (d + fl);
    let sr = d / (d + fr);
    let s = 0.5 * (sl + sr);

    if s == 1.0 { return None; }

    Some(s / (1.0 - s))
}
```

--- 
# Curve 만들기
## 🧩 1) Conic arc는 2차 Rational Bézier 곡선으로 표현된다
- 세 점 P0–P1–P2로 정의되는 conic arc는 다음 형태의 2차 rational Bézier curve로 구성된다:
```math
C(t)=\frac{(1-t)^2P_0+2wt(1-t)P_1+t^2P_2}{(1-t)^2+2wt(1-t)+t^2}
```
- 여기서
  - P0, P1, P2 = 제어점
  - w = 중간 제어점의 weight
  - t ∈ [0,1]
- 즉, weight는 오직 P1에만 적용된다.

## 🧩 2) w는 바로 “중간 제어점 weight”
- Rust 함수:
```rust
Some(s / (1.0 - s))
```

- 이 값이 바로 P1의 weight다.
- 즉:
  - P0 → weight = 1
  - P1 → weight = w
  - P2 → weight = 1
- 이렇게 구성하면 된다.

## 🧩 3) 실제 NURBS control point 구성 방식
- NURBS에서 control point는 보통 homogeneous 좌표 (xw, yw, zw, w) 로 저장한다.
- 따라서:
- ✔ P0
```rust
Pw0 = (P0.x * 1, P0.y * 1, P0.z * 1, 1)
```

- ✔ P1
```rust
Pw1 = (P1.x * w, P1.y * w, P1.z * w, w)
```

- ✔ P2
```rust
Pw2 = (P2.x * 1, P2.y * 1, P2.z * 1, 1)
```


## 🧩 4) Knot vector는 2차 Bézier이므로 고정
- 2차 Bézier NURBS curve의 knot vector는 항상:
```
[0,0,0,1,1,1]
```
- 즉:
```
knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
degree = 2;
```

## 🧩 5) 정리: w로 conic arc 만드는 전체 절차
- 세 점 P0, P1, P2 입력
- 함수로 w 계산
- control point를 homogeneous로 구성
- P0 = (x, y, z, 1)
- P1 = (xw, yw, z*w, w)
- P2 = (x, y, z, 1)
- degree = 2
- knot = [0,0,0,1,1,1]
- 이것이 바로 conic arc
- 이렇게 하면 원에 가장 가까운 conic arc가 정확히 만들어진다.

## 🧠 왜 이렇게 되는가?
- 2차 rational Bézier curve는 conic section(원, 타원, 포물선, 쌍곡선)을 정확히 표현할 수 있다.
- weight w가 커지면 곡선이 P1 쪽으로 당겨지고
- w가 1이면 일반 quadratic Bézier
- 특정 w 값에서 원호에 가장 가까운 형태가 된다.

---

