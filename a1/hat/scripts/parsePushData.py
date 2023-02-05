import matplotlib
import numpy as np
from matplotlib import pyplot as plt
from numpy import genfromtxt

n = 8192

hat_data = np.genfromtxt('../data/hatAvgPushCycles.txt', delimiter=',')[:-1]
vec_data = np.genfromtxt('../data/vecAvgPushCycles.txt', delimiter=',')[:-1]

# Print in coordinate form for pgfplots
for (a, b) in zip(np.arange(0, n), hat_data):
    print(f"({a}, {b})")
