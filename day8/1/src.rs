use std::fs::File;
use std::io::prelude::*;
// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn exec_line(program_code: String, pc:usize, counter: i32)->(usize, i32){
    let mut next_pc = pc;
    let mut next_counter = counter;
    let opcode_arg: Vec<String> = program_code.split(" ").map(|s|s.to_string()).collect();

    if opcode_arg[0] == "nop"{
        next_pc +=1;
        println!("{:<10} \x1b[0;36m PC:{:<7}\x1b[0m \x1b[0;32mCOUNTER:{}\x1b[0m", program_code, pc, counter);
    }else if opcode_arg[0] == "jmp"{

        let arg: String = opcode_arg[1].to_string();

        let sign_s = arg.chars().nth(0).expect("char not found");
        
        let arg:usize = opcode_arg[1].trim_start_matches(sign_s).to_string().parse().unwrap();

        if sign_s == '-'{
            next_pc -= arg;
        }
        else {
            next_pc += arg;
        }

        println!("\x1b[0;35m{:<10}\x1b[0m \x1b[0;36m PC:{:<7}\x1b[0m \x1b[0;32mCOUNTER:{}\x1b[0m", program_code, pc, counter);
    }else if opcode_arg[0] == "acc"{
        next_pc +=1;
        let arg: String = opcode_arg[1].to_string();

        let sign_s = arg.chars().nth(0).expect("char not found");

        let arg:i32 = opcode_arg[1].trim_start_matches(sign_s).to_string().parse().unwrap();

        if sign_s == '-'{
            next_counter -= arg;
        }
        else {
            next_counter += arg;
        }
        //println!("{}", sign);
        println!("\x1b[0;33m{:<10}\x1b[0m \x1b[0;36m PC:{:<7}\x1b[0m \x1b[0;32mCOUNTER:{}\x1b[0m", program_code, pc, counter);
    }


    (next_pc,next_counter)
}

fn main(){
    let input:String = read_file("../input.txt").expect("Errore nella lettura file");
    let program_code: Vec<String> = input.lines().map(|line|line.to_string()).collect();

    let mut pc_history: Vec<usize> = Vec::<usize>::new();
    let mut pc: usize = 0;

    let mut counter: i32 = 0;

    while !pc_history.contains(&pc) && pc < program_code.len(){
        pc_history.push(pc);
        let (ret_pc, ret_counter) = exec_line(program_code[pc].to_string(), pc, counter);
        pc = ret_pc;
        counter = ret_counter;

        //println!("{:?}", pc_history);
        //println!("{:?}", !pc_history.contains(&pc));
        //println!("{:?}", (pc < program_code.len()));
    }
    
}