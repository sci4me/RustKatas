/// This function finds the closest integer to zero in a list.
/// If two different numbers tie for distance from zero, the positive one is returned.
/// If the list is empty, None is returned.
///
/// # Examples
/// ```
/// use closest::*;
/// assert_eq!(to_zero(&vec![5, -2, 8, 4, 7, 2]), Some(2));
/// ```
pub fn to_zero(list: &Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }

    let mut closest : i32 = std::i32::MIN;
    let mut closest_dist : i32 = std::i32::MAX;

    for nn in list {
        let n = *nn;
        let dist = n.abs();
        if dist <= closest_dist {
            if dist == closest_dist && n < 0 { continue; }
            closest = n;
            closest_dist = dist;
        }
    }

    Some(closest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produces_one_value() {
        let test = vec![-10];
        let result = to_zero(&test);
        assert_eq!(result, Some(-10));
    }

    #[test]
    fn test_produces_smallest_positive() {
        let test = vec![5, 2, 8, 4, 7, 1];
        let result = to_zero(&test);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_produces_no_result() {
        let test = vec![];
        let result = to_zero(&test);
        assert_eq!(result, None);
    }

    #[test]
    fn test_produces_greatest_negative() {
        let test = vec![-5, -2, -8, -4, -7, -1];
        let result = to_zero(&test);
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_produces_negative() {
        let test = vec![5, 2, 8, 4, 7, -1];
        let result = to_zero(&test);
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_produces_positive() {
        let test = vec![-5, -2, -8, -4, -7, 1];
        let result = to_zero(&test);
        assert_eq!(result, Some(1));
    }    

    #[test]
    fn test_favor_positive() {
        let test1 = vec![5, -2, 8, 4, 7, 2];
        let test2 = vec![5, 2, 8, 4, 7, -2];
        let result1 = to_zero(&test1);
        let result2 = to_zero(&test2);
        assert_eq!(result1, Some(2));
        assert_eq!(result2, Some(2));
    }
}