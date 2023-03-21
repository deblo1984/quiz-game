use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Write};

#[derive(Serialize, Deserialize)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: usize,
}

fn main() {
    let questions = load_questions("questions.json").unwrap();
    print!("{esc}c", esc = 27 as char);
    println!("Welcome to the rust quiz game!");
    loop {
        println!("Please Select an option");
        println!("1. Start game");
        println!("2. Quit");
        print!("Your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                print!("{esc}c", esc = 27 as char);
                play_game(&questions);
            }
            "2" => {
                println!("Goodbye");
                break;
            }
            _ => {
                println!("Invalid choice, select between 1 or 2!")
            }
        }
    }
}

fn play_game(questions: &[Question]) {
    let mut score = 0;
    let mut rng = rand::thread_rng();
    let shuffled_questions = questions.choose_multiple(&mut rng, 3);
    //questions.shuffle(&mut rng);
    for question in shuffled_questions {
        if ask_question(&question) {
            score += 1;
            //print!("{esc}c", esc = 27 as char);
        } else {
            //print!("{esc}c", esc = 27 as char);
        }
    }
    println!("Game over! You scored {}/10", score);
    println!("");
}

fn load_questions(filename: &str) -> io::Result<Vec<Question>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let questions = serde_json::from_reader(reader)?;
    Ok(questions)
}

fn ask_question(question: &Question) -> bool {
    println!("{}", question.question);
    for (i, option) in question.options.iter().enumerate() {
        let letter = match i {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            _ => unreachable!(),
        };
        println!("{} {}", letter, option);
    }
    print! {"Your answer: "}
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let answer = input.trim().to_lowercase();
    match answer.as_str() {
        "a" | "A" if question.answer == 0 => {
            println!("Correct!");
            println!("");
            true
        }
        "b" | "B" if question.answer == 1 => {
            println!("Correct!");
            println!("");
            true
        }
        "c" | "C" if question.answer == 2 => {
            println!("Correct!");
            println!("");
            true
        }
        "D" | "D" if question.answer == 3 => {
            println!("Correct!");
            println!("");
            true
        }
        _ => {
            println!("Incorrect!");
            println!("");
            false
        }
    }
}
