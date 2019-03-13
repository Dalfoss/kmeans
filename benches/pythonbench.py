import time
import sys
import random
from sklearn.cluster import KMeans
import numpy as np

sys.path.append("../target/release/")

import libedist



def run_kmeans(points, k):
    start = time.perf_counter()
    final_centroids = libedist.kmeans(points, k)
    end = time.perf_counter()

    delta = end - start
    print("Runtime: %f seconds" % delta)
    return final_centroids




def run_sk_kmeans(points, k):
    start = time.perf_counter()
    kmeans = KMeans(n_init=1, verbose=1, n_clusters=k).fit(points)
    end = time.perf_counter()
    delta = end - start
    print("Runtime: %f seconds" % delta)

    return kmeans.cluster_centers_

if __name__ == "__main__":
    # sklean points and centroids
    sk_points = np.random.randint(1000, size=(100000,2))

    # libedist points and centroids
    points = []
    for point in sk_points:
        points.append((point[0], point[1]))
    result = run_kmeans(points, 100)
    sk_result = run_sk_kmeans(sk_points, 100)
