
## ✅ 수식 및 알고리즘 검토
### 1. distance_closest_point_to
- 선분에 대해 pt3에서 가장 가까운 점을 구하고, 그 거리 반환
- 내부적으로 Segment2D::closest_param_to와 point_at(t) 사용
- 수식적으로 정확함:

$$
d=\| \mathbf{p_{\mathnormal{3}}}-\mathbf{b}\| ,\quad \mathbf{b}=\mathrm{projection\  of\  }\mathbf{p_{\mathnormal{3}}}\mathrm{\  onto\  segment\  }\mathbf{p_{\mathnormal{1}}p_{\mathnormal{2}}}
$$

### 2. square_distance
- 단순한 유클리드 거리 제곱 계산:

$$
d^2=(x_2-x_1)^2+(y_2-y_1)^2
$$

- 정확함

### 3. simplify_recursive
- RDP 알고리즘의 핵심:
- 선분 시작–끝 기준으로 가장 먼 점 찾기
- 그 점이 거리 기준 이상이면 재귀 분할
- 정확한 알고리즘 흐름

### 4. simplify_internal
- 폐합된 폴리라인을 처리하기 위해:
- 시작점에서 가장 먼 점을 기준으로 양쪽 분할
- 각각 단순화 후 병합
- RDP 알고리즘의 폐합 확장 버전으로 적절함

### 5. simplify
- 폐합 여부 판단: points[0] == points[n - 1]
- 개방형일 경우 끝점 반복 제거
- 재귀 시드 설정 후 단순화 수행
- 전체 흐름이 안정적이고 정확함

## ✅ 개선 포인트
- simplify_internal에서 가장 먼 점 기준 분할은 RDP의 폐합 확장으로 괜찮지만,  
    곡선이 매우 비대칭일 경우 성능이 떨어질 수 있음 → 선택적 개선 가능

## ✅ 결론
- 수식적으로 문제 없음
- 알고리즘 흐름 정확함
- 폐합/개방 폴리라인 모두 안정적으로 처리
- RDP 알고리즘을 잘 반영한 구조


