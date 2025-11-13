# QuickHull / Monotone Chain
두 알고리즘 모두 수학적으로 타당하며, 각각 `QuickHull` 과 `Monotone Chain` 방식으로 `2D Convex Hull` 을 계산합니다.  
아래에 수식 기반 알고리즘 설명과 Rust 소스 흐름을 단계별로 정리.

## ✳️ 1. QuickHull 알고리즘 수학적 설명
QuickHull은 `Divide & Conquer` 방식으로 최외곽 점들을 재귀적으로 연결하여 Convex Hull을 구성합니다.
### 📐 수학적 개념
- 기본 선분 선택:
좌표 기준 가장 왼쪽 점 A와 가장 오른쪽 점 B를 선택  
    → 이 두 점은 항상 Convex Hull에 포함됨
- 거리 계산:
점 P가 선분 AB에서 얼마나 떨어져 있는지 계산  

$$
d(P,AB)=\frac{|(B_x-A_x)(A_y-P_y)-(B_y-A_y)(A_x-P_x)|}{\sqrt{(B_x-A_x)^2+(B_y-A_y)^2}}
$$

- 벡터 외적 (삼각형 방향 판별):

$$
\mathrm{cross}(A,B,P)=(B_x-A_x)(P_y-A_y)-(B_y-A_y)(P_x-A_x)
$$

- 양수면 P는 AB의 왼쪽에 있음
- 재귀적 분할:  
    가장 먼 점 F를 기준으로 AF, FB로 나누고, 각 구간에 대해 반복

🧩 Rust 소스 흐름
```rust
let a = min_by_x(v); // 가장 왼쪽
let b = max_by_x(v); // 가장 오른쪽
```

- `side(a, b, p)` → 외적 기반 방향 판별
- `farthest(a, b, vv)` → 가장 먼 점 찾기
- `recurse(left, a, f, hull)` → 왼쪽 영역 재귀
- `recurse(right, f, b, hull)` → 오른쪽 영역 재귀


## ✳️ 2. Monotone Chain 알고리즘 수학적 설명
`Monotone Chain` 은 정렬 후 상/하 방향으로 Hull을 구성하는 방식입니다.
### 📐 수학적 개념
- 정렬:  
    점들을 x 기준 오름차순, tie-break는 y 기준
- 외적 기반 방향 판별:

$$
\mathrm{cross}(A,B,C)=(B_x-A_x)(C_y-A_y)-(B_y-A_y)(C_x-A_x)
$$ 

- 음수면 시계 방향 → Convex 유지
- 스택 기반 구성:
    - lower hull: 왼쪽 → 오른쪽
    - upper hull: 오른쪽 → 왼쪽

### 🧩 Rust 소스 흐름
```rust
v.sort_by(on_is_left_of); // 좌표 정렬
```

- lower와 upper는 각각 스택처럼 동작
- on_cross_vec_2d(a, b, c) >= 0.0 → 시계 방향이 아니면 pop
- 마지막에 lower + upper로 병합

## ✅ 알고리즘 비교 요약

| 항목               | QuickHull                            | Monotone Chain                     |
|--------------------|--------------------------------------|------------------------------------|
| 시간 복잡도        | 평균 O(n log n), 최악 O(n²)          | 항상 O(n log n)                    |
| 방식               | Divide & Conquer                     | 정렬 후 스택 기반 처리             |
| 구현 난이도        | 중간 (재귀적 분할)                   | 낮음 (단순 반복)                   |
| 수학적 핵심        | 외적 + 거리                          | 외적 + 정렬                        |
| 입력 정렬 필요     | ❌ 불필요                             | ✅ 필요                             |

---

QuickHull (분할정복)

아이디어: x-최소/최대점을 잇는 직선으로 점들을 위/아래 집합으로 나눈 뒤,  
각 집합에서 그 선분으로부터 가장 멀리 떨어진 점을 골라 삼각형 바깥쪽(즉, 선분의 바깥쪽)에 있는 점들만 남기고 재귀적으로 진행.

## 단계 그림

- 예시 점들(좌표는 안 중요, 배치점에 중점을 둠):
```
          * p7
    * p3        * p6

* p0                      * p5

        * p2      * p4
                * p1

```
- 좌/우 극점 찾기 (xmin = p0, xmax = p5), 초기 분할선 p0–p5:

```
* p0 ------------------------------------------ * p5
   위쪽 집합(△)                          아래쪽 집합(▽)
             △ p3, p7, p6, p2, p4         ▽ p1

```

- 위쪽 집합에서 p0–p5에 가장 먼 점 p7 선택 → 삼각형 p0–p7–p5 내부 제거:
```
            * p7 (farthest)
   * p3                     * p6

* p0 ------------------------------------------ * p5

        * p2      * p4
                * p1
(삼각형 p0–p7–p5 내부 점들은 버림)
```
- 두 재귀 문제로 분할: [p0, p7]의 바깥쪽 / [p7, p5]의 바깥쪽
- 왼쪽 호출 (p0–p7): 선분에서 가장 먼 점 p3 선택 → 삼각형 p0–p3–p7 내부 제거
- 오른쪽 호출 (p7–p5): 선분에서 가장 먼 점 p6 선택 → 삼각형 p7–p6–p5 내부 제거

- 왼쪽 가지:
```
* p3 (farthest)
|
|
* p0 ---- * p7
```
- 오른쪽 가지:
```
* p7 ---- * p6 (farthest) ---- * p5
```

- 각 가지에서 더 이상 바깥쪽 점이 없을 때 종료. 순서대로 연결하면 볼록껍질:
- Hull 순서 예: p0 → p3 → p7 → p6 → p5 → (… 아래쪽 재귀에서 p1 등) → p0

## 핵심 로직(요약)

- 시작: A = argmin_x, B = argmax_x
- 상/하 집합으로 분할
- 재귀 FindHull(Segment P–Q, Set S_outside):
    - S_outside가 비면 P 추가 후 종료
    - 가장 먼 점 C 찾기
    - 삼각형 P–C–Q 내부 점 제거
    - 왼쪽: FindHull(P–C, S_left)
    - 오른쪽: FindHull(C–Q, S_right)
-최종 연결 순서로 출력
- 복잡도: 평균 O(n log n), 최악 O(n²) (점들이 **적당히** 퍼져 있지 않으면).


##  Monotone Chain (Andrew’s algorithm)
- 아이디어: x(그다음 y)로 정렬 후, 아래 껍질과 위 껍질을 각각 스택처럼 쌓으면서 “오른쪽(시계) 꺾임이면 pop”을 반복.

단계 그림

- 정렬된 점의 가로 배치(왼→오):
```
p0     p2   p1    p4   p3          p5
*------*----*-----*----*-----------*
```

### 1) 아래 껍질(lower) 구축
- 초기 empty → 왼쪽부터 하나씩 push.
- **왼쪽 회전(cross > 0)** 만 유지, **오른쪽/일직선(cross ≤ 0)** 이면 중간 점 pop.
```yaml
Step A: push p0
lower: [p0]

Step B: push p2
lower: [p0, p2]

Step C: 후보 p1
체크 turn(p0, p2, p1):
- 만약 오른쪽으로 꺾임(cross ≤ 0) → p2 pop
→ lower: [p0], 다시 p1 push → [p0, p1]

Step D: 후보 p4
turn(p0, p1, p4) 가 왼쪽이면 keep → [p0, p1, p4]

Step E: 후보 p3
turn(p1, p4, p3) 체크, 오른쪽이면 p4 pop → 다시 turn(p0, p1, p3) …
적절히 pop/push 반복하여 아래 껍질 유지

… 마지막 p5까지 처리 → 예: lower = [p0, p1, p3, p5]
```

### 2) 위 껍질(upper) 구축

오른쪽→왼쪽으로 동일 작업(수평선 위쪽 방향으로 볼록):
```lua
start from p5:
upper: [p5, …]
… 역순으로 pop/push 반복 …
예: upper = [p5, p4, p2, p0]
```

### 3) 합치기
맨 끝점(p0, p5)이 중복되지 않게 lower + upper를 이어서 닫힌 껍질을 만든다.

## 핵심 로직(요약)

- 점들을 (x, y) 오름차순 정렬
- turn(a,b,c) = cross(b - a, c - b)
- 아래 껍질:
    - 각 점 p에 대해 while len>=2 && turn(last2,last1,p) ≤ 0 → pop
    - push p
- 위 껍질:
    - 역순으로 같은 절차
- 합치기(마지막 점 중복 제거)
- 복잡도: O(n log n) (정렬이 지배), 정렬된 상태면 O(n)

### QuickHull vs Monotone Chain 비교 한눈에

- 철학
    - QuickHull: 분할정복(“가장 먼 점”으로 바깥만 남기며 재귀)
    - Monotone Chain: 정렬 + 스택(“왼쪽 회전만 유지”)
- 평균 성능
    - QuickHull: 보통 빠름, 최악 O(n²)
    - Monotone Chain: 안정적 O(n log n)
- 구현 난이도
    - QuickHull: 재귀/거리계산/삼각형 내부 판정
    - Monotone Chain: 구현 간단, 수치적으로 안정적
- 실무 팁
    - 대부분의 일반 데이터: Monotone Chain이 단단하고 빠름
    - 극단적으로 퍼진 데이터(아웃라이어, 매우 얇은 분포): QuickHull도 좋지만 최악 케이스 주의
 
---

  
