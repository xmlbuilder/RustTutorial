# EarCut
- Earcut은 다각형을 삼각형으로 분할하는 고전적인 알고리즘으로,  
- 특히 복잡한 2D 폴리곤을 빠르고 안정적으로 삼각형으로 나누는 데 사용됩니다.  
- 아래 그림은 Earcut 알고리즘이 비볼록 다각형을 어떻게 삼각형으로 분할하는지를 보여줍니다.
## Earcut 알고리즘 시각화

![Ear Cut](/image/ear_cut.png)


## 🧠 Earcut 알고리즘 이론 설명
### 1. 🎯 목적: 폴리곤 삼각형 분할 (Triangulation)
- 입력: 단순 폴리곤(자기 교차 없음), 선택적으로 홀 포함
- 출력: 삼각형 인덱스 리스트 (예: [1, 0, 3, 3, 2, 1])
- 목표: 폴리곤을 삼각형들로 나누어 렌더링, 물리 계산, 메시 생성 등에 활용
### 2. 🧩 핵심 개념: Ear Clipping
- "귀(Ear)"란?
    - 세 개의 연속된 꼭짓점이 이루는 삼각형이 폴리곤 내부에 있고, 그 안에 다른 꼭짓점이 없는 경우  
    - 귀 자르기 과정:
        - 폴리곤에서 귀를 찾는다
        - 해당 귀를 잘라내고 삼각형으로 저장
        - 남은 폴리곤에 대해 반복
        - 모든 꼭짓점이 제거되면 종료
### 3. 🧮 알고리즘 단계
- 초기화: 꼭짓점 리스트를 이중 연결 리스트로 변환
- 홀 처리: 각 홀을 외곽선에 연결하여 단일 링으로 변환
- 귀 탐색 및 제거: CCW 방향으로 귀를 탐색하고 제거
- 삼각형 저장: 제거된 귀를 삼각형으로 저장
- 반복: 더 이상 귀가 없을 때까지 반복
### 4. 📦 데이터 구조
- Node 구조체: 각 꼭짓점은 Node로 표현되며, prev, next, x, y 등의 정보를 가짐
- Z-order 최적화: 공간적 정렬을 통해 귀 탐색 속도 향상

## 🧪 Rust 코드 예시 설명
```rust
let vertices = vec![10.0, 0.0, 0.0, 50.0, 60.0, 60.0, 70.0, 10.0];
let holes = vec![];
let dimensions = 2;

let triangles = earcutr::earcut(&vertices, &holes, dimensions);
```
- `vertices`: [x0, y0, x1, y1, x2, y2, ...] 형태의 꼭짓점 좌표
- `holes`: 폴리곤 내의 구멍 시작 인덱스 (없으면 빈 벡터)
- `dimensions`: 2D 폴리곤이므로 2
- `triangles`: 삼각형 인덱스 결과 (예: [1, 0, 3, 3, 2, 1] → 두 개의 삼각형)

## 📌 Earcut의 장점
- 빠름: 대부분의 경우 O(n^2) 또는 더 빠름
- 간단함: 구현이 직관적이고 안정적
- 다용도: GIS, 게임, CAD, 웹 그래픽 등에서 활용 가능

---

# earcut 테스트
```rust
#[cfg(test)]
mod test {
    use nurbslib::core::ear_cut::{on_triangulate_polygon, on_triangulate_with_holes,
        on_triangulate_polygon_to_mesh, on_triangulate_with_holes_to_mesh};
    use nurbslib::core::geom::Point2D;
```
```rust
    #[test]
    fn test_triangulate_polygon() {
        let polygon = vec![
            Point2D::new(10.0, 0.0),
            Point2D::new(0.0, 50.0),
            Point2D::new(60.0, 60.0),
            Point2D::new(70.0, 10.0),
        ];

        let triangles = on_triangulate_polygon(&polygon);
        println!("Triangulated indices: {:?}", triangles);
    }
```
```rust
    #[test]
    fn test_triangulate_with_holes() {
        let outer = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(100.0, 0.0),
            Point2D::new(100.0, 100.0),
            Point2D::new(0.0, 100.0),
        ];

        let hole = vec![
            Point2D::new(30.0, 30.0),
            Point2D::new(70.0, 30.0),
            Point2D::new(70.0, 70.0),
            Point2D::new(30.0, 70.0),
        ];

        let triangles = on_triangulate_with_holes(&outer, &[hole]);
        println!("Triangulated indices with hole: {:?}", triangles);
    }
```
```rust
    #[test]
    fn test_triangulate_polygon_to_mesh() {
        let polygon = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(100.0, 0.0),
            Point2D::new(100.0, 100.0),
            Point2D::new(0.0, 100.0),
        ];

        let mesh = on_triangulate_polygon_to_mesh(&polygon);

        println!("Vertices:");
        for v in &mesh.vertices {
            println!("({:.1}, {:.1}, {:.1})", v.x, v.y, v.z);
        }

        println!("\nFaces:");
        for f in &mesh.faces {
            println!("{:?}", f);
        }

        println!("\nTriangle count: {}", mesh.triangle_count());
    }
```
```rust
    #[test]
    fn test_triangulate_with_holes_to_mesh() {
        let outer = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(100.0, 0.0),
            Point2D::new(100.0, 100.0),
            Point2D::new(0.0, 100.0),
        ];

        let hole = vec![
            Point2D::new(30.0, 30.0),
            Point2D::new(70.0, 30.0),
            Point2D::new(70.0, 70.0),
            Point2D::new(30.0, 70.0),
        ];

        let mesh = on_triangulate_with_holes_to_mesh(&outer, &[hole]);

        println!("Vertices:");
        for v in &mesh.vertices {
            println!("({:.1}, {:.1}, {:.1})", v.x, v.y, v.z);
        }

        println!("\nFaces:");
        for f in &mesh.faces {
            println!("{:?}", f);
        }
        println!("\nTriangle count: {}", mesh.triangle_count());
    }
```
```rust
    #[test]
    fn test_simple() {
        let vertices = vec![10.0, 0.0, 0.0, 50.0, 60.0, 60.0, 70.0, 10.0];
        let holes = vec![];
        let dimensions = 2;

        let triangles = earcutr::earcut(&vertices, &holes, dimensions);
        println!("{:?}", triangles); // 예: [1, 0, 3, 3, 2, 1]
    }

}
```
---
