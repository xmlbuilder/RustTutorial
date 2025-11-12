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


# bandec / banbks

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

## 🔍 어떻게 동작하냐면…
### 1. bandec 함수
- 밴드 행렬 A를 LU 분해합니다:

$$
A=LU
$$

- 여기서:
- $L$: 하삼각 행렬 (밴드 형태로 al에 저장)
- $U$: 상삼각 행렬 (밴드 형태로 a에 저장)
- $index$: 피벗 정보 (행 교환 기록)
- $d$: 행 교환 부호 (행렬식 부호용)

### 2. banbks 함수
- LU 분해 결과를 이용해 Ax=b를 푸는 두 단계 수행:

#### 🔹 전진 대입 (Forward Substitution)

$$
Ly=Pb
$$

- L과 피벗 정보를 이용해 중간 벡터 y 계산

#### 🔹 후진 대입 (Backward Substitution)

$$
Ux=y
$$

- U를 이용해 최종 해 x 계산

- ✅ 결론
    - 네, 이 구조에서는 Ax = b를 넣으면 x를 정확히 얻을 수 있습니다.
    - 단, A는 밴드 행렬이어야 하고, LU 분해가 가능한 경우에만 정확한 해가 나옵니다.
    - 수치적으로 안정한 방식이며, 대규모 희소 행렬에서도 효율적입니다.


## 🧠 전체 흐름 요약
- ① A_dense 생성 → ② X_true 생성 → ③ B = A_dense × X_true
- ④ A_band 변환 → ⑤ LU 분해 (bandec) → ⑥ 해 계산 (banbks)
- ⑦ B ← x 복원됨 → ⑧ B와 X_true 비교


## 🔍 단계별 상세 설명
### ① A_dense 생성: make_dense_band_dd(n, m1, m2)
- 목적: 대각우세한 밴드 행렬 $A\in \mathbb{R^{\mathnormal{n\times n}}}$ 생성
- 구성:
- 대각 성분: $A_{ii}=6.0+\mathrm{상하\  밴드의\  절댓값\  합}$
- 하부 밴드: $A_{i,i-k}=\sin (i+k)\cdot 0.05$
- 상부 밴드: $A_{i,i+k}=\cos (i+3k)\cdot 0.05$
- ✅ 이 행렬은 LU 분해 시 pivot 없이 안정적으로 처리됩니다.

### ② X_true 생성
```rust
x_true.set(i, 0, (i as f64).sin());
x_true.set(i, 1, (i as f64).cos());
```
- 목적: 해 $x\in \mathbb{R^{\mathnormal{n\times 2}}}$ 생성
- 구성: 두 개의 RHS (열)
- 첫 번째 열: $x_i^{(1)}=\sin (i)$
- 두 번째 열: $x_i^{(2)}=\cos (i)$

### ③ B 생성: dense_mul(&a_dense, &x_true)
- 목적: $B=A\cdot X_{\mathrm{true}}$
- 계산:

$$
B_{i,c}=\sum _{k=0}^{n-1}A_{i,k}\cdot X_{k,c}
$$

- 결과: $B\in \mathbb{R^{\mathnormal{n\times 2}}}$, RHS에 대응하는 결과 벡터

### ④ A_band 변환: dense_to_band(&a_dense, m1, m2)
- 목적: 밴드 행렬 포맷으로 변환
- 포맷:
- $A_{i,j}$ → $A_{\mathrm{band}}[i][m1+j-i]$
- 대각: col = m1
- 하부: m1-1, ..., 0
- 상부: m1+1, ..., m1+m2

### ⑤ LU 분해: bandec(...)
- 입력: a_band, al, index, d
- 출력:
- a_band: 상삼각 행렬 U로 변형됨
- al: 하삼각 행렬 L의 밴드 성분 저장
- index: 피벗 인덱스 (1-based)
- d: 행 교환 부호

### ⑥ 해 계산: banbks(...)
- 입력: LU 분해 결과와 RHS B
- 계산:
- 전진 대입: $Ly=Pb$
- 후진 대입: $Ux=y$
- 결과: B가 해 x로 덮어쓰기됨

### ⑦ 결과 검증
```rust
assert!(nearly_eq(got, want, 1e-10))
```

- got = b.get(i, c) → 계산된 해
- want = x_true.get(i, c) → 원래 해
- 오차 허용 범위: $10^{-10}$
- ✅ 정확하게 복원되었는지 확인

## 📌 핵심 요약
| 단계        | 함수 또는 입력값         | 출력 또는 결과값         | 수학적 의미 또는 목적         |
|-------------|---------------------------|---------------------------|-------------------------------|
| ① 행렬 생성 | `make_dense_band_dd`      | `a_dense`                 | 대각우세 밴드 행렬 A 생성     |
| ② 해 생성   | `x_true`                  | `x_true`                  | 참 해 벡터 x 설정             |
| ③ 곱셈 계산 | `dense_mul`               | `B = A × x_true`          | 선형 시스템 RHS 계산          |
| ④ 포맷 변환 | `dense_to_band`           | `a_band`                  | 밴드 저장 형식으로 변환       |
| ⑤ LU 분해   | `bandec`                  | `a_band`, `al`, `index`, `d` | A = LU 분해 수행              |
| ⑥ 해 계산   | `banbks`                  | `b ← x`                   | Ax = b 해 계산 (x 복원됨)     |
| ⑦ 검증      | `assert!(got ≈ want)`     | 비교 결과                  | 해가 정확히 복원되었는지 확인 |
| ⑧ 반복 또는 확장 | 다양한 A, x | 테스트 루프 또는 벤치마크 | 다양한 케이스에 대해 반복 수행 | 알고리즘 안정성 검증   |


## 📌 전체 목표: 선형 시스템 Ax = b 풀기
- 입력:
- $A\in \mathbb{R^{\mathnormal{n\times n}}}$: 밴드 행렬
- $b\in \mathbb{R^{\mathnormal{n\times r}}}$: 우변 벡터 또는 행렬 (r개의 RHS)
- 출력:
- $x\in \mathbb{R^{\mathnormal{n\times r}}}$: 해

---

## ✅ 1단계: bandec — A를 LU로 분해
### 🔹 입력 데이터
| 인자     | 설명 및 역할                                                                 |
|----------|------------------------------------------------------------------------------|
| `a`      | 밴드 저장 형식의 행렬 A (`n × (m1 + m2 + 1)`), LU 분해 대상                  |
| `m1`, `m2` | 하부 밴드 폭 `m1`, 상부 밴드 폭 `m2` — 밴드 구조 정의용                     |
| `al`     | 하삼각 행렬 L의 밴드 성분 저장소 (`n × m1`) — 분해 결과 중 L의 일부 저장     |
| `index`  | 피벗 인덱스 배열 (`length = n`) — 행 교환 정보 저장 (1-based)                |
| `d`      | 행 교환 부호 (`+1` 또는 `-1`) — 행렬식 부호 추적용                           |

### 🔹 수학적 조치
- LU 분해 수행

$$
A=LU
$$

- L: 단위 하삼각 행렬 (밴드 구조, al에 저장)
- U: 상삼각 행렬 (밴드 구조, a에 덮어쓰기)
- 피벗팅 적용
- 행 교환을 통해 수치 안정성 확보
- 피벗 정보는 index에 저장
### 🔹 출력 결과
- a → U로 변형됨
- al → L의 밴드 성분 저장
- index → 피벗 순서 저장
- d → 행 교환 부호 저장

## ✅ 2단계: banbks — LU와 b로 x 계산
### 🔹 입력 데이터
| 인자     | 설명 및 역할                                                                 |
|----------|------------------------------------------------------------------------------|
| `a`      | LU 분해된 상삼각 행렬 U (밴드 저장 형식, 크기: n × (m1 + m2 + 1))             |
| `al`     | 하삼각 행렬 L의 밴드 성분 저장소 (크기: n × m1)                              |
| `index`  | 피벗 인덱스 배열 (길이 n, 1-based) — `bandec`에서 생성된 행 교환 정보         |
| `b`      | 우변 벡터 또는 행렬 (크기: n × r) — 입력은 b, 출력은 해 x로 덮어쓰기됨       |

### 🔹 수학적 조치
- 전진 대입 (Forward Substitution)

$$
Ly=Pb
$$

- 피벗 순서에 따라 b 재배열
- L을 이용해 중간 벡터 y 계산
- 후진 대입 (Backward Substitution)

$$
Ux=y
$$

- U를 이용해 최종 해 x 계산
- 결과는 b에 덮어쓰기됨 → $b\leftarrow x$

## 🧠 전체 흐름 요약

### ✅ 왜 두 단계가 모두 필요한가?
| 단계     | 역할 및 필요성 설명                                                                 |
|----------|--------------------------------------------------------------------------------------|
| `bandec` | 행렬 A를 LU 분해하여 계산 가능한 형태로 변환함. L, U, 피벗 정보를 생성함.              |
| `banbks` | 분해된 L, U와 우변 벡터 b를 이용해 해 x를 계산함. 실제로 Ax = b를 푸는 단계임.         |

### 🔁 핵심 흐름
- bandec 없으면 A를 계산 가능한 형태로 바꿀 수 없음 → 해를 구할 수 없음
- banbks 없으면 분해된 A를 가지고도 b에 대한 해를 구할 수 없음 → 해를 구할 수 없음



## 🧮 예제 설정
- 행렬 A (3×3 tridiagonal):

$$
A=\left[ \begin{matrix}4&1&0\\ ; 1&4&1\\ ; 0&1&4\end{matrix}\right]
$$

- 해 x:

$$
x=\left[ \begin{matrix}1\\ ; 2\\ ; 3\end{matrix}\right]
$$ 
- 우변 b=Ax:

$$
b=\left[ \begin{matrix}4\cdot 1+1\cdot 2=6\\ ; 1\cdot 1+4\cdot 2+1\cdot 3=12\\ ; 1\cdot 2+4\cdot 3=14\end{matrix}\right] \Rightarrow b=\left[ \begin{matrix}6\\ ; 12\\ ;14\end{matrix}\right] 
$$

### 🔧 Step 1: bandec — LU 분해

LU 분해 결과:

$$
( L = \begin{bmatrix} 1 & 0 & 0 \\ ; 0.25 & 1 & 0 \\ ; 0 & 0.2667 & 1 \end{bmatrix} )
$$

$$
( U = \begin{bmatrix} 4 & 1 & 0 \\ ; 0 & 3.75 & 1 \\ ; 0 & 0 & 3.7333 \end{bmatrix} )
$$

- 피벗 index: [1, 2, 3] (1-based, no row swaps needed)
- 행 교환 부호 d=+1

### 🔧 Step 2: banbks — 해 계산
#### 🔹 전진 대입: 

$$
Ly=b
$$

#### 🔹 후진 대입: 

$$
Ux=y
$$

- ✅ 원래 x와 정확히 일치!

## 🎯 요약
| 단계       | 수식 또는 개념 | 함수 또는 처리 | 설명                         | 결과 또는 목적         |
|------------|----------------|----------------|------------------------------|------------------------|
| 문제 정의  | b = Ax         | —              | 선형 시스템 설정             | 해 x를 구하는 것이 목표 |
| 분해 단계  | A = LU         | `bandec`       | 행렬 A를 LU로 분해           | L, U, 피벗 정보 생성    |
| 해 계산    | LUx = b        | `banbks`       | LU 분해 결과와 b로 x 계산    | 해 x를 얻음            |


---



