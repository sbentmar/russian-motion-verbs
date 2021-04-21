use std::collections::HashSet;
use crate::dictionary::Entry;
use crate::mutation::{Mutation, apply_mutation};
use rand::seq::SliceRandom;
use std::io;
use rand::thread_rng;
use std::time::Instant;

fn generate_mutation(dictionary: &HashSet<Entry>, base_word: &Entry) -> (Mutation, Entry) {
    let mut rng = thread_rng();
    let valid_mutations: Vec<Mutation> = vec![
        // Mutation::ChangeImperfective,
        // Mutation::ChangeConcrete,
        Mutation::ChangePerson,
        Mutation::ChangeAmount,
        Mutation::ChangeTense,
        Mutation::ChangeMood,
        Mutation::ChangeGender
    ];
    let mut mutation = valid_mutations.choose(&mut rng).unwrap();
    let mut mutated_word = apply_mutation(&base_word, mutation);
    while mutated_word.is_err() {
        mutation = valid_mutations.choose(&mut rng).unwrap();
        mutated_word = apply_mutation(&base_word, mutation);
    }
    let correct_answer = dictionary.get(&mutated_word.unwrap()).unwrap();
    (mutation.clone(), correct_answer.clone())
}

pub fn start(dictionary: &HashSet<Entry>) {
    let mut rng = thread_rng();
    let mut dictionary_vec = vec![];
    dictionary_vec.extend(&mut dictionary.iter());

    let mut command = String::new();
    let mut score = 0;
    let mut rounds = 0;
    while command.trim() != "exit" {
        let base_word = dictionary_vec.choose(&mut rng).unwrap();
        let (mutation, correct_answer) = generate_mutation(&dictionary, &base_word);

        println!("Take this word: {:?} and apply this change: {}", base_word, mutation);
        println!("The answer should have this form: {}", correct_answer);

        command = String::new();
        io::stdin().read_line(&mut command).expect("failed to read keyboard input!");
        if command.trim() == correct_answer.word {
            println!("You did it!");
            score += 1;
        } else {
            println!("Wrong! I expected this answer: {}", correct_answer.word);
        }
        rounds += 1;
        println!("Correct: {}/{}", score, rounds);
    }
}

fn benchmark(dictionary: &Vec<Entry>) {
    let valid_mutations: Vec<Mutation> = vec![Mutation::ChangeImperfective, Mutation::ChangeConcrete, Mutation::ChangePerson, Mutation::ChangeGender];
    let start = Instant::now();
    for _ in 0..10_000_000 {
        match apply_mutation(&dictionary.choose(&mut rand::thread_rng()).unwrap(), valid_mutations.choose(&mut rand::thread_rng()).unwrap()) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
}