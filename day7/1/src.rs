use std::fs::File;
use std::io::prelude::*;
// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// --- model

#[derive(Debug, Eq, PartialEq)]
struct BagRule {
    color: String,
    can_contain: Option<Vec<(usize, String)>>,
}

impl BagRule{
    fn println(&self){
        println!("{}", self.color);
        println!("{:?}", self.can_contain);
    }

    fn contain_bag(&self, color: &str)->bool{
        //println!("{:?}",self.can_contain);
        if self.can_contain == None{
            false
        }else if self.can_contain.as_ref().unwrap().iter().filter(|(_n, col)| col == color ).count() >= 1{
            true
        }else{
            false
        }
    }

}

impl From<&str> for BagRule{
    fn from(s: &str) -> Self {
        let rule_scomposed_1: Vec<String> = s.split("bags contain").map(|s|s.to_string()).collect();
        let color_container = String::from(&rule_scomposed_1[0]);
        let bag_can_contain = String::from(&rule_scomposed_1[1]);

        let bag_inside_array = bag_can_contain.trim_end_matches(".").trim_end_matches(" ").trim_start_matches(" ");
        let bag_inside_array = bag_inside_array.split(", ");

        let mut can_contain: Option<Vec<(usize, String)>> = Some(Vec::<(usize, String)>::new());

        for item in bag_inside_array{

            let split_item:Vec<String> = item.split(" ").map(|s|s.to_string()).collect();

            let num_s:String = String::from(&split_item[0]);
            if num_s != "no"{
                let num:usize = num_s.parse().unwrap();

                let item_clone: String = String::from(item); 
                let color_inside = item_clone.trim_start_matches(&num_s)
                                             .trim_end_matches(&split_item[3])
                                             .trim()
                                             .to_string();

                let touple = (num, color_inside);

                let mut can_contain_tmp: Vec<(usize, String)> = can_contain.unwrap();
                can_contain_tmp.push(touple);
                can_contain = Some(can_contain_tmp);
            }else{
                can_contain = None;
                break;
            }
            

        }

        BagRule{color: color_container, can_contain: can_contain }
    }
}







fn main(){
    let input:String = read_file("../input.txt").expect("Errore nella lettura file");

    let vector_rules:Vec<&str> = input.lines().collect();

    let mut color_vec: Vec<String> = Vec::<String>::new();
    let mut color_vec_counted: Vec<String> = Vec::<String>::new();

    let vector_bag_rules: Vec<BagRule> = vector_rules.iter().map(|rule_s|BagRule::from(*rule_s)).collect();

    color_vec.push("shiny gold".to_string());
    let mut counter:usize = 0;

    while !color_vec.is_empty(){
        println!("{:?}",color_vec);

        let curr_color = color_vec.pop().unwrap();
        color_vec_counted.push(String::from(&curr_color));

        for bag_rule in vector_bag_rules.iter(){

            if bag_rule.contain_bag(&curr_color){

                let c = &bag_rule.color.trim();
                
                if !color_vec_counted.contains(&c.to_string()) && !color_vec.contains(&c.to_string()){
                    color_vec.push(c.to_string());
                    counter += 1;
                }
                
            }
        }
        
    }

    
    println!("{}", counter);

}