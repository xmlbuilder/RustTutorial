# Convex Poly
## Convex poly code walkthrough with formulas and test explanations
- 이 코드는 2D 볼록 껍질(monotone chain)과 3D 볼록체 사이 GJK 기반 최근접점 계산을 구현합니다.  
- 각 타입과 함수의 목적, 사용 수식, 수치 안정화 포인트, 그리고 테스트 케이스 의도를 단계별로 설명합니다.

### Index4: 최대 4개의 정점 인덱스를 담는 컨테이너
- 역할: GJK에서 활성화된 지지점(support)들의 원본 점 인덱스를 담습니다.  
    최대 사단 단체(tetrahedron)까지 고려하므로 4개 슬롯.
- 핵심 메서드:
- at(idx): i, j, k, l 중 선택 반환.
- set_at(idx, v): 지정 슬롯에 값 저장.
- as_array(): [i, j, k, l] 배열 반환.
- UNSET: 모든 슬롯이 -1로 비활성화된 상태를 표준화.

### Simplex3D: 0~4개의 점으로 구성된 심플렉스
- 내부 표현: 실제론 점이지만 Vector3D로 저장. 계산을 벡터 연산으로 통일.
- 생성자:
- from_point/segment/triangle/tetra: 정점 수에 맞춰 초기화.
### 유효성 검사
- is_valid(eps): 정점들이 아핀 독립인지 검사.
- n=2: 길이 $\| v_1-v_0\| >\varepsilon$ 
- n=3: 면적 벡터 $\mathbf{x}=(v_1-v_0)\times (v_2-v_0)$ 의 길이 $\| \mathbf{x}\| >\varepsilon$ 
- n=4: 삼중곱 $((v_1-v_0)\times (v_2-v_0))\cdot (v_3-v_0)$ 의 절대값 $>\varepsilon$ 
### 바리센트릭 평가
- evaluate_bary(b[4]):

$$
p(b)=\sum _{i=0}^{n-1}b_i\, v_i
$$

- evaluate_bary_p4(Point4D): 위와 동일, Point4D에서 배열로 변환.
### 부피/면적/길이
- volume():
- n=2: $\| v_1-v_0\|$ 
- n=3: $\frac{1}{2}\| (v_1-v_0)\times (v_2-v_0)\|$ 
- n=4: $\frac{1}{6}\left| ((v_1-v_0)\times (v_2-v_0))\cdot (v_3-v_0)\right|$
- signed_volume(): n=4일 때만 부호를 유지한 체적:

$$
\frac{1}{6}\left( ((v_1-v_0)\times (v_2-v_0))\cdot (v_3-v_0)\right)
$$


### 법선과 수치 안정화
- face_normal(noti): noti 정점을 제외한 면의 비정규화 법선
- 기본: $\mathbf{n}=(v_{i_1}-v_{i_0})\times (v_{i_2}-v_{i_0})$
- cross_care(a,b): 외적의 크기가 너무 작을 때 대체 벡터 조합으로 안정화.  
    세 벡터의 최대절대좌표를 비교해 가장 안정적인 조합 선택.
### 최근접점 계산
- closest_point(p0, bary, at_most): p0 기준으로 심플렉스 좌표계를 평행이동하여 원점에 대한 최근접점 문제로 변환.
- 심플렉스의 모든 정점에 대해 $v_i'=v_i-p_0$.
- **closest_point_to_origin()** 로 바리센트릭 계산.
- 계산된 최근접점 거리 $\| p(b)\|$ 가 at_most 이하여야 true.
- closest_point_to_origin(bary): n에 따라 루틴 분기
- 1plex(선분), 2plex(삼각형), 3plex(사면체)
### closest_1plex(bary)
- 선분 $[v_0,v_1]$ 에 원점을 투영:

$$
t=\frac{-(v_0\cdot (v_1-v_0))}{\| v_1-v_0\| ^2}
$$

- $t ≤ 0 → v0$
- $t ≥ 1 → v1$
- 그 외 → 선분 내부. 바리센트릭 $(1-t,t)$

### closest_2plex(bary)
- 삼각형 면으로의 직교 투영:
- 면 법선 $\mathbf{n}=(v_1-v_0)\times (v_2-v_0)$, 투영점 $p_3=(v_0\cdot \mathbf{n}/\| \mathbf{n}\| ^2)\mathbf{n}$
- $\tilde {v}_i=v_i-p_3$ 를 2D로 축소(법선의 최대축 제외)
- 각 코팩터 유사량 c_3[i]를 계산하여

$$
\det M=c_3[0]+c_3[1]+c_3[2]
$$

- 모두 같은 부호면 내부 → 바리센트릭 $b_i=c_3[i]/\det M$
- 아니면 경계(각 변)로 재귀: 해당 변에서 1D 최근접점 수행 후 전체로 삽입

### closest_3plex(bary)
- 사면체 내부 판정과 경계 재귀:
- 각 면의 코팩터(삼중곱) $c_4[j]$ 계산

$$
c_4[j]=\pm [v_a,v_b,v_c]=\pm ((v_a\times v_b)\cdot v_c)
$$

- 부호는 위치에 따라 번갈아 적용.
- $\det M=\sum _jc_4[j]$, 모두 같은 부호면 내부 → $b_j=c_4[j]/\det M$
- 아니면 해당 면을 제거한 삼각형으로 줄여 2plex 루틴 수행. 최소 거리 혹은 작은 support 크기(활성 좌표 수)를 선택.

### ConvexPoly: 볼록체 인터페이스
- 필수 메서드:
    - count(): 정점 수
    - vertex(i): i번째 정점 좌표
    - support_index(dir, seed): 방향 dir에 대해 최댓점 인덱스. seed로 단축 가능
    - maximum_coordinate(): 수치 중단 조건 등에 쓰는 최대 절대좌표
    - support(dir, seed): support_index를 통해 지지점 좌표 반환.
    - evaluate(idx4, bary): 0~4개 지지점과 바리센트릭으로 점 복원:

$$
p=\sum _{k=0}^3b_k\, v_{\mathrm{idx}[k]},\quad \mathrm{단\  idx}[k]\geq 0
$$

- is_valid_index4(idx4): 각 인덱스가 범위 내인지 확인.
- standardize_index4(idx, bary): 같은 인덱스가 중복될 때 계수 합치고, 사용 슬롯을 앞으로 당겨 표준형으로 정리.

### ConvexHullSlice / ConvexHullOwned
- ConvexHullSlice<'a>: 외부 슬라이스 참조형 볼록체. 복사 없이 빠르게 지원.
- support_index: 모든 점에 대해 $\max \langle p,d\rangle$ 를 찾는 전수 탐색. seed로 시작점 최적화.
- ConvexHullOwned: 내부에 점을 소유하는 볼록체.
- append_vertex: 점 추가, 인덱스 반환.
- 두 구현 모두 동일한 인터페이스로 GJK에서 사용.
### GjkSimplex: GJK에서 쓰는 심플렉스와 대응 인덱스
- simp: Simplex3D
- bary: simp 상의 바리센트릭 좌표
- a_idx/b_idx: 심플렉스 각 정점에 대응하는 원래 A/B의 정점 인덱스
- 핵심 메서드:
    - add_vertex(w, aind, bind): Minkowski 차 w=a-b 추가. 대응 인덱스 기록, 바리 초기화.
    - remove_vertex(i): i 제거 후 배열 압축 및 바리 이동.
    - includes(aind, bind): 이미 포함된 조합 방지.
    - bary_at(i): 심플렉스 내 특정 슬롯의 바리센트릭 계수.
### GJK: 볼록체-볼록체 최근접점 
- convex_poly_closest_point_to_point
- 포인트 p0를 “한 개 정점”을 가진 볼록체로 래핑하여 poly-poly 루틴을 재사용.
- 결과로 A의 Index4, bary를 반환. 필요시 표준화.
### convex_poly_closest_poly_poly
- 개요: A와 B의 Minkowski 차 $A\ominus B=\{ a-b\}$ 에서 원점과의 최근접점을 찾으면, 그 바리센트릭으로 A/B의 각각 최근접점도 동시에 복원 가능.
- 시드:
    - adex/bdex가 유효하면 초기 심플렉스로 활용(“매칭” 검증 후).
    - 아니면 기본 시드: $v=A_0-B_0$
- 루프 핵심:
- 서포트 점 추가:

$$
w=\mathrm{support_{\mathnormal{A}}}(-v)-\mathrm{support_{\mathnormal{B}}}(v)
$$

- 진행량 평가:

$$
\mu =\max (\mu ,\langle \hat {v},w\rangle ),\quad \hat {v}=\frac{v}{\| v\| }
$$

- 정지 조건:
- 심플렉스 정점 수가 4(사면체)에 도달
- 같은 (a_idx,b_idx) 조합이 재등장
- $(\| v\| -\mu )\leq 2\mu \cdot \mathrm{eps}+\mathrm{simplex\_ norm}\cdot 20\epsilon$
- $\mu >\mathrm{at\_ most}$
- $\| v\|$ 이 더 이상 줄지 않음
- 원점 최근접점 업데이트: simp.closest_point_to_origin() → bary → v=p(b)
- bary=0인 정점 제거: 지원 집합 최소화
- 사면체 보정: 볼륨이 충분히 크면 $\| v\| \rightarrow 0$ 으로 간주(침투/접촉 안정화).
- 성공 판단: 최종 $\| v\| \leq \mathrm{at\_ most}$
- 결과 매핑:
    - a_idx/b_idx와 bary로 A/B의 최근접점 계산에 동일 바리 사용 가능(각자의 정점 조합에 매핑).
    - 부족 슬롯은 -1과 0으로 채움.
### 2D convex hull: monotone chain
- 정렬: x→y 정렬 후 중복 제거.
- lower hull: 스택 기반으로 좌회전만 유지.
- 좌회전 조건:

$$
\mathrm{cross_{\mathnormal{2}}}d(p,q,r)=(q.x-p.x)(r.y-p.y)-(q.y-p.y)(r.x-p.x)>0
$$

- upper hull: 역순으로 동일 처리. 마지막 중복 제거.
- 결과: 반시계 방향으로 유니크한 껍질 정점들.
- 차원 판정:
- 1점 → 0차원
- 2점 → 1차원(콜리니어)
- 그 외 → 2차원 다각형
- 보조 함수 is_left_turn는 수치 안정화를 위해 법선 방향 투영 크기 비교를 추가했지만 실제 hull은 cross_2d 기반으로 처리합니다.

### 테스트 설명
#### tests_convex_hull_2d
- rectangle_with_inner_points:
    - 직사각형의 네 꼭짓점과 내부/중복 점 입력.
    - 결과 hull은 네 꼭짓점만 포함해야 하며, 순서는 무관하므로 정렬 후 비교.
- colinear_points:
    - x축 상의 콜리니어 점들.
    - 결과는 양 끝점 두 개만 남아야 하며, 차원은 1로 판정.
- single_point:
    - 단일 점 입력.
    - 결과 hull은 같은 점 하나, 차원 0.
- duplicate_points_only:
    - 동일 점만 여러 개.
    - 결과는 해당 점 하나, 차원 0.
#### tests_convex_poly_3d
- point_inside_cube_should_project_to_itself:
    - 큐브 내부 점 p를 큐브에 투영 → 자신이어야 하며 거리 0에 수렴.
- point_outside_cube_should_project_to_face_center:
    - x=2, y=z=0.5인 점. 큐브의 x=1 면 중심으로 투영되어 q=(1,0.5,0.5), 거리 1이어야 함.
    - point_on_cube_surface_should_remain_on_surface:
    - 점이 면 위에 정확히 위치 → 투영점은 자신, 거리 ~0.
- two_cubes_separated_distance_two:
    - 두 단위 큐브를 x축 방향으로 3만큼 벌림.
    - 최근접 거리 2.0이어야 함. GJK는 분리된 볼록체의 최근접점 쌍을 복원.
- two_cubes_touching_on_face_distance_zero:
    - x=1 면끼리 접하는 배치. at_most=0.1로 거의 0 거리만 허용.
    - 거리 ~0이어야 함. 접촉 검출 목적에 적합.
- two_cubes_overlapping_distance_zero:
    - x 방향으로 0.5 겹침. 침투 상태 → 거리 ~0.
    - point_vs_segment:
    - 원점과 [1,2] 선분의 최근접 거리 1.0 확인.
    - GJK가 1D/0D 혼합 케이스에서도 수렴하는지 검증.
- segment_vs_triangle:
    - x축 선분과 y=1 평면 삼각형 배치. y축 방향 거리 ~1.0.
    - 복합 차원 케이스에서의 정확성 점검.

### 실전 팁과 확장 포인트
- 수치 안정화: cross_care, round_barycentric, 정지 조건의 eps/epsilon 계수는 실제 환경에서 데이터 스케일에 맞춰 조정 필요.
- 요약 출력: 대형 데이터 디버그에는 TextLog의 “열 제한”, “행 제한”, “앞/뒤 요약” 같은 기능을 적용하면 가독성이 좋아집니다.
- API 경계: ConvexPoly 트레이트로 다양한 볼록체 구현을 쉽게 GJK에 연결 가능.  
    Mesh, Polyhedron도 동일 트레이트로 wrapping하면 확장 가능합니다.
- 성능: support_index는 O(n).  
    큰 볼록체엔 hill-climbing이나 Voronoifeature 캐싱 같은 최적화가 유용합니다.

---

지금까지 만든 ConvexPoly / Simplex3D / GJK 최근접점 함수들은 실제로 여러 분야에서 핵심적으로 쓰입니다.  
어디에 응용할 수 있는지 단계별로 정리해드릴게요:

## 📐 기하학 / CAD 응용
- 충돌 판정(Collision Detection)
    GJK 알고리즘은 두 볼록체가 겹치는지, 얼마나 떨어져 있는지 계산하는 데 최적화되어 있습니다.  
    CAD 커널, 게임 엔진, 로봇 시뮬레이션에서 가장 많이 쓰이는 충돌 판정 알고리즘 중 하나.  
- 최근접점 계산(Closest Point Query)  
    두 개의 복잡한 형상 사이에서 가장 가까운 점 쌍을 찾을 수 있습니다.  
    CAD에서 치수 측정, 간섭 검사(interference check)에 활용됩니다.  
- NURBS / Mesh 처리
    ConvexPoly 트레이트를 통해 NURBS 곡선/곡면, 메쉬의 부분 볼록체를 래핑하면 동일한 GJK 루틴으로 최근접점이나 충돌을 판정할 수 있습니다.

## 🎮 게임 / 시뮬레이션
- 물리 엔진(Physics Engine)  
    Unity, Unreal 같은 엔진 내부에서도 GJK + EPA(Expanding Polytope Algorithm)를 써서 충돌 깊이와 접촉점을 계산합니다.
- 실시간 충돌 검사  
    캐릭터와 환경, 총알과 오브젝트 등 빠른 충돌 판정이 필요할 때 GJK가 쓰입니다.
- 경로 계획(Path Planning)  
    로봇이나 AI가 장애물을 피해 움직일 때, 볼록체 충돌 판정으로 안전한 경로를 찾습니다.

## 🤖 로보틱스 / 제조
- 로봇 팔 충돌 회피  
    로봇의 링크(link)를 볼록체로 근사하고, 작업 공간의 장애물과 충돌 여부를 GJK로 판정.
- CNC / 3D 프린팅 시뮬레이션  
    공구(tool)와 소재(workpiece) 간 간섭 여부를 검사해 안전한 가공 경로를 생성.

## 🧮 수학 / 최적화
- 볼록체 간 거리 계산
    - 두 집합 A,B의 최소 거리:

$$
d(A,B)=\min _{a\in A,b\in B}\| a-b\| 
$$

- GJK는 이 문제를 효율적으로 풀어줍니다.
- 선형 독립성 / 부피 판정  
    Simplex3D의 is_valid, volume 함수는 선형대수학적 독립성, 부피 계산에 직접 응용 가능.

## 🧪 테스트 코드 응용- CAD 커널 검증
Cube, Segment, Triangle 같은 단순 기하 구조로 테스트 → 실제 NURBS, Mesh로 확장.  
- 물리 엔진 단위 테스트  
    충돌 여부, 최근접 거리, 접촉점 계산이 정확한지 검증.
- 수치 안정성 테스트  
    cross_care, round_barycentric 같은 보정 루틴이 잘 동작하는지 극단적인 케이스(콜리니어, 평면, 중복점)로 확인.

## ✨ 정리- CAD/CAE: 간섭 검사, 치수 측정
- 게임/시뮬레이션: 충돌 판정, 물리 엔진
- 로보틱스: 경로 계획, 충돌 회피
- 수학/최적화: 볼록체 거리, 부피 계산

---

## 소스 코드

```rust
use crate::core::boundingbox::BoundingBox;
use crate::core::geom::Point2D;
use crate::core::prelude::{Point3D, Point4D, Vector3D};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Index4 {
    pub i: i32,
    pub j: i32,
    pub k: i32,
    pub l: i32,
}
```
```rust
impl Index4 {
    pub const UNSET: Index4 = Index4 {
        i: -1,
        j: -1,
        k: -1,
        l: -1,
    };
```
```rust
impl Index4 {
    #[inline]
    pub fn at(&self, idx: usize) -> i32 {
        match idx {
            0 => self.i,
            1 => self.j,
            2 => self.k,
            3 => self.l,
            _ => panic!("Index4::at out of range"),
        }
    }
```
```rust
impl Index4 {
    #[inline]
    pub fn set_at(&mut self, idx: usize, v: i32) {
        match idx {
            0 => self.i = v,
            1 => self.j = v,
            2 => self.k = v,
            3 => self.l = v,
            _ => panic!("Index4::set_at out of range"),
        }
    }
```
```rust
impl Index4 {
    #[inline]
    pub fn as_array(&self) -> [i32; 4] {
        [self.i, self.j, self.k, self.l]
    }
}

```
```rust
impl Index4 {
#[derive(Clone, Debug)]
pub struct Simplex3D {
    n: usize,              // 0..=4
    v: [Vector3D; 4],      // 실제로는 Point지만 Vector3D 재사용
}
```
```rust
impl Index4 {
impl Simplex3D {
    pub fn new() -> Self {
        Self {
            n: 0,
            v: [Vector3D::zero(); 4],
        }
    }
```
```rust
impl Index4 {
    pub fn from_point(a: Point3D) -> Self {
        let mut s = Self::new();
        s.n = 1;
        s.v[0] = a.to_vector();
        s
    }
```
```rust
impl Index4 {
    pub fn from_segment(a: Point3D, b: Point3D) -> Self {
        let mut s = Self::new();
        s.n = 2;
        s.v[0] = a.to_vector();
        s.v[1] = b.to_vector();
        s
    }
```
```rust
impl Index4 {
    pub fn from_triangle(a: Point3D, b: Point3D, c: Point3D) -> Self {
        let mut s = Self::new();
        s.n = 3;
        s.v[0] = a.to_vector();
        s.v[1] = b.to_vector();
        s.v[2] = c.to_vector();
        s
    }
```
```rust
impl Index4 {
    pub fn from_tetra(a: Point3D, b: Point3D, c: Point3D, d: Point3D) -> Self {
        let mut s = Self::new();
        s.n = 4;
        s.v[0] = a.to_vector();
        s.v[1] = b.to_vector();
        s.v[2] = c.to_vector();
        s.v[3] = d.to_vector();
        s
    }
```
```rust
impl Index4 {
    #[inline]
    pub fn count(&self) -> usize {
        self.n
    }
```
```rust
impl Index4 {
    /// Vertices 가 affine independent 인지 검사
    pub fn is_valid(&self, eps: f64) -> bool {
        if self.n < 2 {
            return true;
        }

        let v = self.v[1] - self.v[0];
        if self.n == 2 {
            return v.length() > eps;
        }

        let w = self.v[2] - self.v[0];
        let x = v.cross(&w);

        if self.n == 3 {
            return x.length() > eps;
        }

        let triple = x.dot(&(self.v[3] - self.v[0]));
        triple.abs() > eps
    }
```
```rust
impl Index4 {
    pub fn vertex(&self, i: usize) -> Point3D {
        Point3D::from(self.v[i])
    }
```
```rust
impl Index4 {
    pub fn vertex_mut(&mut self, i: usize) -> &mut Vector3D {
        &mut self.v[i]
    }
```
```rust
impl Index4 {
    pub fn evaluate_bary(&self, b: &[f64; 4]) -> Point3D {
        let mut p = Vector3D::zero();
        for i in 0..self.n {
            p += self.v[i] * b[i];
        }
        Point3D::from(p)
    }
```
```rust
impl Index4 {
    pub fn evaluate_bary_p4(&self, b: &Point4D) -> Point3D {
        let arr = [b.x, b.y, b.z, b.w];
        self.evaluate_bary(&arr)
    }
```
```rust
impl Index4 {
    /// Count=2: 길이 / Count=3: 면적 / Count=4: 부피
    pub fn volume(&self) -> f64 {
        if self.n < 2 {
            return 0.0;
        }
        let v = self.v[1] - self.v[0];
        if self.n == 2 {
            v.length()
        } else {
            let x = v.cross(&(self.v[2] - self.v[0]));
            if self.n == 3 {
                0.5 * x.length()
            } else {
                (x.dot(&(self.v[3] - self.v[0]))).abs() / 6.0
            }
        }
    }
```
```rust
impl Index4 {
    /// Count == 4 일 때 signed volume, 아니면 UNSET 표시로 NaN 반환
    pub fn signed_volume(&self) -> f64 {
        if self.n != 4 {
            f64::NAN
        } else {
            let v = self.v[1] - self.v[0];
            let x = v.cross(&(self.v[2] - self.v[0]));
            x.dot(&(self.v[3] - self.v[0])) / 6.0
        }
    }
```
```rust
impl Index4 {
    pub fn maximum_coordinate(&self) -> f64 {
        let mut max_val : f64 = 0.0;
        for i in 0..self.n {
            max_val = max_val.max(self.v[i].max_abs_coord());
        }
        max_val
    }
```
```rust
impl Index4 {
    pub fn bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::empty();
        for i in 0..self.n {
            bb.add_point(&self.vertex(i));
        }
        bb
    }
```
```rust
impl Index4 {
    pub fn transform(&mut self, m: &crate::core::transform::Transform) {
        for i in 0..self.n {
            let p = self.vertex(i);
            let tp = m.apply_point(&p);
            self.v[i] = tp.to_vector();
        }
    }
```
```rust
impl Index4 {
    pub fn translate(&mut self, d: Vector3D) {
        for i in 0..self.n {
            self.v[i] += d;
        }
    }
```
```rust
impl Index4 {
    pub fn edge(&self, e0: usize, e1: usize) -> Vector3D {
        if e0 < self.n && e1 < self.n {
            self.v[e1] - self.v[e0]
        } else {
            Vector3D::nan()
        }
    }
```
```rust
impl Index4 {
    pub fn remove_vertex(&mut self, idx: usize) -> bool {
        if idx >= self.n {
            return false;
        }
        for i in idx..(self.n - 1) {
            self.v[i] = self.v[i + 1];
        }
        self.n -= 1;
        true
    }
```
```rust
impl Index4 {
    pub fn add_vertex(&mut self, p: Point3D) -> bool {
        if self.n >= 4 {
            return false;
        }
        self.v[self.n] = p.to_vector();
        self.n += 1;
        true
    }
```
```rust
impl Index4 {
    pub fn set_vertex(&mut self, idx: usize, p: Point3D) -> bool {
        if idx >= self.n {
            return false;
        }
        self.v[idx] = p.to_vector();
        true
    }
```
```rust
impl Index4 {
    fn cross_care(a: &Vector3D, b: &Vector3D) -> Vector3D {
        let mut norm = [0.0; 3];
        norm[0] = a.max_abs_coord();
        norm[1] = b.max_abs_coord();

        let mut axb = a.cross(b);
        let thresh = 1.0e-8;
        let ab = norm[0] * norm[1];
        let ab2 = ab * ab;
        if axb.length_squared() < ab2 * thresh {
            let v0 = *a;
            let v1 = *b;
            let v2 = v0 - v1;
            norm[2] = v2.max_abs_coord();

            let mut maxi = if norm[0] > norm[1] { 0 } else { 1 };
            if norm[2] < norm[maxi] {
                // v[maxi+1], v[maxi+2]
                let va = if maxi == 0 { v1 } else { v0 };
                let vb = v2;
                axb = va.cross(&vb);
                if maxi == 0 {
                    axb = -axb;
                }
            }
        }
        axb
    }
```
```rust
impl Index4 {
    /// noti 를 제외한 면의 (비정규화) 법선
    pub fn face_normal(&self, noti: i32) -> Vector3D {
        if self.n < 3 {
            return Vector3D::nan();
        }
        if self.n == 4 && !(0..=3).contains(&noti) {
            return Vector3D::nan();
        }

        let mut idx = [0usize, 1, 2];
        if self.n == 4 && noti >= 0 && noti < 3 {
            let ni = noti as usize;
            for ii in 0..3 {
                idx[ii] = (ni + 1 + ii) % 4;
            }
        }
        let a = self.v[idx[1]] - self.v[idx[0]];
        let b = self.v[idx[2]] - self.v[idx[0]];
        Self::cross_care(&a, &b)
    }
```
```rust
impl Index4 {
    pub fn face_unit_normal(&self, noti: i32) -> Vector3D {
        let mut n = self.face_normal(noti);
        if !n.is_nan() && n.length_squared() > 0.0 {
            n = n.unitize();
        }
        n
    }
```
```rust
impl Index4 {
    /// P0에서 simplex까지의 최근접점 (bary 로 반환)
    pub fn closest_point(&self, p0: &Point3D, bary: &mut Point4D, at_most: f64) -> bool {
        // 원본: P0 만큼 평행이동해서 원점 distance 를 재는 방식
        let mut shifted = Simplex3D::new();
        shifted.n = self.n;

        let v0 = p0.to_vector();
        let mut too_far = at_most > 0.0;

        for i in 0..self.n {
            shifted.v[i] = self.v[i] - v0;
            if too_far && shifted.v[i].max_abs_coord() < 0.5 * at_most {
                too_far = false;
            }
        }

        if too_far {
            return false;
        }

        let mut tmp = Point4D::zero();
        let ok = shifted.closest_point_to_origin(&mut tmp);
        if !ok {
            return false;
        }

        if at_most >= 0.0 {
            let cp = shifted.evaluate_bary_p4(&tmp).to_vector();
            if cp.length_squared() > at_most * at_most {
                return false;
            }
        }

        *bary = tmp;
        true
    }
```
```rust
impl Index4 {
    pub fn closest_point_to_origin(&self, bary: &mut Point4D) -> bool {
        match self.n {
            0 => false,
            1 => {
                *bary = Point4D::new(1.0, 0.0, 0.0, 0.0);
                true
            }
            2 => self.closest_1plex(bary),
            3 => self.closest_2plex(bary),
            4 => self.closest_3plex(bary),
            _ => false,
        }
    }
```
```rust
impl Index4 {
    fn same_sign(a: f64, b: f64) -> bool {
        a * b > 0.0
    }
```
```rust
impl Index4 {
    fn round_barycentric(b: &mut Point4D) {
        // i 가 0..3이라고 가정
        let mut min_idx: i32 = -1;
        let mut min_val = f64::INFINITY;
        for i in 0..4 {
            let c = match i {
                0 => &mut b.x,
                1 => &mut b.y,
                2 => &mut b.z,
                3 => &mut b.w,
                _ => unreachable!(),
            };
            if *c == 0.0 {
                continue;
            }
            *c = 1.0 - (1.0 - *c);
            if min_idx < 0 || *c < min_val {
                min_idx = i as i32;
                min_val = *c;
            }
        }
        if min_idx >= 0 {
            let i = min_idx as usize;
            let mut s = 0.0;
            for j in 0..4 {
                if j != i {
                    s += match j {
                        0 => b.x,
                        1 => b.y,
                        2 => b.z,
                        3 => b.w,
                        _ => unreachable!(),
                    };
                }
            }
            let ci = match i {
                0 => &mut b.x,
                1 => &mut b.y,
                2 => &mut b.z,
                3 => &mut b.w,
                _ => unreachable!(),
            };
            *ci = 1.0 - s;
        }
    }
```
```rust
impl Index4 {
    fn closest_1plex(&self, bary: &mut Point4D) -> bool {
        let del = self.v[1] - self.v[0];
        let del2 = del.length_squared();
        if del2 <= 0.0 {
            return false;
        }

        let dot = -self.v[0].dot(&del);
        if dot >= del2 {
            *bary = Point4D::new(0.0, 1.0, 0.0, 0.0);
        } else if dot <= 0.0 {
            *bary = Point4D::new(1.0, 0.0, 0.0, 0.0);
        } else {
            let mut b0 = dot / del2;
            b0 = 1.0 - (1.0 - b0);
            *bary = Point4D::new(1.0 - b0, b0, 0.0, 0.0);
        }
        true
    }
```
```rust
impl Index4 {
    fn closest_2plex(&self, bary: &mut Point4D) -> bool {
        // face = 삼각형
        let n = self.face_normal(0);
        let n2 = n.length_squared();
        if n2 <= 0.0 {
            return false;
        }

        // origin 을 삼각형 affine span 으로 투영
        let p3 = (self.v[0].dot(&n) / n2) * n;

        // 변환된 planar 점
        let mut planar = [Vector3D::zero(); 3];
        for i in 0..3 {
            planar[i] = self.v[i] - p3;
        }

        // 가장 큰 좌표 축 제외한 2D 투영
        let j = n.max_abs_coord_index(); // 0,1,2
        let j0 = (j + 1) % 3;
        let j1 = (j + 2) % 3;

        let mut c3 = [0.0; 3];
        let mut det_m = 0.0;

        for i in 0..3 {
            let i0 = (i + 1) % 3;
            let i1 = (i + 2) % 3;
            let a0 = planar[i0][j0];
            let a1 = planar[i0][j1];
            let b0 = planar[i1][j0];
            let b1 = planar[i1][j1];
            c3[i] = a0 * b1 - b0 * a1;
            det_m += c3[i];
        }

        if det_m == 0.0 {
            return false;
        }

        let mut interior = true;
        for j in 0..3 {
            if !Self::same_sign(det_m, c3[j]) {
                interior = false;
                break;
            }
        }

        bary.w = 0.0;
        if interior {
            bary.x = c3[0] / det_m;
            bary.y = c3[1] / det_m;
            bary.z = c3[2] / det_m;
            Self::round_barycentric(bary);
            return true;
        }

        // boundary projection으로 떨어진 케이스
        let mut any_ok = false;
        let mut best_bary = Point4D::zero();
        let mut best_d2 = f64::MAX;

        for j in 0..3 {
            if !Self::same_sign(det_m, c3[j]) {
                // edge (j+1, j+2)
                let s = Simplex3D::from_segment(
                    Point3D::from(planar[(j + 1) % 3]),
                    Point3D::from(planar[(j + 2) % 3]),
                );
                let mut edge_bary = Point4D::zero();
                if s.closest_point_to_origin(&mut edge_bary) {
                    let on_end = edge_bary.x == 1.0 || edge_bary.y == 1.0;
                    let cp = s.evaluate_bary_p4(&edge_bary).to_vector();
                    let d2 = cp.length_squared();

                    // bary -> 전체 삼각형 기준으로 확장
                    let mut full = Point4D::zero();
                    full.w = 0.0;
                    full.set(j, 0.0);
                    full.set((j + 1) % 3, edge_bary.x);
                    full.set((j + 2) % 3, edge_bary.y);

                    if !any_ok || d2 < best_d2 {
                        any_ok = true;
                        best_d2 = d2;
                        best_bary = full;
                    }

                    if !on_end {
                        break;
                    }
                }
            }
        }

        if any_ok {
            *bary = best_bary;
        }
        any_ok
    }
```
```rust
impl Index4 {
    fn closest_3plex(&self, bary: &mut Point4D) -> bool {
        // tetrahedron
        // cofactor 기반 interior test + boundary recursion
        let mut idx = [1usize, 2, 3];
        let mut c4 = [0.0; 4];
        let mut det_m = 0.0;
        let mut sign = 1.0;

        for j in 0..4 {
            // triple product
            let v0 = self.v[idx[0]];
            let v1 = self.v[idx[1]];
            let v2 = self.v[idx[2]];
            let triple = Vector3D::triple(&v0, &v1, &v2);
            c4[j] = sign * triple;
            if j < 3 {
                idx[j] = j;
                sign = -sign;
            }
            det_m += c4[j];
        }

        if det_m == 0.0 {
            return false;
        }

        let mut interior = true;
        for j in 0..4 {
            if !Self::same_sign(det_m, c4[j]) {
                interior = false;
                break;
            }
        }

        if interior {
            bary.x = c4[0] / det_m;
            bary.y = c4[1] / det_m;
            bary.z = c4[2] / det_m;
            bary.w = c4[3] / det_m;
            Self::round_barycentric(bary);
            return true;
        }

        // boundary recursion
        let mut best_d2 = f64::MAX;
        let mut best_support = 5; // support size
        let mut best_bary = Point4D::zero();

        for j in 0..4 {
            if !Self::same_sign(det_m, c4[j]) {
                // j 를 제거한 삼각형으로 reduce
                let mut s = self.clone();
                s.remove_vertex(j);

                let mut b_tri = Point4D::zero();
                if s.closest_2plex(&mut b_tri) {
                    // active support size
                    let support_size = [b_tri.x, b_tri.y, b_tri.z]
                        .iter()
                        .filter(|&&c| c > 0.0)
                        .count();

                    // 삼각형 좌표를 4개의 좌표로 삽입 (j 위치 0)
                    let mut full = Point4D::zero();
                    let mut tri_idx = 0;
                    for i in 0..4 {
                        if i == j {
                            full.set(i, 0.0);
                        } else {
                            let c = match tri_idx {
                                0 => b_tri.x,
                                1 => b_tri.y,
                                2 => b_tri.z,
                                _ => 0.0,
                            };
                            full.set(i, c);
                            tri_idx += 1;
                        }
                    }

                    let cp = self.evaluate_bary_p4(&full).to_vector();
                    let d2 = cp.length_squared();

                    if d2 < best_d2 || (d2 == best_d2 && support_size < best_support) {
                        best_d2 = d2;
                        best_support = support_size;
                        best_bary = full;
                    }
                }
            }
        }

        if best_d2 < f64::MAX {
            *bary = best_bary;
            true
        } else {
            false
        }
    }
}
```
```rust
impl Index4 {
// Point4D 도움 메서드
trait Point4DExt {
    fn zero() -> Self;
    fn set(&mut self, i: usize, v: f64);
}
```
```rust
impl Index4 {
impl Point4DExt for Point4D {
    #[inline]
    fn zero() -> Self {
        Point4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    #[inline]
    fn set(&mut self, i: usize, v: f64) {
        match i {
            0 => self.x = v,
            1 => self.y = v,
            2 => self.z = v,
            3 => self.w = v,
            _ => panic!("Point4D::set index"),
        }
    }
}
```
```rust
impl Index4 {

pub trait ConvexPoly {
    fn count(&self) -> usize;
    fn vertex(&self, i: usize) -> Point3D;
    fn support_index(&self, dir: Vector3D, seed: usize) -> usize;
    fn maximum_coordinate(&self) -> f64;
```
```rust
impl Index4 {
    fn support(&self, dir: Vector3D, seed: usize) -> Point3D {
        let idx = self.support_index(dir, seed);
        self.vertex(idx)
    }
```
```rust
impl Index4 {
    fn evaluate(&self, idx: &Index4, bary: &Point4D) -> Point3D {
        let mut v = Vector3D::zero();
        let indices = idx.as_array();
        let b = [bary.x, bary.y, bary.z, bary.w];

        for k in 0..4 {
            let id = indices[k];
            if id >= 0 {
                v += self.vertex(id as usize).to_vector() * b[k];
            }
        }
        Point3D::from(v)
    }
```
```rust
impl Index4 {
    /// Index4 유효성 체크
    fn is_valid_index4(&self, idx4: &Index4) -> bool {
        let n = self.count() as i32;
        let arr = idx4.as_array();
        arr.iter().all(|&i| i < n)
    }

    /// Index4 + bary 를 표준형으로 정리 (같은 인덱스 합치기 등)
    fn standardize_index4(idx: &mut Index4, bary: &mut Point4D) {
        let mut rdex = Index4::UNSET;
        let mut rb = [0.0; 4];
        let mut ri = 0usize;

        let in_idx = idx.as_array();
        let mut in_b = [bary.x, bary.y, bary.z, bary.w];

        for ii in 0..4 {
            if in_idx[ii] < 0 || in_b[ii] == 0.0 {
                continue;
            }
            // 이미 들어가 있나 확인
            let mut j = 0usize;
            while j < ri && rdex.at(j) != in_idx[ii] {
                j += 1;
            }
            if j == ri {
                rdex.set_at(ri, in_idx[ii]);
                rb[ri] = 0.0;
                ri += 1;
            }
            rb[j] += in_b[ii];
        }

        *idx = rdex;
        bary.x = rb[0];
        bary.y = rb[1];
        bary.z = rb[2];
        bary.w = rb[3];
    }
}
```
```rust
impl Index4 {
/// 기존 C++ ON_ConvexHullRefEx: 포인트 slice 를 참조하는 convex poly
pub struct ConvexHullSlice<'a> {
    pts: &'a [Point3D],
}
```
```rust
impl Index4 {
impl<'a> ConvexHullSlice<'a> {
    pub fn new(pts: &'a [Point3D]) -> Self {
        Self { pts }
    }
}
```
```rust
impl Index4 {
impl<'a> ConvexPoly for ConvexHullSlice<'a> {
    fn count(&self) -> usize {
        self.pts.len()
    }

    fn vertex(&self, i: usize) -> Point3D {
        self.pts[i]
    }

    fn support_index(&self, dir: Vector3D, seed: usize) -> usize {
        let n = self.pts.len();
        if n == 0 {
            return 0;
        }

        let mut best = if seed < n { seed } else { 0 };
        let mut best_dot = self.pts[best].to_vector().dot(&dir);

        for j in 0..n {
            let d = self.pts[j].to_vector().dot(&dir);
            if d > best_dot {
                best_dot = d;
                best = j;
            }
        }
        best
    }

    fn maximum_coordinate(&self) -> f64 {
        let mut m = 0.0;
        for p in self.pts {
            let c = p.max_abs_coord();
            if c > m {
                m = c;
            }
        }
        m
    }
}
```
```rust
impl Index4 {
/// ON_ConvexHullPoint2: 점을 소유하는 convex poly
#[derive(Clone, Debug)]
pub struct ConvexHullOwned {
    pub vertices: Vec<Point3D>,
}
```
```rust
impl Index4 {
impl ConvexHullOwned {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(cap),
        }
    }

    pub fn append_vertex(&mut self, p: Point3D) -> usize {
        self.vertices.push(p);
        self.vertices.len() - 1
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }
}
```
```rust
impl Index4 {
impl ConvexPoly for ConvexHullOwned {
    fn count(&self) -> usize {
        self.vertices.len()
    }

    fn vertex(&self, i: usize) -> Point3D {
        self.vertices[i]
    }

    fn support_index(&self, dir: Vector3D, seed: usize) -> usize {
        let n = self.vertices.len();
        if n == 0 {
            return 0;
        }

        let mut best = if seed < n { seed } else { 0 };
        let mut best_dot = self.vertices[best].to_vector().dot(&dir);

        for j in 0..n {
            let d = self.vertices[j].to_vector().dot(&dir);
            if d > best_dot {
                best_dot = d;
                best = j;
            }
        }
        best
    }

    fn maximum_coordinate(&self) -> f64 {
        let mut m = 0.0;
        for p in &self.vertices {
            let c = p.max_abs_coord();
            if c > m {
                m = c;
            }
        }
        m
    }
}

```
```rust
impl Index4 {
/// 내부에서 쓰는 GJK simplex 구조
struct GjkSimplex {
    simp: Simplex3D,     // Minkowski A - B
    bary: Point4D,       // barycentric on simplex
    a_idx: [i32; 4],
    b_idx: [i32; 4],
}
```
```rust
impl GjkSimplex {
    fn new() -> Self {
        Self {
            simp: Simplex3D::new(),
            bary: Point4D::zero(),
            a_idx: [-1; 4],
            b_idx: [-1; 4],
        }
    }
```
```rust
    fn count(&self) -> usize {
        self.simp.count()
    }
```
```rust
    fn add_vertex(&mut self, v: Vector3D, aind: i32, bind: i32) -> bool {
        let n0 = self.simp.count();
        if n0 >= 4 {
            return false;
        }
        self.simp.add_vertex(Point3D::from(v));
        self.a_idx[n0] = aind;
        self.b_idx[n0] = bind;
        if n0 > 0 {
            self.bary.set(n0, 0.0);
        } else {
            self.bary.x = 1.0;
        }
        true
    }
```
```rust
    fn remove_vertex(&mut self, i: usize) -> bool {
        let n0 = self.simp.count();
        if i >= n0 {
            return false;
        }
        self.simp.remove_vertex(i);
        for j in i..(n0 - 1) {
            self.bary.set(j, self.bary_at(j + 1));
            self.a_idx[j] = self.a_idx[j + 1];
            self.b_idx[j] = self.b_idx[j + 1];
        }
        self.bary.set(n0 - 1, 0.0);
        self.a_idx[n0 - 1] = -1;
        self.b_idx[n0 - 1] = -1;
        true
    }
```
```rust
    fn includes(&self, aind: i32, bind: i32) -> bool {
        let n0 = self.simp.count();
        for i in 0..n0 {
            if self.a_idx[i] == aind && self.b_idx[i] == bind {
                return true;
            }
        }
        false
    }
```
```rust
    fn bary_at(&self, i: usize) -> f64 {
        match i {
            0 => self.bary.x,
            1 => self.bary.y,
            2 => self.bary.z,
            3 => self.bary.w,
            _ => 0.0,
        }
    }
}
```
```rust
pub fn convex_poly_closest_point_to_point<P: ConvexPoly>(
    hull: &P,
    p0: &Point3D,
    dex: &mut Index4,
    bary: &mut Point4D,
    max_dist: f64,
) -> bool {
    // point 를 1개의 vertex 를 가진 hull 로 보고 poly-poly GJK 를 호출
    let pts = [*p0];
    let ph = ConvexHullSlice::new(&pts);

    let mut adex = *dex;
    let mut bdex = Index4::UNSET;

    let ok = convex_poly_closest_poly_poly(hull, &ph, &mut adex, &mut bdex, bary, max_dist);

    if ok {
        *dex = adex;
        // index 정리
        P::standardize_index4(dex, bary);
    }

    ok
}
```
```rust
/// ConvexPoly A와 B 사이의 최근접점을 GJK로 구한다.
/// 결과 bary는 A/B에서 같은 bary 로 쓰인다.
pub fn convex_poly_closest_poly_poly<A: ConvexPoly, B: ConvexPoly>(
    a: &A,
    b: &B,
    adex: &mut Index4,
    bdex: &mut Index4,
    bary: &mut Point4D,
    at_most: f64,
) -> bool {
    if a.count() == 0 || b.count() == 0 {
        *adex = Index4::UNSET;
        *bdex = Index4::UNSET;
        return false;
    }

    let mut gjk = GjkSimplex::new();
    let mut done = false;
    let mut v = Vector3D::zero();

    // seed 가 유효하면 초기 simplex 로 사용
    let mut first_pass = false;
    {
        // MatchingSupport 검사는 간단히 "둘 다 >=0 또는 둘 다 <0" 만 체크
        let a_arr = adex.as_array();
        let b_arr = bdex.as_array();

        let mut match_ok = true;
        let mut nsup = 0;
        for i in 0..4 {
            let a_neg = a_arr[i] < 0;
            let b_neg = b_arr[i] < 0;
            if a_neg != b_neg {
                match_ok = false;
                break;
            }
            if !a_neg {
                nsup += 1;
            }
        }
        if match_ok && nsup > 0 && a.is_valid_index4(adex) && b.is_valid_index4(bdex) {
            let mut i = 0;
            while i < 4 {
                if a_arr[i] < 0 || b_arr[i] < 0 {
                    i += 1;
                    continue;
                }
                if gjk.includes(a_arr[i], b_arr[i]) {
                    break;
                }
                let va = a.vertex(a_arr[i] as usize).to_vector();
                let vb = b.vertex(b_arr[i] as usize).to_vector();
                gjk.add_vertex(va - vb, a_arr[i], b_arr[i]);
                i += 1;
            }
            first_pass = i == 4;
        }
    }

    let mut vlen = f64::MAX;
    let mut vlen_last = f64::MAX;

    while !done {
        if !first_pass {
            // 기본 seed: A[0] - B[0]
            let va0 = a.vertex(0).to_vector();
            let vb0 = b.vertex(0).to_vector();
            v = va0 - vb0;
            gjk.add_vertex(v, 0, 0);
            gjk.bary = Point4D::new(1.0, 0.0, 0.0, 0.0);
            vlen_last = f64::MAX;
            vlen = v.length();
        }

        let mut mu = 0.0;
        let eps = 10000.0 * f64::EPSILON;

        let mut wa = 0usize;
        let mut wb = 0usize;

        while !done && (first_pass || vlen > 0.0) {
            if !first_pass {
                wa = a.support_index(-v, wa);
                wb = b.support_index(v, wb);

                let va = a.vertex(wa).to_vector();
                let vb = b.vertex(wb).to_vector();
                let w = va - vb;

                let unit_v = v / vlen;
                let del = unit_v.dot(&w);
                if del > mu {
                    mu = del;
                }

                let simplex_norm = gjk.simp.maximum_coordinate();
                // stopping 조건 (C++ 코드와 동일 구조, 계수는 그대로)
                if gjk.count() == 4 || gjk.includes(wa as i32, wb as i32) {
                    done = true;
                } else if (vlen - mu) <= 2.0 * mu * eps + simplex_norm * 20.0 * f64::EPSILON
                    || mu > at_most
                    || vlen >= vlen_last
                {
                    done = true;
                }

                if done {
                    break;
                }

                gjk.add_vertex(w, wa as i32, wb as i32);
            }

            // simplex에서 원점까지 최근접점 (bary)
            let mut tmp_bary = Point4D::zero();
            if gjk.simp.closest_point_to_origin(&mut tmp_bary) {
                gjk.bary = tmp_bary;
                first_pass = false;
                v = gjk.simp.evaluate_bary_p4(&gjk.bary).to_vector();
                vlen_last = vlen;
                vlen = v.length();

                // bary==0 인 vertex 제거
                for i in (0..gjk.simp.count()).rev() {
                    if gjk.bary_at(i) == 0.0 {
                        gjk.remove_vertex(i);
                    }
                }
            } else {
                // seed 가 문제였던 경우: 초기 seed 없이 다시 돌릴 수 있게 루프 종료
                break;
            }
        }

        if !done {
            if first_pass {
                first_pass = false;
            } else {
                done = true;
            }
        }

        // tetra 가 "충분히 큰" 경우 0 으로 보정 (RH-xxxx 보정코드와 유사)
        if gjk.count() == 4 && gjk.simp.volume() > f64::EPSILON.sqrt() {
            vlen = 0.0;
        }

        let ok = vlen <= at_most;
        if ok {
            // 부족한 slot 은 -1, bary=0 으로 채운다
            let c = gjk.count();
            for i in c..4 {
                gjk.bary.set(i, 0.0);
                gjk.a_idx[i] = -1;
                gjk.b_idx[i] = -1;
            }

            *adex = Index4 {
                i: gjk.a_idx[0],
                j: gjk.a_idx[1],
                k: gjk.a_idx[2],
                l: gjk.a_idx[3],
            };
            *bdex = Index4 {
                i: gjk.b_idx[0],
                j: gjk.b_idx[1],
                k: gjk.b_idx[2],
                l: gjk.b_idx[3],
            };
            *bary = gjk.bary;
            return true;
        } else {
            return false;
        }
    }

    false
}
```
```rust

// P, Q, R 이 strict left turn 인지 (수치 안정성 고려한 버전)
fn is_left_turn(p: &Point2D, q: &Point2D, r: &Point2D) -> bool {
    let ax = r.x - q.x;
    let ay = r.y - q.y;
    let bx = p.x - q.x;
    let by = p.y - q.y;

    let det = ax * by - bx * ay;
    if det <= 0.0 {
        return false;
    }

    let dirx = r.x - p.x;
    let diry = r.y - p.y;
    let mut nx = -diry;
    let mut ny = dirx;
    let len = (nx * nx + ny * ny).sqrt();
    if len > 0.0 {
        nx /= len;
        ny /= len;
    }

    let delta_x = bx * nx;
    let delta_y = by * ny;

    // 원래 코드의 tolerance 는 ON_ZERO_TOLERANCE / ON_RELATIVE_TOLERANCE
    // 여기서는 간단히 eps 기반으로
    let eps = 1e-12;
    let rel = 1e-8;

    let mut left = false;
    for d in [delta_x, delta_y] {
        if d.abs() > eps && d.abs() > rel * q.x.abs().max(q.y.abs()) {
            left = true;
            break;
        }
    }
    left
}
```
```rust
/// cross((q - p), (r - p)) = (q - p) × (r - p)
#[inline]
fn cross_2d(p: &Point2D, q: &Point2D, r: &Point2D) -> f64 {
    (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
}
```
```rust
/// 2D Convex Hull
/// 반환값: 2 = 2D 다각형, 1 = line, 0 = point, <0 = 에러
///
/// hull: convex hull vertices
/// indices: hull[i] = pts[indices[i]] (옵션)
/// 2D Convex Hull (Monotone Chain)
///
/// 반환값:
///   2  -> 2D polygon
///   1  -> 1D segment (colinear)
///   0  -> a single point
///  <0  -> error (입력 없음)
///
/// `hull`   : 볼록 껍질에 속하는 점들 (반시계 방향, 시작/끝점 한 번씩만)
/// `indices`: hull[i] == pts[indices[i]] 를 만족하는 원본 인덱스 (옵션)
pub fn convex_hull_2d(
    pts: &[Point2D],
    hull: &mut Vec<Point2D>,
    mut indices: Option<&mut Vec<usize>>,
) -> i32 {
    hull.clear();
    if let Some(ind) = &mut indices {
        ind.clear();
    }

    let n = pts.len();
    if n == 0 {
        return -1;
    }

    // 1) x, y 기준으로 정렬된 인덱스
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let pa = &pts[a];
        let pb = &pts[b];
        match pa.x.partial_cmp(&pb.x).unwrap_or(std::cmp::Ordering::Equal) {
            std::cmp::Ordering::Equal => {
                pa.y.partial_cmp(&pb.y).unwrap_or(std::cmp::Ordering::Equal)
            }
            o => o,
        }
    });

    // 2) 중복 점 제거 (같은 좌표는 하나만 사용)
    let mut unique: Vec<usize> = Vec::with_capacity(n);
    for idx in order {
        if unique.is_empty() {
            unique.push(idx);
        } else {
            let last = *unique.last().unwrap();
            if (pts[idx].x - pts[last].x).abs() > 1e-15
                || (pts[idx].y - pts[last].y).abs() > 1e-15
            {
                unique.push(idx);
            }
        }
    }

    if unique.is_empty() {
        return -1;
    }
    if unique.len() == 1 {
        let i0 = unique[0];
        hull.push(pts[i0]);
        if let Some(ind) = &mut indices {
            ind.push(i0);
        }
        return 0; // point
    }

    // 3) Monotone chain: lower / upper
    let mut stack: Vec<usize> = Vec::with_capacity(unique.len() * 2);

    // lower hull
    for &idx in &unique {
        while stack.len() >= 2 {
            let q = stack[stack.len() - 1];
            let p = stack[stack.len() - 2];
            if cross_2d(&pts[p], &pts[q], &pts[idx]) <= 0.0 {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(idx);
    }

    // upper hull
    let lower_len = stack.len();
    for &idx in unique.iter().rev().skip(1) { // 마지막은 이미 lower에 포함
        while stack.len() > lower_len {
            let q = stack[stack.len() - 1];
            let p = stack[stack.len() - 2];
            if cross_2d(&pts[p], &pts[q], &pts[idx]) <= 0.0 {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(idx);
    }

    // 마지막 점은 시작점과 같으므로 제거
    if stack.len() > 1 {
        stack.pop();
    }

    // 4) 결과 hull / indices 채우기
    for &idx in &stack {
        hull.push(pts[idx]);
        if let Some(ind) = &mut indices {
            ind.push(idx);
        }
    }

    // 5) 차원 판정
    let dim = if hull.len() == 1 {
        0
    } else if hull.len() == 2 {
        // 모든 점이 colinear 라면 결국 끝점 2개만 남는다
        1
    } else {
        2
    };

    dim
}
```
```rust

#[cfg(test)]
mod tests_convex_hull_2d {
    use nurbslib::core::convex_poly::convex_hull_2d;
    use nurbslib::core::geom::Point2D;

    fn p2(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }

    /// 헬퍼: (x,y) 쌍을 정렬해서 set 비교
    fn sort_xy(v: &mut Vec<Point2D>) {
        v.sort_by(|a, b| {
            match a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal) {
                std::cmp::Ordering::Equal => {
                    a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal)
                }
                o => o,
            }
        });
    }
```
```rust
    #[test]
    fn convex_hull_2d_rectangle_with_inner_points() {
        // 축 정렬 사각형 꼭지점 + 내부 점들 & 중복 점
        let pts = vec![
            p2(0.0, 0.0),
            p2(1.0, 0.0),
            p2(1.0, 1.0),
            p2(0.0, 1.0),
            p2(0.5, 0.5), // 내부
            p2(0.25, 0.25),
            p2(0.75, 0.75),
            p2(1.0, 0.0), // 중복
        ];

        let mut hull = Vec::new();
        let mut indices = Vec::new();

        let dim = convex_hull_2d(&pts, &mut hull, Some(&mut indices));

        assert_eq!(dim, 2, "rectangle hull dimension must be 2");

        // hull 이 사각형의 네 꼭지점만 포함하는지 (순서는 상관 없음)
        sort_xy(&mut hull);
        let mut expected = vec![p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)];
        sort_xy(&mut expected);

        assert_eq!(
            hull.len(),
            expected.len(),
            "hull vertex count mismatch, got {} vs {}",
            hull.len(),
            expected.len()
        );

        for (a, b) in hull.iter().zip(expected.iter()) {
            assert!(
                (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9,
                "hull vertex {:?} does not match expected {:?}",
                a,
                b
            );
        }
    }
```
```rust
    #[test]
    fn convex_hull_2d_colinear_points() {
        // x축 위에 5개 점 (colinear)
        let pts = vec![
            p2(-1.0, 0.0),
            p2(0.0, 0.0),
            p2(1.0, 0.0),
            p2(0.5, 0.0),
            p2(-0.5, 0.0),
        ];

        let mut hull = Vec::new();
        let mut indices = Vec::new();

        let dim = convex_hull_2d(&pts, &mut hull, Some(&mut indices));

        // 1D (선분) 이라고 판단해야 한다.
        assert_eq!(dim, 1, "colinear points should produce 1D hull");

        // 끝점 두 개만 남는 것이 정상
        assert_eq!(
            hull.len(),
            2,
            "colinear hull should have 2 vertices, got {}",
            hull.len()
        );

        sort_xy(&mut hull);
        assert!(
            (hull[0].x + 1.0).abs() < 1e-9 && hull[0].y.abs() < 1e-9,
            "first hull vertex must be (-1,0), got {:?}",
            hull[0]
        );
        assert!(
            (hull[1].x - 1.0).abs() < 1e-9 && hull[1].y.abs() < 1e-9,
            "second hull vertex must be (1,0), got {:?}",
            hull[1]
        );
    }
```
```rust
    #[test]
    fn convex_hull_2d_single_point() {
        let pts = vec![p2(1.23, -4.56)];

        let mut hull = Vec::new();
        let mut indices = Vec::new();

        let dim = convex_hull_2d(&pts, &mut hull, Some(&mut indices));

        assert_eq!(dim, 0, "single point hull dimension must be 0");
        assert_eq!(hull.len(), 1, "hull should contain exactly one point");

        assert!(
            (hull[0].x - 1.23).abs() < 1e-9 && (hull[0].y + 4.56).abs() < 1e-9,
            "hull[0] must equal input point"
        );
    }
```
```rust
    #[test]
    fn convex_hull_2d_duplicate_points_only() {
        let pts = vec![
            p2(1.0, 1.0),
            p2(1.0, 1.0),
            p2(1.0, 1.0),
            p2(1.0, 1.0),
        ];

        let mut hull = Vec::new();
        let mut indices = Vec::new();

        let dim = convex_hull_2d(&pts, &mut hull, Some(&mut indices));

        // 전부 같은 점이면 결국 0차원 (point)
        assert_eq!(dim, 0);
        assert_eq!(hull.len(), 1);

        assert!(
            (hull[0].x - 1.0).abs() < 1e-9 && (hull[0].y - 1.0).abs() < 1e-9,
            "duplicate hull point must be (1,1)"
        );
    }
}
```

```rust
#[cfg(test)]
mod tests_convex_poly_3d {
    use nurbslib::core::convex_poly::{convex_poly_closest_point_to_point, 
    convex_poly_closest_poly_poly, ConvexHullOwned, ConvexPoly, Index4};
    use nurbslib::core::prelude::{Point3D, Point4D};


    // ------------------------
    // 헬퍼 함수/생성기
    // ------------------------

    fn p3(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    fn dist(a: &Point3D, b: &Point3D) -> f64 {
        let d = *a - *b;
        d.length()
    }

    /// ConvexHullOwned 에 대해, GJK 결과(bary + adex/bdex)에서 최근접점 두 개를 복원
    fn closest_points_from_gjk<P: ConvexPoly>(
        a: &P,
        b: &P,
        adex: &Index4,
        bdex: &Index4,
        bary: &Point4D,
    ) -> (Point3D, Point3D) {
        let pa = a.evaluate(adex, bary);
        let pb = b.evaluate(bdex, bary);
        (pa, pb)
    }
```
```rust
    /// 축정렬 단위 큐브 (min + size)
    fn make_unit_cube(min_x: f64, min_y: f64, min_z: f64, size: f64) -> ConvexHullOwned {
        let mut c = ConvexHullOwned::new();
        let max_x = min_x + size;
        let max_y = min_y + size;
        let max_z = min_z + size;

        c.append_vertex(p3(min_x, min_y, min_z)); // v0
        c.append_vertex(p3(max_x, min_y, min_z)); // v1
        c.append_vertex(p3(max_x, max_y, min_z)); // v2
        c.append_vertex(p3(min_x, max_y, min_z)); // v3
        c.append_vertex(p3(min_x, min_y, max_z)); // v4
        c.append_vertex(p3(max_x, min_y, max_z)); // v5
        c.append_vertex(p3(max_x, max_y, max_z)); // v6
        c.append_vertex(p3(min_x, max_y, max_z)); // v7

        c
    }
```
```rust
    // ============================================================
    // 1. Point vs ConvexPoly 테스트
    // ============================================================

    #[test]
    fn point_inside_cube_should_project_to_itself() {
        let cube = make_unit_cube(0.0, 0.0, 0.0, 1.0);
        let p = p3(0.3, 0.4, 0.5); // 내부 점

        let mut dex = Index4::UNSET;
        let mut bary = Point4D::zero();

        // 거리 제한 없이 큰 값 사용
        let ok = convex_poly_closest_point_to_point(&cube, &p, &mut dex, &mut bary, 1e9);
        assert!(ok, "inside point projection must succeed");

        let q = cube.evaluate(&dex, &bary);
        let d = dist(&p, &q);
        assert!(
            d < 1e-9,
            "inside point should project to itself, got dist = {}, q = {:?}",
            d,
            q
        );
    }
```
```rust
    #[test]
    fn point_outside_cube_should_project_to_face_center() {
        let cube = make_unit_cube(0.0, 0.0, 0.0, 1.0);
        // x 방향으로 바깥, y,z는 가운데
        let p = p3(2.0, 0.5, 0.5);

        let mut dex = Index4::UNSET;
        let mut bary = Point4D::zero();
        let ok = convex_poly_closest_point_to_point(&cube, &p, &mut dex, &mut bary, 1e9);
        assert!(ok, "outside point projection must succeed");

        let q = cube.evaluate(&dex, &bary);
        let d = dist(&p, &q);

        assert!(
            (q.x - 1.0).abs() < 1e-6,
            "closest point x must be 1.0, got {}",
            q.x
        );
        assert!(
            (q.y - 0.5).abs() < 1e-6,
            "closest point y must be 0.5, got {}",
            q.y
        );
        assert!(
            (q.z - 0.5).abs() < 1e-6,
            "closest point z must be 0.5, got {}",
            q.z
        );
        assert!(
            (d - 1.0).abs() < 1e-6,
            "distance should be 1.0, got {}",
            d
        );
    }
```
```rust
    #[test]
    fn point_on_cube_surface_should_remain_on_surface() {
        let cube = make_unit_cube(0.0, 0.0, 0.0, 1.0);
        let p = p3(1.0, 0.2, 0.8); // 정확히 x=1 face 위

        let mut dex = Index4::UNSET;
        let mut bary = Point4D::zero();
        let ok = convex_poly_closest_point_to_point(&cube, &p, &mut dex, &mut bary, 1e9);
        assert!(ok);

        let q = cube.evaluate(&dex, &bary);
        let d = dist(&p, &q);
        assert!(
            d < 1e-9,
            "point on surface should project to itself, dist = {}, q = {:?}",
            d,
            q
        );
    }
```
```rust
    // ============================================================
    // 2. ConvexPoly vs ConvexPoly (두 볼록체)
    // ============================================================

    #[test]
    fn two_cubes_separated_distance_two() {
        let a = make_unit_cube(0.0, 0.0, 0.0, 1.0);
        let b = make_unit_cube(3.0, 0.0, 0.0, 1.0); // x로 3만큼 이동: 최근접 거리 2.0

        let mut adex = Index4::UNSET;
        let mut bdex = Index4::UNSET;
        let mut bary = Point4D::zero();

        let ok = convex_poly_closest_poly_poly(&a, &b, &mut adex, &mut bdex, &mut bary, 1e9);
        assert!(ok, "GJK should converge for separated cubes");

        let (pa, pb) = closest_points_from_gjk(&a, &b, &adex, &bdex, &bary);
        let d = dist(&pa, &pb);

        assert!(
            (d - 2.0).abs() < 1e-5,
            "distance between cubes must be 2.0, got {} (pa={:?}, pb={:?})",
            d,
            pa,
            pb
        );
    }
```
```rust
    #[test]
    fn two_cubes_touching_on_face_distance_zero() {
        let a = make_unit_cube(0.0, 0.0, 0.0, 1.0);
        let b = make_unit_cube(1.0, 0.0, 0.0, 1.0); // x=1 face끼리 접촉

        let mut adex = Index4::UNSET;
        let mut bdex = Index4::UNSET;
        let mut bary = Point4D::zero();

        // at_most 을 작게 잡으면 "거의 0 거리"만 허용하는 충돌 체크 느낌
        let ok = convex_poly_closest_poly_poly(&a, &b, &mut adex, &mut bdex, &mut bary, 0.1);
        assert!(ok, "GJK should detect touching cubes");

        let (pa, pb) = closest_points_from_gjk(&a, &b, &adex, &bdex, &bary);
        let d = dist(&pa, &pb);

        assert!(
            d < 1e-6,
            "touching cubes: distance must be ~0, got {} (pa={:?}, pb={:?})",
            d,
            pa,
            pb
        );
    }
```
```rust
    #[test]
    fn two_cubes_overlapping_distance_zero() {
        let a = make_unit_cube(0.0, 0.0, 0.0, 1.0);
        let b = make_unit_cube(0.5, 0.0, 0.0, 1.0); // x 방향으로 0.5 겹침

        let mut adex = Index4::UNSET;
        let mut bdex = Index4::UNSET;
        let mut bary = Point4D::zero();

        let ok = convex_poly_closest_poly_poly(&a, &b, &mut adex, &mut bdex, &mut bary, 0.1);
        assert!(ok, "GJK should detect overlapping cubes");

        let (pa, pb) = closest_points_from_gjk(&a, &b, &adex, &bdex, &bary);
        let d = dist(&pa, &pb);

        assert!(
            d < 1e-6,
            "overlapping cubes: distance must be ~0, got {} (pa={:?}, pb={:?})",
            d,
            pa,
            pb
        );
    }
```
```rust
    // ============================================================
    // 3. Degenerate cases (점, 선분, 삼각형)
    // ============================================================

    #[test]
    fn point_vs_segment() {
        // A: 단일 점 (원점)
        let mut a = ConvexHullOwned::new();
        a.append_vertex(p3(0.0, 0.0, 0.0));

        // B: x축 상의 선분 [1,2]
        let mut b = ConvexHullOwned::new();
        b.append_vertex(p3(1.0, 0.0, 0.0));
        b.append_vertex(p3(2.0, 0.0, 0.0));

        let mut adex = Index4::UNSET;
        let mut bdex = Index4::UNSET;
        let mut bary = Point4D::zero();

        let ok = convex_poly_closest_poly_poly(&a, &b, &mut adex, &mut bdex, &mut bary, 1e9);
        assert!(ok, "GJK should converge for point vs segment");

        let (pa, pb) = closest_points_from_gjk(&a, &b, &adex, &bdex, &bary);
        let d = dist(&pa, &pb);

        assert!(
            (d - 1.0).abs() < 1e-6,
            "distance from origin to segment [1,2] must be 1.0, got {}",
            d
        );
    }
```
```rust
    #[test]
    fn segment_vs_triangle() {
        // segment [(-1,0,0), (1,0,0)]
        let mut a = ConvexHullOwned::new();
        a.append_vertex(p3(-1.0, 0.0, 0.0));
        a.append_vertex(p3(1.0, 0.0, 0.0));

        // triangle in plane y=1 (위쪽)
        let mut b = ConvexHullOwned::new();
        b.append_vertex(p3(0.0, 1.0, -1.0));
        b.append_vertex(p3(1.0, 1.0, 1.0));
        b.append_vertex(p3(-1.0, 1.0, 1.0));

        let mut adex = Index4::UNSET;
        let mut bdex = Index4::UNSET;
        let mut bary = Point4D::zero();

        let ok = convex_poly_closest_poly_poly(&a, &b, &mut adex, &mut bdex, &mut bary, 1e9);
        assert!(ok, "GJK should converge for segment vs triangle");

        let (pa, pb) = closest_points_from_gjk(&a, &b, &adex, &bdex, &bary);
        let d = dist(&pa, &pb);

        // 대략 y 방향으로 1만큼 떨어진 configuration
        assert!(
            (d - 1.0).abs() < 1e-4,
            "segment vs triangle distance must be ~1.0, got {} (pa={:?}, pb={:?})",
            d,
            pa,
            pb
        );
    }
}
```

---

