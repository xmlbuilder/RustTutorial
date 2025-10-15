# 🧠 대용량 Polyline 단순화 전략

## ✅ 목표
- 입력: 순서대로 정렬된 1억 개의 2D 점
- 출력: 1천만~2천만 개 수준으로 줄인 궤적
- 조건: 궤적의 전체 흐름 유지, 지도에서 렌더링 가능, 실시간 또는 스트리밍 처리

## 🚀 추천 알고리즘
### 1. Streaming Ramer–Douglas–Peucker (RDP)
- 기본 RDP는 재귀적으로 가장 멀리 떨어진 점을 선택하지만, 전체 데이터를 메모리에 올려야 함
- Streaming RDP는 한 번의 패스로 처리하며, 메모리 사용량이 적고 속도가 빠름
- 특징:
- 시작점 기준으로 일정 거리 이상 벗어나는 점만 유지
- 순서 보존
- 실시간 처리 가능
### 2. Visvalingam–Whyatt (VW) 알고리즘
- 방식: 삼각형 면적이 작은 점부터 제거
- 장점: 시각적으로 덜 중요한 점부터 제거
- 단점: 순서 보존이 약함, 전체 데이터 필요
### 3. Sliding Window + Adaptive Threshold
- 방식: 일정 구간(window)마다 궤적의 방향 변화량 또는 곡률을 계산해 중요 점 유지
- 장점: 스트리밍 처리 가능, GPU 병렬화 가능
- 적용: 지도 렌더링용 실시간 최적화에 적합

## 🔧 Rust에서 구현 팁
- geo::Line, geo::Coord로 거리/각도 계산
- VecDeque로 슬라이딩 윈도우 처리
- BinaryHeap으로 중요도 기반 점 유지
- rayon으로 병렬 처리 가능

## 📌 요약 테이블: 대용량 Polyline 단순화 알고리즘 비교
| 알고리즘               | 특징                              | 순서 보존 | 대용량 처리 적합도 |
|------------------------|-----------------------------------|------------|--------------------|
| Streaming RDP          | 거리 기반, 빠른 처리, 실시간 가능 | ✅         | ✅ 매우 적합        |
| Visvalingam–Whyatt     | 면적 기반, 시각적 품질 우수       | ⚠️ 일부 보존 | ⚠️ 제한적           |
| Sliding Window 방식     | 구간별 처리, GPU 병렬화 가능       | ✅         | ✅ 매우 적합        |
| Top-k 중요도 선택      | 곡률/속도 기반, 정확한 개수 제어   | ✅         | ✅ 적합              |


---
# rapidgeo_simplify
Rust에는 CGAL의 Polyline Simplification 기능과 유사한 라이브러리로 rapidgeo_simplify가 있습니다.
이 라이브러리는 Douglas–Peucker 알고리즘 기반으로, 대규모 polyline 데이터를 빠르게 단순화하면서 궤적의 특징을 유지할 수 있음.

## 🧠 rapidgeo_simplify 라이브러리 개요
- 기능: Douglas–Peucker 알고리즘 기반 polyline 단순화
- 특징:
- 거리 기반 threshold 설정 가능
- 시작점과 끝점은 항상 유지
- 다양한 거리 계산 방식 지원 (구면 거리, 평면 거리, 유클리드 거리)
- 병렬 처리(batch feature)로 대량 데이터 처리 가능
- 각도 기반 단순화는 직접적으로 지원하지 않습니다.

### ⚙️ 사용 예시
```rust
use rapidgeo_distance::LngLat;
use rapidgeo_simplify::{simplify_dp_into, SimplifyMethod};

fn main() {
    let points = vec![
        LngLat::new_deg(-122.4194, 37.7749), // San Francisco
        LngLat::new_deg(-122.0, 37.5),
        LngLat::new_deg(-121.5, 37.0),
        LngLat::new_deg(-118.2437, 34.0522), // Los Angeles
    ];

    let mut simplified = Vec::new();
    let count = simplify_dp_into(
        &points,
        50000.0, // 50km tolerance
        SimplifyMethod::GreatCircleMeters,
        &mut simplified,
    );

    println!("Simplified count: {}", count);
}
```

## 📦 주요 기능: rapidgeo_simplify
| 함수 / 기능명         | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| `simplify_dp_into`     | Douglas–Peucker 알고리즘 기반으로 polyline을 단순화. 결과를 Vec에 직접 저장 |
| `SimplifyMethod`       | 거리 계산 방식 선택 (구면 거리, 평면 거리, 유클리드 거리 등)             |
| `batch`                | 여러 polyline을 병렬로 처리하는 기능 (대용량 처리에 적합)               |
| `simplify_dp_mask`     | 유지할 점을 boolean mask로 반환 (필터링이나 시각화에 유용)              |


## ✅ 대용량 처리에 적합한 이유
- Streaming 방식으로 메모리 부담 없이 처리 가능
- 병렬 처리 지원으로 수백만~수억 개 점도 빠르게 처리
- 거리 기반 threshold 조절로 10~20% 수준까지 유연하게 줄일 수 있음

---
# 각도 기반
CGAL의 polyline simplification은 각도 기반 방식도 지원하는데,  
Rust에서도 각도 기반으로 점을 선택하거나 제거하는 방식을 직접 구현하거나 일부 라이브러리에서 유사하게 처리할 수 있음.

## 🧠 각도 기반 Polyline Simplification이란?
- 각 점에서 이전 점–현재 점–다음 점을 연결한 **내각(또는 외각)** 을 계산
- 이 각도가 **거의 직선(180도)** 에 가까우면 중요도가 낮다고 판단 → 제거
- **곡률이 큰 지점(예: 급커브)** 은 유지

## ✅ Rust에서 각도 기반 처리 방법
현재 rapidgeo_simplify는 거리 기반(Douglas–Peucker) 방식이 중심이지만,  
각도 기반 방식은 직접 구현하거나 다음과 같이 처리할 수 있음:  
## 🔧 직접 구현 예시 흐름
```rust
use geo::{Coord};
fn angle_between(p1: Coord, p2: Coord, p3: Coord) -> f64 {
    let v1 = (p1.x - p2.x, p1.y - p2.y);
    let v2 = (p3.x - p2.x, p3.y - p2.y);

    let dot = v1.0 * v2.0 + v1.1 * v2.1;
    let mag1 = (v1.0.powi(2) + v1.1.powi(2)).sqrt();
    let mag2 = (v2.0.powi(2) + v2.1.powi(2)).sqrt();

    let cos_theta = dot / (mag1 * mag2);
    cos_theta.acos().to_degrees()
}
```

- 이 각도를 기준으로 threshold 이하인 점은 제거하면 됨.
- 예: angle < 10° 또는 angle > 170° → 제거 후보

## 📦 요약: Polyline 단순화 방식 비교
| 방식             | 핵심 기준           | 특징 및 장점                              | Rust 구현 가능성     |
|------------------|---------------------|-------------------------------------------|----------------------|
| 거리 기반        | 선분에서의 거리     | Douglas–Peucker, 궤적 유지에 강함         | ✅ rapidgeo_simplify 사용 가능 |
| 각도 기반        | 방향 변화량         | 급커브 유지, 직선 구간 제거에 효과적      | ✅ 직접 구현 필요     |
| 곡률 기반        | 곡선의 휘어짐 정도  | 속도/방향 변화 반영, 중요도 기반 선택     | ✅ 직접 구현 또는 응용 가능 |



## 📌 rapidgeo_simplify의 주요 특징
- ✅ 거리 기반 단순화 (Douglas–Peucker)
- ✅ 구면 거리 / 평면 거리 / 유클리드 거리 선택 가능
- ✅ Streaming 처리 및 병렬 처리(batch) 지원
- ❌ 각도 기반 단순화는 미지원 (직접 구현 필요)

## 🔧 각도 기반 단순화가 필요한 경우?
- 궤적에서 급격한 방향 변화를 유지하고 싶을 때
- 곡률 중심의 중요도 판단이 필요한 경우
- CGAL 스타일의 시각적 품질 중심 단순화가 필요할 때  
이 경우에는 geo 크레이트를 활용해 직접 각도를 계산하고, threshold 이하의 점을 제거하는 방식으로 구현해야 함.

--- 
# geo 크레이트로 각도 기반 구현`
geo 크레이트를 사용하면 각도 기반 Polyline 단순화를 충분히 구현할 수 있음.  
직접 각도를 계산해서 궤적의 굴곡이 큰 지점만 남기고, 직선에 가까운 구간은 제거하는 방식입니다.

## 🧠 각도 기반 단순화 개념
- 각 점에서 이전–현재–다음 점을 연결해 내각을 계산
- 각도가 180°에 가까우면 직선, 작거나 크면 굴곡 → 중요도 판단
- threshold 설정: 예를 들어 angle < 150° 또는 angle > 30°인 점만 유지

## 🔧 Rust + geo 예제 흐름
```rust
use geo::{Coord, Line};

fn angle_between(p1: Coord, p2: Coord, p3: Coord) -> f64 {
    let v1 = (p1.x - p2.x, p1.y - p2.y);
    let v2 = (p3.x - p2.x, p3.y - p2.y);

    let dot = v1.0 * v2.0 + v1.1 * v2.1;
    let mag1 = (v1.0.powi(2) + v1.1.powi(2)).sqrt();
    let mag2 = (v2.0.powi(2) + v2.1.powi(2)).sqrt();

    let cos_theta = dot / (mag1 * mag2);
    cos_theta.acos().to_degrees()
}

fn simplify_by_angle(coords: &[Coord], threshold_deg: f64) -> Vec<Coord> {
    let mut simplified = Vec::new();
    if coords.len() < 3 {
        return coords.to_vec();
    }

    simplified.push(coords[0]); // 첫 점 유지

    for i in 1..coords.len() - 1 {
        let angle = angle_between(coords[i - 1], coords[i], coords[i + 1]);
        if angle < threshold_deg || angle > (180.0 - threshold_deg) {
            simplified.push(coords[i]);
        }
    }

    simplified.push(coords[coords.len() - 1]); // 마지막 점 유지
    simplified
}
```


## ✅ 특징: 각도 기반 Polyline 단순화
| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 순서 보존          | 입력된 점의 순서를 그대로 유지함                                     |
| 굴곡 중심 유지     | 급격한 방향 변화(작은 각도)만 남기고 직선 구간은 제거                |
| threshold 조절 가능 | 각도 기준(예: 150° 이상 또는 30° 이하)으로 유연하게 단순화 가능       |
| geo 기반 구현       | `geo::Coord`와 `geo::Line`을 활용해 각도 계산 및 구조화 가능          |
| CoordLine 활용      | `Coord`로 점 표현, `Line`으로 벡터 방향 계산해 각도 추출에 사용됨     |

----

# simplify_by_angle 실전 적용 
simplify_by_angle 같은 Rust 함수는 가볍고 빠르게 실전 적용이 가능하지만, 제어 범위는 제한적.

## ✅ simplify_by_angle 실전 적용 가능성
| 항목                 | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| 실전 적용 가능        | 지도 렌더링, GPS 궤적 시각화, 경로 최적화 등에 바로 활용 가능          |
| 속도 빠름             | 1-pass 처리로 대용량 데이터도 빠르게 단순화 가능                       |
| 순서 보존             | 입력된 점의 순서를 그대로 유지하며 궤적 흐름을 살림                    |
| threshold 조절 가능   | 각도 기준(예: 150° 이상 또는 30° 이하)으로 유연하게 단순화 가능         |
| geo 크레이트 호환     | `geo::Coord`, `geo::Line`, `LineString` 등과 쉽게 연동 가능             |
| 시각화 및 후처리 용이 | 결과를 `geojson`, `bevy`, `plotters` 등으로 시각화하거나 후속 처리 가능 |

## 🧪 실전에서 쓰기 좋은 예시
- 지도에 GPS 경로 표시할 때
- 실시간 궤적 시각화
- 대용량 polyline을 간단히 줄여서 렌더링 성능 확보
- Boolean 연산 전에 궤적 정리

## 📌 실전 적용 팁
- threshold_deg를 상황에 따라 조절 (예: 10° ~ 30°)
- geo::LineString으로 변환해 geojson으로 내보내기 가능
- simplify_by_angle → geo::Polygon → Boolean 연산 연결 가능


## ✅ simplify_by_angle가 충분한 경우

| 적용 목적               | 적합 여부 | 설명                                                         |
|------------------------|-----------|--------------------------------------------------------------|
| 지도 시각화            | ✅        | 렌더링 성능 확보를 위해 궤적을 간결하게 표현할 수 있음       |
| GPS 경로 요약          | ✅        | 급커브나 주요 흐름만 남기고 직선 구간은 제거 가능             |
| 사용자 인터페이스 표시 | ✅        | 간단한 경로 표현에 적합하며, UI 성능 저하 없이 표시 가능      |
| 대략적인 경로 분석     | ✅        | 전체 흐름을 유지하면서 주요 굴곡만 남겨 분석에 활용 가능      |


## ⚠️ simplify_by_angle가 부족할 수 있는 경우
| 적용 목적                   | 적합 여부 | 설명                                                                 |
|----------------------------|-----------|----------------------------------------------------------------------|
| 정밀한 시뮬레이션 분석      | ❌        | 유량, 압력, 경로 길이 등 물리적 특성이 중요할 경우 미세한 굴곡도 필요함 |
| 안전성 평가 및 취약점 분석 | ❌        | 누출 위험, 구조적 취약점 등은 작은 경로 변화도 민감하게 작용할 수 있음 |
| 시공 도면 생성             | ❌        | 실제 좌표 그대로 유지해야 하며, 단순화된 경로는 오차를 유발할 수 있음 |
| 법적/행정적 경계 지정      | ❌        | 좌표 하나하나가 법적 의미를 가질 수 있어 단순화가 부적절함             |


## 🗺️ 지도 시각화에 적합한 이유
| 항목                 | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| 렌더링 성능 최적화    | 점 수를 대폭 줄여 지도에서 부드럽고 빠르게 표시 가능                   |
| 궤적 흐름 유지        | 급커브, 분기점 등 중요한 지점은 그대로 남아 시각적으로 자연스러움        |
| 순서 보존             | polyline의 흐름이 깨지지 않아 실제 경로와 유사한 형태 유지               |
| 실시간 처리 가능      | 1-pass 처리로 대용량 데이터도 빠르게 단순화 가능                         |
| geo::LineString + geojson | 결과를 `geo::LineString`으로 변환해 `geojson`으로 내보내기 가능         |
| 지도 라이브러리 연동  | Leaflet, Mapbox, Cesium 등과 쉽게 연동 가능                              |


## 🔧 시각화 연동 예시 흐름
- simplify_by_angle로 점 단순화
- 결과를 geo::LineString으로 변환
- geojson으로 내보내기
- 지도 라이브러리 (예: Leaflet, Mapbox, Cesium 등)에서 표시

---
