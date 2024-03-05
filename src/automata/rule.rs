use crate::automata::traits::NextGenApplicable;
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