
/// Returns the highest prime number for a given input, n. 
/// uses Sieve of Eratosthenes

pub fn max_prime_factor(n: usize) -> u32 {
    let mut is_prime: Vec<bool> = vec![true; n + 1];
    let p: u32 = 2;

    while p * p <= (n) {
        if is_prime[p] {
            for i in (p*p..n+1).step_by(p) {
                prime[i] = false;
            }
        }
        p += 1;
    }

    let mut primes: Vec<f64> = Vec::<f64>::new();
    for num in 2..n+1 {
        if is_prime[num] {primes.push(num);}
    }

    primes[is_prime.len() - 1]

}


/// Tests for the largest prime factor algorithm
#[cfg(test)] 

mod tests {
    use super::*;

    #[test]
    fn max_prime_factor_test() {
        assert_eq!(max_prime_factor(600851475143.0), 6857);
    }
}