mod import;
mod bst;
mod employee;
mod avl;
mod export;
mod benchmark;

use import::load_csv;
use export::Exporter;
use rand::thread_rng;
use rand::seq::SliceRandom;
use avl::AVL;
use bst::BST;
use benchmark::{
    measure_bst_insert_sorted,
    measure_avl_insert_sorted,
    measure_bst_insert_random,
    measure_avl_insert_random,
    measure_bst_search,
    measure_avl_search,
    measure_bst_salary_search,
    measure_avl_salary_search,
};

fn main() {
    let employees = load_csv("./data/Employers_data.csv");
    let mut exporter = Exporter::new();

    exporter.write_headers(
        "summary",
        &[
            "Size",
            "BST Insert Sorted",
            "AVL Insert Sorted",
            "BST Insert Random",
            "AVL Insert Random",
            "BST Search Sorted",
            "AVL Search Sorted",
            "BST Search Random",
            "AVL Search Random",
            "BST Salary Search",
            "AVL Salary Search",
        ],
    );

    exporter.write_headers(
        "runs_insert",
        &[
            "Algorithm",
            "Size",
            "Run",
            "Time (s)",
        ],
    );

    exporter.write_headers(
        "runs_search_by_key",
        &[
            "Algorithm",
            "Size",
            "Run",
            "Time (s)",
        ],
    );

    exporter.write_headers(
        "runs_by_salary",
        &[
            "Algorithm",
            "Size",
            "Run",
            "Time (s)",
        ],
    );

    exporter.write_headers(
        "heights",
        &[
            "Algorithm",
            "Size",
            "Run",
            "Height",
        ],
    );

    let sizes = [1000, 5000, 10000];
    let retries = 5;
    for &size in &sizes {
    println!("Calculating for size: {}", size);

    let base = employees.iter().take(size).cloned().collect::<Vec<_>>();

    let mut sorted = base.clone();
    sorted.sort_by_key(|e| e.id);

    let mut random = base.clone();
    random.shuffle(&mut thread_rng());

    // =========================
    // 📏 HEIGHT (fora do retry)
    // =========================

    // BST sorted
    let mut bst_sorted = BST::new();
    for e in &sorted {
        bst_sorted.insert(e.clone());
    }
    exporter.write_run(
        "heights",
        "bst_sorted",
        size,
        1,
        bst_sorted.height() as f32,
    );

    // AVL sorted
    let mut avl_sorted = AVL::new();
    for e in &sorted {
        avl_sorted.insert(e.clone());
    }
    exporter.write_run(
        "heights",
        "avl_sorted",
        size,
        1,
        avl_sorted.get_height() as f32,
    );

    // (opcional: random também)
    let mut bst_random = BST::new();
    for e in &random {
        bst_random.insert(e.clone());
    }
    exporter.write_run(
        "heights",
        "bst_random",
        size,
        1,
        bst_random.height() as f32,
    );

    let mut avl_random = AVL::new();
    for e in &random {
        avl_random.insert(e.clone());
    }
    exporter.write_run(
        "heights",
        "avl_random",
        size,
        1,
        avl_random.get_height() as f32,
    );

    // =========================
    // ⏱️ BENCHMARK (igual antes)
    // =========================

    let mut bst_insert_sorted = 0.0;
    let mut avl_insert_sorted = 0.0;
    let mut bst_insert_random = 0.0;
    let mut avl_insert_random = 0.0;

    let mut bst_search_sorted = 0.0;
    let mut avl_search_sorted = 0.0;
    let mut bst_search_random = 0.0;
    let mut avl_search_random = 0.0;

    let mut bst_salary = 0.0;
    let mut avl_salary = 0.0;

    for run in 1..=retries {
        let t1 = measure_bst_insert_sorted(&sorted);
        bst_insert_sorted += t1;
        exporter.write_run("runs_insert", "bst_insert_sorted", size, run, t1);

        let t2 = measure_avl_insert_sorted(&sorted);
        exporter.write_run("runs_insert", "avl_insert_sorted", size, run, t2);
        avl_insert_sorted += t2;

        let t3 = measure_bst_insert_random(&random);
        exporter.write_run("runs_insert", "bst_insert_random", size, run, t3);
        bst_insert_random += t3;

        let t4 = measure_avl_insert_random(&random);
        exporter.write_run("runs_insert", "avl_insert_random", size, run, t4);
        avl_insert_random += t4;

        let t5 = measure_bst_search(&sorted);
        exporter.write_run("runs_search_by_key", "bst_search_sorted", size, run, t5);
        bst_search_sorted += t5;

        let t6 = measure_avl_search(&sorted);
        exporter.write_run("runs_search_by_key", "avl_search_sorted", size, run, t6);
        avl_search_sorted += t6;

        let t7 = measure_bst_search(&random);
        exporter.write_run("runs_search_by_key", "bst_search_random", size, run, t7);
        bst_search_random += t7;

        let t8 = measure_avl_search(&random);
        exporter.write_run("runs_search_by_key", "avl_search_random", size, run, t8);
        avl_search_random += t8;

        let salary_target = base[0].salary;

        let t9 = measure_bst_salary_search(&random, salary_target);
        exporter.write_run("runs_by_salary", "bst_salary", size, run, t9);
        bst_salary += t9;

        let t10 = measure_avl_salary_search(&random, salary_target);
        exporter.write_run("runs_by_salary", "avl_salary", size, run, t10);
        avl_salary += t10;
    }

    exporter.write_summary(
        "summary",
        size,
        bst_insert_sorted / retries as f32,
        avl_insert_sorted / retries as f32,
        bst_insert_random / retries as f32,
        avl_insert_random / retries as f32,
        bst_search_sorted / retries as f32,
        avl_search_sorted / retries as f32,
        bst_search_random / retries as f32,
        avl_search_random / retries as f32,
        bst_salary / retries as f32,
        avl_salary / retries as f32,
    );
}

    exporter.save();

}