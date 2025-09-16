# Multi Adapter
다중 어댑터(Multi Adapter), 비동기 어댑터(Async Adapter), 
그리고 **enum 기반 전략 어댑터(Enum Strategy Adapter)**를 Rust, C++, C#, Python 네 가지 언어로 비교 설명.

## 🧩 1. 다중 어댑터 (Multi Adapter)

### 🦀 Rust
```rust
trait Printer {
    fn print(&self, content: &str);
}

struct XmlPrinter;
impl XmlPrinter {
    fn print_xml(&self, content: &str) {
        println!("<xml>{}</xml>", content);
    }
}

struct JsonPrinter;
impl JsonPrinter {
    fn print_json(&self, content: &str) {
        println!("{{\"content\": \"{}\"}}", content);
    }
}

enum Format {
    Xml(XmlPrinter),
    Json(JsonPrinter),
}

impl Printer for Format {
    fn print(&self, content: &str) {
        match self {
            Format::Xml(p) => p.print_xml(content),
            Format::Json(p) => p.print_json(content),
        }
    }
}

fn main() {
    let adapter = Format::Xml(XmlPrinter);
    adapter.print("Hello XML");

    let adapter = Format::Json(JsonPrinter);
    adapter.print("Hello JSON");
}
```

### ✅ Rust에서는 enum을 활용해 여러 어댑터를 하나의 타입으로 묶고, match로 분기 처리합니다.
이 방식은 런타임에 타입을 바꾸면서도 안전한 분기 처리가 가능하다는 장점이 있어요.

## 💠 C++
```cpp
class IPrinter {
public:
    virtual void print(const std::string& content) = 0;
};

class XmlPrinter {
public:
    void printXml(const std::string& content) {
        std::cout << "<xml>" << content << "</xml>\n";
    }
};

class JsonPrinter {
public:
    void printJson(const std::string& content) {
        std::cout << "{\"content\": \"" << content << "\"}\n";
    }
};

class XmlAdapter : public IPrinter {
    XmlPrinter printer;
public:
    void print(const std::string& content) override {
        printer.printXml(content);
    }
};

class JsonAdapter : public IPrinter {
    JsonPrinter printer;
public:
    void print(const std::string& content) override {
        printer.printJson(content);
    }
};

int main() {
    XmlPrinter xml;
    JsonPrinter json;

    XmlAdapter xmlAdapter;
    JsonAdapter jsonAdapter;

    xmlAdapter.print("Hello XML");
    jsonAdapter.print("Hello JSON");

    return 0;
}
```


### ✅ C++에서는 인터페이스 기반 다형성으로 각 어댑터를 분리합니다.
런타임에 IPrinter*를 통해 다양한 어댑터를 사용할 수 있어요.

## 🧱 C#
```csharp
interface IPrinter {
    void Print(string content);
}

class XmlPrinter {
    public void PrintXml(string content) => Console.WriteLine($"<xml>{content}</xml>");
}

class JsonPrinter {
    public void PrintJson(string content) => Console.WriteLine($"{{\"content\": \"{content}\"}}");
}

class XmlAdapter : IPrinter {
    private XmlPrinter printer = new();
    public void Print(string content) => printer.PrintXml(content);
}

class JsonAdapter : IPrinter {
    private JsonPrinter printer = new();
    public void Print(string content) => printer.PrintJson(content);
}

class Program {
    static void Main() {
        IPrinter xmlAdapter = new XmlAdapter();
        IPrinter jsonAdapter = new JsonAdapter();

        xmlAdapter.Print("Hello XML");
        jsonAdapter.Print("Hello JSON");
    }
}
```


### ✅ C#도 C++처럼 인터페이스 기반 어댑터 구조를 사용하며, 런타임에 IPrinter로 다형성 구현이 가능합니다.

## 🐍 Python
```python
class XmlPrinter:
    def print_xml(self, content):
        print(f"<xml>{content}</xml>")

class JsonPrinter:
    def print_json(self, content):
        print(f'{{"content": "{content}"}}')

class MultiAdapter:
    def __init__(self, printer):
        self.printer = printer

    def print(self, content):
        if hasattr(self.printer, "print_xml"):
            self.printer.print_xml(content)
        elif hasattr(self.printer, "print_json"):
            self.printer.print_json(content)

def main():
    xml = XmlPrinter()
    json = JsonPrinter()

    adapter1 = MultiAdapter(xml)
    adapter2 = MultiAdapter(json)

    adapter1.print("Hello XML")
    adapter2.print("Hello JSON")

if __name__ == "__main__":
    main()

```

### ✅ Python은 동적 타입 덕분에 hasattr()로 런타임에 어댑터를 분기할 수 있어요.
유연하지만 타입 안전성은 낮습니다.

## ⚡ 2. 비동기 어댑터 (Async Adapter)
### 🦀 Rust (with tokio)
```rust
use tokio::time::{sleep, Duration};

#[async_trait::async_trait]
trait AsyncPrinter {
    async fn print(&self, content: &str);
}

struct AsyncXmlPrinter;
#[async_trait::async_trait]
impl AsyncPrinter for AsyncXmlPrinter {
    async fn print(&self, content: &str) {
        sleep(Duration::from_secs(1)).await;
        println!("<xml>{}</xml>", content);
    }
}

#[tokio::main]
async fn main() {
    let printer = AsyncXmlPrinter;
    printer.print("Hello Async XML").await;
}
```

### ✅ Rust에서는 async_trait을 사용해 비동기 트레잇 구현이 가능합니다.
비동기 어댑터는 await 기반으로 동작하며, tokio 런타임이 필요합니다.

### 💠 C++ (std::async)
```cpp
#include <future>
#include <iostream>

class AsyncPrinter {
public:
    void print(const std::string& content) {
        auto task = std::async(std::launch::async, [content]() {
            std::cout << "<xml>" << content << "</xml>\n";
        });
        task.wait();
    }
};

int main() {
    AsyncPrinter printer;
    printer.print("Hello Async XML");
    return 0;
}
```


### ✅ C++에서는 std::async로 비동기 실행이 가능하지만,
Rust처럼 await 기반은 아니고 future 기반 동기화 방식입니다.

### 🧱 C# (async/await)
```csharp
class AsyncPrinter {
    public async Task PrintAsync(string content) {
        await Task.Delay(1000);
        Console.WriteLine($"<xml>{content}</xml>");
    }
}

class Program {
    static async Task Main() {
        var printer = new AsyncPrinter();
        await printer.PrintAsync("Hello Async XML");
    }
}
```

### ✅ C#은 async/await가 언어에 내장되어 있어 비동기 어댑터 구현이 매우 직관적입니다.

## 🐍 Python (asyncio)
```python
import asyncio

class AsyncPrinter:
    async def print(self, content):
        await asyncio.sleep(1)
        print(f"<xml>{content}</xml>")

async def main():
    printer = AsyncPrinter()
    await printer.print("Hello Async XML")

asyncio.run(main())
```

### ✅ Python도 async/await 기반으로 비동기 어댑터를 쉽게 구현할 수 있어요.

## 🎮 3. Enum 기반 전략 어댑터

### 🦀 Rust
```rust
enum Strategy {
    Xml,
    Json,
}

impl Strategy {
    fn print(&self, content: &str) {
        match self {
            Strategy::Xml => println!("<xml>{}</xml>", content),
            Strategy::Json => println!("{{\"content\": \"{}\"}}", content),
        }
    }
}

fn main() {
    let s = Strategy::Xml;
    s.print("Hello Enum Strategy");

    let s = Strategy::Json;
    s.print("Hello Enum Strategy");
}
```

### ✅ Rust의 enum은 전략을 타입으로 표현하고, match로 분기 처리합니다.
확장성은 낮지만 성능과 명확성이 뛰어나요.

### 💠 C++ (enum + switch)
```cpp
enum class Strategy { Xml, Json };

void print(Strategy s, const std::string& content) {
    switch (s) {
        case Strategy::Xml:
            std::cout << "<xml>" << content << "</xml>\n";
            break;
        case Strategy::Json:
            std::cout << "{\"content\": \"" << content << "\"}\n";
            break;
    }
}

int main() {
    print(Strategy::Xml, "Hello XML Strategy");
    print(Strategy::Json, "Hello JSON Strategy");
    return 0;
}
```


### ✅ C++도 enum class와 switch로 전략 분기 가능하지만,
객체지향적 확장성은 낮습니다.

## 🧱 C# (enum + switch)
```csharp
enum Strategy { Xml, Json }

void Print(Strategy s, string content) {
    switch (s) {
        case Strategy.Xml:
            Console.WriteLine($"<xml>{content}</xml>");
            break;
        case Strategy.Json:
            Console.WriteLine($"{{\"content\": \"{content}\"}}");
            break;
    }
}

class Program {
    static void Main() {
        Print(Strategy.Xml, "Hello XML Strategy");
        Print(Strategy.Json, "Hello JSON Strategy");
    }
}
```


### ✅ C#도 enum 기반 전략 분기가 가능하지만,
인터페이스 기반 전략 패턴이 더 일반적입니다.

## 🐍 Python (Enum + if)
```python
from enum import Enum

class Strategy(Enum):
    XML = 1
    JSON = 2

def print(strategy, content):
    if strategy == Strategy.XML:
        print(f"<xml>{content}</xml>")
    elif strategy == Strategy.JSON:
        print(f'{{"content": "{content}"}}')


def main():
    print(Strategy.XML, "Hello XML Strategy")
    print(Strategy.JSON, "Hello JSON Strategy")

if __name__ == "__main__":
    main()
```


### ✅ Python은 Enum과 if로 전략 분기 가능하며,
동적 타입 덕분에 유연하지만 타입 안전성은 낮습니다.

---

