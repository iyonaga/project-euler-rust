fn solve(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for i in 2..=limit {
        if is_prime(i) {
            sum += i;
        }
    }

    sum
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    } else if num == 2 {
        return true;
    } else if num % 2 == 0 {
        return false;
    }

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn main() {
    println!("Problem 010: {}", solve(2_000_000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p010_test() {
        assert_eq!(17, super::solve(10));
        assert_eq!(142913828922, super::solve(2_000_000));
    }
}
