# JaggedArray
크레이트는 Rust에서 가변 길이의 2차원 배열(즉, 행마다 길이가 다른 배열)을 다루기 위한 라이브러리입니다.  
Cargo.toml 설정과 사용법은 간단하며, 장단점도 명확하게 구분됩니다.

## 🛠️ Cargo.toml 설정 방법
```
[dependencies]
jagged_array = "0.2"
``

- crates.io에 등록된 버전 "0.2"를 사용
- 최신 버전은 JaggedArray on crates.io에서 확인 가능

## 🚀 기본 사용법
```rust
use jagged_array::Jagged2D;

fn main() {
    let mut matrix = Jagged2D::new();
    matrix.push_row(vec![1, 2, 3]);
    matrix.push_row(vec![4, 5]);
    matrix.push_row(vec![6]);

    println!("{:?}", matrix.get(1)); // Some(&[4, 5])
    println!("{:?}", matrix.get_cell(0, 2)); // Some(&3)
}
```

## 주요 메서드
| 메서드              | 설명                                 |
|---------------------|--------------------------------------|
| new()               | 빈 Jagged2D 배열 생성                |
| push_row(Vec<T>)    | 새로운 행을 추가                     |
| get(row)            | 특정 행을 가져옴 (`Option<&[T]>`)    |
| get_cell(row, col)  | 특정 셀을 가져옴 (`Option<&T>`)      |
| rows()              | 전체 행 개수를 반환 (`usize`)        |

## ✅ 장점
- 가변 길이 행 지원: 각 행마다 다른 길이의 데이터를 저장 가능
- 간단한 API: Vec<Vec<T>>보다 명확한 추상화 제공
- Rust 안전성 유지: Option 반환으로 안전한 접근 보장
- 유용한 유틸리티 메서드: 행/셀 접근, 반복자 등 제공

## ⚠️ 단점
| 항목              | 설명                                      |
|-------------------|-------------------------------------------|
| Vec<Vec<T>> 기반  | 내부적으로 중첩 벡터 사용 → 메모리 단편화 가능성 있음 |
| 고정 크기 배열과 호환 어려움 | `Matrix<T, ROW, COL>` 같은 구조와 직접 연산 불가 |
| 수학 연산 미지원   | `add`, `mul` 등 연산은 직접 구현해야 함         |
| 타입 안전성 제한   | 행마다 길이가 달라서 컴파일 타임에 크기 보장 불가 |


## 🧩 언제 쓰면 좋을까?
- CSV, 로그, 텍스트 테이블처럼 행마다 열 개수가 다른 데이터를 다룰 때
- Vec<Vec<T>>를 직접 쓰기엔 너무 복잡하거나 반복되는 패턴이 많을 때
- 고정 크기 배열이 아닌 유연한 구조가 필요한 경우


---

