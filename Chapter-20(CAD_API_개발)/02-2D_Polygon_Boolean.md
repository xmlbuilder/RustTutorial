# 2D Boolean
geo 크레이트의 Boolean 연산 부분만 딱 떼어서 소개.    
이건 CAD나 곡선 모델링에서 진짜 핵심 기능  


## 🧠 geo 크레이트의 Boolean 연산 기능
Boolean 연산은 2D 다각형(Polygon) 간의 기하학적 집합 연산을 의미합니다.
| 연산 함수        | 기능 설명                                 |
|------------------|--------------------------------------------|
| union()          | 두 다각형의 합집합 영역 계산               |
| intersection()   | 두 다각형이 겹치는 영역 추출               |
| difference()     | 첫 번째 다각형에서 두 번째를 뺀 영역 계산  |
| xor()            | 두 다각형의 대칭 차집합 (겹치지 않는 부분) |

이 연산들은 geo::BooleanOps trait을 통해 제공됩니다.
## ⚙️ 사용법: Cargo.toml 설정
```
[dependencies]
geo = "0.30"
```
- 최신 버전 확인: https://crates.io/crates/geo


## 📦 주요 타입
- Polygon<f64>: 다각형 객체
- GeometryCollection<f64>: Boolean 연산 결과가 여러 개의 다각형일 수 있음
- BooleanOps trait: Boolean 연산을 제공하는 확장 trait

### ✅ 예제 1: 두 사각형의 교차 영역
```rust
use geo::{polygon, BooleanOps};

fn main() {
    let poly1 = polygon![
        (x: 0.0, y: 0.0),
        (x: 5.0, y: 0.0),
        (x: 5.0, y: 5.0),
        (x: 0.0, y: 5.0),
    ];

    let poly2 = polygon![
        (x: 3.0, y: 3.0),
        (x: 8.0, y: 3.0),
        (x: 8.0, y: 8.0),
        (x: 3.0, y: 8.0),
    ];

    let result = poly1.intersection(&poly2);
    println!("Intersection: {:?}", result);
}
```


### ✅ 예제 2: 차집합 (poly1 - poly2)
```rust
let result = poly1.difference(&poly2);
println!("Difference: {:?}", result);
```


### ✅ 예제 3: 합집합
```rust
let result = poly1.union(&poly2);
println!("Union: {:?}", result);
```


## ✅ 예제 4: XOR (대칭 차집합)
```rust
let result = poly1.xor(&poly2);
println!("XOR: {:?}", result);
```


## 🧪 테스트 팁
- 결과는 GeometryCollection<Polygon>으로 반환되므로 .iter()로 순회하거나 .0으로 내부 벡터에 접근 가능
- 시각화하려면 geojson, plotters, bevy 등과 연동 가능
- 복잡한 다각형은 simplify()로 정리한 뒤 Boolean 연산하는 게 안정적


## ✅ Boolean 연산 결과 데이터 추출 예제
Boolean 연산 결과는 GeometryCollection<Polygon<f64>> 타입으로 반환됩니다.  
이걸 순회하면서 각 다각형의 좌표를 뽑아낼 수 있음.  
### 🔧 예제: 교차 영역 좌표 추출
```rust
use geo::{polygon, BooleanOps, Geometry};

fn main() {
    let poly1 = polygon![
        (x: 0.0, y: 0.0),
        (x: 5.0, y: 0.0),
        (x: 5.0, y: 5.0),
        (x: 0.0, y: 5.0),
    ];

    let poly2 = polygon![
        (x: 3.0, y: 3.0),
        (x: 8.0, y: 3.0),
        (x: 8.0, y: 8.0),
        (x: 3.0, y: 8.0),
    ];

    let result = poly1.intersection(&poly2);

    for geom in result.0 {
        if let Geometry::Polygon(p) = geom {
            println!("Exterior:");
            for coord in p.exterior().coords_iter() {
                println!("({}, {})", coord.x, coord.y);
            }

            println!("Interior holes:");
            for hole in p.interiors() {
                for coord in hole.coords_iter() {
                    println!("Hole coord: ({}, {})", coord.x, coord.y);
                }
            }
        }
    }
}
```

## 🧠 Convex vs Concave 처리

| 형태               | 처리 가능 여부 | 특징 및 주의사항                          |
|--------------------|----------------|-------------------------------------------|
| Convex 다각형      | ✅ 완벽 지원    | 일반적인 사각형, 원형 등 안정적으로 처리됨 |
| Concave 다각형     | ✅ 완벽 지원    | 오목한 구조도 Boolean 연산에 문제 없음     |
| Self-intersecting  | ⚠️ 제한적 지원 | 일부 복잡한 경우 실패하거나 오류 발생 가능 |
| 복잡한 경계 구조   | ✅ 가능 (권장: simplify) | `simplify()`로 정리 후 연산하면 안정적     |

- 복잡한 Concave 다각형은 simplify() 또는 make_valid()로 정리 후 연산하는 게 안정적.


## ✨ 요약
- Boolean 연산 결과는 GeometryCollection<Polygon>으로 반환
- .exterior()와 .interiors()로 좌표 추출 가능
- Convex/Concave 모두 지원
- 복잡한 경우는 사전 정리(simplify) 추천

## 다각형 연결 구조 추출
geo 크레이트에서 다각형의 edge 연결 구조를 추출하려면,  
다각형의 외곽선(exterior)이나 내부 홀(interiors)을 구성하는 좌표 리스트를 순회하면서 인접한 점들을 edge로 묶는 방식을 사용.

## 🧠 edge 연결 구조란?
- 다각형은 점(Point)들의 리스트로 구성됨
- 이 점들을 순서대로 연결하면 **edge(선분)** 이 됨
- geo에서는 Line 또는 LineString으로 표현 가능

### ✅ 예제: Polygon의 edge 추출
```rust
use geo::{polygon, Line, Coord};

fn main() {
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 5.0, y: 0.0),
        (x: 5.0, y: 5.0),
        (x: 0.0, y: 5.0),
    ];

    let exterior = poly.exterior();
    let coords: Vec<Coord> = exterior.coords_iter().collect();

    let mut edges = Vec::new();
    for i in 0..coords.len() {
        let start = coords[i];
        let end = coords[(i + 1) % coords.len()]; // 닫힌 다각형이므로 마지막 → 첫 점 연결
        edges.push(Line::new(start, end));
    }

    for edge in edges {
        println!("Edge: ({}, {}) → ({}, {})", edge.start.x, edge.start.y, edge.end.x, edge.end.y);
    }
}
```


### 🔧 내부 홀(edge)도 추출하려면?
```rust
for hole in poly.interiors() {
    let coords: Vec<Coord> = hole.coords_iter().collect();
    for i in 0..coords.len() {
        let start = coords[i];
        let end = coords[(i + 1) % coords.len()];
        let edge = Line::new(start, end);
        println!("Hole edge: {:?}", edge);
    }
}
```
## ✨ 요약: Polygon에서 edge 추출을 위한 핵심 API

| 함수 또는 연산        | 설명 또는 반환 타입                        |
|------------------------|--------------------------------------------|
| exterior()             | Polygon의 외곽선 반환 (LineString)         |
| interiors()            | Polygon의 내부 홀들 반환 (Vec<LineString>) |
| coords_iter()          | 좌표 반복자 → Coord { x, y }               |
| Line::new(start, end)  | 두 좌표를 연결한 edge 생성                 |
| % coords.len()         | 마지막 점 → 첫 점 연결 (닫힌 다각형 처리)  |

---

# 추출한 정보 담기

## 아래는 geo의 Polygon을 기반으로
- 외곽선 좌표 (outer_coords)
- 내부 홀 좌표 (inner_coords)
- 연결된 edge 리스트 (edges)  
를 모두 추출해서 Mesh2D라는 구조체에 담는 전체 예제.

## 🧱 구조 정의: Mesh2D
```rust
use geo::{Coord, Line};

#[derive(Debug)]
struct Mesh2D {
    outer_coords: Vec<Coord>,
    inner_coords: Vec<Vec<Coord>>,
    edges: Vec<Line<f64>>,
}
```
### 🧪 전체 예제: Polygon → Mesh2D
```rust
use geo::{polygon, Coord, Line, Polygon};

#[derive(Debug)]
struct Mesh2D {
    outer_coords: Vec<Coord>,
    inner_coords: Vec<Vec<Coord>>,
    edges: Vec<Line<f64>>,
}

fn polygon_to_mesh(polygon: &Polygon<f64>) -> Mesh2D {
    let outer: Vec<Coord> = polygon.exterior().coords_iter().collect();

    let mut edges = Vec::new();
    for i in 0..outer.len() {
        let start = outer[i];
        let end = outer[(i + 1) % outer.len()];
        edges.push(Line::new(start, end));
    }

    let mut inner_coords = Vec::new();
    for ring in polygon.interiors() {
        let coords: Vec<Coord> = ring.coords_iter().collect();
        for i in 0..coords.len() {
            let start = coords[i];
            let end = coords[(i + 1) % coords.len()];
            edges.push(Line::new(start, end));
        }
        inner_coords.push(coords);
    }

    Mesh2D {
        outer_coords: outer,
        inner_coords,
        edges,
    }
}

fn main() {
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 5.0, y: 0.0),
        (x: 5.0, y: 5.0),
        (x: 0.0, y: 5.0),
    ];

    let mesh = polygon_to_mesh(&poly);
    println!("{:#?}", mesh);
}
```


## ✨ 결과 구조
- outer_coords: 외곽선 좌표 리스트
- inner_coords: 내부 홀 좌표 리스트들
- edges: 모든 선분 (외곽 + 내부)

---
# half-edge구조에 데이터 담기
geo 기반 Polygon을 half-edge 구조로 변환하는 방법을 설명.  
이건 메쉬 구조를 좀 더 정밀하게 다루거나, topology 기반 연산을 하고 싶을 때 핵심이 되는 구조.

## 🧠 Half-Edge 구조란?
- Vertex, Edge, Face를 분리해서 표현하는 메쉬 구조
- 각 edge는 반대 방향의 twin edge를 가짐
- edge는 다음 edge(next), 이전 edge(prev), 소속 face, 시작 vertex 등을 참조함
- 다각형의 경계를 따라 순회하거나, 인접 face를 탐색할 때 유용함

## 🧱 구조 정의 예시
```rust
use geo::{Coord, Polygon};

#[derive(Debug)]
struct Vertex {
    id: usize,
    coord: Coord,
}

#[derive(Debug)]
struct HalfEdge {
    id: usize,
    start: usize,         // Vertex ID
    end: usize,           // Vertex ID
    twin: Option<usize>,  // Twin edge ID
    next: Option<usize>,  // Next edge ID
    prev: Option<usize>,  // Previous edge ID
    face: Option<usize>,  // Face ID
}

#[derive(Debug)]
struct Face {
    id: usize,
    edge: usize,          // 시작 edge ID
}

#[derive(Debug)]
struct HalfEdgeMesh {
    vertices: Vec<Vertex>,
    edges: Vec<HalfEdge>,
    faces: Vec<Face>,
}
```


### 🔧 Polygon → HalfEdgeMesh 변환 예제
```rust
fn polygon_to_half_edge_mesh(polygon: &Polygon<f64>) -> HalfEdgeMesh {
    let coords: Vec<Coord> = polygon.exterior().coords_iter().collect();
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    for (i, coord) in coords.iter().enumerate() {
        vertices.push(Vertex { id: i, coord: *coord });
    }

    for i in 0..coords.len() {
        let edge = HalfEdge {
            id: i,
            start: i,
            end: (i + 1) % coords.len(),
            twin: None,       // 단일 polygon이므로 twin은 없음
            next: Some((i + 1) % coords.len()),
            prev: Some((i + coords.len() - 1) % coords.len()),
            face: Some(0),
        };
        edges.push(edge);
    }

    let face = Face { id: 0, edge: 0 };

    HalfEdgeMesh {
        vertices,
        edges,
        faces: vec![face],
    }
}
```

## ✨ 결과 구조 요약: Half-Edge Mesh 구성 요소

| 구조체 이름     | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| `Vertex`       | 하나의 점을 표현. 고유 ID와 좌표(Coord)를 가짐                        |
| `HalfEdge`     | 방향성 있는 edge. 시작/끝 vertex ID, twin/next/prev/face 참조 포함     |
| `Face`         | 하나의 polygon face. 시작 edge ID를 통해 경계 순회 가능                |
| `HalfEdgeMesh` | 전체 메쉬 구조. Vertex, HalfEdge, Face 리스트를 포함하는 컨테이너      |

---



