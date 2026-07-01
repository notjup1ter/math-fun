
/// Returns the highest prime number for a given input, n. 
/// uses Sieve of Eratosthenes

pub fn max_prime_factor(n: i64) -> i64 {
    let mut is_prime: Vec<bool> = vec![true; (n + 1) as usize];
    let mut p: i64 = 2;

    while p * p <= (n) {
        if is_prime[p as usize] {
            for i in (p*p..n+1).step_by(p as usize) {
                is_prime[i as usize] = false;
            }
        }
        p += 1;
    }

    let mut primes: Vec<i64> = Vec::<i64>::new();
    for num in 2..n+1 {
        if is_prime[num as usize] {primes.push(num);}
    }

    primes[is_prime.len() - 1]

}


/// Tests for the largest prime factor algorithm
#[cfg(test)] 

mod tests {
    use super::*;

    #[test]
    fn max_prime_factor_test() {
        assert_eq!(max_prime_factor(600_851_475_143), 6857);
    }
}