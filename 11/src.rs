use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// --- model

#[derive(Debug, Eq, PartialEq)]
struct Group {
    g_answer: Vec<Vec<char>>
}

impl Group {
    fn new(input: Vec<Vec<char>>) -> Group {
        Group {g_answer: input}
    }
}



fn n_yes_question_for_goup(group: &Group)->usize{
    group.g_answer
    .iter()
    .flat_map(|user_answer|user_answer.iter().map(|ch| *ch).collect::<HashSet<char>>())
    .collect::<HashSet<char>>()
    .len()
}



fn main(){
    let input = read_file("./input.txt").unwrap();
    let input_array:Vec<Group> = input.split("\n\n")
                                      .map(|group_answers| {
                                        Group::new(group_answers.lines()
                                            .map(|user_answer| user_answer.chars().collect::<Vec<char>>()).collect())
                                      }).collect::<Vec<Group>>();

    let mut counter = 0;

    for group in input_array.iter(){
        counter += n_yes_question_for_goup(group);
    }
    println!("{}", counter);
}