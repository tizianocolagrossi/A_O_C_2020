use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


fn main(){
    let input:String = read_file("../input.txt").expect("Error reading input");
    let nums:Vec<usize> = input.lines().map(|s_num| s_num.parse().unwrap()).collect();
    let mut sum_to_component: HashMap<usize, Vec<(usize, usize)>> = HashMap::<usize, Vec<(usize, usize)>>::new();
    let nums_len = nums.len();

    for i1 in 0..nums_len{
        for i2 in 0..nums_len{
            let comp_1 = nums[i1];
            let comp_2 = nums[i2];

            if comp_1 != comp_2 && comp_1 < comp_2{
                let sum: usize = comp_1 + comp_2;
                let new_tmp_value:Vec<(usize, usize)> = Vec::<(usize, usize)>::new();
                sum_to_component.entry(sum)
                    .or_insert_with(||new_tmp_value)
                    .push((comp_1, comp_2));
            }
        }
    }

    let mut counter = 0;
    let mut invalid_number = 0;
    for num in &nums{
        if !sum_to_component.contains_key(&num) && counter > 25{
            invalid_number = *num;
            println!("Solution to part 1 {}", num);
            break;
        }
        counter += 1;
    }

    //part 2
    let mut resolved:bool = false;

    for start_index in 0..nums_len{
        let mut accumulator = 0;
        let mut max: usize = nums[start_index];
        let mut min: usize = nums[start_index];

        for curr_index in start_index..nums_len{
            accumulator += nums[curr_index];

            if nums[curr_index] < min {min = nums[curr_index];}
            if nums[curr_index] > max {max = nums[curr_index];} 

            if accumulator == invalid_number{
                println!("Solution to part 2 {}",min+max);
                resolved = true;
                break;
            }else if accumulator > invalid_number{
                break;
            }
        }
        if resolved{
            break;
        }
    }

}