#[cfg(test)]
fn generate_primes_division(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for num in 2..=limit {
        if primes.iter().all(|&prime| num % prime != 0) {
            primes.push(num);
        }
    }
    primes
}

fn generate_primes_eratosthenes(n: usize) -> Vec<u64> {
    let mut sieve = vec![true; n + 1];
    let mut p = 2;
    while p * p <= n {
        if sieve[p] {
            let mut i = p * p;
            while i <= n {
                sieve[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    sieve
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(pr, &is_prime)| if is_prime { Some(pr as u64) } else { None })
        .collect()
}

fn prime_factors(n: u64, primes: &[u64]) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;
    for &prime in primes {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn main() {
    let num = 600851475143;
    let limit = (num as f64).sqrt() as u64;
    let primes = generate_primes_eratosthenes(limit.try_into().unwrap());
    println!("Done generating primes, generated: {}", primes.len());
    let factors = prime_factors(num, &primes);
    println!("Prime factors of {} are: {:?}", num, factors);
    let biggest = factors.iter().max().unwrap();
    println!("Biggest prime factor of {} is: {}", num, biggest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_primes() {
        assert_eq!(generate_primes_division(10), vec![2, 3, 5, 7]);
        assert_eq!(
            generate_primes_division(20),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            generate_primes_division(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_prime_factors() {
        let primes = generate_primes_division(30);
        assert_eq!(prime_factors(10, &primes), vec![2, 5]);
        assert_eq!(prime_factors(20, &primes), vec![2, 2, 5]);
        assert_eq!(prime_factors(30, &primes), vec![2, 3, 5]);
        assert_eq!(prime_factors(35, &primes), vec![5, 7]);
    }

    #[test]
    fn test_generate_primes_eratosthenes() {
        assert_eq!(generate_primes_eratosthenes(10), vec![2, 3, 5, 7]);
        assert_eq!(
            generate_primes_eratosthenes(20),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            generate_primes_eratosthenes(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_prime_factors_eratosthenes() {
        let primes = generate_primes_eratosthenes(30);
        assert_eq!(prime_factors(10, &primes), vec![2, 5]);
        assert_eq!(prime_factors(20, &primes), vec![2, 2, 5]);
        assert_eq!(prime_factors(30, &primes), vec![2, 3, 5]);
        assert_eq!(prime_factors(35, &primes), vec![5, 7]);
    }
}
