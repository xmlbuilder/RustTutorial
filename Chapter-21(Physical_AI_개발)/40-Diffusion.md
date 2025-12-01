# GAN / Diffusion
GAN과 Diffusion 모델은 최근 AI에서 **데이터를 새로 만들어내는** 데 많이 쓰이는 **생성 모델(Generative Models)** 입니다.  
SAR 시뮬레이션이나 합성 데이터 얘기할 때도 자주 언급.

## 🧠 GAN (Generative Adversarial Network)
- 구조: 두 개의 신경망이 경쟁하는 방식
  - Generator: 가짜 데이터를 만들어냄 (예: 가짜 SAR 이미지)
  - Discriminator: 진짜와 가짜를 구분하려고 함
- 학습: Generator가 점점 더 **진짜 같은** 데이터를 만들도록 Discriminator와 경쟁하면서 발전
- 특징:
  - 고해상도 이미지 생성에 강점
  - 데이터 부족한 분야에서 합성 데이터 생성에 활용
  - SAR 응용: 실제 군용 SAR 데이터가 부족할 때, GAN으로 유사한 합성 SAR 이미지를 만들어 AI 학습에 사용

## 🧠 Diffusion Model
- 구조: 데이터를 점점 **노이즈로 변환** 했다가, 다시 **노이즈에서 원래 데이터로 복원** 하는 과정을 학습
- 학습:
  - Forward process: 이미지를 점점 노이즈로 바꿈
  - Reverse process: 노이즈에서 이미지를 점점 복원
- 특징:
  - 최근 이미지 생성 분야에서 GAN보다 더 안정적이고 고품질
  - 다양한 조건(텍스트, 라벨, 다른 이미지)에 맞춰 생성 가능
  - SAR 응용: 특정 지형, 특정 표적 조건을 주면 그에 맞는 SAR 이미지를 합성 가능

## 📌 GAN vs Diffusion 모델 차이점 요약

| 구분        | GAN (Generative Adversarial Network) | Diffusion Model                          | SAR 활용 포인트 |
|-------------|--------------------------------------|------------------------------------------|-----------------|
| 핵심 아이디어 | Generator와 Discriminator가 경쟁하며 학습 | 노이즈 → 데이터 복원 과정을 단계적으로 학습 | 합성 데이터 생성 |
| 학습 방식    | 경쟁적 학습 (Adversarial Training)   | 점진적 노이즈 제거 (Denoising Process)    | 다양한 조건 제어 가능 |
| 장점        | 빠른 학습, 고해상도 이미지 생성 가능 | 안정적 학습, 최근 최고 품질 이미지 생성   | 특정 환경/표적 SAR 합성 |
| 단점        | 학습 불안정, 모드 붕괴(mode collapse) | 계산량 많음, 학습 시간 길어짐             | 리소스 요구 큼 |
| 활용 예시    | 합성 SAR 데이터, 이미지 스타일 변환   | 조건부 SAR 합성, 시뮬레이션 데이터 다양화 | 데이터 부족 보완 |

---

# Python library 이용하기

GAN이나 Diffusion 모델을 **툴 없이 직접 만든다** 는 건, 라이브러리에서 제공하는 고수준 API를 쓰지 않고  
PyTorch나 TensorFlow 같은 프레임워크에서 기본 블록을 직접 구현한다는 의미.

## 🧠 GAN 기본 구조 (PyTorch 예시)
```python
import torch
import torch.nn as nn
import torch.optim as optim

# Generator 정의
class Generator(nn.Module):
    def __init__(self, latent_dim=100):
        super().__init__()
        self.model = nn.Sequential(
            nn.Linear(latent_dim, 256),
            nn.ReLU(),
            nn.Linear(256, 512),
            nn.ReLU(),
            nn.Linear(512, 784),   # 예: 28x28 이미지
            nn.Tanh()
        )
    def forward(self, z):
        return self.model(z)

# Discriminator 정의
class Discriminator(nn.Module):
    def __init__(self):
        super().__init__()
        self.model = nn.Sequential(
            nn.Linear(784, 512),
            nn.LeakyReLU(0.2),
            nn.Linear(512, 256),
            nn.LeakyReLU(0.2),
            nn.Linear(256, 1),
            nn.Sigmoid()
        )
    def forward(self, x):
        return self.model(x)

# 학습 루프
generator = Generator()
discriminator = Discriminator()
criterion = nn.BCELoss()
optimizer_G = optim.Adam(generator.parameters(), lr=0.0002)
optimizer_D = optim.Adam(discriminator.parameters(), lr=0.0002)

for epoch in range(100):
    # 1. 진짜 데이터
    real_data = torch.randn(64, 784)  # 예시: 실제 이미지 벡터
    real_labels = torch.ones(64, 1)

    # 2. 가짜 데이터
    z = torch.randn(64, 100)
    fake_data = generator(z)
    fake_labels = torch.zeros(64, 1)

    # Discriminator 학습
    optimizer_D.zero_grad()
    output_real = discriminator(real_data)
    output_fake = discriminator(fake_data.detach())
    loss_D = criterion(output_real, real_labels) + criterion(output_fake, fake_labels)
    loss_D.backward()
    optimizer_D.step()

    # Generator 학습
    optimizer_G.zero_grad()
    output_fake = discriminator(fake_data)
    loss_G = criterion(output_fake, real_labels)  # Generator는 Discriminator를 속여야 함
    loss_G.backward()
    optimizer_G.step()
```

## 🧠 Diffusion 모델 기본 구조 (PyTorch 예시)
```python
Diffusion은 노이즈 추가 → 노이즈 제거 학습 과정입니다. 아래는 매우 단순화된 버전입니다.
import torch
import torch.nn as nn
import torch.optim as optim

# 간단한 UNet-like 모델 (노이즈 제거기)
class DenoiseModel(nn.Module):
    def __init__(self):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(784, 512),
            nn.ReLU(),
            nn.Linear(512, 784)
        )
    def forward(self, x):
        return self.net(x)

model = DenoiseModel()
optimizer = optim.Adam(model.parameters(), lr=1e-3)
criterion = nn.MSELoss()

# 학습 루프
for epoch in range(100):
    # 원본 데이터
    x0 = torch.randn(64, 784)  # 예시: 실제 이미지
    
    # Forward process: 노이즈 추가
    noise = torch.randn_like(x0)
    xt = x0 + 0.1 * noise  # t-step 노이즈
    
    # Reverse process: 모델이 노이즈 제거
    x_hat = model(xt)
    
    # Loss: 원본과 복원된 데이터 차이
    loss = criterion(x_hat, x0)
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()
```

## 📌 정리
- GAN: Generator와 Discriminator 두 네트워크를 만들어 경쟁시키는 구조.
- Diffusion: 데이터를 점점 노이즈로 변환하고, 그 노이즈를 제거하는 과정을 학습하는 구조.
- 툴 없이 구현한다는 건, 이런 기본 블록(네트워크, 손실 함수, 학습 루프)을 직접 작성하는 걸 의미합니다.


## 🛰️ 1. SAR 합성 이미지 예시 (군용 SAR 시뮬레이션 느낌)
- 군용 SAR 이미지는 실제로는 보안 때문에 얻기 어렵지만, 시뮬레이션으로 비슷한 느낌을 만들 수 있습니다. 기본 아이디어는:
- 배경: 랜덤 지형(산악, 바다, 도시)을 흑백 텍스처로 표현
- 표적: 차량, 선박, 항공기 같은 작은 밝은 점(강한 반사체)
- 스펙클 잡음: SAR 특유의 점묘 잡음을 추가
  
### Python 코드 예시 (시뮬레이션 SAR 이미지)
```python
import numpy as np
import matplotlib.pyplot as plt

# SAR 이미지 크기
H, W = 256, 256

# 배경: 랜덤 지형 (노이즈 기반)
background = np.random.randn(H, W) * 0.3

# 표적: 밝은 점 몇 개 추가
targets = np.zeros((H, W))
for _ in range(10):
    x, y = np.random.randint(0, H), np.random.randint(0, W)
    targets[x, y] = np.random.uniform(3, 5)  # 강한 반사체

# 스펙클 잡음 추가
speckle = np.random.rayleigh(scale=0.5, size=(H, W))

# 최종 SAR 합성 이미지
sar_image = background + targets + speckle

plt.figure(figsize=(6,6))
plt.imshow(sar_image, cmap='gray')
plt.title("Synthetic SAR Simulation (Military-style)")
plt.axis('off')
plt.show()
```
- 👉 이 코드를 실행하면 랜덤 배경 + 강한 반사체 + 스펙클 잡음이 합쳐진 SAR 합성 이미지가 나옵니다.

## 🎨 2. GAN과 Diffusion 모델의 영상 변화 과정
### GAN (Generative Adversarial Network)
- Generator: 처음엔 랜덤 노이즈 → 점점 “진짜 같은” SAR 이미지로 변환
- Discriminator: 진짜/가짜를 구분 → Generator가 더 정교하게 학습
- 영상 변화 느낌:
  - 초기: 랜덤 점묘
  - 중간: 흐릿한 지형 패턴
  - 최종: 표적과 배경이 구분된 SAR 이미지
### Diffusion Model
- Forward process: 원본 이미지를 점점 노이즈로 변환
- Reverse process: 노이즈에서 점차 이미지를 복원
- 영상 변화 느낌:
  - 초기: 완전한 노이즈
  - 중간: 희미한 윤곽이 나타남
  - 최종: 선명한 SAR 이미지 복원

## 📌 AI 활용할 수 있는 포인트
- **SAR 합성 이미지는 랜덤 지형 + 표적 + 스펙클 잡음을 합성해서 시뮬레이션합니다.**
- **GAN은 노이즈에서 점점 진짜 같은 SAR 이미지를 만들어내고, Diffusion은 노이즈를 점차 제거하면서 이미지를 복원합니다.**
- **둘 다 실제 데이터가 부족할 때 합성 SAR 데이터를 만드는 데 유용합니다.**

---



