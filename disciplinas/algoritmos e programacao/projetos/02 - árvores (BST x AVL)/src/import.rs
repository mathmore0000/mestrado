use std::fs::File;
use csv::Reader;
use crate::employee::Employee;

pub fn load_csv(path: &str) -> Vec<Employee> {
    let file = File::open(path).expect("erro ao abrir CSV");
    let mut rdr = Reader::from_reader(file);

    let mut employees = Vec::new();

    for result in rdr.records() {
        let record = result.expect("erro ao ler linha");

        let employee = Employee {
            id: record[0].parse().unwrap(),
            // name: record[1].to_string(),
            // age: record[2].parse().unwrap(),
            salary: record[9].parse().unwrap(),
        };

        employees.push(employee);
    }

    employees
}