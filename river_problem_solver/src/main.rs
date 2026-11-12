pub mod min_priority_queue;

use std::collections::HashMap;
use crate::min_priority_queue::MinPriorityQueue;

#[derive(Debug)]
pub struct State {
    /*
    0: Small A,
    1: Small A,
    2: Big A,
    3: Small B,
    4: Small B,
    5: Big B,
    6: Small Killer,
    7: Big policeman
     */
    left_bank: [bool; 8],
    right_bank: [bool; 8],
}

impl Default for State {
    fn default() -> Self {
        Self {
            left_bank: [true; 8],
            right_bank: [false; 8],
        }
    }
}

enum StateType {
    Occurred,
    Pending,
}

fn solve() {
    let state_types: HashMap<State, StateType> = HashMap::new();


}



fn main() {
    todo!()
}

