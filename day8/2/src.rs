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

fn exec_line_alternatives(program_code: String, pc:usize, counter: i32, index_of_change: usize)->(usize, i32){
    let mut next_pc = pc;
    let mut next_counter = counter;
    let opcode_arg: Vec<String> = program_code.split(" ").map(|s|s.to_string()).collect();

    let mut opcode_s = opcode_arg[0].to_string();

    if pc == index_of_change{
        if opcode_s == "nop"{
            println!("\n\x1b[0;31mvvvvvvvvv NOP CHANGED TO JMP vvvvvvvv\x1b[0m");
            opcode_s = "jmp".to_string();
        }else{
            println!("\n\x1b[0;31mvvvvvvvvv JMP CHANGED TO NOP vvvvvvvv\x1b[0m");
            opcode_s = "nop".to_string();
        }
    }


    if opcode_s == "nop"{
        next_pc +=1;
        println!(" {:<10} \x1b[0;36m PC:{:<7}\x1b[0m \x1b[0;32mCOUNTER:{}\x1b[0m", program_code, pc, counter);
    }else if opcode_s == "jmp"{

        let arg: String = opcode_arg[1].to_string();

        let sign_s = arg.chars().nth(0).expect("char not found");
        
        let arg:usize = opcode_arg[1].trim_start_matches(sign_s).to_string().parse().unwrap();

        if sign_s == '-'{
            next_pc -= arg;
        }
        else {
            next_pc += arg;
        }

        println!(" \x1b[0;35m{:<10}\x1b[0m \x1b[0;36m PC:{:<7}\x1b[0m \x1b[0;32mCOUNTER:{}\x1b[0m", program_code, pc, counter);
    }else if opcode_s == "acc"{
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
        println!(" \x1b[0;33m{:<10}\x1b[0m \x1b[0;36m PC:{:<7}\x1b[0m \x1b[0;32mCOUNTER:{}\x1b[0m", program_code, pc, counter);
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
    println!("FIRST RUN OK POPOLATED PC HISTORY");
    println!("DUMP OF PC_HISTORY ARRAY >{:?}<\n\n", pc_history);
    println!("FIND ALL NOP OR JMP OP IN THE HISTORY ...");

    let mut possible_change: Vec<usize> = Vec::<usize>::new();

    for pc_hist_index in pc_history{
        let code_line = program_code[pc_hist_index].to_string();
        let opcode_arg: Vec<String> = code_line.split(" ").map(|s|s.to_string()).collect();

        if opcode_arg[0] == "nop"{
            possible_change.push(pc_hist_index);
            println!("\x1b[0;32m NOP FOUND AT PC:{}\x1b[0m", pc_hist_index);
        }else if opcode_arg[0] == "jmp"{
            possible_change.push(pc_hist_index);    
            println!("\x1b[0;32m JMP FOUND AT PC:{}\x1b[0m", pc_hist_index);
        }
    } 

    println!("Found {} POSSIBLE ALTERNATIVES EXECUTIONS", possible_change.len() );

    println!(" TRY ALL POSSIBLE ALTERNATIVES" );
    
    let mut counter_attempt = 0;

    for change_index in possible_change{

        counter_attempt += 1;

        pc_history = Vec::<usize>::new();
        pc = 0;
        counter = 0;
        println!("");
        println!(" \x1b[0;33m####################################\x1b[0m");
        println!(" \x1b[0;33m####################################\x1b[0m");
        println!(" \x1b[0;33m##### TESTING NEW ALTERNATIVES #####\x1b[0m");
        println!(" \x1b[0;33m####################################\x1b[0m");
        println!(" \x1b[0;33m####################################\x1b[0m");
        println!("");
        while !pc_history.contains(&pc) && pc < program_code.len(){
            pc_history.push(pc);
            let (ret_pc, ret_counter) = exec_line_alternatives(program_code[pc].to_string(), pc, counter, change_index);
            pc = ret_pc;
            counter = ret_counter;
        }
        if pc ==  program_code.len(){
            println!("");
            println!(" \x1b[0;32m####################################\x1b[0m");
            println!(" \x1b[0;32m####################################\x1b[0m");
            println!(" \x1b[0;32m##### SUCCESS ATTEMPT N {:<6} #####\x1b[0m", counter_attempt);
            println!(" \x1b[0;32m####################################\x1b[0m");
            println!(" \x1b[0;32m####################################\x1b[0m");
            println!("");
            println!("ACC VALUE:{} CURRENT PC:{} TOTAL LENGHT OF CODE:{}", counter, pc, program_code.len());
            println!("");
            println!("");
            break;
        }else{
            println!("");
            println!(" \x1b[0;31m####################################\x1b[0m");
            println!(" \x1b[0;31m####################################\x1b[0m");
            println!(" \x1b[0;31m##### FAILED ATTEMPT N {:<7} #####\x1b[0m", counter_attempt);
            println!(" \x1b[0;31m####################################\x1b[0m");
            println!(" \x1b[0;31m####################################\x1b[0m");
            println!("");
        }

    }

    


    



}