use crate::automata::traits::{NextGenApplicable, NextGenApplicable2D};
use bevy::prelude::*;
#[derive(Resource)]
pub struct Rule{
    rule_array:[bool;8],
}

impl NextGenApplicable for Rule {
    fn get_next_state(&self, prev: bool, current: bool, next: bool) -> bool {
        let rule_applier:usize = match (prev, current, next) {
            (false, false, false) => 0,
            (false, false, true) => 1,
            (false, true, false) => 2,
            (false, true, true) => 3,
            (true, false, false) => 4,
            (true, false, true) => 5,
            (true, true, false) => 6,
            (true, true, true) => 7,
        };

        self.rule_array[7-rule_applier]
    }
}
impl Rule {
    pub fn new(rule: String) -> Rule {
        let mut new_rule: Rule = Rule { rule_array: [false;8] };
        let mut counter  = 8 - rule.len();
        for r in rule.chars(){
            if r == '1' {*(new_rule.rule_array.get_mut(counter).expect("COUNTER IS ALWAYS VALID INDEX")) = true;}
            counter += 1;
        };
        new_rule
    }
}

#[derive(Resource)]
pub struct Rule2D{
    pub under_population: u8,
    pub over_population: u8,
    pub birth_min: u8,
    pub birth_max: u8,
}

impl NextGenApplicable2D for Rule2D {
    fn get_next_state(&self, alive_neighbours: u8, curr_cell:bool) -> bool {
        if curr_cell {
            if alive_neighbours < self.under_population || alive_neighbours > self.over_population {
                return false
            }
            return true
        } else {
            if alive_neighbours >= self.birth_min && alive_neighbours <= self.birth_max {
                return true
            }
            return false
        }
    }
}