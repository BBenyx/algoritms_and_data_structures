#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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
    pub left_bank: [bool; 8],
    pub boat_side: bool,
}

impl State {
    pub fn goal() -> Self {
        Self {
            left_bank: [false; 8],
            boat_side: false,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            left_bank: [true; 8],
            boat_side: true,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum StateType {
    Occurred,
    Pending,
}

pub enum StateChangeError{
    InvalidFLipPair,
    InvalidTargetState,
}