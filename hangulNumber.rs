use std::collections::HashMap;
use std::io::{self, Write};

/// Selected 128 readable Hangul syllables (optimized for natural Korean pronunciation)
const CHARS: [&str; 128] = [
    // ㄱ (9): 받침 없거나 부드러운 받침
    "가", "간", "강", "개", "거", "고", "공", "구", "금",
    // ㄴ (9): 자연스러운 발음
    "나", "날", "남", "내", "너", "노", "눈", "늘", "니",
    // ㄷ (9): 일상 단어 느낌
    "다", "달", "담", "대", "더", "도", "동", "두", "드",
    // ㄹ (9): 부드러운 발음
    "라", "람", "랑", "래", "러", "로", "루", "리", "림",
    // ㅁ (9): 자연스러운 조합
    "마", "만", "말", "매", "머", "모", "무", "문", "미",
    // ㅂ (9): 일상적인 글자
    "바", "반", "방", "배", "보", "봄", "부", "비", "빈",
    // ㅅ (10): 많이 쓰이는 글자
    "사", "산", "상", "새", "서", "선", "소", "송", "수", "시",
    // ㅇ (10): 모음 시작 글자
    "아", "안", "양", "어", "연", "영", "오", "온", "우", "이",
    // ㅈ (9): 자연스러운 발음
    "자", "잔", "장", "재", "저", "조", "주", "중", "지",
    // ㅊ (9): 깔끔한 발음
    "차", "찬", "창", "채", "천", "초", "춘", "충", "치",
    // ㅋ (9): 외래어 느낌 최소화
    "카", "칸", "코", "쿠", "크", "키", "캐", "케", "콩",
    // ㅌ (9): 부드러운 발음
    "타", "탄", "태", "터", "토", "통", "투", "트", "티",
    // ㅍ (9): 자연스러운 글자
    "파", "판", "패", "포", "풍", "프", "피", "팔", "품",
    // ㅎ (9): 일상적인 글자
    "하", "한", "해", "허", "호", "홍", "화", "후", "히",
];

/// Hangul Number Converter (Base-128, Variable Length)
pub struct HangulNumberConverter {
    reverse_map: HashMap<String, usize>,
}

impl HangulNumberConverter {
    /// Creates a new HangulNumberConverter instance
    pub fn new() -> Self {
        let mut reverse_map = HashMap::new();
        for (i, &ch) in CHARS.iter().enumerate() {
            reverse_map.insert(ch.to_string(), i);
        }
        
        if CHARS.len() != 128 {
            eprintln!("Warning: CHARS size is {}, expected 128.", CHARS.len());
        }
        
        HangulNumberConverter { reverse_map }
    }

    /// Encodes a non-negative integer into a variable-length Hangul string with a specific seed.
    /// 
    /// # Arguments
    /// * `num` - The number to encode (must be >= 0)
    /// * `seed` - The seed to use for scrambling (0~127)
    /// 
    /// # Returns
    /// The encoded Hangul string (seed + scrambled data)
    pub fn encode_with_seed(&self, num: u64, seed: usize) -> Result<String, String> {
        if seed >= 128 {
            return Err("Seed must be between 0 and 127".to_string());
        }

        let seed_char = CHARS[seed];

        // Special case for 0
        if num == 0 {
            let scrambled = (0 + seed) % 128;
            return Ok(format!("{}{}", seed_char, CHARS[scrambled]));
        }

        // Convert to base-128 digits (least significant first)
        let mut digits: Vec<usize> = Vec::new();
        let mut temp = num;
        while temp > 0 {
            digits.push((temp % 128) as usize);
            temp /= 128;
        }

        // Reverse to get most significant first
        digits.reverse();

        // Scramble each digit with seed and map to Hangul
        let mut result = String::from(seed_char);
        for d in digits {
            let scrambled = (d + seed) % 128;
            result.push_str(CHARS[scrambled]);
        }

        Ok(result)
    }

    /// Encodes a non-negative integer into a variable-length Hangul string.
    /// The first character is a random seed that scrambles the remaining characters.
    /// Same number can produce different encodings, but all decode to the same value.
    /// 
    /// # Arguments
    /// * `num` - The number to encode (must be >= 0)
    /// 
    /// # Returns
    /// The encoded Hangul string (seed + scrambled data)
    pub fn encode(&self, num: u64) -> Result<String, String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        // Simple random seed using current time
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as usize % 128;
        
        self.encode_with_seed(num, seed)
    }

    /// Returns all 128 possible encodings for a number.
    /// 
    /// # Arguments
    /// * `num` - The number to encode
    /// 
    /// # Returns
    /// Vector of all 128 encoded strings
    pub fn encode_all(&self, num: u64) -> Result<Vec<String>, String> {
        let mut results = Vec::with_capacity(128);
        for seed in 0..128 {
            results.push(self.encode_with_seed(num, seed)?);
        }
        Ok(results)
    }

    /// Decodes a Hangul string back to a number.
    /// 
    /// # Arguments
    /// * `s` - The Hangul string to decode (first char is seed)
    /// 
    /// # Returns
    /// The decoded number
    pub fn decode(&self, s: &str) -> Result<u64, String> {
        // Get grapheme clusters (each Hangul character is one grapheme)
        let chars: Vec<&str> = s.graphemes(true).collect();
        
        if chars.len() < 2 {
            return Err("Invalid string: must be at least 2 characters".to_string());
        }

        // First character is the seed
        let seed_char = chars[0];
        let seed = self.reverse_map
            .get(seed_char)
            .ok_or_else(|| format!("Invalid seed character: {}", seed_char))?;

        // Decode remaining characters
        let mut num: u64 = 0;
        for &c in &chars[1..] {
            let scrambled = self.reverse_map
                .get(c)
                .ok_or_else(|| format!("Invalid character: {}", c))?;
            // Unscramble: original = (scrambled - seed + 128) % 128
            let original = (*scrambled + 128 - seed) % 128;
            num = num * 128 + original as u64;
        }

        Ok(num)
    }
}

impl Default for HangulNumberConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for grapheme iteration (simplified version)
trait Graphemes {
    fn graphemes(&self, extended: bool) -> GraphemeIterator;
}

impl Graphemes for str {
    fn graphemes(&self, _extended: bool) -> GraphemeIterator {
        GraphemeIterator { s: self, pos: 0 }
    }
}

struct GraphemeIterator<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Iterator for GraphemeIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.s.len() {
            return None;
        }

        let start = self.pos;
        let mut chars = self.s[start..].chars();
        
        if let Some(c) = chars.next() {
            self.pos += c.len_utf8();
            Some(&self.s[start..self.pos])
        } else {
            None
        }
    }
}

fn format_number_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }
    result
}

fn main() {
    let converter = HangulNumberConverter::new();

    println!("=== Hangul Number Converter (Base-128, Variable Length) ===");
    println!("Enter a non-negative integer to encode.");
    println!("Type 'exit' to quit.\n");

    loop {
        print!("Enter number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let answer = input.trim();

        if answer.is_empty() || answer.to_lowercase() == "exit" {
            break;
        }

        // Remove commas and parse
        let cleaned_answer = answer.replace(',', "");
        let num: u64 = match cleaned_answer.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.\n");
                continue;
            }
        };

        println!("\nAll 128 encodings for {}:", format_number_with_commas(num));
        println!("--------------------------------------------------");

        match converter.encode_all(num) {
            Ok(all_encodings) => {
                // Print in rows of 8 for readability
                for row in 0..16 {
                    let mut items: Vec<String> = Vec::new();
                    for col in 0..8 {
                        let idx = row * 8 + col;
                        let encoded = &all_encodings[idx];
                        let decoded = converter.decode(encoded).unwrap_or(0);
                        let check = if decoded == num { "✓" } else { "✗" };
                        items.push(format!("{}{}", encoded, check));
                    }
                    println!("{}", items.join("  "));
                }

                println!(
                    "\nTotal: 128 variants, Length: {} chars each",
                    all_encodings[0].chars().count()
                );
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        println!("--------------------------------------------------\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_zero() {
        let converter = HangulNumberConverter::new();
        for seed in 0..128 {
            let encoded = converter.encode_with_seed(0, seed).unwrap();
            let decoded = converter.decode(&encoded).unwrap();
            assert_eq!(decoded, 0);
        }
    }

    #[test]
    fn test_encode_decode_various() {
        let converter = HangulNumberConverter::new();
        let test_values: Vec<u64> = vec![1, 127, 128, 255, 1000, 16384, 1000000, u64::MAX / 2];
        
        for num in test_values {
            for seed in 0..128 {
                let encoded = converter.encode_with_seed(num, seed).unwrap();
                let decoded = converter.decode(&encoded).unwrap();
                assert_eq!(decoded, num, "Failed for num={}, seed={}", num, seed);
            }
        }
    }

    #[test]
    fn test_all_128_encodings() {
        let converter = HangulNumberConverter::new();
        let all = converter.encode_all(12345).unwrap();
        assert_eq!(all.len(), 128);
        
        for encoded in &all {
            let decoded = converter.decode(encoded).unwrap();
            assert_eq!(decoded, 12345);
        }
    }

    #[test]
    fn test_invalid_seed() {
        let converter = HangulNumberConverter::new();
        assert!(converter.encode_with_seed(100, 128).is_err());
        assert!(converter.encode_with_seed(100, 200).is_err());
    }

    #[test]
    fn test_invalid_decode() {
        let converter = HangulNumberConverter::new();
        assert!(converter.decode("가").is_err()); // Too short
        assert!(converter.decode("").is_err()); // Empty
    }
}
