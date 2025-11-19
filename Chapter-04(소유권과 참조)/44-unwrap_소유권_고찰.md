# unwrap 소유권 이전이 문제
```rust
pub normals: Option<Vec<Vector3f>>,
let normals = tri_mesh.normals.unwrap().as_slice();
```

## 🧠 반환되는 데이터 타입
```
&[Vector3<f32>]
```
- 즉, nalgebra::Vector3<f32> 타입의 슬라이스
- 각 요소는 삼각형 하나의 법선 벡터를 나타냅니다
- tri_mesh.faces.len()과 동일한 길이를 갖습니다
- 소유권이 이전 된다음 참조가 되어서 컴파일 에러 발생 (`unwrap`)

```
let normals = tri_mesh.normals.unwrap().as_slice();
```
- 소유권이 넘어갑니다. 
- 정확히 말하면 unwrap()이 Option<Vec<_>>의 내부 값을 move하기 때문에, tri_mesh가 & 참조일 경우 컴파일 에러가 발생합니다.

## 🔍 왜 move가 발생하는가?
- tri_mesh.normals의 타입은 `Option<Vec<Vector3f>>`
- unwrap()은 `Vec<Vector3f>` 를 `소유권을 가져오는 방식` 으로 반환함
- 따라서 tri_mesh.normals는 move되고, 이후 tri_mesh는 더 이상 유효하지 않음

```rust
let normals = tri_mesh.normals.as_ref().map(|v| v.as_slice());
```
- as_ref()는 Option<&Vec<_>>로 바꿔줌 → move 없이 참조만 얻음
- map()으로 내부 Vec를 &[T]로 변환

```rust
if let Some(normals) = tri_mesh.normals.as_ref() {
    for n in normals {
        println!("normal = {:?}", n);
    }
}

let normals_f64 = tri_mesh.normals.as_ref().map(|n| convert_normals_to_f64(n));

fn convert_normals_to_f64(normals: &[Vector3f]) -> Vec<Vector3D> {
    normals
        .iter()
        .map(|n| Vector3D::new(n[0] as f64, n[1] as f64, n[2] as f64))
        .collect()
}
```

### 🧠 소유권 헷갈리는 대표 사례 정리

| 코드 예시                                  | 설명                                               | 소유권 이동 여부                  |
|-------------------------------------------|----------------------------------------------------|-----------------------------------|
| `let x = some_option.unwrap();`           | 내부 값을 가져오며 `Option`을 move함               | ✅ 이동됨 → 이후 `some_option` 사용 불가 |
| `let x = some_option.as_ref().unwrap();`  | 내부 값을 참조함 (`Option`은 그대로 유지됨)        | ❌ 이동 안 됨 → 안전하게 참조     |
| `let x = &some_option.unwrap();`          | `unwrap()`에서 move 발생 → 참조 불가               | ✅ 이동됨 → 컴파일 에러 발생      |
| `let x = some_vec[0];`                    | `Copy` 타입이면 복사, 아니면 move 발생              | ⚠️ 타입에 따라 다름               |
| `let x = &some_vec[0];`                   | 요소를 참조함                                     | ❌ 이동 안 됨 → 안전하게 참조     |


### ✅ 기억하면 좋은 패턴
- as_ref() → Option<&T>로 바꿔줌 (소유권 안 뺏김)
- clone() → 소유권이 필요한데 원본도 유지하고 싶을 때
- iter() → 참조 기반 순회
- into_iter() → move 기반 순회 (소유권 넘김)

---

## 요약 

```
let normals = tri_mesh.normals.unwrap().as_slice();
```

### 이 코드는 다음과 같은 소유권 흐름을 따릅니다:
- tri_mesh.normals는 Option<Vec<Vector3f>> 입니다.
- unwrap()을 호출하면 Vec<Vector3f>의 소유권이 tri_mesh에서 이동합니다.
- 그 다음 .as_slice()는 그 벡터의 참조를 반환하지만, 이미 tri_mesh.normals는 move되었기 때문에 tri_mesh 자체가 더 이상 유효하지 않게 됩니다.
- 즉, 참조를 얻기 위해 unwrap을 썼지만, 그 순간 tri_mesh의 일부를 가져가버려서 tri_mesh는 더 이상 쓸 수 없게 된 것임.

## ✅ 안전하게 참조만 하고 싶다면
```rust
let normals = tri_mesh.normals.as_ref().map(|v| v.as_slice());
```
- as_ref()는 Option<&Vec<_>>로 바꿔줍니다.
- unwrap() 대신 if let Some(normals) = ... 패턴을 쓰면 더 안전하고 명확.
- Rust의 소유권은 **move냐, 참조냐**의 싸움이고, `unwrap()` 은 항상 move를 유발한다는 걸 기억하면 많은 혼란이 줄어듬.

---
