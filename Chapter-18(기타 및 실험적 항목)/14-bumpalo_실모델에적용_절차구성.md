# bumpalo

## 🧱 목표: bumpalo로 3D 배열을 Chunk 기반으로 할당

- bumpalo::Bump을 사용해 풀에서 메모리 할당
- 3D 배열을 단일 Vec 형태로 flatten해서 관리
- i, j, k → 1D 인덱스로 변환하여 접근

## ✅ 실전 예제: 3D 배열 할당 + 접근
```rust
use bumpalo::Bump;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;
const DEPTH: usize = 128;

fn main() {
    // 메모리 풀 생성
    let bump = Bump::new();

    // 3D 배열을 1D Vec으로 flatten해서 할당
    let volume = bump.alloc_slice_fill_with(WIDTH * HEIGHT * DEPTH, || 0.0f32);

    // 접근 함수: (i, j, k) → 1D 인덱스
    let index = |i: usize, j: usize, k: usize| -> usize {
        i * HEIGHT * DEPTH + j * DEPTH + k
    };

    // 예시: 값 설정
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            for k in 0..DEPTH {
                let idx = index(i, j, k);
                volume[idx] = (i + j + k) as f32;
            }
        }
    }

    // 예시: 값 읽기
    let sample = volume[index(10, 20, 30)];
    println!("(10,20,30)의 값: {}", sample);
}
```


## 🧠 실전 팁: Marching Cubes에 적용할 때
- volume은 scalar field (예: density, SDF 등)로 사용
- bumpalo는 할당 속도가 매우 빠르고 해제는 전체 한 번에 가능
- alloc_slice_fill_with은 초기화까지 해주므로 안전
- 필요 시 bump.reset()으로 전체 해제 가능

## 🔧 확장 아이디어
| 기능                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| bumpalo::collections::Vec | bump 풀 기반 Vec → 동적 크기 배열을 풀에서 할당 가능, 반복적 할당에 유리 |
| bumpalo::boxed::Box       | bump 풀 기반 Box → 고정 크기 배열을 안전하게 할당할 때 사용             |
| bump.reset()              | 전체 풀 메모리 초기화 → free_all처럼 한 번에 해제 가능                  |
| Arc<[T]>                  | 병렬 처리 시 공유 배열로 사용 → 스레드 간 안전한 참조 가능              |



## 🧠 실전 팁
- bumpalo::collections::Vec은 반복적으로 할당되는 vertex, edge 리스트에 적합
- Box<[T]>는 고정 크기 scalar field, voxel grid 등에 적합
- bump.reset()은 한 프레임 단위로 전체 메모리 초기화할 때 유용
- Arc<[T]>는 멀티스레드 Marching Cubes에서 각 스레드가 동일한 scalar field를 참조할 때 사용


## Cargo.toml에 아래처럼 의존성을 추가

```
[dependencies]
bumpalo = "3.14"
```
---

# 구조체로 연결
Marching Cubes처럼 복합적인 3D 데이터를 다룰 때는  
단순 배열보다 구조체로 잡는 게 훨씬 명확하고 유지보수도 쉬움.  
특히 scalar field와 vector field를 동시에 다뤄야 할 때는 구조체로 묶는 게 정답입니다.  

## 🧱 예제: bumpalo로 구조체 기반 3D Volume 잡기
```rust
use bumpalo::Bump;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;
const DEPTH: usize = 128;

#[derive(Debug)]

struct Volume<'a> {
    scalar: &'a mut [f32], // density or SDF
    vector: &'a mut [(f32, f32, f32)], // gradient or velocity
}

fn main() {
    let bump = Bump::new();

    let scalar = bump.alloc_slice_fill_with(WIDTH * HEIGHT * DEPTH, || 0.0f32);
    let vector = bump.alloc_slice_fill_with(WIDTH * HEIGHT * DEPTH, || (0.0f32, 0.0f32, 0.0f32));

    let volume = Volume { scalar, vector };

    // 인덱스 계산 함수
    let index = |i: usize, j: usize, k: usize| -> usize {
        i * HEIGHT * DEPTH + j * DEPTH + k
    };

    // 예시: scalar와 vector 값 설정
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            for k in 0..DEPTH {
                let idx = index(i, j, k);
                volume.scalar[idx] = (i + j + k) as f32;
                volume.vector[idx] = (i as f32, j as f32, k as f32);
            }
        }
    }

    // 예시 출력
    let idx = index(10, 20, 30);
    println!("Scalar @ (10,20,30): {}", volume.scalar[idx]);
    println!("Vector @ (10,20,30): {:?}", volume.vector[idx]);
}

```

## 🔧 확장 아이디어
| 필드                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| scalar: &[f32]           | 밀도, 거리 함수(SDF), 온도 등 scalar field → Marching Cubes의 기준값 |
| vector: &[(f32, f32, f32)] | gradient, normal, velocity 등 → 시각화나 물리 해석에 사용             |
| material: &[u8]          | voxel의 재질 ID → 렌더링 시 색상/질감 결정에 활용                     |
| mask: &[bool]            | 활성/비활성 여부 → 계산 대상 필터링, sparse field 처리에 유용         |


---

# 내부 구조

HashMap<(i, j, k), T> 같은 구조는 sparse field에서 빠르게 찾아가는 실전적인 해결책

## 🔍 왜 map이 부적이 되는가?
- 빠른 탐색
- HashMap은 평균 O(1) 접근 → i, j, k 좌표 기반으로 빠르게 찾음
- sparse 구조 대응
- 전체 3D 배열을 만들지 않고  
    → 필요한 좌표만 저장 → 메모리 절약
- 제어권 확보
- 내가 넣고, 내가 지우고, 내가 해제 시점 결정  
    → bumpalo나 시스템에 의탁하지 않고 직접 관리
- 실전에서 살아남는 구조
- 할당 실패, 조각화, 캐시 미스 등  
    → map 기반 구조는 유연하게 대응 가능

## ✅ 실전 예시: sparse volume with HashMap
```rust
use std::collections::HashMap;

type Coord = (usize, usize, usize);

fn main() {
    let mut volume: HashMap<Coord, f32> = HashMap::new();

    // 값 설정
    volume.insert((10, 20, 30), 0.85);
    volume.insert((50, 60, 70), 1.0);

    // 값 조회
    if let Some(val) = volume.get(&(10, 20, 30)) {
        println!("(10,20,30)의 값: {}", val);
    }

    // 값 제거
    volume.remove(&(50, 60, 70));
}
```

---
# Wrapper로 구조 감싸기

사용자가 Voxel만 넣고 Triangle만 빼가면 되도록 내부에서 모든 걸 연동하는 간단한 Wrapper 구조를 하나 만듬.
Marching Cubes 기반으로 iso surface를 추출하는 구조.

## 🧱 구조체: Voxel, Triangle, VolumeMesh
```rust
#[derive(Clone, Copy, Default)]
pub struct Voxel {
    pub scalar: f32,
    pub vector: (f32, f32, f32),
    pub material: u8,
    pub mask: bool,
}

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [(f32, f32, f32); 3],
}

pub struct VolumeMesh {
    pub voxels: Vec<Voxel>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
```


## ⚙️ 핵심 메서드: extract_surface()
```rust
impl VolumeMesh {
    pub fn extract_surface(&self, iso: f32) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let index = |x, y, z| -> usize {
            x * self.height * self.depth + y * self.depth + z
        };

        for x in 0..self.width - 1 {
            for y in 0..self.height - 1 {
                for z in 0..self.depth - 1 {
                    let mut cube = [0.0f32; 8];
                    for i in 0..8 {
                        let (dx, dy, dz) = cube_offset(i);
                        let idx = index(x + dx, y + dy, z + dz);
                        cube[i] = self.voxels[idx].scalar;
                    }

                    let tris = marching_cubes_cell(&cube, iso, (x, y, z));
                    triangles.extend(tris);
                }
            }
        }

        triangles
    }
}
```


## 🧩 보조 함수: cube_offset, marching_cubes_cell
```rust
fn cube_offset(i: usize) -> (usize, usize, usize) {
    match i {
        0 => (0, 0, 0),
        1 => (1, 0, 0),
        2 => (1, 1, 0),
        3 => (0, 1, 0),
        4 => (0, 0, 1),
        5 => (1, 0, 1),
        6 => (1, 1, 1),
        7 => (0, 1, 1),
        _ => (0, 0, 0),
    }
}

fn marching_cubes_cell(scalars: &[f32; 8], iso: f32, origin: (usize, usize, usize)) -> Vec<Triangle> {
    // 여기에 edge table, tri table, vertex interpolation 넣으면 됨
    // 지금은 빈 벡터 반환
    Vec::new()
}
```

## ✅ 사용 예시
```rust
let mut volume = VolumeMesh {
    voxels: vec![Voxel::default(); WIDTH * HEIGHT * DEPTH],
    width: WIDTH,
    height: HEIGHT,
    depth: DEPTH,
};

volume.voxels[index(10, 20, 30)].scalar = 1.0;

let triangles = volume.extract_surface(0.5);
```

