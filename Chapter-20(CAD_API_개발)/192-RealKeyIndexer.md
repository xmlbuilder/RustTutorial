


## 📘 RealKeyIndexer — 문서화 
- (Rust API Reference + Algorithm Guide)
## 개요
- RealKeyIndexer는 중복 없는 실수(real) 값 집합을 효율적으로 관리하기 위한 자료구조다.
- 다음과 같은 기능을 제공한다:
    - tolerance 기반 중복 제거
    - 정렬된 실수 키 유지
    - 빠른 검색 (BTreeMap 기반 O(log n))
    - NaN 입력 금지
    - tombstone + compact 지원
    - 대량 데이터에서도 안정적
- 이 구조는 중복이 의미 없는 실수 파라미터 집합을 다루는 데 최적화되어 있으며,  
    NURBS 엔진에서 다음과 같은 용도에 적합하다:
    - subdivision parameter set
    - intersection parameter set
    - trimming curve parameter collection
    - unique breakpoints
    - tolerance 기반 실수 집합 관리
- 반대로 **중복이 의미 있는 구조(Knot Vector 등)** 에는 적합하지 않다.

## 🧩 구조 정의
```rust
pub struct RealKeyIndexer {
    keys: Vec<f64>,                          // idx → value
    map: BTreeMap<OrderedFloat<f64>, usize>, // value → idx
    tol: f64,
}
```

## 필드 설명
| Field | Type | Description |
|-------|--------|-------------|
| keys | Vec<f64> | Stores values by index. May contain tombstones (NaN) after removals. |
| map  | BTreeMap<OrderedFloat<f64>, usize> | Maps sorted unique real values to their index in `keys`. Enables fast tolerance-based lookup. |
| tol  | f64 | Tolerance used to determine whether two real numbers are considered equal. |


## ⚙️ 알고리즘 설명
- RealKeyIndexer는 다음 두 가지 핵심 알고리즘을 기반으로 한다.

### 1) Tolerance 기반 중복 제거 알고리즘
- 실수 비교는 부동소수점 오차 때문에 직접 비교가 불가능하다.
- 따라서 RealKeyIndexer는 다음 규칙을 사용한다:
- 두 실수 a,b가 다음을 만족하면 동일한 값으로 간주한다:
```math
|a-b|<\mathrm{tol}
```
- 구현 방식
- BTreeMap에서
```math
[a-\mathrm{tol},\  a+\mathrm{tol}]
```
- 범위의 key들을 range 검색한다.
- 해당 범위에서 실제로 tolerance 안에 있는 값이 있으면  
    그 인덱스를 재사용한다.
- 없으면 새로운 값을 삽입한다.
- 이 방식은:
    - HashMap보다 빠르고 안정적
    - tolerance 기반 검색이 O(log n)
    - 정렬된 key 유지
- 라는 장점이 있다.
### 2) 정렬된 실수 집합 유지 알고리즘BTreeMap은 key를 자동으로 정렬한다.
- 따라서 RealKeyIndexer는 별도의 정렬 과정 없이  
    항상 정렬된 실수 집합을 유지한다.
- 정렬된 키는 다음과 같이 얻을 수 있다:
```rust
let sorted = key_indexer.sorted_keys();
```
## 📚 함수 설명
- new(tol: f64) -> Self지정한 tolerance로 새로운 인덱서를 생성한다.
- default_new() -> Self기본 tolerance(1e-12)로 생성한다.
- insert(value: f64) -> usize새로운 실수 값을 삽입한다.
    - tolerance 범위 내 기존 값이 있으면 해당 인덱스를 반환
    - 없으면 새로운 인덱스를 생성
- value.is_nan()이면 panic 발생 (NaN 금지 정책)
- 수식
```math
\exists x\in S\mathrm{\  such\  that\  }|x-v|<\mathrm{tol}\Rightarrow \mathrm{return\  index(x)}
```
- 그 외:
```math
S=S\cup \{ v\}
```
- find_idx(value: f64) -> Option<usize>tolerance 기반으로 value와 동일한 값을 찾는다.
- remove(value: f64) -> Option<usize>value와 tolerance 범위 내  
    동일한 값을 찾아 삭제한다.
    - 삭제된 위치는 tombstone(NaN)으로 표시
- compact() 호출 전까지 keys 배열은 sparse 상태가 될 수 있음
- compact() tombstone(NaN) 항목을 제거하고 keys와 map을 재구성한다.
- keys는 연속된 실수 배열로 재정렬
- map은 새 인덱스에 맞게 재구성
- 성능: O(n)
- sorted_keys() -> Vec<f64>정렬된 실수 리스트를 반환한다.
- min_key(), max_key()정렬된 key의 최소/최대 값을 반환한다.

### 🧪 사용 예제
```rust
let mut ki = RealKeyIndexer::new(1e-12);

let i0 = ki.insert(0.3);
let i1 = ki.insert(0.5);
let i2 = ki.insert(0.3000000000000001); // tolerance 안 → i0 재사용

assert_eq!(i0, i2);

let sorted = ki.sorted_keys();
assert_eq!(sorted, vec![0.3, 0.5]);
```
🔄 NURBS 엔진과의 대응|  |  | 
| C Function / Process | Rust (RealKeyIndexer) | Description |
|----------------------|------------------------|-------------|
| on_update_hash       | insert()               | Tolerance 기반으로 실수 파라미터를 삽입하고 중복을 제거함 |
| on_get_hash_keys             | sorted_keys()          | 해시 테이블에서 모든 실수를 꺼내는 작업을 대체. 항상 정렬된 리스트 반환 |
| sort + unique        | 내부적으로 자동 처리됨 | BTreeMap 기반이라 정렬 + 중복 제거가 자동으로 보장됨 |
| ERA sentinel         | 필요 없음              | Rust에서는 NaN 금지 + BTreeMap 사용으로 ERA 구조 제거 |
| Hash table buckets   | BTreeMap               | 버킷 기반 해시 대신 정렬 맵으로 안정적이고 빠른 검색 제공 |



### 🎯 NURBS 엔진에서의 활용
- RealKeyIndexer는 다음 상황에서 매우 유용하다:
    - 곡선/곡면 subdivision
    - intersection parameter 수집
    - trimming curve parameter 정리
    - tolerance 기반 파라미터 정리
    - 중복 없는 파라미터 집합 생성
    - 정렬된 u 값 리스트 생성
- 특히:
    - 중복이 의미 없는 파라미터 집합
    - 정렬이 필요한 파라미터 집합
    - tolerance 기반 비교가 필요한 경우
- 에 최적화되어 있다.
## 🧠 요약
- RealKeyIndexer는:
    - tolerance 기반 중복 제거
    - 정렬 유지
    - 빠른 검색
    - NaN 금지
    - 대량 데이터에서도 안정적
- 이라는 특징을 가진 NURBS 엔진용 실수 파라미터 관리의 최적 구조다.
----

## 소스 코드
```rust
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

/// 실수 기반 KeyIndexer
#[derive(Clone, Debug)]
pub struct RealKeyIndexer {
    keys: Vec<f64>,                          // idx → value
    map: BTreeMap<OrderedFloat<f64>, usize>, // value → idx
    tol: f64,
}

impl RealKeyIndexer {
    pub const ARRAY_INDEX_NONE: isize = -1;

    pub fn new(tol: f64) -> Self {
        Self {
            keys: Vec::new(),
            map: BTreeMap::new(),
            tol,
        }
    }

    pub fn default_new() -> Self {
        Self::new(1e-12)
    }

    #[inline]
    fn wrap(v: f64) -> OrderedFloat<f64> {
        OrderedFloat(v)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.map.clear();
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn keys(&self) -> &[f64] {
        &self.keys
    }

    pub fn find_idx(&self, value: f64) -> Option<usize> {
        let v = Self::wrap(value);
        let low = Self::wrap(value - self.tol);
        let high = Self::wrap(value + self.tol);

        self.map
            .range(low..=high)
            .find(|(k, _)| (k.into_inner() - v.into_inner()).abs() < self.tol)
            .map(|(_, &idx)| idx)
    }

    pub fn find_idx_raw(&self, value: f64) -> isize {
        self.find_idx(value)
            .map(|i| i as isize)
            .unwrap_or(Self::ARRAY_INDEX_NONE)
    }

    pub fn contains(&self, value: f64) -> bool {
        self.find_idx(value).is_some()
    }

    pub fn get_value(&self, idx: usize) -> Option<f64> {
        self.keys.get(idx).copied()
    }

    /// tolerance 기반 중복 제거 포함 삽입
    pub fn insert(&mut self, value: f64) -> usize {
        if value.is_nan() {
            panic!("RealKeyIndexer does not accept NaN values");
        }

        if let Some(idx) = self.find_idx(value) {
            return idx;
        }

        let idx = self.keys.len();
        self.keys.push(value);
        self.map.insert(Self::wrap(value), idx);
        idx
    }


    pub fn remove(&mut self, value: f64) -> Option<usize> {
        let idx = self.find_idx(value)?;
        let key = Self::wrap(self.keys[idx]);
        self.map.remove(&key);
        self.keys[idx] = f64::NAN; // tombstone
        Some(idx)
    }

    pub fn compact(&mut self) {
        let mut new_keys = Vec::new();
        let mut new_map = BTreeMap::new();

        for &v in &self.keys {
            if v.is_nan() {
                continue;
            }
            let idx = new_keys.len();
            new_keys.push(v);
            new_map.insert(Self::wrap(v), idx);
        }

        self.keys = new_keys;
        self.map = new_map;
    }

    pub fn sorted_keys(&self) -> Vec<f64> {
        self.map.keys().map(|k| k.into_inner()).collect()
    }

    pub fn min_key(&self) -> Option<f64> {
        self.map.keys().next().map(|k| k.into_inner())
    }

    pub fn max_key(&self) -> Option<f64> {
        self.map.keys().next_back().map(|k| k.into_inner())
    }
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use rand::Rng;
    use nurbslib::core::real_key_indexer::RealKeyIndexer;

    const TOL: f64 = 1e-12;

    fn new_indexer() -> RealKeyIndexer {
        RealKeyIndexer::new(TOL)
    }
```
```rust
    #[test]
    fn insert_basic_values() {
        let mut ki = new_indexer();
        assert_eq!(ki.insert(1.0), 0);
        assert_eq!(ki.insert(2.0), 1);
        assert_eq!(ki.insert(3.0), 2);
        assert_eq!(ki.len(), 3);
    }
```
```rust
    #[test]
    fn insert_duplicate_within_tolerance() {
        let mut ki = new_indexer();
        let idx1 = ki.insert(1.0);
        let idx2 = ki.insert(1.0 + 1e-13);
        assert_eq!(idx1, idx2);
        assert_eq!(ki.len(), 1);
    }
```
```rust
    #[test]
    fn insert_duplicate_outside_tolerance() {
        let mut ki = new_indexer();
        let idx1 = ki.insert(1.0);
        let idx2 = ki.insert(1.0 + 1e-9);
        assert_ne!(idx1, idx2);
        assert_eq!(ki.len(), 2);
    }
```
```rust
    #[test]
    fn find_idx_works() {
        let mut ki = new_indexer();
        ki.insert(0.3);
        ki.insert(0.5);
        assert_eq!(ki.find_idx(0.3), Some(0));
        assert_eq!(ki.find_idx(0.5), Some(1));
        assert_eq!(ki.find_idx(0.7), None);
    }
```
```rust
    #[test]
    fn remove_key_basic() {
        let mut ki = new_indexer();
        ki.insert(1.0);
        ki.insert(2.0);
        assert_eq!(ki.remove(1.0), Some(0));
        assert!(ki.get_value(0).unwrap().is_nan());
        assert_eq!(ki.len(), 2);
    }
```
```rust
    #[test]
    fn compact_removes_tombstones() {
        let mut ki = new_indexer();
        ki.insert(1.0);
        ki.insert(2.0);
        ki.insert(3.0);
        ki.remove(2.0);
        ki.compact();

        assert_eq!(ki.len(), 2);
        assert_eq!(ki.sorted_keys(), vec![1.0, 3.0]);
    }
```
```rust
    #[test]
    fn sorted_keys_are_correct() {
        let mut ki = new_indexer();
        ki.insert(3.0);
        ki.insert(1.0);
        ki.insert(2.0);
        assert_eq!(ki.sorted_keys(), vec![1.0, 2.0, 3.0]);
    }
```
```rust
    #[test]
    fn min_max_keys() {
        let mut ki = new_indexer();
        ki.insert(10.0);
        ki.insert(5.0);
        ki.insert(20.0);
        assert_eq!(ki.min_key(), Some(5.0));
        assert_eq!(ki.max_key(), Some(20.0));
    }
```
```rust
    #[test]
    fn insert_negative_values() {
        let mut ki = new_indexer();
        ki.insert(-3.0);
        ki.insert(-1.0);
        ki.insert(-2.0);
        assert_eq!(ki.sorted_keys(), vec![-3.0, -2.0, -1.0]);
    }
```
```rust
    #[test]
    fn insert_zero_and_neg_zero() {
        let mut ki = new_indexer();
        let idx1 = ki.insert(0.0);
        let idx2 = ki.insert(-0.0);
        assert_eq!(idx1, idx2);
    }

    #[test]
    fn insert_infinity() {
        let mut ki = new_indexer();
        ki.insert(f64::INFINITY);
        ki.insert(f64::NEG_INFINITY);
        assert_eq!(ki.sorted_keys(), vec![f64::NEG_INFINITY, f64::INFINITY]);
    }
```
```rust
    #[test]
    #[should_panic]
    fn insert_nan_is_stored_and_removable() {
        let mut ki = new_indexer();
        let idx = ki.insert(f64::NAN);
        assert!(ki.get_value(idx).unwrap().is_nan());
        assert!(ki.remove(f64::NAN).is_some());
    }
```
```rust
    #[test]
    fn large_scale_insert() {
        let mut ki = RealKeyIndexer::new(TOL);
        for i in 0..20_000 {
            ki.insert(i as f64 * 0.1);
        }
        assert_eq!(ki.len(), 20_000);
    }
```
```rust
    #[test]
    fn random_insert_and_find() {
        let mut rng = rand::thread_rng();
        let mut ki = new_indexer();

        let mut values = Vec::new();
        for _ in 0..2000 {
            let v = rng.gen_range(-1000.0..1000.0);
            values.push(v);
            ki.insert(v);
        }

        for v in values {
            assert!(ki.find_idx(v).is_some());
        }
    }
```
```rust
    #[test]
    fn tolerance_cluster_test() {
        let mut ki = new_indexer();
        ki.insert(1.0);
        ki.insert(1.0 + 1e-13);
        ki.insert(1.0 + 2e-13);
        ki.insert(1.0 + 3e-13);

        assert_eq!(ki.len(), 1);
    }
```
```rust
    #[test]
    fn tolerance_separation_test() {
        let mut ki = new_indexer();
        ki.insert(1.0);
        ki.insert(1.0 + 1e-9);
        ki.insert(1.0 + 2e-9);

        assert_eq!(ki.len(), 3);
    }
```
```rust
    #[test]
    fn large_scale_insert_100k() {
        let mut ki = RealKeyIndexer::new(TOL);

        // 100,000개의 실수 삽입
        for i in 0..100_000 {
            ki.insert(i as f64 * 0.001);
        }

        assert_eq!(ki.len(), 100_000);
    }
```
```rust
    #[test]
    fn large_scale_insert_with_tolerance_clusters() {
        let mut ki = RealKeyIndexer::new(TOL);

        // 50,000개의 값이 모두 tolerance 안에 들어가므로 1개로 압축되어야 함
        for _ in 0..50_000 {
            ki.insert(1.0 + 1e-13);
        }

        assert_eq!(ki.len(), 1);
    }
```
```rust
    #[test]
    fn large_scale_random_insert_and_find() {
        let mut rng = rand::thread_rng();
        let mut ki = RealKeyIndexer::new(TOL);

        let mut values = Vec::new();

        // 20,000개의 랜덤 실수 삽입
        for _ in 0..20_000 {
            let v = rng.gen_range(-10_000.0..10_000.0);
            values.push(v);
            ki.insert(v);
        }

        // 삽입한 값들이 모두 검색 가능해야 함
        for v in values {
            assert!(ki.find_idx(v).is_some());
        }
    }
```
```rust
    #[test]
    fn large_scale_remove_and_compact() {
        let mut ki = RealKeyIndexer::new(TOL);

        // 50,000개 삽입
        for i in 0..50_000 {
            ki.insert(i as f64);
        }

        // 절반 제거
        for i in 0..25_000 {
            ki.remove(i as f64);
        }

        // compact 수행
        ki.compact();

        assert_eq!(ki.len(), 25_000);

        // 남은 값들이 정확히 정렬되어 있어야 함
        let sorted = ki.sorted_keys();
        assert_eq!(sorted.len(), 25_000);
        assert_eq!(sorted[0], 25_000.0);
        assert_eq!(sorted[24_999], 49_999.0);
    }
```
```rust
    #[test]
    fn large_scale_sorted_keys_correctness() {
        let mut ki = RealKeyIndexer::new(TOL);

        for i in (0..100_000).rev() {
            ki.insert(i as f64);
        }

        let sorted = ki.sorted_keys();

        // 정렬된 상태인지 확인
        for i in 0..100_000 {
            assert_eq!(sorted[i], i as f64);
        }
    }
}
```
---
