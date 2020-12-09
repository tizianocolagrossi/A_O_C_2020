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







fn main(){
    let input = read_file("./input.txt").unwrap();
    let input_array:Vec<Group> = input.split("\n\n")
                                      .map(|group_answers| {
                                        Group::new(group_answers.lines()
                                            .map(|user_answer| user_answer.chars().collect::<Vec<char>>()).collect())
                                      }).collect::<Vec<Group>>();


    let mut counter = 0;
    for group in input_array.iter(){
        let mut answer_common_group: Option<HashSet<char>> = None;

        for user_answer in &group.g_answer{
            let mut current_answer: HashSet<char> = HashSet::<char>::new();
            for ch in user_answer{
                current_answer.insert(*ch);
            }

            match answer_common_group{
                None => answer_common_group = Some(current_answer.clone()),
                Some(x)=> answer_common_group = Some(x.intersection(&current_answer).map(|c| *c).collect()),
            }
            
        }

        counter += answer_common_group.unwrap().len();

    }
    println!("{}", counter);
}