# Default (파생 속성)
Rust에서 #[derive(Default)]가 없으면 에러가 나는 경우는 꽤 자주 발생하는데,  
그 이유는 구조체나 열거형이 Default 트레이트를 요구받는 상황에서 자동 구현이 없기 때문이에요.

## 🧠 #[derive(Default)]의 역할
```rust
#[derive(Default)]
struct Config {
    port: u16,
    host: String,
}
```

- 이 코드는 Config::default()를 자동으로 생성해줘요
- port는 0, host는 ""로 초기화됨 (각 필드의 Default 값)

## ❌ 에러가 나는 대표적인 상황
### 1. T: Default 제네릭 바운드가 있을 때
```rust
fn init<T: Default>() -> T {
    T::default()
}
```
→ 이 함수에서 T가 Default를 구현하지 않으면 컴파일 에러 발생

## 2. Option<T>나 Vec<T> 같은 필드가 있고, Default를 요구받을 때
```rust
struct App {
    config: Option<Config>,
}

fn main() {
    let app = App::default(); // ❌ 에러: App에 Default 구현 없음
}
```

→ 해결: #[derive(Default)] 추가하거나 직접 impl Default 작성

### ⚠️ 왜 자동이 아니냐면…
Rust는 명시적 설계 철학을 따르기 때문에, 모든 필드가 Default를 구현한다고 해서 구조체 전체가 자동으로 Default를 갖는다고 가정하지 않음.
예를 들어 NonZeroUsize 같은 타입은 0으로 초기화하면 런타임 오류가 날 수 있기 때문에, 자동 Default는 위험할 수 있음.

## ✨ 커스터마이징도 가능
```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "localhost".to_string(),
        }
    }
}
```

→ #[derive(Default)] 없이도 직접 구현 가능

## ✅ 요약
| 상황                            | `#[derive(Default)]` 없으면 발생하는 문제 |
|---------------------------------|------------------------------------------|
| `T: Default` 제네릭 요구        | 컴파일 에러                              |
| `.default()` 호출               | 메서드 없음 에러                         |
| 필드가 `Option`, `Vec` 등일 때 | 구조체 전체에 `Default` 없으면 에러      |

---

## default 실전 예제

```rust
#[derive(Clone, Debug, Default)]
pub struct MassProperties {
    pub mass_type: i32, // 2: area, 3: volume (matching original comments)
    pub mass: f64,
    pub valid_mass: bool,
    pub valid_centroid: bool,
    pub x0: f64, pub y0: f64, pub z0: f64,

    pub valid_first: bool,
    pub world_x: f64, pub world_y: f64, pub world_z: f64,

    pub valid_second: bool,
    pub world_xx: f64, pub world_yy: f64, pub world_zz: f64,

    pub valid_product: bool,
    pub world_xy: f64, pub world_yz: f64, pub world_zx: f64,

    pub ccs_xx: f64, pub ccs_yy: f64, pub ccs_zz: f64,
}


impl Default for MassProperties {
    fn default() -> Self {
        Self {
            mass_type: 0,
            mass: 0.0,
            valid_mass: false,
            valid_centroid: false,
            x0: 0.0, y0: 0.0, z0: 0.0,
            valid_first: false,
            world_x: 0.0, world_y: 0.0, world_z: 0.0,
            valid_second: false,
            world_xx: 0.0, world_yy: 0.0, world_zz: 0.0,
            valid_product: false,
            world_xy: 0.0, world_yz: 0.0, world_zx: 0.0,
            ccs_xx: 0.0, ccs_yy: 0.0, ccs_zz: 0.0,
        }
    }
}

```
## 사용법
```rust
let props = MassProperties::default();
```



