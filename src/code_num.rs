// - short_number: A number with unadjusted digits (e.g., 678954)
// - large_number: A number adjusted to D digits (e.g., 67895400000, 67895432121)
// - code_number: A number adjusted to D digits with digits defaulting to the binary representation of E (e.g., 67895400000 -> 67895400111 (D=11, E=7), 67895432121 -> 67895432121)
// - short_array: A left-aligned array corresponding to short_number (e.g., [6, 7, 8, 9, 5, 4])
// - large_array: A left-aligned array corresponding to large_number (e.g., [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0])
// - code_array: A left-aligned array corresponding to code_number
//
// - encode: Converts a short_number or large_number to a code_number
// - truncate: Truncates a large_number or code_number to a short_number

/// Represents a mesh code number.
/// - D: The number of digits in the code.
/// - E: The default value of the right-aligned digits (represented in binary).
///
/// For example:
/// - CodeNum<11, 0> : CodeNum::from_number(678954, 6) -> CodeNum(67895400000)
/// - CodeNum<11, 5> : CodeNum::from_number(678954, 6) -> CodeNum(67895400101)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeNum<const D: usize, const E: u8>(u64);

impl<const D: usize, const E: u8> Default for CodeNum<D, E> {
    fn default() -> Self {
        CodeNum(0)
    }
}

impl<const D: usize, const E: u8> CodeNum<D, E> {
    /// Creates a new CodeNum instance from an array.
    pub fn new(array: &[u8]) -> Self {
        let large_array = short_array_to_large_array::<D>(array);
        CodeNum(encode::<D, E>(large_array))
    }

    /// Creates a new CodeNum instance from a number.
    pub fn from_number(short_number: u64) -> Self {
        let large_array = short_number_to_large_array::<D, E>(short_number);
        CodeNum(encode::<D, E>(large_array))
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
fn short_number_to_large_array<const D: usize, const E: u8>(short_number: u64) -> [u8; D] {
    let mut large_array = [0u8; D];
    let mut number = short_number;
    while number < 10u64.pow((D - 1) as u32) {
        number *= 10;
    }

    for i in (0..D).rev() {
        large_array[i] = (number % 10) as u8;
        number /= 10;
    }
    large_array
}

/// 67895400000 -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] (when D=11)
fn code_number_to_code_array<const D: usize, const E: u8>(code_number: u64) -> [u8; D] {
    let mut code_array = [0u8; D];
    let mut number = code_number;

    for i in (0..D).rev() {
        code_array[i] = (number % 10) as u8;
        number /= 10;
    }
    code_array
}

/// [6, 7, 8, 9, 5, 4] -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] (when D=11)
fn short_array_to_large_array<const D: usize>(short_array: &[u8]) -> [u8; D] {
    let mut large_array = [0u8; D];
    large_array[..short_array.len()].copy_from_slice(short_array);
    large_array
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 1] (when E=1, D=11)
/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 1, 0] (when E=2, D=11)
/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 1, 1] (when E=3, D=11)
/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 1, 0, 1] (when E=5, D=11)
/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] (when E=7, D=11)
fn large_array_to_code_array<const D: usize, const E: u8>(large_array: [u8; D]) -> [u8; D] {
    let mut code_array = large_array;
    let mut e_value = E;
    let mut bit_position = 0;

    while e_value > 0 {
        if e_value & 1 == 1 {
            let index = D - 1 - bit_position;
            if index < D && code_array[index] == 0 {
                code_array[index] = 1;
            }
        }
        e_value >>= 1;
        bit_position += 1;
    }

    code_array
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] -> 67895400111 (when D=11)
fn code_array_to_code_number<const D: usize>(code_array: [u8; D]) -> u64 {
    code_array.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]  -> 67895400111 (when D=11, E=7)
fn encode<const D: usize, const E: u8>(large_array: [u8; D]) -> u64 {
    let code_array = large_array_to_code_array::<D, E>(large_array);
    code_array_to_code_number(code_array)
}

/// 67895432124 -> 6789 (code_length = 4), 6789543212 (code_length = 10) (when D=11)
fn truncate<const D: usize>(large_number: u64, code_length: usize) -> u64 {
    large_number / 10u64.pow(D as u32 - code_length as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_number_to_large_array() {
        assert_eq!(
            short_number_to_large_array::<11, 3>(678954),
            [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            short_number_to_large_array::<11, 3>(67895432124),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_large_array_to_code_array() {
        // E=7 (binary: 111)
        assert_eq!(
            large_array_to_code_array::<11, 7>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]
        );
        // E=2 (binary: 10)
        assert_eq!(
            large_array_to_code_array::<11, 2>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            [6, 7, 8, 9, 5, 4, 0, 0, 0, 1, 0]
        );
        // E=5 (binary: 101)
        assert_eq!(
            large_array_to_code_array::<11, 5>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            [6, 7, 8, 9, 5, 4, 0, 0, 1, 0, 1]
        );
        // Non-zero values should remain unchanged
        assert_eq!(
            large_array_to_code_array::<11, 7>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
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
            code_array_to_code_number::<11>([6, 7, 8, 9, 5, 4, 0, 0, 0, 1, 0]),
            67895400010
        );
        assert_eq!(
            code_array_to_code_number::<11>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(
            encode::<11, 7>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            67895400111
        );
        assert_eq!(
            encode::<11, 5>([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            67895400101
        );
        assert_eq!(
            encode::<11, 7>([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate::<11>(67895432124, 4), 6789);
        assert_eq!(truncate::<11>(67895432124, 10), 6789543212);
    }
}
