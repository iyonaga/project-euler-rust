fn solve(input: u64) -> u64 {
    let mut num: u64 = 1;
    let mut count: u64 = 0;

    while count != input {
        num += 1;

        if is_prime(num) {
            count += 1;
        }
    }

    num
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
    println!("Problem 007: {}", solve(10001));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p007_test() {
        assert_eq!(13, super::solve(6));
        assert_eq!(104743, super::solve(10001));
    }
}
