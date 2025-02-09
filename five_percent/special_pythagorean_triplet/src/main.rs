fn main() {
    let max = 1000;
    for i in 1..=max {
        for j in (i + 1)..=max {
            for k in (j + 1)..=max {
                let sum = i + j + k;
                // println!("{i} {j} {k}");
                if sum < max {
                    continue;
                } else if sum == max {
                    if is_triplet(i, j, k) {
                        println!("Result: {}", i * j * k);
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn is_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}


