# Spade CDT (2D Mesher Builder)

## 소스 코드
```rust
use spade::ConstrainedDelaunayTriangulation as CDT;
use spade::Point2;
use spade::Triangulation;
use crate::core::mesh::Mesh;
use crate::core::prelude::{Point3D};

pub struct MeshBuilder;
```
```rust
impl MeshBuilder {
    /// 외곽 루프와 홀 루프들을 받아 Mesh 생성
    pub fn from_loops(
        outer: Vec<Point2<f64>>,
        holes: Vec<Vec<Point2<f64>>>,
    ) -> Mesh {
        // 1) CDT 초기화
        let mut cdt: CDT<Point2<f64>> = CDT::default();

        // 2) 외곽 루프 삽입
        let outer_handles = Self::add_loop_constraints(&mut cdt, &outer);

        // 3) 홀 루프 삽입
        for hole in holes {
            let _ = Self::add_loop_constraints(&mut cdt, &hole);
        }

        // 4) 삼각형 추출
        let mut vertices: Vec<Point3D> = Vec::new();
        let mut faces: Vec<[u32; 4]> = Vec::new();

        for v in cdt.vertices() {
            let p = v.position();
            vertices.push(Point3D::new(p.x, p.y, 0.0));
        }

        for f in cdt.fixed_inner_faces() {
            let [va, vb, vc] = cdt.face(f).vertices();
            let (a, b, c) = (va.fix(), vb.fix(), vc.fix());
            let pa = cdt.vertex(a).position();
            let pb = cdt.vertex(b).position();
            let pc = cdt.vertex(c).position();

            // 삼각형 face 추가
            let ia = a.index() as u32;
            let ib = b.index() as u32;
            let ic = c.index() as u32;
            faces.push([ia, ib, ic, ic]);
        }

        Mesh::new(vertices, faces)
    }
```
```rust
    /// 루프 점들을 CDT에 삽입하고 제약 엣지 추가
    fn add_loop_constraints(
        cdt: &mut CDT<Point2<f64>>,
        loop_pts: &[Point2<f64>],
    ) -> Vec<spade::handles::FixedVertexHandle> {
        let mut hs = Vec::with_capacity(loop_pts.len());
        for &p in loop_pts {
            let h = cdt.insert(p).expect("insert failed");
            hs.push(h);
        }
        for i in 0..loop_pts.len() {
            let a = hs[i];
            let b = hs[(i + 1) % loop_pts.len()];
            let _ = cdt.add_constraint(a, b);
        }
        hs
    }
```
```rust
    pub fn from_loops_with_constraints(
        outer: Vec<Point2<f64>>,
        holes: Vec<Vec<Point2<f64>>>,
        constraints: Vec<(Point2<f64>, Point2<f64>)>, // 추가 제약 에지
    ) -> Mesh {
        // 1) CDT 초기화
        let mut cdt: CDT<Point2<f64>> = CDT::default();

        // 2) 외곽 루프 삽입
        let _outer_handles = Self::add_loop_constraints(&mut cdt, &outer);

        // 3) 홀 루프 삽입
        for hole in holes {
            let _ = Self::add_loop_constraints(&mut cdt, &hole);
        }

        // 4) 추가 제약 에지 삽입
        for (p1, p2) in constraints {
            let h1 = cdt.insert(p1).expect("insert failed for constraint point1");
            let h2 = cdt.insert(p2).expect("insert failed for constraint point2");
            let _ = cdt.add_constraint(h1, h2);
        }

        // 5) 삼각형 추출 → Mesh 변환
        let mut vertices: Vec<Point3D> = Vec::new();
        let mut faces: Vec<[u32; 4]> = Vec::new();

        for v in cdt.vertices() {
            let p = v.position();
            vertices.push(Point3D::new(p.x, p.y, 0.0));
        }

        for f in cdt.fixed_inner_faces() {
            let [va, vb, vc] = cdt.face(f).vertices();
            let (a, b, c) = (va.fix(), vb.fix(), vc.fix());
            let ia = a.index() as u32;
            let ib = b.index() as u32;
            let ic = c.index() as u32;
            faces.push([ia, ib, ic, ic]); // 삼각형 face
        }

        Mesh::new(vertices, faces)
    }
}
```
---

첨부해주신 Spade 기반 CDT(Constrained Delaunay Triangulation) 테스트 코드들을 함수별로 정리하고,  
이 함수들을 활용했을 때 얻을 수 있는 장점까지 정리.

# 📑 테스트 함수 정리

| 함수명                                         | 주요 기능                          | 설명                                   |
|------------------------------------------------|-----------------------------------|----------------------------------------|
| cdt_bulk_load_with_hole                        | 외곽 + 홀을 가진 도형 CDT 로딩     | 외곽과 내부 홀을 정의하고 CDT 생성 후 삼각형 개수 검증 |
| cdt_with_constraints_and_hole_to_stl           | CDT 생성 후 STL 파일로 내보내기    | 외곽과 홀을 제약 조건으로 추가, 내부 face 추출 후 STL 저장 |
| insert_edge_next_to_hole_and_export_stl        | 홀 옆 제약 에지 추가 후 STL 저장   | 홀 꼭짓점과 외곽 꼭짓점을 연결하는 제약 에지 삽입 후 STL 저장 |
| insert_two_vertices_inside_and_connect_with_constraint | 내부 정점 삽입 후 제약 에지 연결 | 내부 정점 두 개 삽입 후 제약 에지 연결, 유효 삼각


## 🛠 이 함수들을 쓰는 장점
- 복잡한 도형 처리 가능
- 단순한 삼각분할이 아니라 **홀(내부 구멍)** 이나 **제약 조건(특정 에지 강제 연결)** 을 포함한 복잡한 도형을 안정적으로 처리할 수 있음.
- 기하학적 안정성 확보
- Spade의 CDT는 Delaunay 조건을 만족하면서 제약 조건을 반영하기 때문에, 삼각형이 뒤틀리거나 왜곡되는 문제를 줄임.
- 자동화된 STL 출력
- 테스트 코드에서 삼각형을 추출해 바로 STL 파일로 내보내기가 가능 → CAD/3D 프린팅 워크플로우와 쉽게 연계.
- 도메인 필터링 기능
- point_in_polygon을 이용해 외곽 내부이면서 홀 외부인 삼각형만 선별 → 원하는 영역만 메쉬로 추출 가능.
- 제약 에지 삽입 검증
- add_constraint로 특정 에지를 강제로 삽입하고, 실제 CDT에 반영되었는지 확인 가능 → Topology 설계 시 유용.
- 테스트 기반 안정성 확보
- 각 기능을 테스트 함수로 분리해 검증 → 라이브러리 업데이트나 코드 변경 시에도 안정적으로 동작하는지 확인 가능.

🎯 요약
- 이 테스트 함수들은 Spade CDT의 다양한 활용 시나리오(홀 처리, 제약 에지 삽입, 내부 정점 추가, STL 출력)를 검증하는 예제들입니다.
- 장점은 복잡한 도형을 안정적으로 삼각분할하고, 필요한 영역만 추출하며, 3D 파일로 바로 내보낼 수 있다는 점입니다.
- Topology 구성, CAD/CAE, 3D 프린팅 등 다양한 응용 분야에서 활용 가치가 큽니다.


```rust
#[cfg(test)]
mod tests {
    use spade::ConstrainedDelaunayTriangulation as CDT;
    use spade::handles::FixedVertexHandle;
    use spade::{InsertionError, Point2, Triangulation};
    use std::fs::File;
    use std::io::{Result as IoResult, Write};

    // 단순 ray-crossing 포함 판정 (경계 포함)
    fn point_in_polygon(pt: Point2<f64>, poly: &[Point2<f64>]) -> bool {
        if poly.len() < 3 {
            return false;
        }
        let (mut inside, mut j) = (false, poly.len() - 1);
        for i in 0..poly.len() {
            let (xi, yi) = (poly[i].x, poly[i].y);
            let (xj, yj) = (poly[j].x, poly[j].y);
            let intersect = ((yi > pt.y) != (yj > pt.y))
                && (pt.x < (xj - xi) * (pt.y - yi) / (yj - yi + f64::EPSILON) + xi);
            if intersect {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
```
```rust
    /// 루프 점들을 넣고 인접쌍으로 제약 엣지 추가
    fn add_loop_constraints(
        cdt: &mut CDT<Point2<f64>>,
        loop_pts: &[Point2<f64>],
    ) -> Vec<FixedVertexHandle> {
        let mut hs: Vec<FixedVertexHandle> = Vec::with_capacity(loop_pts.len());
        for &p in loop_pts {
            let h = cdt.insert(p).expect("insert failed"); // <-- Result 이므로 unwrap/expect 필요
            hs.push(h);
        }
        for i in 0..loop_pts.len() {
            let a = hs[i];
            let b = hs[(i + 1) % loop_pts.len()];
            // spade 2.x: 반환값 사용 안 해도 OK
            let _ = cdt.add_constraint(a, b);
        }
        hs
    }
```
```rust
    /// 간단 ASCII STL 저장 (z=0 평면)
    fn save_stl_ascii(path: &str, tris: &[[[f64; 3]; 3]]) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut f = File::create(path)?;
        writeln!(f, "solid cdt")?;
        for t in tris {
            // 평면 z=0 이므로 법선 대충 (0,0,1)
            writeln!(f, "  facet normal 0 0 1")?;
            writeln!(f, "    outer loop")?;
            for p in t {
                writeln!(f, "      vertex {} {} {}", p[0], p[1], p[2])?;
            }
            writeln!(f, "    endloop")?;
            writeln!(f, "  endfacet")?;
        }
        writeln!(f, "endsolid cdt")?;
        Ok(())
    }
```
```rust
    fn loop_edges(n: usize, base: usize) -> Vec<[usize; 2]> {
        (0..n).map(|i| [base + i, base + (i + 1) % n]).collect()
    }
```
```rust
    #[test]
    fn cdt_bulk_load_with_hole() {
        // 1) 정점 나열: 외곽 + 홀
        let outer = vec![
            Point2::new(-10.0, -10.0),
            Point2::new(10.0, -10.0),
            Point2::new(10.0, 10.0),
            Point2::new(-10.0, 10.0),
        ];
        let hole = vec![
            Point2::new(-3.0, -3.0),
            Point2::new(3.0, -3.0),
            Point2::new(3.0, 3.0),
            Point2::new(-3.0, 3.0),
        ];

        let mut verts = Vec::new();
        verts.extend_from_slice(&outer);
        let hole_base = verts.len();
        verts.extend_from_slice(&hole);

        // 2) 제약 엣지(외곽 + 홀)
        let mut edges = Vec::new();
        edges.extend(loop_edges(outer.len(), 0));
        edges.extend(loop_edges(hole.len(), hole_base));

        // 3) CDT 생성
        let cdt: CDT<Point2<f64>> = CDT::bulk_load_cdt(verts, edges).expect("bulk load failed");

        // 4) 삼각형 순회 (faces → vertices → 좌표)
        let mut tri_count = 0usize;
        for f in cdt.fixed_inner_faces() {
            let [va, vb, vc] = cdt.face(f).vertices();

            // ⬇ 동적 → 고정 변환
            let (a, b, c) = (va.fix(), vb.fix(), vc.fix());

            let _p0 = cdt.vertex(a).position();
            let _p1 = cdt.vertex(b).position();
            let _p2 = cdt.vertex(c).position();
            tri_count += 1;
        }
        assert!(tri_count > 0);
    }
```
```rust    
    #[test]
    fn cdt_with_constraints_and_hole_to_stl() {
        // 1) 외곽: 20x20 정사각형(시계/반시계 상관 없음)
        let outer = vec![
            Point2::new(-10.0, -10.0),
            Point2::new(10.0, -10.0),
            Point2::new(10.0, 10.0),
            Point2::new(-10.0, 10.0),
        ];

        // 2) 홀: 가운데 6x6 정사각형
        let hole = vec![
            Point2::new(-3.0, -3.0),
            Point2::new(3.0, -3.0),
            Point2::new(3.0, 3.0),
            Point2::new(-3.0, 3.0),
        ];

        // 3) CDT 생성 + 제약 엣지 추가
        let mut cdt: CDT<Point2<f64>> = CDT::default(); //
        add_loop_constraints(&mut cdt, &outer);
        add_loop_constraints(&mut cdt, &hole);

        // 4) 모든 내부 face 순회 → 삼각형 좌표 추출
        //    (faces 는 핸들이고, 각 face 에서 정점 핸들을 얻어 좌표를 읽는다)
        let mut tris_xy: Vec<[Point2<f64>; 3]> = Vec::new();
        for f in cdt.fixed_inner_faces() {
            let [va, vb, vc] = cdt.face(f).vertices();

            // ⬇ 동적 → 고정 변환
            let (a, b, c) = (va.fix(), vb.fix(), vc.fix());

            let _vv = cdt.face(f).vertices(); // [FixedVertexHandle;3]
            let p0 = cdt.vertex(a).position();
            let p1 = cdt.vertex(b).position();
            let p2 = cdt.vertex(c).position();
            tris_xy.push([p0, p1, p2]);
        }

        // 5) 외곽 안쪽 && 홀 바깥만 선별
        let mut kept: Vec<[[f64; 3]; 3]> = Vec::new();
        for tri in tris_xy {
            let c = Point2::new(
                (tri[0].x + tri[1].x + tri[2].x) / 3.0,
                (tri[0].y + tri[1].y + tri[2].y) / 3.0,
            );
            if point_in_polygon(c, &outer) && !point_in_polygon(c, &hole) {
                kept.push([
                    [tri[0].x, tri[0].y, 0.0],
                    [tri[1].x, tri[1].y, 0.0],
                    [tri[2].x, tri[2].y, 0.0],
                ]);
            }
        }

        // 6) STL 로 저장 (프로젝트 루트/target/ 에 생김)
        let path = "target/cdt_constraints_with_hole.stl";
        save_stl_ascii(path, &kept).expect("stl write failed");

        // 간단 검증: 한 개 이상 삼각형이 있어야 함
        assert!(
            !kept.is_empty(),
            "no triangles kept after clipping with hole"
        );
    }
```
```rust
    fn p(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }
```
```rust
    fn push_loop(verts: &mut Vec<Point2<f64>>, edges: &mut Vec<[usize; 2]>, poly: &[Point2<f64>]) {
        let base = verts.len();
        verts.extend_from_slice(poly);
        let n = poly.len();
        for i in 0..n {
            edges.push([base + i, base + (i + 1) % n]);
        }
    }
```
```rust
    fn approx_eq2(a: Point2<f64>, b: Point2<f64>, eps: f64) -> bool {
        (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps
    }
```
```rust
    fn find_vertex_handle(
        cdt: &CDT<Point2<f64>>,
        target: Point2<f64>,
        eps: f64,
    ) -> Option<FixedVertexHandle> {
        for vh_dyn in cdt.vertices() {
            let fh = vh_dyn.fix();
            if approx_eq2(cdt.vertex(fh).position(), target, eps) {
                return Some(fh);
            }
        }
        None
    }
```
```rust
    // 레이 캐스팅 포인트-인-폴리곤 (경계 포함을 true 취급)
    fn point_in_poly(pt: Point2<f64>, poly: &[Point2<f64>]) -> bool {
        let mut inside = false;
        let n = poly.len();
        for i in 0..n {
            let a = poly[i];
            let b = poly[(i + 1) % n];

            // 경계 위
            let cross = (b.x - a.x) * (pt.y - a.y) - (b.y - a.y) * (pt.x - a.x);
            let on_seg = (cross.abs() < 1e-12)
                && (pt.x - a.x).min(pt.x - b.x) <= 1e-12
                && (pt.x - a.x).max(pt.x - b.x) >= -1e-12
                && (pt.y - a.y).min(pt.y - b.y) <= 1e-12
                && (pt.y - a.y).max(pt.y - b.y) >= -1e-12;
            if on_seg {
                return true;
            }
            let intersect = ((a.y > pt.y) != (b.y > pt.y))
                && (pt.x < (b.x - a.x) * (pt.y - a.y) / (b.y - a.y + 0.0) + a.x);
            if intersect {
                inside = !inside;
            }
        }
        inside
    }
```
```rust
    // ASCII STL 저장 (z=0)
    fn write_ascii_stl(path: &str, tris: &[[Point2<f64>; 3]]) -> IoResult<()> {
        let mut f = File::create(path)?;
        writeln!(f, "solid cdt")?;
        for [a, b, c] in tris {
            let ux = b.x - a.x;
            let uy = b.y - a.y;
            let vx = c.x - a.x;
            let vy = c.y - a.y;
            let nx = 0.0;
            let ny = 0.0;
            let nz = ux * vy - uy * vx; // 2D에서 z 성분만 의미
            writeln!(f, "  facet normal {} {} {}", nx, ny, nz)?;
            writeln!(f, "    outer loop")?;
            writeln!(f, "      vertex {} {} 0", a.x, a.y)?;
            writeln!(f, "      vertex {} {} 0", b.x, b.y)?;
            writeln!(f, "      vertex {} {} 0", c.x, c.y)?;
            writeln!(f, "    endloop")?;
            writeln!(f, "  endfacet")?;
        }
        writeln!(f, "endsolid cdt")?;
        Ok(())
    }
```
```rust
    // ---------- 테스트 ----------
    #[test]
    fn insert_edge_next_to_hole_and_export_stl() -> Result<(), InsertionError> {
        // 1) 바깥 사각형과 안쪽 사각형(홀)
        let outer = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let hole = vec![p(3.0, 3.0), p(7.0, 3.0), p(7.0, 7.0), p(3.0, 7.0)];

        // 2) CDT 로딩 (버텍스/엣지 일괄)
        let mut verts = Vec::new();
        let mut edges = Vec::new();
        push_loop(&mut verts, &mut edges, &outer);
        push_loop(&mut verts, &mut edges, &hole);
        let mut cdt: CDT<Point2<f64>> = CDT::bulk_load_cdt(verts.clone(), edges)?;

        // 3) "홀 옆" 제약 에지 삽입: (홀의 우상단 7,7) ↔ (바깥 우상단 10,10)
        let eps = 1e-9;
        let va = find_vertex_handle(&cdt, p(7.0, 7.0), eps).expect("hole vertex not found");
        let vb = find_vertex_handle(&cdt, p(10.0, 10.0), eps).expect("outer vertex not found");

        //    ※ spade 2.x: 제약 에지 추가는 add_constraint(va, vb)
        cdt.add_constraint(va, vb);

        // 4) “도메인(outer 안) - (hole 안)” 영역의 삼각형만 수집
        let mut kept: Vec<[Point2<f64>; 3]> = Vec::new();
        for fh in cdt.fixed_inner_faces() {
            let [a, b, c] = cdt.face(fh).vertices(); // 동적 핸들
            let (a, b, c) = (a.fix(), b.fix(), c.fix()); // 고정 핸들로 변환
            let pa = cdt.vertex(a).position();
            let pb = cdt.vertex(b).position();
            let pc = cdt.vertex(c).position();
            let centroid = p((pa.x + pb.x + pc.x) / 3.0, (pa.y + pb.y + pc.y) / 3.0);

            if point_in_poly(centroid, &outer) && !point_in_poly(centroid, &hole) {
                kept.push([pa, pb, pc]);
            }
        }

        // 5) STL 로 저장 (테스트 실행 위치에 파일 생성)
        write_ascii_stl("target/cdt_edge_near_hole.stl", &kept).expect("stl write failed");

        // 눈으로 확인 편의 출력
        println!("kept tris = {}", kept.len());
        assert!(!kept.is_empty());
        Ok(())
    }
```
```rust
    // 경계 포함을 true 취급하는 2D 레이 캐스팅
    fn has_edge(cdt: &CDT<Point2<f64>>, a: FixedVertexHandle, b: FixedVertexHandle) -> bool {
        for fh in cdt.fixed_inner_faces() {
            let [v0, v1, v2] = cdt.face(fh).vertices();
            let ids = [v0.fix(), v1.fix(), v2.fix()];
            for k in 0..3 {
                let u = ids[k];
                let v = ids[(k + 1) % 3];
                if (u == a && v == b) || (u == b && v == a) {
                    return true;
                }
            }
        }
        false
    }
```
```rust
    #[test]
    fn insert_two_vertices_inside_and_connect_with_constraint() -> Result<(), InsertionError> {
        // 1) 바깥/홀
        let outer = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let hole = vec![p(3.0, 3.0), p(7.0, 3.0), p(7.0, 7.0), p(3.0, 7.0)];

        // 2) CDT 생성
        let mut verts = Vec::new();
        let mut edges = Vec::new();
        push_loop(&mut verts, &mut edges, &outer);
        push_loop(&mut verts, &mut edges, &hole);
        let mut cdt: CDT<Point2<f64>> = CDT::bulk_load_cdt(verts.clone(), edges)?;

        // 3) 내부 정점 두 개 삽입 (y=8 수평선: 홀 y=7 위라 교차 없음)
        let va_pos = p(2.0, 8.0);
        let vb_pos = p(8.0, 8.0);
        let va = cdt.insert(va_pos)?; // FixedVertexHandle
        let vb = cdt.insert(vb_pos)?;

        // 4) 두 정점을 제약 에지로 연결
        cdt.add_constraint(va, vb); // 교차 시 Err(IntersectingExistingConstraint)

        // 5) 실제로 제약 에지가 들어갔는지 확인(근접 좌표로 매칭)
        let found = has_edge(&cdt, va, vb);
        println!("constraint edge inserted? {}", found);
        assert!(found, "제약 에지가 CDT 에 없습니다.");

        // 6) “outer 내부 && hole 외부” 삼각형만 STL 로 저장
        let mut kept: Vec<[Point2<f64>; 3]> = Vec::new();
        for fh in cdt.fixed_inner_faces() {
            let [a, b, c] = cdt.face(fh).vertices();
            let (a, b, c) = (a.fix(), b.fix(), c.fix());
            let pa = cdt.vertex(a).position();
            let pb = cdt.vertex(b).position();
            let pc = cdt.vertex(c).position();
            let centroid = p((pa.x + pb.x + pc.x) / 3.0, (pa.y + pb.y + pc.y) / 3.0);
            if point_in_poly(centroid, &outer) && !point_in_poly(centroid, &hole) {
                kept.push([pa, pb, pc]);
            }
        }
        write_ascii_stl("target/cdt_interior_edge.stl", &kept).expect("stl write failed");
        println!("kept tris = {}", kept.len());
        Ok(())
    }
}
```
## 🛠 삼각형 리스트를 Mesh로 변환하는 함수
```rust
/// 삼각형 리스트를 Mesh로 변환
/// vertices: 정점 좌표 배열
/// tris: 삼각형 인덱스 배열 (각 삼각형은 [v0, v1, v2])
pub fn triangles_to_mesh(vertices: Vec<Point3D>, tris: Vec<[u32; 3]>) -> Mesh {
    let mut faces = Vec::<[u32; 4]>::with_capacity(tris.len());
    for t in tris {
        // 삼각형은 [v0, v1, v2, v2] 형태로 저장
        faces.push([t[0], t[1], t[2], t[2]]);
    }
    Mesh::new(vertices, faces)
}
```


## 🔍 사용 예시
```rust
fn main() {
    // 정점 정의
    let vertices = vec![
        Point3D::new(0.0, 0.0, 0.0),
        Point3D::new(1.0, 0.0, 0.0),
        Point3D::new(0.0, 1.0, 0.0),
    ];

    // 삼각형 정의 (0,1,2)
    let tris = vec![[0, 1, 2]];

    // Mesh 변환
    let mesh = triangles_to_mesh(vertices, tris);

    println!("Mesh vertices: {}", mesh.vertices.len()); // 3
    println!("Mesh faces: {}", mesh.faces.len());       // 1
}
```

## 🎯 요약
- 삼각형 리스트를 Mesh 구조로 변환할 때는 각 삼각형을 [v0, v1, v2, v2] 형태로 저장해야 합니다.
- 이렇게 하면 Mesh 내부에서 삼각형과 사각형을 동일한 자료구조로 다룰 수 있습니다.
- 위 함수는 on_tri_list_to_mesh와 동일한 역할을 하지만, 이름을 더 직관적으로 바꿔서 사용하기 편리하게 만들었습니다.

---


## 🛠 테스트 코드 예시
```rust
#[cfg(test)]
mod tests {
    use super::MeshBuilder;
    use spade::Point2;

    #[test]
    fn test_simple_square_no_hole() {
        // 외곽: 10x10 정사각형
        let outer = vec![
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            Point2::new(10.0, 10.0),
            Point2::new(0.0, 10.0),
        ];
        let mesh = MeshBuilder::from_loops(outer, vec![]);
        println!("Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.faces.len());
        assert!(mesh.faces.len() > 0);
    }
```
```rust
    #[test]
    fn test_square_with_hole() {
        // 외곽: 20x20
        let outer = vec![
            Point2::new(-10.0, -10.0),
            Point2::new(10.0, -10.0),
            Point2::new(10.0, 10.0),
            Point2::new(-10.0, 10.0),
        ];
        // 홀: 6x6
        let hole = vec![
            Point2::new(-3.0, -3.0),
            Point2::new(3.0, -3.0),
            Point2::new(3.0, 3.0),
            Point2::new(-3.0, 3.0),
        ];
        let mesh = MeshBuilder::from_loops(outer, vec![hole]);
        println!("Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.faces.len());
        assert!(mesh.faces.len() > 0);
    }
```
```rust
    #[test]
    fn test_multiple_holes() {
        // 외곽: 30x30
        let outer = vec![
            Point2::new(-15.0, -15.0),
            Point2::new(15.0, -15.0),
            Point2::new(15.0, 15.0),
            Point2::new(-15.0, 15.0),
        ];
        // 홀1: 작은 사각형
        let hole1 = vec![
            Point2::new(-5.0, -5.0),
            Point2::new(-2.0, -5.0),
            Point2::new(-2.0, -2.0),
            Point2::new(-5.0, -2.0),
        ];
        // 홀2: 또 다른 사각형
        let hole2 = vec![
            Point2::new(2.0, 2.0),
            Point2::new(5.0, 2.0),
            Point2::new(5.0, 5.0),
            Point2::new(2.0, 5.0),
        ];
        let mesh = MeshBuilder::from_loops(outer, vec![hole1, hole2]);
        println!("Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.faces.len());
        assert!(mesh.faces.len() > 0);
    }
```
```rust
    #[test]
    fn test_triangle_outer() {
        // 외곽: 삼각형
        let outer = vec![
            Point2::new(0.0, 0.0),
            Point2::new(5.0, 0.0),
            Point2::new(2.5, 5.0),
        ];
        let mesh = MeshBuilder::from_loops(outer, vec![]);
        println!("Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.faces.len());
        assert!(mesh.faces.len() > 0);
    }
}
```

## ✅ 테스트 시나리오
- 단순 정사각형 → 외곽만 있는 경우 Mesh 생성
- 정사각형 + 홀 → 외곽과 내부 홀을 포함한 Mesh 생성
- 여러 개의 홀 → 외곽과 두 개 이상의 홀을 포함한 Mesh 생성
- 삼각형 외곽 → 외곽이 삼각형인 경우 Mesh 생성

## 🎯 요약
이 테스트 세트로 MeshBuilder::from_loops의 다양한 입력 케이스를 검증할 수 있습니다.
- 외곽만 있는 경우
- 홀 포함된 경우
- 여러 홀 포함된 경우
- 삼각형 외곽


---

# constrained 추가


```rust
#[cfg(test)]
mod test_constrained {

    use spade::Point2;
    use nurbslib::core::mesh_builder::MeshBuilder;

    #[test]
    fn test_square_with_hole_and_constraint() {
        // 외곽: 20x20 정사각형
        let outer = vec![
            Point2::new(-10.0, -10.0),
            Point2::new(10.0, -10.0),
            Point2::new(10.0, 10.0),
            Point2::new(-10.0, 10.0),
        ];

        // 홀: 가운데 6x6 정사각형
        let hole = vec![
            Point2::new(-3.0, -3.0),
            Point2::new(3.0, -3.0),
            Point2::new(3.0, 3.0),
            Point2::new(-3.0, 3.0),
        ];

        // 제약 에지: 홀 꼭짓점 ↔ 외곽 꼭짓점 연결
        let constraints = vec![
            (Point2::new(3.0, 3.0), Point2::new(10.0, 10.0)),
            (Point2::new(-3.0, -3.0), Point2::new(-10.0, -10.0)),
        ];

        // Mesh 생성
        let mesh = MeshBuilder::from_loops_with_constraints(outer, vec![hole], constraints);

        println!("Vertices: {}", mesh.vertices.len());
        println!("Faces: {}", mesh.faces.len());

        // 검증: 삼각형이 하나 이상 생성되어야 함
        assert!(mesh.faces.len() > 0);
    }
```
```rust
    #[test]
    fn test_multiple_constraints() {
        // 외곽: 30x30 정사각형
        let outer = vec![
            Point2::new(-15.0, -15.0),
            Point2::new(15.0, -15.0),
            Point2::new(15.0, 15.0),
            Point2::new(-15.0, 15.0),
        ];

        // 홀: 가운데 10x10 정사각형
        let hole = vec![
            Point2::new(-5.0, -5.0),
            Point2::new(5.0, -5.0),
            Point2::new(5.0, 5.0),
            Point2::new(-5.0, 5.0),
        ];

        // 여러 제약 에지 추가
        let constraints = vec![
            (Point2::new(-5.0, -5.0), Point2::new(-15.0, -15.0)),
            (Point2::new(5.0, 5.0), Point2::new(15.0, 15.0)),
            (Point2::new(-5.0, 5.0), Point2::new(-15.0, 15.0)),
        ];

        let mesh = MeshBuilder::from_loops_with_constraints(outer, vec![hole], constraints);

        println!("Vertices: {}", mesh.vertices.len());
        println!("Faces: {}", mesh.faces.len());

        assert!(mesh.faces.len() > 0);
    }
```
```rust
    #[test]
    fn test_triangle_outer_with_constraint() {
        // 외곽: 삼각형
        let outer = vec![
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            Point2::new(5.0, 8.0),
        ];

        // 제약 에지: 삼각형 내부에 대각선 추가
        let constraints = vec![
            (Point2::new(0.0, 0.0), Point2::new(5.0, 8.0)),
        ];

        let mesh = MeshBuilder::from_loops_with_constraints(outer, vec![], constraints);

        println!("Vertices: {}", mesh.vertices.len());
        println!("Faces: {}", mesh.faces.len());

        assert!(mesh.faces.len() > 0);
    }
}
```

## ✅ 테스트 시나리오
- 정사각형 + 홀 + 제약 에지 → 홀과 외곽을 연결하는 제약 에지 포함 Mesh 생성
- 정사각형 + 홀 + 여러 제약 에지 → 여러 제약 에지를 삽입하여 Mesh 생성
- 삼각형 외곽 + 제약 에지 → 삼각형 외곽에 내부 대각선 제약 에지 추가

---

