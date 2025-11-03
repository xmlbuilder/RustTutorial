# 🧠 wrapping_sub란?
```rust
let m = knots.len().wrapping_sub(1);
```

```rust
if pu > m { return None; }
```

- 1wrapping_sub` 는 언더플로우가 발생해도 panic 없이 값을 wrap-around시킵니다.
- 예: 0usize.wrapping_sub(1) → `usize::MAX` 로 wrap됨
- 일반적인 - 1 연산은 usize에서 언더플로우 시 panic을 유발할 수 있음

## 🔍 왜 사용했을까?
- knots.len()이 0일 경우 len() - 1은 panic
- wrapping_sub(1)을 쓰면 panic 없이 m == usize::MAX가 되어 이후 조건에서 `pu > m` 로 걸러짐
- ✅ 안전한 `언더플로우 방지` 용 처리

---
