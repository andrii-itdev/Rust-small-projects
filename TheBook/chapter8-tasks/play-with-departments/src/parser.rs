//mod crate::queries;

use crate::queries;

// Examples:
// Add Sally to Engineering
// Add Amir to Sales
// List personel in department
// List personel in company sorted by department alphabetically
// List personel in departament sorted by name alphabetically ascending
//struct Query {}

fn parse_addquery(it : &mut std::str::SplitWhitespace<'_>) -> queries::AddQuery {
    let person_name : String;
    let department : queries::DepartmentType;

    if let Some(name) = it.next() {
        person_name = name.to_string();

        if let Some(to) = it.next() {
            if to != "to" {
                panic!("Error::Expected 'to' token, {} found", to);
            }
        }

        if let Some(dep) = it.next() {
            match dep {
                "engineering" => { 
                    department = queries::DepartmentType::Engineering; 
                },
                "sales" => { 
                    department = queries::DepartmentType::Sales; 
                },
                _ => {
                    panic!("Error::No '{}' department is found", dep);
                }
            }
        }
        else {
            panic!("Error::No department is specified");
        }
    }
    else { 
        panic!("Error::No person name is specified");
    }
    queries::AddQuery { department, person_name }
}

fn parse_sort_parameters_how(it : &mut std::iter::Peekable<&mut std::str::SplitWhitespace<'_>>) 
        -> queries::SortHow {
    if let Some(sort_how) = it.peek() {
        if sort_how != &"alphabetically" {
            println!("Error::Incorrect search parameter '{}' expected 'alphabetically'", sort_how);
        }
        it.next();
    }
    queries::SortHow::Alphabetically
}

fn parse_sort_parameters_order(it : &mut std::iter::Peekable<&mut std::str::SplitWhitespace<'_>>) 
        -> queries::SortOrder {
    if let Some(sort_order) = it.peek() {
        match sort_order {
            &"descending" => {
                return queries::SortOrder::Descending;
            }
            &"ascending" => (),
            _ => {
                panic!("Error::Incorrect search parameter {}", sort_order);
            }
        }
    }
    queries::SortOrder::Ascending
}

fn parse_sort_parameters(it : &mut std::str::SplitWhitespace) -> queries::SortParameters {

    let sortby : queries::SortBy; let sorthow : queries::SortHow; let sort_order : queries::SortOrder;
    if let Some(b) = it.next() {
        if b == "by" {
            if let Some(by_what) = it.next() {
                match by_what {
                    "name" => {
                        sortby = queries::SortBy::Name;
                    },
                    "department" => {
                        sortby = queries::SortBy::Department
                    },
                    _ => {
                        panic!("Error::Not such thing as {}. Specify by what should I sort", by_what)
                    }
                }
                let mut p = it.peekable();
                sorthow = parse_sort_parameters_how(&mut p);
                sort_order = parse_sort_parameters_order(&mut p);
            }
            else {
                panic!("Error::Specify by what should I sort")
            }
        }
        else{
            panic!("Error::No such thing as '{}'", b);
        }
    }
    else{
        sortby = queries::SortBy::Name; 
        sorthow = queries::SortHow::Alphabetically; 
        sort_order = queries::SortOrder::Ascending;
    }

    queries::SortParameters { by : sortby, how : sorthow, order : sort_order }
}

fn parse_sort(mut it : &mut std::str::SplitWhitespace) -> queries::NeedSort {

    if let Some(sorted) = it.next() {
        if sorted == "sorted" || sorted == "sort" {
            return queries::NeedSort::Yes(parse_sort_parameters(&mut it));
        }
        else {
            return queries::NeedSort::No;
        }
    }
    else { return queries::NeedSort::No; }
}

fn parse_listquery(mut it : &mut std::str::SplitWhitespace) -> queries::ListQuery {
    let q_what : queries::QueryListWhat;
    let q_where : queries::QueryListWhere;
    let q_sort : queries::NeedSort;

    if let Some(wh) = it.next() {
        match wh {
            "personel" => q_what = queries::QueryListWhat::Personel,
            "departments" => q_what = queries::QueryListWhat::Departments,
            _ => { 
                panic!("Error::No such thing '{}'. Specify what to search (personel, etc.)", wh);
            }
        }

        if let Some(to) = it.next() {
            if to != "in" {
                panic!("Error::Expected 'in' token, {} found", to);
            }
        }

        if let Some(from_where) = it.next() {
            
            match from_where {
                "department" => q_where = queries::QueryListWhere::Department,
                "company" => q_where = queries::QueryListWhere::Company,
                _ => {
                    panic!("Error::No such thing '{}'. Specify where to search (department, company, etc.)", from_where);
                }
            }
        }
        else {
            println!("Searching in the 'company' by default");
            q_where = queries::QueryListWhere::Company;
        }
    }
    else{
        panic!("Error::Nothing specified to list")
    }
    q_sort = parse_sort(&mut it);

    queries::ListQuery { what : q_what, where_from : q_where, sort :  q_sort }
}

pub fn parse_query(q : &String) -> queries::QueryType 
{
    let query : queries::QueryType;
    let lowercased = q.to_lowercase();
    let mut it = lowercased.split_whitespace();
    if let Some(q_type) = it.next() {
        if q_type == "add" {
            let add_query = parse_addquery(&mut it);
            query = queries::QueryType::Add(add_query);
        }
        else if q_type == "list" {
            let list_query = parse_listquery(&mut it);
            query = queries::QueryType::List(list_query);
        }
        else if q_type == "exit" {
            query = queries::QueryType::Exit;
        }
        else { 
            println!("Error::Incorrect query \n\tCouldn't parse at {}", q_type);
            query = queries::QueryType::NoQuery; 
        }
    }
    else{
        println!("Error::No query specified");
        query = queries::QueryType::NoQuery;
    }
    println!("-> Query was parsed");
    query
}
