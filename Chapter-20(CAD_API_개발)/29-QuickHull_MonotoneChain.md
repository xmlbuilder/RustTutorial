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
| 최악 케이스 예시  

---

