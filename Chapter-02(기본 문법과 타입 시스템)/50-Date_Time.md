# Date / Time

Rust도 날짜와 시간 처리를 아주 잘 지원합니다.  
Rust에서는 라는 크레이트(라이브러리)를 주로 사용하며,  
Java나 Kotlin에서 했던 대부분의 작업을 유사하게 구현할 수 있음.  
Rust에서 어떻게 표현되는지 간단히 정리:

## 🛠️ Cargo.toml 설정
```
[dependencies]
chrono = "0.4"
chrono-tz = "0.8"
```
- 최신 버전은 crates.io에서 확인 가능.  
- 위 버전은 2025년 기준 안정적으로 사용되는 버전입니다.

## 📦 설치 방법 요약
- 프로젝트 루트에 있는 Cargo.toml 파일을 열고 위 내용을 추가하세요.
- 터미널에서 프로젝트 디렉터리로 이동한 뒤 cargo build 또는 cargo run을 실행하면 자동으로 설치됩니다.

## ✨ 사용 예시
```rust
use chrono::{Utc, NaiveDate, NaiveDateTime, Duration};
use chrono_tz::Asia::Seoul;

fn main() {
    let now = Utc::now();
    let seoul_time = now.with_timezone(&Seoul);
    println!("서울 기준 현재 시각: {}", seoul_time);
}
```

## 🕒 Rust에서 시간 처리 – chrono 크레이트 기반 요약
| 기능 설명                     | 예시 코드                                                                 |
|------------------------------|---------------------------------------------------------------------------|
| 날짜/시간 생성 | `let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);`              |
| 현재 시각 조회 | `let now = Local::now();`                                                |
| 날짜 더하기 | `let future = dt + Duration::days(10);`                                  |
| 날짜/시간 구성 요소 조회 | `dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second()`   |
| 반복 날짜 출력 | `for i in 0..5 { println!("{}", dt + Duration::weeks(i * 2)); }`         |
| 디데이 계산 | `let days = (target - today).num_days();`                                |
| 월의 시작/마지막 요일 | `dt.weekday()`, `dt.with_day(1).unwrap()`, `dt.with_day(dt.days_in_month())` |
| 시간대 변환 | `let utc = Utc::now(); let seoul = utc.with_timezone(&FixedOffset::east(9*3600));` |
| 포맷팅  | `dt.format("%Y-%m-%d %H:%M:%S").to_string()`                              |
| Instant 및 Epoch 처리 | `let instant = Utc::now(); let epoch = instant.timestamp();`            


### 🧪 간단 샘플 코드: 날짜 생성 및 더하기
```rust
use chrono::{NaiveDate, NaiveDateTime, Duration};
fn main() {
    let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(0, 0, 0);
    let future = dt + Duration::days(3) + Duration::hours(4);
    println!("기준 시각: {}", dt);
    println!("결과 시각: {}", future);
}
```

### 🌍 시간대 변환 예시
```rust
use chrono::{Utc, FixedOffset};
fn main() {
    let utc_now = Utc::now();
    let seoul = utc_now.with_timezone(&FixedOffset::east(9 * 3600));
    println!("UTC: {}", utc_now);
    println!("서울 시간: {}", seoul);
}
```

## 🕓 LocalDateTime
### 생성
| 메서드 이름 | 설명                           | Kotlin 예시 코드                                | Rust 대응 코드 예시                                         |
|-------------|--------------------------------|--------------------------------------------------|-------------------------------------------------------------|
| now()  | 현재 시스템의 날짜와 시간 생성 | `val now = LocalDateTime.now()` | `let now = chrono::Local::now().naive_local();`             |
| of(...) | 지정한 날짜와 시간으로 생성     | `val dt = LocalDateTime.of(2024, 1, 1, 9, 0)` | `let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, Local};
fn main() {
    // ✅ 현재 시스템의 날짜와 시간 생성 (Kotlin: LocalDateTime.now())
    let now = Local::now().naive_local();
    println!("현재 시각: {}", now);

    // ✅ 지정한 날짜와 시간으로 생성 (Kotlin: LocalDateTime.of(...))
    let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);
    println!("지정된 시각: {}", dt);
}
```

#### 🔍 실행 결과 예시
```
현재 시각: 2025-10-11 13:35:12
지정된 시각: 2024-01-01 09:00:00
```

### 변환
| 항목 구분   | 반환 타입   | Kotlin 예시 코드     | Rust 대응 코드 예시                                                                 |
|-------------|--------------|-------------------------------------|-------------------------------------------------|
| LocalDate   | 날짜 추출    | `dt.toLocalDate()` | `let date = dt.date();`                                                             |
| LocalTime   | 시간 추출    | `dt.toLocalTime()`     | `let time = dt.time();`                                                             |
| EpochSecond | UTC 기준 초  | `dt.toEpochSecond(ZoneOffset.of("+09:00"))` | `let epoch = dt.timestamp() - offset.local_minus_utc() as i64;`                     |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime, Duration, FixedOffset};

fn main() {
    // 기준 LocalDateTime 생성
    let dt: NaiveDateTime = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);

    // ✅ toLocalDate() – 날짜만 추출
    let date = dt.date();
    println!("LocalDate: {}", date); // 2024-01-01

    // ✅ toLocalTime() – 시간만 추출
    let time = dt.time();
    println!("LocalTime: {}", time); // 09:00:00

    // ✅ toEpochSecond(offset) – UTC 기준 초 단위 시간 반환
    let offset = FixedOffset::east(9 * 3600); // +09:00
    let epoch_second = dt.timestamp() - offset.local_minus_utc() as i64;
    println!("Epoch Second (+09:00 기준): {}", epoch_second); // 1704067200
}
```

#### 🔍 실행 결과 예시
```
LocalDate: 2024-01-01
LocalTime: 09:00:00
Epoch Second (+09:00 기준): 1704067200
```

### 조회
| 항목 구분     | 설명    | Kotlin 예시 코드    | Rust 대응 코드 예시                        |
|---------------|--------------------|--------------------------------------------|--------------------------------------------|
| 날짜 조회     | 연도, 월, 일 조회 | `dt.year`, `dt.month`, `dt.dayOfMonth` | `dt.year(), dt.month(), dt.day()`          |
| 시간 조회     | 시, 분, 초 조회  | `dt.hour`, `dt.minute`, `dt.second` | `dt.hour(), dt.minute(), dt.second()`      |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    // 기준 LocalDateTime 생성
    let dt: NaiveDateTime = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 15, 30);

    // ✅ 날짜 구성 요소 조회
    println!("연도: {}", dt.year());
    println!("월: {}", dt.month());
    println!("일: {}", dt.day());

    // ✅ 시간 구성 요소 조회
    println!("시: {}", dt.hour());
    println!("분: {}", dt.minute());
    println!("초: {}", dt.second());
}
```

#### 🔍 실행 결과 예시
```
연도: 2024
월: 1
일: 1
시: 9
분: 15
초: 30
```

### 비교
| 항목 구분 | 설명   | Kotlin 예시 코드  | Rust 대응 코드 예시 |
|-----------|--------------------|-------------------|----------------------|
| 이전 비교 | dt1이 dt2보다 이전 | `dt1.isBefore(dt2)` | `dt1 < dt2`  |
| 이후 비교 | dt1이 dt2보다 이후 | `dt1.isAfter(dt2)` | `dt1 > dt2` |
| 동일 비교 | dt1과 dt2가 동일   | `dt1.isEqual(dt2)` | `dt1 == dt2` |


#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let dt1 = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);
    let dt2 = NaiveDate::from_ymd(2024, 1, 2).and_hms(9, 0, 0);

    // ✅ isBefore()
    println!("dt1 < dt2: {}", dt1 < dt2); // true

    // ✅ isAfter()
    println!("dt1 > dt2: {}", dt1 > dt2); // false

    // ✅ isEqual()
    println!("dt1 == dt2: {}", dt1 == dt2); // false
}
```


#### 🔍 실행 결과 예시
```
dt1 < dt2: true
dt1 > dt2: false
dt1 == dt2: false
```

### 수정
| 항목 구분 | 설명                           | Kotlin 예시 코드                                      | Rust 대응 코드 예시                                                   |
|-----------|--------------------------------|------------------------------------------|------------------------------------------------------------------------|
| 일자 변경 | `ChronoField.DAY_OF_MONTH` 수정 | `dt.with(ChronoField.DAY_OF_MONTH, 15)` | `let updated = dt.with_day(15).unwrap();`                              |
| 연월 변경 | 연도 및 월 직접 지정하여 변경   | `dt.withYear(2025)` `dt.withMonth(12)`  | `let updated = NaiveDate::from_ymd(2025, 12, dt.day())`<br>`.and_hms(dt.hour(), dt.minute(), dt.second());` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);

    // ✅ with(...) – 일자 변경
    let changed_day = dt.with_day(15).unwrap();
    println!("일자 변경: {}", changed_day); // 2024-01-15T09:00:00

    // ✅ withYear() – 연도 변경
    let changed_year = NaiveDate::from_ymd(2025, dt.month(), dt.day()).and_hms(dt.hour(), dt.minute(), dt.second());
    println!("연도 변경: {}", changed_year); // 2025-01-01T09:00:00

    // ✅ withMonth() – 월 변경
    let changed_month = NaiveDate::from_ymd(dt.year(), 12, dt.day()).and_hms(dt.hour(), dt.minute(), dt.second());
    println!("월 변경: {}", changed_month); // 2024-12-01T09:00:00
}
```

#### 🔍 실행 결과 예시
```
일자 변경: 2024-01-15 09:00:00
연도 변경: 2025-01-01 09:00:00
월 변경: 2024-12-01 09:00:00
```


### 추가/감소
| 항목 구분     | 설명                           | Kotlin 예시 코드     | Rust 대응 코드 예시                                                                 |
|---------------|--------------------------------|------------------------------|--------------------------------------------------------------------------------------|
| 일수 더하기/빼기 | 지정된 시간 단위로 더하거나 빼기 | `dt.plus(3, ChronoUnit.DAYS)`     | `let added = dt + Duration::days(3);`<br>`let subtracted = dt - Duration::days(3);` |
| 연도/일수 더하기 | 연도 또는 일수를 더하기         | `dt.plusYears(1)` `dt.plusDays(10)` | `let added_year = NaiveDate::from_ymd(dt.year() + 1, dt.month(), dt.day())`<br>`.and_hms(dt.hour(), dt.minute(), dt.second());`<br>`let added_days = dt + Duration::days(10);` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime, Duration};

fn main() {
    let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);

    // ✅ plus(...) – 일수 더하기
    let added = dt + Duration::days(3);
    println!("3일 추가: {}", added); // 2024-01-04 09:00:00

    // ✅ minus(...) – 일수 빼기
    let subtracted = dt - Duration::days(3);
    println!("3일 감소: {}", subtracted); // 2023-12-29 09:00:00

    // ✅ plusYears()
    let added_year = NaiveDate::from_ymd(dt.year() + 1, dt.month(), dt.day()).and_hms(dt.hour(), dt.minute(), dt.second());
    println!("1년 추가: {}", added_year); // 2025-01-01 09:00:00

    // ✅ plusDays()
    let added_days = dt + Duration::days(10);
    println!("10일 추가: {}", added_days); // 2024-01-11 09:00:00
}
```

#### 🔍 실행 결과 예시
```
3일 추가: 2024-01-04 09:00:00
3일 감소: 2023-12-29 09:00:00
1년 추가: 2025-01-01 09:00:00
10일 추가: 2024-01-11 09:00:00
```


### 포맷팅
| 항목 구분     | 설명     | Kotlin 예시 코드         | Rust 대응 코드 예시       |
|---------------|--------------------------------|--------------------------------|----------------------------------------|
| 문자열 포맷팅 | 지정한 포맷 패턴으로 문자열 변환 | `dt.format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss"))` | `dt.format("%Y-%m-%d %H:%M:%S").to_string()`   |



#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let dt: NaiveDateTime = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 5, 30);

    // ✅ 포맷팅 – 지정한 패턴으로 문자열 변환
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("포맷된 문자열: {}", formatted); // 2024-01-01 09:05:30
}
```

### 🔍 실행 결과 예시
```
포맷된 문자열: 2024-01-01 09:05:30
```


## 🌍 ZonedDateTime

### 생성

| 항목 구분 | 설명                         | Kotlin 예시 코드                                     | Rust 대응 코드 예시                                                                               |
|-----------|---------------------------------------|-----------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| 현재 시각 | 현재 또는 지정된 시간대 기준 현재 시각 | `ZonedDateTime.now(ZoneId.of("Asia/Seoul"))` | `let now = Utc::now().with_timezone`<br>`(&FixedOffset::east(9 * 3600));`<br>`let now = chrono_tz::Asia::Seoul::now();` |
| 지정 생성 | 날짜, 시간, 시간대를 지정하여 생성     | `ZonedDateTime.of(LocalDate.of(2024,1,1), `<br>`LocalTime.of(9,0), `<br>`ZoneId.of("Asia/Seoul"))` | `let naive = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);`<br>`let zoned = chrono_tz::Asia::Seoul.from_local_datetime`<br>`(&naive).unwrap();` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime, Utc};
use chrono_tz::Asia::Seoul;

fn main() {
    // ✅ 현재 서울 시간 기준 ZonedDateTime 생성
    let now_seoul = Utc::now().with_timezone(&Seoul);
    println!("현재 서울 시각: {}", now_seoul);

    // ✅ 지정된 날짜, 시간, 시간대로 ZonedDateTime 생성
    let naive = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);
    let zoned = Seoul.from_local_datetime(&naive).unwrap();
    println!("지정된 서울 시간: {}", zoned);
}
```

### 🔍 실행 결과 예시
```
현재 서울 시간: 2025-10-11 13:42:00 +09:00
지정된 서울 시간: 2024-01-01 09:00:00 +09:00
```



### 타임존 관리

| 항목 구분             | 설명                                        | Kotlin 예시 코드                                              | Rust 대응 코드 예시                                                                 |
|-----------------------|---------------------------------------------------|---------------------------------------------------------------|------------------------------------------------------------------------------|
| 동일한 순간 변환      | UTC 기준 동일한 순간을 다른 시간대로 변환  | `zdt.withZoneSameInstant`<br>`(ZoneId.of("Europe/London"))` | `let converted = original.with_timezone`<br>`(&chrono_tz::Europe::London);`   |
| 동일한 로컬 시각 유지 | 로컬 시각 유지하며 시간대만 변경 (실제 시각은 달라짐) | `zdt.withZoneSameLocal`<br>`(ZoneId.of("America/New_York"))` | `let local = NaiveDate::from_ymd(2024, 1, 1)`<br>`.and_hms(9, 0, 0);`<br>`let changed = chrono_tz::America::New_York`<br>`.from_local_datetime(&local).unwrap();` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime, Utc};
use chrono_tz::{Europe::London, America::New_York, Asia::Seoul};

fn main() {
    // 기준 ZonedDateTime (서울 기준)
    let naive = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);
    let seoul_time = Seoul.from_local_datetime(&naive).unwrap();

    // ✅ withZoneSameInstant() – UTC 기준 동일한 순간을 런던 시간으로 변환
    let same_instant_london = seoul_time.with_timezone(&London);
    println!("동일한 순간 (런던 기준): {}", same_instant_london);

    // ✅ withZoneSameLocal() – 로컬 시각 유지하며 뉴욕 시간대로 변경
    let same_local_newyork = New_York.from_local_datetime(&naive).unwrap();
    println!("동일한 로컬 시각 (뉴욕 기준): {}", same_local_newyork);
}
```

#### 🔍 실행 결과 예시
```
동일한 순간 (런던 기준): 2024-01-01 00:00:00 +00:00
동일한 로컬 시각 (뉴욕 기준): 2024-01-01 09:00:00 -05:00
```


### 조회/변환

| 항목 구분         | 설명                                  | Kotlin 예시 코드      | Rust 대응 코드 예시   |
|-------------------|---------------------------------------|------------------------------|-------------------------------------------|
| 오프셋/시간대 조회 | 오프셋 및 시간대 정보 조회   | `zdt.offset` `zdt.zone`   | `zoned.offset()` `zoned.timezone()` `zoned.timezone().name()` |
| 로컬 시각 변환     | ZonedDateTime → LocalDateTime | `zdt.toLocalDateTime()` | `let local = zoned.naive_local();` |
| Instant 변환       | ZonedDateTime → Instant | `zdt.toInstant()` | `let instant = zoned.with_timezone(&Utc);` |
| Epoch 초 반환      | UTC 기준 초 단위 시간 반환 | `zdt.toEpochSecond()` | `let epoch = zoned.timestamp();` |



#### 🧪 Rust 실행 샘플 코드
```rust
use chrono::{NaiveDate, NaiveDateTime, Utc};
use chrono_tz::Asia::Seoul;

fn main() {
    // 기준 ZonedDateTime 생성
    let naive = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);
    let zoned = Seoul.from_local_datetime(&naive).unwrap();

    // ✅ getOffset(), getZone()
    println!("오프셋: {}", zoned.offset()); // +09:00
    println!("시간대: {}", zoned.timezone()); // Asia/Seoul

    // ✅ toLocalDateTime()
    let local = zoned.naive_local();
    println!("LocalDateTime: {}", local); // 2024-01-01 09:00:00

    // ✅ toInstant()
    let instant = zoned.with_timezone(&Utc);
    println!("Instant (UTC 기준): {}", instant); // 2024-01-01 00:00:00 +00:00

    // ✅ toEpochSecond()
    let epoch = zoned.timestamp();
    println!("Epoch Second: {}", epoch); // 1704067200
}
```

#### 🔍 실행 결과 예시
```
오프셋: +09:00
시간대: Asia/Seoul
LocalDateTime: 2024-01-01 09:00:00
Instant (UTC 기준): 2024-01-01 00:00:00 +00:00
Epoch Second: 1704067200
```

## ⏱️ Instant

### 생성

| 항목 구분     | 설명                                  | Kotlin 예시 코드                          | Rust 대응 코드 예시                                                                 |
|---------------|---------------------------------------|--------------------------------------------|--------------------------------------------------------------------------------------|
| 현재 시각     | 현재 UTC 기준의 시각 생성  | `val now = Instant.now()` | `let now = Utc::now();`                                                             |
| 에포크 기반   | 초 또는 밀리초 기준 Instant 생성 | `Instant.ofEpochSecond(1760156340)` | `let instant = Utc.timestamp(1760156340, 0);`<br>`let instant_ms = Utc.timestamp_millis(1760156340000);` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{Utc, TimeZone};

fn main() {
    // ✅ 현재 UTC 기준 Instant 생성
    let now = Utc::now();
    println!("현재 UTC 시각: {}", now);

    // ✅ 에포크 기준 초 단위 Instant 생성
    let instant = Utc.timestamp(1760156340, 0);
    println!("에포크 초 기반 Instant: {}", instant);

    // ✅ 에포크 기준 밀리초 단위 Instant 생성
    let instant_ms = Utc.timestamp_millis(1760156340000);
    println!("에포크 밀리초 기반 Instant: {}", instant_ms);
}
```

#### 🔍 실행 결과 예시
```
현재 UTC 시각: 2025-10-11 06:34:16.039146600 UTC
에포크 초 기반 Instant: 2025-10-11 04:19:00 UTC
에포크 밀리초 기반 Instant: 2025-10-11 04:19:00 UTC
```

### 변환
| 항목 구분         | 설명                                  | Kotlin 예시 코드             | Rust 대응 코드 예시                            |
|-------------------|---------------------------------------|--------------------------------------------------------------------------------------------------------------|
| Offset 변환       | 지정된 오프셋 기준 OffsetDateTime 생성 | `instant.atOffset(ZoneOffset.of("+09:00"))` | `let offset_dt = instant.with_timezone(&FixedOffset::east(9 * 3600));` |
| Zoned 변환        | 지정된 시간대 기준 ZonedDateTime 생성 | `instant.atZone(ZoneId.of("Asia/Seoul"))`  | `let zoned_dt = instant.with_timezone(&chrono_tz::Asia::Seoul);` |

#### 🧪 Rust 샘플 코드
```rust
use chrono::{Utc, FixedOffset};
use chrono_tz::Asia::Seoul;

fn main() {
    // ✅ Instant 생성
    let instant = Utc::now();

    // ✅ atOffset(offset) – OffsetDateTime 변환
    let offset_dt = instant.with_timezone(&FixedOffset::east(9 * 3600));
    println!("OffsetDateTime (+09:00): {}", offset_dt);

    // ✅ atZone(zone) – ZonedDateTime 변환
    let zoned_dt = instant.with_timezone(&Seoul);
    println!("ZonedDateTime (Asia/Seoul): {}", zoned_dt);
}
```

#### 🔍 실행 결과 예시
```
OffsetDateTime (+09:00): 2025-10-11 13:46:00 +09:00
ZonedDateTime (Asia/Seoul): 2025-10-11 13:46:00 +09:00
```


### 조회
| 항목 구분     | 설명                           | Kotlin 예시 코드         | Rust 대응 코드 예시                          |
|---------------|--------------------------------|----------------------------|----------------------------------------------|
| 초 단위 조회  | UTC 기준 초 단위 시간 반환     | `instant.epochSecond`     | `instant.timestamp()`                        |
| 나노초 조회   | 현재 초의 나노초 부분 반환     | `instant.nano`            | `instant.timestamp_subsec_nanos()`           |

#### 🧪 Rust 샘플 코드
```rust
use chrono::Utc;

fn main() {
    let instant = Utc::now();

    // ✅ UTC 기준 초 단위 시간 반환
    let epoch_second = instant.timestamp();
    println!("Epoch Second: {}", epoch_second);

    // ✅ 현재 초의 나노초 부분 반환
    let nano = instant.timestamp_subsec_nanos();
    println!("Nano of Second: {}", nano);
}
```

#### 🔍 실행 결과 예시
```
Epoch Second: 1760156820
Nano of Second: 123456789
```

---


