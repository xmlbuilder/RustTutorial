# extend_with_same_derivatives

## 1. 함수의 목적 한 줄 요약
- extend_with_same_derivatives는:
- Bezier 곡선을 시작/끝 방향으로 **연장** 하되,
- 원래 곡선과 0차~p차까지의 동차(homogeneous) 미분이 완전히 일치하도록  
    새로운 Bezier 곡선을 만들어주는 함수.

- 즉:
- 입력: BezierCurve C(t), 차수 p, 제어점 $P_0,\dots ,P_p$
- 출력: 새로운 BezierCurve Q(t), 같은 차수 p, 제어점 $Q_0,\dots ,Q_p$
- 조건:
- Start 연장일 때:
```math
Q^{(k)}(0)=C^{(k)}(0),\quad k=0,1,\dots ,p
```
- End 연장일 때:
```math
Q^{(k)}(1)=C^{(k)}(1),\quad k=0,1,\dots ,p
```
- 그리고 이건 **동차 좌표(4D, Point4D)** 에서의 미분을 맞추는 거라  
    rational/non-rational 모두에 대해 잘 동작한다.

## 2. Bezier 곡선과 미분의 기본 수식
- 차수 p Bezier 곡선:
```math
C(t)=\sum _{i=0}^pB_i^p(t)P_i
```
여기서 $B_i^p(t)$ 는 Bernstein basis:
```math
B_i^p(t)={p \choose i}t^i(1-t)^{p-i}
```
- k차 미분은:
```math
C^{(k)}(t)=\sum _{i=0}^{p-k}B_i^{p-k}(t)\cdot D_i^{(k)}
```
- 여기서 $D_i^{(k)}$ 는 제어점들의 선형결합(차분)으로 표현된다.
- 특히 t=0 또는 t=1에서의 미분은  
    제어점들의 **유한 차분(finite difference)** 로 정확히 표현 가능하다.
- 이 함수는 바로 그 **끝점에서의 유한 차분 구조** 를 이용해서  
    새로운 Bezier 제어점 $Q_i$ 들을 계산하는 알고리즘이다.

## 3. 함수 전체 구조 개관
```rust
pub fn extend_with_same_derivatives(&self, side: ExtendSide, reverse_param: bool) -> BezierCurve
```

- side
    - ExtendSide::Start → 시작 쪽으로 연장
    - ExtendSide::End → 끝 쪽으로 연장
- reverse_param
    - 파라미터 방향을 뒤집어서 쓸지 여부
- 여기서는 수식 이해에 집중하자:
    - reverse_param=false → “자연스러운” 방향
    - reverse_param=true → 뒤집힌 방향에서의 연장
- 내부 공통:
    - 차수 p
    - 제어점 P_i=pw[i]
    - 이항계수 테이블 bin = C(p, j) (Pascal triangle)
    - 출력:
    - 새 제어점 $Q_i=qw[i]$
    - 같은 차수 p의 BezierCurve

## 4. 공통 준비 단계
```rust
let p = self.degree;

if p == 0 {
    return self.clone();
}

debug_assert!(self.ctrl.len() == p + 1);

let pw = &self.ctrl;
let bin = on_pascal_triangle_f64(p);
let mut qw = vec![Point4D::zero(); p + 1];
```

- 차수 0: 상수 곡선 → 연장 개념이 의미 없으니 그대로 반환
- 제어점 개수 체크: Bezier는 항상 degree+1개
- bin[i][j]: ${i \choose j}$ 이항계수
- qw: 결과 제어점 배열, 처음엔 전부 0

## 5. Start 쪽 연장 (ExtendSide::Start)
### 5-1. 첫 제어점 배치
```rust
if reverse_param {
    qw[0] = pw[0];
} else {
    qw[p] = pw[0];
}
```

- reverse_param=false일 때:
    - 새 곡선의 마지막 제어점 $Q_p$ 에 원래의 $P_0$ 를 둔다.
    - 즉, 새 곡선의 끝점이 원래 곡선의 시작점과 일치하도록.
- reverse_param=true일 때:
    - 새 곡선의 첫 제어점 $Q_0$ 에 $P_0$ 를 둔다.
- 여기서 이미 **어느 쪽에서 미분을 맞출지** 의 기준점이 정해진다.
### 5-2. 부호 제어 변수 초기화
```rust
let mut dgs = -1.0;
let mut cgs = 1.0;
let mut sgs = -1.0;
```

- 이 세 값은 루프를 돌면서 부호 패턴을 제어하는 역할을 한다.
- 유한 차분과 조합할 때 alternating sign(+, -, +, - ...)을 만들기 위해 필요하다.
### 5-3. i 루프: 1차~p차 미분 조건을 순차적으로 만족시키기
```rust
for i in 1..=p {
    let mut dls = dgs;
    let mut cls = cgs;

    // Dw = Σ_{j=0..i} (dls * C(i,j)) * Pw[j], with alternating sign
    let mut dw = Point4D::zero();
    for j in 0..=i {
        let fac = dls * bin[i][j];
        dw.add_scaled(fac, &pw[j]);
        dls = -dls;
    }

    // Aw = Σ_{j=0..i-1} (cls * C(i,j)) * (rev? Qw[j] : Qw[p-j]), alt sign
    let mut aw = Point4D::zero();
    for j in 0..i {
        let fac = cls * bin[i][j];
        if reverse_param {
            aw.add_scaled(fac, &qw[j]);
        } else {
            aw.add_scaled(fac, &qw[p - j]);
        }
        cls = -cls;
    }

    if reverse_param {
        // Qw[i] = -Dw + Aw
        qw[i] = dw.scaled(-1.0).add(aw);
    } else {
        // Qw[p-i] = sgs*Dw + Aw
        qw[p - i] = dw.scaled(sgs).add(aw);
    }

    dgs = -dgs;
    cgs = -cgs;
    sgs = -sgs;
}
```

- 여기서 핵심은:
    - Dw: 원래 곡선의 제어점 P_j들로부터 만들어지는 **목표 미분** 에 해당하는 항
    - Aw: 이미 결정된 Q 제어점들로부터 만들어지는 **기여분**
- 마지막에:
```math
Q\_\mathrm{새로운}=(\mathrm{부호}\cdot Dw)+Aw
```
- 형태로 새 제어점을 하나씩 풀어내는 구조.
- 조금 더 수식적으로 보면:
- Bezier 끝점에서의 k차 미분은  
    제어점들의 선형결합 + 이항계수 + 부호 패턴으로 표현된다.
- 이 루프는 그 관계식을 그대로 코드로 옮긴 것:
    - Dw는 “원래 곡선의 k차 미분에 해당하는 조합”
    - Aw는 “이미 정해진 Q 제어점들이 만들어내는 k차 미분의 일부”
    - 새로 구하는 Q 하나는
        **전체 목표 미분 - 이미 기여한 부분** 을 맞추기 위해 결정된다.
- 즉:
- i를 1에서 p까지 올리면서 1차, 2차, ..., p차 미분 조건을 순차적으로  
    만족시키도록 새로운 제어점들을 하나씩 풀어내는 알고리즘이다.
## 6. End 쪽 연장 
- (ExtendSide::End)구조는 Start와 거의 대칭인데,
    이번엔 **끝점 t=1**에서의 미분을 맞추는 버전이다.
### 6-1. 첫 제어점 배치if 
```rust
!reverse_param {
    qw[0] = pw[p];
} else {
    qw[p] = pw[p];
}
```
- reverse_param=false:
    - 새 곡선의 시작점 $Q_0$ 에 원래의 $P_p$ 를 둔다.
    - 즉, 새 곡선의 시작점이 원래 곡선의 끝점과 일치.
- reverse_param=true:
    - 새 곡선의 끝점 $Q_p$ 에 $P_p$.
### 6-2. 부호 변수 초기화
```rust
let mut cgs = 1.0;
let mut sgs = 1.0;
```
### 6-3. i 루프
```rust
for i in 1..=p {
    let mut dls = 1.0;
    let mut cls = cgs;

    // Dw = Σ_{j=0..i} (dls * C(i,j)) * Pw[p-j], with alternating sign
    let mut dw = Point4D::zero();
    for j in 0..=i {
        let fac = dls * bin[i][j];
        dw.add_scaled(fac, &pw[p - j]);
        dls = -dls;
    }

    // Aw = Σ_{j=0..i-1} (cls * C(i,j)) * (rev? Qw[p-j] : Qw[j]), alt sign
    let mut aw = Point4D::zero();
    for j in 0..i {
        let fac = cls * bin[i][j];
        if !reverse_param {
            aw.add_scaled(fac, &qw[j]);
        } else {
            aw.add_scaled(fac, &qw[p - j]);
        }
        cls = -cls;
    }

    if reverse_param {
        // Qw[p-i] = sgs*Dw + Aw
        qw[p - i] = dw.scaled(sgs).add(aw);
    } else {
        // Qw[i] = Dw + Aw
        qw[i] = dw.add(aw);
    }

    cgs = -cgs;
    sgs = -sgs;
}
```
- 여기도 마찬가지로:
- Pw[p-j]를 쓰는 이유: 끝점 t=1에서의 유한 차분 구조를 반영
    - Dw: 원래 곡선의 끝점에서의 k차 미분에 해당하는 조합
    - Aw: 이미 정해진 Q 제어점들이 만들어내는 기여분
    - 마지막에 새 Q를 하나씩 풀어내면서 1차~p차 미분을 순차적으로 맞춘다.
## 7. 이 함수가 실제로 보장하는 것
- 정리하면, 이 함수는:- Bezier 곡선의 끝점에서의 k차 미분이  
    제어점들의 유한 차분 + 이항계수 + 부호 패턴으로 표현된다는 사실을 이용해서
- **원래 곡선의 제어점 $P_i$** 로부터 **새 곡선의 제어점 $Q_i$** 를 계산한다.
- 그 결과:
- Start 연장:
```math
Q^{(k)}(0)=C^{(k)}(0),\quad k=0..p
```
- End 연장:
```math
Q^{(k)}(1)=C^{(k)}(1),\quad k=0..p
```
- 그리고 이 모든 연산은 **Point4D(동차 좌표)** 에서 이루어지므로:
    - rational Bezier (NURBS의 segment)에도 그대로 적용 가능
    - non-rational은 w=1이라 자연스럽게 포함
## 8. 이 함수의 용도 (실전 맥락)이 함수는 보통 이런 데서 쓰인다:
- Bezier strip extension
- surface 경계에서의 derivative field를 유지하면서 바깥으로 strip을 하나 더 붙일 때
- cross-boundary derivative field 생성
- blend surface, fillet, transition surface 만들 때
- 경계에서의 값/미분을 유지한 채 확장된 패치를 만들기 위해
- Virtual Topology 기반 mesher에서
    - 경계 곡선을 연장해서 안정적인 intersection / trimming / meshing을 하기 위해

## 9. 한 줄로 정리
- extend_with_same_derivatives는 Bezier 곡선을 시작/끝 방향으로 연장하면서,
    0차~p차까지의 동차 미분을 원래 곡선과 완전히 일치시키도록
- 새 제어점을 유한 차분 + 이항계수 조합으로 풀어내는 알고리즘이다.

---

## ⭐ extend_with_same_derivatives는
- 곡선의 **형상** 을 연장하는 게 아니라,  
    곡선의 **끝점에서의 미분(0~p차)** 을 그대로 유지한 채  
    새로운 Bezier 곡선을 만들어내는 함수다.
- 즉,
- 연장되는 것은 **미분 정보(derivative field)** 이지, 기존 곡선의 기하(shape)가 아니다.

### 1. 이 함수는 “곡선을 이어붙이는” 함수가 아니다
- 보통 “extend”라는 단어 때문에:
    - 기존 곡선 뒤에 뭔가 붙인다
    - 기존 곡선이 길어진다
    - 기존 곡선의 shape가 확장된다
- 이 함수는 그런 의미의 extend가 아니다.
이 함수는:
- 원래 곡선과 같은 미분을 갖는 새로운 Bezier 곡선을 하나 더 만들어주는 함수다.

- 즉,
- 기존 곡선은 그대로 있고, 그 옆에 “미분이 동일한 또 하나의 Bezier 곡선”을 만들어내는 것이다.

### 2. 무엇을 연장하는가?
- **끝점에서의 미분(0~p차)** 을 연장한다
- Bezier 곡선의 끝점에서의 미분은 제어점으로 표현된다:
    - 0차 미분: 위치
    - 1차 미분: 접선
    - 2차 미분: 곡률
    - …
    - p차 미분까지
- 이 함수는:
    - 원래 곡선의 끝점에서의 모든 미분(0~p차)을  
        새로운 곡선의 시작점(또는 끝점)에서 그대로 재현한다.

- 즉:
- Start 연장:
```math
Q^{(k)}(0)=C^{(k)}(0)
```
- End 연장:
```math
Q^{(k)}(1)=C^{(k)}(1)
```
- 여기서 Q(t)는 새로 만들어진 Bezier 곡선이다.

### 3. 왜 이런 “미분 연장”이 필요한가?
- 이 함수는 CAD 커널에서 다음 용도로 쓰인다:
- ✔ 1) Bezier strip extension
- Surface 경계에서 derivative field를 유지한 채  
    바깥으로 strip을 하나 더 붙일 때
- ✔ 2) cross-boundary derivative field 생성
- Blend surface, fillet, transition surface 만들 때  
    경계에서의 미분을 유지한 채 확장된 패치를 만들기 위해
- ✔ 3) Virtual Topology 기반 mesher
- 경계 곡선을 연장해서  
    안정적인 intersection / trimming / meshing을 하기 위해
- 즉:
    - 곡선의 shape를 늘리는 게 아니라,  
        경계에서의 미분 조건을 유지한 **새로운 곡선** 을 만드는 것이다.


### 4. 왜 side(START/END)만 있나?
- **어느 끝점의 미분을 복제할지** 만 결정하면 되기 때문
- 이 함수는:
    - 기존 곡선의 시작점에서의 미분을 복제할지
    - 기존 곡선의 끝점에서의 미분을 복제할지
- 이 두 가지 중 하나만 결정하면 된다.
- 그래서 입력은:
```
side: ExtendSide
```

- 뿐이다.
- START
    - 기존 곡선의 t=0에서의 미분을 복제한 새 곡선을 만든다.
- END
    - 기존 곡선의 t=1에서의 미분을 복제한 새 곡선을 만든다.

## 5. 그럼 새 곡선은 어디에 붙는가?
- 이 함수는 “붙이는” 기능이 없다.
- 그건 BezierSurface::extend_strip_with_same_derivatives 같은 상위 함수가 한다.
- 즉:
    - 이 함수는 **미분을 유지한 새 곡선 Q** 만 만든다.
    - 그 Q를 어디에 붙일지는 상위 함수가 결정한다.
- 그래서 이 함수는 pure math transform이다.

## 6. 한 문장으로 정리하면
- extend_with_same_derivatives는  
    기존 Bezier 곡선의 시작/끝점에서의 0~p차 미분을 그대로 유지하는
    새로운 Bezier 곡선을 만들어내는 함수이며, 곡선의 shape를 연장하는 것이 아니라  
    **미분 조건** 을 연장하는 것이다.

--

## 1. 원래 곡선 C(t)의 정의
- 원래 곡선:
```math
C(t)=(1-t)^2P_0+2t(1-t)P_1+t^2P_2
```
- 끝점 t=1에서:
- 위치
```math
C(1)=P_2
```

- 먼저 일반식 미분:
- 1차 미분
```math
C'(t)=2(1-t)(P_1-P_0)+2t(P_2-P_1)C'(1)=2(P_2-P_1)
```
- 2차 미분
```math
C''(t)=2(P_2-2P_1+P_0)
```
```math
C''(1)=2(P_2-2P_1+P_0)
```

## 2. 새 곡선 Q(t)의 정의
- 새 곡선:
```math
Q(t)=(1-t)^2Q_0+2t(1-t)Q_1+t^2Q_2
```
- 마찬가지로:
```math
Q(1)=Q_2
```
```math
Q'(1)=2(Q_2-Q_1)
```
```math
Q''(1)=2(Q_2-2Q_1+Q_0)
```

## 3. “끝점에서 미분을 같게 한다”는 조건 세우기
- 강제하는 조건:
```math
Q(1)=C(1),\quad Q'(1)=C'(1),\quad Q''(1)=C''(1)
```
- 즉:
- 0차 미분(위치)
```math
Q_2=P_2
```
- 1차 미분(접선)
```math
2(Q_2-Q_1)=2(P_2-P_1)\Rightarrow Q_2-Q_1=P_2-P_1\Rightarrow Q_1=Q_2-(P_2-P_1)
```
- 여기서 $Q_2=P_2$ 대입하면:
```math
Q_1=P_2-(P_2-P_1)
```
- 2차 미분(곡률 관련)
```math
2(Q_2-2Q_1+Q_0)=2(P_2-2P_1+P_0)\Rightarrow Q_2-2Q_1+Q_0=P_2-2P_1+P_0
```
- 여기서 $Q_2$, $Q_1$를 이미 알고 있으니 $Q_0$ 를 풀 수 있다:
```math
Q_0=(P_2-2P_1+P_0)-Q_2+2Q_1
```
- 다시 $Q_2=P_2$, $Q_1=P_2-(P_2-P_1)$ 넣으면:
- 정리하면:
```math
Q_0=P_2-2(P_2-P_1)+(P_2-2P_1+P_0)4.
``` 
- 결론
- 그래서 나온 식이 바로:
```math
Q_2=P_2
```
```math
Q_1=P_2-(P_2-P_1)
```
```math
Q_0=P_2-2(P_2-P_1)+(P_2-2P_1+P_0)
```
- 즉, **끝점에서 0차, 1차, 2차 미분을 모두 같게 한다** 는 조건을  
    Q₀, Q₁, Q₂에 대해 풀어낸 선형 시스템의 해.
---

## 소스 코드
```rust
pub fn extend_with_same_derivatives(
    &self,
    side: ExtendSide,
    reverse_param: bool,
) -> BezierCurve {
    let p = self.degree;

    // Degree 0 => constant
    if p == 0 {
        return self.clone();
    }

    // Safety: control points must match degree+1
    debug_assert!(
        self.ctrl.len() == p + 1,
        "BezierCurve.ctrl.len() must be degree+1"
    );

    let pw = &self.ctrl;
    let bin = on_pascal_triangle_f64(p);

    // Output control points Qw[0..=p]
    let mut qw = vec![Point4D::zero(); p + 1];

    match side {
        ExtendSide::Start => {
            //   if(rev==YES) Qw[0]=Pw[0]; else Qw[p]=Pw[0];
            //   dgs=-1; cgs=1; sgs=-1;
            //   for i=1..p: ...
            if reverse_param {
                qw[0] = pw[0];
            } else {
                qw[p] = pw[0];
            }

            let mut dgs = -1.0_f64;
            let mut cgs = 1.0_f64;
            let mut sgs = -1.0_f64;

            for i in 1..=p {
                let mut dls = dgs;
                let mut cls = cgs;

                // Dw = Σ_{j=0..i} (dls * C(i,j)) * Pw[j], with alternating sign
                let mut dw = Point4D::zero();
                for j in 0..=i {
                    let fac = dls * bin[i][j];
                    dw.add_scaled(fac, &pw[j]);
                    dls = -dls;
                }

                // Aw = Σ_{j=0..i-1} (cls * C(i,j)) * (rev? Qw[j] : Qw[p-j]), alt sign
                let mut aw = Point4D::zero();
                for j in 0..i {
                    let fac = cls * bin[i][j];
                    if reverse_param {
                        aw.add_scaled(fac, &qw[j]);
                    } else {
                        aw.add_scaled(fac, &qw[p - j]);
                    }
                    cls = -cls;
                }

                if reverse_param {
                    // Dw *= -1; Qw[i] = Dw + Aw;
                    qw[i] = dw.scaled(-1.0).add(aw);
                } else {
                    // Dw *= sgs; Qw[p-i] = Dw + Aw;
                    qw[p - i] = dw.scaled(sgs).add(aw);
                }

                dgs = -dgs;
                cgs = -cgs;
                sgs = -sgs;
            }
        }

        ExtendSide::End => {
            //   if(rev==NO) Qw[0]=Pw[p]; else Qw[p]=Pw[p];
            //   cgs=1; sgs=1;
            //   for i=1..p: ...
            if !reverse_param {
                qw[0] = pw[p];
            } else {
                qw[p] = pw[p];
            }

            let mut cgs = 1.0_f64;
            let mut sgs = 1.0_f64;

            for i in 1..=p {
                let mut dls = 1.0_f64;
                let mut cls = cgs;

                // Dw = Σ_{j=0..i} (dls * C(i,j)) * Pw[p-j], with alternating sign
                let mut dw = Point4D::zero();
                for j in 0..=i {
                    let fac = dls * bin[i][j];
                    dw.add_scaled(fac, &pw[p - j]);
                    dls = -dls;
                }

                // Aw = Σ_{j=0..i-1} (cls * C(i,j)) * (rev? Qw[p-j] : Qw[j]), alt sign
                let mut aw = Point4D::zero();
                for j in 0..i {
                    let fac = cls * bin[i][j];
                    if !reverse_param {
                        aw.add_scaled(fac, &qw[j]);
                    } else {
                        aw.add_scaled(fac, &qw[p - j]);
                    }
                    cls = -cls;
                }

                if reverse_param {
                    // Dw *= sgs; Qw[p-i] = Dw + Aw;
                    qw[p - i] = dw.scaled(sgs).add(aw);
                } else {
                    // Qw[i] = Dw + Aw;
                    qw[i] = dw.add(aw);
                }

                cgs = -cgs;
                sgs = -sgs;
            }
        }
    }

    BezierCurve {
        dim: self.dim,
        degree: p,
        ctrl: qw,
    }
}
```
---
