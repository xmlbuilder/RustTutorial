# iter_mut().collect()
- iter_mut().collect()로 만든 Vec<&mut NurbsSurface>를 함수 인자로 넘기는 방법.


## 🧩 1. 현재 코드
### 🧩 1. 첫번째 방식
```rust
let mut refs: Vec<&mut NurbsSurface> = surs.iter_mut().collect();
```


### 🧩 2. 두번째 형태
- ✔ Vec<&mut NurbsSurface>로 모으려면 이렇게 해야 한다
```rust
let mut refs: Vec<&mut NurbsSurface> = surs.iter_mut().collect::<Vec<_>>();
```

- 또는 더 간단하게:
```rust
let mut refs = surs.iter_mut().collect::<Vec<&mut NurbsSurface>>();
```

- 이렇게 하면 refs는:
```rust
Vec<&mut NurbsSurface>
```

- 타입이 된다.

### 🧩 3. 함수 인자를 &mut [&mut NurbsSurface]로 받고 싶다면?
- Rust에서 함수 시그니처는 이렇게 만들 수 있음:
```rust
fn process_surfaces(surs: &mut [&mut NurbsSurface]) {
    // surs[i] 로 접근 가능
}
```

- 그럼 호출은 이렇게 하면 된다:
```rust
let mut refs: Vec<&mut NurbsSurface> = surs.iter_mut().collect();
process_surfaces(&mut refs[..]);
```

- 여기서 &mut refs[..]는:
    - &mut [T] 슬라이스
    - 즉 &mut [&mut NurbsSurface] 타입으로 자동 변환된다

### 🧩 4. 전체 예제
```rust
fn process_surfaces(surs: &mut [&mut NurbsSurface]) {
    for s in surs.iter_mut() {
        s.do_something();
    }
}

fn main() {
    let mut surs: Vec<NurbsSurface> = load_surfaces();

    // Vec<&mut NurbsSurface> 생성
    let mut refs = surs.iter_mut().collect::<Vec<&mut NurbsSurface>>();

    // 함수 호출
    process_surfaces(&mut refs[..]);
}
```


## 🧩 5. 핵심 요약
| 목적                         | 코드                                                                 |
|------------------------------|----------------------------------------------------------------------|
| Vec<&mut NurbsSurface> 생성 | let mut refs = surs.iter_mut().collect::<Vec<&mut NurbsSurface>>(); |
| 함수 인자를 &mut [&mut ...] | fn f(s: &mut [&mut NurbsSurface]) {}                                 |
| Vec을 함수에 넘기기         | f(&mut refs[..]);                                                    |

---
