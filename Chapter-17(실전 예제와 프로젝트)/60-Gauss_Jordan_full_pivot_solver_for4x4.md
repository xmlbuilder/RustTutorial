## Gauss–Jordan full pivot solver: step-by-step with formulas
- 이 알고리즘은 4×4 시스템 Ax=d를 풀기 위해 전 피벗팅(full pivoting)을 사용해 증분적으로 단위 행렬 형태로 만듭니다. 
- 내부적으로 행 벡터 $p0,p1,p2,p3\in \mathbb{R^{\mathnormal{5}}}$ 를 유지하며, 각 행은 $[a_0,a_1,a_2,a_3\mid b]$ 형태입니다.

### 초기 최대 피벗 선택과 행/열 스왑

- 최대 피벗 선택: 전체 행렬에서 절대값이 최대인 원소를 찾아 위치 (i,j)로 둡니다.

$$
(i,j)=\arg \max _{r\in \{ 0..3\} ,\, c\in \{ 0..3\} }|A_{rc}|
$$

- 행 스왑: 선택된 행을 첫 번째로 이동.

$$
\mathrm{row_{\mathnormal{0}}}\leftrightarrow \mathrm{row_{\mathnormal{i}}}
$$

- 열 스왑 및 해 변수 재지정: 선택된 열을 첫 번째로 이동.
- 동시에 해 변수 포인터를 재배치해 변수 순서를 추적합니다.

$$
\mathrm{col_{\mathnormal{0}}}\leftrightarrow \mathrm{col_{\mathnormal{j}}}
$$

- 예: $x\leftrightarrow y$ 또는 $x\leftrightarrow z$, $x\leftrightarrow w$.
- 첫 번째 피벗 정규화와 아래쪽 제거
  - 피벗 정규화: 첫 행을 피벗 p0[0]로 나눠 피벗을 1로 만듭니다.

$$  
p0[k]\leftarrow \frac{p0[k]}{p0[0]},\quad k\in \{ 1,2,3,4\}
$$

- 아래 행 제거: 첫 열의 나머지 행을 0으로 만듭니다.

$$
p\ell [k]\leftarrow p\ell [k]-p\ell [0]\cdot p0[k],\quad \ell \in \{ 1,2,3\} , k\in \{ 1,2,3,4\} 
$$

### 두 번째 피벗 선택, 열/행 스왑, 정규화, 제거

- 서브행렬 피벗 선택: 하위 서브행렬 $(\ell \in \{ 1,2,3\} ,\, c\in \{ 1,2,3\} )$ 에서 최대 절대값 피벗을 선택.

$$  
(i',j')=\arg \max _{\ell \in \{ 1,2,3\} , c\in \{ 1,2,3\} }|p_{\ell }[c]|
$$

- 열 스왑(열 1과 j’):

$$
\mathrm{col_{\mathnormal{1}}}\leftrightarrow \mathrm{col_{\mathnormal{1+j'}}}
$$

- 동시에 $y\leftrightarrow z$ 또는 $y\leftrightarrow w$ 로 변수 포인터를 재배치.
- 행 스왑(행 1과 i’):

$$
p1\leftrightarrow p_{1+i'}
$$

- 피벗 정규화:

$$
p1[k]\leftarrow \frac{p1[k]}{p1[1]},\quad k\in \{ 2,3,4\} 
$$

- 아래 행 제거:

$$
p\ell [k]\leftarrow p\ell [k]-p\ell [1]\cdot p1[k],\quad \ell \in \{ 2,3\} , k\in \{ 2,3,4\} 
$$


### 세 번째 피벗 선택, 열/행 스왑, 정규화, 제거

- 서브행렬 피벗 선택: $(\ell \in \{ 2,3\} ,\, c\in \{ 2,3\} )$ 에서 선택.

$$
(i'',j'')=\arg \max _{\ell \in \{ 2,3\} ,\, c\in \{ 2,3\} }|p_{\ell }[c]|
$$

- 열 스왑(열 2와 3):

$$
\mathrm{col_{\mathnormal{2}}}\leftrightarrow \mathrm{col_{\mathnormal{3}}}
$$

- 동시에 $z\leftrightarrow w$ 포인터 재배치.
- 행 스왑(행 2와 3):

$$
p2\leftrightarrow p3
$$

- 피벗 정규화 및 아래 행 제거:

$$
p2[3]\leftarrow \frac{p2[3]}{p2[2]},\quad p2[4]\leftarrow \frac{p2[4]}{p2[2]}p3[3]\leftarrow p3[3]-p3[2]\cdot p2[3],\quad p3[4]\leftarrow p3[4]-p3[2]\cdot p2[4]
$$

- 네 번째 피벗 확인과 후진 대입
- 네 번째 피벗: p3[3] 확인. 0이면 랭크 3 처리:

$$
z\leftarrow p2[4],\quad y\leftarrow p1[4]-p1[2]\cdot z,\quad x\leftarrow p0[4]-p0[1]\cdot y-p0[2]\cdot z
$$

- 랭크 4일 때 후진 대입:

$$
p3[4]\leftarrow \frac{p3[4]}{p3[3]}
$$

$$
p2[4]\leftarrow p2[4]-p2[3]\cdot p3[4]
$$

$$
p1[4]\leftarrow p1[4]-p1[2]\cdot p2[4]-p1[3]\cdot p3[4]
$$

$$
p0[4]\leftarrow p0[4]-p0[1]\cdot p1[4]-p0[2]\cdot p2[4]-p0[3]\cdot p3[4]
$$

- 최종 해는 열 스왑에 따라 재지정된 출력 포인터에

$$
x\leftarrow p0[4],\quad y\leftarrow p1[4],\quad z\leftarrow p2[4],\quad w\leftarrow p3[4]
$$

- 피벗 비율: 수치적 안정성 지표로 최소/최대 피벗 비율을 기록.

$$
\mathrm{pivot\_ ratio}=\frac{\min \{ |pivots|\} }{\max \{ |pivots|\} }
$$

## 소스 코드
```rust
pub fn solve_4x4_full_pivot(
    row0: &[f64; 4],
    row1: &[f64; 4],
    row2: &[f64; 4],
    row3: &[f64; 4],
    d0: f64,
    d1: f64,
    d2: f64,
    d3: f64,
    x_addr: &mut f64,
    y_addr: &mut f64,
    z_addr: &mut f64,
    w_addr: &mut f64,
    pivot_ratio: &mut f64,
) -> i32 {
    // workarray: 4 rows of [a0,a1,a2,a3 | rhs]
    let mut p0 = [0.0f64; 5];
    let mut p1 = [0.0f64; 5];
    let mut p2 = [0.0f64; 5];
    let mut p3 = [0.0f64; 5];

    *pivot_ratio = 0.0;
    *x_addr = 0.0;
    *y_addr = 0.0;
    *z_addr = 0.0;
    *w_addr = 0.0;

    // find max pivot in the whole 4x4 to swap row0/col0
    let mut i = 0usize;
    let mut j = 0usize;
    let mut x = row0[0].abs();
    let mut y;

    y = row0[1].abs();
    if y > x {
        x = y;
        j = 1;
    }
    y = row0[2].abs();
    if y > x {
        x = y;
        j = 2;
    }
    y = row0[3].abs();
    if y > x {
        x = y;
        j = 3;
    }

    y = row1[0].abs();
    if y > x {
        x = y;
        i = 1;
        j = 0;
    }
    y = row1[1].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }
    y = row1[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 2;
    }
    y = row1[3].abs();
    if y > x {
        x = y;
        i = 1;
        j = 3;
    }

    y = row2[0].abs();
    if y > x {
        x = y;
        i = 2;
        j = 0;
    }
    y = row2[1].abs();
    if y > x {
        x = y;
        i = 2;
        j = 1;
    }
    y = row2[2].abs();
    if y > x {
        x = y;
        i = 2;
        j = 2;
    }
    y = row2[3].abs();
    if y > x {
        x = y;
        i = 2;
        j = 3;
    }

    y = row3[0].abs();
    if y > x {
        x = y;
        i = 3;
        j = 0;
    }
    y = row3[1].abs();
    if y > x {
        x = y;
        i = 3;
        j = 1;
    }
    y = row3[2].abs();
    if y > x {
        x = y;
        i = 3;
        j = 2;
    }
    y = row3[3].abs();
    if y > x {
        x = y;
        i = 3;
        j = 3;
    }

    if x == 0.0 {
        return 0; // rank = 0
    }

    let mut maxpiv = x;
    let mut minpiv = x;

    // place rows into p0..p3 with row swap for i
    match i {
        1 => {
            p0[..4].copy_from_slice(row1);
            p0[4] = d1;
            p1[..4].copy_from_slice(row0);
            p1[4] = d0;
            p2[..4].copy_from_slice(row2);
            p2[4] = d2;
            p3[..4].copy_from_slice(row3);
            p3[4] = d3;
        }
        2 => {
            p0[..4].copy_from_slice(row2);
            p0[4] = d2;
            p1[..4].copy_from_slice(row1);
            p1[4] = d1;
            p2[..4].copy_from_slice(row0);
            p2[4] = d0;
            p3[..4].copy_from_slice(row3);
            p3[4] = d3;
        }
        3 => {
            p0[..4].copy_from_slice(row3);
            p0[4] = d3;
            p1[..4].copy_from_slice(row1);
            p1[4] = d1;
            p2[..4].copy_from_slice(row2);
            p2[4] = d2;
            p3[..4].copy_from_slice(row0);
            p3[4] = d0;
        }
        _ => {
            p0[..4].copy_from_slice(row0);
            p0[4] = d0;
            p1[..4].copy_from_slice(row1);
            p1[4] = d1;
            p2[..4].copy_from_slice(row2);
            p2[4] = d2;
            p3[..4].copy_from_slice(row3);
            p3[4] = d3;
        }
    }

    // swap columns to move max pivot to column 0; track output addresses
    let mut x_out = x_addr;
    let mut y_out = y_addr;
    let mut z_out = z_addr;
    let mut w_out = w_addr;

    match j {
        1 => {
            std::mem::swap(&mut x_out, &mut y_out);
            for r in [&mut p0, &mut p1, &mut p2, &mut p3] {
                r.swap(0, 1);
            }
        }
        2 => {
            std::mem::swap(&mut x_out, &mut z_out);
            for r in [&mut p0, &mut p1, &mut p2, &mut p3] {
                r.swap(0, 2);
            }
        }
        3 => {
            std::mem::swap(&mut x_out, &mut w_out);
            for r in [&mut p0, &mut p1, &mut p2, &mut p3] {
                r.swap(0, 3);
            }
        }
        _ => {}
    }

    // Normalize row p0 so that p0[0]=1 and eliminate below
    let mut val = 1.0 / p0[0];
    for k in 1..=4 {
        p0[k] *= val;
    }
    val = -p1[0];
    if val != 0.0 {
        for k in 1..=4 {
            p1[k] += val * p0[k];
        }
    }
    val = -p2[0];
    if val != 0.0 {
        for k in 1..=4 {
            p2[k] += val * p0[k];
        }
    }
    val = -p3[0];
    if val != 0.0 {
        for k in 1..=4 {
            p3[k] += val * p0[k];
        }
    }

    // Next pivot among submatrix [p1..p3][1..3]
    i = 0;
    j = 0;
    x = p1[1].abs();
    y = p1[2].abs();
    if y > x {
        x = y;
        j = 1;
    }
    y = p1[3].abs();
    if y > x {
        x = y;
        j = 2;
    }

    y = p2[1].abs();
    if y > x {
        x = y;
        i = 1;
        j = 0;
    }
    y = p2[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }
    y = p2[3].abs();
    if y > x {
        x = y;
        i = 1;
        j = 2;
    }

    y = p3[1].abs();
    if y > x {
        x = y;
        i = 2;
        j = 0;
    }
    y = p3[2].abs();
    if y > x {
        x = y;
        i = 2;
        j = 1;
    }
    y = p3[3].abs();
    if y > x {
        x = y;
        i = 2;
        j = 2;
    }

    if x == 0.0 {
        *x_out = p0[4];
        return 1; // rank = 1
    }
    if x > maxpiv {
        maxpiv = x;
    } else if x < minpiv {
        minpiv = x;
    }

    // swap columns within 1..3 to set pivot at [*,1]
    match j {
        1 => {
            for r in [&mut p0, &mut p1, &mut p2, &mut p3] {
                r.swap(1, 2);
            }
            std::mem::swap(&mut y_out, &mut z_out);
        }
        2 => {
            for r in [&mut p0, &mut p1, &mut p2, &mut p3] {
                r.swap(1, 3);
            }
            std::mem::swap(&mut y_out, &mut w_out);
        }
        _ => {}
    }

    // swap rows among p1,p2,p3 to set pivot row at p1
    match i {
        1 => {
            std::mem::swap(&mut p1, &mut p2);
        }
        2 => {
            std::mem::swap(&mut p1, &mut p3);
        }
        _ => {}
    }

    // Normalize p1 and eliminate below in column 1
    val = 1.0 / p1[1];
    for k in 2..=4 {
        p1[k] *= val;
    }
    val = -p2[1];
    if val != 0.0 {
        for k in 2..=4 {
            p2[k] += val * p1[k];
        }
    }
    val = -p3[1];
    if val != 0.0 {
        for k in 2..=4 {
            p3[k] += val * p1[k];
        }
    }

    // Next pivot among [p2..p3][2..3]
    i = 0;
    j = 0;
    x = p2[2].abs();
    y = p2[3].abs();
    if y > x {
        x = y;
        j = 1;
    }
    y = p3[2].abs();
    if y > x {
        x = y;
        i = 1;
        j = 0;
    }
    y = p3[3].abs();
    if y > x {
        x = y;
        i = 1;
        j = 1;
    }

    if x == 0.0 {
        *y_out = p2[4];
        *x_out = p0[4] - p0[1] * (*y_out);
        return 2; // rank = 2
    }
    if x > maxpiv {
        maxpiv = x;
    } else if x < minpiv {
        minpiv = x;
    }

    // swap columns 2 and 3 if needed
    if j == 1 {
        for r in [&mut p0, &mut p1, &mut p2, &mut p3] {
            r.swap(2, 3);
        }
        std::mem::swap(&mut z_out, &mut w_out);
    }

    // swap rows p2,p3 if needed
    if i == 1 {
        std::mem::swap(&mut p2, &mut p3);
    }

    // Normalize p2 and eliminate below
    val = 1.0 / p2[2];
    p2[3] *= val;
    p2[4] *= val;

    val = -p3[2];
    if val != 0.0 {
        p3[3] += val * p2[3];
        p3[4] += val * p2[4];
    }

    // Final pivot p3[3]
    x = p3[3].abs();
    if x == 0.0 {
        *z_out = p2[4];
        *y_out = p1[4] - p1[2] * (*z_out);
        *x_out = p0[4] - p0[1] * (*y_out) - p0[2] * (*z_out);
        return 3; // rank = 3
    }
    if x > maxpiv {
        maxpiv = x;
    } else if x < minpiv {
        minpiv = x;
    }

    // Back substitution using compact operations
    p3[4] /= p3[3];
    p2[4] -= p2[3] * p3[4];
    p1[4] -= p1[2] * p2[4] + p1[3] * p3[4];
    p0[4] -= p1[4] * p0[1] + p2[4] * p0[2] + p3[4] * p0[3];

    // Write solution to the possibly swapped outputs
    *x_out = p0[4];
    *y_out = p1[4];
    *z_out = p2[4];
    *w_out = p3[4];
    *pivot_ratio = minpiv / maxpiv;

    4 // rank = 4
}
```
----

## solver: step-by-step with formulas

- LU(부분 피벗팅) solver: step-by-step with formulas이 알고리즘은 A=LU로 분해하고, 대입 벡터를 동시에 갱신하여 Ux=y를 후진 대입으로 풉니다. 
- 여기서 L은 하삼각(단위 대각), U는 상삼각입니다.

### 단계 1: 각 단계에서 부분 피벗 선택과 행 스왑- 피벗 선택:

$$
k\in \{ 0,1,2,3\} ,\quad i^{\ast }=\arg \max _{i\in \{ k,\dots ,3\} }|A_{ik}|
$$

- 특이성 검사: $|A_{i^{\ast }k}|\leq 0$ 이면 특이 행렬로 실패.
- 행 스왑:

$$
A_{k,j}\leftrightarrow A_{i^{\ast },j},\quad \forall j\in \{ k,\dots ,3\} b_k\leftrightarrow b_{i^{\ast }}
$$

- 피벗 인덱스도 함께 교환(필요 시).

### 단계 2: 소거와 L 저장

- 소거 계수(멀티플라이어) 계산 및 L 저장:

$$
m_i=\frac{A_{ik}}{A_{kk}},\quad A_{ik}\leftarrow m_i,\quad \forall i\in \{ k+1,\dots ,3\} 
$$

- U 갱신(상삼각 부분):

$$
A_{ij}\leftarrow A_{ij}-m_i\cdot A_{kj},\quad \forall i\in \{ k+1,\dots ,3\} ,\; j\in \{ k+1,\dots ,3\}
$$

- 우변 갱신:

$$
b_i\leftarrow b_i-m_i\cdot b_k,\quad \forall i\in \{ k+1,\dots ,3\} 
$$

### 단계 3: 후진 대입으로 Ux=b 풀이

- 초기화: x를 빈 배열로 둠.
- 후진 대입:

$$
x_i=\frac{b_i-\sum _{j=i+1}^3A_{ij}\cdot x_j}{A_{ii}},\quad i=3,2,1,0
$$

- 해 기록: $b\leftarrow x$ 로 결과를 저장(요구사항에 맞춤).

## 소스 코드
```rust
pub fn solve_4x4_lu(
    a: &mut [[f64; 4]; 4],
    b: &mut [f64; 4],
) -> bool {
    let mut piv = [0, 1, 2, 3];

    // LU factorization with partial pivoting
    for k in 0..4 {
        // pivot selection
        let mut max_i = k;
        let mut max_v = a[k][k].abs();
        for i in (k + 1)..4 {
            let v = a[i][k].abs();
            if v > max_v {
                max_v = v;
                max_i = i;
            }
        }
        if max_v <= 0.0 {
            return false; // singular
        }
        if max_i != k {
            for j in k..4 {
                let tmp = a[k][j];
                a[k][j] = a[max_i][j];
                a[max_i][j] = tmp;

            }
            {
                let tmp = b[k];
                b[k] = b[max_i];
                b[max_i] = tmp;
            }
            {
                let tmp = piv[k];
                piv[k] = piv[max_i];
                piv[max_i] = tmp;
            }
        }

        // eliminate
        let akk = a[k][k];
        for i in (k + 1)..4 {
            let m = a[i][k] / akk;
            a[i][k] = m; // store L factor
            for j in (k + 1)..4 {
                a[i][j] -= m * a[k][j];
            }
            b[i] -= m * b[k];
        }
    }

    // back substitution (solve Ux = b)
    let mut x = [0.0f64; 4];
    for i in (0..4).rev() {
        let mut sum = b[i];
        for j in (i + 1)..4 {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    // write back solution into b
    for i in 0..4 {
        b[i] = x[i];
    }
    true
}
```
---

## 핵심 차이와 의미

- Gauss–Jordan full pivot: 행/열 모두 교환하여 최대 피벗을 전면에 배치하므로 수치적 안정성이 높고,  
  과정 중에 변수 순서가 바뀌므로 해의 포인터 재지정이 필요합니다.
- 각 단계에서 행을 정규화하고 아래 원소를 제거하며, 마지막에 컴팩트한 후진 대입으로 해를 얻습니다.
  
- LU partial pivot: 각 열에서 부분 피벗만 사용하므로 구현이 간결하고 빠릅니다.
- L의 하삼각 요소를 행렬 내에 저장하고 동시에 우변을 갱신하여, 한 번의 분해 후 표준 후진 대입으로 해를 구합니다.
- 두 방식은 동일한 해를 산출하며, full pivot 방식은 랭크와 피벗 비율을 통해 문제의 조건수와 안정성을  
  더 잘 추적할 수 있다는 장점이 있습니다.

### 테스트 코드
```
#[test]
fn solve_4x4_full_pivot_test() {
    // Full pivot version
    let row0 = [3.0, 2.0, -1.0, 4.0];
    let row1 = [2.0, -2.0, 4.0, 0.0];
    let row2 = [-1.0, 0.5, -1.0, 1.0];
    let row3 = [1.0, 2.0, 3.0, -1.0];
    let (d0, d1, d2, d3) = (1.0, -2.0, 0.0, 3.0);

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    let mut pr = 0.0;

    let rank = solve_4x4_full_pivot(
        &row0, &row1, &row2, &row3,
        d0, d1, d2, d3,
        &mut x, &mut y, &mut z, &mut w,
        &mut pr,
    );
    println!("rank={rank}, solution=({x}, {y}, {z}, {w}), pivot_ratio={pr}");

    // LU version
    let mut a = [
        [3.0, 2.0, -1.0, 4.0],
        [2.0, -2.0, 4.0, 0.0],
        [-1.0, 0.5, -1.0, 1.0],
        [1.0, 2.0, 3.0, -1.0],
    ];
    let mut b = [1.0, -2.0, 0.0, 3.0];
    let ok = solve_4x4_lu(&mut a, &mut b);
    println!("ok={ok}, x={:?}", b);
}
```
```
rank=4, solution=(0.1325301204819277, 1.180722891566265, 0.024096385542168752, -0.4337349397590361), pivot_ratio=0.3346774193548387
ok=true, x=[0.13253012048192767, 1.180722891566265, 0.024096385542168645, -0.4337349397590361]
```

---
