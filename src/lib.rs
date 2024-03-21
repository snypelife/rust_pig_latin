use is_vowel::IsRomanceVowel;

pub struct Config {
    pub sentence: String
}

impl Config {
    pub fn parse(args: Vec<String>)-> Config {
        let default_sentence: String = "The quick brown fox jumped over the lazy dog".to_string();
        let sentence: String = args.get(1).unwrap_or(&default_sentence).to_string();
        Config { sentence }
    }
}

pub fn run(sentence: &String)-> String {
    sentence
      .to_lowercase()
      .split_terminator(" ")
      .map(|w| rearrange_word(&w.to_string()))
      .collect::<Vec<String>>()
      .join(" ")
}

struct Word {
    start: String,
    rest: String,
    special_chars: String,
    vowel_start: bool,
    previous_char: char
}

impl Word {
    fn default()-> Word {
        Word {
            start: String::new(),
            rest: String::new(),
            special_chars: String::new(),
            vowel_start: false,
            previous_char: char::default()
        }
    }

    fn to_string(&self)-> String {
        [
            &self.rest,
            &self.start,
            &if &self.rest == "" && &self.start == "" {
                ""
            } else if self.vowel_start {
              "way"
            } else {
              "ay"
            }.to_string(),
            &self.special_chars
        ]
        .iter()
        .fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        })
        .to_string()
    }
}

fn rearrange_word(word: &String)-> String {
    let rearrange = |mut word: Word, (i, ch): (usize, char)| {
        if !ch.is_alphanumeric() {
            word.special_chars.push(ch);
        } else if is_vowel(ch) {
            if ch == 'u' && word.previous_char == 'q' {
                word.start.push(ch);
            } else if ch == 'y' && i == 0 {
                word.start.push(ch);
            } else {
                word.rest.push(ch);
                if i == 0 {
                  word.vowel_start = true;
                }
            }
        } else {
            if word.rest.len() == 0 {
              word.start.push(ch);
            } else {
              word.rest.push(ch);
            }
        }
        word.previous_char = ch;
        word
    };

    word
        .chars()
        .enumerate()
        .fold(Word::default(), rearrange)
        .to_string()
}

fn is_vowel(ch: char)-> bool {
    ch.is_romance_vowel() || ch == 'y'
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn handles_single_word() {
      let cases = [
        ("hello", "ellohay"),
        ("over", "overway"),
        ("queen", "eenquay"),
        ("you", "ouyay"),
        ("my", "ymay"),
      ];

      for case in cases {
        assert_eq!(case.1, rearrange_word(&case.0.to_string()));
      }
    }

    #[test]
    fn handles_sentence() {
      let cases = [
        (
          "Hello, my name is Hal",
          "ellohay, ymay amenay isway alhay"
        ),
        (
          "Hello there you cheeky little monkey",
          "ellohay erethay ouyay eekychay ittlelay onkeymay"
        )
      ];

      for case in cases {
        assert_eq!(case.1, run(&case.0.to_string()));
      }
    }

    #[test]
    fn handles_multiline_sentence() {
      let cases = [
        (
          "A world of dew,
          And within every dewdrop
          A world of struggle.",
          "away orldway ofway ewday,
          andway ithinway everyway ewdropday
          away orldway ofway ugglestray."
        ),
      ];

      for case in cases {
        assert_eq!(case.1, run(&case.0.to_string()));
      }
    }
}
