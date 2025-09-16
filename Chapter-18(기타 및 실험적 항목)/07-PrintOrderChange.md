# Print Order Change
println! 포맷팅은 정말 강력하고 유연.
출력 순서 변경, 명명된 인자 사용, 패딩과 정렬까지 핵심 포인트를 체계적으로 정리.

### 🧠 1. 출력 순서 변경 – 인덱스 기반
```rust
println!("This is {1} {2}, son of {0} {2}.", father, son, family);
```
- {0}, {1}, {2}는 인자 순서를 지정
- 출력: 
```
This is Adrian Fahrenheit Țepeș, son of Vlad Țepeș.
```

#### 전체 코드
```rust
fn main() {
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!("This is {1} {2}, son of {0} {2}.", father_name, son_name, family_name);
    //This is Adrian Fahrenheit Țepeș, son of Vlad Țepeș.
}
```

## ✅ Tip: 인덱스를 명시하면 순서가 바뀌어도 안전하게 출력 가능

### 🧠 2. 명명된 인자 사용 – 가독성 향상
```rust
println!(
    "{city1} is in {country} and {city2} is also in {country}, but {city3} is not in {country}.",
    city1 = "Seoul",
    city2 = "Busan",
    city3 = "Tokyo",
    country = "Korea"
);
```

- 변수명을 직접 지정해 포맷에 삽입
- 출력: 
```
Seoul is in Korea and Busan is also in Korea, but Tokyo is not in Korea.
```

✅ Tip: 긴 문자열이나 다중 인자일 때 명명된 인자가 훨씬 명확함



### 🧠 3. 패딩과 정렬 – 시각적 포맷 제어
중앙 정렬 + 커스텀 패딩
```rust
println!("{:=^11}", "a"); 
```

- 출력: 
```
=====a=====
```

- =: 패딩 문자
- ^: 중앙 정렬
- 11: 전체 너비
좌우 정렬 + 공백 패딩
```rust
println!("{: <15}{: >15}", "|", "|");
```

- 첫 번째 |: 왼쪽 정렬
- 두 번째 |: 오른쪽 정렬
- 출력: |                            |
명명된 인자 + 방향별 패딩
```rust
println!("{city1:-<15}{city2:->15}", city1 = "SEOUL", city2 = "TOKYO");
```

- -<15: 왼쪽 정렬, -로 채움
- ->15: 오른쪽 정렬, -로 채움
- 출력: 
```
SEOUL--------------------TOKYO
```
### 전체 소스
```rust
fn main() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
}
```
- 출력
```
// ---------TODAY'S NEWS---------
// |                            |
// SEOUL--------------------TOKYO

```
✅ Tip: 시각적으로 정렬된 출력이 필요할 때 매우 유용

## 📦 포맷 요약표
| 표현             | 패딩 문자 / 정렬 방식 | 출력 예시                  |
|------------------|------------------------|-----------------------------|
| `{0}{1}`         | 인덱스 기반             | `This is son of father`     |
| `{name = "x"}`   | 명명된 인자             | `x is in Korea`             |
| `{:=^11}`        | `=` 중앙 정렬           | `=====a=====`               |
| `{: <15}`        | 공백 왼쪽 정렬          | `|              `           |
| `{:->15}`        | `-` 오른쪽 정렬         | `-------------TOKYO`        |

---


