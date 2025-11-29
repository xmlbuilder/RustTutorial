## Radar 기반 실시간 차량 감지 설계
먼저, 이건 단순한 기술 설계가 아니라 목표를 명확히 하고 데이터 흐름을 안정적으로 만들기 위한 구조화 작업.  
**실시간** 과 **차량 감지** 라는 두 조건을 만족시키려면 데이터 수집–정렬–가공–학습–서빙–로그/DB 저장까지 한 번에 설계해야 합니다.

## 시스템 파이프라인 개요
- 센서 입력: 레이더 프레임(예: Range–Doppler–Angle 큐브, FFT/CFAR 결과)
- 시간 동기화: 타임스탬프 정렬, 드롭/보간 정책, IMU/캔버스 신호와의 동기
- 신호 처리: 노이즈 제거, 클러터 억제, CFAR/도플러 추출, 트래킹(칼만/JPDA, TBD)
- 특징 생성: 스펙트로그램 특징, 도플러/거리 피크, 클러스터 중심, 속도/가속도, SNR, 스캔 일관성
- AI 추론: 경량 모델로 **차량/비차량** 감지 + 박스/트랙 업데이트
- 결과 저장: 실시간 알림 + DB 기록(감지 이벤트, 트랙, 스냅샷, 품질 지표)

## 데이터베이스 스키마 설계
레이더 특성상 **프레임 데이터는 크고, 이벤트/트랙은 비교적 작고 잦게 발생** 합니다.  
저장 비용과 질의 성능을 고려해 원시 데이터와 요약 데이터를 분리.


### 📦 프레임 데이터는 크다
- 프레임 데이터란: 레이더가 한 번 스캔할 때 생성하는 전체 신호 맵 (예: Range–Doppler–Angle Cube)
- 이 데이터는 수천~수만 개의 포인트로 구성되어 있고, 고해상도 2D/3D 배열이기 때문에 용량이 큽니다
- 예: 한 프레임이 1MB~10MB 이상일 수도 있음
- 저장 방식: 보통은 DB에 직접 저장하지 않고, 파일로 저장하고 경로만 DB에 기록

### 🧠 이벤트/트랙은 작고 잦다
- 이벤트: 특정 시간에 차량이 감지되었거나, 이상 움직임이 발생한 것
- 트랙: 여러 프레임에 걸쳐 이어지는 하나의 차량 궤적 (예: 차량 A가 3초간 이동)
- 이들은 요약된 정보만 담고 있어서 데이터 크기가 작습니다
- 예: 차량의 거리, 속도, 각도, 신뢰도 등 몇 개의 숫자만 저장
- 그리고 프레임마다 여러 개의 이벤트/트랙이 발생하므로 빈도가 높습니다

### 🔄 왜 이렇게 나누는가?
- 프레임 데이터는 무겁고 느리게 저장 (샘플링, 조건부 저장)
- 이벤트/트랙은 가볍고 빠르게 저장 (실시간 DB 기록, 조회, 분석)
- 이 구조 덕분에 실시간성과 분석 효율성을 동시에 확보할 수 있어요

#### ✅ 요약
- **프레임 데이터는 크고, 이벤트/트랙은 비교적 작고 잦게 발생한다** 는 말은
- ➡️ 센서에서 나오는 원시 데이터는 무겁고, 의미 있는 요약 데이터는 작고 자주 생긴다는 뜻입니다.


### 핵심 테이블
- 레이더 프레임(Raw)
  - 필드: id, timestamp, sensor_id, frame_index, sampling_cfg, rd_map_uri, angle_map_uri, temperature, vehicle_speed
  - 설명: 대용량 RD/Angle 맵은 파일(객체 스토리지)로 저장하고 DB엔 메타만 기록
- 검출(Detections)
  - 필드: detection_id, frame_id, range_m, doppler_mps, angle_deg, snr_db, power, cluster_id
  - 설명: CFAR/클러스터링 결과의 단일 포인트들
- 트랙(Tracks)
  - 필드: track_id, first_seen_ts, last_seen_ts, status, current_range_m, current_speed_mps, angle_deg, track_confidence
  - 설명: 다중 프레임에 걸친 대상의 상태(칼만 필터 등)
- AI 결과(AI_inference)
  - 필드: inference_id, track_id, timestamp, class_label, class_prob, bbox_rao, model_version, latency_ms
  - 설명: 차량/비차량 등 분류, 확률, 모델 버전과 지연시간
- 라벨(Labeling)
  - 필드: label_id, track_id, annotator, timestamp, class_label, quality_note
  - 설명: 학습용 GT(인간/규칙/다중센서 융합) 라벨
- 운영 메트릭(Ops_metrics)
  - 필드: ts, sensor_id, dropped_frames, avg_latency_ms, cpu_gpu_util, memory_mb, temp_c
  - 설명: 실시간 성능과 안정성 지표
운영에서는 원시 프레임(대용량) 저장을 주기/조건(이상 상황, 샘플링율)으로 제한하고, 요약 테이블을 중심으로 조회하는 구조가 안정적입니다.


## AI 모델 설계
레이더는 영상과 달리 주파수·거리·각도·속도 성분이 핵심이라, **경량 + 시계열/공간 특화** 가 관건입니다.

**시계열/공간 특화** 라는 말은 AI 모델이나 데이터 처리 방식이 시간 흐름과 공간 구조를 잘 반영하도록 설계되었다는 뜻입니다.

### 🕒 시계열 특화란?
- 시간에 따라 변화하는 데이터를 다룰 수 있도록 설계된 모델
  - 예: 차량의 속도, 위치, 각도가 시간에 따라 어떻게 변하는지
- 사용하는 모델 예시:
  - RNN (Recurrent Neural Network)
  - LSTM, GRU
  - Temporal CNN
  - Transformer (시간 순서에 맞게 Attention 적용)
#### 활용 예
- 차량이 3초 동안 점점 빨라졌는지, 갑자기 멈췄는지
- 이상한 움직임(급가속, 급정지)을 시간 흐름으로 감지

#### 📍 공간 특화란?
- 공간 구조나 위치 관계를 잘 반영하는 모델
  - 예: 레이더에서 차량이 어디에 위치했는지, 주변에 뭐가 있었는지
- 사용하는 모델 예시:
  - CNN (Convolutional Neural Network)
  - PointNet, PointTransformer (3D 포인트 클라우드용)
  - Graph Neural Network (공간 관계를 그래프로 표현)
#### 활용 예
- 차량이 도로 중앙에 있는지, 차선 밖으로 벗어났는지
- 주변 물체와의 거리, 방향성 등을 고려한 판단

### ✅ 결론
- “시계열/공간 특화”란
- ➡️ 시간 흐름과 공간 구조를 동시에 고려해서 더 똑똑하게 판단하는 AI 설계 방식입니다.

- 입력 형태 선택
  - 특징 기반: CFAR 피크 집합 → 정규화된 벡터 특징(범위, 속도, SNR, 클러스터 크기, 프레임 지속성)
  - 맵 기반: Range–Doppler/Range–Angle 맵의 패치(2D CNN 가능)
  - 트랙 시계열: 트랙별 시간 시퀀스(속도/거리 변화, SNR 추이) → RNN/Temporal CNN/Transformer Lite
- 모델 후보
  - 경량 분류기: Logistic Regression, SVM, Gradient Boosting → 빠르고 실시간에 유리
  - 경량 CNN: 작은 커널/채널의 2D CNN → RD/RA 패치 입력
  - Temporal 모델: TCN/LSTM/GRU → 트랙 기반 시계열 패턴 반영
  - 하이브리드: 규칙(CFAR+트래킹)로 후보 생성 → AI로 최종 분류/확률 보정
- 경량화 전략
  - 정량화: 8-bit/16-bit 고정소수점
  - 프루닝/지식증류: 작은 학생 모델로 성능 유지
  - 입력 축소: 저해상도 패치, 핵심 피처만 선택
  - 지연 제어: 배치=1, 스트리밍 방식, 메모리 재사용

## 라벨링과 학습 데이터 구축
실시간 시스템의 성능을 좌우하는 건 데이터 품질. 라벨 설계가 가장 중요합니다.
- 라벨 소스
  - 다중 센서 융합: 카메라/라이다와 시공간 정합 → 차량 GT 생성
  - 인간 검수: 도심/고속/야간/우천 등 케이스별 샘플 검수
  - 규칙 보조: 속도/크기 범위로 초기 라벨 자동화 후 인간 교정
- 라벨 단위
  - 트랙 단위 라벨: track_id 전체를 차량/비차량으로 표기(프레임 변동에 강함)
  - 프레임 단위 보조: 특정 프레임 이벤트(급가속, 급감속, 클러터 근접) 태깅
- 데이터 분할
  - 환경별 셋: 도심/고속/우천/야간/혼잡/공사구간 등으로 분리해 일반화 확인
  - 시간 기반 분리: 누수 방지(같은 트랙이 train/test에 동시에 들어가지 않게)
- 품질 메트릭
  - 정확도/재현율/정밀도: 차량 미검/오검 균형 확인
  - 추론 지연/드롭율: 실시간성 유지
  - 트랙 일관성: 프레임별 출렁임을 트랙 확률로 매끈하게 유지

## 실시간 서빙과 운영
- 추론 파이프라인
  - 버퍼링: 센서→DSP→추론 입력 큐(지연 한도 설정)
  - 후처리: NMS/트랙 갱신, 확률 스무딩(엑스포넌셜/베이지안)
  - 알림/로깅: 신뢰도 임계치로 이벤트 발생, DB 기록
- 성능 관리
  - 지연 목표: 예, 30–50 ms/frame 수준 목표치 설정
  - 폴백 경로: AI 실패 시 규칙 기반 결과 전달
  - 모델 롤링: A/B 테스트, 모델 버전/해시 DB 기록
- 안전/윤리
  - 프라이버시: 개인 식별 데이터 저장 금지(레이더는 비교적 유리)
  - 오검 방지: 신뢰도 낮을 때 경고 레벨로 분류, 휴먼-인-더-루프 운영

바로 적용할 체크리스트
- 센서: 타임스탬프 표준화(UTC/PPST), 드롭·보간 정책 문서화
  - 신호 처리: CFAR 파라미터/윈도우, 트래킹 필터의 상태 정의
  - 특징 세트: 최소 특징 10–20개 선정(SNR, cluster size, velocity, persistence 등)
- 모델: 경량 분류기부터 시작 → 필요 시 CNN/Temporal로 확장
- DB: 원시 프레임은 객체 스토리지, DB엔 메타/요약/결과/라벨 저장
- 운영: 지연/드롭/온도/자원 메트릭 상시 수집, 모델 버전 관리

## Radar 차량 감지 파이프라인 Rust 예제
임베디드 장비에서 감지 → Linux 서버로 전송(TCP 또는 HTTP) → 서버가 MySQL에 저장 → 브라우저에서 조회.  
아래 예제는 바로 실행 가능한 형태로, 최소한의 구성만 담았습니다.

### 데이터 스키마와 Rust 구조체
서버와 DB가 공유하는 데이터 계약입니다. JSON으로 직렬화해 전송합니다.
```
// Cargo.toml (공통)
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
```
```rust
// 감지(Detections)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Detection {
    pub detection_id: String,
    pub frame_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub sensor_id: String,
    pub range_m: f32,
    pub doppler_mps: f32,
    pub angle_deg: f32,
    pub snr_db: f32,
    pub cluster_id: Option<String>,
}
```
```rust
// 트랙(Tracks)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub track_id: String,
    pub first_seen_ts: chrono::DateTime<chrono::Utc>,
    pub last_seen_ts: chrono::DateTime<chrono::Utc>,
    pub status: String, // "active", "lost" 등
    pub current_range_m: f32,
    pub current_speed_mps: f32,
    pub angle_deg: f32,
    pub track_confidence: f32,
}
```
```rust
// AI 결과(AI_inference)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AiInference {
    pub inference_id: String,
    pub track_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub class_label: String,    // "vehicle", "non-vehicle" 등
    pub class_prob: f32,        // 0.0~1.0
    pub model_version: String,  // "radar-lite-v1"
    pub latency_ms: u32,
}
```
```rust
// 운영 메트릭(Ops_metrics)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpsMetric {
    pub ts: chrono::DateTime<chrono::Utc>,
    pub sensor_id: String,
    pub dropped_frames: u32,
    pub avg_latency_ms: f32,
    pub cpu_util: f32,
    pub memory_mb: f32,
    pub temp_c: f32,
}
```

## MySQL 테이블 DDL
### 운영에서 원시 RD/Angle 맵은 객체 스토리지(S3 등)에 두고, 메타만 저장합니다. 우선 핵심 테이블 4개만.
```sql
CREATE TABLE detections (
  detection_id VARCHAR(64) PRIMARY KEY,
  frame_id     VARCHAR(64) NOT NULL,
  timestamp    DATETIME(6) NOT NULL,
  sensor_id    VARCHAR(64) NOT NULL,
  range_m      FLOAT NOT NULL,
  doppler_mps  FLOAT NOT NULL,
  angle_deg    FLOAT NOT NULL,
  snr_db       FLOAT NOT NULL,
  cluster_id   VARCHAR(64) NULL,
  INDEX idx_frame_ts (frame_id, timestamp),
  INDEX idx_sensor_ts (sensor_id, timestamp)
);
```
```sql
CREATE TABLE tracks (
  track_id          VARCHAR(64) PRIMARY KEY,
  first_seen_ts     DATETIME(6) NOT NULL,
  last_seen_ts      DATETIME(6) NOT NULL,
  status            VARCHAR(16) NOT NULL,
  current_range_m   FLOAT NOT NULL,
  current_speed_mps FLOAT NOT NULL,
  angle_deg         FLOAT NOT NULL,
  track_confidence  FLOAT NOT NULL,
  INDEX idx_last_seen (last_seen_ts)
);
```
```sql
CREATE TABLE ai_inference (
  inference_id  VARCHAR(64) PRIMARY KEY,
  track_id      VARCHAR(64) NOT NULL,
  timestamp     DATETIME(6) NOT NULL,
  class_label   VARCHAR(32) NOT NULL,
  class_prob    FLOAT NOT NULL,
  model_version VARCHAR(64) NOT NULL,
  latency_ms    INT NOT NULL,
  INDEX idx_track_ts (track_id, timestamp),
  FOREIGN KEY (track_id) REFERENCES tracks(track_id)
);
```
```sql
CREATE TABLE ops_metrics (
  id            BIGINT AUTO_INCREMENT PRIMARY KEY,
  ts            DATETIME(6) NOT NULL,
  sensor_id     VARCHAR(64) NOT NULL,
  dropped_frames INT NOT NULL,
  avg_latency_ms FLOAT NOT NULL,
  cpu_util       FLOAT NOT NULL,
  memory_mb      FLOAT NOT NULL,
  temp_c         FLOAT NOT NULL,
  INDEX idx_sensor_ts (sensor_id, ts)
);
```


### 임베디드 장비: TCP로 실시간 전송
리소스가 제한된 환경에서는 단순 TCP + JSON이 안정적입니다. (보다 효율적이라면 CBOR/MessagePack을 쓰면 됩니다.)
```
// Cargo.toml (임베디드 송신)
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use std::io::Write;
use std::net::TcpStream;

fn send_detection_tcp(server_addr: &str, det: &Detection) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(server_addr)?;
    // 프레이밍: length-prefix로 메시지 경계 보장
    let payload = serde_json::to_vec(det).unwrap();
    let len = (payload.len() as u32).to_be_bytes();
    stream.write_all(&len)?;
    stream.write_all(&payload)?;
    Ok(())
}
```

- 프레이밍: 길이(4바이트 big-endian) + JSON 바디. 서버에서 정확히 메시지를 분할 가능.
- 전송 정책: 배치 전송(예: 10개 누적) 또는 트랙 업데이트에 맞춰 이벤트 전송.

Linux 서버: TCP 수신 + MySQL 저장  
tokio로 비동기 TCP 서버를 만들고, sqlx로 MySQL에 저장합니다.
```
// Cargo.toml (서버)
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["mysql", "runtime-tokio-rustls", "chrono"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
````
```rust
use anyhow::Result;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://user:pass@127.0.0.1:3306/radar_db")
        .await?;

    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("TCP server listening on 9000");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let pool = pool.clone();

        tokio::spawn(async move {
            // length-prefixed loop
            loop {
                let mut len_buf = [0u8; 4];
                if socket.read_exact(&mut len_buf).await.is_err() {
                    break;
                }
                let len = u32::from_be_bytes(len_buf) as usize;

                let mut payload = vec![0u8; len];
                if socket.read_exact(&mut payload).await.is_err() {
                    break;
                }

                // JSON → Detection
                match serde_json::from_slice::<Detection>(&payload) {
                    Ok(det) => {
                        if let Err(e) = save_detection(&pool, &det).await {
                            eprintln!("DB save error: {e}");
                        }
                    }
                    Err(e) => eprintln!("JSON parse error: {e}"),
                }
            }
        });
    }
}
```
```rust
async fn save_detection(pool: &sqlx::MySqlPool, det: &Detection) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO detections
        (detection_id, frame_id, timestamp, sensor_id, range_m, doppler_mps, angle_deg, snr_db, cluster_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
          frame_id=VALUES(frame_id),
          timestamp=VALUES(timestamp),
          sensor_id=VALUES(sensor_id),
          range_m=VALUES(range_m),
          doppler_mps=VALUES(doppler_mps),
          angle_deg=VALUES(angle_deg),
          snr_db=VALUES(snr_db),
          cluster_id=VALUES(cluster_id)
        "#,
        det.detection_id,
        det.frame_id,
        det.timestamp,     // sqlx는 chrono::DateTime<Utc> 지원
        det.sensor_id,
        det.range_m,
        det.doppler_mps,
        det.angle_deg,
        det.snr_db,
        det.cluster_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
```

- 포인트: TCP 서버는 length-prefix로 안정적 수신, 파싱 실패/DB 실패는 로깅만 하고 루프 유지.
- 확장: 같은 방식으로 Track, AiInference 등도 엔드포인트 추가 가능.

## 서버: HTTP/웹 브라우저용 API
브라우저 연동이 필요하면, axum으로 간단한 REST API를 추가합니다. 임베디드가 HTTP로도 보낼 수 있고, 브라우저는 조회만 해도 됩니다.
```
// Cargo.toml (HTTP)
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["mysql", "runtime-tokio-rustls", "chrono"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
```
```rust
use axum::{routing::{get, post}, Router, Json, extract::State};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    pool: sqlx::MySqlPool,
}
```
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://user:pass@127.0.0.1:3306/radar_db")
        .await?;

    let state = AppState { pool };

    let app = Router::new()
        .route("/api/detections", post(post_detection).get(list_detections))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("HTTP server on {addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
```
```rust
async fn post_detection(
    State(state): State<AppState>,
    Json(det): Json<Detection>
) -> Result<Json<&'static str>, (axum::http::StatusCode, String)> {
    save_detection(&state.pool, &det).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json("ok"))
}
```
```rust
async fn list_detections(
    State(state): State<AppState>
) -> Result<Json<Vec<Detection>>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query!(
        r#"SELECT detection_id, frame_id, timestamp as "timestamp: chrono::DateTime<chrono::Utc>",
           sensor_id, range_m, doppler_mps, angle_deg, snr_db, cluster_id
           FROM detections ORDER BY timestamp DESC LIMIT 100"#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let list = rows.into_iter().map(|r| Detection {
        detection_id: r.detection_id,
        frame_id: r.frame_id,
        timestamp: r.timestamp,
        sensor_id: r.sensor_id,
        range_m: r.range_m,
        doppler_mps: r.doppler_mps,
        angle_deg: r.angle_deg,
        snr_db: r.snr_db,
        cluster_id: r.cluster_id,
    }).collect();

    Ok(Json(list))
}
```
```rust

- 브라우저: /api/detections를 GET으로 조회. 임베디드가 HTTP POST로 바로 전송도 가능.
- 선택지: 실시간은 TCP, 운영/관제는 HTTP로 병행하면 좋습니다.

## 운영 팁과 신뢰성 보강
- 재전송/버퍼링: 임베디드가 네트워크 불안정 시 큐에 쌓고 재시도. 메시지에 detection_id로 멱등성 확보.
- 압축/경량화: JSON 대신 MessagePack/CBOR, 또는 snappy/zstd 압축.
- 보안: 사내망이라도 TLS 적용(HTTP/2, Rustls), API 키/토큰.
- 스키마 버전: 구조체에 schema_version 필드를 두고 서버에서 호환 처리.
- 원시 데이터: 대용량 RD/RA 맵은 파일 스토리지에 저장하고 MySQL엔 경로/해시만.

## 결론
- 이렇게 해도 충분히 됩니다. 시작은 단순 TCP + JSON + MySQL로, 운영에 맞춰 HTTP API와 시각화 페이지를 추가.
- 임베디드→서버 경로는 “TCP(실시간) 또는 HTTP(운영/관제)”, DB는 MySQL로 요약데이터 중심 저장이 현실적입니다.


## 📌 서버에서 쌓을 수 있는 학습 데이터 종류
### 1. 원시 감지 데이터 (Detections)
- 거리(range), 속도(doppler), 각도(angle), SNR 같은 레이더 기본 피처
- 프레임별로 수십~수백 개의 포인트가 쌓임
- 활용: 클러스터링, 물체 후보 생성, 노이즈 패턴 학습
### 2. 트랙 데이터 (Tracks)
- 여러 프레임에 걸쳐 이어진 객체의 궤적
- 속도 변화, 거리 변화, 방향성, 지속 시간
- 활용: 차량 vs 비차량 구분, 행동 패턴 학습
### 3. AI 추론 결과 (Inference)
- 모델이 예측한 클래스(label)과 확률(probability)
- 모델 버전, 추론 지연(latency)까지 기록
- 활용: 모델 성능 모니터링, 재학습용 피드백 루프
### 4. 라벨 데이터 (Labels)
- 사람이 직접 붙인 정답(차량/비차량, 차량 종류 등)
- 다른 센서(카메라, LiDAR)와 융합해서 만든 Ground Truth
- 활용: 지도 학습용 학습 데이터셋
### 5. 운영 메트릭 (Ops Metrics)
- dropped frames, latency, CPU/GPU 사용률, 온도
- 활용: 실시간성 보장, 모델 최적화 연구

## 🧠 학습 데이터셋으로 만드는 방법
- Raw → Feature Engineering
  - 거리, 속도, 각도, SNR → 벡터화
  - 트랙별 시계열 특징 추출 (속도 변화율, 지속 시간 등)
- Feature + Label 결합
  - DB에서 라벨 테이블과 조인
  - “이 트랙은 차량이다” 같은 Ground Truth 생성
- Dataset Export
  - MySQL → CSV/Parquet로 추출
  - 학습 파이프라인(Python, Rust ML, PyTorch 등)으로 전달
- 모델 학습
  - 지도 학습: 차량/비차량 분류
  - 비지도 학습: 패턴 클러스터링 (예: 차량 종류별 움직임)

## ✅ 결론
서버는 단순 저장소가 아니라 AI 학습 데이터셋을 자동으로 쌓는 허브가 됩니다.
- Detections → 원시 피처
- Tracks → 시계열 패턴
- Labels → Ground Truth
- Inference → 모델 성능 로그

이렇게 쌓인 데이터를 기반으로, 차량 감지 정확도를 점점 높이는 재학습 루프를 만들 수 있음.  
서버에 쌓인 레이더 감지 데이터와 트랙 데이터를 잘 활용하면 시간별 이동량, 차량 각도, 차량 종류, 평균 속도 같은 고급 학습을 할 수 있습니다.  

## 📌 어떤 학습이 가능한가?
### 1. 시간별 이동량 분석
- 트랙 데이터에 first_seen_ts, last_seen_ts가 있으므로
→ 특정 시간대(예: 1분, 1시간 단위)마다 차량 수를 집계 가능
- 활용: 교통량 패턴 분석, 러시아워 탐지
### 2. 차량 각도 추정
- 레이더는 angle_deg를 제공
- 트랙별로 각도 변화를 기록하면 차량의 진행 방향 추정 가능
- 활용: 교차로에서 좌회전/우회전 패턴 분석
### 3. 차량 종류 분류
- AI 모델을 학습시켜 차량 종류(승용차, 트럭, 버스 등) 분류 가능
- 입력: 레이더 특징(크기, 반사 강도, 속도 패턴) + 라벨 데이터
- 활용: 교통 통계, 특정 차량군 모니터링
### 4. 평균 속도 계산
- 트랙별 current_speed_mps를 시간에 따라 평균화
- 특정 구간/시간대별 평균 속도 산출 가능
- 활용: 도로 혼잡도 평가, 위험 운행 탐지

## 🧠 학습 데이터셋 구성 예시
- Features (입력)
  - Range, Doppler, Angle, SNR
  - 트랙 지속 시간, 속도 변화율, 각도 변화율
  - 클러스터 크기, 반사 강도
- Labels (정답)
  - 차량 여부 (차량/비차량)
  - 차량 종류 (승용차/트럭/버스 등)
  - 이동 방향 (좌회전/우회전/직진)
  - Derived Metrics
  - 시간별 차량 수
  - 평균 속도
- 각도 분포

## ✅ 결론
서버에 쌓인 데이터로 충분히 교통량, 차량 종류, 평균 속도, 이동 방향까지 학습할 수 있습니다.
즉, 단순 “차량 감지”를 넘어서 교통 분석 AI로 확장할 수 있는 것임.

---


서버에 쌓인 레이더 기반 데이터와 AI 모델을 잘 설계하면 새로운 차종 탐지와 비정상적인 차량 움직임 감지까지 확장할 수 있음.

## 🚗 새로운 차종 탐지
- 방법 1: 지도 학습
  - 라벨링된 데이터(승용차, 트럭, 버스 등)를 학습 → 차량 종류 분류 모델 구축
  - 새로운 차종이 들어오면, 기존 클래스와 다른 패턴을 보이는지 확인 가능
- 방법 2: 비지도 학습 (클러스터링)
  - Range–Doppler–Angle 특징을 기반으로 클러스터링
  - 기존 클래스에 속하지 않는 새로운 그룹이 생기면 “새로운 차종 후보”로 탐지
- 방법 3: 전이 학습 (Transfer Learning)
  - 기존 차량 분류 모델에 새로운 라벨을 추가해 재학습 → 빠르게 확장 가능

## 🛑 비정상적인 차량 움직임 감지
- 속도 패턴 분석
  - 평균 속도 대비 급격한 가속/감속 → 위험 운행 탐지
- 궤적 분석
  - 각도 변화율이 비정상적으로 크거나, 지그재그 움직임 → 음주/위험 운전 가능성
- 시간/장소 기반 이상 탐지
  - 특정 구간에서 허용 속도 초과, 역주행 패턴 → 이상 행동으로 분류
- AI 기반 이상 탐지 모델
  - 정상 차량 움직임 데이터를 학습 → 새로운 입력이 정상 분포에서 벗어나면 “이상”으로 판단 (Anomaly Detection)

## 🧠 학습 데이터 활용
- 새로운 차종: DB에 저장된 트랙 + 라벨 데이터를 기반으로 차량 종류별 특징 벡터를 학습
- 비정상 움직임: 트랙 시계열 데이터를 기반으로 정상/비정상 패턴을 분류하는 모델 학습
- 결과: 교통량 분석뿐 아니라 교통 안전 모니터링 시스템으로 확장 가능

## ✅ 결론
- 서버에 쌓인 레이더 데이터는 단순히 “차량 감지”를 넘어서
- ➡️ 새로운 차종 탐지와 비정상적인 차량 움직임 감지까지 가능합니다.
- 즉, 교통 분석 + 안전 모니터링을 동시에 할 수 있는 AI 플랫폼으로 발전시킬 수 있음.

## 📌 서버에서 학습시키는 단계
### 1. 데이터 수집 & 저장
- 임베디드 장비 → 서버 → MySQL DB에 저장
- 테이블: detections, tracks, ai_inference, labels
- 원시 데이터는 파일 스토리지(S3, NAS)에 두고, DB에는 메타데이터만 기록
### 2. 데이터 추출 (ETL)
- Python/Rust에서 DB 연결 → SQL로 필요한 데이터 추출
- 예: 특정 시간대 차량 트랙, 라벨이 붙은 차량 종류 데이터
- 추출 후 CSV/Parquet 같은 학습 친화적 포맷으로 변환

```python
import pandas as pd
import sqlalchemy

engine = sqlalchemy.create_engine("mysql+pymysql://user:pass@localhost/radar_db")
df = pd.read_sql("SELECT * FROM tracks JOIN labels USING(track_id)", engine)
```      

### 3. 전처리 & 특징 추출
- Range, Doppler, Angle, SNR → 벡터화
- 트랙별 평균 속도, 각도 변화율, 지속 시간 계산
- 라벨과 결합해 supervised dataset 생성
### 4. 모델 학습
- 분류 모델: 차량 vs 비차량, 차량 종류
- 회귀 모델: 평균 속도 예측, 이동 거리 추정
- 이상 탐지 모델: 비정상 궤적 탐지
- 프레임워크: PyTorch, TensorFlow, scikit-learn, linfa(Rust)

```python
from sklearn.ensemble import RandomForestClassifier

X = df[["range_m","doppler_mps","angle_deg","snr_db","avg_speed","angle_change"]]
y = df["class_label"]

model = RandomForestClassifier()
model.fit(X, y)
```

### 5. 모델 저장 & 배포
- 학습된 모델을 파일로 저장 (.pkl, .onnx)
- 서버에 올려서 실시간 추론 API로 제공
- 추론 결과는 다시 DB에 기록 → 재학습 데이터로 활용
### 6. 재학습 루프
- 주기적으로 새로운 데이터 + 라벨을 반영해 모델 업데이트
- 모델 버전 관리 (예: radar-lite-v1, radar-lite-v2)
- 성능 모니터링: 정확도, 재현율, 지연시간

## ✅ 결론
서버는 단순 저장소가 아니라 데이터 허브 + 학습 파이프라인 역할을 합니다.
- DB에서 데이터를 꺼내 전처리 → 학습 → 모델 저장 → 추론 API → 결과 다시 DB에 기록
- 이렇게 하면 자동 재학습 루프가 만들어져서, 시간이 지날수록 차량 감지/분류/이상 탐지 정확도가 올라갑니다.

## **서버 학습 파이프라인 흐름도**
![서버 학습 파이프라인 흐름도](/image/서버_학습_파이프라인.png)


---


