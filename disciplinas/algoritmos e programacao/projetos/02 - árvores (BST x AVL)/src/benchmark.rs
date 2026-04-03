use std::time::Instant;
use crate::employee::Employee;
use crate::bst::BST;
use crate::avl::AVL;

pub fn measure_bst_insert_sorted(data: &Vec<Employee>) -> f32 {
    let mut tree = BST::new();

    let start = Instant::now();

    for e in data.iter().take(data.len()) {
        tree.insert(e.clone());
    }

    start.elapsed().as_secs_f32()
}

pub fn measure_avl_insert_sorted(data: &Vec<Employee>) -> f32 {
    let mut tree = AVL::new();

    let start = Instant::now();

    for e in data.iter().take(data.len()) {
        tree.insert(e.clone());
    }

    start.elapsed().as_secs_f32()
}

pub fn measure_bst_insert_random(data: &Vec<Employee>) -> f32 {
    let mut tree = BST::new();

    let start = Instant::now();

    for e in data.iter().take(data.len()) {
        tree.insert(e.clone());
    }

    start.elapsed().as_secs_f32()
}

pub fn measure_avl_insert_random(data: &Vec<Employee>) -> f32 {
    let mut tree = AVL::new();

    let start = Instant::now();

    for e in data.iter().take(data.len()) {
        tree.insert(e.clone());
    }

    start.elapsed().as_secs_f32()
}

pub fn measure_bst_search(data: &Vec<Employee>) -> f32 {
    let mut tree = BST::new();

    for e in data.iter().take(data.len()) {
        tree.insert(e.clone());
    }

    let ids: Vec<i32> = data.iter().take(data.len()).map(|e| e.id).collect();

    let start = Instant::now();

    for id in &ids {
        tree.search(*id);
    }

    start.elapsed().as_secs_f32()
}

pub fn measure_avl_search(data: &Vec<Employee>) -> f32 {
    let mut tree = AVL::new();

    for e in data.iter().take(data.len()) {
        tree.insert(e.clone());
    }

    let ids: Vec<i32> = data.iter().take(data.len()).map(|e| e.id).collect();

    let start = Instant::now();

    for id in &ids {
        tree.search(*id);
    }

    start.elapsed().as_secs_f32()
}

pub fn measure_bst_salary_search(data: &Vec<Employee>, salary: i32) -> f32 {
    let mut tree = BST::new();

    // build tree
    for e in data {
        tree.insert(e.clone());
    }

    let start = Instant::now();

    let _ = tree.find_by_salary(salary);

    start.elapsed().as_secs_f32()
}

pub fn measure_avl_salary_search(data: &Vec<Employee>, salary: i32) -> f32 {
    let mut tree = AVL::new();

    for e in data {
        tree.insert(e.clone());
    }

    let start = Instant::now();

    let _ = tree.find_by_salary(salary);

    start.elapsed().as_secs_f32()
}
