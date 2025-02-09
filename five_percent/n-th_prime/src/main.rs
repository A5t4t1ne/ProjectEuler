use primes::{PrimeSet, Sieve};

fn main() {
    let mut pset = Sieve::new();
    let n = 10001;
    if let Some((_, prime)) = pset.iter().enumerate().nth(n-1) {
        println!("The {}th prime number is: {}", n, prime);
    }
}
