# Solving Linear Systems with `svdcmp` (SVD-based Least Squares)

- This note explains how to solve linear systems and least–squares problems using the outputs of our `svdcmp` routine.  
Formulas are written in GitHub‑friendly LaTeX.

---

## Setup and Notation

Given $\(A \in \mathbb{R}^{m\times n}\)$ and the SVD

$$
A = U \Sigma V^\top,
$$

where
- $\(U \in \mathbb{R}^{m\times n}\)$ has orthonormal columns $\((U^\top U = I_n)\)$,
- $\(V \in \mathbb{R}^{n\times n}\)$ is orthogonal $\((V^\top V = I_n)\)$,
- $\(\Sigma = \mathrm{diag}(\sigma_1,\dots,\sigma_n)\)$ with $\(\sigma_1 \ge \cdots \ge \sigma_n \ge 0\)$.

Our `svdcmp(&mut a, &mut w, &mut v)` returns:
- `a`  $\(\Rightarrow\)$ $\(U\)$ (thin, size $\(m\times n\)$ ),
- `w`  $\(\Rightarrow\)$ $\(\{\sigma_i\}_{i=1}^n\)$,
- `v`  $\(\Rightarrow\)$ $\(V\)$ ( size $\(n\times n\)$ ).

We are interested in solving $\(A x \approx b\)$ ( least squares if $\(m \gt n\)$ ).

---

## Pseudoinverse Solution (Minimum‑Norm Least Squares)

The Moore–Penrose pseudoinverse of $\(A\)$ is

$$
A^{+} = V \Sigma^{+} U^\top,
\qquad
\Sigma^{+} = \mathrm{diag} \big(\sigma_1^{+},\dots,\sigma_n^{+}\big),
$$

with

$$
\sigma_i^{+} =
\begin{cases}
1/\sigma_i, & \sigma_i \gt \tau,
0, & \sigma_i \le \tau,
\end{cases}
$$


where $\(\tau\)$ is a numerical tolerance (see **Tolerances** below).

Then the minimum‑norm least‑squares solution is

$$
x^{\*} = V \Sigma^{+} U^\top b.
$$

**Coordinate form.** Let $\(c_i = U[:,i]^\top b\)$. Then

$$
x^{\*} = \sum_{i:\ \sigma_i \gt \tau} \frac{c_i}{\sigma_i} V[:,i].
$$

Equivalently:

$$
\text{(i) } y_i =
\begin{cases}
c_i/\sigma_i,& \sigma_i \gt \tau,\\
0,& \sigma_i \le \tau,
\end{cases}
\quad
\text{(ii) } x^{\*} = V y.
$$

**Residual.** The residual $\(r = b - A x^{\*}\)$ lies in the orthogonal complement of $\(\mathrm{range}(A)\)$. 
In exact arithmetic,

$$
r = \sum_{i:\ \sigma_i \le \tau} (U[:,i]^\top b) U[:,i],
\qquad
\|r\|_2 = \Big(\sum_{i:\ \sigma_i \le \tau} (U[:,i]^\top b)^2\Big)^{1/2}.
$$

---

## Multiple Right‑Hand Sides

For $\(B \in \mathbb{R}^{m\times r}\)$, solve $\(\min_X \|AX - B\|_F\)$:

$$
Y = U^\top B \in \mathbb{R}^{n\times r},
\quad
\tilde Y_{i,:} =
\begin{cases}
Y_{i,:}/\sigma_i,& \sigma_i \gt \tau,\\
0,& \sigma_i \le \tau,
\end{cases}
\quad
X = V \tilde Y \in \mathbb{R}^{n\times r}.
$$

---

## Choosing the Tolerance $\(\tau\)$

A practical choice is

$$
\tau = \mathrm{rcond}\cdot \sigma_{\max},\qquad
\mathrm{rcond} \in [10^{-12},\ 10^{-6}]\ \text{(double)}.
$$

Common recipes:
- $\(\mathrm{rcond} = \epsilon \cdot \max(m,n)\)$, with machine epsilon $\(\epsilon \approx 2.22\times10^{-16}\)$.
- Or user‑set based on problem scale (noise level).

Small $\(\sigma_i\)$ $\((\le \tau)\)$ are effectively “numerical zero”; setting $\(\sigma_i^{+}=0\)$ stabilizes the solution and yields the minimum‑norm solution among all least‑squares minimizers.

---

## Truncated SVD (Rank‑ $\(k\)$ ) Solution

Keep only the first $\(k\)$ singular triplets $\((U_k,\Sigma_k,V_k)\)$. The rank‑$\(k\)$ solution is

$$
x_k = V_k \Sigma_k^{-1} U_k^\top b
= \sum_{i=1}^{k} \frac{U[:,i]^\top b}{\sigma_i} V[:,i].
$$

This minimizes $\(\|A x - b\|_2\)$ over all $\(x\)$ in the span of the top $\(k\)$ right singular vectors and implements the optimal low‑rank regularizer (Eckart–Young–Mirsky).

---

## Tikhonov / Ridge Regularization

Solve $\(\min_x \big(\|A x - b\|_2^2 + \lambda^2 \|x\|_2^2\big)\)$. With SVD,

$$
x_\lambda = V \Phi_\lambda U^\top b,
\qquad
\Phi_\lambda = \mathrm{diag} \Big( \frac{\sigma_i}{\sigma_i^2 + \lambda^2} \Big).
$$

Component‑wise filter form:

$$
x_\lambda = \sum_{i=1}^{n} \frac{\sigma_i}{\sigma_i^2 + \lambda^2} (U[:,i]^\top b) V[:,i].
$$

Larger $\(\lambda\)$ damps directions associated with small $\(\sigma_i\)$, trading bias for variance.

---

## Error and Conditioning at a Glance

- Spectral norm: $\(\lVert A\rVert_2 = \max_i \sigma_i\)$, Frobenius: $\(\|A\|_F = \big(\sum_i \sigma_i^2\big)^{1/2}\)$.
- Condition number: $\(\kappa_2(A) = \sigma_{\max}/\sigma_{\min}\)$ (or $\(\sigma_{\min\gt\tau}\)$ in floating‑point).
- Minimum‑norm property: among all least‑squares solutions, $\(x^{\*} = A^{+} b\)$ is the one with smallest $\(\|x\|_2\)$.

---

## Mapping to Code (`svdcmp` outputs)

Let `svdcmp(&mut a, &mut w, &mut v)` produce $\(U(=a), \sigma(=w), V(=v)\)$. Then for a single $\(b\)$:
1. **Project** $\(c_i = U[:,i]^\top b\)$.
2. **Scale** $\(y_i = c_i/\sigma_i\)$ if $\(\sigma_i \gt \tau\)$, else $\(0\)$.
3. **Back‑transform** $\(x = V y\)$.

This matches the snippet:
```rust
let mut y = vec![0.0; n];
for i in 0..n {
    let mut dot = 0.0;
    for r in 0..m {
        dot += a.at(r as i32, i as i32) * b[r]; // U[:,i] · b
    }
    let sigma = w[i].abs();
    y[i] = if sigma > tol { dot / sigma } else { 0.0 };
}

let mut x = vec![0.0; n];
for j in 0..n {
    let mut s = 0.0;
    for i in 0..n {
        s += v.at(j as i32, i as i32) * y[i];
    }
    x[j] = s;
}
```

---

## Complexity

- Once the SVD is available, solving one RHS costs $\(O(mn + n^2)\)$ (forming $\(U^\top b\)$ and multiplying by $\(V\)$ ).
- For $\(r\)$ RHS, cost is $\(O(r(mn + n^2))\)$ with simple batching (can be reduced by BLAS‑level operations).

---

## Sanity Checks (Unit Tests)

- Orthogonality: $\(\|U^\top U - I\|_F, \|V^\top V - I\|_F \lt 10^{-10}\)$.
- Reconstruction: $\(\|A - U\Sigma V^\top\|_F / \|A\|_F \ll 1\)$.
- Least squares: residual $\(\|b - A x^{\*}\|_2\)$ minimal, and $\(x^{\*}\)$ has minimal $\(\|x\|_2\)$ among minimizers.
- Rank‑deficient cases: components with $\(\sigma_i \le \tau\)$ are safely nulled.

---

## Summary

With `svdcmp` we have $\(A = U\Sigma V^\top\)$ . Solving $\(A x \approx b\)$ reduces to three stable steps:

$$
x^{\*} = V \Sigma^{+} U^\top b,\quad
\Sigma^{+}_{ii} =
\begin{cases}
1/\sigma_i,& \sigma_i \gt \tau,\\
0,& \sigma_i \le \tau.
\end{cases}
$$

This yields the minimum‑norm least‑squares solution; truncated SVD and Tikhonov filters fall out as simple modifications.
