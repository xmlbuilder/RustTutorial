# 📘 TextLog 정리
## 1. 기본 구조
- TextLog
    - 문자열, 파일, 버퍼에 출력 가능
    - 들여쓰기(indent)와 출력 상세 수준(LevelOfDetail) 관리
    - 다양한 타입(f32, f64, Point, Vector, Matrix 등)을 포맷팅해서 출력

## 2. 주요 함수 설명
### 🔹 출력 제어
- print_str(&mut self, text: &str)
    - 단순 문자열 출력
- print_line(&mut self, text: &str)
    - 문자열 + 줄바꿈 출력
- print_space(), print_tab(), print_newline()
    - 공백, 탭, 줄바꿈 출력
### 🔹 들여쓰기 관리
- push_indent()
    - 들여쓰기 레벨 증가
- pop_indent()
    - 들여쓰기 레벨 감소
- set_indent_size(size: usize)
    - 들여쓰기 단위(공백 수) 설정
### 🔹 숫자 출력
- print_f32(value: f32)
- print_f64(value: f64)
    - 소수점 자리수 지정하여 출력
### 🔹 기하 구조 출력
- print_point2d(Point2D)
- print_point3d(Point3D)
- print_vector2d(Vector2D)
- print_vector3d(Vector3D)
- print_point4d(Point4D)
    - 좌표/벡터를 보기 좋은 포맷으로 출력
### 🔹 행렬 및 변환
- print_matrix(&Matrix)
    - 행렬 전체 출력
- print_matrix_with_col_limit(&Matrix, max_cols: usize)
    - 한 줄에 출력할 열 개수 제한
- print_matrix4(&Matrix4)
    - 4x4 변환 행렬 출력
- print_xform(&Xform)
    - 변환 행렬 출력
### 🔹 기타 타입
- print_uuid(Uuid)
    - UUID 출력
- print_color(Color)
    - RGBA 색상 출력
- print_knot_vector(KnotVector)
    - NURBS Knot Vector 출력
- print_interval(Interval)
    - Domain 구간 출력

## 3. 고급 구조 출력
- print_bezier_curve(&BezierCurve)
    - Bezier 곡선 제어점 출력
- print_bezier_surface(&BezierSurface)
    - Bezier 곡면 제어점 출력
- print_nurbs_curve(&NurbsCurve)
    - NURBS 곡선 (차원, degree, domain, knot, ctrl) 출력
- print_nurbs_iso_curve(&NurbsIsoCurveData)
    - IsoCurve 데이터 출력

## 4. 대용량 데이터 출력 기능
- Vector 출력 제한
    - 긴 벡터를 max_cols 개씩 끊어서 출력
```rust
print_vector(label, &data, max_cols)
```

- Matrix 출력 제한
    - 행렬의 각 행을 max_cols 개씩 끊어서 출력
```rust
print_matrix_with_col_limit(label, &matrix, max_cols)
```        


## ✨ 요약
- TextLog는 단순 로그가 아니라 기하학적/수치 데이터 출력용 포맷터
- 다양한 타입을 지원하며, 대용량 데이터도 열 제한 기능으로 가독성 있게 출력 가능
- Bezier, NURBS, Matrix, UUID, Color 등 CAD/Geometry 관련 구조체까지 확장 완료

---

## 행(row) 제한과 요약 출력(앞뒤만 보여주고 중간 생략) 기능
대용량 데이터 출력 시 꼭 필요한 기능이라서, 구조와 예시까지 준비했습니다.

## 📘 TextLog 확장: 행 제한 & 요약 출력
### 1. 행(row) 제한 기능
큰 배열이나 행렬을 출력할 때, 지정한 행까지만 보여주고 나머지는 생략 표시(...)를 넣습니다.

```rust
impl TextLog {
    /// Print matrix with row limit
    pub fn print_matrix_with_row_limit(&mut self, label: &str, matrix: &Matrix, max_rows: usize, max_cols: usize) {
        self.print_line(&format!("{} ({}x{}):", label, matrix.rows, matrix.cols));
        self.push_indent();

        let rows_to_show = matrix.rows.min(max_rows);
        for i in 0..rows_to_show {
            let mut row = vec![];
            for j in 0..matrix.cols {
                row.push(matrix.get(i, j));
            }
            self.print_vector(&format!("row[{}]", i), &row, max_cols);
        }

        if matrix.rows > max_rows {
            self.print_line(&format!("... ({} more rows omitted)", matrix.rows - max_rows));
        }

        self.pop_indent();
    }
}
```


### 2. 요약 출력 (앞뒤만 보여주고 중간 생략)
긴 벡터나 배열을 출력할 때, 앞부분과 뒷부분만 보여주고 중간은 ...로 생략합니다.

```rust
impl TextLog {
    /// Print vector with summary (head + tail, omit middle)
    pub fn print_vector_summary(&mut self, label: &str, data: &[f64], head: usize, tail: usize) {
        self.print_line(&format!("{} (len = {}):", label, data.len()));
        self.push_indent();

        let len = data.len();
        if len <= head + tail {
            // 전체 출력
            self.print_vector("all", data, head.max(10));
        } else {
            // 앞부분 출력
            self.print_vector("head", &data[..head], head.max(10));
            // 생략 표시
            self.print_line("... (middle omitted)");
            // 뒷부분 출력
            self.print_vector("tail", &data[len - tail..], tail.max(10));
        }

        self.pop_indent();
    }
}
```


## 🧪 사용 예시
```rust
fn main() {
    let mut log = TextLog::new();

    // 긴 벡터
    let vec = (0..100).map(|i| i as f64).collect::<Vec<_>>();
    log.print_vector_summary("BigVector", &vec, 5, 5);

    // 큰 행렬
    let mut mat = Matrix::new(20, 5);
    for i in 0..20 {
        for j in 0..5 {
            mat.set(i, j, (i * 10 + j) as f64);
        }
    }
    log.print_matrix_with_row_limit("BigMatrix", &mat, 5, 5);

    println!("{}", log.output);
}
```


## 📊 출력 예시
```
BigVector (len = 100):
    head (len = 5):
        [0]:   0.000000   1.000000   2.000000   3.000000   4.000000
    ... (middle omitted)
    tail (len = 5):
       [95]:  95.000000  96.000000  97.000000  98.000000  99.000000
```
```
BigMatrix (20x5):
  row[0] (len = 5):
        [0]:   0.000000   1.000000   2.000000   3.000000   4.000000
  row[1] (len = 5):
       [10]:  10.000000  11.000000  12.000000  13.000000  14.000000
  row[2] (len = 5):
       [20]:  20.000000  21.000000  22.000000  23.000000  24.000000
  row[3] (len = 5):
       [30]:  30.000000  31.000000  32.000000  33.000000  34.000000
  row[4] (len = 5):
       [40]:  40.000000  41.000000  42.000000  43.000000  44.000000
  ... (15 more rows omitted)
```


## ✨ 요약
- 행 제한: 너무 큰 행렬은 지정한 행까지만 출력하고 나머지는 생략
- 요약 출력: 긴 벡터는 앞/뒤만 출력하고 중간은 ... 처리
- 대용량 데이터도 가독성 유지하면서 확인 가능

---

## 소스 코드
```rust
// text_log.rs

use std::fmt::{self, Write};
use std::fs::File;
use std::io::{self, Write as IoWrite};
use std::path::Path;
use enterpolation::Chain;
use crate::core::bezier_curve::BezierCurve;
use crate::core::bezier_surface::BezierSurface;
use crate::core::color::Color;
use crate::core::geom::{Point2D, Vector2D};
use crate::core::matrix4::Matrix4;
use crate::core::matrix::Matrix;
use crate::core::nurbs_curve::NurbsCurve;
use crate::core::nurbs_surface::NurbsIsoCurveData;
use crate::core::prelude::{Interval, KnotVector, Point3D, Point4D, Vector3D};
use crate::core::xform::Xform;

pub struct TextLog {
    pub output: String,
    pub indent_level: usize,
    pub indent_size: usize,
}
```
```rust
impl TextLog {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            indent_size: 4,
        }
    }
```
```rust
    fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_size)
    }
```
```rust
    pub fn print_str(&mut self, text: &str) {
        let _ = write!(self.output, "{}{}", self.indent(), text);
    }
```
```rust
    pub fn print_line(&mut self, text: &str) {
        let _ = writeln!(self.output, "{}{}", self.indent(), text);
    }
```
```rust
    pub fn print_f32(&mut self, value: f32) {
        self.print_line(&format!("{:.6}", value));
    }
```
```rust
    pub fn print_f64(&mut self, value: f64) {
        self.print_line(&format!("{:.15}", value));
    }
```
```rust
    pub fn print_point2d(&mut self, p: Point2D) {
        self.print_line(&format!("({:.6}, {:.6})", p.x, p.y));
    }
```
```rust
    pub fn print_point3d(&mut self, p: Point3D) {
        self.print_line(&format!("({:.6}, {:.6}, {:.6})", p.x, p.y, p.z));
    }
```
```rust
    pub fn print_vector2d(&mut self, v: Vector2D) {
        self.print_line(&format!("<{:.6}, {:.6}>", v.x, v.y));
    }
```
```rust
    pub fn print_vector3d(&mut self, v: Vector3D) {
        self.print_line(&format!("<{:.6}, {:.6}, {:.6}>", v.x, v.y, v.z));
    }
```
```rust
    pub fn push_indent(&mut self) {
        self.indent_level += 1;
    }
```
```rust
    pub fn pop_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
}
```
```rust
impl TextLog {
    pub fn print_color(&mut self, color: Color) {
        self.print_line(&format!("RGBA({}, {}, {}, {})", color.r, color.g, color.b, color.a));
    }
```
```rust
    pub fn print_xform(&mut self, xform: &Xform) {
        self.print_line("Xform 4x4 Matrix:");
        self.push_indent();
        for row in &xform.m {
            let row_str = row.iter()
                .map(|v| format!("{:>10.6}", v))
                .collect::<Vec<_>>()
                .join(" ");
            self.print_line(&row_str);
        }
        self.pop_indent();
    }
}
```
```rust
impl TextLog {
    pub fn print_point4d(&mut self, p: &Point4D) {
        self.print_line(&format!("({:.6}, {:.6}, {:.6}, {:.6})", p.x, p.y, p.z, p.w));
    }
```
```rust
    pub fn print_knot_vector(&mut self, kv: &KnotVector) {
        self.print_line("Knot Vector:");
        self.push_indent();
        for (i, k) in kv.knots.iter().enumerate() {
            self.print_line(&format!("k[{}] = {:.6}", i, k));
        }
        self.pop_indent();
    }
```
```rust
    pub fn print_interval(&mut self, interval: Interval) {
        self.print_line(&format!("Domain: [{:.6}, {:.6}]", interval.t0, interval.t1));
    }
```
```rust
    pub fn print_bezier_curve(&mut self, curve: &BezierCurve) {
        self.print_line(&format!("Bezier Curve (degree {})", curve.degree));
        self.push_indent();
        for (i, p) in curve.ctrl.iter().enumerate() {
            self.print_line(&format!("ctrl[{}]:", i));
            self.push_indent();
            self.print_point4d(&p);
            self.pop_indent();
        }
        self.pop_indent();
    }
```
```rust
    pub fn print_bezier_surface(&mut self, surface: &BezierSurface) {
        self.print_line(&format!("Bezier Surface (u_degree {}, v_degree {})", surface.u_degree, surface.v_degree));
        self.push_indent();
        for (u, row) in surface.ctrl.iter().enumerate() {
            self.print_line(&format!("u[{}]:", u));
            self.push_indent();
            for (v, p) in row.iter().enumerate() {
                self.print_line(&format!("v[{}]:", v));
                self.push_indent();
                self.print_point4d(&p);
                self.pop_indent();
            }
            self.pop_indent();
        }
        self.pop_indent();
    }
```
```rust
    pub fn print_nurbs_curve(&mut self, curve: &NurbsCurve) {
        self.print_line(&format!("NURBS Curve (dim {}, degree {})", curve.dimension, curve.degree));
        self.push_indent();
        self.print_interval(curve.domain);
        self.print_knot_vector(&curve.kv);
        for (i, p) in curve.ctrl.iter().enumerate() {
            self.print_line(&format!("ctrl[{}]:", i));
            self.push_indent();
            self.print_point4d(&p);
            self.pop_indent();
        }
        self.pop_indent();
    }
```
```rust
    pub fn print_nurbs_iso_curve(&mut self, iso: &NurbsIsoCurveData) {
        self.print_line(&format!("NURBS IsoCurve (degree {})", iso.degree));
        self.push_indent();
        self.print_knot_vector(&iso.knot);
        for (i, p) in iso.ctrl.iter().enumerate() {
            self.print_line(&format!("ctrl[{}]:", i));
            self.push_indent();
            self.print_point4d(&p);
            self.pop_indent();
        }
        self.pop_indent();
    }
```
```rust
    pub fn print_matrix4(&mut self, m: &Matrix4) {
        self.print_line("Matrix4:");
        self.push_indent();
        for row in &m.m {
            let row_str = row.iter()
                .map(|v| format!("{:>10.6}", v))
                .collect::<Vec<_>>()
                .join(" ");
            self.print_line(&row_str);
        }
        self.pop_indent();
    }
```
```rust
    pub fn print_matrix(&mut self, m: &Matrix) {
        self.print_line(&format!("Matrix ({}x{}):", m.row_count(), m.col_count()));
        self.push_indent();
        for i in 0..m.row_count() {
            let mut row_str = String::new();
            for j in 0..m.col_count() {
                let idx = m.idx(i as i32, j as i32);
                let val = m.data[idx];
                row_str += &format!("{:>10.6} ", val);
            }
            self.print_line(&row_str.trim_end());
        }
        self.pop_indent();
    }
}
```
```rust
impl TextLog {
    /// Print a 1D vector with a maximum number of columns per line
    pub fn print_vector(&mut self, label: &str, data: &[f64], max_cols: usize) {
        self.print_line(&format!("{} (len = {}):", label, data.len()));
        self.push_indent();
        for (i, chunk) in data.chunks(max_cols).enumerate() {
            let values = chunk
                .iter()
                .map(|v| format!("{:>10.6}", v))
                .collect::<Vec<_>>()
                .join(" ");
            self.print_line(&format!("[{}]: {}", i * max_cols, values));
        }
        self.pop_indent();
    }
```
```rust
    /// Print a 2D matrix with column limit per row
    pub fn print_matrix_with_col_limit(&mut self, label: &str, matrix: &Matrix, max_cols: usize) {
        self.print_line(&format!("{} ({}x{}):", label, matrix.row_count(), matrix.col_count()));
        self.push_indent();
        for i in 0..matrix.row_count() {
            let mut row = vec![];
            for j in 0..matrix.col_count() {
                row.push(matrix.get(i, j));
            }
            self.print_vector(&format!("row[{}]", i), &row, max_cols);
        }
        self.pop_indent();
    }
}
```
```rust
impl TextLog {
    /// Print matrix with row limit
    pub fn print_matrix_with_row_limit(&mut self, label: &str, matrix: &Matrix, max_rows: usize, max_cols: usize) {
        self.print_line(&format!("{} ({}x{}):", label, matrix.row_count(), matrix.col_count()));
        self.push_indent();

        let rows_to_show = matrix.row_count().min(max_rows);
        for i in 0..rows_to_show {
            let mut row = vec![];
            for j in 0..matrix.col_count() {
                row.push(matrix.get(i, j));
            }
            self.print_vector(&format!("row[{}]", i), &row, max_cols);
        }

        if matrix.row_count() > max_rows {
            self.print_line(&format!("... ({} more rows omitted)", matrix.row_count() - max_rows));
        }

        self.pop_indent();
    }
}
```
```rust
impl TextLog {
    /// Print vector with summary (head + tail, omit middle)
    pub fn print_vector_summary(&mut self, label: &str, data: &[f64], head: usize, tail: usize) {
        self.print_line(&format!("{} (len = {}):", label, data.len()));
        self.push_indent();

        let len = data.len();
        if len <= head + tail {
            // 전체 출력
            self.print_vector("all", data, head.max(10));
        } else {
            // 앞부분 출력
            self.print_vector("head", &data[..head], head.max(10));
            // 생략 표시
            self.print_line("... (middle omitted)");
            // 뒷부분 출력
            self.print_vector("tail", &data[len - tail..], tail.max(10));
        }

        self.pop_indent();
    }
}
```

---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::bezier_curve::BezierCurve;
    use nurbslib::core::color::Color;
    use nurbslib::core::geom::{Point2D, Point3D, Vector2D, Vector3D};
    use nurbslib::core::matrix::Matrix;
    use nurbslib::core::prelude::Point4D;
    use nurbslib::core::text_log::TextLog;
    use nurbslib::core::xform::Xform;

    #[test]
    fn case1() {
        let mut log = TextLog::new();

        log.print_f32(3.14159);
        log.print_f64(2.718281828459045);
        log.print_point2d(Point2D { x: 1.0, y: 2.0 });
        log.print_point3d(Point3D { x: 3.0, y: 4.0, z: 5.0 });
        log.print_vector2d(Vector2D { x: -1.0, y: -2.0 });
        log.print_vector3d(Vector3D { x: 0.0, y: 1.0, z: 2.0 });

        println!("{}", log.output);
    }
```
```rust
    #[test]
    fn case2() {
        let mut log = TextLog::new();

        let color = Color { r: 1.0, g: 0.5, b: 1.0, a: 1.0 };

        let xform = Xform::from_cols(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );


        log.print_color(color);
        log.print_xform(&xform);

        println!("{}", log.output);
    }
```
```rust
    #[test]
    fn test_bezier_curve() {
        let mut log = TextLog::new();

        let curve = BezierCurve {
            degree: 3,
            ctrl: vec![
                Point4D { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                Point4D { x: 1.0, y: 2.0, z: 0.0, w: 1.0 },
                Point4D { x: 2.0, y: 2.0, z: 0.0, w: 1.0 },
                Point4D { x: 3.0, y: 0.0, z: 0.0, w: 1.0 },
            ],
        };

        log.print_bezier_curve(&curve);
        println!("{}", log.output);
    }
```
```rust
    #[test]
    fn matrix_print() {
        let mut log = TextLog::new();

        let vec = (0..20).map(|i| i as f64).collect::<Vec<_>>();
        log.print_vector("MyVector", &vec, 5);

        let mut mat = Matrix::with_dims(3, 8);
        for i in 0..3 {
             for j in 0..8 {
                 mat.set(i, j, (i * 10 + j) as f64);
             }
         }
         log.print_matrix_with_col_limit("MyMatrix", &mat, 4);

         println!("{}", log.output);
    }
```
```rust
    #[test]
    fn print_some() {
        let mut log = TextLog::new();

        // 긴 벡터
        let vec = (0..100).map(|i| i as f64).collect::<Vec<_>>();
        log.print_vector_summary("BigVector", &vec, 5, 5);

        // 큰 행렬
        let mut mat = Matrix::with_dims(20, 5);
        for i in 0..20 {
            for j in 0..5 {
                mat.set(i, j, (i * 10 + j) as f64);
            }
        }
        log.print_matrix_with_row_limit("BigMatrix", &mat, 5, 5);

        println!("{}", log.output);
    }

}
```
---
