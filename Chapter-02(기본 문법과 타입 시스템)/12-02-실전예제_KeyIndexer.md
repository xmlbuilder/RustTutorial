# KeyIndexer
아래는 KeyIndexer 구조체를 중심으로 HashMap 관점에서 정리한 문서화 내용입니다.

## 🧠 KeyIndexer: HashMap 기반 키 인덱싱 유틸리티
KeyIndexer는 i32 타입의 키를 효율적으로 관리하고 인덱싱하는 구조체입니다.  
내부적으로 HashMap<i32, usize>를 활용하여 키의 빠른 조회 및 삽입을 지원하며, 키 배열과 최대 키 값을 함께 관리합니다.

## 📦 구조 개요
```rust
use std::collections::HashMap;

pub struct KeyIndexer {
    keys: Vec<i32>,              // 키 저장 순서 유지
    map: HashMap<i32, usize>,    // 키 → 인덱스 매핑
    max_key: i32,                // 현재까지 삽입된 최대 키
}
```

## 🧩 HashMap 활용 포인트

| 메서드 또는 기능                  | 설명 |
|----------------------------------|------|
| `map.get(&key)`                 | 키에 해당하는 인덱스를 조회합니다. |
| `map.insert(key, idx)`         | 새로운 키를 삽입하고 인덱스를 등록합니다. |
| `HashMap::with_capacity(size)` | 초기 해시 테이블 크기를 설정합니다. |
| `map.clear()`                  | 해시 테이블을 초기화합니다. |
| `map.reserve(size)`           | 해시 테이블의 메모리를 사전 확보합니다. |
| `set_hash_table_size(size)`   | 기존 키를 기반으로 새로운 크기의 해시 테이블을 재구성합니다. |


## 🛠 주요 메서드 설명
### 생성 및 초기화
- new(hash_size: usize): 지정된 크기의 해시 테이블로 초기화
- default_new(): 기본 크기(50_000)로 초기화
### 키 관리
- insert_key(key: i32) -> usize: 키가 존재하지 않으면 삽입하고 인덱스 반환
- find_idx(key: i32) -> Option<usize>: 키에 해당하는 인덱스 조회
- find_idx_raw(key: i32) -> isize: 조회 실패 시 -1 반환하는 버전
### 해시 테이블 조작
- set_hash_table_size(size: usize): 기존 키를 기반으로 새로운 크기의 HashMap 재생성
- set_buffer_size(size: usize): Vec와 HashMap의 메모리 사전 확보
### 키 배열 설정
- set_key_array(keys: &[i32]): 외부 키 배열을 기반으로 내부 상태 초기화 및 해시 매핑 구성
### 기타 유틸리티
- clear(): 전체 상태 초기화
- get_size(): 키 개수 반환
- get_key_slice(): 키 배열 슬라이스 반환
- get_max_key(): 현재까지 삽입된 최대 키 반환
### 📌 기본 상수
```rust
pub const DEFAULT_ARRAY_HASHMAP_SIZE: usize = 50_000;
pub const ARRAY_INDEX_NONE: isize = -1;
```

### 💡 사용 예시
```rust
let mut indexer = KeyIndexer::default_new();
indexer.insert_key(42);
indexer.insert_key(7);

assert_eq!(indexer.find_idx(42), Some(0));
assert_eq!(indexer.find_idx_raw(99), -1);
```

## 📚 요약
KeyIndexer는 HashMap을 기반으로 키의 인덱싱과 조회를 빠르게 처리할 수 있는 구조체입니다.  
대량의 키를 다루는 상황에서 성능과 메모리 효율을 고려한 설계로, 다양한 데이터 처리 시 유용하게 활용될 수 있습니다.

---

## 소스 코드
```rust
use std::collections::HashMap;

/// KeyIndexer
#[derive(Clone, Debug)]
pub struct KeyIndexer {
    keys: Vec<i32>,
    map: HashMap<i32, usize>,
    max_key: i32,
}
```
```rust
impl KeyIndexer {
    pub const DEFAULT_ARRAY_HASHMAP_SIZE: usize = 50_000;
    pub const ARRAY_INDEX_NONE: isize = -1;

    pub fn new(hash_size: usize) -> Self {
        let cap = if hash_size == 0 {
            Self::DEFAULT_ARRAY_HASHMAP_SIZE
        } else {
            hash_size
        };
        Self {
            keys: Vec::new(),
            map: HashMap::with_capacity(cap),
            max_key: 0,
        }
    }

    pub fn default_new() -> Self {
        Self::new(Self::DEFAULT_ARRAY_HASHMAP_SIZE)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.map.clear();
        self.max_key = 0;
    }

    pub fn get_size(&self) -> usize {
        self.keys.len()
    }

    pub fn get_key_slice(&self) -> &[i32] {
        &self.keys
    }

    pub fn get_max_key(&self) -> i32 {
        self.max_key
    }

    pub fn find_idx(&self, key: i32) -> Option<usize> {
        self.map.get(&key).copied()
    }

    pub fn find_idx_raw(&self, key: i32) -> isize {
        self.find_idx(key)
            .map(|i| i as isize)
            .unwrap_or(Self::ARRAY_INDEX_NONE)
    }

    pub fn insert_key(&mut self, key: i32) -> usize {
        if let Some(&idx) = self.map.get(&key) {
            return idx;
        }
        let idx = self.keys.len();
        self.keys.push(key);
        self.map.insert(key, idx);
        if key > self.max_key {
            self.max_key = key;
        }
        idx
    }

    pub fn set_hash_table_size(&mut self, size: usize) {
        let mut new_map = HashMap::with_capacity(size);
        for (i, &k) in self.keys.iter().enumerate() {
            new_map.insert(k, i);
        }
        self.map = new_map;
    }

    pub fn set_buffer_size(&mut self, size: usize) {
        self.keys.reserve(size);
        self.map.reserve(size);
    }

    pub fn set_key_array(&mut self, keys: &[i32]) {
        self.keys.clear();
        self.map.clear();

        self.keys.reserve(keys.len());
        self.keys.extend_from_slice(keys);

        self.map.reserve(keys.len());
        for (i, &k) in self.keys.iter().enumerate() {
            self.map.insert(k, i);
        }

        self.max_key = self
            .keys
            .iter()
            .copied()
            .max_by(|a, b| a.cmp(b))
            .unwrap_or(0);
    }
}
```

### 테스트 코드
```rust
#[test]
fn key_indexer_basic() {
    let mut ki = KeyIndexer::new(1024);
    assert_eq!(ki.find_idx_raw(10), KeyIndexer::ARRAY_INDEX_NONE);

    let i0 = ki.insert_key(10);
    let i1 = ki.insert_key(20);
    let i2 = ki.insert_key(10); // 기존 인덱스 반환
    assert_eq!(i0, i2);
    assert_eq!(ki.find_idx_raw(20), i1 as isize);
    assert_eq!(ki.get_size(), 2);
    assert_eq!(ki.get_max_key(), 20);

    // set_key_array
    ki.set_key_array(&[5, 8, 13, 21]);
    assert_eq!(ki.get_size(), 4);
    assert!(ki.find_idx_raw(13) >= 0);
    assert_eq!(ki.get_max_key(), 21);

    // 해시 테이블 크기 재설정(재해싱)
    ki.set_hash_table_size(2048);
    assert!(ki.find_idx_raw(13) >= 0);

    // 버퍼 리저브
    ki.set_buffer_size(1000);
    assert_eq!(ki.get_key_slice(), &[5, 8, 13, 21]);
}
```

