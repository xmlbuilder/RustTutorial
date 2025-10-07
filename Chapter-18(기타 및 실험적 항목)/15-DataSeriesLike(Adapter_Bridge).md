## 어댑터(Adaptor)”/“브리지(Bridge)
우리가 만든 DataSeriesLike/ChannelLike는 전형적인 “어댑터(Adaptor)”/“브리지(Bridge)” 스타일을 Rust의 트레이트로 풀어낸 겁니다.  
핵심은 알고리즘(함수) ↔ 데이터구조(타입) 를 느슨하게 결합해서 재사용성과 성능을 동시에 얻는 것.

### 왜 이 기법이 좋은가 (일반적 관점)

- 결합도↓, 재사용성↑: 알고리즘은 “시간축/값을 읽을 수 있다”는 계약(인터페이스)만 의존합니다. 
- 실제 타입이 Channel, (xs, ys) 튜플, DataSeries 등 무엇이든 상관없음.
- 테스트 용이성: 무거운 실전 타입 대신 가벼운 어댑터(예: 슬라이스 튜플)로 알고리즘을 쉽게 단위 테스트.
- 점진적 이식/교체: 기존 C++ 구조를 그대로 옮겨오면서도 표면을 트레이트로 맞추면, 내부 구현은 나중에 바꿔도 외부 알고리즘은 그대로.
- 여러 표현을 같은 API로: 원시 슬라이스, 소유 Vec, 참조 타입, 캐시 래퍼 등 다양한 “표현”을 같은 함수에 넣을 수 있음.

### 왜 Rust에서 특히 더 좋나
- 제로코스트 추상화: 제네릭+트레이트(정적 디스패치)는 컴파일 타임에 모노모픽으로 풀려서 가상 호출 비용 없이 인라인 최적화됩니다.
- 안전한 인터페이스: 소유권/수명 검사로 잘못된 참조, UAF를 컴파일러가 잡아줌.
- 확장 가능한 다형성: 상속이 없는 Rust에서 “행동 공유”는 트레이트가 정석. 기본 메서드 제공으로 공통 유틸도 쉽게 얹음.
- 어댑터를 쉽게 합성: 참조 타입(&T), 튜플((&[f64], &[f64])), 새 타입(newtype) 등 여러 레이어의 어댑터를 붙여도 비용이 거의 없음.
- 동적 디스패치 옵션: 필요하면 Box<dyn ChannelLike>로 이질적 컬렉션도 담을 수 있음(성능/유연성 트레이드오프를 선택적으로).

### 우리 코드에 바로 대응되는 이점

#### 알고리즘-타입 분리
- get_channel_total_sum* 류 함수는 Channel에 고정되지 않고 ChannelLike만 요구  
    → 나중에 파일 스트리밍 채널, 메모리맵 채널, 가공 파이프 채널도 그대로 사용 가능.

- 테스트 더 가볍게  
    실제 Channel을 만들지 않고도 (&xs[..], &ys[..]) 같은 슬라이스 튜플에 DataSeriesLike만 구현해서 단위 테스트를 돌릴 수 있음.

- 성능 그대로
    제네릭 바운드(C: ChannelLike)는 대부분 정적 디스패치 → C++ 인라인에 대응되는 최적화가 자연스럽게 일어남.

- 참조/뷰를 그대로 사용  
    impl<T: ChannelLike + ?Sized> ChannelLike for &T 같은 참조에 대한 블랭킷 구현으로, &Channel도 바로 넣을 수 있어 불필요한 복제를 피함.

#### 간단 예시 (요약)
```rust
pub trait DataSeriesLike {
    fn xs(&self) -> &[f64];
    fn ys(&self) -> &[f64];
}

pub trait ChannelLike: DataSeriesLike {
    fn channel_name(&self) -> &str;
}

// 가벼운 어댑터: 슬라이스 튜플도 시리즈처럼 취급
impl<'a> DataSeriesLike for (&'a [f64], &'a [f64]) {
    fn xs(&self) -> &[f64] { self.0 }
    fn ys(&self) -> &[f64] { self.1 }
}

// 알고리즘은 트레이트에만 의존
pub fn area_under_curve<C: DataSeriesLike>(c: &C) -> f64 {
    let (xs, ys) = (c.xs(), c.ys());
    xs.windows(2).zip(ys.windows(2))
        .map(|(xt, yt)| (xt[1]-xt[0]) * 0.5 * (yt[0]+yt[1]))
        .sum()
}
```

### 언제 어떤 방식?
- 최고 성능·제네릭: fn foo<T: ChannelLike>(t: &T) (정적 디스패치, 모노모픽)
- 런타임 다형성: fn foo(t: &dyn ChannelLike) (동적 디스패치, 이질적 컬렉션/플러그인)
- 타입소유 바꾸지 못할 때: 새 타입(Newtype) 또는 래퍼로 트레이트를 “부여”해서 어댑터화
- 외부타입에 직접 구현 못할 때(고아 규칙): 같은 크레이트 안의 새 타입으로 감싸서 구현
- 요약하면, Rust의 트레이트 기반 어댑터는 C++의 인터페이스/어댑터 패턴을 더 안전하고, 더 빠르고, 더 쓰기 쉽게 만든 형태예요.  
    우리 프로젝트처럼 다양한 채널 표현을 같은 처리 파이프라인에 태우고, 테스트를 가볍게 돌리고, 나중에 타입을 바꿀 자유를 확보하려면  
    가장 적합한 접근입니다.

---
## DataSeries 가 없어도 됌
특정 struct DataSeries가 없어도, 그 타입이 DataSeriesLike 트레이트만 구현하면 해당 함수를 그대로 쓸 수 있음.  
Rust에선 “타입이 무엇이냐”보다 “그 타입이 무슨 동작(메서드) 을 제공하느냐(트레이트)로 다형성을 얻습니다.

### 핵심 포인트만 정리하면:
- 함수는 DataSeriesLike에만 의존 → 그 트레이트를 impl 한 어떤 타입이든 인자로 넣을 수 있음.
- 실제 데이터 구조가 Channel, 튜플, 래퍼 등 무엇이든 상관없음.
- 제네릭 + 트레이트는 정적 디스패치(제로코스트) 라서 성능 손해도 없음.

### 예시들:
```rust
pub trait DataSeriesLike {
    fn xs(&self) -> &[f64];
    fn ys(&self) -> &[f64];
}

// 1) 가벼운 어댑터: (xs, ys) 슬라이스 튜플
impl<'a> DataSeriesLike for (&'a [f64], &'a [f64]) {
    fn xs(&self) -> &[f64] { self.0 }
    fn ys(&self) -> &[f64] { self.1 }
}

// 2) 실제 타입(예: Channel)에도 같은 트레이트만 구현
struct Channel { xs: Vec<f64>, ys: Vec<f64>, name: String }
impl DataSeriesLike for Channel {
    fn xs(&self) -> &[f64] { &self.xs }
    fn ys(&self) -> &[f64] { &self.ys }
}

// 3) 알고리즘은 트레이트에만 의존
pub fn peak_y(series: &impl DataSeriesLike) -> f64 {
    series.ys().iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

// 사용 예
let xs = [0.0, 1.0, 2.0];
let ys = [1.0, 3.0, 2.0];
let p1 = peak_y(&(xs.as_slice(), ys.as_slice())); // 튜플로 사용

let ch = Channel { xs: xs.to_vec(), ys: ys.to_vec(), name: "A".into() };
let p2 = peak_y(&ch); // 실제 채널 타입으로 사용
```

### 보너스 팁:
- 외부 타입(고아 규칙 때문에 직접 impl 못 하는 경우)은 새 래퍼 타입(newtype) 으로 감싸서 impl DataSeriesLike for MyWrapper 하면 됩니다.
- 여러 타입을 한 컨테이너에 담아 다루려면 Vec<Box<dyn DataSeriesLike>> 같은 동적 디스패치도 선택 가능(유연성↑, 약간의 비용).
- 함수 시그니처를 fn f(s: &impl DataSeriesLike) 또는 fn f<T: DataSeriesLike + ?Sized>(s: &T)로 두면,  
    참조만 넘겨도 충분해서 추가 레퍼런스용 impl가 필요 없습니다.

---

### 실제 코드
```rust
trait DataSeriesLikeX {
    fn xs(&self) -> &[f64];
    fn ys(&self) -> &[f64];
}

trait ChannelLikeX {
    fn name(&self) -> &str;
    fn xs(&self) -> &[f64];
    fn ys(&self) -> &[f64];
}

// ---------- 어댑터 구현 (여러 소스에 붙이기) ----------
// 1) 튜플 (&[f64], &[f64])
impl DataSeriesLikeX for (&[f64], &[f64]) {
    fn xs(&self) -> &[f64] { self.0 }
    fn ys(&self) -> &[f64] { self.1 }
}

// 2) 간단한 테스트용 구조체
#[derive(Default)]
struct TestSeries { xs: Vec<f64>, ys: Vec<f64> }
impl DataSeriesLikeX for TestSeries {
    fn xs(&self) -> &[f64] { &self.xs }
    fn ys(&self) -> &[f64] { &self.ys }
}

// Channel-like 쪽도 동일 컨셉
#[derive(Default)]
struct TestChannelX {
    name: String,
    xs: Vec<f64>,
    ys: Vec<f64>,
}

impl ChannelLikeX for TestChannelX {
    fn name(&self) -> &str { &self.name }
    fn xs(&self) -> &[f64] { &self.xs }
    fn ys(&self) -> &[f64] { &self.ys }
}

// ---------- like-전용 유틸 함수 ----------
fn peak_y<S: DataSeriesLikeX>(s: &S) -> Option<(f64, f64)> {
    let xs = s.xs();
    let ys = s.ys();
    if xs.is_empty() || ys.is_empty() || xs.len() != ys.len() {
        return None;
    }
    let (mut best_i, mut best_y) = (0usize, f64::NEG_INFINITY);
    for (i, &y) in ys.iter().enumerate() {
        if y > best_y { best_y = y; best_i = i; }
    }
    Some((xs[best_i], best_y))
}

fn sum_channels_x<C: ChannelLikeX>(chs: &[C]) -> Option<(Vec<f64>, Vec<f64>)> {
    if chs.is_empty() { return None; }
    let xs = chs[0].xs().to_vec();
    let mut ys = vec![0.0; xs.len()];
    for ch in chs {
        for (i, &v) in ch.ys().iter().enumerate() {
            ys[i] += v;
        }
    }
    Some((xs, ys))
}

// ---------- 테스트 ----------
#[test]
fn data_series_like_x_works_with_tuple_and_struct() {
    // tuple adapter
    let xs = [0.0, 1.0, 2.0, 3.0];
    let ys = [1.0, 5.0, 2.0, 4.0];
    let tup = (&xs[..], &ys[..]);
    let peak = peak_y(&tup).unwrap();
    assert_eq!(peak, (1.0, 5.0));

    // struct adapter
    let s = TestSeries { xs: xs.to_vec(), ys: ys.to_vec() };
    let peak2 = peak_y(&s).unwrap();
    assert_eq!(peak2, peak);
}

#[test]
fn channel_like_x_sum_works() {
    // 2개 채널 합산
    let xs = [0.0, 1.0, 2.0];
    let ys1 = [1.0, 2.0, 3.0];
    let ys2 = [0.5, 0.5, 0.5];

    let c1 = TestChannelX { name: "A".into(), xs: xs.to_vec(), ys: ys1.to_vec() };
    let c2 = TestChannelX { name: "B".into(), xs: xs.to_vec(), ys: ys2.to_vec() };
    let (sum_x, sum_y) = sum_channels_x(&[c1, c2]).unwrap();
    assert_eq!(sum_x, xs.to_vec());
    assert_eq!(sum_y, vec![1.5, 2.5, 3.5]);
}
```


