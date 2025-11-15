# Channel

## 📘 Channel 구조체 핵심 기능 요약
### 1. 📦 생성 및 초기화

| 함수 이름     | 설명                                                                 |
|---------------|----------------------------------------------------------------------|
| `new()`       | 기본 생성자. 모든 필드를 초기값으로 설정합니다.                      |
| `from_xy()`   | 이름, 설명, X/Y 데이터를 기반으로 채널 생성 및 메타데이터 자동 설정. |
| `copy_from()` | 다른 Channel 인스턴스의 모든 속성과 데이터를 복사합니다.             |
| `clear()`     | 모든 데이터와 메타데이터를 초기화합니다.                             |


### 2. 🧪 데이터 설정 및 접근

| 함수 이름                          | 설명                                                                 |
|-----------------------------------|----------------------------------------------------------------------|
| `set_data_x`, `set_data_y`        | X 또는 Y 데이터를 `TArray<f64>`로 직접 설정합니다.                   |
| `set_data_vec_x`, `set_data_vec_y`| X 또는 Y 데이터를 `Vec<f64>`로 설정합니다 (`TArray` 내부에 복사됨).   |
| `data_x`, `data_y`                | X 또는 Y 데이터에 대한 불변 참조를 반환합니다.                        |
| `data_x_mut`, `data_y_mut`        | X 또는 Y 데이터에 대한 가변 참조를 반환합니다.                        |
| `change_channel_data()`           | X/Y 데이터를 모두 교체하고 시간/샘플 수/통계 메타데이터를 자동 갱신합니다. |



### 3. 🕒 시간 정보 설정 및 슬라이싱

| 함수 이름                             | 설명                                                                 |
|--------------------------------------|----------------------------------------------------------------------|
| `set_start_time`, `set_end_time`     | 채널의 시작 시간과 종료 시간을 수동으로 설정합니다.                  |
| `cut_time(start, end)`               | 시간 값 범위에 해당하는 구간만 잘라내어 X/Y 데이터를 슬라이싱합니다.  |
| `cut_time_index(start_id, end_id)`   | 인덱스 기반으로 데이터를 슬라이싱합니다 (1-based 인덱스 사용).       |
| `refresh_time_and_counts_from_x()`   | X 데이터 기반으로 시작/종료 시간, 샘플 수, 샘플 간격을 자동 갱신합니다. |


### 4. 📊 통계 및 메타데이터

| 함수 이름                         | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `calc_min_max()`                 | Y 데이터의 최소값과 최대값을 계산하여 `(min, max)` 형태로 반환합니다. |
| `calc_max_min_value_time()`      | Y 데이터의 최대/최소값과 해당 시간(X값)을 반환합니다. `(max, min, t_max, t_min)` |
| `calc_abs_max_min_value()`       | Y 데이터의 절대값 기준 최대/최소값을 반환합니다. `(abs_max, abs_min)` |
| `update_props_basic()`           | 채널의 이름(Name)과 설명(Desc)만 메타데이터로 갱신합니다.             |
| `update_props_all()`             | 이름, 설명, 샘플 수, 시간 정보, Y의 최대/최소값 등 모든 메타데이터를 갱신합니다. |
| `update_props_all_with_min_max()`| 외부에서 전달된 Y의 최소/최대값을 사용하여 메타데이터를 갱신합니다.   |



### 5. 🧮 필터 및 오프셋 처리

| 함수 이름             | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| `set_offset()`       | `ChannelOffset` 트레잇을 구현한 객체를 이용해 Y 데이터에 오프셋을 적용합니다. |
| `apply_filter()`     | 필터 이름(`&str`)과 `FilterEngine`을 이용해 Y 데이터에 필터를 적용합니다.     |
| `apply_filter_kind()`| `ConvFilter` 열거형을 이용해 필터를 적용합니다. 내부적으로 `apply_filter()`를 호출합니다. |


### 6. 📁 CSV 로딩 및 컨테이너

| 구조체 이름        | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| `CsvChannelLoader` | CSV 파일에서 채널 데이터를 로딩하는 구조체. 헤더와 각 열의 데이터를 관리합니다. |
| `ChannelContainer` | 여러 채널(`TArray<f64>`)을 이름 기반으로 저장하고, 일괄 처리 및 계산 기능을 제공합니다. |


### 7. CsvChannelLoader 및 ChannelContainer 주요 메서드

| 함수 이름                  | 설명                                                                 |
|---------------------------|----------------------------------------------------------------------|
| `from_path()`             | 지정된 경로의 CSV 파일을 읽어 채널 데이터를 로딩합니다.               |
| `get_column()`            | 특정 헤더 이름에 해당하는 데이터 열(`Vec<f64>`)을 반환합니다.         |
| `apply_to_all_channels()`| 모든 채널에 대해 주어진 함수를 적용합니다 (`FnMut(&mut TArray<f64>)`). |
| `compute_injury_metric()`| 지정된 채널들에 대해 사용자 정의 계산 함수를 적용하여 결과를 반환합니다. |
| `get_channel_names()`     | 현재 저장된 모든 채널의 이름 목록을 반환합니다.                        |



### ✅ 수치 안정성 및 경계 조건
- ON_TOL6를 기준으로 시간 비교 → 부동소수점 오차 대응
- unwrap_or(0) 처리 → 슬라이싱 실패 시 안전한 기본값 제공
- sample_interval 계산 시 n > 1 조건 → 단일 샘플 예외 처리
    - 전체적으로 수치적 안정성과 예외 처리가 잘 되어 있습니다.


```rust
use crate::core::key_value_pool::KeyValuePool;
use crate::core::tarray::TArray;
use crate::core::types::ON_TOL6;
use crate::injury::conv_filter::{ConvFilter, FilterEngine};

const TIME_INTERVAL: f64 = 0.0001;
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

        let idx_start =
            on_find_index_within_eps(self.data_x.as_slice(), start, ON_TOL6).unwrap_or(0);
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
        self.calc_min_max().map(|(mn, mx)| (mn, mx))
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
```
```rust
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
```rust

use crate::core::tarray::TArray;
use std::collections::HashMap;

pub struct ChannelContainer {
    items: HashMap<String, TArray<f64>>,
}
```
```rust
impl ChannelContainer {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, data: TArray<f64>) {
        self.items.insert(name, data);
    }

    pub fn get(&self, name: &str) -> Option<&TArray<f64>> {
        self.items.get(name)
    }

    pub fn get_many(&self, names: &[&str]) -> Vec<&TArray<f64>> {
        names.iter().filter_map(|&n| self.get(n)).collect()
    }

    pub fn apply_to_all_channels<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut TArray<f64>),
    {
        for data in self.items.values_mut() {
            func(data);
        }
    }

    pub fn compute_injury_metric<F>(&self, names: &[&str], func: F) -> Option<Vec<f64>>
    where
        F: Fn(&[&TArray<f64>]) -> Vec<f64>,
    {
        let arrays = self.get_many(names);
        if arrays.len() != names.len() {
            None
        } else {
            Some(func(&arrays))
        }
    }

    pub fn get_channel_names(&self) -> Vec<&String> {
        self.items.keys().collect()
    }
}
```
```rust
pub struct CsvChannelLoader {
    headers: Vec<String>,
    data: HashMap<String, Vec<f64>>,
}
```
```rust
impl CsvChannelLoader {
    pub fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut rdr = Reader::from_path(path)?;
        let headers = rdr
            .headers()?
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut data: HashMap<String, Vec<f64>> =
            headers.iter().map(|h| (h.clone(), Vec::new())).collect();
        for record in rdr.records() {
            let record = record?;
            for (i, field) in record.iter().enumerate() {
                if let Ok(val) = field.trim().parse::<f64>() {



                    data.get_mut(&headers[i]).unwrap().push(val);
                }
            }
        }

        Ok(Self { headers, data })
    }

    pub fn header_count(&self) -> usize {
        self.headers.len()
    }

    pub fn get_header_by_index(&self, index: usize) -> Option<&String> {
        self.headers.get(index)
    }

    pub fn get_column(&self, header: &str) -> Option<&Vec<f64>> {
        self.data.get(header)
    }
}
```
```rust
#[allow(unused)]
fn read_chanel_file(
    path: String,
    scale: f64,
    offset: f64,
    data_start: usize,
    data_end: usize,
) -> Result<ChannelContainer, Box<dyn Error>> {
    let loader = CsvChannelLoader::from_path(path.as_str())?;
    let mut container = ChannelContainer::new();
    let header_count = loader.header_count();
    for header in (0..header_count)
        .filter_map(|i| loader.get_header_by_index(i))
        .filter(|h| !h.is_empty())
    {
        if let Some(raw) = loader.get_column(header) {
            let src = TArray::from(raw.clone());
            let mut tgt = TArray::from(vec![]);

            //Channel Operation needed
            //exec_sae_filter(&src, &mut tgt, 0.0001, 300.0);

            container.insert(header.clone(), tgt);
        }
    }

    // 🔧 전처리: 모든 채널에 스케일과 오프셋 적용 + 구간 슬라이스
    container.apply_to_all_channels(|data| {
        data.scale(scale); // 단위 변환
        data.offset(offset); // 센서 기준점 보정
        if data_start != 0 || data_end != 0 {
            data.slice(data_start, data_end); // 시간 구간 추출
        }
    });
    Ok(container)
}
```
```rust
fn apply_to_all_channels(chn_container: &mut ChannelContainer, scale: f64, offset: f64, start_index: usize, end_index: usize) {
    chn_container.apply_to_all_channels(|data| {
        data.scale(scale); // 단위 변환
        data.offset(offset); // 센서 기준점 보정
        data.slice(start_index, end_index); // 시간 구간 추출
    });
}
```
```rust
fn change_channel_data(chn_source: &TArray<f64>, chn_target: &mut TArray<f64>) {

    chn_target.data = chn_source.data.clone();

    //exec_sae_filter(&src, &mut tgt, 0.0001, 300.0);

}
```
```rust
pub fn on_read_channel_file(path: String) -> Result<ChannelContainer, Box<dyn Error>> {
    let loader = CsvChannelLoader::from_path(path.as_str())?;
    let mut container = ChannelContainer::new();

    let header_count = loader.header_count();
    for header in (0..header_count)
        .filter_map(|i| loader.get_header_by_index(i))
        .filter(|h| !h.is_empty())
    {
        if let Some(raw) = loader.get_column(header) {
            let src = TArray::from(raw.clone());
            let mut tgt = TArray::from(vec![]);
            change_channel_data(&src,&mut tgt);
            container.insert(header.clone(), tgt);
        }
    }

    //apply_to_all_channels(&mut container, 9.81, -0.5, 0, 100);

    Ok(container)
}
```


## 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::tarray::TArray;
    use nurbslib::injury::channel::Channel;
    use nurbslib::injury::conv_filter::ConvFilter;
    use nurbslib::injury::csv_channel_loader::on_read_channel_file;

    #[test]
    fn csv_reader_test() {
        match on_read_channel_file("asset/injury.csv".to_string()) {
            Ok(chn_container) => {
                let channel_names = chn_container.get_channel_names();
                for channel_name in channel_names {
                    println!("Channel name: {}", channel_name);

                    if let Some(data) = chn_container.get(channel_name) {

                        println!("  - size : {:?}", data.len());

                        if channel_name == "11FEMRRI00H3FOZB"
                        {
                            println!("{:}", data);
                        }

                    }
                }
            }
            Err(e) => {
                eprintln!("파일 읽기 실패: {}", e);
            }
        }
    }
```
```rust
    #[test]
    fn test_channel() {
        // 1. 채널 생성
        let mut ch = Channel::new();
        ch.set_channel_name("Accel Z");
        ch.set_channel_desc("Z축 가속도");

        // 2. 시간 및 데이터 설정
        let x = (0..100).map(|i| i as f64 * 0.001).collect::<Vec<_>>();
        let y = x.iter().map(|t| (2.0 * std::f64::consts::PI * t).sin()).collect::<Vec<_>>();

        ch.set_data_vec_x(x);
        ch.set_data_vec_y(y);

        // 3. 메타데이터 갱신
        ch.update_props_all();



        // 4. 필터 적용 (예: 이동 평균 필터)
        let mut target_y: TArray<f64> = TArray::new();
        ConvFilter::exec_sae_filter_600(ch.data_y(), &mut target_y,  0.0001);

        println!("필터 적용 성공 여부: {}", target_y.len());
        ch.set_data_y(target_y);
        println!("최대/최소: {:?}", ch.calc_min_max_value());
    }
```
```rust
    #[test]
    fn test_channel_min_max() {
        let mut ch = Channel::new();
        ch.set_channel_name("Test");
        ch.set_channel_desc("테스트 채널");

        let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y = vec![10.0, -5.0, 3.0, 8.0, -2.0];

        ch.set_data_vec_x(x);
        ch.set_data_vec_y(y);

        let (min, max) = ch.calc_min_max_value().unwrap();
        assert_eq!(min, -5.0);
        assert_eq!(max, 10.0);
    }
```
```rust
    #[test]
    fn test_channel_filter_application() {
        let mut ch = Channel::new();
        let x = (0..10).map(|i| i as f64).collect::<Vec<_>>();
        let y = vec![1.0; 10]; // 상수 신호

        ch.set_data_vec_x(x);
        ch.set_data_vec_y(y);

        let (min, max) = ch.calc_min_max_value().unwrap();
        assert_eq!(min, 1.0);
        assert_eq!(max, 1.0);
    }
}
```
---

 구조체 및 역할 요약
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
| `find_closest_slice(z_mm)`       | ✅ 있음         | $\min |z_i - z_{\text{target}}|$ — Z 위치 거리 최소화                         |
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
| `gen_volume_rendering`        | `set_slices`, `render_mip`, `render_xray`, `interpolated_slice` | ✅ 있음         | MIP: $I(x,y) = \max_k I_k(x,y)$ <br> Xray: $I(x,y) = \frac{1}{N} \sum_k I_k(x,y)$ <br> 보간: $I = (1 - t) I_0 + t I_1$, $t = \frac{z - z_0}{z_1 - z_0}$ |
| `test_extract_and_render_mip` | `set_slices`, `render_mip` | ✅ 있음         | $I(x,y) = \max_k I_k(x,y)$                                |
| `test_set_slices_and_ordering`| `set_slices`               | ✅ 있음        | 슬라이스 정렬만 수행                                          |
| `test_extract_slice`          | `extract_slice`            | ✅ 내부 거리 계산 | $\min |z_i - z_{\text{target}}|$                           |
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
| 슬라이스 거리 비교 (`find_closest_slice`) | $\min |z_i - z_{\text{target}}|$                              |
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
