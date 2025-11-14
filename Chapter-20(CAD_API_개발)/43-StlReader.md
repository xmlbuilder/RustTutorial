# StlReader

StlReader 구조체의 기능을 문서화한 내용입니다.  
STL 파일의 바이너리/ASCII 포맷을 자동 판별하고, 삼각형 면 정보를 추출하여 Mesh 구조에 구축하는 전체 흐름을 정리.

# 📦 StlReader 문서화

## 🧩 개요

`StlReader`는 STL 파일을 읽어 `Mesh` 객체로 변환하는 유틸리티입니다.  
STL 포맷은 3D 모델링에서 널리 사용되며, ASCII 또는 Binary 형식으로 저장됩니다.  
이 리더는 자동으로 포맷을 판별하고, 삼각형 면 정보를 추출하여 정점 및 면(face) 정보를 구성합니다.

---

## 🧠 주요 기능

| 메서드 이름           | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| `run(path, mesh)`      | STL 파일을 열고 포맷을 판별한 뒤, 적절한 파서(`read_binary` 또는 `read_ascii`) 호출 |
| `read_binary(path, mesh)` | Binary STL 파일을 파싱하여 삼각형 면 정보를 추출                    |
| `read_ascii(path, mesh)`  | ASCII STL 파일을 파싱하여 삼각형 면 정보를 추출                     |
| `build_mesh(mesh, raw_facets)` | 중복 정점 제거 및 면 구성, 정점 인덱스 매핑, 법선 벡터 계산 수행 |

---

## 📂 파일 포맷 판별

```rust
let is_binary = header.iter().any(|&b| b > 127);
```

- STL 파일의 첫 128바이트를 검사하여 바이너리 여부를 판별
- ASCII는 일반적으로 ASCII 문자만 포함되며, 바이너리는 확장 바이트가 포함됨

## 🧩 Binary STL 파싱 흐름
- 80바이트 헤더 스킵
- 삼각형 개수 읽기 (u32)
- 각 삼각형에 대해:
- 법선 벡터 (3 × f32)
- 3개의 정점 (3 × 3 × f32)
- 속성 바이트 수 (u16, 무시)
- raw_facets에 저장

🧩 ASCII STL 파싱 흐름
- 줄 단위로 읽기
- vertex 키워드가 있는 줄에서 좌표 추출
- endfacet 키워드가 나오면 3개의 정점이 모였는지 확인 후 저장

## 🧱 Mesh 구축 로직 (build_mesh)

| 단계                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| BoundingBox 계산      | 모든 정점의 경계 박스를 계산하여 정밀도 기준(tol)을 설정              |
| `spatial_map` 생성    | 좌표를 정규화하여 근접 정점을 빠르게 탐색하기 위한 해시 맵 생성       |
| 중복 정점 제거        | tol 기준으로 기존 정점과 유사한지 검사하여 중복 제거 및 인덱스 재사용 |
| `face[3] = face[2]`   | STL은 삼각형이지만 내부적으로 4개로 확장 (쿼드 호환 또는 정렬 목적)   |
| `mesh.vertices` 확장  | 새로 생성된 정점을 기존 `mesh`에 병합 (오프셋 적용)                   |
| `mesh.faces` 구성     | 정점 인덱스를 기반으로 면(face) 추가                                 |
| `mesh.compute_normals()` | 모든 면에 대해 법선 벡터 자동 계산                                 |

## 📌 정점 중복 제거 방식
```rust
let key = [
    (p[0] / tol).round() as i32,
    (p[1] / tol).round() as i32,
    (p[2] / tol).round() as i32,
];
```

- tol = sqrt(EPSILON) × bounding box diagonal
- 좌표를 정규화하여 근접한 정점을 동일 키로 간주
- spatial_map을 통해 빠른 중복 탐색 및 인덱스 재사용

## ✅ 사용 예시
```rust
let mut mesh = Mesh::new();
StlReader::run("model.stl", &mut mesh)?;
```
- STL 파일을 읽고 mesh.vertices, mesh.faces가 채워짐
- 자동으로 법선 벡터까지 계산됨

---

## 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::mesh::Mesh;
    use nurbslib::core::stl_reader::StlReader;
    use nurbslib::core::stl_writer::StlWriter;

    #[test]
    fn test_stl_read_write_cycle() {
        let input_path = "asset/test_input.stl";
        let output_path = "asset/test_output.stl";

        // 1. STL 내용 작성
        let stl_content = r#"
solid triangle
  facet normal 0 0 1
    outer loop
      vertex 0 0 0
      vertex 1 0 0
      vertex 0 1 0
    endloop
  endfacet
endsolid triangle
"#;

        // 2. 입력 파일 생성
        std::fs::write(input_path, stl_content).unwrap();

        // 3. 읽기
        let mut mesh = Mesh::default(); // 또는 Mesh::new(vec![], vec![])
        StlReader::run(input_path, &mut mesh).unwrap();

        // 4. 쓰기
        StlWriter::run(output_path, &mesh, false).unwrap();

        // 5. 다시 읽기
        let mut mesh2 = Mesh::default();
        StlReader::run(output_path, &mut mesh2).unwrap();

        // 6. 검증
        assert_eq!(mesh.vertices.len(), mesh2.vertices.len());
        assert_eq!(mesh.faces.len(), mesh2.faces.len());

        println!("vertices: {}, faces: {}", mesh.vertices.len(), mesh2.faces.len());

        // 7. 정리
        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }
}
```

---
