# Generic Associated Types
코드는 Rust의 **GAT (Generic Associated Types)**를 활용한 트레이트 설계.
아래에 하나씩 풀어서 설명.

## 🧠 전체 구조 요약
```rust
trait Scene {
    type Iter<'a>: Iterator<Item = &'a Node> where Self: 'a;
    fn nodes(&self) -> Self::Iter<'_>;
}
```

이 트레이트는 다음을 의미:
- Scene이라는 추상 타입은  
    nodes()라는 메서드를 통해  
    Node에 대한 이터레이터를 반환하는데  
    그 이터레이터는 수명 'a에 따라 달라지는 타입이다

## 🔍 핵심 개념: GAT (Generic Associated Type)
### ✅ type Iter<'a>: Iterator<Item = &'a Node>
- Scene이 정의하는 이터레이터 타입은 'a라는 수명을 받아야 함
- 즉, Scene이 어떤 구조체든 간에,  
    그 구조체의 내부 데이터를 'a 수명으로 빌려주는 이터레이터를 정의해야 함
### ✅ where Self: 'a
- Scene 객체 자체가 'a 동안 살아 있어야 함
- 즉, nodes()를 호출할 때 &self가 'a 동안 유효해야 그 수명으로 &'a Node를 반환할 수 있음

### 🧪 예시로 풀어보기
```rust
struct MyScene {
    nodes: Vec<Node>,
}

impl Scene for MyScene {
    type Iter<'a> = std::slice::Iter<'a, Node>;

    fn nodes(&self) -> Self::Iter<'_> {
        self.nodes.iter()
    }
}
```

- MyScene은 내부에 Vec<Node>를 가지고 있고
- nodes()를 호출하면 &self.nodes를 빌려서 slice::Iter<'a, Node>를 반환함
- 이때 'a는 &self의 수명과 동일하게 '_'로 추론됨

### ✨ 왜 이렇게 쓰는가?
| 항목              | 설명                                                                 |
|-------------------|----------------------------------------------------------------------|
| `Scene` / `dyn Scene` | GAT를 쓰면 객체 안전한 트레이트로 설계 가능. `dyn Scene`으로도 사용 가능       |
| `Vec` / `SmallVec`    | 구조체마다 다른 이터레이터 타입을 정의 가능. `Vec`, `SmallVec`, `slice::Iter` 등 |
| `nodes()`            | 수명 추론이 자동으로 이루어짐. 호출자가 수명을 직접 지정할 필요 없음             |



### 🧩 비교: GAT 없이 하면?
```rust
trait Scene {
    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Node> + 'a>;
}
```

- 이 방식은 Box heap 할당이 필요하고
- 성능 손해 + 객체 안전성 제한이 생길 수 있음

## ✅ 요약
이 코드는 “Scene이 내부 데이터를 빌려주는 이터레이터를 정의하는데,  
그 이터레이터의 타입은 수명 'a에 따라 달라진다”는 의미.  
GAT를 쓰면 수명 안전 + 성능 최적화 + 객체 안전성을 동시에 잡을 수 있음.  

---



