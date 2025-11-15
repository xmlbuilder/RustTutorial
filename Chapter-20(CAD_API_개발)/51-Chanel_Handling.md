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
