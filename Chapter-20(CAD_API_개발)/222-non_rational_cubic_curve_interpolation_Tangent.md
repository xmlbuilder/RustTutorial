# C¹ non-rational cubic curve interpolation Tangent 구하기
## 🎯 전체 맥락
- 이 함수는 C¹ non-rational cubic curve interpolation을 한다.
- 즉:
    - 주어진 점들 $P_0,P_1,...,P_k$ 를 정확히 지나가고
    - 각 점에서의 접선 방향 $T_i$ 를 계산해서
    - cubic Bézier segment들을 이어붙여
    - 전체가 C¹ 연속이 되도록 만든다.
- 여기서 핵심은 T[i] (각 점의 tangent) 를 어떻게 정하는 방식.
- 그 방법이 바로:
    - Bessel 방식
    - Akima 방식
- 둘 다 **자연스럽고 부드러운** 접선을 만드는 기법이지만 철학이 완전히 다르다.

## 🟦 1) TangentMode::Bessel — “곡선 길이 기반의 평균 기울기 방식”
- ✔ 핵심 아이디어
  - Bessel 방식은 곡선의 chord length(현 길이) 를 이용해  
      양쪽 방향 벡터를 “길이 비율”로 섞어서 접선을 만든다.
  - 즉:
      - $S[i+1]=P_i-P_{i-1}$

  - 이 두 벡터를 적절한 비율로 섞어서 T[i]를 만든다.
- ✔ 왜 chord length를 쓰는가
  - 점 간 간격이 일정하지 않을 때:
      - 가까운 점은 영향이 적고
      - 먼 점은 영향이 크도록
      - 자연스럽게 가중치를 주기 위해서다.
- ✔ 코드에서의 핵심 부분
```rust
let dui = u[i] - u[i - 1];
let dui1 = u[i + 1] - u[i];
let denom = dui + dui1;
let alf = dui / denom;
let oma = 1.0 - alf;

oma /= dui;
alf /= dui1;

t[i] = oma*S[i+1] + alf*S[i+2];
```

- 여기서:
    - u[]는 chord length parameterization
    - dui, dui1은 양쪽 segment 길이
    - 길이가 긴 쪽 segment가 더 큰 영향력을 갖는다
    - 결과적으로 곡선이 자연스럽게 이어지는 접선이 나온다
- ✔ 특징 요약

| 항목               | 설명                                   |
|--------------------|----------------------------------------|
| 기본 철학          | chord-length 기반 평균 기울기          |
| 부드러움           | 매우 부드러운 C1 접선                  |
| 데이터 간격 균일   | 매우 잘 동작                           |
| 데이터 간격 불균일 | 약간 민감할 수 있음                    |
| 급격한 변화(sharp) | overshoot 가능                         |

- 즉, 일반적인 smooth curve interpolation에 적합하다.

## 🟧 2) TangentMode::Akima — “기울기 변화량 기반의 로버스트 방식”
- Akima 방식은 급격한 변화(outlier, sharp turn) 에 강한 방식이다.
- ✔ 핵심 아이디어
  - Akima는 **기울기 변화량** 을 이용해  
      양쪽 방향 벡터의 신뢰도(weight) 를 계산한다.
- 코드에서:
```rust
let a = |S[i]   × S[i+1]|   // 앞쪽 기울기 변화량
let b = |S[i+2] × S[i+3]|   // 뒤쪽 기울기 변화량
let alf = a / (a + b)
let oma = 1 - alf
T[i] = oma*S[i+1] + alf*S[i+2]
```

- ✔ 왜 cross product(벡터의 외적)를 쓰는가?
- 두 벡터의 외적 크기:
```
|A × B| = |A| * |B| * sin(theta)
```
- 여기서 theta는 두 벡터 사이의 각도.
- 즉:
    - 두 segment가 일직선에 가까우면 → sin(theta) ≈ 0 → 외적 ≈ 0
    - 두 segment가 급격히 꺾이면 → sin(theta) 커짐 → 외적 커짐

- 여기서:
    - S[i] × S[i+1] 는 두 segment의 **방향 변화량**
    - 즉, 코너가 얼마나 꺾였는지를 나타낸다
    - 변화량이 큰 쪽은 **신뢰도가 낮다**
    - 변화량이 작은 쪽은 **더 부드럽다 → 더 신뢰할 수 있다**
- 그래서:
    - 급격히 꺾인 부분에서는 더 안정적인 쪽 segment가 더 큰 weight를 갖는다
    - 결과적으로 overshoot 없이 안정적인 tangent가 나온다
- ✔ 특징 요약

| 항목               | 설명                                   |
|--------------------|----------------------------------------|
| 기본 철학          | 기울기 변화량 기반 가중치              |
| 부드러움           | 부드러움 + 안정성                      |
| 데이터 불규칙      | 매우 강함                              |
| 급격한 변화(sharp) | overshoot 거의 없음                    |
| noisy 데이터       | 매우 강함                              |

- 즉, sharp turn, noisy data, CAD에서의 안정성이 중요할 때 적합하다.

## 🟩 Bessel vs Akima 비교 요약

| 항목               | Bessel                          | Akima                           |
|--------------------|----------------------------------|----------------------------------|
| 기본 철학          | chord-length 기반 평균           | 기울기 변화량 기반 가중치        |
| 부드러움           | 매우 부드러움                    | 부드러움 + 안정성                |
| 급격한 변화 처리   | overshoot 가능                   | overshoot 거의 없음              |
| 데이터 불규칙성    | 약함                             | 매우 강함                        |
| noisy 데이터       | 약함                             | 매우 강함                        |
| CAD 안정성         | 보통                             | 매우 좋음                        |
| 적용 상황          | 일반 smooth interpolation        | sharp turn, noisy, CAD 안정성    |


## 🎯 결론: 어떤 걸 언제 쓰면 좋을까?
- Bessel
    - 점들이 비교적 고르게 분포
    - 부드러운 곡선이 목표
    - overshoot 걱정 없음
    - 일반적인 smooth interpolation
- Akima
    - 데이터가 불규칙하거나 noisy
    - sharp turn이 존재
    - overshoot 방지 필요
    - CAD/CAM에서 안정성이 중요할 때 Akima가 더 안전하고 실용적인 경우가 많다.

---
