
use std::io;

mod queries;
mod parser;
mod storage;

use storage::storage as Storage;
use storage::record as Record;

fn readline() -> String {
    let mut input = String::new();
    let read_result = io::stdin().read_line(&mut input);
    if let Result::Ok(_) = read_result {
        return input;        
    }
    else{
        println!("Error: Couldn't read from console");
        return "".to_string()
    }
}

fn fill_db_with_data(db : &mut Storage) {
    let tmp_q = parser::parse_query(&"Add Alex to Engineering".to_string());
    db.execute(tmp_q);
    let tmp_q = parser::parse_query(&"Add Nadia to Engineering".to_string());
    db.execute(tmp_q);
    let tmp_q = parser::parse_query(&"Add Opra to sales".to_string());
    db.execute(tmp_q);
    let tmp_q = parser::parse_query(&"Add Gamma to sales".to_string());
    db.execute(tmp_q);
    let tmp_q = parser::parse_query(&"Add Lisa to Engineering".to_string());
    db.execute(tmp_q);
    let tmp_q = parser::parse_query(&"Add Emma to sales".to_string());
    db.execute(tmp_q);
}

fn main() {
    //let departments : HM<_,_>;
    let mut db : Storage = Storage::new();
    fill_db_with_data(&mut db);
    loop {
        let s = readline();
        let query : queries::QueryType = parser::parse_query(&s);
        if let queries::QueryType::Exit = query {
            println!("Exited the program :(");
            return;   
        }
        else{
            println!("{:#?}", query);
            db.execute(query);
        }
    }
    //println!("Your line: {}", s);
}

// Examples:
// Add Sally to Engineering
// Add Amir to Sales
// List personel in department
// List personel in company sorted by department alphabetically
// List personel in departament sorted by name alphabetically ascending
// List departments in company sorted by name alphabetically ascending
//struct Query {}
