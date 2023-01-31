use std::collections::HashMap;
use std::io::{self, Read};

struct Vocabulary {
    words: HashMap<String, usize>,
    counts: Vec<(String, usize)>,
}

impl Vocabulary {
    fn new() -> Vocabulary {
        Vocabulary {
            words: HashMap::new(),
            counts: vec![],
        }
    }

    fn add_word(&mut self, word: &str) {
        let count = self.words.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    fn build(&mut self) {
        self.counts = self
            .words
            .iter()
            .map(|(word, count)| (word.to_owned(), *count))
            .collect();

        self.counts.sort_by(|a, b| b.1.cmp(&a.1));
    }

    fn print_most_common(&self, n: usize) {
        let n = n.min(self.counts.len());

        for (word, count) in self.counts[..n].iter() {
            println!("{}: {}", word, count);
        }
    }
}

fn main() {
    let mut vocabulary = Vocabulary::new();
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    for word in input.split_whitespace() {
        vocabulary.add_word(word);
    }

    vocabulary.build();
    vocabulary.print_most_common(10);
}
