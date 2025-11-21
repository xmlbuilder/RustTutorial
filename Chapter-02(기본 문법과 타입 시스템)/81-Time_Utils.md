# Time Util
chrono/chrono-tz 기반 테스트 코드들을 함수별로 정리.  
각 테스트가 어떤 기능을 검증하는지, 어떤 역할을 하는지 설명.

## 📑 함수별 기능 정리
### 1. test_now
- 기능: 현재 UTC 시각을 가져와서 서울 시간대(Asia/Seoul)로 변환
- 포인트: Utc::now() → with_timezone(&Seoul)로 타임존 변환
```rust
#[test]
fn test_now() {
    let now = Utc::now();
    let seoul_time = now.with_timezone(&chrono_tz::Asia::Seoul);
    println!("{}", seoul_time);
}
```

### 2. test_time_add
- 기능: NaiveDateTime에 Duration을 더해 날짜/시간을 계산
- 포인트: + Duration::days(3) + Duration::hours(4) → 3일 4시간 뒤 시각 계산

```rust
#[test]
fn test_time_add() {
    let dt = NaiveDate::from_ymd_opt(2024, 1, 1)
        .unwrap()
        .and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap());
    let future = dt + Duration::days(3) + Duration::hours(4);
    println!("{}", dt);
    println!("{}", future);
}
```

### 3. test_time_zone
- 기능: UTC 시각을 FixedOffset(+9시간)으로 변환
- 포인트: with_timezone(&FixedOffset::east(9*3600)) → 서울 표준시와 같은 오프셋 적용
```rust
#[test]
#[allow(deprecated)]
fn test_time_zone() {
    let utc_now = Utc::now();
    let seoul = utc_now.with_timezone(&FixedOffset::east(9 * 3600));
    println!("{}", utc_now);
    println!("{}", seoul);
}
```

### 4. test_local_time
- 기능: 현재 로컬 시각(Local::now())을 NaiveDateTime으로 가져오기
- 포인트: naive_local()은 오프셋 없는 로컬 시각을 반환
```rust
#[test]
#[allow(deprecated)]
fn test_local_time() {
    let now = Local::now().naive_local();
    println!("{}", now);
    let dt = NaiveDate::from_ymd(2020, 1, 1).and_hms(12, 0, 0);
    println!("{}", dt);
}
```

#### 5. test_local_date_time
- 기능: 로컬 시각에서 날짜/시간 분리, epoch 계산, 연/월/일/시/분/초 출력
- 포인트:
- date.year(), date.month(), date.day() → 날짜 정보
- time.hour(), time.minute(), time.second() → 시간 정보
- timestamp() → epoch second 계산
```rust
#[test]
#[allow(deprecated)]
fn test_local_date_time() {
    let now = Local::now().naive_local();
    let date = now.date();
    let time = now.time();
    println!("{}", date);
    println!("{}", time);

    let offset = FixedOffset::east(9 * 3600);
    let epoch_second = now.timestamp() - offset.local_minus_utc() as i64;
    println!("{}", epoch_second);

    print!("{}", date.year());
    print!("{}", date.month());
    print!("{}", date.day());

    println!("{}", now.hour());
    println!("{}", now.minute());
    println!("{}", time.second());

    println!("{}", time.hour());
    print!("{}", time.minute());
    print!("{}", time.second());
}
```
### 6. time_compare
- 기능: 두 NaiveDateTime 비교
- 포인트: <, >, == 연산으로 시각 비교 (Java의 isBefore, isAfter, isEqual과 유사)
```rust
#[test]
#[allow(deprecated)]
fn time_compare() {
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

### 7. change_date_time
- 기능: 날짜/연도/월 변경
- 포인트:
- with_day(15) → 일자 변경
- 새 NaiveDate 생성으로 연도/월 변경
```rust
#[test]
#[allow(deprecated)]
fn change_date_time() {
    let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);

    // ✅ with(...) – 일자 변경
    let changed_day = dt.with_day(15).unwrap();
    println!("일자 변경: {}", changed_day); // 2024-01-15T09:00:00

    // ✅ withYear() – 연도 변경
    let changed_year = NaiveDate::from_ymd(2025, dt.month(), dt.day()).and_hms(
        dt.hour(),
        dt.minute(),
        dt.second(),
    );
    println!("연도 변경: {}", changed_year); // 2025-01-01T09:00:00

    // ✅ withMonth() – 월 변경
    let changed_month = NaiveDate::from_ymd(dt.year(), 12, dt.day()).and_hms(
        dt.hour(),
        dt.minute(),
        dt.second(),
    );
    println!("월 변경: {}", changed_month); // 2024-12-01T09:00:00
}
```

#### 8. add_subtract_date_time
- 기능: 날짜 더하기/빼기
- 포인트:
- + Duration::days(3) → 3일 추가
- - Duration::days(3) → 3일 감소
- 연도 변경은 새 NaiveDate 생성으로 처리
```rust
#[test]
#[allow(deprecated)]
fn add_subtract_date_time() {
    let dt = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);

    // ✅ plus(...) – 일수 더하기
    let added = dt + Duration::days(3);
    println!("3일 추가: {}", added); // 2024-01-04 09:00:00

    // ✅ minus(...) – 일수 빼기
    let subtracted = dt - Duration::days(3);
    println!("3일 감소: {}", subtracted); // 2023-12-29 09:00:00

    // ✅ plusYears()
    let added_year = NaiveDate::from_ymd(dt.year() + 1, dt.month(), dt.day()).and_hms(
        dt.hour(),
        dt.minute(),
        dt.second(),
    );
    println!("1년 추가: {}", added_year); // 2025-01-01 09:00:00

    // ✅ plusDays()
    let added_days = dt + Duration::days(10);
    println!("10일 추가: {}", added_days); // 2024-01-11 09:00:00
}
```

### 9. date_formatting
- 기능: NaiveDateTime을 문자열로 포맷팅
- 포인트: format("%Y-%m-%d %H:%M:%S") → "2024-01-01 09:05:30"
```rust
#[test]
#[allow(deprecated)]
fn date_formatting() {
    let dt: NaiveDateTime = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 5, 30);
    // ✅ 포맷팅 – 지정한 패턴으로 문자열 변환
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("포맷된 문자열: {}", formatted); // 2024-01-01 09:05:30
}
```

### 10. timezone_date
- 기능: 특정 타임존(Asia/Seoul) 기준 시각 생성
- 포인트:
- Utc::now().with_timezone(&Seoul) → 현재 서울 시각
- Seoul.from_local_datetime(&naive) → 지정된 날짜/시간을 서울 시간대에 맞게 생성
```rust
#[test]
#[allow(deprecated)]
fn timezone_date() {
    // ✅ 현재 서울 시간 기준 ZonedDateTime 생성
    let now_seoul = Utc::now().with_timezone(&Seoul);
    println!("현재 서울 시각: {}", now_seoul);

    // ✅ 지정된 날짜, 시간, 시간대로 ZonedDateTime 생성
    let naive = NaiveDate::from_ymd(2024, 1, 1).and_hms(9, 0, 0);
    let zoned = Seoul.from_local_datetime(&naive).unwrap();
    println!("지정된 서울 시간: {}", zoned);
}
```

### 11. manage_time_zone
- 기능: 타임존 변환
- 포인트:
- with_timezone(&London) → 동일 순간을 런던 시간으로 변환
- New_York.from_local_datetime(&naive) → 동일 로컬 시각을 뉴욕 시간대에 맞게 생성
```rust
#[test]
#[allow(deprecated)]
fn manage_time_zone() {
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

### 12. timezone_offset
- 기능: 타임존 정보 확인 및 변환
- 포인트:
- zoned.offset() → 오프셋(+09:00)
- zoned.timezone() → 시간대(Asia/Seoul)
- naive_local() → 로컬 시각
- with_timezone(&Utc) → UTC 기준 Instant
- timestamp() → Epoch Second
```rust
#[test]
#[allow(deprecated)]
fn timezone_offset() {
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

### 13. instant_create
- 기능: Instant 생성
- 포인트:
- Utc::now() → 현재 Instant
- Utc.timestamp(sec, nanos) → 에포크 초 기반 Instant
- Utc.timestamp_millis(ms) → 에포크 밀리초 기반 Instant

```rust
#[test]
#[allow(deprecated)]
fn instant_create() {
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

### 14. instant_convert
- 기능: Instant를 다른 시간대/오프셋으로 변환
- 포인트:
- with_timezone(&FixedOffset::east(9*3600)) → OffsetDateTime
- with_timezone(&Seoul) → ZonedDateTime
```rust
#[test]
#[allow(deprecated)]
fn instant_convert() {
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


### 15. instant_query
- 기능: Instant에서 epoch/nano 정보 추출
- 포인트:
- timestamp() → Epoch Second
- timestamp_subsec_nanos() → 나노초 부분
```rust
#[test]
#[allow(deprecated)]
fn instant_query() {
    let instant = Utc::now();

    // ✅ UTC 기준 초 단위 시간 반환
    let epoch_second = instant.timestamp();
    println!("Epoch Second: {}", epoch_second);

    // ✅ 현재 초의 나노초 부분 반환
    let nano = instant.timestamp_subsec_nanos();
    println!("Nano of Second: {}", nano);
}
```

## 🎯 요약
- 시간 생성: Utc::now(), Local::now(), NaiveDate::from_ymd
- 시간 연산: + Duration, - Duration, with_day, with_month, with_year
- 비교: <, >, ==
- 포맷팅: format("%Y-%m-%d %H:%M:%S")
- 타임존 관리: with_timezone, from_local_datetime, offset, timezone
- Instant 관리: timestamp, timestamp_millis, timestamp_subsec_nanos


# Chrono / Chrono-tz 테스트 함수 정리

| 함수명                | 주요 기능                                   | 포인트 설명                                                                 |
|-----------------------|---------------------------------------------|------------------------------------------------------------------------------|
| test_now              | 현재 UTC 시각을 서울 시간대로 변환           | `Utc::now()` → `with_timezone(&Seoul)`                                       |
| test_time_add         | 날짜/시간 더하기                            | `+ Duration::days(3) + Duration::hours(4)`                                   |
| test_time_zone        | UTC → FixedOffset 변환                      | `with_timezone(&FixedOffset::east(9*3600))`                                  |
| test_local_time       | 로컬 시각 가져오기                          | `Local::now().naive_local()`                                                 |
| test_local_date_time  | 날짜/시간 분리, epoch 계산                  | `date.year(), time.hour(), timestamp()`                                      |
| time_compare          | 두 NaiveDateTime 비교                       | `<`, `>`, `==` 연산                                                          |
| change_date_time      | 날짜/연도/월 변경                           | `with_day(15)`, 새 `NaiveDate` 생성                                          |
| add_subtract_date_time| 날짜 더하기/빼기                            | `+ Duration::days(3)`, `- Duration::days(3)`                                 |
| date_formatting       | 날짜/시간 포맷팅                            | `format("%Y-%m-%d %H:%M:%S")`                                                |
| timezone_date         | 특정 타임존 기준 시각 생성                   | `Utc::now().with_timezone(&Seoul)`, `Seoul.from_local_datetime(&naive)`      |
| manage_time_zone      | 타임존 변환                                 | `with_timezone(&London)`, `New_York.from_local_datetime(&naive)`             |
| timezone_offset       | 오프셋/타임존 확인, Instant 변환             | `offset()`, `timezone()`, `naive_local()`, `with_timezone(&Utc)`, `timestamp()` |
| instant_create        | Instant 생성                                | `Utc::now()`, `Utc.timestamp(sec, nanos)`, `Utc.timestamp_millis(ms)`        |
| instant_convert       | Instant → Offset/Zoned 변환                 | `with_timezone(&FixedOffset)`, `with_timezone(&Seoul)`                       |
| instant_query         | Instant 정보 추출                           | `timestamp()`, `timestamp_subsec_nanos()`                                    |

---

# Utility


## 🛠 코드 예시
```rust
use chrono::{DateTime, Local, Utc, Duration, NaiveDate};

pub struct TimeUtils;
```
```rust
impl TimeUtils {
    /// 1) 현재 시간을 "YYYY-MM-DD-HH-MM" 형식 문자열로 반환
    pub fn now_stamp() -> String {
        let now: DateTime<Local> = Local::now();
        now.format("%Y-%m-%d-%H-%M").to_string()
    }

    /// 2) 프로그램 시작/종료 시간 기록 및 사용 시간 계산
    pub fn program_usage(start: DateTime<Utc>, end: DateTime<Utc>) -> Duration {
        end - start
    }

    /// 3) 설치일과 오늘 날짜 비교 → 사용 기간 반환 (일 단위)
    pub fn license_usage(install_date: NaiveDate) -> Duration {
        let today = Local::now().naive_local().date();
        today - install_date
    }

    /// 4) 특정 함수 실행 시간 측정
    pub fn measure<F>(func: F) -> Duration
    where
        F: FnOnce(),
    {
        let start = Utc::now();
        func();
        let end = Utc::now();
        end - start
    }
}
```

## 🔍 사용 예시
```rust
#[cfg(test)]
mod tests {
    use chrono::{Utc, NaiveDate};
    use std::thread;
    use std::time::Duration as StdDuration;
    use nurbslib::core::time_utils::TimeUtils;

    #[test]
    fn test_now_stamp() {
        let stamp = TimeUtils::now_stamp();
        println!("현재 타임스탬프: {}", stamp);
        assert!(stamp.len() >= 16); // "YYYY-MM-DD-HH-MM" 형태
    }
```
```rust
    #[test]
    fn test_program_usage() {
        let start = Utc::now();
        thread::sleep(StdDuration::from_secs(2));
        let end = Utc::now();
        let usage = TimeUtils::program_usage(start, end);
        println!("프로그램 사용 시간: {} 초", usage.num_seconds());
        assert!(usage.num_seconds() >= 2);
    }
```
```rust
    #[test]
    fn test_license_usage() {
        let install_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let duration = TimeUtils::license_usage(install_date);
        println!("설치 후 사용 기간: {} 일", duration.num_days());
        assert!(duration.num_days() > 300); // 현재 시점 기준 300일 이상일 것
    }
```
```rust
    #[test]
    fn test_measure_function_time() {
        let elapsed = TimeUtils::measure(|| {
            thread::sleep(StdDuration::from_secs(1));
        });
        println!("함수 실행 시간: {} 초", elapsed.num_seconds());
        assert!(elapsed.num_seconds() >= 1);
    }
```
```rust
    #[test]
    fn total_test() {
        // 1) 문서 출력용 타임스탬프
        println!("문서 타임스탬프: {}", TimeUtils::now_stamp());

        // 2) 프로그램 시작/종료 시간
        let start = Utc::now();
        // ... 프로그램 실행 ...
        let end = Utc::now();
        let usage = TimeUtils::program_usage(start, end);
        println!("프로그램 사용 시간: {} 초", usage.num_seconds());

        // 3) 라이센스 체크 (설치일: 2024-01-01)
        let install_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let license_duration = TimeUtils::license_usage(install_date);
        println!("설치 후 사용 기간: {} 일", license_duration.num_days());

        // 4) 함수 실행 시간 측정
        let elapsed = TimeUtils::measure(|| {
            // 실행할 함수/코드 블록
            std::thread::sleep(std::time::Duration::from_secs(2));
        });
        println!("함수 실행 시간: {} 초", elapsed.num_seconds());
    }
}
```

## 🎯 요약
- now_stamp() → 현재 시간을 "YYYY-MM-DD-HH-MM" 문자열로 반환
- program_usage(start, end) → 프로그램 시작/종료 시간 차이를 Duration으로 반환
- license_usage(install_date) → 설치일과 오늘 날짜 차이를 계산해 라이센스 기간 체크
- measure(func) → 함수 실행 시간을 측정
---

