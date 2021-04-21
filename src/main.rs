mod dictionary;
mod mutation;
mod game;

use std::path::Path;
use std::collections::HashSet;
use crate::dictionary::Entry;
use std::iter::FromIterator;
use crate::game::start;

const FILENAME: &str = "dictionary.csv";

fn main() {
    let dictionary = dictionary::parse(Path::new(FILENAME)).unwrap_or_else(|e| panic!("Received error when reading file {}: {:?}", FILENAME, e));

    let filtered_dictionary = dictionary.iter().cloned().filter(|k| k.mood.is_some()).collect::<Vec<_>>();

    let dictionary_set: HashSet<Entry> = HashSet::from_iter(filtered_dictionary);

    start(&dictionary_set);
}
