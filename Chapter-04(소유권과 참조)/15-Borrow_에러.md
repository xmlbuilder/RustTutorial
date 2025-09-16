# Borrow 에러 원인 분석

에러 코드 E0502는 "mutable borrow와 immutable borrow가 동시에 발생했다"는 뜻인데,  
Rust는 이런 상황을 절대 허용하지 않아요.

## 🧠 핵심 원리: Rust의 참조 규칙
Rust는 다음 두 가지 규칙을 동시에 만족해야 합니다:
- 하나의 mutable 참조만 허용 (&mut T)
- 또는 여러 개의 immutable 참조만 허용 (&T)
즉, mutable과 immutable 참조가 동시에 존재하면 컴파일 에러가 납니다.


### 🔍 코드 분석
```rust
if let Some(root) = self.root.as_mut() {
    let set = self.elements_to_analyze.clone(); // ❌ 여기서 self를 immutable하게 참조
    self.spatial_subdivision(root, &set, self.limit, &mut tri_count); // ✅ root는 mutable 참조
}
```


### 문제 발생 흐름
- self.root.as_mut() → self.root에 대해 mutable borrow 발생
- self.elements_to_analyze.clone() → self 전체에 대해 immutable borrow 발생
- self.spatial_subdivision(...) → self를 다시 사용 → mutable 참조 중에 immutable 참조가 겹침
- self.root.as_mut()는 self의 일부를 mutable하게 빌림
- 그런데 clone()은 self를 immutable하게 빌림
- Rust는 동시에 mutable + immutable 참조를 허용하지 않음 → 컴파일 에러 발생

###  🧠 핵심 원리: clone()은 &self를 받는다
```
pub trait Clone {
    fn clone(&self) -> Self;
}
```
- clone()은 &self를 인자로 받기 때문에 불변 참조가 필요해요
- 즉, self.elements_to_analyze.clone()은 self를 읽기용으로 빌린 것
- 이 시점에서 self는 immutable borrow 상태가 됩니다


### 🧠 메모리 관점에서 설명
Rust는 **데이터 경쟁(data race)**을 방지하기 위해,
mutable 참조가 활성화된 동안에는 해당 객체에 대한 다른 접근을 금지합니다.
- self.root.as_mut()는 self.root를 독점적으로 수정할 수 있는 권한을 부여
- 그런데 self.elements_to_analyze.clone()은 self를 읽기용으로 접근하려고 함
- Rust는 self.root가 self의 일부이기 때문에, self 전체를 잠금(lock) 처리한 것으로 간주
- 따라서 self를 다시 immutable하게 접근하는 순간, 충돌이 발생하고 E0502가 뜨는 거예요

### ✅ 해결 방법
### 방법 1: clone()을 먼저 수행해 mutable borrow 이전에 끝내기
```rust
let set = self.elements_to_analyze.clone(); // ✅ 먼저 clone
if let Some(root) = self.root.as_mut() {
    let mut tri_count = 0;
    self.spatial_subdivision(root, &set, self.limit, &mut tri_count);
}
```
이렇게 하면 self에 대한 immutable 접근이 먼저 끝나고, 이후에 mutable borrow가 발생하므로 안전해요.


### 방법 2: self.spatial_subdivision을 분리된 함수로 빼기
```rust
fn build(&mut self) {
    let set = self.elements_to_analyze.clone();
    self.build_internal(set);
}

fn build_internal(&mut self, set: Vec<...>) {
    if let Some(root) = self.root.as_mut() {
        let mut tri_count = 0;
        self.spatial_subdivision(root, &set, self.limit, &mut tri_count);
    }
}
```

이렇게 하면 clone()의 lifetime이 짧아지고, borrow checker가 더 쉽게 판단할 수 있어요.


## 🧠 요약
| 원인                          | 설명                                               |
|-------------------------------|----------------------------------------------------|
| mutable borrow (`root`)       | `self.root`에 대한 독점적 접근                     |
| immutable borrow (`clone()`)  | `self` 전체에 대한 읽기 접근                       |
| 충돌 발생                     | Rust는 둘을 동시에 허용하지 않음                   |
| 해결 방법                     | `clone()`을 먼저 수행하거나 함수 분리로 scope 제어 |



Rust의 borrow checker는 메모리 안전을 보장하기 위한 강력한 도구예요.

