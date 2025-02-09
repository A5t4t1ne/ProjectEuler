use primes::{PrimeSet, Sieve};

fn main() {
    let mut pset = Sieve::new();
    let n = 2_000_000;
    let mut sum: u64 = 0;
    for (_, prime) in pset.iter().enumerate(){
        if prime >= n {
            break;
        }
        sum += prime;
    }
    println!("{sum}");
}


/*
 * The sum of the primes below $10$ is $2 + 3 + 5 + 7 = 17$.
 * Find the sum of all the primes below two million.
 */
