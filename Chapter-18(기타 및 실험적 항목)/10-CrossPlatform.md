# 크로스 플랫폼과 GUI 프로그래밍
Rust로 크로스 플랫폼과 GUI 프로그래밍을 다루는 방법을 핵심만 정리.  
실제 예제와 툴체인, GUI 라이브러리까지 모두 포함된 아주 실용적인 자료.

## 🧭 1. 크로스 플랫폼 개발: Rust의 강점
Rust는 단일 코드베이스로 다양한 플랫폼에 배포할 수 있는 구조를 갖추고 있어  
크로스 플랫폼 개발에 매우 적합합니다.

### ✅ 주요 타겟 플랫폼 (Tier1 기준)

| 타겟 명칭                   | 설명                                 |
|----------------------------|--------------------------------------|
| x86_64-unknown-linux-gnu   | 64비트 Linux (glibc 기반, 가장 널리 사용됨) |
| x86_64-pc-windows-msvc     | 64비트 Windows (MSVC 툴체인 기반)         |
| x86_64-apple-darwin        | 64비트 macOS (Darwin 커널 기반)         |
| aarch64-unknown-linux-gnu  | ARM64 Linux (서버, 라즈베리파이 등)     |


## 🛠️ 툴체인 설정 예시
```rust
rustup target add aarch64-unknown-linux-gnu
```

- .cargo/config에 링커 설정 추가
- `cargo build --target=aarch64-unknown-linux-gnu` 로 빌드
- file 명령어로 바이너리 확인 후 라즈베리파이 등에 배포

## 🤖 2. Android NDK + Rust
Rust는 Android NDK와 연동하여 네이티브 라이브러리 개발이 가능합니다.
### 🔧 개발 환경 구성
- Android Studio 설치 → SDK Manager에서 NDK, CMake, Command-line Tools 설치
- Rust에 Android 타겟 등록: aarch64-linux-android, armv7-linux-androideabi 등
- .cargo/config에 Android용 링커 설정
  
### 🧬 JNI 연동 예시
Rust 함수명은 JNI 명명 규칙을 따라야 합니다:
```
#[no_mangle]
pub extern "C" fn Java_com_example_MainActivity_helloRust(...) { ... }
```
- jni 크레이트 사용
- JNIEnv를 통해 Java ↔ Rust 자료형 변환
- 빌드된 .so 파일을 jniLibs/ 경로에 복사하여 Android 앱에서 호출

### ⚡ 3. 성능 비교: Rust vs Java
| 벤치마크 항목       | Java 실행 시간 (초) | Rust 실행 시간 (초) | 비고                         |
|---------------------|---------------------|----------------------|------------------------------|
| fannkuch-redux      | 40.3                | 3.51                 | 순열 계산, 캐시 민감         |
| n-body              | 6.77                | 2.81                 | 물리 시뮬레이션              |
| spectral-norm       | 1.54                | 0.71                

- Rust는 메모리 안전성과 성능에서 Java를 압도합니다.
- 특히 CPU 집약적인 작업에서 차이가 큽니다.
- Rust는 GC 없이도 안전한 메모리 관리를 제공
- NDK를 통해 Java의 병목을 Rust로 대체 가능

## 🖼️ 4. GUI 프로그래밍: Rust의 선택지
Rust는 GUI에 특화된 언어는 아니지만, 서드파티 라이브러리를 통해 충분히 구현 가능합니다.

### 🧊 ICED
- 선언적 GUI 프레임워크 (React 스타일)
- 데스크톱 + 크로스 플랫폼 지원
- iced::Application 기반 구조

### ⚡ egui
- 즉시 모드 GUI (Immediate Mode)
- 웹 + 데스크톱 지원 (wasm도 가능)
- 간결한 API, 빠른 반응성

### 🧱 gtk-rs
- GTK+ 바인딩
- 안정적이고 레퍼런스 많은 GUI 툴킷
- Linux 데스크톱에 강력한 지원

### 🧩 요약: Rust로 크로스 플랫폼 & GUI 프로그래밍

| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 크로스 플랫폼 개발 | Rust 툴체인으로 다양한 OS/기기 타겟 빌드 가능 (`x86_64`, `aarch64`, `wasm` 등) |
| Android NDK 연동   | JNI 기반으로 Rust 네이티브 함수 호출 가능, 고성능 모듈에 적합               |
| 성능 비교          | Java 대비 대부분 벤치마크에서 Rust가 우수한 성능 발휘                        |
| GUI 라이브러리     | `iced`, `egui`, `gtk-rs` 등으로 데스크톱/웹 UI 구현 가능                     |
| 실전 적용 전략     | 성능이 중요한 영역에 Rust 적용, UI는 서드파티 툴킷으로 보완                   |

---


# Android Native Development Kit(NDK)
Rust의 성능과 안전성을 Android Native Development Kit(NDK)와 결합하여
고성능 네이티브 모듈을 설계하는 전략을 단계별로 설명.

## 🧠 1. 왜 Rust인가?
- 성능: Java보다 빠른 실행 속도 (벤치마크에서 최대 10배 이상 차이)
- 메모리 안전성: 컴파일 타임에 메모리 오류 방지
- NDK 친화성: C/C++처럼 NDK에 통합 가능, no_std 환경도 지원
- Google도 채택: Android Framework 일부가 Rust로 작성됨

## 🛠️ 2. 개발 환경 구성
### ✅ 필수 도구
- Android Studio + NDK + CMake + Command-line Tools
- Rust 툴체인: rustup target add aarch64-linux-android 등
- .cargo/config.toml에 Android용 링커 설정
```
[target.aarch64-linux-android]
ar = "path/to/llvm-ar"
linker = "path/to/aarch64-linux-android-clang"
```


## 🧬 3. Rust 라이브러리 생성
### ✅ JNI 연동을 위한 함수 정의
Rust 함수는 JNI 명명 규칙을 따라야 합니다:
```
#[no_mangle]
pub extern "C" fn Java_com_example_MainActivity_helloRust(
    env: JNIEnv,
    _: JClass,
    input: JString,
) {
    let message: String = env.get_string(&input).unwrap().into();
    println!("Hello from Rust: {}", message);
}
```

- jni 크레이트 사용
- Java ↔ Rust 자료형 변환은 JNIEnv의 메서드로 처리

## 📦 4. 빌드 및 배포
### ✅ 빌드 명령
cargo build --target aarch64-linux-android --release


- .so 파일 생성됨 → Android 프로젝트의 jniLibs/arm64-v8a/에 복사

### ✅ 배포 경로 예시

| 타겟 아키텍처       | Android 배포 경로                        |
|---------------------|------------------------------------------|
| aarch64 (arm64)     | `app/src/main/jniLibs/arm64-v8a`         |
| armv7               | `app/src/main/jniLibs/armeabi-v7a`       |
| i686 (x86)          | `app/src/main/jniLibs/x86`               |
| x86_64              | `app/src/main/jniLibs/x86_64`            |


### ⚙️ 5. 성능 최적화 전략

| 전략                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| `#[inline(always)]`   | 함수 호출 오버헤드를 줄이기 위해 강제 인라인 처리                     |
| `no_std`              | 표준 라이브러리 제거로 바이너리 크기 축소 및 임베디드 환경 대응 가능     |
| `packed_simd`         | SIMD 명령어를 활용한 벡터 연산 최적화 (CPU 병렬 처리에 유리)           |
| `unsafe`              | 안전성 체크를 우회하여 성능 극대화 (단, 철저한 검증 필요)              |
| `cargo bench` + `criterion` | 벤치마크 도구를 활용한 정량적 성능 측정 및 튜닝                     |



### 🧩 6. 실전 적용 예시
- 게임 엔진의 물리 계산 모듈
- 이미지 필터링 / 신호 처리
- AI 추론 엔진의 전처리 로직
- 암호화 / 압축 / 해시 연산

### ✅ 마무리 요약

| 항목             | 요약 내용                                      |
|------------------|------------------------------------------------|
| 언어 선택         | Rust: 성능 + 안전성 + 크로스 플랫폼 지원         |
| 대상 플랫폼       | Android (NDK), Linux, Windows, macOS 등         |
| 출력 결과         | `.so` 파일 생성 (NDK용 네이티브 라이브러리)     |
| 배포 경로         | `jniLibs/arm64-v8a`, `jniLibs/x86_64` 등        |
| 최적화 전략       | `no_std`, `SIMD`, `unsafe`, `cargo bench` 등    |

---


