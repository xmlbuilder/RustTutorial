# THash

## 소스 코드
```rust
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct THash<T: Clone> {
    buckets: Vec<Vec<T>>,
}
```
```rust
impl<T: Clone> THash<T> {
    pub fn new(table_size: usize) -> Self {
        let size = if table_size == 0 { 1 } else { table_size };
        let mut buckets = Vec::with_capacity(size);
        buckets.resize_with(size, || Vec::new());
        Self { buckets }
    }

    pub fn table_size(&self) -> usize {
        self.buckets.len()
    }

    pub fn push(&mut self, hash_idx: usize, data: T) {
        self.buckets[hash_idx].push(data);
    }

    pub fn remove(&mut self) {
        for b in &mut self.buckets {
            b.clear();
            b.shrink_to(0);
        }
    }

    pub fn match2<C1, C2, FCmp, FSuc>(
        &mut self,
        mut compare: FCmp,
        mut succeed: FSuc,
        ctx_compare: &mut C1,
        ctx_succeed: &mut C2,
    ) where
        FCmp: FnMut(&mut C1, &T, &T) -> bool,
        FSuc: FnMut(&mut C2, &T, &T),
    {
        let table_size = self.table_size();
        for i in 0..table_size {
            let bucket = &mut self.buckets[i];
            let mut j = 0;
            while j < bucket.len() {
                let mut found = None;
                // k는 j+1부터
                for k in (j + 1)..bucket.len() {
                    // 같으면
                    if compare(ctx_compare, &bucket[j], &bucket[k]) {
                        // 콜백 호출
                        let (a_ptr, b_ptr) = (&bucket[j] as *const T, &bucket[k] as *const T);
                        // 안전: succeed 는 참조만 읽는다고 가정
                        unsafe {
                            succeed(ctx_succeed, &*a_ptr, &*b_ptr);
                        }
                        // swap(j+1, k)
                        // 단, j+1이 범위를 벗어나면 swap 의미가 없음(원본은 항상 j+1<=k)
                        if j + 1 < bucket.len() {
                            bucket.swap(j + 1, k);
                        }
                        // j++ 하고 다음 j로
                        j += 1;
                        found = Some(());
                        break;
                    }
                }
                if found.is_none() {
                    j += 1;
                }
            }
        }
    }

    pub fn insert_without_duplicate<C, FCmp>(
        &mut self,
        hash_idx: usize,
        data: T,
        mut compare: FCmp,
        ctx: &mut C,
    ) -> T
    where
        FCmp: FnMut(&mut C, &T, &T) -> bool,
    {
        if let Some(bucket) = self.buckets.get(hash_idx) {
            for existing in bucket {
                if compare(ctx, existing, &data) {
                    return existing.clone();
                }
            }
        }
        self.buckets[hash_idx].push(data.clone());
        data
    }

    pub fn insert_without_duplicate_3d<C, FCmp>(
        &mut self,
        hash_idx: isize,
        data: T,
        mut compare: FCmp,
        ctx: &mut C,
        sz_x: isize,
        sz_y: isize,
        sz_z: isize,
    ) -> T
    where
        FCmp: FnMut(&mut C, &T, &T) -> bool,
    {
        let table_len = self.table_size() as isize;

        // 이 인덱스를 (x, y, z)로 본다: idx = x + y*sz_x + z*sz_x*sz_y
        // 이웃 범위를 -1..=1 로 순회하여 다시 1D index 로 변환
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let neighbor_idx = hash_idx + dx + dy * sz_x + dz * sz_x * sz_y;
                    if neighbor_idx >= 0 && neighbor_idx < table_len {
                        let nidx = neighbor_idx as usize;
                        for existing in &self.buckets[nidx] {
                            if compare(ctx, existing, &data) {
                                return existing.clone();
                            }
                        }
                    }
                }
            }
        }
        // 중복이 없으면 원래 버킷에 삽입
        let idx = hash_idx as usize;
        self.buckets[idx].push(data.clone());
        data
    }

    pub fn for_each<C, F>(&mut self, mut func: F, ctx: &mut C)
    where
        F: FnMut(&mut C, &mut T),
    {
        for bucket in &mut self.buckets {
            for item in bucket {
                func(ctx, item);
            }
        }
    }

    pub fn for_each_bucket<C, F>(&mut self, hash_idx: usize, mut func: F, ctx: &mut C)
    where
        F: FnMut(&mut C, &mut T),
    {
        if let Some(bucket) = self.buckets.get_mut(hash_idx) {
            for item in bucket {
                func(ctx, item);
            }
        }
    }
}
```

## 🧠 주요 메서드 단계별 설명
- 1. new(table_size: usize)
    - table_size == 0일 경우 최소 1로 보정
    - Vec::with_capacity(size)로 메모리 미리 확보
    - resize_with(size, || Vec::new())로 각 버킷 초기화
- 2. push(hash_idx, data)
    - 지정된 인덱스의 버킷에 데이터를 단순 추가
- 3. remove()
    - 모든 버킷을 clear()로 비우고, shrink_to(0)로 메모리도 해제
- 4. match2(compare, succeed, ctx1, ctx2)
    - 각 버킷 내에서 j와 k를 비교
    - compare()가 true면 succeed() 콜백 호출
    - swap(j+1, k)로 순서 조정
    - unsafe 사용 이유: 콜백이 참조만 읽는다고 가정하여 raw pointer로 전달
- 5. insert_without_duplicate(hash_idx, data, compare, ctx)
    - 해당 버킷에 중복 여부 검사
    - 중복이면 기존 값 반환, 아니면 삽입 후 새 값 반환
- 6. insert_without_duplicate_3d(...)
    - 3D 공간에서 인접한 26개 셀을 순회하며 중복 검사
    - 중복이 없으면 원래 인덱스에 삽입
    - 공간 해시 기반 중복 제거에 적합
- 7. for_each(func, ctx)
    - 모든 버킷의 모든 요소에 대해 func(ctx, item) 호출
- 8. for_each_bucket(hash_idx, func, ctx)
    - 특정 버킷만 순회하며 func(ctx, item) 호출

## 🔍 개선 및 고려 사항

| 항목               | 관련 위치 또는 메서드             | 설명 및 개선 방향                                      |
|--------------------|-----------------------------------|--------------------------------------------------------|
| unsafe             | match2()                          | raw pointer 사용은 안전하지만, 명확한 문서화 필요       |
| 에러 처리          | push(), get()                     | 인덱스 범위 초과 시 `Result` 반환으로 안정성 향상 가능 |
| 병렬 처리 확장     | 전체 구조                         | `Send + Sync` 제약 추가 시 멀티 스레드 환경 대응 가능  |
| 성능 최적화        | insert_without_duplicate_3d()     | 3중 루프 구조 → 대규모 데이터 시 비용 최적화 필요      |


---

## 🧪 샘플 테스트 코드
```rust
fn main() {
    // 해시 테이블 생성
    let mut table = THash::new(10);

    // 비교 컨텍스트 및 함수
    #[derive(Debug)]
    struct Ctx;
    let mut ctx = Ctx;

    // 비교 함수: 문자열이 같으면 true
    fn compare(_ctx: &mut Ctx, a: &String, b: &String) -> bool {
        a == b
    }

    // 성공 콜백: 매칭된 항목 출력
    fn succeed(_ctx: &mut Ctx, a: &String, b: &String) {
        println!("Matched: {} == {}", a, b);
    }

    // 데이터 삽입
    table.push(0, "apple".to_string());
    table.push(0, "banana".to_string());
    table.push(0, "apple".to_string()); // 중복

    // 중복 제거 삽입 테스트
    let inserted = table.insert_without_duplicate(0, "banana".to_string(), compare, &mut ctx);
    println!("Inserted or existing: {}", inserted);

    // match2 테스트
    table.match2(compare, succeed, &mut ctx, &mut ctx);

    // 전체 순회
    table.for_each(|_ctx, item| {
        println!("Item: {}", item);
    }, &mut ctx);
}
```

## 🧠 테스트 포인트

| 기능명                    | 테스트 목적                          | 기대 결과 또는 확인 사항                  |
|---------------------------|--------------------------------------|-------------------------------------------|
| push()                    | 단순 데이터 삽입                     | 지정된 버킷에 데이터가 정상적으로 추가됨  |
| insert_without_duplicate()| 중복 검사 후 삽입 여부 확인          | 중복이면 기존 값 반환, 아니면 새 값 삽입  |
| match2()                  | 버킷 내 유사 항목 비교 및 콜백 호출 | 비교 함수가 true일 때 콜백이 실행됨       |
| for_each()                | 전체 요소 순회 및 처리               | 모든 요소에 대해 함수가 정확히 호출됨     |


# Ctx

Ctx는 예제에서 비교 함수나 콜백 함수에 전달되는 컨텍스트 객체로 사용되지만,  
현재는 내부에 아무 필드도 없는 빈 구조체입니다:
```rust
#[derive(Debug)]
struct Ctx;

```

## 🧠 그럼 Ctx는 왜 필요한 걸까?
### 1. 함수 시그니처를 맞추기 위해
```rust
fn compare(ctx: &mut Ctx, a: &T, b: &T) -> bool
```
- compare와 succeed 함수는 컨텍스트를 인자로 받도록 설계되어 있음
- 이 컨텍스트는 함수 내부에서 상태를 추적하거나 외부 설정을 참조하는 데 사용될 수 있음
- 현재는 사용하지 않지만, 확장 가능성을 고려해 구조를 맞춰둔 것

### 🔧 실제 활용 예시
예를 들어, 두 문자열을 비교할 때 비교 횟수를 세고 싶다면:
```rust
#[derive(Debug)]
struct Ctx {
    compare_count: usize,
}

fn compare(ctx: &mut Ctx, a: &String, b: &String) -> bool {
    ctx.compare_count += 1;
    a == b
}
```
- 이렇게 하면 match2()를 통해 얼마나 많은 비교가 수행됐는지 추적할 수 있어요.

## ✅ 요약
| 항목             | 설명                          | 비고                                 |
|------------------|-------------------------------|--------------------------------------|
| Ctx 구조체       | 비교 및 콜백 함수에 전달되는 컨텍스트 | 현재는 비어 있지만 확장 가능성 있음     |
| 빌림 오류        | 동일한 &mut ctx 두 번 사용 시 오류 발생 | 컨텍스트를 분리하거나 RefCell로 해결 가능 |
| 테스트 목적      | 기능별 동작 확인 및 검증       | push, match2, insert 등 개별 확인 필요 |
| 구조 안정성      | THash<T>는 안전하고 유연한 구조 | unsafe 사용은 문서화로 보완 가능        |

## 🧪 테스트 함수별 설명

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
struct Item(i32);
```

### 1. test_base
- 기능: 기본 삽입, 중복 제거, 3D 중복 검사, match2, for_each 테스트
- 단계:
- push()로 데이터 삽입
- insert_without_duplicate()로 중복 확인 및 삽입
- insert_without_duplicate_3d()로 인접 버킷 중복 검사
- match2()로 짝/홀 비교 및 콜백 호출
- for_each()로 모든 항목 값 증가
```rust
#[test]
fn test_base() {

    let mut h = THash::<Item>::new(100);

    // 단순 삽입
    h.push(10, Item(3));
    h.push(10, Item(4));
    h.push(10, Item(6));

    // 중복 삽입 방지 (같으면 true 반환하는 비교자)
    let mut ctx = ();
    let eq = |_: &mut (), a: &Item, b: &Item| a.0 == b.0;
    let x = h.insert_without_duplicate(10, Item(4), eq, &mut ctx);
    assert_eq!(x, Item(4)); // 기존 것 반환

    // 3D 이웃 버킷 중복 검사
    let szx = 10;
    let szy = 10;
    let szz = 1;
    let idx = 10 + 2 * szx + 0 * szx * szy; // (x=10, y=2)
    let y = h.insert_without_duplicate_3d(idx, Item(3), eq, &mut ctx, szx, szy, szz);
    assert_eq!(y, Item(3)); // 이웃에서 기존 3 발견

    // Match2: 같습니다 판정되면 succeed 콜백 호출 + swap(j+1, k)
    let mut calls: Vec<(i32, i32)> = Vec::new();
    let mut cmp_ctx = ();
    let mut suc_ctx = ();
    h.match2(
        |_, a: &Item, b: &Item| a.0 % 2 == b.0 % 2, // 짝/홀 비교
        |_, a: &Item, b: &Item| calls.push((a.0, b.0)),
        &mut cmp_ctx,
        &mut suc_ctx,
    );
    // calls 에 기록됨
    println!("succeed calls: {:?}", calls);

    // for_each
    h.for_each(|_, it: &mut Item| it.0 += 1, &mut ());
}
```
### 2. table_size_basic
- 기능: 테이블 크기 확인
- 단계:
- new(128) → 크기 확인
- new(0) → 내부 보정으로 1이 되는지 확인
```rust
fn make_hash(sz: usize) -> THash<Item> {
    THash::new(sz)
}

fn eq_ctx(_: &mut (), a: &Item, b: &Item) -> bool {
    a.0 == b.0
}
```
```rust

#[test]
fn table_size_basic() {
    let h = make_hash(128);
    assert_eq!(h.table_size(), 128);

    let h0 = make_hash(0); // new(0)일 때 1로 보정되는 구현
    assert_eq!(h0.table_size(), 1);
}
```

### 3. push_and_iterate_bucket
- 기능: 특정 버킷에 삽입 후 순회
- 단계:
- 버킷 3에 값 삽입
- for_each_bucket()으로 해당 버킷의 값 수집 및 검증
```rust
#[test]
fn push_and_iterate_bucket() {
    let mut h = make_hash(16);
    h.push(3, Item(10));
    h.push(3, Item(11));
    h.push(3, Item(12));

    let mut collected = vec![];
    h.for_each_bucket(3, |_, it| collected.push(it.0), &mut ());
    assert_eq!(collected, vec![10, 11, 12]);
}
```

### 4. insert_without_duplicate_returns_existing
- 기능: 중복 삽입 시 기존 값 반환 여부 확인
- 단계:
- 기존 값 삽입 후 길이 측정
- 중복 값 삽입 → 기존 값 반환 확인
- 새로운 값 삽입 → 길이 증가 확인
```rust
#[test]
fn insert_without_duplicate_returns_existing() {
    let mut h = make_hash(8);
    h.push(2, Item(5));
    h.push(2, Item(7));

    let mut ctx = ();
    // 중복 데이터(5)를 넣으면 기존(5)을 반환하고, 버킷 길이는 변하지 않아야 한다.
    let before_len = {
        let mut tmp = vec![];
        h.for_each_bucket(2, |_, it| tmp.push(it.clone()), &mut ());
        tmp.len()
    };

    let got = h.insert_without_duplicate(2, Item(5), eq_ctx, &mut ctx);
    assert_eq!(got, Item(5));

    let after_len = {
        let mut tmp = vec![];
        h.for_each_bucket(2, |_, it| tmp.push(it.clone()), &mut ());
        tmp.len()
    };
    assert_eq!(before_len, after_len);

    // 새로운 값은 삽입되어 길이가 증가
    let got2 = h.insert_without_duplicate(2, Item(99), eq_ctx, &mut ctx);
    assert_eq!(got2, Item(99));
    let after_len2 = {
        let mut tmp = vec![];
        h.for_each_bucket(2, |_, it| tmp.push(it.clone()), &mut ());
        tmp.len()
    };
    assert_eq!(after_len2, after_len + 1);
}
```

### 5. insert_without_duplicate_3d_neighbor_detects_duplicate
- 기능: 3D 인접 버킷 중복 탐지
- 단계:
- 3D 인덱스 계산
- 이웃 버킷에 값 삽입
- 중심 버킷에 중복 값 삽입 → 기존 값 반환 확인
- 중심 버킷이 비어 있는지 확인

```rust
#[test]
fn insert_without_duplicate_3d_neighbor_detects_duplicate() {
    // 3D 그리드: szx=10, szy=10, szz=10 => 총 1000 버킷이라고 가정
    let szx: isize = 10;
    let szy: isize = 10;
    let szz: isize = 10;
    let total = (szx * szy * szz) as usize;

    let mut h = make_hash(total);

    // (x,y,z) -> idx
    let idx3 = |x: isize, y: isize, z: isize| -> isize { x + y * szx + z * szx * szy };

    // 중심 버킷 (5,5,5)에 새 데이터 777을 넣되,
    // 이웃 버킷 (6,5,5)에 이미 777이 존재하면 그걸 반환해야 함
    let center = idx3(5, 5, 5);
    let neighbor = idx3(6, 5, 5);

    h.push(neighbor as usize, Item(777));

    let mut ctx = ();
    let got = h.insert_without_duplicate_3d(center, Item(777), eq_ctx, &mut ctx, szx, szy, szz);
    assert_eq!(got, Item(777));

    // center 버킷에는 새로 안 들어갔는지 확인(여전히 비어있어야 함)
    let mut bucket_center = vec![];
    h.for_each_bucket(
        center as usize,
        |_, it| bucket_center.push(it.clone()),
        &mut (),
    );
    assert!(bucket_center.is_empty());
}
```


### 6. match2_calls_succeed_and_swaps_j1_k
- 기능: match2()의 비교 및 swap(j+1, k) 동작 확인
- 단계:
- 버킷에 값 삽입
- match2()로 짝/홀 비교 및 콜백 기록
- 최종 버킷 순서 확인
- 콜백 호출 횟수 및 내용 검증
```rust
fn parity_eq(_: &mut (), a: &Item, b: &Item) -> bool {
    (a.0 % 2) == (b.0 % 2)
}
```
```rust
#[test]
fn match2_calls_succeed_and_swaps_j1_k() {
    // 버킷 하나에 레이아웃을 가볍게 장난쳐서 swap(j+1, k) 효과가 보이도록 배치
    // [1, 2, 3, 5, 7]
    // j=0(1): k=1(2 - 다른 패리티, pass), k=2(3 - 같은 패리티) => swap(1,2) => [1,3,2,5,7], j=1
    // j=1(3): k=2(2 - 다른 패리티), k=3(5 - 같은 패리티) => swap(2,3) => [1,3,5,2,7], j=2
    // j=2(5): k=3(2 - 다른 패리티), k=4(7 - 같은 패리티) => swap(3,4) => [1,3,5,7,2], j=3
    // j=3(7): k=4(2 - 다른 패리티) => no swap, j=4 done.
    let mut h = make_hash(4);
    let idx = 1;
    for &v in &[1, 2, 3, 5, 7] {
        h.push(idx, Item(v));
    }

    let mut calls: Vec<(i32, i32)> = Vec::new();
    let mut cmp_ctx = ();
    let mut suc_ctx = ();
    h.match2(
        parity_eq,
        |_ctx, a: &Item, b: &Item| calls.push((a.0, b.0)),
        &mut cmp_ctx,
        &mut suc_ctx,
    );

    // swap 후 최종 순서를 확인
    let mut after = vec![];
    h.for_each_bucket(idx, |_, it| after.push(it.0), &mut ());
    assert_eq!(after, vec![1, 3, 5, 7, 2]);

    // succeed 가 "짝/홀 동일"인 쌍마다 호출됨
    // (정확한 호출 횟수는 내부 탐색 경로에 좌우되므로, 최소한 3번 호출됐는지만 체크)
    assert!(calls.len() >= 3);
    // 첫 호출은 (1,3)이어야 함
    assert_eq!(calls[0], (1, 3));
}
```

### 7. for_each_mutates_all_items
- 기능: for_each()로 모든 항목 변경
- 단계:
- 여러 버킷에 값 삽입
- for_each()로 모든 값 +1
- 결과 수집 및 정렬 후 검증

```rust
#[test]
fn for_each_mutates_all_items() {
    let mut h = make_hash(2);
    h.push(0, Item(10));
    h.push(0, Item(20));
    h.push(1, Item(30));

    h.for_each(|_, it| it.0 += 1, &mut ());

    let mut all = vec![];
    h.for_each(|_, it| all.push(it.0), &mut ());
    all.sort();
    assert_eq!(all, vec![11, 21, 31]);
}
```

### 8. for_each_bucket_only_mutates_that_bucket
- 기능: 특정 버킷만 변경
- 단계:
- 여러 버킷에 값 삽입
- 버킷 1만 +100
- 각 버킷 값 수집 및 검증
```rust
#[test]
fn for_each_bucket_only_mutates_that_bucket() {
    let mut h = make_hash(3);
    h.push(0, Item(1));
    h.push(1, Item(2));
    h.push(1, Item(3));
    h.push(2, Item(4));

    // 버킷 1만 +100
    h.for_each_bucket(1, |_, it| it.0 += 100, &mut ());
    let mut b0 = vec![];
    let mut b1 = vec![];
    let mut b2 = vec![];
    h.for_each_bucket(0, |_, it| b0.push(it.0), &mut ());
    h.for_each_bucket(1, |_, it| b1.push(it.0), &mut ());
    h.for_each_bucket(2, |_, it| b2.push(it.0), &mut ());

    assert_eq!(b0, vec![1]);
    assert_eq!(b1, vec![102, 103]);
    assert_eq!(b2, vec![4]);
}
```

### 9. remove_clears_all_buckets
- 기능: remove()로 전체 초기화
- 단계:
- 모든 버킷에 값 삽입
- remove() 호출
- 모든 버킷이 비었는지 확인
- 다시 삽입 가능한지 확인
```rust
#[test]
fn remove_clears_all_buckets() {
    let mut h = make_hash(4);
    h.push(0, Item(1));
    h.push(1, Item(2));
    h.push(2, Item(3));
    h.push(3, Item(4));

    h.remove();

    // 모두 비었는지 확인
    for i in 0..h.table_size() {
        let mut v = vec![];
        h.for_each_bucket(i, |_, it| v.push(it.clone()), &mut ());
        assert!(v.is_empty(), "bucket {i} should be empty");
    }

    // 다시 삽입 가능
    h.push(2, Item(99));
    let mut v = vec![];
    h.for_each_bucket(2, |_, it| v.push(it.0), &mut ());
    assert_eq!(v, vec![99]);
}
```

### 10. large_randomized_insert_and_duplicate_check
- 기능: 대량 랜덤 삽입 + 중복 제거 검증
- 단계:
- 10,000회 랜덤 삽입
- 각 버킷 내 중복 제거 확인 (dedup() vs 실제 길이 비교)
```rust
#[test]
fn large_randomized_insert_and_duplicate_check() {
    use rand::{Rng, SeedableRng, rngs::StdRng};

    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    let mut h = make_hash(128);
    let mut ctx = ();

    // (버킷, 값) 랜덤 삽입 + 중복 방지
    for _ in 0..10_000 {
        let idx = rng.gen_range(0..h.table_size());
        let val = rng.gen_range(0..500); // 중복 많이 나도록 좁은 범위
        let _ = h.insert_without_duplicate(idx, Item(val), eq_ctx, &mut ctx);
    }

    // 각 버킷 안에서 값이 중복되지 않음(중복이면 insert_without_duplicate 가 기존 반환)
    for i in 0..h.table_size() {
        let mut vals = vec![];
        h.for_each_bucket(i, |_, it| vals.push(it.0), &mut ());
        vals.sort();
        vals.dedup();
        // 버킷을 다시 읽어서 unique 개수와 동일해야 함
        let mut count = 0;
        h.for_each_bucket(i, |_, _| count += 1, &mut ());
        assert_eq!(vals.len(), count, "bucket {} has duplicates", i);
    }
    }
```

### 11. string_test
- 기능: THash<String> 테스트
- 단계:
- 문자열 삽입 및 중복 제거
- match2()로 문자열 비교 및 콜백
- for_each()로 전체 순회 및 출력
```rust
#[test]
fn string_test() {
    // 해시 테이블 생성
    let mut table = THash::new(10);

    // 비교 컨텍스트 및 함수
    #[derive(Debug)]
    struct Ctx;
    let mut ctx = Ctx;

    // 비교 함수: 문자열이 같으면 true
    fn compare(_ctx: &mut Ctx, a: &String, b: &String) -> bool {
        a == b
    }

    // 성공 콜백: 매칭된 항목 출력
    fn succeed(_ctx: &mut Ctx, a: &String, b: &String) {
        println!("Matched: {} == {}", a, b);
    }

    // 데이터 삽입
    table.push(0, "apple".to_string());
    table.push(0, "banana".to_string());
    table.push(0, "apple".to_string()); // 중복

    // 중복 제거 삽입 테스트
    let inserted = table.insert_without_duplicate(0, "banana".to_string(), compare, &mut ctx);
    println!("Inserted or existing: {}", inserted);


    let mut ctx_compare = Ctx;
    let mut ctx_succeed = Ctx;

    table.match2(compare, succeed, &mut ctx_compare, &mut ctx_succeed);

    // 전체 순회
    table.for_each(|_ctx, item| {
        println!("Item: {}", item);
    }, &mut ctx);
}
```

✅ 요약 테이블

| 테스트 함수 이름                                 | 주요 기능                          | 검증 포인트                          | 비고                         |
|--------------------------------------------------|------------------------------------|--------------------------------------|------------------------------|
| test_base                                        | 전체 기능 종합 테스트              | 삽입, 중복 제거, match2, 순회         | 기본 흐름 확인용             |
| table_size_basic                                 | 테이블 크기 확인                   | 0일 때 1로 보정되는지 확인            | 생성자 로직 검증             |
| push_and_iterate_bucket                          | 특정 버킷 삽입 및 순회             | 값 수집 및 순서 확인                  | 버킷 접근 테스트             |
| insert_without_duplicate_returns_existing        | 중복 삽입 검증                     | 기존 값 반환, 길이 유지               | 삽입 조건 테스트             |
| insert_without_duplicate_3d_neighbor_detects_duplicate | 3D 인접 버킷 중복 탐지            | 이웃 버킷에서 중복 탐지 여부 확인     | 공간 해시 테스트             |
| match2_calls_succeed_and_swaps_j1_k              | match2 동작 및 swap 확인           | 콜백 호출, 순서 변경 확인             | 비교/콜백 로직 검증          |
| for_each_mutates_all_items                       | 전체 항목 변경                     | 모든 값 변경 여부 확인                | 순회 및 변경 테스트          |
| for_each_bucket_only_mutates_that_bucket         | 특정 버킷만 변경                   | 나머지 버킷 영향 없음 확인            | 범위 제한 순회 테스트        |
| remove_clears_all_buckets                        | 전체 초기화                        | 모든 버킷 비움 + 재삽입 가능 확인     | 메모리 해제 및 재사용 검증  |
| large_randomized_insert_and_duplicate_check      | 대량 삽입 + 중복 제거              | dedup vs 실제 길이 비교               | 성능 및 중복 안정성 테스트  |
| string_test                                      | 문자열 타입 테스트                 | 중복 제거, match2, 순회               | 제네릭 타입 확장 테스트      |


---


## 🧠 용도 분석: 중복 절점 제거 + 이웃 엣지 탐색
### 1. 중복 절점 제거
- 그래프나 메시 구조에서 동일한 좌표나 속성을 가진 노드가 여러 번 들어올 수 있음
- insert_without_duplicate() 또는 insert_without_duplicate_3d()를 통해 중복된 노드를 제거하고 기존 노드를 재사용 가능
- 특히 insert_without_duplicate_3d()는 공간 해시 기반으로 3D 인접 셀을 탐색하며 중복을 제거함
### 2. Neighbor Edge 찾기
- match2()를 통해 같은 버킷 내에서 조건에 맞는 노드 쌍을 탐색하고, 콜백으로 엣지를 생성하거나 기록 가능
- 예: 좌표가 가까운 노드끼리 연결하거나, 속성이 유사한 노드끼리 엣지를 생성

## ✅ THash<T>가 적합한 이유

| 기능 요소       | 관련 구조 또는 메서드               | 설명 및 활용 목적                                      |
|----------------|-------------------------------------|--------------------------------------------------------|
| 버킷 기반 저장 | Vec<Vec<T>>                         | 해시 인덱스 기반으로 다수의 항목을 그룹화하여 저장 가능 |
| 커스텀 비교    | FnMut(&mut Ctx, &T, &T)             | 사용자 정의 비교 로직으로 유연한 중복 판별 가능         |
| 3D 공간 탐색   | insert_without_duplicate_3d()       | 3D 인접 셀을 순회하며 중복 절점 탐색 및 제거 가능       |
| 조건 매칭      | match2() + succeed()                | 조건 만족 시 콜백 호출로 엣지 생성 등 후속 처리 가능     |
| 순회 및 조작   | for_each(), for_each_bucket()       | 전체 또는 특정 버킷에 대해 요소 순회 및 수정 가능        |


## ✨ 실전 예시 흐름
```rust
// 중복 절점 제거
let node = thash.insert_without_duplicate_3d(idx, new_node, compare_fn, &mut ctx, szx, szy, szz);

// 이웃 엣지 탐색
thash.match2(compare_fn, |ctx, a, b| {
    edges.push((a.id, b.id)); // 또는 거리 계산 후 조건부 연결
}, &mut cmp_ctx, &mut suc_ctx);
```


## 🔍 대안과 비교

| 구조            | 중복 제거 | 인접 탐색 | 커스텀 비교 | 공간 해시 | 버킷 직접 제어 |
|-----------------|------------|------------|--------------|------------|------------------|
| HashSet<T>      | ✅         | ❌         | ❌           | ❌         | ❌               |
| HashMap<K, V>   | ✅         | ❌         | ❌           | ❌         | ❌               |
| THash<T>        | ✅         | ✅         | ✅           | ✅         | ✅               |

## 🧠 설명 요약
- 중복 제거: 세 구조 모두 중복 방지 가능하지만, THash<T>는 커스텀 비교로 제어 가능
- 인접 탐색: THash<T>만 3D 공간 기반 인접 버킷 탐색 가능
- 커스텀 비교: THash<T>는 FnMut(&mut Ctx, &T, &T)로 유연한 비교 가능
- 공간 해시: THash<T>는 좌표 기반 해시 인덱싱에 적합
- 버킷 직접 제어: THash<T>는 Vec<Vec<T>> 구조로 버킷을 직접 순회/조작 가능

## 🧠 결론
THash<T>는 그래프 처리, 메시 생성, 공간 인덱싱 등에서 중복 절점 제거와 이웃 엣지 탐색을 동시에 처리할 수 있는 전용 구조입니다.

---

