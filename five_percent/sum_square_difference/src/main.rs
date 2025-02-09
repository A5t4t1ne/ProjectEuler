


fn main() {
    let num = 100;
    let a = square_of_sum(num);
    let b = sum_of_squares(num);
    println!("{a} - {b} = {}", a-b);
}


fn sum_of_squares(max_num: i32) -> i64 {
    let mut sum: i64 = 0;
    
    for i in 1..=max_num {
        sum += (i * i) as i64;
    }
    sum
}

fn square_of_sum(max_num: i32) -> i64 {
    let sum: i64 = (1..=max_num as i64).sum();
    
    sum * sum
}
