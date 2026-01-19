## PeriodicDomain
- PeriodicDomain은 주기적/폐합 곡면의 파라메터 공간을 다루는 핵심적인 도우미 클래스

### 📌 구조체 필드 설명
```rust
pub struct PeriodicDomain {
    pub dom: [Interval; 2],   // u, v 방향의 파라메터 구간
    pub closed: [bool; 2],    // 각 방향이 닫혀 있는지 여부 (true면 seam 존재)
    pub norm_band: f64,        // seam 근처 band 폭 (normalized 좌표 기준)
    deck: [i32; 2],           // 현재 covering space에서 몇 번째 "층"인지
    n_prev: Point2D,           // 이전 점의 normalized 좌표 (seam crossing 판단용)
}
````
- dom: 원래 곡면의 파라메터 구간. 예: u ∈ [0,1], v ∈ [0,1].
- closed: 각 방향이 닫혀 있으면 seam이 존재. 예: 원통은 u 방향이 닫힘.
- norm_band: seam 근처에서 crossing을 감지하는 band 폭. 보통 0.2~0.3.
- deck: covering space에서 현재 몇 번째 주기인지. seam을 넘을 때 ±1씩 변함.
- n_prev: 이전 점의 normalized 좌표. seam crossing 여부를 판단하는 기준.

### 📌 주요 함수 설명
### 1. new / initialize
- 도메인, 닫힘 여부, band 폭을 설정하고 초기 상태를 만든다.
- deck은 [0,0], nprev는 unset으로 시작.
### 2. lift_to_cover(pin, stealth)
- 핵심 알고리즘: seam을 넘어가는 점을 covering space로 올려서 연속성을 유지.
- 단계:
  - 입력 점을 normalized 좌표로 변환 (dom.normalized_parameter_at).
  - 닫힌 방향이면 정수 부분을 deck_in으로 분리, 소수 부분은 0,1)로 유지.
  - 이전 점(n_prev)과 비교:
  - 이전 점이 band 아래쪽(< normband)이고 현재 점이 band 위쪽(> 1-normband)이면 seam을 거꾸로 넘어간 것 → deck 감소.
  - 반대로 이전 점이 band 위쪽이고 현재 점이 band 아래쪽이면 deck 증가.
  - 최종 좌표 = dom.parameter_at(deck + normalized).
  - stealth=false면 내부 상태(deck, nprev)를 갱신. stealth=true면 상태는 그대로 두고 결과만 반환.
- 👉 알고리즘 성격: covering space 개념을 이용한 seam crossing detection.
- 이는 위상수학의 covering map 아이디어를 그대로 구현한 것으로, seam을 넘을 때 deck을 ±1 조정해 연속성을 유지합니다.
### 3. lift_inverse(p)
- covering space 좌표를 다시 normalized domain으로 되돌림.
- 즉, dom.normalized_parameter_at(p)를 반환.
- lift_to_cover와 역관계: lift_inverse(lift_to_cover(p)) == p.

## 📌 사용된 알고리즘
- Normalized Parameter Mapping: 구간 [t0,t1]을 [0,1]로 정규화.
- Deck Tracking: seam crossing을 감지해 deck을 ±1 조정.
- Band Detection: seam 근처 band를 설정해 crossing 여부를 안정적으로 판별.
- Covering Space Lift: seam을 넘어도 좌표가 연속적으로 이어지도록 deck을 포함한 좌표를 반환.

## 📌 사용법 예시
```rust
fn main() {
    let dom_u = Interval { min: 0.0, max: 1.0 };
    let dom_v = Interval { min: 0.0, max: 1.0 };
    let mut pd = PeriodicDomain::new([dom_u, dom_v], [true, false], 0.2);

    // seam 근처 점들
    let p1 = Point2D { x: 0.95, y: 0.5 };
    let p2 = Point2D { x: 0.05, y: 0.5 };

    // 첫 점 lift
    let lifted1 = pd.lift_to_cover(p1, false);
    // seam crossing → deck 증가
    let lifted2 = pd.lift_to_cover(p2, false);

    println!("Lifted1: {:?}, Lifted2: {:?}", lifted1, lifted2);
    // 결과: lifted2.x ≈ 1.05 (deck=+1 적용)
}
```


## 📌 요약
- PeriodicDomain은 seam을 넘어가는 점을 covering space로 올려 연속성을 유지하는 도우미.
- 내부적으로 normalized 좌표 변환 + deck 추적 + band 기반 seam crossing 판별 알고리즘을 사용.
- lift_to_cover는 seam crossing을 감지해 deck을 조정, lift_inverse는 covering space → domain 역변환.
- 실제 사용 시 곡선의 점들을 순차적으로 lift_to_cover에 넣으면, seam을 넘어도 끊김 없는 파라메터 좌표를 얻을 수 있습니다.


---
## 소스 코드
```rust
use crate::core::geom::Point2D;
use crate::core::prelude::Interval;

#[derive(Debug)]
pub struct PeriodicDomain {
    pub dom: [Interval; 2],
    pub closed: [bool; 2],
    pub norm_band: f64,
    pub deck: [i32; 2],
    pub n_prev: Point2D,
}

impl PeriodicDomain {
    pub fn new(dom: [Interval; 2], closed: [bool; 2], norm_band: f64) -> Self {
        Self {
            dom,
            closed,
            norm_band,
            deck: [0, 0],
            n_prev: Point2D::UNSET,
        }
    }

    pub fn initialize(&mut self, dom: [Interval; 2], closed: [bool; 2], norm_band: f64) {
        self.dom = dom;
        self.closed = closed;
        self.norm_band = norm_band;
        self.deck = [0, 0];
        self.n_prev = Point2D::UNSET;
    }

    /// Lift point into covering space, adjusting deck if seam crossed.
    pub fn lift_to_cover(&mut self, pin: Point2D, stealth: bool) -> Point2D {
        let mut out = pin;
        let mut n_next = Point2D { x: 0.0, y: 0.0 };
        let mut deck_in = [0, 0];

        // decompose pin
        for i in 0..2 {
            let coord = if i == 0 { out.x } else { out.y };
            let mut nn = self.dom[i].normalized_parameter_at(coord);
            if self.closed[i] {
                deck_in[i] = nn.floor() as i32;
                nn -= deck_in[i] as f64;
            }
            if i == 0 { n_next.x = nn; } else { n_next.y = nn; }
        }

        if self.n_prev == Point2D::UNSET {
            if !stealth {
                self.n_prev = n_next;
                self.deck = deck_in;
            }
            return pin;
        }

        let mut deck = self.deck;
        for i in 0..2 {
            if !self.closed[i] {
                continue;
            }
            let prev = if i == 0 { self.n_prev.x } else { self.n_prev.y };
            let next = if i == 0 { n_next.x } else { n_next.y };
            if prev < self.norm_band && next > 1.0 - self.norm_band {
                deck[i] -= 1;
            } else if prev > 1.0 - self.norm_band && next < self.norm_band {
                deck[i] += 1;
            }
        }

        out.x = self.dom[0].parameter_at(deck[0] as f64 + n_next.x);
        out.y = self.dom[1].parameter_at(deck[1] as f64 + n_next.y);

        if !stealth {
            self.deck = deck;
            self.n_prev = n_next;
        }

        out
    }

    /// Projection back to domain (inverse of lift).
    pub fn lift_inverse(&self, p: Point2D) -> Point2D {
        Point2D {
            x: self.dom[0].normalized_parameter_at(p.x),
            y: self.dom[1].normalized_parameter_at(p.y),
        }
    }
}
```
### 샘플 코드
```rust
#[cfg(test)]
mod test_periodic_domain{
    use nurbslib::core::geom::Point2D;
    use nurbslib::core::periodic_domain::PeriodicDomain;
    use nurbslib::core::prelude::Interval;

    #[test]
    fn main() {
        let dom_u = Interval { t0: 0.0, t1: 1.0 };
        let dom_v = Interval { t0: 0.0, t1: 1.0 };
        let mut pd = PeriodicDomain::new([dom_u, dom_v], [true, false], 1.0 / 3.0);

        let p1 = Point2D { x: 0.95, y: 0.5 };
        let p2 = Point2D { x: 0.05, y: 0.5 };

        let lifted1 = pd.lift_to_cover(p1, false);
        let lifted2 = pd.lift_to_cover(p2, false);

        println!("Lifted1: {:?}, Lifted2: {:?}", lifted1, lifted2);
    }

    #[test]
    fn test_basic_lift_no_seam() {
        let dom_u = Interval { t0: 0.0, t1: 1.0 };
        let dom_v = Interval { t0: 0.0, t1: 1.0 };
        let mut pd = PeriodicDomain::new([dom_u, dom_v], [false, false], 1.0/3.0);

        let p = Point2D { x: 0.5, y: 0.5 };
        let lifted = pd.lift_to_cover(p, false);
        assert!((lifted.x - 0.5).abs() < 1e-12);
        assert!((lifted.y - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_seam_crossing_forward() {
        let dom_u = Interval { t0: 0.0, t1: 1.0 };
        let dom_v = Interval { t0: 0.0, t1: 1.0 };
        let mut pd = PeriodicDomain::new([dom_u, dom_v], [true, false], 0.2);

        let p1 = Point2D { x: 0.95, y: 0.5 };
        let p2 = Point2D { x: 0.05, y: 0.5 };

        let lifted1 = pd.lift_to_cover(p1, false);
        let lifted2 = pd.lift_to_cover(p2, false);

        // seam을 넘어 deck이 증가해야 함
        assert!(lifted2.x > 1.0);
    }

    #[test]
    fn test_seam_crossing_backward() {
        let dom_u = Interval { t0: 0.0, t1: 1.0 };
        let dom_v = Interval { t0: 0.0, t1: 1.0 };
        let mut pd = PeriodicDomain::new([dom_u, dom_v], [true, false], 0.2);

        let p1 = Point2D { x: 0.05, y: 0.5 };
        let p2 = Point2D { x: 0.95, y: 0.5 };

        let lifted1 = pd.lift_to_cover(p1, false);
        let lifted2 = pd.lift_to_cover(p2, false);

        // seam을 거꾸로 넘어 deck이 감소해야 함
        assert!(lifted2.x < 0.0);
    }

    #[test]
    fn test_stealth_mode() {
        let dom_u = Interval { t0: 0.0, t1: 1.0 };
        let dom_v = Interval { t0: 0.0, t1: 1.0 };
        let mut pd = PeriodicDomain::new([dom_u, dom_v], [true, false], 0.2);

        let p1 = Point2D { x: 0.95, y: 0.5 };
        let p2 = Point2D { x: 0.05, y: 0.5 };

        let _ = pd.lift_to_cover(p1, false);

        // 상태 스냅샷
        let deck_before = pd.deck;
        let nprev_before = pd.n_prev;

        // stealth 모드 호출
        let _ = pd.lift_to_cover(p2, true);

        // 상태가 바뀌지 않았는지 확인
        assert_eq!(pd.deck, deck_before);
        assert_eq!(pd.n_prev, nprev_before);

        // 이제 실제로 상태를 바꾸는 호출
        let _ = pd.lift_to_cover(p2, false);

        // 상태가 바뀌었는지 확인
        assert_ne!(pd.deck, deck_before);
    }

    #[test]
    fn test_lift_inverse_round_trip() {
        let dom_u = Interval { t0: 0.0, t1: 10.0 };
        let dom_v = Interval { t0: 0.0, t1: 5.0 };
        let pd = PeriodicDomain::new([dom_u, dom_v], [false, false], 0.3);

        let p = Point2D { x: 7.0, y: 2.5 };
        let inv = pd.lift_inverse(p);

        // normalized 좌표가 [0,1] 범위에 있어야 함
        assert!(inv.x >= 0.0 && inv.x <= 1.0);
        assert!(inv.y >= 0.0 && inv.y <= 1.0);
    }
}
```
---

