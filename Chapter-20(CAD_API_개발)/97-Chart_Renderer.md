# Chart Renderer
## Chart renderer documentation
이 문서는 내부 모듈로 사용하는 Rust chart renderer의 구조, 설정, 기능, 사용법을 단계별로 정리합니다.  
폰트는 선택적으로 활성화할 수 있으며, 폰트가 없을 경우에도 차트(그리드, 축, 시리즈)는 정상 렌더링됩니다.  

## Overview and architecture
- 목적: 2D 라인 시리즈를 고정 크기 RGBA8 버퍼에 렌더링하고 PNG로 저장합니다.
- 구성 요소:
    - Canvas: RGBA8 버퍼 기반 저수준 드로잉(라인, 사각형, 채움, 블렌드).
    - Series: 이름, 색상, 두께, 심볼 등을 가진 데이터 시리즈.
    - Ticks: 축 눈금(주/보조 눈금, 표기 소수점 자리) 자동 생성.
    - Renderer: 여백/스타일/라벨/폰트/뷰포트 관리 및 전체 렌더 파이프라인.
    - FontTTF(옵션): rusttype 기반 텍스트 길이 계산/그리기.


## Cargo.toml 설정
```
rusttype = "0.9.3"
```

## Core modules
### Canvas: 저수준 드로잉 API
- Label: 버퍼
- 채널 4(RGBA8) pixels: Vec<u8>로 이루어진 고정 크기 캔버스.
- Label: 주요 함수
- blend(x, y, Color): src-over 블렌딩(프리멀티 스타일 근사).
- set(x, y, Color): 단순 픽셀 설정.
- hline, vline, rect, fill_rect, line, tri_fill: 기본 도형 그리기.
- save_png(path): PNG 파일 저장.
- Label: 좌표
- 좌상단(0,0), x는 오른쪽으로 증가, y는 아래로 증가.
### Series: 데이터 시리즈
- Label: 필드
- name: String, pts: Vec<Point { x: f32, y: f32 }>
- color: Color, thickness: i32, show_symbol: bool, symbol: Circle|Square, symbol_size: i32
- Label: 용도
- 선 그리기와 심볼 렌더링. pts는 화면 좌표로 변환되어 연결됩니다.
### Ticks: 축 눈금 생성
- Label: nice number 알고리즘
- 범위에서 사람 친화적인 간격(1, 2, 5, 10의 스케일)을 선택.
- Label: 출력
- majors: Vec<f64>, minors: Vec<f64>, decimals: i32, step_major, step_minor
### Renderer: 전체 렌더링 파이프라인
- Label: 스타일과 레이아웃
- 여백 left/right/top/bottom, 배경 bg_out/bg_in, 그리드 grid_major/minor, 축/텍스트 색상 axis/text
- 라벨: title, xlabel, ylabel
- Label: 뷰포트와 좌표 변환
- 내부 영역: (vx, vy, vw, vh)
- 좌표 변환: X(x: f64) -> i32, Y(y: f64) -> i32 (상하 반전 처리 포함)
- Label: 렌더 순서
- 데이터 경계 추정 및 패딩
- 축 눈금 계산(tx, ty)
- 폰트가 있으면 자동 여백 보정
- 내부 배경 채우기
- 보조/주 그리드 그리기
- 외곽 박스 그리기
- 시리즈 선/심볼 렌더링
- 라벨/타이틀(폰트 활성 시) 렌더링

## Fonts and conditional compilation
### FontTTF 구현(옵션)
- Label: 로드
- FontTTF::load_from_file(Path, pixel_height) -> Option<Self>
- Label: 측정/출력
- text_width(&str) -> i32, text_height() -> i32
- draw_text(&mut Canvas, x, y, &str, Color) (baseline 기준으로 출력)
- Label: 자동 여백
- y축 라벨 최대 폭, x축 라벨/제목 높이를 기준으로 left/bottom/top 보정

## Step-by-step usage
### 1) 데이터 준비
- Label: 시리즈 작성
- let mut s1 = Series::default();
- s1.name = "Sine".into(); s1.color = Color::rgba(30,144,255,255);
- for i in 0..200 { let x = i as f32 * 0.05; s1.pts.push(Point { x, y: (x * 2.0).sin() }); }
### 2) 렌더러 구성
- Label: 기본 설정
- let mut r = Renderer::default();
- r.title = "Demo Chart".into(); r.xlabel = "X".into(); r.ylabel = "Y".into();
- r.attach(&s1);
### 3) 폰트(선택)
- Label: feature 활성 빌드
- cargo run --features ttf
- Label: 로드 및 연결
- let font = FontTTF::load_from_file(Path::new("fonts/Roboto-Regular.ttf"), 18).unwrap();
- r.font = Some(&font); (Renderer 내부에 #[cfg(feature = "ttf")]로 선언된 필드)
### 4) 렌더 및 저장
- Label: 렌더 호출
- let canvas = r.render(800, 480);
- Label: 저장
- canvas.save_png("chart.png")?;

## Troubleshooting
- Label: **폰트 라벨이 겹침**
- 자동 여백 보정이 적용되지만 좁은 해상도에서는 겹칠 수 있습니다. left/bottom/top을 수동으로 늘려주세요.
- Label: “저해상도에서 라인이 끊겨 보임”
- thickness를 키우거나 해상도를 늘리세요. 내장 브레젠험 라인은 픽셀 기반입니다.

## Performance and extensibility
- Label: 성능 팁
- 대형 캔버스에서는 라인/채움 루프가 비용이 큽니다. 필요 시 blend 최소화, 라인 배치 최적화가 유용합니다.
- Label: 확장 아이디어
- 범례 박스, 축 라벨 회전(Y축 세로), NaN/빈 구간 스킵, 여러 y축, 영역(area) 채우기, 산점도 포인트 렌더링.
- Label: 좌표 스케일링
- 현재는 선형 스케일입니다. 로그축이나 커스텀 스케일은 X/Y 변환 함수만 교체하면 됩니다.

---
## 라벨 회전
Y축 라벨을 세로 방향으로 회전해서 출력하는 로직을 추가.  
Rust에서는 rusttype을 사용하면 글리프 좌표를 직접 제어할 수 있기 때문에, 회전된 텍스트도 구현 가능합니다.

### 🎯 목표
- Y축 라벨(ylabel)을 세로 방향으로 출력
- 기존 draw_text()는 수평 출력 → 새로 draw_text_rotated() 함수 추가
- 회전은 90도 시계 방향 기준으로 글리프 좌표를 변환

## 🧱 구현 방식
### 1. FontTTF에 회전 텍스트 함수 추가
```rust
pub fn draw_text_rotated(&self, canvas: &mut Canvas, x: i32, y: i32, s: &str, col: Color) {
    let scale = rusttype::Scale::uniform(self.scale_px);
    let mut pen_y = y as f32;

    for ch in s.chars() {
        let gid = self.font.glyph(ch).scaled(scale);
        let h = gid.h_metrics();
        let positioned = gid.positioned(rusttype::point(x as f32, pen_y));

        if let Some(bb) = positioned.pixel_bounding_box() {
            positioned.draw(|gx, gy, a| {
                let px = bb.min.x + gx as i32;
                let py = bb.min.y + gy as i32;

                // 회전: (px, py) → (py', -px') 기준점 기준
                let dx = px - x;
                let dy = py - y;
                let rx = x - dy;
                let ry = y + dx;

                if a > 0.0 {
                    let mut c = col;
                    c.a = ((col.a as f32) * a).round().clamp(0.0, 255.0) as u8;
                    canvas.blend(rx, ry, c);
                }
            });
        }
        pen_y += h.advance_width;
    }
}
```

- 기준점 (x, y)에서 각 글자 위치를 계산하고, (dx, dy)를 90도 회전:

$$
(x',y')=(x-dy,y+dx)
$$

- pen_y는 글자 간 간격을 수직으로 누적

### 2. Renderer에서 Y축 라벨 출력 시 회전 적용

```rust
if !self.ylabel.is_empty() {
    let th = font.text_height();
    let x = self.vx - 8; // 기준점 x
    let y = self.vy + self.vh / 2 - font.text_width(&self.ylabel) / 2; // 기준점 y
    font.draw_text_rotated(&mut canvas, x, y, &self.ylabel, self.text);
}
```

- 기준점은 Y축 중앙
- text_width()는 회전 시 높이로 쓰이므로 위치 계산에 사용

## ✅ 결과
- Y축 라벨이 세로 방향으로 회전되어 출력
- 기존 라벨과 겹치지 않도록 vx 여백을 충분히 확보해야 함
- draw_text_rotated()는 기존 draw_text()와 병행 사용 가능

---
## 샘플 이미지

![Chart Renderer](/image/chart_renderer.png)

---

## 소스 코드
```rust
// chart_renderer.rs

use std::cmp::{max, min};
use std::f64;
use std::path::Path;

use image::{ImageFormat, ColorType};
use crate::core::image::{ImageBuffer, ImgErr};

// ---------------------------
// Color and RGBA helper
// ---------------------------
#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

```
```rust
impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
```
```rust
// ---------------------------
// Canvas backed by your Image
// channels = 4 (RGBA8)
// ---------------------------
#[derive(Clone, Debug)]
pub struct Canvas {
    pub w: i32,
    pub h: i32,
    pub img: ImageBuffer, // channels=4
}
```
```rust
impl Canvas {
    pub fn new(w: i32, h: i32, bg: Color) -> Self {
        let mut img = ImageBuffer {
            width: w as u32,
            height: h as u32,
            channels: 4,
            pixels: vec![0u8; (w * h * 4) as usize],
        };
        // clear to bg
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 4) as usize;
                img.pixels[i + 0] = bg.r;
                img.pixels[i + 1] = bg.g;
                img.pixels[i + 2] = bg.b;
                img.pixels[i + 3] = bg.a;
            }
        }
        Self { w, h, img }
    }
```
```rust
    #[inline]
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.w && y < self.h
    }
```
```rust
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, c: Color) {
        if !self.in_bounds(x, y) {
            return;
        }
        let i = ((y as u32 * self.img.width + x as u32) * 4) as usize;
        self.img.pixels[i + 0] = c.r;
        self.img.pixels[i + 1] = c.g;
        self.img.pixels[i + 2] = c.b;
        self.img.pixels[i + 3] = c.a;
    }
```
```rust
    // src-over blend (premultiplied-style, like C++)
    #[inline]
    pub fn blend(&mut self, x: i32, y: i32, s: Color) {
        if !self.in_bounds(x, y) {
            return;
        }
        let i = ((y as u32 * self.img.width + x as u32) * 4) as usize;
        let dr = self.img.pixels[i + 0];
        let dg = self.img.pixels[i + 1];
        let db = self.img.pixels[i + 2];
        let da = self.img.pixels[i + 3];
        let sa = s.a as u32;

        // outA
        let out_a = sa + ((da as u32) * (255 - sa) + 127) / 255;
        let blend_c = |sc: u8, dc: u8| -> u8 {
            (((sc as u32) * sa + (dc as u32) * (255 - sa) + 127) / 255) as u8
        };
        let or_ = blend_c(s.r, dr);
        let og = blend_c(s.g, dg);
        let ob = blend_c(s.b, db);

        self.img.pixels[i + 0] = or_;
        self.img.pixels[i + 1] = og;
        self.img.pixels[i + 2] = ob;
        self.img.pixels[i + 3] = out_a as u8;
    }
```
```rust
    pub fn hline(&mut self, mut x1: i32, mut x2: i32, y: i32, c: Color, th: i32) {
        if y < 0 || y >= self.h {
            return;
        }
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        x1 = x1.max(0);
        x2 = x2.min(self.w - 1);
        for yy in -(th / 2)..=(th / 2) {
            let Y = y + yy;
            if Y < 0 || Y >= self.h {
                continue;
            }
            for x in x1..=x2 {
                self.blend(x, Y, c);
            }
        }
    }
```
```rust
    pub fn vline(&mut self, x: i32, mut y1: i32, mut y2: i32, c: Color, th: i32) {
        if x < 0 || x >= self.w {
            return;
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }
        y1 = y1.max(0);
        y2 = y2.min(self.h - 1);
        for xx in -(th / 2)..=(th / 2) {
            let X = x + xx;
            if X < 0 || X >= self.w {
                continue;
            }
            for y in y1..=y2 {
                self.blend(X, y, c);
            }
        }
    }
```
```rust
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Color, th: i32) {
        self.hline(x, x + w - 1, y, c, th);
        self.hline(x, x + w - 1, y + h - 1, c, th);
        self.vline(x, y, y + h - 1, c, th);
        self.vline(x + w - 1, y, y + h - 1, c, th);
    }
```
```rust
    pub fn fill_rect(&mut self, mut x: i32, mut y: i32, W: i32, H: i32, c: Color) {
        let x2 = min(self.w, x + W);
        let y2 = min(self.h, y + H);
        x = max(0, x);
        y = max(0, y);
        for j in y..y2 {
            for i in x..x2 {
                self.blend(i, j, c);
            }
        }
    }
```
```rust
    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, c: Color, th: i32) {
        let plot = |this: &mut Canvas, x: i32, y: i32| {
            for dy in -(th / 2)..=(th / 2) {
                for dx in -(th / 2)..=(th / 2) {
                    this.blend(x + dx, y + dy, c);
                }
            }
        };
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            plot(self, x0, y0);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
```
```rust
    pub fn tri_fill(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, c: Color) {
        let mut edge = |xa: i32, ya: i32, xb: i32, yb: i32, y: i32, out_x: &mut f64| {
            if ya == yb {
                *out_x = xa as f64;
                return;
            }
            *out_x = xa as f64 + (xb - xa) as f64 * (y - ya) as f64 / (yb - ya) as f64;
        };
        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
            std::mem::swap(&mut x0, &mut x1);
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
            std::mem::swap(&mut x1, &mut x2);
        }
        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
            std::mem::swap(&mut x0, &mut x1);
        }
        for y in y0..=y2 {
            let mut xa = 0f64;
            let mut xb = 0f64;
            if y < y1 {
                edge(x0, y0, x1, y1, y, &mut xa);
                edge(x0, y0, x2, y2, y, &mut xb);
            } else {
                edge(x1, y1, x2, y2, y, &mut xa);
                edge(x0, y0, x2, y2, y, &mut xb);
            }
            if xa > xb {
                std::mem::swap(&mut xa, &mut xb);
            }
            let a = xa.floor() as i32;
            let b = xb.ceil() as i32;
            for x in a..=b {
                self.blend(x, y, c);
            }
        }
    }
```
```rust
    pub fn save_png<P: AsRef<Path>>(&self, path: P) -> Result<(), ImgErr> {
        image::save_buffer_with_format(
            path.as_ref(),
            &self.img.pixels,
            self.img.width,
            self.img.height,
            ColorType::Rgba8,
            ImageFormat::Png,
        )
            .map_err(ImgErr::Image)?;
        Ok(())
    }
}
```
```rust
// ---------------------------
// Optional font: rusttype-based
// When font is None, labels are skipped.
// ---------------------------

pub struct FontTTF {
    font: rusttype::Font<'static>,
    scale_px: f32,
    ascent_px: f32,
    line_height_px: f32,
}
```
```rust
impl FontTTF {
    pub fn load_from_file(path: &Path, pixel_height: i32) -> Option<Self> {
        let data = std::fs::read(path).ok()?;
        let font = rusttype::Font::try_from_vec(data)?;
        let scale = rusttype::Scale::uniform(pixel_height as f32);
        let vmetrics = font.v_metrics(scale);
        Some(Self {
            font,
            scale_px: pixel_height as f32,
            ascent_px: vmetrics.ascent,
            line_height_px: (vmetrics.ascent - vmetrics.descent + vmetrics.line_gap),
        })
    }
```
```rust
    pub fn text_width(&self, s: &str) -> i32 {
        let scale = rusttype::Scale::uniform(self.scale_px);
        let mut x = 0.0f32;
        for ch in s.chars() {
            let gid = self.font.glyph(ch).scaled(scale);
            let h = gid.h_metrics();
            x += h.advance_width;
        }
        x.round() as i32
    }
```
```rust    
    pub fn text_height(&self) -> i32 {
        self.line_height_px.round() as i32
    }
```
```rust
    pub fn draw_text(&self, canvas: &mut Canvas, x: i32, y: i32, s: &str, col: Color) {
        // baseline similar to C++: y is top, add ascent
        let baseline = y + self.ascent_px.round() as i32;
        let scale = rusttype::Scale::uniform(self.scale_px);
        let mut pen_x = x as f32;

        for ch in s.chars() {
            let gid = self.font.glyph(ch).scaled(scale).positioned(rusttype::point(pen_x, baseline as f32));
            if let Some(bb) = gid.pixel_bounding_box() {
                gid.draw(|gx, gy, a| {
                    let px = bb.min.x + gx as i32;
                    let py = bb.min.y + gy as i32;
                    if a > 0.0 {
                        let mut c = col;
                        c.a = ((col.a as f32) * a).round().clamp(0.0, 255.0) as u8;
                        canvas.blend(px, py, c);
                    }
                });
            }
            pen_x += gid.unpositioned().h_metrics().advance_width;
        }
    }
```
```rust
    pub fn draw_text_rotated(&self, canvas: &mut Canvas, x: i32, y: i32, s: &str, col: Color) {
        let scale = rusttype::Scale::uniform(self.scale_px);
        let mut pen_y = y as f32;

        for ch in s.chars() {
            let gid = self.font.glyph(ch).scaled(scale);
            let h = gid.h_metrics();
            let positioned = gid.positioned(rusttype::point(x as f32, pen_y));

            if let Some(bb) = positioned.pixel_bounding_box() {
                positioned.draw(|gx, gy, a| {
                    let px = bb.min.x + gx as i32;
                    let py = bb.min.y + gy as i32;

                    // 회전: (px, py) → (py', -px') 기준점 기준
                    let dx = px - x;
                    let dy = py - y;
                    let rx = x - dy;
                    let ry = y + dx;

                    if a > 0.0 {
                        let mut c = col;
                        c.a = ((col.a as f32) * a).round().clamp(0.0, 255.0) as u8;
                        canvas.blend(rx, ry, c);
                    }
                });
            }
            pen_y += h.advance_width;
        }
    }
}
```
```rust
// ---------------------------
// Chart data structures
// ---------------------------
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
```
```rust
#[derive(Clone, Debug)]
pub struct Series {
    pub name: String,
    pub pts: Vec<Point>,
    pub color: Color,
    pub thickness: i32,
    pub show_symbol: bool,
    pub symbol: Symbol,
    pub symbol_size: i32,
}
```
```rust
impl Default for Series {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            pts: Vec::new(),
            color: Color::rgba(30, 144, 255, 255),
            thickness: 2,
            show_symbol: false,
            symbol: Symbol::Circle,
            symbol_size: 3,
        }
    }
}
```
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    Circle,
    Square,
}
```
```rust
#[derive(Clone, Debug)]
pub struct Ticks {
    pub vmin: f64,
    pub vmax: f64,
    pub step_major: f64,
    pub step_minor: f64,
    pub decimals: i32,
    pub majors: Vec<f64>,
    pub minors: Vec<f64>,
}
```
```rust
fn nice_num(x: f64, round: bool) -> f64 {
    let expv = (x.abs()).log10().floor();
    let f = x / 10f64.powf(expv);
    let nf = if round {
        if f < 1.5 {
            1.0
        } else if f < 3.0 {
            2.0
        } else if f < 7.0 {
            5.0
        } else {
            10.0
        }
    } else {
        if f <= 1.0 {
            1.0
        } else if f <= 2.0 {
            2.0
        } else if f <= 5.0 {
            5.0
        } else {
            10.0
        }
    };
    nf * 10f64.powf(expv)
}
```
```rust
fn compute_ticks(vmin: f64, vmax: f64, pixel_span: i32, target_px: i32) -> Ticks {
    let mut vmin = vmin;
    let mut vmax = vmax;
    if vmax < vmin {
        std::mem::swap(&mut vmin, &mut vmax);
    }
    if (vmax - vmin).abs() < f64::EPSILON {
        vmax += 1.0;
        vmin -= 1.0;
    }
    let target_count = max(2, pixel_span / max(30, target_px));
    let range = nice_num(vmax - vmin, false);
    let step = nice_num(range / (target_count as f64), true);
    let nice_min = (vmin / step).floor() * step;
    let nice_max = (vmax / step).ceil() * step;

    let decimals = if step < 1.0 {
        (((-step.log10()).ceil() as i32) + 1).clamp(0, 8)
    } else {
        0
    };
    let mut majors = Vec::new();
    let mut minors = Vec::new();
    let eps = 1e-12;
    let mut v = nice_min;
    while v <= nice_max + eps {
        majors.push(v);
        v += step;
    }
    let mN = 5;
    for i in 0..(majors.len().saturating_sub(1)) {
        let a = majors[i];
        let b = majors[i + 1];
        let d = (b - a) / (mN as f64);
        for k in 1..mN {
            let mv = a + (k as f64) * d;
            if mv >= vmin - eps && mv <= vmax + eps {
                minors.push(mv);
            }
        }
    }

    Ticks {
        vmin,
        vmax,
        step_major: step,
        step_minor: step / 5.0,
        decimals,
        majors,
        minors,
    }
}
```
```rust
// ---------------------------
// Renderer
// ---------------------------
pub struct Renderer<'a> {
    // margins
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,

    // style
    pub bg_out: Color,
    pub bg_in: Color,
    pub grid_major: Color,
    pub grid_minor: Color,
    pub axis: Color,
    pub text: Color,

    // labels
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,


    pub font: Option<&'a FontTTF>,

    // data
    pub series: Vec<&'a Series>,

    // internal
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    tx: Ticks,
    ty: Ticks,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
}
```
```rust
impl<'a> Default for Renderer<'a> {
    fn default() -> Self {
        Self {
            left: 100,
            right: 40,
            top: 30,
            bottom: 45,
            bg_out: Color::rgba(245, 245, 245, 255),
            bg_in: Color::rgba(255, 255, 255, 255),
            grid_major: Color::rgba(190, 190, 190, 255),
            grid_minor: Color::rgba(220, 220, 220, 255),
            axis: Color::rgba(20, 20, 20, 255),
            text: Color::rgba(20, 20, 20, 255),
            title: String::new(),
            xlabel: String::new(),
            ylabel: String::new(),
            font: None,
            series: Vec::new(),
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
            tx: Ticks {
                vmin: 0.0,
                vmax: 1.0,
                step_major: 0.1,
                step_minor: 0.02,
                decimals: 0,
                majors: Vec::new(),
                minors: Vec::new(),
            },
            ty: Ticks {
                vmin: 0.0,
                vmax: 1.0,
                step_major: 0.1,
                step_minor: 0.02,
                decimals: 0,
                majors: Vec::new(),
                minors: Vec::new(),
            },
            vx: 0,
            vy: 0,
            vw: 1,
            vh: 1,
        }
    }
}
```
```rust
impl<'a> Renderer<'a> {
    pub fn attach(&mut self, s: &'a Series) {
        self.series.push(s);
    }

    fn compute_bounds(&mut self) {
        self.xmin = f64::INFINITY;
        self.ymin = f64::INFINITY;
        self.xmax = -f64::INFINITY;
        self.ymax = -f64::INFINITY;
        for s in &self.series {
            for p in &s.pts {
                self.xmin = self.xmin.min(p.x as f64);
                self.xmax = self.xmax.max(p.x as f64);
                self.ymin = self.ymin.min(p.y as f64);
                self.ymax = self.ymax.max(p.y as f64);
            }
        }
        if !(self.xmin < self.xmax) {
            self.xmin -= 1.0;
            self.xmax += 1.0;
        }
        if !(self.ymin < self.ymax) {
            self.ymin -= 1.0;
            self.ymax += 1.0;
        }
        let dx = self.xmax - self.xmin;
        let dy = self.ymax - self.ymin;
        self.xmin -= dx * 0.02;
        self.xmax += dx * 0.02;
        self.ymin -= dy * 0.02;
        self.ymax += dy * 0.02;
    }
```
```rust
    #[inline]
    fn X(&self, x: f64) -> i32 {
        self.vx + ((x - self.xmin) * (self.vw as f64) / (self.xmax - self.xmin).max(1e-12)).round() as i32
    }
```
```rust    
    #[inline]
    fn Y(&self, y: f64) -> i32 {
        self.vy + self.vh - ((y - self.ymin) * (self.vh as f64) / (self.ymax - self.ymin).max(1e-12)).round() as i32
    }
```
```rust
    fn fmt(v: f64, dec: i32) -> String {
        if v.abs() > 1e6 || (v.abs() > 0.0 && v.abs() < 1e-4) {
            format!("{:.*e}", dec.max(0) as usize, v)
        } else {
            format!("{:.*}", dec.max(0) as usize, v)
        }
    }
```
```rust
    pub fn render(&mut self, W: i32, H: i32) -> Canvas {
        let mut L = self.left;
        let mut R = self.right;
        let mut T = self.top;
        let mut B = self.bottom;

        self.compute_bounds();
        self.tx = compute_ticks(self.xmin, self.xmax, (W - L - R).max(1), 90);
        self.ty = compute_ticks(self.ymin, self.ymax, (H - T - B).max(1), 60);

        // auto margins using font
        if let Some(font) = self.font {
            let mut max_yw = 0;
            for v in &self.ty.majors {
                let s = Self::fmt(*v, self.ty.decimals);
                max_yw = max(max_yw, font.text_width(&s));
            }
            L = max(L, 8 + max_yw + 8);
            let mut needB = 6 + font.text_height();
            if !self.xlabel.is_empty() {
                needB += 4 + font.text_height();
            }
            B = max(B, needB);
            let needT = if !self.title.is_empty() {
                font.text_height() + 8
            } else {
                0
            };
            T = max(T, needT);
        }

        let mut canvas = Canvas::new(W, H, self.bg_out);
        self.vx = L;
        self.vy = T;
        self.vw = (W - L - R).max(1);
        self.vh = (H - T - B).max(1);

        // inner background
        canvas.fill_rect(self.vx, self.vy, self.vw, self.vh, self.bg_in);

        // grid
        for &v in &self.tx.minors {
            let x = self.X(v);
            canvas.vline(x, self.vy, self.vy + self.vh, self.grid_minor, 1);
        }
        for &v in &self.ty.minors {
            let y = self.Y(v);
            canvas.hline(self.vx, self.vx + self.vw, y, self.grid_minor, 1);
        }
        for &v in &self.tx.majors {
            let x = self.X(v);
            canvas.vline(x, self.vy, self.vy + self.vh, self.grid_major, 1);
        }
        for &v in &self.ty.majors {
            let y = self.Y(v);
            canvas.hline(self.vx, self.vx + self.vw, y, self.grid_major, 1);
        }

        // border
        canvas.rect(self.vx, self.vy, self.vw, self.vh, self.axis, 2);

        // series
        for s in &self.series {
            if s.pts.len() < 1 {
                continue;
            }
            for i in 0..(s.pts.len() - 1) {
                let p0 = &s.pts[i];
                let p1 = &s.pts[i + 1];
                canvas.line(
                    self.X(p0.x as f64),
                    self.Y(p0.y as f64),
                    self.X(p1.x as f64),
                    self.Y(p1.y as f64),
                    s.color,
                    s.thickness,
                );
            }
            if s.show_symbol {
                for p in &s.pts {
                    let cx = self.X(p.x as f64);
                    let cy = self.Y(p.y as f64);
                    let r = s.symbol_size.max(1);
                    match s.symbol {
                        Symbol::Square => {
                            canvas.fill_rect(cx - r, cy - r, 2 * r, 2 * r, s.color);
                        }
                        Symbol::Circle => {
                            for yy in -r..=r {
                                for xx in -r..=r {
                                    if xx * xx + yy * yy <= r * r {
                                        canvas.blend(cx + xx, cy + yy, s.color);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // labels (optional if font provided)
        if let Some(font) = self.font {
            // X labels
            let in_range = |v: f64, lo: f64, hi: f64| {
                let eps = 1e-12;
                v >= lo - eps && v <= hi + eps
            };
            let mut last_r = -1_000_000i32;
            for &v in &self.tx.majors {
                if !in_range(v, self.xmin, self.xmax) {
                    continue;
                }
                let s = Self::fmt(v, self.tx.decimals);
                let tw = font.text_width(&s);
                let x = self.X(v) - tw / 2;
                let y = self.vy + self.vh + 6;
                if x > last_r + 6 {
                    font.draw_text(&mut canvas, x, y, &s, self.text);
                    last_r = x + tw;
                }
            }
            if !self.xlabel.is_empty() {
                let tw = font.text_width(&self.xlabel);
                font.draw_text(
                    &mut canvas,
                    self.vx + self.vw / 2 - tw / 2,
                    self.vy + self.vh + 6 + font.text_height(),
                    &self.xlabel,
                    self.text,
                );
            }

            // Y labels (avoid overlap scanning from bottom)
            let mut last_b = -1_000_000i32;
            for v in self.ty.majors.iter().rev() {
                if !in_range(*v, self.ymin, self.ymax) {
                    continue;
                }
                let s = Self::fmt(*v, self.ty.decimals);
                let tw = font.text_width(&s);
                let th = font.text_height();
                let x = self.vx - tw - 8;
                let y = self.Y(*v) - th / 2;
                if y > last_b + th / 2 {
                    font.draw_text(&mut canvas, x, y, &s, self.text);
                    last_b = y;
                }
            }

            if !self.ylabel.is_empty() {
                let tw = font.text_width(&self.ylabel);
                font.draw_text(
                    &mut canvas,
                    self.vx - tw - 12,
                    self.vy - font.text_height() / 2 - 12,
                    &self.ylabel,
                    self.text,
                );
            }

            // if !self.ylabel.is_empty() {
            //     let th = font.text_height();
            //     let x = self.vx - 8; // 기준점 x
            //     let y = self.vy + self.vh / 2 - font.text_width(&self.ylabel) / 2; // 기준점 y
            //     font.draw_text_rotated(&mut canvas, x, y, &self.ylabel, self.text);
            // }



            if !self.title.is_empty() {
                let tw = font.text_width(&self.title);
                font.draw_text(
                    &mut canvas,
                    self.vx + self.vw / 2 - tw / 2,
                    self.vy - font.text_height() - 4,
                    &self.title,
                    self.text,
                );
            }
        }

        canvas
    }
}
```

---

## 테스트 코드
```rust
#[cfg(test)]
mod tests{
    use std::path::Path;
    use nurbslib::core::chart_renderer::{Color, FontTTF, Point, Renderer, Series};

    fn create_image() -> Result<(), Box<dyn std::error::Error>> {
        let mut s1 = Series::default();
        s1.name = "Sine".into();
        s1.color = Color::rgba(30, 144, 255, 255);
        for i in 0..200 {
            let x = i as f32 * 0.05;
            s1.pts.push(Point { x, y: (x * 2.0).sin() });
        }

        let mut r = Renderer::default();
        r.title = "Demo Chart".into();
        r.xlabel = "X".into();
        r.ylabel = "Y".into();

        // Optional font
        let font = FontTTF::load_from_file(Path::new("asset/fonts/NotoSans-Light.ttf"), 18).expect("font");
        r.font = Some(&font);

        r.attach(&s1);
        let canvas = r.render(800, 480);

        // Save PNG via image crate (already handled inside Canvas::save_png if preferred)
        image::save_buffer_with_format(
            "asset/chart.png",
            &canvas.img.pixels,
            canvas.img.width,
            canvas.img.height,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )?;

        Ok(())
    }
```
```rust    #[test]
    fn image_test()
    {
        create_image().expect("image test");
    }

}
```
