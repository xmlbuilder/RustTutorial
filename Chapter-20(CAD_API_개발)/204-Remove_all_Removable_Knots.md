## Remove all Removable Knots Inplace
- **곡선 매듭 제거(knot removal)** 알고리즘.
- 한 줄 요약하면:
- **주어진 NURBS 곡선에서, 기하 오차가 허용 범위 안에서 제거 가능한 모든 매듭을 in-place로 최대한 제거하는 함수**


### 1. 함수의 목적과 시그니처
```rust
pub fn on_remove_all_removable_knots_inplace(
    curve: &mut NurbsCurve,
    mut tol: Real
) -> Result<bool>
```

- 입력:
    - curve: NURBS 곡선 (제어점 + 매듭 벡터 + 차수)
    - tol: 허용 기하 오차 (곡선이 원래 곡선에서 얼마나 벗어나도 되는지)
- 출력:
    - Ok(true) → 하나 이상 매듭이 실제로 제거됨
    - Ok(false) → 제거된 매듭 없음
- 역할:
    - 제거 가능한 매듭들을 찾아서, 제어점과 매듭 벡터를 직접 수정(in-place) 하며  
        가능한 한 많이 제거한다.

### 2. 기본 파라미터와 안전성 체크
```rust
let p = curve.degree as usize;
if p == 0 { return Ok(false); }

if curve.ctrl.len() < p + 1 || curve.kv.knots.len() < p + 2 {
    return Ok(false);
}

let mut n = curve.ctrl.len() - 1;     // 제어점 최대 인덱스
let mut m = curve.kv.knots.len() - 1; // 매듭 최대 인덱스
let ns = n;                           // 원래 제어점 개수 기억
```

- 차수 0이면 제거할 매듭 개념 자체가 의미 없음 → 종료
- 제어점/매듭 개수가 최소 조건 안 되면 종료
    - n: 제어점 인덱스 최댓값
    - m: 매듭 인덱스 최댓값

### 3. 유리 곡선(rational)일 때 tol 조정
```rust
let mut rat = false;
if curve.is_rational() {
    if let Some((wmin, _wmax, _pmin, pmax)) =
        curve.compute_min_max_weights_and_positions()
    {
        tol = (tol * wmin) / (1.0 + pmax);
        rat = true;
    }
}
```

- 유리 NURBS는 weight에 따라 기하 오차가 다르게 느껴지므로  
    원래 tol을 weight와 곡선 길이/범위에 맞게 스케일링한다.
- 대략적인 의도:
```math
\mathrm{tol_{eff}}=\frac{\mathrm{tol}\cdot w_{\min }}{1+p_{\max }}
```
- rat = true → 나중에 weight 범위 체크할 때 사용.

## 4. 보조 배열들: 에러/범위/기저함수 최대값
```rust
let mut br = vec![BIGD; m + 1];   // 각 매듭에 대한 "제거 에러 상한"
let mut sr = vec![0usize; m + 1]; // 각 매듭의 중복도(multiplicity)
let mut er = vec![0.0; m + 1];    // 각 span에서 누적 에러
let mut te = vec![0.0; m + 1];    // 임시 에러

let mut minl = vec![0.0; p + 1];
let mut maxl = vec![0.0; p + 1];
let mut minr = vec![0.0; p + 1];
let mut maxr = vec![0.0; p + 1];
let mut maxv = vec![0.0; p + 2];  // basis 함수 최대값
```

- br[i]: i번째 매듭을 제거했을 때의 기하 오차 상한(bound)
    - on_curve_knot_remove_error_bound(curve, i, sr[i])로 계산
- sr[i]: i번째 매듭의 multiplicity (같은 값이 몇 번 반복되는지)
- er[i]: 각 span에서 현재까지 누적된 에러
- te[i]: 이번에 제거를 시도할 때 임시로 계산한 에러
- minl/maxl/minr/maxr/maxv는  
    기저함수(N_i,p)의 최소/최대값을 span별로 추적해서
    **이 매듭을 제거했을 때 최악의 경우 에러가 얼마나 커질 수 있는지** 를 계산하는 데 쓰인다.

## 5. 각 매듭에 대한 초기 에러 상한 계산
```rust
{
    let u = &curve.kv.knots;
    let mut r = p + 1;
    while r <= n {
        let i0 = r;
        while r <= n && u[r] == u[r + 1] {
            r += 1;
        }
        sr[r] = r - i0 + 1; // multiplicity
        br[r] = on_curve_knot_remove_error_bound(curve, r, sr[r]);
        r += 1;
    }
}
```

- r는 내부 매듭 인덱스 (양 끝 p+1은 clamped라 제거 대상 아님)
- 같은 값이 연속된 구간을 찾아 multiplicity sr[r] 기록
- on_curve_knot_remove_error_bound로  
    **이 매듭을 s번 제거했을 때의 에러 상한** 을 계산해서 br[r]에 저장
- **어떤 매듭이 제거하기 쉬운지** 를 미리 평가해두는 것.

## 6. 메인 루프: 제거 가능한 매듭을 반복적으로 제거
```rust
let mut removed_any = false;

loop {
    if m < 2 * p + 1 { break; }

    // 1) 내부 매듭 중에서 br가 가장 작은 것 선택
    let mut r_best = p + 1;
    let mut b_best = br[r_best];
    let mut s_best = sr[r_best];

    for i in (p + 2)..=(m - p - 1) {
        if br[i] < b_best {
            b_best = br[i];
            s_best = sr[i];
            r_best = i;
        }
    }

    // 제거할 수 있는 매듭이 더 이상 없으면 종료
    if b_best == BIGD || b_best == NOREM {
        break;
    }

    let r = r_best;
    let s = s_best;
    let u = &curve.kv.knots;
    let mut rmf = true;
    ...
}
```

- m < 2*p+1이면 내부 매듭이 더 이상 없음 → 종료
- br[i]가 가장 작은 매듭 r을 선택
    - **제거했을 때 에러가 가장 작을 것으로 예상되는 매듭**
- b_best == BIGD 또는 NOREM이면 제거 불가 → 종료
- 이제 선택된 매듭 r에 대해  
    정말 제거해도 되는지, 에러가 tol 안에 드는지를 정밀하게 검사한다.

## 7. 기저함수 최대값 기반 에러 검증 (홀수/짝수 분기)
- p + s가 홀수냐 짝수냐에 따라  
    기저함수 조합 방식이 달라진다.
- on_compute_knot_minmax_per_span를 호출해서  
    특정 구간에서 B-spline basis의 최소/최대값을 구한다.
- 그걸 이용해 maxv[i] (각 span에서의 최대 기여량)를 계산하고,  
    te[i] = er[i] + maxv[i] * b_best로 **이번에 이 매듭을 제거했을 때 에러 상한** 을 갱신한다.
- 만약 어떤 span에서 te[i] > tol이면 → 이 매듭은 제거하면 안 됨.
- 핵심 수식 구조만 잡아보면:
- 홀수 케이스에서:
- 짝수 케이스에서는 maxv[0..p]만 사용.
- 그리고:
```rust
te[i] = er[i] + maxv[idx] * b_best;
if te[i] > tol { rmf = false; break; }
```

- rmf == true → 이 매듭은 기하 오차 tol 안에서 제거 가능
- rmf == false → 제거하면 안 됨 → br[r] = NOREM

## 8. 실제 매듭 제거: 새로운 제어점 계산
- 에러 검증을 통과하면, 이제 진짜로 매듭을 제거한다.
```rust
let fout = (2 * r - s - p) / 2;
let first = r - p;
let last = r - s;
let off = first - 1;

let mut rw = vec![Point4D::zero(); 2 * p];

rw[0] = curve.ctrl[off];
rw[last + 1 - off] = curve.ctrl[last + 1];

let mut i = first;
let mut j = last;
let mut ii = 1usize;
let mut jj = last - off;

while j > i {
    let alf = (u[i + p + 1] - u[i]) / (u[r] - u[i]);
    let oma = 1.0 - alf;

    let bet = (u[j + p + 1] - u[j]) / (u[j + p + 1] - u[r]);
    let omb = 1.0 - bet;

    rw[ii] = Point4D::linear_comb(curve.ctrl[i], alf, rw[ii - 1], oma);
    rw[jj] = Point4D::linear_comb(curve.ctrl[j], bet, rw[jj + 1], omb);

    i += 1;
    j -= 1;
    ii += 1;
    jj -= 1;
}
```

- 이 부분은 De Boor 역방향에 가까운 구조:
- 기존 제어점 Qw[i]들을 매듭 r을 제거한 후에도 곡선이 최대한 유지되도록  
    새로운 제어점 Rw[]로 재구성하는 과정.
- alf, bet는 knot 구간 비율:
```math
\alpha =\frac{U_{i+p+1}-U_i}{U_r-U_i},\quad \beta =\frac{U_{j+p+1}-U_j}{U_{j+p+1}-U_r}
```
- Rw는 좌/우에서 동시에 안쪽으로 들어오면서 대칭적으로 새로운 제어점을 만들어낸다.
- 홀수 케이스에서는 중간 점을 한 번 더 보정:
```rust
if ((p + s) % 2) == 1 {
    let k = (p + s + 1) / 2;
    let alf = (u[r] - u[r - k]) / (u[r - k + p + 1] - u[r - k]);
    let bet = (u[r] - u[r - k + 1]) / (u[r - k + p + 2] - u[r - k + 1]);
    let lam = alf / (alf + bet);
    let oml = 1.0 - lam;

    rw[jj + 1] = Point4D::linear_comb(rw[jj + 1], lam, rw[ii - 1], oml);
}
```


## 9. 유리 곡선일 때 weight 범위 체크
```rust
if rat {
    let mut wmin = BIGD;
    let mut wmax = SMAD;

    let mut i = first;
    let mut j = last;
    while j > i {
        let wi = rw[i - off].w;
        let wj = rw[j - off].w;
        ...
    }

    if wmin < WMIN || wmax > WMAX {
        br[r] = NOREM;
        continue;
    }
}
```
- 새로 만들어진 제어점들의 weight가 너무 작거나(WMIN보다 작음) 너무 크면(WMAX보다 큼)
    - 수치적으로 불안정해질 수 있으므로 제거 취소.

## 10. 새 제어점 저장 + 배열/매듭/에러 정보 갱신
```rust
// curve.ctrl에 rw를 대칭 복사
{
    let mut i = first;
    let mut j = last;
    while j > i {
        curve.ctrl[i] = rw[i - off];
        curve.ctrl[j] = rw[j - off];
        i += 1;
        j -= 1;
    }
}
```
```rust
// 에러/상태 배열 한 칸씩 당기기
for i in (r + 1)..=m {
    br[i - 1] = br[i];
    sr[i - 1] = sr[i];
    er[i - 1] = er[i];
}
```rust
// 매듭 한 칸씩 당기기
for i in (r + 1)..=m {
    curve.kv.knots[i - 1] = curve.kv.knots[i];
}
```
```rust
// 제어점 한 칸씩 당기기
for i in (fout + 1)..=n {
    curve.ctrl[i - 1] = curve.ctrl[i];
}
```
```rust
n -= 1;
m -= 1;

curve.ctrl.truncate(n + 1);
curve.kv.knots.truncate(m + 1);
...
removed_any = true;
```

- 실제로 매듭 r을 제거했으니:
    - 매듭 벡터에서 U[r] 제거
    - 제어점 배열에서 하나 제거
    - 관련 보조 배열들도 한 칸씩 당기고 truncate
    - n == p가 되면 더 이상 내부 매듭 없음 → 종료

## 11. 주변 매듭들의 에러 상한 재계산
```rust
let k_upd = std::cmp::max(r.saturating_sub(p), p + 1);
let l_upd = std::cmp::min(n, r + p - s);

for i in k_upd..=l_upd {
    if curve.kv.knots[i] != curve.kv.knots[i + 1] && br[i] != NOREM {
        br[i] = on_curve_knot_remove_error_bound(curve, i, sr[i]);
    }
}
```

- 매듭 하나 제거하면, 그 주변 매듭들의 **제거 가능성** 도 바뀐다.
- 그래서 r 주변의 인덱스 범위만 다시 on_curve_knot_remove_error_bound로 갱신.

## 12. 최종 정리
```rust
if n < ns {
    curve.ctrl.shrink_to_fit();
    curve.kv.knots.shrink_to_fit();
}

Ok(removed_any)
```

- 제어점/매듭이 실제로 줄어들었으면 메모리도 정리
- 하나라도 제거했으면 true, 아니면 false

- 전체 요약
- 이 함수는:
    - 각 매듭이 제거될 경우의 에러 상한을 미리 계산하고
    - 가장 에러가 적게 나는 매듭부터 하나씩 제거를 시도하고
    - 기저함수 최대값 기반으로 “정말 tol 안에 들어오는지”를 검증하고
    - 통과하면 De Boor 역연산으로 새로운 제어점을 계산하고
    - 매듭/제어점/에러 정보를 갱신하면서 반복해서
    - 더 이상 제거 가능한 매듭이 없을 때까지 진행한다.
- 즉,
**곡선의 형상을 최대한 유지하면서, 불필요한 매듭과 제어점을 줄이는 최적화 루틴**.

---

```rust
pub fn on_remove_all_removable_knots_inplace(curve: &mut NurbsCurve, mut tol: Real) -> Result<bool> {
    const NOREM: Real = 1.0e25;
    const CTO: Real = 1.0e-5;
    const WMIN: Real = 1.0e-02;
    const WMAX: Real  = 20.0;

    // 너 프로젝트의 BIGD/SMAD가 있으면 그걸 쓰는 게 좋고,
    // 없으면 안전하게 큰 값/작은 값으로 둔다.
    const BIGD: Real = 1.0e100;
    const SMAD: Real = -1.0e100;

    let p = curve.degree as usize;
    if p == 0 {
        return Ok(false);
    }
    if curve.ctrl.len() < p + 1 || curve.kv.knots.len() < p + 2 {
        return Ok(false);
    }

    // local notation
    let mut n = curve.ctrl.len() - 1;       // highest ctrl index
    let mut m = curve.kv.knots.len() - 1;   // highest knot index
    let ns = n;

    // --- Adjust removal tolerance in case of rational curves ---
    let mut rat = false;
    if curve.is_rational() {
        if let Some((wmin, _wmax, _pmin, pmax)) = curve.compute_min_max_weights_and_positions() {
            // C: tol = (tol*wmin)/(1.0+pmax);
            tol = (tol * wmin) / (1.0 + pmax);
            rat = true;
        }
    }

    let lto = CTO * (curve.kv.knots[m] - curve.kv.knots[0]).abs();

    // --- local memory (br,sr,er,te) ---
    // "highest index" 기준으로 할당. Rust는 length = m+1
    let mut br = vec![BIGD; m + 1];
    let mut sr = vec![0usize; m + 1];
    let mut er = vec![0.0 as Real; m + 1];
    let mut te = vec![0.0 as Real; m + 1];

    // basis extrema buffers (minl/maxl/minr/maxr/max)
    let mut minl = vec![0.0 as Real; p + 1];
    let mut maxl = vec![0.0 as Real; p + 1];
    let mut minr = vec![0.0 as Real; p + 1];
    let mut maxr = vec![0.0 as Real; p + 1];
    let mut maxv = vec![0.0 as Real; p + 2]; // max[p+1]까지 씀

    // --- Compute knot removal error bounds for each distinct knot  ---
    // r = p+1; while r <= n:
    {
        let u = &curve.kv.knots;
        let mut r = p + 1;
        while r <= n {
            let i0 = r;
            while r <= n && u[r] == u[r + 1] {
                r += 1;
            }
            sr[r] = r - i0 + 1;
            br[r] = on_curve_knot_remove_error_bound(curve, r, sr[r]);
            r += 1;
        }
    }

    // --- Main loop: try to remove knots until no more removable ---
    let mut removed_any = false;

    loop {
        // Find knot with smallest error bound among internal knots: i = p+1..=m-p-1
        if m < 2 * p + 1 {
            break;
        }

        let mut r_best = p + 1;
        let mut b_best = br[r_best];
        let mut s_best = sr[r_best];

        for i in (p + 2)..=(m - p - 1) {
            if br[i] < b_best {
                b_best = br[i];
                s_best = sr[i];
                r_best = i;
            }
        }

        // If no more removable knot -> finished
        if b_best == BIGD || b_best == NOREM {
            break;
        }

        let r = r_best;
        let s = s_best;
        let u = &curve.kv.knots;

        // ---- Compute maximum(s) of basis functions over each span ----
        // rmf = TRUE; then compute k,l and maxv[]
        let mut rmf = true;

        if ((p + s) % 2) == 1 {
            // odd
            // k = (p+s+1)/2;
            let k = (p + s + 1) / 2;
            // l = r-k+p+1;
            let l = r - k + p + 1;

            // alf = (U[r]-U[r-k])/(U[r-k+p+1]-U[r-k]);
            let alf = (u[r] - u[r - k]) / (u[r - k + p + 1] - u[r - k]);
            // bet = (U[r]-U[r-k+1])/(U[r-k+p+2]-U[r-k+1]);
            let bet = (u[r] - u[r - k + 1]) / (u[r - k + p + 2] - u[r - k + 1]);
            let omb = 1.0 - bet;
            // lam = alf/(alf+bet);
            let lam = alf / (alf + bet);
            let oml = 1.0 - lam;

            // left basis extrema
            let left = on_compute_knot_minmax_per_span(&curve.kv, (r - k) as Index, p as Degree, lto);
            if let Ok((mn, mx, _)) = left {
                minl.copy_from_slice(&mn[..]);
                maxl.copy_from_slice(&mx[..]);
            } else {
                for i in 0..=p {
                    minl[i] = 0.0;
                    maxl[i] = 1.0;
                }
            }

            // right basis extrema
            let right = on_compute_knot_minmax_per_span(&curve.kv, (r - k + 1) as Index, p as Degree, lto);
            if let Ok((mn, mx, _)) = right {
                minr.copy_from_slice(&mn[..]);
                maxr.copy_from_slice(&mx[..]);
            } else {
                for i in 0..=p {
                    minr[i] = 0.0;
                    maxr[i] = 1.0;
                }
            }

            // maxv[0] = lam*alf*maxl[0];
            maxv[0] = (lam * alf * maxl[0]).abs();

            for i in 1..=p {
                minl[i] *= lam * alf;
                maxl[i] *= lam * alf;

                minr[i - 1] *= oml * omb;
                maxr[i - 1] *= oml * omb;

                let a = (maxl[i] - minr[i - 1]).abs();
                let b = (maxr[i - 1] - minl[i]).abs();
                maxv[i] = if a > b { a } else { b };
            }

            maxv[p + 1] = (oml * omb * maxr[p]).abs();

            // ---- Check the error (C loop: i=r-k..l) ----
            for i in (r - k)..=l {
                if u[i] != u[i + 1] {
                    let idx = i - (r - k); // 0..p+1
                    te[i] = er[i] + maxv[idx] * b_best;
                    if te[i] > tol {
                        rmf = false;
                        break;
                    }
                }
            }

            if rmf {
                // accept: er = te
                for i in (r - k)..=l {
                    if u[i] != u[i + 1] {
                        er[i] = te[i];
                    }
                }
            }
        } else {
            // even
            let k = (p + s) / 2;
            let l = r - k + p;

            // maxv[0..=p] will store max per span
            let one = on_compute_knot_minmax_per_span(&curve.kv, (r - k) as Index, p as Degree, lto);
            if let Ok((_mn, mx, _)) = one {
                // mx len = p+1
                for i in 0..=p {
                    maxv[i] = mx[i];
                }
            } else {
                for i in 0..=p {
                    maxv[i] = 1.0;
                }
            }

            for i in (r - k)..=l {
                if u[i] != u[i + 1] {
                    let idx = i - (r - k); // 0..p
                    te[i] = er[i] + maxv[idx] * b_best;
                    if te[i] > tol {
                        rmf = false;
                        break;
                    }
                }
            }

            if rmf {
                for i in (r - k)..=l {
                    if u[i] != u[i + 1] {
                        er[i] = te[i];
                    }
                }
            }
        }

        if !rmf {
            // Knot is not removable
            br[r] = NOREM;
            continue;
        }

        // ---- Remove knot: compute new control points for one removal step ----
        // C:
        // fout = (2*r-s-p)/2; first=r-p; last=r-s; off=first-1;
        let fout = (2 * r - s - p) / 2;
        let first = r - p;
        let last = r - s;
        let off = first - 1;

        // Rw size = 2*p (C alloc)
        let mut rw = vec![Point4D::zero(); 2 * p];

        // init ends
        rw[0] = curve.ctrl[off];
        rw[last + 1 - off] = curve.ctrl[last + 1];

        let mut i = first;
        let mut j = last;
        let mut ii = 1usize;
        let mut jj = last - off;

        // while (j-i) > 0
        while j > i {
            let alf = (u[i + p + 1] - u[i]) / (u[r] - u[i]);
            let oma = 1.0 - alf;

            let bet = (u[j + p + 1] - u[j]) / (u[j + p + 1] - u[r]);
            let omb = 1.0 - bet;

            // Rw[ii] = alf*Qw[i] + oma*Rw[ii-1]
            rw[ii] = Point4D::linear_comb(curve.ctrl[i], alf, rw[ii - 1], oma);

            // Rw[jj] = bet*Qw[j] + omb*Rw[jj+1]
            rw[jj] = Point4D::linear_comb(curve.ctrl[j], bet, rw[jj + 1], omb);

            i += 1;
            j -= 1;
            ii += 1;
            jj -= 1;
        }

        // ---- Check for disallowed weights (rat==YES) ----
        if rat {
            let mut wmin = BIGD;
            let mut wmax = SMAD;

            let mut i = first;
            let mut j = last;
            while j > i {
                let wi = rw[i - off].w;
                let wj = rw[j - off].w;
                if wi < wmin { wmin = wi; }
                if wj < wmin { wmin = wj; }
                if wi > wmax { wmax = wi; }
                if wj > wmax { wmax = wj; }
                i += 1;
                j -= 1;
            }

            // WMIN/WMAX는 네 프로젝트 전역 상수라고 가정
            if wmin < WMIN || wmax > WMAX {
                br[r] = NOREM;
                continue;
            }
        }

        // ---- Save new control points ----
        // if (p+s)%2 odd -> average mid
        if ((p + s) % 2) == 1 {
            // Rw[jj+1] = lam*Rw[jj+1] + oml*Rw[ii-1]
            // 여기서 lam/oml은 위 odd-branch에서 계산했었는데,
            // Rust에선 다시 계산 (C도 동일 파라미터)
            let k = (p + s + 1) / 2;
            let alf = (u[r] - u[r - k]) / (u[r - k + p + 1] - u[r - k]);
            let bet = (u[r] - u[r - k + 1]) / (u[r - k + p + 2] - u[r - k + 1]);
            let lam = alf / (alf + bet);
            let oml = 1.0 - lam;

            rw[jj + 1] = Point4D::linear_comb(rw[jj + 1], lam, rw[ii - 1], oml);
        }

        // copy symmetric into curve.ctrl
        {
            let mut i = first;
            let mut j = last;
            while j > i {
                curve.ctrl[i] = rw[i - off];
                curve.ctrl[j] = rw[j - off];
                i += 1;
                j -= 1;
            }
        }

        // ---- Shift down some parameters ----
        // if s==1 er[r-1] = max(er[r-1],er[r])
        if s == 1 {
            let a = er[r - 1];
            let b = er[r];
            er[r - 1] = if a > b { a } else { b };
        }
        // if s>1 sr[r-1] = sr[r]-1
        if s > 1 {
            sr[r - 1] = sr[r].saturating_sub(1);
        }

        // shift down br/sr/er (i=r+1..=m)
        for i in (r + 1)..=m {
            br[i - 1] = br[i];
            sr[i - 1] = sr[i];
            er[i - 1] = er[i];
        }

        // ---- Shift down knots and control points ----
        // knots: for i=r+1..=m U[i-1]=U[i]
        for i in (r + 1)..=m {
            curve.kv.knots[i - 1] = curve.kv.knots[i];
        }

        // ctrl: for i=fout+1..=n Qw[i-1]=Qw[i]
        for i in (fout + 1)..=n {
            curve.ctrl[i - 1] = curve.ctrl[i];
        }

        // update sizes
        n -= 1;
        m -= 1;

        // truncate vectors to new sizes
        curve.ctrl.truncate(n + 1);
        curve.kv.knots.truncate(m + 1);

        br.truncate(m + 1);
        sr.truncate(m + 1);
        er.truncate(m + 1);
        te.truncate(m + 1);

        removed_any = true;

        // If no more internal knots -> finished (C: if n == p break)
        if n == p {
            break;
        }

        // ---- Update error bounds near removed knot  ----
        // k = max(r-p, p+1), l = min(n, r+p-s)
        let k_upd = std::cmp::max(r.saturating_sub(p), p + 1);
        let l_upd = std::cmp::min(n, r + p - s);

        for i in k_upd..=l_upd {
            if curve.kv.knots[i] != curve.kv.knots[i + 1] && br[i] != NOREM {
                br[i] = on_curve_knot_remove_error_bound(curve, i, sr[i]);
            }
        }
        // continue loop
    }

    // Rust는 truncate로 이미 컴팩트해짐. 필요하면 shrink_to_fit 정도.
    if n < ns {
        curve.ctrl.shrink_to_fit();
        curve.kv.knots.shrink_to_fit();
    }

    Ok(removed_any)
}
```
---
