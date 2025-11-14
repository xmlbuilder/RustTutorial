# Image
## 🖼️ Rust 이미지 처리 모듈 문서화 및 점검
이 모듈은 다양한 이미지 버퍼 구조체(Image, ImageBuffer, DicomImageBuffer, PictureData)를 정의하고,  
이미지 로딩, 저장, 픽셀 접근, 필터링, 변환 등의 기능을 제공합니다.

## 📦 주요 구조체 및 기능 요약
### 1. Image
- 일반 이미지 버퍼 구조체
- RGBA, RGB, Grayscale 지원
- 주요 필드:
- width, height: 이미지 크기
- channels: 채널 수 (1, 3, 4)
- pixels: 픽셀 데이터 (u8 배열)

### 🧩 주요 메서드
| 메서드 이름         | 설명                                                        |
|---------------------|-------------------------------------------------------------|
| new_gray            | 그레이스케일 이미지 생성 (채널=1)                          |
| gray_intensity      | (x,y) 위치의 밝기 계산 (RGB → Gray 변환 포함)               |
| set_gray            | (x,y)에 밝기 값 설정 (채널 수에 따라 RGB 설정)              |
| load                | 파일에서 이미지 로드 (RGBA8로 통일)                         |
| save                | 현재 이미지 버퍼를 파일로 저장                              |
| get_pixel /         | (x,y) 위치의 픽셀 값 읽기 / 쓰기                            |
| set_pixel           |                                                             |
| flipped_vertical    | 수직으로 뒤집은 이미지 반환                                 |
| to_grayscale        | RGB(A) → Grayscale 변환                                     |
| alpha_channel       | 알파 채널만 추출 (없으면 255로 채움)                        |
| swizzle_bgr         | RGB ↔ BGR 스위즐                                            |
| invert              | 채널 반전 (255 - 값)                                        |
| conv3x3_gray        | 3x3 커널 기반 컨볼루션 (Grayscale)                          |
| emboss              | 엠보싱 효과 적용                                            |
| sobel               | 소벨 엣지 검출                                              |
| gaussian_blur       | 가우시안 블러 (채널별 적용)                                 |



## 2. ImageBuffer
- Image의 간단 래퍼
- 주요 기능:
    - from_file: 파일 로드
    - gray_intensity: 밝기 계산
    - set_gray: 밝기 설정
    - save_png: PNG 저장

## 3. DicomImageBuffer
- DICOM 스타일 8-bit 그레이 이미지
- 주요 기능:
    - new_from_bytes: 바이트 배열로 생성
    - get_pixel_mut: 픽셀 접근 (mutable)
    - gray_intensity: 밝기 계산

## 4. PictureData<T>
- 2D 격자에 Arc<T> 저장
- 주요 기능:
    - new: 행/열 기반 생성
    - at, at_mut: 위치 기반 접근

## 5. 유틸리티 함수

| 함수 이름 | 설명 |
|-----------|------|
| `set_rgb` | RGB 이미지 버퍼에 (x, y) 위치의 픽셀을 지정된 R, G, B 값으로 설정합니다. 좌표가 유효하지 않으면 무시됩니다. |
| `draw_line` | Bresenham 알고리즘을 사용하여 RGB 이미지 버퍼에 두 점 사이의 선을 그립니다. 선은 검은색으로 설정됩니다. |
| `draw_point` | RGB 이미지 버퍼에 (x, y) 위치를 중심으로 3x3 빨간색 점을 그립니다. 시각적으로 강조된 점 표현에 사용됩니다. |
| `save_2d_points_as_bmp` | 2D 좌표 배열과 선 연결 정보를 기반으로 BMP 이미지를 생성하고 저장합니다. 점은 빨간색, 선은 검은색으로 표시됩니다. |
| `save_polylines_as_bmp` | 여러 개의 폴리라인(점 배열)을 BMP 이미지로 저장합니다. 각 선분은 검은색으로 그려지며, 좌표 범위에 따라 자동 스케일링됩니다. |


## ✅ 점검 및 개선 포인트
### 1. Image::conv3x3_gray
- ✅ 커널 정규화 옵션 좋음
- ❗ at() 클로저는 경계 보정이 잘 되어 있지만, 성능 최적화를 위해 unsafe 접근 고려 가능
### 2. Image::gaussian_blur
- ✅ 커널 생성 로직 정확함
- ❗ alpha 채널을 블러링하지 않음 → 선택적 블러링 옵션 추가 가능
### 3. Image::save
- ❗ 확장자가 없거나 잘못된 경우 PNG로 저장됨 → 사용자에게 경고 메시지 추가 고려
### 4. OnPictureData<T>
- ✅ Arc<T>를 격자에 저장하는 구조는 병렬 처리에 적합
- ❗ idx()에서 assert! 사용 → Result 기반 오류 처리로 변경하면 안정성 향상


---


## 소스
```rust
use crate::math::point2d::Point2D;
use image::{ColorType, ImageFormat};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum ImgErr {
    #[error("image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("invalid pixel coordinate ({x},{y})")]
    OOB { x: u32, y: u32 },
    #[error("mismatched channels (expected {exp}, got {got})")]
    ChMismatch { exp: u8, got: usize },
    #[error("invalid buffer size")]
    Buf,
}
```
```rust
#[derive(Clone, Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub channels: u8, // 1, 3, 또는 4
    pub pixels: Vec<u8>,
}
```
```rust
impl Image {
    #[inline]
    pub fn new_gray(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            channels: 1,
            pixels: vec![0u8; (width * height) as usize],
        }
    }
```
```rust
    /// (x,y) 위치의 그레이 강도를 리턴 (채널 수에 따라 계산)
    #[inline]
    pub fn gray_intensity(&self, x: u32, y: u32) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        let idx = ((y * self.width + x) * self.channels as u32) as usize;
        match self.channels {
            1 => self.pixels[idx] as f32,
            3 | 4 => {
                let r = self.pixels[idx] as f32;
                let g = self.pixels[idx + 1] as f32;
                let b = self.pixels[idx + 2] as f32;
                0.299 * r + 0.587 * g + 0.114 * b
            }
            _ => 0.0,
        }
    }
```
```rust
    /// (x,y)에 그레이 값 쓰기 (channels=1이면 그대로, 3/4면 RGB 모두 동일값, A=255 유지/세팅)
    #[inline]
    pub fn set_gray(&mut self, x: u32, y: u32, v: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * self.channels as u32) as usize;
        match self.channels {
            1 => {
                self.pixels[idx] = v;
            }
            3 => {
                self.pixels[idx] = v;
                self.pixels[idx + 1] = v;
                self.pixels[idx + 2] = v;
            }
            4 => {
                self.pixels[idx] = v;
                self.pixels[idx + 1] = v;
                self.pixels[idx + 2] = v;
                self.pixels[idx + 3] = 255;
            }
            _ => {}
        }
    }
```
```rust
    /// 파일 확장자에 맞춰 로드. 기본은 RGBA8로 통일(다루기 쉬움).
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImgErr> {
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        Ok(Self {
            width: rgba.width(),
            height: rgba.height(),
            channels: 4,
            pixels: rgba.into_raw(),
        })
    }
```
```rust
    /// 현재 버퍼를 파일로 저장 (확장자 보고 포맷 결정).
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImgErr> {
        let p = path.as_ref();
        let fmt = match p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase()
            .as_str()
        {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "bmp" => ImageFormat::Bmp,
            "tga" => ImageFormat::Tga,
            _ => ImageFormat::Png,
        };
        let ct = match self.channels {
            1 => ColorType::L8,
            3 => ColorType::Rgb8,
            4 => ColorType::Rgba8,
            _ => return Err(ImgErr::Buf),
        };
        image::save_buffer_with_format(p, &self.pixels, self.width, self.height, ct, fmt)?;
        Ok(())
    }
```
```rust
    #[inline]
    fn idx(&self, x: u32, y: u32) -> Result<usize, ImgErr> {
        if x >= self.width || y >= self.height {
            return Err(ImgErr::OOB { x, y });
        }
        Ok(((y * self.width + x) * self.channels as u32) as usize)
    }
```
```rust
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<&[u8], ImgErr> {
        let i = self.idx(x, y)?;
        Ok(&self.pixels[i..i + self.channels as usize])
    }
```
```rust
    pub fn set_pixel(&mut self, x: u32, y: u32, pix: &[u8]) -> Result<(), ImgErr> {
        if pix.len() != self.channels as usize {
            return Err(ImgErr::ChMismatch {
                exp: self.channels,
                got: pix.len(),
            });
        }
        let i = self.idx(x, y)?;
        self.pixels[i..i + pix.len()].copy_from_slice(pix);
        Ok(())
    }
```
```rust
    /// 수직 플립한 새 버퍼 반환
    pub fn flipped_vertical(&self) -> Self {
        let mut out = self.clone();
        let row_bytes = (self.width * self.channels as u32) as usize;

        for y in 0..self.height / 2 {
            let y2 = self.height - 1 - y;
            let i1 = (y * self.width) as usize * self.channels as usize;
            let i2 = (y2 * self.width) as usize * self.channels as usize;

            // 두 줄이 겹치지 않도록 안전하게 분리
            let (start, end) = if i1 < i2 {
                let (start, rest) = out.pixels.split_at_mut(i2);
                let a = &mut start[i1..i1 + row_bytes];
                let b = &mut rest[..row_bytes];
                (a, b)
            } else {
                let (start, rest) = out.pixels.split_at_mut(i1);
                let b = &mut start[i2..i2 + row_bytes];
                let a = &mut rest[..row_bytes];
                (a, b)
            };

            start.swap_with_slice(end);
        }

        out
    }
```
```rust
    /// RGB(A) → Gray(1채널, 0..255)
    pub fn to_grayscale(&self) -> Self {
        match self.channels {
            1 => self.clone(),
            3 | 4 => {
                let mut g = Vec::with_capacity((self.width * self.height) as usize);
                let ch = self.channels as usize;
                for p in self.pixels.chunks(ch) {
                    let r = p[0] as f32;
                    let gch = p[1] as f32;
                    let b = p[2] as f32;
                    let y = 0.299 * r + 0.587 * gch + 0.114 * b;
                    g.push(y.clamp(0.0, 255.0) as u8);
                }
                Self {
                    width: self.width,
                    height: self.height,
                    channels: 1,
                    pixels: g,
                }
            }
            _ => self.clone(),
        }
    }
```
```rust
    /// 알파 채널만 추출 (없으면 255로 채운 단일 채널)
    pub fn alpha_channel(&self) -> Self {
        let mut a = Vec::with_capacity((self.width * self.height) as usize);
        match self.channels {
            4 => {
                for p in self.pixels.chunks(4) {
                    a.push(p[3]);
                }
            }
            3 | 1 => {
                a.resize((self.width * self.height) as usize, 255);
            }
            _ => {}
        }
        Self {
            width: self.width,
            height: self.height,
            channels: 1,
            pixels: a,
        }
    }
```
```rust
    /// BGR 스위즐 (RGB/ RGBA만)
    pub fn swizzle_bgr(&self) -> Self {
        if self.channels == 3 || self.channels == 4 {
            let ch = self.channels as usize;
            let mut out = self.pixels.clone();
            for px in out.chunks_mut(ch) {
                px.swap(0, 2); // R <-> B
            }
            Self {
                width: self.width,
                height: self.height,
                channels: self.channels,
                pixels: out,
            }
        } else {
            self.clone()
        }
    }
```
```rust
    /// 채널 반전 (모든 바이트 255 - v)
    pub fn invert(&self) -> Self {
        let mut out = self.pixels.clone();
        for v in &mut out {
            *v = 255u8.saturating_sub(*v);
        }
        Self {
            width: self.width,
            height: self.height,
            channels: self.channels,
            pixels: out,
        }
    }
```
```rust
    /// 3x3 컨볼루션(그레이스케일 입력) - 커널 합 정규화 optional
    pub fn conv3x3_gray(&self, kernel: [[f32; 3]; 3], normalize: bool, bias: f32) -> Self {
        let gray = if self.channels == 1 {
            self.clone()
        } else {
            self.to_grayscale()
        };
        let w = gray.width as i32;
        let h = gray.height as i32;
        let mut out = vec![0u8; (w * h) as usize];

        let mut ksum = 0.0f32;
        for r in kernel {
            for v in r {
                ksum += v;
            }
        }
        let norm = if normalize && ksum.abs() > f32::EPSILON {
            ksum
        } else {
            1.0
        };

        let at = |x: i32, y: i32| -> u8 {
            let xx = x.clamp(0, w - 1) as u32;
            let yy = y.clamp(0, h - 1) as u32;
            gray.pixels[(yy * gray.width + xx) as usize]
        };

        for y in 0..h {
            for x in 0..w {
                let mut acc = 0.0f32;
                for ky in -1..=1 {
                    for kx in -1..=1 {
                        let p = at(x + kx, y + ky) as f32;
                        acc += p * kernel[(ky + 1) as usize][(kx + 1) as usize];
                    }
                }
                let v = (acc / norm + bias).clamp(0.0, 255.0) as u8;
                out[(y * w + x) as usize] = v;
            }
        }

        Self {
            width: gray.width,
            height: gray.height,
            channels: 1,
            pixels: out,
        }
    }
```
```rust
    /// 엠보싱(3x3)
    pub fn emboss(&self) -> Self {
        let k: [[f32; 3]; 3] = [[-2.0, -1.0, 0.0], [-1.0, 1.0, 1.0], [0.0, 1.0, 2.0]];
        self.conv3x3_gray(k, false, 128.0)
    }
```
```rust
    /// 소벨 엣지(그레이스케일)
    pub fn sobel(&self) -> Self {
        let gx = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];
        let gy = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

        let g = if self.channels == 1 {
            self.clone()
        } else {
            self.to_grayscale()
        };
        let w = g.width as i32;
        let h = g.height as i32;
        let mut out = vec![0u8; (w * h) as usize];

        let at = |x: i32, y: i32| -> u8 {
            let xx = x.clamp(0, w - 1) as u32;
            let yy = y.clamp(0, h - 1) as u32;
            g.pixels[(yy * g.width + xx) as usize]
        };

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let mut sx = 0.0f32;
                let mut sy = 0.0f32;
                for ky in -1..=1 {
                    for kx in -1..=1 {
                        let p = at(x + kx, y + ky) as f32;
                        sx += p * gx[(ky + 1) as usize][(kx + 1) as usize];
                        sy += p * gy[(ky + 1) as usize][(kx + 1) as usize];
                    }
                }
                let mag = (sx * sx + sy * sy).sqrt().clamp(0.0, 255.0) as u8;
                out[(y * w + x) as usize] = mag;
            }
        }
        Self {
            width: g.width,
            height: g.height,
            channels: 1,
            pixels: out,
        }
    }
```
```rust
    /// 가우시안 블러 (RGB/ RGBA는 채널별, Gray는 그대로)
    pub fn gaussian_blur(&self, kernel_size: usize, sigma: f32) -> Self {
        fn kernel(size: usize, sigma: f32) -> Vec<f32> {
            let c = (size / 2) as i32;
            let mut k = vec![0.0; size * size];
            let mut sum = 0.0;
            for y in -c..=c {
                for x in -c..=c {
                    let v = (-((x * x + y * y) as f32) / (2.0 * sigma * sigma)).exp();
                    k[((y + c) as usize) * size + (x + c) as usize] = v;
                    sum += v;
                }
            }
            for v in &mut k {
                *v /= sum.max(1e-12);
            }
            k
        }
        let ksz = kernel_size.max(3) | 1; // 홀수
        let k = kernel(ksz, sigma.max(1e-3));
        let c = (ksz / 2) as i32;

        let ch = self.channels as usize;
        let mut out = vec![0u8; self.pixels.len()];
        let at = |x: i32, y: i32, cidx: usize| -> u8 {
            let xx = x.clamp(0, self.width as i32 - 1) as u32;
            let yy = y.clamp(0, self.height as i32 - 1) as u32;
            self.pixels[((yy * self.width + xx) as usize) * ch + cidx]
        };

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                for cidx in 0..ch.min(3) {
                    // RGB까지 블러, A는 그대로 두거나 필요시 블러
                    let mut acc = 0.0f32;
                    for ky in -c..=c {
                        for kx in -c..=c {
                            let p = at(x + kx, y + ky, cidx) as f32;
                            let w = k[((ky + c) as usize) * ksz + (kx + c) as usize];
                            acc += p * w;
                        }
                    }
                    out[((y as u32 * self.width + x as u32) as usize) * ch + cidx] =
                        acc.clamp(0.0, 255.0) as u8;
                }
                if ch == 4 {
                    // 알파는 유지
                    out[((y as u32 * self.width + x as u32) as usize) * 4 + 3] = at(x, y, 3);
                }
            }
        }
        Self {
            width: self.width,
            height: self.height,
            channels: self.channels,
            pixels: out,
        }
    }
}
```
```rust
#[derive(Clone, Debug)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub pixels: Vec<u8>,
}
```
```rust
impl ImageBuffer {
    pub fn new(width: u32, height: u32, channels: u8) -> Self {
        Self {
            width,
            height,
            channels,
            pixels: vec![0; (width * height * channels as u32) as usize],
        }
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ImgErr> {
        let im = Image::load(path)?;
        Ok(Self {
            width: im.width,
            height: im.height,
            channels: im.channels,
            pixels: im.pixels,
        })
    }
    pub fn gray_intensity(&self, x: u32, y: u32) -> Result<f32, ImgErr> {
        let i = ((y * self.width + x) * self.channels as u32) as usize;
        match self.channels {
            1 => Ok(self.pixels[i] as f32),
            3 | 4 => {
                let r = self.pixels[i] as f32;
                let g = self.pixels[i + 1] as f32;
                let b = self.pixels[i + 2] as f32;
                Ok(0.299 * r + 0.587 * g + 0.114 * b)
            }
            _ => Ok(0.0),
        }
    }
    pub fn set_gray(&mut self, x: u32, y: u32, v: u8) -> Result<(), ImgErr> {
        let i = ((y * self.width + x) * self.channels as u32) as usize;
        match self.channels {
            1 => {
                self.pixels[i] = v;
            }
            3 => {
                self.pixels[i] = v;
                self.pixels[i + 1] = v;
                self.pixels[i + 2] = v;
            }
            4 => {
                self.pixels[i] = v;
                self.pixels[i + 1] = v;
                self.pixels[i + 2] = v;
                self.pixels[i + 3] = 255;
            }
            _ => {}
        }
        Ok(())
    }
    pub fn save_png<P: AsRef<Path>>(&self, path: P) -> Result<(), ImgErr> {
        let ct = match self.channels {
            1 => ColorType::L8,
            3 => ColorType::Rgb8,
            4 => ColorType::Rgba8,
            _ => return Err(ImgErr::Buf),
        };
        image::save_buffer(path, &self.pixels, self.width, self.height, ct)?;
        Ok(())
    }
}
```
```rust
/// 8-bit DICOM 그레이 가정(원본 C++와 동일 컨셉)
#[derive(Clone, Debug)]
pub struct DicomImageBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // channels=1
}
```
```rust
impl DicomImageBuffer {
    pub fn new_from_bytes(width: u32, height: u32, bytes: &[u8]) -> Result<Self, ImgErr> {
        let need = (width * height) as usize;
        if bytes.len() < need {
            return Err(ImgErr::Buf);
        }
        Ok(Self {
            width,
            height,
            pixels: bytes[..need].to_vec(),
        })
    }
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> Result<&mut u8, ImgErr> {
        if x >= self.width || y >= self.height {
            return Err(ImgErr::OOB { x, y });
        }
        let i = (y * self.width + x) as usize;
        Ok(&mut self.pixels[i])
    }
    pub fn gray_intensity(&self, x: u32, y: u32) -> Result<f32, ImgErr> {
        if x >= self.width || y >= self.height {
            return Err(ImgErr::OOB { x, y });
        }
        Ok(self.pixels[(y * self.width + x) as usize] as f32)
    }
}
```
```rust
pub struct PictureData<T> {
    rows: usize,
    cols: usize,
    pixels: Vec<Option<Arc<T>>>,
}
```
```rust
impl<T> PictureData<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0 && cols > 0);
        Self {
            rows,
            cols,
            pixels: vec![None; rows * cols],
        }
    }
    fn idx(&self, r: usize, c: usize) -> usize {
        assert!(r < self.rows && c < self.cols);
        r * self.cols + c
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn at(&self, r: usize, c: usize) -> Option<&Arc<T>> {
        self.pixels[self.idx(r, c)].as_ref()
    }
    pub fn at_mut(&mut self, r: usize, c: usize) -> &mut Option<Arc<T>> {
        let i = self.idx(r, c);
        &mut self.pixels[i]
    }
}
```
```rust
fn set_rgb(img: &mut [u8], width: u32, x: i32, y: i32, r: u8, g: u8, b: u8) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as u32, y as u32);
    if x >= width {
        return;
    }
    let idx = ((y * width + x) * 3) as usize;
    if idx + 2 >= img.len() {
        return;
    }
    img[idx] = b;
    img[idx + 1] = g;
    img[idx + 2] = r;
}
```
```rust
fn draw_line(img: &mut [u8], w: u32, h: u32, x0: i32, y0: i32, x1: i32, y1: i32) {
    let (dx, sx) = ((x1 - x0).abs(), if x0 < x1 { 1 } else { -1 });
    let (dy, sy) = (-(y1 - y0).abs(), if y0 < y1 { 1 } else { -1 });
    let (mut err, mut x, mut y) = (dx + dy, x0, y0);
    loop {
        if x >= 0 && y >= 0 && (x as u32) < w && (y as u32) < h {
            set_rgb(img, w, x, y, 0, 0, 0);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
```
```rust
fn draw_point(img: &mut [u8], w: u32, h: u32, x: i32, y: i32) {
    for dy in -1..=1 {
        for dx in -1..=1 {
            let (xx, yy) = (x + dx, y + dy);
            if xx >= 0 && yy >= 0 && (xx as u32) < w && (yy as u32) < h {
                set_rgb(img, w, xx, yy, 255, 0, 0);
            }
        }
    }
}
```
```rust
pub fn save_2d_points_as_bmp(
    points: &[Point2D],
    segments: &[(usize, usize)],
    filename: &str,
    width: u32,
    height: u32,
) -> Result<(), ImgErr> {
    if points.is_empty() || width == 0 || height == 0 {
        return Ok(());
    }

    // bounds
    let (mut minx, mut miny) = (f64::INFINITY, f64::INFINITY);
    let (mut maxx, mut maxy) = (-f64::INFINITY, -f64::INFINITY);
    for p in points {
        minx = minx.min(p.x);
        maxx = maxx.max(p.x);
        miny = miny.min(p.y);
        maxy = maxy.max(p.y);
    }
    let dx = (maxx - minx).max(1e-12);
    let dy = (maxy - miny).max(1e-12);
    let scale = 0.9 * ((width as f64) / dx).min((height as f64) / dy);
    let (cx, cy) = (0.5 * (minx + maxx), 0.5 * (miny + maxy));

    // 이미지 버퍼 (RGB)
    let mut buf = vec![255u8; (width * height * 3) as usize];

    // 좌표 변환
    let to_screen = |p: &Point2D| -> (i32, i32) {
        let x = (scale * (p.x - cx) + (width as f64) / 2.0).round() as i32;
        let y = (scale * (p.y - cy) + (height as f64) / 2.0).round() as i32;
        let y = height as i32 - 1 - y; // y flip
        (x, y)
    };

    // 선
    for &(i0, i1) in segments {
        if i0 < points.len() && i1 < points.len() {
            let (x0, y0) = to_screen(&points[i0]);
            let (x1, y1) = to_screen(&points[i1]);
            draw_line(&mut buf, width, height, x0, y0, x1, y1);
        }
    }
    // 점
    for p in points {
        let (x, y) = to_screen(p);
        draw_point(&mut buf, width, height, x, y);
    }

    image::save_buffer(filename, &buf, width, height, ColorType::Rgb8)?;
    Ok(())
}
```
```rust
pub fn save_polylines_as_bmp(
    polylines: &[Vec<Point2D>],
    filename: &str,
    width: u32,
    height: u32,
    minp: Point2D,
    maxp: Point2D,
) -> Result<(), ImgErr> {
    let mut buf = vec![255u8; (width * height * 3) as usize];
    let sx = (width.saturating_sub(10)) as f64 / (maxp.x - minp.x).abs().max(1e-12);
    let sy = (height.saturating_sub(10)) as f64 / (maxp.y - minp.y).abs().max(1e-12);
    let s = sx.min(sy);
    let (ox, oy) = (5.0 - minp.x * s, 5.0 - minp.y * s);

    let to_screen = |p: &Point2D| -> (i32, i32) {
        let x = (p.x * s + ox).round() as i32;
        let mut y = (p.y * s + oy).round() as i32;
        y = height as i32 - 1 - y;
        (x, y)
    };

    for line in polylines {
        if line.len() < 2 {
            continue;
        }
        for i in 1..line.len() {
            let (x0, y0) = to_screen(&line[i - 1]);
            let (x1, y1) = to_screen(&line[i]);
            draw_line(&mut buf, width, height, x0, y0, x1, y1);
        }
    }

    image::save_buffer(filename, &buf, width, height, ColorType::Rgb8)?;
    Ok(())
}

```

---

# 테스트 

지금까지 작성된 테스트 코드 전체를 기능별로 정리하고,  
각 테스트가 어떤 구조체와 함수를 검증하는지 한눈에 볼 수 있도록 문서화.

## 📦 테스트 대상 구조체 및 함수

| 구조체 / 모듈        | 테스트 대상 함수 목록                                                                 |
|----------------------|----------------------------------------------------------------------------------------|
| `Image`                | new_gray, set_gray, gray_intensity, set_pixel, get_pixel, to_grayscale, invert,       |
|                      | flipped_vertical, emboss, sobel, gaussian_blur, save, load, alpha_channel, swizzle_bgr|
| `ImageBuffer`        | new, set_gray, gray_intensity, save_png                                                |
| `DicomImageBuffer`     | new_from_bytes, gray_intensity                                                         |
| `PictureData<T>`       | new, at, at_mut                                                                        |
| `유틸리티 함수`        | save_2d_points_as_bmp                                                                  |


## 🧪 테스트 함수 목록 및 설명

| 테스트 함수 이름                             | 관련 기능 또는 구조체                      |
|---------------------------------------------|--------------------------------------------|
| `load_edit_save`                               | Image 저장 및 로드                         |
| `filters`                                       | to_grayscale, emboss, sobel, gaussian_blur |
| `draw_points_segments`                          | save_2d_points_as_bmp                       |
| `picture_data_grid`                             | PictureData                                 |
| `test_new_gray_and_set_get_gray`               | Image (그레이 생성 및 픽셀 설정/조회)      |
| `test_get_set_pixel`                            | Image (RGB 픽셀 설정/조회)                 |
| `test_to_grayscale`                             | Image (RGB → Grayscale 변환)               |
| `test_invert`                                   | Image (픽셀 반전)                          |
| `test_flipped_vertical`                         | Image (수직 플립)                          |
| `test_emboss_and_sobel`                         | Image (엠보싱, 소벨 필터)                  |
| `test_gaussian_blur`                            | Image (가우시안 블러)                      |
| `test_save_and_load`                            | Image 저장 및 로드                         |
| `test_on_image_buffer_gray_intensity_and_save` | ImageBuffer                               |
| `test_dicom_image_buffer_access`               | DicomImageBuffer                            |
| `test_alpha_channel_extraction`                | Image (알파 채널 추출)                     |
| `test_swizzle_bgr`                              | Image (RGB ↔ BGR 변환)                     |


## 📂 생성되는 테스트 이미지 파일

| 파일 경로                    | 관련 기능 또는 구조체  |
|-----------------------------|-------------------------|
| target/test_out.png         | Image 저장 및 로드 테스트 |
| target/gray.png             | Image::to_grayscale 결과 |
| target/emboss.png           | Image::emboss 결과       |
| target/sobel.png            | Image::sobel 결과        |
| target/gauss.png            | Image::gaussian_blur 결과 |
| target/box.bmp              | save_2d_points_as_bmp 결과 |
| target/on_image_buffer.png  | ImageBuffer 저장 결과  |


## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;
    use nurbslib::core::geom::Point2D;
    use nurbslib::core::image::{on_save_2d_points_as_bmp, DicomImageBuffer, Image, ImageBuffer, PictureData};

    #[test]
    fn load_edit_save() {
        // 작은 테스트용 빈 이미지 생성 → 저장 → 다시 로드
        let mut im = Image {
            width: 64,
            height: 64,
            channels: 4,
            pixels: vec![255; 64 * 64 * 4],
        };
        // 좌상단 16x16을 파란색으로
        for y in 0..16 {
            for x in 0..16 {
                im.set_pixel(x, y, &[0, 0, 255, 255]).unwrap();
            }
        }
        im.save("target/test_out.png").unwrap();
        let re = Image::load("target/test_out.png").unwrap();
        assert_eq!(re.width, 64);
        assert_eq!(re.channels, 4);
    }
```
```rust
    #[test]
    fn filters() {
        let mut im = Image {
            width: 64,
            height: 64,
            channels: 3,
            pixels: vec![0; 64 * 64 * 3],
        };
        // 가운데 흰 원 비슷하게
        for y in 0..64 {
            for x in 0..64 {
                let dx = x as i32 - 32;
                let dy = y as i32 - 32;
                let r2 = dx * dx + dy * dy;
                let v = if r2 < 20 * 20 { 255 } else { 40 };
                let i = ((y * 64 + x) * 3) as usize;
                im.pixels[i] = v;
                im.pixels[i + 1] = v;
                im.pixels[i + 2] = v;
            }
        }
        im.to_grayscale().save("target/gray.png").unwrap();
        im.emboss().save("target/emboss.png").unwrap();
        im.sobel().save("target/sobel.png").unwrap();
        im.gaussian_blur(9, 2.0).save("target/gauss.png").unwrap();
    }
```
```rust
    #[test]
    fn draw_points_segments() {
        let pts = vec![
            Point2D { x: -1.0, y: -1.0 },
            Point2D { x: 1.0, y: -1.0 },
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: -1.0, y: 1.0 },
        ];
        let segs = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        on_save_2d_points_as_bmp(&pts, &segs, "target/box.bmp", 256, 256).unwrap();
    }

    #[test]
    fn picture_data_grid() {
        let mut grid: PictureData<String> = PictureData::new(2, 3);
        *grid.at_mut(0, 0) = Some(Arc::new("hello".to_string()));
        assert_eq!(grid.at(0, 0).unwrap().as_str(), "hello");
    }
```
```rust
    #[test]
    fn test_new_gray_and_set_get_gray() {
        let mut img = Image::new_gray(10, 10);
        img.set_gray(5, 5, 128);
        let intensity = img.gray_intensity(5, 5);
        assert_eq!(intensity, 128.0);
    }
```
```rust
    #[test]
    fn test_get_set_pixel() {
        let mut img = Image {
            width: 2,
            height: 2,
            channels: 3,
            pixels: vec![0; 12],
        };
        let pix = [10, 20, 30];
        img.set_pixel(1, 1, &pix).unwrap();
        let got = img.get_pixel(1, 1).unwrap();
        assert_eq!(got, &pix);
    }
```
```rust
    #[test]
    fn test_to_grayscale() {
        let img = Image {
            width: 1,
            height: 1,
            channels: 3,
            pixels: vec![255, 0, 0], // red
        };
        let gray = img.to_grayscale();
        assert_eq!(gray.channels, 1);
        assert!(gray.pixels[0] > 0);
    }
```
```rust
    #[test]
    fn test_invert() {
        let img = Image {
            width: 1,
            height: 1,
            channels: 1,
            pixels: vec![100],
        };
        let inv = img.invert();
        assert_eq!(inv.pixels[0], 155);
    }
```
```rust
    #[test]
    fn test_flipped_vertical() {
        let mut img = Image {
            width: 2,
            height: 2,
            channels: 1,
            pixels: vec![1, 2, 3, 4],
        };
        let flipped = img.flipped_vertical();
        assert_eq!(flipped.pixels, vec![3, 4, 1, 2]);
    }
```
```rust
    #[test]
    fn test_emboss_and_sobel() {
        let img = Image::new_gray(5, 5);
        let emb = img.emboss();
        let sob = img.sobel();
        assert_eq!(emb.width, img.width);
        assert_eq!(sob.height, img.height);
    }
```
```rust
    #[test]
    fn test_gaussian_blur() {
        let img = Image {
            width: 3,
            height: 3,
            channels: 1,
            pixels: vec![0, 0, 0, 0, 255, 0, 0, 0, 0],
        };
        let blurred = img.gaussian_blur(3, 1.0);
        assert_eq!(blurred.pixels.len(), 9);
    }
```
```rust
    #[test]
    fn test_save_and_load() {
        let path = Path::new("test_output.png");
        let img = Image {
            width: 2,
            height: 2,
            channels: 4,
            pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255],
        };
        img.save(&path).unwrap();
        let loaded = Image::load(&path).unwrap();
        assert_eq!(loaded.width, img.width);
        assert_eq!(loaded.height, img.height);
        std::fs::remove_file(path).unwrap();
    }
```
```rust
    #[test]
    fn test_on_image_buffer_gray_intensity_and_save() {
        let mut buf = ImageBuffer::new(4, 4, 3);
        buf.set_gray(1, 1, 120).unwrap();
        let intensity = buf.gray_intensity(1, 1).unwrap();
        let expected = 120.0;
        let actual = buf.gray_intensity(1, 1).unwrap();
        assert!((actual - expected).abs() < 1e-3);

        buf.save_png("target/on_image_buffer.png").unwrap();
        assert!(std::path::Path::new("target/on_image_buffer.png").exists());
    }
```
```rust
    #[test]
    fn test_dicom_image_buffer_access() {
        let raw = vec![10u8; 100];
        let dicom = DicomImageBuffer::new_from_bytes(10, 10, &raw).unwrap();
        assert_eq!(dicom.gray_intensity(5, 5).unwrap(), 10.0);
    }
```
```rust
    #[test]
    fn test_alpha_channel_extraction() {
        let img = Image {
            width: 2,
            height: 1,
            channels: 4,
            pixels: vec![10, 20, 30, 40, 50, 60, 70, 80],
        };
        let alpha = img.alpha_channel();
        assert_eq!(alpha.channels, 1);
        assert_eq!(alpha.pixels, vec![40, 80]);
    }
```
```rust
    #[test]
    fn test_swizzle_bgr() {
        let img = Image {
            width: 1,
            height: 1,
            channels: 3,
            pixels: vec![10, 20, 30], // R=10, G=20, B=30
        };
        let bgr = img.swizzle_bgr();
        assert_eq!(bgr.pixels, vec![30, 20, 10]); // BGR → RGB
    }
}
```

---

