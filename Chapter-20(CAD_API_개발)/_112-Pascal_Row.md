# Pascal Row
## 🎯 결론부터: pascal_row는 이항계수(binomial coefficients) 를 만드는 함수다
- 즉,
- pascal_row(row, k) 는 다음을 만든다:
```math
{k \choose 0},\  {k \choose 1},\  {k \choose 2},\  \dots ,\  {k \choose k}
```
- 예를 들어:
  - k = 0 → [1]
  - k = 1 → [1, 1]
  - k = 2 → [1, 2, 1]
  - k = 3 → [1, 3, 3, 1]
  - k = 4 → [1, 4, 6, 4, 1]
- 즉, 파스칼 삼각형(Pascal’s Triangle) 의 한 행을 만드는 함수다.

## 🎯 왜 NURBS에서 이항계수가 필요할까?
- 유리 기저함수의 도함수는 다음과 같은 형태를 가진다:

```math
R_i^{(k)}(u)=\frac{1}{W(u)}\left( w_iN_i^{(k)}(u)-\sum _{j=1}^k{k \choose j}W^{(j)}(u)R_i^{(k-j)}(u)\right)
```
 
- 여기서 등장하는 ${k \choose j}$
  - 이게 바로 pascal_row가 만드는 이항계수다.
- 즉:
  - Rational basis 도함수는 단순한 미분이 아니라
  - Quotient Rule(나눗셈 미분) 의 고차 도함수 버전
  - 즉, Leibniz rule 을 사용한다
- Leibniz rule:
```math
(f/g)^{(k)}=\frac{1}{g}\left( f^{(k)}-\sum _{j=1}^k{k \choose j}g^{(j)}(f/g)^{(k-j)}\right)
``` 
- 이항계수는 여기서 반드시 필요하다.

## 🎯 C 코드에서 pascal_row가 쓰이는 위치
- C의 N_kntacd:
```cpp
for( j=1; j<=k; j++ )
{
    v -= tri[k][j] * d[j] * RD[k-j][i];
}
```

- 여기서 tri[k][j] 가 바로 이항계수다.
- Rust에서는:
```rust
pascal_row(&mut row, k);
...
v -= (row[j] as Real) * d[j] * rd[k - j];
```

- 즉:
  - row[j] = binomial(k, j)
  - d[j] = W^{(j)}(u)
  - rd[k-j] = R_i^{(k-j)}(u)
- 이 조합이 바로 유리 기저함수의 고차 도함수 공식이다.

## 🎯 요약: pascal_row 의 수학적 의미

| 항목 | 의미 | 수식 |
|------|------|-------|
| pascal_row | 파스칼 삼각형의 한 행 생성 |  |
| 생성되는 값 | 이항계수(binomial coefficients) | $\(\binom{k}{j}\)$ |
| 사용 목적 | 유리 기저함수 도함수 계산(Leibniz rule) |  |
| 등장 이유 | $R_i(u) = (w_i N_i) / W(u)$ 의 고차 도함수에서 필요 |  |
| 적용 위치 | $R_i^{(k)}(u)$ 계산 시 $\(\binom{k}{j} W^{(j)} R_i^{(k-j)}\)$ 항에 사용 |  |




## 🔥 직관적 예시
- 예를 들어 2차 도함수:
```math
R_i''(u)=\frac{1}{W}\left( w_iN_i''-2W'R_i'-W''R_i\right)
``` 
- 여기서 2 는 ${2 \choose 1}=2$
- 이항계수다.
- 3차 도함수는 더 복잡해지고 이항계수가 더 많이 등장한다.

---

## 코드
```rust
fn pascal_row(row: &mut [usize], k: usize) {
    debug_assert!(row.len() >= k + 1);
    row[0] = 1;
    for i in 1..=k {
        row[i] = row[i - 1] * (k + 1 - i) / i;
    }
}
```
---

