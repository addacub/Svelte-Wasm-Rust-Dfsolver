use crate::puzzle::solver::{SolverSingleThreaded};

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use serde_wasm_bindgen as swb;

pub mod puzzle;
pub mod utils;

#[wasm_bindgen]
pub fn solve(day: usize, month: usize) -> Result<JsValue, swb::Error> {
    let mut dragon = SolverSingleThreaded::new(day, month);
    dragon.find_solution_set();
    dragon.remove_duplicates();

    serde_wasm_bindgen::to_value(dragon.get_solution_set())
}
