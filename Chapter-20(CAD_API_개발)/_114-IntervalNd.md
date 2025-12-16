## IntervalNd rust source documentation
- 이 문서는 N차원 도메인 관리 구조체 ExtentNd와 보조 타입 Interval의 설계 의도, 함수별 동작, 단계적 처리 흐름을 정리합니다. 
- 목표는 다차원 파라메트릭 공간에서 값의 경계 관리(클램프, 주기적 래핑)와 디버깅을 간결하고 일관되게 지원하는 것입니다.

### Data structures
### Interval
- Purpose: 1차원 구간을 표현합니다. 방향성(t0≤t1 또는 t0>t1)을 허용하며, 포함·클램프·래핑 연산을 제공합니다.
- Fields:
  - t0: 구간의 시작 파라미터
  - t1: 구간의 끝 파라미터
- Core invariants:
  - Validity: 길이가 0이어도 허용되지만, 래핑 시에는 길이 l=t1−t0>0이 필요합니다.
  - Min/Max: 내부적으로 방향을 고려해 min()과 max()를 계산합니다.
### Key methods
- new(t0, t1): 구간 생성.
- length():

$$ 
\mathrm{length}=|t_1-t_0|
$$
- contains(x): x∈[min(), max()] 여부 반환.
- clamp(x): x를 [min(), max()]로 제한.
- periodic_wrap(x): 길이 l=t1−t0에 대해 x를 구간 t0, t0+l)로 래핑.

$$
k=\left\lfloor \frac{x-t_0}{l}\right\rfloor ,\quad y=x-k\cdot l
$$

- min()/max(): 방향에 관계없이 하한/상한을 반환.

### IntervalNd
- Purpose: N차원 도메인(각 차원별 Interval)을 벡터로 관리하고, 다차원 입력 벡터를  
  차원별 규칙에 따라 클램프 또는 주기적 래핑합니다.
- Fields:
  - dim: 차원 수
  - extents: 길이 dim의 Interval 벡터

### Public API
- Constructor
  - new(dim): 길이가 dim인 도메인을 생성하고 각 차원을 기본 구간 [0,1]로 초기화합니다.
#### Domain operations
- clamp_vector(input, periodic): 입력 벡터를 각 차원 구간에 맞게 변환합니다.
  - Parameters:
    - input: 길이 dim의 실수 벡터
    - periodic: 길이 dim의 bool 슬라이스(옵션). true인 차원은 래핑, false인 차원은 클램프 처리
- Returns: (transformed, counts)
    - transformed: 변환된 벡터
    - counts: Some(vec![u64; dim]) 또는 None. 비주기(clamp) 차원에서 경계 밖 → 경계로 조정된 횟수 증가
- Behavior:
  - Periodic=true: Interval::periodic_wrap(x)를 적용합니다.
  - Periodic=false: Interval::clamp(x)를 적용합니다.
  - Counts: periodic 차원에서는 out-of-bounds를 카운트하지 않습니다.  
    clamp 차원에서만 (clamped−original)≠0이면 카운트합니다.
- dump(): 도메인 상태를 표준 출력에 인쇄합니다.
- Display impl: 문자열로 포맷해 출력 가능.

### Step-by-step function behavior
#### clamp_vector(input, periodic)
- Preconditions:
  - Input length check: input.len()==dim
  - Periodicity length check: periodic.is_some()이면 periodic.unwrap().len()==dim
- Initialization:
  - Output vector: out.reserve(dim)
  - Counts: periodic가 Some이면 counts=Some(vec![0; dim]), 아니면 None
- Loop for each dimension i:
  - Load: val=input[i], ext=&extents[i]
  - Periodic flag: is_periodic=periodic.map(pp[i]).unwrap_or(false)
- Transform:
  - If periodic:
    - wrap: out.push(ext.periodic_wrap(val))
    - counts: unchanged
  - If not periodic:
    - clamp: let clamped=ext.clamp(val); out.push(clamped)
    - counts: if (clamped−val).abs()>ε then counts[i]++
- Return: (out, counts)


#### periodic_wrap formula
- Domain length: l=t1−t0. l≤0이면 x를 그대로 반환(래핑 불가).
- Index: k=floor((x−t0)/l)
- Wrapped: y=x−k·l ∈ t0, t0+l)

#### clamp behavior
- Closed interval: [min(), max()]에 대해 x.clamp(min(), max())를 적용합니다.
- Direction agnostic: t0>t1인 경우에도 min/max로 처리하므로 안전합니다.

---

## Examples
Basic clamping and wrapping
- Domain: dim=3, extents[0]=[-1,1], extents[1]=[0,10], extents[2]=[0,2]
- Input: [2.5, −5.0, 3.5]
- Periodic flags: [false, true, false]
- Process:
  - 0: clamp 2.5→1.0, count+=1
  - 1: wrap −5.0 on [0,10] → 5.0, count unchanged
  - 2: clamp 3.5→2.0, count+=1
- Result: Clamped=[1.0, 5.0, 2.0], Counts=Some([1,0,1])

### Design notes
- Dimension flexibility: Curve(1D), Surface(2D)뿐 아니라 최적화·시뮬레이션에서 임의 N차원 파라메트릭 공간을 다루기 위해 일반화했습니다.
- Direction robustness: Interval이 감소 구간(t0>t1)을 허용하므로, 역방향 도메인에도 안전합니다.
- Periodic semantics: 래핑은 **구간 길이 l를 기준으로 t0로부터 상대 위치를 유지** 하도록 정의했습니다.  
  카운트는 “경계 위반을 클램프한 경우”에만 의미가 있어 periodic 차원에서는 증가시키지 않습니다.
- Performance: Vec<Interval>로 간결하게 관리하며, 반복당 상수 시간 연산입니다.

### Error handling and edge cases
- Zero-length intervals: l=0이면 periodic_wrap은 입력을 그대로 반환합니다. clamp는 min==max로 수렴합니다.
- NaN/Inf inputs: periodic_wrap/clamp 내부에서 특별히 거르지 않으므로 호출 전 유효성 확인을 권장합니다.
- Out-of-bounds counting epsilon: 현재 f64::EPSILON을 사용합니다. 필요 시 도메인별 허용 오차로 파라미터화할 수 있습니다.

### Extension ideas
- Tolerance-aware clamp: includes_with_tol(tol)와 연동해 카운트 조건을 tol 기반으로 조정.
- Per-dimension policies: 각 차원별로 clamp/periodic 설정 외에 “mirror”, “saturate with bias” 같은 정책 추가.
- Normalized mapping: Interval의 normalized_parameter_at/parameter_at과 결합해 0–1 공간과 도메인 간 왕복 변환 유틸 추가.
- Batch API: 여러 입력 벡터를 한 번에 처리하는 메서드로 throughput 향상.

---
## 소스 코드
```rust
use std::fmt;
use crate::core::prelude::Interval;

/// N차원 도메인
#[derive(Debug, Clone)]
pub struct IntervalNd {
    pub dim: usize,
    pub extents: Vec<Interval>,
}

impl IntervalNd {
    pub fn new(dim: usize) -> Self {
        let mut extents = Vec::with_capacity(dim);
        for _ in 0..dim {
            extents.push(Interval::new(0.0, 1.0));
        }
        Self { dim, extents }
    }

    /// 벡터를 클램프/래핑
    pub fn clamp_vector(
        &self,
        input: &[f64],
        periodic: Option<&[bool]>,
    ) -> (Vec<f64>, Option<Vec<u64>>) {
        assert_eq!(input.len(), self.dim);
        if let Some(p) = periodic {
            assert_eq!(p.len(), self.dim);
        }

        let mut out = Vec::with_capacity(self.dim);
        let mut counts = if periodic.is_some() { Some(vec![0u64; self.dim]) } else { None };

        for i in 0..self.dim {
            let val = input[i];
            let ext = &self.extents[i];
            let is_periodic = periodic.map(|p| p[i]).unwrap_or(false);

            if is_periodic {
                out.push(ext.periodic_wrap(val));
            } else {
                let clamped = ext.clamp(val);
                out.push(clamped);
                if let Some(ref mut c) = counts {
                    if (clamped - val).abs() > f64::EPSILON {
                        c[i] += 1;
                    }
                }
            }
        }
        (out, counts)
    }

    /// 디버그 출력
    pub fn dump(&self) {
        println!("ExtentNd: dim = {}", self.dim);
        for (i, ext) in self.extents.iter().enumerate() {
            println!("\t[{}] = [{}, {}]", i, ext.t0, ext.t1);
        }
    }
}

impl fmt::Display for IntervalNd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ExtentNd(dim={})", self.dim)?;
        for (i, ext) in self.extents.iter().enumerate() {
            writeln!(f, "  [{}] = [{}, {}]", i, ext.t0, ext.t1)?;
        }
        Ok(())
    }
}
```

---

## 테스트 코드
```rust
#[cfg(test)]
mod test {
    use nurbslib::core::interval_nd::IntervalNd;
    use nurbslib::core::prelude::Interval;

    #[test]
    fn main() {
        let mut dom = IntervalNd::new(3);
        dom.extents[0] = Interval::new(-1.0, 1.0);
        dom.extents[1] = Interval::new(0.0, 10.0);
        dom.extents[2] = Interval::new(0.0, 2.0);

        let input = vec![2.5, -5.0, 3.5];
        let periodic = vec![false, true, false];
        let (clamped, counts) = dom.clamp_vector(&input, Some(&periodic));

        println!("Input: {:?}", input);
        println!("Clamped: {:?}", clamped);
        println!("OutOfBounds counts: {:?}", counts);
        dom.dump();
    }
}
```

### 📌 상황 정리
- 도메인 정의
  - 0번 차원: [-1, 1]
  - 1번 차원: [0, 10]
  - 2번 차원: [0, 2]
- 입력 벡터
  - [2.5, -5.0, 3.5]

### 📌 차원별 처리 과정
- 0번 차원 (범위: -1 ~ 1, 입력: 2.5)
  - 입력값 2.5는 범위 [−1, 1] 밖.
  - Clamp 처리 → 가장 가까운 끝점 1.0으로 조정.
  - OutOfBounds count 증가 → 1.
- 1번 차원 (범위: 0 ~ 10, 입력: -5.0)
  - 입력값 -5.0은 범위 [0, 10] 밖.
  - Clamp 처리 → 가장 가까운 끝점 0.0으로 조정해야 정상인데, 결과가 5.0으로 나온 건 periodic wrap이 적용된 경우예요.
  - periodic flag가 true라면 -5.0을 길이 10짜리 구간에 wrap → 5.0.
  - OutOfBounds count는 periodic이면 증가하지 않음 → 0.
- 2번 차원 (범위: 0 ~ 2, 입력: 3.5)
  - 입력값 3.5는 범위 [0, 2] 밖.
  - Clamp 처리 → 가장 가까운 끝점 2.0으로 조정.
  - OutOfBounds count 증가 → 1.

### 📌 최종 결과
- Clamped 벡터: [1.0, 5.0, 2.0]
- OutOfBounds count: [1, 0, 1]
- 즉:
  - 0번 차원은 clamp로 1.0, out-of-bounds 1회.
  - 1번 차원은 periodic wrap으로 5.0, out-of-bounds 없음.
  - 2번 차원은 clamp로 2.0, out-of-bounds 1회.
 
---

