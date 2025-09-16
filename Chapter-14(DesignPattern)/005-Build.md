# 🧱 Builder 패턴 개요

Builder 패턴은 복잡한 객체를 단계적으로 생성할 수 있게 해주는 **생성 패턴(Creational Pattern)**입니다.
특히 필드가 많거나 선택적 속성이 있는 객체를 만들 때 유용하며,
객체 생성 로직을 분리함으로써 코드의 가독성과 유지보수성을 높여줍니다.


## 🦀 Rust 예제
```rust
struct Person {
    name: String,
    age: u8,
    job: String,
}

struct PersonBuilder {
    name: String,
    age: u8,
    job: String,
}

impl PersonBuilder {
    fn new() -> Self {
        PersonBuilder {
            name: String::new(),
            age: 0,
            job: String::new(),
        }
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    fn job(mut self, job: &str) -> Self {
        self.job = job.to_string();
        self
    }

    fn build(self) -> Person {
        Person {
            name: self.name,
            age: self.age,
            job: self.job,
        }
    }
}

fn main() {
    let person = PersonBuilder::new()
        .name("JungHwan")
        .age(35)
        .job("Rust Developer")
        .build();

    println!("Name: {}, Age: {}, Job: {}", person.name, person.age, person.job);
}
```


### ✅ Rust 설명
- PersonBuilder는 Person을 만들기 위한 중간 단계 객체입니다.
- 각 설정 메서드는 self를 반환하여 메서드 체이닝을 가능하게 합니다.
- build() 메서드는 최종적으로 Person 객체를 반환합니다.
- Rust에서는 mut self를 사용해 내부 값을 변경하고, self를 반환해 체이닝을 구현합니다.

## 💠 C++ 예제
```cpp
#include <iostream>
#include <string>

class Person {
public:
    std::string name;
    int age;
    std::string job;

    void show() const {
        std::cout << "Name: " << name << ", Age: " << age << ", Job: " << job << "\n";
    }
};

class PersonBuilder {
    Person person;

public:
    PersonBuilder& name(const std::string& n) {
        person.name = n;
        return *this;
    }

    PersonBuilder& age(int a) {
        person.age = a;
        return *this;
    }

    PersonBuilder& job(const std::string& j) {
        person.job = j;
        return *this;
    }

    Person build() {
        return person;
    }
};

int main() {
    Person person = PersonBuilder()
        .name("JungHwan")
        .age(35)
        .job("C++ Developer")
        .build();

    person.show();
    return 0;
}
```


## 🧱 C# 예제
```csharp
using System;

class Person {
    public string Name { get; set; }
    public int Age { get; set; }
    public string Job { get; set; }

    public void Show() => Console.WriteLine($"Name: {Name}, Age: {Age}, Job: {Job}");
}

class PersonBuilder {
    private Person person = new();

    public PersonBuilder Name(string name) {
        person.Name = name;
        return this;
    }

    public PersonBuilder Age(int age) {
        person.Age = age;
        return this;
    }

    public PersonBuilder Job(string job) {
        person.Job = job;
        return this;
    }

    public Person Build() => person;
}

class Program {
    static void Main() {
        var person = new PersonBuilder()
            .Name("JungHwan")
            .Age(35)
            .Job("C# Developer")
            .Build();

        person.Show();
    }
}
```


## 🐍 Python 예제
```python
class Person:
    def __init__(self, name="", age=0, job=""):
        self.name = name
        self.age = age
        self.job = job

    def show(self):
        print(f"Name: {self.name}, Age: {self.age}, Job: {self.job}")

class PersonBuilder:
    def __init__(self):
        self.person = Person()

    def name(self, name):
        self.person.name = name
        return self

    def age(self, age):
        self.person.age = age
        return self

    def job(self, job):
        self.person.job = job
        return self

    def build(self):
        return self.person

def main():
    person = PersonBuilder()\
        .name("JungHwan")\
        .age(35)\
        .job("Python Developer")\
        .build()

    person.show()

if __name__ == "__main__":
    main()
```


## 🧠 Rust 중심 요약 비교
| 언어     | 체이닝 방식         | 빌더 반환 방식 | 최종 객체 반환 방식 | 특징                         |
|----------|---------------------|----------------|---------------------|------------------------------|
| Rust     | `mut self → Self`   | `self`         | `build() → Person`  | 안전한 소유권, 명확한 타입     |
| C++      | `&self → *this`     | `*this`        | `build() → Person`  | 참조 기반 체이닝              |
| C#       | `this`              | `this`         | `Build() → Person`  | GC 기반, 속성 접근 쉬움        |
| Python   | `self`              | `self`         | `build() → Person`  | 유연한 구조, 타입 검사 없음    |

---



