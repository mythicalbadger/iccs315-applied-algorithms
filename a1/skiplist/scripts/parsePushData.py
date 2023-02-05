import numpy as np
from numpy import genfromtxt

n = 8192

skiplist_data = np.genfromtxt('../data/skipListIndividualPushData.txt', delimiter=',')[:-1]
map_data = np.genfromtxt('../data/orderedMapIndividualPushData.txt', delimiter=',')[:-1]

def parse(data_type=0):
    data = map_data
    if data_type != 0:
        data = skiplist_data

    for (a, b) in zip(range(0, n), data):
        print("({}, {})".format(str(a), str(b)))

parse(1)
