# DSL ( Domain-Specific Language)

## ✨ DSL의 특징
- 간결함: 복잡한 로직 없이 핵심만 표현 가능
- 제한된 문법: 범용 언어보다 기능은 적지만, 목적에 맞게 최적화됨
- 도메인 친화적: 해당 분야 전문가가 쉽게 이해하고 사용할 수 있음


지금까지 구성한 DSL 기반 시나리오 시스템은
두 가지 유형의 분석 함수를 처리할 수 있도록 설계되어 있습니다:
- 수식 기반 시나리오 (Formula)
- 절차형 분석 함수 (Procedure)
아래는 전체 구조와 가능한 함수 타입, 등록 방식, 실행 방식 등을 문서화한 상세 가이드.

## 📘 시나리오 시스템 문서
### 1. 🔧 시나리오 종류 (ScenarioKind)
```rust
pub enum ScenarioKind {
    Formula,   // 수식 기반 DSL 시나리오
    Procedure, // 절차형 분석 함수
}
```

- Formula: formula 필드에 수식을 정의하고, inputs를 통해 필요한 metric을 명시
- Procedure: 내부에서 직접 ResultContainer에 값을 넣는 분석 함수

### 2. 📦 시나리오 정의 구조 (ScenarioDefinition)
```rust
pub struct ScenarioDefinition {
    pub name: String,
    pub description: Option<String>,
    pub inputs: Vec<(String, InputType)>,
    pub formula: String,
    pub output: String,
    pub kind: ScenarioKind,
}
```

- inputs: 분석에 필요한 metric 또는 channel 이름과 타입 (InputType::Metric, InputType::Channel)
- formula: 수식 문자열 (예: "HIC15 * 0.5 + Age")
- output: 결과 metric 이름
- kind: 시나리오 종류 (Formula 또는 Procedure)

### 3. 🧠 분석 함수 타입
#### ✅ 수식 기반 시나리오 (AnalysisScenario)
```rust
pub type AnalysisScenario = Box<dyn for<'a> Fn(&'a ResultContainer) -> f64 + Send + Sync>;
```

- ResultContainer에서 metric 값을 읽어와서 계산
- 결과는 f64로 반환됨
- 내부에서 parse_and_eval(formula, context) 사용
#### ✅ 절차형 분석 함수 (AnalysisProcedure)
```rust
pub type AnalysisProcedure = Box<dyn Fn(&mut ResultContainer, &DataContainer) + Send + Sync>;
```

- ResultContainer에 직접 metric 또는 vector 값을 저장
- DataContainer에서 raw 데이터를 읽어와서 처리
- 반환값 없음 (-> ())

### 4. 🏗️ 등록 방식 (ScenarioRegister)
```rust
pub struct ScenarioRegister {
    scenarios: HashMap<String, AnalysisScenario>,
    procedures: HashMap<String, AnalysisProcedure>,
}
```

등록 메서드
```rust
pub fn register(&mut self, name: &str, func: AnalysisScenario)
pub fn register_procedure(&mut self, name: &str, proc: AnalysisProcedure)
```
- Formula는 register()로 등록
- Procedure는 register_procedure()로 등록

### 5. 🚀 실행 방식
수식 기반 실행
pub fn run_scenario(&self, name: &str, container: &ResultContainer) -> Option<f64>


- container에서 metric을 읽고 계산
- 결과 f64 반환
#### 절차형 실행
```rust
pub fn run_procedure(&self, name: &str, result: &mut ResultContainer, data: &DataContainer) -> bool
```

- result에 직접 metric을 저장
- data에서 raw 데이터를 읽어와 처리
- 성공 여부 (true/false) 반환

### 6. 🧩 시나리오 로딩 및 등록 흐름
```rust
for def in definitions {
    match def.kind {
        ScenarioKind::Formula => {
            match def.to_function() {
                Ok(func) => reg.register(&def.output, func),
                Err(e) => eprintln!("Failed to compile formula '{}': {}", def.name, e),
            }
        }
        ScenarioKind::Procedure => {
            match def.to_procedure() {
                Ok(proc) => reg.register_procedure(&def.output, proc),
                Err(e) => eprintln!("Failed to compile procedure '{}': {}", def.name, e),
            }
        }
    }
}
```

- to_function()은 수식 기반 DSL을 AnalysisScenario로 변환
- to_procedure()는 이름 기반으로 절차형 함수를 찾아 AnalysisProcedure로 반환

### 7. 🧭 절차형 함수 등록 예시
```rust
fn get_procedure_by_name(name: &str) -> Option<AnalysisProcedure> {
    match name {
        "HeadInjury" => Some(Box::new(calc_head_injury)),
        "NeckForce" => Some(Box::new(calc_neck_force)),
        _ => None,
    }
}

impl ScenarioDefinition {
    pub fn to_procedure(&self) -> Result<AnalysisProcedure, String> {
        get_procedure_by_name(&self.name)
            .ok_or_else(|| format!("Unknown procedure scenario: {}", self.name))
    }
}
```

## 📌 DSL 시나리오 시스템 요약

| 구성 요소             | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| AnalysisScenario      | `Fn(&ResultContainer) -> f64`<br>수식 기반 시나리오 함수 타입         |
| AnalysisProcedure     | `Fn(&mut ResultContainer, &DataContainer)`<br>절차형 분석 함수 타입   |
| ScenarioKind          | `Formula` 또는 `Procedure`<br>시나리오 종류를 구분하는 enum          |
| ScenarioDefinition    | 시나리오 정의 구조

-----

# JSON To Parser

## 🧠 1. JSON 파일 읽기 함수
```rust
use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;

pub fn load_scenarios_from_file(path: &str) -> Result<Vec<ScenarioDefinition>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let scenarios = from_reader(reader)?;
    Ok(scenarios)
}

```

## 🚀 2. 등록 흐름
```rust
fn load_and_register_scenarios(path: &str) {
    match load_scenarios_from_file(path) {
        Ok(definitions) => {
            if let Ok(mut reg) = SCENARIO_REGISTER.lock() {
                for def in definitions {
                    match def.kind {
                        ScenarioKind::Formula => {
                            match def.to_function() {
                                Ok(func) => reg.register(&def.output, func),
                                Err(e) => eprintln!("Failed to compile formula '{}': {}", def.name, e),
                            }
                        }
                        ScenarioKind::Procedure => {
                            match def.to_procedure() {
                                Ok(proc) => reg.register_procedure(&def.output, proc),
                                Err(e) => eprintln!("Failed to compile procedure '{}': {}", def.name, e),
                            }
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to load scenarios: {}", e),
    }
}
```

## ✅ DSL 시나리오 시스템 - JSON 연동 요약

| 항목                         | 설명 또는 반환 타입                     |
|------------------------------|----------------------------------------|
| ScenarioDefinition + serde   | `#[derive(Serialize, Deserialize)]`로 JSON과 연결 가능 |
| scenarios.json               | 외부에서 시나리오 정의를 관리하는 JSON 파일 |
| load_scenarios_from_file()   | `Result<Vec<ScenarioDefinition>, Error>`<br>JSON 파일을 읽어 구조체로 파싱 |
| load_and_register_scenarios()| 시나리오 종류(`kind`)에 따라 자동 등록 처리 |
| ScenarioKind                 | `Formula` 또는 `Procedure`로 시나리오 타입 구분 |
| to_function()                | `ScenarioDefinition` → `AnalysisScenario` 변환 |
| to_procedure()               | `ScenarioDefinition` → `AnalysisProcedure` 변환 |

---

# Formula 구성

DSL 시스템의 핵심인 formula 문자열이 어떻게 실제 수식으로 평가되는지
그 원리와 코드 흐름, 그리고 사용 가능한 함수 목록까지 자세히 설명.

## 🧠 수식 문자열이 수식으로 평가되는 원리
### 1. 사용되는 평가 엔진: 수식 파서 또는 스크립트 엔진
Rust에서는 수식 문자열을 평가하기 위해 다음 중 하나를 사용할 수 있어요:
-  — 간단한 수학 수식 평가 라이브러리
-  — 강력한 스크립트 엔진, DSL 구축에 적합
- 직접 구현한 파서 — Shunting Yard 알고리즘 기반 수식 해석기
formula: "HIC15 * 0.5 + Age" 같은 문자열을 처리하려면 이 문자열을 파싱 → 변수 치환 → 평가하는 과정이 필요합니다.

#### 🔧 코드 흐름 예시
```rust
pub fn to_function(&self) -> Result<AnalysisScenario, String> {
    let formula = self.formula.clone();
    let inputs = self.inputs.clone();

    Ok(Box::new(move |container: &ResultContainer| -> f64 {
        let mut context = HashMap::new();

        for (name, _) in &inputs {
            if let Some(value) = container.get_metric(name) {
                context.insert(name.clone(), value);
            }
        }

        // 평가 함수 호출 (예: meval, rhai, 또는 custom)
        parse_and_eval(&formula, &context).unwrap_or(0.0)
    }))
}
```

핵심 단계
- formula 문자열을 복사
- inputs에 정의된 metric 값을 ResultContainer에서 가져옴
- context에 변수 이름 → 값으로 매핑
- parse_and_eval() 함수에서 문자열을 실제 수식으로 평가

#### 🧪 parse_and_eval() 함수의 역할
이 함수는 다음을 수행합니다:
- 수식 문자열을 파싱 ("HIC15 * 0.5 + Age")
- 변수 이름을 context에서 치환
- 연산자 우선순위에 따라 계산
- 결과 f64 반환
예를 들어:
```rust
let formula = "HIC15 * 0.5 + Age";
let context = {
    "HIC15" => 100.0,
    "Age" => 25.0
};

let result = parse_and_eval(formula, context); // → 75.0
```


###  🧮 사용 가능한 함수 목록 (예: meval 기준)
```
+ - * / ^ ( )         // 기본 연산자
sqrt(x)               // 제곱근
ln(x), log(x)         // 로그
sin(x), cos(x), tan(x)// 삼각 함수
abs(x)                // 절댓값
min(a, b), max(a, b)  // 최소/최대
exp(x)                // 지수
floor(x), ceil(x)     // 내림/올림
```

### ⚠️ 함수 지원 여부는 사용하는 엔진에 따라 다릅니다.
Rhai를 쓰면 사용자 정의 함수, 조건문, 반복문까지도 가능해져요


## 📌 수식화되는 지점 요약

| 단계                        | 설명 또는 반환 타입                     |
|-----------------------------|----------------------------------------|
| ScenarioDefinition.formula  | 수식 문자열 (예: `"HIC15 * 0.5 + Age"`) |
| to_function()               | → `AnalysisScenario`로 변환 (실행 가능한 함수) |
| ResultContainer             | 입력 metric 값을 제공하는 컨테이너     |
| parse_and_eval()            | 수식 문자열을 파싱하고 평가하는 함수   |
|                             | 결과값 `f64` 반환                      |

meval::eval_str(&replaced)에 들어가기 전에 "HIC15 * 0.5 + Age" 같은 수식 문자열은
변수 치환을 통해 "100 * 0.5 + 25"처럼 숫자 기반 수식 문자열로 바뀌고,
그걸 meval이 실제로 계산해서 f64 결과를 반환하는 구조입니다.

## 🧠 전체 흐름 정리
### 1. 원래 수식 문자열
```rust
let formula = "HIC15 * 0.5 + Age";
```

### 2. 컨텍스트 맵 생성
```rust
let context = HashMap::from([
    ("HIC15".to_string(), 100.0),
    ("Age".to_string(), 25.0),
]);
```

### 3. 변수 치환
```rust
let replaced = replace_variables(&formula, &context);
// 결과: "100 * 0.5 + 25"

```
replace_variables()는 수식 문자열에서 "HIC15"와 "Age"를 찾아서
context에 있는 값으로 치환하는 함수입니다.

### 4. 수식 평가
let result = meval::eval_str(&replaced).unwrap();
// 결과: 75.0

---


## 🔧 replace_variables() 함수 예시
```rust
fn replace_variables(formula: &str, context: &HashMap<String, f64>) -> String {
    let mut replaced = formula.to_string();
    for (key, value) in context {
        let pattern = format!(r"\b{}\b", key); // 정확한 변수명만 치환
        let value_str = format!("{}", value);
        replaced = regex::Regex::new(&pattern)
            .unwrap()
            .replace_all(&replaced, value_str)
            .to_string();
    }
    replaced
}
```

이 함수는 regex를 사용해서 변수명을 정확히 찾아내고,
해당 값을 문자열로 치환합니다.


## ✅ 수식 평가 흐름 요약

| 단계                  | 설명 또는 값                          |
|-----------------------|---------------------------------------|
| 수식 문자열           | `"HIC15 * 0.5 + Age"`                 |
| 변수 컨텍스트         | `{ "HIC15": 100.0, "Age": 25.0 }`     |
| 치환된 수식 문자열    | `"100 * 0.5 + 25"`                    |
| 수식 평가             | `meval::eval_str("100 * 0.5 + 25")`   |
| 최종 결과             | `75.0`                                |


## 🧠 기본 DSL 수식 평가의 한계
- meval::eval_str("x^2 + 3*x") → 단순 계산은 가능
- 하지만 "d/dx(x^2)" 또는 "∫x dx" 같은 기호적 미분/적분은 지원하지 않음
- 이유: meval은 값을 계산하는 엔진이지, 함수 자체를 다루는 엔진은 아님

## 🧠 고급: 자동 미분 + DSL → Diffsol 라이브러리
- Diffsol은 Rust에서 ODE, DAE, 자동 미분, 적분을 처리할 수 있는 고급 라이브러리
- 자체 DSL을 사용해서 수식 정의 가능
- 내부적으로 LLVM 또는 Cranelift로 JIT 컴파일
- 적분, 민감도 분석, 이벤트 핸들링 등도 지원

## DSL 시스템에 적용하려면?

| 방식        | 미분/적분 지원 | 적용 위치 또는 방법                                 |
|-------------|----------------|-----------------------------------------------------|
| meval       | ❌ 지원 안 됨   | 단순 수식 평가만 가능 (`to_function()` 내부에서 사용) |
| 수치 계산   | ✅ 가능         | `to_function()` 내부에서 `finite_difference()` 또는 `trapezoidal_rule()` 호출 |
| Diffsol     | ✅ 고급 지원    | DSL 확장 또는 별도 엔진으로 연동 (ODE, 자동 미분, 적분 등) |

---


## 전체 코드
```rust


//FunctionRegister

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::core::calc_injury::{compute_hic15, compute_resultant};
use crate::core::data_container::DataContainer;
use crate::core::tarray::TArray;

pub enum FunctionType {
    Scalar(Box<dyn Fn(&[&TArray<f64>]) -> f64 + Send + Sync>),
    Vector(Box<dyn Fn(&[&TArray<f64>]) -> Vec<f64> + Send + Sync>),
}

pub struct FunctionRegister {
    registry: HashMap<String, FunctionType>,
}

impl FunctionRegister {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub fn register_scalar<F>(&mut self, name: &str, func: F)
    where
        F: Fn(&[&TArray<f64>]) -> f64 + Send + Sync +'static,
    {
        self.registry.insert(name.to_string(), FunctionType::Scalar(Box::new(func)));
    }

    pub fn register_vector<F>(&mut self, name: &str, func: F)
    where
        F: Fn(&[&TArray<f64>]) -> Vec<f64> + Send + Sync +'static,
    {
        self.registry.insert(name.to_string(), FunctionType::Vector(Box::new(func)));
    }

    pub fn run_scalar(&self, name: &str, args: &[&TArray<f64>]) -> Option<f64> {
        match self.registry.get(name)? {
            FunctionType::Scalar(f) => Some(f(args)),
            _ => None,
        }
    }

    pub fn run_vector(&self, name: &str, args: &[&TArray<f64>]) -> Option<Vec<f64>> {
        match self.registry.get(name)? {
            FunctionType::Vector(f) => Some(f(args)),
            _ => None,
        }
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.registry.contains_key(name)
    }

    pub fn get_function_names(&self) -> Vec<&String> {
        self.registry.keys().collect()
    }

    pub fn run_scalar_by_keys(
        &self,
        name: &str,
        data: &DataContainer,
        keys: &[&str],
    ) -> Option<f64> {

        let args: Vec<&TArray<f64>> = keys
            .iter()
            .filter_map(|key| data.get(key))
            .collect();

        if args.len() != keys.len() {
            return None; // 일부 키가 누락됨
        }
        self.run_scalar(name, &args)
    }

    pub fn run_vector_by_keys(
        &self,
        name: &str,
        data: &DataContainer,
        keys: &[&str],
    ) -> Option<Vec<f64>> {

        let args: Vec<&TArray<f64>> = keys
            .iter()
            .filter_map(|key| data.get(key))
            .collect();

        if args.len() != keys.len() {
            return None; // 일부 키가 누락됨
        }
        self.run_vector(name, &args)
    }
}

pub static FUNCTION_REGISTER: Lazy<Mutex<FunctionRegister>> = Lazy::new(|| {
    let mut reg = FunctionRegister::new();

    // 초기 등록 예시
    reg.register_scalar("HIC15", |arr| {
        let r = compute_resultant(arr);
        compute_hic15(arr)
    });

    reg.register_vector("HeadAcc", compute_resultant);
    Mutex::new(reg)
});



//ScenarioDefinition

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::result_container::ResultContainer;
use crate::core::scenario_register::{AnalysisProcedure, AnalysisScenario};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Metric,
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScenarioKind {
    Formula,
    Procedure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ScenarioDefinition {
    pub name: String,
    pub description: Option<String>,
    pub inputs: Vec<(String, InputType)>,
    pub formula: String,
    pub output: String,
    pub kind: ScenarioKind
}

fn get_procedure_by_name(name: &str) -> Option<AnalysisProcedure> {
    match name {
        "HeadInjury" => Some(Box::new(crate::core::calc_injury::calc_head_injury)),
        // 다른 절차형 시나리오도 여기에 추가
        _ => None,
    }
}




impl ScenarioDefinition {
    pub fn to_function(&self) -> Result<AnalysisScenario, String> {
        let formula = self.formula.clone();
        let inputs = self.inputs.clone();

        let func = move |container: &ResultContainer| -> f64 {
            let mut context = HashMap::new();

            for (key, input_type) in &inputs {
                match input_type {
                    InputType::Metric => {
                        if let Some(val) = container.get_metric(key) {
                            context.insert(key.clone(), val);
                        } else {
                            // 이건 런타임에서 발생하므로 여전히 panic 또는 fallback 필요
                            panic!("Missing metric: {}", key);
                        }
                    }
                    InputType::Channel => {
                        panic!("Channel input '{}' not yet supported in formula", key);
                    }
                }
            }

            parse_and_eval(&formula, &context)
        };

        Ok(Box::new(func))
    }

    pub fn to_procedure(&self) -> Result<AnalysisProcedure, String> {
        get_procedure_by_name(&self.name)
            .ok_or_else(|| format!("Unknown procedure scenario: {}", self.name))
    }


}

fn parse_and_eval(expr: &str, context: &HashMap<String, f64>) -> f64 {
    let replaced = context.iter().fold(expr.to_string(), |acc, (k, v)| {
        acc.replace(k, &v.to_string())
    });
    meval::eval_str(&replaced).expect("Failed to evaluate expression")
}



// ScenarioRegister

use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::core::calc_injury::calc_head_injury;
use crate::core::data_container::DataContainer;
use crate::core::result_container::ResultContainer;

pub type AnalysisScenario = Box<dyn for<'a> Fn(&'a ResultContainer) -> f64 + Send + Sync>;
pub type AnalysisProcedure = Box<dyn Fn(&mut ResultContainer, &DataContainer) + Send + Sync>;



pub struct ScenarioRegister {
    scenarios: HashMap<String, AnalysisScenario>,
    procedures: HashMap<String, AnalysisProcedure>,

}

impl ScenarioRegister {
    pub fn new() -> Self {
        Self { scenarios: HashMap::new(), procedures: Default::default() }
    }

    pub fn register(&mut self, name: &str, func: AnalysisScenario) {
        self.scenarios.insert(name.to_string(), func);
    }

    pub fn run_scenario(&self, name: &str, container: &ResultContainer) -> Option<f64> {
        self.scenarios.get(name).map(|func| func(container))
    }



    pub fn has(&self, name: &str) -> bool {
        self.scenarios.contains_key(name)
    }

    pub fn list(&self) -> Vec<&String> {
        self.scenarios.keys().collect()
    }

    pub fn register_procedure(&mut self, name: &str, proc: AnalysisProcedure) {
        self.procedures.insert(name.to_string(), proc);
    }

    pub fn run_procedure(&self, name: &str, result: &mut ResultContainer, data: &DataContainer) -> bool {
        if let Some(proc) = self.procedures.get(name) {
            proc(result, data);
            true
        } else {
            false
        }
    }



}

pub static SCENARIO_REGISTER: Lazy<Mutex<ScenarioRegister>> = Lazy::new(|| {
    let mut reg = ScenarioRegister::new();
    reg.register_procedure("HeadInjury",  Box::new(calc_head_injury));

    Mutex::new(reg)
});


//ScenarioLoader

use std::fs;
use crate::core::scenario_definition::ScenarioDefinition;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load_all_from(dir: &str) -> Vec<ScenarioDefinition> {
        let mut scenarios = Vec::new();
        for entry in fs::read_dir(dir).expect("Failed to read scenario directory") {
            let path = entry.unwrap().path();
            if path.extension().map(|ext| ext == "json").unwrap_or(false) {
                let content = fs::read_to_string(&path).expect("Failed to read file");
                let def: ScenarioDefinition = serde_json::from_str(&content)
                    .expect("Failed to parse scenario JSON");
                scenarios.push(def);
            }
        }
        scenarios
    }
}
```

// Scenarios.json

```
[
  {
    "name": "HIC_Adjusted",
    "description": "보정된 HIC",
    "inputs": [
      ["HIC15", "Metric"],
      ["Age", "Metric"]
    ],
    "formula": "HIC15 * 0.5 + Age",
    "output": "HIC_Adjusted",
    "kind": "Formula"
  },
  {
    "name": "HeadInjury",
    "description": "HIC 계산",
    "inputs": [],
    "formula": "",
    "output": "HIC",
    "kind": "Procedure"
  }
]



```

## Test 

```rust
use std::error::Error;
use geometry::core::calc_injury::compute_resultant;
use geometry::core::csv_dataloader::CsvDataLoader;
use geometry::core::data_container::DataContainer;
use geometry::core::tarray::TArray;
use geometry::utils::filter::exec_sae_filter;

fn read_file(path : String) -> Result<DataContainer, Box<dyn Error>> {
    let loader = CsvDataLoader::from_path(path.as_str())?;
    let mut container = DataContainer::new();

    for i in 0..loader.header_count() {
        let header = loader.get_header_by_index(i).unwrap();
        let raw = loader.get_column(header).unwrap();
        let src = TArray::from(raw.clone());
        let mut tgt = TArray::from(vec![]);

        exec_sae_filter(&src, &mut tgt, 0.0001, 300.0);
        container.insert(header.clone(), tgt);
    }

    // 🔧 전처리: 모든 채널에 스케일과 오프셋 적용 + 구간 슬라이스
    container.apply_to_all_channels(|data| {
        data.scale(9.81);     // 단위 변환
        data.offset(-0.5);    // 센서 기준점 보정
        data.slice(100, 300); // 시간 구간 추출
    });

    // 📈 상해치 계산: 3축 합성 가속도
    let accel_axes = ["13HEADCG00H3ACXA", "13HEADCG00H3ACYA", "13HEADCG00H3ACZA"];
    if let Some(resultant) = container.compute_injury_metric(&accel_axes, compute_resultant) {
        println!("Resultant: {:?}", resultant);
    } else {
        println!("Resultant 계산 실패: 필요한 채널이 없습니다.");
    }

    Ok(container)
}

#[cfg(test)]
mod tests {
    use geometry::core::data_container::DataContainer;
    use geometry::core::function_register::FUNCTION_REGISTER;
    use geometry::core::tarray::TArray;

    #[test]
    fn test_hic15_function_execution() {
        // 샘플 데이터 생성

        // 샘플 TArray 생성
        let accel_x = TArray::from_vec(vec![1.0, 2.0, 3.0]);
        let accel_y = TArray::from_vec(vec![0.0, 0.0, 0.0]);
        let accel_z = TArray::from_vec(vec![0.0, 0.0, 0.0]);

        // args 배열 구성
        let args = vec![&accel_x, &accel_y, &accel_z];

        // 함수 실행
        let reg = FUNCTION_REGISTER.lock().unwrap();
        let result = reg.run_scalar("HIC15", &args);
        // 결과 확인
        assert!(result.is_some());
        println!("HIC15 result: {:?}", result.unwrap());

    }

    fn test_hic15_function_execution2() {

        let accel_x = TArray::from_vec(vec![1.0, 2.0, 3.0]);
        let accel_y = TArray::from_vec(vec![0.0, 0.0, 0.0]);
        let accel_z = TArray::from_vec(vec![0.0, 0.0, 0.0]);

        let mut data = DataContainer::new();
        data.insert("AccelX".to_string(), accel_x);
        data.insert("AccelY".to_string(), accel_y);
        data.insert("AccelZ".to_string(), accel_z);

        let result = FUNCTION_REGISTER
            .lock()
            .unwrap()
            .run_scalar_by_keys("HIC15", &data, &["AccelX", "AccelY", "AccelZ"]);

        println!("HIC15 result: {:?}", result);
    }
}

use geometry::core::result_container::ResultContainer;
use geometry::core::scenario_definition::{InputType, ScenarioDefinition, ScenarioKind};

fn metric(key: &str) -> (String, InputType) {
    (key.into(), InputType::Metric)
}

#[test]
fn scenario_definition_test() {
    let def = ScenarioDefinition {
        name: "HIC_Adjusted".into(),
        description: Some("보정된 HIC".into()),
        inputs: vec![
            ("HIC15".into(), InputType::Metric),
            ("Age".into(), InputType::Metric),
        ],
        formula: "HIC15 * 0.5 + Age".into(),
        output: "HIC_Adjusted".into(),
        kind: ScenarioKind::Formula,
    };

    let func = def.to_function();
    let mut container = ResultContainer::new();
    container.insert_metric("HIC15", 100.0);
    container.insert_metric("Age", 25.0);
}


use geometry::core::scenario_definition::ScenarioKind;
use geometry::core::scenario_loader::ScenarioLoader;
use geometry::core::scenario_register::{ScenarioRegister, SCENARIO_REGISTER};

#[test]
fn scenario_load_test() {
    let definitions = ScenarioLoader::load_all_from("scenarios/");

    if let Ok(mut reg) = SCENARIO_REGISTER.lock() {
        for def in definitions {
            match def.to_function() {
                Ok(func) => {
                    reg.register(&def.output, func); // ✅ 여기서 &str 넘김
                }
                Err(e) => {
                    eprintln!("Failed to compile scenario '{}': {}", def.name, e);
                }
            }
        }
    }
}


fn scenario_load_test2() {
    let definitions = ScenarioLoader::load_all_from("scenarios/");

    if let Ok(mut reg) = SCENARIO_REGISTER.lock() {
        for def in definitions {
            match def.kind {
                ScenarioKind::Formula => {
                    match def.to_function() {
                        Ok(func) => reg.register(&def.output, func),
                        Err(e) => eprintln!("Failed to compile formula '{}': {}", def.name, e),
                    }
                }
                ScenarioKind::Procedure => {
                    match def.to_procedure() {
                        Ok(proc) => reg.register_procedure(&def.output, proc),
                        Err(e) => eprintln!("Failed to compile procedure '{}': {}", def.name, e),
                    }
                }
            }
        }
    }

}
```