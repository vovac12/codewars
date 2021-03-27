use std::collections::HashMap;

static MORSE_PAIRS: [(&str, &str); 43] = [
    ("A", ".-"),
    ("B", "-..."),
    ("C", "-.-."),
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
    ("1", ".----"),
    ("2", "..---"),
    ("3", "...--"),
    ("4", "....-"),
    ("5", "....."),
    ("6", "-...."),
    ("7", "--..."),
    ("8", "---.."),
    ("9", "----."),
    ("0", "-----"),
    ("), ( ", "--..--"),
    (".", ".-.-.-"),
    ("?", "..--.."),
    ("/", "-..-."),
    ("-", "-....-"),
    ("(", "-.--."),
    (")", "-.--.-"),
];

struct MorseDecoder {
    morse_code: HashMap<&'static str, &'static str>,
}

impl MorseDecoder {
    fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: MORSE_PAIRS.iter().map(|(a, b)| (*b, *a)).collect(),
        }
    }
}

impl MorseDecoder {
    pub fn decode_bits(&self, encoded: &str) -> String {
        let mut curr = match encoded.chars().next() {
            Some(c) => c,
            None => return "".to_string(),
        };
        let unit = encoded
            .split('0')
            .filter_map(|x| if !x.is_empty() { Some(x.len()) } else { None })
            .min()
            .unwrap();
        let mut count = 0;
        let mut res = String::new();
        for c in encoded
            .chars()
            .skip_while(|&c| c == '0')
            .chain(Some('0').into_iter())
        {
            if curr != c {
                res += match curr {
                    '0' => match count / unit {
                        3 => " ",
                        7 => "   ",
                        _ => "",
                    },
                    '1' => match count / unit {
                        3 => "-",
                        _ => ".",
                    },
                    _ => "",
                };
                curr = c;
                count = 1;
            } else {
                count += 1;
            }
        }
        res
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .split("   ")
            .map(move |x| {
                x.split(' ')
                    .map(move |x| self.morse_code.get(x).unwrap().to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

fn main() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bits() {
        let decoder = MorseDecoder::new();
        assert_eq!(".... . -.--   .--- ..- -.. .".as_bytes(), decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011").as_bytes());
    }
    #[test]
    fn examples() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
    }
}
