/// This function finds the closest integer to zero in a list.
/// If two different numbers tie for distance from zero, the positive one is returned.
/// If the list is empty, None is returned.
///
/// # Examples
/// ```
/// assert_eq!(closest::to_zero(vec![5, -2, 8, 4, 7, 2]), Some(2));
/// ```
pub fn to_zero(list: Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }

    let mut closest : i32 = std::i32::MIN;
    let mut closest_dist : i32 = std::i32::MAX;

    for n in list {
        let dist = n.abs();
        if dist <= closest_dist {
            if dist == closest_dist && n < 0 { continue; }
            closest = n;
            closest_dist = dist;
        }
    }

    return Some(closest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(to_zero(vec![-10]), Some(-10));
        assert_eq!(to_zero(vec![5, 2, 8, 4, 7, 1]), Some(1));
        assert_eq!(to_zero(vec![]), None);
        assert_eq!(to_zero(vec![-5, -2, -8, -4, -7, -1]), Some(-1));
        assert_eq!(to_zero(vec![5, 2, 8, 4, 7, -1]), Some(-1));
        assert_eq!(to_zero(vec![-5, -2, -8, -4, -7, 1]), Some(1));
        assert_eq!(to_zero(vec![5, -2, 8, 4, 7, 2]), Some(2));
        assert_eq!(to_zero(vec![5, 2, 8, 4, 7, -2]), Some(2));
    }
}