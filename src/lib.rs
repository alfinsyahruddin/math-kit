
#[cfg(feature="add")]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(feature="subtract")]
pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(feature="multiply")]
pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(feature="divide")]
pub fn divide(left: usize, right: usize) -> usize {
    left / right
}

#[cfg(feature="discount")]
pub fn discount(total: f64, discount: f64) -> f64 {
    total - (total * (discount / 100.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_discount() {
        let result = discount(100_000.0, 25.0);
        assert_eq!(result, 75_000.0);
    }
}
