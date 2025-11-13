## 📐 사용된 수식 요약
### 1. 선형 시스템
$A\cdot x=b$
### 2. LU 분해 (Doolittle 방식, 부분 피벗 포함)

$$
A=L\cdot U\quad \mathrm{where\  }L\mathrm{\  is\  lower\  triangular\  (unit\  diagonal),\  }U\mathrm{\  is\  upper\  triangular}
$$

### 3. 전진 대입 (Forward Substitution)

$$
L\cdot y=P\cdot b\quad \Rightarrow \quad y_i=b_i-\sum _{j=0}^{i-1}L_{ij}y_j
$$

### 4. 후진 대입 (Backward Substitution)

$$
U\cdot x=y\quad \Rightarrow \quad x_i=\frac{y_i-\sum _{j=i+1}^{n-1}U_{ij}x_j}{U_{ii}}
$$

## 🧭 함수별 단계 설명
### 🔹 on_solve_linear_system_vec
- 입력: Matrix 타입의 A, Vec<f64> 타입의 b
- 단계:
    - LU 분해 (부분 피벗 포함)
    - 행 스왑 시 b도 같이 스왑
    - L과 U를 in-place로 A에 저장
    - 전진 대입으로 y 계산 → b에 저장
    - 후진 대입으로 x 계산 → b에 저장

### 🔹 on_solve_linear_system_dense_mut
- 입력: Vec<Vec<f64>> 타입의 A, Vec<f64> 타입의 b
- 단계:
    - LU 분해 (부분 피벗 포함)
    - 행 스왑 시 b도 같이 스왑
    - L과 U를 in-place로 A에 저장
    - 전진 대입 → b에 y 저장
    - 후진 대입 → b에 x 저장

### 🔹 on_solve_linear_system_dense
- 입력: 불변 참조 A, b
- 단계:
    - 내부 복사본 생성
    - on_solve_linear_system_dense_mut 호출
    - 성공 시 x 반환

### 🔹 on_lu_solve_multiple_rhs
- 입력: Matrix A, Vec<f64> b (행우선, nrhs 열)
- 단계:
    - LU 분해 → on_lu_decompose_inplace
    - 각 RHS에 대해:
    - 피벗 적용
    - 전진 대입
    - 후진 대입
    - 결과를 b에 다시 저장

### 🔹 on_lu_solve_single_rhs_inplace
- 입력: Matrix A, Vec<f64> b
- 단계:
    - LU 분해
    - 피벗 적용
    - 전진 대입
    - 후진 대입

### 🔹 on_lu_solve_multiple_rhs_inplace
- 입력: Matrix A, Vec<f64> b (행우선, nrhs 열)
- 단계:
    - LU 분해
    - 각 RHS에 대해:
    - 피벗 적용
    - 전진 대입
    - 후진 대입
    - 결과를 b에 다시 저장

## 📊 선형 시스템 해법 함수 정리 표

| 함수 이름                          | 입력 형태                     | 기능 요약                            | 수학적 핵심 수식 또는 처리 흐름                        |
|-----------------------------------|-------------------------------|-------------------------------------|--------------------------------------------------------|
| `on_solve_linear_system_vec`      | `Matrix`, `Vec<f64>`          | in-place LU 분해 + 전진/후진 대입   | $ A = LU,\ Ly = b,\ Ux = y$                         |
| `on_solve_linear_system_dense_mut`| `Vec<Vec<f64>>`, `Vec<f64>`   | 일반 벡터 기반 in-place 해법        | $ A = LU,\ Ly = b,\ Ux = y$                         |
| `on_solve_linear_system_dense`    | `&[Vec<f64>]`, `&[f64]`       | 복사 기반 안전한 해법               | 내부 복사 후 dense_mut 호출                           |
| `on_lu_solve_multiple_rhs`        | `Matrix`, `Vec<f64>`          | 다중 RHS 해법 (행우선 저장)         | $ A = LU,\ Pb \rightarrow y \rightarrow x$         |



---
# 샘플 예제
아래는 선형 시스템 해법 함수들 각각에 대한 간단한 Rust 샘플 예제입니다.  
각 예제는 함수의 사용법을 익히는 데 도움이 되도록 최소한의 코드로 구성되어 있으며,  
입력 행렬과 벡터를 설정하고 해를 확인하는 방식입니다.

# 📘 함수별 샘플 예제
### 1. on_solve_linear_system_vec
```rust
#[test]
fn example_on_solve_linear_system_vec() {
    let mut a = Matrix::from_vec2(&vec![
        vec![2.0, 1.0],
        vec![5.0, 7.0],
    ]);
    let mut b = vec![11.0, 13.0];
    let success = on_solve_linear_system_vec(&mut a, &mut b);
    assert!(success);
    println!("x = {:?}", b); // 결과: [7.111..., -3.222...]
}
```

### 2. on_solve_linear_system_dense_mut
```rust
#[test]
fn example_on_solve_linear_system_dense_mut() {
    let mut a = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ];
    let mut b = vec![5.0, 6.0];
    let success = on_solve_linear_system_dense_mut(&mut a, &mut b);
    assert!(success);
    println!("x = {:?}", b); // 결과: [-4.0, 4.5]
}
```

### 3. on_solve_linear_system_dense
```rust
#[test]
fn example_on_solve_linear_system_dense() {
    let a = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ];
    let b = vec![5.0, 6.0];
    let x = on_solve_linear_system_dense(&a, &b).unwrap();
    println!("x = {:?}", x); // 결과: [-4.0, 4.5]
}
```


### 4. on_lu_solve_multiple_rhs
```rust
#[test]
fn example_on_lu_solve_multiple_rhs() {
    let a = Matrix::from_vec2(&vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let mut b = vec![
        5.0, 6.0, // 첫 번째 RHS
        7.0, 8.0, // 두 번째 RHS
    ];
    let success = on_lu_solve_multiple_rhs(&a, &mut b, 2);
    assert!(success);
    println!("X = {:?}", b); // 결과: [-4.0, 4.5], [-5.0, 5.5]
}
```


## 🧪 함수별 테스트 예제
### 1. on_solve_linear_system_vec
```rust
#[test]
fn test_on_solve_linear_system_vec() {
    let mut a = Matrix::from_vec2(&vec![
        vec![3.0, 2.0],
        vec![1.0, 4.0],
    ]);
    let mut b = vec![10.0, 11.0];
    let success = on_solve_linear_system_vec(&mut a, &mut b);
    assert!(success);
    assert!((b[0] - 2.0).abs() < 1e-6);
    assert!((b[1] - 2.0).abs() < 1e-6);
}
```


### 2. on_solve_linear_system_dense_mut
```rust
#[test]
fn test_on_solve_linear_system_dense_mut() {
    let mut a = vec![
        vec![2.0, 1.0],
        vec![5.0, 7.0],
    ];
    let mut b = vec![11.0, 13.0];
    let success = on_solve_linear_system_dense_mut(&mut a, &mut b);
    assert!(success);
    assert!((b[0] - 7.111111).abs() < 1e-6);
    assert!((b[1] + 3.222222).abs() < 1e-6);
}
```


### 3. on_solve_linear_system_dense
```rust
#[test]
fn test_on_solve_linear_system_dense() {
    let a = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ];
    let b = vec![5.0, 6.0];
    let x = on_solve_linear_system_dense(&a, &b).unwrap();
    assert!((x[0] + 4.0).abs() < 1e-6);
    assert!((x[1] - 4.5).abs() < 1e-6);
}
```


### 4. on_lu_solve_multiple_rhs
```rust
#[test]
fn test_on_lu_solve_multiple_rhs() {
    let a = Matrix::from_vec2(&vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let mut b = vec![
        5.0, 6.0, // 첫 번째 RHS
        7.0, 8.0, // 두 번째 RHS
    ];
    let success = on_lu_solve_multiple_rhs(&a, &mut b, 2);
    assert!(success);
    assert!((b[0] + 4.0).abs() < 1e-6);
    assert!((b[1] - 4.5).abs() < 1e-6);
    assert!((b[2] + 5.0).abs() < 1e-6);
    assert!((b[3] - 5.5).abs() < 1e-6);
}
```
---





