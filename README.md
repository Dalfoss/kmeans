# kmeans
The purpose of this project is to implement a version of kmeans as a python
module written in Rust.  The goal is a better performing kmeans module for
python, than is currently available through packages like sklearn.

## Improvements over other implementations
### Point sorting
Probably the biggest improvement over other kmeans implementations I've seen out
there - including sklearn - is the changes I've made to how points - or 'cases'
if you will - are sorted to their closest centroid.

The idea is actually quite simple, and probably not novel at all. Instead of
calculating the squared distance from all centroids to all points in one go, one
should take a step back and think about what information is acutally needed.

To determine which points belongs to which centroids, the only information that
has to be present in memory is the location of all centroids, and the one point
we are sorting.  Which is why it is important that we calculate the distance
from one point, to all the centroids.
if we do the calculation the other way around, and find the distance from a
given centroid to all the points, we will have to keep a `k*n` matrix in memory to
do the final sorting, instead of just a `1*n` list.

When you have calculated the distance to all centroids for that one point, you
can iterate through the list of distances, and find the smallest right off the
bat. Resulting in only having to store that one value in memory.

The advantage of this approach is two fold:

First there is the obvious reduction in memory usage, by a factor of `k` with k
being the number of centroids.

But there is also another advantage that i didn't realize the significance of,
until i had made the implementation. It turned out that this aproach had not
only reduced the memory consumption dramatically, but also the runtime by as
much as a factor of 10.  I thought about this for a moment, and it makes
sense. Instead of having to write this big list of `1*n` distances from one
point to all centroids to memory, only to later find the minimum of, the
comparisons needed to determine the minimum distance was done right away. And
only one number was written to the final array in memory, that was needed to
move the centroids.

I consider the above a great example of how keeping read and writes to memory on
the heap in mind, can dramatically improve the performance of your code. Being
new to low level systems programming languages like Rust, it is a lesson i wont
soon forget.

# Future prospects
If this kmeans implementations sparks sufficient interest in the community,
implementations other ML algorithms may be considered in the future.