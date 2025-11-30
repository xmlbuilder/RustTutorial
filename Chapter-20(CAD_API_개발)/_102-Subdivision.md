# Subdivision

## 1D Subdivision Algorithms (subdivide & subdivide_adaptive)

### Overview

This document describes two 1‑dimensional subdivision algorithms used to  
generate mesh points along a line segment or polyline:

- 1.  **subdivide** -- metric‑based linear subdivision
- 2.  **subdivide_adaptive** -- target‑aware exponential subdivision with  
    smooth start/end grading

Both methods generate: 
- New point positions along the parametric axis `pos` 
- Corresponding metric values at each new point `metrics` 
- Resulting index list representing subdivided nodes `indices`

### 1. subdivide - Linear Metric-Based Subdivision

#### Inputs

-   `pos[i]` --- coordinate of ith node
-   `metrics[i]` --- desired element size at ith node
-   `nodes[k]` --- index list defining a polyline
-   `force_up` --- round‑up subdivision count
-   `force_even` --- force even total element count
-   `min_n` --- minimum edges per segment
-   `max_n` --- maximum edges per segment
-   `indices` --- output reordered node indices

#### Purpose
Divide each polyline segment proportionally to its metric value, 
- ensuring: 
    - No segment has fewer than `min_n` edges 
    - No segment has more than `max_n` edges 
    - Global evenness when requested

#### Ideal Edge Count Formula

- For each segment:

```
    length = abs(x1 - x0)
    m_avg  = (m0 + m1) * 0.5

    ideal_edges = length / m_avg

    if ideal_edges < 1 => ideal_edges = 1
    if force_up       => edges = ceil(ideal_edges)
    else               => edges = round(ideal_edges)

    edges = clamp(edges, min_n, max_n)
```

#### New Points

Interior new points computed via linear interpolation:
```
    t = k / edges
    x_new = (1 - t)*x0 + t*x1
    m_new = (1 - t)*m0 + t*m1
```

---

## 2. subdivide_adaptive - Smooth Target-Based Adaptive Subdivision

### New Concept

Instead of linear interpolation of metric values, we construct a **smooth bell‑shaped metric curve**:

-   Start size → increase smoothly
-   Reach (or attempt to reach) a target size
-   Maintain plateau if possible
-   Decrease smoothly to end size
-   Never exceed target
-   Reduce total node count as much as possible

### Inputs
- start_size  = metrics[nodes[0]]
- end_size    = metrics[nodes.back()]
- target_size = maximum desired metric
- length      = pos[end] - pos[start]
- edges_min   = minimal acceptable subdivisions
- edges_max   = maximal acceptable subdivisions

### Parameterization

Normalize coordinate:

```
u = (x - x_start) / length; 0 <= u <= 1
```
### Shape Functions

We define three regions:

- 1.  **Growth region** (`u < u_peak`)
- 2.  **Plateau region** (`u_peak <= u <= 1 - u_peak`)
- 3.  **Decay region**

- A smooth exponential-like growth is used:
```
g(u) = start_size + (target_size - start_size) *
       (1 - exp(-k * u)) / (1 - exp(-k))
```

- Similarly, decay is symmetric:
```
d(u) = end_size + (target_size - end_size) *
       (1 - exp(-k * (1 - u))) / (1 - exp(-k))
```

- Final metric profile:
```
if u < u_peak      => m(u) = g(u)
if u_peak < u < 1-u_peak => m(u) = target_size
else               => m(u) = d(u)
```

- `u_peak` is determined by checking if the start  
    → target and target  
    → end growth curves meet without crossing.  

### Ideal Edge Count

- Total edge count is determined using average metric:
```
avg_m = integral(m(u) du) / 1.0

ideal_edges = length / avg_m
ideal_edges = clamp(round(ideal_edges), edges_min, edges_max)
```

### Node Placement
- Nodes are placed so that element size approximates `m(u)`:
```
x_next = x_prev + m(u_prev)
repeat until x reaches end
```

---

## Example Metric Profiles

### Linear (subdivide)

- Smooth linear interpolation of metric values.

### Adaptive (subdivide_adaptive)

- Smooth exponential bell curve reaching target region only when possible.

---

## Summary Table

| Feature               | subdivide       | subdivide_adaptive |
|----------------------|----------------|--------------------|
| Metric interpolation | Linear         |  Exponential + Plateau |
| Target size limit    |  No            |   Yes                  |
| Mesh count reduction |  Minimal       |   Strong               |
| Smoothness           |  Moderate      |   Very high            |
| Good for             |  Simple grading |   Boundary layers, curved features |

---

## Output

- Both functions return:
```
ret >= 0  => number of generated edges
ret < 0   => error code
```
- And output updated:
```
pos[]      ; x-coordinates
metrics[]  ; metric values at new nodes
indices[]  ; ordering of subdivided nodes
```
---

