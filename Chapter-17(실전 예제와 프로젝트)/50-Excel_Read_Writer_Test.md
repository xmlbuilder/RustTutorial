# umya_spreadsheet

## 📘 Excel 작업에 자주 쓰이는 umya_spreadsheet API 정리

### 📄 파일 생성 및 저장

| 기능 설명           | 사용 API 예시                                      |
|--------------------|----------------------------------------------------|
| 새 엑셀 파일 생성   | `umya_spreadsheet::new_file()`                    |
| 엑셀 파일 읽기      | `reader::xlsx::read(path)`                        |
| 엑셀 파일 저장      | `writer::xlsx::write(&book, path)`                |
| 경량 압축 저장      | `writer::xlsx::write_light(&book, path)`          |
| CSV 파일 저장       | `writer::csv::write(&book, path, Some(&opt))`     |


## 📑 시트 관련

| 기능 설명             | 사용 API 예시                                      |
|----------------------|----------------------------------------------------|
| 새 시트 생성          | `book.new_sheet("SheetName")`                     |
| 시트 복사 및 추가     | `book.add_sheet(clone_sheet)`                     |
| 시트 삭제             | `book.remove_sheet_by_name("SheetName")`          |
| 시트 가져오기         | `book.get_sheet_by_name("SheetName")`             |
| 활성 시트 설정        | `book.set_active_sheet(index)`                    |

## 🧮 셀 관련

| 기능 설명             | 사용 API 예시                                      |
|----------------------|----------------------------------------------------|
| 셀 값 설정 (문자열)   | `sheet.get_cell_mut("A1").set_value("text")`      |
| 셀 값 설정 (숫자)     | `set_value_number(123)`                           |
| 셀 값 설정 (불리언)   | `set_value_bool(true)`                            |
| 셀 값 읽기            | `get_cell("A1").unwrap().get_value()`             |
| 셀 삭제               | `remove_cell((row, col))`                         |
| 셀 병합               | `add_merge_cells("A1:B2")`                        |

## 🎨 스타일 관련

| 기능 설명               | 사용 API 예시                                                      |
|------------------------|---------------------------------------------------------------------|
| 셀 배경색 설정          | `set_background_color(Color::COLOR_BLUE)`                          |
| 셀 배경 패턴 설정       | `set_background_color_with_pattern(...)`                           |
| 글자색 설정             | `get_font_mut().get_color_mut().set_argb(...)`                     |
| 폰트 크기 설정          | `get_font_size_mut().set_val(20f64)`                               |
| 테두리 스타일 설정      | `get_borders_mut().get_bottom_mut().set_border_style(...)`         |
| 셀 줄바꿈 설정          | `get_alignment_mut().set_wrap_text(true)`                          |
| 셀 범위 스타일 적용     | `sheet.set_style_by_range("A1:A3", style)`                         |

## 📊 차트 관련

| 기능 설명         | 사용 API 예시                                         |
|------------------|--------------------------------------------------------|
| 차트 생성         | `Chart::new_chart(ChartType::LineChart, ...)`        |
| 차트 삽입         | `sheet.add_chart(chart)`                              |

이 외에도 다양한 차트 타입이 지원됩니다:
- ChartType::PieChart
- ChartType::BarChart
- ChartType::AreaChart
- ChartType::ScatterChart
- ChartType::BubbleChart
- ChartType::RadarChart
- ChartType::Line3DChart, Pie3DChart, Bar3DChart 등 3D 차트도 포함


## 🖼 이미지 관련

| 기능 설명           | 사용 API 예시                                |
|--------------------|-----------------------------------------------|
| 이미지 삽입         | `sheet.add_image(image)`                     |
| 이미지 교체         | `image.change_image("path")`                 |
| 이미지 다운로드     | `image.download_image("path")`               |

## 🧪 기타 기능

| 기능 설명                 | 사용 API 예시                                         |
|--------------------------|--------------------------------------------------------|
| 열 너비 자동 조정         | `get_column_dimension_mut("A").set_auto_width(true)`  |
| 열 너비 수동 설정         | `set_width(60f64)`                                    |
| HTML → RichText 변환      | `html_to_richtext(html)`                              |



# 테스트 코드 설명
라이브러리를 활용한 엑셀 처리 테스트 기능 전체 요약을 정리해.  
특히 read_and_wite_xlsm_method는 시트 복사/삭제, 다양한 차트 삽입 등 고급 기능을 포함.


## 📋 전체 테스트 기능 요약표

| 테스트 함수 이름               | 주요 기능 또는 포인트                         |
|-------------------------------|-----------------------------------------------|
| excel_read_writer_tests       | 기본 읽기/쓰기 테스트                         |
| excel_write_float_cell        | 숫자 입력, 수식(SUM(A1:B1)), 병합(D1:F1), 스타일, 반복 입력(A1~A10) |
| read_and_write_by_empty       | 빈 파일 처리 (.xlsx)                          |
| lazy_read_and_write_by_empty  | lazy_read 방식 빈 파일 처리                   |
| read_and_wite_method          | 셀 삭제, 포맷 확인, 스타일 설정               |
| read_and_wite                 | 이미지 처리(M17), read_and_wite_method 포함   |
| read_and_wite_xlsm_method     | 시트 복사/삭제, 다양한 차트 삽입              |
| read_and_write_xlsm           | .xlsm 파일 처리                               |
| insert_and_remove_cells       | 행/열 삽입 및 제거                            |
| new_sheet_and_edit            | 시트 생성, 셀 값 설정, 스타일 범위 적용       |
| new_file_and_edit             | 새 파일 생성, 셀 값/스타일/병합/열 너비 설정 |
| new_and_write                 | 빈 새 파일 저장                               |
| duplicate_sheet               | 시트 이름 중복 방지 테스트                    |
| writer_csv                    | CSV 저장 (Shift-JIS, trim, wrap)              |
| compression_test              | 압축 저장(write vs write_light)              |
| html_to_rich_text_test        | HTML → RichText 변환 및 줄바꿈 설정           |


## ✅ 기본 읽기/쓰기 테스트

| 테스트 함수 이름               | 입출력 파일 경로 요약            |
|-------------------------------|----------------------------------|
| excel_read_writer_tests       | template.xlsx → output.xlsx     |
| excel_write_float_cell        | (다양한 셀 조작, 파일 저장 포함) |
| read_and_write_by_empty       | aaa_empty.xlsx → bbb_empty.xlsx |
| lazy_read_and_write_by_empty  | aaa_empty.xlsx → bbb_lazy_empty.xlsx |


## ✅ 셀 조작 및 포맷 테스트

| 테스트 함수 이름       | 주요 기능 요약                                |
|------------------------|-----------------------------------------------|
| read_and_wite_method   | 셀 값 설정/삭제, 포맷된 값 검증, 스타일 설정 |


## ✅ 이미지 처리 테스트

| 테스트 함수 이름 | 이미지 셀 위치 | 관련 메서드 호출             |
|------------------|----------------|-------------------------------|
| read_and_wite    | M17            | read_and_wite_method 포함     |


## ✅ 고급 기능 테스트 (XLSM 포함)

| 테스트 함수 이름           | 주요 기능 요약                                               |
|----------------------------|--------------------------------------------------------------|
| read_and_wite_xlsm_method | 시트 복사/삭제, 다양한 차트 삽입(Line, Pie, 3D 등), 이미지 삽입 |


## ✅ 전체 기능 커버리지

| 기능 영역                 | 테스트 여부 | 관련 테스트 함수 이름                     |
|--------------------------|-------------|-------------------------------------------|
| 셀 읽기/쓰기              | ✅           | new_file_and_edit, read_and_wite_method   |
| 셀 삭제                   | ✅           | read_and_wite_method                      |
| 셀 포맷 확인              | ✅           | read_and_wite_method                      |
| 수식 설정                 | ✅           | excel_write_float_cell                    |
| 날짜 입력                 | ✅           | excel_write_float_cell                    |
| 병합 셀                   | ✅           | excel_write_float_cell, new_file_and_edit |
| 스타일 적용               | ✅           | new_file_and_edit, new_sheet_and_edit     |
| 반복 입력                 | ✅           | excel_write_float_cell                    |
| 시트 생성/복사/삭제       | ✅           | read_and_wite_xlsm_method, duplicate_sheet |
| 이미지 삽입/교체/다운로드 | ✅           | read_and_wite, read_and_wite_xlsm_method  |
| 차트 삽입                 | ✅           | read_and_wite_xlsm_method                 |
| lazy_read 처리            | ✅           | lazy_read_and_write_by_empty, new_sheet_and_edit |
| 빈 파일 처리              | ✅           | read_and_write_by_empty, lazy_read_and_write_by_empty |
| 행/열 삽입 및 제거        | ✅           | insert_and_remove_cells                   |
| 스타일 범위 설정 및 검증  | ✅           | new_sheet_and_edit                        |
| CSV 저장                  | ✅           | writer_csv                                |
| 압축 저장                 | ✅           | compression_test                          |
| HTML → RichText 변환      | ✅           | html_to_rich_text_test                    |


## 📊 차트 삽입 요약 (Sheet7)

| 차트 종류        | 셀 범위     | 데이터 시트 | 시리즈 범위                  | 시리즈 수 |
|------------------|-------------|--------------|------------------------------|------------|
| LineChart        | A1 → B2     | New Sheet    | G7:G10, H7:H10               | 2          |
| PieChart         | B1 → C2     | New Sheet    | G7:G10, H7:H10               | 2          |
| DoughnutChart    | C1 → D2     | New Sheet    | G7:G10, H7:H10               | 2          |
| AreaChart        | D1 → E2     | New Sheet    | G7:G10, H7:H10               | 2          |
| BarChart         | E1 → F2     | New Sheet    | G7:G10, H7:H10               | 2          |
| Bar3DChart       | A2 → B3     | New Sheet    | G7:G10, H7:H10               | 2          |
| Line3DChart      | B2 → C3     | New Sheet    | G7:G10, H7:H10               | 2          |
| Pie3DChart       | C2 → D3     | New Sheet    | G7:G10, H7:H10               | 2          |
| Area3DChart      | D2 → E3     | New Sheet    | G7:G10, H7:H10               | 2          |
| OfPieChart       | E2 → F3     | New Sheet    | G7:G10, H7:H10               | 2          |
| BubbleChart      | A3 → B4     | New Sheet    | G7:G10, H7:H10, I7:I10       | 3          |
| RadarChart       | B3 → C4     | New Sheet    | G7:G10, H7:H10, I7:I10       | 3          |
| ScatterChart     | C3 → D4     | New Sheet    | G7:G10, H7:H10               | 2          |


## 📊 차트 삽입 기능 정리표 (Sheet7 기준)

| 차트 종류        | 셀 위치     | 데이터 시트 | 시리즈 범위                        | 시리즈 수 | 포인트 수 | 언어   | 타이틀 설정 |
|------------------|-------------|--------------|------------------------------------|------------|-------------|--------|--------------|
| LineChart        | A1 → B2     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| PieChart         | B1 → C2     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| DoughnutChart    | C1 → D2     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| AreaChart        | D1 → E2     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| BarChart         | E1 → F2     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| Bar3DChart       | A2 → B3     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| Line3DChart      | B2 → C3     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| Pie3DChart       | C2 → D3     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| Area3DChart      | D2 → E3     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| OfPieChart       | E2 → F3     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |
| BubbleChart      | A3 → B4     | New Sheet    | G7:G10, H7:H10, I7:I10             | 3          | 4           | ja-JP | ✅            |
| RadarChart       | B3 → C4     | New Sheet    | G7:G10, H7:H10, I7:I10             | 3          | 4           | ja-JP | ✅            |
| ScatterChart     | C3 → D4     | New Sheet    | G7:G10, H7:H10                     | 2          | 4           | ja-JP | ✅            |

- "A1" 셀 값 설정 및 검증
- 시트 복사 (New Sheet) 및 삭제 (DeletedSheet)
- "Sheet7"에 다양한 차트 삽입:
- LineChart
- PieChart
- DoughnutChart
- AreaChart
- BarChart
- Bar3DChart 

## 🖼 이미지 삽입 기능 요약

| 테스트 함수 이름       | 시트 이름     | 셀 위치 | 이미지 파일 경로       | 기능 설명     |
|------------------------|---------------|---------|------------------------|----------------|
| read_and_wite          | Sheet1        | M17     | asset/sample1.png      | 이미지 교체     |
| read_and_wite_xlsm_method | Sheet Image | B3      | asset/sample1.png      | 이미지 삽입     |

---

## 🧪 테스트 코드 기능 요약 및 설명

### 1. excel_read_writer_tests
```rust
#[test]
fn excel_read_writer_tests() {
    let path = Path::new("asset/template.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    sheet.get_cell_mut("A1").set_value("1234.0".to_string());
    writer::xlsx::write(&book, "asset/output.xlsx").unwrap();
}
```

### 2. excel_write_float_cell
```rust
#[test]
fn excel_write_float_cell() {
    // 1. 엑셀 파일 읽기
    // 1. 엑셀 파일 읽기
    let path = Path::new("asset/Template.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    // 2. 시트 가져오기
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    // 3. A1, B1 셀에 float 값을 문자열로 설정 (Excel 에서 숫자로 인식됨)
    sheet.get_cell_mut("A1").set_value("1234.56".to_string());
    sheet.get_cell_mut("B1").set_value("7890.12".to_string());

    // 4. 날짜 셀 입력 (Excel 이 날짜로 인식)
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

    let _sheet2 = book.get_sheet_by_name_mut("Sheet2").unwrap();
    let sheet_name = "MySheet";
    let _ = book.new_sheet(sheet_name).unwrap();
    let sheet = book.get_sheet_by_name_mut(sheet_name).unwrap();
    sheet.get_cell_mut("A1").set_value("Hello from MySheet");

    for sheet in book.get_sheet_collection() {
        println!("시트 이름: {}", sheet.get_name());
    }

    for sheet in book.get_sheet_collection_mut() {
        let name = sheet.get_name().to_string(); // 복사해서 소유권 획득
        sheet
            .get_cell_mut("G1")
            .set_value(format!("Hello from {}", name));
    }
    // 9. 엑셀 파일 저장
    let _ = writer::xlsx::write(&book, "asset/output.xlsx");
}

```
### 3. read_and_wite
```rust
fn read_and_wite_method(book: &mut umya_spreadsheet::Spreadsheet) {
    let _ = book
        .get_sheet_mut(&0)
        .unwrap()
        .get_cell_mut("A1")
        .set_value("TEST1");
    let a1_value = book.get_sheet(&0).unwrap().get_value("A1");
    assert_eq!("TEST1", a1_value);
    let _ = book.get_sheet_mut(&0).unwrap().remove_cell((&1, &1));
    let a1 = book.get_sheet(&0).unwrap().get_cell("A1");
    assert_eq!(a1, None);
    let _ = book.get_sheet_mut(&0).unwrap().remove_cell((&1, &2));
    let a2_value = book.get_sheet(&0).unwrap().get_value("A2");
    assert_eq!(a2_value, "");
    let b5_value = book.get_sheet(&0).unwrap().get_value("B5");
    assert_eq!(" ", b5_value);

    assert_eq!(
        "1.0000",
        book.get_sheet(&0).unwrap().get_formatted_value((&2, &20))
    );
    assert_eq!(
        "$3,333.0000",
        book.get_sheet(&0).unwrap().get_formatted_value("B21")
    );
    assert_eq!(
        "$ 333.00",
        book.get_sheet(&0).unwrap().get_formatted_value("B22")
    );
    assert_eq!(
        "2020年3月",
        book.get_sheet(&0).unwrap().get_formatted_value("B23")
    );
    assert_eq!(
        "2:33 pm",
        book.get_sheet(&0).unwrap().get_formatted_value("B24")
    );
    assert_eq!(
        "5.00%",
        book.get_sheet(&0).unwrap().get_formatted_value("B25")
    );
    assert_eq!(
        "1/2",
        book.get_sheet(&0).unwrap().get_formatted_value("B26")
    );
    assert_eq!(
        "12/15/2020 14:01",
        book.get_sheet(&0).unwrap().get_formatted_value("B27")
    );
    assert_eq!(
        "444",
        book.get_sheet(&0).unwrap().get_formatted_value("B28")
    );
    assert_eq!(
        "14-Dec-20",
        book.get_sheet(&0).unwrap().get_formatted_value("B29")
    );
    assert_eq!(
        "2020年10月1日",
        book.get_sheet(&0).unwrap().get_formatted_value("B30")
    );
    assert_eq!(
        "1.2345",
        book.get_sheet(&0).unwrap().get_formatted_value("B31")
    );
    assert_eq!(
        "1.2",
        book.get_sheet(&0).unwrap().get_formatted_value("B32")
    );
    assert_eq!(
        "12,345,675,544.00",
        book.get_sheet(&0).unwrap().get_formatted_value("B33")
    );
    assert_eq!(
        "1.235",
        book.get_sheet(&0).unwrap().get_formatted_value("B34")
    );
    assert_eq!("1", book.get_sheet(&0).unwrap().get_formatted_value("B35"));
    assert_eq!("", book.get_sheet(&0).unwrap().get_formatted_value("B36"));
    assert_eq!(
        "123456789012345678",
        book.get_sheet(&0).unwrap().get_formatted_value("B37")
    );

    let _ = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("49046881.119999997");

    let _ = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("A1")
        .get_number_format_mut()
        .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

    let value = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_formatted_value("A1");
    assert_eq!("49,046,881.12", &value);

    let fg = umya_spreadsheet::Color::default()
        .set_argb(umya_spreadsheet::Color::COLOR_BLACK)
        .to_owned();
    let fill = umya_spreadsheet::PatternFill::default()
        .set_foreground_color(fg)
        .to_owned();
    book.get_sheet_by_name_mut("Sheet5")
        .unwrap()
        .get_row_dimension_mut(&5u32)
        .get_style_mut()
        .get_fill_mut()
        .set_pattern_fill(fill);
    let font_color = umya_spreadsheet::Color::default()
        .set_argb(umya_spreadsheet::Color::COLOR_WHITE)
        .to_owned();
    book.get_sheet_by_name_mut("Sheet5")
        .unwrap()
        .get_row_dimension_mut(&5u32)
        .get_style_mut()
        .get_font_mut()
        .set_color(font_color);

    let _ = book
        .get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_cell_mut("A1")
        .get_style_mut()
        .get_font_mut()
        .set_name("Arial");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_row_dimension_mut(&3)
        .set_height(46.0);
}
```
```rust
#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("asset/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    read_and_wite_method(&mut book);

    book.get_sheet_by_name("Sheet1")
        .unwrap()
        .get_image("M17")
        .unwrap()
        .download_image("asset/bbb.png");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_image_mut("M17")
        .unwrap()
        .change_image("asset/sample1.png");

    // writer
    let path = std::path::Path::new("asset/bbb.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}
```
### 4. read_and_write_by_empty
```rust
#[test]
fn read_and_write_by_empty() {
    // reader
    let path = std::path::Path::new("asset/aaa_empty.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("asset/bbb_empty.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}
```

### 5. lazy_read_and_write_by_empty
```rust
#[test]
fn lazy_read_and_write_by_empty() {
    // reader
    let path = std::path::Path::new("asset/aaa_empty.xlsx");
    let book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();

    // writer
    let path = std::path::Path::new("asset/bbb_lazy_empty.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}
```

### ✅ 6. new_file_and_edit
- 새 엑셀 파일 생성 → 시트 추가 (Sheet2, Sheet3)
- 셀 값 설정: 문자열, 숫자, 불리언
- 셀 스타일 설정:
- 테두리 (set_border_style)
- 글자색 (set_argb)
- 배경색 (set_background_color, set_background_color_with_pattern)
- 열 너비 자동 조정 (set_auto_width)
- 셀 줄바꿈 (set_wrap_text)
- 폰트 크기 변경 (set_font_size_mut)
- 셀 병합 (add_merge_cells)
- 엑셀 파일 저장 (writer::xlsx::write)

### 소스 코드
```rust
#[test]
fn new_file_and_edit() {
    // new file.
    let mut book = umya_spreadsheet::new_file();

    // new worksheet.
    let _ = book.new_sheet("Sheet2");
    let _ = book.new_sheet("Sheet3");

    // change value.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("TEST1");
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell("A1")
        .unwrap()
        .get_value();
    assert_eq!("TEST1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((2, 2))
        .set_value_number(1);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((&2, &2))
        .unwrap()
        .get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((2, 2))
        .set_value_number(1);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((&2, &2))
        .unwrap()
        .get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((3, 3))
        .set_value_bool(true);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((&3, &3))
        .unwrap()
        .get_value();
    assert_eq!("TRUE", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((3, 3))
        .set_value_bool(true);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((&3, &3))
        .unwrap()
        .get_value();
    assert_eq!("TRUE", a1_value);

    // add bottom border.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .get_borders_mut()
        .get_bottom_mut()
        .set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut((&3, &2))
        .get_borders_mut()
        .get_left_mut()
        .set_border_style(umya_spreadsheet::Border::BORDER_THIN);

    // change font color.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .get_font_mut()
        .get_color_mut()
        .set_argb("00FF0000");

    // change background color.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .set_background_color(umya_spreadsheet::Color::COLOR_BLUE);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A2")
        .set_background_color_with_pattern(
            umya_spreadsheet::Color::COLOR_BLUE,
            umya_spreadsheet::Color::COLOR_RED,
            umya_spreadsheet::PatternValues::DarkGrid,
        );

    let worksheet = book.get_sheet_by_name_mut("Sheet3").unwrap();
    worksheet.get_column_dimension_mut("A").set_auto_width(true);

    worksheet.get_cell_mut("E1").set_value("テスト");
    worksheet.get_cell_mut("E2").set_value("うみゃーねっと");
    worksheet.get_cell_mut("E3").set_value("案案案案");
    worksheet.get_column_dimension_mut("E").set_auto_width(true);

    worksheet.get_cell_mut("F1").set_value("AAAAAAAAAAAAAAAAAA");
    worksheet.get_cell_mut("F2").set_value("BBBBBBBBBBB");
    worksheet
        .get_cell_mut("F4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("F").set_auto_width(true);

    worksheet.get_cell_mut("G1").set_value("AAAAAAAAAAAAAAAAAA");
    worksheet.get_cell_mut("G2").set_value("BBBBBBBBBBB");
    worksheet
        .get_cell_mut("G3")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("G").set_width(60f64);

    worksheet.get_cell_mut("D1").set_value("テスト");
    worksheet.get_cell_mut("D2").set_value("うみゃーねっと");
    worksheet.get_cell_mut("D3").set_value("案案案案");
    worksheet.get_column_dimension_mut("D").set_auto_width(true);

    worksheet.get_cell_mut("H1").set_value("テスト");
    worksheet
        .get_cell_mut("H2")
        .set_value("うみゃーねっと\nうみゃーねっと")
        .get_style_mut()
        .get_alignment_mut()
        .set_wrap_text(true);
    worksheet.get_cell_mut("H3").set_value("案案案案");
    worksheet.get_column_dimension_mut("H").set_auto_width(true);

    worksheet.get_cell_mut("I1").set_value("テスト");
    worksheet
        .get_cell_mut("I2")
        .set_value("うみゃーねっと")
        .get_style_mut()
        .get_font_mut()
        .get_font_size_mut()
        .set_val(20f64);
    worksheet.get_cell_mut("I3").set_value("案案案案");
    worksheet.get_column_dimension_mut("I").set_auto_width(true);

    worksheet
        .get_cell_mut("J2")
        .set_value("うみゃーねっと")
        .get_style_mut()
        .get_font_mut()
        .get_font_size_mut()
        .set_val(5f64);
    worksheet.get_column_dimension_mut("J").set_auto_width(true);

    worksheet
        .get_cell_mut("K4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("K").set_auto_width(true);

    worksheet
        .get_cell_mut("L4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("L").set_auto_width(true);

    worksheet
        .get_cell_mut("M4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("M").set_auto_width(true);

    worksheet
        .get_cell_mut("N1")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("N").set_auto_width(true);

    worksheet.add_merge_cells("K8:L8");
    worksheet.add_merge_cells("M8:M10");
    worksheet.add_merge_cells("N:N");

    // writer.
    let path = std::path::Path::new("asset/eee.xlsx");
    umya_spreadsheet::writer::xlsx::write(&book, path).unwrap();
}
```

### ✅ 7. new_and_write
- 빈 엑셀 파일 생성 후 저장
- 최소 작업 흐름 테스트

### 소스 코드
```rust
#[test]
fn new_and_write() {
    // new file.
    let book = umya_spreadsheet::new_file();

    // writer.
    let path = std::path::Path::new("asset/fff.xlsx");
    umya_spreadsheet::writer::xlsx::write(&book, path).unwrap();
}
```

### ✅ 8. duplicate_sheet
- 동일한 이름의 시트 생성 시 오류 발생 확인
- 시트 이름 중복 방지 테스트

### 소스 코드
```rust
#[test]
fn duplicate_sheet() {
    let mut book = umya_spreadsheet::new_file();
    let _ = book.new_sheet("Sheet2");
    if book.new_sheet("Sheet2").is_ok() {
        panic!("getting new sheet..")
    }
}
```

### ✅ 9. writer_csv
- CSV 파일로 저장
- Shift-JIS 인코딩, trim, 따옴표 감싸기 옵션 설정
- CSV 출력 테스트

### 소스 코드
```rust
#[test]
fn writer_csv() {
    let mut book = umya_spreadsheet::new_file();
    book.set_active_sheet(1);
    let sheet = book.new_sheet("Sheet2").unwrap();
    // ---
    sheet.get_cell_mut("A1").set_value(" TEST");
    sheet.get_cell_mut("B1").set_value("あいうえお");
    sheet.get_cell_mut("C1").set_value("漢字");
    sheet.get_cell_mut("E1").set_value("1");
    // ---
    sheet.get_cell_mut("A2").set_value("TEST ");
    sheet.get_cell_mut("B2").set_value("あいうえお");
    sheet.get_cell_mut("C2").set_value("漢字");
    // ---
    sheet.get_cell_mut("A3").set_value(" TEST ");
    // ---

    // writer
    let mut option = umya_spreadsheet::structs::CsvWriterOption::default();
    option.set_csv_encode_value(umya_spreadsheet::structs::CsvEncodeValues::ShiftJis);
    option.set_do_trim(true);
    option.set_wrap_with_char("\"");
    let path = std::path::Path::new("asset/bbb.csv");
    let _ = umya_spreadsheet::writer::csv::write(&book, path, Some(&option));
}
```

### ✅ 10. compression_test
- .xlsx 파일 읽기 후 두 가지 압축 방식으로 저장:
- write() → 표준 압축
- write_light() → 경량 압축
- 파일 크기 최적화 테스트

### 소스 코드
```rust
#[test]
fn compression_test() {
    // reader
    let path = std::path::Path::new("asset/aaa.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("asset/bbb_comp_standard.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);

    // writer
    let path = std::path::Path::new("asset/bbb_comp_light.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write_light(&book, path);
}
```

### ✅ 11. html_to_rich_text_test
- HTML 문자열을 RichText로 변환 (html_to_richtext)
- 셀에 리치 텍스트 삽입 (set_rich_text)
- 줄바꿈 설정 (set_wrap_text)
- HTML → Excel 스타일 변환 테스트

### 소스 코드
```rust
#[test]
fn html_to_rich_text_test() {
    let path = std::path::Path::new("asset/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    let html = r##"<font color="red">test</font><br><font class="test" color="#48D1CC">TE<b>S</b>T<br/>TEST</font>"##;
    let rich_text = umya_spreadsheet::helper::html::html_to_richtext(html).unwrap();

    sheet.get_cell_mut("G16").set_rich_text(rich_text);
    sheet
        .get_cell_mut("G16")
        .get_style_mut()
        .get_alignment_mut()
        .set_wrap_text(true);

    let path = std::path::Path::new("asset/bbb_html_to_rich_text.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}
```

## 🧪 고급 기능 설명
### 🧷 12. read_and_write_xlsm
- .xlsm 파일 읽기 및 저장
- read_and_wite_xlsm_method 호출로 다양한 차트 삽입 포함
- 차트 종류: Line, Pie, Doughnut, Area, Bar, 3D, Bubble, Radar, Scatter 등
- 이미지 삽입도 포함

### 소스 코드
```rust
fn read_and_wite_xlsm_method(book: &mut umya_spreadsheet::Spreadsheet) {
    let _ = book
        .get_sheet_mut(&0)
        .unwrap()
        .get_cell_mut((1, 1))
        .set_value("TEST1");
    let a1_value = book
        .get_sheet(&0)
        .unwrap()
        .get_cell((&1, &1))
        .unwrap()
        .get_value();
    assert_eq!("TEST1", a1_value);

    // copy sheet
    let mut clone_sheet = book.get_sheet(&0).unwrap().clone();
    clone_sheet.set_name("New Sheet");
    let _ = book.add_sheet(clone_sheet);

    // remove sheet
    let mut clone_sheet = book.get_sheet(&0).unwrap().clone();
    clone_sheet.set_name("DeletedSheet");
    let _ = book.add_sheet(clone_sheet);
    book.get_sheet_by_name("DeletedSheet").unwrap();
    book.remove_sheet_by_name("DeletedSheet").unwrap();

    // add chart (line chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A1");
    to_marker.set_coordinate("B2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::LineChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title")
        .set_grouping(umya_spreadsheet::drawing::charts::GroupingValues::Standard);
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (pie chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B1");
    to_marker.set_coordinate("C2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::PieChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (doughnut chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C1");
    to_marker.set_coordinate("D2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::DoughnutChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (area chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D1");
    to_marker.set_coordinate("E2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::AreaChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (bar chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E1");
    to_marker.set_coordinate("F2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::BarChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (bar 3d chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A2");
    to_marker.set_coordinate("B3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::Bar3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (line 3d chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B2");
    to_marker.set_coordinate("C3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::Line3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (pie 3d chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C2");
    to_marker.set_coordinate("D3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::Pie3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (area 3d chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D2");
    to_marker.set_coordinate("E3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::Area3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (of pie chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E2");
    to_marker.set_coordinate("F3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::OfPieChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (bubble chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A3");
    to_marker.set_coordinate("B4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
        "New Sheet!$I$7:$I$10",
    ];
    let series_title_list = vec!["Line1", "Line2", "Line3"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::BubbleChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (radar chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B3");
    to_marker.set_coordinate("C4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
        "New Sheet!$I$7:$I$10",
    ];
    let series_title_list = vec!["Line1", "Line2", "Line3"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::RadarChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (scatter chart)
    let mut from_marker =
        umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C3");
    to_marker.set_coordinate("D4");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart
        .new_chart(
            umya_spreadsheet::structs::ChartType::ScatterChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // Add Image
    let _ = book.new_sheet("Sheet Image");
    let mut marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    marker.set_coordinate("B3");
    let mut image = umya_spreadsheet::structs::Image::default();
    image.new_image("asset/sample1.png", marker);
    book.get_sheet_by_name_mut("Sheet Image")
        .unwrap()
        .add_image(image);
}
```

```rust
#[test]
fn read_and_write_xlsm() {
    // reader
    let path = std::path::Path::new("asset/aaa.xlsm");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    read_and_wite_xlsm_method(&mut book);

    // writer
    let path = std::path::Path::new("asset/bbb.xlsm");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}
```


### 🧷 13. insert_and_remove_cells
- "Sheet1"에 행/열 삽입:
- insert_new_row, insert_new_column, insert_new_column_by_index
- 행/열 제거:
- remove_row, remove_column_by_index
- 셀 구조 변경 테스트

### 소스 코드
```rust
#[test]
fn insert_and_remove_cells() {
    // reader
    let path = std::path::Path::new("asset/aaa_insertCell.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    book.insert_new_row("Sheet1", &2, &3);
    book.insert_new_column("Sheet1", "B", &3);
    book.insert_new_column_by_index("Sheet1", &2, &3);

    book.remove_row("Sheet1", &6, &2);
    book.remove_column_by_index("Sheet1", &6, &2);

    // writer
    let path = std::path::Path::new("asset/bbb_insertCell.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}
```
### 🧷 14. new_sheet_and_edit
- "Sheet2233" 시트 생성
- "A2" 셀에 값 설정
- "A3:A4" 범위에 배경색 스타일 적용
- 저장 후 다시 읽어와 값과 스타일 검증
- 스타일 범위 적용 및 검증 테스트

### 소스 코드
```rust
#[test]
fn new_sheet_and_edit() {
    const BG_COLOR: &str = "#333";
    const TEST_SHEET: &str = "Sheet2233";

    let path = std::path::Path::new("asset/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();

    // set cell value
    let sheet = book.new_sheet(TEST_SHEET).unwrap();
    let cell = sheet.get_cell_mut("A2");
    let _ = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", style);

    let path = std::path::Path::new("asset/bbb_new_sheet_value.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);

    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .get_sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .get_cell("A2")
        .unwrap()
        .get_value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A3")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb();

        assert_eq!(a3_bg, BG_COLOR);
    }

    {
        let a4_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A4")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb();

        assert_eq!(a4_bg, BG_COLOR);
    }
}
```
