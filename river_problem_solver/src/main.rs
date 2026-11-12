pub mod rules;
pub mod state;
use crate::state::{State, StateType};
use crate::rules::{valid_flip_pair, valid_state};

use std::collections::{HashMap, LinkedList};


type FlipPair = (usize, Option<usize>);



fn solve() -> bool {
    let mut state_types: HashMap<State, StateType> = HashMap::new();
    let mut visit_stack: Vec<State> = Vec::new();
    let mut graph: Vec<LinkedList<State>> = Vec::new();

    let mut current_state = State::default();
    let goal_state = State::goal();

    loop {
        println!("In loop");
        let returned_state = depth_first_search(
            &mut state_types,
            &mut visit_stack,
            &mut graph,
            current_state.clone(),
            goal_state.clone()
        );

        match returned_state {
            Some(val) =>  {
                if val == goal_state {
                    return true;
                }
            },
            None => {
                let Some(new_current_state) = visit_stack.pop() else {
                    println!("false check");
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
    graph: &mut Vec<LinkedList<State>>,
    current_state: State,
    goal_state: State,
) -> Option<State> {
    if !valid_state(&current_state) {
        println!("Not valid current state!");
        return None};
    println!("HashMap len: {}", state_types.len());

    graph.push(LinkedList::from([current_state]));
    let vertex = graph.len() - 1;
    let current_state = graph[graph.len() - 1].front().unwrap().clone(); //TODO

    let Some(current_state_type) = state_types.get_mut(&current_state) else {
        println!("Not valid current state!2");
        return None;
    };
    if *current_state_type == StateType::Occurred {
        println!("Not valid current state!3");
        return None;
    } else if *current_state_type == StateType::Pending {
        *current_state_type = StateType::Occurred;
    }


    let mut first_match = current_state.clone();

    for pair in gen_flip_pairs(&current_state) {
        let mut child = current_state.clone();
        flip(&mut child, pair);

        if valid_state(&child) {

            match state_types.get(&child) {
                Some(_) => (),
                None => {
                    if first_match == current_state {
                        first_match = child;

                        state_types.insert(first_match.clone(), StateType::Pending);
                        println!("HashMap len: {}", state_types.len());
                        let match_ref = graph[vertex].iter().last().unwrap();

                    } else {

                        state_types.insert(child.clone(), StateType::Pending);
                        println!("HashMap len: {}", state_types.len());
                        visit_stack.push(child);
                    }
                },
            }
        }
    }
    if first_match == current_state {
        return None;
    } else if first_match == goal_state {
        return Some(first_match);
    }
    println!("Current first match: {:?}", &first_match);

    depth_first_search(state_types, visit_stack, graph, first_match, goal_state)
}

fn gen_flip_pairs(state: &State) -> Vec<FlipPair> {
    let mut pairs = Vec::new();

    let rowers: [usize; 3] = [2,5,7];
    for rower in rowers {
        for i in 0..9 {
            let passenger = if i >= 8 {
                None
            } else {
                Some(i)
            };

            let flip_pair = FlipPair::from((rower, passenger));

            if valid_flip_pair(&flip_pair, state) {
                pairs.push(flip_pair);
            }
        }
    }
    pairs
}

fn flip(state: &mut State, flip_pair: FlipPair) {
    state.left_bank[flip_pair.0] = !state.left_bank[flip_pair.0];
    if let Some(passenger_idx) = flip_pair.1 {
        state.left_bank[passenger_idx] = !state.left_bank[passenger_idx];
    }
}




fn main() {
   println!("{:?}", solve());
}

