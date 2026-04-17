use rand::seq::SliceRandom;
use rand::thread_rng;

// [0-9][a-z][A-Z] shuffled - must match Java implementation exactly
const ELEMENTS: &[char] = &[
    'V', 'd', 'q', 'N', 'Z', 'h', '1', 'S', 'm', 'H', 'U', 'L', '2', 'J', '5', 's', 'l', 'a', '6', 'P', 'e',
    '3', 'o', 'T', 'C', 'i', 'b', 'u', 'x', 'Y', 'E', 'K', 'R', 'B', 'Q', 'z', '4', 'k', 'r', 't', 'g', '8',
    'W', 'I', 'O', 'F', 'c', 'p', 'D', 'X', 'f', 'v', '7', '9', 'j', '0', 'w', 'M', 'A', 'y', 'n', 'G'
];

const ELEMENTS_LEN: usize = 62;

pub struct ShortUrlUtil {
    length: usize,
    max_num: u64,
}

impl ShortUrlUtil {
    pub fn new(length: usize) -> Self {
        if length == 0 {
            panic!("Length must be > 0");
        }
        let max_num = (ELEMENTS_LEN as u64).pow(length as u32) - 1;
        Self { length, max_num }
    }

    /// Convert base62 string to base10 number
    pub fn base62_to_base10(&self, s: &str) -> Result<u64, String> {
        let mut n: u64 = 0;
        for ch in s.chars() {
            let index = ELEMENTS.iter().position(|&c| c == ch);
            match index {
                Some(idx) => {
                    n = n * ELEMENTS_LEN as u64 + idx as u64;
                }
                None => return Err(format!("Unknown char: {}", ch)),
            }
        }
        Ok(n)
    }

    /// Convert base10 number to base62 string
    pub fn base10_to_base62(&self, n: u64) -> Result<String, String> {
        if n > self.max_num {
            return Err(format!("Max number is: {}", self.max_num));
        }

        let mut n = n;
        let mut chars: Vec<char> = Vec::new();

        while n != 0 {
            chars.push(ELEMENTS[(n % ELEMENTS_LEN as u64) as usize]);
            n /= ELEMENTS_LEN as u64;
        }

        chars.reverse();

        // Pad with first element to reach desired length
        let first = ELEMENTS[0];
        let result: String = std::iter::repeat(first)
            .take(self.length.saturating_sub(chars.len()))
            .chain(chars.into_iter())
            .collect();

        Ok(result)
    }

    /// Generate random short URI
    pub fn random_str(&self) -> String {
        let mut rng = thread_rng();
        let mut elements: Vec<char> = ELEMENTS.to_vec();
        elements.shuffle(&mut rng);

        elements.into_iter()
            .take(self.length)
            .collect()
    }

    pub fn max_num(&self) -> u64 {
        self.max_num
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base62_roundtrip() {
        let util = ShortUrlUtil::new(4);
        for i in 0..100 {
            let encoded = util.base10_to_base62(i).unwrap();
            let decoded = util.base62_to_base10(&encoded).unwrap();
            assert_eq!(i, decoded);
        }
    }

    #[test]
    fn test_random_str_length() {
        for len in 1..=6 {
            let util = ShortUrlUtil::new(len);
            let s = util.random_str();
            assert_eq!(s.len(), len);
        }
    }
}
