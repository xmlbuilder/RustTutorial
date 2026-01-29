# iter position / find / enumerator / any
- iter().position()은 Rust에서 정말 자주 쓰이는 **검색 계열** 함수 중 하나.
- 이걸 제대로 이해하면 find, rposition, find_map, enumerate 같은 함수들도 자연스럽게 연결.

## 🔷 1) iter().position() — 조건을 만족하는 첫 번째 인덱스 찾기
```rust
let arr = [10, 20, 30, 40];
let idx = arr.iter().position(|x| *x == 30);
assert_eq!(idx, Some(2));
```

### ✔ 특징
- 앞에서부터 순회
- 조건을 만족하는 첫 번째 요소의 인덱스(Some) 반환
- 없으면 None
### ✔ 반환 타입
- Option<usize>


### ✔ 언제 쓰나?
- knot vector에서 특정 knot 위치 찾기
- control point 배열에서 조건에 맞는 첫 요소 찾기
- surface row/column에서 특정 값 찾기

## 🔷 2) iter().rposition() — 뒤에서부터 검색
```rust
let arr = [1, 2, 3, 2, 1];
let idx = arr.iter().rposition(|x| *x == 2);
assert_eq!(idx, Some(3));
```

### ✔ 특징
- 뒤에서부터 검색
- 조건을 만족하는 마지막 요소의 인덱스 반환
### ✔ 언제 쓰나?
- knot vector에서 마지막 중복 knot 위치 찾기
- surface에서 뒤쪽 row/column 검색

## 🔷 3) iter().find() — 요소 자체를 반환
```rust
let arr = [10, 20, 30];
let val = arr.iter().find(|x| **x > 15);
assert_eq!(val, Some(&20));
```

### ✔ 특징
- 조건을 만족하는 첫 번째 요소의 참조 반환
- 인덱스가 아니라 값 자체가 필요할 때 사용

## 🔷 4) iter().find_map() — 변환 + 검색 동시에
```rust
let arr = ["10", "xx", "30"];
let val = arr.iter().find_map(|s| s.parse::<i32>().ok());
assert_eq!(val, Some(10));
```

### ✔ 특징
- map + find를 한 번에
- Option을 반환하는 closure를 넣으면 첫 번째 Some(...)을 반환
### ✔ 언제 쓰나?
- 문자열 → 숫자 변환하면서 첫 유효 값 찾기
- control point에서 weight가 특정 조건을 만족하는 첫 점 찾기

## 🔷 5) iter().enumerate() — 인덱스 + 값 동시에
```rust
for (i, v) in arr.iter().enumerate() {
    println!("{i}: {v}");
}
```

### ✔ 특징
- (index, &value) 형태로 순회
- position과 비슷하지만 모든 요소를 순회한다
### ✔ 언제 쓰나?
- knot vector 전체를 순회하면서 index도 필요할 때
- control point 배열에서 index와 값 둘 다 필요할 때

## 🔷 6) iter().filter() / filter_map()
- filter
```rust
let v: Vec<_> = arr.iter().filter(|x| **x > 10).collect();
```

- filter_map
```rust
let v: Vec<_> = arr.iter().filter_map(|x| x.parse().ok()).collect();
```

### ✔ 특징
- 조건을 만족하는 모든 요소를 모음
- position과 달리 여러 개를 반환

## 🔷 7) iter().any() / all() — 조건 검사
```rust
arr.iter().any(|x| *x == 30); // true
arr.iter().all(|x| *x > 0);   // true
```

### ✔ 언제 쓰나?
- knot vector가 정렬되어 있는지 검사
- control point weight가 모두 양수인지 검사
- surface row/column이 특정 조건을 만족하는지 검사

## 🔷 8) iter().take_while() / skip_while()
```rust
arr.iter().take_while(|x| **x < 30);
```

### ✔ 특징
- 조건이 깨질 때까지 앞에서부터 가져오기
- knot vector에서 특정 구간만 추출할 때 유용

## 🔥 NURBS 커널에서 특히 자주 쓰이는 패턴
- ✔ 1) knot span 찾기
```rust
let span = knots.iter().rposition(|u| *u <= x).unwrap();
```

- ✔ 2) multiplicity 계산
```rust
let s = knots[r..].iter().take_while(|u| **u == knots[r]).count();
```

- ✔ 3) control point에서 특정 weight 찾기
```rust
let idx = ctrl.iter().position(|p| p.w < 0.01);
```

- ✔ 4) surface row/column 검사
```rust
let ok = row.iter().all(|p| p.w > 0.0);
```
---

