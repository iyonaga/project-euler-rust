fn solve(input: u64) -> u64 {
    let mut prime_factors = 0;
    let mut d = 2;
    let mut n = input;

    while n > 1 {
        while n % d == 0 {
            prime_factors = d;
            n /= d;
        }

        d += 1;
    }

    prime_factors
}

fn main() {
    println!("Problem 003: {}", solve(600851475143));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p003_test() {
        assert_eq!(6857, super::solve(600851475143));
    }
}
