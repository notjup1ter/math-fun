Initial Approach: iteratively create fibonacci vector to fill all values < 4000000, then filter out those that are odd then sum the remaining vector.

--Runtime
---O(N) due to iterator methods
---Benchmark Data
138.29 ns with 139.11 upper bound

One way I found to optimize this was to reduce the size of the fib array to ONLY be even numbers. The Fibonacci even numbers have a unique pattern that if starting with [0,2] then each following Fibonacci even, F_e, follows the pattern of F_e = 4F_(e-1) + F(e-2). This eliminates the odds altogether and makes iteration a bit faster. I additionally did not use a helper function for creating the sequence to reduce extra function calls.

--Runtime
---Still O(N), but reduced space and so technically its O(N/3)
---Benchmark Data
85.745 ns with a 85.936 upper bound
-1.61x faster than the original approach




