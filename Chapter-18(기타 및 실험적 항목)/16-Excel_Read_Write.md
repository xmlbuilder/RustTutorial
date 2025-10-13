# #xcel Read /Write

## ✅ Cargo.toml 설정
```
[dependencies]
umya-spreadsheet = "1.0"
chrono = "0.4"
```
## 전체 샘플 코드: Excel에 float 값 쓰기
```rust
#[cfg(test)]
mod tests {
    use umya_spreadsheet::{reader, writer};
    use std::path::Path;

    #[test]
    fn excel_write_float_cell() {
        // 1. 엑셀 파일 읽기
        let path = Path::new("template.xlsx");
        let mut book = reader::xlsx::read(path).unwrap();

        // 2. 시트 가져오기
        let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

        // 3. 셀에 float 값을 문자열로 설정 (Excel에서 숫자로 인식됨)
        sheet.get_cell_mut("A1").set_value("1234.56".to_string());
        sheet.get_cell_mut("B1").set_value("7890.12".to_string());

        // 4. 엑셀 파일 저장
        writer::xlsx::write(&book, "output.xlsx").unwrap();
    }
}
```
## ✅ 단계별 함수 정리
| 단계 | 설명                     | 함수 또는 메서드                                      | 비고                         |
|------|--------------------------|--------------------------------------------------------|------------------------------|
| 1️⃣   | Excel 파일 읽기          | `reader::xlsx::read(Path)`                            | 기존 파일 열기               |
| 2️⃣   | 시트 가져오기           | `book.get_sheet_by_name_mut("Sheet1")`                | 시트 이름으로 mutable 참조   |
| 3️⃣   | 셀에 값 쓰기 (float)     | `cell.set_value("1234.56".to_string())`               | 문자열로 넣으면 숫자로 인식 |
| 4️⃣   | Excel 파일 저장          | `writer::xlsx::write(&book, "output.xlsx")`           | 변경 내용 저장               |


## 💡 참고
- set_value()는 String만 받기 때문에 f64는 .to_string()으로 변환 필요
- Excel은 "1234.56" 같은 문자열도 숫자 셀로 자동 인식함
- get_cell_mut("A1")처럼 셀 주소는 "A1" 형식의 문자열로 지정


## ✅ 왜 문자열로 넣는가?
- umya_spreadsheet의 set_value()는 내부적으로 String만 받습니다
- f64를 직접 넣으면 컴파일 오류 발생
- "1234.56"처럼 문자열로 넣으면 Excel이 자동으로 숫자로 인식합니다

## ✅ 결과
- A1 셀: 1234.56 (Excel에서 숫자 셀로 보임)
- B1 셀: 7890.12
- 수식, 정렬, 숫자 서식 등 Excel 기능 정상 작동

---

# ✅ 전체 과정 단계별 설명

## 소스 코드
```rust
fn excel_write_float_cell() {
    // 1. 엑셀 파일 읽기
    // 1. 엑셀 파일 읽기
    let path = Path::new("asset/template.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    // 2. 시트 가져오기
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    // 3. A1, B1 셀에 float 값을 문자열로 설정 (Excel에서 숫자로 인식됨)
    sheet.get_cell_mut("A1").set_value("1234.56".to_string());
    sheet.get_cell_mut("B1").set_value("7890.12".to_string());

    // 4. 날짜 셀 입력 (Excel이 날짜로 인식)
    let date = NaiveDate::from_ymd_opt(2025, 10, 13).unwrap();
    sheet.get_cell_mut("D2").set_value(date.to_string());

    // 5. 수식 셀 설정
    sheet.get_cell_mut("C1").set_formula("SUM(A1:B1)");

    // 6. 병합 셀 설정 및 값 입력
    sheet.add_merge_cells("D1:F1");
    sheet.get_cell_mut("D1").set_value("Merged Cell");

    // 7. 스타일 생성 및 적용 (노란 배경)
    let mut style = Style::default();
    style
        .get_fill_mut()
        .get_pattern_fill_mut()
        .get_foreground_color_mut()
        .set_argb("FFFF00"); // 노란색

    sheet.get_cell_mut("D1").set_style(style);

    // 8. A1~A10 반복 입력 (1.5씩 증가)
    for i in 1..=10 {
        let coord = format!("A{}", i);
        sheet
            .get_cell_mut(coord.as_str())
            .set_value(format!("{}", i as f64 * 1.5));
    }

    // 9. 엑셀 파일 저장
    writer::xlsx::write(&book, "asset/output.xlsx").unwrap();

}
```


## 1️⃣ 엑셀 파일 읽기
```rust
let path = Path::new("asset/template.xlsx");
let mut book = reader::xlsx::read(path).unwrap();
```

- template.xlsx 파일을 읽어서 book 객체로 로딩
- 기존 시트와 셀 구조를 유지한 채 수정 가능

## 2️⃣ 시트 가져오기
```rust
let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
```

- "Sheet1"이라는 이름의 시트를 mutable 참조로 가져옴
- 이후 셀 수정, 병합, 스타일 적용 등을 이 시트를 기준으로 진행

## 3️⃣ 셀에 float 값 입력 (A1, B1)
```rust
sheet.get_cell_mut("A1").set_value("1234.56".to_string());
sheet.get_cell_mut("B1").set_value("7890.12".to_string());
```

- f64 값을 문자열로 변환해서 입력
- Excel은 "1234.56" 같은 문자열도 숫자 셀로 자동 인식함
- 수식 계산, 정렬, 숫자 서식 등 정상 작동

## 4️⃣ 날짜 셀 입력 (D2)
```rust
let date = NaiveDate::from_ymd_opt(2025, 10, 13).unwrap();
sheet.get_cell_mut("D2").set_value(date.to_string());
```

- 날짜를 "2025-10-13" 형식의 문자열로 입력
- Excel은 이를 날짜 셀로 자동 인식함

## 5️⃣ 수식 셀 설정 (C1)
```rust
sheet.get_cell_mut("C1").set_formula("SUM(A1:B1)");
```

- C1 셀에 수식 =SUM(A1:B1)을 설정
- Excel에서 자동 계산되어 A1 + B1 결과가 표시됨

## 6️⃣ 셀 병합 (D1:F1)
```rust
sheet.add_merge_cells("D1:F1");
sheet.get_cell_mut("D1").set_value("Merged Cell");
```

- D1부터 F1까지 셀을 병합
- 병합된 셀의 시작점인 D1에만 값을 설정해야 함

## 7️⃣ 셀 스타일 적용 (D1) -> 현재 적용 안됨(원인 분석 중)
```rust
// 스타일 객체 생성 및 배경색 설정
let mut style = Style::default();
style
    .get_fill_mut()
    .get_pattern_fill_mut()
    .get_foreground_color_mut()
    .set_argb("FFFF00");

sheet.get_cell_mut("D1").set_style(style);
```

- D1 셀에 노란 배경 스타일 적용 시도
- 현재 umya_spreadsheet에서 스타일이 Excel에 반영되지 않을 수 있음
- Excel에서 직접 스타일을 설정해두는 방식이 더 안정적

## 8️⃣ 반복적으로 셀 값 입력 (A1~A10)
```rust
for i in 1..=10 {
    let coord = format!("A{}", i);
    sheet
        .get_cell_mut(coord.as_str())
        .set_value(format!("{}", i as f64 * 1.5));
}
```

- A1부터 A10까지 셀에 1.5, 3.0, 4.5 … 15.0까지 입력
- 반복적으로 float 값을 문자열로 변환해 입력
- Excel에서 숫자 셀로 인식됨

## 9️⃣ 엑셀 파일 저장
```rust
writer::xlsx::write(&book, "asset/output.xlsx").unwrap();
```
- 수정된 내용을 "output.xlsx" 파일로 저장
- 기존 템플릿을 기반으로 값, 수식, 병합 등이 반영된 새 파일 생성

## 🧾 결과 요약 (Excel에서 보이는 내용)
|   A    |       B       |         C         |       D       |   E   |   F   |
|--------|---------------|-------------------|---------------|-------|-------|
| 1234.56|    7890.12    | =SUM(A1:B1)       | Merged Cell   |       |       |
|  3.0   |               |                   | 2025-10-13    |       |       |
|  4.5   |               |                   |               |       |       |
|  6.0   |               |                   |               |       |       |
|  7.5   |               |                   |               |       |       |
|  9.0   |               |                   |               |       |       |
| 10.5   |               |                   |               |       |       |
| 12.0   |               |                   |               |       |       |
| 13.5   |               |                   |               |       |       |
| 15.0   |               |                   |               |       |       |


----

# Sheet 수정 코드 분석

```rust

//1.
let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

//2.
let sheet2 = book.get_sheet_by_name_mut("Sheet2").unwrap();

//3.
let sheet_name = "MySheet";
book.new_sheet(sheet_name);
let sheet = book.get_sheet_by_name_mut(sheet_name).unwrap();
sheet.get_cell_mut("A1").set_value("Hello from MySheet");

//4.
for sheet in book.get_sheet_collection() {
    println!("시트 이름: {}", sheet.get_name());
}

//5.
for sheet in book.get_sheet_collection_mut() {
    let name = sheet.get_name().to_string(); // 복사해서 소유권 획득
    sheet.get_cell_mut("G1").set_value(format!("Hello from {}", name));
}
```

umya_spreadsheet에서 다른 시트로 이동하거나 작업하는 방법은 매우 간단.  
아래에 시트를 가져오고, 새 시트를 만들고, 시트 간 이동하는 흐름을 단계별로 정리.  

## ✅ 1. 시트 가져오기 (기존 시트)
```rust
let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
```

- "Sheet1"이라는 이름의 시트를 mutable 참조로 가져옵니다
- 이후 셀 수정, 병합, 수식 설정 등을 이 시트에서 수행

## ✅ 2. 다른 시트로 이동 (예: "Sheet2")
```rust
let sheet2 = book.get_sheet_by_name_mut("Sheet2").unwrap();
```

- "Sheet2"라는 이름의 시트가 이미 존재해야 합니다
- 없다면 다음 단계에서 새로 만들 수 있어요

## ✅ 3. 새 시트 만들기
```rust
use umya_spreadsheet::Worksheet;
let sheet_name = "MySheet";
book.new_sheet(sheet_name);
let sheet = book.get_sheet_by_name_mut(sheet_name).unwrap();
sheet.get_cell_mut("A1").set_value("Hello from MySheet");
```

- "MySheet"라는 이름의 새 시트를 생성하고 book에 추가
- 이후 get_sheet_by_name_mut("MySheet")로 접근 가능

## ✅ 4. 시트 목록 확인 및 반복 처리
```rust
for sheet in book.get_sheet_collection() {
    println!("시트 이름: {}", sheet.get_name());
}
```
- get_sheet_collection()은 Vec<&Worksheet>를 반환합니다
- 각 Worksheet에서 get_name()으로 시트 이름을 가져올 수 있어요


- 모든 시트 이름을 확인할 수 있어요
- 반복적으로 여러 시트에 작업할 때 유용

## ✅ 5. 여러 시트에 값 쓰기
```rust
for sheet in book.get_sheet_collection_mut() {
    let name = sheet.get_name();
    sheet.get_cell_mut("A1").set_value(format!("Hello from {}", name));
}
```

## 🔍 설명
- get_sheet_collection_mut() → 모든 시트에 대한 mutable 참조 목록 반환
- sheet.get_name() → 해당 시트의 이름 반환
- get_cell_mut("A1") → A1 셀에 접근해서 값 설정

---
