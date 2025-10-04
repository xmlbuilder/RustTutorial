# IntoIterator / FromIterator

## 샘플 코드
```rust
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
}
```

이 코드는 IntoIterator와 Iterator 트레이트를 직접 구현해서 2차원 좌표 그리드(Grid)를 순회 가능한 구조로 만든 예제.  
그리고 이걸 통해 IntoIterator와 FromIterator의 원리와 역할 설명.

## 🧩 먼저 핵심 개념부터 정리

### ✅ IntoIterator
- for x in something 문법을 가능하게 해주는 트레이트
- into_iter() 메서드를 통해 소유권을 넘기면서 순회 가능한 구조를 반환
- Vec<T>, HashMap<K, V>, Option<T> 등 대부분의 컬렉션이 구현함

### ✅ Iterator
- next() 메서드를 통해 하나씩 값을 반환하는 반복자
- IntoIterator는 Iterator를 반환하는 트레이트고, Iterator는 실제 순회 로직을 담당

### ✅ FromIterator
- collect()를 가능하게 해주는 트레이트
- 여러 값을 모아서 하나의 컬렉션으로 만드는 역할

## 🔍 코드 단계별 설명
### 1. Grid 구조체
```rust
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}
```

- x_coords와 y_coords를 조합해서 (x, y) 좌표를 생성할 수 있는 구조

### 2. IntoIterator 구현
```rust
impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;

    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}
```

- Grid를 순회 가능한 구조로 만들기 위해 IntoIterator를 구현
- into_iter()는 GridIter라는 반복자를 반환

### 3. GridIter 반복자 구조
```rust
struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}
```

- i는 x축 인덱스, j는 y축 인덱스
- → x_coords[i], y_coords[j]를 조합해서 (x, y)를 생성

### 4. Iterator 구현
```rust
impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}
```

- next()는 (x, y)를 하나씩 반환
- x축을 순회하다가 끝나면 y축을 하나 증가시키고 x축을 다시 시작
- → 전체 (x, y) 조합을 순회

### 5. main()에서 사용
```rust
for (x, y) in grid {
    println!("point = {x}, {y}");
}
```

- grid는 IntoIterator를 구현했기 때문에 for 루프에서 사용 가능
    → 내부적으로 grid.into_iter()가 호출되고, GridIter가 반환됨

### ✅ FromIterator는 어디에 쓰일까?
```rust
let points: Vec<(u32, u32)> = grid.into_iter().collect();
```

- collect()는 FromIterator를 구현한 타입에 대해 사용 가능
- Vec<T>는 FromIterator<T>를 구현했기 때문에  
    → 반복자를 통해 값을 모아서 벡터로 만들 수 있음

---
# Iterator::next() ?
Iterator::next()만 구현하면 반복은 되는데, 왜 굳이 IntoIterator와 FromIterator까지 들어오냐?
    → 이건 Rust의 표준 순회 생태계와 구조적 통합을 위한 설계적 선택입니다.

## 🧩 핵심 차이: Iterator vs IntoIterator vs FromIterator
| 트레이트 이름       | 핵심 메서드 / 문법 | 역할 설명                                               |
|--------------------|--------------------|----------------------------------------------------------|
| `Iterator`         | `next()`           | 반복자 자체. 하나씩 값을 반환하며 순회 로직을 담당함     |
| `IntoIterator`     | `for x in ...`     | 반복 가능한 구조체. `for` 루프를 가능하게 함             |
| `FromIterator`     | `collect()`        | 반복 결과를 모아서 새로운 컬렉션으로 만드는 수집자 역할  |


## 🔍 왜 IntoIterator가 필요한가?
### ✅ 1. for x in grid 문법을 가능하게 하려면
- Rust는 for 루프에서 IntoIterator를 요구
- Iterator만 구현하면 grid.next()는 가능하지만  
    → for x in grid는 안 됨
### ✅ 2. 소유권을 넘기면서 순회할 수 있게 하려면
- into_iter(self)는 self를 소비하면서 반복자를 반환 
    → 값을 move하면서 순회할 수 있음

## 🔍 왜 FromIterator가 필요한가?
### ✅ 1. collect()를 가능하게 하려면
```rust
let points: Vec<_> = grid.into_iter().collect();
```

- collect()는 내부적으로 FromIterator를 호출
    → 반복자에서 나온 값을 모아서 Vec, HashMap, String 등으로 변환
### ✅ 2. 구조적 수집을 자동화하려면
- FromIterator를 구현하면  
    → 반복자에서 나온 값들을 내가 만든 구조체로 자동 수집 가능

### ✅ 요약: 왜 둘 다 필요한가?
```rust
// 반복자만 있으면
let mut iter = grid.into_iter(); // 직접 호출해야 함
while let Some(xy) = iter.next() { println!("{:?}", xy); }

// IntoIterator가 있으면
for xy in grid { println!("{:?}", xy); } // 더 자연스럽고 idiomatic

// FromIterator가 있으면
let vec: Vec<_> = grid.into_iter().collect(); // 자동 수집 가능
```

---


## 🧩 비교 요약: Rust vs C#
| 개념 역할         | Rust                          | C#                                |
|------------------|-------------------------------|------------------------------------|
| 반복 가능한 구조 | `IntoIterator`                | `IEnumerable<T>`                  |
| 반복자 자체      | `Iterator::next()`            | `IEnumerator<T>::MoveNext()`      |
| 반복 결과 수집   | `FromIterator::collect()`     | `List<T>::Add()` 또는 `ToList()`  |
| 반복 문법        | `for x in collection`         | `foreach (var x in collection)`   |

✅ Rust에서의 구조적 차이
- Rust는 IntoIterator를 통해 소유권을 넘기면서 반복 가능
- Iterator는 next()를 통해 값을 하나씩 반환
- FromIterator는 collect()를 통해 반복 결과를 구조로 수집  
    → C#에서는 foreach가 IEnumerable<T>를 요구하지만,  
    → Rust에서는 for가 IntoIterator를 요구하고,  
    → 반복자 자체는 Iterator로 분리되어 있음  
