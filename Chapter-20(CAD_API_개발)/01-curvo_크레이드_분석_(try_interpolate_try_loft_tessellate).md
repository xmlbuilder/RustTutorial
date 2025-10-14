# curvo 크레이드 분석 (try_interpolate / try_loft / tessellate)

curvo 크레이트에서 가장 배울 만한 함수 3가지는 try_interpolate, try_loft, tessellate입니다.  
이 함수들은 각각 NURBS 곡선 생성, 서페이스 생성, 렌더링을 위한 삼각형 분할을 담당하며,  
수학적 로직과 Rust 스타일이 잘 녹아 있습니다.

## 1️⃣ try_interpolate: NURBS 곡선 보간 생성
### 📌 목적
주어진 3D 점들을 통과하는 NURBS 곡선을 생성합니다.

### 🧩 핵심 로직
- 입력: &[Point3<T>] (제어점), degree: usize
- 내부에서 노트 벡터 생성, 가중치 설정, B-스플라인 보간 수행
- Result<Self, Error> 반환 → 실패 가능성 명확히 표현

### 🧠 배울 점
- 제네릭 수치 타입 (T: Float)을 활용한 수학적 추상화
- nalgebra와의 연동으로 공간 좌표 처리
- 보간 알고리즘을 함수형 스타일로 표현 (iter().zip().map() 등)

## 2️⃣ try_loft: 곡선 배열로부터 서페이스 생성
### 📌 목적
여러 NURBS 곡선을 연결해 NURBS 서페이스를 생성합니다.

### 🧩 핵심 로직
- 입력: &[NurbsCurve3D<T>], degree: Option<usize>
- 각 곡선의 제어점을 정렬 → 2D 제어점 배열 생성
- NurbsSurface::new()로 서페이스 구성
  
### 🧠 배울 점
- 곡선 간 정렬과 일관성 유지 (곡선 수, 제어점 수 체크)
- 옵션 처리 (Option<usize>)를 통해 유연한 API 설계
- 불변성 유지: 모든 입력은 참조로 받고, 내부에서 새 구조 생성

## 3️⃣ tessellate: 서페이스를 삼각형으로 분할
### 📌 목적
NURBS 서페이스를 렌더링 가능한 삼각형 mesh로 변환합니다.

### 🧩 핵심 로직
- 입력: Option<usize> (세분화 정도)
- u, v 파라메트릭 공간을 균등하게 샘플링
- 각 샘플 점을 evaluate(u, v)로 계산 → 삼각형 구성

### 🧠 배울 점
- 파라메트릭 공간 샘플링: linspace, grid, zip 활용
- 삼각형 mesh 구성: 인덱스 계산, vertex 연결 방식
- 성능 최적화: 반복자 기반 처리로 메모리 효율 확보

## ✅ 요약
| 함수             | 기능 설명                          | 주요 입력                          | 배울 점 요약                                 |
|------------------|-------------------------------------|-------------------------------------|----------------------------------------------|
| try_interpolate  | 주어진 점들을 통과하는 NURBS 곡선 생성 | `&[Point3<T>]`, `degree: usize`     | 보간 알고리즘, 제네릭 수치 타입, 에러 처리  |
| try_loft         | 여러 곡선을 연결해 NURBS 서페이스 생성 | `&[NurbsCurve3D<T>]`, `Option<usize>` | 구조 정렬, API 유연성, 불변성 유지           |
| tessellate       | 서페이스를 삼각형 mesh로 분할         | `Option<usize>` (세분화 정도)       | 파라메트릭 샘플링, 렌더링 최적화, 반복자 활용 |

---

# try_interpolate  분석
curvo의 try_interpolate 함수는 NURBS 곡선을 보간하는 핵심 함수입니다.  
아래에 이 함수의 구조와 수학적 흐름을 단계적으로 설명.

## 🔍 1단계: 함수 시그니처
```rust
pub fn try_interpolate(points: &[Point3<T>], degree: usize) -> Result<Self, Error>
where
    T: Float,
```
- 입력: 3D 점 배열 (Point3<T>), 곡선 차수 (degree)
- 반환: Result<NurbsCurve3D<T>, Error> — 실패 가능성 포함
- 제네릭 타입 T는 Float trait을 요구 → f32, f64 등 사용 가능

## 🔧 2단계: 노트 벡터 생성
```rust
let knots = KnotVector::generate_interpolation(points.len(), degree)?;
```
- NURBS 보간을 위해 노트 벡터 생성
- generate_interpolation(n, p)는 보간에 적합한 open uniform knot vector를 생성
- 예: n = 4, p = 3이면 [0, 0, 0, 0, 1, 1, 1, 1]

## 🧮 3단계: 선형 시스템 구성
```rust
let system = LinearSystem::from_interpolation(points, &knots, degree)?;
```

- 각 점을 통과하는 NURBS 곡선을 만들기 위해 선형 시스템 구성
- 이 시스템은 B-스플라인 basis function을 기반으로 만들어짐
- 내부적으로 De Boor 알고리즘 또는 basis matrix를 활용

## 🧩 4단계: 제어점 계산
```rust
let control_points = system.solve()?;
```

- 선형 시스템을 풀어서 제어점(control points) 계산
- 이 제어점들이 실제 NURBS 곡선을 정의함
- solve()는 Gaussian elimination 또는 LU decomposition 기반일 수 있음

## 🧷 5단계: 곡선 객체 생성
```rust
Ok(NurbsCurve3D {
    control_points,
    knots,
    weights: vec![T::one(); control_points.len()],
    degree,
})
```

- 계산된 제어점과 노트 벡터, 기본 가중치(전부 1.0)를 사용해 곡선 생성
- weights는 기본적으로 균등하게 설정됨 → NURBS가 B-스플라인과 동일하게 작동

## ✅ 요약 흐름
입력 점들 → 노트 벡터 생성 → 선형 시스템 구성 → 제어점 계산 → NURBS 곡선 생성

## ✨ 배울 점
- 수학적 보간 알고리즘을 Rust로 표현하는 방식
- 에러 처리와 제네릭 타입 활용
- 불변성 유지와 참조 기반 설계
- 구조적 API 설계 (try_ prefix, Result 반환)

---

## 1️⃣ generate_interpolation: 노트 벡터 생성
### 📌 목적
주어진 점 개수와 곡선 차수에 맞는 open uniform knot vector를 생성합니다.
### 🧩 단계별 흐름
pub fn generate_interpolation(n: usize, degree: usize) -> Result<KnotVector<T>, Error>

- n: 제어점 개수
- degree: 곡선 차수 (예: 3차 → cubic)
#### 🔹 Step 1: 총 노트 개수 계산
```rust
let m = n + degree + 1;
```

- NURBS에서 노트 벡터 길이는 n + p + 1
#### 🔹 Step 2: 노트 벡터 초기화
```rust
let mut knots = vec![T::zero(); m];
```

- T는 Float trait을 만족하는 제네릭 타입 (f64, f32 등)
#### 🔹 Step 3: 시작과 끝 노트 고정
```rust
for i in 0..=degree {
    knots[i] = T::zero();
    knots[m - 1 - i] = T::one();
}
```
- 시작과 끝은 각각 0과 1로 고정 → open uniform 구조
#### 🔹 Step 4: 내부 노트 균등 분포
```rust
let step = T::one() / T::from_usize(n - degree).unwrap();
for i in (degree + 1)..(m - degree - 1) {
    knots[i] = T::from_usize(i - degree).unwrap() * step;
}
```
- 내부 노트는 균등하게 분포됨 → 보간 안정성 확보

## 2️⃣ LinearSystem::solve(): 제어점 계산
### 📌 목적
보간 조건을 만족하는 제어점을 계산하기 위해 선형 시스템을 풀어냅니다.
### 🧩 단계별 흐름
```rust
pub fn solve(&self) -> Result<Vec<Point3<T>>, Error>
```

#### 🔹 Step 1: 시스템 행렬 구성
- A 행렬: basis function 값으로 구성 (A[i][j] = N_j(t_i))
- b 벡터: 입력 점들 (Point3<T>)
#### 🔹 Step 2: LU 분해 또는 Gaussian elimination
- 내부적으로 nalgebra 또는 ndarray 기반으로 선형 시스템 풀이
- 예: A.solve_into(b) 또는 A.lu().solve(b)
#### 🔹 Step 3: 결과 벡터 반환
- 계산된 x 벡터가 제어점 역할을 함
- Vec<Point3<T>> 형태로 반환

## ✅ 요약 흐름
| 함수                    | 역할                          | 주요 입력                        | 핵심 처리 방식 및 배울 점                         |
|-------------------------|-------------------------------|----------------------------------|--------------------------------------------------|
| generate_interpolation  | NURBS 보간용 노트 벡터 생성     | `n: usize`, `degree: usize`      | open uniform 방식, 균등 분포, 제네릭 수치 타입 활용 |
| LinearSystem::solve()   | 보간 조건 만족하는 제어점 계산 | 내부 basis 행렬 + 점 벡터        | 선형 시스템 구성, LU 분해 또는 Gaussian elimination |

---

# try_loft 

이번엔 curvo 크레이트의 핵심 함수 중 하나인 try_loft를 단계적으로 분석.  
이 함수는 여러 NURBS 곡선을 연결해서 하나의 NURBS 서페이스를 만드는 기능을 담당합니다.

## 1️⃣ 함수 시그니처
```rust
pub fn try_loft(curves: &[NurbsCurve3D<T>], degree: Option<usize>) -> Result<Self, Error>
where
    T: Float,
```

- 입력:
- curves: NURBS 곡선 배열
- degree: 서페이스의 V 방향 차수 (옵션)
- 반환: Result<NurbsSurface<T>, Error>

## 2️⃣ 입력 검증
```rust
if curves.len() < 2 {
    return Err(Error::TooFewCurves);
}
```

- 최소 2개 이상의 곡선이 있어야 loft 가능
- 각 곡선의 제어점 수가 동일해야 함 → 내부에서 체크

## 3️⃣ 제어점 배열 구성
```rust
let control_points: Vec<Vec<Point3<T>>> = curves
    .iter()
    .map(|c| c.control_points.clone())
    .collect();
```

- 각 곡선의 제어점을 추출해서 2D 배열로 구성
- 결과적으로 control_points[u][v] 형태의 서페이스 제어점 배열 생성

## 4️⃣ 노트 벡터 생성
- U 방향은 곡선의 제어점 수와 차수 기반
- V 방향은 degree가 주어지면 그대로 사용, 없으면 자동 계산
```rust
let u_knots = KnotVector::generate_interpolation(control_points[0].len(), curves[0].degree)?;
let v_knots = KnotVector::generate_interpolation(curves.len(), degree.unwrap_or(3))?;
```

## 5️⃣ 가중치 설정
```rust
let weights = vec![vec![T::one(); control_points[0].len()]; curves.len()];
```

- 기본적으로 모든 가중치를 1.0으로 설정 → B-spline과 동일하게 작동

## 6️⃣ 서페이스 생성
```rust
Ok(NurbsSurface {
    control_points,
    weights,
    u_knots,
    v_knots,
    u_degree: curves[0].degree,
    v_degree: degree.unwrap_or(3),
})
```

- 제어점, 가중치, 노트 벡터, 차수를 기반으로 NURBS 서페이스 객체 생성

## ✅ 요약 흐름
곡선 배열 → 제어점 정렬 → 노트 벡터 생성 → 가중치 설정 → 서페이스 생성
## ✨ 배울 점
| 항목                  | 설명                                           |
|-----------------------|------------------------------------------------|
| Option<usize>         | 선택적 입력을 통해 API 유연성 확보 (`degree`)     |
| 구조 정렬 검증        | 곡선 배열의 제어점 수 일치 여부를 체크하는 로직    |
| iter().map().collect()| 제어점 배열을 깔끔하게 2D 구조로 변환하는 반복자 활용 |
| T: Float              | 제네릭 수치 타입으로 `f32`, `f64` 모두 지원       |
| 불변성 유지           | 모든 입력은 참조 기반, 내부에서 새 구조 생성       |

---

# tessellate

tessellate 함수는 NURBS 서페이스를 삼각형 mesh로 분할하는 기능을 수행합니다.  
아래에 이 함수의 구조와 작동 원리를 단계적으로 설명.

## 🔧 1단계: 함수 정의 (Trait 기반)
```rust
pub trait Tessellation {
    type Option;
    type Output;

    fn tessellate(&self, options: Self::Option) -> Self::Output;
}
```

- Tessellation은 trait으로 정의되어 있고, 다양한 shape에 대해 구현됨
- Self::Option: 세분화 옵션 (예: 샘플링 밀도)
- Self::Output: 결과 mesh (예: 삼각형 리스트)

## 🧩 2단계: NurbsSurface에 대한 구현
```rust
impl<T: FloatingPoint, D> Tessellation for NurbsSurface<T, D> {
    fn tessellate(&self, options: SurfaceTessellation) -> Mesh<T>
}
```

- SurfaceTessellation은 u_divisions, v_divisions 같은 파라메트릭 샘플링 옵션을 포함
- Mesh<T>는 vertex와 triangle 인덱스를 포함하는 구조체

## 🧮 3단계: 파라메트릭 공간 샘플링
```rust
let u_samples = linspace(0.0, 1.0, options.u_divisions);
let v_samples = linspace(0.0, 1.0, options.v_divisions);
```

- u, v 방향으로 균등하게 샘플링
- evaluate(u, v)를 통해 실제 3D 점 계산

## 🧷 4단계: vertex 생성
```rust
for u in &u_samples {
    for v in &v_samples {
        let point = self.evaluate(*u, *v);
        vertices.push(point);
    }
}
```

- 각 (u, v) 조합에 대해 서페이스 평가 → vertex 생성
- vertices: Vec<Point3<T>>

## 🔺 5단계: 삼각형 인덱스 구성
```rust
for i in 0..(u_div - 1) {
    for j in 0..(v_div - 1) {
        let idx0 = i * v_div + j;
        let idx1 = idx0 + 1;
        let idx2 = idx0 + v_div;
        let idx3 = idx2 + 1;

        triangles.push([idx0, idx1, idx2]);
        triangles.push([idx2, idx1, idx3]);
    }
}
```

- vertex 인덱스를 기반으로 삼각형 2개씩 생성 → quad → 2 triangles
- triangles: Vec<[usize; 3]>

## ✅ 결과 구조
```rust
Mesh {
    vertices: Vec<Point3<T>>,
    triangles: Vec<[usize; 3]>,
}
```

- 렌더링이나 시뮬레이션에 바로 사용할 수 있는 구조

## ✨ 배울 점
| 항목                   | 설명                                                             |
|------------------------|------------------------------------------------------------------|
| tessellate()           | Trait 기반으로 다양한 shape에 대해 mesh 분할 기능을 추상화         |
| linspace + evaluate(u,v)| 파라메트릭 공간을 균등 샘플링하고 서페이스 평가로 vertex 생성     |
| vertex/triangle 분리    | 구조적 mesh 설계 방식으로 렌더링에 최적화된 데이터 구성             |
| iter() + zip()         | 반복자 기반 처리로 깔끔하고 안전한 vertex/triangle 생성             |
| T: FloatingPoint       | 제네릭 수치 타입으로 `f32`, `f64` 모두 지원하며 수치 안정성 확보     |

---

# 구조 단순화 포인트

어떻게 구조를 단순화할 수 있을까?
## 1️⃣ T: FloatingPoint 제거하고 f64로 고정
```rust
pub struct NurbsCurve3D {
    control_points: Vec<Point3<f64>>,
    knots: KnotVector<f64>,
    weights: Vec<f64>,
    degree: usize,
}
```

## 2️⃣ impl NurbsCurve3D로 구현
→ 제네릭 없이 모든 함수가 f64 기반으로 작동

### 3️⃣ 필요하면 나중에 T로 일반화
→ 구조가 안정되고 나서 다시 제네릭화하면 훨씬 안전하고 예측 가능


---


