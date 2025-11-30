#  Geo Booleanop

## 📌 AI 활용 사례
### 1. 데이터 전처리
- 이미지나 센서 데이터에서 추출된 영역(Polygon)을 서로 합치거나 빼서 **관심 영역(ROI)** 을 정의
- 예: 카메라/레이더가 감지한 영역을 합집합으로 병합 → 학습용 입력 데이터 생성
### 2. 라벨 정제
- 사람이 만든 라벨과 자동 생성된 라벨을 교집합/차집합으로 비교 → 정확한 Ground Truth 확보
- 예: AI가 탐지한 차량 영역과 실제 라벨의 교집합 → 정답률 계산
### 3. 데이터 증강
- 기존 다각형을 합집합/차집합으로 변형해 새로운 학습 샘플 생성
- 예: 건물 영역과 도로 영역을 XOR → **도로 위 건물 없는 영역** 데이터셋 생성
### 4. 이상 탐지
- 정상 영역과 새로운 입력 영역의 차집합을 계산 → 비정상 패턴 감지
- 예: 교통 흐름에서 차량이 있어서는 안 되는 영역에 들어왔는지 확인
### 5. 멀티센서 융합
- 카메라, 라이다, 레이더 등 서로 다른 센서가 감지한 영역을 Boolean 연산으로 결합
- 예: 라이다 감지 영역 ∩ 카메라 감지 영역 → 신뢰도 높은 학습 데이터

## ✅ 결론
geo_booleanop은 단순한 GIS 도구가 아니라, AI 학습 데이터 전처리와 증강, 라벨 정제, 이상 탐지에 직접 활용될 수 있습니다.  
특히 공간적 패턴을 다루는 AI(자율주행, 드론, 로보틱스, 스마트시티)에서 매우 유용.

**AI 파이프라인에서 geo_booleanop을 활용하는 단계별 예시** 를 그림으로 정리.

---


## 📦 기본 준비
```
# Cargo.toml
[dependencies]
geo = "0.28"
geo-booleanop = "0.4"
```

## 1️⃣ 두 다각형의 합집합 (union)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let union = poly1.union(&poly2);
    println!("Union result: {:?}", union);
}
```


## 2️⃣ 두 다각형의 교집합 (intersection)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let intersection = poly1.intersection(&poly2);
    println!("Intersection result: {:?}", intersection);
}
```


## 3️⃣ 두 다각형의 차집합 (difference)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let difference = poly1.difference(&poly2);
    println!("Difference result: {:?}", difference);
}
```


## 4️⃣ 두 다각형의 배타적 논리합 (xor)
```rust
use geo::{polygon, Polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1: Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 2.0, y: 0.0),
        (x: 2.0, y: 2.0),
        (x: 0.0, y: 2.0),
    ];

    let poly2: Polygon<f64> = polygon![
        (x: 1.0, y: 1.0),
        (x: 3.0, y: 1.0),
        (x: 3.0, y: 3.0),
        (x: 1.0, y: 3.0),
    ];

    let xor = poly1.xor(&poly2);
    println!("XOR result: {:?}", xor);
}
```
## ✅ 정리
- union → 두 영역 합치기
- intersection → 겹치는 부분만 추출
- difference → 한 영역에서 다른 영역 빼기
- xor → 겹치지 않는 부분만 추출

---

# 단계별 활용


## 단계별 활용 예시
### 1️⃣ 데이터 수집
- 센서(카메라, 라이다, 레이더) 또는 GIS 시스템에서 다각형 영역(Polygon) 데이터를 얻음
- 예: 차량 감지 → 차량의 위치를 다각형으로 표현
```rust
use geo::{polygon, Polygon};

let car_area: Polygon<f64> = polygon![
    (x: 0.0, y: 0.0),
    (x: 2.0, y: 0.0),
    (x: 2.0, y: 2.0),
    (x: 0.0, y: 2.0),
];
```
### 2️⃣ 라벨 데이터와 비교
- 사람이 만든 라벨(정답 영역)과 AI가 감지한 영역을 **교집합(intersection)** 으로 비교
- 겹치는 부분이 많을수록 정확도가 높음
```rust
use geo_booleanop::boolean::BooleanOp;

let ground_truth: Polygon<f64> = polygon![
    (x: 1.0, y: 1.0),
    (x: 3.0, y: 1.0),
    (x: 3.0, y: 3.0),
    (x: 1.0, y: 3.0),
];

let overlap = car_area.intersection(&ground_truth);
println!("Overlap area: {:?}", overlap);
```

### 3️⃣ 데이터 증강
- 기존 영역을 합집합(union) 또는 **차집합(difference)** 으로 변형해 새로운 학습 샘플 생성
- 예: 차량 영역 + 도로 영역 → **차량이 도로 위에 있는 데이터셋**
```rust
let road_area: Polygon<f64> = polygon![
    (x: -1.0, y: -1.0),
    (x: 4.0, y: -1.0),
    (x: 4.0, y: 4.0),
    (x: -1.0, y: 4.0),
];

let car_on_road = car_area.union(&road_area);
println!("Car on road area: {:?}", car_on_road);
```

### 4️⃣ 이상 탐지
- 정상 영역과 새로운 입력 영역의 **차집합(difference)** 을 계산 → 비정상 패턴 감지
- 예: 차량이 도로 밖에 있는 경우
```rust
let abnormal = car_area.difference(&road_area);
println!("Abnormal area (car outside road): {:?}", abnormal);
```

### 5️⃣ 멀티센서 융합
- 카메라와 라이다가 감지한 영역을 **교집합(intersection)** 으로 결합 → 신뢰도 높은 데이터 생성
- AI 학습 시 **센서 융합 데이터셋** 으로 활용
```rust
let camera_area = car_area.clone();
let lidar_area = ground_truth.clone();

let fused = camera_area.intersection(&lidar_area);
println!("Fused sensor area: {:?}", fused);
```

## ✅ 정리
- 교집합(intersection) → 정확도 평가, 센서 융합
- 합집합(union) → 데이터 증강, 영역 병합
- 차집합(difference) → 이상 탐지, 라벨 정제
- XOR → 겹치지 않는 영역 분석

### 흐름도

![GeoBoolean 흐름도](/image/geo_boolean_ai.png)

---

