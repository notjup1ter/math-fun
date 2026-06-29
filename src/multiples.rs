//This is a function that will handle the sum of all multiples of 3 and 5 from 1 to any given n > 1
pub fn sum_multiples_3or5_iter(n:u32) -> u32 {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

//here, k is the multiple, and n is the upper limit
pub fn sum_multiples_k_guass(n:u32, k: u32) -> u32 {
    let num_valid_terms: u32 = (n-1) / k;
    k * (num_valid_terms * (num_valid_terms+1) / 2)
}

pub fn sum_multiples_3or5_guass(n:u32) -> u32 {
    let sum_3 = sum_multiples_k_guass(n, 3);
    let sum_5 = sum_multiples_k_guass(n, 5);
    let sum_15 = sum_multiples_k_guass(n, 15); // to handle multiples of both (duplication)
    sum_3 + sum_5 - sum_15
}