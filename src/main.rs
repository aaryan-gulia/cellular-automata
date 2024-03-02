use std::io;

mod elementary_automata{
    pub struct AutomataState{
        state_vec: Vec<bool>,
        generation: u32,
        rule:Rule
    }
    impl AutomataState{
        pub fn new(state_vec:Vec<bool>,rule: Rule)->AutomataState{
            let new_state = AutomataState { state_vec:state_vec, generation: 0 ,rule:rule};
            new_state
        }
        pub fn move_next_gen(&mut self){
            let max_len = self.state_vec.len();
            let mut new_state_vec:Vec<bool> = Vec::new();
            for pointer in 0..max_len{
                let prev:&bool = if pointer == 0 {
                    self.state_vec.get(self.state_vec.len()-1).expect("SIZE - 1 IS ALWAYS BE A VALID INDEX")
                } else{
                    self.state_vec.get(pointer-1).expect("pointer - 1 will never be invalid")
                };
                let next:&bool = match self.state_vec.get(pointer+1){
                    Some(t) => t,
                    None => {
                        self.state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                    }
                };
                let ordinal:&bool = match self.state_vec.get(pointer){
                    Some(t) => t,
                    None => {
                        self.state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                    }
                };
                let mut rule_applier:usize = 0;
                if *prev == false && *ordinal == false && *next == false {rule_applier=0;}
                else if *prev == false && *ordinal == false && *next == true {rule_applier=1;}
                else if *prev == false && *ordinal == true && *next == false {rule_applier=2;}
                else if *prev == false && *ordinal == true && *next == true {rule_applier=3;}
                else if *prev == true && *ordinal == false && *next == false {rule_applier=4;}
                else if *prev == true && *ordinal == false && *next == true {rule_applier=5;}
                else if *prev == true && *ordinal == true && *next == false {rule_applier=6;}
                else if *prev == true && *ordinal == true && *next == true {rule_applier=7;}

                new_state_vec.push(self.rule.rule_array[7-rule_applier]);
            }
            self.state_vec = new_state_vec;
            self.generation += 1;
        }
        pub fn print_automata(&self){
            let mut print_string = String::new();
            //print_string.push_str("generation ");
            //print_string.push_str(&(self.generation.to_string()));
            //print_string.push_str(": ");
            for ordinal in &self.state_vec{
                if *ordinal {
                    print_string.push('*');
                }else{
                    print_string.push(' ');
                }
            }
            println!("{}",print_string);
        }
    }
    pub struct Rule{
        rule_array:[bool;8],
    }

    impl Rule {
        pub fn new(rule: String) ->Rule{
            let mut new_rule:Rule = Rule { rule_array: [false;8] };
            let mut counter  = 8 - rule.len();
            for r in rule.chars(){
                if r == '1' {*(new_rule.rule_array.get_mut(counter).expect("COUNTER IS ALWAYS VALID INDEX")) = true;}
                counter += 1;
            };
            new_rule
        }
    }
}

fn main() {
    println!("Welcome to the elementary cellular automata!");
    println!("Please enter the rule number (0-255) you'd like to see:");
    let mut rule = String::new();
    io::stdin().read_line(&mut rule).expect("NOT ABLE TO READ RULE");
    let rule:u32 = match rule.trim().parse(){
        Ok(r) => r,
        Err(_) => panic!("INVALID RULE: {}",rule),
    };
    let rule = format!("{:b}", rule);
    let rule = elementary_automata::Rule::new(rule);

    // Hardcoding the initial state vector
    let mut initial_state_vec = vec![false; 101];
    initial_state_vec[51] = true;
    let mut automata = elementary_automata::AutomataState::new(initial_state_vec,rule);

    println!("Please enter the number of generations to play:");
    let mut generations = String::new();
    io::stdin().read_line(&mut generations).expect("NOT ABLE TO READ GENERATIONS");
    let generations:u32 = match generations.trim().parse(){
        Ok(r) => r,
        Err(_) => panic!("INVALID GENERATIONS: {}",generations),
    };
    automata.print_automata();
    for _gen in 1..=generations{
        automata.move_next_gen();
        automata.print_automata();
    }
}
