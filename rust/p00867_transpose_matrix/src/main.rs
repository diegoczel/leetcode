fn main() {
    println!("Hello, world!");
    transpose(vec![vec![1, 2], vec![3, 4]]);
    println!();
    transpose(vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]);
}

fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v = vec![vec![0; matrix.len()]; matrix[0].len()];
    
    if matrix.len() == 0 {
        return v;
    }

    if matrix.len() == 1 && matrix[0].len() == 1 {
        return matrix;
    }

    for lin in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            v[col][lin] = matrix[lin][col];
        }
    }

    v
}

#[cfg(test)]
pub mod tests {
    use crate::transpose;

    #[test]
    fn transpose_test1() {
        assert_eq!(
            transpose(vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]), 
            vec![vec![1,4,7], vec![2,5,8], vec![3,6,9]]);
    }

    #[test]
    fn transpose_test2() {
        assert_eq!(
            transpose(vec![vec![1,2], vec![3,4]]), 
            vec![vec![1,3], vec![2,4]]);
    }

    #[test]
    fn transpose_test3() {
        assert_eq!(
            transpose(vec![vec![1,2,3], vec![4,5,6]]), 
            vec![vec![1,4], vec![2,5], vec![3,6]]);
    }

    #[test]
    fn transpose_test4() {
        assert_eq!(
            transpose(vec![vec![7,2]]), 
            vec![vec![7], vec![2]]);
    }
}