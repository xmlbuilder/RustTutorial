# DynaKeyContainer

아래는 DynaKeyContainer 구조체에 대한 문서입니다.  
Dyna Key 파일을 섹션 단위로 파싱하고 관리하는 역할을 명확하게 설명.

## 📦 DynaKeyContainer 문서
### 🧠 개요
DynaKeyContainer는 LS-DYNA 키워드 기반 입력 파일(Dyna Key format)을 섹션 단위로 파싱하고 저장하는 컨테이너입니다.  
각 섹션은 `*KEYWORD` 로 시작하며, 해당 키워드에 대한 설정 또는 데이터를 포함합니다.  
이 구조는 키워드별로 내용을 분리하고, 후속 파싱 및 분석을 위한 기반을 제공합니다.

## 🧱 구조 정의
```rust
pub struct DynaSection {
    pub keyword: String,   // 예: "*MAT_037"
    pub content: String,   // 키워드 이후의 모든 줄 (빈 줄 포함, *로 시작하는 줄 제외)
}
```

```rust
pub struct DynaKeyContainer {
    pub sections: Vec<DynaSection>, // 전체 섹션 목록
}
```

## 🔧 주요 기능

| 메서드 이름         | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| `from_file(path)`      | Dyna Key 파일을 읽고 키워드(*...) 단위로 섹션을 분리하여 저장합니다. |
| `find_by_keyword(key)` | 주어진 키워드 이름으로 섹션을 검색하여 Vec로 반환합니다.             |
| `count_by_keyword(key)`| 해당 키워드 이름의 섹션 개수를 반환합니다.                           |


## 📄 파싱 규칙
- *로 시작하는 줄은 키워드로 인식 → DynaSection.keyword
- 다음 키워드 전까지의 모든 줄은 해당 섹션의 내용 → DynaSection.content
- content에는 빈 줄도 포함됨
- content에는 *로 시작하는 줄이 절대 포함되지 않음
- 마지막 줄의 불필요한 \n은 제거하여 빈 줄로 오인되지 않도록 처리

## 🧪 사용 예시
```rust
let container = DynaKeyContainer::from_file("example.k")?;

for section in &container.sections {
    println!("Keyword: {}", section.keyword);
    println!("Content:\n{}", section.content);
}
```

```rust
let container = DynaKeyContainer::from_file("example.k")?;
let mat_sections = container.find_by_keyword("*MAT_037");

for section in mat_sections {
    println!("Found section:\n{}", section.content);
}
```

```rust
let container = DynaKeyContainer::from_file("example.k")?;
let mat_count = container.count_by_keyword("*MAT_037");
println!("*MAT_037 섹션 개수: {}", mat_count);

let curves = container.find_by_keyword("*DEFINE_CURVE");
println!("*DEFINE_CURVE 섹션 개수: {}", curves.len());

```

## 🔗 TextParser 연동
각 섹션의 content는 TextParser에 넘겨서 필드 단위로 분석할 수 있습니다:
```rust
let mut parser = TextParser::new();
parser.set_text(&section.content);

while let Some(line) = parser.valid_next_line() {
    let value = parser.psr_get_float(10, 0.0);
    // ...
}
```

## ✅ 요약

| 구성 요소        | 역할 설명                                           |
|------------------|----------------------------------------------------|
| *KEYWORD         | 섹션의 시작을 나타내며, DynaSection.keyword로 저장 |
| content          | 키워드 이후의 모든 줄 (빈 줄 포함, *로 시작하는 줄 제외) |
| DynaSection      | keyword + content를 하나의 섹션으로 구성           |
| DynaKeyContainer | 전체 섹션을 Vec<DynaSection>로 보관                |
| TextParser       | 각 섹션의 content를 줄/필드 단위로 분석            |

---
## 전체 소스
```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// 하나의 Dyna Key 섹션
#[derive(Debug)]
pub struct DynaSection {
    pub keyword: String,
    pub content: String,
}
```
```rust
/// 전체 Dyna Key 파일 컨테이너
#[derive(Debug, Default)]
pub struct DynaKeyContainer {
    pub sections: Vec<DynaSection>,
}
```
```rust
impl DynaKeyContainer {
    /// 파일 경로로부터 Dyna Key 파일을 파싱
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut container = DynaKeyContainer::default();
        let mut current_keyword: Option<String> = None;
        let mut current_content = String::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim_end();

            if trimmed.starts_with('*') {
                // 이전 섹션 저장
                if let Some(keyword) = current_keyword.take() {
                    let content = current_content.strip_suffix('\n').unwrap_or(&current_content).to_string();
                    container.sections.push(DynaSection {
                        keyword,
                        content,
                    });
                    current_content.clear();
                }
                // 새 키워드 시작
                current_keyword = Some(trimmed.to_string());
            } else {
                // 키워드가 시작된 이후의 모든 줄을 content에 저장
                current_content.push_str(trimmed);
                current_content.push('\n');
            }
        }

        // 마지막 섹션 저장
        if let Some(keyword) = current_keyword {
            container.sections.push(DynaSection {
                keyword,
                content: current_content,
            });
        }

        Ok(container)
    }

    pub fn find_by_keyword(&self, key: &str) -> Vec<&DynaSection> {
        self.sections
            .iter()
            .filter(|section| section.keyword == key)
            .collect()
    }

    pub fn count_by_keyword(&self, key: &str) -> usize {
        self.sections
            .iter()
            .filter(|section| section.keyword == key)
            .count()
    }
}
```
```rust
// 향후 추가할 구조
/*
match section.keyword.as_str() {
    "*MAT_037" => parse_mat_037(&section.content),
    "*DEFINE_CURVE" => parse_define_curve(&section.content),
    "*END" => {}, // 무시
    _ => println!("Unhandled keyword: {}", section.keyword),
}

struct Mat037 {
    mid: i32,
    ro: f32,
    e: f32,
    pr: f32,
    sigy: f32,
    etan: f32,
    r: f32,
    hlcid: i32,
}
*/
```
---





