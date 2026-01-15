# Preference / Config

- 이 구조는 사용자 설정을 메모리와 파일에 안전하게 저장하고 불러오는 하이브리드 설정 시스템.  
- 아래에 전체 구조, 기능, 사용된 알고리즘을 정리.

## 🧱 전체 구조 요약

| 구성 요소                     | 역할                                           | 방향성               |
|------------------------------|------------------------------------------------|----------------------|
| `Preferences`                | 사용자 설정을 메모리에 저장하는 구조체         | 상태 저장            |
| `RwLock<Preferences>`        | 스레드 안전한 전역 접근을 위한 락              | 동시성 제어          |
| `Config`                     | TOML 기반 설정 파일을 읽고 key-value로 구성     | 설정 로딩            |
| `on_save_config_to_toml()`   | `Config` 객체를 TOML 파일로 저장               | Config → TOML        |
| `on_load_config_from_file()` | TOML 파일을 `Config`로 로드                    | TOML → Config        |
| `on_save_config_from_pref()` | `Preferences` 값을 `Config`에 주입             | Preferences → Config |
| `on_load_config_from_pref()` | `Config` 값을 `Preferences`에 반영             | Config → Preferences |

## ⚙️ 주요 기능 설명
### 1️⃣ Preferences: 메모리 기반 사용자 설정
```rust
#[derive(Debug, Clone, Default)]
pub struct Preferences {
    pub global_mesh: f64,
    pub local_mesh_sizes: HashMap<usize, f64>,
}
```
- global_mesh: 전체 메시 크기
- local_mesh_sizes: 특정 ID별 메시 크기
- RwLock으로 감싸서 동시 접근 가능
- on_get_*, on_set_*, on_push_* 함수들로 접근

### 2️⃣ Config: 파일 기반 설정 저장소
- `Config::builder().add_source(File::with_name("Settings"))` 로 TOML 파일 로드
- set_override()로 메모리에서 설정값 덮어쓰기 가능
- get()으로 설정값 읽기

### 3️⃣ 저장 흐름: Preferences → Config → TOML
```rust
on_save_config_from_pref(); // prefs → config
on_save_config_to_toml(&config, path); // config → TOML
```
- prefs().read()로 현재 설정 읽기
- config.set("prefs.global_mesh", value)로 설정값 주입
- try_deserialize()로 전체 맵 추출 후 TOML 변환

### 4️⃣ 불러오기 흐름: TOML → Config → Preferences
```rust
let config = on_load_config_from_file(path); // TOML → config
on_set_global_mesh(config.get("prefs.global_mesh").unwrap()); // config → prefs
```

- 설정 파일에서 prefs.global_mesh 키를 읽어 Preferences에 반영
- 로컬 메시 크기 맵은 아직 연동되지 않았지만 확장 가능

## 🔁 사용된 알고리즘 및 패턴

| 기술/패턴                  | 설명 또는 반환 타입                     |
|---------------------------|----------------------------------------|
| `RwLock`                  | 스레드 안전한 읽기/쓰기 동시 접근 제어 |
| `Lazy`                    | 전역 싱글톤 초기화 (지연 로딩)         |
| `Config`                  | 설정 계층 구성 및 key-value 관리       |
| `try_deserialize()`       | `HashMap<String, Value>`로 설정 추출   |
| `toml::to_string()`       | TOML 문자열로 직렬화                   |
| `fs::write()` / `fs::read_to_string()` | 파일 저장 및 로딩 처리         |


## 🧪 테스트 구조
- preference_test: 메모리 기반 설정 조작 확인
- test_config_mem: TOML 문자열을 직접 메모리에 로드
- test_config: 설정 저장 및 불러오기 흐름 검증

## ✅ 요약
- 이 구조는 다음을 만족합니다:
    - 메모리 기반 설정 관리 (Preferences)
    - 파일 기반 설정 저장/복원 (Config + TOML)
    - 스레드 안전성 (RwLock)
    - 확장 가능성 (로컬 메시 맵, 사용자 정보, UI 설정 등 추가 가능)


## ✅ 1. Cargo.toml에 필요한 크레이트 추가
```
[dependencies]
config = "0.13"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
```


## ✅ 2. 코드 상단에 use 추가
```rust
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;
```
- serde_json::Value → JSON 구조를 표현하는 타입
- toml::Value → TOML 구조를 표현하는 타입

## ✅ 전체 함수 예시 (정상 작동)
```rust
use config::Config;
use std::fs;
use std::path::PathBuf;
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

pub fn on_save_config_to_toml(config: &Config, path: PathBuf) {
    // 1. config → serde_json::Value
    let json_value: JsonValue = config.clone().try_deserialize().unwrap();

    // 2. json → toml
    let toml_value: TomlValue = toml::Value::try_from(json_value).expect("Failed to convert to TOML");

    // 3. TOML 문자열로 직렬화
    let toml_str = toml::to_string(&toml_value).expect("Failed to serialize to TOML");

    // 4. 파일 저장
    fs::write(path, toml_str).expect("Failed to write TOML file");
}
```

## ✅ 요약

| 타입 이름            | 필요 크레이트 및 `use` 경로           |
|---------------------|----------------------------------------|
| `JsonValue`         | `serde_json` 크레이트<br>`use serde_json::Value` |
| `TomlValue`         | `toml` 크레이트<br>`use toml::Value`         |

---

## 소스 코드

```rust
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use config::{Config, File};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;


#[derive(Debug, Clone, Default)]
pub struct Preferences {
    pub global_mesh: f64,
    pub local_mesh_sizes: HashMap<usize, f64>,
}
```
```rust
static PREFS: Lazy<RwLock<Preferences>> = Lazy::new(|| RwLock::new(Preferences::default()));
#[inline]
fn prefs() -> &'static RwLock<Preferences> {
    &PREFS
}
```
```rust
pub fn on_get_global_mesh() -> f64 {
    prefs().read().global_mesh
}
```
```rust
pub fn on_set_global_mesh(v: f64) {
    prefs().write().global_mesh = v;
}
```
```rust
pub fn on_get_local_mesh_sizes() -> HashMap<usize, f64> {
    prefs().read().local_mesh_sizes.clone()
}
```
```rust
pub fn on_set_local_mesh_sizes(k : Vec<usize>, v: Vec<f64>) {
    if k.len() != v.len() {
        prefs().write().local_mesh_sizes.clear();
        for i in 0..k.len() {
            prefs().write().local_mesh_sizes.insert(k[i], v[i]);
        }
    }
}
```
```rust
pub fn on_push_local_mesh_size(k: usize, x: f64) {
    prefs().write().local_mesh_sizes.insert(k, x);
}
```
```rust
pub fn on_get_local_mesh_size(k: usize) -> f64 {  prefs().read().local_mesh_sizes.get(&k).unwrap().clone() }
```
```rust
pub fn on_clear_local_mesh_sizes() {
    prefs().write().local_mesh_sizes.clear();
}
```
```rust
pub fn with_read<F, R>(f: F) -> R
where
    F: FnOnce(&Preferences) -> R,
{
    f(&*prefs().read())
}
```
```rust
pub fn with_write<F, R>(f: F) -> R
where
    F: FnOnce(&mut Preferences) -> R,
{
    f(&mut *prefs().write())
}
```
```rust
pub fn on_reset_to_default() {
    *prefs().write() = Preferences::default();
}
```
```rust
pub fn on_save_config_from_pref() {
    let prefs = prefs().read();

    let mut settings = Config::builder()
        .add_source(File::with_name("Settings"))
        .set_override("prefs.global_mesh", prefs.global_mesh)
        .unwrap();

    for (k, v) in prefs.local_mesh_sizes.iter() {
        let key = format!("prefs.local_mesh_sizes.{}", k);
        settings = settings.set_override(key, *v).unwrap();
    }

    let config = settings.build().unwrap();
    on_save_config_to_toml(&config, PathBuf::from("Settings.toml"));
}
```
```rust
pub fn on_load_config_from_pref() {
    let config = on_load_config_from_file(PathBuf::from("Settings.toml"));

    let global_mesh: f64 = config.get("prefs.global_mesh").unwrap();
    on_set_global_mesh(global_mesh);

    let mut local_map = HashMap::new();
    if let Ok(table) = config.get_table("prefs.local_mesh_sizes") {
        for (k, v) in table.iter() {
            if let Ok(id) = k.parse::<usize>() {
                if let Some(val) = v.clone().into_float().ok() {
                    local_map.insert(id, val);
                }
            }
        }
    }
    prefs().write().local_mesh_sizes = local_map;
}
```
```rust
pub fn on_save_config_to_toml(config: &Config, path: PathBuf) {
    // 1. config → serde_json::Value
    let json_value : JsonValue  = config.clone().try_deserialize().unwrap();

    // 2. json → toml
    let toml_value : TomlValue = toml::Value::try_from(json_value).expect("Failed to convert to TOML");

    // 3. TOML 문자열로 직렬화
    let toml_str = toml::to_string(&toml_value).expect("Failed to serialize to TOML");

    // 4. 파일 저장
    fs::write(path, toml_str).expect("Failed to write TOML file");
}
```
```rust
pub fn on_load_config_from_file(path: PathBuf) -> Config {
    Config::builder()
        .add_source(File::with_name(path.to_str().unwrap()).required(true)) // 예: "Settings" → Settings.toml
        .build()
        .expect("Failed to load config")
}
```
```rust
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use config::{Config, File};
    use nurbslib::core::preference::{on_load_config_from_file, on_save_config_to_toml};

    #[test]
    fn test_config_mem() {
        let settings = Config::builder()
            .add_source(config::File::from_str(
                r#"
            username = "junghwan"
            theme = "dark"
            "#,
                config::FileFormat::Toml,
            ))
            .build()
            .unwrap();

        let username: String = settings.get("username").unwrap();
        println!("Username: {}", username);
    }
```
```rust
    #[test]
    fn test_config() {
        {
            let mut settings = Config::builder()
                .add_source(File::with_name("Settings"))
                .set_override("username", "Hyangseon")
                .unwrap()
                // set 대신 set_override 사용
                // Settings.toml 또는 Settings.ini
                .build()
                .unwrap();

            on_save_config_to_toml(&mut settings, PathBuf::from("Settings.toml"));
        }

        {
            let mut settings = Config::builder()
                .add_source(File::with_name("Settings"))
                .build()
                .unwrap();


            on_load_config_from_file(PathBuf::from("Settings.toml"));
            let username: String = settings.get("username").unwrap();
            println!("Username: {}", username);
        }
    }
}
```
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::preference::{on_clear_local_mesh_sizes, on_get_global_mesh, 
    on_get_local_mesh_sizes, on_push_local_mesh_size, on_set_global_mesh, with_write};

    #[test]
    fn preference_test() {
        on_set_global_mesh(5.0);
        on_push_local_mesh_size(1, 0.5);
        on_push_local_mesh_size(2, 1.0);

        with_write(|p| {
            // 여전히 락은 내부에서만 사용됨
            p.local_mesh_sizes.retain(|&x, &mut val| x >= 2);
            p.global_mesh = 8.0;
        });

        println!("gm = {}", on_get_global_mesh());
        println!("locals = {:?}", on_get_local_mesh_sizes());
    }
```
```rust
    #[test]
    fn test_preferences_save_and_load() {
        use std::path::PathBuf;
        use nurbslib::core::preference::{
            on_set_global_mesh, on_push_local_mesh_size,
            on_save_config_from_pref, on_load_config_from_pref,
            on_get_global_mesh, on_get_local_mesh_size,
        };

        // 1. 설정 초기화
        on_set_global_mesh(3.14);
        on_push_local_mesh_size(101, 0.1);
        on_push_local_mesh_size(202, 0.2);

        // 2. 저장
        on_save_config_from_pref();

        // 3. 초기화 후 불러오기
        on_set_global_mesh(0.0);
        on_clear_local_mesh_sizes();
        on_load_config_from_pref();

        // 4. 검증
        assert_eq!(on_get_global_mesh(), 3.14);
        assert_eq!(on_get_local_mesh_size(101), 0.1);
        assert_eq!(on_get_local_mesh_size(202), 0.2);

        println!("✅ Preferences restored successfully.");
    }
}
```
---
