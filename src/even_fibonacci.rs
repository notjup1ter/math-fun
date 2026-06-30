/// Returns the sum of all even fibbonacci numbers that do not exceed 
/// input n

fn create_fib_sequence(n:u32) -> Vec<u32> {
    let mut fib_vec : Vec<u32> = vec![1, 2];

    // Fibonacci sequence construction
    loop {
        let temp_sum: u32 = fib_vec[fib_vec.len()-1] + fib_vec[fib_vec.len()-2];
        if temp_sum > n {break;} else {fib_vec.push(temp_sum)}
    }
    fib_vec
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
}