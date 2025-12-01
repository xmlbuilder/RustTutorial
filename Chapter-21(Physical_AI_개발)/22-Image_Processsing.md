# Image Processing 필요성

이미지 프로세싱 모듈은 단순히 사진을 다루는 도구를 넘어서, AI 개발의 기반 기술로서 여러 가지 중요한 기여를 합니다.

## 📌 AI 개발에서 이미지 프로세싱의 기여
### 1. 데이터 전처리 (Preprocessing)
- AI 모델은 원본 이미지를 그대로 쓰지 않고, 정규화/리사이즈/노이즈 제거 같은 전처리가 필요합니다.
  - 예: normalize_min_max, resize_bilinear, gaussian_blur_f64 → 학습 데이터 품질 향상.
### 2. 특징 추출 (Feature Extraction)
- 전통적인 컴퓨터 비전에서는 에지 검출(Canny), 모폴로지 연산 등을 통해 중요한 구조를 뽑아냅니다.
- 딥러닝에서도 CNN이 자동으로 특징을 학습하지만, 전처리 단계에서 에지/윤곽 강조를 해주면 학습이 더 안정적일 수 있습니다.
### 3. 데이터 증강 (Data Augmentation)
- AI 모델은 다양한 입력을 경험해야 일반화가 잘 됩니다.
- rotate_gray_about_center, warp_rgb, resize_nearest 같은 변환 함수는 데이터 증강에 활용되어 모델의 성능을 높입니다.
### 4. 후처리 (Postprocessing)
- 모델이 출력한 결과(예: segmentation mask, heatmap)를 사람이 보기 좋게 변환해야 합니다.
- ndarray_f64_to_gray, draw_rectangle 같은 함수는 결과 시각화와 검증에 쓰입니다.
### 5. 고전 CV와 딥러닝의 연결
- canny_edges, erode_gray, dilate_gray 같은 전통적인 영상 처리 기법은 딥러닝과 결합해 하이브리드 파이프라인을 만들 수 있습니다.
  - 예: CNN으로 coarse segmentation → imageproc 모폴로지 연산으로 경계 다듬기.

## 📊 정리: AI 개발에서의 역할

| 단계        | 관련 함수들                                | AI 개발 효과                |
|-------------|--------------------------------------------|-----------------------------|
| 전처리      | normalize_min_max, resize_bilinear         | 데이터 품질 향상            |
| 특징 추출   | canny_edges, gaussian_blur_gray            | 구조적 정보 강조            |
| 데이터 증강 | rotate_rgb_about_center, warp_dynamic      | 일반화 성능 개선            |
| 후처리      | draw_rectangle, ndarray_f64_to_gray        | 결과 시각화 및 평가         |
| 하이브리드  | erode_gray, dilate_gray                    | 딥러닝 + 전통 CV 결합       |

---

## 📌 image_utils 함수 문서화

### 타입 정의
```rust
/// 프로젝트 내부에서 쓰는 기본 이미지 타입:
/// - row-major 배열 (rows, cols) = (height, width)
/// - 값 범위 [0,1] 로 정규화된 f64
pub type ImgF64 = Array2<f64>;
```

### 변환 관련
```rust
/// GrayImage(u8) → Array2<f64> (0.0~1.0 정규화)
/// - 픽셀 값을 0~255 → 0.0~1.0 범위로 변환
fn gray_to_ndarray_f64(img: &GrayImage) -> ImgF64

/// Array2<f64> (0.0~1.0) → GrayImage(u8)
/// - NaN 값은 0으로 처리
/// - 범위를 [0,1]로 clamp 후 u8 변환
fn ndarray_f64_to_gray(arr: &ImgF64) -> GrayImage

/// 디스크에서 이미지를 읽어서 Gray + [0,1] 로 변환
pub fn load_gray_f64<P: AsRef<Path>>(path: P) -> Result<ImgF64, image::ImageError>

/// ImgF64([0,1])를 Gray(u8)로 저장
pub fn save_gray_f64<P: AsRef<Path>>(img: &ImgF64, path: P) -> Result<(), image::ImageError>

/// DynamicImage → ImgF64 (grayscale + [0,1])
pub fn dynimage_to_gray_f64(img: &DynamicImage) -> ImgF64

/// ImgF64 → DynamicImage::ImageLuma8
pub fn gray_f64_to_dynimage(img: &ImgF64) -> DynamicImage
```

### 필터링 / 변환
```rust
/// Gaussian blur (imageproc::filter::gaussian_blur_f32 래핑)
pub fn gaussian_blur_f64(img: &ImgF64, sigma: f32) -> ImgF64

/// 3x3 sharpen (imageproc::filter::sharpen3x3)
pub fn sharpen3x3_f64(img: &ImgF64) -> ImgF64

/// bilinear resize
pub fn resize_bilinear(img: &ImgF64, new_width: u32, new_height: u32) -> ImgF64

/// nearest-neighbor resize (빠르게 보고 싶을 때)
pub fn resize_nearest(img: &ImgF64, new_width: u32, new_height: u32) -> ImgF64

/// 간단한 contrast stretch: [min,max] → [0,1]
pub fn normalize_min_max(img: &ImgF64) -> ImgF64
```
### Gray/RGB 공통 처리
```rust
/// Gaussian Blur (RGB)
pub fn apply_gaussian_blur(img: &RgbImage, sigma: f32) -> RgbImage

/// Convert to grayscale
pub fn to_grayscale(img: &DynamicImage) -> GrayImage

/// Canny Edge Detection
pub fn detect_edges(img: &GrayImage, low_thresh: f32, high_thresh: f32) -> GrayImage

/// Draw rectangle
pub fn draw_rectangle(img: &mut RgbImage, x: i32, y: i32, w: u32, h: u32, color: [u8; 3])
```


### 모폴로지 / 에지 검출
```rust
/// GrayImage 기준 Gaussian blur
pub fn gaussian_blur_gray(img: &GrayImage, sigma: f32) -> GrayImage

/// Canny 엣지 검출
/// low / high : 히스테리시스 threshold (0.0 ~ 255.0)
pub fn canny_edges(img: &GrayImage, low: f32, high: f32) -> GrayImage

/// Erode (침식)
/// norm: Norm::L1 (다이아몬드), Norm::LInf (정사각형)
/// k   : 반경 (픽셀 단위)
pub fn erode_gray(img: &GrayImage, norm: Norm, k: u8) -> GrayImage

/// Dilate (팽창)
pub fn dilate_gray(img: &GrayImage, norm: Norm, k: u8) -> GrayImage
```

### 기하학 변환 (Warp / Rotation)
```rust
/// 3x3 행렬을 Projection으로 만들어주는 헬퍼
/// m: row-major [m00, m01, m02, m10, m11, m12, m20, m21, m22]
pub fn make_projection(m: [f32; 9]) -> Projection

/// GrayImage 에 projective transform 적용
pub fn warp_gray(img: &GrayImage, proj: &Projection, interpolation: Interpolation, default: Luma<u8>)
  -> GrayImage

/// RgbImage 에 projective transform 적용
pub fn warp_rgb(img: &RgbImage, proj: &Projection, interpolation: Interpolation, default: Rgb<u8>)
  -> RgbImage

/// DynamicImage 을 자동으로 Gray 또는 RGB로 warp
pub fn warp_dynamic(img: &DynamicImage, proj: &Projection, interpolation: Interpolation)
  -> DynamicImage

/// 단순 Translation 을 Projection 으로 만드는 예시
pub fn make_translation_projection(tx: f32, ty: f32) -> Projection

/// 회전 (GrayImage) – 중심 기준
pub fn rotate_gray_about_center(img: &GrayImage, angle_rad: f32, interpolation: Interpolation, default: Luma<u8>)
  -> GrayImage

/// 회전 (RgbImage) – 중심 기준
pub fn rotate_rgb_about_center(img: &RgbImage, angle_rad: f32, interpolation: Interpolation, default: Rgb<u8>)
  -> RgbImage
```


## 📊 요약
- 데이터 변환 계열: gray_to_ndarray_f64, ndarray_f64_to_gray, load_gray_f64, save_gray_f64, dynimage_to_gray_f64,  
  gray_f64_to_dynimage
- 필터링/리사이즈 계열: gaussian_blur_f64, sharpen3x3_f64, resize_bilinear, resize_nearest, normalize_min_max
- Gray/RGB 공통 처리: apply_gaussian_blur, to_grayscale, detect_edges, draw_rectangle
- 모폴로지/에지 검출: gaussian_blur_gray, canny_edges, erode_gray, dilate_gray
- 기하학 변환: make_projection, warp_gray, warp_rgb, warp_dynamic, make_translation_projection,  
  rotate_gray_about_center, rotate_rgb_about_center

---

```rust
use ndarray::Array2;
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use std::path::Path;
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_hollow_rect;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
use imageproc::geometric_transformations::{rotate_about_center, warp, Interpolation, Projection};
use imageproc::morphology::{dilate, erode};
use imageproc::rect::Rect;

/// 프로젝트 내부에서 쓰는 기본 이미지 타입:
/// row-major, (rows, cols) = (height, width), 값 범위 [0,1]
pub type ImgF64 = Array2<f64>;

/// GrayImage(u8) → Array2<f64> (0.0~1.0 정규화)
pub fn gray_to_ndarray_f64(img: &GrayImage) -> ImgF64 {
    let (width, height) = img.dimensions();
    let mut arr = Array2::<f64>::zeros((height as usize, width as usize));
    for (x, y, p) in img.enumerate_pixels() {
        let v = p[0] as f64 / 255.0;
        arr[(y as usize, x as usize)] = v;
    }
    arr
}
```
```rust
/// Array2<f64> (0.0~1.0) → GrayImage(u8)
pub fn ndarray_f64_to_gray(arr: &ImgF64) -> GrayImage {
    let (h, w) = arr.dim();
    let mut img: GrayImage = ImageBuffer::new(w as u32, h as u32);

    for y in 0..h {
        for x in 0..w {
            let mut v = arr[(y, x)];
            if v.is_nan() {
                v = 0.0;
            }
            let v_clamped = v.clamp(0.0, 1.0);
            let byte = (v_clamped * 255.0 + 0.5) as u8;
            img.put_pixel(x as u32, y as u32, Luma([byte]));
        }
    }
    img
}
```
```rust
/// 디스크에서 이미지를 읽어서 Gray + [0,1] 로 변환
pub fn load_gray_f64<P: AsRef<Path>>(path: P) -> Result<ImgF64, image::ImageError> {
    let r#dyn = ImageReader::open(path)?.decode()?;
    let gray = r#dyn.to_luma8();
    Ok(gray_to_ndarray_f64(&gray))
}
```
```rust
/// ImgF64([0,1])를 Gray(u8)로 저장
pub fn save_gray_f64<P: AsRef<Path>>(img: &ImgF64, path: P) -> Result<(), image::ImageError> {
    let gray = ndarray_f64_to_gray(img);
    gray.save(path)?;
    Ok(())
}
```
```rust
/// DynamicImage → ImgF64 (grayscale + [0,1])
pub fn dynimage_to_gray_f64(img: &DynamicImage) -> ImgF64 {
    let gray = img.to_luma8();
    gray_to_ndarray_f64(&gray)
}
```
```rust
/// ImgF64 → DynamicImage::ImageLuma8
pub fn gray_f64_to_dynimage(img: &ImgF64) -> DynamicImage {
    let gray = ndarray_f64_to_gray(img);
    DynamicImage::ImageLuma8(gray)
}
```
```rust
/// Gaussian blur (imageproc::filter::gaussian_blur_f32 래핑)
pub fn gaussian_blur_f64(img: &ImgF64, sigma: f32) -> ImgF64 {
    use imageproc::filter::gaussian_blur_f32;

    let gray = ndarray_f64_to_gray(img);
    let blurred = gaussian_blur_f32(&gray, sigma);
    gray_to_ndarray_f64(&blurred)
}
```
```rust
/// 3x3 sharpen (imageproc::filter::sharpen3x3)
pub fn sharpen3x3_f64(img: &ImgF64) -> ImgF64 {
    use imageproc::filter::sharpen3x3;

    let gray = ndarray_f64_to_gray(img);
    let sharpened = sharpen3x3(&gray);
    gray_to_ndarray_f64(&sharpened)
}
```
```rust
/// bilinear resize
pub fn resize_bilinear(img: &ImgF64, new_width: u32, new_height: u32) -> ImgF64 {
    let gray = ndarray_f64_to_gray(img);
    let resized = image::imageops::resize(&gray, new_width, new_height, FilterType::Triangle);
    gray_to_ndarray_f64(&resized)
}
```
```rust
/// nearest-neighbor resize (빠르게 보고 싶을 때)
pub fn resize_nearest(img: &ImgF64, new_width: u32, new_height: u32) -> ImgF64 {
    let gray = ndarray_f64_to_gray(img);
    let resized = image::imageops::resize(&gray, new_width, new_height, FilterType::Nearest);
    gray_to_ndarray_f64(&resized)
}
```
```rust
/// 간단한 contrast stretch: [min,max] → [0,1]
pub fn normalize_min_max(img: &ImgF64) -> ImgF64 {
    let mut min_v = f64::INFINITY;
    let mut max_v = f64::NEG_INFINITY;

    for v in img.iter() {
        if v.is_nan() { continue; }
        if *v < min_v { min_v = *v; }
        if *v > max_v { max_v = *v; }
    }

    if !min_v.is_finite() || !max_v.is_finite() || (max_v - min_v).abs() < 1e-12 {
        // degenerate: 그냥 복사
        return img.clone();
    }

    let scale = 1.0 / (max_v - min_v);
    let mut out = img.clone();
    for v in out.iter_mut() {
        *v = (*v - min_v) * scale;
    }
    out
}
```
```rust
/// Gaussian Blur
pub fn apply_gaussian_blur(img: &RgbImage, sigma: f32) -> RgbImage {
    gaussian_blur_f32(img, sigma)
}
```
```rust
/// Convert to grayscale
pub fn to_grayscale(img: &DynamicImage) -> GrayImage {
    img.to_luma8()
}
```
```rust
/// Canny Edge Detection
pub fn detect_edges(img: &GrayImage, low_thresh: f32, high_thresh: f32) -> GrayImage {
    canny(img, low_thresh, high_thresh)
}
```
```rust
/// Draw rectangle
pub fn draw_rectangle(img: &mut RgbImage, x: i32, y: i32, w: u32, h: u32, color: [u8; 3]) {
    let rect = Rect::at(x, y).of_size(w, h);
    draw_hollow_rect(img, rect, image::Rgb(color));
}
```
```rust
/// 간단한 정사각형 커널 생성 (radius = 1 → 3x3, radius = 2 → 5x5 ...)
fn square_kernel(radius: u32) -> Vec<Vec<bool>> {
    let size = 2 * radius + 1;
    vec![vec![true; size as usize]; size as usize]
}
```
```rust
/// GrayImage 기준 Gaussian blur (sigma: 표준편차)
pub fn gaussian_blur_gray(img: &GrayImage, sigma: f32) -> GrayImage {
    gaussian_blur_f32(img, sigma)
}
```
```rust
/// Canny 엣지 검출
/// low / high 는 히스테리시스 threshold (0.0 ~ 255.0 정도로 사용)
pub fn canny_edges(img: &GrayImage, low: f32, high: f32) -> GrayImage {
    canny(img, low, high)
}
```
```rust
/// Erode (침식) – imageproc 0.25 API
/// norm: Norm::L1 (다이아몬드), Norm::LInf (정사각형)
/// k   : 반경 (픽셀 단위)
pub fn erode_gray(img: &GrayImage, norm: Norm, k: u8) -> GrayImage {
    erode(img, norm, k)
}
```
```rust
/// Dilate (팽창) – imageproc 0.25 API
pub fn dilate_gray(img: &GrayImage, norm: Norm, k: u8) -> GrayImage {
    dilate(img, norm, k)
}
```
```rust
/// 3x3 행렬을 Projection으로 만들어주는 헬퍼
/// m: row-major [m00, m01, m02, m10, m11, m12, m20, m21, m22]
pub fn make_projection(m: [f32; 9]) -> Projection {
    Projection::from_matrix(m)
        .expect("invalid projection matrix (not invertible?)")
}
```
```rust
/// GrayImage 에 projective transform 적용
pub fn warp_gray(
    img: &GrayImage,
    proj: &Projection,
    interpolation: Interpolation,
    default: Luma<u8>,
) -> GrayImage {
    warp(img, proj, interpolation, default)
}
```
```rust
/// RgbImage 에 projective transform 적용
pub fn warp_rgb(
    img: &RgbImage,
    proj: &Projection,
    interpolation: Interpolation,
    default: Rgb<u8>,
) -> RgbImage {
    warp(img, proj, interpolation, default)
}
```
```rust
/// DynamicImage 을 자동으로 Gray 또는 RGB로 warp
pub fn warp_dynamic(
    img: &DynamicImage,
    proj: &Projection,
    interpolation: Interpolation,
) -> DynamicImage {
    match img {
        DynamicImage::ImageLuma8(g) => {
            let def = Luma([0u8]);
            DynamicImage::ImageLuma8(warp_gray(g, proj, interpolation, def))
        }
        DynamicImage::ImageRgb8(c) => {
            let def = Rgb([0u8, 0u8, 0u8]);
            DynamicImage::ImageRgb8(warp_rgb(c, proj, interpolation, def))
        }
        other => {
            // 필요 시 format 변환 (일단 Gray로)
            let g = other.to_luma8();
            let def = Luma([0u8]);
            DynamicImage::ImageLuma8(warp_gray(&g, proj, interpolation, def))
        }
    }
}
```
```rust
// ========================== 예: 간단한 Affine ==========================
/// 단순 Translation 을 Projection 으로 만드는 예시
pub fn make_translation_projection(tx: f32, ty: f32) -> Projection {
    // [ 1 0 tx ]
    // [ 0 1 ty ]
    // [ 0 0 1  ]
    make_projection([
        1.0, 0.0, tx,
        0.0, 1.0, ty,
        0.0, 0.0, 1.0,
    ])
}
```
```rust
/// 회전 + 중앙 기준 회전은 imageproc 의 rotate_about_center 를 직접 써도 됨
pub fn rotate_gray_about_center(
    img: &GrayImage,
    angle_rad: f32,
    interpolation: Interpolation,
    default: Luma<u8>,
) -> GrayImage {
    rotate_about_center(img, angle_rad, interpolation, default)
}
```
```rust
pub fn rotate_rgb_about_center(
    img: &RgbImage,
    angle_rad: f32,
    interpolation: Interpolation,
    default: Rgb<u8>,
) -> RgbImage {
    rotate_about_center(img, angle_rad, interpolation, default)
}
```

---
## 테스트 코드
```rust
#[cfg(test)]
mod tests_case1 {
    use ndarray::{array, Array2};
    use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
    use imageproc::geometric_transformations::Interpolation;
    use imageproc::distance_transform::Norm;
    use nurbslib::core::image_utils::{canny_edges, dilate_gray, dynimage_to_gray_f64,
      erode_gray, gaussian_blur_f64, gray_f64_to_dynimage, make_projection, normalize_min_max,
      resize_bilinear, resize_nearest, rotate_gray_about_center, rotate_rgb_about_center,
      warp_gray, warp_rgb};

    fn approx_eq(a: f64, b: f64, tol: f64) {
        let diff = (a - b).abs();
        assert!(
            diff <= tol,
            "a={} b={} diff={} > tol={}",
            a, b, diff, tol
        );
    }

    // ------------------------------
    // 1. Gray <-> f64 변환 테스트
    // ------------------------------
    #[test]
    fn dynimage_gray_roundtrip_f64() {
        // 3x2 작은 그레이 이미지 만들기
        let w = 3;
        let h = 2;
        let mut gray: GrayImage = ImageBuffer::new(w, h);
        // 패턴: 0, 64, 128 / 192, 255, 32
        let vals = [
            [0u8, 64, 128],
            [192, 255, 32],
        ];
        for y in 0..h {
            for x in 0..w {
                gray.put_pixel(x, y, Luma([vals[y as usize][x as usize]]));
            }
        }

        let r#dyn = DynamicImage::ImageLuma8(gray.clone());
        let f64_img = dynimage_to_gray_f64(&r#dyn);
        let dyn2 = gray_f64_to_dynimage(&f64_img);
        let gray2 = dyn2.to_luma8();

        assert_eq!(gray.dimensions(), gray2.dimensions());

        // u8 값이 완전히 같지 않을 수 있으니 ±1 정도 허용
        for y in 0..h {
            for x in 0..w {
                let v0 = gray.get_pixel(x, y)[0];
                let v1 = gray2.get_pixel(x, y)[0];
                let diff = (v0 as i16 - v1 as i16).abs();
                assert!(
                    diff <= 1,
                    "pixel ({},{}) -> {} vs {}, diff={} > 1",
                    x, y, v0, v1, diff
                );
            }
        }
    }
```
```rust
    // ------------------------------
    // 2. gaussian_blur_f64: 임펄스 테스트
    // ------------------------------
    #[test]
    fn gaussian_blur_f64_impulse_response() {
        // 7x7, 가운데만 1.0
        let mut img = Array2::<f64>::zeros((7, 7));
        img[(3, 3)] = 1.0;

        let blurred = gaussian_blur_f64(&img, 1.0);

        // 가운데 값은 0 < v < 1
        let center = blurred[(3, 3)];
        assert!(center > 0.0 && center < 1.0, "center={}", center);

        // 이웃 픽셀도 양수
        let n1 = blurred[(3, 4)];
        let n2 = blurred[(4, 3)];
        assert!(n1 > 0.0 && n2 > 0.0, "n1={}, n2={}", n1, n2);

        // 중심이 이웃보다 크다고 가정 (가우시안)
        assert!(center >= n1 && center >= n2);
    }
```
```rust
    // ------------------------------
    // 3. normalize_min_max 선형 스케일 테스트
    // ------------------------------
    #[test]
    fn normalize_min_max_linear_mapping() {
        // min=2, max=8
        let img = array![
            [2.0, 4.0],
            [6.0, 8.0],
        ];
        let out = normalize_min_max(&img);

        // 기대값: (v - 2) / (8 - 2) = (v - 2) / 6
        let expected = array![
            [0.0, (4.0 - 2.0) / 6.0],
            [(6.0 - 2.0) / 6.0, 1.0],
        ];

        for y in 0..2 {
            for x in 0..2 {
                approx_eq(out[(y, x)], expected[(y, x)], 1e-12);
            }
        }
    }
```
```rust
    // ------------------------------
    // 4. erode / dilate 동작 테스트
    // ------------------------------
    #[test]
    fn erode_and_dilate_simple_pattern() {
        // 5x5, 가운데 십자 모양 (255)
        let mut img: GrayImage = ImageBuffer::new(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                let v = if x == 2 || y == 2 { 255u8 } else { 0u8 };
                img.put_pixel(x, y, Luma([v]));
            }
        }

        // white pixel count helper
        fn count_white(img: &GrayImage) -> usize {
            img.pixels().filter(|p| p[0] > 128).count()
        }

        let count_orig = count_white(&img);
        assert!(count_orig > 0);

        // 침식: 하얀 영역이 줄어들어야 함
        let eroded = erode_gray(&img, Norm::LInf, 1);
        let count_eroded = count_white(&eroded);
        assert!(
            count_eroded < count_orig,
            "erode: count_eroded={} >= count_orig={}",
            count_eroded,
            count_orig
        );

        // 팽창: 다시 증가 (원본보다 크거나 같게)
        let dilated = dilate_gray(&eroded, Norm::LInf, 1);
        let count_dilated = count_white(&dilated);
        assert!(
            count_dilated >= count_eroded,
            "dilate: count_dilated={} < count_eroded={}",
            count_dilated,
            count_eroded
        );
    }
```
```rust
    // ------------------------------
    // 5. Canny 엣지 검출 간단 테스트
    // ------------------------------
    #[test]
    fn canny_edges_simple_diagonal_line() {
        let mut img: GrayImage = ImageBuffer::new(16, 16);
        // 대각선에 255
        for i in 0..16 {
            img.put_pixel(i, i, Luma([255u8]));
        }

        let edges = canny_edges(&img, 50.0, 150.0);

        // 일부 픽셀은 엣지로 검출되어야 함
        let edge_count = edges.pixels().filter(|p| p[0] > 0).count();
        assert!(
            edge_count > 0,
            "no edges detected, edge_count={}",
            edge_count
        );
    }
```
```rust
    // ------------------------------
    // 6. warp: 단위 변환 (identity) 테스트
    // ------------------------------
    #[test]
    fn warp_gray_identity_projection() {
        // 5x5, 중앙 한 픽셀만 255
        let mut img: GrayImage = ImageBuffer::new(5, 5);
        img.put_pixel(2, 2, Luma([255u8]));

        // 단위 행렬 projection
        let proj = make_projection([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]);

        let warped = warp_gray(&img, &proj, Interpolation::Nearest, Luma([0u8]));
        assert_eq!(img.dimensions(), warped.dimensions());

        // 최소한 중앙 픽셀은 그대로 있어야 한다(대략적인 identity 확인)
        assert_eq!(warped.get_pixel(2, 2)[0], 255u8);
    }
```
```rust
    #[test]
    fn warp_rgb_identity_projection() {
        let mut img: RgbImage = ImageBuffer::new(5, 5);
        img.put_pixel(1, 3, Rgb([10u8, 20u8, 30u8]));

        let proj = make_projection([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]);

        let warped = warp_rgb(&img, &proj, Interpolation::Nearest, Rgb([0u8, 0u8, 0u8]));
        assert_eq!(img.dimensions(), warped.dimensions());

        let orig = img.get_pixel(1, 3);
        let w = warped.get_pixel(1, 3);
        assert_eq!(orig[0], w[0]);
        assert_eq!(orig[1], w[1]);
        assert_eq!(orig[2], w[2]);
    }
```
```rust
    // ------------------------------
    // 7. 회전(rotate_about_center) smoke test
    // ------------------------------
    #[test]
    fn rotate_gray_about_center_smoke() {
        let mut img: GrayImage = ImageBuffer::new(10, 10);
        // 중앙 십자 표시
        for i in 0..10 {
            img.put_pixel(5, i, Luma([255u8]));
            img.put_pixel(i, 5, Luma([255u8]));
        }

        let rotated = rotate_gray_about_center(
            &img,
            std::f32::consts::FRAC_PI_4, // 45 degrees
            Interpolation::Nearest,
            Luma([0u8]),
        );

        // 크기 유지
        assert_eq!(img.dimensions(), rotated.dimensions());

        // 픽셀 총 합은 어느 정도 유지 (완전히 같지는 않아도)
        let sum_orig: u32 = img.pixels().map(|p| p[0] as u32).sum();
        let sum_rot: u32 = rotated.pixels().map(|p| p[0] as u32).sum();

        // 너무 많이 줄어들지 않았는지만 체크 (대략 50% 이상)
        assert!(
            sum_rot as f64 > 0.5 * sum_orig as f64,
            "sum_rot={} sum_orig={}",
            sum_rot,
            sum_orig
        );
    }
```
```rust
    #[test]
    fn rotate_rgb_about_center_smoke() {
        let mut img: RgbImage = ImageBuffer::new(8, 8);
        // (3,3)에만 색 지정
        img.put_pixel(3, 3, Rgb([100u8, 150u8, 200u8]));

        let rotated = rotate_rgb_about_center(
            &img,
            std::f32::consts::FRAC_PI_2, // 90 degrees
            Interpolation::Nearest,
            Rgb([0u8, 0u8, 0u8]),
        );

        assert_eq!(img.dimensions(), rotated.dimensions());

        // 전체 R+G+B 합이 0은 아니어야 함 (색 정보가 유지되었다는 정도만 확인)
        let sum_rot: u32 = rotated
            .pixels()
            .map(|p| p[0] as u32 + p[1] as u32 + p[2] as u32)
            .sum();
        assert!(sum_rot > 0);
    }
```
```rust
    // ------------------------------
    // 8. resize_bilinear / resize_nearest 기본 테스트
    // ------------------------------
    #[test]
    fn resize_bilinear_and_nearest_basic() {
        let img = array![
            [0.0, 1.0],
            [0.5, 0.75],
        ];
        let img2 = resize_bilinear(&img, 4, 4);
        assert_eq!(img2.dim(), (4, 4));

        let img3 = resize_nearest(&img, 4, 4);
        assert_eq!(img3.dim(), (4, 4));

        // 값이 [0,1] 범위를 벗어나지 않는지만 간단히 체크
        for v in img2.iter().chain(img3.iter()) {
            assert!(*v >= 0.0 && *v <= 1.0);
        }
    }
}
```
```rust
#[cfg(test)]
mod tests_case2 {
    use super::*;
    use image::{GrayImage, RgbImage, Luma, Rgb};
    use imageproc::distance_transform::Norm;
    use imageproc::geometric_transformations::Interpolation;
    use nurbslib::core::image_utils::{canny_edges, dilate_gray, draw_rectangle, erode_gray,
      gaussian_blur_gray, gray_to_ndarray_f64, make_translation_projection, ndarray_f64_to_gray,
      rotate_gray_about_center, warp_gray};

    fn make_test_gray() -> GrayImage {
        // 5x5 단순 GrayImage 생성
        let mut img = GrayImage::new(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                img.put_pixel(x, y, Luma([((x + y) * 20) as u8]));
            }
        }
        img
    }

    fn make_test_rgb() -> RgbImage {
        // 5x5 단순 RgbImage 생성
        let mut img = RgbImage::new(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                img.put_pixel(x, y, Rgb([x as u8 * 50, y as u8 * 50, 128]));
            }
        }
        img
    }

    #[test]
    fn test_gray_to_ndarray_and_back() {
        let gray = make_test_gray();
        let arr = gray_to_ndarray_f64(&gray);
        let gray2 = ndarray_f64_to_gray(&arr);
        assert_eq!(gray.dimensions(), gray2.dimensions());
    }
```
```rust
    #[test]
    fn test_gaussian_blur_gray() {
        let gray = make_test_gray();
        let blurred = gaussian_blur_gray(&gray, 1.0);
        assert_eq!(blurred.width(), gray.width());
        assert_eq!(blurred.height(), gray.height());
    }
```
```rust
    #[test]
    fn test_canny_edges() {
        let gray = make_test_gray();
        let edges = canny_edges(&gray, 50.0, 100.0);
        assert_eq!(edges.width(), gray.width());
        assert_eq!(edges.height(), gray.height());
    }
```
```rust
    #[test]
    fn test_dilate_and_erode() {
        let gray = make_test_gray();
        let dilated = dilate_gray(&gray, Norm::LInf, 1);
        let eroded = erode_gray(&gray, Norm::LInf, 1);
        assert_eq!(dilated.width(), gray.width());
        assert_eq!(eroded.height(), gray.height());
    }
```
```rust
    #[test]
    fn test_warp_gray_translation() {
        let gray = make_test_gray();
        let proj = make_translation_projection(1.0, 1.0);
        let warped = warp_gray(&gray, &proj, Interpolation::Nearest, Luma([0]));
        assert_eq!(warped.width(), gray.width());
        assert_eq!(warped.height(), gray.height());
    }
```
```rust
    #[test]
    fn test_rotate_gray_about_center() {
        let gray = make_test_gray();
        let rotated = rotate_gray_about_center(&gray, std::f32::consts::PI/4.0, Interpolation::Nearest, Luma([0]));
        assert_eq!(rotated.width(), gray.width());
        assert_eq!(rotated.height(), gray.height());
    }
```
```rust
    #[test]
    fn test_draw_rectangle() {
        let mut rgb = make_test_rgb();
        draw_rectangle(&mut rgb, 1, 1, 3, 3, [255, 0, 0]);
        assert_eq!(rgb.width(), 5);
        assert_eq!(rgb.height(), 5);
    }
}
```
---
