# Channel 정리
아래는 Channel 구조체와 관련 메서드들을 문서 형식으로 정리한 요약 문서입니다.  
이 문서는 구조 설명, 주요 기능, 트레잇, 유틸리티 함수, 개선 포인트까지 포함합니다.

## 📘 Channel 구조 문서 요약
### 🧩 구조 개요
Channel은 시계열 또는 CAD 데이터의 단일 채널을 표현하는 구조체로, X/Y 데이터와 메타데이터를 함께 관리합니다.
| 필드명          | 타입           | 설명                                                             |
|-----------------|----------------|------------------------------------------------------------------|
| name            | String         | 채널의 이름. 사용자 또는 시스템이 지정하는 식별자 역할             |
| desc            | String         | 채널에 대한 설명. 용도, 출처, 의미 등을 기술할 수 있음             |
| num_sample      | usize          | 데이터 샘플 개수. `data_x` 또는 `data_y`의 길이와 일치해야 함       |
| offset_value    | f64            | Y 데이터에 적용된 오프셋 값. 보정 또는 기준점 이동에 사용됨         |
| start_time      | f64            | 데이터의 시작 시간. `data_x`의 첫 번째 값과 일치함                  |
| end_time        | f64            | 데이터의 종료 시간. `data_x`의 마지막 값과 일치함                   |
| sample_interval | f64            | 샘플 간격. 등간격일 경우 `data_x[1] - data_x[0]`으로 계산됨         |
| data_x          | TArray<f64>    | 시간 또는 X축 데이터. 시계열 또는 CAD 좌표의 기준축 역할            |
| data_y          | TArray<f64>    | 값 또는 Y축 데이터. 측정값, 속성값, CAD 요소의 세부값 등으로 사용됨 |
| props           | KeyValuePool   | 채널의 메타데이터 저장소. 이름, 설명, 통계값 등을 키-값으로 관리     |


## 🧪 주요 메서드 기능
### 📌 기본 설정 및 접근
| 메서드명                          | 입력 타입         | 설명                                                             |
|-----------------------------------|-------------------|------------------------------------------------------------------|
| `new()`                           | -                 | 기본 생성자. `Default` 트레잇을 통해 초기화된 Channel 반환         |
| `set_channel_name(name)`          | `impl Into<String>` | 채널 이름 설정                                                   |
| `set_channel_desc(desc)`          | `impl Into<String>` | 채널 설명 설정                                                   |
| `set_data_sample_size(n)`         | `usize`           | 샘플 개수 설정 (`num_sample`)                                     |
| `set_data_sample_interval(dt)`    | `f64`             | 샘플 간격 설정 (`sample_interval`)                                |
| `set_data_x(x)` / `set_data_y(y)` | `TArray<f64>`     | X/Y 데이터를 `TArray`로 직접 설정                                 |
| `set_data_vec_x(x)` / `set_data_vec_y(y)` | `Vec<f64>` | X/Y 데이터를 `Vec<f64>`로 설정하여 내부 `TArray`에 복사           |
| `data_x()` / `data_y()`           | `&TArray<f64>`    | X/Y 데이터에 대한 불변 참조 반환                                  |
| `data_x_mut()` / `data_y_mut()`   | `&mut TArray<f64>`| X/Y 데이터에 대한 가변 참조 반환                                  |


### 📌 데이터 처리 및 분석
| 메서드명                               | 입력 타입                        | 설명                                                                 |
|----------------------------------------|----------------------------------|----------------------------------------------------------------------|
| `cut_time(start, end)`                 | `f64, f64`                       | 시간 범위 기반으로 X/Y 데이터를 잘라내고 메타데이터를 갱신합니다.       |
| `cut_time_index(start_id, end_id)`     | `isize, isize`                   | 인덱스 범위 기반으로 X/Y 데이터를 잘라내고 메타데이터를 갱신합니다.     |
| `change_channel_data(x, y)`            | `TArray<f64>, TArray<f64>`       | X/Y 데이터를 교체하고 시간/샘플 수/통계 메타데이터를 갱신합니다.        |
| `set_offset(offsetter)`                | `impl ChannelOffset`             | 외부 오프셋 계산기를 통해 Y 데이터에 오프셋을 적용하고 통계값을 갱신합니다. |
| `apply_filter(name, engine)`           | `&str, impl FilterEngine`        | 필터 이름을 기반으로 Y 데이터에 필터를 적용하고 통계값을 갱신합니다.     |
| `apply_filter_kind(kind, engine)`      | `ConvFilter, impl FilterEngine`  | 필터 종류(enum)을 기반으로 필터를 적용합니다.                          |


### 📌 통계 및 메타데이터
| 메서드명                          | 입력 타입 | 설명                                                                 |
|-----------------------------------|-----------|----------------------------------------------------------------------|
| `calc_min_max()`                  | -         | Y 데이터의 최소값과 최대값을 계산하여 `(min, max)` 형태로 반환합니다. |
| `calc_max_min_value()`            | -         | Y 데이터의 최대값과 최소값을 `(max, min)` 순서로 반환합니다.          |
| `calc_abs_max_min_value()`        | -         | Y 데이터의 절댓값 기준으로 최대/최소값을 계산하여 반환합니다.         |
| `calc_max_min_value_time()`       | -         | Y 데이터의 최대/최소값과 해당 시간(X값)을 함께 반환합니다.            |
| `refresh_time_and_counts_from_x()`| -         | X 데이터 기반으로 시작/종료 시간, 샘플 수, 샘플 간격을 자동 갱신합니다. |


### 📌 메타데이터 관리
| 메서드명                          | 입력 타입         | 설명                                                                 |
|-----------------------------------|-------------------|----------------------------------------------------------------------|
| `key_values()` / `key_values_mut()` | `KeyValuePool`   | 채널의 메타데이터에 대한 불변/가변 접근을 제공합니다.                 |
| `add_key_val_data(key, val)`     | `impl Into<String>` | 키-값 쌍을 메타데이터에 추가합니다.                                 |
| `update_props_basic()`           | -                 | `Name`, `Desc`만 메타데이터에 반영합니다.                            |
| `update_props_all()`             | -                 | 모든 메타데이터(`Name`, `Desc`, `Num`, `Time`, `Min/Max`)를 자동 갱신합니다. |
| `update_props_all_with_min_max(y_min, y_max)` | `f64, f64` | Min/Max 값을 외부에서 받아서 메타데이터를 갱신합니다.                |
| `set_props_pairs(pairs)`         | `Iterator<(&'static str, String)>` | 키-값 쌍을 반복자로 받아서 메타데이터에 일괄 설정합니다. |


## 🧩 관련 트레잇
### 🔧 ChannelOffset
```rust
pub trait ChannelOffset {
    fn calc_channel_offset(
        &self,
        x: &TArray<f64>,
        y: &TArray<f64>,
    ) -> Option<(TArray<f64>, f64, f64)>;
}
```
- Y 데이터에 오프셋을 적용하고, 새로운 Y + Min/Max 반환

### 🧩 유틸리티 함수
| 함수명                      | 입력 타입           | 설명                                                                 |
|-----------------------------|---------------------|----------------------------------------------------------------------|
| `calc_min_max_slice(s)`     | `&[f64]`            | 주어진 슬라이스에서 최소값과 최대값을 계산하여 `(min, max)` 형태로 반환합니다. |
| `find_index_within_eps(xs, target, eps)` | `&[f64], f64, f64` | 오차 허용 범위 `eps` 내에서 `target` 값과 가장 가까운 인덱스를 찾아 반환합니다. |

---

## 소스 코드
```rust
use crate::core::key_value_pool::KeyValuePool;
use crate::core::tarray::TArray;
use crate::math::math_extra::ON_TOL6;
use crate::traits::filter_engine::FilterEngine;
use crate::utils::filter::filter::ConvFilter;

const TIME_INTERVAL : f64 = 0.0001;
#[derive(Clone, Debug, Default)]
pub struct Channel {
    name: String,
    desc: String,
    num_sample: usize,
    offset_value: f64,

    start_time: f64,
    end_time: f64,
    sample_interval: f64,

    data_x: TArray<f64>,
    data_y: TArray<f64>,

    pub(crate) props: KeyValuePool,
}
```
```rust
impl Channel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_data_sample_size(&mut self, n: usize) {
        self.num_sample = n;
    }

    pub fn data_sample_size(&self) -> usize {
        self.num_sample
    }

    pub fn set_data_sample_interval(&mut self, dt: f64) {
        self.sample_interval = dt;
    }

    pub fn data_sample_interval(&self) -> f64 {
        self.sample_interval
    }

    pub fn set_channel_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn channel_name(&self) -> &str {
        &self.name
    }

    pub fn set_channel_desc(&mut self, desc: impl Into<String>) {
        self.desc = desc.into();
    }

    pub fn channel_desc(&self) -> &str {
        &self.desc
    }

    pub fn set_start_time(&mut self, t: f64) {
        self.start_time = t;
    }

    pub fn start_time(&self) -> f64 {
        self.start_time
    }

    pub fn set_end_time(&mut self, t: f64) {
        self.end_time = t;
    }

    pub fn end_time(&self) -> f64 {
        self.end_time
    }

    pub fn data_x(&self) -> &TArray<f64> {
        &self.data_x
    }

    pub fn data_y(&self) -> &TArray<f64> {
        &self.data_y
    }

    pub fn data_x_mut(&mut self) -> &mut TArray<f64> {
        &mut self.data_x
    }

    pub fn data_y_mut(&mut self) -> &mut TArray<f64> {
        &mut self.data_y
    }

    pub fn set_data_x(&mut self, x: TArray<f64>) {
        self.data_x = x
    }
    pub fn set_data_y(&mut self, y: TArray<f64>) {
        self.data_y = y;
    }

    pub fn set_data_vec_x(&mut self, x: Vec<f64>) {
        self.data_x.set_data(x)
    }
    pub fn set_data_vec_y(&mut self, y: Vec<f64>) {
        self.data_y.set_data(y)
    }

    pub fn key_values(&self) -> &KeyValuePool {
        &self.props
    }
    pub fn key_values_mut(&mut self) -> &mut KeyValuePool {
        &mut self.props
    }

    pub fn add_key_val_data(&mut self, key: impl Into<String>, val: impl Into<String>) {
        let _ = self.props.set(key, val);
    }

    pub fn clear(&mut self) {
        self.data_x.remove_all();
        self.data_y.remove_all();
        self.props.clear();
        self.num_sample = 0;
        self.sample_interval = 0.0;
        self.start_time = 0.0;
        self.end_time = 0.0;
        self.offset_value = 0.0;
        self.name.clear();
        self.desc.clear();
    }

    pub fn copy_from(&mut self, other: &Channel) {
        self.name = other.name.clone();
        self.desc = other.desc.clone();
        self.num_sample = other.num_sample;
        self.offset_value = other.offset_value;

        self.start_time = other.start_time;
        self.end_time = other.end_time;
        self.sample_interval = other.sample_interval;

        self.data_x = other.data_x.clone();
        self.data_y = other.data_y.clone();

        self.props.clear();
        for (k, v) in other.props.iter_in_insert_order() {
            let _ = self.props.set(k, v);
        }
    }

    pub fn from_xy(
        chn_name: impl Into<String>,
        desc: impl Into<String>,
        data_x: TArray<f64>,
        data_y: TArray<f64>,
    ) -> Self {
        let mut ch = Channel::new();
        ch.set_channel_name(chn_name.into());
        ch.set_channel_desc(desc.into());

        ch.set_data_x(data_x);
        ch.set_data_y(data_y);

        ch.refresh_time_and_counts_from_x();

        let (y_min, y_max) = ch.calc_min_max().unwrap_or((0.0, 0.0));

        let ch_name = ch.channel_name().to_string();
        let ch_desc = ch.channel_desc().to_string();

        let _ = ch.props.set("Name", ch_name);
        let _ = ch.props.set("Desc", ch_desc);
        let _ = ch
            .props
            .set("Num of Channel", ch.data_x.get_count().to_string());
        let _ = ch.props.set("Start Time", format!("{}", ch.start_time));
        let _ = ch.props.set("End Time", format!("{}", ch.end_time));
        let _ = ch
            .props
            .set("Time Interval", format!("{}", ch.sample_interval));
        let _ = ch.props.set("Maximum", format!("{}", y_max));
        let _ = ch.props.set("Minimum", format!("{}", y_min));
        ch
    }

    pub fn rename(&mut self, chn_name: impl Into<String>, desc: impl Into<String>) {
        let chn_name = chn_name.into();
        let desc = desc.into();
        self.set_channel_name(chn_name.clone());
        self.set_channel_desc(desc.clone());
        let _ = self.props.set("Name", chn_name);
        let _ = self.props.set("Desc", desc);
    }

    pub fn cut_time(&mut self, start: f64, end: f64) -> bool {
        if self.data_x.is_empty() || start > end {
            return false;
        }

        let idx_start = on_find_index_within_eps(self.data_x.as_slice(), start, ON_TOL6).unwrap_or(0);
        let idx_end = on_find_index_within_eps(self.data_x.as_slice(), end, ON_TOL6).unwrap_or(0);
        if idx_end < idx_start {
            return false;
        }

        let n = idx_end - idx_start + 1;
        let mut new_x = TArray::with_size(n);
        let mut new_y = TArray::with_size(n);

        for (i, j) in (idx_start..=idx_end).enumerate() {
            new_x[i] = self.data_x[j];
            new_y[i] = self.data_y[j];
        }

        self.start_time = new_x[0];
        self.end_time = new_x[n - 1];
        self.num_sample = n;
        self.sample_interval = if n > 1 { new_x[1] - new_x[0] } else { 0.0 };

        self.props.clear();
        let (y_min, y_max) = on_calc_min_max_slice(new_y.as_slice()).unwrap_or((0.0, 0.0));

        let ch_name = self.channel_name().to_string();
        let ch_desc = self.channel_desc().to_string();

        let _ = self.props.set("Name", ch_name);
        let _ = self.props.set("Desc", ch_desc);
        let _ = self.props.set("Num of Channel", n.to_string());
        let _ = self.props.set("Start Time", format!("{}", self.start_time));
        let _ = self.props.set("End Time", format!("{}", self.end_time));
        let _ = self
            .props
            .set("Time Interval", format!("{}", self.sample_interval));
        let _ = self.props.set("Maximum", format!("{}", y_max));
        let _ = self.props.set("Minimum", format!("{}", y_min));

        self.data_x = new_x;
        self.data_y = new_y;
        true
    }

    pub fn cut_time_index(&mut self, start_id_1based: isize, end_id_1based: isize) -> bool {
        if self.data_x.is_empty() {
            return false;
        }

        let len = self.data_x.get_size() as isize;
        let mut s = start_id_1based - 1;
        let mut e = end_id_1based - 1;

        if s < 0 {
            s = 0;
        }
        if e >= len {
            e = len - 1;
        }
        if e < s {
            return false;
        }

        let n = (e - s + 1) as usize;
        let mut new_x = TArray::with_size(n);
        let mut new_y = TArray::with_size(n);

        for (i, j) in (s as usize..=e as usize).enumerate() {
            new_x[i] = self.data_x[j];
            new_y[i] = self.data_y[j];
        }

        self.num_sample = n;
        self.start_time = new_x[0];
        self.end_time = new_x[n - 1];
        self.sample_interval = if n > 1 { new_x[1] - new_x[0] } else { 0.0 };

        let (y_min, y_max) = on_calc_min_max_slice(new_y.as_slice()).unwrap_or((0.0, 0.0));

        self.props.clear();

        let ch_name = self.channel_name().to_string();
        let ch_desc = self.channel_desc().to_string();

        let _ = self.props.set("Name", ch_name);
        let _ = self.props.set("Desc", ch_desc);
        let _ = self.props.set("Num of Channel", n.to_string());
        let _ = self.props.set("Start Time", format!("{}", self.start_time));
        let _ = self.props.set("End Time", format!("{}", self.end_time));
        let _ = self
            .props
            .set("Time Interval", format!("{}", self.sample_interval));
        let _ = self.props.set("Maximum", format!("{}", y_max));
        let _ = self.props.set("Minimum", format!("{}", y_min));

        self.data_x = new_x;
        self.data_y = new_y;
        true
    }

    pub fn set_offset<O: ChannelOffset>(&mut self, offsetter: &O) -> bool {
        if let Some((new_y, y_min, y_max)) =
            offsetter.calc_channel_offset(&self.data_x, &self.data_y)
        {
            self.data_y = new_y;
            let _ = self.props.set("Maximum", format!("{}", y_max));
            let _ = self.props.set("Minimum", format!("{}", y_min));
            true
        } else {
            false
        }
    }

    pub fn apply_filter<E: FilterEngine>(&mut self, filter_name: &str, engine: &E) -> bool {
        let dt = if self.sample_interval.abs() > ON_TOL6 {
            self.sample_interval
        } else {
            TIME_INTERVAL
        };
        if let Some(filtered) = engine.apply(filter_name, &self.data_y, dt) {
            self.data_y = filtered;
            let (y_min, y_max) = self.calc_min_max().unwrap_or((0.0, 0.0));
            let _ = self.props.set("Maximum", format!("{}", y_max));
            let _ = self.props.set("Minimum", format!("{}", y_min));
            true
        } else {
            false
        }
    }

    pub fn apply_filter_kind<E: FilterEngine>(&mut self, kind: ConvFilter, engine: &E) -> bool {
        self.apply_filter(ConvFilter::as_str(&kind), engine)
    }

    pub fn calc_min_max_value(&self) -> Option<(f64, f64)> {
        self.calc_min_max().map(|(mn, mx)| (mn,mx))
    }

    pub fn calc_abs_max_min_value(&self) -> Option<(f64, f64)> {
        let s = self.data_y.as_slice();
        if s.is_empty() {
            return None;
        }
        let mut mn = s[0].abs();
        let mut mx = mn;
        for &v in &s[1..] {
            let a = v.abs();
            if a < mn {
                mn = a;
            }
            if a > mx {
                mx = a;
            }
        }
        Some((mx, mn))
    }

    pub fn calc_max_min_value_time(&self) -> Option<(f64, f64, f64, f64)> {
        let x = self.data_x.as_slice();
        let y = self.data_y.as_slice();
        if x.is_empty() || y.is_empty() || x.len() != y.len() {
            return None;
        }

        let mut min_val = y[0];
        let mut max_val = y[0];
        let mut min_t = x[0];
        let mut max_t = x[0];

        for i in 1..y.len() {
            let v = y[i];
            if v > max_val {
                max_val = v;
                max_t = x[i];
            }
            if v < min_val {
                min_val = v;
                min_t = x[i];
            }
        }
        Some((max_val, min_val, max_t, min_t))
    }

    pub fn change_channel_data(&mut self, x: TArray<f64>, y: TArray<f64>) {
        self.data_x = x;
        self.data_y = y;
        self.refresh_time_and_counts_from_x();

        let (y_min, y_max) = self.calc_min_max().unwrap_or((0.0, 0.0));
        let _ = self
            .props
            .set("Num of Channel", self.num_sample.to_string());
        let _ = self.props.set("Start Time", format!("{}", self.start_time));
        let _ = self.props.set("End Time", format!("{}", self.end_time));
        let _ = self
            .props
            .set("Time Interval", format!("{}", self.sample_interval));
        let _ = self.props.set("Maximum", format!("{}", y_max));
        let _ = self.props.set("Minimum", format!("{}", y_min));
    }

    fn refresh_time_and_counts_from_x(&mut self) {
        let n = self.data_x.get_count();
        self.num_sample = n;
        if n > 0 {
            self.start_time = self.data_x[0];
            self.end_time = self.data_x[n - 1];
        } else {
            self.start_time = 0.0;
            self.end_time = 0.0;
        }
        self.sample_interval = if n > 1 {
            self.data_x[1] - self.data_x[0]
        } else {
            0.0
        };
    }

    fn calc_min_max(&self) -> Option<(f64, f64)> {
        on_calc_min_max_slice(self.data_y.as_slice())
    }

    fn set_props_pairs<I>(&mut self, pairs: I)
    where
        I: IntoIterator<Item = (&'static str, String)>,
    {
        for (k, v) in pairs {
            let _ = self.props.set(k, v);
        }
    }

    /// Name / Desc 만 갱신 (rename 등에 사용)
    pub fn update_props_basic(&mut self) {
        let name = self.name.clone();
        let desc = self.desc.clone();
        self.set_props_pairs([("Name", name), ("Desc", desc)]);
    }

    /// 모든 메타 갱신 (Min/Max를 내부에서 계산)
    pub fn update_props_all(&mut self) {
        // 1) 로컬 소유값 준비(빌림 충돌 방지)
        let name = self.name.clone();
        let desc = self.desc.clone();
        let num = self.data_x.get_count().to_string();
        let st = self.start_time.to_string();
        let et = self.end_time.to_string();
        let dt = self.sample_interval.to_string();

        // 2) Y 통계 계산(불변 대여) -> 값 복사 후 종료
        let (y_min, y_max) = self.calc_min_max().unwrap_or((0.0, 0.0));
        let ymin = y_min.to_string();
        let ymax = y_max.to_string();

        // 3) 실제 set 호출(가변 대여) — 이미 필요한 값은 모두 소유 String 상태
        self.set_props_pairs([
            ("Name", name),
            ("Desc", desc),
            ("Num of Channel", num),
            ("Start Time", st),
            ("End Time", et),
            ("Time Interval", dt),
            ("Maximum", ymax),
            ("Minimum", ymin),
        ]);
    }

    /// 모든 메타 갱신 (Min/Max 를 이미 갖고 있을 때 사용)
    pub fn update_props_all_with_min_max(&mut self, y_min: f64, y_max: f64) {
        let name = self.name.clone();
        let desc = self.desc.clone();
        let num = self.data_x.get_count().to_string();
        let st = self.start_time.to_string();
        let et = self.end_time.to_string();
        let dt = self.sample_interval.to_string();

        self.set_props_pairs([
            ("Name", name),
            ("Desc", desc),
            ("Num of Channel", num),
            ("Start Time", st),
            ("End Time", et),
            ("Time Interval", dt),
            ("Maximum", y_max.to_string()),
            ("Minimum", y_min.to_string()),
        ]);
    }
}
```

```rust
pub trait ChannelOffset {
    fn calc_channel_offset(
        &self,
        x: &TArray<f64>,
        y: &TArray<f64>,
    ) -> Option<(TArray<f64>, f64, f64)>;
}

fn on_calc_min_max_slice(s: &[f64]) -> Option<(f64, f64)> {
    if s.is_empty() {
        return None;
    }
    let mut mn = s[0];
    let mut mx = s[0];
    for &v in &s[1..] {
        if v < mn {
            mn = v;
        }
        if v > mx {
            mx = v;
        }
    }
    Some((mn, mx))
}
```

```rust
fn on_find_index_within_eps(xs: &[f64], target: f64, eps: f64) -> Option<usize> {
    xs.iter().position(|&t| (t - target).abs() <= eps)
}
```

---





