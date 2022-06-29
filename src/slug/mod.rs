
use deunicode::deunicode_char;

const SEP: char = '-';

pub fn slugify<S: AsRef<str>>(s: S) -> String {
  _slugify(s.as_ref())
}

pub fn _slugify(s: &str) -> String {
  let mut slug = String::new();
  let mut prev: char = SEP; // starts with sep to avoid leading '-'
  
  for c in s.chars() {
    if c.is_ascii() {
      if let Some(c) = convert(c, prev) {
        slug.push(c);
        prev = c;
      }
    }else{
      for x in deunicode_char(c).unwrap_or("-").chars() {
        if let Some(x) = convert(x, prev) {
          slug.push(x);
          prev = x;
        }
      }
    }
  }
  
  slug
}

fn convert(c: char, p: char) -> Option<char> {
  match c {
    '0'..='9' | 'a'..='z' => Some(c),
    'A'..='Z' => Some(((c as u8) - b'A' + b'a') as char),
    _ => if p != SEP {
      Some(SEP)
    }else{
      None
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_slugify() {
    assert_eq!("a".to_string(), slugify("a"));
    assert_eq!("e".to_string(), slugify("é"));
    assert_eq!("".to_string(), slugify(" "));
    assert_eq!("".to_string(), slugify("    "));
    assert_eq!("abc".to_string(), slugify("abc"));
    assert_eq!("abc-def".to_string(), slugify("abc def"));
    assert_eq!("abc-def".to_string(), slugify("abc_def"));
    assert_eq!("abc-def-ghi".to_string(), slugify("abc_def-GHI"));
    assert_eq!("abc-def-ghi-789".to_string(), slugify("abc_def-GHI   789"));
  }
  
}
