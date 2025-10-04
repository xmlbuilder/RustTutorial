# 메모리 레이 아웃 분석
```rust
struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("멍멍, 제 이름은 {}입니다.", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("냐옹!")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
}
```

## pets를 할당한 이후의 메모리 레이아웃:

![메모리 레이아웃](/image/메모리_layout.png)




