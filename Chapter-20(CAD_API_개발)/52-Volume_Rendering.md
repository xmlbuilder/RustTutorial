# Volume Rendering

## CtSliceInfo 구조체 필드 설명

| 필드 이름        | 타입         | 설명                                           |
|------------------|--------------|------------------------------------------------|
| `image`          | `Option<Arc<Image>` | 슬라이스 이미지. 없을 수도 있음 (`None`)         |
| `slice_location` | `f64`        | 슬라이스의 Z축 위치(mm). 공간상 위치 정보       |
| `slice_index`    | `i32`        | 슬라이스 인덱스. 일반적으로 0 이상이면 유효함   |
| `slice_thickness`| `f64`        | 슬라이스 두께(mm). CT 간격 또는 해상도 정보     |
| `source_path`    | `String`     | 원본 이미지 파일 경로. 로딩 또는 추적용         |

- is_valid() → 이미지가 존재하고 인덱스가 0 이상이면 유효한 슬라이스로 간주

## VolumeRendering 구조체 필드 설명

| 필드 이름 | 타입               | 설명                                           |
|-----------|--------------------|------------------------------------------------|
| `slices`  | `Vec<CtSliceInfo>` | CT 슬라이스 정보 목록. Z축 위치 기준으로 정렬됨 |


## 🧩 주요 기능 및 단계별 처리 흐름
### 1. 슬라이스 설정 및 정렬
```rust
pub fn set_slices(&mut self, mut slices: Vec<CtSliceInfo>)
```
- 슬라이스를 Z축 위치(slice_location) 기준으로 정렬하여 내부에 저장

### 2. 특정 Z 위치에서 슬라이스 추출
```rust
pub fn extract_slice(&self, z_mm: f64) -> Option<Arc<Image>>
```
- 입력 Z(mm) 위치에서 가장 가까운 슬라이스를 찾아 이미지 반환

### 3. MIP (Maximum Intensity Projection) 렌더링
```rust
pub fn render_mip(&self) -> Option<Arc<Image>>
```

- 각 픽셀 위치에서 슬라이스들 중 최대 그레이값을 선택하여 2D 이미지 생성

#### 📐 수식:

$$
I_{\mathrm{MIP}}(x,y)=\max _kI_k(x,y)
$$

### 4. X-ray (평균 투영) 렌더링
```rust
pub fn render_xray(&self) -> Option<Arc<Image>>
```

- 각 픽셀 위치에서 슬라이스들의 평균 그레이값을 계산하여 2D 이미지 생성
##### 📐 수식:

$$
I_{\mathrm{Xray}}(x,y)=\frac{1}{N}\sum _{k=1}^NI_k(x,y)
$$


### 5. 보간 슬라이스 생성

```rust
pub fn interpolated_slice(&self, z_mm: f64) -> Option<Arc<Image>>
```

- z_mm이 두 슬라이스 사이에 위치할 경우, 선형 보간으로 중간 슬라이스 생성

#### 📐 수식:

$$
I(x,y)=(1-t)\cdot I_0(x,y)+t\cdot I_1(x,y)\quad \mathrm{where\  }t=\frac{z-z_0}{z_1-z_0}
$$

### 6. 단일 복셀 강도 조회
```rust
pub fn voxel_intensity(&self, x: u32, y: u32, z: i32) -> Option<f32>
```
- (x, y, z) 위치의 복셀 강도 반환 (슬라이스 유효성 검사 포함)

## 🧰 유틸리티 함수 목록

| 함수 이름                          | 반환값         | 설명                                                                 |
|-----------------------------------|----------------|----------------------------------------------------------------------|
| `clamp_to_byte(v: i32)`           | `u8`           | 입력 정수 `v`를 0~255 범위로 클램핑하여 `u8`로 변환합니다.           |
| `make_empty_gray(w, h)`           | `Arc<Image>`   | 지정된 너비와 높이의 빈 그레이스케일 이미지를 생성합니다.            |
| `draw_disk(img, cx, cy, r, val)`  | 없음           | 이미지에 중심 `(cx, cy)`과 반지름 `r`를 갖는 원형을 `val` 값으로 채웁니다. |
| `draw_ring(img, cx, cy, r0, r1, val)` | 없음        | 이미지에 중심 `(cx, cy)`과 내외부 반지름 `r0`, `r1`를 갖는 링을 그립니다. |
| `draw_diag(img, val)`             | 없음           | 이미지의 대각선에 `val` 값을 적용하여 선을 그립니다.                  |


## ✅ 테스트 예시
```rust
#[test]
fn test_extract_and_render_mip() {
    use std::sync::Arc;
    use crate::core::image::Image;

    let mut vr = VolumeRendering::new();

    let mut slices = vec![];
    for i in 0..5 {
        let mut img = Image::new_gray(64, 64);
        draw_disk(&mut img, 32, 32, 10 + i, 50 + i as u8);
        let slice = CtSliceInfo::new(Some(Arc::new(img)), i, i as f64 * 1.0, 1.0);
        slices.push(slice);
    }

    vr.set_slices(slices);

    let mip = vr.render_mip().unwrap();
    assert_eq!(mip.width, 64);
    assert_eq!(mip.height, 64);

    let val = mip.gray_intensity(32, 32);
    assert!(val >= 50);
}
```

## 📐 수식 점검: 주요 함수별 분석

| 함수 이름                          | 수식 사용 여부 | 관련 수식 및 의미                                                                 |
|-----------------------------------|----------------|------------------------------------------------------------------------------------|
| `clamp_to_byte(v: i32)`           | ✅ 있음         | $\min(255, \max(0, v))$ — 0~255 범위로 클램핑                                 |
| `make_empty_gray(w, h)`           |  ✅ 있음          | 빈 이미지 생성                                                             |
| `draw_disk(img, cx, cy, r, val)`  | ✅ 있음         | $dx^2 + dy^2 \leq r^2$ — 원 내부 픽셀 판별                                    |
| `draw_ring(img, cx, cy, r0, r1, val)` | ✅ 있음      | $r_0^2 \leq dx^2 + dy^2 \leq r_1^2$ — 링 영역 판별                            |
| `draw_diag(img, val)`             | ✅ 있음         | $x = y$ — 대각선 픽셀 설정                                                     |
| `set_slices()`                    | ✅ 있음           | 슬라이스 정렬만 수행                                                       |
| `find_closest_slice(z_mm)`       | ✅ 있음         | $\min \|z_i - z_{\text{target}}\|$ — Z 위치 거리 최소화                         |
| `extract_slice(z_mm)`            |  ✅ 있음           | 가장 가까운 슬라이스 반환                                                  |
| `render_mip()`                   | ✅ 있음         | $I(x, y) = \max_k I_k(x, y)$ — 최대 강도 투영                                 |
| `render_xray()`                  | ✅ 있음         | $I(x, y) = \frac{1}{N} \sum_k I_k(x, y)$ — 평균 투영                          |
| `interpolated_slice(z_mm)`      | ✅ 있음         | $I(x, y) = (1 - t) I_0(x, y) + t I_1(x, y)$,  
  $t = \frac{z - z_0}{z_1 - z_0}$ — 선형 보간 |
| `voxel_intensity(x, y, z)`       |  ✅ 있음          | 단일 픽셀 강도 조회                                                        |


---

## 소스 코드

```rust
use crate::core::image::Image;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CtSliceInfo {
    pub image: Option<Arc<Image>>,
    pub slice_location: f64,
    pub slice_index: i32,
    pub slice_thickness: f64,
    pub source_path: String,
}
```
```rust
impl CtSliceInfo {
    pub fn new(img: Option<Arc<Image>>, index: i32, location: f64, thickness: f64) -> Self {
        Self {
            image: img,
            slice_location: location,
            slice_index: index,
            slice_thickness: thickness,
            source_path: String::new(),
        }
    }
    pub fn is_valid(&self) -> bool {
        self.image.is_some() && self.slice_index >= 0
    }
}
```

```rust
use crate::core::ct_slice_info::CtSliceInfo;
use crate::core::image::Image;
use std::cmp::{max, min};
use std::sync::Arc;

#[inline]
fn clamp_to_byte(v: i32) -> u8 {
    min(255, max(0, v)) as u8
}
```
```rust
#[derive(Default)]
pub struct VolumeRendering {
    pub slices: Vec<CtSliceInfo>,
}
```
```rust
impl VolumeRendering {
    pub fn new() -> Self {
        Self { slices: Vec::new() }
    }
```
```rust
    pub fn set_slices(&mut self, mut slices: Vec<CtSliceInfo>) {
        slices.sort_by(|a, b| a.slice_location.partial_cmp(&b.slice_location).unwrap());
        self.slices = slices;
    }
```
```rust
    fn find_closest_slice(&self, z_mm: f64) -> Option<&CtSliceInfo> {
        let mut best: Option<&CtSliceInfo> = None;
        let mut best_d = f64::INFINITY;
        for s in &self.slices {
            let d = (s.slice_location - z_mm).abs();
            if d < best_d {
                best_d = d;
                best = Some(s);
            }
        }
        best
    }
```
```rust
    pub fn extract_slice(&self, z_mm: f64) -> Option<Arc<Image>> {
        self.find_closest_slice(z_mm)?.image.clone()
    }
```
```rust
    pub fn render_mip(&self) -> Option<Arc<Image>> {
        let first = self.slices.iter().find_map(|s| s.image.as_ref())?.clone();
        let (w, h) = (first.width, first.height);
        // 결과는 그레이 1채널로 생성
        let mut out = Image::new_gray(w, h);

        for y in 0..h {
            for x in 0..w {
                let mut mg = 0i32;
                for s in &self.slices {
                    if let Some(img) = &s.image {
                        if img.width == w && img.height == h {
                            mg = max(mg, img.gray_intensity(x, y) as i32);
                        }
                    }
                }
                out.set_gray(x, y, clamp_to_byte(mg));
            }
        }
        Some(Arc::new(out))
    }
```
```rust
    pub fn render_xray(&self) -> Option<Arc<Image>> {
        let first = self.slices.iter().find_map(|s| s.image.as_ref())?.clone();
        let (w, h) = (first.width, first.height);
        let mut out = Image::new_gray(w, h);

        for y in 0..h {
            for x in 0..w {
                let mut sum = 0i64;
                let mut cnt = 0i64;
                for s in &self.slices {
                    if let Some(img) = &s.image {
                        if img.width == w && img.height == h {
                            sum += img.gray_intensity(x, y) as i64;
                            cnt += 1;
                        }
                    }
                }
                let avg = if cnt > 0 { (sum / cnt) as i32 } else { 0 };
                out.set_gray(x, y, clamp_to_byte(avg));
            }
        }
        Some(Arc::new(out))
    }
```
```rust
    pub fn interpolated_slice(&self, z_mm: f64) -> Option<Arc<Image>> {
        if self.slices.len() < 2 {
            return self.extract_slice(z_mm);
        }
        let s = &self.slices;
        for i in 1..s.len() {
            let (z0, z1) = (s[i - 1].slice_location, s[i].slice_location);
            if z0 <= z_mm && z_mm <= z1 {
                let denom = z1 - z0;
                let t = if denom.abs() > f64::EPSILON {
                    (z_mm - z0) / denom
                } else {
                    0.0
                };
                let (img0, img1) = match (&s[i - 1].image, &s[i].image) {
                    (Some(a), Some(b)) => (a, b),
                    _ => return self.extract_slice(z_mm),
                };
                if img0.width != img1.width || img0.height != img1.height {
                    return self.extract_slice(z_mm);
                }
                let (w, h) = (img0.width, img0.height);
                let mut out = Image::new_gray(w, h);

                for y in 0..h {
                    for x in 0..w {
                        let g0 = img0.gray_intensity(x, y) as f64;
                        let g1 = img1.gray_intensity(x, y) as f64;
                        let g = ((1.0 - t) * g0 + t * g1 + 0.5).round() as i32;
                        out.set_gray(x, y, clamp_to_byte(g));
                    }
                }
                return Some(Arc::new(out));
            }
        }
        self.extract_slice(z_mm)
    }
```
```rust
    pub fn voxel_intensity(&self, x: u32, y: u32, z: i32) -> Option<f32> {
        if z < 0 || (z as usize) >= self.slices.len() {
            return None;
        }
        let s = &self.slices[z as usize];
        if !s.is_valid() {
            return None;
        }
        s.image.as_ref().map(|im| im.gray_intensity(x, y))
    }
}
```
```rust
pub fn make_empty_gray(w: u32, h: u32) -> Arc<Image> {
    Arc::new(Image::new_gray(w, h))
}
```
```rust
pub fn on_draw_disk(img: &mut Image, cx: i32, cy: i32, r: i32, val: u8) {
    let (w, h) = (img.width as i32, img.height as i32);
    let r2 = r * r;
    let y0 = (cy - r).max(0);
    let y1 = (cy + r).min(h - 1);
    for y in y0..=y1 {
        let x0 = (cx - r).max(0);
        let x1 = (cx + r).min(w - 1);
        for x in x0..=x1 {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= r2 {
                img.set_gray(x as u32, y as u32, val);
            }
        }
    }
}
```
```rust
pub fn on_draw_ring(img: &mut Image, cx: i32, cy: i32, r0: i32, r1: i32, val: u8) {
    let (w, h) = (img.width as i32, img.height as i32);
    let r0s = r0 * r0;
    let r1s = r1 * r1;
    let y0 = (cy - r1).max(0);
    let y1 = (cy + r1).min(h - 1);
    for y in y0..=y1 {
        let x0 = (cx - r1).max(0);
        let x1 = (cx + r1).min(w - 1);
        for x in x0..=x1 {
            let dx = x - cx;
            let dy = y - cy;
            let d = dx * dx + dy * dy;
            if r0s <= d && d <= r1s {
                img.set_gray(x as u32, y as u32, val);
            }
        }
    }
}
```
```rust
pub fn on_draw_diag(img: &mut Image, val: u8) {
    let m = img.width.min(img.height);
    for i in 0..m {
        img.set_gray(i, i, val);
    }
}
```

---

# 테스트
✅ VolumeRendering 테스트 정리표
| 테스트 함수 이름               | 검증 대상 함수             | 수식 사용 여부 | 관련 수식 또는 처리 방식                                      |
|-------------------------------|----------------------------|----------------|---------------------------------------------------------------|
| `gen_volume_rendering`        | `set_slices`,  `render_mip`, <br> `render_xray`, `interpolated_slice` | ✅ 있음 | MIP: $I(x,y) = \max_k I_k(x,y)$ <br> Xray: $I(x,y) = \frac{1}{N} \sum_k I_k(x,y)$ <br> 보간: $I = (1 - t) I_0 + t I_1$, $t = \frac{z - z_0}{z_1 - z_0}$ |
| `test_extract_and_render_mip` | `set_slices`, <br> `render_mip` | ✅ 있음         | $I(x,y) = \max_k I_k(x,y)$                                |
| `test_set_slices_and_ordering`| `set_slices`               | ✅ 있음        | 슬라이스 정렬만 수행                                          |
| `test_extract_slice`          | `extract_slice`            | ✅ 내부 거리 계산 | $\min \|z_i - z_{\text{target}}\|$                           |
| `test_voxel_intensity`        | `voxel_intensity`          | ✅ 있음         | 단일 픽셀 강도 조회                                           |
| `test_invalid_voxel_access`   | `voxel_intensity`          | ✅ 있음         | 인덱스 범위 및 유효성 검사                                    |



## 📐 VolumeRendering 관련 수식 정리표

| 관련 기능/함수                  | 수식 표현                                                                 |
|----------------------------------|----------------------------------------------------------------------------|
| MIP 렌더링 (`render_mip`)        | $I(x, y) = \max_k I_k(x, y)$                                           |
| X-ray 렌더링 (`render_xray`)     | $I(x, y) = \frac{1}{N} \sum_k I_k(x, y)$                               |
| 보간 슬라이스 (`interpolated_slice`) | $I(x, y) = (1 - t) I_0(x, y) + t I_1(x, y)$                             |
| 보간 계수 t 계산                 | $t = \frac{z - z_0}{z_1 - z_0}$                                        |
| 슬라이스 거리 비교 (`find_closest_slice`) | $\min \|z_i - z_{\text{target}}\|$                              |
| 원형 그리기 (`draw_disk`)       | $dx^2 + dy^2 \leq r^2$                                                 |
| 링 그리기 (`draw_ring`)         | $r_0^2 \leq dx^2 + dy^2 \leq r_1^2$                                    |
| 대각선 그리기 (`draw_diag`)     | $x = y$                                                                |


## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use nurbslib::core::ct_slice_info::CtSliceInfo;
    use nurbslib::core::image::Image;
    use nurbslib::core::volume_rendering::{make_empty_gray, on_draw_diag, on_draw_disk, on_draw_ring, VolumeRendering};
```
```rust
    #[test]
    fn gen_volume_rendering() {
        // 가짜 슬라이스 3장 만들기
        let (w, h) = (256u32, 256u32);
        let mut s0 = Image::new_gray(w, h);
        let mut s1 = Image::new_gray(w, h);
        let mut s2 = Image::new_gray(w, h);

        on_draw_disk(&mut s0, 128, 128, 60, 120);
        on_draw_ring(&mut s1, 128, 128, 40, 80, 200);
        on_draw_diag(&mut s2, 255);

        let slices = vec![
            CtSliceInfo::new(Some(Arc::new(s0)), 0, 0.0, 1.0),
            CtSliceInfo::new(Some(Arc::new(s1)), 1, 2.0, 1.0),
            CtSliceInfo::new(Some(Arc::new(s2)), 2, 4.0, 1.0),
        ];

        let mut vol = VolumeRendering::new();
        vol.set_slices(slices);

        let _mip = vol.render_mip().unwrap();
        let _xray = vol.render_xray().unwrap();
        let mid = vol.interpolated_slice(1.0).unwrap(); // z=1.0 보간
        mid.save("asset/mip.png").unwrap();
    }
```
```rust
    #[test]
    fn test_extract_and_render_mip() {
        use std::sync::Arc;

        let mut vr = VolumeRendering::new();

        let mut slices = vec![];
        for i in 0..5 {
            let mut img = Image::new_gray(64, 64);
            on_draw_disk(&mut img, 32, 32, 10 + i, 50 + i as u8);
            let slice = CtSliceInfo::new(Some(Arc::new(img)), i, i as f64 * 1.0, 1.0);
            slices.push(slice);
        }

        vr.set_slices(slices);

        let mip = vr.render_mip().unwrap();
        assert_eq!(mip.width, 64);
        assert_eq!(mip.height, 64);

        let val = mip.gray_intensity(32, 32);
        assert!(val >= 50 as f32);
    }
```
```rust
    #[test]
    fn test_set_slices_and_ordering() {
        let mut vr = VolumeRendering::new();

        let mut slices = vec![
            CtSliceInfo::new(Some(make_empty_gray(32, 32)), 2, 20.0, 1.0),
            CtSliceInfo::new(Some(make_empty_gray(32, 32)), 0, 0.0, 1.0),
            CtSliceInfo::new(Some(make_empty_gray(32, 32)), 1, 10.0, 1.0),
        ];

        vr.set_slices(slices);

        assert_eq!(vr.slices.len(), 3);
        assert!(vr.slices[0].slice_location <= vr.slices[1].slice_location);
        assert!(vr.slices[1].slice_location <= vr.slices[2].slice_location);
    }
```
```rust
    #[test]
    fn test_extract_slice() {
        let mut vr = VolumeRendering::new();

        let img = make_empty_gray(32, 32);
        let slice = CtSliceInfo::new(Some(img.clone()), 0, 5.0, 1.0);
        vr.set_slices(vec![slice]);

        let extracted = vr.extract_slice(5.1).unwrap();
        assert_eq!(Arc::ptr_eq(&extracted, &img), true);
    }
```
```rust
    #[test]
    fn test_voxel_intensity() {
        let mut vr = VolumeRendering::new();

        let mut img = Image::new_gray(16, 16);
        img.set_gray(5, 5, 128);
        let slice = CtSliceInfo::new(Some(Arc::new(img)), 0, 0.0, 1.0);
        vr.set_slices(vec![slice]);

        let value = vr.voxel_intensity(5, 5, 0).unwrap();
        assert_eq!(value, 128.0);
    }
```
```rust
    #[test]
    fn test_invalid_voxel_access() {
        let vr = VolumeRendering::new();
        assert!(vr.voxel_intensity(0, 0, 0).is_none());
        assert!(vr.voxel_intensity(0, 0, -1).is_none());
    }

}
```

---
