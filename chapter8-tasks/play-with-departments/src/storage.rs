
//use std::collections::hash_set::Iter;
//use std::collections::HashMap;
use std::collections::HashSet;
use crate::queries;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct record {
    department : queries::DepartmentType,
    person : String,
}

pub struct storage {
    pub records : Vec<record>,
    //pub by_person_name : HashMap<&String, &record>,
    //pub by_department_type : HashMap<queries::DepartmentType, record>,
}


pub enum QueryResult {
    Personel(Vec<String>),
    Departments(Vec<queries::DepartmentType>),
    Nothing
}

impl storage {

    pub fn new() -> storage {
        storage { records : Vec::new() }
    }

    pub fn insert_record(self : &mut Self, rec : record) {
        //let rec_ref = &rec;
        //let person_name = rec.person;
        self.records.push(rec);
        //self.by_person_name.insert(&rec_ref.person, &rec);
    }
    pub fn insert(self : &mut Self, name : String, dep : queries::DepartmentType) {
        let new_rec = record { person : name, department : dep };
        self.insert_record(new_rec);
    }
/*
    pub fn select_by_person(self : &Self, p_name : &String) -> Vec<&record> {

        let mut result = Vec::new();
        for r in &self.records {
            if r.person == *p_name {
                println!("Selection added by person name");
                result.push(r);
            }
        }
        result
    }
    
    pub fn select_by_department(self : &Self, dep : &queries::DepartmentType) -> Vec<&record> {

        let mut result = Vec::new();
        for r in &self.records {
            if r.department == *dep {
                println!("Selection added by department");
                result.push(r);
            }
        }
        result
    }
*/
    pub fn select_all_people(self : &Self) -> Vec<String> {

        let mut people = HashSet::new();
        let mut vec = Vec::new();
        for r in &self.records {
            if people.insert(r.person.to_string()){
                vec.push(r.person.to_string());
            }
        }
        vec
    }
    pub fn select_all_departments(self : &Self) -> Vec<queries::DepartmentType> {

        let mut deps = HashSet::new();
        let mut vec = Vec::new();
        for r in &self.records {
            if deps.insert(r.department){
                vec.push(r.department);
            }

        }
        vec
    }

    pub fn select_people_where(self : &Self, dep_type : queries::DepartmentType) -> Vec<&String> {
        let mut people = HashSet::new();
        for r in &self.records {
            if r.department == dep_type {
                people.insert(r);
            }
        }
        let mut people_vec = Vec::new();
        for item in people {
            people_vec.push(&item.person);
        }
        people_vec
    }

    pub fn execute_query(self : &mut Self, query : &queries::ListQuery) -> QueryResult {
        //if let queries::QueryListWhat::Personel = query.what 
        match query.what
        {
            queries::QueryListWhat::Personel => {
                match query.where_from {
                    queries::QueryListWhere::Department => {
                        let mut personel = self.select_all_people();
                    }
                    queries::QueryListWhere::Company => {
                        let mut personel = self.select_all_people();
                        if let queries::NeedSort::Yes(sort) = &query.sort {
                            if let queries::SortOrder::Ascending = sort.order {
                                personel.sort();
                            }
                            else{
                                personel.sort_by(|a, b| b.cmp(a))
                            }
                        }
                        return QueryResult::Personel(personel);
                    }
                }
            }
            queries::QueryListWhat::Departments => {
                match query.where_from {
                    queries::QueryListWhere::Department => {
                        let mut departments = self.select_all_departments();
                        if let queries::NeedSort::Yes(sort) = &query.sort {
                            departments.sort();
                            if let queries::SortOrder::Ascending = sort.order {
                                departments.sort();
                            }
                            else {
                                departments.sort_by(|a, b| b.cmp(a));
                            }
                        }
                        return QueryResult::Departments(departments);
                    }
                    queries::QueryListWhere::Company => {
                    }
                }
            }
        }
        QueryResult::Nothing
    }

    pub fn execute_non_query(self : &mut Self, query : &queries::AddQuery) {
        self.insert(query.person_name.to_string(), query.department);
    }

    
    pub fn execute(self : &mut Self, query : queries::QueryType) {
        
        match query {
            queries::QueryType::Add(add_query) => {
                self.execute_non_query(&add_query);
            },
            queries::QueryType::List(list_query) => {
                let result = self.execute_query(&list_query);
                match result {
                    QueryResult::Departments(v) => {
                        println!("Retrived records: ");
                        for i in v {
                            println!("{:#?}", i);
                        }
                    },
                    QueryResult::Personel(v) => {
                        println!("Retrived records: ");
                        for i in v {
                            println!("{}", i);
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

