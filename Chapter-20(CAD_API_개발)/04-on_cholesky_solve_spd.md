# on_cholesky_solve_spd 

이 함수는 대칭 양의 정부호(SPD) 행렬에 대해 Cholesky 분해를 수행하고,  
그 결과를 이용해 선형 시스템 Ax=b를 푸는 알고리즘입니다.  
아래에 수학적 수식과 단계별 설명을 정리.

## 🧮 수학적 배경
Cholesky 분해는 SPD 행렬 $A\in \mathbb{R^{\mathnormal{n\times n}}}$ 에 대해 다음을 만족하는 하삼각 행렬 L을 찾는 과정입니다:  

$$
A=LL^{\top }
$$

이후 선형 시스템 $Ax=b$ 는 다음 두 단계로 풀 수 있습니다:
- 전진 대입 (Forward substitution): $Ly=b$
- 후진 대입 (Backward substitution): $L^{\top }x=y$

## 🧩 단계별 수식 설명
### ✅ 1단계: Cholesky 분해
```rust
for k in 0..n {
    ...
}
```

#### 이 루프는 행렬 A를 하삼각 행렬 L로 분해합니다. 각 단계에서:
- 대각 원소 계산:

$$
L_{kk}=\sqrt{A_{kk}-\sum _{p=0}^{k-1}L_{kp}^2}
$$

- 하삼각 원소 계산:

$$
L_{ik}=\frac{A_{ik}-\sum _{p=0}^{k-1}L_{ip}L_{kp}}{L_{kk}}\quad \mathrm{for\  }i>k
$$

- 상삼각 영역은 0으로 설정:

$$
g[k * n + j] = 0.0;
$$

이는 L을 명시적으로 하삼각 행렬로 유지하기 위함입니다.

### ✅ 2단계: 전진 대입 $Ly=b$
```rust
for i in 0..n {
    ...
}
```

#### 이 루프는 다음을 계산합니다:

$$
y_i=\frac{b_i-\sum _{j=0}^{i-1}L_{ij}y_j}{L_{ii}}
$$

여기서 b는 입력 벡터이며, 결과는 y로 덮어씌워집니다.

### ✅ 3단계: 후진 대입 $L^{\top }x=y$
```rust
for i in (0..n).rev() {
    ...
}
```

#### 이 루프는 다음을 계산합니다:

$$
x_i=\frac{y_i-\sum _{j=i+1}^{n-1}L_{ji}x_j}{L_{ii}}
$$

여기서 $L^{\top }$ 은 L의 전치 행렬이며, 결과는 x로 덮어씌워집니다.

## 📌 요약

| 단계            | 수식                                                              | 의미                         |
|-----------------|-------------------------------------------------------------------|------------------------------|
| 분해            | \$A = L L^{\top}$                                            | SPD 행렬 분해                |
| 대각 원소       | \$L_{kk} = \sqrt{A_{kk} - \sum_{p=0}^{k-1} L_{kp}^2}$           | 안정성 검사 포함             |
| 하삼각 원소     | \$L_{ik} = \frac{A_{ik} - \sum_{p=0}^{k-1} L_{ip} L_{kp}}{L_{kk}}$ | 하삼각 요소 계산             |
| 전진 대입       | \$y_i = \frac{b_i - \sum_{j=0}^{i-1} L_{ij} y_j}{L_{ii}}$       | 중간 결과 계산               |
| 후진 대입       | \$x_i = \frac{y_i - \sum_{j=i+1}^{n-1} L_{ji} x_j}{L_{ii}}$     | 최종 해 계산                 |


---


# 코드 검증

## 소스 코드
```rust
pub fn on_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    for k in 0..n {
        let mut sum = 0.0;
        for p in 0..k {
            let l = g[k * n + p];
            sum += l * l;
        }
        let diag = g[k * n + k] - sum;
        if diag <= ON_TOL14 {
            return false;
        }
        g[k * n + k] = diag.sqrt();
        for i in (k + 1)..n {
            let mut s = 0.0;
            for p in 0..k {
                s += g[i * n + p] * g[k * n + p];
            }
            g[i * n + k] = (g[i * n + k] - s) / g[k * n + k];
        }
        for j in (k + 1)..n {
            g[k * n + j] = 0.0;
        }
    }
    for i in 0..n {
        let mut s = 0.0;
        for j in 0..i {
            s += g[i * n + j] * b[j];
        }
        b[i] = (b[i] - s) / g[i * n + i];
    }
    for i in (0..n).rev() {
        let mut s = 0.0;
        for j in (i + 1)..n {
            s += g[j * n + i] * b[j];
        }
        b[i] = (b[i] - s) / g[i * n + i];
    }
    true
}
```
on_cholesky_solve_spd 함수는 수학적으로 Cholesky 분해와 선형 시스템 풀이의 핵심 원리를 정확히 따르고 있습니다.  
아래에 수학적 검증 포인트를 단계별로 정리.

### ✅ 1. Cholesky 분해 단계
```rust
for k in 0..n {
    ...
}
```
- 대각 원소 계산:

$$
L_{kk}=\sqrt{A_{kk}-\sum _{p=0}^{k-1}L_{kp}^2}
$$

- SPD 행렬의 성질을 이용해 양의 값이 나와야 하며, diag <= ON_TOL14 체크로 안정성 확보

- 하삼각 원소 계산:

$$
L_{ik}=\frac{A_{ik}-\sum _{p=0}^{k-1}L_{ip}L_{kp}}{L_{kk}}\quad \mathrm{for\  }i>k
$$

- 분해된 L을 하삼각 형태로 저장하며, 상삼각 영역은 0으로 설정

### ✅ 검증 결과: 수식과 구현이 일치하며, SPD 조건을 만족하는 경우에만 진행됨

### ✅ 2. 전진 대입 $Ly=b$
```rust
for i in 0..n {
    ...
}
```

- 계산식:

$$
y_i=\frac{b_i-\sum _{j=0}^{i-1}L_{ij}y_j}{L_{ii}}
$$

- 하삼각 행렬 L을 이용해 중간 결과 y 계산

### ✅ 검증 결과: 전진 대입 수식과 구현이 정확히 일치

### ✅ 3. 후진 대입 $L^{\top }x=y$
```rust
for i in (0..n).rev() {
    ...
}
```

- 계산식:

$$
x_i=\frac{y_i-\sum _{j=i+1}^{n-1}L_{ji}x_j}{L_{ii}}
$$

- $L^{\top }$ 을 이용해 최종 해 x 계산
### ✅ 검증 결과: 후진 대입 수식과 구현이 정확히 일치


## 🧠 최종 판단
- ✅ 수학적으로 정확한 Cholesky 분해 및 해법 적용
- ✅ SPD 조건 검사 포함 (diag <= ON_TOL14)
- ✅ 메모리 효율을 위해 in-place 방식 사용
- ✅ 전진/후진 대입 모두 수식과 일치

---

# nalgebra 
아래는 기존 함수의 입력/출력 시그니처는 그대로 유지하면서 내부 구현을  라이브러리 기반으로 대체한 버전입니다.  
g는 SPD 행렬 A, b는 우변 벡터이며, 결과는 b에 직접 덮어씌워집니다.

## ✅ nalgebra 기반 대체 함수
```rust
use nalgebra::{DMatrix, DVector, Cholesky};

pub fn on_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    // 1. 입력 슬라이스를 nalgebra 행렬/벡터로 변환
    let a = DMatrix::from_row_slice(n, n, g);
    let mut b_vec = DVector::from_column_slice(b);

    // 2. Cholesky 분해 시도
    if let Some(chol) = Cholesky::new(a) {
        // 3. 시스템 Ax = b 풀기
        let x = chol.solve(&b_vec);

        // 4. 결과를 b 슬라이스에 덮어쓰기
        b.copy_from_slice(x.as_slice());
        true
    } else {
        false
    }
}
```

### 📌 설명
- DMatrix::from_row_slice는 [f64] 슬라이스를 행렬로 변환
- Cholesky::new는 SPD 행렬에 대해 분해 시도 (실패 시 None)
- chol.solve(&b_vec)는 Ax=b 해를 계산
- 결과는 b에 덮어씌워서 기존 인터페이스 유지

### 🧪 주의사항
- nalgebra는 std 기반이므로 no_std 환경에서는 사용 불가
- 입력 슬라이스는 행 우선(row-major) 순서여야 함
- Cholesky::new는 내부적으로 SPD 여부를 검사하므로 ON_TOL14 같은 임계값 체크는 생략 가능

---


