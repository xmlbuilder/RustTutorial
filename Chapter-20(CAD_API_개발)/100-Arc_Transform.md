# Arc Transform
## 📘 핵심 아이디어
- Arc는 Circle과 Interval로 구성됨
- Transform을 적용하면:
- circle → 평면, 중심, 반지름이 변환됨 (이미 Circle::transform 구현 있음)
- angle → 단순히 시작/끝 각도 구간이므로, 회전/스케일에 따라 달라지지 않음
- 이유: Arc는 circle 위의 특정 각도 범위를 나타내므로, circle이 변환되면 그 위의 동일한 각도 범위가 유지됨

## 🛠 구현
```rust
impl Arc {
    pub fn transform(&mut self, m: &Transform) {
        // 1) circle 변환
        self.circle.transform(m);

        // 2) angle은 그대로 둠
        // (Arc는 circle 위의 [t0, t1] 구간을 의미하므로,
        // circle이 변환되면 동일한 angle 구간이 유지됨)
    }
}
```

## 🔎 추가 고려사항
- 비균일 스케일링(non-uniform scaling):
    - circle이 ellipse로 변할 수 있음 → 현재 Circle::transform에서 radius를 heuristic으로 처리했듯, arc도 동일하게 따라감
- 회전/이동:
    - circle이 회전/이동하면 arc도 자동으로 따라감 (angle은 그대로)
- 복사 vs 변경:
    - &self로 두면 불변 참조라서 내부 변경 불가 → &mut self로 바꿔야 함

## ✅ 요약
- Arc::transform은 단순히 내부 circle.transform(m)을 호출하고, angle은 그대로 두면 됩니다.
- 즉, arc는 circle의 변환을 그대로 따라가고, angle 구간은 변하지 않습니다.
