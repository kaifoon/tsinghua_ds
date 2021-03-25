//! String Matching Algorithm

/// KMP str match algorithm
pub fn kmp_match(text: &str, pattern: &str) -> Option<usize> {
    let next = build_next(pattern);

    let (n, m) = (text.len() as i32, pattern.len() as i32);
    let (text, pattern) = (text.as_bytes(), pattern.as_bytes());
    let (mut i, mut j) = (0, 0);
    while j < m && i < n {
        if j < 0 || text[i as usize] == pattern[j as usize] {
            j += 1;
            i += 1;
        } else {
            j = next[j as usize];
        }
    }
    if (i - j) as usize == text.len() {
        None
    } else {
        Some((i - j) as usize)
    }
}

fn build_next(pattern: &str) -> Vec<i32> {
    let (m, pattern) = (pattern.len(), pattern.as_bytes());
    let mut next = vec![0; m];
    let (mut j, mut t) = (0, -1);
    next[0] = -1;

    while j < m - 1 {
        if t < 0 || pattern[j] == pattern[t as usize] {
            j += 1;
            t += 1;
            next[j] = t;
        } else {
            t = next[t as usize];
        }
    }
    next
}

/// BM Bad Char string match algorithm
pub fn bmbc_match(text: &str, pattern: &str) -> Option<usize> {
    let bc = build_bc(pattern);
    let (n, m) = (text.len(), pattern.len());
    let (text, pattern) = (text.as_bytes(), pattern.as_bytes());
    let (mut i, mut j) = (0, m - 1);

    while i + j < n {
        if text[i + j] == pattern[j] {
            if j == 0 {
                return Some(i);
            }
            j -= 1;
        } else {
            let prev = bc[text[i + j] as usize];
            // * all char match
            if prev == -1 {
                i = i + j + 1;
            } else if (prev as usize) < j {
                i += j - prev as usize;
            } else {
                i += 1;
            }
            j = m - 1;
        }
    }

    None
}

fn build_bc(pattern: &str) -> [i32; 256] {
    let mut bc = [-1; 256];
    let pattern = pattern.as_bytes();
    for j in 0..pattern.len() {
        bc[pattern[j] as usize] = j as i32;
    }
    bc
}

/// Karp-Rabin string match hashing algorithm
pub fn karp_rabin(text: &str, pattern: &str) -> Option<usize> {
    const BASE: usize = 65539;
    if pattern.len() == 0 {
        return Some(0);
    }

    let mut power = 1;
    let m = pattern.len();

    for _ in 0..m {
        power = (power * 31) % BASE;
    }

    let (text, pattern) = (text.as_bytes(), pattern.as_bytes());

    let mut hash_code = 0;
    for i in 0..m {
        hash_code = (hash_code * 31 + pattern[i] as usize) % BASE;
    }

    let mut source_code = 0;
    for i in 0..text.len() {
        source_code = (source_code * 31 + text[i] as usize) % BASE;
        if i < m - 1 {
            continue;
        }

        if i >= m {
            source_code = ((source_code as isize
                - (text[i - m] as usize * power) as isize % BASE as isize)
                + BASE as isize) as usize
                % BASE;
        }
        if source_code == hash_code {
            if &text[i + 1 - m..i + 1] == pattern {
                return Some(i + 1 - m);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kmp_match_test() {
        let text = "I learn Tsinghua DataStructure to improve my programming skill";
        let pattern = "Tsinghua DataStructure";
        assert_eq!(kmp_match(text, pattern), Some(8));
        let pattern = "I";
        assert_eq!(kmp_match(text, pattern), Some(0));
        let pattern = "l";
        assert_eq!(kmp_match(text, pattern), Some(2));
        let pattern = "a";
        assert_eq!(kmp_match(text, pattern), Some(4));
        let pattern = "skill";
        assert_eq!(kmp_match(text, pattern), Some(57));
        let pattern = "help";
        assert_eq!(kmp_match(text, pattern), None);
    }
    #[test]
    fn karprabin_match_test() {
        let text = "I learn Tsinghua DataStructure to improve my programming skill";
        let pattern = "Tsinghua DataStructure";
        assert_eq!(karp_rabin(text, pattern), Some(8));
        let pattern = "I";
        assert_eq!(karp_rabin(text, pattern), Some(0));
        let pattern = "l";
        assert_eq!(karp_rabin(text, pattern), Some(2));
        let pattern = "a";
        assert_eq!(karp_rabin(text, pattern), Some(4));
        let pattern = "skill";
        assert_eq!(karp_rabin(text, pattern), Some(57));
        let pattern = "help";
        assert_eq!(karp_rabin(text, pattern), None);
    }
    #[test]
    fn bmbc_match_test() {
        let text = "I learn Tsinghua DataStructure to improve my programming skill";
        let pattern = "Tsinghua DataStructure";
        assert_eq!(bmbc_match(text, pattern), Some(8));
        let pattern = "I";
        assert_eq!(bmbc_match(text, pattern), Some(0));
        let pattern = "l";
        assert_eq!(bmbc_match(text, pattern), Some(2));
        let pattern = "a";
        assert_eq!(bmbc_match(text, pattern), Some(4));
        let pattern = "skill";
        assert_eq!(bmbc_match(text, pattern), Some(57));
        let pattern = "help";
        assert_eq!(bmbc_match(text, pattern), None);
    }
}
