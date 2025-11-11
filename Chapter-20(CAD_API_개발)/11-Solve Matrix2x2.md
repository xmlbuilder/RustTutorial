#  Solve Matrix2x2

제공해주신 Rust 코드 두 개는 모두 2x2 선형 방정식 시스템을 푸는 함수입니다.  
아래에 수학적 원리와 수식 설명을 정리.

## 🧮 문제 정의: 2x2 선형 방정식
다음과 같은 연립방정식을 풀고자 합니다:  

$$
\begin{aligned}
ax + by &= e,\\
cx + dy &= f
\end{aligned}
$$

이를 행렬 형태로 표현하면:  

$$
\left[ \begin{matrix}a&b&c&d\end{matrix}\right] \cdot \left[ \begin{matrix}x\\ y\end{matrix}\right] =\left[ \begin{matrix}e\\ f\end{matrix}\right]
$$


## ✳️ on_solve_2x2_fast 함수 분석
이 함수는 위 연립방정식을 직접 해석적으로 푸는 방식입니다.  

### 🔹 주요 수식
- 행렬식 (determinant):

$$
\mathrm{det}=a\cdot d-b\cdot c
$$

- 해 $(x = s, y = t)$:  

$$
x=\frac{e\cdot d-b\cdot f}{\mathrm{det}},\quad y=\frac{a\cdot f-e\cdot c}{\mathrm{det}}
$$

## ✅ 수식 검증
이 수식은 **크래머의 공식(Cramer's Rule)** 에 기반한 정확한 해석입니다.  
조건 `det ≠ 0` 일 때 유일한 해가 존재하며, 코드에서도 `det.abs() < ON_TOL12 * scale` 로 판별하고 있습니다.

## ✳️ on_solve_2x2 함수 분석
이 함수는 **가우스 소거법(Gaussian Elimination)** 을 기반으로 해를 구합니다.
### 🔹 처리 순서
- 피벗 선택: 4개 계수 중 절댓값이 가장 큰 항을 기준으로 행/열 스왑
- 정규화: 첫 피벗을 1로 만들기 위해 나눔
- 소거: 두 번째 행에서 첫 번째 항 제거
- 두 번째 피벗 확인: 0이면 rank=1 (해가 하나 또는 무수히 많음)
- 역대입(back substitution): 해 계산
- 열 스왑 여부에 따라 x, y 위치 조정
- pivot_ratio 계산: 최소/최대 피벗 비율 → 수치 안정성 판단
### 🔹 수식 흐름
- 첫 번째 피벗: $m_{00}$
- 정규화:  

$$
m_{01}\leftarrow \frac{m_{01}}{m_{00}},\quad d_0\leftarrow \frac{d_0}{m_{00}}
$$

- 소거:  

$$
m_{11}\leftarrow m_{11}-m_{10}\cdot m_{01},\quad d_1\leftarrow d_1-m_{10}\cdot d_0
$$

- 두 번째 피벗: $m_{11}$
- 역대입:  

$$
y=\frac{d_1}{m_{11}},\quad x=d_0-m_{01}\cdot y
$$

## ✅ 수식 검증
이 방식은 수치적 안정성을 고려한 가우스 소거법 구현이며, 열/행 스왑과 pivot_ratio 계산이 포함되어 있어 실전에서 매우 유용합니다.

## 📊 두 방식 비교

| 항목           | on_solve_2x2_fast           | on_solve_2x2                        | 설명 요약                                  |
|----------------|-----------------------------|-------------------------------------|--------------------------------------------|
| 방식           | 해석적 (크래머의 공식)      | 수치적 (가우스 소거법)              | 수학적 공식 vs. 수치적 안정성 고려         |
| 조건 검사      | det ≠ 0                     | 피벗 ≠ 0, rank 판별                  | 유일 해 존재 여부 판단 방식 차이           |
| 수치 안정성    | 낮음                        | 높음                                | 열/행 스왑, pivot_ratio로 안정성 확보      |
| 성능           | 빠름                        | 안정적이나 약간 느릴 수 있음        | 계산량 적음 vs. 안정성 우선                |
| 반환값         | Option<(x, y)>              | Solve2x2Result {rank, x, y, ratio}  | 간단한 결과 vs. 상세한 구조체 반환         |
| 활용 추천      | 빠른 판별, 단순 계산용      | 정밀 계산, 디버깅 및 로그 분석용    | 목적에 따라 선택 가능                      |

---

## 소스 코드
```rust
#[derive(Copy, Clone, Debug)]
pub struct Solve2x2Result {
    pub rank: i32,
    pub x: f64,
    pub y: f64,
    pub pivot_ratio: f64,
}
```
```rust
pub fn on_solve_2x2(
    mut m00: f64,
    mut m01: f64,
    mut m10: f64,
    mut m11: f64,
    mut d0: f64,
    mut d1: f64,
) -> Solve2x2Result {
    use core::mem;

    // pivot 선택 (최대 절댓값)
    let mut which = 0usize;
    let mut vmax = m00.abs();
    let v01 = m01.abs();
    if v01 > vmax {
        vmax = v01;
        which = 1;
    }
    let v10 = m10.abs();
    if v10 > vmax {
        vmax = v10;
        which = 2;
    }
    let v11 = m11.abs();
    if v11 > vmax {
        vmax = v11;
        which = 3;
    }

    let mut x = 0.0;
    let mut y = 0.0;
    let mut pivot_ratio = 0.0;

    if vmax == 0.0 {
        return Solve2x2Result {
            rank: 0,
            x,
            y,
            pivot_ratio,
        };
    }

    // val5=max pivot, val6=min pivot (초기값은 vmax)
    let mut val5 = vmax;
    let mut val6 = vmax;

    // 열 스왑?
    let mut swapped_cols = false;
    if which % 2 == 1 {
        swapped_cols = true;
        mem::swap(&mut m00, &mut m01);
        mem::swap(&mut m10, &mut m11);
    }
    // 행 스왑?
    if which > 1 {
        mem::swap(&mut d0, &mut d1);
        mem::swap(&mut m00, &mut m10);
        mem::swap(&mut m01, &mut m11);
    }

    // 첫 피벗으로 정규화
    let inv = 1.0 / m00;
    m01 *= inv;
    d0 *= inv;

    // 소거
    if m10 != 0.0 {
        m11 -= m10 * m01;
        d1 -= m10 * d0;
    }

    // 두 번째 피벗 체크 (정확 비교)
    if m11 == 0.0 {
        return Solve2x2Result {
            rank: 1,
            x,
            y,
            pivot_ratio: 0.0,
        };
    }

    // pivot ratio 갱신
    let v = m11.abs();
    if v > val5 {
        val5 = v;
    } else if v < val6 {
        val6 = v;
    }
    pivot_ratio = val6 / val5;

    // back substitution
    d1 /= m11;
    if m01 != 0.0 {
        d0 -= m01 * d1;
    }

    if !swapped_cols {
        x = d0;
        y = d1;
    } else {
        y = d0;
        x = d1;
    }

    Solve2x2Result {
        rank: 2,
        x,
        y,
        pivot_ratio,
    }
}
```
```rust
pub fn solve_2x2_fast(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Option<(f64, f64)> {
    let scale = a
        .abs()
        .max(b.abs())
        .max(c.abs())
        .max(d.abs())
        .max(e.abs())
        .max(f.abs())
        .max(1.0);
    let det = a * d - b * c;
    if det.abs() < ON_TOL12 * scale {
        return None;
    }
    let s = (e * d - b * f) / det;
    let t = (a * f - e * c) / det;
    Some((s, t))
}
```


