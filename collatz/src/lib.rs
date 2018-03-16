extern crate rayon;

use rayon::prelude::*;

/// This function returns the next number in the collatz sequence.
///
/// # Examples
/// ```
/// use collatz::*;
/// assert_eq!(collatz(2), 1);
/// assert_eq!(collatz(3), 10);
/// ```
pub fn collatz(x: u64) -> u64 {
    if x % 2 == 0 {
        return x / 2;
    }
    3 * x + 1
}

/// This function counts the number of steps it takes for a number to reach 1 by following the collatz sequence.
///
/// # Examples
/// ```
/// use collatz::*;
/// assert_eq!(steps(5), 5); // 5 -> 16 -> 8 -> 4 -> 2 -> 1
/// ```
pub fn steps(x: u64) -> u64 {
    if x == 0 {
        panic!("x must be > 0");
    }

    let mut count : u64 = 0;
    let mut n = x;

    while n != 1 {
        n = collatz(n);
        count += 1;
    }

    count
}

/// This function calculates the sum of each invocation of the 'steps' function in a range of positive integers.
/// 
/// # Examples
/// ```
/// use collatz::*;
/// assert_eq!(steps_sum(1, 7), 39);
/// ```
pub fn steps_sum(start: u64, end: u64) -> u64 {
    assert!(start > 0, "start must be > 0");
    assert!(end > start, "end must be > start");

    (start..end+1).map(steps).sum()
}

/// This function calculates the sum of each invocation of the 'steps' function in a range of positive integers.
/// This function is equivalent to 'steps_sum' except that it executes in parallel using the rayon library.
///
/// # Examples
/// ```
/// use collatz::*;
/// assert_eq!(steps_sum(1, 7), 39);
/// ```
pub fn par_steps_sum(start: u64, end: u64) -> u64 {
    assert!(start > 0, "start must be > 0");
    assert!(end > start, "end must be > start");

    (start..end+1).into_par_iter().map(steps).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(1), 4);      // 3 * 1 + 1 = 4
        assert_eq!(collatz(2), 1);      // 2 / 2     = 1
        assert_eq!(collatz(3), 10);     // 3 * 3 + 1 = 10
        assert_eq!(collatz(4), 2);      // 4 / 2     = 2
        assert_eq!(collatz(5), 16);     // 5 * 3 + 1 = 16
    }

    #[test]
    #[should_panic]
    fn test_steps_doesnt_accept_zero() {
        steps(0);
    }

    #[test]
    fn test_steps() {
        assert_eq!(steps(1), 0);        // 1
        assert_eq!(steps(2), 1);        // 2 -> 1
        assert_eq!(steps(3), 7);        // 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
        assert_eq!(steps(4), 2);        // 4 -> 2 -> 1
        assert_eq!(steps(5), 5);        // 5 -> 16 -> 8 -> 4 -> 2 -> 1
        assert_eq!(steps(6), 8);        // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
        assert_eq!(steps(7), 16);       // 7 -> 22 -> 11 -> 34 -> 17 -> 52 -> 26 -> 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    }

    #[test]
    #[should_panic]
    fn test_steps_sum_rejects_zero() {
        steps_sum(0, 10);
    }

    #[test]
    #[should_panic]
    fn test_steps_sum_rejects_end_less_than_start() {
        steps_sum(10, 5);
    }

    #[test]
    fn test_steps_sum() {
        assert_eq!(steps_sum(1, 7), 39); // 39 is the sum of steps(1)..steps(7) taken from test_steps
    }

    #[test]
    #[should_panic]
    fn test_par_steps_sum_rejects_end_less_than_start() {
        par_steps_sum(10, 5);
    }

    #[test]
    fn test_par_steps_sum() {
        assert_eq!(par_steps_sum(1, 7), 39); // 39 is the sum of steps(1)..steps(7) taken from test_steps
    }
}