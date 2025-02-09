fn main() {
    let max_div: i32 = 20;
    let mut curr_num: i64 = max_div as i64;

    while !is_evenly_div(curr_num, max_div){
        curr_num += 1;
    }

    println!("{curr_num}");
}

fn is_evenly_div(num: i64, max_div: i32) -> bool{
    for i in 1..=max_div {
        if num % i as i64 != 0 {
            return false;
        }
    }
    return true;
}

/*
max_num = 20

found = False
num = max_num
while not found:
    for i in range(1, max_num + 1):
        if num % i != 0:
            num += 1
            break
    else:
        print(num)
        break
*/
