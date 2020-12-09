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

fn generate_rule_hashmap(vector_bag_rules:&Vec<BagRule>)-> HashMap::<String, Option<&Vec<(usize, String)>>>{
    let mut hash_rule = HashMap::<String, Option<&Vec<(usize, String)>>>::new();
    for rule in vector_bag_rules.iter(){
        hash_rule.insert(rule.color.trim().to_string(), rule.can_contain.as_ref());
    }
    hash_rule
}


fn rec_deep_scan(current_color_bag: String , hashmap: &HashMap::<String, Option<&Vec<(usize, String)>>>)->usize{
    println!("{}", current_color_bag);
    //println!("{:?}", hashmap);
    if hashmap[&current_color_bag] == None{
        0
    }else{
        hashmap[&current_color_bag]
            .expect("error on unwrap in rec deep find")
            .iter().map(|(size, bag_color)|{
                println!("{} {}",size, bag_color);
                size * (1+rec_deep_scan(bag_color.to_string(), hashmap))
            }).sum()
    }
}




fn main(){
    let input:String = read_file("../input.txt").expect("Errore nella lettura file");

    let vector_rules:Vec<&str> = input.lines().collect();

    let vector_bag_rules: Vec<BagRule> = vector_rules.iter().map(|rule_s|BagRule::from(*rule_s)).collect();

    let hashmap_rules: HashMap::<String, Option<&Vec<(usize, String)>>> = generate_rule_hashmap(&vector_bag_rules);

    //println!("{:?}", hashmap_rules);

    let start_bag = "shiny gold".to_string();

    let count = rec_deep_scan(start_bag, &hashmap_rules);
    println!("{}",count);

}