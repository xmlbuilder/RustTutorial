# 🧵 Coons Patch Mesh 생성기 문서 정리
## 📦 구조체 요약

### Vec3f
- 3D 벡터 (x, y, z)
- 벡터 연산 지원: 덧셈, 뺄셈, 스칼라 곱, 내적, 외적, 길이, 정규화
### Vec2f
- 2D 벡터 (x, y)
- 텍스처 좌표용
### CoonsMesh
- Coons 패치 결과를 담는 메쉬 구조체
          - `vertices`: 정점 리스트
          - `faces`: 면 리스트 ([u32; 4] → 삼각형은 마지막 인덱스를 중복)
          - `v_normals`: 정점 노멀
          - `tex_coords`: 텍스처 좌표
### TriStyle
- 삼각형 분할 방식
- AlignLeft, AlignRight, UnionJack
### CoonsOptions
- 패치 생성 옵션
          - `quad_mesh`: 사각형 메쉬 여부
          - `tri_style:` 삼각형 분할 방식
          - `build_normals`: 노멀 생성 여부
          - `build_tex_coord`: 텍스처 좌표 생성 여부
          - `use_arc_len_sampling`: 경계 파라미터를 호장 기반으로 할지 여부
          - `force_corner_match`: 코너 정렬 강제 여부
### CoonsBoundaryMaps
- 경계 파라미터 맵 (UV 및 원곡선 파라미터)

### 🧮 Coons Patch 수식 정리
Coons Patch는 경계 곡선 4개를 기반으로 내부 보간된 표면을 생성합니다.  
수식은 다음과 같습니다:  

$$
P(s, t) = (1 - s) \cdot L(t) + s \cdot R(t) + (1 - t) \cdot B(s) + t \cdot T(s)
          - (1 - s)(1 - t) \cdot C_{00}
          - (1 - s)t \cdot C_{01}
          - s(1 - t) \cdot C_{10}
          - st \cdot C_{11}
$$


- $L(t)$ , $R(t)$: 좌/우 경계 곡선
- $B(s)$ , $T(s)$ : 하/상 경계 곡선
- $C_{ij}$: 네 코너 점
- $s,t\in [0,1]$: 정규화된 파라미터
이 수식은 경계 보간의 합에서 코너 중복을 제거하는 방식으로 작동합니다.

## ⚙️ 주요 함수 설명
### on_build_coons_patch_mesh(...)
- 입력: 4개의 경계 곡선 (bottom, top, left, right)
- 출력: CoonsMesh와 선택적 CoonsBoundaryMaps
- 내부:
    - 정점 계산: Coons 수식 기반
    - 면 생성: 사각형 또는 삼각형
    - 텍스처 좌표 및 노멀 생성 (옵션에 따라)
### recompute_normals(...)
- 각 면의 노멀을 계산하고 정점 노멀을 누적 후 정규화
### coons_into_mesh(...)
- CoonsMesh를 일반적인 Mesh 타입으로 변환

### 🧩 STL 호환 삼각형 처리
삼각형은 [a, b, c, c] 형태로 저장되어 STL 포맷과 호환되도록 구성됩니다.

---

