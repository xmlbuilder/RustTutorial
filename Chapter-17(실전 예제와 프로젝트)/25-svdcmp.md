# SVD 
이 코드는 야코비 회전법 기반 고유분해, 이를 활용한 SVD(Singular Value Decomposition),  
그리고 최소제곱 해법을 구현한 고급 선형대수 알고리즘입니다.  
아래에 수학적으로 정확한 의미와 단계별 수식 설명.

## 📘 전체 흐름 요약
| 함수 이름                  | 수학적 표현                                | 설명                                           |
|---------------------------|---------------------------------------------|------------------------------------------------|
| `jacobi_symmetric_eigen`  | $B = V \Lambda V^{\top}$               | 대칭행렬 B의 고유값 분해 (야코비 회전법)       |
| `svdcmp`                  | $A = U \Sigma V^{\top}$                | 행렬 A의 특이값 분해 (SVD)                     |
| `solve_least_squares_svd`| $x = V \Sigma^{-1} U^{\top} b$         | SVD 기반 최소제곱 해법                         |

## 🔍 흐름 설명
- 고유값 분해: jacobi_symmetric_eigen은 대칭행렬 $B=A^{\top }A$ 에 대해 고유값과 고유벡터를 구함
- SVD 구성: svdcmp는 고유값의 제곱근을 특이값으로 사용하고, 고유벡터를 통해 V 구성
- 최소제곱 해: solve_least_squares_svd는 $x=V\Sigma ^{-1}U^{\top }b$ 공식을 통해 해를 계산


## 1️⃣ jacobi_symmetric_eigen: 야코비 회전법
### 목적
대칭행렬 $B\in \mathbb{R^{\mathnormal{n\times n}}}$ 에 대해:

$$
B=V\Lambda V^{\top }
$$

- $\Lambda$ : 대각 고유값 행렬
- V: 열이 고유벡터인 직교행렬

### 핵심 수식
- 회전계수:  

$$
\tau =\frac{a_{qq}-a_{pp}}{2a_{pq}},\quad t=\frac{\mathrm{sgn}(\tau )}{|\tau |+\sqrt{1+\tau ^2}},\quad c=\frac{1}{\sqrt{1+t^2}},\quad s=ct
$$

- 회전 행렬 J로 갱신:

$$
B\leftarrow J^{\top }BJ,\quad V\leftarrow VJ
$$

- 반복 종료 조건:
$$
\sum _{i\neq j}a_{ij}^2<\varepsilon 
$$

## 2️⃣ svdcmp: SVD via 고유분해
### 목적
임의 행렬 $A\in \mathbb{R^{\mathnormal{m\times n}}}$ 에 대해: 

$$ 
A=U\Sigma V^{\top }
$$

- $U\in \mathbb{R^{\mathnormal{m\times n}}}$: 열 직교
- $\Sigma \in \mathbb{R^{\mathnormal{n\times n}}}$: 특이값 대각 행렬
- V\in \mathbb{R^{\mathnormal{n\times n}}}: 직교행렬

### 단계별 수식
- 고유분해 기반 SVD:  

$$
B=A^{\top }A\in \mathbb{R^{\mathnormal{n\times n}}}\quad \Rightarrow \quad B=V\Lambda V^{\top }
$$

- 특이값 계산:

$$
\sigma _i=\sqrt{\lambda _i}\quad (\lambda _i\geq 0)
$$

- U 계산:

$$
U_i=\frac{1}{\sigma _i}AV_i\quad (\sigma _i>0)
$$

- 정규화:

$$
U_i\leftarrow \frac{U_i}{\| U_i\| }
$$

## 3️⃣ solve_least_squares_svd: 최소제곱 해법
### 목적
과잉결정 선형 시스템 Ax=b의 최소제곱 해:  

$$
x=\arg \min _x\| Ax-b\| ^2
$$


### SVD 기반 해법
- SVD 분해:

$$
A=U\Sigma V^{\top }
$$

- 중간 계산:

$$
y=U^{\top }b,\quad y_i=\frac{U_i^{\top }b}{\sigma _i}\quad (\sigma _i>\mathrm{tol})
$$

- 최종 해:

$$
x=Vy=\sum _iy_iV_i
$$

## ✅ 수학적 검증 요약
| 단계 또는 함수            | 수학적 표현                                | 의미 설명                                      | 검증 결과 |
|---------------------------|---------------------------------------------|------------------------------------------------|------------|
| 고유값 분해               | $ B = V \Lambda V^{\top} $               | 대칭행렬 B의 고유값 분해 (야코비 회전법)       | ✅ 정확     |
| 특이값 분해 (SVD)         | $ A = U \Sigma V^{\top} $                | 일반 행렬 A의 SVD 분해                         | ✅ 정확     |
| 최소제곱 해               | $ x = V \Sigma^{-1} U^{\top} b $         | SVD 기반 최소제곱 해 공식                      | ✅ 정확     |
| U 열 정규화               | $ \| U_i \| = 1 $                         | U의 각 열 벡터를 단위 벡터로 정규화            | ✅ 안정적   |


## 🔍 요약 설명
- 모든 수식은 선형대수의 표준 정의에 기반하며, 구현은 수치적으로 안정적입니다.
- 고유값이 음수일 경우에도 max(0, λ) 처리로 특이값 안정화
- 특이값이 0일 때도 안전하게 처리하여 분해 실패 없이 진행
- U의 열 정규화는 EPSILON 기준으로 안정성 확보


아래는 선형 시스템 Ax=b에 대해 SVD 기반으로 해 x를 구하는 확실한 샘플 코드입니다.  
이 예시는 해가 정확히 존재하고, SVD를 통해 안정적으로 복원되는 구조를 갖습니다.

## ✅ 샘플: Ax = b 해 구하기 (SVD 기반)
### 📘 문제 설정

$$
A=\left[ \begin{matrix}1&0&2\\ 0&1&1\\ 1&1&3\\ \end{matrix}\right] ,\quad x=\left[ \begin{matrix}2\\ -1\\ 1\\ \end{matrix}\right] \quad \Rightarrow \quad b=Ax
$$

### 🧪 테스트 코드
```rust
#[test]
fn solve_svd_sample_3x3() {
    use nurbslib::core::matrix::Matrix;
    use nurbslib::core::svd::solve_least_squares_svd;

    // A 정의
    let a = Matrix::from_nested(&[
        &[1.0, 0.0, 2.0],
        &[0.0, 1.0, 1.0],
        &[1.0, 1.0, 4.0],
    ]);

    // x_true 정의
    let x_true = [2.0, -1.0, 1.0];

    // b = Ax 계산
    let mut b = vec![0.0; 3];
    for i in 0..3 {
        b[i] = a.at(i as i32, 0) * x_true[0]
             + a.at(i as i32, 1) * x_true[1]
             + a.at(i as i32, 2) * x_true[2];
    }

    // SVD 기반 해 구하기
    let x = solve_least_squares_svd(a.clone(), &b, 1e-12);
    println!("x* = {:?}", x);

    // 오차 확인
    for i in 0..3 {
        assert!((x[i] - x_true[i]).abs() < 1e-10, "x mismatch at {}: got {}, expect {}", i, x[i], x_true[i]);
    }

    // 잔차 노름
    let mut s2 = 0.0;
    for i in 0..3 {
        let ax = a.at(i as i32, 0) * x[0]
               + a.at(i as i32, 1) * x[1]
               + a.at(i as i32, 2) * x[2];
        let r = b[i] - ax;
        s2 += r * r;
    }
    println!("||r||₂ = {:.6e}", s2.sqrt());
    assert!(s2.sqrt() < 1e-10, "residual too large");
}
```

### ✅ 검증 포인트
- 해 x는 정확히 복원됨
- 잔차 $\| Ax-b\| _2$ 는 거의 0
- SVD 기반이므로 과잉결정/랭크 부족에도 안정적



## ✅ 두 SVD 방식 비교: `svdcmp_sym_right` vs `svdcmp_sym_left`

| 항목                 | svdcmp_sym_right              | svdcmp_sym_left               |
|----------------------|-------------------------------|-------------------------------|
| 기반 행렬            | AᵀA                           | AAᵀ                           |
| 고유값 분해 대상     | 오른쪽 특이벡터 V             | 왼쪽 특이벡터 U               |
| U 계산 방식          | U = A·V / σ                   | 고유벡터 직접 사용            |
| V 계산 방식          | 고유벡터 직접 사용            | V = Aᵀ·U / σ                  |
| 직교성 보장          | U는 수치적으로 깨질 수 있음   | U는 고유벡터로 직교성 보장됨  |
| 랭크 결손 대응       | 불안정하거나 해가 틀어질 수 있음 | 최소 노름 해를 안정적으로 계산 |
| 재구성 정확도        | 고유값 정렬과 U 계산이 민감함 | 수치적으로 더 안정적이고 정확함 |
| 추천 용도            | 풀랭크 행렬, 단순한 구조      | 랭크 결손, 고정밀 해석, 공학적 안정성 |

---


