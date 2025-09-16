# MySQL에 연결
Rust에서 MySQL에 연결하고 쿼리하는 방법을 완전 집중해서, 실전 위주로 단계별로 설명.  
로컬에 MySQL이 설치되어 있고, 계정으로 로그인해서 데이터를 가져오고 싶으시다면 아래 순서대로 진행하시면 됩니다.

## 🦀 1단계: mysql 크레이트 설치
먼저 Cargo.toml에 MySQL 클라이언트 라이브러리를 추가합니다.
[dependencies]
mysql = "25.0.1"


최신 버전은 crates.io/mysql에서 확인 가능


🦀 2단계: 기본 연결 및 SELECT 쿼리
use mysql::*;

fn main() -> Result<()> {
    // ① MySQL 접속 URL 구성
    let url = "mysql://root:*****@localhost:3306/dbname";

    // ② 커넥션 풀 생성
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // ③ SELECT 쿼리 실행 및 결과 매핑
    let users: Vec<String> = conn.query_map(
        "SELECT name FROM users",
        |name: String| name,
    )?;

    // ④ 결과 출력
    for user in users {
        println!("User: {}", user);
    }

    Ok(())
}

## 출력 결과
```
User: 김철수
User: 이영희
User: 박민수
User: 최지은
User: 정우성
User: 한지민
User: 오세훈
User: 윤아름
User: 장동건
User: 배수지
```

## 함수 추가
```rust
let result: Option<String> = conn.exec_first(
        "SELECT email FROM customers WHERE name = :name",
        params! { "name" => "김철수" },
    )?;
    if let Some(email) = result {
        println!("김철수의 이메일: {}", email);
    }
```
### 출력 결과
```
김철수의 이메일: chulsoo@example.com
```


### 🧠 Rust MySQL 주요 함수 설명

| 함수 / 타입        | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| `Pool::new(url)`   | 주어진 URL로 커넥션 풀을 생성합니다. 여러 연결을 효율적으로 관리할 수 있어요. |
| `get_conn()`       | 커넥션 풀에서 실제 DB 연결 객체를 하나 가져옵니다. 쿼리를 실행할 때 사용됩니다. |
| `query_map()`      | SELECT 쿼리를 실행하고, 결과를 Rust 타입으로 매핑합니다. 반복 가능한 벡터로 반환돼요. |
| `Result<()>`       | 에러 처리를 위한 표준 타입입니다. `?` 연산자와 함께 사용하면 깔끔한 에러 전파가 가능합니다. |



🔐 접속 URL 구성법
```rust
mysql://<username>:<password>@<host>:<port>/<database>
```

예시:
```rust
mysql://root:1234@localhost:3306/mydb
```


## 🧪 3단계: INSERT / UPDATE / DELETE 예제
```rust
conn.exec_drop(
    "INSERT INTO users (name, age) VALUES (:name, :age)",
    params! {
        "name" => "JungHwan",
        "age" => 42,
    },
)?;
```

exec_drop은 결과를 반환하지 않는 쿼리에 사용됩니다.


### 🧪 4단계: Prepared Statement 방식
```rust
let stmt = conn.prep("SELECT id, name FROM users WHERE age > :age")?;
let result = conn.exec_map(
    &stmt,
    params! { "age" => 30 },
    |(id, name): (u32, String)| {
        format!("{}: {}", id, name)
    },
)?;

for row in result {
    println!("{}", row);
}
```

보안과 성능을 위해 Prepared Statement를 사용하는 것이 좋습니다.


## 🧰 5단계: 에러 처리와 디버깅 팁
- ? 연산자를 사용하면 에러를 자동으로 전파할 수 있어요.
- dbg!()를 사용하면 변수 값을 쉽게 출력할 수 있어요.
- mysql::Error를 match로 처리하면 세부 에러 분류도 가능합니다.

## 📦 6단계: 실전 구조 예시 (모듈화)
```rust
mod db {
    use mysql::*;
    pub fn get_connection() -> Result<PooledConn> {
        let url = "mysql://root:1234@localhost:3306/mydb";
        let pool = Pool::new(url)?;
        Ok(pool.get_conn()?)
    }
}


fn main() -> Result<()> {
    let mut conn = db::get_connection()?;
    let names: Vec<String> = conn.query_map("SELECT name FROM users", |name: String| name)?;
    for name in names {
        println!("User: {}", name);
    }
    Ok(())
}
```


### 🧪 7단계: Rust에서 MySQL 연결 확인 팁
- MySQL 서버가 실행 중인지 확인: mysql.server start 또는 systemctl start mysql
- 포트 확인: 기본은 3306
- 사용자 권한 확인: GRANT ALL PRIVILEGES ON dbname.* TO 'username'@'localhost';

## 🧠 확장 아이디어
- dotenv로 비밀번호 숨기기
- serde와 함께 JSON으로 결과 출력
- tokio-mysql로 비동기 처리 (고급)



----

# dotenv 사용하기

## 🔐 1. dotenv로 비밀번호 숨기기
환경 변수로 민감한 정보를 숨기면 보안과 유지보수에 훨씬 유리합니다.
### ✅ 설치
```
[dependencies]
dotenv = "0.15"
```

### ✅ .env 파일 생성
```
DATABASE_URL=mysql://root:1234@localhost:3306/mydb
```

.env 파일은 루트 디렉토리에 위치하며, .gitignore에 반드시 포함시켜야 합니다.

## ✅ 코드에서 사용
```rust

use dotenv::dotenv;
use std::env;

use mysql::{Pool, Opts};


use mysql::prelude::Queryable;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("환경 변수 DATABASE_URL 누락");
            return Err("환경 변수 누락".into());
        }
    };

    let opts = Opts::from_url(&url)?; // ✅ URL → Opts 변환
    let pool = Pool::new(opts)?;      // ✅ Pool 생성

    let mut conn = pool.get_conn()?;
    let users: Vec<String> = conn.query_map("SELECT name FROM customers", |name: String| name)?;
    for name in users {
        println!("User: {}", name);
    }

    Ok(())
}

```

---

## 📦 2. serde로 JSON 출력하기

Rust에서 DB 결과를 JSON으로 출력하려면 serde를 사용하면 깔끔하게 직렬화할 수 있어요.


### ✅ 설치
```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### ✅ 구조체 정의
```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Customer {
    name: String,
    email: String,
    join_date: String,
}
```

### ✅ DB 결과를 구조체로 매핑
```rust
let customers: Vec<Customer> = conn.query_map(
    "SELECT name, email, join_date FROM customers",
    |(name, email, join_date): (String, String, String)| {
        Customer { name, email, join_date }
    },
)?;
```

### ✅ JSON 출력
```rust
let json = serde_json::to_string_pretty(&customers)?;
println!("{}", json);
```

to_string_pretty는 보기 좋은 들여쓰기 형태로 출력합니다.


### 🧠 결과 예시
```
[
  {
    "name": "김철수",
    "email": "chulsoo@example.com",
    "join_date": "2025-09-01 10:00:00"
  },
  ...
]
```

### 전체 코드

```rust
use dotenv::dotenv;
use std::env;

use mysql::{Pool, Opts};


use mysql::prelude::Queryable;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct Customer {
    name: String,
    email: String,
    join_date: String,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("환경 변수 DATABASE_URL 누락");
            return Err("환경 변수 누락".into());
        }
    };

    let opts = Opts::from_url(&url)?; // ✅ URL → Opts 변환
    let pool = Pool::new(opts)?;      // ✅ Pool 생성

    let mut conn = pool.get_conn()?;
    let customers: Vec<Customer> = conn.query_map(
        "SELECT name, email, join_date FROM customers",
        |(name, email, join_date): (String, String, String)| {
            Customer { name, email, join_date }
        },
    )?;

    let json = serde_json::to_string_pretty(&customers)?;
    println!("{}", json);
    Ok(())
}


```

### 출력 결과
```
[
  {
    "name": "김철수",
    "email": "chulsoo@example.com",
    "join_date": "2025-09-01 10:00:00"
  },
  {
    "name": "이영희",
    "email": "younghee@example.com",
    "join_date": "2025-09-02 11:15:00"
  },
  {
    "name": "박민수",
    "email": "minsoo@example.com",
    "join_date": "2025-09-03 09:30:00"
  },
  {
    "name": "최지은",
    "email": "jieun@example.com",
    "join_date": "2025-09-04 14:45:00"
  },
  {
    "name": "정우성",
    "email": "woosung@example.com",
    "join_date": "2025-09-05 16:20:00"
  },
  {
    "name": "한지민",
    "email": "jimin@example.com",
    "join_date": "2025-09-06 08:50:00"
  },
  {
    "name": "오세훈",
    "email": "sehoon@example.com",
    "join_date": "2025-09-07 13:10:00"
  },
  {
    "name": "윤아름",
    "email": "areum@example.com",
    "join_date": "2025-09-08 17:40:00"
  },
  {
    "name": "장동건",
    "email": "donggun@example.com",
    "join_date": "2025-09-09 12:00:00"
  },
  {
    "name": "배수지",
    "email": "suzy@example.com",
    "join_date": "2025-09-10 15:30:00"
  }
]



```



