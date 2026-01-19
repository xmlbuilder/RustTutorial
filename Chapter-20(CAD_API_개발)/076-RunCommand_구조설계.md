# RunCommand
지금까지 만든 RunCommand 구조를 전체적으로 정리.  
현재는 Mesh만 결과물로 가정했지만, 나중에 Surface, Cad 등으로 확장할 수 있도록 결과 전달 구조까지 포함해서 설명하겠습니다.

## 🧩 RunCommand 기본 구조
```rust
pub enum RunResult {
    Mesh(Mesh),
    // 앞으로 Surface, Cad 등 추가 가능
}

pub trait RunCommand {
    fn run(&mut self) -> anyhow::Result<RunResult>;
}
```

- RunResult: 실행 결과를 담는 공통 enum
- RunCommand: 모든 커맨드가 구현해야 하는 인터페이스

## 📦 주요 커맨드 구현체
| 커맨드            | 설명                | 결과            |
|-------------------|---------------------|-----------------|
| `BdfImportCommand`  | BDF 파일을 읽어 Mesh 생성 | RunResult::Mesh |
| `BdfExportCommand`  | Mesh를 BDF 파일로 저장    | RunResult::Mesh |
| `StlImportCommand`  | STL 파일을 읽어 Mesh 생성 | RunResult::Mesh |
| `StlExportCommand`  | Mesh를 STL 파일로 저장    | RunResult::Mesh |
| `MergeCommand`      | 여러 Mesh를 병합          | RunResult::Mesh |


## 🔗 CommandManager
여러 커맨드를 등록하고 독립적으로 실행하는 매니저.
```rust
pub struct CommandManager {
    commands: Vec<Box<dyn RunCommand>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn add_command(&mut self, cmd: Box<dyn RunCommand>) {
        self.commands.push(cmd);
    }

    pub fn run_all(&mut self) -> Vec<anyhow::Result<RunResult>> {
        self.commands.iter_mut().map(|cmd| cmd.run()).collect()
    }
}
```
- 단순히 커맨드를 모아 실행 → 결과는 Vec<RunResult>로 관리
- 결과 전달은 아직 없음 → 나중에 run_all_with_pipeline() 같은 확장 가능

## 🔀 Pipeline
앞 단계 결과를 다음 단계로 연결하는 구조.
```rust
pub struct Pipeline {
    stages: Vec<Box<dyn RunCommand>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_stage(&mut self, cmd: Box<dyn RunCommand>) {
        self.stages.push(cmd);
    }

    pub fn execute(&mut self) -> anyhow::Result<RunResult> {
        let mut current: Option<RunResult> = None;
        for stage in &mut self.stages {
            current = Some(stage.run()?);
        }
        current.ok_or_else(|| anyhow::anyhow!("Pipeline produced no result"))
    }
}
```

- 현재는 단일 결과만 전달
- 나중에 MergeCommand 같은 다중 입력이 필요하면 Vec<RunResult>를 전달하는 구조로 확장 가능

## 📑 전체 흐름 요약
- RunResult: 결과물 통합 관리 (Mesh → Surface → Cad 등 확장 가능)
- RunCommand: 모든 커맨드가 동일한 인터페이스로 실행
- CommandManager: 여러 커맨드를 독립적으로 실행하고 결과 모음
- Pipeline: 커맨드를 순차적으로 연결해 결과 전달
- 확장 포인트:
- 결과 전달 방식 → Option<RunResult> → Vec<RunResult>로 확장
- 다양한 결과 타입 추가 → RunResult::Surface, RunResult::Cad 등

## 🎯 결론
- 지금은 Mesh만 다루지만, 구조적으로 이미 확장 가능하게 설계되어 있음
- CommandManager는 독립 실행 관리, Pipeline은 결과 전달 연결
- 나중에 결과 전달을 강화하면 Import → Merge → Export 같은 워크플로우를 완벽하게 구성할 수 있음

---



## 🛠 구조
```rust
#[derive(Debug, Clone)]
pub enum RunResult {
    Mesh(Mesh),
    Cad(Cad),
    Surface(Surface),
}

pub trait RunCommand {
    fn run(&mut self) -> anyhow::Result<RunResult>;
}
```


## 📦 예제 구현
```rust
pub struct BdfExportCommand {
    pub mesh: Mesh,
    pub path: String,
}

impl RunCommand for BdfExportCommand {
    fn run(&mut self) -> anyhow::Result<RunResult> {
        let mut writer = BdfWriter::new(&self.path)?;
        writer.run(&[self.mesh.clone()])?;
        Ok(RunResult::Mesh(self.mesh.clone()))
    }
}
```


## 🎯 CommandManager 예시
```rust
pub struct CommandManager {
    commands: Vec<Box<dyn RunCommand>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn add_command(&mut self, cmd: Box<dyn RunCommand>) {
        self.commands.push(cmd);
    }

    pub fn run_all(&mut self) -> Vec<anyhow::Result<RunResult>> {
        self.commands.iter_mut().map(|cmd| cmd.run()).collect()
    }
}
```


## ✅ 장점
- RunResult로 통일하면 다양한 결과물을 한 타입으로 관리 가능
- CommandManager에서 여러 커맨드를 모아 실행하고 결과를 한꺼번에 처리할 수 있음
- 이후 match 문으로 결과를 분기 처리하면 됩니다:
```rust
for result in manager.run_all() {
    match result {
        Ok(RunResult::Mesh(m)) => println!("Got Mesh {:?}", m),
        Ok(RunResult::Cad(c)) => println!("Got CAD {:?}", c),
        Ok(RunResult::Surface(s)) => println!("Got Surface {:?}", s),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```
```rust
/// 파이프라인 매니저
pub struct Pipeline {
    stages: Vec<Box<dyn RunCommand>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_stage(&mut self, cmd: Box<dyn RunCommand>) {
        self.stages.push(cmd);
    }

    pub fn execute(&mut self) -> anyhow::Result<RunResult> {
        let mut current: Option<RunResult> = None;
        for stage in &mut self.stages {
            current = Some(stage.run(current)?);
        }
        current.ok_or_else(|| anyhow::anyhow!("Pipeline produced no result"))
    }
}
```

---

# 전 단계 결과를 모아서 다음 단계로 전달 

##  해결 방향
- RunCommand 인터페이스 확장
    - 현재는 `fn run(&mut self) -> Result<RunResult>` 로 고정되어 있어서 입력을 받을 수 없음
    - `fn run(&mut self, input: Option<RunResult>) -> Result<RunResult>` 처럼 입력을 받을 수 있도록 바꾸면 파이프라인처럼 연결 가능

- Pipeline 설계
    - 각 단계의 결과를 current에 저장하고, 다음 단계에 전달
    - MergeCommand는 여러 입력을 받아야 하므로, 이전 단계들의 결과를 모아 전달하는 방식 필요

## ✅ 설계 예시
- RunCommand trait 수정
```rust
pub trait RunCommand {
    fn run(&mut self, input: Option<RunResult>) -> anyhow::Result<RunResult>;
}
```

- MergeCommand 수정
```rust
pub struct MergeCommand {
    pub eps: f64,
}

impl RunCommand for MergeCommand {
    fn run(&mut self, input: Option<RunResult>) -> anyhow::Result<RunResult> {
        // input이 여러 개일 수 있으니, Pipeline에서 Vec<RunResult>를 모아 전달
        if let Some(RunResult::Mesh(mesh)) = input {
            // 단일 입력이면 그대로 반환
            Ok(RunResult::Mesh(mesh))
        } else {
            Err(anyhow::anyhow!("MergeCommand requires Mesh input"))
        }
    }
}
```

- Pipeline에서 여러 결과 모아 전달
```rust
pub struct Pipeline {
    stages: Vec<Box<dyn RunCommand>>,
}
```
```rust
impl Pipeline {
    pub fn execute(&mut self) -> anyhow::Result<RunResult> {
        let mut results: Vec<RunResult> = Vec::new();

        for stage in &mut self.stages {
            let input = results.last().cloned(); // 직전 결과를 입력으로 전달
            let output = stage.run(input)?;
            results.push(output);
        }

        // 마지막 결과 반환
        results.last().cloned().ok_or_else(|| anyhow::anyhow!("Pipeline produced no result"))
    }
}
```


## 🎯 MergeCommand에 여러 입력 전달하기
만약 MergeCommand가 여러 Mesh를 합쳐야 한다면, Pipeline이 직전 결과 하나만 넘기는 구조로는 부족합니다.  
이럴 때는 특정 커맨드만 여러 결과를 받을 수 있도록 설계해야 합니다:  
```rust
impl MergeCommand {
    pub fn run_multiple(&mut self, inputs: Vec<RunResult>) -> anyhow::Result<RunResult> {
        let meshes: Vec<Mesh> = inputs.into_iter().filter_map(|r| {
            if let RunResult::Mesh(m) = r { Some(m) } else { None }
        }).collect();

        if meshes.is_empty() {
            return Ok(RunResult::Mesh(Mesh::new(vec![], vec![])));
        }

        let mut merged = meshes[0].clone();
        for mesh in meshes.iter().skip(1) {
            merged = on_merge_meshes(&merged, mesh, self.eps);
        }
        Ok(RunResult::Mesh(merged))
    }
}
```

## ✍️ 결론
- RunCommand를 입력을 받을 수 있게 바꾸고, Pipeline이 결과를 다음 커맨드에 전달하도록 설계해야 합니다.
- MergeCommand처럼 여러 입력이 필요한 경우는 Pipeline이 앞 단계 결과들을 모아 run_multiple로 전달하는 방식이 적합합니다.

---

