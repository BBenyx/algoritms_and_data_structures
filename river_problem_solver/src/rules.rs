use crate::FlipPair;
use crate::state::State;

pub fn valid_flip_pair(flip_pair: &FlipPair, state: State) -> bool {
    match *flip_pair {
        (2, Some(val)) => {
            val != 2
                && state.left_bank[2] == state.left_bank[val]
                && state.boat_side == state.left_bank[val]
        }
        (5, Some(val)) => {
            val != 5
                && state.left_bank[5] == state.left_bank[val]
                && state.boat_side == state.left_bank[val]
        }
        (7, Some(val)) => {
            val != 7
                && state.left_bank[7] == state.left_bank[val]
                && state.boat_side == state.left_bank[val]
        }
        (2, None) => state.boat_side == state.left_bank[2],
        (5, None) => state.boat_side == state.left_bank[5],
        (7, None) => state.boat_side == state.left_bank[7],
        _ => false,
    }
}

pub fn valid_state(state: State) -> bool {
    let normal = match_state_rule(state);
    if !normal {
        return false;
    }

    let mut negated_state = state;
    for elem in negated_state.left_bank.iter_mut() {
        *elem = !*elem
    }

    match_state_rule(negated_state)
}

fn match_state_rule(state: State) -> bool {
    match state.left_bank {
        // Everyone is on the other side
        [false, false, false, false, false, false, false, false] => true,
        // Everyone is on the other side except the killer
        [false, false, false, false, false, false, true, false] => true,
        // Only the killer is on the other side
        [true, true, true, true, true, true, false, true] => true,
        // Strong A is on the left side but the policeman keeps everyone else safe on the right bank
        [_, _, true, false, false, false, false, false] => true,
        // Strong B is on the left side but the policeman keeps everyone else safe on the right bank
        [false, false, false, _, _, true, false, false] => true,
        // The policeman is always with the weak people and with the killer
        [_, _, false, _, _, false, false, false] => true,
        [_, _, true, _, _, true, false, false] => true,
        // Policeman is together with the killer
        [_, _, _, _, _, _, true, true] => true,
        [_, _, _, _, _, _, _, _] => false,
    }
}
