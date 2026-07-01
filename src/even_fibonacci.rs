/// Returns the sum of all even fibbonacci numbers that do not exceed 
/// input n



pub fn create_fib_sequence(n:u32) -> Vec<u32> {
    let mut fib_vec : Vec<u32> = vec![1, 2];

    // Fibonacci sequence construction
    loop {
        let temp_sum: u32 = fib_vec[fib_vec.len()-1] + fib_vec[fib_vec.len()-2];
        if temp_sum > n {break;} else {fib_vec.push(temp_sum)}
    }
    fib_vec
}

pub fn even_fib_sum(n: u32) -> u32 {
    create_fib_sequence(n).into_iter()
        .filter(|x| x % 2 == 0)
        .sum()
}

/// Constructs a vector of only even fibonacci numbers up to input n
/// and returns its sum
pub fn even_fib_sum_v2(n: u32) -> u32 {
    let mut fib_vec: Vec<u32> = vec![0,2];

    loop {
        let temp_sum: u32 = 4 * fib_vec[fib_vec.len() - 1] + fib_vec[fib_vec.len() - 2];
        if temp_sum > n {break;} else {fib_vec.push(temp_sum);}
    }
    fib_vec.into_iter().sum()
}




/// Unit Testing

#[cfg(test)]

mod tests {
    use super::*;

    // Tests for ensuring Fibonacci sequences are created correctly
    #[test]
    fn fib_seq_creation_test() {
        assert_eq!(create_fib_sequence(100), vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }

    // Test for ensuring the proper sum of even Fibonacci numbers is
    // produced
    #[test]
    fn even_fib_sum_test() {
        assert_eq!(even_fib_sum(4000000), 4613732)
    } 

    // Test for correctness on version 2 of the even sum algorithm
    #[test]
    fn even_fib_sum_v2_test() {
        assert_eq!(even_fib_sum_v2(4000000), 4613732)
    }
}