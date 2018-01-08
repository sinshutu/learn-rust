pub fn eratosthenes(n: i32) -> Vec<i32> {
    let mut arr: Vec<i32> = init(n);
    let mut rtn: Vec<i32> = Vec::new();
    if arr.len() == 0 {
        return rtn
    }
    loop {
        let (prime, tail) = head_tail(arr);
        rtn.push(prime[0]);
        if tail.len() == 0 {
            break;
        }
        arr = reject_n_times(tail.to_vec(), prime[0]);
    }
    rtn
}

fn head_tail(v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let (a, b) = v.split_at(1);
    (a.to_vec(), b.to_vec())
}

fn init(n: i32) -> Vec<i32> {
    (2..n+1).collect()
}

fn reject_n_times(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut rtn = vec![];
    for i in v {
        if i%n != 0 {
            rtn.push(i);
        }
    }
    rtn
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_init() {
        assert_eq!(init(0), []);
        assert_eq!(init(5), [2, 3, 4, 5]);
    }

    #[test]
    fn run_reject_n_times() {
        assert_eq!(reject_n_times(vec![3, 4, 5, 6], 2), [3, 5]);
        assert_eq!(reject_n_times(vec![5, 7, 9], 3), [5, 7]);
    }

    #[test]
    fn when_prime_number_len_zero() {
        assert_eq!(eratosthenes(0), []);
    }

    #[test]
    fn when_prime_number_len_2() {
        assert_eq!(eratosthenes(2), [2]);
    }
    #[test]
    fn when_prime_number_len_3() {
        assert_eq!(eratosthenes(3), [2, 3]);
    }
}
