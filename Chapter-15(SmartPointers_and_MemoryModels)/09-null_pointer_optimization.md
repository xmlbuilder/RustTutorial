

# null pointer optimization

Rust의 널 포인터 최적화(null pointer optimization) 개념을 실험적으로 보여주는 예제입니다.
아래에 개념 설명, 코드 분석, 그리고 왜 이게 중요한지까지 깊이 있게 정리.

## 🧠 널 포인터 최적화란?
Rust에서는 Option<T> 타입이 있을 때,
일반적으로 T의 크기보다 더 큰 메모리를 차지할 것 같지만,
특정 타입에 대해서는 Option<T>가 T와 같은 크기를 갖도록 최적화합니다.

### 예시:
```rust
size_of::<&i32>() == size_of::<Option<&i32>>() // true
```

- Rust는 참조 타입(&T)에서 **널 포인터(null pointer)**를 None으로 사용하고
- 널이 아닌 포인터를 Some(&T)로 사용함
- 따라서 추가 메모리 없이 Option<&T>를 표현할 수 있음

## 🔍 코드 분석
```rust
use std::mem::transmute;
macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}
```
- transmute는 타입을 강제로 변환하는 unsafe 함수
- dbg_bits! 매크로는 표현식의 비트 패턴을 출력함
- stringify!($e)는 표현식을 문자열로 출력

## 🔧 main() 함수 해석
```rust
unsafe {
    println!("bool:");
    dbg_bits!(false, u8); // 0x0
    dbg_bits!(true, u8);  // 0x1
}
```


- bool은 1바이트로 표현됨
- false → 0x0, true → 0x1

## 🔧 Option<bool> 출력
```rust
dbg_bits!(None::<bool>, u8);       // 0x2
dbg_bits!(Some(false), u8);        // 0x0
dbg_bits!(Some(true), u8);         // 0x1
```

- Rust는 Option<bool>을 1바이트로 표현함
- None은 **불가능한 값(0x2)**로 표현 → bool은 0과 1만 사용하므로 가능
- 이게 바로 enum 최적화의 핵심

## 🔧 Option<Option<bool>>
```rust
dbg_bits!(Some(Some(false)), u8);  // 0x0
dbg_bits!(Some(Some(true)), u8);   // 0x1
dbg_bits!(Some(None::<bool>), u8); // 0x2
dbg_bits!(None::<Option<bool>>, u8); // 0x3
```

- 2단계 Option이지만 여전히 1바이트로 표현
- Rust는 가능한 값의 조합을 분석해서 최적화된 표현을 만들어냄

## 🔧 Option<&i32>
```rust
dbg_bits!(None::<&i32>, usize);     // 0x0
dbg_bits!(Some(&0i32), usize);      // 0x주소값
```

- None은 **널 포인터(0x0)**로 표현
- Some(&0i32)는 실제 메모리 주소값으로 표현
- 이게 바로 널 포인터 최적화의 대표 사례

### ⚠️ 주의사항
이 코드는 비트 패턴을 확인하는 실험용이며,
transmute는 컴파일러가 보장하지 않는 내부 표현을 강제로 해석하는 방식입니다.

- Rust는 비트 패턴을 보장하지 않음
- 이 코드를 실무에서 사용하면 undefined behavior 발생 가능
- 오직 학습용, 디버깅용으로만 사용해야 함


## ✅ 요약: 널 포인터 최적화
| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| 최적화 대상            | Option<&T> 타입                                                     |
| 크기 비교              | size_of::<Option<&T>>() == size_of::<&T>()                          |
| 최적화 방식            | None은 null 포인터(0x0), Some은 유효한 주소값으로 표현              |
| 메모리 절약 효과        | 추가 메모리 없이 Option 표현 가능 → 시스템 프로그래밍에 유리          |
| 주의사항               | 비트 패턴은 컴파일러가 보장하지 않음 → transmute 사용은 unsafe       |

## 💡 결론
Rust는 안전성과 성능을 동시에 추구하는 언어이며,
이런 최적화는 그 철학을 보여주는 대표적인 사례입니다.
하지만 비트 패턴에 의존하는 코드는 위험하므로,
반드시 공식 API와 안전한 추상화를 통해 사용하는 것이 바람직합니다.


