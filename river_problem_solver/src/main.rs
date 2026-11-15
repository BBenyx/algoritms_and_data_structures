pub mod rules;
pub mod state;
use crate::rules::{valid_flip_pair, valid_state};
use crate::state::{State, StateType};

use std::collections::HashMap;

type FlipPair = (usize, Option<usize>);

fn solve() -> bool {
    let mut state_types: HashMap<State, StateType> = HashMap::new();
    let mut visit_stack: Vec<State> = Vec::new();

    let mut current_state = State::default();
    let goal_state = State::goal();

    state_types.insert(current_state, StateType::Pending);
    loop {
        let returned_state = depth_first_search(
            &mut state_types,
            &mut visit_stack,
            current_state,
            goal_state,
        );

        match returned_state {
            Some(val) => {
                if val == goal_state {
                    return true;
                }
            }
            None => {
                let Some(new_current_state) = visit_stack.pop() else {
                    return false;
                };
                current_state = new_current_state;
            }
        }
    }
}

fn depth_first_search(
    state_types: &mut HashMap<State, StateType>,
    visit_stack: &mut Vec<State>,
    current_state: State,
    goal_state: State,
) -> Option<State> {
    assert!(
        valid_state(current_state),
        "Invalid state was given to depth first search fn!"
    );

    let Some(current_state_type) = state_types.get_mut(&current_state) else {
        panic!("Depth first search fn parent not in the state types!");
    };
    if *current_state_type == StateType::Occurred {
        panic!("An occurred state was given to depth first search fn.");
    } else if *current_state_type == StateType::Pending {
        *current_state_type = StateType::Occurred;
    }

    let mut first_match = current_state;

    for pair in gen_flip_pairs(current_state) {
        let child = flip(current_state, pair);

        if valid_state(child) && state_types.get(&child).is_none() {
            if first_match == current_state {
                first_match = child;
                state_types.insert(first_match, StateType::Pending);
            } else {
                state_types.insert(child, StateType::Pending);
                visit_stack.push(child);
            }
        }
    }

    if first_match == current_state {
        return None;
    } else if first_match == goal_state {
        return Some(first_match);
    }
    
    depth_first_search(state_types, visit_stack, first_match, goal_state)
}

fn gen_flip_pairs(state: State) -> Vec<FlipPair> {
    let mut pairs = Vec::new();

    let rowers: [usize; 3] = [2, 5, 7];
    for rower in rowers {
        for i in 0..9 {
            let passenger = if i >= 8 { None } else { Some(i) };

            let flip_pair = FlipPair::from((rower, passenger));

            if valid_flip_pair(&flip_pair, state) {
                pairs.push(flip_pair);
            }
        }
    }
    pairs
}

fn flip(state: State, flip_pair: FlipPair) -> State {
    let mut new_state = state;
    new_state.left_bank[flip_pair.0] = !new_state.left_bank[flip_pair.0];
    if let Some(passenger_idx) = flip_pair.1 {
        new_state.left_bank[passenger_idx] = !new_state.left_bank[passenger_idx];
    };
    new_state.boat_side = !new_state.boat_side;
    new_state
}

fn main() {
    println!("{:?}", solve());
}
