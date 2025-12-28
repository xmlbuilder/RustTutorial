# 📘 Rust DataFrame Library – 함수 설명 & 사용법
- 이 문서는 DataFrame/Series 라이브러리의 전체 기능을 체계적으로 정리한 공식 문서 스타일 가이드다.

## 🧱 기본 구조
- Series
  - 1D labeled array
  - 구성 요소:
  - name: String
  - index: Vec<String>
  - data: ND (shape = [n])
- DataFrame
  - 2D labeled table
  - 구성 요소:
  - index: Vec<String>
  - columns: Vec<String>
  - data: ND (shape = [nrows, ncols])

### 📌 DataFrame 생성
- new(index, columns, data_2d)
```rust
let df = DataFrame::new(
    vec!["r1".into(), "r2".into()],
    vec!["A".into(), "B".into()],
    ND::from_vec2(vec![vec![1.,2.], vec![3.,4.]])
);
```

### 📐 기본 정보
- shape() → (rows, cols)
```rust
let (n, m) = df.shape();
```

### 📎 컬럼 선택
- col(name) → Series
```rust
let s = df.col("A");

cols(&["A","B"]) → DataFrame
let sub = df.cols(&["A", "B"]);
```
### 🔍 iloc – 위치 기반 인덱싱
- df.iloc().rows(&[0,2,4])
- df.iloc().row_range(0..3)
- df.iloc().cols(&[1,3])
- df.iloc().col_range(1..4)
- df.iloc().at(2, 3)


- 예:
```rust
let row = df.iloc().rows(&[0]);
let value = df.iloc().at(1, 2);
```

- 🔍 loc – 라벨 기반 인덱싱
```rust
df.loc().rows(&["row1", "row3"])
```

- 🧪 Series 연산
- gt(v) → Vec<bool>
```rust
let mask = df.col("A").gt(10.0);
```

- apply(f) → Series
```rust
let s2 = df.col("A").apply(|x| x * 2.0, "A2");
```

## 🧹 DataFrame 필터링
- filter_rows(mask)
```rust
let mask = df.col("A").gt(10.0);
let filtered = df.filter_rows(&mask);
```

## ➕ 컬럼 추가/교체
- with_column(name, &Series)
```rust
let s = df.col("A").apply(|x| x * 2.0, "A2");
let df2 = df.with_column("A2", &s);
```

## 🧽 결측치 처리
- fillna(v)
```rust
let df2 = df.fillna(0.0);
```
- dropna() – NaN 포함된 row 제거
```rust
let df2 = df.dropna();
```

## 🗑 컬럼 제거
- drop_cols(&["A","C"])
```rust
let df2 = df.drop_cols(&["A"]);
```

## 📊 정렬
- sort_values(by, ascending)
```rust
let df2 = df.sort_values("A", true);
```


## 🧩 GroupBy
- groupby_i64(col)
```rust
let g = df.groupby_i64("part_id");
```

- size()
```rust
let size_df = g.size();
```

- mean() – 그룹별 평균
```rust
let mean_df = g.mean();
```

## 🧾 head / tail
- df.head(5)
- df.tail(5)


## 🔗 DataFrame 결합
- concat_rows(a, b)
```rust
let df3 = DataFrame::concat_rows(&df1, &df2);
```

- concat_cols(a, b)
```rust
let df3 = DataFrame::concat_cols(&df1, &df2);
```


## 📄 CSV 입출력
- read_csv(path, options)
```rust
let opt = CsvOptions::default();
let df = DataFrame::read_csv("data.csv", opt)?;
```

- to_csv(path, options)
```rust
df.to_csv("out.csv", CsvOptions::default())?;
```


## 📈 describe() – 통계 요약
- count / mean / std / min / max
```rust
let stats = df.describe();
println!("{}", stats);
```


## 🔢 value_counts_i64(col)
```rust
let vc = df.value_counts_i64("part_id");
```


## 🔁 drop_duplicates(subset)
```rust
let df2 = df.drop_duplicates(Some(&["node_id"]));
```

## 🧰 유틸 함수
- row_vec(a, r)
- col_vec(a, c)
- v1_vec(v)
  - NDArray → Vec 변환용.

## 🏭 fem_pipeline() – 예제 파이프라인
- CSV 로드
  - head 출력
  - describe
  - groupby mean
  - stress > 200 필터
  - node_id 중복 제거
  - CSV 저장
  - fem_pipeline("input.csv", "result")?;

## 🎉 마무리
- 이 문서 하나면 DataFrame 라이브러리를 처음 보는 사람도 바로 사용할 수 있는 수준.
- ND 엔진 + DataFrame 엔진으로, 이제 Rust에서 진짜 Pandas 스타일 분석이 가능.

---

## 소스 코드
```rust
use std::collections::{BTreeMap, HashSet};
use std::{fmt, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use ndarray::{s, Array2, ArrayView1, ArrayView2, Ix1};
use crate::core::nd::ND;
```
```rust
pub type Index = Vec<String>;
pub type Columns = Vec<String>;
```
```rust

pub struct Series {
    pub name: String,
    pub index: Index,
    pub data: ND,   // shape [n]
}
```
```rust
pub struct DataFrame {
    pub index: Index,
    pub columns: Columns,
    pub data: ND,   // shape [nrows, ncols]
}
```
```rust
impl DataFrame {
    pub fn new(index: Index, columns: Columns, data_2d: ND) -> Self {
        let a = data_2d.as_2d();
        assert_eq!(index.len(), a.nrows(), "DataFrame::new: index len mismatch");
        assert_eq!(columns.len(), a.ncols(), "DataFrame::new: columns len mismatch");
        Self { index, columns, data: data_2d }
    }
```
```rust
    pub fn shape(&self) -> (usize, usize) {
        let a = self.data.as_2d();
        (a.nrows(), a.ncols())
    }
```
```rust
    pub fn col(&self, name: &str) -> Series {
        let j = self
            .columns
            .iter()
            .position(|c| c == name)
            .unwrap_or_else(|| panic!("col: no such column '{name}'"));
        Series {
            name: name.to_string(),
            index: self.index.clone(),
            data: self.data.col2(j),
        }
    }
```
```rust
    pub fn cols(&self, names: &[&str]) -> DataFrame {
        let idx: Vec<usize> = names
            .iter()
            .map(|n| self.columns.iter().position(|c| c == *n)
                .unwrap_or_else(|| panic!("cols: no such column '{n}'")))
            .collect();
        let new_cols: Columns = names.iter().map(|s| s.to_string()).collect();
        DataFrame::new(
            self.index.clone(),
            new_cols,
            self.data.take(1, &idx),
        )
    }
}
```
```rust
pub struct Iloc<'a>(&'a DataFrame);
```
```rust
impl DataFrame {
    pub fn iloc(&self) -> Iloc<'_> { Iloc(self) }
}
```
```rust
impl<'a> Iloc<'a> {
    pub fn rows(&self, idx: &[usize]) -> DataFrame {
        let df = self.0;
        let new_index: Index = idx.iter().map(|&i| df.index[i].clone()).collect();
        DataFrame::new(new_index, df.columns.clone(), df.data.take(0, idx))
    }
```
```rust
    pub fn row_range(&self, r: std::ops::Range<usize>) -> DataFrame {
        let df = self.0;
        let new_index: Index = df.index[r.clone()].to_vec();
        DataFrame::new(new_index, df.columns.clone(), df.data.slice_axis(0, r))
    }
```
```rust
    pub fn cols(&self, idx: &[usize]) -> DataFrame {
        let df = self.0;
        let new_cols: Columns = idx.iter().map(|&j| df.columns[j].clone()).collect();
        DataFrame::new(df.index.clone(), new_cols, df.data.take(1, idx))
    }
```
```rust
    pub fn col_range(&self, r: std::ops::Range<usize>) -> DataFrame {
        let df = self.0;
        let new_cols: Columns = df.columns[r.clone()].to_vec();
        DataFrame::new(df.index.clone(), new_cols, df.data.slice_axis(1, r))
    }
```
```rust
    pub fn at(&self, r: usize, c: usize) -> f64 {
        self.0.data.at2(r, c)
    }
}
```
```rust
pub struct Loc<'a>(&'a DataFrame);
```
```rust
impl DataFrame {
    pub fn loc(&self) -> Loc<'_> { Loc(self) }
}
```
```rust
impl<'a> Loc<'a> {
    pub fn rows(&self, labels: &[&str]) -> DataFrame {
        let df = self.0;
        let idx: Vec<usize> = labels.iter().map(|lab| {
            df.index.iter().position(|x| x == *lab)
                .unwrap_or_else(|| panic!("loc.rows: no such label '{lab}'"))
        }).collect();

        df.iloc().rows(&idx)
    }
}
```
```rust
impl Series {
    pub fn gt(&self, v: f64) -> Vec<bool> {
        self.data.data().iter().map(|&x| x > v).collect()
    }
}
```
```rust
impl DataFrame {
    pub fn filter_rows(&self, mask: &[bool]) -> DataFrame {
        assert_eq!(mask.len(), self.index.len(), "filter_rows: len mismatch");
        let new_index: Index = self.index.iter().zip(mask.iter())
            .filter_map(|(lab, &b)| b.then_some(lab.clone()))
            .collect();
        DataFrame::new(new_index, self.columns.clone(), self.data.mask_rows(mask))
    }
}
```
```rust
impl DataFrame {
    pub fn with_column(&self, name: &str, s: &Series) -> DataFrame {
        assert_eq!(self.index.len(), s.index.len(), "with_column: index len mismatch");
        // (A안) index 라벨까지 동일하다고 가정하거나 assert
        assert_eq!(self.index, s.index, "with_column: index labels mismatch");

        let (n, m) = self.shape();
        let mut cols = self.columns.clone();
        let mut data = self.data.as_2d().to_owned(); // Array2<f64>

        if let Some(j) = cols.iter().position(|c| c == name) {
            // replace
            let v = s.data.data().view().into_dimensionality::<Ix1>().unwrap();
            for i in 0..n { data[(i, j)] = v[i]; }
        } else {
            // append
            cols.push(name.to_string());
            let mut new = Array2::<f64>::zeros((n, m + 1));
            new.slice_mut(s![.., 0..m]).assign(&data);
            let v = s.data.data().view().into_dimensionality::<Ix1>().unwrap();
            for i in 0..n { new[(i, m)] = v[i]; }
            data = new;
        }
        DataFrame::new(self.index.clone(), cols, ND::from_array(data.into_dyn()))
    }
}
```
```rust
impl Series {
    pub fn apply<F>(&self, f: F, new_name: &str) -> Series
    where F: Fn(f64) -> f64
    {
        let v = self.data.data().view().into_dimensionality::<Ix1>().unwrap();
        let out: Vec<f64> = v.iter().map(|&x| f(x)).collect();
        Series { name: new_name.to_string(), index: self.index.clone(), data: ND::from_vec(out) }
    }
}
```
```rust
impl DataFrame {
    pub fn fillna(&self, v: f64) -> DataFrame {
        DataFrame::new(self.index.clone(), self.columns.clone(), self.data.fill_nan(v))
    }
```
```rust
    /// row-wise dropna: row에 NaN이 하나라도 있으면 제거
    pub fn dropna(&self) -> DataFrame {
        let a = self.data.as_2d();
        let mut keep = Vec::with_capacity(a.nrows());
        for i in 0..a.nrows() {
            let mut ok = true;
            for j in 0..a.ncols() {
                if a[(i,j)].is_nan() { ok = false; break; }
            }
            keep.push(ok);
        }
        self.filter_rows(&keep)
    }
```
```rust
    pub fn drop_cols(&self, names: &[&str]) -> DataFrame {
        let drop: std::collections::HashSet<&str> = names.iter().copied().collect();
        let keep_idx: Vec<usize> = self.columns.iter().enumerate()
            .filter_map(|(j,c)| (!drop.contains(c.as_str())).then_some(j))
            .collect();
        let new_cols: Columns = keep_idx.iter().map(|&j| self.columns[j].clone()).collect();
        DataFrame::new(self.index.clone(), new_cols, self.data.take(1, &keep_idx))
    }
}
```
```rust
impl DataFrame {
    pub fn sort_values(&self, by: &str, ascending: bool) -> DataFrame {
        let s = self.col(by);
        let mut perm = s.data.argsort1();
        if !ascending { perm.reverse(); }

        let new_index: Index = perm.iter().map(|&i| self.index[i].clone()).collect();
        DataFrame::new(new_index, self.columns.clone(), self.data.permute_rows2(&perm))
    }
}
```
```rust
pub struct GroupBy<'a> {
    df: &'a DataFrame,
    key_col_idx: usize,
    groups: BTreeMap<i64, Vec<usize>>,
}
```
```rust
impl DataFrame {
    pub fn groupby_i64(&self, key_col: &str) -> GroupBy<'_> {
        let key_col_idx = self.columns.iter().position(|c| c == key_col)
            .unwrap_or_else(|| panic!("groupby_i64: no such column '{key_col}'"));

        let a = self.data.as_2d();
        let keyv = a.column(key_col_idx);

        let mut groups: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (i, &x) in keyv.iter().enumerate() {
            if x.is_nan() { continue; }
            let k = x as i64;
            groups.entry(k).or_default().push(i);
        }

        GroupBy { df: self, key_col_idx, groups }
    }
}
```
```rust
impl<'a> GroupBy<'a> {
    pub fn size(&self) -> DataFrame {
        let mut index: Vec<String> = Vec::with_capacity(self.groups.len());
        let mut keys: Vec<f64> = Vec::with_capacity(self.groups.len());
        let mut sizes: Vec<f64> = Vec::with_capacity(self.groups.len());

        for (k, rows) in self.groups.iter() {
            index.push(k.to_string());
            keys.push(*k as f64);
            sizes.push(rows.len() as f64);
        }

        let mut flat = Vec::with_capacity(index.len() * 2);
        for i in 0..index.len() {
            flat.push(keys[i]);
            flat.push(sizes[i]);
        }

        let arr = ndarray::Array2::from_shape_vec((index.len(), 2), flat).unwrap();
        DataFrame::new(index, vec!["key".into(), "size".into()], ND::from_array(arr.into_dyn()))
    }
```
```rust
    /// mean of each column per group (skip NaN). Includes key_col too (원하면 나중에 제외 옵션 추가 가능)
    pub fn mean(&self) -> DataFrame {
        let df = self.df;
        let a = df.data.as_2d();
        let ncols = a.ncols();

        let mut out_index: Vec<String> = Vec::with_capacity(self.groups.len());
        let mut out_flat: Vec<f64> = Vec::with_capacity(self.groups.len() * ncols);

        for (k, rows) in self.groups.iter() {
            out_index.push(k.to_string());

            for j in 0..ncols {
                let mut sum = 0.0;
                let mut cnt = 0.0;
                for &i in rows.iter() {
                    let x = a[(i, j)];
                    if x.is_nan() { continue; }
                    sum += x;
                    cnt += 1.0;
                }
                out_flat.push(if cnt > 0.0 { sum / cnt } else { f64::NAN });
            }
        }

        let out = ndarray::Array2::from_shape_vec((out_index.len(), ncols), out_flat).unwrap();
        DataFrame::new(out_index, df.columns.clone(), ND::from_array(out.into_dyn()))
    }
}
```
```rust
impl DataFrame {
    pub fn head(&self, n: usize) -> DataFrame {
        let k = n.min(self.index.len());
        self.iloc().row_range(0..k)
    }
```
```rust
    pub fn tail(&self, n: usize) -> DataFrame {
        let len = self.index.len();
        let k = n.min(len);
        self.iloc().row_range(len - k..len)
    }
```
```rust
    pub fn concat_rows(a: &DataFrame, b: &DataFrame) -> DataFrame {
        assert_eq!(a.columns, b.columns, "concat_rows: column mismatch");

        let mut index = a.index.clone();
        index.extend(b.index.iter().cloned());

        let a2 = a.data.as_2d();
        let b2 = b.data.as_2d();

        let mut out = ndarray::Array2::<f64>::zeros((a2.nrows() + b2.nrows(), a2.ncols()));
        out.slice_mut(ndarray::s![0..a2.nrows(), ..]).assign(&a2);
        out.slice_mut(ndarray::s![a2.nrows().., ..]).assign(&b2);

        DataFrame::new(index, a.columns.clone(), ND::from_array(out.into_dyn()))
    }
```
```rust
    /// axis=1: col concat (index must match)
    pub fn concat_cols(a: &DataFrame, b: &DataFrame) -> DataFrame {
        assert_eq!(a.index, b.index, "concat_cols: index mismatch");

        let a2 = a.data.as_2d();
        let b2 = b.data.as_2d();
        assert_eq!(a2.nrows(), b2.nrows(), "concat_cols: nrows mismatch");

        let mut out = Array2::<f64>::zeros((a2.nrows(), a2.ncols() + b2.ncols()));
        out.slice_mut(s![.., 0..a2.ncols()]).assign(&a2);
        out.slice_mut(s![.., a2.ncols()..]).assign(&b2);

        let mut cols = a.columns.clone();
        cols.extend(b.columns.iter().cloned());

        DataFrame::new(a.index.clone(), cols, ND::from_array(out.into_dyn()))
    }
}
```
```rust
impl DataFrame {
    /// 보기 좋게 출력하기 위한 문자열(기본 head_rows=10)
    pub fn to_pretty_string(&self, head_rows: usize) -> String {
        let (n, m) = self.shape();
        let show_n = head_rows.min(n);

        // column widths
        let mut w_idx = "index".len();
        for i in 0..show_n {
            w_idx = w_idx.max(self.index[i].len());
        }

        let mut w_col: Vec<usize> = self.columns.iter().map(|c| c.len().max(6)).collect();
        let a = self.data.as_2d();

        for j in 0..m {
            for i in 0..show_n {
                let s = format!("{}", a[(i, j)]);
                w_col[j] = w_col[j].max(s.len());
            }
        }

        let mut out = String::new();

        // header
        out.push_str(&format!("{:width$}  ", "index", width = w_idx));
        for j in 0..m {
            out.push_str(&format!("{:>width$}  ", self.columns[j], width = w_col[j]));
        }
        out.push('\n');

        // rows
        for i in 0..show_n {
            out.push_str(&format!("{:width$}  ", self.index[i], width = w_idx));
            for j in 0..m {
                out.push_str(&format!("{:>width$}  ", a[(i, j)], width = w_col[j]));
            }
            out.push('\n');
        }

        if show_n < n {
            out.push_str(&format!("... ({} more rows)\n", n - show_n));
        }

        out
    }
}
```
```rust
impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 기본 10줄
        write!(f, "{}", self.to_pretty_string(10))
    }
}
```
```rust
impl DataFrame {
    /// describe(): count/mean/std/min/max (skip NaN), ddof=0
    pub fn describe(&self) -> DataFrame {
        let a = self.data.as_2d();
        let ncols = a.ncols();

        let stat_rows = vec![
            "count".to_string(),
            "mean".to_string(),
            "std".to_string(),
            "min".to_string(),
            "max".to_string(),
        ];

        // shape: [5, ncols]
        let mut out = Array2::<f64>::from_elem((stat_rows.len(), ncols), f64::NAN);

        for j in 0..ncols {
            let col = a.column(j);

            let mut n = 0.0;
            let mut sum = 0.0;
            let mut minv = f64::INFINITY;
            let mut maxv = f64::NEG_INFINITY;

            for &x in col.iter() {
                if x.is_nan() { continue; }
                n += 1.0;
                sum += x;
                if x < minv { minv = x; }
                if x > maxv { maxv = x; }
            }

            // count
            out[(0, j)] = n;

            if n <= 0.0 {
                // mean/std/min/max는 NaN 유지
                continue;
            }

            let mean = sum / n;

            // population std (ddof=0)
            let mut var = 0.0;
            for &x in col.iter() {
                if x.is_nan() { continue; }
                let d = x - mean;
                var += d * d;
            }
            var /= n;
            let std = var.sqrt();

            out[(1, j)] = mean;
            out[(2, j)] = std;
            out[(3, j)] = minv;
            out[(4, j)] = maxv;
        }

        DataFrame::new(stat_rows, self.columns.clone(), ND::from_array(out.into_dyn()))
    }
}
```
```rust
#[derive(Clone, Debug)]
pub struct CsvOptions {
    pub delimiter: char,        // default ','
    pub has_header: bool,       // default true
    pub has_index: bool,        // default true (첫 컬럼이 index)
    pub trim: bool,             // default true
    pub comment_prefix: Option<char>, // 예: Some('#')면 #로 시작하는 줄 무시
}
```
```rust
impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            delimiter: ',',
            has_header: true,
            has_index: true,
            trim: true,
            comment_prefix: None,
        }
    }
}
```
```rust
fn split_csv_line(line: &str, delim: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;

    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                // "" -> literal quote
                if in_quotes && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            c if c == delim && !in_quotes => {
                out.push(cur);
                cur = String::new();
            }
            _ => cur.push(ch),
        }
    }
    out.push(cur);
    out
}
```
```rust
impl DataFrame {
    pub fn read_csv(path: &str, opt: CsvOptions) -> io::Result<DataFrame> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let delim = opt.delimiter;

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            let mut s = line?;
            if opt.trim { s = s.trim().to_string(); }
            if s.is_empty() { continue; }
            if let Some(c) = opt.comment_prefix {
                if s.starts_with(c) { continue; }
            }
            lines.push(s);
        }

        if lines.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "read_csv: empty file"));
        }

        // header 처리
        let mut pos = 0usize;
        let mut columns: Vec<String> = Vec::new();

        if opt.has_header {
            let header = split_csv_line(&lines[0], delim);
            pos = 1;

            if opt.has_index {
                if header.len() < 2 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "read_csv: header too short"));
                }
                columns = header[1..].iter().map(|s| if opt.trim { s.trim().to_string() } else { s.clone() }).collect();
            } else {
                columns = header.iter().map(|s| if opt.trim { s.trim().to_string() } else { s.clone() }).collect();
            }
        }

        // 데이터 rows 파싱
        let mut index: Vec<String> = Vec::new();
        let mut rows_values: Vec<Vec<f64>> = Vec::new();

        for (i, line) in lines[pos..].iter().enumerate() {
            let fields0 = split_csv_line(line, delim);
            let fields: Vec<String> = if opt.trim {
                fields0.into_iter().map(|x| x.trim().to_string()).collect()
            } else {
                fields0
            };

            if fields.is_empty() { continue; }

            let (idx_label, data_fields): (String, &[String]) = if opt.has_index {
                let idx = fields.get(0).cloned().unwrap_or_else(|| (pos + i).to_string());
                let rest = if fields.len() >= 2 { &fields[1..] } else { &[] };
                (idx, rest)
            } else {
                ((pos + i).to_string(), fields.as_slice())
            };

            // header가 없으면 첫 데이터 줄에서 columns 길이 결정
            if !opt.has_header && columns.is_empty() {
                columns = (0..data_fields.len()).map(|k| format!("c{k}")).collect();
            }

            // data_fields 길이가 columns보다 짧으면 NaN padding, 길면 잘라냄(A안)
            let m = columns.len();
            if m == 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "read_csv: no columns"));
            }

            let mut row: Vec<f64> = Vec::with_capacity(m);
            for j in 0..m {
                let s = data_fields.get(j).map(|x| x.as_str()).unwrap_or("");
                row.push(parse_f64_or_nan(s));
            }

            index.push(idx_label);
            rows_values.push(row);
        }

        if rows_values.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "read_csv: no data rows"));
        }

        let n = rows_values.len();
        let m = columns.len();
        let mut flat: Vec<f64> = Vec::with_capacity(n * m);
        for r in rows_values {
            debug_assert_eq!(r.len(), m);
            flat.extend_from_slice(&r);
        }

        let arr = Array2::from_shape_vec((n, m), flat)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "read_csv: shape error"))?;

        Ok(DataFrame::new(index, columns, ND::from_array(arr.into_dyn())))
    }

    pub fn to_csv(&self, path: &str, opt: CsvOptions) -> io::Result<()> {
        let mut f = File::create(path)?;
        let delim = opt.delimiter;

        // header
        if opt.has_header {
            if opt.has_index {
                write!(f, "index")?;
                for c in &self.columns {
                    write!(f, "{delim}{}", escape_csv_field(c, delim))?;
                }
                writeln!(f)?;
            } else {
                for (j, c) in self.columns.iter().enumerate() {
                    if j > 0 { write!(f, "{delim}")?; }
                    write!(f, "{}", escape_csv_field(c, delim))?;
                }
                writeln!(f)?;
            }
        }

        let a = self.data.as_2d();
        let (n, m) = (a.nrows(), a.ncols());

        for i in 0..n {
            if opt.has_index {
                write!(f, "{}", escape_csv_field(&self.index[i], delim))?;
            }
            for j in 0..m {
                if opt.has_index || j > 0 {
                    write!(f, "{delim}")?;
                }
                // NaN은 "NaN"으로 저장
                let x = a[(i, j)];
                if x.is_nan() {
                    write!(f, "NaN")?;
                } else {
                    // 기본 포맷(필요하면 나중에 옵션으로 precision 추가 가능)
                    write!(f, "{}", x)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
```
```rust
fn parse_f64_or_nan(s: &str) -> f64 {
    let t = s.trim();
    if t.is_empty() { return f64::NAN; }
    let tl = t.to_ascii_lowercase();
    if tl == "nan" || tl == "null" || tl == "none" { return f64::NAN; }
    t.parse::<f64>().unwrap_or(f64::NAN)
}
```
```rust
fn escape_csv_field(s: &str, delim: char) -> String {
    // delim/quote/newline이 있으면 quote로 감싸고 내부 quote는 ""로 escape
    let need_quote = s.contains(delim) || s.contains('"') || s.contains('\n') || s.contains('\r');
    if !need_quote { return s.to_string(); }
    let mut out = String::new();
    out.push('"');
    for ch in s.chars() {
        if ch == '"' { out.push('"'); out.push('"'); }
        else { out.push(ch); }
    }
    out.push('"');
    out
}
```
```rust
impl DataFrame {
    /// value_counts for numeric column interpreted as i64.
    /// Returns DataFrame with columns: ["key","count"] and index = key string.
    pub fn value_counts_i64(&self, col: &str) -> DataFrame {
        let j = self.columns.iter().position(|c| c == col)
            .unwrap_or_else(|| panic!("value_counts_i64: no such column '{col}'"));

        let a = self.data.as_2d();
        let v = a.column(j);

        let mut map: BTreeMap<i64, usize> = BTreeMap::new();
        for &x in v.iter() {
            if x.is_nan() { continue; }
            let k = x as i64;
            *map.entry(k).or_insert(0) += 1;
        }

        let mut index: Vec<String> = Vec::with_capacity(map.len());
        let mut flat: Vec<f64> = Vec::with_capacity(map.len() * 2);

        for (k, cnt) in map.iter() {
            index.push(k.to_string());
            flat.push(*k as f64);
            flat.push(*cnt as f64);
        }

        let arr = ndarray::Array2::from_shape_vec((index.len(), 2), flat).unwrap();
        DataFrame::new(index, vec!["key".into(), "count".into()], ND::from_array(arr.into_dyn()))
    }
}
```
```rust
impl DataFrame {
    /// Drop duplicate rows. If subset is Some(["A","B"]) then only those columns are considered.
    /// keep="first" 정책만 지원 (첫 등장 유지)
    pub fn drop_duplicates(&self, subset: Option<&[&str]>) -> DataFrame {
        let a = self.data.as_2d();
        let (n, m) = (a.nrows(), a.ncols());

        let cols_idx: Vec<usize> = match subset {
            None => (0..m).collect(),
            Some(names) => names.iter().map(|name| {
                self.columns.iter().position(|c| c == *name)
                    .unwrap_or_else(|| panic!("drop_duplicates: no such column '{name}'"))
            }).collect(),
        };

        fn norm_bits(x: f64) -> u64 {
            if x.is_nan() { 0x7ff8_0000_0000_0000u64 } else { x.to_bits() }
        }

        let mut seen: HashSet<Vec<u64>> = HashSet::new();
        let mut keep_rows: Vec<usize> = Vec::new();

        for i in 0..n {
            let mut key: Vec<u64> = Vec::with_capacity(cols_idx.len());
            for &j in &cols_idx {
                key.push(norm_bits(a[(i, j)]));
            }
            if seen.insert(key) {
                keep_rows.push(i);
            }
        }

        self.iloc().rows(&keep_rows)
    }
}
```
```rust
pub fn row_vec(a: &ArrayView2<f64>, r: usize) -> Vec<f64> {
    a.row(r).iter().copied().collect()
}
```
```rust
pub fn col_vec(a: &ArrayView2<f64>, c: usize) -> Vec<f64> {
    a.column(c).iter().copied().collect()
}
```
```rust
pub fn v1_vec(v: ArrayView1<f64>) -> Vec<f64> {
    v.iter().copied().collect()
}
```
```rust
pub fn fem_pipeline(input_csv: &str, out_prefix: &str) -> std::io::Result<()> {
    let opt = CsvOptions::default();

    // 1) load
    let df = DataFrame::read_csv(input_csv, opt.clone())?;

    // 2) quick overview
    println!("df shape = {:?}", df.shape());
    println!("{}", df.head(5));
    println!("describe(stress):\n{}", df.cols(&["stress"]).describe());

    // 3) group by part_id and mean
    let gmean = df.groupby_i64("part_id").mean();
    gmean.to_csv(&format!("{out_prefix}_part_mean.csv"), opt.clone())?;

    // 4) outlier filtering: stress > 200
    let stress = df.col("stress");
    let mask: Vec<bool> = stress.data.iter().map(|&x| !x.is_nan() && x > 200.0).collect();
    let high = df.filter_rows(&mask);
    high.to_csv(&format!("{out_prefix}_stress_gt_200.csv"), opt.clone())?;

    // 5) duplicates 제거 (node_id 중복 제거)
    let uniq_nodes = df.drop_duplicates(Some(&["node_id"]));
    uniq_nodes.to_csv(&format!("{out_prefix}_uniq_nodes.csv"), opt)?;

    Ok(())
}
```
---

### 테스트 코드
```rust
use nurbslib::core::nd::ND;
use nurbslib::core::pandas::{row_vec, DataFrame}; // 네 경로에 맞춰 수정
```
```rust
fn df_small() -> DataFrame {
    // index: r0,r1,r2
    let index = vec!["r0".into(), "r1".into(), "r2".into()];
    let cols  = vec!["A".into(), "B".into(), "C".into()];
    // 3x3:
    // r0: 1, 10, 100
    // r1: 2, 20, 200
    // r2: 3, 30, 300
    let data = ND::from_vec_2d(3, 3, vec![
        1.0, 10.0, 100.0,
        2.0, 20.0, 200.0,
        3.0, 30.0, 300.0,
    ]);
    DataFrame::new(index, cols, data)
}
```
```rust
fn vec_eq(got: &[f64], exp: &[f64]) {
    assert_eq!(got.len(), exp.len());
    for i in 0..got.len() {
        assert!((got[i] - exp[i]).abs() <= 0.0, "mismatch at {i}: {} vs {}", got[i], exp[i]);
    }
}
```
```rust
#[test]
fn df_invariants_shape() {
    let df = df_small();
    let (n, m) = df.shape();
    assert_eq!(n, 3);
    assert_eq!(m, 3);
    assert_eq!(df.index.len(), 3);
    assert_eq!(df.columns.len(), 3);
}
```
```rust
#[test]
fn df_col_and_cols() {
    let df = df_small();

    let a = df.col("A");
    assert_eq!(a.data.ndim(), 1);
    vec_eq(&a.data.to_vec_fallback(), &[1.0, 2.0, 3.0]);

    let sub = df.cols(&["C", "A"]);
    let v = sub.data.as_2d();
    // [C,A] =>
    // r0: 100,1
    // r1: 200,2
    // r2: 300,3
    assert_eq!(v.nrows(), 3);
    assert_eq!(v.ncols(), 2);
    vec_eq(v.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[100.0, 1.0]);
    vec_eq(v.row(1).iter().copied().collect::<Vec<_>>().as_slice(), &[200.0, 2.0]);
    vec_eq(v.row(2).iter().copied().collect::<Vec<_>>().as_slice(), &[300.0, 3.0]);
}
```
```rust
#[test]
fn df_iloc_rows_cols_ranges_at() {
    let df = df_small();

    let r = df.iloc().rows(&[2, 0]);
    assert_eq!(r.index, vec!["r2".to_string(), "r0".to_string()]);
    let v = r.data.as_2d();
    vec_eq(v.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[3.0, 30.0, 300.0]);
    vec_eq(v.row(1).iter().copied().collect::<Vec<_>>().as_slice(), &[1.0, 10.0, 100.0]);

    let c = df.iloc().cols(&[1, 0]); // [B,A]
    assert_eq!(c.columns, vec!["B".to_string(), "A".to_string()]);
    let vc = c.data.as_2d();
    vec_eq(vc.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[10.0, 1.0]);

    let rr = df.iloc().row_range(1..3);
    assert_eq!(rr.index, vec!["r1".to_string(), "r2".to_string()]);

    let cr = df.iloc().col_range(0..2);
    assert_eq!(cr.columns, vec!["A".to_string(), "B".to_string()]);

    assert_eq!(df.iloc().at(2, 1), 30.0);
}
```
```rust
#[test]
fn df_filter_rows_and_fill_dropna() {
    let mut df = df_small();

    // NaN 넣기 (r1,B)
    let mut a = df.data.as_2d().to_owned();
    a[(1, 1)] = f64::NAN;
    df.data = ND::from_array(a.into_dyn());

    // fillna
    let f = df.fillna(-9.0);
    assert_eq!(f.data.as_2d()[(1, 1)], -9.0);

    // dropna: NaN 있는 row(r1) 제거 => r0,r2 남음
    let d = df.dropna();
    assert_eq!(d.index, vec!["r0".to_string(), "r2".to_string()]);
    let vd = d.data.as_2d();
    vec_eq(vd.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[1.0, 10.0, 100.0]);
    vec_eq(vd.row(1).iter().copied().collect::<Vec<_>>().as_slice(), &[3.0, 30.0, 300.0]);

    // filter_rows: keep r2 only
    let only_r2 = df.filter_rows(&[false, false, true]);
    assert_eq!(only_r2.index, vec!["r2".to_string()]);
    vec_eq(only_r2.data.as_2d().row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[3.0, 30.0, 300.0]);
}
```
```rust
#[test]
fn df_sort_values() {
    let df = df_small();
    let s = df.sort_values("B", false); // descending by B: 30,20,10
    assert_eq!(s.index, vec!["r2".to_string(), "r1".to_string(), "r0".to_string()]);
    let v = s.data.as_2d();

    vec_eq(&row_vec(&v, 0), &[3.0, 30.0, 300.0]);
    vec_eq(&row_vec(&v, 2), &[1.0, 10.0, 100.0]);

    vec_eq(&s.data.row_vec2(2), &[1.0, 10.0, 100.0]);
}
```
```rust
#[cfg(test)]
mod df_tests_case1 {
    use nurbslib::core::nd::ND;
    use nurbslib::core::pandas::{CsvOptions, DataFrame};

    fn df_small() -> DataFrame {
        let index = vec!["r0".into(), "r1".into(), "r2".into()];
        let cols  = vec!["A".into(), "B".into(), "K".into()];
        // K는 group key 용 (0,1,0)
        let data = ND::from_vec_2d(3, 3, vec![
            1.0, 10.0, 0.0,
            2.0, 20.0, 1.0,
            3.0, 30.0, 0.0,
        ]);
        DataFrame::new(index, cols, data)
    }

    fn row_vec(df: &DataFrame, r: usize) -> Vec<f64> {
        df.data.as_2d().row(r).iter().copied().collect()
    }
```
```rust
    #[test]
    fn df_head_tail() {
        let df = df_small();
        let h = df.head(2);
        assert_eq!(h.index, vec!["r0".to_string(), "r1".to_string()]);
        assert_eq!(row_vec(&h, 1), vec![2.0, 20.0, 1.0]);

        let t = df.tail(2);
        assert_eq!(t.index, vec!["r1".to_string(), "r2".to_string()]);
        assert_eq!(row_vec(&t, 0), vec![2.0, 20.0, 1.0]);
    }
```
```rust
    #[test]
    fn df_concat_rows_cols() {
        let df = df_small();
        let h = df.head(2);
        let t = df.tail(1);

        let cr = DataFrame::concat_rows(&h, &t);
        assert_eq!(cr.shape(), (3, 3));
        assert_eq!(cr.index, vec!["r0".to_string(), "r1".to_string(), "r2".to_string()]);
        assert_eq!(row_vec(&cr, 2), vec![3.0, 30.0, 0.0]);

        let left = df.cols(&["A","B"]);
        let right = df.cols(&["K"]);
        let cc = DataFrame::concat_cols(&left, &right);
        assert_eq!(cc.columns, vec!["A".to_string(), "B".to_string(), "K".to_string()]);
        assert_eq!(row_vec(&cc, 1), vec![2.0, 20.0, 1.0]);
    }
```
```rust
    #[test]
    fn df_describe_basic() {
        let df = df_small().cols(&["A","B"]); // 2 cols
        let d = df.describe();
        assert_eq!(d.index, vec!["count","mean","std","min","max"]);
        assert_eq!(d.columns, vec!["A".to_string(), "B".to_string()]);

        let a = d.data.as_2d();
        // count A,B = 3
        assert_eq!(a[(0,0)], 3.0);
        assert_eq!(a[(0,1)], 3.0);
        // min A=1, max A=3
        assert_eq!(a[(3,0)], 1.0);
        assert_eq!(a[(4,0)], 3.0);
    }
```
```rust
    #[test]
    fn df_groupby_i64_size_mean() {
        let df = df_small();
        let gb = df.groupby_i64("K");

        let sz = gb.size();
        // keys: 0,1
        assert_eq!(sz.index, vec!["0".to_string(), "1".to_string()]);
        // size for key0=2, key1=1
        let a = sz.data.as_2d();
        assert_eq!(a[(0,1)], 2.0);
        assert_eq!(a[(1,1)], 1.0);

        let m = gb.mean();
        // mean rows:
        // key0 rows: r0,r2 => A mean=2, B mean=20, K mean=0
        // key1 rows: r1 => A=2, B=20, K=1
        assert_eq!(m.index, vec!["0".to_string(), "1".to_string()]);
        assert_eq!(row_vec(&m, 0), vec![2.0, 20.0, 0.0]);
        assert_eq!(row_vec(&m, 1), vec![2.0, 20.0, 1.0]);
    }
```
```rust
    #[test]
    fn df_pretty_string_smoke() {
        let df = df_small();
        let s = df.to_pretty_string(10);
        assert!(s.contains("index"));
        assert!(s.contains("A"));
        assert!(s.contains("r0"));
    }
```
```rust
    #[test]
    fn df_value_counts_i64() {
        let df = df_small(); // K: 0,1,0
        let vc = df.value_counts_i64("K");
        assert_eq!(vc.index, vec!["0".to_string(), "1".to_string()]);
        let a = vc.data.as_2d();
        assert_eq!(a[(0, 1)], 2.0);
        assert_eq!(a[(1, 1)], 1.0);
    }
```
```rust
    #[test]
    fn compare_with_pandas_golden_groupby_mean() {
        let opt = CsvOptions::default();

        let df = DataFrame::read_csv("./tests/data/fem_input.csv", opt.clone()).unwrap();
        let got = df.groupby_i64("part_id").mean();

        let exp = DataFrame::read_csv("./tests/data/pandas_part_mean.csv", opt).unwrap();

        assert_eq!(got.columns, exp.columns);
        assert_eq!(got.index, exp.index);

        let a = got.data.as_2d();
        let b = exp.data.as_2d();
        assert_eq!(a.dim(), b.dim());

        for i in 0..a.nrows() {
            for j in 0..a.ncols() {
                let x = a[(i,j)];
                let y = b[(i,j)];
                if x.is_nan() { assert!(y.is_nan()); }
                else { assert!((x-y).abs() <= 1e-12, "mismatch at ({i},{j}) {x} vs {y}"); }
            }
        }
    }
```
```rust
    #[test]
    fn df_drop_duplicates_basic() {
        // rows:
        // r0: 1,10
        // r1: 2,20
        // r2: 1,10  (duplicate of r0)
        let index = vec!["r0".into(),"r1".into(),"r2".into()];
        let cols  = vec!["A".into(),"B".into()];
        let data = ND::from_vec_2d(3,2, vec![1.0,10.0, 2.0,20.0, 1.0,10.0]);
        let df = DataFrame::new(index, cols, data);

        let d = df.drop_duplicates(None);
        assert_eq!(d.index, vec!["r0".to_string(),"r1".to_string()]);

        let d2 = df.drop_duplicates(Some(&["A"]));
        // A만 보면 r0,r2 duplicate -> r0 keep
        assert_eq!(d2.index, vec!["r0".to_string(),"r1".to_string()]);
    }
}
```
```rust
use nurbslib::core::nd::ND;
use nurbslib::core::pandas::{row_vec, DataFrame}; // 네 경로에 맞춰 수정

fn df_small() -> DataFrame {
    // index: r0,r1,r2
    let index = vec!["r0".into(), "r1".into(), "r2".into()];
    let cols  = vec!["A".into(), "B".into(), "C".into()];
    // 3x3:
    // r0: 1, 10, 100
    // r1: 2, 20, 200
    // r2: 3, 30, 300
    let data = ND::from_vec_2d(3, 3, vec![
        1.0, 10.0, 100.0,
        2.0, 20.0, 200.0,
        3.0, 30.0, 300.0,
    ]);
    DataFrame::new(index, cols, data)
}

fn vec_eq(got: &[f64], exp: &[f64]) {
    assert_eq!(got.len(), exp.len());
    for i in 0..got.len() {
        assert!((got[i] - exp[i]).abs() <= 0.0, "mismatch at {i}: {} vs {}", got[i], exp[i]);
    }
}
```
```rust
#[test]
fn df_invariants_shape() {
    let df = df_small();
    let (n, m) = df.shape();
    assert_eq!(n, 3);
    assert_eq!(m, 3);
    assert_eq!(df.index.len(), 3);
    assert_eq!(df.columns.len(), 3);
}
```
```rust
#[test]
fn df_col_and_cols() {
    let df = df_small();

    let a = df.col("A");
    assert_eq!(a.data.ndim(), 1);
    vec_eq(&a.data.to_vec_fallback(), &[1.0, 2.0, 3.0]);

    let sub = df.cols(&["C", "A"]);
    let v = sub.data.as_2d();
    // [C,A] =>
    // r0: 100,1
    // r1: 200,2
    // r2: 300,3
    assert_eq!(v.nrows(), 3);
    assert_eq!(v.ncols(), 2);
    vec_eq(v.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[100.0, 1.0]);
    vec_eq(v.row(1).iter().copied().collect::<Vec<_>>().as_slice(), &[200.0, 2.0]);
    vec_eq(v.row(2).iter().copied().collect::<Vec<_>>().as_slice(), &[300.0, 3.0]);
}
```
```rust
#[test]
fn df_iloc_rows_cols_ranges_at() {
    let df = df_small();

    let r = df.iloc().rows(&[2, 0]);
    assert_eq!(r.index, vec!["r2".to_string(), "r0".to_string()]);
    let v = r.data.as_2d();
    vec_eq(v.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[3.0, 30.0, 300.0]);
    vec_eq(v.row(1).iter().copied().collect::<Vec<_>>().as_slice(), &[1.0, 10.0, 100.0]);

    let c = df.iloc().cols(&[1, 0]); // [B,A]
    assert_eq!(c.columns, vec!["B".to_string(), "A".to_string()]);
    let vc = c.data.as_2d();
    vec_eq(vc.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[10.0, 1.0]);

    let rr = df.iloc().row_range(1..3);
    assert_eq!(rr.index, vec!["r1".to_string(), "r2".to_string()]);

    let cr = df.iloc().col_range(0..2);
    assert_eq!(cr.columns, vec!["A".to_string(), "B".to_string()]);

    assert_eq!(df.iloc().at(2, 1), 30.0);
}
```
```rust
#[test]
fn df_filter_rows_and_fill_dropna() {
    let mut df = df_small();

    // NaN 넣기 (r1,B)
    let mut a = df.data.as_2d().to_owned();
    a[(1, 1)] = f64::NAN;
    df.data = ND::from_array(a.into_dyn());

    // fillna
    let f = df.fillna(-9.0);
    assert_eq!(f.data.as_2d()[(1, 1)], -9.0);

    // dropna: NaN 있는 row(r1) 제거 => r0,r2 남음
    let d = df.dropna();
    assert_eq!(d.index, vec!["r0".to_string(), "r2".to_string()]);
    let vd = d.data.as_2d();
    vec_eq(vd.row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[1.0, 10.0, 100.0]);
    vec_eq(vd.row(1).iter().copied().collect::<Vec<_>>().as_slice(), &[3.0, 30.0, 300.0]);

    // filter_rows: keep r2 only
    let only_r2 = df.filter_rows(&[false, false, true]);
    assert_eq!(only_r2.index, vec!["r2".to_string()]);
    vec_eq(only_r2.data.as_2d().row(0).iter().copied().collect::<Vec<_>>().as_slice(), &[3.0, 30.0, 300.0]);
}
```
```rust
#[test]
fn df_sort_values() {
    let df = df_small();
    let s = df.sort_values("B", false); // descending by B: 30,20,10
    assert_eq!(s.index, vec!["r2".to_string(), "r1".to_string(), "r0".to_string()]);
    let v = s.data.as_2d();

    vec_eq(&row_vec(&v, 0), &[3.0, 30.0, 300.0]);
    vec_eq(&row_vec(&v, 2), &[1.0, 10.0, 100.0]);

    vec_eq(&s.data.row_vec2(2), &[1.0, 10.0, 100.0]);
}
```
```rust
#[cfg(test)]
mod df_tests_case1 {
    use nurbslib::core::nd::ND;
    use nurbslib::core::pandas::{CsvOptions, DataFrame};
    fn df_small() -> DataFrame {
        let index = vec!["r0".into(), "r1".into(), "r2".into()];
        let cols  = vec!["A".into(), "B".into(), "K".into()];
        // K는 group key 용 (0,1,0)
        let data = ND::from_vec_2d(3, 3, vec![
            1.0, 10.0, 0.0,
            2.0, 20.0, 1.0,
            3.0, 30.0, 0.0,
        ]);
        DataFrame::new(index, cols, data)
    }

    fn row_vec(df: &DataFrame, r: usize) -> Vec<f64> {
        df.data.as_2d().row(r).iter().copied().collect()
    }
```
```rust
    #[test]
    fn df_head_tail() {
        let df = df_small();
        let h = df.head(2);
        assert_eq!(h.index, vec!["r0".to_string(), "r1".to_string()]);
        assert_eq!(row_vec(&h, 1), vec![2.0, 20.0, 1.0]);

        let t = df.tail(2);
        assert_eq!(t.index, vec!["r1".to_string(), "r2".to_string()]);
        assert_eq!(row_vec(&t, 0), vec![2.0, 20.0, 1.0]);
    }
```
```rust
    #[test]
    fn df_concat_rows_cols() {
        let df = df_small();
        let h = df.head(2);
        let t = df.tail(1);

        let cr = DataFrame::concat_rows(&h, &t);
        assert_eq!(cr.shape(), (3, 3));
        assert_eq!(cr.index, vec!["r0".to_string(), "r1".to_string(), "r2".to_string()]);
        assert_eq!(row_vec(&cr, 2), vec![3.0, 30.0, 0.0]);

        let left = df.cols(&["A","B"]);
        let right = df.cols(&["K"]);
        let cc = DataFrame::concat_cols(&left, &right);
        assert_eq!(cc.columns, vec!["A".to_string(), "B".to_string(), "K".to_string()]);
        assert_eq!(row_vec(&cc, 1), vec![2.0, 20.0, 1.0]);
    }
```
```rust
    #[test]
    fn df_describe_basic() {
        let df = df_small().cols(&["A","B"]); // 2 cols
        let d = df.describe();
        assert_eq!(d.index, vec!["count","mean","std","min","max"]);
        assert_eq!(d.columns, vec!["A".to_string(), "B".to_string()]);

        let a = d.data.as_2d();
        // count A,B = 3
        assert_eq!(a[(0,0)], 3.0);
        assert_eq!(a[(0,1)], 3.0);
        // min A=1, max A=3
        assert_eq!(a[(3,0)], 1.0);
        assert_eq!(a[(4,0)], 3.0);
    }
```
```rust
    #[test]
    fn df_groupby_i64_size_mean() {
        let df = df_small();
        let gb = df.groupby_i64("K");

        let sz = gb.size();
        // keys: 0,1
        assert_eq!(sz.index, vec!["0".to_string(), "1".to_string()]);
        // size for key0=2, key1=1
        let a = sz.data.as_2d();
        assert_eq!(a[(0,1)], 2.0);
        assert_eq!(a[(1,1)], 1.0);

        let m = gb.mean();
        // mean rows:
        // key0 rows: r0,r2 => A mean=2, B mean=20, K mean=0
        // key1 rows: r1 => A=2, B=20, K=1
        assert_eq!(m.index, vec!["0".to_string(), "1".to_string()]);
        assert_eq!(row_vec(&m, 0), vec![2.0, 20.0, 0.0]);
        assert_eq!(row_vec(&m, 1), vec![2.0, 20.0, 1.0]);
    }
```
```rust
    #[test]
    fn df_pretty_string_smoke() {
        let df = df_small();
        let s = df.to_pretty_string(10);
        assert!(s.contains("index"));
        assert!(s.contains("A"));
        assert!(s.contains("r0"));
    }
```
```rust
    #[test]
    fn df_value_counts_i64() {
        let df = df_small(); // K: 0,1,0
        let vc = df.value_counts_i64("K");
        assert_eq!(vc.index, vec!["0".to_string(), "1".to_string()]);
        let a = vc.data.as_2d();
        assert_eq!(a[(0, 1)], 2.0);
        assert_eq!(a[(1, 1)], 1.0);
    }
```
```rust
    #[test]
    fn compare_with_pandas_golden_groupby_mean() {
        let opt = CsvOptions::default();

        let df = DataFrame::read_csv("./tests/data/fem_input.csv", opt.clone()).unwrap();
        let got = df.groupby_i64("part_id").mean();

        let exp = DataFrame::read_csv("./tests/data/pandas_part_mean.csv", opt).unwrap();

        assert_eq!(got.columns, exp.columns);
        assert_eq!(got.index, exp.index);

        let a = got.data.as_2d();
        let b = exp.data.as_2d();
        assert_eq!(a.dim(), b.dim());

        for i in 0..a.nrows() {
            for j in 0..a.ncols() {
                let x = a[(i,j)];
                let y = b[(i,j)];
                if x.is_nan() { assert!(y.is_nan()); }
                else { assert!((x-y).abs() <= 1e-12, "mismatch at ({i},{j}) {x} vs {y}"); }
            }
        }
    }
```
```rust
    #[test]
    fn df_drop_duplicates_basic() {
        // rows:
        // r0: 1,10
        // r1: 2,20
        // r2: 1,10  (duplicate of r0)
        let index = vec!["r0".into(),"r1".into(),"r2".into()];
        let cols  = vec!["A".into(),"B".into()];
        let data = ND::from_vec_2d(3,2, vec![1.0,10.0, 2.0,20.0, 1.0,10.0]);
        let df = DataFrame::new(index, cols, data);

        let d = df.drop_duplicates(None);
        assert_eq!(d.index, vec!["r0".to_string(),"r1".to_string()]);

        let d2 = df.drop_duplicates(Some(&["A"]));
        // A만 보면 r0,r2 duplicate -> r0 keep
        assert_eq!(d2.index, vec!["r0".to_string(),"r1".to_string()]);
    }
}
```
