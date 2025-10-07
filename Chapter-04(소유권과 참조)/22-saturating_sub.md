# saturating_sub
saturating_sub는 Rust에서 정수 타입의 안전한 뺄셈 연산을 제공하는 메서드.

## 핵심: 
언더플로우가 발생할 경우, 0으로 "포화(saturate)"시켜서 안전하게 처리한다.


## 🔧 기본 개념
```rust
let a: u8 = 10;
let b: u8 = 20;
let result = a.saturating_sub(b); // result = 0
```

- 일반적으로 10 - 20은 -10인데, u8은 음수를 표현할 수 없음  
- saturating_sub는 이런 경우 패닉을 일으키지 않고, 결과를 0으로 고정

## 🧠 왜 필요한가?
Rust는 기본적으로 정수 오버플로우/언더플로우에 민감:
- 디버그 모드에서는 오버플로우 시 panic!
- 릴리즈 모드에서는 wrap-around (예: 255 + 1 = 0 in u8)
하지만 어떤 상황에서는 오버플로우를 허용하지 않고,  
최댓값 또는 최솟값으로 고정하고 싶을 때가 있음.  
그럴 때 saturating_add, saturating_sub 같은 메서드가 유용.

## ✨ 관련 메서드들
| 메서드 이름         | 설명                                                         |
|--------------------|--------------------------------------------------------------|
| `saturating_add(x)` | 덧셈 결과가 타입의 최대값을 초과하면, 최대값으로 고정         |
| `saturating_sub(x)` | 뺄셈 결과가 0보다 작아지면, 0으로 고정                         |
| `saturating_mul(x)` | 곱셈 결과가 타입의 최대값을 초과하면, 최대값으로 고정         |

## 참고 소스
```rust
let a: u8 = 250;
let b: u8 = 10;
let c = a.saturating_add(b); // c = 255 (u8::MAX)

let x: u8 = 5;
let y: u8 = 10;
let z = x.saturating_sub(y); // z = 0

let m: u32 = 1_000_000;
let n: u32 = 10_000;
let r = m.saturating_mul(n); // r = u32::MAX (포화)
```

## 💡 예시
```rust
fn decrement_counter(counter: u32) -> u32 {
    counter.saturating_sub(1)
}
```

→ counter = 0일 때도 panic 없이 0 유지
→ 게임, UI, 타이머 등에서 자주 쓰이는 패턴

## 🔍 참고 자료
Rust 공식 문서에서도 saturating_sub는  
**"overflow 없이 안전한 산술 연산을 보장하는 방법"** 으로 소개.


## 💬 결론
saturating_sub는 언더플로우를 막고,  
최솟값으로 고정시켜주는 안전한 뺄셈 연산자입니다.  
특히 u8, u32 같은 부호 없는 타입에서 유용하게 쓰임.

---

## 실전 코드

```rust
pub fn from_ctrl(control_points: Vec<Vec<T>>) -> Self {
        assert!(!control_points.is_empty(), "control_points must not be empty");
        let nu = control_points.len();
        let nv = control_points[0].len();
        assert!(nv > 0, "each u-row must have at least one v point");
        for row in &control_points {
            assert!(
                row.len() == nv,
                "non-rectangular control net: expected v-count {}, got {}",
                nv,
                row.len()
            );
        }

        Self {
            degree_u: nu.saturating_sub(1),
            degree_v: nv.saturating_sub(1),
            is_rational_u: false,
            is_rational_v: false,
            is_periodic_u: false,
            is_periodic_v: false,
            control_points,
        }
    }
```

이 코드에서 degree_u: nu.saturating_sub(1)와 degree_v: nv.saturating_sub(1)을 사용한 이유는 안전한 차수 계산을 위해서임.

## 🔍 saturating_sub(1)을 쓴 이유
```rust
let degree_u = nu.saturating_sub(1);
let degree_v = nv.saturating_sub(1);
```

이건 일반적인 nu - 1, nv - 1 대신 **포화 뺄셈(saturating subtraction)** 을 사용.

## 왜?
- nu나 nv가 1일 경우 → degree = 0 (정상)
- nu나 nv가 0일 경우 → degree = 0 (오류 방지)
만약 그냥 nu - 1을 썼다면:
- nu = 0일 때 → 0 - 1 = -1 → usize에서 언더플로우 발생 → 패닉 or wrap-around

## ✅ saturating_sub의 역할
언더플로우가 발생할 경우, 결과를 0으로 고정시켜서 안전하게 처리

즉, 이 코드는 제어점이 너무 적을 때도 안전하게 초기화되도록 설계.

## 💡 왜 assert로 이미 체크했는데도 또 saturating_sub을 썼을까?
```rust
assert!(!control_points.is_empty());
assert!(nv > 0);
```

→ 이 assert들이 이미 nu >= 1, nv >= 1을 보장.
하지만 saturating_sub를 쓰면:
- 코드가 더 견고해지고   
- 나중에 assert가 제거되거나 변경되더라도 안전성 유지
- 테스트나 디버깅 중 실수로 빈 벡터가 들어와도 panic 없이 처리 가능

## 💬 결론
saturating_sub(1)은 언더플로우를 방지하고,  
degree_u, degree_v를 항상 0 이상으로 안전하게 설정하기 위한 선택입니다.  
assert로 이미 체크했더라도, 이건 방어적 프로그래밍.  




