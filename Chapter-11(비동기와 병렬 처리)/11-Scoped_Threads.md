# Scoped Threads
아래는 Rust의 범위 스레드(Scoped Threads) 개념을 두 샘플을 통해 단계적으로 구조적으로 정리한 내용입니다.
C++ 스타일의 스레드와 비교해도 의미가 분명히 드러나는 구조.

## 🧩 Scoped Threads란?
Rust에서 thread::spawn()은 스레드가 독립적으로 실행되며, 클로저 안에서 외부 변수에 접근하려면 'static 수명이 필요합니다.  
하지만 thread::scope()는 스레드의 수명을 현재 스코프에 묶어줌으로써,  
외부 변수에 안전하게 접근할 수 있도록 보장합니다.  
즉, 스레드가 스코프를 벗어나기 전에 반드시 종료되도록 구조적으로 제어하는 방식입니다.  

### 🔍 샘플 1: 일반 스레드 (❌ 컴파일 오류 발생)
```rust
use std::thread;

fn foo() {
    let s = String::from("안녕하세요");
    thread::spawn(|| {
        println!("길이: {}", s.len()); // ❌ s는 move되지 않아서 접근 불가
    });
}

fn main() {
    foo();
}
```


❌ 문제점
- thread::spawn()은 'static 수명을 요구
- s는 지역 변수 → 스레드가 실행될 때 이미 drop될 수 있음  
    → 컴파일 오류: s는 클로저 안에서 접근 불가

### ✅ 샘플 2: 범위 스레드 (Scoped Thread)
```rust
use std::thread;

fn main() {
    let s = String::from("안녕하세요");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("길이: {}", s.len()); // ✅ 안전하게 접근 가능
        });
    });
}
```

## ✅ 해결 방식
- thread::scope()는 스레드의 수명을 현재 스코프에 묶음
- scope.spawn()으로 생성된 스레드는  
    → 스코프가 끝나기 전에 반드시 join됨
- s는 스코프 내에서 유효하므로  
    → 클로저에서 안전하게 접근 가능

## 🔧 핵심 차이 요약
| 항목               | thread::spawn()               | thread::scope()                      |
|--------------------|-------------------------------|--------------------------------------|
| 스레드 수명        | `'static` 요구                | 현재 스코프에 묶임                   |
| 외부 변수 접근     | move 필요, 제한적              | 안전하게 접근 가능                   |
| 자동 join 여부     | 수동 join 필요                | 스코프 종료 시 자동 join             |
| 안전성             | 런타임 오류 가능성 있음        | 컴파일 시점에 안전성 보장            |

## 🧩 핵심: thread::scope는 스레드의 수명을 현재 스코프에 “묶는다”
```rust
use std::thread;

fn main() {
    let s = String::from("안녕하세요");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("{}", s); // ✅ 안전하게 접근 가능
        });
    }); // ✅ 여기서 스레드가 반드시 종료됨
}
```

## ✅ 보호 원리
- scope.spawn()으로 생성된 스레드는
현재 스코프가 끝나기 전에 반드시 join됨
- Rust는 scope 내부 클로저에 'scope 라이프타임을 부여함  
    → s는 'scope 라이프타임을 가지므로  
    → 스레드가 s보다 오래 살아남을 수 없음  
- 컴파일러가 스레드의 수명과 변수의 수명을 정적으로 분석해서 런타임 오류 없이 안전하게 보장

## 🔍 내부적으로 어떤 일이 일어나는가?
- thread::scope는 Scope<'scope> 타입을 생성
- scope.spawn()은 FnOnce() + 'scope 클로저만 받음
- 즉, 클로저 안에서 'scope 라이프타임을 가진 변수만 접근 가능
- Rust는 'scope가 끝나기 전에 모든 스레드가 종료되도록 보장

## ✅ 이게 왜 중요한가?
- 일반 thread::spawn()은 'static 수명을 요구  
    → 외부 변수 접근 불가 or move 필요
- thread::scope()는 'scope 수명을 부여  
    → 외부 변수에 안전하게 접근 가능 → 스레드가 스코프를 벗어나지 않도록 구조적으로 보호


---

