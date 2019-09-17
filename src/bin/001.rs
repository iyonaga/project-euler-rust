fn solve(limit: u32) -> u32 {
    (0..limit).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

fn main() {
    println!("Problem 001: {}", solve(1000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p001_test() {
        assert_eq!(23, super::solve(10));
        assert_eq!(233168, super::solve(1000));
    }
}
