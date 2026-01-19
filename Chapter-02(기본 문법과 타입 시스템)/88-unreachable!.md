# unreachable!
- unreachable!는 러스트에서 **“여기까지 실행이 오면 안 된다”** 는 걸 컴파일러와 런타임에  
  동시에 선언하는 매크로.
- 간단히 말하면:
  - 논리상 절대 도달하면 안 되는 코드 위치에 두는 것
  - 만약 실제로 거기까지 실행이 오면 **패닉(panic)** 을 일으킴

## 1. 기본 동작
```rust
fn foo(p: usize) {
    match p {
        2 => println!("p=2"),
        3 => println!("p=3"),
        _ => unreachable!(),
    }
}
```

- 위 코드에서 foo(2) 또는 foo(3)은 정상 동작하지만,  
- `foo(10)을 호출하면 _ => unreachable!()` 분기로 들어가서:
  - 런타임에 thread 'main' panicked at 'internal error: entered unreachable code' 같은  
    메시지와 함께 패닉 발생

## 2. 왜 쓰는가?
- 논리적으로 모든 경우를 이미 처리했는데, 컴파일러는 그걸 모를 때
```rust
enum EndWhere { Start, End }

fn f(w: EndWhere) {
    match w {
        EndWhere::Start => { /* ... */ }
        EndWhere::End => { /* ... */ }
        // 여기서 더 이상 다른 값이 없다고 "사람은" 아는데,
        // 컴파일러는 enum이 나중에 확장될 수도 있다고 생각할 수 있음
        //_ => unreachable!(), // 이런 식으로 방어적으로 둘 수도 있음
    }
}
```
- 수학적으로/논리적으로 절대 발생할 수 없는 분기
  - 예: 이미 p가 2 또는 3인 걸 위에서 체크했는데,
  - 아래에서 match p { 2 => ..., 3 => ..., _ => unreachable!() } 같은 경우.

## 3. 컴파일러 최적화와의 관계
- unreachable!()는 컴파일러에게도 힌트를 줌:
  - **이 분기는 절대 실행되지 않는다고 가정해도 된다**

- 그래서:
  - 최적화 시 불필요한 분기 제거
  - 패턴 매칭 단순화
  - dead code 제거
- 같은 이점이 생길 수 있음.

## 4. 주의할 점
- unreachable!()가 실제로 실행되면 무조건 패닉이기 때문에,  
  **진짜로 논리상 절대 올 수 없는 곳** 에만 써야 한다.
- 입력 검증을 대신하는 용도로 쓰면 안 됨  
  (그건 return Err(...) 같은 걸로 처리해야 하는 영역)

## 5. 코드에서의 의미
```rust
match p {
    2 => { /* ... */ }
    3 => { /* ... */ }
    _ => unreachable!(),
}
```
- 위처럼 썼다면, 이미 위에서:
```rust
if p != 2 && p != 3 {
    return Err("INP_ERR: N_fitkad requires p=2 or p=3".into());
}
```

- 이렇게 걸러놨기 때문에,
  - 논리상 match p에서 _ 분기는 절대 실행될 수 없고
  - 만약 실행된다면, 그건 버그이므로 바로 패닉을 내는 게 맞다 라는 의도를 코드로 표현.

---

