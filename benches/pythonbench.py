import time
import sys
import random
from sklearn.cluster import KMeans
import numpy as np

sys.path.append("../target/release/")

import libedist



def run_kmeans(points, centroids):
    start = time.perf_counter()
    final_centroids = libedist.kmeans(centroids, points)
    end = time.perf_counter()

    delta = end - start
    print("Runtime: %f seconds" % delta)
    return final_centroids




def run_sk_kmeans(points, centroids):
    start = time.perf_counter()
    kmeans = KMeans(init=centroids, n_init=1, verbose=1, n_clusters=1000).fit(points)
    end = time.perf_counter()
    delta = end - start
    print("Runtime: %f seconds" % delta)

    return kmeans.cluster_centers_

if __name__ == "__main__":

    # sklean points and centroids
    sk_points = np.random.randint(1000, size=(100000,2))
    sk_centroids = sk_points[np.random.choice(len(sk_points)-1, 1000)]

    # libedist points and centroids
    points = []
    centroids = []

    for point in sk_points:
        points.append((point[0], point[1]))
    for centroid in sk_centroids:
        centroids.append((centroid[0], centroid[1]))
    result = run_kmeans(points, centroids)
    sk_result = run_sk_kmeans(sk_points, sk_centroids)
    
#    run_kmeans()
