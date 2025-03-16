//! - short_number: A number with unadjusted digits (e.g., 678954)
//! - large_number: A number adjusted to D digits (e.g., 67895400000, 67895432121)
//! - code_number: A number adjusted to D digits with the lower E digits defaulting to 1 (e.g., 67895400111, 67895432121)
//! - raw_array: A left-aligned array corresponding to large_number (e.g., [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0])
//! - code_array: A left-aligned array corresponding to code_number

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeNum<const D: usize, const E: usize>(u64);

impl<const D: usize, const E: usize> CodeNum<D, E> {
    /// Creates a new CodeNum instance from an D-digit array.
    pub fn new(array: [u8; D], code_length: usize) -> Self {
        let large_number = raw_array_to_large_number::<D, E>(array);
        CodeNum(truncate_and_encode::<D, E>(large_number, code_length))
    }

    /// Creates a new CodeNum instance from a number.
    pub fn from_number(short_number: u64, code_length: usize) -> Self {
        let raw_array = short_number_to_raw_array::<D, E>(short_number);
        Self::new(raw_array, code_length)
    }

    /// Converts a CodeNum instance to an D-digit array.
    pub fn to_array(self) -> [u8; D] {
        code_number_to_code_array::<D, E>(self.0)
    }

    /// Converts a CodeNum instance to a number.
    pub fn to_number(self, code_length: usize) -> u64 {
        truncate::<D>(self.0, code_length)
    }
}

/// 678954 -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] (when D=11)
fn short_number_to_raw_array<const D: usize, const E: usize>(short_number: u64) -> [u8; D] {
    let mut raw_array = [0u8; D];
    let mut number = short_number;
    while number < 10u64.pow((D - 1) as u32) {
        number *= 10;
    }

    for i in (0..D).rev() {
        raw_array[i] = (number % 10) as u8;
        number /= 10;
    }
    raw_array
}

/// 67895400000 -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] (when D=11)
fn code_number_to_code_array<const D: usize, const E: usize>(code_number: u64) -> [u8; D] {
    let mut code_array = [0u8; D];
    let mut number = code_number;

    for i in (0..D).rev() {
        code_array[i] = (number % 10) as u8;
        number /= 10;
    }
    code_array
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> 67895400000 (when D=11)
fn raw_array_to_large_number<const D: usize, const E: usize>(raw_array: [u8; D]) -> u64 {
    raw_array.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] (when E=3, D=11)
fn raw_array_to_code_array<const D: usize, const E: usize>(raw_array: [u8; D]) -> [u8; D] {
    let mut code_array = raw_array;
    for code in code_array.iter_mut().skip(D - E) {
        *code = if *code == 0 { 1 } else { *code };
    }
    code_array
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] -> 67895400111 (when D=11)
fn code_array_to_code_number<const D: usize>(code_array: [u8; D]) -> u64 {
    code_array.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// 678954 -> 67895400111 (when E=3, D=11)
fn encode<const D: usize, const E: usize>(short_number: u64) -> u64 {
    let raw_array = short_number_to_raw_array::<D, E>(short_number);
    let code_array = raw_array_to_code_array::<D, E>(raw_array);
    code_array_to_code_number(code_array)
}

/// 67895432124 -> 6789 (code_length = 4), 6789543212 (code_length = 10) (when D=11)
fn truncate<const D: usize>(large_number: u64, code_length: usize) -> u64 {
    large_number / 10u64.pow(D as u32 - code_length as u32)
}

/// 67895432124 -> 67890000111 (code_length = 4), 67895432121 (code_length = 10) (when E=3, D=11)
fn truncate_and_encode<const D: usize, const E: usize>(
    large_number: u64,
    code_length: usize,
) -> u64 {
    encode::<D, E>(truncate::<D>(large_number, code_length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_number_to_raw_array() {
        assert_eq!(
            short_number_to_raw_array::<11, 3>(678954),
            [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            short_number_to_raw_array::<11, 3>(67895432124),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_raw_array_to_large_number() {
        assert_eq!(
            raw_array_to_large_number::<11, 3>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            67895400000
        );
        assert_eq!(
            raw_array_to_large_number::<11, 3>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_raw_array_to_code_array() {
        assert_eq!(
            raw_array_to_code_array::<11, 3>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]
        );
        assert_eq!(
            raw_array_to_code_array::<11, 3>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_code_array_to_code_number() {
        assert_eq!(
            code_array_to_code_number::<11>([6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]),
            67895400111
        );
        assert_eq!(
            code_array_to_code_number::<11>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode::<11, 3>(678954), 67895400111);
        assert_eq!(encode::<11, 3>(67895432100), 67895432111);
        assert_eq!(encode::<11, 3>(67895432124), 67895432124);
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate::<11>(67895432124, 4), 6789);
        assert_eq!(truncate::<11>(67895432124, 10), 6789543212);
    }

    #[test]
    fn test_truncate_and_encode() {
        assert_eq!(truncate_and_encode::<11, 3>(67895432124, 4), 67890000111);
        assert_eq!(truncate_and_encode::<11, 3>(67895432124, 10), 67895432121);
    }
}
