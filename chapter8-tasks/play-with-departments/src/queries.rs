
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(PartialOrd, Eq, Ord, Hash)]
pub enum DepartmentType {
    Sales,
    Engineering
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AddQuery {
    pub person_name : String,
    pub department : DepartmentType
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum QueryListWhat {
    Personel,
    Departments
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum QueryListWhere {
    Department,
    Company
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SortBy {
    Department,
    Name
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SortHow {
    Alphabetically
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct SortParameters {
    pub by : SortBy,
    pub how : SortHow,
    pub order : SortOrder
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum NeedSort {
    Yes(SortParameters),
    No
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ListQuery {
    pub what : QueryListWhat,
    pub where_from : QueryListWhere,
    pub sort : NeedSort,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum QueryType {
    Add(AddQuery),
    List(ListQuery),
    Exit,
    NoQuery
}
