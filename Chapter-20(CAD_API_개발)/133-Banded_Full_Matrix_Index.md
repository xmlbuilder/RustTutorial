## 📘 B-spline 보간 선형계 구성 원리
## 1. 목적
- 곡선 또는 표면 보간 시, 각 방향(u 또는 v)에 대해 B-spline basis를 사용하여 선형 시스템 Ax=b 를 구성한다.
- 이 문서는 Rust 기반 NURBS 커널에서 full matrix 기준으로 선형계를 구성하는 방법을 설명한다.

## 2. 기본 구조
### 2.1 보간 대상
- 데이터: $F[i][j]\in \mathbb{R}$
- 파라미터: $u_i\in [0,1]$, $v_j\in [0,1]$
- 목표: 각 방향(u 또는 v)에 대해 보간된 계수 $x_k$ 를 구함

### 2.2 u 방향 보간 (열 j 고정)
- 고정된 열 j에 대해,
- 각 행 i에서 다음 식을 만족하는 $x_k$ 를 구한다:
```math
\sum _{k=0}^nN_{k,p}(u_i)\cdot x_k=F[i][j]
```
- 즉,
```math
Ax=b
```
- $A_{i,k}=N_{k,p}(u_i)$
- $b_i=F[i][j]$

## 3. B-spline basis의 국소 지지 특성
- B-spline basis $N_{k,p}(u)$ 는 **국소 지지(local support)** 를 가진다.
- 즉, 각 u_i에 대해 basis가 non-zero인 k는 딱 p+1 개뿐이다.
### 3.1 span 계산
- $s=\mathrm{span}(u_i)$
- non-zero basis index: k=s-p,s-p+1,...,s

## 4. Full matrix 기준으로 A 구성
### 4.1 행렬 A의 구성
- 행: 데이터 포인트 index i
- 열: control point index k
- 각 u_i에 대해:
```math
A_{i,s-p+r}=N_{r,p}(u_i),\quad r=0..p
```
- 즉, basis vector N[0..p]는
- 행 i의 열 s-p,...,s에 들어간다.

### 4.2 Rust 코드 예시
```rust
let span = on_find_span(n, p, u[i], &knots);
let basis = on_basis_func_ret_vec(&knots, span, u[i], p);
let first_col = span - p;
```
```rust
for k in 0..=p {
    let col = first_col + k;
    a[i][col] = basis[k];
}
```


## 5. 주의: Piegl의 banded matrix와의 차이
- Piegl 원본 C 코드에서는 banded matrix를 사용한다:
```rust
l = j - i - 1;
A[i][l + k] = N[k];
```

- 이 식은 banded 내부 좌표계를 전제로 한 것이며,
- full matrix에서는 절대 그대로 사용하면 안 된다.
- banded 포맷에서의 열 인덱스 변환은 대략 이런 형태:

$$
𝑐\\_band = (𝑘\\_full − 𝑖) +(diagonal offset)
$$

- 즉, **행 i 기준으로 몇 칸 대각에서 떨어졌나** 로 저장한다.
- 그래서 A[i]의 길이는 n+1이 아니라 ub(=2p-1) 쪽으로 맞춰져 있어야 정상.

- 변환 공식:
  - banded → full:
  ```math
  c=b+i-(p-1)
  ```
  - full → banded:
  ```math
  b=c-i+(p-1)
  ```
  
## 6. 결론
- Rust 커널에서는 full matrix를 사용하므로,
- basis vector는 항상:
```math
\mathrm{column\  index}=\mathrm{span}-p+k
```
- 로 계산해야 한다.
- Piegl의 banded matrix 인덱스 공식은 full matrix에서는 적용되지 않으므로 포팅 시 반드시 변환해야 한다.

---

## 참고 소스
```rust
pub fn on_banded_to_full_col(i: usize, b: usize, p: usize) -> isize {
    let sbw = p as isize - 1;
    (b as isize) + (i as isize) - sbw
}
```
```rust
pub fn on_full_to_banded_col(i: usize, c: usize, p: usize) -> isize {
    let sbw = p as isize - 1;
    (c as isize) - (i as isize) + sbw
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::math_extensions::{on_banded_to_full_col, on_full_to_banded_col};

    #[test]
    fn band_full_index_roundtrip() {
        for p in 2..6 {
            for i in 0..10usize {
                for c in 0..10usize {
                    let b = on_full_to_banded_col(i, c, p);
                    let c2 = on_banded_to_full_col(i, b as usize, p);
                    // b가 음수일 수 있으니 조건부
                    if b >= 0 {
                        assert_eq!(c2, c as isize);
                    }
                }
            }
        }
    }
```
```rust
    #[test]
    fn band_index_range_matches_bandwidth() {
        for p in 2..6 {
            let ub = 2*p - 1;
            let sbw = p - 1;
            for i in 0..20usize {
                // full col이 [i-(p-1) .. i+(p-1)] 범위면 밴드 안에 들어야 함
                for dc in -(sbw as isize)..=(sbw as isize) {
                    let c = i as isize + dc;
                    if c < 0 { continue; }
                    let b = on_full_to_banded_col(i, c as usize, p);
                    assert!(0 <= b && (b as usize) < ub);
                }
            }
        }
    }
}
```


## 🎯 핵심 결론
- banded matrix는 전체 행렬 크기를 줄이지만, 각 행의 내부 저장 공간은 오히려 더 넓어질 수 있다.
- 즉, 전체적으로는 메모리를 아끼지만, 각 행에서 사용하는 인덱스 범위는 full matrix보다 더 커질 수 있다.

## 🔍 왜 그런 일이 생기는가?
- ✔ full matrix
  - 크기: (n+1) × (n+1)
  - 각 행 i에서 basis는 열 k=s-p,...,s에 들어감
  - 즉, basis는 항상 control index에 대응되는 열에 들어간다
  - 인덱스는 절대 좌표로 계산됨 → a[i][col]
- ✔ banded matrix
  - 크기: (n+1) × (2p−1)
  - 각 행 i에서 basis는 band 내부 좌표로 저장됨
  - 대각선은 항상 a[i][p−1]에 위치
  - 열 인덱스는 상대 좌표: b = c − i + (p−1)
  - 그래서 basis를 넣을 때: a[i][l + k] 형태가 됨

## 🔥 왜 banded에서 더 큰 인덱스가 나올 수 있는가?
- 예를 들어 보자.
- 예시: degree p = 3, n = 8
  - full matrix: 9×9
  - banded matrix: 9×(2p−1) = 9×5

- 데이터 포인트 u[i] = u[1]
  - span = 4 (예시)
  - basis index: k = 1, 2, 3, 4
  - control index: c = span − p + k = 1, 2, 3, 4
- full matrix:
  - 열 인덱스: 1, 2, 3, 4
  - a[1][1], a[1][2], a[1][3], a[1][4]
- banded matrix:
  - banded 열 인덱스:
  ```math
  b=c-i+(p-1)=c-1+2=c+1
  ```
  - 2, 3, 4, 5
  - a[1][2], a[1][3], a[1][4], a[1][5] ← ❗ 여기서 5가 나옴
  - banded matrix의 열 인덱스가 full matrix보다 더 커질 수 있다

## 💥 왜 에러가 터지는가?
- Rust에서 a[i][l + k]를 그대로 쓰면:
  - l = j − i − 1
  - k = 0..p
  - l + k가 banded 내부 인덱스인데
  - 이걸 full matrix에 그대로 쓰면 l + k가 실제로 존재하지 않는 열이 될 수 있다
- 즉, banded 공식은 내부 좌표계 전용인데 그걸 full matrix에 쓰면 인덱스가 너무 커져서 배열 범위를 벗어난다.

## Banded vs Full Matrix Index 비교

| 구분 | Full Matrix | Banded Matrix |
|------|-------------|----------------|
| 대각선(diagonal) 위치 | `a[i][i]` | `a[i][p-1]` |
| Basis가 들어가는 열 인덱스 | `a[i][span - p + k]` | `a[i][(span - p + k) - i + (p - 1)]` |


- banded에서는 i가 작고 span이 크면
- l + k가 커져서 full matrix보다 더 큰 인덱스가 나올 수 있다

## 🎯 결론
- banded matrix는 전체 메모리는 줄이지만, 각 행의 내부 인덱스는 더 커질 수 있다
- banded 내부 인덱스 공식은 full matrix에 절대 쓰면 안 된다
- Rust에서는 full matrix를 쓰는 게 일반적이므로 항상 col = span − p + k 공식만 써야 한다

---

