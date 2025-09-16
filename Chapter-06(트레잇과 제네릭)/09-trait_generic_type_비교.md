# Generic & Trait 
Rust에서 Generic과 Trait을 결합하는 방식을 보여주며,  
특히 trait Printable<AGE>처럼 트레이트에 제네릭 타입 파라미터를 사용하는 경우와 trait Printable { type Age; ... }처럼 associated type을 사용하는 경우의 차이를 비교할 수 있음.

### 🧠 두 방식 비교: Generic Type vs Associated Type
| 방식                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `trait Printable<AGE>`        | 트레이트 자체가 제네릭 타입 파라미터를 받음. 구현 시마다 타입을 명시해야 함 |
| `trait Printable { type Age; }` | 트레이트 내부에 연관 타입을 선언. 구현 시 `type Age = ...`로 지정함         |



## ✅ 장단점 비교
### 🎯 Generic Type vs Associated Type

| 항목                     | Generic Type (`trait Printable<AGE>`)                     | Associated Type (`trait Printable { type Age; }`)         |
|--------------------------|------------------------------------------------------------|------------------------------------------------------------|
| 선언 방식                | 트레이트에 타입 파라미터를 직접 명시                         | 트레이트 내부에 `type Age`로 연관 타입 선언                 |
| 구현 시 타입 지정        | `impl<ADDR> Printable<u32> for Person<ADDR>`              | `impl Printable for Person { type Age = u32; }`            |
| 함수 인자 제한 방식      | `&dyn Printable<u32>`                                      | `&dyn Printable<Age = u32>`                                |
| 다형성 유연성            | 여러 타입에 대해 같은 트레이트를 다양한 타입으로 구현 가능     | 타입마다 고정된 연관 타입을 지정해야 함                     |
| 복잡한 제약 표현         | 제네릭 바운드와 함께 사용하면 표현이 복잡해질 수 있음         | `where` 절과 함께 사용하면 가독성 좋게 제약 표현 가능       |
| trait object 사용        | 제한적 (`dyn Printable<u32>`만 가능)                        | trait object에 더 적합 (`dyn Printable<Age = u32>`)         |
| Rust 표준 라이브러리 사용| 대부분 associated type 방식 사용 (`Iterator`, `Future` 등) | Rust idiom(Rust에서 권장되는 스타일)에 더 가까움             |



## 🧩 코드 예제 기반 설명
### Generic Type 방식 (trait Printable<AGE>)
```rust
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}
```

- AGE는 트레이트를 구현할 때마다 명시적으로 지정해야 함
- impl<ADDR> Printable<u32> for Person<ADDR>처럼 구현
- dyn Printable<u32>로 trait object 사용 가능하지만, 제약이 있음

### Associated Type 방식 (trait Printable { type Age; })
```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

- Self::Age로 트레이트 내부에서 타입을 참조
- impl Printable for Person { type Age = u32; ... }처럼 구현
- dyn Printable<Age = u32>로 trait object 사용이 더 자연스러움

## 🧠 어떤 방식이 더 좋을까?
- 간단한 타입 매핑이나 trait object 사용이 필요한 경우 → associated type 방식이 더 적합
- 다양한 타입 조합을 유연하게 처리하고 싶다면 → generic type parameter 방식이 유리
Rust 표준 라이브러리도 대부분 associated type을 채택하고 있어요 
(Iterator::Item, Future::Output 등).  
따라서 Rust 스타일에 맞추려면 associated type 방식이 더 자연스럽습니다.
