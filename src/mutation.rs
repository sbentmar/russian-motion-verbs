use crate::dictionary::{Gender, Tense, Amount, Mood, Entry};
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

#[derive(Clone)]
pub enum Mutation {
    ChangeGender,
    ChangePerson,
    ChangeAmount,
    ChangeImperfective,
    ChangeConcrete,
    ChangeTense,
    ChangeMood,
}

impl Display for Mutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Mutation::ChangeGender => "change the gender",
            Mutation::ChangePerson => "change the person",
            Mutation::ChangeAmount => "change the number of people",
            Mutation::ChangeImperfective => "change the aspect (perfective/imperfective)",
            Mutation::ChangeConcrete => "change the aspect (concrete/abstract)",
            Mutation::ChangeTense => "change the tense",
            Mutation::ChangeMood => "change the mood",
        })
    }
}

#[derive(Debug)]
pub enum MutationError {
    InvalidMutation,
}

pub fn apply_mutation(entry: &Entry, mutation: &Mutation) -> Result<Entry, MutationError> {
    match mutation {
        Mutation::ChangeGender => change_gender(entry),
        Mutation::ChangePerson => change_person(entry),
        Mutation::ChangeAmount => change_amount(entry),
        Mutation::ChangeImperfective => change_imperfective(entry),
        Mutation::ChangeConcrete => change_concrete(entry),
        Mutation::ChangeTense => change_tense(entry),
        Mutation::ChangeMood => change_mood(entry)
    }
}

fn change_mood(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    match entry.mood {
        None => { return Err(MutationError::InvalidMutation); }
        Some(Mood::Indicative) => {
            new_entry.mood = Some(Mood::Imperative);
            new_entry.person = Some(2);
            new_entry.gender = None;
            new_entry.tense = None;
        }
        Some(Mood::Imperative) => {
            new_entry.mood = Some(Mood::Indicative);
            new_entry.person = None;
            new_entry.gender = Some(Gender::Feminine);
            new_entry.amount = Some(Amount::Singular);
            new_entry.tense = Some(Tense::Past);
        }
    }

    return Ok(new_entry);
}

fn change_tense(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    match entry.tense {
        None => { return Err(MutationError::InvalidMutation); }
        Some(Tense::Present) => {
            new_entry.tense = Some(Tense::Future);
            new_entry.imperfective = false;
        }
        Some(Tense::Future) => {
            new_entry.tense = Some(Tense::Past);
            if entry.amount == Some(Amount::Singular) {
                new_entry.gender = Some(Gender::Masculine);
            }
            new_entry.person = None;
        }
        Some(Tense::Past) => {
            new_entry.tense = Some(Tense::Present);
            new_entry.imperfective = true;
            new_entry.gender = None;
            new_entry.person = Some(3);
        }
    };
    return Ok(new_entry);
}

fn change_concrete(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    new_entry.concrete = !new_entry.concrete;
    return Ok(new_entry);
}

fn change_imperfective(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    new_entry.imperfective = !new_entry.imperfective;
    new_entry.tense = match entry.tense {
        None => None,
        Some(Tense::Present) => Some(Tense::Future),
        Some(Tense::Future) => Some(Tense::Present),
        Some(_) => new_entry.tense,
    };
    return Ok(new_entry);
}

fn change_amount(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    match entry.amount {
        None => return Err(MutationError::InvalidMutation),
        Some(Amount::Plural) => {
            new_entry.amount = Some(Amount::Singular);
            if entry.tense == Some(Tense::Past) {
                new_entry.gender = Some(Gender::Masculine);
            }
        }
        Some(Amount::Singular) => {
            new_entry.amount = Some(Amount::Plural);
            new_entry.gender = None;
        }
    };
    return Ok(new_entry);
}

fn change_person(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    match entry.person {
        Some(1) => {
            if entry.mood == Some(Mood::Indicative) {
                new_entry.person = Some(2);
            }
        }
        Some(2) => {
            if entry.mood == Some(Mood::Indicative) {
                new_entry.person = Some(3);
            }
            else{
                return Err(MutationError::InvalidMutation);
            }
        }
        Some(3) => {
            if entry.mood == Some(Mood::Indicative) {
                new_entry.person = Some(1);
            }
        }
        None | _ => { return Err(MutationError::InvalidMutation); }
    }
    return Ok(new_entry);
}

fn change_gender(entry: &Entry) -> Result<Entry, MutationError> {
    let mut new_entry = entry.clone();
    match entry.gender {
        None => { return Err(MutationError::InvalidMutation); }
        Some(Gender::Masculine) => {
            new_entry.gender = Some(Gender::Feminine);
        }
        Some(Gender::Neutral) => {
            new_entry.gender = Some(Gender::Masculine);
        }
        Some(Gender::Feminine) => {
            new_entry.gender = Some(Gender::Neutral);
        }
    }
    Ok(new_entry)
}