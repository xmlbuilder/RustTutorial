# ASCII 문자열 Slice
이 Rust 함수 substr_ascii는 문자열을 ASCII 기준으로 부분 추출하는 기능을 수행합니다.  
Fortran에서 비슷한 작업을 할 때와 비교해서 설명.

## 🦀 Rust 함수 설명: substr_ascii
```rust
fn substr_ascii(s: &str, offset: usize, len: usize) -> String {
    if offset >= s.len() {
        return String::new();
    }
    let end = (offset + len).min(s.len());
    s[offset..end].to_string()
}
```

## 🔍 기능 요약

| 항목              | 설명                                           |
|-------------------|------------------------------------------------|
| `s: &str`         | 입력 문자열 (UTF-8 기반 슬라이스)              |
| `offset`          | 추출 시작 위치 (바이트 기준 인덱스)            |
| `len`             | 추출할 길이 (바이트 수 기준)                   |
| `s[offset..end]`  | 슬라이스 문법으로 부분 문자열 추출             |
| `.to_string()`    | 슬라이스를 `String` 타입으로 변환               |

## ✅ 경계 조건 처리
- offset >= s.len()이면 빈 문자열 반환 → 범위 초과 방지
- `end = min(offset + len, s.len())` → 끝 범위 자동 조절

## 🧪 Fortran에서 비슷한 작업
Fortran에서는 문자열이 고정 길이 문자 배열로 처리되며, 부분 문자열 추출은 다음과 같이 합니다:
```fortran
character(len=100) :: line
character(len=20) :: subline
integer :: offset, len

offset = 10
len = 20
subline = line(offset:offset+len-1)
```

## 🔍 차이점 비교: Rust vs Fortran 문자열 처리

| 항목             | &str (Rust)             | character(len=...) (Fortran)     |
|------------------|--------------------------|----------------------------------|
| 문자열 타입      | UTF-8 문자열             | 고정 길이 문자 배열              |
| 인덱스 기준      | 바이트 기준              | 문자 기준                        |
| 경계 처리 방식   | `min()`으로 안전 처리    | 직접 인덱스 계산 필요            |
| 슬라이스 문법    | `s[a..b]`                | `s(a:b)`                         |

### ⚠️ 주의: Rust는 UTF-8 기반
- Rust의 &str은 UTF-8이기 때문에, 한글이나 멀티바이트 문자가 포함된 경우 offset이 문자 단위가 아니라 바이트 단위로 동작합니다.
    - 이 함수는 ASCII 문자열에만 안전하게 동작합니다.
### ✅ 해결 방법 (Unicode-safe 추출)
```rust
fn substr_unicode(s: &str, offset: usize, len: usize) -> String {
    s.chars().skip(offset).take(len).collect()
}
```

## ✨ 결론
- Rust의 substr_ascii()는 Fortran의 line(i:j)와 유사한 역할을 하며, ASCII 문자열에 대해 안전하게 부분 추출을 수행합니다.
- Fortran은 문자 단위 인덱싱이 기본이고, Rust는 바이트 단위이므로 멀티바이트 문자 처리 시 주의가 필요합니다.
- Unicode-safe 처리를 원한다면 chars().skip().take() 방식으로 구현해야 합니다.

---


---

