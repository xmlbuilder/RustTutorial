# bandec

**밴드 행렬(Band Matrix)** 에 대한 **LU 분해(bandec)** 와 **전진/후진 대입(banbks)** 을 수행하는 알고리즘입니다.  
아래에 각 함수의 수학적 의미, 수식 정리, 그리고 수학적 타당성 검토를 단계별로 정리.

## 📐 1. DenseMat 트레잇의 수학적 의미
```rust
pub trait DenseMat {
    fn n_rows(&self) -> usize;
    fn n_cols(&self) -> usize;
    fn get(&self, r: usize, c: usize) -> f64;
    fn set(&mut self, r: usize, c: usize, v: f64);
    fn swap_rows(&mut self, i: usize, j: usize);
}
```

## 🔹 수학적 해석
- 행렬 $A\in \mathbb{R^{\mathnormal{n\times m}}}$ 에 대해:
- get(i, j) → $A_{ij}$
- set(i, j, v) → $A_{ij}:=v$
- swap_rows(i, j) → $A_{i\cdot }\leftrightarrow A_{j\cdot }$

### ✅ 수학적 타당성
- 행 스왑은 $A_{i\cdot }\leftrightarrow A_{j\cdot }$ 로 정확히 구현됨
- 열 수만큼 루프를 돌며 각 원소를 교환하므로 수학적으로 문제 없음

## 📘 2. bandec: 밴드 행렬 LU 분해
### 🔹 입력 정의
- $A\in \mathbb{R^{\mathnormal{n\times (m_1+m_2+1)}}}$ : 밴드 행렬
- $L$: 하삼각 밴드 저장용 행렬
- $U$: 상삼각 밴드로 변환된 A
- $\mathrm{index}$: 피벗 인덱스 (1-based)
- $d$: 행 교환 부호
### 🔹 수학적 과정
- 슬라이딩 정렬
- 상단 행들을 왼쪽으로 정렬하여 밴드 형태로 맞춤
- 피벗 선택

$$
\max _{j\in [i,i+m_1]}|A_{j,0}| → pivot row
$$

- 행 교환
- $A_{i\cdot }\leftrightarrow A_{\mathrm{imax}\cdot }, d:=-d$
- 하부 제거 (Forward Elimination)
- $r=\frac{A_{j,0}}{A_{i,0}}$
- $A_{j,k-1}:=A_{j,k}-r\cdot A_{i,k}$
- $L_{i,j-i-1}:=r$
## ✅ 수학적 타당성
- LU 분해의 기본 원리와 일치
- 밴드 구조를 유지하며 A=LU 형태로 분해
- pivoting과 작은 값 방어도 포함되어 안정적

## 📘 3. banbks: 전진/후진 대입
### 🔹 수학적 목적
- `LUx=Pb` 를 풀기 위한 두 단계:
- `Ly=Pb` → 전진 대입
- `Ux=y` → 후진 대입
### 🔹 수식 정리
- 전진 대입 (Forward Substitution)

$$
b_j:=b_j\leftrightarrow b_{\mathrm{index}[j]-1}
$$

$$
b_k:=b_k-L_{j,k-j-1}\cdot b_j\quad \mathrm{for\  }k=j+1\dots j+m_1
$$

- 후진 대입 (Backward Substitution)

$$
x_j:=\left( b_j-\sum _{k=1}^{m_2}U_{j,k}\cdot x_{j+k}\right) /U_{j,0}
$$

### ✅ 수학적 타당성
- 전형적인 LU 분해 기반 선형 시스템 해법
- 밴드 구조를 고려한 최적화된 대입 방식
- pivoting과 밴드 폭 제한을 정확히 반영


## 📊 전체 알고리즘 요약
| 단계           | 수학적 연산                          | 코드 또는 수식 표현                                      | 설명 |
|----------------|--------------------------------------|----------------------------------------------------------|------|
| 행 스왑        | $A_{i\cdot} \leftrightarrow A_{j\cdot}$ | `swap_rows(i, j)`                                        | 피벗 행 교환 |
| LU 분해        | $A = LU$                         | `bandec(...)`                                            | 밴드 행렬 분해 |
| 전진 대입      | $Ly = Pb$                        | $y_k := y_k - L_{jk} \cdot y_j$                      | 하삼각 행렬 L |
| 후진 대입      | $Ux = y$                         | $x_j := \frac{y_j - \sum U_{jk} x_k}{U_{jj}}$        | 상삼각 행렬 U |

---
