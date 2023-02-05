import matplotlib
import numpy as np
from matplotlib import pyplot as plt
from numpy import genfromtxt

n = 8192

hat_data = np.genfromtxt('../data/hatAvgAccessCycles.txt', delimiter=',')[:-1]
vec_data = np.genfromtxt('../data/vecAvgAccessCycles.txt', delimiter=',')[:-1]

# Parse for pgfplot coordinates
for (a,b) in zip(np.arange(0, n), vec_data):
    print(f"({a}, {b})")
