# knots_insertions_for_bezier

## 1. 함수의 목적
```rust
fn knots_insertions_for_bezier(&self, degree: usize) -> KnotVector
```

- 이 함수는:
    - 주어진 곡선의 knot vector U를 보고
    - 내부 knot 들의 현재 중복도(multiplicity) 를 조사한 뒤
    - 모든 내부 knot의 중복도가 degree와 같아지도록 만들기 위해
    - 추가로 삽입해야 할 knot 값들만 모아서 KnotVector로 돌려준다.
- 이걸 하는 이유:
- B-spline/NURBS 곡선을 Bezier 조각들의 연속으로 분할하려면,  
    모든 내부 knot의 중복도가 degree와 같아야 한다.

- 그래서 이 함수는 **Bezier 분해를 위해 필요한 knot 삽입 리스트** 를 만드는 함수다.

## 2. B-spline / NURBS에서 multiplicity와 Bezier 분해
### 2.1 Knot multiplicity 정의
- B-spline 곡선에서 주어진 knot vector $U=\{ u_0,u_1,\dots ,u_m\}$  와 degree p 에 대해,
- 어떤 값 $\xi$ 의 중복도(multiplicity) 는:
```math
\mathrm{mult}(\xi )=\# \{ i\mid u_i=\xi \}
``` 
- 즉, knot vector 안에서 같은 값이 몇 번 반복되는가를 판단.

### 2.2 Bezier segment 조건
- B-spline 곡선을 Bezier 조각들로 분해하려면:
    - 모든 내부 knot $\xi$  (즉, 첫 knot와 마지막 knot를 제외한 값)에 대해  
        그 중복도가 degree p 와 같아야 한다.
-정리하면:
    - $u_0=u_1=\dots =u_p=a$
    - $u_{m-p}=\dots =u_m=b$ (end knots)
    - 내부 각각의 $\xi \in (a,b)$ 에 대해
    - $\mathrm{mult}(\xi )=p$
- 이 조건이 만족되면, 곡선은 그 내부 knot들에서 정확히 C⁰ 분할되고,  
    각 구간이 하나의 Bezier segment가 된다.

## 3. 함수의 수학적 역할
- 이 함수는 현재 곡선의 knot vector U를 보고:
    - 각 내부 knot $\xi$ 의 현재 중복도 $\mathrm{mult}(\xi)$ 를 계산하고
- $\mathrm{mult}(\xi )<p$ 인 경우  
    추가로 삽입해야 할 knot 개수를 $p-\mathrm{mult}(\xi )$  
    로 보고, 그만큼의 $\xi$  를 결과 벡터에 추가한다.
- 즉, 결과 KnotVector { knots: x } 는 다음을 만족한다:

- 이 x를 순서대로 curve.insert_knot() 같은 함수에 넣어주면,  
    결과 곡선의 모든 내부 knot multiplicity가 degree와 같아져서 Bezier 분해가 가능해진다.

## 4. 코드 단계별 설명
- 함수 전체:
```rust
fn knots_insertions_for_bezier(&self, degree: usize) -> KnotVector {
    let u = self.kv.knots.as_slice();
    if u.len() < 2 { return KnotVector { knots: Vec::new() }; }
    let a = u[0];
    let b = *u.last().unwrap();
    let mut x = Vec::<f64>::new();
    let mut i = 0usize;
    while i < u.len() {
        let val = u[i];
        let mut j = i + 1;
        while j < u.len() && u[j] == val { j += 1; }
        let multi = j - i;
        if val > a && val < b {
            if multi < degree {
                for _ in 0..(degree - multi) { x.push(val); }
            }
        }
        i = j;
    }
    KnotVector { knots: x }
}
```

### 4.1 knot vector 슬라이스 가져오기
```rust
let u = self.kv.knots.as_slice();
if u.len() < 2 { return KnotVector { knots: Vec::new() }; }
```
- u는 원본 knot vector U.
- 매우 짧은 경우(사실상 곡선이 성립하지 않는 경우), 그냥 빈 결과 리턴.

### 4.2 시작/끝 knot 값
```rust
let a = u[0];
let b = *u.last().unwrap();
```
- a=u_0 : 첫 knot
- b=u_m : 마지막 knot
- Bezweier 분해에서는 내부 knot만 고려해야 하므로,  
    이 값들을 기준으로 val > a && val < b 조건을 건다.

### 4.3 구간별로 동일한 knot 묶기 (multiplicity 계산)
```rust
let mut x = Vec::<f64>::new();
let mut i = 0usize;
while i < u.len() {
    let val = u[i];
    let mut j = i + 1;
    while j < u.len() && u[j] == val { j += 1; }
    let multi = j - i;
    ...
    i = j;
}
```

- 이 부분은 “현재 위치 i에서 시작하는 동일한 값 블록”을 찾는 루프다.
    - val = u[i]
    - j를 증가시키며 u[j] == val인 동안 진행
    - multi = j - i 가 곧 $\mathrm{mult}(val)$
- 즉, 이 while 블록 하나가:
```math
\mathrm{multi}=\mathrm{mult}(val)
```
- 를 구하는 코드다.

### 4.4 내부 knot만 처리
```rust
if val > a && val < b {
    if multi < degree {
        for _ in 0..(degree - multi) { x.push(val); }
    }
}
```

- 조건:
    - val > a && val < b
        - 내부 knot만 대상으로 한다. (end knots는 무시)
- multi < degree인 경우,
- 부족한 만큼 degree - multi 개를 삽입 리스트에 추가.
- 즉, 수식으로는:
    - 현재 $\mathrm{mult}(val)=\mathrm{multi}$
    - 목표 중복도 = $p=\mathrm{degree}$
- 부족한 개수:
```math
\mathrm{needed}=p-\mathrm{multi}
```
- 그러므로:
```rust
for _ in 0..(degree - multi) { x.push(val); }
```
- 는 정확히 그 수식의 구현이다.

### 4.5 결과 반환
```rust
KnotVector { knots: x }
```

- x는 **Bezier 분해를 위해 추가로 삽입해야 할 knot 값들의 리스트**.
- 이걸 차례대로 insert_knot 등에 넣어주면, 모든 내부 knot의 multiplicity가  
    degree에 맞춰진다.

## 5. 작은 예제로 직관 확인
- 예를 들어:
    - degree p=3
    - knot vector:
```math
U=[0,0,0,0,\; 0.5,\; 1,1,1,1]
```
- 여기서:
    - a=0, b=1
    - 내부 knot는 $\xi =0.5$
    - multiplicity:
```math
\mathrm{mult}(0.5)=1
```
- Bezier 분해를 위해 목표는:
```math
\mathrm{mult}(0.5)=p=3
```
- 따라서 추가 삽입해야 할 개수:
```
3-1=2
```
- 이 함수는:
```
x = [0.5, 0.5]
```
- 를 돌려줄 것이고,
- 이 두 개를 순서대로 insert 하면 곡선은 [0,0,0,0, 0.5,0.5,0.5, 1,1,1,1] 구조의  
    **두 개의 3차 Bezier segment** 로 분해 가능해진다.  

## 6. 요약
- 이 함수는 Bezier 분해를 위한 knot 삽입 후보를 계산하는 함수다.
- 수학적으로는:
- 모든 내부 knot $\xi$ 의 multiplicity를 조사하고,
- multiplicity가 degree보다 작은 경우,
- $p-\mathrm{mult}(\xi )$ 개의 $\xi$ 를 결과 리스트에 추가한다.
- 이렇게 얻은 knot 리스트를 삽입하면, 모든 내부 knot의 multiplicity가 degree와 같아지고,  
    곡선은 Bezier 조각들의 연속으로 분해된다.


## 🎯 핵심 포인트
- Bezier 곡선의 knot vector는 degree*2 + 2 개가 맞다.
- 하지만
    - B‑spline/NURBS 곡선을 Bezier 조각으로 분해할 때는  
        내부 knot multiplicity를 degree로 맞추는 것이 규칙이다.
    - 즉, **degree*2+2** 는 Bezier 자체의 knot 구조,  
        **degree - multiplicity** 는 Bezier 분해를 위한 삽입 규칙이다.

### 1) Bezier 곡선의 knot vector는 왜 degree*2 + 2인가?
- Bezier 곡선은 하나의 segment만 가진 B‑spline이다.
- degree = p 일 때:
    - knot vector 길이 = p + p + 2 = 2p + 2
    - multiplicity:
        - 시작 knot: p+1
        - 끝 knot: p+1
- 예: p = 3 (3차 Bezier)
```
[0,0,0,0, 1,1,1,1]   ← 8개 = 2*3 + 2
```
- 이건 Bezier 자체의 정의다.

## 2) 그런데 우리는 “Bezier 곡선을 만들려는 게 아님”
- **B‑spline 곡선을 Bezier 조각으로 분해하려는 것** 이다
- Bezier 조각으로 분해하려면 규칙이 다르다.
- ✔ 규칙:
    - 모든 내부 knot의 multiplicity가 degree와 같아야 한다.
    - 즉:
        - 내부 knot ξ가 multiplicity = m 이면
        - degree = p 일 때
        - 부족한 만큼 (p - m) 개를 삽입해야 한다.
- 이 규칙은 Bezier segment를 만들기 위한 조건이지  
    Bezier 자체의 knot vector 길이와는 관계가 없다.

## 3) 예제
- 예:
```rust
U = [0,0,0,0, 0.5, 1,1,1,1]
degree = 3
```
- 내부 knot = 0.5
- multiplicity = 1
- Bezier 분해를 위해 필요한 multiplicity = degree = 3
- 따라서:
    - 필요한 삽입 개수 = 3 - 1 = 2
- 그래서 결과는:
```
[0.5, 0.5]
```

## 4) **degree*2+2** 는 왜 여기서 쓰이지 않는가?
- 그건 Bezier 하나의 knot vector 길이일 뿐이고,
- 우리가 지금 하는 작업은:
    - **Bezier 하나를 만드는 것** 이 아니라
    - **B‑spline을 여러 Bezier 조각으로 쪼개는 것** 이다.
- 즉:
    - Bezier 하나의 knot vector는 2p+2
    - 하지만 B‑spline 전체 knot vector는 훨씬 더 길다
    - 내부 knot multiplicity만 p로 맞추면  
        B‑spline은 여러 개의 Bezier segment로 나뉜다
- 그래서 **degree*2+2** 는 여기서 전혀 등장하지 않는다.

## 🎉 결론:
- Bezier 곡선의 knot 구조와  
    Bezier 분해를 위한 B‑spline knot 삽입 규칙을 혼동하면 안된다.
- Bezier 자체:
    - knot count = 2p + 2
    - multiplicity = p+1 at ends
- Bezier 분해:
    - 내부 knot multiplicity = p
    - 부족한 만큼 삽입
- 그래서 0.5가 세개가 아니라, 두 개 더 들어오는 게 맞다.

---
## 소스 코드
```rust
impl NurbsCurve {
    #[allow(unused)]
    pub fn knots_insertions_for_bezier(&self, degree: usize) -> KnotVector {
        let u = self.kv.knots.as_slice();
        if u.len() < 2 { return KnotVector { knots: Vec::new() }; }
        let a = u[0];
        let b = *u.last().unwrap();
        let mut x = Vec::<f64>::new();
        let mut i = 0usize;
        while i < u.len() {
            let val = u[i];
            let mut j = i + 1;
            while j < u.len() && u[j] == val { j += 1; }
            let multi = j - i;
            if val > a && val < b {
                if multi < degree {
                    for _ in 0..(degree - multi) { x.push(val); }
                }
            }
            i = j;
        }
        KnotVector { knots: x }
    }
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::prelude::{KnotVector, NurbsCurve};

    #[test]
    fn test_bezier_insertions_no_internal_knots() {
        // degree = 3, no internal knots → nothing to insert
        let curve = NurbsCurve {
            dimension: 3,
            degree: 3,
            ctrl: vec![],
            kv: KnotVector { knots: vec![0.0, 0.0, 0.0, 0.0,
                                         1.0, 1.0, 1.0, 1.0] },
            domain: Default::default(),
        };

        let ins = curve.knots_insertions_for_bezier(3);
        assert!(ins.knots.is_empty());
    }
```
```rust
    #[test]
    fn test_bezier_insertions_single_internal_knot_missing_two() {
        // Example:
        // U = [0,0,0,0, 0.5, 1,1,1,1]
        // degree = 3
        // mult(0.5) = 1 → need 2 more
        let curve = NurbsCurve {
            dimension: 3,
            degree: 3,
            ctrl: vec![],
            kv: KnotVector { knots: vec![
                0.0,0.0,0.0,0.0,
                0.5,
                1.0,1.0,1.0,1.0
            ]},
            domain: Default::default(),
        };

        let ins = curve.knots_insertions_for_bezier(3);
        assert_eq!(ins.knots, vec![0.5, 0.5]);
    }
```
```rust
    #[test]
    fn test_bezier_insertions_multiple_internal_knots() {
        // Example:
        // U = [0,0,0, 0.3,0.3, 0.7, 1,1,1]
        // degree = 2
        //
        // internal knots:
        // 0.3 → mult = 2 → OK (degree=2)
        // 0.7 → mult = 1 → need 1 more
        let curve = NurbsCurve {
            dimension: 3,
            degree: 2,
            ctrl: vec![],
            kv: KnotVector { knots: vec![
                0.0,0.0,0.0,
                0.3,0.3,
                0.7,
                1.0,1.0,1.0
            ]},
            domain: Default::default(),
        };

        let ins = curve.knots_insertions_for_bezier(2);
        assert_eq!(ins.knots, vec![0.7]);
    }
```
```rust
    #[test]
    fn test_bezier_insertions_internal_knot_already_full() {
        // Example:
        // U = [0,0,0, 0.4,0.4,0.4, 1,1,1]
        // degree = 2
        // mult(0.4) = 3 → already > degree → no insertion
        let curve = NurbsCurve {
            dimension: 3,
            degree: 2,
            ctrl: vec![],
            kv: KnotVector { knots: vec![
                0.0,0.0,0.0,
                0.4,0.4,0.4,
                1.0,1.0,1.0
            ]},
            domain: Default::default(),
        };

        let ins = curve.knots_insertions_for_bezier(2);
        assert!(ins.knots.is_empty());
    }
```
```rust
    #[test]
    fn test_bezier_insertions_multiple_missing() {
        // Example:
        // U = [0,0,0, 0.2, 0.5,0.5, 1,1,1]
        // degree = 3
        //
        // internal:
        // 0.2 → mult = 1 → need 2 more
        // 0.5 → mult = 2 → need 1 more
        let curve = NurbsCurve {
            dimension: 3,
            degree: 3,
            ctrl: vec![],
            kv: KnotVector { knots: vec![
                0.0,0.0,0.0,
                0.2,
                0.5,0.5,
                1.0,1.0,1.0
            ]},
            domain: Default::default(),
        };
        let ins = curve.knots_insertions_for_bezier(3);
        assert_eq!(ins.knots, vec![0.2, 0.2, 0.5]);
    }
}
```
---

