# Wavelet Filter

## 📘 Wavelet 기반 필터 소스 문서화
### 1. 핵심 수식 정리
#### 1.1 Circular Shift (CShift)
- 배열을 원형으로 이동시켜 convolution에 활용.

#### 1.2 Baseline Correction (ZeroMin)
- Row별 양 끝 평균을 이용해 baseline 제거:

```
baselineᵢ = ( avg(xᵢ,0…10%) + avg(xᵢ,90%…end) ) / 2
```

$$
x'_{i,j}=x_{i,j}-\mathrm{baseline_{\mathnormal{i}}}
$$

#### 1.3 Smoothing Filter
- 필터 계수:
- $h=[0,0,0.125,0.375,0.375,0.125]$
- Row별 convolution:

$$
y=\sum _jh[j]\cdot \mathrm{shift}(x,j)
$$

#### 1.4 Wavelet-like Transform
- Low-pass: h
- High-pass (derivative): $g=[0,0,0,-2,2,0]$
- Row/Col 방향 변환:

$$
s=\sum _jh[j]\cdot \mathrm{shift}(x,j),\quad w=\sum _jg[j]\cdot \mathrm{shift}(x,j)
$$

- 결과:
  - $imag\\_ temp$: 저역 필터링된 영상
  - $derive_r$: row 방향 derivative
  - $derive_c$: col 방향 derivative

#### 1.5 Width Sensor 추정
- 센서 index를 head/module/onebed 단위로 나누고 보정 계수 적용:

$$
\mathrm{Width}=a\cdot th1+b\cdot th2+c\cdot th3+d\cdot th4
$$

- a,b,c,d: 센서 배열에서 계산된 보정 값
- th1..th4: 온도별 보정 계수

### 2. 활용 가능한 분야
- 초음파/레이더 영상 처리
  - 스펙클 노이즈 제거, baseline correction, 두께 변화 검출
- 산업 검사
  - 가스관, 배관, 금속 구조물의 두께 이상 탐지
- 의료 영상
  - 초음파 영상에서 조직 두께/경계 검출
- 로보틱스
  - 이동 로봇이 센서로 수집한 데이터를 실시간 필터링하여 이상 탐지

### 3. AI 적용 가능성
#### ✅ 강점
- Feature Engineering:
  - Wavelet 필터가 노이즈 억제 + 에지 강화 → AI 모델 입력 품질 향상
- 다중 스케일 분석:
  - 작은 결함부터 큰 두께 변화까지 다양한 스케일에서 특징 추출 가능
- 실시간성:
  - Rust 구현으로 빠른 처리 → 로봇 이동 중 실시간 이상 탐지 가능

#### ⚠️ 고려사항
- 센서 특성(초음파 vs 레이다)에 따라 필터 파라미터 조정 필요
- 학습 데이터 확보가 중요 (정상/이상 사례 라벨링)

### 4. 가스관/레이더 데이터 학습 전략
- 데이터 축적
  - 로봇이 이동하며 초음파/레이더 스캔 → 2D intensity map 저장
  - 정상 구간 vs 이상 구간 라벨링
- 전처리
  - zero_min으로 baseline 제거
  - smooth_image로 노이즈 억제
  - wavelet_opn으로 row/col derivative 추출
  - Feature 추출
  - 두께 값 (width_sensor 결과)
  - min/max intensity 위치
  - derivative 값의 급격한 변화 영역
- AI 학습
  - 분류 모델: 정상 vs 이상 여부 판정 (CNN, Transformer 기반)
  - 회귀 모델: 두께 값 예측
  - Anomaly Detection: 정상 데이터로 학습 후 이상 패턴 자동 탐지
  - 실시간 적용
  - 로봇 onboard 컴퓨터에서 Rust 필터 실행
  - 추출된 feature를 Edge AI 모델에 입력 → 즉시 이상 여부 판단

## 🎯 결론
- 이 소스는 센서 데이터 전처리 및 Wavelet 기반 특징 추출 모듈로서 AI 파이프라인에 적합합니다.
- 가스관/레이더 데이터 축적 후, 이 필터로 두께 변화 feature를 추출하고 AI 모델에 학습시키면 자동 결함 탐지 시스템을 구축할 수 있습니다.
- 특히 로봇 실시간 검사 시나리오에 강력하게 활용 가능합니다.

---

## CSharp 소스
```csharp
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using Microsoft.VisualBasic;
using NumSharp;

namespace WaveletOpn
{
    public class WaveletHelper
    {
        public static void Swap<T>(ref T first, ref T second)
        {
            var temp = first;
            first = second;
            second = temp;
        }
        
        public static NDArray getNDArrayDoubleFromFile(string strFilePath, int nRow, int nCol)
        {
            var nd1 = np.zeros(new Shape(nRow, nCol)).MakeGeneric<double>();
            var lines = System.IO.File.ReadAllLines(strFilePath);
            Debug.Assert(lines.Length == nRow);
            var cntRow = 0;
            foreach (var line in lines)
            {
               var items = line.Split("\t");
               Debug.Assert(items.Length == nCol);
               for (var i = 0; i < items.Length; i++)
               {
                   nd1[cntRow, i] = double.Parse(items[i]);
               }
               cntRow++;
            }
            return nd1;
        }
        
        public static NDArray getNDArrayIntFromFile(string strFilePath, int nRow, int nCol)
        {
            var nd1 = np.arange(nRow*nCol).reshape(nRow, nCol).MakeGeneric<int>();
            var lines = System.IO.File.ReadAllLines(strFilePath);
            Debug.Assert(lines.Length == nRow);
            var cntRow = 0;
            foreach (var line in lines)
            {
                var items = line.Split("\t");
                Debug.Assert(items.Length == nCol);
                for (var i = 0; i < items.Length; i++)
                {
                    nd1[cntRow, i] = int.Parse(items[i]);
                }
                cntRow++;
            }
            return nd1;
        }

        public static NDArray CShift(NDArray inData, int level)
        {
            Debug.Assert(inData.ndim == 1);
            var inArray = (double[])inData.ToMuliDimArray<double>();
            var outArray = new double[inArray.Length];
            ArrayShift<double>(inArray, ref outArray, level);
            return np.array(outArray);
        }
        
        public static void ArrayShift<T>(T[] inData, ref T[] outData, int level)
        {
            var nSize = inData.Length;
            if (nSize == 0) return;
            while (level < 0)
            {
                level += nSize;
                if (level >= 0) break;
            }
            int nShift = level % nSize;
            Array.Copy(inData, nShift, outData, 0, nSize-nShift);
            Array.Copy(inData, 0, outData, nSize-nShift, nShift);
        }

        public static T[] ArrayShift<T>(T[] inData, int level)
        {
            var nSize = inData.Length;
            if (nSize == 0) return null;
            var outData = new T[nSize];
            while (level < 0)
            {
                level += nSize;
                if (level >= 0) break;
            }
            int nShift = level % nSize;
            
            Array.Copy(inData, nShift, outData, 0, nSize-nShift);
            Array.Copy(inData, 0, outData, nSize-nShift, nShift);
            return outData;
        }
        
        public static double Fix(double nIndex)
        {
            if (nIndex >= 0) return Math.Floor(nIndex);
            else return Math.Ceiling(nIndex);
        }
        
        public static int Fix(int nIndex)
        {
            if (nIndex >= 0) return (int)Math.Floor((double)nIndex);
            else return (int)Math.Ceiling((double)nIndex);
        }

        public static void Transpose(double[,] inData, ref double[,] outData)
        {
            var nRow = inData.GetLength(0);
            var nCol = inData.GetLength(1);
            for (var i = 0; i < nRow; i++)
            {
                for (var j = 0; j < nCol; j++)
                {
                    outData[j, i] = inData[i, j];
                }
            }
        }

        public static NDArray ZeroMin(NDArray inData, int zeroType)
        {
            Debug.Assert(inData.ndim == 2);
            var nRow = inData.shape[0];
            var nCol = inData.shape[1];
            var sum1 = new double[nRow];
            var sum2 = new double[nRow];
            if (zeroType == 2)
            {
                for (var i = 0; i < nRow; i++)
                {
                    sum1[i] = 0;
                    sum2[i] = 0;
                    var nColIndex1 = (int)Fix( nCol * 0.1);
                    for (var j = 0; j < nColIndex1; j++)
                    {
                        sum1[i] = sum1[i] + inData[i, j];
                    }
                    var nColIndex2 = (int)Fix( nCol * 0.9);
                    for (var j = nColIndex2-1; j < nCol; j++)
                    {
                        sum2[i] = sum2[i] + inData[i, j];
                    }
                    sum1[i] /= (double) (nColIndex1);
                    sum2[i] /= (double)(nCol - nColIndex2+1);
                }
            }
            
            double[] sumt = Enumerable.Repeat<double>(0, nRow).ToArray<double>();
            for (var i = 0; i < nRow; i++)
            {
                if (sum1[i] > sum2[i])
                {
                    if (sum1[i] > sum2[i]+10)
                    {
                        sumt[i] = sum2[i];
                    }
                    else{
                        sumt[i] = (sum1[i] + sum2[i]) / 2.0;
                    }
                }
                else
                {
                    if (sum2[i] > sum1[i]+10)
                    {
                        sumt[i] = sum1[i];
                    }else{
                        sumt[i] = (sum1[i] + sum2[i]) / 2.0;
                    }
                }
            }

            var outData = inData.Clone();
            var t_sum = Enumerable.Sum(sumt) / nRow;
            for (var i = 0; i < nRow; i++)
            {
                for (var j = 0; j < nCol; j++)
                {
                    outData[i, j] = inData[i, j] - sumt[i];
                }
            }
            return outData;
        }
        
        public static void ZeroMin(double[,] inData, ref double[,] outData, int zeroType)
        {
            var nRow = inData.GetLength(0);
            var nCol = inData.GetLength(1);
            var sum1 = new double[nRow];
            var sum2 = new double[nRow];
            if (zeroType == 2)
            {
                for (var i = 0; i < nRow; i++)
                {
                    sum1[i] = 0;
                    sum2[i] = 0;
                    var nColIndex1 = (int)Fix( nCol * 0.1);
                    for (var j = 0; j <= nColIndex1; j++)
                    {
                        sum1[i] = sum1[i] + inData[i, j];
                    }
                    var nColIndex2 = (int)Fix( nCol * 0.9);
                    for (var j = nColIndex2; j < nCol; j++)
                    {
                        sum2[i] = sum2[i] + inData[i, j];
                    }
                    sum1[i] /= (double) (nColIndex1 + 1);
                    sum2[i] /= (double)(nCol - nColIndex2);
                }
            }
            
            double[] sumt = Enumerable.Repeat<double>(0, nRow).ToArray<double>();
            
            for (var i = 0; i < nRow; i++)
            {
                if (sum1[i] > sum2[i])
                {
                    if (sum1[i] > sum2[i]+10)
                    {
                        sumt[i] = sum2[i];
                    }
                    else{
                        sumt[i] = (sum1[i] + sum2[i]) / 2.0;
                    }
                }
                else
                {
                    if (sum2[i] > sum1[i]+10)
                    {
                        sumt[i] = sum1[i];
                    }else{
                        sumt[i] = (sum1[i] + sum2[i]) / 2.0;
                    }
                }
            }

            var t_sum = Enumerable.Sum(sumt) / nRow;
            for (var i = 0; i < nRow; i++)
            {
                for (var j = 0; j < nCol; j++)
                {
                    outData[i, j] = inData[i, j] - sumt[i];
                }
            }
        }
        
        public static NDArray SmoothImage(NDArray inData, int level)
        {
            Debug.Assert(inData.ndim == 2);
            var h = np.array(new[] {0, 0, 0.125, 0.375, 0.375, 0.125});
            var sizeh = h.size;
            var inArray = inData.Clone();
            var tempData = inData.Clone();
            var nRow = tempData.shape[0];
            var nCol = tempData.shape[1];
            var sizex= nCol;
            
            for (var i = 0; i < level; i++)
            {
                var L = 1;
                for (var r = 0; r < nRow; r++)
                {
                    var Sx = np.zeros(sizex);
                    var X = tempData[r, Slice.All];
                    for (var j = 0; j < sizeh; j++)
                    {
                        var arrayShift = np.array(ArrayShift<double>(X.ToArray<double>(), -L * (sizeh / 2 - j)));
                        Sx = Sx + h[j] * arrayShift ;
                    }
                    tempData[r] = Sx;
                }     
            }
            NDArray outData = inData.Clone();
            for (var i = 0; i < nRow; i++)
            {
                for (var j = 0; j < nCol; j++)
                {
                    outData[i, j] = tempData[i, j];
                }
            }
            return outData;
        }

        public static void SmoothImage(double[,] inData, ref double[,] outData, int level)
        {
            var h = np.array(new[] {0, 0, 0.125, 0.375, 0.375, 0.125});
            var sizeh = h.size;
            var inArray = np.array(inData);
            var tempData = np.array(inData);
            var nRow = tempData.shape[0];
            var nCol = tempData.shape[1];
            var sizex= nCol;
            for (var i = 0; i < level; i++)
            {
                var L = 1;
                for (var r = 0; r < nRow; r++)
                {
                    var Sx = np.zeros(sizex);
                    var X = tempData[r, Slice.All];
                    for (var j = 0; j < sizeh; j++)
                    {
                        var arrayShift = np.array(ArrayShift<double>(X.ToArray<double>(), -L * (sizeh / 2 - j)));
                        Sx = Sx + h[j] * arrayShift ;
                    }
                    tempData[r] = Sx;
                }     
            }

            for (var i = 0; i < nRow; i++)
            {
                for (var j = 0; j < nCol; j++)
                {
                    outData[i, j] = tempData[i, j];
                }
            }
        }

        public static (NDArray, NDArray, NDArray) WaveletOpn(NDArray image, int level)
        {
            var h = np.array(new double[] {0, 0,  0.125, 0.375, 0.375, 0.125});
            var g = np.array(new double[] {0, 0,  0, -2, 2, 0});

            Debug.Assert(image.ndim == 2);
            var M = image.shape[0];
            var N = image.shape[1];
            
            var h_t = CShift(h,1);
            
            var sizex = N;
            var sizey = M;
            var sizeh = h.size;
            var n = np.arange(sizeh);
            var imagTemp = image.Clone();
            var Derive_r = image.Clone();
            var Derive_c = image.Clone();

            for (var k = 0; k < level; k++)
            {
                var L = Math.Pow(2, k);
                for (var r = 0; r < M; r++)
                {
                    var Sx = np.zeros(sizex);
                    var Wx = np.zeros(sizex);
                    var X = imagTemp[r, Slice.All].Clone();
                    for (var j = 0; j < sizeh; j++)
                    {
                        Sx = Sx + h[j] * CShift(X,(int)(-L * (sizeh/2.0-(double)j)));
                        Wx = Wx + g[j] * CShift(X,(int)(-L * (sizeh/2.0-(double)j)));
                    } 
                    imagTemp[r,Slice.All] = Sx;
                    Derive_r[r, Slice.All] = Wx; 
                    Derive_c[r, Slice.All] = X;
                }
                for (var c = 0; c < N; c++)
                {
                    var Sx = np.zeros(sizey);
                    var Wx = np.zeros(sizey);
                    var X = imagTemp[Slice.All, c].Clone();
                    for (int j = 0; j < sizeh; j++)
                    {
                        var Derive_c_tras = Derive_c[Slice.All, c].transpose().Clone();
                        Sx = Sx + h[j] * CShift(X,(int)(-L * (sizeh / 2 - j)));
                        Wx = Wx + g[j] * CShift(Derive_c_tras, (int)(-L * (sizeh/2 - j)));
                    }
                    Sx = Sx.transpose();
                    Wx = Wx.transpose();
                    imagTemp[Slice.All, c] = Sx;
                    Derive_c[Slice.All,c] = Wx;
                }
            }
            return (imagTemp, Derive_r, Derive_c);
        }

        public static double WidthSensor(int s_sen, int e_sen, double t)
        {
            var th1 = 9.592;
            var th2 = 15.725;
            var th3 = 17.387;
            var th4 = 34.012;
            if (t == 11.1)
            {
                th1 = 9.425;    
                th2 = 16.06;
                th3 = 17.722;
                th4 = 34.346;             
            }
            else if (t == 14.3)
            {
                th1 = 9.508;    
                th2 = 15.894;
                th3 = 17.556;
                th4 = 34.18;
            }
            else if (t == 17.5)
            {
                th1 = 9.592;    
                th2 = 15.725;
                th3 = 17.387;
                th4 = 34.012;
            }

            var a = 0;
            var b = 0;
            var c = 0;
            var d = 0;
            
            if (e_sen - s_sen > 0)
            {
                var s_head = 0;
                if (s_sen % 3 == 0)
                {
                    s_head = Fix(s_sen / 3);
                    a = -2;
                }
                else if (s_sen % 3 == 1)
                {
                    s_head = Fix(s_sen / 3) + 1;
                    a = 0;
                }
                else if (s_sen % 3 == 2)
                {
                    s_head = Fix(s_sen / 3) + 1;
                    a = -1;
                }

                var e_head = 0;
                if (e_sen % 3 == 0)
                {
                    e_head = Fix(e_sen / 3);
                }
                else if (e_sen % 3 == 1)
                {
                    e_head = Fix(e_sen / 3) + 1;
                    a -= 2;
                }
                else if (e_sen % 3 == 2)
                {
                    e_head = Fix(e_sen / 3) + 1;
                    a -= 1;
                }

                var s_module = 0;
                if (s_head % 4 == 0)
                {
                    s_module = Fix(s_head/4);
                    b = -3;
                }
                else if (s_head % 4 == 1)
                {
                    s_module = Fix(s_head/4)+1;
                    b = 0;
                }
                else if (s_head % 4 == 2)
                {
                    s_module = Fix(s_head/4)+1;
                    b= -1;
                }
                else if (s_head % 4 == 3)
                {
                    s_module = Fix(s_head/4)+1;
                    b = -2;
                }
                
                var e_module = 0;
                if (e_head % 4 == 0)
                {
                    e_module = Fix(e_head / 4);
                }
                else if (e_head % 4 == 1)
                {
                    e_module = Fix(e_head/4)+1;
                    b -= 3;
                }
                else if (e_head % 4 == 2)
                {
                    e_module = Fix(e_head/4)+1;
                    b -= 2;
                }
                else if (e_head % 4 == 3)
                {
                    e_module = Fix(e_head/4)+1;
                    b -= 1;
                }

                var s_onebed = 0;
                if (s_module % 2 == 0)
                {
                    s_onebed = Fix(s_module/2);
                    c = -1;
                }
                else if (s_module % 2 == 1)
                {
                    s_onebed = Fix(s_module/2)+1;
                    c = 0;
                }

                var e_onebed = 0;
                if (e_module % 2 == 0)
                {
                    e_onebed = Fix(e_module/2);
                }
                else if (e_module % 2 == 1)
                {
                    e_onebed = Fix(e_module/2)+1;
                    c -= 1;
                }

                a = (e_head - s_head + 1) * 2 + a;
                b = (e_module - s_module + 1) * 3 + b;
                c = (e_onebed - s_onebed + 1) * 1 + c;
                d = (e_onebed - s_onebed);
            }
            else
            {
                
                e_sen += 192;
                var s_head = 0;
                if (s_sen % 3 == 0)
                {
                    s_head = Fix(s_sen/3);
                    a = -2;      
                }
                else if (s_sen % 3 == 1)
                {
                    s_head = Fix(s_sen/3) + 1;
                    a = 0;
                }
                else if (s_sen % 3 == 2)
                {
                    s_head = Fix(s_sen/3) + 1;
                    a = -1;
                }

                var e_head = 0;
                if (e_sen % 3 == 0)
                {
                    e_head = Fix(e_sen/3);
                }
                else if (e_sen % 3 == 1)
                {
                    e_head = Fix(e_sen/3) + 1;
                    a -= 2;
                    
                }
                else if (e_sen % 3 == 2)
                {
                    e_head = Fix(e_sen/3) + 1;
                    a -= 1;
                }

                var s_module = 0;
                if (s_head % 4 == 0)
                {
                    s_module = Fix(s_head/4);
                    b = -3;
                }
                else if (s_head % 4 == 1)
                {
                    s_module = Fix(s_head/4)+1;
                    b = 0;
                }
                else if (s_head % 4 == 2)
                {
                    s_module = Fix(s_head/4)+1;
                    b= -1;
                }
                else if (s_head % 4 == 3)
                {
                    s_module = Fix(s_head/4)+1;
                    b = -2;
                }

                var e_module = 0;
                if (e_head % 4 == 0)
                {
                    e_module = Fix(e_head/4);
                }
                else if (e_head % 4 == 1)
                {
                    e_module = Fix(e_head/4)+1;
                    b -= 3;
                }
                else if (e_head % 4 == 2)
                {
                    e_module = Fix(e_head/4)+1;
                    b -= 2;
                }
                else if (e_head % 4 == 3)
                {
                    e_module = Fix(e_head/4)+1;
                    b -= 1;
                }

                var s_onebed = 0;
                if (s_module % 2 == 0)
                {
                    s_onebed = Fix(s_module/2);
                    c = -1;
                }
                else if (s_module % 2 == 1)
                {
                    s_onebed = Fix(s_module/2)+1;
                    c = 0;
                }

                var e_onebed = 0;
                if (e_module % 2 == 0)
                {
                    e_onebed = Fix(e_module/2);
                }
                else if (e_module % 2 == 1)
                {
                    e_onebed = Fix(e_module/2)+1;
                    c -= 1;
                }
                
                a = (e_head - s_head + 1) * 2 + a;
                b = (e_module - s_module +1) * 3 + b;
                c = (e_onebed - s_onebed +1) * 1 + c;
                d = (e_onebed - s_onebed);
            }
            return (a * th1 + b * th2 + c * th3 + d * th4);
        }
    }
}
```
```csharp
using System;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Security.Cryptography;
using NumSharp;
using NumSharp.Generic;
using NumSharp.Utilities;
using Buffer = System.Buffer;
namespace WaveletOpn
{
    class Program
    {
        static void Main(string[] args)
        {
            var model_data1 = new double[,]
            {
                {1.0000, 2.0000, 2.0000, 10.0000, 35.0000, 35.0000, 30.0000, 30.0000},
                {2.0000, 2.0000, 4.0000, 10.0000, 47.0700, 88.1050, 66.3594, 66.3600},
                {3.0000, 2.0000, 6.0000, 10.0000, 41.8400, 113.4220, 65.8281, 65.8300},
                {4.0000, 4.0000, 2.0000, 10.0000, 73.2200, 60.2260, 22.7500, 22.0300},
                {5.0000, 4.0000, 4.0000, 10.0000, 73.2200, 69.8180, 53.8750, 53.8800},
                {6.0000, 4.0000, 6.0000, 10.0000, 73.2200, 60.2260, 41.6094, 41.6100},
                {7.0000, 6.0000, 2.0000, 10.0000, 104.6000, 53.1960, 19.8125, 20.0000},
                {8.0000, 6.0000, 4.0000, 10.0000, 109.8300, 44.5010, 25.3594, 25.3600},
                {9.0000, 6.0000, 6.0000, 10.0000, 104.6000, 97.6970, 29.5156, 29.5200},
                {10.0000, 2.0000, 2.0000, 20.0000, 41.8400, 44.5010, 39.9531, 39.9500},
                {11.0000, 2.0000, 4.0000, 20.0000, 47.0700, 88.1050, 80.8750, 80.8800},
                {12.0000, 2.0000, 6.0000, 20.0000, 41.8400, 113.4220, 121.6406, 121.6400},
                {13.0000, 4.0000, 2.0000, 20.0000, 73.2200, 44.5010, 36.0469, 36.0500},
                {14.0000, 4.0000, 4.0000, 20.0000, 73.2200, 69.8180, 66.1094, 66.1100},
                {15.0000, 4.0000, 6.0000, 20.0000, 73.2200, 95.1350, 76.9531, 76.9500},
                {16.0000, 6.0000, 2.0000, 20.0000, 104.6000, 44.5010, 32.9688, 32.9700},
                {17.0000, 6.0000, 4.0000, 20.0000, 104.6000, 88.1050, 44.1406, 44.1400},
                {18.0000, 6.0000, 6.0000, 20.0000, 104.6000, 113.4220, 64.9063, 64.9100},
                {19.0000, 2.0000, 2.0000, 30.0000, 36.6100, 44.5010, 60.8438, 60.8400},
                {20.0000, 2.0000, 4.0000, 30.0000, 41.8400, 88.1050, 128.3750, 128.3800},
                {21.0000, 2.0000, 6.0000, 30.0000, 41.8400, 113.4220, 175.2344, 175.2300},
                {22.0000, 4.0000, 2.0000, 30.0000, 67.9900, 44.5010, 43.2813, 43.2800},
                {23.0000, 4.0000, 4.0000, 30.0000, 67.9900, 69.8180, 121.1250, 121.1300},
                {24.0000, 4.0000, 6.0000, 30.0000, 67.9900, 85.5430, 135.0000, 135.0000},
                {25.0000, 6.0000, 2.0000, 30.0000, 99.3700, 68.9210, 43.4219, 43.4200},
                {26.0000, 6.0000, 4.0000, 30.0000, 99.3700, 62.7880, 74.4219, 74.4200},
                {27.0000, 6.0000, 6.0000, 30.0000, 99.3700, 88.1050, 106.0625, 106.0600},
                {28.0000, 2.0000, 2.0000, 50.0000, 36.6100, 34.9090, 100.6562, 100.6600},
                {29.0000, 2.0000, 4.0000, 50.0000, 36.6100, 78.5130, 229.2188, 229.2200},
                {30.0000, 2.0000, 6.0000, 50.0000, 36.6100, 107.2890, 325.5313, 325.5300},
                {31.0000, 4.0000, 2.0000, 50.0000, 62.7600, 44.5010, 83.0156, 83.0200},
                {32.0000, 4.0000, 4.0000, 50.0000, 62.7600, 69.8180, 149.2813, 149.2800},
                {33.0000, 4.0000, 6.0000, 50.0000, 67.9900, 95.1350, 232.5781, 232.5800},
                {34.0000, 6.0000, 2.0000, 50.0000, 94.1400, 68.9210, 61.9063, 61.9100},
                {35.0000, 6.0000, 4.0000, 50.0000, 99.3700, 78.5130, 115.5937, 155.5900},
                {36.0000, 6.0000, 6.0000, 50.0000, 94.1400, 97.6970, 164.3750, 164.3800},
                {37.0000, 2.0000, 2.0000, 70.0000, 31.3800, 34.9090, 130.2188, 130.2200},
                {38.0000, 2.0000, 4.0000, 70.0000, 31.3800, 88.1050, 322.7500, 322.7500},
                {39.0000, 2.0000, 6.0000, 70.0000, 31.3800, 78.5130, 389.3125, 500.0000},
                {40.0000, 4.0000, 2.0000, 70.0000, 57.5300, 34.9090, 101.0938, 101.0900},
                {41.0000, 4.0000, 4.0000, 70.0000, 57.5300, 60.2260, 200.0469, 200.0500},
                {42.0000, 4.0000, 6.0000, 70.0000, 57.5300, 95.1350, 350.1250, 350.1250},
                {43.0000, 6.0000, 2.0000, 70.0000, 83.6800, 53.1960, 80.7813, 80.7800},
                {44.0000, 6.0000, 4.0000, 70.0000, 88.9100, 62.7880, 155.0000, 158.9100},
                {45.0000, 6.0000, 6.0000, 70.0000, 88.9100, 97.6970, 219.4531, 219.4500},
                {46.0000, 2.0000, 2.0000, 80.0000, 31.3800, 34.9090, 158.0000, 158.0000},
                {47.0000, 2.0000, 4.0000, 80.0000, 31.3800, 78.5130, 324.0469, 400.0000},
                {48.0000, 2.0000, 6.0000, 80.0000, 31.3800, 88.1050, 379.8594, 650.0000},
                {49.0000, 4.0000, 2.0000, 80.0000, 52.3000, 34.9090, 108.9062, 108.9100},
                {50.0000, 4.0000, 4.0000, 80.0000, 52.3000, 60.2260, 245.9063, 245.9100},
                {51.0000, 4.0000, 6.0000, 80.0000, 52.3000, 95.1350, 415.2187, 415.2200},
                {52.0000, 6.0000, 2.0000, 80.0000, 78.4500, 53.1960, 85.6094, 85.6100},
                {53.0000, 6.0000, 4.0000, 80.0000, 83.6800, 62.7880, 181.5000, 181.5000},
                {54.0000, 6.0000, 6.0000, 80.0000, 88.9100, 97.6970, 256.6563, 256.6600},
                {55.0000, 2.0000, 3.0000, 30.0000, 36.6100, 54.0930, 89.6094, 89.6100},
                {56.0000, 3.0000, 2.0000, 30.0000, 57.5300, 69.8180, 46.0781, 46.0800},
                {57.0000, 3.0000, 3.0000, 30.0000, 62.7600, 62.7880, 85.0000, 85.0000},
                {58.0000, 3.0000, 4.0000, 30.0000, 57.5300, 69.8180, 109.0313, 109.0300},
                {59.0000, 3.0000, 6.0000, 30.0000, 57.5300, 79.4100, 133.0313, 133.0300},
                {60.0000, 4.0000, 3.0000, 30.0000, 67.9900, 60.2260, 75.6563, 75.6600},
                {61.0000, 6.0000, 3.0000, 30.0000, 99.3700, 62.7880, 58.2344, 58.2300},
                {62.0000, 2.0000, 3.0000, 50.0000, 31.3800, 44.5010, 153.9688, 153.9700},
                {63.0000, 3.0000, 2.0000, 50.0000, 47.0700, 62.7880, 80.4844, 80.4800},
                {64.0000, 3.0000, 3.0000, 50.0000, 47.0700, 53.1960, 137.9688, 137.9700},
                {65.0000, 3.0000, 4.0000, 50.0000, 52.3000, 69.8180, 197.7031, 197.7000},
                {66.0000, 3.0000, 6.0000, 50.0000, 52.3000, 123.0140, 262.8281, 262.8300},
                {67.0000, 4.0000, 3.0000, 50.0000, 62.7600, 62.7880, 128.1875, 128.1900},
                {68.0000, 6.0000, 3.0000, 50.0000, 99.3700, 50.6340, 94.9688, 94.9700},
                {69.0000, 2.0000, 3.0000, 80.0000, 31.3800, 44.5010, 246.2188, 246.2200},
                {70.0000, 3.0000, 2.0000, 80.0000, 41.8400, 44.5010, 129.8281, 129.8300},
                {71.0000, 3.0000, 3.0000, 80.0000, 41.8400, 62.7880, 237.7969, 237.8000},
                {72.0000, 3.0000, 4.0000, 80.0000, 36.6100, 69.8180, 374.3750, 374.3800},
                {73.0000, 3.0000, 6.0000, 80.0000, 36.6100, 107.2890, 414.9375, 500.0000},
                {74.0000, 4.0000, 3.0000, 80.0000, 57.5300, 44.5010, 201.1094, 201.1100},
                {75.0000, 6.0000, 3.0000, 80.0000, 83.6800, 44.5010, 132.1719, 132.1700},
            };

            var int_axial = WaveletHelper.getNDArrayDoubleFromFile(@"D:\Development\csharp\VPK\WaveletOpn\data\Axial1.txt", 40, 100);
            var int_radial = WaveletHelper.getNDArrayDoubleFromFile(@"D:\Development\csharp\VPK\WaveletOpn\data\Radial1.txt", 40, 100);
            var int_circum = WaveletHelper.getNDArrayDoubleFromFile(@"D:\Development\csharp\VPK\WaveletOpn\data\Circum1.txt", 40, 100);
            var HSensor_count = WaveletHelper.getNDArrayIntFromFile(@"D:\Development\csharp\VPK\WaveletOpn\data\SensorNo1.txt", 1, 40);

            var zero_type = 2;
            var using_circum = 1;

            var est_width = np.zeros(4).reshape(new Shape(1, 4));

            int_axial = WaveletHelper.ZeroMin(int_axial,zero_type);
            int_radial = WaveletHelper.ZeroMin(int_radial,zero_type);
            int_circum = WaveletHelper.ZeroMin(int_circum,zero_type);
          
            
            var level = 1;
            int_radial = WaveletHelper.SmoothImage(int_radial, level);
            int_axial = WaveletHelper.SmoothImage(int_axial, level);
            int_circum = WaveletHelper.SmoothImage(int_circum, level);

            int_axial = int_axial.transpose();
            int_radial = int_radial.transpose();
            int_circum = int_circum.transpose();
            

            int_axial = WaveletHelper.SmoothImage(int_axial, level);
            int_radial = WaveletHelper.SmoothImage(int_radial, level);
            int_circum = WaveletHelper.SmoothImage(int_circum, level);
            
            int_axial = int_axial.transpose();
            int_radial = int_radial.transpose();
            int_circum = int_circum.transpose();

            int_circum = int_circum * (-1);
            
            int hall_count = 40;
            var nStartHallCount = hall_count / 4 + 1;
            var nEndHallCount = hall_count / 4 * 3;

            var dAxialMin = 1000.0;
            var sensor_point = -1;
            var distance_point = -1;
            for (var i = nStartHallCount; i <= nEndHallCount; i++)
            {
                for (var j = 30; j <= 70; j++)
                {
                    if (dAxialMin > int_axial[i,j])
                    {
                        dAxialMin = int_axial[i, j];
                        sensor_point = i;
                        distance_point = j;
                    }
                }
            }

            //Circumferential 신호를 이용하여  Width를 구함.
            {
                
                var max_wave1 = -1000.0;
                var sensor_max1 = -1;
                for (var i = 5; i < sensor_point; i++)
                {
                    for (var j = 5; j < distance_point; j++)
                    {
                        if (max_wave1 < int_circum[i, j])
                        {
                            max_wave1 = int_circum[i,j];
                            sensor_max1 = i;
                        }
                    }
                }

                var min_wave1 = 1000.0;
                var sensor_min1 = -1;
                for (var i = sensor_point-1; i < hall_count - 5; i++)
                {
                    for (var j = 5; j < distance_point; j++)
                    {
                        if (min_wave1 > int_circum[i, j])
                        {
                            min_wave1 = int_circum[i,j];
                            sensor_min1 = i;
                        }
                    }
                }

                var nCol = int_circum.shape[1];
                
                var min_wave2 = 1000.0;
                var sensor_min2 = -1;
                for (var i = 5; i < sensor_point; i++)
                {
                    for (var j = distance_point-1; j < nCol - 5; j++)
                    {
                        if (min_wave2 > int_circum[i, j])
                        {
                            min_wave2 = int_circum[i,j];
                            sensor_min2 = i;
                        }
                    }
                }
                
                var max_wave2 = -1000.0;
                var sensor_max2 = -1;
                for (var i = sensor_point-1; i < hall_count - 5; i++)
                {
                    for (var j = distance_point-1; j < nCol - 5; j++)
                    {
                        if (max_wave2 < int_circum[i, j])
                        {
                            max_wave2 = int_circum[i,j];
                            sensor_max2 = i;  
                        }
                    }   
                }
                
                if (Math.Abs(sensor_max1 - sensor_min1) < 15)
                {
                    if (sensor_max1 > sensor_min1)
                    {
                        var temp = sensor_min1;
                        sensor_min1 = sensor_max1;
                        sensor_max1 = temp;
                    }
                }
                
                if (Math.Abs(sensor_max2 - sensor_min2) < 15)
                {
                    if (sensor_max2 > sensor_min2)
                    {
                        var temp = sensor_min2;
                        sensor_min2 = sensor_max2;
                        sensor_max2 = temp;                   
                    }
                }

                int nCntSensorMin1 = (int)HSensor_count[0, sensor_min1];
                int nCntSensorMax1 = (int)HSensor_count[0, sensor_max1];
                int nCntSensorMin2 = (int)HSensor_count[0, sensor_min2];
                int nCntSensorMax2 = (int)HSensor_count[0, sensor_max2];

                var C1 = WaveletHelper.WidthSensor(nCntSensorMax1, nCntSensorMin1, 17.5);
                var C2 = WaveletHelper.WidthSensor(nCntSensorMax2, nCntSensorMin2, 17.5);
                if (using_circum == 1)
                {
                    est_width[0, 0] = C1;
                    est_width[0, 1] = C2;
                    
                }
                else
                {
                    est_width[0, 0] = 10000.0;
                    est_width[0, 1] = 10000.0;
                }
            }
            
            NDArray radial_pull, radial_c, radial_r;
            (radial_pull, radial_r, radial_c)  = WaveletHelper.WaveletOpn(int_radial, 1);
            
            //Radial의 Wavelet Transformation 신호를 이용하여 폭 추정
            {
                var max_wave1 = -1000.0;
                var sensor_max1 = -1;
                for (var i = 5; i < sensor_point; i++)
                {
                    for (var j = 5; j < distance_point; j++)
                    {
                        if (max_wave1 < radial_c[i, j])
                        {
                            max_wave1 = radial_c[i,j];
                            sensor_max1 = i; 
                        }
                    }   
                }
                
                var min_wave1 = 1000.0;
                int sensor_min1 = -1;
                for (int i = sensor_point-1; i < hall_count - 5; i++)
                {
                    for (var j = 5; j < distance_point; j++)
                    {
                        if (min_wave1 > radial_c[i, j])
                        {
                            min_wave1 = radial_c[i,j];
                            sensor_min1 = i;
                        }
                    }
                }
                
                var nCol = int_circum.shape[1];
                var min_wave2 = 1000.0;
                var sensor_min2 = -1;
                for (var i = 5; i < sensor_point; i++)
                {
                    for (var j = distance_point - 1; j < nCol - 5; j++)
                    {
                        if (min_wave2 > radial_c[i, j])
                        {
                            min_wave2 = radial_c[i,j];
                            sensor_min2 = i;
                        }
                    }
                }
                
                var max_wave2 = -1000.0;
                var sensor_max2 = -1;
                for( var i=sensor_point-1; i< hall_count-5; i++)
                {
                    for (var j = distance_point - 1; j < nCol - 5; j++)
                    {
                        if (max_wave2 < radial_c[i, j])
                        {
                            max_wave2 = radial_c[i, j];
                            sensor_max2 = i;
                        }
                    }
                }

                if (Math.Abs(sensor_max1 - sensor_min1) < 15)
                {
                    if (sensor_max1 > sensor_min1)
                    {
                        WaveletHelper.Swap<int>(ref sensor_min1, ref sensor_max1);
                    }
                }
                
                if (Math.Abs(sensor_max2 - sensor_min2) < 15)
                {
                    if (sensor_max2 > sensor_min2)
                    {
                        WaveletHelper.Swap<int>(ref sensor_min2, ref sensor_max2);
                    }
                }
            

                int nCntSensorMin1 = (int)HSensor_count[0, sensor_min1];
                int nCntSensorMax1 = (int)HSensor_count[0, sensor_max1];
                int nCntSensorMin2 = (int)HSensor_count[0, sensor_min2];
                int nCntSensorMax2 = (int)HSensor_count[0, sensor_max2];

                var R_W_1 = WaveletHelper.WidthSensor(nCntSensorMax1, nCntSensorMin1, 17.5);
                var R_W_2 = WaveletHelper.WidthSensor(nCntSensorMax2, nCntSensorMin2, 17.5);
                
                est_width[0, 2] = R_W_1;
                est_width[0, 3] = R_W_2;

            }  
            
            var min_width = 10000.0;
            for (var i = 0; i < 4; i++)
            {
                if (min_width > est_width[0, i])
                {
                    min_width = est_width[0, i];
                    
                }
            }
            var est_w = min_width;
            Console.WriteLine(int_circum.ToString());
            Console.WriteLine(radial_c.ToString());

            
        }
    }
}
```
---

## Rust 소스
```rust
use ndarray::{Array1, Array2, s};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::f64;
use std::path::{Path, PathBuf};

pub struct WaveletHelper;

impl WaveletHelper {
    pub fn swap<T>(a: &mut T, b: &mut T) {
        std::mem::swap(a, b);
    }

    pub fn get_ndarray_double_from_file(
        path: &str,
        nrow: usize,
        ncol: usize,
    ) -> Array2<f64> {
        let file = File::open(path).expect("failed to open file");
        let reader = BufReader::new(file);

        let mut data = Array2::<f64>::zeros((nrow, ncol));
        for (row_idx, line) in reader.lines().enumerate() {
            assert!(row_idx < nrow);
            let line = line.expect("read line failed");
            let items: Vec<&str> = line.split('\t').collect();
            assert_eq!(items.len(), ncol);
            for (col_idx, item) in items.iter().enumerate() {
                let v: f64 = item.trim().parse().expect("parse f64 failed");
                data[(row_idx, col_idx)] = v;
            }
        }
        data
    }
```
```rust
    pub fn get_ndarray_int_from_file(
        path: &str,
        nrow: usize,
        ncol: usize,
    ) -> Array2<i32> {
        let file = File::open(path).expect("failed to open file");
        let reader = BufReader::new(file);

        let mut data = Array2::<i32>::zeros((nrow, ncol));
        for (row_idx, line) in reader.lines().enumerate() {
            assert!(row_idx < nrow);
            let line = line.expect("read line failed");
            let items: Vec<&str> = line.split('\t').collect();
            assert_eq!(items.len(), ncol);
            for (col_idx, item) in items.iter().enumerate() {
                let v: i32 = item.trim().parse().expect("parse i32 failed");
                data[(row_idx, col_idx)] = v;
            }
        }
        data
    }
```
```rust
    // CShift for 1D: circular shift (left for positive level)
    pub fn cshift_1d(in_data: &Array1<f64>, level: isize) -> Array1<f64> {
        let n = in_data.len();
        if n == 0 {
            return Array1::zeros(0);
        }

        let mut lvl = level;
        while lvl < 0 {
            lvl += n as isize;
            if lvl >= 0 {
                break;
            }
        }
        let shift = (lvl as usize) % n;

        let mut out = Array1::<f64>::zeros(n);
        // ArrayShift: copy [shift..] -> [0..n-shift], then [0..shift] -> [n-shift..]
        out.slice_mut(s![0..(n - shift)])
            .assign(&in_data.slice(s![shift..]));
        out.slice_mut(s![(n - shift)..n])
            .assign(&in_data.slice(s![0..shift]));
        out
    }
```
```rust
    // generic ArrayShift on Vec<T>
    pub fn array_shift_vec<T: Clone>(in_data: &[T], level: isize) -> Vec<T> {
        let n = in_data.len();
        if n == 0 {
            return Vec::new();
        }

        let mut lvl = level;
        while lvl < 0 {
            lvl += n as isize;
            if lvl >= 0 {
                break;
            }
        }
        let shift = (lvl as usize) % n;

        let mut out = Vec::<T>::with_capacity(n);
        out.extend_from_slice(&in_data[shift..]);
        out.extend_from_slice(&in_data[..shift]);
        out
    }
```
```rust
    pub fn fix_f64(x: f64) -> f64 {
        if x >= 0.0 {
            x.floor()
        } else {
            x.ceil()
        }
    }
```
```rust
    pub fn fix_i32(x: i32) -> i32 {
        if x >= 0 {
            (x as f64).floor() as i32
        } else {
            (x as f64).ceil() as i32
        }
    }
```
```rust
    pub fn transpose_2d(in_data: &Array2<f64>) -> Array2<f64> {
        in_data.t().to_owned()
    }
```
```rust
    /// ZeroMin for 2D: subtract per-row baseline estimated from both ends (10% + 90% 구간)
    pub fn zero_min(in_data: &Array2<f64>, zero_type: i32) -> Array2<f64> {
        assert_eq!(in_data.ndim(), 2);
        let (nrow, ncol) = in_data.dim();
        let mut sum1 = vec![0.0; nrow];
        let mut sum2 = vec![0.0; nrow];

        if zero_type == 2 {
            for i in 0..nrow {
                sum1[i] = 0.0;
                sum2[i] = 0.0;

                let col_index1 =
                    Self::fix_i32(((ncol as f64) * 0.1) as i32) as usize;
                let col_index2 =
                    Self::fix_i32(((ncol as f64) * 0.9) as i32) as usize;

                // 처음 10% 평균
                for j in 0..col_index1 {
                    sum1[i] += in_data[(i, j)];
                }
                sum1[i] /= col_index1.max(1) as f64;

                // 마지막 10% 평균 (C# 코드와 동일하게: j from col_index2-1..)
                for j in (col_index2.saturating_sub(1))..ncol {
                    sum2[i] += in_data[(i, j)];
                }
                let denom = (ncol - (col_index2.saturating_sub(1))).max(1);
                sum2[i] /= denom as f64;
            }
        }

        let mut sumt = vec![0.0; nrow];
        for i in 0..nrow {
            if sum1[i] > sum2[i] {
                if sum1[i] > sum2[i] + 10.0 {
                    sumt[i] = sum2[i];
                } else {
                    sumt[i] = (sum1[i] + sum2[i]) * 0.5;
                }
            } else {
                if sum2[i] > sum1[i] + 10.0 {
                    sumt[i] = sum1[i];
                } else {
                    sumt[i] = (sum1[i] + sum2[i]) * 0.5;
                }
            }
        }

        let _t_sum: f64 = sumt.iter().sum::<f64>() / (nrow as f64);

        let mut out = in_data.clone();
        for i in 0..nrow {
            for j in 0..ncol {
                out[(i, j)] = in_data[(i, j)] - sumt[i];
            }
        }
        out
    }
```
```rust
    /// SmoothImage (NumSharp 버전) → 1D 필터 h를 각 row에 순차적으로 컨볼루션 + circular shift
    pub fn smooth_image(in_data: &Array2<f64>, level: i32) -> Array2<f64> {
        // h = [0, 0, 0.125, 0.375, 0.375, 0.125]
        let h = Array1::from(vec![0.0, 0.0, 0.125, 0.375, 0.375, 0.125]);
        let sizeh = h.len();

        let mut temp = in_data.clone();
        let (nrow, ncol) = temp.dim();
        let sizex = ncol;

        for _ in 0..level {
            let l = 1; // L = 1 (C# 코드)
            for r in 0..nrow {
                let mut sx = Array1::<f64>::zeros(sizex);
                let x = temp.row(r).to_owned(); // row clone

                for j in 0..sizeh {
                    let shift_amount =
                        -l * ((sizeh as f64) / 2.0 - (j as f64)) as isize;
                    let shifted_vec = Self::array_shift_vec(x.as_slice().unwrap(), shift_amount);
                    let shifted = Array1::from(shifted_vec);
                    sx = sx + shifted * h[j];
                }
                temp.slice_mut(s![r, ..]).assign(&sx);
            }
        }

        temp
    }
```
```rust
    /// WaveletOpn: 2D Wavelet-like 필터링
    /// 반환값: (imagTemp, Derive_r, Derive_c)
    pub fn wavelet_opn(image: &Array2<f64>, level: i32) -> (Array2<f64>, Array2<f64>, Array2<f64>) {
        // h, g 계수 (C# 그대로)
        let h = Array1::from(vec![0.0, 0.0, 0.125, 0.375, 0.375, 0.125]);
        let g = Array1::from(vec![0.0, 0.0, 0.0, -2.0, 2.0, 0.0]);

        assert_eq!(image.ndim(), 2);
        let (m, n) = image.dim();

        let _h_t = Self::cshift_1d(&h, 1); // C#에 있지만 실제 사용은 안 함

        let sizeh = h.len();
        let mut imag_temp = image.clone();
        let mut derive_r = image.clone();
        let mut derive_c = image.clone();

        for k in 0..level {
            let l = 2f64.powi(k) as isize;

            // row 방향
            for r in 0..m {
                let mut sx = Array1::<f64>::zeros(n);
                let mut wx = Array1::<f64>::zeros(n);
                let x = imag_temp.row(r).to_owned();

                for j in 0..sizeh {
                    let shift_amount =
                        -l * ((sizeh as f64) / 2.0 - (j as f64)) as isize;
                    let shifted_vec =
                        Self::array_shift_vec(x.as_slice().unwrap(), shift_amount);
                    let shifted = Array1::from(shifted_vec);

                    sx = sx + &shifted * h[j];
                    wx = wx + &shifted * g[j];
                }

                // 업데이트
                imag_temp.slice_mut(s![r, ..]).assign(&sx);
                derive_r.slice_mut(s![r, ..]).assign(&wx);
                derive_c.slice_mut(s![r, ..]).assign(&x); // 원 본
            }

            // column 방향
            for c in 0..n {
                let mut sx = Array1::<f64>::zeros(m);
                let mut wx = Array1::<f64>::zeros(m);
                let x = imag_temp.column(c).to_owned();

                for j in 0..sizeh {
                    let shift_amount =
                        -l * ((sizeh as f64) / 2.0 - (j as f64)) as isize;
                    // Derive_c^T 사용
                    let derive_c_trans = derive_c.column(c).to_owned(); // row방향 처럼 간주
                    let shifted_s =
                        Self::array_shift_vec(x.as_slice().unwrap(), shift_amount);
                    let shifted_s_arr = Array1::from(shifted_s);

                    let shifted_w =
                        Self::array_shift_vec(derive_c_trans.as_slice().unwrap(), shift_amount);
                    let shifted_w_arr = Array1::from(shifted_w);

                    sx = sx + &shifted_s_arr * h[j];
                    wx = wx + &shifted_w_arr * g[j];
                }

                // 다시 col로
                for i in 0..m {
                    imag_temp[(i, c)] = sx[i];
                    derive_c[(i, c)] = wx[i];
                }
            }
        }

        (imag_temp, derive_r, derive_c)
    }
```
```rust
    /// WidthSensor: s_sen, e_sen 범위와 온도 t에 따른 폭 추정
    pub fn width_sensor(s_sen: i32, mut e_sen: i32, t: f64) -> f64 {
        let mut th1 = 9.592;
        let mut th2 = 15.725;
        let mut th3 = 17.387;
        let mut th4 = 34.012;

        if (t - 11.1).abs() < 1e-9 {
            th1 = 9.425;
            th2 = 16.06;
            th3 = 17.722;
            th4 = 34.346;
        } else if (t - 14.3).abs() < 1e-9 {
            th1 = 9.508;
            th2 = 15.894;
            th3 = 17.556;
            th4 = 34.18;
        } else if (t - 17.5).abs() < 1e-9 {
            th1 = 9.592;
            th2 = 15.725;
            th3 = 17.387;
            th4 = 34.012;
        }

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        if e_sen - s_sen > 0 {
            // forward
            // --- head ---
            let mut s_head = 0;
            if s_sen % 3 == 0 {
                s_head = Self::fix_i32(s_sen / 3);
                a = -2;
            } else if s_sen % 3 == 1 {
                s_head = Self::fix_i32(s_sen / 3) + 1;
                a = 0;
            } else {
                s_head = Self::fix_i32(s_sen / 3) + 1;
                a = -1;
            }

            let mut e_head = 0;
            if e_sen % 3 == 0 {
                e_head = Self::fix_i32(e_sen / 3);
            } else if e_sen % 3 == 1 {
                e_head = Self::fix_i32(e_sen / 3) + 1;
                a -= 2;
            } else {
                e_head = Self::fix_i32(e_sen / 3) + 1;
                a -= 1;
            }

            // --- module ---
            let mut s_module = 0;
            if s_head % 4 == 0 {
                s_module = Self::fix_i32(s_head / 4);
                b = -3;
            } else if s_head % 4 == 1 {
                s_module = Self::fix_i32(s_head / 4) + 1;
                b = 0;
            } else if s_head % 4 == 2 {
                s_module = Self::fix_i32(s_head / 4) + 1;
                b = -1;
            } else {
                s_module = Self::fix_i32(s_head / 4) + 1;
                b = -2;
            }

            let mut e_module = 0;
            if e_head % 4 == 0 {
                e_module = Self::fix_i32(e_head / 4);
            } else if e_head % 4 == 1 {
                e_module = Self::fix_i32(e_head / 4) + 1;
                b -= 3;
            } else if e_head % 4 == 2 {
                e_module = Self::fix_i32(e_head / 4) + 1;
                b -= 2;
            } else {
                e_module = Self::fix_i32(e_head / 4) + 1;
                b -= 1;
            }

            // --- onebed ---
            let mut s_onebed = 0;
            if s_module % 2 == 0 {
                s_onebed = Self::fix_i32(s_module / 2);
                c = -1;
            } else {
                s_onebed = Self::fix_i32(s_module / 2) + 1;
                c = 0;
            }

            let mut e_onebed = 0;
            if e_module % 2 == 0 {
                e_onebed = Self::fix_i32(e_module / 2);
            } else {
                e_onebed = Self::fix_i32(e_module / 2) + 1;
                c -= 1;
            }

            a = (e_head - s_head + 1) * 2 + a;
            b = (e_module - s_module + 1) * 3 + b;
            c = (e_onebed - s_onebed + 1) * 1 + c;
            d = e_onebed - s_onebed;
        } else {
            // wrap-around case
            e_sen += 192;

            let mut s_head = 0;
            if s_sen % 3 == 0 {
                s_head = Self::fix_i32(s_sen / 3);
                a = -2;
            } else if s_sen % 3 == 1 {
                s_head = Self::fix_i32(s_sen / 3) + 1;
                a = 0;
            } else {
                s_head = Self::fix_i32(s_sen / 3) + 1;
                a = -1;
            }

            let mut e_head = 0;
            if e_sen % 3 == 0 {
                e_head = Self::fix_i32(e_sen / 3);
            } else if e_sen % 3 == 1 {
                e_head = Self::fix_i32(e_sen / 3) + 1;
                a -= 2;
            } else {
                e_head = Self::fix_i32(e_sen / 3) + 1;
                a -= 1;
            }

            let mut s_module = 0;
            if s_head % 4 == 0 {
                s_module = Self::fix_i32(s_head / 4);
                b = -3;
            } else if s_head % 4 == 1 {
                s_module = Self::fix_i32(s_head / 4) + 1;
                b = 0;
            } else if s_head % 4 == 2 {
                s_module = Self::fix_i32(s_head / 4) + 1;
                b = -1;
            } else {
                s_module = Self::fix_i32(s_head / 4) + 1;
                b = -2;
            }

            let mut e_module = 0;
            if e_head % 4 == 0 {
                e_module = Self::fix_i32(e_head / 4);
            } else if e_head % 4 == 1 {
                e_module = Self::fix_i32(e_head / 4) + 1;
                b -= 3;
            } else if e_head % 4 == 2 {
                e_module = Self::fix_i32(e_head / 4) + 1;
                b -= 2;
            } else {
                e_module = Self::fix_i32(e_head / 4) + 1;
                b -= 1;
            }

            let mut s_onebed = 0;
            if s_module % 2 == 0 {
                s_onebed = Self::fix_i32(s_module / 2);
                c = -1;
            } else {
                s_onebed = Self::fix_i32(s_module / 2) + 1;
                c = 0;
            }

            let mut e_onebed = 0;
            if e_module % 2 == 0 {
                e_onebed = Self::fix_i32(e_module / 2);
            } else {
                e_onebed = Self::fix_i32(e_module / 2) + 1;
                c -= 1;
            }

            a = (e_head - s_head + 1) * 2 + a;
            b = (e_module - s_module + 1) * 3 + b;
            c = (e_onebed - s_onebed + 1) * 1 + c;
            d = e_onebed - s_onebed;
        }

        (a as f64) * th1 + (b as f64) * th2 + (c as f64) * th3 + (d as f64) * th4
    }
}
```
```rust
pub fn calc_wavelet_main(axial: PathBuf, radial: PathBuf, cir_cum: PathBuf, sensor_no: PathBuf) {
    use WaveletHelper as WH;

    let int_axial = WH::get_ndarray_double_from_file(
        axial.to_str().unwrap(),
        40,
        100,
    );
    let int_radial = WH::get_ndarray_double_from_file(
        radial.to_str().unwrap(),
        40,
        100,
    );
    let int_cir_cum = WH::get_ndarray_double_from_file(
        cir_cum.to_str().unwrap(),
        40,
        100,
    );
    let h_sensor_count = WH::get_ndarray_int_from_file(
        sensor_no.to_str().unwrap(),
        1,
        40,
    );

    let zero_type = 2;
    let using_circum = 1;

    let mut est_width = Array2::<f64>::zeros((1, 4));

    let mut int_axial = WH::zero_min(&int_axial, zero_type);
    let mut int_radial = WH::zero_min(&int_radial, zero_type);
    let mut int_circum = WH::zero_min(&int_cir_cum, zero_type);

    let level = 1;
    int_radial = WH::smooth_image(&int_radial, level);
    int_axial = WH::smooth_image(&int_axial, level);
    int_circum = WH::smooth_image(&int_circum, level);

    int_axial = int_axial.t().to_owned();
    int_radial = int_radial.t().to_owned();
    int_circum = int_circum.t().to_owned();

    int_axial = WH::smooth_image(&int_axial, level);
    int_radial = WH::smooth_image(&int_radial, level);
    int_circum = WH::smooth_image(&int_circum, level);

    int_axial = int_axial.t().to_owned();
    int_radial = int_radial.t().to_owned();
    int_circum = int_circum.t().to_owned();

    // int_circum *= -1
    int_circum.map_inplace(|v| *v = -*v);

    let hall_count = 40_i32;
    let n_start_hall_count = hall_count / 4 + 1;
    let n_end_hall_count = hall_count / 4 * 3;

    let mut d_axial_min = 1000.0;
    let mut sensor_point = -1_i32;
    let mut distance_point = -1_i32;

    {
        let (rows, cols) = int_axial.dim();
        for i in (n_start_hall_count as usize)..=(n_end_hall_count as usize) {
            for j in 30..=70 {
                if d_axial_min > int_axial[(i, j)] {
                    d_axial_min = int_axial[(i, j)];
                    sensor_point = i as i32;
                    distance_point = j as i32;
                }
            }
        }
    }

    // 1) Circumferential 신호 이용한 폭 추정
    {
        let hall = hall_count as usize;
        let (rows, cols) = int_circum.dim();
        let sensor_point_usize = sensor_point as usize;
        let distance_point_usize = distance_point as usize;

        // max_wave1
        let mut max_wave1 = -1000.0;
        let mut sensor_max1 = -1_i32;
        for i in 5..sensor_point_usize {
            for j in 5..distance_point_usize {
                if max_wave1 < int_circum[(i, j)] {
                    max_wave1 = int_circum[(i, j)];
                    sensor_max1 = i as i32;
                }
            }
        }

        // min_wave1
        let mut min_wave1 = 1000.0;
        let mut sensor_min1 = -1_i32;
        for i in (sensor_point_usize - 1)..(hall - 5) {
            for j in 5..distance_point_usize {
                if min_wave1 > int_circum[(i, j)] {
                    min_wave1 = int_circum[(i, j)];
                    sensor_min1 = i as i32;
                }
            }
        }

        let n_col = cols;

        // min_wave2
        let mut min_wave2 = 1000.0;
        let mut sensor_min2 = -1_i32;
        for i in 5..sensor_point_usize {
            for j in (distance_point_usize - 1)..(n_col - 5) {
                if min_wave2 > int_circum[(i, j)] {
                    min_wave2 = int_circum[(i, j)];
                    sensor_min2 = i as i32;
                }
            }
        }

        // max_wave2
        let mut max_wave2 = -1000.0;
        let mut sensor_max2 = -1_i32;
        for i in (sensor_point_usize - 1)..(hall - 5) {
            for j in (distance_point_usize - 1)..(n_col - 5) {
                if max_wave2 < int_circum[(i, j)] {
                    max_wave2 = int_circum[(i, j)];
                    sensor_max2 = i as i32;
                }
            }
        }

        if (sensor_max1 - sensor_min1).abs() < 15 {
            if sensor_max1 > sensor_min1 {
                WaveletHelper::swap(&mut sensor_min1, &mut sensor_max1);
            }
        }
        if (sensor_max2 - sensor_min2).abs() < 15 {
            if sensor_max2 > sensor_min2 {
                WaveletHelper::swap(&mut sensor_min2, &mut sensor_max2);
            }
        }

        let n_cnt_sensor_min1 = h_sensor_count[(0, sensor_min1 as usize)];
        let n_cnt_sensor_max1 = h_sensor_count[(0, sensor_max1 as usize)];
        let n_cnt_sensor_min2 = h_sensor_count[(0, sensor_min2 as usize)];
        let n_cnt_sensor_max2 = h_sensor_count[(0, sensor_max2 as usize)];

        let c1 = WaveletHelper::width_sensor(
            n_cnt_sensor_max1,
            n_cnt_sensor_min1,
            17.5,
        );
        let c2 = WaveletHelper::width_sensor(
            n_cnt_sensor_max2,
            n_cnt_sensor_min2,
            17.5,
        );

        if using_circum == 1 {
            est_width[(0, 0)] = c1;
            est_width[(0, 1)] = c2;
        } else {
            est_width[(0, 0)] = 10000.0;
            est_width[(0, 1)] = 10000.0;
        }
    }

    // 2) Radial Wavelet 신호 이용
    let (radial_pull, radial_r, radial_c) =
        WaveletHelper::wavelet_opn(&int_radial, 1);

    {
        let hall = hall_count as usize;
        let (rows, cols) = radial_c.dim();
        let sensor_point_usize = sensor_point as usize;
        let distance_point_usize = distance_point as usize;

        let mut max_wave1 = -1000.0;
        let mut sensor_max1 = -1_i32;
        for i in 5..sensor_point_usize {
            for j in 5..distance_point_usize {
                if max_wave1 < radial_c[(i, j)] {
                    max_wave1 = radial_c[(i, j)];
                    sensor_max1 = i as i32;
                }
            }
        }

        let mut min_wave1 = 1000.0;
        let mut sensor_min1 = -1_i32;
        for i in (sensor_point_usize - 1)..(hall - 5) {
            for j in 5..distance_point_usize {
                if min_wave1 > radial_c[(i, j)] {
                    min_wave1 = radial_c[(i, j)];
                    sensor_min1 = i as i32;
                }
            }
        }

        let n_col = cols;

        let mut min_wave2 = 1000.0;
        let mut sensor_min2 = -1_i32;
        for i in 5..sensor_point_usize {
            for j in (distance_point_usize - 1)..(n_col - 5) {
                if min_wave2 > radial_c[(i, j)] {
                    min_wave2 = radial_c[(i, j)];
                    sensor_min2 = i as i32;
                }
            }
        }

        let mut max_wave2 = -1000.0;
        let mut sensor_max2 = -1_i32;
        for i in (sensor_point_usize - 1)..(hall - 5) {
            for j in (distance_point_usize - 1)..(n_col - 5) {
                if max_wave2 < radial_c[(i, j)] {
                    max_wave2 = radial_c[(i, j)];
                    sensor_max2 = i as i32;
                }
            }
        }

        if (sensor_max1 - sensor_min1).abs() < 15 {
            if sensor_max1 > sensor_min1 {
                WaveletHelper::swap(&mut sensor_min1, &mut sensor_max1);
            }
        }
        if (sensor_max2 - sensor_min2).abs() < 15 {
            if sensor_max2 > sensor_min2 {
                WaveletHelper::swap(&mut sensor_min2, &mut sensor_max2);
            }
        }

        let n_cnt_sensor_min1 = h_sensor_count[(0, sensor_min1 as usize)];
        let n_cnt_sensor_max1 = h_sensor_count[(0, sensor_max1 as usize)];
        let n_cnt_sensor_min2 = h_sensor_count[(0, sensor_min2 as usize)];
        let n_cnt_sensor_max2 = h_sensor_count[(0, sensor_max2 as usize)];

        let r_w1 = WaveletHelper::width_sensor(
            n_cnt_sensor_max1,
            n_cnt_sensor_min1,
            17.5,
        );
        let r_w2 = WaveletHelper::width_sensor(
            n_cnt_sensor_max2,
            n_cnt_sensor_min2,
            17.5,
        );

        est_width[(0, 2)] = r_w1;
        est_width[(0, 3)] = r_w2;
    }

    let mut min_width = 10000.0;
    for i in 0..4 {
        if min_width > est_width[(0, i)] {
            min_width = est_width[(0, i)];
        }
    }
    let est_w = min_width;

    println!("int_circum =\n{:?}", int_circum);
    println!("radial_c =\n{:?}", radial_c);
    println!("estimated width = {}", est_w);
}
```
---

