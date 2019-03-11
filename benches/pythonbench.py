import time
import sys
import random
sys.path.append("../target/release/")

import libedist

def run_kmeans():
    points = []
    centroid = []
    for i in range(100):
        centroid.append((random.randint(1,1000),random.randint(1,1000)))
    for i in range(1000000):
        points.append((random.randint(1,1000),random.randint(1,1000)))

    start = time.perf_counter()
    resr = libedist.kmeans(centroid, points)
    end = time.perf_counter()

    delta = end - start
    print("Runtime: %f seconds" % delta)
    sys.stdout.flush()
    return centroid

def test_my_stuff(benchmark):
    result = benchmark(run_kmeans)

if __name__ == "__main__":
    run_kmeans()
