# Vector chunks_mut 심화
chunks_mut을 사용해서 벡터를 mutable하게 일정 크기로 나눈 후,  
for 루프 없이 특정 chunk를 직접 접근하고 싶다면, 반환되는 값의 타입과 인덱싱 방법을 이해해야 합니다.

## 🔍 chunks_mut의 반환 타입
- chunks_mut(n)은 &mut [T] 슬라이스를 n개씩 나눈 mutable 슬라이스의 iterator인 ChunksMut<'_, T>를 반환합니다.

## ✅ 직접 접근하는 방법
- ChunksMut은 Iterator이기 때문에 직접 인덱싱은 불가능하지만, .collect::<Vec<_>>()로 벡터로 변환하면 인덱싱이 가능합니다.
### 예시:
```rust
let mut data = vec![1, 2, 3, 4, 5, 6];
let mut chunks: Vec<_> = data.chunks_mut(2).collect();

// 이제 chunks[0], chunks[1] 등으로 직접 접근 가능
chunks[0][0] = 10;
chunks[1][1] = 20;
```


## ⚠️ 주의점
- chunks_mut는 슬라이스를 나누는 것이므로, 원본 벡터의 참조를 유지한 채로 조작할 수 있습니다.
- collect를 하면 각 chunk는 여전히 원본 벡터의 부분 슬라이스이므로, 복사본이 아니라 참조입니다.
- 필요하다면 chunks_mut을 .nth(i)로 접근할 수도 있지만, 이건 iterator를 소비하므로 반복 접근에는 적합하지 않습니다.
  
```rust
let mut data = vec![1, 2, 3, 4, 5, 6];
let chunk = data.chunks_mut(2).nth(1); // 두 번째 chunk
```

### 🧪 예제: 행렬에서 chunks_mut로 행 단위 접근
```rust
fn main() {
    let rows = 3;
    let cols = 4;
    let mut matrix = vec![
        1, 2, 3, 4,     // row 0
        5, 6, 7, 8,     // row 1
        9, 10, 11, 12,  // row 2
    ];

    // 행 단위로 mutable하게 접근
    let mut row_chunks: Vec<_> = matrix.chunks_mut(cols).collect();

    // 예: 두 번째 행(row 1)의 첫 번째 값을 변경
    row_chunks[1][0] = 100;

    // 결과 출력
    for row in row_chunks {
        println!("{:?}", row);
    }
}
```

### 🧾 출력 결과
```
[1, 2, 3, 4]
[100, 6, 7, 8]
[9, 10, 11, 12]
```

- 이 방식은 Matrix를 1D 벡터로 저장하면서도 행 단위로 쉽게 접근하고 수정할 수 있게 해줌.
- 만약 열 단위로 접근하고 싶다면 cols 기준으로 인덱싱을 계산해야 합니다.

---
  
