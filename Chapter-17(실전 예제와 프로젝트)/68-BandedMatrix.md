# BandedMatrxi

- 우리가 만든 BandedMatrix는:
    - 논리적으로는 n×n 정사각 행렬
    - 물리적으로는 대각선 주변만 저장하는 압축 형태
    - 외부에서는 DenseMat처럼 full matrix처럼 보이지만,
- 내부는 banded 구조를 이용해 메모리·연산을 줄이는 구조체.


## 1. 구조체 필드의 의미
```rust
pub struct BandedMatrix {
    n: usize,      // 정사각 행렬 크기 (n x n)
    kl: usize,     // lower bandwidth (아래로 몇 줄까지 non-zero 허용)
    ku: usize,     // upper bandwidth (위로 몇 줄까지 non-zero 허용)
    data: Vec<Real>, // (kl + ku + 1) * n 크기의 band 저장소
}
```
- n
    - 행/열 개수. 정사각 행렬만 지원하니까 rows = cols = n.
- kl (lower bandwidth)
    - 주대각선에서 아래로 몇 줄까지 non-zero를 허용할지.
    - 예: tridiagonal이면 kl = 1.
- ku (upper bandwidth)
    - 주대각선에서 위로 몇 줄까지 non-zero를 허용할지.
    - 예: tridiagonal이면 ku = 1.
- data
    - 실제 숫자들이 저장되는 버퍼.
    - 크기: (kl + ku + 1) * n
- **band 공간** 에서 row-major로 저장:
    - band의 각 줄이 하나의 **band row**
    - 각 band row는 길이 n (열 index)

## 2. new — 생성자
```rust
pub fn new(n: usize, kl: usize, ku: usize) -> Self {
    let bands = kl + ku + 1;
    Self {
        n,
        kl,
        ku,
        data: vec![0.0; bands * n],
    }
}
```

- 역할:
- n × n banded 행렬을 만들되,
- 아래로 kl, 위로 ku만큼 band를 허용.
- bands = kl + ku + 1
- band row 개수.
    - 예: tridiagonal → kl=1, ku=1 → bands = 3.
- data 크기 = bands * n
    - band row가 bands개, 각 row가 n개의 열을 가진다고 보면 됨.

## 3. idx_band — (r, c)를 band 저장소 인덱스로 변환
```rust
fn idx_band(&self, r: usize, c: usize) -> Option<usize> {
    if c + self.kl < r || r + self.ku < c {
        return None;
    }
    let band_row = self.ku + r as isize - c as isize;
    if band_row < 0 || band_row >= (self.kl + self.ku + 1) as isize {
        return None;
    }
    Some(band_row as usize * self.n + c)
}
```

- 이 함수가 핵심.
    - 입력: 논리적 행렬 인덱스 (r, c)
    - 출력: data 벡터에서의 실제 인덱스 idx 또는 None (band 밖)
### 3-1. band 범위 체크
```rust
if c + self.kl < r || r + self.ku < c {
    return None;
}
```

- 이 조건은 (r, c)가 band 안에 있는지를 판정하는 식.
- 직관적으로:
    - r가 너무 아래쪽이면 (r > c + kl) → band 밖
    - c가 너무 오른쪽/위쪽이면 (c > r + ku) → band 밖
### 3-2. band row 계산
```rust
let band_row = self.ku + r as isize - c as isize;
```

- 중앙 대각선(주대각선)이 band row ku에 위치하도록 정의.
- 예:
- r == c → band_row = ku
- r == c + 1 (한 칸 아래) → band_row = ku + 1
- r == c - 1 (한 칸 위) → band_row = ku - 1
3-3. 최종 인덱스
Some(band_row as usize * self.n + c)


- band row를 먼저 곱하고, 그 안에서 열 c를 더함.
- 즉, data는:
- 0..n-1: band_row 0
- n..2n-1: band_row 1
- …

4. get / set — full matrix처럼 보이게 하는 인터페이스
fn get(&self, r: usize, c: usize) -> Real {
    if let Some(idx) = self.idx_band(r, c) {
        self.data[idx]
    } else {
        0.0
    }
}


- (r, c)가 band 안이면 → 실제 값 반환
- band 밖이면 → 항상 0.0 반환
- 외부에서 보면 진짜 full matrix처럼 보인다.
fn set(&mut self, r: usize, c: usize, v: Real) {
    if let Some(idx) = self.idx_band(r, c) {
        self.data[idx] = v;
    } else {
        if v != 0.0 {
            panic!("Setting non-zero outside band");
        }
    }
}


- band 안이면 → 실제로 저장
- band 밖이면:
- v == 0.0 → 무시 (어차피 0이니까)
- v != 0.0 → panic (논리 오류: band 밖에 non-zero를 넣으려는 시도)
이 설계 덕분에:
- 알고리즘은 그냥 get(i,j), set(i,j,v)만 쓰면 됨.
- full인지 banded인지 신경 쓸 필요 없음.

5. swap_rows — pivoting 없는 banded LU를 전제로 금지
fn swap_rows(&mut self, _i: usize, _j: usize) {
    panic!("swap_rows is not supported for BandedMatrix with no pivoting");
}


- banded LU에서 pivoting(행 교환)을 하지 않기로 했기 때문에,
- swap_rows는 논리적으로 허용되지 않는 연산이다.
- 만약 이걸 허용하면:
- band 구조가 깨질 수 있고
- band 폭이 늘어나서 banded의 장점이 사라진다.
그래서 아예 panic!으로 막아두는 게 안전하다.

6. lu_decompose_inplace — pivoting 없는 banded LU
pub fn lu_decompose_inplace(&mut self, zero_tol: Real) -> bool {
    let n = self.n;
    let kl = self.kl;
    let ku = self.ku;

    for k in 0..n {
        let akk = self.get(k, k);
        if akk.abs() <= zero_tol {
            return false;
        }

        let i_max = (k + kl).min(n - 1);
        for i in (k + 1)..=i_max {
            let lik = self.get(i, k) / akk;
            self.set(i, k, lik);

            let j_max = (k + ku).min(n - 1);
            for j in (k + 1)..=j_max {
                let aij = self.get(i, j);
                let ukj = self.get(k, j);
                self.set(i, j, aij - lik * ukj);
            }
        }
    }
    true
}


이건 일반 LU 알고리즘을 band 구조에 맞게 잘라낸 버전이야.
6-1. pivot 검사
let akk = self.get(k, k);
if akk.abs() <= zero_tol {
    return false;
}


- pivot이 0이면 분해 불가능 → 실패.
6-2. L 업데이트 (아래쪽)
let i_max = (k + kl).min(n - 1);
for i in (k + 1)..=i_max {
    let lik = self.get(i, k) / akk;
    self.set(i, k, lik);
    ...
}


- 일반 LU에서는 i = k+1..n-1이지만,
- banded에서는 아래로 kl까지만 non-zero이므로 i_max = k+kl.
- 그 아래는 어차피 0이므로 건드릴 필요 없음.
6-3. U 업데이트 (오른쪽)
let j_max = (k + ku).min(n - 1);
for j in (k + 1)..=j_max {
    let aij = self.get(i, j);
    let ukj = self.get(k, j);
    self.set(i, j, aij - lik * ukj);
}


- 일반 LU에서는 j = k+1..n-1이지만,
- banded에서는 위로 ku까지만 non-zero이므로 j_max = k+ku.
이렇게 해서:
- 연산 범위를 band 안으로만 제한
→ 0과 곱하고 더하는 쓸데없는 연산을 피함
→ 성능 향상

7. lu_solve — LU 결과를 이용해 Ax = b 풀기
pub fn lu_solve(&self, b: &[Real], x: &mut [Real]) -> bool {
    let n = self.n;
    if b.len() < n || x.len() < n {
        return false;
    }

    let kl = self.kl;
    let ku = self.ku;

    // 1) forward substitution: L y = b
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut s = b[i];
        let j_min = i.saturating_sub(kl);
        for j in j_min..i {
            let lij = self.get(i, j);
            s -= lij * y[j];
        }
        y[i] = s;
    }

    // 2) backward substitution: U x = y
    for i in (0..n).rev() {
        let mut s = y[i];
        let j_max = (i + ku).min(n - 1);
        for j in (i + 1)..=j_max {
            let uij = self.get(i, j);
            s -= uij * x[j];
        }
        let uii = self.get(i, i);
        if uii == 0.0 {
            return false;
        }
        x[i] = s / uii;
    }

    true
}


7-1. forward substitution (L y = b)
- L은 unit lower (대각선 1)
- 일반식:
y_i=b_i-\sum _{j<i}L_{ij}y_j- banded에서는 j 범위를 max(0, i-kl)..i로 제한:
- 그 밖은 0이므로 곱할 필요 없음.
7-2. backward substitution (U x = y)
- 일반식:
x_i=\frac{1}{U_{ii}}\left( y_i-\sum _{j>i}U_{ij}x_j\right) - banded에서는 j 범위를 i+1..min(i+ku, n-1)로 제한:
- 그 밖은 0.

정리 한 줄
- BandedMatrix는:
- 논리적으로는 full n×n 행렬
- 물리적으로는 band만 저장
- get/set으로 full처럼 접근 가능
- idx_band로 band 내부 인덱스를 계산
- lu_decompose_inplace / lu_solve는 band 구조를 이용해 연산 범위를 줄임
- pivoting은 하지 않으므로 swap_rows는 금지

---
## 소스 코드
```rust
use crate::core::matrix::DenseMat;
use crate::core::types::Real;

#[derive(Debug, Clone)]
pub struct BandedMatrix {
    n: usize,        // 정사각 행렬 크기
    kl: usize,       // lower bandwidth
    ku: usize,       // upper bandwidth
    data: Vec<Real>, // (kl + ku + 1) * n, row-major in "band space"
}

impl BandedMatrix {
    pub fn new(n: usize, kl: usize, ku: usize) -> Self {
        let bands = kl + ku + 1;
        Self {
            n,
            kl,
            ku,
            data: vec![0.0; bands * n],
        }
    }

    #[inline]
    fn idx_band(&self, r: usize, c: usize) -> Option<usize> {
        // band 밖이면 None
        // (overflow 안전하게: r > c+kl, c > r+ku 형태로 체크)
        if r > c + self.kl {
            return None;
        }
        if c > r + self.ku {
            return None;
        }

        // 중앙 대각선이 ku에 오도록
        // band_row = ku + r - c  (isize로 계산)
        let band_row = (self.ku as isize) + (r as isize) - (c as isize);
        let bands = (self.kl + self.ku + 1) as isize;
        if band_row < 0 || band_row >= bands {
            return None;
        }

        // row-major in band space: band_row * n + c
        Some((band_row as usize) * self.n + c)
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn kl(&self) -> usize {
        self.kl
    }

    pub fn ku(&self) -> usize {
        self.ku
    }

    pub fn as_slice(&self) -> &[Real] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [Real] {
        &mut self.data
    }

    /// 디버깅/테스트 편의: dense로 뽑기
    pub fn to_dense(&self) -> Vec<Vec<Real>> {
        let n = self.n;
        let mut a = vec![vec![0.0; n]; n];
        for r in 0..n {
            // band 범위만 순회
            let c0 = r.saturating_sub(self.kl);
            let c1 = (r + self.ku).min(n - 1);
            for c in c0..=c1 {
                a[r][c] = self.get(r, c);
            }
        }
        a
    }
}

impl DenseMat for BandedMatrix {
    fn n_rows(&self) -> usize {
        self.n
    }

    fn n_cols(&self) -> usize {
        self.n
    }

    fn get(&self, r: usize, c: usize) -> Real {
        if let Some(idx) = self.idx_band(r, c) {
            self.data[idx]
        } else {
            0.0
        }
    }

    fn set(&mut self, r: usize, c: usize, v: Real) {
        if let Some(idx) = self.idx_band(r, c) {
            self.data[idx] = v;
        } else {
            // band 밖에 non-zero를 넣으려 하면 논리적으로 잘못된 사용
            if v != 0.0 {
                panic!("Setting non-zero outside band: ({},{}) = {}", r, c, v);
            }
        }
    }

    /// banded LU에서는 pivoting을 하지 않으므로 row swap은 허용하지 않는다.
    fn swap_rows(&mut self, _i: usize, _j: usize) {
        panic!("swap_rows is not supported for BandedMatrix with no pivoting");
    }
}

impl BandedMatrix {
    /// pivoting 없는 banded LU 분해 (in-place).
    /// 성공 시 true, 실패(0 pivot 등) 시 false.
    ///
    /// 저장 형식:
    /// - L의 하삼각(대각 제외)은 A의 해당 위치에 저장 (L 대각은 1로 암묵적)
    /// - U는 대각 포함 상삼각이 A의 해당 위치에 저장
    pub fn lu_decompose_inplace(&mut self, zero_tol: Real) -> bool {
        let n = self.n;
        let kl = self.kl;
        let ku = self.ku;

        if n == 0 {
            return true;
        }

        for k in 0..n {
            // pivot 검사
            let akk = self.get(k, k);
            if akk.abs() <= zero_tol {
                return false;
            }

            // i = k+1 .. min(k+kl, n-1)
            let i_max = (k + kl).min(n - 1);
            if k + 1 <= i_max {
                for i in (k + 1)..=i_max {
                    let lik = self.get(i, k) / akk;
                    self.set(i, k, lik);

                    // j = k+1 .. min(k+ku, n-1)
                    let j_max = (k + ku).min(n - 1);
                    if k + 1 <= j_max {
                        for j in (k + 1)..=j_max {
                            let aij = self.get(i, j);
                            let ukj = self.get(k, j);
                            self.set(i, j, aij - lik * ukj);
                        }
                    }
                }
            }
        }
        true
    }

    /// LU가 완료된 banded 행렬에 대해 Ax = b를 푼다.
    /// - self: L/U in-place 저장 상태
    /// - b: RHS
    /// - x: 해 (출력)
    /// - zero_tol: Uii near-zero 체크
    ///
    /// 내부에서 y를 Vec로 할당한다(편의 버전).
    pub fn lu_solve(&self, b: &[Real], x: &mut [Real], zero_tol: Real) -> bool {
        let n = self.n;
        if b.len() < n || x.len() < n {
            return false;
        }
        let mut y = vec![0.0; n];
        self.lu_solve_with_workspace(b, x, &mut y, zero_tol)
    }

    /// 코어 루프용: workspace 버퍼(y)를 외부에서 재사용 가능한 버전(할당 없음).
    pub fn lu_solve_with_workspace(
        &self,
        b: &[Real],
        x: &mut [Real],
        y: &mut [Real],
        zero_tol: Real,
    ) -> bool {
        let n = self.n;
        if b.len() < n || x.len() < n || y.len() < n {
            return false;
        }

        let kl = self.kl;
        let ku = self.ku;

        // 1) forward: L y = b (L unit lower)
        for i in 0..n {
            let mut s = b[i];
            let j_min = i.saturating_sub(kl);
            for j in j_min..i {
                let lij = self.get(i, j);
                s -= lij * y[j];
            }
            y[i] = s; // diag(L)=1
        }

        // 2) backward: U x = y
        for i in (0..n).rev() {
            let mut s = y[i];
            let j_max = (i + ku).min(n - 1);
            if i + 1 <= j_max {
                for j in (i + 1)..=j_max {
                    let uij = self.get(i, j);
                    s -= uij * x[j];
                }
            }
            let uii = self.get(i, i);
            if uii.abs() <= zero_tol {
                return false;
            }
            x[i] = s / uii;
        }

        true
    }
}

/// (참고) DenseMat 일반 버전: pivot 없는 LU (O(n^3))
/// BandedMatrix에는 비효율적이니 band 전용 루틴 사용 권장.
pub fn lu_decompose_no_pivot<M: DenseMat>(a: &mut M, zero_tol: Real) -> bool {
    let n = a.n_rows();
    for k in 0..n {
        let akk = a.get(k, k);
        if akk.abs() <= zero_tol {
            return false;
        }
        for i in (k + 1)..n {
            let lik = a.get(i, k) / akk;
            a.set(i, k, lik);
            for j in (k + 1)..n {
                let aij = a.get(i, j);
                let ukj = a.get(k, j);
                a.set(i, j, aij - lik * ukj);
            }
        }
    }
    true
}
```
---
## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::types::{Real};
    use nurbslib::core::banded_matrix::BandedMatrix;
    use nurbslib::core::matrix::DenseMat;

    #[derive(Clone)]
    struct XorShift64 {
        state: u64,
    }
    impl XorShift64 {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }
        fn next_u64(&mut self) -> u64 {
            let mut x = self.state;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.state = x;
            x
        }
        fn next_f64(&mut self) -> f64 {
            // [0,1)
            let u = self.next_u64() >> 11; // 53 bits
            (u as f64) * (1.0 / ((1u64 << 53) as f64))
        }
        fn next_range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.next_f64()
        }
    }

    // ----------------------------
    // 테스트용: dense LU with partial pivot (비교용)
    // ----------------------------
    fn dense_solve_partial_pivot(a_in: &[Vec<Real>], b_in: &[Real], zero_tol: Real) -> Option<Vec<Real>> {
        let n = a_in.len();
        if n == 0 || b_in.len() != n {
            return None;
        }
        for row in a_in {
            if row.len() != n {
                return None;
            }
        }

        let mut a = a_in.to_vec();
        let mut b = b_in.to_vec();
        let mut p = (0..n).collect::<Vec<_>>();

        // LU with partial pivoting (Doolittle)
        for k in 0..n {
            // pivot row
            let mut piv = k;
            let mut maxv = a[p[k]][k].abs();
            for i in (k + 1)..n {
                let v = a[p[i]][k].abs();
                if v > maxv {
                    maxv = v;
                    piv = i;
                }
            }
            if maxv <= zero_tol {
                return None;
            }
            p.swap(k, piv);

            let pk = p[k];
            let akk = a[pk][k];
            for i in (k + 1)..n {
                let pi = p[i];
                let lik = a[pi][k] / akk;
                a[pi][k] = lik;
                for j in (k + 1)..n {
                    a[pi][j] -= lik * a[pk][j];
                }
            }
        }

        // forward solve Ly = Pb
        let mut y = vec![0.0; n];
        for i in 0..n {
            let pi = p[i];
            let mut s = b[pi];
            for j in 0..i {
                s -= a[pi][j] * y[j];
            }
            y[i] = s; // diag(L)=1
        }

        // backward solve Ux = y
        let mut x = vec![0.0; n];
        for ii in 0..n {
            let i = n - 1 - ii;
            let pi = p[i];
            let mut s = y[i];
            for j in (i + 1)..n {
                s -= a[pi][j] * x[j];
            }
            let uii = a[pi][i];
            if uii.abs() <= zero_tol {
                return None;
            }
            x[i] = s / uii;
        }

        Some(x)
    }

    fn max_abs_diff(a: &[Real], b: &[Real]) -> Real {
        let mut m = 0.0;
        for i in 0..a.len().min(b.len()) {
            let d = (a[i] - b[i]).abs();
            if d > m {
                m = d;
            }
        }
        m
    }




    #[test]
    fn test_banded_tridiagonal_lu_solve() {
        let n = 3;
        let kl = 1;
        let ku = 1;
        let mut a = BandedMatrix::new(n, kl, ku);

        // A =
        // [  2  -1   0 ]
        // [ -1   2  -1 ]
        // [  0  -1   2 ]
        a.set(0, 0, 2.0);
        a.set(0, 1, -1.0);

        a.set(1, 0, -1.0);
        a.set(1, 1, 2.0);
        a.set(1, 2, -1.0);

        a.set(2, 1, -1.0);
        a.set(2, 2, 2.0);

        let b = [1.0, 0.0, 1.0];
        let mut x = [0.0; 3];

        let zero_tol = 1e-12;
        let ok = a.lu_decompose_inplace(zero_tol);
        assert!(ok, "LU decomposition failed");

        let ok = a.lu_solve(&b, &mut x, zero_tol);
        assert!(ok, "LU solve failed");

        // 기대 해: [1, 1, 1]
        for (i, &xi) in x.iter().enumerate() {
            assert!((xi - 1.0).abs() < 1e-10, "x[{}] = {}, expected 1.0", i, xi);
        }
    }

    #[test]
    fn test_banded_zero_pivot_fails() {
        let n = 4;
        let kl = 1;
        let ku = 1;
        let mut a = BandedMatrix::new(n, kl, ku);

        // 대각을 0으로 두어 pivot 실패 유도
        a.set(0, 0, 0.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 2.0);
        a.set(1, 2, 1.0);
        a.set(2, 1, 1.0);
        a.set(2, 2, 2.0);
        a.set(2, 3, 1.0);
        a.set(3, 2, 1.0);
        a.set(3, 3, 2.0);

        let ok = a.lu_decompose_inplace(1e-12);
        assert!(!ok, "Expected LU decomposition to fail due to zero pivot");
    }

    #[test]
    #[should_panic]
    fn test_setting_nonzero_outside_band_panics() {
        let mut a = BandedMatrix::new(5, 1, 1); // tridiagonal
        // (0,2)는 band 밖. non-zero set은 panic
        a.set(0, 2, 1.0);
    }

    #[test]
    fn test_banded_random_compare_dense_partial_pivot() {
        let n = 10;
        let kl = 2;
        let ku = 2;
        let zero_tol = 1e-12;

        let mut rng = XorShift64::new(0x1234_5678_9abc_def0);
        let mut a = BandedMatrix::new(n, kl, ku);

        // 랜덤 band 채우기 + 대각 우세로 무피벗 LU 안정화
        for r in 0..n {
            let c0 = r.saturating_sub(kl);
            let c1 = (r + ku).min(n - 1);
            let mut rowsum = 0.0;
            for c in c0..=c1 {
                if r == c {
                    continue;
                }
                let v = rng.next_range(-0.5, 0.5);
                a.set(r, c, v);
                rowsum += v.abs();
            }
            // 강한 대각 우세
            a.set(r, r, rowsum + 2.0);
        }

        // b 생성
        let mut b = vec![0.0; n];
        for i in 0..n {
            b[i] = rng.next_range(-1.0, 1.0);
        }

        // dense 비교 해
        let dense = a.to_dense();
        let x_ref = dense_solve_partial_pivot(&dense, &b, zero_tol).expect("dense solve failed unexpectedly");

        // band LU + solve
        let mut a_lu = a.clone();
        let ok = a_lu.lu_decompose_inplace(zero_tol);
        assert!(ok, "banded LU failed (unexpected for diagonally dominant)");

        let mut x = vec![0.0; n];
        let ok = a_lu.lu_solve(&b, &mut x, zero_tol);
        assert!(ok, "banded solve failed");

        let err = max_abs_diff(&x, &x_ref);
        assert!(err < 1e-8, "banded x differs from dense pivoted x too much: err={}", err);
    }

    #[test]
    fn test_lu_solve_with_workspace_no_alloc() {
        let n = 6;
        let kl = 1;
        let ku = 1;
        let zero_tol = 1e-12;

        let mut a = BandedMatrix::new(n, kl, ku);
        // 간단한 SPD tri-diagonal
        for i in 0..n {
            a.set(i, i, 2.0);
            if i + 1 < n {
                a.set(i, i + 1, -1.0);
                a.set(i + 1, i, -1.0);
            }
        }

        let mut a_lu = a.clone();
        assert!(a_lu.lu_decompose_inplace(zero_tol));

        let b = vec![1.0; n];
        let mut x = vec![0.0; n];
        let mut y = vec![0.0; n];

        assert!(a_lu.lu_solve_with_workspace(&b, &mut x, &mut y, zero_tol));

        // Ax ≈ b 확인(대충 잔차 체크)
        let dense = a.to_dense();
        let mut r_max = 0.0_f64;
        for i in 0..n {
            let mut s = 0.0_f64;
            for j in 0..n {
                s += dense[i][j] * x[j];
            }
            r_max = r_max.max((s - b[i]).abs());
        }
        assert!(r_max < 1e-9, "residual too large: {}", r_max);
    }
}
```
---
