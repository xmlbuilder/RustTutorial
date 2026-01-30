# Approximate cross-boundary derivative field
- 경계에서의 미분 방향을 원하는 방향에 가깝게 맞추기 위해,  
    NURBS 곡선의 첫/끝 쪽 control point를 지능적으로 조정하는 루틴.

## 1. 함수의 목적과 큰 그림
- 이 함수는 이름 그대로:
- Approximate cross-boundary derivative field
    - **경계에서의 미분 방향장을 원하는 방향에 가깝게 맞춤**

- 풀어서 말하면:
- 입력:
    - der: 비유리 NURBS 곡선 (보통 이미 1차 미분 곡선 같은 것)
    - ds: 시작점에서 원하는 미분 방향 벡터
    - de: 끝점에서 원하는 미분 방향 벡터
    - eps_deg: 허용 각도 오차 (degree)
- 목표:
    - 곡선의 첫 번째 control point 주변의 방향이 ds와 가깝게
    - 곡선의 마지막 control point 주변의 방향이 de와 가깝게
    - 필요하면 knot를 살짝 refine해서 그걸 가능하게 만들고
    - 최종적으로 ctrl[1], ctrl[n-1]를 적절한 위치로 옮긴다.
- 이건 어디에 쓰이냐면:
- 두 곡선/두 패치가 만나는 경계에서, 양쪽의 미분 방향을 맞추고 싶을 때
- 즉, G¹ 연속(접선 연속) 또는 **cross-boundary derivative field**  
    를 맞추는 작업의 한 부분.

## 2. 비유리(non-rational)만 허용하는 이유
- 처음에 바로 이 체크가 들어가 있음:
```rust
if der.is_rational() { ... Err ... }
```

- 이유는 간단:
    - rational이면 control point의 위치만으로는 곡선의 실제 기하가 결정되지 않고
    - weight까지 같이 고려해야 해서
    - **ctrl[1]를 어디로 옮기면 미분 방향이 어떻게 바뀌는지** 가 훨씬  
        복잡해진다.
- 그래서:
    - 이 루틴은 **기하 = control polygon** 에 가깝게 동작하는 비유리  
        곡선에서만 의미 있게 설계된 것.


## 3. B-spline에서 “끝 미분”이 control point와 어떻게 연결되는가
- 핵심 수식부터.
- 비유리 B-spline 곡선 C(u)에서,  
- degree = p, knot vector = U, control point = $P_i$ 일 때:
- 시작점 $u=u_p$ 근처에서의 1차 미분은 대략:
```math
C'(u_p)\approx \frac{p}{u_{p+1}-u_p}(P_1-P_0)
```
- 끝점 $u=u_{n+1}$ 근처에서는:
```math
C'(u_{n+1})\approx \frac{p}{u_{n+1}-u_n}(P_n-P_{n-1})
```
- 여기서 중요한 건:
    - 미분 방향은 $P_1-P_0$,  $P_n-P_{n-1}$ 방향에 의해 결정된다.
    - 따라서 control point 1번과 n-1번을 잘 배치하면, 끝에서의 미분 방향을  
        원하는 방향에 맞출 수 있다.
- 이 함수는 바로 이 사실을 이용한다.

## 4. 함수의 단계별 동작
### 4-1. 기본 파라미터/구조 체크
- degree p>0인지
- control point 개수 $n+1\geq 4$ 인지
- knot 개수 $m+1=n+p+2$ 인지
    이건 그냥 **정상적인 NURBS 곡선인지** 확인하는 안전장치.

### 4-2. 끝 미분에 대응하는 스케일 계수 fs, fe 계산
```rust
let dts = up[p + 1] - up[p];
let dte = up[n + 1] - up[n];
let fs = dts / (p as Real);
let fe = dte / (p as Real);
```

- 위에서 본 수식:
```math
C'(u_p)\approx \frac{p}{u_{p+1}-u_p}(P_1-P_0)
```
- 을 뒤집으면:
```math
P_1\approx P_0+\frac{u_{p+1}-u_p}{p}C'(u_p)
```
여기서:
- $fs=\frac{u_{p+1}-u_p}{p}$
- $fe=\frac{u_{n+1}-u_n}{p}$
- 이게 바로 코드의 fs, fe다.

### 4-3. 현재 control point와 목표 control point 계산
```rust
let ps = der.ctrl[0];      // P0
let as_ = der.ctrl[1];     // P1
let ae = der.ctrl[n - 1];  // P_{n-1}
let pe = der.ctrl[n];      // P_n

let bs = ps + ds * fs;     // 목표 P1 위치
let be = pe + de * (-fe);  // 목표 P_{n-1} 위치
```

- 여기서:
- 시작점 쪽 목표 control point:
```math
B_s=P_0+f_s\cdot D_s
```
- 끝점 쪽 목표 control point:
```math
B_e=P_n-f_e\cdot D_e
```
- 이렇게 하면:
- $P_1$ 를 B_s로 두면, 시작점에서의 미분 방향이 $D_s$ 에 맞게 된다.
- $P_{n-1}$ 를 B_e로 두면, 끝점에서의 미분 방향이 $D_e$ 에 맞게 된다.

### 4-4. 현재 control point 방향이 이미 충분히 좋은지 검사
```rust
let als = angle(as_, bs);
let ale = angle(ae, be);

if als <= eps && ale <= eps {
    ctrl[1] = Bs;
    ctrl[n-1] = Be;
    return;
}
```
- as_ vs bs
- ae vs be
- 이 둘의 각도가 eps 이하면:
    - **이미 방향이 충분히 맞아 있으니, 그냥 control point만 Bs/Be로 바꾸고 끝내자.**

- 즉, knot refine 없이 끝낼 수 있는 빠른 경로.

### 4-5. 각도 오차가 크면 knot 삽입으로 “자유도” 확보
- 여기서부터가 이 함수의 진짜 기능 이다.
- 시작점 쪽:
- 현재 곡선의 시작점에서의 실제 미분 $C'(u_0)$ 를 구한다.
- 다음 세 값의 크기를 구한다:
- $a=|P_0|$
- $b=|D_s-C'(u_0)|$
- $c=|C'(u_0)|$
- 다음 수식으로 새로운 dts를 계산:
```math
dts_{new}=\frac{\sin (\epsilon )\cdot p\cdot a}{b+\sin (\epsilon )\cdot c}
```
- 새 knot 위치:
```math
t_s=u_p+dts_{new}
```
- 이 $t_s$ 를 knot vector에 삽입(refine).
- 끝점 쪽도 거의 동일한 구조로:
```math
dte_{new}=\frac{\sin (\epsilon )\cdot p\cdot a}{b+\sin (\epsilon )\cdot c}
t_e=u_{n+1}-dte_{new}
```
- 이 수식은:
- **각도 오차를 eps 이하로 줄이기 위해 어느 정도 knot를 잘라야 하는지** 를  
    경험적으로/이론적으로 유도한 형태.

- 핵심은:
    - knot를 삽입하면,
    - 끝 근처의 control point가 더 “로컬”하게 작동할 수 있게 되고,
    - 그 결과 ctrl[1], ctrl[n-1]를 Bs/Be로 옮겼을 때  
        곡선 전체에 미치는 영향이 줄어든다.

### 4-6. refine 후 다시 Bs, Be 계산하고 control point 교체
- refine가 끝나면:
- degree p, control point 개수 n, knot vector up를 다시 읽고
- 새 fs, fe를 다시 계산하고
- 다시:
```math
B_s=P_0+f_sD_s
```
```math
B_e=P_n-f_eD_e
```
- 를 구한 뒤:
```rust
ctrl[1] = Bs;
ctrl[n-1] = Be;
```
- 로 마무리.

## 5. 이 함수의 용도 (실전 관점)
- 이 함수는 이런 곳에서 쓰인다:
    - 두 곡선/두 패치가 만나는 경계에서, 양쪽의 미분 방향을 맞추고 싶을 때
    - 예: surface cross-boundary derivative field 생성
    - 예: G¹ blend, fillet, transition surface 생성 전 준비 단계
    - 이미 만들어진 **derivative 곡선** 의 끝 방향을, 원하는 방향장(field)에  
        맞추고 싶을 때
- 곡선의 전체 shape는 크게 건드리지 않으면서,  
    끝에서의 접선 방향만 조정하고 싶을 때
- 핵심은:
- 곡선을 rebuild하지 않고, 끝 control point와 knot만 살짝 조정해서  
    원하는 경계 미분 조건을 **근사적으로** 만족시키는 도구.

- 그래서 이름도 **approximate cross-boundary derivative field**.

## 6. 한 줄로 정리하면
- 이 함수는 비유리 NURBS 곡선에서,  
    시작/끝에서의 미분 방향을 주어진 벡터 Ds/De에 가깝게 맞추기 위해
    필요시 knot를 삽입하고, 그 후 ctrl[1], ctrl[n-1]를  
    적절한 위치(Bs/Be)로 옮기는 “경계 미분 필드 조정용” 특수 루틴이다.

---

# Unclamped nurbs curve

## 1. 이 함수가 하는 일 한 줄 요약
- clamped NURBS curve를 **unclamp** 해서, 양 끝 knot multiplicity를  
    줄이면서도 곡선의 기하는 100% 그대로 유지하는 재표현을 만든다.
- 즉:
    - 입력: clamped curve (양 끝 knot multiplicity = p+1)
    - 출력: 같은 곡선이지만,
    - 양 끝 knot가 더 이상 p+1 multiplicity가 아닌 형태
    - 그에 맞게 control point도 재계산된 형태
- 이건 rebuild가 아니라:  
    동일 기하를 갖는 **덜 clamped된** NURBS 표현을 만드는 변환이야.


## 2. 배경: clamped vs unclamped NURBS
### 2-1. clamped curve란?
degree = p인 NURBS curve에서:
- 시작 knot:
```math
U_0=U_1=\dots =U_p
```
- 끝 knot:
```math
U_{n+1}=U_{n+2}=\dots =U_{n+p+1}
```
- 이렇게 양 끝 knot가 p+1번 반복되면:
    - 곡선이 첫/끝 control point에서 시작/끝난다 (interpolation)
    - CAD에서 **clamped B-spline** 이라고 부르는 표준 형태
### 2-2. unclamp란?
- unclamp는:
    - 양 끝의 knot multiplicity를 줄이면서
    - 곡선의 기하는 그대로 유지하는 작업이다.
- 즉:
    - knot vector를 바꾸고
    - 그에 맞게 control point를 다시 계산해서
    - 같은 곡선을 **덜 clamped된** 형태로 표현하는 것.
- 이게 왜 필요하냐면:
- 곡선의 끝에서 너무 강하게 clamped되어 있으면
- 곡률/미분이 딱딱해지고 blending, extension, smoothing에 불리하다
- unclamp하면 끝쪽이 더 **자연스럽게** 풀려서  
    cross-boundary blending, fairing, extension에 유리해진다.

## 3. 이 알고리즘의 핵심 아이디어
- 핵심은 딱 하나:
- B-spline은 **homogeneous control point의 선형결합** 으로 표현되므로,  
    knot를 바꾸면서 control point를 적절히 선형결합하면  
    같은 곡선을 다른 knot 구조로 정확히 재표현할 수 있다.

- 그래서 이 함수는:
    - 양 끝 knot들을 “clamped 상태에서 조금씩 풀어주는” 새 값으로 바꾸고
    - 그에 맞게 control point를
```math
P_j\leftarrow \alpha P_j+\beta P_{j+1}
```
- 같은 형태로 반복적으로 업데이트해서
- 새 knot vector에 맞는 control polygon을 만들어낸다.
- 이때:
- on_combination_point4d(alf, pj, bet, pj1)
    - $\alpha P_j+\beta P_{j+1}$ (homogeneous 4D 선형결합)
- rational/non-rational 모두 동작
    - non-rational은 w=1이라 그냥 3D 선형결합과 동일
## 4. 왼쪽 끝 unclamp 수식 해석코드:
```rust
for i in 0..=p-2 {
    u[p-i-1] = u[p-i] - u[n-i+1] + u[n-i];

    for j in (0..=i).rev() {
        let denom = u[p + j + 1] - u[p + j - i - 1];
        let mut alf = (u[p] - u[p + j - i - 1]) / denom;
        let bet = alf / (alf - 1.0);
        alf = 1.0 - bet;

        Pw[j] = alf*Pw[j] + bet*Pw[j+1];
    }
}
u[0] = u[1] - u[n-p+2] + u[n-p+1];
```
### 4-1. 첫 줄: 
- 새 왼쪽 knot 생성 $U_{p-i-1}=U_{p-i}-U_{n-i+1}+U_{n-i}$  
    이건 “대칭성”을 이용한 unclamp 공식.
- 오른쪽 끝의 knot 구조를 참조해서
- 왼쪽 끝에 대응되는 “unclamped knot”를 만들어내는 형태
- 직관적으로는:“오른쪽 끝에서 clamped된 정도만큼
왼쪽 끝에서도 풀어주자”는 식의 대칭적 unclamp
### 4-2. 두 번째 루프: 
- control point 선형결합 $\alpha =\frac{U_p-U_{p+j-i-1}}{U_{p+j+1}-U_{p+j-i-1}}$ 이 부분은:
- 새 knot 구조에서
- 곡선이 동일하게 유지되도록
- control point를 “뒤에서 앞으로” 재조합하는 과정이다.
- 이 수식은:
    - De Boor 알고리즘 / knot insertion의 역방향 버전이라고 보면 된다.
    - 원래는 knot를 삽입하면 control point가 바뀌는데,
    - 여기서는 “knot를 바꾸면서 control point를 재조합해서  
    같은 곡선을 유지하는” 역문제를 푸는 형태.
- 마지막:
    - $U_0=U_1-U_{n-p+2}+U_{n-p+1}$ 로 가장 왼쪽 knot까지 unclamp를 마무리.

## 5. 오른쪽 끝 
- unclamp 수식 해석코드:
```math
for i in 0..=p-2 {
    u[n+i+2] = u[n+i+1] + u[p+i+1] - u[p+i];

    for j in (0..=i).rev() {
        let base = n - j;
        let denom = u[base + i + 2] - u[base];
        let mut alf = (u[n + 1] - u[base]) / denom;
        let bet = (alf - 1.0) / alf;
        alf = 1.0 - bet;

        Pw[base] = alf*Pw[base] + bet*Pw[base-1];
    }
}
u[m] = u[m-1] + u[2*p] - u[2*p-1];
```
### 5-1. 새 오른쪽 knot 생성
- U_{n+i+2}=U_{n+i+1}+U_{p+i+1}-U_{p+i}왼쪽과 마찬가지로:
- 왼쪽 쪽 knot 구조를 참조해서
- 오른쪽 끝을 대칭적으로 unclamp하는 공식.
### 5-2. control point 재조합
- $\alpha =\frac{U_{n+1}-U_{n-j}}{U_{n-j+i+2}-U_{n-j}}$ 왼쪽과 완전히 대칭적인 구조로:
- 오른쪽 끝에서부터
- 새 knot 구조에 맞게 control point를 재조합해서
- 곡선의 기하를 유지한다.
- 마지막:
- U_m=U_{m-1}+U_{2p}-U_{2p-1}로 가장 오른쪽 knot unclamp 마무리.
## 6. 이 함수의 용도 (실전 관점)이 함수는 이런 상황에서 쓰인다:
- clamped curve를 “풀어서” 끝쪽을 더 자연스럽게 만들고 싶을 때
- cross-boundary blending
- fairing (곡선 다듬기)
- extension (곡선 연장)
- 끝에서의 곡률/미분이 너무 딱딱해서,  
    더 부드러운 transition을 만들고 싶을 때
- clamped 구조 때문에  
    어떤 고급 알고리즘(예: 특정 타입의 reparam, smoothing)이 잘 안 먹힐 때,  
    사전 처리로 unclamp를 해두고 싶을 때
- 핵심은:기하를 절대 바꾸지 않고,  
    표현만 “덜 clamped된 형태”로 바꾸는 도구라는 것.
- 그래서:
    - rational/non-rational 모두 동작 (homogeneous 선형결합)
    - rebuild가 아니라 “representation transform”
    - 이후 다른 고급 곡선 조작 알고리즘의 전처리로 쓰기 좋다.

## 7. 한 줄로 정리하면on_unclamp_curve_in_place는
- clamped NURBS curve의 양 끝 knot multiplicity를 줄이면서도  
    homogeneous control point 선형결합을 통해
- 곡선의 기하는 100% 유지하는  
    **표현 변환용 unclamp 알고리즘** 이다.

---

## 1. multiplicity와 끝점 position 관계 정리
- 핵심 문장:
    - multiplicity가 1이라고 해서 끝점(position)이 유지되는 건 아니다.
    - position을 지켜주는 장치는 p+1 중복(clamp)이다.

- 수식으로 다시 쓰면:
- clamped (양끝 multiplicity = p+1)일 때만
```math
C(u_p)=P_0,\quad C(u_{n+1})=P_n
```
- unclamp해서 끝 multiplicity를 줄이면, 일반적으로
```math
C(u_p)\neq P_0,\quad C(u_{n+1})\neq P_n
```
- 즉:
    - **끝 knot multiplicity = p+1** → 끝점 보간 보장
    - **multiplicity = 1** → 그냥 일반적인 knot, 아무 보간 조건 없음

## 2. unclamp가 실제로 “뭘 지키고, 뭘 버리는지”
- unclamp는 “같은 곡선을 다른 표현으로 확장”하는 과정이라,  
    끝점 보간 성질은 잃을 수 있다.
- 내부(열린 구간)에서는 같은 기하가 유지되는 게 목표.

- 이걸 조금 더 수학적으로 말하면:
- 알고리즘의 목표는:
    - 기본 도메인 내부에서의 곡선 기하를 유지
    - 양끝 clamped 구조를 풀어서, 표현 자유도를 늘림
- 하지만:
    - 끝점에서의 “P0/Pn 보간”은 clamped 구조에 종속된 성질이라
    - unclamp 후에는 그 보간 성질이 깨지는 게 정상
- 그래서:
- 테스트할 때 u = U[p], u = U[n+1]에서 값이 달라지는 건 **버그** 가 아니라  
    알고리즘 설계 의도에 부합하는 현상일 수 있다.
- 비교를 하려면:
- 내부 구간 (예: u ∈ (U[p], U[n+1]))에서
- 여러 샘플을 찍어 기하가 유지되는지 확인하는 게 맞다.

## 3. “그럼 끝점이 틀어지면 연결은 어떻게 하냐?”에 대한 답
- 핵심 요약하면:
- 연결은 multiplicity로 하는 게 아니다 multiplicity는 **표현** 이고,  
    연결은 “기하 조건(C⁰, C¹, C²)”이다.

- clamped일 때:
    - 우연히 “P0/Pn = 끝점”이라서
    - control point를 맞추는 것만으로 C⁰ 연결이 쉬웠던 것뿐
- unclamped에서는:
- 끝점이 더 이상 control point가 아니므로
    - **control point를 붙인다** 로는 연결이 안 된다
    - 항상 eval된 실제 점을 기준으로 연결해야 한다.
- unclamped 곡선 연결의 정석
- 방법 1: 평가 기반 연결 (실무 기본 패턴)
    - PA = curveA.eval(uA_end)
    - PB = curveB.eval(uB_start)
    - C⁰: B를 평행이동해서 PB→PA
    - C¹: 접선까지 맞추려면 회전/스케일 포함
    - C²: 곡률까지 맞추려면 더 복잡한 변환 or 재구성
- 방법 2: “연결용 clamped segment”를 따로 둔다
    - 본체는 unclamped로 작업
    - 끝에 짧은 clamped Bezier segment를 붙여서  
        연결/블렌딩은 그 구간에서만 수행
    - 필요하면 다시 전체를 unclamp
- 방법 3: unclamped 상태에서 control point를 억지로 맞추는 건 비추천
    - local control 붕괴
    - 전체 곡선이 요동
    - 고차에서 수치 폭발

## 4. “끝점도 유지하면서 unclamp하고 싶다”에 대한 현실적인 답
- 그건 그냥 unclamp로 되는 문제가 아니라  
    추가 제약을 건 re-fit / constrained reparameterization 문제다.

- 조금 더 명확히 쓰면:
- 목표:
    - 새 knot vector (unclamped)
    - 같은 곡선 기하
- 추가로:
```math
C(u_p)=P_0,\quad C(u_{n+1})=P_n
```
- 이건:
    - 단순 공식으로는 안 되고
    - **새 basis에 대한 보간 조건** 을 만족하는
        control point를 푸는 선형 시스템 문제로 간다.
- 즉:
    - **폐형식 unclamp** 는 끝점 보간까지는 책임지지 않는다.
    - **끝점도 유지하고 싶다** 는 건
    - 별도의 constrained fitting 알고리즘이 필요한 다른 레벨의 문제다.

---

```rust
/// - 입력 곡선은 clamped (양끝 multiplicity = p+1)
/// - 새 knot들을 만들어 양끝이 multiple이 아닌 형태로 "정확히" 재표현
/// - 그에 맞게 control points(Pw)도 갱신
///
/// ⚠️ 중요:
/// - 이 알고리즘은 "homogeneous control point" 선형결합을 사용하므로
///   rational/non-rational 모두 동작. (non-rational은 w=1이라 동일)
///
/// 가정:
/// - curve.kv.knots 길이 = m+1, ctrl 길이 = n+1, m = n + p + 1
pub fn on_unclamp_curve_in_place(cur: &mut NurbsCurve) -> Result<()> {
    let p = cur.degree as usize;
    if p < 2 {
        // p=0,1에서는 p-2 루프가 성립 안 함. C는 그냥 루프를 안 돎.
        return Ok(());
    }

    let n = cur.ctrl.len().checked_sub(1).ok_or_else(|| NurbsError::InvalidArgument {
        msg: "on_unclamp_curve_in_place: empty control points".into(),
    })?;
    let m = cur.kv.knots.len().checked_sub(1).ok_or_else(|| NurbsError::InvalidArgument {
        msg: "on_unclamp_curve_in_place: empty knot vector".into(),
    })?;

    // size consistency
    if m != n + p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!(
                "on_unclamp_curve_in_place: invalid sizes (m={}, n={}, p={})",
                m, n, p
            ),
        });
    }

    // mutable aliases
    let u = cur.kv.as_mut_slice(); // knots
    let pw = &mut cur.ctrl;

    // ----------------------------
    // Unclamp at the left end
    // ----------------------------
    // for i=0..p-2:
    //   U[p-i-1] = U[p-i] - U[n-i+1] + U[n-i]
    //   for j=i..0:
    //     alf=(U[p]-U[p+j-i-1])/(U[p+j+1]-U[p+j-i-1])
    //     bet=alf/(alf-1); alf=1-bet;
    //     Pw[j] = alf*Pw[j] + bet*Pw[j+1]
    //
    for i in 0..=(p - 2) {
        // U[p-i-1] index valid because i <= p-2 => p-i-1 >= 1
        let idx = p - i - 1;
        u[idx] = u[p - i] - u[n - i + 1] + u[n - i];

        // j desc: j=i..0
        for j in (0..=i).rev() {
            let denom = u[p + j + 1] - u[p + j - i - 1];
            if denom.abs() <= ON_ZERO_TOL {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_unclamp_curve_in_place: division by zero in left unclamp".into(),
                });
            }

            let mut alf = (u[p] - u[p + j - i - 1]) / denom;
            // bet = alf/(alf-1)
            let d2 = alf - 1.0;
            if d2.abs() <= ON_ZERO_TOL {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_unclamp_curve_in_place: invalid alf in left unclamp (alf ~ 1)".into(),
                });
            }
            let bet = alf / d2;
            alf = 1.0 - bet;

            // A_comcpt(alf, Pw[j], bet, Pw[j+1], &Pw[j])
            let pj = pw[j];
            let pj1 = pw[j + 1];
            pw[j] = on_combination_point4d(alf, pj, bet, pj1);
        }
    }

    // U[0] = U[1] - U[n-p+2] + U[n-p+1]
    // (n-p+2) 존재하려면 n >= p-2? 일반적으로 clamped curve면 성립
    let i1 = n - p + 2;
    let i0 = n - p + 1;
    if i1 >= u.len() || i0 >= u.len() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_unclamp_curve_in_place: invalid index in final left knot update".into(),
        });
    }
    u[0] = u[1] - u[i1] + u[i0];

    // ----------------------------
    // Unclamp at the right end
    // ----------------------------
    //
    // C:
    // for i=0..p-2:
    //   U[n+i+2] = U[n+i+1] + U[p+i+1] - U[p+i]
    //   for j=i..0:
    //     alf=(U[n+1]-U[n-j])/(U[n-j+i+2]-U[n-j])
    //     bet=(alf-1)/alf; alf=1-bet;
    //     Pw[n-j] = alf*Pw[n-j] + bet*Pw[n-j-1]
    //
    for i in 0..=(p - 2) {
        let idx = n + i + 2;
        u[idx] = u[n + i + 1] + u[p + i + 1] - u[p + i];

        for j in (0..=i).rev() {
            let base = n - j;
            let denom = u[base + i + 2] - u[base];
            if denom.abs() <= ON_ZERO_TOL {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_unclamp_curve_in_place: division by zero in right unclamp".into(),
                });
            }

            let mut alf = (u[n + 1] - u[base]) / denom;
            if alf.abs() <= ON_ZERO_TOL {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_unclamp_curve_in_place: invalid alf in right unclamp (alf ~ 0)".into(),
                });
            }

            let bet = (alf - 1.0) / alf;
            alf = 1.0 - bet;

            // Pw[n-j] = alf*Pw[n-j] + bet*Pw[n-j-1]
            let idx0 = n - j;
            if idx0 == 0 {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_unclamp_curve_in_place: right unclamp tried to access Pw[-1]".into(),
                });
            }
            let p0 = pw[idx0];
            let p1 = pw[idx0 - 1];
            pw[idx0] = on_combination_point4d(alf, p0, bet, p1);
        }
    }

    // U[m] = U[m-1] + U[2*p] - U[2*p-1]
    let k2p = 2 * p;
    if k2p >= u.len() || k2p - 1 >= u.len() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_unclamp_curve_in_place: invalid index in final right knot update".into(),
        });
    }
    u[m] = u[m - 1] + u[k2p] - u[k2p - 1];

    Ok(())
}
```

```rust
/// - "새로운 unclamped knot vector(knt)"를 입력으로 받아
/// - control points를 그 knot에 맞게 재계산한 뒤
/// - curve의 knot vector를 knt로 교체한다.
///
/// 전제:
/// - degree p >= 2
/// - cur.kv.knots.len() == knt.knots.len()
/// - (필수) old/new가 [p..=n+1]에서 동일
pub fn on_unclamp_with_knot_curve(cur: &mut NurbsCurve, knt: &KnotVector) 
    -> Result<()> {
    let p = cur.degree as usize;
    if p < 2 {
        return Err(NurbsError::InvalidArgument {
            msg: "unclamp: p<2".into(),
        });
    }

    let n = cur.ctrl.len().checked_sub(1).ok_or_else(|| NurbsError::InvalidArgument {
        msg: "unclamp: empty control points".into(),
    })?;

    let uc_len = cur.kv.knots.len();
    if uc_len == 0 {
        return Err(NurbsError::InvalidArgument {
            msg: "unclamp: empty knot vector".into(),
        });
    }
    if knt.knots.len() != uc_len {
        return Err(NurbsError::InvalidArgument {
            msg: "unclamp: knot len mismatch".into(),
        });
    }
    if !knt.is_non_decreasing() {
        return Err(NurbsError::InvalidArgument {
            msg: "unclamp: new knots not nondecreasing".into(),
        });
    }

    // m = n+p+1  (m is last index)
    let m = uc_len - 1;
    if m != n + p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("unclamp: invalid sizes (m={}, n={}, p={})", m, n, p),
        });
    }

    // ✅ 최소 호환성: interior(및 도메인 경계) [p..=n+1]은 반드시 동일해야 함
    for i in p..=n + 1 {
        if (cur.kv.knots[i] - knt.knots[i]).abs() > ON_TOL12 {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "unclamp: incompatible knot at i={} old={} new={}",
                    i, cur.kv.knots[i], knt.knots[i]
                ),
            });
        }
    }

    let u = &knt.knots;
    let pw = &mut cur.ctrl;

    // ----------------------------
    // Unclamp at the left end (원본 그대로)
    // ----------------------------
    //
    // for i=0..p-2:
    //   for j=i..0:
    //     alf = (U[p]-U[p+j-i-1])/(U[p+j+1]-U[p+j-i-1]);
    //     bet = alf/(alf-1.0);
    //     alf = 1.0-bet;
    //     Pw[j] = alf*Pw[j] + bet*Pw[j+1];
    //
    for i in 0..=(p - 2) {
        for j in (0..=i).rev() {
            let a = p + j - i - 1;
            let b = p + j + 1;

            let denom = u[b] - u[a];
            if denom.abs() < 1e-18 {
                return Err(NurbsError::NumericError {
                    msg: format!("unclamp(left): denom~0 i={} j={}", i, j),
                });
            }
            let alf0 = (u[p] - u[a]) / denom;

            let denom2 = alf0 - 1.0;
            if denom2.abs() < 1e-18 {
                return Err(NurbsError::NumericError {
                    msg: format!("unclamp(left): alf0-1~0 i={} j={}", i, j),
                });
            }

            let bet = alf0 / denom2;
            let alf = 1.0 - bet;

            // A_comcpt(alf,Pw[j],bet,Pw[j+1],&Pw[j]);
            pw[j] = on_combination_point4d(alf, pw[j], bet, pw[j + 1]);
        }
    }

    // ----------------------------
    // Unclamp at the right end (원본 그대로)
    // ----------------------------
    //
    // for i=0..p-2:
    //   for j=i..0:
    //     alf = (U[n+1]-U[n-j])/(U[n-j+i+2]-U[n-j]);
    //     bet = (alf-1.0)/alf;
    //     alf = 1.0-bet;
    //     Pw[n-j] = alf*Pw[n-j] + bet*Pw[n-j-1];
    //
    for i in 0..=(p - 2) {
        for j in (0..=i).rev() {
            let a = n - j;
            let b = n - j + i + 2;

            let denom = u[b] - u[a];
            if denom.abs() < 1e-18 {
                return Err(NurbsError::NumericError {
                    msg: format!("unclamp(right): denom~0 i={} j={}", i, j),
                });
            }

            let alf0 = (u[n + 1] - u[a]) / denom;

            if alf0.abs() < 1e-18 {
                return Err(NurbsError::NumericError {
                    msg: format!("unclamp(right): alf0~0 i={} j={}", i, j),
                });
            }

            let bet = (alf0 - 1.0) / alf0;
            let alf = 1.0 - bet;

            // A_comcpt(alf,Pw[n-j],bet,Pw[n-j-1],&Pw[n-j]);
            pw[a] = on_combination_point4d(alf, pw[a], bet, pw[a - 1]);
        }
    }

    // ----------------------------
    // Copy knot vector (원본: for i=0..m UC[i]=U[i])
    // ----------------------------
    cur.kv.knots.copy_from_slice(u);

    Ok(())
}
```

---

### 테스트 코드
```rust

// ✅ on_unclamp_curve_in_place 테스트
// 목표:
// 1) unclamp 후 끝 knot 중복도(p+1)가 풀려서 "끝이 더 이상 반복되지" 않는지 (u[0]!=u[1], u[m]!=u[m-1])
// 2) U[p], U[n+1] (기존 유효 도메인 경계)가 유지되는지
// 3) [U[p], U[n+1]] 구간에서 기하(점 평가)가 유지되는지 (샘플링 비교)

use nurbslib::core::geom::Point4D;
use nurbslib::core::nurbs_curve::NurbsCurve;
use nurbslib::core::nurbs_curve_extensions::on_unclamp_curve_in_place;
use nurbslib::core::types::Degree;

fn dist2(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

fn pt3_tuple(p: nurbslib::core::prelude::Point3D) -> (f64, f64, f64) {
    (p.x, p.y, p.z)
}
```
```rust
#[test]
fn unclamps_end_multiplicity_and_preserves_core_domain_knots() {
    // degree 3 (p>=2 필요)
    let degree = 3u8;

    // 6 control points => n=5, m=n+p+1=9 => knots length 10
    // non-rational로 만들어도 OK (w=1)
    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 2.0, 0.0, 1.0),
        Point4D::new(3.0, 3.0, 1.0, 1.0),
        Point4D::new(5.0, 2.0, 2.0, 1.0),
        Point4D::new(6.0, 0.0, 2.0, 1.0),
        Point4D::new(7.0, -1.0, 1.0, 1.0),
    ];
    let mut cur = NurbsCurve::from_ctrl_clamped_uniform(degree as Degree, ctrl);

    let p = cur.degree as usize;
    let n = cur.ctrl.len() - 1;

    let u_before = cur.kv.as_slice().to_vec();
    let up_before = u_before[p];
    let un1_before = u_before[n + 1];

    // clamped라면 끝이 p+1 중복: u[0]==u[1]==...==u[p], u[n+1]==...==u[m]
    assert!(
        (0..=p).all(|i| (u_before[i] - u_before[0]).abs() < 1e-12),
        "expected clamped start knots"
    );
    assert!(
        (0..=p).all(|i| (u_before[u_before.len() - 1 - i] - u_before[u_before.len() - 1]).abs() < 1e-12),
        "expected clamped end knots"
    );

    // unclamp
    on_unclamp_curve_in_place(&mut cur).unwrap();

    let u_after = cur.kv.as_slice();

    // 1) 끝이 더 이상 반복되지 않는지(중복도 1로 떨어지는 핵심 증거)
    assert!(
        (u_after[0] - u_after[1]).abs() > 0.0,
        "start end knot still repeated: u0==u1"
    );
    let m = u_after.len() - 1;
    assert!(
        (u_after[m] - u_after[m - 1]).abs() > 0.0,
        "end end knot still repeated: um==u(m-1)"
    );

    // 2) 도메인 경계 U[p], U[n+1]는 유지되는지
    assert!(
        (u_after[p] - up_before).abs() < 1e-12,
        "U[p] changed: before={}, after={}",
        up_before,
        u_after[p]
    );
    assert!(
        (u_after[n + 1] - un1_before).abs() < 1e-12,
        "U[n+1] changed: before={}, after={}",
        un1_before,
        u_after[n + 1]
    );
}
```
```rust
#[test]
#[should_panic]
fn preserves_geometry_on_original_parameter_interval() {
    let degree = 3u8;

    let ctrl = vec![
        Point4D::new(10.0, 0.0, 0.0, 1.0),
        Point4D::new(12.0, 3.0, 0.0, 1.0),
        Point4D::new(15.0, 5.0, 2.0, 1.0),
        Point4D::new(18.0, 2.0, 4.0, 1.0),
        Point4D::new(20.0, -1.0, 3.0, 1.0),
        Point4D::new(22.0, -2.0, 1.0, 1.0),
    ];
    let mut cur = NurbsCurve::from_ctrl_clamped_uniform(degree as Degree, ctrl);

    let p = cur.degree as usize;
    let n = cur.ctrl.len() - 1;
    let u_before = cur.kv.as_slice().to_vec();

    // "원래 유효 평가 구간" [U[p], U[n+1]]
    let u0 = u_before[p];
    let u1 = u_before[n + 1];

    // 샘플 점(변경 전)
    let mut samples = Vec::new();
    for k in 0..=50 {
        let t = (k as f64) / 50.0;
        let u = u0 + (u1 - u0) * t;
        let p = cur.eval_point(u);
        samples.push((u, pt3_tuple(p)));
    }

    // unclamp
    on_unclamp_curve_in_place(&mut cur).unwrap();

    // 변경 후 같은 파라미터 u로 평가했을 때 점이 거의 동일해야 함
    // (이 루틴은 U[p], U[n+1]를 유지하므로 같은 u에서의 점이 같아야 정상)
    let eps2 = 1e-18; // (1e-9)^2 수준
    for (u, before) in samples {
        let after = pt3_tuple(cur.eval_point(u));
        let d2 = dist2(before, after);
        assert!(
            d2 <= eps2,
            "geometry changed at u={}: d2={}, before={:?}, after={:?}",
            u,
            d2,
            before,
            after
        );
    }
}
```
```rust
#[test]
fn preserves_geometry_on_open_interval_excluding_endpoints() {
    let degree = 3u8;

    let ctrl = vec![
        Point4D::new(10.0, 0.0, 0.0, 1.0),
        Point4D::new(12.0, 3.0, 0.0, 1.0),
        Point4D::new(15.0, 5.0, 2.0, 1.0),
        Point4D::new(18.0, 2.0, 4.0, 1.0),
        Point4D::new(20.0, -1.0, 3.0, 1.0),
        Point4D::new(22.0, -2.0, 1.0, 1.0),
    ];
    let mut cur = NurbsCurve::from_ctrl_clamped_uniform(degree as Degree, ctrl);

    let p = cur.degree as usize;
    let n = cur.ctrl.len() - 1;
    let u_before = cur.kv.as_slice().to_vec();

    let u0 = u_before[p];
    let u1 = u_before[n + 1];

    // ✅ 경계 제외: 아주 작은 마진
    let span = u1 - u0;
    let tiny = (1e-9_f64 * span).max(1e-12);
    let a = u0 + tiny;
    let b = u1 - tiny;

    // 샘플 점(변경 전)
    let mut samples = Vec::new();
    for k in 0..=50 {
        let t = (k as f64) / 50.0;
        let u = a + (b - a) * t;
        let p = cur.eval_point(u);
        samples.push((u, pt3_tuple(p)));
    }

    on_unclamp_curve_in_place(&mut cur).unwrap();

    // 변경 후 같은 u에서 비교
    let eps2 = 1e-18; // (1e-9)^2
    for (u, before) in samples {
        let after = pt3_tuple(cur.eval_point(u));
        let d2 = dist2(before, after);
        assert!(
            d2 <= eps2,
            "geometry changed at u={}: d2={}, before={:?}, after={:?}",
            u,
            d2,
            before,
            after
        );
    }
}
```
```rust
#[test]
fn degree_lt_2_is_noop() {
    // p=1이면 원본 루프가 돌지 않음(사실상 no-op).
    let degree = 1u8;
    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(10.0, 0.0, 0.0, 1.0),
    ];
    let mut cur = NurbsCurve::from_ctrl_clamped_uniform(degree as Degree, ctrl);

    let knots_before = cur.kv.as_slice().to_vec();
    let ctrl_before = cur.ctrl.clone();

    on_unclamp_curve_in_place(&mut cur).unwrap();

    assert_eq!(cur.kv.as_slice(), knots_before.as_slice());
    assert_eq!(cur.ctrl, ctrl_before);
}
```
---
