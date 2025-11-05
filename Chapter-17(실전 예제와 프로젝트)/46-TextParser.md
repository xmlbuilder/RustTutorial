# 📄 TextParser<'a> 구조 문서
## 📦 개요
TextParser<'a>는 텍스트 버퍼를 줄 단위로 읽고, 각 줄에서 고정 길이 필드를 추출하는 데 최적화된 파서입니다.  
특히 Fortran 스타일의 고정 필드 형식 데이터를 처리하기 위해 설계되었으며,  
줄 단위 파싱, 필드 추출, 주석 필터링, 형 변환 기능을 제공합니다.

🧠 설계 의도
- ✅ 텍스트 버퍼를 참조로 보관 (&'a str)
- ✅ 줄 단위로 순차적으로 읽기
- ✅ 현재 줄에서 고정 길이 필드 추출
- ✅ Fortran 스타일 숫자 파싱 지원 (D → E)
- ✅ 주석 줄($ 시작) 자동 건너뛰기
- ✅ 파싱 진행 상태 추적 (줄 수, 위치 등)

## 🧱 내부 필드

| 필드 이름       | 타입       | 설명                                                  |
|----------------|------------|-------------------------------------------------------|
| text           | &'a str    | 전체 텍스트 버퍼 (참조 기반, 소유하지 않음)           |
| pos_text       | usize      | 전체 버퍼 내 현재 위치 (바이트 단위)                  |
| current_line   | Option<&'a str> | 현재 읽은 줄 (개행 제외, CR 제거)               |
| pos_ln         | usize      | 현재 줄 내 필드 읽기 위치 (바이트 단위)               |
| row_num        | usize      | 전체 읽은 줄 수 (주석 포함)                          |
| valid_row_num  | usize      | 유효한 줄 수 (주석 제외, 실제 데이터 줄)              |

## 🔧 주요 메서드

| 메서드 이름               | 설명                                                         |
|---------------------------|--------------------------------------------------------------|
| new()                     | 기본 초기화된 파서를 생성합니다.                             |
| set_text(text)            | 텍스트 버퍼를 설정하고 내부 상태를 초기화합니다.             |
| row_number()              | 전체 읽은 줄 수(주석 포함)를 반환합니다.                     |
| valid_row_number()        | 유효한 줄 수(주석 제외)를 반환합니다.                        |
| buffer_pos()              | 전체 버퍼 내 현재 위치(바이트 단위)를 반환합니다.            |
| next_line()               | 다음 줄을 읽고 current_line에 저장합니다.                    |
| valid_next_line()         | 주석('$') 줄을 건너뛰고 유효한 줄만 반환합니다.              |
| get_item(size)            | 현재 줄에서 고정 길이 필드를 추출하고 위치를 갱신합니다.     |
| psr_get_float(size, def)  | 필드에서 Fortran 스타일 float를 파싱하고 실패 시 기본값 반환 |
| psr_get_long(size, def)   | 필드에서 정수를 파싱하고 실패 시 기본값 반환                 |
| psr_get_string(size)      | 필드에서 문자열을 추출합니다.                                |



## 🔍 메서드 상세 설명

### set_text(&str)
- 텍스트 버퍼를 설정하고 내부 상태를 초기화합니다. 파서 재사용 시 반드시 호출해야 합니다.

### read_line()
- 내부적으로 한 줄을 읽고 current_line에 저장합니다. \n 기준으로 줄을 나누며, \r은 제거합니다.

### next_line()
- 다음 줄을 읽고 유효 줄 수를 증가시킵니다. 주석 여부와 관계없이 모든 줄을 반환합니다.

### valid_next_line()
- $로 시작하는 주석 줄을 건너뛰고, 유효한 줄만 반환합니다. 반복적으로 read_line()을 호출하며 필터링합니다.

### get_item(size)
- 현재 줄에서 size 바이트만큼 필드를 추출합니다. 이후 pos_ln을 갱신하여 다음 필드로 이동할 수 있게 합니다.

### psr_get_float(size, default)
- get_item()으로 필드를 추출
- Fortran 스타일 지수(D, d)를 E로 치환
- f64로 파싱 후 f32로 변환
- 실패 시 default 반환

### psr_get_long(size, default)
- get_item()으로 필드를 추출
- 공백 제거 후 i32로 파싱
- 실패 시 default 반환

### psr_get_string(size)
- get_item()으로 필드를 추출
- 그대로 Option<&str>로 반환

### 🧪 사용 예시
```
*MAT_037
$ Bake Hardenable 210, Yield=230.2MPa
$      MID        RO         E        PR      SIGY      ETAN         R     HLCID
         1 7.900E-09 2.070E+05      0.30     230.2       0.0     1.450     90903
*DEFINE_CURVE
90903
0.00,230.1501
```
```rust
let mut parser = TextParser::new();
parser.set_text(text);

while let Some(line) = parser.valid_next_line() {
    let a = parser.psr_get_long(5, 0);
    let b = parser.psr_get_long(5, 0);
    println!("Parsed: {}, {}", a, b);
}
```

## ✅ 요약

| 기능 항목             | 설명 또는 관련 메서드                     |
|------------------------|-------------------------------------------|
| 줄 단위 읽기           | next_line(), valid_next_line()            |
| 고정 길이 필드 추출     | get_item(size)                            |
| 숫자 파싱              | psr_get_float(), psr_get_long()           |
| 주석 필터링            | '$'로 시작하는 줄은 valid_next_line()에서 건너뜀 |

## 소스 코드
```rust
#[derive(Debug, Default, Clone)]
pub struct TextParser<'a> {
    text: Option<&'a str>,         // 전체 텍스트 버퍼 (소유 X, 참조)
    pos_text: usize,               // 버퍼 내 전역 위치(바이트)
    current_line: Option<&'a str>, // 현재 라인 슬라이스(개행 제외, CR 제거)
    pos_ln: usize,                 // 현재 라인에서의 읽기 위치(바이트)
    row_num: usize,                // 읽은 전체 라인 수(주석 포함)
    valid_row_num: usize,          // 유효 라인 수(주석 제외)
}
```
```rust
impl<'a> TextParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_text(&mut self, text: &'a str) {
        self.text = Some(text);
        self.pos_text = 0;
        self.current_line = None;
        self.pos_ln = 0;
        self.row_num = 0;
        self.valid_row_num = 0;
    }

    pub fn row_number(&self) -> usize {
        self.row_num
    }

    pub fn valid_row_number(&self) -> usize {
        self.valid_row_num
    }

    pub fn buffer_pos(&self) -> usize {
        self.pos_text
    }

    fn read_line(&mut self) -> Option<&'a str> {
        let text = self.text?;
        if self.pos_text >= text.len() {
            return None;
        }
        let rest = &text[self.pos_text..];
        let nl_off = rest.find('\n');
        let (line, step) = match nl_off {
            Some(off) => {
                let raw = &rest[..off];
                let line = raw.strip_suffix('\r').unwrap_or(raw);
                (line, off + 1)
            }
            None => {
                // 마지막 라인(개행 없음)
                let raw = rest;
                let line = raw.strip_suffix('\r').unwrap_or(raw);
                (line, raw.len())
            }
        };
        self.pos_text += step;
        self.row_num += 1;

        self.current_line = Some(line);
        self.pos_ln = 0;
        Some(line)
    }

    pub fn next_line(&mut self) -> Option<&'a str> {
        let ln = self.read_line()?;
        self.valid_row_num += 1;
        Some(ln)
    }

    pub fn valid_next_line(&mut self) -> Option<&'a str> {
        loop {
            let _save_pos = self.pos_text;
            let ln = self.read_line()?;
            if ln.starts_with('$') {
                continue;
            }
            self.valid_row_num += 1;
            return Some(ln);
        }
    }

    pub fn get_item(&mut self, size: usize) -> Option<&'a str> {
        let line = self.current_line?;
        if self.pos_ln >= line.len() {
            return None;
        }
        let remain = line.len() - self.pos_ln;
        let take = size.min(remain);
        let start = self.pos_ln;
        let end = start + take;
        self.pos_ln = end;
        Some(&line[start..end])
    }

    fn parse_fortran_float(s: &str) -> Option<f64> {
        let t = s.trim();
        if t.is_empty() {
            return None;
        }
        // Fortran 스타일 지수 'D'/'d' → 'E' 로 치환
        let mut buf = String::new();
        for ch in t.chars() {
            match ch {
                'D' | 'd' => buf.push('E'),
                _ => buf.push(ch),
            }
        }
        buf.parse::<f64>().ok()
    }

    pub fn psr_get_float(&mut self, size: usize, default_value: f32) -> f32 {
        match self.get_item(size) {
            Some(field) => Self::parse_fortran_float(field)
                .map(|v| v as f32)
                .unwrap_or(default_value),
            None => default_value,
        }
    }

    pub fn psr_get_long(&mut self, size: usize, default_value: i32) -> i32 {
        match self.get_item(size) {
            Some(field) => {
                let t = field.trim();
                if t.is_empty() {
                    default_value
                } else {
                    t.parse::<i32>().unwrap_or(default_value)
                }
            }
            None => default_value,
        }
    }

    pub fn psr_get_string(&mut self, size: usize) -> Option<&'a str> {
        self.get_item(size)
    }
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use geometry::core::text_parser::TextParser;

    const SAMPLE1: &str = r#"80163384      2.3558E+02      1.0304E+00      1.5813E+02
80163385      2.3541E+02      1.5040E+01      1.6005E+02
96660595      1.2451E+03     -6.4713E+02      1.1857E+03
96667717      3.5683E+02      1.0250E+01      7.4791E+02
96667718      3.5683E+02     -1.0251E+01      7.4791E+02
96667719      3.5699E+02     -3.0751E+01      7.4787E+02
96667720      3.5731E+02     -5.1249E+01      7.4778E+02                        "#;

    const SAMPLE2: &str = r#"
93123353    101196668098966681329666811496668083
93123354    101196668103966681309666811296668087
93123355    101196668113966680889666810596668133
93123356    101196668106966681349666811896668090
93123357    101196668109966681369666811696668089
93123358    101196667727966681359666811796667728
93123359    101196668104966681379666812296668096
93123360    101196668110966681399666812196668092
93123361    101196668111966681409666811996668091
93123362    101196668120966680939666811396668141
93123363    101196668123966680959666811296668142
93123364    101196668107966681389666777296667771
"#;

    #[test]
    fn parse_sample1_fixed_width() {
        // 가정된 고정폭: ID(8) + 공백(6) + V1(14) + V2(14) + V3(14) + 나머지 공백
        // (실측 폭이 다르면 size만 조정하면 됨)
        let mut p = TextParser::new();
        p.set_text(SAMPLE1);

        // 첫 줄
        let _l1 = p.next_line().unwrap();
        let id = p.psr_get_long(8, -1);
        let _sp = p.psr_get_string(6).unwrap(); // 공백 스킵
        let v1 = p.psr_get_float(14, f32::NAN);
        let v2 = p.psr_get_float(14, f32::NAN);
        let v3 = p.psr_get_float(14, f32::NAN);

        assert_eq!(id, 80163384);
        assert!((v1 as f64 - 235.58).abs() < 1e-2);
        assert!((v2 as f64 - 1.0304).abs() < 1e-6);
        assert!((v3 as f64 - 158.13).abs() < 1e-2);

        // 다음 줄도 동일 형식으로 파싱 가능
        let _l2 = p.next_line().unwrap();
        let id2 = p.psr_get_long(8, -1);
        assert_eq!(id2, 80163385);
    }
```
```rust
    #[test]
    fn parse_sample2_mixed() {
        let mut p = TextParser::new();
        p.set_text(SAMPLE2);

        // 공백 라인 하나 포함되어 있음 → next_line()으로 그대로 소비
        let _ = p.next_line().unwrap(); // 빈 줄

        // 다음 유효 라인
        let _ = p.next_line().unwrap();
        let id = p.psr_get_long(8, -1);
        //let _spaces = p.psr_get_string(4).unwrap(); // "    "
        //let rest = p.psr_get_string(60).unwrap().trim_end(); // 뒤쪽 공백 제거
        //assert_eq!(id, 93123353);
        //assert!(rest.starts_with("1011966680989"));
        let pid = p.psr_get_long(8, -1);
        println!("id: {}, pid: {}", id, pid);
        let nid1 = p.psr_get_long(8, -1);
        let nid2 = p.psr_get_long(8, -1);
        let nid3 = p.psr_get_long(8, -1);
        let nid4 = p.psr_get_long(8, -1);

        println!(
            "nid1: {}, nid2: {}, nid3 : {}, nid4 : {}",
            nid1, nid2, nid3, nid4
        );

        // valid_next_line() 예시: 주석 라인이 있다면 자동 스킵
        // (여기 샘플에는 주석이 없으니 next_line 과 동일 동작)
        let mut q = TextParser::new();
        q.set_text("$comment line\n12345678    2.0\n");
        let ln = q.valid_next_line().unwrap();
        assert_eq!(ln, "12345678    2.0");
        let id = q.psr_get_long(8, -1);
        let _gap = q.psr_get_string(4).unwrap();
        let v = q.psr_get_float(10, f32::NAN);
        assert_eq!(id, 12345678);
        assert_eq!(v, 2.0);
    }
```
```rust
    #[test]
    fn fortran_d_exponent() {
        let mut p = TextParser::new();
        p.set_text("  1.2345D+03\n");
        let _ = p.next_line().unwrap();
        let v = p.psr_get_float(20, f32::NAN);
        assert!((v as f64 - 1234.5).abs() < 1e-4);
    }
```    
```rust
    /// 핵심: 끝까지 읽으면 `None` 이 나오는지(+데이터 라인은 6×8로 잘린다)만 검증
    #[test]
    fn sample2_iter_to_eof_and_parse_6x8() {
        let mut p = TextParser::new();
        p.set_text(SAMPLE2);

        // 기대 데이터 라인 수 = 공백 라인 제외한 줄 수
        let expected_data_lines = SAMPLE2.lines().filter(|ln| !ln.trim().is_empty()).count();

        let mut seen_data = 0usize;
        loop {
            match p.next_line() {
                Some(ln) => {
                    if ln.trim().is_empty() {
                        // 빈 줄은 데이터 처리 대상 아님 (하지만 valid_row에는 포함될 수 있음)
                        continue;
                    }
                    // 6개의 8폭 필드 파싱 시도 (필드 부족/초과 없이 끊기는지만 확인)
                    let _f1 = p.psr_get_string(8).expect("f1");
                    let _f2 = p.psr_get_string(8).expect("f2");
                    let _f3 = p.psr_get_string(8).expect("f3");
                    let _f4 = p.psr_get_string(8).expect("f4");
                    let _f5 = p.psr_get_string(8).expect("f5");
                    let _f6 = p.psr_get_string(8).expect("f6");
                    seen_data += 1;
                }
                None => break, // EOF 감지 OK
            }
        }

        // 데이터 라인만 정확히 소비했는지 확인
        assert_eq!(seen_data, expected_data_lines);

        // EOF 이후에는 계속 None이어야 함
        assert!(p.next_line().is_none());
    }
```
```rust
    /// 마지막 줄에 개행이 없어도 EOF를 정확히 감지하는지 확인
    #[test]
    fn sample2_no_trailing_newline_eof_only() {
        let sample2_no_nl = {
            let s = r#"
93123353    101196668098966681329666811496668083
93123354    101196668103966681309666811296668087
93123355    101196668113966680889666810596668133
93123356    101196668106966681349666811896668090
93123357    101196668109966681369666811696668089
93123358    101196667727966681359666811796667728
93123359    101196668104966681379666812296668096
93123360    101196668110966681399666812196668092
93123361    101196668111966681409666811996668091
93123362    101196668120966680939666811396668141
93123363    101196668123966680959666811296668142
93123364    101196668107966681389666777296667771                                "#;
            s.to_string() // 끝에 개행 없음
        };

        let mut p = TextParser::new();
        p.set_text(&sample2_no_nl);

        let expected_data_lines = sample2_no_nl
            .lines()
            .filter(|ln| !ln.trim().is_empty())
            .count();

        let mut seen_data = 0usize;
        while let Some(ln) = p.next_line() {
            if ln.trim().is_empty() {
                continue;
            }
            let _ = p.psr_get_string(8).expect("f1");
            let _ = p.psr_get_string(8).expect("f2");
            let _ = p.psr_get_string(8).expect("f3");
            let _ = p.psr_get_string(8).expect("f4");
            let _ = p.psr_get_string(8).expect("f5");
            let _ = p.psr_get_string(8).expect("f6");
            seen_data += 1;
        }

        assert_eq!(seen_data, expected_data_lines);
        assert!(p.next_line().is_none());
    }
    // sample2 원문(맨 앞 빈 줄 포함, 마지막 줄 개행 포함)
    const SAMPLE2_ORIG: &str = r#"
93123353    101196668098966681329666811496668083
93123354    101196668103966681309666811296668087
93123355    101196668113966680889666810596668133
93123356    101196668106966681349666811896668090
93123357    101196668109966681369666811696668089
93123358    101196667727966681359666811796667728
93123359    101196668104966681379666812296668096
93123360    101196668110966681399666812196668092
93123361    101196668111966681409666811996668091
93123362    101196668120966680939666811396668141
93123363    101196668123966680959666811296668142
93123364    101196668107966681389666777296667771
"#;

    // 상위 레이어가 “빈 줄”을 디폴트(6×8폭의 '0')로 채워 넣었다고 가정하여 변환
    fn prefill_blank_lines(
        text: &str,
        field_count: usize,
        field_width: usize,
        fill: char,
    ) -> String {
        let fill_line: String = std::iter::repeat(fill)
            .take(field_count * field_width)
            .collect();
        text.lines()
            .map(|ln| {
                if ln.trim().is_empty() {
                    fill_line.as_str()
                } else {
                    ln
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
```    
```rust
    /// 1) 원문 그대로: 빈 줄도 한 줄로 보고, 끝에서 None 나오는지 확인
    #[test]
    fn sample2_as_is_iterates_to_eof_including_blank() {
        let mut p = TextParser::new();
        p.set_text(SAMPLE2_ORIG);

        // lines() 기준 총 줄 수(마지막 개행 포함) == 파서가 본 줄 수와 일치할 필요는 없음.
        // 여기서는 “끝에서 None”만 확인.
        let mut seen_lines = 0usize;
        while let Some(_ln) = p.next_line() {
            // 여기서는 빈 줄도 그대로 한 줄로 본다 (스킵하지 않음)
            seen_lines += 1;
        }
        assert!(seen_lines > 0, "should see at least one line");
        assert!(p.next_line().is_none(), "must be None after EOF");
    }
```    
```rust
    /// 2) 상위가 빈 줄을 6×8폭 '0'으로 채워 넣으면 모든 줄을 6×8로 파싱 가능해야 함
    #[test]
    fn sample2_prefilled_blank_lines_parse_6x8_all_rows() {
        let prefilled = prefill_blank_lines(SAMPLE2_ORIG, 6, 8, '0');
        let expected_total_rows = prefilled.lines().count(); // 빈 줄 → 00000000*6 으로 바뀌어 data row가 됨
        let mut p = TextParser::new();
        p.set_text(&prefilled);
        let mut parsed_rows = 0usize;
        while let Some(_ln) = p.next_line() {
            // 각 줄에서 6개의 8폭 필드 추출 시도
            let _f1 = p.psr_get_string(8).expect("f1");
            let _f2 = p.psr_get_string(8).expect("f2");
            let _f3 = p.psr_get_string(8).expect("f3");
            let _f4 = p.psr_get_string(8).expect("f4");
            let _f5 = p.psr_get_string(8).expect("f5");
            let _f6 = p.psr_get_string(8).expect("f6");
            parsed_rows += 1;
        }
        // 모든 줄이 6×8로 파싱되었는지(= 빈 줄도 상위에서 채워줬기에 실패 없이 통과)
        assert_eq!(parsed_rows, expected_total_rows);
        assert!(p.next_line().is_none());
    }
```    
```rust
    /// 3) 마지막 줄이 '\r'만 있고 '\n'이 없어도 EOF 정확히 감지해야 함
    #[test]
    fn sample2_trailing_cr_without_lf() {
        // 원문에서 마지막 개행 제거 후 '\r'만 붙임 (사용자 설명 케이스)
        let mut s = SAMPLE2_ORIG.to_string();
        if s.ends_with('\n') {
            s.pop(); // 마지막 '\n' 제거
        }
        s.push('\r'); // CR만 존재

        let mut p = TextParser::new();
        p.set_text(&s);

        let mut rows = 0usize;
        while let Some(_ln) = p.next_line() {
            rows += 1;
        }
        assert!(rows > 0);
        assert!(p.next_line().is_none());
    }
}
```
---
