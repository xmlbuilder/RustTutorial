# 🧠 Adapter 패턴 개요
Adapter 패턴은 서로 다른 인터페이스를 가진 객체들을 연결해주는 구조적 디자인 패턴입니다.

## 🦀 Rust 예제
```rust
// 기존 인터페이스 (Adaptee)
struct LegacyPrinter;

impl LegacyPrinter {
    fn print_xml(&self, content: &str) {
        println!("<xml>{}</xml>", content);
    }
}

// 기대하는 인터페이스 (Target)
trait ModernPrinter {
    fn print(&self, content: &str);
}

// 어댑터
struct PrinterAdapter {
    adaptee: LegacyPrinter,
}

impl ModernPrinter for PrinterAdapter {
    fn print(&self, content: &str) {
        self.adaptee.print_xml(content);
    }
}

fn main() {
    let legacy = LegacyPrinter;
    let adapter = PrinterAdapter { adaptee: legacy };

    adapter.print("Hello, Adapter Pattern!");
}
```

### ✅ Rust 설명
- LegacyPrinter는 기존 XML 기반 출력 방식만 지원합니다.
- ModernPrinter는 클라이언트가 기대하는 일반적인 출력 인터페이스입니다.
- PrinterAdapter는 ModernPrinter 트레잇을 구현하면서 내부적으로 LegacyPrinter를 호출합니다.
- Box<dyn Trait> 없이도 어댑터를 통해 인터페이스 변환이 가능합니다.

## 💠 C++ 예제
```cpp
#include <iostream>
#include <memory>

class LegacyPrinter {
public:
    void printXML(const std::string& content) {
        std::cout << "<xml>" << content << "</xml>\n";
    }
};

class ModernPrinter {
public:
    virtual void print(const std::string& content) = 0;
    virtual ~ModernPrinter() = default;
};

class PrinterAdapter : public ModernPrinter {
    LegacyPrinter* adaptee;

public:
    PrinterAdapter(LegacyPrinter* lp) : adaptee(lp) {}

    void print(const std::string& content) override {
        adaptee->printXML(content);
    }
};

int main() {
    LegacyPrinter legacy;
    PrinterAdapter adapter(&legacy);

    adapter.print("Hello, Adapter Pattern!");
    return 0;
}
```


## 🧱 C# 예제
```csharp
using System;

class LegacyPrinter {
    public void PrintXml(string content) {
        Console.WriteLine($"<xml>{content}</xml>");
    }
}

interface IModernPrinter {
    void Print(string content);
}

class PrinterAdapter : IModernPrinter {
    private LegacyPrinter adaptee;

    public PrinterAdapter(LegacyPrinter adaptee) {
        this.adaptee = adaptee;
    }

    public void Print(string content) {
        adaptee.PrintXml(content);
    }
}

class Program {
    static void Main() {
        var legacy = new LegacyPrinter();
        IModernPrinter adapter = new PrinterAdapter(legacy);

        adapter.Print("Hello, Adapter Pattern!");
    }
}
```


## 🐍 Python 예제
```python
class LegacyPrinter:
    def print_xml(self, content):
        print(f"<xml>{content}</xml>")

class ModernPrinter:
    def print(self, content):
        raise NotImplementedError

class PrinterAdapter(ModernPrinter):
    def __init__(self, adaptee):
        self.adaptee = adaptee

    def print(self, content):
        self.adaptee.print_xml(content)

def main():
    legacy = LegacyPrinter()
    adapter = PrinterAdapter(legacy)
    adapter.print("Hello, Adapter Pattern!")

if __name__ == "__main__":
    main()
```


## 🧩 요약 비교
| 언어     | 기존 인터페이스 | 기대 인터페이스   | 어댑터 구현 방식             | 특징                         |
|----------|------------------|--------------------|------------------------------|------------------------------|
| Rust     | `LegacyPrinter`  | `ModernPrinter`    | `impl Trait for Adapter`     | 트레잇 기반, 안전한 타입 시스템 |
| C++      | `LegacyPrinter`  | `ModernPrinter`    | `virtual` / `override`       | 포인터 기반, 수동 메모리 관리  |
| C#       | `LegacyPrinter`  | `IModernPrinter`   | `interface` / `class`        | GC 기반, 명확한 인터페이스 구조 |
| Python   | `LegacyPrinter`  | `ModernPrinter`    | 상속 기반 (`class Adapter`) | 유연하지만 타입 안전성 낮음     |

---

