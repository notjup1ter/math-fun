//This is a function that will handle the sum of all multiples of 3 and 5 from 1 to any given n > 1
pub fn sum_multiples_3or5(n:u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {sum += i;}
    }
    sum
}