use std::cmp::{min, max};

// TODO make the function generic over all signed integers
type INT = i32;

pub struct Result {
    gcd: INT,
    x: INT,
    y: INT
}
/// Run a resursive Extended Euclidian Algorithm
/// # Arguments
///
/// * `a` integer
/// * `b` integer
///
/// Returns a tuple:
///
/// * (gcd, x, y)
/// where
/// `gcd` is the greatest common divisor of `a` & `b`
/// `x` and `y` satisfy the equation: ax + by = gcd(a, b)
pub fn extended_euclidian_algorithm(a: INT, b: INT) -> Result {
    // The algorithm assumes that a > b.
    // We initialize r_0 = a, r_1 = b
    // We initialize s_0 = 1, s_1 = 0
    // We initialize t_0 = 1, t_1 = 0
    // For explanation see https://www.wikiwand.com/en/Extended_Euclidean_algorithm
    let mut r_0 = max(a, b);
    let mut s_0: INT = 1;
    let mut t_0: INT = 0;

    let mut r_1 = min(a,b);
    let mut s_1: INT = 0;
    let mut t_1: INT = 1;

    let mut q: INT;
    let mut r_k: INT;
    let mut s_k: INT;
    let mut t_k: INT;

    // We iterate as long as the remainder is non-zero
    loop {
        // find the remainder
        r_k = r_0 % r_1;

        if r_k == 0 {
            // If the remainder is 0, return the previous terms
            return Result{ gcd: r_1, x: s_1, y: t_1 };
        }
        // Otherwise, we continue...

        // Find the quotient
        q = r_0 / r_1;

        // Find the auxiliary terms s, t for next iteration
        s_k = s_0 - q * s_1;
        t_k = t_0 - q * t_1;

        // redefine terms
        r_0 = r_1;
        s_0 = s_1;
        t_0 = t_1;

        r_1 = r_k;
        s_1 = s_k;
        t_1 = t_k;
    }
}

#[cfg(test)]
mod tests {
    use crate::extended_euclidian_algorithm;

    #[test]
    fn gcd_two_primes() {
        let result = extended_euclidian_algorithm(7, 11);
        assert_eq!(result.gcd, 1);
    }

    #[test]
    fn gcd_coprimes() {
        let result = extended_euclidian_algorithm(22, 15);
        assert_eq!(result.gcd, 1);
        assert_eq!(result.x, -2);
        assert_eq!(result.y, 3);
        // Check that indeed ax + by = gcd(a, b)
        assert_eq!(result.x*22 + result.y*15, result.gcd);
    }

    #[test]
    fn gcd_2_4() {
        let result = extended_euclidian_algorithm(2, 4);
        assert_eq!(result.gcd, 2);
    }

    #[test]
    fn gcd_4_2() {
        let result = extended_euclidian_algorithm(4, 2);
        assert_eq!(result.gcd, 2);
    }

    #[test]
    fn gcd_16_20() {
        let result = extended_euclidian_algorithm(16, 20);
        assert_eq!(result.gcd, 4);
    }

    #[test]
    fn gcd_240_46() {
        let result = extended_euclidian_algorithm(240, 46);
        assert_eq!(result.gcd, 2);
        assert_eq!(result.x, -9);
        assert_eq!(result.y, 47);
    }

    #[test]
    fn gcd_100_101() {
        let result = extended_euclidian_algorithm(100, 101);
        assert_eq!(result.gcd, 1);
    }

    #[test]
    fn gcd_100_1() {
        let result = extended_euclidian_algorithm(100, 1);
        assert_eq!(result.gcd, 1);
    }
}
