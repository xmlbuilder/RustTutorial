# Gauss Quad integration
아래는 Gauss Quad 주요 수치적분 함수들의 용도와 수학적 의미를 정리한 내용입니다. 
다양한 가우스형 수치적분 기법을 제공하며, 각각의 함수는 특정한 가중 함수(weight function)와 정의역(domain)에 최적화되어 있습니다.


## 📚 함수별 용도 및 수학적 의미

| 함수 이름       | 적분 형태                                                                 | 정의역             | 가중 함수 또는 조건            |
|----------------|----------------------------------------------------------------------------|--------------------|-------------------------------|
| GaussHermite   | $\int_{-\infty}^{\infty} e^{-x^2} f(x) \, dx$                         |$(-\infty, \infty)$ |$e^{-x^2}$                |
| GaussJacobi    | $\int_{-1}^{1} (1 - x)^\alpha (1 + x)^\beta f(x) \, dx$                |$[-1, 1]$       |$\alpha, \beta > -1$      |
| GaussLaguerre  | $\int_{0}^{\infty} x^\alpha e^{-x} f(x) \, dx$                         |$[0, \infty)$   |$x^\alpha e^{-x}$         |
| Midpoint       | $\int_a^b f(x) \, dx \approx \sum f\left( \frac{x_{i-1} + x_i}{2} \right) \Delta x$ |$[a, b]$        | 등간격 단순 중점 근사         |
| Simpson        | $\int_a^b f(x) \, dx \approx \frac{h}{3}(f_0 + 4f_1 + 2f_2 + \dots + f_n)$ |$[a, b]$        | 2차 다항식 보간 기반          |


## 🧠 수학적 의미 요약
### 1. Gauss-Hermite
- 정의: 무한 구간에서 $e^{-x^2}$ 가중치 하의 적분을 고차 다항식으로 정확히 근사.
- 적용 예시: 확률론(정규분포), 양자역학의 해석적 적분 등.
### 2. Gauss-Jacobi
- 정의: $(1-x)^{\alpha }(1+x)^{\beta }$ 가중치 하의 적분.
- 적용 예시: 베타 분포, 특수 함수 해석, 경계층 문제 등.
### 3. Gauss-Laguerre
- 정의: $x^{\alpha }e^{-x}$ 가중치 하의 무한 구간 적분.
- 적용 예시: 양자역학, 방사성 붕괴, 포아송 분포 관련 적분.
### 4. Midpoint Rule
- 정의: 각 구간의 중간점에서 함수값을 평가하여 적분 근사.
- 특징: 구현이 간단하지만 정확도는 낮음. $O(h^2)$ 오차.
### 5. Simpson’s Rule
- 정의: 2차 다항식으로 보간하여 적분 근사. $O(h^4)$ 정확도.
- 특징: 일반 함수에 대해 높은 정확도 제공. 짝수 개 구간 필요.



## 📐 왜 만들어졌을까?
이 세 가지는 모두 Gaussian Quadrature의 특별한 형태입니다.  
Gaussian Quadrature는 다음과 같은 목적을 가지고 개발되었습니다:
- 다항식 적분의 정확도 극대화: n개의 노드를 사용할 때, 최대 2n-1차 다항식까지 정확하게 적분할 수 있음
- 가중 함수에 따라 최적화: 각 방식은 특정한 가중 함수 w(x)에 대해 정의됨
- 정의역에 따라 구분: 무한 구간, 유한 구간, 양의 실수 구간 등 다양한 적분 구간에 대응



## 🧪 사용 예시: 함수별 코드
### 1. GaussHermite

```rust
use gauss_quad::GaussHermite;

fn main() {
    let quad = GaussHermite::init(10); // 10차 적분
    let result = quad.integrate(|x| x.powi(2)); // ∫ e^{-x^2} x² dx
    println!("Hermite integral ≈ {}", result);
}
```
- 입력: 차수 10, 함수 $f(x)=x^2$
- 출력: 적분 근사값

### 2. GaussJacobi
```rust
use gauss_quad::GaussJacobi;

fn main() {
    let quad = GaussJacobi::init(5, 0.0, 0.0); // Legendre 형태
    let result = quad.integrate(|x| x.sin());
    println!("Jacobi integral ≈ {}", result);
}
```

- 입력: 차수 5, α = 0, β = 0, 함수 $f(x)=\sin (x)$
- 출력: 적분 근사값

### 3. GaussLaguerre
```rust
use gauss_quad::GaussLaguerre;

fn main() {
    let quad = GaussLaguerre::init(4, 0.0); // α = 0
    let result = quad.integrate(|x| x.exp()); // ∫ x^0 e^{-x} e^x dx = ∫ dx
    println!("Laguerre integral ≈ {}", result); // 결과는 무한대에 수렴
}
```

- 입력: 차수 4, α = 0, 함수 $f(x)=e^x$
- 출력: 적분 근사값

### 4. Midpoint
```rust
use gauss_quad::Midpoint;

fn main() {
    let quad = Midpoint::init(100);
    let result = quad.integrate(0.0, 1.0, |x| x * x); // ∫₀¹ x² dx = 1/3
    println!("Midpoint integral ≈ {}", result);
}
```

- 입력: 구간 [0, 1], 분할 수 100, 함수 $f(x)=x^2$
- 출력: 적분 근사값 (≈ 0.333)

### 5. Simpson
```rust
use gauss_quad::Simpson;

fn main() {
    let quad = Simpson::init(4); // 짝수 분할 수
    let result = quad.integrate(0.0, 1.0, |x| x * x);
    println!("Simpson integral ≈ {}", result);
}
```

- 입력: 구간 [0, 1], 분할 수 4, 함수 f(x)=x^2
- 출력: 적분 근사값 (≈ 0.333)

## ✅ 요약: 어떻게 사용할까?
- 적분할 함수를 정의하세요: `|x| x.sin()` 또는 `|x| x.powi(2)`
- 적절한 규칙 선택: 정의역과 가중 함수에 따라 GaussHermite, Jacobi, Laguerre 등 선택
- 차수 또는 분할 수 설정: 정확도를 높이려면 차수를 높이세요
- integrate 함수 호출: 결과는 f(x)에 대한 근사 적분값


### 시각화 자료

```python
# Generating tutorial-style visualization comparing five numerical integration methods
import numpy as np
import matplotlib.pyplot as plt
from scipy.integrate import quad
from numpy.polynomial.hermite import hermgauss
from numpy.polynomial.laguerre import laggauss
from numpy.polynomial.legendre import leggauss

# Define the function to integrate
def f(x):
    return x**2

# True integrals for comparison
true_integrals = {
    "Gauss-Hermite": np.sqrt(np.pi) / 2,  # ∫ x^2 e^{-x^2} dx from -∞ to ∞
    "Gauss-Jacobi": 2 / 3,                # ∫ x^2 dx from -1 to 1
    "Gauss-Laguerre": 2.0,                # ∫ x^2 e^{-x} dx from 0 to ∞
    "Midpoint": 1 / 3,                    # ∫ x^2 dx from 0 to 1
    "Simpson": 1 / 3                      # ∫ x^2 dx from 0 to 1
}

# Number of nodes
n = 10

# Prepare figure
plt.style.use('seaborn-v0_8')
fig, axs = plt.subplots(3, 2, figsize=(14, 12))
axs = axs.flatten()

# Gauss-Hermite
x, w = hermgauss(n)
approx_hermite = np.sum(w * f(x))
axs[0].bar(x, w * f(x), width=0.2, label=f'Approx: {approx_hermite:.6f}\nError: {abs(approx_hermite - true_integrals["Gauss-Hermite"]):.2e}')
axs[0].set_title("Gauss-Hermite")
axs[0].legend()
axs[0].set_xlabel("x")
axs[0].set_ylabel("Weight × f(x)")

# Gauss-Jacobi (α=0, β=0 → Gauss-Legendre)
x, w = leggauss(n)
approx_jacobi = np.sum(w * f(x))
axs[1].bar(x, w * f(x), width=0.2, label=f'Approx: {approx_jacobi:.6f}\nError: {abs(approx_jacobi - true_integrals["Gauss-Jacobi"]):.2e}')
axs[1].set_title("Gauss-Jacobi (Legendre)")
axs[1].legend()
axs[1].set_xlabel("x")
axs[1].set_ylabel("Weight × f(x)")

# Gauss-Laguerre
x, w = laggauss(n)
approx_laguerre = np.sum(w * f(x))
axs[2].bar(x, w * f(x), width=0.5, label=f'Approx: {approx_laguerre:.6f}\nError: {abs(approx_laguerre - true_integrals["Gauss-Laguerre"]):.2e}')
axs[2].set_title("Gauss-Laguerre")
axs[2].legend()
axs[2].set_xlabel("x")
axs[2].set_ylabel("Weight × f(x)")

# Midpoint Rule
a, b = 0.0, 1.0
h = (b - a) / n
midpoints = a + h * (np.arange(n) + 0.5)
approx_midpoint = np.sum(f(midpoints)) * h
axs[3].bar(midpoints, f(midpoints) * h, width=h, label=f'Approx: {approx_midpoint:.6f}\nError: {abs(approx_midpoint - true_integrals["Midpoint"]):.2e}')
axs[3].set_title("Midpoint Rule")
axs[3].legend()
axs[3].set_xlabel("x")
axs[3].set_ylabel("f(x) × h")

# Simpson's Rule
x = np.linspace(0, 1, 2 * n + 1)
y = f(x)
h = (x[1] - x[0])
approx_simpson = (h / 3) * (y[0] + 2 * np.sum(y[2:-1:2]) + 4 * np.sum(y[1::2]) + y[-1])
axs[4].plot(x, y, label='f(x)')
axs[4].fill_between(x, y, alpha=0.3)
axs[4].set_title("Simpson's Rule")
axs[4].legend([f'Approx: {approx_simpson:.6f}\nError: {abs(approx_simpson - true_integrals["Simpson"]):.2e}'])
axs[4].set_xlabel("x")
axs[4].set_ylabel("f(x)")

# Hide unused subplot
axs[5].axis('off')

plt.tight_layout()
output_path = "/mnt/data/integration_methods_comparison.png"
plt.savefig(output_path)
print("Created visualization comparing five numerical integration methods and saved as integration_methods_comparison.png")
```


## 📘 테스트 코드의 목적 요약

| 테스트 모듈            | 대상 타입       | 테스트 목적 설명                                                   |
|------------------------|------------------|---------------------------------------------------------------------|
| `tests_hermite`        | `GaussHermite`   | Hermite 적분 규칙의 노드 및 가중치 정확성 검증, 객체 비교 테스트       |
| `tests_jocobi`         | `GaussJacobi`    | 다양한 α, β 설정에 따른 Jacobi 적분 규칙의 노드/가중치 검증 및 예외 처리 |
| `tests_gauss_laguerre` | `GaussLaguerre`  | Laguerre 적분 규칙의 다양한 α 설정에 대한 노드/가중치 검증             |
| `tests_midpoints`      | `Midpoint`       | 중점법 적분 결과가 기대값과 얼마나 가까운지 확인                       |
| `tests_simpson`        | `Simpson`        | Simpson 규칙 적분 결과가 기대값과 얼마나 정확히 근사하는지 확인        |


## 📐 수학적 의미 및 수식 정리
### 1. Gauss-Hermite
- 적분 형태:

$$
\int _{-\infty }^{\infty }e^{-x^2}f(x)\, dx
$$

- 노드/가중치 계산: Hermite 다항식의 근과 가중치
- 예시 수식:
- 노드: $x_i = Hermite$ 다항식의 근
- 가중치: $w_i=\frac{2^{n-1}n!\sqrt{\pi }}{n^2[H_{n-1}(x_i)]^2}$

### 2. Gauss-Jacobi
- 적분 형태:

$$
\int _{-1}^1(1-x)^{\alpha }(1+x)^{\beta }f(x)\, dx
$$

- 노드/가중치 계산: Jacobi 다항식의 근과 가중치
- 조건: $\alpha$ ,$\beta >-1$
- 예시 수식:
- 노드: $x_i = Jacobi$ 다항식 $P_n^{(\alpha ,\beta )}(x)$ 의 근
- 가중치: $w_i = Golub-Welsch$ 알고리즘으로 계산
### 3. Gauss-Laguerre
- 적분 형태:

$$
\int _0^{\infty }x^{\alpha }e^{-x}f(x)\, dx
$$

- 노드/가중치 계산: Laguerre 다항식의 근과 가중치
- 예시 수식:
- 노드: $x_i = L_n^{(\alpha )}(x)$ 의 근
- 가중치: $w_i=\frac{x_i}{[L_{n+1}^{(\alpha )}(x_i)]^2}$

### 4. Midpoint Rule
- 적분 근사:

$$
\int _a^bf(x)\, dx\approx \sum _{i=1}^nf\left( \frac{x_{i-1}+x_i}{2}\right) \Delta x
$$

- 오차 차수: $O(h^2)$

### 5. Simpson’s Rule
- 적분 근사:


$$
\int _a^bf(x)\, dx\approx \frac{h}{3}\left( f_0+4f_1+2f_2+\dots +f_n\right) 
$$

- 오차 차수: $O(h^4)$

✅ 테스트의 의미
- 정확한 노드와 가중치 검증: 수치적분의 핵심은 노드와 가중치의 정확성입니다. 테스트는 이를 고정된 기대값과 비교하여 검증합니다.
- 객체 동등성 확인: .clone()과 == 비교를 통해 객체가 정확히 복제되고 비교 가능한지 확인합니다.
- 예외 처리 확인: Jacobi에서 α, β가 너무 작을 경우 panic이 발생하는지 테스트합니다.
- 적분 결과 검증: Midpoint와 Simpson은 $\int _0^1x^2dx=\frac{1}{3}$ 을 근사하여 정확도를 확인합니다.

```rust

#[cfg(test)]
mod tests_hermite {
    use gauss_quad::GaussHermite;
    #[test]
    fn golub_welsch_3() {
        let (x, w) = GaussHermite::nodes_and_weights(3);
        let x_should = [1.224_744_871_391_589, 0.0, -1.224_744_871_391_589];
        let w_should = [
            0.295_408_975_150_919_35,
            1.181_635_900_603_677_4,
            0.295_408_975_150_919_35,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-15);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-15);
        }
    }

    #[test]
    fn check_derives() {
        let quad = GaussHermite::init(10);
        let quad_clone = quad.clone();
        assert_eq!(quad, quad_clone);
        let other_quad = GaussHermite::init(3);
        assert_ne!(quad, other_quad);
    }
}
```
```rust
#[cfg(test)]
mod tests_jocobi {
    use gauss_quad::GaussJacobi;
    #[test]
    fn golub_welsch_5_alpha_0_beta_0() {
        let (x, w) = GaussJacobi::nodes_and_weights(5, 0.0, 0.0);
        let x_should = [
            -0.906_179_845_938_664,
            -0.538_469_310_105_683_1,
            0.0,
            0.538_469_310_105_683_1,
            0.906_179_845_938_664,
        ];
        let w_should = [
            0.236_926_885_056_189_08,
            0.478_628_670_499_366_47,
            0.568_888_888_888_888_9,
            0.478_628_670_499_366_47,
            0.236_926_885_056_189_08,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-15);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-15);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_2_alpha_1_beta_0() {
        let (x, w) = GaussJacobi::nodes_and_weights(2, 1.0, 0.0);
        let x_should = [-0.689_897_948_556_635_7, 0.289_897_948_556_635_64];
        let w_should = [1.272_165_526_975_908_7, 0.727_834_473_024_091_3];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-14);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-14);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_5_alpha_1_beta_0() {
        let (x, w) = GaussJacobi::nodes_and_weights(5, 1.0, 0.0);
        let x_should = [
            -0.920_380_285_897_062_6,
            -0.603_973_164_252_783_7,
            0.0,
            0.390_928_546_707_272_2,
            0.802_929_828_402_347_2,
        ];
        let w_should = [
            0.387_126_360_906_606_74,
            0.668_698_552_377_478_2,
            0.585_547_948_338_679_2,
            0.295_635_480_290_466_66,
            0.062_991_658_086_769_1,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-14);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-14);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_5_alpha_0_beta_1() {
        let (x, w) = GaussJacobi::nodes_and_weights(5, 0.0, 1.0);
        let x_should = [
            -0.802_929_828_402_347_2,
            -0.390_928_546_707_272_2,
            0.0,
            0.603_973_164_252_783_7,
            0.920_380_285_897_062_6,
        ];
        let w_should = [
            0.062_991_658_086_769_1,
            0.295_635_480_290_466_66,
            0.585_547_948_338_679_2,
            0.668_698_552_377_478_2,
            0.387_126_360_906_606_74,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-14);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-14);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_50_alpha_42_beta_23() {
        let (x, w) = GaussJacobi::nodes_and_weights(50, 42.0, 23.0);
        let x_should = [
            -0.936_528_233_152_541_2,
            -0.914_340_864_546_088_5,
            -0.892_159_904_972_709_7,
            -0.869_216_909_221_225_6,
            -0.845_277_228_769_225_6,
            -0.820_252_766_348_056_8,
            -0.794_113_540_498_529_6,
            -0.766_857_786_572_463_5,
            -0.738_499_459_607_423_4,
            -0.709_062_235_514_446_8,
            -0.678_576_327_905_629_3,
            -0.647_076_661_181_635_3,
            -0.614_601_751_027_635_6,
            -0.581_192_977_458_508_4,
            -0.546_894_086_695_451_9,
            -0.511_750_831_826_105_3,
            -0.475_810_700_347_493_84,
            -0.439_122_697_460_417_9,
            -0.401_737_165_777_708_5,
            -0.363_705_629_046_518_04,
            -0.325_080_651_686_135_1,
            -0.285_915_708_544_232_9,
            -0.246_265_060_906_733_86,
            -0.206_183_635_819_408_85,
            -0.165_726_906_401_709_62,
            -0.124_950_771_176_147_79,
            -0.083_911_430_566_871_42,
            -0.042_665_258_670_068_65,
            -0.001_268_668_170_195_549_6,
            0.040_222_034_151_539_98,
            0.081_750_804_545_872_01,
            0.123_262_036_301_197_46,
            0.164_700_756_351_269_24,
            0.206_012_852_393_607_17,
            0.247_145_341_670_134_97,
            0.288_046_697_452_241,
            0.328_667_256_796_052_5,
            0.368_959_744_983_174_2,
            0.408_879_971_241_114_4,
            0.448_387_782_372_734_86,
            0.487_448_416_419_391_24,
            0.526_034_498_798_180_8,
            0.564_129_114_046_126_2,
            0.601_730_771_388_207_7,
            0.638_861_919_860_897_4,
            0.675_584_668_752_041_4,
            0.712_032_766_455_434_9,
            0.748_486_131_436_470_7,
            0.785_585_184_777_517_6,
            0.825_241_342_102_355_2,
        ];
        let w_should = [
            7.48575322545471E-18,
            4.368160045795394E-15,
            5.475_092_226_093_74E-13,
            2.883_802_894_000_164_4E-11,
            8.375_974_400_943_034E-10,
            1.551_169_281_097_026_6E-8,
            2.002_752_126_655_06E-7,
            1.914_052_885_645_138E-6,
            1.412_973_977_680_798E-5,
            8.315_281_580_948_582E-5,
            3.996_349_769_672_429E-4,
            0.001_598_442_290_393_378_4,
            0.005_401_484_462_492_892,
            0.015_609_515_951_961_325,
            0.038_960_859_894_776_14,
            0.084_675_992_815_357_84,
            0.161_320_272_041_780_37,
            0.270_895_707_022_142,
            0.402_766_052_144_190_03,
            0.532_134_840_644_357_2,
            0.626_561_850_396_477_3,
            0.658_939_504_140_677_5,
            0.619_968_794_555_102,
            0.522_392_634_872_676_4,
            0.394_418_806_923_720_8,
            0.266_845_588_852_137_27,
            0.161_693_943_297_351_4,
            0.087_665_230_931_323_02,
            0.042_462_146_242_945_82,
            0.018_336_610_588_859_478,
            0.007_040_822_524_198_700_5,
            0.002_395_953_515_750_436_4,
            7.196_709_691_248_771E-4,
            1.898_822_582_266_401E-4,
            4.375_352_582_937_183E-5,
            8.744_218_873_447_381E-6,
            1.503_255_708_913_270_4E-6,
            2.201_263_417_180_834_2E-7,
            2.713_269_374_479_116_4E-8,
            2.774_921_681_532_996E-9,
            2.313_546_085_591_984_2E-10,
            1.538_220_559_204_994_4E-11,
            7.931_012_545_002_62E-13,
            3.057_666_218_185_739E-14,
            8.393_076_986_026_449E-16,
            1.531_180_072_630_389E-17,
            1.675_381_720_821_777_5E-19,
            9.300_961_857_933_663E-22,
            1.912_538_194_408_499_4E-24,
            6.645_776_758_516_211E-28,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-10);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-10);
        }
    }
```
```rust
    #[test]
    fn check_derives() {
        let quad = GaussJacobi::init(10, 0.0, 1.0);
        let quad_clone = quad.clone();
        assert_eq!(quad, quad_clone);
        let other_quad = GaussJacobi::init(10, 1.0, 0.0);
        assert_ne!(quad, other_quad);
    }
```
```rust
    #[test]
    #[should_panic]
    fn panics_for_too_small_alpha() {
        GaussJacobi::init(3, -2.0, 1.0);
    }
```
```rust
    #[test]
    #[should_panic]
    fn panics_for_too_small_beta() {
        GaussJacobi::init(3, 1.0, -2.0);
    }
}
```
```rust
#[cfg(test)]
mod tests_gauss_laguerre {
    use gauss_quad::GaussLaguerre;

    #[test]
    fn golub_welsch_2_alpha_5() {
        let (x, w) = GaussLaguerre::nodes_and_weights(2, 5.0);
        let x_should = [4.354_248_688_935_409, 9.645_751_311_064_59];
        let w_should = [82.677_868_380_553_63, 37.322_131_619_446_37];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-12);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-12);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_3_alpha_0() {
        let (x, w) = GaussLaguerre::nodes_and_weights(3, 0.0);
        let x_should = [
            0.415_774_556_783_479_1,
            2.294_280_360_279_042,
            6.289_945_082_937_479_4,
        ];
        let w_should = [
            0.711_093_009_929_173,
            0.278_517_733_569_240_87,
            0.010_389_256_501_586_135,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-14);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-14);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_3_alpha_1_5() {
        let (x, w) = GaussLaguerre::nodes_and_weights(3, 1.5);
        let x_should = [
            1.220_402_317_558_883_8,
            3.808_880_721_467_068,
            8.470_716_960_974_048,
        ];
        let w_should = [
            0.730_637_894_350_016,
            0.566_249_100_686_605_7,
            0.032_453_393_142_515_25,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-14);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-14);
        }
    }
```
```rust
    #[test]
    fn golub_welsch_5_alpha_negative() {
        let (x, w) = GaussLaguerre::nodes_and_weights(5, -0.9);
        let x_should = [
            0.020_777_151_319_288_104,
            0.808_997_536_134_602_1,
            2.674_900_020_624_07,
            5.869_026_089_963_398,
            11.126_299_201_958_641,
        ];
        let w_should = [
            8.738_289_241_242_436,
            0.702_782_353_089_744_5,
            0.070_111_720_632_849_48,
            0.002_312_760_116_115_564,
            1.162_358_758_613_074_8E-5,
        ];
        for (i, x_val) in x_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*x_val, x[i], epsilon = 1e-14);
        }
        for (i, w_val) in w_should.iter().enumerate() {
            approx::assert_abs_diff_eq!(*w_val, w[i], epsilon = 1e-14);
        }
    }
```
```rust
    #[test]
    fn check_derives() {
        let quad = GaussLaguerre::init(10, 1.0);
        let quad_clone = quad.clone();
        assert_eq!(quad, quad_clone);
        let other_quad = GaussLaguerre::init(10, 2.0);
        assert_ne!(quad, other_quad);
    }
}
```
```rust
#[cfg(test)]
mod tests_midpoints {
    use gauss_quad::Midpoint;
```
```rust
    #[test]
    fn check_midpoint_integration() {
        let quad = Midpoint::init(100);
        let integral = quad.integrate(0.0, 1.0, |x| x * x);
        approx::assert_abs_diff_eq!(integral, 1.0 / 3.0, epsilon = 0.0001);
    }
```
```rust
    #[test]
    fn check_derives() {
        let quad = Midpoint::init(10);
        let quad_clone = quad.clone();
        assert_eq!(quad, quad_clone);
        let other_quad = Midpoint::init(3);
        assert_ne!(quad, other_quad);
    }
}
```
```rust
#[cfg(test)]
mod tests_simpson {
    use gauss_quad::Simpson;

    #[test]
    fn check_simpson_integration() {
        let quad = Simpson::init(2);
        let integral = quad.integrate(0.0, 1.0, |x| x * x);
        approx::assert_abs_diff_eq!(integral, 1.0 / 3.0, epsilon = 0.0001);
    }
```
```rust
    #[test]
    fn check_derives() {
        let quad = Simpson::init(10);
        let quad_clone = quad.clone();
        assert_eq!(quad, quad_clone);
        let other_quad = Simpson::init(3);
        assert_ne!(quad, other_quad);
    }
}
```
---
