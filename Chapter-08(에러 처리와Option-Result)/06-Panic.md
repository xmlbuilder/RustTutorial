# Panic

## 🛡️ panic 이후에도 실행을 계속하는 방법
1. std::panic::catch_unwind 사용
Rust 표준 라이브러리에서는 catch_unwind를 통해 panic을 **복구(recover)**할 수 있습니다.

```rust
use std::panic;
 fn main() 
 { 
    let result = panic::catch_unwind(|| 
        { 
            println!("🚀 실행 중..."); panic!("💥 문제가 발생했습니다!"); 
        }
    ); 
    match result { 
        Ok(_) => println!("✅ 정상 종료"), 
        Err(_) => println!("⚠️ panic 발생했지만 복구됨"), 
    } 
    println!("➡️ 다음 단계로 계속 진행"); 
} 
```

## 🔍 설명
- catch_unwind는 클로저 내부에서 발생한 panic을 감지하고 Result로 반환합니다.
-  Err(_)일 경우 panic이 발생한 것이고, 이를 통해 프로그램을 중단하지 않고 다음 로직으로 넘어갈 수 있습니다.

## ⚠️ panic 복구 시 주의사항 (`catch_unwind` 관련)
| 항목         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| `catch_unwind` | panic을 감지하고 `Result`로 반환 → 프로그램 중단 없이 복구 가능       |
| `UnwindSafe`   | `catch_unwind`에 넘기는 클로저는 `UnwindSafe` 트레이트를 만족해야 함   |
| `UnwindSafe + RefCell` | 내부 상태를 변경하는 구조체는 기본적으로 `UnwindSafe`가 아님 → 주의 필요 |
| 제한 사항      | FFI 경계, 멀티스레드 환경에서는 사용에 주의. 남용 시 예측 불가능한 동작 가능성 있음 |


## 🧠 실전 팁
- 테스트 코드에서 #[should_panic]을 활용해 panic을 예상하고 검증할 수 있어요.
- 라이브러리 개발 시에는 catch_unwind를 통해 사용자 코드의 panic을 격리하는 전략이 유용합니다.
- 하지만 일반적인 애플리케이션에서는 panic을 미리 방지하는 로직이 더 바람직합니다 (Result, Option 활용).
---
