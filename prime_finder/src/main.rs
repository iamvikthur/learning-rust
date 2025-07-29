fn generate_primes(n: u32) -> Vec<u32> {
    if n < 2 {
        return vec![];
    }

    // USING Sieve of Eratosthenes
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i as u32) } else { None })
        .collect()
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn count_primes(vec: &Vec<u32>) -> usize {
    vec.iter().filter(|&&x| is_prime(x)).count()
}

fn filter_primes(vec: &Vec<u32>) -> Vec<u32> {
    vec.iter().cloned().filter(|&x| is_prime(x)).collect()
}

fn main() {
    let limit = 20;
    let primes = generate_primes(limit);
    println!("Prime numbers up to {}: {:?}", limit, primes);

    let numbers = vec![10, 5, 8, 12, 7, 17];
    let count = count_primes(&numbers);
    let filtered_primes = filter_primes(&numbers);

    println!("Count of primes in the vector: {}", count);
    println!("Filtered primes: {:?}", filtered_primes);
}
