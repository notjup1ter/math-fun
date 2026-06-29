//This is a function that will handle the sum of all multiples of 3 and 5 from 1 to any given n > 1
pub fn sum_multiples_3or5(n:u32) -> u32 {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}