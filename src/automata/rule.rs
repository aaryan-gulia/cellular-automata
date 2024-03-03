use crate::automata::traits::NextGenApplicable;
use bevy::prelude::*;
#[derive(Resource)]
pub struct Rule{
    rule_array:[bool;8],
}

impl NextGenApplicable for Rule {
    fn get_next_state(&self, prev: bool, current: bool, next: bool) -> bool {
        let mut rule_applier:usize = 0;
        if prev == false && current == false && next == false {rule_applier=0;}
        else if prev == false && current == false && next == true {rule_applier=1;}
        else if prev == false && current == true && next == false {rule_applier=2;}
        else if prev == false && current == true && next == true {rule_applier=3;}
        else if prev == true && current == false && next == false {rule_applier=4;}
        else if prev == true && current == false && next == true {rule_applier=5;}
        else if prev == true && current == true && next == false {rule_applier=6;}
        else if prev == true && current == true && next == true {rule_applier=7;}

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