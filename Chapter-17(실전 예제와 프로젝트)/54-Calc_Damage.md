## 📘 전체 함수 요약 및 수식 정리
### 1. add_array1(a1, a2, s1, s2)
- 기능: 두 벡터의 선형 조합을 계산합니다.
- 수식:

$$
\mathrm{result}=s_1\cdot \vec {a}_1+s_2\cdot \vec {a}_2
$$

### 2. d2xx(x, dx, f, c, k)
- 기능: 2차 미분 항 계산. 외력 $\vec {f}$ 에서 감쇠력 $C\cdot \dot {x}$ 와 강성력 $K\cdot x$ 를 뺀 결과.
- 수식:

$$
\ddot {x}=\vec {f}-C\cdot \dot {x}-K\cdot x
$$


### 3 central_diff(t, y)
#### 🔧 기능
- 시간 배열 $t=[t_0,t_1,\dots ,t_{n-1}]$ 과 벡터 배열

$$
y=[\vec {y}_0,\vec {y}_1,\dots ,\vec {y}_{n-1}]
$$

를 받아,
- 각 시점에서의 미분 벡터 $\frac{d\vec {y}}{dt}$ 를 계산합니다.
- 중앙차분법을 사용하며, 양 끝은 전진/후진 차분으로 처리합니다.

#### 📐 수식 정리
- ✅ 내부 지점 (중앙차분)

$$
\left. \frac{d\vec {y}}{dt}\right| _i=\frac{\vec {y}_{i+1}-\vec {y}_{i-1}}{t_{i+1}-t_{i-1}},\quad \mathrm{for\  }1\leq i\leq n-2
$$

- ✅ 시작점 (전진 차분)

$$
\left. \frac{d\vec {y}}{dt}\right| _0=\frac{\vec {y}_1-\vec {y}_0}{t_1-t_0}
$$

- ✅ 끝점 (후진 차분)

$$
\left. \frac{d\vec {y}}{dt}\right| _{n-1}=\frac{\vec {y}_{n-1}-\vec {y}_{n-2}}{t_{n-1}-t_{n-2}}
$$

#### 📦 출력
- 반환값은 $\left[ \frac{d\vec {y}}{dt}\right] _0,\left[ \frac{d\vec {y}}{dt}\right] _1,\dots ,\left[ \frac{d\vec {y}}{dt}\right] _{n-1}$ 형태의 벡터 배열입니다.
- 각 성분 x,y,z 에 대해 위 수식을 독립적으로 적용합니다.



### 4. cum_trapz(t, dy)
- 기능: 사다리꼴 적분법을 사용하여 누적 적분값을 계산합니다.
- 수식:

$$
y_{i+1}=y_i+\frac{dt}{2}\cdot (dy_i+dy_{i+1})
$$

### 5. rk4ode(time, accel)
- 기능: 3자유도 2차 미분 방정식을 4차 Runge-Kutta 방법으로 적분합니다.
- 수식 (단순화된 형태):

$$
\begin{aligned}\vec {k}_1&=f(t_i,x_i,\dot {x}_i)\\ \quad \vec {k}_2&=f\left( t_i+\frac{dt}{2},x_i+\frac{dt}{2}\vec {k}_1\right) \\ \quad \vec {k}_3&=f\left( t_i+\frac{dt}{2},x_i+\frac{dt}{2}\vec {k}_2\right) \\ \quad \vec {k}_4&=f(t_i+dt,x_i+dt\cdot \vec {k}_3)\\ \quad x_{i+1}&=x_i+\frac{dt}{6}(\vec {k}_1+2\vec {k}_2+2\vec {k}_3+\vec {k}_4)\end{aligned}
$$


### 6. calc_damage(time, data, unit)
- 기능: 시간 및 가속도/속도 데이터를 기반으로 손상 궤적을 계산하고, 최대 손상량(DMax)을 반환합니다.
- 수식:

$$
D(t)=\beta \cdot \| \vec {x}(t)\| _2,\quad D_{\mathrm{max}}=\max _tD(t)
$$

- 여기서 $\vec {x}(t)$ 는 rk4ode로 계산된 손상 궤적
- $\beta$ 는 손상 민감도 계수

### 7. calc_damage_risk(damage)
- 기능: 손상량을 기반으로 위험도(확률)를 계산합니다.
- 수식:

$$
R=1-\exp \left( -\exp \left( a\cdot \ln (0.957\cdot D+0.017)-a\cdot \ln b\right) \right)
$$ 

### 8. calc_ubric(time, data, unit)
- 기능: UBRIC 지표 계산 (속도/가속도 기반)
- 수식:

$$
\mathrm{UBRIC}=\sqrt{T(v_x,a_x)^2+T(v_y,a_y)^2+T(v_z,a_z)^2}
$$

- $T(v,a)=v+(a-v)\cdot e^{-a/v}$

### 9. calc_ubric_risk(ubric)
- 기능: UBRIC 값을 기반으로 위험도(확률)를 계산합니다.
- 수식:

$$
R=1-\exp \left( -\exp \left( a\cdot \ln (1.054\cdot U-0.014)-a\cdot \ln b\right) \right)
$$ 


## 🧠 정리 요약

| 함수 이름           | 설명                     | 수식 표현                                                   |
|--------------------|--------------------------|-------------------------------------------------------------|
| `add_array1`        | 벡터 선형 결합            | $s_1 \vec{a}_1 + s_2 \vec{a}_2$                         |
| `d2xx`             | 외력 기반 가속도 계산     | $\vec{f} - C \dot{x} - Kx$                              |
| `central_diff`     | 중앙차분 미분             | $\frac{y_{i+1} - y_{i-1}}{t_{i+1} - t_{i-1}}$           |
| `cum_trapz`        | 누적 사다리꼴 적분        | $y_{i+1} = y_i + \frac{dt}{2}(dy_i + dy_{i+1})$         |
| `rk4ode`           | 2차계 ODE Runge-Kutta 적분 | (4차 RK 수식 적용)                                          |
| `calc_damage`      | 손상 궤적 및 최대 손상량  | $D_{\mathrm{max}} = \max \beta \| \vec{x}(t) \|$        |
| `calc_damage_risk` | 손상 기반 위험도 계산     | $1 - \exp(-\exp(\ldots))$                               |
| `calc_ubric`       | UBRIC 지표 계산           | $\sqrt{T_x^2 + T_y^2 + T_z^2}$                          |
| `calc_ubric_risk`  | UBRIC 기반 위험도 계산    | $1 - \exp(-\exp(\ldots))$                               |



## 🧠 rk4ode 수식 정리 (3자유도 2차계 ODE)
### 🔧 기능 요약
- 시간 배열 t와 외력(가속도) 배열 a(t)를 받아,
- 감쇠 행렬 $C$, 강성 행렬 $K$ 를 기반으로
- 상태 벡터 $x(t)$, 속도 $\dot {x}(t)$, 가속도 $\ddot {x}(t)$ 를 적분합니다.

### 📐 수식 구성
#### 1. 시스템 방정식

$$
\ddot {x}(t)=f(t,x,\dot {x})=\vec {a}(t)-C\cdot \dot {x}(t)-K\cdot x(t)
$$

#### 2. Runge-Kutta 4차 적분
각 시간 구간 [t_i,t_{i+1}]에 대해:

$$
\begin{aligned}k_1^x&=\dot {x}_i\\ k_1^v&=f(t_i,x_i,\dot {x}_i)\\ \quad k_2^x&=\dot {x}_i+\frac{dt}{2}k_1^v\\ \quad k_2^v&=f\left( t_i+\frac{dt}{2},x_i+\frac{dt}{2}k_1^x,k_2^x\right) \\ \quad k_3^x&=\dot {x}_i+\frac{dt}{2}k_2^v\\ k_3^v&=f\left( t_i+\frac{dt}{2},x_i+\frac{dt}{2}k_2^x,k_3^x\right) \\ \quad k_4^x&=\dot {x}_i+dt\cdot k_3^v\\ \quad k_4^v&=f(t_i+dt,x_i+dt\cdot k_3^x,k_4^x)\\ \quad \end{aligned}
$$

#### 3. 상태 업데이트

$$
\begin{aligned}x_{i+1}&=x_i+dt\cdot \frac{1}{6}(k_1^x+2k_2^x+2k_3^x+k_4^x)\\ \dot {x}_{i+1}&=\dot {x}_i+dt\cdot \frac{1}{6}(k_1^v+2k_2^v+2k_3^v+k_4^v)\end{aligned}
$$


## ✅ 수식 점검 결과

| 함수 이름         | 수식 표현                                                                                   | 설명 |
|------------------|----------------------------------------------------------------------------------------------|------|
| `add_array1`      | $s_1 \vec{a}_1 + s_2 \vec{a}_2$                                                           | 벡터 선형 결합 |
| `d2xx`           | $\vec{f} - C \cdot \dot{x} - K \cdot x$                                                  | 외력에서 감쇠력과 강성력 제거 |
| `central_diff`   | $\frac{y_{i+1} - y_{i-1}}{t_{i+1} - t_{i-1}}$                                             | 중앙차분 미분 |
| `cum_trapz`      | $y_{i+1} = y_i + \frac{dt}{2}(dy_i + dy_{i+1})$                                           | 사다리꼴 적분 |
| `rk4ode`         | $x_{i+1} = x_i + dt \cdot \frac{1}{6}(k_1^x + 2k_2^x + 2k_3^x + k_4^x)$<br>$\dot{x}_{i+1} = \dot{x}_i + dt \cdot \frac{1}{6}(k_1^v + 2k_2^v + 2k_3^v + k_4^v)$ | 4차 Runge-Kutta 적분 |
| `calcDamage`     | $D_{\mathrm{max}} = \max_i \left( \beta \cdot \| \vec{x}_i \| \right)$                    | 손상 궤적의 최대 크기 |
| `calcDamageRisk` | $1 - \exp\left(-\exp\left(\log(0.957D + 0.017) \cdot a - \log(b) \cdot a\right)\right)$   | 손상 기반 위험도 |
| `calcUBRIC`      | $\text{UBRIC} = \sqrt{T_x^2 + T_y^2 + T_z^2}$,<br>$T_j = v_j + (a_j - v_j) \cdot e^{-a_j / v_j}$ | 속도/가속도 기반 UBRIC |
| `calcUBRICRisk`  | $1 - \exp\left(-\exp\left(\log(1.054U - 0.014) \cdot a - \log(b) \cdot a\right)\right)$   | UBRIC 기반 위험도 |



## ✅ 수식 점검 결과: rk4ode 함수

| 항목             | 수식 표현                                                                 |
|------------------|---------------------------------------------------------------------------|
| 시스템 방정식     | $\ddot{x} = a - C \cdot \dot{x} - K \cdot x$                          |
| 평균 외력         | $a_{\mathrm{ave}} = \frac{a_i + a_{i+1}}{2}$                          |
| k₁ 계산          | $k_1^x = \dot{x}_i$, $k_1^v = f(t_i, x_i, \dot{x}_i)$              |
| k₂ 계산          | $k_2^x = \dot{x}_i + \frac{dt}{2} k_1^v$,<br> $k_2^v = f(t_i + \frac{dt}{2}, x_i + \frac{dt}{


## 🧠 정리 요약: rk4ode 함수 수식

| 구성 요소     | 수식 표현                                                                 |
|---------------|----------------------------------------------------------------------------|
| 시스템 방정식 | $\ddot{x} = a(t) - C \cdot \dot{x} - K \cdot x$                        |
| k1 계산       | $k_1^x = \dot{x}_i$, $k_1^v = f(t_i, x_i, \dot{x}_i)$              |
| k2 계산       | $k_2^x = \dot{x}_i + \frac{dt}{2} k_1^v$, $k_2^v = f(t_i + \frac{dt}{2}, x_i + \frac{dt}{2} k_1^x, k_2^x)$ |
| k3 계산       | $k_3^x = \dot{x}_i + \frac{dt}{2} k_2^v$, $k_3^v = f(t_i + \frac{dt}{2}, x_i + \frac{dt}{2} k_2^x, k_3^x)$ |
| k4 계산       | $k_4^x = \dot{x}_i + dt \cdot k_3^v$, $k_4^v = f(t_i + dt, x_i + dt \cdot k_3^x, k_4^x)$ |
| 상태 업데이트 | $x_{i+1} = x_i + dt \cdot \frac{1}{6}(k_1^x + 2k_2^x + 2k_3^x + k_4^x)$ |
|               | $\dot{x}_{i+1} = \dot{x}_i + dt \cdot \frac{1}{6}(k_1^v + 2k_2^v + 2k_3^v + k_4^v)$ |

---

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use crate::core::tvector::{TVector3, TVector3d};
use crate::injury::damage_data::{damage_data, DummyType};

pub type TArrayd = Vec<f64>;
pub type TArrayVec3d = Vec<TVector3d>;


pub struct CSVDamageParser;
```
```rust
impl CSVDamageParser {
    /// return: (time, data[xyz], unit[3])
    /// unit[0] = 시간 스케일(1 또는 0.001)
    /// unit[1] = 속도/가속도 플래그 (속도=1, 가속도=0)
    /// unit[2] = 데이터 스케일 계수
    pub fn parse<P: AsRef<Path>>(path: P) -> std::io::Result<(TArrayd, TArrayVec3d, TVector3d)> {
        let file = File::open(path)?;
        let mut rdr = BufReader::new(file);

        // -------- 1) 첫 줄: 더미 타입 (예: "Gender, H305")
        let mut line = String::new();
        if rdr.read_line(&mut line)? == 0 {
            return Ok((vec![], vec![], TVector3d::default()));
        }
        let fields = split_keep_empty(&line);
        if fields.len() < 2 {
            return Ok((vec![], vec![], TVector3d::default()));
        }
        let dummy_str = fields[1].trim();
        {
            let mut gd = damage_data().write().unwrap();
            match dummy_str {
                "H305" => gd.set_dummy_type(DummyType::H305),
                "H350" => gd.set_dummy_type(DummyType::H350),
                _ => { /* 모르면 그대로 둠 */ }
            }
        }

        // -------- 2) 둘째 줄: 헤더 + 단위 괄호
        line.clear();
        if rdr.read_line(&mut line)? == 0 {
            return Ok((vec![], vec![], TVector3d::default()));
        }
        let fields = split_keep_empty(&line);
        if fields.len() != 4 {
            return Ok((vec![], vec![], TVector3d::default()));
        }

        let mut unit: TVector3d = TVector3::new(1.0, 0.0, 1.0); // (time, vel/acc flag, scale)

        // 괄호 안 단위 추출: "(...)" → 그룹 1
        let re = Regex::new(r"\(([a-zA-Z0-9/\^\-]+)\)").unwrap();

        // i==0: time 단위
        if let Some(u) = capture_unit_lower(&fields[0], &re) {
            match u.as_str() {
                "s"  => unit[0] = 1.0,
                "ms" => unit[0] = 0.001,
                _ => {}
            }
        }

        // i==1: 각속도/각가속도 단위
        if let Some(u) = capture_unit_lower(&fields[1], &re) {
            // 속도 계열 → unit[1]=1, 스케일은 rad/s → 내부 rad/s 로 맞춤
            // 가속도 계열 → unit[1]=0, 스케일은 rad/s^2 → 내부 rad/s^2 로 맞춤
            match u.as_str() {
                // --- 속도 계열 ---
                "krad/s"   => { unit[1] = 1.0; unit[2] = 1000.0; }
                "deg/s"    => { unit[1] = 1.0; unit[2] = 1.0 / 57.2958; }
                "rad/ms"   => { unit[1] = 1.0; unit[2] = 1000.0; }
                "krad/ms"  => { unit[1] = 1.0; unit[2] = 1_000_000.0; }
                // --- 가속도 계열 ---
                "rad/s/s"  => { unit[1] = 0.0; unit[2] = 1.0; }
                "krad/s^2" => { unit[1] = 0.0; unit[2] = 1000.0; }
                "rad/ms^2" => { unit[1] = 0.0; unit[2] = 1_000_000.0; }
                "krad/ms^2"=> { unit[1] = 0.0; unit[2] = 1_000.0 * 1_000_000.0; }
                _ => {}
            }
        }

        // -------- 3) 나머지 라인: 숫자 테이블 (time, x, y, z)
        let mut rows: Vec<Vec<String>> = Vec::new();
        line.clear();
        loop {
            line.clear();
            let n = rdr.read_line(&mut line)?;
            if n == 0 { break; }
            let line_trim = line.trim();
            if line_trim.is_empty() { continue; }
            rows.push(split_keep_empty(line_trim));
        }

        if rows.is_empty() {
            return Ok((vec![], vec![], unit));
        }

        let n_row = rows.len();
        let n_col = rows[0].len(); // 기대: 4 (time, x, y, z)

        let mut time: TArrayd = vec![0.0; n_row];
        let mut data: TArrayVec3d = vec![TVector3d::default(); n_row];

        for (i, items) in rows.iter_mut().enumerate() {
            fn parse_f64_or_zero(s: &str) -> f64 {
                let s = s.trim();
                if s.is_empty() { 0.0 } else { s.parse::<f64>().unwrap_or(0.0) }
            }
            if n_col >= 1 { time[i]    = parse_f64_or_zero(&items[0]); }
            if n_col >= 2 { data[i][0] = parse_f64_or_zero(&items[1]); }
            if n_col >= 3 { data[i][1] = parse_f64_or_zero(&items[2]); }
            if n_col >= 4 { data[i][2] = parse_f64_or_zero(&items[3]); }
        }

        Ok((time, data, unit))
    }
}
```
```rust
// 콤마 분할 (빈 항목 유지) + trim
fn split_keep_empty(s: &str) -> Vec<String> {
    s.split(',')
        .map(|x| x.trim().to_string())
        .collect()
}
```
```rust
// "(...)" 안의 소문자 단위 문자열 반환
fn capture_unit_lower(s: &str, re: &Regex) -> Option<String> {
    if let Some(caps) = re.captures(s) {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str().trim().to_ascii_lowercase());
        }
    }
    None
}
```


```rust
use crate::core::injury::damage_data::damage_data;
use crate::core::tvector::TVector3d;

#[inline]
fn add_array1(a1: &TVector3d, a2: &TVector3d, s1: f64, s2: f64) -> TVector3d {
    TVector3d {
        x: s1 * a1.x + s2 * a2.x,
        y: s1 * a1.y + s2 * a2.y,
        z: s1 * a1.z + s2 * a2.z,
    }
}
```
```rust
#[inline]
fn d2xx(
    x: &TVector3d,
    dx: &TVector3d,
    f: &TVector3d,
    c: &[[f64; 3]; 3],
    k: &[[f64; 3]; 3],
) -> TVector3d {
    // F - C*dx - K*x
    let cx = TVector3d {
        x: c[0][0] * dx.x + c[0][1] * dx.y + c[0][2] * dx.z,
        y: c[1][0] * dx.x + c[1][1] * dx.y + c[1][2] * dx.z,
        z: c[2][0] * dx.x + c[2][1] * dx.y + c[2][2] * dx.z,
    };
    let kx = TVector3d {
        x: k[0][0] * x.x + k[0][1] * x.y + k[0][2] * x.z,
        y: k[1][0] * x.x + k[1][1] * x.y + k[1][2] * x.z,
        z: k[2][0] * x.x + k[2][1] * x.y + k[2][2] * x.z,
    };
    TVector3d {
        x: f.x - cx.x - kx.x,
        y: f.y - cx.y - kx.y,
        z: f.z - cx.z - kx.z,
    }
}
```
```rust
/// 중앙차분: accel이 필요할 때 vel → accel, pos → vel 등으로 사용
fn central_diff(t: &[f64], y: &[TVector3d]) -> Vec<TVector3d> {
    assert_eq!(t.len(), y.len());
    let n = t.len();
    let mut res = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];

    if n == 0 {
        return res;
    }
    if n == 1 {
        return res; // 미분 불가 → 0
    }

    // C++ 구현과 동일 규칙(양 끝 1차 차분, 내부 중앙차분)
    // 원문은 dt/dy를 조금 특이하게 만들었는데, 의미상 중앙차분을 충실히 재현
    res[0] = {
        let dt = t[1] - t[0];
        if dt.abs() > 0.0 {
            TVector3d {
                x: (y[1].x - y[0].x) / dt,
                y: (y[1].y - y[0].y) / dt,
                z: (y[1].z - y[0].z) / dt,
            }
        } else {
            TVector3d { x: 0.0, y: 0.0, z: 0.0 }
        }
    };
    for i in 1..(n - 1) {
        let dt = t[i + 1] - t[i - 1];
        if dt.abs() > 0.0 {
            res[i] = TVector3d {
                x: (y[i + 1].x - y[i - 1].x) / dt,
                y: (y[i + 1].y - y[i - 1].y) / dt,
                z: (y[i + 1].z - y[i - 1].z) / dt,
            }
        } else {
            res[i] = TVector3d { x: 0.0, y: 0.0, z: 0.0 };
        }
    }
    res[n - 1] = {
        let dt = t[n - 1] - t[n - 2];
        if dt.abs() > 0.0 {
            TVector3d {
                x: (y[n - 1].x - y[n - 2].x) / dt,
                y: (y[n - 1].y - y[n - 2].y) / dt,
                z: (y[n - 1].z - y[n - 2].z) / dt,
            }
        } else {
            TVector3d { x: 0.0, y: 0.0, z: 0.0 }
        }
    };
    res
}
```
```rust
/// 누적 사다리꼴 적분 (벡터 3성분)
fn cum_trapz(t: &[f64], dy: &[TVector3d]) -> Vec<TVector3d> {
    assert_eq!(t.len(), dy.len());
    let n = t.len();
    let mut y = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];
    for i in 0..(n - 1) {
        let dt = t[i + 1] - t[i];
        y[i + 1].x = y[i].x + 0.5 * dt * (dy[i + 1].x + dy[i].x);
        y[i + 1].y = y[i].y + 0.5 * dt * (dy[i + 1].y + dy[i].y);
        y[i + 1].z = y[i].z + 0.5 * dt * (dy[i + 1].z + dy[i].z);
    }
    y
}
```
```rust
/// 3자유도 2차계 ODE를 4차 Runge-Kutta로 적분
fn rk4ode(time: &[f64], accel: &[TVector3d]) -> Vec<TVector3d> {
    let n = time.len();

    // DamageData 파라미터 로드
    let gd = damage_data().read().unwrap();
    let a1 = gd.get_dummy_damage_param(0);
    let kxx = gd.get_dummy_damage_param(1);
    let kyy = gd.get_dummy_damage_param(2);
    let kzz = gd.get_dummy_damage_param(3);
    let kxz = gd.get_dummy_damage_param(4);

    // kxy, kyz 는 0
    let kxy = 0.0;
    let kyz = 0.0;

    let k = [
        [kxx + kxy + kxz, -kxy, -kxz],
        [-kxy, kxy + kyy + kyz, -kyz],
        [-kxz, -kyz, kxz + kyz + kzz],
    ];
    let mut c = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = a1 * k[i][j];
        }
    }

    // 상태 벡터: x, dx, d2x
    let mut x = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];
    let mut dx = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];
    let mut d2x = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];

    for i in 1..n {
        let dt = time[i] - time[i - 1];
        let xi = x[i - 1];
        let dxi = dx[i - 1];
        let ac1 = accel[i - 1];
        let ac2 = accel[i];
        let ac_ave = TVector3d {
            x: 0.5 * (ac1.x + ac2.x),
            y: 0.5 * (ac1.y + ac2.y),
            z: 0.5 * (ac1.z + ac2.z),
        };

        let k1x = dxi;
        let k1v = d2xx(&xi, &k1x, &ac1, &c, &k);

        let k2x = add_array1(&dxi, &k1v, 1.0, 0.5 * dt);
        let k2v = d2xx(&add_array1(&xi, &k1x, 1.0, 0.5 * dt), &k2x, &ac_ave, &c, &k);

        let k3x = add_array1(&dxi, &k2v, 1.0, 0.5 * dt);
        let k3v = d2xx(&add_array1(&xi, &k2x, 1.0, 0.5 * dt), &k3x, &ac_ave, &c, &k);

        let k4x = add_array1(&dxi, &k3v, 1.0, dt);
        let k4v = d2xx(&add_array1(&xi, &k3x, 1.0, dt), &k4x, &ac2, &c, &k);

        let kx12 = add_array1(&k1x, &k2x, 1.0, 2.0);
        let kx34 = add_array1(&k3x, &k4x, 2.0, 1.0);
        let kx = add_array1(&kx12, &kx34, 1.0 / 6.0, 1.0 / 6.0);

        let kv12 = add_array1(&k1v, &k2v, 1.0, 2.0);
        let kv34 = add_array1(&k3v, &k4v, 2.0, 1.0);
        let kv = add_array1(&kv12, &kv34, 1.0 / 6.0, 1.0 / 6.0);

        d2x[i] = kv;
        dx[i] = TVector3d {
            x: dxi.x + kv.x * dt,
            y: dxi.y + kv.y * dt,
            z: dxi.z + kv.z * dt,
        };
        x[i] = TVector3d {
            x: xi.x + kx.x * dt,
            y: xi.y + kx.y * dt,
            z: xi.z + kx.z * dt,
        };
    }

    x
}
```
```rust
/// time, data 길이 일치 필수.
/// unit = [time_scale, flag(0=가속도, !=0=속도→가속도), accel_scale]
pub fn calc_damage(
    time: &[f64],
    data: &[TVector3d],
    unit: &TVector3d,
) -> (bool, f64, Vec<TVector3d>) {
    if time.len() != data.len() {
        return (false, -1.0, Vec::new());
    }
    let n = time.len();

    // 단위 변환
    let mut t_scaled = vec![0.0; n];
    let mut data_scaled = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];
    for i in 0..n {
        t_scaled[i] = unit.x * time[i];
        data_scaled[i] = TVector3d {
            x: unit.z * data[i].x,
            y: unit.z * data[i].y,
            z: unit.z * data[i].z,
        };
    }

    // accel 준비
    let accel = if unit.y != 0.0 {
        // 속도 제공 → 중앙차분으로 가속도 생성
        central_diff(&t_scaled, &data_scaled)
    } else {
        // 이미 가속도로 간주
        data_scaled
    };

    // 계 해석
    let dmg_traj = rk4ode(&t_scaled, &accel);

    // DMax = beta * |x|_2 의 최대 (beta = damage_param[5])
    let beta = damage_data().read().unwrap().get_dummy_damage_param(5);
    let mut dmax = f64::NEG_INFINITY;
    for v in &dmg_traj {
        let di = beta * (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        if di > dmax {
            dmax = di;
        }
    }

    (true, dmax, dmg_traj)
}
```
```rust
pub fn calc_damage_risk(damage: f64) -> TVector3d {
    // x = 1 - exp(-exp( log(0.957*D + 0.017)*a - log(b)*a ))
    let a = [4.078, 3.875, 6.051];
    let b = [0.394, 0.459, 0.646];
    let g = 0.957 * damage + 0.017;

    let mut out = TVector3d { x: 0.0, y: 0.0, z: 0.0 };
    let gg = g.max(f64::MIN_POSITIVE); // 로그 안정화
    let ln_gg = gg.ln();
    for (i, (ai, bi)) in a.iter().zip(b.iter()).enumerate() {
        let val = 1.0 - (-((ln_gg * ai) - (bi as &f64).ln() * ai).exp()).exp();
        match i {
            0 => out.x = val,
            1 => out.y = val,
            _ => out.z = val,
        }
    }
    out
}
```
```rust
pub fn calc_ubric_risk(ubric: f64) -> TVector3d {
    // x = 1 - exp(-exp( log(1.054*U - 0.014)*a - log(b)*a ))
    let a = [4.078, 3.875, 6.051];
    let b = [0.394, 0.459, 0.646];
    let g = 1.054 * ubric - 0.014;

    let mut out = TVector3d { x: 0.0, y: 0.0, z: 0.0 };
    let gg = if g > 0.0 { g } else { f64::MIN_POSITIVE };
    let ln_gg = gg.ln();
    for (i, (ai, bi)) in a.iter().zip(b.iter()).enumerate() {
        let val = 1.0 - (-((ln_gg * ai) - (bi as &f64).ln() * ai).exp()).exp();
        match i {
            0 => out.x = val,
            1 => out.y = val,
            _ => out.z = val,
        }
    }
    out
}
```
```rust
/// unit = [time_scale, flag(0=가속도 입력, !=0=속도 입력), scale(가속/속도 공용 스케일)]
pub fn calc_ubric(time: &[f64], data: &[TVector3d], unit: &TVector3d) -> f64 {
    assert_eq!(time.len(), data.len());
    let n = time.len();

    // 단위 적용
    let mut t_scaled = vec![0.0; n];
    let mut kin_scaled = vec![TVector3d { x: 0.0, y: 0.0, z: 0.0 }; n];
    for i in 0..n {
        t_scaled[i] = unit.x * time[i];
        kin_scaled[i] = TVector3d {
            x: unit.z * data[i].x,
            y: unit.z * data[i].y,
            z: unit.z * data[i].z,
        };
    }

    let accel: Vec<TVector3d>;
    let vel: Vec<TVector3d>;

    if unit.y == 0.0 {
        // 입력이 가속도 → 적분해서 속도 생성
        accel = kin_scaled.clone();
        vel = cum_trapz(&t_scaled, &accel);
    } else {
        // 입력이 속도 → 중앙차분으로 가속도 생성
        vel = kin_scaled.clone();
        accel = central_diff(&t_scaled, &vel);
    }

    // 최댓값/최솟값 탐색
    let mut max_vel = TVector3d { x: -1.0e8, y: -1.0e8, z: -1.0e8 };
    let mut min_vel = TVector3d { x:  1.0e8, y:  1.0e8, z:  1.0e8 };
    let mut max_acc = TVector3d { x: -1.0e8, y: -1.0e8, z: -1.0e8 };

    for i in 0..n {
        let v = vel[i];
        let a = accel[i];
        max_vel.x = max_vel.x.max(v.x);
        max_vel.y = max_vel.y.max(v.y);
        max_vel.z = max_vel.z.max(v.z);
        min_vel.x = min_vel.x.min(v.x);
        min_vel.y = min_vel.y.min(v.y);
        min_vel.z = min_vel.z.min(v.z);
        max_acc.x = max_acc.x.max(a.x.abs());
        max_acc.y = max_acc.y.max(a.y.abs());
        max_acc.z = max_acc.z.max(a.z.abs());
    }

    let gd = damage_data().read().unwrap();
    let wxcr = [gd.get_dummy_ubric_param(0), gd.get_dummy_ubric_param(1), gd.get_dummy_ubric_param(2)];
    let axcr = [gd.get_dummy_ubric_param(3), gd.get_dummy_ubric_param(4), gd.get_dummy_ubric_param(5)];

    let velx = (max_vel.x - min_vel.x) / wxcr[0];
    let vely = (max_vel.y - min_vel.y) / wxcr[1];
    let velz = (max_vel.z - min_vel.z) / wxcr[2];
    let accx = max_acc.x / axcr[0];
    let accy = max_acc.y / axcr[1];
    let accz = max_acc.z / axcr[2];

    // term = v + (a - v)*exp(-a/v)
    fn term(v: f64, a: f64) -> f64 {
        if v.abs() < f64::EPSILON {
            // v≈0이면 정의역 이슈 → a만 사용 (C++ 에서는 0 나눗셈이 가능해지므로 방지)
            return a;
        }
        v + (a - v) * (-(a / v)).exp()
    }

    let t1 = term(velx, accx);
    let t2 = term(vely, accy);
    let t3 = term(velz, accz);

    (t1 * t1 + t2 * t2 + t3 * t3).sqrt()
}
```
```rust
use once_cell::sync::OnceCell;
use std::sync::RwLock;
use crate::core::tvector::{TVector3, TVector3d};
use crate::injury::damage_utils::{calc_damage, calc_damage_risk, calc_ubric, calc_ubric_risk};
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DummyType {
    H305,
    H350,
}
```
```rust
#[derive(Debug, Clone)]
pub struct DamageData {
    dummy_type: DummyType,
    dummy_damage_param: [f64; 6], // a1, kxx, kyy, kzz, kxz, beta
    dummy_ubric_param: [f64; 6], // wxcr, wycr, wzcr, axcr, aycr, azcr
}
```
```rust
impl DamageData {
    fn new() -> Self {
        let mut gd = Self {
            dummy_type: DummyType::H350,
            dummy_damage_param: [0.0; 6],
            dummy_ubric_param: [0.0; 6],
        };
        gd.update_dummy_param();
        gd
    }
```
```rust
    fn update_dummy_param(&mut self) {
        match self.dummy_type {
            DummyType::H350 => {
                self.dummy_damage_param = [
                    5.9148e-3, // a1
                    32142.0,   // kxx
                    23493.0,   // kyy
                    16935.0,   // kzz
                    1636.3,    // kxz
                    2.9903,    // beta
                ];
                self.dummy_ubric_param = [
                    211.0, 171.0, 115.0, // wxcr, wycr, wzcr
                    20000.0, 10300.0, 7760.0, // axcr, aycr, azcr
                ];
            }
            DummyType::H305 => {
                self.dummy_damage_param = [
                    8.3175e-3, // a1
                    34561.0,   // kxx
                    31940.0,   // kyy
                    25285.0,   // kzz
                    993.98,    // kxz
                    3.5673,    // beta
                ];
                self.dummy_ubric_param = [
                    202.0, 199.0, 149.0, // wxcr, wycr, wzcr
                    19100.0, 18300.0, 14200.0, // axcr, aycr, azcr
                ];
            }
        }
    }
```
```rust
    pub fn dummy_type(&self) -> DummyType {
        self.dummy_type
    }
```
```rust
    pub fn set_dummy_type(&mut self, dt: DummyType) {
        self.dummy_type = dt;
        self.update_dummy_param();
    }
```
```rust
    pub fn get_dummy_damage_param(&self, idx: usize) -> f64 {
        self.dummy_damage_param[idx]
    }

    pub fn get_dummy_ubric_param(&self, idx: usize) -> f64 {
        self.dummy_ubric_param[idx]
    }
```
```rust
    pub fn calc_damage(&self,
        time: &[f64],
        data: &[TVector3d],
        unit: &TVector3d) ->(f64, TVector3d)
    {
        let (ret, max_damage, dmg_risk_vec) = calc_damage(time, data, unit);
        let max_damage_vec = calc_damage_risk(max_damage);
        (max_damage, TVector3d::new(max_damage_vec[0] * 100.0, max_damage_vec[1] * 100.0, max_damage_vec[2] * 100.0))
    }
```
```rust
    pub fn calc_ubric(&self,
                       time: &[f64],
                       data: &[TVector3d],
                       unit: &TVector3d) ->(f64, TVector3d)
    {
        let ubric = calc_ubric(time, data, unit);
        let ubric_risk = calc_ubric_risk(ubric);
        (ubric, TVector3d::new(ubric_risk[0] * 100.0, ubric_risk[1] * 100.0, ubric_risk[2] * 100.0))
    }
}
```
```
```rust
static DAMAGE: OnceCell<RwLock<DamageData>> = OnceCell::new();

pub fn damage_data() -> &'static RwLock<DamageData> {
    DAMAGE.get_or_init(|| RwLock::new(DamageData::new()))
}
```

```rust
#[test]
fn parse_minimal_h350_rad_s2() {
    let file_path = "asset/damage_data.csv";
    let (time, data, unit) = CSVDamageParser::parse(file_path).unwrap();

    // 더미 타입 반영 여부
    let dmg_data = damage_data().read().unwrap();

    let (max_damage, dmg_risk_vec) = dmg_data.calc_damage(&time, data.as_slice(), &unit);
    println!("max_damage: {:?}", max_damage);
    println!("AIS 1+ Risk: {:?}, AIS 2+ Risk: {:?}, AIS 4+ Risk:  {:?}", dmg_risk_vec[0], dmg_risk_vec[1], dmg_risk_vec[2]);

    let (u_bric, ubric_risk) = dmg_data.calc_ubric(&time, data.as_slice(), &unit);
    println!("u_bric: {:?}", u_bric);
    println!("AIS 1+ Risk: {:?}, AIS 2+ Risk: {:?}, AIS 4+ Risk:  {:?}", ubric_risk[0], ubric_risk[1], ubric_risk[2]);
}
```
---
