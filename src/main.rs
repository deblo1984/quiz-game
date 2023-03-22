use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{self, stdin, stdout, BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: usize,
}

#[derive(Serialize, Deserialize)]
struct CreateQuestion {
    name: String,
    question: String,
    options: Vec<String>,
    answer: usize,
}

fn main() {
    //let mut questions = load_questions("questions.json").unwrap();
    print!("{esc}c", esc = 27 as char);
    println!("Welcome to the rust quiz game!");
    loop {
        println!("Please Select an option");
        println!("1. Start game");
        println!("2. Create quiz topic");
        println!("3. quit");
        print!("Your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                print!("{esc}c", esc = 27 as char);
                play_game();
            }
            "2" => {
                print!("{esc}c", esc = 27 as char);
                create_question();
            }
            "3" => {
                print!("{esc}c", esc = 27 as char);
                break;
            }
            _ => {
                println!("Invalid choice, select between 1 or 2!")
            }
        }
    }
}

fn play_game() {
    let dir_path = "./data";
    let path = Path::new(&dir_path);

    if !path.exists() {
        std::fs::create_dir(path).unwrap();
    }
    let files = std::fs::read_dir(path).unwrap();
    let json_files: Vec<String> = files
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(ext) = file_name.to_str().and_then(|name| name.split('.').last()) {
                    if ext == "json" {
                        return Some(file_name.to_string_lossy().into_owned());
                    }
                }
            }
            None
        })
        .collect();
    if json_files.is_empty() {
        println!("No JSON files found.");
    } else {
        println!("JSON files:");
        for file in &json_files {
            println!("{}", file);
        }

        let mut input = String::new();
        println!("Enter the name of the file to display: ");
        stdin().read_line(&mut input).unwrap();
        let file_name = input.trim();

        if !json_files.contains(&file_name.to_string()) {
            println!("File not found.");
        } else {
            let file_path = format!("{}/{}", dir_path, file_name);
            let questions = load_questions(&file_path).unwrap();
            print!("{esc}c", esc = 27 as char);
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
            println!("Game over! You scored {}/{}", score, &questions.len());
            println!("");
            //questions.push(result);
        }
    }
}

fn create_question() {
    println!("Insert Topic name: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let topic_name = input.trim().to_lowercase();
    input.clear();

    let mut questions = Vec::new();

    for i in 1..3 {
        println!("Question no {} :", i);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let question = input.trim().to_string();

        input.clear();

        let mut options = Vec::new();
        for index in 1..5 {
            input.clear();
            println!("Enter answer {} for question {}: ", index, i);
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            options.push(input.trim().to_string());
        }

        input.clear();
        println!("Enter answer for Question {} [0-3]", i);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let answer = input.trim().parse::<usize>().unwrap_or(0);
        input.clear();
        //insert into json object
        let question_obj = Question {
            question,
            options,
            answer,
        };
        questions.push(question_obj);
    }
    let json = json!(questions);
    let mut file = File::create(topic_name + ".json").unwrap();
    serde_json::to_writer_pretty(&mut file, &json).unwrap();
    println!("Topic create successfully");
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
