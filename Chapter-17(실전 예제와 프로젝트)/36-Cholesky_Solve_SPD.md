# Cholesky Solve SPD
Cholesky Solve는 대칭 양의 정부호(SPD) 행렬을 효율적으로 푸는 방법입니다.  
SPD 행렬은 Cholesky 분해가 가능하며, 이를 통해 선형 방정식 `Ax = b` 를 빠르고 안정적으로 해결할 수 있습니다.

## 🧮 Cholesky 분해
### 기본 분해식
- `A = LLᵀ`
- `A`: 대칭 양의 정부호 행렬
- `L`: 하삼각 행렬
- `Lᵀ`: L의 전치 행렬
- 이 분해는 LU 분해보다 계산량이 적고 수치적으로 안정적입니다.

## ✅ SPD 행렬이란?
SPD(Symmetric Positive Definite) 행렬은 다음 조건을 만족하는 정방 행렬입니다:
- 대칭성:  `A = Aᵀ` 
- 양의 정부호성: 모든 벡터  $xᵀAx > 0 \quad \text{for all } x ≠ 0$ 
- 이 조건을 만족하면 Cholesky 분해가 가능하며, 분해 결과는 유일합니다 (단, 부호 선택에 따라 다를 수 있음).

## 🧩 Cholesky Solve의 과정
선형 방정식 Ax = b를 푸는 과정은 다음과 같습니다:
- 분해: `A = LLᵀ`
- 전방 대입: `Ly = b` 를 풀어 y 구하기
- 후방 대입: `Lᵀ x = y` 를 풀어 x 구하기
- 이 방식은 역행렬을 직접 구하지 않고도 해를 구할 수 있어 계산 효율이 높습니다.

## ⚡ 장점
- 계산 효율성: LU 분해보다 약 절반의 계산량
- 수치적 안정성: SPD 조건 덕분에 오차에 강함
- 메모리 절약: 하삼각 행렬 하나만 저장하면 됨

## 📌 활용 예시
- 머신러닝에서 커널 행렬 처리
- 물리 시뮬레이션에서 강체 해석
- 금융 수학에서 위험 모델링

---

## 소스 코드
```rust 
pub fn on_cholesky_solve_spd(g: &mut [f64], b: &mut [f64], n: usize) -> bool {
    for k in 0..n {
        let mut sum = 0.0;
        for p in 0..k { let l = g[k*n + p]; sum += l*l; }
        let diag = g[k*n + k] - sum;
        if diag <= ON_TOL14 { return false; }
        g[k*n + k] = diag.sqrt();
        for i in (k+1)..n {
            let mut s = 0.0;
            for p in 0..k { s += g[i*n + p] * g[k*n + p]; }
            g[i*n + k] = (g[i*n + k] - s) / g[k*n + k];
        }
        for j in (k+1)..n { g[k*n + j] = 0.0; }
    }
    for i in 0..n {
        let mut s = 0.0;
        for j in 0..i { s += g[i*n + j] * b[j]; }
        b[i] = (b[i] - s) / g[i*n + i];
    }
    for i in (0..n).rev() {
        let mut s = 0.0;
        for j in (i+1)..n { s += g[j*n + i] * b[j]; }
        b[i] = (b[i] - s) / g[i*n + i];
    }
    true
}
```

### 테스트 코드
```rust


#[cfg(test)]
mod tests_cholesky {
    use geometry::geom::utils::math::{on_cholesky_solve_spd};

    // 유틸: 행렬-벡터 곱 (row-major)
    fn mat_vec(g: &[f64], x: &[f64], n: usize) -> Vec<f64> {
        let mut y = vec![0.0; n];
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += g[i*n + j] * x[j];
            }
            y[i] = s;
        }
        y
    }
    fn vec_sub(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b).map(|(x,y)| x-y).collect()
    }
    fn norm2(v: &[f64]) -> f64 {
        v.iter().map(|&z| z*z).sum::<f64>().sqrt()
    }

    fn make_spd_from_a(a: &[f64], m: usize, n: usize, lambda: f64) -> Vec<f64> {
        let mut g = vec![0.0; n*n];
        for i in 0..n {
            for j in 0..n {
                let mut s = 0.0;
                for k in 0..m {
                    // A is row-major m×n, A[k*n + i] = (k,i)
                    s += a[k*n + i] * a[k*n + j];
                }
                g[i*n + j] = s;
            }
        }
        // (선택) 수치대칭 보정: G = 0.5(G + Gᵀ)
        for i in 0..n {
            for j in (i+1)..n {
                let avg = 0.5 * (g[i*n + j] + g[j*n + i]);
                g[i*n + j] = avg;
                g[j*n + i] = avg;
            }
        }
        // 약한 정칙화
        for i in 0..n { g[i*n + i] += lambda; }
        g
    }

    #[test]
    fn solves_small_2x2_spd() {
        // G = [[4, 1],
        //      [1, 3]], x_true = [1, 2]
        // b = G x = [6, 7]
        let n = 2usize;
        let mut g = vec![4.0, 1.0,
                         1.0, 3.0];
        let mut b = vec![6.0, 7.0];
        let ok = on_cholesky_solve_spd(&mut g, &mut b, n);
        assert!(ok, "Cholesky must succeed on SPD");

        let x = b; // 해는 b에 들어있음
        assert!((x[0] - 1.0).abs() < 1e-12);
        assert!((x[1] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn solves_random_spd_from_ata() {
        // 재현 가능한 작은 랜덤 A (m ≥ n)
        let m = 6usize;
        let n = 4usize;
        let a = [
            1.0,  0.5, -0.3,  0.2,
            -0.4, 1.2,  0.7, -0.1,
            0.3, -0.8,  1.1,  0.6,
            0.9,  0.1,  0.2,  0.7,
            -0.2, 0.4,  0.9, -0.5,
            0.6,  0.3, -0.4,  1.0,
        ];

        // SPD: G = AᵀA + 1e-3 I
        let g_orig = make_spd_from_a(&a, m, n, 1e-3);

        // 임의 x_true, b = G * x_true
        let x_true = [0.7, -1.2, 2.0, 0.5];
        let b_vec  = mat_vec(&g_orig, &x_true, n);
        let mut x  = b_vec.clone(); // 해 저장용 버퍼 재사용

        // 분해/해 구하기 (이 함수가 g를 L로 덮어쓰고, x에 해를 씁니다)
        let mut g_fact = g_orig.clone();              // ★ 원본 G 보존
        let ok = on_cholesky_solve_spd(&mut g_fact, &mut x, n);
        assert!(ok, "SPD system must solve");

        // 잔차: 반드시 원본 G로 계산!
        let gx = mat_vec(&g_orig, &x, n);
        let mut r = 0.0;
        for i in 0..n {
            let ri = gx[i] - b_vec[i];
            r += ri * ri;
        }
        r = r.sqrt();

        // 해 오차 (절대오차 L1)
        let mut e = 0.0;
        for i in 0..n { e += (x[i] - x_true[i]).abs(); }

        assert!(r < 1e-9, "residual should be tiny, got {}", r);
        assert!(e < 1e-6, "solution error should be small, got {}", e);
    }

    #[test]
    fn scaling_invariance() {
        // G x = b를 α로 스케일 → (αG) x = α b → 해 동일
        let n = 3usize;
        let mut g = vec![
            3.0, 0.5, 0.2,
            0.5, 2.0, 0.3,
            0.2, 0.3, 1.5,
        ];
        // x_true = [1, -2, 3]
        let x_true = [1.0, -2.0, 3.0];
        let b0 = mat_vec(&g, &x_true, n);

        // scale
        let alpha = 20.0;
        let mut g_scaled = g.clone();
        for v in &mut g_scaled { *v *= alpha; }
        let mut b_scaled = b0.clone();
        for v in &mut b_scaled { *v *= alpha; }

        let mut x0 = b0.clone();
        assert!(on_cholesky_solve_spd(&mut g, &mut x0, n));
        let mut x1 = b_scaled.clone();
        assert!(on_cholesky_solve_spd(&mut g_scaled, &mut x1, n));

        for i in 0..n {
            assert!((x0[i] - x1[i]).abs() < 1e-10, "scaled solution mismatch");
            assert!((x0[i] - x_true[i]).abs() < 1e-9, "solution must match truth");
        }
    }

    #[test]
    fn near_singular_but_spd_regularized() {
        let n = 3usize;
        // ill-conditioned → 약한 정칙화
        let g_orig = vec![
            1e-12 + 1e-6, 0.0,        0.0,
            0.0,          1.0 + 1e-6, 0.2,
            0.0,          0.2,        1.5 + 1e-6,
        ];
        let x_true = [1.0, 2.0, -1.0];
        let b_orig = mat_vec(&g_orig, &x_true, n);

        let mut g = g_orig.clone();
        let mut b = b_orig.clone();
        let ok = on_cholesky_solve_spd(&mut g, &mut b, n);
        assert!(ok);

        let r = vec_sub(&mat_vec(&g_orig, &b, n), &b_orig);
        assert!(norm2(&r) < 1e-8, "residual too large: {}", norm2(&r));
    }

    #[test]
    fn non_spd_should_fail() {
        // 대칭이지만 PD가 아님: diag(1, 0, -1)
        let n = 3usize;
        let mut g = vec![
            1.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0,-1.0,
        ];
        let mut b = vec![1.0, 2.0, 3.0];
        let ok = on_cholesky_solve_spd(&mut g, &mut b, n);
        assert!(!ok, "indefinite matrix should be rejected");
    }

    #[test]
    fn trivial_sizes() {
        // n=0
        let mut g0: Vec<f64> = vec![];
        let mut b0: Vec<f64> = vec![];
        assert!(on_cholesky_solve_spd(&mut g0, &mut b0, 0));

        // n=1
        let g_orig = vec![4.0];
        let b_orig = vec![8.0];
        let mut g = g_orig.clone();
        let mut b = b_orig.clone();
        assert!(on_cholesky_solve_spd(&mut g, &mut b, 1));
        assert!((b[0] - 2.0).abs() < 1e-12);

        // 잔차
        assert!((g_orig[0]*b[0] - b_orig[0]).abs() < 1e-12);
    }
}
```
