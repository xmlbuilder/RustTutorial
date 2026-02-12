# expect 던져지 에러 내용 잡는 법
## .expect(...)에서 던져진 에러 내용을 어떻게 출력하느냐?

- 즉, .expect("msg")가 실패했을 때 원래 에러(error) 를 보고 싶은 상황.

### ✔ 결론부터: .expect()는 에러 내용을 “숨긴다”
- Rust의 .expect("msg")는 내부적으로 이렇게 동작:
```rust
match result {
    Ok(v) => v,
    Err(e) => panic!("{}: {:?}", msg, e),
}
```

- 즉:
    - 원래 Err(e) 를 그대로 출력하는 게 아니라
    - 네가 넣은 문자열 + 에러의 Debug 출력을 panic 메시지로 만든다.
- 그래서 .expect("...")를 쓰면 원래 에러 타입이 가진 정보가 일부 가려질 수 있다.

### ✔ 원래 에러를 그대로 보고 싶으면?
#### 1) .unwrap()을 쓰면 원래 에러만 출력됨
```rust
let v = something.unwrap();
```
- panic 메시지는:
```rust
called `Result::unwrap()` on an `Err` value: <원래 에러 내용>
```
- 즉, 에러의 Debug 출력이 그대로 나온다.

#### 2) .expect()에서도 에러를 함께 보고 싶으면?
- 이렇게 쓰면 된다:
```rust
.expect(&format!("something failed: {:?}", e))
```

- 하지만 .expect() 안에서는 e를 직접 접근할 수 없음.
- 그래서 에러를 직접 출력하고 종료하는 방식을 쓰는 게 더 좋다.

#### 3) 가장 깔끔한 방법: match로 직접 에러 출력
```rust
match something {
    Ok(v) => v,
    Err(e) => {
        eprintln!("Error occurred: {:?}", e);
        panic!("Aborting");
    }
}
```

- 이러면:
    - 원래 에러(e) 를 원하는 형식으로 출력 가능
    - panic 메시지는 별도로 설정 가능

#### 4) .unwrap_or_else()를 쓰면 더 깔끔함
- Rust 공식 스타일:
```rust
let v = something.unwrap_or_else(|e| {
    eprintln!("Error: {:?}", e);
    panic!("abort");
});
```

- 이 방식이 가장 많이 쓰인다.

#### 5) .expect()로 원래 에러를 “그대로” 보고 싶다면?
- Rust는 .expect() 안에서 에러를 직접 조작할 수 없기 때문에
- 원래 에러를 그대로 보고 싶으면 .unwrap()을 쓰는 게 정답이다.

## ✔ 요약 (암기용)

| Method             | Shows original error? | Notes                                      |
|--------------------|------------------------|---------------------------------------------|
| .unwrap()          | YES                    | Panic message includes the Err(...) value   |
| .expect("msg")     | PARTIAL                | Shows your msg + Debug of error             |
| match { ... }      | YES (full control)     | You print the error exactly as you want     |
| .unwrap_or_else()  | YES (full control)     | Idiomatic way to log error before panic     |

---



