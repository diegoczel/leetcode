fn main() {
    println!("Hello, world!");
}


fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut s: i32 = 0;
    let mut r = vec![0; nums.len()];

    for (ind, val) in nums.iter().enumerate() {
        s = s + val;
        r[ind] = s;
    }

    r
}

#[cfg(test)]
mod tests {
    use crate::running_sum;

    #[test]
    fn running_sum_test1() {
        assert_eq!(running_sum(vec![0; 0]), vec![0; 0]);
    }

    #[test]
    fn running_sum_test2() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn running_sum_test3() {
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn running_sum_test4() {
        assert_eq!(running_sum(vec![1]), vec![1]);
    }
}