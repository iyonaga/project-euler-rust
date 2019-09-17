fn solve(input: u64) -> u64 {
    (1..=input).fold(1, |total, cur| lcm(total, cur))
}

fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    println!("Problem 005: {}", solve(20));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p005_test() {
        assert_eq!(2520, super::solve(10));
        assert_eq!(232792560, super::solve(20));
    }
}
