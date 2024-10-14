use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

pub struct Words<'a> {
    pub words: HashMap<&'a str, &'a str>,
    rng: ThreadRng,
}

impl<'a> Words<'a> {
    pub fn new(file_contents: &'a str) -> Self {
        let words = file_contents.trim().lines().map(|line| {
            line.split_once(',')
                .unwrap_or_else(|| panic!("Line {line} doesn't have a comma"))
        });

        Self {
            words: HashMap::from_iter(words),
            rng: rand::thread_rng(),
        }
    }

    pub fn random(&mut self) -> Option<(&&str, &&str)> {
        self.words.iter().choose(&mut self.rng)
    }
}
