# 어댑터(Adaptor)”/“브리지(Bridge)
우리가 만든 DataSeriesLike/ChannelLike는 전형적인 “어댑터(Adaptor)”/“브리지(Bridge)” 스타일을 Rust의 트레이트로 풀어낸 겁니다.  
핵심은 알고리즘(함수) ↔ 데이터구조(타입) 를 느슨하게 결합해서 재사용성과 성능을 동시에 얻는 것.

## 왜 이 기법이 좋은가 (일반적 관점)

- 결합도↓, 재사용성↑: 알고리즘은 “시간축/값을 읽을 수 있다”는 계약(인터페이스)만 의존합니다. 
- 실제 타입이 Channel, (xs, ys) 튜플, DataSeries 등 무엇이든 상관없음.
- 테스트 용이성: 무거운 실전 타입 대신 가벼운 어댑터(예: 슬라이스 튜플)로 알고리즘을 쉽게 단위 테스트.
- 점진적 이식/교체: 기존 C++ 구조를 그대로 옮겨오면서도 표면을 트레이트로 맞추면, 내부 구현은 나중에 바꿔도 외부 알고리즘은 그대로.
- 여러 표현을 같은 API로: 원시 슬라이스, 소유 Vec, 참조 타입, 캐시 래퍼 등 다양한 “표현”을 같은 함수에 넣을 수 있음.

## 왜 Rust에서 특히 더 좋나
- 제로코스트 추상화: 제네릭+트레이트(정적 디스패치)는 컴파일 타임에 모노모픽으로 풀려서 가상 호출 비용 없이 인라인 최적화됩니다.
- 안전한 인터페이스: 소유권/수명 검사로 잘못된 참조, UAF를 컴파일러가 잡아줌.
- 확장 가능한 다형성: 상속이 없는 Rust에서 “행동 공유”는 트레이트가 정석. 기본 메서드 제공으로 공통 유틸도 쉽게 얹음.
- 어댑터를 쉽게 합성: 참조 타입(&T), 튜플((&[f64], &[f64])), 새 타입(newtype) 등 여러 레이어의 어댑터를 붙여도 비용이 거의 없음.
- 동적 디스패치 옵션: 필요하면 Box<dyn ChannelLike>로 이질적 컬렉션도 담을 수 있음(성능/유연성 트레이드오프를 선택적으로).

## 우리 코드에 바로 대응되는 이점

### 알고리즘-타입 분리
- get_channel_total_sum* 류 함수는 Channel에 고정되지 않고 ChannelLike만 요구  
    → 나중에 파일 스트리밍 채널, 메모리맵 채널, 가공 파이프 채널도 그대로 사용 가능.

- 테스트 더 가볍게  
    실제 Channel을 만들지 않고도 (&xs[..], &ys[..]) 같은 슬라이스 튜플에 DataSeriesLike만 구현해서 단위 테스트를 돌릴 수 있음.

- 성능 그대로
    제네릭 바운드(C: ChannelLike)는 대부분 정적 디스패치 → C++ 인라인에 대응되는 최적화가 자연스럽게 일어남.

- 참조/뷰를 그대로 사용  
    impl<T: ChannelLike + ?Sized> ChannelLike for &T 같은 참조에 대한 블랭킷 구현으로, &Channel도 바로 넣을 수 있어 불필요한 복제를 피함.

### 간단 예시 (요약)
```rust
pub trait DataSeriesLike {
    fn xs(&self) -> &[f64];
    fn ys(&self) -> &[f64];
}

pub trait ChannelLike: DataSeriesLike {
    fn channel_name(&self) -> &str;
}

// 가벼운 어댑터: 슬라이스 튜플도 시리즈처럼 취급
impl<'a> DataSeriesLike for (&'a [f64], &'a [f64]) {
    fn xs(&self) -> &[f64] { self.0 }
    fn ys(&self) -> &[f64] { self.1 }
}

// 알고리즘은 트레이트에만 의존
pub fn area_under_curve<C: DataSeriesLike>(c: &C) -> f64 {
    let (xs, ys) = (c.xs(), c.ys());
    xs.windows(2).zip(ys.windows(2))
        .map(|(xt, yt)| (xt[1]-xt[0]) * 0.5 * (yt[0]+yt[1]))
        .sum()
}
```

## 언제 어떤 방식?
- 최고 성능·제네릭: fn foo<T: ChannelLike>(t: &T) (정적 디스패치, 모노모픽)
- 런타임 다형성: fn foo(t: &dyn ChannelLike) (동적 디스패치, 이질적 컬렉션/플러그인)
- 타입소유 바꾸지 못할 때: 새 타입(Newtype) 또는 래퍼로 트레이트를 “부여”해서 어댑터화
- 외부타입에 직접 구현 못할 때(고아 규칙): 같은 크레이트 안의 새 타입으로 감싸서 구현
- 요약하면, Rust의 트레이트 기반 어댑터는 C++의 인터페이스/어댑터 패턴을 더 안전하고, 더 빠르고, 더 쓰기 쉽게 만든 형태예요.  
    우리 프로젝트처럼 다양한 채널 표현을 같은 처리 파이프라인에 태우고, 테스트를 가볍게 돌리고, 나중에 타입을 바꿀 자유를 확보하려면  
    가장 적합한 접근입니다.

---
# DataSeries 가 없어도 됌
특정 struct DataSeries가 없어도, 그 타입이 DataSeriesLike 트레이트만 구현하면 해당 함수를 그대로 쓸 수 있음.  
Rust에선 “타입이 무엇이냐”보다 “그 타입이 무슨 동작(메서드) 을 제공하느냐(트레이트)로 다형성을 얻습니다.

## 핵심 포인트만 정리하면:
- 함수는 DataSeriesLike에만 의존 → 그 트레이트를 impl 한 어떤 타입이든 인자로 넣을 수 있음.
- 실제 데이터 구조가 Channel, 튜플, 래퍼 등 무엇이든 상관없음.
- 제네릭 + 트레이트는 정적 디스패치(제로코스트) 라서 성능 손해도 없음.

## 예시들:
```rust
pub trait DataSeriesLike {
    fn xs(&self) -> &[f64];
    fn ys(&self) -> &[f64];
}

// 1) 가벼운 어댑터: (xs, ys) 슬라이스 튜플
impl<'a> DataSeriesLike for (&'a [f64], &'a [f64]) {
    fn xs(&self) -> &[f64] { self.0 }
    fn ys(&self) -> &[f64] { self.1 }
}

// 2) 실제 타입(예: Channel)에도 같은 트레이트만 구현
struct Channel { xs: Vec<f64>, ys: Vec<f64>, name: String }
impl DataSeriesLike for Channel {
    fn xs(&self) -> &[f64] { &self.xs }
    fn ys(&self) -> &[f64] { &self.ys }
}

// 3) 알고리즘은 트레이트에만 의존
pub fn peak_y(series: &impl DataSeriesLike) -> f64 {
    series.ys().iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

// 사용 예
let xs = [0.0, 1.0, 2.0];
let ys = [1.0, 3.0, 2.0];
let p1 = peak_y(&(xs.as_slice(), ys.as_slice())); // 튜플로 사용

let ch = Channel { xs: xs.to_vec(), ys: ys.to_vec(), name: "A".into() };
let p2 = peak_y(&ch); // 실제 채널 타입으로 사용
```

## 보너스 팁:
- 외부 타입(고아 규칙 때문에 직접 impl 못 하는 경우)은 새 래퍼 타입(newtype) 으로 감싸서 impl DataSeriesLike for MyWrapper 하면 됩니다.
- 여러 타입을 한 컨테이너에 담아 다루려면 Vec<Box<dyn DataSeriesLike>> 같은 동적 디스패치도 선택 가능(유연성↑, 약간의 비용).
- 함수 시그니처를 fn f(s: &impl DataSeriesLike) 또는 fn f<T: DataSeriesLike + ?Sized>(s: &T)로 두면,  
    참조만 넘겨도 충분해서 추가 레퍼런스용 impl가 필요 없습니다.

---


## 실제 코드
```rust
// src/core/lib_channel.rs
#![allow(clippy::many_single_char_names)]

use std::cmp::{max_by, min_by};
use crate::core::channel::Channel;
use crate::math::data_series::DataSeries;

/// 상대 오차로 동등성 판정
#[inline]
pub fn check_equal(a: f64, b: f64, rel_tol: f64) -> bool {
    (a - b).abs() <= rel_tol * a.abs().max(b.abs())
}

/// C++: DiffrXYVal(...) -> (값, 코드)
/// 코드 의미(원본 로직 유지):
///  0: 일반
///  1: 좌측 차분 사용
///  2: 우측 차분 사용
///  3: 보간 혼합
/// -3: 수평(미분 0) / -4: 수직(무한대) / -5: dir 잘못됨
pub fn diffr_xy_val(
    x0: f64, x1: f64, x2: f64,
    y0: f64, y1: f64, y2: f64,
    dir: i32,
) -> (f64, i32) {
    let mut code = 0;

    let x = match dir {
        0 => x0,
        1 => x1,
        2 => x2,
        _ => return (0.0, -5),
    };

    if x0 == x1 && x1 == x2 {
        if (dir == 1 && y0 == y2) || (dir == 0 && y0 == y1) || (dir == 2 && y1 == y2) {
            return (0.0, -3);
        } else {
            // 수직
            return (if y2 > y0 { f64::MAX } else { f64::MIN }, -4);
        }
    } else if x0 == x1 {
        code = 2;
        return ((y2 - y1) / (x2 - x1), code);
    } else if x1 == x2 {
        code = 1;
        return ((y1 - y0) / (x1 - x0), code);
    } else {
        let alpha = (x1 - x0).abs() / ((x1 - x0).abs() + (x2 - x1).abs());

        if alpha < 0.25 && check_equal(x1, x0, 1.0e-5) {
            code = 2;
            return ((y2 - y1) / (x2 - x1), code);
        } else if alpha > 0.75 && check_equal(x1, x2, 1.0e-5) {
            code = 1;
            return ((y1 - y0) / (x1 - x0), code);
        } else if (x1 > x0.max(x2)) || (x1 < x0.min(x2)) {
            if dir == 1 {
                code = 3;
                return (
                    alpha * (y2 - y1) / (x2 - x1) + (1.0 - alpha) * (y1 - y0) / (x1 - x0),
                    code,
                );
            } else if dir == 0 {
                code = 1;
                return ((y1 - y0) / (x1 - x0), code);
            } else {
                code = 2;
                return ((y2 - y1) / (x2 - x1), code);
            }
        } else {
            if (alpha <= 0.5 && check_equal(x1, x0, 1.0e-5))
                || (alpha > 0.5 && check_equal(x1, x2, 1.0e-5))
            {
                code = 3;
            }
            let xt = 2.0 * x;
            let v =
                y0 * (xt - x1 - x2) / ((x0 - x1) * (x0 - x2)) +
                    y1 * (xt - x0 - x2) / ((x1 - x0) * (x1 - x2)) +
                    y2 * (xt - x0 - x1) / ((x2 - x0) * (x2 - x1));
            return (v, code);
        }
    }
}

/// C++: DiffrXYTVal(...) -> (값, 코드)
pub fn diffr_xyt_val(
    x0: f64, x1: f64, x2: f64, x3: f64,
    y0: f64, y1: f64, y2: f64, y3: f64,
) -> (f64, i32) {
    let (d0, c0) = diffr_xy_val(x0, x1, x2, y0, y1, y2, 0);
    let (d1, c1) = diffr_xy_val(x0, x1, x2, y0, y1, y2, 1);
    let (d2, c2) = diffr_xy_val(x1, x2, x3, y1, y2, y3, 1);
    if c1 == 0 && c2 == 0 {
        let alpha = (x1 - x0) / (x2 - x1);
        ((d0 + 2.0 * (d1 + alpha * (d1 - d2))) / 3.0, 0)
    } else {
        (d0, 0)
    }
}

/// 등간격 미분
pub fn calc_diff_uniform(src: &[f64], tgt: &mut [f64], step: f64, scale: f64) -> bool {
    let n = src.len();
    if n < 3 || tgt.len() != n { return false; }

    let dx_fac = scale / (2.0 * step);

    // 경계(전방/후방 3점식)
    {
        let yi = src[0];
        let yp = src[1];
        let yp2 = src[2];
        let yp3 = if n > 3 { src[3] } else { src[2] };
        tgt[0] = (-7.0 * yi + 6.0 * yp + 3.0 * yp2 - 2.0 * yp3) * dx_fac / 3.0;
    }

    for i in 1..n - 1 {
        let ym = src[i - 1];
        let yp = src[i + 1];
        tgt[i] = (yp - ym) * dx_fac;
    }

    {
        let ym2 = src[n.saturating_sub(4)];
        let yi  = src[n - 2];
        let yp  = src[n - 1];
        let ym  = src[n.saturating_sub(3)];
        tgt[n - 1] = (7.0 * yp - 6.0 * yi - 3.0 * ym + 2.0 * ym2) * dx_fac / 3.0;
    }
    true
}

/// 비등간격 미분(시간 벡터 필요)
pub fn calc_diff_nonuniform(time: &[f64], src: &[f64], tgt: &mut [f64], scale: f64) -> bool {
    let n = time.len();
    if n < 3 || src.len() != n || tgt.len() != n { return false; }

    // i=0
    {
        let (d, _) = diffr_xyt_val(time[0], time[1], time[2], time.get(3).copied().unwrap_or(time[2]),
                                   src[0],  src[1],  src[2],  src.get(3).copied().unwrap_or(src[2]));
        tgt[0] = d * scale;
    }

    for i in 1..n - 1 {
        let xm = time[i - 1];
        let xi = time[i];
        let xp = time[i + 1];
        let ym = src[i - 1];
        let yi = src[i];
        let yp = src[i + 1];
        let (mut d, code) = diffr_xy_val(xm, xi, xp, ym, yi, yp, 1);

        if code == 3 && i >= 3 && i + 3 < n {
            let (d1, _) = diffr_xyt_val(xi, time[i - 1], time[i - 2], time[i - 3],
                                        yi, src[i - 1], src[i - 2], src[i - 3]);
            let (d2, _) = diffr_xyt_val(xi, time[i + 1], time[i + 2], time[i + 3],
                                        yi, src[i + 1], src[i + 2], src[i + 3]);
            let alpha = (xi - xm).abs() / ((xi - xm).abs() + (xp - xi).abs());
            d = alpha * d2 + (1.0 - alpha) * d1;
        } else if code == 1 && i >= 3 {
            let (d1, _) = diffr_xyt_val(xi, time[i - 1], time[i - 2], time[i - 3],
                                        yi, src[i - 1], src[i - 2], src[i - 3]);
            d = d1;
        } else if code == 2 && i + 3 < n {
            let (d2, _) = diffr_xyt_val(xi, time[i + 1], time[i + 2], time[i + 3],
                                        yi, src[i + 1], src[i + 2], src[i + 3]);
            d = d2;
        }

        tgt[i] = d * scale;
    }

    // i = n-1
    {
        let (d, _) = diffr_xyt_val(time[n - 1], time[n - 2], time[n - 3], time[n - 4],
                                   src[n - 1],  src[n - 2],  src[n - 3],  src[n - 4]);
        tgt[n - 1] = d * scale;
    }
    true
}

/// 러스트스럽게 Channel/DataSeries 의존을 느슨하게 하기 위한 얇은 어댑터
pub trait ChannelLike {
    fn name(&self) -> &str;
    fn desc(&self) -> &str { "" }
    fn set_desc(&mut self, s: &str);

    fn x(&self) -> &[f64];
    fn y(&self) -> &[f64];
    fn set_xy(&mut self, xs: Vec<f64>, ys: Vec<f64>);

    fn set_sample_interval(&mut self, dt: f64);
    fn set_start_end(&mut self, t0: f64, t1: f64);

    /// 키-값 메타 저장 (없으면 no-op 로 둬도 됨)
    fn set_prop(&mut self, key: &str, val: &str);
}

pub trait DataSeriesLike {
    fn add_point(&mut self, x: f32, y: f32);
    fn set_title(&mut self, s: &str);
}

/// 채널 → 데이터시리즈
pub fn cnv_channel_to_data_series<C: ChannelLike, D: DataSeriesLike>(ch: &C, ds: &mut D) {
    let xs = ch.x();
    let ys = ch.y();
    for i in 0..xs.len().min(ys.len()) {
        ds.add_point(xs[i] as f32, ys[i] as f32);
    }
    ds.set_title(ch.name());
}

/// 시간 범위 자르기
pub fn exec_channel_trim<C: ChannelLike>(ch: &mut C, tmin: f64, tmax: f64) -> bool {
    let (xs, ys) = (ch.x(), ch.y());
    if xs.is_empty() || xs.len() != ys.len() { return false; }

    let mut xo = Vec::with_capacity(xs.len());
    let mut yo = Vec::with_capacity(xs.len());
    for (&x, &y) in xs.iter().zip(ys) {
        if x >= tmin && x <= tmax {
            xo.push(x);
            yo.push(y);
        }
    }
    if xo.len() < 2 { return false; }

    let dt = if xo.len() >= 2 { xo[1] - xo[0] } else { 0.0 };
    ch.set_xy(xo.clone(), yo);
    ch.set_sample_interval(dt);
    ch.set_start_end(xo[0], *xo.last().unwrap());
    true
}

/// Y 반전
pub fn exec_channel_reverse<C: ChannelLike>(ch: &mut C) -> bool {
    let (xs, ys) = (ch.x(), ch.y());
    if xs.len() != ys.len() { return false; }
    let mut yo = ys.to_vec();
    for y in &mut yo { *y = -*y; }
    ch.set_xy(xs.to_vec(), yo);
    true
}

/// (max, min) 계산
pub fn calc_max_min_value(values: &[f64]) -> Option<(f64, f64)> {
    if values.is_empty() { return None; }
    let mut vmax = values[0];
    let mut vmin = values[0];
    for &v in &values[1..] {
        if v > vmax { vmax = v; }
        if v < vmin { vmin = v; }
    }
    Some((vmax, vmin))
}

/// 오프셋 타입
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OffsetType { #[default]
None, Max, Min, Zero, Range, TimeStart, TimeEnd }

/// C++ ChannelOption 대체(필요 필드만)
#[derive(Clone, Debug, Default)]
pub struct ChannelOption {
    pub cut_time: bool,
    pub cut_start_time: f64,
    pub cut_end_time: f64,

    pub off_typ: OffsetType,
    pub offset_start_time: f64,
    pub offset_end_time: f64,
}

/// 시간 컷 + 결과 통계
pub fn calc_channel_time_cut(
    opt: &ChannelOption,
    x: &[f64], y: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, f64, f64, f64, f64)> {
    if x.len() < 2 || x.len() != y.len() { return None; }

    let (mut xo, mut yo) = if !opt.cut_time {
        (x.to_vec(), y.to_vec())
    } else {
        let mut xi = Vec::with_capacity(x.len());
        let mut yi = Vec::with_capacity(x.len());
        for (&xx, &yy) in x.iter().zip(y) {
            if xx >= opt.cut_start_time && xx <= opt.cut_end_time {
                xi.push(xx);
                yi.push(yy);
            }
        }
        if xi.is_empty() { return None; }
        (xi, yi)
    };

    let (t0, t1) = (*xo.first().unwrap(), *xo.last().unwrap());
    let (vmax, vmin) = calc_max_min_value(&yo)?;
    Some((xo, yo, t0, t1, vmin, vmax))
}

/// 오프셋 적용 + 결과 통계(새 Y 반환)
pub fn calc_channel_offset(
    opt: &ChannelOption,
    x: &[f64], y: &[f64],
) -> Option<(Vec<f64>, f64, f64)> {
    if x.len() < 1 || x.len() != y.len() { return None; }
    if opt.off_typ == OffsetType::None { return None; }

    let offset = match opt.off_typ {
        OffsetType::Max => calc_max_min_value(y)?.0,
        OffsetType::Min => calc_max_min_value(y)?.1,
        OffsetType::Zero => {
            let eps = 1.0e-6;
            let mut v = y[0];
            for (&xx, &yy) in x.iter().zip(y) {
                if xx >= -eps && xx <= eps { v = yy; break; }
            }
            v
        }
        OffsetType::Range => {
            let mut sum = 0.0;
            let mut cnt = 0usize;
            for (&xx, &yy) in x.iter().zip(y) {
                if xx >= opt.offset_start_time && xx <= opt.offset_end_time {
                    sum += yy; cnt += 1;
                }
            }
            if cnt == 0 { 0.0 } else { sum / (cnt as f64) }
        }
        OffsetType::TimeStart => y[0],
        OffsetType::TimeEnd   => *y.last().unwrap(),
        OffsetType::None      => 0.0,
    };

    let mut yo = Vec::with_capacity(y.len());
    for &yy in y { yo.push(yy - offset); }
    let (vmax, vmin) = calc_max_min_value(&yo)?;
    Some((yo, vmin, vmax))
}

/// 배열 → 채널 설정(메타 포함)
pub fn cnv_array_to_channel<C: ChannelLike>(
    name: &str, desc: &str,
    xs: &[f64], ys: &[f64],
    ch: &mut C,
) -> bool {
    if xs.len() < 2 || xs.len() != ys.len() { return false; }

    let t0 = xs[0];
    let t1 = *xs.last().unwrap();
    let dt = xs[1] - xs[0];

    ch.set_xy(xs.to_vec(), ys.to_vec());
    ch.set_sample_interval(dt);
    ch.set_start_end(t0, t1);
    ch.set_desc(desc);

    if let Some((vmax, vmin)) = calc_max_min_value(ys) {
        ch.set_prop("Name", name);
        ch.set_prop("Desc", desc);
        ch.set_prop("Num of Channel", &format!("{}", xs.len()));
        ch.set_prop("Start Time", &format!("{}", t0));
        ch.set_prop("End Time", &format!("{}", t1));
        ch.set_prop("Time Interval", &format!("{}", dt));
        ch.set_prop("Maximum", &format!("{}", vmax));
        ch.set_prop("Minimum", &format!("{}", vmin));
    }
    true
}

/// 데이터시리즈 → 채널 (원본 C++에는 getXValues를 두 번 호출하는 버그가 있었음. 여기선 X/Y 모두 가져온다고 가정)
pub trait DataSeriesReader {
    fn x_values(&self) -> Vec<f64>;
    fn y_values(&self) -> Vec<f64>;
    fn title(&self) -> &str;
}

pub fn cnv_data_series_to_channel<C: ChannelLike, D: DataSeriesReader>(
    ds: &D, ch: &mut C,
) -> bool {
    let xs = ds.x_values();
    let ys = ds.y_values();
    if xs.len() < 2 || xs.len() != ys.len() { return false; }
    cnv_array_to_channel(ds.title(), ds.title(), &xs, &ys, ch)
}

/// 여러 채널 합(Y 합산)
pub fn get_channel_total_sum<C: ChannelLike>(chs: &[C]) -> Option<(Vec<f64>, Vec<f64>, f64)> {
    let first = chs.first()?;
    let xs = first.x().to_vec();
    let n = xs.len();
    if n == 0 { return None; }

    let mut sumy = vec![0.0; n];
    for ch in chs {
        if ch.x().len() != n || ch.y().len() != n { return None; }
        for i in 0..n { sumy[i] += ch.y()[i]; }
    }
    let vmax = sumy.iter().copied().fold(f64::MIN, f64::max);
    Some((xs, sumy, vmax))
}

/// 합산 후 임계치 이상 구간의 (시작, 종료) 시간 찾기
pub fn get_channel_total_sum_by_limit<C: ChannelLike>(
    chs: &[C],
    limit: f64,
) -> Option<(Vec<f64>, Vec<f64>, f64, f64)> {
    let (xs, ys, _) = get_channel_total_sum(chs)?;

    // xs가 move 되기 전에 기본값을 꺼내 둔다
    let default_t0 = *xs.first()?;        // 또는 xs[0]
    let default_t1 = *xs.last()?;         // 맨 끝

    let (mut t0, mut t1) = (None, None);
    for i in 0..ys.len() {
        if t0.is_none() && ys[i] >= limit {
            t0 = Some(xs[i]);
        }
        if t0.is_some() && ys[i] <= limit {
            t1 = Some(xs[i]);
            break;
        }
    }

    let t0 = t0.unwrap_or(default_t0);
    let t1 = t1.unwrap_or(default_t1);

    // 여기서 xs, ys를 move
    Some((xs, ys, t0, t1))
}

/// 전체 채널들의 (최솟값, 최댓값)
pub fn get_channel_total_min_max<C: ChannelLike>(chs: &[C]) -> Option<(f64, f64)> {
    if chs.is_empty() { return None; }
    let mut gmin = f64::MAX;
    let mut gmax = f64::MIN;
    for ch in chs {
        let (vmax, vmin) = calc_max_min_value(ch.y())?;
        if vmax > gmax { gmax = vmax; }
        if vmin < gmin { gmin = vmin; }
    }
    Some((gmin, gmax))
}

/// 각 채널의 피크(최대값) 배열
pub fn get_channel_total_peak_array<C: ChannelLike>(chs: &[C]) -> Option<Vec<f64>> {
    if chs.is_empty() { return None; }
    let mut peaks = Vec::with_capacity(chs.len());
    for ch in chs {
        let (vmax, _) = calc_max_min_value(ch.y())?;
        peaks.push(vmax);
    }
    Some(peaks)
}

impl DataSeriesLike for DataSeries {
    fn add_point(&mut self, x: f32, y: f32) { self.add_point(x, y); }
    fn set_title(&mut self, s: &str) { self.set_title(s); }
}


impl ChannelLike for Channel {
    fn name(&self) -> &str { self.channel_name() }
    fn set_desc(&mut self, s: &str) { self.set_channel_desc(s.to_string()); }
    fn x(&self) -> &[f64] { self.data_x().as_slice() }
    fn y(&self) -> &[f64] { self.data_y().as_slice() }
    fn set_xy(&mut self, xs: Vec<f64>, ys: Vec<f64>) { self.set_xy(xs, ys) }
    fn set_sample_interval(&mut self, dt: f64) { self.set_sample_interval(dt) }
    fn set_start_end(&mut self, t0: f64, t1: f64) { self.set_start_end(t0, t1) }
    fn set_prop(&mut self, k: &str, v: &str) { let _ = self.props.set(k, v); }
}


impl DataSeriesReader for DataSeries {
    fn x_values(&self) -> Vec<f64> { self.x_values().to_vec() }
    fn y_values(&self) -> Vec<f64> { self.y_values().to_vec() }
    fn title(&self) -> &str { self.title() }
}



```