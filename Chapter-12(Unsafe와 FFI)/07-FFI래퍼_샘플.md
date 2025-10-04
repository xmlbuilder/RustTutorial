#  rust와 C 함수 연동

## 샘플 코드
```rust
mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "매크로"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // 불투명 타입입니다. https://doc.rust-lang.org/nomicon/ffi.html을 참고하세요.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // readdir(3)의 Linux man 페이지에 따른 레이아웃입니다.
    // 여기서 ino_t 및 off_t는
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}의 정의에 따라 확인됩니다.
    #[cfg(not(target_os = "매크로"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // dir(5)의 macOS man 페이지에 따른 레이아웃입니다.
    #[cfg(all(target_os = "매크로"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "매크로", target_arch = "x86_64")))]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        // https://github.com/rust-lang/libc/issues/414 및
 // stat(2)에 관한 macOS man 페이지의 _DARWIN_FEATURE_64_BIT_INODE 섹션을 참고하세요.
 //
 // ' 이 업데이트가 제공되기 전에 존재했던 플랫폼은'
 // Intel 및 PowerPC의 macOS (iOS/wearOS 등이 아님)를 의미합니다.
        #[cfg(all(target_os = "매크로", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // opendir을 호출하고 제대로 작동하면 Ok 값을 반환하고
        // 그렇지 않으면 메시지와 함께 Err을 반환합니다.
        let path =
            CString::new(path).map_err(|err| format!("잘못된 경로: {err}"))?;
        // SAFETY: path.as_ptr()은 NULL일 수 없습니다.
        let dir = unsafe { ffi::opendir(path.as_ptr()) };
        if dir.is_null() {
            Err(format!("{:?}을(를) 열 수 없습니다.", path))
        } else {
            Ok(DirectoryIterator { path, dir })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // NULL 포인터를 다시 얻을 때까지 readdir을 계속 호출합니다.
        // SAFETY: self.dir은 NULL이 아닙니다.
        let dirent = unsafe { ffi::readdir(self.dir) };
        if dirent.is_null() {
            // 디렉터리의 끝에 도달했습니다.
            return None;
        }
        // SAFETY: dirent는 NULL이 아니며 dirent.d_name은 NUL
        // 종료됩니다.
        let d_name = unsafe { CStr::from_ptr((*dirent).d_name.as_ptr()) };
        let os_str = OsStr::from_bytes(d_name.to_bytes());
        Some(os_str.to_owned())
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // 필요에 따라 closedir을 호출합니다.
        if !self.dir.is_null() {
            // SAFETY: self.dir은 NULL이 아닙니다.
            if unsafe { ffi::closedir(self.dir) } != 0 {
                panic!("{:?}을(를) 닫을 수 없습니다.", self.path);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("파일: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_nonexisting_directory() {
        let iter = DirectoryIterator::new("no-such-directory");
        assert!(iter.is_err());
    }

    #[test]
    fn test_empty_directory() -> Result<(), Box<dyn Error>> {
        let tmp = tempfile::TempDir::new()?;
        let iter = DirectoryIterator::new(
            tmp.path().to_str().ok_or("경로에 UTF-8이 아닌 문자가 있음")?,
        )?;
        let mut entries = iter.collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, &[".", ".."]);
        Ok(())
    }

    #[test]
    fn test_nonempty_directory() -> Result<(), Box<dyn Error>> {
        let tmp = tempfile::TempDir::new()?;
        std::fs::write(tmp.path().join("foo.txt"), "Foo 다이어리\n")?;
        std::fs::write(tmp.path().join("bar.png"), "<PNG>\n")?;
        std::fs::write(tmp.path().join("crab.rs"), "//! Crab\n")?;
        let iter = DirectoryIterator::new(
            tmp.path().to_str().ok_or("경로에 UTF-8이 아닌 문자가 있음")?,
        )?;
        let mut entries = iter.collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, &[".", "..", "bar.png", "crab.rs", "foo.txt"]);
        Ok(())
    }
}
```

이 예제는 Rust에서 C의 디렉토리 관련 함수(opendir, readdir, closedir)를 FFI로 안전하게 감싸는 구조를 보여주는 자료입니다.
아래에 전체 내용을 체계적으로 정리해드릴게요.

## 🧩 목적: C의 디렉토리 함수들을 Rust에서 안전하게 사용하기
Rust는 FFI(Foreign Function Interface)를 통해 C 함수를 호출할 수 있습니다.
이 예제에서는 C의 디렉토리 함수들:
- opendir(const char*) → DIR*
- readdir(DIR*) → dirent*
- closedir(DIR*) → int
을 Rust에서 안전하게 감싸는 래퍼를 구현합니다.

## 🧵 문자열 타입 정리 (std::ffi)

| 타입              | 인코딩 방식 | 용도                          | 주요 변환 흐름 예시                          |
|-------------------|-------------|-------------------------------|----------------------------------------------|
| `str`, `String`   | UTF-8       | Rust 내부 문자열 처리         | `String::from("abc")` → `&str`               |
| `CStr`, `CString` | NUL 종료    | C 함수와 연동                 | `CString::new("abc")` → `.as_ptr()`          |
| `OsStr`, `OsString` | OS 정의   | OS 경로, 파일 이름 등과 연동 | `OsStr::from_bytes(&[u8])` → `.to_os_string()` |

## 🔄 주요 변환 흐름 요약
&str
  ↓ CString::new()
CString
  ↓ .as_ptr() → *const c_char
*const c_char
  ↓ CStr::from_ptr()
&CStr
  ↓ .to_bytes()
&[u8]
  ↓ OsStrExt::from_bytes()
&OsStr
  ↓ .to_os_string()
OsString

- CString::new(str) → NUL 종료 문자열 생성
- .as_ptr() → C 함수에 넘길 포인터
- CStr::from_ptr(ptr) → C에서 받은 포인터를 안전하게 감싸기
- CStr.to_bytes() → 바이트 슬라이스로 변환
- OsStrExt::from_bytes() → OS 문자열로 변환
- OsStr.to_os_string() → 복사해서 Rust에서 사용

## 🧩 FFI 모듈 구조
```rust
mod ffi {
    // C 타입 정의
    use std::os::raw::{c_char, c_int, c_long, c_uchar, c_ulong, c_ushort};

    // DIR 구조체: 불투명 타입
    #[repr(C)]
    pub struct DIR { ... }

    // dirent 구조체: 플랫폼별 레이아웃
    #[repr(C)]
    pub struct dirent { ... }

    // 외부 C 함수 선언
    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

```
- #[repr(C)] → C ABI와 호환되도록 구조체 정렬
- extern "C" → C 함수 호출을 위한 선언

## 🧭 DirectoryIterator 구조
```rust
#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}
```

- path: 디렉토리 경로를 C 문자열로 저장
- dir: opendir()로 얻은 DIR 포인터

## 🔧 구현해야 할 메서드들
### DirectoryIterator::new(path: &str)
- CString::new(path) → C 문자열로 변환
- opendir(c_str.as_ptr()) 호출
- 실패 시 Err(String) 반환
### Iterator::next()
- readdir(self.dir) 호출
- NULL이면 None 반환
- dirent.d_name → CStr → OsStr → OsString 변환
### Drop::drop()
- closedir(self.dir) 호출

## ✅ 최종 사용 예시
```rust
fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    for name in iter {
        println!("파일: {:?}", name);
    }
    Ok(())
}
```
- 현재 디렉토리의 파일 목록을 출력
- Rust의 Iterator 트레이트를 통해 idiomatic하게 사용
