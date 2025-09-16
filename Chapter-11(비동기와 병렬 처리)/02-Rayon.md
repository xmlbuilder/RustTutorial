# Rayon
Rust에서 Rayon을 활용한 병렬 처리에 대한 완전 정리.

## ⚡ Rayon: Rust의 병렬 처리 라이브러리
### 1️⃣ 개요
- Rayon은 Rust에서 데이터 병렬화를 쉽게 구현할 수 있도록 설계된 라이브러리입니다.
- Rust의 이터레이터 API와 자연스럽게 통합되어, 기존 순차 코드에 병렬성을 쉽게 추가할 수 있습니다.
- 내부적으로 워크 스틸링 기반 스레드 풀을 사용하여 효율적인 병렬 실행을 지원합니다.

### 2️⃣ 설치 및 선언
cargo add rayon


use rayon::prelude::*;


- prelude::*를 통해 par_iter, par_sort, par_iter_mut 등 병렬 API를 사용할 수 있습니다.

### 3️⃣ 핵심 기능 요약
| 기능              | 설명 |
|-------------------|------|
| `par_iter()`       | 병렬 읽기 이터레이터. 여러 스레드에서 데이터를 동시에 읽으며 처리 |
| `par_iter_mut()`   | 병렬 가변 이터레이터. 각 원소를 병렬로 수정 가능 |
| `par_sort()`       | 병합 정렬 기반의 병렬 정렬. 대규모 데이터에 효과적 |
| `for_each()`       | 병렬 반복 처리. 각 작업을 병렬로 실행 |
| `.map().sum()`     | 병렬 매핑 후 집계. 계산 집약적인 작업에 적합 |



### 4️⃣ 예제: 병렬 vs 순차 계산 비교
```rust
use rayon::prelude::*;
use std::time::SystemTime;

fn sum_of_squares(input : &Vec<i32>) -> i32 {
    input.par_iter()
        .map(|&i| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            i*i
        })
        .sum()
}

fn sum_of_squares_seq(input : &Vec<i32>) -> i32 {
    input.iter()
        .map(|&i| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            i*i
        })
        .sum()
}

fn main() {

    let start = SystemTime::now();
    sum_of_squares(&(1..100).collect());
    println!("{}ms", start.elapsed().unwrap().as_millis()); //93ms

    let start = SystemTime::now();
    sum_of_squares_seq(&(1..100).collect());
    println!("{}ms", start.elapsed().unwrap().as_millis()); //1028ms
}

```

#### 코드 요약
```rust
fn sum_of_squares(input: &Vec<i32>) -> i32 {
    input.par_iter()
        .map(|&i| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            i * i
        })
        .sum()
}

fn sum_of_squares_seq(input: &Vec<i32>) -> i32 {
    input.iter()
        .map(|&i| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            i * i
        })
        .sum()
}
```

- 병렬 실행: 약 90ms
- 순차 실행: 약 1000ms
- 단순히 iter() → par_iter()로 바꾸는 것만으로 병렬화 가능

### 5️⃣ 예제: 병렬 가변 처리 (par_iter_mut)
```rust
use rayon::prelude::*;
use std::time::SystemTime;

fn plus_one(x: &mut i32){
    *x += 1;
    std::thread::sleep(std::time::Duration::from_millis(10));
}

fn increment_all_seq(input: &mut [i32]){
    input.iter_mut().for_each(plus_one);
}

fn increment_all(input: &mut [i32]){
    input.par_iter_mut().for_each(plus_one);
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    let start = SystemTime::now();
    increment_all(&mut data);
    println!("{:?} - {}ms", data, start.elapsed().unwrap().as_millis()); //[2, 3, 4, 5, 6] - 11ms

    let start = SystemTime::now();
    increment_all_seq(&mut data);
    println!("{:?} - {}ms", data, start.elapsed().unwrap().as_millis()); //[3, 4, 5, 6, 7] - 51ms
}

```

#### 코드 요약
```rust
fn plus_one(x: &mut i32) {
    *x += 1;
    std::thread::sleep(std::time::Duration::from_millis(10));
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(plus_one);
}
```

- 각 원소를 병렬로 수정
- 순차보다 5배 이상 빠름

### 6️⃣ 예제: 병렬 정렬 (par_sort)
```rust
use rayon::prelude::*;
use std::time::SystemTime;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut data1: Vec<i32> = (0..1_000_000).map(|_| rng.gen_range(0..=100)).collect();

    let mut data2 = data1.clone();

    let start = SystemTime::now();
    data1.par_sort();
    println!("{}ms", start.elapsed().unwrap().as_millis()); //114ms

    let start = SystemTime::now();
    data2.sort();
    println!("{}ms", start.elapsed().unwrap().as_millis()); //780ms

    assert_eq!(data1, data2);
}

```

####  코드 요약
```rust
let mut data1: Vec<i32> = (0..1_000_000).map(|_| rand::thread_rng().gen_range(0..=100)).collect();
let mut data2 = data1.clone();

data1.par_sort(); // 병렬 정렬: 약 114ms
data2.sort();     // 순차 정렬: 약 780ms
```

- par_sort()는 내부적으로 병합 정렬을 병렬로 수행
- 대규모 데이터 정렬에 매우 효과적

### 라이브러리 비교

| 라이브러리 | 주요 용도                        | 최적화 대상             | 특징                          |
|------------|----------------------------------|--------------------------|-------------------------------|
| Rayon      | 병렬 계산                        | CPU 중심 작업            | 이터레이터 기반 병렬 처리, 자동 스레드 풀 |
| Tokio      | 비동기 네트워크 및 IO 처리       | IO 중심 작업             | Future 기반 비동기 실행, 런타임 필요     |



### ✅ 언제 Rayon을 써야 할까?
- 대량의 데이터를 반복 처리할 때
- 계산 중심의 작업을 병렬화하고 싶을 때
- 기존 순차 이터레이터를 병렬로 바꾸고 싶을 때
- 정렬, 필터링, 집계 등 고성능 데이터 처리에 적합

---


# Rayon 확장 

Rayon을 기반으로 확장 가능한 고급 활용 예제입니다.
이미지 처리, 파일 파싱, 멀티코어 벤치마크까지 세 가지 실전 시나리오로 나눠서 설명드릴게요.

## 🖼️ 1. 이미지 처리: 병렬 픽셀 변환
Rayon은 이미지의 각 픽셀을 병렬로 처리하는 데 매우 적합합니다.  
예를 들어, 밝기 조절이나 필터 적용을 병렬로 수행할 수 있습니다.

### ✅ 예제: 이미지 밝기 증가
```rust
use rayon::prelude::*;
use image::{open, DynamicImage, GenericImageView, Rgba};

fn brighten(img: &DynamicImage, amount: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut buffer = img.to_rgba8();

    buffer.par_chunks_mut(4).for_each(|pixel| {
        pixel[0] = pixel[0].saturating_add(amount);
        pixel[1] = pixel[1].saturating_add(amount);
        pixel[2] = pixel[2].saturating_add(amount);
    });

    DynamicImage::ImageRgba8(buffer)
}
```


- par_chunks_mut(4): RGBA 픽셀 단위로 병렬 처리
- saturating_add: 오버플로우 없이 밝기 증가

## 📄 2. 파일 파싱: 병렬 CSV 처리
대량의 CSV 데이터를 병렬로 파싱하고 집계할 수 있습니다. Rayon은 각 줄을 병렬로 처리하는 데 적합합니다.
### ✅ 예제: 병렬로 CSV 행 처리
```rust
use rayon::prelude::*;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("data.csv").unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let total: usize = lines.par_iter()
        .filter_map(|line| line.split(',').nth(2)) // 3번째 열
        .filter_map(|val| val.parse::<usize>().ok())
        .sum();

    println!("Total of column 3: {}", total);
}
```

- par_iter()로 각 행을 병렬 처리
- filter_map으로 유효한 숫자만 집계

## 🧪 3. 멀티코어 벤치마크: 병렬 계산 성능 측정
Rayon은 CPU 코어를 자동으로 활용하므로, 병렬 계산의 성능을 측정하기에 적합합니다.
### ✅ 예제: 병렬 vs 순차 벤치마크
```rust
use rayon::prelude::*;
use std::time::Instant;

fn heavy_computation(x: u64) -> u64 {
    (1..=x).map(|i| i * i).sum()
}

fn main() {
    let data: Vec<u64> = (1..=10_000).collect();

    let start = Instant::now();
    let _seq: u64 = data.iter().map(|&x| heavy_computation(x)).sum();
    println!("Sequential: {:?}", start.elapsed());

    let start = Instant::now();
    let _par: u64 = data.par_iter().map(|&x| heavy_computation(x)).sum();
    println!("Parallel: {:?}", start.elapsed());
}
```

- par_iter()로 병렬 계산
- Instant::now()로 실행 시간 비교

## 🧠 확장 요약
| 확장 분야       | 핵심 기능 또는 API             | 특징                          | 활용 예시                         |
|----------------|-------------------------------|-------------------------------|-----------------------------------|
| 이미지 처리     | `par_chunks_mut` + `image`     | 픽셀 단위 병렬 수정            | 밝기 조절, 필터 적용              |
| 파일 파싱       | `par_iter` + `filter_map`      | 행 단위 병렬 파싱 및 집계      | CSV 분석, 로그 처리               |
| 멀티코어 벤치마크 | `Instant` + `par_iter`         | 병렬 계산 성능 측정            | 알고리즘 비교, 최적화 테스트       |

---


