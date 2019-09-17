fn solve(input: u64) -> u64 {
    let sum_of_squares: u64 = (1..=input).map(|x| x * x).sum();
    let sum: u64 = (1..=input).sum();
    let square_ob_sum = sum.pow(2);
    square_ob_sum - sum_of_squares
}

fn main() {
    println!("Problem 006: {}", solve(100));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p006_test() {
        assert_eq!(2640, super::solve(10));
        assert_eq!(25164150, super::solve(100));
    }
}
