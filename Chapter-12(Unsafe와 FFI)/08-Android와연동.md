
# Rust / Android

## 1. Android Native 프로젝트에 Rust를 통합하는 방법 (NDK 기반)
Rust를 NDK 기반의 C/C++ 네이티브 코드처럼 빌드해서 Android 앱에 포함시키는 방식입니다.
#### ✅ 전체 흐름 요약

```
Rust 코드 → Cargo 빌드 → .so 라이브러리 생성 → Android JNI 연동 → Java/Kotlin에서 호출
```


## 🔧 단계별 설명
### ① Rust 라이브러리 생성
```
cargo new --lib rust_ffi
```

### Cargo.toml에 다음 추가:
```
[lib]
crate-type = ["cdylib"]
```

### ② Rust 코드 작성 (예: src/lib.rs)
```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

### ③ 빌드 대상 설정
```
rustup target add aarch64-linux-android
```

### ④ NDK toolchain으로 빌드
```
cargo build --target aarch64-linux-android --release
```
→ target/aarch64-linux-android/release/librust_ffi.so 생성됨

### ⑤ Android Studio에 .so 추가
- app/src/main/jniLibs/arm64-v8a/librust_ffi.so 복사
- MainActivity.kt에서 JNI로 호출:

```kotlin
external fun rust_add(a: Int, b: Int): Int

init {
    System.loadLibrary("rust_ffi")
}
```

---

## 🧩 2. Android 플랫폼 개발 (AOSP)에서 Soong으로 Rust 통합
AOSP 개발자라면 Soong 빌드 시스템을 통해 Rust 모듈을 직접 등록할 수 있습니다.
### ✅ Android.bp 예시
```
rust_library {
    name: "librust_ffi",
    crate_name: "rust_ffi",
    srcs: ["src/lib.rs"],
    crate_type: "cdylib",
    target: {
        android_arm64: {
            enabled: true,
        },
    },
}
```
→ AOSP 빌드 시 Rust 모듈이 자동으로 .so로 빌드됨

## 📦 JNI 바인딩 자동화 도구
- jni 크레이트: Rust에서 JNI 타입을 직접 다룰 수 있음
- ndk-build, cargo-ndk: 빌드 자동화
- bindgen: C 헤더 → Rust 바인딩 자동 생성

---
# AOSP

**AOSP(Android Open Source Project)**는 오픈소스 프로젝트이기 때문에,  
Rust를 Android 플랫폼에 통합하거나 Soong 빌드 시스템을 활용하는 데 가입은 필요하지 않습니다.

## ✅ AOSP는 누구나 접근 가능한 오픈소스입니다
- AOSP 공식 사이트에서 소스 코드, 문서, 빌드 도구 모두 공개되어 있음
- Git을 통해 소스 클론 가능:
```rust
repo init -u https://android.googlesource.com/platform/manifest
repo sync
```
- Rust 모듈을 Soong에 추가하거나 수정하는 것도 자유롭게 가능


## 🔐 가입이 필요한 경우 (AOSP 기준)
가입이 필요한 건 다음과 같은 기여자 또는 내부 개발자 역할일 때입니다:

| 역할 구분             | 설명                                                  | 가입 목적 및 필요성                    |
|----------------------|-------------------------------------------------------|----------------------------------------|
| 핵심 커널 기여자      | Android 커널에 직접 패치하거나 새로운 기능을 제안하는 개발자 | Gerrit 권한, 코드 리뷰 및 병합 가능     |
| HAL 개발자           | 하드웨어 추상화 계층(HAL)을 설계 및 구현하는 엔지니어     | Vendor 브랜치 접근, 테스트 환경 설정    |
| 시스템 서비스 개발자 | AOSP의 Java 기반 시스템 서비스를 개발하는 역할           | AOSP 내부 구조 이해, CTS 대응 필요      |
| 보안 패치 담당자      | Android 보안 취약점 수정 및 CVE 대응을 담당하는 개발자     | 비공개 브랜치 접근, 보안 리뷰 권한      |
| 빌드/릴리즈 엔지니어  | AOSP 빌드 시스템 및 OTA 릴리즈를 관리하는 역할            | 빌드 서버 접근, 릴리즈 태그 관리        |
| CTS/VTS 테스트 담당자 | Compatibility Test Suite 및 Vendor Test Suite 운영자     | 테스트 결과 공유, 인증 프로세스 대응    |
→ 즉, AOSP를 사용하는 것과 AOSP에 기여하는 것은 다릅니다.

Android Studio의 일반적인 앱 템플릿은 Rust를 직접 지원하지 않기 때문에,
Rust를 Android에 통합하려면 별도의 도구와 설정이 필요합니다.

---


## 📦 Android Studio에서 Rust 개발을 위한 준비물
### ✅ 1. Rust 설치
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Rust toolchain 설치
- cargo, rustc, rustup 명령어 사용 가능

### ✅ 2. Android NDK 설치
- Android Studio → SDK Manager → NDK 설치
- 또는 수동 설치: NDK 다운로드 페이지

### ✅ 3. Rust용 Android 타겟 추가
```
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
```

- Android 기기용 크로스 컴파일 타겟

### ✅ 4. cargo-ndk 설치 (빌드 자동화 도구)
```
cargo install cargo-ndk
```

- Rust 라이브러리를 Android용 .so로 빌드하는 도구
- 예시:
```
cargo ndk -t arm64-v8a -o ./jniLibs build --release
```


### ✅ 5. Android Studio 프로젝트 설정
- app/src/main/jniLibs/arm64-v8a/에 .so 파일 복사
- MainActivity.kt에서 JNI로 Rust 함수 호출
- CMakeLists.txt는 필요 없음 (Rust는 CMake 없이 빌드됨)


### 🔧 Rust 라이브러리 설정 예시 (Cargo.toml)
```
[lib]
crate-type = ["cdylib"]
```

→ Rust가 .so 형태로 빌드되도록 설정

## 🔍 비교: C++ + CMake vs Rust + cargo-ndk

| 항목                     | C++ + CMake (ndk-build)                  | Rust + cargo-ndk                         |
|--------------------------|------------------------------------------|------------------------------------------|
| 빌드 설정 파일           | `CMakeLists.txt`, `Android.mk`           | `Cargo.toml`, `cargo-ndk`                |
| ABI 타겟 지정            | `abiFilters` in `build.gradle` or CMake  | `cargo ndk -t arm64-v8a build`           |
| 외부 함수 노출 방식      | `extern "C"`                              | `#[no_mangle] extern "C"`                |
| 출력 라이브러리 형식     | `.so`                                     | `.so`                                     |
| 언어 특성                | 수동 메모리 관리, 복잡한 헤더 구조       | 안전한 메모리 모델, 명시적 unsafe 경계   |
| 크로스 컴파일 지원       | CMake toolchain 설정 필요                 | `cargo-ndk`가 자동으로 처리              |
| NDK 통합 난이도          | 중간 ~ 높음                               | 낮음 ~ 중간                               |
| 디버깅 및 심볼 관리      | gdb, lldb, ndk-stack                      | `gdb`, `lldb`, `cargo build --debug`     |
| JNI 연동                 | 수동 바인딩 필요                          | `jni` crate로 안전하게 래핑 가능         |


### ✅ Rust가 더 쉬운 이유
- cargo가 빌드, 테스트, 배포까지 통합 관리
- cargo-ndk가 NDK toolchain을 자동으로 감지하고 설정
- .so 파일만 Android Studio에 복사하면 끝
- CMake 없이도 완전한 네이티브 라이브러리 생성 가능

### Android Kernel
```
Android 커널의 구성 흐름
Upstream Linux 커널
        ↓
Android Common Kernel (Google 수정본)
        ↓
Vendor 커널 (SoC 제조사 커스터마이징)
        ↓
Device 커널 (OEM이 최종 조정)
```


## ✅ 각 단계 설명
### 1. Upstream Linux 커널
- 리눅스 커널 커뮤니티에서 관리하는 공식 커널
- Android도 여기서 시작함
### 2. Android Common Kernel (ACK)
- Google이 Android에 필요한 기능을 추가한 커널
- 예: wakelocks, binder IPC, ashmem, low memory killer 등
- GitHub에서 공개됨: android/kernel
### 3. Vendor 커널
- Qualcomm, Samsung, MediaTek 등 SoC 제조사가 ACK를 기반으로 수정
- 하드웨어 드라이버, 전력 관리, 보안 기능 추가
### 4. Device 커널
- 스마트폰 제조사(OEM)가 최종적으로 디바이스에 맞게 조정
- 예: 카메라, 센서, 디스플레이 드라이버 등

### 🔍 예시: Pixel 스마트폰의 커널
- Google Pixel은 ACK 기반으로 직접 커널을 빌드
- 커널 소스는 android/kernel에서 확인 가능
- 커널 빌드 시 defconfig와 Device Tree를 통해 디바이스별 설정 적용

---
# rust와 Android 연동


### 🧩 1. Rust 코드 (lib.rs)
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- #[no_mangle]은 함수 이름을 그대로 유지
- extern "C"는 C ABI로 노출

### 🧩 2. Rust 빌드 (cargo-ndk 사용)
```
cargo ndk -t arm64-v8a -o ./jniLibs build --release
```

- 결과: jniLibs/arm64-v8a/libyourlib.so

### 🧩 3. Java 코드에서 호출 (JNI)
```java
public class NativeLib {
    static {
        System.loadLibrary("yourlib"); // libyourlib.so
    }

    public native int add(int a, int b);
}
```


### 🧩 4. Kotlin에서 사용
```
val native = NativeLib()
val result = native.add(3, 5)
println("Result from Rust: $result")
```


### 🧩 5. JNI 헤더 생성 (선택적)
```
javac NativeLib.java
javah -jni NativeLib
```

→ 생성된 .h 파일은 Rust에서 C 인터페이스로 사용할 수 있음 (단, Rust에서는 직접 매핑하는 경우가 많아 생략 가능)

### ✅ 디렉토리 구조 예시
```
app/
├── src/
│   └── main/
│       ├── java/
│       │   └── NativeLib.java
│       └── jniLibs/
│           └── arm64-v8a/
│               └── libyourlib.so
```



