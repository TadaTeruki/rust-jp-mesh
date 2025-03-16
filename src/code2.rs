#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code2(u64);

impl Code2 {
    pub fn new(array: [u8; 11], code_length: usize) -> Self {
        let number = raw_slice_to_number(array);
        Code2(truncate_and_format_number(number, code_length))
    }

    pub fn from_number(number: u64, code_length: usize) -> Self {
        let slice = number_to_raw_slice(number);
        Self::new(slice, code_length)
    }

    pub fn to_slice(&self) -> [u8; 11] {
        number_to_raw_slice(self.0)
    }

    pub fn to_number(&self, code_length: usize) -> u64 {
        self.0 / 10u64.pow(11 - code_length as u32)
    }
}

/// number to [u8; 11].
/// 678954 -> [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
fn number_to_raw_slice(number: u64) -> [u8; 11] {
    let mut code_2 = [0u8; 11];
    let mut number = number;
    while number < 10u64.pow(10) {
        number *= 10;
    }

    for i in (0..11).rev() {
        code_2[i] = (number % 10) as u8;
        number /= 10;
    }
    code_2
}

/// [u8; 11] to number.
/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> 67895400000
fn raw_slice_to_number(code_2: [u8; 11]) -> u64 {
    code_2.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// change 0 to 1 if the index is greater than or equal to 8.
/// [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0] -> [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]
fn raw_slice_to_formatted_slice(code_2: [u8; 11]) -> [u8; 11] {
    let mut code_2 = code_2;
    for code in code_2.iter_mut().skip(8) {
        *code = if *code == 0 { 1 } else { *code };
    }
    code_2
}

/// [u8; 11] to formatted number.
/// [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1] -> 67895400111
fn formatted_slice_to_formatted_number(code_2: [u8; 11]) -> u64 {
    code_2.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// format number.
/// 678954 -> 67895400111
fn format_number(number: u64) -> u64 {
    let raw = number_to_raw_slice(number);
    let code_2 = raw_slice_to_formatted_slice(raw);
    formatted_slice_to_formatted_number(code_2)
}

/// truncate number.
/// 67895432124 -> 6789 (code_length = 4), 6789543212 (code_length = 10)
fn truncate_number(number: u64, code_length: usize) -> u64 {
    number / 10u64.pow(11 - code_length as u32)
}

/// truncate and format number.
/// 67895432124 -> 67890000111 (code_length = 4), 67895432121 (code_length = 10)
fn truncate_and_format_number(number: u64, code_length: usize) -> u64 {
    format_number(truncate_number(number, code_length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_raw_slice() {
        assert_eq!(
            number_to_raw_slice(678954),
            [6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            number_to_raw_slice(67895432124),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_raw_slice_to_number() {
        assert_eq!(
            raw_slice_to_number([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            67895400000
        );
        assert_eq!(
            raw_slice_to_number([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_raw_slice_to_formatted_slice() {
        assert_eq!(
            raw_slice_to_formatted_slice([6, 7, 8, 9, 5, 4, 0, 0, 0, 0, 0]),
            [6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]
        );
        assert_eq!(
            raw_slice_to_formatted_slice([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            [6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]
        );
    }

    #[test]
    fn test_formatted_slice_to_formatted_number() {
        assert_eq!(
            formatted_slice_to_formatted_number([6, 7, 8, 9, 5, 4, 0, 0, 1, 1, 1]),
            67895400111
        );
        assert_eq!(
            formatted_slice_to_formatted_number([6, 7, 8, 9, 5, 4, 3, 2, 1, 2, 4]),
            67895432124
        );
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(678954), 67895400111);
        assert_eq!(format_number(67895432100), 67895432111);
        assert_eq!(format_number(67895432124), 67895432124);
    }

    #[test]
    fn test_truncate_number() {
        assert_eq!(truncate_number(67895432124, 4), 6789);
        assert_eq!(truncate_number(67895432124, 10), 6789543212);
    }

    #[test]
    fn test_truncate_and_format_number() {
        assert_eq!(truncate_and_format_number(67895432124, 4), 67890000111);
        assert_eq!(truncate_and_format_number(67895432124, 10), 67895432121);
    }
}
