# 📘 Surface Function Degree Elevation
- Bezier Surface Function Degree Elevation은 Bezier Surface의 control point가 아닌,  
    control value(스칼라 값) 그리드에 대해 차수를 올리는 알고리즘이다.
- 즉, 다음과 같은 2D 스칼라 배열:
    - $f_{i,j}$ 에 대해 U 또는 V 방향으로 차수를 올려서 $g_{i,j}$ 를 만드는 과정이다.

## 1. Surface Function이란?
- Bezier Surface는 보통 4D control point $P_{i,j}$ 로 정의되지만,  
    일부 알고리즘에서는 스칼라 값 그리드가 필요하다.
- 예:
    - 곡면의 높이값(height field)
    - 곡면의 스칼라 필드(온도, 압력 등)
    - 곡면의 weight function
    - 곡면의 partial derivative coefficient
    - 곡면의 basis function 변환
- 이런 경우, 각 grid cell은 단순한 실수 값이다:
```math
f_{i,j}\in \mathbb{R}
```
- 이 스칼라 값 배열에 대해 Bezier 차수 상승을 적용하는 것이 바로 Surface Function Degree Elevation이다.

## 2. 문제 정의
- 원래 차수: r
    - 증가량: t
    - 새 차수: r+t
- 원래 스칼라 값:
```math
f_0,f_1,\dots ,f_r
```
- 새 스칼라 값:
```math
g_0,g_1,\dots ,g_{r+t}
```
- 이때, Bezier 곡선 차수 상승과 동일한 방식으로 스칼라 값도 변환해야 한다.

## 3. 핵심 수학
- Bezier function:
```math
F(u)=\sum _{i=0}^rf_iB_{i,r}(u)
```
- 차수 상승 후:
```math
F(u)=\sum _{k=0}^{r+t}g_kB_{k,r+t}(u)
```
- 두 식이 동일한 함수가 되려면:
```math
g_k=\sum _{i=\max (0,k-t)}^{\min (k,r)}E[k][i]\cdot f_i
```
- 여기서 E 는 degree elevation matrix:
```math
E[k][i]=\frac{{r \choose i}{t \choose k-i}}{{r+t \choose k}}
```
## 4. Surface Function에 적용
- Surface Function은 2D grid이므로:
    - UDIR: column 고정, u 방향으로 차수 상승
    - VDIR: row 고정, v 방향으로 차수 상승
- UDIR (u 방향)
```math
g_{i,roc}=\sum _{k=\max (0,i-t)}^{\min (i,r)}E[i][k]\cdot f_{k,roc}
```
- VDIR (v 방향)
```math
g_{roc,j}=\sum _{k=\max (0,j-t)}^{\min (j,r)}E[j][k]\cdot f_{roc,k}
```
- 여기서:
    - roc: row or column index
    - f,l: 계산할 index 범위
    - E: degree elevation matrix

## 5. 왜 Row/Column 단위로 처리하는가?
- Bezier Surface는 tensor product 구조:
```math
S(u,v)=\sum _iB_{i,p}(u)\sum _jB_{j,q}(v)f_{i,j}
```
- 따라서:
    - U 방향 차수 상승 → 각 V column을 독립적인 Bezier function으로 처리
    - V 방향 차수 상승 → 각 U row를 독립적인 Bezier function으로 처리
- 이 방식이:
    - 수학적으로 정확
    - 계산 효율적
    - 메모리 접근이 단순
    - CAD/NURBS 표준 방식

## 6. 알고리즘 절차 (Pseudo Code)
- UDIR
```rust
for i = f..l:
    a = max(0, i - t)
    b = min(i, r)
    g[i][roc] = Σ_{k=a..b} E[i][k] * f[k][roc]
```

- VDIR
```rust
for j = f..l:
    a = max(0, j - t)
    b = min(j, r)
    g[roc][j] = Σ_{k=a..b} E[j][k] * f[roc][k]
```


## 7. Rust 구현과의 연결
- Rust 함수:
```rust
pub fn on_bezier_surface_function_degree_elevate_rowcol(...)
```
- fp → 원래 스칼라 grid
- fq → 새 스칼라 grid
- rm → degree elevation matrix
- dir → UDIR / VDIR
- f,l → 계산 범위
- roc → row/column index

## 8. 예시
- 원래 값:
```
f=[10,20,30]
```
- 차수 상승: r=2,t=1→r+t=3
    - Elevation matrix:
```math
E=\left[ \begin{matrix}1&0&0\\ \frac{2}{3}&\frac{1}{3}&0\\ 0&\frac{1}{3}&\frac{2}{3}\\ 0&0&1\end{matrix}\right]
``` 
- 새 값:
```math
g_0=1\cdot 10=10
```
```math
g_1=\frac{2}{3}10+\frac{1}{3}20=13.33
```
```math
g_2=\frac{1}{3}20+\frac{2}{3}30=26.66
```
```math
g_3=1\cdot 30=30
```

## 9. 결론
- Surface Function Degree Elevation은:
    - ✔ Bezier function의 차수 상승을
    - ✔ Surface의 row/column 단위로 적용하는 알고리즘이다.
    - ✔ 스칼라 값이므로 control point가 아닌 control value에 적용된다.
    - ✔ 정확한 수학적 변환이며, 형상(함수 형태)을 완전히 보존한다.
    - ✔ degree elevation matrix 기반의 가중합으로 계산된다.
    - ✔ CAD/NURBS 시스템에서 매우 중요한 기본 연산이다.

---

## 🔥 1. 단순 보간이 아니다 → “조합 기반의 공학적 보간”이다
- 일반적인 보간(interpolation)은:
    - 두 값 사이를 선형으로 잇거나
    - 스플라인으로 부드럽게 잇거나
    - 단순한 곡선 fitting을 한다
- 즉, 기존 값 사이를 채우는 것이 목적이다.
- 하지만 Bezier degree elevation은 완전히 다르다.
- ✔ 기존 값의 **선형 보간** 이 아니라
- ✔ 기존 값들의 **이항계수 기반 조합(Weighted Combination)** 이다.
- 수식:
```math
g_k=\sum _{i=\max (0,k-t)}^{\min (k,r)}\frac{{r \choose i}{t \choose k-i}}{{r+t \choose k}}f_i
```
- 이건 단순한 보간이 아니라:
    - 확률적 조합
    - Bernstein basis 변환
    - 조합론적 가중합
    - 형상 보존 변환
- 이 네 가지 성질을 동시에 가진다.

## 🔧 2. 왜 공학 보간(Engineering Interpolation)에 유리한가?
- ✔ (1) 형상 보존 (Shape-preserving)
    - 차수를 올려도 원래 함수/곡선/곡면의 형상이 100% 유지된다.
    - 즉,
        - 값은 바뀌지만
        - 함수는 바뀌지 않는다
    - 이건 공학에서 매우 중요하다.
    - 예:
        - FEM shape function
        - CFD boundary interpolation
        - CAD 곡면 refinement
        - NURBS 기반 해석
    - 이런 곳에서는 형상 보존 + 해상도 증가가 필수다.

- ✔ (2) 안정성 (Numerical Stability)
    - Bernstein basis는 수치적으로 매우 안정적이다.
        - 선형 보간보다 안정적
        - 고차 스플라인보다 안정적
        - FEM shape function과 동일한 안정성
    - 그래서 공학 계산에서 선호된다.

- ✔ (3) 부드러운 고차 보간 가능
    - 차수를 올리면:
        - 더 많은 control value 생성
        - 더 부드러운 고차 함수 표현 가능
        - 고차 미분도 안정적으로 계산 가능
    - 이는 FEM/CFD에서 고차 요소(p-refinement) 와 동일한 개념이다.

- ✔ (4) Tensor-product 구조 → 2D/3D 확장 쉬움
    - U/V 방향 독립적으로 차수를 올릴 수 있기 때문에:
        - 2D 스칼라 필드
        - 3D 스칼라 필드
        - Surface function
        - Volume function
    - 모두 동일한 방식으로 확장 가능.
    - 이건 공학에서 mesh refinement 와 동일한 개념이다.

## 3. Difference From Simple Interpolation

| Method                 | Mathematical Nature                     | Behavior                          | Engineering Meaning                     |
|------------------------|------------------------------------------|------------------------------------|------------------------------------------|
| Linear Interpolation   | Simple linear blend                      | Connects two values directly       | Low accuracy, not shape-preserving       |
| Spline Interpolation   | Piecewise polynomial                     | Smooth curve between samples       | Smooth but does NOT preserve original    |
| Bezier Degree Elevation | Binomial-coefficient weighted combination | Generates new control values from all neighbors | Shape-preserving, stable, engineering-grade refinement |

- 즉, Bezier degree elevation은 **보간** 이 아니라 **기능적 고차화(Function Refinement)** 이다.

## 🚀 4. 그래서 Surface Function Degree Elevation이 강력한 이유
- 2D 스칼라 배열에 적용하면:
    - 기존 surface function의 형상은 그대로
    - 해상도만 증가
    - 중간 값은 단순 보간이 아니라 수학적으로 최적의 조합
    - 공학적 해석에 바로 사용 가능
- 예:
    - 곡면의 weight function refinement
    - 곡면의 partial derivative field refinement
    - FEM shape function 고차화
    - CFD boundary condition smoothing
    - CAD surface refinement

## 🎯 5. 결론
- ✔ 단순 보간이 아니다
- ✔ 기존 값들의 조합으로 만들어지는 “공학적 고차 보간”이다
- ✔ 형상 보존 + 안정성 + 고차화
- ✔ 공학 해석(FEM/CFD/CAD)에 매우 유리
- ✔ 2D/3D 스칼라 필드에도 그대로 적용 가능

---

## 소스 코드
```rust
/// 편의 헬퍼:
/// degree elevation matrix를 내부에서 생성해서 한 row/column만 올림.
///
/// - fp  : (old_deg+1) x N (UDir) or N x (old_deg+1) (VDir)
/// - old_deg : r
/// - t   : increment
/// - dir : elev dir
/// - f,l : 범위 (i 또는 j)
/// - roc : row / column index
/// - fq  : (old_deg+t+1) x N or N x (old_deg+t+1)
pub fn on_bezier_surface_function_degree_elevate_rowcol(
    fp: &[Vec<f64>],
    old_deg: usize,
    t: usize,
    dir: SurfaceDir,
    f: usize,
    l: usize,
    roc: usize,
    fq: &mut [Vec<f64>],
) {
    let rm = on_degree_elevation_matrix(old_deg, t);
    on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
        fp, old_deg, t, &rm, dir, f, l, roc, fq,
    );
}
```
```rust
/// Elevate the degree of a Bezier *surface function* (scalar control values)
/// for a single row/column, using a precomputed degree elevation matrix.
///
/// Rust equivalent of C B_sfn del:
///   B_sfndel(fp,r,t,&dm,dir,f,l,roc,fq);
///
/// Parameters:
/// - fp  : original control values, size = (r+1) x N  (UDir) or N x (r+1) (VDir)
/// - r   : original degree in the elevated direction
/// - t   : increment (new degree = r + t)
/// - rm  : degree elevation matrix, size = (r+t+1) x (r+1)
/// - dir : SurfaceDir::UDir or SurfaceDir::VDir
/// - f,l : first and last indices in the elevated direction to compute (inclusive)
/// - roc : row or column index orthogonal to the elevated direction
/// - fq  : output control values, must have size (r+t+1) x N (UDir) or N x (r+t+1) (VDir)
pub fn on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
    fp: &[Vec<f64>],
    r: usize,
    t: usize,
    rm: &[Vec<f64>],
    dir: SurfaceDir,
    f: usize,
    l: usize,
    roc: usize,
    fq: &mut [Vec<f64>],
) {
    let new_deg = r + t;

    debug_assert_eq!(rm.len(), new_deg + 1);
    debug_assert!(rm.iter().all(|row| row.len() == r + 1));
    debug_assert!(f <= l);
    debug_assert!(l <= new_deg);

    match dir {
        SurfaceDir::UDir => {
            // fp: (r+1) x M, fq: (new_deg+1) x M, column=roc 고정
            debug_assert!(fp.len() >= r + 1);
            debug_assert!(fq.len() >= new_deg + 1);
            debug_assert!(fp.iter().all(|row| roc < row.len()));
            debug_assert!(fq.iter().all(|row| roc < row.len()));

            for i in f..=l {
                let a = if i > t { i - t } else { 0 };
                let b = if i > r { r } else { i };

                let mut sum = 0.0;
                for k in a..=b {
                    sum += rm[i][k] * fp[k][roc];
                }
                fq[i][roc] = sum;
            }
        }

        SurfaceDir::VDir => {
            // fp: N x (r+1), fq: N x (new_deg+1), row=roc 고정
            debug_assert!(roc < fp.len());
            debug_assert!(roc < fq.len());
            debug_assert!(fp[roc].len() >= r + 1);
            debug_assert!(fq[roc].len() >= new_deg + 1);

            for j in f..=l {
                let a = if j > t { j - t } else { 0 };
                let b = if j > r { r } else { j };

                let mut sum = 0.0;
                for k in a..=b {
                    sum += rm[j][k] * fp[roc][k];
                }
                fq[roc][j] = sum;
            }
        }
    }
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod tests_surface_function_degree_elevate {
    use nurbslib::core::basis::on_degree_elevation_matrix;
    use nurbslib::core::bezier_surface::on_bezier_surface_function_degree_elevate_rowcol_with_matrix;
    use nurbslib::core::types::SurfaceDir;


    // 간단한 스칼라 값 생성 헬퍼
    fn val(i: usize, j: usize) -> f64 {
        (i as f64) * 10.0 + (j as f64)
    }

    #[test]
    fn test_surface_function_degree_elevate_u_direction() {
        let r = 3usize;   // original degree
        let t = 1usize;   // elevate by 1 → new degree = 4
        let q = 2usize;   // v-direction count

        // fp: (r+1) x (q+1)
        let mut fp = vec![vec![0.0; q + 1]; r + 1];
        for i in 0..=r {
            for j in 0..=q {
                fp[i][j] = val(i, j);
            }
        }

        // fq: (r+t+1) x (q+1)
        let mut fq = vec![vec![0.0; q + 1]; r + t + 1];

        // degree elevation matrix
        let rm = on_degree_elevation_matrix(r, t);

        // column index
        let roc = 1usize;

        // compute only i = 0..4
        on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
            &fp,
            r,
            t,
            &rm,
            SurfaceDir::UDir,
            0,
            r + t,
            roc,
            &mut fq,
        );

        // 검증: 첫 control value는 동일해야 함
        assert!((fq[0][roc] - fp[0][roc]).abs() < 1e-12);

        // 검증: 마지막 control value도 동일해야 함
        assert!((fq[r + t][roc] - fp[r][roc]).abs() < 1e-12);

        // 중간 값은 행렬 기반 가중합으로 계산됨
        // 직접 계산해서 비교
        for i in 1..(r + t) {
            let mut expected = 0.0;
            let a = if i > t { i - t } else { 0 };
            let b = if i > r { r } else { i };
            for k in a..=b {
                expected += rm[i][k] * fp[k][roc];
            }
            assert!((fq[i][roc] - expected).abs() < 1e-12);
        }
    }
```
```rust
    #[test]
    fn test_surface_function_degree_elevate_v_direction() {
        let r = 2usize;   // original degree in v
        let t = 2usize;   // elevate by 2 → new degree = 4
        let p = 3usize;   // u count

        // fp: (p+1) x (r+1)
        let mut fp = vec![vec![0.0; r + 1]; p + 1];
        for i in 0..=p {
            for j in 0..=r {
                fp[i][j] = val(i, j);
            }
        }

        // fq: (p+1) x (r+t+1)
        let mut fq = vec![vec![0.0; r + t + 1]; p + 1];

        let rm = on_degree_elevation_matrix(r, t);

        let roc = 2usize; // row index

        on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
            &fp,
            r,
            t,
            &rm,
            SurfaceDir::VDir,
            0,
            r + t,
            roc,
            &mut fq,
        );

        // 첫 control value 동일
        assert!((fq[roc][0] - fp[roc][0]).abs() < 1e-12);

        // 마지막 control value 동일
        assert!((fq[roc][r + t] - fp[roc][r]).abs() < 1e-12);

        // 중간 값 검증
        for j in 1..(r + t) {
            let mut expected = 0.0;
            let a = if j > t { j - t } else { 0 };
            let b = if j > r { r } else { j };
            for k in a..=b {
                expected += rm[j][k] * fp[roc][k];
            }
            assert!((fq[roc][j] - expected).abs() < 1e-12);
        }
    }
}
```
```rust
#[cfg(test)]
mod tests_surface_function_degree_elevate_chatgpt {

    // tests/bezier_surface_function_degree_elevate_tests.rs
    //
    // 대상 함수:
    // - on_bezier_surface_function_degree_elevate_rowcol_with_matrix()
    // - (필요하면) on_bezier_surface_function_degree_elevate_rowcol()
    //
    // 테스트 전략:
    // 1) UDir: u-degree r -> r+t 로 올린 fq를 "전체 column(roc)" 반복으로 채운 뒤,
    //    임의 (u,v)에서 scalar Bezier surface function 값이 fp와 동일한지 확인 (정확히 보존되어야 함).
    // 2) VDir도 동일.
    // 3) row/col kernel이 full 결과의 특정 roc에서 동일한지 확인.
    //
    // 중복 최소화:
    // - Bernstein: crate::core::basis::on_bernstein 사용
    // - 난수: 간단한 xorshift (테스트 내에만)

    use nurbslib::core::basis::{on_bernstein, on_degree_elevation_matrix};
    use nurbslib::core::bezier_surface::on_bezier_surface_function_degree_elevate_rowcol_with_matrix;
    use nurbslib::core::types::SurfaceDir;


    #[derive(Clone)]
    struct XorShift64 {
        s: u64,
    }
    impl XorShift64 {
        fn new(seed: u64) -> Self { Self { s: seed } }
        fn next_u64(&mut self) -> u64 {
            let mut x = self.s;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.s = x;
            x
        }
        fn next_f64(&mut self) -> f64 {
            let u = self.next_u64() >> 11; // 53 bits
            (u as f64) * (1.0 / ((1u64 << 53) as f64))
        }
        fn range_f64(&mut self, a: f64, b: f64) -> f64 {
            a + (b - a) * self.next_f64()
        }
    }

    // scalar Bezier surface function eval:
    // fp size (p+1) x (q+1), degree p in u, q in v
    fn eval_scalar_surface(fp: &[Vec<f64>], p: usize, q: usize, u: f64, v: f64) -> f64 {
        let mut sum = 0.0;
        for i in 0..=p {
            let bu = on_bernstein(p, i, u);
            for j in 0..=q {
                let bv = on_bernstein(q, j, v);
                sum += bu * bv * fp[i][j];
            }
        }
        sum
    }

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }
```
```rust
    #[test]
    fn sfndel_udirection_preserves_values() {
        let mut rng = XorShift64::new(0x1234_5678);

        let r = 4usize; // u-degree
        let q = 5usize; // v-degree
        let t = 3usize; // elevate amount
        let new_r = r + t;

        // fp: (r+1) x (q+1)
        let mut fp = vec![vec![0.0; q + 1]; r + 1];
        for i in 0..=r {
            for j in 0..=q {
                fp[i][j] = rng.range_f64(-3.0, 3.0);
            }
        }

        // fq: (new_r+1) x (q+1)
        let mut fq = vec![vec![0.0; q + 1]; new_r + 1];

        let rm = on_degree_elevation_matrix(r, t);

        // fill all columns (roc = v-index)
        for roc in 0..=q {
            on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
                &fp,
                r,
                t,
                &rm,
                SurfaceDir::UDir,
                0,
                new_r,
                roc,
                &mut fq,
            );
        }

        // Compare evals at random (u,v)
        let tol = 1e-12;
        for _ in 0..500 {
            let u = rng.next_f64();
            let v = rng.next_f64();

            let a = eval_scalar_surface(&fp, r, q, u, v);
            let b = eval_scalar_surface(&fq, new_r, q, u, v);

            assert!(approx(a, b, tol), "UDir mismatch: u={} v={} a={} b={} diff={}", u, v, a, b, (a-b).abs());
        }
    }
```
```rust
    #[test]
    fn sfndel_vdirection_preserves_values() {
        let mut rng = XorShift64::new(0xDEAD_BEEF);

        let p = 6usize; // u-degree
        let r = 3usize; // v-degree (elevate dir degree is r here)
        let t = 2usize;
        let new_r = r + t;

        // fp: (p+1) x (r+1)
        let mut fp = vec![vec![0.0; r + 1]; p + 1];
        for i in 0..=p {
            for j in 0..=r {
                fp[i][j] = rng.range_f64(-2.0, 2.0);
            }
        }

        // fq: (p+1) x (new_r+1)
        let mut fq = vec![vec![0.0; new_r + 1]; p + 1];

        let rm = on_degree_elevation_matrix(r, t);

        // fill all rows (roc = u-index)
        for roc in 0..=p {
            on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
                &fp,
                r,
                t,
                &rm,
                SurfaceDir::VDir,
                0,
                new_r,
                roc,
                &mut fq,
            );
        }

        // Compare evals at random (u,v)
        let tol = 1e-12;
        for _ in 0..500 {
            let u = rng.next_f64();
            let v = rng.next_f64();

            let a = eval_scalar_surface(&fp, p, r, u, v);
            let b = eval_scalar_surface(&fq, p, new_r, u, v);

            assert!(approx(a, b, tol), "VDir mismatch: u={} v={} a={} b={} diff={}", u, v, a, b, (a-b).abs());
        }
    }
```
```rust
    #[test]
    fn sfndel_kernel_matches_full_for_one_roc_udirection() {
        let mut rng = XorShift64::new(0xAAAA_BBBB);

        let r = 5usize;
        let q = 4usize;
        let t = 2usize;
        let new_r = r + t;

        let mut fp = vec![vec![0.0; q + 1]; r + 1];
        for i in 0..=r {
            for j in 0..=q {
                fp[i][j] = rng.range_f64(-5.0, 5.0);
            }
        }

        let rm = on_degree_elevation_matrix(r, t);

        // full fill
        let mut fq_full = vec![vec![0.0; q + 1]; new_r + 1];
        for roc in 0..=q {
            on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
                &fp, r, t, &rm, SurfaceDir::UDir, 0, new_r, roc, &mut fq_full,
            );
        }

        // kernel fill only one roc
        let roc = 3usize;
        let mut fq_one = vec![vec![0.0; q + 1]; new_r + 1];
        on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
            &fp, r, t, &rm, SurfaceDir::UDir, 0, new_r, roc, &mut fq_one,
        );

        let tol = 1e-14;
        for i in 0..=new_r {
            let a = fq_full[i][roc];
            let b = fq_one[i][roc];
            assert!(approx(a, b, tol), "roc column mismatch i={} a={} b={} diff={}", i, a, b, (a-b).abs());
        }
    }

}
```
```rust
#[cfg(test)]
mod tests_surface_function_degree_elevate_sample {
    use nurbslib::core::basis::on_degree_elevation_matrix;
    use nurbslib::core::bezier_surface::on_bezier_surface_function_degree_elevate_rowcol_with_matrix;
    use nurbslib::core::types::SurfaceDir;

    // 간단한 스칼라 값 생성
    fn val(i: usize, j: usize) -> f64 {
        (i as f64) * 10.0 + (j as f64)
    }
```
```rust
    #[test]
    fn test_scalar_surface_degree_elevate_u_dir_by_2() {
        // 원래 U 차수 r = 3  → control values: 0,1,2,3
        // 차수 2 상승 → new degree = 5 → control values: 0,1,2,3,4,5
        let r = 3usize;
        let t = 2usize; // 차수 2 상승
        let q = 2usize; // v 방향 개수

        // fp: (r+1) x (q+1) = 4 x 3
        let mut fp = vec![vec![0.0; q + 1]; r + 1];
        for i in 0..=r {
            for j in 0..=q {
                fp[i][j] = val(i, j);
            }
        }

        // fq: (r+t+1) x (q+1) = 6 x 3
        let mut fq = vec![vec![0.0; q + 1]; r + t + 1];

        // degree elevation matrix (5 x 4)
        let rm = on_degree_elevation_matrix(r, t);

        // column index (v 방향)
        let roc = 1usize;

        // 전체 범위 계산
        on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
            &fp,
            r,
            t,
            &rm,
            SurfaceDir::UDir,
            0,
            r + t,
            roc,
            &mut fq,
        );

        // 원래 값 확인
        assert!((fq[0][roc] - fp[0][roc]).abs() < 1e-12);
        assert!((fq[5][roc] - fp[3][roc]).abs() < 1e-12);

        // 중간 control value 2개가 새로 생겼는지 확인
        // 새 degree = 5 → index: 0,1,2,3,4,5
        // 기존 degree = 3 → index: 0,1,2,3
        // 새로 생긴 index = 4, 5 중 4만 중간값 (5는 끝점)
        // 하지만 t=2 이므로 실제로 새 control value는 index 1~4 전체가 재계산됨

        // 직접 계산한 expected 값과 비교
        for i in 0..=r + t {
            let a = if i > t { i - t } else { 0 };
            let b = if i > r { r } else { i };

            let mut expected = 0.0;
            for k in a..=b {
                expected += rm[i][k] * fp[k][roc];
            }

            assert!(
                (fq[i][roc] - expected).abs() < 1e-12,
                "Mismatch at index {}: got {}, expected {}",
                i,
                fq[i][roc],
                expected
            );
        }

        // 중간 control value가 실제로 존재하는지 출력 확인용
        println!("Original values (U dir): {:?}", fp.iter().map(|r| r[roc]).collect::<Vec<_>>());
        println!("Elevated values (U dir): {:?}", fq.iter().map(|r| r[roc]).collect::<Vec<_>>());
    }
}
```
```rust
#[cfg(test)]
mod tests_surface_function_degree_elevate_sample_2d {
    use nurbslib::core::basis::on_degree_elevation_matrix;
    use nurbslib::core::bezier_surface::on_bezier_surface_function_degree_elevate_rowcol_with_matrix;
    use nurbslib::core::types::SurfaceDir;

    // 간단한 스칼라 값 생성
    fn val(i: usize, j: usize) -> f64 {
        (i as f64) * 10.0 + (j as f64)
    }

    #[test]
    fn test_scalar_surface_degree_elevate_u_dir_by_2() {
        // 원래 U 차수 r = 3  → control values: 0,1,2,3
        // 차수 2 상승 → new degree = 5 → control values: 0,1,2,3,4,5
        let r = 3usize;
        let t = 2usize; // 차수 2 상승
        let q = 2usize; // v 방향 개수

        // fp: (r+1) x (q+1) = 4 x 3
        let mut fp = vec![vec![0.0; q + 1]; r + 1];
        for i in 0..=r {
            for j in 0..=q {
                fp[i][j] = val(i, j);
            }
        }

        // fq: (r+t+1) x (q+1) = 6 x 3
        let mut fq = vec![vec![0.0; q + 1]; r + t + 1];

        // degree elevation matrix (5 x 4)
        let rm = on_degree_elevation_matrix(r, t);

        // column index (v 방향)
        let roc = 1usize;

        // 전체 범위 계산
        on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
            &fp,
            r,
            t,
            &rm,
            SurfaceDir::UDir,
            0,
            r + t,
            roc,
            &mut fq,
        );

        // 원래 값 확인
        assert!((fq[0][roc] - fp[0][roc]).abs() < 1e-12);
        assert!((fq[5][roc] - fp[3][roc]).abs() < 1e-12);

        // 중간 control value 2개가 새로 생겼는지 확인
        // 새 degree = 5 → index: 0,1,2,3,4,5
        // 기존 degree = 3 → index: 0,1,2,3
        // 새로 생긴 index = 4, 5 중 4만 중간값 (5는 끝점)
        // 하지만 t=2 이므로 실제로 새 control value는 index 1~4 전체가 재계산됨

        // 직접 계산한 expected 값과 비교
        for i in 0..=r + t {
            let a = if i > t { i - t } else { 0 };
            let b = if i > r { r } else { i };

            let mut expected = 0.0;
            for k in a..=b {
                expected += rm[i][k] * fp[k][roc];
            }

            assert!(
                (fq[i][roc] - expected).abs() < 1e-12,
                "Mismatch at index {}: got {}, expected {}",
                i,
                fq[i][roc],
                expected
            );
        }

        // 중간 control value가 실제로 존재하는지 출력 확인용
        println!("Original values (U dir): {:?}", fp.iter().map(|r| r[roc]).collect::<Vec<_>>());
        println!("Elevated values (U dir): {:?}", fq.iter().map(|r| r[roc]).collect::<Vec<_>>());
    }
}
```
```rust
#[cfg(test)]
mod tests_surface_function_degree_elevate_full_2d_vdir {
    use nurbslib::core::basis::on_degree_elevation_matrix;
    use nurbslib::core::bezier_surface::on_bezier_surface_function_degree_elevate_rowcol_with_matrix;
    use nurbslib::core::types::SurfaceDir;

    // 보기 쉬운 스칼라 값 생성: i*10 + j
    fn val(i: usize, j: usize) -> f64 {
        (i as f64) * 10.0 + (j as f64)
    }

    #[test]
    fn test_full_2d_surface_function_degree_elevate_v_dir() {
        let p = 3usize; // U 방향 개수
        let q = 2usize; // V 방향 원래 차수
        let t = 2usize; // V 방향 차수 2 상승 → new degree = 4

        // fp: (p+1) x (q+1) = 4 x 3
        let mut fp = vec![vec![0.0; q + 1]; p + 1];
        for i in 0..=p {
            for j in 0..=q {
                fp[i][j] = val(i, j);
            }
        }

        // fq: (p+1) x (q+t+1) = 4 x 5
        let mut fq = vec![vec![0.0; q + t + 1]; p + 1];

        // degree elevation matrix (5 x 3)
        let rm = on_degree_elevation_matrix(q, t);

        // 전체 2D 배열을 V 방향으로 차수 상승
        for roc in 0..=p {
            on_bezier_surface_function_degree_elevate_rowcol_with_matrix(
                &fp,
                q,
                t,
                &rm,
                SurfaceDir::VDir,
                0,
                q + t,
                roc,
                &mut fq,
            );
        }

        // 출력 확인
        println!("Original 2D array (fp):");
        for row in &fp {
            println!("{:?}", row);
        }

        println!("\nElevated 2D array (fq):");
        for row in &fq {
            println!("{:?}", row);
        }

        // 첫 열과 마지막 열은 동일해야 함
        for i in 0..=p {
            assert!((fq[i][0] - fp[i][0]).abs() < 1e-12);
            assert!((fq[i][q + t] - fp[i][q]).abs() < 1e-12);
        }

        // 중간 열은 degree elevation matrix 기반으로 계산됨
        for j in 1..(q + t) {
            for i in 0..=p {
                let a = if j > t { j - t } else { 0 };
                let b = if j > q { q } else { j };

                let mut expected = 0.0;
                for k in a..=b {
                    expected += rm[j][k] * fp[i][k];
                }

                assert!(
                    (fq[i][j] - expected).abs() < 1e-12,
                    "Mismatch at (i={}, j={}): got {}, expected {}",
                    i,
                    j,
                    fq[i][j],
                    expected
                );
            }
        }
    }
}
```
---

