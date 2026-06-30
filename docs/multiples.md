The initial approach I took was a brute force method to practice iterators
this is O(n) time, with n being the upper limit that is provided in the input

Initial benchmark for all multiples up to (excluding) 100000:
-- 26.354 microseconds
-- upper bound of 26.365 microseconds

The problem with this approach is that it is still iterative, but a smart guy named Carl Friedrich Guass came up with a way to approximate this sum usig symmetric addition.
the general formula for this is k * n(n+1) / 2
The way this works is by: 
1. obtaining the number of elements that qualify as a multiple of k as a list (1..n)
2. adding the left and right elements together, making n pairs of n + 1 -> n(n+1)
3. dividing this by 2 to avoid two passes of pairs
4. multipling that estimate of pairs by k to get the sum of those elements (notice initial division of k from the list) 

Benchmark for all multiples up to (excluding) 100000:
-- 1.495 NANOSECONDS
-- upper bound of 1.4976 ns 
-- 25000x faster that the iterative version, how cool!