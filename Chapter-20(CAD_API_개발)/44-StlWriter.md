# STLWriter
이 코드는 3D 메시 데이터를 STL 파일 형식(ASCII 또는 Binary)으로 저장하는 기능을 제공합니다.

## 📦 STL 파일 작성기 (StlWriter)
이 모듈은 Mesh 구조체를 기반으로 STL 파일을 생성하는 기능을 제공합니다.  
STL은 3D 모델을 저장하는 데 사용되는 형식으로, 주로 3D 프린팅에 사용됩니다.

## 📘 구조체 정의
```rust
pub struct StlWriter {
    path: String,
    binary: bool,
}
```
- path: 생성할 STL 파일의 경로
- binary: true일 경우 Binary STL 형식으로, false일 경우 ASCII STL 형식으로 저장

## 🧩 함수 설명
### 🔧 StlWriter::run
```rust
pub fn run(path: &str, mesh: &Mesh, binary: bool) -> Result<()>
```

- 설명: 주어진 경로와 메시 데이터를 기반으로 STL 파일을 생성합니다. binary 플래그에 따라 ASCII 또는 Binary 형식으로 저장합니다.
- 매개변수:
  - path: 저장할 파일 경로
  - mesh: STL로 변환할 메시 데이터
  - binary: true면 Binary STL, false면 ASCII STL
- 반환값: 파일 생성 성공 여부를 나타내는 Result<()>

### 🏗️ StlWriter::new
```rust
pub fn new(path: &str, binary: bool) -> Result<Self>
```
- 설명: 새로운 StlWriter 인스턴스를 생성합니다.
- 매개변수:
  - path: STL 파일 경로
  - binary: 저장 형식 선택
- 반환값: StlWriter 인스턴스

## ✏️ StlWriter::run_ascii
```rust
pub fn run_ascii(&mut self, mesh: &Mesh) -> Result<()>
```

- 설명: 메시 데이터를 ASCII STL 형식으로 저장합니다.
- 동작:
  - STL 파일을 생성하고 solid 블록을 시작
  - 각 면(face)에 대해 법선 벡터(normal)를 계산
  - 각 꼭짓점(vertex)을 facet 블록에 작성
  - endsolid으로 종료
- 반환값: 파일 작성 성공 여부

## 💾 StlWriter::run_binary
```rust
pub fn run_binary(&mut self, mesh: &Mesh) -> Result<()>
```

- 설명: 메시 데이터를 Binary STL 형식으로 저장합니다.
- 동작:
- 80바이트 헤더 작성
- 면(face) 개수 기록
- 각 면에 대해:
  - 법선 벡터(normal) 기록
  - 세 꼭짓점(vertex) 좌표 기록
  - 속성 바이트 수(0) 기록
  - 반환값: 파일 작성 성공 여부

## 📌 참고 사항
- mesh.faces는 각 면을 구성하는 정점 인덱스 배열입니다 (예: [0, 1, 2])
- mesh.vertices는 정점 좌표를 담고 있는 배열입니다
- cross_pt와 unitize는 벡터 연산을 위한 메서드로, 외적 및 단위 벡터 변환을 수행합니다
- Binary STL은 공간 효율이 높고 빠르지만, ASCII STL은 사람이 읽기 쉬운 형식입니다

---
## 소스 코드
```rust
use std::fs::File;
use std::io::{Result, Write};
use crate::core::mesh::Mesh;

pub struct StlWriter {
    path: String,
    binary: bool,
}
```
```rust
impl StlWriter {
    pub fn run(path: &str, mesh: &Mesh, binary: bool) -> Result<()> {
        let mut writer = StlWriter::new(path, binary)?;
        match writer.binary {
            true => { Ok(writer.run_binary(mesh)?) },
            false => { Ok(writer.run_ascii(mesh)?) },
        }
    }
}
```
```rust
impl StlWriter {
    pub fn new(path: &str, binary: bool) -> Result<Self> {
        Ok(Self {
            path: path.to_string(),
            binary,
        })
    }

    pub fn run_ascii(&mut self, mesh: &Mesh) -> Result<()> {
        let mut file = File::create(&self.path)?;
        writeln!(file, "solid mesh")?;

        for face in &mesh.faces {
            let v0 = mesh.vertices[face[0] as usize];
            let v1 = mesh.vertices[face[1] as usize];
            let v2 = mesh.vertices[face[2] as usize];
            let n = (v1 - v0).cross_pt(&(v2 - v0)).unitize();

            writeln!(file, "  facet normal {} {} {}", n.x, n.y, n.z)?;
            writeln!(file, "    outer loop")?;
            writeln!(file, "      vertex {} {} {}", v0.x, v0.y, v0.z)?;
            writeln!(file, "      vertex {} {} {}", v1.x, v1.y, v1.z)?;
            writeln!(file, "      vertex {} {} {}", v2.x, v2.y, v2.z)?;
            writeln!(file, "    endloop")?;
            writeln!(file, "  endfacet")?;
        }
        writeln!(file, "endsolid mesh")?;
        Ok(())
    }
```
```rust
    pub fn run_binary(&mut self, mesh: &Mesh) -> Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};

        let mut file = File::create(&self.path)?;
        let header = [0u8; 80];
        file.write_all(&header)?;
        file.write_u32::<LittleEndian>(mesh.faces.len() as u32)?;

        for face in &mesh.faces {
            let v0 = mesh.vertices[face[0] as usize];
            let v1 = mesh.vertices[face[1] as usize];
            let v2 = mesh.vertices[face[2] as usize];
            let n = (v1 - v0).cross_pt(&(v2 - v0)).unitize();

            for val in &[n.x, n.y, n.z] {
                file.write_f32::<LittleEndian>(*val as f32)?;
            }
            for v in &[v0, v1, v2] {
                file.write_f32::<LittleEndian>(v.x as f32)?;
                file.write_f32::<LittleEndian>(v.y as f32)?;
                file.write_f32::<LittleEndian>(v.z as f32)?;
            }
            file.write_u16::<LittleEndian>(0)?; // attribute byte count
        }
        Ok(())
    }
}
```
---

## 테스트 코드 
```rust
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use nurbslib::core::mesh::Mesh;
    use nurbslib::core::prelude::Point3D;
    use nurbslib::core::stl_writer::StlWriter;

    fn create_test_mesh() -> Mesh {
        Mesh {
            vertices: vec![
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 1.0, y: 0.0, z: 0.0 },
                Point3D { x: 0.0, y: 1.0, z: 0.0 },
            ],
            faces: vec![[0, 1, 2, 2]],
            normals: None,
        }
    }
```
```rust
    #[test]
    fn test_ascii_stl_writer() {
        let mesh = create_test_mesh();
        let path = "test_ascii.stl";
        let result = StlWriter::run(path, &mesh, false);
        assert!(result.is_ok());
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }
```
```rust
    #[test]
    fn test_binary_stl_writer() {
        let mesh = create_test_mesh();
        let path = "test_binary.stl";
        let result = StlWriter::run(path, &mesh, true);
        assert!(result.is_ok());
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }
}
```
---


