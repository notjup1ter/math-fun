

/// Returns the sum of all multiples of three or five, from 0 to the upper limit n, iteratively
/// 
/// The input and output are unsigned, 32-bit integers
pub fn sum_multiples_3or5_iter(n:u32) -> u32 {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

/// Same as sum_multiples_3or5_iter, but using Carl Guass' formula for a given k
/// 
/// Input n is the same as above, but k represents the number to find multiples of
pub fn sum_multiples_k_guass(n:u32, k: u32) -> u32 {
    let num_valid_terms: u32 = (n-1) / k;
    k * (num_valid_terms * (num_valid_terms+1) / 2)
}

/// Calculates the sum of all multiples of both 3 and 5 using Carl Guass' formula
pub fn sum_multiples_3or5_guass(n:u32) -> u32 {
    let sum_3 = sum_multiples_k_guass(n, 3);
    let sum_5 = sum_multiples_k_guass(n, 5);
    let sum_15 = sum_multiples_k_guass(n, 15); // to handle multiples of both (duplication)
    sum_3 + sum_5 - sum_15
}


///Unit Testing

#[cfg(test)]
mod tests {
    use super::*;

    // Testing correctness of iterative version
    #[test]
    fn iterative_tests() {
        assert_eq!(sum_multiples_3or5_iter(1000), 233168);
        assert_eq!(sum_multiples_3or5_iter(4), 3);
        assert_eq!(sum_multiples_3or5_iter(2), 0);
    }

    // Testing correctness of Guass' formula implementation
    #[test]
    fn guass_tests() {
        assert_eq!(sum_multiples_k_guass(1000, 3), 166833);
        assert_eq!(sum_multiples_k_guass(1000, 5) , 99500);
    }

    // Testing equality of both types of functions with original problem
    #[test]
    fn compare_guass_and_iterative() {
        assert_eq!(sum_multiples_3or5_guass(1000), sum_multiples_3or5_iter(1000));
    }
}