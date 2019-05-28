import time
import sys
import random
from sklearn.cluster import KMeans
import numpy as np
import matplotlib.pyplot as plt
sys.path.append("../target/release/")

import libedist
   
def run_kmeans(points, k):
    for i in ["regular", "multithreaded"]:
        start = time.perf_counter()
        final_centroids = libedist.kmeans(points, k, "kmeans_pp", i)
        end = time.perf_counter()

        delta = end - start
        print("Runtime " +i+": %f seconds" % delta)
    return final_centroids


def run_kmeans_random(points, k):
    for i in ["regular", "multithreaded"]:
        start = time.perf_counter()
        final_centroids = libedist.kmeans(points, k, "random", i)
        end = time.perf_counter()

        delta = end - start
        print("Runtime " +i+": %f seconds" % delta)
    return final_centroids


def run_sk_kmeans(points, k):
    start = time.perf_counter()
    kmeans = KMeans(verbose=1, n_clusters=k).fit(points)
    end = time.perf_counter()
    delta = end - start
    print("Runtime: %f seconds" % delta)

    return kmeans.cluster_centers_

def kmeans_score(points, centroids):
    score = 0
    for i in centroids:
        for j in points:
            score += ((i[0]-j[0])**2 + (i[1]-j[1])**2)/1e6
    return score

if __name__ == "__main__":
    # sklean points and centroids
    sk_points = np.loadtxt('data/birchgrid.txt')
 
    
    # libedist points and centroids
    points = []
    for point in sk_points:
        points.append([point[0], point[1]])
#    result1 = run_kmeans_random(points, 100)
    result2 = run_kmeans(points, 100)
    sk_result = run_sk_kmeans(sk_points, 100)
    print(len(result2))
    result2x = [i[0] for i in result2]
    result2y = [i[1] for i in result2]
    skx = [i[0] for i in sk_result]
    sky = [i[1] for i in sk_result]


#    print("libedist score: " + str(kmeans_score(points, result2)))
#    print("sk score: " + str(kmeans_score(sk_points, sk_result)))
    
    plt.plot(skx, sky, 'o', color='green')
    plt.plot(result2x, result2y, 'x', color='red')
    plt.show()
