
use std::collections::HashMap as HM;

fn print_all_string(hm : &HM<String, i32>) {
    for (key, value) in hm {
        println!("'{}' : {}", key, value);
    }
}

fn print_all(hm : &HM<&str, i32>) {
    for (key, value) in hm {
        println!("{} : {}", key, value);
    }
}

fn print_all_str(hm : &HM<&str, &str>) {
    for (key, value) in hm {
        println!("{} : {}", key, value);
    }
}

fn create_hashmap(){    
    let mut elections = HM::new();
    let republicans = "Republicans";
    let democrats = "Democrats";
    elections.insert(republicans, 10);
    elections.insert(democrats, 50);

    print_all(&elections);
}

fn overwriting_value(){
    let mut elections = HM::new();
    elections.insert("Republicans", "Yellow");
    elections.insert("Republicans", "Red");
    print_all_str(&elections);
}

fn no_value_insert() {
    let mut elections = HM::new();
    
    elections.insert("Republicans", 10);
    elections.entry("Democrats1.0").or_insert(50);
    elections.entry("Democrats2.0").or_insert(100);
    elections.entry("Democrats2.0").or_default();
    
    print_all(&elections);
}

fn updating_values(){
    let text = "hello, Hello, HeLLo world wonderful world";
    
    let mut map = HM::new();

    for word in text
    .split(|c| (c == ' ' || c == '\t' || c == ',' || c == '\r') )
    .filter(|c| c != &"" && c != &" " )
    {
        let lowercased = word.to_lowercase();
        let count = map.entry(lowercased).or_insert(0);
        *count += 1;
    }
    print_all_string(&map);
}

fn zip_hashmap(){
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];
    let scores: HM<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter()).collect();
    println!("{:#?}", scores);
}


fn main() {
    zip_hashmap();
    //create_hashmap();
    //overwriting_value();
    //no_value_insert();
    //updating_values();
}
