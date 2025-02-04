fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];
    loop {
        let mut changed = false;
        for i in 0..n {
            if i > 0 && ratings[i] > ratings[i-1] && candies[i] <= candies[i-1] {
                candies[i] = candies[i-1] + 1;
                changed = true;
            }
            if i < n-1 && ratings[i] > ratings[i+1] && candies[i] <= candies[i+1] {
                candies[i] = candies[i+1] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    candies.iter().sum()
}

fn candy1(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];
    for i in 1..n {
        if ratings[i] > ratings[i-1] {
            candies[i] = candies[i-1] + 1;
        }
    }
    let mut result = candies[n-1];
    for i in (0..n-1).rev() {
        if ratings[i] > ratings[i+1] && candies[i] <= candies[i+1] {
            candies[i] = candies[i+1] + 1;
        }
        result += candies[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_candy() {
        assert_eq!(candy(vec![1,0,2]), 5);
        assert_eq!(candy(vec![1,2,2]), 4);
    }
}