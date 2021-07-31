pub fn to_pig_latin(sentence: &str) -> String {
  utils::split_into_words(sentence)
    .iter()
    .map(|&w| word_to_pig_latin::convert(w))
    .collect::<Vec<String>>()
    .join(" ")
}

mod word_to_pig_latin {
  use super::utils;

  pub fn convert(word: &str) -> String {
    if word.starts_with(utils::is_vowel) {
      convert_vowel(word)
    } else {
      convert_consonant(word)
    }
  }
  
  fn convert_consonant(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
      Some(c) => format!("{}-{}{}", chars.as_str(), c, "ay"),
      None => String::from(""),
    }
  }

  fn convert_vowel(word: &str) -> String {
    format!("{}-hay", word)
  }
}

mod utils {
  pub fn split_into_words(sentence: &str) -> Vec<&str> {
    sentence
      .split(' ')
      .collect()
  }

  pub fn is_vowel(c: char) -> bool {
    match c {
      'a' | 'e' | 'i' | 'o' | 'u' => true,
      _ => false,
    }
  }
}