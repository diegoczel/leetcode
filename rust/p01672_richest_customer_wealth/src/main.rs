fn main() {
    println!("Hello, world!");
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    if accounts.len() == 0 || accounts[0].len() == 0 {
        return 0;
    }

    let mut r = 0;

    for x in 0..accounts.len() {
        let mut s = 0;
        for y in 0..accounts[x].len() {
            s = s + accounts[x][y];
        }

        if s > r {
            r = s;
        }
    }

   r
}

#[cfg(test)]
mod tests {
    use crate::maximum_wealth;

    #[test]
    fn maximum_wealth_test1() {
        assert_eq!(maximum_wealth(vec![]), 0);
        assert_eq!(maximum_wealth(vec![vec![]]), 0);
    }

    #[test]
    fn maximum_wealth_test2() {
        assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    }

    #[test]
    fn maximum_wealth_test3() {
        assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
    }

    #[test]
    fn maximum_wealth_test4() {
        assert_eq!(maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3]]), 17);
    }
}
