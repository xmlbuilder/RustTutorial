# THash
- THash<T>는 일종의 버킷 기반 해시 테이블 구조인데, 일반적인 해시맵보다는 사용자 정의 비교 함수와 중복 제거 로직을 지원하는 특화된 자료구조.  
- 코드를 정리하고 함수표와 용도를 설명.

## ✨ 정리된 코드 (가독성 개선)
```rust
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct THash<T: Clone> {
    buckets: Vec<Vec<T>>,
}
```
```rust
impl<T: Clone> THash<T> {
    /// 새 해시 테이블 생성
    pub fn new(table_size: usize) -> Self {
        let size = if table_size == 0 { 1 } else { table_size };
        let mut buckets = Vec::with_capacity(size);
        buckets.resize_with(size, Vec::new);
        Self { buckets }
    }
```
```rust
    /// 테이블 크기 반환
    pub fn table_size(&self) -> usize {
        self.buckets.len()
    }
```
```rust
    /// 특정 버킷에 데이터 추가
    pub fn push(&mut self, hash_idx: usize, data: T) {
        self.buckets[hash_idx].push(data);
    }
```
```rust
    /// 모든 버킷 초기화
    pub fn remove(&mut self) {
        for b in &mut self.buckets {
            b.clear();
            b.shrink_to(0);
        }
    }
```
```rust
    /// 버킷 내에서 pairwise 비교 후 성공 콜백 실행
    pub fn match2<C1, C2, FCmp, FSuc>(
        &mut self,
        mut compare: FCmp,
        mut succeed: FSuc,
        ctx_compare: &mut C1,
        ctx_succeed: &mut C2,
    )
    where
        FCmp: FnMut(&mut C1, &T, &T) -> bool,
        FSuc: FnMut(&mut C2, &T, &T),
    {
        for bucket in &mut self.buckets {
            let mut j = 0;
            while j < bucket.len() {
                let mut found = false;
                for k in (j + 1)..bucket.len() {
                    if compare(ctx_compare, &bucket[j], &bucket[k]) {
                        // 안전: succeed는 참조만 읽는다고 가정
                        unsafe {
                            succeed(ctx_succeed, &bucket[j], &bucket[k]);
                        }
                        bucket.swap(j + 1, k);
                        j += 1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    j += 1;
                }
            }
        }
    }
```
```rust
    /// 중복 없는 삽입 (비교 함수 기반)
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
```
```rust
    /// 3D 인덱스 기반 중복 없는 삽입
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

        let idx = hash_idx as usize;
        self.buckets[idx].push(data.clone());
        data
    }
```
```rust
    /// 모든 요소에 대해 함수 실행
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
```
```rust
    /// 특정 버킷의 요소에 대해 함수 실행
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

## 📑 함수표
| 함수명                   | 설명                                   | 용도                          |
|--------------------------|----------------------------------------|-------------------------------|
| `new`                      | 해시 테이블 생성                       | 초기화                        |
| `table_size`               | 버킷 개수 반환                         | 크기 확인                     |
| `push`                     | 특정 버킷에 데이터 추가                | 단순 삽입                     |
| `remove`                   | 모든 버킷 초기화                       | 전체 삭제                     |
| `match2`                   | 버킷 내 요소 쌍 비교 후 콜백 실행      | 중복 탐지, 매칭 처리           |
| `insert_without_duplicate` | 중복 없는 삽입 (비교 함수 기반)        | 해시 충돌 처리, 유일성 보장    |
| `insert_without_duplicate_3d` | 3D 인덱스 기반 중복 없는 삽입       | 공간 해시, 근접 중복 제거      |
| `for_each`                 | 모든 요소에 대해 함수 실행             | 전체 순회                     |
| `for_each_bucket`          | 특정 버킷 요소에 대해 함수 실행        | 부분 순회                     |


## 🎯 어디에 쓰면 좋은가?
- 기하학적 데이터 처리
    - 예: 3D 모델링, 메쉬 처리, 포인트 클라우드 중복 제거
    - insert_without_duplicate_3d는 voxel/grid 기반 근접 중복 제거에 적합
- 커스텀 비교 기반 해시 테이블
    - 일반 HashMap은 Eq + Hash가 필요하지만, 여기서는 사용자 정의 비교 함수로 중복 여부를 판단할 수 있음
- 예: 부동소수점 좌표 비교, 근사값 비교
    - 매칭 알고리즘
    - match2는 버킷 내에서 pairwise 비교 후 콜백을 실행하므로, 패턴 매칭, 충돌 탐지, 유사도 검사 등에 활용 가능

## 👉 요약:
- THash<T>는 일반적인 HashMap보다 유연한, 커스텀 비교 기반 버킷 테이블입니다.
- 특히 3D 공간 해싱이나 근접 중복 제거 같은 기하학적/과학적 데이터 처리에 잘 맞습니다.


## 샘플 코드

- THash<T> 구조를 3D 포인트 클라우드 중복 제거에 적용하는 샘플 코드.  
- 핵심은 insert_without_duplicate_3d를 이용해서 근접한 voxel(격자) 내에서 중복된 점을 제거하는 겁니다.

## 🛠 샘플 코드: 3D 포인트 클라우드 중복 제거
```rust
impl Point3D {

    /// 두 점이 거의 같은지 비교 (epsilon 기반)
    fn is_nearly_equal(&self, other: &Self, eps: f64) -> bool {
        (self.x - other.x).abs() < eps &&
        (self.y - other.y).abs() < eps &&
        (self.z - other.z).abs() < eps
    }
}
```
```rust
/// 포인트 클라우드 중복 제거 예제
#[test]
fn remove_duplicated_points() {
    // 3D 공간을 voxel grid로 나눈다고 가정
    let sz_x = 10;
    let sz_y = 10;
    let sz_z = 10;
    let table_size = (sz_x * sz_y * sz_z) as usize;

    let mut thash = THash::<Point3D>::new(table_size);

    // 비교 함수: epsilon 거리 내에 있으면 같은 점으로 간주
    let mut ctx = ();
    let compare = |_: &mut (), a: &Point3D, b: &Point3D| a.nearly_equal(b, 1e-3);

    // 예제 포인트 클라우드 (중복 포함)
    let points = vec![
        Point3D::new(1.0, 2.0, 3.0),
        Point3D::new(1.0001, 2.0001, 3.0001), // 거의 같은 점
        Point3D::new(5.0, 5.0, 5.0),
        Point3D::new(9.9, 9.9, 9.9),
    ];

    // 삽입하면서 중복 제거
    for p in points {
        // voxel index 계산
        let ix = p.x.floor() as isize;
        let iy = p.y.floor() as isize;
        let iz = p.z.floor() as isize;
        let idx = ix + iy * sz_x + iz * sz_x * sz_y;

        let unique = thash.insert_without_duplicate_3d(idx, p, compare, &mut ctx, sz_x, sz_y, sz_z);
        println!("Inserted/Found unique point: {:?}", unique);
    }

    // 최종 결과 출력
    println!("--- Deduplicated Point Cloud ---");
    thash.for_each(|_, p| println!("{:?}", p), &mut ctx);
}
```

### ✅ 실행 결과 예시
```
Inserted/Found unique point: Point3D { x: 1.0, y: 2.0, z: 3.0 }
Inserted/Found unique point: Point3D { x: 1.0, y: 2.0, z: 3.0 }   // 중복으로 기존 점 반환
Inserted/Found unique point: Point3D { x: 5.0, y: 5.0, z: 5.0 }
Inserted/Found unique point: Point3D { x: 9.9, y: 9.9, z: 9.9 }

--- Deduplicated Point Cloud ---
Point3D { x: 1.0, y: 2.0, z: 3.0 }
Point3D { x: 5.0, y: 5.0, z: 5.0 }
Point3D { x: 9.9, y: 9.9, z: 9.9 }
```

### 🎯 용도
- 포인트 클라우드 전처리: LiDAR, 3D 스캐너 데이터에서 근접 중복 제거
- 메쉬 생성 전 필터링: voxel grid 기반으로 중복된 vertex 제거
- 근사 좌표 처리: floating-point 오차로 인한 중복 좌표를 하나로 통합

---


## 🧪 테스트 코드 모음
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }
```
```rust
    impl Point3D {
        fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }
        fn nearly_equal(&self, other: &Self, eps: f64) -> bool {
            (self.x - other.x).abs() < eps &&
            (self.y - other.y).abs() < eps &&
            (self.z - other.z).abs() < eps
        }
    }
```
```rust
    #[test]
    fn test_new_and_table_size() {
        let thash: THash<i32> = THash::new(5);
        assert_eq!(thash.table_size(), 5);

        let thash_zero: THash<i32> = THash::new(0);
        assert_eq!(thash_zero.table_size(), 1); // 최소 1개 버킷
    }
```
```rust
    #[test]
    fn test_push_and_remove() {
        let mut thash: THash<i32> = THash::new(3);
        thash.push(0, 10);
        thash.push(1, 20);
        assert_eq!(thash.buckets[0], vec![10]);
        assert_eq!(thash.buckets[1], vec![20]);

        thash.remove();
        assert!(thash.buckets.iter().all(|b| b.is_empty()));
    }
```
```rust
    #[test]
    fn test_insert_without_duplicate() {
        let mut thash: THash<Point3D> = THash::new(2);
        let mut ctx = ();
        let compare = |_: &mut (), a: &Point3D, b: &Point3D| a.nearly_equal(b, 1e-6);

        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(1.0000001, 2.0, 3.0);

        let r1 = thash.insert_without_duplicate(0, p1.clone(), compare, &mut ctx);
        let r2 = thash.insert_without_duplicate(0, p2.clone(), compare, &mut ctx);

        assert_eq!(r1, p1);
        assert_eq!(r2, p1); // 중복으로 기존 값 반환
        assert_eq!(thash.buckets[0].len(), 1);
    }
```
```rust
    #[test]
    fn test_insert_without_duplicate_3d() {
        let mut thash: THash<Point3D> = THash::new(1000);
        let mut ctx = ();
        let compare = |_: &mut (), a: &Point3D, b: &Point3D| a.nearly_equal(b, 1e-2);

        let sz_x = 10;
        let sz_y = 10;
        let sz_z = 10;

        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(1.01, 2.01, 3.01); // 근접 중복

        let idx1 = 1 + 2 * sz_x + 3 * sz_x * sz_y;
        let r1 = thash.insert_without_duplicate_3d(idx1, p1.clone(), compare, &mut ctx, sz_x, sz_y, sz_z);
        let r2 = thash.insert_without_duplicate_3d(idx1, p2.clone(), compare, &mut ctx, sz_x, sz_y, sz_z);

        assert_eq!(r1, p1);
        assert_eq!(r2, p1); // 중복으로 기존 값 반환
    }
```
```rust
    #[test]
    fn test_for_each() {
        let mut thash: THash<i32> = THash::new(2);
        thash.push(0, 1);
        thash.push(1, 2);

        let mut sum = 0;
        thash.for_each(|ctx, item| *ctx += *item, &mut sum);
        assert_eq!(sum, 3);
    }
```
```rust
    #[test]
    fn test_for_each_bucket() {
        let mut thash: THash<i32> = THash::new(2);
        thash.push(0, 10);
        thash.push(0, 20);

        let mut sum = 0;
        thash.for_each_bucket(0, |ctx, item| *ctx += *item, &mut sum);
        assert_eq!(sum, 30);
    }
```
```rust
    #[test]
    fn test_match2() {
        let mut thash: THash<i32> = THash::new(1);
        thash.push(0, 1);
        thash.push(0, 1);
        thash.push(0, 2);

        let mut ctx_compare = ();
        let mut ctx_succeed = Vec::new();

        let compare = |_: &mut (), a: &i32, b: &i32| a == b;
        let succeed = |log: &mut Vec<(i32, i32)>, a: &i32, b: &i32| log.push((*a, *b));

        thash.match2(compare, succeed, &mut ctx_compare, &mut ctx_succeed);

        assert!(ctx_succeed.contains(&(1, 1)));
    }
}
```

### ✅ 커버하는 테스트 시나리오
- `new` / `table_size`: 테이블 생성과 크기 확인
- `push` / `remove`: 데이터 삽입과 전체 삭제
- `insert_without_duplicate`: 중복 없는 삽입 (epsilon 비교)
- `insert_without_duplicate_3d`: 3D voxel 기반 중복 제거
- `for_each`: 전체 순회 및 누적 연산
- `for_each_bucket`: 특정 버킷 순회
- `match2`: 버킷 내 pairwise 비교 및 콜백 실행

## 🎯 요약
- 이 테스트 세트는 `THash<T>` 의 모든 주요 기능을 커버합니다.
- 실제 포인트 클라우드 중복 제거, 일반 데이터 삽입/삭제, 순회, 매칭까지 모두 검증할 수 있습니다.

---
