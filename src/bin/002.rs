fn solve() -> i32 {
    let limit = 4_000_000;
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;

    loop {
        if c % 2 == 0 {
            sum += c;
        }

        if c > limit {
            break;
        }

        c = a + b;
        a = b;
        b = c;
    }

    sum
}

fn main() {
    println!("Problem 002: {}", solve());
}

#[cfg(test)]
mod tests {
    #[test]
    fn p002_test() {
        assert_eq!(4613732, super::solve());
    }
}
