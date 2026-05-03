# Bernstein Basis & Derivatives (Practical Notes)

## 1. Definition

Bernstein basis of degree p:

B_{i,p}(t) = C(p, i) * t^i * (1 - t)^(p - i)

where:
- i = 0..p
- t ∈ [0,1]

---

## 2. Key Properties

### Partition of Unity
∑ B_{i,p}(t) = 1

### Positivity
B_{i,p}(t) ≥ 0

### Endpoint interpolation
B_{0,p}(0) = 1  
B_{p,p}(1) = 1

---

## 3. Recursive Form (de Casteljau)

B_{i,p}(t) =
(1 - t) * B_{i,p-1}(t) + t * B_{i-1,p-1}(t)

---

## 4. First Derivative

d/dt B_{i,p}(t) = p * (B_{i-1,p-1}(t) - B_{i,p-1}(t))

Important property:

∑ B'_{i,p}(t) = 0

---

## 5. Higher Derivatives

General form:

d^r/dt^r B_{i,p}(t) =
p! / (p-r)! *
∑_{k=0}^{r} (-1)^k C(r,k) B_{i-r+k, p-r}(t)

---

## 6. Bezier Curve Evaluation

Given control points P_i:

C(t) = ∑ B_{i,p}(t) * P_i

C'(t) = ∑ B'_{i,p}(t) * P_i

C''(t) = ∑ B''_{i,p}(t) * P_i

---

## 7. Important Numerical Behavior

### Sum rules

- ∑ B = 1
- ∑ B' = 0
- ∑ B'' = 0

### Floating point issue

Due to cancellation:

large - large ≈ small error

Typical error:
- 1e-12 ~ 1e-6 depending on degree

---

## 8. Practical Tips

✔ Use tolerance ~1e-6 for derivative sum tests  
✔ Use Kahan summation for stability  
✔ Prefer recursive derivative for high degree  

---

## 9. Summary

Bernstein basis is:

✔ stable  
✔ geometrically intuitive  
✔ essential for Bezier / NURBS  

But:

⚠ derivative evaluation is numerically sensitive
