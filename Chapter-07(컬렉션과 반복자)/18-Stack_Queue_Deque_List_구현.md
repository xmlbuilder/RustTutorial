# Stack / Queue / Deque / List 구현
Rust에서는 Stack, Queue, Deque, List를 직접 제공하지 않지만,  
표준 라이브러리의 컬렉션을 활용해 효율적으로 구현할 수 있습니다.  
가장 많이 쓰이는 타입은 Vec, VecDeque, LinkedList입니다.

## 🧠 Rust에서 주요 자료구조 처리 방식
| 자료구조   | 타입            | 주요 메서드                          |
|------------|------------------|--------------------------------------|
| Stack      | `Vec<T>`           | `push()`, `pop()`, `last()`                |
| Queue      | `VecDeque<T>`      | `push_back()`, `pop_front()`             |
| Deque      | `VecDeque<T>`      | `push_front()`, `pop_back()`             |
| List       | `LinkedList<T>`    | `push_back()`, `pop_front()`             |

## 🧪 예제 코드
### ✔ Stack (LIFO)
```rust
fn main() {
    let mut stack = Vec::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

### ✔ Queue (FIFO)
```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    while let Some(front) = queue.pop_front() {
        println!("Dequeued: {}", front);
    }
}
```

### ✔ Deque (양방향 큐)
```rust
use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();
    deque.push_front(1);
    deque.push_back(2);
    deque.push_front(0);

    println!("Front: {:?}", deque.front());
    println!("Back: {:?}", deque.back());
}
```

### ✔ List (LinkedList)
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back("A");
    list.push_back("B");
    list.push_front("Start");

    for item in list {
        println!("Item: {}", item);
    }
}
```

---

# 값만 확인 / 제거하면서 확인

`VecDeque` 에서 `pop_front()` 와 `pop_back()` 은 값을 꺼내면서 제거하지만,  
제거하지 않고 값만 확인하는 함수도 있습니다.

## ✅ 값만 확인하는 함수
| 메서드   | 반환 타입     | 설명                         |
|----------|----------------|------------------------------|
| `front()`  | `Option<&T>`     | 맨 앞 요소를 참조로 반환       |
| `back()`   | `Option<&T>`     | 맨 뒤 요소를 참조로 반환       |

이 함수들은 값을 제거하지 않고 `Some(&value)` 또는 `None` 을 반환합니다.

### 🧪 예제
```rust
use std::collections::VecDeque;
fn main() {
    let mut deque = VecDeque::from([10, 20, 30]);

    println!("앞쪽 값: {:?}", deque.front()); // Some(&10)
    println!("뒤쪽 값: {:?}", deque.back());  // Some(&30)

    // 값은 그대로 유지됨
    println!("전체: {:?}", deque); // [10, 20, 30]
}
```
## ✅ 요약
| 목적           | 메서드                    | 값 제거 여부 | 반환 타입       |
|----------------|----------------------------|--------------|-----------------|
| 값 꺼내기      | `pop_front()`, `pop_back() `   | ✅ 제거됨     | Option<T>       |
| 값 확인        | `front()`, `back()`            | ❌ 유지됨     | Option<&T>      |

---

# 사용 방법

## ✅ 언제 어떤 걸 써야 할까?
- `Vec<T>`: 가장 빠르고 일반적인 선택. Stack에 적합.
- `VecDeque<T>`: 양방향 큐가 필요할 때. Queue/Deque에 적합.
- `LinkedList<T>`: 삽입/삭제가 빈번하고 중간 조작이 많을 때. 하지만 성능은 낮음.


##  소유권, 참조, 이터레이터 연동
Rust에서 `Stack`, `Queue`, `Deque`, `List` 자료구조를 다룰 때  
소유권(ownership), 참조(borrowing), 그리고 map/filter 같은 이터레이터 체인과 어떻게 연결되는지 설명.

## 🧠 자료구조별 소유권과 이터레이터 연결 요약
| 자료구조   | 타입            | 소유권 이동 여부       | 참조 유지 여부       | map/filter 연결 방식                      |
|------------|------------------|-------------------------|-----------------------|-------------------------------------------|
| Stack      | `Vec<T>`           | `into_iter()`로 이동    | `iter()`로 참조 유지  | `map(|x| ...)`, `filter(|x| ...)` 가능     |
| Queue      | `VecDeque<T>`      | 동일                    | 동일                  | `iter()`로 순차 처리 가능                  |
| Deque      | `VecDeque<T>`      | 동일                    | 동일                  | 앞뒤 모두 처리 가능                        |
| List       | `LinkedList<T>`    | `into_iter()`로 이동    | `iter()`로 참조 유지  | 느리지만 `map`, `filter` 가능              |


## 🧪 Stack 예제: 소유권 vs 참조 + map 연결

### 소유권 이전 (`into_iter`)
```rust
fn main() {
    let mut stack = vec![10, 20, 30];

    // 소유권 이동: stack은 이후 사용 불가
    let doubled: Vec<i32> = stack.into_iter()
        .map(|x| x * 2)
        .collect();

    println!("Doubled: {:?}", doubled);
    // println!("{:?}", stack); // ❌ 오류: 소유권 이동됨
}
```

### 참조 (`iter`)
```rust
fn main() {
    let stack = vec![10, 20, 30];

    // 참조 유지: stack은 이후에도 사용 가능
    let doubled: Vec<i32> = stack.iter()
        .map(|x| x * 2)
        .collect();

    println!("Doubled: {:?}", doubled);
    println!("Original: {:?}", stack); // ✅ 사용 가능
}
```


## 🧪 Queue 예제: filter로 조건 추출
```rust
use std::collections::VecDeque;

fn main() {
    let queue: VecDeque<i32> = VecDeque::from([1, 2, 3, 4, 5]);
    let evens: Vec<i32> = queue.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x)
        .collect();

    println!("짝수만 추출: {:?}", evens);
    println!("원본 큐: {:?}", queue); // ✅ 참조 기반
}
```


## 🧪 Deque 예제: 앞뒤 처리 + map 연결
```rust
use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();
    deque.push_front("A");
    deque.push_back("B");
    deque.push_front("C");

    let uppercased: Vec<String> = deque.iter()
        .map(|s| s.to_uppercase())
        .collect();

    println!("대문자 변환: {:?}", uppercased);
}
```


## 🧪 List 예제: 소유권 이동과 참조 비교
```rust
use std::collections::LinkedList;

fn main() {
    let list: LinkedList<i32> = LinkedList::from([1, 2, 3, 4]);

    // 참조 기반 filter
    let filtered: Vec<i32> = list.iter()
        .filter(|&&x| x > 2)
        .map(|&x| x)
        .collect();

    println!("필터링 결과: {:?}", filtered);
    println!("원본 리스트: {:?}", list); // ✅ 참조 기반
}
```


## ✅ 핵심 요약
- into_iter() → 소유권 이동. 원본 사용 불가.
- iter() → 참조 유지. 원본 계속 사용 가능.
- map, filter는 iter() 기반으로 안전하게 연결 가능.
- Vec<T>와 VecDeque<T>는 가장 빠르고 유연함.
- LinkedList<T>는 느리지만 중간 삽입/삭제에 유리.

---


# VecDeque<T> into_iter
VecDeque<T>도 into_iter()를 지원합니다  
즉, VecDeque에서도 소유권을 이동하면서 반복(iteration) 할 수 있음.  

| 메서드         | 반복자 타입       | 접근 방식 |
|----------------|-------------------|-----------|
| .iter()        | Iter<'_, T>       | &T        |
| .iter_mut()    | IterMut<'_, T>    | &mut T    |
| .into_iter()   | IntoIter<T>       | T         |

## 🧪 예제: VecDeque에서 into_iter() 사용
```rust
use std::collections::VecDeque;

fn main() {
    let deque: VecDeque<String> = VecDeque::from([
        "JungHwan".to_string(),
        "Rustacean".to_string(),
        "Developer".to_string(),
    ]);

    // 소유권 이동
    let uppercased: Vec<String> = deque.into_iter()
        .map(|s| s.to_uppercase())
        .collect();

    println!("{:?}", uppercased);
    // println!("{:?}", deque); // ❌ 오류: deque는 이동됨
}
```


## ✅ 요약
- VecDeque<T>는 Vec<T>와 동일하게 into_iter()를 지원합니다.
- into_iter()는 소유권을 가져가므로 원본은 이후 사용할 수 없습니다.
- `참조 기반 반복` 이 필요하면 `.iter()` 또는 `.iter_mut()` 을 사용하세요.


## 🧠 VecDeque<T> 주요 함수 요약
| 메서드            | 설명                                 |
|-------------------|--------------------------------------|
| `push_back(x)`      | 뒤에 요소 추가                        |
| `push_front(x)`     | 앞에 요소 추가                        |
| `pop_back()`        | 뒤에서 요소 제거 및 반환              |
| `pop_front()`       | 앞에서 요소 제거 및 반환              |
| `front()`           | 앞 요소 참조 반환 (`Option<&T>`)      |
| `back()`            | 뒤 요소 참조 반환 (`Option<&T>`)      |
| `len()`             | 요소 개수 반환                        |
| `is_empty()`        | 비어 있는지 확인                      |
| `clear()`           | 모든 요소 제거                        |
| `iter()`            | 불변 참조 반복자 (`&T`)               |
| `iter_mut()`        | 가변 참조 반복자 (`&mut T`)           |
| `into_iter()`       | 소유권 이동 반복자 (`T`)              |


## 🧪 VecDeque<T> 함수 예제
```rust
use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();

    // push
    deque.push_back(10);
    deque.push_front(5);

    // pop
    let back = deque.pop_back();   // Some(10)
    let front = deque.pop_front(); // Some(5)

    // 다시 채우기
    deque.push_back(20);
    deque.push_back(30);

    // 조회
    println!("Front: {:?}", deque.front()); // Some(&20)
    println!("Back: {:?}", deque.back());   // Some(&30)

    // 길이
    println!("Length: {}", deque.len());    // 2
    println!("Empty?: {}", deque.is_empty()); // false

    // 반복자
    for x in deque.iter() {
        println!("Iter: {}", x);
    }

    for x in deque.iter_mut() {
        *x *= 2;
    }

    println!("Modified: {:?}", deque);

    // 소유권 이동
    let moved: Vec<i32> = deque.into_iter().map(|x| x + 1).collect();
    println!("Moved: {:?}", moved);
}
```


## 🧠 LinkedList<T> 주요 함수 요약
| 메서드            | 설명                                 |
|-------------------|--------------------------------------|
| `push_back(x)`      | 뒤에 요소 추가                        |
| `push_front(x)`     | 앞에 요소 추가                        |
| `pop_back()`        | 뒤에서 요소 제거 및 반환              |
| `pop_front()`       | 앞에서 요소 제거 및 반환              |
| `front()`           | 앞 요소 참조 반환 (`Option<&T>`)      |
| `back()`            | 뒤 요소 참조 반환 (`Option<&T>`)      |
| `len()`             | 요소 개수 반환                        |
| `is_empty()`        | 비어 있는지 확인                      |
| `clear()`           | 모든 요소 제거                        |
| `iter()`            | 불변 참조 반복자 (`&T`)               |
| `iter_mut()`        | 가변 참조 반복자 (`&mut T`)           |
| `into_iter()`       | 소유권 이동 반복자 (`T`)              |


## 🧪 LinkedList<T> 함수 예제
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    // push
    list.push_back("Rust");
    list.push_front("Hello");

    // pop
    let last = list.pop_back();   // Some("Rust")
    let first = list.pop_front(); // Some("Hello")

    // 다시 채우기
    list.push_back("JungHwan");
    list.push_back("Developer");

    // 조회
    println!("Front: {:?}", list.front()); // Some(&"JungHwan")
    println!("Back: {:?}", list.back());   // Some(&"Developer")

    // 길이
    println!("Length: {}", list.len());    // 2
    println!("Empty?: {}", list.is_empty()); // false

    // 반복자
    for x in list.iter() {
        println!("Iter: {}", x);
    }

    for x in list.iter_mut() {
        *x = &format!("{}!", x);
    }

    println!("Modified: {:?}", list);

    // 소유권 이동
    let moved: Vec<String> = list.into_iter().map(|x| x.to_string()).collect();
    println!("Moved: {:?}", moved);
}
```

## ✅ 선택 팁
- `VecDeque<T>`: 빠르고 유연한 양방향 큐. 대부분의 큐/데크 작업에 적합.
- `LinkedList<T>`: 중간 삽입/삭제가 많을 때 유리하지만 성능은 낮음.
- 두 타입 모두 iter, into_iter, filter, map 등 이터레이터 체인 완벽 지원.

---

# VecDeque와 LinkedList가 흔히 쓰이지 않는 이유

Rust에서 VecDeque와 LinkedList가 흔히 쓰이지 않는 이유는 성능과 메모리 구조 때문이며,  
특정 상황에서는 매우 유용합니다. 아래에 이유와 추천 사용처, 실전 샘플을 정리.

## 🧠 왜 잘 안 쓰일까?
### 🔹 VecDeque<T>가 잘 안 쓰이는 이유
- 내부적으로 ring buffer를 사용 → 일반적인 Vec<T>보다 캐시 친화도가 낮음
- 중간 삽입/삭제는 느림 → 대부분의 작업이 앞/뒤에서만 이뤄질 때만 유리
- Vec<T>로도 대부분의 큐/스택 작업이 충분히 처리됨

### 🔹 LinkedList<T>가 잘 안 쓰이는 이유
- 메모리 분산 구조 → 캐시 효율이 낮고 순회가 느림
- 중간 삽입/삭제는 빠르지만, iterator 위치가 필요 → 실전에서 자주 쓰기 까다로움
- 대부분의 경우 Vec<T>나 VecDeque<T>로 더 빠르게 처리 가능

## ✅ 언제 쓰면 좋을까?
| 상황                         | 추천 타입       | 관련 메서드                  |
|------------------------------|------------------|-------------------------------|
| 양방향 큐 처리               | VecDeque<T>      | push_front(), pop_back()      |
| 실시간 이벤트 큐             | VecDeque<T>      | push_back(), pop_front()      |
| 중간 삽입/삭제가 빈번한 경우 | LinkedList<T>    | cursor_mut(), insert_after()  |
| 리스트 병합/분할이 많은 경우 | LinkedList<T>    | append(), split_off()         |

## 🧪 실전 샘플
### ✔ VecDeque: 실시간 이벤트 큐
```rust
use std::collections::VecDeque;

fn main() {
    let mut events = VecDeque::new();

    // 이벤트 발생
    events.push_back("MouseMove");
    events.push_back("KeyPress");

    // 이벤트 처리
    while let Some(event) = events.pop_front() {
        println!("Handling event: {}", event);
    }
}
```


### ✔ VecDeque: 양방향 명령 스택
```rust
use std::collections::VecDeque;

fn main() {
    let mut history = VecDeque::new();

    history.push_back("Open File");
    history.push_back("Edit Line");
    history.push_front("Undo");

    println!("Recent: {:?}", history.front());
    println!("Last: {:?}", history.back());
}
```


### ✔ LinkedList: 중간 삽입/삭제 (`insert_after`)
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from([1, 2, 4]);

    // 중간 삽입: 3을 2 뒤에 삽입
    let mut cursor = list.cursor_front_mut();
    while let Some(&val) = cursor.current() {
        if val == 2 {
            cursor.insert_after(3);
            break;
        }
        cursor.move_next();
    }
    println!("List: {:?}", list); //[1, 2, 3, 4]
}
```


### ✔ LinkedList: 리스트 병합 (`append`)
```rust
use std::collections::LinkedList;

fn main() {
    let mut a = LinkedList::from([1, 2]);
    let mut b = LinkedList::from([3, 4]);

    a.append(&mut b); // b는 비워지고 a에 병합됨
    println!("Merged: {:?}", a);
}
```

## ✅ 요약
- VecDeque<T>는 양방향 큐나 실시간 이벤트 처리에 적합
- LinkedList<T>는 중간 삽입/삭제, 리스트 병합/분할에 유리
- 일반적인 경우에는 Vec<T>가 더 빠르고 효율적이기 때문에 덜 쓰임

---

# borrow checker 회피 하거나 우회
Rust의 엄격한 빌림 규칙(borrow checker) 때문에 LinkedList에서  
동시에 여러 가변 참조나 불변 참조 + 가변 참조를 시도하면 컴파일 오류가 발생합니다.  
하지만 이걸 회피하거나 우회하는 방법을 소개 합니다.

## 🧠 왜 오류가 나는가?
Rust는 다음을 동시에 허용하지 않습니다:

- ✅ 여러 개의 가변 참조 (&mut)
- ✅ **불변 참조 (&)** 와 **가변 참조 (&mut)** 를 동시에
```rust
let mut list = LinkedList::new();
let a = list.front();       // 불변 참조
let b = list.front_mut();   // ❌ 오류: 불변 참조가 살아있음
```

## ✅ 회피 방법
### 1. 스코프 분리로 참조 수명 분리
```rust
let mut list = LinkedList::from([1, 2, 3]);

{
    let front = list.front(); // 불변 참조
    println!("Front: {:?}", front);
} // 참조가 여기서 drop됨

{
    let front_mut = list.front_mut(); // ✅ 이제 가변 참조 가능
    *front_mut = 10;
}
```

### 2. pop_front() / pop_back()으로 소유권 가져오기
```rust
let mut list = LinkedList::from([1, 2, 3]);

if let Some(mut val) = list.pop_front() {
    val += 100;
    println!("Modified: {}", val);
    list.push_front(val); // 다시 넣기
}
```
### 3. cursor_mut() 사용으로 중간 삽입/삭제 처리
```rust
let mut list = LinkedList::from([1, 2, 3]);

let mut cursor = list.cursor_front_mut();
while let Some(&val) = cursor.current() {
    if val == 2 {
        cursor.insert_after(99); // 중간 삽입
        break;
    }
    cursor.move_next();
}
```
- cursor_mut()은 내부적으로 단일 가변 참조를 유지하므로 안전하게 중간 조작이 가능합니다.

###  4. RefCell로 런타임 체크 기반 내부 가변성 사용 (`borrow`)
```rust
use std::cell::RefCell;
use std::collections::LinkedList;

fn main() {
    let list = RefCell::new(LinkedList::from([1, 2, 3]));
    {
        let front = list.borrow().front(); // 불변 참조
        println!("Front: {:?}", front);
    }

    {
        let mut front_mut = list.borrow_mut();
        if let Some(x) = front_mut.front_mut() {
            *x = 42;
        }
    }

    println!("Modified list: {:?}", list.borrow());
}
```

#### ⚠️ RefCell은 런타임에 빌림 오류를 검사하므로 단일 스레드에서만 사용 가능합니다.

## ✅ 요약
| 상황 또는 전략                  | 사용 방법 또는 메서드              |
|--------------------------------|-----------------------------------|
| 중간 삽입/삭제를 안전하게 처리 | cursor_mut()                      |
| 값 소유권으로 수정 후 재삽입    | pop_front(), push_front()         |
| 런타임 빌림 검사로 유연하게 처리 | RefCell<LinkedList<T>>            |

---

# Vec & LinkedList 비교

Vec은 중간 삽입이나 병합 시 전체 데이터를 이동해야 하므로 비용이 크고,  
LinkedList는 노드 기반이라 중간 삽입과 병합이 더 효율적입니다.  
하지만 Vec은 캐시 친화적이고 대부분의 경우 더 빠릅니다.

## 🧠 핵심 차이: 중간 삽입과 병합
| 작업 유형     | Vec<T> 사용법                | LinkedList<T> 사용법                  |
|---------------|------------------------------|----------------------------------------|
| 중간 삽입     | `insert(index, value)`         | `cursor_mut().insert_after()`           |
| 병합 (뒤에 추가) | `extend()`, `splice()`            | `append()`                               |
| 리스트 분할    | (직접 구현 필요)             | `split_off(index)`                       |
| 성능 특성     | 빠르지만 중간 삽입은 느림     | 중간 삽입/삭제에 유리, 순회는 느림     |



## 🧪 중간 삽입 예제
### ✔ Vec 중간 삽입 (`insert`)
```rust
fn main() {
    let mut vec = vec![1, 2, 4];
    vec.insert(2, 3); // 2번 인덱스에 3 삽입
    println!("{:?}", vec); // [1, 2, 3, 4]
}
```

### ✔ LinkedList 중간 삽입 (`cursor_mut().insert_after()`)
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from([1, 2, 4]);
    let mut cursor = list.cursor_front_mut();

    while let Some(&val) = cursor.current() {
        if val == 2 {
            cursor.insert_after(3);
            break;
        }
        cursor.move_next();
    }

    println!("{:?}", list); // [1, 2, 3, 4]
}
```

## 🧪 병합 예제
### ✔ Vec 병합 (extend 또는 splice)

#### `extend`
```rust
fn main() {
    let mut a = vec![1, 2];
    let b = vec![3, 4];

    a.extend(b); // 병합
    println!("{:?}", a); // [1, 2, 3, 4]
}
```

####  `splice`
```rust
fn main() {
    let mut vec = vec![1, 5];
    let slice = &[2, 3, 4];

    vec.splice(1..1, slice.iter().cloned()); // 1번 위치에 삽입
    println!("{:?}", vec); // [1, 2, 3, 4, 5]
}
```


### ✔ LinkedList 병합 (`append`)
```rust
use std::collections::LinkedList;

fn main() {
    let mut a = LinkedList::from([1, 2]);
    let mut b = LinkedList::from([3, 4]);

    a.append(&mut b); // b는 비워지고 a에 병합됨
    println!("{:?}", a); // [1, 2, 3, 4]
}
```
## ✅ 언제 어떤 걸 써야 할까?
- Vec<T>: 대부분의 경우 추천. 빠르고 단순. 중간 삽입은 비용이 크지만 병합은 extend로 간단.
- LinkedList<T>: 중간 삽입/삭제가 빈번하거나 병합/분할이 자주 일어나는 경우에 적합.

---

# VecDeque<T> / LinkedList<T>

아래는 VecDeque<T>와 LinkedList<T>의 비교, split_off()를 활용한 리스트 분할,  
그리고 병렬 처리에 적합한 구조까지 실전 중심으로 완전 정리한 자료입니다.

## 🧠 VecDeque<T> vs LinkedList<T> 비교
| 항목             | VecDeque<T>                         | LinkedList<T>                          |
|------------------|--------------------------------------|----------------------------------------|
| 내부 구조        | Ring buffer                         | Doubly linked list                     |
| 앞/뒤 삽입/삭제  | 빠름 (O(1))                          | 빠름 (O(1))                            |
| 중간 삽입/삭제   | 느림 (O(n))                          | 빠름 (O(1) with cursor_mut)            |
| 순회 성능        | 빠름 (캐시 친화적)                   | 느림 (메모리 분산)                     |
| 병합/분할        | 직접 구현 필요                       | append(), split_off()로 간단 처리      |
| 추천 용도        | 양방향 큐, 이벤트 큐                 | 중간 조작, 리스트 병합/분할            |


## ✂️ `split_off()` 를 활용한 리스트 분할
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from([1, 2, 3, 4, 5]);

    // 3번째 이후를 분할
    let second_half = list.split_off(3);

    println!("앞쪽 리스트: {:?}", list);        // [1, 2, 3]
    println!("뒤쪽 리스트: {:?}", second_half); // [4, 5]
}
```
- split_off(index)는 해당 인덱스부터 끝까지를 새로운 리스트로 분리합니다.


## ⚙️ 병렬 처리에 적합한 구조
Rust에서는 병렬 처리 시 **데이터 경쟁(race condition)** 을 피하기 위해  
스레드 안전한 컨테이너를 사용해야 합니다.

## ✅ 추천 구조
| 구조                         | 설명 및 용도                                      |
|------------------------------|--------------------------------------------------|
| Arc<Mutex<T>>                | 여러 스레드에서 안전하게 공유. 락 기반 동기화. 단일 접근. |
| Arc<RwLock<T>>               | 읽기 다중, 쓰기 단일. 읽기 성능 우선 시 적합.           |
| crossbeam::deque::Worker     | lock-free 작업 큐. 스레드 풀에서 작업 분배에 적합.       |
| flume::Sender/Receiver       | 채널 기반 큐. 생산자-소비자 모델에 적합.                |

## 🧪 병렬 처리 예제: `Arc<Mutex<VecDeque>>`
```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;

fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    let handles: Vec<_> = (0..5).map(|i| {
        let q = Arc::clone(&queue);
        thread::spawn(move || {
            let mut q = q.lock().unwrap();
            q.push_back(i);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("최종 큐: {:?}", queue.lock().unwrap());
}
```


## ✅ 요약
| 기능/상황         | 추천 구조 또는 메서드             |
|-------------------|-----------------------------------|
| 양방향 큐         | VecDeque<T>                       |
| 중간 삽입/삭제    | LinkedList<T> + cursor_mut()      |
| 리스트 병합/분할  | LinkedList<T> + append(), split_off() |
| 병렬 처리         | Arc<Mutex<T>>, Arc<RwLock<T>>     |

---

# 변환 패턴

Rust의 `LinkedList<T>` 는 `Vec<T>` 로 바로 변환하는 전용 메서드가 없습니다.  
따라서 iter() 또는 into_iter()를 활용해서 수동으로 변환해야 함.

## ✅ LinkedList → Vec 변환 방법

### ✔ 방법 1: `iter().cloned().collect()`
```rust
use std::collections::LinkedList;

fn main() {
    let list: LinkedList<i32> = LinkedList::from([1, 2, 3]);
    let vec: Vec<i32> = list.iter().cloned().collect();

    println!("{:?}", vec); // [1, 2, 3]
}

iter()는 참조를 반환하므로 cloned()로 값 복사 필요
```

### ✔ 방법 2: `into_iter().collect()` (소유권 이동)
```rust
let list: LinkedList<i32> = LinkedList::from([1, 2, 3]);
let vec: Vec<i32> = list.into_iter().collect();
```
이 방식은 list를 더 이상 사용할 수 없음 (소유권 이동됨)


## ✅ 요약
| 목적                     | 방법                              | 특징                     |
|--------------------------|-----------------------------------|--------------------------|
| 참조 기반 복사           | iter().cloned().collect()         | 원본 유지됨              |
| 소유권 이동 기반 변환    | into_iter().collect()             | 원본 소멸됨              |
| 전용 변환 메서드         | ❌ 없음                            | 수동 변환 필요           |


## 전체 변환 패턴

아래는 Vec, VecDeque, LinkedList 간의 변환 패턴을 실전 예제와 함께 완전 정리한 자료입니다.

## 🔄 변환 패턴 요약
| 변환 방향               | 방법                                | 특징                     |
|------------------------|-------------------------------------|--------------------------|
| `Vec → VecDeque`         | `VecDeque::from(vec)`               | 빠름                     |
| `Vec → LinkedList`       | `LinkedList::from(vec)`             | 빠름                     |
| `VecDeque → Ve`c         | `deque.iter().cloned().collect()`   | 값 복사 필요             |
|` LinkedList → Vec`       | `list.iter().cloned().collect()`    | 값 복사 필요             |
| `VecDeque → LinkedList`  | `LinkedList::from(deque.iter().cloned())` | 중간 복사 필요     |
| `LinkedList → VecDeque`  | `VecDeque::from(list.iter().cloned())`   | 중간 복사 필요     |


## 🧪 실전 예제
### ✔ Vec → VecDeque
```rust
use std::collections::VecDeque;

let vec = vec![1, 2, 3];
let deque: VecDeque<_> = VecDeque::from(vec);
```


###  ✔ Vec → LinkedList
```rust
use std::collections::LinkedList;

let vec = vec![1, 2, 3];
let list: LinkedList<_> = LinkedList::from(vec);
```


### ✔ VecDeque → Vec
```rust
let deque = VecDeque::from([1, 2, 3]);
let vec: Vec<_> = deque.iter().cloned().collect();
```


### ✔ LinkedList → Vec
```rust
let list = LinkedList::from([1, 2, 3]);
let vec: Vec<_> = list.iter().cloned().collect();
```


### ✔ VecDeque → LinkedList
```rust
let deque = VecDeque::from([1, 2, 3]);
let list: LinkedList<_> = LinkedList::from(deque.iter().cloned());
```


### ✔ LinkedList → VecDeque
```rust
let list = LinkedList::from([1, 2, 3]);
let deque: VecDeque<_> = VecDeque::from(list.iter().cloned());
```


## ✅ 요약 포인트
- Vec은 대부분의 컬렉션으로 직접 변환 가능
- VecDeque와 LinkedList 간 변환은 iter().cloned() 필요
- 소유권을 유지하려면 iter() + cloned()
- 소유권을 넘겨도 된다면 into_iter() 사용 가능

---

