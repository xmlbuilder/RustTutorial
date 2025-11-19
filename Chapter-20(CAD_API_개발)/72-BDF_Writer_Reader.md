# BdfWriter / BdfReader

아래는 BdfWriter와 BdfReader 구조에 대한 정리된 문서화입니다.  
각 구조체의 목적, 주요 메서드, 동작 방식, 필드 설명, 확장 포인트까지 체계적으로 정리.

## 📘 BDF 입출력 구조 문서
### 📤 BdfWriter
### ✅ 목적
BdfWriter는 하나 이상의 Mesh 객체를 받아 NASTRAN BDF 형식의 텍스트 파일로 출력하는 구조체입니다.  
주로 구조 해석용 전처리 데이터를 생성하는 데 사용됩니다.

### 🧱 주요 필드

| 필드명 | 타입   | 설명                          |
|--------|--------|-------------------------------|
| path   | String | 출력할 `.bdf` 파일 경로       |


### 🔧 주요 메서드 표

| 메서드명              | 반환 타입     | 설명                                                                 |
|-----------------------|---------------|----------------------------------------------------------------------|
| `new(path: &str)`     | `Result<Self>`| 지정된 경로로 BdfWriter 인스턴스를 생성                             |
| `run(&mut self, &[Mesh])` | `Result<()>` | 전체 BDF 파일을 생성하고 정점 및 요소 데이터를 출력                  |
| `write_header(&mut File)` | `Result<()>` | 해석 조건 및 BEGIN BULK 블록 출력                                   |
| `write_grid_cards(&mut File, &Mesh, node_offset)` | `Result<()>` | 각 정점에 대해 `GRID` 카드 출력                                     |
| `write_ctria3_cards(&mut File, &Mesh, node_offset, elem_offset)` | `Result<()>` | 각 삼각형 요소에 대해 `CTRIA3` 카드 출력                            |

### 🔧 주요 메서드
- new(path: &str) -> Result<Self>
    - 주어진 경로로 BdfWriter 인스턴스를 생성
- run(&mut self, meshes: &[Mesh]) -> Result<()>
    - 전체 BDF 파일을 생성
    - 내부적으로 다음 단계로 구성됨:
    - write_header() – 해석 설정 및 BEGIN BULK
    - write_grid_cards() – 정점(GRID) 카드 출력
    - write_ctria3_cards() – 삼각형 요소(CTRIA3) 카드 출력
    - ENDDATA 출력
- write_header(file: &mut File) -> Result<()>
    - BDF 파일의 해석 조건 및 BULK 시작부를 출력
- write_grid_cards(file, mesh, node_offset)
    - 각 정점에 대해 GRID 카드 출력
    - 좌표는 on_format_field_field8()로 8자리 고정폭 포맷
- write_ctria3_cards(file, mesh, node_offset, elem_offset)
    - 각 삼각형 면에 대해 CTRIA3 카드 출력

## 🧩 확장 포인트
- CQUAD4, PSHELL, MAT1 등 다른 카드 추가 시 write_*_cards() 함수 추가
- 좌표 포맷을 16자리로 확장하려면 on_format_field_field16() 도입

## 소스 코드
```rust
use std::fs::File;
use std::io::{Result, Write};
use crate::core::file_utils::on_format_field_field8;
use crate::core::mesh::Mesh;

pub struct BdfWriter {
    path: String,
}
```
```rust
impl BdfWriter {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self { path: path.to_string() })
    }
```
```rust
    pub fn run(&mut self, meshes: &[Mesh]) -> Result<()> {
        let mut file = File::create(&self.path)?;
        self.write_header(&mut file)?;

        let mut node_offset = 1;
        let mut elem_offset = 1;

        for mesh in meshes {
            self.write_grid_cards(&mut file, mesh, node_offset)?;
            self.write_ctria3_cards(&mut file, mesh, node_offset, elem_offset)?;
            node_offset += mesh.vertices.len() as u32;
            elem_offset += mesh.faces.len() as u32;
        }

        writeln!(file, "ENDDATA")?;
        Ok(())
    }
```
```rust
    fn write_header(&self, file: &mut File) -> Result<()> {
        writeln!(file, "ASSIGN UNIT=12, FORM=FORMATTED")?;
        writeln!(file, "SOL 101")?;
        writeln!(file, "CEND")?;
        writeln!(file, "SEALL = ALL")?;
        writeln!(file, "SUPER = ALL")?;
        writeln!(file, "TITLE = BDF file")?;
        writeln!(file, "ECHO = NONE")?;
        writeln!(file, "SUBCASE 1")?;
        writeln!(file, "SUBTITLE=Geometry Data")?;
        writeln!(file, "DISPLACEMENT(SORT1,REAL)=ALL")?;
        writeln!(file, "SPCFORCES(SORT1,REAL)=ALL")?;
        writeln!(file, "STRESS(SORT1,REAL,VONMISES,BILIN)=ALL")?;
        writeln!(file, "BEGIN BULK")?;
        writeln!(file, "PARAM   POST    -1")?;
        writeln!(file, "PARAM   AUTOSPC YES")?;
        writeln!(file, "PARAM   PRTMAXIM YES")?;
        Ok(())
    }
```
```rust
    fn write_grid_cards(&self, file: &mut File, mesh: &Mesh, node_offset: u32) -> Result<()> {
        for (i, pt) in mesh.vertices.iter().enumerate() {
            let id = node_offset + i as u32;
            writeln!(
                file,
                "{:<8}{:>8}{:>8}{:>8}{:>8}{:>8}",
                "GRID",
                id,
                "",
                on_format_field_field8(pt.x),
                on_format_field_field8(pt.y),
                on_format_field_field8(pt.z)
            )?;
        }
        Ok(())
    }
```
```rust
    fn write_ctria3_cards(&self, file: &mut File, mesh: &Mesh, node_offset: u32, elem_offset: u32) -> Result<()> {
        for (i, face) in mesh.faces.iter().enumerate() {
            let id = elem_offset + i as u32;
            let v1 = node_offset + face[0];
            let v2 = node_offset + face[1];
            let v3 = node_offset + face[2];
            writeln!(
                file,
                "{:<8}{:>8}{:>8}{:>8}{:>8}{:>8}",
                "CTRIA3",
                id,
                1,
                v1,
                v2,
                v3
            )?;
        }
        Ok(())
    }
}
```

---

### 📥 BdfReader
### ✅ 목적
BdfReader는 .bdf 파일을 읽어 Mesh 구조체로 파싱합니다.  
주로 해석 결과 후처리나 시각화를 위한 전처리 단계로 사용됩니다.  

### 🧱 주요 필드
| 필드명 | 타입   | 설명                          |
|--------|--------|-------------------------------|
| path   | String | 입력할 `.bdf` 파일 경로       |

### 🔧 주요 메서드 표

| 메서드명              | 반환 타입     | 설명                                                                 |
|-----------------------|---------------|----------------------------------------------------------------------|
| `new(path: &str)`     | `Result<Self>`| 지정된 경로로 BdfReader 인스턴스를 생성                             |
| `run(&mut self, &mut Mesh)` | `Result<()>` | `.bdf` 파일을 읽어 `Mesh`에 정점 및 요소 데이터를 채움               |
| `parse_grid(&str, &mut Mesh, &mut HashMap)` | `Result<()>` | 일반 `GRID` 카드 파싱                                               |
| `parse_grid_long(&mut Peekable<Lines>, &mut Mesh, &mut HashMap)` | `Result<()>` | 연장된 `GRID*` 카드 파싱 (다음 줄까지 읽음)                         |
| `parse_ctria3(&str, &mut Mesh, &HashMap)` | `Result<()>` | 삼각형 요소 `CTRIA3` 카드 파싱 및 정점 인덱스 매핑                  |


### 🔧 주요 메서드 정리
- new(path: &str) -> Result<Self>
    - 주어진 경로로 BdfReader 인스턴스를 생성
- run(&mut self, mesh: &mut Mesh) -> Result<()>
    - .bdf 파일을 읽어 Mesh에 정점과 면을 채움
    - 내부적으로 다음 단계로 구성됨:
    - parse_grid() – 일반 GRID 카드 파싱
    - parse_grid_long() – 연장된 GRID* 카드 파싱
    - parse_ctria3() – CTRIA3 카드 파싱
    - mesh.compute_normals() 호출
- parse_grid(line, mesh, node_map)
    - 8자리 필드 기반 GRID 카드 파싱
-parse_grid_long(lines, mesh, node_map)
    - 16자리 필드 기반 GRID* 카드 파싱 (다음 줄까지 읽음)
-parse_ctria3(line, mesh, node_map)
    - 삼각형 요소 CTRIA3 파싱 및 정점 인덱스 매핑

## 🧩 확장 포인트
- CQUAD4, CHEXA, PSHELL 등 요소 추가 시 parse_*() 함수 확장
- GRID* 외에도 CORD2R, MAT1, PBAR 등 지원 가능
- node_map을 외부로 노출하면 ID 기반 매핑 유지 가능

## 🧪 테스트 및 검증 전략
- Round-trip 테스트: Mesh → BdfWriter → BdfReader → Mesh 비교
- 좌표 정밀도 테스트: 포맷터와 파서 간 오차 검증
- 카드 누락/오류 처리: 잘못된 입력에 대한 견고성 테스트

## ✅ 결론
이 구조는 다음과 같은 장점을 갖습니다:
- 단일 책임 원칙 준수: Writer/Reader 분리
- 서브 함수화로 유지보수 용이
- 확장성 확보: 카드별 함수 분리로 기능 추가 간편
- 실전 적용 가능성 높음: BDF 포맷 요구사항 충족

## 소스 코드
```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use crate::core::geom::Point3D;
use crate::core::mesh::Mesh;

pub struct BdfReader {
    path: String,
}
```
```rust
impl BdfReader {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            path: path.to_string(),
        })
    }
```
```rust
    pub fn run(&mut self, mesh: &mut Mesh) -> Result<()> {
        let file = BufReader::new(File::open(&self.path)?);
        let mut node_map: HashMap<u32, u32> = HashMap::new();
        let mut lines = file.lines().peekable();

        while let Some(line) = lines.next() {
            let line = line?;
            let card = line.get(0..8).unwrap_or("").trim();

            match card {
                "GRID" => self.parse_grid(&line, mesh, &mut node_map)?,
                "GRID*" => self.parse_grid_long(&line, &mut lines, mesh, &mut node_map)?,
                "CTRIA3" => self.parse_ctria3(&line, mesh, &node_map)?,
                _ => {}
            }
        }
        mesh.compute_normals();
        Ok(())
    }
```
```rust
    fn parse_grid(&self, line: &str, mesh: &mut Mesh, node_map: &mut HashMap<u32, u32>) -> Result<()> {

        let id = parse_field(&line, 8, 16);
        let x = parse_field(&line, 24, 32);
        let y = parse_field(&line, 32, 40);
        let z = parse_field(&line, 40, 48);

        if let (Some(id), Some(x), Some(y), Some(z)) = (id, x, y, z) {
            let pt = Point3D::new(x, y, z);
            node_map.insert(id, mesh.vertices.len() as u32);
            mesh.vertices.push(pt);
        }
        Ok(())
    }
```
```rust
    fn parse_grid_long(&self, line: &str, lines: &mut std::iter::Peekable<Lines<BufReader<File>>>, mesh: &mut Mesh, node_map: &mut HashMap<u32, u32>) -> Result<()> {

        let id = parse_field(&line, 8, 24);
        let x = parse_field(&line, 40, 56);
        let y = parse_field(&line, 56, 72);

        // 다음 줄에서 z 좌표 추출
        let z = lines
            .peek()
            .and_then(|l| l.as_ref().ok())
            .and_then(|l| parse_field(l, 8, 24));

        if let (Some(id), Some(x), Some(y), Some(z)) = (id, x, y, z) {
            let pt = Point3D::new(x, y, z);
            node_map.insert(id, mesh.vertices.len() as u32);
            mesh.vertices.push(pt);
            lines.next(); // consume z-line
        }
        Ok(())
    }
```
```rust
    fn parse_ctria3(&self, line: &str, mesh: &mut Mesh, node_map: &HashMap<u32, u32>) -> Result<()> {
        let n1 = parse_field(&line, 24, 32);
        let n2 = parse_field(&line, 32, 40);
        let n3 = parse_field(&line, 40, 48);
        if let (Some(id1), Some(id2), Some(id3)) = (n1, n2, n3) {
            if let (Some(v1), Some(v2), Some(v3)) =
                (node_map.get(&id1), node_map.get(&id2), node_map.get(&id3))
            {
                mesh.faces.push([*v1, *v2, *v3, *v3]);
            }
        }
        Ok(())
    }
}
```
```rust
fn parse_field<T: std::str::FromStr>(line: &str, start: usize, end: usize) -> Option<T> {
    line.get(start..end)?.trim().parse().ok()
}
```
---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::bdf_reader::BdfReader;
    use nurbslib::core::bdf_writer::BdfWriter;
    use nurbslib::core::mesh::Mesh;

    #[test]
    fn test_bdf_read_write() {
        let mut mesh = Mesh::default();


        let mut reader = BdfReader::new("asset/input.bdf").unwrap();
        reader.run(&mut mesh).expect("TODO: panic message");


        println!("{:?}", mesh);


        let mut writer = BdfWriter::new("asset/output.bdf").unwrap();
        writer.run(&vec![mesh]).expect("run failed");
    }
}
```

---



