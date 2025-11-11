# on_extract_signed_regions
아래는 on_extract_signed_regions 함수의 수학적 원리, 단계별 수식, 그리고 소스 코드 흐름 설명을 정리한 내용입니다.  
이 함수는 2D grid에서 양수/음수로 나뉜 영역을 탐색하고, 각 영역의 경계 점만 추출하는 알고리즘입니다.

## 소스
```rust
pub fn on_extract_signed_regions(grid: &Vec<Vec<f64>>) -> Vec<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if visited[i][j] {
                continue;
            }
            let sign = grid[i][j] >= 0.0;
            let mut region = Vec::new();
            let mut stack = vec![(i, j)];

            while let Some((r, c)) = stack.pop() {
                if r >= rows || c >= cols || visited[r][c] || (grid[r][c] >= 0.0) != sign {
                    continue;
                }
                visited[r][c] = true;
                region.push((r, c));

                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nr = r.wrapping_add(dr as usize);
                    let nc = c.wrapping_add(dc as usize);
                    if nr < rows && nc < cols {
                        stack.push((nr, nc));
                    }
                }
            }

            // 경계 점 추출
            let mut boundary = Vec::new();
            for &(r, c) in &region {
                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nr = r.wrapping_add(dr as usize);
                    let nc = c.wrapping_add(dc as usize);
                    if nr >= rows || nc >= cols || !region.contains(&(nr, nc)) {
                        boundary.push((r, c));
                        break;
                    }
                }
            }

            boundary.sort(); // Top → Left
            regions.push(boundary);
        }
    }

    regions
}
```


## 🧠 수학적 원리 요약
### 1. 부호 기반 영역 분할
- 각 셀의 중심값 v_{i,j}가 양수 또는 음수인지 판단
- 같은 부호끼리 연결된 셀들을 하나의 영역으로 묶음
### 2. 연결성 기준
- 기본은 4방향 연결성: 상, 하, 좌, 우
- 확장 가능: 8방향 연결성 (대각선 포함)
### 3. 경계 셀 판정
- 셀 (i,j)가 영역에 속하면서,
- 인접한 셀 중 하나라도 다른 부호이거나 영역 밖이면 → 경계 셀

## ✏️ 핵심 수식
### 🔹 연결성 판정

$$
\mathrm{neighbor}(i,j)=\{ (i\pm 1,j),(i,j\pm 1)\}
$$

$$
\mathrm{same\_ sign}(i,j)=\mathrm{sign}(v_{i,j})=\mathrm{sign}(v_{n,m})
$$

### 🔹 경계 셀 조건

$$
\exists (n,m)\in \mathrm{neighbor}(i,j)\mathrm{\  such\  that\  }(n,m)\notin \mathrm{region}\Rightarrow (i,j)\in \mathrm{boundary}
$$

## 🧩 단계별 소스 흐름 설명
### 1. 초기화
```rust
let mut visited = vec![vec![false; cols]; rows];
let mut regions = Vec::new();
```
- 방문 여부를 저장할 2D 배열 생성
- 결과 영역 리스트 초기화

### 2. 영역 탐색 (Flood Fill)
```rust
for i in 0..rows {
    for j in 0..cols {
        if visited[i][j] { continue; }
        let sign = grid[i][j] >= 0.0;
        let mut region = Vec::new();
        let mut stack = vec![(i, j)];
```
- 아직 방문하지 않은 셀을 시작점으로
- 같은 부호의 셀들을 DFS/BFS로 탐색

### 3. 셀 방문 및 연결
```rust
while let Some((r, c)) = stack.pop() {
    if visited[r][c] || (grid[r][c] >= 0.0) != sign { continue; }
    visited[r][c] = true;
    region.push((r, c));
    for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
        ...
    }
}
```
- 연결된 셀들을 하나의 region으로 묶음
- 4방향 이웃 기준으로 확장

### 4. 경계 셀 추출
```rust
for &(r, c) in &region {
    for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
        let nr = r.wrapping_add(dr as usize);
        let nc = c.wrapping_add(dc as usize);
        if nr >= rows || nc >= cols || !region.contains(&(nr, nc)) {
            boundary.push((r, c));
            break;
        }
    }
}
```

- 각 셀의 이웃 중 하나라도 영역 밖이면 → 경계 셀로 간주
- boundary 리스트에 추가

#### 5. 정렬 및 결과 반환
```rust
boundary.sort(); // Top → Left
regions.push(boundary);
```

- 경계 셀들을 (i,j) 기준으로 정렬
- 최종적으로 Vec<Vec<(usize, usize)>> 형태로 반환

### ✅ 출력 예시
```rust
[
  [(0,0), (0,1), ...], // 음수 영역 경계
  [(3,3), (3,4), ...], // 양수 영역 경계
  [(6,6)]              // 고립된 셀
]
```

![Extract Region Pont](/image/extract_region.png)


## 🧠 핵심 아이디어: 경계 셀 연결 순서 추적
- 입력: Vec<(usize, usize)> — 경계 셀 리스트
- 출력: Vec<(usize, usize)> — 연결된 loop 순서로 정렬된 경계 셀
- 방법:
- 시작 셀에서 시작
- 인접한 셀 중 아직 사용되지 않은 셀을 선택
- 연결된 순서대로 리스트를 구성
- 모든 셀을 사용할 때까지 반복

```rust
fn on_sort_boundary_loop(boundary: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut used = HashSet::new();
    let mut looped = Vec::new();

    // 시작점: 가장 위쪽, 왼쪽 셀
    let mut current = *boundary.iter().min().unwrap();
    looped.push(current);
    used.insert(current);

    while looped.len() < boundary.len() {
        let (r, c) = current;
        let neighbors = [
            (r.wrapping_sub(1), c),     // 위
            (r + 1, c),                 // 아래
            (r, c.wrapping_sub(1)),     // 왼쪽
            (r, c + 1),                 // 오른쪽
            (r.wrapping_sub(1), c.wrapping_sub(1)), // 좌상
            (r.wrapping_sub(1), c + 1), // 우상
            (r + 1, c.wrapping_sub(1)), // 좌하
            (r + 1, c + 1),             // 우하
        ];

        // 인접한 경계 셀 중 아직 사용되지 않은 셀 선택
        if let Some(&next) = neighbors.iter()
            .filter(|&&n| boundary.contains(&n) && !used.contains(&n))
            .next()
        {
            looped.push(next);
            used.insert(next);
            current = next;
        } else {
            break; // 더 이상 연결할 셀이 없으면 종료
        }
    }

    looped
}
```

## ✅ 사용 예시
```rust
let boundary = vec![(3,3), (3,4), (4,3), (4,4), (4,5), (3,5)];
let looped = sort_boundary_loop(&boundary);

for (r, c) in looped {
    println!("({}, {})", r, c);
}
```
![Extract Region Pont](/image/extract_region_line.png)

## 📌 주의사항
- 이 알고리즘은 단일 loop를 구성하는 경계 셀에 적합합니다
- 경계가 분리된 여러 loop일 경우, 각 loop마다 따로 적용해야 합니다
- 성능이 중요할 경우, HashSet 대신 BitSet이나 Vec<bool>로 최적화 가능




