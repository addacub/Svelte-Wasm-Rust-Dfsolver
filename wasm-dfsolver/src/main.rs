use std::time::SystemTime;

use dfsolver::puzzle::{
    piece::PieceBoardPosition,
    solver::{SolverMultiThreaded, SolverSingleThreaded},
};

fn main() {
    let day = 21;
    let month = 5;

    let start_time = SystemTime::now();
    let mut dragon = SolverMultiThreaded::new(day, month);
    dragon.find_solution_set();
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).ok().unwrap();

    println!(
        "Program took {:?} to execute.",
        (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9).to_string()
    );
    println!(
        "{} solution(s) were found.",
        dragon.get_solution_set().len()
    );
    dragon.remove_duplicates();
    println!(
        "{} unique solution(s) were found.",
        dragon.get_solution_set().len()
    );
    if dragon.get_solution_set().len() > 0 {
        print_solution(0, dragon.get_solution_set());
    }
}

/// Print out the specified solution from the solution set
fn print_solution(index: usize, solution_set: &Vec<Vec<PieceBoardPosition>>) {
    println!(
        "Solution {} of {} is shown below:",
        index + 1,
        solution_set.len()
    );
    let solution = &solution_set[index];
    for piece in solution {
        println!(
            "Place {} at position (row, col) = ({}, {}) with the following orientation:\n",
            piece.get_name(),
            piece.get_board_position().0,
            piece.get_board_position().1
        );
        println!("{}", piece.get_orienation());
        println!("\n");
    }
}
