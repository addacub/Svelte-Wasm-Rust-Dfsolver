use std::{fs::File, io::Write, ops::Range, path::PathBuf, time::SystemTime};

use dfsolver::puzzle::solver::{SolverMultiThreaded, SolverSingleThreaded};

#[test]
#[ignore]
/// Runs through all combinations of days and months
fn bench_test() {
    let file_name = "one_layer_of_multithreading".to_string();

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let mut failed: Vec<[String; 5]> = Vec::new();
    let mut passed: Vec<[String; 5]> = Vec::new();

    for (index, month) in months.iter().enumerate() {
        for day in (Range { start: 1, end: 32 }) {
            println!("Finding solution for {} of {}", day, month);
            let start_time = SystemTime::now();
            let mut dragon = SolverMultiThreaded::new(day, index + 1);
            dragon.find_solution_set();
            let end_time = SystemTime::now();

            let duration = end_time.duration_since(start_time).ok().unwrap();
            let solution_set = dragon.get_solution_set().clone();
            dragon.remove_duplicates();
            let unique_solution_set = dragon.get_solution_set();

            if !solution_set.is_empty() {
                let number_solutions = solution_set.len();
                let number_unique_solutions = unique_solution_set.len();
                passed.push([
                    day.to_string(),
                    month.to_string(),
                    number_solutions.to_string(),
                    number_unique_solutions.to_string(),
                    (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9).to_string(),
                ]);
            } else {
                failed.push([
                    day.to_string(),
                    month.to_string(),
                    "None".to_string(),
                    "None".to_string(),
                    (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9).to_string(),
                ])
            }
        }
    }

    // Write to csv file
    let mut failed_name = file_name.clone();
    failed_name.push_str("_failed");

    let mut passed_name = file_name.clone();
    passed_name.push_str("_passed");

    write_to_file(failed, failed_name);
    write_to_file(passed, passed_name);
}

/// Writes results to a text file
fn write_to_file(results: Vec<[String; 5]>, mut file_name: String) {
    // Check file extension has been added
    if !file_name.ends_with(".txt") {
        file_name += ".txt";
    }

    // Create file path
    let mut relative_path = PathBuf::from(
        "/home/cubea/Documents/repos/rust-projects/rust-dragon-fjord-solver/tests/results",
    );
    relative_path.push(file_name);

    let mut counter: u8 = 1;
    loop {
        if relative_path.exists() {
            let file_name = relative_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            relative_path.pop();

            if !file_name.contains("copy") {
                relative_path.push(file_name.replace(".txt", "_copy.txt"));
            } else {
                relative_path.push(file_name.replace(".txt", &(counter.to_string() + ".txt")));
            }

            counter += 1;
        } else {
            break;
        }
    }

    // Open a file in write-only (ignoring errors)
    // This creates the file if it does not exist (and empty the file if it exists)
    let mut file = match File::create(&relative_path) {
        Err(why) => panic!("Couldn't create {}: {}", relative_path.display(), why),
        Ok(file) => file,
    };

    // Write to file
    writeln!(
        &mut file,
        "Day, Month, No. Solutions, No. Unique Solutions, Seconds to Solve"
    )
    .unwrap();
    for result in results {
        writeln!(&mut file, "{}", result.join(",")).unwrap();
    }
}
