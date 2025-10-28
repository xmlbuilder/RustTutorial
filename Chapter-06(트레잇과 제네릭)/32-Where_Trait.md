# 🧠 where 절 설명
Rust에서 `where` 절은 제네릭 타입이나 트레이트 바운드를 더 명확하게 표현할 때 사용.  
특히 복잡한 조건이 많을 때 가독성을 높여줍니다.

## 📦 기본 예시
```rust
fn do_something<T>(item: T) 
where
    T: Clone + Debug,
{
    println!("{:?}", item.clone());
}
```
- T: Clone + Debug는 T가 Clone과 Debug 트레이트를 구현해야 함을 의미
- where 절을 쓰면 함수 시그니처가 깔끔해지고 조건을 분리할 수 있음

## ✅ 언제 where를 쓰는가?

| 상황         | 예시 코드                                      |
|--------------|------------------------------------------------|
| 제네릭 제약이 길 때 | `fn foo<T: TraitA + TraitB + TraitC>`            |
| 여러 타입에 제약이 있을 때 | `where T: TraitA, U: TraitB`                      |
| impl 블록에서 제약을 줄 때 | `impl<T> MyStruct<T> where T: Trait`              |

---

