fn max_area(height: Vec<i32>) -> i32 {
    let mut area = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    while i < j {
        area = area.max((j - i) as i32 * height[i].min(height[j]));
        if height[i] < height[j] {
            i += 1
        } else {
            j -= 1
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}