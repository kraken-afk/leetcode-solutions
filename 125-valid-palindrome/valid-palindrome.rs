impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    let filtered_s = filter_alphabet(&s);
    let len: usize = filtered_s.len();
    let median: usize = if len & 1 == 0 { len / 2 } else { (len + 1) / 2 };
    let s_as_bytes = filtered_s.as_bytes();

    for i in 0..median {
        if s_as_bytes[i] != s_as_bytes[len - (i + 1)] {
            return false;
        }
    }
    return true;
    }
}

fn filter_alphabet(s: &str) -> String {
    let alphabet: Vec<u8> = (b'a'..=b'z').chain(b'0'..=b'9').collect();
    let mut result: Vec<u8> = Vec::new();

    for c in s.as_bytes() {
        let c_lowercase = c.to_ascii_lowercase();
        if alphabet.contains(&c_lowercase) {
            result.push(c_lowercase)
        }
    }

    let ster: Result<&str, std::str::Utf8Error> = std::str::from_utf8(&result);
    match ster {
        Err(e) => panic!("Someting went error: {}", e),
        Ok(s) => {
            return String::from(s);
        }
    }
}