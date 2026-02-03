


# 📘 TextLog 
- Lightweight Hierarchical Debug Logger for Geometry Kernels
- TextLog은 NURBS, Bézier, Matrix, Transform 등
    기하학 기반 엔진을 디버깅하기 위해 설계된 계층형 텍스트 로그 시스템이다.
- Rust의 println! 기반 디버깅은
    - 구조화되지 않고
    - 복잡한 데이터 타입을 보기 어렵고
    - 테스트 환경에서 사용하기 불편하다.
- TextLog는 이런 문제를 해결하기 위해 만들어졌다.

## ✨ 주요 특징
- ✔ 1. 계층형(indent) 구조
    - 함수 호출, 내부 계산, 반복문 등
    - 논리적 깊이에 따라 자동으로 들여쓰기가 적용된다.
- ✔ 2. 기하 타입 전용 출력 지원
- 다음 타입을 보기 좋은 형태로 출력한다:
    - Point2D, Point3D, Point4D
    - Vector2D, Vector3D
    - Matrix, Matrix4
    - BezierCurve, BezierSurface
    - NurbsCurve, NurbsIsoCurveData
    - KnotVector, Interval
    - Color
- ✔ 3. 테스트 환경에서 사용 가능
- TextLog.output은 String이므로  
    테스트에서 로그를 캡처하고 검증할 수 있다.
- ✔ 4. 대규모 데이터 출력 기능
    - 1D 벡터 chunk 출력
    - 2D 행렬 row/column 제한 출력
    - head/tail summary 출력
    - 생략된 부분 자동 표시
- ✔ 5. 디버깅 중간 상태를 명확하게 보여줌
- 특히 NURBS 커널에서 흔한 문제:
    - 잘못된 w(weight)
    - 비정상적인 knot vector
    - 변환 행렬 오류
    - 제어점 배열 인덱스 문제
    - 평면성/곡률 검사 중간 값
- 이런 것들을 즉시 눈으로 확인할 수 있다.

## 🧱 구조
```rust
pub struct TextLog {
    pub output: String,
    pub indent_level: usize,
    pub indent_size: usize,
}
```
- output — 최종 로그 문자열
- indent_level — 현재 들여쓰기 깊이
- indent_size — 들여쓰기 간격(기본 4칸)

## 🛠 기본 사용법
```rust
let mut log = TextLog::new();

log.print_line("Start computing curve");
log.push_indent();

log.print_point3d(Point3D::new(1.0, 2.0, 3.0));

log.pop_indent();
println!("{}", log.output);
```

## 출력 예:
```
Start computing curve
    (1.000000, 2.000000, 3.000000)
```


## 🔍 예시: NURBS 곡선 디버깅
```rust
log.print_nurbs_curve(&curve);
```

## 출력:
```
NURBS Curve (dim 3, degree 2)
    Domain: [0.000000, 1.000000]
    Knot Vector:
        k[0] = 0.000000
        k[1] = 0.000000
        k[2] = 0.000000
        k[3] = 1.000000
        k[4] = 1.000000
        k[5] = 1.000000
    ctrl[0]:
        (0.000000, 0.000000, 0.000000, 1.000000)
    ctrl[1]:
        (1.000000, 1.000000, 0.000000, 2.000000)
    ctrl[2]:
        (2.000000, 0.000000, 0.000000, 1.000000)
```

- 이런 출력 덕분에
    - w=2.0이 들어간 문제를 즉시 파악할 수 있었다.

## 📐 예시: 행렬 디버깅
```rust
log.print_matrix4(&xform);
```

- 출력:
```
Matrix4:
    1.000000  0.000000  0.000000  0.000000
    0.000000  1.000000  0.000000  0.000000
    0.000000  0.000000  1.000000  0.000000
    0.000000  0.000000  0.000000  1.000000
```


## 📊 대규모 데이터 출력 예시
- 1D 벡터 요약
```rust
log.print_vector_summary("weights", &curve.weights, 5, 5);
```

- 출력:
```
weights (len = 120):
    head:
        [0]:   1.000000  1.000000  1.000000  1.000000  1.000000
    ... (middle omitted)
    tail:
        [115]: 1.000000  1.000000  1.000000  1.000000  1.000000
```

## 🎯 설계 의도
- NURBS/Bezier/Matrix 기반의 기하 커널은
- 중간 계산이 매우 복잡하고, 디버깅이 어렵다.
- 특히 다음 문제들은 로그 없이는 찾기 어렵다:
    - w(weight) 오류
    - knot vector 불일치
    - 제어점 인덱스 오류
    - 행렬 변환 오류
    - 평면성/곡률 검사 실패
    - SVD/행렬 연산 중 NaN 발생
- TextLog는 이런 문제를 사람이 읽기 좋은 형태로 시각화하기 위해 설계되었다.

--- 
## 소스 코드
```rust
use crate::core::bezier_curve::BezierCurve;
use crate::core::bezier_surface::BezierSurface;
use crate::core::color::Color;
use crate::core::geom::{Point2D, Vector2D};
use crate::core::matrix::Matrix;
use crate::core::matrix4::Matrix4;
use crate::core::nurbs_curve::NurbsCurve;
use crate::core::nurbs_surface::NurbsIsoCurveData;
use crate::core::prelude::{Interval, KnotVector, Point3D, Point4D, Vector3D};
use crate::core::types::Real;
use crate::core::xform::Xform;
use enterpolation::Chain;
use std::fmt::Write;
```
```rust
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

    fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_size)
    }

    pub fn print_str(&mut self, text: &str) {
        let _ = write!(self.output, "{}{}", self.indent(), text);
    }

    pub fn print_line(&mut self, text: &str) {
        let _ = writeln!(self.output, "{}{}", self.indent(), text);
    }

    pub fn print_f32(&mut self, value: f32) {
        self.print_line(&format!("{:.6}", value));
    }

    pub fn print_f64(&mut self, value: Real) {
        self.print_line(&format!("{:.15}", value));
    }

    pub fn print_point2d(&mut self, p: Point2D) {
        self.print_line(&format!("({:.6}, {:.6})", p.x, p.y));
    }

    pub fn print_point3d(&mut self, p: Point3D) {
        self.print_line(&format!("({:.6}, {:.6}, {:.6})", p.x, p.y, p.z));
    }

    pub fn print_vector2d(&mut self, v: Vector2D) {
        self.print_line(&format!("<{:.6}, {:.6}>", v.x, v.y));
    }

    pub fn print_vector3d(&mut self, v: Vector3D) {
        self.print_line(&format!("<{:.6}, {:.6}, {:.6}>", v.x, v.y, v.z));
    }

    pub fn push_indent(&mut self) {
        self.indent_level += 1;
    }

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
        self.print_line(&format!(
            "RGBA({}, {}, {}, {})",
            color.r, color.g, color.b, color.a
        ));
    }

    pub fn print_xform(&mut self, xform: &Xform) {
        self.print_line("Xform 4x4 Matrix:");
        self.push_indent();
        for row in &xform.m {
            let row_str = row
                .iter()
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

    pub fn print_knot_vector(&mut self, kv: &KnotVector) {
        self.print_line("Knot Vector:");
        self.push_indent();
        for (i, k) in kv.knots.iter().enumerate() {
            self.print_line(&format!("k[{}] = {:.6}", i, k));
        }
        self.pop_indent();
    }

    pub fn print_interval(&mut self, interval: Interval) {
        self.print_line(&format!("Domain: [{:.6}, {:.6}]", interval.t0, interval.t1));
    }

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

    pub fn print_bezier_surface(&mut self, surface: &BezierSurface) {
        self.print_line(&format!(
            "Bezier Surface (u_degree {}, v_degree {})",
            surface.u_degree, surface.v_degree
        ));
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

    pub fn print_nurbs_curve(&mut self, curve: &NurbsCurve) {
        self.print_line(&format!(
            "NURBS Curve (dim {}, degree {})",
            curve.dimension, curve.degree
        ));
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

    pub fn print_matrix4(&mut self, m: &Matrix4) {
        self.print_line("Matrix4:");
        self.push_indent();
        for row in &m.m {
            let row_str = row
                .iter()
                .map(|v| format!("{:>10.6}", v))
                .collect::<Vec<_>>()
                .join(" ");
            self.print_line(&row_str);
        }
        self.pop_indent();
    }

    pub fn print_matrix(&mut self, m: &Matrix) {
        self.print_line(&format!("Matrix ({}x{}):", m.rows(), m.cols()));
        self.push_indent();
        for i in 0..m.rows() {
            let mut row_str = String::new();
            for j in 0..m.cols() {
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
    pub fn print_vector(&mut self, label: &str, data: &[Real], max_cols: usize) {
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

    /// Print a 2D matrix with column limit per row
    pub fn print_matrix_with_col_limit(&mut self, label: &str, matrix: &Matrix, max_cols: usize) {
        self.print_line(&format!("{} ({}x{}):", label, matrix.rows(), matrix.cols()));
        self.push_indent();
        for i in 0..matrix.rows() {
            let mut row = vec![];
            for j in 0..matrix.cols() {
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
    pub fn print_matrix_with_row_limit(
        &mut self,
        label: &str,
        matrix: &Matrix,
        max_rows: usize,
        max_cols: usize,
    ) {
        self.print_line(&format!("{} ({}x{}):", label, matrix.rows(), matrix.cols()));
        self.push_indent();

        let rows_to_show = matrix.rows().min(max_rows);
        for i in 0..rows_to_show {
            let mut row = vec![];
            for j in 0..matrix.cols() {
                row.push(matrix.get(i, j));
            }
            self.print_vector(&format!("row[{}]", i), &row, max_cols);
        }

        if matrix.rows() > max_rows {
            self.print_line(&format!(
                "... ({} more rows omitted)",
                matrix.rows() - max_rows
            ));
        }

        self.pop_indent();
    }
}
```
```rust
impl TextLog {
    /// Print vector with summary (head + tail, omit middle)
    pub fn print_vector_summary(&mut self, label: &str, data: &[Real], head: usize, tail: usize) {
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
