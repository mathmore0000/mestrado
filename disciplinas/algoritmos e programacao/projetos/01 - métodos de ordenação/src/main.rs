mod benchmark;
mod generator;
mod sorts;
mod export;

use benchmark::measure_time_elapsed;
use export::XlsxExporter;
use generator::{generate_vec, InputPattern};

use sorts::{
    bubble_sort,
    selection_sort,
    insertion_sort,
    merge_sort,
    quick_sort,
};

fn main() {

    let sizes = [1000, 10000, 100000];
    let retry_times = 5;

    let patterns = [
        InputPattern::Random,
        InputPattern::Ascending,
        InputPattern::Descending,
    ];

    let algorithms: [(&str, fn(&mut [i32])); 5] = [
        ("bubble", bubble_sort),
        ("selection", selection_sort),
        ("insertion", insertion_sort),
        ("merge", merge_sort),
        ("quick", quick_sort),
    ];

    println!("Each combination will be executed {} times\n", retry_times);

    let mut exporter = XlsxExporter::new();

    for pattern in patterns.iter() {

        let pattern_name = format!("{:?}", pattern).to_lowercase();

        println!("Pattern: {}\n", pattern_name);

        let run_sheet_name = format!("runs_{}", pattern_name);
        let summary_sheet_name = format!("summary_{}", pattern_name);

        exporter.write_headers(&run_sheet_name, &["Algorithm", "Size", "Run", "Time"]);
        exporter.write_headers(&summary_sheet_name, &["Size", "Bubble", "Selection", "Insertion", "Merge", "Quick"]);

        for size in sizes {

            println!("Vector size: {}", size);

            let base_data = generate_vec(size, *pattern);

            let mut totals = vec![0.0f32; algorithms.len()];

            for run in 1..=retry_times {

                for (i, (name, algorithm)) in algorithms.iter().enumerate() {

                    let time = measure_time_elapsed(base_data.clone(), *algorithm);

                    totals[i] += time;

                    exporter.write_run(
                        &run_sheet_name,
                        name,
                        size,
                        run,
                        time,
                    );
                }
            }

            let averages: Vec<f32> = totals
                .iter()
                .map(|t| t / retry_times as f32)
                .collect();

            for (i, (name, _)) in algorithms.iter().enumerate() {
                println!("{} avg: {}", name, averages[i]);
            }

            exporter.write_summary(
                &summary_sheet_name,
                size,
                averages[0],
                averages[1],
                averages[2],
                averages[3],
                averages[4],
            );

            println!();
        }

        println!("------------------------------------\n");
    }

    exporter.save();
}