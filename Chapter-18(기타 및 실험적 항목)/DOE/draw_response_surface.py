import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401 (needed for 3D)

# 1. CSV 읽기
df = pd.read_csv("surface_points.csv")  # columns: x1,x2,y

# 2. 격자로 reshape
#   nx, ny는 Rust에서 썼던 것과 동일해야 함
nx = 50
ny = 50

x1_vals = df["x1"].values.reshape((nx, ny))
x2_vals = df["x2"].values.reshape((nx, ny))
y_vals  = df["y"].values.reshape((nx, ny))

# 3D surface plot
fig = plt.figure(figsize=(8, 6))
ax = fig.add_subplot(111, projection="3d")
ax.plot_surface(x1_vals, x2_vals, y_vals, cmap="viridis", edgecolor="none")
ax.set_xlabel("X1")
ax.set_ylabel("X2")
ax.set_zlabel("Y")
ax.set_title("Response Surface (Quadratic)")
plt.tight_layout()
plt.show()

# 2D contour plot
plt.figure(figsize=(6, 5))
cont = plt.contourf(x1_vals, x2_vals, y_vals, levels=20, cmap="viridis")
plt.colorbar(cont, label="Y")
plt.xlabel("X1")
plt.ylabel("X2")
plt.title("Response Surface Contour")
plt.tight_layout()
plt.show()
