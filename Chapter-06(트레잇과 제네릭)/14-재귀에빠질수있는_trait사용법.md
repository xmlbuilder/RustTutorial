# Recursive  문제 해결

## 문제 코드
```rust
pub fn distance(&self, pt: &Self) -> f64 
{ 
    self.distance(pt) 
} 

impl Distance for Point3D { 
    fn distance(&self, other: &Self) -> f64 { 
        (*other - *self).length() 
    } 
}
```

이 코드에서 
```rust
pub fn distance(&self, pt: &Self) -> f64 { self.distance(pt) }
```

는 재귀 호출에 빠질 수 있습니다,
어떤 distance를 호출하느냐에 따라 결과가 완전히 달라져요.

## 🔍 핵심 문제: self.distance(pt)는 어떤 distance를 호출하는가?
Rust는 메서드 호출 시 우선순위를 이렇게 따져요:
- 현재 타입에 정의된 메서드
- 트레잇에 정의된 메서드 (해당 트레잇이 in-scope일 경우)
지금 상황에서 distance라는 이름이 자기 자신을 호출하고 있기 때문에,
만약 pub fn distance(&self, pt: &Self)가 impl Point3D 안에 있고
그리고 Distance 트레잇도 Point3D에 impl되어 있다면…
```rust
self.distance(pt)
```
→ 이 호출은 자기 자신을 다시 호출하게 됩니다
→ 즉, 무한 재귀에 빠지고
→ 런타임에서 스택 오버플로우가 발생합니다

### ✅ 해결 방법

### 방법 1: 트레잇 메서드를 명시적으로 호출
```rust
pub fn distance(&self, pt: &Self) -> f64 {
    Distance::distance(self, pt)
}
```

이렇게 하면 Distance 트레잇의 distance 메서드를 명확하게 호출하게 되어
재귀에 빠지지 않습니다


### 방법 2: 메서드 이름을 다르게 하기
```rust
pub fn euclidean_distance(&self, pt: &Self) -> f64 {
    (*pt - *self).length()
}
```

이름을 바꾸면 혼동이 사라지고, 재귀 위험도 없어져요


## ✨ 요약
| 코드 형태                          | 결과                         |
|-----------------------------------|------------------------------|
| `self.distance(pt)`               | ❌ 자기 자신을 재귀 호출함     |
| `Distance::distance(self, pt)`    | ✅ 트레잇 메서드 명시 호출     |
| 메서드 이름 변경 (`euclidean_distance`) | ✅ 혼동 없이 안전하게 작동     |





