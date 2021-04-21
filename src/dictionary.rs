use serde::Deserialize;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Debug};
use std::fmt;

extern crate rand;

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Amount {
    Singular,
    Plural,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Gender {
    Masculine,
    Neutral,
    Feminine,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Tense {
    Present,
    Past,
    Future,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Mood {
    Indicative,
    Imperative,
}

#[derive(Debug)]
pub enum DictionaryError {
    FailedToOpen,
    FailedToParse,
}

#[derive(serde::Deserialize, Clone, Eq)]
pub struct Entry {
    pub word: String,
    pub base: String,
    pub person: Option<u8>,
    pub amount: Option<Amount>,
    pub imperfective: bool,
    pub concrete: bool,
    pub gender: Option<Gender>,
    pub mood: Option<Mood>,
    pub meaning: String,
    pub tense: Option<Tense>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::from("");
        result.push_str(&format!("{}: ", self.base));
        result.push_str(&format!("{}", match self.person {
            Some(1) => "1st person ",
            Some(2) => "2nd person ",
            Some(3) => "3rd person ",
            None | _ => ""
        }));
        result.push_str(&format!("{}", if self.amount.is_some() { format!("{:?} ", (self.amount).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("{}", if self.imperfective { "imperfective " } else { "perfective " }));
        result.push_str(&format!("{}", if self.concrete { "concrete " } else { "abstract " }));
        result.push_str(&format!("{}", if self.gender.is_some() { format!("{:?} ", (self.gender).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("{}", if self.mood.is_some() { format!("{:?} ", (self.mood).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("{}", if self.tense.is_some() { format!("{:?} ", (self.tense).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("-> {} ", self.meaning));
        write!(f, "{}", result.trim().to_lowercase())
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::from(&self.word);
        result.push_str(&format!(" based off {}: ", self.base));
        result.push_str(&format!("{}", match self.person {
            Some(1) => "1st person ",
            Some(2) => "2nd person ",
            Some(3) => "3rd person ",
            None | _ => ""
        }));
        result.push_str(&format!("{}", if self.amount.is_some() { format!("{:?} ", (self.amount).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("{}", if self.imperfective { "imperfective " } else { "perfective " }));
        result.push_str(&format!("{}", if self.concrete { "concrete " } else { "abstract " }));
        result.push_str(&format!("{}", if self.gender.is_some() { format!("{:?} ", (self.gender).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("{}", if self.mood.is_some() { format!("{:?} ", (self.mood).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("{}", if self.tense.is_some() { format!("{:?} ", (self.tense).as_ref().unwrap()) } else { String::from("") }));
        result.push_str(&format!("-> {} ", self.meaning));
        write!(f, "{}", result.trim().to_lowercase())
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base &&
            self.person == other.person &&
            self.amount == other.amount &&
            self.imperfective == other.imperfective &&
            self.concrete == other.concrete &&
            self.gender == other.gender &&
            self.mood == other.mood &&
            self.tense == other.tense
    }
}

impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.person.hash(state);
        self.amount.hash(state);
        self.imperfective.hash(state);
        self.concrete.hash(state);
        self.gender.hash(state);
        self.tense.hash(state);
        self.mood.hash(state);
    }
}

fn read_dictionary(path: &Path) -> Result<Vec<Entry>, DictionaryError> {
    let mut reader = match csv::Reader::from_path(path) {
        Ok(r) => r,
        Err(err) => {
            println!("Error received when opening file: {:?}", err);
            return Err(DictionaryError::FailedToOpen);
        }
    };

    let mut dict: Vec<Entry> = vec![];
    for result in reader.deserialize() {
        let entry: Entry = match result {
            Ok(s) => s,
            Err(err) => {
                println!("Error received when parsing file: {:?}", err);
                return Err(DictionaryError::FailedToParse);
            }
        };
        dict.push(entry);
    }
    Ok(dict)
}

pub fn parse(path: &Path) -> Result<Vec<Entry>, DictionaryError> {
    read_dictionary(path)
}
