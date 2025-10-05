# mut 위치

## 문제 있는 소스
```rust

fn on_normalize(mut v: Vector3D) -> Option<Vector3D> {
    let len2 = v.x*v.x + v.y*v.y + v.z*v.z;
    if len2 == 0.0 { return None; }
    let inv = 1.0/len2.sqrt();
    v.x *= inv; v.y *= inv; v.z *= inv;
    Some(v)
}

fn on_normalize(mut v: &Vector3D) -> Option<Vector3D> {
    let len2 = v.x*v.x + v.y*v.y + v.z*v.z;
    if len2 == 0.0 { return None; }
    let inv = 1.0/len2.sqrt();
    v.x *= inv; v.y *= inv; v.z *= inv;
    Some(v)
}



```
핵심은 Rust의 가변 참조(&mut)와 값 복사(Copy)의 차이에 있어요.
당신이 작성한 on_normalize(mut v: Vector3D)는 값을 복사해서 수정하는 방식이고,
on_normalize(mut v: &Vector3D)는 참조를 통해 읽기만 하겠다는 의미예요.

## 🔍 왜 &Vector3D로 하면 에러가 나는가?
### 1. &Vector3D는 불변 참조예요
```rust
fn on_normalize(mut v: &Vector3D)
```

- 여기서 mut v는 참조 자체가 변경 가능하다는 뜻이지
v.x, v.y, v.z를 변경할 수 있다는 뜻이 아니에요
- &Vector3D는 내부 필드를 수정할 수 없어요 → ❌ v.x *= inv

### 2. Vector3D는 Copy 타입이라면, 값으로 받는 게 더 자연스러움
```rust
fn on_normalize(mut v: Vector3D)
```

- 이건 v를 복사해서 함수 안에서 자유롭게 수정 가능
- 수정 후 Some(v)로 반환하면 끝

### ✅ 해결 방법 3가지
### 방법 1: 값으로 받기 (현재 방식)
```rust
fn on_normalize(mut v: Vector3D) -> Option<Vector3D>
```

→ 가장 간단하고 안전함

### 방법 2: 가변 참조로 받고 직접 수정
```rust
fn on_normalize(v: &mut Vector3D) -> bool {
    let len2 = v.x*v.x + v.y*v.y + v.z*v.z;
    if len2 == 0.0 { return false; }
    let inv = 1.0 / len2.sqrt();
    v.x *= inv; v.y *= inv; v.z *= inv;
    true
}
```

→ v를 직접 수정하고, 성공 여부만 반환

## 방법 3: 불변 참조로 받고 새 값 반환 (복사)
```rust
fn on_normalize(v: &Vector3D) -> Option<Vector3D> {
    let len2 = v.x*v.x + v.y*v.y + v.z*v.z;
    if len2 == 0.0 { return None; }
    let inv = 1.0 / len2.sqrt();
    Some(Vector3D {
        x: v.x * inv,
        y: v.y * inv,
        z: v.z * inv,
    })
}
```

→ 원본은 그대로 두고, 새 벡터를 반환

## 💬 결론
- &Vector3D는 불변 참조이기 때문에 내부 필드를 수정할 수 없음.
- mut v: &Vector3D는 “참조 변수 v를 바꿀 수 있다”는 뜻이지, “참조 대상의 값을 바꿀 수 있다”는 뜻은 아님.


## 🔍 다시 정리하면
```rust
fn on_normalize(mut v: &Vector3D)
```

- 여기서 mut는 참조 변수 v 자체를 다른 참조로 바꿀 수 있다는 뜻
- 하지만 v가 가리키는 Vector3D는 불변이기 때문에 내부 필드 수정은 ❌
반면에:
```rust
fn on_normalize(mut v: Vector3D)
```

- v는 값 자체이므로 자유롭게 수정 가능
- mut는 v.x, v.y, v.z를 바꿀 수 있게 해줌

### 💡 `mut` 위치에 따른 의미 차이

| 선언 방식              | 의미 설명                                                                 |
|------------------------|---------------------------------------------------------------------------|
| `mut v: Vector3D`      | `v`는 값 자체이며, 내부 필드(`x`, `y`, `z`)를 자유롭게 수정 가능             |
| `mut v: &Vector3D`     | `v`는 참조 변수이며, 참조 자체를 다른 주소로 바꿀 수 있지만 **대상은 불변** |
| `v: &mut Vector3D`     | `v`는 가변 참조이며, **참조 대상의 필드를 직접 수정 가능**                   |

---



