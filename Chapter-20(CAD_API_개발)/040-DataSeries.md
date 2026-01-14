# DataSeries

아래는 DataSeries 모듈의 전체 함수 및 구조체 정리와 테스트 코드 요약을 표로 정리한 내용입니다.  
실무 문서나 README에 바로 활용할 수 있도록 Markdown 아스키 형식으로 구성했습니다.

## 📦 주요 구조체 및 열거형
| 이름           | 설명                                      |
|----------------|-------------------------------------------|
| `GrpPt2f`       | 2D 점 구조체 (fx, fy)                     |
| `PenType`       | 선 스타일 (Solid, Dash 등)                |
| `SymbolType`    | 심볼 모양 (Rectangle, Ellipse)            |
| `DataSeries`    | 시계열 데이터 및 시각화 속성 포함         |
| `DataSeriesVec` | `&[&DataSeries]` 타입 별칭                |

## 🧩 주요 함수 정리
| 함수 이름                  | 설명                                      |
|---------------------------|-------------------------------------------|
| `new`, `default`          | 기본 생성자                               |
| `with_count`, `set_count` | 초기 크기 설정 및 변경                    |
| `add_point`, `add_points_from_arrays` | 점 추가                         |
| `clear_points`, `ds_clear`| 점 제거                                   |
| `clone_from`, `clone_with_range` | 복사 및 범위 필터링             |
| `get`, `get_count`        | 점 접근 및 개수                           |
| `get_index`, `get_value`  | x값 기반 인덱스 및 보간값 계산            |
| `calc_y_max_min`          | y값의 최대/최소 계산                      |
| `set_y_scale`             | y값 스케일 조정                           |
| `x_arrays`, `y_arrays`    | x/y 배열 추출 (f32)                        |
| `x_arrays_f64`, `y_arrays_f64` | x/y 배열 추출 (f64)               |
| `uniform_dt`              | 등간격 여부 판단                          |
| `float_round`             | 소수점 반올림                             |
| `draw_series`, `save_series_to_png` | 시각화 및 이미지 저장         |


```rust
use crate::core::tarray::TArray;

use plotters::prelude::{
    BLUE, BitMapBackend, ChartBuilder, IntoDrawingArea, IntoFont, LineSeries, RED, WHITE,
};
use std::collections::HashMap;
use std::ops::Range;
```
```rust
#[derive(Clone, Debug, Default)]
pub struct GrpPt2f {
    pub fx: f32,
    pub fy: f32,
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub enum PenType {
    NoPen = 0,
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    CustomDash,
}
```
```rust
#[derive(Clone, Copy, Debug)]
pub enum SymbolType {
    Rectangle = 0,
    Ellipse,
}
```
```rust
#[derive(Clone, Debug)]
pub struct DataSeries {
    pub col: (u32, u32, u32),
    pub title: String,
    pub show_symbol: bool,
    pub pen_type: PenType,
    pub symbol_type: SymbolType,
    pub visible: bool,
    pub symbol_size: i32,
    pub line_thick: i32,
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub points: TArray<GrpPt2f>,
}
```
```rust
impl DataSeries {
    pub fn get(&self, p0: u32) -> GrpPt2f {
        self.points[p0 as usize].clone()
    }
}
```
```rust
impl DataSeries {
    pub fn get_count(&self) -> u32 {
        self.points.len() as u32
    }
}
```
```rust
impl DataSeries {
    pub fn default() -> Self {
        Self {
            col: (128, 0, 0),
            title: "No Title".to_string(),
            show_symbol: false,
            pen_type: PenType::Solid,
            symbol_type: SymbolType::Rectangle,
            visible: true,
            symbol_size: 3,
            line_thick: 2,
            max_x: 0.0,
            max_y: 0.0,
            min_x: 0.0,
            min_y: 0.0,
            points: TArray::new(),
        }
    }
}
```
```rust
impl DataSeries {
    pub fn new() -> Self {
        Self {
            col: (128, 0, 0),
            title: "No Title".to_string(),
            show_symbol: false,
            pen_type: PenType::Solid,
            symbol_type: SymbolType::Rectangle,
            visible: true,
            symbol_size: 3,
            line_thick: 2,
            max_x: 0.0,
            max_y: 0.0,
            min_x: 0.0,
            min_y: 0.0,
            points: TArray::new(),
        }
    }
```
```rust
    pub fn with_count(n: usize) -> Self {
        let mut ds = Self::new();
        ds.set_count(n);
        ds
    }
```
```rust
    pub fn set_count(&mut self, n: usize) {
        self.points.set_size(n);
    }
```
```rust
    pub fn ds_len(ds: &DataSeries) -> usize {
        ds.points.len()
    }
 ```
```rust   
    pub fn ds_is_empty(ds: &DataSeries) -> bool {
        ds.points.len() == 0
    }
```
```rust
    pub fn ds_clear(ds: &mut DataSeries) {
        ds.points.remove_all();
    }
```
```rust   
    pub fn ds_set_count(ds: &mut DataSeries, n: usize) {
        ds.points.set_size(n);
    }
```
```rust   
    pub fn ds_add(ds: &mut DataSeries, x: f32, y: f32) {
        ds.points.append(GrpPt2f { fx: x, fy: y });
    }
```
```rust   
    pub fn uniform_dt(ds: &DataSeries) -> Option<f32> {
        if ds.points.len() >= 2 {
            Some(ds.points[1].fx - ds.points[0].fx)
        } else {
            None
        }
    }
```
```rust   
    pub fn float_round(x: f32, digits: i32) -> f32 {
        let p = 10f32.powi(digits);
        (x * p).round() / p
    }
}
```
```rust   
impl DataSeries {
    pub fn clone_from(&mut self, other: &DataSeries) {
        self.col = other.col;
        self.title = other.title.clone();
        self.show_symbol = other.show_symbol;
        self.pen_type = other.pen_type;
        self.symbol_type = other.symbol_type;
        self.line_thick = other.line_thick;
        self.visible = other.visible;
        self.symbol_size = other.symbol_size;
        self.points = other.points.clone();
    }
```
```rust   
    pub fn clone_with_range(&mut self, other: &DataSeries, x_min: f32, x_max: f32) {
        self.clone_from(other);
        self.points.remove_all();
        for pt in other.points.iter() {
            if pt.fx >= x_min - (f64::EPSILON as f32) && pt.fx <= x_max + (f64::EPSILON as f32) {
                self.points.append(pt.clone());
            }
        }
    }
}
```
```rust   
impl DataSeries {
    pub fn add_point(&mut self, fx: f32, fy: f32) {
        self.points.append(GrpPt2f { fx, fy });
    }
```
```rust   
    pub fn add_points_from_arrays(&mut self, fx: &[f32], fy: &[f32]) {
        self.points.remove_all();
        for (&x, &y) in fx.iter().zip(fy.iter()) {
            self.points.append(GrpPt2f { fx: x, fy: y });
        }
    }
```
```rust   
    pub fn clear_points(&mut self) {
        self.points.remove_all();
    }
```
```rust   
    pub fn calc_y_max_min(&self) -> Option<(f32, f32)> {
        if self.points.is_empty() {
            return None;
        }
        let mut f_min = f32::MAX;
        let mut f_max = f32::MIN;
        for pt in self.points.iter() {
            f_max = f_max.max(pt.fy);
            f_min = f_min.min(pt.fy);
        }
        Some((f_max, f_min))
    }
```
```rust   
    pub fn set_y_scale(&mut self, scale: f32) {
        for pt in self.points.iter_mut() {
            pt.fy *= scale;
        }
    }
}
```
```rust   
impl DataSeries {
    pub fn get_index(&self, x: f32) -> Option<usize> {
        let data = self.points.as_slice();
        if data.is_empty() {
            return None;
        }
        if x <= data[0].fx {
            return Some(0);
        }
        if x >= data[data.len() - 1].fx {
            return Some(data.len() - 1);
        }

        let mut low = 0;
        let mut high = data.len() - 1;
        while low < high {
            let mid = (low + high) / 2;
            if x < data[mid].fx {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        Some(low.saturating_sub(1))
    }
```
```rust   
    pub fn x_arrays(&self) -> Vec<f32> {
        let data = self.points.as_slice();
        if data.is_empty() {
            vec![]
        } else {
            let mut arrays: Vec<f32> = vec![];
            for i in 1..data.len() {
                arrays.push(self.points[i].fx);
            }
            arrays
        }
    }
```
```rust   
    pub fn y_arrays(&self) -> Vec<f32> {
        let data = self.points.as_slice();
        if data.is_empty() {
            vec![]
        } else {
            let mut arrays: Vec<f32> = vec![];
            for i in 1..data.len() {
                arrays.push(self.points[i].fy);
            }
            arrays
        }
    }
```
```rust   
    pub fn x_arrays_f64(&self) -> Vec<f64> {
        let data = self.points.as_slice();
        if data.is_empty() {
            vec![]
        } else {
            let mut arrays: Vec<f64> = vec![];
            for i in 1..data.len() {
                arrays.push(self.points[i].fx as f64);
            }
            arrays
        }
    }
```
```rust   
    pub fn y_arrays_f64(&self) -> Vec<f64> {
        let data = self.points.as_slice();
        if data.is_empty() {
            vec![]
        } else {
            let mut arrays: Vec<f64> = vec![];
            for i in 1..data.len() {
                arrays.push(self.points[i].fy as f64);
            }
            arrays
        }
    }
```
```rust   
    pub fn get_value(&self, x: f32) -> f32 {
        let data = self.points.as_slice();
        if data.is_empty() {
            return 0.0;
        }

        match self.get_index(x) {
            Some(idx) if idx + 1 < data.len() => {
                let pt1 = &data[idx];
                let pt2 = &data[idx + 1];
                if x == pt1.fx {
                    pt1.fy
                } else {
                    let r = ((x - pt1.fx) / (pt2.fx - pt1.fx)).clamp(0.0, 1.0);
                    pt1.fy + r * (pt2.fy - pt1.fy)
                }
            }
            Some(idx) => data[idx].fy,
            None => 0.0,
        }
    }
}
```
```rust   
impl DataSeries {
    pub fn draw_series(ds: &DataSeries) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let (x_min, x_max) = match (ds.points.first(), ds.points.last()) {
            (Some(first), Some(last)) => (first.fx, last.fx),
            _ => return Err("points 배열이 비어 있습니다.".into()),
        };

        let (y_min, y_max) = ds.calc_y_max_min().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(&ds.title, ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(
            ds.points.iter().map(|pt| (pt.fx, pt.fy)),
            &RED,
        ))?;

        Ok(())
    }
```
```rust   
    pub fn save_series_to_png(
        ds: &DataSeries,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let (x_min, x_max) = match (ds.points.first(), ds.points.last()) {
            (Some(first), Some(last)) => (first.fx, last.fx),
            _ => return Err("points 배열이 비어 있습니다.".into()),
        };

        let (y_min, y_max) = ds.calc_y_max_min().unwrap_or((0.0, 1.0));

        let mut chart = ChartBuilder::on(&root)
            .caption(&ds.title, ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart
            .configure_mesh()
            .x_desc("Time") // X축 제목
            .y_desc("Displacement") // Y축 제목
            .axis_desc_style(("sans-serif", 16).into_font()) // 축 제목 폰트
            // .label_style(("sans-serif", 12))               // 눈금 라벨 폰트(옵션)
            .draw()?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(
            ds.points.iter().map(|pt| (pt.fx, pt.fy)),
            &BLUE,
        ))?;

        Ok(())
    }
}
```

```rust   
#[allow(non_snake_case)]
pub type DataSeriesVec<'a> = &'a [&'a DataSeries];
```

---

# 테스트 코드

✅ 테스트 코드 요약
| 테스트 이름                          | 검증 내용                                      | 통과 여부 |
|-------------------------------------|-----------------------------------------------|-----------|
| `test_new_and_with_count`           | 생성자 및 초기 크기 설정                      | ✅        |
| `test_set_count_and_clear`          | 크기 변경 및 점 제거                         | ✅        |
| `test_add_point_and_array`          | 단일/배열 점 추가                            | ✅        |
| `test_clone_and_clone_with_range`   | 전체 복사 및 범위 필터링                     | ✅        |
| `test_get_index_and_value`          | x값 기반 인덱스 및 보간값 계산               | ✅        |
| `test_y_max_min_and_scale`          | y값 최대/최소 및 스케일 조정                 | ✅        |
| `test_x_y_export_image`             | 사인 함수 시각화 및 이미지 저장              | ✅        |

## 📌 시각화 예시
- save_series_to_png 함수는 plotters 라이브러리를 사용하여 .png 이미지로 시각화
- 예제: 사인 함수 $y=\sin (x)$ 를 100개 점으로 시각화 → data_series_y_sine.png


```rust   
#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use nurbslib::core::data_series::DataSeries;

    const EPSILON: f32 = 1e-6;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
```
```rust   
    #[test]
    fn test_new_and_with_count() {
        let ds = DataSeries::new();
        assert_eq!(ds.title, "No Title");
        assert_eq!(ds.points.len(), 0);

        let ds2 = DataSeries::with_count(5);
        assert_eq!(ds2.points.len(), 5);
    }
```
```rust   
    #[test]
    fn test_set_count_and_clear() {
        let mut ds = DataSeries::new();
        ds.set_count(10);
        assert_eq!(ds.points.len(), 10);
        ds.clear_points();
        assert_eq!(ds.points.len(), 0);
    }
```
```rust   
    #[test]
    fn test_add_point_and_array() {
        let mut ds = DataSeries::new();
        ds.add_point(1.0, 2.0);
        assert_eq!(ds.points.len(), 1);
        assert_eq!(ds.points[0].fx, 1.0);
        assert_eq!(ds.points[0].fy, 2.0);

        let fx = vec![1.0, 2.0, 3.0];
        let fy = vec![4.0, 5.0, 6.0];
        ds.add_points_from_arrays(&fx, &fy);
        assert_eq!(ds.points.len(), 3);
        assert_eq!(ds.points[2].fx, 3.0);
        assert_eq!(ds.points[2].fy, 6.0);
    }
```
```rust   
    #[test]
    fn test_clone_and_clone_with_range() {
        let mut ds1 = DataSeries::new();
        ds1.add_point(1.0, 10.0);
        ds1.add_point(2.0, 20.0);
        ds1.add_point(3.0, 30.0);

        let mut ds2 = DataSeries::new();
        ds2.clone_from(&ds1);
        assert_eq!(ds2.points.len(), 3);
        assert_eq!(ds2.points[1].fy, 20.0);

        let mut ds3 = DataSeries::new();
        ds3.clone_with_range(&ds1, 1.5, 2.5);
        assert_eq!(ds3.points.len(), 1);
        assert_eq!(ds3.points[0].fx, 2.0);
    }
```
```rust   
    #[test]
    fn test_get_index_and_value() {
        let mut ds = DataSeries::new();

        ds.add_point(1.0, 10.0);
        ds.add_point(2.0, 20.0);
        ds.add_point(3.0, 30.0);

        let idx = ds.get_index(2.5).unwrap();
        assert_eq!(idx, 1);

        let val = ds.get_value(2.5);
        assert!(approx_eq(val, 25.0));
    }
```
```rust   
    #[test]
    fn test_y_max_min_and_scale() {
        let mut ds = DataSeries::new();
        ds.add_point(0.0, -5.0);
        ds.add_point(1.0, 10.0);
        ds.add_point(2.0, 3.0);

        let (max, min) = ds.calc_y_max_min().unwrap();
        assert!(approx_eq(max, 10.0));
        assert!(approx_eq(min, -5.0));

        ds.set_y_scale(2.0);
        let (max2, min2) = ds.calc_y_max_min().unwrap();
        assert!(approx_eq(max2, 20.0));
        assert!(approx_eq(min2, -10.0));

        let _ = DataSeries::save_series_to_png(&ds, "data_series_y_max_min_and_scale.png");
    }
```
```rust       
    #[test]
    fn test_x_y_export_image() {
        let mut ds = DataSeries::new();
        ds.title = "Sine Value".to_string();
        let n = 100usize; // 등분 개수
        let step = PI / n as f32; // 한 스텝 크기 = π/100
        let y: Vec<f32> = (0..=n) // 0..=n → 101개 점 (0과 π 모두 포함)
            .map(|i| {
                let t = i as f32 * step; // t ∈ [0, π]
                t.sin() // 여기서 원하는 함수로 바꾸면 됨
            })
            .collect();
        let x: Vec<f32> = (0..=n).map(|i| i as f32 * step).collect();

        for i in 0..=n {
            ds.add_point(x[i], y[i]);
        }

        DataSeries::save_series_to_png(&ds, "data_series_y_sine.png").expect("save series png");
    }
}
```

---
