//! - short_number: A number with unadjusted digits (e.g., 678954)
//! - large_number: A number adjusted to 11 digits (e.g., 67895400000, 67895432121)
//! - code_number: A number adjusted to 11 digits with the lower D digits defaulting to 1 (e.g., 67895400111, 67895432121)
//! - raw_array: A left-aligned array corresponding to large_number (e.g., [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0])
//! - code_array: A left-aligned array corresponding to code_number

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code11<const D: usize>(u64);

impl<const D: usize> Code11<D> {
    /// Creates a new Code11 instance from an 11-digit array.
    pub fn new(array: [u8; 11], code_length: usize) -> Self {
        let large_number = raw_array_to_large_number(array);
        Code11(truncate_and_encode::<D>(large_number, code_length))
    }

    /// Creates a new Code11 instance from a number.
    pub fn from_number(short_number: u64, code_length: usize) -> Self {
        let raw_array = short_number_to_raw_array(short_number);
        Self::new(raw_array, code_length)
    }

    /// Converts a Code11 instance to an 11-digit array.
    pub fn to_array(self) -> [u8; 11] {
        code_number_to_code_array(self.0)
    }

    /// Converts a Code11 instance to a number.
    pub fn to_number(self, code_length: usize) -> u64 {
        self.0 / 10u64.pow(11 - code_length as u32)
    }
}

/// 678954 -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
fn short_number_to_raw_array(short_number: u64) -> [u8; 11] {
    let mut raw_array = [0u8; 11];
    let mut number = short_number;
    while number < 10u64.pow(10) {
        number *= 10;
    }

    for i in (0..11).rev() {
        raw_array[i] = (number % 10) as u8;
        number /= 10;
    }
    raw_array
}

/// 67895400000 -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
fn code_number_to_code_array(code_number: u64) -> [u8; 11] {
    let mut code_array = [0u8; 11];
    let mut number = code_number;

    for i in (0..11).rev() {
        code_array[i] = (number % 10) as u8;
        number /= 10;
    }
    code_array
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> 67895400000
fn raw_array_to_large_number(raw_array: [u8; 11]) -> u64 {
    raw_array.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] (when D=3)
fn raw_array_to_code_array<const D: usize>(raw_array: [u8; 11]) -> [u8; 11] {
    let mut code_array = raw_array;
    for code in code_array.iter_mut().skip(11 - D) {
        *code = if *code == 0 { 1 } else { *code };
    }
    code_array
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] -> 67895400111
fn code_array_to_code_number(code_array: [u8; 11]) -> u64 {
    code_array.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// 678954 -> 67895400111 (when D=3)
fn encode<const D: usize>(short_number: u64) -> u64 {
    let raw_array = short_number_to_raw_array(short_number);
    let code_array = raw_array_to_code_array::<D>(raw_array);
    code_array_to_code_number(code_array)
}

/// 67895432124 -> 6789 (code_length = 4), 6789543212 (code_length = 10)
fn truncate(large_number: u64, code_length: usize) -> u64 {
    large_number / 10u64.pow(11 - code_length as u32)
}

/// 67895432124 -> 67890000111 (code_length = 4), 67895432121 (code_length = 10) (when D=3)
fn truncate_and_encode<const D: usize>(large_number: u64, code_length: usize) -> u64 {
    encode::<D>(truncate(large_number, code_length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_number_to_raw_array() {
        assert_eq!(
            short_number_to_raw_array(678954),
            [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            short_number_to_raw_array(67895432124),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_raw_array_to_large_number() {
        assert_eq!(
            raw_array_to_large_number([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            67895400000
        );
        assert_eq!(
            raw_array_to_large_number([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_raw_array_to_code_array() {
        assert_eq!(
            raw_array_to_code_array::<3>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]
        );
        assert_eq!(
            raw_array_to_code_array::<3>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_code_array_to_code_number() {
        assert_eq!(
            code_array_to_code_number([6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]),
            67895400111
        );
        assert_eq!(
            code_array_to_code_number([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode::<3>(678954), 67895400111);
        assert_eq!(encode::<3>(67895432100), 67895432111);
        assert_eq!(encode::<3>(67895432124), 67895432124);
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate(67895432124, 4), 6789);
        assert_eq!(truncate(67895432124, 10), 6789543212);
    }

    #[test]
    fn test_truncate_and_encode() {
        assert_eq!(truncate_and_encode::<3>(67895432124, 4), 67890000111);
        assert_eq!(truncate_and_encode::<3>(67895432124, 10), 67895432121);
    }
}
