# Zip / Unzip
Rust에서 ZIP 파일을 다루는 내용을 문서화해서 정리.  
프로젝트 내에서 참고할 수 있는 기술 문서 형태로 구성했습니다.

## 📚 ZIP 유틸리티 문서
### 1. 사용 Crate
- zip
  - ZIP 아카이브 생성 및 해제 지원
  - 다양한 압축 방식 지원 (Stored, Deflate, Bzip2, Zstd 등)
- walkdir
  - 디렉토리 순회용 (전체 디렉토리 압축 시 사용)
```
[dependencies]
zip = "0.6"
walkdir = "2.5"
```
## 2. 주요 함수
### (1) 디렉토리 전체 압축
```rust
pub fn zip_directory(src_dir: &Path, dst_file: &Path) -> zip::result::ZipResult<()>
```
  - 입력: 원본 디렉토리 경로, 결과 ZIP 파일 경로
  - 출력: ZIP 파일 생성
  - 동작: walkdir로 디렉토리 순회 → 파일은 ZipWriter::start_file로 추가 → 디렉토리는 add_directory로 추가

### (2) ZIP 파일 해제
```rust
pub fn unzip_file(zip_file: &Path, dst_dir: &Path) -> zip::result::ZipResult<()>
```
  - 입력: ZIP 파일 경로, 출력 디렉토리 경로
  - 출력: ZIP 파일 해제
  - 동작: ZipArchive로 ZIP 열기 → 각 엔트리 추출 → 디렉토리 생성 및 파일 복사

### (3) 단일 파일 ZIP → 내용 읽기
```rust
pub fn read_zip_content(zip_path: &Path) -> zip::result::ZipResult<String>
```
  - 입력: ZIP 파일 경로
  - 출력: 첫 번째 파일의 문자열 내용
  - 동작: ZipArchive::by_index(0) → read_to_string

### (4) 문자열 → 단일 파일 ZIP 저장
```rust
pub fn write_zip_content(zip_path: &Path, filename: &str, contents: &str) -> zip::result::ZipResult<()>
```
  
  - 입력: ZIP 파일 경로, 파일명, 문자열 내용
  - 출력: ZIP 파일 생성
  - 동작: ZipWriter::start_file → write_all(contents.as_bytes())

---

## 3. 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_single_file_zip() {
        let zip_path = Path::new("test.zip");
        let content = "Hello JungHwan!";
        write_zip_content(zip_path, "hello.txt", content).unwrap();

        let read_content = read_zip_content(zip_path).unwrap();
        assert_eq!(content, read_content);

        fs::remove_file(zip_path).unwrap();
    }
```
```rust
    #[test]
    fn test_directory_zip_unzip() {
        let src_dir = Path::new("example_dir");
        let zip_path = Path::new("example.zip");
        let dst_dir = Path::new("unzipped_dir");

        // 디렉토리 압축
        zip_directory(src_dir, zip_path).unwrap();

        // 압축 해제
        unzip_file(zip_path, dst_dir).unwrap();

        // 결과 확인 (예: 특정 파일 존재 여부)
        assert!(dst_dir.join("some_file.txt").exists());

        fs::remove_file(zip_path).unwrap();
        fs::remove_dir_all(dst_dir).unwrap();
    }
}
```

## 4. 활용 시나리오
- 설정 파일 백업: 프로젝트 설정 디렉토리를 ZIP으로 묶어 저장.
- 리소스 배포: 이미지/모델 파일을 ZIP으로 묶어 배포 후 해제.
- 로그 아카이빙: 로그 디렉토리를 주기적으로 ZIP으로 압축.

## ✅ 정리
- zip_directory / unzip_file: 디렉토리 전체 압축/해제
- write_zip_content / read_zip_content: 단일 파일 ZIP 처리
- 테스트 코드로 기능 검증 가능
- GUI/서버 환경에서 파일 업로드/다운로드, 백업, 배포에 활용 가능

---

## 소스 코드
```rust
use std::fs::{File, create_dir_all};
use std::io::{Write, BufWriter, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::{ZipWriter, ZipArchive};
use zip::write::FileOptions;

/// 디렉토리 전체를 ZIP 으로 묶기
pub fn zip_directory(src_dir: &Path, dst_file: &Path) -> zip::result::ZipResult<()> {
    let file = File::create(dst_file)?;
    let buf = BufWriter::new(file);
    let mut zip = ZipWriter::new(buf);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(src_dir).unwrap();

        if path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() {
            // 디렉토리 엔트리 추가
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    Ok(())
}
```
```rust
/// ZIP 파일을 디렉토리로 풀기
pub fn unzip_file(zip_file: &Path, dst_dir: &Path) -> zip::result::ZipResult<()> {
    let file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dst_dir.join(file.name());

        if file.name().ends_with('/') {
            create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
```
```rust
/// ZIP 파일에서 단일 파일 내용 읽기
pub fn read_zip_content(zip_path: &Path) -> zip::result::ZipResult<String> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    if archive.len() == 0 {
        return Ok(String::new());
    }

    // 첫 번째 엔트리만 읽음
    let mut zip_file = archive.by_index(0)?;
    let mut contents = String::new();
    zip_file.read_to_string(&mut contents)?;
    Ok(contents)
}
```
```rust
/// content 를 ZIP 파일로 저장하기 (단일 파일)
pub fn write_zip_content(zip_path: &Path, filename: &str, contents: &str) -> zip::result::ZipResult<()> {
    let file = File::create(zip_path)?;
    let buf = BufWriter::new(file);
    let mut zip = ZipWriter::new(buf);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    zip.start_file(filename, options)?;
    zip.write_all(contents.as_bytes())?;
    zip.finish()?;
    Ok(())
}
```
---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{Write, BufWriter};
    use zip::write::FileOptions;
    use std::io::Read;
    use zip::ZipArchive;

    fn zip() -> zip::result::ZipResult<()> {
        let file = File::create("asset/archive.zip")?;
        let buf = BufWriter::new(file);
        let mut zip = zip::ZipWriter::new(buf);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("hello.txt", options)?;
        zip.write_all(b"Hello, JungHwan!")?;

        zip.finish()?;
        Ok(())
    }
```
```rust
    #[test]
    fn test_zip() {

        zip().expect("zip")
    }

    fn unzip() -> zip::result::ZipResult<()> {
        let file = File::open("asset/archive.zip")?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            println!("Filename: {}", file.name());

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("Contents: {}", contents);
        }
        Ok(())
    }
```
```rust
    #[test]
    fn test_unzip() {

        unzip().unwrap();
    }

}
```

#[cfg(test)]
mod tests_zip_unzip {

    use std::fs;
    use std::path::Path;
    use nurbslib::core::zip_utils::{read_zip_content, write_zip_content};

    #[test]
    fn test_write_and_read_zip_content() {
        let zip_path = Path::new("asset/test_single.zip");

        // 1. ZIP 파일 생성
        let content = "Hello JungHwan, this is a test!";
        let filename = "hello.txt";
        write_zip_content(zip_path, filename, content).expect("Failed to write zip");

        // 2. ZIP 파일에서 내용 읽기
        let read_content = read_zip_content(zip_path).expect("Failed to read zip");

        // 3. 내용 검증
        assert_eq!(content, read_content);

        // 4. 테스트 후 파일 삭제
        fs::remove_file(zip_path).unwrap();
    }
```
```rust
    #[test]
    fn test_empty_zip_content() {
        let zip_path = Path::new("asset/empty.zip");

        // 빈 내용 저장
        write_zip_content(zip_path, "empty.txt", "").expect("Failed to write empty zip");

        // 읽기
        let read_content = read_zip_content(zip_path).expect("Failed to read empty zip");

        // 검증
        assert_eq!(read_content, "");

        fs::remove_file(zip_path).unwrap();
    }
```
```rust
    #[test]
    fn test_multiple_calls() {
        let zip_path = Path::new("asset/multi.zip");

        // 첫 번째 내용 저장
        write_zip_content(zip_path, "file1.txt", "First content").expect("Failed to write zip");

        // 읽기
        let read_content1 = read_zip_content(zip_path).expect("Failed to read zip");
        assert_eq!(read_content1, "First content");

        // 두 번째 내용 덮어쓰기
        write_zip_content(zip_path, "file2.txt", "Second content").expect("Failed to overwrite zip");

        // 읽기
        let read_content2 = read_zip_content(zip_path).expect("Failed to read zip");
        assert_eq!(read_content2, "Second content");

        fs::remove_file(zip_path).unwrap();
    }
}
```
