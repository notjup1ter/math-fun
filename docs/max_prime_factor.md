# Max Prime Factor

## Initial approach: Sieve of Eratosthenes
Although it will __compile__, it will absolutely not actually run. Why is that? The reason is that the original problem wants us to find the largest prime factor of a number ~600 billion! If we were to take the approach of initializing all numbers between 2 and that, itll be 600 GB OF RAM! Therefore, we need a solution that maximizes memory as well.