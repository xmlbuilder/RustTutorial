# Matrix 연산시 발생 하는 소유권 문제 극복
Matrix 연산은 동시 접근, 가변 참조, slice 분할, unsafe 우회 같은 Rust의 메모리 모델과 정면으로 부딪히는 영역이라  
확실한 정리와 기법의 체계화가 필수.

## 🧠 왜 Matrix 연산에서 이런 일이 빈번한가
- 행/열 단위로 접근 → &mut row[i], &mut col[j] 등
- 부분 행렬 추출 → split_at_mut, chunks_mut, windows_mut
- 동시 연산 → A[i][j] += B[i][j] 같은 병렬적 구조
- 성능 최적화 → unsafe pointer 접근이 유혹됨
- 라이브러리 연동 → nalgebra, ndarray, cgmath 등에서 내부 참조가 많음

## ✅ Matrix 연산에서 안전하게 빌릴 수 있는 기법들
| 기법               | 안전성 수준 | 설명                                                                 |
|--------------------|-------------|----------------------------------------------------------------------|
| `split_at_mut()`   | ✅ 안전      | slice를 겹치지 않는 두 부분으로 나눠서 각각 `&mut` 참조 가능                |
| `chunks_mut()`     | ✅ 안전      | 고정 크기 블록으로 나눠서 병렬 연산 가능. 겹치지 않는 영역을 순회           |
| `windows_mut()`    | ✅ 안전      | 슬라이딩 윈도우 방식으로 겹치지 않는 영역을 순차적으로 접근 가능            |
| `RefCell<Matrix>`  | ⚠️ 런타임 체크 | 내부 가변성. 컴파일 타임엔 허용되지만 중복 borrow 시 panic 발생             |
| `raw pointer`      | ❌ unsafe    | `*mut T`로 우회. 메모리 충돌 시 UB 발생 가능. 반드시 `unsafe` 블록 필요     |
| `scoped 접근`      | ✅ 안전      | 스코프를 분리하면 컴파일러가 “동시 접근 아님”으로 판단해 허용               |
| `split_array_mut()`| ✅ 안전      | 배열을 고정 크기 튜플로 분할해 각 요소를 독립적으로 수정 가능 (Rust 1.50+) |

### 🔧 예시: `split_at_mut` 로 행렬 분할
```rust
fn update_matrix(matrix: &mut [[f64; 4]; 4]) {
    let (top, bottom) = matrix.split_at_mut(2);
    top[0][0] += 1.0;
    bottom[1][3] -= 2.0;
}
```

- `split_at_mut()` 로 행 단위 분할
- 각 영역은 겹치지 않으므로 &mut 참조 허용

### 🔥 고급 전략
- Matrix를 flat Vec으로 표현 → Vec<f64> + manual indexing
- Matrix를 Rc<RefCell<>>로 감싸기 → 공유 + 내부 가변성
- Matrix를 unsafe로 접근하되 영역 분리 보장 → 고성능 연산 시

## 💬 결론
Matrix 연산은 Rust의 메모리 모델과 가장 민감하게 충돌하는 영역입니다.  
그래서 안전한 빌림 기법을 정리해두는 것 자체가  
Rust에서 수학적 설계를 할 수 있는 기반이 됩니다.  

---

# Matrix borrow 총 정리

아래에 Matrix 연산에서 안전하게 빌릴 수 있는 기법들에 대해 각각의 샘플 코드를 모두 정리.

## ✅ 1. split_at_mut() — slice를 겹치지 않게 나누기
```rust
fn split_at_mut_example() {
    let mut matrix = [1, 2, 3, 4, 5, 6];
    let (left, right) = matrix.split_at_mut(3); //[1, 2, 3] , [4, 5, 6]

    left[0] += 10;
    right[2] += 20;

    println!("Result: {:?}", matrix); // [11, 2, 3, 4, 5, 26]
}
```

## ✅ 2. chunks_mut() — 고정 크기 블록으로 나누기
```rust
fn chunks_mut_example() {
    let mut matrix = [1, 2, 3, 4, 5, 6]; 

    for chunk in matrix.chunks_mut(2) { //[1, 2], [3, 4], [5, 6]
        chunk[0] *= 2;
    }

    println!("Result: {:?}", matrix); // [2, 2, 6, 4, 10, 6]
}
```


## ✅ 3. windows_mut() — 슬라이딩 윈도우 방식
```rust
fn windows_mut_example() {
    let mut matrix = [1, 2, 3, 4, 5];

    for window in matrix.windows_mut(2) { 
        window[0] += window[1];
    }

    println!("Result: {:?}", matrix); // [3, 5, 7, 9, 5]
}
```


---

### 🔍 코드 분석

#### 🧠 핵심 개념: windows_mut(2)
- windows_mut(2)는 길이 2짜리 슬라이딩 윈도우를 만들어서  
겹치면서 순차적으로 배열을 순회합니다.
- 반환되는 window는 [i, i+1]에 해당하는 가변 참조 슬라이스입니다.

#### 🔄 단계별 실행 흐름 — 초기 배열: [1, 2, 3, 4, 5]
| 반복 | window     | 연산 수행                       | 배열 변화 후           |
|------|------------|----------------------------------|------------------------|
| 1    | [1, 2]     | window[0] += window[1] → 1+2=3   | [3, 2, 3, 4, 5]        |
| 2    | [2, 3]     | window[0] += window[1] → 2+3=5   | [3, 5, 3, 4, 5]        |
| 3    | [3, 4]     | window[0] += window[1] → 3+4=7   | [3, 5, 7, 4, 5]        |
| 4    | [4, 5]     | window[0] += window[1] → 4+5=9   | [3, 5, 7, 9, 5]        |

- 마지막 요소 5는 윈도우의 끝이라서 window[1]로 쓰이지 않음, 그대로 유지됨

#### 📌 주의할 점
- windows_mut()은 겹치는 슬라이스를 가변 참조로 반환하므로
→ 내부 요소를 직접 수정할 수 있음
- 하지만 동시에 여러 요소를 수정하면 겹침으로 인해 버그 발생 가능
→ 이 예제는 window[0]만 수정하므로 안전

#### 💡 요약
- windows_mut(n)은 [i, i+1, ..., i+n-1] 슬라이스를 반복적으로 반환
- 각 슬라이스는 원본 배열의 가변 참조이므로 직접 수정 가능
- 겹치는 구조이므로 수정 대상이 겹치지 않도록 주의 필요
---


## ⚠️ 4. RefCell<Matrix> — 내부 가변성
```rust
use std::cell::RefCell;
fn refcell_example() {
    let matrix = RefCell::new(vec![1, 2, 3]);

    {
        let mut borrow = matrix.borrow_mut();
        borrow[0] += 10;
    }

    {
        let borrow = matrix.borrow();
        println!("Result: {:?}", borrow); // [11, 2, 3]
    }
}
```


## ❌ 5. raw pointer — unsafe 접근
```rust
fn raw_pointer_example() {
    let mut matrix = [1, 2, 3];
    let p1 = &mut matrix[0] as *mut i32;
    let p2 = &mut matrix[2] as *mut i32;

    unsafe {
        *p1 += 100;
        *p2 += 200;
    }

    println!("Result: {:?}", matrix); // [101, 2, 203]
}
```


## ✅ 6. scoped 접근 — 스코프 분리로 컴파일러 설득
```rust
fn scoped_access_example() {
    let mut matrix = [1, 2, 3];

    {
        let a = &mut matrix[0];
        *a += 10;
    }

    {
        let b = &mut matrix[2];
        *b += 20;
    }

    println!("Result: {:?}", matrix); // [11, 2, 23]
}

```

## ✅ 7. split_array_mut() — 배열을 튜플로 분할 (Rust 1.50+)
```rust
fn split_array_mut_example() {
    let matrix = &mut [1, 2, 3, 4];
    let [a, b, c, d] = matrix else { panic!("Wrong size") };

    *a += 1;
    *d += 2;

    println!("Result: {:?}", matrix); // [2, 2, 3, 6]
}
```

## 💬 결론
이 샘플들을 직접 실행해보면 Rust가 왜 이런 방식으로 메모리 접근을 제한하고,  
어떻게 안전하게 우회할 수 있는지를 이해 하게 됨

