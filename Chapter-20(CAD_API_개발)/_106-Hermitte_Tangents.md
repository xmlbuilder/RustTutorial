## Hermite Tangent 계산 수식 설명
### 1. 함수 목적
ComputeHermiteTangents는 주어진 점 배열 $Q=\{ Q_0,Q_1,\dots ,Q_{n-1}\}$ 에 대해  
**Hermite 보간용 접선 벡터(tangent vectors)** 를 계산하는 함수입니다.  
이 접선들은 곡선을 매끄럽게 이어주기 위해 각 점에서의 방향을 정의합니다.  

### 2. 주요 단계
#### (a) 폐곡선 여부 확인
```cpp
int nClosed = Q[0] == Q[count - 1] ? 1 : 0;
```
- 시작점과 끝점이 같으면 폐곡선(closed curve)으로 판단합니다.
- 폐곡선이면 양 끝점에서 특별한 처리 없이 내부 점들의 차분 벡터를 사용합니다.

#### (b) 차분 벡터 계산
```cpp
arTan[i+1] = Q[i] - Q[i-1];
```
- 인접한 점들의 차이를 이용해 **구간 벡터(segment vector)** 를 구합니다.
- 이는 각 구간의 기본 방향 벡터가 됩니다.

#### (c) 양 끝점 보정
- 폐곡선이 아닌 경우, 양 끝점에서 접선 벡터가 불안정할 수 있으므로 보정합니다.
-  앞쪽 끝점 보정
```cpp
if (2.0 * |arTan[2]|^2 > |arTan[3]|^2)
    arTan[1] = 2*arTan[2] - arTan[3];
else
    arTan[1] = 2*ratio*arTan[2] - arTan[3];
```

- 두 번째 구간 벡터와 세 번째 구간 벡터의 길이를 비교하여, 앞쪽 접선을 안정적으로 보정합니다.
- ratio = |arTan[3]| / |arTan[2]| 를 사용해 길이 비율에 따라 보정합니다.
- 뒤쪽 끝점 보정

```cpp
if (2.0 * |arTan[count]|^2 > |arTan[count-1]|^2)
    arTan[count+1] = 2*arTan[count] - arTan[count-1];
else
    arTan[count+1] = 2*ratio*arTan[count] - arTan[count-1];
```
- 끝점에서도 동일한 방식으로 보정합니다.

#### (d) 접선 벡터 계산
```cpp
double length1 = |arTan[i] × arTan[i+1]|;
double length2 = |arTan[i+2] × arTan[i+3]|;
double r = (length1 + length2 == 0) ? 1.0 : length1 / (length1 + length2);
tan = (1-r)*arTan[i+1] + r*arTan[i+2];
```

- **교차곱(cross product)** 를 이용해 인접 구간의 방향 차이를 측정합니다.
- $length1=|arTan[i]\times arTan[i+1]|$
- $length2=|arTan[i+2]\times arTan[i+3]|$
- 비율 r을 계산하여 두 구간 벡터를 가중 평균합니다.
- 결과 벡터를 단위화(unitize)하여 최종 접선으로 사용합니다.

## 3. 수식 요약
- 구간 벡터:
  
$$
v_i=Q_i-Q_{i-1}
$$

- 끝점 보정:

$$
v_1 =
\begin{cases}
  2v_2 - v_3, & \text{if } 2|v_2|^2 > |v_3|^2, \quad
  2\cdot \dfrac{|v_3|}{|v_2|} v_2 - v_3, & \text{otherwise}.
\end{cases}
$$

- 가중치 계산:

$$
r=\frac{|v_i\times v_{i+1}|}{|v_i\times v_{i+1}|+|v_{i+2}\times v_{i+3}|}
$$
- 접선 벡터:

$$
t_i=(1-r)v_{i+1}+rv_{i+2},\quad \hat {t}_i=\frac{t_i}{|t_i|}
$$

## 4. 의미
- 이 알고리즘은 Hermite 보간 곡선을 만들 때 각 점에서의 접선을 안정적으로 계산합니다.
- 끝점에서는 인접 벡터의 길이 비율을 고려해 보정하여 곡선이 매끄럽게 이어지도록 합니다.
- 교차곱을 이용한 가중치는 곡선의 **굽힘(curvature)** 을 반영하여 접선 방향을 자연스럽게 조정합니다.

- 👉 요약하면, 이 함수는 점 배열로부터 Hermite 보간에 필요한 접선 벡터를 계산하는 알고리즘이며,
  - 내부 점은 단순 차분 벡터,
  - 끝점은 보정된 벡터,
  - 최종 접선은 교차곱 기반 가중 평균으로 얻습니다.
 
---
### 소스 코드
```rust
pub fn compute_hermite_tangents(
    q: &SimpleArray<Point3D>,
    ar_tangent: &mut SimpleArray<Vector3D>,
    corner_end: bool,
) -> usize {
    let cnt = q.count();
    let n_closed = if q[0] == q[cnt - 1] { 1 } else { 0 };

    let mut ar_tan = SimpleArray::<Vector3D>::new();
    ar_tan.set_capacity(cnt + 3);
    ar_tan.data.resize(cnt + 3, Vector3D::default());

    ar_tangent.set_capacity(cnt);

    if n_closed != 0 && !corner_end {
        ar_tan[0] = (q[cnt - 2] - q[cnt - 3]).to_vec();
        ar_tan[1] = (q[cnt - 1] - q[cnt - 2]).to_vec();
        for index in 1..cnt {
            ar_tan[index + 1] = (q[index] - q[index - 1]).to_vec();
        }
        ar_tan[cnt + 1] = (q[1] - q[0]).to_vec();
        ar_tan[cnt + 2] = (q[2] - q[1]).to_vec();
    } else {
        for i in 1..cnt {
            ar_tan[i + 1] = (q[i] - q[i - 1]).to_vec();
        }

        if 2.0 * ar_tan[2].length_squared() > ar_tan[3].length_squared() {
            ar_tan[1] = ar_tan[2] * 2.0 - ar_tan[3];
            ar_tan[0] = ar_tan[1] * 2.0 - ar_tan[2];
        } else {
            let ratio = ar_tan[3].length() / ar_tan[2].length();
            ar_tan[1] = ar_tan[2] * (2.0 * ratio) - ar_tan[3];
            ar_tan[0] = ar_tan[1] * 2.0 - ar_tan[2] * ratio;
        }

        if 2.0 * ar_tan[cnt].length_squared() > ar_tan[cnt - 1].length_squared() {
            ar_tan[cnt + 1] = ar_tan[cnt] * 2.0 - ar_tan[cnt - 1];
            ar_tan[cnt + 2] = ar_tan[cnt + 1] * 2.0 - ar_tan[cnt];
        } else {
            let ratio = ar_tan[cnt - 1].length() / ar_tan[cnt].length();
            ar_tan[cnt + 1] = ar_tan[cnt] * (2.0 * ratio) - ar_tan[cnt - 1];
            ar_tan[cnt + 2] = ar_tan[cnt + 1] * 2.0 - ar_tan[cnt] * ratio;
        }
    }

    for i in 0..cnt {
        let length1 = ar_tan[i].cross(&ar_tan[i + 1]).length();
        let length2 = ar_tan[i + 2].cross(&ar_tan[i + 3]).length();
        let r = if (length1 + length2) == 0.0 {
            1.0
        } else {
            length1 / (length1 + length2)
        };
        let mut tan = ar_tan[i + 1] * (1.0 - r) + ar_tan[i + 2] * r;
        if !tan.normalize() {
            tan = Vector3D::zero();
        }
        ar_tangent.append(tan);
    }
    ar_tangent.count()
}
```
---

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::functions::compute_hermite_tangents;
    use nurbslib::core::prelude::{Point3D, Vector3D};
    use nurbslib::core::simple_array::SimpleArray;

    #[test]
    fn compute_hermite_tangents_test() {
        // 테스트용 점 배열 (직선과 곡선 혼합)
        let mut q = SimpleArray::<Point3D>::new();
        q.append(Point3D { x: 0.0, y: 0.0, z: 0.0 });
        q.append(Point3D { x: 1.0, y: 0.0, z: 0.0 });
        q.append(Point3D { x: 2.0, y: 1.0, z: 0.0 });
        q.append(Point3D { x: 3.0, y: 1.0, z: 0.0 });
        q.append(Point3D { x: 4.0, y: 0.0, z: 0.0 });

        // 결과를 담을 tangent 배열
        let mut tangents = SimpleArray::<Vector3D>::new();

        // Hermite tangent 계산
        let count = compute_hermite_tangents(&q, &mut tangents, false);

        println!("총 tangent 개수: {}", count);
        for (i, t) in tangents.data.iter().enumerate() {
            println!("Tangent {}: ({:.4}, {:.4}, {:.4})", i, t.x, t.y, t.z);
        }
    }
}
```
## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::functions::compute_hermite_tangents;
    use nurbslib::core::prelude::{Point3D, Vector3D};
    use nurbslib::core::simple_array::SimpleArray;

    #[test]
    fn compute_hermite_tangents_test() {
        // 테스트용 점 배열 (직선과 곡선 혼합)
        let mut q = SimpleArray::<Point3D>::new();
        q.append(Point3D { x: 0.0, y: 0.0, z: 0.0 });
        q.append(Point3D { x: 1.0, y: 0.0, z: 0.0 });
        q.append(Point3D { x: 2.0, y: 1.0, z: 0.0 });
        q.append(Point3D { x: 3.0, y: 1.0, z: 0.0 });
        q.append(Point3D { x: 4.0, y: 0.0, z: 0.0 });

        // 결과를 담을 tangent 배열
        let mut tangents = SimpleArray::<Vector3D>::new();

        // Hermite tangent 계산
        let count = compute_hermite_tangents(&q, &mut tangents, false);

        println!("총 tangent 개수: {}", count);
        for (i, t) in tangents.data.iter().enumerate() {
            println!("Tangent {}: ({:.4}, {:.4}, {:.4})", i, t.x, t.y, t.z);
        }
    }
}
```

### 1. Tangent 개수
- 입력한 점이 5개였으므로 tangent도 5개가 나오는 게 맞습니다.
- compute_hermite_tangents는 각 점마다 하나의 tangent를 생성하므로 총 tangent 개수: 5는 정상입니다.

### 2. Tangent 방향 분석
- Tangent 0: (0.9556, -0.2947, 0.0000)
  - 거의 x축 방향(오른쪽)인데 약간 아래쪽(-y)으로 기울어져 있습니다. 시작점에서 다음 점으로 가는 방향을 반영한 결과입니다.
- Tangent 1: (0.8944, 0.4472, 0.0000)
  - 오른쪽 위 대각선 방향. 두 번째 점에서 위쪽으로 꺾이는 구간을 잘 반영합니다.  
    (이 벡터는 길이가 1인 단위 벡터로, 실제로 (2,1)을 정규화한 값과 동일합니다.)
- Tangent 2: (0.8944, 0.4472, 0.0000)
  - 세 번째 점에서도 같은 방향이 나왔습니다. 이는 두 번째와 세 번째 구간이 같은 기울기를 가지기 때문에 tangent가 동일하게 계산된 것입니다.
- Tangent 3: (0.8944, -0.4472, 0.0000)
  - 오른쪽 아래 대각선 방향. 네 번째 점에서 곡선이 다시 아래로 꺾이는 구간을 반영합니다.
- Tangent 4: (0.5547, -0.8321, 0.0000)
  - 오른쪽보다 아래쪽 성분이 더 큰 방향. 마지막 점에서 곡선이 아래로 내려가는 흐름을 잘 잡아낸 결과입니다.

### 3. 정상성 확인
- 모든 tangent는 길이가 1에 가까운 단위 벡터입니다. (normalize()가 잘 적용됨)
- 각 tangent는 인접 구간의 방향을 반영하면서, 교차곱 기반 가중치로 곡선의 굽힘(curvature)을 고려한 결과입니다.
- 출력된 값들은 점 배열의 기하학적 배치와 일관성이 있습니다:
- 시작점: 거의 직선 → tangent 0은 x축 근처
- 중간점: 위로 꺾임 → tangent 1, 2는 위 대각선
- 다음점: 아래로 꺾임 → tangent 3은 아래 대각선
- 끝점: 더 강하게 아래로 → tangent 4는 아래쪽 성분이 큰 벡터

### ✅ 결론: 출력된 tangent 벡터들은 알고리즘과 점 배열의 기하학적 특성을 잘 반영하고 있으며, 정상적으로 계산된 결과입니다.

## Hermitte Tangent 이미지

![Hermitte Tangent](/image/Hermitte_Tangents.png)

