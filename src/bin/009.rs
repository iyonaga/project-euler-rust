fn solve(input: u64) -> u64 {
    let mut result: u64 = 0;

    for a in 1..input {
        for b in a..input {
            if a + b >= input {
                continue;
            }

            let c = input - a - b;
            if c > 0 && a * a + b * b == c * c {
                result = a * b * c;
                break;
            }
        }
    }

    result
}

fn main() {
    println!("Problem 009: {}", solve(1000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p009_test() {
        assert_eq!(31875000, super::solve(1000));
    }
}
