fn solve(divisors: u64) -> u64 {
    let mut answer: u64 = 0;

    for i in 2.. {
        let num: u64 = (1..i).sum();

        if get_factors_total(num) > divisors {
            answer = num;
            break;
        }
    }

    answer
}

fn get_factors_total(num: u64) -> u64 {
    let mut factors: Vec<u64> = Vec::new();
    let mut i: u64 = 1;

    while i * i < num {
        if num % i == 0 && !factors.contains(&i) {
            factors.push(i);
            factors.push(num / i);
        }

        i += 1;
    }

    factors.len() as u64
}

fn main() {
    println!("Problem 012: {}", solve(500));
}

#[cfg(test)]
mod tests {
    #[test]
    fn p012_test() {
        assert_eq!(28, super::solve(5));
        assert_eq!(76576500, super::solve(500));
    }
}
