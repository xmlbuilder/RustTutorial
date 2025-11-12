# on_make_frame_matrix

## 📐 함수 목적
- 기준점 o를 원점으로 하고,
- 벡터 ex, ey, ez_hint를 기반으로 **직교 좌표축 (x, y, z)** 를 구성하여,
- 4×4 변환 행렬을 생성합니다.

## 🔢 수학적 처리 흐름
### 1. X축 정규화
```rust
let x = ex.unitize();
```
$$
\mathbf{x}=\frac{\mathbf{e_{\mathnormal{x}}}}{\| \mathbf{e_{\mathnormal{x}}}\| }
$$

### 2. Y축 정규화 (X축에 수직 성분만 남김)
```rust
let y_raw = ey - x * Vector::dot(&ey, &x);
let mut y = y_raw.unitize();
```

$$
\mathbf{y_{\mathrm{raw}}}=\mathbf{e_{\mathnormal{y}}}-(\mathbf{e_{\mathnormal{y}}}\cdot \mathbf{x})\cdot \mathbf{x}
$$

$$
\mathbf{y}=\frac{\mathbf{y_{\mathrm{raw}}}}{\| \mathbf{y_{\mathrm{raw}}}\| }
$$


### 3. Y축 보정 (ey가 x와 거의 평행할 경우)
```rust
if !y.is_valid() || y.length() < 1e-14 {
    let y_alt = ex.cross(&ez_hint).cross(&ex);
    y = y_alt.unitize();
}
```

- 보정 벡터: $\mathbf{y_{\mathrm{alt}}}=(\mathbf{e_{\mathnormal{x}}}\times \mathbf{e_{\mathnormal{z}}})\times \mathbf{e_{\mathnormal{x}}}$
- 이중 외적을 통해 $\mathbf{e_{\mathnormal{x}}}$ 에 수직인 안정적인 벡터 생성

### 4. Z축 계산
```rust
let z = x.cross(&y).unitize();
```

$$
\mathbf{z}=\frac{\mathbf{x}\times \mathbf{y}}{\| \mathbf{x}\times \mathbf{y}\| }
$$


### 5. Y축 재정의 (Z축과 X축으로부터)
```rust
let y = z.cross(&x).unitize();
```

$$
\mathbf{y}=\frac{\mathbf{z}\times \mathbf{x}}{\| \mathbf{z}\times \mathbf{x}\| }
$$

- 이렇게 하면 x, y, z가 정확히 직교하게 됩니다.

### 6. 최종 변환 행렬 생성
```rust
Transform::from_cols(
    [x.x, x.y, x.z, 0.0],
    [y.x, y.y, y.z, 0.0],
    [z.x, z.y, z.z, 0.0],
    [o.x, o.y, o.z, 1.0],
)
```

- 최종 행렬:

$$
M=\left[ \begin{matrix}x_x&y_x&z_x&o_x\\ ,& x_y&y_y&z_y&o_y \\ ,& x_z&y_z&z_z&o_z \\ ,& 0&0&0&1\end{matrix}\right]
$$

- 이 행렬은 기준점 o를 원점으로 하고, x, y, z 축을 열 벡터로 갖는 좌표계 변환 행렬입니다.

## ✅ 수학적 타당성
- 모든 벡터는 정규화되어 있어 직교성 확보됨
- 외적을 통해 축을 생성하므로 오른손 좌표계 유지
- 보정 로직은 ey가 x와 거의 평행할 때 안정적인 y축을 생성함
- 최종 행렬은 동차 좌표계에서 변환 행렬로 사용 가능


```rust
pub fn on_make_frame_matrix(
    o: &Point,
    ex: &Vector,
    ey: &Vector,
    ez_hint: &Vector,
) -> Transform {
    let x = ex.unitize();
    // Y를 X에 수직 성분만 남겨 정규화
    let y_raw = ey - x * Vector::dot(&ey, &x);
    let mut y = y_raw.unitize();
    if !y.is_valid() || y.length() < 1e-14 {
        // ey가 좋지 않으면 (ex×ez)×ex 로 보정
        let y_alt = ex.cross(&ez_hint).cross(&ex);
        y = y_alt.unitize();
    }
    let z = x.cross(&y).unitize();
    let y = z.cross(&x).unitize();

    // ⚠️ Assumption: the following assumes a 4×4 constructor where columns represent axes and the last column is the origin
    // Use a constructor that matches your project’s Transform convention
    Transform::from_cols(
        [x.x, x.y, x.z, 0.0],
        [y.x, y.y, y.z, 0.0],
        [z.x, z.y, z.z, 0.0],
        [o.x, o.y, o.z, 1.0],
    )
}
```
