fn solve(digit: u32) -> u64 {
    let mut max_pal: u64 = 0;
    let min = 10_u64.pow(digit - 1);
    let max = 10_u64.pow(digit) - 1;

    for x in min..=max {
        for y in min..=max {
            let prod = x * y;
            if is_palindromic(prod) && prod > max_pal {
                max_pal = prod;
            }
        }
    }

    max_pal
}

fn is_palindromic(num: u64) -> bool {
    let s = num.to_string();
    s.chars().rev().collect::<String>() == s
}

fn main() {
    println!("Problem 004: {}", solve(3));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p004_test() {
        assert_eq!(9009, super::solve(2));
        assert_eq!(906609, super::solve(3));
    }
}
